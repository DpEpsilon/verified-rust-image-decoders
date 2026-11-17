//! Runtime non-vacuity for all 1,024 combinations of the ten declared values.
use gif_verus::policy::{DisposalUndefined, ExtSizes, FrameBounds, GceConsumers, GceDuplicate,
    GceOrphan, LzwMin, LzwTail, MissingTrailer, Policy, Trailing};
use gif_verus::{Frames, Info, Profile, TryError};
use serde_json::Value;

#[derive(Debug, PartialEq, Eq)]
struct Frame {
    rgba: Vec<u8>, rect: (u16, u16, u16, u16), delay_cs: u16, disposal: u8,
    user_input: bool, transparent: Option<u8>, interlaced: bool, recovered: bool,
}
fn frames(b: &[u8], p: Option<Policy>) -> (Info, Vec<Frame>, Option<gif_verus::Error>) {
    let mut d = match p {Some(p) => Frames::try_new_with_record(b, p), None => Frames::try_new(b)}.unwrap();
    let info = *d.info();
    let mut fs = Vec::new();
    let mut error = None;
    while let Some(r) = d.next_frame() {
        match r {
            Ok(v) => fs.push(Frame {rgba: v.rgba.to_vec(), rect: v.rect, delay_cs: v.delay_cs,
                disposal: v.disposal, user_input: v.user_input, transparent: v.transparent,
                interlaced: v.interlaced, recovered: v.recovered}),
            Err(TryError::Format(e)) => {error = Some(e); break;}
            Err(e) => panic!("unexpected allocation failure: {e:?}"),
        }
    }
    (info, fs, error)
}
fn record(mask: u16) -> Policy {
    let mut p = Policy::of_profile(Profile::Strict);
    if mask & 1 != 0 {p.frame_bounds = FrameBounds::Clip;}
    if mask & 2 != 0 {p.disposal_undefined = DisposalUndefined::Keep;}
    if mask & 4 != 0 {p.gce_duplicate = GceDuplicate::LastWins;}
    if mask & 8 != 0 {p.gce_consumers = GceConsumers::ImageOnly;}
    if mask & 16 != 0 {p.gce_orphan = GceOrphan::Ignore;}
    if mask & 32 != 0 {p.ext_sizes = ExtSizes::Ignore;}
    if mask & 64 != 0 {p.lzw_min = LzwMin::Range1to11Checked;}
    if mask & 128 != 0 {p.lzw_tail = LzwTail::StopAtRaster;}
    if mask & 256 != 0 {p.missing_trailer = MissingTrailer::AtEof;}
    if mask & 512 != 0 {p.trailing = Trailing::Ignore;}
    p
}
fn hex(s: &str) -> Vec<u8> {
    let nib = |c: u8| match c {b'0'..=b'9' => c - b'0', b'a'..=b'f' => c - b'a' + 10, _ => panic!("non-hex")};
    s.as_bytes().chunks_exact(2).map(|p| 16 * nib(p[0]) + nib(p[1])).collect()
}

#[test]
fn every_declared_value_combination_preserves_strict_stills_streams_and_frame_prefixes() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../spec-frozen/VECTORS.json");
    let vectors: Vec<Value> = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    for name in ["two-colour-m2", "lzw-clear-after-raster-then-eoi", "anim-dispose-previous-middle",
        "netscape-two-loop-subblocks-last-wins", "basic-87a", "anim-lzw-error-in-second-frame-keeps-first"] {
        let v = vectors.iter().find(|v| v["name"] == name).unwrap();
        let b = hex(v["input_hex"].as_str().unwrap());
        let still = gif_verus::try_decode_still(&b).unwrap();
        let strict = frames(&b, None);
        for mask in 0..1024 {
            let p = record(mask);
            let (got, recovered) = gif_verus::gif::try_decode_still_with_record(&b, p).unwrap();
            assert_eq!(got, still, "{name}, record {mask}");
            assert!(!recovered, "{name}, record {mask}");
            let relaxed = frames(&b, Some(p));
            assert_eq!(strict.0, relaxed.0, "{name}, record {mask}");
            assert!(relaxed.1.len() >= strict.1.len(), "{name}, record {mask}");
            assert_eq!(strict.1, relaxed.1[..strict.1.len()], "{name}, record {mask}");
            if strict.2.is_none() {assert_eq!(strict, relaxed, "{name}, record {mask}");}
        }
    }
}
