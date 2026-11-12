//! Formal reference model for TGA-CORE-SPEC 1.0 (milestone M2).
//!
//! This file is written *independently of the executable code*: every definition
//! below is a transcription of the frozen specification text (R1..R6, R8, R14)
//! in mathematical integers over `Seq<u8>`. Nothing here calls, mirrors the
//! control flow of, or is derived from `crate::tga`. The executable carries
//! postconditions phrased in terms of these definitions (T5); the definitions
//! themselves are never justified by running the executable.
//!
//! M2 needs exactly the total header model `MH` of THEOREMS.md ("Total reference
//! models"), plus the arithmetic lemmas that bound a successful header record.
//! `MP`/`MR` and the pixel models are later milestones.
use vstd::prelude::*;

#[cfg(verus_only)]
use crate::tga::Error;

verus! {

// ---------------------------------------------------------------------------
// R1, R2, R6: the spec's numeric limits, as mathematical integers.
// ---------------------------------------------------------------------------

/// R1: `MAX_INPUT = 67,108,864` bytes.
pub open spec fn max_input() -> int {
    67108864
}

/// R6: `MAX_OUTPUT = 67,108,864` bytes (`4*width*height`).
pub open spec fn max_output() -> int {
    67108864
}

/// R2: `MAX_DIMENSION = 8,192`.
pub open spec fn max_dimension() -> int {
    8192
}

/// R1: `LE16(p) = b[p] + 256*b[p+1]`. Defined (meaningful) once both bytes
/// exist; callers only invoke it under `18 <= |b|`.
pub open spec fn le16(b: Seq<u8>, p: int) -> int {
    b[p] as int + 256 * (b[p + 1] as int)
}

// ---------------------------------------------------------------------------
// R1, R3, R5: type / depth / attribute classification.
// ---------------------------------------------------------------------------

/// R1: the six supported image types `{1,2,3,9,10,11}`.
pub open spec fn type_ok(t: u8) -> bool {
    ||| t == 1u8
    ||| t == 2u8
    ||| t == 3u8
    ||| t == 9u8
    ||| t == 10u8
    ||| t == 11u8
}

/// R1: indexed types 1/9.
pub open spec fn is_indexed(t: u8) -> bool {
    t == 1u8 || t == 9u8
}

/// R3: supported image depths per image type. Indexed 8/16; true-colour
/// 15/16/24/32; greyscale 8/16.
pub open spec fn depth_ok(t: u8, d: u8) -> bool {
    if is_indexed(t) {
        d == 8u8 || d == 16u8
    } else if t == 2u8 || t == 10u8 {
        d == 15u8 || d == 16u8 || d == 24u8 || d == 32u8
    } else {
        d == 8u8 || d == 16u8
    }
}

/// R3: `sample_bytes = ceil(depth/8)`, in natural numbers.
pub open spec fn sample_bytes_of(d: u8) -> int {
    (d as int + 7) / 8
}

/// R4: supported colour-map entry depths `{15,16,24,32}`.
pub open spec fn map_entry_depth_ok(md: u8) -> bool {
    md == 15u8 || md == 16u8 || md == 24u8 || md == 32u8
}

/// R5: the attribute-count policy for a *map entry* depth (indexed images).
pub open spec fn map_attr_ok(md: u8, attr: u8) -> bool {
    if md == 15u8 || md == 24u8 {
        attr == 0u8
    } else if md == 16u8 {
        attr == 0u8 || attr == 1u8
    } else if md == 32u8 {
        attr == 0u8 || attr == 8u8
    } else {
        false
    }
}

/// R5: the descriptor attribute count is `descriptor & 0x0f`. For indexed
/// images the map entry depth governs; otherwise the image depth governs.
pub open spec fn attr_ok(b: Seq<u8>) -> bool {
    let attr = b[17] & 0x0fu8;
    if is_indexed(b[2]) {
        map_attr_ok(b[7], attr)
    } else if b[2] == 2u8 || b[2] == 10u8 {
        if b[16] == 15u8 || b[16] == 24u8 {
            attr == 0u8
        } else if b[16] == 16u8 {
            attr == 0u8 || attr == 1u8
        } else if b[16] == 32u8 {
            attr == 0u8 || attr == 8u8
        } else {
            false
        }
    } else {
        if b[16] == 8u8 {
            attr == 0u8
        } else if b[16] == 16u8 {
            attr == 0u8 || attr == 8u8
        } else {
            false
        }
    }
}

// ---------------------------------------------------------------------------
// R4: colour-map coherence and range.
// ---------------------------------------------------------------------------

/// R4: the coherence/entry-depth failures that return `InvalidColorMap`.
/// With flag zero: `first`, `length` and entry depth must all be zero, and an
/// indexed image requires an attached map. With flag one: the length must be
/// nonzero and the entry depth must be one of `{15,16,24,32}`.
pub open spec fn cm_bad(b: Seq<u8>) -> bool {
    if b[1] == 0u8 {
        ||| le16(b, 3) != 0
        ||| le16(b, 5) != 0
        ||| b[7] != 0u8
        ||| is_indexed(b[2])
    } else {
        le16(b, 5) == 0 || !map_entry_depth_ok(b[7])
    }
}

/// R4: `first + length > 65,536`, tested only under flag one.
pub open spec fn cm_range_bad(b: Seq<u8>) -> bool {
    le16(b, 3) + le16(b, 5) > 65536
}

/// R12: the exact 18 ASCII bytes `TRUEVISION-XFILE.\0`.
pub open spec fn tga_signature() -> Seq<u8> {
    seq![
        84u8, 82, 85, 69, 86, 73, 83, 73, 79, 78, 45, 88, 70, 73, 76, 69, 46, 0,
    ]
}

// ---------------------------------------------------------------------------
// MH: the total ordered header-and-resource model (THEOREMS.md).
// ---------------------------------------------------------------------------

/// The record `MH` returns on success: the exact raw header fields (including
/// the R1 x/y origin fields at offsets 8 and 10), `N`, `sample_bytes` and
/// `map_bytes`, all in natural numbers before narrowing. The x/y origin fields
/// are metadata only (R1: "Coordinates are metadata, not an extra crop or
/// offset"; R9 flips come from descriptor bits 4/5, not from these fields), so
/// they are carried but never affect the decoded pixels.
pub struct MHModel {
    pub id_length: u8,
    pub color_map_flag: u8,
    pub image_type: u8,
    pub map_first: u32,
    pub map_length: u32,
    pub map_entry_depth: u8,
    pub x_origin: u32,
    pub y_origin: u32,
    pub width: u32,
    pub height: u32,
    pub pixel_depth: u8,
    pub descriptor: u8,
    /// `N = width*height`.
    pub n: int,
    /// `ceil(pixel_depth/8)`.
    pub sample_bytes: int,
    /// `length*ceil(map_entry_depth/8)` under flag one, else zero.
    pub map_bytes: int,
}

/// `MH(b)`: check the R1 input cap and 18-byte header first, then type, flag,
/// dimensions, interleave, image depth, map coherence/depth, map range,
/// attribute count and output cap, in R14 order. Returns `Err` at the first
/// check failure, otherwise the exact record. No absent byte is read.
pub open spec fn mh(b: Seq<u8>) -> Result<MHModel, Error> {
    if b.len() > max_input() {
        Err(Error::FileTooLarge)
    } else if b.len() < 18 {
        Err(Error::TruncatedHeader)
    } else if !type_ok(b[2]) {
        Err(Error::UnsupportedImageType)
    } else if b[1] > 1u8 {
        Err(Error::InvalidColorMapFlag)
    } else if !(1 <= le16(b, 12) && le16(b, 12) <= max_dimension())
        || !(1 <= le16(b, 14) && le16(b, 14) <= max_dimension()) {
        Err(Error::InvalidDimensions)
    } else if b[17] & 0xc0u8 != 0u8 {
        Err(Error::UnsupportedInterleave)
    } else if !depth_ok(b[2], b[16]) {
        Err(Error::UnsupportedPixelDepth)
    } else if cm_bad(b) {
        Err(Error::InvalidColorMap)
    } else if b[1] == 1u8 && cm_range_bad(b) {
        Err(Error::ColorMapRange)
    } else if !attr_ok(b) {
        Err(Error::InvalidAttributeBits)
    } else if 4 * (le16(b, 12) * le16(b, 14)) > max_output() {
        Err(Error::OutputTooLarge)
    } else {
        Ok(
            MHModel {
                id_length: b[0],
                color_map_flag: b[1],
                image_type: b[2],
                map_first: le16(b, 3) as u32,
                map_length: le16(b, 5) as u32,
                map_entry_depth: b[7],
                x_origin: le16(b, 8) as u32,
                y_origin: le16(b, 10) as u32,
                width: le16(b, 12) as u32,
                height: le16(b, 14) as u32,
                pixel_depth: b[16],
                descriptor: b[17],
                n: le16(b, 12) * le16(b, 14),
                sample_bytes: sample_bytes_of(b[16]),
                map_bytes: if b[1] == 1u8 {
                    le16(b, 5) * sample_bytes_of(b[7])
                } else {
                    0
                },
            },
        )
    }
}

/// The success bounds T2 asserts, as a predicate over an `MHModel`.
pub open spec fn mh_bounds(m: MHModel) -> bool {
    &&& m.n == m.width as int * (m.height as int)
    &&& 1 <= ((m.width) as int) <= max_dimension()
    &&& 1 <= ((m.height) as int) <= max_dimension()
    &&& 1 <= m.n
    &&& 4 * m.n <= max_output()
    &&& m.n <= 16777216
    &&& 0 <= m.map_bytes <= 262140
    &&& 1 <= m.sample_bytes <= 4
    // R7: every header-derived position used later (the ID end `18+ID_length`
    // and the map end `18+ID_length+map_bytes`) is far below the input cap and
    // any target `usize`.
    &&& 18 + (m.id_length as int) + m.map_bytes <= 262413
}

// ---------------------------------------------------------------------------
// Arithmetic lemmas (used only to justify the executable's bounds).
// ---------------------------------------------------------------------------

/// `le16(b,p) < 2^16` whenever both bytes exist: the two `u8` reads cannot
/// reach `256*256`.
pub proof fn lemma_le16_lt(b: Seq<u8>, p: int)
    requires
        p + 2 <= b.len(),
    ensures
        0 <= le16(b, p) < 65536,
{
    assert(0 <= b[p] as int);
    assert((b[p] as int) < 256);
    assert(0 <= b[p + 1] as int);
    assert((b[p + 1] as int) < 256);
    assert(le16(b, p) < 65536) by (nonlinear_arith)
        requires
            le16(b, p) == b[p] as int + 256 * (b[p + 1] as int),
            0 <= b[p] as int,
            (b[p] as int) < 256,
            0 <= b[p + 1] as int,
            (b[p + 1] as int) < 256,
    ;
    assert(le16(b, p) >= 0) by (nonlinear_arith)
        requires
            le16(b, p) == b[p] as int + 256 * (b[p + 1] as int),
            0 <= b[p] as int,
            0 <= b[p + 1] as int,
    ;
}

/// `sample_bytes_of(d) <= 4` for every supported depth `d`. The depths in
/// `{8,15,16,24,32}` are exactly the supported ones.
pub proof fn lemma_sample_bytes_le_4(d: u8)
    requires
        d == 8u8 || d == 15u8 || d == 16u8 || d == 24u8 || d == 32u8,
    ensures
        1 <= sample_bytes_of(d) <= 4,
{
    reveal(sample_bytes_of);
    if d == 8u8 {
        assert(sample_bytes_of(d) == 1);
    } else if d == 15u8 {
        assert(sample_bytes_of(d) == 2);
    } else if d == 16u8 {
        assert(sample_bytes_of(d) == 2);
    } else if d == 24u8 {
        assert(sample_bytes_of(d) == 3);
    } else {
        assert(sample_bytes_of(d) == 4);
    }
}

/// The success bounds hold for every `MHModel` that `MH` returns. This is the
/// arithmetic content of T2, proved from the model alone (no executable code).
pub proof fn lemma_mh_ok_bounds(b: Seq<u8>, m: MHModel)
    requires
        mh(b) == Ok(m),
    ensures
        mh_bounds(m),
{
    reveal(mh);
    reveal(mh_bounds);
    assert(b.len() >= 18);
    let w = le16(b, 12);
    let h = le16(b, 14);
    assert(m == MHModel {
        id_length: b[0],
        color_map_flag: b[1],
        image_type: b[2],
        map_first: le16(b, 3) as u32,
        map_length: le16(b, 5) as u32,
        map_entry_depth: b[7],
        x_origin: le16(b, 8) as u32,
        y_origin: le16(b, 10) as u32,
        width: le16(b, 12) as u32,
        height: le16(b, 14) as u32,
        pixel_depth: b[16],
        descriptor: b[17],
        n: le16(b, 12) * le16(b, 14),
        sample_bytes: sample_bytes_of(b[16]),
        map_bytes: if b[1] == 1u8 {
            le16(b, 5) * sample_bytes_of(b[7])
        } else {
            0
        },
    });
    lemma_le16_lt(b, 12);
    lemma_le16_lt(b, 14);
    lemma_le16_lt(b, 5);
    assert(1 <= w <= 8192);
    assert(1 <= h <= 8192);
    assert(m.width == w as u32);
    assert(m.height == h as u32);
    assert(w as u32 as int == w);
    assert(h as u32 as int == h);
    assert(m.n == w * h);
    assert(4 * (w * h) <= 67108864) by (nonlinear_arith)
        requires
            1 <= w <= 8192,
            1 <= h <= 8192,
            4 * (w * h) <= 67108864,
    ;
    assert(w * h <= 16777216) by (nonlinear_arith)
        requires
            1 <= w <= 8192,
            1 <= h <= 8192,
            4 * (w * h) <= 67108864,
    ;
    assert(m.sample_bytes == sample_bytes_of(b[16]));
    // The image depth passed R3, so it is one of the supported depths.
    assert(depth_ok(b[2], b[16]));
    reveal(depth_ok);
    reveal(is_indexed);
    assert(b[16] == 8u8 || b[16] == 15u8 || b[16] == 16u8 || b[16] == 24u8 || b[16] == 32u8);
    lemma_sample_bytes_le_4(b[16]);
    assert(m.map_bytes >= 0);
    if b[1] == 1u8 {
        assert(m.map_bytes == le16(b, 5) * sample_bytes_of(b[7]));
        assert(map_entry_depth_ok(b[7]));
        reveal(map_entry_depth_ok);
        assert(b[7] == 15u8 || b[7] == 16u8 || b[7] == 24u8 || b[7] == 32u8);
        lemma_sample_bytes_le_4(b[7]);
        assert(le16(b, 5) <= 65535);
        assert(sample_bytes_of(b[7]) <= 4);
        assert(m.map_bytes <= 262140) by (nonlinear_arith)
            requires
                m.map_bytes == le16(b, 5) * sample_bytes_of(b[7]),
                le16(b, 5) <= 65535,
                sample_bytes_of(b[7]) <= 4,
                sample_bytes_of(b[7]) >= 1,
        ;
        assert(m.map_bytes >= 0) by (nonlinear_arith)
            requires
                m.map_bytes == le16(b, 5) * sample_bytes_of(b[7]),
                le16(b, 5) >= 0,
                sample_bytes_of(b[7]) >= 1,
        ;
    } else {
        assert(b[1] == 0u8);
        assert(m.map_bytes == 0);
    }
    assert(m.map_bytes <= 262140);
    assert(m.width == w as u32);
    assert(m.height == h as u32);
    assert(m.n == m.width as int * (m.height as int));
    assert(1 <= m.width as int <= max_dimension());
    assert(1 <= m.height as int <= max_dimension());
    assert(1 <= m.n) by (nonlinear_arith)
        requires
            m.n == w * h,
            1 <= w,
            1 <= h,
    ;
    assert(4 * m.n <= max_output());
    assert(m.n <= 16777216);
    assert(0 <= m.map_bytes <= 262140);
    assert(1 <= m.sample_bytes <= 4);
    assert(m.id_length as int <= 255);
    assert(18 + (m.id_length as int) + m.map_bytes <= 262413) by (nonlinear_arith)
        requires
            m.id_length as int <= 255,
            m.map_bytes <= 262140,
    ;
}

/// The same bounds, as a `forall` that the SMT solver can apply wherever
/// `mh(b) == Ok(m)` is in scope.
pub proof fn lemma_mh_ok_bounds_auto(b: Seq<u8>)
    ensures
        forall|m: MHModel| mh(b) == Ok(m) ==> #[trigger] mh_bounds(m),
{
    assert forall|m: MHModel| mh(b) == Ok(m) implies #[trigger] mh_bounds(m) by {
        lemma_mh_ok_bounds(b, m);
    }
}

/// `j * stride + stride <= length * stride` for `j < length`, `stride >= 0`:
/// the map-entry read stays inside a slice of `length * stride` bytes.
pub proof fn lemma_map_index_bound(j: int, length: int, stride: int)
    requires
        0 <= j < length,
        0 <= stride,
    ensures
        0 <= j * stride + stride <= length * stride,
{
    assert(j * stride + stride <= length * stride) by (nonlinear_arith)
        requires
            0 <= j < length,
            0 <= stride,
    ;
}

/// Quotient/remainder bounds for the R9 coordinate split: for `1 <= w`,
/// `0 <= k < w*h`, the coordinates `x = k % w` and `y = k / w` satisfy
/// `0 <= x < w` and `0 <= y < h`.
pub proof fn lemma_coords(w: int, h: int, k: int)
    requires
        1 <= w,
        1 <= h,
        0 <= k,
        k < w * h,
    ensures
        0 <= k % w < w,
        0 <= k / w < h,
{
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod(k, w);
    assert(k == w * (k / w) + k % w);
    assert(0 <= k % w) by (nonlinear_arith)
        requires
            k == w * (k / w) + k % w,
            w >= 1,
            k % w >= -w,
    ;
    assert(k % w < w) by (nonlinear_arith)
        requires
            k == w * (k / w) + k % w,
            w >= 1,
    ;
    assert(k / w < h) by (nonlinear_arith)
        requires
            k == w * (k / w) + k % w,
            1 <= w,
            0 <= k % w,
            k < w * h,
    ;
}

/// The oriented-write safety bound: for `0 <= x < w`, `0 <= y < h` and the two
/// independent axis flips, the oriented index `by*w + bx` lies in `[0, w*h)`.
pub proof fn lemma_offset_bound(w: int, h: int, y: int, x: int, bx: int, by: int)
    requires
        1 <= w,
        1 <= h,
        0 <= x < w,
        0 <= y < h,
        bx == x || bx == w - 1 - x,
        by == y || by == h - 1 - y,
    ensures
        0 <= by * w + bx < w * h,
{
    if bx == w - 1 - x {
        assert(bx <= w - 1) by (nonlinear_arith)
            requires
                0 <= x,
                bx == w - 1 - x,
        ;
        assert(bx >= 0) by (nonlinear_arith)
            requires
                x < w,
                bx == w - 1 - x,
        ;
    }
    if by == h - 1 - y {
        assert(by <= h - 1) by (nonlinear_arith)
            requires
                0 <= y,
                by == h - 1 - y,
        ;
        assert(by >= 0) by (nonlinear_arith)
            requires
                y < h,
                by == h - 1 - y,
        ;
    }
    assert(0 <= bx < w);
    assert(0 <= by < h);
    assert(by * w + bx < w * h) by (nonlinear_arith)
        requires
            1 <= w,
            0 <= bx < w,
            0 <= by < h,
    ;
}

// ---------------------------------------------------------------------------
// M3: R9's coordinate map and its bijection lemma.
// ---------------------------------------------------------------------------

/// One axis of R9: `a-1-t` when the flip bit is set, else `t`. Both R9 axes are
/// of this form; `oxr`/`oyr` below fix each axis's polarity.
pub open spec fn spec_axis(a: int, flip: bool, t: int) -> int {
    if flip {
        a - 1 - t
    } else {
        t
    }
}

/// R9's horizontal coordinate `X = width-1-x` when descriptor bit 4 is set,
/// else `x`.
pub open spec fn oxr(w: int, fx: bool, x: int) -> int {
    if fx {
        w - 1 - x
    } else {
        x
    }
}

/// R9's vertical coordinate `Y = y` when descriptor bit 5 is set, else
/// `height-1-y`. Note the opposite polarity to `X`.
pub open spec fn oyr(h: int, fy: bool, y: int) -> int {
    if fy {
        y
    } else {
        h - 1 - y
    }
}

/// `oyr` is `spec_axis` with the flip bit complemented.
pub proof fn lemma_oyr_is_axis(h: int, fy: bool, y: int)
    ensures
        oyr(h, fy, y) == spec_axis(h, !fy, y),
{
    reveal(oyr);
    reveal(spec_axis);
}

/// `oxr` is `spec_axis` with the flip bit as given.
pub proof fn lemma_oxr_is_axis(w: int, fx: bool, x: int)
    ensures
        oxr(w, fx, x) == spec_axis(w, fx, x),
{
    reveal(oxr);
    reveal(spec_axis);
}

/// R9's coordinate map in *cell* units: input sample `k` of a `1<=w,h` image has
/// logical coordinates `x=k%width`, `y=k/width` and is written at cell
/// `f(k)=Y(k)*width+X(k)`, where `X=width-1-x` when the horizontal-origin bit is
/// set, else `x`, and `Y=y` when the vertical-origin bit is set, else
/// `height-1-y`. The byte offset the executable computes is `4*f(k)`.
pub open spec fn fcoord(w: int, h: int, fx: bool, fy: bool, k: int) -> int {
    oyr(h, fy, k / w) * w + oxr(w, fx, k % w)
}

/// The cell of the source sample written to output cell `q`: the same axis
/// flips applied to `q`'s own quotient and remainder. This is `f`'s inverse.
pub open spec fn gcoord(w: int, h: int, fx: bool, fy: bool, q: int) -> int {
    oyr(h, fy, q / w) * w + oxr(w, fx, q % w)
}

/// Each axis flip is an involution on `[0,a)`.
pub proof fn lemma_axis_involutive(a: int, flip: bool, t: int)
    requires
        0 <= t < a,
    ensures
        0 <= spec_axis(a, flip, t) < a,
        spec_axis(a, flip, spec_axis(a, flip, t)) == t,
{
    reveal(spec_axis);
    if flip {
        assert(spec_axis(a, flip, t) == a - 1 - t);
        assert(0 <= a - 1 - t < a) by (nonlinear_arith)
            requires
                0 <= t < a,
        ;
        assert(spec_axis(a, flip, a - 1 - t) == a - 1 - (a - 1 - t)) by {
            reveal(spec_axis);
        }
        assert(a - 1 - (a - 1 - t) == t);
    } else {
        assert(spec_axis(a, flip, t) == t);
    }
}

/// `c == y*w + x` with `0 <= x < w` pins down `c / w == y` and `c % w == x`.
pub proof fn lemma_coord_digits(c: int, x: int, y: int, w: int)
    requires
        c == y * w + x,
        0 <= x < w,
        w > 0,
    ensures
        c / w == y,
        c % w == x,
{
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(c, w, y, x);
}

/// `f` maps `[0,w*h)` into `[0,w*h)`.
pub proof fn lemma_fcoord_range(w: int, h: int, fx: bool, fy: bool, k: int)
    requires
        1 <= w,
        1 <= h,
        0 <= k < w * h,
    ensures
        0 <= fcoord(w, h, fx, fy, k) < w * h,
{
    reveal(fcoord);
    lemma_coords(w, h, k);
    lemma_axis_involutive(w, fx, k % w);
    lemma_axis_involutive(h, !fy, k / w);
    lemma_oyr_is_axis(h, fy, k / w);
    lemma_offset_bound(
        w,
        h,
        k / w,
        k % w,
        spec_axis(w, fx, k % w),
        spec_axis(h, !fy, k / w),
    );
}

/// `q`'s preimage is in range and `f` inverts on it: `f(g(q)) == q`.
pub proof fn lemma_fcoord_inverse(w: int, h: int, fx: bool, fy: bool, q: int)
    requires
        1 <= w,
        1 <= h,
        0 <= q < w * h,
    ensures
        0 <= gcoord(w, h, fx, fy, q) < w * h,
        fcoord(w, h, fx, fy, gcoord(w, h, fx, fy, q)) == q,
{
    reveal(fcoord);
    reveal(gcoord);
    reveal(oyr);
    reveal(oxr);
    reveal(spec_axis);
    lemma_coords(w, h, q);
    lemma_axis_involutive(w, fx, q % w);
    lemma_axis_involutive(h, !fy, q / w);
    lemma_oyr_is_axis(h, fy, q / w);
    lemma_oxr_is_axis(w, fx, q % w);
    let bx = spec_axis(w, fx, q % w);
    let by = spec_axis(h, !fy, q / w);
    assert(0 <= bx < w);
    assert(0 <= by < h);
    assert(gcoord(w, h, fx, fy, q) == by * w + bx) by {
        reveal(gcoord);
        reveal(oyr);
        reveal(oxr);
    }
    assert(0 <= by * w + bx < w * h) by (nonlinear_arith)
        requires
            1 <= w,
            0 <= bx < w,
            0 <= by < h,
    ;
    // digits of g(q)
    lemma_coord_digits(by * w + bx, bx, by, w);
    assert((by * w + bx) / w == by);
    assert((by * w + bx) % w == bx);
    // f(g(q)) = oyr(h,fy,by)*w + oxr(w,fx,bx) = (q/w)*w + (q%w) = q
    assert(fcoord(w, h, fx, fy, by * w + bx) == oyr(h, fy, by) * w + oxr(w, fx, bx)) by {
        reveal(fcoord);
    }
    assert(oyr(h, fy, by) == q / w);
    assert(oxr(w, fx, bx) == q % w);
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod(q, w);
    assert(q == w * (q / w) + q % w);
    assert((q / w) * w + q % w == q) by (nonlinear_arith)
        requires
            q == w * (q / w) + q % w,
    ;
    assert(oyr(h, fy, by) * w + oxr(w, fx, bx) == q) by (nonlinear_arith)
        requires
            oyr(h, fy, by) == q / w,
            oxr(w, fx, bx) == q % w,
            (q / w) * w + q % w == q,
    ;
}

/// `exists! k<N: f(k)=q`: surjectivity in the manifest's form. Uniqueness is
/// `lemma_orientation_bijection`'s injectivity clause.
pub open spec fn surjective_at(w: int, h: int, fx: bool, fy: bool, q: int) -> bool {
    exists|k: int| 0 <= k < w * h && #[trigger] fcoord(w, h, fx, fy, k) == q
}

/// `g` is a left inverse of `f`: `g(f(k)) == k` for `k<N`.
pub proof fn lemma_fcoord_left_inverse(w: int, h: int, fx: bool, fy: bool, k: int)
    requires
        1 <= w,
        1 <= h,
        0 <= k < w * h,
    ensures
        gcoord(w, h, fx, fy, fcoord(w, h, fx, fy, k)) == k,
{
    reveal(fcoord);
    reveal(gcoord);
    reveal(oyr);
    reveal(oxr);
    reveal(spec_axis);
    lemma_coords(w, h, k);
    lemma_axis_involutive(w, fx, k % w);
    lemma_axis_involutive(h, !fy, k / w);
    lemma_oyr_is_axis(h, fy, k / w);
    lemma_oxr_is_axis(w, fx, k % w);
    let bx = spec_axis(w, fx, k % w);
    let by = spec_axis(h, !fy, k / w);
    assert(0 <= bx < w);
    assert(0 <= by < h);
    assert(fcoord(w, h, fx, fy, k) == by * w + bx) by {
        reveal(fcoord);
    }
    lemma_coord_digits(by * w + bx, bx, by, w);
    assert((by * w + bx) / w == by);
    assert((by * w + bx) % w == bx);
    assert(gcoord(w, h, fx, fy, by * w + bx) == oyr(h, fy, by) * w + oxr(w, fx, bx)) by {
        reveal(gcoord);
        reveal(oyr);
        reveal(oxr);
    }
    assert(oyr(h, fy, by) == k / w);
    assert(oxr(w, fx, bx) == k % w);
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod(k, w);
    assert(k == w * (k / w) + k % w);
    assert(oyr(h, fy, by) * w + oxr(w, fx, bx) == (k / w) * w + k % w) by (nonlinear_arith)
        requires
            oyr(h, fy, by) == k / w,
            oxr(w, fx, bx) == k % w,
    ;
    assert((k / w) * w + k % w == k) by (nonlinear_arith)
        requires
            k == w * (k / w) + k % w,
    ;
}

/// The manifest's orientation-map bijection lemma, for every `1<=w,h<=8192`,
/// `N=w*h` and both settings of each origin bit: `f` maps `[0,N)` onto `[0,N)`
/// bijectively. Stated as range, injectivity, and unique existence of a
/// preimage, so T4 can use injectivity for its writes-distinct clause and L3/L4
/// surjectivity for "every cell is written".
pub proof fn lemma_orientation_bijection(w: int, h: int, fx: bool, fy: bool)
    requires
        1 <= w,
        1 <= h,
    ensures
        forall|k: int| 0 <= k < w * h ==> 0 <= #[trigger] fcoord(w, h, fx, fy, k) < w * h,
        forall|j: int, k: int| 0 <= j < w * h && 0 <= k < w * h && #[trigger] fcoord(
            w,
            h,
            fx,
            fy,
            j,
        ) == #[trigger] fcoord(w, h, fx, fy, k) ==> j == k,
        forall|q: int| 0 <= q < w * h ==> #[trigger] fcoord(
            w,
            h,
            fx,
            fy,
            gcoord(w, h, fx, fy, q),
        ) == q && 0 <= gcoord(w, h, fx, fy, q) < w * h,
        forall|q: int| 0 <= q < w * h ==> #[trigger] surjective_at(w, h, fx, fy, q),
{
    assert forall|k: int| 0 <= k < w * h implies 0 <= #[trigger] fcoord(w, h, fx, fy, k) < w
        * h by {
        lemma_fcoord_range(w, h, fx, fy, k);
    }
    assert forall|j: int, k: int| 0 <= j < w * h && 0 <= k < w * h && #[trigger] fcoord(
        w,
        h,
        fx,
        fy,
        j,
    ) == #[trigger] fcoord(w, h, fx, fy, k) implies j == k by {
        lemma_fcoord_left_inverse(w, h, fx, fy, j);
        lemma_fcoord_left_inverse(w, h, fx, fy, k);
    }
    assert forall|q: int| 0 <= q < w * h implies #[trigger] fcoord(
        w,
        h,
        fx,
        fy,
        gcoord(w, h, fx, fy, q),
    ) == q && 0 <= gcoord(w, h, fx, fy, q) < w * h by {
        lemma_fcoord_inverse(w, h, fx, fy, q);
    }
    assert forall|q: int| 0 <= q < w * h implies #[trigger] surjective_at(w, h, fx, fy, q) by {
        let k0 = gcoord(w, h, fx, fy, q);
        lemma_fcoord_inverse(w, h, fx, fy, q);
        assert(0 <= k0 < w * h);
        assert(fcoord(w, h, fx, fy, k0) == q);
    }
}

// ---------------------------------------------------------------------------
// M3: R8's byte-exact colour conversion, as a total model.
// ---------------------------------------------------------------------------

/// R8: expand a five-bit channel. `(c<<3)|(c>>2)`; for `c<32` the two bit
/// fields do not overlap, so the value is `8c + c/4`. `lemma_rep5` relates this
/// integer form to the bit operations.
pub open spec fn rep5(c: int) -> int {
    c * 8 + c / 4
}

/// R8's `LE16` over a byte sequence `s` at offset `p`.
pub open spec fn le16s(s: Seq<u8>, p: int) -> int {
    s[p] as int + 256 * (s[p + 1] as int)
}

/// R8: `ceil(d/8)` in the model's arithmetic.
pub open spec fn stride_of(d: int) -> int {
    (d + 7) / 8
}

/// R8: the index an 8- or 16-bit indexed sample carries.
pub open spec fn sindex(s: Seq<u8>, sb: int) -> int {
    if sb == 1 {
        s[0] as int
    } else {
        s[0] as int + 256 * (s[1] as int)
    }
}

/// R8's blue, green and red fields of a 15/16-bit word `v`: `v&31`,
/// `(v>>5)&31`, `(v>>10)&31`.
pub open spec fn blue5(v: int) -> int {
    v % 32
}

pub open spec fn green5(v: int) -> int {
    (v / 32) % 32
}

pub open spec fn red5(v: int) -> int {
    (v / 1024) % 32
}

/// R8: bit 15 of a 16-bit word is set (used for 16-bit map entries with
/// attribute count one).
pub open spec fn bit15(v: int) -> bool {
    v / 32768 == 1
}

/// The conversion-relevant projection of a successful header: the image kind
/// (0 indexed, 1 true-colour, 2 greyscale), the image depth, the attribute
/// count, the checked map's `first`/`length`/entry depth, `N` and the sample
/// width. `MR` and the conversion model are stated over this record, so they do
/// not depend on the executable `Header` type.
pub struct PktH {
    pub kind: int,
    pub depth: int,
    pub attr: int,
    pub map_first: int,
    pub map_len: int,
    pub map_depth: int,
    pub n: int,
    pub sample_bytes: int,
}

/// R8: a 15/16-bit word's three replicated five-bit channels.
pub open spec fn rgb555(v: int) -> Seq<u8> {
    seq![
        rep5(red5(v)) as u8,
        rep5(green5(v)) as u8,
        rep5(blue5(v)) as u8,
    ]
}

/// R8: the 15-bit layout, alpha `a` (255 for depth 15, bit 15 ignored).
pub open spec fn bytes4_15(v: int, a: u8) -> Seq<u8> {
    rgb555(v).push(a)
}

/// R8: convert a validated (non-indexed) sample `s` of `h.depth` bytes. This is
/// the independent transcription of R8's six image cases; the executable's
/// `convert_sample` is proved equal to it, not the other way round.
pub open spec fn conv_sample(h: PktH, s: Seq<u8>) -> Seq<u8> {
    if h.kind == 1 && (h.depth == 15 || h.depth == 16) {
        // True-colour 15/16: five-bit channels, alpha 255 for counts 0 and 1.
        bytes4_15(le16s(s, 0), 255)
    } else if h.kind == 1 && h.depth == 24 {
        seq![s[2], s[1], s[0], 255u8]
    } else if h.kind == 1 && h.depth == 32 {
        seq![s[2], s[1], s[0], (if h.attr == 8 { s[3] } else { 255u8 })]
    } else if h.kind == 2 && h.depth == 8 {
        seq![s[0], s[0], s[0], 255u8]
    } else if h.kind == 2 && h.depth == 16 {
        seq![s[0], s[0], s[0], (if h.attr == 8 { s[1] } else { 255u8 })]
    } else {
        // Outside the supported-depth domain (`OutsideDomain`, not an API error).
        seq![0u8, 0, 0, 255]
    }
}

/// R8's per-entry attribute policy, as a total spec function: the entry depth
/// and the authoritative attribute count decide alpha. `entry_alpha` in the
/// executable is proved equal to this.
pub open spec fn entry_alpha_spec(ed: int, attr: int, v: int, a: int) -> u8 {
    if ed == 15 {
        255
    } else if ed == 16 {
        if attr == 1 && !bit15(v) {
            0
        } else {
            255
        }
    } else if ed == 24 {
        255
    } else if ed == 32 {
        if attr == 8 {
            a as u8
        } else {
            255
        }
    } else {
        255
    }
}

/// R8: convert map entry `index` of the validated map slice `map`. The entry's
/// bytes are `map[off..off+stride]` with `off = (index-first)*stride`, and the
/// entry-depth attribute policy is R8's: bit 15 controls alpha only for 16-bit
/// entries with count one, the stored A byte only for 32-bit with count eight.
pub open spec fn conv_map(h: PktH, map: Seq<u8>, index: int) -> Seq<u8> {
    let stride = stride_of(h.map_depth);
    let off = (index - h.map_first) * stride;
    if h.map_depth == 15 {
        bytes4_15(le16s(map, off), 255)
    } else if h.map_depth == 16 {
        rgb555(le16s(map, off)).push(entry_alpha_spec(h.map_depth, h.attr, le16s(map, off), 0))
    } else if h.map_depth == 24 {
        seq![map[off + 2], map[off + 1], map[off], 255u8]
    } else if h.map_depth == 32 {
        seq![
            map[off + 2],
            map[off + 1],
            map[off],
            entry_alpha_spec(h.map_depth, h.attr, 0, map[off + 3] as int),
        ]
    } else {
        seq![0u8, 0, 0, 255]
    }
}

/// R8: the complete conversion of one validated sample: indexed samples first
/// read their index and then convert the corresponding map entry.
pub open spec fn conv2(h: PktH, map: Seq<u8>, s: Seq<u8>) -> Seq<u8> {
    if h.kind == 0 {
        conv_map(h, map, sindex(s, h.sample_bytes))
    } else {
        conv_sample(h, s)
    }
}

/// `rep5(c) == (c<<3)|(c>>2)`, so the integer form above really is R8's bit
/// replication, and the result is a byte.
pub proof fn lemma_rep5(c: u32)
    requires
        c < 32u32,
    ensures
        (((c << 3u32) | (c >> 2u32)) as int) == rep5(c as int),
        0 <= rep5(c as int) < 256,
{
    reveal(rep5);
    assert((((c << 3u32) | (c >> 2u32)) as int) == rep5(c as int)) by (bit_vector)
        requires
            c < 32u32,
    ;
    assert(0 <= rep5(c as int) < 256) by (nonlinear_arith)
        requires
            c < 32u32,
            rep5(c as int) == (c as int) * 8 + (c as int) / 4,
    ;
}

