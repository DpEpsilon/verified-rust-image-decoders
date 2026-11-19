//! Authored frozen-vector witnesses, independent model expectations followed by
//! actual public calls. This module exists only in the verifier configuration.
use crate::{
    inflate_zlib_into, model as m, InflateError as E, InflateOptions, SegmentSlices, Status,
};
use vstd::prelude::*;

verus! {

// Wire words are mathematical bit sequences, never executable decode tables.
pub(crate) open spec fn wire_word(z: Seq<u8>, start: nat, n: nat) -> Seq<bool>
    decreases n,
{
    if n == 0 { Seq::empty() }
    else { wire_word(z, start, (n - 1) as nat).push(m::bit(z, (start + n - 1) as nat)) }
}

pub(crate) proof fn follow_word(lengths: Seq<u8>, z: Seq<u8>, start: nat, s: int, n: nat)
    requires m::lengths_ok(lengths), m::Kraft(lengths) <= crate::limits::CODE_SPACE,
        0 <= s < lengths.len(), 0 < lengths[s], n <= lengths[s],
        start + lengths[s] <= 8 * z.len(),
        wire_word(z, start, lengths[s] as nat) == m::cw(lengths, s).unwrap(),
    ensures m::decode_prefix(lengths, z, start, wire_word(z, start, n))
        == (m::Decoded::Symbol { symbol: s as nat, position: (start + lengths[s]) as nat }),
    decreases lengths[s] - n,
{
    m::lemma_msb_bits_len(m::cw_value(lengths, s), lengths[s] as nat);
    // Every shorter wire prefix is a prefix of the authored canonical word.
    lemma_wire_prefix(z, start, n, lengths[s] as nat);
    if n == lengths[s] {
        m::lemma_decode_prefix_hit(lengths, z, start, wire_word(z, start, n), s);
    } else {
        let bits = wire_word(z, start, n);
        assert(m::prefix(bits, m::cw(lengths, s).unwrap()));
        assert(m::live(lengths, bits));
        assert forall|t: int| !m::exact(lengths, bits, t) by {
            if m::exact(lengths, bits, t) {
                m::lemma_msb_bits_len(m::cw_value(lengths, t), lengths[t] as nat);
                m::lemma_prefix_free(lengths, t, s);
                assert(false);
            }
        }
        m::lemma_decode_prefix_step(lengths, z, start, bits);
        follow_word(lengths, z, start, s, n + 1);
    }
}

pub(crate) proof fn lemma_wire_prefix(z: Seq<u8>, start: nat, a: nat, b: nat)
    requires a <= b,
    ensures wire_word(z, start, a) == wire_word(z, start, b).take(a as int),
        wire_word(z, start, b).len() == b,
    decreases b,
{
    if b > 0 {
        if a < b { lemma_wire_prefix(z, start, a, (b - 1) as nat); }
        else { lemma_wire_prefix(z, start, (b - 1) as nat, (b - 1) as nat); }
        assert(wire_word(z, start, b).len() == b);
        assert(wire_word(z, start, a) =~= wire_word(z, start, b).take(a as int));
    }
}

spec fn fixed_count(n: nat, k: nat) -> nat {
    if n == 7 { if k <= 256 { 0 } else if k <= 280 { (k - 256) as nat } else { 24 } }
    else if n == 8 { if k <= 144 { k } else if k <= 280 { 144 } else { (k - 136) as nat } }
    else if n == 9 { if k <= 144 { 0 } else if k <= 256 { (k - 144) as nat } else { 112 } }
    else { 0 }
}

proof fn fixed_counts(n: nat, k: nat)
    requires k <= crate::limits::FIXED_LIT_COUNT,
    ensures m::count_prefix(m::fixed_lit_lengths(), n, k) == fixed_count(n, k),
    decreases k,
{
    if k > 0 { fixed_counts(n, (k - 1) as nat); }
}

proof fn fixed_next(n: nat)
    requires n <= 9,
    ensures m::next_code(m::fixed_lit_lengths(), n)
        == if n <= 7 { 0nat } else if n == 8 { 48nat } else { 400nat },
    decreases n,
{
    if n > 0 {
        fixed_next((n - 1) as nat);
        fixed_counts((n - 1) as nat, crate::limits::FIXED_LIT_COUNT as nat);
    }
}

pub(crate) proof fn fixed_word(s: int)
    requires 0 <= s < crate::limits::FIXED_LIT_COUNT,
    ensures m::cw(m::fixed_lit_lengths(), s) == Some(m::msb_bits(
        if s < 144 { (48 + s) as nat } else if s < 256 { (400 + s - 144) as nat }
        else if s < 280 { (s - 256) as nat } else { (192 + s - 280) as nat },
        m::fixed_lit_lengths()[s] as nat)),
{
    fixed_next(m::fixed_lit_lengths()[s] as nat);
    fixed_counts(m::fixed_lit_lengths()[s] as nat, s as nat);
}

pub(crate) proof fn fixed_decode(z: Seq<u8>, start: nat, s: int)
    requires 0 <= s < crate::limits::FIXED_LIT_COUNT,
        start + m::fixed_lit_lengths()[s] <= 8 * z.len(),
        wire_word(z, start, m::fixed_lit_lengths()[s] as nat) == m::msb_bits(
            if s < 144 { (48 + s) as nat } else if s < 256 { (400 + s - 144) as nat }
            else if s < 280 { (s - 256) as nat } else { (192 + s - 280) as nat },
            m::fixed_lit_lengths()[s] as nat),
    ensures m::MDec(m::fixed_lit_lengths(), z, start)
        == (m::Decoded::Symbol { symbol: s as nat, position: (start + m::fixed_lit_lengths()[s]) as nat }),
{
    m::lemma_fixed_codes(true);
    fixed_word(s);
    follow_word(m::fixed_lit_lengths(), z, start, s, 0);
}

proof fn dist_counts(n: nat, k: nat)
    requires k <= crate::limits::FIXED_DIST_COUNT,
    ensures m::count_prefix(m::fixed_dist_lengths(), n, k) == if n == 5 { k } else { 0nat },
    decreases k,
{
    if k > 0 { dist_counts(n, (k - 1) as nat); }
}

proof fn dist_next(n: nat)
    requires n <= 5,
    ensures m::next_code(m::fixed_dist_lengths(), n) == 0,
    decreases n,
{
    if n > 0 { dist_next((n - 1) as nat); dist_counts((n - 1) as nat, crate::limits::FIXED_DIST_COUNT as nat); }
}

pub(crate) proof fn fixed_dist_decode(z: Seq<u8>, start: nat, s: int)
    requires 0 <= s < crate::limits::FIXED_DIST_COUNT, start + 5 <= 8 * z.len(),
        wire_word(z, start, 5) == m::msb_bits(s as nat, 5),
    ensures m::MDec(m::fixed_dist_lengths(), z, start)
        == (m::Decoded::Symbol { symbol: s as nat, position: start + 5 }),
{
    m::lemma_fixed_codes(true);
    dist_next(5); dist_counts(5, s as nat);
    assert(m::cw(m::fixed_dist_lengths(), s) == Some(m::msb_bits(s as nat, 5)));
    follow_word(m::fixed_dist_lengths(), z, start, s, 0);
}

/// zlib/empty-idat-only: the extractor supplies Z=[], not no inflate call.
fn empty_idat_only() {
    let wire: [u8; 0] = [];
    proof {
        assert(m::Inflate(seq![], 4, true, false).status == m::InflateStatus::Error(m::InflateFault::TruncatedZlib)) by (compute);
        assert(m::Inflate(seq![], 4, true, false).out == Seq::<u8>::empty()) by (compute);
    }
    let parts: [&[u8]; 1] = [&wire];
    proof {
        reveal_with_fuel(crate::segments::parts_prefix, 2);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 15) by (compute);
        assert(parts@[0]@ == wire@);
        assert(crate::segments::parts_prefix(parts@, 1) =~= wire@);
    }
    let mut source = match SegmentSlices::new(&parts) {
        Some(source) => source,
        None => { proof { assert(false); } return; },
    };
    let mut output = [0xa5u8; 4];
    proof {
        assert(output@ =~= seq![0xa5u8, 0xa5, 0xa5, 0xa5]);
    }
    let ghost initial = output@;
    let result = inflate_zlib_into(&mut source, &mut output, InflateOptions::STRICT);
    proof {
        assert(result.status == Status::Err(E::TruncatedZlib));
        assert(result.produced == 0);
        assert(output@.skip(0) == initial.skip(0));
        assert(output@.skip(0) =~= output@);
        assert(initial.skip(0) =~= initial);
        assert(output@ =~= seq![0xa5u8, 0xa5, 0xa5, 0xa5]);
    }
    check_public(&wire, Status::Err(E::TruncatedZlib), Ghost(Seq::<u8>::empty()));
}

