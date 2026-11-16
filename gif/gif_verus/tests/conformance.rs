//! Conformance harness: every record of `../spec-draft/VECTORS.json` (SPEC.md
//! G17; 373 records) and the probe expectations `tests/data/PROBES.json`
//! (generated from `../policy-draft/probes` by `tools/gen_probe_expect.py`, the
//! inputs `validate.py` checks, paired with `gif_oracle.py` for Strict and
//! `policy_oracle.py` for RecommendedBeta) against the `try_*` API.
//!
//! A record without `profile` runs the Strict entry points; `RecommendedBeta`
//! runs through the profile selector. The two `@ImageCrateCompat-0.25.10`
//! records exercise only the metadata field `loop_scan = FirstSubBlockOnly`
//! (milestone P2 adds that profile); they run under the test-only record
//! "RecommendedBeta with `loop_scan = FirstSubBlockOnly`" (feature
//! `test-records`), which `policy_oracle.py` gives the same result as
//! `ImageCrateCompat-0.25.10` on both inputs.
//!
//! Compared: accept/reject and the error name, width/height, every RGBA byte
//! (expanded from `rgba_runs`), every frame field, `recovered` on profile
//! records, `loop_count`, `end`, and for `plays` records `plays`/`total_plays`.
//! The tests run without a fault-injecting allocator, so `OutOfMemory` is a
//! failure.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use gif_verus::policy::Policy;
use gif_verus::{plays, Frames, Plays, Profile, TryError};
use serde_json::{json, Value};

fn gif_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().expect("crate has a parent").to_path_buf()
}

fn load_vectors() -> Vec<Value> {
    let path = gif_dir().join("spec-draft/VECTORS.json");
    if !path.exists() {
        // VECTORS.json is gitignored; regenerate it as SPEC.md G17 describes.
        let st = Command::new("python3")
            .arg("-I")
            .arg(gif_dir().join("spec-draft/gen_vectors.py"))
            .current_dir(gif_dir().join("spec-draft"))
            .status()
            .expect("run gen_vectors.py");
        assert!(st.success(), "gen_vectors.py failed");
    }
    let text = fs::read_to_string(&path).expect("read VECTORS.json");
    serde_json::from_str::<Value>(&text).expect("parse VECTORS.json").as_array().unwrap().clone()
}

fn load_probes() -> Vec<Value> {
    let crate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let path = crate_dir.join("tests/data/PROBES.json");
    if !path.exists() {
        let st = Command::new("python3")
            .arg("-I")
            .arg(crate_dir.join("tools/gen_probe_expect.py"))
            .arg(&path)
            .status()
            .expect("run tools/gen_probe_expect.py");
        assert!(st.success(), "gen_probe_expect.py failed");
    }
    let text = fs::read_to_string(&path).expect("read PROBES.json");
    serde_json::from_str::<Value>(&text).expect("parse PROBES.json").as_array().unwrap().clone()
}

fn hex(s: &str) -> Vec<u8> {
    let b = s.as_bytes();
    assert!(b.len() % 2 == 0);
    let nib = |c: u8| match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        b'A'..=b'F' => c - b'A' + 10,
        _ => panic!("non-hex"),
    };
    (0..b.len() / 2).map(|i| (nib(b[2 * i]) << 4) | nib(b[2 * i + 1])).collect()
}

/// G17 input expansion: `input_hex + repeat_hex×repeat + suffix_hex`, then `pad_to`.
fn materialize(v: &Value) -> Vec<u8> {
    let mut b = hex(v["input_hex"].as_str().unwrap());
    if let Some(n) = v.get("repeat").and_then(|x| x.as_u64()) {
        let unit = hex(v["repeat_hex"].as_str().unwrap());
        b.reserve(unit.len() * n as usize);
        for _ in 0..n {
            b.extend_from_slice(&unit);
        }
        b.extend_from_slice(&hex(v["suffix_hex"].as_str().unwrap()));
    }
    if let Some(n) = v.get("pad_to").and_then(|x| x.as_u64()) {
        b.resize(n as usize, 0);
    }
    b
}

fn runs(rgba: &[u8]) -> Value {
    let mut out: Vec<(u64, String)> = Vec::new();
    for px in rgba.chunks(4) {
        let h: String = px.iter().map(|x| format!("{:02x}", x)).collect();
        match out.last_mut() {
            Some((c, last)) if *last == h => *c += 1,
            _ => out.push((1, h)),
        }
    }
    Value::Array(out.into_iter().map(|(c, h)| json!({"count": c, "hex": h})).collect())
}

#[derive(Clone, Copy)]
enum Sel {
    Strict,
    Profile(Profile),
    Record(Policy),
}

