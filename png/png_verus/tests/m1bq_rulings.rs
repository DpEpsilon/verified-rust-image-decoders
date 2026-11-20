//! The council's rulings on the M1b questions (council report M1BQ, decided
//! 2026-11-20; `QUESTIONS.md` Q1, Q2, Q5). Every expected value below is taken
//! from the ruling text, not from `policy_oracle.py`: the inputs are built here
//! from stored-block zlib streams, so no vector file and no third-party byte is
//! involved.
//!
//! - Q1 (b): under `zlib_tail = IgnoreTail` the tail after a matching Adler-32
//!   is ignored only when the output is exactly `R` bytes; a short stream with a
//!   tail is `TrailingZlibData`, a short stream without one `TruncatedImageData`.
//!   Still and fdAT-frame cases, and a record other than the named profile.
//! - Q2 (c): in the frames constructor's walk with APNG on, a bad-CRC fcTL that
//!   `crc_ancillary = SkipExceptFdat` would drop is the APNG structural error
//!   `CrcMismatch` (R4 under `apng_errors = Static`); still decoding and the
//!   fallback's re-walk drop it; mandatory CRC failures are never caught.
//! - Q5b (b): `Frames::try_new(b)` is `Frames::try_new_with_policy(b, Strict)`.
//!
//! `m1bq_policy_vectors_present` checks that `../policy-draft/VECTORS-POLICY.json`
//! holds the RecommendedBeta records phase 2 added (their expectations are
//! checked by `conformance.rs`).

use png_verus::policy::{ApngErrors, CrcAncillary, Policy, ZlibTail, RECOMMENDED_BETA, STRICT};
use png_verus::{Frames, Profile, TryError};
use serde_json::Value;
use std::path::PathBuf;

// ---------------------------------------------------------------- builders

fn crc32(data: &[u8]) -> u32 {
    let mut c = 0xffff_ffffu32;
    for &x in data {
        c ^= x as u32;
        for _ in 0..8 {
            c = if c & 1 != 0 { 0xedb8_8320 ^ (c >> 1) } else { c >> 1 };
        }
    }
    !c
}

fn adler32(data: &[u8]) -> u32 {
    let (mut a, mut b) = (1u32, 0u32);
    for &x in data {
        a = (a + x as u32) % 65521;
        b = (b + a) % 65521;
    }
    (b << 16) | a
}

/// A zlib stream holding `data` in one final stored block.
fn zlib(data: &[u8]) -> Vec<u8> {
    let n = data.len() as u16;
    let mut z = vec![0x78, 0x01, 0x01];
    z.extend_from_slice(&n.to_le_bytes());
    z.extend_from_slice(&(!n).to_le_bytes());
    z.extend_from_slice(data);
    z.extend_from_slice(&adler32(data).to_be_bytes());
    z
}

fn chunk(ty: &[u8; 4], data: &[u8]) -> Vec<u8> {
    let mut c = (data.len() as u32).to_be_bytes().to_vec();
    c.extend_from_slice(ty);
    c.extend_from_slice(data);
    let crc = crc32(&c[4..]);
    c.extend_from_slice(&crc.to_be_bytes());
    c
}

/// As `chunk`, with the stored CRC off by one.
fn bad_crc(ty: &[u8; 4], data: &[u8]) -> Vec<u8> {
    let mut c = chunk(ty, data);
    let n = c.len();
    c[n - 1] ^= 1;
    c
}

const W: u32 = 2;
const H: u32 = 2;

fn png(chunks: &[Vec<u8>]) -> Vec<u8> {
    let mut b = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    let mut ihdr = W.to_be_bytes().to_vec();
    ihdr.extend_from_slice(&H.to_be_bytes());
    ihdr.extend_from_slice(&[8, 0, 0, 0, 0]); // 8-bit greyscale, not interlaced
    b.extend(chunk(b"IHDR", &ihdr));
    for c in chunks {
        b.extend_from_slice(c);
    }
    b.extend(chunk(b"IEND", &[]));
    b
}

