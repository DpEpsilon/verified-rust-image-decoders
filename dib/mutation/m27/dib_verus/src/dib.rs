//! Verified DIB/BMP/ICO-entry decoder: the public `decode` has exact MP result
//! and a ghost work counter attached to its concrete loops (T9 and T12).
//!
//! `src/dib.rs.m1` preserves the original plain implementation. Every input
//! read is guarded, and each output write is proved in bounds. The work counter
//! charges the library zero fill by bytes touched, each raw row/pixel body,
//! each RLE command/painter body, and each ICO alpha/AND row/pixel body.

use vstd::prelude::*;

verus! {

/// R1: the input is at most 64 MiB; a longer slice is `FileTooLarge`.
pub const MAX_INPUT: usize = 67_108_864;

/// R1/R5: the RGBA output is at most 64 MiB (`4*N <= MAX_OUTPUT`).
pub const MAX_OUTPUT: usize = 67_108_864;

/// R3: each visible dimension is in `[1, MAX_DIMENSION]`.
pub const MAX_DIMENSION: u32 = 8_192;

/// The four decode modes. R1: the DIB starts at 14 for `BmpFile`, at 0
/// otherwise; both ICO modes double the stored height and carry an AND mask.
/// R19: `IcoEntryDibSlack` is `IcoEntryDib` except that bytes after a complete
/// AND mask are ignored instead of `TrailingBytes`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    BmpFile,
    BareDib,
    IcoEntryDib,
    IcoEntryDibSlack,
}

/// R1/R19: the shared-ICO-mode predicate — either ICO mode. Every R1–R18 ICO
/// rule applies to a mode satisfying this; only R12's final cursor-equality
/// check is governed by `exact_tail` instead.
pub open spec fn is_ico(mode: Mode) -> bool {
    mode == Mode::IcoEntryDib || mode == Mode::IcoEntryDibSlack
}

/// R12/R14/R19: the exact-tail predicate — the modes that reject bytes after
/// the structure (`BareDib`, `IcoEntryDib`). `BmpFile` ignores a suffix and
/// `IcoEntryDibSlack` ignores post-AND bytes, so neither is `exact_tail`.
pub open spec fn exact_tail(mode: Mode) -> bool {
    mode == Mode::BareDib || mode == Mode::IcoEntryDib
}

/// R15: the closed, case-sensitive error set, in the order R15 lists it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    FileTooLarge,
    TruncatedFileHeader,
    BadMagic,
    InvalidFileSize,
    InvalidPixelOffset,
    TruncatedDibHeader,
    UnsupportedHeaderSize,
    TruncatedHeader,
    InvalidPlanes,
    InvalidDimensions,
    UnsupportedPixelDepth,
    UnsupportedCompression,
    InvalidCompressionDepth,
    UnsupportedOrientation,
    InvalidColorCount,
    OutputTooLarge,
    TruncatedMasks,
    InvalidBitfields,
    TruncatedPalette,
    TruncatedPixels,
    PaletteIndexOutOfRange,
    RleTruncated,
    RleBounds,
    InvalidImageSize,
    TruncatedAndMask,
    TrailingBytes,
}

impl Error {
    /// The exact R15 spelling, as used by `VECTORS.json` expectations.
    pub fn name(&self) -> &'static str {
        match self {
            Error::FileTooLarge => "FileTooLarge",
            Error::TruncatedFileHeader => "TruncatedFileHeader",
            Error::BadMagic => "BadMagic",
            Error::InvalidFileSize => "InvalidFileSize",
            Error::InvalidPixelOffset => "InvalidPixelOffset",
            Error::TruncatedDibHeader => "TruncatedDibHeader",
            Error::UnsupportedHeaderSize => "UnsupportedHeaderSize",
            Error::TruncatedHeader => "TruncatedHeader",
            Error::InvalidPlanes => "InvalidPlanes",
            Error::InvalidDimensions => "InvalidDimensions",
            Error::UnsupportedPixelDepth => "UnsupportedPixelDepth",
            Error::UnsupportedCompression => "UnsupportedCompression",
            Error::InvalidCompressionDepth => "InvalidCompressionDepth",
            Error::UnsupportedOrientation => "UnsupportedOrientation",
            Error::InvalidColorCount => "InvalidColorCount",
            Error::OutputTooLarge => "OutputTooLarge",
            Error::TruncatedMasks => "TruncatedMasks",
            Error::InvalidBitfields => "InvalidBitfields",
            Error::TruncatedPalette => "TruncatedPalette",
            Error::TruncatedPixels => "TruncatedPixels",
            Error::PaletteIndexOutOfRange => "PaletteIndexOutOfRange",
            Error::RleTruncated => "RleTruncated",
            Error::RleBounds => "RleBounds",
            Error::InvalidImageSize => "InvalidImageSize",
            Error::TruncatedAndMask => "TruncatedAndMask",
            Error::TrailingBytes => "TrailingBytes",
        }
    }
}

/// A decoded image: exact visible `u32` dimensions and exactly `4*width*height`
/// bytes of tightly packed, non-premultiplied RGBA8, left-to-right, top row
/// first (SPEC preamble).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decoded {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

// ---------------------------------------------------------------------------
// Bounds-proving byte readers.
//
// Unlike M1 these carry a precondition that the bytes exist. The proof never
// depends on a "returns 0 past the end" default: every call site proves its own
// guard, which is what T1's no-out-of-range-access clause requires.
// ---------------------------------------------------------------------------

/// One byte at `p`; the caller proves `p` is inside the slice.
#[inline]
fn rd(b: &[u8], p: usize) -> (r: u8)
    requires
        p < b@.len(),
    ensures
        r as int == b@[p as int],
{
    b[p]
}

/// `LE16(p)`; the caller proves both bytes exist.
#[inline]
fn rd16(b: &[u8], p: usize) -> (r: u32)
    requires
        p + 2 <= b@.len(),
    ensures
        r as int == crate::model::le16(b@, p as int),
{
    let lo = b[p] as u32;
    let hi = b[p + 1] as u32;
    proof {
        assert(crate::model::le16(b@, p as int) == b@[p as int] + 256 * (b@[p as int + 1] as int));
        assert((lo | (hi << 8)) as int == (lo as int) + ((hi << 8) as int)) by (bit_vector)
            requires
                (lo as int) < 256,
                (hi as int) < 256,
        ;
        assert(((hi << 8) as int) == 256 * (hi as int)) by (bit_vector)
            requires
                (hi as int) < 256,
        ;
    }
    lo | (hi << 8)
}

/// `LE32(p)`; the caller proves all four bytes exist.
#[inline]
fn rd32(b: &[u8], p: usize) -> (r: u32)
    requires
        p + 4 <= b@.len(),
    ensures
        r as int == crate::model::le32(b@, p as int),
{
    proof {
        assert(b@.len() == b.len());
        assert(b@.len() <= usize::MAX);
        assert(p + 2 <= usize::MAX) by (nonlinear_arith)
            requires
                p + 4 <= b@.len(),
                b@.len() <= usize::MAX,
        ;
    }
    proof {
        assert(b@.len() == b.len());
        assert(b@.len() <= usize::MAX);
    }
    let a = rd16(b, p);
    let c = rd16(b, p + 2);
    proof {
        assert(crate::model::le32(b@, p as int) == b@[p as int] + 256 * (b@[p as int + 1] as int)
            + 65536 * (b@[p as int + 2] as int) + 16777216 * (b@[p as int + 3] as int));
        assert(a as int == b@[p as int] + 256 * (b@[p as int + 1] as int));
        assert(c as int == b@[p as int + 2] + 256 * (b@[p as int + 3] as int));
        assert((a | (c << 16)) as int == (a as int) + ((c << 16) as int)) by (bit_vector)
            requires
                (a as int) < 65536,
                (c as int) < 65536,
        ;
        assert(((c << 16) as int) == 65536 * (c as int)) by (bit_vector)
            requires
                (c as int) < 65536,
        ;
    }
    a | (c << 16)
}

/// `S32(p)`; the caller proves all four bytes exist.
#[inline]
fn rd32s(b: &[u8], p: usize) -> (r: i32)
    requires
        p + 4 <= b@.len(),
    ensures
        r as int == crate::model::s32(b@, p as int),
{
    let u = rd32(b, p);
    proof {
        reveal(crate::model::s32);
        assert(u as int == crate::model::le32(b@, p as int));
        crate::model::lemma_le32_lt(b@, p as int);
        let l = crate::model::le32(b@, p as int);
        assert(0 <= l < 4294967296);
        assert((u as i32) as int == crate::model::s32(b@, p as int)) by {
            if l >= 2147483648 {
                assert(u >= 2147483648u32);
                assert((u as i32) as int == u as int - 4294967296) by (bit_vector)
                    requires
                        u >= 2147483648u32,
                ;
            } else {
                assert(u < 2147483648u32);
                assert((u as i32) as int == u as int) by (bit_vector)
                    requires
                        u < 2147483648u32,
                ;
            }
        }
    }
    u as i32
}

/// Bytes left from `pos`; the caller proves `pos <= len` (M1's saturating form,
/// with the guard stated instead of hidden inside `saturating_sub`).
#[inline]
fn rem(len: usize, pos: usize) -> (r: usize)
    requires
        pos <= len,
    ensures
        r == len - pos,
{
    len - pos
}

/// R9: `row_stride = 4*ceil(width*depth/32)`, four-byte aligned. `width <= 8192`
/// and `depth <= 32` bound the result by 32,768 (T5).
fn stride(width: u32, bits: u32) -> (r: usize)
    requires
        1 <= width <= 8192,
        1 <= bits <= 32,
    ensures
        r as int == crate::model::stride_of(width as int, bits as int),
        0 <= r <= 32768,
{
    let w = width as u64;
    let d = bits as u64;
    proof {
        crate::model::lemma_stride_bound(width as int, bits as int);
        assert(w == width);
        assert(d == bits);
        assert(w * d <= 262144) by (nonlinear_arith)
            requires
                w <= 8192u64,
                d <= 32u64,
        ;
        assert((w * d + 31) / 32 <= 8192) by (nonlinear_arith)
            requires
                w * d <= 262144,
        ;
        assert((w * d + 31) / 32 * 4 <= 32768) by (nonlinear_arith)
            requires
                (w * d + 31) / 32 <= 8192,
        ;
    }
    (((w * d + 31) / 32) * 4) as usize
}

/// R6: a component mask `m != 0` is interpreted as
/// `scale(component, m) = floor((255*component + floor(max/2)) / max)` where
/// `shift = trailing_zeros(m)`, `max = m >> shift`, `component = (word & m) >> shift`.
/// A zero A mask means opaque (255).
fn channel(word: u32, mask: u32) -> (r: u8)
    ensures
        r as int == crate::model::channel_of(word, mask),
{
    broadcast use vstd::std_specs::bits::axiom_u32_trailing_zeros;

    proof {
        reveal(crate::model::channel_of);
        reveal(crate::model::scale_of);
    }
    if mask == 0 {
        return 255;
    }
    let shift = mask.trailing_zeros();
    let max = mask >> shift;
    let component = (word & mask) >> shift;
    proof {
        use vstd::std_specs::bits::u32_trailing_zeros;
        assert(shift == u32_trailing_zeros(mask));
        assert(shift < 32u32);
        assert((mask >> shift) & 1u32 == 1u32);
        assert((max & 1u32) == 1u32);
        assert(max != 0u32) by (bit_vector)
            requires
                (max & 1u32) == 1u32,
        ;
        assert((word & mask) <= mask) by (bit_vector);
        assert(component <= max) by (bit_vector)
            requires
                (word & mask) <= mask,
                component == (word & mask) >> shift,
                max == mask >> shift,
        ;
    }
    let num: u64 = 255u64 * (component as u64) + (max as u64) / 2;
    let den: u64 = max as u64;
    proof {
        assert((max as u64) / 2u64 == (max as int) / 2) by (bit_vector)
            requires
                (max as int) >= 0,
        ;
        assert(num as int == 255 * (component as int) + (max as int) / 2) by (nonlinear_arith)
            requires
                num == 255u64 * (component as u64) + (max as u64) / 2u64,
                (component as int) >= 0,
                (max as int) >= 0,
                (max as u64) / 2u64 == (max as int) / 2,
        ;
        assert(den != 0u64) by (bit_vector)
            requires
                max != 0u32,
                den == max as u64,
        ;
    }
    let val: u64 = num / den;
    proof {
        assert((val as int) == (num as int) / (den as int)) by (nonlinear_arith)
            requires
                den != 0u64,
                val == num / den,
                (num as int) >= 0,
        ;
        assert(val as int == crate::model::scale_of(component as int, max as int));
        assert(val <= 255u64) by (nonlinear_arith)
            requires
                component <= max,
                max >= 1,
                val as int == (255 * (component as int) + (max as int) / 2) / (max as int),
        ;
    }
    val as u8
}

/// R6: R,G,B masks must be nonzero, every nonzero mask a single contiguous
/// set-bit run wholly inside the `bits` low bits, and all four masks pairwise
/// disjoint. Unrolled over the four masks; the `used` accumulator is exactly
/// the model's pairwise-disjoint conjunction.
fn masks_valid(m0: u32, m1: u32, m2: u32, m3: u32, bits: u32) -> (r: bool)
    ensures
        r == crate::model::masks_ok(m0, m1, m2, m3, bits as int),
{
    proof {
        reveal(crate::model::masks_ok);
        reveal(crate::model::within_bits);
        reveal(crate::model::contiguous);
        reveal(crate::model::low_of);
    }
    if m0 == 0 || m1 == 0 || m2 == 0 {
        return false;
    }
    if !single_run(m0) || !single_run(m1) || !single_run(m2) {
        return false;
    }
    if m3 != 0 && !single_run(m3) {
        return false;
    }
    if !inside(m0, bits) || !inside(m1, bits) || !inside(m2, bits) || !inside(m3, bits) {
        return false;
    }
    if (m0 & m1) != 0 || (m0 & m2) != 0 || (m0 & m3) != 0 || (m1 & m2) != 0 || (m1 & m3) != 0
        || (m2 & m3) != 0 {
        return false;
    }
    true
}

/// R6: `m != 0` and its lowest set bit shifted down yields a single run.
fn single_run(m: u32) -> (r: bool)
    requires
        m != 0u32,
    ensures
        r == crate::model::contiguous(m),
{
    proof {
        reveal(crate::model::contiguous);
        reveal(crate::model::low_of);
    }
    let low = m & 0u32.wrapping_sub(m);
    proof {
        assert(low == crate::model::low_of(m));
        assert(low != 0u32) by (bit_vector)
            requires
                m != 0u32,
                low == m & 0u32.wrapping_sub(m),
        ;
    }
    let compact = m / low;
    proof {
        assert(compact == m / low);
    }
    (compact & compact.wrapping_add(1)) == 0
}

/// R6: `m` has no bit at or above position `bits`.
fn inside(m: u32, bits: u32) -> (r: bool)
    ensures
        r == crate::model::within_bits(m, bits as int),
{
    proof {
        reveal(crate::model::within_bits);
    }
    if bits >= 32 {
        true
    } else {
        (m >> bits) == 0
    }
}

/// R7: `colour(ix)` = palette entry `ix` as `(R,G,B,255)`; `ix >= count` is
/// `PaletteIndexOutOfRange`. The caller proves the palette slice ends inside `b`.
fn colour(
    b: &[u8],
    palette_start: usize,
    pal_stride: usize,
    count: u32,
    ix: u32,
) -> (r: Result<[u8; 4], Error>)
    requires
        palette_end_valid(b, palette_start, pal_stride, count),
    ensures
        match r {
            Ok(c) => c@ == crate::model::pal_entry(b@, palette_start as int, pal_stride as int, ix
                as int) && ix < count,
            Err(e) => e == Error::PaletteIndexOutOfRange && ix >= count,
        },
{
    if ix >= count {
        return Err(Error::PaletteIndexOutOfRange);
    }
    proof {
        assert((ix as int) * (pal_stride as int) + (pal_stride as int) <= (count as int) * (pal_stride
            as int)) by (nonlinear_arith)
            requires
                (ix as int) < (count as int),
                pal_stride >= 3,
        ;
        assert(b@.len() == b.len());
        assert(b@.len() <= usize::MAX);
        assert((ix as int) * (pal_stride as int) + (pal_stride as int) <= (count as int) * (
        pal_stride as int)) by (nonlinear_arith)
            requires
                (ix as int) < (count as int),
                pal_stride >= 3,
        ;
        assert((palette_start as int) + (ix as int) * (pal_stride as int) + 3 <= b@.len())
            by (nonlinear_arith)
            requires
                (palette_start as int) + (count as int) * (pal_stride as int) <= b@.len(),
                (ix as int) * (pal_stride as int) + (pal_stride as int) <= (count as int) * (
                pal_stride as int),
                pal_stride >= 3,
        ;
        assert((palette_start as int) + (ix as int) * (pal_stride as int) + 3 <= usize::MAX)
            by (nonlinear_arith)
            requires
                (palette_start as int) + (ix as int) * (pal_stride as int) + 3 <= b@.len(),
                b@.len() <= usize::MAX,
        ;
        assert((palette_start as int) + (ix as int) * (pal_stride as int) + 3 == (palette_start
            + (ix as usize) * pal_stride + 3) as int) by {
            assert(((ix as usize) * pal_stride) as int == (ix as int) * (pal_stride as int))
                by (nonlinear_arith)
                requires
                    (ix as int) >= 0,
                    (pal_stride as int) >= 0,
                    ((ix as usize) * pal_stride) as int == (ix as int) * (pal_stride as int),
            ;
        }
    }
    let p = palette_start + (ix as usize) * pal_stride;
    proof {
        assert(p + 3 <= b@.len());
        assert(p + 2 < b@.len());
        assert(p + 1 < b@.len());
        assert(p < b@.len());
        reveal(crate::model::pal_entry);
    }
    let c = [rd(b, p + 2), rd(b, p + 1), rd(b, p), 255];
    proof {
        let e = crate::model::pal_entry(b@, palette_start as int, pal_stride as int, ix as int);
        assert(e == crate::model::pal_entry(b@, palette_start as int, pal_stride as int, ix as int));
        assert(e[0] == b@[p as int + 2]);
        assert(e[1] == b@[p as int + 1]);
        assert(e[2] == b@[p as int]);
        assert(e[3] == 255u8);
        assert(c@ == e) by {
            assert(c@.len() == 4);
            assert(c@[0] == e[0]);
            assert(c@[1] == e[1]);
            assert(c@[2] == e[2]);
            assert(c@[3] == e[3]);
            vstd::seq::lemma_seq_ext_equal(c@, e);
        }
    }
    Ok(c)
}

/// The palette-end side condition `colour` needs: with `pal_stride >= 3`, the
/// indexed entry `ix < count` ends inside `b`.
spec fn palette_end_valid(
    b: &[u8],
    palette_start: usize,
    pal_stride: usize,
    count: u32,
) -> bool {
    &&& (pal_stride == 3 || pal_stride == 4)
    &&& (palette_start as int) + (count as int) * (pal_stride as int) <= b@.len()
}

/// Write four RGBA bytes at `o`; the caller proves `o + 4 <= out.len()`. The
/// frame condition is the byte-level `replay_one`, which is what lets the ghost
/// write accumulator track the concrete buffer.
#[inline]
fn put(out: &mut Vec<u8>, o: usize, c: [u8; 4])
    requires
        o + 4 <= old(out)@.len(),
    ensures
        final(out)@.len() == old(out)@.len(),
        final(out)@[o as int] == c@[0],
        final(out)@[o as int + 1] == c@[1],
        final(out)@[o as int + 2] == c@[2],
        final(out)@[o as int + 3] == c@[3],
        final(out)@ == crate::model::replay_bytes_at(old(out)@, o as int, c@),
{
    out.set(o, c[0]);
    out.set(o + 1, c[1]);
    out.set(o + 2, c[2]);
    out.set(o + 3, c[3]);
    proof {
        reveal(crate::model::replay_bytes_at);
        let old = old(out)@;
        let b1 = old.update(o as int, c@[0]);
        let b2 = b1.update(o as int + 1, c@[1]);
        let b3 = b2.update(o as int + 2, c@[2]);
        let b4 = b3.update(o as int + 3, c@[3]);
        assert(final(out)@ == b4) by {
            assert forall|i: int| 0 <= i < old.len() implies final(out)@[i] == #[trigger] b4[i] by {
                if i == o as int {
                    vstd::seq::lemma_seq_update_same(old, o as int, c@[0]);
                    vstd::seq::lemma_seq_update_different(b1, i, o as int + 1, c@[1]);
                    vstd::seq::lemma_seq_update_different(b2, i, o as int + 2, c@[2]);
                    vstd::seq::lemma_seq_update_different(b3, i, o as int + 3, c@[3]);
                } else if i == o as int + 1 {
                    vstd::seq::lemma_seq_update_different(b1, i, o as int, c@[0]);
                    vstd::seq::lemma_seq_update_same(b1, o as int + 1, c@[1]);
                    vstd::seq::lemma_seq_update_different(b2, i, o as int + 2, c@[2]);
                    vstd::seq::lemma_seq_update_different(b3, i, o as int + 3, c@[3]);
                } else if i == o as int + 2 {
                    vstd::seq::lemma_seq_update_different(b1, i, o as int, c@[0]);
                    vstd::seq::lemma_seq_update_different(b2, i, o as int + 1, c@[1]);
                    vstd::seq::lemma_seq_update_same(b2, o as int + 2, c@[2]);
                    vstd::seq::lemma_seq_update_different(b3, i, o as int + 3, c@[3]);
                } else if i == o as int + 3 {
                    vstd::seq::lemma_seq_update_different(b1, i, o as int, c@[0]);
                    vstd::seq::lemma_seq_update_different(b2, i, o as int + 1, c@[1]);
                    vstd::seq::lemma_seq_update_different(b3, i, o as int + 2, c@[2]);
                    vstd::seq::lemma_seq_update_same(b3, o as int + 3, c@[3]);
                } else {
                }
            }
        }
    }
}

/// Overwrite one alpha byte; the caller proves `o < out.len()`.
#[inline]
fn put_alpha(out: &mut Vec<u8>, o: usize, v: u8)
    requires
        o < old(out)@.len(),
    ensures
        final(out)@.len() == old(out)@.len(),
        final(out)@ == old(out)@.update(o as int, v),
{
    out.set(o, v);
}


// ---------------------------------------------------------------------------
// The parsed header record and its model correspondence (T2).
// ---------------------------------------------------------------------------

/// The R1..R7 header fields the decoder carries, at the exact offsets the spec
/// names, with every derived quantity computed in `u64`/`usize` after the guard
/// that makes the narrowing safe.
struct Header {
    mode: Mode,
    /// R1: the DIB start (`14` for `BmpFile`, else `0`).
    s: usize,
    /// R2: the DIB header size `H`.
    hsize: usize,
    width: u32,
    height: u32,
    stored_height: i32,
    planes: u32,
    bits: u32,
    compression: u32,
    declared: u32,
    used: u32,
    /// R7: `count`, the indexed palette size (`0` if not indexed).
    count: u32,
    /// R7: `table_count`.
    table_count: u32,
    /// R7: the palette entry stride (3 or 4).
    pal_stride: usize,
    palette_start: usize,
    palette_end: usize,
    pixel_start: usize,
    /// R6: the four masks in R,G,B,A order.
    masks: [u32; 4],
    is_bmp: bool,
    core: bool,
    top_down: bool,
    /// R5: `N = width*height`.
    n: usize,
}

impl Header {
    /// R1: `s` as the model's integer.
    spec fn spec_s(self) -> int {
        self.s as int
    }

    spec fn spec_hsize(self) -> int {
        self.hsize as int
    }

    spec fn spec_width(self) -> int {
        self.width as int
    }

    spec fn spec_height(self) -> int {
        self.height as int
    }

    spec fn spec_stored(self) -> int {
        self.stored_height as int
    }

    spec fn spec_planes(self) -> int {
        self.planes as int
    }

    spec fn spec_depth(self) -> int {
        self.bits as int
    }

    spec fn spec_compression(self) -> int {
        self.compression as int
    }

    spec fn spec_declared(self) -> int {
        self.declared as int
    }

    spec fn spec_used(self) -> int {
        self.used as int
    }

    spec fn spec_count(self) -> int {
        self.count as int
    }

    spec fn spec_table_count(self) -> int {
        self.table_count as int
    }

    spec fn spec_pal_stride(self) -> int {
        self.pal_stride as int
    }

    spec fn spec_palette_start(self) -> int {
        self.palette_start as int
    }

    spec fn spec_palette_end(self) -> int {
        self.palette_end as int
    }

    spec fn spec_pixel_start(self) -> int {
        self.pixel_start as int
    }

    spec fn spec_mask_r(self) -> int {
        self.masks[0] as int
    }

    spec fn spec_mask_g(self) -> int {
        self.masks[1] as int
    }

    spec fn spec_mask_b(self) -> int {
        self.masks[2] as int
    }

    spec fn spec_mask_a(self) -> int {
        self.masks[3] as int
    }

    spec fn spec_n(self) -> int {
        self.n as int
    }

    /// R9: the raw row stride (the model's `stride`), or `0` for RLE.
    spec fn spec_stride(self) -> int {
        if self.compression == 0u32 || self.compression == 3u32 {
            crate::model::stride_of(self.spec_width(), self.spec_depth())
        } else {
            0
        }
    }

    /// R4: whether the compression is RLE8/RLE4.
    spec fn spec_is_rle(self) -> bool {
        self.compression == 1u32 || self.compression == 2u32
    }

    /// R12/R19: whether an ICO AND mask applies — either ICO mode.
    spec fn spec_is_ico(self) -> bool {
        is_ico(self.mode)
    }

    /// R3: top-down when the stored height is negative.
    spec fn spec_top_down(self) -> bool {
        self.stored_height < 0
    }

    /// R4: a BITFIELDS header carries only a 16- or 32-bit word.
    spec fn spec_mask_depth_ok(self) -> bool {
        crate::model::mask_depth_ok(self.spec_compression(), self.spec_depth())
    }
}

/// The success invariant `parse_header` establishes, specialised to the bounds
/// the pixel decoder needs (T2/T3 for the concrete `Header`).
spec fn header_inv_light(h: &Header) -> bool {
    &&& 1 <= h.spec_width() <= 8192
    &&& 1 <= h.spec_height() <= 8192
    &&& crate::model::depth_ok(h.spec_depth())
    &&& 0 <= h.spec_depth() <= 32
    &&& h.spec_n() == h.spec_width() * h.spec_height()
    &&& 4 * h.spec_n() <= 67108864
    &&& (h.compression == 0u32 || h.compression == 3u32) ==> 0 <= h.spec_stride() <= 32768
    &&& h.spec_top_down() == (h.stored_height < 0)
    &&& h.top_down == h.spec_top_down()
    &&& (is_ico(h.mode) ==> !h.spec_top_down())
    &&& h.spec_mask_depth_ok()
}