/// deflate/stored-only: two grey scanlines have raw bytes [filter, A, filter, B].
fn stored_only() {
    let wire = [0x78u8, 0x01, 0x01, 0x04, 0x00, 0xfb, 0xff, 0x00, 0x41, 0x00, 0x42, 0x01, 0x09, 0x00, 0x84];
    proof {
        assert(m::Inflate(seq![0x78u8, 0x01, 0x01, 0x04, 0x00, 0xfb, 0xff, 0x00, 0x41, 0x00, 0x42, 0x01, 0x09, 0x00, 0x84], 4, true, false).status == m::InflateStatus::Done) by (compute);
        assert(m::Inflate(seq![0x78u8, 0x01, 0x01, 0x04, 0x00, 0xfb, 0xff, 0x00, 0x41, 0x00, 0x42, 0x01, 0x09, 0x00, 0x84], 4, true, false).out == seq![0u8, 0x41, 0, 0x42]) by (compute);
    }
    let parts: [&[u8]; 1] = [&wire];
    proof {
        reveal_with_fuel(crate::segments::parts_prefix, 2);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 15) by (compute);
        assert(parts@[0]@ == wire@);
        assert(crate::segments::parts_prefix(parts@, 1) =~= wire@);
    }
    let mut source = match SegmentSlices::new(&parts) {
        Some(source) => source,
        None => { proof { assert(false); } return; },
    };
    let mut output = [0xa5u8; 4];
    proof {
        assert(output@ =~= seq![0xa5u8, 0xa5, 0xa5, 0xa5]);
    }
    let ghost initial = output@;
    let result = inflate_zlib_into(&mut source, &mut output, InflateOptions::STRICT);
    proof {
        assert(result.status == Status::Done);
        assert(result.produced == 4);
        assert(output@ == seq![0u8, 0x41, 0, 0x42]);
    }
    let ghost raw = seq![0u8, 0x41, 0, 0x42];
    check_public(&wire, Status::Done, Ghost(raw));
}

