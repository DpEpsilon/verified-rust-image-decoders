//! Runtime conformance over the 810 records of the Stage-1 split (PLAN.md
//! "Tests"): `../spec-frozen/VECTORS.json` (599 Strict records),
//! `../limits-draft/VECTORS-CAPS.json` (13 cap-dependent Strict records, both
//! `pad_to` records included) and `../policy-draft/VECTORS-POLICY.json` (198
//! profile records). The vector files are git-ignored (PNG-Q16) and read from
//! the working tree; a missing file fails the test.
//!
//! Records of a profile this crate does not implement yet (the two
//! `ImageCrateCompat-0.25.10` records, milestone P2) are counted as skipped,
//! never as passed. `PNG_VECTORS=file` runs one further record file.

use png_verus::{AllocSite, Frames, Profile, TryError};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::path::PathBuf;

fn hex_decode(s: &str) -> Vec<u8> {
    let b = s.as_bytes();
    let v = |c: u8| match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        b'A'..=b'F' => c - b'A' + 10,
        _ => panic!("bad hex"),
    };
    b.chunks(2).map(|p| (v(p[0]) << 4) | v(p[1])).collect()
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

/// Expand `rgba_runs` / `rgba16_runs` into bytes.
fn runs(v: &Value) -> Vec<u8> {
    let mut out = Vec::new();
    for r in v.as_array().expect("runs") {
        let px = hex_decode(r["hex"].as_str().unwrap());
        for _ in 0..r["count"].as_u64().unwrap() {
            out.extend_from_slice(&px);
        }
    }
    out
}

fn first_diff(a: &[u8], b: &[u8]) -> String {
    if a.len() != b.len() {
        return format!("length {} vs expected {}", a.len(), b.len());
    }
    let i = a.iter().zip(b).position(|(x, y)| x != y).unwrap();
    format!("byte {i}: got {:02x} expected {:02x}", a[i], b[i])
}

fn err_name(e: &TryError) -> String {
    e.name().to_string()
}

fn events_json(v: &[Option<&str>; 3]) -> Value {
    Value::Array(v.iter().flatten().map(|s| Value::String(s.to_string())).collect())
}

fn any_event(v: &[Option<&str>; 3]) -> bool {
    v.iter().any(|e| e.is_some())
}

fn site_name(s: AllocSite) -> &'static str {
    match s {
        AllocSite::Raw => "Raw",
        AllocSite::Output => "Output",
        AllocSite::Saved => "Saved",
        AllocSite::IccWindow => "IccWindow",
        AllocSite::Icc => "Icc",
    }
}