/// T2: the executable record reconstructs the model record `m` exactly — every
/// field, raw and derived.
spec fn hdr_matches(m: crate::model::MHRecord, h: Header) -> bool {
    &&& m.mode == h.mode
    &&& m.s == h.spec_s()
    &&& m.hsize == h.spec_hsize()
    &&& m.width == h.spec_width()
    &&& m.height == h.spec_height()
    &&& m.stored_height == h.spec_stored()
    &&& m.planes == h.spec_planes()
    &&& m.depth == h.spec_depth()
    &&& m.compression == h.spec_compression()
    &&& m.declared_image == h.spec_declared()
    &&& m.color_used == h.spec_used()
    &&& m.count == h.spec_count()
    &&& m.table_count == h.spec_table_count()
    &&& m.palette_stride == h.spec_pal_stride()
    &&& m.palette_start == h.spec_palette_start()
    &&& m.palette_end == h.spec_palette_end()
    &&& m.pixel_start == h.spec_pixel_start()
    &&& m.mask_r == h.spec_mask_r()
    &&& m.mask_g == h.spec_mask_g()
    &&& m.mask_b == h.spec_mask_b()
    &&& m.mask_a == h.spec_mask_a()
    &&& m.has_masks == (h.compression == 3u32)
    &&& m.core == h.core
    &&& m.is_bmp == h.is_bmp
    &&& m.top_down == h.top_down
    &&& m.n == h.spec_n()
    &&& m.stride == h.spec_stride()
}

/// The facts `parse_header` proves about an accepted record (T2/T3): T3's bounds
/// plus everything the pixel decoder needs to justify its indexing.
spec fn header_inv(b: Seq<u8>, h: Header) -> bool {
    &&& b.len() <= crate::model::max_input()
    &&& crate::model::mh_bounds(header_record(h))
    &&& h.palette_end <= b.len()
    &&& h.pixel_start <= b.len()
    &&& h.palette_start + (h.count as int) * (h.pal_stride as int) <= b.len()
    &&& (h.pal_stride == 3 || h.pal_stride == 4)
    &&& h.n as int == h.spec_width() * h.spec_height()
    &&& (h.compression == 0u32 || h.compression == 3u32) ==> h.spec_stride() <= 32768
    &&& (h.compression == 1u32 ==> h.spec_depth() == 8)
    &&& (h.compression == 2u32 ==> h.spec_depth() == 4)
    &&& (h.compression == 1u32 || h.compression == 2u32) ==> h.count >= 1u32
    &&& h.bits == 1u32 || h.bits == 4u32 || h.bits == 8u32 || h.bits == 16u32 || h.bits == 24u32
        || h.bits == 32u32
    &&& h.spec_mask_depth_ok()
}

/// The `MHRecord` a `Header` denotes, used only to state `mh_bounds`.
spec fn header_record(h: Header) -> crate::model::MHRecord {
    crate::model::MHRecord {
        mode: h.mode,
        s: h.spec_s(),
        hsize: h.spec_hsize(),
        width: h.spec_width(),
        height: h.spec_height(),
        stored_height: h.spec_stored(),
        planes: h.spec_planes(),
        depth: h.spec_depth(),
        compression: h.spec_compression(),
        declared_image: h.spec_declared(),
        color_used: h.spec_used(),
        count: h.spec_count(),
        table_count: h.spec_table_count(),
        palette_stride: h.spec_pal_stride(),
        palette_start: h.spec_palette_start(),
        palette_end: h.spec_palette_end(),
        pixel_start: h.spec_pixel_start(),
        mask_r: h.spec_mask_r(),
        mask_g: h.spec_mask_g(),
        mask_b: h.spec_mask_b(),
        mask_a: h.spec_mask_a(),
        has_masks: h.compression == 3u32,
        core: h.core,
        is_bmp: h.is_bmp,
        top_down: h.top_down,
        n: h.spec_n(),
        stride: h.spec_stride(),
    }
}


/// R1: the BMP file-header prefix. `Ok` carries `(s, bfOffBits)`; the
/// postcondition is the model's `r1_prefix`.
fn bmp_prefix(bytes: &[u8]) -> (r: Result<(usize, u64), Error>)
    ensures
        match crate::model::r1_prefix(bytes@) {
            Ok(p) => match r {
                Ok(q) => q.0 as int == p.0 && q.1 as int == p.1,
                Err(_) => false,
            },
            Err(e2) => match r {
                Ok(_) => false,
                Err(e) => e == e2,
            },
        },
{
    let len = bytes.len();
    if len < 14 {
        proof {
            reveal(crate::model::r1_prefix);
        }
        return Err(Error::TruncatedFileHeader);
    }
    proof {
        assert(len as int == bytes@.len());
    }
    if bytes[0] != b'B' || bytes[1] != b'M' {
        proof {
            reveal(crate::model::r1_prefix);
            assert(bytes@[0] == bytes[0]);
            assert(bytes@[1] == bytes[1]);
            assert(bytes@[0] != 66u8 || bytes@[1] != 77u8);
        }
        return Err(Error::BadMagic);
    }
    let bf_size = rd32(bytes, 2) as u64;
    let res6 = rd16(bytes, 6);
    let res8 = rd16(bytes, 8);
    let bad_size = (bf_size != 0 && bf_size > len as u64) || res6 != 0 || res8 != 0;
    if bad_size {
        proof {
            reveal(crate::model::r1_prefix);
            assert(bf_size as int == crate::model::le32(bytes@, 2));
            assert(res6 as int == crate::model::le16(bytes@, 6));
            assert(res8 as int == crate::model::le16(bytes@, 8));
            assert(len as int == bytes@.len());
            crate::model::lemma_le32_lt(bytes@, 2);
            assert((bf_size != 0 && bf_size > len as u64) == (crate::model::le32(bytes@, 2) != 0
                && crate::model::le32(bytes@, 2) > bytes@.len()));
        }
        return Err(Error::InvalidFileSize);
    }
    proof {
        assert(len as int == bytes@.len());
        assert(!(bf_size != 0 && bf_size > len as u64));
    }
    let off = rd32(bytes, 10) as u64;
    if off < 18 || off > len as u64 {
        proof {
            reveal(crate::model::r1_prefix);
            assert(off as int == crate::model::le32(bytes@, 10));
            assert(len as int == bytes@.len());
            crate::model::lemma_le32_lt(bytes@, 10);
        }
        return Err(Error::InvalidPixelOffset);
    }
    proof {
        reveal(crate::model::r1_prefix);
        assert(off as int == crate::model::le32(bytes@, 10));
        assert(len as int == bytes@.len());
        assert(off as int >= 18 && off as int <= bytes@.len());
    }
    Ok((14, off))
}
/// R1..R7: parse and validate the header prefix in R14 order. `Ok` carries the
/// record; T2's model equality and T3's bounds are the postcondition.
fn parse_header(bytes: &[u8], mode: Mode) -> (r: Result<Header, Error>)
    ensures
        match crate::model::mh(bytes@, mode) {
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
            Ok(h) => header_inv(bytes@, h) && header_inv_light(&h),
            Err(_) => true,
        },
{
    let len = bytes.len();
    if len > MAX_INPUT {
        proof {
            reveal(crate::model::mh);
        }
        return Err(Error::FileTooLarge);
    }
    proof {
        reveal(crate::model::mh);
        reveal(crate::model::max_input);
        assert(bytes@.len() <= crate::model::max_input());
    }
    let is_bmp = match mode {
        Mode::BmpFile => true,
        _ => false,
    };
    let start: usize;
    let file_offset: u64;
    match mode {
        Mode::BmpFile => {
            let p = match bmp_prefix(bytes) {
                Ok(p) => p,
                Err(e) => {
                    proof {
                        reveal(crate::model::mh);
                        reveal(crate::model::r1_prefix);
                    }
                    return Err(e);
                },
            };
            start = p.0;
            file_offset = p.1;
            proof {
                reveal(crate::model::mh);
                reveal(crate::model::r1_prefix);
                assert(crate::model::r1_prefix(bytes@) == Ok((start as int, file_offset as int)));
            }
        }
        _ => {
            proof {
                reveal(crate::model::mh);
                reveal(crate::model::r1_prefix);
            }
            start = 0;
            file_offset = 0;
        }
    }
    proof {
        reveal(crate::model::mh);
    }
    let f = match dib_fields(bytes, mode, start, file_offset, is_bmp) {
        Ok(f) => f,
        Err(e) => {
            proof {
                reveal(crate::model::mh);
                reveal(crate::model::r2r5);
                reveal(crate::model::r2r7);
                reveal(crate::model::r1_prefix);
            }
            return Err(e);
        },
    };
    proof {
        reveal(crate::model::r2r7);
        reveal(crate::model::r6r7);
        assert(bytes@.len() <= crate::model::max_input());
        assert(bytes@.len() >= f.spec_raw().pos0);
        assert(f.spec_raw().compression == 3 && !(f.spec_raw().hsize == 40 || f.spec_raw().hsize
            == 64) ==> f.spec_raw().hsize >= 52) by {
            reveal(crate::model::hsize_ok);
        }
        if is_bmp {
            assert((file_offset as int) == crate::model::le32(bytes@, 10));
            assert((file_offset as int) <= bytes@.len());
        }
    }
    match dib_finish(bytes, &f, file_offset, is_bmp) {
        Ok(h) => {
            proof {
                reveal(crate::model::mh);
                reveal(crate::model::r2r7);
                reveal(crate::model::r2r5);
            }
            Ok(h)
        }
        Err(e) => {
            proof {
                reveal(crate::model::mh);
                reveal(crate::model::r2r7);
            }
            Err(e)
        }
    }
}

/// R2..R7: the DIB header and palette prefix, with `file_offset` the BMP
/// `bfOffBits`. The postcondition is the model's `r2r7`.
/// R2..R5: the raw header fields the second phase needs, plus `N`.
struct Fields {
    mode: Mode,
    start: usize,
    size: usize,
    core: bool,
    width_i: i32,
    signed_height: i32,
    planes: u32,
    bits: u32,
    compression: u32,
    declared: u32,
    used: u32,
    height_v: u32,
    n: usize,
    is_bmp: bool,
}

impl Fields {
    spec fn spec_raw(self) -> crate::model::RawFields {
        crate::model::RawFields {
            hsize: self.size as int,
            core: self.core,
            width: self.width_i as int,
            stored: self.signed_height as int,
            planes: self.planes as int,
            depth: self.bits as int,
            compression: self.compression as int,
            declared: self.declared as int,
            used: self.used as int,
            height: self.height_v as int,
            pos0: self.start as int + self.size as int,
        }
    }

    spec fn spec_n(self) -> int {
        self.n as int
    }
}

/// The record `RawFields` a `Fields` denotes.
spec fn fields_matches(m: crate::model::RawFields, f: Fields) -> bool {
    &&& m.hsize == f.size as int
    &&& m.core == f.core
    &&& m.width == f.width_i as int
    &&& m.stored == f.signed_height as int
    &&& m.planes == f.planes as int
    &&& m.depth == f.bits as int
    &&& m.compression == f.compression as int
    &&& m.declared == f.declared as int
    &&& m.used == f.used as int
    &&& m.height == f.height_v as int
    &&& m.pos0 == f.start as int + f.size as int
}

/// R3: the visible height, `abs(stored)` normally and `abs(stored)/2` for ICO.
fn visible_height(mode: Mode, abs_h: u32) -> (r: u32)
    ensures
        r as int == crate::model::visible_height_of(mode, abs_h as int),
{
    proof {
        reveal(crate::model::visible_height_of);
    }
    match mode {
        Mode::IcoEntryDib | Mode::IcoEntryDibSlack => {
            abs_h / 2
        }
        _ => {
            abs_h
        }
    }
}

/// `abs` of an `i32` that is not `i32::MIN`, as a `u32`.
fn abs_i32(x: i32) -> (r: u32)
    requires
        x != i32::MIN,
    ensures
        r as int == crate::model::abs_int(x as int),
{
    if x < 0 {
        proof {
            reveal(crate::model::abs_int);
        }
        (-(x as i64)) as u32
    } else {
        proof {
            reveal(crate::model::abs_int);
        }
        x as u32
    }
}

/// R2..R5: parse and validate the header fields up to the output cap. The
/// postcondition is the model's `r2r5`.
fn dib_fields(
    bytes: &[u8],
    mode: Mode,
    start: usize,
    file_offset: u64,
    is_bmp: bool,
) -> (r: Result<Fields, Error>)
    requires
        is_bmp == (mode == Mode::BmpFile),
        is_bmp ==> start == 14,
        !is_bmp ==> start == 0 && file_offset == 0,
    ensures
        match crate::model::r2r5(bytes@, mode, start as int) {
            Ok(m) => match r {
                Ok(f) => f.mode == mode && f.is_bmp == (mode == Mode::BmpFile) && fields_matches(m, f)
                    && f.spec_n() == f.width_i as int * (f.height_v as int) && crate::model::hsize_ok(
                    f.size as int,
                ) && m.pos0 == f.start as int + f.size as int && bytes@.len() >= m.pos0,
                Err(_) => false,
            },
            Err(e2) => match r {
                Ok(_) => false,
                Err(e) => e == e2,
            },
        },
{
    let len = bytes.len();

    // -----------------------------------------------------------------------
    // R2: DIB header size and layout.
    // -----------------------------------------------------------------------
    if len < start + 4 {
        proof {
            reveal(crate::model::r2r7);
        }
        return Err(Error::TruncatedDibHeader);
    }
    let size = rd32(bytes, start);
    proof {
        assert(size as int == crate::model::le32(bytes@, start as int));
    }
    let core = match size {
        12 => true,
        16 | 40 | 52 | 56 | 64 | 108 | 124 => false,
        _ => {
            proof {
                reveal(crate::model::mh);
                reveal(crate::model::hsize_ok);
                assert(!crate::model::hsize_ok(size as int));
            }
            return Err(Error::UnsupportedHeaderSize);
        },
    };
    proof {
        reveal(crate::model::hsize_ok);
        assert(crate::model::hsize_ok(size as int));
    }
    if len < start + size as usize {
        proof {
            reveal(crate::model::mh);
        }
        return Err(Error::TruncatedHeader);
    }
    proof {
        assert(bytes@.len() >= start + size as int);
    }

    // Fields; compression/declared/used are zero for the layouts that imply them.
    let width_i: i32;
    let signed_height: i32;
    let planes: u32;
    let bits: u32;
    let compression: u32;
    let declared_image: u32;
    let used: u32;
    if size == 12 {
        // R2: the OS/2 CORE layout uses unsigned LE16 width/height.
        proof {
            assert(bytes@.len() >= start + 12);
        }
        width_i = rd16(bytes, start + 4) as i32;
        signed_height = rd16(bytes, start + 6) as i32;
        planes = rd16(bytes, start + 8);
        bits = rd16(bytes, start + 10);
        compression = 0u32;
        declared_image = 0u32;
        used = 0u32;
    } else if size == 16 {
        proof {
            assert(bytes@.len() >= start + 16);
        }
        width_i = rd32s(bytes, start + 4);
        signed_height = rd32s(bytes, start + 8);
        planes = rd16(bytes, start + 12);
        bits = rd16(bytes, start + 14);
        compression = 0u32;
        declared_image = 0u32;
        used = 0u32;
    } else {
        proof {
            assert(bytes@.len() >= start + size as int);
            assert(size >= 40);
        }
        width_i = rd32s(bytes, start + 4);
        signed_height = rd32s(bytes, start + 8);
        planes = rd16(bytes, start + 12);
        bits = rd16(bytes, start + 14);
        compression = rd32(bytes, start + 16);
        declared_image = rd32(bytes, start + 20);
        used = rd32(bytes, start + 32);
    }

    proof {
        // Field/model correspondences used by every later guard.
        if size == 12 {
            assert(width_i as int == crate::model::le16(bytes@, start as int + 4));
            assert(signed_height as int == crate::model::le16(bytes@, start as int + 6));
            assert(planes as int == crate::model::le16(bytes@, start as int + 8));
            assert(bits as int == crate::model::le16(bytes@, start as int + 10));
            assert(core);
        } else {
            assert(width_i as int == crate::model::s32(bytes@, start as int + 4));
            assert(signed_height as int == crate::model::s32(bytes@, start as int + 8));
            assert(planes as int == crate::model::le16(bytes@, start as int + 12));
            assert(bits as int == crate::model::le16(bytes@, start as int + 14));
            if size != 16 {
                assert(compression as int == crate::model::le32(bytes@, start as int + 16));
                assert(declared_image as int == crate::model::le32(bytes@, start as int + 20));
                assert(used as int == crate::model::le32(bytes@, start as int + 32));
            }
            assert(!core);
        }
    }

    // -----------------------------------------------------------------------
    // R3: planes, dimensions, ICO stored-height rule, dimension caps.
    // -----------------------------------------------------------------------
    if planes != 1 {
        proof {
            reveal(crate::model::mh);
        }
        return Err(Error::InvalidPlanes);
    }
    if width_i <= 0 || signed_height == 0 || signed_height == i32::MIN {
        proof {
            reveal(crate::model::mh);
            reveal(crate::model::s32_min);
            assert(signed_height as int == 0 || signed_height as int == crate::model::s32_min()
                || width_i <= 0);
        }
        return Err(Error::InvalidDimensions);
    }
    match mode {
        Mode::IcoEntryDib | Mode::IcoEntryDibSlack => {
            if signed_height < 0 || signed_height % 2 != 0 {
                proof {
                    reveal(crate::model::mh);
                }
                return Err(Error::InvalidDimensions);
            }
        }
        _ => {},
    }
    let abs_h = abs_i32(signed_height);
    let height_v = visible_height(mode, abs_h);
    proof {
        reveal(crate::model::abs_int);
        reveal(crate::model::visible_height_of);
        assert(abs_h as int == crate::model::abs_int(signed_height as int));
        assert(height_v as int == crate::model::visible_height_of(mode, abs_h as int));
        assert(abs_h >= 1) by (nonlinear_arith)
            requires
                abs_h as int == crate::model::abs_int(signed_height as int),
                signed_height != 0,
                signed_height != i32::MIN,
        ;
        match mode {
            Mode::IcoEntryDib | Mode::IcoEntryDibSlack => {
                assert((abs_h as int) % 2 == 0) by {
                    assert(signed_height % 2 == 0) by (bit_vector)
                        requires
                            abs_h as int == crate::model::abs_int(signed_height as int),
                            signed_height as int % 2 == 0,
                            signed_height >= 0,
                    ;
                }
                assert(crate::model::visible_height_of(mode, abs_h as int) == abs_h as int / 2);
                assert(height_v as int == abs_h as int / 2);
            }
            _ => {
                assert(crate::model::visible_height_of(mode, abs_h as int) == abs_h as int);
                assert(height_v as int == abs_h as int);
            }
        }
        match mode {
            Mode::IcoEntryDib | Mode::IcoEntryDibSlack => {
                assert(abs_h as int >= 2) by (nonlinear_arith)
                    requires
                        abs_h as int % 2 == 0,
                        abs_h as int >= 1,
                ;
                assert(height_v >= 1) by (nonlinear_arith)
                    requires
                        height_v as int == abs_h as int / 2,
                        abs_h as int >= 2,
                ;
            }
            _ => {
                assert(height_v >= 1) by (nonlinear_arith)
                    requires
                        height_v as int == abs_h as int,
                        abs_h as int >= 1,
                ;
            }
        }
    }
    if width_i as u32 > MAX_DIMENSION || height_v as u32 > MAX_DIMENSION {
        proof {
            reveal(crate::model::mh);
            assert(width_i as int > crate::model::max_dimension() || height_v as int
                > crate::model::max_dimension());
        }
        return Err(Error::InvalidDimensions);
    }
    proof {
        assert(width_i >= 1);
        assert(width_i <= 8192) by (nonlinear_arith)
            requires
                width_i as u32 <= 8192u32,
                width_i >= 1,
        ;
        assert(height_v >= 1);
        assert(height_v <= 8192) by (nonlinear_arith)
            requires
                height_v as u32 <= 8192u32,
                height_v >= 1,
        ;
        assert(width_i as u64 <= 8192u64) by (nonlinear_arith)
            requires
                width_i <= 8192,
                width_i >= 1,
        ;
        assert(height_v as u64 <= 8192u64) by (nonlinear_arith)
            requires
                height_v <= 8192,
                height_v >= 1,
        ;
        assert((width_i as u64) * (height_v as u64) <= 67108864u64) by (nonlinear_arith)
            requires
                width_i as u64 <= 8192u64,
                height_v as u64 <= 8192u64,
        ;
    }

    // -----------------------------------------------------------------------
    // R4: depth, compression and the depth/compression pairing.
    // -----------------------------------------------------------------------
    proof {
        reveal(crate::model::depth_ok);
    }
    if bits != 1 && bits != 4 && bits != 8 && bits != 16 && bits != 24 && bits != 32 {
        proof {
            reveal(crate::model::mh);
            reveal(crate::model::depth_ok);
        }
        return Err(Error::UnsupportedPixelDepth);
    }
    if compression > 3 {
        proof {
            reveal(crate::model::mh);
        }
        return Err(Error::UnsupportedCompression);
    }
    let rle = compression == 1 || compression == 2;
    if (compression == 1 && bits != 8) || (compression == 2 && bits != 4) || (compression == 3
        && bits != 16 && bits != 32) {
        proof {
            reveal(crate::model::mh);
        }
        return Err(Error::InvalidCompressionDepth);
    }
    let ico = match mode {
        Mode::IcoEntryDib | Mode::IcoEntryDibSlack => true,
        _ => false,
    };
    if (signed_height < 0 && rle) || (ico && rle) {
        proof {
            reveal(crate::model::mh);
        }
        return Err(Error::UnsupportedOrientation);
    }

    // -----------------------------------------------------------------------
    // R5: colour-count rule, then `N = width*height` and the output cap.
    // -----------------------------------------------------------------------
    if bits <= 8 {
        let limit: u32 = 1u32 << bits;
        proof {
            reveal(crate::model::pow2);
            crate::model::lemma_pow2_pos(bits as int);
            if bits == 1u32 {
                assert(crate::model::pow2(bits as int) == 2);
            } else if bits == 4u32 {
                assert(crate::model::pow2(bits as int) == 16);
            } else {
                assert(bits == 8u32);
                assert(crate::model::pow2(bits as int) == 256);
            }
            assert(limit as int == crate::model::pow2(bits as int)) by (bit_vector)
                requires
                    bits == 1u32 || bits == 4u32 || bits == 8u32,
                    limit == 1u32 << bits,
            ;
        }
        if used > limit {
            proof {
                reveal(crate::model::mh);
            }
            return Err(Error::InvalidColorCount);
        }
    } else {
        let bmp = match mode {
            Mode::BmpFile => true,
            _ => false,
        };
        if used != 0 && !bmp {
            proof {
                reveal(crate::model::mh);
                assert(mode != Mode::BmpFile);
            }
            return Err(Error::InvalidColorCount);
        }
    }

    proof {
        assert(width_i >= 1);
        assert(height_v >= 1);
        assert(width_i as int >= 0);
        assert(height_v as int >= 0);
        assert(width_i as u64 == width_i as int) by (nonlinear_arith)
            requires
                width_i >= 1,
        ;
        assert(height_v as u64 == height_v as int) by (nonlinear_arith)
            requires
                height_v >= 1,
        ;
    }
    let n64 = width_i as u64 * height_v as u64;
    proof {
        assert(n64 as int == width_i as int * (height_v as int));
        assert(width_i as int >= 1);
        assert(height_v as int >= 1) by (nonlinear_arith)
            requires
                height_v as int == crate::model::abs_int(signed_height as int) / 2
                    || height_v as int == crate::model::abs_int(signed_height as int),
                height_v as u32 >= 1,
        ;
        assert(n64 as int <= 67108864) by (nonlinear_arith)
            requires
                n64 == width_i as int * (height_v as int),
                width_i as int <= 8192,
                height_v as int <= 8192,
        ;
    }
    if 4 * n64 > MAX_OUTPUT as u64 {
        proof {
            reveal(crate::model::mh);
            assert(4 * n64 as int == 4 * (width_i as int * (height_v as int)));
        }
        return Err(Error::OutputTooLarge);
    }
    proof {
        assert(4 * n64 <= 67108864);
        assert(n64 as int <= 16777216) by (nonlinear_arith)
            requires
                n64 == width_i as int * (height_v as int),
                width_i as int >= 1,
                height_v as int >= 1,
                4 * n64 <= 67108864,
        ;
        assert(n64 <= usize::MAX as u64) by (nonlinear_arith)
            requires
                n64 <= 16777216,
        ;
    }
    let n = n64 as usize;
    proof {
        assert(n as int == n64 as int);
    }

    // -----------------------------------------------------------------------

    Ok(
        Fields {
            mode,
            start,
            size: size as usize,
            core,
            width_i,
            signed_height,
            planes,
            bits,
            compression,
            declared: declared_image,
            used,
            height_v,
            n,
            is_bmp,
        },
    )
}