/// Reusable witness harness: no expected value comes from an executable oracle.
fn check_public(wire: &[u8], expected: Status, raw: Ghost<Seq<u8>>)
    requires wire.len() <= crate::limits::SEGMENT_BYTES_MAX, raw@.len() <= 4,
        crate::status_eq(expected, m::Inflate(wire@, 4, true, false).status),
        raw@ == m::Inflate(wire@, 4, true, false).out,
{
    let mut output = [0xa5u8; 4];
    check_at(wire, &mut output, expected, raw);
}

pub(crate) fn check_at(wire: &[u8], output: &mut [u8], expected: Status, raw: Ghost<Seq<u8>>)
    requires wire.len() <= crate::limits::SEGMENT_BYTES_MAX, raw@.len() <= old(output).len(),
        crate::status_eq(expected, m::Inflate(wire@, old(output).len() as nat, true, false).status),
        raw@ == m::Inflate(wire@, old(output).len() as nat, true, false).out,
{
    let limit = output.len() as u64;
    let parts: [&[u8]; 1] = [wire];
    proof { reveal_with_fuel(crate::segments::parts_prefix, 2);
        assert(crate::segments::parts_prefix(parts@, 1) =~= wire@); }
    let mut source = match SegmentSlices::new(&parts) {
        Some(source) => source,
        None => { proof { assert(false); } return; },
    };
    let result = inflate_zlib_into(&mut source, output, InflateOptions::STRICT);
    proof {
        assert(result.status == expected);
        assert(result.produced == raw@.len());
        assert(output@.take(result.produced as int) == raw@);
    }
    let mut source = match SegmentSlices::new(&parts) {
        Some(source) => source,
        None => { proof { assert(false); } return; },
    };
    let mut ring = [0xccu8; crate::limits::WINDOW_MAX];
    proof { crate::limits::caps_ok(); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(ring@.len() == crate::limits::WINDOW_MAX); }
    let measured = crate::inflate_zlib_measure(&mut source, &mut ring, limit, InflateOptions::STRICT);
    proof {
        assert(measured.status == expected);
        assert(measured.produced == raw@.len());
        assert(crate::ring_repr(ring@, raw@));
    }
}


