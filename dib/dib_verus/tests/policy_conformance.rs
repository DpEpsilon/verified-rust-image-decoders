//! Policy conformance harness: runs `policy/POLICY-VECTORS.json` against
//! `try_decode_with_policy` for all four named profiles (`Strict`, `Recommended`,
//! `GdkCompat-2.42.12`, `ImageCrateCompat-0.25.10`), all 102 vectors each, and
//! checks the 71-file BMP corpus acceptance per profile. This sits next to
//! `tests/conformance.rs` (the unchanged 302 frozen 1.1 vectors) and
//! `tests/corpus.rs`.
//!
//! Per POLICY-SPEC a policy vector is `{name, mode, input_hex, expected}` where
//! `mode` is `BmpFile` or `BareDib`; successes carry `{width, height, rgba_runs,
//! recovered}` and errors `{error}`. Byte expansion is not needed: no policy
//! vector uses `input_repeat`/`repeat_from`.
//!
//! The public-vs-custom-record equality on the generated differential fixtures
//! (`tools/policy_cases.py`) is a crate unit test, not here: `try_decode_custom_policy`
//! and the record constructors are `pub(crate)` under `#[cfg(test)]` (POLICY-SPEC:
//! the field record is internal and experimental, not a public constructor), so an
//! integration test cannot name them. See `src/policy_tests.rs`.

use std::fs;
use std::path::PathBuf;

use dib_verus::dib::{try_decode_with_policy, BmpMode, BmpProfile, Decoded, Error, TryError};
use serde_json::Value;


/// Fold a `try_*` result into the frozen-typed result for a test: a
/// `Format(e)` is the frozen error, and `OutOfMemory` is a test failure (the
/// tests run without a fault-injecting allocator, so it never occurs).
fn fmt_part(r: Result<Decoded, TryError>) -> Result<Decoded, Error> {
    match r {
        Ok(d) => Ok(d),
        Err(TryError::Format(e)) => Err(e),
        Err(TryError::OutOfMemory(o)) => {
            panic!("unexpected OutOfMemory: site {:?}, elements {}", o.site(), o.elements())
        }
    }
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crate has a parent")
        .to_path_buf()
}

fn vectors_path() -> PathBuf {
    root().join("policy/POLICY-VECTORS.json")
}

fn nibble(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        b'A'..=b'F' => c - b'A' + 10,
        _ => panic!("non-hex byte in vector"),
    }
}

fn hex_decode(s: &str) -> Vec<u8> {
    assert!(s.len() % 2 == 0, "hex string has even length");
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(s.len() / 2);
    let mut i = 0;
    while i < bytes.len() {
        out.push((nibble(bytes[i]) << 4) | nibble(bytes[i + 1]));
        i += 2;
    }
    out
}

fn mode_of(v: &Value) -> BmpMode {
    match v["mode"].as_str().expect("mode") {
        "BmpFile" => BmpMode::BmpFile,
        "BareDib" => BmpMode::BareDib,
        other => panic!("mode {other} has no policy entry point"),
    }
}

fn le16(b: &[u8], o: usize) -> u32 {
    b[o] as u32 | ((b[o + 1] as u32) << 8)
}

fn le32(b: &[u8], o: usize) -> u32 {
    b[o] as u32 | ((b[o + 1] as u32) << 8) | ((b[o + 2] as u32) << 16) | ((b[o + 3] as u32) << 24)
}

/// Stock gdk-pixbuf's `has_alpha` rule, computed here straight from the file's
/// DIB header bytes rather than from any decoder output: `biBitCount == 32`, or
/// `biCompression` is 1 (RLE8) or 2 (RLE4). The DIB starts at 14 for `BmpFile`.
/// The CORE (12-byte) layout puts `biBitCount` at +10 with `biCompression`
/// implied zero; the 16-byte layout puts it at +14 with `biCompression` implied
/// zero; every other accepted layout carries both at +14/+16.
fn header_rule(file: &[u8]) -> bool {
    let s = 14;
    let hsize = le32(file, s);
    let (bits, compression) = if hsize == 12 {
        (le16(file, s + 10), 0)
    } else if hsize == 16 {
        (le16(file, s + 14), 0)
    } else {
        (le16(file, s + 14), le32(file, s + 16))
    };
    bits == 32 || compression == 1 || compression == 2
}