/// R6..R7: parse masks and palette geometry, then build the record. Given a
/// `Fields` that is exactly the model's `r2r5` record, the postcondition is the
/// model's `r2r7`.
fn dib_finish(
    bytes: &[u8],
    f: &Fields,
    file_offset: u64,
    is_bmp: bool,
) -> (r: Result<Header, Error>)
    requires
        f.is_bmp == is_bmp,
        f.is_bmp ==> f.start == 14,
        !f.is_bmp ==> f.start == 0 && file_offset == 0,
        is_bmp == (f.mode == Mode::BmpFile),
        crate::model::r2r5(bytes@, f.mode, f.start as int) == Ok(f.spec_raw()),
        f.spec_n() == f.width_i as int * (f.height_v as int),
        bytes@.len() <= crate::model::max_input(),
        is_bmp ==> (file_offset as int) == crate::model::le32(bytes@, 10),
        is_bmp ==> (file_offset as int) <= bytes@.len(),
        bytes@.len() >= f.spec_raw().pos0,
        f.spec_raw().compression == 3 && !(f.spec_raw().hsize == 40 || f.spec_raw().hsize == 64)
            ==> f.spec_raw().hsize >= 52,
    ensures
        match crate::model::r6r7(bytes@, f.mode, f.start as int, file_offset as int, is_bmp, f.spec_raw()) {
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
            Ok(h) => header_inv(bytes@, h) && header_inv_light(&h),
            Err(_) => true,
        },
{
    let len = bytes.len();
    let start = f.start;
    let size = f.size;
    let core = f.core;
    let width_i = f.width_i;
    let signed_height = f.signed_height;
    let planes = f.planes;
    let bits = f.bits;
    let compression = f.compression;
    let declared_image = f.declared;
    let used = f.used;
    let height_v = f.height_v;
    let n = f.n;
    let mode = f.mode;
    let mut pos = start + size as usize;
    let mut masks = [0u32; 4];
    let mut mask_fields: [u32; 4] = [0u32; 4];
    if compression == 3 {
        if size == 40 || size == 64 {
            // R6: three external masks immediately after the header.
            if len - pos < 12 {
                proof {
                    reveal(crate::model::mh);
                    assert(bytes@.len() - (start + size as int) < 12);
                }
                return Err(Error::TruncatedMasks);
            }
            proof {
                assert(pos + 12 <= len);
            }
            masks[0] = rd32(bytes, pos);
            masks[1] = rd32(bytes, pos + 4);
            masks[2] = rd32(bytes, pos + 8);
            masks[3] = 0;
            mask_fields[0] = masks[0];
            mask_fields[1] = masks[1];
            mask_fields[2] = masks[2];
            mask_fields[3] = 0;
            pos += 12;
        } else {
            // R6: masks inside the header: 52 has three (alpha zero), 56/108/124 four.
            proof {
                assert(bytes@.len() >= start + 40);
            }
            masks[0] = rd32(bytes, start + 40);
            masks[1] = rd32(bytes, start + 44);
            masks[2] = rd32(bytes, start + 48);
            let m3 = if size >= 56 {
                proof {
                    assert(bytes@.len() >= start + 56);
                }
                rd32(bytes, start + 52)
            } else {
                0
            };
            masks[3] = m3;
            mask_fields[0] = masks[0];
            mask_fields[1] = masks[1];
            mask_fields[2] = masks[2];
            mask_fields[3] = m3;
        }
        proof {
            reveal(crate::model::as_u32);
            if size == 40 || size == 64 {
                assert(mask_fields[0] as int == crate::model::le32(bytes@, start as int + size as int));
                assert(mask_fields[1] as int == crate::model::le32(bytes@, start as int + size
                    as int + 4));
                assert(mask_fields[2] as int == crate::model::le32(bytes@, start as int + size
                    as int + 8));
                assert(mask_fields[3] == 0);
            } else {
                assert(mask_fields[0] as int == crate::model::le32(bytes@, start as int + 40));
                assert(mask_fields[1] as int == crate::model::le32(bytes@, start as int + 44));
                assert(mask_fields[2] as int == crate::model::le32(bytes@, start as int + 48));
                if size >= 56 {
                    assert(mask_fields[3] as int == crate::model::le32(bytes@, start as int + 52));
                } else {
                    assert(mask_fields[3] == 0);
                }
            }
        }
        let ok = masks_valid(masks[0], masks[1], masks[2], masks[3], bits);
        proof {
            reveal(crate::model::masks_ok);
        }
        if !ok {
            proof {
                reveal(crate::model::mh);
            }
            return Err(Error::InvalidBitfields);
        }
    }

    // -----------------------------------------------------------------------
    // R7: palette geometry (computed in u64), offset check, availability.
    // -----------------------------------------------------------------------
    let count: u32 = if bits <= 8 {
        if used != 0 {
            used
        } else {
            1u32 << bits
        }
    } else {
        0
    };
    let pal_stride: usize = if core {
        3
    } else {
        4
    };
    let table_count: u32 = if bits <= 8 {
        count
    } else {
        used
    };
    proof {
        reveal(crate::model::pow2);
        if bits <= 8 {
            if used != 0 {
                assert(count == used);
                assert(count as int == used as int);
            } else {
                assert(count == 1u32 << bits);
                assert(count as int == crate::model::pow2(bits as int)) by (bit_vector)
                    requires
                        bits == 1u32 || bits == 4u32 || bits == 8u32,
                        count == 1u32 << bits,
                ;
            }
            assert(table_count == count);
        } else {
            assert(count == 0);
            assert(table_count == used);
        }
    }
    let palette_length: u64 = table_count as u64 * pal_stride as u64;
    let palette_end64: u64 = pos as u64 + palette_length;
    if is_bmp && (file_offset as u64) < palette_end64 {
        proof {
            reveal(crate::model::mh);
        }
        return Err(Error::InvalidPixelOffset);
    }
    if (len as u64) < palette_end64 {
        proof {
            reveal(crate::model::mh);
        }
        return Err(Error::TruncatedPalette);
    }
    proof {
        assert(palette_end64 <= len as u64) by (nonlinear_arith)
            requires
                (len as u64) >= palette_end64,
        ;
        assert(palette_end64 <= usize::MAX as u64) by (nonlinear_arith)
            requires
                palette_end64 <= len as u64,
                len as u64 <= usize::MAX as u64,
        ;
    }
    let palette_end = palette_end64 as usize;
    let palette_start = pos;
    let pixel_start = if is_bmp {
        file_offset as usize
    } else {
        palette_start + (table_count as usize) * pal_stride
    };
    pos = pixel_start;

    // Build the record.
    let h = Header {
        mode,
        s: start,
        hsize: size as usize,
        width: width_i as u32,
        height: height_v as u32,
        stored_height: signed_height,
        planes,
        bits,
        compression,
        declared: declared_image,
        used,
        count,
        table_count,
        pal_stride,
        palette_start,
        palette_end,
        pixel_start,
        masks: mask_fields,
        is_bmp,
        core,
        top_down: signed_height < 0,
        n,
    };
    proof {
        reveal(crate::model::mh);
        reveal(crate::model::r2r5);
        reveal(crate::model::r2r7);
        reveal(crate::model::r6r7_result);
        reveal(crate::model::pow2);
        reveal(crate::model::as_u32);
        reveal(crate::model::stride_of);
        reveal(crate::model::is_ico);
        reveal(crate::model::exact_tail);
        reveal(crate::model::visible_height_of);
        reveal(hdr_matches);
        reveal(header_inv);
        reveal(header_record);
        let rf = crate::model::RawFields {
            hsize: size as int,
            core,
            width: width_i as int,
            stored: signed_height as int,
            planes: planes as int,
            depth: bits as int,
            compression: compression as int,
            declared: declared_image as int,
            used: used as int,
            height: height_v as int,
            pos0: start as int + size as int,
        };
        assert(f.spec_raw() == rf);
        let m = crate::model::r6r7_result(bytes@, mode, start as int, file_offset as int, is_bmp, rf);
        assert(crate::model::r6r7(bytes@, mode, start as int, file_offset as int, is_bmp, rf) == Ok(m)) by {
            crate::model::lemma_r6r7_eq_result(bytes@, mode, start as int, file_offset as int, is_bmp, rf);
            assert(!(rf.compression == 3 && (rf.hsize == 40 || rf.hsize == 64) && bytes@.len() - rf.pos0
                < 12));
            assert(rf.compression != 3 || crate::model::masks_ok(
                crate::model::as_u32(if rf.compression == 3 && (rf.hsize == 40 || rf.hsize == 64) {
                    crate::model::le32(bytes@, rf.pos0)
                } else if rf.compression == 3 {
                    crate::model::le32(bytes@, start as int + 40)
                } else {
                    0
                }),
                crate::model::as_u32(if rf.compression == 3 && (rf.hsize == 40 || rf.hsize == 64) {
                    crate::model::le32(bytes@, rf.pos0 + 4)
                } else if rf.compression == 3 {
                    crate::model::le32(bytes@, start as int + 44)
                } else {
                    0
                }),
                crate::model::as_u32(if rf.compression == 3 && (rf.hsize == 40 || rf.hsize == 64) {
                    crate::model::le32(bytes@, rf.pos0 + 8)
                } else if rf.compression == 3 {
                    crate::model::le32(bytes@, start as int + 48)
                } else {
                    0
                }),
                crate::model::as_u32(if rf.compression == 3 && (rf.hsize == 40 || rf.hsize == 64) {
                    0
                } else if rf.compression == 3 && rf.hsize >= 56 {
                    crate::model::le32(bytes@, start as int + 52)
                } else {
                    0
                }),
                rf.depth,
            ));
            assert(!(is_bmp && (file_offset as int) < crate::model::palette_end_of(rf)));
            assert(!(bytes@.len() < crate::model::palette_end_of(rf)));
        }
        reveal(crate::model::r6r7_result);
        assert(m == crate::model::r6r7_result(bytes@, mode, start as int, file_offset as int, is_bmp, rf));
        assert(hdr_matches(m, h)) by {
            reveal(hdr_matches);
            reveal(crate::model::r6r7_result);
            assert(m.mode == h.mode);
            assert(m.hsize == h.spec_hsize());
            assert(m.width == h.spec_width());
            assert(m.height == h.spec_height());
            assert(m.stored_height == h.spec_stored());
            assert(m.planes == h.spec_planes());
            assert(m.depth == h.spec_depth());
            assert(m.compression == h.spec_compression());
            assert(m.declared_image == h.spec_declared());
            assert(m.color_used == h.spec_used());
            assert(m.count == h.spec_count());
            assert(m.table_count == h.spec_table_count());
            assert(m.palette_stride == h.spec_pal_stride());
            assert(m.palette_start == h.spec_palette_start());
            assert(m.palette_end == h.spec_palette_end());
            assert(m.pixel_start == h.spec_pixel_start()) by {
                reveal(crate::model::r6r7_result);
                if is_bmp {
                    assert(m.pixel_start == file_offset as int);
                    assert(h.pixel_start == file_offset as usize);
                    assert((file_offset as usize) as int == file_offset as int) by (nonlinear_arith)
                        requires
                            file_offset as int <= bytes@.len(),
                            bytes@.len() <= usize::MAX,
                    ;
                } else {
                    assert(m.pixel_start == m.palette_end);
                    assert(h.pixel_start == h.palette_end);
                }
            }
            assert(m.mask_r == h.spec_mask_r());
            assert(m.mask_g == h.spec_mask_g());
            assert(m.mask_b == h.spec_mask_b());
            assert(m.mask_a == h.spec_mask_a());
            assert(m.has_masks == (h.compression == 3u32));
            assert(m.core == h.core);
            assert(m.is_bmp == h.is_bmp);
            assert(m.top_down == h.top_down);
            assert(m.n == h.spec_n());
            assert(m.stride == h.spec_stride());
        }
        crate::model::lemma_r6r7_ok_bounds(
            bytes@,
            mode,
            start as int,
            file_offset as int,
            is_bmp,
            rf,
            m,
        );
        assert(crate::model::mh_bounds(m));
        assert(header_inv(bytes@, h)) by {
            reveal(header_inv);
            reveal(header_record);
            assert(crate::model::mh_bounds(header_record(h)));
            assert(h.palette_end <= bytes@.len());
            assert(h.pixel_start <= bytes@.len()) by {
                if is_bmp {
                    assert(h.pixel_start == file_offset as usize);
                    assert((file_offset as int) <= bytes@.len());
                } else {
                    assert(h.pixel_start == palette_start + (table_count as usize) * pal_stride);
                    assert(h.palette_end == h.pixel_start);
                    assert(h.palette_end <= bytes@.len());
                }
            }
            assert(h.palette_start + (h.count as int) * (h.pal_stride as int) <= bytes@.len());
            assert(h.pal_stride == 3 || h.pal_stride == 4);
            assert(h.n as int == h.spec_width() * h.spec_height());
            assert((h.compression == 0u32 || h.compression == 3u32) ==> h.spec_stride() <= 32768) by {
                if h.compression == 0u32 || h.compression == 3u32 {
                    crate::model::lemma_stride_bound(h.spec_width(), h.spec_depth());
                }
            }
            assert(bytes@.len() <= crate::model::max_input());
        }
        assert(header_inv_light(&h)) by {
            reveal(header_inv_light);
            reveal(crate::model::mh_bounds);
            reveal(crate::model::depth_ok);
            assert(crate::model::mh_bounds(header_record(h)));
            assert(crate::model::depth_ok(bits as int));
            assert(h.spec_n() == h.spec_width() * h.spec_height());
            assert(h.spec_top_down() == (h.stored_height < 0));
            assert(h.spec_mask_depth_ok()) by {
                reveal(crate::model::mask_depth_ok);
                if compression == 3 {
                    assert(bits == 16u32 || bits == 32u32);
                }
            }
            if compression == 0 || compression == 3 {
                crate::model::lemma_stride_bound(width_i as int, bits as int);
            }
        }
    }
    Ok(h)
}

// ---------------------------------------------------------------------------
// R9: the oriented output position.
// ---------------------------------------------------------------------------

/// R9: source row `y` maps to output row `y` (top-down) or `height-1-y`.
spec fn map_row_spec(h: &Header, y: int) -> int {
    if h.spec_top_down() {
        y
    } else {
        h.spec_height() - 1 - y
    }
}

/// R5: `1 <= width <= 8192` and `1 <= height <= 8192` give the orientation
/// bijection on `[0,N)`.
proof fn lemma_map_row_bound(h: &Header, y: int, x: int)
    requires
        header_inv_light(h),
        0 <= y < h.spec_height(),
        0 <= x < h.spec_width(),
    ensures
        0 <= map_row_spec(h, y) < h.spec_height(),
        0 <= map_row_spec(h, y) * h.spec_width() + x < h.spec_n(),
{
    reveal(map_row_spec);
    reveal(header_inv_light);
    if h.spec_top_down() {
        assert(0 <= y < h.spec_height());
    } else {
        assert(0 <= h.spec_height() - 1 - y < h.spec_height()) by (nonlinear_arith)
            requires
                0 <= y < h.spec_height(),
        ;
    }
    assert(map_row_spec(h, y) * h.spec_width() + x < h.spec_n()) by (nonlinear_arith)
        requires
            0 <= map_row_spec(h, y) < h.spec_height(),
            0 <= x < h.spec_width(),
            h.spec_n() == h.spec_width() * h.spec_height(),
            h.spec_width() >= 1,
    ;
}

/// R9: the output byte offset `4*(map_row(y)*width + x)`; the caller supplies
/// `0 <= x < width`, `0 <= y < height`.
fn out_offset(h: &Header, x: usize, y: usize) -> (r: usize)
    requires
        header_inv_light(h),
        (x as int) < h.spec_width(),
        (y as int) < h.spec_height(),
    ensures
        r as int == 4 * (map_row_spec(h, y as int) * h.spec_width() + x as int),
        r as int == 4 * crate::model::orient_cell(
            h.spec_width(),
            h.spec_height(),
            h.spec_top_down(),
            x as int,
            y as int,
        ),
        (r as int) + 4 <= 4 * h.spec_n(),
{
    proof {
        lemma_map_row_bound(h, y as int, x as int);
    }
    proof {
        lemma_map_row_bound(h, y as int, x as int);
    }
    let by = if h.stored_height < 0 {
        y
    } else {
        h.height as usize - 1 - y
    };
    proof {
        reveal(map_row_spec);
        reveal(header_inv_light);
        if h.spec_top_down() {
            assert(by == y);
        } else {
            assert((h.height as usize) as int == h.spec_height());
            assert(by as int == h.spec_height() - 1 - y as int) by (nonlinear_arith)
                requires
                    by == (h.height as usize - 1 - y),
                    (h.height as usize) as int == h.spec_height(),
                    (y as int) < h.spec_height(),
            ;
        }
        assert(by as int == map_row_spec(h, y as int));
        assert((h.width as usize) as int == h.spec_width());
        assert(by * (h.width as usize) <= usize::MAX) by (nonlinear_arith)
            requires
                (by as int) < h.spec_height(),
                h.spec_height() <= 8192,
                (h.width as usize) as int == h.spec_width(),
                h.spec_width() <= 8192,
        ;
        assert(4 * (by * (h.width as usize) + x) + 4 <= usize::MAX) by (nonlinear_arith)
            requires
                (by as int) < h.spec_height(),
                (x as int) < h.spec_width(),
                h.spec_width() <= 8192,
                h.spec_height() <= 8192,
                (h.width as usize) as int == h.spec_width(),
        ;
    }
    let r = 4 * (by * (h.width as usize) + x);
    proof {
        assert(r as int == 4 * (map_row_spec(h, y as int) * h.spec_width() + x as int)) by (nonlinear_arith)
            requires
                by as int == map_row_spec(h, y as int),
                (h.width as usize) as int == h.spec_width(),
                r == 4 * (by * (h.width as usize) + x),
        ;
        reveal(crate::model::orient_cell);
        reveal(crate::model::map_row_of);
        assert(map_row_spec(h, y as int) == crate::model::map_row_of(
            h.spec_height(),
            h.spec_top_down(),
            y as int,
        ));
        assert(r as int == 4 * crate::model::orient_cell(
            h.spec_width(),
            h.spec_height(),
            h.spec_top_down(),
            x as int,
            y as int,
        ));
        lemma_map_row_bound(h, y as int, x as int);
        assert((r as int) + 4 <= 4 * h.spec_n()) by (nonlinear_arith)
            requires
                (map_row_spec(h, y as int) * h.spec_width() + x as int) < h.spec_n(),
                r as int == 4 * (map_row_spec(h, y as int) * h.spec_width() + x as int),
        ;
    }
    r
}


// ---------------------------------------------------------------------------
// T3: the single allocation, and the proof that no other path allocates.
// ---------------------------------------------------------------------------

/// T3: the one output allocation. `header_inv_light` gives `4*N <= 67,108,864`,
/// so the request is exactly `4*N` bytes and fits every target `usize`.
fn alloc_output(h: &Header, steps: &mut Ghost<nat>) -> (r: Vec<u8>)
    requires
        header_inv_light(h),
    ensures
        r@.len() == 4 * h.spec_n(),
        r@ == crate::model::zero_bytes(4 * h.spec_n()),
        final(steps)@ == old(steps)@ + 4 * h.spec_n(),
{
    proof {
        reveal(header_inv_light);
        assert(h.spec_n() == h.spec_width() * h.spec_height());
        assert(h.spec_n() >= 1) by (nonlinear_arith)
            requires
                h.spec_width() >= 1,
                h.spec_height() >= 1,
                h.spec_n() == h.spec_width() * h.spec_height(),
        ;
        assert(4 * h.spec_n() <= 67108864);
        assert(h.n <= 16777216) by (nonlinear_arith)
            requires
                h.spec_n() == h.spec_width() * h.spec_height(),
                h.spec_width() >= 1,
                h.spec_height() >= 1,
                4 * h.spec_n() <= 67108864,
        ;
        assert(h.n == h.spec_n());
        assert(4 * h.n <= usize::MAX) by (nonlinear_arith)
            requires
                h.n <= 16777216,
        ;
    }
    let out = vec![0u8; 4 * h.n];
    // A library bulk fill touches every output byte, even though the call is one expression.
    proof { *steps = Ghost(steps@ + (4 * h.spec_n()) as nat); }
    proof {
        assert(out@.len() == 4 * h.n);
        assert(out@ == crate::model::zero_bytes(4 * h.n)) by {
            reveal(crate::model::zero_bytes);
            broadcast use vstd::seq::group_seq_lemmas;
            assert(out@ =~= crate::model::zero_bytes(4 * h.n));
        }
    }
    out
}

// The public decoder tracks the call-site count as a ghost local; the only
// executable allocation remains the vec! expression above.


// The input is immutable. Every reader on the pixel path now carries a
// precondition that the addressed byte exists (M3 closed the M2b `read_at`
// guard): a computed raw/AND/palette address is proved inside its checked row,
// mask or palette span, so the "returns 0 past the end" branch is gone.
fn raw_sample(b: &[u8], h: &Header, pos: usize, x: usize, line: usize) -> (r: Result<[u8; 4], Error>)
    requires
        header_inv(b@, *h),
        header_inv_light(h),
        line as int == crate::model::stride_of(h.spec_width(), h.spec_depth()),
        pos + line <= b@.len(),
        (x as int) < h.spec_width(),
        h.compression == 0 || h.compression == 3,
    ensures
        match r {
            Ok(c) => crate::model::raw_conv(b@, header_record(*h), pos as int, x as int) == c@
                && crate::model::raw_index_ok(b@, header_record(*h), pos as int, x as int),
            Err(e) => e == Error::PaletteIndexOutOfRange && !crate::model::raw_index_ok(
                b@,
                header_record(*h),
                pos as int,
                x as int,
            ),
        },
{
    proof {
        reveal(header_inv_light);
        reveal(crate::model::depth_ok);
    }
    let bits = h.bits;
    proof {
        assert(bits == 1u32 || bits == 4u32 || bits == 8u32 || bits == 16u32 || bits == 24u32
            || bits == 32u32);
        assert((bits as int) <= 32);
        assert((bits as usize) as int == bits as int);
        assert((x as int) * (bits as int) <= 262144) by (nonlinear_arith)
            requires (x as int) < h.spec_width(), h.spec_width() <= 8192, (bits as int) <= 32;
        assert((x as int) * (bits as int) <= usize::MAX) by (nonlinear_arith)
            requires (x as int) * (bits as int) <= 262144;
    }
    let bit_index: usize = x * (bits as usize);
    proof {
        assert(bit_index as int == (x as int) * (bits as int)) by (nonlinear_arith)
            requires
                bit_index == x * (bits as usize),
                (bits as usize) as int == bits as int,
                (x as int) * (bits as int) <= usize::MAX;
        assert((bit_index as int) <= 262144);
    }
    let off = bit_index / 8;
    proof {
        reveal(crate::model::stride_of);
        assert(off as int == (x as int) * (bits as int) / 8) by (nonlinear_arith)
            requires off == bit_index / 8, bit_index as int == (x as int) * (bits as int);
        assert((off as int) <= 32768) by (nonlinear_arith)
            requires off as int == (x as int) * (bits as int) / 8,
                (x as int) < h.spec_width(), h.spec_width() <= 8192, (bits as int) <= 32;
        assert(pos + off <= usize::MAX) by (nonlinear_arith)
            requires (off as int) <= 32768, pos <= b@.len(), b@.len() <= 67108864;
    }
    let p = pos + off;
    if bits <= 8 {
        proof {
            crate::model::lemma_subbyte_within_stride(h.spec_width(), bits as int, x as int);
            reveal(crate::model::stride_of);
            assert((off as int) + 1 <= line as int);
            assert((pos as int) + (off as int) + 1 <= b@.len()) by (nonlinear_arith)
                requires pos + line <= b@.len(), (off as int) + 1 <= line as int;
            assert((p as int) + 1 <= b@.len());
            assert(p < b@.len());
            crate::model::lemma_subbyte_shift(bits as int, x as int);
        }
        let shift = 8 - bits - (bit_index % 8) as u32;
        let mask = if bits == 1 { 1u32 } else if bits == 4 { 15u32 } else { 255u32 };
        let byte = rd(b, p);
        let ix = ((byte as u32) >> shift) & mask;
        proof {
            reveal(header_record);
            reveal(crate::model::raw_conv);
            reveal(crate::model::sub_index);
            reveal(crate::model::sub_shift);
            reveal(crate::model::sub_mask);
            assert(bits == 1u32 || bits == 4u32 || bits == 8u32);
            assert(crate::model::sub_mask(bits as int) == mask) by {
                if bits == 1 {
                    assert(crate::model::sub_mask(bits as int) == 1u32);
                } else if bits == 4 {
                    assert(crate::model::sub_mask(bits as int) == 15u32);
                } else {
                    assert(bits == 8u32);
                    assert(crate::model::sub_mask(bits as int) == 255u32);
                }
            }
            assert(bit_index as int == (x as int) * (bits as int));
            assert((bit_index % 8) as int == ((x as int) * (bits as int)) % 8);
            assert(shift as int == 8 - (bits as int) - ((x as int) * (bits as int)) % 8) by {
                assert(shift as int == 8 - (bits as int) - (bit_index % 8) as int);
            }
            assert(crate::model::sub_shift(bits as int, x as int) == shift as int);
            assert(0 <= crate::model::sub_shift(bits as int, x as int) < 32);
            assert(h.spec_depth() == bits as int);
            assert(h.spec_palette_start() == h.palette_start as int);
            assert(h.spec_pal_stride() == h.pal_stride as int);
            assert((p as int) == (pos as int) + (off as int));
            assert(off as int == (x as int) * (bits as int) / 8);
            assert(byte == b@[(pos as int) + (x as int) * (bits as int) / 8]);
            assert(crate::model::sub_index(byte, bits as int, x as int) == ix as int) by {
                assert(
                    crate::model::sub_index(byte, bits as int, x as int)
                        == ((byte as u32 >> (shift as u32)) & mask) as int);
            }
        }
        let c = colour(b, h.palette_start, h.pal_stride, h.count, ix);
        proof {
            reveal(crate::model::raw_index_ok);
            match c {
                Ok(cc) => {
                    assert(ix < h.count);
                    assert(h.spec_count() == h.count as int);
                    assert(crate::model::sub_index(
                        b@[pos as int + (x as int) * (bits as int) / 8],
                        bits as int,
                        x as int,
                    ) == ix as int);
                    assert(crate::model::raw_index_ok(
                        b@,
                        header_record(*h),
                        pos as int,
                        x as int,
                    ));
                    assert(crate::model::conv(
                        b@,
                        header_record(*h),
                        crate::model::sub_index(byte, bits as int, x as int),
                    ) == crate::model::pal_entry(
                        b@,
                        h.spec_palette_start(),
                        h.spec_pal_stride(),
                        crate::model::sub_index(byte, bits as int, x as int),
                    ));
                    assert(crate::model::raw_conv(b@, header_record(*h), pos as int, x as int)
                        == crate::model::conv(
                        b@,
                        header_record(*h),
                        crate::model::sub_index(byte, bits as int, x as int),
                    ));
                    assert(cc@ == crate::model::raw_conv(b@, header_record(*h), pos as int, x as int));
                },
                Err(_) => {},
            }
        }
        c
    } else {
        proof {
            reveal(crate::model::raw_index_ok);
            reveal(header_record);
            assert(crate::model::raw_index_ok(b@, header_record(*h), pos as int, x as int));
        }
        proof {
            assert(bits == 16u32 || bits == 24u32 || bits == 32u32);
            assert((bits as int) % 8 == 0);
            assert(off as int == (x as int) * ((bits as int) / 8)) by (nonlinear_arith)
                requires
                    off as int == (x as int) * (bits as int) / 8,
                    (bits as int) % 8 == 0,
                    (bits as int) >= 0;
            crate::model::lemma_sample_within_stride(h.spec_width(), bits as int, x as int);
            reveal(crate::model::stride_of);
            assert((off as int) + ((bits as int) / 8) <= line as int);
            assert((p as int) == (pos as int) + (off as int));
            assert((p as int) + ((bits as int) / 8) <= b@.len()) by (nonlinear_arith)
                requires
                    pos + line <= b@.len(),
                    (p as int) == (pos as int) + (off as int),
                    (off as int) + ((bits as int) / 8) <= line as int;
        }
        if h.compression == 3 {
            proof {
                reveal(header_inv);
                reveal(header_inv_light);
                reveal(crate::model::mask_depth_ok);
                reveal(header_record);
                reveal(crate::model::raw_conv);
                reveal(crate::model::conv_rgb);
                reveal(crate::model::bitfield_seq);
                assert(bits == 16u32 || bits == 32u32);
                assert((p as int) + ((bits as int) / 8) <= b@.len());
                assert(off as int == (x as int) * (bits as int) / 8);
                if bits == 16 {
                    assert((p as int) + 2 <= b@.len()) by (nonlinear_arith)
                        requires (p as int) + ((bits as int) / 8) <= b@.len(), bits == 16u32;
                    assert(p + 1 < b@.len());
                    assert(p < b@.len());
                } else {
                    assert((p as int) + 4 <= b@.len()) by (nonlinear_arith)
                        requires (p as int) + ((bits as int) / 8) <= b@.len(), bits == 32u32;
                    assert(p + 3 < b@.len());
                    assert(p + 2 < b@.len());
                    assert(p + 1 < b@.len());
                    assert(p < b@.len());
                }
            }
            if bits == 16 {
                let word = rd16(b, p);
                proof {
                    reveal(header_record);
                    reveal(crate::model::conv_rgb);
                    assert(h.spec_mask_r() == h.masks[0] as int);
                    assert(h.spec_mask_g() == h.masks[1] as int);
                    assert(h.spec_mask_b() == h.masks[2] as int);
                    assert(h.spec_mask_a() == h.masks[3] as int);
                    assert(crate::model::le16(b@, p as int) == word as int);
                    assert(crate::model::conv_rgb(b@, header_record(*h), p as int)
                        == crate::model::bitfield_seq(header_record(*h), crate::model::le16(b@, p
                        as int)));
                }
                Ok([channel(word, h.masks[0]), channel(word, h.masks[1]),
                    channel(word, h.masks[2]), channel(word, h.masks[3])])
            } else {
                let word = rd32(b, p);
                proof {
                    reveal(header_record);
                    reveal(crate::model::conv_rgb);
                    assert(h.spec_mask_r() == h.masks[0] as int);
                    assert(h.spec_mask_g() == h.masks[1] as int);
                    assert(h.spec_mask_b() == h.masks[2] as int);
                    assert(h.spec_mask_a() == h.masks[3] as int);
                    assert(crate::model::le32(b@, p as int) == word as int);
                    assert(crate::model::conv_rgb(b@, header_record(*h), p as int)
                        == crate::model::bitfield_seq(header_record(*h), crate::model::le32(b@, p
                        as int)));
                }
                Ok([channel(word, h.masks[0]), channel(word, h.masks[1]),
                    channel(word, h.masks[2]), channel(word, h.masks[3])])
            }
        } else if bits == 16 {
            proof {
                reveal(header_inv);
                reveal(header_record);
                reveal(crate::model::raw_conv);
                reveal(crate::model::conv_rgb);
                reveal(crate::model::rgb555_seq);
                assert((p as int) + 2 <= b@.len()) by (nonlinear_arith)
                    requires (p as int) + ((bits as int) / 8) <= b@.len(), bits == 16u32;
                assert(p + 1 < b@.len());
                assert(p < b@.len());
                assert(off as int == (x as int) * (bits as int) / 8);
            }
            let word = rd16(b, p);
            proof {
                assert(crate::model::le16(b@, p as int) == word as int);
                assert(crate::model::conv_rgb(b@, header_record(*h), p as int)
                    == crate::model::rgb555_seq(crate::model::le16(b@, p as int)));
                assert(crate::model::channel_of(word, 0x7c00u32)
                    == crate::model::channel_of((word as u32), 0x7c00u32));
                assert(crate::model::channel_of(word, 0x03e0u32)
                    == crate::model::channel_of((word as u32), 0x03e0u32));
                assert(crate::model::channel_of(word, 0x001fu32)
                    == crate::model::channel_of((word as u32), 0x001fu32));
            }
            Ok([channel(word, 0x7c00), channel(word, 0x03e0), channel(word, 0x001f), 255])
        } else if bits == 24 {
            proof {
                reveal(header_inv);
                reveal(header_record);
                reveal(crate::model::raw_conv);
                reveal(crate::model::conv_rgb);
                assert((p as int) + 3 <= b@.len()) by (nonlinear_arith)
                    requires (p as int) + ((bits as int) / 8) <= b@.len(), bits == 24u32;
                assert(p + 2 < b@.len());
                assert(p + 1 < b@.len());
                assert(p < b@.len());
                assert(off as int == (x as int) * (bits as int) / 8);
                assert((p as int) == (pos as int) + (off as int));
            }
            let c = [rd(b, p + 2), rd(b, p + 1), rd(b, p), 255];
            proof {
                assert(c@ == crate::model::raw_conv(b@, header_record(*h), pos as int, x as int))
                    by {
                    assert(c@ == seq![
                        b@[p as int + 2],
                        b@[p as int + 1],
                        b@[p as int],
                        255u8,
                    ]);
                    assert(crate::model::conv_rgb(b@, header_record(*h), p as int) == seq![
                        b@[p as int + 2],
                        b@[p as int + 1],
                        b@[p as int],
                        255u8,
                    ]);
                }
            }
            Ok(c)
        } else {
            proof {
                reveal(header_inv);
                reveal(header_record);
                reveal(crate::model::raw_conv);
                reveal(crate::model::conv_rgb);
                assert(bits == 32u32);
                assert((p as int) + 4 <= b@.len()) by (nonlinear_arith)
                    requires (p as int) + ((bits as int) / 8) <= b@.len(), bits == 32u32;
                assert(p + 3 < b@.len());
                assert(p + 2 < b@.len());
                assert(p + 1 < b@.len());
                assert(p < b@.len());
                assert(off as int == (x as int) * (bits as int) / 8);
                assert((p as int) == (pos as int) + (off as int));
            }
            let c = match h.mode {
                Mode::IcoEntryDib | Mode::IcoEntryDibSlack => {
                    [rd(b, p + 2), rd(b, p + 1), rd(b, p), rd(b, p + 3)]
                },
                _ => [rd(b, p + 2), rd(b, p + 1), rd(b, p), 255],
            };
            proof {
                assert(c@ == crate::model::raw_conv(b@, header_record(*h), pos as int, x as int))
                    by {
                    if is_ico(h.mode) {
                        assert(is_ico(header_record(*h).mode));
                        assert(c@ == seq![
                            b@[p as int + 2],
                            b@[p as int + 1],
                            b@[p as int],
                            b@[p as int + 3],
                        ]);
                        assert(crate::model::conv_rgb(b@, header_record(*h), p as int) == seq![
                            b@[p as int + 2],
                            b@[p as int + 1],
                            b@[p as int],
                            b@[p as int + 3],
                        ]);
                    } else {
                        assert(!is_ico(header_record(*h).mode));
                        assert(c@ == seq![
                            b@[p as int + 2],
                            b@[p as int + 1],
                            b@[p as int],
                            255u8,
                        ]);
                        assert(crate::model::conv_rgb(b@, header_record(*h), p as int) == seq![
                            b@[p as int + 2],
                            b@[p as int + 1],
                            b@[p as int],
                            255u8,
                        ]);
                    }
                }
            }
            Ok(c)
        }
    }
}

