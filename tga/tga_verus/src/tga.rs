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
use alloc::vec;
use alloc::vec::Vec;

use crate::model::{MHModel, PktH};
use crate::policy::{
    AlphaOptions, AttributeBitsOptions, DepthsOptions, ImageTypeOptions, IndexRangeOptions,
    InterleaveOptions, Map16AlphaOptions, Map5Options, MapHeaderOptions, MapIndexOptions,
    Policy, Rgb5Options, RleOverrunOptions,
};

verus! {

/// R1: the input is at most 64 MiB; a larger slice is `FileTooLarge`.
pub const MAX_INPUT: usize = 67_108_864;

/// R6: the RGBA output is at most 64 MiB (`4*width*height`).
pub(crate) const MAX_OUTPUT: usize = 67_108_864;

/// R2: each image dimension is between 1 and 8,192 inclusive.
pub(crate) const MAX_DIMENSION: u32 = 8_192;

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
    /// R15's spelling table, in spec position: the `open` same-`impl` sibling
    /// that `alloc_guard::sites::TryError::name_spec` reads. `open` and
    /// field-free, so it is well-formed everywhere, including in the guard's
    /// `sites.rs`.
    #[doc(hidden)]
    pub open spec fn name_spec(&self) -> &'static str {
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

    /// The exact R15 spelling, as used by `VECTORS.json` expectations.
    ///
    /// No spec clause and no `#[verifier::when_used_as_spec]`, matching DIB's
    /// shipped guard: the attribute disturbs the crate-wide encoding and makes
    /// an unrelated function report a resource-limit failure (dib
    /// `alloc-wip-1` run 504 onward). `name()` is called in exec position only
    /// (the test binaries); the spelling table `name_spec` above and
    /// `TryError::name_spec` are the spec-position statements of R15's names.
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
pub(crate) struct Header {
    pub(crate) id_length: u8,
    pub(crate) color_map_flag: u8,
    pub(crate) image_type: u8,
    pub(crate) map_first: u32,
    pub(crate) map_length: u32,
    pub(crate) map_entry_depth: u8,
    pub(crate) x_origin: u32,
    pub(crate) y_origin: u32,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) pixel_depth: u8,
    pub(crate) descriptor: u8,
}

impl Header {
    pub(crate) open spec fn spec_is_indexed(self) -> bool {
        crate::model::is_indexed(self.image_type)
    }

    pub(crate) open spec fn spec_is_rle(self) -> bool {
        self.image_type == 9u8 || self.image_type == 10u8 || self.image_type == 11u8
    }

    /// R3: `ceil(depth/8)`, one of 1, 2, 3, 4 for supported image depths.
    pub(crate) open spec fn spec_sample_bytes(self) -> int {
        crate::model::sample_bytes_of(self.pixel_depth)
    }

    /// R4/R7: `ceil(map_entry_depth/8)`, one of 1, 2, 3, 4 for a valid map.
    pub(crate) open spec fn spec_map_stride(self) -> int {
        crate::model::sample_bytes_of(self.map_entry_depth)
    }

    /// R5: the attribute count of descriptor bits 0..3.
    pub(crate) open spec fn spec_attr(self) -> u8 {
        self.descriptor & 0x0fu8
    }

    pub(crate) open spec fn spec_flip_x(self) -> bool {
        self.descriptor & 0x10u8 != 0u8
    }

