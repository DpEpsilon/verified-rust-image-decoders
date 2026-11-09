//! Conformance harness for ICNS-CORE-SPEC 1.1.
//!
//! Loads every vector from `spec-frozen/VECTORS.json` (R18), runs it against the crate, and
//! compares the result with the vector's expectation. Every vector runs `parse` first, including
//! decoding operations, and reports any parse error before decoding (R18).

use icns_creusot::{combine, decode_mask, decode_rgb, parse, Entry, Error};
use serde_json::Value;

/// Decode a lowercase contiguous hex string into bytes.
fn hex_to_bytes(s: &str) -> Vec<u8> {
    assert!(s.len() % 2 == 0, "hex string has odd length");
    (0..s.len() / 2)
        .map(|i| u8::from_str_radix(&s[2 * i..2 * i + 2], 16).expect("valid hex"))
        .collect()
}

fn hex_to_fixed(s: &str, width: usize) -> Vec<u8> {
    let b = hex_to_bytes(s);
    assert_eq!(b.len(), width, "pixel run hex must be {width} bytes");
    b
}

/// Build the input slice for a vector: `input_repeat` (default one) copies of `input_hex`.
fn vector_input(v: &Value) -> Vec<u8> {
    let hex = v["input_hex"].as_str().expect("input_hex");
    let repeat = v
        .get("input_repeat")
        .and_then(Value::as_u64)
        .unwrap_or(1);
    let unit = hex_to_bytes(hex);
    let mut out = Vec::with_capacity(unit.len() * repeat as usize);
    for _ in 0..repeat {
        out.extend_from_slice(&unit);
    }
    out
}

/// Compare the parsed entries with the vector's expected `entries` array (R1 fields, exactly).
fn entries_match(actual: &[Entry], expected: &Value) -> Result<(), String> {
    let exp = expected.as_array().expect("entries array");
    if actual.len() != exp.len() {
        return Err(format!(
            "entry count: got {}, want {}",
            actual.len(),
            exp.len()
        ));
    }
    for (i, (a, e)) in actual.iter().zip(exp.iter()).enumerate() {
        let type_hex = e["type_hex"].as_str().expect("type_hex");
        if a.type_hex != type_hex {
            return Err(format!("entry {i}: type_hex got {}, want {}", a.type_hex, type_hex));
        }
        let p = e["payload"].as_array().expect("payload");
        let want_payload = [p[0].as_u64().unwrap() as u32, p[1].as_u64().unwrap() as u32];
        if a.payload != want_payload {
            return Err(format!(
                "entry {i}: payload got {:?}, want {:?}",
                a.payload, want_payload
            ));
        }
        let want_opaque = e["opaque_candidate"].as_bool().expect("opaque_candidate");
        if a.opaque_candidate != want_opaque {
            return Err(format!(
                "entry {i}: opaque_candidate got {}, want {}",
                a.opaque_candidate, want_opaque
            ));
        }
        let want_side = match &e["nominal_side"] {
            Value::Null => None,
            Value::Number(n) => Some(n.as_u64().unwrap() as u32),
            other => panic!("bad nominal_side {other}"),
        };
        if a.nominal_side != want_side {
            return Err(format!(
                "entry {i}: nominal_side got {:?}, want {:?}",
                a.nominal_side, want_side
            ));
        }
    }
    Ok(())
}

/// Expand a `runs` array into the full byte sequence it represents.
fn expand_runs(expected: &Value, key: &str, pixel_width: usize) -> Vec<u8> {
    let runs = expected[key].as_array().expect("runs array must be present");
    let mut out = Vec::new();
    for run in runs {
        let count = run["count"].as_u64().expect("count") as usize;
        assert!(count > 0, "run count must be positive");
        let pixel = hex_to_fixed(run["hex"].as_str().expect("hex"), pixel_width);
        for _ in 0..count {
            out.extend_from_slice(&pixel);
        }
    }
    out
}