// ---------------------------------------------------------------------------
// ---------------------------------------------------------------------------
// T6 (M4a): the raw/BITFIELDS helper at the manifest's quantification, with the
// ghost final cursor/position on both exits, and the concrete pixels written so
// far tied to the model's partial output.
// ---------------------------------------------------------------------------

/// The `MHRecord` a `Header` denotes (the same record `hdr_matches` compares).
spec fn hrec(h: &Header) -> crate::model::MHRecord {
    header_record(*h)
}

/// `header_record` keeps the executable's `top_down`, which `header_inv_light`
/// pins to `stored_height < 0` — the model's own `top_down`.
proof fn lemma_hrec_top_down(h: &Header)
    requires
        header_inv_light(h),
    ensures
        hrec(h).top_down == h.spec_top_down(),
{
    reveal(hrec);
    reveal(header_record);
    reveal(header_inv_light);
}

/// `header_record` keeps the executable's `mode` field verbatim.
proof fn lemma_hrec_mode(h: &Header)
    ensures
        hrec(h).mode == h.mode,
{
    reveal(hrec);
    reveal(header_record);
}

/// `header_record` keeps every dimension the model's `cell_of` uses.
proof fn lemma_hrec_dims(h: &Header)
    requires
        header_inv_light(h),
    ensures
        hrec(h).width == h.spec_width(),
        hrec(h).height == h.spec_height(),
        hrec(h).n == h.spec_n(),
{
    reveal(hrec);
    reveal(header_record);
    reveal(header_inv_light);
}

/// The model's raw cursor/position at the point the concrete loop stops: the
/// `Ok` final `(pos, x, y)` or the `Err` ghost `(pos, x, y)`.
spec fn raw_final(
    b: &[u8],
    h: &Header,
    pos0: usize,
    x0: usize,
    y0: usize,
) -> Result<(int, int, int), (crate::model::RawErr, int, int, int)> {
    crate::model::raw_gcur(b@, hrec(h), pos0 as int, x0 as int, y0 as int)
}

/// T6: the raw transition loop from an arbitrary reachable start state
/// `(pos0, x0, y0)`. On success the returned cursor is `MR`'s final cursor; on
/// failure the exact first error is returned and the ghost state `gcur` carries
/// `MR`'s final cursor and position. The concrete buffer always holds exactly the
/// model's write list `raw_writes(...)` replayed onto the R8 zero fill — which
/// is the model's partial output through the first failing check on failure.
fn raw_loop(
    b: &[u8],
    h: &Header,
    out: &mut Vec<u8>,
    pos0: usize,
    gcur: &mut (usize, usize, usize),
    steps: &mut Ghost<nat>,
) -> (r: Result<(), Error>)
    requires
        header_inv(b@, *h), header_inv_light(h),
        crate::model::raw_reachable(b@, hrec(h), pos0 as int, 0, 0),
        old(out)@.len() == 4 * h.spec_n(),
        old(out)@ == crate::model::zero_bytes(4 * h.spec_n()),
        old(gcur).0 as int == pos0 as int,
        old(gcur).1 == 0,
        old(gcur).2 == 0,
    ensures
        final(steps)@ <= old(steps)@ + h.spec_n() + h.spec_height() + 1,
        final(out)@.len() == 4 * h.spec_n(),
        final(gcur).0 as int <= b@.len(),
        final(gcur).0 as int >= h.pixel_start as int,
        // T6, error half: on `Err(e)` the returned error is the model's exact
        // first error and the ghost state `*final(gcur)` is the model's final
        // cursor/position (through the failing check).
        match r {
            Ok(()) => crate::model::raw_gcur(b@, hrec(h), pos0 as int, 0, 0) == Ok((
                final(gcur).0 as int,
                final(gcur).1 as int,
                final(gcur).2 as int,
            )),
            Err(e) => crate::model::raw_gcur(b@, hrec(h), pos0 as int, 0, 0) == Err((
                crate::model::RawErr::Api(e),
                final(gcur).0 as int,
                final(gcur).1 as int,
                final(gcur).2 as int,
            )),
        },
        // T6, pixel half: the concrete buffer is the model's write list replayed
        // onto the zero fill (`raw_writes` is the model's partial output through
        // the first failing check on failure, and its full output on success).
        final(out)@ == crate::model::bytes_replay(
            crate::model::zero_bytes(4 * h.spec_n()),
            crate::model::raw_writes(b@, hrec(h), pos0 as int, 0, 0),
        ),
{
    let line = stride(h.width, h.bits);
    let mut pos = pos0;
    let mut x: usize = 0;
    let mut y: usize = 0;
    let ghost s0: nat = steps@;
    let ghost mut gw: Seq<crate::model::Write> = Seq::empty();
    proof {
        reveal(crate::model::raw_reachable);
        reveal(crate::model::raw_writes);
        reveal(crate::model::raw_gcur);
        reveal(crate::model::raw_full);
        reveal(crate::model::bytes_replay);
        assert(line as int == h.spec_stride());
        assert(h.n == h.spec_n());
        assert(crate::model::raw_writes(b@, hrec(h), pos as int, 0, 0) == gw
            + crate::model::raw_writes(b@, hrec(h), pos as int, x as int, y as int));
        assert(out@ == crate::model::bytes_replay(crate::model::zero_bytes(4 * h.spec_n()), gw));
    }
    loop
        invariant
            header_inv(b@, *h), header_inv_light(h),
            out@.len() == 4 * h.spec_n(),
            h.compression == 0 || h.compression == 3,
            0 <= x <= h.spec_width(), 0 <= y <= h.spec_height(),
            pos as int <= b@.len(), pos as int >= h.pixel_start as int,
            line as int == h.spec_stride(),
            s0 == old(steps)@,
            steps@ == s0 + (y as nat) * (h.spec_width() + 1) + (x as nat),
            x > 0 ==> (pos as int) + (line as int) <= b@.len(),
            y as int >= h.spec_height() ==> x == 0,
            gcur.0 as int == pos as int, gcur.1 as int == x as int, gcur.2 as int == y as int,
            // Link the running state back to the start: the model's recursion is
            // followed exactly, so both `raw_gcur` and `raw_writes` stay equal.
            crate::model::raw_gcur(b@, hrec(h), pos0 as int, 0, 0) == crate::model::raw_gcur(
                b@,
                hrec(h),
                pos as int,
                x as int,
                y as int,
            ),
            crate::model::raw_writes(b@, hrec(h), pos0 as int, 0, 0) == gw
                + crate::model::raw_writes(b@, hrec(h), pos as int, x as int, y as int),
            out@ == crate::model::bytes_replay(crate::model::zero_bytes(4 * h.spec_n()), gw),
        decreases (h.spec_height() - y as int) as nat, (h.spec_width() - x as int) as nat,
    {
        proof {
            assert((y as int) * (h.spec_width() + 1) + (x as int)
                <= h.spec_n() + h.spec_height()) by (nonlinear_arith)
                requires
                    0 <= y <= h.spec_height(), 0 <= x <= h.spec_width(),
                    y == h.spec_height() ==> x == 0,
                    h.spec_n() == h.spec_width() * h.spec_height(),
                    h.spec_width() >= 1;
            *steps = Ghost(steps@ + 1);
        }
        if y >= h.height as usize {
            proof {
                reveal(crate::model::raw_full);
                reveal(crate::model::raw_gcur);
                reveal(crate::model::raw_writes);
                assert(crate::model::raw_full(b@, hrec(h), pos as int, x as int, y as int) == Ok((
                    pos as int,
                    x as int,
                    y as int,
                    Seq::<crate::model::Write>::empty(),
                )));
                assert(crate::model::raw_writes(b@, hrec(h), pos as int, x as int, y as int)
                    == Seq::<crate::model::Write>::empty());
                assert(crate::model::raw_writes(b@, hrec(h), pos0 as int, 0, 0) == gw) by {
                    broadcast use vstd::seq_lib::group_seq_lib_default;
                }
                assert(crate::model::raw_writes(b@, hrec(h), pos0 as int, 0, 0) == gw);
                assert(out@ == crate::model::bytes_replay(
                    crate::model::zero_bytes(4 * h.spec_n()),
                    crate::model::raw_writes(b@, hrec(h), pos0 as int, 0, 0),
                ));
            }
            return Ok(());
        }
        if x >= h.width as usize {
            proof {
                reveal(crate::model::raw_full);
                reveal(crate::model::raw_gcur);
                reveal(crate::model::raw_writes);
                assert(crate::model::raw_gcur(b@, hrec(h), pos as int, x as int, y as int)
                    == crate::model::raw_gcur(b@, hrec(h), (pos as int) + (line as int), 0, (y
                    as int) + 1));
                assert(crate::model::raw_writes(b@, hrec(h), pos as int, x as int, y as int)
                    == crate::model::raw_writes(
                    b@,
                    hrec(h),
                    (pos as int) + (line as int),
                    0,
                    (y as int) + 1,
                ));
            }
            pos = pos + line;
            x = 0;
            y = y + 1;
            gcur.0 = pos;
            gcur.1 = x;
            gcur.2 = y;
            proof {
                assert((y as nat) * (h.spec_width() + 1)
                    == ((y - 1) as nat) * (h.spec_width() + 1) + h.spec_width() + 1)
                    by (nonlinear_arith)
                    requires y >= 1, h.spec_width() >= 1;
            }
            continue;
        }
        if b.len() - pos < line {
            proof {
                reveal(crate::model::raw_full);
                reveal(crate::model::raw_gcur);
                reveal(crate::model::raw_writes);
                assert(x == 0);
                assert(b@.len() - (pos as int) < line as int);
                assert(crate::model::raw_writes(b@, hrec(h), pos as int, x as int, y as int)
                    == Seq::<crate::model::Write>::empty());
                assert(crate::model::raw_writes(b@, hrec(h), pos0 as int, 0, 0) == gw) by {
                    broadcast use vstd::seq_lib::group_seq_lib_default;
                }
                assert(crate::model::raw_writes(b@, hrec(h), pos0 as int, 0, 0) == gw);
                assert(out@ == crate::model::bytes_replay(
                    crate::model::zero_bytes(4 * h.spec_n()),
                    crate::model::raw_writes(b@, hrec(h), pos0 as int, 0, 0),
                ));
            }
            return Err(Error::TruncatedPixels);
        }
        proof {
            assert(pos + line <= b@.len()) by (nonlinear_arith)
                requires b@.len() - pos >= line;
        }
        let c = match raw_sample(b, h, pos, x, line) {
            Ok(c) => c,
            Err(e) => {
                proof {
                    reveal(crate::model::raw_full);
                    reveal(crate::model::raw_gcur);
                    reveal(crate::model::raw_writes);
                    assert(!crate::model::raw_index_ok(b@, hrec(h), pos as int, x as int));
                    assert(crate::model::raw_writes(b@, hrec(h), pos as int, x as int, y as int)
                        == Seq::<crate::model::Write>::empty());
                    assert(crate::model::raw_writes(b@, hrec(h), pos0 as int, 0, 0) == gw) by {
                        broadcast use vstd::seq_lib::group_seq_lib_default;
                    }
                    assert(crate::model::raw_writes(b@, hrec(h), pos0 as int, 0, 0) == gw);
                    assert(out@ == crate::model::bytes_replay(
                        crate::model::zero_bytes(4 * h.spec_n()),
                        crate::model::raw_writes(b@, hrec(h), pos0 as int, 0, 0),
                    ));
                }
                return Err(e);
            },
        };
        proof {
            crate::model::lemma_raw_writes_step(b@, hrec(h), pos as int, x as int, y as int);
            assert(crate::model::raw_conv(b@, hrec(h), pos as int, x as int) == c@);
        }
        let o = out_offset(h, x, y);
        proof {
            lemma_hrec_top_down(h);
            lemma_hrec_dims(h);
            reveal(crate::model::cell_of);
            assert(o as int == 4 * crate::model::cell_of(hrec(h), x as int, y as int));
        }
        put(out, o, c);
        proof {
            crate::model::lemma_bytes_replay_push(
                crate::model::zero_bytes(4 * h.spec_n()),
                gw,
                crate::model::Write {
                    cell: crate::model::cell_of(hrec(h), x as int, y as int),
                    px: c@,
                },
            );
            reveal(crate::model::raw_writes);
        }
        proof {
            gw = gw.push(crate::model::Write {
                cell: crate::model::cell_of(hrec(h), x as int, y as int),
                px: c@,
            });
        }
        x = x + 1;
        gcur.1 = x;
    }
}

/// T9/T6 bridge: the concrete raw helper's returned `Result` and buffer are
/// exactly `mr_raw_out` — `MR`'s raw transition replayed onto the zero fill, as
/// the exact `Result` (`Ok((cursor, image))` or the exact first error).
spec fn raw_done_eq(
    b: Seq<u8>,
    m: crate::model::MHRecord,
    pos0: int,
    out: Seq<u8>,
    r: Result<usize, Error>,
) -> bool {
    match crate::model::mr_raw_out(b, m, pos0) {
        Ok((pc, o)) => match r {
            Ok(p) => p as int == pc && out == o,
            Err(_) => false,
        },
        Err((e, _pc, _xc, _yc)) => match e {
            crate::model::RawErr::Api(e2) => match r { Ok(_) => false, Err(er) => er == e2 },
            crate::model::RawErr::Outside => false,
        },
    }
}

fn decode_raw(b: &[u8], h: &Header, out: &mut Vec<u8>, steps: &mut Ghost<nat>) -> (r: Result<usize, Error>)
    requires
        header_inv(b@, *h), header_inv_light(h),
        old(out)@.len() == 4 * h.spec_n(),
        old(out)@ == crate::model::zero_bytes(4 * h.spec_n()),
        h.compression == 0 || h.compression == 3,
    ensures
        final(steps)@ <= old(steps)@ + h.spec_n() + h.spec_height() + 1,
        final(out)@.len() == 4 * h.spec_n(),
        match r { Ok(p) => h.pixel_start <= p <= b@.len(), Err(_) => true },
        // T9/T6: the returned result and buffer are exactly `MR`'s raw
        // transition replayed onto the zero fill.
        raw_done_eq(b@, hrec(h), h.pixel_start as int, final(out)@, r),
        // T6 on the whole image, at the manifest's quantification (start state
        // `(pixel_start, 0, 0)`): the exact first error and ghost cursor/position
        // or the exact success cursor, and the exact pixels.
        match r {
            Ok(p) => crate::model::raw_gcur(
                b@,
                hrec(h),
                h.pixel_start as int,
                0,
                0,
            ) == Ok((p as int, 0, h.spec_height())) && p as int <= b@.len(),
            Err(e) => exists|gp: int, gx: int, gy: int|
                crate::model::raw_gcur(b@, hrec(h), h.pixel_start as int, 0, 0) == Err((
                    crate::model::RawErr::Api(e),
                    gp,
                    gx,
                    gy,
                )),
        },
        final(out)@ == crate::model::bytes_replay(
            crate::model::zero_bytes(4 * h.spec_n()),
            crate::model::raw_writes(b@, hrec(h), h.pixel_start as int, 0, 0),
        ),
{
    proof {
        reveal(crate::model::raw_reachable);
        reveal(crate::model::zero_bytes);
        reveal(crate::model::bytes_replay);
        reveal(crate::model::raw_writes);
        reveal(crate::model::raw_gcur);
        reveal(crate::model::raw_full);
        assert(h.n == h.spec_n());
        assert(crate::model::raw_reachable(b@, hrec(h), h.pixel_start as int, 0, 0));
    }
    let mut gcur: (usize, usize, usize) = (h.pixel_start, 0, 0);
    let r = raw_loop(b, h, out, if h.mode == Mode::BmpFile { h.palette_end } else { h.pixel_start }, &mut gcur, steps);
    proof {
        reveal(crate::model::raw_full);
        reveal(crate::model::raw_gcur);
        match r {
            Ok(()) => {
                assert(crate::model::raw_gcur(b@, hrec(h), h.pixel_start as int, 0, 0) == Ok((
                    gcur.0 as int,
                    gcur.1 as int,
                    gcur.2 as int,
                )));
                crate::model::lemma_raw_gcur_ok_shape(b@, hrec(h), h.pixel_start as int, 0, 0);
                assert(exists|pc: int| crate::model::raw_gcur(
                    b@,
                    hrec(h),
                    h.pixel_start as int,
                    0,
                    0,
                ) == Ok((pc, 0, h.spec_height())));
                assert(crate::model::raw_gcur(b@, hrec(h), h.pixel_start as int, 0, 0) == Ok((
                    gcur.0 as int,
                    0,
                    h.spec_height(),
                )));
                assert(gcur.1 == 0);
                assert(gcur.2 as int == h.spec_height());
            },
            Err(_) => {},
        }
    }
    proof {
        assert(gcur.0 as int <= b@.len());
        assert(gcur.0 as int >= h.pixel_start as int);
    }
    proof {
        // T9/T6 bridge: fold `raw_loop`'s two projections back into `mr_raw_out`.
        reveal(raw_done_eq);
        reveal(crate::model::mr_raw_out);
        reveal(crate::model::raw_full);
        crate::model::lemma_raw_full_proj(b@, hrec(h), h.pixel_start as int, 0, 0);
    }
    match r {
        Ok(()) => Ok(gcur.0),
        Err(e) => Err(e),
    }
}

/// R8/R10: prefill every output pixel with palette entry zero (alpha 255). The
/// result is exactly the model's `pal_zero_bytes` fill, whatever the buffer held
/// before. `count >= 1` for RLE (`header_inv`), so the fallback arm is dead.
fn fill_rle(b: &[u8], h: &Header, out: &mut Vec<u8>, steps: &mut Ghost<nat>)
    requires
        header_inv(b@, *h), header_inv_light(h),
        old(out)@.len() == 4 * h.spec_n(),
        h.compression == 1 || h.compression == 2,
    ensures
        final(steps)@ == old(steps)@ + h.spec_n(),
        final(out)@.len() == old(out)@.len(),
        final(out)@ == crate::model::pal_zero_bytes(b@, hrec(h)),
{
    let ghost m = hrec(h);
    let ghost e = crate::model::pal_entry(b@, m.palette_start, m.palette_stride, 0);
    proof {
        reveal(hrec);
        reveal(header_record);
        reveal(header_inv);
        assert(m.palette_start == h.palette_start as int);
        assert(m.palette_stride == h.pal_stride as int);
        assert(m.n == h.spec_n());
        assert(h.count >= 1u32);
    }
    let c = match colour(b, h.palette_start, h.pal_stride, h.count, 0) {
        Ok(c) => c,
        Err(_) => [0, 0, 0, 255],
    };
    proof {
        assert(c@ == e);
        assert(e.len() == 4) by {
            reveal(crate::model::pal_entry);
        }
    }
    let mut i: usize = 0;
    let ghost s0: nat = steps@;
    while i < h.n
        invariant
            s0 == old(steps)@, steps@ == s0 + i,
            header_inv_light(h),
            out@.len() == 4 * h.spec_n(),
            i <= h.n,
            m == hrec(h),
            c@ == e,
            e.len() == 4,
            forall|k: int| 0 <= k < i ==> #[trigger] crate::model::quad_at(out@, 4 * k, e),
        decreases h.n - i,
    {
        let o = 4 * i;
        let ghost before = out@;
        put(out, o, c);
        proof {
            reveal(crate::model::quad_at);
            assert forall|k: int| 0 <= k < i + 1 implies #[trigger] crate::model::quad_at(
                out@,
                4 * k,
                e,
            ) by {
                if k < i {
                    assert(crate::model::quad_at(before, 4 * k, e));
                    assert(4 * k + 3 < o as int);
                    crate::model::lemma_replay_bytes_at_other(before, o as int, c@, 4 * k);
                    crate::model::lemma_replay_bytes_at_other(before, o as int, c@, 4 * k + 1);
                    crate::model::lemma_replay_bytes_at_other(before, o as int, c@, 4 * k + 2);
                    crate::model::lemma_replay_bytes_at_other(before, o as int, c@, 4 * k + 3);
                } else {
                    assert(k == i);
                }
            }
        }
        i += 1;
        proof { *steps = Ghost(steps@ + 1); }
    }
    proof {
        assert(i == h.n);
        crate::model::lemma_pal_zero_bytes_len(b@, m);
        crate::model::lemma_quad_at_to_pal_zero(out@, b@, m, crate::model::pal_zero_bytes(b@, m));
    }
}

// ---------------------------------------------------------------------------
// T7 (M4a): the RLE4/RLE8 helper at the manifest's quantification, with the
// ghost final cursor/position on both exits, and the concrete pixels written so
// far tied to the model's partial output.
// ---------------------------------------------------------------------------

