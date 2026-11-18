use inflate_verus::{
    inflate_zlib_into, inflate_zlib_measure, limits::*, InflateError as E, InflateOptions as O,
    SegmentSlices, Segments, Status,
};

fn stored(data: &[u8], declared: u16, trailer: bool) -> Vec<u8> {
    let mut z = vec![0x78, 0x01, 1];
    z.extend(declared.to_le_bytes());
    z.extend((!declared).to_le_bytes());
    z.extend(data);
    if trailer {
        let (mut a, mut b) = (1u32, 0u32);
        for &x in data {
            a = (a + u32::from(x)) % ADLER_MOD;
            b = (b + a) % ADLER_MOD;
        }
        z.extend(((b << 16) | a).to_be_bytes());
    }
    z
}
fn check(z: &[u8], limit: usize, status: Status, expected: &[u8]) {
    let mut parts = vec![&[][..]];
    for byte in z.chunks(1) {
        parts.push(byte);
        parts.push(&[]);
    }
    let mut src = SegmentSlices::new(&parts).unwrap();
    let mut out = vec![0xa5; limit];
    let r = inflate_zlib_into(&mut src, &mut out, O::STRICT);
    assert_eq!(r.status, status);
    assert_eq!(r.produced, expected.len() as u64);
    assert_eq!(&out[..expected.len()], expected);
    assert!(out[expected.len()..].iter().all(|&x| x == 0xa5));
    let mut src = SegmentSlices::new(&parts).unwrap();
    let mut ring = vec![0xcc; WINDOW_MAX];
    let m = inflate_zlib_measure(&mut src, &mut ring, limit as u64, O::STRICT);
    assert_eq!((m.status, m.produced), (r.status, r.produced));
    for j in expected.len().saturating_sub(WINDOW_MAX)..expected.len() {
        assert_eq!(ring[j % WINDOW_MAX], expected[j]);
    }
}

#[test]
fn stored_prefix_and_limit_precedence_d5() {
    check(
        &stored(b"abc", 5, false),
        9,
        Status::Err(E::TruncatedZlib),
        b"abc",
    );
    check(
        &stored(b"abc", 5, false),
        3,
        Status::Err(E::TruncatedZlib),
        b"abc",
    );
    check(&stored(b"abcd", 5, false), 3, Status::Full, b"abc");
    check(&stored(b"abc", 3, true), 3, Status::Done, b"abc");
    check(&stored(b"abc", 3, true), 2, Status::Full, b"ab");
    check(&stored(b"", 0, true), 0, Status::Done, b"");
    check(
        &stored(b"", 1, false),
        0,
        Status::Err(E::TruncatedZlib),
        b"",
    );
    check(&stored(b"a", 1, false), 0, Status::Full, b"");
}
#[test]
fn exact_limit_does_not_skip_adler_or_tail() {
    let good = stored(b"abc", 3, true);
    for n in 0..4 {
        check(
            &good[..good.len() - 4 + n],
            3,
            Status::Err(E::TruncatedZlib),
            b"abc",
        );
    }
    let mut bad = good.clone();
    *bad.last_mut().unwrap() ^= 1;
    check(&bad, 3, Status::Err(E::AdlerMismatch), b"abc");
    let mut trailing = good;
    trailing.push(0);
    check(&trailing, 3, Status::Err(E::TrailingZlibData), b"abc");
    let parts = [&trailing[..]];
    let mut src = SegmentSlices::new(&parts).unwrap();
    let mut out = [0u8; 3];
    assert_eq!(
        inflate_zlib_into(&mut src, &mut out, O::RECOMMENDED_BETA).status,
        Status::Done
    );
    // IgnoreTail applies only after exactly L bytes and a matching Adler-32.
    let mut src = SegmentSlices::new(&parts).unwrap();
    let mut out = [0u8; 4];
    assert_eq!(
        inflate_zlib_into(&mut src, &mut out, O::RECOMMENDED_BETA).status,
        Status::Err(E::TrailingZlibData)
    );
}
#[test]
fn empty_z_and_header_order() {
    check(&[], 0, Status::Err(E::TruncatedZlib), b"");
    check(&[0x78], 1, Status::Err(E::TruncatedZlib), b"");
    check(&[0x78, 0], 0, Status::Err(E::InvalidZlibHeader), b"");
    for cmf in [0x77u8, 0x88, 0x78] {
        let fdict = if cmf == 0x78 { 0x20u16 } else { 0 };
        let base = u16::from(cmf) * 256 + fdict;
        let flag = (fdict + (31 - base % 31) % 31) as u8;
        check(
            &[cmf, flag],
            0,
            Status::Err(if cmf == 0x78 {
                E::ZlibPresetDictionary
            } else {
                E::UnsupportedZlibMethod
            }),
            b"",
        );
    }
}
#[test]
fn eof_cursor_and_empty_segments_are_stable() {
    let parts = [&[][..]; 1024];
    let mut s = SegmentSlices::new(&parts).unwrap();
    for _ in 0..10 {
        assert_eq!(s.read(), None);
        assert_eq!(s.pos(), 0);
    }
    assert_eq!(s.seg_index(), parts.len());
}
#[test]
fn million_empty_segments_preserve_the_stream() {
    let z = stored(b"", 0, true);
    let mut parts = vec![&[][..]; 1_000_000];
    parts.push(&z);
    let mut s = SegmentSlices::new(&parts).unwrap();
    let r = inflate_zlib_into(&mut s, &mut [], O::STRICT);
    assert_eq!(r.status, Status::Done);
    assert_eq!(r.produced, 0);
    assert_eq!(s.seg_index(), parts.len());
    assert_eq!(s.pos(), z.len() as u64);
}
#[test]
fn measuring_limit_is_u64_not_a_slice_length() {
    let z = stored(b"hello", 5, true);
    let parts = [&z[..]];
    let mut s = SegmentSlices::new(&parts).unwrap();
    let mut ring = vec![0xa7; WINDOW_MAX];
    let r = inflate_zlib_measure(&mut s, &mut ring, u64::MAX, O::STRICT);
    assert_eq!(r.status, Status::Done);
    assert_eq!(r.produced, 5);
}
#[test]
fn adler_deferral_and_ring_wrap() {
    // Stored blocks exercise wraparound without relying on a compressor.
    let data = vec![255; u16::MAX as usize];
    let z = stored(&data, u16::MAX, true);
    check(&z, data.len(), Status::Done, &data);
    check(&z, WINDOW_MAX + 7, Status::Full, &data[..WINDOW_MAX + 7]);
}
