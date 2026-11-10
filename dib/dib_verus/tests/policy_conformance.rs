//! Policy conformance harness: runs `policy/POLICY-VECTORS.json` against
//! `decode_with_policy` for the two P1 profiles (`Strict`, `Recommended`), all
//! 102 vectors each. This sits next to `tests/conformance.rs` (the unchanged 302
//! frozen 1.1 vectors) and `tests/corpus.rs`.
//!
//! Per POLICY-SPEC a policy vector is `{name, mode, input_hex, expected}` where
//! `mode` is `BmpFile` or `BareDib`; successes carry `{width, height, rgba_runs,
//! recovered}` and errors `{error}`. Byte expansion is not needed: no policy
//! vector uses `input_repeat`/`repeat_from`.

use std::fs;
use std::path::PathBuf;

use dib_verus::dib::{decode_with_policy, BmpMode, BmpProfile, Error};
use serde_json::Value;

fn vectors_path() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .parent()
        .expect("crate has a parent")
        .join("policy/POLICY-VECTORS.json")
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

fn profile_of(name: &str) -> BmpProfile {
    match name {
        "Strict" => BmpProfile::Strict,
        "Recommended" => BmpProfile::Recommended,
        other => panic!("P1 has no profile {other}"),
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

/// Run one policy vector, returning `Err(message)` on any mismatch.
fn check_vector(v: &Value, profile: BmpProfile) -> Result<(), String> {
    let expected = &v["expected"];
    let mode = mode_of(v);
    assert!(
        v.get("input_repeat").is_none() && v.get("repeat_from").is_none(),
        "policy vectors do not repeat"
    );
    let bytes = hex_decode(v["input_hex"].as_str().expect("input_hex"));
    let (result, recovered) = decode_with_policy(&bytes, mode, profile);

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
fn policy_conformance_strict_and_recommended() {
    let path = vectors_path();
    let text =
        fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let profiles: Value = serde_json::from_str(&text).expect("POLICY-VECTORS.json parses");
    let profiles = profiles.as_object().expect("top-level JSON object");

    for name in ["Strict", "Recommended"] {
        let vectors = profiles
            .get(name)
            .unwrap_or_else(|| panic!("profile {name} present"))
            .as_array()
            .expect("profile vectors are an array");
        let profile = profile_of(name);

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