fn selector(v: &Value) -> Sel {
    match v.get("profile").and_then(|p| p.as_str()) {
        None => Sel::Strict,
        Some("RecommendedBeta") => Sel::Profile(Profile::RecommendedBeta),
        Some("ImageCrateCompat-0.25.10") => {
            Sel::Record(Policy::of_profile(Profile::RecommendedBeta).with_loop_scan_first_only())
        }
        Some(p) => panic!("unknown profile {p}"),
    }
}

fn err_name(e: &TryError) -> Value {
    match e {
        TryError::Format(f) => json!({"error": f.name()}),
        TryError::OutOfMemory(o) => panic!("unexpected OutOfMemory at {:?} ({} elements)", o.site(), o.elements()),
    }
}

fn open<'a>(b: &'a [u8], sel: Sel) -> Result<Frames<'a>, TryError> {
    match sel {
        Sel::Strict => Frames::try_new(b),
        Sel::Profile(p) => Frames::try_new_with_policy(b, p),
        Sel::Record(r) => Frames::try_new_with_record(b, r),
    }
}

fn plays_json(p: Plays) -> Value {
    match p {
        Plays::Once => json!("Once"),
        Plays::Forever => json!("Forever"),
        Plays::Repeats(n) => json!({"Repeats": n}),
    }
}

/// The actual result in the record's schema (mirrors `check_vectors.actual`).
fn actual(v: &Value) -> Value {
    let b = materialize(v);
    let sel = selector(v);
    let profiled = !matches!(sel, Sel::Strict);
    let entry = v["entry"].as_str().unwrap();
    if entry == "still" {
        let r = match sel {
            Sel::Strict => gif_verus::try_decode_still(&b).map(|s| (s, false)),
            Sel::Profile(p) => gif_verus::try_decode_still_with_policy(&b, p),
            Sel::Record(r) => gif_verus::gif::try_decode_still_with_record(&b, r),
        };
        return match r {
            Err(e) => err_name(&e),
            Ok((s, rec)) => {
                assert_eq!(s.rgba.len(), 4 * s.width as usize * s.height as usize);
                assert!(!(matches!(sel, Sel::Strict) && rec));
                let mut o = json!({"width": s.width, "height": s.height, "rgba_runs": runs(&s.rgba)});
                if profiled {
                    o["recovered"] = json!(rec);
                }
                o
            }
        };
    }
    let mut d = match open(&b, sel) {
        Err(e) => return err_name(&e),
        Ok(d) => d,
    };
    let info = *d.info();
    if entry == "plays" {
        let p = plays(info.loop_count);
        return json!({"loop_count": info.loop_count, "plays": plays_json(p), "total_plays": p.total_plays()});
    }
    let summary = v["expected"].get("summary").is_some();
    let mut frames = Vec::new();
    let mut count = 0u64;
    let mut last = Vec::new();
    let end;
    loop {
        match d.next_frame() {
            None => {
                end = json!("trailer");
                break;
            }
            Some(Err(e)) => {
                end = err_name(&e);
                break;
            }
            Some(Ok(f)) => {
                count += 1;
                assert_eq!(f.rgba.len(), 4 * info.width as usize * info.height as usize);
                if summary {
                    last.clear();
                    last.extend_from_slice(f.rgba);
                    continue;
                }
                let mut o = json!({
                    "rect": [f.rect.0, f.rect.1, f.rect.2, f.rect.3],
                    "delay_cs": f.delay_cs, "disposal": f.disposal, "user_input": f.user_input,
                    "transparent": f.transparent, "interlaced": f.interlaced,
                    "rgba_runs": runs(f.rgba),
                });
                if profiled {
                    o["recovered"] = json!(f.recovered);
                } else {
                    assert!(!f.recovered, "Strict never recovers");
                }
                frames.push(o);
            }
        }
    }
    // Fused: every later call returns None.
    for _ in 0..3 {
        assert!(d.next_frame().is_none(), "decoder not fused");
    }
    if summary {
        return json!({"summary": true, "width": info.width, "height": info.height,
                      "frame_count": count, "last_rgba_runs": runs(&last), "end": end});
    }
    json!({"width": info.width, "height": info.height, "loop_count": info.loop_count,
           "frames": frames, "end": end})
}

fn is_large(v: &Value) -> bool {
    v.get("pad_to").is_some() || v.get("repeat").is_some()
}

fn run(records: &[Value], what: &str) -> usize {
    let mut bad = Vec::new();
    for v in records {
        let a = actual(v);
        if a != v["expected"] {
            let mut sa = a.to_string();
            sa.truncate(400);
            let mut se = v["expected"].to_string();
            se.truncate(400);
            bad.push(format!("MISMATCH {}\n  actual   {}\n  expected {}", v["name"], sa, se));
        }
    }
    for m in &bad {
        eprintln!("{m}");
    }
    eprintln!("{what}: {}/{} records agree", records.len() - bad.len(), records.len());
    assert!(bad.is_empty(), "{} {what} records disagree", bad.len());
    records.len()
}