/// Compare one record; `Ok(())` or a failure description.
fn check(rec: &Value, b: &[u8], profile: Profile, profiled: bool) -> Result<(), String> {
    let exp = &rec["expected"];
    let entry = rec["entry"].as_str().unwrap();
    let want_err = exp.get("error").and_then(|e| e.as_str());
    match entry {
        "still" | "still16" => {
            let wide = entry == "still16";
            let (got, _) = if wide {
                let (r, t) = png_verus::try_decode_still16_traced(b, profile);
                (r.map(|(s, e)| (s.width, s.height, s.rgba16, e)), t)
            } else {
                let (r, t) = png_verus::try_decode_still_traced(b, profile);
                (r.map(|(s, e)| (s.width, s.height, s.rgba, e)), t)
            };
            match (got, want_err) {
                (Err(e), Some(w)) => {
                    if err_name(&e) == w { Ok(()) } else { Err(format!("error {} expected {w}", err_name(&e))) }
                }
                (Err(e), None) => Err(format!("error {} expected success", err_name(&e))),
                (Ok(_), Some(w)) => Err(format!("success expected {w}")),
                (Ok((w, h, px, ev)), None) => {
                    if exp["width"] != w || exp["height"] != h {
                        return Err(format!("size {w}x{h}"));
                    }
                    let want = runs(&exp[if wide { "rgba16_runs" } else { "rgba_runs" }]);
                    if px != want {
                        return Err(format!("pixels: {}", first_diff(&px, &want)));
                    }
                    if profiled {
                        if let Some(r) = exp.get("recovered") {
                            if *r != Value::Bool(any_event(&ev)) {
                                return Err(format!("recovered {}", any_event(&ev)));
                            }
                        }
                        if let Some(e) = exp.get("events") {
                            if *e != events_json(&ev) {
                                return Err(format!("events {ev:?}"));
                            }
                        }
                    }
                    Ok(())
                }
            }
        }
        "frames" => {
            let mut d = match (Frames::try_new_with_policy(b, profile), want_err) {
                (Err(e), Some(w)) => {
                    return if err_name(&e) == w { Ok(()) } else { Err(format!("error {} expected {w}", err_name(&e))) };
                }
                (Err(e), None) => return Err(format!("constructor error {}", err_name(&e))),
                (Ok(_), Some(w)) => return Err(format!("constructor success expected {w}")),
                (Ok(d), None) => d,
            };
            let info = *d.info();
            if exp["width"] != info.width || exp["height"] != info.height {
                return Err("canvas size".into());
            }
            let np = match info.num_plays {
                Some(n) => Value::from(n),
                None => Value::Null,
            };
            if exp["num_plays"] != np {
                return Err(format!("num_plays {np}"));
            }
            let want_frames = exp["frames"].as_array().unwrap();
            let mut k = 0;
            let end;
            loop {
                match d.try_next() {
                    Ok(Some(f)) => {
                        let Some(wf) = want_frames.get(k) else { return Err(format!("extra frame {k}")) };
                        let rect = Value::from(vec![f.rect.0, f.rect.1, f.rect.2, f.rect.3]);
                        if wf["rect"] != rect {
                            return Err(format!("frame {k} rect {rect}"));
                        }
                        let delay = match f.delay {
                            Some((n, dd)) => Value::from(vec![n, dd]),
                            None => Value::Null,
                        };
                        if wf["delay"] != delay || wf["dispose"] != f.dispose || wf["blend"] != f.blend {
                            return Err(format!("frame {k} fields"));
                        }
                        let want = runs(&wf["rgba_runs"]);
                        if f.rgba != want.as_slice() {
                            return Err(format!("frame {k} pixels: {}", first_diff(f.rgba, &want)));
                        }
                        if profiled {
                            if let Some(r) = wf.get("recovered") {
                                if *r != Value::Bool(f.recovered) {
                                    return Err(format!("frame {k} recovered {}", f.recovered));
                                }
                            }
                            if let Some(e) = wf.get("events") {
                                if *e != events_json(&f.event_names()) {
                                    return Err(format!("frame {k} events {:?}", f.event_names()));
                                }
                            }
                        }
                        k += 1;
                    }
                    Ok(None) => {
                        end = Value::from("iend");
                        break;
                    }
                    Err(e) => {
                        end = serde_json::json!({ "error": err_name(&e) });
                        break;
                    }
                }
            }
            if k != want_frames.len() {
                return Err(format!("{k} frames, expected {}", want_frames.len()));
            }
            if exp["end"] != end {
                return Err(format!("end {end}"));
            }
            // Fused (D3): every later call returns Ok(None).
            for _ in 0..2 {
                if !matches!(d.try_next(), Ok(None)) {
                    return Err("not fused".into());
                }
            }
            Ok(())
        }
        "icc" => {
            let (got, (sites, nreq)) = png_verus::try_icc_profile_traced(b);
            let trace = &sites[..nreq];
            if let Some(req) = rec.get("req") {
                let t: Vec<Value> =
                    trace.iter().map(|(s, n)| Value::from(vec![Value::from(site_name(*s)), Value::from(*n)])).collect();
                if *req != Value::Array(t.clone()) {
                    return Err(format!("req {t:?}"));
                }
            }
            match (got, want_err) {
                (Err(e), Some(w)) => {
                    if err_name(&e) == w { Ok(()) } else { Err(format!("error {} expected {w}", err_name(&e))) }
                }
                (Err(e), None) => Err(format!("error {} expected success", err_name(&e))),
                (Ok(_), Some(w)) => Err(format!("success expected {w}")),
                (Ok(None), None) => {
                    if exp.get("icc") == Some(&Value::Null) { Ok(()) } else { Err("no profile".into()) }
                }
                (Ok(Some(p)), None) => {
                    let sha = hex(&Sha256::digest(&p));
                    if exp["icc_len"] == p.len() && exp["icc_sha256"] == sha.as_str() {
                        Ok(())
                    } else {
                        Err(format!("profile {} bytes {sha}", p.len()))
                    }
                }
            }
        }
        _ => Err(format!("unknown entry {entry}")),
    }
}

struct Tally {
    pass: usize,
    skip: usize,
    fail: Vec<String>,
}

fn run_file(path: &PathBuf) -> Tally {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let recs: Vec<Value> = serde_json::from_str(&text).expect("json");
    let mut t = Tally { pass: 0, skip: 0, fail: Vec::new() };
    for rec in &recs {
        let name = rec["name"].as_str().unwrap();
        let (profile, profiled) = match rec.get("profile").and_then(|p| p.as_str()) {
            None | Some("Strict") => (Profile::Strict, false),
            Some("RecommendedBeta") => (Profile::RecommendedBeta, true),
            Some(_) => {
                t.skip += 1;
                continue;
            }
        };
        let mut b = hex_decode(rec["input_hex"].as_str().unwrap());
        if let Some(n) = rec.get("pad_to").and_then(|n| n.as_u64()) {
            b.resize(n as usize, 0);
        }
        match check(rec, &b, profile, profiled) {
            Ok(()) => t.pass += 1,
            Err(e) => t.fail.push(format!("{name}: {e}")),
        }
    }
    println!(
        "{}: {} records, {} pass, {} fail, {} skipped (profile not implemented)",
        path.display(),
        recs.len(),
        t.pass,
        t.fail.len(),
        t.skip
    );
    for f in &t.fail {
        println!("  FAIL {f}");
    }
    t
}

#[test]
fn vectors() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    let mut files = vec![
        root.join("spec-frozen/VECTORS.json"),
        root.join("limits-draft/VECTORS-CAPS.json"),
        root.join("policy-draft/VECTORS-POLICY.json"),
    ];
    if let Ok(extra) = std::env::var("PNG_VECTORS") {
        files = vec![PathBuf::from(extra)];
    }
    let mut failed = 0;
    let (mut pass, mut skip) = (0, 0);
    for f in &files {
        let t = run_file(f);
        failed += t.fail.len();
        pass += t.pass;
        skip += t.skip;
    }
    println!("total: {pass} pass, {failed} fail, {skip} skipped");
    assert_eq!(failed, 0, "{failed} records failed");
}
