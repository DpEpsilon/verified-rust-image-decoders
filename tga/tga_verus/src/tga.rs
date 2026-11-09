//! M2: the TGA-CORE-SPEC 1.0 implementation (R1..R17), inside `verus!`.
//!
//! The executable behaviour is byte-identical to the M1 plain implementation
//! (kept as `tga.rs.m1`): same algorithms, same order of checks, same results.
//! Three std constructs Verus cannot model are replaced by hand-written
//! equivalents (`slice::copy_from_slice` -> four index writes, `[u8]` slice
//! equality against `[u8; 18]` -> an index loop, the `matches!` macro ->
//! explicit `==` disjunctions).
//!
//! Contracts proved here (M2):
//!   * T5: `parse_header(b)` equals the total header model `MH(b)` exactly, as
//!     the success record or as the first error, for every representable `b`.
//!   * T2: on success the header record satisfies T2's bounds (width and height
//!     in `1..=8,192`, `N <= 16,777,216`, `map_bytes <= 262,140`), justified by
//!     `model::mh_bounds` proved from `MH` alone.
//!   * T1: `decode` is total: no panic, no out-of-bounds read or write, no
//!     unchecked `usize` overflow or narrowing, for every representable slice.
//!     The input cap is tested before any input-dependent allocation and the
//!     successful RGBA request is exactly `4*N <= 67,108,864`.

use vstd::prelude::*;

verus! {

/// R1: the input is at most 64 MiB; a larger slice is `FileTooLarge`.
pub const MAX_INPUT: usize = 67_108_864;

/// R6: the RGBA output is at most 64 MiB (`4*width*height`).
pub const MAX_OUTPUT: usize = 67_108_864;

/// R2: each image dimension is between 1 and 8,192 inclusive.
pub const MAX_DIMENSION: u32 = 8_192;

/// The closed, case-sensitive error set of R15. Exactly eighteen variants;
/// `name()` returns the exact spelling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    FileTooLarge,
    TruncatedHeader,
    UnsupportedImageType,
    InvalidColorMapFlag,
    InvalidDimensions,
    UnsupportedInterleave,
    UnsupportedPixelDepth,
    InvalidColorMap,
    ColorMapRange,
    InvalidAttributeBits,
    OutputTooLarge,
    TruncatedImageId,
    TruncatedColorMap,
    TruncatedPixels,
    RleTruncated,
    RleOverrun,
    ColorIndexOutOfRange,
    TrailingBytes,
}

impl Error {
    pub fn name(&self) -> &'static str {
        match self {
            Error::FileTooLarge => "FileTooLarge",
            Error::TruncatedHeader => "TruncatedHeader",
            Error::UnsupportedImageType => "UnsupportedImageType",
            Error::InvalidColorMapFlag => "InvalidColorMapFlag",
            Error::InvalidDimensions => "InvalidDimensions",
            Error::UnsupportedInterleave => "UnsupportedInterleave",
            Error::UnsupportedPixelDepth => "UnsupportedPixelDepth",
            Error::InvalidColorMap => "InvalidColorMap",
            Error::ColorMapRange => "ColorMapRange",
            Error::InvalidAttributeBits => "InvalidAttributeBits",
            Error::OutputTooLarge => "OutputTooLarge",
            Error::TruncatedImageId => "TruncatedImageId",
            Error::TruncatedColorMap => "TruncatedColorMap",
            Error::TruncatedPixels => "TruncatedPixels",
            Error::RleTruncated => "RleTruncated",
            Error::RleOverrun => "RleOverrun",
            Error::ColorIndexOutOfRange => "ColorIndexOutOfRange",
            Error::TrailingBytes => "TrailingBytes",
        }
    }
}

/// A decoded image: exact `u32` dimensions and exactly `4*width*height` bytes
/// of tightly packed, non-premultiplied RGBA8, top row first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decoded {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

/// The R1 header fields, at the exact offsets named in R1. All fields are the
/// raw little-endian `LE16` values; the parser does not narrow them.
struct Header {
    id_length: u8,
    color_map_flag: u8,
    image_type: u8,
    map_first: u32,
    map_length: u32,
    map_entry_depth: u8,
    x_origin: u32,
    y_origin: u32,
    width: u32,
    height: u32,
    pixel_depth: u8,
    descriptor: u8,
}

impl Header {
    spec fn spec_is_indexed(self) -> bool {
        crate::model::is_indexed(self.image_type)
    }

    spec fn spec_is_rle(self) -> bool {
        self.image_type == 9u8 || self.image_type == 10u8 || self.image_type == 11u8
    }

    /// R3: `ceil(depth/8)`, one of 1, 2, 3, 4 for supported image depths.
    spec fn spec_sample_bytes(self) -> int {
        crate::model::sample_bytes_of(self.pixel_depth)
    }

    /// R4/R7: `ceil(map_entry_depth/8)`, one of 1, 2, 3, 4 for a valid map.
    spec fn spec_map_stride(self) -> int {
        crate::model::sample_bytes_of(self.map_entry_depth)
    }

    /// R5: the attribute count of descriptor bits 0..3.
    spec fn spec_attr(self) -> u8 {
        self.descriptor & 0x0fu8
    }

    spec fn spec_flip_x(self) -> bool {
        self.descriptor & 0x10u8 != 0u8
    }

    spec fn spec_flip_y(self) -> bool {
        self.descriptor & 0x20u8 != 0u8
    }

    fn is_indexed(&self) -> (r: bool)
        ensures
            r == self.spec_is_indexed(),
    {
        proof { type_ok_eq(self.image_type); }
        self.image_type == 1 || self.image_type == 9
    }

    fn is_rle(&self) -> (r: bool)
        ensures
            r == self.spec_is_rle(),
    {
        self.image_type == 9 || self.image_type == 10 || self.image_type == 11
    }

    /// R3: `ceil(depth/8)`, one of 1, 2, 3, 4 for supported image depths.
    fn sample_bytes(&self) -> (r: usize)
        ensures
            r as int == self.spec_sample_bytes(),
    {
        ((self.pixel_depth as usize) + 7) / 8
    }

    /// R4/R7: `ceil(map_entry_depth/8)`, one of 1, 2, 3, 4 for a valid map.
    fn map_bytes_per_entry(&self) -> (r: usize)
        ensures
            r as int == self.spec_map_stride(),
    {
        ((self.map_entry_depth as usize) + 7) / 8
    }

    /// R5: the attribute count of descriptor bits 0..3.
    fn attribute_count(&self) -> (r: u8)
        ensures
            r == self.spec_attr(),
    {
        self.descriptor & 0x0f
    }

    fn flip_x(&self) -> (r: bool)
        ensures
            r == self.spec_flip_x(),
    {
        self.descriptor & 0x10 != 0
    }

    fn flip_y(&self) -> (r: bool)
        ensures
            r == self.spec_flip_y(),
    {
        self.descriptor & 0x20 != 0
    }
}

/// `type_ok` is exhausted by the six supported types, so the executable's
/// explicit comparison is equivalent to the model predicate.
proof fn type_ok_eq(t: u8)
    ensures
        (t == 1u8 || t == 2u8 || t == 3u8 || t == 9u8 || t == 10u8 || t == 11u8)
            == crate::model::type_ok(t),
{
    reveal(crate::model::type_ok);
}

/// The facts `parse_header` proves about an accepted record (T2). Everything the
/// pixel decoder needs to justify its indexing and arithmetic is a conjunct
/// here.
spec fn header_inv(h: Header) -> bool {
    &&& crate::model::type_ok(h.image_type)
    &&& h.color_map_flag <= 1u8
    &&& 1 <= (h.width as int) <= 8192
    &&& 1 <= (h.height as int) <= 8192
    &&& h.descriptor & 0xc0u8 == 0u8
    &&& crate::model::depth_ok(h.image_type, h.pixel_depth)
    &&& 1 <= h.spec_sample_bytes() <= 4
    &&& (h.map_first as int + h.map_length as int) <= 65536
    &&& 4 * (h.width as int * (h.height as int)) <= 67108864
    &&& (h.color_map_flag == 1u8 ==> 1 <= (h.map_length as int) <= 65535)
    &&& (h.color_map_flag == 1u8 ==> crate::model::map_entry_depth_ok(h.map_entry_depth))
    &&& (h.color_map_flag == 1u8 ==> 1 <= h.spec_map_stride() <= 4)
    &&& (h.color_map_flag == 0u8 ==> h.map_length == 0 && h.map_entry_depth == 0 && h.map_first
        == 0)
    &&& (crate::model::is_indexed(h.image_type) ==> h.color_map_flag == 1u8)
}

/// T5: the executable record reconstructs the model record `m` exactly. The
/// model's derived fields (`N`, `sample_bytes`, `map_bytes`) are the record's
/// own formulas over the raw fields, so this is record equality.
spec fn hdr_matches(m: crate::model::MHModel, h: Header) -> bool {
    &&& m.id_length == h.id_length
    &&& m.color_map_flag == h.color_map_flag
    &&& m.image_type == h.image_type
    &&& m.map_first == h.map_first
    &&& m.map_length == h.map_length
    &&& m.map_entry_depth == h.map_entry_depth
    &&& m.x_origin == h.x_origin
    &&& m.y_origin == h.y_origin
    &&& m.width == h.width
    &&& m.height == h.height
    &&& m.pixel_depth == h.pixel_depth
    &&& m.descriptor == h.descriptor
    &&& m.n == h.width as int * (h.height as int)
    &&& m.sample_bytes == crate::model::sample_bytes_of(h.pixel_depth)
    &&& m.map_bytes == if h.color_map_flag == 1u8 {
        h.map_length as int * crate::model::sample_bytes_of(h.map_entry_depth)
    } else {
        0
    }
}

/// The map slice length the decoder carries for a given header.
spec fn map_len_of(flag: u8, length: u32, stride: int) -> int {
    if flag == 1u8 {
        length as int * stride
    } else {
        0
    }
}

/// The decoder's carried record: the header plus `N` and the map slice.
spec fn dec_inv(h: Header, map: &[u8], n: usize) -> bool {
    &&& header_inv(h)
    &&& n == h.width as int * (h.height as int)
    &&& n >= 1
    &&& map@.len() == map_len_of(h.color_map_flag, h.map_length, h.spec_map_stride())
}

/// The map end position the model computes, related to the executable map end.
spec fn map_end_of(m: crate::model::MHModel, h: Header) -> int {
    18 + (h.id_length as int) + (m.map_bytes)
}

/// T7: the model result tuple `(width, height, rgba)` of an executable `Decoded`.
pub open spec fn dec_vals(d: Decoded) -> (int, int, Seq<u8>) {
    (d.width as int, d.height as int, d.rgba@)
}

/// T7/T9: `decode(bytes)` equals `MP(bytes)` as the exact `Result` — the exact
/// three-tuple, or the exact error. This is the relation the public entry
/// carries; `MP` is the model's definition, not a call to any executable code.
pub open spec fn decode_eq(r: Result<Decoded, Error>, b: Seq<u8>) -> bool {
    match r {
        Ok(d) => crate::model::mp(b) == Ok(dec_vals(d)),
        Err(e) => crate::model::mp(b) == Err(e),
    }
}

/// T7 at the raw helper: the executable result equals `MP`'s raw phase as the
/// exact `Result` — on `Ok` the model accepts with this final cursor (the pixels
/// are checked separately by `wrote_samples`); on `Err` the model's exact first
/// error. The model's ghost error fields (cursor, produced, pixels at the
/// failing check) are proof observations and are not compared to any API result
/// field. Stated on the cursor projection `mp_raw_cur`, which carries the same
/// ordered checks as `mp_raw` (`model::lemma_mp_raw_cur_proj`).
spec fn raw_result_eq(
    b: Seq<u8>,
    p: crate::model::PktH,
    c0: int,
    cur_final: int,
    r: Result<(), Error>,
) -> bool {
    match crate::model::mp_raw_cur(b, p, c0, 0) {
        Ok(cend) => match r {
            Ok(_) => cur_final == cend,
            Err(_) => false,
        },
        Err(e) => match r {
            Ok(_) => false,
            Err(e2) => e2 == e,
        },
    }
}

/// T6 at the RLE helper, the pixel half: the produced pixels are exactly
/// `MR`'s packet pixels, written at their oriented cells. On success the whole
/// output is the model's decoded sequence placed by R9 (`rle_quad_ok` over all
/// `N` pixels, which `model::lemma_quad_ok_to_orient` turns into the model's
/// `orient_bytes` assembly); on an error no partial output is returned, so only
/// the error is observed.
/// T6: the model's `mr_cur` at the packet-stream start reporting error `e` gives
/// the `rle_result_eq` error conclusion (the model's own error is `e`, whatever
/// its ghost cursor/produced/pixel fields are).
proof fn lemma_rle_err(
    b: Seq<u8>,
    p: crate::model::PktH,
    map: Seq<u8>,
    c0: int,
    e: Error,
)
    requires
        crate::model::mr_cur(b, p, map, c0, 0) == Err(crate::model::MrErr::Api(e)),
        c0 >= 0,
        0 <= p.sample_bytes,
        0 <= p.n,
    ensures
        rle_result_eq(b, p, map, c0, c0, Err(e)),
{
    reveal(rle_result_eq);
    reveal(crate::model::mr);
    crate::model::lemma_mr_cur_pix_proj(b, p, map, c0, 0, Seq::<Seq<u8>>::empty());
    match crate::model::mr(b, p, map, c0, 0) {
        Ok(r) => {
            assert(crate::model::mr_cur(b, p, map, c0, 0) == Ok((r.0, r.1)));
        },
        Err(t) => {
            assert(t.0 == crate::model::MrErr::Api(e));
        },
    }
}

/// T6 from a nonzero starting produced count: the model's `mr_cur` at `(c0, p0)`
/// reporting error `e` gives the `rle_result_eq_from` error conclusion.
proof fn lemma_rle_err_from(
    b: Seq<u8>,
    p: crate::model::PktH,
    map: Seq<u8>,
    c0: int,
    p0: int,
    e: Error,
)
    requires
        crate::model::mr_cur(b, p, map, c0, p0) == Err(crate::model::MrErr::Api(e)),
        c0 >= 0,
        0 <= p0 <= p.n,
        0 <= p.sample_bytes,
        0 <= p.n,
    ensures
        rle_result_eq_from(b, p, map, c0, p0, c0, Err(e)),
{
    reveal(rle_result_eq_from);
    reveal(crate::model::mr);
    crate::model::lemma_mr_cur_pix_proj(b, p, map, c0, p0, Seq::<Seq<u8>>::empty());
    match crate::model::mr(b, p, map, c0, p0) {
        Ok(r) => {
            assert(crate::model::mr_cur(b, p, map, c0, p0) == Ok((r.0, r.1)));
        },
        Err(t) => {
            assert(t.0 == crate::model::MrErr::Api(e));
        },
    }
}

/// T6: the model's `mr_cur` at the packet-stream start accepting with cursor
/// `cend` and produced `n` gives the `rle_result_eq` success conclusion.
proof fn lemma_rle_ok(
    b: Seq<u8>,
    p: crate::model::PktH,
    map: Seq<u8>,
    c0: int,
    cend: int,
    n: int,
)
    requires
        crate::model::mr_cur(b, p, map, c0, 0) == Ok((cend, n)),
        n == p.n,
        c0 >= 0,
        0 <= p.sample_bytes,
        0 <= p.n,
    ensures
        rle_result_eq(b, p, map, c0, cend, Ok(())),
{
    reveal(rle_result_eq);
    reveal(crate::model::mr);
    crate::model::lemma_mr_cur_pix_proj(b, p, map, c0, 0, Seq::<Seq<u8>>::empty());
    match crate::model::mr(b, p, map, c0, 0) {
        Ok(r) => {
            assert(crate::model::mr_cur(b, p, map, c0, 0) == Ok((r.0, r.1)));
            assert(r.0 == cend);
            assert(r.1 == n);
        },
        Err(t) => {
            assert(crate::model::mr_cur(b, p, map, c0, 0) == Err(t.0));
        },
    }
}

/// T6 from a nonzero starting produced count: the model's `mr_cur` at `(c0, p0)`
/// accepting with cursor `cend` and produced `n` gives the `rle_result_eq_from`
/// success conclusion.
proof fn lemma_rle_ok_from(
    b: Seq<u8>,
    p: crate::model::PktH,
    map: Seq<u8>,
    c0: int,
    p0: int,
    cend: int,
    n: int,
)
    requires
        crate::model::mr_cur(b, p, map, c0, p0) == Ok((cend, n)),
        n == p.n,
        c0 >= 0,
        0 <= p0 <= p.n,
        0 <= p.sample_bytes,
        0 <= p.n,
    ensures
        rle_result_eq_from(b, p, map, c0, p0, cend, Ok(())),
{
    reveal(rle_result_eq_from);
    reveal(crate::model::mr);
    crate::model::lemma_mr_cur_pix_proj(b, p, map, c0, p0, Seq::<Seq<u8>>::empty());
    match crate::model::mr(b, p, map, c0, p0) {
        Ok(r) => {
            assert(crate::model::mr_cur(b, p, map, c0, p0) == Ok((r.0, r.1)));
            assert(r.0 == cend);
            assert(r.1 == n);
        },
        Err(t) => {
            assert(crate::model::mr_cur(b, p, map, c0, p0) == Err(t.0));
        },
    }
}

spec fn rle_pixels_eq_from(
    b: Seq<u8>,
    p: crate::model::PktH,
    map: Seq<u8>,
    c0: int,
    p0: int,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    out: Seq<u8>,
    r: Result<(), Error>,
) -> bool {
    &&& out.len() == 4 * p.n
    &&& match crate::model::mr_cur(b, p, map, c0, p0) {
        Ok(_) => crate::model::rle_quad_ok_off(
            out,
            crate::model::mr_pix(b, p, map, c0, p0),
            w,
            hgt,
            fx,
            fy,
            p0,
        ),
        Err(_) => true,
    }
}

/// `rle_full_eq_from`: the helper's result equals `MR` exactly from `(c0, p0)`.
spec fn rle_full_eq_from(
    b: Seq<u8>,
    p: crate::model::PktH,
    map: Seq<u8>,
    c0: int,
    p0: int,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    cur_final: int,
    out: Seq<u8>,
    r: Result<(), Error>,
) -> bool {
    &&& rle_result_eq_from(b, p, map, c0, p0, cur_final, r)
    &&& rle_pixels_eq_from(b, p, map, c0, p0, w, hgt, fx, fy, out, r)
}

/// `rle_result_eq_from`: T6's exact cursor/produced/error projection of `MR`
/// from an arbitrary starting state `(c0, p0)`.
spec fn rle_result_eq_from(
    b: Seq<u8>,
    p: crate::model::PktH,
    map: Seq<u8>,
    c0: int,
    p0: int,
    cur_final: int,
    r: Result<(), Error>,
) -> bool {
    match crate::model::mr(b, p, map, c0, p0) {
        Ok((cend, prod, _px)) => match r {
            Ok(_) => cur_final == cend && prod == p.n,
            Err(_) => false,
        },
        Err((e, _c, _pr, _px)) => match r {
            Ok(_) => false,
            Err(e2) => match e {
                crate::model::MrErr::Api(ea) => e2 == ea,
                crate::model::MrErr::Outside => false,
            },
        },
    }
}

/// T6, stated on `MR`'s own result: the concrete RLE helper's result equals
/// `MR(s,H,map,cursor0,produced0)` exactly. On `Ok((cend, pend, px))` the helper
/// returned `Ok`, its final cursor is `cend`, its final produced count is `pend`
/// (which is `N`), and `px`, the converted RGBA pixels produced since
/// `produced0`, sits in the output at the oriented cells `f(produced0 + j)`. On
/// `Err((e, gc, gp, gpx))` the helper returned exactly the API error `e`, its
/// final cursor is `MR`'s ghost cursor `gc`, its final produced count is `MR`'s
/// ghost produced count `gp`, and the partial pixels `gpx` produced before the
/// failing check sit at their oriented cells in the output. `out` is the whole
/// `4*N`-byte output buffer the helper leaves behind.
spec fn rle_exact_from(
    b: Seq<u8>,
    p: crate::model::PktH,
    map: Seq<u8>,
    c0: int,
    p0: int,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    cur_final: int,
    prod_final: int,
    out: Seq<u8>,
    r: Result<(), Error>,
) -> bool {
    &&& out.len() == 4 * p.n
    &&& match crate::model::mr(b, p, map, c0, p0) {
        Ok((cend, pend, px)) => {
            &&& r is Ok
            &&& cur_final == cend
            &&& prod_final == pend
            &&& pend == p.n
            &&& crate::model::rle_quad_ok_off(out, px, w, hgt, fx, fy, p0)
        },
        Err((e, gc, gp, gpx)) => match r {
            Ok(_) => false,
            Err(e2) => {
                &&& e == crate::model::MrErr::Api(e2)
                &&& cur_final == gc
                &&& prod_final == gp
                &&& crate::model::rle_quad_ok_off(out, gpx, w, hgt, fx, fy, p0)
            },
        },
    }
}

/// One packet of the concrete helper against the model: from the control
/// position `cur0`, produced count `prod0` and accumulated pixels `acc0` to the
/// state `(cur1, prod1, acc1)` the helper leaves. On `Ok` the model's
/// cursor/ghost-cursor/pixel projections are carried across the packet (the
/// step of `decode_rle`'s loop invariants). On `Err(e)` the model's run from
/// `(cur0, prod0)` fails with exactly `e`, its ghost cursor is `cur1`, and its
/// pixels are the failing packet's partial output, so `acc1` is the whole run's
/// `mr_pix`.
spec fn pkt_post(
    b: Seq<u8>,
    p: crate::model::PktH,
    map: Seq<u8>,
    cur0: int,
    prod0: int,
    cur1: int,
    prod1: int,
    acc0: Seq<Seq<u8>>,
    acc1: Seq<Seq<u8>>,
    r: Result<(), Error>,
) -> bool {
    match r {
        Ok(_) => {
            &&& prod0 < prod1
            &&& crate::model::mr_cur(b, p, map, cur0, prod0) == crate::model::mr_cur(b, p, map, cur1, prod1)
            &&& crate::model::mr_gcur(b, p, map, cur0, prod0) == crate::model::mr_gcur(b, p, map, cur1, prod1)
            &&& acc0 + crate::model::mr_pix(b, p, map, cur0, prod0) == acc1 + crate::model::mr_pix(b, p, map, cur1, prod1)
        },
        Err(e) => {
            &&& crate::model::mr_cur(b, p, map, cur0, prod0) == Err(crate::model::MrErr::Api(e))
            &&& crate::model::mr_gcur(b, p, map, cur0, prod0) == Ok(cur1)
            &&& acc0 + crate::model::mr_pix(b, p, map, cur0, prod0) == acc1
        },
    }
}

