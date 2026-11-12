//! Test-only custom records; expectations come from the independent frozen Python oracle.
use crate::dib::{
    Decoded, Error,
    try_decode_custom_policy, try_decode_with_policy, profile_policy_exec, strict_policy_exec,
    BmpMode, TryError,
    BmpProfile, HeadersOptions, HighCountOptions,
    BitfieldOffsetOptions, MasksOptions, MetadataOptions, OffsetOptions, PlanesOptions, Policy,
    ReservedOptions, ShortIndexedGapOptions, UnsupportedOptions, ShortPixelsOptions,
    RleOverrunOptions, RleEolOptions, RleEobOptions, RleDeltaOptions, RleFillOptions,
    PaletteIndexOptions, ScalingOptions, Bitfield16AlphaOptions, IncompleteMetadataOptions,
    HeaderBytesOptions,
};

/// Fold a `try_*` result into the frozen-typed result: `Format(e)` is the
/// frozen error, and `OutOfMemory` is a test failure (no fault-injecting
/// allocator here, so it never occurs).
fn fmt_part(r: Result<Decoded, TryError>) -> Result<Decoded, Error> {
    match r {
        Ok(d) => Ok(d),
        Err(TryError::Format(e)) => Err(e),
        Err(TryError::OutOfMemory(o)) => {
            panic!("unexpected OutOfMemory: site {:?}, elements {}", o.site(), o.elements())
        }
    }
}

use serde_json::Value;
use std::collections::BTreeMap;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn hex_decode(s: &str) -> Vec<u8> {
    assert_eq!(s.len() % 2, 0);
    s.as_bytes().chunks_exact(2).map(|pair| {
        let digit = |b: u8| (b as char).to_digit(16).expect("hex digit") as u8;
        (digit(pair[0]) << 4) | digit(pair[1])
    }).collect()
}

fn le16(b: &[u8], o: usize) -> u32 {
    b[o] as u32 | ((b[o + 1] as u32) << 8)
}

fn le32(b: &[u8], o: usize) -> u32 {
    b[o] as u32 | ((b[o + 1] as u32) << 8) | ((b[o + 2] as u32) << 16) | ((b[o + 3] as u32) << 24)
}

/// The gdk-pixbuf `has_alpha` rule read from the input's own DIB header bytes
/// (start 14 for `BmpFile`, 0 for `BareDib`): `biBitCount == 32` or
/// `biCompression` in {1, 2}. CORE (12) and 16-byte layouts imply
/// `biCompression = 0`. Independent of any decoder output.
fn header_rule(bytes: &[u8], mode: BmpMode) -> bool {
    let s = match mode {
        BmpMode::BmpFile => 14,
        _ => 0,
    };
    let hsize = le32(bytes, s);
    let (bits, compression) = if hsize == 12 {
        (le16(bytes, s + 10), 0)
    } else if hsize == 16 {
        (le16(bytes, s + 14), 0)
    } else {
        (le16(bytes, s + 14), le32(bytes, s + 16))
    };
    bits == 32 || compression == 1 || compression == 2
}