/// zlib/cm-7: R=4 for the authored 1x2, 8-bit grey image.
fn zlib_cm_7() {
    let wire = [0x77u8, 0x09, 0x01, 0x04, 0x00, 0xfb, 0xff, 0x00, 0x41, 0x00, 0x42, 0x01, 0x09, 0x00, 0x84];
    proof {
        assert(crate::limits::SEGMENT_BYTES_MAX >= 15) by (compute);
        assert(m::Inflate(seq![0x77u8, 0x09, 0x01, 0x04, 0x00, 0xfb, 0xff, 0x00, 0x41, 0x00, 0x42, 0x01, 0x09, 0x00, 0x84], 4, true, false).status == m::InflateStatus::Error(m::InflateFault::UnsupportedZlibMethod)) by (compute);
        assert(m::Inflate(seq![0x77u8, 0x09, 0x01, 0x04, 0x00, 0xfb, 0xff, 0x00, 0x41, 0x00, 0x42, 0x01, 0x09, 0x00, 0x84], 4, true, false).out == Seq::<u8>::empty()) by (compute);
    }
    let ghost raw = seq![0u8, 0x41, 0, 0x42];
    check_public(&wire, Status::Err(E::UnsupportedZlibMethod), Ghost(Seq::<u8>::empty()));
}

/// zlib/cinfo-8: R=4 for the authored 1x2, 8-bit grey image.
fn zlib_cinfo_8() {
    let wire = [0x88u8, 0x1c, 0x01, 0x04, 0x00, 0xfb, 0xff, 0x00, 0x41, 0x00, 0x42, 0x01, 0x09, 0x00, 0x84];
    proof {
        assert(crate::limits::SEGMENT_BYTES_MAX >= 15) by (compute);
        assert(m::Inflate(seq![0x88u8, 0x1c, 0x01, 0x04, 0x00, 0xfb, 0xff, 0x00, 0x41, 0x00, 0x42, 0x01, 0x09, 0x00, 0x84], 4, true, false).status == m::InflateStatus::Error(m::InflateFault::UnsupportedZlibMethod)) by (compute);
        assert(m::Inflate(seq![0x88u8, 0x1c, 0x01, 0x04, 0x00, 0xfb, 0xff, 0x00, 0x41, 0x00, 0x42, 0x01, 0x09, 0x00, 0x84], 4, true, false).out == Seq::<u8>::empty()) by (compute);
    }
    let ghost raw = seq![0u8, 0x41, 0, 0x42];
    check_public(&wire, Status::Err(E::UnsupportedZlibMethod), Ghost(Seq::<u8>::empty()));
}

/// zlib/fdict: R=4 for the authored 1x2, 8-bit grey image.
fn zlib_fdict() {
    let wire = [0x78u8, 0xbb, 0x00, 0x00, 0x00, 0x01, 0x01, 0x04, 0x00, 0xfb, 0xff, 0x00, 0x41, 0x00, 0x42, 0x01, 0x09, 0x00, 0x84];
    proof {
        assert(crate::limits::SEGMENT_BYTES_MAX >= 19) by (compute);
        assert(m::Inflate(seq![0x78u8, 0xbb, 0x00, 0x00, 0x00, 0x01, 0x01, 0x04, 0x00, 0xfb, 0xff, 0x00, 0x41, 0x00, 0x42, 0x01, 0x09, 0x00, 0x84], 4, true, false).status == m::InflateStatus::Error(m::InflateFault::ZlibPresetDictionary)) by (compute);
        assert(m::Inflate(seq![0x78u8, 0xbb, 0x00, 0x00, 0x00, 0x01, 0x01, 0x04, 0x00, 0xfb, 0xff, 0x00, 0x41, 0x00, 0x42, 0x01, 0x09, 0x00, 0x84], 4, true, false).out == Seq::<u8>::empty()) by (compute);
    }
    let ghost raw = seq![0u8, 0x41, 0, 0x42];
    check_public(&wire, Status::Err(E::ZlibPresetDictionary), Ghost(Seq::<u8>::empty()));
}