/// Run one vector. Returns `Ok(())` on a match, `Err(detail)` on a mismatch.
fn run_vector(v: &Value) -> Result<(), String> {
    let name = v["name"].as_str().unwrap_or("<unnamed>");
    let bytes = vector_input(v);
    let operation = v["operation"].as_str().expect("operation");
    let expected = &v["expected"];

    // R18: parse runs first for every vector; report its error before any decoding.
    let parsed = parse(&bytes);

    if operation == "parse" {
        // If the expectation is an error, compare error names; otherwise compare entries.
        if let Some(err) = expected.get("error") {
            let want = err.as_str().unwrap();
            return match parsed {
                Err(e) => {
                    if e.name() == want {
                        Ok(())
                    } else {
                        Err(format!("{name}: parse error got {}, want {}", e.name(), want))
                    }
                }
                Ok(_) => Err(format!("{name}: parse succeeded, want error {want}")),
            };
        }
        let entries = parsed.map_err(|e| format!("{name}: parse failed with {}", e.name()))?;
        return entries_match(&entries, &expected["entries"]).map_err(|d| format!("{name}: {d}"));
    }

    // Decoding operations: a parse error is reported before decoding.
    let entries = parsed.map_err(|e| format!("{name}: parse failed with {}", e.name()))?;

    let result: Result<(u32, Vec<u8>), Error> = match operation {
        "rgb" => {
            let ci = v["color_index"].as_u64().expect("color_index") as u32;
            decode_rgb(&bytes, &entries, ci).map(|(side, _, px)| (side, px))
        }
        "mask" => {
            let mi = v["mask_index"].as_u64().expect("mask_index") as u32;
            decode_mask(&bytes, &entries, mi).map(|(side, _, a)| (side, a))
        }
        "combine" => {
            let ci = v.get("color_index").and_then(Value::as_u64).map(|x| x as u32);
            let mi = v.get("mask_index").and_then(Value::as_u64).map(|x| x as u32);
            combine(&bytes, &entries, ci, mi).map(|(side, _, px)| (side, px))
        }
        other => return Err(format!("{name}: unknown operation {other}")),
    };

    if let Some(err) = expected.get("error") {
        let want = err.as_str().unwrap();
        return match result {
            Err(e) => {
                if e.name() == want {
                    Ok(())
                } else {
                    Err(format!("{name}: error got {}, want {}", e.name(), want))
                }
            }
            Ok(_) => Err(format!("{name}: decoding succeeded, want error {want}")),
        };
    }

    // Success: compare side and pixel runs.
    let (side, pixels) = result.map_err(|e| format!("{name}: decoding failed with {}", e.name()))?;
    let want_side = expected["side"].as_u64().expect("side") as u32;
    if side != want_side {
        return Err(format!("{name}: side got {side}, want {want_side}"));
    }
    let (key, pixel_width) = if expected.get("rgb_runs").is_some() {
        ("rgb_runs", 3)
    } else if expected.get("alpha_runs").is_some() {
        ("alpha_runs", 1)
    } else if expected.get("rgba_runs").is_some() {
        ("rgba_runs", 4)
    } else {
        return Err(format!("{name}: no pixel runs in expectation"));
    };
    let want_pixels = expand_runs(expected, key, pixel_width);
    let want_len = want_side as usize * want_side as usize * pixel_width;
    if want_pixels.len() != want_len {
        return Err(format!(
            "{name}: expected runs expand to {} bytes, want {}",
            want_pixels.len(),
            want_len
        ));
    }
    if pixels != want_pixels {
        let first = pixels
            .iter()
            .zip(want_pixels.iter())
            .position(|(a, b)| a != b)
            .unwrap_or(0);
        return Err(format!(
            "{name}: pixels differ at byte {first}: got {:?}, want {:?}",
            &pixels[first..(first + 4).min(pixels.len())],
            &want_pixels[first..(first + 4).min(want_pixels.len())]
        ));
    }
    Ok(())
}

#[test]
fn all_vectors() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../spec-frozen/VECTORS.json");
    let text = std::fs::read_to_string(path).expect("read VECTORS.json");
    let vectors: Vec<Value> = serde_json::from_str(&text).expect("parse VECTORS.json");
    assert!(!vectors.is_empty(), "no vectors loaded");

    let mut failures: Vec<String> = Vec::new();
    let total = vectors.len();
    for v in &vectors {
        if let Err(detail) = run_vector(v) {
            failures.push(detail);
        }
    }
    let passed = total - failures.len();
    println!("conformance: {passed}/{total} vectors passed");
    if !failures.is_empty() {
        for f in &failures {
            println!("FAIL {f}");
        }
        panic!("{}/{} vectors passed, {} failed", passed, total, failures.len());
    }
    assert_eq!(passed, 111, "expected 111 vectors, ran {total}");
    println!("conformance: {passed}/{total} vectors passed");
}