/// The filtered data (filter 0) of a 2 × 2 greyscale image: `R = 6` bytes.
fn raw(v: [u8; 4]) -> Vec<u8> {
    vec![0, v[0], v[1], 0, v[2], v[3]]
}

fn rgba(v: [u8; 4]) -> Vec<u8> {
    v.iter().flat_map(|&g| [g, g, g, 255]).collect()
}

fn actl(frames: u32) -> Vec<u8> {
    let mut d = frames.to_be_bytes().to_vec();
    d.extend_from_slice(&0u32.to_be_bytes());
    chunk(b"acTL", &d)
}

/// A full-canvas fcTL payload with sequence number `seq`.
fn fctl_data(seq: u32) -> Vec<u8> {
    let mut d = seq.to_be_bytes().to_vec();
    for v in [W, H, 0, 0] {
        d.extend_from_slice(&v.to_be_bytes());
    }
    d.extend_from_slice(&[0, 1, 0, 10, 0, 0]); // delay 1/10, dispose NONE, blend SOURCE
    d
}

fn fdat(seq: u32, payload: &[u8]) -> Vec<u8> {
    let mut d = seq.to_be_bytes().to_vec();
    d.extend_from_slice(payload);
    chunk(b"fdAT", &d)
}

const DEFAULT: [u8; 4] = [0x10, 0x20, 0x30, 0x40];
const FRAME: [u8; 4] = [0x50, 0x60, 0x70, 0x80];
const OTHER: [u8; 4] = [0x90, 0xa0, 0xb0, 0xc0];

// ---------------------------------------------------------------- runners

fn name(e: &TryError) -> String {
    e.name().to_string()
}

/// The still entry's outcome: `Err(name)` or `Ok((rgba, recovered))`.
fn still(b: &[u8], p: Profile) -> Result<(Vec<u8>, bool), String> {
    match p {
        Profile::Strict => png_verus::try_decode_still(b).map(|s| (s.rgba, false)).map_err(|e| name(&e)),
        _ => png_verus::try_decode_still_with_policy(b, p).map(|(s, r)| (s.rgba, r)).map_err(|e| name(&e)),
    }
}

/// The frames entry: the constructor's error, or every frame's `(rgba,
/// recovered)` and how the sequence ended (`"iend"` or the error's name).
#[derive(Debug, PartialEq)]
enum Fr {
    Ctor(String),
    Frames(Vec<(Vec<u8>, bool)>, String),
}

fn drain(r: Result<Frames<'_>, TryError>) -> Fr {
    let mut d = match r {
        Ok(d) => d,
        Err(e) => return Fr::Ctor(name(&e)),
    };
    let mut v = Vec::new();
    loop {
        match d.try_next() {
            Ok(Some(f)) => v.push((f.rgba.to_vec(), f.recovered)),
            Ok(None) => return Fr::Frames(v, "iend".into()),
            Err(e) => return Fr::Frames(v, name(&e)),
        }
    }
}

fn frames(b: &[u8], p: Profile) -> Fr {
    drain(Frames::try_new_with_policy(b, p))
}

fn frames_rec(b: &[u8], pol: Policy) -> Fr {
    drain(Frames::try_new_with_record(b, pol))
}

fn ctor(e: &str) -> Fr {
    Fr::Ctor(e.into())
}

/// The R4 fallback: the default image as the only frame, `recovered = true`.
fn r4_default() -> Fr {
    Fr::Frames(vec![(rgba(DEFAULT), true)], "iend".into())
}

const RB: Profile = Profile::RecommendedBeta;

/// IgnoreTail with every other field Strict: a record other than the named
/// profile to which the Q1 rule applies (M1BQ-Q1 record scope).
const STRICT_IGNORETAIL: Policy = Policy { zlib_tail: ZlibTail::IgnoreTail, ..STRICT };

// ---------------------------------------------------------------- Q1

