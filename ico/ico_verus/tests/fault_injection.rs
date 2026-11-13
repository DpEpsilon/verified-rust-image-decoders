//! Fault-injection test (alloc-delta THEOREMS-ALLOC-DELTA §5): every frozen
//! vector, under eight allocation schedules, at ICO's `EntryList` reservation
//! (through `try_parse`) and at DIB's `Output` reservation inside an ICO decode
//! (through `try_decode_entry`, the `Dib(Output)` event).
//!
//! The allocator lives in `support/fault_alloc.rs`; one serial test owns every
//! injection window, and thread-local state excludes other harness threads. The
//! checks are host runtime observations, not a proof of the physical allocation
//! trace.
//!
//! Scope of the site check: the returned `OutOfMemory` is asserted to carry
//! `AllocSite::EntryList` (parse path) or `AllocSite::Dib(Output)` (decode path)
//! and the exact failed size. This test does not observe ICO's imported logical
//! allocation log: that log is ghost state, which runtime Rust cannot read. The
//! mapping from DIB's log to ICO's log is covered by the verified `import_dib`
//! postcondition, not by this test.

mod support;

use std::fs;
use std::path::PathBuf;

use serde_json::Value;

use ico_verus::ico::{try_decode_entry, try_parse, Entry, EntryDefect, EntryKind, EntryStatus, MAX_INPUT};

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

fn expand_input(v: &Value) -> (usize, Option<Vec<u8>>) {
    let seed = hex_decode(v["input_hex"].as_str().expect("input_hex"));
    let repeat = v.get("input_repeat").and_then(|r| r.as_u64()).unwrap_or(1) as usize;
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
            EntryStatus::Valid { kind, payload: [start, end] }
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

/// One serial test owns all fault windows; the allocator also excludes other
/// threads, so concurrent tests cannot consume its failure countdown.
#[test]
fn serial_fault_injection_all_vectors() {
    let path = vectors_path();
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let vectors: Value = serde_json::from_str(&text).expect("VECTORS.json parses");
    let vectors = vectors.as_array().expect("top-level JSON array");
    assert_eq!(vectors.len(), 217, "vector count");

    let mut parse_cases = 0usize;
    let mut decode_cases = 0usize;

    for v in vectors {
        let operation = v["operation"].as_str().expect("operation");
        let (length, bytes) = expand_input(v);
        // Over-cap fixtures are decided by length alone; the harness materialises
        // a zeroed buffer of the exact expanded length (R2 tests the length
        // first, so its contents never matter).
        let bytes = match bytes {
            Some(b) => b,
            None => {
                assert!(length > MAX_INPUT);
                vec![0u8; length]
            }
        };

        if operation == "parse" {
            support::fault_alloc::check_parse(|| try_parse(&bytes));
            parse_cases += 1;
            continue;
        }

        assert_eq!(operation, "decode", "operation is parse or decode");
        let index = v["index"].as_u64().expect("index") as u32;
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
            match try_parse(&bytes) {
                Ok(e) => {
                    owned_entries = e;
                    &owned_entries
                }
                Err(_) => {
                    // A directory-wide error is reproduced by `try_decode_entry`
                    // with an empty list; the decode path makes no reservation.
                    support::fault_alloc::check_decode(|| try_decode_entry(&bytes, &[], index));
                    decode_cases += 1;
                    continue;
                }
            }
        };
        support::fault_alloc::check_decode(|| try_decode_entry(&bytes, entries, index));
        decode_cases += 1;
    }

    support::fault_alloc::assert_suite_exercised();
    println!(
        "serial faults: {parse_cases} parse + {decode_cases} decode calls; eight allocation schedules each"
    );
    assert_eq!(parse_cases + decode_cases, 217);
}