    pub(crate) open spec fn spec_flip_y(self) -> bool {
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
#[doc(hidden)]
pub open spec fn dec_vals(d: Decoded) -> (int, int, Seq<u8>) {
    (d.width as int, d.height as int, d.rgba@)
}

/// T7/T9: `try_decode(bytes)` equals `MP(bytes)` as the exact `Result` — the exact
/// three-tuple, or the exact error. This is the relation the public entry
/// carries; `MP` is the model's definition, not a call to any executable code.
#[doc(hidden)]
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
            b@.len() <= usize::MAX,
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

/// R1..R14: decode one TGA byte slice through the sealed reservation log. Total
/// on every representable slice (T1). On success the allocation requested is
/// exactly `4*N <= 67,108,864` (T2/T1); on a failed reservation the result is
/// `OutOfMemory(Output, 4N)`, never a partial output.
fn decode_tracked(bytes: &[u8], steps: &mut Ghost<nat>,
    Tracked(log): Tracked<&mut AllocLog>) -> (r: Result<Decoded, TryError>)
    requires
        old(log).view() == Seq::<(AllocSite, nat, bool)>::empty(),
    ensures
        // T8's success bounds and T10's termination surface unchanged.
        match r {
            Ok(d) => {
                &&& 1 <= (d.width as int) <= 8192
                &&& 1 <= (d.height as int) <= 8192
                &&& d.rgba@.len() == 4 * (d.width as int * (d.height as int))
                &&& 4 * (d.width as int * (d.height as int)) <= 67108864
            },
            Err(_) => true,
        },
        // T7/T9/T11: the frozen predicate on `crate::alloc_guard::sites::fmt_part(r)` (A-TGA-1 clause 1).
        !matches!(r, Err(TryError::OutOfMemory(_))) ==> decode_eq(crate::alloc_guard::sites::fmt_part(r), bytes@),
        // A-TGA-5 (charge bound, the added TGA obligation; frozen A-TGA-4 is
        // termination): the only counter charge on this path is the `4N` output
        // initialisation, charged after a *successful* reservation. On success
        // `steps` is exactly `4*model_checked_n`; an `OutOfMemory` result did no
        // zeroing, so it charges nothing beyond what was spent before its failed
        // reservation. Both arms stay below `16*(|b|+N)`.
        final(steps)@ as int
            == old(steps)@ as int
                + (if matches!(r, Err(TryError::OutOfMemory(_))) { 0 } else { 4 * model_checked_n(bytes@) }),
        // A-TGA-1/2: the one-reservation log shape.
        at_site_log(bytes@, matches!(r, Err(TryError::OutOfMemory(_))), final(log).view()),
        // The `PreAlloc` corollary: a rejected header never reserves.
        pre_alloc(bytes@) ==> final(log).view().len() == 0,
        // The failure names `Req_TGA`'s one `(Output, 4N)` request.
        match r {
            Err(TryError::OutOfMemory(o)) =>
                (o.site_spec(), o.elements_spec() as nat) == req_tga(bytes@)[0],
            _ => true,
        },
{
    // R1/R2/R6: every header check, including the input cap, runs before any
    // input-dependent allocation (inside `parse_header`).
    let h = match parse_header(bytes) {
        Ok(h) => h,
        Err(e) => {
            proof {
                // No reservation: the log is empty and `MH` failed, so
                // `req(L) == [] == Req_TGA`.
                reveal(pre_alloc);
                reveal(crate::model::mh);
                reveal(decode_eq);
                reveal(crate::alloc_guard::sites::fmt_part);
                reveal(req_tga);
                lemma_at_site_log_prealloc(bytes@, log.view());
                crate::model::lemma_mp_mh_err(bytes@, e);
            }
            return Err(TryError::Format(e));
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

    // R6: allocate exactly `4*N` bytes, bounded by MAX_OUTPUT, through the
    // guard. The failed reservation returns `OutOfMemory`; nothing else is
    // built and no partial output escapes (A-TGA-1 clause 2).
    let mut out = match alloc_output_guarded(n, steps, Tracked(log)) {
        Ok(o) => o,
        Err(o) => {
            proof {
                reveal(req_tga);
                reveal(crate::model::mh);
                reveal(crate::alloc_guard::sites::fmt_part);
                assert(crate::model::mh(bytes@) == Ok(m));
                lemma_at_site_log_oom(bytes@, m, log.view());
                assert(crate::alloc_guard::req_of_log(log.view()) == req_tga(bytes@));
                assert(log.view().len() == 1 && !log.view()[0].2);
                assert((o.site_spec(), o.elements_spec() as nat) == req_tga(bytes@)[0]);
            }
            return Err(TryError::OutOfMemory(o));
        },
    };
    proof {
        reveal(req_tga);
        lemma_at_site_log_ok(bytes@, m, log.view());
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
        return Err(TryError::Format(Error::TruncatedImageId));
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
        return Err(TryError::Format(Error::TruncatedColorMap));
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
                    return Err(TryError::Format(Error::TrailingBytes));
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
                    return Err(TryError::Format(Error::TrailingBytes));
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
            Err(TryError::Format(e))
        },
    };
    r
}

/// The logged entry point: creates the one log with `new_log`, threads it
/// through `decode_tracked`, and returns it with the work counter. A-TGA-1's
/// `G`/`Prog`, the `PreAlloc` corollary, the A-TGA-5 charge bound and the
/// failure naming are its postconditions. (Frozen A-TGA-4 is termination; the
/// charge bound is the added A-TGA-5 — see alloc-delta/ERRATA.md.)
#[doc(hidden)]
pub fn try_decode_with_steps_logged(bytes: &[u8])
    -> (r: (Result<Decoded, TryError>, Ghost<nat>, Tracked<AllocLog>))
    ensures
        r.1@ as int <= 16 * (bytes@.len() + model_checked_n(bytes@)),
        g(bytes@, r.0, r.2@.view()),
        prog(bytes@, r.0, r.2@.view()),
        crate::alloc_guard::sites::log_tied(r.0, r.2@.view()),
        pre_alloc(bytes@) ==> !matches!(r.0, Err(TryError::OutOfMemory(_)))
            && decode_eq(crate::alloc_guard::sites::fmt_part(r.0), bytes@) && r.2@.view().len() == 0,
        match r.0 {
            Err(TryError::OutOfMemory(o)) =>
                (o.site_spec(), o.elements_spec() as nat) == req_tga(bytes@)[0],
            _ => true,
        },
{
    let tracked mut log = crate::alloc_guard::new_log();
    let mut steps = Ghost(0nat);
    let result = decode_tracked(bytes, &mut steps, Tracked(&mut log));
    proof {
        reveal(g);
        reveal(g_clause1);
        reveal(g_clause2);
        reveal(g_clause3);
        reveal(prog);
        reveal(prog_concl);
        reveal(crate::alloc_guard::sites::log_tied);
        reveal(pre_alloc);
        reveal(req_tga);
        reveal(crate::alloc_guard::sites::fmt_part);
        reveal(at_site_log);
        reveal(crate::alloc_guard::req_of_log);
        reveal(crate::alloc_guard::all_ok);
        // A-TGA-5: the charged work never exceeds `16*(|b|+N)`.
        assert(steps@ as int <= 16 * (bytes@.len() + model_checked_n(bytes@))) by {
            reveal(model_checked_n);
            reveal(crate::model::mh);
        }
        lemma_req_tga_len(bytes@);
        lemma_req_tga_clause3(bytes@);
    }
    (result, steps, Tracked(log))
}

/// The public 1.0 decoder: `try_decode_with_steps_logged` with the counter and
/// the log discarded. Its result relation is `decode_eq` on `crate::alloc_guard::sites::fmt_part(r)` for
/// every non-`OutOfMemory` result, and `OutOfMemory` names the failed
/// `(Output, 4N)` reservation.
pub fn try_decode(bytes: &[u8]) -> (r: Result<Decoded, TryError>)
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
        !matches!(r, Err(TryError::OutOfMemory(_))) ==> decode_eq(crate::alloc_guard::sites::fmt_part(r), bytes@),
        match r {
            Err(TryError::OutOfMemory(o)) => o.site_spec() == AllocSite::Output
                && o.elements_spec() as int == 4 * model_checked_n(bytes@),
            _ => true,
        },
{
    let r = try_decode_with_steps_logged(bytes);
    proof {
        reveal(crate::alloc_guard::sites::fmt_part);
        reveal(req_tga);
        reveal(model_checked_n);
        reveal(crate::model::mh);
        reveal(crate::alloc_guard::req_of_log);
        reveal(crate::alloc_guard::all_ok);
        if let Err(TryError::OutOfMemory(o)) = r.0 {
            assert((o.site_spec(), o.elements_spec() as nat) == req_tga(bytes@)[0]);
            assert(req_tga(bytes@)[0].1 == 4 * model_checked_n(bytes@) as nat);
            assert(o.site_spec() == AllocSite::Output);
        }
    }
    r.0
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
/// input, `try_decode(b) = Err(e) iff MP(b) = Err(e)` for each R15 error, with the
/// header check order and the RLE control/overrun/full-packet/index order
/// explicit; `decode` returns neither pixels nor a second error. This is the
/// `Err` half of `decode_eq`, stated as a witness over `try_decode` itself.
#[allow(dead_code)]
fn t9_precedence(bytes: &[u8]) -> (r: Result<Decoded, TryError>)
    ensures
        !matches!(r, Err(TryError::OutOfMemory(_))) ==> decode_eq(crate::alloc_guard::sites::fmt_part(r), bytes@),
{
    let r = try_decode(bytes);
    proof {
        reveal(crate::alloc_guard::sites::fmt_part);
        reveal(decode_eq);
    }
    r
}

/// T7/T8/T11 · total functional correctness, success and resource bounds, and
/// the integrated call path, at the manifest's quantification: for every byte
/// slice representable on either target, `try_decode(bytes)` equals `MP(bytes)` as
/// the exact `Result` — all output bytes, both flips, alpha, map offsets, footer
/// behavior and first error — and on success `1 <= w,h <= 8192`,
/// `|p| = 4*w*h <= 67,108,864`, and every output cell `q < N` satisfies
/// `t8_cell` (the per-pixel clause, proved alongside the public result, not left
/// as an orphan witness).
#[allow(dead_code)]
fn t7_decode_exact(bytes: &[u8]) -> (r: Result<Decoded, TryError>)
    ensures
        !matches!(r, Err(TryError::OutOfMemory(_))) ==> decode_eq(crate::alloc_guard::sites::fmt_part(r), bytes@),
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
        // T8's "iff": `MP` accepts exactly when the decoder does, with the same triple.
        match r {
            Ok(d) => crate::model::mp(bytes@) == Ok((d.width as int, d.height as int, d.rgba@)),
            Err(e) => match e {
                TryError::Format(e) => crate::model::mp(bytes@) == Err(e),
                TryError::OutOfMemory(_) => true,
            },
        },
{
    let r = try_decode(bytes);
    proof {
        reveal(crate::alloc_guard::sites::fmt_part);
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
fn t1_decode_safe(bytes: &[u8]) -> (r: Result<Decoded, TryError>)
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
    try_decode(bytes)
}

// ---------------------------------------------------------------------------
// Piece B: the profiled decoder (POLICY-SPEC.md / THEOREMS-POLICY-DELTA.md).
//
// Everything below is additive: the 1.0 `decode`/`parse_header` and every
// helper they call are unchanged, so the 1.0 obligations keep their proofs and
// `decode_with_policy` is the only new public surface.
// ---------------------------------------------------------------------------

// --- B1: the exec guard wrappers -------------------------------------------

/// R1 `image_type` under `p`: the executable test of `type_ok_policy`.
fn type_ok_hdr_exec(t: u8, p: Policy) -> (r: bool)
    ensures
        r == crate::policy_model::type_ok_policy(t, p),
{
    let core = t == 1u8 || t == 2u8 || t == 3u8 || t == 9u8 || t == 10u8 || t == 11u8;
    let fallback = match p.image_type {
        ImageTypeOptions::GreyFallback => true,
        _ => false,
    };
    let r = core || fallback;
    proof {
        reveal(crate::model::type_ok);
        reveal(crate::policy_model::type_ok_policy);
        assert(core == crate::model::type_ok(t));
        assert(r == crate::policy_model::type_ok_policy(t, p));
    }
    r
}

/// R1: the executable image kind of a type byte (0 indexed, 1 true-colour,
/// 2 greyscale). Equals the model's `kind_policy`.
fn kind_hdr_exec(t: u8, p: Policy) -> (r: u8)
    ensures
        r as int == crate::policy_model::kind_policy(t, p),
{
    let core = t == 1u8 || t == 2u8 || t == 3u8 || t == 9u8 || t == 10u8 || t == 11u8;
    let r = if core {
        if t == 1u8 || t == 9u8 {
            0u8
        } else if t == 2u8 || t == 10u8 {
            1u8
        } else {
            2u8
        }
    } else {
        2u8
    };
    proof {
        reveal(crate::model::type_ok);
        reveal(crate::model::is_indexed);
        reveal(crate::policy_model::kind_policy);
        reveal(crate::policy_model::kind_of_type);
        assert(core == crate::model::type_ok(t));
        if core {
            assert(crate::policy_model::kind_of_type(t) == r as int);
            assert(crate::policy_model::kind_policy(t, p) == r as int);
        } else {
            assert(crate::policy_model::kind_policy(t, p) == 2);
            assert(r == 2u8);
        }
    }
    r
}

/// R2 `map_header`: the executable flag test of `map_flag_ok_policy_flags`.
fn flag_ok_hdr_exec(flag: u8, p: Policy) -> (r: bool)
    ensures
        r == crate::policy_model::map_flag_ok_policy_flags(flag, p),
{
    let r = match p.map_header {
        MapHeaderOptions::Core | MapHeaderOptions::IgnoreAbsent => flag <= 1u8,
        MapHeaderOptions::Gdk | MapHeaderOptions::Image => true,
    };
    proof {
        reveal(crate::policy_model::map_flag_ok_policy_flags);
        assert(r == crate::policy_model::map_flag_ok_policy_flags(flag, p));
    }
    r
}

/// R2 `map_header`: the executable "is a map present" test.
fn flag_present_hdr_exec(flag: u8, p: Policy) -> (r: bool)
    ensures
        r == crate::policy_model::map_present_policy_flags(flag, p),
{
    let r = match p.map_header {
        MapHeaderOptions::Core | MapHeaderOptions::IgnoreAbsent => flag == 1u8,
        MapHeaderOptions::Gdk => flag != 0u8,
        MapHeaderOptions::Image => flag == 1u8,
    };
    proof {
        reveal(crate::policy_model::map_present_policy_flags);
        assert(r == crate::policy_model::map_present_policy_flags(flag, p));
    }
    r
}

/// R3 `depths`: the executable depth test under `o`.
fn depth_ok_hdr_exec(kind: u8, d: u8, o: DepthsOptions) -> (r: bool)
    ensures
        r == crate::policy_model::depth_ok_policy(kind as int, d, o),
{
    let r = match o {
        DepthsOptions::Core | DepthsOptions::Image => {
            if kind == 0u8 {
                d == 8u8 || d == 16u8
            } else if kind == 1u8 {
                d == 15u8 || d == 16u8 || d == 24u8 || d == 32u8
            } else {
                d == 8u8 || d == 16u8
            }
        },
        DepthsOptions::Gdk => {
            if kind == 0u8 {
                d == 8u8
            } else if kind == 1u8 {
                d == 16u8 || d == 24u8 || d == 32u8
            } else {
                d == 8u8 || d == 16u8
            }
        },
    };
    proof {
        reveal(crate::policy_model::depth_ok_policy);
        assert(r == crate::policy_model::depth_ok_policy(kind as int, d, o));
    }
    r
}

/// R5: the CORE attribute-count set, executably.
fn core_attr_ok_exec(kind: u8, d: u8, count: u8) -> (r: bool)
    ensures
        r == crate::policy_model::core_attr_allowed(kind as int, d as int, count as int),
{
    let r = if kind == 2u8 {
        if d == 8u8 {
            count == 0u8
        } else if d == 16u8 {
            count == 0u8 || count == 8u8
        } else {
            count == 0u8
        }
    } else {
        if d == 15u8 || d == 24u8 {
            count == 0u8
        } else if d == 16u8 {
            count == 0u8 || count == 1u8
        } else if d == 32u8 {
            count == 0u8 || count == 8u8
        } else {
            count == 0u8
        }
    };
    proof {
        reveal(crate::policy_model::core_attr_allowed);
        assert(r == crate::policy_model::core_attr_allowed(kind as int, d as int, count as int));
    }
    r
}

/// R5: the ImageTable attribute-count set, executably.
fn image_attr_ok_exec(kind: u8, d: u8, count: u8) -> (r: bool)
    ensures
        r == crate::policy_model::image_attr_allowed(kind as int, d as int, count as int),
{
    let r = if kind == 2u8 && d == 8u8 {
        count == 0u8 || count == 8u8
    } else if kind == 2u8 && d == 16u8 {
        count == 8u8
    } else {
        core_attr_ok_exec(kind, d, count)
    };
    proof {
        reveal(crate::policy_model::image_attr_allowed);
        assert(r == crate::policy_model::image_attr_allowed(kind as int, d as int, count as int));
    }
    r
}

/// R5: the option's acceptance test, executably.
fn attr_ok_hdr_exec(kind: u8, d: u8, count: u8, o: AttributeBitsOptions) -> (r: bool)
    ensures
        r == crate::policy_model::attr_ok_policy(kind as int, d as int, count as int, o),
{
    let r = match o {
        AttributeBitsOptions::Lenient => true,
        AttributeBitsOptions::Exact => core_attr_ok_exec(kind, d, count),
        AttributeBitsOptions::ImageTable => image_attr_ok_exec(kind, d, count),
    };
    proof {
        reveal(crate::policy_model::attr_ok_policy);
        assert(r == crate::policy_model::attr_ok_policy(kind as int, d as int, count as int, o));
    }
    r
}

/// R5: the effective attribute count after the acceptance test, executably.
fn eff_attr_exec(kind: u8, d: u8, count: u8, o: AttributeBitsOptions) -> (r: u8)
    ensures
        r as int == crate::policy_model::eff_attr(kind as int, d as int, count as int, o),
{
    let table = match o {
        AttributeBitsOptions::ImageTable => image_attr_ok_exec(kind, d, count),
        _ => core_attr_ok_exec(kind, d, count),
    };
    let table8 = match o {
        AttributeBitsOptions::ImageTable => image_attr_ok_exec(kind, d, 8u8),
        _ => core_attr_ok_exec(kind, d, 8u8),
    };
    let r = if table {
        count
    } else if table8 {
        8u8
    } else {
        0u8
    };
    proof {
        reveal(crate::policy_model::eff_attr);
        reveal(crate::policy_model::attr_table_allowed);
        assert(table == crate::policy_model::attr_table_allowed(
            kind as int,
            d as int,
            count as int,
            o,
        ));
        assert(table8 == crate::policy_model::attr_table_allowed(kind as int, d as int, 8, o));
        assert(r as int == crate::policy_model::eff_attr(kind as int, d as int, count as int, o));
    }
    r
}

/// R4: the coherence/entry-depth failure, executably.
fn coherence_bad_hdr_exec(
    first: u32,
    length: u32,
    edepth: u8,
    present: bool,
    kind: u8,
    o: MapHeaderOptions,
) -> (r: bool)
    ensures
        r == crate::policy_model::map_coherence_bad_flags(
            first as int,
            length as int,
            edepth,
            present,
            kind as int,
            o,
        ),
{
    let md_ok = edepth == 15u8 || edepth == 16u8 || edepth == 24u8 || edepth == 32u8;
    let r = match o {
        MapHeaderOptions::Core => {
            if !present {
                first != 0u32 || length != 0u32 || edepth != 0u8 || kind == 0u8
            } else {
                length == 0u32 || !md_ok
            }
        },
        MapHeaderOptions::IgnoreAbsent => {
            if !present {
                kind == 0u8
            } else {
                length == 0u32 || !md_ok
            }
        },
        MapHeaderOptions::Gdk => {
            if !present {
                kind == 0u8
            } else {
                length != 0u32 && !md_ok
            }
        },
        MapHeaderOptions::Image => {
            if !present {
                kind == 0u8
            } else {
                !md_ok
            }
        },
    };
    proof {
        reveal(crate::policy_model::map_coherence_bad_flags);
        reveal(crate::model::map_entry_depth_ok);
        assert(md_ok == crate::model::map_entry_depth_ok(edepth));
        assert(r == crate::policy_model::map_coherence_bad_flags(
            first as int,
            length as int,
            edepth,
            present,
            kind as int,
            o,
        ));
    }
    r
}

/// R4: the map range failure, executably.
fn range_bad_hdr_exec(first: u32, length: u32, present: bool, o: MapHeaderOptions) -> (r: bool)
    ensures
        r == crate::policy_model::map_range_bad_flags(first as int, length as int, present, o),
{
    let r = match o {
        MapHeaderOptions::Core | MapHeaderOptions::IgnoreAbsent => {
            present && first as u64 + length as u64 > 65536u64
        },
        MapHeaderOptions::Gdk | MapHeaderOptions::Image => false,
    };
    proof {
        reveal(crate::policy_model::map_range_bad_flags);
        assert(r == crate::policy_model::map_range_bad_flags(
            first as int,
            length as int,
            present,
            o,
        ));
    }
    r
}

/// R3 `depths = Image`: the index-depth-over-entry-depth failure, executably.
fn index_depth_bad_hdr_exec(
    present: bool,
    kind: u8,
    depth: u8,
    edepth: u8,
    o: DepthsOptions,
) -> (r: bool)
    ensures
        r == crate::policy_model::image_index_depth_bad_flags(
            present,
            kind as int,
            depth,
            edepth,
            o,
        ),
{
    let r = match o {
        DepthsOptions::Image => present && kind == 0u8 && depth > edepth,
        _ => false,
    };
    proof {
        reveal(crate::policy_model::image_index_depth_bad_flags);
        assert(r == crate::policy_model::image_index_depth_bad_flags(
            present,
            kind as int,
            depth,
            edepth,
            o,
        ));
    }
    r
}

// --- B1: the verdict and the parser -----------------------------------------

/// The `Result<(), Error>` projection of a header model result: `Ok` for any
/// accepted record, the error otherwise.
pub(crate) open spec fn of_mh(r: Result<MHModel, Error>) -> Result<(), Error> {
    match r {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

/// `mh_policy_of` on a parsed header's own fields.
pub(crate) open spec fn mh_policy_of_hdr(h: Header, p: Policy) -> Result<MHModel, Error> {
    crate::policy_model::mh_policy_of(
        h.id_length,
        h.color_map_flag,
        h.image_type,
        h.map_first as int,
        h.map_length as int,
        h.map_entry_depth,
        h.x_origin as int,
        h.y_origin as int,
        h.width as int,
        h.height as int,
        h.pixel_depth,
        h.descriptor,
        p,
    )
}

/// The verdict `header_verdict` decides: the error of `mh_policy_of` on the
/// header's fields, or `Ok(())` when it accepts.
pub(crate) open spec fn verdict_of_header(h: Header, p: Policy) -> Result<(), Error> {
    of_mh(mh_policy_of_hdr(h, p))
}

proof fn lemma_of_mh_ok(r: Result<MHModel, Error>)
    requires
        of_mh(r) == Ok(()),
    ensures
        r is Ok,
{
    reveal(of_mh);
    match r {
        Ok(_) => {},
        Err(_) => {},
    }
}

proof fn lemma_of_mh_err(r: Result<MHModel, Error>, e: Error)
    requires
        of_mh(r) == Err(e),
    ensures
        r == Err(e),
{
    reveal(of_mh);
    match r {
        Ok(_) => {},
        Err(e2) => {
            assert(e2 == e);
        },
    }
}

proof fn lemma_u32_roundtrip(x: u32)
    ensures
        x as int as u32 == x,
{
}

proof fn lemma_u8_roundtrip(x: u8)
    ensures
        x as int as u8 == x,
{
}

/// `mh_of_header(h,p)`: the model record a parsed policy header reconstructs —
/// the header's own raw values with the policy `map_bytes` formula. T5 for the
/// parser is `mh_policy(bytes@,p) == Ok(mh_of_header(h,p))`.
pub(crate) open spec fn mh_of_header(h: Header, p: Policy) -> MHModel {
    MHModel {
        id_length: h.id_length,
        color_map_flag: h.color_map_flag,
        image_type: h.image_type,
        map_first: h.map_first,
        map_length: h.map_length,
        map_entry_depth: h.map_entry_depth,
        x_origin: h.x_origin,
        y_origin: h.y_origin,
        width: h.width,
        height: h.height,
        pixel_depth: h.pixel_depth,
        descriptor: h.descriptor,
        n: h.width as int * (h.height as int),
        sample_bytes: crate::model::sample_bytes_of(h.pixel_depth),
        map_bytes: crate::policy_model::map_bytes_flags(
            h.color_map_flag,
            h.map_length as int,
            h.map_entry_depth,
            p,
        ),
    }
}

/// The parser's invariant on an accepted policy header. Every clause holds for
/// all four `map_header` and `depths` values; `lemma_mh_policy_of_ok_facts`
/// supplies them from the accepted verdict.
pub(crate) open spec fn header_inv_policy(h: Header, p: Policy) -> bool {
    &&& crate::policy_model::type_ok_policy(h.image_type, p)
    &&& crate::policy_model::map_flag_ok_policy_flags(h.color_map_flag, p)
    &&& 1 <= (h.width as int) <= 8192
    &&& 1 <= (h.height as int) <= 8192
    &&& (h.descriptor & 0xc0u8 == 0u8 || p.interleave == InterleaveOptions::Ignore)
    &&& crate::policy_model::depth_ok_policy(
        crate::policy_model::kind_policy(h.image_type, p),
        h.pixel_depth,
        p.depths,
    )
    &&& 1 <= h.spec_sample_bytes() <= 4
    &&& 4 * (h.width as int * (h.height as int)) <= 67108864
    &&& (crate::policy_model::kind_policy(h.image_type, p) == 0
        ==> crate::policy_model::map_present_policy_flags(h.color_map_flag, p))
    &&& (crate::policy_model::map_present_policy_flags(h.color_map_flag, p) && (h.map_length
        as int) > 0 ==> crate::model::map_entry_depth_ok(h.map_entry_depth))
    &&& (h.map_length as int) <= 65535
    &&& crate::policy_model::map_bytes_flags(
        h.color_map_flag,
        h.map_length as int,
        h.map_entry_depth,
        p,
    ) <= 262140
}

/// R1..R7 under `p`: validate a parsed header's fields in the model's R14
/// order, returning the model's exact first error. Every guard is an exec
/// wrapper proved equal to the matching `*_flags` predicate, so the result
/// equals `verdict_of_header`.
fn header_verdict(h: &Header, p: Policy) -> (r: Result<(), Error>)
    ensures
        r == verdict_of_header(*h, p),
{
    let t_ok = type_ok_hdr_exec(h.image_type, p);
    if !t_ok {
        proof {
            reveal(crate::policy_model::mh_policy_of);
            assert(!crate::policy_model::type_ok_policy(h.image_type, p));
            reveal(of_mh);
            assert(verdict_of_header(*h, p) == Err(Error::UnsupportedImageType));
        }
        return Err(Error::UnsupportedImageType);
    }
    let kind = kind_hdr_exec(h.image_type, p);
    let f_ok = flag_ok_hdr_exec(h.color_map_flag, p);
    if !f_ok {
        proof {
            reveal(crate::policy_model::mh_policy_of);
            reveal(of_mh);
            assert(verdict_of_header(*h, p) == Err(Error::InvalidColorMapFlag));
        }
        return Err(Error::InvalidColorMapFlag);
    }
    let present = flag_present_hdr_exec(h.color_map_flag, p);
    let dims_ok = h.width >= 1u32 && h.width <= MAX_DIMENSION && h.height >= 1u32
        && h.height <= MAX_DIMENSION;
    if !dims_ok {
        proof {
            reveal(crate::policy_model::mh_policy_of);
            reveal(crate::model::max_dimension);
            reveal(of_mh);
            assert(!(1 <= h.width as int && h.width as int <= crate::model::max_dimension())
                || !(1 <= h.height as int && h.height as int <= crate::model::max_dimension()));
            assert(verdict_of_header(*h, p) == Err(Error::InvalidDimensions));
        }
        return Err(Error::InvalidDimensions);
    }
    let il_ok = match p.interleave {
        InterleaveOptions::Reject => h.descriptor & 0xc0u8 == 0u8,
        InterleaveOptions::Ignore => true,
    };
    if !il_ok {
        proof {
            reveal(crate::policy_model::mh_policy_of);
            reveal(crate::model::max_dimension);
            reveal(of_mh);
            assert(h.descriptor & 0xc0u8 != 0u8);
            assert(p.interleave == InterleaveOptions::Reject);
            assert(verdict_of_header(*h, p) == Err(Error::UnsupportedInterleave));
        }
        return Err(Error::UnsupportedInterleave);
    }
    let d_ok = depth_ok_hdr_exec(kind, h.pixel_depth, p.depths);
    if !d_ok {
        proof {
            reveal(crate::policy_model::mh_policy_of);
            reveal(crate::model::max_dimension);
            reveal(of_mh);
            assert(kind as int == crate::policy_model::kind_policy(h.image_type, p));
            assert(verdict_of_header(*h, p) == Err(Error::UnsupportedPixelDepth));
        }
        return Err(Error::UnsupportedPixelDepth);
    }
    let coh_bad = coherence_bad_hdr_exec(
        h.map_first,
        h.map_length,
        h.map_entry_depth,
        present,
        kind,
        p.map_header,
    );
    if coh_bad {
        proof {
            reveal(crate::policy_model::mh_policy_of);
            reveal(crate::model::max_dimension);
            reveal(of_mh);
            assert(kind as int == crate::policy_model::kind_policy(h.image_type, p));
            assert(present == crate::policy_model::map_present_policy_flags(
                h.color_map_flag,
                p,
            ));
            assert(verdict_of_header(*h, p) == Err(Error::InvalidColorMap));
        }
        return Err(Error::InvalidColorMap);
    }
    let rng_bad = range_bad_hdr_exec(h.map_first, h.map_length, present, p.map_header);
    if rng_bad {
        proof {
            reveal(crate::policy_model::mh_policy_of);
            reveal(crate::model::max_dimension);
            reveal(of_mh);
            assert(kind as int == crate::policy_model::kind_policy(h.image_type, p));
            assert(!crate::policy_model::map_coherence_bad_flags(
                h.map_first as int,
                h.map_length as int,
                h.map_entry_depth,
                present,
                kind as int,
                p.map_header,
            ));
            assert(verdict_of_header(*h, p) == Err(Error::ColorMapRange));
        }
        return Err(Error::ColorMapRange);
    }
    let idx_bad = index_depth_bad_hdr_exec(present, kind, h.pixel_depth, h.map_entry_depth, p.depths);
    if idx_bad {
        proof {
            reveal(crate::policy_model::mh_policy_of);
            reveal(crate::model::max_dimension);
            reveal(of_mh);
            assert(kind as int == crate::policy_model::kind_policy(h.image_type, p));
            assert(!crate::policy_model::map_range_bad_flags(
                h.map_first as int,
                h.map_length as int,
                present,
                p.map_header,
            ));
            assert(verdict_of_header(*h, p) == Err(Error::UnsupportedPixelDepth));
        }
        return Err(Error::UnsupportedPixelDepth);
    }
    let adepth = if kind == 0u8 {
        h.map_entry_depth
    } else {
        h.pixel_depth
    };
    let attr = h.attribute_count();
    let a_ok = attr_ok_hdr_exec(kind, adepth, attr, p.attribute_bits);
    if !a_ok {
        proof {
            reveal(crate::policy_model::mh_policy_of);
            reveal(crate::model::max_dimension);
            reveal(of_mh);
            assert(kind as int == crate::policy_model::kind_policy(h.image_type, p));
            assert(adepth as int == (if kind as int == 0 {
                h.map_entry_depth as int
            } else {
                h.pixel_depth as int
            }));
            assert(attr as int == (h.descriptor & 0x0fu8) as int);
            assert(verdict_of_header(*h, p) == Err(Error::InvalidAttributeBits));
        }
        return Err(Error::InvalidAttributeBits);
    }
    proof {
        assert((h.width as u64) * (h.height as u64) <= 67108864) by (nonlinear_arith)
            requires
                h.width <= 8192u32,
                h.height <= 8192u32,
        ;
    }
    let n64 = h.width as u64 * h.height as u64;
    proof {
        assert(4 * n64 <= u64::MAX) by (nonlinear_arith)
            requires
                n64 <= 67108864,
        ;
    }
    if 4 * n64 > MAX_OUTPUT as u64 {
        proof {
            reveal(crate::policy_model::mh_policy_of);
            reveal(crate::model::max_dimension);
            reveal(crate::model::max_output);
            reveal(of_mh);
            assert(attr as int == (h.descriptor & 0x0fu8) as int);
            assert(!(4 * (h.width as int * (h.height as int)) <= crate::model::max_output()));
            assert(verdict_of_header(*h, p) == Err(Error::OutputTooLarge));
        }
        return Err(Error::OutputTooLarge);
    }
    proof {
        reveal(crate::policy_model::mh_policy_of);
        reveal(crate::model::max_dimension);
        reveal(crate::model::max_output);
        reveal(of_mh);
        // The guards above passed, so `mh_policy_of` takes its accepting branch
        // and returns its own `Ok` literal; the literal is `mh_of_header`.
        assert(mh_policy_of_hdr(*h, p) == Ok(mh_of_header(*h, p))) by {
            reveal(mh_of_header);
            assert(h.map_first as int as u32 == h.map_first) by {
                lemma_u32_roundtrip(h.map_first);
            }
            assert(h.map_length as int as u32 == h.map_length) by {
                lemma_u32_roundtrip(h.map_length);
            }
            assert(h.x_origin as int as u32 == h.x_origin) by {
                lemma_u32_roundtrip(h.x_origin);
            }
            assert(h.y_origin as int as u32 == h.y_origin) by {
                lemma_u32_roundtrip(h.y_origin);
            }
            assert(h.width as int as u32 == h.width) by {
                lemma_u32_roundtrip(h.width);
            }
            assert(h.height as int as u32 == h.height) by {
                lemma_u32_roundtrip(h.height);
            }
        }
        assert(verdict_of_header(*h, p) == Ok(()));
    }
    Ok(())
}

/// Either `mh_of_header` or the model's own error, from the accepted verdict.
proof fn lemma_mh_of_header_of_verdict(h: &Header, p: Policy)
    requires
        verdict_of_header(*h, p) == Ok(()),
    ensures
        mh_policy_of_hdr(*h, p) == Ok(mh_of_header(*h, p)),
{
    reveal(verdict_of_header);
    reveal(of_mh);
    assert(mh_policy_of_hdr(*h, p) is Ok);
    reveal(crate::policy_model::mh_policy_of);
    reveal(mh_of_header);
    assert(mh_policy_of_hdr(*h, p) == Ok(mh_of_header(*h, p))) by {
        assert(h.map_first as int as u32 == h.map_first) by {
            lemma_u32_roundtrip(h.map_first);
        }
        assert(h.map_length as int as u32 == h.map_length) by {
            lemma_u32_roundtrip(h.map_length);
        }
        assert(h.x_origin as int as u32 == h.x_origin) by {
            lemma_u32_roundtrip(h.x_origin);
        }
        assert(h.y_origin as int as u32 == h.y_origin) by {
            lemma_u32_roundtrip(h.y_origin);
        }
        assert(h.width as int as u32 == h.width) by {
            lemma_u32_roundtrip(h.width);
        }
        assert(h.height as int as u32 == h.height) by {
            lemma_u32_roundtrip(h.height);
        }
    }
}

/// The facts `header_inv_policy` collects, from the accepted verdict.
proof fn lemma_mh_of_header_inv(b: Seq<u8>, h: &Header, p: Policy)
    requires
        crate::policy_model::mh_policy(b, p) == Ok(mh_of_header(*h, p)),
    ensures
        header_inv_policy(*h, p),
{
    crate::policy_model::lemma_mh_policy_ok_bounds(b, mh_of_header(*h, p), p);
    assert(mh_of_header(*h, p).map_bytes <= 262140);
    reveal(crate::policy_model::mh_policy_of);
    reveal(crate::model::max_dimension);
    reveal(crate::model::max_output);
    reveal(crate::model::map_entry_depth_ok);
    reveal(crate::model::sample_bytes_of);
    reveal(crate::policy_model::map_bytes_flags);
    reveal(crate::policy_model::map_present_policy_flags);
    reveal(header_inv_policy);
    reveal(crate::policy_model::depth_ok_policy);
    reveal(crate::policy_model::kind_policy);
    reveal(crate::policy_model::kind_of_type);
    reveal(crate::model::is_indexed);
    reveal(crate::model::type_ok);
    assert(crate::policy_model::type_ok_policy(h.image_type, p));
    assert(crate::policy_model::map_flag_ok_policy_flags(h.color_map_flag, p));
    assert(1 <= h.width as int <= 8192);
    assert(1 <= h.height as int <= 8192);
    assert(h.descriptor & 0xc0u8 == 0u8 || p.interleave == InterleaveOptions::Ignore);
    assert(crate::policy_model::depth_ok_policy(
        crate::policy_model::kind_policy(h.image_type, p),
        h.pixel_depth,
        p.depths,
    ));
    assert(!crate::policy_model::map_coherence_bad_flags(
        h.map_first as int,
        h.map_length as int,
        h.map_entry_depth,
        crate::policy_model::map_present_policy_flags(h.color_map_flag, p),
        crate::policy_model::kind_policy(h.image_type, p),
        p.map_header,
    ));
    assert(!crate::policy_model::map_range_bad_flags(
        h.map_first as int,
        h.map_length as int,
        crate::policy_model::map_present_policy_flags(h.color_map_flag, p),
        p.map_header,
    ));
    assert(!crate::policy_model::image_index_depth_bad_flags(
        crate::policy_model::map_present_policy_flags(h.color_map_flag, p),
        crate::policy_model::kind_policy(h.image_type, p),
        h.pixel_depth,
        h.map_entry_depth,
        p.depths,
    ));
    assert(4 * (h.width as int * (h.height as int)) <= 67108864);
    // the indexed-image needs a present map
    if crate::policy_model::kind_policy(h.image_type, p) == 0 {
        assert(crate::policy_model::map_present_policy_flags(h.color_map_flag, p));
    }
    // the sample width
    if crate::policy_model::kind_policy(h.image_type, p) == 0 {
        assert(h.pixel_depth == 8u8 || h.pixel_depth == 16u8);
    } else if crate::policy_model::kind_policy(h.image_type, p) == 1 {
        assert(h.pixel_depth == 15u8 || h.pixel_depth == 16u8 || h.pixel_depth == 24u8
            || h.pixel_depth == 32u8);
    } else {
        assert(h.pixel_depth == 8u8 || h.pixel_depth == 16u8);
    }
    crate::model::lemma_sample_bytes_le_4(h.pixel_depth);
    assert(1 <= h.spec_sample_bytes() <= 4);
    // the map-entry depth and the map-slice bound
    if crate::policy_model::map_present_policy_flags(h.color_map_flag, p)
        && (h.map_length as int) > 0 {
        assert(crate::model::map_entry_depth_ok(h.map_entry_depth));
        crate::model::lemma_sample_bytes_le_4(h.map_entry_depth);
        assert(crate::model::sample_bytes_of(h.map_entry_depth) <= 4);
    }
    // the map-slice bound: from `mh_of_header`'s own `map_bytes` equation and
    // the model's `lemma_mh_policy_ok_bounds` on the accepted record.
    assert(crate::policy_model::map_bytes_flags(
        h.color_map_flag,
        h.map_length as int,
        h.map_entry_depth,
        p,
    ) <= 262140);
    assert(header_inv_policy(*h, p));
}

/// R1..R7 under `p`: the 18-byte header prefix, validated by `header_verdict`.
/// T5: on `Ok`, `mh_policy(bytes@,p)` is exactly `Ok(mh_of_header(h,p))`; on
/// `Err`, it is exactly that error. The 1.0 `parse_header` is untouched.
fn parse_header_policy(b: &[u8], p: Policy) -> (r: Result<Header, Error>)
    ensures
        match crate::policy_model::mh_policy(b@, p) {
            Ok(m) => match r {
                Ok(h) => m == mh_of_header(h, p),
                Err(_) => false,
            },
            Err(e2) => match r {
                Ok(_) => false,
                Err(e) => e == e2,
            },
        },
        match r {
            Ok(h) => 18 <= b@.len() <= crate::model::max_input() && header_inv_policy(h, p),
            Err(_) => true,
        },
{
    if b.len() > MAX_INPUT {
        proof {
            reveal(crate::policy_model::mh_policy);
            reveal(crate::model::max_input);
        }
        return Err(Error::FileTooLarge);
    }
    proof {
        reveal(crate::policy_model::mh_policy);
        reveal(crate::model::max_input);
        assert(b@.len() <= crate::model::max_input());
    }
    if b.len() < 18 {
        proof {
            reveal(crate::policy_model::mh_policy);
            reveal(crate::model::max_input);
        }
        return Err(Error::TruncatedHeader);
    }
    proof {
        reveal(crate::policy_model::mh_policy);
        assert(b@.len() >= 18);
    }
    let h = Header {
        id_length: b[0],
        color_map_flag: b[1],
        image_type: b[2],
        map_first: le16(b, 3),
        map_length: le16(b, 5),
        map_entry_depth: b[7],
        x_origin: le16(b, 8),
        y_origin: le16(b, 10),
        width: le16(b, 12),
        height: le16(b, 14),
        pixel_depth: b[16],
        descriptor: b[17],
    };
    let v = header_verdict(&h, p);
    proof {
        // T5: tie the executable fields to the input slice, then rewrite
        // `mh_policy` through the proved `lemma_mh_policy_eq_of_header`.
        crate::policy_model::lemma_mh_policy_eq_of_header(
            b@,
            b@[0],
            b@[1],
            b@[2],
            crate::model::le16(b@, 3),
            crate::model::le16(b@, 5),
            b@[7],
            crate::model::le16(b@, 8),
            crate::model::le16(b@, 10),
            crate::model::le16(b@, 12),
            crate::model::le16(b@, 14),
            b@[16],
            b@[17],
            p,
        );
        reveal(crate::model::le16);
        assert(h.id_length == b@[0]);
        assert(h.color_map_flag == b@[1]);
        assert(h.image_type == b@[2]);
        assert(h.map_entry_depth == b@[7]);
        assert(h.pixel_depth == b@[16]);
        assert(h.descriptor == b@[17]);
        assert(h.map_first as int == crate::model::le16(b@, 3));
        assert(h.map_length as int == crate::model::le16(b@, 5));
        assert(h.x_origin as int == crate::model::le16(b@, 8));
        assert(h.y_origin as int == crate::model::le16(b@, 10));
        assert(h.width as int == crate::model::le16(b@, 12));
        assert(h.height as int == crate::model::le16(b@, 14));
        assert(crate::policy_model::mh_policy(b@, p) == mh_policy_of_hdr(h, p));
        assert(v == verdict_of_header(h, p));
    }
    match v {
        Ok(()) => {
            proof {
                assert(mh_policy_of_hdr(h, p) == Ok(mh_of_header(h, p)));
                assert(crate::policy_model::mh_policy(b@, p) == Ok(mh_of_header(h, p)));
                lemma_mh_of_header_inv(b@, &h, p);
                assert(18 <= b@.len() <= crate::model::max_input());
            }
            Ok(h)
        },
        Err(e) => {
            proof {
                lemma_of_mh_err(mh_policy_of_hdr(h, p), e);
                assert(mh_policy_of_hdr(h, p) == Err(e));
                assert(crate::policy_model::mh_policy(b@, p) == Err(e));
            }
            Err(e)
        },
    }
}

/// The conversion-relevant projection of a *policy* accepted header: the
/// kind under `p`, the **effective** attribute count, and the surviving map
/// fields. `pkt_of_policy(h,p) == policy_model::pkt_of_mh_policy(mh_of_header(h,p), p)`.
pub(crate) open spec fn pkt_of_policy(h: &Header, p: Policy) -> crate::model::PktH {
    crate::model::PktH {
        kind: crate::policy_model::kind_policy(h.image_type, p),
        depth: h.pixel_depth as int,
        attr: crate::policy_model::eff_count_img(
            h.image_type,
            h.pixel_depth,
            h.map_entry_depth,
            (h.descriptor & 0x0fu8) as int,
            p,
        ),
        map_first: h.map_first as int,
        map_len: h.map_length as int,
        map_depth: h.map_entry_depth as int,
        n: h.width as int * (h.height as int),
        sample_bytes: h.spec_sample_bytes(),
    }
}

/// `pkt_of_policy` equals the model's policy projection on the accepted record.
proof fn lemma_pkt_of_policy_eq(h: &Header, p: Policy, m: MHModel)
    requires
        m == mh_of_header(*h, p),
    ensures
        pkt_of_policy(h, p) == crate::policy_model::pkt_of_mh_policy(m, p),
{
    reveal(pkt_of_policy);
    reveal(mh_of_header);
    reveal(crate::policy_model::pkt_of_mh_policy);
    reveal(crate::policy_model::kind_policy);
    reveal(crate::policy_model::eff_count_img);
    reveal(crate::policy_model::eff_attr);
    reveal(crate::policy_model::attr_table_allowed);
    reveal(crate::model::sample_bytes_of);
    reveal(crate::model::stride_of);
    crate::model::lemma_stride_eq(h.pixel_depth);
}


// --- B2: the policy conversion layer ---------------------------------------

/// R5: the executable alpha rule on the effective attribute count.
fn stored_alpha_exec(attr: u8, p: Policy) -> (r: bool)
    ensures
        r == crate::policy_model::stored_alpha_img_attr(attr as int, p),
{
    let r = match p.alpha {
        AlphaOptions::Stored => true,
        AlphaOptions::AttributeBits => attr == 8u8,
    };
    proof {
        reveal(crate::policy_model::stored_alpha_img_attr);
    }
    r
}

/// R8 `rgb5`: one five-bit channel under the policy rule, as a byte.
fn rgb5_chan_exec(c: u16, rule: Rgb5Options) -> (r: u8)
    requires
        c < 32,
    ensures
        r == crate::policy_model::rgb5_chan(c as int, rule),
{
    match rule {
        Rgb5Options::Replicate => {
            proof {
                crate::model::lemma_rep5(c as u32);
                reveal(crate::policy_model::rgb5_chan);
                assert(0 <= crate::model::rep5(c as int) < 256);
                assert(((c << 3) | (c >> 2)) as int == crate::model::rep5(c as int)) by (bit_vector)
                    requires
                        c < 32,
                ;
            }
            ((c << 3) | (c >> 2)) as u8
        },
        Rgb5Options::Round => {
            proof {
                reveal(crate::policy_model::rgb5_chan);
                reveal(crate::policy_model::round5);
                assert(0 <= (255 * (c as int) + 15) / 31 < 256) by (nonlinear_arith)
                    requires
                        c < 32,
                ;
            }
            ((255u32 * (c as u32) + 15) / 31) as u8
        },
    }
}

/// R8 `map5`: one five-bit channel under the policy rule, as a byte.
fn map5_chan_exec(c: u16, rule: Map5Options) -> (r: u8)
    requires
        c < 32,
    ensures
        r == crate::policy_model::map5_chan(c as int, rule),
{
    match rule {
        Map5Options::Replicate => {
            proof {
                crate::model::lemma_rep5(c as u32);
                reveal(crate::policy_model::map5_chan);
                assert(0 <= crate::model::rep5(c as int) < 256);
                assert(((c << 3) | (c >> 2)) as int == crate::model::rep5(c as int)) by (bit_vector)
                    requires
                        c < 32,
                ;
            }
            ((c << 3) | (c >> 2)) as u8
        },
        Map5Options::Round => {
            proof {
                reveal(crate::policy_model::map5_chan);
                reveal(crate::policy_model::round5);
                assert(0 <= (255 * (c as int) + 15) / 31 < 256) by (nonlinear_arith)
                    requires
                        c < 32,
                ;
            }
            ((255u32 * (c as u32) + 15) / 31) as u8
        },
        Map5Options::GdkSwapShift => {
            proof {
                reveal(crate::policy_model::map5_chan);
                assert(0 <= (c as int) * 8 < 256) by (nonlinear_arith)
                    requires
                        c < 32,
                ;
                assert(((c as u32) * 8) as int == (c as int) * 8) by (nonlinear_arith)
                    requires
                        c < 32,
                ;
            }
            ((c as u32) * 8) as u8
        },
    }
}

/// R8: a 15/16-bit true-colour word's three channels plus `alpha`.
fn conv555_policy(v: u16, rule: Rgb5Options, alpha: u8) -> (r: [u8; 4])
    ensures
        r@.len() == 4,
        r@[3] == alpha,
        r@ =~= crate::policy_model::rgb555_policy(v as int, rule).push(alpha),
{
    proof {
        reveal(crate::policy_model::rgb555_policy);
        crate::model::lemma_rgb_fields(v as u32);
        crate::policy_model::lemma_rgb555_policy_chan(v as int, rule);
    }
    let rr = [
        rgb5_chan_exec((v >> 10) & 31, rule),
        rgb5_chan_exec((v >> 5) & 31, rule),
        rgb5_chan_exec(v & 31, rule),
        alpha,
    ];
    proof {
        assert(rr@[0] == crate::policy_model::rgb5_chan(crate::model::red5(v as int), rule));
        assert(rr@[1] == crate::policy_model::rgb5_chan(crate::model::green5(v as int), rule));
        assert(rr@[2] == crate::policy_model::rgb5_chan(crate::model::blue5(v as int), rule));
        assert(rr@ =~= crate::policy_model::rgb555_policy(v as int, rule).push(alpha)) by {
            assert(rr@.len() == 4);
            assert(rr@[3] == alpha);
        }
    }
    rr
}

/// R8: a 15/16-bit palette word's three channels under `map5`.
fn map555_exec(v: u16, rule: Map5Options) -> (r: [u8; 3])
    ensures
        r@.len() == 3,
        r@ =~= crate::policy_model::map555_policy(v as int, rule),
{
    proof {
        reveal(crate::policy_model::map555_policy);
        crate::model::lemma_rgb_fields(v as u32);
        crate::policy_model::lemma_map555_policy_chan(v as int, rule);
    }
    let f0 = match rule {
        Map5Options::GdkSwapShift => v & 31,
        _ => (v >> 10) & 31,
    };
    let f2 = match rule {
        Map5Options::GdkSwapShift => (v >> 10) & 31,
        _ => v & 31,
    };
    let rr = [
        map5_chan_exec(f0, rule),
        map5_chan_exec((v >> 5) & 31, rule),
        map5_chan_exec(f2, rule),
    ];
    proof {
        assert(rr@[0] == crate::policy_model::map5_chan(
            crate::policy_model::map5_field0(v as int, rule),
            rule,
        ));
        assert(rr@[1] == crate::policy_model::map5_chan(crate::model::green5(v as int), rule));
        assert(rr@[2] == crate::policy_model::map5_chan(
            crate::policy_model::map5_field2(v as int, rule),
            rule,
        ));
        assert(rr@ =~= crate::policy_model::map555_policy(v as int, rule)) by {
            assert(rr@.len() == 3);
        }
    }
    rr
}

/// R8 `map16_alpha`: the 16-bit palette entry's alpha under the policy. The
/// rule bit is taken by `match` (so the exec condition is a plain `bool`) and
/// its equality with the spec comparison is asserted for the case lemmas.
fn map_entry_alpha_exec(edepth: u8, attr: u8, v: u16, p: Policy) -> (r: u8)
    ensures
        r == crate::policy_model::map_entry_alpha_exec(edepth, attr as int, v as int, 0, p),
{
    let rule15 = match p.map16_alpha {
        Map16AlphaOptions::Bit15 => true,
        _ => false,
    };
    proof {
        reveal(crate::model::bit15);
        assert(rule15 == (p.map16_alpha == Map16AlphaOptions::Bit15));
    }
    if edepth == 16u8 {
        if attr == 1u8 {
            if rule15 {
                let set15 = (v as u32) & 0x8000u32 != 0u32;
                if set15 {
                    proof {
                        crate::model::lemma_bit15(v as u32);
                        assert(crate::model::bit15(v as int)) by {
                            assert(set15 == crate::model::bit15(v as int));
                        }
                        crate::policy_model::lemma_alpha_bit15_set(edepth, attr, v, p);
                    }
                    255u8
                } else {
                    proof {
                        crate::model::lemma_bit15(v as u32);
                        assert(!crate::model::bit15(v as int)) by {
                            assert(set15 == crate::model::bit15(v as int));
                        }
                        crate::policy_model::lemma_alpha_bit15_clear(edepth, attr, v, p);
                    }
                    0u8
                }
            } else {
                proof {
                    assert(p.map16_alpha != Map16AlphaOptions::Bit15);
                    crate::policy_model::lemma_alpha_not_bit15rule(edepth, attr, v, p);
                }
                255u8
            }
        } else {
            proof {
                crate::policy_model::lemma_alpha_attr_not1(edepth, attr, v, p);
            }
            255u8
        }
    } else {
        proof {
            crate::policy_model::lemma_alpha_depth_not16(edepth, attr, v, p);
        }
        255u8
    }
}

/// R8 `index_range = Black`: the substitute of an out-of-range index.
fn black_substitute_exec(map_depth: u8) -> (r: [u8; 4])
    ensures
        r@ =~= crate::policy_model::black_substitute(map_depth as int),
{
    let r = if map_depth == 32u8 {
        [0u8, 0u8, 0u8, 0u8]
    } else {
        [0u8, 0u8, 0u8, 255u8]
    };
    proof {
        reveal(crate::policy_model::black_substitute);
    }
    r
}

/// R8: convert one non-indexed sample under `p` into RGBA. The record's `attr`
/// is the effective count, so the alpha rule below is `stored_alpha_policy` on
/// the record, and the depth table is the one the header's kind selects.
fn convert_sample_policy(h: &Header, s: &[u8], p: Policy) -> (r: [u8; 4])
    requires
        s@.len() == h.spec_sample_bytes(),
        crate::policy_model::type_ok_policy(h.image_type, p),
        crate::policy_model::kind_policy(h.image_type, p) != 0,
        crate::policy_model::depth_ok_policy(
            crate::policy_model::kind_policy(h.image_type, p),
            h.pixel_depth,
            p.depths,
        ),
        1 <= h.spec_sample_bytes() <= 4,
        h.spec_sample_bytes() == crate::model::stride_of(h.pixel_depth as int),
    ensures
        r@.len() == 4,
        r@ == crate::policy_model::conv_sample_policy(pkt_of_policy(h, p), s@, p),
{
    let kind = kind_hdr_exec(h.image_type, p);
    let raw_count = h.attribute_count();
    let attr = eff_attr_exec(kind, h.pixel_depth, raw_count, p.attribute_bits);
    proof {
        reveal(crate::policy_model::conv_sample_policy);
        reveal(pkt_of_policy);
        reveal(crate::policy_model::eff_count_img);
        reveal(crate::policy_model::eff_attr);
        reveal(crate::policy_model::attr_table_allowed);
        reveal(crate::policy_model::stored_alpha_policy);
        reveal(crate::policy_model::stored_alpha_img_attr);
        reveal(crate::policy_model::gov_depth);
        reveal(crate::model::sample_bytes_of);
        reveal(crate::model::stride_of);
        crate::policy_model::lemma_depth_ok_pixel(
            crate::policy_model::kind_policy(h.image_type, p),
            h.pixel_depth,
            p.depths,
        );
        assert(kind as int == crate::policy_model::kind_policy(h.image_type, p));
        assert(pkt_of_policy(h, p).kind == kind as int);
        assert(pkt_of_policy(h, p).depth == h.pixel_depth as int);
        assert(pkt_of_policy(h, p).sample_bytes == h.spec_sample_bytes());
        assert(raw_count == h.descriptor & 0x0fu8);
        assert(raw_count as int == (h.descriptor & 0x0fu8) as int) by {
            assert(raw_count == h.descriptor & 0x0fu8);
        }
        assert(pkt_of_policy(h, p).attr == attr as int);
    }
    let r: [u8; 4] = if kind == 1u8 {
        if h.pixel_depth == 15 || h.pixel_depth == 16 {
            proof {
                assert(crate::policy_model::depth_ok_policy(1, h.pixel_depth, p.depths));
                assert(h.spec_sample_bytes() == 2);
                assert(pkt_of_policy(h, p).kind == 1);
                assert(pkt_of_policy(h, p).depth == 15 || pkt_of_policy(h, p).depth == 16);
            }
            let b0 = s[0];
            let b1 = s[1];
            let v = b0 as u16 | (b1 as u16) << 8;
            proof {
                reveal(crate::model::le16s);
                assert(v as int == (b0 as int) + 256 * (b1 as int)) by (bit_vector)
                    requires
                        v == (b0 as u16) | ((b1 as u16) << 8),
                ;
                assert(crate::model::le16s(s@, 0) == (b0 as int) + 256 * (b1 as int));
            }
            conv555_policy(v, p.rgb5, 255)
        } else if h.pixel_depth == 24 {
            proof {
                assert(crate::policy_model::depth_ok_policy(1, h.pixel_depth, p.depths));
                assert(h.spec_sample_bytes() == 3);
                assert(pkt_of_policy(h, p).kind == 1);
                assert(pkt_of_policy(h, p).depth == 24);
            }
            let rr = [s[2], s[1], s[0], 255];
            proof {
                assert(rr@ =~= seq![s@[2], s@[1], s@[0], 255u8]) by {
                    assert(rr@.len() == 4);
                }
            }
            rr
        } else {
            proof {
                assert(crate::policy_model::depth_ok_policy(1, h.pixel_depth, p.depths));
                assert(h.pixel_depth == 32u8);
                assert(h.spec_sample_bytes() == 4);
                assert(pkt_of_policy(h, p).kind == 1);
                assert(pkt_of_policy(h, p).depth == 32);
            }
            let take = stored_alpha_exec(attr, p);
            let alpha = if take {
                s[3]
            } else {
                255u8
            };
            let rr = [s[2], s[1], s[0], alpha];
            proof {
                if take {
                    assert(alpha == s@[3]);
                } else {
                    assert(alpha == 255u8);
                }
                assert(rr@ =~= seq![
                    s@[2],
                    s@[1],
                    s@[0],
                    (if crate::policy_model::stored_alpha_policy(pkt_of_policy(h, p), p) {
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
    } else {
        if h.pixel_depth == 8 {
            proof {
                assert(crate::policy_model::depth_ok_policy(2, h.pixel_depth, p.depths));
                assert(h.spec_sample_bytes() == 1);
                assert(pkt_of_policy(h, p).kind == 2);
                assert(pkt_of_policy(h, p).depth == 8);
            }
            [s[0], s[0], s[0], 255]
        } else {
            proof {
                assert(h.pixel_depth != 8u8);
                reveal(crate::policy_model::depth_ok_policy);
                assert(crate::policy_model::depth_ok_policy(2, h.pixel_depth, p.depths));
                assert(h.pixel_depth == 16u8);
                assert(h.spec_sample_bytes() == 2);
                assert(pkt_of_policy(h, p).kind == 2);
                assert(pkt_of_policy(h, p).depth == 16);
            }
            let take = stored_alpha_exec(attr, p);
            let alpha = if take {
                s[1]
            } else {
                255u8
            };
            let rr = [s[0], s[0], s[0], alpha];
            proof {
                if take {
                    assert(alpha == s@[1]);
                } else {
                    assert(alpha == 255u8);
                }
                assert(rr@ =~= seq![
                    s@[0],
                    s@[0],
                    s@[0],
                    (if crate::policy_model::stored_alpha_policy(pkt_of_policy(h, p), p) {
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
    };
    r
}

/// R4/R8 `map_index`: the entry number `j` a raw index selects, executably.
fn entry_j_exec(h: &Header, index: u32, p: Policy) -> (r: u32)
    requires
        crate::policy_model::entry_j(index as int, h.map_first as int, p) >= 0,
    ensures
        r as int == crate::policy_model::entry_j(index as int, h.map_first as int, p),
{
    let r = match p.map_index {
        MapIndexOptions::Absolute => index - h.map_first,
        MapIndexOptions::ZeroBased => index,
    };
    proof {
        reveal(crate::policy_model::entry_j);
        if p.map_index == MapIndexOptions::Absolute {
            assert((index as int) - h.map_first as int >= 0);
            assert((index - h.map_first) as int == (index as int) - h.map_first as int);
        }
    }
    r
}

/// R8/R4: the executable policy index-range test.
fn index_in_range_policy(h: &Header, index: u32, p: Policy) -> (r: bool)
    ensures
        r == crate::policy_model::in_range_policy(pkt_of_policy(h, p), index as int, p),
{
    let r = match p.map_index {
        MapIndexOptions::Absolute => {
            index >= h.map_first && (index - h.map_first) < h.map_length
        },
        MapIndexOptions::ZeroBased => index < h.map_length,
    };
    proof {
        reveal(crate::policy_model::in_range_policy);
        reveal(crate::policy_model::entry_j);
        reveal(pkt_of_policy);
        assert(pkt_of_policy(h, p).map_first == h.map_first as int);
        assert(pkt_of_policy(h, p).map_len == h.map_length as int);
        if p.map_index == MapIndexOptions::Absolute {
            assert(crate::policy_model::entry_j(index as int, h.map_first as int, p) == (index
                as int) - h.map_first as int);
            if index >= h.map_first {
                assert((index - h.map_first) as int == (index as int) - h.map_first as int);
            }
        } else {
            assert(crate::policy_model::entry_j(index as int, h.map_first as int, p) == index
                as int);
        }
    }
    r
}

/// R4/R8: convert map entry `index` under `p`. The caller has validated the
/// index, so it lies in `0 <= j < length` and the `Black` substitute is not
/// reachable here.
fn convert_index_policy(h: &Header, map: &[u8], index: u32, p: Policy) -> (r: [u8; 4])
    requires
        crate::policy_model::type_ok_policy(h.image_type, p),
        crate::policy_model::kind_policy(h.image_type, p) == 0,
        crate::policy_model::map_present_policy_flags(h.color_map_flag, p),
        crate::policy_model::in_range_policy(pkt_of_policy(h, p), index as int, p),
        map@.len() == crate::policy_model::map_bytes_flags(
            h.color_map_flag,
            h.map_length as int,
            h.map_entry_depth,
            p,
        ),
        1 <= h.spec_map_stride() <= 4,
        h.map_length as int <= 65535,
        crate::model::map_entry_depth_ok(h.map_entry_depth),
    ensures
        r@.len() == 4,
        r@ == crate::policy_model::conv_map_policy_ok(pkt_of_policy(h, p), map@, index as int, p),
{
    let stride = h.map_bytes_per_entry();
    proof {
        reveal(crate::policy_model::in_range_policy);
        reveal(crate::policy_model::entry_j);
        reveal(pkt_of_policy);
        reveal(crate::policy_model::map_bytes_flags);
        reveal(crate::policy_model::map_present_policy_flags);
        reveal(crate::policy_model::conv_map_policy_ok);
        reveal(crate::model::sample_bytes_of);
        reveal(crate::model::stride_of);
        crate::model::lemma_stride_eq(h.map_entry_depth);
        assert(crate::policy_model::kind_policy(h.image_type, p) == 0);
        assert(pkt_of_policy(h, p).kind == 0);
        assert(pkt_of_policy(h, p).map_depth == h.map_entry_depth as int);
        assert(pkt_of_policy(h, p).map_len == h.map_length as int);
        assert(0 <= crate::policy_model::entry_j(index as int, h.map_first as int, p));
        assert(crate::policy_model::entry_j(index as int, h.map_first as int, p) < h.map_length as int);
        assert(h.map_length as int > 0);
        assert(crate::policy_model::map_bytes_flags(
            h.color_map_flag,
            h.map_length as int,
            h.map_entry_depth,
            p,
        ) == h.map_length as int * crate::model::sample_bytes_of(h.map_entry_depth));
        assert(map@.len() == h.map_length as int * h.spec_map_stride());
        assert(1 <= h.spec_map_stride() <= 4);
        reveal(crate::model::map_entry_depth_ok);
        assert(h.map_entry_depth == 15u8 || h.map_entry_depth == 16u8 || h.map_entry_depth == 24u8
            || h.map_entry_depth == 32u8);
    }
    let j = entry_j_exec(h, index, p);
    proof {
        assert(j as int == crate::policy_model::entry_j(index as int, h.map_first as int, p));
        assert((j as int) < h.map_length as int);
        assert((j as int) * (stride as int) + stride <= usize::MAX) by (nonlinear_arith)
            requires
                (j as int) < h.map_length as int,
                h.map_length as int <= 65535,
                stride as int == h.spec_map_stride(),
                1 <= h.spec_map_stride() <= 4,
        ;
        assert((j as int) * (stride as int) + stride <= map@.len()) by (nonlinear_arith)
            requires
                (j as int) < h.map_length as int,
                stride as int == h.spec_map_stride(),
                map@.len() == h.map_length as int * h.spec_map_stride(),
                h.spec_map_stride() >= 1,
        ;
    }
    let start = j as usize * stride;
    proof {
        assert(start as int == (j as int) * h.spec_map_stride());
        assert((start as int) + (stride as int) <= map@.len());
    }
    let s = &map[start..start + stride];
    proof {
        assert(s@ == map@.subrange(start as int, start as int + stride as int));
        assert(s@.len() == stride as int);
    }
    let raw_count = h.attribute_count();
    let attr = eff_attr_exec(kind_hdr_exec(h.image_type, p), h.map_entry_depth, raw_count, p.attribute_bits);
    proof {
        reveal(crate::policy_model::eff_count_img);
        reveal(crate::policy_model::eff_attr);
        reveal(crate::policy_model::stored_alpha_policy);
        reveal(crate::policy_model::stored_alpha_img_attr);
        reveal(crate::policy_model::stored_alpha_policy_attr);
        assert(raw_count as int == (h.descriptor & 0x0fu8) as int) by {
            assert(raw_count == h.descriptor & 0x0fu8);
        }
        assert(pkt_of_policy(h, p).attr == attr as int);
        crate::policy_model::lemma_stored_alpha_policy_attr(pkt_of_policy(h, p), p);
    }
    let r: [u8; 4] = match h.map_entry_depth {
        15 | 16 => {
            proof {
                assert(h.spec_map_stride() == 2);
                assert(pkt_of_policy(h, p).map_depth == 15 || pkt_of_policy(h, p).map_depth == 16);
                assert(pkt_of_policy(h, p).attr == attr as int);
                crate::policy_model::lemma_map555_policy_eq(map@, start as int, p.map5);
                crate::policy_model::lemma_map_entry_alpha_policy_eq(
                    pkt_of_policy(h, p),
                    map@,
                    start as int,
                    h.map_entry_depth,
                    p,
                );
            }
            let b0 = s[0];
            let b1 = s[1];
            let v = b0 as u16 | (b1 as u16) << 8;
            proof {
                reveal(crate::model::le16s);
                assert(v as int == (b0 as int) + 256 * (b1 as int)) by (bit_vector)
                    requires
                        v == (b0 as u16) | ((b1 as u16) << 8),
                ;
                assert(crate::model::le16s(map@, start as int) == (b0 as int) + 256 * (b1 as int));
                assert(crate::model::le16s(s@, 0) == crate::model::le16s(map@, start as int));
            }
            let three = map555_exec(v, p.map5);
            let alpha = map_entry_alpha_exec(h.map_entry_depth, attr, v, p);
            proof {
                reveal(crate::policy_model::conv_map_policy_ok);
                reveal(crate::policy_model::map_entry_alpha_policy);
                assert(three@ =~= crate::policy_model::map555_policy(v as int, p.map5));
                assert(alpha == crate::policy_model::map_entry_alpha_policy(
                    pkt_of_policy(h, p),
                    v as int,
                    h.map_entry_depth as int,
                    p,
                ));
            }
            let rr = [three[0], three[1], three[2], alpha];
            proof {
                assert(rr@ =~= three@.push(alpha)) by {
                    assert(rr@.len() == 4);
                    assert(rr@[3] == alpha);
                }
                assert(crate::policy_model::map555_policy(v as int, p.map5).push(alpha)
                    =~= crate::policy_model::conv_map_policy_ok(
                    pkt_of_policy(h, p),
                    map@,
                    index as int,
                    p,
                ));
            }
            rr
        },
        24 => {
            proof {
                assert(h.spec_map_stride() == 3);
                assert(pkt_of_policy(h, p).map_depth == 24);
            }
            let rr = [s[2], s[1], s[0], 255];
            proof {
                reveal(crate::policy_model::conv_map_policy_ok);
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
        },
        _ => {
            proof {
                reveal(crate::model::map_entry_depth_ok);
                assert(h.map_entry_depth == 32u8);
                assert(h.spec_map_stride() == 4);
                assert(pkt_of_policy(h, p).map_depth == 32);
                assert(pkt_of_policy(h, p).attr == attr as int);
            }
            let take = stored_alpha_exec(attr, p);
            let alpha = if take {
                s[3]
            } else {
                255u8
            };
            let rr = [s[2], s[1], s[0], alpha];
            proof {
                reveal(crate::policy_model::conv_map_policy_ok);
                if take {
                    assert(alpha == map@[start as int + 3]);
                } else {
                    assert(alpha == 255u8);
                }
                assert(rr@ =~= seq![
                    map@[start as int + 2],
                    map@[start as int + 1],
                    map@[start as int],
                    (if crate::policy_model::stored_alpha_policy(pkt_of_policy(h, p), p) {
                        map@[start as int + 3]
                    } else {
                        255u8
                    }),
                ]) by {
                    assert(rr@.len() == 4);
                }
            }
            rr
        },
    };
    r
}

/// R8/R11 under `p`: convert one validated sample, checking the policy index
/// range for indexed samples before any map read and substituting black under
/// `index_range = Black`.
fn convert_one_policy(h: &Header, map: &[u8], indexed: bool, s: &[u8], p: Policy, subst: &mut bool)
    -> (r: Result<[u8; 4], Error>)
    requires
        crate::policy_model::type_ok_policy(h.image_type, p),
        indexed == (crate::policy_model::kind_policy(h.image_type, p) == 0),
        crate::policy_model::depth_ok_policy(
            crate::policy_model::kind_policy(h.image_type, p),
            h.pixel_depth,
            p.depths,
        ),
        indexed ==> crate::policy_model::map_present_policy_flags(h.color_map_flag, p),
        indexed ==> (h.map_length as int) > 0 ==> crate::model::map_entry_depth_ok(
            h.map_entry_depth,
        ),
        indexed ==> (h.spec_sample_bytes() == 1 || h.spec_sample_bytes() == 2),
        !indexed ==> 1 <= h.spec_sample_bytes() <= 4,
        !indexed ==> h.spec_sample_bytes() == crate::model::stride_of(h.pixel_depth as int),
        s@.len() == h.spec_sample_bytes(),
        s@.len() >= 1,
        map@.len() == crate::policy_model::map_bytes_flags(
            h.color_map_flag,
            h.map_length as int,
            h.map_entry_depth,
            p,
        ),
        h.map_length as int <= 65535,
    ensures
        *final(subst) == (*old(subst) || (indexed && crate::policy_model::pkt_subst(
            pkt_of_policy(h, p), crate::model::sindex(s@, h.spec_sample_bytes()), p))),
        match r {
            Ok(px) => px@.len() == 4,
            Err(_) => true,
        },
        match r {
            Ok(px) => px@ == crate::policy_model::conv2_policy(
                pkt_of_policy(h, p),
                map@,
                s@,
                p,
            ),
            Err(_) => true,
        },
        match r {
            Ok(_) => !indexed || crate::policy_model::in_range_policy(
                pkt_of_policy(h, p),
                crate::model::sindex(s@, h.spec_sample_bytes()),
                p,
            ) || p.index_range == IndexRangeOptions::Black,
            Err(e) => indexed && !crate::policy_model::in_range_policy(
                pkt_of_policy(h, p),
                crate::model::sindex(s@, h.spec_sample_bytes()),
                p,
            ) && p.index_range == IndexRangeOptions::Reject && e == Error::ColorIndexOutOfRange,
        },
{
    if indexed {
        proof {
            reveal(pkt_of_policy);
            reveal(crate::policy_model::conv2_policy);
            reveal(crate::policy_model::in_range_policy);
            reveal(crate::policy_model::entry_j);
            reveal(crate::model::stride_of);
            reveal(crate::model::sample_bytes_of);
            assert(crate::policy_model::kind_policy(h.image_type, p) == 0);
            assert(pkt_of_policy(h, p).kind == 0);
            assert(pkt_of_policy(h, p).sample_bytes == h.spec_sample_bytes());
        }
        let idx = sample_index(s);
        proof {
            assert(idx as int == crate::model::sindex(s@, h.spec_sample_bytes()));
        }
        let in_range = index_in_range_policy(h, idx, p);
        let black = match p.index_range {
            IndexRangeOptions::Black => true,
            IndexRangeOptions::Reject => false,
        };
        if !in_range && !black {
            proof {
                assert(!crate::policy_model::in_range_policy(
                    pkt_of_policy(h, p),
                    crate::model::sindex(s@, h.spec_sample_bytes()),
                    p,
                ));
            }
            Err(Error::ColorIndexOutOfRange)
        } else if !in_range {
            proof {
                reveal(crate::policy_model::conv2_policy);
                reveal(crate::policy_model::conv_map_policy);
                reveal(crate::policy_model::entry_j);
                reveal(crate::policy_model::black_substitute);
                assert(!crate::policy_model::in_range_policy(
                    pkt_of_policy(h, p),
                    crate::model::sindex(s@, h.spec_sample_bytes()),
                    p,
                ));
                assert(p.index_range == IndexRangeOptions::Black);
                assert(crate::policy_model::conv_map_policy(
                    pkt_of_policy(h, p),
                    map@,
                    crate::model::sindex(s@, h.spec_sample_bytes()),
                    p,
                ) == crate::policy_model::black_substitute(pkt_of_policy(h, p).map_depth));
            }
            *subst = true;
            Ok(black_substitute_exec(h.map_entry_depth))
        } else {
            proof {
                assert(crate::policy_model::in_range_policy(
                    pkt_of_policy(h, p),
                    crate::model::sindex(s@, h.spec_sample_bytes()),
                    p,
                ));
                reveal(pkt_of_policy);
                crate::policy_model::lemma_in_range_policy_pos(
                    pkt_of_policy(h, p),
                    crate::model::sindex(s@, h.spec_sample_bytes()),
                    p,
                );
                assert(h.map_length as int > 0);
                reveal(crate::model::map_entry_depth_ok);
                assert((h.map_length as int) > 0);
                assert(crate::model::map_entry_depth_ok(h.map_entry_depth));
                crate::policy_model::lemma_map_entry_depth_stride(h.map_entry_depth);
                assert(1 <= h.spec_map_stride() <= 4) by {
                    crate::policy_model::lemma_sample_bytes_of_stride(h.map_entry_depth);
                }
                crate::policy_model::lemma_conv_map_policy_in_range(
                    pkt_of_policy(h, p),
                    map@,
                    crate::model::sindex(s@, h.spec_sample_bytes()),
                    p,
                );
                assert(crate::policy_model::conv_map_policy(
                    pkt_of_policy(h, p),
                    map@,
                    crate::model::sindex(s@, h.spec_sample_bytes()),
                    p,
                ) == crate::policy_model::conv_map_policy_ok(
                    pkt_of_policy(h, p),
                    map@,
                    crate::model::sindex(s@, h.spec_sample_bytes()),
                    p,
                ));
            }
            Ok(convert_index_policy(h, map, idx, p))
        }
    } else {
        proof {
            reveal(crate::policy_model::conv2_policy);
            assert(crate::policy_model::kind_policy(h.image_type, p) != 0);
        }
        Ok(convert_sample_policy(h, s, p))
    }
}


// --- B3: the policy pixel phase (raw loop) ---------------------------------

/// T7 at the policy raw helper: the executable result equals `MP_policy`'s raw
/// phase as the exact `Result` — on `Ok` the final cursor; on `Err` the model's
/// exact first error. Stated on `mp_raw_cur_policy`, which carries the same
/// ordered checks as `mp_raw_acc_policy`.
spec fn raw_result_eq_policy(
    b: Seq<u8>,
    p: PktH,
    pol: Policy,
    c0: int,
    cur_final: int,
    r: Result<(), Error>,
) -> bool {
    match crate::policy_model::mp_raw_cur_policy(b, p, pol, c0, 0) {
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

/// The policy raw loop: `N` samples converted under `p`, each written at its
/// oriented cell. `c0g` is the function-entry cursor.
fn decode_raw_policy(
    bytes: &[u8],
    h: &Header,
    map: &[u8],
    indexed: bool,
    out: &mut Vec<u8>,
    cursor: &mut usize,
    n: usize,
    c0g: usize,
    p: Policy,
    steps: &mut Ghost<nat>,
    subst: &mut bool,
) -> (r: Result<(), Error>)
    requires
        header_inv_policy(*h, p),
        n == h.width as int * (h.height as int),
        *cursor == c0g,
        indexed == (crate::policy_model::kind_policy(h.image_type, p) == 0),
        indexed ==> crate::policy_model::map_present_policy_flags(h.color_map_flag, p),
        indexed ==> (h.map_length as int) > 0 ==> crate::model::map_entry_depth_ok(
            h.map_entry_depth,
        ),
        old(out)@.len() == 4 * n,
        bytes@.len() >= 18,
        bytes@.len() <= crate::model::max_input(),
        *cursor <= bytes@.len(),
        map@.len() == crate::policy_model::map_bytes_flags(
            h.color_map_flag,
            h.map_length as int,
            h.map_entry_depth,
            p,
        ),
        crate::policy_model::map_bytes_flags(
            h.color_map_flag,
            h.map_length as int,
            h.map_entry_depth,
            p,
        ) <= 262140,
    ensures
        r is Ok ==> *final(subst) == (*old(subst) || crate::policy_model::raw_subst_policy(
            bytes@, pkt_of_policy(h, p), p, c0g as int, n as int)),
        final(steps)@ <= old(steps)@ + 5 * n,
        final(out)@.len() == 4 * n,
        *final(cursor) <= bytes@.len(),
        match r {
            Ok(_) => *final(cursor) == *old(cursor) + n * (h.spec_sample_bytes() as usize),
            Err(_) => true,
        },
        match r {
            Ok(_) => crate::policy_model::wrote_samples_policy(
                final(out)@,
                bytes@,
                pkt_of_policy(h, p),
                map@,
                p,
                h.width as int,
                h.height as int,
                h.spec_flip_x(),
                h.spec_flip_y(),
                *old(cursor) as int,
                h.spec_sample_bytes(),
                n as int,
            ),
            Err(_) => true,
        },
        raw_result_eq_policy(
            bytes@,
            pkt_of_policy(h, p),
            p,
            c0g as int,
            *final(cursor) as int,
            r,
        ),
{
    let ghost start_steps = steps@;
    let sb = h.sample_bytes();
    let width = h.width;
    let height = h.height;
    let (fx, fy) = (h.flip_x(), h.flip_y());
    let ghost pkt = pkt_of_policy(h, p);
    let ghost c0: usize = c0g;
    proof {
        reveal(header_inv_policy);
        crate::policy_model::lemma_sample_bytes_of_stride(h.pixel_depth);
        reveal(pkt_of_policy);
        assert(sb as int == h.spec_sample_bytes());
        assert(sb >= 1);
        assert(pkt.n == n as int);
        assert(pkt.sample_bytes == sb as int);
        assert(pkt.kind == (if indexed {
            0
        } else {
            crate::policy_model::kind_policy(h.image_type, p)
        }));
        crate::policy_model::lemma_wrote_samples_policy_none(
            out@,
            bytes@,
            pkt,
            map@,
            p,
            width as int,
            height as int,
            fx,
            fy,
            c0 as int,
            sb as int,
        );
        crate::policy_model::lemma_raw_prefix_ok_policy_zero(bytes@, pkt, p, c0 as int);
        reveal(crate::policy_model::mp_raw_cur_policy);
    }
    let mut k: usize = 0;
    while k < n
        invariant
            k <= n,
            *subst == (*old(subst) || crate::policy_model::raw_subst_policy(bytes@, pkt, p, c0 as int, k as int)),
            start_steps == old(steps)@,
            steps@ == start_steps + 5 * k,
            n <= 16777216,
            *cursor == c0 + k * sb,
            c0 == c0g,
            crate::policy_model::wrote_samples_policy(
                out@,
                bytes@,
                pkt,
                map@,
                p,
                width as int,
                height as int,
                fx,
                fy,
                c0 as int,
                sb as int,
                k as int,
            ),
            crate::policy_model::raw_prefix_ok_policy(bytes@, pkt, p, c0 as int, k as int),
            crate::policy_model::mp_raw_cur_policy(bytes@, pkt, p, c0 as int, 0)
                == crate::policy_model::mp_raw_cur_policy(bytes@, pkt, p, c0 as int, k as int),
            header_inv_policy(*h, p),
            indexed == (crate::policy_model::kind_policy(h.image_type, p) == 0),
            indexed ==> crate::policy_model::map_present_policy_flags(h.color_map_flag, p),
            out@.len() == 4 * n,
            sb as int == h.spec_sample_bytes(),
            sb >= 1,
            sb <= 4,
            1 <= (width as int) <= 8192,
            1 <= (height as int) <= 8192,
            width as int * (height as int) == n,
            fx == h.spec_flip_x(),
            fy == h.spec_flip_y(),
            *cursor <= bytes@.len(),
            bytes@.len() <= crate::model::max_input(),
            map@.len() == crate::policy_model::map_bytes_flags(
                h.color_map_flag,
                h.map_length as int,
                h.map_entry_depth,
                p,
            ),
            map@.len() <= 262140,
            pkt == pkt_of_policy(h, p),
            pkt.n == n as int,
        decreases n - k,
    {
        proof { *steps = Ghost(steps@ + 1); }
        if bytes.len() - *cursor < sb {
            proof {
                reveal(raw_result_eq_policy);
                assert(c0 as int + k as int * sb as int + sb as int > bytes@.len())
                    by (nonlinear_arith)
                    requires
                        *cursor == c0 + k * sb,
                        bytes.len() - *cursor < sb,
                ;
                crate::policy_model::lemma_mp_raw_cur_policy_trunc(
                    bytes@,
                    pkt,
                    p,
                    c0 as int,
                    k as int,
                );
                assert(crate::policy_model::mp_raw_cur_policy(bytes@, pkt, p, c0 as int, k as int)
                    == Err(Error::TruncatedPixels));
                assert(crate::policy_model::mp_raw_cur_policy(bytes@, pkt, p, c0 as int, 0) == Err(
                    Error::TruncatedPixels,
                ));
                assert(raw_result_eq_policy(
                    bytes@,
                    pkt,
                    p,
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
            assert(s@.len() == sb as int);
            assert(s@.len() == h.spec_sample_bytes());
            assert(s@ == bytes@.subrange(
                c0 as int + k as int * sb as int,
                c0 as int + k as int * sb as int + sb as int,
            ));
            assert(c0 as int + k as int * sb as int + sb as int <= bytes@.len()) by (nonlinear_arith)
                requires
                    *cursor == c0 + k * sb,
                    end == *cursor + sb,
                    end <= bytes@.len(),
            ;
        }
        proof {
            reveal(header_inv_policy);
            reveal(crate::policy_model::gov_depth);
            reveal(crate::policy_model::kind_policy);
        }
        let rgba = match convert_one_policy(h, map, indexed, s, p, subst) {
            Ok(px) => px,
            Err(e) => {
                proof {
                    reveal(raw_result_eq_policy);
                    assert(indexed);
                    assert(pkt.kind == 0);
                    crate::policy_model::lemma_mp_raw_cur_policy_index(
                        bytes@,
                        pkt,
                        p,
                        c0 as int,
                        k as int,
                    );
                    assert(crate::policy_model::mp_raw_cur_policy(bytes@, pkt, p, c0 as int, k as int)
                        == Err(Error::ColorIndexOutOfRange));
                    assert(crate::policy_model::mp_raw_cur_policy(bytes@, pkt, p, c0 as int, 0) == Err(
                        Error::ColorIndexOutOfRange,
                    ));
                    assert(e == Error::ColorIndexOutOfRange);
                    assert(raw_result_eq_policy(
                        bytes@,
                        pkt,
                        p,
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
            assert(width as int * (height as int) == n);
            assert(off + 4 <= 4 * n) by (nonlinear_arith)
                requires
                    (off as int) <= 4 * (n - 1),
                    1 <= n,
                    n <= 16777216,
            ;
        }
        let ghost out_before = out@;
        write_pixel(out, off, &rgba);
        proof { *steps = Ghost(steps@ + 4); }
        proof {
            reveal(crate::policy_model::wrote_samples_policy);
            reveal(crate::model::quad_at);
            assert(rgba@ == crate::policy_model::conv2_policy(pkt, map@, s@, p));
            assert(s@ == bytes@.subrange(
                c0 as int + k as int * sb as int,
                c0 as int + k as int * sb as int + sb as int,
            ));
            // the new write is the `j = k` case
            assert(crate::model::quad_at(
                out@,
                4 * crate::model::fcoord(width as int, height as int, fx, fy, k as int),
                crate::policy_model::conv2_policy(pkt, map@, bytes@.subrange(
                    c0 as int + k as int * sb as int,
                    c0 as int + k as int * sb as int + sb as int,
                ), p),
            )) by {
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
                assert forall|i: int| 0 <= i < 4 implies out@[off as int + i] == #[trigger] rgba@[i] by {}
            }
            crate::policy_model::lemma_write_preserves_policy_quads(
                out_before,
                out@,
                bytes@,
                pkt,
                map@,
                p,
                width as int,
                height as int,
                fx,
                fy,
                c0 as int,
                sb as int,
                k as int,
                off as int,
            );
            crate::policy_model::lemma_wrote_samples_policy_step(
                out@,
                out_before,
                bytes@,
                pkt,
                map@,
                p,
                width as int,
                height as int,
                fx,
                fy,
                c0 as int,
                sb as int,
                k as int,
            );
            // the sample is decodable, so the raw prefix extends
            crate::policy_model::lemma_raw_sample_ok_policy_of_loop(
                bytes@,
                pkt,
                p,
                c0 as int,
                k as int,
                indexed,
            );
            crate::policy_model::lemma_raw_prefix_ok_policy_step(
                bytes@,
                pkt,
                p,
                c0 as int,
                k as int,
            );
            // and the model's cursor recursion is unchanged
            reveal(crate::policy_model::mp_raw_cur_policy);
        }
        proof {
            assert(c0 + (k + 1) * sb == end) by (nonlinear_arith)
                requires
                    c0 + k * sb + sb == end,
            ;
        }
        *cursor = end;
        k = k + 1;
        proof { reveal(crate::policy_model::raw_subst_policy); }
    }
    proof {
        reveal(raw_result_eq_policy);
        assert(k == n);
        assert(crate::policy_model::mp_raw_cur_policy(bytes@, pkt, p, c0 as int, n as int) == Ok(
            c0 as int + n as int * sb as int,
        )) by {
            reveal(crate::policy_model::mp_raw_cur_policy);
        }
        assert(crate::policy_model::mp_raw_cur_policy(bytes@, pkt, p, c0 as int, 0) == Ok(c0 as int
            + n as int * sb as int));
        assert(raw_result_eq_policy(bytes@, pkt, p, c0 as int, *cursor as int, Ok(())));
    }
    Ok(())
}


// --- B3: the policy RLE packet stream ---------------------------------------

/// `pkt_out` under `pol`: the accumulator is the pixels produced since `p0`,
/// written at their oriented cells.
spec fn pkt_out_policy(
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

/// One packet of the policy stream: control presence, the overrun rule, then the
/// repeat or literal helper. The packet's availability and cursor advance follow
/// `rle_overrun`.
fn rle_packet_policy(
    bytes: &[u8],
    h: &Header,
    map: &[u8],
    indexed: bool,
    out: &mut Vec<u8>,
    cursor: &mut usize,
    produced: &mut usize,
    n: usize,
    p0g: usize,
    p: Policy,
    Ghost(acc0): Ghost<Seq<Seq<u8>>>,
    steps: &mut Ghost<nat>,
    clip: &mut bool, subst: &mut bool,
) -> (r: (Result<(), Error>, Ghost<Seq<Seq<u8>>>))
    requires
        header_inv_policy(*h, p),
        n == h.width as int * (h.height as int),
        indexed == (crate::policy_model::kind_policy(h.image_type, p) == 0),
        indexed ==> crate::policy_model::map_present_policy_flags(h.color_map_flag, p),
        old(out)@.len() == 4 * n,
        *old(cursor) <= bytes@.len(),
        p0g <= *old(produced) < n,
        bytes@.len() <= crate::model::max_input(),
        map@.len() == crate::policy_model::map_bytes_flags(
            h.color_map_flag,
            h.map_length as int,
            h.map_entry_depth,
            p,
        ),
        map@.len() <= 262140,
        pkt_out_policy(
            old(out)@,
            acc0,
            h.width as int,
            h.height as int,
            h.spec_flip_x(),
            h.spec_flip_y(),
            p0g as int,
            *old(produced) as int,
        ),
    ensures
        r.0 is Ok ==> crate::policy_model::event_step_policy(bytes@, pkt_of_policy(h, p), map@, p,
            *old(cursor) as int, *old(produced) as int, *final(cursor) as int, *final(produced) as int,
            *old(clip), *old(subst), *final(clip), *final(subst)),
        *old(produced) <= *final(produced),
        final(steps)@ as int <= old(steps)@ as int + 5 * (*final(produced) as int - *old(produced) as int) + 2,
        final(out)@.len() == 4 * n,
        *final(cursor) <= bytes@.len(),
        *final(produced) <= n,
        pkt_out_policy(
            final(out)@,
            r.1@,
            h.width as int,
            h.height as int,
            h.spec_flip_x(),
            h.spec_flip_y(),
            p0g as int,
            *final(produced) as int,
        ),
        crate::policy_model::pkt_post_policy(
            bytes@,
            pkt_of_policy(h, p),
            map@,
            p,
            *old(cursor) as int,
            *old(produced) as int,
            *final(cursor) as int,
            *final(produced) as int,
            acc0,
            r.1@,
            r.0,
        ),
{
    proof { *steps = Ghost(steps@ + 1); }
    let ghost pkt = pkt_of_policy(h, p);
    proof {
        reveal(header_inv_policy);
        reveal(pkt_of_policy);
        crate::policy_model::lemma_sample_bytes_of_stride(h.pixel_depth);
        assert(pkt.n == n as int);
        assert(pkt.sample_bytes == h.spec_sample_bytes());
    }
    // Control presence.
    if *cursor >= bytes.len() {
        proof {
            crate::policy_model::lemma_pkt_policy_ctrl_absent(
                bytes@,
                pkt,
                map@,
                p,
                *cursor as int,
                *produced as int,
                acc0,
            );
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
    // Overrun under `Reject`.
    let reject_overrun = match p.rle_overrun {
        crate::policy::RleOverrunOptions::Reject => count > n - *produced,
        _ => false,
    };
    proof {
        assert(reject_overrun == (count as int > n as int - *produced as int && p.rle_overrun
            == crate::policy::RleOverrunOptions::Reject));
    }
    if reject_overrun {
        proof {
            crate::policy_model::lemma_pkt_policy_overrun(
                bytes@,
                pkt,
                map@,
                p,
                cur0 as int,
                *produced as int,
                acc0,
            );
        }
        return (Err(Error::RleOverrun), Ghost(acc0));
    }
    if control & 0x80 != 0 {
        rle_repeat_packet_policy(
            bytes,
            h,
            map,
            indexed,
            out,
            cursor,
            produced,
            n,
            p0g,
            p,
            control,
            count,
            Ghost(cur0 as int),
            Ghost(*produced as int),
            Ghost(acc0),
            steps, clip, subst,
        )
    } else {
        rle_literal_packet_policy(
            bytes,
            h,
            map,
            indexed,
            out,
            cursor,
            produced,
            n,
            p0g,
            p,
            control,
            count,
            Ghost(cur0 as int),
            Ghost(*produced as int),
            Ghost(acc0),
            steps, clip, subst,
        )
    }
}

/// A policy repeat packet: exactly one complete sample, painted `self.paint`
/// times, the cursor advancing one sample past the control byte.
fn rle_repeat_packet_policy(
    bytes: &[u8],
    h: &Header,
    map: &[u8],
    indexed: bool,
    out: &mut Vec<u8>,
    cursor: &mut usize,
    produced: &mut usize,
    n: usize,
    p0g: usize,
    p: Policy,
    control: u8,
    count: usize,
    Ghost(cur0): Ghost<int>,
    Ghost(pg0): Ghost<int>,
    Ghost(acc0): Ghost<Seq<Seq<u8>>>,
    steps: &mut Ghost<nat>,
    clip: &mut bool, subst: &mut bool,
) -> (r: (Result<(), Error>, Ghost<Seq<Seq<u8>>>))
    requires
        header_inv_policy(*h, p),
        n == h.width as int * (h.height as int),
        indexed == (crate::policy_model::kind_policy(h.image_type, p) == 0),
        indexed ==> crate::policy_model::map_present_policy_flags(h.color_map_flag, p),
        old(out)@.len() == 4 * n,
        0 <= cur0 < bytes@.len(),
        *old(cursor) as int == cur0 + 1,
        *old(produced) as int == pg0,
        bytes@[cur0] == control,
        crate::model::rle_repeat(control as int),
        count as int == crate::model::rle_count(control as int),
        1 <= count <= 128,
        p0g <= *old(produced) < n,
        !(count as int > n as int - *old(produced) as int && p.rle_overrun
            == crate::policy::RleOverrunOptions::Reject),
        bytes@.len() <= crate::model::max_input(),
        map@.len() == crate::policy_model::map_bytes_flags(
            h.color_map_flag,
            h.map_length as int,
            h.map_entry_depth,
            p,
        ),
        map@.len() <= 262140,
        pkt_out_policy(
            old(out)@,
            acc0,
            h.width as int,
            h.height as int,
            h.spec_flip_x(),
            h.spec_flip_y(),
            p0g as int,
            *old(produced) as int,
        ),
    ensures
        r.0 is Ok ==> crate::policy_model::event_step_policy(bytes@, pkt_of_policy(h, p), map@, p,
            cur0, pg0, *final(cursor) as int, *final(produced) as int,
            *old(clip), *old(subst), *final(clip), *final(subst)),
        *old(produced) <= *final(produced),
        final(steps)@ as int <= old(steps)@ as int + 5 * (*final(produced) as int - *old(produced) as int) + 1,
        final(out)@.len() == 4 * n,
        *final(cursor) <= bytes@.len(),
        *final(produced) <= n,
        pkt_out_policy(
            final(out)@,
            r.1@,
            h.width as int,
            h.height as int,
            h.spec_flip_x(),
            h.spec_flip_y(),
            p0g as int,
            *final(produced) as int,
        ),
        crate::policy_model::pkt_post_policy(
            bytes@,
            pkt_of_policy(h, p),
            map@,
            p,
            cur0,
            pg0,
            *final(cursor) as int,
            *final(produced) as int,
            acc0,
            r.1@,
            r.0,
        ),
{
    let ghost sub0 = *subst;
    let clipped = count > n - *produced;
    let ghost start_steps = steps@;
    let ghost start_produced = *produced;
    let ghost pkt = pkt_of_policy(h, p);
    let sb = h.sample_bytes();
    let width = h.width;
    let height = h.height;
    let (fx, fy) = (h.flip_x(), h.flip_y());
    proof {
        reveal(pkt_out_policy);
        reveal(pkt_of_policy);
        crate::policy_model::lemma_sample_bytes_of_stride(h.pixel_depth);
        reveal(header_inv_policy);
        assert(sb as int == h.spec_sample_bytes());
        assert(sb >= 1);
        assert(sb <= 4);
        assert(pkt.n == n as int);
        assert(pkt.sample_bytes == sb as int);
        assert(bytes@[cur0] == control);
        assert(acc0.len() == pg0 - p0g as int);
    }
    // Repeat packet: exactly one complete sample.
    if bytes.len() - *cursor < sb {
        proof {
            crate::policy_model::lemma_pkt_policy_rep_missing(
                bytes@,
                pkt,
                map@,
                p,
                cur0,
                pg0,
                acc0,
            );
        }
        return (Err(Error::RleTruncated), Ghost(acc0));
    }
    let end = *cursor + sb;
    let s = &bytes[*cursor..end];
    *cursor = end;
    proof {
        assert(s@.len() == sb as int);
        assert(s@ == bytes@.subrange(cur0 + 1, cur0 + 1 + sb as int));
        assert(*cursor as int == cur0 + 1 + sb as int);
    }
    proof { *steps = Ghost(steps@ + 1); }
    let rgba = match convert_one_policy(h, map, indexed, s, p, subst) {
        Ok(px) => px,
        Err(e) => {
            proof {
                assert(indexed);
                assert(pkt.kind == 0);
                assert(e == Error::ColorIndexOutOfRange);
                crate::policy_model::lemma_pkt_policy_rep_index(
                    bytes@,
                    pkt,
                    map@,
                    p,
                    cur0,
                    pg0,
                    acc0,
                );
            }
            return (Err(e), Ghost(acc0));
        },
    };
    proof {
        assert(rgba@ == crate::policy_model::conv2_policy(
            pkt,
            map@,
            bytes@.subrange(cur0 + 1, cur0 + 1 + sb as int),
            p,
        ));
    }
    let ghost here = pkt.kind == 0 && crate::policy_model::pkt_subst(pkt, crate::model::sindex(s@, sb as int), p);
    let paint = if count < n - *produced {
        count
    } else {
        n - *produced
    };
    let ghost g0: usize = *produced;
    let ghost mut acc: Seq<Seq<u8>> = acc0;
    proof {
        crate::model::lemma_pix_pow_zero(rgba@);
        assert(acc == acc0 + crate::model::pix_pow(rgba@, 0)) by {
            broadcast use vstd::seq_lib::group_seq_lib_default;
        }
    }
    let mut i: usize = 0;
    while i < paint
        invariant
            i <= paint,
            sub0 == *old(subst),
            *subst == (sub0 || here),
            start_produced == g0,
            start_produced == *old(produced),
            start_steps == old(steps)@,
            steps@ == start_steps + 1 + 4 * i,
            paint <= count,
            *produced == g0 + i,
            g0 + paint <= n,
            *produced <= n,
            count >= 1,
            out@.len() == 4 * n,
            rgba@.len() == 4,
            n <= 16777216,
            width as int * (height as int) == n,
            1 <= (width as int) <= 8192,
            1 <= (height as int) <= 8192,
            fx == h.spec_flip_x(),
            fy == h.spec_flip_y(),
            p0g <= g0,
            acc == acc0 + crate::model::pix_pow(rgba@, i as int),
            acc.len() == *produced as int - p0g as int,
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
            header_inv_policy(*h, p),
            pkt == pkt_of_policy(h, p),
            sb as int == h.spec_sample_bytes(),
            *cursor as int == cur0 + 1 + sb as int,
            bytes@.len() <= crate::model::max_input(),
        decreases paint - i,
    {
        let off = output_offset(width, height, fx, fy, *produced);
        proof {
            assert(*produced < n) by (nonlinear_arith)
                requires
                    *produced == g0 + i,
                    i < paint,
                    g0 + paint <= n,
            ;
            assert(off + 4 <= 4 * n) by (nonlinear_arith)
                requires
                    (off as int) <= 4 * (n - 1),
                    1 <= n,
                    n <= 16777216,
            ;
        }
        let ghost out_before = out@;
        write_pixel(out, off, &rgba);
        proof { *steps = Ghost(steps@ + 4); }
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
            crate::model::lemma_pix_pow_peel(rgba@, i as int);
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
        reveal(pkt_out_policy);
        assert(*produced == g0 + paint);
        assert(acc == acc0 + crate::model::pix_pow(rgba@, paint as int));
        crate::policy_model::lemma_mr_pix_policy_rep(bytes@, pkt, map@, p, cur0, pg0);
        crate::policy_model::lemma_packet_events(bytes@, pkt, map@, p, cur0, pg0, count as int, paint as int, *cursor as int, here);
        assert(crate::policy_model::mr_pix_policy(bytes@, pkt, map@, p, cur0, pg0)
            == crate::model::pix_pow(rgba@, paint as int) + crate::policy_model::mr_pix_policy(
            bytes@,
            pkt,
            map@,
            p,
            cur0 + 1 + sb as int,
            pg0 + paint as int,
        ));
        crate::policy_model::lemma_pkt_policy_rep_ok(
            bytes@,
            pkt,
            map@,
            p,
            cur0,
            pg0,
            acc0,
            acc,
        );
    }
    *clip = *clip || clipped;
    (Ok(()), Ghost(acc))
}

/// A policy literal packet: under `Reject`/`ClipPacket` the whole packet's bytes
/// are present, under `ClipSamples` only the painted samples; each sample's index
/// is checked in output order and the cursor advances by the consumed bytes.
fn rle_literal_packet_policy(
    bytes: &[u8],
    h: &Header,
    map: &[u8],
    indexed: bool,
    out: &mut Vec<u8>,
    cursor: &mut usize,
    produced: &mut usize,
    n: usize,
    p0g: usize,
    p: Policy,
    control: u8,
    count: usize,
    Ghost(cur0): Ghost<int>,
    Ghost(pg0): Ghost<int>,
    Ghost(acc0): Ghost<Seq<Seq<u8>>>,
    steps: &mut Ghost<nat>,
    clip: &mut bool, subst: &mut bool,
) -> (r: (Result<(), Error>, Ghost<Seq<Seq<u8>>>))
    requires
        header_inv_policy(*h, p),
        n == h.width as int * (h.height as int),
        indexed == (crate::policy_model::kind_policy(h.image_type, p) == 0),
        indexed ==> crate::policy_model::map_present_policy_flags(h.color_map_flag, p),
        old(out)@.len() == 4 * n,
        0 <= cur0 < bytes@.len(),
        *old(cursor) as int == cur0 + 1,
        *old(produced) as int == pg0,
        bytes@[cur0] == control,
        !crate::model::rle_repeat(control as int),
        count as int == crate::model::rle_count(control as int),
        1 <= count <= 128,
        p0g <= *old(produced) < n,
        !(count as int > n as int - *old(produced) as int && p.rle_overrun
            == crate::policy::RleOverrunOptions::Reject),
        bytes@.len() <= crate::model::max_input(),
        map@.len() == crate::policy_model::map_bytes_flags(
            h.color_map_flag,
            h.map_length as int,
            h.map_entry_depth,
            p,
        ),
        map@.len() <= 262140,
        pkt_out_policy(
            old(out)@,
            acc0,
            h.width as int,
            h.height as int,
            h.spec_flip_x(),
            h.spec_flip_y(),
            p0g as int,
            *old(produced) as int,
        ),
    ensures
        r.0 is Ok ==> crate::policy_model::event_step_policy(bytes@, pkt_of_policy(h, p), map@, p,
            cur0, pg0, *final(cursor) as int, *final(produced) as int,
            *old(clip), *old(subst), *final(clip), *final(subst)),
        *old(produced) <= *final(produced),
        final(steps)@ as int <= old(steps)@ as int + 5 * (*final(produced) as int - *old(produced) as int) + 1,
        final(out)@.len() == 4 * n,
        *final(cursor) <= bytes@.len(),
        *final(produced) <= n,
        pkt_out_policy(
            final(out)@,
            r.1@,
            h.width as int,
            h.height as int,
            h.spec_flip_x(),
            h.spec_flip_y(),
            p0g as int,
            *final(produced) as int,
        ),
        crate::policy_model::pkt_post_policy(
            bytes@,
            pkt_of_policy(h, p),
            map@,
            p,
            cur0,
            pg0,
            *final(cursor) as int,
            *final(produced) as int,
            acc0,
            r.1@,
            r.0,
        ),
{
    let ghost sub0 = *subst;
    let clipped = count > n - *produced;
    let ghost start_steps = steps@;
    let ghost start_produced = *produced;
    let ghost pkt = pkt_of_policy(h, p);
    let sb = h.sample_bytes();
    let width = h.width;
    let height = h.height;
    let (fx, fy) = (h.flip_x(), h.flip_y());
    proof {
        reveal(pkt_out_policy);
        reveal(pkt_of_policy);
        crate::policy_model::lemma_sample_bytes_of_stride(h.pixel_depth);
        reveal(header_inv_policy);
        assert(sb as int == h.spec_sample_bytes());
        assert(sb >= 1);
        assert(sb <= 4);
        assert(pkt.n == n as int);
        assert(pkt.sample_bytes == sb as int);
        assert(acc0.len() == pg0 - p0g as int);
    }
    let paint = if count < n - *produced {
        count
    } else {
        n - *produced
    };
    let need = match p.rle_overrun {
        crate::policy::RleOverrunOptions::ClipSamples => paint,
        _ => count,
    };
    proof {
        if p.rle_overrun == crate::policy::RleOverrunOptions::ClipSamples {
            assert(need == paint);
        } else {
            assert(need == count);
            assert(paint <= count);
        }
        assert(paint <= need);
        assert(need <= count);
        assert(need * sb <= 512) by (nonlinear_arith)
            requires
                need <= count,
                count <= 128,
                sb <= 4,
        ;
    }
    let total = need * sb;
    if bytes.len() - *cursor < total {
        proof {
            crate::policy_model::lemma_pkt_policy_lit_missing(
                bytes@,
                pkt,
                map@,
                p,
                cur0,
                pg0,
                acc0,
            );
        }
        return (Err(Error::RleTruncated), Ghost(acc0));
    }
    let end = *cursor + total;
    let packet = &bytes[*cursor..end];
    *cursor = end;
    proof {
        assert(packet@.len() == need * sb);
        assert(packet@ == bytes@.subrange(cur0 + 1, cur0 + 1 + need as int * sb as int));
        assert(*cursor as int == cur0 + 1 + need as int * sb as int);
    }
    let ghost g0: usize = *produced;
    let ghost mut acc: Seq<Seq<u8>> = acc0;
    proof {
        assert(crate::policy_model::lit_out_policy(bytes@, pkt, map@, p, cur0 + 1, 0)
            == Seq::<Seq<u8>>::empty()) by {
            reveal(crate::policy_model::lit_out_policy);
        }
        assert(acc == acc0 + crate::policy_model::lit_out_policy(bytes@, pkt, map@, p, cur0 + 1, 0))
            by {
            broadcast use vstd::seq_lib::group_seq_lib_default;
        }
    }
    let mut i: usize = 0;
    while i < paint
        invariant
            i <= paint,
            sub0 == *old(subst),
            *subst == (sub0 || crate::policy_model::lit_flags_policy(bytes@, pkt, p, cur0 + 1, i as int).0),
            start_produced == g0,
            start_produced == *old(produced),
            start_steps == old(steps)@,
            steps@ == start_steps + 5 * i,
            paint <= count,
            paint <= need,
            *produced == g0 + i,
            g0 + paint <= n,
            *produced <= n,
            count >= 1,
            packet@.len() == need * sb,
            cur0 + 1 + need as int * sb as int <= bytes@.len(),
            width == h.width,
            height == h.height,
            paint as int == (if (count as int) < n as int - pg0 { count as int } else { n as int - pg0 }),
            need == (if p.rle_overrun == crate::policy::RleOverrunOptions::ClipSamples { paint } else { count }),
            !(count as int > n as int - pg0 && p.rle_overrun == crate::policy::RleOverrunOptions::Reject),
            *cursor as int == cur0 + 1 + need as int * sb as int,
            sb as int == h.spec_sample_bytes(),
            pkt == pkt_of_policy(h, p),
            pkt.n == n as int,
            sb >= 1,
            sb <= 4,
            out@.len() == 4 * n,
            n <= 16777216,
            width as int * (height as int) == n,
            1 <= (width as int) <= 8192,
            1 <= (height as int) <= 8192,
            header_inv_policy(*h, p),
            map@.len() == crate::policy_model::map_bytes_flags(
                h.color_map_flag, h.map_length as int, h.map_entry_depth, p,
            ),
            indexed == (crate::policy_model::kind_policy(h.image_type, p) == 0),
            fx == h.spec_flip_x(),
            fy == h.spec_flip_y(),
            0 <= cur0 < bytes@.len(),
            packet@ == bytes@.subrange(cur0 + 1, cur0 + 1 + need as int * sb as int),
            p0g <= g0,
            g0 as int == pg0,
            acc == acc0 + crate::policy_model::lit_out_policy(bytes@, pkt, map@, p, cur0 + 1, i as int),
            crate::policy_model::lit_ok_policy(bytes@, pkt, cur0 + 1, i as int, p),
            acc.len() == *produced as int - p0g as int,
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
            count as int == crate::model::rle_count(bytes@[cur0] as int),
            !crate::model::rle_repeat(bytes@[cur0] as int),
            bytes@.len() <= crate::model::max_input(),
        decreases paint - i,
    {
        proof {
            assert(i * sb + sb <= need * sb) by (nonlinear_arith)
                requires
                    i < paint,
                    paint <= need,
                    sb >= 0,
            ;
            assert(i * sb + sb <= packet@.len());
        }
        let s = &packet[i * sb..i * sb + sb];
        proof {
            reveal(header_inv_policy);
            reveal(crate::policy_model::gov_depth);
            reveal(crate::policy_model::kind_policy);
            reveal(pkt_of_policy);
            assert(map@.len() == crate::policy_model::map_bytes_flags(
                h.color_map_flag,
                h.map_length as int,
                h.map_entry_depth,
                p,
            ));
            assert(h.map_length as int <= 65535);
            assert(pkt.sample_bytes == sb as int);
            assert(i as int * sb as int + sb as int == (i as int + 1) * sb as int) by (nonlinear_arith);
            assert(s@ == packet@.subrange(i as int * sb as int, (i as int + 1) * sb as int));
            vstd::seq::lemma_seq_subrange_composition(
                bytes@,
                cur0 + 1,
                cur0 + 1 + need as int * sb as int,
                (i * sb) as int,
                ((i + 1) * sb) as int,
            );
            assert(s@ == bytes@.subrange(
                cur0 + 1 + i as int * pkt.sample_bytes,
                cur0 + 1 + (i + 1) as int * pkt.sample_bytes,
            ));
        }
        proof { *steps = Ghost(steps@ + 1); }
        let rgba = match convert_one_policy(h, map, indexed, s, p, subst) {
            Ok(px) => px,
            Err(e) => {
                proof {
                    assert(indexed);
                    assert(pkt.kind == 0);
                    assert(e == Error::ColorIndexOutOfRange);
                    crate::policy_model::lemma_lit_good_policy_first_bad(
                        bytes@, pkt, p, cur0 + 1, paint as int, i as int,
                    );
                    assert(crate::policy_model::lit_good_policy(
                        bytes@, pkt, cur0 + 1, paint as int, p,
                    ) == i as int);
                    crate::policy_model::lemma_pkt_policy_lit_index(
                        bytes@,
                        pkt,
                        map@,
                        p,
                        cur0,
                        pg0,
                        acc0,
                        acc,
                    );
                }
                return (Err(e), Ghost(acc));
            },
        };
        let off = output_offset(width, height, fx, fy, *produced);
        proof {
            assert(*produced < n) by (nonlinear_arith)
                requires
                    *produced == g0 + i,
                    i < paint,
                    g0 + paint <= n,
            ;
            assert(off + 4 <= 4 * n) by (nonlinear_arith)
                requires
                    (off as int) <= 4 * (n - 1),
                    1 <= n,
                    n <= 16777216,
            ;
        }
        let ghost out_before = out@;
        write_pixel(out, off, &rgba);
        proof { *steps = Ghost(steps@ + 4); }
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
            assert(s@ == bytes@.subrange(
                cur0 + 1 + (i - 1) as int * pkt.sample_bytes,
                cur0 + 1 + i as int * pkt.sample_bytes,
            ));
            crate::policy_model::lemma_lit_flags_extend(bytes@, pkt, p, cur0 + 1, i as int - 1);
            crate::policy_model::lemma_lit_ok_policy_extend(
                bytes@, pkt, p, cur0 + 1, i as int - 1,
            );
            crate::policy_model::lemma_lit_out_policy_peel(bytes@, pkt, map@, p, cur0 + 1, i as int - 1);
            assert(crate::policy_model::lit_out_policy(bytes@, pkt, map@, p, cur0 + 1, i as int)
                == crate::policy_model::lit_out_policy(bytes@, pkt, map@, p, cur0 + 1, i as int - 1)
                + seq![rgba@]);
            vstd::seq_lib::lemma_concat_associative(
                acc0,
                crate::policy_model::lit_out_policy(bytes@, pkt, map@, p, cur0 + 1, i as int - 1),
                seq![rgba@],
            );
            acc = acc + seq![rgba@];
            assert(acc == acc0 + crate::policy_model::lit_out_policy(
                bytes@,
                pkt,
                map@,
                p,
                cur0 + 1,
                i as int,
            ));
            assert(acc.len() == *produced as int - p0g as int) by {
                crate::policy_model::lemma_lit_out_policy_len(bytes@, pkt, map@, p, cur0 + 1, i as int);
                broadcast use vstd::seq::group_seq_lemmas;
            }
        }
    }
    proof {
        reveal(pkt_out_policy);
        assert(*produced == g0 + paint);
        assert(acc == acc0 + crate::policy_model::lit_out_policy(bytes@, pkt, map@, p, cur0 + 1, paint
            as int));
        crate::policy_model::lemma_mr_pix_policy_lit(bytes@, pkt, map@, p, cur0, pg0);
        crate::policy_model::lemma_packet_events(bytes@, pkt, map@, p, cur0, pg0, count as int, paint as int, *cursor as int,
            crate::policy_model::lit_flags_policy(bytes@, pkt, p, cur0 + 1, paint as int).0);
        crate::policy_model::lemma_pkt_policy_lit_ok(
            bytes@,
            pkt,
            map@,
            p,
            cur0,
            pg0,
            acc0,
            acc,
        );
    }
    *clip = *clip || clipped;
    (Ok(()), Ghost(acc))
}

/// Execute the policy packet stream without iterating over discarded samples.
fn decode_rle_policy(
    bytes: &[u8], h: &Header, map: &[u8], indexed: bool,
    out: &mut Vec<u8>, cursor: &mut usize, produced: &mut usize,
    n: usize, c0g: usize, p0g: usize, p: Policy,
    steps: &mut Ghost<nat>,
    clip: &mut bool, subst: &mut bool,
) -> (r: Result<(), Error>)
    requires
        header_inv_policy(*h, p),
        n == h.width as int * h.height as int,
        indexed == (crate::policy_model::kind_policy(h.image_type, p) == 0),
        indexed ==> crate::policy_model::map_present_policy_flags(h.color_map_flag, p),
        old(out)@.len() == 4 * n,
        *old(cursor) == c0g, c0g <= bytes@.len(),
        *old(produced) == p0g, p0g <= n,
        bytes@.len() <= crate::model::max_input(),
        map@.len() == crate::policy_model::map_bytes_flags(h.color_map_flag, h.map_length as int, h.map_entry_depth, p),
        map@.len() <= 262140,
    ensures
        match crate::policy_model::mr_policy(bytes@, pkt_of_policy(h, p), map@, p, c0g as int, p0g as int) {
            Ok((c, pr, px)) => r is Ok && *final(cursor) == c && *final(produced) == pr
                && crate::model::wrote_px_off(final(out)@, px, h.width as int, h.height as int,
                    h.spec_flip_x(), h.spec_flip_y(), p0g as int, px.len() as int),
            Err((e, c, pr, px)) => e == crate::model::MrErr::Api(r->Err_0) && r is Err
                && *final(cursor) == c && *final(produced) == pr
                && crate::model::wrote_px_off(final(out)@, px, h.width as int, h.height as int,
                    h.spec_flip_x(), h.spec_flip_y(), p0g as int, px.len() as int),
        },
        r is Ok ==> *final(clip) == (*old(clip) || crate::policy_model::mr_flags_policy(bytes@, pkt_of_policy(h, p), map@, p, c0g as int, p0g as int).0),
        r is Ok ==> *final(subst) == (*old(subst) || crate::policy_model::mr_flags_policy(bytes@, pkt_of_policy(h, p), map@, p, c0g as int, p0g as int).1),
        final(steps)@ as int <= old(steps)@ as int + 7 * (n as int - p0g as int) + 2,
        final(out)@.len() == 4 * n,
        *final(cursor) <= bytes@.len(),
        p0g <= *final(produced) <= n,
        crate::policy_model::mr_pix_policy(bytes@, pkt_of_policy(h, p), map@, p, c0g as int, p0g as int).len()
            == *final(produced) as int - p0g as int,
        crate::model::wrote_px_off(final(out)@,
            crate::policy_model::mr_pix_policy(bytes@, pkt_of_policy(h, p), map@, p, c0g as int, p0g as int),
            h.width as int, h.height as int, h.spec_flip_x(), h.spec_flip_y(),
            p0g as int, *final(produced) as int - p0g as int),
        match r {
            Ok(()) => *final(produced) == n &&
                crate::policy_model::mr_cur_policy(bytes@, pkt_of_policy(h, p), map@, p, c0g as int, p0g as int)
                    == Ok((*final(cursor) as int, n as int)),
            Err(e) => crate::policy_model::mr_cur_policy(bytes@, pkt_of_policy(h, p), map@, p, c0g as int, p0g as int)
                    == Err(crate::model::MrErr::Api(e)) &&
                crate::policy_model::mr_gcur_policy(bytes@, pkt_of_policy(h, p), map@, p, c0g as int, p0g as int)
                    == Ok(*final(cursor) as int),
        },
{
    let ghost start_steps = steps@;
    let ghost pkt = pkt_of_policy(h, p);
    let ghost mut acc: Seq<Seq<u8>> = Seq::empty();
    proof {
        reveal(pkt_out_policy);
        reveal(crate::model::wrote_px_off);
        broadcast use vstd::seq_lib::group_seq_lib_default;
    }
    while *produced < n
        invariant
            crate::policy_model::event_step_policy(bytes@, pkt, map@, p, c0g as int, p0g as int,
                *cursor as int, *produced as int, *old(clip), *old(subst), *clip, *subst),
            start_steps == old(steps)@,
            steps@ as int <= start_steps as int + 7 * (*produced as int - p0g as int),
            header_inv_policy(*h, p),
            pkt == pkt_of_policy(h, p),
            n == h.width as int * h.height as int,
            indexed == (crate::policy_model::kind_policy(h.image_type, p) == 0),
            indexed ==> crate::policy_model::map_present_policy_flags(h.color_map_flag, p),
            out@.len() == 4 * n,
            *cursor <= bytes@.len(), p0g <= *produced <= n,
            bytes@.len() <= crate::model::max_input(),
            map@.len() == crate::policy_model::map_bytes_flags(h.color_map_flag, h.map_length as int, h.map_entry_depth, p),
            map@.len() <= 262140,
            c0g <= bytes@.len(), p0g <= n,
            pkt.n == n as int, 1 <= pkt.sample_bytes,
            pkt_out_policy(out@, acc, h.width as int, h.height as int, h.spec_flip_x(), h.spec_flip_y(), p0g as int, *produced as int),
            crate::policy_model::mr_pix_policy(bytes@, pkt, map@, p, c0g as int, p0g as int)
                == acc + crate::policy_model::mr_pix_policy(bytes@, pkt, map@, p, *cursor as int, *produced as int),
            crate::policy_model::mr_cur_policy(bytes@, pkt, map@, p, c0g as int, p0g as int)
                == crate::policy_model::mr_cur_policy(bytes@, pkt, map@, p, *cursor as int, *produced as int),
            crate::policy_model::mr_gcur_policy(bytes@, pkt, map@, p, c0g as int, p0g as int)
                == crate::policy_model::mr_gcur_policy(bytes@, pkt, map@, p, *cursor as int, *produced as int),
        decreases n - *produced,
    {
        let (result, pixels) = rle_packet_policy(bytes, h, map, indexed, out, cursor, produced, n, p0g, p, Ghost(acc), steps, clip, subst);
        proof {
            acc = pixels@;
            reveal(crate::policy_model::pkt_post_policy);
            reveal(crate::policy_model::event_step_policy);
            reveal(pkt_out_policy);
        }
        match result {
            Ok(()) => {},
            Err(e) => {
                proof {
                    crate::policy_model::lemma_mr_state_policy_proj(bytes@, pkt, map@, p, c0g as int, p0g as int, Seq::empty());
                    reveal(crate::policy_model::mr_policy);
                    broadcast use vstd::seq_lib::group_seq_lib_default;
                }
 return Err(e); },
        }
    }
    proof {
        reveal(pkt_out_policy);
        reveal(crate::policy_model::event_step_policy);
        reveal(crate::policy_model::mr_flags_policy);
        reveal(pkt_of_policy);
        assert(pkt.n == n as int);
        reveal(crate::policy_model::mr_cur_policy);
        reveal(crate::policy_model::mr_pix_policy);
        assert(crate::policy_model::mr_pix_policy(bytes@, pkt, map@, p, *cursor as int, *produced as int) == Seq::<Seq<u8>>::empty());
        broadcast use vstd::seq_lib::group_seq_lib_default;
    }
    proof {
        crate::policy_model::lemma_mr_state_policy_proj(bytes@, pkt, map@, p, c0g as int, p0g as int, Seq::empty());
        reveal(crate::policy_model::mr_policy);
        broadcast use vstd::seq_lib::group_seq_lib_default;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// alloc-delta: guard threading and the `try_*` API (SPEC.md §1-§4,
// THEOREMS-ALLOC-DELTA.md §0/§2, A-TGA-1..3 and the added A-TGA-5; frozen
// A-TGA-4 is termination).
// ---------------------------------------------------------------------------

pub use crate::alloc_guard::sites::TryError;
use crate::alloc_guard::{AllocSite, AllocLog, OutOfMemory};


/// `Req_TGA(b)`: the ordered requests the `decode` family issues when every
/// reservation succeeds (SPEC.md §4): the one `(Output, 4N)` request iff
/// `MH(b)` is `Ok`, else `[]`. `N` is `0` on a rejected header, so `req_tga` is
/// then empty.
#[doc(hidden)]
pub open spec fn req_tga(b: Seq<u8>) -> Seq<(AllocSite, nat)> {
    match crate::model::mh(b) {
        Ok(m) => seq![(AllocSite::Output, 4 * m.n as nat)],
        Err(_) => Seq::empty(),
    }
}

/// The same requests for the policy entry point, whose header model is
/// `mh_policy` (the policy `Reach` is `MH_policy(b, p)` is `Ok`).
#[doc(hidden)]
pub open spec fn req_tga_policy(b: Seq<u8>, p: Policy) -> Seq<(AllocSite, nat)> {
    match crate::policy_model::mh_policy(b, p) {
        Ok(m) => seq![(AllocSite::Output, 4 * m.n as nat)],
        Err(_) => Seq::empty(),
    }
}

/// Clause 1 of `G` (THEOREMS-ALLOC-DELTA.md §0): a non-`OutOfMemory` result
/// satisfies the frozen predicate on `crate::alloc_guard::sites::fmt_part(r)`, has exactly the requests of
/// `Req_TGA`, and every log entry is `Ok`.
#[doc(hidden)]
pub open spec fn g_clause1(
    b: Seq<u8>,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    match r {
        Err(TryError::OutOfMemory(_)) => true,
        _ => decode_eq(crate::alloc_guard::sites::fmt_part(r), b)
            && crate::alloc_guard::req_of_log(l) == req_tga(b)
            && crate::alloc_guard::all_ok(l),
    }
}

/// Clause 2 of `G`: an `OutOfMemory` result has a non-empty log whose requests
/// are a prefix of `Req_TGA`, every entry but the last `Ok`, and the last entry
/// not `Ok` and equal to the returned `(site, elements)`.
#[doc(hidden)]
pub open spec fn g_clause2(
    b: Seq<u8>,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    match r {
        Err(TryError::OutOfMemory(o)) =>
            l.len() >= 1
            && crate::alloc_guard::req_of_log(l).is_prefix_of(req_tga(b))
            && (forall|j: int| 0 <= j < l.len() - 1 ==> l[j].2)
            && !l[l.len() - 1].2
            && l[l.len() - 1].0 == o.site_spec()
            && l[l.len() - 1].1 == o.elements_spec() as nat,
        _ => true,
    }
}

/// Clause 3 of `G`: every request is within its site's maximum, proved from the
/// checked header before the guard call.
#[doc(hidden)]
pub open spec fn g_clause3(b: Seq<u8>) -> bool {
    forall|j: int| 0 <= j < req_tga(b).len() ==> req_tga(b)[j].1 <= crate::alloc_guard::MAX_BYTES
}

/// Theorem shape `G` for TGA's `decode` family (THEOREMS-ALLOC-DELTA.md
/// A-TGA-1), proved on the logged entry point `try_decode_with_steps_logged`.
#[doc(hidden)]
pub open spec fn g(
    b: Seq<u8>,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    g_clause1(b, r, l) && g_clause2(b, r, l) && g_clause3(b)
}

/// `Prog` (THEOREMS-ALLOC-DELTA.md §0): if every log entry is `Ok`, the result
/// is not `OutOfMemory`, `decode_eq` holds, and `req(L) == Req_TGA`.
#[doc(hidden)]
pub open spec fn prog(
    b: Seq<u8>,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    crate::alloc_guard::all_ok(l) ==> prog_concl(b, r, l)
}

/// The `Prog` conclusion, split out so `prog` reveals to the solver as
/// `all_ok(l) ==> prog_concl(..)`.
#[doc(hidden)]
pub open spec fn prog_concl(
    b: Seq<u8>,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    !matches!(r, Err(TryError::OutOfMemory(_)))
        && decode_eq(crate::alloc_guard::sites::fmt_part(r), b)
        && crate::alloc_guard::req_of_log(l) == req_tga(b)
}

/// The `PreAlloc` corollary (THEOREMS-ALLOC-DELTA.md §0): `Req_TGA` empty means
/// the header was rejected before any reservation, so the log is empty and the
/// result is a format error satisfying `decode_eq`.
#[doc(hidden)]
pub open spec fn pre_alloc(b: Seq<u8>) -> bool {
    req_tga(b).len() == 0
}

/// The log-only facts `at_site_log` gives, in the shape the entry points' proof
/// blocks consume: `req(L)` is `Req_TGA`, which has at most one element, so `L`
/// has at most one element, and a failure is the log's last (only) entry.
pub(crate) open spec fn at_site_log(
    b: Seq<u8>,
    r_is_oom: bool,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    crate::alloc_guard::req_of_log(l) == req_tga(b)
        && (if r_is_oom {
            l.len() >= 1 && !l[l.len() - 1].2
        } else {
            crate::alloc_guard::all_ok(l)
        })
}

/// The `Ok`-exit `at_site_log`: after the one successful `(Output, 4N)`
/// reservation on an empty log, `req(L)` is exactly `Req_TGA`.
pub(crate) proof fn lemma_at_site_log_ok(
    b: Seq<u8>,
    m: crate::model::MHModel,
    l: Seq<(AllocSite, nat, bool)>,
)
    requires
        crate::model::mh(b) == Ok(m),
        l == Seq::<(AllocSite, nat, bool)>::empty().push(
            (AllocSite::Output, 4 * m.n as nat, true)),
    ensures at_site_log(b, false, l),
{
    reveal(at_site_log);
    reveal(req_tga);
    reveal(crate::alloc_guard::req_of_log);
    reveal(crate::alloc_guard::all_ok);
    assert(crate::alloc_guard::req_of_log(l) =~= req_tga(b));
}

/// The `OutOfMemory`-exit `at_site_log`.
pub(crate) proof fn lemma_at_site_log_oom(
    b: Seq<u8>,
    m: crate::model::MHModel,
    l: Seq<(AllocSite, nat, bool)>,
)
    requires
        crate::model::mh(b) == Ok(m),
        l == Seq::<(AllocSite, nat, bool)>::empty().push(
            (AllocSite::Output, 4 * m.n as nat, false)),
    ensures at_site_log(b, true, l),
{
    reveal(at_site_log);
    reveal(req_tga);
    reveal(crate::alloc_guard::req_of_log);
    reveal(crate::alloc_guard::all_ok);
    assert(crate::alloc_guard::req_of_log(l) =~= req_tga(b));
}

/// The pre-reservation `at_site_log`.
pub(crate) proof fn lemma_at_site_log_prealloc(
    b: Seq<u8>,
    l: Seq<(AllocSite, nat, bool)>,
)
    requires
        l.len() == 0,
        crate::model::mh(b) is Err,
    ensures at_site_log(b, false, l),
{
    reveal(at_site_log);
    reveal(req_tga);
    reveal(crate::alloc_guard::req_of_log);
    reveal(crate::alloc_guard::all_ok);
    assert(crate::alloc_guard::req_of_log(l) =~= req_tga(b));
}

/// Clause 3's content, as a named lemma: `Req_TGA`'s single element is `4N`
/// with `1 <= N <= 16,777,216`, so it is within `MAX_BYTES`. Proved from the
/// model's `mh_bounds`, never from an executable read.
pub(crate) proof fn lemma_req_tga_clause3(b: Seq<u8>)
    ensures g_clause3(b),
{
    reveal(req_tga);
    reveal(g_clause3);
    if req_tga(b).len() > 0 {
        let m = choose|m: crate::model::MHModel| crate::model::mh(b) == Ok(m);
        assert(crate::model::mh(b) == Ok(m));
        crate::model::lemma_mh_ok_bounds(b, m);
        reveal(crate::model::mh_bounds);
        assert(req_tga(b)[0].1 == 4 * m.n as nat);
        assert(m.n <= 16777216);
        assert(4 * m.n <= 67108864);
    }
}

/// The one-request shape, as a lemma: `Req_TGA` has at most one element, so
/// `req(L) == Req_TGA` plus the last-entry equality gives the prefix clause on
/// the `Err` arm.
pub(crate) proof fn lemma_req_tga_len(b: Seq<u8>)
    ensures req_tga(b).len() <= 1,
{
    reveal(req_tga);
}

/// The log-only facts `at_site_log` gives: `req(L)` is `Req_TGA`, which has at
/// most one element, so `L` has at most one element, and a failure is the log's
/// last (only) entry.
pub(crate) proof fn lemma_at_site_log_parts(
    b: Seq<u8>,
    r_is_oom: bool,
    l: Seq<(AllocSite, nat, bool)>,
)
    requires at_site_log(b, r_is_oom, l),
    ensures
        crate::alloc_guard::req_of_log(l) == req_tga(b),
        req_tga(b).len() <= 1,
        l.len() <= 1,
        r_is_oom ==> (l.len() == 1 && !l[0].2),
        !r_is_oom ==> crate::alloc_guard::all_ok(l),
{
    reveal(at_site_log);
    reveal(req_tga);
    reveal(crate::alloc_guard::req_of_log);
    reveal(crate::alloc_guard::all_ok);
}

/// The policy path's `at_site_log`, whose request set is `Req_TGA_policy`.
pub(crate) open spec fn at_site_log_policy(
    b: Seq<u8>,
    p: Policy,
    r_is_oom: bool,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    crate::alloc_guard::req_of_log(l) == req_tga_policy(b, p)
        && (if r_is_oom {
            l.len() == 1 && !l[0].2
        } else {
            crate::alloc_guard::all_ok(l)
        })
}

pub(crate) proof fn lemma_at_site_log_policy_ok(
    b: Seq<u8>,
    p: Policy,
    m: crate::model::MHModel,
    l: Seq<(AllocSite, nat, bool)>,
)
    requires
        crate::policy_model::mh_policy(b, p) == Ok(m),
        l == Seq::<(AllocSite, nat, bool)>::empty().push(
            (AllocSite::Output, 4 * m.n as nat, true)),
    ensures at_site_log_policy(b, p, false, l),
{
    reveal(at_site_log_policy);
    reveal(req_tga_policy);
    reveal(crate::alloc_guard::req_of_log);
    reveal(crate::alloc_guard::all_ok);
    assert(crate::alloc_guard::req_of_log(l) =~= req_tga_policy(b, p));
}

pub(crate) proof fn lemma_at_site_log_policy_oom(
    b: Seq<u8>,
    p: Policy,
    m: crate::model::MHModel,
    l: Seq<(AllocSite, nat, bool)>,
)
    requires
        crate::policy_model::mh_policy(b, p) == Ok(m),
        l == Seq::<(AllocSite, nat, bool)>::empty().push(
            (AllocSite::Output, 4 * m.n as nat, false)),
    ensures at_site_log_policy(b, p, true, l),
{
    reveal(at_site_log_policy);
    reveal(req_tga_policy);
    reveal(crate::alloc_guard::req_of_log);
    reveal(crate::alloc_guard::all_ok);
    assert(crate::alloc_guard::req_of_log(l) =~= req_tga_policy(b, p));
}

pub(crate) proof fn lemma_at_site_log_policy_prealloc(
    b: Seq<u8>,
    p: Policy,
    l: Seq<(AllocSite, nat, bool)>,
)
    requires
        l.len() == 0,
        crate::policy_model::mh_policy(b, p) is Err,
    ensures at_site_log_policy(b, p, false, l),
{
    reveal(at_site_log_policy);
    reveal(req_tga_policy);
    reveal(crate::alloc_guard::req_of_log);
    reveal(crate::alloc_guard::all_ok);
    assert(crate::alloc_guard::req_of_log(l) =~= req_tga_policy(b, p));
}

pub(crate) proof fn lemma_req_tga_policy_clause3(b: Seq<u8>, p: Policy)
    ensures forall|j: int| 0 <= j < req_tga_policy(b, p).len()
        ==> req_tga_policy(b, p)[j].1 <= crate::alloc_guard::MAX_BYTES,
{
    reveal(req_tga_policy);
    if req_tga_policy(b, p).len() > 0 {
        let m = choose|m: crate::model::MHModel| crate::policy_model::mh_policy(b, p) == Ok(m);
        assert(crate::policy_model::mh_policy(b, p) == Ok(m));
        crate::policy_model::lemma_mh_policy_ok_bounds(b, m, p);
        reveal(crate::model::mh_bounds);
        assert(req_tga_policy(b, p)[0].1 == 4 * m.n as nat);
        assert(m.n <= 16777216);
        assert(4 * m.n <= 67108864);
    }
}

/// `Req_TGA_policy`'s single element, when it exists, is `(Output, 4N)` with
/// `N = policy_checked_n`.
pub(crate) proof fn lemma_req_tga_policy_elem(b: Seq<u8>, p: Policy)
    ensures req_tga_policy(b, p).len() == 1
        ==> req_tga_policy(b, p)[0].1 == 4 * policy_checked_n(b, p) as nat,
{
    reveal(req_tga_policy);
    reveal(policy_checked_n);
}

pub(crate) proof fn lemma_req_tga_policy_len(b: Seq<u8>, p: Policy)
    ensures req_tga_policy(b, p).len() <= 1,
{
    reveal(req_tga_policy);
}

/// Clause 1 of the policy path's `G`: a non-`OutOfMemory` result satisfies the
/// policy `mp_policy` on `crate::alloc_guard::sites::fmt_part(r)`, has exactly
/// `Req_TGA_policy`'s requests and every entry `Ok`. This is the policy twin of
/// `g_clause1`'s `decode_eq` disjunct (`policy_result` is `decode_eq`'s policy
/// analogue). The `recovered` component is not part of this clause: the frozen
/// `Eq_f` for TGA is `decode_eq`/`mp_policy` alone, and the flag is stated
/// separately as `r is Ok ==> r.1 == recovered_policy(b, p)` together with
/// `r is Err ==> !r.1`, on the entry points and in `pre_alloc_policy_concl`.
pub(crate) open spec fn g_policy_clause1(
    b: Seq<u8>,
    p: Policy,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    match r {
        Err(TryError::OutOfMemory(_)) => true,
        _ => policy_result(crate::alloc_guard::sites::fmt_part(r)) == crate::policy_model::mp_policy(b, p)
            && crate::alloc_guard::req_of_log(l) == req_tga_policy(b, p)
            && crate::alloc_guard::all_ok(l),
    }
}

/// Clause 2 of the policy path's `G`.
pub(crate) open spec fn g_policy_clause2(
    b: Seq<u8>,
    p: Policy,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    match r {
        Err(TryError::OutOfMemory(o)) =>
            l.len() >= 1
            && crate::alloc_guard::req_of_log(l).is_prefix_of(req_tga_policy(b, p))
            && (forall|j: int| 0 <= j < l.len() - 1 ==> l[j].2)
            && !l[l.len() - 1].2
            && l[l.len() - 1].0 == o.site_spec()
            && l[l.len() - 1].1 == o.elements_spec() as nat,
        _ => true,
    }
}

pub(crate) open spec fn g_policy_clause3(b: Seq<u8>, p: Policy) -> bool {
    forall|j: int| 0 <= j < req_tga_policy(b, p).len()
        ==> req_tga_policy(b, p)[j].1 <= crate::alloc_guard::MAX_BYTES
}

/// Theorem shape `G` for the policy entry point.
pub(crate) open spec fn g_policy(
    b: Seq<u8>,
    p: Policy,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    g_policy_clause1(b, p, r, l) && g_policy_clause2(b, p, r, l) && g_policy_clause3(b, p)
}

pub(crate) open spec fn prog_policy(
    b: Seq<u8>,
    p: Policy,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    crate::alloc_guard::all_ok(l) ==> prog_policy_concl(b, p, r, l)
}

/// The `Prog` conclusion of the policy path, split out so `prog_policy` reveals
/// to the solver as `all_ok(l) ==> prog_policy_concl(..)`. Like `prog_concl` it
/// carries the functional relation, here `mp_policy` on
/// `crate::alloc_guard::sites::fmt_part(r)`, alongside the request equality.
pub(crate) open spec fn prog_policy_concl(
    b: Seq<u8>,
    p: Policy,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    !matches!(r, Err(TryError::OutOfMemory(_)))
        && policy_result(crate::alloc_guard::sites::fmt_part(r)) == crate::policy_model::mp_policy(b, p)
        && crate::alloc_guard::req_of_log(l) == req_tga_policy(b, p)
}

/// The policy path's `PreAlloc` corollary over the returned log:
/// `Req_TGA_policy` empty means the header was rejected before any reservation,
/// so the log is empty, the result is a format error satisfying `mp_policy`, and
/// `recovered` is false. The policy twin of `pre_alloc`, strengthened with the
/// functional relation and the flag component of `g_policy_clause1`. Its
/// `r.1 == false` is `r.0 is Err ==> !r.1` for the only non-`Ok` outcome here.
pub(crate) open spec fn pre_alloc_policy_concl(
    b: Seq<u8>,
    p: Policy,
    r: (Result<Decoded, TryError>, bool),
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    req_tga_policy(b, p).len() == 0
        ==> l.len() == 0
            && !matches!(r.0, Err(TryError::OutOfMemory(_)))
            && policy_result(crate::alloc_guard::sites::fmt_part(r.0)) == crate::policy_model::mp_policy(b, p)
            && !r.1
}

/// The policy path's `G` and `Prog`, derived from `at_site_log_policy` and the
/// helper's own `mp_policy` postcondition. The `OutOfMemory` naming clause is
/// the helper's, so `lemma_req_tga_policy_len` supplies the prefix clause.
/// `r.1 == recovered_policy(b, p)` on `Ok`, `false` on `Format`, carries the
/// flag component of `g_policy_clause1`. The `PreAlloc` corollary
/// `pre_alloc_policy_concl` is a separate `ensures` from the same hypotheses.
pub(crate) proof fn lemma_g_policy_of_shape(
    b: Seq<u8>,
    p: Policy,
    r: (Result<Decoded, TryError>, bool),
    l: Seq<(AllocSite, nat, bool)>,
)
    requires
        at_site_log_policy(b, p, matches!(r.0, Err(TryError::OutOfMemory(_))), l),
        !matches!(r.0, Err(TryError::OutOfMemory(_))) ==>
            policy_result(crate::alloc_guard::sites::fmt_part(r.0)) == crate::policy_model::mp_policy(b, p),
        match r.0 {
            Ok(_) => r.1 == crate::policy_model::recovered_policy(b, p),
            Err(_) => !r.1,
        },
        match r.0 {
            Err(TryError::OutOfMemory(o)) =>
                (o.site_spec(), o.elements_spec() as nat) == req_tga_policy(b, p)[0],
            _ => true,
        },
    ensures
        g_policy(b, p, r.0, l),
        prog_policy(b, p, r.0, l),
        pre_alloc_policy_concl(b, p, r, l),
{
    reveal(g_policy);
    reveal(g_policy_clause1);
    reveal(g_policy_clause2);
    reveal(g_policy_clause3);
    reveal(prog_policy);
    reveal(prog_policy_concl);
    reveal(pre_alloc_policy_concl);
    reveal(at_site_log_policy);
    reveal(req_tga_policy);
    reveal(crate::alloc_guard::req_of_log);
    reveal(crate::alloc_guard::all_ok);
    lemma_req_tga_policy_len(b, p);
    lemma_req_tga_policy_clause3(b, p);
    match r.0 {
        Err(TryError::OutOfMemory(o)) => {
            assert(crate::alloc_guard::req_of_log(l).is_prefix_of(req_tga_policy(b, p)));
            assert(crate::alloc_guard::req_of_log(l)[0] == (o.site_spec(), o.elements_spec() as nat));
            assert(l[0].0 == o.site_spec());
            assert(l[0].1 == o.elements_spec() as nat);
            assert(l.len() - 1 == 0);
        },
        _ => {
            // `PreAlloc` corollary: an empty request set means no reservation, so
            // an empty log, a non-`OutOfMemory` result and `recovered == false`.
            if req_tga_policy(b, p).len() == 0 {
                assert(crate::alloc_guard::req_of_log(l).len() == 0);
                assert(l.len() == 0);
            }
        },
    }
}

/// The `#[inline(never)]` guarded output reservation: `try_zeroed` on an empty
/// log, `4N` zero bytes, the sole allocation site. Replaces the `vec![0u8; 4n]`
/// of `alloc_output`.
///
/// The `4N` initialisation is charged to `steps` only on the `Ok` arm, when the
/// reservation succeeded and the zeroing loop actually ran. A failed
/// reservation did no allocator work and zeroed nothing, so its charge is zero
/// beyond whatever was spent before the call: the counter is left unchanged.
fn alloc_output_guarded(
    n: usize,
    steps: &mut Ghost<nat>,
    Tracked(log): Tracked<&mut AllocLog>,
) -> (r: Result<Vec<u8>, OutOfMemory>)
    requires 1 <= n <= 16777216,
    ensures
        match r {
            Ok(out) => {
                &&& final(steps)@ == old(steps)@ + 4 * n
                &&& out@.len() == 4 * n
            },
            Err(o) => {
                &&& final(steps)@ == old(steps)@
                &&& o.elements_spec() == 4 * n && o.site_spec() == AllocSite::Output
            },
        },
        final(log).view() == old(log).view().push((AllocSite::Output, 4 * n as nat, r is Ok)),
{
    proof {
        assert(n * 1 <= crate::alloc_guard::MAX_BYTES) by (nonlinear_arith)
            requires n <= 16777216;
    }
    match crate::alloc_guard::try_zeroed(AllocSite::Output, 4 * n, Tracked(log)) {
        Ok(v) => {
            *steps = Ghost(steps@ + (4 * n) as nat);
            Ok(v)
        },
        Err(o) => Err(o),
    }
}

#[doc(hidden)]
pub open spec fn policy_result(r: Result<Decoded, Error>) -> Result<(int, int, Seq<u8>), Error> {
    match r { Ok(d) => Ok((d.width as int, d.height as int, d.rgba@)), Err(e) => Err(e) }
}

/// `N` from the 1.0 header model: `mh`'s record on success, else `0`.
#[doc(hidden)]
pub open spec fn model_checked_n(b: Seq<u8>) -> int {
    match crate::model::mh(b) { Ok(m) => m.n, Err(_) => 0 }
}


/// The policy path's `PreAlloc` corollary: `Req_TGA_policy` empty means the
/// header was rejected before any reservation.
pub(crate) open spec fn pre_alloc_policy(b: Seq<u8>, p: Policy) -> bool {
    req_tga_policy(b, p).len() == 0
}

#[doc(hidden)]
pub open spec fn policy_checked_n(b: Seq<u8>, p: Policy) -> int {
    match crate::policy_model::mh_policy(b, p) { Ok(m) => m.n, Err(_) => 0 }
}

#[doc(hidden)]
pub open spec fn decoded_bounds(r: Result<Decoded, Error>) -> bool {
    match r {
        Ok(d) => 1 <= d.width as int <= 8192 && 1 <= d.height as int <= 8192
            && d.rgba@.len() == 4 * (d.width as int * d.height as int) && d.rgba@.len() <= 67108864,
        Err(_) => true,
    }
}

fn map_bytes_policy_exec(h: &Header, p: Policy) -> (r: usize)
    requires header_inv_policy(*h, p),
    ensures r as int == crate::policy_model::map_bytes_flags(h.color_map_flag, h.map_length as int, h.map_entry_depth, p), r <= 262140,
{
    if !flag_present_hdr_exec(h.color_map_flag, p) { return 0; }
    if h.map_length == 0 {
        proof {
            assert(h.map_length as int * crate::model::sample_bytes_of(h.map_entry_depth) == 0) by (nonlinear_arith)
                requires h.map_length == 0;
            reveal(crate::policy_model::map_bytes_flags);
        }
        return 0;
    }
    let stride = h.map_bytes_per_entry();
    proof {
        reveal(header_inv_policy);
        crate::policy_model::lemma_sample_bytes_of_stride(h.map_entry_depth);
        crate::policy_model::lemma_map_entry_depth_stride(h.map_entry_depth);
        assert(h.map_length as int * stride as int <= 262140) by (nonlinear_arith)
            requires h.map_length as int <= 65535, 0 <= stride <= 4;
    }
    h.map_length as usize * stride
}

fn suffix_policy_exec(bytes: &[u8], cursor: usize, p: Policy) -> (ok: bool)
    requires cursor <= bytes@.len(), bytes@.len() >= 18,
    ensures ok == crate::policy_model::mp_suffix_ok_policy(bytes@, p, cursor as int),
{
    if matches!(p.trailing, crate::policy::TrailingOptions::Ignore) { return true; }
    if cursor == bytes.len() { return true; }
    if bytes.len() - cursor < 26 { return false; }
    let ok = signature_tail(bytes);
    proof {
        crate::model::lemma_suffix_tail(bytes@, cursor as int);
        reveal(crate::model::mp_suffix_ok); reveal(crate::model::footer);
    }
    ok
}

/// Pixel decoding and suffix validation, after the checked ID and map slices.
fn finish_policy(bytes: &[u8], h: &Header, map: &[u8], mut out: Vec<u8>, c0: usize,
    n: usize, p: Policy, steps: &mut Ghost<nat>) -> (r: (Result<Decoded, Error>, bool))
    requires
        header_inv_policy(*h, p),
        crate::policy_model::mh_policy(bytes@, p) == Ok(mh_of_header(*h, p)),
        18 <= bytes@.len() <= crate::model::max_input(),
        n == h.width as int * h.height as int, out@.len() == 4 * n,
        c0 as int == 18 + h.id_length as int + mh_of_header(*h, p).map_bytes,
        18 + h.id_length as int <= c0 <= bytes@.len(),
        map@ == bytes@.subrange(18 + h.id_length as int, c0 as int),
    ensures
        policy_result(r.0) == crate::policy_model::mp_after_policy(bytes@, mh_of_header(*h, p), p),
        r.0 is Ok ==> r.1 == crate::policy_model::recovered_policy(bytes@, p),
        r.0 is Err ==> !r.1,
        decoded_bounds(r.0),
        final(steps)@ as int <= old(steps)@ as int + 7 * n + 20,
{
    let ghost m = mh_of_header(*h, p);
    let ghost pkt = pkt_of_policy(h, p);
    proof {
        reveal(header_inv_policy); reveal(mh_of_header);
        lemma_pkt_of_policy_eq(h, p, m);
        crate::policy_model::lemma_sample_bytes_of_stride(h.pixel_depth);
        crate::policy_model::lemma_policy_phase(bytes@, m, p);
    }
    let indexed = kind_hdr_exec(h.image_type, p) == 0;
    let rle = h.image_type == 9 || h.image_type == 10 || h.image_type == 11;
    let mut cursor = c0;
    let mut produced = 0usize;
    let mut clip = false;
    let mut subst = false;
    let result = if rle {
        let result = decode_rle_policy(bytes, h, map, indexed, &mut out, &mut cursor, &mut produced,
            n, c0, 0, p, steps, &mut clip, &mut subst);
        proof {
            reveal(crate::policy_model::policy_phase);
            if result is Ok {
                let px = crate::policy_model::mr_pix_policy(bytes@, pkt, map@, p, c0 as int, 0);
                crate::model::lemma_wrote_px_off_full(out@, px, h.width as int, h.height as int,
                    h.spec_flip_x(), h.spec_flip_y(), 0);
                crate::model::lemma_wrote_px_off_zero_is_quad_ok(out@, px, h.width as int, h.height as int,
                    h.spec_flip_x(), h.spec_flip_y());
                crate::policy_model::lemma_rle_quad_to_orient_policy(out@,
                    crate::policy_model::mr_pix_policy(bytes@, pkt, map@, p, c0 as int, 0),
                    h.width as int, h.height as int, h.spec_flip_x(), h.spec_flip_y());
            }
        }
        result
    } else {
        let result = decode_raw_policy(bytes, h, map, indexed, &mut out, &mut cursor, n, c0, p, steps, &mut subst);
        proof {
            reveal(raw_result_eq_policy); reveal(crate::policy_model::policy_phase);
            if result is Ok {
                crate::policy_model::lemma_wrote_to_orient_policy(out@, bytes@, pkt, map@, p,
                    h.width as int, h.height as int, h.spec_flip_x(), h.spec_flip_y(), c0 as int);
            }
        }
        result
    };
    match result { Err(e) => { return (Err(e), false); }, Ok(()) => {} }
    proof { *steps = Ghost(steps@ + 18); }
    if !suffix_policy_exec(bytes, cursor, p) { return (Err(Error::TrailingBytes), false); }
    proof {
        reveal(crate::policy_model::mp_assemble_policy);
        reveal(crate::policy_model::mp_pixels_policy);
        reveal(crate::policy_model::recovered_policy);
    }
    (Ok(Decoded { width: h.width, height: h.height, rgba: out }), clip || subst)
}

/// Header-accepted path; every allocation follows the full header checks. The
/// one reservation is the guarded output; on `OutOfMemory` the call returns
/// `Err(TryError::OutOfMemory)`, and nothing after the reservation runs.
fn decode_complete_policy(bytes: &[u8], h: Header, p: Policy, steps: &mut Ghost<nat>,
    Tracked(log): Tracked<&mut AllocLog>)
    -> (r: (Result<Decoded, TryError>, bool))
    requires
        18 <= bytes@.len() <= crate::model::max_input(), header_inv_policy(h, p),
        crate::policy_model::mh_policy(bytes@, p) == Ok(mh_of_header(h, p)),
        old(log).view() == Seq::<(AllocSite, nat, bool)>::empty(),
    ensures
        // Clause 1 for a non-`OutOfMemory` result, clause 2 for one.
        !matches!(r.0, Err(TryError::OutOfMemory(_))) ==>
            policy_result(crate::alloc_guard::sites::fmt_part(r.0)) == crate::policy_model::mp_after_policy(bytes@, mh_of_header(h, p), p)
            && crate::alloc_guard::req_of_log(final(log).view()) == req_tga_policy(bytes@, p)
            && crate::alloc_guard::all_ok(final(log).view()),
        match r.0 {
            Err(TryError::OutOfMemory(o)) =>
                final(log).view().len() == 1 && !final(log).view()[0].2
                && final(log).view()[0].0 == o.site_spec()
                && final(log).view()[0].1 == o.elements_spec() as nat
                && crate::alloc_guard::req_of_log(final(log).view()) == req_tga_policy(bytes@, p)
                && r.1 == false,
            _ => true,
        },
        r.0 is Ok ==> r.1 == crate::policy_model::recovered_policy(bytes@, p),
        r.0 is Err ==> !r.1,
        !matches!(r.0, Err(TryError::OutOfMemory(_))) ==> decoded_bounds(crate::alloc_guard::sites::fmt_part(r.0)),
        final(steps)@ as int <= old(steps)@ as int + 11 * mh_of_header(h, p).n + 20,
{
    proof {
        reveal(header_inv_policy);
        assert(1 <= h.width as int * h.height as int <= 16777216) by (nonlinear_arith)
            requires 1 <= h.width as int <= 8192, 1 <= h.height as int <= 8192,
                4 * (h.width as int * h.height as int) <= 67108864;
    }
    let n = h.width as usize * h.height as usize;
    let ghost m0 = mh_of_header(h, p);
    let out = match alloc_output_guarded(n, steps, Tracked(log)) {
        Ok(o) => o,
        Err(o) => {
            proof {
                reveal(req_tga_policy);
                reveal(header_inv_policy);
                assert(crate::policy_model::mh_policy(bytes@, p) == Ok(m0));
                lemma_at_site_log_policy_oom(bytes@, p, m0, log.view());
                assert(crate::alloc_guard::req_of_log(log.view()) == req_tga_policy(bytes@, p));
                assert(log.view().len() == 1 && !log.view()[0].2);
                assert((o.site_spec(), o.elements_spec() as nat) == req_tga_policy(bytes@, p)[0]);
            }
            return (Err(TryError::OutOfMemory(o)), false);
        },
    };
    proof {
        reveal(req_tga_policy);
        lemma_at_site_log_policy_ok(bytes@, p, m0, log.view());
        assert(out@.len() == 4 * n);
    }
    let id_end = 18usize + h.id_length as usize;
    if id_end > bytes.len() {
        proof { reveal(crate::policy_model::mp_after_policy); }
        return (Err(TryError::Format(Error::TruncatedImageId)), false);
    }
    let map_bytes = map_bytes_policy_exec(&h, p);
    let map_end = id_end + map_bytes;
    if map_end > bytes.len() {
        proof { reveal(crate::policy_model::mp_after_policy); }
        return (Err(TryError::Format(Error::TruncatedColorMap)), false);
    }
    let map = &bytes[id_end..map_end];
    let (r, flag) = finish_policy(bytes, &h, map, out, map_end, n, p, steps);
    proof {
        reveal(crate::alloc_guard::sites::fmt_part);
        reveal(crate::alloc_guard::req_of_log);
        reveal(crate::alloc_guard::all_ok);
        if r is Err {
            assert(crate::policy_model::mh_policy(bytes@, p) == Ok(m0));
        }
    }
    (match r { Ok(d) => Ok(d), Err(e) => Err(TryError::Format(e)) }, flag)
}

/// Charged units: header bytes inspected (capped at 18), 4N zeroed output bytes,
/// one sample attempt plus four output-byte writes, one RLE dispatch per packet,
/// and at most 18 suffix-signature bytes. Fixed-size scalar work belongs to the
/// corresponding attempt or dispatch; discarded packet positions have no charge.
fn decode_policy_tracked(bytes: &[u8], p: Policy, steps: &mut Ghost<nat>,
    Tracked(log): Tracked<&mut AllocLog>)
    -> (r: (Result<Decoded, TryError>, bool))
    requires
        old(log).view() == Seq::<(AllocSite, nat, bool)>::empty(),
    ensures
        !matches!(r.0, Err(TryError::OutOfMemory(_))) ==>
            policy_result(crate::alloc_guard::sites::fmt_part(r.0)) == crate::policy_model::mp_policy(bytes@, p)
            && at_site_log_policy(bytes@, p, false, final(log).view()),
        match r.0 {
            Err(TryError::OutOfMemory(o)) =>
                at_site_log_policy(bytes@, p, true, final(log).view())
                && (o.site_spec(), o.elements_spec() as nat) == req_tga_policy(bytes@, p)[0]
                && r.1 == false,
            _ => true,
        },
        r.0 is Ok ==> r.1 == crate::policy_model::recovered_policy(bytes@, p),
        r.0 is Err ==> !r.1,
        !matches!(r.0, Err(TryError::OutOfMemory(_))) ==> decoded_bounds(crate::alloc_guard::sites::fmt_part(r.0)),
        // The policy `PreAlloc` corollary over the returned log: a rejected
        // header reserves nothing and returns a format error with `recovered`
        // false and the exact `mp_policy` result.
        pre_alloc_policy_concl(bytes@, p, r, final(log).view()),
        final(steps)@ as int <= old(steps)@ as int + 16 * (bytes@.len() + policy_checked_n(bytes@, p)),
{
    proof { *steps = Ghost(steps@ + if bytes@.len() < 18 { bytes@.len() } else { 18nat }); }
    // `PreAlloc`: a header error reserves nothing.
    proof {
        reveal(pre_alloc_policy);
        reveal(req_tga_policy);
        reveal(at_site_log_policy);
        reveal(crate::alloc_guard::req_of_log);
        reveal(crate::alloc_guard::all_ok);
    }
    let h = match parse_header_policy(bytes, p) {
        Ok(h) => h,
        Err(e) => {
            proof {
                reveal(crate::policy_model::mp_policy);
                reveal(pre_alloc_policy_concl);
                reveal(policy_checked_n);
                reveal(req_tga_policy);
                reveal(at_site_log_policy);
                reveal(crate::alloc_guard::req_of_log);
                reveal(crate::alloc_guard::all_ok);
                reveal(crate::alloc_guard::sites::fmt_part);
                lemma_at_site_log_policy_prealloc(bytes@, p, log.view());
                // `mp_policy`'s header phase: the same first error.
                assert(crate::policy_model::mh_policy(bytes@, p) == Err(e));
                assert(crate::policy_model::mp_policy(bytes@, p) == Err(e));
            }
            return (Err(TryError::Format(e)), false);
        },
    };
    let result = decode_complete_policy(bytes, h, p, steps, Tracked(log));
    proof { reveal(crate::policy_model::mp_policy); reveal(policy_checked_n); }
    result
}

/// The logged policy entry point: creates the one log, threads it through
/// `decode_policy_tracked`, and returns the result with the log. Proves the
/// policy path's `G`/`Prog` from the helper's `at_site_log_policy`.
pub(crate) fn try_decode_policy_tracked(bytes: &[u8], p: Policy, steps: &mut Ghost<nat>)
    -> (r: (Result<Decoded, TryError>, bool, Tracked<AllocLog>))
    ensures
        !matches!(r.0, Err(TryError::OutOfMemory(_))) ==>
            policy_result(crate::alloc_guard::sites::fmt_part(r.0)) == crate::policy_model::mp_policy(bytes@, p)
            && decoded_bounds(crate::alloc_guard::sites::fmt_part(r.0)),
        g_policy(bytes@, p, r.0, r.2@.view()),
        prog_policy(bytes@, p, r.0, r.2@.view()),
        pre_alloc_policy_concl(bytes@, p, (r.0, r.1), r.2@.view()),
        match r.0 {
            Err(TryError::OutOfMemory(o)) => r.1 == false
                && (o.site_spec(), o.elements_spec() as nat) == req_tga_policy(bytes@, p)[0],
            Ok(_) => r.1 == crate::policy_model::recovered_policy(bytes@, p),
            Err(TryError::Format(_)) => !r.1,
        },
        final(steps)@ as int <= old(steps)@ as int + 16 * (bytes@.len() + policy_checked_n(bytes@, p)),
{
    let tracked mut log = crate::alloc_guard::new_log();
    let result = decode_policy_tracked(bytes, p, steps, Tracked(&mut log));
    proof {
        reveal(g_policy);
        reveal(g_policy_clause1);
        reveal(g_policy_clause2);
        reveal(g_policy_clause3);
        reveal(prog_policy);
        reveal(prog_policy_concl);
        reveal(pre_alloc_policy_concl);
        reveal(req_tga_policy);
        reveal(crate::alloc_guard::sites::fmt_part);
        reveal(crate::alloc_guard::req_of_log);
        reveal(crate::alloc_guard::all_ok);
        lemma_req_tga_policy_len(bytes@, p);
        lemma_req_tga_policy_clause3(bytes@, p);
        lemma_g_policy_of_shape(bytes@, p, result, log.view());
    }
    (result.0, result.1, Tracked(log))
}

/// The public policy decoder for named profiles: `try_decode_policy_tracked`
/// with the log discarded. Its result relation is `mp_policy` on `crate::alloc_guard::sites::fmt_part(r.0)`
/// for every non-`OutOfMemory` result, and `OutOfMemory` names the failed
/// `(Output, 4N)` reservation.
pub fn try_decode_with_policy(bytes: &[u8], profile: crate::policy::Profile)
    -> (r: (Result<Decoded, TryError>, bool))
    ensures
        !matches!(r.0, Err(TryError::OutOfMemory(_))) ==>
            policy_result(crate::alloc_guard::sites::fmt_part(r.0)) == crate::policy_model::mp_policy(bytes@, crate::policy::profile_policy(profile))
            && decoded_bounds(crate::alloc_guard::sites::fmt_part(r.0)),
        match r.0 {
            Err(TryError::OutOfMemory(o)) => r.1 == false
                && o.site_spec() == AllocSite::Output
                && o.elements_spec() as int == 4 * policy_checked_n(bytes@, crate::policy::profile_policy(profile)),
            Ok(_) => r.1 == crate::policy_model::recovered_policy(bytes@, crate::policy::profile_policy(profile)),
            Err(TryError::Format(_)) => !r.1,
        },
{
    let p = crate::policy::profile_policy_exec(profile);
    let mut steps = Ghost(0nat);
    let r = try_decode_policy_tracked(bytes, p, &mut steps);
    proof {
        reveal(crate::policy::profile_policy);
        reveal(crate::alloc_guard::sites::fmt_part);
        reveal(policy_checked_n);
        reveal(req_tga_policy);
        if let Err(TryError::OutOfMemory(o)) = r.0 {
            assert(policy_checked_n(bytes@, p)
                == policy_checked_n(bytes@, crate::policy::profile_policy(profile)));
            lemma_req_tga_policy_elem(bytes@, p);
            assert(req_tga_policy(bytes@, p).len() == 1);
            assert(o.elements_spec() as int
                == req_tga_policy(bytes@, p)[0].1);
            assert(o.site_spec() == AllocSite::Output);
        }
    }
    (r.0, r.1)
}

/// Public executable witness that Strict is exactly the 1.0 decoder: for
/// every input the two calls produce the same error or identical dimensions
/// and RGBA bytes, and Strict never reports recovery.
pub(crate) fn t7_strict_is_1_0(bytes: &[u8])
    -> (r: ((Result<Decoded, TryError>, bool), Result<Decoded, TryError>))
    ensures
        r.0.1 == false,
        // Agreement on every outcome: `OutOfMemory` is the sole case where the
        // two independent reservations may differ, and even then each names its
        // own failed `(Output, 4N)`.
        match (r.0.0, r.1) {
            (Err(TryError::OutOfMemory(o1)), Err(TryError::OutOfMemory(o2))) =>
                o1.site_spec() == AllocSite::Output && o2.site_spec() == AllocSite::Output
                && o1.elements_spec() == o2.elements_spec(),
            (Err(TryError::OutOfMemory(_)), _) => true,
            (_, Err(TryError::OutOfMemory(_))) => true,
            _ => policy_result(crate::alloc_guard::sites::fmt_part(r.0.0)) == policy_result(crate::alloc_guard::sites::fmt_part(r.1)),
        },
        match (r.0.0, r.1) {
            (Err(TryError::OutOfMemory(_)), _) => true,
            (_, Err(TryError::OutOfMemory(_))) => true,
            (Ok(strict), Ok(original)) =>
                strict.width == original.width && strict.height == original.height
                    && strict.rgba@ == original.rgba@,
            (Err(TryError::Format(e1)), Err(TryError::Format(e2))) => e1 == e2,
            _ => false,
        },
        !matches!(r.1, Err(TryError::OutOfMemory(_))) ==> decode_eq(crate::alloc_guard::sites::fmt_part(r.1), bytes@),
{
    let original = try_decode(bytes);
    let strict = try_decode_with_policy(bytes, crate::policy::Profile::Strict);
    proof {
        crate::policy_model::lemma_mp_policy_strict_is_mp(bytes@);
        crate::policy_model::lemma_recovered_policy_strict(bytes@);
        reveal(crate::policy::profile_policy);
        reveal(policy_result);
        reveal(decode_eq);
        reveal(crate::alloc_guard::sites::fmt_part);
        reveal(crate::policy_model::mp_policy);
        reveal(crate::model::mp);
    }
    (strict, original)
}

fn t2_t5_policy_header(b: &[u8], p: Policy) -> (r: Result<Header, Error>)
    ensures
        match crate::policy_model::mh_policy(b@, p) {
            Ok(m) => match r {
                Ok(h) => m == mh_of_header(h, p),
                Err(_) => false,
            },
            Err(e2) => match r {
                Ok(_) => false,
                Err(e) => e == e2,
            },
        },
        match r {
            Ok(h) => 18 <= b@.len() <= crate::model::max_input() && header_inv_policy(h, p),
            Err(_) => true,
        },
{ parse_header_policy(b, p) }

fn t4_policy_conversion(h: &Header, map: &[u8], indexed: bool, s: &[u8], p: Policy, subst: &mut bool)
    -> (r: Result<[u8; 4], Error>)
    requires
        crate::policy_model::type_ok_policy(h.image_type, p),
        indexed == (crate::policy_model::kind_policy(h.image_type, p) == 0),
        crate::policy_model::depth_ok_policy(
            crate::policy_model::kind_policy(h.image_type, p),
            h.pixel_depth,
            p.depths,
        ),
        indexed ==> crate::policy_model::map_present_policy_flags(h.color_map_flag, p),
        indexed ==> (h.map_length as int) > 0 ==> crate::model::map_entry_depth_ok(
            h.map_entry_depth,
        ),
        indexed ==> (h.spec_sample_bytes() == 1 || h.spec_sample_bytes() == 2),
        !indexed ==> 1 <= h.spec_sample_bytes() <= 4,
        !indexed ==> h.spec_sample_bytes() == crate::model::stride_of(h.pixel_depth as int),
        s@.len() == h.spec_sample_bytes(),
        s@.len() >= 1,
        map@.len() == crate::policy_model::map_bytes_flags(
            h.color_map_flag,
            h.map_length as int,
            h.map_entry_depth,
            p,
        ),
        h.map_length as int <= 65535,
    ensures
        *final(subst) == (*old(subst) || (indexed && crate::policy_model::pkt_subst(
            pkt_of_policy(h, p), crate::model::sindex(s@, h.spec_sample_bytes()), p))),
        match r {
            Ok(px) => px@.len() == 4,
            Err(_) => true,
        },
        match r {
            Ok(px) => px@ == crate::policy_model::conv2_policy(
                pkt_of_policy(h, p),
                map@,
                s@,
                p,
            ),
            Err(_) => true,
        },
        match r {
            Ok(_) => !indexed || crate::policy_model::in_range_policy(
                pkt_of_policy(h, p),
                crate::model::sindex(s@, h.spec_sample_bytes()),
                p,
            ) || p.index_range == IndexRangeOptions::Black,
            Err(e) => indexed && !crate::policy_model::in_range_policy(
                pkt_of_policy(h, p),
                crate::model::sindex(s@, h.spec_sample_bytes()),
                p,
            ) && p.index_range == IndexRangeOptions::Reject && e == Error::ColorIndexOutOfRange,
        },
{ convert_one_policy(h, map, indexed, s, p, subst) }

fn t3_t6_policy_packets(
    bytes: &[u8], h: &Header, map: &[u8], indexed: bool,
    out: &mut Vec<u8>, cursor: &mut usize, produced: &mut usize,
    n: usize, c0g: usize, p0g: usize, p: Policy,
    steps: &mut Ghost<nat>,
    clip: &mut bool, subst: &mut bool,
) -> (r: Result<(), Error>)
    requires
        header_inv_policy(*h, p),
        n == h.width as int * h.height as int,
        indexed == (crate::policy_model::kind_policy(h.image_type, p) == 0),
        indexed ==> crate::policy_model::map_present_policy_flags(h.color_map_flag, p),
        old(out)@.len() == 4 * n,
        *old(cursor) == c0g, c0g <= bytes@.len(),
        *old(produced) == p0g, p0g <= n,
        bytes@.len() <= crate::model::max_input(),
        map@.len() == crate::policy_model::map_bytes_flags(h.color_map_flag, h.map_length as int, h.map_entry_depth, p),
        map@.len() <= 262140,
    ensures
        match crate::policy_model::mr_policy(bytes@, pkt_of_policy(h, p), map@, p, c0g as int, p0g as int) {
            Ok((c, pr, px)) => r is Ok && *final(cursor) == c && *final(produced) == pr
                && crate::model::wrote_px_off(final(out)@, px, h.width as int, h.height as int,
                    h.spec_flip_x(), h.spec_flip_y(), p0g as int, px.len() as int),
            Err((e, c, pr, px)) => e == crate::model::MrErr::Api(r->Err_0) && r is Err
                && *final(cursor) == c && *final(produced) == pr
                && crate::model::wrote_px_off(final(out)@, px, h.width as int, h.height as int,
                    h.spec_flip_x(), h.spec_flip_y(), p0g as int, px.len() as int),
        },
        r is Ok ==> *final(clip) == (*old(clip) || crate::policy_model::mr_flags_policy(bytes@, pkt_of_policy(h, p), map@, p, c0g as int, p0g as int).0),
        r is Ok ==> *final(subst) == (*old(subst) || crate::policy_model::mr_flags_policy(bytes@, pkt_of_policy(h, p), map@, p, c0g as int, p0g as int).1),
        final(steps)@ as int <= old(steps)@ as int + 7 * (n as int - p0g as int) + 2,
        final(out)@.len() == 4 * n,
        *final(cursor) <= bytes@.len(),
        p0g <= *final(produced) <= n,
        crate::policy_model::mr_pix_policy(bytes@, pkt_of_policy(h, p), map@, p, c0g as int, p0g as int).len()
            == *final(produced) as int - p0g as int,
        crate::model::wrote_px_off(final(out)@,
            crate::policy_model::mr_pix_policy(bytes@, pkt_of_policy(h, p), map@, p, c0g as int, p0g as int),
            h.width as int, h.height as int, h.spec_flip_x(), h.spec_flip_y(),
            p0g as int, *final(produced) as int - p0g as int),
        match r {
            Ok(()) => *final(produced) == n &&
                crate::policy_model::mr_cur_policy(bytes@, pkt_of_policy(h, p), map@, p, c0g as int, p0g as int)
                    == Ok((*final(cursor) as int, n as int)),
            Err(e) => crate::policy_model::mr_cur_policy(bytes@, pkt_of_policy(h, p), map@, p, c0g as int, p0g as int)
                    == Err(crate::model::MrErr::Api(e)) &&
                crate::policy_model::mr_gcur_policy(bytes@, pkt_of_policy(h, p), map@, p, c0g as int, p0g as int)
                    == Ok(*final(cursor) as int),
        },
{ decode_rle_policy(bytes, h, map, indexed, out, cursor, produced, n, c0g, p0g, p, steps, clip, subst) }

/// Universal safety, output bounds, termination, recovery exactness and work.
/// The counters observe the executable called by the public profile API; the
/// allocation count is the guard log's length (`G` clause 1/2 replaces the old
/// ghost `allocations` counter).
fn t1_t8_t10_w_a_e_policy(bytes: &[u8], p: Policy)
    -> (r: (Result<Decoded, TryError>, bool, Ghost<nat>))
    ensures
        !matches!(r.0, Err(TryError::OutOfMemory(_))) ==>
            decoded_bounds(crate::alloc_guard::sites::fmt_part(r.0))
            && policy_result(crate::alloc_guard::sites::fmt_part(r.0)) == crate::policy_model::mp_policy(bytes@, p),
        r.0 is Ok ==> r.1 == crate::policy_model::recovered_policy(bytes@, p),
        r.0 is Err ==> !r.1,
        match r.0 {
            Err(TryError::OutOfMemory(o)) => o.site_spec() == AllocSite::Output
                && o.elements_spec() as int == 4 * policy_checked_n(bytes@, p),
            _ => true,
        },
        r.2@ as int <= 16 * (bytes@.len() + policy_checked_n(bytes@, p)),
{
    let mut steps = Ghost(0nat);
    let r = try_decode_policy_tracked(bytes, p, &mut steps);
    proof {
        reveal(crate::alloc_guard::sites::fmt_part);
        reveal(policy_checked_n);
        reveal(decoded_bounds);
        reveal(req_tga_policy);
        reveal(crate::alloc_guard::req_of_log);
        reveal(crate::alloc_guard::all_ok);
        reveal(crate::policy_model::mp_policy);
        if let Err(TryError::OutOfMemory(o)) = r.0 {
            assert((o.site_spec(), o.elements_spec() as nat) == req_tga_policy(bytes@, p)[0]);
            assert(o.site_spec() == AllocSite::Output);
        }
    }
    (r.0, r.1, steps)
}

/// T5/T7/T9: exact RGBA, recovery and first error for each named profile.
fn t7_t9_policy_exact(bytes: &[u8], profile: crate::policy::Profile)
    -> (r: (Result<Decoded, TryError>, bool))
    ensures
        !matches!(r.0, Err(TryError::OutOfMemory(_))) ==>
            policy_result(crate::alloc_guard::sites::fmt_part(r.0)) == crate::policy_model::mp_policy(bytes@, crate::policy::profile_policy(profile)),
        r.0 is Ok ==> r.1 == crate::policy_model::recovered_policy(bytes@, crate::policy::profile_policy(profile)),
        r.0 is Err ==> !r.1,
{ try_decode_with_policy(bytes, profile) }

/// R transfers the model's four error-site argument to both executable calls.
fn r_recommended_conservative(bytes: &[u8])
    -> (r: ((Result<Decoded, TryError>, bool), (Result<Decoded, TryError>, bool)))
    ensures
        match (r.0.0, r.1.0) {
            (Ok(strict), Ok(recommended)) =>
                policy_result(crate::alloc_guard::sites::fmt_part(Ok(recommended))) == policy_result(crate::alloc_guard::sites::fmt_part(Ok(strict)))
                && !r.1.1,
            _ => true,
        },
{
    let strict = try_decode_with_policy(bytes, crate::policy::Profile::Strict);
    let recommended = try_decode_with_policy(bytes, crate::policy::Profile::Recommended);
    proof {
        reveal(crate::alloc_guard::sites::fmt_part);
        if let Ok(ref d) = strict.0 {
            crate::policy_model::lemma_mp_policy_recommended_on_strict_ok(bytes@, dec_vals(*d));
        }
    }
    (strict, recommended)
}

/// E's no-event corollary applies to every record with both event sites disabled.
fn e_policy_no_events(bytes: &[u8], p: Policy) -> (r: (Result<Decoded, TryError>, bool))
    requires p.rle_overrun == crate::policy::RleOverrunOptions::Reject,
        p.index_range == crate::policy::IndexRangeOptions::Reject,
    ensures !r.1,
{
    let mut steps = Ghost(0nat);
    let r = try_decode_policy_tracked(bytes, p, &mut steps);
    proof {
        reveal(crate::policy_model::recovered_policy);
        reveal(crate::alloc_guard::sites::fmt_part);
        crate::policy_model::lemma_recovered_policy_reject(bytes@, p);
        match r.0 {
            Err(TryError::OutOfMemory(_)) => { assert(r.1 == false); },
            Ok(_) => {
                assert(r.1 == crate::policy_model::recovered_policy(bytes@, p));
                assert(!r.1);
            },
            Err(TryError::Format(_)) => {},
        }
    }
    (r.0, r.1)
}

/// The custom-record decoder's own contract, stated on a `pub(crate)` function
/// that every build verifies, so the contract counts in the verified total.
/// `try_decode_custom_policy` (`#[cfg(test)]`, below) is the only caller, and it
/// only forwards this contract; the wrapper's own `ensures` is not what the
/// verifier checked, this is.
///
/// The `recovered` equality holds only on `Ok`. On either error variant the
/// flag is `false`, and `false` is not `recovered_policy(bytes, p)` in general: a
/// format error can follow a completed clip or substitution event. The smallest
/// case is a one-pixel true-colour RLE image — `image_type = 10`,
/// `width = height = 1`, `pixel_depth = 24`, `rle_overrun = ClipPacket` —
/// carrying a repeat packet whose count is 2 (one more than `N = 1`) and one
/// trailing byte: the packet clips (setting the clip event), then the suffix
/// check returns `TrailingBytes`. The executable returns
/// `Err(Format(TrailingBytes))` with `recovered == false`, while
/// `recovered_policy` is `true` because the clip event occurred. The exact
/// `mp_policy` result still matches on that arm, which is why the `policy_result`
/// equality keeps both error variants.
#[allow(dead_code)] // called only from `#[cfg(test)]` code, so the lib build sees no caller
pub(crate) fn decode_custom_policy_core(bytes: &[u8], p: Policy)
    -> (r: (Result<Decoded, TryError>, bool))
    ensures
        !matches!(r.0, Err(TryError::OutOfMemory(_))) ==>
            policy_result(crate::alloc_guard::sites::fmt_part(r.0)) == crate::policy_model::mp_policy(bytes@, p),
        r.0 is Ok ==> r.1 == crate::policy_model::recovered_policy(bytes@, p),
        r.0 is Err ==> !r.1,
{
    let mut steps = Ghost(0nat);
    let r = try_decode_policy_tracked(bytes, p, &mut steps);
    proof { reveal(crate::alloc_guard::sites::fmt_part); }
    (r.0, r.1)
}

/// Arbitrary executable records are reachable only in unit tests, through this
/// wrapper over the verified `decode_custom_policy_core`.
#[cfg(test)]
pub(crate) fn try_decode_custom_policy(bytes: &[u8], p: Policy)
    -> (r: (Result<Decoded, TryError>, bool))
    ensures
        !matches!(r.0, Err(TryError::OutOfMemory(_))) ==>
            policy_result(crate::alloc_guard::sites::fmt_part(r.0)) == crate::policy_model::mp_policy(bytes@, p),
        r.0 is Ok ==> r.1 == crate::policy_model::recovered_policy(bytes@, p),
        r.0 is Err ==> !r.1,
{
    decode_custom_policy_core(bytes, p)
}

} // verus!
