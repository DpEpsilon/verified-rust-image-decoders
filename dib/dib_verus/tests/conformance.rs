//! Conformance harness: runs every vector in `spec-1.1-frozen/VECTORS.json`
//! against the decoder in `src/dib.rs`. (The first 214 records are the unchanged
//! 1.0 fixtures; the appended ones exercise `IcoEntryDibSlack`.)
//!
//! Per R16 each vector is `{name, mode, input_hex, expected}` and may add
//! `input_repeat` (whole-input repetition) or the suffix form
//! `repeat_from` + `input_suffix_hex`. Successes carry `{width, height, rgba_runs}`
//! where the runs are maximal in top-first row-major order and their counts sum to
//! `width*height`; errors carry `{error}`. Every fixture invokes `decode`.

use std::fs;
use std::path::PathBuf;

use dib_verus::dib::{decode, Error, Mode};
use serde_json::Value;

fn vectors_path() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .parent()
        .expect("crate has a parent")
        .join("spec-1.1-frozen/VECTORS.json")
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

const MAX_INPUT: usize = 67_108_864;

/// Expand one vector's input per R16. Returns `None` when the expanded length
/// exceeds `MAX_INPUT`: R1 settles such a fixture as `FileTooLarge` without
/// materializing it (the oracle does the same).
fn input_bytes(v: &Value) -> Option<Vec<u8>> {
    let hex = v["input_hex"].as_str().expect("input_hex");
    let raw = hex_decode(hex);
    let repeat = v
        .get("input_repeat")
        .and_then(|r| r.as_u64())
        .unwrap_or(1) as usize;

    if let Some(cut) = v.get("repeat_from").and_then(|c| c.as_u64()) {
        let cut = cut as usize;
        let suffix = hex_decode(v["input_suffix_hex"].as_str().expect("input_suffix_hex"));
        assert!(cut < raw.len(), "repeat_from below input length");
        let length = cut + (raw.len() - cut) * repeat + suffix.len();
        if length > MAX_INPUT {
            return None;
        }
        let mut out = Vec::with_capacity(length);
        out.extend_from_slice(&raw[..cut]);
        let tail = &raw[cut..];
        for _ in 0..repeat {
            out.extend_from_slice(tail);
        }
        out.extend_from_slice(&suffix);
        Some(out)
    } else {
        let length = raw.len() * repeat;
        if length > MAX_INPUT {
            return None;
        }
        let mut out = Vec::with_capacity(length);
        for _ in 0..repeat {
            out.extend_from_slice(&raw);
        }
        Some(out)
    }
}

fn mode_of(v: &Value) -> Mode {
    match v["mode"].as_str().expect("mode") {
        "BmpFile" => Mode::BmpFile,
        "BareDib" => Mode::BareDib,
        "IcoEntryDib" => Mode::IcoEntryDib,
        "IcoEntryDibSlack" => Mode::IcoEntryDibSlack,
        other => panic!("unknown mode {other}"),
    }
}

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

/// Run one vector, returning `Err(message)` on any mismatch.
fn check_vector(v: &Value) -> Result<(), String> {
    let expected = &v["expected"];
    let mode = mode_of(v);

    let want_error = expected.get("error").and_then(|e| e.as_str());
    let result = match input_bytes(v) {
        Some(bytes) => decode(&bytes, mode),
        // R1: over-cap input is `FileTooLarge` in every mode.
        None => return match want_error {
            Some("FileTooLarge") => Ok(()),
            Some(other) => Err(format!("over-cap input: expected error {other}, got FileTooLarge")),
            None => Err("over-cap input but vector expects success".to_string()),
        },
    };

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
    assert_eq!(vectors.len(), 302, "vector count");
    assert_eq!(passed, 302, "all vectors pass");
}