/// The image streams of the Q1 cases, with the expected RecommendedBeta (and
/// any IgnoreTail record) result: `None` for success with `recovered = false`.
fn q1_cases() -> Vec<(&'static str, Vec<u8>, Option<&'static str>, &'static str)> {
    let full = raw(DEFAULT);
    let short = &full[..3]; // one row of two: 3 of R = 6 bytes
    let with = |mut z: Vec<u8>, tail: &[u8]| {
        z.extend_from_slice(tail);
        z
    };
    let mut bad_adler = zlib(short);
    let n = bad_adler.len();
    bad_adler[n - 1] ^= 1;
    vec![
        // (case, stream, IgnoreTail result, Strict result)
        ("short stream plus 2 bytes", with(zlib(short), &[0, 0]), Some("TrailingZlibData"), "TrailingZlibData"),
        ("short stream without a tail", zlib(short), Some("TruncatedImageData"), "TruncatedImageData"),
        ("full stream plus 2 bytes", with(zlib(&full), &[0, 0]), None, "TrailingZlibData"),
        ("short stream plus a second complete stream", with(zlib(short), &zlib(&full[3..])), Some("TrailingZlibData"), "TrailingZlibData"),
        ("short stream, wrong Adler-32, plus 1 byte", with(bad_adler, &[0]), Some("AdlerMismatch"), "AdlerMismatch"),
    ]
}

#[test]
fn q1_ignoretail_still() {
    for (case, z, want_rb, want_strict) in q1_cases() {
        let b = png(&[chunk(b"IDAT", &z)]);
        let want = match want_rb {
            Some(e) => Err(e.to_string()),
            None => Ok((rgba(DEFAULT), false)),
        };
        assert_eq!(still(&b, RB), want, "RecommendedBeta still: {case}");
        assert_eq!(still(&b, Profile::Strict), Err(want_strict.to_string()), "Strict still: {case}");
        // The frames constructor decodes the same default image.
        let want_fr = match want_rb {
            Some(e) => ctor(e),
            None => Fr::Frames(vec![(rgba(DEFAULT), false)], "iend".into()),
        };
        assert_eq!(frames(&b, RB), want_fr, "RecommendedBeta frames: {case}");
        assert_eq!(frames_rec(&b, STRICT_IGNORETAIL), want_fr, "Strict + IgnoreTail record: {case}");
    }
}

#[test]
fn q1_ignoretail_fdat_frame() {
    // Default image = frame 0 (fcTL before IDAT); frame 1 is fdAT-based.
    for (case, z, want_rb, want_strict) in q1_cases() {
        let b = png(&[
            actl(2),
            chunk(b"fcTL", &fctl_data(0)),
            chunk(b"IDAT", &zlib(&raw(DEFAULT))),
            chunk(b"fcTL", &fctl_data(1)),
            fdat(2, &z),
        ]);
        let end = |e: Option<&str>| match e {
            Some(e) => Fr::Frames(vec![(rgba(DEFAULT), false)], e.to_string()),
            None => Fr::Frames(vec![(rgba(DEFAULT), false), (rgba(DEFAULT), false)], "iend".into()),
        };
        // "frames end TrailingZlibData" for the short stream with a tail, and
        // likewise for every other case: inflate errors are never R4.
        assert_eq!(frames(&b, RB), end(want_rb), "RecommendedBeta fdAT frame: {case}");
        assert_eq!(frames_rec(&b, STRICT_IGNORETAIL), end(want_rb), "Strict + IgnoreTail fdAT frame: {case}");
        assert_eq!(frames(&b, Profile::Strict), end(Some(want_strict)), "Strict fdAT frame: {case}");
    }
}

// ---------------------------------------------------------------- Q2

/// IHDR, acTL(1), the default image (not part of the animation), fcTL 0, the
/// frame's chunks; `tail` follows fcTL 0's first fdAT.
fn anim(first: Vec<u8>, tail: &[Vec<u8>]) -> Vec<u8> {
    let mut cs = vec![actl(1), chunk(b"IDAT", &zlib(&raw(DEFAULT))), chunk(b"fcTL", &fctl_data(0)), first];
    cs.extend_from_slice(tail);
    png(&cs)
}

