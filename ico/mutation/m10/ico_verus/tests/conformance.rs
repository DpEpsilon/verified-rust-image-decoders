//! ICO conformance harness: runs every vector in `spec-frozen/VECTORS.json`
//! against the plain implementation in `src/ico.rs`.
//!
//! Per R12 each vector is `{name, input_hex, operation, expected}`, with `index`
//! present exactly for `operation=decode` and an optional `entries_override`.
//! `input_hex` is a seed; `input_repeat` (positive, default 1) repeats the whole
//! seed, and the paired `repeat_from`/`input_suffix_hex` form
//! `seed[:cut] + seed[cut:]^repeat + suffix` with `0 <= cut < len(seed)`.
//!
//! A `parse` vector expects `{entries:[...]}` in R1 schema; a `decode` vector
//! first runs `parse` to construct the supplied entries (unless
//! `entries_override` supplies them) and expects `{width,height,rgba_runs}` or
//! `{error}`. Two vectors expand past the 64 MiB parse cap; R2 decides those by
//! length alone, so the harness materialises a zeroed buffer of the exact
//! expanded length and checks that both entry points report `FileTooLarge`.

use std::fs;
use std::path::PathBuf;

use serde_json::Value;

use ico_verus::ico::{
    decode_entry, parse, Entry, EntryDefect, EntryKind, EntryStatus, Error, MAX_INPUT,
};

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

/// R12 expansion: return `(length, bytes)` where `bytes` is `None` when the
/// expanded length exceeds `MAX_INPUT`, so no over-cap fixture is ever built.
fn expand_input(v: &Value) -> (usize, Option<Vec<u8>>) {
    let seed = hex_decode(v["input_hex"].as_str().expect("input_hex"));
    let repeat = v
        .get("input_repeat")
        .and_then(|r| r.as_u64())
        .unwrap_or(1) as usize;
    assert!(repeat > 0, "input_repeat is positive");

    let (length, build) = if let Some(cut) = v.get("repeat_from").and_then(|c| c.as_u64()) {
        let cut = cut as usize;
        assert!(cut < seed.len(), "0 <= cut < len(seed)");
        let suffix = hex_decode(v["input_suffix_hex"].as_str().expect("input_suffix_hex"));
        let length = cut + (seed.len() - cut) * repeat + suffix.len();
        if length > MAX_INPUT {
            (length, None)
        } else {
            let mut bytes = Vec::with_capacity(length);
            bytes.extend_from_slice(&seed[..cut]);
            for _ in 0..repeat {
                bytes.extend_from_slice(&seed[cut..]);
            }
            bytes.extend_from_slice(&suffix);
            (length, Some(bytes))
        }
    } else {
        assert!(
            v.get("input_suffix_hex").is_none(),
            "input_suffix_hex requires repeat_from"
        );
        let length = seed.len() * repeat;
        if length > MAX_INPUT {
            (length, None)
        } else {
            let mut bytes = Vec::with_capacity(length);
            for _ in 0..repeat {
                bytes.extend_from_slice(&seed);
            }
            (length, Some(bytes))
        }
    };
    (length, build)
}

// ---------------------------------------------------------------------------
// JSON <-> typed records (R1 schema)
// ---------------------------------------------------------------------------

fn opt_u16(v: &Value, key: &str) -> Option<u16> {
    match v.get(key) {
        None => None,
        Some(Value::Null) => None,
        Some(n) => Some(n.as_u64().expect("u16") as u16),
    }
}

fn entry_from_json(v: &Value) -> Entry {
    let status = match v.get("status").expect("status") {
        Value::Object(m) if m.contains_key("invalid") => {
            let name = m["invalid"].as_str().expect("invalid name");
            let defect = match name {
                "InvalidEntryReserved" => EntryDefect::InvalidEntryReserved,
                "EmptyEntry" => EntryDefect::EmptyEntry,
                "EntryOverlapsDirectory" => EntryDefect::EntryOverlapsDirectory,
                "EntryOutOfBounds" => EntryDefect::EntryOutOfBounds,
                other => panic!("unknown EntryDefect {other}"),
            };
            EntryStatus::Invalid(defect)
        }
        Value::Object(m) if m.contains_key("valid") => {
            let valid = &m["valid"];
            let kind = match valid["kind"].as_str().expect("kind") {
                "Png" => EntryKind::Png,
                "Dib" => EntryKind::Dib,
                other => panic!("unknown EntryKind {other}"),
            };
            let payload = valid["payload"].as_array().expect("payload");
            let start = payload[0].as_u64().expect("start") as u32;
            let end = payload[1].as_u64().expect("end") as u32;
            EntryStatus::Valid {
                kind,
                payload: [start, end],
            }
        }
        other => panic!("bad status {other}"),
    };
    Entry {
        width: v["width"].as_u64().expect("width") as u16,
        height: v["height"].as_u64().expect("height") as u16,
        color_count: v["color_count"].as_u64().expect("color_count") as u8,
        raw_offset: v["raw_offset"].as_u64().expect("raw_offset") as u32,
        raw_size: v["raw_size"].as_u64().expect("raw_size") as u32,
        planes: opt_u16(v, "planes"),
        bit_count: opt_u16(v, "bit_count"),
        hotspot_x: opt_u16(v, "hotspot_x"),
        hotspot_y: opt_u16(v, "hotspot_y"),
        status,
    }
}