/// `red5`, `green5`, `blue5` are R8's `(v>>10)&31`, `(v>>5)&31`, `v&31` and
/// each is five bits.
pub proof fn lemma_rgb_fields(v: u32)
    requires
        v < 65536u32,
    ensures
        red5(v as int) == ((v >> 10u32) & 31u32) as int,
        green5(v as int) == ((v >> 5u32) & 31u32) as int,
        blue5(v as int) == (v & 31u32) as int,
        0 <= red5(v as int) < 32,
        0 <= green5(v as int) < 32,
        0 <= blue5(v as int) < 32,
{
    reveal(red5);
    reveal(green5);
    reveal(blue5);
    assert(red5(v as int) == ((v >> 10u32) & 31u32) as int) by (bit_vector)
        requires
            v < 65536u32,
    ;
    assert(green5(v as int) == ((v >> 5u32) & 31u32) as int) by (bit_vector)
        requires
            v < 65536u32,
    ;
    assert(blue5(v as int) == (v & 31u32) as int) by (bit_vector)
        requires
            v < 65536u32,
    ;
    assert(0 <= red5(v as int) < 32) by (nonlinear_arith)
        requires
            v < 65536u32,
            red5(v as int) == (v as int / 1024) % 32,
    ;
    assert(0 <= green5(v as int) < 32) by (nonlinear_arith)
        requires
            v < 65536u32,
            green5(v as int) == (v as int / 32) % 32,
    ;
    assert(0 <= blue5(v as int) < 32) by (nonlinear_arith)
        requires
            v < 65536u32,
            blue5(v as int) == (v as int) % 32,
    ;
}

/// `bit15(v)` is exactly `v & 0x8000 != 0`.
pub proof fn lemma_bit15(v: u32)
    requires
        v < 65536u32,
    ensures
        bit15(v as int) == (v & 0x8000u32 != 0u32),
{
    reveal(bit15);
    if v >= 32768u32 {
        assert((v & 0x8000u32) != 0u32) by (bit_vector)
            requires
                v < 65536u32,
                v >= 32768u32,
        ;
        assert(v as int / 32768 == 1) by (nonlinear_arith)
            requires
                v < 65536u32,
                v >= 32768u32,
        ;
    } else {
        assert((v & 0x8000u32) == 0u32) by (bit_vector)
            requires
                v < 32768u32,
        ;
        assert(v as int / 32768 == 0) by (nonlinear_arith)
            requires
                v < 32768u32,
        ;
    }
}

/// The map-entry read of a validated index lies inside the checked slice.
pub open spec fn map_read_in_slice(h: PktH, idx: int) -> bool {
    h.map_first <= idx < h.map_first + h.map_len ==> 0 <= (idx - h.map_first) * stride_of(
        h.map_depth,
    ) + stride_of(h.map_depth) <= h.map_len * stride_of(h.map_depth)
}

/// The map-entry read of an in-range index stays inside a slice of
/// `length * stride` bytes.
pub proof fn lemma_map_read_bound(first: int, len: int, stride: int, index: int)
    requires
        0 <= stride,
        first <= index < first + len,
        len >= 0,
    ensures
        0 <= (index - first) * stride + stride <= len * stride,
{
    assert((index - first) * stride + stride <= len * stride) by (nonlinear_arith)
        requires
            0 <= index - first < len,
            stride >= 0,
    ;
}

/// The colour-conversion lemma, separate from the pixel loops: for every
/// supported image kind and depth, and every validated sample and map slice,
/// `conv2` is the exact R8 byte tuple, has four bytes, and (for indexed images)
/// its map read lies inside the checked slice. Each depth case is named, so no
/// supported image type or entry depth is skipped.
pub proof fn lemma_colour_conversion(h: PktH, map: Seq<u8>, s: Seq<u8>)
    requires
        s.len() == h.sample_bytes,
        1 <= h.sample_bytes <= 4,
        h.kind == 0 || h.kind == 1 || h.kind == 2,
        h.kind == 0 ==> (h.depth == 8 || h.depth == 16) && (h.map_depth == 15 || h.map_depth == 16
            || h.map_depth == 24 || h.map_depth == 32),
        h.kind == 1 ==> (h.depth == 15 || h.depth == 16 || h.depth == 24 || h.depth == 32),
        h.kind == 2 ==> (h.depth == 8 || h.depth == 16),
        h.kind == 0 ==> h.map_first <= sindex(s, h.sample_bytes) < h.map_first + h.map_len,
        h.kind == 0 ==> map.len() == h.map_len * stride_of(h.map_depth),
    ensures
        conv2(h, map, s).len() == 4,
        h.kind == 0 ==> 0 <= (sindex(s, h.sample_bytes) - h.map_first) * stride_of(h.map_depth)
            + stride_of(h.map_depth) <= map.len(),
        // Indexed: the map entry's own four depth policies, spelled out.
        h.kind == 0 ==> conv2(h, map, s) == conv_map(h, map, sindex(s, h.sample_bytes)),
        h.kind == 0 && h.map_depth == 15 ==> conv2(h, map, s) == bytes4_15(
            le16s(map, (sindex(s, h.sample_bytes) - h.map_first) * 2),
            255,
        ),
        h.kind == 0 && h.map_depth == 16 ==> conv2(h, map, s) == {
            let off = (sindex(s, h.sample_bytes) - h.map_first) * 2;
            rgb555(le16s(map, off)).push(entry_alpha_spec(16, h.attr, le16s(map, off), 0))
        },
        h.kind == 0 && h.map_depth == 24 ==> conv2(h, map, s) == {
            let off = (sindex(s, h.sample_bytes) - h.map_first) * 3;
            seq![map[off + 2], map[off + 1], map[off], 255u8]
        },
        h.kind == 0 && h.map_depth == 32 ==> conv2(h, map, s) == {
            let off = (sindex(s, h.sample_bytes) - h.map_first) * 4;
            seq![map[off + 2], map[off + 1], map[off], entry_alpha_spec(32, h.attr, 0, map[off + 3] as int)]
        },
        // True-colour 15/16: three replicated five-bit channels, alpha 255.
        h.kind == 1 && (h.depth == 15 || h.depth == 16) ==> conv2(h, map, s) == bytes4_15(
            le16s(s, 0),
            255,
        ),
        // True-colour 24 and 32.
        h.kind == 1 && h.depth == 24 ==> conv2(h, map, s) == seq![s[2], s[1], s[0], 255u8],
        h.kind == 1 && h.depth == 32 ==> conv2(h, map, s) == seq![
            s[2],
            s[1],
            s[0],
            (if h.attr == 8 {
                s[3]
            } else {
                255u8
            }),
        ],
        // Greyscale 8 and 16.
        h.kind == 2 && h.depth == 8 ==> conv2(h, map, s) == seq![s[0], s[0], s[0], 255u8],
        h.kind == 2 && h.depth == 16 ==> conv2(h, map, s) == seq![
            s[0],
            s[0],
            s[0],
            (if h.attr == 8 {
                s[1]
            } else {
                255u8
            }),
        ],
{
    reveal(conv2);
    if h.kind == 0 {
        reveal(conv_map);
        reveal(stride_of);
        reveal(bytes4_15);
        reveal(rgb555);
        assert(0 <= stride_of(h.map_depth)) by (nonlinear_arith)
            requires
                h.map_depth == 15 || h.map_depth == 16 || h.map_depth == 24 || h.map_depth == 32,
        ;
        assert(stride_of(h.map_depth) == 2 || stride_of(h.map_depth) == 3 || stride_of(h.map_depth)
            == 4) by {
            reveal(stride_of);
            if h.map_depth == 15 {
            } else if h.map_depth == 16 {
            } else if h.map_depth == 24 {
            } else {
            }
        }
        lemma_map_read_bound(
            h.map_first,
            h.map_len,
            stride_of(h.map_depth),
            sindex(s, h.sample_bytes),
        );
    } else {
        reveal(conv_sample);
        reveal(bytes4_15);
    }
    assert(conv2(h, map, s).len() == 4);
}

// ---------------------------------------------------------------------------
// M3: MR, the total R11 packet-stream model, and its repetition algebra.
// ---------------------------------------------------------------------------

/// R11: `count = (control&0x7f)+1`, in 1..=128. For a byte `control` this is
/// `control+1` below 128 and `control-127` at or above it; `lemma_rle_count_bit`
/// relates the form to the bit field.
pub open spec fn rle_count(control: int) -> int {
    if control >= 128 {
        control - 127
    } else {
        control + 1
    }
}

/// `rle_count(control)` is `(control & 0x7f) + 1` for every byte.
pub proof fn lemma_rle_count_bit(control: u8)
    ensures
        rle_count(control as int) == (((control & 0x7fu8) as int) + 1),
{
    reveal(rle_count);
    if control >= 128u8 {
        assert(rle_count(control as int) == control as int - 127);
        assert((control & 0x7fu8) as int == control as int - 128) by (bit_vector)
            requires
                control >= 128u8,
        ;
    } else {
        assert(rle_count(control as int) == control as int + 1);
        assert((control & 0x7fu8) as int == control as int) by (bit_vector)
            requires
                control < 128u8,
        ;
    }
}

/// R11: control bit 7 set means a repeat packet (one sample, `count` times).
pub open spec fn rle_repeat(control: int) -> bool {
    control >= 128
}

/// R8's index-range test: `first <= index < first+length`.
pub open spec fn in_range(h: PktH, index: int) -> bool {
    h.map_first <= index < h.map_first + h.map_len
}

/// `q` repeated `count` times, for a sequence of pixels, as the constant
/// function on `0..count`. Defined for `count >= 0`.
pub open spec fn pix_pow(q: Seq<u8>, count: int) -> Seq<Seq<u8>> {
    Seq::new(count as nat, |j: int| q)
}

/// `|q^count| == count`.
pub proof fn lemma_pix_pow_len(q: Seq<u8>, count: int)
    requires
        count >= 0,
    ensures
        pix_pow(q, count).len() == count,
{
    reveal(pix_pow);
    broadcast use vstd::seq::group_seq_lemmas;
}

/// `q^k`'s `j`th pixel is `q`.
pub proof fn lemma_pix_pow_index(q: Seq<u8>, count: int, j: int)
    requires
        0 <= j < count,
    ensures
        pix_pow(q, count)[j] == q,
{
    reveal(pix_pow);
    broadcast use vstd::seq::group_seq_lemmas;
}

/// `q^0` is the empty pixel sequence.
pub proof fn lemma_pix_pow_zero(q: Seq<u8>)
    ensures
        pix_pow(q, 0) == Seq::<Seq<u8>>::empty(),
{
    reveal(pix_pow);
    lemma_pix_pow_len(q, 0);
    broadcast use vstd::seq::group_seq_lemmas;
    assert(pix_pow(q, 0).len() == 0);
    assert(pix_pow(q, 0) =~= Seq::<Seq<u8>>::empty());
}

/// `(q^a) + (q^b) == q^(a+b)`.
pub proof fn lemma_pix_pow_add(q: Seq<u8>, a: int, b: int)
    requires
        a >= 0,
        b >= 0,
    ensures
        pix_pow(q, a) + pix_pow(q, b) == pix_pow(q, a + b),
{
    reveal(pix_pow);
    lemma_pix_pow_len(q, a);
    lemma_pix_pow_len(q, b);
    lemma_pix_pow_len(q, a + b);
    broadcast use vstd::seq::group_seq_lemmas;
    assert(pix_pow(q, a) + pix_pow(q, b) =~= pix_pow(q, a + b)) by {
        assert forall|i: int| 0 <= i < a + b implies (pix_pow(q, a) + pix_pow(q, b))[i]
            == pix_pow(q, a + b)[i] by {
            if i < a {
                assert((pix_pow(q, a) + pix_pow(q, b))[i] == pix_pow(q, a)[i]);
                lemma_pix_pow_index(q, a, i);
            } else {
                assert((pix_pow(q, a) + pix_pow(q, b))[i] == pix_pow(q, b)[i - a]);
                lemma_pix_pow_index(q, b, i - a);
            }
            lemma_pix_pow_index(q, a + b, i);
        }
    }
}

/// R11's literal packet output: the conversion of each of `count` samples,
/// starting at absolute stream offset `start`, in output order.
pub open spec fn lit_out(s: Seq<u8>, h: PktH, map: Seq<u8>, start: int, count: int)
    -> Seq<Seq<u8>>
    decreases count
{
    if count <= 0 {
        Seq::empty()
    } else {
        seq![conv2(h, map, s.subrange(start, start + h.sample_bytes))] + lit_out(
            s,
            h,
            map,
            start + h.sample_bytes,
            count - 1,
        )
    }
}

/// R11: every sample of a literal packet has an in-range index (indexed images
/// only); "the first out-of-range index in output order" is reported once, as
/// `ColorIndexOutOfRange`, for the whole packet.
pub open spec fn lit_ok(s: Seq<u8>, h: PktH, start: int, count: int) -> bool
    decreases count
{
    if count <= 0 {
        true
    } else if h.kind != 0 {
        lit_ok(s, h, start + h.sample_bytes, count - 1)
    } else {
        in_range(h, sindex(s.subrange(start, start + h.sample_bytes), h.sample_bytes)) && lit_ok(
            s,
            h,
            start + h.sample_bytes,
            count - 1,
        )
    }
}

/// The number of leading samples of a literal packet whose index is in range:
/// the first out-of-range sample is at offset `lit_good`, so `lit_good` pixels
/// have already been produced when that check fails.
pub open spec fn lit_good(s: Seq<u8>, h: PktH, start: int, count: int) -> int
    decreases count
{
    if count <= 0 {
        count
    } else if h.kind != 0 {
        // Non-indexed: no sample can fail the check, so the whole packet is good.
        count
    } else {
        let g = lit_good(s, h, start + h.sample_bytes, count - 1);
        if in_range(h, sindex(s.subrange(start, start + h.sample_bytes), h.sample_bytes)) {
            1 + g
        } else {
            0
        }
    }
}

/// `lit_good` is in `[0, count]`.
pub proof fn lemma_lit_good_range(s: Seq<u8>, h: PktH, start: int, count: int)
    requires
        0 <= count,
    ensures
        0 <= lit_good(s, h, start, count) <= count,
    decreases count,
{
    reveal(lit_good);
    if count <= 0 {
    } else if h.kind != 0 {
    } else {
        lemma_lit_good_range(s, h, start + h.sample_bytes, count - 1);
    }
}

/// The model's error wrapper (THEOREMS.md:13): a reachable call yields one of
/// R15's 18 API errors (`Api`), while a call on other records/offsets yields the
/// internal proof marker `Outside`, which is **not** a nineteenth error and never
/// equates to an executable `Error`.
pub enum MrErr {
    Api(crate::tga::Error),
    Outside,
}

/// R11's one continuous packet stream (`produced` and the cursor threaded).
/// `s` is the entire finite byte stream, `cur` the absolute cursor, `prod` the
/// number of pixels already produced since `produced0`, `out` the converted
/// RGBA pixels produced so far. The check order is: control presence, overrun,
/// whole-packet bytes, then each sample's index in output order; the cursor and
/// produced count are those the concrete helper holds at each check.
pub open spec fn mr_at(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    cur: int,
    prod: int,
    out: Seq<Seq<u8>>,
) -> Result<(int, int, Seq<Seq<u8>>), (MrErr, int, int, Seq<Seq<u8>>)>
    decreases (h.n - prod) as nat
{
    if cur < 0 || cur > s.len() || prod < 0 || prod > h.n {
        // OutsideDomain: a call on other records/offsets, not an API result.
        Err((MrErr::Outside, cur, prod, out))
    } else if prod == h.n {
        Ok((cur, prod, out))
    } else if cur >= s.len() {
        Err((MrErr::Api(Error::RleTruncated), cur, prod, out))
    } else {
        let control = s[cur] as int;
        let count = rle_count(control);
        if count > h.n - prod {
            Err((MrErr::Api(Error::RleOverrun), cur + 1, prod, out))
        } else {
            let start = cur + 1;
            if rle_repeat(control) {
                let pe = start + h.sample_bytes;
                if s.len() < pe {
                    Err((MrErr::Api(Error::RleTruncated), start, prod, out))
                } else {
                    let smp = s.subrange(start, pe);
                    if h.kind == 0 && !in_range(h, sindex(smp, h.sample_bytes)) {
                        Err((MrErr::Api(Error::ColorIndexOutOfRange), pe, prod, out))
                    } else {
                        mr_at(
                            s,
                            h,
                            map,
                            pe,
                            prod + count,
                            out + pix_pow(conv2(h, map, smp), count),
                        )
                    }
                }
            } else {
                let total = count * h.sample_bytes;
                let pe = start + total;
                if s.len() < pe {
                    Err((MrErr::Api(Error::RleTruncated), start, prod, out))
                } else if !lit_ok(s, h, start, count) {
                    let g = lit_good(s, h, start, count);
                    Err((MrErr::Api(Error::ColorIndexOutOfRange), pe, prod + g, out + lit_out(
                        s,
                        h,
                        map,
                        start,
                        g,
                    )))
                } else {
                    mr_at(s, h, map, pe, prod + count, out + lit_out(s, h, map, start, count))
                }
            }
        }
    }
}

/// `MR(s,H,map,cursor0,produced0)`: the packet model at the start of packet data.
pub open spec fn mr(s: Seq<u8>, h: PktH, map: Seq<u8>, cursor0: int, produced0: int)
    -> Result<(int, int, Seq<Seq<u8>>), (MrErr, int, int, Seq<Seq<u8>>)> {
    mr_at(s, h, map, cursor0, produced0, Seq::empty())
}

/// The cursor-and-produced projection of `MR`: the same ordered checks and the
/// same final cursor and produced count, without the pixel bookkeeping.
pub open spec fn mr_cur(s: Seq<u8>, h: PktH, map: Seq<u8>, cur: int, prod: int)
    -> Result<(int, int), MrErr>
    decreases (h.n - prod) as nat
{
    if cur < 0 || cur > s.len() || prod < 0 || prod > h.n {
        Err(MrErr::Outside)
    } else if prod == h.n {
        Ok((cur, prod))
    } else if cur >= s.len() {
        Err(MrErr::Api(Error::RleTruncated))
    } else {
        let control = s[cur] as int;
        let count = rle_count(control);
        if count > h.n - prod {
            Err(MrErr::Api(Error::RleOverrun))
        } else {
            let start = cur + 1;
            if rle_repeat(control) {
                let pe = start + h.sample_bytes;
                if s.len() < pe {
                    Err(MrErr::Api(Error::RleTruncated))
                } else {
                    let smp = s.subrange(start, pe);
                    if h.kind == 0 && !in_range(h, sindex(smp, h.sample_bytes)) {
                        Err(MrErr::Api(Error::ColorIndexOutOfRange))
                    } else {
                        mr_cur(s, h, map, pe, prod + count)
                    }
                }
            } else {
                let total = count * h.sample_bytes;
                let pe = start + total;
                if s.len() < pe {
                    Err(MrErr::Api(Error::RleTruncated))
                } else if !lit_ok(s, h, start, count) {
                    Err(MrErr::Api(Error::ColorIndexOutOfRange))
                } else {
                    mr_cur(s, h, map, pe, prod + count)
                }
            }
        }
    }
}

/// The pixel sequence `MR` produces from `(cur, prod)` until it stops: the
/// concatenation of every packet's converted pixels, and for a literal packet
/// whose first out-of-range sample is at offset `g`, only the `g` pixels before
/// that check.
pub open spec fn mr_pix(s: Seq<u8>, h: PktH, map: Seq<u8>, cur: int, prod: int)
    -> Seq<Seq<u8>>
    decreases (h.n - prod) as nat
{
    if cur < 0 || cur > s.len() || prod < 0 || prod > h.n {
        Seq::empty()
    } else if prod == h.n {
        Seq::empty()
    } else if cur >= s.len() {
        Seq::empty()
    } else {
        let control = s[cur] as int;
        let count = rle_count(control);
        if count > h.n - prod {
            Seq::empty()
        } else {
            let start = cur + 1;
            if rle_repeat(control) {
                let pe = start + h.sample_bytes;
                if s.len() < pe {
                    Seq::empty()
                } else {
                    let smp = s.subrange(start, pe);
                    if h.kind == 0 && !in_range(h, sindex(smp, h.sample_bytes)) {
                        Seq::empty()
                    } else {
                        pix_pow(conv2(h, map, smp), count) + mr_pix(s, h, map, pe, prod + count)
                    }
                }
            } else {
                let total = count * h.sample_bytes;
                let pe = start + total;
                if s.len() < pe {
                    Seq::empty()
                } else if !lit_ok(s, h, start, count) {
                    lit_out(s, h, map, start, lit_good(s, h, start, count))
                } else {
                    lit_out(s, h, map, start, count) + mr_pix(s, h, map, pe, prod + count)
                }
            }
        }
    }
}

/// `mr_at`'s result, projected: the cursor `mr_cur` reports and the pixels
/// `mr_pix` reports are exactly the model's own final values, and the produced
/// count advances by the pixel count.
pub proof fn lemma_mr_cur_pix_proj(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    cur: int,
    prod: int,
    out: Seq<Seq<u8>>,
)
    requires
        0 <= h.sample_bytes,
        0 <= prod <= h.n,
        cur >= 0,
    ensures
        match mr_at(s, h, map, cur, prod, out) {
            Ok(r) => mr_cur(s, h, map, cur, prod) == Ok((r.0, r.1)) && r.2 == out + mr_pix(
                s,
                h,
                map,
                cur,
                prod,
            ) && r.1 == prod + mr_pix(s, h, map, cur, prod).len(),
            Err(t) => mr_cur(s, h, map, cur, prod) == Err(t.0) && t.3 == out + mr_pix(
                s,
                h,
                map,
                cur,
                prod,
            ) && t.2 == prod + mr_pix(s, h, map, cur, prod).len(),
        },
    decreases (h.n - prod) as nat,
{
    if cur < 0 || cur > s.len() || prod < 0 || prod > h.n {
        reveal(mr_at);
        reveal(mr_cur);
        reveal(mr_pix);
    } else if prod == h.n {
        reveal(mr_at);
        reveal(mr_cur);
        reveal(mr_pix);
        broadcast use vstd::seq_lib::group_seq_lib_default;
        assert(out + mr_pix(s, h, map, cur, prod) == out);
    } else if cur >= s.len() {
        reveal(mr_at);
        reveal(mr_cur);
        reveal(mr_pix);
    } else {
        let control = s[cur] as int;
        let count = rle_count(control);
        if count > h.n - prod {
            reveal(mr_at);
            reveal(mr_cur);
            reveal(mr_pix);
        } else {
            let start = cur + 1;
            if rle_repeat(control) {
                let pe = start + h.sample_bytes;
                if s.len() < pe {
                    reveal(mr_at);
                    reveal(mr_cur);
                    reveal(mr_pix);
                } else {
                    let smp = s.subrange(start, pe);
                    if h.kind == 0 && !in_range(h, sindex(smp, h.sample_bytes)) {
                        reveal(mr_at);
                        reveal(mr_cur);
                        reveal(mr_pix);
                    } else {
                        reveal(mr_at);
                        reveal(mr_cur);
                        reveal(mr_pix);
                        lemma_pix_pow_len(conv2(h, map, smp), count);
                        let out1 = out + pix_pow(conv2(h, map, smp), count);
                        assert(out1 == out + pix_pow(conv2(h, map, smp), count));
                        lemma_mr_cur_pix_proj(s, h, map, pe, prod + count, out1);
                        assert(mr_pix(s, h, map, cur, prod) == pix_pow(conv2(h, map, smp), count)
                            + mr_pix(s, h, map, pe, prod + count));
                        vstd::seq_lib::lemma_concat_associative(
                            out,
                            pix_pow(conv2(h, map, smp), count),
                            mr_pix(s, h, map, pe, prod + count),
                        );
                        assert((out + pix_pow(conv2(h, map, smp), count)) + mr_pix(
                            s,
                            h,
                            map,
                            pe,
                            prod + count,
                        ) == out + (pix_pow(conv2(h, map, smp), count) + mr_pix(
                            s,
                            h,
                            map,
                            pe,
                            prod + count,
                        )));
                        assert(pix_pow(conv2(h, map, smp), count).len() == count);
                        assert(out1 + mr_pix(s, h, map, pe, prod + count) == out + mr_pix(
                            s,
                            h,
                            map,
                            cur,
                            prod,
                        ));
                    }
                }
            } else {
                let total = count * h.sample_bytes;
                let pe = start + total;
                if s.len() < pe {
                    reveal(mr_at);
                    reveal(mr_cur);
                    reveal(mr_pix);
                } else if !lit_ok(s, h, start, count) {
                    reveal(mr_at);
                    reveal(mr_cur);
                    reveal(mr_pix);
                    let g = lit_good(s, h, start, count);
                    lemma_lit_good_range(s, h, start, count);
                    assert(0 <= g <= count);
                    assert(s.len() >= start + g * h.sample_bytes) by (nonlinear_arith)
                        requires
                            s.len() >= start + count * h.sample_bytes,
                            0 <= g <= count,
                            h.sample_bytes >= 0,
                    ;
                    lemma_lit_out_len(s, h, map, start, g);
                    assert(mr_pix(s, h, map, cur, prod) == lit_out(s, h, map, start, g));
                    broadcast use vstd::seq_lib::group_seq_lib_default;
                    assert(out + lit_out(s, h, map, start, g) == out + mr_pix(s, h, map, cur, prod));
                } else {
                    reveal(mr_at);
                    reveal(mr_cur);
                    reveal(mr_pix);
                    lemma_lit_out_len(s, h, map, start, count);
                    let out1 = out + lit_out(s, h, map, start, count);
                    lemma_mr_cur_pix_proj(s, h, map, pe, prod + count, out1);
                    assert(mr_pix(s, h, map, cur, prod) == lit_out(s, h, map, start, count) + mr_pix(
                        s,
                        h,
                        map,
                        pe,
                        prod + count,
                    ));
                    vstd::seq_lib::lemma_concat_associative(
                        out,
                        lit_out(s, h, map, start, count),
                        mr_pix(s, h, map, pe, prod + count),
                    );
                    assert((out + lit_out(s, h, map, start, count)) + mr_pix(
                        s,
                        h,
                        map,
                        pe,
                        prod + count,
                    ) == out + (lit_out(s, h, map, start, count) + mr_pix(
                        s,
                        h,
                        map,
                        pe,
                        prod + count,
                    )));
                    assert(lit_out(s, h, map, start, count).len() == count);
                    assert(out + mr_pix(s, h, map, cur, prod) == out1 + mr_pix(
                        s,
                        h,
                        map,
                        pe,
                        prod + count,
                    ));
                }
            }
        }
    }
}

/// `lit_out` peels its last sample: `lit_out(start,k+1) == lit_out(start,k) +
/// [conv(sample k)]`.
pub proof fn lemma_lit_out_snoc(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    start: int,
    k: int,
    j: int,
)
    requires
        0 <= k,
        0 <= h.sample_bytes,
    ensures
        lit_out(s, h, map, start, k + 1) == lit_out(s, h, map, start, k) + seq![conv2(
            h,
            map,
            s.subrange(start + k * h.sample_bytes, start + k * h.sample_bytes + h.sample_bytes),
        )],
    decreases k,
{
    reveal(lit_out);
    broadcast use vstd::seq::group_seq_lemmas;
    if k == 0 {
        assert(lit_out(s, h, map, start, 0) == Seq::<Seq<u8>>::empty()) by {
            reveal(lit_out);
        }
        assert(lit_out(s, h, map, start, 1) == seq![conv2(
            h,
            map,
            s.subrange(start, start + h.sample_bytes),
        )]) by {
            reveal(lit_out);
            broadcast use vstd::seq_lib::group_seq_lib_default;
            assert(lit_out(s, h, map, start + h.sample_bytes, 0) == Seq::<Seq<u8>>::empty()) by {
                reveal(lit_out);
            }
        }
    } else {
        lemma_lit_out_snoc(s, h, map, start + h.sample_bytes, k - 1, j);
        assert(lit_out(s, h, map, start, k + 1) == seq![conv2(
            h,
            map,
            s.subrange(start, start + h.sample_bytes),
        )] + lit_out(s, h, map, start + h.sample_bytes, k)) by {
            reveal(lit_out);
        }
        assert(lit_out(s, h, map, start, k) == seq![conv2(
            h,
            map,
            s.subrange(start, start + h.sample_bytes),
        )] + lit_out(s, h, map, start + h.sample_bytes, k - 1)) by {
            reveal(lit_out);
        }
        assert(lit_out(s, h, map, start + h.sample_bytes, k) == lit_out(
            s,
            h,
            map,
            start + h.sample_bytes,
            k - 1,
        ) + seq![conv2(
            h,
            map,
            s.subrange(
                start + h.sample_bytes + (k - 1) * h.sample_bytes,
                start + h.sample_bytes + (k - 1) * h.sample_bytes + h.sample_bytes,
            ),
        )]);
        assert(start + h.sample_bytes + (k - 1) * h.sample_bytes == start + k * h.sample_bytes)
            by (nonlinear_arith)
            requires
                k >= 1,
        ;
        vstd::seq_lib::lemma_concat_associative(
            seq![conv2(h, map, s.subrange(start, start + h.sample_bytes))],
            lit_out(s, h, map, start + h.sample_bytes, k - 1),
            seq![conv2(
                h,
                map,
                s.subrange(start + k * h.sample_bytes, start + k * h.sample_bytes + h.sample_bytes),
            )],
        );
    }
}

/// The `j`th pixel of a literal packet is the conversion of its `j`th sample.
pub proof fn lemma_lit_out_index(s: Seq<u8>, h: PktH, map: Seq<u8>, start: int, count: int, j: int)
    requires
        0 <= j < count,
    ensures
        lit_out(s, h, map, start, count)[j] == conv2(
            h,
            map,
            s.subrange(start + j * h.sample_bytes, start + j * h.sample_bytes + h.sample_bytes),
        ),
    decreases count,
{
    reveal(lit_out);
    let sb = h.sample_bytes;
    let c = conv2(h, map, s.subrange(start, start + sb));
    let head = Seq::<Seq<u8>>::empty().push(c);
    let rest = lit_out(s, h, map, start + sb, count - 1);
    assert(lit_out(s, h, map, start, count) == head + rest);
    vstd::seq::lemma_seq_push_len(Seq::<Seq<u8>>::empty(), c);
    if j == 0 {
        vstd::seq::lemma_seq_add_index1_alt(head, rest, 0);
        vstd::seq::lemma_seq_push_index_same(Seq::<Seq<u8>>::empty(), c, 0);
    } else {
        lemma_lit_out_len(s, h, map, start + sb, count - 1);
        vstd::seq::lemma_seq_add_index2_alt(head, rest, j - 1);
        lemma_lit_out_index(s, h, map, start + sb, count - 1, j - 1);
        assert(start + sb + (j - 1) * sb == start + j * sb) by (nonlinear_arith)
            requires
                j >= 1,
        ;
    }
}

/// `wrote_px`: the first `k` decoded pixels have been written at their oriented
/// cells. With `k = px.len()` this is `rle_quad_ok`.
pub open spec fn wrote_px(
    out: Seq<u8>,
    px: Seq<Seq<u8>>,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    k: int,
) -> bool {
    forall|j: int| 0 <= j < k ==> #[trigger] quad_at(out, 4 * fcoord(w, hgt, fx, fy, j), px[j])
}

/// `wrote_px` is monotone in `k`.
pub proof fn lemma_wrote_px_weaken(
    out: Seq<u8>,
    px: Seq<Seq<u8>>,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    k: int,
    m: int,
)
    requires
        wrote_px(out, px, w, hgt, fx, fy, k),
        0 <= m <= k,
    ensures
        wrote_px(out, px, w, hgt, fx, fy, m),
{
    reveal(wrote_px);
}

/// `wrote_px` survives appending pixels, below the old length.
pub proof fn lemma_wrote_px_extend_right(
    out: Seq<u8>,
    px: Seq<Seq<u8>>,
    extra: Seq<Seq<u8>>,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    k: int,
)
    requires
        wrote_px(out, px, w, hgt, fx, fy, k),
        k <= px.len(),
    ensures
        wrote_px(out, px + extra, w, hgt, fx, fy, k),
{
    reveal(wrote_px);
    assert forall|j: int| 0 <= j < k implies #[trigger] quad_at(
        out,
        4 * fcoord(w, hgt, fx, fy, j),
        (px + extra)[j],
    ) by {
        vstd::seq::lemma_seq_add_index1_alt(px, extra, j);
    }
}

/// Extending the decoded sequence by a packet's pixels keeps `wrote_px` for the
/// cells the packet itself covers.
pub proof fn lemma_wrote_px_step(
    out_old: Seq<u8>,
    out_new: Seq<u8>,
    px: Seq<Seq<u8>>,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    k: int,
)
    requires
        wrote_px(out_new, px + Seq::<Seq<u8>>::empty(), w, hgt, fx, fy, k),
        wrote_px(out_old, px, w, hgt, fx, fy, k),
    ensures
        wrote_px(out_new, px, w, hgt, fx, fy, k),
{
    reveal(wrote_px);
    assert(px + Seq::<Seq<u8>>::empty() == px) by {
        broadcast use vstd::seq_lib::group_seq_lib_default;
    }
}

/// The individual step of a decoded pixel: writing pixel `k` at its oriented
/// cell extends `wrote_px` from `k` to `k+1`.
pub proof fn lemma_wrote_px_single(
    out: Seq<u8>,
    px: Seq<Seq<u8>>,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    k: int,
)
    requires
        wrote_px(out, px, w, hgt, fx, fy, k),
        k < px.len(),
        quad_at(out, 4 * fcoord(w, hgt, fx, fy, k), px[k]),
    ensures
        wrote_px(out, px, w, hgt, fx, fy, k + 1),
{
    reveal(wrote_px);
    assert forall|j: int| 0 <= j < k + 1 implies #[trigger] quad_at(
        out,
        4 * fcoord(w, hgt, fx, fy, j),
        px[j],
    ) by {
        if j < k {
        }
    }
}

/// `wrote_px` over the model's own pixel length is `rle_quad_ok`.
pub proof fn lemma_wrote_px_eq_rle_quad_ok(
    out: Seq<u8>,
    px: Seq<Seq<u8>>,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
)
    ensures
        wrote_px(out, px, w, hgt, fx, fy, px.len() as int) == rle_quad_ok(out, px, w, hgt, fx, fy),
{
    reveal(wrote_px);
    reveal(rle_quad_ok);
}

/// `wrote_px_off`: the first `k` pixels of `px` are written at their oriented
/// cells, offset by `base`: pixel `px[j]` (logical sample `base + j`) sits at
/// oriented cell `f(base + j)`. This generalizes `wrote_px` (the `base = 0`
/// case) to the arbitrary starting produced count T6 requires after the RLE
/// helper's entry.
pub open spec fn wrote_px_off(
    out: Seq<u8>,
    px: Seq<Seq<u8>>,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    base: int,
    k: int,
) -> bool {
    forall|j: int| 0 <= j < k ==> #[trigger] quad_at(out, 4 * fcoord(w, hgt, fx, fy, base + j), px[j])
}

/// `wrote_px` is the `base = 0` case of `wrote_px_off`.
pub proof fn lemma_wrote_px_off_zero(
    out: Seq<u8>,
    px: Seq<Seq<u8>>,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    k: int,
)
    ensures
        wrote_px_off(out, px, w, hgt, fx, fy, 0, k) == wrote_px(out, px, w, hgt, fx, fy, k),
{
    reveal(wrote_px_off);
    reveal(wrote_px);
    if wrote_px_off(out, px, w, hgt, fx, fy, 0, k) {
        assert(wrote_px(out, px, w, hgt, fx, fy, k)) by {
            assert forall|j: int| 0 <= j < k implies #[trigger] quad_at(
                out,
                4 * fcoord(w, hgt, fx, fy, j),
                px[j],
            ) by {
                assert(quad_at(out, 4 * fcoord(w, hgt, fx, fy, 0 + j), px[j]));
                assert(0 + j == j) by (nonlinear_arith)
                    requires
                        0 <= j,
                ;
            }
        }
    }
    if wrote_px(out, px, w, hgt, fx, fy, k) {
        assert(wrote_px_off(out, px, w, hgt, fx, fy, 0, k)) by {
            assert forall|j: int| 0 <= j < k implies #[trigger] quad_at(
                out,
                4 * fcoord(w, hgt, fx, fy, 0 + j),
                px[j],
            ) by {
                assert(0 + j == j) by (nonlinear_arith)
                    requires
                        0 <= j,
                ;
            }
        }
    }
}

/// `wrote_px_off` is monotone in `k`.
pub proof fn lemma_wrote_px_off_weaken(
    out: Seq<u8>,
    px: Seq<Seq<u8>>,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    base: int,
    k: int,
    m: int,
)
    requires
        wrote_px_off(out, px, w, hgt, fx, fy, base, k),
        0 <= m <= k,
    ensures
        wrote_px_off(out, px, w, hgt, fx, fy, base, m),
{
    reveal(wrote_px_off);
}