fn policy_of(v: &Value) -> Policy {
    let mut p = strict_policy_exec();
    p.file_size = match v["file_size"].as_str().expect("file_size") {
        "AtMostFileLength" => MetadataOptions::Declared,
        "Ignore" => MetadataOptions::Ignore,
        other => panic!("unsupported file_size {other}"),
    };
    p.image_size = match v["image_size"].as_str().expect("image_size") {
        "Exact" => MetadataOptions::Declared,
        "Ignore" => MetadataOptions::Ignore,
        other => panic!("unsupported image_size {other}"),
    };
    p.planes = match v["planes"].as_str().expect("planes") {
        "Must1" => PlanesOptions::Must1,
        "Ignore" => PlanesOptions::Ignore,
        other => panic!("unsupported planes {other}"),
    };
    p.reserved = match v["reserved"].as_str().expect("reserved") {
        "Zero" => ReservedOptions::Zero,
        "Ignore" => ReservedOptions::Ignore,
        other => panic!("unsupported reserved {other}"),
    };
    p.offset = match v["offset"].as_str().expect("offset") {
        "Declared" => OffsetOptions::Declared,
        "AllowAny" => OffsetOptions::AllowAny,
        "Gdk" => OffsetOptions::Gdk,
        other => panic!("unsupported offset {other}"),
    };
    p.high_count = match v["high_count"].as_str().expect("high_count") {
        "BmpTable" => HighCountOptions::BmpTable,
        "Ignore" => HighCountOptions::Ignore,
        "GdkBound" => HighCountOptions::GdkBound,
        other => panic!("unsupported high_count {other}"),
    };
    p.headers = match v["headers"].as_str().expect("headers") {
        "All1_1" => HeadersOptions::All1_1,
        "No16" => HeadersOptions::No16,
        "No16No64" => HeadersOptions::No16No64,
        other => panic!("unsupported headers {other}"),
    };
    p.masks = match v["masks"].as_str().expect("masks") {
        "Reject" => MasksOptions::Reject,
        "Independent" => MasksOptions::Independent,
        "DefaultRGB" => MasksOptions::DefaultRGB,
        other => panic!("unsupported masks {other}"),
    };
    p.bitfield_offset = match v["bitfield_offset"].as_str().expect("bitfield_offset") {
        "DeclaredOffset" => BitfieldOffsetOptions::DeclaredOffset,
        "MetadataEnd" => BitfieldOffsetOptions::MetadataEnd,
        other => panic!("unsupported bitfield_offset {other}"),
    };
    p.short_indexed_gap = match v["short_indexed_gap"].as_str().expect("short_indexed_gap") {
        "Decode" => ShortIndexedGapOptions::Decode,
        "Blank" => ShortIndexedGapOptions::Blank,
        other => panic!("unsupported short_indexed_gap {other}"),
    };
    p.unsupported = match v["unsupported"].as_str().expect("unsupported") {
        "Reject" => UnsupportedOptions::Reject,
        "Rgb2" => UnsupportedOptions::Rgb2,
        other => panic!("unsupported unsupported {other}"),
    };
    p.short_pixels = match v["short_pixels"].as_str().expect("short_pixels") {
        "Reject" => ShortPixelsOptions::Reject,
        "ZeroFill" => ShortPixelsOptions::ZeroFill,
        other => panic!("unsupported short_pixels {other}"),
    };
    p.rle_overrun = match v["rle_overrun"].as_str().expect("rle_overrun") {
        "Reject" => RleOverrunOptions::Reject,
        "Clip" => RleOverrunOptions::Clip,
        "Image" => RleOverrunOptions::Image,
        other => panic!("unsupported rle_overrun {other}"),
    };
    p.rle_eol = match v["rle_eol"].as_str().expect("rle_eol") {
        "OnlyFinal" => RleEolOptions::OnlyFinal,
        "StopAtHeight" => RleEolOptions::StopAtHeight,
        other => panic!("unsupported rle_eol {other}"),
    };
    p.rle_eob = match v["rle_eob"].as_str().expect("rle_eob") {
        "Required" => RleEobOptions::Required,
        "AtInputEnd" => RleEobOptions::AtInputEnd,
        other => panic!("unsupported rle_eob {other}"),
    };
    p.rle_delta = match v["rle_delta"].as_str().expect("rle_delta") {
        "Inside" => RleDeltaOptions::Inside,
        "ClampStop" => RleDeltaOptions::ClampStop,
        other => panic!("unsupported rle_delta {other}"),
    };
    p.rle_fill = match v["rle_fill"].as_str().expect("rle_fill") {
        "Palette0" => RleFillOptions::Palette0,
        "OpaqueBlack" => RleFillOptions::OpaqueBlack,
        "TransparentBlack" => RleFillOptions::TransparentBlack,
        other => panic!("unsupported rle_fill {other}"),
    };
    p.palette_index = match v["palette_index"].as_str().expect("palette_index") {
        "Reject" => PaletteIndexOptions::Reject,
        "Black" => PaletteIndexOptions::Black,
        other => panic!("unsupported palette_index {other}"),
    };
    p.scaling = match v["scaling"].as_str().expect("scaling") {
        "Round" => ScalingOptions::Round,
        "Image" => ScalingOptions::Image,
        "Gdk" => ScalingOptions::Gdk,
        other => panic!("unsupported scaling {other}"),
    };
    p.bitfield16_alpha = match v["bitfield16_alpha"].as_str().expect("bitfield16_alpha") {
        "Mask" => Bitfield16AlphaOptions::Mask,
        "Opaque" => Bitfield16AlphaOptions::Opaque,
        other => panic!("unsupported bitfield16_alpha {other}"),
    };
    p.incomplete_metadata = match v["incomplete_metadata"].as_str().expect("incomplete_metadata") {
        "Reject" => IncompleteMetadataOptions::Reject,
        "Blank" => IncompleteMetadataOptions::Blank,
        other => panic!("unsupported incomplete_metadata {other}"),
    };
    p.header_bytes = match v["header_bytes"].as_str().expect("header_bytes") {
        "Full" => HeaderBytesOptions::Full,
        "ParsedFields" => HeaderBytesOptions::ParsedFields,
        other => panic!("unsupported header_bytes {other}"),
    };
    p
}