/// Paint `q` run pixels with code byte `code` starting at `(x, y)`,
/// accumulating the model's `Write`s in the ghost `gw`. On success all `q` are
/// painted; on `PaletteIndexOutOfRange` the first out-of-range index is at
/// position `j`, so exactly the leading `j` (the model's `run_good`) are painted
/// and `gw` is `run_writes(0..j)`.
fn paint_run(
    b: &[u8],
    h: &Header,
    out: &mut Vec<u8>,
    code: usize,
    x: usize,
    y: usize,
    q: usize,
    Ghost(gw): Ghost<Seq<crate::model::Write>>,
    steps: &mut Ghost<nat>,
) -> (r: Result<(), Error>)
    requires
        header_inv(b@, *h), header_inv_light(h),
        (y as int) < h.spec_height(),
        x + q <= h.spec_width(), q <= 255,
        old(out)@.len() == 4 * h.spec_n(),
        old(out)@ == crate::model::bytes_replay(crate::model::pal_zero_bytes(b@, hrec(h)), gw),
        h.spec_depth() == 4 || h.spec_depth() == 8,
        h.spec_compression() == 1 || h.spec_compression() == 2,
        (code as int) <= 255,
    ensures
        old(steps)@ <= final(steps)@ <= old(steps)@ + q,
        r is Ok ==> final(steps)@ == old(steps)@ + q,
        final(out)@.len() == 4 * h.spec_n(),
        match r {
            Ok(()) => {
                &&& final(out)@ == crate::model::bytes_replay(
                    crate::model::pal_zero_bytes(b@, hrec(h)),
                    gw + crate::model::run_writes(
                        b@,
                        hrec(h),
                        x as int,
                        y as int,
                        code as int,
                        0,
                        q as int,
                    ),
                )
                &&& crate::model::run_ok(hrec(h), code as int, q as int)
            },
            Err(e) => {
                &&& e == Error::PaletteIndexOutOfRange
                &&& crate::model::run_good(hrec(h), code as int, q as int, 0) < q as int
                &&& final(out)@ == crate::model::bytes_replay(
                    crate::model::pal_zero_bytes(b@, hrec(h)),
                    gw + crate::model::run_writes(
                        b@,
                        hrec(h),
                        x as int,
                        y as int,
                        code as int,
                        0,
                        crate::model::run_good(hrec(h), code as int, q as int, 0),
                    ),
                )
            },
        },
{
    let mut j: usize = 0;
    let ghost s0: nat = steps@;
    let ghost mut gj: Seq<crate::model::Write> = gw;
    proof {
        reveal(crate::model::run_index);
        reveal(crate::model::run_writes);
        reveal(header_inv);
        assert(crate::model::run_writes(b@, hrec(h), x as int, y as int, code as int, 0, 0)
            == Seq::<crate::model::Write>::empty());
        assert(gj == gw + crate::model::run_writes(
            b@,
            hrec(h),
            x as int,
            y as int,
            code as int,
            0,
            0,
        )) by {
            broadcast use vstd::seq_lib::group_seq_lib_default;
        };
    }
    while j < q
        invariant
            s0 == old(steps)@, steps@ == s0 + j,
            header_inv(b@, *h), header_inv_light(h),
            (y as int) < h.spec_height(),
            x + q <= h.spec_width(), q <= 255,
            j <= q,
            (code as int) <= 255,
            h.spec_depth() == 4 || h.spec_depth() == 8,
            out@.len() == 4 * h.spec_n(),
            out@ == crate::model::bytes_replay(
                crate::model::pal_zero_bytes(b@, hrec(h)),
                gj,
            ),
            gj == gw + crate::model::run_writes(
                b@,
                hrec(h),
                x as int,
                y as int,
                code as int,
                0,
                j as int,
            ),
            forall|t: int| 0 <= t < j ==> #[trigger] crate::model::index_ok(
                hrec(h),
                crate::model::run_index(code as int, h.spec_depth(), t),
            ),
        decreases q - j,
    {
        proof { *steps = Ghost(steps@ + 1); }
        let ix: u32 = if h.bits == 8 {
            code as u32
        } else {
            ((code >> (if j % 2 == 0 { 4 } else { 0 })) & 0x0f) as u32
        };
        proof {
            reveal(crate::model::run_index);
            reveal(crate::model::depth_ok);
            reveal(header_inv);
            reveal(header_inv_light);
            assert((code as int) <= 255);
            assert((code as int) < 256) by (nonlinear_arith)
                requires (code as int) <= 255;
            crate::model::lemma_run_index_eq(code as int, h.spec_depth(), j as int);
            if h.bits == 8u32 {
                assert(h.spec_depth() == 8);
                assert(crate::model::run_index(code as int, h.spec_depth(), j as int)
                    == code as int);
                assert(ix as int == code as int);
            } else {
                assert(h.bits == 4u32);
                assert(h.spec_depth() == 4);
                if j % 2 == 0 {
                    assert(crate::model::run_index(code as int, h.spec_depth(), j as int)
                        == (code as int) / 16);
                    assert(((code >> 4) & 0x0f) as int == (code as int) / 16) by (bit_vector)
                        requires (code as int) >= 0, (code as int) < 256;
                    assert(ix as int == (code as int) / 16);
                } else {
                    assert(crate::model::run_index(code as int, h.spec_depth(), j as int)
                        == (code as int) % 16);
                    assert((code & 0x0f) as int == (code as int) % 16) by (bit_vector)
                        requires (code as int) >= 0, (code as int) < 256;
                    assert(((code >> 0) & 0x0f) == (code & 0x0f)) by (bit_vector);
                    assert(ix as int == (code as int) % 16);
                }
            }
            assert(ix as int == crate::model::run_index(code as int, h.spec_depth(), j as int));
            crate::model::lemma_run_writes_unfold(
                b@,
                hrec(h),
                x as int,
                y as int,
                code as int,
                0,
                q as int,
            );
        }
        let c = match colour(b, h.palette_start, h.pal_stride, h.count, ix) {
            Ok(c) => c,
            Err(e) => {
                proof {
                    // `ix` is out of range, so the model's good prefix is exactly
                    // `j`; `gj` is `run_writes(0..j)`.
                    reveal(crate::model::index_ok);
                    reveal(crate::model::run_good);
                    assert(!crate::model::index_ok(
                        hrec(h),
                        crate::model::run_index(code as int, h.spec_depth(), j as int),
                    ));
                    assert forall|t: int| 0 <= t < j implies #[trigger] crate::model::index_ok(
                        hrec(h),
                        crate::model::run_index(code as int, h.spec_depth(), t),
                    ) by {
                        reveal(crate::model::run_good);
                    }
                    crate::model::lemma_run_good_prefix(
                        hrec(h),
                        code as int,
                        q as int,
                        j as int,
                    );
                    assert(crate::model::run_good(hrec(h), code as int, q as int, 0) == j as int);
                }
                return Err(e);
            },
        };
        proof {
            reveal(crate::model::index_ok);
            reveal(crate::model::run_index);
            assert(ix < h.count);
            assert(crate::model::index_ok(
                hrec(h),
                crate::model::run_index(code as int, h.spec_depth(), j as int),
            ));
            assert forall|t: int| 0 <= t < j + 1 implies #[trigger] crate::model::index_ok(
                hrec(h),
                crate::model::run_index(code as int, h.spec_depth(), t),
            ) by {
                if t < j {
                }
            }
        }
        let o = out_offset(h, x + j, y);
        proof {
            lemma_hrec_top_down(h);
            lemma_hrec_dims(h);
            reveal(crate::model::cell_of);
            assert(o as int == 4 * crate::model::cell_of(hrec(h), (x + j) as int, y as int));
        }
        put(out, o, c);
        proof {
            reveal(crate::model::run_writes);
            crate::model::lemma_bytes_replay_push(
                crate::model::pal_zero_bytes(b@, hrec(h)),
                gj,
                crate::model::Write {
                    cell: crate::model::cell_of(hrec(h), (x + j) as int, y as int),
                    px: c@,
                },
            );
        }
        proof {
            reveal(crate::model::run_writes);
            crate::model::lemma_run_writes_snoc(
                b@,
                hrec(h),
                x as int,
                y as int,
                code as int,
                j as int,
                q as int,
            );
            gj = gj.push(crate::model::Write {
                cell: crate::model::cell_of(hrec(h), (x + j) as int, y as int),
                px: c@,
            });
            vstd::seq_lib::lemma_concat_associative(
                gw,
                crate::model::run_writes(b@, hrec(h), x as int, y as int, code as int, 0, j as int),
                seq![crate::model::Write {
                    cell: crate::model::cell_of(hrec(h), (x + j) as int, y as int),
                    px: c@,
                }],
            );
        }
        j = j + 1;
    }
    proof {
        // The loop completed, so every index `0..q` is in range.
        assert(j == q);
        assert forall|t: int| 0 <= t < q implies #[trigger] crate::model::index_ok(
            hrec(h),
            crate::model::run_index(code as int, h.spec_depth(), t),
        ) by {
        }
        crate::model::lemma_run_good_bound(hrec(h), code as int, q as int, 0);
        crate::model::lemma_run_good_is_run_ok(hrec(h), code as int, q as int);
    }
    Ok(())
}

/// Paint the `q` pixels of an absolute packet whose payload starts at `pay`
/// (RLE8: one index byte per pixel; RLE4: high nibble first), accumulating the
/// model's `Write`s in the ghost `gw`. On success all `q` are painted; on
/// `PaletteIndexOutOfRange` exactly the leading `abs_good` are painted and the
/// accumulator is `abs_writes(0..abs_good)`.
fn paint_abs(
    b: &[u8],
    h: &Header,
    out: &mut Vec<u8>,
    pay: usize,
    x: usize,
    y: usize,
    q: usize,
    Ghost(gw): Ghost<Seq<crate::model::Write>>,
    steps: &mut Ghost<nat>,
) -> (r: Result<(), Error>)
    requires
        header_inv(b@, *h), header_inv_light(h),
        (y as int) < h.spec_height(),
        x + q <= h.spec_width(), 1 <= q <= 255,
        old(out)@.len() == 4 * h.spec_n(),
        old(out)@ == crate::model::bytes_replay(crate::model::pal_zero_bytes(b@, hrec(h)), gw),
        h.spec_depth() == 4 || h.spec_depth() == 8,
        h.spec_compression() == 1 || h.spec_compression() == 2,
        (pay as int) + crate::model::rle_payload(q as int, h.spec_depth()) <= b@.len(),
    ensures
        old(steps)@ <= final(steps)@ <= old(steps)@ + q,
        r is Ok ==> final(steps)@ == old(steps)@ + q,
        final(out)@.len() == 4 * h.spec_n(),
        match r {
            Ok(()) => {
                &&& final(out)@ == crate::model::bytes_replay(
                    crate::model::pal_zero_bytes(b@, hrec(h)),
                    gw + crate::model::abs_writes(
                        b@, hrec(h), x as int, y as int, pay as int, 0, q as int,
                    ),
                )
                &&& crate::model::abs_ok(b@, hrec(h), pay as int, q as int)
            },
            Err(e) => {
                &&& e == Error::PaletteIndexOutOfRange
                &&& crate::model::abs_good(b@, hrec(h), pay as int, q as int, 0) < q as int
                &&& final(out)@ == crate::model::bytes_replay(
                    crate::model::pal_zero_bytes(b@, hrec(h)),
                    gw + crate::model::abs_writes(
                        b@, hrec(h), x as int, y as int, pay as int, 0,
                        crate::model::abs_good(b@, hrec(h), pay as int, q as int, 0),
                    ),
                )
            },
        },
{
    let mut j: usize = 0;
    let ghost s0: nat = steps@;
    let ghost mut gj: Seq<crate::model::Write> = gw;
    let ghost m = hrec(h);
    proof {
        reveal(crate::model::abs_index);
        reveal(crate::model::abs_writes);
        reveal(header_inv);
        reveal(hrec);
        reveal(header_record);
        assert(m.depth == h.spec_depth());
        assert(m.count == h.count as int);
        assert(crate::model::abs_writes(b@, m, x as int, y as int, pay as int, 0, 0)
            == Seq::<crate::model::Write>::empty());
        assert(gj == gw + crate::model::abs_writes(b@, m, x as int, y as int, pay as int, 0, 0)) by {
            broadcast use vstd::seq_lib::group_seq_lib_default;
        };
    }
    while j < q
        invariant
            s0 == old(steps)@, steps@ == s0 + j,
            header_inv(b@, *h), header_inv_light(h),
            m == hrec(h),
            m.depth == h.spec_depth(),
            m.count == h.count as int,
            (y as int) < h.spec_height(),
            x + q <= h.spec_width(), 1 <= q <= 255,
            j <= q,
            h.spec_depth() == 4 || h.spec_depth() == 8,
            (pay as int) + crate::model::rle_payload(q as int, h.spec_depth()) <= b@.len(),
            out@.len() == 4 * h.spec_n(),
            out@ == crate::model::bytes_replay(crate::model::pal_zero_bytes(b@, m), gj),
            gj == gw + crate::model::abs_writes(b@, m, x as int, y as int, pay as int, 0, j as int),
            forall|t: int| 0 <= t < j ==> #[trigger] crate::model::index_ok(
                m,
                crate::model::abs_index(b@, h.spec_depth(), pay as int, t),
            ),
        decreases q - j,
    {
        proof { *steps = Ghost(steps@ + 1); }
        proof {
            reveal(crate::model::rle_payload);
            reveal(header_inv);
            reveal(header_inv_light);
            if h.bits == 8u32 {
                assert(crate::model::rle_payload(q as int, h.spec_depth()) == q as int);
                assert(pay + j < b@.len());
            } else {
                assert(h.bits == 4u32);
                assert(crate::model::rle_payload(q as int, h.spec_depth()) == (q as int + 1) / 2);
                assert(j / 2 <= (q - 1) / 2) by (nonlinear_arith)
                    requires j < q, j >= 0, q >= 1;
                assert((q as int - 1) / 2 < (q as int + 1) / 2) by (nonlinear_arith)
                    requires q >= 1;
                assert(pay + j / 2 < b@.len());
            }
        }
        let byte: u8 = if h.bits == 8 { rd(b, pay + j) } else { rd(b, pay + j / 2) };
        let ix: u32 = if h.bits == 8 {
            byte as u32
        } else {
            ((byte >> (if j % 2 == 0 { 4 } else { 0 })) & 0x0f) as u32
        };
        proof {
            reveal(crate::model::abs_index);
            if h.bits == 8u32 {
                assert(byte as int == b@[pay as int + j as int]);
                assert(ix as int == crate::model::abs_index(b@, h.spec_depth(), pay as int, j as int));
            } else {
                assert(byte as int == b@[pay as int + (j as int) / 2]);
                if j % 2 == 0 {
                    assert(((byte >> 4) & 0x0f) as int == (byte as int) / 16) by (bit_vector);
                    assert(ix as int == (byte as int) / 16);
                } else {
                    assert(((byte >> 0) & 0x0f) as int == (byte as int) % 16) by (bit_vector);
                    assert(ix as int == (byte as int) % 16);
                }
                assert(ix as int == crate::model::abs_index(b@, h.spec_depth(), pay as int, j as int));
            }
        }
        let c = match colour(b, h.palette_start, h.pal_stride, h.count, ix) {
            Ok(c) => c,
            Err(e) => {
                proof {
                    reveal(crate::model::index_ok);
                    assert(!crate::model::index_ok(
                        m,
                        crate::model::abs_index(b@, h.spec_depth(), pay as int, j as int),
                    ));
                    crate::model::lemma_abs_good_prefix(b@, m, pay as int, q as int, j as int);
                    assert(crate::model::abs_good(b@, m, pay as int, q as int, 0) == j as int);
                }
                return Err(e);
            },
        };
        proof {
            reveal(crate::model::index_ok);
            assert(ix < h.count);
            assert(crate::model::index_ok(
                m,
                crate::model::abs_index(b@, h.spec_depth(), pay as int, j as int),
            ));
            assert forall|t: int| 0 <= t < j + 1 implies #[trigger] crate::model::index_ok(
                m,
                crate::model::abs_index(b@, h.spec_depth(), pay as int, t),
            ) by {
                if t < j {
                }
            }
        }
        let o = out_offset(h, x + j, y);
        proof {
            lemma_hrec_top_down(h);
            lemma_hrec_dims(h);
            reveal(crate::model::cell_of);
            assert(o as int == 4 * crate::model::cell_of(m, (x + j) as int, y as int));
        }
        put(out, o, c);
        proof {
            let w = crate::model::Write {
                cell: crate::model::cell_of(m, (x + j) as int, y as int),
                px: c@,
            };
            crate::model::lemma_bytes_replay_push(crate::model::pal_zero_bytes(b@, m), gj, w);
            assert(c@ == crate::model::conv(b@, m, crate::model::abs_index(b@, m.depth, pay as int, j as int)));
            crate::model::lemma_abs_writes_snoc_at(b@, m, x as int, y as int, pay as int, j as int, 0);
            gj = gj.push(w);
            vstd::seq_lib::lemma_concat_associative(
                gw,
                crate::model::abs_writes(b@, m, x as int, y as int, pay as int, 0, j as int),
                seq![w],
            );
        }
        j = j + 1;
    }
    proof {
        assert(j == q);
        crate::model::lemma_abs_good_bound(b@, m, pay as int, q as int, 0);
        crate::model::lemma_abs_good_is_abs_ok(b@, m, pay as int, q as int);
    }
    Ok(())
}

/// T7's per-command contract: one command read at `(p0, x0, y0)` with the
/// ghost accumulator `gw0` leaves `(p1, x1, y1, gw1)`. `Ok(false)` continues
/// (both `MR` projections are carried across the command); `Ok(true)` is EOB
/// (`MR` succeeds at the exit state); `Err(e)` is the model's exact first error
/// with its ghost cursor/position, and `gw1` is the model's whole write list.
spec fn cmd_post(
    b: Seq<u8>,
    m: crate::model::MHRecord,
    p0: int, x0: int, y0: int,
    p1: int, x1: int, y1: int,
    gw0: Seq<crate::model::Write>,
    gw1: Seq<crate::model::Write>,
    r: Result<bool, Error>,
) -> bool {
    match r {
        Ok(eob) => if eob {
            &&& crate::model::mr_gcur(b, m, p0, x0, y0) == Ok::<(int, int, int), (crate::model::RleErr, int, int, int)>((p1, x1, y1))
            &&& gw0 + crate::model::mr_writes(b, m, p0, x0, y0) == gw1
        } else {
            &&& p1 >= p0 + 2
            &&& crate::model::mr_gcur(b, m, p0, x0, y0) == crate::model::mr_gcur(b, m, p1, x1, y1)
            &&& gw0 + crate::model::mr_writes(b, m, p0, x0, y0) == gw1 + crate::model::mr_writes(b, m, p1, x1, y1)
        },
        Err(e) => {
            &&& crate::model::mr_gcur(b, m, p0, x0, y0) == Err::<(int, int, int), (crate::model::RleErr, int, int, int)>((crate::model::RleErr::Api(e), p1, x1, y1))
            &&& gw0 + crate::model::mr_writes(b, m, p0, x0, y0) == gw1
        },
    }
}

/// R11: an absolute packet `00 q` (`3 <= q <= 255`) after its two command bytes
/// (`p0` is the command cursor, `*old(pos) == p0 + 2`): row bounds, then the
/// whole padded payload, then each index in order. The pad byte is consumed
/// before painting, so an index failure leaves the cursor after the packet.
fn rle_abs_packet(
    b: &[u8],
    h: &Header,
    out: &mut Vec<u8>,
    pos: &mut usize,
    x: &mut usize,
    y: &mut usize,
    q: usize,
    Ghost(p0): Ghost<int>,
    Ghost(gw0): Ghost<Seq<crate::model::Write>>,
    steps: &mut Ghost<nat>,
) -> (r: (Result<bool, Error>, Ghost<Seq<crate::model::Write>>))
    requires
        header_inv(b@, *h), header_inv_light(h),
        h.compression == 1 || h.compression == 2,
        old(out)@.len() == 4 * h.spec_n(),
        0 <= p0, p0 + 2 == *old(pos) as int, *old(pos) <= b@.len(),
        b@[p0] == 0, b@[p0 + 1] as int == q as int, 3 <= q <= 255,
        *old(x) <= h.spec_width(), *old(y) <= h.spec_height(),
        old(out)@ == crate::model::bytes_replay(crate::model::pal_zero_bytes(b@, hrec(h)), gw0),
    ensures
        r.0 != Ok(true),
        *old(y) == h.spec_height() ==> final(steps)@ == old(steps)@,
        r.0 == Ok(false) ==> *final(y) < h.spec_height(),
        old(steps)@ <= final(steps)@ <= old(steps)@ + q,
        final(steps)@ as int <= old(steps)@ as int + h.spec_width() - *old(x) as int,
        r.0 == Ok(false) ==> final(steps)@ == old(steps)@ + q
            && *final(x) == *old(x) + q && *final(y) == *old(y),
        final(out)@.len() == 4 * h.spec_n(),
        *final(pos) <= b@.len(), *final(pos) >= *old(pos),
        *final(x) <= h.spec_width(), *final(y) <= h.spec_height(),
        final(out)@ == crate::model::bytes_replay(crate::model::pal_zero_bytes(b@, hrec(h)), r.1@),
        cmd_post(
            b@, hrec(h), p0, *old(x) as int, *old(y) as int,
            *final(pos) as int, *final(x) as int, *final(y) as int, gw0, r.1@, r.0,
        ),
{
    let ghost m = hrec(h);
    let ghost x0: int = *x as int;
    let ghost y0: int = *y as int;
    proof {
        reveal(hrec);
        reveal(header_record);
        reveal(header_inv);
        reveal(header_inv_light);
        reveal(cmd_post);
        assert(m.width == h.spec_width());
        assert(m.height == h.spec_height());
        assert(m.depth == h.spec_depth());
        assert(m.depth == 4 || m.depth == 8);
        assert(gw0 + Seq::<crate::model::Write>::empty() == gw0) by {
            broadcast use vstd::seq_lib::group_seq_lib_default;
        }
    }
    if *y >= h.height as usize || *x + q > h.width as usize {
        proof {
            crate::model::lemma_mr_exit_abs_bounds(b@, m, p0, x0, y0);
        }
        return (Err(Error::RleBounds), Ghost(gw0));
    }
    let payload = if h.bits == 8 { q } else { (q + 1) / 2 };
    let padded = payload + payload % 2;
    proof {
        reveal(crate::model::rle_payload);
        reveal(crate::model::rle_padded);
        assert(payload as int == crate::model::rle_payload(q as int, m.depth));
        assert(padded as int == crate::model::rle_padded(q as int, m.depth));
        assert(payload <= padded) by (nonlinear_arith)
            requires padded == payload + payload % 2;
    }
    if b.len() - *pos < padded {
        proof {
            crate::model::lemma_mr_exit_abs_trunc(b@, m, p0, x0, y0);
        }
        return (Err(Error::RleTruncated), Ghost(gw0));
    }
    let pay = *pos;
    *pos = *pos + padded;
    proof {
        assert(pay as int == p0 + 2);
        assert(*pos as int == p0 + 2 + crate::model::rle_padded(q as int, m.depth));
        assert(*pos <= b@.len());
        assert((pay as int) + crate::model::rle_payload(q as int, h.spec_depth()) <= b@.len());
    }
    let pr = paint_abs(b, h, out, pay, *x, *y, q, Ghost(gw0), steps);
    match pr {
        Ok(()) => {
            proof {
                crate::model::lemma_mr_step_abs(b@, m, p0, x0, y0);
                crate::model::lemma_mr_step_abs_gcur(b@, m, p0, x0, y0);
            }
            let ghost ws = crate::model::abs_writes(b@, m, x0, y0, p0 + 2, 0, q as int);
            let ghost gw1 = gw0 + ws;
            proof {
                vstd::seq_lib::lemma_concat_associative(
                    gw0, ws, crate::model::mr_writes(b@, m, *pos as int, x0 + q as int, y0),
                );
            }
            *x = *x + q;
            (Ok(false), Ghost(gw1))
        },
        Err(e) => {
            proof {
                crate::model::lemma_mr_exit_abs_index(b@, m, p0, x0, y0);
            }
            let ghost gw1 = gw0 + crate::model::abs_writes(
                b@, m, x0, y0, p0 + 2, 0, crate::model::abs_good(b@, m, p0 + 2, q as int, 0),
            );
            (Err(e), Ghost(gw1))
        },
    }
}