/// One write step at offset `base`: `wrote_px_off` over the first `i` pixels
/// survives a write that changes only the four bytes at `4*f(base+i)` and puts
/// `px[i]` there.
pub proof fn lemma_wrote_px_off_write_step(
    out_new: Seq<u8>,
    out_old: Seq<u8>,
    px: Seq<Seq<u8>>,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    base: int,
    i: int,
)
    requires
        1 <= w,
        1 <= hgt,
        0 <= base,
        px.len() <= w * hgt - base,
        0 <= i < px.len(),
        out_old.len() == 4 * (w * hgt),
        out_new.len() == out_old.len(),
        forall|b: int|
            0 <= b < out_old.len() && !(4 * fcoord(w, hgt, fx, fy, base + i) <= b < 4 * fcoord(
                w,
                hgt,
                fx,
                fy,
                base + i,
            ) + 4) ==> #[trigger] out_new[b] == out_old[b],
        quad_at(out_new, 4 * fcoord(w, hgt, fx, fy, base + i), px[i]),
        wrote_px_off(out_old, px, w, hgt, fx, fy, base, i),
    ensures
        wrote_px_off(out_new, px, w, hgt, fx, fy, base, i + 1),
{
    reveal(wrote_px_off);
    assert(0 <= base + i < w * hgt) by (nonlinear_arith)
        requires
            0 <= i < px.len(),
            px.len() <= w * hgt - base,
            0 <= base,
    ;
    assert forall|j: int| 0 <= j < i + 1 implies #[trigger] quad_at(
        out_new,
        4 * fcoord(w, hgt, fx, fy, base + j),
        px[j],
    ) by {
        if j < i {
            assert(quad_at(out_old, 4 * fcoord(w, hgt, fx, fy, base + j), px[j]));
            assert(0 <= base + j < w * hgt) by (nonlinear_arith)
                requires
                    0 <= j < i,
                    0 <= i < px.len(),
                    px.len() <= w * hgt - base,
                    0 <= base,
            ;
            lemma_fcoord_range(w, hgt, fx, fy, base + j);
            lemma_fcoord_range(w, hgt, fx, fy, base + i);
            lemma_offset_injective(w, hgt, fx, fy, base + j, base + i);
            let fj = fcoord(w, hgt, fx, fy, base + j);
            let fk = fcoord(w, hgt, fx, fy, base + i);
            assert(fj != fk);
            assert(0 <= fj < w * hgt);
            assert(0 <= fk < w * hgt);
            assert(0 <= 4 * fj) by (nonlinear_arith)
                requires
                    0 <= fj,
            ;
            assert(0 <= 4 * fk) by (nonlinear_arith)
                requires
                    0 <= fk,
            ;
            assert(4 * fj + 4 <= out_old.len()) by (nonlinear_arith)
                requires
                    0 <= fj < w * hgt,
                    out_old.len() == 4 * (w * hgt),
                    w * hgt >= 0,
            ;
            assert(4 * fk + 4 <= out_old.len()) by (nonlinear_arith)
                requires
                    0 <= fk < w * hgt,
                    out_old.len() == 4 * (w * hgt),
                    w * hgt >= 0,
            ;
            if fj < fk {
                assert(4 * fj + 4 <= 4 * fk) by (nonlinear_arith)
                    requires
                        fj < fk,
                ;
            } else {
                assert(fj > fk);
                assert(4 * fk + 4 <= 4 * fj) by (nonlinear_arith)
                    requires
                        fk < fj,
                ;
            }
            assert(out_new[4 * fj + 0] == out_old[4 * fj + 0]);
            assert(out_new[4 * fj + 1] == out_old[4 * fj + 1]);
            assert(out_new[4 * fj + 2] == out_old[4 * fj + 2]);
            assert(out_new[4 * fj + 3] == out_old[4 * fj + 3]);
            reveal(quad_at);
        }
    }
}

/// `wrote_px_off` at the model's own pixel length and offset `base` is the
/// suffix relation: every sample `base..base+px.len()` is written. Used for the
/// RLE helper's success postcondition.
pub proof fn lemma_wrote_px_off_full(
    out: Seq<u8>,
    px: Seq<Seq<u8>>,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    base: int,
)
    ensures
        wrote_px_off(out, px, w, hgt, fx, fy, base, px.len() as int) == rle_quad_ok_off(
            out,
            px,
            w,
            hgt,
            fx,
            fy,
            base,
        ),
{
    reveal(wrote_px_off);
    reveal(rle_quad_ok_off);
}

/// `wrote_px_off` with `base = 0` is `rle_quad_ok`.
pub proof fn lemma_wrote_px_off_zero_is_quad_ok(
    out: Seq<u8>,
    px: Seq<Seq<u8>>,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
)
    ensures
        rle_quad_ok_off(out, px, w, hgt, fx, fy, 0) == rle_quad_ok(out, px, w, hgt, fx, fy),
{
    reveal(rle_quad_ok_off);
    reveal(rle_quad_ok);
    if rle_quad_ok_off(out, px, w, hgt, fx, fy, 0) {
        assert(rle_quad_ok(out, px, w, hgt, fx, fy)) by {
            assert forall|j: int| 0 <= j < px.len() implies #[trigger] quad_at(
                out,
                4 * fcoord(w, hgt, fx, fy, j),
                px[j],
            ) by {
                assert(quad_at(out, 4 * fcoord(w, hgt, fx, fy, 0 + j), px[j]));
                assert(0 + j == j) by (nonlinear_arith)
                    requires
                        0 <= j,
                ;
            }
        }
    }
    if rle_quad_ok(out, px, w, hgt, fx, fy) {
        assert(rle_quad_ok_off(out, px, w, hgt, fx, fy, 0)) by {
            assert forall|j: int| 0 <= j < px.len() implies #[trigger] quad_at(
                out,
                4 * fcoord(w, hgt, fx, fy, 0 + j),
                px[j],
            ) by {
                assert(0 + j == j) by (nonlinear_arith)
                    requires
                        0 <= j,
                ;
            }
        }
    }
}

/// `wrote_px_off` survives appending pixels below the old length.
pub proof fn lemma_wrote_px_off_extend_right(
    out: Seq<u8>,
    px: Seq<Seq<u8>>,
    extra: Seq<Seq<u8>>,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    base: int,
    k: int,
)
    requires
        wrote_px_off(out, px, w, hgt, fx, fy, base, k),
        k <= px.len(),
    ensures
        wrote_px_off(out, px + extra, w, hgt, fx, fy, base, k),
{
    reveal(wrote_px_off);
    assert forall|j: int| 0 <= j < k implies #[trigger] quad_at(
        out,
        4 * fcoord(w, hgt, fx, fy, base + j),
        (px + extra)[j],
    ) by {
        vstd::seq::lemma_seq_add_index1_alt(px, extra, j);
    }
}

/// Appending a whole packet's pixels keeps `wrote_px_off` for the cells the
/// packet itself does not reach.
pub proof fn lemma_wrote_px_off_append_packet(
    out: Seq<u8>,
    px: Seq<Seq<u8>>,
    pkt: Seq<Seq<u8>>,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    base: int,
    k: int,
)
    requires
        wrote_px_off(out, px, w, hgt, fx, fy, base, k),
        k <= px.len(),
    ensures
        wrote_px_off(out, px + pkt, w, hgt, fx, fy, base, k),
{
    reveal(wrote_px_off);
    assert forall|j: int| 0 <= j < k implies #[trigger] quad_at(
        out,
        4 * fcoord(w, hgt, fx, fy, base + j),
        (px + pkt)[j],
    ) by {
        vstd::seq::lemma_seq_add_index1_alt(px, pkt, j);
    }
}

/// `mr_pix` at `(c0, prod0)` splits associatively: the outer loop's accumulator
/// algebra, generalized to a nonzero starting produced count.
pub proof fn lemma_mr_pix_assoc_from(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    c0: int,
    prod0: int,
    a: Seq<Seq<u8>>,
    x: Seq<Seq<u8>>,
    y: Seq<Seq<u8>>,
)
    requires
        mr_pix(b, p, map, c0, prod0) == a + (x + y),
    ensures
        mr_pix(b, p, map, c0, prod0) == (a + x) + y,
{
    vstd::seq_lib::lemma_concat_associative(a, x, y);
}

/// `lit_pref(s,h,start,k)`: the first `k` samples of a literal packet from
/// `start` all have in-range indices (true when the image is not indexed).
/// Defined by the same head-first recursion as `lit_ok`.
pub open spec fn lit_pref(s: Seq<u8>, h: PktH, start: int, k: int) -> bool
    decreases k
{
    if k <= 0 {
        true
    } else if h.kind != 0 {
        lit_pref(s, h, start + h.sample_bytes, k - 1)
    } else {
        in_range(h, sindex(s.subrange(start, start + h.sample_bytes), h.sample_bytes)) && lit_pref(
            s,
            h,
            start + h.sample_bytes,
            k - 1,
        )
    }
}

/// `lit_ok` at one sample implies that sample's index is in range.
pub proof fn lemma_lit_ok_implies_in_range(
    s: Seq<u8>,
    h: PktH,
    start: int,
    count: int,
    i: int,
)
    requires
        h.kind == 0,
        lit_ok(s, h, start, count),
        0 <= i < count,
    ensures
        in_range(
            h,
            sindex(
                s.subrange(start + i * h.sample_bytes, start + i * h.sample_bytes + h.sample_bytes),
                h.sample_bytes,
            ),
        ),
    decreases count,
{
    reveal(lit_ok);
    if i == 0 {
        assert(s.subrange(start + 0 * h.sample_bytes, start + 0 * h.sample_bytes + h.sample_bytes)
            == s.subrange(start, start + h.sample_bytes));
    } else {
        assert(lit_ok(s, h, start + h.sample_bytes, count - 1));
        lemma_lit_ok_implies_in_range(s, h, start + h.sample_bytes, count - 1, i - 1);
        assert(start + h.sample_bytes + (i - 1) * h.sample_bytes == start + i * h.sample_bytes)
            by (nonlinear_arith)
            requires
                i >= 1,
        ;
    }
}

/// A non-indexed image's `lit_pref` is trivially true.
pub proof fn lemma_lit_pref_nonindexed(s: Seq<u8>, h: PktH, start: int, k: int)
    requires
        h.kind != 0,
        0 <= k,
    ensures
        lit_pref(s, h, start, k),
    decreases k,
{
    reveal(lit_pref);
    if k <= 0 {
    } else {
        lemma_lit_pref_nonindexed(s, h, start + h.sample_bytes, k - 1);
    }
}

/// `lit_pref` at the packet length is `lit_ok`.
pub proof fn lemma_lit_pref_eq_ok(s: Seq<u8>, h: PktH, start: int, count: int)
    ensures
        lit_pref(s, h, start, count) == lit_ok(s, h, start, count),
    decreases count,
{
    reveal(lit_pref);
    reveal(lit_ok);
    if count <= 0 {
    } else if h.kind != 0 {
        lemma_lit_pref_eq_ok(s, h, start + h.sample_bytes, count - 1);
    } else {
        lemma_lit_pref_eq_ok(s, h, start + h.sample_bytes, count - 1);
    }
}

/// Appending one decodable sample to a `lit_pref` extends it by one.
pub proof fn lemma_lit_pref_succ(s: Seq<u8>, h: PktH, start: int, k: int)
    requires
        k >= 0,
        lit_pref(s, h, start, k),
        h.kind == 0 ==> in_range(
            h,
            sindex(s.subrange(start + k * h.sample_bytes, start + k * h.sample_bytes
                + h.sample_bytes), h.sample_bytes),
        ),
    ensures
        lit_pref(s, h, start, k + 1),
    decreases k,
{
    reveal(lit_pref);
    if k == 0 {
        assert(lit_pref(s, h, start + h.sample_bytes, 0));
        assert(start + 0 * h.sample_bytes == start) by (nonlinear_arith);
    } else if h.kind != 0 {
        assert(start + h.sample_bytes + (k - 1) * h.sample_bytes == start + k * h.sample_bytes)
            by (nonlinear_arith)
            requires
                k >= 1,
        ;
        lemma_lit_pref_succ(s, h, start + h.sample_bytes, k - 1);
    } else {
        assert(start + h.sample_bytes + (k - 1) * h.sample_bytes == start + k * h.sample_bytes)
            by (nonlinear_arith)
            requires
                k >= 1,
        ;
        lemma_lit_pref_succ(s, h, start + h.sample_bytes, k - 1);
    }
}

/// A sample that fails the index test ends the good prefix.
pub proof fn lemma_lit_pref_bad(s: Seq<u8>, h: PktH, start: int, k: int, count: int)
    requires
        h.kind == 0,
        0 <= k < count,
        lit_pref(s, h, start, k),
        !in_range(
            h,
            sindex(s.subrange(start + k * h.sample_bytes, start + k * h.sample_bytes
                + h.sample_bytes), h.sample_bytes),
        ),
    ensures
        lit_good(s, h, start, count) == k,
    decreases k,
{
    reveal(lit_pref);
    reveal(lit_good);
    assert(count >= 1) by (nonlinear_arith)
        requires
            0 <= k < count,
    ;
    if k == 0 {
        assert(s.subrange(start + 0 * h.sample_bytes, start + 0 * h.sample_bytes + h.sample_bytes)
            == s.subrange(start, start + h.sample_bytes));
        assert(!in_range(h, sindex(s.subrange(start, start + h.sample_bytes), h.sample_bytes)));
        assert(lit_good(s, h, start, count) == 0);
    } else {
        assert(in_range(h, sindex(s.subrange(start, start + h.sample_bytes), h.sample_bytes)));
        assert(lit_pref(s, h, start + h.sample_bytes, k - 1));
        assert(start + h.sample_bytes + (k - 1) * h.sample_bytes == start + k * h.sample_bytes)
            by (nonlinear_arith)
            requires
                k >= 1,
        ;
        assert(!in_range(
            h,
            sindex(s.subrange(
                start + h.sample_bytes + (k - 1) * h.sample_bytes,
                start + h.sample_bytes + (k - 1) * h.sample_bytes + h.sample_bytes,
            ), h.sample_bytes),
        ));
        assert(lit_good(s, h, start, count) == 1 + lit_good(
            s,
            h,
            start + h.sample_bytes,
            count - 1,
        ));
        lemma_lit_pref_bad(s, h, start + h.sample_bytes, k - 1, count - 1);
        assert(1 + (k - 1) == k);
    }
}

/// `lit_pref` at the whole packet means `lit_good` is the whole packet.
pub proof fn lemma_lit_pref_good_all(s: Seq<u8>, h: PktH, start: int, count: int)
    requires
        h.kind == 0,
        0 <= count,
        lit_pref(s, h, start, count),
    ensures
        lit_good(s, h, start, count) == count,
    decreases count,
{
    reveal(lit_pref);
    reveal(lit_good);
    if count <= 0 {
    } else {
        assert(in_range(h, sindex(s.subrange(start, start + h.sample_bytes), h.sample_bytes)));
        assert(lit_pref(s, h, start + h.sample_bytes, count - 1));
        assert(lit_good(s, h, start, count) == 1 + lit_good(
            s,
            h,
            start + h.sample_bytes,
            count - 1,
        ));
        lemma_lit_pref_good_all(s, h, start + h.sample_bytes, count - 1);
        assert(1 + (count - 1) == count);
    }
}

/// The same, for a non-indexed image/// The same, for a non-indexed image: every sample is good, so `lit_good` is
/// the whole packet.
pub proof fn lemma_lit_good_nonindexed(s: Seq<u8>, h: PktH, start: int, count: int)
    requires
        h.kind != 0,
        0 <= count,
    ensures
        lit_good(s, h, start, count) == count,
    decreases count,
{
    reveal(lit_good);
    if count <= 0 {
    } else {
        lemma_lit_good_nonindexed(s, h, start + h.sample_bytes, count - 1);
    }
}

/// A prefix of `lit_out` agrees with any longer prefix at the same start.
pub proof fn lemma_lit_out_prefix(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    start: int,
    a: int,
    b: int,
    j: int,
)
    requires
        0 <= j < a <= b,
    ensures
        lit_out(s, h, map, start, a)[j] == lit_out(s, h, map, start, b)[j],
{
    lemma_lit_out_index(s, h, map, start, a, j);
    lemma_lit_out_index(s, h, map, start, b, j);
}

/// A slice of a slice: with `packet = b.subrange(o, o+m)`, the window
/// `[i*sb, i*sb+sb)` of `packet` is the same bytes as the absolute window.
pub proof fn lemma_subrange_of_subrange(b: Seq<u8>, o: int, m: int, i: int, sb: int)
    requires
        0 <= o,
        0 <= m,
        0 <= i,
        0 <= sb,
        i * sb + sb <= m,
        o + m <= b.len(),
    ensures
        b.subrange(o, o + m).subrange(i * sb, i * sb + sb) == b.subrange(
            o + i * sb,
            o + i * sb + sb,
        ),
{
    broadcast use vstd::seq::group_seq_lemmas;
    let packet = b.subrange(o, o + m);
    let outer = b.subrange(o + i * sb, o + i * sb + sb);
    let inner = packet.subrange(i * sb, i * sb + sb);
    assert(inner.len() == sb) by {
        broadcast use vstd::seq::group_seq_lemmas;
    }
    assert(outer.len() == sb) by {
        broadcast use vstd::seq::group_seq_lemmas;
    }
    assert(inner =~= outer) by {
        assert forall|j: int| 0 <= j < sb implies inner[j] == #[trigger] outer[j] by {
            assert(inner[j] == packet[i * sb + j]) by {
                broadcast use vstd::seq::group_seq_lemmas;
            }
            assert(packet[i * sb + j] == b[o + (i * sb + j)]) by {
                broadcast use vstd::seq::group_seq_lemmas;
            }
            assert(outer[j] == b[o + i * sb + j]) by {
                broadcast use vstd::seq::group_seq_lemmas;
            }
            assert(o + i * sb + j == o + (i * sb + j)) by (nonlinear_arith);
        }
    }
}

/// The `k`th pixel of the current repeat packet, indexed in the stream's whole
/// pixel sequence: with the pixels before this packet in `pre`, this packet's
/// `count` copies of `c` and the rest after, position `pg0 + k` is `c`.
pub proof fn lemma_mr_pix_current_repeat(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    c0: int,
    pg0: int,
    c: Seq<u8>,
    count: int,
    rest: Seq<Seq<u8>>,
    pre: Seq<Seq<u8>>,
    k: int,
)
    requires
        mr_pix(b, p, map, c0, 0) == pre + (pix_pow(c, count) + rest),
        pre.len() == pg0,
        0 <= k < count,
    ensures
        mr_pix(b, p, map, c0, 0)[pg0 + k] == c,
{
    let pk = pix_pow(c, count);
    broadcast use vstd::seq::group_seq_lemmas;
    lemma_pix_pow_len(c, count);
    assert(mr_pix(b, p, map, c0, 0) == pre + (pk + rest));
    assert((pre + (pk + rest))[pre.len() + k] == (pk + rest)[k]) by {
        vstd::seq::lemma_seq_add_index2_alt(pre, pk + rest, k);
    }
    assert(pk[k] == c) by {
        vstd::seq::lemma_seq_add_index1_alt(pk, rest, k);
        lemma_pix_pow_index(c, count, k);
    }
    assert(pg0 + k == pre.len() + k) by (nonlinear_arith)
        requires
            pre.len() == pg0,
    ;
}

/// The `k`th pixel of the current literal packet, indexed in the stream's whole
/// pixel sequence: position `pg0 + k` is that sample's conversion.
pub proof fn lemma_mr_pix_current_lit(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    c0: int,
    pg0: int,
    start: int,
    count: int,
    rest: Seq<Seq<u8>>,
    pre: Seq<Seq<u8>>,
    k: int,
)
    requires
        mr_pix(b, p, map, c0, 0) == pre + (lit_out(b, p, map, start, count) + rest),
        pre.len() == pg0,
        0 <= k < count,
    ensures
        mr_pix(b, p, map, c0, 0)[pg0 + k] == conv2(
            p,
            map,
            b.subrange(start + k * p.sample_bytes, start + k * p.sample_bytes + p.sample_bytes),
        ),
{
    let lo = lit_out(b, p, map, start, count);
    broadcast use vstd::seq::group_seq_lemmas;
    lemma_lit_out_len(b, p, map, start, count);
    assert((pre + (lo + rest))[pre.len() + k] == (lo + rest)[k]) by {
        vstd::seq::lemma_seq_add_index2_alt(pre, lo + rest, k);
    }
    assert(lo[k] == conv2(
        p,
        map,
        b.subrange(start + k * p.sample_bytes, start + k * p.sample_bytes + p.sample_bytes),
    )) by {
        vstd::seq::lemma_seq_add_index1_alt(lo, rest, k);
        lemma_lit_out_index(b, p, map, start, count, k);
    }
    assert(pg0 + k == pre.len() + k) by (nonlinear_arith)
        requires
            pre.len() == pg0,
    ;
}

/// The RLE loop's ghost relation: the model's whole pixel sequence splits into
/// the pixels produced so far (`acc`), the current packet's pixels from
/// `(cur, prod)`, and the rest — and the model's own pixel length is `N`.
pub open spec fn mr_pix_root_prefix(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    c0: int,
    acc: Seq<Seq<u8>>,
    cur: int,
    prod: int,
    n: int,
) -> bool {
    &&& prod <= n
    &&& acc.len() == prod
    &&& mr_pix(b, p, map, c0, 0) == acc + mr_pix(b, p, map, cur, prod)
}

/// The model's pixel sequence has exactly `N - produced0` pixels when it runs
/// out of work legally, so at `produced0 = 0` it has `N`.
pub proof fn lemma_mr_pix_len(s: Seq<u8>, h: PktH, map: Seq<u8>, c0: int, prod0: int)
    requires
        0 <= prod0 <= h.n,
        0 <= h.sample_bytes,
    ensures
        mr_pix(s, h, map, c0, prod0).len() + prod0 <= h.n,
        mr_pix(s, h, map, c0, prod0).len() <= h.n,
    decreases (h.n - prod0) as nat,
{
    reveal(mr_pix);
    if prod0 == h.n {
        broadcast use vstd::seq_lib::group_seq_lib_default;
    } else if c0 < 0 || c0 > s.len() {
    } else if c0 >= s.len() {
    } else {
        let control = s[c0] as int;
        let count = rle_count(control);
        if count > h.n - prod0 {
        } else {
            let start = c0 + 1;
            if rle_repeat(control) {
                let pe = start + h.sample_bytes;
                if s.len() < pe {
                } else {
                    let smp = s.subrange(start, pe);
                    if h.kind == 0 && !in_range(h, sindex(smp, h.sample_bytes)) {
                    } else {
                        lemma_pix_pow_len(conv2(h, map, smp), count);
                        lemma_mr_pix_len(s, h, map, pe, prod0 + count);
                        broadcast use vstd::seq::group_seq_lemmas;
                    }
                }
            } else {
                let total = count * h.sample_bytes;
                let pe = start + total;
                if s.len() < pe {
                } else if !lit_ok(s, h, start, count) {
                    let g = lit_good(s, h, start, count);
                    lemma_lit_good_range(s, h, start, count);
                    lemma_lit_out_len(s, h, map, start, g);
                } else {
                    lemma_lit_out_len(s, h, map, start, count);
                    lemma_mr_pix_len(s, h, map, pe, prod0 + count);
                    broadcast use vstd::seq::group_seq_lemmas;
                }
            }
        }
    }
}

/// Unfold `mr_cur` and `mr_pix` at the packet-stream start.
pub proof fn lemma_mr_reveal(s: Seq<u8>, h: PktH, map: Seq<u8>, c0: int, prod0: int)
    requires
        0 <= prod0 <= h.n,
        0 <= h.sample_bytes,
    ensures
        mr_pix(s, h, map, c0, prod0).len() <= h.n,
{
    lemma_mr_pix_len(s, h, map, c0, prod0);
}

/// One inner write step: if every earlier cell of `out_new` still holds its
/// pixel and cell `k` holds `px[k]`, then `wrote_px` extends to `k+1`.
pub proof fn lemma_wrote_px_step_common(
    out_new: Seq<u8>,
    px: Seq<Seq<u8>>,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    k: int,
)
    requires
        forall|j: int| 0 <= j < k ==> #[trigger] quad_at(out_new, 4 * fcoord(w, hgt, fx, fy, j), px[j]),
        quad_at(out_new, 4 * fcoord(w, hgt, fx, fy, k), px[k]),
    ensures
        wrote_px(out_new, px, w, hgt, fx, fy, k + 1),
{
    reveal(wrote_px);
    assert forall|j: int| 0 <= j < k + 1 implies #[trigger] quad_at(
        out_new,
        4 * fcoord(w, hgt, fx, fy, j),
        px[j],
    ) by {
        if j < k {
        }
    }
}

/// Appending a single pixel to the decoded sequence: `wrote_px` at `k` carries
/// over, and the new pixel's cell is the new last one.
pub proof fn lemma_wrote_px_snoc(
    out: Seq<u8>,
    px: Seq<Seq<u8>>,
    c: Seq<u8>,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    k: int,
)
    requires
        wrote_px(out, px, w, hgt, fx, fy, k),
        k == px.len(),
        quad_at(out, 4 * fcoord(w, hgt, fx, fy, k), c),
    ensures
        wrote_px(out, px + seq![c], w, hgt, fx, fy, k + 1),
{
    reveal(wrote_px);
    assert forall|j: int| 0 <= j < k + 1 implies #[trigger] quad_at(
        out,
        4 * fcoord(w, hgt, fx, fy, j),
        (px + seq![c])[j],
    ) by {
        if j < k {
            vstd::seq::lemma_seq_add_index1_alt(px, seq![c], j);
        } else {
            broadcast use vstd::seq::group_seq_lemmas;
            assert((px + seq![c])[k] == c);
        }
    }
}

/// Appending a whole packet's pixels: `wrote_px` at `k` carries over.
pub proof fn lemma_wrote_px_append_packet(
    out: Seq<u8>,
    px: Seq<Seq<u8>>,
    pkt: Seq<Seq<u8>>,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    k: int,
)
    requires
        wrote_px(out, px, w, hgt, fx, fy, k),
        k <= px.len(),
    ensures
        wrote_px(out, px + pkt, w, hgt, fx, fy, k),
{
    reveal(wrote_px);
    assert forall|j: int| 0 <= j < k implies #[trigger] quad_at(
        out,
        4 * fcoord(w, hgt, fx, fy, j),
        (px + pkt)[j],
    ) by {
        vstd::seq::lemma_seq_add_index1_alt(px, pkt, j);
    }
}

/// The `i`th pixel of a packet appended after `pre` (with `pre.len() == g0`) is
/// the packet's own `i`th pixel.
pub proof fn lemma_packet_pixel_at(
    pre: Seq<Seq<u8>>,
    pkt: Seq<Seq<u8>>,
    g0: int,
    i: int,
)
    requires
        pre.len() == g0,
        0 <= i < pkt.len(),
    ensures
        (pre + pkt)[g0 + i] == pkt[i],
{
    vstd::seq::lemma_seq_add_index2_alt(pre, pkt, i);
    assert(g0 + i == pre.len() + i) by (nonlinear_arith)
        requires
            pre.len() == g0,
    ;
}

/// `mr_pix` at the packet-stream start splits into the current packet and the
/// rest, for the repeat case as the concrete loop sees it.
pub proof fn lemma_mr_split_repeat(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    c0: int,
    cur: int,
    prod: int,
    c: Seq<u8>,
    count: int,
)
    requires
        0 <= cur < b.len(),
        0 <= prod < p.n,
        0 <= p.sample_bytes,
        rle_count(b[cur] as int) <= p.n - prod,
        rle_repeat(b[cur] as int),
        b.len() >= cur + 1 + p.sample_bytes,
        p.kind == 0 ==> in_range(
            p,
            sindex(b.subrange(cur + 1, cur + 1 + p.sample_bytes), p.sample_bytes),
        ),
        c == conv2(p, map, b.subrange(cur + 1, cur + 1 + p.sample_bytes)),
        count == rle_count(b[cur] as int),
    ensures
        mr_pix(b, p, map, cur, prod) == pix_pow(c, count) + mr_pix(
            b,
            p,
            map,
            cur + 1 + p.sample_bytes,
            prod + count,
        ),
        mr_cur(b, p, map, cur, prod) == mr_cur(b, p, map, cur + 1 + p.sample_bytes, prod + count),
{
    reveal(mr_pix);
    reveal(mr_cur);
}

/// A literal packet's pixel contribution plus the continuation: the model's
/// whole output for a literal packet at `start` followed by `rest`. `count` is
/// the packet's run count and `rest` the pixels the model produces after it.
/// When the packet is not fully available the contribution is empty; when some
/// sample's index is out of range only the good prefix is produced.
pub open spec fn lit_pix(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    start: int,
    count: int,
    rest: Seq<Seq<u8>>,
) -> Seq<Seq<u8>> {
    if s.len() < start + count * h.sample_bytes {
        Seq::empty()
    } else if !lit_ok(s, h, start, count) {
        lit_out(s, h, map, start, lit_good(s, h, start, count))
    } else {
        lit_out(s, h, map, start, count) + rest
    }
}

/// One literal-packet step: a sample with an in-range index is produced and the
/// rest of the packet follows.
pub proof fn lemma_lit_pix_step(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    start: int,
    count: int,
    rest: Seq<Seq<u8>>,
)
    requires
        1 <= count,
        s.len() >= start + count * h.sample_bytes,
        lit_ok(s, h, start, count),
        h.sample_bytes >= 0,
    ensures
        lit_pix(s, h, map, start, count, rest) == seq![conv2(
            h,
            map,
            s.subrange(start, start + h.sample_bytes),
        )] + lit_pix(s, h, map, start + h.sample_bytes, count - 1, rest),
{
    reveal(lit_pix);
    reveal(lit_ok);
    reveal(lit_out);
    reveal(lit_good);
    lemma_lit_out_len(s, h, map, start, count);
    assert(start + h.sample_bytes + (count - 1) * h.sample_bytes == start + count * h.sample_bytes)
        by (nonlinear_arith)
        requires
            count >= 1,
            h.sample_bytes >= 0,
    ;
    if h.kind == 0 {
        assert(lit_ok(s, h, start + h.sample_bytes, count - 1));
        lemma_lit_pref_eq_ok(s, h, start + h.sample_bytes, count - 1);
        lemma_lit_pref_good_all(s, h, start + h.sample_bytes, count - 1);
        assert(lit_pix(s, h, map, start + h.sample_bytes, count - 1, rest) == lit_out(
            s,
            h,
            map,
            start + h.sample_bytes,
            count - 1,
        ) + rest);
    } else {
        assert(lit_ok(s, h, start + h.sample_bytes, count - 1));
        lemma_lit_good_nonindexed(s, h, start + h.sample_bytes, count - 1);
        assert(lit_pix(s, h, map, start + h.sample_bytes, count - 1, rest) == lit_out(
            s,
            h,
            map,
            start + h.sample_bytes,
            count - 1,
        ) + rest);
    }
    assert(lit_out(s, h, map, start, count) == seq![conv2(
        h,
        map,
        s.subrange(start, start + h.sample_bytes),
    )] + lit_out(s, h, map, start + h.sample_bytes, count - 1)) by {
        reveal(lit_out);
    }
    vstd::seq_lib::lemma_concat_associative(
        seq![conv2(h, map, s.subrange(start, start + h.sample_bytes))],
        lit_out(s, h, map, start + h.sample_bytes, count - 1),
        rest,
    );
    assert(lit_out(s, h, map, start, count) + rest == seq![conv2(
        h,
        map,
        s.subrange(start, start + h.sample_bytes),
    )] + (lit_out(s, h, map, start + h.sample_bytes, count - 1) + rest));
    assert(lit_pix(s, h, map, start, count, rest) == lit_out(s, h, map, start, count) + rest);
    assert(lit_pix(s, h, map, start + h.sample_bytes, count - 1, rest) == lit_out(
        s,
        h,
        map,
        start + h.sample_bytes,
        count - 1,
    ) + rest);
}

/// An empty literal packet contributes only the continuation.
pub proof fn lemma_lit_pix_zero(s: Seq<u8>, h: PktH, map: Seq<u8>, start: int, rest: Seq<Seq<u8>>)
    requires
        start <= s.len(),
    ensures
        lit_pix(s, h, map, start, 0, rest) == rest,
{
    reveal(lit_pix);
    reveal(lit_ok);
    reveal(lit_out);
    broadcast use vstd::seq::group_seq_lemmas;
    assert(lit_ok(s, h, start, 0));
    assert(lit_out(s, h, map, start, 0) == Seq::<Seq<u8>>::empty());
    assert(lit_out(s, h, map, start, 0) + rest == rest);
}

/// `lit_good` at a packet whose samples are all good.
pub proof fn lemma_lit_good_good_all(s: Seq<u8>, h: PktH, start: int, count: int)
    requires
        h.kind == 0,
        0 <= count,
        lit_ok(s, h, start, count),
    ensures
        lit_good(s, h, start, count) == count,
{
    lemma_lit_pref_eq_ok(s, h, start, count);
    lemma_lit_pref_good_all(s, h, start, count);
}

/// `mr_pix` at a present, fitting literal packet unfolds to `lit_pix`.
pub proof fn lemma_mr_pix_unfold_lit(s: Seq<u8>, h: PktH, map: Seq<u8>, cur: int, prod: int)
    requires
        0 <= cur < s.len(),
        0 <= prod < h.n,
        rle_count(s[cur] as int) <= h.n - prod,
        !rle_repeat(s[cur] as int),
        s.len() >= cur + 1 + rle_count(s[cur] as int) * h.sample_bytes,
    ensures
        mr_pix(s, h, map, cur, prod) == lit_pix(
            s,
            h,
            map,
            cur + 1,
            rle_count(s[cur] as int),
            mr_pix(s, h, map, cur + 1 + rle_count(s[cur] as int) * h.sample_bytes, prod + rle_count(
                s[cur] as int,
            )),
        ),
{
    reveal(mr_pix);
    reveal(lit_pix);
}

/// `mr_pix` at a present repeat packet unfolds to the packet copies plus the
/// rest.
pub proof fn lemma_mr_pix_unfold_rep(s: Seq<u8>, h: PktH, map: Seq<u8>, cur: int, prod: int)
    requires
        0 <= cur < s.len(),
        0 <= prod < h.n,
        rle_count(s[cur] as int) <= h.n - prod,
        rle_repeat(s[cur] as int),
        s.len() >= cur + 1 + h.sample_bytes,
        h.kind == 0 ==> in_range(
            h,
            sindex(s.subrange(cur + 1, cur + 1 + h.sample_bytes), h.sample_bytes),
        ),
    ensures
        mr_pix(s, h, map, cur, prod) == pix_pow(
            conv2(h, map, s.subrange(cur + 1, cur + 1 + h.sample_bytes)),
            rle_count(s[cur] as int),
        ) + mr_pix(s, h, map, cur + 1 + h.sample_bytes, prod + rle_count(s[cur] as int)),
        mr_cur(s, h, map, cur, prod) == mr_cur(
            s,
            h,
            map,
            cur + 1 + h.sample_bytes,
            prod + rle_count(s[cur] as int),
        ),
{
    reveal(mr_pix);
    reveal(mr_cur);
}

/// `mr_cur` at a present, fitting literal packet unfolds to the rest.
pub proof fn lemma_mr_cur_unfold_lit(s: Seq<u8>, h: PktH, map: Seq<u8>, cur: int, prod: int)
    requires
        0 <= cur < s.len(),
        0 <= prod < h.n,
        rle_count(s[cur] as int) <= h.n - prod,
        !rle_repeat(s[cur] as int),
        s.len() >= cur + 1 + rle_count(s[cur] as int) * h.sample_bytes,
        lit_ok(s, h, cur + 1, rle_count(s[cur] as int)),
    ensures
        mr_cur(s, h, map, cur, prod) == mr_cur(
            s,
            h,
            map,
            cur + 1 + rle_count(s[cur] as int) * h.sample_bytes,
            prod + rle_count(s[cur] as int),
        ),
{
    reveal(mr_cur);
}

/// A literal packet that fails its first sample's index produces no pixels.
pub proof fn lemma_lit_pix_bad0(s: Seq<u8>, h: PktH, map: Seq<u8>, start: int, count: int, rest: Seq<Seq<u8>>)
    requires
        1 <= count,
        s.len() >= start + count * h.sample_bytes,
        h.kind == 0,
        !in_range(h, sindex(s.subrange(start, start + h.sample_bytes), h.sample_bytes)),
    ensures
        lit_pix(s, h, map, start, count, rest) == Seq::empty(),
{
    reveal(lit_pix);
    reveal(lit_good);
    reveal(lit_out);
    broadcast use vstd::seq_lib::group_seq_lib_default;
    assert(lit_good(s, h, start, count) == 0);
    assert(lit_out(s, h, map, start, 0) == Seq::<Seq<u8>>::empty()) by {
        reveal(lit_out);
    }
}

/// A cell of a repeated pixel block: position `pre.len()+i` of
/// `pre + c^count` is `c`.
pub proof fn lemma_pix_pow_at(pre: Seq<Seq<u8>>, c: Seq<u8>, count: int, i: int)
    requires
        0 <= i < count,
    ensures
        (pre + pix_pow(c, count))[pre.len() + i] == c,
{
    vstd::seq::lemma_seq_add_index2_alt(pre, pix_pow(c, count), i);
    lemma_pix_pow_index(c, count, i);
}

/// A cell of a literal packet appended after `pre`: position `pre.len()+i` of
/// `pre + lit_out(start,count)` is sample `i`'s conversion.
pub proof fn lemma_lit_out_at_abs(
    pre: Seq<Seq<u8>>,
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    start: int,
    count: int,
    i: int,
)
    requires
        0 <= i < count,
    ensures
        (pre + lit_out(s, h, map, start, count))[pre.len() + i] == conv2(
            h,
            map,
            s.subrange(start + i * h.sample_bytes, start + i * h.sample_bytes + h.sample_bytes),
        ),
{
    let lo = lit_out(s, h, map, start, count);
    lemma_lit_out_len(s, h, map, start, count);
    vstd::seq::lemma_seq_add_index2_alt(pre, lo, i);
    lemma_lit_out_index(s, h, map, start, count, i);
}