/// The output side of one packet: the accumulated pixels `acc` are the
/// `prod - p0` pixels produced since the helper's entry, each written at its
/// oriented cell `f(p0 + j)`.
spec fn pkt_out(
    out: Seq<u8>,
    acc: Seq<Seq<u8>>,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    p0: int,
    prod: int,
) -> bool {
    &&& acc.len() == prod - p0
    &&& crate::model::wrote_px_off(out, acc, w, hgt, fx, fy, p0, prod - p0)
}

/// Exit 1: no control byte. The model fails with `RleTruncated` at the cursor.
proof fn lemma_pkt_ctrl_absent(
    b: Seq<u8>,
    p: crate::model::PktH,
    map: Seq<u8>,
    cur0: int,
    prod0: int,
    acc0: Seq<Seq<u8>>,
)
    requires
        0 <= cur0 <= b.len(),
        cur0 >= b.len(),
        0 <= prod0 < p.n,
    ensures
        pkt_post(b, p, map, cur0, prod0, cur0, prod0, acc0, acc0, Err(Error::RleTruncated)),
{
    reveal(pkt_post);
    crate::model::lemma_mr_cur_ctrl_absent(b, p, map, cur0, prod0);
    assert(crate::model::mr_gcur(b, p, map, cur0, prod0) == Ok(cur0)) by {
        reveal(crate::model::mr_gcur);
    }
    assert(acc0 + Seq::<Seq<u8>>::empty() == acc0) by {
        broadcast use vstd::seq_lib::group_seq_lib_default;
    }
}

/// Exit 2: the run would overrun `N`. The model fails with `RleOverrun` just
/// after the control byte.
proof fn lemma_pkt_overrun(
    b: Seq<u8>,
    p: crate::model::PktH,
    map: Seq<u8>,
    cur0: int,
    prod0: int,
    acc0: Seq<Seq<u8>>,
)
    requires
        0 <= cur0 < b.len(),
        0 <= prod0 < p.n,
        crate::model::rle_count(b[cur0] as int) > p.n - prod0,
    ensures
        pkt_post(b, p, map, cur0, prod0, cur0 + 1, prod0, acc0, acc0, Err(Error::RleOverrun)),
{
    reveal(pkt_post);
    crate::model::lemma_mr_cur_overrun(b, p, map, cur0, prod0);
    assert(crate::model::mr_gcur(b, p, map, cur0, prod0) == Ok(cur0 + 1)) by {
        reveal(crate::model::mr_gcur);
    }
    assert(acc0 + Seq::<Seq<u8>>::empty() == acc0) by {
        broadcast use vstd::seq_lib::group_seq_lib_default;
    }
}

/// Exit 3: a repeat packet without its complete sample. The model fails with
/// `RleTruncated` at the first sample byte.
proof fn lemma_pkt_rep_missing(
    b: Seq<u8>,
    p: crate::model::PktH,
    map: Seq<u8>,
    cur0: int,
    prod0: int,
    acc0: Seq<Seq<u8>>,
)
    requires
        0 <= cur0 < b.len(),
        0 <= prod0 < p.n,
        crate::model::rle_count(b[cur0] as int) <= p.n - prod0,
        crate::model::rle_repeat(b[cur0] as int),
        b.len() < cur0 + 1 + p.sample_bytes,
    ensures
        pkt_post(b, p, map, cur0, prod0, cur0 + 1, prod0, acc0, acc0, Err(Error::RleTruncated)),
{
    reveal(pkt_post);
    crate::model::lemma_mr_cur_rep_missing(b, p, map, cur0, prod0);
    assert(crate::model::mr_gcur(b, p, map, cur0, prod0) == Ok(cur0 + 1)) by {
        reveal(crate::model::mr_gcur);
    }
    assert(acc0 + Seq::<Seq<u8>>::empty() == acc0) by {
        broadcast use vstd::seq_lib::group_seq_lib_default;
    }
}

/// Exit 4: a repeat packet whose sample index is out of range. The model fails
/// with `ColorIndexOutOfRange` at the packet end.
proof fn lemma_pkt_rep_index(
    b: Seq<u8>,
    p: crate::model::PktH,
    map: Seq<u8>,
    cur0: int,
    prod0: int,
    acc0: Seq<Seq<u8>>,
)
    requires
        0 <= cur0 < b.len(),
        0 <= prod0 < p.n,
        crate::model::rle_count(b[cur0] as int) <= p.n - prod0,
        crate::model::rle_repeat(b[cur0] as int),
        b.len() >= cur0 + 1 + p.sample_bytes,
        p.kind == 0,
        !crate::model::in_range(
            p,
            crate::model::sindex(b.subrange(cur0 + 1, cur0 + 1 + p.sample_bytes), p.sample_bytes),
        ),
    ensures
        pkt_post(
            b,
            p,
            map,
            cur0,
            prod0,
            cur0 + 1 + p.sample_bytes,
            prod0,
            acc0,
            acc0,
            Err(Error::ColorIndexOutOfRange),
        ),
{
    reveal(pkt_post);
    crate::model::lemma_mr_cur_rep_index(b, p, map, cur0, prod0);
    assert(crate::model::mr_gcur(b, p, map, cur0, prod0) == Ok(cur0 + 1 + p.sample_bytes)) by {
        reveal(crate::model::mr_gcur);
    }
    assert(acc0 + Seq::<Seq<u8>>::empty() == acc0) by {
        broadcast use vstd::seq_lib::group_seq_lib_default;
    }
}

/// Exit 5: a literal packet without all its samples. The model fails with
/// `RleTruncated` at the first sample byte.
proof fn lemma_pkt_lit_missing(
    b: Seq<u8>,
    p: crate::model::PktH,
    map: Seq<u8>,
    cur0: int,
    prod0: int,
    acc0: Seq<Seq<u8>>,
)
    requires
        0 <= cur0 < b.len(),
        0 <= prod0 < p.n,
        crate::model::rle_count(b[cur0] as int) <= p.n - prod0,
        !crate::model::rle_repeat(b[cur0] as int),
        b.len() < cur0 + 1 + crate::model::rle_count(b[cur0] as int) * p.sample_bytes,
    ensures
        pkt_post(b, p, map, cur0, prod0, cur0 + 1, prod0, acc0, acc0, Err(Error::RleTruncated)),
{
    reveal(pkt_post);
    crate::model::lemma_mr_cur_lit_missing(b, p, map, cur0, prod0);
    assert(crate::model::mr_gcur(b, p, map, cur0, prod0) == Ok(cur0 + 1)) by {
        reveal(crate::model::mr_gcur);
    }
    assert(acc0 + Seq::<Seq<u8>>::empty() == acc0) by {
        broadcast use vstd::seq_lib::group_seq_lib_default;
    }
}

/// Exit 6: a complete literal packet whose samples `0..i` have in-range indices
/// and whose sample `i` does not. The model fails with `ColorIndexOutOfRange`
/// at the packet end, having produced exactly the `i` converted samples
/// (`lit_good = i`), which the concrete loop has already written.
proof fn lemma_pkt_lit_index(
    b: Seq<u8>,
    p: crate::model::PktH,
    map: Seq<u8>,
    cur0: int,
    prod0: int,
    count: int,
    i: int,
    acc0: Seq<Seq<u8>>,
    acc1: Seq<Seq<u8>>,
)
    requires
        0 <= cur0 < b.len(),
        0 <= prod0 < p.n,
        0 <= p.sample_bytes,
        count == crate::model::rle_count(b[cur0] as int),
        count <= p.n - prod0,
        !crate::model::rle_repeat(b[cur0] as int),
        b.len() >= cur0 + 1 + count * p.sample_bytes,
        0 <= i < count,
        p.kind == 0,
        crate::model::lit_pref(b, p, cur0 + 1, i),
        !crate::model::in_range(
            p,
            crate::model::sindex(
                b.subrange(
                    cur0 + 1 + i * p.sample_bytes,
                    cur0 + 1 + i * p.sample_bytes + p.sample_bytes,
                ),
                p.sample_bytes,
            ),
        ),
        acc1 == acc0 + crate::model::lit_out(b, p, map, cur0 + 1, i),
    ensures
        pkt_post(
            b,
            p,
            map,
            cur0,
            prod0,
            cur0 + 1 + count * p.sample_bytes,
            prod0 + i,
            acc0,
            acc1,
            Err(Error::ColorIndexOutOfRange),
        ),
{
    reveal(pkt_post);
    // the first bad sample is `i`, so the packet is not all good
    crate::model::lemma_lit_pref_bad(b, p, cur0 + 1, i, count);
    assert(crate::model::lit_good(b, p, cur0 + 1, count) == i);
    crate::model::lemma_lit_pref_eq_ok(b, p, cur0 + 1, count);
    if crate::model::lit_ok(b, p, cur0 + 1, count) {
        crate::model::lemma_lit_ok_implies_in_range(b, p, cur0 + 1, count, i);
        assert(false);
    }
    crate::model::lemma_mr_cur_lit_index(b, p, map, cur0, prod0);
    crate::model::lemma_mr_gcur_step_lit_err(b, p, map, cur0, prod0, count, i);
}

/// Step: a present repeat packet with an in-range sample converting to `c`,
/// written `count` times. The model's projections carry across the packet.
proof fn lemma_pkt_rep_ok(
    b: Seq<u8>,
    p: crate::model::PktH,
    map: Seq<u8>,
    cur0: int,
    prod0: int,
    count: int,
    c: Seq<u8>,
    acc0: Seq<Seq<u8>>,
    acc1: Seq<Seq<u8>>,
)
    requires
        0 <= cur0 < b.len(),
        0 <= prod0 < p.n,
        0 <= p.sample_bytes,
        count == crate::model::rle_count(b[cur0] as int),
        1 <= count <= p.n - prod0,
        crate::model::rle_repeat(b[cur0] as int),
        b.len() >= cur0 + 1 + p.sample_bytes,
        p.kind == 0 ==> crate::model::in_range(
            p,
            crate::model::sindex(b.subrange(cur0 + 1, cur0 + 1 + p.sample_bytes), p.sample_bytes),
        ),
        c == crate::model::conv2(p, map, b.subrange(cur0 + 1, cur0 + 1 + p.sample_bytes)),
        acc1 == acc0 + crate::model::pix_pow(c, count),
    ensures
        pkt_post(b, p, map, cur0, prod0, cur0 + 1 + p.sample_bytes, prod0 + count, acc0, acc1, Ok(())),
{
    reveal(pkt_post);
    crate::model::lemma_mr_packet_split_rep(b, p, map, cur0, prod0, count, c);
    crate::model::lemma_mr_gcur_step_rep(b, p, map, cur0, prod0, count);
    vstd::seq_lib::lemma_concat_associative(
        acc0,
        crate::model::pix_pow(c, count),
        crate::model::mr_pix(b, p, map, cur0 + 1 + p.sample_bytes, prod0 + count),
    );
}

/// Step: a present literal packet whose `count` samples all have in-range
/// indices. The model's projections carry across the packet.
proof fn lemma_pkt_lit_ok(
    b: Seq<u8>,
    p: crate::model::PktH,
    map: Seq<u8>,
    cur0: int,
    prod0: int,
    count: int,
    acc0: Seq<Seq<u8>>,
    acc1: Seq<Seq<u8>>,
)
    requires
        0 <= cur0 < b.len(),
        0 <= prod0 < p.n,
        0 <= p.sample_bytes,
        count == crate::model::rle_count(b[cur0] as int),
        1 <= count <= p.n - prod0,
        !crate::model::rle_repeat(b[cur0] as int),
        b.len() >= cur0 + 1 + count * p.sample_bytes,
        crate::model::lit_pref(b, p, cur0 + 1, count),
        acc1 == acc0 + crate::model::lit_out(b, p, map, cur0 + 1, count),
    ensures
        pkt_post(
            b,
            p,
            map,
            cur0,
            prod0,
            cur0 + 1 + count * p.sample_bytes,
            prod0 + count,
            acc0,
            acc1,
            Ok(()),
        ),
{
    reveal(pkt_post);
    crate::model::lemma_lit_pref_eq_ok(b, p, cur0 + 1, count);
    crate::model::lemma_mr_packet_split_lit(b, p, map, cur0, prod0, count);
    crate::model::lemma_mr_gcur_step_lit(b, p, map, cur0, prod0, count);
    vstd::seq_lib::lemma_concat_associative(
        acc0,
        crate::model::lit_out(b, p, map, cur0 + 1, count),
        crate::model::mr_pix(b, p, map, cur0 + 1 + count * p.sample_bytes, prod0 + count),
    );
}

/// Every exit of `decode_rle`, from its loop state: the accumulated pixels are
/// the whole run's `mr_pix`, written at their oriented cells, and the model's
/// cursor projection is the exit's (`Ok` with cursor `cur_final` and produced
/// `N`, or the exit's error with ghost cursor `cur_final`). Concludes all of
/// the helper's T6 postconditions, including `rle_exact_from` on `MR`'s own
/// result tuple.
proof fn lemma_rle_post_from_state(
    b: Seq<u8>,
    p: crate::model::PktH,
    map: Seq<u8>,
    c0: int,
    p0: int,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    cur_final: int,
    prod_final: int,
    out: Seq<u8>,
    acc: Seq<Seq<u8>>,
    r: Result<(), Error>,
)
    requires
        c0 >= 0,
        0 <= p0 <= p.n,
        0 <= p.sample_bytes,
        out.len() == 4 * p.n,
        crate::model::mr_pix(b, p, map, c0, p0) == acc,
        acc.len() == prod_final - p0,
        crate::model::wrote_px_off(out, acc, w, hgt, fx, fy, p0, prod_final - p0),
        match r {
            Ok(_) => crate::model::mr_cur(b, p, map, c0, p0) == Ok((cur_final, p.n)) && prod_final
                == p.n,
            Err(e) => crate::model::mr_cur(b, p, map, c0, p0) == Err(crate::model::MrErr::Api(e))
                && crate::model::mr_gcur(b, p, map, c0, p0) == Ok(cur_final),
        },
    ensures
        rle_result_eq_from(b, p, map, c0, p0, cur_final, r),
        rle_pixels_eq_from(b, p, map, c0, p0, w, hgt, fx, fy, out, r),
        rle_full_eq_from(b, p, map, c0, p0, w, hgt, fx, fy, cur_final, out, r),
        rle_exact_from(b, p, map, c0, p0, w, hgt, fx, fy, cur_final, prod_final, out, r),
        crate::model::mr_gcur(b, p, map, c0, p0) == Ok(cur_final) || r is Ok,
{
    reveal(rle_result_eq_from);
    reveal(rle_pixels_eq_from);
    reveal(rle_full_eq_from);
    reveal(rle_exact_from);
    reveal(crate::model::mr);
    crate::model::lemma_wrote_px_off_full(out, acc, w, hgt, fx, fy, p0);
    assert(crate::model::rle_quad_ok_off(out, acc, w, hgt, fx, fy, p0));
    crate::model::lemma_mr_cur_pix_proj(b, p, map, c0, p0, Seq::<Seq<u8>>::empty());
    crate::model::lemma_mr_gcur_proj(b, p, map, c0, p0, Seq::<Seq<u8>>::empty());
    assert(Seq::<Seq<u8>>::empty() + acc == acc) by {
        broadcast use vstd::seq_lib::group_seq_lib_default;
    }
    match r {
        Ok(_) => {
            lemma_rle_ok_from(b, p, map, c0, p0, cur_final, p.n);
        },
        Err(e) => {
            lemma_rle_err_from(b, p, map, c0, p0, e);
        },
    }
}

/// The `p0 = 0` case of `rle_result_eq_from` is `rle_result_eq`.
proof fn lemma_rle_result_eq_from_zero(
    b: Seq<u8>,
    p: crate::model::PktH,
    map: Seq<u8>,
    c0: int,
    cur_final: int,
    r: Result<(), Error>,
)
    ensures
        rle_result_eq_from(b, p, map, c0, 0, cur_final, r) == rle_result_eq(b, p, map, c0, cur_final, r),
{
    reveal(rle_result_eq_from);
    reveal(rle_result_eq);
}

/// The `p0 = 0` case of `rle_pixels_eq_from` is `rle_pixels_eq`.
proof fn lemma_rle_pixels_eq_from_zero(
    b: Seq<u8>,
    p: crate::model::PktH,
    map: Seq<u8>,
    c0: int,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    out: Seq<u8>,
    r: Result<(), Error>,
)
    ensures
        rle_pixels_eq_from(b, p, map, c0, 0, w, hgt, fx, fy, out, r) == rle_pixels_eq(
            b,
            p,
            map,
            c0,
            w,
            hgt,
            fx,
            fy,
            out,
            r,
        ),
{
    reveal(rle_pixels_eq_from);
    reveal(rle_pixels_eq);
    crate::model::lemma_wrote_px_off_zero_is_quad_ok(
        out,
        crate::model::mr_pix(b, p, map, c0, 0),
        w,
        hgt,
        fx,
        fy,
    );
}

/// The `p0 = 0` case of `rle_full_eq_from` is `rle_full_eq`.
proof fn lemma_rle_full_eq_from_zero(
    b: Seq<u8>,
    p: crate::model::PktH,
    map: Seq<u8>,
    c0: int,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    cur_final: int,
    out: Seq<u8>,
    r: Result<(), Error>,
)
    ensures
        rle_full_eq_from(b, p, map, c0, 0, w, hgt, fx, fy, cur_final, out, r) <==> rle_full_eq(
            b,
            p,
            map,
            c0,
            w,
            hgt,
            fx,
            fy,
            cur_final,
            out,
            r,
        ),
{
    reveal(rle_full_eq_from);
    reveal(rle_full_eq);
    lemma_rle_result_eq_from_zero(b, p, map, c0, cur_final, r);
    lemma_rle_pixels_eq_from_zero(b, p, map, c0, w, hgt, fx, fy, out, r);
}

spec fn rle_pixels_eq(
    b: Seq<u8>,
    p: crate::model::PktH,
    map: Seq<u8>,
    c0: int,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    out: Seq<u8>,
    r: Result<(), Error>,
) -> bool {
    &&& out.len() == 4 * p.n
    &&& match crate::model::mr_cur(b, p, map, c0, 0) {
        Ok(_) => crate::model::rle_quad_ok(out, crate::model::mr_pix(b, p, map, c0, 0), w, hgt, fx, fy),
        Err(_) => true,
    }
}

/// T6/T7: from `rle_result_eq` on a successful helper call, the model's
/// cursor/pixel projections.
proof fn lemma_rle_result_ok(
    b: Seq<u8>,
    p: crate::model::PktH,
    map: Seq<u8>,
    c0: int,
    cur_final: int,
)
    requires
        rle_result_eq(b, p, map, c0, cur_final, Ok(())),
        c0 >= 0,
        0 <= p.sample_bytes,
        p.n >= 0,
    ensures
        crate::model::mr_cur(b, p, map, c0, 0) == Ok((cur_final, p.n)),
        crate::model::mr(b, p, map, c0, 0) == Ok((
            cur_final,
            p.n,
            crate::model::mr_pix(b, p, map, c0, 0),
        )),
{
    reveal(rle_result_eq);
    reveal(crate::model::mr);
    match crate::model::mr(b, p, map, c0, 0) {
        Ok(r) => {
            assert(r.0 == cur_final);
            assert(r.1 == p.n);
            crate::model::lemma_mr_at_ok(b, p, map, c0, r.0, r.1, r.2);
        },
        Err(t) => {
            assert(false);
        },
    }
}

/// T6/T7: from `rle_result_eq` on a failing helper call, the model's error
/// projection.
proof fn lemma_rle_result_err(
    b: Seq<u8>,
    p: crate::model::PktH,
    map: Seq<u8>,
    c0: int,
    cur_final: int,
    e: Error,
)
    requires
        rle_result_eq(b, p, map, c0, cur_final, Err(e)),
        c0 >= 0,
        0 <= p.sample_bytes,
        p.n >= 0,
    ensures
        crate::model::mr_cur(b, p, map, c0, 0) == Err(crate::model::MrErr::Api(e)),
{
    reveal(rle_result_eq);
    reveal(crate::model::mr);
    match crate::model::mr(b, p, map, c0, 0) {
        Ok(r) => {
            assert(false);
        },
        Err(t) => {
            assert(t.0 == crate::model::MrErr::Api(e));
            crate::model::lemma_mr_at_err(b, p, map, c0, t.0, t.1, t.2, t.3);
        },
    }
}

/// T6/T7: from `raw_result_eq` on a successful helper call, the model's cursor
/// projection.
proof fn lemma_raw_result_ok(
    b: Seq<u8>,
    p: crate::model::PktH,
    c0: int,
    cur_final: int,
)
    requires
        raw_result_eq(b, p, c0, cur_final, Ok(())),
        p.sample_bytes >= 0,
        p.n >= 0,
        c0 >= 0,
    ensures
        crate::model::mp_raw_cur(b, p, c0, 0) == Ok(cur_final),
{
    reveal(raw_result_eq);
    match crate::model::mp_raw_cur(b, p, c0, 0) {
        Ok(cend) => {
            assert(cend == cur_final);
        },
        Err(e) => {
            assert(false);
        },
    }
}