/// R10/R11: one RLE command at `*old(pos)`: two-byte presence, then the
/// encoded run / EOB / EOL / delta / absolute cases in R14 order. The returned
/// ghost list is the accumulator after this command; `cmd_post` relates the
/// entry and exit states on every exit.
fn rle_command(
    b: &[u8],
    h: &Header,
    out: &mut Vec<u8>,
    pos: &mut usize,
    x: &mut usize,
    y: &mut usize,
    Ghost(gw0): Ghost<Seq<crate::model::Write>>,
    steps: &mut Ghost<nat>,
) -> (r: (Result<bool, Error>, Ghost<Seq<crate::model::Write>>))
    requires
        header_inv(b@, *h), header_inv_light(h),
        h.compression == 1 || h.compression == 2,
        old(out)@.len() == 4 * h.spec_n(),
        *old(pos) <= b@.len(),
        *old(x) <= h.spec_width(), *old(y) <= h.spec_height(),
        old(out)@ == crate::model::bytes_replay(crate::model::pal_zero_bytes(b@, hrec(h)), gw0),
    ensures
        final(steps)@ >= old(steps)@ + 1,
        *old(y) == h.spec_height() ==> final(steps)@ == old(steps)@ + 1,
        r.0 == Ok(false) && *final(y) == h.spec_height() ==> *final(x) == 0,
        match r.0 {
            Ok(true) => final(steps)@ == old(steps)@ + 1,
            Ok(false) => final(steps)@ as int + (*old(y) as int) * h.spec_width() + *old(x) as int
                <= old(steps)@ as int + 1 + (*final(y) as int) * h.spec_width() + *final(x) as int,
            Err(_) => final(steps)@ as int <= old(steps)@ as int + 1 + h.spec_width() - *old(x) as int,
        },
        r.0 == Ok(true) ==> *final(pos) >= *old(pos) + 2,
        final(out)@.len() == 4 * h.spec_n(),
        *final(pos) <= b@.len(), *final(pos) >= *old(pos),
        *final(x) <= h.spec_width(), *final(y) <= h.spec_height(),
        final(out)@ == crate::model::bytes_replay(crate::model::pal_zero_bytes(b@, hrec(h)), r.1@),
        cmd_post(
            b@, hrec(h), *old(pos) as int, *old(x) as int, *old(y) as int,
            *final(pos) as int, *final(x) as int, *final(y) as int, gw0, r.1@, r.0,
        ),
{
    let ghost m = hrec(h);
    let ghost p0: int = *pos as int;
    let ghost x0: int = *x as int;
    let ghost y0: int = *y as int;
    proof {
        reveal(hrec);
        reveal(header_record);
        reveal(header_inv);
        reveal(header_inv_light);
        reveal(cmd_post);
        assert(m.width == h.spec_width());
        assert(m.height == h.spec_height());
        assert(m.depth == h.spec_depth());
        assert(m.depth == 4 || m.depth == 8);
        assert(gw0 + Seq::<crate::model::Write>::empty() == gw0) by {
            broadcast use vstd::seq_lib::group_seq_lib_default;
        }
    }
    proof { *steps = Ghost(steps@ + 1); }
    if b.len() - *pos < 2 {
        proof {
            crate::model::lemma_mr_exit_truncated(b@, m, p0, x0, y0);
        }
        return (Err(Error::RleTruncated), Ghost(gw0));
    }
    let quantity = rd(b, *pos) as usize;
    let code = rd(b, *pos + 1) as usize;
    *pos = *pos + 2;
    proof {
        assert(quantity as int == b@[p0]);
        assert(code as int == b@[p0 + 1]);
        assert(*pos as int == p0 + 2);
    }
    if quantity != 0 {
        if *y >= h.height as usize || *x + quantity > h.width as usize {
            proof {
                crate::model::lemma_mr_exit_run_bounds(b@, m, p0, x0, y0);
            }
            return (Err(Error::RleBounds), Ghost(gw0));
        }
        let pr = paint_run(b, h, out, code, *x, *y, quantity, Ghost(gw0), steps);
        match pr {
            Ok(()) => {
                proof {
                    crate::model::lemma_mr_step_run(b@, m, p0, x0, y0);
                    crate::model::lemma_mr_step_run_gcur(b@, m, p0, x0, y0);
                }
                let ghost ws = crate::model::run_writes(b@, m, x0, y0, code as int, 0, quantity as int);
                let ghost gw1 = gw0 + ws;
                proof {
                    vstd::seq_lib::lemma_concat_associative(
                        gw0, ws, crate::model::mr_writes(b@, m, p0 + 2, x0 + quantity as int, y0),
                    );
                }
                *x = *x + quantity;
                proof {
                    assert(steps@ == old(steps)@ + 1 + quantity);
                    assert(*x as int == x0 + quantity as int);
                }
                return (Ok(false), Ghost(gw1));
            },
            Err(e) => {
                proof {
                    crate::model::lemma_mr_exit_run_index(b@, m, p0, x0, y0);
                }
                let ghost gw1 = gw0 + crate::model::run_writes(
                    b@, m, x0, y0, code as int, 0,
                    crate::model::run_good(m, code as int, quantity as int, 0),
                );
                proof { assert(steps@ as int <= old(steps)@ as int + 1 + h.spec_width() - x0); }
                return (Err(e), Ghost(gw1));
            },
        }
    } else if code == 1 {
        proof {
            crate::model::lemma_mr_exit_eob(b@, m, p0, x0, y0);
        }
        return (Ok(true), Ghost(gw0));
    } else if code == 0 {
        if *y >= h.height as usize {
            proof {
                crate::model::lemma_mr_exit_eol_bounds(b@, m, p0, x0, y0);
            }
            return (Err(Error::RleBounds), Ghost(gw0));
        }
        proof {
            crate::model::lemma_mr_step_eol(b@, m, p0, x0, y0);
        }
        *x = 0;
        *y = *y + 1;
        proof {
            assert(y0 * h.spec_width() + x0 <= (*y as int) * h.spec_width() + *x as int)
                by (nonlinear_arith) requires x0 <= h.spec_width(), *y as int == y0 + 1;
        }
        return (Ok(false), Ghost(gw0));
    } else if code == 2 {
        if b.len() - *pos < 2 {
            proof {
                crate::model::lemma_mr_exit_delta_trunc(b@, m, p0, x0, y0);
            }
            return (Err(Error::RleTruncated), Ghost(gw0));
        }
        let dx = rd(b, *pos) as usize;
        let dy = rd(b, *pos + 1) as usize;
        *pos = *pos + 2;
        proof {
            assert(dx as int == b@[p0 + 2]);
            assert(dy as int == b@[p0 + 3]);
            assert(*pos as int == p0 + 4);
        }
        if *y + dy >= h.height as usize || *x + dx >= h.width as usize {
            proof {
                crate::model::lemma_mr_exit_delta_bounds(b@, m, p0, x0, y0);
            }
            return (Err(Error::RleBounds), Ghost(gw0));
        }
        proof {
            crate::model::lemma_mr_step_delta(b@, m, p0, x0, y0);
        }
        *x = *x + dx;
        *y = *y + dy;
        proof {
            assert(y0 * h.spec_width() + x0 <= (*y as int) * h.spec_width() + *x as int)
                by (nonlinear_arith)
                requires 0 <= dy, h.spec_width() >= 0,
                    *y as int == y0 + dy as int, *x as int == x0 + dx as int, 0 <= dx;
        }
        return (Ok(false), Ghost(gw0));
    } else {
        let reply = rle_abs_packet(b, h, out, pos, x, y, code, Ghost(p0), Ghost(gw0), steps);
        proof {
            match reply.0 {
                Ok(false) => {
                    assert(steps@ == old(steps)@ + 1 + code);
                    assert(y0 * h.spec_width() + x0 + (code as int)
                        == (*y as int) * h.spec_width() + *x as int);
                },
                Err(_) => {
                    assert(steps@ as int <= old(steps)@ as int + 1 + h.spec_width() - x0);
                },
                Ok(true) => {},
            }
        }
        reply
    }
}

/// T7 on `MR`'s own result tuple at an arbitrary reachable start state
/// `(p0, x0, y0)` with the ghost accumulator `gw0` (the writes before the
/// start): on `Ok((pc, xc, yc, ws))` the loop returned `Ok`, its final state is
/// `(pc, xc, yc)` and the accumulator grew by exactly `ws`; on
/// `Err((Api(e), pc, xc, yc, ws))` the loop returned exactly `e`, its final
/// state is the model's ghost `(pc, xc, yc)` and the accumulator grew by the
/// model's partial output `ws`.
spec fn rle_exact_from(
    b: Seq<u8>,
    m: crate::model::MHRecord,
    p0: int, x0: int, y0: int,
    gc: (int, int, int),
    gw0: Seq<crate::model::Write>,
    gw1: Seq<crate::model::Write>,
    r: Result<(), Error>,
) -> bool {
    match crate::model::mr(b, m, p0, x0, y0) {
        Ok((pc, xc, yc, ws)) => {
            &&& r is Ok
            &&& gc.0 == pc && gc.1 == xc && gc.2 == yc
            &&& gw1 == gw0 + ws
        },
        Err((e, pc, xc, yc, ws)) => {
            &&& (match r { Ok(_) => false, Err(e2) => e == crate::model::RleErr::Api(e2) })
            &&& gc.0 == pc && gc.1 == xc && gc.2 == yc
            &&& gw1 == gw0 + ws
        },
    }
}

/// A bounded RLE cursor has positional potential at most two image lengths;
/// the second image length covers an arbitrary reachable state at y=height, x>0.
proof fn lemma_rle_work_potential(w: int, h: int, x: int, y: int)
    requires 1 <= w, 1 <= h, 0 <= x <= w, 0 <= y <= h,
    ensures 0 <= y * w + x <= 2 * w * h,
{
    assert(y * w + x <= 2 * w * h) by (nonlinear_arith)
        requires 1 <= w, 1 <= h, 0 <= x <= w, 0 <= y <= h;
}

/// T7: the RLE4/RLE8 command loop from an arbitrary reachable start state
/// `(*old(pos), *old(x), *old(y))`, for every finite stream. The variant is
/// `|b| - pos` (every command consumes at least two bytes); the nested
/// `<= 255`-pixel painters are `paint_run` and `paint_abs`. On both exits the
/// result, the final state and the ghost accumulator are `MR`'s
/// (`rle_exact_from`), and the buffer is the accumulator replayed onto the
/// palette-zero fill.
fn rle_loop(
    b: &[u8],
    h: &Header,
    out: &mut Vec<u8>,
    pos: &mut usize,
    x: &mut usize,
    y: &mut usize,
    Ghost(gw0): Ghost<Seq<crate::model::Write>>,
    steps: &mut Ghost<nat>,
) -> (r: (Result<(), Error>, Ghost<Seq<crate::model::Write>>))
    requires
        header_inv(b@, *h), header_inv_light(h),
        h.compression == 1 || h.compression == 2,
        old(out)@.len() == 4 * h.spec_n(),
        *old(pos) <= b@.len(),
        *old(x) <= h.spec_width(), *old(y) <= h.spec_height(),
        old(out)@ == crate::model::bytes_replay(crate::model::pal_zero_bytes(b@, hrec(h)), gw0),
    ensures
        (*old(x) == 0 && *old(y) == 0) ==> final(steps)@ as int <= old(steps)@ as int
            + (b@.len() - *old(pos) as int) / 2 + 1 + h.spec_n(),
        final(steps)@ as int <= old(steps)@ as int
            + (b@.len() - *old(pos) as int) / 2 + 1 + 2 * h.spec_n(),
        final(out)@.len() == 4 * h.spec_n(),
        *final(pos) <= b@.len(), *final(pos) >= *old(pos),
        *final(x) <= h.spec_width(), *final(y) <= h.spec_height(),
        final(out)@ == crate::model::bytes_replay(crate::model::pal_zero_bytes(b@, hrec(h)), r.1@),
        rle_exact_from(
            b@, hrec(h), *old(pos) as int, *old(x) as int, *old(y) as int,
            (*final(pos) as int, *final(x) as int, *final(y) as int),
            gw0, r.1@, r.0,
        ),
{
    let ghost m = hrec(h);
    let ghost p_start: int = *pos as int;
    let ghost x_start: int = *x as int;
    let ghost y_start: int = *y as int;
    let ghost mut gw: Seq<crate::model::Write> = gw0;
    let ghost s_start: nat = steps@;
    let ghost mut commands: nat = 0;
    // On continuing commands, all emitted iterations paint a pixel; the
    // difference between work and command charges counts those painted pixels.
    let ghost mut painted: nat = 0;
    proof {
        lemma_rle_work_potential(h.spec_width(), h.spec_height(), x_start, y_start);
        reveal(hrec);
        reveal(header_record);
        reveal(header_inv);
        reveal(header_inv_light);
        reveal(crate::model::rle_reachable);
        assert(crate::model::mr_writes(b@, m, p_start, x_start, y_start)
            == Seq::<crate::model::Write>::empty() + crate::model::mr_writes(b@, m, p_start, x_start, y_start)) by {
            broadcast use vstd::seq_lib::group_seq_lib_default;
        }
        // T7's domain holds at the start state, so `mr` is `mr_full` here.
        assert(m.compression == h.compression as int);
        assert(m.depth == h.spec_depth());
        assert(m.depth == 4 || m.depth == 8);
        assert(crate::model::rle_reachable(b@, m, p_start, x_start, y_start));
    }
    loop
        invariant
            header_inv(b@, *h), header_inv_light(h),
            h.compression == 1 || h.compression == 2,
            out@.len() == 4 * h.spec_n(),
            *pos <= b@.len(), *pos as int >= p_start,
            *x <= h.spec_width(), *y <= h.spec_height(),
            m == hrec(h),
            s_start == old(steps)@,
            2 * (commands as int) <= *pos as int - p_start,
            steps@ == s_start + commands + painted,
            painted as int + y_start * h.spec_width() + x_start
                <= (*y as int) * h.spec_width() + *x as int,
            (x_start == 0 && y_start == 0) ==> (*y < h.spec_height() || *x == 0),
            (x_start == 0 && y_start == 0) ==> painted as int <= h.spec_n(),
            steps@ as int + y_start * h.spec_width() + x_start
                <= s_start as int + commands as int + (*y as int) * h.spec_width() + *x as int,
            p_start == *old(pos) as int, x_start == *old(x) as int, y_start == *old(y) as int,
            crate::model::rle_reachable(b@, m, p_start, x_start, y_start),
            crate::model::mr_gcur(b@, m, p_start, x_start, y_start)
                == crate::model::mr_gcur(b@, m, *pos as int, *x as int, *y as int),
            gw0 + crate::model::mr_writes(b@, m, p_start, x_start, y_start)
                == gw + crate::model::mr_writes(b@, m, *pos as int, *x as int, *y as int),
            out@ == crate::model::bytes_replay(crate::model::pal_zero_bytes(b@, m), gw),
        decreases b@.len() - *pos as int,
    {
        let ghost pb: int = *pos as int;
        let ghost xb: int = *x as int;
        let ghost yb: int = *y as int;
        let ghost gwb = gw;
        let ghost sb: nat = steps@;
        proof {
            lemma_rle_work_potential(h.spec_width(), h.spec_height(), xb, yb);
            reveal(header_inv_light);
            assert(yb * h.spec_width() + xb <= 2 * h.spec_n()) by (nonlinear_arith)
                requires 0 <= yb <= h.spec_height(), 0 <= xb <= h.spec_width(),
                    1 <= h.spec_height(), 1 <= h.spec_width(),
                    h.spec_n() == h.spec_width() * h.spec_height();
            assert(sb as int + y_start * h.spec_width() + x_start
                <= s_start as int + commands as int + yb * h.spec_width() + xb);
        }
        let (pr, g) = rle_command(b, h, out, pos, x, y, Ghost(gw), steps);
        proof {
            reveal(header_inv_light);
            lemma_rle_work_potential(h.spec_width(), h.spec_height(), *x as int, *y as int);
            assert(2 * commands as int <= (b@.len() - p_start));
            gw = g@;
            reveal(cmd_post);
            reveal(rle_exact_from);
            reveal(crate::model::mr);
        }
        match pr {
            Ok(eob) => {
                if eob {
                    proof {
                        // Count the final EOB as a complete command: it consumed two bytes.
                        assert(2 * (commands as int + 1) <= *pos as int - p_start);
                        assert(commands as int + 1 <= (b@.len() - p_start) / 2)
                            by (nonlinear_arith)
                            requires 2 * (commands as int + 1) <= b@.len() - p_start;
                        assert(2 * (commands as int) <= b@.len() - p_start);
                        assert(commands as int <= (b@.len() - p_start) / 2)
                            by (nonlinear_arith)
                            requires 2 * (commands as int) <= b@.len() - p_start;
                        assert(steps@ as int <= sb as int + 1);
                        assert(yb * h.spec_width() + xb <= 2 * h.spec_n());
                        if x_start == 0 && y_start == 0 {
                            assert(sb == s_start + commands + painted);
                            assert(painted <= h.spec_n());
                            assert(steps@ as int <= s_start as int + commands as int + 1 + h.spec_n());
                        }
                        assert(steps@ as int <= s_start as int + commands as int + 1 + 2 * h.spec_n());
                        assert(crate::model::mr_gcur(b@, m, p_start, x_start, y_start)
                            == Ok::<(int, int, int), (crate::model::RleErr, int, int, int)>((*pos as int, *x as int, *y as int)));
                        assert(gw0 + crate::model::mr_writes(b@, m, p_start, x_start, y_start) == gw);
                        crate::model::lemma_mr_gcur_proj(b@, m, p_start, x_start, y_start);
                    }
                    return (Ok(()), Ghost(gw));
                }
                proof {
                    assert(2 * (commands as int + 1) <= *pos as int - p_start);
                    assert(steps@ as int + y_start * h.spec_width() + x_start
                        <= s_start as int + (commands as int + 1)
                            + (*y as int) * h.spec_width() + *x as int);
                    commands = commands + 1;
                    assert(s_start + commands <= steps@);
                    painted = (steps@ - s_start - commands) as nat;
                    assert(steps@ == s_start + commands + painted);
                    assert(painted as int + y_start * h.spec_width() + x_start
                        <= (*y as int) * h.spec_width() + *x as int);
                    if x_start == 0 && y_start == 0 {
                        assert(*y < h.spec_height() || *x == 0);
                        crate::model::rle_painted_at_most_N(
                            h.spec_width(), h.spec_height(), painted as int, *x as int, *y as int);
                    }
                    assert(crate::model::mr_gcur(b@, m, p_start, x_start, y_start)
                        == crate::model::mr_gcur(b@, m, *pos as int, *x as int, *y as int));
                    assert(gwb + crate::model::mr_writes(b@, m, pb, xb, yb)
                        == gw + crate::model::mr_writes(b@, m, *pos as int, *x as int, *y as int));
                }
            },
            Err(e) => {
                proof {
                    // A failing command is complete only if it advanced at least two bytes;
                    // the shorter two-byte-presence failure gets its separate +1 charge.
                    if *pos as int >= pb + 2 {
                        assert(2 * (commands as int + 1) <= *pos as int - p_start);
                        assert(commands as int + 1 <= (b@.len() - p_start) / 2)
                            by (nonlinear_arith)
                            requires 2 * (commands as int + 1) <= b@.len() - p_start;
                    }
                    assert(2 * (commands as int) <= b@.len() - p_start);
                    assert(commands as int <= (b@.len() - p_start) / 2)
                        by (nonlinear_arith)
                        requires 2 * (commands as int) <= b@.len() - p_start;
                    assert((yb + 1) * h.spec_width() <= 2 * h.spec_n())
                        by (nonlinear_arith)
                        requires 0 <= yb <= h.spec_height(), 1 <= h.spec_width(),
                            1 <= h.spec_height(), h.spec_n() == h.spec_width() * h.spec_height();
                    assert(steps@ as int <= sb as int + 1 + h.spec_width() - xb);
                    assert(sb as int + y_start * h.spec_width() + x_start
                        <= s_start as int + commands as int + yb * h.spec_width() + xb);
                    assert(steps@ as int <= s_start as int + commands as int + 1 + 2 * h.spec_n())
                        by (nonlinear_arith)
                        requires
                            steps@ as int <= sb as int + 1 + h.spec_width() - xb,
                            sb as int + y_start * h.spec_width() + x_start
                                <= s_start as int + commands as int + yb * h.spec_width() + xb,
                            (yb + 1) * h.spec_width() <= 2 * h.spec_n(),
                            0 <= y_start, 0 <= x_start, 0 <= h.spec_width();
                    if x_start == 0 && y_start == 0 {
                        // A rejected palette index costs one painter iteration;
                        // counting it as painted still fits in the remaining row.
                        if yb == h.spec_height() {
                            assert(xb == 0);
                            assert(steps@ == sb + 1);
                            assert(painted <= h.spec_n());
                            assert(sb == s_start + commands + painted);
                            assert(steps@ as int <= s_start as int + commands as int + 1 + h.spec_n());
                        } else {
                            assert((yb + 1) * h.spec_width() <= h.spec_n())
                                by (nonlinear_arith)
                                requires 0 <= yb < h.spec_height(), h.spec_width() >= 1,
                                    h.spec_n() == h.spec_width() * h.spec_height();
                            assert(painted as int <= yb * h.spec_width() + xb);
                            assert(steps@ as int - sb as int - 1 + painted as int <= h.spec_n())
                                by (nonlinear_arith)
                                requires
                                    steps@ as int <= sb as int + 1 + h.spec_width() - xb,
                                    painted as int <= yb * h.spec_width() + xb,
                                    (yb + 1) * h.spec_width() <= h.spec_n();
                            assert(sb == s_start + commands + painted);
                            assert(steps@ as int <= s_start as int + commands as int + 1 + h.spec_n());
                        }
                    }
                    assert(crate::model::mr_gcur(b@, m, p_start, x_start, y_start)
                        == Err::<(int, int, int), (crate::model::RleErr, int, int, int)>((crate::model::RleErr::Api(e), *pos as int, *x as int, *y as int)));
                    assert(gw0 + crate::model::mr_writes(b@, m, p_start, x_start, y_start) == gw);
                    crate::model::lemma_mr_gcur_proj(b@, m, p_start, x_start, y_start);
                }
                return (Err(e), Ghost(gw));
            },
        }
    }
}

/// T7 on the whole image: `rle_exact_from` at the public start state
/// `(pixel_start, 0, 0)` with an empty accumulator, so the buffer is `MR`'s
/// write list replayed onto the palette-zero fill.
spec fn rle_exact(
    b: Seq<u8>,
    m: crate::model::MHRecord,
    p0: int,
    gc: (int, int, int),
    out: Seq<u8>,
    r: Result<usize, Error>,
) -> bool {
    match crate::model::mr(b, m, p0, 0, 0) {
        Ok((pc, xc, yc, ws)) => {
            &&& (match r { Ok(p) => p as int == pc, Err(_) => false })
            &&& gc.0 == pc && gc.1 == xc && gc.2 == yc
            &&& out == crate::model::bytes_replay(crate::model::pal_zero_bytes(b, m), ws)
        },
        Err((e, pc, xc, yc, ws)) => {
            &&& (match r { Ok(_) => false, Err(e2) => e == crate::model::RleErr::Api(e2) })
            &&& gc.0 == pc && gc.1 == xc && gc.2 == yc
            &&& out == crate::model::bytes_replay(crate::model::pal_zero_bytes(b, m), ws)
        },
    }
}

/// T7: the RLE4/RLE8 helper on the public path — the R8 palette-zero fill,
/// then `rle_loop` from the checked pixel cursor `(pixel_start, 0, 0)`. On
/// both exits the returned result, the ghost final state `*gcur` and the buffer
/// are `MR`'s (`rle_exact`).
fn decode_rle(
    b: &[u8],
    h: &Header,
    out: &mut Vec<u8>,
    gcur: &mut (usize, usize, usize),
    steps: &mut Ghost<nat>,
) -> (r: Result<usize, Error>)
    requires
        header_inv(b@, *h), header_inv_light(h),
        old(out)@.len() == 4 * h.spec_n(),
        h.compression == 1 || h.compression == 2,
    ensures
        final(steps)@ as int <= old(steps)@ as int + h.spec_n()
            + (b@.len() - h.pixel_start as int) / 2 + 1 + h.spec_n(),
        final(out)@.len() == 4 * h.spec_n(),
        match r { Ok(p) => h.pixel_start <= p <= b@.len(), Err(_) => true },
        h.pixel_start <= final(gcur).0 <= b@.len(),
        rle_exact(
            b@, hrec(h), h.pixel_start as int,
            (final(gcur).0 as int, final(gcur).1 as int, final(gcur).2 as int),
            final(out)@, r,
        ),
{
    fill_rle(b, h, out, steps);
    let ghost m = hrec(h);
    let mut pos = h.pixel_start;
    let mut x: usize = 0;
    let mut y: usize = 0;
    proof {
        reveal(header_inv);
        reveal(crate::model::bytes_replay);
        assert(out@ == crate::model::bytes_replay(
            crate::model::pal_zero_bytes(b@, m), Seq::<crate::model::Write>::empty()));
    }
    let (r, g) = rle_loop(b, h, out, &mut pos, &mut x, &mut y, Ghost(Seq::empty()), steps);
    proof {
        reveal(rle_exact);
        reveal(rle_exact_from);
        broadcast use vstd::seq_lib::group_seq_lib_default;
        match crate::model::mr(b@, m, h.pixel_start as int, 0, 0) {
            Ok((pc, xc, yc, ws)) => {
                assert(g@ == Seq::<crate::model::Write>::empty() + ws);
                assert(g@ == ws);
            },
            Err((e, pc, xc, yc, ws)) => {
                assert(g@ == Seq::<crate::model::Write>::empty() + ws);
                assert(g@ == ws);
            },
        }
    }
    gcur.0 = pos;
    gcur.1 = x;
    gcur.2 = y;
    match r {
        Ok(()) => Ok(pos),
        Err(e) => Err(e),
    }
}