/// QUESTIONS.md Q2's splice example: the frame's zlib stream split over two
/// fdAT chunks with a bad-CRC fcTL between them whose sequence number makes
/// the second fdAT the next expected one.
fn splice() -> Vec<u8> {
    let z = zlib(&raw(FRAME));
    let (a, c) = z.split_at(z.len() / 2);
    anim(fdat(1, a), &[bad_crc(b"fcTL", &fctl_data(2)), fdat(2, c)])
}

/// Packet section 3.6's two-complete-streams row: the second stream, behind
/// the bad-CRC fcTL, would otherwise be hidden by IgnoreTail.
fn two_streams() -> Vec<u8> {
    anim(fdat(1, &zlib(&raw(FRAME))), &[bad_crc(b"fcTL", &fctl_data(2)), fdat(2, &zlib(&raw(OTHER)))])
}

/// A bad-CRC fcTL followed by a bad-CRC fdAT.
fn then_bad_fdat() -> Vec<u8> {
    let mut d = 2u32.to_be_bytes().to_vec();
    d.extend_from_slice(&zlib(&raw(OTHER)));
    anim(fdat(1, &zlib(&raw(FRAME))), &[bad_crc(b"fcTL", &fctl_data(2)), bad_crc(b"fdAT", &d)])
}

/// A bad-CRC fcTL followed by a chunk whose type has a non-letter byte.
fn then_bad_framing() -> Vec<u8> {
    anim(fdat(1, &zlib(&raw(FRAME))), &[bad_crc(b"fcTL", &fctl_data(2)), chunk(b"ab1C", &[])])
}

#[test]
fn q2_recommended_beta() {
    for (case, b) in [("splice", splice()), ("two complete streams", two_streams())] {
        assert_eq!(frames(&b, RB), r4_default(), "RecommendedBeta frames: {case}");
        assert_eq!(still(&b, RB), Ok((rgba(DEFAULT), false)), "RecommendedBeta still: {case}");
        assert_eq!(frames(&b, Profile::Strict), ctor("CrcMismatch"), "Strict frames: {case}");
        assert_eq!(still(&b, Profile::Strict), Err("CrcMismatch".into()), "Strict still: {case}");
    }
    // The fallback does not suppress the fdAT CRC check, in either entry.
    let b = then_bad_fdat();
    assert_eq!(frames(&b, RB), ctor("CrcMismatch"));
    assert_eq!(still(&b, RB), Err("CrcMismatch".into()));
    // The fallback's re-walk validates the rest of the chunk stream.
    let b = then_bad_framing();
    assert_eq!(frames(&b, RB), ctor("InvalidChunkType"));
    assert_eq!(still(&b, RB), Err("InvalidChunkType".into()));
    // Control: the same files with good CRCs animate under both profiles.
    let good = {
        let z = zlib(&raw(FRAME));
        let (a, c) = z.split_at(z.len() / 2);
        let mut cs = vec![actl(2), chunk(b"IDAT", &zlib(&raw(DEFAULT))), chunk(b"fcTL", &fctl_data(0)), fdat(1, a)];
        cs.push(fdat(2, c));
        cs.push(chunk(b"fcTL", &fctl_data(3)));
        cs.push(fdat(4, &zlib(&raw(OTHER))));
        png(&cs)
    };
    let want = Fr::Frames(vec![(rgba(FRAME), false), (rgba(OTHER), false)], "iend".into());
    assert_eq!(frames(&good, RB), want);
    assert_eq!(frames(&good, Profile::Strict), want);
}

#[test]
fn q2_mixed_records() {
    let b = splice();
    // RecommendedBeta's fields with apng_errors = Reject: the structural error surfaces.
    let reject = Policy { apng_errors: ApngErrors::Reject, ..RECOMMENDED_BETA };
    assert_eq!(frames_rec(&b, reject), ctor("CrcMismatch"));
    // crc_ancillary = Reject: the fcTL CRC failure is mandatory, and Static does not catch it.
    let mandatory = Policy { crc_ancillary: CrcAncillary::Reject, ..RECOMMENDED_BETA };
    assert_eq!(frames_rec(&b, mandatory), ctor("CrcMismatch"));
    // Static with the tolerated fcTL condition: R4, as the named profile.
    assert_eq!(frames_rec(&b, RECOMMENDED_BETA), r4_default());
    // A bad-CRC fcTL followed by a bad-CRC fdAT is CrcMismatch under both values of apng_errors.
    let b = then_bad_fdat();
    assert_eq!(frames_rec(&b, reject), ctor("CrcMismatch"));
    assert_eq!(frames_rec(&b, RECOMMENDED_BETA), ctor("CrcMismatch"));
}