/// T6/T7: from `raw_result_eq` on a failing helper call, the model's error
/// projection.
proof fn lemma_raw_result_err(
    b: Seq<u8>,
    p: crate::model::PktH,
    c0: int,
    cur_final: int,
    e: Error,
)
    requires
        raw_result_eq(b, p, c0, cur_final, Err(e)),
        p.n >= 0,
        c0 >= 0,
    ensures
        crate::model::mp_raw_cur(b, p, c0, 0) == Err(e),
{
    reveal(raw_result_eq);
    match crate::model::mp_raw_cur(b, p, c0, 0) {
        Ok(cend) => {
            assert(false);
        },
        Err(e2) => {
            assert(e2 == e);
        },
    }
}

/// T6: the RLE helper's result equals `MR` exactly — the cursor/produced/error
/// projection (`rle_result_eq`) and the produced pixels (`rle_pixels_eq`).
spec fn rle_full_eq(
    b: Seq<u8>,
    p: crate::model::PktH,
    map: Seq<u8>,
    c0: int,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    cur_final: int,
    out: Seq<u8>,
    r: Result<(), Error>,
) -> bool {
    &&& rle_result_eq(b, p, map, c0, cur_final, r)
    &&& rle_pixels_eq(b, p, map, c0, w, hgt, fx, fy, out, r)
}

/// T6 at the RLE helper: on `Ok` the returned cursor is the model's final cursor
/// and the produced count is `N`; on `Err` the returned error is the model's
/// exact first error (control presence, then overrun, then whole-packet bytes,
/// then each sample's index). The produced pixels are checked separately by the
/// model-vs-executable agreement lemma.
spec fn rle_result_eq(
    b: Seq<u8>,
    p: crate::model::PktH,
    map: Seq<u8>,
    c0: int,
    cur_final: int,
    r: Result<(), Error>,
) -> bool {
    match crate::model::mr(b, p, map, c0, 0) {
        Ok((cend, prod, _px)) => match r {
            Ok(_) => cur_final == cend && prod == p.n,
            Err(_) => false,
        },
        Err((e, _c, _pr, _px)) => match r {
            Ok(_) => false,
            Err(e2) => match e {
                crate::model::MrErr::Api(ea) => e2 == ea,
                crate::model::MrErr::Outside => false,
            },
        },
    }
}

/// `LE16(p) = b[p] + 256*b[p+1]` (R1). Callers guarantee both bytes exist.
fn le16(b: &[u8], p: usize) -> (r: u32)
    requires
        p + 2 <= b@.len(),
    ensures
        r as int == crate::model::le16(b@, p as int),
{
    b[p] as u32 + 256 * (b[p + 1] as u32)
}

/// R12: do the final 18 bytes of `b` equal the exact footer signature?
/// Mirrors `bytes[bytes.len()-18..] != SIGNATURE` of M1.
fn signature_tail(b: &[u8]) -> (r: bool)
    requires
        b@.len() >= 18,
    ensures
        r == (b@.subrange(b@.len() as int - 18, b@.len() as int) == crate::model::tga_signature()),
{
    let sig: [u8; 18] = [
        84u8, 82, 85, 69, 86, 73, 83, 73, 79, 78, 45, 88, 70, 73, 76, 69, 46, 0,
    ];
    proof {
        assert(sig@ =~= crate::model::tga_signature());
    }
    let base = b.len() - 18;
    let mut i: usize = 0;
    while i < 18
        invariant
            i <= 18,
            base == b@.len() - 18,
            base as int + 18 == b@.len(),
            sig@ == crate::model::tga_signature(),
            forall|k: int| 0 <= k < i ==> b@[base as int + k] == sig@[k],
        decreases 18 - i,
    {
        if b[base + i] != sig[i] {
            proof {
                assert(!(b@.subrange(b@.len() as int - 18, b@.len() as int)
                    == crate::model::tga_signature())) by {
                    assert(b@.subrange(b@.len() as int - 18, b@.len() as int)[i as int] == b@[base as int
                        + i as int]);
                    assert(sig@[i as int] == crate::model::tga_signature()[i as int]);
                }
            }
            return false;
        }
        i = i + 1;
    }
    proof {
        assert(b@.subrange(b@.len() as int - 18, b@.len() as int) =~= crate::model::tga_signature()) by {
            assert forall|k: int| 0 <= k < 18 implies b@.subrange(
                b@.len() as int - 18,
                b@.len() as int,
            )[k] == crate::model::tga_signature()[k] by {
                assert(b@.subrange(b@.len() as int - 18, b@.len() as int)[k] == b@[base as int + k]);
                assert(sig@[k] == crate::model::tga_signature()[k]);
            }
        }
    }
    true
}

/// R1/R2: the 18-byte header prefix, validated in the R14 order up to and
/// including `OutputTooLarge`. `Ok` carries the raw fields; T5's model equality
/// and T2's bounds are the postcondition.
fn parse_header(b: &[u8]) -> (r: Result<Header, Error>)
    ensures
        match crate::model::mh(b@) {
            Ok(m) => match r {
                Ok(h) => hdr_matches(m, h),
                Err(_) => false,
            },
            Err(e2) => match r {
                Ok(_) => false,
                Err(e) => e == e2,
            },
        },
        match r {
            Ok(h) => 18 <= b@.len() <= crate::model::max_input() && header_inv(h),
            Err(_) => true,
        },
{
    // R1: input cap first, then the header length.
    if b.len() > MAX_INPUT {
        proof {
            reveal(crate::model::mh);
            reveal(crate::model::max_input);
        }
        return Err(Error::FileTooLarge);
    }
    proof {
        reveal(crate::model::mh);
        reveal(crate::model::max_input);
        assert(b@.len() <= crate::model::max_input());
    }
    if b.len() < 18 {
        proof {
            reveal(crate::model::mh);
        }
        return Err(Error::TruncatedHeader);
    }
    proof {
        reveal(crate::model::mh);
        assert(b@.len() >= 18);
    }

    let image_type = b[2];
    // R1: exactly the six supported types.
    proof { type_ok_eq(image_type); }
    let supported_type = image_type == 1 || image_type == 2 || image_type == 3 || image_type == 9
        || image_type == 10 || image_type == 11;
    if !supported_type {
        proof {
            reveal(crate::model::mh);
        }
        return Err(Error::UnsupportedImageType);
    }
    proof {
        reveal(crate::model::mh);
    }

    let color_map_flag = b[1];
    // R2: the map flag is 0 or 1.
    if color_map_flag > 1 {
        proof {
            reveal(crate::model::mh);
        }
        return Err(Error::InvalidColorMapFlag);
    }
    proof {
        reveal(crate::model::mh);
    }

    let width = le16(b, 12);
    let height = le16(b, 14);
    // R2: both dimensions in 1..=8,192.
    if width < 1 || width > MAX_DIMENSION || height < 1 || height > MAX_DIMENSION {
        proof {
            reveal(crate::model::mh);
            reveal(crate::model::max_dimension);
            assert(crate::model::le16(b@, 12) == width as int);
            assert(crate::model::le16(b@, 14) == height as int);
        }
        return Err(Error::InvalidDimensions);
    }
    proof {
        reveal(crate::model::mh);
        reveal(crate::model::max_dimension);
        assert(crate::model::le16(b@, 12) == width as int);
        assert(crate::model::le16(b@, 14) == height as int);
    }

    let descriptor = b[17];
    // R2: the two high descriptor bits must be zero (no interleaving).
    if descriptor & 0xc0 != 0 {
        proof {
            reveal(crate::model::mh);
        }
        return Err(Error::UnsupportedInterleave);
    }
    proof {
        reveal(crate::model::mh);
    }

    let pixel_depth = b[16];
    // R3: image depth is checked before map metadata or attribute count. The
    // inline `match` mirrors `model::depth_ok` literally.
    let depth_ok = match image_type {
        1 | 9 => pixel_depth == 8 || pixel_depth == 16,
        2 | 10 => {
            pixel_depth == 15 || pixel_depth == 16 || pixel_depth == 24 || pixel_depth == 32
        }
        3 | 11 => pixel_depth == 8 || pixel_depth == 16,
        _ => false,
    };
    proof {
        reveal(crate::model::depth_ok);
        reveal(crate::model::is_indexed);
        if image_type == 1u8 || image_type == 9u8 {
            assert(depth_ok == (crate::model::depth_ok(image_type, pixel_depth)));
        } else if image_type == 2u8 || image_type == 10u8 {
            assert(depth_ok == (crate::model::depth_ok(image_type, pixel_depth)));
        } else {
            assert(depth_ok == (crate::model::depth_ok(image_type, pixel_depth)));
        }
    }
    if !depth_ok {
        proof {
            reveal(crate::model::mh);
        }
        return Err(Error::UnsupportedPixelDepth);
    }
    proof {
        reveal(crate::model::mh);
    }

    let map_first = le16(b, 3);
    let map_length = le16(b, 5);
    let map_entry_depth = b[7];
    // R4: coherence and map depth come before the range check.
    if color_map_flag == 0 {
        if map_first != 0 || map_length != 0 || map_entry_depth != 0 {
            proof {
                reveal(crate::model::mh);
                reveal(crate::model::cm_bad);
                reveal(crate::model::is_indexed);
                assert(crate::model::le16(b@, 3) == map_first as int);
                assert(crate::model::le16(b@, 5) == map_length as int);
            }
            return Err(Error::InvalidColorMap);
        }
        // R4: an indexed image requires an attached map.
        if image_type == 1 || image_type == 9 {
            proof {
                reveal(crate::model::mh);
                reveal(crate::model::cm_bad);
                reveal(crate::model::is_indexed);
                assert(crate::model::le16(b@, 3) == map_first as int);
                assert(crate::model::le16(b@, 5) == map_length as int);
                assert(map_first == 0u32);
                assert(map_length == 0u32);
                assert(map_entry_depth == 0u8);
            }
            return Err(Error::InvalidColorMap);
        }
    } else {
        let md_ok = map_entry_depth == 15 || map_entry_depth == 16 || map_entry_depth == 24
            || map_entry_depth == 32;
        proof {
            reveal(crate::model::map_entry_depth_ok);
            assert(md_ok == crate::model::map_entry_depth_ok(map_entry_depth));
        }
        if map_length == 0 || !md_ok {
            proof {
                reveal(crate::model::mh);
                reveal(crate::model::cm_bad);
                reveal(crate::model::is_indexed);
                reveal(crate::model::map_entry_depth_ok);
                assert(crate::model::le16(b@, 3) == map_first as int);
                assert(crate::model::le16(b@, 5) == map_length as int);
                assert(color_map_flag == 1u8);
            }
            return Err(Error::InvalidColorMap);
        }
        // R4: only after depth/coherence; applies to non-indexed images too.
        if map_first + map_length > 65_536 {
            proof {
                reveal(crate::model::mh);
                reveal(crate::model::cm_bad);
                reveal(crate::model::is_indexed);
                reveal(crate::model::map_entry_depth_ok);
                assert(crate::model::le16(b@, 3) == map_first as int);
                assert(crate::model::le16(b@, 5) == map_length as int);
            }
            return Err(Error::ColorMapRange);
        }
    }
    proof {
        reveal(crate::model::mh);
        reveal(crate::model::cm_bad);
        reveal(crate::model::cm_range_bad);
        reveal(crate::model::is_indexed);
        reveal(crate::model::map_entry_depth_ok);
        assert(crate::model::le16(b@, 3) == map_first as int);
        assert(crate::model::le16(b@, 5) == map_length as int);
    }

    // R5: descriptor attribute count, governed by the image depth for
    // non-indexed images and by the map entry depth for indexed images. The
    // inline `match` mirrors `model::attr_ok` literally.
    let attr = descriptor & 0x0f;
    let attr_ok = if image_type == 1 || image_type == 9 {
        match map_entry_depth {
            15 | 24 => attr == 0,
            16 => attr == 0 || attr == 1,
            32 => attr == 0 || attr == 8,
            _ => false,
        }
    } else if image_type == 2 || image_type == 10 {
        match pixel_depth {
            15 | 24 => attr == 0,
            16 => attr == 0 || attr == 1,
            32 => attr == 0 || attr == 8,
            _ => false,
        }
    } else {
        match pixel_depth {
            8 => attr == 0,
            16 => attr == 0 || attr == 8,
            _ => false,
        }
    };
    proof {
        reveal(crate::model::attr_ok);
        reveal(crate::model::map_attr_ok);
        reveal(crate::model::is_indexed);
        if image_type == 1u8 || image_type == 9u8 {
            assert(attr_ok == crate::model::attr_ok(b@));
        } else if image_type == 2u8 || image_type == 10u8 {
            assert(attr_ok == crate::model::attr_ok(b@));
        } else {
            assert(attr_ok == crate::model::attr_ok(b@));
        }
    }
    if !attr_ok {
        proof {
            reveal(crate::model::mh);
        }
        return Err(Error::InvalidAttributeBits);
    }
    proof {
        reveal(crate::model::mh);
    }

    // R6: `N` and `4*N` with checked u64 arithmetic; cap the output.
    proof {
        assert((width as u64) * (height as u64) <= 67108864) by (nonlinear_arith)
            requires
                width <= 8192u32,
                height <= 8192u32,
        ;
    }
    let n64 = width as u64 * height as u64;
    proof {
        assert(4 * n64 <= u64::MAX) by (nonlinear_arith)
            requires
                n64 <= 67108864,
        ;
    }
    if 4 * n64 > MAX_OUTPUT as u64 {
        proof {
            reveal(crate::model::mh);
            reveal(crate::model::max_output);
            assert(crate::model::le16(b@, 12) == width as int);
            assert(crate::model::le16(b@, 14) == height as int);
            assert(n64 == width as int * (height as int));
        }
        return Err(Error::OutputTooLarge);
    }
    proof {
        reveal(crate::model::mh);
        reveal(crate::model::max_output);
        assert(crate::model::le16(b@, 12) == width as int);
        assert(crate::model::le16(b@, 14) == height as int);
        assert(n64 == width as int * (height as int));
        assert(4 * n64 <= 67108864);
        assert(n64 <= 16777216) by (nonlinear_arith)
            requires
                n64 == width as int * (height as int),
                1 <= width as int <= 8192,
                1 <= height as int <= 8192,
                4 * (width as int * (height as int)) <= 67108864,
        ;
        // N <= 16,777,216, so `n` fits every target's `usize`.
        assert(n64 <= usize::MAX as u64) by (nonlinear_arith)
            requires
                n64 <= 16777216,
        ;
    }
    let h = Header {
        id_length: b[0],
        color_map_flag,
        image_type,
        map_first,
        map_length,
        map_entry_depth,
        x_origin: le16(b, 8),
        y_origin: le16(b, 10),
        width,
        height,
        pixel_depth,
        descriptor,
    };
    proof {
        let m = crate::model::MHModel {
            id_length: b@[0],
            color_map_flag: b@[1],
            image_type: b@[2],
            map_first: crate::model::le16(b@, 3) as u32,
            map_length: crate::model::le16(b@, 5) as u32,
            map_entry_depth: b@[7],
            x_origin: crate::model::le16(b@, 8) as u32,
            y_origin: crate::model::le16(b@, 10) as u32,
            width: crate::model::le16(b@, 12) as u32,
            height: crate::model::le16(b@, 14) as u32,
            pixel_depth: b@[16],
            descriptor: b@[17],
            n: crate::model::le16(b@, 12) * crate::model::le16(b@, 14),
            sample_bytes: crate::model::sample_bytes_of(b@[16]),
            map_bytes: if b@[1] == 1u8 {
                crate::model::le16(b@, 5) * crate::model::sample_bytes_of(b@[7])
            } else {
                0
            },
        };
        reveal(crate::model::mh);
        reveal(crate::model::cm_bad);
        reveal(crate::model::cm_range_bad);
        reveal(crate::model::attr_ok);
        reveal(crate::model::map_attr_ok);
        reveal(crate::model::depth_ok);
        reveal(crate::model::is_indexed);
        reveal(crate::model::map_entry_depth_ok);
        reveal(crate::model::max_dimension);
        reveal(crate::model::max_output);
        reveal(crate::model::max_input);
        reveal(crate::model::sample_bytes_of);
        assert(crate::model::mh(b@) == Ok(m));
        // T5: the executable record reconstructs the model record.
        reveal(hdr_matches);
        assert(hdr_matches(m, h));
        // T2's invariant.
        reveal(header_inv);
        crate::model::lemma_le16_lt(b@, 3);
        crate::model::lemma_le16_lt(b@, 5);
        crate::model::lemma_le16_lt(b@, 12);
        crate::model::lemma_le16_lt(b@, 14);
        assert(1 <= h.spec_sample_bytes() <= 4) by {
            assert(crate::model::depth_ok(h.image_type, h.pixel_depth));
            if crate::model::is_indexed(h.image_type) {
                assert(h.pixel_depth == 8u8 || h.pixel_depth == 16u8);
            } else if h.image_type == 2u8 || h.image_type == 10u8 {
                assert(h.pixel_depth == 15u8 || h.pixel_depth == 16u8 || h.pixel_depth == 24u8
                    || h.pixel_depth == 32u8);
            } else {
                assert(h.pixel_depth == 8u8 || h.pixel_depth == 16u8);
            }
            crate::model::lemma_sample_bytes_le_4(h.pixel_depth);
        }
        if h.color_map_flag == 1u8 {
            assert(crate::model::map_entry_depth_ok(h.map_entry_depth));
            crate::model::lemma_sample_bytes_le_4(h.map_entry_depth);
        }
        assert(header_inv(h));
    }
    Ok(h)
}

/// The conversion-relevant projection of an accepted header (see
/// `crate::model::PktH`): image kind, depth, attribute count, the map's
/// `first`/`length`/entry depth, `N` and the sample width. `MR` and the
/// conversion model are stated over this record, so their statements do not
/// depend on the executable `Header` type.
spec fn pkt_of(h: &Header) -> crate::model::PktH {
    crate::model::PktH {
        kind: if h.spec_is_indexed() {
            0
        } else if h.image_type == 2u8 || h.image_type == 10u8 {
            1
        } else {
            2
        },
        depth: h.pixel_depth as int,
        attr: h.spec_attr() as int,
        map_first: h.map_first as int,
        map_len: h.map_length as int,
        map_depth: h.map_entry_depth as int,
        n: h.width as int * (h.height as int),
        sample_bytes: h.spec_sample_bytes(),
    }
}

/// `header_inv` gives `pkt_of`'s conversion-domain facts.
proof fn lemma_pkt_of_domain(h: &Header)
    requires
        header_inv(*h),
    ensures
        crate::model::pkt_of_domain(pkt_of(h)),
{
    reveal(header_inv);
    reveal(pkt_of);
    reveal(crate::model::pkt_of_domain);
    reveal(crate::model::is_indexed);
    reveal(crate::model::depth_ok);
    reveal(crate::model::map_entry_depth_ok);
    reveal(crate::model::sample_bytes_of);
    if crate::model::is_indexed(h.image_type) {
        assert(h.pixel_depth == 8u8 || h.pixel_depth == 16u8);
    } else if h.image_type == 2u8 || h.image_type == 10u8 {
        assert(h.pixel_depth == 15u8 || h.pixel_depth == 16u8 || h.pixel_depth == 24u8
            || h.pixel_depth == 32u8);
    } else {
        assert(h.pixel_depth == 8u8 || h.pixel_depth == 16u8);
    }
}

// --- R8 conversion helpers -------------------------------------------------

/// R8: expand a five-bit channel `c` to eight bits by bit replication.
fn expand5(c: u16) -> (r: u8)
    requires
        c < 32,
    ensures
        r as int == crate::model::rep5(c as int),
{
    let v = (c << 3) | (c >> 2);
    proof {
        reveal(crate::model::rep5);
        assert((c & 31) == c) by (bit_vector)
            requires
                c < 32,
        ;
        assert(v as int == c as int * 8 + c as int / 4) by (bit_vector)
            requires
                c < 32,
                v == (c << 3) | (c >> 2),
        ;
        assert(c as int * 8 + c as int / 4 < 256) by (nonlinear_arith)
            requires
                c < 32,
        ;
    }
    v as u8
}

/// R8: convert a 15/16-bit little-endian colour word `v` with the given alpha.
/// Blue is `v&31`, green `(v>>5)&31`, red `(v>>10)&31`.
fn conv555(v: u16, alpha: u8) -> (r: [u8; 4])
    ensures
        r@.len() == 4,
        r@[3] == alpha,
        r@ == crate::model::rgb555(v as int).push(alpha),
{
    proof {
        reveal(crate::model::rgb555);
        assert(((v >> 10) & 31) < 32) by (bit_vector);
        assert(((v >> 5) & 31) < 32) by (bit_vector);
        assert((v & 31) < 32) by (bit_vector);
        crate::model::lemma_rgb_fields(v as u32);
    }
    [
        expand5((v >> 10) & 31),
        expand5((v >> 5) & 31),
        expand5(v & 31),
        alpha,
    ]
}

/// R8: the per-entry attribute policy, resolved from the entry depth and the
/// authoritative attribute count. Used for both map entries and true-colour
/// samples (the same 15/16/24/32 depths share the bit layout).
fn entry_alpha(entry_depth: u8, attr: u8, v: u16, a_byte: u8) -> (r: u8)
    ensures
        r == crate::model::entry_alpha_spec(
            entry_depth as int,
            attr as int,
            v as int,
            a_byte as int,
        ),
{
    proof {
        reveal(crate::model::entry_alpha_spec);
    }
    match entry_depth {
        15 => 255,
        16 => {
            proof {
                crate::model::lemma_bit15(v as u32);
                assert((v & 0x8000u16 == 0u16) == !crate::model::bit15(v as int)) by {
                    assert(crate::model::bit15(v as int) == (v as u32 & 0x8000u32 != 0u32));
                    assert((v & 0x8000u16) == (v as u32 & 0x8000u32) as u16) by (bit_vector);
                }
            }
            if attr == 1 && v & 0x8000 == 0 {
                0
            } else {
                255
            }
        }
        24 => 255,
        32 => {
            if attr == 8 {
                a_byte
            } else {
                255
            }
        }
        _ => 255,
    }
}

