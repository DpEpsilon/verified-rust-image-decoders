//! Custom records and independent oracle comparisons exist only in unit tests.
extern crate std;
use std::prelude::v1::*;
use crate::policy::*;
use crate::tga::{try_decode, try_decode_custom_policy, try_decode_with_policy, Decoded, Error, TryError};
use serde_json::Value;
use std::collections::BTreeMap;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf()
}

fn hex(s: &str) -> Vec<u8> {
    assert_eq!(s.len() % 2, 0);
    s.as_bytes().chunks_exact(2).map(|b| {
        let digit = |c: u8| (c as char).to_digit(16).expect("hex digit") as u8;
        16 * digit(b[0]) + digit(b[1])
    }).collect()
}

fn input(row: &Value) -> Vec<u8> {
    hex(row["input_hex"].as_str().unwrap()).repeat(row["input_repeat"].as_u64().unwrap_or(1) as usize)
}

fn profile(name: &str) -> Profile {
    match name {
        "Strict" => Profile::Strict,
        "RecommendedBeta" => Profile::RecommendedBeta,
        "GdkCompat-2.42.12" => Profile::GdkCompat,
        "ImageCrateCompat-0.25.10" => Profile::ImageCrateCompat,
        _ => panic!("unknown profile {name}"),
    }
}

fn policy(v: &Value) -> Policy {
    macro_rules! field {
        ($name:ident, $kind:ident, $($variant:ident),+) => {
            match v[stringify!($name)].as_str().expect(stringify!($name)) {
                $(stringify!($variant) => $kind::$variant,)+
                other => panic!("unknown {} value {other}", stringify!($name)),
            }
        };
    }
    Policy {
        trailing: field!(trailing, TrailingOptions, FooterOnly, Ignore),
        rle_overrun: field!(rle_overrun, RleOverrunOptions, Reject, ClipPacket, ClipSamples),
        attribute_bits: field!(attribute_bits, AttributeBitsOptions, Exact, Lenient, ImageTable),
        map_header: field!(map_header, MapHeaderOptions, Core, IgnoreAbsent, Gdk, Image),
        depths: field!(depths, DepthsOptions, Core, Gdk, Image),
        alpha: field!(alpha, AlphaOptions, AttributeBits, Stored),
        map16_alpha: field!(map16_alpha, Map16AlphaOptions, Bit15, Opaque),
        rgb5: field!(rgb5, Rgb5Options, Replicate, Round),
        map5: field!(map5, Map5Options, Replicate, Round, GdkSwapShift),
        map_index: field!(map_index, MapIndexOptions, Absolute, ZeroBased),
        index_range: field!(index_range, IndexRangeOptions, Reject, Black),
        interleave: field!(interleave, InterleaveOptions, Reject, Ignore),
        image_type: field!(image_type, ImageTypeOptions, Core, GreyFallback),
    }
}

fn fmt_part(r: Result<Decoded, TryError>) -> Result<Decoded, Error> {
    match r {
        Ok(d) => Ok(d),
        Err(TryError::Format(e)) => Err(e),
        Err(TryError::OutOfMemory(o)) => {
            panic!("unexpected OutOfMemory: site {:?}, elements {}", o.site(), o.elements())
        }
    }
}

fn check(result: &(Result<Decoded, Error>, bool), expected: &Value) -> Result<bool, String> {
    if let Some(error) = expected["error"].as_str() {
        return match &result.0 {
            Err(e) if e.name() == error && !result.1 => Ok(false),
            Err(e) => Err(format!("got {}, recovered={}; expected {error}, false", e.name(), result.1)),
            Ok(_) => Err(format!("accepted; expected {error}")),
        };
    }
    let d = result.0.as_ref().map_err(|e| format!("unexpected {}", e.name()))?;
    if d.width as u64 != expected["width"].as_u64().unwrap()
        || d.height as u64 != expected["height"].as_u64().unwrap()
        || result.1 != expected["recovered"].as_bool().unwrap_or(false) {
        return Err(format!("shape/recovery mismatch: {}x{}, recovered={}", d.width, d.height, result.1));
    }
    let mut offset = 0usize;
    for run in expected["rgba_runs"].as_array().unwrap() {
        let pixel = hex(run["hex"].as_str().unwrap());
        assert_eq!(pixel.len(), 4);
        let count = run["count"].as_u64().unwrap() as usize;
        assert!(count > 0);
        let end = offset + 4 * count;
        if end > d.rgba.len() || d.rgba[offset..end].chunks_exact(4).any(|p| p != pixel) {
            return Err(format!("RGBA run mismatch at byte {offset}"));
        }
        offset = end;
    }
    if offset != d.rgba.len() || offset != 4 * d.width as usize * d.height as usize {
        return Err("output length differs from expected run lengths".into());
    }
    Ok(true)
}

#[test]
fn frozen_policy_vectors() {
    let vectors: Value = serde_json::from_str(&std::fs::read_to_string(root().join("policy/POLICY-VECTORS.json")).unwrap()).unwrap();
    assert_eq!(vectors["profiles"].as_object().unwrap().len(), 4);
    for (name, rows) in vectors["profiles"].as_object().unwrap() {
        let rows = rows.as_array().unwrap();
        assert_eq!(rows.len(), 302);
        for row in rows {
            let bytes = input(row);
            let (r0, r1) = try_decode_with_policy(&bytes, profile(name));
            let result = (fmt_part(r0), r1);
            assert!(check(&result, &row["expected"]).is_ok(), "{name} {}: {:?}", row["name"], check(&result, &row["expected"]));
        }
        println!("policy conformance {name}: 302/302");
    }
    let rows = vectors["custom"].as_array().unwrap();
    assert_eq!(rows.len(), 28);
    for row in rows {
        let (r0, r1) = try_decode_custom_policy(&input(row), policy(&row["policy"]));
        let result = (fmt_part(r0), r1);
        assert!(check(&result, &row["expected"]).is_ok(), "custom {}: {:?}", row["name"], check(&result, &row["expected"]));
    }
    println!("custom frozen conformance: 28/28");
}