/// The `BmpMode` a custom or profile row names; ICO modes have no custom record.
fn mode_of(row: &Value) -> BmpMode {
    match row["mode"].as_str().expect("mode") {
        "BmpFile" => BmpMode::BmpFile,
        "BareDib" => BmpMode::BareDib,
        other => panic!("custom policies do not accept {other}"),
    }
}

fn check(row: &Value) -> Result<bool, String> {
    let bytes = hex_decode(row["input_hex"].as_str().expect("input"));
    let mode = mode_of(row);
    let (result, recovered) = try_decode_custom_policy(&bytes, mode, policy_of(&row["policy"]));
    let result = fmt_part(result);
    let want = &row["expected"];
    if let Some(error) = want["error"].as_str() {
        return match result {
            Err(got) if got.name() == error && !recovered => Ok(false),
            Err(got) => Err(format!("error {}, recovered={recovered}; expected {error}, false", got.name())),
            Ok(_) => Err(format!("accepted; expected {error}")),
        };
    }
    let decoded = result.map_err(|e| format!("unexpected {}", e.name()))?;
    if decoded.width as u64 != want["width"].as_u64().expect("width")
        || decoded.height as u64 != want["height"].as_u64().expect("height")
        || recovered != want["recovered"].as_bool().expect("recovered") {
        return Err(format!("shape/recovery mismatch: {}x{}, recovered={recovered}", decoded.width, decoded.height));
    }
    let expected = hex_decode(want["rgba_hex"].as_str().expect("rgba_hex"));
    if decoded.rgba != expected {
        let at = decoded.rgba.iter().zip(&expected).position(|(a,b)| a != b);
        return Err(format!("RGBA mismatch at byte {at:?}; lengths {}/{}", decoded.rgba.len(), expected.len()));
    }
    // The `has_alpha` field on every custom-record success: the rule read from
    // the input's own header bytes.
    let want_alpha = header_rule(&bytes, mode);
    if decoded.rle_or_32bpp != want_alpha {
        return Err(format!(
            "rle_or_32bpp {} != header rule {want_alpha}",
            decoded.rle_or_32bpp
        ));
    }
    Ok(true)
}
/// Check all frozen custom inputs, including every retained option value.
#[test]
fn frozen_custom_vectors() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().expect("parent").to_path_buf();
    let rows: Vec<Value> = serde_json::from_str(
        &std::fs::read_to_string(root.join("policy/CUSTOM-VECTORS.json")).expect("vectors")
    ).expect("custom JSON");
    assert_eq!(rows.len(), 66);
    for row in &rows {
        let mut normalized = row.clone();
        if let Some(runs) = row["expected"]["rgba_runs"].as_array() {
            let hex = runs.iter().map(|run| run["hex"].as_str().expect("hex")
                .repeat(run["count"].as_u64().expect("count") as usize))
                .collect::<String>();
            normalized["expected"]["rgba_hex"] = Value::String(hex);
        }
        let result = check(&normalized);
        assert!(result.is_ok(), "{}: {:?}", row["name"], result.err());
    }
    println!("frozen custom vectors: 66/66 matched");
}