/// R8: convert one non-indexed sample of exactly `sample_bytes` bytes into RGBA.
/// The precondition ties the slice length to the header's depth, so every read
/// below is in bounds.
fn convert_sample(h: &Header, s: &[u8]) -> (r: [u8; 4])
    requires
        s@.len() == h.spec_sample_bytes(),
        crate::model::depth_ok(h.image_type, h.pixel_depth),
        crate::model::type_ok(h.image_type),
        !crate::model::is_indexed(h.image_type),
        1 <= h.spec_sample_bytes() <= 4,
        crate::model::pkt_of_domain(pkt_of(h)),
        h.spec_sample_bytes() == crate::model::stride_of(h.pixel_depth as int),
    ensures
        r@.len() == 4,
        // The colour-conversion lemma's byte-exact result for non-indexed samples.
        r@ == crate::model::conv_sample(pkt_of(h), s@),
{
    proof {
        reveal(pkt_of);
        reveal(crate::model::pkt_of_domain);
    }
    let attr = h.attribute_count();
    let r: [u8; 4] = match h.image_type {
        2 | 10 => {
            if h.pixel_depth == 15 || h.pixel_depth == 16 {
                proof {
                    reveal(crate::model::sample_bytes_of);
                    reveal(crate::model::conv_sample);
                    reveal(crate::model::bytes4_15);
                    assert(h.spec_sample_bytes() == 2);
                    assert(h.pixel_depth as int == 15 || h.pixel_depth as int == 16);
                    assert(pkt_of(h).kind == 1);
                }
                let b0 = s[0];
                let b1 = s[1];
                let v = b0 as u16 | (b1 as u16) << 8;
                proof {
                    reveal(crate::model::le16s);
                    assert(s@[0] == b0);
                    assert(s@[1] == b1);
                    assert(v as int == (b0 as int) + 256 * (b1 as int)) by (bit_vector)
                        requires
                            v == (b0 as u16) | ((b1 as u16) << 8),
                    ;
                    assert(crate::model::le16s(s@, 0) == (b0 as int) + 256 * (b1 as int));
                }
                conv555(v, 255)
            } else if h.pixel_depth == 24 {
                let rr = [s[2], s[1], s[0], 255];
                proof {
                    reveal(crate::model::sample_bytes_of);
                    reveal(crate::model::conv_sample);
                    assert(h.spec_sample_bytes() == 3);
                    assert(pkt_of(h).kind == 1);
                    assert(!(h.pixel_depth == 15) && !(h.pixel_depth == 16));
                    assert(pkt_of(h).depth == 24);
                    assert(rr@ == seq![s@[2], s@[1], s@[0], 255u8]) by {
                        assert(rr@[0] == s@[2]);
                        assert(rr@[1] == s@[1]);
                        assert(rr@[2] == s@[0]);
                        assert(rr@[3] == 255u8);
                        assert(rr@ =~= seq![s@[2], s@[1], s@[0], 255u8]) by {
                            assert(rr@.len() == 4);
                        }
                    }
                }
                rr
            } else {
                proof {
                    reveal(crate::model::depth_ok);
                    reveal(crate::model::sample_bytes_of);
                    reveal(crate::model::conv_sample);
                    assert(h.pixel_depth == 32u8);
                    assert(h.spec_sample_bytes() == 4);
                    assert(pkt_of(h).kind == 1);
                    assert(pkt_of(h).depth == 32);
                    assert(pkt_of(h).attr == attr as int);
                }
                let alpha = if attr == 8 { s[3] } else { 255 };
                proof {
                    assert(alpha == if attr == 8 { s@[3] } else { 255u8 });
                }
                let rr = [s[2], s[1], s[0], alpha];
                proof {
                    assert(rr@ =~= seq![
                        s@[2],
                        s@[1],
                        s@[0],
                        (if attr == 8 {
                            s@[3]
                        } else {
                            255u8
                        }),
                    ]) by {
                        assert(rr@.len() == 4);
                    }
                }
                rr
            }
        }
        3 | 11 => {
            if h.pixel_depth == 8 {
                proof {
                    reveal(crate::model::sample_bytes_of);
                    reveal(crate::model::conv_sample);
                    assert(h.spec_sample_bytes() == 1);
                    assert(pkt_of(h).kind == 2);
                    assert(pkt_of(h).depth == 8);
                }
                [s[0], s[0], s[0], 255]
            } else {
                proof {
                    reveal(crate::model::depth_ok);
                    reveal(crate::model::sample_bytes_of);
                    reveal(crate::model::conv_sample);
                    assert(h.pixel_depth == 16u8);
                    assert(h.spec_sample_bytes() == 2);
                    assert(pkt_of(h).kind == 2);
                    assert(pkt_of(h).depth == 16);
                    assert(pkt_of(h).attr == attr as int);
                }
                let alpha = if attr == 8 { s[1] } else { 255 };
                proof {
                    assert(alpha == if attr == 8 { s@[1] } else { 255u8 });
                }
                let rr = [s[0], s[0], s[0], alpha];
                proof {
                    assert(rr@ =~= seq![
                        s@[0],
                        s@[0],
                        s@[0],
                        (if attr == 8 {
                            s@[1]
                        } else {
                            255u8
                        }),
                    ]) by {
                        assert(rr@.len() == 4);
                    }
                }
                rr
            }
        }
        _ => {
            proof {
                reveal(crate::model::is_indexed);
                reveal(crate::model::type_ok);
                assert(false);
            }
            [0, 0, 0, 255]
        }
    };
    proof {
        reveal(crate::model::conv_sample);
        reveal(pkt_of);
        reveal(crate::model::is_indexed);
        reveal(crate::model::type_ok);
        reveal(crate::model::depth_ok);
        // the non-indexed image types are 2/10 (true-colour) and 3/11 (greyscale)
        if h.image_type == 2u8 || h.image_type == 10u8 {
            assert(pkt_of(h).kind == 1);
        } else {
            assert(h.image_type == 3u8 || h.image_type == 11u8);
            assert(pkt_of(h).kind == 2);
        }
    }
    r
}

/// R8/R4: convert map entry `index` (an absolute index in `first..first+length`)
/// to RGBA. The caller has validated `index` and `map`'s length, so the slice
/// read is in bounds.
fn convert_index(h: &Header, map: &[u8], index: u32) -> (r: [u8; 4])
    requires
        header_inv(*h),
        (h.map_first as int) <= index as int,
        (index as int) < (h.map_first as int + h.map_length as int),
        map@.len() == map_len_of(h.color_map_flag, h.map_length, h.spec_map_stride()),
        crate::model::map_entry_depth_ok(h.map_entry_depth),
        1 <= h.spec_map_stride() <= 4,
    ensures
        r@.len() == 4,
        // The colour-conversion lemma's map-entry result for a validated index.
        r@ == crate::model::conv_map(pkt_of(h), map@, index as int),
{
    proof {
        reveal(header_inv);
        reveal(pkt_of);
        reveal(crate::model::map_entry_depth_ok);
        crate::model::lemma_stride_eq(h.map_entry_depth);
        assert(h.spec_map_stride() == crate::model::stride_of(h.map_entry_depth as int));
        assert(h.map_entry_depth == 15u8 || h.map_entry_depth == 16u8 || h.map_entry_depth == 24u8
            || h.map_entry_depth == 32u8);
        assert(crate::model::stride_of(h.map_entry_depth as int) == 2
            || crate::model::stride_of(h.map_entry_depth as int) == 3
            || crate::model::stride_of(h.map_entry_depth as int) == 4);
    }
    let stride = h.map_bytes_per_entry();
    proof {
        reveal(header_inv);
        reveal(crate::model::stride_of);
        assert(stride as int == h.spec_map_stride());
        assert((index - h.map_first) as int == index as int - h.map_first as int);
        assert((((index - h.map_first) as int)) < h.map_length as int);
        assert(((((index - h.map_first) as int)) * (stride as int) + (stride as int))
            <= usize::MAX) by (nonlinear_arith)
            requires
                ((((index - h.map_first) as int))) < h.map_length as int,
                h.map_length as int <= 65535,
                stride as int == h.spec_map_stride(),
                1 <= h.spec_map_stride() <= 4,
        ;
        crate::model::lemma_map_index_bound(
            (index - h.map_first) as int,
            h.map_length as int,
            h.spec_map_stride(),
        );
    }
    let j = (index - h.map_first) as usize;
    let start = j * stride;
    proof {
        reveal(crate::model::conv_map);
        reveal(crate::model::stride_of);
        assert(start as int == (index as int - h.map_first as int) * crate::model::stride_of(
            h.map_entry_depth as int,
        ));
        if h.color_map_flag == 1u8 {
            assert(start as int + stride as int <= h.map_length as int * h.spec_map_stride());
        } else {
            assert(h.map_length == 0);
            assert(map@.len() == 0);
            assert(start == 0);
        }
        assert(start as int + stride as int <= map@.len());
        assert(start as int >= 0);
    }
    let s = &map[start..start + stride];
    proof {
        assert(s@ == map@.subrange(start as int, start as int + stride as int));
        assert(s@.len() == stride as int);
    }
    let attr = h.attribute_count();
    let r: [u8; 4] = match h.map_entry_depth {
        15 | 16 => {
            proof {
                reveal(crate::model::sample_bytes_of);
                reveal(crate::model::stride_of);
                assert(h.spec_map_stride() == 2);
                assert(pkt_of(h).map_depth == 15 || pkt_of(h).map_depth == 16);
                assert(pkt_of(h).map_first == h.map_first as int);
                assert(pkt_of(h).attr == attr as int);
            }
            let b0 = s[0];
            let b1 = s[1];
            let v = b0 as u16 | (b1 as u16) << 8;
            proof {
                reveal(crate::model::le16s);
                assert(s@[0] == b0);
                assert(s@[1] == b1);
                assert(v as int == (b0 as int) + 256 * (b1 as int)) by (bit_vector)
                    requires
                        v == (b0 as u16) | ((b1 as u16) << 8),
                ;
                crate::model::lemma_bit15(v as u32);
                if v as u32 >= 32768u32 {
                    assert(crate::model::bit15(v as int));
                }
                assert((v & 0x8000u16 == 0u16) == !crate::model::bit15(v as int)) by {
                    assert(crate::model::bit15(v as int) == (v as u32 & 0x8000u32 != 0u32));
                    assert((v & 0x8000u16) == (v as u32 & 0x8000u32) as u16) by (bit_vector);
                }
            }
            if h.map_entry_depth == 15 {
                proof {
                    reveal(crate::model::conv_map);
                    reveal(crate::model::entry_alpha_spec);
                    reveal(crate::model::le16s);
                    assert(crate::model::conv_map(pkt_of(h), map@, index as int) == crate::model::bytes4_15(
                        crate::model::le16s(map@, start as int),
                        255,
                    ));
                    assert(crate::model::le16s(map@, start as int) == crate::model::le16s(s@, 0));
                }
                conv555(v, 255)
            } else {
                let alpha = entry_alpha(h.map_entry_depth, attr, v, 0);
                proof {
                    assert(alpha == crate::model::entry_alpha_spec(16, attr as int, v as int, 0));
                    assert(crate::model::conv_map(pkt_of(h), map@, index as int) == crate::model::rgb555(
                        crate::model::le16s(map@, start as int),
                    ).push(if attr == 1u8 && !crate::model::bit15(crate::model::le16s(map@, start as int)) {
                        0u8
                    } else {
                        255u8
                    }));
                }
                conv555(v, alpha)
            }
        }
        24 => {
            proof {
                reveal(crate::model::sample_bytes_of);
                reveal(crate::model::stride_of);
                assert(h.spec_map_stride() == 3);
                assert(pkt_of(h).map_depth == 24);
                assert(crate::model::conv_map(pkt_of(h), map@, index as int) == seq![
                    map@[start as int + 2],
                    map@[start as int + 1],
                    map@[start as int],
                    255u8,
                ]);
                assert(s@[2] == map@[start as int + 2]);
                assert(s@[1] == map@[start as int + 1]);
                assert(s@[0] == map@[start as int]);
            }
            let rr = [s[2], s[1], s[0], 255];
            proof {
                assert(rr@ =~= seq![
                    map@[start as int + 2],
                    map@[start as int + 1],
                    map@[start as int],
                    255u8,
                ]) by {
                    assert(rr@.len() == 4);
                }
            }
            rr
        }
        _ => {
            proof {
                reveal(crate::model::map_entry_depth_ok);
                reveal(crate::model::sample_bytes_of);
                reveal(crate::model::stride_of);
                assert(h.map_entry_depth == 32u8);
                assert(h.spec_map_stride() == 4);
                assert(pkt_of(h).map_depth == 32);
                assert(pkt_of(h).attr == attr as int);
                assert(crate::model::conv_map(pkt_of(h), map@, index as int) == seq![
                    map@[start as int + 2],
                    map@[start as int + 1],
                    map@[start as int],
                    (if attr == 8 {
                        map@[start as int + 3]
                    } else {
                        255u8
                    }),
                ]);
                assert(s@[3] == map@[start as int + 3]);
                assert(s@[2] == map@[start as int + 2]);
                assert(s@[1] == map@[start as int + 1]);
                assert(s@[0] == map@[start as int]);
            }
            let alpha = entry_alpha(h.map_entry_depth, attr, 0, s[3]);
            proof {
                assert(alpha == if attr == 8 { s@[3] } else { 255u8 });
            }
            let rr = [s[2], s[1], s[0], alpha];
            proof {
                assert(rr@ =~= seq![
                    map@[start as int + 2],
                    map@[start as int + 1],
                    map@[start as int],
                    (if attr == 8 {
                        map@[start as int + 3]
                    } else {
                        255u8
                    }),
                ]) by {
                    assert(rr@.len() == 4);
                }
            }
            rr
        }
    };
    r
}

/// R8: read an index from a 1- or 2-byte little-endian indexed sample.
fn sample_index(b: &[u8]) -> (r: u32)
    requires
        b@.len() >= 1,
        b@.len() <= 2,
    ensures
        r as int == crate::model::sindex(b@, b@.len() as int),
{
    proof {
        reveal(crate::model::sindex);
    }
    if b.len() >= 2 {
        let b0 = b[0];
        let b1 = b[1];
        let v = b0 as u32 | (b1 as u32) << 8;
        proof {
            assert(b@[0] == b0);
            assert(b@[1] == b1);
            assert(b@.len() == 2);
            assert(v as int == (b0 as int) + 256 * (b1 as int)) by (bit_vector)
                requires
                    v == (b0 as u32) | ((b1 as u32) << 8),
            ;
        }
        v
    } else {
        proof {
            assert(b@.len() == 1);
        }
        b[0] as u32
    }
}

/// R8/R11: the indexed-sample range test, `first <= index < first+length`.
fn index_in_range(h: &Header, index: u32) -> (r: bool)
    requires
        (h.map_first as int + h.map_length as int) <= 65536,
    ensures
        r == (h.map_first as int <= index as int && (index as int) < (h.map_first as int
            + h.map_length as int)),
{
    index >= h.map_first && index < h.map_first + h.map_length
}

/// R9: the flipped x coordinate `X = width-1-x` when bit 4 is set, else `x`.
spec fn spec_ox(width: u32, flip_x: bool, x: int) -> int {
    if flip_x {
        width as int - 1 - x
    } else {
        x
    }
}

/// R9: the flipped y coordinate `Y = y` when bit 5 is set, else `height-1-y`.
spec fn spec_oy(height: u32, flip_y: bool, y: int) -> int {
    if flip_y {
        y
    } else {
        height as int - 1 - y
    }
}

/// R9: the oriented output offset of pixel `k`, `4*(Y*width+X)`, in bytes.
/// `k < N` and `1 <= width,height <= 8192`, so the offset is `< 4*N`, which R6
/// already bounded by `MAX_OUTPUT`.
fn output_offset(width: u32, height: u32, flip_x: bool, flip_y: bool, k: usize) -> (r: usize)
    requires
        1 <= (width as int) <= 8192,
        1 <= (height as int) <= 8192,
        (k as int) < width as int * (height as int),
    ensures
        r as int == 4 * (spec_oy(height, flip_y, k as int / (width as int)) * (width as int)
            + spec_ox(width, flip_x, k as int % (width as int))),
        0 <= r as int,
        (r as int) <= 4 * (width as int * (height as int) - 1),
{
    let w = width as usize;
    proof {
        crate::model::lemma_coords(width as int, height as int, k as int);
        assert((k as int) % (width as int) >= 0);
        assert(((k as int) % (width as int)) < width as int);
        assert((k as int) / (width as int) >= 0);
        assert(((k as int) / (width as int)) < height as int);
    }
    let x = k % w;
    let y = k / w;
    proof {
        assert(w == width as int);
        assert(x as int == k as int % (width as int));
        assert(y as int == k as int / (width as int));
        assert((x as int) < width as int);
        assert((y as int) < height as int);
        assert(0 <= x as int);
    }
    let bx = if flip_x { w - 1 - x } else { x };
    let by = if flip_y { y } else { height as usize - 1 - y };
    proof {
        let bxi: int = bx as int;
        let byi: int = by as int;
        let wi: int = width as int;
        let hi: int = height as int;
        assert(bxi == spec_ox(width, flip_x, x as int));
        assert(byi == spec_oy(height, flip_y, y as int));
        crate::model::lemma_offset_bound(wi, hi, y as int, x as int, bxi, byi);
        assert(byi * wi + bxi < wi * hi);
        assert(wi * hi <= 67108864) by (nonlinear_arith)
            requires
                1 <= wi <= 8192,
                1 <= hi <= 8192,
        ;
        assert(4 * (wi * hi) <= usize::MAX) by (nonlinear_arith)
            requires
                wi * hi <= 67108864,
        ;
        assert(by * w + bx < wi * hi) by (nonlinear_arith)
            requires
                byi * wi + bxi < wi * hi,
                w == wi,
                byi == by as int,
                bxi == bx as int,
        ;
        assert(4 * (by * w + bx) <= usize::MAX) by (nonlinear_arith)
            requires
                by * w + bx < wi * hi,
                wi * hi <= 67108864,
                w == wi,
        ;
    }
    4 * (by * w + bx)
}

/// R8/R9/R1: write four RGBA bytes at `off`. Replaces `copy_from_slice`
/// (unsupported read/write shape in Verus) with four identical index writes.
fn write_pixel(out: &mut Vec<u8>, off: usize, px: &[u8; 4])
    requires
        off + 4 <= old(out)@.len(),
    ensures
        final(out)@.len() == old(out)@.len(),
        final(out)@[off as int] == px@[0],
        final(out)@[off as int + 1] == px@[1],
        final(out)@[off as int + 2] == px@[2],
        final(out)@[off as int + 3] == px@[3],
        // Every other byte is untouched.
        forall|i: int| 0 <= i < old(out)@.len() && !(off as int <= i < off as int + 4)
            ==> final(out)@[i] == #[trigger] old(out)@[i],
        // The four written bytes are a `quad_at` at `off`.
        forall|i: int| 0 <= i < 4 ==> final(out)@[off as int + i] == px@[i],
{
    out.set(off, px[0]);
    out.set(off + 1, px[1]);
    out.set(off + 2, px[2]);
    out.set(off + 3, px[3]);
}

/// R8/R11: convert one validated sample, checking the index range for indexed
/// samples before any map read.
fn decode_one(h: &Header, map: &[u8], indexed: bool, s: &[u8]) -> (r: Result<[u8; 4], Error>)
    requires
        header_inv(*h),
        map@.len() == map_len_of((*h).color_map_flag, (*h).map_length, (*h).spec_map_stride()),
        indexed == (*h).spec_is_indexed(),
        indexed ==> (*h).color_map_flag == 1u8,
        s@.len() == (*h).spec_sample_bytes(),
        s@.len() >= 1,
    ensures
        match r {
            Ok(px) => px@.len() == 4,
            Err(_) => true,
        },
        // The complete R8 conversion of the sample (`conv2`), including the
        // indexed case's map-entry read after the range test.
        match r {
            Ok(px) => px@ == crate::model::conv2(pkt_of(h), map@, s@),
            Err(_) => true,
        },
        // The index test is the only failure: a sample converts unless it is an
        // indexed sample whose index is out of range, and that is the exact error.
        match r {
            Ok(px) => indexed ==> crate::model::in_range(
                pkt_of(h),
                crate::model::sindex(s@, (*h).spec_sample_bytes()),
            ),
            Err(e) => indexed && !crate::model::in_range(
                pkt_of(h),
                crate::model::sindex(s@, (*h).spec_sample_bytes()),
            ) && e == Error::ColorIndexOutOfRange,
        },
        // The guards of one decodable raw sample, as the model states them.
        match r {
            Ok(px) => crate::model::raw_sample_ok_at(pkt_of(h), s@),
            Err(_) => true,
        },
{
    if indexed {
        proof {
            lemma_pkt_of_domain(h);
            reveal(pkt_of);
            reveal(crate::model::pkt_of_domain);
            reveal(crate::model::conv2);
        }
        let idx = sample_index(s);
        proof {
            assert(idx as int == crate::model::sindex(s@, (*h).spec_sample_bytes()));
            assert(pkt_of(h).kind == 0);
        }
        proof {
            reveal(header_inv);
        }
        let in_range = index_in_range(h, idx);
        if !in_range {
            return Err(Error::ColorIndexOutOfRange);
        }
        proof {
            reveal(header_inv);
            assert((*h).map_first as int <= idx as int);
            assert((idx as int) < ((*h).map_first as int + (*h).map_length as int));
            assert((*h).color_map_flag == 1u8);
            reveal(crate::model::map_entry_depth_ok);
        }
        Ok(convert_index(h, map, idx))
    } else {
        proof {
            reveal(header_inv);
            reveal(pkt_of);
            reveal(crate::model::conv2);
            lemma_pkt_of_domain(h);
            reveal(crate::model::pkt_of_domain);
        }
        Ok(convert_sample(h, s))
    }
}

