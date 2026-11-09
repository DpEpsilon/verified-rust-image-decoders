//! Conformance harness: runs every vector in `spec-frozen/VECTORS.json` against
//! the plain implementation in `src/icns.rs`.
//!
//! Per R18, each vector first runs `parse`; a parse error is reported before any
//! decoding. Missing `color_index`/`mask_index` encode `None` for `combine`.

use std::fs;
use std::path::PathBuf;

use icns_verus::icns::{combine, decode_mask, decode_rgb, parse, Entry, Error};

use serde_json::Value;

fn vectors_path() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .parent()
        .expect("crate has a parent")
        .join("spec-frozen/VECTORS.json")
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

fn input_bytes(v: &Value) -> Vec<u8> {
    let hex = v["input_hex"].as_str().expect("input_hex");
    let repeat = v
        .get("input_repeat")
        .and_then(|r| r.as_u64())
        .unwrap_or(1) as usize;
    let unit = hex_decode(hex);
    let mut bytes = Vec::with_capacity(unit.len() * repeat);
    for _ in 0..repeat {
        bytes.extend_from_slice(&unit);
    }
    bytes
}

/// Expand a `{count, hex}` run list into the full byte sequence, checking that
/// `hex` has the expected fixed width and that the counts sum to `total`.
fn expand_runs(runs: &Value, width: usize, total: usize) -> Vec<u8> {
    let runs = runs.as_array().expect("runs is an array");
    let mut out = Vec::new();
    let mut sum = 0usize;
    for run in runs {
        let count = run["count"].as_u64().expect("count") as usize;
        assert!(count > 0, "run count is positive");
        let hex = run["hex"].as_str().expect("hex");
        assert_eq!(hex.len(), width * 2, "fixed-width pixel hex");
        let pixel = hex_decode(hex);
        for _ in 0..count {
            out.extend_from_slice(&pixel);
        }
        sum += count;
    }
    assert_eq!(sum, total, "run counts sum to side*side");
    out
}

fn check_parse_entries(actual: &[Entry], expected: &Value) {
    let want = expected.as_array().expect("entries array");
    assert_eq!(actual.len(), want.len(), "entry count");
    for (i, (a, w)) in actual.iter().zip(want.iter()).enumerate() {
        assert_eq!(
            a.type_hex,
            w["type_hex"].as_str().unwrap(),
            "entry {i} type_hex"
        );
        let pl = w["payload"].as_array().unwrap();
        assert_eq!(
            a.payload,
            [pl[0].as_u64().unwrap() as u32, pl[1].as_u64().unwrap() as u32],
            "entry {i} payload"
        );
        assert_eq!(
            a.opaque_candidate,
            w["opaque_candidate"].as_bool().unwrap(),
            "entry {i} opaque_candidate"
        );
        let side = match w.get("nominal_side") {
            None | Some(Value::Null) => None,
            Some(x) => Some(x.as_u64().unwrap() as u32),
        };
        assert_eq!(a.nominal_side, side, "entry {i} nominal_side");
    }
}

/// Run a vector, returning `Err(message)` on any mismatch.
fn check_vector(v: &Value) -> Result<(), String> {
    let expected = &v["expected"];
    let bytes = input_bytes(v);
    let op = v["operation"].as_str().expect("operation");

    let entries = match parse(&bytes) {
        Ok(e) => e,
        Err(err) => {
            let want = expected
                .get("error")
                .and_then(|e| e.as_str())
                .ok_or_else(|| "parse returned an error but success was expected".to_string())?;
            if err.name() != want {
                return Err(format!("parse error {} != expected {want}", err.name()));
            }
            return Ok(());
        }
    };

    if let Some(want) = expected.get("error").and_then(|e| e.as_str()) {
        if op == "parse" {
            return Err(format!("expected parse error {want}, parse succeeded"));
        }
        let err = decode(v, &bytes, &entries)
            .err()
            .ok_or_else(|| format!("expected decode error {want}, decode succeeded"))?;
        if err.name() != want {
            return Err(format!("decode error {} != expected {want}", err.name()));
        }
        return Ok(());
    }

    if op == "parse" {
        check_parse_entries(&entries, &expected["entries"]);
        return Ok(());
    }

    let decoded = decode(v, &bytes, &entries).map_err(|e| e.name().to_string())?;
    let side = expected["side"].as_u64().expect("side") as u32;
    if decoded.width != side || decoded.height != side {
        return Err(format!(
            "sides ({},{}) != side {side}",
            decoded.width, decoded.height
        ));
    }
    let n = (side as usize) * (side as usize);
    let (key, width) = if expected.get("rgb_runs").is_some() {
        ("rgb_runs", 3)
    } else if expected.get("alpha_runs").is_some() {
        ("alpha_runs", 1)
    } else {
        ("rgba_runs", 4)
    };
    let want = expand_runs(&expected[key], width, n);
    if decoded.data != want {
        return Err(format!(
            "pixel bytes differ: got {} bytes, expected {} bytes",
            decoded.data.len(),
            want.len()
        ));
    }
    Ok(())
}

fn decode(
    v: &Value,
    bytes: &[u8],
    entries: &[Entry],
) -> Result<icns_verus::icns::Decoded, Error> {
    match v["operation"].as_str().expect("operation") {
        "rgb" => {
            let ci = v["color_index"].as_u64().expect("color_index") as u32;
            decode_rgb(bytes, entries, ci)
        }
        "mask" => {
            let mi = v["mask_index"].as_u64().expect("mask_index") as u32;
            decode_mask(bytes, entries, mi)
        }
        "combine" => {
            let ci = v
                .get("color_index")
                .and_then(|x| x.as_u64())
                .map(|x| x as u32);
            let mi = v
                .get("mask_index")
                .and_then(|x| x.as_u64())
                .map(|x| x as u32);
            combine(bytes, entries, ci, mi)
        }
        other => panic!("unexpected operation for decode: {other}"),
    }
}

#[test]
fn conformance_all_vectors() {
    let path = vectors_path();
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let vectors: Value = serde_json::from_str(&text).expect("VECTORS.json parses");
    let vectors = vectors.as_array().expect("top-level JSON array");

    let mut passed = 0usize;
    let mut failures: Vec<String> = Vec::new();

    for v in vectors {
        let name = v["name"].as_str().expect("name").to_string();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| check_vector(v)));
        match result {
            Ok(Ok(())) => passed += 1,
            Ok(Err(msg)) => failures.push(format!("{name}: {msg}")),
            Err(panic) => {
                let msg = panic
                    .downcast_ref::<String>()
                    .cloned()
                    .or_else(|| panic.downcast_ref::<&str>().map(|s| s.to_string()))
                    .unwrap_or_else(|| "panicked".to_string());
                failures.push(format!("{name}: {msg}"));
            }
        }
    }

    println!("conformance: {passed}/{} vectors passed", vectors.len());
    for f in &failures {
        eprintln!("FAIL {f}");
    }
    assert!(failures.is_empty(), "{} vectors failed", failures.len());
    assert_eq!(vectors.len(), 111, "vector count");
    assert_eq!(passed, 111, "all vectors pass");
}