fn alpha_present(h: &Header, out: &Vec<u8>, steps: &mut Ghost<nat>) -> (r: bool)
    requires
        header_inv_light(h),
        out@.len() == 4 * h.spec_n(),
    ensures
        r == crate::model::xor_alpha_present(out@, h.spec_n()),
        old(steps)@ <= final(steps)@ <= old(steps)@ + h.spec_n(),
{
    let mut i: usize = 0;
    let ghost s0: nat = steps@;
    while i < h.n
        invariant
            s0 == old(steps)@, steps@ == s0 + i,
            header_inv_light(h), out@.len() == 4 * h.spec_n(), i <= h.n,
            forall|j: int| 0 <= j < i ==> #[trigger] out@[4 * j + 3] == 0u8,
        decreases h.n - i,
    {
        proof { *steps = Ghost(steps@ + 1); }
        if out[4 * i + 3] != 0 {
            proof {
                reveal(crate::model::xor_alpha_present);
                assert(exists|j: int| 0 <= j < h.spec_n() && #[trigger] out@[4 * j + 3] != 0u8);
            }
            return true;
        }
        i += 1;
    }
    proof {
        reveal(crate::model::xor_alpha_present);
        assert(!crate::model::xor_alpha_present(out@, h.spec_n())) by {
            if crate::model::xor_alpha_present(out@, h.spec_n()) {
                let j = choose|j: int| 0 <= j < h.spec_n() && #[trigger] out@[4 * j + 3] != 0u8;
                assert(out@[4 * j + 3] == 0u8);
            }
        }
    }
    false
}

fn apply_and(b: &[u8], h: &Header, out: &mut Vec<u8>, pos: usize,
    mask_stride: usize, and_bytes: usize, has_xor_alpha: bool,
    steps: &mut Ghost<nat>)
    requires
        header_inv_light(h),
        old(out)@.len() == 4 * h.spec_n(),
        pos <= b@.len(),
        pos + and_bytes <= b@.len(),
        b@.len() <= 67108864,
        and_bytes == mask_stride * (h.height as usize),
        mask_stride as int == crate::model::stride_of(h.spec_width(), 1),
        is_ico(h.mode),
        !h.spec_top_down(),
        has_xor_alpha ==> !crate::model::xor_alpha_present(old(out)@, h.spec_n()),
    ensures
        final(steps)@ == old(steps)@ + h.spec_n() + h.spec_height(),
        final(out)@.len() == 4 * h.spec_n(),
        final(out)@ == crate::model::ma_output(
            b@, hrec(h), pos as int, old(out)@, has_xor_alpha),
{
    let ghost original = old(out)@;
    proof {
        lemma_hrec_dims(h);
        lemma_hrec_top_down(h);
        crate::model::lemma_ma_prefix_ends(b@, hrec(h), pos as int, original, has_xor_alpha);
        assert(out@ == crate::model::ma_prefix(
            b@, hrec(h), pos as int, original, has_xor_alpha, 0));
    }
    proof {
        reveal(header_inv_light);
        crate::model::lemma_stride_bound(h.spec_width(), 1);
        assert(mask_stride <= 32768);
    }
    let mut y: usize = 0;
    let ghost s0: nat = steps@;
    while y < h.height as usize
        invariant
            s0 == old(steps)@,
            steps@ as int == s0 as int + (y as int) * (h.spec_width() + 1),
            header_inv_light(h), out@.len() == 4 * h.spec_n(),
            pos + and_bytes <= b@.len(),
            b@.len() <= 67108864,
            and_bytes == mask_stride * (h.height as usize),
            mask_stride <= 32768,
            mask_stride as int == crate::model::stride_of(h.spec_width(), 1),
            y <= h.spec_height(),
            is_ico(h.mode), !h.spec_top_down(),
            original.len() == 4 * h.spec_n(),
            has_xor_alpha ==> !crate::model::xor_alpha_present(original, h.spec_n()),
            out@ == crate::model::ma_prefix(b@, hrec(h), pos as int, original,
                has_xor_alpha, (y as int) * h.spec_width()),
        decreases h.height as usize - y,
    {
        proof { *steps = Ghost(steps@ + 1); }
        proof {
            assert((y + 1) * mask_stride <= and_bytes) by (nonlinear_arith)
                requires
                    and_bytes == mask_stride * (h.height as usize),
                    y + 1 <= h.height as usize;
            assert((y as int) * (mask_stride as int) <= and_bytes as int) by (nonlinear_arith)
                requires (y + 1) * mask_stride <= and_bytes, mask_stride >= 0;
            assert((((y as usize) * mask_stride) as int) == (y as int) * (mask_stride as int))
                by (nonlinear_arith)
                requires
                    ((y as int) * (mask_stride as int)) <= and_bytes as int,
                    and_bytes <= b@.len(),
                    b@.len() <= 67108864;
        }
        let row = pos + y * mask_stride;
        proof {
            assert(row + mask_stride <= pos + and_bytes) by (nonlinear_arith)
                requires
                    row == pos + y * mask_stride,
                    (y + 1) * mask_stride <= and_bytes,
            ;
            assert(row + mask_stride <= b@.len()) by (nonlinear_arith)
                requires row + mask_stride <= pos + and_bytes, pos + and_bytes <= b@.len();
            assert(row + mask_stride <= usize::MAX);
        }
        let mut x: usize = 0;
        while x < h.width as usize
            invariant
                s0 == old(steps)@,
                steps@ as int == s0 as int + (y as int) * (h.spec_width() + 1) + 1 + x as int,
                header_inv_light(h), out@.len() == 4 * h.spec_n(),
                pos + and_bytes <= b@.len(),
                b@.len() <= 67108864,
                mask_stride <= 32768,
                and_bytes == mask_stride * (h.height as usize),
                mask_stride as int == crate::model::stride_of(h.spec_width(), 1),
                y < h.spec_height(), x <= h.spec_width(),
                row == pos + y * mask_stride,
                row + mask_stride <= pos + and_bytes,
                row + mask_stride <= b@.len(),
                is_ico(h.mode), !h.spec_top_down(),
                original.len() == 4 * h.spec_n(),
                has_xor_alpha ==> !crate::model::xor_alpha_present(original, h.spec_n()),
                out@ == crate::model::ma_prefix(b@, hrec(h), pos as int, original,
                    has_xor_alpha, (y as int) * h.spec_width() + (x as int)),
            decreases h.width as usize - x,
        {
            proof {
                assert(mask_stride >= 1) by (nonlinear_arith)
                    requires mask_stride as int == crate::model::stride_of(h.spec_width(), 1),
                        h.spec_width() >= 1;
                crate::model::and_bit_offset_bound(h.spec_width(), x as int);
                reveal(crate::model::stride_of);
                assert(x / 8 + 1 <= mask_stride);
                assert(row + x / 8 + 1 <= row + mask_stride) by (nonlinear_arith)
                    requires x / 8 + 1 <= mask_stride;
                assert(row + x / 8 + 1 <= b@.len()) by (nonlinear_arith)
                    requires row + x / 8 + 1 <= row + mask_stride, row + mask_stride <= b@.len();
                assert(row + x / 8 < b@.len());
            }
            let raw_bit = rd(b, row + x / 8);
            let bit = raw_bit & (0x80u8 >> (x % 8));
            let o = out_offset(h, x, y) + 3;
            proof {
                lemma_hrec_dims(h);
                lemma_hrec_top_down(h);
                reveal(crate::model::and_bit);
                reveal(crate::model::and_stride);
                reveal(crate::model::cell_of);
                assert(hrec(h).top_down == false);
                assert((o as int) == 4 * ((h.spec_height() - 1 - y as int)
                    * h.spec_width() + x as int) + 3);
                assert((row + x / 8) as int == (pos as int) + (y as int)
                    * mask_stride as int + (x as int) / 8) by (nonlinear_arith)
                    requires row == pos + y * mask_stride;
                assert((bit != 0) == crate::model::and_bit(
                    b@, pos as int, h.spec_width(), x as int, y as int));
            }
            let ghost before = out@;
            if bit != 0 { put_alpha(out, o, 0); }
            else if has_xor_alpha { put_alpha(out, o, 255); }
            proof {
                let m = hrec(h);
                let k = o as int;
                let done = (y as int) * h.spec_width() + (x as int);
                crate::model::lemma_ma_prefix_step(
                    b@, m, pos as int, original, has_xor_alpha, x as int, y as int);
                reveal(crate::model::ma_byte);
                reveal(crate::model::ma_prefix);
                reveal(crate::model::ma_source);
                crate::model::lemma_ma_source_cell(m, x as int, y as int);
                assert(crate::model::ma_source(m, k) == done);
                assert(k % 4 == 3);
                assert((k / 4) % m.width == x as int);
                assert(m.height - 1 - (k / 4) / m.width == y as int);
                assert(crate::model::and_bit(b@, pos as int, m.width,
                    (k / 4) % m.width, m.height - 1 - (k / 4) / m.width) == (bit != 0));
                assert(!has_xor_alpha || !crate::model::xor_alpha_present(original, m.n));
                if bit == 0 && !has_xor_alpha {
                    assert(before[k] == original[k]);
                    vstd::seq::lemma_seq_update_same(before, k, original[k]);
                    assert(out@ == before.update(k, original[k]));
                }
                assert(out@ == before.update(k,
                    crate::model::ma_byte(b@, m, pos as int, original, has_xor_alpha, k)));
                assert(out@ == crate::model::ma_prefix(
                    b@, m, pos as int, original, has_xor_alpha, done + 1));
            }
            x += 1;
            proof { *steps = Ghost(steps@ + 1); }
        }
        proof {
            assert(x == h.width as usize);
            assert((y as int) * h.spec_width() + (x as int)
                == ((y + 1) as int) * h.spec_width())
                by (nonlinear_arith)
                requires x as int == h.spec_width();
        }
        y += 1;
        proof {
            assert((y as int) * (h.spec_width() + 1)
                == ((y - 1) as int) * (h.spec_width() + 1) + 1 + h.spec_width())
                by (nonlinear_arith)
                requires y >= 1, h.spec_width() >= 1;
        }
    }
    proof {
        lemma_hrec_dims(h);
        assert(y as int == h.spec_height());
        assert(steps@ as int == s0 as int + h.spec_n() + h.spec_height())
            by (nonlinear_arith)
            requires steps@ as int == s0 as int + (y as int) * (h.spec_width() + 1),
                y as int == h.spec_height(),
                h.spec_n() == h.spec_width() * h.spec_height();
        assert(steps@ == old(steps)@ + h.spec_n() + h.spec_height());
        assert((y as int) * h.spec_width() == hrec(h).n)
            by (nonlinear_arith)
            requires hrec(h).n == h.spec_width() * h.spec_height(),
                y as int == h.spec_height();
        crate::model::lemma_ma_prefix_ends(b@, hrec(h), pos as int, original, has_xor_alpha);
        assert(out@ == crate::model::ma_output(
            b@, hrec(h), pos as int, original, has_xor_alpha));
    }
}

/// T9/T8: the concrete mask/suffix helper equals the model's `mp_mask_phase` as
/// the exact `Result` in **every** mode — the R12 image-size check, then ICO mask
/// availability and the whole-image XOR-alpha/AND policy, then the
/// `BareDib`/`IcoEntryDib` trailing-byte closure (`BmpFile` ignores a suffix).
/// On the success branch the buffer is the model's exact image.
spec fn mask_done_eq(
    b: Seq<u8>,
    m: crate::model::MHRecord,
    pos: int,
    out0: Seq<u8>,
    out1: Seq<u8>,
    r: Result<(), Error>,
) -> bool {
    let a = crate::model::and_bytes(m.width, m.height);
    if !crate::model::mp_image_size_ok(m, pos - m.pixel_start) {
        r == Err(Error::InvalidImageSize)
    } else if is_ico(m.mode) {
        if pos + a > b.len() {
            r == Err(Error::TruncatedAndMask)
        } else if exact_tail(m.mode) && pos + a != b.len() {
            r == Err(Error::TrailingBytes)
        } else {
            r == Ok(()) && out1 == crate::model::ma_output(
                b,
                m,
                pos,
                out0,
                crate::model::explicit_alpha(m),
            )
        }
    } else if m.mode == Mode::BmpFile {
        r == Ok(()) && out1 == out0
    } else if pos != b.len() {
        r == Err(Error::TrailingBytes)
    } else {
        r == Ok(()) && out1 == out0
    }
}

/// T9: the public `decode` result equals `MP(b, mode)` as the exact `Result`,
/// including every output byte — the manifest's T9 statement.
pub open spec fn decode_mp_eq(r: Result<Decoded, Error>, b: Seq<u8>, mode: Mode) -> bool {
    match r {
        Ok(d) => crate::model::mp(b, mode) == Ok((d.width as int, d.height as int, d.rgba@)),
        Err(e) => crate::model::mp(b, mode) == Err(e),
    }
}

/// `MP`'s R7 palette-fit test passes for a `header_inv` record that `MH` returns
/// and `hdr_matches` ties to the executable's `Header`.
proof fn lemma_mp_palette_fits(b: Seq<u8>, mode: Mode, m: crate::model::MHRecord, h: &Header)
    requires
        crate::model::mh(b, mode) == Ok(m),
        hdr_matches(m, *h),
        header_inv(b, *h),
    ensures
        m.palette_start + m.table_count * m.palette_stride <= b.len(),
{
    crate::model::lemma_mh_palette_end(b, mode, m);
    reveal(hdr_matches);
    reveal(header_inv);
    assert(m.palette_end == h.spec_palette_end());
    assert(h.spec_palette_end() == h.palette_end as int);
    assert(m.palette_start + m.table_count * m.palette_stride == m.palette_end);
    assert(m.palette_end <= b.len());
}

/// `hdr_matches` fixes every field of `header_record(h)`, so the record the
/// executable's `Header` denotes is the model record `m` itself.
proof fn lemma_hrec_matches(m: crate::model::MHRecord, h: &Header)
    requires
        hdr_matches(m, *h),
    ensures
        hrec(h) == m,
{
    reveal(hrec);
    reveal(header_record);
    reveal(hdr_matches);
    assert(hrec(h) == m);
}

/// `finish_mask`'s `mask_done_eq` on `Ok` names the mask phase's `Ok` payload.
proof fn lemma_mask_done_ok(
    b: Seq<u8>,
    m: crate::model::MHRecord,
    pos: int,
    out0: Seq<u8>,
    out1: Seq<u8>,
)
    requires
        mask_done_eq(b, m, pos, out0, out1, Ok(())),
    ensures
        exists|end: int| crate::model::mp_mask_phase(b, m, pos, pos - m.pixel_start, out0)
            == Ok((end, out1)),
{
    reveal(mask_done_eq);
    reveal(crate::dib::is_ico);
    reveal(crate::dib::exact_tail);
    if !crate::model::mp_image_size_ok(m, pos - m.pixel_start) {
        assert(false);
    }
    if is_ico(m.mode) {
        if pos + crate::model::and_bytes(m.width, m.height) > b.len() {
            assert(false);
        }
        assert(exact_tail(m.mode) ==> pos + crate::model::and_bytes(m.width, m.height) == b.len())
            by {
            if exact_tail(m.mode) {
                if pos + crate::model::and_bytes(m.width, m.height) != b.len() {
                    assert(false);
                }
            }
        }
        crate::model::lemma_mp_mask_phase_ico_ok(b, m, pos, out0, out1);
        assert(crate::model::mp_mask_phase(b, m, pos, pos - m.pixel_start, out0)
            == Ok((pos + crate::model::and_bytes(m.width, m.height), out1)));
    } else {
        reveal(crate::model::mp_mask_phase);
        if m.mode == Mode::BareDib {
            if pos != b.len() {
                assert(false);
            }
        }
    }
}

/// T9 (raw path): the concrete raw helper and `finish_mask` results assemble
/// `MP`'s exact `Ok`, given `MP`'s own R1–R7 header (`MH`) and the palette-fit
/// test.
proof fn lemma_mp_raw_assemble(
    b: Seq<u8>,
    mode: Mode,
    m: crate::model::MHRecord,
    h: &Header,
    p: int,
    out0: Seq<u8>,
    out1: Seq<u8>,
)
    requires
        crate::model::mh(b, mode) == Ok(m),
        hdr_matches(m, *h),
        header_inv(b, *h),
        m.compression == 0 || m.compression == 3,
        crate::model::mr_raw_out(b, m, m.pixel_start) == Ok((p, out0)),
        mask_done_eq(b, m, p, out0, out1, Ok(())),
    ensures
        crate::model::mp(b, mode) == Ok((m.width, m.height, out1)),
{
    lemma_mp_palette_fits(b, mode, m, h);
    lemma_mask_done_ok(b, m, p, out0, out1);
    let end = choose|e: int| crate::model::mp_mask_phase(
        b, m, p, p - m.pixel_start, out0) == Ok((e, out1));
    reveal(crate::model::mp);
    reveal(crate::model::mp_raw);
    reveal(crate::model::mp_decoded);
    reveal(crate::model::mh);
    assert(m.palette_start + m.table_count * m.palette_stride <= b.len());
    assert(crate::model::mr_raw_out(b, m, m.pixel_start) == Ok((p, out0)));
    assert(crate::model::mp_raw(b, m) == Ok((out1, end)));
    assert(crate::model::mp_decoded(m, crate::model::mp_raw(b, m)) == Ok((m.width, m.height, out1)));
    assert(crate::model::mp(b, mode) == crate::model::mp_decoded(m, crate::model::mp_raw(b, m)));
}

/// T9 (RLE path): the concrete RLE helper and `finish_mask` results assemble
/// `MP`'s exact `Ok`.
proof fn lemma_mp_rle_assemble(
    b: Seq<u8>,
    mode: Mode,
    m: crate::model::MHRecord,
    h: &Header,
    p: int,
    out0: Seq<u8>,
    out1: Seq<u8>,
)
    requires
        crate::model::mh(b, mode) == Ok(m),
        hdr_matches(m, *h),
        header_inv(b, *h),
        m.compression == 1 || m.compression == 2,
        crate::model::mr_out(b, m, m.pixel_start, 0, 0) == Ok((p, out0)),
        mask_done_eq(b, m, p, out0, out1, Ok(())),
    ensures
        crate::model::mp(b, mode) == Ok((m.width, m.height, out1)),
{
    lemma_mp_palette_fits(b, mode, m, h);
    lemma_mask_done_ok(b, m, p, out0, out1);
    let end = choose|e: int| crate::model::mp_mask_phase(
        b, m, p, p - m.pixel_start, out0) == Ok((e, out1));
    reveal(crate::model::mp);
    reveal(crate::model::mp_rle);
    reveal(crate::model::mp_decoded);
    reveal(crate::model::mh);
    assert(m.palette_start + m.table_count * m.palette_stride <= b.len());
    assert(crate::model::mr_out(b, m, m.pixel_start, 0, 0) == Ok((p, out0)));
    assert(crate::model::mp_rle(b, m) == Ok((out1, end)));
    assert(crate::model::mp_decoded(m, crate::model::mp_rle(b, m)) == Ok((m.width, m.height, out1)));
    assert(crate::model::mp(b, mode) == crate::model::mp_decoded(m, crate::model::mp_rle(b, m)));
}

/// T9/T8 bridge: on the ICO success branch, the model's `ma` (finish_mask's
/// contract) and the mask phase `mp_mask_phase` agree on `Ok` payloads when the
/// AND plane is available and, in the exact-tail mode, closes the slice — `ma`'s
/// `Ok` image is `ma_output`, the same byte sequence `mp_mask_phase` returns on
/// its own success branch. R19: holds for both ICO modes; the slack mode may
/// have `end < b.len()`.
proof fn lemma_ma_mp_mask_phase(
    b: Seq<u8>,
    m: crate::model::MHRecord,
    pos: int,
    x_len: int,
    out: Seq<u8>,
    end: int,
    image: Seq<u8>,
)
    requires
        is_ico(m.mode),
        x_len == pos - m.pixel_start,
        pos + crate::model::and_bytes(m.width, m.height) <= b.len(),
        exact_tail(m.mode) ==> pos + crate::model::and_bytes(m.width, m.height) == b.len(),
        crate::model::ma(b, m, pos, out) == Ok((end, image)),
    ensures
        crate::model::mp_mask_phase(b, m, pos, x_len, out) == Ok((end, image)),
{
    reveal(crate::dib::is_ico);
    reveal(crate::dib::exact_tail);
    reveal(crate::model::mp_mask_phase);
    reveal(crate::model::mp_image_size_ok);
    reveal(crate::model::ma);
    reveal(crate::model::ma_exact);
    reveal(crate::model::ma_domain);
    reveal(crate::model::image_size_ok);
    reveal(crate::model::and_bytes);
    reveal(crate::model::and_stride);
    if !crate::model::ma_domain(b, m, pos, out) {
        assert(false);
    }
    if !crate::model::image_size_ok(m.declared_image, x_len,
        crate::model::and_bytes(m.width, m.height), true) {
        assert(false);
    }
    if pos + crate::model::and_bytes(m.width, m.height) > b.len() {
        assert(false);
    }
    assert(crate::model::ma_exact(b, m, pos, out, crate::model::explicit_alpha(m))
        == Ok((end, image)));
    assert(end == pos + crate::model::and_bytes(m.width, m.height));
    assert(image == crate::model::ma_output(
        b, m, pos, out, crate::model::explicit_alpha(m)));
}

fn finish_mask(b: &[u8], h: &Header, out: &mut Vec<u8>, pos: usize,
    steps: &mut Ghost<nat>) -> (r: Result<(), Error>)
    requires
        header_inv_light(h),
        old(out)@.len() == 4 * h.spec_n(),
        h.pixel_start <= pos <= b@.len(),
        b@.len() <= 67108864,
    ensures
        old(steps)@ <= final(steps)@ <= old(steps)@ + 2 * h.spec_n() + h.spec_height(),
        final(out)@.len() == old(out)@.len(),
        match r { Ok(()) => true, Err(e) => e == Error::InvalidImageSize
            || e == Error::TruncatedAndMask || e == Error::TrailingBytes },
        // T9/T8: the helper equals the model's order-0 `mp_mask_phase` exactly,
        // in every mode (R19 included: the slack mode skips only the final
        // cursor-equality test).
        mask_done_eq(b@, hrec(h), pos as int, old(out)@, final(out)@, r),
        // T8: in either ICO mode the helper is `MA` at this reachable complete XOR
        // state — the exact error with the buffer untouched, or `MA`'s image and
        // end cursor (then the R12 `TrailingBytes` closure, exact-tail mode only).
        // `MA`'s `Outside` marker is impossible here: the state is in `ma_domain`.
        is_ico(h.mode) ==> {
            match crate::model::ma(b@, hrec(h), pos as int, old(out)@) {
                Err(crate::model::MaErr::Api(e)) => r == Err(e) && final(out)@ == old(out)@,
                Err(crate::model::MaErr::Outside) => false,
                Ok((end, image)) => final(out)@ == image &&
                    (if end == b@.len() || !exact_tail(h.mode) { r == Ok(()) }
                    else { r == Err(Error::TrailingBytes) }),
            }
        },
        // T8, write-list form: on success with the fallback active, the buffer is
        // `ma_mask`'s AND write list applied to the XOR alpha bytes.
        is_ico(h.mode) ==> {
            match crate::model::ma(b@, hrec(h), pos as int, old(out)@) {
                Ok((end, image)) => {
                    let explicit = crate::model::explicit_alpha(hrec(h));
                    &&& crate::model::ma_mask(b@, hrec(h), pos as int, pos as int - h.pixel_start as int, explicit)
                        == Ok::<Seq<crate::model::Write>, crate::model::MaErr>(
                            crate::model::and_writes(b@, hrec(h), pos as int, 0, 0, explicit))
                    &&& !(explicit && crate::model::xor_alpha_present(old(out)@, hrec(h).n)) ==>
                        final(out)@ == crate::model::alpha_replay(
                            old(out)@, crate::model::and_writes(b@, hrec(h), pos as int, 0, 0, explicit))
                },
                Err(_) => true,
            }
        },
{
    let ghost original = old(out)@;
    proof {
        reveal(hrec);
        reveal(header_record);
        reveal(header_inv_light);
        reveal(crate::model::ma);
        reveal(crate::model::ma_domain);
        reveal(crate::model::explicit_alpha);
        reveal(crate::dib::is_ico);
        reveal(crate::dib::exact_tail);
        if is_ico(h.mode) {
            lemma_hrec_dims(h);
            lemma_hrec_top_down(h);
            assert(crate::model::ma_domain(b@, hrec(h), pos as int, original));
            assert(crate::model::explicit_alpha(hrec(h))
                == (h.bits == 32u32 && (h.compression == 0u32 || h.masks[3] != 0u32)));
            crate::model::lemma_ma_mask_connect(b@, hrec(h), pos as int, original);
        }
    }
    let image_len = pos - h.pixel_start;
    let mask_stride = match h.mode {
        Mode::IcoEntryDib | Mode::IcoEntryDibSlack => stride(h.width, 1),
        _ => 0usize,
    };
    proof {
        reveal(header_inv_light);
        assert(mask_stride <= 32768);
        assert(mask_stride * (h.height as usize) <= 268435456) by (nonlinear_arith)
            requires mask_stride <= 32768, h.height <= 8192;
        assert(mask_stride * (h.height as usize) <= usize::MAX);
    }
    let and_bytes = mask_stride * h.height as usize;
    proof {
        reveal(hrec);
        reveal(header_record);
        reveal(crate::model::ma_exact);
        reveal(crate::model::image_size_ok);
        reveal(crate::model::and_bytes);
        reveal(crate::model::and_stride);
        reveal(crate::dib::is_ico);
        assert(hrec(h).declared_image == h.declared as int);
        assert(hrec(h).pixel_start == h.pixel_start as int);
        if is_ico(h.mode) {
            assert(mask_stride as int == crate::model::stride_of(h.spec_width(), 1));
            assert(hrec(h).width == h.spec_width());
            assert(hrec(h).height == h.spec_height());
            assert((and_bytes as int) == (mask_stride as int) * h.spec_height())
                by (nonlinear_arith)
                requires and_bytes == mask_stride * (h.height as usize);
            assert(crate::model::and_stride(hrec(h).width) == mask_stride as int);
            assert(crate::model::and_bytes(hrec(h).width, hrec(h).height)
                == hrec(h).height * crate::model::and_stride(hrec(h).width));
            assert(and_bytes as int == crate::model::and_bytes(
                hrec(h).width, hrec(h).height)) by (nonlinear_arith)
                requires
                    (and_bytes as int) == (mask_stride as int) * h.spec_height(),
                    hrec(h).height == h.spec_height(),
                    crate::model::and_stride(hrec(h).width) == mask_stride as int,
                    crate::model::and_bytes(hrec(h).width, hrec(h).height)
                        == hrec(h).height * crate::model::and_stride(hrec(h).width);
        }
    }
    proof {
        assert((image_len as u64) + (and_bytes as u64) <= u64::MAX) by (nonlinear_arith)
            requires image_len <= b@.len(), b@.len() <= 67108864,
                and_bytes <= 268435456;
    }
    let ico = match h.mode {
        Mode::IcoEntryDib | Mode::IcoEntryDibSlack => true,
        _ => false,
    };
    if h.declared != 0 {
        let valid = h.declared as u64 == image_len as u64 ||
            (ico && h.declared as u64 == image_len as u64 + and_bytes as u64);
        if !valid {
            proof {
                reveal(crate::dib::is_ico);
                assert(ico == is_ico(h.mode));
                // T9: the model's `mp_mask_phase` fails the same way.
                reveal(mask_done_eq);
                reveal(crate::model::mp_image_size_ok);
                reveal(crate::model::mp_mask_phase);
                reveal(crate::model::image_size_ok);
                reveal(hrec);
                reveal(header_record);
                reveal(crate::model::and_bytes);
                reveal(crate::model::and_stride);
                reveal(header_inv_light);
                reveal(crate::dib::is_ico);
                assert(image_len as int == pos as int - h.pixel_start as int);
                assert(hrec(h).declared_image == h.declared as int);
                assert(hrec(h).pixel_start == h.pixel_start as int);
                assert(hrec(h).mode == h.mode);
                if is_ico(h.mode) {
                    assert((and_bytes as int) == crate::model::and_bytes(
                        hrec(h).width, hrec(h).height)) by (nonlinear_arith)
                        requires
                            and_bytes == mask_stride * h.height as usize,
                            mask_stride as int == crate::model::and_stride(hrec(h).width),
                            hrec(h).height == h.spec_height(),
                            crate::model::and_bytes(hrec(h).width, hrec(h).height)
                                == hrec(h).height * crate::model::and_stride(hrec(h).width);
                }
                assert(!crate::model::mp_image_size_ok(
                    hrec(h), pos as int - h.pixel_start as int));
            }
            return Err(Error::InvalidImageSize);
        }
    }
    proof {
        reveal(mask_done_eq);
        reveal(crate::model::mp_image_size_ok);
        reveal(crate::model::mp_mask_phase);
        reveal(crate::model::image_size_ok);
        reveal(crate::model::and_bytes);
        reveal(crate::model::and_stride);
        reveal(hrec);
        reveal(header_record);
        reveal(header_inv_light);
        reveal(crate::dib::is_ico);
        assert(image_len as int == pos as int - h.pixel_start as int);
        assert(hrec(h).declared_image == h.declared as int);
        assert(hrec(h).pixel_start == h.pixel_start as int);
        assert(hrec(h).mode == h.mode);
        if is_ico(h.mode) {
            assert((and_bytes as int) == crate::model::and_bytes(
                hrec(h).width, hrec(h).height)) by (nonlinear_arith)
                requires
                    and_bytes == mask_stride * h.height as usize,
                    mask_stride as int == crate::model::and_stride(hrec(h).width),
                    hrec(h).height == h.spec_height(),
                    crate::model::and_bytes(hrec(h).width, hrec(h).height)
                        == hrec(h).height * crate::model::and_stride(hrec(h).width);
        }
        assert(crate::model::mp_image_size_ok(hrec(h), pos as int - h.pixel_start as int));
    }
    let ico = match h.mode {
        Mode::IcoEntryDib | Mode::IcoEntryDibSlack => true,
        _ => false,
    };
    if ico {
        if b.len() - pos < and_bytes {
            proof {
                reveal(mask_done_eq);
                reveal(crate::model::mp_mask_phase);
                reveal(crate::model::mp_image_size_ok);
                reveal(crate::dib::is_ico);
                assert(pos + crate::model::and_bytes(hrec(h).width, hrec(h).height)
                    > b@.len());
            }
            return Err(Error::TruncatedAndMask);
        }
        let has_xor_alpha = h.bits == 32 && (h.compression == 0 || h.masks[3] != 0);
        let any_alpha = alpha_present(h, out, steps);
        if !has_xor_alpha || !any_alpha {
            apply_and(b, h, out, pos, mask_stride, and_bytes, has_xor_alpha, steps);
        } else {
            proof {
                crate::model::lemma_ma_explicit_alpha_keep(
                    b@, hrec(h), pos as int, original);
                assert(out@ == original);
            }
        }
        proof {
            reveal(crate::model::ma_exact);
            reveal(crate::model::explicit_alpha);
            assert(out@ == crate::model::ma_output(
                b@, hrec(h), pos as int, original, has_xor_alpha));
        }
        let end = pos + and_bytes;
        let exact = match h.mode {
            Mode::BareDib | Mode::IcoEntryDib => true,
            _ => false,
        };
        if exact && end != b.len() {
            proof {
                reveal(mask_done_eq);
                reveal(crate::model::mp_mask_phase);
                reveal(crate::model::mp_image_size_ok);
                reveal(crate::dib::exact_tail);
                assert(exact == exact_tail(h.mode));
                assert(end as int == pos as int + crate::model::and_bytes(
                    hrec(h).width, hrec(h).height));
                assert(end as int != b@.len());
            }
            return Err(Error::TrailingBytes);
        }
        proof {
            reveal(mask_done_eq);
            reveal(crate::model::mp_mask_phase);
            reveal(crate::model::mp_image_size_ok);
            reveal(crate::model::explicit_alpha);
            reveal(crate::model::ma);
            reveal(crate::model::ma_exact);
            reveal(crate::model::ma_domain);
            reveal(crate::dib::exact_tail);
            assert(end as int == pos as int + crate::model::and_bytes(
                hrec(h).width, hrec(h).height));
            assert(exact ==> end as int == b@.len()) by {
                if exact {
                    assert(exact_tail(h.mode));
                    assert(end as int == b@.len());
                }
            }
            assert(is_ico(hrec(h).mode));
            assert(is_ico(h.mode) == ico) by { reveal(crate::dib::is_ico); }
            // The whole policy is `ma_output`; on the success branch this is
            // `mp_mask_phase`'s own ICO success payload.
            assert(out@ == crate::model::ma_output(
                b@, hrec(h), pos as int, original, crate::model::explicit_alpha(hrec(h))));
            assert(crate::model::ma(b@, hrec(h), pos as int, original) == Ok((
                end as int,
                crate::model::ma_output(
                    b@, hrec(h), pos as int, original,
                    crate::model::explicit_alpha(hrec(h))),
            )));
            lemma_ma_mp_mask_phase(
                b@, hrec(h), pos as int, pos as int - h.pixel_start as int, original,
                end as int, out@);
        }
    } else {
        match h.mode {
            Mode::BareDib => {
                if pos != b.len() {
                    proof {
                        reveal(mask_done_eq);
                        reveal(crate::model::mp_mask_phase);
                        reveal(crate::model::mp_image_size_ok);
                        reveal(hrec);
                        reveal(header_record);
                        reveal(header_inv_light);
                        lemma_hrec_mode(h);
                        assert(hrec(h).mode == Mode::BareDib);
                        assert(pos as int != b@.len());
                    }
                    return Err(Error::TrailingBytes);
                }
                proof {
                    reveal(mask_done_eq);
                    reveal(crate::model::mp_mask_phase);
                    reveal(crate::model::mp_image_size_ok);
                    reveal(hrec);
                    reveal(header_record);
                    reveal(header_inv_light);
                    lemma_hrec_mode(h);
                    assert(hrec(h).mode == Mode::BareDib);
                    assert(pos as int == b@.len());
                    assert(final(out)@ == original);
                }
            },
            Mode::BmpFile => {
                proof {
                    reveal(mask_done_eq);
                    reveal(crate::model::mp_mask_phase);
                    reveal(crate::model::mp_image_size_ok);
                    reveal(hrec);
                    reveal(header_record);
                    reveal(header_inv_light);
                    lemma_hrec_mode(h);
                    assert(hrec(h).mode == Mode::BmpFile);
                    assert(final(out)@ == original);
                }
            },
            Mode::IcoEntryDib | Mode::IcoEntryDibSlack => {},
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// M3 statement witnesses: the manifest's T4, T5 and T11 claims, at their own
// quantification and on the actual helpers.
// ---------------------------------------------------------------------------

/// T4 · conversion, for the raw/BITFIELDS helper at the manifest's
/// quantification. For every `header_inv`/`header_inv_light` record, every
/// `x < width` and every row position with the whole checked stride available,
/// the concrete `raw_sample` returns exactly the model `raw_conv`, which
/// `model::lemma_raw_conv_eq_conv` has proved is the manifest's `Conv` at the
/// sample's own word or palette index. The `colour` helper is exact against
/// `pal_entry` by its postcondition.
#[allow(dead_code)]
proof fn t4_conversion(b: Seq<u8>, h: Header)
    requires
        header_inv(b, h),
        header_inv_light(&h),
        (h.compression == 0 || h.compression == 3),
    ensures
        crate::model::mh_bounds(header_record(h)),
        // every palette read of a validated index stays inside the borrowed
        // palette slice
        forall|ix: int|
            #![trigger crate::model::pal_entry(b, h.spec_palette_start(), h.spec_pal_stride(), ix)]
            0 <= ix < h.spec_count() ==> h.spec_palette_start() + ix * h.spec_pal_stride() + 3 <= b.len(),
{
    reveal(header_inv);
    reveal(header_inv_light);
    assert(crate::model::mh_bounds(header_record(h)));
    assert(h.spec_palette_start() + h.spec_count() * h.spec_pal_stride() <= b.len());
    assert(h.spec_pal_stride() == 3 || h.spec_pal_stride() == 4);
    assert forall|ix: int|
        #![trigger crate::model::pal_entry(b, h.spec_palette_start(), h.spec_pal_stride(), ix)]
        0 <= ix < h.spec_count() implies h.spec_palette_start() + ix * h.spec_pal_stride() + 3 <= b.len() by {
        crate::model::palette_stride_bounds(
            b,
            h.spec_palette_start(),
            h.spec_pal_stride(),
            h.spec_count(),
            ix,
        );
    }
}

/// T4 · the raw helper itself: its postcondition (proved in the body) states the
/// `raw_conv` equality for every `x < width` when a full stride is available.
/// The colour-conversion lemma then identifies `raw_conv` with `Conv`.
#[allow(dead_code)]
fn t4_raw_sample_eq(b: &[u8], h: &Header, pos: usize, x: usize, line: usize) -> (r: Result<[u8; 4], Error>)
    requires
        header_inv(b@, *h),
        header_inv_light(h),
        line as int == crate::model::stride_of(h.spec_width(), h.spec_depth()),
        pos + line <= b@.len(),
        (x as int) < h.spec_width(),
        h.compression == 0 || h.compression == 3,
    ensures
        match r {
            Ok(c) => crate::model::raw_conv(b@, header_record(*h), pos as int, x as int) == c@
                && crate::model::raw_conv(b@, header_record(*h), pos as int, x as int)
                == crate::model::conv(
                    b@,
                    header_record(*h),
                    if h.spec_depth() <= 8 {
                        crate::model::sub_index(
                            b@[(pos as int) + (x as int) * h.spec_depth() / 8],
                            h.spec_depth(),
                            x as int,
                        )
                    } else {
                        crate::model::sample_word(
                            b@,
                            header_record(*h),
                            (pos as int) + (x as int) * h.spec_depth() / 8,
                        )
                    },
                ),
            Err(_) => true,
        },
{
    let res = raw_sample(b, h, pos, x, line);
    proof {
        reveal(header_inv_light);
        reveal(header_record);
        crate::model::lemma_raw_conv_eq_conv(
            b@,
            header_record(*h),
            pos as int,
            x as int,
            line as int,
        );
    }
    res
}

/// T5 · orientation and row arithmetic, on a concrete accepted header:
///  * the stride `4*ceil(width*depth/32)` is four-byte aligned and at most
///    32,768, and covers the last raw sample (`stride_roundup_safe`,
///    `row_last_sample_within_stride`);
///  * each raw sample write is within `4N` (`pixel_byte_offset_bound`);
///  * the bottom-up/top-down map is a bijection `[0, N) -> [0, N)`
///    (`vertical_flip_bijection`);
///  * distinct source cells map to distinct output cells;
///  * the 1-bpp AND stride and MSB-first offset stay inside the checked mask
///    (`and_bit_offset_bound`).
#[allow(dead_code)]
proof fn t5_rows(h: &Header, x1: usize, r1: usize, x2: usize, r2: usize, q: usize)
    requires
        header_inv_light(h),
        1 <= h.spec_depth() <= 32,
        (x1 as int) < h.spec_width(),
        (r1 as int) < h.spec_height(),
        (x2 as int) < h.spec_width(),
        (r2 as int) < h.spec_height(),
        (q as int) < h.spec_n(),
    ensures
        // stride
        crate::model::stride_of(h.spec_width(), h.spec_depth()) == ((h.spec_width() * h.spec_depth()
            + 31) / 32) * 4,
        0 <= crate::model::stride_of(h.spec_width(), h.spec_depth()) <= 32768,
        crate::model::stride_of(h.spec_width(), h.spec_depth()) % 4 == 0,
        // the last sample of column x1 stays inside the stride
        h.spec_depth() % 8 == 0 ==> (x1 as int) * (h.spec_depth() / 8) + h.spec_depth() / 8
            <= crate::model::stride_of(h.spec_width(), h.spec_depth()),
        // every oriented cell lies inside [0, N)
        0 <= crate::model::orient_cell(
            h.spec_width(),
            h.spec_height(),
            h.spec_top_down(),
            x1 as int,
            r1 as int,
        ) < h.spec_n(),
        4 * crate::model::orient_cell(
            h.spec_width(),
            h.spec_height(),
            h.spec_top_down(),
            x1 as int,
            r1 as int,
        ) + 4 <= 4 * h.spec_n(),
        // the map is a bijection: injective and surjective
        (crate::model::orient_cell(
            h.spec_width(),
            h.spec_height(),
            h.spec_top_down(),
            x1 as int,
            r1 as int,
        ) == crate::model::orient_cell(
            h.spec_width(),
            h.spec_height(),
            h.spec_top_down(),
            x2 as int,
            r2 as int,
        )) <==> (x1 as int == x2 as int && r1 as int == r2 as int),
        exists|x: int, r: int|
            0 <= x < h.spec_width() && 0 <= r < h.spec_height() && crate::model::orient_cell(
                h.spec_width(),
                h.spec_height(),
                h.spec_top_down(),
                x,
                r,
            ) == q,
        // the 1-bpp AND MSB-first bit index stays inside the checked stride
        x1 / 8 + 1 <= crate::model::stride_of(h.spec_width(), 1),
{
    reveal(header_inv_light);
    crate::model::stride_roundup_safe(h.spec_width(), h.spec_depth());
    if h.spec_depth() % 8 == 0 {
        crate::model::row_last_sample_within_stride(h.spec_width(), h.spec_depth(), x1 as int);
    }
    crate::model::pixel_byte_offset_bound(
        h.spec_width(),
        h.spec_height(),
        h.spec_top_down(),
        x1 as int,
        r1 as int,
    );
    crate::model::vertical_flip_bijection(
        h.spec_width(),
        h.spec_height(),
        h.spec_top_down(),
        x1 as int,
        r1 as int,
        x2 as int,
        r2 as int,
        q as int,
    );
    crate::model::and_bit_offset_bound(h.spec_width(), x1 as int);
}

/// T11 · termination, stated on the two concrete decoders:
///  * the raw loop's variant is `height - y` for the row loop and `width - x`
///    for the column loop (both `decreases`);
///  * the RLE command loop's variant is `|b| - pos` and each successful command
///    consumes at least two bytes, so it strictly decreases and EOB returns;
///  * the two `<= 255` inner parsers use `q - j`/`quantity - j`;
///  * an incomplete command is rejected before any payload byte is read
///    (`lemma_rle_cmd_no_retry`).
#[allow(dead_code)]
proof fn t11_termination(h: &Header, p: int, q: int, j: int)
    requires
        header_inv_light(h),
        0 <= p,
        h.compression == 1 || h.compression == 2,
        0 <= j <= q <= 255,
    ensures
        // the nested emitted-pixel variant strictly decreases
        j < q ==> q - (j + 1) < q - j,
        q - j >= 0,
{
    reveal(header_inv_light);
    crate::model::lemma_rle_paint_terminates(q, j);
}

/// T11 · the model-level RLE command loop: the variant `|b| - cursor` strictly
/// decreases because each complete command consumes >= 2 bytes, so the number of
/// commands is at most `(|b| - pixel_start)/2` and an EOB returns.
#[allow(dead_code)]
proof fn t11_rle_cmd_model(b: Seq<u8>, depth: int, p: int)
    requires
        depth == 4 || depth == 8,
        0 <= p <= b.len(),
    ensures
        p + 2 * crate::model::rle_cmd_count(b, depth, p) <= b.len(),
        crate::model::rle_cmd_count(b, depth, p) <= (b.len() - p) / 2,
{
    crate::model::lemma_rle_cmd_count_bound(b, depth, p);
}

/// T11 · the concrete decoders terminate on every accepted header. The witness
/// runs the actual `decode_raw`/`decode_rle` helpers (whose loops carry the
/// `decreases` clauses `height - y`, `width - x`, `|b| - pos`) on a validated
/// record and returns their cursor; no `requires` permits a missing byte, an
/// incomplete command or an out-of-range index, so each loop's variant strictly
/// decreases and the call returns.
#[allow(dead_code)]
fn t11_decode_terminates(b: &[u8], h: &Header, out: &mut Vec<u8>) -> (r: Result<usize, Error>)
    requires
        header_inv(b@, *h),
        header_inv_light(h),
        old(out)@.len() == 4 * h.spec_n(),
        old(out)@ == crate::model::zero_bytes(4 * h.spec_n()),
        h.compression <= 3,
    ensures
        final(out)@.len() == 4 * h.spec_n(),
        match r { Ok(p) => h.pixel_start <= p <= b@.len(), Err(_) => true },
{
    if h.compression == 0 || h.compression == 3 {
        let mut steps = Ghost(0nat);
        decode_raw(b, h, out, &mut steps)
    } else {
        proof {
            assert(h.compression == 1u32 || h.compression == 2u32);
        }
        let mut gcur: (usize, usize, usize) = (h.pixel_start, 0, 0);
        let mut steps = Ghost(0nat);
        decode_rle(b, h, out, &mut gcur, &mut steps)
    }
}

/// N is zero on a failed header, and the checked visible pixel count otherwise.
/// Public because `decode_with_steps`'s exported T12 bound states it.
pub open spec fn t12_checked_n(b: Seq<u8>, mode: Mode) -> int {
    match crate::model::mh(b, mode) {
        Ok(m) => m.n,
        Err(_) => 0,
    }
}

proof fn lemma_t12_phase_budget(l: int, n: int, h: int)
    requires 0 <= l, 1 <= n, 0 <= h <= n,
    ensures
        4*n + (n+h+1) + (2*n+h) <= 16*(l+n),
        4*n + (2*n + l/2 + 1) + (2*n+h) <= 16*(l+n),
{
    assert(l/2 <= l) by (nonlinear_arith) requires 0 <= l;
}

/// T1: all inputs and all three modes; no precondition on the public API.
/// T2: the actual first call is the MH-exact parser, not a parallel parser.
/// T3: the only allocating call is reached once after that parser succeeds.
fn decode_tracked(bytes: &[u8], mode: Mode, steps: &mut Ghost<nat>) -> (r: Result<Decoded, Error>)
    ensures
        final(steps)@ as int <= old(steps)@ as int
            + 16 * (bytes@.len() + t12_checked_n(bytes@, mode)),
        // T13: every success has the exact `4*w*h` output, within the caps.
        match r {
            Ok(d) => 1 <= d.width as int <= 8192 && 1 <= d.height as int <= 8192
                && d.rgba@.len() == 4 * (d.width as int * d.height as int)
                && d.rgba@.len() <= 67108864,
            Err(_) => true,
        },
        // T9: `decode(bytes, mode)` equals `MP(bytes, mode)` as the exact
        // `Result`, including every output byte, for every slice and mode.
        decode_mp_eq(r, bytes@, mode),
        match crate::model::mh(bytes@, mode) {
            Err(e) => r == Err(e),
            Ok(_) => true,
        },
{
    let ghost start_steps: nat = steps@;
    let ghost mut alloc_count: nat = 0;
    let h = match parse_header(bytes, mode) {
        Ok(h) => h,
        Err(e) => {
            proof { assert(alloc_count == 0); }
            return Err(e);
        },
    };
    proof {
        // T9/T10: the header error half — the record `parse_header` returns is
        // `MH`'s, so a failure here fails `MP` with the same error.
        reveal(decode_mp_eq);
        match crate::model::mh(bytes@, mode) {
            Ok(_) => {},
            Err(e) => { crate::model::lemma_mp_mh_err(bytes@, mode, e); },
        }
    }
    let mut out = alloc_output(&h, steps);
    proof {
        alloc_count = alloc_count + 1;
        assert(0 <= alloc_count <= 1);
    }
    // T9: record `MH`'s `Ok` record, which `parse_header` proves `hdr_matches`.
    let ghost m = choose|m: crate::model::MHRecord| crate::model::mh(bytes@, mode) == Ok(m)
        && hdr_matches(m, h);
    proof {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes@, mode) == Ok(m));
        assert(hdr_matches(m, h));
        lemma_hrec_matches(m, &h);
        assert(hrec(&h) == m);
        assert(m.pixel_start == h.spec_pixel_start());
        assert(m.compression == h.spec_compression());
        assert(t12_checked_n(bytes@, mode) == h.spec_n());
        assert(steps@ == start_steps + 4 * h.spec_n());
        reveal(header_inv_light);
        assert(h.spec_height() <= h.spec_n()) by (nonlinear_arith)
            requires 1 <= h.spec_width(), 1 <= h.spec_height(),
                h.spec_n() == h.spec_width() * h.spec_height();
        lemma_t12_phase_budget(bytes@.len() as int, h.spec_n(), h.spec_height());
    }
    let mut gcur: (usize, usize, usize) = (h.pixel_start, 0, 0);
    let raw = h.compression == 0 || h.compression == 3;
    let p = if raw {
        decode_raw(bytes, &h, &mut out, steps)
    } else {
        decode_rle(bytes, &h, &mut out, &mut gcur, steps)
    };
    proof {
        assert((bytes@.len() - h.pixel_start as int) / 2 <= bytes@.len() / 2)
            by (nonlinear_arith)
            requires 0 <= h.pixel_start <= bytes@.len();
        if raw {
            assert(steps@ as int <= start_steps as int + 4*h.spec_n()
                + (h.spec_n() + h.spec_height() + 1));
        } else {
            assert(steps@ as int <= start_steps as int + 4*h.spec_n()
                + (2*h.spec_n() + bytes@.len()/2 + 1));
        }
        assert(steps@ as int <= start_steps as int
            + 16*(bytes@.len() + t12_checked_n(bytes@, mode)));
    }
    let ghost after_pixels: nat = steps@;
    let pos = match p {
        Ok(pos) => pos,
        Err(e) => {
            proof {
                // T9/T10, raw or RLE phase error.
                reveal(decode_mp_eq);
                if raw {
                    reveal(raw_done_eq);
                    reveal(crate::model::mr_raw_out);
                    crate::model::lemma_raw_full_proj(
                        bytes@, hrec(&h), h.pixel_start as int, 0, 0);
                    assert(crate::model::mr_raw_out(bytes@, m, m.pixel_start) is Err);
                    assert((crate::model::mr_raw_out(bytes@, m, m.pixel_start)->Err_0).0
                        == crate::model::RawErr::Api(e));
                    crate::model::lemma_mp_raw_err(bytes@, mode, m, e);
                } else {
                    reveal(rle_exact);
                    reveal(crate::model::mr_out);
                    assert(crate::model::mr_out(bytes@, m, m.pixel_start, 0, 0) is Err);
                    assert((crate::model::mr_out(bytes@, m, m.pixel_start, 0, 0)->Err_0).0
                        == crate::model::RleErr::Api(e));
                    crate::model::lemma_mp_rle_err(bytes@, mode, m, e);
                }
            }
            return Err(e);
        },
    };
    let ghost out_xor = out@;
    let mask_result = finish_mask(bytes, &h, &mut out, pos, steps);
    proof {
        assert(steps@ as int <= after_pixels as int + 2*h.spec_n() + h.spec_height());
        assert(steps@ as int <= start_steps as int
            + 16*(bytes@.len() + t12_checked_n(bytes@, mode)));
    }
    match mask_result {
        Ok(()) => {},
        Err(e) => {
            proof {
                // T9/T10, mask/suffix phase error.
                reveal(decode_mp_eq);
                reveal(mask_done_eq);
                assert(mask_done_eq(bytes@, m, pos as int, out_xor, out@, Err(e)));
                if e == Error::InvalidImageSize {
                    crate::model::lemma_mp_mask_phase_size_err(
                        bytes@, m, pos as int, out_xor);
                    assert(crate::model::mp_mask_phase(
                        bytes@, m, pos as int, pos as int - m.pixel_start, out_xor)
                        == Err(crate::model::MpErr::Api(e)));
                } else {
                    // The remaining errors are the ICO mask/`TrailingBytes`
                    // closures; `MP`'s phase reports them the same way. R19: the
                    // slack mode has no `TrailingBytes` site, so on `is_ico` a
                    // non-truncated failure forces the exact-tail mode.
                    reveal(crate::model::mp_mask_phase);
                    reveal(crate::model::mp_image_size_ok);
                    reveal(crate::dib::is_ico);
                    reveal(crate::dib::exact_tail);
                    assert(crate::model::mp_image_size_ok(
                        m, pos as int - m.pixel_start));
                    assert(m.mode == h.mode);
                    assert(is_ico(m.mode) == (m.mode == Mode::IcoEntryDib
                        || m.mode == Mode::IcoEntryDibSlack)) by { reveal(crate::dib::is_ico); }
                    if m.mode == Mode::IcoEntryDib || m.mode == Mode::IcoEntryDibSlack {
                        if (pos as int) + crate::model::and_bytes(m.width, m.height) > bytes@.len() {
                            assert(crate::model::mp_mask_phase(
                                bytes@, m, pos as int, pos as int - m.pixel_start, out_xor)
                                == Err(crate::model::MpErr::Api(Error::TruncatedAndMask)));
                            assert(e == Error::TruncatedAndMask);
                        } else {
                            assert(exact_tail(m.mode));
                            assert((pos as int) + crate::model::and_bytes(m.width, m.height)
                                < bytes@.len());
                            assert(crate::model::mp_mask_phase(
                                bytes@, m, pos as int, pos as int - m.pixel_start, out_xor)
                                == Err(crate::model::MpErr::Api(Error::TrailingBytes)));
                            assert(e == Error::TrailingBytes);
                        }
                    } else {
                        assert(m.mode == Mode::BareDib);
                        assert(crate::model::mp_mask_phase(
                            bytes@, m, pos as int, pos as int - m.pixel_start, out_xor)
                            == Err(crate::model::MpErr::Api(Error::TrailingBytes)));
                        assert(e == Error::TrailingBytes);
                    }
                }
                if raw {
                    crate::model::lemma_mp_mask_phase_err_raw(
                        bytes@, mode, m, pos as int, out_xor, e);
                } else {
                    crate::model::lemma_mp_mask_phase_err_rle(
                        bytes@, mode, m, pos as int, out_xor, e);
                }
            }
            return Err(e);
        },
    }
    proof { assert(alloc_count == 1); }
    proof {
        // T9: assemble `MP`'s exact `Ok` from the two helpers' results.
        reveal(decode_mp_eq);
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes@, mode) == Ok(m));
        assert(hdr_matches(m, h));
        if raw {
            reveal(raw_done_eq);
            crate::model::lemma_raw_full_proj(bytes@, hrec(&h), h.pixel_start as int, 0, 0);
            assert(crate::model::mr_raw_out(bytes@, hrec(&h), h.pixel_start as int)
                == Ok((pos as int, out_xor)));
            assert(crate::model::mr_raw_out(bytes@, m, m.pixel_start)
                == Ok((pos as int, out_xor)));
            reveal(mask_done_eq);
            lemma_mp_raw_assemble(bytes@, mode, m, &h, pos as int, out_xor, out@);
        } else {
            reveal(rle_exact);
            reveal(crate::model::mr_out);
            assert(crate::model::mr_out(bytes@, hrec(&h), h.pixel_start as int, 0, 0)
                == Ok((pos as int, out_xor)));
            assert(crate::model::mr_out(bytes@, m, m.pixel_start, 0, 0)
                == Ok((pos as int, out_xor)));
            reveal(mask_done_eq);
            lemma_mp_rle_assemble(bytes@, mode, m, &h, pos as int, out_xor, out@);
        }
    }
    Ok(Decoded { width: h.width, height: h.height, rgba: out })
}