/// R1..R14: decode one TGA byte slice. Total on every representable slice (T1).
/// On success the allocation requested is exactly `4*N <= 67,108,864` (T2/T1).
pub fn decode(bytes: &[u8]) -> (r: Result<Decoded, Error>)
    ensures
        match r {
            Ok(d) => {
                &&& 1 <= (d.width as int) <= 8192
                &&& 1 <= (d.height as int) <= 8192
                &&& d.rgba@.len() == 4 * (d.width as int * (d.height as int))
                &&& 4 * (d.width as int * (d.height as int)) <= 67108864
            },
            Err(_) => true,
        },
        // T7/T9/T11: `decode(bytes)` equals `MP(bytes)` exactly.
        decode_eq(r, bytes@),
{
    // R1/R2/R6: every header check, including the input cap, runs before any
    // input-dependent allocation (inside `parse_header`).
    let h = match parse_header(bytes) {
        Ok(h) => h,
        Err(e) => {
            proof {
                // T7/T9: an `MH` error is `MP`'s error, in the same R14 order.
                crate::model::lemma_mp_mh_err(bytes@, e);
                reveal(decode_eq);
            }
            return Err(e);
        },
    };
    proof {
        // The header succeeded, so `MH(bytes)` is `Ok`; keep its record.
        assert(18 <= bytes@.len() <= crate::model::max_input());
        reveal(header_inv);
        assert(bytes@.len() <= 67108864);
        assert(1 <= h.width as int <= 8192);
        assert(1 <= h.height as int <= 8192);
        assert((h.width as usize) * (h.height as usize) <= usize::MAX) by (nonlinear_arith)
            requires
                1 <= h.width as int <= 8192,
                1 <= h.height as int <= 8192,
        ;
    }
    let ghost m: crate::model::MHModel = choose|m: crate::model::MHModel| crate::model::mh(
        bytes@,
    ) == Ok(m) && hdr_matches(m, h);
    proof {
        reveal(hdr_matches);
        assert(crate::model::mh(bytes@) == Ok(m));
        assert(hdr_matches(m, h));
        lemma_pkt_of_hdr_matches(m, h);
        crate::model::lemma_mh_selfcons(bytes@, m);
        reveal(crate::model::mh_selfcons);
        assert(m.n == h.width as int * (h.height as int));
        assert(crate::model::pkt_of_mh(m).n >= 0);
        assert(crate::model::mp_id_end(m) == 18 + (h.id_length as int));
        assert(crate::model::mp_map_end(m) == map_end_of(m, h));
        reveal(map_end_of);
    }
    let width = h.width;
    let height = h.height;
    proof {
        reveal(header_inv);
        assert(crate::model::mh(bytes@) == Ok(m));
        assert(hdr_matches(m, h));
        lemma_pkt_of_hdr_matches(m, h);
        crate::model::lemma_mh_selfcons(bytes@, m);
        reveal(crate::model::mh_selfcons);
        assert(m.n == h.width as int * (h.height as int));
        assert(m.width == h.width);
        assert(m.height == h.height);
        assert(1 <= m.width as int <= 8192);
        assert(1 <= m.height as int <= 8192);
        assert(m.n >= 1) by (nonlinear_arith)
            requires
                m.n == h.width as int * (h.height as int),
                1 <= h.width as int,
                1 <= h.height as int,
        ;
        assert(crate::model::pkt_of_mh(m).n >= 0);
    }
    // R6: `N = width*height`; R2 caps each dimension at 8,192 so `4*N` is at
    // most 67,108,864, far below `usize::MAX` on both targets.
    let n = width as usize * height as usize;
    let indexed = h.is_indexed();
    proof {
        reveal(header_inv);
        assert(n == h.width as int * (h.height as int));
        assert(n == m.n);
        assert(4 * n <= 67108864) by (nonlinear_arith)
            requires
                4 * (h.width as int * (h.height as int)) <= 67108864,
                n == h.width as int * (h.height as int),
        ;
        assert(4 * n <= usize::MAX);
    }

    // R6: allocate exactly `4*N` bytes, bounded by MAX_OUTPUT.
    let mut out = vec![0u8; 4 * n];
    proof {
        assert(out@.len() == 4 * n);
    }

    // R7: skip the image ID, then consume the map. `18 <= bytes.len()` holds.
    let id_end = 18usize + h.id_length as usize;
    if id_end > bytes.len() {
        proof {
            reveal(crate::model::mp_id_end);
            assert(crate::model::mp_id_end(m) == 18 + (h.id_length as int));
            assert(id_end as int == 18 + (h.id_length as int));
            assert(crate::model::mp_id_end(m) > bytes@.len());
            crate::model::lemma_mp_trunc_id(bytes@, m);
            reveal(decode_eq);
        }
        return Err(Error::TruncatedImageId);
    }
    let mut cursor = id_end;

    let map_start = cursor;
    let stride = h.map_bytes_per_entry();
    proof {
        reveal(header_inv);
        assert(stride as int == h.spec_map_stride());
        if h.color_map_flag == 1u8 {
            assert(h.map_length as int <= 65535);
            assert(1 <= h.spec_map_stride() <= 4);
            assert((h.map_length as usize) * (stride as usize) <= usize::MAX) by (nonlinear_arith)
                requires
                    1 <= h.map_length as int <= 65535,
                    1 <= h.spec_map_stride() <= 4,
                    stride as int == h.spec_map_stride(),
            ;
        }
    }
    let map_bytes = if h.color_map_flag == 1 {
        (h.map_length as usize) * stride
    } else {
        0
    };
    proof {
        reveal(header_inv);
        if h.color_map_flag == 1u8 {
            assert(map_bytes as int == h.map_length as int * h.spec_map_stride());
        } else {
            assert(h.map_length == 0);
            assert(map_bytes == 0);
        }
        assert(map_bytes as int
            == map_len_of(h.color_map_flag, h.map_length, h.spec_map_stride()));
    }
    proof {
        reveal(header_inv);
        if h.color_map_flag == 1u8 {
            assert(map_bytes as int == h.map_length as int * h.spec_map_stride());
            assert(h.map_length as int <= 65535);
            assert(h.spec_map_stride() <= 4);
            assert(map_bytes <= 262140) by (nonlinear_arith)
                requires
                    map_bytes as int == h.map_length as int * h.spec_map_stride(),
                    h.map_length as int <= 65535,
                    h.spec_map_stride() <= 4,
            ;
        } else {
            assert(map_bytes == 0);
        }
        assert(map_start <= 67108864);
        assert(map_start + map_bytes <= usize::MAX) by (nonlinear_arith)
            requires
                map_bytes <= 262140,
                map_start <= 67108864,
        ;
    }
    let map_end = map_start + map_bytes;
    if map_end > bytes.len() {
        proof {
            reveal(crate::model::mp_id_end);
            reveal(crate::model::mp_map_end);
            assert(map_end as int == 18 + (h.id_length as int) + (m.map_bytes)) by {
                reveal(hdr_matches);
                reveal(map_end_of);
                reveal(crate::model::mp_map_end);
                if h.color_map_flag == 1u8 {
                    assert(m.map_bytes == h.map_length as int * crate::model::sample_bytes_of(
                        h.map_entry_depth,
                    ));
                    assert(h.spec_map_stride() == crate::model::sample_bytes_of(
                        h.map_entry_depth,
                    ));
                    assert(map_bytes as int == h.map_length as int * h.spec_map_stride());
                } else {
                    assert(m.map_bytes == 0);
                    assert(map_bytes == 0);
                }
            }
            assert(crate::model::mp_map_end(m) == map_end as int);
            assert(crate::model::mp_id_end(m) <= bytes@.len()) by (nonlinear_arith)
                requires
                    id_end as int == 18 + (h.id_length as int),
                    crate::model::mp_id_end(m) == 18 + (h.id_length as int),
                    !(id_end > bytes.len()),
            ;
            assert(crate::model::mp_map_end(m) > bytes@.len());
            crate::model::lemma_mp_trunc_map(bytes@, m);
            reveal(decode_eq);
        }
        return Err(Error::TruncatedColorMap);
    }
    let map = &bytes[map_start..map_end];
    proof {
        reveal(header_inv);
        assert(map@.len() == map_bytes as int);
        assert(map@.len() == map_len_of(h.color_map_flag, h.map_length, h.spec_map_stride()));
        assert(map@ == bytes@.subrange(crate::model::mp_id_end(m), crate::model::mp_map_end(m)))
            by {
            assert(map_start as int == crate::model::mp_id_end(m));
            assert(map_end as int == crate::model::mp_map_end(m));
        }
    }
    cursor = map_end;

    proof {
        reveal(header_inv);
        reveal(dec_inv);
        assert(n == h.width as int * (h.height as int));
        assert(1 <= h.width as int);
        assert(1 <= h.height as int);
        assert(n >= 1) by (nonlinear_arith)
            requires
                n == h.width as int * (h.height as int),
                1 <= h.width as int,
                1 <= h.height as int,
        ;
        assert(map@.len() == map_bytes as int);
        assert(map_bytes as int
            == map_len_of(h.color_map_flag, h.map_length, h.spec_map_stride()));
        assert(dec_inv(*&h, map, n));
        assert(indexed == h.spec_is_indexed());
        assert(4 * n <= usize::MAX);
        assert(cursor == map_end);
        assert(cursor as int == crate::model::mp_map_end(m));
    }

    let c0_dec: usize = cursor;
    // The RLE helper's produced counter (T6's ghost produced state, observed
    // concretely); the public decoder always starts it at zero.
    let mut produced_dec: usize = 0;
    // R10/R11: consume `N` pixels, then test the suffix (R12).
    let pr = if h.is_rle() {
        decode_rle(bytes, &h, map, indexed, &mut out, &mut cursor, &mut produced_dec, n, c0_dec, 0)
    } else {
        decode_raw(bytes, &h, map, indexed, &mut out, &mut cursor, n, c0_dec)
    };
    let r = match pr {
        Ok(()) => {
            proof {
                reveal(header_inv);
                assert(cursor <= bytes@.len());
                assert(out@.len() == 4 * n);
                reveal(crate::model::mp_pixels);
                assert(crate::model::pkt_of_mh(m) == pkt_of(&h));
                if h.spec_is_rle() {
                    lemma_rle_result_ok(
                        bytes@,
                        crate::model::pkt_of_mh(m),
                        map@,
                        crate::model::mp_map_end(m),
                        cursor as int,
                    );
                    reveal(crate::model::mr);
                    match crate::model::mr(
                        bytes@,
                        crate::model::pkt_of_mh(m),
                        map@,
                        crate::model::mp_map_end(m),
                        0,
                    ) {
                        Ok(r) => {
                            assert(r.1 == n as int);
                            crate::model::lemma_mr_at_ok(
                                bytes@,
                                crate::model::pkt_of_mh(m),
                                map@,
                                crate::model::mp_map_end(m),
                                r.0,
                                r.1,
                                r.2,
                            );
                            assert(r.2 == crate::model::mr_pix(
                                bytes@,
                                crate::model::pkt_of_mh(m),
                                map@,
                                crate::model::mp_map_end(m),
                                0,
                            ));
                            assert(r.2.len() == n as int);
                            assert(crate::model::mr_pix(
                                bytes@,
                                crate::model::pkt_of_mh(m),
                                map@,
                                crate::model::mp_map_end(m),
                                0,
                            ).len() == n as int);
                        },
                        Err(_) => {},
                    }
                    assert(crate::model::mp_pixels(bytes@, m, crate::model::mp_id_end(m)) == crate::model::mr_pix(
                        bytes@,
                        crate::model::pkt_of_mh(m),
                        map@,
                        crate::model::mp_map_end(m),
                        0,
                    ));
                    assert(crate::model::mp_pixels(bytes@, m, crate::model::mp_id_end(m)).len()
                        == h.width as int * (h.height as int)) by (nonlinear_arith)
                        requires
                            crate::model::mp_pixels(bytes@, m, crate::model::mp_id_end(m)) == crate::model::mr_pix(
                                bytes@,
                                crate::model::pkt_of_mh(m),
                                map@,
                                crate::model::mp_map_end(m),
                                0,
                            ),
                            crate::model::mr_pix(
                                bytes@,
                                crate::model::pkt_of_mh(m),
                                map@,
                                crate::model::mp_map_end(m),
                                0,
                            ).len() == n as int,
                            n == h.width as int * (h.height as int),
                    ;
                    assert(crate::model::rle_quad_ok(
                        out@,
                        crate::model::mr_pix(
                            bytes@,
                            crate::model::pkt_of_mh(m),
                            map@,
                            crate::model::mp_map_end(m),
                            0,
                        ),
                        h.width as int,
                        h.height as int,
                        h.spec_flip_x(),
                        h.spec_flip_y(),
                    )) by {
                        lemma_rle_pixels_eq_from_zero(
                            bytes@,
                            crate::model::pkt_of_mh(m),
                            map@,
                            crate::model::mp_map_end(m),
                            h.width as int,
                            h.height as int,
                            h.spec_flip_x(),
                            h.spec_flip_y(),
                            out@,
                            Ok(()),
                        );
                        reveal(rle_pixels_eq);
                    }
                } else {
                    lemma_raw_result_ok(
                        bytes@,
                        crate::model::pkt_of_mh(m),
                        crate::model::mp_map_end(m),
                        cursor as int,
                    );
                    assert(crate::model::raw_prefix_ok(
                        bytes@,
                        crate::model::pkt_of_mh(m),
                        map@,
                        crate::model::mp_map_end(m),
                        n as int,
                    )) by {
                        reveal(raw_result_eq);
                    }
                }
            }
            // R12: exactly the end, or a suffix of at least 26 bytes ending in
            // the footer signature with its eight preceding offset bytes.
            let suffix = bytes.len() - cursor;
            if suffix != 0 {
                if suffix < 26 {
                    proof {
                        assert(cursor < bytes@.len()) by (nonlinear_arith)
                            requires
                                cursor + suffix == bytes@.len(),
                                suffix != 0,
                        ;
                        assert(bytes@.len() - cursor < 26) by (nonlinear_arith)
                            requires
                                cursor + suffix == bytes@.len(),
                                suffix < 26,
                        ;
                        crate::model::lemma_mp_suffix_short(bytes@, cursor as int);
                        if h.spec_is_rle() {
                            crate::model::lemma_mp_trailing_rle(bytes@, m, cursor as int);
                        } else {
                            crate::model::lemma_mp_trailing_raw(bytes@, m, cursor as int);
                        }
                        assert(crate::model::mp(bytes@) == Err(Error::TrailingBytes));
                    }
                    return Err(Error::TrailingBytes);
                }
                proof {
                    assert(bytes.len() - 18 >= cursor + 8) by {
                        assert(cursor + suffix == bytes@.len());
                    }
                    assert(bytes.len() - 18 + 18 <= bytes@.len());
                    crate::model::lemma_suffix_tail(bytes@, cursor as int);
                }
                let ok = signature_tail(bytes);
                if !ok {
                    proof {
                        crate::model::lemma_mp_suffix_bad(bytes@, cursor as int);
                        if h.spec_is_rle() {
                            crate::model::lemma_mp_trailing_rle(bytes@, m, cursor as int);
                        } else {
                            crate::model::lemma_mp_trailing_raw(bytes@, m, cursor as int);
                        }
                        assert(crate::model::mp(bytes@) == Err(Error::TrailingBytes));
                    }
                    return Err(Error::TrailingBytes);
                }
                proof {
                    crate::model::lemma_mp_suffix_footer(bytes@, cursor as int);
                }
            } else {
                proof {
                    crate::model::lemma_mp_suffix_zero(bytes@, cursor as int);
                }
            }
            proof {
                // T7: the model accepts with the same dimensions and the output
                // is its oriented assembly.
                if h.spec_is_rle() {
                    crate::model::lemma_mp_rle_ok(
                        bytes@,
                        m,
                        cursor as int,
                        crate::model::mr_pix(
                            bytes@,
                            crate::model::pkt_of_mh(m),
                            map@,
                            crate::model::mp_map_end(m),
                            0,
                        ),
                    );
                } else {
                    crate::model::lemma_mp_raw_ok_result(bytes@, m, cursor as int);
                    crate::model::lemma_acc_conv_len(
                        bytes@,
                        crate::model::pkt_of_mh(m),
                        map@,
                        crate::model::mp_map_end(m),
                        m.n,
                    );
                    assert(crate::model::mp_pixels(bytes@, m, crate::model::mp_id_end(m)) == crate::model::acc_conv(
                        bytes@,
                        crate::model::pkt_of_mh(m),
                        map@,
                        crate::model::mp_map_end(m),
                        m.n,
                    ));
                    assert(crate::model::mp_pixels(bytes@, m, crate::model::mp_id_end(m)).len()
                        == h.width as int * (h.height as int)) by (nonlinear_arith)
                        requires
                            crate::model::mp_pixels(bytes@, m, crate::model::mp_id_end(m)) == crate::model::acc_conv(
                                bytes@,
                                crate::model::pkt_of_mh(m),
                                map@,
                                crate::model::mp_map_end(m),
                                m.n,
                            ),
                            crate::model::acc_conv(
                                bytes@,
                                crate::model::pkt_of_mh(m),
                                map@,
                                crate::model::mp_map_end(m),
                                m.n,
                            ).len() == m.n,
                            m.n == h.width as int * (h.height as int),
                            m.n >= 0,
                    ;
                }
                crate::model::lemma_quad_ok_to_orient(
                    out@,
                    crate::model::mp_pixels(bytes@, m, crate::model::mp_id_end(m)),
                    h.width as int,
                    h.height as int,
                    h.spec_flip_x(),
                    h.spec_flip_y(),
                );
                reveal(decode_eq);
                reveal(dec_vals);
                reveal(crate::model::mp_pixels);
                reveal(crate::model::mp_assemble);
                assert(out@ == crate::model::orient_bytes(
                    crate::model::mp_pixels(bytes@, m, crate::model::mp_id_end(m)),
                    h.width as int,
                    h.height as int,
                    h.spec_flip_x(),
                    h.spec_flip_y(),
                    m.n,
                ));
            }
            Ok(Decoded {
                width,
                height,
                rgba: out,
            })
        },
        Err(e) => {
            proof {
                reveal(header_inv);
                assert(cursor <= bytes@.len());
                assert(out@.len() == 4 * n);
                assert(crate::model::pkt_of_mh(m) == pkt_of(&h));
                if h.spec_is_rle() {
                    lemma_rle_result_err(
                        bytes@,
                        crate::model::pkt_of_mh(m),
                        map@,
                        crate::model::mp_map_end(m),
                        cursor as int,
                        e,
                    );
                    crate::model::lemma_mp_rle_err(bytes@, m, e);
                } else {
                    lemma_raw_result_err(
                        bytes@,
                        crate::model::pkt_of_mh(m),
                        crate::model::mp_map_end(m),
                        cursor as int,
                        e,
                    );
                    crate::model::lemma_mp_raw_err_result(bytes@, m, e);
                }
                assert(crate::model::mp(bytes@) == Err(e));
                reveal(decode_eq);
            }
            Err(e)
        },
    };
    r
}

/// T7/T11: a model record `m` matching the executable header gives the same
/// `PktH` projection, so the model's conversion record and the executable's are
/// the same.
proof fn lemma_pkt_of_hdr_matches(m: crate::model::MHModel, h: Header)
    requires
        hdr_matches(m, h),
    ensures
        crate::model::pkt_of_mh(m) == pkt_of(&h),
        crate::model::mp_id_end(m) == 18 + (h.id_length as int),
        crate::model::mp_map_end(m) == map_end_of(m, h),
{
    reveal(hdr_matches);
    reveal(pkt_of);
    reveal(crate::model::pkt_of_mh);
    reveal(crate::model::mp_id_end);
    reveal(crate::model::mp_map_end);
    reveal(crate::model::is_indexed);
    reveal(crate::model::sample_bytes_of);
    reveal(map_end_of);
    crate::model::lemma_mp_pkt_h(m);
    reveal(crate::model::mp_pkt_h);
    reveal(hdr_matches);
    reveal(map_len_of);
    if h.color_map_flag == 1u8 {
        assert(m.map_bytes == h.map_length as int * crate::model::sample_bytes_of(
            h.map_entry_depth,
        ));
        assert(h.spec_map_stride() == crate::model::sample_bytes_of(h.map_entry_depth));
    } else {
        assert(m.map_bytes == 0);
    }
}