/// zlib/one-byte: R=4 for the authored 1x2, 8-bit grey image.
fn zlib_one_byte() {
    let wire = [0x78u8];
    proof {
        assert(crate::limits::SEGMENT_BYTES_MAX >= 1) by (compute);
        assert(m::Inflate(seq![0x78u8], 4, true, false).status == m::InflateStatus::Error(m::InflateFault::TruncatedZlib)) by (compute);
        assert(m::Inflate(seq![0x78u8], 4, true, false).out == Seq::<u8>::empty()) by (compute);
    }
    let ghost raw = seq![0u8, 0x41, 0, 0x42];
    check_public(&wire, Status::Err(E::TruncatedZlib), Ghost(Seq::<u8>::empty()));
}

/// zlib/bad-adler: R=4 for the authored 1x2, 8-bit grey image.
fn zlib_bad_adler() {
    let wire = [0x78u8, 0x01, 0x01, 0x04, 0x00, 0xfb, 0xff, 0x00, 0x41, 0x00, 0x42, 0x00, 0x00, 0x00, 0x01];
    proof {
        assert(crate::limits::SEGMENT_BYTES_MAX >= 15) by (compute);
        assert(m::Inflate(seq![0x78u8, 0x01, 0x01, 0x04, 0x00, 0xfb, 0xff, 0x00, 0x41, 0x00, 0x42, 0x00, 0x00, 0x00, 0x01], 4, true, false).status == m::InflateStatus::Error(m::InflateFault::AdlerMismatch)) by (compute);
        assert(m::Inflate(seq![0x78u8, 0x01, 0x01, 0x04, 0x00, 0xfb, 0xff, 0x00, 0x41, 0x00, 0x42, 0x00, 0x00, 0x00, 0x01], 4, true, false).out == seq![0u8, 0x41, 0, 0x42]) by (compute);
    }
    let ghost raw = seq![0u8, 0x41, 0, 0x42];
    check_public(&wire, Status::Err(E::AdlerMismatch), Ghost(raw));
}

/// zlib/missing-adler: R=4 for the authored 1x2, 8-bit grey image.
fn zlib_missing_adler() {
    let wire = [0x78u8, 0x01, 0x01, 0x04, 0x00, 0xfb, 0xff, 0x00, 0x41, 0x00, 0x42];
    proof {
        assert(crate::limits::SEGMENT_BYTES_MAX >= 11) by (compute);
        assert(m::Inflate(seq![0x78u8, 0x01, 0x01, 0x04, 0x00, 0xfb, 0xff, 0x00, 0x41, 0x00, 0x42], 4, true, false).status == m::InflateStatus::Error(m::InflateFault::TruncatedZlib)) by (compute);
        assert(m::Inflate(seq![0x78u8, 0x01, 0x01, 0x04, 0x00, 0xfb, 0xff, 0x00, 0x41, 0x00, 0x42], 4, true, false).out == seq![0u8, 0x41, 0, 0x42]) by (compute);
    }
    let ghost raw = seq![0u8, 0x41, 0, 0x42];
    check_public(&wire, Status::Err(E::TruncatedZlib), Ghost(raw));
}

/// zlib/partial-adler: R=4 for the authored 1x2, 8-bit grey image.
fn zlib_partial_adler() {
    let wire = [0x78u8, 0x01, 0x01, 0x04, 0x00, 0xfb, 0xff, 0x00, 0x41, 0x00, 0x42, 0x01, 0x09];
    proof {
        assert(crate::limits::SEGMENT_BYTES_MAX >= 13) by (compute);
        assert(m::Inflate(seq![0x78u8, 0x01, 0x01, 0x04, 0x00, 0xfb, 0xff, 0x00, 0x41, 0x00, 0x42, 0x01, 0x09], 4, true, false).status == m::InflateStatus::Error(m::InflateFault::TruncatedZlib)) by (compute);
        assert(m::Inflate(seq![0x78u8, 0x01, 0x01, 0x04, 0x00, 0xfb, 0xff, 0x00, 0x41, 0x00, 0x42, 0x01, 0x09], 4, true, false).out == seq![0u8, 0x41, 0, 0x42]) by (compute);
    }
    let ghost raw = seq![0u8, 0x41, 0, 0x42];
    check_public(&wire, Status::Err(E::TruncatedZlib), Ghost(raw));
}