/// The public decoder executes the same tracked body with a zeroed ghost counter.
/// The ghost argument and its updates erase; they add no executable operations.
pub fn decode(bytes: &[u8], mode: Mode) -> (r: Result<Decoded, Error>)
    ensures
        decode_mp_eq(r, bytes@, mode),
        match crate::model::mh(bytes@, mode) {
            Err(e) => r == Err(e),
            Ok(_) => true,
        },
        match r {
            Ok(d) => 1 <= d.width as int <= 8192 && 1 <= d.height as int <= 8192
                && d.rgba@.len() == 4 * (d.width as int * d.height as int)
                && d.rgba@.len() <= 67108864,
            Err(_) => true,
        },
{
    let mut steps = Ghost(0nat);
    decode_tracked(bytes, mode, &mut steps)
}

/// The public tracked decoder (R19/T12): the T12 witness body, exported so a
/// caller (the ICO crate) can invoke the same decode path and receive the ghost
/// work counter. The result is `decode(bytes, mode)` and the ghost count `r.1@`
/// is the number of charged concrete steps; the T12 bound
/// `r.1@ <= 16*(|bytes| + N)`, `N = 0` on a rejected header and the validated
/// `w*h` otherwise, and `decode_mp_eq` (T9: the result equals the model `MP`)
/// are its proved postconditions. The ghost return and every counter update
/// erase, so the executable behaviour is byte-for-byte `decode`'s.
pub fn decode_with_steps(bytes: &[u8], mode: Mode) -> (r: (Result<Decoded, Error>, Ghost<nat>))
    ensures
        r.1@ as int <= 16 * (bytes@.len() + t12_checked_n(bytes@, mode)),
        decode_mp_eq(r.0, bytes@, mode),
{
    let mut steps = Ghost(0nat);
    let result = decode_tracked(bytes, mode, &mut steps);
    (result, steps)
}

/// T10 · error precedence, at the manifest's quantification. For every R15 error
/// `e`, `decode(b, mode) = Err(e)` iff `MP(b, mode) = Err(e)`; the `decode = MP`
/// postcondition above discharges the `forall b, mode` part, and this witness
/// states the iff shape on the actual public entry point (R14's operational
/// ordering is proved through the T9 composition of `parse_header` (T2), the raw
/// helper (T6), the RLE helper (T7) and the mask helper (T8), each of which
/// carries the model's exact first error).
#[allow(dead_code)]
fn t10_error_precedence(bytes: &[u8], mode: Mode, e: Error) -> (r: Result<Decoded, Error>)
    ensures
        (r == Err(e)) == (crate::model::mp(bytes@, mode) == Err(e)),
{
    let r = decode(bytes, mode);
    proof {
        // `decode = MP` (T9) holds unconditionally; the iff follows.
    }
    r
}

/// T13 · success and resource bounds, at the manifest's quantification. Every
/// success has the model's exact `(w, h, p)` and the exact `4*w*h <= 67,108,864`
/// output length; the `decode = MP` postcondition and `model::lemma_mp_ok_shape`
/// discharge the `forall b, mode` part, this witness states it on the public
/// entry point.
#[allow(dead_code)]
fn t13_success_bounds(bytes: &[u8], mode: Mode) -> (r: Result<Decoded, Error>)
    ensures
        match r {
            Ok(d) => {
                &&& crate::model::mp(bytes@, mode) == Ok((
                    d.width as int, d.height as int, d.rgba@))
                &&& 1 <= d.width as int <= 8192
                &&& 1 <= d.height as int <= 8192
                &&& d.rgba@.len() == 4 * (d.width as int * d.height as int)
                &&& d.rgba@.len() <= 67108864
            },
            Err(_) => true,
        },
{
    decode(bytes, mode)
}

} // verus!