/// R10: raw samples, `N` of `sample_bytes` each, availability checked first.
fn decode_raw(
    bytes: &[u8],
    h: &Header,
    map: &[u8],
    indexed: bool,
    out: &mut Vec<u8>,
    cursor: &mut usize,
    n: usize,
    c0g: usize,
) -> (r: Result<(), Error>)
    requires
        dec_inv(*h, map, n),
        *cursor == c0g,
        indexed == (*h).spec_is_indexed(),
        indexed ==> (*h).color_map_flag == 1u8,
        old(out)@.len() == 4 * n,
        bytes@.len() >= 18,
        bytes@.len() <= crate::model::max_input(),
        *cursor <= bytes@.len(),
    ensures
        final(out)@.len() == 4 * n,
        *final(cursor) <= bytes@.len(),
        // T10: raw decoding consumes exactly one sample of `sample_bytes` bytes
        // per iteration, `N` of them on success.
        match r {
            Ok(_) => *final(cursor) == *old(cursor) + n * ((*h).spec_sample_bytes() as usize),
            Err(_) => true,
        },
        // T4: every one of the `N` input samples is written to its oriented
        // byte offset with the exact R8 bytes: total coverage with no gaps.
        match r {
            Ok(_) => crate::model::wrote_samples(
                final(out)@,
                bytes@,
                pkt_of(h),
                map@,
                (*h).width as int,
                (*h).height as int,
                (*h).spec_flip_x(),
                (*h).spec_flip_y(),
                *old(cursor) as int,
                (*h).spec_sample_bytes(),
                n as int,
            ),
            Err(_) => true,
        },
        // T7: the exact `Result` of `MP`'s raw phase — on success the final
        // cursor (the produced pixels are `wrote_samples` above); on failure the
        // exact first error.
        raw_result_eq(bytes@, pkt_of(h), c0g as int, *final(cursor) as int, r),
        // T7: on success every sample was decodable, so the model's own raw
        // run is the `N` input-order conversions.
        match r {
            Ok(_) => crate::model::raw_prefix_ok(bytes@, pkt_of(h), map@, c0g as int, n as int),
            Err(_) => true,
        },
{
    let sb = h.sample_bytes();
    let width = h.width;
    let height = h.height;
    let (fx, fy) = (h.flip_x(), h.flip_y());
    proof {
        reveal(dec_inv);
        reveal(header_inv);
        assert(sb as int == (*h).spec_sample_bytes());
        assert(sb >= 1);
        assert(bytes@.len() <= 67108864);
    }
    let ghost c0: usize = c0g;
    proof {
        assert(c0 as int == c0g as int);
        assert(*cursor == c0);
    }
    proof {
        // The raw prefix invariant at `k=0` is empty, and the model's cursor
        // equation holds trivially.
        reveal(crate::model::raw_prefix_ok);
        assert(0 <= 0 && forall|j: int| 0 <= j < 0 ==> #[trigger] crate::model::raw_sample_ok(
            bytes@,
            pkt_of(h),
            map@,
            c0 as int,
            j,
        ));
    }
    let mut k: usize = 0;
    while k < n
        invariant
            k <= n,
            n <= 16777216,
            // T10: raw decoding consumes exactly one sample per iteration.
            *cursor == c0 + k * sb,
            c0 as int == c0g as int,
            // T4: each of the first `k` samples has already been written to its
            // oriented offset with the exact converted bytes.
            crate::model::wrote_samples(
                out@,
                bytes@,
                pkt_of(h),
                map@,
                width as int,
                height as int,
                fx,
                fy,
                c0 as int,
                sb as int,
                k as int,
            ),
            // T7: the samples `0..k` are decodable, and the model's own cursor
            // recursion is unchanged from the loop start.
            crate::model::raw_prefix_ok(bytes@, pkt_of(h), map@, c0 as int, k as int),
            crate::model::mp_raw_cur(bytes@, pkt_of(h), c0 as int, 0) == crate::model::mp_raw_cur(
                bytes@,
                pkt_of(h),
                c0 as int,
                k as int,
            ),
            dec_inv(*h, map, n),
            indexed == (*h).spec_is_indexed(),
            indexed ==> (*h).color_map_flag == 1u8,
            out@.len() == 4 * n,
            sb as int == (*h).spec_sample_bytes(),
            sb >= 1,
            sb <= 4,
            1 <= (width as int) <= 8192,
            1 <= (height as int) <= 8192,
            width as int * (height as int) == n,
            fx == (*h).spec_flip_x(),
            fy == (*h).spec_flip_y(),
            *cursor <= bytes@.len(),
            bytes@.len() <= crate::model::max_input(),
        decreases n - k,
    {
        if bytes.len() - *cursor < sb {
            proof {
                // T7: the model's cursor recursion reports `TruncatedPixels` at
                // this sample.
                reveal(pkt_of);
                reveal(raw_result_eq);
                reveal(crate::model::raw_prefix_ok);
                assert(pkt_of(h).n == n as int);
                assert(pkt_of(h).sample_bytes == sb as int);
                assert(c0 as int + k as int * sb as int + sb as int > bytes@.len())
                    by (nonlinear_arith)
                    requires
                        *cursor == c0 + k * sb,
                        bytes.len() - *cursor < sb,
                ;
                crate::model::lemma_mp_raw_cur_trunc(bytes@, pkt_of(h), c0 as int, k as int);
                assert(crate::model::mp_raw_cur(bytes@, pkt_of(h), c0 as int, k as int) == Err(
                    Error::TruncatedPixels,
                ));
                assert(crate::model::mp_raw_cur(bytes@, pkt_of(h), c0 as int, 0) == crate::model::mp_raw_cur(
                    bytes@,
                    pkt_of(h),
                    c0 as int,
                    k as int,
                ));
                assert(crate::model::mp_raw_cur(bytes@, pkt_of(h), c0 as int, 0) == Err(
                    Error::TruncatedPixels,
                ));
                assert(raw_result_eq(
                    bytes@,
                    pkt_of(h),
                    c0 as int,
                    *cursor as int,
                    Err(Error::TruncatedPixels),
                ));
            }
            return Err(Error::TruncatedPixels);
        }
        let end = *cursor + sb;
        proof {
            assert(end <= bytes@.len());
            assert(end <= usize::MAX);
        }
        let s = &bytes[*cursor..end];
        proof {
            reveal(dec_inv);
            assert(s@.len() == sb as int);
            assert(s@.len() == (*h).spec_sample_bytes());
            assert(s@ == bytes@.subrange(c0 as int + k as int * sb as int, c0 as int + k as int * sb
                as int + sb as int));
        }
        let rgba = match decode_one(h, map, indexed, s) {
            Ok(px) => px,
            Err(e) => {
                proof {
                    // T7: the model's cursor recursion reports
                    // `ColorIndexOutOfRange` at this sample.
                    reveal(pkt_of);
                    reveal(raw_result_eq);
                    reveal(crate::model::is_indexed);
                    assert(pkt_of(h).n == n as int);
                    assert(pkt_of(h).sample_bytes == sb as int);
                    assert(indexed == (*h).spec_is_indexed());
                    assert(crate::model::is_indexed((*h).image_type));
                    assert(pkt_of(h).kind == 0) by {
                        reveal(pkt_of);
                        reveal(crate::model::is_indexed);
                    }
                    crate::model::lemma_mp_raw_cur_index(bytes@, pkt_of(h), c0 as int, k as int);
                    assert(crate::model::mp_raw_cur(bytes@, pkt_of(h), c0 as int, k as int) == Err(
                        Error::ColorIndexOutOfRange,
                    ));
                    assert(crate::model::mp_raw_cur(bytes@, pkt_of(h), c0 as int, 0) == crate::model::mp_raw_cur(
                        bytes@,
                        pkt_of(h),
                        c0 as int,
                        k as int,
                    ));
                    assert(crate::model::mp_raw_cur(bytes@, pkt_of(h), c0 as int, 0) == Err(
                        Error::ColorIndexOutOfRange,
                    ));
                    assert(e == Error::ColorIndexOutOfRange);
                    assert(raw_result_eq(
                        bytes@,
                        pkt_of(h),
                        c0 as int,
                        *cursor as int,
                        Err(Error::ColorIndexOutOfRange),
                    ));
                }
                return Err(e);
            },
        };
        let off = output_offset(width, height, fx, fy, k);
        proof {
            reveal(dec_inv);
            assert(width as int * (height as int) == n);
            assert(off + 4 <= 4 * n) by (nonlinear_arith)
                requires
                    (off as int) <= 4 * (n - 1),
                    1 <= n,
                    n <= 16777216,
            ;
            assert(4 * n <= usize::MAX);
        }
        proof {
            reveal(dec_inv);
            assert(*cursor as int + sb as int == end as int);
            assert(s@ == bytes@.subrange(*cursor as int, *cursor as int + sb as int));
            assert(rgba@ == crate::model::conv2(pkt_of(h), map@, s@));
            assert(*cursor as int == c0 as int + k as int * sb as int);
        }
        let ghost out_before = out@;
        proof {
            reveal(crate::model::wrote_samples);
            assert(crate::model::wrote_samples(
                out_before,
                bytes@,
                pkt_of(h),
                map@,
                width as int,
                height as int,
                fx,
                fy,
                c0 as int,
                sb as int,
                k as int,
            ));
        }
        write_pixel(out, off, &rgba);
        proof {
            reveal(crate::model::wrote_samples);
            reveal(crate::model::quad_at);
            assert(crate::model::oxr(width as int, fx, k as int % (width as int))
                == spec_ox(width, fx, k as int % (width as int))) by {
                reveal(crate::model::oxr);
                reveal(spec_ox);
            }
            assert(crate::model::oyr(height as int, fy, k as int / (width as int))
                == spec_oy(height, fy, k as int / (width as int))) by {
                reveal(crate::model::oyr);
                reveal(spec_oy);
            }
            assert(4 * crate::model::fcoord(width as int, height as int, fx, fy, k as int)
                == off as int) by {
                reveal(crate::model::fcoord);
            }
            // the newly written pixel is the invariant's `j = k` case
            assert(crate::model::quad_at(
                out@,
                4 * crate::model::fcoord(width as int, height as int, fx, fy, k as int),
                crate::model::conv2(pkt_of(h), map@, bytes@.subrange(
                    c0 as int + k as int * sb as int,
                    c0 as int + k as int * sb as int + sb as int,
                )),
            )) by {
                assert forall|i: int| 0 <= i < 4 implies out@[off as int + i] == #[trigger] rgba@[i] by {}
                assert(out@[off as int] == rgba@[0]);
                assert(out@[off as int + 1] == rgba@[1]);
                assert(out@[off as int + 2] == rgba@[2]);
                assert(out@[off as int + 3] == rgba@[3]);
            }
            // every earlier pixel is preserved: the offsets are distinct
            assert forall|j: int| 0 <= j < k implies #[trigger] crate::model::quad_at(
                out@,
                4 * crate::model::fcoord(width as int, height as int, fx, fy, j),
                crate::model::conv2(pkt_of(h), map@, bytes@.subrange(
                    c0 as int + j * sb as int,
                    c0 as int + j * sb as int + sb as int,
                )),
            ) by {
                let wi: int = width as int;
                let hi: int = height as int;
                assert(0 <= j < wi * hi) by {
                    assert(j < (k as int));
                    assert((k as int) < n);
                    assert(wi * hi == n);
                }
                assert(0 <= (k as int) < wi * hi) by {
                    assert((k as int) < n);
                    assert(wi * hi == n);
                }
                let fj = crate::model::fcoord(width as int, height as int, fx, fy, j);
                let fk = crate::model::fcoord(width as int, height as int, fx, fy, k as int);
                crate::model::lemma_offset_injective(width as int, height as int, fx, fy, j, k as int);
                assert(fj != fk);
                if fj < fk {
                    assert(4 * fj + 4 <= 4 * fk) by (nonlinear_arith)
                        requires
                            fj < fk,
                    ;
                } else {
                    assert(fj > fk);
                    assert(4 * fk + 4 <= 4 * fj) by (nonlinear_arith)
                        requires
                            fj > fk,
                    ;
                }
                // the earlier write is untouched: its four bytes lie outside
                // the range this iteration wrote
                assert(crate::model::wrote_samples(
                    out_before,
                    bytes@,
                    pkt_of(h),
                    map@,
                    width as int,
                    height as int,
                    fx,
                    fy,
                    c0 as int,
                    sb as int,
                    k as int,
                ));
                crate::model::lemma_wrote_samples_at(
                    out_before,
                    bytes@,
                    pkt_of(h),
                    map@,
                    width as int,
                    height as int,
                    fx,
                    fy,
                    c0 as int,
                    sb as int,
                    k as int,
                    j,
                );
                assert(crate::model::quad_at(
                    out_before,
                    4 * fj,
                    crate::model::conv2(pkt_of(h), map@, bytes@.subrange(
                        c0 as int + j * sb as int,
                        c0 as int + j * sb as int + sb as int,
                    )),
                ));
                crate::model::lemma_fcoord_range(width as int, height as int, fx, fy, j);
                assert(4 * fj + 4 <= 4 * n) by (nonlinear_arith)
                    requires
                        0 <= fj < width as int * (height as int),
                        width as int * (height as int) == n,
                ;
                assert(4 * fj + 4 <= out@.len());
                assert forall|i: int| 0 <= i < 4 implies out@[4 * fj + i] == #[trigger] out_before[4
                    * fj + i] by {
                    assert(!(off as int <= 4 * fj + i < off as int + 4));
                    assert(off as int == 4 * fk);
                }
            }
            crate::model::lemma_wrote_samples_step(
                out@,
                out_before,
                bytes@,
                pkt_of(h),
                map@,
                width as int,
                height as int,
                fx,
                fy,
                c0 as int,
                sb as int,
                k as int,
            );
            assert(end as int == c0 as int + k as int * sb as int + sb as int) by (nonlinear_arith)
                requires
                    *cursor as int == c0 as int + k as int * sb as int,
                    end as int == *cursor as int + sb as int,
            ;
            assert(k as int * sb as int + sb as int == (k as int + 1) * sb as int)
                by (nonlinear_arith);
            // T7: the sample `k` is decodable, so the raw prefix extends and the
            // model's cursor recursion is unchanged.
            crate::model::lemma_raw_prefix_ok_step(bytes@, pkt_of(h), map@, c0 as int, k as int);
            crate::model::lemma_mp_raw_cur_step(bytes@, pkt_of(h), c0 as int, k as int);
        }
        *cursor = end;
        k = k + 1;
    }
    proof {
        // T7: the whole raw image decoded, so `mp_raw` is `Ok` with all `N`
        // converted pixels and this final cursor.
        reveal(pkt_of);
        assert(pkt_of(h).n == n as int);
        assert(pkt_of(h).sample_bytes == sb as int);
        assert(k == n);
        assert(crate::model::mp_raw_cur(bytes@, pkt_of(h), c0 as int, n as int) == Ok(
            c0 as int + n as int * sb as int,
        )) by {
            reveal(crate::model::mp_raw_cur);
        }
    }
    Ok(())
}

/// R11: one continuous RLE packet stream across scanlines. The loop calls
/// `rle_packet` once per packet; `produced` is the pixel counter the manifest's
/// ghost produced state is compared to (it starts at `p0g`, the caller's
/// `produced0`). Every exit, success or any of the six failures, is closed by
/// `lemma_rle_post_from_state` from the same loop state.
fn decode_rle(
    bytes: &[u8],
    h: &Header,
    map: &[u8],
    indexed: bool,
    out: &mut Vec<u8>,
    cursor: &mut usize,
    produced: &mut usize,
    n: usize,
    c0g: usize,
    p0g: usize,
) -> (r: Result<(), Error>)
    requires
        dec_inv(*h, map, n),
        *cursor == c0g,
        *produced == p0g,
        p0g <= n,
        indexed == (*h).spec_is_indexed(),
        indexed ==> (*h).color_map_flag == 1u8,
        old(out)@.len() == 4 * n,
        *cursor <= bytes@.len(),
    ensures
        final(out)@.len() == 4 * n,
        *final(cursor) <= bytes@.len(),
        *final(produced) <= n,
        // T6: the exact cursor/produced/error projection of `MR` at the entry
        // state `(c0g, p0g)`.
        rle_result_eq_from(bytes@, pkt_of(h), map@, c0g as int, p0g as int, *final(cursor) as int, r),
        // T6/T7: on success the output holds the model's produced pixels at
        // their oriented cells from `p0g`.
        rle_pixels_eq_from(
            bytes@,
            pkt_of(h),
            map@,
            c0g as int,
            p0g as int,
            (*h).width as int,
            (*h).height as int,
            (*h).spec_flip_x(),
            (*h).spec_flip_y(),
            final(out)@,
            r,
        ),
        rle_full_eq_from(
            bytes@,
            pkt_of(h),
            map@,
            c0g as int,
            p0g as int,
            (*h).width as int,
            (*h).height as int,
            (*h).spec_flip_x(),
            (*h).spec_flip_y(),
            *final(cursor) as int,
            final(out)@,
            r,
        ),
        // T6 on failure: the concrete final cursor equals MR's ghost cursor.
        crate::model::mr_gcur(bytes@, pkt_of(h), map@, c0g as int, p0g as int) == Ok(
            *final(cursor) as int,
        ) || r is Ok,
        // T6 verbatim, on `MR`'s own result tuple: the error, the final/ghost
        // cursor, the final/ghost produced count and the (partial) pixels at
        // their oriented cells in the final output.
        rle_exact_from(
            bytes@,
            pkt_of(h),
            map@,
            c0g as int,
            p0g as int,
            (*h).width as int,
            (*h).height as int,
            (*h).spec_flip_x(),
            (*h).spec_flip_y(),
            *final(cursor) as int,
            *final(produced) as int,
            final(out)@,
            r,
        ),
{
    let width = h.width;
    let height = h.height;
    let (fx, fy) = (h.flip_x(), h.flip_y());
    proof {
        reveal(dec_inv);
        reveal(header_inv);
        reveal(pkt_of);
        assert(pkt_of(h).n == n as int);
        assert(pkt_of(h).sample_bytes == (*h).spec_sample_bytes());
        assert(1 <= pkt_of(h).sample_bytes <= 4);
        assert(c0g as int >= 0);
        assert(p0g as int <= pkt_of(h).n);
        crate::model::lemma_mr_pix_len(bytes@, pkt_of(h), map@, c0g as int, p0g as int);
    }
    let ghost mut acc: Seq<Seq<u8>> = Seq::empty();
    while *produced < n
        invariant
            *produced <= n,
            p0g <= *produced,
            acc.len() == *produced as int - p0g as int,
            crate::model::mr_pix(bytes@, pkt_of(h), map@, c0g as int, p0g as int) == acc + crate::model::mr_pix(
                bytes@,
                pkt_of(h),
                map@,
                *cursor as int,
                *produced as int,
            ),
            crate::model::mr_cur(bytes@, pkt_of(h), map@, c0g as int, p0g as int) == crate::model::mr_cur(
                bytes@,
                pkt_of(h),
                map@,
                *cursor as int,
                *produced as int,
            ),
            crate::model::mr_gcur(bytes@, pkt_of(h), map@, c0g as int, p0g as int) == crate::model::mr_gcur(
                bytes@,
                pkt_of(h),
                map@,
                *cursor as int,
                *produced as int,
            ),
            crate::model::wrote_px_off(
                out@,
                acc,
                width as int,
                height as int,
                fx,
                fy,
                p0g as int,
                *produced as int - p0g as int,
            ),
            dec_inv(*h, map, n),
            indexed == (*h).spec_is_indexed(),
            indexed ==> (*h).color_map_flag == 1u8,
            out@.len() == 4 * n,
            width == (*h).width,
            height == (*h).height,
            fx == (*h).spec_flip_x(),
            fy == (*h).spec_flip_y(),
            *cursor <= bytes@.len(),
        decreases n - *produced,
    {
        let ghost cur0: usize = *cursor;
        let ghost pg0: usize = *produced;
        let ghost acc0: Seq<Seq<u8>> = acc;
        let (pr, g) = rle_packet(bytes, h, map, indexed, out, cursor, produced, n, p0g, Ghost(acc));
        proof {
            acc = g@;
            reveal(pkt_post);
            reveal(pkt_out);
        }
        match pr {
            Ok(()) => {
                proof {
                    assert(crate::model::mr_pix(bytes@, pkt_of(h), map@, c0g as int, p0g as int)
                        == acc0 + crate::model::mr_pix(bytes@, pkt_of(h), map@, cur0 as int, pg0 as int));
                    assert(acc0 + crate::model::mr_pix(bytes@, pkt_of(h), map@, cur0 as int, pg0 as int)
                        == acc + crate::model::mr_pix(bytes@, pkt_of(h), map@, *cursor as int, *produced as int));
                }
            },
            Err(e) => {
                proof {
                    reveal(dec_inv);
                    reveal(pkt_of);
                    assert(pkt_of(h).n == n as int);
                    assert(crate::model::mr_cur(bytes@, pkt_of(h), map@, c0g as int, p0g as int) == Err(
                        crate::model::MrErr::Api(e),
                    ));
                    assert(crate::model::mr_gcur(bytes@, pkt_of(h), map@, c0g as int, p0g as int) == Ok(
                        *cursor as int,
                    ));
                    assert(crate::model::mr_pix(bytes@, pkt_of(h), map@, c0g as int, p0g as int) == acc);
                    lemma_rle_post_from_state(
                        bytes@,
                        pkt_of(h),
                        map@,
                        c0g as int,
                        p0g as int,
                        width as int,
                        height as int,
                        fx,
                        fy,
                        *cursor as int,
                        *produced as int,
                        out@,
                        acc,
                        Err(e),
                    );
                }
                return Err(e);
            },
        }
    }
    proof {
        // T6: the model's packet stream ended with cursor `cursor` and produced
        // count `N` from the entry state `(c0g, p0g)`; the pixels produced since
        // `p0g` are `acc`, written at their oriented cells from `p0g`.
        reveal(dec_inv);
        reveal(pkt_of);
        assert(pkt_of(h).n == n as int);
        assert(*produced == n);
        assert(crate::model::mr_pix(bytes@, pkt_of(h), map@, *cursor as int, *produced as int)
            == Seq::<Seq<u8>>::empty()) by {
            reveal(crate::model::mr_pix);
        }
        assert(crate::model::mr_pix(bytes@, pkt_of(h), map@, c0g as int, p0g as int) == acc) by {
            broadcast use vstd::seq_lib::group_seq_lib_default;
        }
        assert(crate::model::mr_cur(bytes@, pkt_of(h), map@, *cursor as int, *produced as int) == Ok((
            *cursor as int,
            *produced as int,
        ))) by {
            reveal(crate::model::mr_cur);
        }
        lemma_rle_post_from_state(
            bytes@,
            pkt_of(h),
            map@,
            c0g as int,
            p0g as int,
            width as int,
            height as int,
            fx,
            fy,
            *cursor as int,
            *produced as int,
            out@,
            acc,
            Ok(()),
        );
    }
    Ok(())
}