/// zlib/trailing-byte: R=4 for the authored 1x2, 8-bit grey image.
fn zlib_trailing_byte() {
    let wire = [0x78u8, 0x01, 0x01, 0x04, 0x00, 0xfb, 0xff, 0x00, 0x41, 0x00, 0x42, 0x01, 0x09, 0x00, 0x84, 0x00];
    proof {
        assert(crate::limits::SEGMENT_BYTES_MAX >= 16) by (compute);
        assert(m::Inflate(seq![0x78u8, 0x01, 0x01, 0x04, 0x00, 0xfb, 0xff, 0x00, 0x41, 0x00, 0x42, 0x01, 0x09, 0x00, 0x84, 0x00], 4, true, false).status == m::InflateStatus::Error(m::InflateFault::TrailingZlibData)) by (compute);
        assert(m::Inflate(seq![0x78u8, 0x01, 0x01, 0x04, 0x00, 0xfb, 0xff, 0x00, 0x41, 0x00, 0x42, 0x01, 0x09, 0x00, 0x84, 0x00], 4, true, false).out == seq![0u8, 0x41, 0, 0x42]) by (compute);
    }
    let ghost raw = seq![0u8, 0x41, 0, 0x42];
    check_public(&wire, Status::Err(E::TrailingZlibData), Ghost(raw));
}

/// zlib/second-stream: R=4 for the authored 1x2, 8-bit grey image.
fn zlib_second_stream() {
    let wire = [0x78u8, 0x01, 0x01, 0x04, 0x00, 0xfb, 0xff, 0x00, 0x41, 0x00, 0x42, 0x01, 0x09, 0x00, 0x84, 0x78, 0x01, 0x01, 0x04, 0x00, 0xfb, 0xff, 0x00, 0x41, 0x00, 0x42, 0x01, 0x09, 0x00, 0x84];
    proof {
        assert(crate::limits::SEGMENT_BYTES_MAX >= 30) by (compute);
        assert(m::Inflate(seq![0x78u8, 0x01, 0x01, 0x04, 0x00, 0xfb, 0xff, 0x00, 0x41, 0x00, 0x42, 0x01, 0x09, 0x00, 0x84, 0x78, 0x01, 0x01, 0x04, 0x00, 0xfb, 0xff, 0x00, 0x41, 0x00, 0x42, 0x01, 0x09, 0x00, 0x84], 4, true, false).status == m::InflateStatus::Error(m::InflateFault::TrailingZlibData)) by (compute);
        assert(m::Inflate(seq![0x78u8, 0x01, 0x01, 0x04, 0x00, 0xfb, 0xff, 0x00, 0x41, 0x00, 0x42, 0x01, 0x09, 0x00, 0x84, 0x78, 0x01, 0x01, 0x04, 0x00, 0xfb, 0xff, 0x00, 0x41, 0x00, 0x42, 0x01, 0x09, 0x00, 0x84], 4, true, false).out == seq![0u8, 0x41, 0, 0x42]) by (compute);
    }
    let ghost raw = seq![0u8, 0x41, 0, 0x42];
    check_public(&wire, Status::Err(E::TrailingZlibData), Ghost(raw));
}

/// deflate/stored-nlen: R=4 for the authored 1x2, 8-bit grey image.
fn deflate_stored_nlen() {
    let wire = [0x78u8, 0x01, 0x01, 0x04, 0x00, 0xfa, 0xff, 0x00, 0x41, 0x00, 0x42, 0x01, 0x09, 0x00, 0x84];
    proof {
        assert(crate::limits::SEGMENT_BYTES_MAX >= 15) by (compute);
        assert(m::Inflate(seq![0x78u8, 0x01, 0x01, 0x04, 0x00, 0xfa, 0xff, 0x00, 0x41, 0x00, 0x42, 0x01, 0x09, 0x00, 0x84], 4, true, false).status == m::InflateStatus::Error(m::InflateFault::StoredLengthMismatch)) by (compute);
        assert(m::Inflate(seq![0x78u8, 0x01, 0x01, 0x04, 0x00, 0xfa, 0xff, 0x00, 0x41, 0x00, 0x42, 0x01, 0x09, 0x00, 0x84], 4, true, false).out == Seq::<u8>::empty()) by (compute);
    }
    let ghost raw = seq![0u8, 0x41, 0, 0x42];
    check_public(&wire, Status::Err(E::StoredLengthMismatch), Ghost(Seq::<u8>::empty()));
}