/// Unpack an outside-write frame condition at one index.
pub proof fn lemma_outside_write(
    out_new: Seq<u8>,
    out_old: Seq<u8>,
    off_old: int,
    i: int,
)
    requires
        out_new.len() == out_old.len(),
        forall|t: int| 0 <= t < out_old.len() && !(off_old <= t < off_old + 4) ==> #[trigger] out_new[t]
            == out_old[t],
        0 <= i < out_old.len(),
        !(off_old <= i < off_old + 4),
    ensures
        out_new[i] == out_old[i],
{
}

/// A four-byte group at `off_new` is unchanged by a write at `off_old`, when the
/// two groups do not overlap and everything outside `off_old..off_old+4` is
/// unchanged.
pub proof fn lemma_quad_unchanged(
    out_new: Seq<u8>,
    out_old: Seq<u8>,
    off_new: int,
    off_old: int,
    c: Seq<u8>,
)
    requires
        out_new.len() == out_old.len(),
        0 <= off_new,
        off_new + 4 <= out_old.len(),
        0 <= off_old,
        off_old + 4 <= out_old.len(),
        off_new + 4 <= off_old || off_old + 4 <= off_new,
        quad_at(out_old, off_new, c),
        forall|i: int| 0 <= i < out_old.len() && !(off_old <= i < off_old + 4) ==> #[trigger] out_new[i]
            == out_old[i],
    ensures
        quad_at(out_new, off_new, c),
{
    reveal(quad_at);
    if off_new + 4 <= off_old {
        assert(!(off_old <= off_new + 0 < off_old + 4)) by (nonlinear_arith)
            requires
                off_new + 4 <= off_old,
        ;
        assert(!(off_old <= off_new + 1 < off_old + 4)) by (nonlinear_arith)
            requires
                off_new + 4 <= off_old,
        ;
        assert(!(off_old <= off_new + 2 < off_old + 4)) by (nonlinear_arith)
            requires
                off_new + 4 <= off_old,
        ;
        assert(!(off_old <= off_new + 3 < off_old + 4)) by (nonlinear_arith)
            requires
                off_new + 4 <= off_old,
        ;
    } else {
        assert(off_old + 4 <= off_new);
        assert(!(off_old <= off_new + 0 < off_old + 4)) by (nonlinear_arith)
            requires
                off_old + 4 <= off_new,
        ;
        assert(!(off_old <= off_new + 1 < off_old + 4)) by (nonlinear_arith)
            requires
                off_old + 4 <= off_new,
        ;
        assert(!(off_old <= off_new + 2 < off_old + 4)) by (nonlinear_arith)
            requires
                off_old + 4 <= off_new,
        ;
        assert(!(off_old <= off_new + 3 < off_old + 4)) by (nonlinear_arith)
            requires
                off_old + 4 <= off_new,
        ;
    }
    assert(off_new + 3 < out_old.len()) by (nonlinear_arith)
        requires
            off_new + 4 <= out_old.len(),
    ;
    lemma_outside_write(out_new, out_old, off_old, off_new);
    lemma_outside_write(out_new, out_old, off_old, off_new + 1);
    lemma_outside_write(out_new, out_old, off_old, off_new + 2);
    lemma_outside_write(out_new, out_old, off_old, off_new + 3);
    assert(out_new[off_new] == out_old[off_new]);
    assert(out_new[off_new + 1] == out_old[off_new + 1]);
    assert(out_new[off_new + 2] == out_old[off_new + 2]);
    assert(out_new[off_new + 3] == out_old[off_new + 3]);
    reveal(quad_at);
}

/// One write step of a decoded-pixel loop: `wrote_px` over the first `k` cells
/// of `out_old` survives a write that changes only the four bytes at
/// `4*f(k)` and puts `px[k]` there.
pub proof fn lemma_wrote_px_write_step(
    out_new: Seq<u8>,
    out_old: Seq<u8>,
    px: Seq<Seq<u8>>,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    k: int,
)
    requires
        1 <= w,
        1 <= hgt,
        px.len() <= w * hgt,
        0 <= k < px.len(),
        out_old.len() == 4 * (w * hgt),
        out_new.len() == out_old.len(),
        forall|i: int| 0 <= i < out_old.len() && !(4 * fcoord(w, hgt, fx, fy, k) <= i < 4
            * fcoord(w, hgt, fx, fy, k) + 4) ==> #[trigger] out_new[i] == out_old[i],
        quad_at(out_new, 4 * fcoord(w, hgt, fx, fy, k), px[k]),
        wrote_px(out_old, px, w, hgt, fx, fy, k),
    ensures
        wrote_px(out_new, px, w, hgt, fx, fy, k + 1),
{
    reveal(wrote_px);
    assert(0 <= k < w * hgt);
    assert forall|j: int| 0 <= j < k + 1 implies #[trigger] quad_at(
        out_new,
        4 * fcoord(w, hgt, fx, fy, j),
        px[j],
    ) by {
        if j < k {
            assert(quad_at(out_old, 4 * fcoord(w, hgt, fx, fy, j), px[j]));
            assert(0 <= j < w * hgt);
            lemma_fcoord_range(w, hgt, fx, fy, j);
            lemma_fcoord_range(w, hgt, fx, fy, k);
            lemma_offset_injective(w, hgt, fx, fy, j, k);
            let fj = fcoord(w, hgt, fx, fy, j);
            let fk = fcoord(w, hgt, fx, fy, k);
            assert(fj != fk);
            assert(0 <= fj < w * hgt);
            assert(0 <= fk < w * hgt);
            assert(0 <= 4 * fj) by (nonlinear_arith)
                requires
                    0 <= fj,
            ;
            assert(0 <= 4 * fk) by (nonlinear_arith)
                requires
                    0 <= fk,
            ;
            assert(4 * fj + 4 <= out_old.len()) by (nonlinear_arith)
                requires
                    0 <= fj < w * hgt,
                    out_old.len() == 4 * (w * hgt),
                    w * hgt >= 0,
            ;
            assert(4 * fk + 4 <= out_old.len()) by (nonlinear_arith)
                requires
                    0 <= fk < w * hgt,
                    out_old.len() == 4 * (w * hgt),
                    w * hgt >= 0,
            ;
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
            lemma_quad_unchanged(out_new, out_old, 4 * fj, 4 * fk, px[j]);
        }
    }
}

/// `q^count` peels its first pixel: `q^count == [q] + q^(count-1)`.
pub proof fn lemma_pix_pow_peel(q: Seq<u8>, count: int)
    requires
        count >= 1,
    ensures
        pix_pow(q, count) == seq![q] + pix_pow(q, count - 1),
{
    reveal(pix_pow);
    broadcast use vstd::seq::group_seq_lemmas;
    assert(pix_pow(q, count) =~= seq![q] + pix_pow(q, count - 1)) by {
        assert forall|i: int| 0 <= i < count implies pix_pow(q, count)[i] == (seq![q] + pix_pow(
            q,
            count - 1,
        ))[i] by {
            if i == 0 {
                vstd::seq::lemma_seq_add_index1_alt(seq![q], pix_pow(q, count - 1), 0);
                broadcast use vstd::seq::group_seq_lemmas;
            } else {
                vstd::seq::lemma_seq_add_index2_alt(seq![q], pix_pow(q, count - 1), i - 1);
                assert(pix_pow(q, count - 1)[i - 1] == q);
            }
        }
    }
}

/// A literal packet's contribution peels its first sample, whenever that
/// sample's index is in range: `lit_pix(start,count,rest) == [conv(sample 0)] +
/// lit_pix(start+sb,count-1,rest)`.
pub proof fn lemma_lit_pix_peel(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    start: int,
    count: int,
    rest: Seq<Seq<u8>>,
)
    requires
        1 <= count,
        h.sample_bytes >= 0,
        s.len() >= start + count * h.sample_bytes,
        h.kind == 0 ==> in_range(h, sindex(s.subrange(start, start + h.sample_bytes), h.sample_bytes)),
    ensures
        lit_pix(s, h, map, start, count, rest) == seq![conv2(
            h,
            map,
            s.subrange(start, start + h.sample_bytes),
        )] + lit_pix(s, h, map, start + h.sample_bytes, count - 1, rest),
{
    reveal(lit_pix);
    reveal(lit_ok);
    reveal(lit_good);
    reveal(lit_out);
    let sb = h.sample_bytes;
    let c = conv2(h, map, s.subrange(start, start + sb));
    assert(s.len() >= start + sb + (count - 1) * sb) by (nonlinear_arith)
        requires
            s.len() >= start + count * sb,
            count >= 1,
            sb >= 0,
    ;
    if !lit_ok(s, h, start, count) {
        let g = lit_good(s, h, start, count);
        lemma_lit_good_range(s, h, start, count);
        if h.kind == 0 {
            assert(g == 1 + lit_good(s, h, start + sb, count - 1));
            assert(lit_good(s, h, start + sb, count - 1) == g - 1) by (nonlinear_arith)
                requires
                    g == 1 + lit_good(s, h, start + sb, count - 1),
            ;
            assert(!lit_ok(s, h, start + sb, count - 1));
        } else {
            lemma_lit_good_nonindexed(s, h, start, count);
            lemma_lit_good_nonindexed(s, h, start + sb, count - 1);
            lemma_lit_pref_nonindexed(s, h, start, count);
            lemma_lit_pref_eq_ok(s, h, start, count);
            assert(lit_ok(s, h, start, count));
        }
        assert(lit_pix(s, h, map, start, count, rest) == lit_out(s, h, map, start, g));
        assert(lit_pix(s, h, map, start + sb, count - 1, rest) == lit_out(
            s,
            h,
            map,
            start + sb,
            lit_good(s, h, start + sb, count - 1),
        ));
        assert(lit_out(s, h, map, start, g) == seq![c] + lit_out(s, h, map, start + sb, g - 1)) by {
            reveal(lit_out);
            assert(lit_good(s, h, start, count) == 1 + lit_good(s, h, start + sb, count - 1)) by {
                reveal(lit_good);
            }
            lemma_lit_good_range(s, h, start + sb, count - 1);
            assert(lit_good(s, h, start, count) >= 1) by (nonlinear_arith)
                requires
                    lit_good(s, h, start, count) == 1 + lit_good(s, h, start + sb, count - 1),
                    lit_good(s, h, start + sb, count - 1) >= 0,
            ;
        }
        assert(lit_out(s, h, map, start + sb, g - 1) == lit_out(
            s,
            h,
            map,
            start + sb,
            lit_good(s, h, start + sb, count - 1),
        ));
    } else {
        assert(lit_pix(s, h, map, start, count, rest) == lit_out(s, h, map, start, count) + rest);
        if h.kind == 0 {
            assert(lit_ok(s, h, start + sb, count - 1));
            lemma_lit_pref_eq_ok(s, h, start + sb, count - 1);
            lemma_lit_pref_good_all(s, h, start + sb, count - 1);
            assert(lit_good(s, h, start + sb, count - 1) == count - 1);
            assert(lit_pix(s, h, map, start + sb, count - 1, rest) == lit_out(
                s,
                h,
                map,
                start + sb,
                count - 1,
            ) + rest);
        } else {
            lemma_lit_pref_nonindexed(s, h, start, count);
            lemma_lit_pref_eq_ok(s, h, start, count);
            assert(lit_ok(s, h, start, count));
            assert(lit_ok(s, h, start + sb, count - 1));
            lemma_lit_good_nonindexed(s, h, start + sb, count - 1);
            assert(lit_pix(s, h, map, start + sb, count - 1, rest) == lit_out(
                s,
                h,
                map,
                start + sb,
                count - 1,
            ) + rest) by {
                reveal(lit_pix);
                reveal(lit_ok);
            }

            assert(lit_pix(s, h, map, start + sb, count - 1, rest) == lit_out(
                s,
                h,
                map,
                start + sb,
                count - 1,
            ) + rest);
        }
        assert(lit_out(s, h, map, start, count) == seq![c] + lit_out(s, h, map, start + sb, count - 1))
            by {
            reveal(lit_out);
        }
        vstd::seq_lib::lemma_concat_associative(
            seq![c],
            lit_out(s, h, map, start + sb, count - 1),
            rest,
        );
        assert(lit_out(s, h, map, start, count) + rest == seq![c] + (lit_out(
            s,
            h,
            map,
            start + sb,
            count - 1,
        ) + rest));
    }
}

/// One repeat-packet copy: the block peels its first copy.
pub proof fn lemma_rep_pix_peel(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    c: Seq<u8>,
    count: int,
    rest: Seq<Seq<u8>>,
)
    requires
        1 <= count,
        c == conv2(p, map, b.subrange(0, 0)) || true,
    ensures
        pix_pow(c, count) + rest == seq![c] + (pix_pow(c, count - 1) + rest),
{
    lemma_pix_pow_peel(c, count);
    vstd::seq_lib::lemma_concat_associative(seq![c], pix_pow(c, count - 1), rest);
}



/// A repeat packet's split, in the exact shape the concrete loop sees: with
/// `count` the control's run count, `cur_after = cur0+1+sample_bytes` and
/// `prod_after = prod+count`, the model's pixels from `(cur0,prod)` are the
/// packet's `count` copies of `c` followed by the rest.
pub proof fn lemma_mr_packet_split_rep(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    cur0: int,
    prod: int,
    count: int,
    c: Seq<u8>,
)
    requires
        0 <= cur0 < b.len(),
        0 <= prod < p.n,
        0 <= p.sample_bytes,
        count == rle_count(b[cur0] as int),
        count <= p.n - prod,
        rle_repeat(b[cur0] as int),
        b.len() >= cur0 + 1 + p.sample_bytes,
        p.kind == 0 ==> in_range(
            p,
            sindex(b.subrange(cur0 + 1, cur0 + 1 + p.sample_bytes), p.sample_bytes),
        ),
        c == conv2(p, map, b.subrange(cur0 + 1, cur0 + 1 + p.sample_bytes)),
    ensures
        mr_pix(b, p, map, cur0, prod) == pix_pow(c, count) + mr_pix(
            b,
            p,
            map,
            cur0 + 1 + p.sample_bytes,
            prod + count,
        ),
        mr_cur(b, p, map, cur0, prod) == mr_cur(b, p, map, cur0 + 1 + p.sample_bytes, prod + count),
{
    reveal(mr_pix);
    reveal(mr_cur);
}

/// A literal packet's split, in the exact shape the concrete loop sees, for a
/// packet whose samples all have in-range indices.
pub proof fn lemma_mr_packet_split_lit(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    cur0: int,
    prod: int,
    count: int,
)
    requires
        0 <= cur0 < b.len(),
        0 <= prod < p.n,
        0 <= p.sample_bytes,
        count == rle_count(b[cur0] as int),
        count <= p.n - prod,
        !rle_repeat(b[cur0] as int),
        b.len() >= cur0 + 1 + count * p.sample_bytes,
        lit_ok(b, p, cur0 + 1, count),
    ensures
        mr_pix(b, p, map, cur0, prod) == lit_out(b, p, map, cur0 + 1, count) + mr_pix(
            b,
            p,
            map,
            cur0 + 1 + count * p.sample_bytes,
            prod + count,
        ),
        mr_cur(b, p, map, cur0, prod) == mr_cur(b, p, map, cur0 + 1 + count * p.sample_bytes, prod
            + count),
{
    reveal(mr_pix);
    reveal(mr_cur);
    reveal(lit_pix);
}

/// `wrote_samples` over all `N` samples is `rle_quad_ok` on the input-order
/// conversions.
pub proof fn lemma_wrote_samples_eq_quad_ok(
    out: Seq<u8>,
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    c0: int,
    sb: int,
    k: int,
)
    requires
        wrote_samples(out, b, p, map, w, hgt, fx, fy, c0, sb, k),
        k == p.n,
        sb == p.sample_bytes,
        w * hgt == p.n,
        acc_conv(b, p, map, c0, k).len() == k,
    ensures
        rle_quad_ok(out, acc_conv(b, p, map, c0, k), w, hgt, fx, fy),
{
    reveal(wrote_samples);
    reveal(rle_quad_ok);
    assert forall|j: int| 0 <= j < k implies #[trigger] quad_at(
        out,
        4 * fcoord(w, hgt, fx, fy, j),
        acc_conv(b, p, map, c0, k)[j],
    ) by {
        assert(0 <= j < k);
        assert(quad_at(
            out,
            4 * fcoord(w, hgt, fx, fy, j),
            conv2(p, map, b.subrange(c0 + j * sb, c0 + j * sb + sb)),
        ));
        lemma_acc_conv_index(b, p, map, c0, k, j);
        assert(acc_conv(b, p, map, c0, k)[j] == conv2(
            p,
            map,
            b.subrange(c0 + j * p.sample_bytes, c0 + j * p.sample_bytes + p.sample_bytes),
        ));
        assert(c0 + j * sb + sb == c0 + j * p.sample_bytes + p.sample_bytes) by (nonlinear_arith)
            requires
                sb == p.sample_bytes,
        ;
    }
}

/// `acc_conv` has exactly `k` pixels.
pub proof fn lemma_acc_conv_len(b: Seq<u8>, p: PktH, map: Seq<u8>, c0: int, k: int)
    requires
        0 <= k,
    ensures
        acc_conv(b, p, map, c0, k).len() == k,
{
    reveal(acc_conv);
    broadcast use vstd::seq::group_seq_lemmas;
}

/// The reverse error projection for the raw phase: an error of the cursor-only
/// model is the full model's first error.
pub proof fn lemma_mp_raw_err_rev(b: Seq<u8>, p: PktH, map: Seq<u8>, c0: int, e: Error)
    requires
        mp_raw_cur(b, p, c0, 0) == Err(e),
        p.n >= 0,
    ensures
        match mp_raw(b, p, map, c0) {
            Ok(_) => false,
            Err(t) => t.0 == e,
        },
{
    lemma_mp_raw_err(b, p, map, c0, e);
    reveal(mp_raw_errs);
}

/// The reverse error projection for the packet phase.
pub proof fn lemma_mr_err_rev(b: Seq<u8>, p: PktH, map: Seq<u8>, c0: int, e: MrErr)
    requires
        mr_cur(b, p, map, c0, 0) == Err(e),
        0 <= p.sample_bytes,
        p.n >= 0,
        c0 >= 0,
    ensures
        match mr(b, p, map, c0, 0) {
            Ok(_) => false,
            Err(t) => t.0 == e,
        },
{
    reveal(mr);
    lemma_mr_cur_pix_proj(b, p, map, c0, 0, Seq::<Seq<u8>>::empty());
    match mr(b, p, map, c0, 0) {
        Ok(r) => {
            assert(mr_cur(b, p, map, c0, 0) == Ok((r.0, r.1)));
        },
        Err(t) => {
            assert(mr_cur(b, p, map, c0, 0) == Err(t.0));
        },
    }
}

/// The packet phase's success gives the model's produced pixels.
pub proof fn lemma_mr_ok_pix(b: Seq<u8>, p: PktH, map: Seq<u8>, c0: int, cend: int, prod: int)
    requires
        mr_cur(b, p, map, c0, 0) == Ok((cend, prod)),
        0 <= p.sample_bytes,
        p.n >= 0,
        c0 >= 0,
    ensures
        match mr(b, p, map, c0, 0) {
            Ok(r) => r.0 == cend && r.1 == prod && r.2 == mr_pix(b, p, map, c0, 0),
            Err(_) => false,
        },
{
    reveal(mr);
    lemma_mr_cur_pix_proj(b, p, map, c0, 0, Seq::<Seq<u8>>::empty());
    broadcast use vstd::seq_lib::group_seq_lib_default;
    match mr(b, p, map, c0, 0) {
        Ok(r) => {
            assert(mr_cur(b, p, map, c0, 0) == Ok((r.0, r.1)));
            assert(r.0 == cend);
            assert(r.1 == prod);
            assert(r.2 == Seq::<Seq<u8>>::empty() + mr_pix(b, p, map, c0, 0));
        },
        Err(t) => {
            assert(mr_cur(b, p, map, c0, 0) == Err(t.0));
        },
    }
}

/// The raw phase's success gives the model's converted pixels.
pub proof fn lemma_mp_raw_ok_pix(b: Seq<u8>, p: PktH, map: Seq<u8>, c0: int, cend: int)
    requires
        mp_raw_cur(b, p, c0, 0) == Ok(cend),
        p.sample_bytes >= 1,
        raw_prefix_ok(b, p, map, c0, p.n),
    ensures
        mp_raw(b, p, map, c0) == Ok((acc_conv(b, p, map, c0, p.n), cend)),
{
    reveal(mp_raw);
    lemma_mp_raw_ok(b, p, map, c0);
    lemma_mp_raw_ok_end(b, p, map, c0, cend);
    lemma_mp_raw_cur_proj(b, p, map, c0, 0, Seq::<Seq<u8>>::empty());
    match mp_raw(b, p, map, c0) {
        Ok(r) => {
            assert(r.1 == cend);
        },
        Err(t) => {
            assert(mp_raw_cur(b, p, c0, 0) == Err(t.0));
        },
    }
}

/// `MP`'s ID end and map end for a header record.
pub open spec fn mp_id_end(m: MHModel) -> int {
    18 + (m.id_length as int)
}

pub open spec fn mp_map_end(m: MHModel) -> int {
    mp_id_end(m) + m.map_bytes
}

/// `MP` with a `TruncatedImageId` failure.
pub proof fn lemma_mp_trunc_id(b: Seq<u8>, m: MHModel)
    requires
        mh(b) == Ok(m),
        mp_id_end(m) > b.len(),
    ensures
        mp(b) == Err(Error::TruncatedImageId),
{
    reveal(mp);
    reveal(mp_after);
    reveal(mp_id_end);
}

/// `MP` with a `TruncatedColorMap` failure.
pub proof fn lemma_mp_trunc_map(b: Seq<u8>, m: MHModel)
    requires
        mh(b) == Ok(m),
        mp_id_end(m) <= b.len(),
        mp_map_end(m) > b.len(),
    ensures
        mp(b) == Err(Error::TruncatedColorMap),
{
    reveal(mp);
    reveal(mp_after);
    reveal(mp_id_end);
    reveal(mp_map_end);
}

/// `MP`'s RLE success: the packet model reached `cursor` after all `N` pixels
/// and the suffix rule holds, so `MP` returns the oriented assembly.
pub proof fn lemma_mp_rle_ok(b: Seq<u8>, m: MHModel, cursor: int, pix: Seq<Seq<u8>>)
    requires
        mh(b) == Ok(m),
        mp_id_end(m) <= b.len(),
        mp_map_end(m) <= b.len(),
        m.image_type == 9u8 || m.image_type == 10u8 || m.image_type == 11u8,
        mr_cur(b, pkt_of_mh(m), b.subrange(mp_id_end(m), mp_map_end(m)), mp_map_end(m), 0) == Ok((
            cursor,
            m.n,
        )),
        mr_pix(b, pkt_of_mh(m), b.subrange(mp_id_end(m), mp_map_end(m)), mp_map_end(m), 0) == pix,
        mp_suffix_ok(b, cursor),
    ensures
        mp(b) == Ok(
            (m.width as int, m.height as int, orient_bytes(
                pix,
                m.width as int,
                m.height as int,
                m.descriptor & 0x10u8 != 0u8,
                m.descriptor & 0x20u8 != 0u8,
                m.n,
            )),
        ),
{
    lemma_mr_ok_pix(
        b,
        pkt_of_mh(m),
        b.subrange(mp_id_end(m), mp_map_end(m)),
        mp_map_end(m),
        cursor,
        m.n,
    );
    reveal(mp);
    reveal(mp_after);
    reveal(mp_assemble);
    reveal(mp_pixels);
    reveal(mp_id_end);
    reveal(mp_map_end);
}

/// `MP`'s raw success: all `N` samples decode and the suffix rule holds, so `MP`
/// returns the oriented assembly of the input-order conversions.
pub proof fn lemma_mp_raw_ok_result(b: Seq<u8>, m: MHModel, cursor: int)
    requires
        mh(b) == Ok(m),
        mp_id_end(m) <= b.len(),
        mp_map_end(m) <= b.len(),
        !(m.image_type == 9u8 || m.image_type == 10u8 || m.image_type == 11u8),
        mp_raw_cur(b, pkt_of_mh(m), mp_map_end(m), 0) == Ok(cursor),
        raw_prefix_ok(
            b,
            pkt_of_mh(m),
            b.subrange(mp_id_end(m), mp_map_end(m)),
            mp_map_end(m),
            m.n,
        ),
        pkt_of_mh(m).sample_bytes >= 1,
        mp_suffix_ok(b, cursor),
    ensures
        mp(b) == Ok(
            (m.width as int, m.height as int, orient_bytes(
                acc_conv(
                    b,
                    pkt_of_mh(m),
                    b.subrange(mp_id_end(m), mp_map_end(m)),
                    mp_map_end(m),
                    m.n,
                ),
                m.width as int,
                m.height as int,
                m.descriptor & 0x10u8 != 0u8,
                m.descriptor & 0x20u8 != 0u8,
                m.n,
            )),
        ),
{
    lemma_mp_raw_ok_pix(
        b,
        pkt_of_mh(m),
        b.subrange(mp_id_end(m), mp_map_end(m)),
        mp_map_end(m),
        cursor,
    );
    reveal(mp);
    reveal(mp_after);
    reveal(mp_assemble);
    reveal(mp_pixels);
    reveal(mp_id_end);
    reveal(mp_map_end);
}

/// `MP`'s RLE failure.
pub proof fn lemma_mp_rle_err(b: Seq<u8>, m: MHModel, e: Error)
    requires
        mh(b) == Ok(m),
        mp_id_end(m) <= b.len(),
        mp_map_end(m) <= b.len(),
        m.image_type == 9u8 || m.image_type == 10u8 || m.image_type == 11u8,
        mr_cur(b, pkt_of_mh(m), b.subrange(mp_id_end(m), mp_map_end(m)), mp_map_end(m), 0) == Err(
            MrErr::Api(e),
        ),
        pkt_of_mh(m).sample_bytes >= 0,
        pkt_of_mh(m).n >= 0,
    ensures
        mp(b) == Err(e),
{
    lemma_mr_err_rev(
        b,
        pkt_of_mh(m),
        b.subrange(mp_id_end(m), mp_map_end(m)),
        mp_map_end(m),
        MrErr::Api(e),
    );
    reveal(mp);
    reveal(mp_after);
    reveal(mp_id_end);
    reveal(mp_map_end);
}

/// A successful `MR` on a reachable starting state has produced exactly `N`
/// pixels (the produced count only advances by packet counts that sum to `N`).
pub proof fn lemma_mr_at_ok_prod(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    cur: int,
    prod: int,
    out: Seq<Seq<u8>>,
)
    requires
        0 <= cur <= s.len(),
        0 <= prod <= h.n,
        0 <= h.sample_bytes,
        h.n <= 16777216,
    ensures
        match mr_at(s, h, map, cur, prod, out) {
            Ok(r) => r.1 == h.n,
            Err(_) => true,
        },
    decreases (h.n - prod) as nat,
{
    reveal(mr_at);
    if cur < 0 || cur > s.len() || prod < 0 || prod > h.n {
    } else if prod == h.n {
    } else if cur >= s.len() {
    } else {
        let control = s[cur] as int;
        let count = rle_count(control);
        if count > h.n - prod {
        } else {
            let start = cur + 1;
            if rle_repeat(control) {
                let pe = start + h.sample_bytes;
                if s.len() < pe {
                } else {
                    let smp = s.subrange(start, pe);
                    if h.kind == 0 && !in_range(h, sindex(smp, h.sample_bytes)) {
                    } else {
                        lemma_mr_at_ok_prod(
                            s,
                            h,
                            map,
                            pe,
                            prod + count,
                            out + pix_pow(conv2(h, map, smp), count),
                        );
                    }
                }
            } else {
                let total = count * h.sample_bytes;
                let pe = start + total;
                if s.len() < pe {
                } else if !lit_ok(s, h, start, count) {
                } else {
                    lemma_mr_at_ok_prod(
                        s,
                        h,
                        map,
                        pe,
                        prod + count,
                        out + lit_out(s, h, map, start, count),
                    );
                }
            }
        }
    }
}

/// `MP`'s raw failure.
pub proof fn lemma_mp_raw_err_result(b: Seq<u8>, m: MHModel, e: Error)
    requires
        mh(b) == Ok(m),
        mp_id_end(m) <= b.len(),
        mp_map_end(m) <= b.len(),
        !(m.image_type == 9u8 || m.image_type == 10u8 || m.image_type == 11u8),
        mp_raw_cur(b, pkt_of_mh(m), mp_map_end(m), 0) == Err(e),
        pkt_of_mh(m).n >= 0,
    ensures
        mp(b) == Err(e),
{
    lemma_mp_raw_err_rev(
        b,
        pkt_of_mh(m),
        b.subrange(mp_id_end(m), mp_map_end(m)),
        mp_map_end(m),
        e,
    );
    reveal(mp);
    reveal(mp_after);
    reveal(mp_id_end);
    reveal(mp_map_end);
}

/// `MP`'s suffix failure, RLE phase.
pub proof fn lemma_mp_trailing_rle(b: Seq<u8>, m: MHModel, cursor: int)
    requires
        mh(b) == Ok(m),
        mp_id_end(m) <= b.len(),
        mp_map_end(m) <= b.len(),
        m.image_type == 9u8 || m.image_type == 10u8 || m.image_type == 11u8,
        mr_cur(b, pkt_of_mh(m), b.subrange(mp_id_end(m), mp_map_end(m)), mp_map_end(m), 0) == Ok((
            cursor,
            m.n,
        )),
        !mp_suffix_ok(b, cursor),
    ensures
        mp(b) == Err(Error::TrailingBytes),
{
    lemma_mr_ok_pix(
        b,
        pkt_of_mh(m),
        b.subrange(mp_id_end(m), mp_map_end(m)),
        mp_map_end(m),
        cursor,
        m.n,
    );
    reveal(mp);
    reveal(mp_after);
    reveal(mp_id_end);
    reveal(mp_map_end);
    reveal(mp_suffix_ok);
}

/// `MP`'s suffix failure, raw phase.
pub proof fn lemma_mp_trailing_raw(b: Seq<u8>, m: MHModel, cursor: int)
    requires
        mh(b) == Ok(m),
        mp_id_end(m) <= b.len(),
        mp_map_end(m) <= b.len(),
        !(m.image_type == 9u8 || m.image_type == 10u8 || m.image_type == 11u8),
        mp_raw_cur(b, pkt_of_mh(m), mp_map_end(m), 0) == Ok(cursor),
        !mp_suffix_ok(b, cursor),
    ensures
        mp(b) == Err(Error::TrailingBytes),
{
    lemma_mp_raw_ok_end(b, pkt_of_mh(m), b.subrange(mp_id_end(m), mp_map_end(m)), mp_map_end(m), cursor);
    reveal(mp);
    reveal(mp_after);
    reveal(mp_id_end);
    reveal(mp_map_end);
    reveal(mp_suffix_ok);
}

/// A suffix's final 18 bytes are the input's final 18 bytes whenever the suffix
/// has at least 18 bytes.
pub proof fn lemma_suffix_tail(b: Seq<u8>, cursor: int)
    requires
        0 <= cursor,
        cursor + 18 <= b.len(),
    ensures
        b.subrange(cursor, b.len() as int).subrange(
            b.subrange(cursor, b.len() as int).len() as int - 18,
            b.subrange(cursor, b.len() as int).len() as int,
        ) == b.subrange(b.len() as int - 18, b.len() as int),
{
    let sfx = b.subrange(cursor, b.len() as int);
    assert(sfx.len() == b.len() - cursor);
    broadcast use vstd::seq::group_seq_lemmas;
    assert(sfx.subrange(sfx.len() as int - 18, sfx.len() as int) =~= b.subrange(
        b.len() as int - 18,
        b.len() as int,
    )) by {
        assert(sfx.len() as int - 18 == b.len() as int - 18 - cursor);
        assert forall|j: int| 0 <= j < 18 implies sfx.subrange(sfx.len() as int - 18,
            sfx.len() as int)[j] == #[trigger] b.subrange(b.len() as int - 18, b.len() as int)[j]
            by {
            assert(sfx.subrange(sfx.len() as int - 18, sfx.len() as int)[j] == sfx[sfx.len()
                as int - 18 + j]) by {
                broadcast use vstd::seq::group_seq_lemmas;
            }
            assert(sfx[sfx.len() as int - 18 + j] == b[cursor + (sfx.len() as int - 18 + j)]) by {
                broadcast use vstd::seq::group_seq_lemmas;
            }
            assert(sfx.len() as int - 18 + j == b.len() as int - 18 - cursor + j) by (nonlinear_arith)
                requires
                    sfx.len() == b.len() - cursor,
            ;
            assert(b[cursor + (b.len() as int - 18 - cursor + j)] == b[b.len() as int - 18 + j])
                by (nonlinear_arith)
                requires
                    cursor >= 0,
            ;
            assert(b.subrange(b.len() as int - 18, b.len() as int)[j] == b[b.len() as int - 18 + j])
                by {
                broadcast use vstd::seq::group_seq_lemmas;
            }
        }
    }
}

/// `mp_suffix_ok` from the executable's suffix tests.
pub proof fn lemma_mp_suffix_zero(b: Seq<u8>, cursor: int)
    requires
        cursor == b.len(),
    ensures
        mp_suffix_ok(b, cursor),
{
    reveal(mp_suffix_ok);
}

/// `mp_suffix_ok` from a footer-carrying suffix of at least 26 bytes.
pub proof fn lemma_mp_suffix_footer(b: Seq<u8>, cursor: int)
    requires
        0 <= cursor,
        cursor + 26 <= b.len(),
        b.subrange(cursor as int, b.len() as int).subrange(
            b.subrange(cursor as int, b.len() as int).len() as int - 18,
            b.subrange(cursor as int, b.len() as int).len() as int,
        ) == tga_signature(),
    ensures
        mp_suffix_ok(b, cursor),
{
    reveal(mp_suffix_ok);
    reveal(footer);
}

/// The executable's short-suffix rejection is the model's.
pub proof fn lemma_mp_suffix_short(b: Seq<u8>, cursor: int)
    requires
        0 <= cursor,
        cursor < b.len(),
        b.len() - cursor < 26,
    ensures
        !mp_suffix_ok(b, cursor),
{
    reveal(mp_suffix_ok);
    reveal(footer);
    broadcast use vstd::seq::group_seq_lemmas;
}

/// The executable's signature mismatch is the model's.
pub proof fn lemma_mp_suffix_bad(b: Seq<u8>, cursor: int)
    requires
        0 <= cursor,
        cursor + 26 <= b.len(),
        b.subrange(cursor as int, b.len() as int).subrange(
            b.subrange(cursor as int, b.len() as int).len() as int - 18,
            b.subrange(cursor as int, b.len() as int).len() as int,
        ) != tga_signature(),
    ensures
        !mp_suffix_ok(b, cursor),
{
    reveal(mp_suffix_ok);
    reveal(footer);
}

/// From the packet model's exact result, its cursor/produced projection and its
/// pixel sequence.
pub proof fn lemma_mr_at_ok(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    c0: int,
    cend: int,
    prod: int,
    pix: Seq<Seq<u8>>,
)
    requires
        mr(b, p, map, c0, 0) == Ok((cend, prod, pix)),
        c0 >= 0,
        0 <= p.sample_bytes,
        p.n >= 0,
    ensures
        mr_cur(b, p, map, c0, 0) == Ok((cend, prod)),
        mr_pix(b, p, map, c0, 0) == pix,
        mr_pix(b, p, map, c0, 0).len() == prod,
{
    lemma_mr_cur_pix_proj(b, p, map, c0, 0, Seq::<Seq<u8>>::empty());
    broadcast use vstd::seq_lib::group_seq_lib_default;
    reveal(mr);
    assert(Seq::<Seq<u8>>::empty() + mr_pix(b, p, map, c0, 0) == mr_pix(b, p, map, c0, 0));
}

/// From the packet model's error result, its error projection.
pub proof fn lemma_mr_at_err(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    c0: int,
    e: MrErr,
    gc: int,
    gp: int,
    gpx: Seq<Seq<u8>>,
)
    requires
        mr(b, p, map, c0, 0) == Err((e, gc, gp, gpx)),
        c0 >= 0,
        0 <= p.sample_bytes,
        p.n >= 0,
    ensures
        mr_cur(b, p, map, c0, 0) == Err(e),
{
    lemma_mr_cur_pix_proj(b, p, map, c0, 0, Seq::<Seq<u8>>::empty());
    reveal(mr);
}

/// The model's ghost error state at the packet-stream start: the first error
/// and the cursor/produced/pixels MR holds at the failing check. This is a
/// projection of `MR` itself (the manifest's "proof observations"), so a
/// concrete helper can be compared to it.
pub open spec fn mr_ghost(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    c0: int,
    p0: int,
) -> (MrErr, int, int, Seq<Seq<u8>>) {
    match mr(s, h, map, c0, p0) {
        Ok(r) => (MrErr::Api(Error::RleTruncated), r.0, r.1, r.2),
        Err(t) => t,
    }
}