/// Render a typed record back into R1 JSON so one comparison covers every field.
fn entry_to_json(e: &Entry) -> Value {
    let status = match e.status {
        EntryStatus::Invalid(d) => serde_json::json!({ "invalid": d.name() }),
        EntryStatus::Valid { kind, payload } => serde_json::json!({
            "valid": {
                "kind": match kind { EntryKind::Png => "Png", EntryKind::Dib => "Dib" },
                "payload": [payload[0], payload[1]],
            }
        }),
    };
    serde_json::json!({
        "width": e.width,
        "height": e.height,
        "color_count": e.color_count,
        "raw_offset": e.raw_offset,
        "raw_size": e.raw_size,
        "status": status,
        "planes": e.planes,
        "bit_count": e.bit_count,
        "hotspot_x": e.hotspot_x,
        "hotspot_y": e.hotspot_y,
    })
}

fn rgba_runs_to_bytes(runs: &Value, total: usize) -> Vec<u8> {
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

/// The exact `{error: name}` spelling for an ICO error, including the DIB wrap.
fn error_name(e: &Error) -> String {
    match e {
        Error::Dib(d) => format!("Dib({})", d.name()),
        other => other.name().to_string(),
    }
}

// ---------------------------------------------------------------------------
// Vector execution
// ---------------------------------------------------------------------------

fn check_parse(bytes: &[u8], expected: &Value) -> Result<(), String> {
    if let Some(want) = expected.get("error").and_then(|e| e.as_str()) {
        return match parse(bytes) {
            Ok(entries) => Err(format!("expected error {want}, parsed {} entries", entries.len())),
            Err(e) => {
                let got = error_name(&e);
                if got == want {
                    Ok(())
                } else {
                    Err(format!("error {got} != expected {want}"))
                }
            }
        };
    }
    let want = expected["entries"].as_array().expect("entries array");
    match parse(bytes) {
        Err(e) => Err(format!("expected {} entries, got error {}", want.len(), e.name())),
        Ok(entries) => {
            if entries.len() != want.len() {
                return Err(format!("entry count {} != {}", entries.len(), want.len()));
            }
            for (i, w) in want.iter().enumerate() {
                let got = entry_to_json(&entries[i]);
                let want_json = normalized_entry(w);
                if got != want_json {
                    return Err(format!("entry {i}: {got} != {want_json}"));
                }
            }
            Ok(())
        }
    }
}

/// Normalise a JSON entry so absent optional keys compare equal to `null`.
fn normalized_entry(w: &Value) -> Value {
    entry_to_json(&entry_from_json(w))
}

fn check_decode(
    bytes: &[u8],
    operation_entries: &[Entry],
    index: u32,
    expected: &Value,
) -> Result<(), String> {
    let result = decode_entry(bytes, operation_entries, index);
    if let Some(want) = expected.get("error").and_then(|e| e.as_str()) {
        return match result {
            Ok(_) => Err(format!("expected error {want}, decode succeeded")),
            Err(e) => {
                let got = error_name(&e);
                if got == want {
                    Ok(())
                } else {
                    Err(format!("error {got} != expected {want}"))
                }
            }
        };
    }
    let decoded = result.map_err(|e| format!("unexpected error {}", error_name(&e)))?;
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
    let want = rgba_runs_to_bytes(&expected["rgba_runs"], n);
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

/// Run one vector. Returns `Err(message)` on any mismatch.
fn check_vector(v: &Value) -> Result<(), String> {
    let expected = &v["expected"];
    let operation = v["operation"].as_str().expect("operation");
    let (length, bytes) = expand_input(v);

    // An over-cap fixture is decided by its length alone (R2), for parse and
    // decode alike. The fixture is only materialised here (a zeroed buffer of the
    // exact expanded length): `R2` tests the length first, so its contents never
    // matter, and the two vectors that need it are the only ones whose inputs
    // exceed the cap.
    if length > MAX_INPUT {
        let want = expected["error"].as_str().expect("over-cap expects error");
        if want != "FileTooLarge" {
            return Err(format!("over-cap length {length} but expected {want}"));
        }
        let fixture = vec![0u8; length];
        if operation == "decode" {
            let index = v["index"].as_u64().expect("index") as u32;
            return match decode_entry(&fixture, &[], index) {
                Err(e) if error_name(&e) == "FileTooLarge" => Ok(()),
                other => Err(format!("over-cap decode: {other:?}")),
            };
        }
        return match parse(&fixture) {
            Err(Error::FileTooLarge) => Ok(()),
            other => Err(format!("over-cap parse: {other:?}")),
        };
    }

    let bytes = bytes.expect("under-cap fixture is built");

    if operation == "parse" {
        return check_parse(&bytes, expected);
    }
    assert_eq!(operation, "decode", "operation is parse or decode");
    let index = v["index"].as_u64().expect("index") as u32;

    // R12: an override supplies the typed entries directly and tests totality.
    // Otherwise the supplied entries are exactly `parse`'s ordinary output. When
    // parse itself returns a directory-wide error, that error is the expected
    // result and `decode_entry` must reproduce it, so the harness calls
    // `decode_entry` itself (with an empty list) rather than reporting parse's
    // error.
    let owned_entries: Vec<Entry>;
    let entries: &[Entry] = if let Some(over) = v.get("entries_override") {
        owned_entries = over
            .as_array()
            .expect("entries_override array")
            .iter()
            .map(entry_from_json)
            .collect();
        &owned_entries
    } else {
        match parse(&bytes) {
            Ok(e) => {
                owned_entries = e;
                &owned_entries
            }
            Err(_) => return check_decode(&bytes, &[], index, expected),
        }
    };

    check_decode(&bytes, entries, index, expected)
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
    assert_eq!(vectors.len(), 217, "vector count");
    assert_eq!(passed, 217, "all vectors pass");
}