/// The four frozen profile names, in POLICY-SPEC table order, paired with the
/// selector value each names.
const PROFILES: [(&str, BmpProfile); 4] = [
    ("Strict", BmpProfile::Strict),
    ("Recommended", BmpProfile::Recommended),
    ("GdkCompat-2.42.12", BmpProfile::GdkCompat),
    ("ImageCrateCompat-0.25.10", BmpProfile::ImageCrateCompat),
];

/// Expand a `{count, hex}` run list into the full RGBA byte sequence, checking
/// that each `hex` is four bytes and that the counts sum to `total`.
fn expand_runs(runs: &Value, total: usize) -> Vec<u8> {
    let runs = runs.as_array().expect("rgba_runs is an array");
    let mut out = Vec::new();
    let mut sum = 0usize;
    for run in runs {
        let count = run["count"].as_u64().expect("count") as usize;
        assert!(count > 0, "run count is positive");
        let hex = run["hex"].as_str().expect("hex");
        assert_eq!(hex.len(), 8, "eight hex digits per RGBA pixel");
        let pixel = hex_decode(hex);
        for _ in 0..count {
            out.extend_from_slice(&pixel);
        }
        sum += count;
    }
    assert_eq!(sum, total, "run counts sum to width*height");
    out
}

/// Run one policy vector, returning `Err(message)` on any mismatch.
fn check_vector(v: &Value, profile: BmpProfile) -> Result<(), String> {
    let expected = &v["expected"];
    let mode = mode_of(v);
    assert!(
        v.get("input_repeat").is_none() && v.get("repeat_from").is_none(),
        "policy vectors do not repeat"
    );
    let bytes = hex_decode(v["input_hex"].as_str().expect("input_hex"));
    let (result, recovered) = try_decode_with_policy(&bytes, mode, profile);
    let result = fmt_part(result);

    let want_error = expected.get("error").and_then(|e| e.as_str());
    if let Some(want) = want_error {
        return match result {
            Ok(_) => Err(format!("expected error {want}, decode succeeded")),
            Err(e) => {
                if e.name() == want {
                    Ok(())
                } else {
                    Err(format!("error {} != expected {want}", e.name()))
                }
            }
        };
    }

    let decoded = result.map_err(|e: Error| format!("unexpected error {}", e.name()))?;
    let width = expected["width"].as_u64().expect("width") as u32;
    let height = expected["height"].as_u64().expect("height") as u32;
    if decoded.width != width || decoded.height != height {
        return Err(format!(
            "dimensions ({},{}) != expected ({width},{height})",
            decoded.width, decoded.height
        ));
    }
    let n = width as usize * height as usize;
    if decoded.rgba.len() != 4 * n {
        return Err(format!("rgba length {} != 4*{n} = {}", decoded.rgba.len(), 4 * n));
    }
    // `rle_or_32bpp`: the gdk-pixbuf `has_alpha` rule read from the vector's own
    // header bytes, for `BmpFile` at DIB start 14 and for `BareDib` at 0. The
    // rule holds on every policy vector's success, not only under GdkCompat, so
    // all four profiles are covered here.
    let s = match mode {
        BmpMode::BmpFile => 14,
        _ => 0,
    };
    let hsize = le32(&bytes, s);
    let (bits, compression) = if hsize == 12 {
        (le16(&bytes, s + 10), 0)
    } else if hsize == 16 {
        (le16(&bytes, s + 14), 0)
    } else {
        (le16(&bytes, s + 14), le32(&bytes, s + 16))
    };
    let want_alpha = bits == 32 || compression == 1 || compression == 2;
    if decoded.rle_or_32bpp != want_alpha {
        return Err(format!(
            "rle_or_32bpp {} != header rule {want_alpha} (bits {bits}, compression {compression})",
            decoded.rle_or_32bpp
        ));
    }
    let want_recovered = expected["recovered"].as_bool().expect("recovered");
    if recovered != want_recovered {
        return Err(format!("recovered {recovered} != expected {want_recovered}"));
    }
    let want = expand_runs(&expected["rgba_runs"], n);
    if decoded.rgba != want {
        let first = (0..n).find(|&i| decoded.rgba[4 * i..4 * i + 4] != want[4 * i..4 * i + 4]);
        match first {
            Some(i) => Err(format!(
                "rgba differs at pixel {i}: got {:02x?}, expected {:02x?}",
                &decoded.rgba[4 * i..4 * i + 4],
                &want[4 * i..4 * i + 4]
            )),
            None => Err("rgba lengths differ".to_string()),
        }
    } else {
        Ok(())
    }
}