/// The ghost cursor of `MR`: on `Ok`, the final cursor; on `Err`, the cursor
/// `MR` holds at the failing check (the manifest's ghost-state cursor). Its
/// branch structure is `mr_cur`'s, so the concrete loop maintains
/// `mr_gcur(c0g,p0g) == mr_gcur(cur,prod)` exactly as it maintains `mr_cur`.
pub open spec fn mr_gcur(s: Seq<u8>, h: PktH, map: Seq<u8>, cur: int, prod: int)
    -> Result<int, MrErr>
    decreases (h.n - prod) as nat
{
    if cur < 0 || cur > s.len() || prod < 0 || prod > h.n {
        Err(MrErr::Outside)
    } else if prod == h.n {
        Ok(cur)
    } else if cur >= s.len() {
        Ok(cur)
    } else {
        let control = s[cur] as int;
        let count = rle_count(control);
        if count > h.n - prod {
            Ok(cur + 1)
        } else {
            let start = cur + 1;
            if rle_repeat(control) {
                let pe = start + h.sample_bytes;
                if s.len() < pe {
                    Ok(start)
                } else {
                    let smp = s.subrange(start, pe);
                    if h.kind == 0 && !in_range(h, sindex(smp, h.sample_bytes)) {
                        Ok(pe)
                    } else {
                        mr_gcur(s, h, map, pe, prod + count)
                    }
                }
            } else {
                let total = count * h.sample_bytes;
                let pe = start + total;
                if s.len() < pe {
                    Ok(start)
                } else if !lit_ok(s, h, start, count) {
                    Ok(pe)
                } else {
                    mr_gcur(s, h, map, pe, prod + count)
                }
            }
        }
    }
}

/// `mr_gcur` is determined by `mr_cur` and the produced counts: if the cursor
/// projection is the same at `(c0,p0)` and `(cur,prod)` (same Ok/Err,
/// same value), the ghost cursor is too. Proved by induction on `N - prod`.
pub proof fn lemma_mr_gcur_agree(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    c0: int,
    p0: int,
    cur: int,
    prod: int,
)
    requires
        0 <= p0 <= h.n,
        0 <= prod <= h.n,
        0 <= h.sample_bytes,
        mr_cur(s, h, map, c0, p0) == mr_cur(s, h, map, cur, prod),
        mr_gcur(s, h, map, c0, p0) == mr_gcur(s, h, map, cur, prod),
    ensures
        mr_gcur(s, h, map, c0, p0) == mr_gcur(s, h, map, cur, prod),
{
    reveal(mr_gcur);
}

/// `mr_gcur` is `MR`'s own ghost cursor: on `Ok` it is the final cursor, and on
/// an API error it is the cursor `MR` holds at the failing check (the second
/// field of `mr_at`'s error tuple). Proved by the induction of
/// `lemma_mr_cur_pix_proj`, so a concrete helper's final cursor that equals
/// `mr_gcur` equals the ghost cursor of `MR`'s result (T6).
pub proof fn lemma_mr_gcur_proj(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    cur: int,
    prod: int,
    out: Seq<Seq<u8>>,
)
    requires
        0 <= h.sample_bytes,
        0 <= prod <= h.n,
        cur >= 0,
    ensures
        match mr_at(s, h, map, cur, prod, out) {
            Ok(r) => mr_gcur(s, h, map, cur, prod) == Ok(r.0),
            Err(t) => t.0 == MrErr::Outside || mr_gcur(s, h, map, cur, prod) == Ok(t.1),
        },
    decreases (h.n - prod) as nat,
{
    reveal(mr_at);
    reveal(mr_gcur);
    if cur < 0 || cur > s.len() || prod < 0 || prod > h.n {
    } else if prod == h.n {
    } else if cur >= s.len() {
    } else {
        let control = s[cur] as int;
        let count = rle_count(control);
        if count > h.n - prod {
        } else {
            let start = cur + 1;
            if rle_repeat(control) {
                let pe = start + h.sample_bytes;
                if s.len() < pe {
                } else {
                    let smp = s.subrange(start, pe);
                    if h.kind == 0 && !in_range(h, sindex(smp, h.sample_bytes)) {
                    } else {
                        lemma_mr_gcur_proj(
                            s,
                            h,
                            map,
                            pe,
                            prod + count,
                            out + pix_pow(conv2(h, map, smp), count),
                        );
                    }
                }
            } else {
                let total = count * h.sample_bytes;
                let pe = start + total;
                if s.len() < pe {
                } else if !lit_ok(s, h, start, count) {
                } else {
                    lemma_mr_gcur_proj(
                        s,
                        h,
                        map,
                        pe,
                        prod + count,
                        out + lit_out(s, h, map, start, count),
                    );
                }
            }
        }
    }
}

/// A control byte with bit 7 clear is not a repeat packet.
pub proof fn lemma_rle_repeat_clear(control: u8)
    requires
        (control as int) < 128,
    ensures
        !rle_repeat(control as int),
{
    reveal(rle_repeat);
}

/// Step for the literal branch whose sample `i` is the first with an
/// out-of-range index: `mr_gcur` at the start is the failing cursor
/// `cur + 1 + count*sample_bytes` (the packet end).
pub proof fn lemma_mr_gcur_step_lit_err(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    cur: int,
    prod: int,
    count: int,
    i: int,
)
    requires
        0 <= cur < s.len(),
        0 <= prod < h.n,
        0 <= h.sample_bytes,
        count == rle_count(s[cur] as int),
        count <= h.n - prod,
        !rle_repeat(s[cur] as int),
        s.len() >= cur + 1 + count * h.sample_bytes,
        0 <= i < count,
        !lit_ok(s, h, cur + 1, count),
        lit_good(s, h, cur + 1, count) == i,
    ensures
        mr_gcur(s, h, map, cur, prod) == Ok(cur + 1 + count * h.sample_bytes),
{
    reveal(mr_gcur);
    reveal(lit_ok);
}

/// Step: `mr_gcur` at the start is `mr_gcur` after a present repeat packet.
pub proof fn lemma_mr_gcur_step_rep(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    cur: int,
    prod: int,
    count: int,
)
    requires
        0 <= cur < s.len(),
        0 <= prod < h.n,
        0 <= h.sample_bytes,
        count == rle_count(s[cur] as int),
        count <= h.n - prod,
        rle_repeat(s[cur] as int),
        s.len() >= cur + 1 + h.sample_bytes,
        h.kind == 0 ==> in_range(
            h,
            sindex(s.subrange(cur + 1, cur + 1 + h.sample_bytes), h.sample_bytes),
        ),
    ensures
        mr_gcur(s, h, map, cur, prod) == mr_gcur(s, h, map, cur + 1 + h.sample_bytes, prod + count),
{
    reveal(mr_gcur);
}

/// Step: `mr_gcur` at the start is `mr_gcur` after a present, all-good literal.
pub proof fn lemma_mr_gcur_step_lit(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    cur: int,
    prod: int,
    count: int,
)
    requires
        0 <= cur < s.len(),
        0 <= prod < h.n,
        0 <= h.sample_bytes,
        count == rle_count(s[cur] as int),
        count <= h.n - prod,
        !rle_repeat(s[cur] as int),
        s.len() >= cur + 1 + count * h.sample_bytes,
        lit_ok(s, h, cur + 1, count),
    ensures
        mr_gcur(s, h, map, cur, prod) == mr_gcur(
            s,
            h,
            map,
            cur + 1 + count * h.sample_bytes,
            prod + count,
        ),
{
    reveal(mr_gcur);
}

/// The model's ghost pixels at `(cur, prod)`, whatever the branch.
pub open spec fn mr_ghost_pix(s: Seq<u8>, h: PktH, map: Seq<u8>, cur: int, prod: int)
    -> Seq<Seq<u8>> {
    mr_ghost(s, h, map, cur, prod).3
}

/// From the raw model's exact result, its cursor projection and pixels.
pub proof fn lemma_mp_raw_at_ok(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    c0: int,
    acc: Seq<Seq<u8>>,
    cend: int,
)
    requires
        mp_raw(b, p, map, c0) == Ok((acc, cend)),
        p.sample_bytes >= 0,
        p.n >= 0,
        c0 >= 0,
    ensures
        mp_raw_cur(b, p, c0, 0) == Ok(cend),
{
    lemma_mp_raw_cur_proj(b, p, map, c0, 0, Seq::<Seq<u8>>::empty());
    reveal(mp_raw);
}

/// From the raw model's error result, its error projection.
pub proof fn lemma_mp_raw_at_err(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    c0: int,
    e: Error,
    gc: int,
    gp: int,
    gpx: Seq<Seq<u8>>,
)
    requires
        mp_raw(b, p, map, c0) == Err((e, gc, gp, gpx)),
        p.n >= 0,
        c0 >= 0,
    ensures
        mp_raw_cur(b, p, c0, 0) == Err(e),
{
    lemma_mp_raw_cur_proj(b, p, map, c0, 0, Seq::<Seq<u8>>::empty());
    reveal(mp_raw);
}

/// The shape of an accepted `MP`: some `MH` record `m` built the result, and
/// the pixel sequence is exactly `N` pixels that the assembly places.
pub open spec fn mp_ok_shape(b: Seq<u8>, w: int, hgt: int, out: Seq<u8>) -> bool {
    exists|m: MHModel| mh(b) == Ok(m) && m.width as int == w && m.height as int == hgt
        && mp_id_end(m) <= b.len() && mp_map_end(m) <= b.len()
        && mp_pixels(b, m, mp_id_end(m)).len() == m.n
        && out == orient_bytes(
        mp_pixels(b, m, mp_id_end(m)),
        w,
        hgt,
        m.descriptor & 0x10u8 != 0u8,
        m.descriptor & 0x20u8 != 0u8,
        m.n,
    )
}

/// One output cell of an accepted `MP`: the manifest's T8 per-cell claim as a
/// predicate over `MP(b) = Ok(w,h,out)`. Cell `q < N` is the four bytes of the
/// model's source pixel `g(q)` (the orientation bijection's inverse, so R9's
/// assignment is unique) at the oriented position `f(g(q)) = q`; the model's
/// `mp_pixels` entry is itself R8's `conv2` of the source sample.
pub open spec fn t8_cell(b: Seq<u8>, w: int, hgt: int, out: Seq<u8>, q: int) -> bool {
    exists|m: MHModel| mh(b) == Ok(m) && m.width as int == w && m.height as int == hgt
        && m.n == w * hgt
        && ({
            let fx = m.descriptor & 0x10u8 != 0u8;
            let fy = m.descriptor & 0x20u8 != 0u8;
            let k = gcoord(w, hgt, fx, fy, q);
            let px = mp_pixels(b, m, mp_id_end(m));
            &&& 0 <= k < m.n
            &&& fcoord(w, hgt, fx, fy, k) == q
            &&& out[4 * q] == px[k][0]
            &&& out[4 * q + 1] == px[k][1]
            &&& out[4 * q + 2] == px[k][2]
            &&& out[4 * q + 3] == px[k][3]
        })
}

/// The projection lemma Finding 3 asks for: an accepted `MP(b) = Ok(w,h,out)`
/// satisfies `mp_ok_shape(b,w,h,out)`. It restates `MP`'s own success shape, so
/// the T8 per-cell statement has a real `MP` hypothesis rather than taking the
/// existential as a premise.
pub proof fn lemma_mp_accepted_shape(b: Seq<u8>, w: int, hgt: int, out: Seq<u8>)
    requires
        mp(b) == Ok((w, hgt, out)),
    ensures
        mp_ok_shape(b, w, hgt, out),
{
    reveal(mp);
    reveal(mp_ok_shape);
    reveal(mp_after);
    reveal(mp_id_end);
    reveal(mp_map_end);
    reveal(mp_assemble);
    reveal(mp_pixels);
    match mh(b) {
        Ok(m) => {
            assert(w == m.width as int);
            assert(hgt == m.height as int);
            if m.image_type == 9u8 || m.image_type == 10u8 || m.image_type == 11u8 {
                reveal(mr);
                match mr(
                    b,
                    pkt_of_mh(m),
                    b.subrange(mp_id_end(m), mp_id_end(m) + m.map_bytes),
                    mp_map_end(m),
                    0,
                ) {
                    Ok(rr) => {
                        assert(rr.1 == m.n) by {
                            lemma_mr_at_ok_prod(
                                b,
                                pkt_of_mh(m),
                                b.subrange(mp_id_end(m), mp_id_end(m) + m.map_bytes),
                                mp_map_end(m),
                                0,
                                Seq::<Seq<u8>>::empty(),
                            );
                        }
                        lemma_mr_at_ok(
                            b,
                            pkt_of_mh(m),
                            b.subrange(mp_id_end(m), mp_id_end(m) + m.map_bytes),
                            mp_map_end(m),
                            rr.0,
                            rr.1,
                            rr.2,
                        );
                        assert(mp_pixels(b, m, mp_id_end(m)) == mr_pix(
                            b,
                            pkt_of_mh(m),
                            b.subrange(mp_id_end(m), mp_id_end(m) + m.map_bytes),
                            mp_map_end(m),
                            0,
                        ));
                    },
                    Err(_) => {},
                }
            } else {
                assert(mp_pixels(b, m, mp_id_end(m)) == acc_conv(
                    b,
                    pkt_of_mh(m),
                    b.subrange(mp_id_end(m), mp_id_end(m) + m.map_bytes),
                    mp_map_end(m),
                    m.n,
                ));
                lemma_acc_conv_len(
                    b,
                    pkt_of_mh(m),
                    b.subrange(mp_id_end(m), mp_id_end(m) + m.map_bytes),
                    mp_map_end(m),
                    m.n,
                );
            }
            assert(mp_pixels(b, m, mp_id_end(m)).len() == m.n);
            assert(out == orient_bytes(
                mp_pixels(b, m, mp_id_end(m)),
                w,
                hgt,
                m.descriptor & 0x10u8 != 0u8,
                m.descriptor & 0x20u8 != 0u8,
                m.n,
            )) by {
                assert(out == mp_assemble(b, m, mp_id_end(m)));
            }
            assert(mh(b) == Ok(m));
        },
        Err(_) => {},
    }
}

/// T8's per-cell statement, discharged for every accepted `MP(b) = Ok(w,h,out)`
/// and every cell `q < N`: `t8_cell` holds with the record `m` `mp_ok_shape`
/// supplies. This is the bridge Finding 3 asks for — the `MP(b) = Ok(...)`
/// hypothesis is real, not the existential `mp_ok_shape` taken as a premise.
pub proof fn lemma_mp_t8_cell(b: Seq<u8>, w: int, hgt: int, out: Seq<u8>, q: int)
    requires
        mp(b) == Ok((w, hgt, out)),
        1 <= w <= 8192,
        1 <= hgt <= 8192,
        0 <= q < w * hgt,
    ensures
        t8_cell(b, w, hgt, out, q),
{
    lemma_mp_accepted_shape(b, w, hgt, out);
    reveal(mp_ok_shape);
    reveal(t8_cell);
    let m = choose|m: MHModel| mh(b) == Ok(m) && m.width as int == w && m.height as int == hgt
        && mp_id_end(m) <= b.len() && mp_map_end(m) <= b.len()
        && mp_pixels(b, m, mp_id_end(m)).len() == m.n
        && out == orient_bytes(
        mp_pixels(b, m, mp_id_end(m)),
        w,
        hgt,
        m.descriptor & 0x10u8 != 0u8,
        m.descriptor & 0x20u8 != 0u8,
        m.n,
    );
    lemma_mh_selfcons(b, m);
    reveal(mh_selfcons);
    let px = mp_pixels(b, m, mp_id_end(m));
    let fx = m.descriptor & 0x10u8 != 0u8;
    let fy = m.descriptor & 0x20u8 != 0u8;
    assert(m.n == w * hgt) by (nonlinear_arith)
        requires
            m.n == m.width as int * (m.height as int),
            m.width as int == w,
            m.height as int == hgt,
    ;
    assert(px.len() == w * hgt) by (nonlinear_arith)
        requires
            px.len() == m.n,
            m.n == w * hgt,
    ;
    let k = gcoord(w, hgt, fx, fy, q);
    lemma_fcoord_inverse(w, hgt, fx, fy, q);
    assert(0 <= k < m.n) by (nonlinear_arith)
        requires
            0 <= k < w * hgt,
            m.n == w * hgt,
    ;
    assert(fcoord(w, hgt, fx, fy, k) == q);
    assert(out == orient_bytes(px, w, hgt, fx, fy, w * hgt));
    // The four bytes, index by index, through the orientation index lemma.
    lemma_orient_bytes_index(px, w, hgt, fx, fy, w * hgt, 4 * q);
    lemma_orient_bytes_index(px, w, hgt, fx, fy, w * hgt, 4 * q + 1);
    lemma_orient_bytes_index(px, w, hgt, fx, fy, w * hgt, 4 * q + 2);
    lemma_orient_bytes_index(px, w, hgt, fx, fy, w * hgt, 4 * q + 3);
    assert(4 * q / 4 == q) by (nonlinear_arith);
    assert((4 * q + 1) / 4 == q) by (nonlinear_arith);
    assert((4 * q + 2) / 4 == q) by (nonlinear_arith);
    assert((4 * q + 3) / 4 == q) by (nonlinear_arith);
    assert(4 * q % 4 == 0) by (nonlinear_arith);
    assert((4 * q + 1) % 4 == 1) by (nonlinear_arith);
    assert((4 * q + 2) % 4 == 2) by (nonlinear_arith);
    assert((4 * q + 3) % 4 == 3) by (nonlinear_arith);
    reveal(px_byte);
    assert(out[4 * q] == px[k][0]);
    assert(out[4 * q + 1] == px[k][1]);
    assert(out[4 * q + 2] == px[k][2]);
    assert(out[4 * q + 3] == px[k][3]);
    assert(t8_cell(b, w, hgt, out, q)) by {
        assert(exists|mm: MHModel| mh(b) == Ok(mm) && mm.width as int == w
            && mm.height as int == hgt && mm.n == w * hgt && ({
            let ffx = mm.descriptor & 0x10u8 != 0u8;
            let ffy = mm.descriptor & 0x20u8 != 0u8;
            let kk = gcoord(w, hgt, ffx, ffy, q);
            let pxx = mp_pixels(b, mm, mp_id_end(mm));
            &&& 0 <= kk < mm.n
            &&& fcoord(w, hgt, ffx, ffy, kk) == q
            &&& out[4 * q] == pxx[kk][0]
            &&& out[4 * q + 1] == pxx[kk][1]
            &&& out[4 * q + 2] == pxx[kk][2]
            &&& out[4 * q + 3] == pxx[kk][3]
        }));
    }
}

/// `mr_pix` at the packet-stream start, expressed with an explicit prefix: the
/// outer loop's split is stable across one packet.
pub proof fn lemma_mr_pix_assoc(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    c0: int,
    a: Seq<Seq<u8>>,
    x: Seq<Seq<u8>>,
    y: Seq<Seq<u8>>,
)
    requires
        mr_pix(b, p, map, c0, 0) == a + (x + y),
    ensures
        mr_pix(b, p, map, c0, 0) == (a + x) + y,
{
    vstd::seq_lib::lemma_concat_associative(a, x, y);
}

/// `MR`'s failure branches, at the model's own check points./// `MR`'s failure branches, at the model's own check points.

/// Control byte absent: `RleTruncated`, nothing produced.
pub proof fn lemma_mr_cur_ctrl_absent(s: Seq<u8>, h: PktH, map: Seq<u8>, cur: int, prod: int)
    requires
        0 <= cur <= s.len(),
        0 <= prod < h.n,
        cur >= s.len(),
    ensures
        mr_cur(s, h, map, cur, prod) == Err(MrErr::Api(Error::RleTruncated)),
        mr_pix(s, h, map, cur, prod) == Seq::empty(),
{
    reveal(mr_cur);
    reveal(mr_pix);
}

/// `MR`'s exact error tuple at each failure branch, at the model's own check
/// points (the whole-run `mr`, not just the cursor projection).
pub proof fn lemma_mr_err_ctrl_absent(s: Seq<u8>, h: PktH, map: Seq<u8>, cur: int, prod: int)
    requires
        0 <= cur <= s.len(),
        0 <= prod < h.n,
        cur >= s.len(),
    ensures
        mr(s, h, map, cur, prod) == Err((MrErr::Api(Error::RleTruncated), cur, prod, Seq::<Seq<u8>>::empty())),
{
    reveal(mr);
    reveal(mr_at);
}

pub proof fn lemma_mr_err_overrun(s: Seq<u8>, h: PktH, map: Seq<u8>, cur: int, prod: int)
    requires
        0 <= cur < s.len(),
        0 <= prod < h.n,
        rle_count(s[cur] as int) > h.n - prod,
    ensures
        mr(s, h, map, cur, prod) == Err((
            MrErr::Api(Error::RleOverrun),
            cur + 1,
            prod,
            Seq::<Seq<u8>>::empty(),
        )),
{
    reveal(mr);
    reveal(mr_at);
}

pub proof fn lemma_mr_err_rep_missing(s: Seq<u8>, h: PktH, map: Seq<u8>, cur: int, prod: int)
    requires
        0 <= cur < s.len(),
        0 <= prod < h.n,
        rle_count(s[cur] as int) <= h.n - prod,
        rle_repeat(s[cur] as int),
        s.len() < cur + 1 + h.sample_bytes,
    ensures
        mr(s, h, map, cur, prod) == Err((MrErr::Api(Error::RleTruncated), cur + 1, prod, Seq::<Seq<u8>>::empty())),
{
    reveal(mr);
    reveal(mr_at);
}

pub proof fn lemma_mr_err_rep_index(s: Seq<u8>, h: PktH, map: Seq<u8>, cur: int, prod: int)
    requires
        0 <= cur < s.len(),
        0 <= prod < h.n,
        rle_count(s[cur] as int) <= h.n - prod,
        rle_repeat(s[cur] as int),
        s.len() >= cur + 1 + h.sample_bytes,
        h.kind == 0,
        !in_range(h, sindex(s.subrange(cur + 1, cur + 1 + h.sample_bytes), h.sample_bytes)),
    ensures
        mr(s, h, map, cur, prod) == Err((
            MrErr::Api(Error::ColorIndexOutOfRange),
            cur + 1 + h.sample_bytes,
            prod,
            Seq::<Seq<u8>>::empty(),
        )),
{
    reveal(mr);
    reveal(mr_at);
}

pub proof fn lemma_mr_err_lit_missing(s: Seq<u8>, h: PktH, map: Seq<u8>, cur: int, prod: int)
    requires
        0 <= cur < s.len(),
        0 <= prod < h.n,
        rle_count(s[cur] as int) <= h.n - prod,
        !rle_repeat(s[cur] as int),
        s.len() < cur + 1 + rle_count(s[cur] as int) * h.sample_bytes,
    ensures
        mr(s, h, map, cur, prod) == Err((MrErr::Api(Error::RleTruncated), cur + 1, prod, Seq::<Seq<u8>>::empty())),
{
    reveal(mr);
    reveal(mr_at);
}

pub proof fn lemma_mr_err_lit_index(s: Seq<u8>, h: PktH, map: Seq<u8>, cur: int, prod: int)
    requires
        0 <= cur < s.len(),
        0 <= prod < h.n,
        rle_count(s[cur] as int) <= h.n - prod,
        !rle_repeat(s[cur] as int),
        s.len() >= cur + 1 + rle_count(s[cur] as int) * h.sample_bytes,
        !lit_ok(s, h, cur + 1, rle_count(s[cur] as int)),
    ensures
        mr(s, h, map, cur, prod) == Err((
            MrErr::Api(Error::ColorIndexOutOfRange),
            cur + 1 + rle_count(s[cur] as int) * h.sample_bytes,
            prod + lit_good(s, h, cur + 1, rle_count(s[cur] as int)),
            lit_out(s, h, map, cur + 1, lit_good(s, h, cur + 1, rle_count(s[cur] as int))),
        )),
{
    reveal(mr);
    reveal(mr_at);
}

/// The ghost error state of each of `MR`'s failure branches, at the model's own
/// check points: the first error, the cursor and produced count held at that
/// check, and the pixels produced before it. These are the manifest's proof
/// observations that T6's failure case must relate to the concrete helper.
pub proof fn lemma_mr_ghost_ctrl_absent(s: Seq<u8>, h: PktH, map: Seq<u8>, cur: int, prod: int)
    requires
        0 <= cur <= s.len(),
        0 <= prod < h.n,
        cur >= s.len(),
    ensures
        mr_ghost(s, h, map, cur, prod) == (MrErr::Api(Error::RleTruncated), cur, prod, Seq::<Seq<u8>>::empty()),
{
    reveal(mr_ghost);
    reveal(mr);
    reveal(mr_at);
}

pub proof fn lemma_mr_ghost_overrun(s: Seq<u8>, h: PktH, map: Seq<u8>, cur: int, prod: int)
    requires
        0 <= cur < s.len(),
        0 <= prod < h.n,
        rle_count(s[cur] as int) > h.n - prod,
    ensures
        mr_ghost(s, h, map, cur, prod) == (MrErr::Api(Error::RleOverrun), cur + 1, prod, Seq::<Seq<u8>>::empty()),
{
    reveal(mr_ghost);
    reveal(mr);
    reveal(mr_at);
}

pub proof fn lemma_mr_ghost_rep_missing(s: Seq<u8>, h: PktH, map: Seq<u8>, cur: int, prod: int)
    requires
        0 <= cur < s.len(),
        0 <= prod < h.n,
        rle_count(s[cur] as int) <= h.n - prod,
        rle_repeat(s[cur] as int),
        s.len() < cur + 1 + h.sample_bytes,
    ensures
        mr_ghost(s, h, map, cur, prod) == (MrErr::Api(Error::RleTruncated), cur + 1, prod, Seq::<Seq<u8>>::empty()),
{
    reveal(mr_ghost);
    reveal(mr);
    reveal(mr_at);
}

pub proof fn lemma_mr_ghost_rep_index(s: Seq<u8>, h: PktH, map: Seq<u8>, cur: int, prod: int)
    requires
        0 <= cur < s.len(),
        0 <= prod < h.n,
        rle_count(s[cur] as int) <= h.n - prod,
        rle_repeat(s[cur] as int),
        s.len() >= cur + 1 + h.sample_bytes,
        h.kind == 0,
        !in_range(h, sindex(s.subrange(cur + 1, cur + 1 + h.sample_bytes), h.sample_bytes)),
    ensures
        mr_ghost(s, h, map, cur, prod) == (
            MrErr::Api(Error::ColorIndexOutOfRange),
            cur + 1 + h.sample_bytes,
            prod,
            Seq::<Seq<u8>>::empty(),
        ),
{
    reveal(mr_ghost);
    reveal(mr);
    reveal(mr_at);
}

pub proof fn lemma_mr_ghost_lit_missing(s: Seq<u8>, h: PktH, map: Seq<u8>, cur: int, prod: int)
    requires
        0 <= cur < s.len(),
        0 <= prod < h.n,
        rle_count(s[cur] as int) <= h.n - prod,
        !rle_repeat(s[cur] as int),
        s.len() < cur + 1 + rle_count(s[cur] as int) * h.sample_bytes,
    ensures
        mr_ghost(s, h, map, cur, prod) == (MrErr::Api(Error::RleTruncated), cur + 1, prod, Seq::<Seq<u8>>::empty()),
{
    reveal(mr_ghost);
    reveal(mr);
    reveal(mr_at);
}

pub proof fn lemma_mr_ghost_lit_index(s: Seq<u8>, h: PktH, map: Seq<u8>, cur: int, prod: int)
    requires
        0 <= cur < s.len(),
        0 <= prod < h.n,
        rle_count(s[cur] as int) <= h.n - prod,
        !rle_repeat(s[cur] as int),
        s.len() >= cur + 1 + rle_count(s[cur] as int) * h.sample_bytes,
        !lit_ok(s, h, cur + 1, rle_count(s[cur] as int)),
    ensures
        mr_ghost(s, h, map, cur, prod) == (
            MrErr::Api(Error::ColorIndexOutOfRange),
            cur + 1 + rle_count(s[cur] as int) * h.sample_bytes,
            prod + lit_good(s, h, cur + 1, rle_count(s[cur] as int)),
            lit_out(s, h, map, cur + 1, lit_good(s, h, cur + 1, rle_count(s[cur] as int))),
        ),
{
    reveal(mr_ghost);
    reveal(mr);
    reveal(mr_at);
}

/// Run count exceeds the pixels remaining: `RleOverrun`, checked before any
/// packet byte is read.
pub proof fn lemma_mr_cur_overrun(s: Seq<u8>, h: PktH, map: Seq<u8>, cur: int, prod: int)
    requires
        0 <= cur < s.len(),
        0 <= prod < h.n,
        rle_count(s[cur] as int) > h.n - prod,
    ensures
        mr_cur(s, h, map, cur, prod) == Err(MrErr::Api(Error::RleOverrun)),
        mr_pix(s, h, map, cur, prod) == Seq::empty(),
{
    reveal(mr_cur);
    reveal(mr_pix);
}

/// A repeat packet's single sample is missing: `RleTruncated`, nothing produced.
pub proof fn lemma_mr_cur_rep_missing(s: Seq<u8>, h: PktH, map: Seq<u8>, cur: int, prod: int)
    requires
        0 <= cur < s.len(),
        0 <= prod < h.n,
        rle_count(s[cur] as int) <= h.n - prod,
        rle_repeat(s[cur] as int),
        s.len() < cur + 1 + h.sample_bytes,
    ensures
        mr_cur(s, h, map, cur, prod) == Err(MrErr::Api(Error::RleTruncated)),
        mr_pix(s, h, map, cur, prod) == Seq::empty(),
{
    reveal(mr_cur);
    reveal(mr_pix);
}

/// A repeat packet's single sample has an out-of-range index.
pub proof fn lemma_mr_cur_rep_index(s: Seq<u8>, h: PktH, map: Seq<u8>, cur: int, prod: int)
    requires
        0 <= cur < s.len(),
        0 <= prod < h.n,
        rle_count(s[cur] as int) <= h.n - prod,
        rle_repeat(s[cur] as int),
        s.len() >= cur + 1 + h.sample_bytes,
        h.kind == 0,
        !in_range(
            h,
            sindex(s.subrange(cur + 1, cur + 1 + h.sample_bytes), h.sample_bytes),
        ),
    ensures
        mr_cur(s, h, map, cur, prod) == Err(MrErr::Api(Error::ColorIndexOutOfRange)),
        mr_pix(s, h, map, cur, prod) == Seq::empty(),
{
    reveal(mr_cur);
    reveal(mr_pix);
}

/// A literal packet's bytes are missing: `RleTruncated`, nothing produced.
pub proof fn lemma_mr_cur_lit_missing(s: Seq<u8>, h: PktH, map: Seq<u8>, cur: int, prod: int)
    requires
        0 <= cur < s.len(),
        0 <= prod < h.n,
        rle_count(s[cur] as int) <= h.n - prod,
        !rle_repeat(s[cur] as int),
        s.len() < cur + 1 + rle_count(s[cur] as int) * h.sample_bytes,
    ensures
        mr_cur(s, h, map, cur, prod) == Err(MrErr::Api(Error::RleTruncated)),
        mr_pix(s, h, map, cur, prod) == Seq::empty(),
{
    reveal(mr_cur);
    reveal(mr_pix);
}

/// A literal packet whose first out-of-range sample is at offset `g`: the `g`
/// pixels before it are produced, then `ColorIndexOutOfRange`.
pub proof fn lemma_mr_cur_lit_index(s: Seq<u8>, h: PktH, map: Seq<u8>, cur: int, prod: int)
    requires
        0 <= cur < s.len(),
        0 <= prod < h.n,
        rle_count(s[cur] as int) <= h.n - prod,
        !rle_repeat(s[cur] as int),
        s.len() >= cur + 1 + rle_count(s[cur] as int) * h.sample_bytes,
        !lit_ok(s, h, cur + 1, rle_count(s[cur] as int)),
    ensures
        mr_cur(s, h, map, cur, prod) == Err(MrErr::Api(Error::ColorIndexOutOfRange)),
        mr_pix(s, h, map, cur, prod) == lit_out(
            s,
            h,
            map,
            cur + 1,
            lit_good(s, h, cur + 1, rle_count(s[cur] as int)),
        ),
{
    reveal(mr_cur);
    reveal(mr_pix);
}



/// `mr_pix` at a repeat packet: the packet's `count` copies of its converted
/// sample, then the rest.
pub proof fn lemma_mr_pix_repeat(s: Seq<u8>, h: PktH, map: Seq<u8>, cur: int, prod: int)
    requires
        0 <= cur < s.len(),
        0 <= prod < h.n,
        0 <= h.sample_bytes,
        rle_count(s[cur] as int) <= h.n - prod,
        rle_repeat(s[cur] as int),
        s.len() >= cur + 1 + h.sample_bytes,
        h.kind == 0 ==> in_range(
            h,
            sindex(s.subrange(cur + 1, cur + 1 + h.sample_bytes), h.sample_bytes),
        ),
    ensures
        mr_pix(s, h, map, cur, prod) == pix_pow(
            conv2(h, map, s.subrange(cur + 1, cur + 1 + h.sample_bytes)),
            rle_count(s[cur] as int),
        ) + mr_pix(s, h, map, cur + 1 + h.sample_bytes, prod + rle_count(s[cur] as int)),
        mr_cur(s, h, map, cur, prod) == mr_cur(
            s,
            h,
            map,
            cur + 1 + h.sample_bytes,
            prod + rle_count(s[cur] as int),
        ),
{
    reveal(mr_pix);
    reveal(mr_cur);
}

/// `mr_pix` at a successful literal packet: the packet's `count` conversions,
/// then the rest.
pub proof fn lemma_mr_pix_literal(s: Seq<u8>, h: PktH, map: Seq<u8>, cur: int, prod: int)
    requires
        0 <= cur < s.len(),
        0 <= prod < h.n,
        0 <= h.sample_bytes,
        rle_count(s[cur] as int) <= h.n - prod,
        !rle_repeat(s[cur] as int),
        s.len() >= cur + 1 + rle_count(s[cur] as int) * h.sample_bytes,
        lit_ok(s, h, cur + 1, rle_count(s[cur] as int)),
    ensures
        mr_pix(s, h, map, cur, prod) == lit_out(
            s,
            h,
            map,
            cur + 1,
            rle_count(s[cur] as int),
        ) + mr_pix(s, h, map, cur + 1 + rle_count(s[cur] as int) * h.sample_bytes, prod
            + rle_count(s[cur] as int)),
        mr_cur(s, h, map, cur, prod) == mr_cur(
            s,
            h,
            map,
            cur + 1 + rle_count(s[cur] as int) * h.sample_bytes,
            prod + rle_count(s[cur] as int),
        ),
{
    reveal(mr_pix);
    reveal(mr_cur);
}

/// The encoded length `|P|` of the control/sample packet of the repetition
/// lemma: one control byte plus `sample_bytes` sample bytes.
pub open spec fn pkt_len(sample_bytes: int) -> int {
    1 + sample_bytes
}

/// One control/sample packet `P` (length `1+sample_bytes`, control `0xff`, a
/// fixed sample of `sample_bytes` bytes whose complete conversion is `c`) read
/// at absolute cursor `cur`: the model steps from `(cur,prod,out)` to
/// `(cur+|P|, prod+128, out + c^128)`.
pub open spec fn packet_step(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    cur: int,
    prod: int,
    sample_bytes: int,
    c: Seq<u8>,
) -> bool {
    &&& cur >= 0
    &&& cur + 1 + sample_bytes <= s.len()
    &&& s[cur] as int == 255
    &&& rle_count(255) == 128
    &&& sample_bytes == h.sample_bytes
    &&& 1 <= h.sample_bytes
    &&& conv2(h, map, s.subrange(cur + 1, cur + 1 + sample_bytes)) == c
    &&& (h.kind == 0 ==> in_range(
        h,
        sindex(s.subrange(cur + 1, cur + 1 + sample_bytes), h.sample_bytes),
    ))
    &&& prod + 128 <= h.n
}

/// One packet step, as the model actually executes it.
pub proof fn lemma_mr_packet_step(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    cur: int,
    prod: int,
    c: Seq<u8>,
    out: Seq<Seq<u8>>,
)
    requires
        packet_step(s, h, map, cur, prod, h.sample_bytes, c),
        0 <= prod,
        prod < h.n,
    ensures
        mr_at(s, h, map, cur, prod, out) == mr_at(
            s,
            h,
            map,
            cur + pkt_len(h.sample_bytes),
            prod + 128,
            out + pix_pow(c, 128),
        ),
{
    reveal(packet_step);
    reveal(mr_at);
    reveal(rle_count);
    reveal(rle_repeat);
    reveal(pkt_len);
    let control: int = s[cur] as int;
    assert(control == 255);
    lemma_rle_count_bit(255u8);
    assert(s[cur] as int == 255);
    assert(rle_count(255) == 128);
    assert(rle_count(control) == 128);
    assert(0 <= h.sample_bytes);
    assert(s.len() >= cur + 1 + h.sample_bytes);
    assert(cur < s.len()) by (nonlinear_arith)
        requires
            s.len() >= cur + 1 + h.sample_bytes,
            0 <= h.sample_bytes,
    ;
    assert(cur <= s.len());
    assert(!(prod > h.n)) by (nonlinear_arith)
        requires
            prod < h.n,
    ;
    assert(!(128 > h.n - prod)) by (nonlinear_arith)
        requires
            prod + 128 <= h.n,
    ;
    assert(rle_repeat(control));
    assert(s.len() >= cur + 1);
    if h.kind == 0 {
        reveal(packet_step);
        assert(in_range(h, sindex(s.subrange(cur + 1, cur + 1 + h.sample_bytes), h.sample_bytes)));
    }
    assert(s.subrange(cur + 1, cur + 1 + h.sample_bytes) == s.subrange(
        cur + 1,
        cur + 1 + h.sample_bytes,
    ));
    assert(mr_at(s, h, map, cur, prod, out) == mr_at(
        s,
        h,
        map,
        cur + pkt_len(h.sample_bytes),
        prod + 128,
        out + pix_pow(c, 128),
    )) by {
        reveal(mr_at);
        reveal(rle_count);
        reveal(rle_repeat);
        reveal(pkt_len);
    }
}