/// R11: one packet of the stream, from the control byte: control presence,
/// then the overrun test, then the repeat or literal packet. `acc0` is the
/// ghost sequence of pixels produced since the helper's entry (`p0g`); the
/// returned ghost sequence is the same after this packet. The result relates
/// the entry state `(*old(cursor), *old(produced))` to the exit state by
/// `pkt_post`, uniformly over the success and the six failure exits.
fn rle_packet(
    bytes: &[u8],
    h: &Header,
    map: &[u8],
    indexed: bool,
    out: &mut Vec<u8>,
    cursor: &mut usize,
    produced: &mut usize,
    n: usize,
    p0g: usize,
    Ghost(acc0): Ghost<Seq<Seq<u8>>>,
) -> (r: (Result<(), Error>, Ghost<Seq<Seq<u8>>>))
    requires
        dec_inv(*h, map, n),
        indexed == (*h).spec_is_indexed(),
        indexed ==> (*h).color_map_flag == 1u8,
        old(out)@.len() == 4 * n,
        *old(cursor) <= bytes@.len(),
        p0g <= *old(produced) < n,
        pkt_out(
            old(out)@,
            acc0,
            (*h).width as int,
            (*h).height as int,
            (*h).spec_flip_x(),
            (*h).spec_flip_y(),
            p0g as int,
            *old(produced) as int,
        ),
    ensures
        final(out)@.len() == 4 * n,
        *final(cursor) <= bytes@.len(),
        *final(produced) <= n,
        pkt_out(
            final(out)@,
            r.1@,
            (*h).width as int,
            (*h).height as int,
            (*h).spec_flip_x(),
            (*h).spec_flip_y(),
            p0g as int,
            *final(produced) as int,
        ),
        pkt_post(
            bytes@,
            pkt_of(h),
            map@,
            *old(cursor) as int,
            *old(produced) as int,
            *final(cursor) as int,
            *final(produced) as int,
            acc0,
            r.1@,
            r.0,
        ),
{
    proof {
        reveal(dec_inv);
        reveal(header_inv);
        reveal(pkt_of);
        assert(pkt_of(h).n == n as int);
        assert(1 <= pkt_of(h).sample_bytes <= 4);
    }
    // Control presence.
    if *cursor >= bytes.len() {
        proof {
            lemma_pkt_ctrl_absent(bytes@, pkt_of(h), map@, *cursor as int, *produced as int, acc0);
        }
        return (Err(Error::RleTruncated), Ghost(acc0));
    }
    let ghost cur0: usize = *cursor;
    let control = bytes[*cursor];
    *cursor = *cursor + 1;
    let count = (control & 0x7f) as usize + 1;
    proof {
        assert(bytes@[cur0 as int] == control);
        assert(control & 0x7f <= 127u8) by (bit_vector);
        assert(count <= 128);
        assert(count >= 1);
        reveal(crate::model::rle_count);
        crate::model::lemma_rle_count_bit(control);
        assert(count as int == crate::model::rle_count(control as int));
        assert(crate::model::rle_repeat(control as int) == (control & 0x80 != 0)) by (bit_vector);
        assert(*cursor == cur0 + 1);
    }

    // Overrun: checked before reading any packet sample.
    if count > n - *produced {
        proof {
            lemma_pkt_overrun(bytes@, pkt_of(h), map@, cur0 as int, *produced as int, acc0);
        }
        return (Err(Error::RleOverrun), Ghost(acc0));
    }
    proof {
        assert((count as int) <= (n as int) - (*produced as int)) by (nonlinear_arith)
            requires
                count <= n - *produced,
                *produced <= n,
        ;
    }

    if control & 0x80 != 0 {
        rle_repeat_packet(
            bytes, h, map, indexed, out, cursor, produced, n, p0g, control, count,
            Ghost(cur0 as int), Ghost(*produced as int), Ghost(acc0),
        )
    } else {
        rle_literal_packet(
            bytes, h, map, indexed, out, cursor, produced, n, p0g, control, count,
            Ghost(cur0 as int), Ghost(*produced as int), Ghost(acc0),
        )
    }
}

/// R11: a repeat packet after its control byte (`*old(cursor)` is the first
/// sample byte; `control` is the control byte at `*old(cursor) - 1`, `count` its
/// run length, already checked against the remaining pixels). Exactly one
/// complete sample is required; it is converted once and written `count` times.
fn rle_repeat_packet(
    bytes: &[u8],
    h: &Header,
    map: &[u8],
    indexed: bool,
    out: &mut Vec<u8>,
    cursor: &mut usize,
    produced: &mut usize,
    n: usize,
    p0g: usize,
    control: u8,
    count: usize,
    Ghost(cur0): Ghost<int>,
    Ghost(pg0): Ghost<int>,
    Ghost(acc0): Ghost<Seq<Seq<u8>>>,
) -> (r: (Result<(), Error>, Ghost<Seq<Seq<u8>>>))
    requires
        dec_inv(*h, map, n),
        indexed == (*h).spec_is_indexed(),
        indexed ==> (*h).color_map_flag == 1u8,
        old(out)@.len() == 4 * n,
        0 <= cur0 < bytes@.len(),
        *old(cursor) as int == cur0 + 1,
        *old(produced) as int == pg0,
        bytes@[cur0] == control,
        crate::model::rle_repeat(control as int),
        count as int == crate::model::rle_count(control as int),
        1 <= count <= 128,
        p0g <= *old(produced) < n,
        count <= n - *old(produced),
        pkt_out(
            old(out)@,
            acc0,
            (*h).width as int,
            (*h).height as int,
            (*h).spec_flip_x(),
            (*h).spec_flip_y(),
            p0g as int,
            *old(produced) as int,
        ),
    ensures
        final(out)@.len() == 4 * n,
        *final(cursor) <= bytes@.len(),
        *final(produced) <= n,
        pkt_out(
            final(out)@,
            r.1@,
            (*h).width as int,
            (*h).height as int,
            (*h).spec_flip_x(),
            (*h).spec_flip_y(),
            p0g as int,
            *final(produced) as int,
        ),
        pkt_post(
            bytes@,
            pkt_of(h),
            map@,
            cur0,
            pg0,
            *final(cursor) as int,
            *final(produced) as int,
            acc0,
            r.1@,
            r.0,
        ),
{
    let sb = h.sample_bytes();
    let width = h.width;
    let height = h.height;
    let (fx, fy) = (h.flip_x(), h.flip_y());
    proof {
        reveal(dec_inv);
        reveal(header_inv);
        reveal(pkt_of);
        reveal(pkt_out);
        assert(sb as int == (*h).spec_sample_bytes());
        assert(sb >= 1);
        assert(sb <= 4);
        assert(pkt_of(h).n == n as int);
        assert(pkt_of(h).sample_bytes == sb as int);
        assert(bytes@[cur0] == control);
        assert(0 <= cur0 < bytes@.len());
        assert(acc0.len() == pg0 - p0g as int);
    }
    // Repeat packet: exactly one complete sample.
    if bytes.len() - *cursor < sb {
        proof {
            assert(cur0 + 1 + sb as int > bytes@.len()) by (nonlinear_arith)
                requires
                    *cursor as int == cur0 + 1,
                    bytes.len() - *cursor < sb,
            ;
            lemma_pkt_rep_missing(bytes@, pkt_of(h), map@, cur0, pg0, acc0);
        }
        return (Err(Error::RleTruncated), Ghost(acc0));
    }
    proof {
        assert(*cursor as int + sb as int <= bytes@.len()) by (nonlinear_arith)
            requires
                bytes.len() - *cursor >= sb,
                sb >= 0,
        ;
    }
    let end = *cursor + sb;
    proof {
        assert(end <= bytes@.len());
        assert(end as int == cur0 + 1 + sb as int);
    }
    let s = &bytes[*cursor..end];
    *cursor = end;
    proof {
        assert(s@.len() == sb as int);
        assert(s@.len() == (*h).spec_sample_bytes());
        assert(s@ == bytes@.subrange(cur0 + 1, cur0 + 1 + sb as int));
        assert(*cursor as int == cur0 + 1 + sb as int);
    }
    let rgba = match decode_one(h, map, indexed, s) {
        Ok(px) => px,
        Err(e) => {
            proof {
                reveal(pkt_of);
                reveal(crate::model::is_indexed);
                assert(indexed);
                assert(pkt_of(h).kind == 0);
                assert(e == Error::ColorIndexOutOfRange);
                assert(!crate::model::in_range(pkt_of(h), crate::model::sindex(s@, sb as int)));
                lemma_pkt_rep_index(bytes@, pkt_of(h), map@, cur0, pg0, acc0);
            }
            return (Err(e), Ghost(acc0));
        },
    };
    proof {
        assert(rgba@ == crate::model::conv2(pkt_of(h), map@, bytes@.subrange(cur0 + 1, cur0 + 1 + sb as int)));
        assert(pkt_of(h).kind == 0 ==> crate::model::in_range(
            pkt_of(h),
            crate::model::sindex(bytes@.subrange(cur0 + 1, cur0 + 1 + sb as int), sb as int),
        )) by {
            reveal(crate::model::is_indexed);
        }
    }
    let ghost g0: usize = *produced;
    let ghost mut acc: Seq<Seq<u8>> = acc0;
    proof {
        crate::model::lemma_pix_pow_zero(rgba@);
        assert(acc == acc0 + crate::model::pix_pow(rgba@, 0)) by {
            broadcast use vstd::seq_lib::group_seq_lib_default;
        }
    }
    let mut i: usize = 0;
    while i < count
        invariant
            i <= count,
            *produced == g0 + i,
            g0 + count <= n,
            *produced <= n,
            count >= 1,
            out@.len() == 4 * n,
            rgba@.len() == 4,
            n <= 16777216,
            width as int * (height as int) == n,
            1 <= (width as int) <= 8192,
            1 <= (height as int) <= 8192,
            fx == (*h).spec_flip_x(),
            fy == (*h).spec_flip_y(),
            p0g <= g0,
            // T6: the repeat packet's `i` copies so far.
            acc == acc0 + crate::model::pix_pow(rgba@, i as int),
            acc.len() == *produced as int - p0g as int,
            crate::model::wrote_px_off(out@, acc, width as int, height as int, fx, fy, p0g as int, *produced as int - p0g as int),
        decreases count - i,
    {
        proof {
            assert(*produced < n) by (nonlinear_arith)
                requires
                    *produced == g0 + i,
                    i < count,
                    g0 + count <= n,
            ;
            assert(acc.len() == *produced as int - p0g as int);
        }
        let off = output_offset(width, height, fx, fy, *produced);
        proof {
            assert(off + 4 <= 4 * n) by (nonlinear_arith)
                requires
                    (off as int) <= 4 * (n - 1),
                    1 <= n,
                    n <= 16777216,
            ;
            assert(4 * n <= usize::MAX);
        }
        let ghost out_before = out@;
        proof {
            crate::model::lemma_wrote_px_off_append_packet(
                out_before,
                acc,
                seq![rgba@],
                width as int,
                height as int,
                fx,
                fy,
                p0g as int,
                *produced as int - p0g as int,
            );
        }
        write_pixel(out, off, &rgba);
        proof {
            assert(4 * crate::model::fcoord(width as int, height as int, fx, fy, *produced as int)
                == off as int) by {
                reveal(crate::model::fcoord);
                assert(crate::model::oxr(width as int, fx, *produced as int % (width as int))
                    == spec_ox(width, fx, *produced as int % (width as int))) by {
                    reveal(crate::model::oxr);
                    reveal(spec_ox);
                }
                assert(crate::model::oyr(height as int, fy, *produced as int / (width as int))
                    == spec_oy(height, fy, *produced as int / (width as int))) by {
                    reveal(crate::model::oyr);
                    reveal(spec_oy);
                }
            }
            assert((acc + seq![rgba@])[*produced as int - p0g as int] == rgba@) by {
                broadcast use vstd::seq::group_seq_lemmas;
            }
            crate::model::lemma_wrote_px_off_write_step(
                out@,
                out_before,
                acc + seq![rgba@],
                width as int,
                height as int,
                fx,
                fy,
                p0g as int,
                *produced as int - p0g as int,
            );
        }
        *produced = *produced + 1;
        i = i + 1;
        proof {
            // extend the ghost accumulator by this copy
            crate::model::lemma_pix_pow_peel(rgba@, i as int);
            assert(acc0 + crate::model::pix_pow(rgba@, i as int) == (acc0 + (crate::model::pix_pow(
                rgba@,
                i as int,
            ) + Seq::<Seq<u8>>::empty()))) by {
                broadcast use vstd::seq_lib::group_seq_lib_default;
            }
            vstd::seq_lib::lemma_concat_associative(
                acc0,
                crate::model::pix_pow(rgba@, i as int - 1),
                seq![rgba@],
            );
            assert(crate::model::pix_pow(rgba@, i as int) == crate::model::pix_pow(
                rgba@,
                i as int - 1,
            ) + seq![rgba@]);
            acc = acc + seq![rgba@];
            assert(acc == acc0 + crate::model::pix_pow(rgba@, i as int));
            assert(acc.len() == *produced as int - p0g as int) by {
                crate::model::lemma_pix_pow_len(rgba@, i as int);
                broadcast use vstd::seq::group_seq_lemmas;
            }
        }
    }
    proof {
        reveal(pkt_out);
        assert(*produced as int == pg0 + count as int);
        assert(acc == acc0 + crate::model::pix_pow(rgba@, count as int));
        lemma_pkt_rep_ok(bytes@, pkt_of(h), map@, cur0, pg0, count as int, rgba@, acc0, acc);
    }
    (Ok(()), Ghost(acc))
}

/// R11: a literal packet after its control byte (`*old(cursor)` is the first
/// sample byte; `control` is the control byte at `*old(cursor) - 1`, `count` its
/// sample count, already checked against the remaining pixels). All
/// `count*sample_bytes` bytes are required before decoding any sample, then
/// each sample's index is checked in order and its pixel written.
fn rle_literal_packet(
    bytes: &[u8],
    h: &Header,
    map: &[u8],
    indexed: bool,
    out: &mut Vec<u8>,
    cursor: &mut usize,
    produced: &mut usize,
    n: usize,
    p0g: usize,
    control: u8,
    count: usize,
    Ghost(cur0): Ghost<int>,
    Ghost(pg0): Ghost<int>,
    Ghost(acc0): Ghost<Seq<Seq<u8>>>,
) -> (r: (Result<(), Error>, Ghost<Seq<Seq<u8>>>))
    requires
        dec_inv(*h, map, n),
        indexed == (*h).spec_is_indexed(),
        indexed ==> (*h).color_map_flag == 1u8,
        old(out)@.len() == 4 * n,
        0 <= cur0 < bytes@.len(),
        *old(cursor) as int == cur0 + 1,
        *old(produced) as int == pg0,
        bytes@[cur0] == control,
        !crate::model::rle_repeat(control as int),
        count as int == crate::model::rle_count(control as int),
        1 <= count <= 128,
        p0g <= *old(produced) < n,
        count <= n - *old(produced),
        pkt_out(
            old(out)@,
            acc0,
            (*h).width as int,
            (*h).height as int,
            (*h).spec_flip_x(),
            (*h).spec_flip_y(),
            p0g as int,
            *old(produced) as int,
        ),
    ensures
        final(out)@.len() == 4 * n,
        *final(cursor) <= bytes@.len(),
        *final(produced) <= n,
        pkt_out(
            final(out)@,
            r.1@,
            (*h).width as int,
            (*h).height as int,
            (*h).spec_flip_x(),
            (*h).spec_flip_y(),
            p0g as int,
            *final(produced) as int,
        ),
        pkt_post(
            bytes@,
            pkt_of(h),
            map@,
            cur0,
            pg0,
            *final(cursor) as int,
            *final(produced) as int,
            acc0,
            r.1@,
            r.0,
        ),
{
    let sb = h.sample_bytes();
    let width = h.width;
    let height = h.height;
    let (fx, fy) = (h.flip_x(), h.flip_y());
    proof {
        reveal(dec_inv);
        reveal(header_inv);
        reveal(pkt_of);
        reveal(pkt_out);
        assert(sb as int == (*h).spec_sample_bytes());
        assert(sb >= 1);
        assert(sb <= 4);
        assert(pkt_of(h).n == n as int);
        assert(pkt_of(h).sample_bytes == sb as int);
        assert(bytes@[cur0] == control);
        assert(0 <= cur0 < bytes@.len());
        assert(acc0.len() == pg0 - p0g as int);
        assert(count * sb <= 512) by (nonlinear_arith)
            requires
                count <= 128,
                sb <= 4,
        ;
    }
    // Literal packet: `count*sample_bytes` bytes, all present before
    // decoding any sample, then each sample's index checked in order.
    let total = count * sb;
    if bytes.len() - *cursor < total {
        proof {
            assert(cur0 + 1 + count as int * sb as int > bytes@.len()) by (nonlinear_arith)
                requires
                    *cursor as int == cur0 + 1,
                    bytes.len() - *cursor < total,
                    total == count * sb,
            ;
            lemma_pkt_lit_missing(bytes@, pkt_of(h), map@, cur0, pg0, acc0);
        }
        return (Err(Error::RleTruncated), Ghost(acc0));
    }
    proof {
        assert(*cursor as int + total as int <= bytes@.len()) by (nonlinear_arith)
            requires
                bytes.len() - *cursor >= total,
                total >= 0,
        ;
    }
    let end = *cursor + total;
    proof {
        assert(end <= bytes@.len());
        assert(end as int == cur0 + 1 + count as int * sb as int) by (nonlinear_arith)
            requires
                *cursor as int == cur0 + 1,
                total == count * sb,
                end as int == *cursor as int + total as int,
        ;
    }
    let packet = &bytes[*cursor..end];
    *cursor = end;
    proof {
        assert(packet@.len() == count * sb);
        assert(packet@ == bytes@.subrange(cur0 + 1, cur0 + 1 + count as int * sb as int));
        assert(cur0 + 1 + count as int * sb as int <= bytes@.len());
    }
    let ghost g0: usize = *produced;
    let ghost mut acc: Seq<Seq<u8>> = acc0;
    proof {
        assert(crate::model::lit_out(bytes@, pkt_of(h), map@, cur0 + 1, 0) == Seq::<Seq<u8>>::empty()) by {
            reveal(crate::model::lit_out);
        }
        assert(acc == acc0 + crate::model::lit_out(bytes@, pkt_of(h), map@, cur0 + 1, 0)) by {
            broadcast use vstd::seq_lib::group_seq_lib_default;
        }
        assert(crate::model::lit_pref(bytes@, pkt_of(h), cur0 + 1, 0)) by {
            reveal(crate::model::lit_pref);
        }
    }
    let mut i: usize = 0;
    while i < count
        invariant
            i <= count,
            *produced == g0 + i,
            g0 + count <= n,
            *produced <= n,
            count >= 1,
            packet@.len() == count * sb,
            packet@.len() <= 512,
            *cursor <= bytes@.len(),
            *cursor as int == cur0 + 1 + count as int * sb as int,
            sb as int == (*h).spec_sample_bytes(),
            sb as int == pkt_of(h).sample_bytes,
            pkt_of(h).n == n as int,
            sb >= 1,
            sb <= 4,
            out@.len() == 4 * n,
            n <= 16777216,
            width as int * (height as int) == n,
            1 <= (width as int) <= 8192,
            1 <= (height as int) <= 8192,
            dec_inv(*h, map, n),
            indexed == (*h).spec_is_indexed(),
            indexed ==> (*h).color_map_flag == 1u8,
            fx == (*h).spec_flip_x(),
            fy == (*h).spec_flip_y(),
            width == (*h).width,
            height == (*h).height,
            0 <= cur0 < bytes@.len(),
            packet@ == bytes@.subrange(cur0 + 1, cur0 + 1 + count as int * sb as int),
            p0g <= g0,
            g0 as int == pg0,
            // T6: the literal packet's `i` conversions so far.
            acc == acc0 + crate::model::lit_out(bytes@, pkt_of(h), map@, cur0 + 1, i as int),
            acc.len() == *produced as int - p0g as int,
            crate::model::wrote_px_off(out@, acc, width as int, height as int, fx, fy, p0g as int, *produced as int - p0g as int),
            crate::model::lit_pref(bytes@, pkt_of(h), cur0 + 1, i as int),
            count as int == crate::model::rle_count(bytes@[cur0] as int),
            !crate::model::rle_repeat(bytes@[cur0] as int),
        decreases count - i,
    {
        proof {
            assert(i < count);
            assert(i * sb + sb <= count * sb) by (nonlinear_arith)
                requires
                    i < count,
                    sb >= 0,
            ;
            assert(i * sb + sb <= packet@.len());
            assert(acc.len() == *produced as int - p0g as int);
        }
        let s = &packet[i * sb..i * sb + sb];
        proof {
            reveal(dec_inv);
            assert(s@.len() == sb as int);
            assert(s@.len() == (*h).spec_sample_bytes());
            assert(*produced < n) by (nonlinear_arith)
                requires
                    *produced == g0 + i,
                    i < count,
                    g0 + count <= n,
            ;
            crate::model::lemma_subrange_of_subrange(
                bytes@,
                cur0 + 1,
                count as int * sb as int,
                i as int,
                sb as int,
            );
            assert(s@ == bytes@.subrange(
                cur0 + 1 + i as int * sb as int,
                cur0 + 1 + i as int * sb as int + sb as int,
            ));
        }
        let rgba = match decode_one(h, map, indexed, s) {
            Ok(px) => px,
            Err(e) => {
                proof {
                    reveal(pkt_of);
                    reveal(crate::model::is_indexed);
                    assert(indexed);
                    assert(pkt_of(h).kind == 0);
                    assert(e == Error::ColorIndexOutOfRange);
                    assert(!crate::model::in_range(pkt_of(h), crate::model::sindex(s@, sb as int)));
                    lemma_pkt_lit_index(
                        bytes@,
                        pkt_of(h),
                        map@,
                        cur0,
                        pg0,
                        count as int,
                        i as int,
                        acc0,
                        acc,
                    );
                    reveal(pkt_out);
                }
                return (Err(e), Ghost(acc));
            },
        };
        let off = output_offset(width, height, fx, fy, *produced);
        proof {
            assert(off + 4 <= 4 * n) by (nonlinear_arith)
                requires
                    (off as int) <= 4 * (n - 1),
                    1 <= n,
                    n <= 16777216,
            ;
            assert(4 * n <= usize::MAX);
            assert(rgba@ == crate::model::conv2(
                pkt_of(h),
                map@,
                bytes@.subrange(
                    cur0 + 1 + i as int * sb as int,
                    cur0 + 1 + i as int * sb as int + sb as int,
                ),
            ));
            assert(pkt_of(h).kind == 0 ==> crate::model::in_range(
                pkt_of(h),
                crate::model::sindex(s@, sb as int),
            )) by {
                reveal(crate::model::is_indexed);
            }
        }
        let ghost out_before = out@;
        proof {
            crate::model::lemma_wrote_px_off_append_packet(
                out_before,
                acc,
                seq![rgba@],
                width as int,
                height as int,
                fx,
                fy,
                p0g as int,
                *produced as int - p0g as int,
            );
        }
        write_pixel(out, off, &rgba);
        proof {
            assert(4 * crate::model::fcoord(width as int, height as int, fx, fy, *produced as int)
                == off as int) by {
                reveal(crate::model::fcoord);
                assert(crate::model::oxr(width as int, fx, *produced as int % (width as int))
                    == spec_ox(width, fx, *produced as int % (width as int))) by {
                    reveal(crate::model::oxr);
                    reveal(spec_ox);
                }
                assert(crate::model::oyr(height as int, fy, *produced as int / (width as int))
                    == spec_oy(height, fy, *produced as int / (width as int))) by {
                    reveal(crate::model::oyr);
                    reveal(spec_oy);
                }
            }
            assert((acc + seq![rgba@])[*produced as int - p0g as int] == rgba@) by {
                broadcast use vstd::seq::group_seq_lemmas;
            }
            crate::model::lemma_wrote_px_off_write_step(
                out@,
                out_before,
                acc + seq![rgba@],
                width as int,
                height as int,
                fx,
                fy,
                p0g as int,
                *produced as int - p0g as int,
            );
        }
        *produced = *produced + 1;
        i = i + 1;
        proof {
            // extend the ghost accumulator by this sample's conversion
            crate::model::lemma_lit_out_snoc(
                bytes@,
                pkt_of(h),
                map@,
                cur0 + 1,
                i as int - 1,
                i as int - 1,
            );
            assert(crate::model::conv2(
                pkt_of(h),
                map@,
                bytes@.subrange(
                    cur0 + 1 + (i as int - 1) * sb as int,
                    cur0 + 1 + (i as int - 1) * sb as int + sb as int,
                ),
            ) == rgba@);
            vstd::seq_lib::lemma_concat_associative(
                acc0,
                crate::model::lit_out(bytes@, pkt_of(h), map@, cur0 + 1, i as int - 1),
                seq![rgba@],
            );
            acc = acc + seq![rgba@];
            assert(acc == acc0 + crate::model::lit_out(bytes@, pkt_of(h), map@, cur0 + 1, i as int));
            assert(acc.len() == *produced as int - p0g as int) by {
                crate::model::lemma_lit_out_len(bytes@, pkt_of(h), map@, cur0 + 1, i as int);
                broadcast use vstd::seq::group_seq_lemmas;
            }
            crate::model::lemma_lit_pref_succ(bytes@, pkt_of(h), cur0 + 1, i as int - 1);
        }
    }
    proof {
        reveal(pkt_out);
        assert(*produced as int == pg0 + count as int);
        lemma_pkt_lit_ok(bytes@, pkt_of(h), map@, cur0, pg0, count as int, acc0, acc);
    }
    (Ok(()), Ghost(acc))
}