/// deflate/fixed-literals: authored raw grey scanlines [0, A, 0, B].
#[verifier::spinoff_prover]
fn fixed_literals() {
    hide(m::MDec);
    hide(m::decode_prefix);
    hide(m::cw);
    hide(m::MCompressedStep);
    hide(m::MCompressed);
    let wire = [0x78u8, 0x01, 0x63, 0x70, 0x64, 0x70, 0x02, 0x00, 0x01, 0x09, 0x00, 0x84];
    proof {
        let z = seq![0x78u8, 0x01, 0x63, 0x70, 0x64, 0x70, 0x02, 0x00, 0x01, 0x09, 0x00, 0x84];
        assert(wire_word(seq![0x78u8, 0x01, 0x63, 0x70, 0x64, 0x70, 0x02, 0x00, 0x01, 0x09, 0x00, 0x84], 19, 8) == m::msb_bits(48, 8)) by (compute);
        assert(wire_word(seq![0x78u8, 0x01, 0x63, 0x70, 0x64, 0x70, 0x02, 0x00, 0x01, 0x09, 0x00, 0x84], 27, 8) == m::msb_bits(113, 8)) by (compute);
        assert(wire_word(seq![0x78u8, 0x01, 0x63, 0x70, 0x64, 0x70, 0x02, 0x00, 0x01, 0x09, 0x00, 0x84], 35, 8) == m::msb_bits(48, 8)) by (compute);
        assert(wire_word(seq![0x78u8, 0x01, 0x63, 0x70, 0x64, 0x70, 0x02, 0x00, 0x01, 0x09, 0x00, 0x84], 43, 8) == m::msb_bits(114, 8)) by (compute);
        assert(wire_word(seq![0x78u8, 0x01, 0x63, 0x70, 0x64, 0x70, 0x02, 0x00, 0x01, 0x09, 0x00, 0x84], 51, 7) == m::msb_bits(0, 7)) by (compute);
        assert(crate::limits::WINDOW_MAX == 32768) by (compute);
        assert(m::pow2(15) == 32768) by (compute);
        fixed_decode(z, 19, 0); fixed_decode(z, 27, 65); fixed_decode(z, 35, 0);
        fixed_decode(z, 43, 66); fixed_decode(z, 51, 256);
        assert(seq![0u8].push(65) =~= seq![0u8, 65]);
        assert(seq![0u8, 65].push(0) =~= seq![0u8, 65, 0]);
        assert(seq![0u8, 65, 0].push(66) =~= seq![0u8, 65, 0, 66]);
        assert(m::MCompressedStep(z, 19, Seq::empty(), 4, 32768, m::fixed_lit_lengths(), m::fixed_dist_lengths())
            == (m::SymbolEffect::Next { out: seq![0u8], position: 27 })) by { reveal(m::MCompressedStep); }
        assert(m::MCompressedStep(z, 27, seq![0u8], 4, 32768, m::fixed_lit_lengths(), m::fixed_dist_lengths())
            == (m::SymbolEffect::Next { out: seq![0u8, 65], position: 35 })) by { reveal(m::MCompressedStep); }
        assert(m::MCompressedStep(z, 35, seq![0u8, 65], 4, 32768, m::fixed_lit_lengths(), m::fixed_dist_lengths())
            == (m::SymbolEffect::Next { out: seq![0u8, 65, 0], position: 43 })) by { reveal(m::MCompressedStep); }
        assert(m::MCompressedStep(z, 43, seq![0u8, 65, 0], 4, 32768, m::fixed_lit_lengths(), m::fixed_dist_lengths())
            == (m::SymbolEffect::Next { out: seq![0u8, 65, 0, 66], position: 51 })) by { reveal(m::MCompressedStep); }
        assert(m::MCompressedStep(z, 51, seq![0u8, 65, 0, 66], 4, 32768, m::fixed_lit_lengths(), m::fixed_dist_lengths())
            == (m::SymbolEffect::End { out: seq![0u8, 65, 0, 66], position: 58 })) by { reveal(m::MCompressedStep); }
        assert(m::MCompressed(z, 19, Seq::empty(), 4, 32768, m::fixed_lit_lengths(), m::fixed_dist_lengths())
            == (m::StoredEffect::End { out: seq![0u8, 65, 0, 66], position: 58 })) by { reveal_with_fuel(m::MCompressed, 6); }
        assert(m::MFinish(seq![0x78u8, 0x01, 0x63, 0x70, 0x64, 0x70, 0x02, 0x00, 0x01, 0x09, 0x00, 0x84], 58, seq![0u8, 65, 0, 66], 4, false)
            == (m::StoredEffect::End { out: seq![0u8, 65, 0, 66], position: 96 })) by (compute);
        assert(m::field_value(seq![0x78u8, 0x01, 0x63, 0x70, 0x64, 0x70, 0x02, 0x00, 0x01, 0x09, 0x00, 0x84], 16, 1) == 1) by (compute);
        assert(m::field_value(seq![0x78u8, 0x01, 0x63, 0x70, 0x64, 0x70, 0x02, 0x00, 0x01, 0x09, 0x00, 0x84], 17, 2) == 1) by (compute);
        assert(m::MOneBlock(z, 16, Seq::empty(), 4, 32768, true)
            == (true, m::StoredEffect::End { out: seq![0u8, 65, 0, 66], position: 58 }));
        assert(m::MBlk(z, 16, Seq::empty(), 4, 32768, true)
            == (m::StoredEffect::End { out: seq![0u8, 65, 0, 66], position: 58 }));
        assert(m::Inflate(z, 4, true, false).status == m::InflateStatus::Done);
        assert(m::Inflate(z, 4, true, false).out == seq![0u8, 0x41, 0, 0x42]);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 12) by (compute);
    }
    let ghost raw = seq![0u8, 0x41, 0, 0x42];
    check_public(&wire, Status::Done, Ghost(raw));
}