/// The packet-repetition lemma (`|P|` in the manifest is `pkt_len`), by
/// induction on `k>=0`: for a reachable header
/// and map, `N-produced0 = 128*k`, a valid starting cursor, and a control/sample
/// packet `P` of length `1+sample_bytes` whose control is `0xff` and whose
/// complete sample converts to colour `c`, reading `P` `k` times in a row takes
/// the model from `(cursor0,produced0)` to `(cursor0 + k*|P|, N)` and appends
/// `c` exactly `128*k` times. The base case `k=0` has `produced0=N` (the `P^0`
/// empty word) and reads no packet; the step uses `count=128` and the remaining
/// pixels `>= 128`.
/// The packet-repetition lemma (`|P|` in the manifest is `pkt_len`), by
/// induction on `k>=0`: for a reachable header
/// and map, `N-produced0 = 128*k`, a valid starting cursor, and a control/sample
/// packet `P` of length `1+sample_bytes` whose control is `0xff` and whose
/// complete sample converts to colour `c`, reading `P` `k` times in a row takes
/// the model from `(cursor0,produced0)` to `(cursor0 + k*|P|, N)` and appends
/// `c` exactly `128*k` times. The base case `k=0` has `produced0=N` (the `P^0`
/// empty word) and reads no packet; the step uses `count=128` and the remaining
/// pixels `>= 128`. Stated with an initial pixel prefix `out0`, so it composes;
/// `lemma_packet_repetition_mr` is the `MR` form of the manifest.
pub proof fn lemma_packet_repetition(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    cursor0: int,
    produced0: int,
    k: int,
    c: Seq<u8>,
    out0: Seq<Seq<u8>>,
)
    requires
        k >= 0,
        h.n - produced0 == 128 * k,
        0 <= produced0 <= h.n,
        0 <= cursor0,
        0 <= h.sample_bytes,
        0 <= h.n,
        cursor0 <= s.len(),
        forall|j: int| 0 <= j < k ==> #[trigger] packet_step(
            s,
            h,
            map,
            cursor0 + j * pkt_len(h.sample_bytes),
            produced0 + j * 128,
            h.sample_bytes,
            c,
        ),
    ensures
        mr_at(s, h, map, cursor0, produced0, out0) == Ok(
            (cursor0 + k * pkt_len(h.sample_bytes), h.n, out0 + pix_pow(c, 128 * k)),
        ),
    decreases k,
{
    reveal(mr);
    assert(pkt_len(h.sample_bytes) >= 1) by {
        reveal(pkt_len);
    }
    if k == 0 {
        assert(produced0 == h.n) by (nonlinear_arith)
            requires
                h.n - produced0 == 0,
                produced0 <= h.n,
        ;
        assert(k * pkt_len(h.sample_bytes) == 0) by (nonlinear_arith)
            requires
                k == 0,
        ;
        assert(128 * k == 0) by (nonlinear_arith)
            requires
                k == 0,
        ;
        lemma_pix_pow_zero(c);
        assert(mr_at(s, h, map, cursor0, produced0, out0) == Ok((cursor0, produced0, out0))) by {
            reveal(mr_at);
        }
        broadcast use vstd::seq_lib::group_seq_lib_default;
        assert(out0 + pix_pow(c, 0) == out0);
    } else {
        assert(produced0 < h.n) by (nonlinear_arith)
            requires
                h.n - produced0 == 128 * k,
                k >= 1,
        ;
        assert(produced0 + 128 <= h.n) by (nonlinear_arith)
            requires
                h.n - produced0 == 128 * k,
                k >= 1,
        ;
        // the first packet is valid at (cursor0, produced0)
        assert(cursor0 + 0 * pkt_len(h.sample_bytes) == cursor0) by (nonlinear_arith);
        assert(produced0 + 0 * 128 == produced0) by (nonlinear_arith);
        assert(packet_step(s, h, map, cursor0, produced0, h.sample_bytes, c));
        lemma_mr_packet_step(s, h, map, cursor0, produced0, c, out0);
        // the remaining `k-1` packets are valid from the new cursor/produced
        assert forall|j: int| 0 <= j < k - 1 implies #[trigger] packet_step(
            s,
            h,
            map,
            cursor0 + pkt_len(h.sample_bytes) + j * pkt_len(h.sample_bytes),
            produced0 + 128 + j * 128,
            h.sample_bytes,
            c,
        ) by {
            assert(packet_step(
                s,
                h,
                map,
                cursor0 + (j + 1) * pkt_len(h.sample_bytes),
                produced0 + (j + 1) * 128,
                h.sample_bytes,
                c,
            ));
            assert(cursor0 + pkt_len(h.sample_bytes) + j * pkt_len(h.sample_bytes) == cursor0
                + (j + 1) * pkt_len(h.sample_bytes)) by (nonlinear_arith);
            assert(produced0 + 128 + j * 128 == produced0 + (j + 1) * 128) by (nonlinear_arith);
        }
        assert(0 <= cursor0 + pkt_len(h.sample_bytes)) by (nonlinear_arith)
            requires
                0 <= cursor0,
                pkt_len(h.sample_bytes) >= 1,
        ;
        assert(cursor0 + pkt_len(h.sample_bytes) <= s.len()) by {
            reveal(packet_step);
            reveal(pkt_len);
        }
        assert(h.n - (produced0 + 128) == 128 * (k - 1)) by (nonlinear_arith)
            requires
                h.n - produced0 == 128 * k,
                k >= 1,
        ;
        lemma_packet_repetition(
            s,
            h,
            map,
            cursor0 + pkt_len(h.sample_bytes),
            produced0 + 128,
            k - 1,
            c,
            out0 + pix_pow(c, 128),
        );
        // arithmetic: final cursor and produced count
        assert(cursor0 + pkt_len(h.sample_bytes) + (k - 1) * pkt_len(h.sample_bytes) == cursor0
            + k * pkt_len(h.sample_bytes)) by (nonlinear_arith);
        assert(produced0 + 128 + (k - 1) * 128 == h.n) by (nonlinear_arith)
            requires
                h.n - produced0 == 128 * k,
                k >= 1,
        ;
        // pixel accumulation: (out0 + c^128) + c^(128*(k-1)) == out0 + c^(128*k)
        lemma_pix_pow_add(c, 128, 128 * (k - 1));
        vstd::seq_lib::lemma_concat_associative(out0, pix_pow(c, 128), pix_pow(c, 128 * (k - 1)));
        assert(out0 + pix_pow(c, 128) + pix_pow(c, 128 * (k - 1)) == out0 + (pix_pow(c, 128)
            + pix_pow(c, 128 * (k - 1))));
        assert(pix_pow(c, 128) + pix_pow(c, 128 * (k - 1)) == pix_pow(c, 128 + 128 * (k - 1)));
        assert(128 + 128 * (k - 1) == 128 * k) by (nonlinear_arith)
            requires
                k >= 1,
        ;
        assert(pix_pow(c, 128 + 128 * (k - 1)) == pix_pow(c, 128 * k));
        assert(out0 + pix_pow(c, 128) + pix_pow(c, 128 * (k - 1)) == out0 + pix_pow(c, 128 * k));
    }
}

/// `MR(prefix+P^k,H,map,cursor0,produced0)` in the manifest's form: the packet
/// model run from the stream start of packet data, with no pixel prefix.
pub proof fn lemma_packet_repetition_mr(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    cursor0: int,
    produced0: int,
    k: int,
    c: Seq<u8>,
)
    requires
        k >= 0,
        h.n - produced0 == 128 * k,
        0 <= produced0 <= h.n,
        0 <= cursor0,
        0 <= h.sample_bytes,
        0 <= h.n,
        cursor0 <= s.len(),
        forall|j: int| 0 <= j < k ==> #[trigger] packet_step(
            s,
            h,
            map,
            cursor0 + j * pkt_len(h.sample_bytes),
            produced0 + j * 128,
            h.sample_bytes,
            c,
        ),
    ensures
        mr(s, h, map, cursor0, produced0) == Ok(
            (cursor0 + k * pkt_len(h.sample_bytes), h.n, pix_pow(c, 128 * k)),
        ),
{
    reveal(mr);
    lemma_packet_repetition(s, h, map, cursor0, produced0, k, c, Seq::<Seq<u8>>::empty());
    broadcast use vstd::seq_lib::group_seq_lib_default;
    assert(Seq::<Seq<u8>>::empty() + pix_pow(c, 128 * k) == pix_pow(c, 128 * k));
}

/// The conversion-domain facts about `h`, as a predicate the executable can
/// carry: the image kind is one of the three, the depth and (for indexed
/// images) the map entry depth are supported, and the sample width is the
/// depth's byte count.
pub open spec fn pkt_of_domain(h: PktH) -> bool {
    &&& (h.kind == 0 || h.kind == 1 || h.kind == 2)
    &&& h.sample_bytes == stride_of(h.depth)
    &&& (h.kind == 0 ==> (h.depth == 8 || h.depth == 16) && (h.map_depth == 15 || h.map_depth == 16
        || h.map_depth == 24 || h.map_depth == 32))
    &&& (h.kind == 1 ==> (h.depth == 15 || h.depth == 16 || h.depth == 24 || h.depth == 32))
    &&& (h.kind == 2 ==> (h.depth == 8 || h.depth == 16))
    &&& 1 <= h.sample_bytes <= 4
}

/// `sample_bytes_of` and `stride_of` are the same `ceil(d/8)`.
pub proof fn lemma_stride_eq(d: u8)
    ensures
        sample_bytes_of(d) == stride_of(d as int),
{
    reveal(sample_bytes_of);
    reveal(stride_of);
}

// ---------------------------------------------------------------------------
// M3: T4's "distinct k write distinct output positions", as a lemma.
// ---------------------------------------------------------------------------

/// R9's byte offset `4*f(k)` is injective on `[0,N)`: distinct input samples
/// write four distinct output bytes' starting positions.
pub proof fn lemma_offset_injective(w: int, h: int, fx: bool, fy: bool, j: int, k: int)
    requires
        1 <= w,
        1 <= h,
        0 <= j < w * h,
        0 <= k < w * h,
        j != k,
    ensures
        fcoord(w, h, fx, fy, j) != fcoord(w, h, fx, fy, k),
{
    if fcoord(w, h, fx, fy, j) == fcoord(w, h, fx, fy, k) {
        lemma_fcoord_left_inverse(w, h, fx, fy, j);
        lemma_fcoord_left_inverse(w, h, fx, fy, k);
        assert(j == k);
    }
}

/// `output_offset`'s formula is `fcoord`: the coordinate model and the
/// executable's oriented offset agree on every cell.
pub proof fn lemma_offset_is_fcoord(w: int, h: int, fx: bool, fy: bool, k: int)
    requires
        1 <= w,
        1 <= h,
        0 <= k < w * h,
    ensures
        4 * fcoord(w, h, fx, fy, k) == 4 * (spec_axis(h, !fy, k / w) * w + spec_axis(
            w,
            fx,
            k % w,
        )),
{
    reveal(fcoord);
    reveal(oyr);
    reveal(oxr);
    reveal(spec_axis);
}

/// The manifest's "checked `4*f(k)+4<=4*N` follows from `4*N<=MAX_OUTPUT`":
/// for every `k<N` and `4*N<=MAX_OUTPUT`, the four bytes starting at `4*f(k)` fit
/// inside the output and inside any target `usize`.
pub proof fn lemma_output_offset_bound(w: int, h: int, fx: bool, fy: bool, k: int)
    requires
        1 <= w,
        1 <= h,
        0 <= k < w * h,
        4 * (w * h) <= 67108864,
    ensures
        0 <= 4 * fcoord(w, h, fx, fy, k),
        4 * fcoord(w, h, fx, fy, k) + 4 <= 4 * (w * h),
        4 * fcoord(w, h, fx, fy, k) + 4 <= 67108864,
{
    lemma_fcoord_range(w, h, fx, fy, k);
    assert(4 * fcoord(w, h, fx, fy, k) + 4 <= 4 * (w * h)) by (nonlinear_arith)
        requires
            0 <= fcoord(w, h, fx, fy, k) < w * h,
    ;
    assert(4 * (w * h) <= 67108864);
}


// ---------------------------------------------------------------------------
// M3: T3's termination and resource facts, at the model level.
// ---------------------------------------------------------------------------

/// Every one of the 256 control bytes gives a repeat/literal count in 1..=128,
/// so each accepted packet makes progress and the loop variant decreases.
pub proof fn lemma_rle_count_range(control: u8)
    ensures
        1 <= rle_count(control as int) <= 128,
{
    reveal(rle_count);
    if control >= 128u8 {
        assert(rle_count(control as int) == control as int - 127);
    } else {
        assert(rle_count(control as int) == control as int + 1);
    }
}


/// `MR` on a reachable `(cursor0, produced0)` is total and makes progress: for
/// every finite stream and every control byte it either returns an error or
/// reaches `N`, and along the way `0 <= produced <= N` and the ghost pixel
/// sequence grows by exactly the pixels produced. Proved by induction on
/// `N - produced`, `mr_at`'s own decreases metric.
pub proof fn lemma_mr_at_progress(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    cur: int,
    prod: int,
    out0: Seq<Seq<u8>>,
)
    requires
        0 <= cur <= s.len(),
        0 <= prod <= h.n,
        0 <= h.n,
        0 <= h.sample_bytes,
        h.n <= 16777216,
    ensures
        match mr_at(s, h, map, cur, prod, out0) {
            Ok(r) => 0 <= r.0 <= s.len() && prod <= r.1 <= h.n && r.2.len() == out0.len() + (r.1
                - prod),
            Err(_) => true,
        },
    decreases (h.n - prod) as nat,
{
    reveal(mr_at);
    if prod == h.n {
        assert(mr_at(s, h, map, cur, prod, out0) == Ok((cur, prod, out0)));
    } else if cur >= s.len() {
        assert(mr_at(s, h, map, cur, prod, out0) == Err((MrErr::Api(Error::RleTruncated), cur, prod, out0)));
    } else {
        let control = s[cur] as int;
        let count = rle_count(control);
        lemma_rle_count_range(s[cur]);
        if count > h.n - prod {
            assert(mr_at(s, h, map, cur, prod, out0) == Err((
                MrErr::Api(Error::RleOverrun),
                cur + 1,
                prod,
                out0,
            )));
        } else {
            let start = cur + 1;
            if rle_repeat(control) {
                let pe = start + h.sample_bytes;
                if s.len() < pe {
                    assert(mr_at(s, h, map, cur, prod, out0) == Err((
                        MrErr::Api(Error::RleTruncated),
                        start,
                        prod,
                        out0,
                    )));
                } else {
                    let smp = s.subrange(start, pe);
                    if h.kind == 0 && !in_range(h, sindex(smp, h.sample_bytes)) {
                        assert(mr_at(s, h, map, cur, prod, out0) == Err((
                            MrErr::Api(Error::ColorIndexOutOfRange),
                            pe,
                            prod,
                            out0,
                        )));
                    } else {
                        lemma_pix_pow_len(conv2(h, map, smp), count);
                        assert(0 <= pe <= s.len());
                        assert(prod + count <= h.n) by (nonlinear_arith)
                            requires
                                !(count > h.n - prod),
                        ;
                        assert(1 <= count);
                        let out1 = out0 + pix_pow(conv2(h, map, smp), count);
                        assert(out1.len() == out0.len() + count) by {
                            broadcast use vstd::seq::group_seq_axioms;
                        }
                        assert(0 < h.n - (prod + count) || prod + count == h.n) by (nonlinear_arith)
                            requires
                                prod + count <= h.n,
                        ;
                        lemma_mr_at_progress(s, h, map, pe, prod + count, out1);
                    }
                }
            } else {
                let total = count * h.sample_bytes;
                let pe = start + total;
                if s.len() < pe {
                    assert(mr_at(s, h, map, cur, prod, out0) == Err((
                        MrErr::Api(Error::RleTruncated),
                        start,
                        prod,
                        out0,
                    )));
                } else if !lit_ok(s, h, start, count) {
                    assert(mr_at(s, h, map, cur, prod, out0) == Err((
                        MrErr::Api(Error::ColorIndexOutOfRange),
                        pe,
                        prod + lit_good(s, h, start, count),
                        out0 + lit_out(s, h, map, start, lit_good(s, h, start, count)),
                    )));
                } else {
                    lemma_lit_out_len(s, h, map, start, count);
                    assert(0 <= pe <= s.len());
                    assert(prod + count <= h.n) by (nonlinear_arith)
                        requires
                            !(count > h.n - prod),
                    ;
                    assert(1 <= count);
                    let out1 = out0 + lit_out(s, h, map, start, count);
                    assert(out1.len() == out0.len() + count) by {
                        broadcast use vstd::seq::group_seq_axioms;
                    }
                    assert(0 < h.n - (prod + count) || prod + count == h.n) by (nonlinear_arith)
                        requires
                            prod + count <= h.n,
                    ;
                    lemma_mr_at_progress(s, h, map, pe, prod + count, out1);
                }
            }
        }
    }
}

/// A literal packet's model output has exactly `count` pixels, each four bytes.
pub proof fn lemma_lit_out_len(s: Seq<u8>, h: PktH, map: Seq<u8>, start: int, count: int)
    requires
        0 <= count,
    ensures
        lit_out(s, h, map, start, count).len() == count,
    decreases count,
{
    reveal(lit_out);
    if count <= 0 {
    } else {
        lemma_lit_out_len(s, h, map, start + h.sample_bytes, count - 1);
        broadcast use vstd::seq::group_seq_lemmas;
    }
}

/// The four converted bytes `c` sit at byte offset `off` of `out`.
pub open spec fn quad_at(out: Seq<u8>, off: int, c: Seq<u8>) -> bool {    &&& out[off] == c[0]
    &&& out[off + 1] == c[1]
    &&& out[off + 2] == c[2]
    &&& out[off + 3] == c[3]
}

/// T4's per-pixel content invariant, stated over the input stream: after `k`
/// samples, each converted pixel `j<k` sits at R9's oriented byte offset.
pub open spec fn wrote_samples(
    out: Seq<u8>,
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    start: int,
    sb: int,
    k: int,
) -> bool {
    forall|j: int| 0 <= j < k ==> #[trigger] quad_at(
        out,
        4 * fcoord(w, hgt, fx, fy, j),
        conv2(h, map, s.subrange(start + j * sb, start + j * sb + sb)),
    )
}

/// Unpack `wrote_samples` at a single index.
pub proof fn lemma_wrote_samples_at(
    out: Seq<u8>,
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    start: int,
    sb: int,
    k: int,
    j: int,
)
    requires
        wrote_samples(out, s, h, map, w, hgt, fx, fy, start, sb, k),
        0 <= j < k,
    ensures
        quad_at(
            out,
            4 * fcoord(w, hgt, fx, fy, j),
            conv2(h, map, s.subrange(start + j * sb, start + j * sb + sb)),
        ),
{
    reveal(wrote_samples);
}

/// Preserving one write of a new pixel at index `k` extends the invariant.
pub proof fn lemma_wrote_samples_step(
    out1: Seq<u8>,
    out0: Seq<u8>,
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    start: int,
    sb: int,
    k: int,
)
    requires
        wrote_samples(out0, s, h, map, w, hgt, fx, fy, start, sb, k),
        quad_at(
            out1,
            4 * fcoord(w, hgt, fx, fy, k),
            conv2(h, map, s.subrange(start + k * sb, start + k * sb + sb)),
        ),
        forall|j: int| 0 <= j < k ==> #[trigger] quad_at(
            out1,
            4 * fcoord(w, hgt, fx, fy, j),
            conv2(h, map, s.subrange(start + j * sb, start + j * sb + sb)),
        ),
    ensures
        wrote_samples(out1, s, h, map, w, hgt, fx, fy, start, sb, k + 1),
{
    reveal(wrote_samples);
    assert forall|j: int| 0 <= j < k + 1 implies #[trigger] quad_at(
        out1,
        4 * fcoord(w, hgt, fx, fy, j),
        conv2(h, map, s.subrange(start + j * sb, start + j * sb + sb)),
    ) by {
        if j < k {
        }
    }
}

// ---------------------------------------------------------------------------
// M4a: MP, the complete total decoder model.
// ---------------------------------------------------------------------------

/// R12: the 26-byte footer structure — the suffix is at least 26 bytes and its
/// final 18 bytes are the exact `TRUEVISION-XFILE.\0` signature.
pub open spec fn footer(suffix: Seq<u8>) -> bool {
    26 <= suffix.len() && suffix.subrange(suffix.len() as int - 18, suffix.len() as int)
        == tga_signature()
}

/// The conversion-relevant projection of a successful `MH` record, built from
/// the model record rather than from the executable `Header`, so `MP` does not
/// depend on executable types. `mp_pkt_h` records the agreement with the record's
/// own fields; it is a proved triviality, not a premise.
pub open spec fn pkt_of_mh(m: MHModel) -> PktH {
    PktH {
        kind: if is_indexed(m.image_type) {
            0
        } else if m.image_type == 2u8 || m.image_type == 10u8 {
            1
        } else {
            2
        },
        depth: m.pixel_depth as int,
        attr: (m.descriptor & 0x0fu8) as int,
        map_first: m.map_first as int,
        map_len: m.map_length as int,
        map_depth: m.map_entry_depth as int,
        n: m.n,
        sample_bytes: m.sample_bytes,
    }
}

/// `pkt_of_mh` reads the record's own fields.
pub open spec fn mp_pkt_h(m: MHModel) -> bool {
    &&& pkt_of_mh(m).depth == m.pixel_depth as int
    &&& pkt_of_mh(m).sample_bytes == m.sample_bytes
    &&& pkt_of_mh(m).n == m.n
    &&& pkt_of_mh(m).map_first == m.map_first as int
    &&& pkt_of_mh(m).map_len == m.map_length as int
    &&& pkt_of_mh(m).map_depth == m.map_entry_depth as int
    &&& pkt_of_mh(m).attr == (m.descriptor & 0x0fu8) as int
}

pub proof fn lemma_mp_pkt_h(m: MHModel)
    ensures
        mp_pkt_h(m),
{
    reveal(mp_pkt_h);
    reveal(pkt_of_mh);
}

/// R10: the raw pixel phase, with `k` of the `N` samples already decoded and
/// their converted pixels in `acc` (input order). Sample `k` lives at absolute
/// offset `c0 + k*sample_bytes`. Availability is checked before the sample read;
/// an indexed sample's index is checked at that pixel, before any later byte is
/// inspected. Returns the produced pixels and the final cursor;
/// `ColorIndexOutOfRange` carries the cursor just after the bad sample,
/// `TruncatedPixels` the cursor of the missing sample. The ghost `produced`
/// count is `k`.
pub open spec fn mp_raw_acc(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    c0: int,
    k: int,
    acc: Seq<Seq<u8>>,
) -> Result<(Seq<Seq<u8>>, int), (Error, int, int, Seq<Seq<u8>>)>
    decreases (p.n - k) as nat
{
    if k < 0 || k > p.n || p.sample_bytes <= 0 {
        // OutsideDomain: not a reachable raw cursor (a reachable `MH` record has
        // `1 <= sample_bytes <= 4`).
        Err((Error::ColorIndexOutOfRange, c0 + k * p.sample_bytes, k, acc))
    } else if k == p.n {
        Ok((acc, c0 + k * p.sample_bytes))
    } else if c0 + k * p.sample_bytes + p.sample_bytes > b.len() {
        Err((Error::TruncatedPixels, c0 + k * p.sample_bytes, k, acc))
    } else {
        let smp = b.subrange(c0 + k * p.sample_bytes, c0 + k * p.sample_bytes + p.sample_bytes);
        if p.kind == 0 && !in_range(p, sindex(smp, p.sample_bytes)) {
            Err((Error::ColorIndexOutOfRange, c0 + k * p.sample_bytes + p.sample_bytes, k, acc))
        } else {
            mp_raw_acc(b, p, map, c0, k + 1, acc + seq![conv2(p, map, smp)])
        }
    }
}

/// R10 from the start of pixel data: no sample decoded yet, the whole `N`.
pub open spec fn mp_raw(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    c0: int,
) -> Result<(Seq<Seq<u8>>, int), (Error, int, int, Seq<Seq<u8>>)> {
    mp_raw_acc(b, p, map, c0, 0, Seq::empty())
}

/// The input-order converted pixels of the first `k` raw samples: `acc_conv[k]` is
/// the R8 conversion of the sample at absolute offset `c0 + j*sample_bytes`.
pub open spec fn acc_conv(b: Seq<u8>, p: PktH, map: Seq<u8>, c0: int, k: int) -> Seq<Seq<u8>> {
    let sb = p.sample_bytes;
    Seq::new(k as nat, |j: int| conv2(
        p,
        map,
        b.subrange(c0 + j * sb, c0 + j * sb + sb),
    ))
}

/// `acc_conv` at `k+1` is `acc_conv` at `k` plus the `k`th sample's conversion.
pub proof fn lemma_acc_conv_step(b: Seq<u8>, p: PktH, map: Seq<u8>, c0: int, k: int)
    requires
        0 <= k,
    ensures
        acc_conv(b, p, map, c0, k + 1) == acc_conv(b, p, map, c0, k) + seq![conv2(
            p,
            map,
            b.subrange(c0 + k * p.sample_bytes, c0 + k * p.sample_bytes + p.sample_bytes),
        )],
{
    reveal(acc_conv);
    broadcast use vstd::seq::group_seq_lemmas;
    let sb = p.sample_bytes;
    assert(acc_conv(b, p, map, c0, k + 1) =~= acc_conv(b, p, map, c0, k) + seq![conv2(
        p,
        map,
        b.subrange(c0 + k * sb, c0 + k * sb + sb),
    )]) by {
        assert forall|i: int| 0 <= i < k + 1 implies #[trigger] acc_conv(b, p, map, c0, k + 1)[i]
            == (acc_conv(b, p, map, c0, k) + seq![conv2(
            p,
            map,
            b.subrange(c0 + k * sb, c0 + k * sb + sb),
        )])[i] by {
            if i < k {
                assert(acc_conv(b, p, map, c0, k)[i] == acc_conv(b, p, map, c0, k + 1)[i]);
                assert((acc_conv(b, p, map, c0, k) + seq![conv2(
                    p,
                    map,
                    b.subrange(c0 + k * sb, c0 + k * sb + sb),
                )])[i] == acc_conv(b, p, map, c0, k)[i]);
            } else {
                assert(i == k);
                assert(acc_conv(b, p, map, c0, k + 1)[k] == conv2(
                    p,
                    map,
                    b.subrange(c0 + k * sb, c0 + k * sb + sb),
                ));
                assert((acc_conv(b, p, map, c0, k) + seq![conv2(
                    p,
                    map,
                    b.subrange(c0 + k * sb, c0 + k * sb + sb),
                )])[k] == conv2(p, map, b.subrange(c0 + k * sb, c0 + k * sb + sb)));
            }
        }
    }
}

/// The self-consistency of a successful `MH` record: the derived fields are the
/// record's own formulas over the raw fields, and the checks' side conditions
/// hold. Everything `MP` needs about a header, without reference to the input
/// bytes' actual content beyond the slice length checks.
pub open spec fn mh_selfcons(m: MHModel) -> bool {
    &&& type_ok(m.image_type)
    &&& m.color_map_flag <= 1u8
    &&& depth_ok(m.image_type, m.pixel_depth)
    &&& m.n == m.width as int * (m.height as int)
    &&& m.sample_bytes == sample_bytes_of(m.pixel_depth)
    &&& m.map_first as int + m.map_length as int <= 65536
    &&& (m.color_map_flag == 1u8 ==> map_entry_depth_ok(m.map_entry_depth))
    &&& (m.color_map_flag == 1u8 ==> 1 <= m.map_length <= 65535)
    &&& (m.color_map_flag == 1u8 ==> m.map_bytes == m.map_length as int * sample_bytes_of(
        m.map_entry_depth,
    ))
    &&& (m.color_map_flag == 0u8 ==> m.map_bytes == 0 && m.map_length == 0 && m.map_first == 0)
    &&& (is_indexed(m.image_type) ==> m.color_map_flag == 1u8)
}

/// A successful `MH` record is self-consistent, and replays the record's own
/// formulas for `N`, `sample_bytes` and `map_bytes`.
pub proof fn lemma_mh_selfcons(b: Seq<u8>, m: MHModel)
    requires
        mh(b) == Ok(m),
    ensures
        mh_selfcons(m),
{
    reveal(mh);
    reveal(mh_selfcons);
    assert(b.len() >= 18);
    lemma_le16_lt(b, 3);
    lemma_le16_lt(b, 5);
    assert(le16(b, 3) + le16(b, 5) <= 65536);
    assert(!cm_bad(b));
    reveal(cm_bad);
    assert(!(!type_ok(b[2])));
    assert(!(b[1] > 1u8));
    assert(!(!depth_ok(b[2], b[16])));
    assert(!attr_ok(b) == false);
    reveal(attr_ok);
    if b[1] == 1u8 {
        assert(!(!map_entry_depth_ok(b[7])));
        assert(!(le16(b, 5) == 0));
        reveal(map_entry_depth_ok);
        assert(1 <= le16(b, 5) <= 65535);
    } else {
        assert(b[1] == 0u8);
    }
    assert(m == MHModel {
        id_length: b[0],
        color_map_flag: b[1],
        image_type: b[2],
        map_first: le16(b, 3) as u32,
        map_length: le16(b, 5) as u32,
        map_entry_depth: b[7],
        x_origin: le16(b, 8) as u32,
        y_origin: le16(b, 10) as u32,
        width: le16(b, 12) as u32,
        height: le16(b, 14) as u32,
        pixel_depth: b[16],
        descriptor: b[17],
        n: le16(b, 12) * le16(b, 14),
        sample_bytes: sample_bytes_of(b[16]),
        map_bytes: if b[1] == 1u8 {
            le16(b, 5) * sample_bytes_of(b[7])
        } else {
            0
        },
    });
    assert(type_ok(m.image_type));
}

/// `mp` propagates any `MH` error unchanged: the header check order of T5/T9.
pub proof fn lemma_mp_mh_err(b: Seq<u8>, e: Error)
    requires
        mh(b) == Err(e),
    ensures
        mp(b) == Err(e),
{
    reveal(mp);
}

/// `MP`'s header and slice phase: `Ok` records are narrowed to `u32` dimensions
/// and the map slice is exactly `map_bytes` bytes.
pub proof fn lemma_mp_after_trunc_id(b: Seq<u8>, m: MHModel)
    requires
        mh(b) == Ok(m),
        18 + (m.id_length as int) > b.len(),
    ensures
        mp_after(b, m) == Err(Error::TruncatedImageId),
{
    reveal(mp_after);
}

pub proof fn lemma_mp_after_trunc_map(b: Seq<u8>, m: MHModel)
    requires
        mh(b) == Ok(m),
        18 + (m.id_length as int) <= b.len(),
        18 + (m.id_length as int) + m.map_bytes > b.len(),
    ensures
        mp_after(b, m) == Err(Error::TruncatedColorMap),
{
    reveal(mp_after);
}

/// The map slice of `MP` has exactly `map_bytes` bytes.
pub proof fn lemma_mp_map_len(b: Seq<u8>, m: MHModel)
    requires
        mh(b) == Ok(m),
        18 + (m.id_length as int) + m.map_bytes <= b.len(),
    ensures
        mh_selfcons(m),
        b.subrange(18 + (m.id_length as int), 18 + (m.id_length as int) + m.map_bytes).len()
            == m.map_bytes,
{
    lemma_mh_selfcons(b, m);
    reveal(mh_selfcons);
}

/// `pkt_of_mh`'s conversion-domain facts for a successful record.
pub open spec fn mp_conv_domain(m: MHModel) -> bool {
    pkt_of_domain(pkt_of_mh(m))
}

pub proof fn lemma_mp_conv_domain(b: Seq<u8>, m: MHModel)
    requires
        mh(b) == Ok(m),
    ensures
        mh_selfcons(m),
        mp_conv_domain(m),
        pkt_of_mh(m).n == m.n,
        pkt_of_mh(m).sample_bytes == m.sample_bytes,
        pkt_of_mh(m).map_first == m.map_first as int,
        pkt_of_mh(m).map_len == m.map_length as int,
        pkt_of_mh(m).map_depth == m.map_entry_depth as int,
        pkt_of_mh(m).depth == m.pixel_depth as int,
        pkt_of_mh(m).kind == (if is_indexed(m.image_type) {
            0 as int
        } else if m.image_type == 2u8 || m.image_type == 10u8 {
            1 as int
        } else {
            2 as int
        }),
{
    lemma_mh_selfcons(b, m);
    reveal(mh_selfcons);
    reveal(mp_conv_domain);
    reveal(pkt_of_domain);
    reveal(pkt_of_mh);
    lemma_stride_eq(m.pixel_depth);
    let p = pkt_of_mh(m);
    assert(p.sample_bytes == m.sample_bytes);
    assert(p.sample_bytes == sample_bytes_of(m.pixel_depth));
    assert(stride_of(m.pixel_depth as int) == sample_bytes_of(m.pixel_depth));
    assert(p.kind == 0 || p.kind == 1 || p.kind == 2);
    reveal(depth_ok);
    reveal(is_indexed);
    if is_indexed(m.image_type) {
        assert(m.pixel_depth == 8u8 || m.pixel_depth == 16u8);
        assert(map_entry_depth_ok(m.map_entry_depth));
        reveal(map_entry_depth_ok);
        assert(m.map_entry_depth == 15u8 || m.map_entry_depth == 16u8 || m.map_entry_depth == 24u8
            || m.map_entry_depth == 32u8);
    } else if m.image_type == 2u8 || m.image_type == 10u8 {
        assert(m.pixel_depth == 15u8 || m.pixel_depth == 16u8 || m.pixel_depth == 24u8
            || m.pixel_depth == 32u8);
    } else {
        assert(m.pixel_depth == 8u8 || m.pixel_depth == 16u8);
    }
    lemma_sample_bytes_le_4(m.pixel_depth);
    assert(1 <= p.sample_bytes <= 4);
}

/// A successful `MH` record's raw header fields, read back from the input at
/// the R1 offsets, plus `attr_ok`. The Strict policy bridge needs the raw
/// fields (to compare `pkt_of_mh(m)` with `pkt_of_mh_policy(m, Strict)` field
/// by field) and the attribute-count fact (Strict's `Exact` acceptance agrees
/// with `mh`'s).
pub proof fn lemma_mh_fields(b: Seq<u8>, m: MHModel)
    requires
        mh(b) == Ok(m),
    ensures
        18 <= b.len(),
        m.id_length == b[0],
        m.color_map_flag == b[1],
        m.image_type == b[2],
        m.map_first == le16(b, 3) as u32,
        m.map_length == le16(b, 5) as u32,
        m.map_entry_depth == b[7],
        m.width == le16(b, 12) as u32,
        m.height == le16(b, 14) as u32,
        m.pixel_depth == b[16],
        m.descriptor == b[17],
        m.n == le16(b, 12) * le16(b, 14),
        m.sample_bytes == sample_bytes_of(b[16]),
        attr_ok(b),
{
    reveal(mh);
    reveal(mh_selfcons);
    lemma_mh_selfcons(b, m);
}

/// A successful raw cursor at the start stays `Ok` at every later `k <= N`: the
/// recursion only advances on decodable samples and stops at `N`.
pub proof fn lemma_mp_raw_cur_mono_ok(b: Seq<u8>, p: PktH, c0: int, k: int, cend: int)
    requires
        mp_raw_cur(b, p, c0, 0) == Ok(cend),
        0 <= k <= p.n,
    ensures
        mp_raw_cur(b, p, c0, k) == Ok(cend),
    decreases k,
{
    if k > 0 {
        lemma_mp_raw_cur_mono_ok(b, p, c0, k - 1, cend);
        assert(0 <= k - 1 < p.n) by (nonlinear_arith)
            requires
                k > 0,
                k <= p.n,
        ;
        reveal(mp_raw_cur);
        if p.sample_bytes <= 0 {
            assert(mp_raw_cur(b, p, c0, k - 1) == Err(Error::ColorIndexOutOfRange));
        }
        if c0 + (k - 1) * p.sample_bytes + p.sample_bytes > b.len() {
            assert(mp_raw_cur(b, p, c0, k - 1) == Err(Error::TruncatedPixels));
        }
        if p.kind == 0 {
            let smp = b.subrange(c0 + (k - 1) * p.sample_bytes, c0 + (k - 1) * p.sample_bytes
                + p.sample_bytes);
            if !in_range(p, sindex(smp, p.sample_bytes)) {
                assert(mp_raw_cur(b, p, c0, k - 1) == Err(Error::ColorIndexOutOfRange));
            }
        }
    }
}