// ---------------------------------------------------------------------------
// M4a statement witnesses: the exact T6-T9/T11 claims.
// ---------------------------------------------------------------------------

/// T6 · exact packet semantics, at the manifest's quantification: for each
/// reachable successful `MH` header/map prefix (`dec_inv`, proved by
/// `parse_header`, never assumed), every finite stream continuation `s` and
/// every valid starting `(cursor0, produced0)` with `p0 <= N`, the concrete RLE
/// helper's result equals `MR(s,H,map,cursor0,produced0)` exactly — the
/// cursor/produced/error projection (`rle_result_eq_from`), the produced pixels
/// written at R9's oriented cells from `produced0` (`rle_pixels_eq_from`) and
/// their combination (`rle_full_eq_from`) — with no restriction to valid
/// packets: the packet's control, overrun, whole-packet and index checks are the
/// model's, in that order, and every failure carries the model's exact first
/// error first and the model's ghost cursor/produced/pixels. The helper starts
/// from the cursor and produced count the caller holds (`*old(cursor)`,
/// `*old(produced)`), not only from zero, and the stream `bytes` is any finite
/// slice with `*old(cursor) <= |bytes|` (no input-size cap).
///
/// `rle_exact_from` is the manifest's statement on `MR`'s own result tuple: on
/// `MR = Ok(c', p', pixels)` the helper returned `Ok`, `*final(cursor) = c'`,
/// `*final(produced) = p' = N`, and `pixels` sit at their oriented cells in the
/// final output; on `MR = Err(e, c', p', pixels)` the helper returned exactly
/// `e`, `*final(cursor) = c'`, `*final(produced) = p'`, and the partial `pixels`
/// sit at their oriented cells in the final output.
#[allow(dead_code)]
fn t6_rle_exact(
    bytes: &[u8],
    h: &Header,
    map: &[u8],
    indexed: bool,
    out: &mut Vec<u8>,
    cursor: &mut usize,
    produced: &mut usize,
    n: usize,
) -> (r: Result<(), Error>)
    requires
        dec_inv(*h, map, n),
        *produced <= n,
        indexed == (*h).spec_is_indexed(),
        indexed ==> (*h).color_map_flag == 1u8,
        old(out)@.len() == 4 * n,
        *cursor <= bytes@.len(),
    ensures
        final(out)@.len() == 4 * n,
        *final(cursor) <= bytes@.len(),
        *final(produced) <= n,
        rle_full_eq_from(
            bytes@,
            pkt_of(h),
            map@,
            *old(cursor) as int,
            *old(produced) as int,
            (*h).width as int,
            (*h).height as int,
            (*h).spec_flip_x(),
            (*h).spec_flip_y(),
            *final(cursor) as int,
            final(out)@,
            r,
        ),
        rle_exact_from(
            bytes@,
            pkt_of(h),
            map@,
            *old(cursor) as int,
            *old(produced) as int,
            (*h).width as int,
            (*h).height as int,
            (*h).spec_flip_x(),
            (*h).spec_flip_y(),
            *final(cursor) as int,
            *final(produced) as int,
            final(out)@,
            r,
        ),
{
    let c0: usize = *cursor;
    let p0: usize = *produced;
    let out_len: usize = out.len();
    proof {
        reveal(rle_full_eq_from);
        reveal(rle_result_eq_from);
        reveal(rle_pixels_eq_from);
        reveal(rle_exact_from);
    }
    let r = decode_rle(bytes, h, map, indexed, out, cursor, produced, n, c0, p0);
    proof {
        assert(out@.len() == out_len);
        assert(out_len == 4 * n);
    }
    r
}

/// T9 · precedence, at the manifest's quantification: for every representable
/// input, `decode(b) = Err(e) iff MP(b) = Err(e)` for each R15 error, with the
/// header check order and the RLE control/overrun/full-packet/index order
/// explicit; `decode` returns neither pixels nor a second error. This is the
/// `Err` half of `decode_eq`, stated as a witness over `decode` itself.
#[allow(dead_code)]
fn t9_precedence(bytes: &[u8]) -> (r: Result<Decoded, Error>)
    ensures
        match r {
            Ok(d) => crate::model::mp(bytes@) == Ok((d.width as int, d.height as int, d.rgba@)),
            Err(e) => crate::model::mp(bytes@) == Err(e),
        },
{
    decode(bytes)
}

/// T7/T8/T11 · total functional correctness, success and resource bounds, and
/// the integrated call path, at the manifest's quantification: for every byte
/// slice representable on either target, `decode(bytes)` equals `MP(bytes)` as
/// the exact `Result` — all output bytes, both flips, alpha, map offsets, footer
/// behavior and first error — and on success `1 <= w,h <= 8192`,
/// `|p| = 4*w*h <= 67,108,864`, and every output cell `q < N` satisfies
/// `t8_cell` (the per-pixel clause, proved alongside the public result, not left
/// as an orphan witness).
#[allow(dead_code)]
fn t7_decode_exact(bytes: &[u8]) -> (r: Result<Decoded, Error>)
    ensures
        decode_eq(r, bytes@),
        match r {
            Ok(d) => {
                &&& 1 <= (d.width as int) <= 8192
                &&& 1 <= (d.height as int) <= 8192
                &&& d.rgba@.len() == 4 * (d.width as int * (d.height as int))
                &&& 4 * (d.width as int * (d.height as int)) <= 67108864
                &&& forall|q: int|
                    0 <= q < d.width as int * (d.height as int) ==> #[trigger] crate::model::t8_cell(
                        bytes@,
                        d.width as int,
                        d.height as int,
                        d.rgba@,
                        q,
                    )
            },
            Err(_) => true,
        },
        // T8's "iff": `MP` accepts exactly when `decode` does, with the same triple.
        match r {
            Ok(d) => crate::model::mp(bytes@) == Ok((d.width as int, d.height as int, d.rgba@)),
            Err(e) => crate::model::mp(bytes@) == Err(e),
        },
{
    let r = decode(bytes);
    proof {
        if let Ok(d) = &r {
            assert forall|q: int|
                0 <= q < d.width as int * (d.height as int) implies #[trigger] crate::model::t8_cell(
                bytes@,
                d.width as int,
                d.height as int,
                d.rgba@,
                q,
            ) by {
                crate::model::lemma_mp_t8_cell(
                    bytes@,
                    d.width as int,
                    d.height as int,
                    d.rgba@,
                    q,
                );
            }
        }
    }
    r
}

/// T8 · the per-pixel clause, at the manifest's quantification. On an accepted
/// `MP(b) = Ok(w,h,out)` with `1 <= w,h <= 8192` and `|out| = 4*w*h <=
/// 67,108,864`: for every output cell `q < N = w*h`, `t8_cell` holds — the four
/// bytes `out[4q..4q+4]` are R8's non-premultiplied conversion of the model's
/// source sample `g(q)` at the oriented position `f(g(q)) = q`, with the unique
/// orientation bijection inverse `g(q)` and the successful `MH` record that
/// built the result. The hypothesis is `MP(b) = Ok(...)` itself (Finding 3); the
/// projection from it to the record is `model::lemma_mp_accepted_shape`, and the
/// per-cell content is `model::lemma_mp_t8_cell`.
#[allow(dead_code)]
proof fn t8_output_cells(b: Seq<u8>, w: int, hgt: int, out: Seq<u8>, q: int)
    requires
        crate::model::mp(b) == Ok((w, hgt, out)),
        1 <= w <= 8192,
        1 <= hgt <= 8192,
        0 <= q < w * hgt,
    ensures
        crate::model::t8_cell(b, w, hgt, out, q),
{
    crate::model::lemma_mp_t8_cell(b, w, hgt, out, q);
}

// ---------------------------------------------------------------------------
// M3 statement witnesses: the exact T3/T4/T10 claims.// ---------------------------------------------------------------------------
// M3 statement witnesses: the exact T3/T4/T10 claims.
// ---------------------------------------------------------------------------

/// T3 · packet safety/termination, stated on `decode_rle` at the manifest's
/// quantification: for every accepted header/map prefix (`dec_inv` — proved by
/// `parse_header`, not assumed) and every remaining finite stream `s`, the
/// actual RLE helper terminates (its loop variant is `N - produced`, and
/// `lemma_rle_count_range` shows every one of the 0..255 controls yields a
/// repeat/literal count in 1..=128, so `produced` strictly increases), performs
/// no invalid access (discharged by verifying the body: no `requires` permits a
/// missing byte, an overrunning run or an out-of-palette index), and requests no
/// pixel buffer beyond `4*N` (the output is allocated exactly once as `4*N`, and
/// each `write_pixel` is guarded by `off + 4 <= 4*N`).
#[allow(dead_code)]
fn t3_rle_safe(
    bytes: &[u8],
    h: &Header,
    map: &[u8],
    indexed: bool,
    out: &mut Vec<u8>,
    cursor: &mut usize,
    n: usize,
) -> (r: Result<(), Error>)
    requires
        dec_inv(*h, map, n),
        indexed == (*h).spec_is_indexed(),
        indexed ==> (*h).color_map_flag == 1u8,
        old(out)@.len() == 4 * n,
        *cursor <= bytes@.len(),
    ensures
        final(out)@.len() == 4 * n,
        *final(cursor) <= bytes@.len(),
{
    proof {
        // every control byte in 0..=255 has a count in 1..=128
        assert forall|c: u8| 1 <= #[trigger] crate::model::rle_count(c as int) <= 128 by {
            crate::model::lemma_rle_count_range(c);
        }
    }
    let mut produced: usize = 0;
    decode_rle(bytes, h, map, indexed, out, cursor, &mut produced, n, *cursor, 0)
}

/// T4 · conversion/orientation bounds, stated on a concrete accepted header and
/// every `k<N`:
///  * `sample_bytes` is 1..4 and the conversion reads only those bytes (the
///    colour-conversion lemma);
///  * the oriented byte offset satisfies `4*f(k)+4 <= 4*N`;
///  * distinct `k` write distinct output byte positions (the bijection lemma's
///    injectivity).
#[allow(dead_code)]
proof fn t4_bounds(h: &Header, k: usize, j: usize)
    requires
        header_inv(*h),
        (k as int) < (*h).width as int * ((*h).height as int),
        (j as int) < (*h).width as int * ((*h).height as int),
        k != j,
        h.spec_sample_bytes() == crate::model::stride_of(h.pixel_depth as int),
    ensures
        1 <= (*h).spec_sample_bytes() <= 4,
        4 * crate::model::fcoord(
            (*h).width as int,
            (*h).height as int,
            (*h).spec_flip_x(),
            (*h).spec_flip_y(),
            k as int,
        ) + 4 <= 4 * ((*h).width as int * ((*h).height as int)),
        4 * crate::model::fcoord(
            (*h).width as int,
            (*h).height as int,
            (*h).spec_flip_x(),
            (*h).spec_flip_y(),
            k as int,
        ) != 4 * crate::model::fcoord(
            (*h).width as int,
            (*h).height as int,
            (*h).spec_flip_x(),
            (*h).spec_flip_y(),
            j as int,
        ),
        // the map access of a validated index lies inside the checked slice
        forall|idx: int| #[trigger] crate::model::map_read_in_slice(pkt_of(h), idx),
{
    reveal(header_inv);
    assert(1 <= (*h).spec_sample_bytes() <= 4);
    crate::model::lemma_output_offset_bound(
        (*h).width as int,
        (*h).height as int,
        (*h).spec_flip_x(),
        (*h).spec_flip_y(),
        k as int,
    );
    crate::model::lemma_offset_injective(
        (*h).width as int,
        (*h).height as int,
        (*h).spec_flip_x(),
        (*h).spec_flip_y(),
        j as int,
        k as int,
    );
    reveal(crate::model::map_read_in_slice);
    reveal(pkt_of);
    assert forall|idx: int| #[trigger] crate::model::map_read_in_slice(pkt_of(h), idx) by {
        if (*h).color_map_flag == 1u8 {
            reveal(header_inv);
            reveal(crate::model::stride_of);
            assert(1 <= (*h).spec_map_stride() <= 4);
            crate::model::lemma_stride_eq((*h).map_entry_depth);
            assert(0 <= crate::model::stride_of((*h).map_entry_depth as int));
            if (*h).map_first as int <= idx < (*h).map_first as int + ((*h).map_length as int) {
                crate::model::lemma_map_read_bound(
                    (*h).map_first as int,
                    (*h).map_length as int,
                    crate::model::stride_of((*h).map_entry_depth as int),
                    idx,
                );
            }
        } else {
            reveal(header_inv);
            assert((*h).map_length == 0);
            reveal(crate::model::map_read_in_slice);
            assert(!((*h).map_first as int <= idx < (*h).map_first as int + ((*h).map_length as int)));
        }
    }
}

/// T4 at the loop level: on success the raw loop has written every input sample
/// to its oriented position with the exact bytes, and the RLE helper's writes
/// are all inside `4*N`.
#[allow(dead_code)]
fn t4_raw_total(
    bytes: &[u8],
    h: &Header,
    map: &[u8],
    indexed: bool,
    out: &mut Vec<u8>,
    cursor: &mut usize,
    n: usize,
) -> (r: Result<(), Error>)
    requires
        dec_inv(*h, map, n),
        (indexed == (*h).spec_is_indexed()),
        indexed ==> (*h).color_map_flag == 1u8,
        old(out)@.len() == 4 * n,
        bytes@.len() >= 18,
        bytes@.len() <= crate::model::max_input(),
        *cursor <= bytes@.len(),
        !(*h).spec_is_rle(),
    ensures
        final(out)@.len() == 4 * n,
        *final(cursor) <= bytes@.len(),
        match r {
            Ok(_) => crate::model::wrote_samples(
                final(out)@,
                bytes@,
                pkt_of(h),
                map@,
                (*h).width as int,
                (*h).height as int,
                (*h).spec_flip_x(),
                (*h).spec_flip_y(),
                *old(cursor) as int,
                (*h).spec_sample_bytes(),
                n as int,
            ),
            Err(_) => true,
        },
{
    decode_raw(bytes, h, map, indexed, out, cursor, n, *cursor)
}

/// T10 · termination, stated on the two decoders:
///  * raw decoding advances the cursor by exactly one sample per iteration and
///    stops after exactly `N` samples;
///  * RLE decoding's loop variant `N - produced` strictly decreases because
///    every control yields `count` in 1..=128 (`lemma_rle_count_range`), so the
///    packet loop is bounded by `ceil(N/1)` iterations and returns as soon as a
///    packet is missing, overruns or holds an out-of-range index;
///  * neither decoder recurses on input length, and all writes are guarded by
///    `off + 4 <= 4*N`, so no output write can recurse.
#[allow(dead_code)]
fn t10_termination(
    bytes: &[u8],
    h: &Header,
    map: &[u8],
    indexed: bool,
    out: &mut Vec<u8>,
    cursor: &mut usize,
    n: usize,
) -> (r: Result<(), Error>)
    requires
        dec_inv(*h, map, n),
        indexed == (*h).spec_is_indexed(),
        indexed ==> (*h).color_map_flag == 1u8,
        old(out)@.len() == 4 * n,
        bytes@.len() >= 18,
        bytes@.len() <= crate::model::max_input(),
        *cursor <= bytes@.len(),
    ensures
        final(out)@.len() == 4 * n,
        *final(cursor) <= bytes@.len(),
        // exactly one sample per raw pixel on success
        match r {
            Ok(_) => (*h).spec_is_rle() || *final(cursor) == *old(cursor) + n * ((*h).spec_sample_bytes()
                as usize),
            Err(_) => true,
        },
{
    if h.is_rle() {
        let mut produced: usize = 0;
        decode_rle(bytes, h, map, indexed, out, cursor, &mut produced, n, *cursor, 0)
    } else {
        decode_raw(bytes, h, map, indexed, out, cursor, n, *cursor)
    }
}

// ---------------------------------------------------------------------------
// M2 statement witnesses: the exact T1/T2/T5 claims on the public entry points.
// ---------------------------------------------------------------------------

/// T5, stated exactly as the manifest quantifies it: for every byte slice
/// representable on either target, `parse_header(b)` equals `MH(b)` — the exact
/// success record or the exact first error, including the R14 check order.
#[allow(dead_code)]
fn t5_header_exact(b: &[u8]) -> (r: Result<Header, Error>)
    ensures
        match crate::model::mh(b@) {
            Ok(m) => match r {
                Ok(h) => hdr_matches(m, h),
                Err(_) => false,
            },
            Err(e2) => match r {
                Ok(_) => false,
                Err(e) => e == e2,
            },
        },
{
    parse_header(b)
}

/// T2, stated on the accepted record: on success `model::mh` returns a record
/// `m` matching the executable header and satisfying `model::mh_bounds`
/// (`1 <= w,h <= 8,192`, `N <= 16,777,216`, `map_bytes <= 262,140`, strides
/// `1..=4`). `mh_bounds` is proved in the model from `MH` alone.
#[allow(dead_code)]
fn t2_header_bounds(b: &[u8]) -> (r: Result<Header, Error>)
    ensures
        match r {
            Ok(h) => match crate::model::mh(b@) {
                Ok(m) => hdr_matches(m, h) && crate::model::mh_bounds(m),
                Err(_) => false,
            },
            Err(e) => match crate::model::mh(b@) {
                Ok(_) => false,
                Err(e2) => e == e2,
            },
        },
{
    proof {
        crate::model::lemma_mh_ok_bounds_auto(b@);
    }
    parse_header(b)
}

/// T1, stated on the whole decode entry: for every byte slice representable on
/// either target, `decode` terminates without panic, out-of-bounds access or
/// unchecked overflow/narrowing (discharged by verifying `decode`, which has no
/// `requires`), rejects an over-long input before any input-dependent
/// allocation, and on success requests exactly `4*N <= 67,108,864` bytes.
#[allow(dead_code)]
fn t1_decode_safe(bytes: &[u8]) -> (r: Result<Decoded, Error>)
    ensures
        match r {
            Ok(d) => {
                &&& 1 <= (d.width as int) <= 8192
                &&& 1 <= (d.height as int) <= 8192
                &&& d.rgba@.len() == 4 * (d.width as int * (d.height as int))
                &&& 4 * (d.width as int * (d.height as int)) <= 67108864
            },
            Err(_) => true,
        },
{
    decode(bytes)
}

} // verus!