#[test]
fn vectors_small() {
    let all = load_vectors();
    assert_eq!(all.len(), 373, "VECTORS.json has 373 records");
    let small: Vec<Value> = all.into_iter().filter(|v| !is_large(v)).collect();
    assert_eq!(small.len(), 368);
    run(&small, "VECTORS.json (small)");
}

#[test]
fn vectors_large_padding() {
    let all = load_vectors();
    let big: Vec<Value> = all.into_iter().filter(|v| v.get("pad_to").is_some()).collect();
    assert_eq!(big.len(), 3);
    run(&big, "VECTORS.json (256 MiB padding)");
}

#[test]
fn vectors_large_frames() {
    let all = load_vectors();
    let big: Vec<Value> = all.into_iter().filter(|v| v.get("repeat").is_some()).collect();
    assert_eq!(big.len(), 2);
    run(&big, "VECTORS.json (65,536 frames)");
}

#[test]
fn probes() {
    let recs = load_probes();
    assert!(recs.len() >= 4 * 328, "PROBES.json covers the probe inputs");
    run(&recs, "PROBES.json");
}

/// Any further record file in the same schema, for differential runs against the
/// oracles (for example `tools/gen_mutants.py`): set `GIF_RECORDS=path`.
#[test]
fn extra_records_from_env() {
    let Ok(path) = std::env::var("GIF_RECORDS") else { return };
    let text = fs::read_to_string(&path).expect("read GIF_RECORDS");
    let recs = serde_json::from_str::<Value>(&text).expect("parse").as_array().unwrap().clone();
    run(&recs, &path);
}

#[test]
fn plays_exhaustive() {
    // G20 over all 65,537 inputs (check_plays.py): injective, total at most 65,536.
    assert_eq!(plays(None), Plays::Once);
    assert_eq!(Plays::Once.total_plays(), Some(1));
    assert_eq!(plays(Some(0)), Plays::Forever);
    assert_eq!(Plays::Forever.total_plays(), None);
    for n in 1..=u16::MAX {
        assert_eq!(plays(Some(n)), Plays::Repeats(n));
        assert_eq!(plays(Some(n)).total_plays(), Some(n as u32 + 1));
    }
}

#[test]
fn error_names_closed_list() {
    // The 33 names of G16, each with its exact spelling, in a fixed order.
    use gif_verus::Error::*;
    let all = [
        (FileTooLarge, "FileTooLarge"), (TruncatedHeader, "TruncatedHeader"),
        (BadSignature, "BadSignature"), (UnsupportedVersion, "UnsupportedVersion"),
        (InvalidDimensions, "InvalidDimensions"), (OutputTooLarge, "OutputTooLarge"),
        (TruncatedColorTable, "TruncatedColorTable"), (MissingTrailer, "MissingTrailer"),
        (UnknownBlock, "UnknownBlock"), (TruncatedExtension, "TruncatedExtension"),
        (DuplicateGraphicControl, "DuplicateGraphicControl"),
        (InvalidGraphicControl, "InvalidGraphicControl"), (UndefinedDisposal, "UndefinedDisposal"),
        (InvalidApplicationExtension, "InvalidApplicationExtension"),
        (InvalidPlainText, "InvalidPlainText"),
        (GraphicControlWithoutGraphic, "GraphicControlWithoutGraphic"),
        (TooManyFrames, "TooManyFrames"), (TruncatedImageDescriptor, "TruncatedImageDescriptor"),
        (InvalidFrameDimensions, "InvalidFrameDimensions"), (FrameOutsideScreen, "FrameOutsideScreen"),
        (PixelBudgetExceeded, "PixelBudgetExceeded"), (NoColorTable, "NoColorTable"),
        (TruncatedImageData, "TruncatedImageData"), (InvalidLzwMinCodeSize, "InvalidLzwMinCodeSize"),
        (InvalidLzwCode, "InvalidLzwCode"), (TooManyPixels, "TooManyPixels"),
        (TooFewPixels, "TooFewPixels"), (MissingEndOfInformation, "MissingEndOfInformation"),
        (DataAfterEndOfInformation, "DataAfterEndOfInformation"),
        (ColorIndexOutOfRange, "ColorIndexOutOfRange"), (TrailingBytes, "TrailingBytes"),
        (NoImage, "NoImage"), (UnsupportedGraphicControlTarget, "UnsupportedGraphicControlTarget"),
    ];
    assert_eq!(all.len(), 33);
    for (e, n) in all {
        assert_eq!(e.name(), n);
        assert_eq!(TryError::Format(e).name(), n);
    }
}