// D5 synthetic companion: two present stored bytes precede a missing third byte.
fn truncated_stored_payload() {
    let wire = [0x78u8, 0x01, 0x01, 0x04, 0x00, 0xfb, 0xff, 0x00, 0x41];
    proof {
        assert(crate::limits::SEGMENT_BYTES_MAX >= 9) by (compute);
        assert(m::Inflate(seq![0x78u8, 0x01, 0x01, 0x04, 0x00, 0xfb, 0xff, 0x00, 0x41], 4, true, false).status == m::InflateStatus::Error(m::InflateFault::TruncatedZlib)) by (compute);
        assert(m::Inflate(seq![0x78u8, 0x01, 0x01, 0x04, 0x00, 0xfb, 0xff, 0x00, 0x41], 4, true, false).out == seq![0u8, 65]) by (compute);
        assert(m::Inflate(seq![0x78u8, 0x01, 0x01, 0x04, 0x00, 0xfb, 0xff, 0x00, 0x41], 2, true, false).status == m::InflateStatus::Error(m::InflateFault::TruncatedZlib)) by (compute);
        assert(m::Inflate(seq![0x78u8, 0x01, 0x01, 0x04, 0x00, 0xfb, 0xff, 0x00, 0x41], 2, true, false).out == seq![0u8, 65]) by (compute);
        assert(m::Inflate(seq![0x78u8, 0x01, 0x01, 0x04, 0x00, 0xfb, 0xff, 0x00, 0x41], 1, true, false).status == m::InflateStatus::Full) by (compute);
        assert(m::Inflate(seq![0x78u8, 0x01, 0x01, 0x04, 0x00, 0xfb, 0xff, 0x00, 0x41], 1, true, false).out == seq![0u8]) by (compute);
    }
    let ghost two = seq![0u8, 65];
    let ghost one = seq![0u8];
    let mut out4 = [0xa5u8; 4];
    check_at(&wire, &mut out4, Status::Err(E::TruncatedZlib), Ghost(two));
    let mut out2 = [0xa5u8; 2];
    check_at(&wire, &mut out2, Status::Err(E::TruncatedZlib), Ghost(two));
    let mut out1 = [0xa5u8; 1];
    check_at(&wire, &mut out1, Status::Full, Ghost(one));
}

} // verus!
