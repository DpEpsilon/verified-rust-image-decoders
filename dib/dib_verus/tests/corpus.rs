//! Corpus regression: run `try_decode` over every `../corpus` file and compare with the
//! frozen Python oracle's answer in `tests/corpus_expected.json`.
//!
//! `.bmp` files go to `try_decode` directly in `BmpFile` mode. `.ico`/`.cur` files are first
//! reduced by the first-DIB selection shim from `../run_corpus.py` (a measurement shim,
//! not part of the core contract) and then decoded in `IcoEntryDib` mode. The expected
//! file was generated once by `tools/corpus_expected.py`, which imports the frozen oracle;
//! the test itself needs no Python.

use std::fs;
use std::path::{Path, PathBuf};

use dib_verus::dib::{try_decode, Decoded, Error, Mode, TryError};
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

fn hex_decode(s: &str) -> Vec<u8> {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len() / 2);
    let mut i = 0;
    while i + 1 < b.len() {
        let hi = (b[i] as char).to_digit(16).expect("hex hi") as u8;
        let lo = (b[i + 1] as char).to_digit(16).expect("hex lo") as u8;
        out.push((hi << 4) | lo);
        i += 2;
    }
    out
}

/// Container shim result: either one DIB slice, or the shim's own rejection kind.
enum Entry {
    Dib(Vec<u8>),
    Kind(&'static str),
}

/// `entry()` from `../run_corpus.py`, transliterated. Picks the first non-PNG member and
/// compensates for two single-entry CUR fixtures that declare an eight-byte length.
fn entry(data: &[u8], cursor: bool) -> Entry {
    if data.len() < 6
        || &data[..2] != b"\x00\x00"
        || (&data[2..4] != b"\x01\x00" && &data[2..4] != b"\x02\x00")
    {
        return Entry::Kind("ContainerInvalid");
    }
    let n = u16::from_le_bytes([data[4], data[5]]) as usize;
    if n == 0 || data.len() < 6 + 16 * n {
        return Entry::Kind("ContainerInvalid");
    }
    let mut png = 0usize;
    for i in 0..n {
        let p = 6 + 16 * i;
        let length = u32::from_le_bytes([data[p + 8], data[p + 9], data[p + 10], data[p + 11]])
            as usize;
        let offset = u32::from_le_bytes([data[p + 12], data[p + 13], data[p + 14], data[p + 15]])
            as usize;
        if length == 0 || offset < 6 + 16 * n || offset + length > data.len() {
            return Entry::Kind("ContainerInvalid");
        }
        let mut len = length;
        if cursor && length == 8 && n == 1 && offset + length < data.len() {
            len = data.len() - offset;
        }
        let content = &data[offset..offset + len];
        if content.starts_with(b"\x89PNG\r\n\x1a\n") {
            png += 1;
            continue;
        }
        return Entry::Dib(content.to_vec());
    }
    if png > 0 {
        Entry::Kind("ContainerPngOnly")
    } else {
        Entry::Kind("ContainerNoDib")
    }
}

fn mode_of(s: &str) -> Mode {
    match s {
        "BmpFile" => Mode::BmpFile,
        "BareDib" => Mode::BareDib,
        "IcoEntryDib" => Mode::IcoEntryDib,
        other => panic!("unknown mode {other}"),
    }
}

#[test]
fn corpus_matches_oracle() {
    // The corpus (Pillow test images) is not redistributed: run tools/fetch_corpus.sh and
    // tools/corpus_expected.py first. Without it this test is skipped.
    let expected_path = root().join("dib_verus/tests/corpus_expected.json");
    if !expected_path.exists() || !root().join("corpus").is_dir() {
        eprintln!("corpus: skipped (corpus/ or tests/corpus_expected.json missing; see README)");
        return;
    }
    let text = fs::read_to_string(&expected_path).expect("read corpus_expected.json");
    let doc: Value = serde_json::from_str(&text).expect("corpus_expected.json parses");
    let files = doc["files"].as_array().expect("files array");

    let corpus = root().join("corpus");
    let mut checked = 0usize;
    let mut accepted = 0usize;
    let mut failures: Vec<String> = Vec::new();

    for row in files {
        let name = row["file"].as_str().expect("file");
        let outcome = row["outcome"].as_str().expect("outcome");
        let mode = mode_of(row["mode"].as_str().expect("mode"));
        let path = corpus.join(name);
        let data = match fs::read(&path) {
            Ok(d) => d,
            Err(e) => {
                failures.push(format!("{name}: read failed: {e}"));
                continue;
            }
        };

        let dib: Vec<u8>;
        let bytes: &[u8] = match mode {
            Mode::BmpFile => &data,
            _ => {
                let cursor = Path::new(name)
                    .extension()
                    .map(|e| e.eq_ignore_ascii_case("cur"))
                    .unwrap_or(false);
                match entry(&data, cursor) {
                    Entry::Dib(d) => {
                        dib = d;
                        &dib
                    }
                    Entry::Kind(kind) => {
                        if outcome == kind {
                            checked += 1;
                        } else {
                            failures.push(format!("{name}: shim kind {kind} != expected {outcome}"));
                        }
                        continue;
                    }
                }
            }
        };

        let result = fmt_part(try_decode(bytes, mode));
        if outcome == "Accepted" {
            accepted += 1;
            match result {
                Err(e) => failures.push(format!("{name}: expected accept, got {}", e.name())),
                Ok(d) => {
                    let width = row["width"].as_u64().expect("width") as u32;
                    let height = row["height"].as_u64().expect("height") as u32;
                    let want = hex_decode(row["rgba_hex"].as_str().expect("rgba_hex"));
                    if d.width != width || d.height != height {
                        failures.push(format!(
                            "{name}: dims ({},{}) != expected ({width},{height})",
                            d.width, d.height
                        ));
                    } else if d.rgba != want {
                        let n = (width as usize) * (height as usize);
                        let first = (0..n)
                            .find(|&i| d.rgba[4 * i..4 * i + 4] != want[4 * i..4 * i + 4]);
                        match first {
                            Some(i) => failures.push(format!(
                                "{name}: rgba differs at pixel {i}: got {:02x?}, expected {:02x?}",
                                &d.rgba[4 * i..4 * i + 4],
                                &want[4 * i..4 * i + 4]
                            )),
                            None => failures.push(format!("{name}: rgba length mismatch")),
                        }
                    }
                }
            }
        } else {
            match result {
                Ok(_) => failures.push(format!("{name}: expected error {outcome}, decode succeeded")),
                Err(e) => {
                    if e.name() != outcome {
                        failures.push(format!("{name}: error {} != expected {outcome}", e.name()));
                    }
                }
            }
        }
        checked += 1;
    }

    println!("corpus: {checked} files checked, {accepted} accepted");
    for f in &failures {
        eprintln!("FAIL {f}");
    }
    assert!(failures.is_empty(), "{} corpus files failed", failures.len());
    assert_eq!(files.len(), 84, "corpus row count");
    assert_eq!(checked, 84, "all corpus files checked");
}