/// A successful 1.0 raw run makes every sample up to `N` decodable (present and
/// in range for an indexed image): `raw_prefix_ok` for every prefix.
pub proof fn lemma_raw_prefix_ok_of_cur_ok(b: Seq<u8>, p: PktH, map: Seq<u8>, c0: int, k: int, cend: int)
    requires
        mp_raw_cur(b, p, c0, 0) == Ok(cend),
        0 <= k <= p.n,
    ensures
        raw_prefix_ok(b, p, map, c0, k),
{
    reveal(raw_prefix_ok);
    reveal(raw_sample_ok);
    if p.n >= 0 {
        assert forall|j: int| 0 <= j < k implies #[trigger] raw_sample_ok(b, p, map, c0, j) by {
            assert(0 <= j < p.n) by (nonlinear_arith)
                requires
                    0 <= j,
                    j < k,
                    k <= p.n,
            ;
            lemma_mp_raw_cur_mono_ok(b, p, c0, j, cend);
            reveal(mp_raw_cur);
            if p.sample_bytes <= 0 {
                assert(mp_raw_cur(b, p, c0, j) == Err(Error::ColorIndexOutOfRange));
            }
            if c0 + j * p.sample_bytes + p.sample_bytes > b.len() {
                assert(mp_raw_cur(b, p, c0, j) == Err(Error::TruncatedPixels));
            }
            if p.kind == 0 {
                let smp = b.subrange(c0 + j * p.sample_bytes, c0 + j * p.sample_bytes
                    + p.sample_bytes);
                if !in_range(p, sindex(smp, p.sample_bytes)) {
                    assert(mp_raw_cur(b, p, c0, j) == Err(Error::ColorIndexOutOfRange));
                }
            }
        }
    }
}

/// One raw sample is decodable at absolute offset `c0 + j*sample_bytes`: its
/// `sample_bytes` bytes are present and, for an indexed image, its index is in
/// range. These are exactly the two guards the executable's raw loop passes
/// before converting the sample.
pub open spec fn raw_sample_ok(b: Seq<u8>, p: PktH, map: Seq<u8>, c0: int, j: int) -> bool {
    &&& c0 + j * p.sample_bytes + p.sample_bytes <= b.len()
    &&& (p.kind == 0 ==> in_range(
        p,
        sindex(
            b.subrange(c0 + j * p.sample_bytes, c0 + j * p.sample_bytes + p.sample_bytes),
            p.sample_bytes,
        ),
    ))
}

/// The guards of a present, already-sliced sample: what `decode_one` needs.
pub open spec fn raw_sample_ok_at(p: PktH, s: Seq<u8>) -> bool {
    p.sample_bytes >= 1 && s.len() == p.sample_bytes && (p.kind == 0 ==> in_range(
        p,
        sindex(s, p.sample_bytes),
    ))
}

/// The cursor-and-error projection of `mp_raw`: the same ordered checks and the
/// same final cursor, without the pixel bookkeeping. This is what a decoder loop
/// can carry as a single equation.
pub open spec fn mp_raw_cur(b: Seq<u8>, p: PktH, c0: int, k: int) -> Result<int, Error>
    decreases (p.n - k) as nat
{
    if k < 0 || k > p.n || p.sample_bytes <= 0 {
        Err(Error::ColorIndexOutOfRange)
    } else if k == p.n {
        Ok(c0 + k * p.sample_bytes)
    } else if c0 + k * p.sample_bytes + p.sample_bytes > b.len() {
        Err(Error::TruncatedPixels)
    } else {
        let smp = b.subrange(c0 + k * p.sample_bytes, c0 + k * p.sample_bytes + p.sample_bytes);
        if p.kind == 0 && !in_range(p, sindex(smp, p.sample_bytes)) {
            Err(Error::ColorIndexOutOfRange)
        } else {
            mp_raw_cur(b, p, c0, k + 1)
        }
    }
}

/// `mp_raw_cur` is exactly the cursor/error projection of `mp_raw_acc`, for every
/// starting `k` and every accumulator. No pixel content is needed: the two
/// recursions take the same branches, so this is a one-line induction.
pub proof fn lemma_mp_raw_cur_proj(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    c0: int,
    k: int,
    acc: Seq<Seq<u8>>,
)
    ensures
        match mp_raw_acc(b, p, map, c0, k, acc) {
            Ok(r) => mp_raw_cur(b, p, c0, k) == Ok(r.1),
            Err((e, _c, _pr, _px)) => mp_raw_cur(b, p, c0, k) == Err(e),
        },
    decreases (p.n - k) as nat,
{
    if k < 0 || k > p.n || p.sample_bytes <= 0 {
        reveal(mp_raw_acc);
        reveal(mp_raw_cur);
    } else if k == p.n {
        reveal(mp_raw_acc);
        reveal(mp_raw_cur);
    } else if c0 + k * p.sample_bytes + p.sample_bytes > b.len() {
        reveal(mp_raw_acc);
        reveal(mp_raw_cur);
    } else {
        reveal(mp_raw_acc);
        reveal(mp_raw_cur);
        let smp = b.subrange(c0 + k * p.sample_bytes, c0 + k * p.sample_bytes + p.sample_bytes);
        if p.kind == 0 && !in_range(p, sindex(smp, p.sample_bytes)) {
        } else {
            lemma_mp_raw_cur_proj(b, p, map, c0, k + 1, acc + seq![conv2(p, map, smp)]);
        }
    }
}

/// `MP`'s raw phase fails, and its first error is `e`.
pub open spec fn mp_raw_errs(b: Seq<u8>, p: PktH, map: Seq<u8>, c0: int, e: Error) -> bool {
    match mp_raw(b, p, map, c0) {
        Ok(_) => false,
        Err(t) => t.0 == e,
    }
}

/// The error projection of `MP`'s raw phase: if the cursor-only model reports
/// error `e`, the full model (pixels included) reports the same first error.
pub proof fn lemma_mp_raw_err(b: Seq<u8>, p: PktH, map: Seq<u8>, c0: int, e: Error)
    requires
        mp_raw_cur(b, p, c0, 0) == Err(e),
    ensures
        mp_raw_errs(b, p, map, c0, e),
{
    reveal(mp_raw_errs);
    reveal(mp_raw);
    lemma_mp_raw_cur_proj(b, p, map, c0, 0, Seq::<Seq<u8>>::empty());
    match mp_raw(b, p, map, c0) {
        Ok(r) => {
            assert(mp_raw_cur(b, p, c0, 0) == Ok(r.1));
        },
        Err(t) => {
            assert(mp_raw_cur(b, p, c0, 0) == Err(t.0));
            assert(t.0 == e);
        },
    }
}

// ---------------------------------------------------------------------------
// The packet stream's success shape: 1.0-only facts about a successful
// `MR`/`mr_at` at `(cur, prod)`, used by the Strict policy bridge. A success
// at a present control byte with a fitting packet (`count <= N-prod`, the one
// place `MR` and `mr_cur` differ) fixes the next cursor, and each branch fixes
// the pixel sequence, from the recursion's own shape.
// ---------------------------------------------------------------------------

/// `MR(s,h,map,cur,prod,out0) == Ok r` at a fitting packet gives the produced
/// count and the pixel sequence. The final cursor `r.0` is the whole stream's,
/// carried unchanged by the caller.
pub proof fn lemma_mr_ok_shape(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    cur: int,
    prod: int,
    out0: Seq<Seq<u8>>,
    r: (int, int, Seq<Seq<u8>>),
)
    requires
        mr_at(s, h, map, cur, prod, out0) == Ok(r),
        0 <= h.sample_bytes,
    ensures
        r.1 == prod + mr_pix(s, h, map, cur, prod).len(),
        r.2 == out0 + mr_pix(s, h, map, cur, prod),
{
    lemma_mr_cur_pix_proj(s, h, map, cur, prod, out0);
}

/// The pixels `MR` produces from `(cur, prod)` at a fitting repeat packet: the
/// converted sample repeated `count` times, then the continuation.
pub proof fn lemma_mr_pix_ok_rep(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    cur: int,
    prod: int,
    c: Seq<u8>,
    count: int,
)
    requires
        0 <= cur < s.len(),
        0 <= prod < h.n,
        rle_count(s[cur] as int) == count,
        count <= h.n - prod,
        rle_repeat(s[cur] as int),
        s.len() >= cur + 1 + h.sample_bytes,
        c == conv2(h, map, s.subrange(cur + 1, cur + 1 + h.sample_bytes)),
        h.kind == 0 ==> in_range(
            h,
            sindex(s.subrange(cur + 1, cur + 1 + h.sample_bytes), h.sample_bytes),
        ),
    ensures
        mr_pix(s, h, map, cur, prod) == pix_pow(c, count) + mr_pix(
            s,
            h,
            map,
            cur + 1 + h.sample_bytes,
            prod + count,
        ),
{
    reveal(mr_pix);
    reveal(rle_repeat);
}

/// The pixels `MR` produces at a fitting literal packet whose every sample has
/// an in-range index: the conversions of all `count` samples, then the
/// continuation.
pub proof fn lemma_mr_pix_ok_lit(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    cur: int,
    prod: int,
    count: int,
)
    requires
        0 <= cur < s.len(),
        0 <= prod < h.n,
        rle_count(s[cur] as int) == count,
        count <= h.n - prod,
        !rle_repeat(s[cur] as int),
        s.len() >= cur + 1 + count * h.sample_bytes,
        lit_ok(s, h, cur + 1, count),
        0 <= h.sample_bytes,
    ensures
        mr_pix(s, h, map, cur, prod) == lit_out(s, h, map, cur + 1, count) + mr_pix(
            s,
            h,
            map,
            cur + 1 + count * h.sample_bytes,
            prod + count,
        ),
        lit_good(s, h, cur + 1, count) == count,
{
    reveal(mr_pix);
    reveal(rle_repeat);
    lemma_rle_repeat_clear(s[cur]);
    if h.kind == 0 {
        lemma_lit_good_good_all(s, h, cur + 1, count);
    } else {
        lemma_lit_good_nonindexed(s, h, cur + 1, count);
    }
}

/// The pixels `MR` produces at a fitting literal packet with an out-of-range
/// index: only the good prefix. `g` is `lit_good`, the sample at which the
/// index check fails, so `MR`'s pixel sequence stops there.
pub proof fn lemma_mr_pix_ok_lit_prefix(
    s: Seq<u8>,
    h: PktH,
    map: Seq<u8>,
    cur: int,
    prod: int,
    count: int,
    g: int,
)
    requires
        0 <= cur < s.len(),
        0 <= prod < h.n,
        rle_count(s[cur] as int) == count,
        count <= h.n - prod,
        !rle_repeat(s[cur] as int),
        s.len() >= cur + 1 + count * h.sample_bytes,
        lit_good(s, h, cur + 1, count) == g,
        0 <= h.sample_bytes,
        h.kind == 0,
        !lit_ok(s, h, cur + 1, count),
    ensures
        mr_pix(s, h, map, cur, prod) == lit_out(s, h, map, cur + 1, g),
{
    reveal(mr_pix);
    reveal(rle_repeat);
    lemma_rle_repeat_clear(s[cur]);
}

/// The success projection of `MP`'s raw phase: if the cursor-only model accepts,
/// the full model accepts with the same final cursor.
pub proof fn lemma_mp_raw_ok_end(b: Seq<u8>, p: PktH, map: Seq<u8>, c0: int, cend: int)
    requires
        mp_raw_cur(b, p, c0, 0) == Ok(cend),
    ensures
        match mp_raw(b, p, map, c0) {
            Ok(r) => r.1 == cend,
            Err(_) => false,
        },
{
    reveal(mp_raw);
    lemma_mp_raw_cur_proj(b, p, map, c0, 0, Seq::<Seq<u8>>::empty());
    match mp_raw(b, p, map, c0) {
        Ok(r) => {
            assert(mp_raw_cur(b, p, c0, 0) == Ok(r.1));
        },
        Err(t) => {
            assert(mp_raw_cur(b, p, c0, 0) == Err(t.0));
        },
    }
}

/// One raw iteration's branch, as the model's cursor projection sees it: when
/// sample `k` is missing the model's next result is `TruncatedPixels`.
pub proof fn lemma_mp_raw_cur_trunc(b: Seq<u8>, p: PktH, c0: int, k: int)
    requires
        0 <= k < p.n,
        p.sample_bytes >= 1,
        c0 + k * p.sample_bytes + p.sample_bytes > b.len(),
    ensures
        mp_raw_cur(b, p, c0, k) == Err(Error::TruncatedPixels),
{
    reveal(mp_raw_cur);
}

/// When sample `k`'s index is out of range the model's next result is
/// `ColorIndexOutOfRange`.
pub proof fn lemma_mp_raw_cur_index(b: Seq<u8>, p: PktH, c0: int, k: int)
    requires
        0 <= k < p.n,
        p.sample_bytes >= 1,
        c0 + k * p.sample_bytes + p.sample_bytes <= b.len(),
        p.kind == 0,
        !in_range(
            p,
            sindex(
                b.subrange(c0 + k * p.sample_bytes, c0 + k * p.sample_bytes + p.sample_bytes),
                p.sample_bytes,
            ),
        ),
    ensures
        mp_raw_cur(b, p, c0, k) == Err(Error::ColorIndexOutOfRange),
{
    reveal(mp_raw_cur);
}

/// When sample `k` is decodable the model's result is unchanged: the iteration
/// makes progress without changing the model's final answer.
pub proof fn lemma_mp_raw_cur_step(b: Seq<u8>, p: PktH, c0: int, k: int)
    requires
        0 <= k < p.n,
        p.sample_bytes >= 1,
        c0 + k * p.sample_bytes + p.sample_bytes <= b.len(),
        p.kind == 0 ==> in_range(
            p,
            sindex(
                b.subrange(c0 + k * p.sample_bytes, c0 + k * p.sample_bytes + p.sample_bytes),
                p.sample_bytes,
            ),
        ),
    ensures
        mp_raw_cur(b, p, c0, k) == mp_raw_cur(b, p, c0, k + 1),
{
    reveal(mp_raw_cur);
}

/// Every sample `j < k` is decodable.
pub open spec fn raw_prefix_ok(b: Seq<u8>, p: PktH, map: Seq<u8>, c0: int, k: int) -> bool {
    0 <= k && forall|j: int| 0 <= j < k ==> #[trigger] raw_sample_ok(b, p, map, c0, j)
}

/// Unpack `raw_prefix_ok` at one sample index.
pub proof fn lemma_raw_prefix_ok_at(b: Seq<u8>, p: PktH, map: Seq<u8>, c0: int, k: int, j: int)
    requires
        raw_prefix_ok(b, p, map, c0, k),
        0 <= j < k,
    ensures
        raw_sample_ok(b, p, map, c0, j),
{
    reveal(raw_prefix_ok);
}

/// Extend `raw_prefix_ok` by one decodable sample.
pub proof fn lemma_raw_prefix_ok_step(b: Seq<u8>, p: PktH, map: Seq<u8>, c0: int, k: int)
    requires
        raw_prefix_ok(b, p, map, c0, k),
        0 <= k,
        raw_sample_ok(b, p, map, c0, k),
    ensures
        raw_prefix_ok(b, p, map, c0, k + 1),
{
    reveal(raw_prefix_ok);
    assert forall|j: int| 0 <= j < k + 1 implies #[trigger] raw_sample_ok(b, p, map, c0, j) by {
        if j < k {
            lemma_raw_prefix_ok_at(b, p, map, c0, k, j);
        }
    }
}

/// Every sample `j < N` is decodable: the whole image decodes. The domain is
/// nonnegative.
pub open spec fn raw_total_ok(b: Seq<u8>, p: PktH, map: Seq<u8>, c0: int) -> bool {
    p.n >= 0 && forall|j: int| 0 <= j < p.n ==> #[trigger] raw_sample_ok(b, p, map, c0, j)
}

/// The whole image is the prefix of all `N` samples.
pub proof fn lemma_raw_total_to_prefix(b: Seq<u8>, p: PktH, map: Seq<u8>, c0: int)
    requires
        raw_total_ok(b, p, map, c0),
    ensures
        raw_prefix_ok(b, p, map, c0, p.n),
{
    reveal(raw_total_ok);
    reveal(raw_prefix_ok);
    assert forall|j: int| 0 <= j < p.n implies #[trigger] raw_sample_ok(b, p, map, c0, j) by {}
}

/// The whole image is the prefix of all `N` samples (and `N` is nonnegative).
pub proof fn lemma_raw_prefix_to_total(b: Seq<u8>, p: PktH, map: Seq<u8>, c0: int)
    requires
        raw_prefix_ok(b, p, map, c0, p.n),
        p.n >= 0,
    ensures
        raw_total_ok(b, p, map, c0),
{
    reveal(raw_prefix_ok);
    reveal(raw_total_ok);
    assert forall|j: int| 0 <= j < p.n implies #[trigger] raw_sample_ok(b, p, map, c0, j) by {}
}

/// Unpack `raw_total_ok` at one sample index.
pub proof fn lemma_raw_total_ok_at(b: Seq<u8>, p: PktH, map: Seq<u8>, c0: int, j: int)
    requires
        raw_total_ok(b, p, map, c0),
        0 <= j < p.n,
    ensures
        raw_sample_ok(b, p, map, c0, j),
{
    reveal(raw_total_ok);
}

/// `acc_conv` of no samples is the empty pixel sequence.
pub proof fn lemma_acc_conv_zero(b: Seq<u8>, p: PktH, map: Seq<u8>, c0: int)
    ensures
        acc_conv(b, p, map, c0, 0) == Seq::<Seq<u8>>::empty(),
{
    reveal(acc_conv);
    broadcast use vstd::seq::group_seq_lemmas;
    assert(acc_conv(b, p, map, c0, 0).len() == 0);
    assert(acc_conv(b, p, map, c0, 0) =~= Seq::<Seq<u8>>::empty());
}

/// `mp_raw` is `mp_raw_acc` at its starting point.
pub proof fn lemma_mp_raw_unfold_root(b: Seq<u8>, p: PktH, map: Seq<u8>, c0: int)
    ensures
        mp_raw(b, p, map, c0) == mp_raw_acc(b, p, map, c0, 0, Seq::<Seq<u8>>::empty()),
{
    reveal(mp_raw);
}

/// The raw model run from the start on a fully decodable image: all `N`
/// converted pixels in input order, cursor just past the last sample.
pub proof fn lemma_mp_raw_final(b: Seq<u8>, p: PktH, map: Seq<u8>, c0: int, k: int)
    requires
        0 <= k <= p.n,
        p.sample_bytes >= 1,
        raw_prefix_ok(b, p, map, c0, p.n),
    ensures
        mp_raw_acc(b, p, map, c0, k, acc_conv(b, p, map, c0, k)) == Ok(
            (acc_conv(b, p, map, c0, p.n), c0 + p.n * p.sample_bytes),
        ),
    decreases (p.n - k) as nat,
{
    if k == p.n {
        reveal(mp_raw_acc);
    } else {
        lemma_raw_prefix_ok_at(b, p, map, c0, p.n, k);
        lemma_mp_raw_step_ok(b, p, map, c0, k);
        lemma_mp_raw_final(b, p, map, c0, k + 1);
    }
}

/// The whole raw image decodes: `mp_raw` is `Ok` with all `N` pixels.
pub proof fn lemma_mp_raw_ok(b: Seq<u8>, p: PktH, map: Seq<u8>, c0: int)
    requires
        p.sample_bytes >= 1,
        raw_prefix_ok(b, p, map, c0, p.n),
    ensures
        mp_raw(b, p, map, c0) == Ok((acc_conv(b, p, map, c0, p.n), c0 + p.n * p.sample_bytes)),
{
    lemma_mp_raw_unfold_root(b, p, map, c0);
    lemma_acc_conv_zero(b, p, map, c0);
    lemma_mp_raw_final(b, p, map, c0, 0);
}

/// `k` successful raw steps are `mp_raw_acc` run from the start: the prefix
/// relation the truncation and index-error lemmas start from.
pub proof fn lemma_mp_raw_prefix(b: Seq<u8>, p: PktH, map: Seq<u8>, c0: int, k: int)
    requires
        0 <= k <= p.n,
        p.sample_bytes >= 1,
        raw_prefix_ok(b, p, map, c0, k),
    ensures
        mp_raw_acc(b, p, map, c0, 0, Seq::<Seq<u8>>::empty()) == mp_raw_acc(
            b,
            p,
            map,
            c0,
            k,
            acc_conv(b, p, map, c0, k),
        ),
    decreases k,
{
    if k == 0 {
        lemma_acc_conv_zero(b, p, map, c0);
    } else {
        lemma_mp_raw_prefix(b, p, map, c0, k - 1);
        lemma_raw_prefix_ok_at(b, p, map, c0, k, k - 1);
        lemma_mp_raw_step_ok(b, p, map, c0, k - 1);
    }
}

/// The raw model's first error when sample `k` is missing: `TruncatedPixels` at
/// that sample's cursor, with the `k` pixels before it already produced.
pub proof fn lemma_mp_raw_first_trunc(b: Seq<u8>, p: PktH, map: Seq<u8>, c0: int, k: int)
    requires
        0 <= k < p.n,
        p.sample_bytes >= 1,
        raw_prefix_ok(b, p, map, c0, k),
        c0 + k * p.sample_bytes + p.sample_bytes > b.len(),
    ensures
        mp_raw(b, p, map, c0) == Err((
            Error::TruncatedPixels,
            c0 + k * p.sample_bytes,
            k,
            acc_conv(b, p, map, c0, k),
        )),
{
    reveal(mp_raw);
    lemma_mp_raw_prefix(b, p, map, c0, k);
    lemma_mp_raw_step_trunc(b, p, map, c0, k);
}

/// The raw model's first error when sample `k`'s index is out of range:
/// `ColorIndexOutOfRange` just past that sample, with the `k` pixels before it
/// produced and no later sample inspected.
pub proof fn lemma_mp_raw_first_index(b: Seq<u8>, p: PktH, map: Seq<u8>, c0: int, k: int)
    requires
        0 <= k < p.n,
        p.sample_bytes >= 1,
        raw_prefix_ok(b, p, map, c0, k),
        c0 + k * p.sample_bytes + p.sample_bytes <= b.len(),
        p.kind == 0,
        !in_range(
            p,
            sindex(
                b.subrange(c0 + k * p.sample_bytes, c0 + k * p.sample_bytes + p.sample_bytes),
                p.sample_bytes,
            ),
        ),
    ensures
        mp_raw(b, p, map, c0) == Err((
            Error::ColorIndexOutOfRange,
            c0 + k * p.sample_bytes + p.sample_bytes,
            k,
            acc_conv(b, p, map, c0, k),
        )),
{
    reveal(mp_raw);
    lemma_mp_raw_prefix(b, p, map, c0, k);
    lemma_mp_raw_step_index(b, p, map, c0, k);
}

/// `acc_conv`'s `j`th pixel is the conversion of the `j`th sample.
pub proof fn lemma_acc_conv_index(b: Seq<u8>, p: PktH, map: Seq<u8>, c0: int, k: int, j: int)
    requires
        0 <= j < k,
    ensures
        acc_conv(b, p, map, c0, k)[j] == conv2(
            p,
            map,
            b.subrange(c0 + j * p.sample_bytes, c0 + j * p.sample_bytes + p.sample_bytes),
        ),
{
    reveal(acc_conv);
    broadcast use vstd::seq::group_seq_lemmas;
}

/// One raw-model step, as `mp_raw_acc` executes it, at the loop's `acc_conv(k)`.
pub proof fn lemma_mp_raw_step(b: Seq<u8>, p: PktH, map: Seq<u8>, c0: int, k: int)
    requires
        0 <= k < p.n,
        p.sample_bytes >= 1,
    ensures
        mp_raw_acc(b, p, map, c0, k, acc_conv(b, p, map, c0, k)) == if c0 + k * p.sample_bytes
            + p.sample_bytes > b.len() {
            Err((Error::TruncatedPixels, c0 + k * p.sample_bytes, k, acc_conv(b, p, map, c0, k)))
        } else if p.kind == 0 && !in_range(
            p,
            sindex(
                b.subrange(c0 + k * p.sample_bytes, c0 + k * p.sample_bytes + p.sample_bytes),
                p.sample_bytes,
            ),
        ) {
            Err((
                Error::ColorIndexOutOfRange,
                c0 + k * p.sample_bytes + p.sample_bytes,
                k,
                acc_conv(b, p, map, c0, k),
            ))
        } else {
            mp_raw_acc(b, p, map, c0, k + 1, acc_conv(b, p, map, c0, k + 1))
        },
{
    reveal(mp_raw_acc);
    reveal(acc_conv);
    broadcast use vstd::seq::group_seq_lemmas;
    assert(k != p.n);
    if c0 + k * p.sample_bytes + p.sample_bytes > b.len() {
    } else {
        let smp = b.subrange(c0 + k * p.sample_bytes, c0 + k * p.sample_bytes + p.sample_bytes);
        if p.kind == 0 && !in_range(p, sindex(smp, p.sample_bytes)) {
        } else {
            lemma_acc_conv_step(b, p, map, c0, k);
        }
    }
}

/// The truncation branch of one raw step.
pub proof fn lemma_mp_raw_step_trunc(b: Seq<u8>, p: PktH, map: Seq<u8>, c0: int, k: int)
    requires
        0 <= k < p.n,
        p.sample_bytes >= 1,
        k < p.n,
        c0 + k * p.sample_bytes + p.sample_bytes > b.len(),
    ensures
        mp_raw_acc(b, p, map, c0, k, acc_conv(b, p, map, c0, k)) == Err((
            Error::TruncatedPixels,
            c0 + k * p.sample_bytes,
            k,
            acc_conv(b, p, map, c0, k),
        )),
{
    reveal(mp_raw_acc);
    reveal(acc_conv);
    broadcast use vstd::seq::group_seq_lemmas;
}

/// The out-of-range-index branch of one raw step.
pub proof fn lemma_mp_raw_step_index(b: Seq<u8>, p: PktH, map: Seq<u8>, c0: int, k: int)
    requires
        0 <= k < p.n,
        p.sample_bytes >= 1,
        c0 + k * p.sample_bytes + p.sample_bytes <= b.len(),
        p.kind == 0,
        !in_range(
            p,
            sindex(
                b.subrange(c0 + k * p.sample_bytes, c0 + k * p.sample_bytes + p.sample_bytes),
                p.sample_bytes,
            ),
        ),
    ensures
        mp_raw_acc(b, p, map, c0, k, acc_conv(b, p, map, c0, k)) == Err((
            Error::ColorIndexOutOfRange,
            c0 + k * p.sample_bytes + p.sample_bytes,
            k,
            acc_conv(b, p, map, c0, k),
        )),
{
    reveal(mp_raw_acc);
    reveal(acc_conv);
    broadcast use vstd::seq::group_seq_lemmas;
    assert(k != p.n);
    assert(!(k < 0));
    assert(!(k > p.n));
}

/// The successful branch of one raw step.
pub proof fn lemma_mp_raw_step_ok(b: Seq<u8>, p: PktH, map: Seq<u8>, c0: int, k: int)
    requires
        0 <= k < p.n,
        p.sample_bytes >= 1,
        raw_sample_ok(b, p, map, c0, k),
    ensures
        mp_raw_acc(b, p, map, c0, k, acc_conv(b, p, map, c0, k)) == mp_raw_acc(
            b,
            p,
            map,
            c0,
            k + 1,
            acc_conv(b, p, map, c0, k + 1),
        ),
{
    reveal(raw_sample_ok);
    lemma_mp_raw_step(b, p, map, c0, k);
}

/// `mp_raw_acc` never produces more than `N` pixels.
pub proof fn lemma_mp_raw_acc_len(
    b: Seq<u8>,
    p: PktH,
    map: Seq<u8>,
    c0: int,
    k: int,
    acc: Seq<Seq<u8>>,
)
    requires
        0 <= k <= p.n,
        p.sample_bytes >= 1,
    ensures
        match mp_raw_acc(b, p, map, c0, k, acc) {
            Ok(r) => r.0.len() == acc.len() + (p.n - k) && r.1 == c0 + p.n * p.sample_bytes,
            Err(_) => true,
        },
    decreases (p.n - k) as nat,
{
    reveal(mp_raw_acc);
    if k == p.n {
    } else if c0 + k * p.sample_bytes + p.sample_bytes > b.len() {
    } else {
        let smp = b.subrange(c0 + k * p.sample_bytes, c0 + k * p.sample_bytes + p.sample_bytes);
        if p.kind == 0 && !in_range(p, sindex(smp, p.sample_bytes)) {
        } else {
            let acc1 = acc + seq![conv2(p, map, smp)];
            assert(acc1.len() == acc.len() + 1) by {
                broadcast use vstd::seq::group_seq_axioms;
            }
            lemma_mp_raw_acc_len(b, p, map, c0, k + 1, acc1);
            assert(p.n - (k + 1) + 1 == p.n - k) by (nonlinear_arith)
                requires
                    k < p.n,
            ;
            assert(acc1.len() + (p.n - (k + 1)) == acc.len() + (p.n - k)) by (nonlinear_arith)
                requires
                    acc1.len() == acc.len() + 1,
                    p.n - (k + 1) + 1 == p.n - k,
            ;
        }
    }
}

/// R12's suffix rule after the last pixel: the cursor must equal the input end,
/// or leave a footer-carrying suffix of at least 26 bytes.
pub open spec fn mp_suffix_ok(b: Seq<u8>, cursor: int) -> bool {
    let suffix = b.subrange(cursor, b.len() as int);
    suffix.len() == 0 || footer(suffix)
}

/// The `j`th byte of a four-byte pixel.
pub open spec fn px_byte(q: Seq<u8>, j: int) -> u8 {
    if j == 0 {
        q[0]
    } else if j == 1 {
        q[1]
    } else if j == 2 {
        q[2]
    } else {
        q[3]
    }
}

/// R9/R8: the oriented, RGBA-interleaved output of a decoded pixel sequence:
/// `4*N` bytes, byte `i` is byte `i%4` of pixel `gcoord(i/4)` (the orientation
/// bijection's inverse). Every byte is written exactly once; `f(g(q)) = q` makes
/// cell `i/4` the image of source sample `g(i/4)`.
pub open spec fn orient_bytes(
    px: Seq<Seq<u8>>,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    n: int,
) -> Seq<u8> {
    Seq::new((4 * n) as nat, |i: int| px_byte(px[gcoord(w, hgt, fx, fy, i / 4)], i % 4))
}

/// The pixels `MP` decodes, in input (logical sample) order: the raw samples'
/// conversions, or the packet stream's produced pixels.
pub open spec fn mp_pixels(b: Seq<u8>, m: MHModel, id_end: int) -> Seq<Seq<u8>> {
    let p = pkt_of_mh(m);
    let map = b.subrange(id_end, id_end + m.map_bytes);
    let c0 = id_end + m.map_bytes;
    if m.image_type == 9u8 || m.image_type == 10u8 || m.image_type == 11u8 {
        mr_pix(b, p, map, c0, 0)
    } else {
        acc_conv(b, p, map, c0, m.n)
    }
}

/// R9/R8/R13: the oriented output bytes of a fully decoded image.
pub open spec fn mp_assemble(b: Seq<u8>, m: MHModel, id_end: int) -> Seq<u8> {
    orient_bytes(
        mp_pixels(b, m, id_end),
        m.width as int,
        m.height as int,
        m.descriptor & 0x10u8 != 0u8,
        m.descriptor & 0x20u8 != 0u8,
        m.n,
    )
}

/// R9/R8 at the pixel level, offset by `base`: pixel `j` of the decoded
/// sequence is logical sample `base + j`, written at oriented cell `f(base+j)`.
/// `rle_quad_ok` is the `base = 0` case.
pub open spec fn rle_quad_ok_off(
    out: Seq<u8>,
    px: Seq<Seq<u8>>,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    base: int,
) -> bool {
    forall|j: int| 0 <= j < px.len() ==> #[trigger] quad_at(out, 4 * fcoord(w, hgt, fx, fy, base
        + j), px[j])
}

/// R9/R8 at the pixel level: pixel `j` of the decoded sequence sits at the
/// oriented cell `fcoord(j)` with its exact four bytes.
pub open spec fn rle_quad_ok(
    out: Seq<u8>,
    px: Seq<Seq<u8>>,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
) -> bool {
    forall|j: int| 0 <= j < px.len() ==> #[trigger] quad_at(out, 4 * fcoord(w, hgt, fx, fy, j), px[j])
}

/// `i/4 < n` from `i < 4*n` and `n >= 0`, via the division identity.
pub proof fn lemma_div4_lt(i: int, n: int)
    requires
        0 <= i < 4 * n,
    ensures
        0 <= i / 4 < n,
{
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod(i, 4);
    assert(i == 4 * (i / 4) + i % 4);
    assert(0 <= i % 4);
    assert(4 * (i / 4) <= i) by (nonlinear_arith)
        requires
            i == 4 * (i / 4) + i % 4,
            0 <= i % 4,
    ;
    assert(0 <= i / 4) by (nonlinear_arith)
        requires
            i >= 0,
    ;
    assert(i / 4 < n) by (nonlinear_arith)
        requires
            4 * (i / 4) <= i,
            i < 4 * n,
            n >= 0,
    ;
}

/// Every byte of `orient_bytes` is byte `i%4` of the source sample's conversion
/// at the bijection inverse, for `1 <= w,h <= 8192`, `px.len() = N = w*h`.
pub proof fn lemma_orient_bytes_index(
    px: Seq<Seq<u8>>,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    n: int,
    i: int,
)
    requires
        1 <= w <= 8192,
        1 <= hgt <= 8192,
        px.len() == n,
        w * hgt == n,
        0 <= i < 4 * n,
    ensures
        orient_bytes(px, w, hgt, fx, fy, n)[i] == px_byte(
            px[gcoord(w, hgt, fx, fy, i / 4)],
            i % 4,
        ),
        0 <= gcoord(w, hgt, fx, fy, i / 4) < n,
        fcoord(w, hgt, fx, fy, gcoord(w, hgt, fx, fy, i / 4)) == i / 4,
{
    reveal(orient_bytes);
    broadcast use vstd::seq::group_seq_lemmas;
    reveal(px_byte);
    lemma_div4_lt(i, n);
    assert(0 <= i / 4 < w * hgt);
    lemma_fcoord_inverse(w, hgt, fx, fy, i / 4);
    assert(0 <= gcoord(w, hgt, fx, fy, i / 4) < w * hgt);
}

/// `quad_ok` on a full pixel sequence and a full output buffer pins down every
/// output byte: `out == orient_bytes(px, ...)`.
pub proof fn lemma_quad_ok_to_orient(
    out: Seq<u8>,
    px: Seq<Seq<u8>>,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
)
    requires
        1 <= w,
        1 <= hgt,
        px.len() == w * hgt,
        out.len() == 4 * (w * hgt),
        rle_quad_ok(out, px, w, hgt, fx, fy),
    ensures
        out == orient_bytes(px, w, hgt, fx, fy, w * hgt),
{
    reveal(rle_quad_ok);
    reveal(orient_bytes);
    broadcast use vstd::seq::group_seq_lemmas;
    assert(out =~= orient_bytes(px, w, hgt, fx, fy, w * hgt)) by {
        assert forall|i: int| 0 <= i < 4 * (w * hgt) implies out[i] == #[trigger] orient_bytes(
            px,
            w,
            hgt,
            fx,
            fy,
            w * hgt,
        )[i] by {
            lemma_div4_lt(i, w * hgt);
            lemma_fcoord_inverse(w, hgt, fx, fy, i / 4);
            let k = gcoord(w, hgt, fx, fy, i / 4);
            assert(0 <= k < w * hgt);
            assert(quad_at(out, 4 * fcoord(w, hgt, fx, fy, k), px[k]));
            assert(fcoord(w, hgt, fx, fy, k) == i / 4);
            assert(4 * (i / 4) + i % 4 == i) by (nonlinear_arith)
                requires
                    i >= 0,
            ;
            reveal(quad_at);
            reveal(px_byte);
            if i % 4 == 0 {
            } else if i % 4 == 1 {
            } else if i % 4 == 2 {
            } else {
            }
        }
    }
}

/// `MP` after a successful `MH`: R7's ID and map slices, then R10's raw phase or
/// R11's packet stream, then R12's suffix test, then R9/R8's output assembly.
/// Every branch returns immediately with the first error.
pub open spec fn mp_after(b: Seq<u8>, m: MHModel) -> Result<(int, int, Seq<u8>), Error> {
    let id_end = 18 + (m.id_length as int);
    if id_end > b.len() {
        Err(Error::TruncatedImageId)
    } else {
        let map_end = id_end + m.map_bytes;
        if map_end > b.len() {
            Err(Error::TruncatedColorMap)
        } else {
            let map = b.subrange(id_end, map_end);
            let p = pkt_of_mh(m);
            if m.image_type == 9u8 || m.image_type == 10u8 || m.image_type == 11u8 {
                match mr(b, p, map, map_end, 0) {
                    // `map_end <= b.len()` and `0 <= 0 <= m.n`, so this call is
                    // in `MR`'s domain: the `Outside` arm is unreachable (and is
                    // the internal proof marker, never an API error).
                    Err((MrErr::Api(e), _c, _pr, _px)) => Err(e),
                    Err((MrErr::Outside, _c, _pr, _px)) => Err(Error::ColorIndexOutOfRange),
                    Ok((cursor, _produced, _px)) => {
                        if mp_suffix_ok(b, cursor) {
                            Ok((m.width as int, m.height as int, mp_assemble(b, m, id_end)))
                        } else {
                            Err(Error::TrailingBytes)
                        }
                    },
                }
            } else {
                match mp_raw(b, p, map, map_end) {
                    Err((e, _c, _pr, _px)) => Err(e),
                    Ok((_px, cursor)) => {
                        if mp_suffix_ok(b, cursor) {
                            Ok((m.width as int, m.height as int, mp_assemble(b, m, id_end)))
                        } else {
                            Err(Error::TrailingBytes)
                        }
                    },
                }
            }
        }
    }
}