/// The selector a `profile-…/<name>` oracle group names.
fn profile_of_group(group: &str) -> Option<BmpProfile> {
    let name = group.rsplit('/').next().expect("group has a slash");
    match name {
        "Strict" => Some(BmpProfile::Strict),
        "Recommended" => Some(BmpProfile::Recommended),
        "GdkCompat-2.42.12" => Some(BmpProfile::GdkCompat),
        "ImageCrateCompat-0.25.10" => Some(BmpProfile::ImageCrateCompat),
        _ => None,
    }
}

#[test]
fn custom_policies_match_python_oracle() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().expect("parent").to_path_buf();
    let mut child = Command::new("python3")
        .arg("-I").arg(root.join("tools/policy_cases.py"))
        .stdout(Stdio::piped()).stderr(Stdio::inherit()).spawn().expect("Python oracle");
    let stdout = child.stdout.take().expect("oracle stdout");
    let mut groups: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    let mut failures = Vec::new();
    let mut failed = 0usize;
    // The public `decode_with_policy(profile)` result must equal
    // `decode_custom_policy` at that profile's record — the record
    // `profile_policy_exec` produces, which the verifier proves equals
    // `profile_policy(profile)`. The generated fixtures and every prefix of them
    // exercise the two compatibility profiles entry for entry.
    let mut profile_checked = 0usize;
    for line in BufReader::new(stdout).lines() {
        let row: Value = serde_json::from_str(&line.expect("oracle line")).expect("oracle JSON");
        let group = row["group"].as_str().expect("group");
        let counts = groups.entry(group.to_owned()).or_default();
        counts.0 += 1;
        match check(&row) {
            Ok(accepted) => counts.1 += usize::from(accepted),
            Err(message) => {
                failed += 1;
                if failures.len() < 30 {
                    failures.push(format!("{}: {message}; policy={}", row["name"], row["policy"]));
                }
            }
        }
        if group.starts_with("profile-fixtures/") || group.starts_with("profile-prefixes/")
            || group.starts_with("profile-corpus/") {
            let profile = profile_of_group(group).expect("a named profiled group");
            let bytes = hex_decode(row["input_hex"].as_str().expect("input"));
            let mode = mode_of(&row);
            // Field-for-field: the release constructor's record equals the record
            // `policy_of` builds from the oracle's own profile assignment, so the
            // two full assignments cannot diverge.
            if profile_policy_exec(profile) != policy_of(&row["policy"]) {
                failed += 1;
                if failures.len() < 30 {
                    failures.push(format!(
                        "{}: release {profile:?} record != oracle-assigned record (group {group})",
                        row["name"]
                    ));
                }
            }
            let (public_r, public_rec) = try_decode_with_policy(&bytes, mode, profile);
            let public = (fmt_part(public_r), public_rec);
            let (custom_r, custom_rec) = try_decode_custom_policy(&bytes, mode, profile_policy_exec(profile));
            let custom = (fmt_part(custom_r), custom_rec);
            if public != custom {
                failed += 1;
                if failures.len() < 30 {
                    failures.push(format!(
                        "{}: public {profile:?} != custom record (group {group})",
                        row["name"]
                    ));
                }
            }
            profile_checked += 1;
        }
    }
    assert!(child.wait().expect("oracle exit").success(), "Python oracle failed");
    for (group, (total, accepted)) in &groups {
        println!("P2 {group}: {total} checked, {accepted} accepted");
    }
    let fixtures = groups["rgb2"].0 / 2;
    assert!(fixtures >= 39);
    assert_eq!(groups["all-combinations"].0, 1296 * fixtures);
    assert_eq!(groups["coupled-new"].0, 12 * fixtures);
    assert_eq!(groups.keys().filter(|k| k.starts_with("corpus/")).count(), 51);
    assert_eq!(groups["custom-frozen"].0, 66);
    assert_eq!(groups["profile-corpus/GdkCompat-2.42.12"], (71, 58));
    assert_eq!(groups["profile-corpus/ImageCrateCompat-0.25.10"], (71, 55));
    // 71 corpus + 220 fixtures + every proper prefix of each fixture, per profile.
    assert!(profile_checked >= 2 * (71 + 220), "profile equality rows: {profile_checked}");
    println!("profile public==custom rows: {profile_checked}");
    assert_eq!(failed, 0, "custom-policy mismatches:\n{}", failures.join("\n"));
}