#[test]
fn policies_match_independent_oracle() {
    let mut child = Command::new("python3").arg("-I").arg(root().join("tools/policy_cases.py"))
        .stdout(Stdio::piped()).stderr(Stdio::inherit()).spawn().expect("start oracle");
    let stdout = child.stdout.take().unwrap();
    let mut groups: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    let mut failures = Vec::new();
    let mut failed = 0usize;
    for line in BufReader::new(stdout).lines() {
        let row: Value = serde_json::from_str(&line.expect("oracle line")).expect("oracle JSON");
        let bytes = input(&row);
        let p = policy(&row["policy"]);
        let (r0, r1) = try_decode_custom_policy(&bytes, p);
        let result = (fmt_part(r0), r1);
        let counts = groups.entry(row["group"].as_str().unwrap().to_owned()).or_default();
        counts.0 += 1;
        match check(&result, &row["expected"]) {
            Ok(ok) => counts.1 += usize::from(ok),
            Err(message) => {
                failed += 1;
                if failures.len() < 30 { failures.push(format!("{} {}: {message}; {}", row["group"], row["name"], row["policy"])); }
            }
        }
        if let Some(name) = row["profile"].as_str() {
            let named = profile(name);
            assert_eq!(profile_policy_exec(named), p, "record for {name}");
            let (n0, n1) = try_decode_with_policy(&bytes, named);
            assert_eq!((fmt_part(n0), n1), result, "public/custom {name} {}", row["name"]);
            if name == "Strict" && row["group"] == "profile-core/Strict" {
                assert_eq!(fmt_part(try_decode(&bytes)), result.0, "1.0/Strict {}", row["name"]);
            }
        }
    }
    assert!(child.wait().expect("oracle exit").success());
    for (group, (total, accepted)) in &groups { println!("oracle {group}: {total} checked, {accepted} accepted"); }
    for name in ["Strict", "RecommendedBeta", "GdkCompat-2.42.12", "ImageCrateCompat-0.25.10"] {
        assert_eq!(groups[&format!("profile-fixtures/{name}")].0, 302);
        assert_eq!(groups[&format!("profile-prefixes/{name}")].0, 7578);
        assert_eq!(groups[&format!("profile-core/{name}")].0, 169);
    }
    assert_eq!(groups["custom-frozen"].0, 28);
    assert_eq!(groups["generated/all-fields"].0, 512);
    assert_eq!(groups["coupled/palette"].0, 192 * 24);
    assert_eq!(groups["coupled/header"].0, 36 * 32);
    assert!(groups["coupled/rle"].0 >= 12 * 32);
    assert_eq!(groups.keys().filter(|s| s.starts_with("option/")).count(), 32);
    for (group, (total, _)) in &groups {
        if group.starts_with("option/") { assert_eq!(*total, 398); }
    }
    println!("oracle total: {} checked, {failed} mismatches", groups.values().map(|n| n.0).sum::<usize>());
    assert_eq!(failed, 0, "{}", failures.join("\n"));
}

/// Sol's finding-2 counterexample, as a regression test: a format error can
/// follow a completed clip event, so on an error result the `recovered` flag is
/// `false` while `recovered_policy` (the closed event disjunction) is `true`.
///
/// The input is a 1x1, 24-bit true-colour RLE image (`image_type = 10`) with one
/// repeat packet encoding two pixels (count 2 > N = 1) and one trailing byte,
/// under a record with `rle_overrun = ClipPacket`. The packet clips (setting the
/// clip event), then the suffix check returns `TrailingBytes`. The oracle agrees
/// on the error (`policy_oracle.py:304` raises after `recovered |= over` at
/// `:298`); the flag is not observable through the error path.
#[test]
fn format_error_after_a_clip_has_a_false_flag() {
    let p = Policy {
        trailing: TrailingOptions::FooterOnly,
        rle_overrun: RleOverrunOptions::ClipPacket,
        attribute_bits: AttributeBitsOptions::Exact,
        map_header: MapHeaderOptions::Core,
        depths: DepthsOptions::Core,
        alpha: AlphaOptions::AttributeBits,
        map16_alpha: Map16AlphaOptions::Bit15,
        rgb5: Rgb5Options::Replicate,
        map5: Map5Options::Replicate,
        map_index: MapIndexOptions::Absolute,
        index_range: IndexRangeOptions::Reject,
        interleave: InterleaveOptions::Reject,
        image_type: ImageTypeOptions::Core,
    };
    // 18-byte header: type 10 (RLE true-colour), 1x1, depth 24, top-down; then a
    // repeat packet 0x81 (count 2 > N = 1) with sample 01 00 00 (BGR); then one
    // trailing byte 0x00.
    let bytes = hex("00000a0000000000000000000100010018208101000000");
    let (r0, r1) = try_decode_custom_policy(&bytes, p);
    match r0 {
        Err(TryError::Format(e)) => assert_eq!(e, Error::TrailingBytes),
        other => panic!("expected TrailingBytes, got {:?}", other.map(|d| d.width)),
    }
    assert!(!r1, "an error result reports recovered = false");
}