/// `MP(b)`, the complete total decoder model: `MH` first (propagating its
/// error), then the ID/map slices, the pixel phase, the suffix test and the
/// oriented RGBA assembly.
pub open spec fn mp(b: Seq<u8>) -> Result<(int, int, Seq<u8>), Error> {
    match mh(b) {
        Err(e) => Err(e),
        Ok(m) => mp_after(b, m),
    }
}

/// `Accepted(b) iff exists w,h,p: MP(b)=Ok(w,h,p)`.
pub open spec fn accepted(b: Seq<u8>) -> bool {
    exists|w: int, h: int, p: Seq<u8>| mp(b) == Ok((w, h, p))
}

// ===========================================================================
// L1-L4: the four large vectors' symbolic lemmas (THEOREMS.md, "Non-vacuity
// and compact vector protocol").
//
// Each general statement is proved from the model's own definitions; each of
// the four vectors is then instantiated by a corollary that identifies the
// fixture's bytes with the manifest's symbolic construction. The input lengths,
// the packet sizes and the 67,108,864-byte output are established
// arithmetically (sequence-length lemmas, `pix_pow`, the orientation bijection),
// never by expanding the input or evaluating the output.
// ===========================================================================

/// A byte repeated `n` times: L1/L2's zero runs.
pub open spec fn rep_byte(v: u8, n: nat) -> Seq<u8> {
    Seq::new(n, |i: int| v)
}

pub proof fn lemma_rep_byte_len(v: u8, n: nat)
    ensures
        rep_byte(v, n).len() == n,
{
    reveal(rep_byte);
    vstd::seq::lemma_seq_new_len(n, |i: int| v);
}

pub proof fn lemma_rep_byte_index(v: u8, n: nat, i: int)
    requires
        0 <= i < n,
    ensures
        rep_byte(v, n)[i] == v,
{
    reveal(rep_byte);
    vstd::seq::lemma_seq_new_index(n, |i: int| v, i);
}

/// The manifest's `H(t,w,h,d,a)`: the 18-byte R1 header with image type `t`,
/// dimensions `w,h` (`0 <= w,h < 65,536`), pixel depth `d`, descriptor `a` and
/// every other field zero.
pub open spec fn hdr18(t: u8, w: int, h: int, d: u8, a: u8) -> Seq<u8> {
    Seq::new(18, |i: int| hdr18_byte(t, w, h, d, a, i))
}

pub open spec fn hdr18_byte(t: u8, w: int, h: int, d: u8, a: u8, i: int) -> u8 {
    if i == 0 || i == 1 {
        0u8
    } else if i == 2 {
        t
    } else if 3 <= i <= 11 {
        0u8
    } else if i == 12 {
        (w % 256) as u8
    } else if i == 13 {
        (w / 256) as u8
    } else if i == 14 {
        (h % 256) as u8
    } else if i == 15 {
        (h / 256) as u8
    } else if i == 16 {
        d
    } else if i == 17 {
        a
    } else {
        0u8
    }
}

pub proof fn lemma_hdr18_len(t: u8, w: int, h: int, d: u8, a: u8)
    ensures
        hdr18(t, w, h, d, a).len() == 18,
{
    reveal(hdr18);
    vstd::seq::lemma_seq_new_len(18, |i: int| hdr18_byte(t, w, h, d, a, i));
}

pub proof fn lemma_hdr18_index(t: u8, w: int, h: int, d: u8, a: u8, i: int)
    requires
        0 <= i < 18,
    ensures
        hdr18(t, w, h, d, a)[i] == hdr18_byte(t, w, h, d, a, i),
{
    reveal(hdr18);
    vstd::seq::lemma_seq_new_index(18, |i: int| hdr18_byte(t, w, h, d, a, i), i);
}

/// Index `i >= |p|` of `p + rest` is `rest[i - |p|]`.
pub proof fn lemma_tail_index(p: Seq<u8>, rest: Seq<u8>, i: int)
    requires
        p.len() <= i < p.len() + rest.len(),
    ensures
        (p + rest)[i] == rest[i - p.len()],
{
    vstd::seq::lemma_seq_add_len(p, rest);
    vstd::seq::lemma_seq_add_index2(p, rest, i);
}

/// `MH` reads only the first 18 bytes and the total input length, so a header
/// prefix that keeps the input under the cap gives the same header verdict.
pub proof fn lemma_mh_prefix(p: Seq<u8>, rest: Seq<u8>)
    requires
        p.len() == 18,
        18 + rest.len() <= max_input(),
    ensures
        mh(p + rest) == mh(p),
{
    reveal(mh);
    reveal(max_input);
    reveal(max_dimension);
    reveal(max_output);
    reveal(le16);
    reveal(type_ok);
    reveal(is_indexed);
    reveal(depth_ok);
    reveal(cm_bad);
    reveal(cm_range_bad);
    reveal(attr_ok);
    reveal(map_entry_depth_ok);
    reveal(map_attr_ok);
    vstd::seq::lemma_seq_add_len(p, rest);
    assert forall|i: int| 0 <= i < 18 implies #[trigger] (p + rest)[i] == p[i] by {
        vstd::seq::lemma_seq_add_index1(p, rest, i);
    }
    assert(le16(p + rest, 3) == le16(p, 3));
    assert(le16(p + rest, 5) == le16(p, 5));
    assert(le16(p + rest, 12) == le16(p, 12));
    assert(le16(p + rest, 14) == le16(p, 14));
}

/// A four-byte pixel pattern repeated `count` times: the R16 run list expanded,
/// and (for a constant decoded pixel) exactly `MP`'s oriented output.
pub open spec fn rgba_pat(c: Seq<u8>, count: nat) -> Seq<u8> {
    Seq::new((4 * count) as nat, |i: int| c[i % 4])
}

pub proof fn lemma_rgba_pat_len(c: Seq<u8>, count: nat)
    ensures
        rgba_pat(c, count).len() == 4 * count,
{
    reveal(rgba_pat);
    vstd::seq::lemma_seq_new_len((4 * count) as nat, |i: int| c[i % 4]);
}

/// A repeat stream of period `d`: byte `i` is `pats[i % d]` (`d = |pats|`).
pub open spec fn rep_pats(pats: Seq<u8>, k: nat) -> Seq<u8> {
    Seq::new((pats.len() * k) as nat, |i: int| pats[i % (pats.len() as int)])
}

pub proof fn lemma_rep_pats_len(pats: Seq<u8>, k: nat)
    requires
        pats.len() > 0,
    ensures
        rep_pats(pats, k).len() == pats.len() * k,
{
    reveal(rep_pats);
    vstd::seq::lemma_seq_new_len(
        (pats.len() * k) as nat,
        |i: int| pats[i % (pats.len() as int)],
    );
}

pub proof fn lemma_rep_pats_index(pats: Seq<u8>, k: nat, i: int)
    requires
        pats.len() > 0,
        0 <= i < pats.len() * k,
    ensures
        rep_pats(pats, k)[i] == pats[i % (pats.len() as int)],
{
    reveal(rep_pats);
    vstd::seq::lemma_seq_new_index(
        (pats.len() * k) as nat,
        |i: int| pats[i % (pats.len() as int)],
        i,
    );
}

// ---------------------------------------------------------------------------
// L1 — input overflow.
// ---------------------------------------------------------------------------

/// L1: `forall b, |b| > 67,108,864: MP(b) = Err(FileTooLarge)`.
pub proof fn lemma_l1_oversized(b: Seq<u8>)
    requires
        b.len() > 67108864,
    ensures
        mp(b) == Err(Error::FileTooLarge),
{
    reveal(mp);
    reveal(mh);
    reveal(max_input);
}

/// L1's manifest instantiation `input-over-64m`: a zero byte repeated 67,108,865
/// times.
pub proof fn lemma_l1_input_over_64m()
    ensures
        mp(rep_byte(0u8, 67108865)) == Err(Error::FileTooLarge),
{
    lemma_rep_byte_len(0u8, 67108865);
    lemma_l1_oversized(rep_byte(0u8, 67108865));
}

// ---------------------------------------------------------------------------
// L2 — exact input boundary.
// ---------------------------------------------------------------------------

/// L2: `forall b, |b| = 67,108,864 and b[2] = 0: MP(b) = Err(UnsupportedImageType)`.
/// This rejects `>=` in place of `>` in R1's length test.
pub proof fn lemma_l2_at_limit(b: Seq<u8>)
    requires
        b.len() == 67108864,
        b[2] == 0u8,
    ensures
        mp(b) == Err(Error::UnsupportedImageType),
{
    reveal(mp);
    reveal(mh);
    reveal(max_input);
    reveal(type_ok);
    assert(!(b.len() > max_input()));
    assert(!(b.len() < 18));
    assert(!type_ok(b[2]));
}

/// L2's manifest instantiation `input-at-64m`: a zero byte repeated 67,108,864
/// times.
pub proof fn lemma_l2_input_at_64m()
    ensures
        mp(rep_byte(0u8, 67108864)) == Err(Error::UnsupportedImageType),
{
    lemma_rep_byte_len(0u8, 67108864);
    lemma_rep_byte_index(0u8, 67108864, 2);
    lemma_l2_at_limit(rep_byte(0u8, 67108864));
}

/// The zero-run lengths L1/L2 use: `|rep_byte(0, n)| = n` is the whole reason
/// R1's strict `>` test rejects `n = MAX_INPUT` and accepts `n = MAX_INPUT + 1`.
pub proof fn lemma_l1_l2_lengths()
    ensures
        rep_byte(0u8, 67108865).len() == 67108865,
        rep_byte(0u8, 67108864).len() == 67108864,
        rep_byte(0u8, 67108864).len() == max_input(),
{
    reveal(max_input);
    lemma_rep_byte_len(0u8, 67108865);
    lemma_rep_byte_len(0u8, 67108864);
}

// ===========================================================================
// L3/L4 — the two maximum-output vectors.
//
// Both are an 18-byte R1 header followed by one two-byte (respectively
// three-byte) repeat packet written 131,072 times: 128 pixels per packet times
// 131,072 packets is exactly 16,777,216 pixels, i.e. R6's exact output cap of
// 67,108,864 RGBA bytes. `l3_hdr`/`l4_hdr` are the manifest's
// `H(11,8192,2048,8,32)`/`H(10,4096,4096,16,33)` written out byte by byte; the
// header verdicts and the single-packet steps are then decidable by `compute`,
// and the periodicity lemma lifts that one packet to all 131,072 of them
// without expanding the stream.
// ===========================================================================

/// Byte `i` of `hdr + pats^k` (where `hdr` is 18 bytes) is `pats[i % |pats|]`.
pub proof fn lemma_pkt_at(hdr: Seq<u8>, pats: Seq<u8>, k: nat, i: int)
    requires
        hdr.len() == 18,
        pats.len() > 0,
        0 <= i < pats.len() * k,
    ensures
        (hdr + rep_pats(pats, k))[18 + i] == pats[i % (pats.len() as int)],
{
    broadcast use vstd::seq::group_seq_lemmas;
    lemma_rep_pats_len(pats, k);
    vstd::seq::lemma_seq_add_len(hdr, rep_pats(pats, k));
    assert(rep_pats(pats, k).len() == pats.len() * k);
    assert(18 + i < hdr.len() + rep_pats(pats, k).len()) by (nonlinear_arith)
        requires
            hdr.len() == 18,
            18 + i < 18 + pats.len() * k,
            0 <= i,
    ;
    vstd::seq::lemma_seq_add_index2(hdr, rep_pats(pats, k), 18 + i);
    assert(18 + i - hdr.len() == i) by (nonlinear_arith)
        requires
            hdr.len() == 18,
    ;
    lemma_rep_pats_index(pats, k, i);
}

/// `pats^1 == pats` for a positive-length pattern.
pub proof fn lemma_rep_pats_one(pats: Seq<u8>)
    requires
        pats.len() > 0,
    ensures
        rep_pats(pats, 1) == pats,
{
    broadcast use vstd::seq::group_seq_lemmas;
    lemma_rep_pats_len(pats, 1);
    assert(rep_pats(pats, 1).len() == pats.len());
    assert(rep_pats(pats, 1) =~= pats) by {
        assert forall|i: int| 0 <= i < pats.len() implies rep_pats(pats, 1)[i]
            == #[trigger] pats[i] by {
            lemma_rep_pats_index(pats, 1, i);
            vstd::arithmetic::div_mod::lemma_small_mod(i as nat, pats.len() as nat);
            assert(i % (pats.len() as int) == i);
        }
    }
}

/// The control byte of the `j`th packet of `hdr + pats^k` is the pattern's when
/// `|pats| = 1 + sb`.
pub proof fn lemma_pkt_control(hdr: Seq<u8>, pats: Seq<u8>, k: nat, sb: int, j: int)
    requires
        hdr.len() == 18,
        pats.len() == 1 + sb,
        0 <= sb,
        0 <= j < k,
    ensures
        (hdr + rep_pats(pats, k))[18 + j * (1 + sb)] == pats[0],
{
    broadcast use vstd::seq::group_seq_lemmas;
    assert(pats.len() > 0) by (nonlinear_arith)
        requires
            pats.len() == 1 + sb,
            0 <= sb,
    ;
    assert(j * (1 + sb) < pats.len() * k) by (nonlinear_arith)
        requires
            pats.len() == 1 + sb,
            0 <= sb,
            j < k,
    ;
    lemma_pkt_at(hdr, pats, k, j * (1 + sb));
    assert(j * (1 + sb) % (pats.len() as int) == 0) by {
        vstd::arithmetic::div_mod::lemma_mod_multiples_vanish(j, 0, 1 + sb);
        vstd::arithmetic::div_mod::lemma_small_mod(0nat, (1 + sb) as nat);
        assert(pats.len() as int == 1 + sb);
    }
}

/// The `j`th packet's sample window in `hdr + pats^k` is the pattern's.
pub proof fn lemma_pkt_window(hdr: Seq<u8>, pats: Seq<u8>, k: nat, sb: int, j: int)
    requires
        hdr.len() == 18,
        pats.len() == 1 + sb,
        1 <= 1 + sb,
        0 <= j < k,
    ensures
        (hdr + rep_pats(pats, k)).subrange(
            18 + j * (1 + sb) + 1,
            18 + j * (1 + sb) + 1 + sb,
        ) == pats.subrange(1, 1 + sb),
{
    broadcast use vstd::seq::group_seq_lemmas;
    let s = hdr + rep_pats(pats, k);
    let lo = 18 + j * (1 + sb) + 1;
    let hi = lo + sb;
    lemma_rep_pats_len(pats, k);
    vstd::seq::lemma_seq_add_len(hdr, rep_pats(pats, k));
    assert(rep_pats(pats, k).len() == pats.len() * k);
    assert(pats.len() == 1 + sb);
    assert(s.len() == 18 + (1 + sb) * k) by (nonlinear_arith)
        requires
            s.len() == hdr.len() + rep_pats(pats, k).len(),
            hdr.len() == 18,
            rep_pats(pats, k).len() == (1 + sb) * k,
    ;
    assert(hi <= s.len()) by (nonlinear_arith)
        requires
            s.len() == 18 + (1 + sb) * k,
            hi == 18 + j * (1 + sb) + 1 + sb,
            j < k,
            sb >= 0,
    ;
    vstd::seq::lemma_seq_subrange_len(pats, 1, 1 + sb);
    assert(pats.subrange(1, 1 + sb).len() == sb);
    assert(s.subrange(lo, hi) =~= pats.subrange(1, 1 + sb)) by {
        assert forall|t: int| 0 <= t < sb implies s.subrange(lo, hi)[t] == pats.subrange(
            1,
            1 + sb,
        )[t] by {
            broadcast use vstd::seq::group_seq_lemmas;
            vstd::seq::lemma_seq_subrange_index(pats, 1, 1 + sb, t);
            vstd::seq::lemma_seq_subrange_index(s, lo, hi, t);
            assert(s.subrange(lo, hi)[t] == s[lo + t]);
            assert(lo + t == 18 + (j * (1 + sb) + 1 + t)) by (nonlinear_arith)
                requires
                    lo == 18 + j * (1 + sb) + 1,
            ;
            assert(s[18 + (j * (1 + sb) + 1 + t)] == rep_pats(pats, k)[j * (1 + sb) + 1 + t]) by {
                assert(18 + (j * (1 + sb) + 1 + t) - hdr.len() == j * (1 + sb) + 1 + t) by (
                nonlinear_arith)
                    requires
                        hdr.len() == 18,
                ;
                vstd::seq::lemma_seq_add_index2(
                    hdr,
                    rep_pats(pats, k),
                    18 + (j * (1 + sb) + 1 + t),
                );
            }
            assert(j * (1 + sb) + 1 + t < pats.len() * k) by (nonlinear_arith)
                requires
                    pats.len() == 1 + sb,
                    j < k,
                    0 <= t < sb,
            ;
            lemma_rep_pats_index(pats, k, j * (1 + sb) + 1 + t);
            assert((j * (1 + sb) + 1 + t) % ((1 + sb) as int) == 1 + t) by {
                vstd::arithmetic::div_mod::lemma_mod_multiples_vanish(j, 1 + t, 1 + sb);
                vstd::arithmetic::div_mod::lemma_small_mod((1 + t) as nat, (1 + sb) as nat);
            }
            assert((j * (1 + sb) + 1 + t) % (pats.len() as int) == 1 + t) by {
                assert(pats.len() as int == 1 + sb);
            }
            assert(pats[(j * (1 + sb) + 1 + t) % (pats.len() as int)] == pats[1 + t]) by {
                assert((j * (1 + sb) + 1 + t) % (pats.len() as int) == 1 + t);
            }
            assert(pats.subrange(1, 1 + sb)[t] == pats[1 + t]) by {
                vstd::seq::lemma_seq_subrange_index(pats, 1, 1 + sb, t);
                assert(pats.subrange(1, 1 + sb)[t] == pats[t + 1]);
                assert(pats[t + 1] == pats[1 + t]) by (nonlinear_arith);
            }
        }
    }
}

/// One packet of a periodic repeat stream steps the model exactly like the first
/// packet of the pattern.
pub proof fn lemma_packet_step_periodic(
    hdr: Seq<u8>,
    pats: Seq<u8>,
    k: nat,
    h: PktH,
    c: Seq<u8>,
    j: int,
)
    requires
        hdr.len() == 18,
        pats.len() == pkt_len(h.sample_bytes),
        h.n == 128 * k,
        packet_step(hdr + pats, h, Seq::<u8>::empty(), 18, 0, h.sample_bytes, c),
        0 <= j < k,
    ensures
        packet_step(
            hdr + rep_pats(pats, k),
            h,
            Seq::<u8>::empty(),
            18 + j * pkt_len(h.sample_bytes),
            j * 128,
            h.sample_bytes,
            c,
        ),
{
    broadcast use vstd::seq::group_seq_lemmas;
    reveal(packet_step);
    reveal(pkt_len);
    let sb = h.sample_bytes;
    let s = hdr + rep_pats(pats, k);
    let cur = 18 + j * pkt_len(sb);
    assert(pkt_len(sb) == 1 + sb) by {
        reveal(pkt_len);
    }
    assert(pats.len() == 1 + sb);
    assert(sb >= 1) by {
        reveal(packet_step);
    }
    lemma_rep_pats_len(pats, k);
    vstd::seq::lemma_seq_add_len(hdr, rep_pats(pats, k));
    assert(rep_pats(pats, k).len() == pats.len() * k);
    assert(s.len() == 18 + (1 + sb) * k) by (nonlinear_arith)
        requires
            s.len() == hdr.len() + rep_pats(pats, k).len(),
            hdr.len() == 18,
            rep_pats(pats, k).len() == (1 + sb) * k,
    ;
    assert(cur == 18 + j * (1 + sb)) by {
        reveal(pkt_len);
    }
    assert(cur + 1 + sb <= s.len()) by (nonlinear_arith)
        requires
            cur == 18 + j * (1 + sb),
            s.len() == 18 + (1 + sb) * k,
            j < k,
            sb >= 0,
    ;
    assert(s[cur] == pats[0]) by {
        assert(cur == 18 + j * (1 + sb) + 0) by (nonlinear_arith)
            requires
                cur == 18 + j * (1 + sb),
        ;
        lemma_pkt_control(hdr, pats, k, sb, j);
        assert(18 + j * (1 + sb) == 18 + j * (1 + sb) + 0);
    }
    assert(s[cur] as int == 255) by {
        reveal(packet_step);
        assert(pats[0] as int == 255) by {
            assert((hdr + pats)[18] == pats[0]);
        }
    }
    lemma_pkt_window(hdr, pats, k, sb, j);
    assert(s.subrange(cur + 1, cur + 1 + sb) == pats.subrange(1, 1 + sb)) by {
        assert(cur + 1 == 18 + j * (1 + sb) + 1) by (nonlinear_arith)
            requires
                cur == 18 + j * (1 + sb),
        ;
        assert(cur + 1 + sb == 18 + j * (1 + sb) + 1 + sb) by (nonlinear_arith)
            requires
                cur == 18 + j * (1 + sb),
        ;
    }
    assert((hdr + pats).subrange(18 + 0 * (1 + sb) + 1, 18 + 0 * (1 + sb) + 1 + sb)
        == pats.subrange(1, 1 + sb)) by {
        lemma_pkt_window(hdr, pats, 1, sb, 0);
        assert(1 <= 1 + sb) by (nonlinear_arith)
            requires
                sb >= 0,
        ;
    }
    assert(conv2(h, Seq::<u8>::empty(), s.subrange(cur + 1, cur + 1 + sb)) == c);
    assert(j * 128 + 128 <= h.n) by (nonlinear_arith)
        requires
            j < k,
            h.n == 128 * k,
    ;
}

/// Any constant pixel sequence oriented through R9's bijection is exactly that
/// pixel's R16 run list expanded.
pub proof fn lemma_orient_const(
    px: Seq<Seq<u8>>,
    c: Seq<u8>,
    w: int,
    hgt: int,
    fx: bool,
    fy: bool,
    n: int,
)
    requires
        1 <= w,
        1 <= hgt,
        w * hgt == n,
        px.len() == n,
        c.len() == 4,
        forall|j: int| 0 <= j < n ==> #[trigger] px[j] == c,
    ensures
        orient_bytes(px, w, hgt, fx, fy, n) == rgba_pat(c, n as nat),
{
    broadcast use vstd::seq::group_seq_lemmas;
    reveal(orient_bytes);
    reveal(rgba_pat);
    reveal(px_byte);
    lemma_rgba_pat_len(c, n as nat);
    assert(orient_bytes(px, w, hgt, fx, fy, n).len() == 4 * n) by {
        reveal(orient_bytes);
        vstd::seq::lemma_seq_new_len((4 * n) as nat, |i: int| px_byte(
            px[gcoord(w, hgt, fx, fy, i / 4)],
            i % 4,
        ));
    }
    assert(orient_bytes(px, w, hgt, fx, fy, n) =~= rgba_pat(c, n as nat)) by {
        assert forall|i: int| 0 <= i < 4 * n implies orient_bytes(
            px,
            w,
            hgt,
            fx,
            fy,
            n,
        )[i] == #[trigger] rgba_pat(c, n as nat)[i] by {
            lemma_div4_lt(i, n);
            lemma_fcoord_inverse(w, hgt, fx, fy, i / 4);
            let q = gcoord(w, hgt, fx, fy, i / 4);
            assert(0 <= q < n);
            assert(px[q] == c);
            assert(orient_bytes(px, w, hgt, fx, fy, n)[i] == px_byte(c, i % 4)) by {
                vstd::seq::lemma_seq_new_index((4 * n) as nat, |i: int| px_byte(
                    px[gcoord(w, hgt, fx, fy, i / 4)],
                    i % 4,
                ), i);
            }
            vstd::seq::lemma_seq_new_index((4 * n) as nat, |i: int| c[i % 4], i);
            assert(0 <= i % 4 < 4) by (nonlinear_arith)
                requires
                    i >= 0,
            ;
            reveal(px_byte);
        }
    }
}

// ---------------------------------------------------------------------------
// L3 — maximum greyscale output.
// ---------------------------------------------------------------------------

/// `H(11,8192,2048,8,32)`, byte by byte.
pub open spec fn l3_hdr() -> Seq<u8> {
    seq![
        0x00u8, 0x00, 0x0b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x00,
        0x08, 0x08, 0x20,
    ]
}

/// The manifest's `S = bytes(0xff, 0x7f)`: control `0xff`, one greyscale sample
/// `0x7f`.
pub open spec fn l3_s() -> Seq<u8> {
    seq![0xffu8, 0x7f]
}

/// The decoded colour of L3's sample: greyscale depth 8, attribute count zero,
/// so R8 gives `[Y,Y,Y,255]` for `Y = 0x7f`.
pub open spec fn l3_c() -> Seq<u8> {
    seq![0x7fu8, 0x7f, 0x7f, 0xff]
}

/// L3's header record `H(11,8192,2048,8,32)`.
pub open spec fn l3_m() -> MHModel {
    MHModel {
        id_length: 0u8,
        color_map_flag: 0u8,
        image_type: 11u8,
        map_first: 0u32,
        map_length: 0u32,
        map_entry_depth: 0u8,
        x_origin: 0u32,
        y_origin: 0u32,
        width: 8192u32,
        height: 2048u32,
        pixel_depth: 8u8,
        descriptor: 32u8,
        n: 16777216,
        sample_bytes: 1,
        map_bytes: 0,
    }
}

/// L3's input `H(11,8192,2048,8,32) + S^131072`.
pub open spec fn l3_input() -> Seq<u8> {
    l3_hdr() + rep_pats(l3_s(), 131072)
}

/// L3: `MP(b) = Ok(8192, 2048, (bytes(0x7f,0x7f,0x7f,0xff))^16777216)`,
/// `|b| = 262,162` and `|rgba| = 67,108,864`.
pub proof fn lemma_l3_output_at_64m_8192x2048()
    ensures
        l3_input().len() == 262162,
        mp(l3_input()) == Ok((8192, 2048, rgba_pat(l3_c(), 16777216))),
        rgba_pat(l3_c(), 16777216).len() == 67108864,
{
    broadcast use vstd::seq::group_seq_lemmas;
    let b = l3_input();
    // length
    reveal(l3_hdr);
    reveal(l3_s);
    lemma_rep_pats_len(l3_s(), 131072);
    vstd::seq::lemma_seq_add_len(l3_hdr(), rep_pats(l3_s(), 131072));
    assert(rep_pats(l3_s(), 131072).len() == 262144);
    assert(l3_hdr().len() == 18);
    assert(b.len() == 262162);
    // the header verdict
    lemma_mh_prefix(l3_hdr(), rep_pats(l3_s(), 131072));
    reveal(max_input);
    assert(mh(b) == mh(l3_hdr()));
    assert(mh(l3_hdr()) == Ok(l3_m())) by (compute);
    // the single packet, decided by compute
    assert(packet_step(l3_hdr() + l3_s(), pkt_of_mh(l3_m()), Seq::<u8>::empty(), 18, 0, 1, l3_c()))
        by (compute);
    // the packet stream
    let h = pkt_of_mh(l3_m());
    reveal(pkt_of_mh);
    reveal(is_indexed);
    reveal(pkt_len);
    reveal(l3_m);
    assert(h.sample_bytes == 1);
    assert(h.n == 16777216);
    assert(pkt_len(h.sample_bytes) == 2);
    lemma_rep_pats_one(l3_s());
    assert(pkt_len(h.sample_bytes) == l3_s().len()) by {
        reveal(l3_s);
    }
    assert forall|j: int| 0 <= j < 131072 implies #[trigger] packet_step(
        b,
        h,
        Seq::<u8>::empty(),
        18 + j * pkt_len(h.sample_bytes),
        0 + j * 128,
        h.sample_bytes,
        l3_c(),
    ) by {
        assert(packet_step(
            l3_hdr() + rep_pats(l3_s(), 1),
            h,
            Seq::<u8>::empty(),
            18,
            0,
            1,
            l3_c(),
        )) by {
            lemma_rep_pats_one(l3_s());
        }
        lemma_packet_step_periodic(l3_hdr(), l3_s(), 131072, h, l3_c(), j);
    }
    lemma_packet_repetition_mr(b, h, Seq::<u8>::empty(), 18, 0, 131072, l3_c());
    assert(mr(b, h, Seq::<u8>::empty(), 18, 0) == Ok((
        262162,
        16777216,
        pix_pow(l3_c(), 16777216),
    )));
    // the projections and the suffix
    lemma_mr_cur_pix_proj(b, h, Seq::<u8>::empty(), 18, 0, Seq::<Seq<u8>>::empty());
    assert(mr_cur(b, h, Seq::<u8>::empty(), 18, 0) == Ok((262162, 16777216)));
    assert(mr_pix(b, h, Seq::<u8>::empty(), 18, 0) == pix_pow(l3_c(), 16777216)) by {
        lemma_pix_pow_len(l3_c(), 16777216);
        vstd::seq::lemma_seq_add_len(Seq::<Seq<u8>>::empty(), mr_pix(
            b,
            h,
            Seq::<u8>::empty(),
            18,
            0,
        ));
    }
    assert(mp_id_end(l3_m()) == 18) by {
        reveal(mp_id_end);
    }
    assert(mp_map_end(l3_m()) == 18) by {
        reveal(mp_map_end);
        reveal(l3_m);
    }
    assert(b.subrange(18, 18) == Seq::<u8>::empty());
    assert(mp_suffix_ok(b, 262162)) by {
        reveal(mp_suffix_ok);
        vstd::seq::lemma_seq_subrange_len(b, 262162, b.len() as int);
        assert(b.subrange(262162, b.len() as int).len() == 0);
    }
    lemma_mp_rle_ok(b, l3_m(), 262162, pix_pow(l3_c(), 16777216));
    assert(orient_bytes(
        pix_pow(l3_c(), 16777216),
        8192,
        2048,
        l3_m().descriptor & 0x10u8 != 0u8,
        l3_m().descriptor & 0x20u8 != 0u8,
        16777216,
    ) == rgba_pat(l3_c(), 16777216)) by {
        assert forall|j: int| 0 <= j < 16777216 implies #[trigger] pix_pow(
            l3_c(),
            16777216,
        )[j] == l3_c() by {
            lemma_pix_pow_index(l3_c(), 16777216, j);
        }
        reveal(l3_c);
        assert(l3_c().len() == 4);
        lemma_orient_const(
            pix_pow(l3_c(), 16777216),
            l3_c(),
            8192,
            2048,
            l3_m().descriptor & 0x10u8 != 0u8,
            l3_m().descriptor & 0x20u8 != 0u8,
            16777216,
        );
    }
    lemma_rgba_pat_len(l3_c(), 16777216);
}

// ---------------------------------------------------------------------------
// L4 — maximum true-colour output.
// ---------------------------------------------------------------------------

/// `H(10,4096,4096,16,33)`, byte by byte.
pub open spec fn l4_hdr() -> Seq<u8> {
    seq![
        0x00u8, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00,
        0x10, 0x10, 0x21,
    ]
}

/// The manifest's `C = bytes(0xff, 0x00, 0xfc)`: control `0xff`, one 16-bit
/// true-colour sample. Its little-endian word is `0xfc00`: red field
/// `(v>>10)&31 = 31`, green `(v>>5)&31 = 0`, blue `v&31 = 0`.
pub open spec fn l4_c0() -> Seq<u8> {
    seq![0xffu8, 0x00, 0xfc]
}

/// The decoded colour of L4's sample: 16-bit true-colour, red 31 -> 255,
/// green 0, blue 0, so `[0xff, 0x00, 0x00, 0xff]`, alpha 255 for both
/// attribute counts.
pub open spec fn l4_c() -> Seq<u8> {
    seq![0xffu8, 0x00, 0x00, 0xff]
}

/// L4's header record `H(10,4096,4096,16,33)`.
pub open spec fn l4_m() -> MHModel {
    MHModel {
        id_length: 0u8,
        color_map_flag: 0u8,
        image_type: 10u8,
        map_first: 0u32,
        map_length: 0u32,
        map_entry_depth: 0u8,
        x_origin: 0u32,
        y_origin: 0u32,
        width: 4096u32,
        height: 4096u32,
        pixel_depth: 16u8,
        descriptor: 33u8,
        n: 16777216,
        sample_bytes: 2,
        map_bytes: 0,
    }
}

/// L4's input `H(10,4096,4096,16,33) + C^131072`.
pub open spec fn l4_input() -> Seq<u8> {
    l4_hdr() + rep_pats(l4_c0(), 131072)
}

/// L4: `MP(b) = Ok(4096, 4096, (bytes(0xff,0,0,0xff))^16777216)`,
/// `|b| = 393,234` and `|rgba| = 67,108,864`.
pub proof fn lemma_l4_output_at_64m_4096_square()
    ensures
        l4_input().len() == 393234,
        mp(l4_input()) == Ok((4096, 4096, rgba_pat(l4_c(), 16777216))),
        rgba_pat(l4_c(), 16777216).len() == 67108864,
{
    broadcast use vstd::seq::group_seq_lemmas;
    let b = l4_input();
    // length
    reveal(l4_hdr);
    reveal(l4_c0);
    lemma_rep_pats_len(l4_c0(), 131072);
    vstd::seq::lemma_seq_add_len(l4_hdr(), rep_pats(l4_c0(), 131072));
    assert(rep_pats(l4_c0(), 131072).len() == 393216);
    assert(l4_hdr().len() == 18);
    assert(b.len() == 393234);
    // the header verdict
    lemma_mh_prefix(l4_hdr(), rep_pats(l4_c0(), 131072));
    reveal(max_input);
    assert(mh(b) == mh(l4_hdr()));
    assert(mh(l4_hdr()) == Ok(l4_m())) by (compute);
    // the single packet, decided by compute
    assert(packet_step(l4_hdr() + l4_c0(), pkt_of_mh(l4_m()), Seq::<u8>::empty(), 18, 0, 2, l4_c()))
        by (compute);
    // the packet stream
    let h = pkt_of_mh(l4_m());
    reveal(pkt_of_mh);
    reveal(is_indexed);
    reveal(pkt_len);
    reveal(l4_m);
    assert(h.sample_bytes == 2);
    assert(h.n == 16777216);
    assert(pkt_len(h.sample_bytes) == 3);
    lemma_rep_pats_one(l4_c0());
    assert(pkt_len(h.sample_bytes) == l4_c0().len()) by {
        reveal(l4_c0);
    }
    assert forall|j: int| 0 <= j < 131072 implies #[trigger] packet_step(
        b,
        h,
        Seq::<u8>::empty(),
        18 + j * pkt_len(h.sample_bytes),
        0 + j * 128,
        h.sample_bytes,
        l4_c(),
    ) by {
        assert(packet_step(
            l4_hdr() + rep_pats(l4_c0(), 1),
            h,
            Seq::<u8>::empty(),
            18,
            0,
            2,
            l4_c(),
        )) by {
            lemma_rep_pats_one(l4_c0());
        }
        lemma_packet_step_periodic(l4_hdr(), l4_c0(), 131072, h, l4_c(), j);
    }
    lemma_packet_repetition_mr(b, h, Seq::<u8>::empty(), 18, 0, 131072, l4_c());
    assert(mr(b, h, Seq::<u8>::empty(), 18, 0) == Ok((
        393234,
        16777216,
        pix_pow(l4_c(), 16777216),
    )));
    // the projections and the suffix
    lemma_mr_cur_pix_proj(b, h, Seq::<u8>::empty(), 18, 0, Seq::<Seq<u8>>::empty());
    assert(mr_cur(b, h, Seq::<u8>::empty(), 18, 0) == Ok((393234, 16777216)));
    assert(mr_pix(b, h, Seq::<u8>::empty(), 18, 0) == pix_pow(l4_c(), 16777216)) by {
        lemma_pix_pow_len(l4_c(), 16777216);
        vstd::seq::lemma_seq_add_len(Seq::<Seq<u8>>::empty(), mr_pix(
            b,
            h,
            Seq::<u8>::empty(),
            18,
            0,
        ));
    }
    assert(mp_id_end(l4_m()) == 18) by {
        reveal(mp_id_end);
    }
    assert(mp_map_end(l4_m()) == 18) by {
        reveal(mp_map_end);
        reveal(l4_m);
    }
    assert(b.subrange(18, 18) == Seq::<u8>::empty());
    assert(mp_suffix_ok(b, 393234)) by {
        reveal(mp_suffix_ok);
        vstd::seq::lemma_seq_subrange_len(b, 393234, b.len() as int);
        assert(b.subrange(393234, b.len() as int).len() == 0);
    }
    lemma_mp_rle_ok(b, l4_m(), 393234, pix_pow(l4_c(), 16777216));
    assert(orient_bytes(
        pix_pow(l4_c(), 16777216),
        4096,
        4096,
        l4_m().descriptor & 0x10u8 != 0u8,
        l4_m().descriptor & 0x20u8 != 0u8,
        16777216,
    ) == rgba_pat(l4_c(), 16777216)) by {
        assert forall|j: int| 0 <= j < 16777216 implies #[trigger] pix_pow(
            l4_c(),
            16777216,
        )[j] == l4_c() by {
            lemma_pix_pow_index(l4_c(), 16777216, j);
        }
        reveal(l4_c);
        assert(l4_c().len() == 4);
        lemma_orient_const(
            pix_pow(l4_c(), 16777216),
            l4_c(),
            4096,
            4096,
            l4_m().descriptor & 0x10u8 != 0u8,
            l4_m().descriptor & 0x20u8 != 0u8,
            16777216,
        );
    }
    lemma_rgba_pat_len(l4_c(), 16777216);
}

} // verus!
