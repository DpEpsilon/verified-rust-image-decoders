//! Conformance harness: runs every vector in `spec-frozen/VECTORS.json` against
//! the plain implementation in `src/tga.rs`.
//!
//! Per R16 each vector is `{name, input_hex, expected}`, optionally with
//! `input_repeat`. Successes carry `{width, height, rgba_runs}` where the runs
//! are maximal in top-first row-major order and their counts sum to
//! `width*height`; errors carry `{error}`. Every fixture invokes `decode`.

use std::fs;
use std::path::PathBuf;

use serde_json::Value;
use tga_verus::tga::{decode, Error};

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

/// Expand `input_hex` and then repeat the whole decoded sequence `input_repeat`
/// times (default one).
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

/// Expand a `{count, hex}` run list into the full RGBA byte sequence, checking
/// that each `hex` is four bytes and that the counts sum to `width*height`.
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

/// Run one vector, returning `Err(message)` on any mismatch.
fn check_vector(v: &Value) -> Result<(), String> {
    let expected = &v["expected"];
    let bytes = input_bytes(v);
    let result = decode(&bytes);

    if let Some(want) = expected.get("error").and_then(|e| e.as_str()) {
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
        return Err(format!(
            "rgba length {} != 4*{n} = {}",
            decoded.rgba.len(),
            4 * n
        ));
    }
    let want = expand_runs(&expected["rgba_runs"], n);
    if decoded.rgba != want {
        // Locate the first differing pixel for a useful message.
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
    assert_eq!(vectors.len(), 169, "vector count");
    assert_eq!(passed, 169, "all vectors pass");
}