#[test]
fn policy_conformance_all_profiles() {
    let path = vectors_path();
    let text =
        fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let profiles: Value = serde_json::from_str(&text).expect("POLICY-VECTORS.json parses");
    let profiles = profiles.as_object().expect("top-level JSON object");

    for (name, profile) in PROFILES {
        let vectors = profiles
            .get(name)
            .unwrap_or_else(|| panic!("profile {name} present"))
            .as_array()
            .expect("profile vectors are an array");

        let mut passed = 0usize;
        let mut failures: Vec<String> = Vec::new();
        for v in vectors {
            let vname = v["name"].as_str().expect("name").to_string();
            let result =
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| check_vector(v, profile)));
            match result {
                Ok(Ok(())) => passed += 1,
                Ok(Err(msg)) => failures.push(format!("{vname}: {msg}")),
                Err(panic) => {
                    let msg = panic
                        .downcast_ref::<String>()
                        .cloned()
                        .or_else(|| panic.downcast_ref::<&str>().map(|s| s.to_string()))
                        .unwrap_or_else(|| "panicked".to_string());
                    failures.push(format!("{vname}: {msg}"));
                }
            }
        }

        println!("policy conformance: {name}: {passed}/{} vectors passed", vectors.len());
        for f in &failures {
            eprintln!("FAIL [{name}] {f}");
        }
        assert!(failures.is_empty(), "{} {name} vectors failed", failures.len());
        assert_eq!(vectors.len(), 102, "{name} vector count");
        assert_eq!(passed, 102, "all {name} vectors pass");
    }
}

/// Per-profile acceptance over the 71 BMP corpus files. `Strict` additionally
/// matches the frozen oracle answer in `tests/corpus_expected.json` byte for
/// byte; the other three profiles assert their acceptance counts (the oracle's,
/// computed once and recorded here) and that every accepted image is
/// well-formed.
#[test]
fn policy_conformance_corpus_acceptance() {
    let corpus = root().join("corpus");
    let expected_path = root().join("dib_verus/tests/corpus_expected.json");
    if !expected_path.exists() || !corpus.is_dir() {
        eprintln!("policy corpus: skipped (corpus/ or corpus_expected.json missing)");
        return;
    }
    let doc: Value =
        serde_json::from_str(&fs::read_to_string(&expected_path).expect("read corpus_expected"))
            .expect("corpus_expected.json parses");
    let files = doc["files"].as_array().expect("files array");
    let bmp: Vec<&Value> = files
        .iter()
        .filter(|row| row["mode"].as_str().expect("mode") == "BmpFile")
        .collect();
    assert_eq!(bmp.len(), 71, "BMP corpus file count");

    // The oracle's acceptance counts per frozen profile (POLICY-SPEC "Named
    // profiles"); `Strict` matches the shipped 1.1 corpus runner's 50.
    let want_accept: [(&str, BmpProfile, usize); 4] = [
        ("Strict", BmpProfile::Strict, 50),
        ("Recommended", BmpProfile::Recommended, 52),
        ("GdkCompat-2.42.12", BmpProfile::GdkCompat, 58),
        ("ImageCrateCompat-0.25.10", BmpProfile::ImageCrateCompat, 55),
    ];

    let mut failures: Vec<String> = Vec::new();
    for (name, profile, want) in want_accept {
        let mut accepted = 0usize;
        for row in &bmp {
            let file = row["file"].as_str().expect("file");
            let data = match fs::read(corpus.join(file)) {
                Ok(d) => d,
                Err(e) => {
                    failures.push(format!("[{name}] {file}: read failed: {e}"));
                    continue;
                }
            };
            let (result, _recovered) = try_decode_with_policy(&data, BmpMode::BmpFile, profile);
            let result = fmt_part(result);
            match result {
                Ok(d) => {
                    accepted += 1;
                    let n = d.width as usize * d.height as usize;
                    if d.rgba.len() != 4 * n {
                        failures.push(format!(
                            "[{name}] {file}: rgba length {} != 4*{n}",
                            d.rgba.len()
                        ));
                    }
                    // The frozen 1.1 answer is Strict's: on a Strict acceptance
                    // the dimensions and every RGBA byte must match it.
                    if name == "Strict" {
                        let w = row["width"].as_u64().expect("width") as u32;
                        let h = row["height"].as_u64().expect("height") as u32;
                        let want_rgba =
                            hex_decode(row["rgba_hex"].as_str().expect("rgba_hex"));
                        if row["outcome"].as_str().expect("outcome") != "Accepted" {
                            failures.push(format!(
                                "[{name}] {file}: Strict accepted an oracle error row"
                            ));
                        } else if d.width != w || d.height != h {
                            failures.push(format!(
                                "[{name}] {file}: dims ({},{}) != frozen ({w},{h})",
                                d.width, d.height
                            ));
                        } else if d.rgba != want_rgba {
                            failures.push(format!("[{name}] {file}: rgba differs from frozen"));
                        }
                    }
                }
                Err(e) => {
                    // Under Strict the rejection must also name the frozen error.
                    if name == "Strict" {
                        let outcome = row["outcome"].as_str().expect("outcome");
                        if outcome == "Accepted" {
                            failures.push(format!(
                                "[{name}] {file}: frozen accept, got error {}",
                                e.name()
                            ));
                        } else if e.name() != outcome {
                            failures.push(format!(
                                "[{name}] {file}: error {} != frozen {outcome}",
                                e.name()
                            ));
                        }
                    }
                }
            }
        }
        println!("policy corpus: {name}: {accepted}/71 accepted");
        assert_eq!(accepted, want, "{name} corpus acceptance");
    }

    for f in &failures {
        eprintln!("FAIL {f}");
    }
    assert!(failures.is_empty(), "{} corpus mismatches", failures.len());
}