#[test]
fn q2_bad_actl_stays_dropped() {
    // A bad-CRC acTL in a file with no other APNG chunk: a still, recovered = false.
    let b = png(&[bad_crc(b"acTL", &[0, 0, 0, 1, 0, 0, 0, 0]), chunk(b"IDAT", &zlib(&raw(DEFAULT)))]);
    assert_eq!(frames(&b, RB), Fr::Frames(vec![(rgba(DEFAULT), false)], "iend".into()));
    // With a fcTL and fdAT after it: fcTL without acTL, a structural error, R4.
    let b = png(&[
        bad_crc(b"acTL", &[0, 0, 0, 1, 0, 0, 0, 0]),
        chunk(b"IDAT", &zlib(&raw(DEFAULT))),
        chunk(b"fcTL", &fctl_data(0)),
        fdat(1, &zlib(&raw(FRAME))),
    ]);
    assert_eq!(frames(&b, RB), r4_default());
}

// ---------------------------------------------------------------- Q5b

fn hex_decode(s: &str) -> Vec<u8> {
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
}

fn records(rel: &str) -> Vec<Value> {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join(rel);
    serde_json::from_str(&std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("{}: {e}", p.display()))).unwrap()
}

#[test]
fn q5b_try_new_is_strict() {
    let mut n = 0;
    for rec in records("spec-frozen/VECTORS.json") {
        if rec["entry"] != "frames" {
            continue;
        }
        let b = hex_decode(rec["input_hex"].as_str().unwrap());
        assert_eq!(drain(Frames::try_new(&b)), frames(&b, Profile::Strict), "{}", rec["name"]);
        n += 1;
    }
    assert!(n > 0);
    for b in [splice(), two_streams()] {
        assert_eq!(drain(Frames::try_new(&b)), frames(&b, Profile::Strict));
    }
}

// ---------------------------------------------------------------- pending

/// The RecommendedBeta records the rulings add to VECTORS-POLICY.json (Q1:
/// packet section 2.7's six rows, still and fdAT frame; Q2: the splice and
/// two-complete-streams rows, the bad-fdAT and bad-framing fallbacks, each with
/// its still variant). Their expectations are checked by `conformance.rs`
/// once present; this test only requires that they exist.
#[test]
fn m1bq_policy_vectors_present() {
    let names: Vec<String> = records("policy-draft/VECTORS-POLICY.json")
        .iter()
        .filter(|r| r["profile"] == "RecommendedBeta")
        .map(|r| r["name"].as_str().unwrap().to_string())
        .collect();
    let want = [
        "policy/zlib-tail/short-with-tail/recommended-beta",
        "policy/zlib-tail/short-no-tail/recommended-beta",
        "policy/zlib-tail/full-with-tail/recommended-beta",
        "policy/zlib-tail/short-then-second-stream/recommended-beta",
        "policy/zlib-tail/short-bad-adler-with-tail/recommended-beta",
        "policy/zlib-tail/fdat-short-with-tail/recommended-beta",
        "policy/apng-static/fctl-bad-crc-splice/recommended-beta",
        "policy/apng-static/fctl-bad-crc-splice-still/recommended-beta",
        "policy/apng-static/fctl-bad-crc-two-streams/recommended-beta",
        "policy/apng-static/fctl-bad-crc-two-streams-still/recommended-beta",
        "policy/apng-static/fctl-bad-crc-then-fdat-bad-crc/recommended-beta",
        "policy/apng-static/fctl-bad-crc-then-fdat-bad-crc-still/recommended-beta",
    ];
    let missing: Vec<&str> = want.iter().copied().filter(|w| !names.iter().any(|n| n == w)).collect();
    assert!(missing.is_empty(), "missing records: {missing:?}");
}