/// Under `GdkCompat` the decoder is the gdk-pixbuf replacement, so the returned
/// `rle_or_32bpp` — the value a loader maps to `has_alpha` — must equal the rule
/// computed here from each file's own `biBitCount`/`biCompression` bytes, on the
/// 71-file BMP corpus, for every file the profile accepts. This is the runtime
/// check that the verified field tracks the header and never a pixel value.
#[test]
fn gdkcompat_alpha_field_matches_header_rule_on_corpus() {
    let corpus = root().join("corpus");
    let expected_path = root().join("dib_verus/tests/corpus_expected.json");
    if !expected_path.exists() || !corpus.is_dir() {
        eprintln!("gdkcompat alpha: skipped (corpus/ or corpus_expected.json missing)");
        return;
    }
    let doc: Value =
        serde_json::from_str(&fs::read_to_string(&expected_path).expect("read corpus_expected"))
            .expect("corpus_expected.json parses");
    let files = doc["files"].as_array().expect("files array");
    let bmp: Vec<&Value> = files
        .iter()
        .filter(|row| row["mode"].as_str().expect("mode") == "BmpFile")
        .collect();
    assert_eq!(bmp.len(), 71, "BMP corpus file count");

    let mut accepted = 0usize;
    let mut failures: Vec<String> = Vec::new();
    for row in &bmp {
        let file = row["file"].as_str().expect("file");
        let data = match fs::read(corpus.join(file)) {
            Ok(d) => d,
            Err(e) => {
                failures.push(format!("{file}: read failed: {e}"));
                continue;
            }
        };
        let want = header_rule(&data);
        match fmt_part(try_decode_with_policy(&data, BmpMode::BmpFile, BmpProfile::GdkCompat).0) {
            Ok(d) => {
                accepted += 1;
                if d.rle_or_32bpp != want {
                    failures.push(format!(
                        "{file}: rle_or_32bpp {} != header rule {want}",
                        d.rle_or_32bpp
                    ));
                }
            }
            // A rejected file returns no `Decoded`, so there is no field to
            // check; GdkCompat's rejections are the same 13 this profile always
            // makes (the acceptance count below pins that).
            Err(_) => {}
        }
    }
    println!("gdkcompat alpha: {accepted}/71 accepted, field checked");
    assert_eq!(accepted, 58, "GdkCompat corpus acceptance");
    for f in &failures {
        eprintln!("FAIL {f}");
    }
    assert!(failures.is_empty(), "{} gdkcompat alpha mismatches", failures.len());
}
