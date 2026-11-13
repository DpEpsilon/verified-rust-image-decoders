//! Formal reference model for DIB-CORE-SPEC 1.0 (milestone M2).
//!
//! This file is written *independently of the executable code*: every definition
//! below is a transcription of the frozen specification text (`spec-frozen/SPEC.md`,
//! R1..R7, R13, R14, R15) over `Seq<u8>` and mathematical integers. Nothing here
//! calls, mirrors the control flow of, or is derived from `crate::dib`. The
//! executable carries postconditions phrased in terms of these definitions (T2);
//! the definitions themselves are never justified by running the executable.
//!
//! M2 needs exactly the total header model `MH` of THEOREMS.md ("Total mathematical
//! models"), plus the arithmetic lemmas that bound a successful header record.
//! `MP`/`MR`/`MA` and the pixel models are later milestones.
use vstd::prelude::*;

use crate::dib::{
    BitfieldOffsetOptions, BmpMode, Error, HeadersOptions, HighCountOptions, MasksOptions,
    MetadataOptions, Mode, OffsetOptions, PlanesOptions, Policy, ReservedOptions,
    ShortIndexedGapOptions, UnsupportedOptions,
};

verus! {

// ---------------------------------------------------------------------------
// R1, R3, R5: the spec's numeric limits, as mathematical integers.
// ---------------------------------------------------------------------------

/// R1: `MAX_INPUT = 67,108,864` bytes.
pub open spec fn max_input() -> int {
    67108864
}

/// R1/R5: `MAX_OUTPUT = 67,108,864` bytes (`4*N`).
pub open spec fn max_output() -> int {
    67108864
}

/// R3: `MAX_DIMENSION = 8,192`.
pub open spec fn max_dimension() -> int {
    8192
}

/// `2^d` for `0 <= d <= 8`, written as an unrolled chain so no recursion or
/// `compute` budget is involved. Values above 8 repeat 256; the spec only asks
/// for `d` in `{1,4,8}` and `d <= 8`.
pub open spec fn pow2(d: int) -> int {
    if d <= 0 {
        1
    } else if d == 1 {
        2
    } else if d == 2 {
        4
    } else if d == 3 {
        8
    } else if d == 4 {
        16
    } else if d == 5 {
        32
    } else if d == 6 {
        64
    } else if d == 7 {
        128
    } else {
        256
    }
}

/// `abs(x)` for mathematical integers.
pub open spec fn abs_int(x: int) -> int {
    if x < 0 {
        -x
    } else {
        x
    }
}

/// `S32_MIN`: the one stored height the spec rejects explicitly.
pub open spec fn s32_min() -> int {
    -2147483648
}

/// R1/R19: the shared-ICO-mode predicate — either ICO mode (`IcoEntryDib` or
/// `IcoEntryDibSlack`). Every R1–R18 ICO rule applies to a mode satisfying this;
/// only R12's final cursor-equality check is governed by `exact_tail` instead.
/// The 1.0 model's `x == Mode::IcoEntryDib` tests become `is_ico(x)` here, so on
/// the three 1.0 modes `is_ico` reduces to the old test exactly.
pub open spec fn is_ico(mode: Mode) -> bool {
    mode == Mode::IcoEntryDib || mode == Mode::IcoEntryDibSlack
}

/// R12/R14/R19: the exact-tail predicate — the modes that reject bytes after the
/// structure (`BareDib`, `IcoEntryDib`). `BmpFile` ignores a suffix and
/// `IcoEntryDibSlack` ignores bytes after a complete AND mask, so neither is
/// `exact_tail`.
pub open spec fn exact_tail(mode: Mode) -> bool {
    mode == Mode::BareDib || mode == Mode::IcoEntryDib
}

/// R19: the three 1.0 modes, i.e. every mode but the new slack mode.
pub open spec fn old_mode(mode: Mode) -> bool {
    mode == Mode::BmpFile || mode == Mode::BareDib || mode == Mode::IcoEntryDib
}

/// R3: the visible height from the stored height's absolute value. Both ICO
/// modes halve the stored height.
pub open spec fn visible_height_of(mode: Mode, abs_stored: int) -> int {
    if is_ico(mode) {
        abs_stored / 2
    } else {
        abs_stored
    }
}

// ---------------------------------------------------------------------------
// R1, R2, R6: guarded little-endian readers.
//
// The functions are total over every `p`; the spec's guarantee is that they are
// only *used* after the corresponding availability check, which the executable's
// guard chain enforces and the model's own guard chain mirrors.
// ---------------------------------------------------------------------------

/// R1/R2: `LE16(p) = b[p] + 256*b[p+1]`.
pub open spec fn le16(b: Seq<u8>, p: int) -> int {
    b[p] as int + 256 * (b[p + 1] as int)
}

/// R2/R6: `LE32(p)`, four little-endian bytes.
pub open spec fn le32(b: Seq<u8>, p: int) -> int {
    b[p] as int + 256 * (b[p + 1] as int) + 65536 * (b[p + 2] as int) + 16777216 * (b[p + 3] as int)
}

/// R2: `S32(p)`, the signed two's-complement reading of `LE32(p)`.
pub open spec fn s32(b: Seq<u8>, p: int) -> int {
    if le32(b, p) >= 2147483648 {
        le32(b, p) - 4294967296
    } else {
        le32(b, p)
    }
}

/// The `u32` a little-endian word denotes. `le32` is always `< 2^32`, so this is
/// the identity on values the model reads; the mask predicates are stated over
/// `u32` because R6's run test is a word-level bit test.
pub open spec fn as_u32(x: int) -> u32 {
    x as u32
}

// ---------------------------------------------------------------------------
// R4, R6, R9: classification and arithmetic used by the guards.
// ---------------------------------------------------------------------------

/// R4: the six supported bit depths.
pub open spec fn depth_ok(d: int) -> bool {
    d == 1 || d == 4 || d == 8 || d == 16 || d == 24 || d == 32
}

/// R2: the eight accepted DIB header sizes.
pub open spec fn hsize_ok(h: int) -> bool {
    h == 12 || h == 16 || h == 40 || h == 52 || h == 56 || h == 64 || h == 108 || h == 124
}

/// R6: the lowest set bit of `m`, `m & 0u32.wrapping_sub(m)`.
pub open spec fn low_of(m: u32) -> u32 {
    m & 0u32.wrapping_sub(m)
}

/// R6: `m` is one contiguous run of set bits when shifted down to bit zero. The
/// division by `low_of(m)` shifts the run to bit zero; the 32-bit wrapping add
/// makes an all-ones run give `compact+1 == 0`, so that case also succeeds.
/// Total: `contiguous(0)` is `true`, since the executable only calls it under
/// `m != 0` and no other case depends on it.
pub open spec fn contiguous(m: u32) -> bool {
    if m == 0u32 {
        true
    } else {
        let compact = m / low_of(m);
        (compact & compact.wrapping_add(1u32)) == 0u32
    }
}

/// R6: `m` has no bit at or above position `bits` (a `bits >= 32` mask spans the
/// whole word).
pub open spec fn within_bits(m: u32, bits: int) -> bool {
    if bits >= 32 {
        true
    } else {
        (m >> (bits as u32)) == 0u32
    }
}

/// R6: the four-mask rule: R,G,B nonzero; every nonzero mask a contiguous run
/// inside the depth's low bits; and all four pairwise disjoint. The executable's
/// check is the unrolled form of this conjunction.
pub open spec fn masks_ok(m0: u32, m1: u32, m2: u32, m3: u32, bits: int) -> bool {
    &&& m0 != 0u32
    &&& m1 != 0u32
    &&& m2 != 0u32
    &&& contiguous(m0)
    &&& contiguous(m1)
    &&& contiguous(m2)
    &&& contiguous(m3)
    &&& within_bits(m0, bits)
    &&& within_bits(m1, bits)
    &&& within_bits(m2, bits)
    &&& within_bits(m3, bits)
    &&& (m0 & m1) == 0u32
    &&& (m0 & m2) == 0u32
    &&& (m0 & m3) == 0u32
    &&& (m1 & m2) == 0u32
    &&& (m1 & m3) == 0u32
    &&& (m2 & m3) == 0u32
}

/// R4: BITFIELDS is accepted only at depth 16 or 32. Stated as a fact about a
/// validated `(compression, depth)` pair, so the pixel decoder can read a
/// BITFIELDS word of exactly two or four bytes.
pub open spec fn mask_depth_ok(compression: int, depth: int) -> bool {
    compression == 3 ==> (depth == 16 || depth == 32)
}

/// R9: `stride = 4*ceil(width*depth/32)`.
pub open spec fn stride_of(width: int, depth: int) -> int {
    ((width * depth + 31) / 32) * 4
}

/// All ones through the highest set bit: `(1 << bit_length(x)) - 1`.
/// The fixed shifts fill holes without a loop or a shift by the word width.
pub open spec fn channel_span(x: u32) -> u32 {
    let x = x | (x >> 1u32);
    let x = x | (x >> 2u32);
    let x = x | (x >> 4u32);
    let x = x | (x >> 8u32);
    x | (x >> 16u32)
}

pub proof fn lemma_channel_span_bounds(x: u32)
    ensures x <= channel_span(x),
        (channel_span(x) & channel_span(x).wrapping_add(1u32)) == 0u32,
        (x & x.wrapping_add(1u32)) == 0u32 ==> channel_span(x) == x,
{
    let a = x | (x >> 1u32);
    let b = a | (a >> 2u32);
    let c = b | (b >> 4u32);
    let d = c | (c >> 8u32);
    let r = d | (d >> 16u32);
    assert(r == channel_span(x));
    assert(x <= r && (r & r.wrapping_add(1u32)) == 0u32
        && ((x & x.wrapping_add(1u32)) == 0u32 ==> r == x)) by (bit_vector)
        requires a == x | (x >> 1u32), b == a | (a >> 2u32),
            c == b | (b >> 4u32), d == c | (c >> 8u32), r == d | (d >> 16u32);
}

/// R6: the interpreted value of component mask `mask` applied to `word`:
/// `shift = trailing_zeros(mask)`, `max = (1 << bit_length(mask >> shift)) - 1`,
/// `component = (word & mask) >> shift`,
/// `scale = (255*component + max/2) / max`. A zero mask yields the opaque 255.
/// Stated over `u32`/`u64` because the executable's rounding is a word-level
/// computation.
pub open spec fn channel_of(word: u32, mask: u32) -> u8 {
    if mask == 0u32 {
        255u8
    } else {
        let shift = mask.trailing_zeros();
        let max = channel_span(mask >> shift);
        let component = (word & mask) >> shift;
        scale_of(component as int, max as int) as u8
    }
}

/// R6: `scale(component, max) = floor((255*component + floor(max/2)) / max)`, in
/// mathematical integers. Callers pass `1 <= max` and `0 <= component <= max`.
pub open spec fn scale_of(component: int, max: int) -> int {
    (255 * component + max / 2) / max
}

// ---------------------------------------------------------------------------
// MH: the exact success record (R13, THEOREMS.md).
// ---------------------------------------------------------------------------

/// The record `MH` returns on success: the exact raw fields and every derived
/// quantity, in mathematical integers before any narrowing to `usize`.
pub struct MHRecord {
    pub mode: Mode,
    /// R1: the DIB start `s` (14 for `BmpFile`, 0 otherwise).
    pub s: int,
    /// R2: the DIB header size `H`.
    pub hsize: int,
    /// R3: the visible width.
    pub width: int,
    /// R3: the visible height (`abs(stored)` or `stored/2` for ICO).
    pub height: int,
    /// R3: the stored signed height as read.
    pub stored_height: int,
    /// R2: `planes`, required to be exactly one.
    pub planes: int,
    /// R4: the bit depth.
    pub depth: int,
    /// R4: the compression value.
    pub compression: int,
    /// R2: `biSizeImage`.
    pub declared_image: int,
    /// R5: `biClrUsed`.
    pub color_used: int,
    /// R7: `count`, the indexed palette size (`0` if not indexed).
    pub count: int,
    /// R7: `table_count`, the number of table entries present.
    pub table_count: int,
    /// R7: the palette entry stride (3 for CORE, 4 otherwise).
    pub palette_stride: int,
    /// R7: the palette start offset.
    pub palette_start: int,
    /// R7: `palette_start + table_count*palette_stride`.
    pub palette_end: int,
    /// R1/R7: the pixel cursor (`bfOffBits`, or `palette_end`).
    pub pixel_start: int,
    /// R6: the R,G,B,A masks; zero when compression is not BITFIELDS.
    pub mask_r: int,
    pub mask_g: int,
    pub mask_b: int,
    pub mask_a: int,
    /// R6: whether BITFIELDS masks were parsed.
    pub has_masks: bool,
    /// R2: whether the 12-byte CORE layout applies.
    pub core: bool,
    /// R1: whether the BMP file header was parsed.
    pub is_bmp: bool,
    /// R3: negative stored height outside ICO means top-down.
    pub top_down: bool,
    /// R5: `N = width*height`.
    pub n: int,
    /// R9: the raw row stride (`0` for RLE).
    pub stride: int,
}

/// R19: `m` with only its `mode` field replaced. The header record `MH` returns
/// is otherwise identical for `IcoEntryDib` and `IcoEntryDibSlack` on the same
/// input, so this is how the two modes' records relate.
pub open spec fn with_mode(m: MHRecord, mode: Mode) -> MHRecord {
    MHRecord {
        mode,
        s: m.s,
        hsize: m.hsize,
        width: m.width,
        height: m.height,
        stored_height: m.stored_height,
        planes: m.planes,
        depth: m.depth,
        compression: m.compression,
        declared_image: m.declared_image,
        color_used: m.color_used,
        count: m.count,
        table_count: m.table_count,
        palette_stride: m.palette_stride,
        palette_start: m.palette_start,
        palette_end: m.palette_end,
        pixel_start: m.pixel_start,
        mask_r: m.mask_r,
        mask_g: m.mask_g,
        mask_b: m.mask_b,
        mask_a: m.mask_a,
        has_masks: m.has_masks,
        core: m.core,
        is_bmp: m.is_bmp,
        top_down: m.top_down,
        n: m.n,
        stride: m.stride,
    }
}

/// R1: the optional BMP file-header prefix, returning `(s, bfOffBits)` or the
/// first error. `s = 14` on success.
pub open spec fn r1_prefix(b: Seq<u8>) -> Result<(int, int), Error> {
    if b.len() < 14 {
        Err(Error::TruncatedFileHeader)
    } else if b[0] != 66u8 || b[1] != 77u8 {
        Err(Error::BadMagic)
    } else if (le32(b, 2) != 0 && le32(b, 2) > b.len()) || le16(b, 6) != 0 || le16(b, 8) != 0 {
        Err(Error::InvalidFileSize)
    } else if le32(b, 10) < 18 || le32(b, 10) > b.len() {
        Err(Error::InvalidPixelOffset)
    } else {
        Ok((14int, le32(b, 10)))
    }
}

/// R2..R5: the raw header fields, before masks and palette. The record the
/// second phase needs, so the R14 ordering of the two phases is explicit.
pub struct RawFields {
    pub hsize: int,
    pub core: bool,
    pub width: int,
    pub stored: int,
    pub planes: int,
    pub depth: int,
    pub compression: int,
    pub declared: int,
    pub used: int,
    pub height: int,
    pub pos0: int,
}

/// R2..R5: header size, layout, planes, dimensions, depth, compression,
/// orientation, colour count and output cap, in R14 order.
pub open spec fn r2r5(b: Seq<u8>, mode: Mode, s: int) -> Result<RawFields, Error> {
    if b.len() < s + 4 {
        Err(Error::TruncatedDibHeader)
    } else if !hsize_ok(le32(b, s)) {
        Err(Error::UnsupportedHeaderSize)
    } else if b.len() < s + le32(b, s) {
        Err(Error::TruncatedHeader)
    } else {
        let hsize: int = le32(b, s);
        let core: bool = hsize == 12;
        let width_i: int = if core {
            le16(b, s + 4)
        } else {
            s32(b, s + 4)
        };
        let stored: int = if core {
            le16(b, s + 6)
        } else {
            s32(b, s + 8)
        };
        let planes: int = if core {
            le16(b, s + 8)
        } else {
            le16(b, s + 12)
        };
        let depth: int = if core {
            le16(b, s + 10)
        } else {
            le16(b, s + 14)
        };
        let compression: int = if core || hsize == 16 {
            0
        } else {
            le32(b, s + 16)
        };
        let declared: int = if core || hsize == 16 {
            0
        } else {
            le32(b, s + 20)
        };
        let used: int = if core || hsize == 16 {
            0
        } else {
            le32(b, s + 32)
        };
        let height_v: int = visible_height_of(mode, abs_int(stored));
        if planes != 1 {
            Err(Error::InvalidPlanes)
        } else if width_i <= 0 || stored == 0 || stored == s32_min() {
            Err(Error::InvalidDimensions)
        } else if is_ico(mode) && (stored < 0 || stored % 2 != 0) {
            Err(Error::InvalidDimensions)
        } else if width_i > max_dimension() || height_v > max_dimension() {
            Err(Error::InvalidDimensions)
        } else if !depth_ok(depth) {
            Err(Error::UnsupportedPixelDepth)
        } else if compression > 3 {
            Err(Error::UnsupportedCompression)
        } else if (compression == 1 && depth != 8) || (compression == 2 && depth != 4) || (
        compression == 3 && depth != 16 && depth != 32) {
            Err(Error::InvalidCompressionDepth)
        } else if (stored < 0 && (compression == 1 || compression == 2)) || (is_ico(mode)
            && (compression == 1 || compression == 2)) {
            Err(Error::UnsupportedOrientation)
        } else if (depth <= 8 && used > pow2(depth)) || (depth > 8 && used != 0 && mode
            != Mode::BmpFile) {
            Err(Error::InvalidColorCount)
        } else if 4 * (width_i * height_v) > max_output() {
            Err(Error::OutputTooLarge)
        } else {
            Ok(
                RawFields {
                    hsize,
                    core,
                    width: width_i,
                    stored,
                    planes,
                    depth,
                    compression,
                    declared,
                    used,
                    height: height_v,
                    pos0: s + hsize,
                },
            )
        }
    }
}

/// R7: the palette start offset `pos` for validated fields.
pub open spec fn palette_start_of(rf: RawFields) -> int {
    if rf.compression == 3 && (rf.hsize == 40 || rf.hsize == 64) {
        rf.pos0 + 12
    } else {
        rf.pos0
    }
}

/// R7: `table_count*palette_stride` for validated fields.
pub open spec fn palette_stride_of(rf: RawFields) -> int {
    if rf.core {
        3int
    } else {
        4int
    }
}

pub open spec fn table_count_of(rf: RawFields) -> int {
    if rf.depth <= 8 {
        if rf.used != 0 {
            rf.used
        } else {
            pow2(rf.depth)
        }
    } else {
        rf.used
    }
}

pub open spec fn palette_length_of(rf: RawFields) -> int {
    table_count_of(rf) * palette_stride_of(rf)
}

/// R7: the palette end offset for validated fields.
pub open spec fn palette_end_of(rf: RawFields) -> int {
    palette_start_of(rf) + palette_length_of(rf)
}

/// The record `r6r7` builds once its R6/R7 guards have passed. This is the
/// record-construction half of `r6r7`, split out so the guard reasoning and the
/// field-by-field comparison can be proved separately.
pub open spec fn r6r7_result(
    b: Seq<u8>,
    mode: Mode,
    s: int,
    file_offset: int,
    is_bmp: bool,
    rf: RawFields,
) -> MHRecord {
    let compression: int = rf.compression;
    let depth: int = rf.depth;
    let used: int = rf.used;
    let external_masks: bool = rf.hsize == 40 || rf.hsize == 64;
    let pos: int = if compression == 3 && external_masks {
        rf.pos0 + 12
    } else {
        rf.pos0
    };
    let count: int = if depth <= 8 {
        if used != 0 {
            used
        } else {
            pow2(depth)
        }
    } else {
        0
    };
    let table_count: int = if depth <= 8 {
        count
    } else {
        used
    };
    let pal_stride: int = if rf.core {
        3
    } else {
        4
    };
    let palette_end: int = pos + table_count * pal_stride;
    MHRecord {
        mode,
        s,
        hsize: rf.hsize,
        width: rf.width,
        height: rf.height,
        stored_height: rf.stored,
        planes: rf.planes,
        depth,
        compression,
        declared_image: rf.declared,
        color_used: used,
        count,
        table_count,
        palette_stride: pal_stride,
        palette_start: pos,
        palette_end,
        pixel_start: if is_bmp {
            file_offset
        } else {
            palette_end
        },
        mask_r: if compression == 3 && external_masks {
            le32(b, rf.pos0)
        } else if compression == 3 {
            le32(b, s + 40)
        } else {
            0
        },
        mask_g: if compression == 3 && external_masks {
            le32(b, rf.pos0 + 4)
        } else if compression == 3 {
            le32(b, s + 44)
        } else {
            0
        },
        mask_b: if compression == 3 && external_masks {
            le32(b, rf.pos0 + 8)
        } else if compression == 3 {
            le32(b, s + 48)
        } else {
            0
        },
        mask_a: if compression == 3 && external_masks {
            0
        } else if compression == 3 && rf.hsize >= 56 {
            le32(b, s + 52)
        } else {
            0
        },
        has_masks: compression == 3,
        core: rf.core,
        is_bmp,
        top_down: rf.stored < 0,
        n: rf.width * rf.height,
        stride: if compression == 0 || compression == 3 {
            stride_of(rf.width, depth)
        } else {
            0
        },
    }
}

/// R6..R7: masks and palette geometry, given validated fields. `s` and
/// `file_offset`/`is_bmp` are the R1 outputs.
pub open spec fn r6r7(
    b: Seq<u8>,
    mode: Mode,
    s: int,
    file_offset: int,
    is_bmp: bool,
    rf: RawFields,
) -> Result<MHRecord, Error> {
    let compression: int = rf.compression;
    let depth: int = rf.depth;
    let used: int = rf.used;
    let pos0: int = rf.pos0;
    let external_masks: bool = rf.hsize == 40 || rf.hsize == 64;
    if compression == 3 && external_masks && b.len() - pos0 < 12 {
        Err(Error::TruncatedMasks)
    } else {
        let raw0: int = if compression == 3 && external_masks {
            le32(b, pos0)
        } else if compression == 3 {
            le32(b, s + 40)
        } else {
            0
        };
        let raw1: int = if compression == 3 && external_masks {
            le32(b, pos0 + 4)
        } else if compression == 3 {
            le32(b, s + 44)
        } else {
            0
        };
        let raw2: int = if compression == 3 && external_masks {
            le32(b, pos0 + 8)
        } else if compression == 3 {
            le32(b, s + 48)
        } else {
            0
        };
        let raw3: int = if compression == 3 && external_masks {
            0
        } else if compression == 3 && rf.hsize >= 56 {
            le32(b, s + 52)
        } else {
            0
        };
        if compression == 3 && !masks_ok(
            as_u32(raw0),
            as_u32(raw1),
            as_u32(raw2),
            as_u32(raw3),
            depth,
        ) {
            Err(Error::InvalidBitfields)
        } else {
            let pos: int = if compression == 3 && external_masks {
                pos0 + 12
            } else {
                pos0
            };
            let count: int = if depth <= 8 {
                if used != 0 {
                    used
                } else {
                    pow2(depth)
                }
            } else {
                0
            };
            let table_count: int = if depth <= 8 {
                count
            } else {
                used
            };
            let pal_stride: int = if rf.core {
                3
            } else {
                4
            };
            let palette_length: int = table_count * pal_stride;
            let palette_end: int = pos + palette_length;
            if is_bmp && file_offset < palette_end {
                Err(Error::InvalidPixelOffset)
            } else if b.len() < palette_end {
                Err(Error::TruncatedPalette)
            } else {
                Ok(r6r7_result(b, mode, s, file_offset, is_bmp, rf))
            }
        }
    }
}

/// `r6r7` returns `r6r7_result` exactly when its R6/R7 guards pass. Stated with
/// the guard facts as explicit hypotheses, so the executable can discharge them
/// from its own concrete checks without unfolding the model.
pub proof fn lemma_r6r7_eq_result(
    b: Seq<u8>,
    mode: Mode,
    s: int,
    file_offset: int,
    is_bmp: bool,
    rf: RawFields,
)
    requires
        // R6: the external-mask availability test does not trip, and the
        // four-mask validity test passes (both only relevant at BITFIELDS).
        !(rf.compression == 3 && (rf.hsize == 40 || rf.hsize == 64) && b.len() - rf.pos0 < 12),
        rf.compression != 3 || masks_ok(
            as_u32(if rf.compression == 3 && (rf.hsize == 40 || rf.hsize == 64) {
                le32(b, rf.pos0)
            } else if rf.compression == 3 {
                le32(b, s + 40)
            } else {
                0
            }),
            as_u32(if rf.compression == 3 && (rf.hsize == 40 || rf.hsize == 64) {
                le32(b, rf.pos0 + 4)
            } else if rf.compression == 3 {
                le32(b, s + 44)
            } else {
                0
            }),
            as_u32(if rf.compression == 3 && (rf.hsize == 40 || rf.hsize == 64) {
                le32(b, rf.pos0 + 8)
            } else if rf.compression == 3 {
                le32(b, s + 48)
            } else {
                0
            }),
            as_u32(if rf.compression == 3 && (rf.hsize == 40 || rf.hsize == 64) {
                0
            } else if rf.compression == 3 && rf.hsize >= 56 {
                le32(b, s + 52)
            } else {
                0
            }),
            rf.depth,
        ),
        // R7: the BMP offset check and the palette availability check both pass.
        !(is_bmp && file_offset < palette_end_of(rf)),
        !(b.len() < palette_end_of(rf)),
    ensures
        r6r7(b, mode, s, file_offset, is_bmp, rf) == Ok(r6r7_result(
            b,
            mode,
            s,
            file_offset,
            is_bmp,
            rf,
        )),
{
    reveal(r6r7);
    reveal(r6r7_result);
}

/// R2..R7: the DIB header and palette prefix beginning at `s`, with
/// `file_offset` the BMP `bfOffBits` (meaningful only when `is_bmp`).
pub open spec fn r2r7(
    b: Seq<u8>,
    mode: Mode,
    s: int,
    file_offset: int,
    is_bmp: bool,
) -> Result<MHRecord, Error> {
    match r2r5(b, mode, s) {
        Err(e) => Err(e),
        Ok(rf) => r6r7(b, mode, s, file_offset, is_bmp, rf),
    }
}

/// `MH(b, mode)`: the R1 file prefix (for `BmpFile`), then R2..R7 in R14 order,
/// returning the exact record or the first error. No absent byte is read.
pub open spec fn mh(b: Seq<u8>, mode: Mode) -> Result<MHRecord, Error> {
    if b.len() > max_input() {
        Err(Error::FileTooLarge)
    } else if mode == Mode::BmpFile {
        match r1_prefix(b) {
            Err(e) => Err(e),
            Ok(p) => r2r7(b, mode, p.0, p.1, true),
        }
    } else {
        r2r7(b, mode, 0, 0, false)
    }
}

// ---------------------------------------------------------------------------
// Arithmetic lemmas over the model (justifying the executable bounds).
// ---------------------------------------------------------------------------

/// `LE16(p) < 2^16` whenever both bytes exist.
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
    assert(0 <= le16(b, p)) by (nonlinear_arith)
        requires
            le16(b, p) == b[p] as int + 256 * (b[p + 1] as int),
            0 <= b[p] as int,
            0 <= b[p + 1] as int,
    ;
}

/// `LE32(p) < 2^32` whenever all four bytes exist.
pub proof fn lemma_le32_lt(b: Seq<u8>, p: int)
    requires
        p + 4 <= b.len(),
    ensures
        0 <= le32(b, p) < 4294967296,
{
    assert(0 <= b[p] as int);
    assert((b[p] as int) < 256);
    assert(0 <= b[p + 1] as int);
    assert((b[p + 1] as int) < 256);
    assert(0 <= b[p + 2] as int);
    assert((b[p + 2] as int) < 256);
    assert(0 <= b[p + 3] as int);
    assert((b[p + 3] as int) < 256);
    assert(le32(b, p) < 4294967296) by (nonlinear_arith)
        requires
            le32(b, p) == b[p] as int + 256 * (b[p + 1] as int) + 65536 * (b[p + 2] as int)
                + 16777216 * (b[p + 3] as int),
            0 <= b[p] as int,
            (b[p] as int) < 256,
            0 <= b[p + 1] as int,
            (b[p + 1] as int) < 256,
            0 <= b[p + 2] as int,
            (b[p + 2] as int) < 256,
            0 <= b[p + 3] as int,
            (b[p + 3] as int) < 256,
    ;
    assert(0 <= le32(b, p)) by (nonlinear_arith)
        requires
            le32(b, p) == b[p] as int + 256 * (b[p + 1] as int) + 65536 * (b[p + 2] as int)
                + 16777216 * (b[p + 3] as int),
            0 <= b[p] as int,
            0 <= b[p + 1] as int,
            0 <= b[p + 2] as int,
            0 <= b[p + 3] as int,
    ;
}

/// `2^d >= 1` for every `d`.
pub proof fn lemma_pow2_pos(d: int)
    ensures
        1 <= pow2(d),
{
    reveal(pow2);
    if d <= 0 {
        assert(pow2(d) == 1);
    } else if d == 1 {
        assert(pow2(d) == 2);
    } else if d == 2 {
        assert(pow2(d) == 4);
    } else if d == 3 {
        assert(pow2(d) == 8);
    } else if d == 4 {
        assert(pow2(d) == 16);
    } else if d == 5 {
        assert(pow2(d) == 32);
    } else if d == 6 {
        assert(pow2(d) == 64);
    } else if d == 7 {
        assert(pow2(d) == 128);
    } else {
        assert(pow2(d) == 256);
    }
}

/// `2^d <= 256` for every `d` (the unrolled definition saturates at 256 and
/// has no branch above 2^8), so no `d <= 8` hypothesis is needed.
pub proof fn lemma_pow2_le_256(d: int)
    ensures
        pow2(d) <= 256,
{
    lemma_pow2_pos(d);
    reveal(pow2);
    if d <= 0 {
        assert(pow2(d) == 1);
    } else if d <= 7 {
        assert(pow2(d) <= 128) by {
            if d == 1 {
                assert(pow2(d) == 2);
            } else if d == 2 {
                assert(pow2(d) == 4);
            } else if d == 3 {
                assert(pow2(d) == 8);
            } else if d == 4 {
                assert(pow2(d) == 16);
            } else if d == 5 {
                assert(pow2(d) == 32);
            } else if d == 6 {
                assert(pow2(d) == 64);
            } else {
                assert(d == 7);
                assert(pow2(d) == 128);
            }
        }
    } else {
        assert(pow2(d) == 256);
    }
}

/// R9: `stride_of(w, d) = 4*ceil(w*d/32)` is at most 32,768 and nonnegative for
/// a validated `1 <= w <= 8192`, `1 <= d <= 32`.
pub proof fn lemma_stride_bound(w: int, d: int)
    requires
        1 <= w <= 8192,
        1 <= d <= 32,
    ensures
        0 <= stride_of(w, d) <= 32768,
{
    reveal(stride_of);
    assert(stride_of(w, d) == ((w * d + 31) / 32) * 4);
    assert(w * d <= 262144) by (nonlinear_arith)
        requires
            1 <= w <= 8192,
            1 <= d <= 32,
    ;
    assert(stride_of(w, d) <= 32768) by (nonlinear_arith)
        requires
            stride_of(w, d) == ((w * d + 31) / 32) * 4,
            w * d <= 262144,
            w >= 1,
            d >= 1,
    ;
    assert((w * d + 31) / 32 >= 0) by (nonlinear_arith)
        requires
            w * d >= 0,
            1 <= w,
            1 <= d,
    ;
    assert(stride_of(w, d) >= 0) by (nonlinear_arith)
        requires
            stride_of(w, d) == ((w * d + 31) / 32) * 4,
            (w * d + 31) / 32 >= 0,
    ;
}

/// R9: `stride_of(w,d)*8 >= w*d`, the key stride fact: every sample of the last
/// column ends within the four-byte-aligned stride.
pub proof fn lemma_stride_times_8(w: int, d: int)
    requires
        1 <= w <= 8192,
        1 <= d <= 32,
    ensures
        stride_of(w, d) * 8 >= w * d,
{
    reveal(stride_of);
    assert(stride_of(w, d) * 8 == ((w * d + 31) / 32) * 32);
    assert(((w * d + 31) / 32) * 32 >= w * d) by (nonlinear_arith)
        requires
            (w * d + 31) / 32 * 32 >= w * d,
    ;
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod(w * d + 31, 32);
    assert((w * d + 31) / 32 * 32 >= w * d) by {
        let q = (w * d + 31) / 32;
        let r = (w * d + 31) % 32;
        assert(w * d + 31 == 32 * q + r);
        assert(0 <= r < 32);
        assert(32 * q >= w * d) by (nonlinear_arith)
            requires
                w * d + 31 == 32 * q + r,
                0 <= r < 32,
        ;
    }
    assert(stride_of(w, d) * 8 >= w * d) by (nonlinear_arith)
        requires
            stride_of(w, d) * 8 == ((w * d + 31) / 32) * 32,
            32 * ((w * d + 31) / 32) >= w * d,
    ;
}

/// R9: for `0 <= x < width` the last sample byte of pixel `x` stays inside the
/// stride: `(x+1)*(depth/8) <= stride`.
pub proof fn lemma_sample_within_stride(width: int, depth: int, x: int)
    requires
        1 <= width <= 8192,
        1 <= depth <= 32,
        depth % 8 == 0,
        0 <= x < width,
    ensures
        x * (depth / 8) + depth / 8 <= stride_of(width, depth),
{
    reveal(stride_of);
    lemma_stride_times_8(width, depth);
    assert(depth / 8 == depth / 8);
    assert((x + 1) * (depth / 8) <= stride_of(width, depth)) by (nonlinear_arith)
        requires
            stride_of(width, depth) * 8 >= width * depth,
            (x + 1) <= width,
            depth % 8 == 0,
            depth >= 0,
            x >= 0,
    ;
    assert(x * (depth / 8) + depth / 8 == (x + 1) * (depth / 8)) by (nonlinear_arith);
}

/// R9: for `1 <= depth <= 8`, the sample byte `x*depth/8` is below the stride.
pub proof fn lemma_subbyte_within_stride(width: int, depth: int, x: int)
    requires
        1 <= width <= 8192,
        1 <= depth <= 8,
        0 <= x < width,
    ensures
        x * depth / 8 + 1 <= stride_of(width, depth),
{
    reveal(stride_of);
    lemma_stride_times_8(width, depth);
    assert(stride_of(width, depth) == ((width * depth + 31) / 32) * 4);
    assert(stride_of(width, depth) * 8 >= width * depth);
    assert(x * depth <= (width - 1) * depth) by (nonlinear_arith)
        requires
            x < width,
            depth >= 0,
    ;
    assert(x * depth < width * depth) by (nonlinear_arith)
        requires
            x < width,
            depth >= 1,
    ;
    assert(x * depth < stride_of(width, depth) * 8) by (nonlinear_arith)
        requires
            x * depth < width * depth,
            stride_of(width, depth) * 8 >= width * depth,
    ;
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod(x * depth, 8);
    assert(x * depth / 8 <= stride_of(width, depth) - 1) by (nonlinear_arith)
        requires
            x * depth < stride_of(width, depth) * 8,
            x * depth == 8 * (x * depth / 8) + x * depth % 8,
            0 <= x * depth % 8 < 8,
            stride_of(width, depth) >= 1,
    ;
}

/// R8: the sub-byte sample shift `8 - depth - (x*depth mod 8)` is in `[0,8)`
/// for `depth in {1,4,8}` (each divides 8, so `x*depth mod 8` is a multiple of
/// `depth` bounded by `8-depth`).
proof fn lemma_two_bit_shift(x: int)
    requires x >= 0,
    ensures 0 <= 6 - x * 2 % 8 <= 7,
{
    assert((x * 2) % 8 == (x % 4) * 2) by (nonlinear_arith) requires x >= 0;
    assert(0 <= x % 4 < 4);
}

pub proof fn lemma_subbyte_shift(depth: int, x: int)
    requires
        depth == 1 || depth == 2 || depth == 4 || depth == 8,
        x >= 0,
    ensures
        0 <= 8 - depth - (x * depth % 8) <= 7,
{
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod(x * depth, 8);
    assert(x * depth % 8 >= 0);
    assert(x * depth % 8 < 8);
    if depth == 1 {
        assert(x * 1 % 8 < 8);
        assert(0 <= 8 - 1 - x % 8 <= 7) by (nonlinear_arith)
            requires
                x % 8 >= 0,
                x % 8 < 8,
                x * 1 % 8 == x % 8,
        ;
    } else if depth == 2 {
        lemma_two_bit_shift(x);
    } else if depth == 4 {
        assert((x * 4) % 8 == 0 || (x * 4) % 8 == 4) by (nonlinear_arith)
            requires
                x * 4 % 8 == (x % 2) * 4,
                x % 2 == 0 || x % 2 == 1,
        ;
        assert(0 <= 8 - 4 - x * 4 % 8 <= 7) by (nonlinear_arith)
            requires
                (x * 4) % 8 == 0 || (x * 4) % 8 == 4,
        ;
    } else {
        assert(depth == 8);
        assert(x * 8 % 8 == 0) by (nonlinear_arith)
            requires
                x * 8 % 8 == 0,
        ;
        assert(0 <= 8 - 8 - x * 8 % 8 <= 7) by (nonlinear_arith)
            requires
                x * 8 % 8 == 0,
        ;
    }
}

/// R12: the AND-mask bit of pixel column `x` is the bit `0x80 >> (x mod 8)` of
/// byte `floor(x/8)`; for a validated width `x < width <= 8192`, that byte index
/// is one below the padded 1-bpp row stride `stride_of(width,1)`. This is the
/// named nonlinear-arithmetic lemma behind the mask read's containment proof.
pub proof fn and_bit_offset_bound(width: int, x: int)
    requires
        1 <= width <= 8192,
        0 <= x < width,
    ensures
        x / 8 + 1 <= stride_of(width, 1),
{
    reveal(stride_of);
    lemma_stride_times_8(width, 1);
    assert(stride_of(width, 1) == ((width + 31) / 32) * 4);
    assert(stride_of(width, 1) * 8 >= width);
    assert(x / 8 + 1 <= stride_of(width, 1)) by (nonlinear_arith)
        requires
            stride_of(width, 1) * 8 >= width,
            x < width,
            x >= 0,
            stride_of(width, 1) == ((width + 31) / 32) * 4,
            width >= 1,
    ;
}

/// T5 · the row-orientation map is a bijection on `[0, N)`. For every validated
/// `1 <= w,h <= 8192` the flip `row` and its `flip(row)` inverse are inverses
/// for both the top-down identity and the bottom-up flip `h-1-r`, and the
/// composed cell map `r*w + x` is injective.
pub proof fn lemma_vertical_flip_bijection(w: int, h: int, r1: int, r2: int, x1: int, x2: int)
    requires
        1 <= w <= 8192,
        1 <= h <= 8192,
        0 <= r1 < h,
        0 <= r2 < h,
        0 <= x1 < w,
        0 <= x2 < w,
    ensures
        (h - 1 - r1) == (h - 1 - r2) <==> r1 == r2,
        0 <= h - 1 - r1 < h,
        (h - 1 - (h - 1 - r1)) == r1,
        r1 * w + x1 == r2 * w + x2 <==> (r1 == r2 && x1 == x2),
{
    assert(0 <= h - 1 - r1 < h) by (nonlinear_arith)
        requires 0 <= r1 < h;
    assert(0 <= h - 1 - r2 < h) by (nonlinear_arith)
        requires 0 <= r2 < h;
    assert((h - 1 - r1) == (h - 1 - r2) <==> r1 == r2) by (nonlinear_arith);
    assert(h - 1 - (h - 1 - r1) == r1) by (nonlinear_arith);
    assert(r1 * w + x1 == r2 * w + x2 <==> (r1 == r2 && x1 == x2)) by (nonlinear_arith)
        requires
            0 <= x1 < w,
            0 <= x2 < w,
            0 <= r1,
            0 <= r2,
            w >= 1,
    ;
}

// ---------------------------------------------------------------------------
// R8/R6: the colour conversion `Conv` (T4).
// ---------------------------------------------------------------------------

/// R1/R2: the little-endian word of `n` bytes at `base` (`1 <= n <= 4`). For
/// `n == 2`/`n == 4` this is `le16`/`le32`.
pub open spec fn word_n(b: Seq<u8>, base: int, n: int) -> int {
    if n <= 0 {
        0
    } else if n == 1 {
        b[base] as int
    } else if n == 2 {
        b[base] as int + 256 * (b[base + 1] as int)
    } else if n == 3 {
        b[base] as int + 256 * (b[base + 1] as int) + 65536 * (b[base + 2] as int)
    } else {
        b[base] as int + 256 * (b[base + 1] as int) + 65536 * (b[base + 2] as int)
            + 16777216 * (b[base + 3] as int)
    }
}

/// R7/R8: palette entry `ix` as RGBA: R at `k == 0`, G at `k == 1`, B at
/// `k == 2` from the entry, the reserved fourth byte ignored, alpha 255.
pub open spec fn pal_entry(b: Seq<u8>, palette_start: int, pal_stride: int, ix: int) -> Seq<u8> {
    seq![
        b[palette_start + ix * pal_stride + 2],
        b[palette_start + ix * pal_stride + 1],
        b[palette_start + ix * pal_stride],
        255u8,
    ]
}

/// R8: a B,G,R[,A]-bearing word as RGBA; the output byte `k` is R at `k == 0`,
/// G at `k == 1`, B at `k == 2` and the alpha at `k == 3`. The word is written
/// with division/modulo so it stays a mathematical integer.
pub open spec fn rgb_byte(word: int, k: int, alpha: int) -> u8 {
    if k == 0 {
        ((word / 65536) % 256) as u8
    } else if k == 1 {
        ((word / 256) % 256) as u8
    } else if k == 2 {
        (word % 256) as u8
    } else {
        alpha as u8
    }
}

/// R8: the BI_RGB 16-bit rounded 5:5:5 conversion; bit 15 ignored, alpha 255.
pub open spec fn rgb555_seq(word: int) -> Seq<u8> {
    seq![
        channel_of(word as u32, 0x7c00u32),
        channel_of(word as u32, 0x03e0u32),
        channel_of(word as u32, 0x001fu32),
        255u8,
    ]
}

/// R8: the full BYTE order as the executable writes it: R at `k == 0`, G at
/// `k == 1`, B at `k == 2`, alpha at `k == 3`.
pub open spec fn rgb_seq(word: int, alpha: int) -> Seq<u8> {
    seq![rgb_byte(word, 0, alpha), rgb_byte(word, 1, alpha), rgb_byte(word, 2, alpha), alpha as u8]
}

/// R8: the BI_RGB 16-bit rounded 5:5:5 conversion as the executable writes it:
/// R,G,B from the 5:5:5 masks (rounded), opaque alpha, bit 15 ignored.
pub open spec fn rgb555_channel(word: int, k: int) -> u8 {
    if k == 0 {
        channel_of(word as u32, 0x7c00u32)
    } else if k == 1 {
        channel_of(word as u32, 0x03e0u32)
    } else if k == 2 {
        channel_of(word as u32, 0x001fu32)
    } else {
        255u8
    }
}

/// R6: a BITFIELDS word under the record's four masks (a zero A mask is 255).
pub open spec fn bitfield_channel(m: MHRecord, word: int, k: int) -> u8 {
    if k == 0 {
        channel_of(word as u32, m.mask_r as u32)
    } else if k == 1 {
        channel_of(word as u32, m.mask_g as u32)
    } else if k == 2 {
        channel_of(word as u32, m.mask_b as u32)
    } else {
        channel_of(word as u32, m.mask_a as u32)
    }
}

/// R6: the four BITFIELDS channels as the executable writes them.
pub open spec fn bitfield_seq(m: MHRecord, word: int) -> Seq<u8> {
    seq![
        channel_of(word as u32, m.mask_r as u32),
        channel_of(word as u32, m.mask_g as u32),
        channel_of(word as u32, m.mask_b as u32),
        channel_of(word as u32, m.mask_a as u32),
    ]
}

/// R8: `Conv` for a non-indexed sample word (16/24/32 bpp RGB or BITFIELDS). The
/// 32-bit BI_RGB alpha is the fourth byte in `IcoEntryDib` and opaque elsewhere.
pub open spec fn conv_word(m: MHRecord, word: int) -> Seq<u8> {
    if m.compression == 3 {
        bitfield_seq(m, word)
    } else if m.depth == 16 {
        rgb555_seq(word)
    } else if m.depth == 24 {
        rgb_seq(word, 255)
    } else {
        rgb_seq(word, if is_ico(m.mode) { (word / 16777216) % 256 } else { 255 })
    }
}

/// R8: `Conv(H, word_or_index)`: the palette entry for an indexed depth, else
/// the word conversion.
pub open spec fn conv(b: Seq<u8>, m: MHRecord, v: int) -> Seq<u8> {
    if m.depth <= 8 {
        pal_entry(b, m.palette_start, m.palette_stride, v)
    } else {
        conv_word(m, v)
    }
}

/// R8: the whole-byte RGBA of a pixel whose bytes start at `base`, following the
/// executable's own reads: BITFIELDS/16-bit words are `LE16`/`LE32` (the
/// executable's `rd16`/`rd32`), 24/32-bit BI_RGB are read byte by byte (B at
/// `base`, G at `base+1`, R at `base+2`, A at `base+3` in ICO only). This is the
/// precise target of the executable `raw_sample` for depth 16/24/32.
pub open spec fn conv_rgb(b: Seq<u8>, m: MHRecord, base: int) -> Seq<u8> {
    if m.compression == 3 {
        if m.depth == 16 {
            bitfield_seq(m, le16(b, base))
        } else {
            bitfield_seq(m, le32(b, base))
        }
    } else if m.depth == 16 {
        rgb555_seq(le16(b, base))
    } else if m.depth == 24 {
        seq![b[base + 2], b[base + 1], b[base], 255u8]
    } else {
        seq![
            b[base + 2],
            b[base + 1],
            b[base],
            if is_ico(m.mode) {
                b[base + 3]
            } else {
                255u8
            },
        ]
    }
}

/// R8: the sub-byte sample shift `8 - depth - (x*depth mod 8)`.
pub open spec fn sub_shift(depth: int, x: int) -> int {
    8 - depth - (x * depth % 8)
}

/// R8: the sub-byte value mask for depth 1, 4 or 8.
pub open spec fn sub_mask(depth: int) -> u32 {
    if depth == 1 {
        1u32
    } else if depth == 2 {
        3u32
    } else if depth == 4 {
        15u32
    } else {
        255u32
    }
}

/// R8: the palette index of raw pixel `x` from the byte `byte` of a
/// `depth in {1,4,8}` row: choose the high bit/nibble first.
pub open spec fn sub_index(byte: u8, depth: int, x: int) -> int {
    ((byte as u32 >> (sub_shift(depth, x) as u32)) & sub_mask(depth)) as int
}

/// R8: the exact RGBA of raw sample `x` of the row starting at byte `base`.
pub open spec fn raw_conv(b: Seq<u8>, m: MHRecord, base: int, x: int) -> Seq<u8> {
    if m.depth <= 8 {
        conv(b, m, sub_index(b[base + x * m.depth / 8], m.depth, x))
    } else {
        conv_rgb(b, m, base + x * m.depth / 8)
    }
}

/// R8: `LE16` and `LE32` viewed as `u32`.
pub open spec fn le16_u32(b: Seq<u8>, p: int) -> u32 {
    le16(b, p) as u32
}

pub open spec fn le32_u32(b: Seq<u8>, p: int) -> u32 {
    le32(b, p) as u32
}

/// R8: the raw little-endian sample word of a non-indexed pixel: `LE16`/`LE32`
/// for the word depths (BITFIELDS and BI_RGB 16/32), a three-byte word for
/// 24-bit BI_RGB.
pub open spec fn sample_word(b: Seq<u8>, m: MHRecord, base: int) -> int {
    if m.compression == 3 {
        if m.depth == 16 {
            le16(b, base)
        } else {
            le32(b, base)
        }
    } else if m.depth == 16 {
        le16(b, base)
    } else if m.depth == 24 {
        b[base] as int + 256 * (b[base + 1] as int) + 65536 * (b[base + 2] as int)
    } else {
        le32(b, base)
    }
}

/// R8: a three-byte little-endian word's member bytes are exactly the bytes the
/// executable reads for 24-bit BI_RGB.
pub proof fn lemma_rgb_byte_three(b: Seq<u8>, base: int)
    requires
        base + 3 <= b.len(),
    ensures
        {
            let w = b[base] as int + 256 * (b[base + 1] as int) + 65536 * (b[base + 2] as int);
            &&& rgb_byte(w, 0, 255) == b[base + 2]
            &&& rgb_byte(w, 1, 255) == b[base + 1]
            &&& rgb_byte(w, 2, 255) == b[base]
        },
{
    reveal(rgb_byte);
    let w = b[base] as int + 256 * (b[base + 1] as int) + 65536 * (b[base + 2] as int);
    assert(0 <= (b[base] as int) < 256);
    assert(0 <= (b[base + 1] as int) < 256);
    assert(0 <= (b[base + 2] as int) < 256);
    assert((w / 65536) % 256 == b[base + 2] as int) by (nonlinear_arith)
        requires
            w == b[base] as int + 256 * (b[base + 1] as int) + 65536 * (b[base + 2] as int);
    assert((w / 256) % 256 == b[base + 1] as int) by (nonlinear_arith)
        requires
            w == b[base] as int + 256 * (b[base + 1] as int) + 65536 * (b[base + 2] as int);
    assert(w % 256 == b[base] as int) by (nonlinear_arith)
        requires
            w == b[base] as int + 256 * (b[base + 1] as int) + 65536 * (b[base + 2] as int);
}

/// R8: `LE32`'s top byte is the `IcoEntryDib` alpha byte.
pub proof fn lemma_le32_top_byte(b: Seq<u8>, base: int)
    requires
        base + 4 <= b.len(),
    ensures
        rgb_byte(le32(b, base), 3, (le32(b, base) / 16777216) % 256) == b[base + 3],
        (le32(b, base) / 16777216) % 256 == b[base + 3] as int,
        rgb_byte(le32(b, base), 0, 255) == b[base + 2],
        rgb_byte(le32(b, base), 1, 255) == b[base + 1],
        rgb_byte(le32(b, base), 2, 255) == b[base],
{
    reveal(rgb_byte);
    reveal(le32);
    let l = le32(b, base);
    assert(l == b[base] as int + 256 * (b[base + 1] as int) + 65536 * (b[base + 2] as int)
        + 16777216 * (b[base + 3] as int));
    assert(0 <= (b[base] as int) < 256);
    assert(0 <= (b[base + 1] as int) < 256);
    assert(0 <= (b[base + 2] as int) < 256);
    assert(0 <= (b[base + 3] as int) < 256);
    assert((l / 16777216) % 256 == b[base + 3] as int) by (nonlinear_arith)
        requires
            l == b[base] as int + 256 * (b[base + 1] as int) + 65536 * (b[base + 2] as int)
                + 16777216 * (b[base + 3] as int);
    assert((l / 65536) % 256 == b[base + 2] as int) by (nonlinear_arith)
        requires
            l == b[base] as int + 256 * (b[base + 1] as int) + 65536 * (b[base + 2] as int)
                + 16777216 * (b[base + 3] as int);
    assert((l / 256) % 256 == b[base + 1] as int) by (nonlinear_arith)
        requires
            l == b[base] as int + 256 * (b[base + 1] as int) + 65536 * (b[base + 2] as int)
                + 16777216 * (b[base + 3] as int);
    assert(l % 256 == b[base] as int) by (nonlinear_arith)
        requires
            l == b[base] as int + 256 * (b[base + 1] as int) + 65536 * (b[base + 2] as int)
                + 16777216 * (b[base + 3] as int);
}

/// T4: the byte-oriented `conv_rgb` equals the manifest's word-oriented
/// `conv_word` at the sample's own word `sample_word`. This connects the
/// executable's byte reads to the manifest's `Conv(H, word_or_index)`.
pub proof fn lemma_conv_rgb_eq_conv_word(b: Seq<u8>, m: MHRecord, base: int)
    requires
        1 <= m.depth <= 32,
        (m.depth == 16 || m.depth == 24 || m.depth == 32),
        m.compression == 0 || m.compression == 3,
        m.compression == 3 ==> (m.depth == 16 || m.depth == 32),
        base + sample_word_len(m) <= b.len(),
    ensures
        conv_rgb(b, m, base) == conv_word(m, sample_word(b, m, base)),
{
    reveal(conv_rgb);
    reveal(conv_word);
    reveal(sample_word);
    reveal(rgb_seq);
    if m.compression == 3 {
        if m.depth == 16 {
            reveal(bitfield_seq);
            assert(conv_rgb(b, m, base) == bitfield_seq(m, le16(b, base)));
            assert(conv_word(m, sample_word(b, m, base)) == bitfield_seq(m, le16(b, base)));
        } else {
            reveal(bitfield_seq);
            assert(conv_rgb(b, m, base) == bitfield_seq(m, le32(b, base)));
            assert(conv_word(m, sample_word(b, m, base)) == bitfield_seq(m, le32(b, base)));
        }
    } else if m.depth == 16 {
        reveal(rgb555_seq);
        assert(conv_rgb(b, m, base) == rgb555_seq(le16(b, base)));
        assert(conv_word(m, sample_word(b, m, base)) == rgb555_seq(le16(b, base)));
    } else if m.depth == 24 {
        lemma_rgb_byte_three(b, base);
        let w = b[base] as int + 256 * (b[base + 1] as int) + 65536 * (b[base + 2] as int);
        assert(sample_word(b, m, base) == w);
        assert(conv_word(m, w) == rgb_seq(w, 255));
        assert(rgb_seq(w, 255) == seq![rgb_byte(w, 0, 255), rgb_byte(w, 1, 255), rgb_byte(w, 2, 255), 255u8]);
        assert(conv_rgb(b, m, base) == seq![b[base + 2], b[base + 1], b[base], 255u8]);
        assert(seq![rgb_byte(w, 0, 255), rgb_byte(w, 1, 255), rgb_byte(w, 2, 255), 255u8]
            == seq![b[base + 2], b[base + 1], b[base], 255u8]);
    } else {
        assert(m.depth == 32);
        lemma_le32_top_byte(b, base);
        let l = le32(b, base);
        assert(sample_word(b, m, base) == l);
        if is_ico(m.mode) {
            assert(is_ico(m.mode));
            assert(conv_rgb(b, m, base) == seq![b[base + 2], b[base + 1], b[base], b[base + 3]]);
            assert(conv_word(m, l) == rgb_seq(l, (l / 16777216) % 256));
            assert((l / 16777216) % 256 == b[base + 3] as int);
            assert(rgb_seq(l, (l / 16777216) % 256) == seq![b[base + 2], b[base + 1], b[base], b[base + 3]]);
        } else {
            assert(!is_ico(m.mode));
            assert(conv_rgb(b, m, base) == seq![b[base + 2], b[base + 1], b[base], 255u8]);
            assert(conv_word(m, l) == rgb_seq(l, 255));
            assert(rgb_seq(l, 255) == seq![b[base + 2], b[base + 1], b[base], 255u8]);
        }
    }
}

/// The byte length the sample word of a non-indexed pixel occupies.
pub open spec fn sample_word_len(m: MHRecord) -> int {
    if m.compression == 3 {
        if m.depth == 16 {
            2
        } else {
            4
        }
    } else if m.depth == 16 {
        2
    } else if m.depth == 24 {
        3
    } else {
        4
    }
}

/// T4: the exact RGBA of raw sample `x` of the row at byte `base` equals the
/// manifest's `Conv(H, word_or_index)` at the sample's own word (non-indexed) or
/// palette index (indexed), for a validated `MHRecord`, a valid column and a
/// sufficiently long slice. The non-indexed case is
/// `lemma_conv_rgb_eq_conv_word`; a byte read never leaves the checked row.
pub proof fn lemma_raw_conv_eq_conv(
    b: Seq<u8>,
    m: MHRecord,
    base: int,
    x: int,
    row_stride: int,
)
    requires
        1 <= m.depth <= 32,
        1 <= m.width <= 8192,
        0 <= x < m.width,
        m.n == m.width * m.height,
        0 <= m.stride <= 32768,
        m.compression == 0 || m.compression == 3,
        m.compression == 3 ==> (m.depth == 16 || m.depth == 32),
        row_stride == stride_of(m.width, m.depth),
        depth_ok(m.depth) || m.depth == 2,
        base + row_stride <= b.len(),
    ensures
        {
            let v = if m.depth <= 8 {
                sub_index(b[base + x * m.depth / 8], m.depth, x)
            } else {
                sample_word(b, m, base + x * m.depth / 8)
            };
            raw_conv(b, m, base, x) == conv(b, m, v)
        },
{
    reveal(raw_conv);
    reveal(conv);
    if m.depth <= 8 {
        // Paletted: both sides look up the same index `v`.
        lemma_subbyte_within_stride(m.width, m.depth, x);
        let v = sub_index(b[base + x * m.depth / 8], m.depth, x);
        assert(raw_conv(b, m, base, x) == conv(b, m, v));
    } else {
        // The sample word fits inside the stride.
        reveal(depth_ok);
        assert(m.depth == 16 || m.depth == 24 || m.depth == 32);
        assert(m.depth % 8 == 0) by (nonlinear_arith)
            requires m.depth == 16 || m.depth == 24 || m.depth == 32;
        lemma_sample_within_stride(m.width, m.depth, x);
        assert(x * m.depth / 8 == x * (m.depth / 8)) by (nonlinear_arith)
            requires m.depth % 8 == 0, x >= 0, m.depth >= 0;
        assert((x * m.depth / 8) + m.depth / 8 <= stride_of(m.width, m.depth));
        assert(x * m.depth / 8 + m.depth / 8 <= row_stride);
        assert(sample_word_len(m) <= m.depth / 8);
        assert(sample_word_len(m) <= m.depth / 8) by {
            reveal(sample_word_len);
            if m.compression == 3 {
                assert(m.depth == 16 || m.depth == 32);
            } else if m.depth == 24 {
                assert(sample_word_len(m) == 3);
            } else {
                assert(m.depth == 16 || m.depth == 32);
            }
        }
        assert((m.depth / 8) as int == m.depth / 8);
        assert(base + x * m.depth / 8 + sample_word_len(m) <= b.len()) by (nonlinear_arith)
            requires
                base + row_stride <= b.len(),
                x * m.depth / 8 + m.depth / 8 <= row_stride,
                sample_word_len(m) <= m.depth / 8,
                x >= 0;
        lemma_conv_rgb_eq_conv_word(b, m, base + x * m.depth / 8);
        reveal(conv_rgb);
        assert(raw_conv(b, m, base, x) == conv_rgb(b, m, base + x * m.depth / 8));
        assert(conv_rgb(b, m, base + x * m.depth / 8)
            == conv_word(m, sample_word(b, m, base + x * m.depth / 8)));
        assert(conv(b, m, sample_word(b, m, base + x * m.depth / 8))
            == conv_word(m, sample_word(b, m, base + x * m.depth / 8)));
    }
}

// ---------------------------------------------------------------------------
// R9: the orientation map as a cell function.
// ---------------------------------------------------------------------------

/// R9: the output row for source row `r` (top-down identity, bottom-up flip).
pub open spec fn map_row_of(h: int, top_down: bool, r: int) -> int {
    if top_down {
        r
    } else {
        h - 1 - r
    }
}

/// R9: the output cell `map_row*r + x` of source `(x, r)`.
pub open spec fn orient_cell(w: int, h: int, top_down: bool, x: int, r: int) -> int {
    map_row_of(h, top_down, r) * w + x
}

// ---------------------------------------------------------------------------
// The manifest's named lemmas (T4, T5, T11).
// ---------------------------------------------------------------------------

/// `stride_roundup_safe`: `stride_of(w,d) = 4*ceil(w*d/32)` is a nonnegative
/// multiple of four and at most 32,768 for a validated `1 <= w <= 8192`,
/// `1 <= d <= 32`.
pub proof fn stride_roundup_safe(w: int, d: int)
    requires
        1 <= w <= 8192,
        1 <= d <= 32,
    ensures
        stride_of(w, d) == ((w * d + 31) / 32) * 4,
        0 <= stride_of(w, d) <= 32768,
        stride_of(w, d) % 4 == 0,
{
    reveal(stride_of);
    lemma_stride_bound(w, d);
    assert(stride_of(w, d) == ((w * d + 31) / 32) * 4);
    assert(stride_of(w, d) % 4 == 0) by (nonlinear_arith)
        requires
            stride_of(w, d) == ((w * d + 31) / 32) * 4;
}

/// `row_last_sample_within_stride`: an even-byte depth's last sample byte stays
/// inside the padded stride (the manifest name for `lemma_sample_within_stride`).
pub proof fn row_last_sample_within_stride(w: int, d: int, x: int)
    requires
        1 <= w <= 8192,
        1 <= d <= 32,
        d % 8 == 0,
        0 <= x < w,
    ensures
        x * (d / 8) + d / 8 <= stride_of(w, d),
{
    lemma_sample_within_stride(w, d, x);
}

/// `product_width_height_bound`: for validated dimensions the pixel count and
/// the output byte count obey the caps.
pub proof fn product_width_height_bound(w: int, h: int)
    requires
        1 <= w <= 8192,
        1 <= h <= 8192,
        4 * (w * h) <= 67108864,
    ensures
        1 <= w * h <= 16777216,
        4 * (w * h) <= 67108864,
{
    assert(1 <= w * h) by (nonlinear_arith)
        requires 1 <= w, 1 <= h;
    assert(w * h <= 16777216) by (nonlinear_arith)
        requires 4 * (w * h) <= 67108864, 1 <= w, 1 <= h;
}

/// `pixel_byte_offset_bound`: the oriented RGBA cell of a source `(x, r)` lies
/// inside `4*N`.
pub proof fn pixel_byte_offset_bound(w: int, h: int, top_down: bool, x: int, r: int)
    requires
        1 <= w <= 8192,
        1 <= h <= 8192,
        0 <= x < w,
        0 <= r < h,
    ensures
        0 <= orient_cell(w, h, top_down, x, r) < w * h,
        4 * orient_cell(w, h, top_down, x, r) + 4 <= 4 * (w * h),
{
    reveal(orient_cell);
    reveal(map_row_of);
    if top_down {
        assert(0 <= r < h);
    } else {
        assert(0 <= h - 1 - r < h) by (nonlinear_arith)
            requires 0 <= r < h;
    }
    assert(0 <= orient_cell(w, h, top_down, x, r) < w * h) by (nonlinear_arith)
        requires
            0 <= map_row_of(h, top_down, r) < h,
            0 <= x < w,
            orient_cell(w, h, top_down, x, r) == map_row_of(h, top_down, r) * w + x,
            w >= 1,
    ;
    assert(4 * orient_cell(w, h, top_down, x, r) + 4 <= 4 * (w * h)) by (nonlinear_arith)
        requires 0 <= orient_cell(w, h, top_down, x, r) < w * h;
}

/// `bit_mask_contiguous_extract`: a nonzero mask's extracted component is at
/// most `max = mask >> shift` and the mask's low set bit is `shift`.
pub proof fn bit_mask_contiguous_extract(mask: u32, word: u32)
    requires
        mask != 0u32,
    ensures
        (mask >> mask.trailing_zeros()) & 1u32 == 1u32,
        (mask >> mask.trailing_zeros()) != 0u32,
        ((word & mask) >> mask.trailing_zeros()) <= (mask >> mask.trailing_zeros()),
{
    broadcast use vstd::std_specs::bits::axiom_u32_trailing_zeros;

    use vstd::std_specs::bits::u32_trailing_zeros;
    assert(mask.trailing_zeros() == u32_trailing_zeros(mask));
    assert(mask.trailing_zeros() < 32u32);
    let shift = mask.trailing_zeros();
    assert((mask >> shift) & 1u32 == 1u32);
    assert((mask >> shift) != 0u32) by (bit_vector)
        requires (mask >> shift) & 1u32 == 1u32;
    assert((word & mask) <= mask) by (bit_vector);
    assert(((word & mask) >> shift) <= (mask >> shift)) by (bit_vector)
        requires
            (word & mask) <= mask;
}

/// `scale_round_bounds`: `floor((255*c + floor(max/2))/max) <= 255` and the
/// intermediate `255*c + floor(max/2)` is nonnegative, for `0 <= c <= max`,
/// `1 <= max <= 2^32`.
pub proof fn scale_round_bounds(c: int, max: int)
    requires
        0 <= c <= max,
        1 <= max,
    ensures
        0 <= scale_of(c, max) <= 255,
        0 <= 255 * c + max / 2 <= 255 * max + max / 2,
{
    reveal(scale_of);
    assert(0 <= 255 * c + max / 2) by (nonlinear_arith)
        requires 0 <= c, 0 <= max;
    assert(255 * c + max / 2 <= 255 * max + max / 2) by (nonlinear_arith)
        requires c <= max, max >= 0;
    assert(scale_of(c, max) <= 255) by (nonlinear_arith)
        requires
            scale_of(c, max) == (255 * c + max / 2) / max,
            c <= max,
            max >= 1,
            max / 2 >= 0;
    assert(scale_of(c, max) >= 0) by (nonlinear_arith)
        requires
            scale_of(c, max) == (255 * c + max / 2) / max,
            255 * c + max / 2 >= 0,
            max >= 1;
}

/// `palette_stride_bounds`: a validated palette stride is 3 or 4, and palette
/// entry `ix < count` reads inside the borrowed slice.
pub proof fn palette_stride_bounds(
    b: Seq<u8>,
    palette_start: int,
    pal_stride: int,
    count: int,
    ix: int,
)
    requires
        count >= 0,
        (pal_stride == 3 || pal_stride == 4),
        palette_start + count * pal_stride <= b.len(),
        0 <= ix < count,
    ensures
        palette_start + ix * pal_stride + 3 <= b.len(),
{
    assert(ix * pal_stride + pal_stride <= count * pal_stride) by (nonlinear_arith)
        requires ix < count, pal_stride >= 3;
    assert(palette_start + ix * pal_stride + 3 <= b.len()) by (nonlinear_arith)
        requires
            palette_start + count * pal_stride <= b.len(),
            ix * pal_stride + pal_stride <= count * pal_stride,
            pal_stride >= 3;
}

/// `rle_monotone_position`: after an RLE command that leaves `(x, y)`, the
/// position stays in `[0,w] x [0,h]`, `y` does not decrease, and the
/// lexicographic key `(y, x)` does not decrease. Covers a run/EOL/delta.
pub proof fn rle_monotone_position(
    w: int,
    h: int,
    x: int,
    y: int,
    q: int,
    dx: int,
    dy: int,
    kind: int,
)
    requires
        1 <= w,
        1 <= h,
        0 <= x <= w,
        0 <= y <= h,
        q >= 0,
        dx >= 0,
        dy >= 0,
        // kind 0: run of q pixels (row bounds); 1: EOL; 2: delta; 3: no move
        kind == 0 ==> (y < h && x + q <= w && q >= 0),
        kind == 1 ==> (y < h),
        kind == 2 ==> (x + dx < w && y + dy < h),
    ensures
        {
            let x2 = if kind == 0 {
                x + q
            } else if kind == 1 {
                0
            } else if kind == 2 {
                x + dx
            } else {
                x
            };
            let y2 = if kind == 1 {
                y + 1
            } else if kind == 2 {
                y + dy
            } else {
                y
            };
            &&& 0 <= x2 <= w
            &&& 0 <= y2 <= h
            &&& y2 >= y
            &&& (y2 > y || x2 >= x)
        },
{
    reveal(map_row_of);
    if kind == 0 {
        assert(0 <= x + q <= w) by (nonlinear_arith)
            requires x + q <= w, q >= 0, x >= 0;
        assert(y >= y);
        assert(y > y || x + q >= x) by (nonlinear_arith)
            requires q >= 0;
    } else if kind == 1 {
        assert(y + 1 <= h) by (nonlinear_arith)
            requires y < h;
        assert(0 <= y + 1 <= h) by (nonlinear_arith)
            requires y < h, y >= 0;
        assert(y + 1 > y);
    } else if kind == 2 {
        assert(x + dx < w);
        assert(y + dy < h);
        assert(0 <= x + dx <= w) by (nonlinear_arith)
            requires x + dx < w, dx >= 0, x >= 0;
        assert(0 <= y + dy <= h) by (nonlinear_arith)
            requires y + dy < h, dy >= 0, y >= 0;
        assert(y + dy >= y) by (nonlinear_arith)
            requires dy >= 0;
        assert(y + dy > y || x + dx >= x) by (nonlinear_arith)
            requires dx >= 0, dy >= 0;
    } else {
        assert(0 <= x <= w);
        assert(0 <= y <= h);
    }
}

/// `vertical_flip_bijection`: the row-orientation map is a bijection on
/// `[0, N)`. The flip `row -> flip(row)` has the inverse `flip(row)` for both
/// the top-down identity and the bottom-up flip `h-1-r`, so the composed cell
/// map `r*w + x` is injective; every cell `q < w*h` is hit by the source
/// `(q % w, inv(q / w))`.
pub proof fn vertical_flip_bijection(
    w: int,
    h: int,
    td: bool,
    x1: int,
    r1: int,
    x2: int,
    r2: int,
    q: int,
)
    requires
        1 <= w <= 8192,
        1 <= h <= 8192,
        0 <= x1 < w,
        0 <= r1 < h,
        0 <= x2 < w,
        0 <= r2 < h,
        0 <= q < w * h,
    ensures
        orient_cell(w, h, td, x1, r1) == orient_cell(w, h, td, x2, r2) <==> (x1 == x2 && r1 == r2),
        0 <= orient_cell(w, h, td, x1, r1) < w * h,
        exists|x: int, r: int|
            0 <= x < w && 0 <= r < h && orient_cell(w, h, td, x, r) == q,
{
    reveal(orient_cell);
    reveal(map_row_of);
    // Range of the map.
    pixel_byte_offset_bound(w, h, td, x1, r1);
    // Injectivity: reduce to the row flip's injectivity and coordinates.
    assert((map_row_of(h, td, r1) * w + x1 == map_row_of(h, td, r2) * w + x2)
        <==> (map_row_of(h, td, r1) == map_row_of(h, td, r2) && x1 == x2)) by (nonlinear_arith)
        requires
            0 <= x1 < w,
            0 <= x2 < w,
            map_row_of(h, td, r1) >= 0,
            map_row_of(h, td, r2) >= 0,
            w >= 1,
    ;
    if td {
        assert(map_row_of(h, td, r1) == r1);
        assert(map_row_of(h, td, r2) == r2);
    } else {
        assert(map_row_of(h, td, r1) == h - 1 - r1);
        assert(map_row_of(h, td, r2) == h - 1 - r2);
        assert((h - 1 - r1 == h - 1 - r2) <==> r1 == r2) by (nonlinear_arith);
    }
    assert(orient_cell(w, h, td, x1, r1) == orient_cell(w, h, td, x2, r2) <==> (x1 == x2 && r1
        == r2));
    // Surjectivity: witness `(q % w, inv(q / w))`.
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod(q, w);
    let qx: int = q % w;
    let qr: int = q / w;
    assert(qx == q % w);
    assert(qr == q / w);
    assert(q == w * qr + qx);
    assert(q == qr * w + qx) by (nonlinear_arith)
        requires q == w * qr + qx;
    assert(0 <= qx < w);
    assert(0 <= qr < h) by (nonlinear_arith)
        requires
            q == qr * w + qx,
            0 <= qx < w,
            q < w * h,
            w >= 1,
            q >= 0,
    ;
    if td {
        assert(orient_cell(w, h, td, qx, qr) == qr * w + qx) by (nonlinear_arith)
            requires
                orient_cell(w, h, td, qx, qr) == map_row_of(h, td, qr) * w + qx,
                map_row_of(h, td, qr) == qr;
        assert(orient_cell(w, h, td, qx, qr) == q);
        assert(exists|x: int, r: int|
            0 <= x < w && 0 <= r < h && orient_cell(w, h, td, x, r) == q) by {
            let x: int = qx;
            let r: int = qr;
            assert(0 <= x < w && 0 <= r < h && orient_cell(w, h, td, x, r) == q);
        }
    } else {
        let rr: int = h - 1 - qr;
        assert(rr == h - 1 - qr);
        assert(0 <= rr < h) by (nonlinear_arith)
            requires rr == h - 1 - qr, 0 <= qr < h;
        assert(map_row_of(h, td, rr) == h - 1 - rr);
        assert(h - 1 - rr == qr) by (nonlinear_arith)
            requires rr == h - 1 - qr;
        assert(orient_cell(w, h, td, qx, rr) == qr * w + qx) by (nonlinear_arith)
            requires
                orient_cell(w, h, td, qx, rr) == map_row_of(h, td, rr) * w + qx,
                map_row_of(h, td, rr) == qr;
        assert(orient_cell(w, h, td, qx, rr) == q);
        assert(exists|x: int, r: int|
            0 <= x < w && 0 <= r < h && orient_cell(w, h, td, x, r) == q) by {
            let x: int = qx;
            let r: int = rr;
            assert(0 <= x < w && 0 <= r < h && orient_cell(w, h, td, x, r) == q);
        }
    }
}

/// `rle_painted_at_most_N`: the paint counter `painted` never exceeds the
/// number of source rows already started times `w` plus the in-row run
/// position, and the whole-image bound is `N = w*h`.
#[allow(non_snake_case)]
pub proof fn rle_painted_at_most_N(w: int, h: int, painted: int, x: int, y: int)
    requires
        1 <= w,
        1 <= h,
        0 <= x <= w,
        0 <= y <= h,
        painted <= y * w + x,
        y == h ==> x == 0,
    ensures
        painted <= w * h,
{
    assert(y * w + x <= w * h) by (nonlinear_arith)
        requires
            0 <= x <= w,
            0 <= y <= h,
            y == h ==> x == 0,
            w >= 0;
    assert(painted <= w * h) by (nonlinear_arith)
        requires painted <= y * w + x, y * w + x <= w * h;
}

/// The per-command step of `rle_painted_at_most_N`: painting `q` pixels at
/// `(x, y)` with `x + q <= w` preserves the invariant.
pub proof fn rle_paint_step(w: int, painted: int, x: int, y: int, q: int)
    requires
        q >= 0,
        x + q <= w,
        painted <= y * w + x,
    ensures
        painted + q <= y * w + (x + q),
{
    assert(painted + q <= y * w + x + q) by (nonlinear_arith)
        requires painted <= y * w + x;
    assert(y * w + x + q == y * w + (x + q)) by (nonlinear_arith);
}

// ---------------------------------------------------------------------------
// T11: the RLE command/absolute-packet parser's termination, at the model level.
// The manifest asks for an explicit decreasing variant for the command loop
// (`|b| - cursor`) and a nested `q - j` for the <= 255 emitted pixels.
// ---------------------------------------------------------------------------

/// R11: the RLE payload byte count for a run/absolute command of `q` pixels at
/// depth 8 (one byte each) or depth 4 (`ceil(q/2)` nibble bytes).
pub open spec fn rle_payload(q: int, depth: int) -> int {
    if depth == 8 {
        q
    } else {
        (q + 1) / 2
    }
}

/// R11: the padded payload, one extra pad byte when the payload is odd.
pub open spec fn rle_padded(q: int, depth: int) -> int {
    rle_payload(q, depth) + rle_payload(q, depth) % 2
}

/// R10/R11: the bytes consumed by the *complete* command at cursor `p` of a
/// depth-4 or depth-8 stream, in R10/R11 order. `0` means the command is
/// incomplete (a shorter stream), which the caller rejects before reading any
/// payload byte. EOB and EOL consume two command bytes; delta consumes four; a
/// run consumes two; an absolute packet consumes `2 + padded`.
pub open spec fn rle_cmd_len(b: Seq<u8>, depth: int, p: int) -> int {
    if p + 2 > b.len() {
        0
    } else if b[p] != 0 {
        2
    } else if b[p + 1] == 1 || b[p + 1] == 0 {
        2
    } else if b[p + 1] == 2 {
        if p + 4 > b.len() {
            0
        } else {
            4
        }
    } else {
        let q = b[p + 1] as int;
        let padded = rle_padded(q, depth);
        if p + 2 + padded > b.len() {
            0
        } else {
            2 + padded
        }
    }
}

/// Whether the command at `p` is an explicit `00 01` EOB (the loop's halt).
pub open spec fn rle_is_eob(b: Seq<u8>, p: int) -> bool {
    p + 2 <= b.len() && b[p] == 0 && b[p + 1] == 1
}

/// The number of complete commands processed from `p` until an incomplete
/// command, an EOB, or the end. Recursive on the strictly decreasing measure
/// `b.len() - p`: every successful command consumes at least two bytes, so the
/// recursion terminates. This is the model-level variant `|b| - cursor`.
pub open spec fn rle_cmd_count(b: Seq<u8>, depth: int, p: int) -> int
    decreases b.len() - p,
{
    if rle_cmd_len(b, depth, p) == 0 || rle_is_eob(b, p) {
        0
    } else {
        1 + rle_cmd_count(b, depth, p + rle_cmd_len(b, depth, p))
    }
}

/// T11: a complete command consumes at least two bytes and stays inside the
/// slice; hence the command cursor strictly increases and the measure `|b| - p`
/// strictly decreases. Covers EOB, EOL, zero-delta and the absolute packet.
pub proof fn lemma_rle_cmd_progress(b: Seq<u8>, depth: int, p: int)
    requires
        depth == 4 || depth == 8,
        0 <= p,
        rle_cmd_len(b, depth, p) != 0,
    ensures
        rle_cmd_len(b, depth, p) >= 2,
        p + rle_cmd_len(b, depth, p) > p,
        p + rle_cmd_len(b, depth, p) <= b.len(),
{
    reveal(rle_cmd_len);
    reveal(rle_padded);
    reveal(rle_payload);
    if p + 2 > b.len() {
        assert(false);
    } else if b[p] != 0 {
        assert(rle_cmd_len(b, depth, p) == 2);
    } else if b[p + 1] == 1 || b[p + 1] == 0 {
        assert(rle_cmd_len(b, depth, p) == 2);
    } else if b[p + 1] == 2 {
        assert(rle_cmd_len(b, depth, p) == 4);
    } else {
        let q = b[p + 1] as int;
        let padded = rle_padded(q, depth);
        assert(q >= 3);
        assert(rle_payload(q, depth) >= 1) by (nonlinear_arith)
            requires depth == 4 || depth == 8, q >= 3;
        assert(padded >= 1) by (nonlinear_arith)
            requires padded == rle_payload(q, depth) + rle_payload(q, depth) % 2,
                rle_payload(q, depth) >= 1,
                rle_payload(q, depth) % 2 >= 0;
        assert(rle_cmd_len(b, depth, p) == 2 + padded);
        assert(p + 2 + padded <= b.len());
    }
}

/// T11: the model command loop's variant `|b| - p` strictly decreases over any
/// number of commands: the cursor after `n` complete commands is `>= p + 2n`.
pub proof fn lemma_rle_cmd_count_bound(b: Seq<u8>, depth: int, p: int)
    requires
        depth == 4 || depth == 8,
        0 <= p <= b.len(),
    ensures
        p + 2 * rle_cmd_count(b, depth, p) <= b.len(),
        rle_cmd_count(b, depth, p) <= (b.len() - p) / 2,
    decreases b.len() - p,
{
    reveal(rle_cmd_count);
    if rle_cmd_len(b, depth, p) == 0 || rle_is_eob(b, p) {
        assert(rle_cmd_count(b, depth, p) == 0);
        assert(rle_cmd_count(b, depth, p) <= (b.len() - p) / 2);
    } else {
        lemma_rle_cmd_progress(b, depth, p);
        let step = rle_cmd_len(b, depth, p);
        assert(p + step > p);
        assert(p + step >= 0) by (nonlinear_arith)
            requires p >= 0, step >= 2;
        lemma_rle_cmd_count_bound(b, depth, p + step);
        assert((p + step) + 2 * rle_cmd_count(b, depth, p + step) <= b.len());
        assert(rle_cmd_count(b, depth, p) == 1 + rle_cmd_count(b, depth, p + step));
        assert(p + 2 * rle_cmd_count(b, depth, p) <= b.len()) by (nonlinear_arith)
            requires
                rle_cmd_count(b, depth, p) == 1 + rle_cmd_count(b, depth, p + step),
                (p + step) + 2 * rle_cmd_count(b, depth, p + step) <= b.len(),
                step >= 2,
                p + step > p;
        assert(rle_cmd_count(b, depth, p) <= (b.len() - p) / 2) by (nonlinear_arith)
            requires p + 2 * rle_cmd_count(b, depth, p) <= b.len();
    }
}

/// T11: the nested emitted-pixel loop. For a run of `q <= 255` pixels the
/// variant `q - j` strictly decreases; this is the `q - j` measure with
/// `q <= 255` bounding the inner loop's iterations.
pub proof fn lemma_rle_paint_terminates(q: int, j: int)
    requires
        0 <= j <= q <= 255,
    ensures
        q - j >= 0,
        j < q ==> q - (j + 1) < q - j,
{
    if j < q {
        assert(q - (j + 1) < q - j) by (nonlinear_arith);
    }
}

/// T11: after an RLE command the loop position `(y, x)` maintains `x <= w`,
/// `y <= h`; the command/absolute/EOB parsers reject an incomplete command
/// before reading any payload byte (`rle_cmd_len(b, depth, p) == 0` implies no
/// payload read), so no zero-progress packet is retried.
pub proof fn lemma_rle_cmd_no_retry(b: Seq<u8>, depth: int, p: int)
    requires
        depth == 4 || depth == 8,
        0 <= p,
    ensures
        rle_cmd_len(b, depth, p) == 0 ==> p + 2 > b.len() || b[p + 1] == 2 || p + 2 + rle_padded(
            b[p + 1] as int,
            depth,
        ) > b.len(),
{
    reveal(rle_cmd_len);
}

/// T3's success bounds, as a predicate over an `MHRecord`.
pub open spec fn mh_bounds(m: MHRecord) -> bool {
    &&& 1 <= m.width <= max_dimension()
    &&& 1 <= m.height <= max_dimension()
    &&& m.n == m.width * m.height
    &&& 1 <= m.n <= 16777216
    &&& 4 * m.n <= max_output()
    &&& 0 <= m.stride <= 32768
}

/// The success bounds hold for every `MHRecord` that `MH` returns. This is the
/// arithmetic content of T2/T3, proved from the model alone (no executable
/// code): the guard chain's width/height rules and the output cap.
pub proof fn lemma_mh_ok_bounds(b: Seq<u8>, mode: Mode, m: MHRecord)
    requires
        mh(b, mode) == Ok(m),
    ensures
        mh_bounds(m),
{
    reveal(mh);
    reveal(r2r7);
    reveal(r1_prefix);
    reveal(mh_bounds);
    reveal(visible_height_of);
    reveal(stride_of);
    reveal(pow2);
    reveal(abs_int);
    reveal(depth_ok);
    reveal(hsize_ok);
    reveal(within_bits);
    reveal(contiguous);
    reveal(low_of);
    reveal(masks_ok);
    assert(m.stride == 0 || m.stride == stride_of(m.width, m.depth));
    if m.stride != 0 {
        lemma_stride_bound(m.width, m.depth);
    }
    assert(1 <= m.width <= max_dimension());
    assert(1 <= m.height <= max_dimension());
    assert(m.n == m.width * m.height);
    assert(1 <= m.n) by (nonlinear_arith)
        requires
            m.n == m.width * m.height,
            1 <= m.width,
            1 <= m.height,
    ;
    assert(4 * m.n <= max_output());
    assert(m.n <= 16777216) by (nonlinear_arith)
        requires
            m.n == m.width * m.height,
            1 <= m.width,
            1 <= m.height,
            4 * m.n <= max_output(),
    ;
}

/// The same bounds for a record built by `r6r7` from validated fields (the
/// split form the executable uses), with no reference to the R1 prefix.
pub proof fn lemma_r6r7_ok_bounds(
    b: Seq<u8>,
    mode: Mode,
    s: int,
    file_offset: int,
    is_bmp: bool,
    rf: RawFields,
    m: MHRecord,
)
    requires
        r2r5(b, mode, s) == Ok(rf),
        r6r7(b, mode, s, file_offset, is_bmp, rf) == Ok(m),
    ensures
        mh_bounds(m),
{
    reveal(r6r7);
    reveal(r2r5);
    assert(1 <= rf.width <= max_dimension());
    assert(1 <= rf.height <= max_dimension());
    assert(4 * (rf.width * rf.height) <= max_output());
    assert(m.width == rf.width);
    assert(m.height == rf.height);
    assert(m.n == rf.width * rf.height);
    reveal(mh_bounds);
    reveal(visible_height_of);
    reveal(stride_of);
    reveal(pow2);
    reveal(abs_int);
    reveal(depth_ok);
    reveal(within_bits);
    reveal(contiguous);
    reveal(low_of);
    reveal(masks_ok);
    assert(m.stride == 0 || m.stride == stride_of(m.width, m.depth));
    if m.stride != 0 {
        lemma_stride_bound(m.width, m.depth);
    }
    assert(1 <= m.width <= max_dimension());
    assert(1 <= m.height <= max_dimension());
    assert(m.n == m.width * m.height);
    assert(1 <= m.n) by (nonlinear_arith)
        requires
            m.n == m.width * m.height,
            1 <= m.width,
            1 <= m.height,
    ;
    assert(4 * m.n <= max_output());
    assert(m.n <= 16777216) by (nonlinear_arith)
        requires
            m.n == m.width * m.height,
            1 <= m.width,
            1 <= m.height,
            4 * m.n <= max_output(),
    ;
    assert(0 <= m.stride <= 32768);
}

/// The same bounds for a record built by `r6r7_policy` from `r2r5_policy`'s
/// validated fields: the geometry rules are `r2r5`'s own, so the bounds are
/// unchanged by any option.
pub proof fn lemma_r6r7_policy_ok_bounds(
    b: Seq<u8>,
    mode: Mode,
    s: int,
    file_offset: int,
    is_bmp: bool,
    rf: RawFields,
    p: Policy,
    m: MHRecord,
)
    requires
        r2r5_policy(b, mode, s, p) == Ok(rf),
        r6r7_policy(b, mode, s, file_offset, is_bmp, rf, p) == Ok(m),
    ensures
        mh_bounds(m),
{
    reveal(r6r7_policy);
    reveal(r6r7_result_policy);
    reveal(r2r5_policy);
    reveal(r2r5);
    reveal(hsize_ok_policy);
    reveal(hsize_ok);
    lemma_r2r5_policy_strict(b, mode, s);
    assert(1 <= rf.width <= max_dimension());
    assert(1 <= rf.height <= max_dimension());
    assert(4 * (rf.width * rf.height) <= max_output());
    assert(m.width == rf.width);
    assert(m.height == rf.height);
    assert(m.n == rf.width * rf.height);
    assert(m.stride == 0 || m.stride == stride_of(m.width, m.depth));
    assert(1 <= m.width <= max_dimension());
    assert(1 <= m.height <= max_dimension());
    assert(1 <= m.n) by (nonlinear_arith)
        requires
            m.n == m.width * m.height,
            1 <= m.width,
            1 <= m.height,
    ;
    assert(4 * m.n <= max_output());
    assert(m.n <= 16777216) by (nonlinear_arith)
        requires
            m.n == m.width * m.height,
            1 <= m.width,
            1 <= m.height,
            4 * m.n <= max_output(),
    ;
    reveal(mh_bounds);
    reveal(stride_of);
    reveal(visible_height_of);
    if m.stride != 0 {
        lemma_stride_bound(m.width, m.depth);
    }
    assert(0 <= m.stride <= 32768);
}

/// `r2r7_policy(Strict)` agrees with `r2r7` on success: the same record.
pub proof fn lemma_r2r7_policy_bridge(b: Seq<u8>, mode: Mode, s: int, file_offset: int, is_bmp: bool, m: MHRecord)
    requires
        r2r7(b, mode, s, file_offset, is_bmp) == Ok(m),
    ensures
        r2r7_policy(b, mode, s, file_offset, is_bmp, crate::dib::strict_policy()) == Ok(m),
{
    reveal(r2r7);
    reveal(r2r7_policy);
    lemma_r2r7_policy_strict(b, mode, s, file_offset, is_bmp);
}

/// `r2r5_policy`'s success gives the R2–R5 geometry facts the R7 record build
/// relies on: the width/height caps, the output cap and the `pos0` bound.
pub proof fn lemma_r2r5_policy_ok(b: Seq<u8>, mode: Mode, s: int, p: Policy, rf: RawFields)
    requires
        r2r5_policy(b, mode, s, p) == Ok(rf),
    ensures
        1 <= rf.width <= max_dimension(),
        1 <= rf.height <= max_dimension(),
        4 * (rf.width * rf.height) <= max_output(),
        rf.pos0 == s + rf.hsize,
        depth_ok(rf.depth) || rf.depth == 2,
        b.len() >= s + header_bytes_needed(rf.hsize, p),
        p.header_bytes == crate::dib::HeaderBytesOptions::Full ==> b.len() >= rf.pos0,
        rf.compression >= 0 && rf.compression <= 3,
{
    reveal(r2r5_policy);
    reveal(r2r5);
    reveal(hsize_ok_policy);
    reveal(hsize_ok);
    reveal(depth_ok);
    reveal(visible_height_of);
}

/// The `forall` form, so the solver can apply the bounds wherever
/// `mh(b,mode) == Ok(m)` is in context.
pub proof fn lemma_mh_ok_bounds_auto(b: Seq<u8>, mode: Mode)
    ensures
        forall|m: MHRecord| #[trigger] mh(b, mode) == Ok(m) ==> mh_bounds(m),
{
    assert forall|m: MHRecord| #[trigger] mh(b, mode) == Ok(m) implies mh_bounds(m) by {
        lemma_mh_ok_bounds(b, mode, m);
    }
}

/// A single pixel write: the output cell and the four RGBA bytes. The model
/// records writes in order so the executable's buffer can be compared to the
/// model's by replaying them onto the initial fill; this is what "the pixels
/// written so far tied to the model's partial output" means concretely.
pub struct Write {
    pub cell: int,
    pub px: Seq<u8>,
}

/// A four-byte RGBA quadruple at byte offset `off`.
pub open spec fn quad_at(out: Seq<u8>, off: int, c: Seq<u8>) -> bool {
    &&& out[off] == c[0]
    &&& out[off + 1] == c[1]
    &&& out[off + 2] == c[2]
    &&& out[off + 3] == c[3]
}

/// The output cell of a pixel written at source `(x, y)` — the same oriented
/// cell the executable's `out_offset` returns.
pub open spec fn cell_of(m: MHRecord, x: int, y: int) -> int {
    orient_cell(m.width, m.height, m.top_down, x, y)
}

/// Byte-level image state: `o` is the `4*n`-byte buffer, `painted` the set of
/// cells written so far (an exact byte-level union of the two write lists).
pub struct Img {
    pub o: Seq<u8>,
    pub painted: Seq<Write>,
}

/// A well-formed image state for an `n`-pixel record whose fill is `fill_bytes`.
pub open spec fn well_formed(img: Img, n: int, fill_bytes: Seq<u8>) -> bool {
    &&& img.o.len() == 4 * n
    &&& fill_bytes.len() == 4 * n
    &&& forall|k: int| 0 <= k < img.painted.len() ==> #[trigger] img.painted[k].cell >= 0
        && img.painted[k].cell < n && img.painted[k].px.len() == 4
    &&& img.o == bytes_replay(fill_bytes, img.painted)
}

/// R8's zero-initialized allocation as raw bytes (`vec![0u8; k]`).
pub open spec fn zero_bytes(k: int) -> Seq<u8> {
    Seq::new(k as nat, |_i: int| 0u8)
}

/// R8/R10: the palette-entry-zero fill of the RLE output, as raw bytes: pixel
/// `q` is `pal_entry(0)` at byte offset `4q`.
pub open spec fn pal_zero_bytes(b: Seq<u8>, m: MHRecord) -> Seq<u8> {
    Seq::new((4 * m.n) as nat, |i: int| pal_entry(
        b,
        m.palette_start,
        m.palette_stride,
        0,
    )[i % 4])
}

/// The four byte updates of one write, on a buffer.
pub open spec fn replay_one(base: Seq<u8>, w: Write) -> Seq<u8> {
    replay_bytes_at(base, 4 * w.cell, w.px)
}

/// The four byte updates at byte offset `o` (the executable's `put`).
pub open spec fn replay_bytes_at(base: Seq<u8>, o: int, px: Seq<u8>) -> Seq<u8> {
    base.update(o, px[0]).update(o + 1, px[1]).update(o + 2, px[2]).update(o + 3, px[3])
}

/// Byte-level replay of a write list onto a byte buffer: each write sets the
/// four bytes `4*cell .. 4*cell+4`. This is exactly the executable's `put`
/// (`out.set` four times), so the ghost accumulator matches the concrete buffer
/// byte for byte.
pub open spec fn bytes_replay(base: Seq<u8>, ws: Seq<Write>)
    -> (r: Seq<u8>)
    decreases ws.len(),
{
    if ws.len() == 0 {
        base
    } else {
        bytes_replay(replay_one(base, ws[0]), ws.drop_first())
    }
}

/// One write appended to the list is the four byte updates on the replay of the
/// shorter list: the executable's ghost accumulator extends by one `put`.
pub proof fn lemma_bytes_replay_push(base: Seq<u8>, ws: Seq<Write>, w: Write)
    ensures
        bytes_replay(base, ws.push(w)) == replay_one(bytes_replay(base, ws), w),
    decreases ws.len(),
{
    reveal(bytes_replay);
    reveal(replay_one);
    if ws.len() == 0 {
        broadcast use vstd::seq_lib::group_seq_lib_default;
        broadcast use vstd::seq::group_seq_lemmas;
        assert(ws == Seq::<Write>::empty()) by {
            broadcast use vstd::seq_lib::group_seq_lib_default;
        }
        assert(ws.push(w) == seq![w]);
        assert(seq![w][0] == w);
        assert(seq![w].drop_first() == Seq::<Write>::empty());
        assert(bytes_replay(base, ws.push(w)) == bytes_replay(replay_one(base, seq![w][0]), seq![w].drop_first()));
        assert(bytes_replay(base, ws.push(w)) == replay_one(base, w));
        assert(bytes_replay(base, ws) == base);
    } else {
        let b4 = replay_one(base, ws[0]);
        assert((ws.push(w)).drop_first() == ws.drop_first().push(w)) by {
            broadcast use vstd::seq_lib::group_seq_lib_default;
        }
        assert(bytes_replay(base, ws) == bytes_replay(b4, ws.drop_first()));
        assert(bytes_replay(base, ws.push(w)) == bytes_replay(b4, ws.drop_first().push(w)));
        lemma_bytes_replay_push(b4, ws.drop_first(), w);
    }
}

/// `replay_bytes_at` changes only the four bytes `o .. o+4`.
pub proof fn lemma_replay_bytes_at_other(base: Seq<u8>, o: int, px: Seq<u8>, idx: int)
    requires
        idx < o || idx >= o + 4,
    ensures
        replay_bytes_at(base, o, px)[idx] == base[idx],
{
    reveal(replay_bytes_at);
    let b1 = base.update(o, px[0]);
    let b2 = b1.update(o + 1, px[1]);
    let b3 = b2.update(o + 2, px[2]);
    vstd::seq::lemma_seq_update_different(base, idx, o, px[0]);
    vstd::seq::lemma_seq_update_different(b1, idx, o + 1, px[1]);
    vstd::seq::lemma_seq_update_different(b2, idx, o + 2, px[2]);
    vstd::seq::lemma_seq_update_different(b3, idx, o + 3, px[3]);
}

/// `bytes_replay` preserves length.
pub proof fn lemma_bytes_replay_len(base: Seq<u8>, ws: Seq<Write>)
    ensures
        bytes_replay(base, ws).len() == base.len(),
    decreases ws.len(),
{
    reveal(bytes_replay);
    reveal(replay_one);
    if ws.len() == 0 {
    } else {
        let o = 4 * ws[0].cell;
        let b1 = base.update(o, ws[0].px[0]);
        let b2 = b1.update(o + 1, ws[0].px[1]);
        let b3 = b2.update(o + 2, ws[0].px[2]);
        let b4 = b3.update(o + 3, ws[0].px[3]);
        vstd::seq::lemma_seq_update_len(base, o, ws[0].px[0]);
        vstd::seq::lemma_seq_update_len(b1, o + 1, ws[0].px[1]);
        vstd::seq::lemma_seq_update_len(b2, o + 2, ws[0].px[2]);
        vstd::seq::lemma_seq_update_len(b3, o + 3, ws[0].px[3]);
        assert(replay_one(base, ws[0]) == b4);
        lemma_bytes_replay_len(b4, ws.drop_first());
    }
}

/// `bytes_replay` of a concatenation is the replay of the replayed prefix:
/// `bytes_replay(bytes_replay(o, a), c) == bytes_replay(o, a + c)`.
pub proof fn lemma_bytes_replay_assoc(o: Seq<u8>, a: Seq<Write>, c: Seq<Write>)
    ensures
        bytes_replay(bytes_replay(o, a), c) == bytes_replay(o, a + c),
    decreases a.len(),
{
    reveal(bytes_replay);
    reveal(replay_one);
    if a.len() == 0 {
        broadcast use vstd::seq_lib::group_seq_lib_default;
        assert(Seq::<Write>::empty() + c == c) by {
            broadcast use vstd::seq_lib::group_seq_lib_default;
        }
    } else {
        let b4 = replay_one(o, a[0]);
        assert((a + c).drop_first() == a.drop_first() + c) by {
            broadcast use vstd::seq_lib::group_seq_lib_default;
        }
        assert(bytes_replay(o, a) == bytes_replay(b4, a.drop_first()));
        lemma_bytes_replay_assoc(b4, a.drop_first(), c);
    }
}

/// A buffer that differs from another only at an index outside `[0, 4n)` is the
/// same on the whole image.
pub proof fn lemma_no_paints_ok(o: Seq<u8>, n: int, fill_bytes: Seq<u8>)
    requires
        o.len() == 4 * n,
        fill_bytes.len() == 4 * n,
        o == fill_bytes,
    ensures
        well_formed(
            Img { o: o, painted: Seq::<Write>::empty() },
            n,
            fill_bytes,
        ),
{
    reveal(well_formed);
    reveal(bytes_replay);
}

/// The executable's `put` extends the ghost write accumulator by one write:
/// after writing cell `w.cell`'s four bytes, the state is well-formed for
/// `painted.push(w)`.
pub proof fn lemma_well_formed_put(
    o: Seq<u8>,
    o_new: Seq<u8>,
    painted: Seq<Write>,
    n: int,
    fill_bytes: Seq<u8>,
    w: Write,
)
    requires
        well_formed(Img { o: o, painted: painted }, n, fill_bytes),
        0 <= w.cell < n,
        w.px.len() == 4,
        o_new == replay_one(o, w),
    ensures
        well_formed(Img { o: o_new, painted: painted.push(w) }, n, fill_bytes),
{
    reveal(well_formed);
    reveal(bytes_replay);
    reveal(replay_one);
    lemma_bytes_replay_push(fill_bytes, painted, w);
    assert(o == bytes_replay(fill_bytes, painted));
    assert(bytes_replay(fill_bytes, painted.push(w)) == replay_one(o, w));
    assert(bytes_replay(fill_bytes, painted.push(w)) == o_new);
    assert forall|k: int| 0 <= k < painted.push(w).len() implies
        #[trigger] painted.push(w)[k].cell >= 0 && painted.push(w)[k].cell < n
            && painted.push(w)[k].px.len() == 4 by {
        if k < painted.len() {
            assert(painted.push(w)[k] == painted[k]) by {
                broadcast use vstd::seq_lib::group_seq_lib_default;
            }
        } else {
            assert(k == painted.len());
            assert(painted.push(w)[k] == w) by {
                broadcast use vstd::seq_lib::group_seq_lib_default;
            }
        }
    }
}

/// A write that changes no byte (the AND fallback's no-op case: bit zero with
/// no explicit XOR alpha) leaves the state well-formed unchanged.
pub proof fn lemma_well_formed_noop(
    o: Seq<u8>,
    o_new: Seq<u8>,
    painted: Seq<Write>,
    n: int,
    fill_bytes: Seq<u8>,
)
    requires
        well_formed(Img { o: o, painted: painted }, n, fill_bytes),
        o_new == o,
    ensures
        well_formed(Img { o: o_new, painted: painted }, n, fill_bytes),
{
    reveal(well_formed);
}

// ---------------------------------------------------------------------------
// MR · the raw/BITFIELDS transition (T6)
// ---------------------------------------------------------------------------

/// The model's internal error wrapper for the raw transition (THEOREMS.md: a
/// call on other records/offsets yields `Outside`, not a 27th API error).
pub enum RawErr {
    Api(Error),
    Outside,
}

/// R8/R7: a raw sample's palette index is in range. Depths above 8 are never
/// indexed, so the test is vacuous there.
pub open spec fn raw_index_ok(b: Seq<u8>, m: MHRecord, pos: int, x: int) -> bool {
    if m.depth <= 8 {
        0 <= sub_index(b[pos + x * m.depth / 8], m.depth, x) < m.count
    } else {
        true
    }
}

/// T6's domain: a raw/BITFIELDS record with a checked row stride.
pub open spec fn raw_reachable(b: Seq<u8>, m: MHRecord, pos: int, x: int, y: int) -> bool {
    &&& (m.compression == 0 || m.compression == 3)
    &&& (m.compression == 3 ==> (m.depth == 16 || m.depth == 32))
    &&& m.stride == stride_of(m.width, m.depth)
    &&& 0 <= m.stride
    &&& 0 <= pos <= b.len()
    &&& 0 <= x <= m.width
    &&& 0 <= y <= m.height
}

/// R9/R8: the raw transition. Rows are visited in source order; a row is only
/// entered when its whole stride is available (`TruncatedPixels` at the row
/// start, the model's ghost cursor `pos`); within a row each sample is
/// converted and checked in order (the first out-of-range palette index gives
/// `PaletteIndexOutOfRange` with the row-start cursor). The write list is the
/// exact sequence of pixel writes; `x >= width` advances the cursor by one
/// stride and starts the next row.
pub open spec fn raw_full(
    b: Seq<u8>,
    m: MHRecord,
    pos: int,
    x: int,
    y: int,
) -> Result<(int, int, int, Seq<Write>), (RawErr, int, int, int, Seq<Write>)>
    decreases (m.height - y) as nat, (m.width - x) as nat,
{
    if y >= m.height {
        Ok((pos, x, y, Seq::<Write>::empty()))
    } else if x >= m.width {
        raw_full(b, m, pos + m.stride, 0, y + 1)
    } else if x == 0 && pos + m.stride > b.len() {
        Err((RawErr::Api(Error::TruncatedPixels), pos, 0, y, Seq::<Write>::empty()))
    } else if !raw_index_ok(b, m, pos, x) {
        Err((RawErr::Api(Error::PaletteIndexOutOfRange), pos, x, y, Seq::<Write>::empty()))
    } else {
        let w = seq![
            Write { cell: cell_of(m, x, y), px: raw_conv(b, m, pos, x) },
        ];
        match raw_full(b, m, pos, x + 1, y) {
            Ok((pc, xc, yc, ws)) => Ok((pc, xc, yc, w + ws)),
            Err((e, pc, xc, yc, ws)) => Err((e, pc, xc, yc, w + ws)),
        }
    }
}

/// MR for the raw path: the exact writes or the exact first error and ghost
/// cursor/position. `raw_full` from the pixel start.
pub open spec fn mr_raw(b: Seq<u8>, m: MHRecord, pos0: int) -> Result<
    (int, Seq<Write>),
    (RawErr, int, int, int),
> {
    match raw_full(b, m, pos0, 0, 0) {
        Ok((pc, _xc, _yc, ws)) => Ok((pc, ws)),
        Err((e, pc, xc, yc, _ws)) => Err((e, pc, xc, yc)),
    }
}

/// T6's whole-image form: the raw writes replayed onto the zero fill, as a byte
/// buffer.
pub open spec fn mr_raw_out(b: Seq<u8>, m: MHRecord, pos0: int) -> Result<
    (int, Seq<u8>),
    (RawErr, int, int, int),
> {
    match raw_full(b, m, pos0, 0, 0) {
        Ok((pc, _xc, _yc, ws)) => Ok((pc, bytes_replay(zero_bytes(4 * m.n), ws))),
        Err((e, pc, xc, yc, _ws)) => Err((e, pc, xc, yc)),
    }
}

/// The raw loop's writes from `(pos, x, y)`: the exact pixel writes on success,
/// the writes painted before the failing check on failure.
pub open spec fn raw_writes(b: Seq<u8>, m: MHRecord, pos: int, x: int, y: int) -> Seq<Write>
    decreases (m.height - y) as nat, (m.width - x) as nat,
{
    match raw_full(b, m, pos, x, y) {
        Ok((_pc, _xc, _yc, ws)) => ws,
        Err((_e, _pc, _xc, _yc, ws)) => ws,
    }
}

/// The raw loop's cursor and position projection: on `Ok` the final
/// `(pos, x, y)`; on `Err` the ghost cursor and position at the failing check.
pub open spec fn raw_gcur(
    b: Seq<u8>,
    m: MHRecord,
    pos: int,
    x: int,
    y: int,
) -> Result<(int, int, int), (RawErr, int, int, int)> {
    match raw_full(b, m, pos, x, y) {
        Ok((pc, xc, yc, _ws)) => Ok((pc, xc, yc)),
        Err((e, pc, xc, yc, _ws)) => Err((e, pc, xc, yc)),
    }
}

/// The cursor/position component of `raw_full`, without the writes.
pub open spec fn raw_cur(
    b: Seq<u8>,
    m: MHRecord,
    pos: int,
    x: int,
    y: int,
) -> Result<(int, int, int), (RawErr, int, int, int)> {
    raw_gcur(b, m, pos, x, y)
}

/// On `Ok`, the raw loop ends at `(pos', 0, h)` — it advances whole rows. The
/// invariant hypothesis `y >= height ==> x == 0` holds at every reached state.
pub proof fn lemma_raw_gcur_ok_shape(b: Seq<u8>, m: MHRecord, pos: int, x: int, y: int)
    requires
        raw_gcur(b, m, pos, x, y) is Ok,
        0 <= x <= m.width,
        0 <= y <= m.height,
        y >= m.height ==> x == 0,
    ensures
        exists|pc: int| raw_gcur(b, m, pos, x, y) == Ok((pc, 0, m.height)),
    decreases (m.height - y) as nat, (m.width - x) as nat,
{
    reveal(raw_gcur);
    reveal(raw_full);
    if y >= m.height {
        assert(y == m.height);
        assert(x == 0);
        assert(raw_gcur(b, m, pos, x, y) == Ok((pos, x, y)));
        assert(exists|pc: int| raw_gcur(b, m, pos, x, y) == Ok((pc, 0, m.height))) by {
            let pc = pos;
            assert(raw_gcur(b, m, pos, x, y) == Ok((pc, 0, m.height)));
        }
    } else if x >= m.width {
        lemma_raw_gcur_ok_shape(b, m, pos + m.stride, 0, y + 1);
        assert(raw_gcur(b, m, pos, x, y) == raw_gcur(b, m, pos + m.stride, 0, y + 1));
    } else if x == 0 && pos + m.stride > b.len() {
        assert(false);
    } else if !raw_index_ok(b, m, pos, x) {
        assert(false);
    } else {
        lemma_raw_gcur_ok_shape(b, m, pos, x + 1, y);
        assert(raw_gcur(b, m, pos, x, y) == raw_gcur(b, m, pos, x + 1, y));
    }
}

/// `raw_full`'s result projects to `raw_gcur` and `raw_writes`.
pub proof fn lemma_raw_full_proj(b: Seq<u8>, m: MHRecord, pos: int, x: int, y: int)
    ensures
        match raw_full(b, m, pos, x, y) {
            Ok(r) => raw_gcur(b, m, pos, x, y) == Ok((r.0, r.1, r.2)) && r.3 == raw_writes(
                b,
                m,
                pos,
                x,
                y,
            ),
            Err(t) => raw_gcur(b, m, pos, x, y) == Err((t.0, t.1, t.2, t.3)) && t.4 == raw_writes(
                b,
                m,
                pos,
                x,
                y,
            ),
        },
    decreases (m.height - y) as nat, (m.width - x) as nat,
{
    reveal(raw_full);
    reveal(raw_gcur);
    reveal(raw_writes);
    if y >= m.height {
    } else if x >= m.width {
        lemma_raw_full_proj(b, m, pos + m.stride, 0, y + 1);
    } else if x == 0 && pos + m.stride > b.len() {
    } else if !raw_index_ok(b, m, pos, x) {
    } else {
        lemma_raw_full_proj(b, m, pos, x + 1, y);
    }
}

/// The executable's raw loop extends the model's write list by one write per
/// sample, in the model's own order.
pub proof fn lemma_raw_writes_step(b: Seq<u8>, m: MHRecord, pos: int, x: int, y: int)
    requires
        0 <= x < m.width,
        0 <= y,
        y < m.height,
        !(x == 0 && pos + m.stride > b.len()),
        raw_index_ok(b, m, pos, x),
    ensures
        raw_writes(b, m, pos, x, y) == seq![
            Write { cell: cell_of(m, x, y), px: raw_conv(b, m, pos, x) },
        ] + raw_writes(b, m, pos, x + 1, y),
        raw_gcur(b, m, pos, x, y) == raw_gcur(b, m, pos, x + 1, y),
{
    reveal(raw_full);
    reveal(raw_writes);
    reveal(raw_gcur);
}

// ---------------------------------------------------------------------------
// MR · the RLE4/RLE8 state machine (T7)
// ---------------------------------------------------------------------------

/// The RLE state-machine error wrapper.
pub enum RleErr {
    Api(Error),
    Outside,
}

/// R5/R7: an indexed pixel's palette index is in range.
pub open spec fn index_ok(m: MHRecord, v: int) -> bool {
    m.depth > 8 || (0 <= v < m.count)
}

/// R10: the index of the `j`-th painted pixel of a run of `q` pixels: RLE8
/// repeats the code byte; RLE4 alternates the high nibble (`j` even, high
/// first) and the low nibble.
pub open spec fn run_index(code: int, depth: int, j: int) -> int {
    if depth == 8 {
        code
    } else if j % 2 == 0 {
        code / 16
    } else {
        code % 16
    }
}

/// R11: the index of the `j`-th painted pixel of an absolute packet whose
/// payload starts at absolute offset `pay`. Stated with the executable's own
/// arithmetic (`>> 4` on the byte for the high nibble, `% 16` for the low), so
/// the painter's `u32` extraction is definitional.
pub open spec fn abs_index(b: Seq<u8>, depth: int, pay: int, j: int) -> int {
    if depth == 8 {
        b[pay + j] as int
    } else if j % 2 == 0 {
        ((b[pay + j / 2] as int) / 16)
    } else {
        ((b[pay + j / 2] as int) % 16)
    }
}

/// The number of leading in-range indices among the run positions `j..q`: the
/// first out-of-range index is at offset `j + run_good`, so that many pixels
/// are painted before the check fails.
pub open spec fn run_good(m: MHRecord, code: int, q: int, j: int) -> int
    decreases (q - j) as nat,
{
    if j >= q {
        0
    } else if !index_ok(m, run_index(code, m.depth, j)) {
        0
    } else {
        1 + run_good(m, code, q, j + 1)
    }
}

/// The whole run's indices are in range.
pub open spec fn run_ok(m: MHRecord, code: int, q: int) -> bool {
    run_good(m, code, q, 0) == q
}

/// The same for an absolute packet.
pub open spec fn abs_good(b: Seq<u8>, m: MHRecord, pay: int, q: int, j: int) -> int
    decreases (q - j) as nat,
{
    if j >= q {
        0
    } else if !index_ok(m, abs_index(b, m.depth, pay, j)) {
        0
    } else {
        1 + abs_good(b, m, pay, q, j + 1)
    }
}

pub open spec fn abs_ok(b: Seq<u8>, m: MHRecord, pay: int, q: int) -> bool {
    abs_good(b, m, pay, q, 0) == q
}

/// A run's good prefix never exceeds `q - j`, and reaches `q - j` exactly when
/// every index is in range. This bounds the partial-paint count on failure.
pub proof fn lemma_run_good_bound(m: MHRecord, code: int, q: int, j: int)
    requires
        0 <= j <= q,
    ensures
        0 <= run_good(m, code, q, j) <= q - j,
        (forall|i: int| j <= i < q ==> #[trigger] index_ok(m, run_index(code, m.depth, i)))
            ==> run_good(m, code, q, j) == q - j,
    decreases (q - j) as nat,
{
    reveal(run_good);
    if j >= q {
    } else if !index_ok(m, run_index(code, m.depth, j)) {
    } else {
        lemma_run_good_bound(m, code, q, j + 1);
    }
}

/// The same bound for an absolute packet.
pub proof fn lemma_abs_good_bound(b: Seq<u8>, m: MHRecord, pay: int, q: int, j: int)
    requires
        0 <= j <= q,
    ensures
        0 <= abs_good(b, m, pay, q, j) <= q - j,
        (forall|i: int| j <= i < q ==> #[trigger] index_ok(m, abs_index(b, m.depth, pay, i)))
            ==> abs_good(b, m, pay, q, j) == q - j,
    decreases (q - j) as nat,
{
    reveal(abs_good);
    if j >= q {
    } else if !index_ok(m, abs_index(b, m.depth, pay, j)) {
    } else {
        lemma_abs_good_bound(b, m, pay, q, j + 1);
    }
}

/// `run_ok` gives every run index in range.
pub proof fn lemma_run_ok_all(m: MHRecord, code: int, q: int, i: int)
    requires
        run_ok(m, code, q),
        0 <= i < q,
        q >= 0,
    ensures
        index_ok(m, run_index(code, m.depth, i)),
{
    reveal(run_ok);
    lemma_run_good_all(m, code, q, 0, i);
}

/// The good prefix has reached the end, so every index from `j` on is in range.
pub proof fn lemma_run_good_all(m: MHRecord, code: int, q: int, j: int, i: int)
    requires
        run_good(m, code, q, j) == q - j,
        0 <= j <= q,
        j <= i < q,
    ensures
        index_ok(m, run_index(code, m.depth, i)),
    decreases (q - j) as nat,
{
    reveal(run_good);
    if j >= q {
    } else if !index_ok(m, run_index(code, m.depth, j)) {
        assert(q - j > 0) by (nonlinear_arith)
            requires j < q;
    } else {
        assert(run_good(m, code, q, j) == 1 + run_good(m, code, q, j + 1));
        assert(run_good(m, code, q, j + 1) == q - (j + 1)) by (nonlinear_arith)
            requires
                run_good(m, code, q, j) == 1 + run_good(m, code, q, j + 1),
                run_good(m, code, q, j) == q - j;
        if i == j {
        } else {
            lemma_run_good_all(m, code, q, j + 1, i);
        }
    }
}

/// `abs_ok` gives every absolute index in range.
pub proof fn lemma_abs_ok_all(b: Seq<u8>, m: MHRecord, pay: int, q: int, i: int)
    requires
        abs_ok(b, m, pay, q),
        0 <= i < q,
        q >= 0,
    ensures
        index_ok(m, abs_index(b, m.depth, pay, i)),
{
    reveal(abs_ok);
    lemma_abs_good_all(b, m, pay, q, 0, i);
}

/// The good prefix has reached the end, so every index from `j` on is in range.
pub proof fn lemma_abs_good_all(b: Seq<u8>, m: MHRecord, pay: int, q: int, j: int, i: int)
    requires
        abs_good(b, m, pay, q, j) == q - j,
        0 <= j <= q,
        j <= i < q,
    ensures
        index_ok(m, abs_index(b, m.depth, pay, i)),
    decreases (q - j) as nat,
{
    reveal(abs_good);
    if j >= q {
    } else if !index_ok(m, abs_index(b, m.depth, pay, j)) {
        assert(q - j > 0) by (nonlinear_arith)
            requires j < q;
    } else {
        assert(abs_good(b, m, pay, q, j) == 1 + abs_good(b, m, pay, q, j + 1));
        assert(abs_good(b, m, pay, q, j + 1) == q - (j + 1)) by (nonlinear_arith)
            requires
                abs_good(b, m, pay, q, j) == 1 + abs_good(b, m, pay, q, j + 1),
                abs_good(b, m, pay, q, j) == q - j;
        if i == j {
        } else {
            lemma_abs_good_all(b, m, pay, q, j + 1, i);
        }
    }
}

/// The writes of a run of pixels at positions `j..k` (only used where every
/// index is in range).
pub open spec fn run_writes(
    b: Seq<u8>,
    m: MHRecord,
    x: int,
    y: int,
    code: int,
    j: int,
    k: int,
) -> Seq<Write>
    decreases (k - j) as nat,
{
    if j >= k {
        Seq::<Write>::empty()
    } else {
        seq![
            Write { cell: cell_of(m, x + j, y), px: conv(b, m, run_index(code, m.depth, j)) },
        ] + run_writes(b, m, x, y, code, j + 1, k)
    }
}

/// The same for an absolute packet.
pub open spec fn abs_writes(
    b: Seq<u8>,
    m: MHRecord,
    x: int,
    y: int,
    pay: int,
    j: int,
    k: int,
) -> Seq<Write>
    decreases (k - j) as nat,
{
    if j >= k {
        Seq::<Write>::empty()
    } else {
        seq![
            Write { cell: cell_of(m, x + j, y), px: conv(b, m, abs_index(b, m.depth, pay, j)) },
        ] + abs_writes(b, m, x, y, pay, j + 1, k)
    }
}

/// T7's domain: an RLE4/RLE8 record with the checked image geometry.
pub open spec fn rle_reachable(b: Seq<u8>, m: MHRecord, p: int, x: int, y: int) -> bool {
    &&& (m.compression == 1 || m.compression == 2)
    &&& (m.depth == 4 || m.depth == 8)
    &&& 1 <= m.width <= max_dimension()
    &&& 1 <= m.height <= max_dimension()
    &&& m.n == m.width * m.height
    &&& 0 <= p <= b.len()
    &&& 0 <= x <= m.width
    &&& 0 <= y <= m.height
}

/// R10/R11: the total RLE state machine. `mr_full` threads the position
/// `(x, y)` in bottom-up source coordinates and the ordered write list. The
/// check order is exactly R14's: two-byte command presence; then, per command,
/// run bounds / EOL bounds / delta presence / delta bounds / absolute bounds /
/// whole padded payload presence; then each painted index in R10–R11 order. On
/// failure it exposes the exact first error and the ghost cursor/coordinates,
/// with the writes painted before the failing check.
pub open spec fn mr_full(
    b: Seq<u8>,
    m: MHRecord,
    p: int,
    x: int,
    y: int,
) -> Result<(int, int, int, Seq<Write>), (RleErr, int, int, int, Seq<Write>)>
    decreases (b.len() - p) as nat,
{
    if p + 2 > b.len() {
        Err((RleErr::Api(Error::RleTruncated), p, x, y, Seq::<Write>::empty()))
    } else {
        let quantity = b[p] as int;
        let code = b[p + 1] as int;
        let p2 = p + 2;
        if quantity != 0 {
            if y >= m.height || x + quantity > m.width {
                Err((RleErr::Api(Error::RleBounds), p2, x, y, Seq::<Write>::empty()))
            } else {
                let g = run_good(m, code, quantity, 0);
                if g < quantity {
                    Err((
                        RleErr::Api(Error::PaletteIndexOutOfRange),
                        p2,
                        x,
                        y,
                        run_writes(b, m, x, y, code, 0, g),
                    ))
                } else {
                    let ws = run_writes(b, m, x, y, code, 0, quantity);
                    match mr_full(b, m, p2, x + quantity, y) {
                        Ok((pc, xc, yc, ws2)) => Ok((pc, xc, yc, ws + ws2)),
                        Err((e, pc, xc, yc, ws2)) => Err((e, pc, xc, yc, ws + ws2)),
                    }
                }
            }
        } else if code == 1 {
            Ok((p2, x, y, Seq::<Write>::empty()))
        } else if code == 0 {
            if y >= m.height {
                Err((RleErr::Api(Error::RleBounds), p2, x, y, Seq::<Write>::empty()))
            } else {
                mr_full(b, m, p2, 0, y + 1)
            }
        } else if code == 2 {
            if p2 + 2 > b.len() {
                Err((RleErr::Api(Error::RleTruncated), p2, x, y, Seq::<Write>::empty()))
            } else {
                let dx = b[p2] as int;
                let dy = b[p2 + 1] as int;
                let p4 = p2 + 2;
                if y + dy >= m.height || x + dx >= m.width {
                    Err((RleErr::Api(Error::RleBounds), p4, x, y, Seq::<Write>::empty()))
                } else {
                    mr_full(b, m, p4, x + dx, y + dy)
                }
            }
        } else {
            let q = code;
            if y >= m.height || x + q > m.width {
                Err((RleErr::Api(Error::RleBounds), p2, x, y, Seq::<Write>::empty()))
            } else {
                let payload = rle_payload(q, m.depth);
                let padded = rle_padded(q, m.depth);
                if p2 + padded > b.len() {
                    Err((RleErr::Api(Error::RleTruncated), p2, x, y, Seq::<Write>::empty()))
                } else {
                    let g = abs_good(b, m, p2, q, 0);
                    if g < q {
                        Err((
                            RleErr::Api(Error::PaletteIndexOutOfRange),
                            p2 + padded,
                            x,
                            y,
                            abs_writes(b, m, x, y, p2, 0, g),
                        ))
                    } else {
                        let ws = abs_writes(b, m, x, y, p2, 0, q);
                        match mr_full(b, m, p2 + padded, x + q, y) {
                            Ok((pc, xc, yc, ws2)) => Ok((pc, xc, yc, ws + ws2)),
                            Err((e, pc, xc, yc, ws2)) => Err((e, pc, xc, yc, ws + ws2)),
                        }
                    }
                }
            }
        }
    }
}

/// The state-and-error projection of `mr_full`: the same ordered checks and
/// the same final cursor/position, without the write bookkeeping.
pub open spec fn mr_cur(
    b: Seq<u8>,
    m: MHRecord,
    p: int,
    x: int,
    y: int,
) -> Result<(int, int, int), RleErr>
    decreases (b.len() - p) as nat,
{
    if p + 2 > b.len() {
        Err(RleErr::Api(Error::RleTruncated))
    } else {
        let quantity = b[p] as int;
        let code = b[p + 1] as int;
        let p2 = p + 2;
        if quantity != 0 {
            if y >= m.height || x + quantity > m.width {
                Err(RleErr::Api(Error::RleBounds))
            } else if !run_ok(m, code, quantity) {
                Err(RleErr::Api(Error::PaletteIndexOutOfRange))
            } else {
                mr_cur(b, m, p2, x + quantity, y)
            }
        } else if code == 1 {
            Ok((p2, x, y))
        } else if code == 0 {
            if y >= m.height {
                Err(RleErr::Api(Error::RleBounds))
            } else {
                mr_cur(b, m, p2, 0, y + 1)
            }
        } else if code == 2 {
            if p2 + 2 > b.len() {
                Err(RleErr::Api(Error::RleTruncated))
            } else {
                let dx = b[p2] as int;
                let dy = b[p2 + 1] as int;
                let p4 = p2 + 2;
                if y + dy >= m.height || x + dx >= m.width {
                    Err(RleErr::Api(Error::RleBounds))
                } else {
                    mr_cur(b, m, p4, x + dx, y + dy)
                }
            }
        } else {
            let q = code;
            if y >= m.height || x + q > m.width {
                Err(RleErr::Api(Error::RleBounds))
            } else {
                let padded = rle_padded(q, m.depth);
                if p2 + padded > b.len() {
                    Err(RleErr::Api(Error::RleTruncated))
                } else if !abs_ok(b, m, p2, q) {
                    Err(RleErr::Api(Error::PaletteIndexOutOfRange))
                } else {
                    mr_cur(b, m, p2 + padded, x + q, y)
                }
            }
        }
    }
}

/// The write-list projection of `mr_full`: the exact pixels written from
/// `(p, x, y)` until the machine stops.
pub open spec fn mr_writes(b: Seq<u8>, m: MHRecord, p: int, x: int, y: int) -> Seq<Write>
    decreases (b.len() - p) as nat,
{
    if p + 2 > b.len() {
        Seq::<Write>::empty()
    } else {
        let quantity = b[p] as int;
        let code = b[p + 1] as int;
        let p2 = p + 2;
        if quantity != 0 {
            if y >= m.height || x + quantity > m.width {
                Seq::<Write>::empty()
            } else {
                let g = run_good(m, code, quantity, 0);
                if g < quantity {
                    run_writes(b, m, x, y, code, 0, g)
                } else {
                    run_writes(b, m, x, y, code, 0, quantity) + mr_writes(b, m, p2, x + quantity, y)
                }
            }
        } else if code == 1 {
            Seq::<Write>::empty()
        } else if code == 0 {
            if y >= m.height {
                Seq::<Write>::empty()
            } else {
                mr_writes(b, m, p2, 0, y + 1)
            }
        } else if code == 2 {
            if p2 + 2 > b.len() {
                Seq::<Write>::empty()
            } else {
                let dx = b[p2] as int;
                let dy = b[p2 + 1] as int;
                let p4 = p2 + 2;
                if y + dy >= m.height || x + dx >= m.width {
                    Seq::<Write>::empty()
                } else {
                    mr_writes(b, m, p4, x + dx, y + dy)
                }
            }
        } else {
            let q = code;
            if y >= m.height || x + q > m.width {
                Seq::<Write>::empty()
            } else {
                let padded = rle_padded(q, m.depth);
                if p2 + padded > b.len() {
                    Seq::<Write>::empty()
                } else {
                    let g = abs_good(b, m, p2, q, 0);
                    if g < q {
                        abs_writes(b, m, x, y, p2, 0, g)
                    } else {
                        abs_writes(b, m, x, y, p2, 0, q) + mr_writes(b, m, p2 + padded, x + q, y)
                    }
                }
            }
        }
    }
}

/// `mr_full`'s result projects to `mr_cur` (state/error) and `mr_writes`
/// (pixels), by induction on the recursion.
pub proof fn lemma_mr_full_proj(b: Seq<u8>, m: MHRecord, p: int, x: int, y: int)
    requires
        0 <= p,
    ensures
        match mr_full(b, m, p, x, y) {
            Ok(r) => mr_cur(b, m, p, x, y) == Ok((r.0, r.1, r.2)) && r.3 == mr_writes(
                b,
                m,
                p,
                x,
                y,
            ),
            Err(t) => mr_cur(b, m, p, x, y) == Err(t.0) && t.4 == mr_writes(b, m, p, x, y),
        },
    decreases (b.len() - p) as nat,
{
    reveal(mr_full);
    reveal(mr_cur);
    reveal(mr_writes);
    reveal(run_ok);
    reveal(abs_ok);
    if p + 2 > b.len() {
    } else {
        let quantity = b[p] as int;
        let code = b[p + 1] as int;
        let p2 = p + 2;
        if quantity != 0 {
            if y >= m.height || x + quantity > m.width {
            } else {
                let g = run_good(m, code, quantity, 0);
                lemma_run_good_bound(m, code, quantity, 0);
                if g < quantity {
                } else {
                    assert(run_ok(m, code, quantity));
                    lemma_run_ok_all(m, code, quantity, 0);
                    let ws = run_writes(b, m, x, y, code, 0, quantity);
                    lemma_mr_full_proj(b, m, p2, x + quantity, y);
                    match mr_full(b, m, p2, x + quantity, y) {
                        Ok(r) => {
                            assert(mr_writes(b, m, p, x, y) == ws + mr_writes(
                                b,
                                m,
                                p2,
                                x + quantity,
                                y,
                            ));
                        },
                        Err(t) => {
                            assert(mr_writes(b, m, p, x, y) == ws + mr_writes(
                                b,
                                m,
                                p2,
                                x + quantity,
                                y,
                            ));
                        },
                    }
                }
            }
        } else if code == 1 {
        } else if code == 0 {
            if y >= m.height {
            } else {
                lemma_mr_full_proj(b, m, p2, 0, y + 1);
            }
        } else if code == 2 {
            if p2 + 2 > b.len() {
            } else {
                let dx = b[p2] as int;
                let dy = b[p2 + 1] as int;
                let p4 = p2 + 2;
                if y + dy >= m.height || x + dx >= m.width {
                } else {
                    lemma_mr_full_proj(b, m, p4, x + dx, y + dy);
                }
            }
        } else {
            let q = code;
            if y >= m.height || x + q > m.width {
            } else {
                let padded = rle_padded(q, m.depth);
                if p2 + padded > b.len() {
                } else {
                    let g = abs_good(b, m, p2, q, 0);
                    lemma_abs_good_bound(b, m, p2, q, 0);
                    if g < q {
                    } else {
                        assert(abs_ok(b, m, p2, q));
                        lemma_abs_ok_all(b, m, p2, q, 0);
                        let ws = abs_writes(b, m, x, y, p2, 0, q);
                        lemma_mr_full_proj(b, m, p2 + padded, x + q, y);
                        match mr_full(b, m, p2 + padded, x + q, y) {
                            Ok(r) => {
                                assert(mr_writes(b, m, p, x, y) == ws + mr_writes(
                                    b,
                                    m,
                                    p2 + padded,
                                    x + q,
                                    y,
                                ));
                            },
                            Err(t) => {
                                assert(mr_writes(b, m, p, x, y) == ws + mr_writes(
                                    b,
                                    m,
                                    p2 + padded,
                                    x + q,
                                    y,
                                ));
                            },
                        }
                    }
                }
            }
        }
    }
}

/// A complete encoded run consumes its command before painting, then appends
/// the continuation's writes. Both projections take the same recursive step.
pub proof fn lemma_mr_step_run(b: Seq<u8>, m: MHRecord, p: int, x: int, y: int)
    requires
        0 <= p, p + 2 <= b.len(), b[p] != 0,
        y < m.height, x + b[p] as int <= m.width,
        run_ok(m, b[p + 1] as int, b[p] as int),
    ensures
        mr_writes(b, m, p, x, y) ==
            run_writes(b, m, x, y, b[p + 1] as int, 0, b[p] as int)
            + mr_writes(b, m, p + 2, x + b[p] as int, y),
        mr_cur(b, m, p, x, y) ==
            mr_cur(b, m, p + 2, x + b[p] as int, y),
{
    reveal(mr_writes);
    reveal(mr_cur);
    reveal(run_ok);
}

/// A complete absolute packet takes its padding before painting and appends
/// the continuation's writes only after every palette index succeeds.
pub proof fn lemma_mr_step_abs(b: Seq<u8>, m: MHRecord, p: int, x: int, y: int)
    requires
        0 <= p, p + 2 <= b.len(), b[p] == 0,
        b[p + 1] >= 3, y < m.height,
        x + b[p + 1] as int <= m.width,
        p + 2 + rle_padded(b[p + 1] as int, m.depth) <= b.len(),
        abs_ok(b, m, p + 2, b[p + 1] as int),
    ensures
        mr_writes(b, m, p, x, y) ==
            abs_writes(b, m, x, y, p + 2, 0, b[p + 1] as int)
            + mr_writes(b, m, p + 2 + rle_padded(b[p + 1] as int, m.depth),
                x + b[p + 1] as int, y),
        mr_cur(b, m, p, x, y) ==
            mr_cur(b, m, p + 2 + rle_padded(b[p + 1] as int, m.depth),
                x + b[p + 1] as int, y),
{
    reveal(mr_writes);
    reveal(mr_cur);
    reveal(abs_ok);
}

/// The cursor/position projection of `mr_full` on both exits: the final
/// `(p, x, y)` on EOB, the ghost `(p, x, y)` at the failing check on error.
pub open spec fn mr_gcur(
    b: Seq<u8>,
    m: MHRecord,
    p: int,
    x: int,
    y: int,
) -> Result<(int, int, int), (RleErr, int, int, int)> {
    match mr_full(b, m, p, x, y) {
        Ok((pc, xc, yc, _ws)) => Ok((pc, xc, yc)),
        Err((e, pc, xc, yc, _ws)) => Err((e, pc, xc, yc)),
    }
}

/// `mr_full`'s result is exactly its two projections `mr_gcur` and
/// `mr_writes`: the lemma that discharges the projection hypothesis from the
/// model's own result (used by the executable's T7 proof).
pub proof fn lemma_mr_gcur_proj(b: Seq<u8>, m: MHRecord, p: int, x: int, y: int)
    requires
        0 <= p,
    ensures
        match mr_full(b, m, p, x, y) {
            Ok(r) => mr_gcur(b, m, p, x, y) == Ok((r.0, r.1, r.2)) && r.3 == mr_writes(
                b,
                m,
                p,
                x,
                y,
            ),
            Err(t) => mr_gcur(b, m, p, x, y) == Err((t.0, t.1, t.2, t.3)) && t.4 == mr_writes(
                b,
                m,
                p,
                x,
                y,
            ),
        },
{
    reveal(mr_gcur);
    lemma_mr_full_proj(b, m, p, x, y);
}

/// L3 row transition: one two-pixel RLE8 run followed by EOL.
/// The write list records both red pixels before moving to the next row.
pub proof fn lemma_l3_row_step(b: Seq<u8>, m: MHRecord, p: int, j: int)
    requires
        0 <= p, p + 4 <= b.len(),
        0 <= j < m.height,
        m.width == 2, m.depth == 8, m.count == 2,
        b[p] == 2, b[p + 1] == 1, b[p + 2] == 0, b[p + 3] == 0,
    ensures
        mr_gcur(b, m, p, 0, j) == mr_gcur(b, m, p + 4, 0, j + 1),
        mr_writes(b, m, p, 0, j) ==
            run_writes(b, m, 0, j, 1, 0, 2) + mr_writes(b, m, p + 4, 0, j + 1),
{
    reveal(run_index);
    reveal(index_ok);
    assert forall|i: int| 0 <= i < 2 implies
        #[trigger] index_ok(m, run_index(1, m.depth, i)) by {
        assert(run_index(1, m.depth, i) == 1);
    };
    lemma_run_good_bound(m, 1, 2, 0);
    assert(run_good(m, 1, 2, 0) == 2);
    lemma_run_good_is_run_ok(m, 1, 2);
    lemma_mr_step_run_gcur(b, m, p, 0, j);
    lemma_mr_step_run(b, m, p, 0, j);
    lemma_mr_step_eol(b, m, p + 2, 2, j);
}

/// The first `j` source rows become the last `2j` top-first pixels,
/// red over the palette-zero (black) fill.
pub open spec fn l3_prefix_image(k: int, j: int) -> Seq<u8> {
    Seq::new((8 * k) as nat, |i: int|
        if i % 4 == 0 && i >= 8 * (k - j) { 255u8 }
        else if i % 4 == 3 { 255u8 }
        else { 0u8 })
}

pub open spec fn l3_prefix_writes(b: Seq<u8>, m: MHRecord, j: int) -> Seq<Write>
    decreases j as nat,
{
    if j <= 0 { Seq::<Write>::empty() }
    else { l3_prefix_writes(b, m, j - 1) + run_writes(b, m, 0, j - 1, 1, 0, 2) }
}

/// Appending one red row changes only its two red channel bytes: every other
/// byte already equals the next prefix image (including the opaque alpha).
pub proof fn lemma_l3_pair_replay(k: int, j: int)
    requires 1 <= k <= 16, 0 <= j < k,
    ensures bytes_replay(
        l3_prefix_image(k, j), seq![
            Write { cell: 2 * (k - 1 - j), px: seq![255u8, 0u8, 0u8, 255u8] },
            Write { cell: 2 * (k - 1 - j) + 1, px: seq![255u8, 0u8, 0u8, 255u8] },
        ]) == l3_prefix_image(k, j + 1),
{
    reveal(bytes_replay);
    reveal(replay_one);
    reveal(replay_bytes_at);
    reveal(l3_prefix_image);
    let before = l3_prefix_image(k, j);
    let after = l3_prefix_image(k, j + 1);
    let red = seq![255u8, 0u8, 0u8, 255u8];
    let first = Write { cell: 2 * (k - 1 - j), px: red };
    let second = Write { cell: 2 * (k - 1 - j) + 1, px: red };
    let ws = seq![first, second];
    let start = 8 * (k - 1 - j);
    assert(before.len() == 8 * k);
    assert(after.len() == 8 * k);
    assert(ws == seq![first].push(second));
    assert(seq![first] == Seq::<Write>::empty().push(first));
    assert(bytes_replay(before, Seq::<Write>::empty()) == before);
    lemma_bytes_replay_push(before, Seq::<Write>::empty(), first);
    lemma_bytes_replay_push(before, seq![first], second);
    assert(bytes_replay(before, ws) == replay_one(replay_one(before, first), second));
    assert(bytes_replay(before, ws).len() == after.len());
    assert(start % 4 == 0) by (nonlinear_arith)
        requires start == 8 * (k - 1 - j);
    assert(bytes_replay(before, ws) =~= after) by {
        assert forall|i: int| 0 <= i < after.len() implies
            #[trigger] bytes_replay(before, ws)[i] == after[i] by {
            assert(before.len() == 8 * k);
            assert(after.len() == 8 * k);
            assert(bytes_replay(before, ws) == replay_one(replay_one(before, first), second));
            if start <= i < start + 8 && i % 4 == 0 {
                assert((i - start) % 4 == 0) by (nonlinear_arith)
                    requires i % 4 == 0, start % 4 == 0;
                assert(i == start || i == start + 4) by (nonlinear_arith)
                    requires 0 <= i - start < 8, (i - start) % 4 == 0;
            }
        }
    }
}

/// The prefix image equals the model's byte-level replay of exactly the
/// first `j` source rows; the red suffix has length `2j` pixels.
pub proof fn lemma_l3_painted_prefix(b: Seq<u8>, m: MHRecord, k: int, j: int)
    requires
        1 <= k <= 16, 0 <= j <= k,
        m.width == 2, m.height == k, m.n == 2 * k,
        m.depth == 8, !m.top_down,
        pal_entry(b, m.palette_start, m.palette_stride, 0)
            == seq![0u8, 0u8, 0u8, 255u8],
        conv(b, m, 1) == seq![255u8, 0u8, 0u8, 255u8],
    ensures bytes_replay(pal_zero_bytes(b, m), l3_prefix_writes(b, m, j))
        == l3_prefix_image(k, j),
    decreases j,
{
    reveal(l3_prefix_writes);
    if j == 0 {
        reveal(bytes_replay);
        reveal(pal_zero_bytes);
        reveal(l3_prefix_image);
        assert(pal_zero_bytes(b, m) =~= l3_prefix_image(k, 0)) by {
            assert forall|i: int| 0 <= i < 8 * k implies
                #[trigger] pal_zero_bytes(b, m)[i] == l3_prefix_image(k, 0)[i] by {
                assert(0 <= i % 4 < 4) by (nonlinear_arith);
            }
        }
    } else {
        lemma_l3_painted_prefix(b, m, k, j - 1);
        let old = l3_prefix_writes(b, m, j - 1);
        let row = run_writes(b, m, 0, j - 1, 1, 0, 2);
        lemma_bytes_replay_assoc(pal_zero_bytes(b, m), old, row);
        reveal(run_writes);
        reveal(run_index);
        reveal(cell_of);
        reveal(orient_cell);
        reveal(map_row_of);
        assert(run_index(1, m.depth, 0) == 1);
        assert(run_index(1, m.depth, 1) == 1);
        assert(cell_of(m, 0, j - 1) == 2 * (k - j));
        assert(cell_of(m, 1, j - 1) == 2 * (k - j) + 1);
        lemma_run_writes_unfold(b, m, 0, j - 1, 1, 0, 2);
        lemma_run_writes_unfold(b, m, 0, j - 1, 1, 1, 2);
        lemma_run_writes_empty(b, m, 0, j - 1, 1, 2, 2);
        assert(row.len() == 2);
        assert(row == seq![
            Write { cell: 2 * (k - j), px: seq![255u8, 0u8, 0u8, 255u8] },
            Write { cell: 2 * (k - j) + 1, px: seq![255u8, 0u8, 0u8, 255u8] },
        ]);
        lemma_l3_pair_replay(k, j - 1);
    }
}

/// The prefix and suffix lists partition exactly the model's RLE writes;
/// this joins the row-count induction to the byte-level prefix induction.
pub proof fn lemma_l3_writes_join(b: Seq<u8>, m: MHRecord, k: int, j: int)
    requires 0 <= j <= k,
    ensures l3_prefix_writes(b, m, j) + l3_writes(b, m, j, k)
        == l3_writes(b, m, 0, k),
    decreases j,
{
    if j == 0 {
        reveal(l3_prefix_writes);
        broadcast use vstd::seq_lib::group_seq_lib_default;
    } else {
        lemma_l3_writes_join(b, m, k, j - 1);
        reveal(l3_prefix_writes);
        reveal(l3_writes);
        vstd::seq_lib::lemma_concat_associative(
            l3_prefix_writes(b, m, j - 1),
            run_writes(b, m, 0, j - 1, 1, 0, 2),
            l3_writes(b, m, j, k),
        );
    }
}

/// Write list for the remaining repeated RLE8 rows `P^(k-j)`.
pub open spec fn l3_writes(b: Seq<u8>, m: MHRecord, j: int, k: int) -> Seq<Write>
    decreases (k - j) as nat,
{
    if j >= k {
        Seq::<Write>::empty()
    } else {
        run_writes(b, m, 0, j, 1, 0, 2) + l3_writes(b, m, j + 1, k)
    }
}

/// L3's parameterised command-parser induction, independent of the header
/// constructor: after `j` rows, the next command sits at `p+4j` and state
/// `(0,j)`; the remaining rows produce exactly `l3_writes(j,k)` before EOB.
pub proof fn lemma_l3_rows(b: Seq<u8>, m: MHRecord, p: int, k: int, j: int)
    requires
        1 <= k <= 16, 0 <= j <= k,
        m.width == 2, m.height == k, m.depth == 8, m.count == 2,
        0 <= p, p + 4 * k + 2 <= b.len(),
        forall|t: int| 0 <= t < k ==> #[trigger] b[p + 4 * t] == 2
            && b[p + 4 * t + 1] == 1 && b[p + 4 * t + 2] == 0 && b[p + 4 * t + 3] == 0,
        b[p + 4 * k] == 0, b[p + 4 * k + 1] == 1,
    ensures
        mr_gcur(b, m, p + 4 * j, 0, j) == Ok((p + 4 * k + 2, 0, k)),
        mr_writes(b, m, p + 4 * j, 0, j) == l3_writes(b, m, j, k),
    decreases k - j,
{
    reveal(l3_writes);
    if j == k {
        lemma_mr_exit_eob(b, m, p + 4 * k, 0, k);
    } else {
        assert(b[p + 4 * j] == 2);
        assert(b[p + 4 * j + 1] == 1);
        assert(b[p + 4 * j + 2] == 0);
        assert(b[p + 4 * j + 3] == 0);
        lemma_l3_row_step(b, m, p + 4 * j, j);
        lemma_l3_rows(b, m, p, k, j + 1);
    }
}

/// Literal forty-byte RLE8 header plus two four-byte BGR0 palette entries:
/// black at index zero and red at index one.
pub open spec fn l3_header(k: int) -> Seq<u8> {
    Seq::new(48, |i: int|
        if i == 0 { 40u8 } else if i == 4 { 2u8 }
        else if i == 8 { k as u8 } else if i == 12 { 1u8 }
        else if i == 14 { 8u8 } else if i == 16 { 1u8 }
        else if i == 32 { 2u8 } else if i == 46 { 255u8 }
        else { 0u8 })
}

/// `P^k E`, where `P = 02 01 00 00` and `E = 00 01`.
pub open spec fn l3_bytes(k: int) -> Seq<u8> {
    l3_header(k) + Seq::new((4 * k) as nat, |i: int|
        if i % 4 == 0 { 2u8 } else if i % 4 == 1 { 1u8 } else { 0u8 })
        + seq![0u8, 1u8]
}

pub open spec fn l3_record(k: int) -> MHRecord {
    MHRecord {
        mode: Mode::BareDib, s: 0, hsize: 40, width: 2, height: k,
        stored_height: k, planes: 1, depth: 8, compression: 1,
        declared_image: 0, color_used: 2, count: 2, table_count: 2,
        palette_stride: 4, palette_start: 40, palette_end: 48,
        pixel_start: 48, mask_r: 0, mask_g: 0, mask_b: 0, mask_a: 0,
        has_masks: false, core: false, is_bmp: false, top_down: false,
        n: 2 * k, stride: 0,
    }
}

pub proof fn lemma_l3_header_fields(k: int)
    requires 1 <= k <= 16,
    ensures r2r5(l3_bytes(k), Mode::BareDib, 0) == Ok(RawFields {
        hsize: 40, core: false, width: 2, stored: k, planes: 1,
        depth: 8, compression: 1, declared: 0, used: 2, height: k,
        pos0: 40,
    }),
{
    let b = l3_bytes(k);
    reveal(l3_bytes);
    reveal(l3_header);
    assert(b.len() == 50 + 4 * k);
    assert(b[0] == 40u8);
    assert(b[4] == 2u8);
    assert(b[8] == k as u8);
    assert(b[12] == 1u8);
    assert(b[14] == 8u8);
    assert(b[16] == 1u8);
    assert(b[32] == 2u8);
    assert(le32(b, 0) == 40);
    assert(s32(b, 4) == 2);
    assert(s32(b, 8) == k);
    assert(le16(b, 12) == 1);
    assert(le16(b, 14) == 8);
    assert(le32(b, 16) == 1);
    assert(le32(b, 20) == 0);
    assert(le32(b, 32) == 2);
    reveal(r2r5);
    reveal(abs_int);
    reveal(visible_height_of);
    reveal(depth_ok);
    reveal(hsize_ok);
}

pub proof fn lemma_l3_header_record(k: int)
    requires 1 <= k <= 16,
    ensures mh(l3_bytes(k), Mode::BareDib) == Ok(l3_record(k)),
{
    let b = l3_bytes(k);
    let rf = RawFields {
        hsize: 40, core: false, width: 2, stored: k, planes: 1,
        depth: 8, compression: 1, declared: 0, used: 2, height: k,
        pos0: 40,
    };
    lemma_l3_header_fields(k);
    assert(r2r5(b, Mode::BareDib, 0) == Ok(rf));
    reveal(r6r7);
    reveal(r6r7_result);
    assert(r6r7(b, Mode::BareDib, 0, 0, false, rf) == Ok(l3_record(k)));
    reveal(r2r7);
    reveal(mh);
}

pub proof fn lemma_l3_concrete(k: int)
    requires 1 <= k <= 16,
    ensures
        l3_bytes(k).len() == 50 + 4 * k,
        mh(l3_bytes(k), Mode::BareDib) == Ok(l3_record(k)),
        mr_gcur(l3_bytes(k), l3_record(k), 48, 0, 0)
            == Ok((50 + 4 * k, 0, k)),
        mr_writes(l3_bytes(k), l3_record(k), 48, 0, 0)
            == l3_writes(l3_bytes(k), l3_record(k), 0, k),
{
    let b = l3_bytes(k);
    let m = l3_record(k);
    assert(b.len() == 50 + 4 * k);
    lemma_l3_header_record(k);
    assert(mh(b, Mode::BareDib) == Ok(m));
    assert forall|t: int| 0 <= t < k implies #[trigger] b[48 + 4 * t] == 2u8
        && b[48 + 4 * t + 1] == 1u8
        && b[48 + 4 * t + 2] == 0u8
        && b[48 + 4 * t + 3] == 0u8 by {
        reveal(l3_bytes);
    };
    assert(b[48 + 4 * k] == 0u8);
    assert(b[48 + 4 * k + 1] == 1u8);
    lemma_l3_rows(b, m, 48, k, 0);
}

/// The concrete fixture's index zero is black and index one is red.
pub proof fn lemma_l3_palette(k: int)
    requires 1 <= k <= 16,
    ensures
        pal_entry(l3_bytes(k), 40, 4, 0) == seq![0u8, 0u8, 0u8, 255u8],
        conv(l3_bytes(k), l3_record(k), 1) == seq![255u8, 0u8, 0u8, 255u8],
{
    let b = l3_bytes(k);
    reveal(l3_bytes);
    reveal(l3_header);
    reveal(pal_entry);
    reveal(conv);
    assert(b[40] == 0u8 && b[41] == 0u8 && b[42] == 0u8);
    assert(b[44] == 0u8 && b[45] == 0u8 && b[46] == 255u8);
}

/// The row and image inductions meet at MR's own successful output tuple.
pub proof fn lemma_l3_rle_image(k: int)
    requires 1 <= k <= 16,
    ensures mr_out(l3_bytes(k), l3_record(k), 48, 0, 0)
        == Ok((50 + 4 * k, l3_prefix_image(k, k))),
{
    let b = l3_bytes(k);
    let m = l3_record(k);
    lemma_l3_concrete(k);
    lemma_l3_palette(k);
    lemma_l3_painted_prefix(b, m, k, k);
    lemma_l3_writes_join(b, m, k, k);
    reveal(l3_writes);
    assert(l3_writes(b, m, k, k) == Seq::<Write>::empty());
    assert(l3_prefix_writes(b, m, k) == l3_writes(b, m, 0, k)) by {
        broadcast use vstd::seq_lib::group_seq_lib_default;
    }
    lemma_mr_gcur_proj(b, m, 48, 0, 0);
    match mr_full(b, m, 48, 0, 0) {
        Ok(r) => {
            assert(r.0 == 50 + 4 * k);
            assert(r.3 == l3_prefix_writes(b, m, k));
            assert(bytes_replay(pal_zero_bytes(b, m), r.3) == l3_prefix_image(k, k));
        },
        Err(_) => { assert(false); },
    }
    reveal(mr_out);
}

/// L3: for every `1 <= k <= 16`, the actual `H_k + P^k + E` model input
/// produces exactly `2k` red opaque pixels, top row first.
pub proof fn lemma_l3_full(k: int)
    requires 1 <= k <= 16,
    ensures mp(l3_bytes(k), Mode::BareDib) == Ok((
        2, k, Seq::new((8 * k) as nat, |i: int|
            if i % 4 == 0 || i % 4 == 3 { 255u8 } else { 0u8 }),
    )),
{
    let b = l3_bytes(k);
    let m = l3_record(k);
    let all_red = Seq::new((8 * k) as nat, |i: int|
        if i % 4 == 0 || i % 4 == 3 { 255u8 } else { 0u8 });
    lemma_l3_concrete(k);
    lemma_l3_rle_image(k);
    reveal(l3_prefix_image);
    assert(l3_prefix_image(k, k) =~= all_red);
    reveal(mp_mask_phase);
    reveal(mp_image_size_ok);
    reveal(image_size_ok);
    assert(mp_mask_phase(b, m, 50 + 4 * k, 2 + 4 * k, all_red)
        == Ok((50 + 4 * k, all_red)));
    reveal(mp_rle);
    assert(mp_rle(b, m) == Ok((all_red, 50 + 4 * k)));
    reveal(mp_decoded);
    reveal(mp);
}

/// The separate maximum-output RLE8 fixture's arithmetic, without building
/// its 266,290-byte input or 64 MiB result in the verifier.
pub proof fn lemma_l3_max_output_arithmetic()
    ensures
        48 + 130 * 2048 + 2 == 266290,
        8192 * 2048 == 16777216,
        4 * (8192 * 2048) == 67108864,
        4 * (8192 * 2048) <= max_output(),
{
    reveal(max_output);
}

// The one-step unfoldings of `mr_full` the executable command loop needs: one
// lemma per exit (R10/R11/R14 order) and one per continuing command. Each is
// a single unfolding of the model under the branch's own hypotheses.

/// Exit: fewer than two command bytes. `RleTruncated` at the command cursor.
pub proof fn lemma_mr_exit_truncated(b: Seq<u8>, m: MHRecord, p: int, x: int, y: int)
    requires
        p + 2 > b.len(),
    ensures
        mr_gcur(b, m, p, x, y) == Err((RleErr::Api(Error::RleTruncated), p, x, y)),
        mr_writes(b, m, p, x, y) == Seq::<Write>::empty(),
{
    reveal(mr_full);
    reveal(mr_gcur);
    reveal(mr_writes);
}

/// Exit: an encoded run leaves its row or starts past the last row.
pub proof fn lemma_mr_exit_run_bounds(b: Seq<u8>, m: MHRecord, p: int, x: int, y: int)
    requires
        0 <= p, p + 2 <= b.len(), b[p] != 0,
        y >= m.height || x + b[p] as int > m.width,
    ensures
        mr_gcur(b, m, p, x, y) == Err((RleErr::Api(Error::RleBounds), p + 2, x, y)),
        mr_writes(b, m, p, x, y) == Seq::<Write>::empty(),
{
    reveal(mr_full);
    reveal(mr_gcur);
    reveal(mr_writes);
}

/// Exit: an encoded run's first out-of-range index, after `run_good` pixels.
pub proof fn lemma_mr_exit_run_index(b: Seq<u8>, m: MHRecord, p: int, x: int, y: int)
    requires
        0 <= p, p + 2 <= b.len(), b[p] != 0,
        y < m.height, x + b[p] as int <= m.width,
        run_good(m, b[p + 1] as int, b[p] as int, 0) < b[p] as int,
    ensures
        mr_gcur(b, m, p, x, y) == Err((RleErr::Api(Error::PaletteIndexOutOfRange), p + 2, x, y)),
        mr_writes(b, m, p, x, y) == run_writes(
            b, m, x, y, b[p + 1] as int, 0, run_good(m, b[p + 1] as int, b[p] as int, 0),
        ),
{
    reveal(mr_full);
    reveal(mr_gcur);
    reveal(mr_writes);
}

/// Exit: `00 01` EOB. Success at the cursor after the command.
pub proof fn lemma_mr_exit_eob(b: Seq<u8>, m: MHRecord, p: int, x: int, y: int)
    requires
        0 <= p, p + 2 <= b.len(), b[p] == 0, b[p + 1] == 1,
    ensures
        mr_gcur(b, m, p, x, y) == Ok((p + 2, x, y)),
        mr_writes(b, m, p, x, y) == Seq::<Write>::empty(),
{
    reveal(mr_full);
    reveal(mr_gcur);
    reveal(mr_writes);
}

/// Exit: `00 00` EOL after the last row.
pub proof fn lemma_mr_exit_eol_bounds(b: Seq<u8>, m: MHRecord, p: int, x: int, y: int)
    requires
        0 <= p, p + 2 <= b.len(), b[p] == 0, b[p + 1] == 0, y >= m.height,
    ensures
        mr_gcur(b, m, p, x, y) == Err((RleErr::Api(Error::RleBounds), p + 2, x, y)),
        mr_writes(b, m, p, x, y) == Seq::<Write>::empty(),
{
    reveal(mr_full);
    reveal(mr_gcur);
    reveal(mr_writes);
}

/// Step: `00 00` EOL inside the image moves to `(0, y+1)` and paints nothing.
pub proof fn lemma_mr_step_eol(b: Seq<u8>, m: MHRecord, p: int, x: int, y: int)
    requires
        0 <= p, p + 2 <= b.len(), b[p] == 0, b[p + 1] == 0, y < m.height,
    ensures
        mr_gcur(b, m, p, x, y) == mr_gcur(b, m, p + 2, 0, y + 1),
        mr_writes(b, m, p, x, y) == mr_writes(b, m, p + 2, 0, y + 1),
{
    reveal(mr_full);
    reveal(mr_gcur);
    reveal(mr_writes);
}

/// Exit: `00 02` delta without its two offset bytes.
pub proof fn lemma_mr_exit_delta_trunc(b: Seq<u8>, m: MHRecord, p: int, x: int, y: int)
    requires
        0 <= p, p + 2 <= b.len(), b[p] == 0, b[p + 1] == 2, p + 4 > b.len(),
    ensures
        mr_gcur(b, m, p, x, y) == Err((RleErr::Api(Error::RleTruncated), p + 2, x, y)),
        mr_writes(b, m, p, x, y) == Seq::<Write>::empty(),
{
    reveal(mr_full);
    reveal(mr_gcur);
    reveal(mr_writes);
}

/// Exit: a delta that escapes the image, after consuming its four bytes.
pub proof fn lemma_mr_exit_delta_bounds(b: Seq<u8>, m: MHRecord, p: int, x: int, y: int)
    requires
        0 <= p, p + 4 <= b.len(), b[p] == 0, b[p + 1] == 2,
        y + b[p + 3] as int >= m.height || x + b[p + 2] as int >= m.width,
    ensures
        mr_gcur(b, m, p, x, y) == Err((RleErr::Api(Error::RleBounds), p + 4, x, y)),
        mr_writes(b, m, p, x, y) == Seq::<Write>::empty(),
{
    reveal(mr_full);
    reveal(mr_gcur);
    reveal(mr_writes);
}

/// Step: a delta inside the image advances both coordinates and paints nothing.
pub proof fn lemma_mr_step_delta(b: Seq<u8>, m: MHRecord, p: int, x: int, y: int)
    requires
        0 <= p, p + 4 <= b.len(), b[p] == 0, b[p + 1] == 2,
        y + (b[p + 3] as int) < m.height, x + (b[p + 2] as int) < m.width,
    ensures
        mr_gcur(b, m, p, x, y) == mr_gcur(b, m, p + 4, x + b[p + 2] as int, y + b[p + 3] as int),
        mr_writes(b, m, p, x, y) == mr_writes(b, m, p + 4, x + b[p + 2] as int, y + b[p + 3] as int),
{
    reveal(mr_full);
    reveal(mr_gcur);
    reveal(mr_writes);
}

/// Exit: an absolute packet that leaves its row or starts past the last row,
/// checked before any payload byte is required.
pub proof fn lemma_mr_exit_abs_bounds(b: Seq<u8>, m: MHRecord, p: int, x: int, y: int)
    requires
        0 <= p, p + 2 <= b.len(), b[p] == 0, b[p + 1] >= 3,
        y >= m.height || x + b[p + 1] as int > m.width,
    ensures
        mr_gcur(b, m, p, x, y) == Err((RleErr::Api(Error::RleBounds), p + 2, x, y)),
        mr_writes(b, m, p, x, y) == Seq::<Write>::empty(),
{
    reveal(mr_full);
    reveal(mr_gcur);
    reveal(mr_writes);
}

/// Exit: an absolute packet whose whole padded payload is not present.
pub proof fn lemma_mr_exit_abs_trunc(b: Seq<u8>, m: MHRecord, p: int, x: int, y: int)
    requires
        0 <= p, p + 2 <= b.len(), b[p] == 0, b[p + 1] >= 3,
        y < m.height, x + b[p + 1] as int <= m.width,
        p + 2 + rle_padded(b[p + 1] as int, m.depth) > b.len(),
    ensures
        mr_gcur(b, m, p, x, y) == Err((RleErr::Api(Error::RleTruncated), p + 2, x, y)),
        mr_writes(b, m, p, x, y) == Seq::<Write>::empty(),
{
    reveal(mr_full);
    reveal(mr_gcur);
    reveal(mr_writes);
}

/// Exit: an absolute packet's first out-of-range index, after `abs_good`
/// pixels; the cursor has consumed the whole padded packet.
pub proof fn lemma_mr_exit_abs_index(b: Seq<u8>, m: MHRecord, p: int, x: int, y: int)
    requires
        0 <= p, p + 2 <= b.len(), b[p] == 0, b[p + 1] >= 3,
        y < m.height, x + b[p + 1] as int <= m.width,
        p + 2 + rle_padded(b[p + 1] as int, m.depth) <= b.len(),
        abs_good(b, m, p + 2, b[p + 1] as int, 0) < b[p + 1] as int,
    ensures
        mr_gcur(b, m, p, x, y) == Err((
            RleErr::Api(Error::PaletteIndexOutOfRange),
            p + 2 + rle_padded(b[p + 1] as int, m.depth),
            x,
            y,
        )),
        mr_writes(b, m, p, x, y) == abs_writes(
            b, m, x, y, p + 2, 0, abs_good(b, m, p + 2, b[p + 1] as int, 0),
        ),
{
    reveal(mr_full);
    reveal(mr_gcur);
    reveal(mr_writes);
}

/// Step: a complete encoded run, on the cursor/position projection.
pub proof fn lemma_mr_step_run_gcur(b: Seq<u8>, m: MHRecord, p: int, x: int, y: int)
    requires
        0 <= p, p + 2 <= b.len(), b[p] != 0,
        y < m.height, x + b[p] as int <= m.width,
        run_ok(m, b[p + 1] as int, b[p] as int),
    ensures
        mr_gcur(b, m, p, x, y) == mr_gcur(b, m, p + 2, x + b[p] as int, y),
{
    reveal(mr_full);
    reveal(mr_gcur);
    reveal(run_ok);
}

/// Step: a complete absolute packet, on the cursor/position projection.
pub proof fn lemma_mr_step_abs_gcur(b: Seq<u8>, m: MHRecord, p: int, x: int, y: int)
    requires
        0 <= p, p + 2 <= b.len(), b[p] == 0,
        b[p + 1] >= 3, y < m.height,
        x + b[p + 1] as int <= m.width,
        p + 2 + rle_padded(b[p + 1] as int, m.depth) <= b.len(),
        abs_ok(b, m, p + 2, b[p + 1] as int),
    ensures
        mr_gcur(b, m, p, x, y) == mr_gcur(
            b, m, p + 2 + rle_padded(b[p + 1] as int, m.depth), x + b[p + 1] as int, y,
        ),
{
    reveal(mr_full);
    reveal(mr_gcur);
    reveal(abs_ok);
}

/// R10/R11: `MR` at the pixel cursor `p0` and the starting position `(x0, y0)`.
/// Outside T7's domain (`rle_reachable`) it returns the internal `Outside`
/// marker, not a 27th API error (THEOREMS.md).
pub open spec fn mr(
    b: Seq<u8>,
    m: MHRecord,
    p0: int,
    x0: int,
    y0: int,
) -> Result<(int, int, int, Seq<Write>), (RleErr, int, int, int, Seq<Write>)> {
    if rle_reachable(b, m, p0, x0, y0) {
        mr_full(b, m, p0, x0, y0)
    } else {
        Err((RleErr::Outside, p0, x0, y0, Seq::<Write>::empty()))
    }
}

/// The RLE run-index step used by the executable's paint loop: the `j`-th
/// painted index of a run of `q` pixels (RLE8 code byte; RLE4 alternating
/// nibbles, high first).
pub open spec fn run_index_at(code: int, depth: int, j: int) -> int {
    run_index(code, depth, j)
}

/// `run_writes` peels its first pixel: the first write is the pair
/// `(cell, conv(run_index j))`, and the rest is the same list from `j+1`.
pub proof fn lemma_run_writes_unfold(
    b: Seq<u8>,
    m: MHRecord,
    x: int,
    y: int,
    code: int,
    j: int,
    k: int,
)
    requires
        j < k,
    ensures
        run_writes(b, m, x, y, code, j, k) == seq![
            Write { cell: cell_of(m, x + j, y), px: conv(b, m, run_index(code, m.depth, j)) },
        ] + run_writes(b, m, x, y, code, j + 1, k),
{
    reveal(run_writes);
}

/// The same for an absolute packet.
pub proof fn lemma_abs_writes_unfold(
    b: Seq<u8>,
    m: MHRecord,
    x: int,
    y: int,
    pay: int,
    j: int,
    k: int,
)
    requires
        j < k,
    ensures
        abs_writes(b, m, x, y, pay, j, k) == seq![
            Write { cell: cell_of(m, x + j, y), px: conv(b, m, abs_index(b, m.depth, pay, j)) },
        ] + abs_writes(b, m, x, y, pay, j + 1, k),
{
    reveal(abs_writes);
}

/// `run_writes` over `0..q` is its first pixel plus the rest.
pub proof fn lemma_run_writes_full(
    b: Seq<u8>,
    m: MHRecord,
    x: int,
    y: int,
    code: int,
    q: int,
)
    requires
        q >= 1,
    ensures
        run_writes(b, m, x, y, code, 0, q) == seq![
            Write { cell: cell_of(m, x, y), px: conv(b, m, run_index(code, m.depth, 0)) },
        ] + run_writes(b, m, x, y, code, 1, q),
{
    lemma_run_writes_unfold(b, m, x, y, code, 0, q);
}

/// `run_writes` over an empty range is empty.
pub proof fn lemma_run_writes_empty(b: Seq<u8>, m: MHRecord, x: int, y: int, code: int, i: int, k: int)
    requires
        i >= k,
    ensures
        run_writes(b, m, x, y, code, i, k) == Seq::<Write>::empty(),
{
    reveal(run_writes);
}

/// `run_writes` over one position is the singleton write of that pixel.
pub proof fn lemma_run_writes_single(b: Seq<u8>, m: MHRecord, x: int, y: int, code: int, i: int)
    ensures
        run_writes(b, m, x, y, code, i, i + 1) == seq![
            Write { cell: cell_of(m, x + i, y), px: conv(b, m, run_index(code, m.depth, i)) },
        ],
{
    reveal(run_writes);
    lemma_run_writes_empty(b, m, x, y, code, i + 1, i + 1);
    broadcast use vstd::seq_lib::group_seq_lib_default;
}


/// `run_writes` appends its last pixel: the step the executable's painter uses.
pub proof fn lemma_run_writes_snoc(
    b: Seq<u8>,
    m: MHRecord,
    x: int,
    y: int,
    code: int,
    j: int,
    q: int,
)
    requires
        0 <= j < q,
    ensures
        run_writes(b, m, x, y, code, 0, j + 1) == run_writes(b, m, x, y, code, 0, j) + seq![
            Write { cell: cell_of(m, x + j, y), px: conv(b, m, run_index(code, m.depth, j)) },
        ],
{
    lemma_run_writes_snoc_at(b, m, x, y, code, j, 0);
}

/// `run_writes(s, j+1) == run_writes(s, j) + [pixel j]`, for every `s <= j`.
pub proof fn lemma_run_writes_snoc_at(
    b: Seq<u8>,
    m: MHRecord,
    x: int,
    y: int,
    code: int,
    j: int,
    s: int,
)
    requires
        0 <= s <= j,
    ensures
        run_writes(b, m, x, y, code, s, j + 1) == run_writes(b, m, x, y, code, s, j) + seq![
            Write { cell: cell_of(m, x + j, y), px: conv(b, m, run_index(code, m.depth, j)) },
        ],
    decreases j - s,
{
    reveal(run_writes);
    if s == j {
        lemma_run_writes_empty(b, m, x, y, code, j, j);
        lemma_run_writes_single(b, m, x, y, code, j);
        broadcast use vstd::seq_lib::group_seq_lib_default;
    } else {
        let head = seq![
            Write { cell: cell_of(m, x + s, y), px: conv(b, m, run_index(code, m.depth, s)) },
        ];
        assert(run_writes(b, m, x, y, code, s, j + 1) == head + run_writes(
            b,
            m,
            x,
            y,
            code,
            s + 1,
            j + 1,
        ));
        assert(run_writes(b, m, x, y, code, s, j) == head + run_writes(b, m, x, y, code, s + 1, j));
        lemma_run_writes_snoc_at(b, m, x, y, code, j, s + 1);
        vstd::seq_lib::lemma_concat_associative(
            head,
            run_writes(b, m, x, y, code, s + 1, j),
            seq![
                Write { cell: cell_of(m, x + j, y), px: conv(b, m, run_index(code, m.depth, j)) },
            ],
        );
    }
}

/// R10: the run index in the executable's own arithmetic. For a byte value
/// `code` at depth 8 it is `code`; at depth 4 it is `code/16` at even `j` and
/// `code%16` at odd `j`. This is the exact form the executable's bit ops
/// produce, so the painter's index equals the model's `run_index`.
pub proof fn lemma_run_index_eq(code: int, depth: int, j: int)
    requires
        0 <= code < 256,
        depth == 4 || depth == 8,
        j >= 0,
    ensures
        run_index(code, depth, j) == (if depth == 8 {
            code
        } else if j % 2 == 0 {
            code / 16
        } else {
            code % 16
        }),
{
    reveal(run_index);
}

/// Unpack a `forall k < i: quad_at(out, 4k, v)` at byte index `idx < 4*i`.
pub proof fn lemma_quad_at_index(out: Seq<u8>, idx: int, i: int, v: Seq<u8>)
    requires
        forall|k: int| 0 <= k < i ==> #[trigger] quad_at(out, 4 * k, v),
        0 <= idx < 4 * i,
        v.len() == 4,
    ensures
        out[idx] == v[idx % 4],
{
    let k = idx / 4;
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod(idx, 4);
    assert(idx == 4 * k + idx % 4);
    assert(0 <= idx % 4 < 4);
    assert(0 <= k < i) by (nonlinear_arith)
        requires
            idx == 4 * k + idx % 4,
            0 <= idx % 4 < 4,
            0 <= idx < 4 * i;
    reveal(quad_at);
    assert(quad_at(out, 4 * k, v));
    assert(out[4 * k] == v[0]);
    assert(out[4 * k + 1] == v[1]);
    assert(out[4 * k + 2] == v[2]);
    assert(out[4 * k + 3] == v[3]);
}

/// If every `4*i` byte of `out` is `v[i%4]` up to `4*n`, then `out` is the
/// `pal_zero_bytes` fill: the byte-level form of the RLE prefill.
pub proof fn lemma_quad_at_to_pal_zero(
    out: Seq<u8>,
    b: Seq<u8>,
    m: MHRecord,
    fill: Seq<u8>,
)
    requires
        out.len() == 4 * m.n,
        fill.len() == 4 * m.n,
        fill == pal_zero_bytes(b, m),
        forall|k: int| 0 <= k < m.n ==> #[trigger] quad_at(
            out,
            4 * k,
            pal_entry(b, m.palette_start, m.palette_stride, 0),
        ),
    ensures
        out == fill,
{
    reveal(pal_zero_bytes);
    assert(out.len() == fill.len());
    assert(out =~= fill) by {
        broadcast use vstd::seq::group_seq_lemmas;
        assert forall|idx: int| 0 <= idx < out.len() implies out[idx] == #[trigger] fill[idx] by {
            lemma_quad_at_index(out, idx, m.n, pal_entry(b, m.palette_start, m.palette_stride, 0));
        }
    }
}

/// The pack form of `pal_zero_bytes`: byte `idx` is `pal_entry(0)[idx % 4]`.
pub proof fn lemma_pal_zero_bytes_index(b: Seq<u8>, m: MHRecord, idx: int)
    requires
        0 <= idx < 4 * m.n,
    ensures
        pal_zero_bytes(b, m)[idx] == pal_entry(b, m.palette_start, m.palette_stride, 0)[idx % 4],
{
    reveal(pal_zero_bytes);
    broadcast use vstd::seq::group_seq_lemmas;
}

/// `pal_zero_bytes` has exactly `4*n` bytes.
pub proof fn lemma_pal_zero_bytes_len(b: Seq<u8>, m: MHRecord)
    requires
        m.n >= 0,
    ensures
        pal_zero_bytes(b, m).len() == 4 * m.n,
{
    reveal(pal_zero_bytes);
    broadcast use vstd::seq::group_seq_lemmas;
}

/// Overwriting pixel `i` (bytes `4i`..`4i+4`) of `pal_zero_bytes` with the same
/// palette entry leaves it unchanged: the R8 prefill is idempotent.
pub proof fn lemma_pal_zero_put(b: Seq<u8>, m: MHRecord, i: int)
    requires
        0 <= i < m.n,
        pal_entry(b, m.palette_start, m.palette_stride, 0).len() == 4,
    ensures
        replay_bytes_at(
            pal_zero_bytes(b, m),
            4 * i,
            pal_entry(b, m.palette_start, m.palette_stride, 0),
        ) == pal_zero_bytes(b, m),
{
    let v = pal_entry(b, m.palette_start, m.palette_stride, 0);
    let base = pal_zero_bytes(b, m);
    reveal(replay_bytes_at);
    lemma_pal_zero_bytes_len(b, m);
    assert(replay_bytes_at(base, 4 * i, v).len() == base.len());
    assert(replay_bytes_at(base, 4 * i, v) =~= base) by {
        reveal(pal_zero_bytes);
        broadcast use vstd::seq::group_seq_lemmas;
        assert forall|idx: int| 0 <= idx < base.len() implies replay_bytes_at(
            base,
            4 * i,
            v,
        )[idx] == #[trigger] base[idx] by {
            vstd::arithmetic::div_mod::lemma_fundamental_div_mod(idx, 4);
            let q = idx / 4;
            let r = idx % 4;
            assert(idx == 4 * q + r);
            assert(0 <= r < 4);
            assert(0 <= q < m.n) by (nonlinear_arith)
                requires
                    idx == 4 * q + r,
                    0 <= r < 4,
                    0 <= idx < 4 * m.n;
            assert(base[idx] == v[r]);
            if idx == 4 * i {
                reveal(replay_bytes_at);
                vstd::seq::lemma_seq_update_same(base, 4 * i, v[0]);
                assert(replay_bytes_at(base, 4 * i, v)[idx] == v[0]);
            } else if idx == 4 * i + 1 {
                reveal(replay_bytes_at);
                vstd::seq::lemma_seq_update_different(base, idx, 4 * i, v[0]);
                vstd::seq::lemma_seq_update_same(base.update(4 * i, v[0]), 4 * i + 1, v[1]);
                assert(replay_bytes_at(base, 4 * i, v)[idx] == v[1]);
            } else if idx == 4 * i + 2 {
                reveal(replay_bytes_at);
                vstd::seq::lemma_seq_update_different(base, idx, 4 * i, v[0]);
                vstd::seq::lemma_seq_update_different(base.update(4 * i, v[0]), idx, 4 * i + 1, v[1]);
                vstd::seq::lemma_seq_update_same(
                    base.update(4 * i, v[0]).update(4 * i + 1, v[1]),
                    4 * i + 2,
                    v[2],
                );
                assert(replay_bytes_at(base, 4 * i, v)[idx] == v[2]);
            } else if idx == 4 * i + 3 {
                reveal(replay_bytes_at);
                vstd::seq::lemma_seq_update_different(base, idx, 4 * i, v[0]);
                vstd::seq::lemma_seq_update_different(base.update(4 * i, v[0]), idx, 4 * i + 1, v[1]);
                vstd::seq::lemma_seq_update_different(
                    base.update(4 * i, v[0]).update(4 * i + 1, v[1]),
                    idx,
                    4 * i + 2,
                    v[2],
                );
                vstd::seq::lemma_seq_update_same(
                    base.update(4 * i, v[0]).update(4 * i + 1, v[1]).update(4 * i + 2, v[2]),
                    4 * i + 3,
                    v[3],
                );
                assert(replay_bytes_at(base, 4 * i, v)[idx] == v[3]);
            } else {
                reveal(replay_bytes_at);
                vstd::seq::lemma_seq_update_different(base, idx, 4 * i, v[0]);
                vstd::seq::lemma_seq_update_different(base.update(4 * i, v[0]), idx, 4 * i + 1, v[1]);
                vstd::seq::lemma_seq_update_different(
                    base.update(4 * i, v[0]).update(4 * i + 1, v[1]),
                    idx,
                    4 * i + 2,
                    v[2],
                );
                vstd::seq::lemma_seq_update_different(
                    base.update(4 * i, v[0]).update(4 * i + 1, v[1]).update(4 * i + 2, v[2]),
                    idx,
                    4 * i + 3,
                    v[3],
                );
                assert(replay_bytes_at(base, 4 * i, v)[idx] == base[idx]);
            }
        }
    }
}

/// /// `run_good(q,j)` is 0 exactly when index `j` is out of range, and otherwise
/// is `1 + run_good(q,j+1)`: the unfold used by the executable painter.
pub proof fn lemma_run_good_unfold(m: MHRecord, code: int, q: int, j: int)
    requires
        0 <= j < q,
    ensures
        run_good(m, code, q, j) == (if !index_ok(m, run_index(code, m.depth, j)) {
            0
        } else {
            1 + run_good(m, code, q, j + 1)
        }),
{
    reveal(run_good);
}

/// When every index before `j` is in range and index `j` is out of range, the
/// good prefix is exactly `j`.
pub proof fn lemma_run_good_prefix(m: MHRecord, code: int, q: int, j: int)
    requires
        0 <= j < q,
        forall|i: int| 0 <= i < j ==> #[trigger] index_ok(m, run_index(code, m.depth, i)),
        !index_ok(m, run_index(code, m.depth, j)),
    ensures
        run_good(m, code, q, 0) == j,
{
    lemma_run_good_from(m, code, q, j, 0);
}

/// Auxiliary: with the prefix hypothesis, `run_good(q, i) == j - i` for every
/// `i` in `[0, j]`.
pub proof fn lemma_run_good_from(m: MHRecord, code: int, q: int, j: int, i: int)
    requires
        0 <= j < q,
        0 <= i <= j,
        forall|t: int| 0 <= t < j ==> #[trigger] index_ok(m, run_index(code, m.depth, t)),
        !index_ok(m, run_index(code, m.depth, j)),
    ensures
        run_good(m, code, q, i) == j - i,
    decreases j - i,
{
    reveal(run_good);
    if i == j {
        lemma_run_good_unfold(m, code, q, j);
        assert(run_good(m, code, q, j) == 0);
    } else {
        lemma_run_good_unfold(m, code, q, i);
        assert(index_ok(m, run_index(code, m.depth, i)));
        lemma_run_good_from(m, code, q, j, i + 1);
    }
}

/// `run_good`/`abs_good` analogues for `abs_writes`: unfold, single, snoc, empty.
pub proof fn lemma_abs_writes_empty(
    b: Seq<u8>,
    m: MHRecord,
    x: int,
    y: int,
    pay: int,
    i: int,
    k: int,
)
    requires
        i >= k,
    ensures
        abs_writes(b, m, x, y, pay, i, k) == Seq::<Write>::empty(),
{
    reveal(abs_writes);
}

pub proof fn lemma_abs_writes_single(
    b: Seq<u8>,
    m: MHRecord,
    x: int,
    y: int,
    pay: int,
    i: int,
)
    ensures
        abs_writes(b, m, x, y, pay, i, i + 1) == seq![
            Write { cell: cell_of(m, x + i, y), px: conv(b, m, abs_index(b, m.depth, pay, i)) },
        ],
{
    reveal(abs_writes);
    lemma_abs_writes_empty(b, m, x, y, pay, i + 1, i + 1);
    broadcast use vstd::seq_lib::group_seq_lib_default;
}

pub proof fn lemma_abs_writes_snoc_at(
    b: Seq<u8>,
    m: MHRecord,
    x: int,
    y: int,
    pay: int,
    j: int,
    s: int,
)
    requires
        0 <= s <= j,
    ensures
        abs_writes(b, m, x, y, pay, s, j + 1) == abs_writes(b, m, x, y, pay, s, j) + seq![
            Write { cell: cell_of(m, x + j, y), px: conv(b, m, abs_index(b, m.depth, pay, j)) },
        ],
    decreases j - s,
{
    reveal(abs_writes);
    if s == j {
        lemma_abs_writes_empty(b, m, x, y, pay, j, j);
        lemma_abs_writes_single(b, m, x, y, pay, j);
        broadcast use vstd::seq_lib::group_seq_lib_default;
    } else {
        let head = seq![
            Write { cell: cell_of(m, x + s, y), px: conv(b, m, abs_index(b, m.depth, pay, s)) },
        ];
        assert(abs_writes(b, m, x, y, pay, s, j + 1) == head + abs_writes(
            b,
            m,
            x,
            y,
            pay,
            s + 1,
            j + 1,
        ));
        assert(abs_writes(b, m, x, y, pay, s, j) == head + abs_writes(b, m, x, y, pay, s + 1, j));
        lemma_abs_writes_snoc_at(b, m, x, y, pay, j, s + 1);
        vstd::seq_lib::lemma_concat_associative(
            head,
            abs_writes(b, m, x, y, pay, s + 1, j),
            seq![
                Write { cell: cell_of(m, x + j, y), px: conv(b, m, abs_index(b, m.depth, pay, j)) },
            ],
        );
    }
}

/// `run_good(q,0) == q` is exactly `run_ok`.
pub proof fn lemma_run_good_is_run_ok(m: MHRecord, code: int, q: int)
    requires
        run_good(m, code, q, 0) == q,
    ensures
        run_ok(m, code, q),
{
    reveal(run_ok);
}

/// `abs_good(q,0) == q` is exactly `abs_ok`.
pub proof fn lemma_abs_good_is_abs_ok(b: Seq<u8>, m: MHRecord, pay: int, q: int)
    requires
        abs_good(b, m, pay, q, 0) == q,
    ensures
        abs_ok(b, m, pay, q),
{
    reveal(abs_ok);
}

/// If every run index is in range, the good prefix is the whole run.
pub proof fn lemma_run_good_all_ok(m: MHRecord, code: int, q: int)
    requires
        0 <= q,
        run_ok(m, code, q),
    ensures
        run_good(m, code, q, 0) == q,
{
    reveal(run_ok);
}

/// If every absolute index is in range, the good prefix is the whole packet.
pub proof fn lemma_abs_good_all_ok(b: Seq<u8>, m: MHRecord, pay: int, q: int)
    requires
        0 <= q,
        abs_ok(b, m, pay, q),
    ensures
        abs_good(b, m, pay, q, 0) == q,
{
    reveal(abs_ok);
}

/// `cpal_zero_bytes` is `bytes_replay` of itself over no writes.
pub proof fn lemma_no_paints_rle(b: Seq<u8>, m: MHRecord)
    ensures
        bytes_replay(pal_zero_bytes(b, m), Seq::<Write>::empty()) == pal_zero_bytes(b, m),
{
    reveal(bytes_replay);
}

/// `abs_writes` over `0..q` is its first pixel plus the rest.
pub proof fn lemma_abs_writes_full(
    b: Seq<u8>,
    m: MHRecord,
    x: int,
    y: int,
    pay: int,
    q: int,
)
    requires
        q >= 1,
    ensures
        abs_writes(b, m, x, y, pay, 0, q) == seq![
            Write { cell: cell_of(m, x, y), px: conv(b, m, abs_index(b, m.depth, pay, 0)) },
        ] + abs_writes(b, m, x, y, pay, 1, q),
{
    reveal(abs_writes);
}

/// `abs_good(q,j)` is 0 exactly when index `j` is out of range.
pub proof fn lemma_abs_good_unfold(b: Seq<u8>, m: MHRecord, pay: int, q: int, j: int)
    requires
        0 <= j < q,
    ensures
        abs_good(b, m, pay, q, j) == (if !index_ok(m, abs_index(b, m.depth, pay, j)) {
            0
        } else {
            1 + abs_good(b, m, pay, q, j + 1)
        }),
{
    reveal(abs_good);
}

pub proof fn lemma_abs_good_prefix(b: Seq<u8>, m: MHRecord, pay: int, q: int, j: int)
    requires
        0 <= j < q,
        forall|i: int| 0 <= i < j ==> #[trigger] index_ok(m, abs_index(b, m.depth, pay, i)),
        !index_ok(m, abs_index(b, m.depth, pay, j)),
    ensures
        abs_good(b, m, pay, q, 0) == j,
{
    lemma_abs_good_from(b, m, pay, q, j, 0);
}

pub proof fn lemma_abs_good_from(b: Seq<u8>, m: MHRecord, pay: int, q: int, j: int, i: int)
    requires
        0 <= j < q,
        0 <= i <= j,
        forall|t: int| 0 <= t < j ==> #[trigger] index_ok(m, abs_index(b, m.depth, pay, t)),
        !index_ok(m, abs_index(b, m.depth, pay, j)),
    ensures
        abs_good(b, m, pay, q, i) == j - i,
    decreases j - i,
{
    reveal(abs_good);
    if i == j {
        lemma_abs_good_unfold(b, m, pay, q, j);
        assert(abs_good(b, m, pay, q, j) == 0);
    } else {
        lemma_abs_good_unfold(b, m, pay, q, i);
        assert(index_ok(m, abs_index(b, m.depth, pay, i)));
        lemma_abs_good_from(b, m, pay, q, j, i + 1);
    }
}

/// T7's whole-image form: the RLE writes replayed onto the R8 palette-zero fill.
pub open spec fn mr_out(
    b: Seq<u8>,
    m: MHRecord,
    p0: int,
    x0: int,
    y0: int,
) -> Result<(int, Seq<u8>), (RleErr, int, int, int)> {
    match mr_full(b, m, p0, x0, y0) {
        Ok((pc, _xc, _yc, ws)) => Ok((pc, bytes_replay(pal_zero_bytes(b, m), ws))),
        Err((e, pc, xc, yc, _ws)) => Err((e, pc, xc, yc)),
    }
}

// ---------------------------------------------------------------------------
// MA · the ICO AND mask (T8)
// ---------------------------------------------------------------------------

/// R12: the 1-bpp AND stride `4*ceil(w/32)`, as `stride_of(w, 1)`.
pub open spec fn and_stride(w: int) -> int {
    stride_of(w, 1)
}

/// R12: the AND plane size `A = height * stride(w, 1)`.
pub open spec fn and_bytes(w: int, h: int) -> int {
    h * and_stride(w)
}

/// R12's permitted `biSizeImage` values: `{0, X, X+A}` in ICO, `{0, X}` else.
pub open spec fn image_size_ok(declared: int, x_len: int, a_bytes: int, ico: bool) -> bool {
    declared == 0 || declared == x_len || (ico && declared == x_len + a_bytes)
}

/// R12: the AND bit of pixel `(x, r)` in the bottom-up mask: bit `0x80>>(x%8)`
/// of byte `floor(x/8)` of AND row `r`.
pub open spec fn and_bit(b: Seq<u8>, pos: int, w: int, x: int, r: int) -> bool {
    (b[pos + r * and_stride(w) + x / 8] & (0x80u8 >> ((x % 8) as usize))) != 0u8
}

/// Auxiliary predicate for a complete available AND mask. Unlike the
/// authoritative `ma_domain`, it requires the mask bytes to be present and
/// therefore excludes the `TruncatedAndMask` error path of `ma`.
pub open spec fn ma_reachable(b: Seq<u8>, m: MHRecord, pos: int) -> bool {
    &&& is_ico(m.mode)
    &&& 1 <= m.width <= max_dimension()
    &&& 1 <= m.height <= max_dimension()
    &&& m.n == m.width * m.height
    &&& 0 <= pos <= b.len()
    &&& pos + and_bytes(m.width, m.height) <= b.len()
}

/// R12: the per-pixel AND fallback as a write list. `has_xor_alpha` is true
/// exactly when the XOR plane has an explicit alpha channel and some XOR alpha
/// byte is nonzero (the whole-image policy). Bit one forces alpha zero; with no
/// explicit alpha, bit zero leaves the XOR alpha unchanged (no write); with
/// explicit alpha, bit zero forces alpha 255.
pub open spec fn and_writes(
    b: Seq<u8>,
    m: MHRecord,
    pos: int,
    x: int,
    y: int,
    has_xor_alpha: bool,
) -> Seq<Write>
    decreases (m.height - y) as nat, (m.width - x) as nat,
{
    if y >= m.height {
        Seq::<Write>::empty()
    } else if x >= m.width {
        and_writes(b, m, pos, 0, y + 1, has_xor_alpha)
    } else {
        let bit = and_bit(b, pos, m.width, x, y);
        let w = if bit {
            seq![
                Write {
                    cell: cell_of(m, x, y),
                    px: seq![0u8, 0u8, 0u8, 0u8],
                },
            ]
        } else if has_xor_alpha {
            seq![
                Write {
                    cell: cell_of(m, x, y),
                    px: seq![0u8, 0u8, 0u8, 255u8],
                },
            ]
        } else {
            Seq::<Write>::empty()
        };
        w + and_writes(b, m, pos, x + 1, y, has_xor_alpha)
    }
}

/// True when the complete XOR plane contains a nonzero alpha byte.
pub open spec fn xor_alpha_present(out: Seq<u8>, n: int) -> bool {
    exists|i: int| 0 <= i < n && #[trigger] out[4 * i + 3] != 0
}

/// The alpha byte after the ICO whole-image alpha policy. The source row of
/// cell `cell` is `height-1-cell/width` because ICO XOR and AND are bottom-up.
pub open spec fn ma_byte(
    b: Seq<u8>, m: MHRecord, pos: int, original: Seq<u8>, explicit: bool, k: int,
) -> u8 {
    if k % 4 != 3 {
        original[k]
    } else {
        let cell = k / 4;
        let x = cell % m.width;
        let y = m.height - 1 - cell / m.width;
        if explicit && xor_alpha_present(original, m.n) {
            original[k]
        } else if and_bit(b, pos, m.width, x, y) {
            0u8
        } else if explicit {
            255u8
        } else {
            original[k]
        }
    }
}

pub open spec fn ma_source(m: MHRecord, k: int) -> int {
    let cell = k / 4;
    (m.height - 1 - cell / m.width) * m.width + cell % m.width
}

/// The inverse of the ICO bottom-up cell mapping for one source pixel.
pub proof fn lemma_ma_source_cell(m: MHRecord, x: int, y: int)
    requires 1 <= m.width, 0 <= x < m.width, 0 <= y < m.height,
    ensures ma_source(m, 4 * ((m.height - 1 - y) * m.width + x) + 3)
        == y * m.width + x,
        (4 * ((m.height - 1 - y) * m.width + x) + 3) % 4 == 3,
        (4 * ((m.height - 1 - y) * m.width + x) + 3) / 4 % m.width == x,
        m.height - 1 - (4 * ((m.height - 1 - y) * m.width + x) + 3) / 4 / m.width == y,
{
    reveal(ma_source);
    let row = m.height - 1 - y;
    let cell = row * m.width + x;
    assert(cell / m.width == row && cell % m.width == x)
        by (nonlinear_arith)
        requires m.width >= 1, 0 <= x < m.width, row >= 0,
            cell == row * m.width + x;
    assert((4 * cell + 3) / 4 == cell) by (nonlinear_arith)
        requires cell >= 0;
}

/// The mask pass after `done` pixels in bottom-up source order.
pub open spec fn ma_prefix(
    b: Seq<u8>, m: MHRecord, pos: int, original: Seq<u8>, explicit: bool, done: int,
) -> Seq<u8> {
    Seq::new(original.len(), |k: int|
        if k % 4 == 3 && ma_source(m, k) < done {
            ma_byte(b, m, pos, original, explicit, k)
        } else {
            original[k]
        })
}

/// Advancing the mask over one pixel changes only that pixel's alpha byte.
pub proof fn lemma_ma_prefix_step(
    b: Seq<u8>, m: MHRecord, pos: int, original: Seq<u8>, explicit: bool,
    x: int, y: int,
)
    requires
        1 <= m.width <= 8192, 1 <= m.height <= 8192, m.n == m.width * m.height,
        0 <= x < m.width, 0 <= y < m.height,
        original.len() == 4 * m.n,
    ensures
        ma_prefix(b, m, pos, original, explicit, y * m.width + x + 1)
            == ma_prefix(b, m, pos, original, explicit, y * m.width + x).update(
                4 * ((m.height - 1 - y) * m.width + x) + 3,
                ma_byte(b, m, pos, original, explicit,
                    4 * ((m.height - 1 - y) * m.width + x) + 3)),
{
    let before = ma_prefix(b, m, pos, original, explicit, y * m.width + x);
    let after = ma_prefix(b, m, pos, original, explicit, y * m.width + x + 1);
    let offset = 4 * ((m.height - 1 - y) * m.width + x) + 3;
    let row = m.height - 1 - y;
    assert(0 <= row < m.height);
    assert(0 <= row * m.width + x < m.n) by (nonlinear_arith)
        requires 0 <= row < m.height, 0 <= x < m.width,
            m.width >= 1, m.n == m.width * m.height;
    assert(0 <= offset < original.len()) by (nonlinear_arith)
        requires 0 <= row * m.width + x < m.n, row == m.height - 1 - y,
            offset == 4 * (row * m.width + x) + 3, original.len() == 4 * m.n;
    assert(after =~= before.update(offset, ma_byte(b, m, pos, original, explicit, offset))) by {
        assert forall|k: int| 0 <= k < original.len() implies
            #[trigger] after[k] == before.update(offset,
                ma_byte(b, m, pos, original, explicit, offset))[k] by {
            reveal(ma_prefix);
            reveal(ma_source);
            if k == offset {
                lemma_ma_source_cell(m, x, y);
                assert(ma_source(m, k) == y * m.width + x);
            } else if k % 4 == 3 {
                let cell = k / 4;
                assert(k == 4 * cell + 3) by (nonlinear_arith)
                    requires k % 4 == 3, cell == k / 4;
                assert(0 <= cell < m.n) by (nonlinear_arith)
                    requires 0 <= k < original.len(), original.len() == 4 * m.n,
                        k == 4 * cell + 3;
                let quotient = cell / m.width;
                let yy = m.height - 1 - quotient;
                let xx = cell % m.width;
                assert(quotient * m.width <= cell) by (nonlinear_arith)
                    requires m.width >= 1, quotient == cell / m.width;
                assert(0 <= quotient);
                assert(quotient < m.height) by {
                    if quotient >= m.height {
                        assert(quotient * m.width >= m.height * m.width)
                            by (nonlinear_arith)
                            requires quotient >= m.height, m.width >= 1;
                        assert(cell < m.height * m.width) by (nonlinear_arith)
                            requires cell < m.n, m.n == m.width * m.height;
                    }
                }
                assert(0 <= yy < m.height);
                assert(0 <= xx < m.width) by (nonlinear_arith)
                    requires m.width >= 1, xx == cell % m.width;
                assert(cell == quotient * m.width + xx) by (nonlinear_arith)
                    requires cell >= 0, m.width >= 1, quotient == cell / m.width,
                        xx == cell % m.width;
                assert(ma_source(m, k) == yy * m.width + xx);
                if ma_source(m, k) == y * m.width + x {
                    lemma_vertical_flip_bijection(m.width, m.height, yy, y, xx, x);
                    assert(yy == y && xx == x);
                    assert(k == offset) by (nonlinear_arith)
                        requires k == 4 * cell + 3,
                            cell == quotient * m.width + xx,
                            yy == m.height - 1 - quotient,
                            yy == y, xx == x,
                            offset == 4 * ((m.height - 1 - y) * m.width + x) + 3;
                }
            }
        }
    }
}

pub proof fn lemma_ma_prefix_ends(
    b: Seq<u8>, m: MHRecord, pos: int, original: Seq<u8>, explicit: bool,
)
    requires
        1 <= m.width, 1 <= m.height, m.n == m.width * m.height,
        original.len() == 4 * m.n,
    ensures
        ma_prefix(b, m, pos, original, explicit, 0) == original,
        ma_prefix(b, m, pos, original, explicit, m.n) ==
            ma_output(b, m, pos, original, explicit),
{
    reveal(ma_prefix);
    reveal(ma_source);
    reveal(ma_output);
    assert(ma_prefix(b, m, pos, original, explicit, 0) =~= original) by {
        assert forall|k: int| 0 <= k < original.len() implies
            #[trigger] ma_prefix(b, m, pos, original, explicit, 0)[k] == original[k] by {
            if k % 4 == 3 {
                assert(0 <= ma_source(m, k)) by (nonlinear_arith)
                    requires 0 <= k < original.len(), original.len() == 4 * m.n,
                        m.n == m.width * m.height, m.width >= 1, m.height >= 1;
            }
        }
    }
    assert(ma_prefix(b, m, pos, original, explicit, m.n) =~=
        ma_output(b, m, pos, original, explicit)) by {
        assert forall|k: int| 0 <= k < original.len() implies
            #[trigger] ma_prefix(b, m, pos, original, explicit, m.n)[k]
                == ma_output(b, m, pos, original, explicit)[k] by {
            if k % 4 == 3 {
                assert(ma_source(m, k) < m.n) by (nonlinear_arith)
                    requires 0 <= k < original.len(), original.len() == 4 * m.n,
                        m.n == m.width * m.height, m.width >= 1, m.height >= 1;
            }
        }
    }
}

/// MA's exact output, including the unchanged RGB channels.
pub open spec fn ma_output(
    b: Seq<u8>, m: MHRecord, pos: int, original: Seq<u8>, explicit: bool,
) -> Seq<u8> {
    Seq::new(original.len(), |k: int| ma_byte(b, m, pos, original, explicit, k))
}

/// Any nonzero explicit alpha byte makes every AND bit irrelevant.
pub proof fn lemma_ma_explicit_alpha_keep(
    b: Seq<u8>, m: MHRecord, pos: int, original: Seq<u8>,
)
    requires xor_alpha_present(original, m.n),
    ensures ma_output(b, m, pos, original, true) == original,
{
    reveal(ma_output);
    reveal(ma_byte);
    assert(ma_output(b, m, pos, original, true) =~= original) by {
        assert forall|k: int| 0 <= k < original.len() implies
            #[trigger] ma_output(b, m, pos, original, true)[k] == original[k] by {
        }
    }
}

/// The all-zero explicit-alpha branch of the ICO mask policy, with no
/// quantifier inside its per-byte definition. This is equal to `ma_output`
/// only when the entire XOR plane has zero alpha.
pub open spec fn ma_fallback_byte(
    b: Seq<u8>, m: MHRecord, pos: int, original: Seq<u8>, k: int,
) -> u8 {
    if k % 4 != 3 {
        original[k]
    } else {
        let cell = k / 4;
        let x = cell % m.width;
        let y = m.height - 1 - cell / m.width;
        if and_bit(b, pos, m.width, x, y) { 0u8 } else { 255u8 }
    }
}

pub proof fn lemma_ma_explicit_alpha_zero_fallback(
    b: Seq<u8>, m: MHRecord, pos: int, original: Seq<u8>,
)
    requires !xor_alpha_present(original, m.n),
    ensures ma_output(b, m, pos, original, true)
        == Seq::new(original.len(), |k: int| ma_fallback_byte(b, m, pos, original, k)),
{
    reveal(ma_output);
    reveal(ma_byte);
    reveal(ma_fallback_byte);
    let fallback = Seq::new(original.len(), |k: int| ma_fallback_byte(b, m, pos, original, k));
    assert(ma_output(b, m, pos, original, true) =~= fallback) by {
        assert forall|k: int| 0 <= k < original.len() implies
            #[trigger] ma_output(b, m, pos, original, true)[k] == fallback[k] by {
            reveal(ma_output);
            reveal(ma_byte);
            reveal(ma_fallback_byte);
        }
    }
}

/// MA's complete mask transition at a reachable XOR state. Unlike a write
/// list, this carries the complete input image and preserves every RGB byte.
pub open spec fn ma_exact(
    b: Seq<u8>, m: MHRecord, pos: int, original: Seq<u8>, explicit: bool,
) -> Result<(int, Seq<u8>), MaErr> {
    let a = and_bytes(m.width, m.height);
    let x_len = pos - m.pixel_start;
    if !image_size_ok(m.declared_image, x_len, a, true) {
        Err(MaErr::Api(Error::InvalidImageSize))
    } else if pos + a > b.len() {
        Err(MaErr::Api(Error::TruncatedAndMask))
    } else {
        Ok((pos + a, ma_output(b, m, pos, original, explicit)))
    }
}

/// T8's error wrapper (the manifest's `OutsideDomain` is not an API error).
pub enum MaErr {
    Api(Error),
    Outside,
}

/// The AND transition in write-list form: given the XOR plane's consumed
/// length `x_len` and the mask position, the exact error or the AND write list
/// of the *applied* fallback (`has_xor_alpha` = the XOR plane has an explicit
/// alpha channel whose bytes are all zero, so bit zero forces 255). The R12
/// image-size check precedes mask availability. The whole-image selection
/// between this list and the untouched XOR plane is made by `ma`;
/// `lemma_ma_mask_connect` ties this list to `ma`'s byte output.
pub open spec fn ma_mask(
    b: Seq<u8>,
    m: MHRecord,
    pos: int,
    x_len: int,
    has_xor_alpha: bool,
) -> Result<Seq<Write>, MaErr> {
    let a = and_bytes(m.width, m.height);
    if !image_size_ok(m.declared_image, x_len, a, true) {
        Err(MaErr::Api(Error::InvalidImageSize))
    } else if pos + a > b.len() {
        Err(MaErr::Api(Error::TruncatedAndMask))
    } else {
        Ok(and_writes(b, m, pos, 0, 0, has_xor_alpha))
    }
}

/// R8/R12: the XOR plane carries an explicit alpha channel — 32-bpp BI_RGB in
/// ICO (the A byte) or BITFIELDS with a nonzero A mask.
pub open spec fn explicit_alpha(m: MHRecord) -> bool {
    m.depth == 32 && (m.compression == 0 || m.mask_a != 0)
}

/// T8's domain: a reachable complete XOR state in `IcoEntryDib` — a checked
/// ICO record (R3 rejects top-down ICO), the pixel cursor inside the slice, and
/// the complete `4N`-byte XOR image. Mask availability is *not* part of the
/// domain: `TruncatedAndMask` is one of MA's two API results.
pub open spec fn ma_domain(b: Seq<u8>, m: MHRecord, pos: int, original: Seq<u8>) -> bool {
    &&& is_ico(m.mode)
    &&& 1 <= m.width <= max_dimension()
    &&& 1 <= m.height <= max_dimension()
    &&& m.n == m.width * m.height
    &&& !m.top_down
    &&& m.pixel_start <= pos <= b.len()
    &&& original.len() == 4 * m.n
}

/// `MA(b, H, out, p)` (THEOREMS.md): on a reachable complete XOR state it
/// computes `A`, checks the R12 `biSizeImage` rule before mask availability,
/// requires `A` mask bytes, then selects the XOR plane (some explicit alpha
/// nonzero) or the bottom-up AND fallback for the whole image, returning
/// `(p', out')` or the exact `InvalidImageSize`/`TruncatedAndMask`. On any
/// other state it returns the internal `Outside` marker, not an API error.
pub open spec fn ma(
    b: Seq<u8>, m: MHRecord, pos: int, original: Seq<u8>,
) -> Result<(int, Seq<u8>), MaErr> {
    if !ma_domain(b, m, pos, original) {
        Err(MaErr::Outside)
    } else {
        ma_exact(b, m, pos, original, explicit_alpha(m))
    }
}

/// One AND write acts on the alpha byte only: `px[3]` at byte `4*cell+3`. The
/// zero RGB components of `and_writes` are placeholders, not writes.
pub open spec fn alpha_one(base: Seq<u8>, w: Write) -> Seq<u8> {
    base.update(4 * w.cell + 3, w.px[3])
}

/// The AND write list applied to the alpha bytes of a buffer, in order.
pub open spec fn alpha_replay(base: Seq<u8>, ws: Seq<Write>) -> Seq<u8>
    decreases ws.len(),
{
    if ws.len() == 0 {
        base
    } else {
        alpha_replay(alpha_one(base, ws[0]), ws.drop_first())
    }
}

/// The AND list from pixel `(x, y)` on, applied to the mask pass through the
/// first `y*w + x` bottom-up pixels, is the whole mask output — the inductive
/// link between the write-list model and the byte-level one, while the fallback
/// is active (no explicit alpha byte is nonzero).
pub proof fn lemma_and_writes_prefix(
    b: Seq<u8>, m: MHRecord, pos: int, original: Seq<u8>, explicit: bool, x: int, y: int,
)
    requires
        1 <= m.width <= 8192, 1 <= m.height <= 8192, m.n == m.width * m.height,
        !m.top_down,
        original.len() == 4 * m.n,
        0 <= x <= m.width, 0 <= y <= m.height,
        y >= m.height ==> x == 0,
        !(explicit && xor_alpha_present(original, m.n)),
    ensures
        alpha_replay(
            ma_prefix(b, m, pos, original, explicit, y * m.width + x),
            and_writes(b, m, pos, x, y, explicit),
        ) == ma_output(b, m, pos, original, explicit),
    decreases (m.height - y) as nat, (m.width - x) as nat,
{
    reveal(and_writes);
    reveal(alpha_replay);
    broadcast use vstd::seq_lib::group_seq_lib_default;
    if y >= m.height {
        assert(y == m.height && x == 0);
        assert(y * m.width + x == m.n) by (nonlinear_arith)
            requires y == m.height, x == 0, m.n == m.width * m.height;
        lemma_ma_prefix_ends(b, m, pos, original, explicit);
        assert(and_writes(b, m, pos, x, y, explicit) == Seq::<Write>::empty());
    } else if x >= m.width {
        assert(x == m.width);
        assert(y * m.width + x == (y + 1) * m.width + 0) by (nonlinear_arith)
            requires x == m.width;
        assert(and_writes(b, m, pos, x, y, explicit) == and_writes(b, m, pos, 0, y + 1, explicit));
        lemma_and_writes_prefix(b, m, pos, original, explicit, 0, y + 1);
    } else {
        let done = y * m.width + x;
        let before = ma_prefix(b, m, pos, original, explicit, done);
        let after = ma_prefix(b, m, pos, original, explicit, done + 1);
        let off = 4 * ((m.height - 1 - y) * m.width + x) + 3;
        let cell = cell_of(m, x, y);
        let rest = and_writes(b, m, pos, x + 1, y, explicit);
        lemma_ma_prefix_step(b, m, pos, original, explicit, x, y);
        lemma_ma_source_cell(m, x, y);
        assert(cell == (m.height - 1 - y) * m.width + x) by {
            reveal(cell_of);
            reveal(orient_cell);
            reveal(map_row_of);
        }
        assert(off == 4 * cell + 3);
        assert(after == before.update(off, ma_byte(b, m, pos, original, explicit, off)));
        let bit = and_bit(b, pos, m.width, x, y);
        assert(ma_byte(b, m, pos, original, explicit, off) == (if bit {
            0u8
        } else if explicit {
            255u8
        } else {
            original[off]
        })) by {
            reveal(ma_byte);
            assert(off % 4 == 3);
            assert((off / 4) % m.width == x);
            assert(m.height - 1 - (off / 4) / m.width == y);
        }
        // The inductive hypothesis for `(x+1, y)`.
        lemma_and_writes_prefix(b, m, pos, original, explicit, x + 1, y);
        assert(y * m.width + (x + 1) == done + 1);
        if bit {
            let w = Write { cell: cell, px: seq![0u8, 0u8, 0u8, 0u8] };
            assert(and_writes(b, m, pos, x, y, explicit) == seq![w] + rest);
            assert((seq![w] + rest)[0] == w);
            assert((seq![w] + rest).drop_first() == rest);
            assert(alpha_one(before, w) == after) by {
                reveal(alpha_one);
            }
        } else if explicit {
            let w = Write { cell: cell, px: seq![0u8, 0u8, 0u8, 255u8] };
            assert(and_writes(b, m, pos, x, y, explicit) == seq![w] + rest);
            assert((seq![w] + rest)[0] == w);
            assert((seq![w] + rest).drop_first() == rest);
            assert(alpha_one(before, w) == after) by {
                reveal(alpha_one);
            }
        } else {
            assert(and_writes(b, m, pos, x, y, explicit) == Seq::<Write>::empty() + rest);
            assert(Seq::<Write>::empty() + rest == rest);
            // No write: the pass through `done` pixels has not touched `off`
            // (its source pixel is `done` itself), so the update is a no-op.
            assert(0 <= cell < m.n) by (nonlinear_arith)
                requires cell == (m.height - 1 - y) * m.width + x,
                    0 <= x < m.width, 0 <= y < m.height, m.width >= 1,
                    m.n == m.width * m.height;
            assert(0 <= off < original.len()) by (nonlinear_arith)
                requires off == 4 * cell + 3, 0 <= cell < m.n, original.len() == 4 * m.n;
            assert(before.len() == original.len()) by {
                reveal(ma_prefix);
            }
            assert(before[off] == original[off]) by {
                reveal(ma_prefix);
                assert(ma_source(m, off) == done);
            }
            vstd::seq::lemma_seq_update_same(before, off, original[off]);
            assert(after =~= before);
        }
    }
}

/// `ma_mask` (write-list form) agrees with `ma` (byte form) on T8's domain:
/// the same two API errors in the same order, and on success the AND write
/// list applied to the XOR alpha bytes is `ma`'s image whenever the fallback is
/// active; when some explicit alpha byte is nonzero `ma` keeps the XOR plane.
pub proof fn lemma_ma_mask_connect(b: Seq<u8>, m: MHRecord, pos: int, original: Seq<u8>)
    requires
        ma_domain(b, m, pos, original),
    ensures
        match ma(b, m, pos, original) {
            Err(e) => ma_mask(b, m, pos, pos - m.pixel_start, explicit_alpha(m)) == Err::<Seq<Write>, MaErr>(e),
            Ok((end, image)) => {
                &&& end == pos + and_bytes(m.width, m.height)
                &&& ma_mask(b, m, pos, pos - m.pixel_start, explicit_alpha(m))
                    == Ok::<Seq<Write>, MaErr>(and_writes(b, m, pos, 0, 0, explicit_alpha(m)))
                &&& (explicit_alpha(m) && xor_alpha_present(original, m.n)) ==> image == original
                &&& !(explicit_alpha(m) && xor_alpha_present(original, m.n)) ==> image
                    == alpha_replay(original, and_writes(b, m, pos, 0, 0, explicit_alpha(m)))
            },
        },
{
    reveal(ma);
    reveal(ma_exact);
    reveal(ma_mask);
    reveal(ma_domain);
    let explicit = explicit_alpha(m);
    let a = and_bytes(m.width, m.height);
    if !image_size_ok(m.declared_image, pos - m.pixel_start, a, true) {
    } else if pos + a > b.len() {
    } else {
        if explicit && xor_alpha_present(original, m.n) {
            lemma_ma_explicit_alpha_keep(b, m, pos, original);
        } else {
            lemma_ma_prefix_ends(b, m, pos, original, explicit);
            lemma_and_writes_prefix(b, m, pos, original, explicit, 0, 0);
            assert(0 * m.width + 0 == 0);
        }
    }
}

// ---------------------------------------------------------------------------
// MP · the assembly (T6/T7/T8 composition; decode = MP is M4b)
// ---------------------------------------------------------------------------

/// MP's error wrapper.
pub enum MpErr {
    Api(Error),
    Outside,
}

/// R12: whether the `biSizeImage` rule accepts `x_len` for this record's mode.
pub open spec fn mp_image_size_ok(m: MHRecord, x_len: int) -> bool {
    let ico = is_ico(m.mode);
    let a = if ico {
        and_bytes(m.width, m.height)
    } else {
        0
    };
    image_size_ok(m.declared_image, x_len, a, ico)
}

/// T8/T7/R19: the mask phase on a complete raw XOR state. The R12 size check
/// precedes mask availability; the AND plane is then consumed. `TrailingBytes`
/// closes the BareDib slice and the exact-tail ICO mode `IcoEntryDib`, while
/// `BmpFile` ignores remaining bytes and `IcoEntryDibSlack` skips *only* the
/// final `end == b.len()` test after a complete AND mask (R19).
pub open spec fn mp_mask_phase(
    b: Seq<u8>,
    m: MHRecord,
    pos: int,
    x_len: int,
    out: Seq<u8>,
) -> Result<(int, Seq<u8>), MpErr> {
    let a = and_bytes(m.width, m.height);
    if !mp_image_size_ok(m, x_len) {
        Err(MpErr::Api(Error::InvalidImageSize))
    } else if is_ico(m.mode) {
        if pos + a > b.len() {
            Err(MpErr::Api(Error::TruncatedAndMask))
        } else {
            let end = pos + a;
            if exact_tail(m.mode) && end != b.len() {
                Err(MpErr::Api(Error::TrailingBytes))
            } else {
                // R12: the whole-image XOR-alpha/AND policy on the complete XOR
                // plane (the same `ma_output` MA returns).
                Ok((end, ma_output(b, m, pos, out, explicit_alpha(m))))
            }
        }
    } else if m.mode == Mode::BmpFile {
        Ok((pos, out))
    } else if pos != b.len() {
        Err(MpErr::Api(Error::TrailingBytes))
    } else {
        Ok((pos, out))
    }
}

/// R13/`MP`: for a raw/BITFIELDS record, the R12 phase and the exact output.
pub open spec fn mp_raw(b: Seq<u8>, m: MHRecord) -> Result<(Seq<u8>, int), MpErr> {
    match mr_raw_out(b, m, m.pixel_start) {
        Err((RawErr::Api(e), _pc, _xc, _yc)) => Err(MpErr::Api(e)),
        Err((RawErr::Outside, _pc, _xc, _yc)) => Err(MpErr::Outside),
        Ok((pc, out)) => match mp_mask_phase(b, m, pc, pc - m.pixel_start, out) {
            Ok((end, out2)) => Ok((out2, end)),
            Err(e) => Err(e),
        },
    }
}

/// R13/`MP`: for an RLE4/RLE8 record, the R12 phase and the exact output.
pub open spec fn mp_rle(b: Seq<u8>, m: MHRecord) -> Result<(Seq<u8>, int), MpErr> {
    match mr_out(b, m, m.pixel_start, 0, 0) {
        Err((RleErr::Api(e), _pc, _xc, _yc)) => Err(MpErr::Api(e)),
        Err((RleErr::Outside, _pc, _xc, _yc)) => Err(MpErr::Outside),
        Ok((pc, out)) => match mp_mask_phase(b, m, pc, pc - m.pixel_start, out) {
            Ok((end, out2)) => Ok((out2, end)),
            Err(e) => Err(e),
        },
    }
}

/// The pixel-buffer-to-`Ok` step shared by both paths.
pub open spec fn mp_decoded(
    m: MHRecord,
    r: Result<(Seq<u8>, int), MpErr>,
) -> Result<(int, int, Seq<u8>), Error> {
    match r {
        Ok((out, _end)) => Ok((m.width, m.height, out)),
        Err(MpErr::Api(e)) => Err(e),
        Err(MpErr::Outside) => Err(Error::TruncatedPixels),
    }
}

/// R13/`MP`: the total reference model — it calls `MH`, requires the palette to
/// fit (R7's `TruncatedPalette`), then performs the R8 fill, the R9 raw or
/// R10–R11 RLE transition, the R12 image-length/AND/suffix checks and returns
/// the exact `Ok(w, h, rgba)` or the exact first `Err(e)`. Never calls the
/// executable parser.
pub open spec fn mp(b: Seq<u8>, mode: Mode) -> Result<(int, int, Seq<u8>), Error> {
    match mh(b, mode) {
        Err(e) => Err(e),
        Ok(m) => {
            if m.palette_start + m.table_count * m.palette_stride > b.len() {
                Err(Error::TruncatedPalette)
            } else if m.compression == 0 || m.compression == 3 {
                mp_decoded(m, mp_raw(b, m))
            } else {
                mp_decoded(m, mp_rle(b, m))
            }
        },
    }
}

/// R13: `Accepted(b,mode)` means `MP(b,mode) = Ok(...)`.
pub open spec fn accepted(b: Seq<u8>, mode: Mode) -> bool {
    exists|w: int, h: int, p: Seq<u8>| #[trigger] mp(b, mode) == Ok((w, h, p))
}



/// T9/T10: any `MH` error is the exact `MP` error — the header phase's
/// precedence is never re-ordered by the pixel phase.
pub proof fn lemma_mp_mh_err(b: Seq<u8>, mode: Mode, e: Error)
    requires
        mh(b, mode) == Err(e),
    ensures
        mp(b, mode) == Err(e),
{
    reveal(mp);
}

/// The `Ok` record's R7 palette geometry: `palette_end` is exactly
/// `palette_start + table_count*palette_stride`. Read off the construction, so
/// the executable can relate its own `palette_end` field to `MP`'s fit test.
pub proof fn lemma_mh_palette_end(b: Seq<u8>, mode: Mode, m: MHRecord)
    requires
        mh(b, mode) == Ok(m),
    ensures
        m.palette_end == m.palette_start + m.table_count * m.palette_stride,
{
    reveal(mh);
    reveal(r2r7);
    reveal(r6r7);
    reveal(r6r7_result);
    reveal(r1_prefix);
}

/// `ma_output` keeps the input length: the ICO mask phase rewrites alpha bytes
/// in place.
pub proof fn lemma_ma_output_len(
    b: Seq<u8>, m: MHRecord, pos: int, original: Seq<u8>, explicit: bool,
)
    ensures
        ma_output(b, m, pos, original, explicit).len() == original.len(),
{
    reveal(ma_output);
}

/// `mp_mask_phase`'s `Ok` payload keeps the XOR plane's length in every mode:
/// `BmpFile`/`BareDib` return it unchanged, `IcoEntryDib` rewrites alpha bytes
/// in place through `ma_output`.
pub proof fn lemma_mp_mask_phase_len(
    b: Seq<u8>, m: MHRecord, pos: int, x_len: int, out: Seq<u8>, end: int, out2: Seq<u8>,
)
    requires
        mp_mask_phase(b, m, pos, x_len, out) == Ok((end, out2)),
    ensures
        out2.len() == out.len(),
{
    reveal(mp_mask_phase);
}

/// `mr_raw_out`'s `Ok` payload is the `4*m.n`-byte zero fill replayed.
pub proof fn lemma_mr_raw_out_len(b: Seq<u8>, m: MHRecord, pos: int, pc: int, out: Seq<u8>)
    requires
        mr_raw_out(b, m, pos) == Ok((pc, out)),
        m.n >= 0,
    ensures
        out.len() == 4 * m.n,
{
    reveal(mr_raw_out);
    reveal(zero_bytes);
    match raw_full(b, m, pos, 0, 0) {
        Ok((_pc, _xc, _yc, ws)) => {
            assert(out == bytes_replay(zero_bytes(4 * m.n), ws));
            crate::model::lemma_bytes_replay_len(zero_bytes(4 * m.n), ws);
            assert(zero_bytes(4 * m.n).len() == 4 * m.n) by {
                reveal(zero_bytes);
                broadcast use vstd::seq::group_seq_lemmas;
            }
        },
        Err(_) => {},
    }
}

/// `mr_out`'s `Ok` payload is the `4*m.n`-byte palette-zero fill replayed.
pub proof fn lemma_mr_out_len(b: Seq<u8>, m: MHRecord, pos: int, pc: int, out: Seq<u8>)
    requires
        mr_out(b, m, pos, 0, 0) == Ok((pc, out)),
        m.n >= 0,
    ensures
        out.len() == 4 * m.n,
{
    reveal(mr_out);
    match mr_full(b, m, pos, 0, 0) {
        Ok((_pc, _xc, _yc, ws)) => {
            assert(out == bytes_replay(pal_zero_bytes(b, m), ws));
            crate::model::lemma_pal_zero_bytes_len(b, m);
            crate::model::lemma_bytes_replay_len(pal_zero_bytes(b, m), ws);
        },
        Err(_) => {},
    }
}

/// T9/T10 (raw path, error): when the raw helper fails with `e`, `MP` fails with
/// the same `e` — `mp_raw` propagates the raw phase's exact first error.
pub proof fn lemma_mp_raw_err(b: Seq<u8>, mode: Mode, m: MHRecord, e: Error)
    requires
        mh(b, mode) == Ok(m),
        m.compression == 0 || m.compression == 3,
        mr_raw_out(b, m, m.pixel_start) is Err,
        (mr_raw_out(b, m, m.pixel_start)->Err_0).0 == RawErr::Api(e),
    ensures
        mp(b, mode) == Err(e),
{
    lemma_mh_palette_end(b, mode, m);
    reveal(mp);
    reveal(mp_raw);
    reveal(mp_decoded);
    reveal(mh);
    assert(m.palette_start + m.table_count * m.palette_stride <= b.len());
    let r = mr_raw_out(b, m, m.pixel_start);
    let pcerr = r->Err_0;
    assert(pcerr.0 is Api);
    assert(mp_raw(b, m) == Err(MpErr::Api(pcerr.0->Api_0)));
    assert(crate::model::mp_decoded(m, mp_raw(b, m)) == Err(pcerr.0->Api_0));
}

/// T9/T10 (RLE path, error): the RLE helper's exact first error is `MP`'s.
pub proof fn lemma_mp_rle_err(b: Seq<u8>, mode: Mode, m: MHRecord, e: Error)
    requires
        mh(b, mode) == Ok(m),
        m.compression == 1 || m.compression == 2,
        mr_out(b, m, m.pixel_start, 0, 0) is Err,
        (mr_out(b, m, m.pixel_start, 0, 0)->Err_0).0 == RleErr::Api(e),
    ensures
        mp(b, mode) == Err(e),
{
    lemma_mh_palette_end(b, mode, m);
    reveal(mp);
    reveal(mp_rle);
    reveal(mp_decoded);
    reveal(mh);
    assert(m.palette_start + m.table_count * m.palette_stride <= b.len());
    let r = mr_out(b, m, m.pixel_start, 0, 0);
    let pcerr = r->Err_0;
    assert(pcerr.0 is Api);
    assert(mp_rle(b, m) == Err(MpErr::Api(pcerr.0->Api_0)));
    assert(crate::model::mp_decoded(m, mp_rle(b, m)) == Err(pcerr.0->Api_0));
}

/// T9/T10 (mask path, error): the mask phase's exact error is `MP`'s, for the
/// raw dispatch (the concrete `decode_raw` returned `Ok(p)` with XOR buffer
/// `out0`, so `mp_raw` reaches the mask phase with the same state).
pub proof fn lemma_mp_mask_phase_err_raw(
    b: Seq<u8>, mode: Mode, m: MHRecord, p: int, out0: Seq<u8>, e: Error,
)
    requires
        mh(b, mode) == Ok(m),
        m.compression == 0 || m.compression == 3,
        mr_raw_out(b, m, m.pixel_start) == Ok((p, out0)),
        mp_mask_phase(b, m, p, p - m.pixel_start, out0) == Err(MpErr::Api(e)),
    ensures
        mp(b, mode) == Err(e),
{
    lemma_mh_palette_end(b, mode, m);
    reveal(mp);
    reveal(mp_raw);
    reveal(mp_decoded);
    reveal(mh);
    assert(m.palette_start + m.table_count * m.palette_stride <= b.len());
    assert(mp_raw(b, m) == Err(MpErr::Api(e)));
}

/// T9/T10 (mask path, error): the same for the RLE dispatch.
pub proof fn lemma_mp_mask_phase_err_rle(
    b: Seq<u8>, mode: Mode, m: MHRecord, p: int, out0: Seq<u8>, e: Error,
)
    requires
        mh(b, mode) == Ok(m),
        m.compression == 1 || m.compression == 2,
        mr_out(b, m, m.pixel_start, 0, 0) == Ok((p, out0)),
        mp_mask_phase(b, m, p, p - m.pixel_start, out0) == Err(MpErr::Api(e)),
    ensures
        mp(b, mode) == Err(e),
{
    lemma_mh_palette_end(b, mode, m);
    reveal(mp);
    reveal(mp_rle);
    reveal(mp_decoded);
    reveal(mh);
    assert(m.palette_start + m.table_count * m.palette_stride <= b.len());
    assert(mp_rle(b, m) == Err(MpErr::Api(e)));
}

/// T9/T8 bridge: `mp_mask_phase` returns `Ok` exactly on `ma`'s success branch
/// with `ma_output`'s image, so the executable's `finish_mask` outcome and
/// `MP`'s mask phase agree byte for byte. Stated in the mode-dispatch shape of
/// `mp_mask_phase` so the executable can discharge it branch by branch. R19:
/// holds for both ICO modes — the slack mode returns `Ok` on `end <= b.len()`
/// (i.e. `end < b.len()` is allowed), the exact-tail mode only when the AND
/// plane closes the slice.
pub proof fn lemma_mp_mask_phase_ico_ok(
    b: Seq<u8>,
    m: MHRecord,
    pos: int,
    out0: Seq<u8>,
    out1: Seq<u8>,
)
    requires
        is_ico(m.mode),
        image_size_ok(m.declared_image, pos - m.pixel_start, and_bytes(m.width, m.height), true),
        pos + and_bytes(m.width, m.height) <= b.len(),
        exact_tail(m.mode) ==> pos + and_bytes(m.width, m.height) == b.len(),
        out1 == ma_output(b, m, pos, out0, explicit_alpha(m)),
    ensures
        mp_mask_phase(b, m, pos, pos - m.pixel_start, out0)
            == Ok((pos + and_bytes(m.width, m.height), out1)),
{
    reveal(is_ico);
    reveal(exact_tail);
    reveal(mp_mask_phase);
    reveal(mp_image_size_ok);
}


/// T9/T8 bridge: `mp_mask_phase`'s R12 failure is the same API error
/// `finish_mask` reports, in every mode.
pub proof fn lemma_mp_mask_phase_size_err(
    b: Seq<u8>,
    m: MHRecord,
    pos: int,
    out0: Seq<u8>,
)
    requires
        !mp_image_size_ok(m, pos - m.pixel_start),
    ensures
        mp_mask_phase(b, m, pos, pos - m.pixel_start, out0)
            == Err(MpErr::Api(Error::InvalidImageSize)),
{
    reveal(mp_mask_phase);
}

/// T13/T9: an `Ok` `MP` result always has the exact `4*w*h` output length and
/// the visibility bounds, restated from the header record's own bounds. The
/// public success contract follows by composing this with `decode = MP`.
pub proof fn lemma_mp_ok_shape(b: Seq<u8>, mode: Mode, w: int, h: int, p: Seq<u8>)
    requires
        mp(b, mode) == Ok((w, h, p)),
    ensures
        1 <= w <= max_dimension(),
        1 <= h <= max_dimension(),
        p.len() == 4 * (w * h),
        4 * (w * h) <= max_output(),
{
    reveal(mp);
    let m = choose|m: MHRecord| mh(b, mode) == Ok(m);
    assert(mh(b, mode) is Ok);
    assert(m.width == w);
    assert(m.height == h);
    lemma_mh_ok_bounds(b, mode, m);
    assert(m.n == w * h);
    assert(4 * (w * h) <= max_output());
    assert(1 <= w <= max_dimension());
    assert(1 <= h <= max_dimension());
    reveal(mp_decoded);
    reveal(mp_raw);
    reveal(mp_rle);
    reveal(mp_mask_phase);
    if m.compression == 0 || m.compression == 3 {
        assert(mp_decoded(m, mp_raw(b, m)) == Ok((w, h, p)));
        match mp_raw(b, m) {
            Ok((out, _end)) => {
                assert(p == out);
                match mr_raw_out(b, m, m.pixel_start) {
                    Ok((pc, out0)) => {
                        lemma_mr_raw_out_len(b, m, m.pixel_start, pc, out0);
                        assert(out0.len() == 4 * m.n);
                        match mp_mask_phase(b, m, pc, pc - m.pixel_start, out0) {
                            Ok((_e2, out2)) => {
                                assert(out == out2);
                                lemma_mp_mask_phase_len(
                                    b, m, pc, pc - m.pixel_start, out0, _e2, out2,
                                );
                                assert(p.len() == out0.len());
                            },
                            Err(_) => {},
                        }
                    },
                    Err(_) => {},
                }
            },
            Err(_) => {},
        }
    } else {
        assert(mp_decoded(m, mp_rle(b, m)) == Ok((w, h, p)));
        match mp_rle(b, m) {
            Ok((out, _end)) => {
                assert(p == out);
                match mr_out(b, m, m.pixel_start, 0, 0) {
                    Ok((pc, out0)) => {
                        lemma_mr_out_len(b, m, m.pixel_start, pc, out0);
                        assert(out0.len() == 4 * m.n);
                        match mp_mask_phase(b, m, pc, pc - m.pixel_start, out0) {
                            Ok((_e2, out2)) => {
                                assert(out == out2);
                                lemma_mp_mask_phase_len(
                                    b, m, pc, pc - m.pixel_start, out0, _e2, out2,
                                );
                                assert(p.len() == out0.len());
                            },
                            Err(_) => {},
                        }
                    },
                    Err(_) => {},
                }
            },
            Err(_) => {},
        }
    }
    assert(p.len() == 4 * (w * h));
}

/// L1: every input longer than `MAX_INPUT` is `FileTooLarge` in every mode. The
/// cap is tested first (R1), before any header byte is read, so the conclusion
/// holds for arbitrary oversized byte sequences — in particular the invalid
/// `BM` header of `precedence-too-large-bmp-bad-magic` need not be expanded.
pub proof fn lemma_mp_l1(b: Seq<u8>, mode: Mode)
    requires
        b.len() > max_input(),
    ensures
        mp(b, mode) == Err(Error::FileTooLarge),
{
    reveal(max_input);
    reveal(mh);
    assert(mh(b, mode) == Err(Error::FileTooLarge));
    lemma_mp_mh_err(b, mode, Error::FileTooLarge);
}

/// L2: the all-zero input of exactly `MAX_INPUT` bytes in `BareDib` is
/// `UnsupportedHeaderSize` — the length test is `>` and the DIB size word is
/// zero. This is the boundary that kills a `>=` input guard.
pub proof fn lemma_mp_l2()
    ensures
        mp(zero_bytes(max_input()), Mode::BareDib) == Err(Error::UnsupportedHeaderSize),
{
    let b = zero_bytes(max_input());
    reveal(zero_bytes);
    assert(b[0] == 0u8);
    assert(b[1] == 0u8);
    assert(b[2] == 0u8);
    assert(b[3] == 0u8);
    reveal(le32);
    assert(le32(b, 0) == 0);
    reveal(hsize_ok);
    assert(!hsize_ok(0));
    reveal(max_input);
    reveal(mh);
    reveal(r2r7);
    reveal(r2r5);
    assert(mh(b, Mode::BareDib) == Err(Error::UnsupportedHeaderSize));
    lemma_mp_mh_err(b, Mode::BareDib, Error::UnsupportedHeaderSize);
}


// ---------------------------------------------------------------------------
// M1.1a: R19. The new mode shares every ICO rule; the models for the three old
// modes are provably unchanged, and the only behavioural difference is at the
// final cursor-equality check (the slack mode skips it).
// ---------------------------------------------------------------------------

/// The 1.1 models mention the mode in exactly two predicates — `is_ico` (shared
/// ICO handling) and `exact_tail` (the R12 final cursor test) — plus the direct
/// `mode == Mode::BmpFile` test for the BMP file-header prefix. On the three 1.0
/// modes those reduce *definitionally* to the 1.0 tests:
///
/// - `is_ico(mode) == (mode == IcoEntryDib)`: the three 1.0 modes were `BmpFile`,
///   `BareDib`, `IcoEntryDib`, and `is_ico` adds only `IcoEntryDibSlack`.
/// - `exact_tail(mode) == (mode != BmpFile)`: in 1.0 the modes that rejected a
///   trailing byte were `BareDib` and `IcoEntryDib`; `BmpFile` ignored a suffix
///   and the new slack mode ignores post-AND bytes.
///
/// Because every clause body in `MH`/`MA`/`MR`/`Conv`/`MP` mentions the mode
/// only through these two predicates (or the `== BmpFile` test), each on the old
/// modes reduces to its 1.0 self, so the 1.1 models are the 1.0 models there.
/// The executable counterpart is the 214 unchanged 1.0 vectors and the old-mode
/// proofs, re-checked by `cargo test` and the verifier.
pub proof fn old_modes_unchanged(b: Seq<u8>, mode: Mode) -> (r: bool)
    requires
        old_mode(mode),
    ensures
        r == true,
        is_ico(mode) == (mode == Mode::IcoEntryDib),
        exact_tail(mode) == (mode != Mode::BmpFile),
{
    reveal(old_mode);
    reveal(is_ico);
    reveal(exact_tail);
    reveal(mh);
    match mode {
        Mode::BmpFile => {
            assert(is_ico(mode) == (mode == Mode::IcoEntryDib));
            assert(exact_tail(mode) == (mode != Mode::BmpFile));
        },
        Mode::BareDib => {
            assert(is_ico(mode) == (mode == Mode::IcoEntryDib));
            assert(exact_tail(mode) == (mode != Mode::BmpFile));
        },
        Mode::IcoEntryDib => {
            assert(is_ico(mode) == (mode == Mode::IcoEntryDib));
            assert(exact_tail(mode) == (mode != Mode::BmpFile));
        },
        Mode::IcoEntryDibSlack => { assert(false); },
    }
    true
}

/// R19: `r2r5` does not distinguish the two ICO modes. Its body branches on
/// `is_ico`, `visible_height_of` and `mode == BmpFile`, all of which agree
/// between `IcoEntryDib` and `IcoEntryDibSlack`, so on the same input the raw
/// header fields it returns are identical.
pub proof fn lemma_r2r5_ico_eq(b: Seq<u8>, s: int)
    ensures
        r2r5(b, Mode::IcoEntryDib, s) == r2r5(b, Mode::IcoEntryDibSlack, s),
{
    reveal(is_ico);
    reveal(visible_height_of);
    reveal(r2r5);
}

/// R19: `r6r7` agrees on the two ICO modes up to the record's `mode` field. The
/// R6/R7 guards (external-mask availability, `masks_ok`, the BMP offset test and
/// the palette-availability test) do not mention the mode, and the record
/// `r6r7_result` stores the mode as a field; hence the error branch is identical
/// and the `Ok` branch is `with_mode(..., IcoEntryDibSlack)`.
pub proof fn lemma_r6r7_ico_eq(b: Seq<u8>, s: int, file_offset: int, is_bmp: bool, rf: RawFields)
    ensures
        r6r7(b, Mode::IcoEntryDibSlack, s, file_offset, is_bmp, rf)
            == (match r6r7(b, Mode::IcoEntryDib, s, file_offset, is_bmp, rf) {
                Err(e) => Err(e),
                Ok(m) => Ok(with_mode(m, Mode::IcoEntryDibSlack)),
            }),
{
    reveal(is_ico);
    reveal(with_mode);
    reveal(r6r7);
    reveal(r6r7_result);
}

/// `slack_mode_same_prefix` (R19): the header record `MH` returns for
/// `IcoEntryDibSlack` is the `IcoEntryDib` record with only the `mode` field
/// changed, and a header error is the same error. This is `MH`'s phase
/// composition (`r2r7` = `r2r5` then `r6r7`) plus the two ICO-agreement lemmas,
/// with the identical R1 length test.
pub proof fn slack_mode_same_prefix(b: Seq<u8>, m: MHRecord)
    requires
        mh(b, Mode::IcoEntryDib) == Ok(m),
    ensures
        mh(b, Mode::IcoEntryDibSlack) == Ok(with_mode(m, Mode::IcoEntryDibSlack)),
{
    reveal(mh);
    reveal(r2r7);
    reveal(r2r5);
    reveal(r6r7);
    reveal(r6r7_result);
    reveal(is_ico);
    reveal(with_mode);
    reveal(visible_height_of);
    reveal(r1_prefix);
    if b.len() > max_input() {
        assert(false);
    }
    lemma_r2r5_ico_eq(b, 0);
    match r2r5(b, Mode::IcoEntryDib, 0) {
        Err(_) => { assert(false); },
        Ok(rf) => {
            assert(r2r5(b, Mode::IcoEntryDibSlack, 0) == Ok(rf));
            lemma_r6r7_ico_eq(b, 0, 0, false, rf);
            match r6r7(b, Mode::IcoEntryDib, 0, 0, false, rf) {
                Err(_) => { assert(false); },
                Ok(mm) => {
                    assert(mm == m);
                    assert(r6r7(b, Mode::IcoEntryDibSlack, 0, 0, false, rf)
                        == Ok(with_mode(mm, Mode::IcoEntryDibSlack)));
                    assert(mh(b, Mode::IcoEntryDibSlack)
                        == Ok(with_mode(m, Mode::IcoEntryDibSlack)));
                },
            }
        },
    }
}

/// `slack_mode_suffix_result` (R19), at the mask phase: `mp_mask_phase(b,Slack)`
/// and `mp_mask_phase(b,IcoEntryDib)` differ **only** when the latter returns
/// `TrailingBytes`, and then the former returns the completed image
/// `Ok((pos + and_bytes, ma_output))`; every other result — the success payload
/// and every other error (`InvalidImageSize`, `TruncatedAndMask`) — is
/// identical. The two modes share the R12 image-size test and the AND
/// availability test (both ask about the mode only through `is_ico`), so the
/// whole difference sits in the final `exact_tail` cursor test alone.
pub proof fn slack_mode_suffix_phase(
    b: Seq<u8>,
    m: MHRecord,
    pos: int,
    x_len: int,
    out0: Seq<u8>,
    r: Result<(int, Seq<u8>), MpErr>,
)
    requires
        m.mode == Mode::IcoEntryDib,
        mp_mask_phase(b, m, pos, x_len, out0) == r,
    ensures
        match r {
            Err(MpErr::Api(Error::TrailingBytes)) => {
                let a = and_bytes(m.width, m.height);
                let ms = with_mode(m, Mode::IcoEntryDibSlack);
                &&& pos + a <= b.len()
                &&& mp_image_size_ok(m, x_len)
                &&& mp_mask_phase(b, ms, pos, x_len, out0)
                    == Ok((pos + a, ma_output(b, ms, pos, out0, explicit_alpha(ms))))
            },
            Err(other) => mp_mask_phase(b, with_mode(m, Mode::IcoEntryDibSlack), pos, x_len, out0)
                == Err(other),
            Ok(ok) => mp_mask_phase(b, with_mode(m, Mode::IcoEntryDibSlack), pos, x_len, out0)
                == Ok(ok),
        },
{
    let ms = with_mode(m, Mode::IcoEntryDibSlack);
    let a = and_bytes(m.width, m.height);
    reveal(is_ico);
    reveal(exact_tail);
    reveal(mp_mask_phase);
    reveal(mp_image_size_ok);
    reveal(with_mode);
    reveal(and_bytes);
    reveal(and_stride);
    reveal(explicit_alpha);
    reveal(ma_output);
    assert(ms.mode == Mode::IcoEntryDibSlack);
    assert(ms.width == m.width && ms.height == m.height);
    assert(ms.declared_image == m.declared_image);
    assert(ms.pixel_start == m.pixel_start);
    assert(is_ico(ms.mode) && !exact_tail(ms.mode));
    assert(and_bytes(ms.width, ms.height) == a);
    assert(mp_image_size_ok(ms, x_len) == mp_image_size_ok(m, x_len));
    // Split on the two phase guards; each branch is a definitional unfolding of
    // both `mp_mask_phase` bodies.
    if !mp_image_size_ok(m, x_len) {
        assert(r == Err(MpErr::Api(Error::InvalidImageSize)));
        assert(mp_mask_phase(b, ms, pos, x_len, out0)
            == Err(MpErr::Api(Error::InvalidImageSize)));
    } else if pos + a > b.len() {
        assert(r == Err(MpErr::Api(Error::TruncatedAndMask)));
        assert(mp_mask_phase(b, ms, pos, x_len, out0)
            == Err(MpErr::Api(Error::TruncatedAndMask)));
    } else if pos + a == b.len() {
        assert(r == Ok((pos + a, ma_output(b, m, pos, out0, explicit_alpha(m)))));
        assert(mp_mask_phase(b, ms, pos, x_len, out0)
            == Ok((pos + a, ma_output(b, ms, pos, out0, explicit_alpha(ms)))));
        assert(ma_output(b, ms, pos, out0, explicit_alpha(ms))
            == ma_output(b, m, pos, out0, explicit_alpha(m)));
    } else {
        assert(r == Err(MpErr::Api(Error::TrailingBytes)));
        assert(mp_mask_phase(b, ms, pos, x_len, out0)
            == Ok((pos + a, ma_output(b, ms, pos, out0, explicit_alpha(ms)))));
    }
}

// ---------------------------------------------------------------------------
// M1.1a: R19 pixel-phase mode congruence. The two ICO modes share every field
// of the header record but `mode`; the raw/RLE/mask phases read the mode only
// through `is_ico` (or the shared fields), so they produce identical writes and
// outputs. These lemmas lift the mask-phase `slack_mode_suffix_result` to the
// whole `MP`.
// ---------------------------------------------------------------------------

/// Two ICO-mode records that agree on every field but `mode`: `ms` is `m` with
/// only its `mode` field replaced, and both modes are ICO.
pub open spec fn ico_pair(m: MHRecord, ms: MHRecord) -> bool {
    &&& m.mode == Mode::IcoEntryDib
    &&& ms.mode == Mode::IcoEntryDibSlack
    &&& with_mode(m, ms.mode) == ms
}

/// The shared fields of an `ico_pair`, extracted for the congruence lemmas.
pub proof fn lemma_ico_pair_fields(m: MHRecord, ms: MHRecord)
    requires
        ico_pair(m, ms),
    ensures
        m.s == ms.s, m.hsize == ms.hsize, m.width == ms.width, m.height == ms.height,
        m.stored_height == ms.stored_height, m.planes == ms.planes, m.depth == ms.depth,
        m.compression == ms.compression, m.declared_image == ms.declared_image,
        m.color_used == ms.color_used, m.count == ms.count, m.table_count == ms.table_count,
        m.palette_stride == ms.palette_stride, m.palette_start == ms.palette_start,
        m.palette_end == ms.palette_end, m.pixel_start == ms.pixel_start,
        m.mask_r == ms.mask_r, m.mask_g == ms.mask_g, m.mask_b == ms.mask_b, m.mask_a == ms.mask_a,
        m.has_masks == ms.has_masks, m.core == ms.core, m.is_bmp == ms.is_bmp,
        m.top_down == ms.top_down, m.n == ms.n, m.stride == ms.stride,
        is_ico(m.mode), is_ico(ms.mode), exact_tail(m.mode), !exact_tail(ms.mode),
{
    reveal(ico_pair);
    reveal(with_mode);
    reveal(is_ico);
    reveal(exact_tail);
}

/// `conv`/`conv_word` congruence on an `ico_pair`: `is_ico` is the only mode
/// predicate they read, and the shared fields fix the rest.
pub proof fn lemma_conv_ico_pair(b: Seq<u8>, m: MHRecord, ms: MHRecord, v: int)
    requires
        ico_pair(m, ms),
    ensures
        conv(b, m, v) == conv(b, ms, v),
        conv_word(m, v) == conv_word(ms, v),
{
    lemma_ico_pair_fields(m, ms);
    reveal(ico_pair);
    reveal(conv);
    reveal(conv_word);
    reveal(is_ico);
    reveal(bitfield_seq);
    reveal(rgb555_seq);
    reveal(rgb_seq);
}

pub proof fn lemma_raw_conv_ico_pair(b: Seq<u8>, m: MHRecord, ms: MHRecord, base: int, x: int)
    requires
        ico_pair(m, ms),
    ensures
        raw_conv(b, m, base, x) == raw_conv(b, ms, base, x),
        raw_index_ok(b, m, base, x) == raw_index_ok(b, ms, base, x),
{
    lemma_ico_pair_fields(m, ms);
    reveal(ico_pair);
    reveal(raw_conv);
    reveal(conv_rgb);
    reveal(conv_word);
    reveal(is_ico);
    reveal(raw_index_ok);
    reveal(pal_entry);
}

pub proof fn lemma_cell_of_ico_pair(m: MHRecord, ms: MHRecord, x: int, y: int)
    requires
        ico_pair(m, ms),
    ensures
        cell_of(m, x, y) == cell_of(ms, x, y),
{
    lemma_ico_pair_fields(m, ms);
    reveal(ico_pair);
    reveal(cell_of);
    reveal(orient_cell);
    reveal(map_row_of);
}

pub proof fn lemma_index_ok_ico_pair(m: MHRecord, ms: MHRecord, v: int)
    requires
        ico_pair(m, ms),
    ensures
        index_ok(m, v) == index_ok(ms, v),
{
    lemma_ico_pair_fields(m, ms);
    reveal(ico_pair);
    reveal(index_ok);
}

pub proof fn lemma_pal_zero_ico_pair(b: Seq<u8>, m: MHRecord, ms: MHRecord)
    requires
        ico_pair(m, ms),
    ensures
        pal_zero_bytes(b, m) == pal_zero_bytes(b, ms),
{
    lemma_ico_pair_fields(m, ms);
    reveal(ico_pair);
    reveal(pal_zero_bytes);
    reveal(pal_entry);
}

/// The AND write list congruence: `and_bit` reads only width/height/pos and
/// `cell_of` only width/height/top_down.
pub proof fn lemma_and_writes_ico_pair(
    b: Seq<u8>, m: MHRecord, ms: MHRecord, pos: int, x: int, y: int, hxa: bool,
)
    requires
        ico_pair(m, ms),
    ensures
        and_writes(b, m, pos, x, y, hxa) == and_writes(b, ms, pos, x, y, hxa),
    decreases (m.height - y) as nat, (m.width - x) as nat,
{
    lemma_ico_pair_fields(m, ms);
    reveal(ico_pair);
    reveal(and_writes);
    reveal(and_bit);
    reveal(and_stride);
    if y >= m.height {
    } else if x >= m.width {
        lemma_and_writes_ico_pair(b, m, ms, pos, 0, y + 1, hxa);
    } else {
        lemma_cell_of_ico_pair(m, ms, x, y);
        lemma_and_writes_ico_pair(b, m, ms, pos, x + 1, y, hxa);
    }
}

pub proof fn lemma_run_writes_ico_pair(
    b: Seq<u8>, m: MHRecord, ms: MHRecord, x: int, y: int, code: int, j: int, k: int,
)
    requires
        ico_pair(m, ms),
    ensures
        run_writes(b, m, x, y, code, j, k) == run_writes(b, ms, x, y, code, j, k),
    decreases (k - j) as nat,
{
    lemma_ico_pair_fields(m, ms);
    reveal(ico_pair);
    reveal(run_writes);
    if j >= k {
    } else {
        lemma_cell_of_ico_pair(m, ms, x + j, y);
        lemma_conv_ico_pair(b, m, ms, run_index(code, m.depth, j));
        lemma_run_writes_ico_pair(b, m, ms, x, y, code, j + 1, k);
    }
}

pub proof fn lemma_abs_writes_ico_pair(
    b: Seq<u8>, m: MHRecord, ms: MHRecord, x: int, y: int, pay: int, j: int, k: int,
)
    requires
        ico_pair(m, ms),
    ensures
        abs_writes(b, m, x, y, pay, j, k) == abs_writes(b, ms, x, y, pay, j, k),
    decreases (k - j) as nat,
{
    lemma_ico_pair_fields(m, ms);
    reveal(ico_pair);
    reveal(abs_writes);
    if j >= k {
    } else {
        lemma_cell_of_ico_pair(m, ms, x + j, y);
        lemma_conv_ico_pair(b, m, ms, abs_index(b, m.depth, pay, j));
        lemma_abs_writes_ico_pair(b, m, ms, x, y, pay, j + 1, k);
    }
}

/// `run_good`/`abs_good` congruence: they read `index_ok` (mode-free up to
/// depth/count) and the slice only.
pub proof fn lemma_run_good_ico_pair(m: MHRecord, ms: MHRecord, code: int, q: int, j: int)
    requires
        ico_pair(m, ms),
    ensures
        run_good(m, code, q, j) == run_good(ms, code, q, j),
    decreases (q - j) as nat,
{
    lemma_ico_pair_fields(m, ms);
    reveal(ico_pair);
    reveal(run_good);
    reveal(ico_pair);
    if j >= q {
    } else {
        lemma_index_ok_ico_pair(m, ms, run_index(code, m.depth, j));
        assert(m.depth == ms.depth);
        lemma_run_good_ico_pair(m, ms, code, q, j + 1);
    }
}

pub proof fn lemma_abs_good_ico_pair(b: Seq<u8>, m: MHRecord, ms: MHRecord, pay: int, q: int, j: int)
    requires
        ico_pair(m, ms),
    ensures
        abs_good(b, m, pay, q, j) == abs_good(b, ms, pay, q, j),
    decreases (q - j) as nat,
{
    lemma_ico_pair_fields(m, ms);
    reveal(ico_pair);
    reveal(abs_good);
    if j >= q {
    } else {
        lemma_index_ok_ico_pair(m, ms, abs_index(b, m.depth, pay, j));
        lemma_abs_good_ico_pair(b, m, ms, pay, q, j + 1);
    }
}

/// `raw_full` congruence: the recursion structure depends only on `stride`,
/// `width`, `height` (equal across the pair), and each leaf's write uses
/// `raw_conv`/`raw_index_ok`.
pub proof fn lemma_raw_full_ico_pair(
    b: Seq<u8>, m: MHRecord, ms: MHRecord, pos: int, x: int, y: int,
)
    requires
        ico_pair(m, ms),
    ensures
        raw_full(b, m, pos, x, y) == raw_full(b, ms, pos, x, y),
    decreases (m.height - y) as nat, (m.width - x) as nat,
{
    lemma_ico_pair_fields(m, ms);
    reveal(ico_pair);
    reveal(raw_full);
    if y >= m.height {
    } else if x >= m.width {
        lemma_raw_full_ico_pair(b, m, ms, pos + m.stride, 0, y + 1);
    } else if x == 0 && pos + m.stride > b.len() {
    } else {
        lemma_raw_conv_ico_pair(b, m, ms, pos, x);
        if !raw_index_ok(b, m, pos, x) {
        } else {
            lemma_cell_of_ico_pair(m, ms, x, y);
            lemma_raw_full_ico_pair(b, m, ms, pos, x + 1, y);
        }
    }
}

/// `mr_full` congruence (the RLE machine): identical recursion structure and
/// leaves, so the two ICO records give the same result.
pub proof fn lemma_mr_full_ico_pair(b: Seq<u8>, m: MHRecord, ms: MHRecord, p: int, x: int, y: int)
    requires
        ico_pair(m, ms),
    ensures
        mr_full(b, m, p, x, y) == mr_full(b, ms, p, x, y),
    decreases (b.len() - p) as nat,
{
    lemma_ico_pair_fields(m, ms);
    reveal(ico_pair);
    reveal(mr_full);
    if p + 2 > b.len() {
    } else {
        let quantity = b[p] as int;
        let code = b[p + 1] as int;
        let p2 = p + 2;
        if quantity != 0 {
            if y >= m.height || x + quantity > m.width {
            } else {
                lemma_run_good_ico_pair(m, ms, code, quantity, 0);
                let g = run_good(m, code, quantity, 0);
                if g < quantity {
                    lemma_run_writes_ico_pair(b, m, ms, x, y, code, 0, g);
                } else {
                    lemma_run_writes_ico_pair(b, m, ms, x, y, code, 0, quantity);
                    lemma_mr_full_ico_pair(b, m, ms, p2, x + quantity, y);
                }
            }
        } else if code == 1 {
        } else if code == 0 {
            if y >= m.height {
            } else {
                lemma_mr_full_ico_pair(b, m, ms, p2, 0, y + 1);
            }
        } else if code == 2 {
            if p2 + 2 > b.len() {
            } else {
                let dx = b[p2] as int;
                let dy = b[p2 + 1] as int;
                let p4 = p2 + 2;
                if y + dy >= m.height || x + dx >= m.width {
                } else {
                    lemma_mr_full_ico_pair(b, m, ms, p4, x + dx, y + dy);
                }
            }
        } else {
            let q = code;
            if y >= m.height || x + q > m.width {
            } else {
                let payload = rle_payload(q, m.depth);
                let padded = rle_padded(q, m.depth);
                if p2 + padded > b.len() {
                } else {
                    lemma_abs_good_ico_pair(b, m, ms, p2, q, 0);
                    let g = abs_good(b, m, p2, q, 0);
                    if g < q {
                        lemma_abs_writes_ico_pair(b, m, ms, x, y, p2, 0, g);
                    } else {
                        lemma_abs_writes_ico_pair(b, m, ms, x, y, p2, 0, q);
                        lemma_mr_full_ico_pair(b, m, ms, p2 + padded, x + q, y);
                    }
                }
            }
        }
    }
}

pub proof fn lemma_mr_raw_out_ico_pair(b: Seq<u8>, m: MHRecord, ms: MHRecord, pos0: int)
    requires
        ico_pair(m, ms),
    ensures
        mr_raw_out(b, m, pos0) == mr_raw_out(b, ms, pos0),
{
    lemma_ico_pair_fields(m, ms);
    reveal(ico_pair);
    reveal(mr_raw_out);
    reveal(bytes_replay);
    reveal(zero_bytes);
    lemma_raw_full_ico_pair(b, m, ms, pos0, 0, 0);
}

pub proof fn lemma_mr_out_ico_pair(b: Seq<u8>, m: MHRecord, ms: MHRecord)
    requires
        ico_pair(m, ms),
    ensures
        mr_out(b, m, m.pixel_start, 0, 0) == mr_out(b, ms, ms.pixel_start, 0, 0),
{
    lemma_ico_pair_fields(m, ms);
    reveal(ico_pair);
    reveal(mr_out);
    lemma_mr_full_ico_pair(b, m, ms, m.pixel_start, 0, 0);
    lemma_pal_zero_ico_pair(b, m, ms);
    reveal(bytes_replay);
}


/// The error/`Ok` payload relationship the MP-level theorem needs, stated once.
pub open spec fn slack_mp_rel(
    b: Seq<u8>, ms: MHRecord, pc: int, xlen: int, out: Seq<u8>,
    r: Result<(int, int, Seq<u8>), Error>,
) -> bool {
    match mp_mask_phase(b, ms, pc, xlen, out) {
        Ok((_end, o)) => r == Ok((ms.width, ms.height, o)),
        Err(MpErr::Api(Error::TrailingBytes)) => exists|w: int, h: int, p: Seq<u8>|
            r == Ok((w, h, p)),
        Err(MpErr::Api(e)) => r == Err(e),
        Err(MpErr::Outside) => false,
    }
}


/// The R19 relationship between `MP(b,IcoEntryDib)` and `MP(b,IcoEntryDibSlack)`
/// as a predicate: the slack mode differs only where the exact-tail mode returns
/// `TrailingBytes`, and then it returns some completed image; every other result
/// is identical.
pub open spec fn slack_mp_ok(b: Seq<u8>, r: Result<(int, int, Seq<u8>), Error>) -> bool {
    match r {
        Ok(d) => mp(b, Mode::IcoEntryDibSlack) == Ok(d),
        Err(Error::TrailingBytes) => exists|w: int, h: int, p: Seq<u8>|
            mp(b, Mode::IcoEntryDibSlack) == Ok((w, h, p)),
        Err(other) => mp(b, Mode::IcoEntryDibSlack) == Err(other),
    }
}

/// `mp_decoded` congruence: it reads the record only for `Ok`'s width/height
/// (equal across an `ico_pair`); errors pass through unchanged.
pub proof fn lemma_mp_decoded_ico_pair(
    m: MHRecord, ms: MHRecord, rr: Result<(Seq<u8>, int), MpErr>,
)
    requires
        ico_pair(m, ms),
    ensures
        mp_decoded(ms, rr) == mp_decoded(m, rr),
{
    lemma_ico_pair_fields(m, ms);
    reveal(ico_pair);
    reveal(mp_decoded);
}

/// The raw phase never returns `TrailingBytes` (its only errors are
/// `TruncatedPixels` and `PaletteIndexOutOfRange`), so neither does `mp_raw`.
pub proof fn lemma_raw_full_err_ne_trailing(b: Seq<u8>, m: MHRecord, pos: int, x: int, y: int)
    ensures
        raw_full(b, m, pos, x, y) is Err ==> (raw_full(b, m, pos, x, y)->Err_0.0)
            != RawErr::Api(Error::TrailingBytes),
    decreases (m.height - y) as nat, (m.width - x) as nat,
{
    reveal(raw_full);
    if y >= m.height {
    } else if x >= m.width {
        lemma_raw_full_err_ne_trailing(b, m, pos + m.stride, 0, y + 1);
    } else if x == 0 && pos + m.stride > b.len() {
    } else if !raw_index_ok(b, m, pos, x) {
    } else {
        lemma_raw_full_err_ne_trailing(b, m, pos, x + 1, y);
    }
}

/// `mp_raw`/`mp_rle` therefore never yield `TrailingBytes`: the only errors it
/// can report come from the raw/RLE phase (`TruncatedPixels`,
/// `PaletteIndexOutOfRange`, `RleTruncated`, `RleBounds`).
pub proof fn lemma_mp_phase_err_ne_trailing(b: Seq<u8>, m: MHRecord, ico: bool)
    requires
        ico,
        m.compression == 0 || m.compression == 3,
        mr_raw_out(b, m, m.pixel_start) is Err,
    ensures
        !(mp_raw(b, m) == Err(MpErr::Api(Error::TrailingBytes))),
{
    reveal(mp_raw);
    reveal(mr_raw_out);
    reveal(mp_decoded);
    match mr_raw_out(b, m, m.pixel_start) {
        Err((e, _pc, _xc, _yc)) => {
            lemma_raw_full_err_ne_trailing(b, m, m.pixel_start, 0, 0);
            match e {
                RawErr::Api(e0) => { assert(e0 != Error::TrailingBytes); },
                RawErr::Outside => {},
            }
        },
        Ok(_) => {},
    }
}

/// `mr_full`'s error, when it fails, is the RLE error kind: `mr_full` returns
/// only `RleErr::Api` (never `Outside`), and only the four R14 RLE errors.
pub proof fn mr_cur_err_kind(b: Seq<u8>, m: MHRecord, p: int, x: int, y: int) -> (r: RleErr)
    requires
        mr_cur(b, m, p, x, y) is Err,
    ensures
        (mr_cur(b, m, p, x, y)->Err_0) == r,
        r is Api,
        r->Api_0 == Error::RleTruncated || r->Api_0 == Error::RleBounds
            || r->Api_0 == Error::PaletteIndexOutOfRange,
    decreases (b.len() - p) as nat,
{
    reveal(mr_cur);
    if p + 2 > b.len() {
        RleErr::Api(Error::RleTruncated)
    } else {
        let quantity = b[p] as int;
        let code = b[p + 1] as int;
        let p2 = p + 2;
        if quantity != 0 {
            if y >= m.height || x + quantity > m.width {
                RleErr::Api(Error::RleBounds)
            } else if !run_ok(m, code, quantity) {
                RleErr::Api(Error::PaletteIndexOutOfRange)
            } else {
                assert(mr_cur(b, m, p, x, y) == mr_cur(b, m, p2, x + quantity, y));
                assert(mr_cur(b, m, p2, x + quantity, y) is Err);
                let r = mr_cur_err_kind(b, m, p2, x + quantity, y);
                assert((mr_cur(b, m, p2, x + quantity, y)->Err_0) == r);
                r
            }
        } else if code == 1 {
            assert(false);
            RleErr::Api(Error::RleTruncated)
        } else if code == 0 {
            if y >= m.height {
                RleErr::Api(Error::RleBounds)
            } else {
                assert(mr_cur(b, m, p, x, y) == mr_cur(b, m, p2, 0, y + 1));
                assert(mr_cur(b, m, p2, 0, y + 1) is Err);
                let r = mr_cur_err_kind(b, m, p2, 0, y + 1);
                assert((mr_cur(b, m, p2, 0, y + 1)->Err_0) == r);
                r
            }
        } else if code == 2 {
            if p2 + 2 > b.len() {
                RleErr::Api(Error::RleTruncated)
            } else {
                let dx = b[p2] as int;
                let dy = b[p2 + 1] as int;
                let p4 = p2 + 2;
                if y + dy >= m.height || x + dx >= m.width {
                    RleErr::Api(Error::RleBounds)
                } else {
                    assert(mr_cur(b, m, p, x, y) == mr_cur(b, m, p4, x + dx, y + dy));
                    assert(mr_cur(b, m, p4, x + dx, y + dy) is Err);
                    let r = mr_cur_err_kind(b, m, p4, x + dx, y + dy);
                    assert((mr_cur(b, m, p4, x + dx, y + dy)->Err_0) == r);
                    r
                }
            }
        } else {
            let q = code;
            if y >= m.height || x + q > m.width {
                RleErr::Api(Error::RleBounds)
            } else {
                let padded = rle_padded(q, m.depth);
                if p2 + padded > b.len() {
                    RleErr::Api(Error::RleTruncated)
                } else if !abs_ok(b, m, p2, q) {
                    RleErr::Api(Error::PaletteIndexOutOfRange)
                } else {
                    assert(mr_cur(b, m, p, x, y) == mr_cur(b, m, p2 + padded, x + q, y));
                    assert(mr_cur(b, m, p2 + padded, x + q, y) is Err);
                    let r = mr_cur_err_kind(b, m, p2 + padded, x + q, y);
                    assert((mr_cur(b, m, p2 + padded, x + q, y)->Err_0) == r);
                    r
                }
            }
        }
    }
}

/// An `MH`-success record has a nonnegative pixel cursor (the R1 `bfOffBits`
/// bound, or the palette end in a non-BMP mode).
pub proof fn lemma_mh_pixel_start_nonneg(b: Seq<u8>, mode: Mode, m: MHRecord)
    requires
        mh(b, mode) == Ok(m),
    ensures
        0 <= m.pixel_start,
{
    reveal(mh);
    reveal(r2r7);
    reveal(r2r5);
    reveal(r6r7);
    reveal(r6r7_result);
    reveal(is_ico);
    reveal(visible_height_of);
    reveal(r1_prefix);
    if mode == Mode::BmpFile {
        assert(r1_prefix(b) is Ok);
        assert(m.pixel_start >= 18);
    } else {
        assert(m.pixel_start == m.palette_end || m.is_bmp);
        assert(m.palette_end >= 0) by {
            reveal(r6r7_result);
        }
    }
}

/// The RLE-path counterpart: `mp_rle` never yields `TrailingBytes` on a failing
/// RLE phase. `mr_out`'s error projects through `mr_full` to `mr_cur`
/// (`lemma_mr_full_proj`), whose error kind is one of the three R14 RLE errors.
pub proof fn lemma_mp_rle_err_ne_trailing(b: Seq<u8>, m: MHRecord)
    requires
        0 <= m.pixel_start,
        mr_out(b, m, m.pixel_start, 0, 0) is Err,
    ensures
        !(mp_rle(b, m) == Err(MpErr::Api(Error::TrailingBytes))),
{
    reveal(mp_rle);
    reveal(mr_out);
    reveal(mr_full);
    reveal(mr_cur);
    match mr_out(b, m, m.pixel_start, 0, 0) {
        Err((e, _pc, _xc, _yc)) => {
            assert(mr_full(b, m, m.pixel_start, 0, 0) is Err);
            lemma_mr_full_proj(b, m, m.pixel_start, 0, 0);
            assert(e == (mr_full(b, m, m.pixel_start, 0, 0)->Err_0).0);
            assert(mr_cur(b, m, m.pixel_start, 0, 0) == Err(e));
            let k = mr_cur_err_kind(b, m, m.pixel_start, 0, 0);
            assert(k == e);
        },
        Ok(_) => {},
    }
}

/// `MP`-level R19, raw path: given the raw phase reaching `(pc, out)` on a
/// record `m` (`IcoEntryDib`) whose slack counterpart is `ms`, the exact
/// `MP(b,IcoEntryDib)` and the terminal `MP(b,Slack)` relationship.
pub proof fn lemma_mp_rel_raw(
    b: Seq<u8>, m: MHRecord, ms: MHRecord, pc: int, out: Seq<u8>,
    r: Result<(int, int, Seq<u8>), Error>,
)
    requires
        ico_pair(m, ms),
        mh(b, Mode::IcoEntryDibSlack) == Ok(ms),
        m.compression == 0 || m.compression == 3,
        mr_raw_out(b, m, m.pixel_start) == Ok((pc, out)),
        mp(b, Mode::IcoEntryDib) == r,
    ensures
        slack_mp_ok(b, r),
{
    lemma_ico_pair_fields(m, ms);
    reveal(ico_pair);
    reveal(mp);
    reveal(mp_decoded);
    reveal(mp_raw);
    reveal(mp_mask_phase);
    reveal(mp_image_size_ok);
    reveal(mh);
    reveal(is_ico);
    reveal(exact_tail);
    reveal(with_mode);
    reveal(bytes_replay);
    lemma_mr_raw_out_ico_pair(b, m, ms, m.pixel_start);
    assert(mr_raw_out(b, ms, ms.pixel_start) == Ok((pc, out)));
    assert(m.pixel_start == ms.pixel_start);
    assert(m.compression == ms.compression);
    assert(ms.compression == 0 || ms.compression == 3);
    assert(m.palette_start + m.table_count * m.palette_stride <= b.len());
    assert(ms.palette_start + ms.table_count * ms.palette_stride <= b.len());
    assert(mp(b, Mode::IcoEntryDibSlack) == mp_decoded(ms, mp_raw(b, ms)));
    let xlen = pc - m.pixel_start;
    assert(mp_image_size_ok(ms, xlen) == mp_image_size_ok(m, xlen));
    assert(and_bytes(ms.width, ms.height) == and_bytes(m.width, m.height));
    // The exact-tail phase result drives `r`: `mp_raw` wraps the same phase.
    assert(mp_raw(b, m) == (match mp_mask_phase(b, m, pc, xlen, out) {
        Ok((end, out2)) => Ok::<_, MpErr>((out2, end)),
        Err(e) => Err(e),
    }));
    assert(mp(b, Mode::IcoEntryDib) == mp_decoded(m, mp_raw(b, m)));
    slack_mode_suffix_phase(b, m, pc, xlen, out, mp_mask_phase(b, m, pc, xlen, out));
    match mp_mask_phase(b, m, pc, xlen, out) {
        Ok((_end, o)) => {
            assert(r == Ok((m.width, m.height, o)));
            assert(mp_mask_phase(b, ms, pc, xlen, out) == Ok((_end, o)));
            assert(mp(b, Mode::IcoEntryDibSlack) == Ok((m.width, m.height, o)));
        },
        Err(MpErr::Api(Error::TrailingBytes)) => {
            let a = and_bytes(m.width, m.height);
            assert(mp_mask_phase(b, ms, pc, xlen, out)
                == Ok((pc + a, ma_output(b, ms, pc, out, explicit_alpha(ms)))));
            assert(r == Err(Error::TrailingBytes));
            let w = m.width;
            let h = m.height;
            let p = ma_output(b, ms, pc, out, explicit_alpha(ms));
            assert(mp(b, Mode::IcoEntryDibSlack) == Ok((w, h, p)));
        },
        Err(MpErr::Api(e2)) => {
            assert(mp_mask_phase(b, ms, pc, xlen, out) == Err(MpErr::Api(e2)));
            assert(r == Err(e2));
            assert(mp(b, Mode::IcoEntryDibSlack) == Err(e2));
        },
        Err(MpErr::Outside) => {},
    }
}

/// `MP`-level R19, RLE path.
pub proof fn lemma_mp_rel_rle(
    b: Seq<u8>, m: MHRecord, ms: MHRecord, pc: int, out: Seq<u8>,
    r: Result<(int, int, Seq<u8>), Error>,
)
    requires
        ico_pair(m, ms),
        mh(b, Mode::IcoEntryDibSlack) == Ok(ms),
        m.compression == 1 || m.compression == 2,
        mr_out(b, m, m.pixel_start, 0, 0) == Ok((pc, out)),
        mp(b, Mode::IcoEntryDib) == r,
    ensures
        slack_mp_ok(b, r),
{
    lemma_ico_pair_fields(m, ms);
    reveal(ico_pair);
    reveal(mp);
    reveal(mp_decoded);
    reveal(mp_rle);
    reveal(mp_mask_phase);
    reveal(mp_image_size_ok);
    reveal(mh);
    reveal(is_ico);
    reveal(exact_tail);
    reveal(with_mode);
    reveal(bytes_replay);
    lemma_mr_out_ico_pair(b, m, ms);
    assert(mr_out(b, ms, ms.pixel_start, 0, 0) == Ok((pc, out)));
    assert(m.pixel_start == ms.pixel_start);
    assert(m.compression == ms.compression);
    assert(ms.compression == 1 || ms.compression == 2);
    assert(m.palette_start + m.table_count * m.palette_stride <= b.len());
    assert(ms.palette_start + ms.table_count * ms.palette_stride <= b.len());
    assert(mp(b, Mode::IcoEntryDibSlack) == mp_decoded(ms, mp_rle(b, ms)));
    let xlen = pc - m.pixel_start;
    assert(mp_image_size_ok(ms, xlen) == mp_image_size_ok(m, xlen));
    assert(and_bytes(ms.width, ms.height) == and_bytes(m.width, m.height));
    assert(mp_rle(b, m) == (match mp_mask_phase(b, m, pc, xlen, out) {
        Ok((end, out2)) => Ok::<_, MpErr>((out2, end)),
        Err(e) => Err(e),
    }));
    assert(mp(b, Mode::IcoEntryDib) == mp_decoded(m, mp_rle(b, m)));
    slack_mode_suffix_phase(b, m, pc, xlen, out, mp_mask_phase(b, m, pc, xlen, out));
    match mp_mask_phase(b, m, pc, xlen, out) {
        Ok((_end, o)) => {
            assert(r == Ok((m.width, m.height, o)));
            assert(mp_mask_phase(b, ms, pc, xlen, out) == Ok((_end, o)));
            assert(mp(b, Mode::IcoEntryDibSlack) == Ok((m.width, m.height, o)));
        },
        Err(MpErr::Api(Error::TrailingBytes)) => {
            let a = and_bytes(m.width, m.height);
            assert(mp_mask_phase(b, ms, pc, xlen, out)
                == Ok((pc + a, ma_output(b, ms, pc, out, explicit_alpha(ms)))));
            assert(r == Err(Error::TrailingBytes));
            let w = m.width;
            let h = m.height;
            let p = ma_output(b, ms, pc, out, explicit_alpha(ms));
            assert(mp(b, Mode::IcoEntryDibSlack) == Ok((w, h, p)));
        },
        Err(MpErr::Api(e2)) => {
            assert(mp_mask_phase(b, ms, pc, xlen, out) == Err(MpErr::Api(e2)));
            assert(r == Err(e2));
            assert(mp(b, Mode::IcoEntryDibSlack) == Err(e2));
        },
        Err(MpErr::Outside) => {},
    }
}

/// `MP`-level R19, header-`Ok` case: given `MH(b,IcoEntryDib) = Ok(m)`, relate
/// `MP(b,IcoEntryDib)` and `MP(b,IcoEntryDibSlack)` at their exact results.
pub proof fn lemma_slack_result_mp_ok(
    b: Seq<u8>, m: MHRecord, r: Result<(int, int, Seq<u8>), Error>,
)
    requires
        mh(b, Mode::IcoEntryDib) == Ok(m),
        mp(b, Mode::IcoEntryDib) == r,
    ensures
        slack_mp_ok(b, r),
{
    slack_mode_same_prefix(b, m);
    let ms = with_mode(m, Mode::IcoEntryDibSlack);
    assert(mh(b, Mode::IcoEntryDibSlack) == Ok(ms));
    assert(ico_pair(m, ms));
    lemma_ico_pair_fields(m, ms);
    reveal(mp);
    reveal(mp_decoded);
    reveal(mp_raw);
    reveal(mp_rle);
    reveal(mp_image_size_ok);
    reveal(mh);
    reveal(is_ico);
    reveal(exact_tail);
    reveal(with_mode);
    reveal(ico_pair);
    reveal(bytes_replay);
    reveal(slack_mp_ok);
    if m.palette_start + m.table_count * m.palette_stride > b.len() {
        assert(mp(b, Mode::IcoEntryDib) == Err(Error::TruncatedPalette));
        assert(r == Err(Error::TruncatedPalette));
        assert(mp(b, Mode::IcoEntryDibSlack) == Err(Error::TruncatedPalette));
        assert(slack_mp_ok(b, r));
    } else if m.compression == 0 || m.compression == 3 {
        assert(m.compression == ms.compression);
        lemma_mr_raw_out_ico_pair(b, m, ms, m.pixel_start);
        match mr_raw_out(b, m, m.pixel_start) {
            Err(_) => {
                // `mp_raw` on a failing raw phase returns `Err(Api(e))` for the
                // raw error `e`, so equal raw results give equal `mp_raw`.
                assert(mr_raw_out(b, ms, ms.pixel_start) == mr_raw_out(b, m, m.pixel_start));
                assert(mp_raw(b, ms) == mp_raw(b, m)) by {
                    reveal(mp_raw);
                }
                assert(ms.palette_start + ms.table_count * ms.palette_stride <= b.len());
                assert(mp(b, Mode::IcoEntryDibSlack) == mp_decoded(ms, mp_raw(b, ms))) by {
                    reveal(mp);
                    reveal(mh);
                    reveal(is_ico);
                }
                assert(mp(b, Mode::IcoEntryDib) == mp_decoded(m, mp_raw(b, m))) by {
                    reveal(mp);
                    reveal(mh);
                    reveal(is_ico);
                }
                lemma_mp_decoded_ico_pair(m, ms, mp_raw(b, m));
                lemma_mp_phase_err_ne_trailing(b, m, true);
                assert(!(mp_raw(b, m) == Err(MpErr::Api(Error::TrailingBytes))));
                assert(!(r == Err(Error::TrailingBytes)));
                assert(mp(b, Mode::IcoEntryDibSlack) == r);
                assert(slack_mp_ok(b, r));
            },
            Ok((pc, out)) => {
                assert(mp(b, Mode::IcoEntryDibSlack) == mp_decoded(ms, mp_raw(b, ms)));
                lemma_mp_rel_raw(b, m, ms, pc, out, r);
                assert(slack_mp_ok(b, r));
            },
        }
    } else {
        assert(m.compression == ms.compression);
        lemma_mr_out_ico_pair(b, m, ms);
        match mr_out(b, m, m.pixel_start, 0, 0) {
            Err(_) => {
                assert(mp_rle(b, ms) == mp_rle(b, m));
                assert(ms.palette_start + ms.table_count * ms.palette_stride <= b.len());
                assert(mp(b, Mode::IcoEntryDibSlack) == mp_decoded(ms, mp_rle(b, ms))) by {
                    reveal(mp);
                    reveal(mh);
                    reveal(is_ico);
                }
                assert(mp(b, Mode::IcoEntryDib) == mp_decoded(m, mp_rle(b, m))) by {
                    reveal(mp);
                    reveal(mh);
                    reveal(is_ico);
                }
                lemma_mp_decoded_ico_pair(m, ms, mp_rle(b, m));
                lemma_mh_pixel_start_nonneg(b, Mode::IcoEntryDib, m);
                lemma_mp_rle_err_ne_trailing(b, m);
                assert(!(mp_rle(b, m) == Err(MpErr::Api(Error::TrailingBytes))));
                assert(!(r == Err(Error::TrailingBytes)));
                assert(mp(b, Mode::IcoEntryDibSlack) == r);
                assert(slack_mp_ok(b, r));
            },
            Ok((pc, out)) => {
                assert(mp(b, Mode::IcoEntryDibSlack) == mp_decoded(ms, mp_rle(b, ms)));
                lemma_mp_rel_rle(b, m, ms, pc, out, r);
                assert(slack_mp_ok(b, r));
            },
        }
    }
}

/// `MP`-level R19: `MP(b,Slack)` and `MP(b,IcoEntryDib)` differ only when the
/// latter returns `TrailingBytes`, in which case the former returns the
/// completed decoded image; every other result — the success image and every
/// other error — is identical. This is the manifest's `slack_mode_suffix_result`.
pub proof fn slack_mode_suffix_result(
    b: Seq<u8>, r: Result<(int, int, Seq<u8>), Error>,
)
    requires
        mp(b, Mode::IcoEntryDib) == r,
    ensures
        slack_mp_ok(b, r),
{
    reveal(mp);
    reveal(mp_decoded);
    reveal(mh);
    reveal(r2r7);
    reveal(r2r5);
    reveal(r6r7);
    reveal(r6r7_result);
    reveal(visible_height_of);
    reveal(r1_prefix);
    reveal(is_ico);
    reveal(with_mode);
    reveal(ico_pair);
    reveal(mp);
    reveal(mp_decoded);
    reveal(mp_raw);
    reveal(mp_rle);
    reveal(mh);
    match mh(b, Mode::IcoEntryDib) {
        Err(e) => {
            assert(mh(b, Mode::IcoEntryDibSlack) == Err(e));
            assert(mp(b, Mode::IcoEntryDib) == Err(e));
            assert(mp(b, Mode::IcoEntryDibSlack) == Err(e));
            assert(r == Err(e));
        },
        Ok(m) => {
            lemma_slack_result_mp_ok(b, m, r);
        },
    }
}

// ---------------------------------------------------------------------------
// P1 · the policy-indexed model (POLICY-SPEC "Independent options",
// THEOREMS-POLICY-DELTA "Model and executable boundary").
//
// `MH_policy` differs from `MH` in exactly one place: R1's `bfSize` test, which
// is skipped when `file_size = Ignore`. `MP_policy` differs from `MP` in exactly
// one place: R12's `biSizeImage` equality, skipped when `image_size = Ignore`.
// Every other guard and every byte-producing transition is `MH`/`MP`'s own
// function, and the derived header record `MH_policy` hands to `MP_policy` is
// one of the two `MHRecord`s unchanged — not a new record type — so the pixel
// phases cannot drift between the strict and profiled models.
// ---------------------------------------------------------------------------

/// `MH_policy`'s R1 prefix: `r1_prefix` with exactly the `file_size` test
/// governed by the option. `Declared` keeps 1.1's test
/// (`le32(b,2) != 0 && le32(b,2) > |b|`); `Ignore` drops that test and keeps the
/// reserved-words test, which belongs to the separate `reserved` field alone.
/// Neither value makes `bfSize` a read bound: the field is validated or skipped,
/// never used to compute an offset or a length.
pub open spec fn r1_size_bad(b: Seq<u8>, p: Policy) -> bool {
    (p.file_size == MetadataOptions::Declared && le32(b, 2) != 0 && le32(b, 2) > b.len())
        || (p.reserved == ReservedOptions::Zero && (le16(b, 6) != 0 || le16(b, 8) != 0))
}

/// R1: whether the declared pixel offset is one the option rejects.
pub open spec fn r1_offset_bad(b: Seq<u8>, p: Policy) -> bool {
    p.offset == OffsetOptions::Declared && (le32(b, 10) < 18 || le32(b, 10) > b.len())
}

pub open spec fn r1_prefix_policy(b: Seq<u8>, p: Policy) -> Result<(int, int), Error> {
    if b.len() < 14 {
        Err(Error::TruncatedFileHeader)
    } else if b[0] != 66u8 || b[1] != 77u8 {
        Err(Error::BadMagic)
    } else if r1_size_bad(b, p) {
        Err(Error::InvalidFileSize)
    } else if r1_offset_bad(b, p) {
        Err(Error::InvalidPixelOffset)
    } else {
        Ok((14int, le32(b, 10)))
    }
}

/// R1/R7 `offset`: whether the declared `bfOffBits` is one R1 rejects. Only
/// `Declared` rejects; `AllowAny`/`Gdk` accept and repair later.
pub open spec fn offset_rejects_r1(b: Seq<u8>, p: Policy) -> bool {
    p.offset == OffsetOptions::Declared && (le32(b, 10) < 18 || le32(b, 10) > b.len())
}

/// R2 `headers`: the accepted DIB header size set per option value.
/// `All1_1` is `hsize_ok` verbatim, so `Strict` reduces definitionally.
pub open spec fn hsize_ok_policy(h: int, o: HeadersOptions) -> bool {
    match o {
        HeadersOptions::All1_1 => hsize_ok(h),
        HeadersOptions::No16 => h == 12 || h == 40 || h == 52 || h == 56 || h == 64 || h == 108
            || h == 124,
        HeadersOptions::No16No64 => h == 12 || h == 40 || h == 52 || h == 56 || h == 108 || h == 124,
    }
}

/// R5/R7 `high_count`: the number of optional high-depth table entries R7 skips.
/// `BmpTable` skips `used`; `Ignore` and `GdkBound` skip none.
pub open spec fn table_count_of_policy(rf: RawFields, o: HighCountOptions) -> int {
    if rf.depth <= 8 {
        table_count_of(rf)
    } else {
        match o {
            HighCountOptions::BmpTable => rf.used,
            HighCountOptions::Ignore => 0,
            HighCountOptions::GdkBound => 0,
        }
    }
}

/// R6 `masks`: the three-value validity rule over the four `u32` words.
/// `Reject` is `masks_ok`; `Independent` checks each mask's own run and depth
/// but not overlap and requires the three RGB masks nonzero; `DefaultRGB`
/// rejects only a bit above depth.
pub open spec fn masks_ok_policy(m0: u32, m1: u32, m2: u32, m3: u32, bits: int, o: MasksOptions)
    -> bool {
    match o {
        MasksOptions::Reject => masks_ok(m0, m1, m2, m3, bits),
        MasksOptions::Independent => {
            &&& m0 != 0u32
            &&& m1 != 0u32
            &&& m2 != 0u32
            &&& contiguous(m0)
            &&& contiguous(m1)
            &&& contiguous(m2)
            &&& contiguous(m3)
            &&& within_bits(m0, bits)
            &&& within_bits(m1, bits)
            &&& within_bits(m2, bits)
            &&& within_bits(m3, bits)
        },
        MasksOptions::DefaultRGB => {
            &&& within_bits(m0, bits)
            &&& within_bits(m1, bits)
            &&& within_bits(m2, bits)
            &&& within_bits(m3, bits)
        },
    }
}

/// R6 `masks = DefaultRGB`: the default mask quadruple substituted when any RGB
/// mask is zero — 5:5:5 opaque at 16 bpp, BGRA 8888 with alpha `0xff000000` at
/// 32 bpp.
pub open spec fn default_rgb_masks(bits: int) -> (u32, u32, u32, u32) {
    if bits == 16 {
        (0x7c00u32, 0x03e0u32, 0x001fu32, 0u32)
    } else {
        (0x00ff0000u32, 0x0000ff00u32, 0x000000ffu32, 0xff000000u32)
    }
}

/// R6: the four mask words after the `masks` option's substitution step. For
/// `DefaultRGB` a zero RGB mask replaces all four; the other two values leave
/// them unchanged.
pub open spec fn masks_after_policy(
    m0: u32,
    m1: u32,
    m2: u32,
    m3: u32,
    bits: int,
    o: MasksOptions,
) -> (u32, u32, u32, u32) {
    match o {
        MasksOptions::DefaultRGB => {
            if !(m0 != 0u32 && m1 != 0u32 && m2 != 0u32) {
                default_rgb_masks(bits)
            } else {
                (m0, m1, m2, m3)
            }
        },
        _ => (m0, m1, m2, m3),
    }
}

/// R5/R7 `high_count = GdkBound`: the modeled acceptance threshold on a
/// nonzero high-depth `biClrUsed` — the low 32 bits of `2^depth`, so 65536 at
/// 16 bpp, 16777216 at 24 bpp and **zero** at 32 bpp (POLICY-SPEC's "the modeled
/// 32-bpp threshold is zero").
pub open spec fn gdk_high_bound(depth: int) -> int {
    if depth == 16 {
        65536
    } else if depth == 24 {
        16777216
    } else {
        0
    }
}

/// R2..R5 with the `planes` and `headers` options: `r2r5` with R2's accepted
/// size set selected by `headers` and R3's `planes == 1` governed by `planes`.
/// Every other guard, and the `RawFields` it builds, are `r2r5`'s own.
pub open spec fn width_i_of(b: Seq<u8>, s: int) -> int {
    if le32(b, s) == 12 {
        le16(b, s + 4)
    } else {
        s32(b, s + 4)
    }
}

pub open spec fn stored_of(b: Seq<u8>, s: int) -> int {
    if le32(b, s) == 12 {
        le16(b, s + 6)
    } else {
        s32(b, s + 8)
    }
}

pub open spec fn planes_of(b: Seq<u8>, s: int) -> int {
    if le32(b, s) == 12 {
        le16(b, s + 8)
    } else {
        le16(b, s + 12)
    }
}

pub open spec fn depth_of(b: Seq<u8>, s: int) -> int {
    if le32(b, s) == 12 {
        le16(b, s + 10)
    } else {
        le16(b, s + 14)
    }
}

pub open spec fn compression_of(b: Seq<u8>, s: int) -> int {
    if le32(b, s) == 12 || le32(b, s) == 16 {
        0
    } else {
        le32(b, s + 16)
    }
}

pub open spec fn used_of(b: Seq<u8>, s: int) -> int {
    if le32(b, s) == 12 || le32(b, s) == 16 {
        0
    } else {
        le32(b, s + 32)
    }
}

pub open spec fn declared_of(b: Seq<u8>, s: int) -> int {
    if le32(b, s) == 12 || le32(b, s) == 16 {
        0
    } else {
        le32(b, s + 20)
    }
}

/// The R4 depth/compression pairing rejection.
pub open spec fn comp_of(b: Seq<u8>, s: int) -> bool {
    (compression_of(b, s) == 1 && depth_of(b, s) != 8) || (compression_of(b, s) == 2 && depth_of(b, s)
        != 4) || (compression_of(b, s) == 3 && depth_of(b, s) != 16 && depth_of(b, s) != 32)
}

pub open spec fn header_bytes_needed(h: int, p: Policy) -> int {
    if p.header_bytes == crate::dib::HeaderBytesOptions::ParsedFields && h >= 40 { 40 } else { h }
}

pub open spec fn policy_masks_missing(b: Seq<u8>, s: int, rf: RawFields, p: Policy) -> bool {
    rf.compression == 3 && (
        if rf.hsize == 40 || rf.hsize == 64 { b.len() - rf.pos0 < 12 }
        else { p.header_bytes == crate::dib::HeaderBytesOptions::ParsedFields
            && b.len() < s + (if rf.hsize >= 56 { 56int } else { 52int }) })
}

pub open spec fn r2r5_policy(
    b: Seq<u8>,
    mode: Mode,
    s: int,
    p: Policy,
) -> Result<RawFields, Error> {
    if b.len() < s + 4 {
        Err(Error::TruncatedDibHeader)
    } else if !hsize_ok_policy(le32(b, s), p.headers) {
        Err(Error::UnsupportedHeaderSize)
    } else if b.len() < s + header_bytes_needed(le32(b, s), p) {
        Err(Error::TruncatedHeader)
    } else {
        let hsize: int = le32(b, s);
        let core: bool = hsize == 12;
        let width_i: int = if core {
            le16(b, s + 4)
        } else {
            s32(b, s + 4)
        };
        let stored: int = if core {
            le16(b, s + 6)
        } else {
            s32(b, s + 8)
        };
        let planes: int = if core {
            le16(b, s + 8)
        } else {
            le16(b, s + 12)
        };
        let depth: int = if core {
            le16(b, s + 10)
        } else {
            le16(b, s + 14)
        };
        let compression: int = if core || hsize == 16 {
            0
        } else {
            le32(b, s + 16)
        };
        let declared: int = if core || hsize == 16 {
            0
        } else {
            le32(b, s + 20)
        };
        let used: int = if core || hsize == 16 {
            0
        } else {
            le32(b, s + 32)
        };
        let height_v: int = visible_height_of(mode, abs_int(stored));
        if p.planes == PlanesOptions::Must1 && planes_of(b, s) != 1 {
            Err(Error::InvalidPlanes)
        } else if width_i_of(b, s) <= 0 || stored_of(b, s) == 0 || stored_of(b, s) == s32_min() {
            Err(Error::InvalidDimensions)
        } else if is_ico(mode) && (stored_of(b, s) < 0 || stored_of(b, s) % 2 != 0) {
            Err(Error::InvalidDimensions)
        } else if width_i_of(b, s) > max_dimension() || visible_height_of(mode, abs_int(stored_of(
            b,
            s,
        ))) > max_dimension() {
            Err(Error::InvalidDimensions)
        } else if !(depth_ok(depth_of(b, s)) || (p.unsupported == UnsupportedOptions::Rgb2
            && le32(b, s) >= 40 && depth_of(b, s) == 2)) {
            Err(Error::UnsupportedPixelDepth)
        } else if compression_of(b, s) > 3 {
            Err(Error::UnsupportedCompression)
        } else if comp_of(b, s) {
            Err(Error::InvalidCompressionDepth)
        } else if (stored_of(b, s) < 0 && (compression_of(b, s) == 1 || compression_of(b, s)
            == 2)) || (is_ico(mode) && (compression_of(b, s) == 1 || compression_of(b, s) == 2)) {
            Err(Error::UnsupportedOrientation)
        } else if (depth_of(b, s) <= 8 && used_of(b, s) > pow2(depth_of(b, s))) || (depth_of(b, s)
            > 8 && used_of(b, s) != 0 && mode != Mode::BmpFile) {
            Err(Error::InvalidColorCount)
        } else if depth_of(b, s) > 8 && used_of(b, s) != 0 && mode == Mode::BmpFile
            && p.high_count == HighCountOptions::GdkBound {
            if used_of(b, s) > gdk_high_bound(depth_of(b, s)) {
                Err(Error::InvalidColorCount)
            } else {
                r2r5_policy_tail(b, mode, s, hsize, core, width_i, stored, planes, depth, compression, declared, used, height_v)
            }
        } else if 4 * (width_i_of(b, s) * visible_height_of(mode, abs_int(stored_of(b, s))))
            > max_output() {
            Err(Error::OutputTooLarge)
        } else {
            Ok(
                RawFields {
                    hsize,
                    core,
                    width: width_i,
                    stored,
                    planes,
                    depth,
                    compression,
                    declared,
                    used,
                    height: height_v,
                    pos0: s + hsize,
                },
            )
        }
    }
}

/// `r2r5_policy`'s tail from the output cap on, with no policy bracket (the
/// `planes`, `headers` and `high_count` guards have all been decided).
pub open spec fn r2r5_policy_tail(
    b: Seq<u8>,
    mode: Mode,
    s: int,
    hsize: int,
    core: bool,
    width_i: int,
    stored: int,
    planes: int,
    depth: int,
    compression: int,
    declared: int,
    used: int,
    height_v: int,
) -> Result<RawFields, Error> {
    if 4 * (width_i * height_v) > max_output() {
        Err(Error::OutputTooLarge)
    } else {
        Ok(
            RawFields {
                hsize,
                core,
                width: width_i,
                stored,
                planes,
                depth,
                compression,
                declared,
                used,
                height: height_v,
                pos0: s + hsize,
            },
        )
    }
}

/// `r2r5_policy`'s `bits <= 8` colour-count guard.
pub proof fn lemma_r2r5_policy_low_count_err(b: Seq<u8>, mode: Mode, s: int, p: Policy)
    requires
        b.len() >= s + 4,
        hsize_ok_policy(le32(b, s), p.headers),
        b.len() >= s + header_bytes_needed(le32(b, s), p),
        !(p.planes == PlanesOptions::Must1 && (if le32(b, s) == 12 {
            le16(b, s + 8)
        } else {
            le16(b, s + 12)
        }) != 1),
        !((if le32(b, s) == 12 {
            le16(b, s + 4) as int
        } else {
            s32(b, s + 4)
        }) <= 0 || (if le32(b, s) == 12 {
            le16(b, s + 6) as int
        } else {
            s32(b, s + 8)
        }) == 0 || (if le32(b, s) == 12 {
            le16(b, s + 6) as int
        } else {
            s32(b, s + 8)
        }) == s32_min()),
        !(is_ico(mode) && ((if le32(b, s) == 12 {
            le16(b, s + 6) as int
        } else {
            s32(b, s + 8)
        }) < 0 || (if le32(b, s) == 12 {
            le16(b, s + 6) as int
        } else {
            s32(b, s + 8)
        }) % 2 != 0)),
        (if le32(b, s) == 12 {
            le16(b, s + 10) as int
        } else {
            le16(b, s + 14)
        }) <= 8,
        (if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 32)
        }) > pow2((if le32(b, s) == 12 {
            le16(b, s + 10) as int
        } else {
            le16(b, s + 14)
        })),
        (if le32(b, s) == 12 {
            le16(b, s + 4) as int
        } else {
            s32(b, s + 4)
        }) <= max_dimension(),
        visible_height_of(mode, abs_int(if le32(b, s) == 12 {
            le16(b, s + 6) as int
        } else {
            s32(b, s + 8)
        })) <= max_dimension(),
        (depth_ok(if le32(b, s) == 12 {
            le16(b, s + 10) as int
        } else {
            le16(b, s + 14)
        }) || (p.unsupported == UnsupportedOptions::Rgb2 && le32(b, s) >= 40
            && le16(b, s + 14) == 2)),
        (if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 16)
        }) <= 3,
        !((if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 16)
        }) == 1 && (if le32(b, s) == 12 {
            le16(b, s + 10) as int
        } else {
            le16(b, s + 14)
        }) != 8),
        !((if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 16)
        }) == 2 && (if le32(b, s) == 12 {
            le16(b, s + 10) as int
        } else {
            le16(b, s + 14)
        }) != 4),
        !((if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 16)
        }) == 3 && (if le32(b, s) == 12 {
            le16(b, s + 10) as int
        } else {
            le16(b, s + 14)
        }) != 16 && (if le32(b, s) == 12 {
            le16(b, s + 10) as int
        } else {
            le16(b, s + 14)
        }) != 32),
        !((if le32(b, s) == 12 {
            le16(b, s + 6) as int
        } else {
            s32(b, s + 8)
        }) < 0 && ((if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 16)
        }) == 1 || (if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 16)
        }) == 2)),
        !(is_ico(mode) && ((if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 16)
        }) == 1 || (if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 16)
        }) == 2)),
    ensures
        r2r5_policy(b, mode, s, p) == Err(Error::InvalidColorCount),
{
    reveal(r2r5_policy);
    reveal(hsize_ok_policy);
    reveal(hsize_ok);
    reveal(pow2);
    reveal(is_ico);
    reveal(visible_height_of);
    reveal(depth_ok);
}

/// `r2r5_policy`'s `bits > 8 && !BmpFile` colour-count guard.
pub proof fn lemma_r2r5_policy_bare_high_err(b: Seq<u8>, mode: Mode, s: int, p: Policy)
    requires
        b.len() >= s + 4,
        hsize_ok_policy(le32(b, s), p.headers),
        b.len() >= s + header_bytes_needed(le32(b, s), p),
        !(p.planes == PlanesOptions::Must1 && (if le32(b, s) == 12 {
            le16(b, s + 8)
        } else {
            le16(b, s + 12)
        }) != 1),
        !((if le32(b, s) == 12 {
            le16(b, s + 4) as int
        } else {
            s32(b, s + 4)
        }) <= 0 || (if le32(b, s) == 12 {
            le16(b, s + 6) as int
        } else {
            s32(b, s + 8)
        }) == 0 || (if le32(b, s) == 12 {
            le16(b, s + 6) as int
        } else {
            s32(b, s + 8)
        }) == s32_min()),
        !(is_ico(mode) && ((if le32(b, s) == 12 {
            le16(b, s + 6) as int
        } else {
            s32(b, s + 8)
        }) < 0 || (if le32(b, s) == 12 {
            le16(b, s + 6) as int
        } else {
            s32(b, s + 8)
        }) % 2 != 0)),
        (if le32(b, s) == 12 {
            le16(b, s + 10) as int
        } else {
            le16(b, s + 14)
        }) > 8,
        (if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 32)
        }) != 0,
        mode != Mode::BmpFile,
        (if le32(b, s) == 12 {
            le16(b, s + 4) as int
        } else {
            s32(b, s + 4)
        }) <= max_dimension(),
        visible_height_of(mode, abs_int(if le32(b, s) == 12 {
            le16(b, s + 6) as int
        } else {
            s32(b, s + 8)
        })) <= max_dimension(),
        (depth_ok(if le32(b, s) == 12 {
            le16(b, s + 10) as int
        } else {
            le16(b, s + 14)
        }) || (p.unsupported == UnsupportedOptions::Rgb2 && le32(b, s) >= 40
            && le16(b, s + 14) == 2)),
        (if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 16)
        }) <= 3,
        !((if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 16)
        }) == 1 && (if le32(b, s) == 12 {
            le16(b, s + 10) as int
        } else {
            le16(b, s + 14)
        }) != 8),
        !((if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 16)
        }) == 2 && (if le32(b, s) == 12 {
            le16(b, s + 10) as int
        } else {
            le16(b, s + 14)
        }) != 4),
        !((if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 16)
        }) == 3 && (if le32(b, s) == 12 {
            le16(b, s + 10) as int
        } else {
            le16(b, s + 14)
        }) != 16 && (if le32(b, s) == 12 {
            le16(b, s + 10) as int
        } else {
            le16(b, s + 14)
        }) != 32),
        !((if le32(b, s) == 12 {
            le16(b, s + 6) as int
        } else {
            s32(b, s + 8)
        }) < 0 && ((if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 16)
        }) == 1 || (if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 16)
        }) == 2)),
        !(is_ico(mode) && ((if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 16)
        }) == 1 || (if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 16)
        }) == 2)),
    ensures
        r2r5_policy(b, mode, s, p) == Err(Error::InvalidColorCount),
{
    reveal(r2r5_policy);
    reveal(hsize_ok_policy);
    reveal(hsize_ok);
    reveal(pow2);
    reveal(is_ico);
    reveal(visible_height_of);
}

/// `r2r5_policy`'s `high_count = GdkBound` guard.
pub proof fn lemma_r2r5_policy_gdk_count_err(b: Seq<u8>, mode: Mode, s: int, p: Policy)
    requires
        b.len() >= s + 4,
        hsize_ok_policy(le32(b, s), p.headers),
        b.len() >= s + header_bytes_needed(le32(b, s), p),
        !(p.planes == PlanesOptions::Must1 && (if le32(b, s) == 12 {
            le16(b, s + 8)
        } else {
            le16(b, s + 12)
        }) != 1),
        !((if le32(b, s) == 12 {
            le16(b, s + 4) as int
        } else {
            s32(b, s + 4)
        }) <= 0 || (if le32(b, s) == 12 {
            le16(b, s + 6) as int
        } else {
            s32(b, s + 8)
        }) == 0 || (if le32(b, s) == 12 {
            le16(b, s + 6) as int
        } else {
            s32(b, s + 8)
        }) == s32_min()),
        !(is_ico(mode) && ((if le32(b, s) == 12 {
            le16(b, s + 6) as int
        } else {
            s32(b, s + 8)
        }) < 0 || (if le32(b, s) == 12 {
            le16(b, s + 6) as int
        } else {
            s32(b, s + 8)
        }) % 2 != 0)),
        (if le32(b, s) == 12 {
            le16(b, s + 10) as int
        } else {
            le16(b, s + 14)
        }) > 8,
        (if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 32)
        }) != 0,
        mode == Mode::BmpFile,
        p.high_count == HighCountOptions::GdkBound,
        (if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 32)
        }) > gdk_high_bound((if le32(b, s) == 12 {
            le16(b, s + 10) as int
        } else {
            le16(b, s + 14)
        })),
        (if le32(b, s) == 12 {
            le16(b, s + 4) as int
        } else {
            s32(b, s + 4)
        }) <= max_dimension(),
        visible_height_of(mode, abs_int(if le32(b, s) == 12 {
            le16(b, s + 6) as int
        } else {
            s32(b, s + 8)
        })) <= max_dimension(),
        (depth_ok(if le32(b, s) == 12 {
            le16(b, s + 10) as int
        } else {
            le16(b, s + 14)
        }) || (p.unsupported == UnsupportedOptions::Rgb2 && le32(b, s) >= 40
            && le16(b, s + 14) == 2)),
        (if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 16)
        }) <= 3,
        !((if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 16)
        }) == 1 && (if le32(b, s) == 12 {
            le16(b, s + 10) as int
        } else {
            le16(b, s + 14)
        }) != 8),
        !((if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 16)
        }) == 2 && (if le32(b, s) == 12 {
            le16(b, s + 10) as int
        } else {
            le16(b, s + 14)
        }) != 4),
        !((if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 16)
        }) == 3 && (if le32(b, s) == 12 {
            le16(b, s + 10) as int
        } else {
            le16(b, s + 14)
        }) != 16 && (if le32(b, s) == 12 {
            le16(b, s + 10) as int
        } else {
            le16(b, s + 14)
        }) != 32),
        !((if le32(b, s) == 12 {
            le16(b, s + 6) as int
        } else {
            s32(b, s + 8)
        }) < 0 && ((if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 16)
        }) == 1 || (if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 16)
        }) == 2)),
        !(is_ico(mode) && ((if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 16)
        }) == 1 || (if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 16)
        }) == 2)),
    ensures
        r2r5_policy(b, mode, s, p) == Err(Error::InvalidColorCount),
{
    reveal(r2r5_policy);
    reveal(hsize_ok_policy);
    reveal(hsize_ok);
    reveal(pow2);
    reveal(is_ico);
    reveal(visible_height_of);
    reveal(gdk_high_bound);
}

/// `r2r5_policy`'s output cap.
pub proof fn lemma_r2r5_policy_output_err(b: Seq<u8>, mode: Mode, s: int, p: Policy)
    requires
        b.len() >= s + 4,
        hsize_ok_policy(le32(b, s), p.headers),
        b.len() >= s + header_bytes_needed(le32(b, s), p),
        !(p.planes == PlanesOptions::Must1 && (if le32(b, s) == 12 {
            le16(b, s + 8)
        } else {
            le16(b, s + 12)
        }) != 1),
        !((if le32(b, s) == 12 {
            le16(b, s + 4) as int
        } else {
            s32(b, s + 4)
        }) <= 0 || (if le32(b, s) == 12 {
            le16(b, s + 6) as int
        } else {
            s32(b, s + 8)
        }) == 0 || (if le32(b, s) == 12 {
            le16(b, s + 6) as int
        } else {
            s32(b, s + 8)
        }) == s32_min()),
        !(is_ico(mode) && ((if le32(b, s) == 12 {
            le16(b, s + 6) as int
        } else {
            s32(b, s + 8)
        }) < 0 || (if le32(b, s) == 12 {
            le16(b, s + 6) as int
        } else {
            s32(b, s + 8)
        }) % 2 != 0)),
        !((if le32(b, s) == 12 {
            le16(b, s + 10) as int
        } else {
            le16(b, s + 14)
        }) <= 8 && (if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 32)
        }) > pow2((if le32(b, s) == 12 {
            le16(b, s + 10) as int
        } else {
            le16(b, s + 14)
        }))),
        !((if le32(b, s) == 12 {
            le16(b, s + 10) as int
        } else {
            le16(b, s + 14)
        }) > 8 && (if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 32)
        }) != 0 && mode != Mode::BmpFile),
        !((if le32(b, s) == 12 {
            le16(b, s + 10) as int
        } else {
            le16(b, s + 14)
        }) > 8 && (if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 32)
        }) != 0 && mode == Mode::BmpFile && p.high_count == HighCountOptions::GdkBound
            && (if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 32)
        }) > gdk_high_bound((if le32(b, s) == 12 {
            le16(b, s + 10) as int
        } else {
            le16(b, s + 14)
        }))),
        4 * ((if le32(b, s) == 12 {
            le16(b, s + 4) as int
        } else {
            s32(b, s + 4)
        }) * visible_height_of(mode, abs_int(if le32(b, s) == 12 {
            le16(b, s + 6) as int
        } else {
            s32(b, s + 8)
        }))) > max_output(),
        (if le32(b, s) == 12 {
            le16(b, s + 4) as int
        } else {
            s32(b, s + 4)
        }) <= max_dimension(),
        visible_height_of(mode, abs_int(if le32(b, s) == 12 {
            le16(b, s + 6) as int
        } else {
            s32(b, s + 8)
        })) <= max_dimension(),
        (depth_ok(if le32(b, s) == 12 {
            le16(b, s + 10) as int
        } else {
            le16(b, s + 14)
        }) || (p.unsupported == UnsupportedOptions::Rgb2 && le32(b, s) >= 40
            && le16(b, s + 14) == 2)),
        (if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 16)
        }) <= 3,
        !((if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 16)
        }) == 1 && (if le32(b, s) == 12 {
            le16(b, s + 10) as int
        } else {
            le16(b, s + 14)
        }) != 8),
        !((if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 16)
        }) == 2 && (if le32(b, s) == 12 {
            le16(b, s + 10) as int
        } else {
            le16(b, s + 14)
        }) != 4),
        !((if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 16)
        }) == 3 && (if le32(b, s) == 12 {
            le16(b, s + 10) as int
        } else {
            le16(b, s + 14)
        }) != 16 && (if le32(b, s) == 12 {
            le16(b, s + 10) as int
        } else {
            le16(b, s + 14)
        }) != 32),
        !((if le32(b, s) == 12 {
            le16(b, s + 6) as int
        } else {
            s32(b, s + 8)
        }) < 0 && ((if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 16)
        }) == 1 || (if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 16)
        }) == 2)),
        !(is_ico(mode) && ((if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 16)
        }) == 1 || (if le32(b, s) == 12 || le32(b, s) == 16 {
            0
        } else {
            le32(b, s + 16)
        }) == 2)),
    ensures
        r2r5_policy(b, mode, s, p) == Err(Error::OutputTooLarge),
{
    reveal(r2r5_policy);
    reveal(hsize_ok_policy);
    reveal(hsize_ok);
    reveal(pow2);
    reveal(is_ico);
    reveal(visible_height_of);
    reveal(gdk_high_bound);
    reveal(abs_int);
}

/// The `masks`-option validity verdict for a `RawFields`' four raw words.
pub open spec fn masks_ok_policy_rf(b: Seq<u8>, s: int, rf: RawFields, o: MasksOptions) -> bool {
    let ms = masks_after_policy(
        raw_masks_policy(b, s, rf).0,
        raw_masks_policy(b, s, rf).1,
        raw_masks_policy(b, s, rf).2,
        raw_masks_policy(b, s, rf).3,
        rf.depth,
        o,
    );
    masks_ok_policy(ms.0, ms.1, ms.2, ms.3, rf.depth, o)
}

/// The `high_count`-selected table count for a `RawFields`.
pub open spec fn table_count_of_policy_rf(rf: RawFields, o: HighCountOptions) -> int {
    table_count_of_policy(rf, o)
}

/// The `masks`-selected four words for a `RawFields`.
pub open spec fn masks_of_policy_rf(b: Seq<u8>, s: int, rf: RawFields, o: MasksOptions)
    -> (u32, u32, u32, u32) {
    if rf.compression != 3 {
        (0u32, 0u32, 0u32, 0u32)
    } else {
        let ms = masks_after_policy(
        raw_masks_policy(b, s, rf).0,
        raw_masks_policy(b, s, rf).1,
        raw_masks_policy(b, s, rf).2,
        raw_masks_policy(b, s, rf).3,
        rf.depth,
        o,
    );
    (ms.0, ms.1, ms.2, ms.3)
    }
}

/// The `r6r7_policy` guard chain, as one predicate over a validated
/// `RawFields`. `r6r7_policy` returns exactly the error this names, or its
/// `Ok` record.
pub open spec fn r6r7_policy_err(b: Seq<u8>, s: int, file_offset: int, is_bmp: bool, rf: RawFields, p: Policy)
    -> Option<Error> {
    if policy_masks_missing(b, s, rf, p) {
        Some(Error::TruncatedMasks)
    } else if rf.compression == 3 && !masks_ok_policy_rf(b, s, rf, p.masks) {
        Some(Error::InvalidBitfields)
    } else if is_bmp && p.offset == OffsetOptions::Declared && file_offset < palette_start_of(rf)
        + table_count_of_policy_rf(rf, p.high_count) * palette_stride_of(rf) {
        Some(Error::InvalidPixelOffset)
    } else if (p.header_bytes == crate::dib::HeaderBytesOptions::Full || table_count_of_policy_rf(rf, p.high_count) != 0) && b.len() < palette_start_of(rf) + table_count_of_policy_rf(rf, p.high_count)
        * palette_stride_of(rf) {
        Some(Error::TruncatedPalette)
    } else {
        None
    }
}

/// `r6r7_policy` returns exactly `r6r7_policy_err`'s error, or the record;
/// stated as the single equality the two-phase lemma needs.
pub proof fn lemma_r6r7_policy_exact(
    b: Seq<u8>, mode: Mode, s: int, file_offset: int, is_bmp: bool, rf: RawFields, p: Policy,
)
    requires
        b.len() >= s + 4,
    ensures
        match r6r7_policy_err(b, s, file_offset, is_bmp, rf, p) {
            Some(e) => r6r7_policy(b, mode, s, file_offset, is_bmp, rf, p) == Err(e),
            None => r6r7_policy(b, mode, s, file_offset, is_bmp, rf, p)
                == Ok(r6r7_result_policy(
                b,
                mode,
                s,
                file_offset,
                is_bmp,
                rf,
                masks_of_policy_rf(b, s, rf, p.masks),
                table_count_of_policy_rf(rf, p.high_count),
            )),
        },
{
    reveal(r6r7_policy);
    reveal(r6r7_result_policy);
    reveal(masks_after_policy);
    reveal(masks_ok_policy);
    reveal(masks_ok_policy_rf);
    reveal(masks_of_policy_rf);
    reveal(raw_masks_policy);
    reveal(table_count_of_policy_rf);
    reveal(table_count_of_policy);
    reveal(palette_start_of);
    reveal(palette_stride_of);
    reveal(r6r7_policy_err);
}

/// The four raw BITFIELDS mask words a `RawFields` names, before the `masks`
/// option's substitution.
pub open spec fn raw_masks_policy(b: Seq<u8>, s: int, rf: RawFields) -> (u32, u32, u32, u32) {
    if rf.compression == 3 {
        if rf.hsize == 40 || rf.hsize == 64 {
            // External masks start at the validated raw field's header end.
            (as_u32(le32(b, rf.pos0)), as_u32(le32(b, rf.pos0 + 4)),
                as_u32(le32(b, rf.pos0 + 8)), 0u32)
        } else if rf.hsize >= 56 {
            // Internal masks: R,G,B,A at `s+40 .. s+56`.
            (as_u32(le32(b, s + 40)), as_u32(le32(b, s + 44)), as_u32(le32(b, s + 48)),
                as_u32(le32(b, s + 52)))
        } else {
            (as_u32(le32(b, s + 40)), as_u32(le32(b, s + 44)), as_u32(le32(b, s + 48)), 0u32)
        }
    } else {
        (0u32, 0u32, 0u32, 0u32)
    }
}

/// `r6r7_policy` success: given the executable's concrete phase facts, the
/// model returns the same record.
pub proof fn lemma_r6r7_policy_record(
    b: Seq<u8>,
    mode: Mode,
    s0: int,
    file_offset: int,
    is_bmp: bool,
    rf: RawFields,
    p: Policy,
    m: MHRecord,
    ms0: u32,
    ms1: u32,
    ms2: u32,
    ms3: u32,
    tc: int,
)
    requires
        !(policy_masks_missing(b, s0, rf, p)),
        r6r7_policy_err(b, s0, file_offset, is_bmp, rf, p) == None,
        (ms0, ms1, ms2, ms3) == masks_of_policy_rf(b, s0, rf, p.masks),
        tc == table_count_of_policy_rf(rf, p.high_count),
        m == r6r7_result_policy(b, mode, s0, file_offset, is_bmp, rf, (
            ms0,
            ms1,
            ms2,
            ms3,
        ), tc),
    ensures
        r6r7_policy(b, mode, s0, file_offset, is_bmp, rf, p) == Ok(m),
{
    reveal(r6r7_policy);
    reveal(r6r7_result_policy);
    reveal(masks_of_policy_rf);
    reveal(table_count_of_policy_rf);
}

pub proof fn lemma_r6r7_policy_offset_err(
    b: Seq<u8>, mode: Mode, s: int, file_offset: int, is_bmp: bool, rf: RawFields, p: Policy,
    pe: int, pe_ico: bool,
)
    requires
        !(policy_masks_missing(b, s, rf, p)),
        !(rf.compression == 3 && !masks_ok_policy_rf(b, s, rf, p.masks)),
        palette_start_of(rf) + table_count_of_policy_rf(rf, p.high_count) * palette_stride_of(rf)
            == pe,
        pe_ico == (is_bmp && p.offset == OffsetOptions::Declared),
        pe_ico,
        file_offset < pe,
    ensures
        r6r7_policy(b, mode, s, file_offset, is_bmp, rf, p) == Err(Error::InvalidPixelOffset),
{
    reveal(r6r7_policy);
    reveal(palette_start_of);
    reveal(palette_stride_of);
    reveal(table_count_of_policy_rf);
}

pub proof fn lemma_r6r7_policy_palette_err(
    b: Seq<u8>, mode: Mode, s: int, file_offset: int, is_bmp: bool, rf: RawFields, p: Policy,
    pe: int,
)
    requires
        !(policy_masks_missing(b, s, rf, p)),
        !(rf.compression == 3 && !masks_ok_policy_rf(b, s, rf, p.masks)),
        palette_start_of(rf) + table_count_of_policy_rf(rf, p.high_count) * palette_stride_of(rf)
            == pe,
        !(is_bmp && p.offset == OffsetOptions::Declared && file_offset < pe),
        b.len() < pe,
        p.header_bytes == crate::dib::HeaderBytesOptions::Full || table_count_of_policy_rf(rf, p.high_count) != 0,
    ensures
        r6r7_policy(b, mode, s, file_offset, is_bmp, rf, p) == Err(Error::TruncatedPalette),
{
    reveal(r6r7_policy);
    reveal(palette_start_of);
    reveal(palette_stride_of);
    reveal(table_count_of_policy_rf);
}

pub proof fn lemma_r6r7_policy_ok(
    b: Seq<u8>, mode: Mode, s: int, file_offset: int, is_bmp: bool, rf: RawFields, p: Policy,
    m: MHRecord,
)
    requires
        !(policy_masks_missing(b, s, rf, p)),
        !(rf.compression == 3 && !masks_ok_policy_rf(b, s, rf, p.masks)),
        !(is_bmp && p.offset == OffsetOptions::Declared && file_offset < palette_start_of(rf)
            + table_count_of_policy_rf(rf, p.high_count) * palette_stride_of(rf)),
        (p.header_bytes == crate::dib::HeaderBytesOptions::ParsedFields && table_count_of_policy_rf(rf, p.high_count) == 0) || b.len() >= palette_start_of(rf) + table_count_of_policy_rf(rf, p.high_count)
            * palette_stride_of(rf),
        m == r6r7_result_policy(
            b,
            mode,
            s,
            file_offset,
            is_bmp,
            rf,
            masks_of_policy_rf(b, s, rf, p.masks),
            table_count_of_policy_rf(rf, p.high_count),
        ),
    ensures
        r6r7_policy(b, mode, s, file_offset, is_bmp, rf, p) == Ok(m),
{
    reveal(r6r7_policy);
    reveal(r6r7_result_policy);
    reveal(palette_start_of);
    reveal(palette_stride_of);
    reveal(table_count_of_policy_rf);
}

/// `r2r5_policy`'s early guards as a lemma taking the executable's concrete/// `r2r5_policy`'s early guards as a lemma taking the executable's concrete
/// facts: the size test, the header-size test and the header availability test
/// have all passed, so the model reaches its R3 `planes` test with the
/// executable's own `planes` value.
pub proof fn lemma_r2r5_policy_planes_err(b: Seq<u8>, mode: Mode, s: int, p: Policy)
    requires
        b.len() >= s + 4,
        hsize_ok_policy(le32(b, s), p.headers),
        b.len() >= s + header_bytes_needed(le32(b, s), p),
        p.planes == PlanesOptions::Must1,
        (if le32(b, s) == 12 {
            le16(b, s + 8)
        } else {
            le16(b, s + 12)
        }) != 1,
    ensures
        r2r5_policy(b, mode, s, p) == Err(Error::InvalidPlanes),
{
    reveal(r2r5_policy);
    reveal(hsize_ok_policy);
    reveal(hsize_ok);
}

/// `r2r5_policy`'s R3 dimension guard: the header fields are as read, and the
/// executable's dimension test has failed.
pub proof fn lemma_r2r5_policy_dims_err(b: Seq<u8>, mode: Mode, s: int, p: Policy)
    requires
        b.len() >= s + 4,
        hsize_ok_policy(le32(b, s), p.headers),
        b.len() >= s + header_bytes_needed(le32(b, s), p),
        !(p.planes == PlanesOptions::Must1 && (if le32(b, s) == 12 {
            le16(b, s + 8)
        } else {
            le16(b, s + 12)
        }) != 1),
        (if le32(b, s) == 12 {
            le16(b, s + 4) as int
        } else {
            s32(b, s + 4)
        }) <= 0 || (if le32(b, s) == 12 {
            le16(b, s + 6) as int
        } else {
            s32(b, s + 8)
        }) == 0 || (if le32(b, s) == 12 {
            le16(b, s + 6) as int
        } else {
            s32(b, s + 8)
        }) == s32_min(),
    ensures
        r2r5_policy(b, mode, s, p) == Err(Error::InvalidDimensions),
{
    reveal(r2r5_policy);
    reveal(hsize_ok_policy);
    reveal(hsize_ok);
}

/// `r2r5_policy`'s R3 ICO stored-height guard.
pub proof fn lemma_r2r5_policy_ico_err(b: Seq<u8>, mode: Mode, s: int, p: Policy)
    requires
        b.len() >= s + 4,
        hsize_ok_policy(le32(b, s), p.headers),
        b.len() >= s + header_bytes_needed(le32(b, s), p),
        !(p.planes == PlanesOptions::Must1 && (if le32(b, s) == 12 {
            le16(b, s + 8)
        } else {
            le16(b, s + 12)
        }) != 1),
        !((if le32(b, s) == 12 {
            le16(b, s + 4) as int
        } else {
            s32(b, s + 4)
        }) <= 0 || (if le32(b, s) == 12 {
            le16(b, s + 6) as int
        } else {
            s32(b, s + 8)
        }) == 0 || (if le32(b, s) == 12 {
            le16(b, s + 6) as int
        } else {
            s32(b, s + 8)
        }) == s32_min()),
        is_ico(mode),
        (if le32(b, s) == 12 {
            le16(b, s + 6) as int
        } else {
            s32(b, s + 8)
        }) < 0 || (if le32(b, s) == 12 {
            le16(b, s + 6) as int
        } else {
            s32(b, s + 8)
        }) % 2 != 0,
    ensures
        r2r5_policy(b, mode, s, p) == Err(Error::InvalidDimensions),
{
    reveal(r2r5_policy);
    reveal(hsize_ok_policy);
    reveal(hsize_ok);
    reveal(is_ico);
    reveal(visible_height_of);
}

/// R6..R7 with the `high_count`, `masks` and `offset` options, over a `RawFields`
/// validated by `r2r5_policy`. The record is `r6r7_result_policy`'s: the same
/// `r6r7_result` with the `masks = DefaultRGB` substitution applied to the four
/// mask words, and the palette table size selected by `high_count`.
pub open spec fn r6r7_policy(
    b: Seq<u8>,
    mode: Mode,
    s: int,
    file_offset: int,
    is_bmp: bool,
    rf: RawFields,
    p: Policy,
) -> Result<MHRecord, Error> {
    let compression: int = rf.compression;
    let depth: int = rf.depth;
    let pos0: int = rf.pos0;
    let external_masks: bool = rf.hsize == 40 || rf.hsize == 64;
    if policy_masks_missing(b, s, rf, p) {
        Err(Error::TruncatedMasks)
    } else if compression == 3 && !masks_ok_policy_rf(b, s, rf, p.masks) {
        Err(Error::InvalidBitfields)
    } else {
        let pos: int = if compression == 3 && external_masks {
            pos0 + 12
        } else {
            pos0
        };
        let tc: int = table_count_of_policy_rf(rf, p.high_count);
        let palette_end: int = palette_start_of(rf) + tc * palette_stride_of(rf);
        if is_bmp && p.offset == OffsetOptions::Declared && file_offset < palette_end {
            Err(Error::InvalidPixelOffset)
        } else if (p.header_bytes == crate::dib::HeaderBytesOptions::Full || tc != 0) && b.len() < palette_end {
            Err(Error::TruncatedPalette)
        } else {
            Ok(
                r6r7_result_policy(
                    b,
                    mode,
                    s,
                    file_offset,
                    is_bmp,
                    rf,
                    masks_of_policy_rf(b, s, rf, p.masks),
                    tc,
                ),
            )
        }
    }
}

/// The record `r6r7_policy` builds: `r6r7_result` with the four post-substitution
/// mask words and the option-selected `table_count`.
pub open spec fn r6r7_result_policy(
    b: Seq<u8>,
    mode: Mode,
    s: int,
    file_offset: int,
    is_bmp: bool,
    rf: RawFields,
    ms: (u32, u32, u32, u32),
    table_count: int,
) -> MHRecord {
    let depth: int = rf.depth;
    let used: int = rf.used;
    let pal_stride: int = if rf.core {
        3
    } else {
        4
    };
    let pos: int = if rf.compression == 3 && (rf.hsize == 40 || rf.hsize == 64) {
        rf.pos0 + 12
    } else {
        rf.pos0
    };
    let count: int = if depth <= 8 {
        if used != 0 {
            used
        } else {
            pow2(depth)
        }
    } else {
        0
    };
    let palette_end: int = pos + table_count * pal_stride;
    MHRecord {
        mode,
        s,
        hsize: rf.hsize,
        width: rf.width,
        height: rf.height,
        stored_height: rf.stored,
        planes: rf.planes,
        depth,
        compression: rf.compression,
        declared_image: rf.declared,
        color_used: used,
        count,
        table_count,
        palette_stride: pal_stride,
        palette_start: pos,
        palette_end,
        pixel_start: if is_bmp {
            file_offset
        } else {
            palette_end
        },
        mask_r: ms.0 as int,
        mask_g: ms.1 as int,
        mask_b: ms.2 as int,
        mask_a: ms.3 as int,
        has_masks: rf.compression == 3,
        core: rf.core,
        is_bmp,
        top_down: rf.stored < 0,
        n: rf.width * rf.height,
        stride: if rf.compression == 0 || rf.compression == 3 {
            stride_of(rf.width, depth)
        } else {
            0
        },
    }
}

/// R2..R7 with every option: `r2r5_policy`, then `r6r7_policy`.
/// `r2r7_policy`'s success record, given the `RawFields` and the phase facts.
pub open spec fn r2r7_policy(
    b: Seq<u8>,
    mode: Mode,
    s: int,
    file_offset: int,
    is_bmp: bool,
    p: Policy,
) -> Result<MHRecord, Error> {
    match r2r5_policy(b, mode, s, p) {
        Err(e) => Err(e),
        Ok(rf) => r6r7_policy(b, mode, s, file_offset, is_bmp, rf, p),
    }
}

/// `MH_policy(b, bm, p)`: the R1–R7 header model with every option of this
/// milestone. With `p = Strict` each option sits at its 1.1 value, so the whole
/// chain reduces to `r1_prefix`/`r2r5`/`r6r7` and `MH_policy(Strict)` is `MH`.
pub open spec fn mh_policy(b: Seq<u8>, bm: BmpMode, p: Policy) -> Result<MHRecord, Error> {
    let mode: Mode = match bm {
        BmpMode::BmpFile => Mode::BmpFile,
        BmpMode::BareDib => Mode::BareDib,
    };
    if b.len() > max_input() {
        Err(Error::FileTooLarge)
    } else if bm == BmpMode::BmpFile {
        match r1_prefix_policy(b, p) {
            Err(e) => Err(e),
            Ok(q) => r2r7_policy(b, mode, q.0, q.1, true, p),
        }
    } else {
        r2r7_policy(b, mode, 0, 0, false, p)
    }
}

/// R12's permitted `biSizeImage` values, with the `image_size` option.
/// `Declared` is `image_size_ok` verbatim; `Ignore` skips the equality.
/// **Neither value uses the declaration to bound a read**: `x_len` is always
/// supplied by the caller from the cursor the pixel phase actually reached,
/// never from `biSizeImage`.
pub open spec fn image_size_ok_policy(
    declared: int,
    x_len: int,
    a_bytes: int,
    ico: bool,
    p: Policy,
) -> bool {
    match p.image_size {
        MetadataOptions::Declared => image_size_ok(declared, x_len, a_bytes, ico),
        MetadataOptions::Ignore => true,
    }
}

/// R12's rule lifted to a record, with the `image_size` option.
pub open spec fn mp_image_size_ok_policy(m: MHRecord, x_len: int, p: Policy) -> bool {
    let ico = is_ico(m.mode);
    let a = if ico {
        and_bytes(m.width, m.height)
    } else {
        0
    };
    image_size_ok_policy(m.declared_image, x_len, a, ico, p)
}

/// T7/T9/R19 with the `image_size` option: `mp_mask_phase` with its R12 test
/// replaced. The AND-plane availability, the ICO whole-image XOR-alpha policy,
/// and the `BmpFile`/`BareDib` suffix rules are unchanged.
pub open spec fn mp_mask_phase_policy(
    b: Seq<u8>,
    m: MHRecord,
    pos: int,
    x_len: int,
    out: Seq<u8>,
    p: Policy,
) -> Result<(int, Seq<u8>), MpErr> {
    let a = and_bytes(m.width, m.height);
    if !mp_image_size_ok_policy(m, x_len, p) {
        Err(MpErr::Api(Error::InvalidImageSize))
    } else if is_ico(m.mode) {
        if pos + a > b.len() {
            Err(MpErr::Api(Error::TruncatedAndMask))
        } else {
            let end = pos + a;
            if exact_tail(m.mode) && end != b.len() {
                Err(MpErr::Api(Error::TrailingBytes))
            } else {
                Ok((end, ma_output(b, m, pos, out, explicit_alpha(m))))
            }
        }
    } else if m.mode == Mode::BmpFile {
        Ok((pos, out))
    } else if pos < b.len() || (pos > b.len() && p.short_pixels == crate::dib::ShortPixelsOptions::Reject) {
        Err(MpErr::Api(Error::TrailingBytes))
    } else {
        Ok((pos, out))
    }
}

/// The executable `finish_mask_policy` outcome against `mp_mask_phase_policy`,
/// in the executable's own shape: `Ok(())` iff the phase is `Ok`, with the
/// buffer equal to the phase's `Ok` payload, and an error naming the phase's
/// exact API error.
pub open spec fn mask_done_policy_eq(
    b: Seq<u8>,
    m: MHRecord,
    pos: int,
    out0: Seq<u8>,
    out1: Seq<u8>,
    r: Result<(), Error>,
    p: Policy,
) -> bool {
    match mp_mask_phase_policy(b, m, pos, pos - offset_effective(m, p), out0, p) {
        Ok((_end, img)) => r == Ok(()) && out1 == img,
        Err(MpErr::Api(e)) => r == Err(e),
        Err(MpErr::Outside) => false,
    }
}

/// T9 (phase bridge): the executable phase outcome equals
/// `mp_mask_phase_policy`'s exact result on both exits.
pub proof fn lemma_mp_policy_phase(
    b: Seq<u8>, m: MHRecord, pos: int, out0: Seq<u8>, out1: Seq<u8>, r: Result<(), Error>, p: Policy,
)
    requires
        m.mode == Mode::BmpFile || m.mode == Mode::BareDib,
        mask_done_policy_eq(b, m, pos, out0, out1, r, p),
    ensures
        match mp_mask_phase_policy(b, m, pos, pos - offset_effective(m, p), out0, p) {
            Ok((_end, img)) => r == Ok(()) && out1 == img,
            Err(MpErr::Api(e)) => r == Err(e),
            Err(MpErr::Outside) => false,
        },
        !(mp_mask_phase_policy(b, m, pos, pos - offset_effective(m, p), out0, p) is Err
            && mp_mask_phase_policy(b, m, pos, pos - offset_effective(m, p), out0, p)->Err_0 is Outside)
            || r != Ok(()),
{
    reveal(mask_done_policy_eq);
    reveal(mp_mask_phase_policy);
}

pub open spec fn popcount_byte(v: u8) -> u32 {
    ((v & 1) as u32 + ((v >> 1) & 1) as u32 + ((v >> 2) & 1) as u32 + ((v >> 3) & 1) as u32
        + ((v >> 4) & 1) as u32 + ((v >> 5) & 1) as u32 + ((v >> 6) & 1) as u32 + ((v >> 7) & 1) as u32) as u32
}

pub open spec fn popcount_word(v: u32) -> u32 {
    (popcount_byte(v as u8) + popcount_byte((v >> 8) as u8)
        + popcount_byte((v >> 16) as u8) + popcount_byte((v >> 24) as u8)) as u32
}

pub open spec fn channel_scaled(word: u32, mask: u32, scaling: crate::dib::ScalingOptions) -> u8 {
    if mask == 0 { 255u8 } else {
        let shift = vstd::std_specs::bits::u32_trailing_zeros(mask);
        let compact = mask >> shift;
        let width = (32u32 - vstd::std_specs::bits::u32_leading_zeros(compact)) as u32;
        match scaling {
            crate::dib::ScalingOptions::Round => channel_of(word, mask),
            crate::dib::ScalingOptions::Image => if width <= 8 { channel_of(word, mask) }
                else { (((word & mask) >> shift) >> (width - 8)) as u8 },
            crate::dib::ScalingOptions::Gdk => {
                let count = popcount_word(mask);
                let n = if count > 8 { 8u32 } else { count };
                let skip = if count > 8 { (count - 8) as u32 } else { 0u32 };
                let value = (word & mask) as u64 >> (shift + skip);
                ((value << (8 - n)) | (if n >= 4 { value >> (2 * n - 8) } else { 0u64 })) as u8
            },
        }
    }
}

pub open spec fn raw_scaled_conv(b: Seq<u8>, m: MHRecord, pos: int, x: int,
    scaling: crate::dib::ScalingOptions, opaque: bool) -> Seq<u8> {
    if m.compression == 3 || m.depth == 16 {
        let base = pos + x * m.depth / 8;
        let word = if m.depth == 16 { le16(b, base) as u32 } else { le32(b, base) as u32 };
        if m.compression == 3 {
            seq![channel_scaled(word, m.mask_r as u32, scaling), channel_scaled(word, m.mask_g as u32, scaling),
                channel_scaled(word, m.mask_b as u32, scaling), if opaque && m.depth == 16 { 255u8 } else { channel_scaled(word, m.mask_a as u32, scaling) }]
        } else {
            seq![channel_scaled(word, 0x7c00u32, scaling), channel_scaled(word, 0x3e0u32, scaling),
                channel_scaled(word, 0x1fu32, scaling), 255u8]
        }
    } else { raw_conv(b, m, pos, x) }
}

pub open spec fn raw_legacy(p: Policy) -> bool {
    p.short_pixels == crate::dib::ShortPixelsOptions::Reject
        && p.scaling == crate::dib::ScalingOptions::Round
        && p.bitfield16_alpha == crate::dib::Bitfield16AlphaOptions::Mask
        && p.palette_index == crate::dib::PaletteIndexOptions::Reject
}

pub type RawPolicyFull = (Result<int, Error>, Seq<Write>, bool);

// ZeroFill advances the logical cursor without reading any partial row.
#[verifier::opaque]
pub open spec fn raw_zero_full(b: Seq<u8>, m: MHRecord, pos: int, x: int, y: int, zero: bool, black: bool, scaling: crate::dib::ScalingOptions, opaque: bool)
    -> RawPolicyFull
    decreases (m.height - y) as nat, (m.width - x) as nat,
{
    if y >= m.height { (Ok(pos), Seq::<Write>::empty(), false) }
    else if x >= m.width { raw_zero_full(b, m, pos + m.stride, 0, y + 1, zero, black, scaling, opaque) }
    else if pos + m.stride > b.len() && !zero {
        (Err(Error::TruncatedPixels), Seq::<Write>::empty(), false)
    } else if pos + m.stride > b.len() {
        let next = raw_zero_full(b, m, pos + m.stride, 0, y + 1, zero, black, scaling, opaque);
        (next.0, next.1, true)
    } else if !raw_index_ok(b, m, pos, x) && !black {
        (Err(Error::PaletteIndexOutOfRange), Seq::<Write>::empty(), false)
    } else {
        let next = raw_zero_full(b, m, pos, x + 1, y, zero, black, scaling, opaque);
        (next.0, seq![Write { cell: cell_of(m, x, y), px: if raw_index_ok(b, m, pos, x) { raw_scaled_conv(b, m, pos, x, scaling, opaque) }
            else { seq![0u8, 0u8, 0u8, 255u8] } }] + next.1, !raw_index_ok(b, m, pos, x) || next.2)
    }
}

pub open spec fn raw_zero_fill(m: MHRecord, zero: bool) -> Seq<u8> {
    if !zero || m.depth == 32 { zero_bytes(4 * m.n) } else { opaque_fill(4 * m.n) }
}

pub open spec fn raw_zero_out(b: Seq<u8>, m: MHRecord, p0: int, zero: bool, black: bool, scaling: crate::dib::ScalingOptions, opaque: bool) -> (Result<(int, Seq<u8>), Error>, bool) {
    let full = raw_zero_full(b, m, p0, 0, 0, zero, black, scaling, opaque);
    (match full.0 {
        Err(e) => Err(e),
        Ok(pos) => Ok((pos, bytes_replay(raw_zero_fill(m, zero), full.1))),
    }, full.2)
}

/// `MP_policy` for a raw/BITFIELDS record: `MR`'s raw transition, then the
/// policy mask phase. Identical to `mp_raw` but for the phase's option.
pub open spec fn mp_raw_policy(b: Seq<u8>, m: MHRecord, p: Policy) -> Result<(Seq<u8>, int), MpErr> {
    if !raw_legacy(p) {
        match raw_zero_out(b, m, offset_effective(m, p), p.short_pixels == crate::dib::ShortPixelsOptions::ZeroFill, p.palette_index == crate::dib::PaletteIndexOptions::Black, p.scaling, p.bitfield16_alpha == crate::dib::Bitfield16AlphaOptions::Opaque).0 {
            Err(e) => Err(MpErr::Api(e)),
            Ok((pc, out)) => match mp_mask_phase_policy(b, m, pc, pc - offset_effective(m, p), out, p) {
                Ok((end, out2)) => Ok((out2, end)), Err(e) => Err(e),
            },
        }
    } else {
    match mr_raw_out(b, m, offset_effective(m, p)) {
        Err((RawErr::Api(e), _pc, _xc, _yc)) => Err(MpErr::Api(e)),
        Err((RawErr::Outside, _pc, _xc, _yc)) => Err(MpErr::Outside),
        Ok((pc, out)) => match mp_mask_phase_policy(b, m, pc, pc - offset_effective(m, p), out, p)
        {
            Ok((end, out2)) => Ok((out2, end)),
            Err(e) => Err(e),
        },
    }
}
}

pub open spec fn packet_index(b: Seq<u8>, depth: int, absolute: bool, value: int, j: int) -> int {
    if absolute { abs_index(b, depth, value, j) } else { run_index(value, depth, j) }
}

pub open spec fn black_packet_writes(b: Seq<u8>, m: MHRecord, absolute: bool, value: int,
    x: int, y: int, q: int) -> Seq<Write> {
    Seq::new(q as nat, |j: int| {
        let ix = packet_index(b, m.depth, absolute, value, j);
        Write { cell: cell_of(m, x + j, y),
            px: if ix >= m.count { seq![0u8, 0u8, 0u8, 255u8] }
                else { pal_entry(b, m.palette_start, m.palette_stride, ix) } }
    })
}

pub open spec fn black_packet_event(b: Seq<u8>, m: MHRecord, absolute: bool, value: int, q: int) -> bool {
    exists|j: int| 0 <= j < q && packet_index(b, m.depth, absolute, value, j) >= m.count
}

pub proof fn lemma_packet_write_lengths(b: Seq<u8>, m: MHRecord, x: int, y: int, value: int, j: int, q: int)
    requires 0 <= j <= q,
    ensures run_writes(b, m, x, y, value, j, q).len() == q - j,
        abs_writes(b, m, x, y, value, j, q).len() == q - j,
    decreases q - j,
{
    reveal(run_writes); reveal(abs_writes);
    if j < q { lemma_packet_write_lengths(b, m, x, y, value, j + 1, q); }
}

pub open spec fn rle_initial(b: Seq<u8>, m: MHRecord, fill: crate::dib::RleFillOptions) -> Seq<u8> {
    match fill {
        crate::dib::RleFillOptions::Palette0 => pal_zero_bytes(b, m),
        crate::dib::RleFillOptions::OpaqueBlack => opaque_fill(4 * m.n),
        crate::dib::RleFillOptions::TransparentBlack => zero_bytes(4 * m.n),
    }
}

pub open spec fn rle_legacy(p: Policy) -> bool {
    p.rle_overrun == crate::dib::RleOverrunOptions::Reject
        && p.rle_eol == crate::dib::RleEolOptions::OnlyFinal
        && p.rle_delta == crate::dib::RleDeltaOptions::Inside
        && p.rle_fill == crate::dib::RleFillOptions::Palette0
        && p.rle_eob == crate::dib::RleEobOptions::Required
        && p.palette_index == crate::dib::PaletteIndexOptions::Reject
}

// Policy RLE command transition. Clipped indices never enter a painter.
pub type ClipStep = (Result<bool, Error>, int, int, int, Seq<Write>, bool);

pub open spec fn clip_paint_count(width: int, x: int, q: int) -> int {
    if q > width - x { width - x } else { q }
}

#[verifier::opaque]
pub open spec fn rle_clip_step(b: Seq<u8>, m: MHRecord, p: int, x: int, y: int,
    overrun: crate::dib::RleOverrunOptions, eol: crate::dib::RleEolOptions, eob: crate::dib::RleEobOptions, black: bool, delta: crate::dib::RleDeltaOptions) -> ClipStep {
    let empty = Seq::<Write>::empty();
    if eob == crate::dib::RleEobOptions::AtInputEnd && p >= 0
        && (y >= m.height || p + 2 > b.len()) {
        (Ok(true), p, x, y, empty, false)
    } else if p < 0 || p + 2 > b.len() {
        (Err(Error::RleTruncated), p, x, y, empty, false)
    } else {
        let q = b[p] as int;
        let code = b[p + 1] as int;
        if q != 0 {
            if y >= m.height || (x + q > m.width && (overrun == crate::dib::RleOverrunOptions::Reject
                || (overrun == crate::dib::RleOverrunOptions::Image && m.depth == 4))) {
                (Err(Error::RleBounds), p + 2, x, y, empty, false)
            } else {
                let count = clip_paint_count(m.width, x, q);
                let good = run_good(m, code, count, 0);
                if black {
                    (Ok(false), p + 2, x + count, y, black_packet_writes(b, m, false, code, x, y, count),
                        x + q > m.width || black_packet_event(b, m, false, code, count))
                } else if good == count {
                    (Ok(false), p + 2, x + count, y,
                        run_writes(b, m, x, y, code, 0, count), x + q > m.width)
                } else {
                    (Err(Error::PaletteIndexOutOfRange), p + 2, x, y,
                        run_writes(b, m, x, y, code, 0, good), x + q > m.width)
                }
            }
        } else if code == 1 {
            (Ok(true), p + 2, x, y, empty, false)
        } else if code == 0 {
            if y >= m.height { (Err(Error::RleBounds), p + 2, x, y, empty, false) }
            else { (Ok(eol == crate::dib::RleEolOptions::StopAtHeight && y + 1 == m.height), p + 2, 0, y + 1, empty, false) }
        } else if code == 2 {
            if p + 4 > b.len() { (Err(Error::RleTruncated), p + 2, x, y, empty, false) }
            else if x + b[p + 2] as int >= m.width || y + b[p + 3] as int >= m.height {
                if delta == crate::dib::RleDeltaOptions::Inside {
                    (Err(Error::RleBounds), p + 4, x, y, empty, false)
                } else {
                    let dx = x + b[p + 2] as int;
                    let dy = y + b[p + 3] as int;
                    (Ok(false), p + 4, if dx > m.width { m.width } else { dx },
                        if dy > m.height { m.height } else { dy }, empty, dx > m.width || dy > m.height)
                }
            } else {
                (Ok(false), p + 4, x + b[p + 2] as int, y + b[p + 3] as int, empty, false)
            }
        } else {
            if y >= m.height || (x + code > m.width && overrun != crate::dib::RleOverrunOptions::Clip) {
                (Err(Error::RleBounds), p + 2, x, y, empty, false)
            } else if p + 2 + rle_padded(code, m.depth) > b.len() {
                (Err(Error::RleTruncated), p + 2, x, y, empty, x + code > m.width)
            } else {
                let count = clip_paint_count(m.width, x, code);
                let good = abs_good(b, m, p + 2, count, 0);
                if black {
                    (Ok(false), p + 2 + rle_padded(code, m.depth), x + count, y,
                        black_packet_writes(b, m, true, p + 2, x, y, count),
                        x + code > m.width || black_packet_event(b, m, true, p + 2, count))
                } else if good == count {
                    (Ok(false), p + 2 + rle_padded(code, m.depth), x + count, y,
                        abs_writes(b, m, x, y, p + 2, 0, count), x + code > m.width)
                } else {
                    (Err(Error::PaletteIndexOutOfRange), p + 2 + rle_padded(code, m.depth), x, y,
                        abs_writes(b, m, x, y, p + 2, 0, good), x + code > m.width)
                }
            }
        }
    }
}

pub type ClipFull = (Result<int, Error>, Seq<Write>, bool);

#[verifier::opaque]
pub open spec fn rle_clip_full(b: Seq<u8>, m: MHRecord, p: int, x: int, y: int,
    overrun: crate::dib::RleOverrunOptions, eol: crate::dib::RleEolOptions, eob: crate::dib::RleEobOptions, black: bool, delta: crate::dib::RleDeltaOptions) -> ClipFull
    decreases b.len() - p,
{
    let s = rle_clip_step(b, m, p, x, y, overrun, eol, eob, black, delta);
    match s.0 {
        Err(e) => (Err(e), s.4, s.5),
        Ok(true) => (Ok(s.1), s.4, s.5),
        Ok(false) => {
            if s.1 > p && s.1 <= b.len() {
                let tail = rle_clip_full(b, m, s.1, s.2, s.3, overrun, eol, eob, black, delta);
                (tail.0, s.4 + tail.1, s.5 || tail.2)
            } else { (Err(Error::RleTruncated), s.4, s.5) }
        },
    }
}

pub open spec fn rle_clip_out(b: Seq<u8>, m: MHRecord, overrun: crate::dib::RleOverrunOptions, eol: crate::dib::RleEolOptions, eob: crate::dib::RleEobOptions, black: bool, delta: crate::dib::RleDeltaOptions, fill: crate::dib::RleFillOptions)
    -> (Result<(int, Seq<u8>), Error>, bool) {
    let r = rle_clip_full(b, m, m.pixel_start, 0, 0, overrun, eol, eob, black, delta);
    (match r.0 {
        Err(e) => Err(e),
        Ok(pos) => Ok((pos, bytes_replay(rle_initial(b, m, fill), r.1))),
    }, r.2 || (r.0 is Ok && fill != crate::dib::RleFillOptions::Palette0 && r.1.len() < m.n))
}

/// `MP_policy` for an RLE4/RLE8 record: `MR`'s RLE transition, then the policy
/// mask phase. Identical to `mp_rle` but for the phase's option. RLE pixels
/// start at the record's own `pixel_start` (the declared `bfOffBits` in
/// `BmpFile`, the palette end in `BareDib`); the `Gdk` blank repair of an
/// invalid indexed offset is applied in `mp_policy` before this call.
pub open spec fn mp_rle_policy(b: Seq<u8>, m: MHRecord, p: Policy) -> Result<(Seq<u8>, int), MpErr> {
    if !rle_legacy(p) {
        match rle_clip_out(b, m, p.rle_overrun, p.rle_eol, p.rle_eob, p.palette_index == crate::dib::PaletteIndexOptions::Black, p.rle_delta, p.rle_fill).0 {
            Err(e) => Err(MpErr::Api(e)),
            Ok((pc, out)) => match mp_mask_phase_policy(b, m, pc, pc - m.pixel_start, out, p) {
                Ok((end, out2)) => Ok((out2, end)),
                Err(e) => Err(e),
            },
        }
    } else {
    match mr_out(b, m, m.pixel_start, 0, 0) {
        Err((RleErr::Api(e), _pc, _xc, _yc)) => Err(MpErr::Api(e)),
        Err((RleErr::Outside, _pc, _xc, _yc)) => Err(MpErr::Outside),
        Ok((pc, out)) => match mp_mask_phase_policy(b, m, pc, pc - m.pixel_start, out, p) {
            Ok((end, out2)) => Ok((out2, end)),
            Err(e) => Err(e),
        },
    }
}
}

/// R1/R7 `offset`: the effective raw pixel start for a record and option under
/// the two BMP modes. `Declared` and `AllowAny` start at the declared
/// `bfOffBits`. `Gdk` clamps a BI_RGB (`compression == 0`) offset below the
/// palette end up to the palette end; BITFIELDS is left at the declared offset
/// (POLICY-SPEC: "Gdk does not clamp this BITFIELDS value to metadata end").
pub open spec fn bitfield_pixel_start(meta: bool, masks: bool, ps: int, pe: int) -> int {
    if meta && masks { pe } else { ps }
}

pub open spec fn offset_effective(m: MHRecord, p: Policy) -> int {
    let start = bitfield_pixel_start(
        p.bitfield_offset == BitfieldOffsetOptions::MetadataEnd,
        m.has_masks, m.pixel_start, m.palette_end);
    match p.offset {
        OffsetOptions::Declared => start,
        OffsetOptions::AllowAny => start,
        OffsetOptions::Gdk => if m.compression == 0 && start < m.palette_end {
            m.palette_end
        } else {
            start
        },
    }
}

/// R1/R12 Gdk blank: a transparent-black or opaque-black initialized image, the
/// two initialized fills POLICY-SPEC's `offset = Gdk` repair returns.
pub open spec fn opaque_fill(k: int) -> Seq<u8> {
    Seq::new(k as nat, |i: int| if i % 4 == 3 {
        255u8
    } else {
        0u8
    })
}

/// R1/R7 `offset = Gdk` blank guard: an indexed record (depth <= 8) whose
/// declared offset is below the palette end or past EOF yields initialized
/// blank output instead of pixels.
/// A short indexed palette followed by a BMP pixel-offset gap selects blank output.
pub open spec fn indexed_gap_blank(m: MHRecord, p: Policy) -> bool {
    p.short_indexed_gap == ShortIndexedGapOptions::Blank && m.is_bmp && m.depth <= 8
        && 0 < m.color_used < pow2(m.depth) && m.pixel_start > m.palette_end
}

/// Both initialized-blank header policies use the same opaque/RLE-transparent fill.
pub open spec fn gdk_blank(b: Seq<u8>, m: MHRecord, p: Policy) -> bool {
    (m.is_bmp && p.offset == OffsetOptions::Gdk && m.depth <= 8 && (m.pixel_start < m.palette_end
        || m.pixel_start > b.len())) || indexed_gap_blank(m, p)
}

/// R1/R7 `offset = Gdk` BI_RGB past-EOF guard: a high-depth BI_RGB offset past
/// EOF rejects with `InvalidPixelOffset` (POLICY-SPEC).
pub open spec fn gdk_rgb_reject(b: Seq<u8>, m: MHRecord, p: Policy) -> bool {
    m.is_bmp && p.offset == OffsetOptions::Gdk && m.depth > 8 && m.compression == 0 && m.pixel_start > b.len()
}

/// R1/R6 `recovered_so_far`: whether the header phase performed a
/// data-affecting recovery (POLICY-SPEC's closed list): an invalid declared
/// pixel offset accepted rather than rejected, a Gdk indexed offset repaired to
/// a blank image or a Gdk BI_RGB offset clamped to the palette end, or a
/// `masks = DefaultRGB` substitution (any RGB mask zero).
/// The original RGB words, not the substituted output masks, determine fallback.
pub open spec fn masks_defaulted(b: Seq<u8>, m: MHRecord, p: Policy) -> bool {
    let q = m.s + (if m.hsize == 40 || m.hsize == 64 { m.hsize } else { 40 });
    p.masks == MasksOptions::DefaultRGB && m.compression == 3
        && 0 <= q && q + 12 <= b.len()
        && (le32(b, q) == 0 || le32(b, q + 4) == 0 || le32(b, q + 8) == 0)
}

pub open spec fn mh_recovered_so_far(b: Seq<u8>, m: MHRecord, p: Policy) -> bool {
    ||| (m.is_bmp && gdk_blank(b, m, p))
    ||| (m.is_bmp && p.offset == OffsetOptions::Gdk && m.compression == 0
        && m.pixel_start < m.palette_end)
    ||| (m.is_bmp && p.offset != OffsetOptions::Declared
        && (m.pixel_start < 18 || m.pixel_start > b.len()))
    ||| (m.is_bmp && p.bitfield_offset == BitfieldOffsetOptions::MetadataEnd
        && m.has_masks && m.pixel_start != m.palette_end)
    ||| masks_defaulted(b, m, p)
    ||| (p.header_bytes == crate::dib::HeaderBytesOptions::ParsedFields && m.s + m.hsize > b.len())
}

// A partial header carries only dimensions validated before R6/R7 failed.
pub open spec fn mh_partial(b: Seq<u8>, bm: BmpMode, p: Policy) -> Option<RawFields> {
    if p.incomplete_metadata != crate::dib::IncompleteMetadataOptions::Blank { None }
    else {
        let s = if bm == BmpMode::BmpFile { 14int } else { 0int };
        match mh_policy(b, bm, p) {
            Err(e) => if e == Error::TruncatedMasks || e == Error::TruncatedPalette {
                match r2r5_policy(b, crate::dib::bmp_mode_to_mode(bm), s, p) {
                    Ok(rf) => if e == Error::TruncatedPalette || rf.hsize == 40 || rf.hsize == 64 {
                        Some(rf)
                    } else { None },
                    Err(_) => None,
                }
            } else { None },
            Ok(_) => None,
        }
    }
}

pub enum PolicyHeader {
    Complete(MHRecord),
    Partial(RawFields),
}

pub open spec fn mh_selected(b: Seq<u8>, bm: BmpMode, p: Policy) -> Result<PolicyHeader, Error> {
    match mh_partial(b, bm, p) {
        Some(rf) => Ok(PolicyHeader::Partial(rf)),
        None => match mh_policy(b, bm, p) {
            Ok(m) => Ok(PolicyHeader::Complete(m)), Err(e) => Err(e),
        },
    }
}

pub open spec fn partial_fill(rf: RawFields) -> Seq<u8> {
    if rf.depth == 32 || rf.compression == 1 || rf.compression == 2 {
        zero_bytes(4 * (rf.width * rf.height))
    } else { opaque_fill(4 * (rf.width * rf.height)) }
}

/// The gdk-pixbuf 2.42.12 `has_alpha` rule on a validated header's
/// `(biBitCount, biCompression)`: true exactly when the depth is 32 or the
/// compression is RLE8 (1) or RLE4 (2). It reads no pixel value, so it is a
/// function of the header record alone. This is the same condition `partial_fill`
/// and the `offset = Gdk` blank branch above select the transparent fill with,
/// restated over the raw integers so a returned image field can be tied to it.
pub open spec fn rle_or_32bpp(depth: int, compression: int) -> bool {
    depth == 32 || compression == 1 || compression == 2
}

/// `rle_or_32bpp` on the record the policy-free header model `MH` returns. An
/// error yields `false`, a value no successful `Decoded` ever carries.
pub open spec fn mh_rle_or_32bpp(b: Seq<u8>, mode: Mode) -> bool {
    match mh(b, mode) {
        Ok(m) => rle_or_32bpp(m.depth, m.compression),
        Err(_) => false,
    }
}

/// `rle_or_32bpp` on the profiled header selection: the partial `RawFields`
/// record when `MH_policy` fails as `TruncatedMasks`/`TruncatedPalette` under
/// `incomplete_metadata = Blank` (R2..R5 validated `depth` and `compression`
/// before that failure, so both are exact), else the complete `MH_policy`
/// record. An error yields `false`. Kept a separate predicate so the two unfolds
/// are named lemmas (`lemma_mh_policy_rle_or_32bpp_partial`/`_complete`): a proof
/// that needs the model tie can call the lemma, and a proof that only needs a
/// `Decoded` field can use the cheap concrete-header form on the executable
/// `Header` instead, so the large header/pixel functions do not unfold
/// `mh_policy` merely to type-check an `ensures`.
pub open spec fn mh_policy_rle_or_32bpp(b: Seq<u8>, bm: BmpMode, p: Policy) -> bool {
    match mh_partial(b, bm, p) {
        Some(rf) => rle_or_32bpp(rf.depth, rf.compression),
        None => match mh_policy(b, bm, p) {
            Ok(m) => rle_or_32bpp(m.depth, m.compression),
            Err(_) => false,
        },
    }
}

/// A successful `MH_policy` record forces `MH_policy`'s partial fallback to
/// `None`: `mh_partial` returns `Some` only on the two recoverable failures.
pub proof fn lemma_mh_partial_none(b: Seq<u8>, bm: BmpMode, p: Policy, m: MHRecord)
    requires crate::model::mh_policy(b, bm, p) == Ok(m),
    ensures crate::model::mh_partial(b, bm, p) is None,
{
    reveal(mh_partial);
}

/// The partial branch of the profiled indicator: on a validated partial record it
/// is `rle_or_32bpp` of that record's `depth`/`compression`.
pub proof fn lemma_mh_policy_rle_or_32bpp_partial(b: Seq<u8>, bm: BmpMode, p: Policy, rf: RawFields)
    requires crate::model::mh_partial(b, bm, p) == Some(rf),
    ensures crate::model::mh_policy_rle_or_32bpp(b, bm, p) == rle_or_32bpp(rf.depth, rf.compression),
{
    reveal(mh_policy_rle_or_32bpp);
}

/// The complete branch of the profiled indicator: on a validated `MH_policy`
/// record it is `rle_or_32bpp` of that record's `depth`/`compression`.
pub proof fn lemma_mh_policy_rle_or_32bpp_complete(b: Seq<u8>, bm: BmpMode, p: Policy, m: MHRecord)
    requires
        crate::model::mh_partial(b, bm, p) is None,
        crate::model::mh_policy(b, bm, p) == Ok(m),
    ensures crate::model::mh_policy_rle_or_32bpp(b, bm, p) == rle_or_32bpp(m.depth, m.compression),
{
    reveal(mh_policy_rle_or_32bpp);
}

/// The whole-image `recovered` flag for a policy result: `true` exactly on a
/// successful decode whose header phase reported a recovery. An error never
/// reports the flag (POLICY-SPEC: it is a property of the returned image).
pub open spec fn pixel_recovered(b: Seq<u8>, m: MHRecord, p: Policy) -> bool {
    !gdk_blank(b, m, p) && (
        ((m.compression == 1 || m.compression == 2) && !rle_legacy(p)
            && rle_clip_out(b, m, p.rle_overrun, p.rle_eol, p.rle_eob, p.palette_index == crate::dib::PaletteIndexOptions::Black, p.rle_delta, p.rle_fill).1)
        || ((m.compression == 0 || m.compression == 3)
            && !raw_legacy(p)
            && raw_zero_out(b, m, offset_effective(m, p), p.short_pixels == crate::dib::ShortPixelsOptions::ZeroFill, p.palette_index == crate::dib::PaletteIndexOptions::Black, p.scaling, p.bitfield16_alpha == crate::dib::Bitfield16AlphaOptions::Opaque).1))
}

pub open spec fn recovered_policy(b: Seq<u8>, bm: BmpMode, p: Policy) -> bool {
    match mp_policy(b, bm, p) {
        Err(_) => false,
        Ok(_) => if mh_partial(b, bm, p) is Some { true } else { match mh_policy(b, bm, p) {
            Ok(m) => mh_recovered_so_far(b, m, p) || pixel_recovered(b, m, p),
            Err(_) => false,
        } },
    }
}

/// `MP_policy(b, bm, p)`: the total policy-indexed reference model for the two
/// BMP modes. It calls `MH_policy`, checks required palette bytes, applies the
/// `offset = Gdk` repairs, then performs the R8 fill, the R9 raw or R10–R11 RLE
/// transition, and the policy R12 phase, returning the exact `Ok(w, h, rgba)`
/// or the exact first `Err(e)`.
pub open spec fn mp_policy(
    b: Seq<u8>,
    bm: BmpMode,
    p: Policy,
) -> Result<(int, int, Seq<u8>), Error> {
    if let Some(rf) = mh_partial(b, bm, p) {
        Ok((rf.width, rf.height, partial_fill(rf)))
    } else {
    match mh_policy(b, bm, p) {
        Err(e) => Err(e),
        Ok(m) => {
            if (p.header_bytes == crate::dib::HeaderBytesOptions::Full || m.table_count != 0) && m.palette_start + m.table_count * m.palette_stride > b.len() {
                Err(Error::TruncatedPalette)
            } else if gdk_blank(b, m, p) {
                // POLICY-SPEC `offset = Gdk`: an invalid indexed offset selects
                // initialized blank output (transparent for RLE, else opaque).
                if m.compression == 1 || m.compression == 2 {
                    Ok((m.width, m.height, zero_bytes(4 * m.n)))
                } else {
                    Ok((m.width, m.height, opaque_fill(4 * m.n)))
                }
            } else if gdk_rgb_reject(b, m, p) {
                Err(Error::InvalidPixelOffset)
            } else if m.compression == 0 || m.compression == 3 {
                mp_decoded(m, mp_raw_policy(b, m, p))
            } else {
                mp_decoded(m, mp_rle_policy(b, m, p))
            }
        },
    }
}

}

// --- The spec-level Strict bridge (THEOREMS-POLICY-DELTA: `MH_1_1 :=
// MH(Strict)`, `MP_1_1 := MP(Strict)`). Each lemma is a definitional reduction
// of the policy-indexed function at `crate::dib::strict_policy()`, so no existing
// T-statement loses anything and no policy function is proved against an
// independently written strict decoder.

/// `r1_prefix_policy` at `Strict` is `r1_prefix` verbatim.
pub proof fn lemma_r1_prefix_policy_strict(b: Seq<u8>)
    ensures
        r1_prefix_policy(b, crate::dib::strict_policy()) == r1_prefix(b),
{
    reveal(r1_prefix);
    reveal(r1_prefix_policy);
    reveal(crate::dib::strict_policy);
}

/// `hsize_ok_policy` at `All1_1` is `hsize_ok` verbatim.
pub proof fn lemma_hsize_ok_policy_strict(h: int)
    ensures
        hsize_ok_policy(h, HeadersOptions::All1_1) == hsize_ok(h),
{
    reveal(hsize_ok_policy);
    reveal(hsize_ok);
}

/// `table_count_of_policy` at `BmpTable` is `table_count_of` verbatim.
pub proof fn lemma_table_count_of_policy_strict(rf: RawFields)
    ensures
        table_count_of_policy(rf, HighCountOptions::BmpTable) == table_count_of(rf),
{
    reveal(table_count_of_policy);
    reveal(table_count_of);
}

/// `masks_ok_policy` at `Reject` is `masks_ok` verbatim.
pub proof fn lemma_masks_ok_policy_reject(m0: u32, m1: u32, m2: u32, m3: u32, bits: int)
    ensures
        masks_ok_policy(m0, m1, m2, m3, bits, MasksOptions::Reject) == masks_ok(m0, m1, m2, m3, bits),
{
    reveal(masks_ok_policy);
    reveal(masks_ok);
}

/// `masks_after_policy` at `Reject`/`Independent` leaves the masks unchanged.
pub proof fn lemma_masks_after_policy_strict(m0: u32, m1: u32, m2: u32, m3: u32, bits: int)
    ensures
        masks_after_policy(m0, m1, m2, m3, bits, MasksOptions::Reject) == (m0, m1, m2, m3),
        masks_after_policy(m0, m1, m2, m3, bits, MasksOptions::Independent) == (m0, m1, m2, m3),
{
    reveal(masks_after_policy);
}

/// `r2r5_policy` at `Strict` is `r2r5` verbatim.
pub proof fn lemma_r2r5_policy_strict(b: Seq<u8>, mode: Mode, s: int)
    ensures
        r2r5_policy(b, mode, s, crate::dib::strict_policy()) == r2r5(b, mode, s),
{
    reveal(r2r5_policy);
    reveal(r2r5);
    reveal(hsize_ok_policy);
    reveal(hsize_ok);
    reveal(crate::dib::strict_policy);
    reveal(crate::model::pow2);
    lemma_hsize_ok_policy_strict(le32(b, s));
}

/// `r6r7_policy` at `Strict` is `r6r7` verbatim, given the same validated
/// `RawFields`. The mask words pass through unchanged, the table count is
/// `BmpTable`'s and the offset check is `Declared`'s.
pub proof fn lemma_r6r7_policy_strict(
    b: Seq<u8>, mode: Mode, s: int, file_offset: int, is_bmp: bool, rf: RawFields,
)
    ensures
        r6r7_policy(b, mode, s, file_offset, is_bmp, rf, crate::dib::strict_policy())
            == r6r7(b, mode, s, file_offset, is_bmp, rf),
{
    reveal(r6r7_policy);
    reveal(r6r7);
    reveal(crate::model::hsize_ok);
    reveal(r6r7_result);
    reveal(r6r7_result_policy);
    reveal(masks_ok_policy_rf);
    reveal(masks_of_policy_rf);
    reveal(masks_after_policy);
    reveal(masks_ok_policy);
    reveal(masks_ok);
    reveal(raw_masks_policy);
    reveal(table_count_of_policy_rf);
    reveal(table_count_of_policy);
    reveal(table_count_of);
    reveal(palette_start_of);
    reveal(palette_stride_of);
    reveal(palette_length_of);
    reveal(palette_end_of);
    reveal(crate::dib::strict_policy);
    reveal(contiguous);
    reveal(within_bits);
    reveal(low_of);
    reveal(pow2);
    assert(table_count_of_policy(rf, HighCountOptions::BmpTable) == table_count_of(rf));
    assert(masks_after_policy(
        raw_masks_policy(b, s, rf).0,
        raw_masks_policy(b, s, rf).1,
        raw_masks_policy(b, s, rf).2,
        raw_masks_policy(b, s, rf).3,
        rf.depth,
        MasksOptions::Reject,
    ) == (raw_masks_policy(b, s, rf).0, raw_masks_policy(b, s, rf).1, raw_masks_policy(b, s, rf).2,
        raw_masks_policy(b, s, rf).3));
    if rf.compression == 3 {
        assert(masks_ok_policy(raw_masks_policy(b, s, rf).0, raw_masks_policy(b, s, rf).1,
            raw_masks_policy(b, s, rf).2, raw_masks_policy(b, s, rf).3, rf.depth,
            MasksOptions::Reject) == masks_ok(raw_masks_policy(b, s, rf).0, raw_masks_policy(b, s,
            rf).1, raw_masks_policy(b, s, rf).2, raw_masks_policy(b, s, rf).3, rf.depth));
    }
    assert(palette_start_of(rf) + table_count_of_policy_rf(rf, HighCountOptions::BmpTable)
        * palette_stride_of(rf) == palette_end_of(rf));
    assert(r6r7_policy(b, mode, s, file_offset, is_bmp, rf, crate::dib::strict_policy()) == r6r7(
        b, mode, s, file_offset, is_bmp, rf)) by {
        reveal(r6r7_policy);
        reveal(r6r7);
        reveal(masks_ok_policy_rf);
        reveal(masks_of_policy_rf);
        reveal(raw_masks_policy);
        reveal(r6r7_result);
        reveal(r6r7_result_policy);
        reveal(palette_start_of);
        reveal(palette_stride_of);
        reveal(table_count_of_policy_rf);
        reveal(table_count_of_policy);
        reveal(table_count_of);
        reveal(palette_end_of);
        reveal(palette_length_of);
    }
}

/// `r2r7_policy` at `Strict` is `r2r7` verbatim.
pub proof fn lemma_r2r7_policy_strict(b: Seq<u8>, mode: Mode, s: int, file_offset: int, is_bmp: bool)
    ensures
        r2r7_policy(b, mode, s, file_offset, is_bmp, crate::dib::strict_policy())
            == r2r7(b, mode, s, file_offset, is_bmp),
{
    reveal(r2r7_policy);
    reveal(r2r7);
    reveal(r2r5_policy);
    reveal(r2r5);
    reveal(r6r7_policy);
    lemma_r2r5_policy_strict(b, mode, s);
    if r2r5(b, mode, s) is Ok {
        let rf = r2r5(b, mode, s)->Ok_0;
        lemma_r6r7_policy_strict(b, mode, s, file_offset, is_bmp, rf);
    }
}

/// `offset_effective` at `Declared` is `m.pixel_start`.
pub proof fn lemma_offset_effective_declared(m: MHRecord)
    ensures
        offset_effective(m, crate::dib::strict_policy()) == m.pixel_start,
{
    reveal(offset_effective);
    reveal(crate::dib::strict_policy);
}

/// `MH_policy(Strict)` is `MH(_)` for both BMP modes.
pub proof fn lemma_mh_policy_strict(b: Seq<u8>, bm: BmpMode)
    ensures
        mh_policy(b, bm, crate::dib::strict_policy()) == mh(b, crate::dib::bmp_mode_to_mode(bm)),
{
    reveal(mh_policy);
    reveal(mh);
    reveal(r1_prefix_policy);
    reveal(r1_prefix);
    reveal(r2r7_policy);
    reveal(crate::dib::strict_policy);
    reveal(crate::dib::bmp_mode_to_mode);
    if b.len() <= max_input() {
        lemma_r1_prefix_policy_strict(b);
        if bm == BmpMode::BmpFile {
            if r1_prefix(b) is Ok {
                let q = r1_prefix(b)->Ok_0;
                lemma_r2r7_policy_strict(b, Mode::BmpFile, q.0, q.1, true);
            }
        } else {
            lemma_r2r7_policy_strict(b, Mode::BareDib, 0, 0, false);
        }
    }
}

/// `image_size_ok_policy` at `Strict` is `image_size_ok` verbatim.
pub proof fn lemma_image_size_ok_policy_strict(declared: int, x_len: int, a_bytes: int, ico: bool)
    ensures
        image_size_ok_policy(declared, x_len, a_bytes, ico, crate::dib::strict_policy())
            == image_size_ok(declared, x_len, a_bytes, ico),
{
    reveal(image_size_ok_policy);
    reveal(image_size_ok);
    reveal(crate::dib::strict_policy);
}

/// `mp_image_size_ok_policy` at `Strict` is `mp_image_size_ok`.
pub proof fn lemma_mp_image_size_ok_policy_strict(m: MHRecord, x_len: int)
    ensures
        mp_image_size_ok_policy(m, x_len, crate::dib::strict_policy())
            == mp_image_size_ok(m, x_len),
{
    reveal(mp_image_size_ok_policy);
    reveal(mp_image_size_ok);
    reveal(crate::dib::strict_policy);
    lemma_image_size_ok_policy_strict(
        m.declared_image,
        x_len,
        if is_ico(m.mode) {
            and_bytes(m.width, m.height)
        } else {
            0
        },
        is_ico(m.mode),
    );
}

/// `mp_mask_phase_policy` at `Strict` is `mp_mask_phase`.
pub proof fn lemma_mp_mask_phase_policy_strict(
    b: Seq<u8>, m: MHRecord, pos: int, x_len: int, out: Seq<u8>,
)
    ensures
        mp_mask_phase_policy(b, m, pos, x_len, out, crate::dib::strict_policy())
            == mp_mask_phase(b, m, pos, x_len, out),
{
    reveal(mp_mask_phase_policy);
    reveal(mp_mask_phase);
    reveal(crate::dib::strict_policy);
    lemma_mp_image_size_ok_policy_strict(m, x_len);
}

/// `mp_raw_policy` at `Strict` is `mp_raw`.
pub proof fn lemma_mp_raw_policy_strict(b: Seq<u8>, m: MHRecord)
    ensures
        mp_raw_policy(b, m, crate::dib::strict_policy()) == mp_raw(b, m),
{
    reveal(mp_raw_policy);
    reveal(mp_raw);
    reveal(offset_effective);
    reveal(crate::dib::strict_policy);
    match mr_raw_out(b, m, m.pixel_start) {
        Ok((pc, out)) => {
            lemma_mp_mask_phase_policy_strict(b, m, pc, pc - m.pixel_start, out);
        },
        Err(_) => {},
    }
}

/// `mh_recovered_so_far` at `Strict` is `false`: no option is at a
/// recovery-producing value.
pub proof fn lemma_mh_recovered_so_far_strict(b: Seq<u8>, m: MHRecord)
    ensures
        !mh_recovered_so_far(b, m, crate::dib::strict_policy()),
{
    reveal(mh_recovered_so_far);
    reveal(gdk_blank);
    reveal(crate::dib::strict_policy);
}

/// A raw record whose pixel cursor is past EOF is `TruncatedPixels` at once:
/// the first row's whole stride is unavailable.
pub proof fn lemma_raw_truncated(b: Seq<u8>, m: MHRecord, pos0: int)
    requires
        1 <= m.height,
        1 <= m.width,
        0 <= m.stride,
        pos0 > b.len(),
    ensures
        raw_full(b, m, pos0, 0, 0) == Err((
            RawErr::Api(Error::TruncatedPixels),
            pos0,
            0,
            0,
            Seq::<Write>::empty(),
        )),
        mr_raw_out(b, m, pos0) == Err((RawErr::Api(Error::TruncatedPixels), pos0, 0, 0)),
{
    reveal(raw_full);
    reveal(mr_raw_out);
}

/// An RLE record whose pixel cursor is past EOF is `RleTruncated` at once: the
/// first two-byte command is unavailable.
pub proof fn lemma_rle_truncated(b: Seq<u8>, m: MHRecord, p0: int)
    requires
        p0 > b.len(),
    ensures
        mr_out(b, m, p0, 0, 0) == Err((RleErr::Api(Error::RleTruncated), p0, 0, 0)),
{
    reveal(mr_full);
    reveal(mr_out);
}

/// A raw record whose effective pixel start is past EOF: `MP_policy` is
/// `TruncatedPixels` (after the two Gdk guards are ruled out).
pub proof fn lemma_mp_policy_raw_trunc(b: Seq<u8>, bm: BmpMode, p: Policy, m: MHRecord)
    requires
        raw_legacy(p),
        mh_policy(b, bm, p) == Ok(m),
        (p.header_bytes == crate::dib::HeaderBytesOptions::ParsedFields && m.table_count == 0) || m.palette_start + m.table_count * m.palette_stride <= b.len(),
        m.compression == 0 || m.compression == 3,
        !gdk_blank(b, m, p),
        !gdk_rgb_reject(b, m, p),
        offset_effective(m, p) > b.len(),
    ensures
        mp_policy(b, bm, p) == Err(Error::TruncatedPixels),
{
    reveal(mp_policy);
    reveal(mp_raw_policy);
    reveal(mp_decoded);
    reveal(offset_effective);
    reveal(gdk_blank);
    reveal(gdk_rgb_reject);
    reveal(raw_full);
    reveal(mr_raw_out);
    assert(mr_raw_out(b, m, offset_effective(m, p)) is Err);
    assert((mr_raw_out(b, m, offset_effective(m, p))->Err_0).0
        == RawErr::Api(Error::TruncatedPixels));
}

/// An RLE record whose pixel cursor is past EOF: `MP_policy` is `RleTruncated`
/// (after the Gdk indexed-blank guard is ruled out).
pub proof fn lemma_mp_policy_rle_trunc(b: Seq<u8>, bm: BmpMode, p: Policy, m: MHRecord)
    requires
        p.rle_eob == crate::dib::RleEobOptions::Required,
        mh_policy(b, bm, p) == Ok(m),
        (p.header_bytes == crate::dib::HeaderBytesOptions::ParsedFields && m.table_count == 0) || m.palette_start + m.table_count * m.palette_stride <= b.len(),
        m.compression == 1 || m.compression == 2,
        !gdk_blank(b, m, p),
        m.pixel_start > b.len(),
    ensures
        mp_policy(b, bm, p) == Err(Error::RleTruncated),
{
    reveal(rle_clip_out);
    reveal(rle_clip_full);
    reveal(rle_clip_step);
    reveal(mp_policy);
    reveal(mp_rle_policy);
    reveal(mp_decoded);
    reveal(gdk_blank);
    reveal(mr_full);
    reveal(mr_out);
    assert(mr_out(b, m, m.pixel_start, 0, 0) is Err);
    assert((mr_out(b, m, m.pixel_start, 0, 0)->Err_0).0 == RleErr::Api(Error::RleTruncated));
}

/// A Gdk BI_RGB high-depth offset past EOF: `MP_policy` is `InvalidPixelOffset`.
pub proof fn lemma_mp_policy_gdk_rgb(b: Seq<u8>, bm: BmpMode, p: Policy, m: MHRecord)
    requires
        mh_policy(b, bm, p) == Ok(m),
        (p.header_bytes == crate::dib::HeaderBytesOptions::ParsedFields && m.table_count == 0) || m.palette_start + m.table_count * m.palette_stride <= b.len(),
        gdk_rgb_reject(b, m, p),
    ensures
        mp_policy(b, bm, p) == Err(Error::InvalidPixelOffset),
{
    reveal(mp_policy);
    reveal(gdk_rgb_reject);
    reveal(gdk_blank);
}

/// `mp_rle_policy` at `Strict` is `mp_rle` (the RLE start is the record's own
/// `pixel_start`, which equals the palette end for an RLE record).
pub proof fn lemma_mp_rle_policy_strict(b: Seq<u8>, m: MHRecord)
    ensures
        mp_rle_policy(b, m, crate::dib::strict_policy()) == mp_rle(b, m),
{
    reveal(mp_rle_policy);
    reveal(mp_rle);
    reveal(crate::dib::strict_policy);
    match mr_out(b, m, m.pixel_start, 0, 0) {
        Ok((pc, out)) => {
            lemma_mp_mask_phase_policy_strict(b, m, pc, pc - m.pixel_start, out);
        },
        Err(_) => {},
    }
}

/// The spec-level Strict bridge: for every byte sequence and both BMP modes,
/// `MP_policy(·, Strict)` is `MP(·, mode)` — equal first errors, equal
/// dimensions and equal RGBA bytes. Using the two `MP`s' own functions, as the
/// delta specifies, rather than proving an equality between independently
/// written decoders.
pub proof fn lemma_mp_policy_strict(b: Seq<u8>, bm: BmpMode)
    ensures
        mp_policy(b, bm, crate::dib::strict_policy()) == mp(b, crate::dib::bmp_mode_to_mode(bm)),
{
    reveal(mp_policy);
    reveal(mp);
    reveal(mp_decoded);
    reveal(mh_policy);
    reveal(mh);
    reveal(crate::dib::strict_policy);
    reveal(gdk_blank);
    reveal(gdk_rgb_reject);
    reveal(offset_effective);
    lemma_mh_policy_strict(b, bm);
    match mh_policy(b, bm, crate::dib::strict_policy()) {
        Ok(m) => {
            lemma_mp_raw_policy_strict(b, m);
            assert(!gdk_blank(b, m, crate::dib::strict_policy()));
            assert(!gdk_rgb_reject(b, m, crate::dib::strict_policy()));
            lemma_mp_rle_policy_strict(b, m);
        },
        Err(_) => {},
    }
}

// --- P1 · the lemmas the profiled executable entry point discharges against.
// `MH_policy`/`MP_policy` reuse `MH`/`MP`'s own records and transitions, so the
// bridge to the strict model is a case on which option value changed.

/// T9 (phase bridge, raw path, success): the `Header`-level policy phase result
/// assembles `MP_policy`'s exact `Ok`, given the model's own header and the
/// caller's palette bound.
pub proof fn lemma_mp_policy_raw_ok(
    b: Seq<u8>, bm: BmpMode, p: Policy, m: MHRecord, pos: int, out0: Seq<u8>, out1: Seq<u8>,
)
    requires
        raw_legacy(p),
        mh_policy(b, bm, p) == Ok(m),
        (p.header_bytes == crate::dib::HeaderBytesOptions::ParsedFields && m.table_count == 0) || m.palette_start + m.table_count * m.palette_stride <= b.len(),
        m.compression == 0 || m.compression == 3,
        !gdk_blank(b, m, p),
        !gdk_rgb_reject(b, m, p),
        mr_raw_out(b, m, offset_effective(m, p)) == Ok((pos, out0)),
        mp_mask_phase_policy(b, m, pos, pos - offset_effective(m, p), out0, p)
            == Ok((pos, out1)),
    ensures
        mp_policy(b, bm, p) == Ok((m.width, m.height, out1)),
{
    lemma_mp_policy_raw_assemble(b, bm, p, m, pos, out0, out1, pos);
}

/// Any `MH_policy` error is the exact `MP_policy` error.
pub proof fn lemma_mp_policy_mh_err(b: Seq<u8>, bm: BmpMode, p: Policy, e: Error)
    requires
        mh_partial(b, bm, p) is None,
        mh_policy(b, bm, p) == Err(e),
    ensures
        mp_policy(b, bm, p) == Err(e),
{
    reveal(mp_policy);
}

/// T9/T10 (raw path, error): the raw transition's exact first error is
/// `MP_policy`'s.
pub proof fn lemma_mp_policy_raw_err(b: Seq<u8>, bm: BmpMode, p: Policy, m: MHRecord, e: Error)
    requires
        raw_legacy(p),
        mh_policy(b, bm, p) == Ok(m),
        (p.header_bytes == crate::dib::HeaderBytesOptions::ParsedFields && m.table_count == 0) || m.palette_start + m.table_count * m.palette_stride <= b.len(),
        m.compression == 0 || m.compression == 3,
        !gdk_blank(b, m, p),
        !gdk_rgb_reject(b, m, p),
        mr_raw_out(b, m, offset_effective(m, p)) is Err,
        (mr_raw_out(b, m, offset_effective(m, p))->Err_0).0 == RawErr::Api(e),
    ensures
        mp_policy(b, bm, p) == Err(e),
{
    reveal(mp_policy);
    reveal(mp_raw_policy);
    reveal(mp_decoded);
    reveal(gdk_blank);
    reveal(gdk_rgb_reject);
    assert(mp_raw_policy(b, m, p) == Err(MpErr::Api(e)));
    assert(mp_decoded(m, mp_raw_policy(b, m, p)) == Err(e));
}

/// T9/T10 (RLE path, error): the RLE transition's exact first error is
/// `MP_policy`'s.
pub proof fn lemma_mp_policy_rle_err(b: Seq<u8>, bm: BmpMode, p: Policy, m: MHRecord, e: Error)
    requires
        rle_legacy(p),
        mh_policy(b, bm, p) == Ok(m),
        (p.header_bytes == crate::dib::HeaderBytesOptions::ParsedFields && m.table_count == 0) || m.palette_start + m.table_count * m.palette_stride <= b.len(),
        m.compression == 1 || m.compression == 2,
        !gdk_blank(b, m, p),
        mr_out(b, m, m.pixel_start, 0, 0) is Err,
        (mr_out(b, m, m.pixel_start, 0, 0)->Err_0).0 == RleErr::Api(e),
    ensures
        mp_policy(b, bm, p) == Err(e),
{
    reveal(mp_policy);
    reveal(mp_rle_policy);
    reveal(mp_decoded);
    reveal(gdk_blank);
    assert(mp_rle_policy(b, m, p) == Err(MpErr::Api(e)));
    assert(mp_decoded(m, mp_rle_policy(b, m, p)) == Err(e));
}

/// T9/T10 (mask path, error, raw dispatch): the policy mask phase's exact error
/// is `MP_policy`'s.
pub proof fn lemma_mp_policy_phase_err_raw(
    b: Seq<u8>, bm: BmpMode, p: Policy, m: MHRecord, pos: int, out0: Seq<u8>, e: Error,
)
    requires
        raw_legacy(p),
        mh_policy(b, bm, p) == Ok(m),
        (p.header_bytes == crate::dib::HeaderBytesOptions::ParsedFields && m.table_count == 0) || m.palette_start + m.table_count * m.palette_stride <= b.len(),
        m.compression == 0 || m.compression == 3,
        !gdk_blank(b, m, p),
        !gdk_rgb_reject(b, m, p),
        mr_raw_out(b, m, offset_effective(m, p)) == Ok((pos, out0)),
        mp_mask_phase_policy(b, m, pos, pos - offset_effective(m, p), out0, p)
            == Err(MpErr::Api(e)),
    ensures
        mp_policy(b, bm, p) == Err(e),
{
    reveal(mp_policy);
    reveal(mp_raw_policy);
    reveal(mp_decoded);
    reveal(gdk_blank);
    reveal(gdk_rgb_reject);
    reveal(offset_effective);
    assert(mp_raw_policy(b, m, p) == Err(MpErr::Api(e)));
}

/// T9/T10 (mask path, error, RLE dispatch): the same for the RLE path.
pub proof fn lemma_mp_policy_phase_err_rle(
    b: Seq<u8>, bm: BmpMode, p: Policy, m: MHRecord, pos: int, out0: Seq<u8>, e: Error,
)
    requires
        rle_legacy(p),
        mh_policy(b, bm, p) == Ok(m),
        (p.header_bytes == crate::dib::HeaderBytesOptions::ParsedFields && m.table_count == 0) || m.palette_start + m.table_count * m.palette_stride <= b.len(),
        m.compression == 1 || m.compression == 2,
        !gdk_blank(b, m, p),
        mr_out(b, m, m.pixel_start, 0, 0) == Ok((pos, out0)),
        mp_mask_phase_policy(b, m, pos, pos - m.pixel_start, out0, p) == Err(MpErr::Api(e)),
    ensures
        mp_policy(b, bm, p) == Err(e),
{
    reveal(mp_policy);
    reveal(mp_rle_policy);
    reveal(mp_decoded);
    reveal(gdk_blank);
    assert(mp_rle_policy(b, m, p) == Err(MpErr::Api(e)));
}

/// T9 (raw path, success): the raw transition and the policy mask phase
/// assemble `MP_policy`'s exact `Ok`. The palette bound is supplied by the
/// caller (the executable's own header invariant).
pub proof fn lemma_mp_policy_raw_assemble(
    b: Seq<u8>,
    bm: BmpMode,
    p: Policy,
    m: MHRecord,
    pos: int,
    out0: Seq<u8>,
    out1: Seq<u8>,
    end: int,
)
    requires
        raw_legacy(p),
        mh_policy(b, bm, p) == Ok(m),
        (p.header_bytes == crate::dib::HeaderBytesOptions::ParsedFields && m.table_count == 0) || m.palette_start + m.table_count * m.palette_stride <= b.len(),
        m.compression == 0 || m.compression == 3,
        !gdk_blank(b, m, p),
        !gdk_rgb_reject(b, m, p),
        mr_raw_out(b, m, offset_effective(m, p)) == Ok((pos, out0)),
        mp_mask_phase_policy(b, m, pos, pos - offset_effective(m, p), out0, p)
            == Ok((end, out1)),
    ensures
        mp_policy(b, bm, p) == Ok((m.width, m.height, out1)),
{
    reveal(mp_policy);
    reveal(mp_raw_policy);
    reveal(mp_decoded);
    reveal(gdk_blank);
    reveal(gdk_rgb_reject);
    reveal(offset_effective);
    assert(mp_raw_policy(b, m, p) == Ok((out1, end)));
    assert(mp_decoded(m, mp_raw_policy(b, m, p)) == Ok((m.width, m.height, out1)));
}

/// T9 (RLE path, success): the same for the RLE path.
pub proof fn lemma_mp_policy_rle_assemble(
    b: Seq<u8>,
    bm: BmpMode,
    p: Policy,
    m: MHRecord,
    pos: int,
    out0: Seq<u8>,
    out1: Seq<u8>,
    end: int,
)
    requires
        rle_legacy(p),
        mh_policy(b, bm, p) == Ok(m),
        (p.header_bytes == crate::dib::HeaderBytesOptions::ParsedFields && m.table_count == 0) || m.palette_start + m.table_count * m.palette_stride <= b.len(),
        m.compression == 1 || m.compression == 2,
        !gdk_blank(b, m, p),
        mr_out(b, m, m.pixel_start, 0, 0) == Ok((pos, out0)),
        mp_mask_phase_policy(b, m, pos, pos - m.pixel_start, out0, p) == Ok((end, out1)),
    ensures
        mp_policy(b, bm, p) == Ok((m.width, m.height, out1)),
{
    reveal(mp_policy);
    reveal(mp_rle_policy);
    reveal(mp_decoded);
    reveal(gdk_blank);
    assert(mp_rle_policy(b, m, p) == Ok((out1, end)));
    assert(mp_decoded(m, mp_rle_policy(b, m, p)) == Ok((m.width, m.height, out1)));
}

/// T13/T9 for the profiled model: an `Ok` `MP_policy` result has the exact
/// `4*w*h` output length and the visibility bounds. The record is `MH_policy`'s,
/// whose bounds are `MH`'s (`lemma_mh_ok_bounds` applies to `MH_policy` too
/// because the two share the R2–R7 chain — proved here by transferring the
/// record's geometry rather than re-deriving it).
pub proof fn lemma_mp_policy_ok_shape(b: Seq<u8>, bm: BmpMode, p: Policy, w: int, h: int, s: Seq<u8>)
    requires
        mp_policy(b, bm, p) == Ok((w, h, s)),
    ensures
        1 <= w <= max_dimension(),
        1 <= h <= max_dimension(),
        s.len() == 4 * (w * h),
        4 * (w * h) <= max_output(),
{
    reveal(mp_policy);
    if let Some(rf) = mh_partial(b, bm, p) {
        reveal(mh_partial); reveal(partial_fill); reveal(zero_bytes); reveal(opaque_fill);
        let start = if bm == BmpMode::BmpFile { 14int } else { 0int };
        lemma_r2r5_policy_ok(b, crate::dib::bmp_mode_to_mode(bm), start, p, rf);
        return;
    }

    reveal(gdk_blank);
    reveal(gdk_rgb_reject);
    let m = choose|m: MHRecord| mh_policy(b, bm, p) == Ok(m);
    assert(mh_policy(b, bm, p) is Ok);
    assert(m.width == w);
    assert(m.height == h);
    lemma_mh_policy_bounds(b, bm, p, m);
    assert(m.n == w * h);
    assert(4 * (w * h) <= max_output());
    assert(1 <= w <= max_dimension());
    assert(1 <= h <= max_dimension());
    assert(s.len() == 4 * (w * h)) by {
        if gdk_blank(b, m, p) {
            reveal(zero_bytes);
            reveal(opaque_fill);
            if m.compression == 1 || m.compression == 2 {
                assert(s == zero_bytes(4 * m.n));
            } else {
                assert(s == opaque_fill(4 * m.n));
            }
        } else {
            reveal(mp_decoded);
            reveal(mp_raw_policy);
            reveal(mp_rle_policy);
            if (m.compression == 0 || m.compression == 3)
                && !raw_legacy(p) {
                reveal(raw_zero_out); reveal(raw_zero_fill);
                let ws = raw_zero_full(b, m, offset_effective(m, p), 0, 0, p.short_pixels == crate::dib::ShortPixelsOptions::ZeroFill, p.palette_index == crate::dib::PaletteIndexOptions::Black, p.scaling, p.bitfield16_alpha == crate::dib::Bitfield16AlphaOptions::Opaque).1;
                lemma_bytes_replay_len(raw_zero_fill(m, p.short_pixels == crate::dib::ShortPixelsOptions::ZeroFill), ws);
                match raw_zero_out(b, m, offset_effective(m, p), p.short_pixels == crate::dib::ShortPixelsOptions::ZeroFill, p.palette_index == crate::dib::PaletteIndexOptions::Black, p.scaling, p.bitfield16_alpha == crate::dib::Bitfield16AlphaOptions::Opaque).0 {
                    Ok((pc, out0)) => match mp_mask_phase_policy(b, m, pc, pc - offset_effective(m, p), out0, p) {
                        Ok((end, out2)) => { lemma_mp_mask_phase_policy_len(b, m, pc, pc - offset_effective(m, p), out0, p, end, out2); },
                        Err(_) => {},
                    },
                    Err(_) => {},
                }
            } else if m.compression == 0 || m.compression == 3 {
                match mr_raw_out(b, m, offset_effective(m, p)) {
                    Ok((pc, out0)) => {
                        lemma_mr_raw_out_len(b, m, offset_effective(m, p), pc, out0);
                        match mp_mask_phase_policy(b, m, pc, pc - offset_effective(m, p), out0, p) {
                            Ok((end, out2)) => {
                                lemma_mp_mask_phase_policy_len(b, m, pc,
                                    pc - offset_effective(m, p), out0, p, end, out2);
                                assert(s == out2);
                            },
                            Err(_) => {},
                        }
                    },
                    Err(_) => {},
                }
            } else if !rle_legacy(p) {
                reveal(rle_clip_out);
                let ws = rle_clip_full(b, m, m.pixel_start, 0, 0, p.rle_overrun, p.rle_eol, p.rle_eob, p.palette_index == crate::dib::PaletteIndexOptions::Black, p.rle_delta).1;
                lemma_bytes_replay_len(rle_initial(b, m, p.rle_fill), ws);
                reveal(rle_initial);
                reveal(pal_zero_bytes);
                match rle_clip_out(b, m, p.rle_overrun, p.rle_eol, p.rle_eob, p.palette_index == crate::dib::PaletteIndexOptions::Black, p.rle_delta, p.rle_fill).0 {
                    Ok((pc, out0)) => match mp_mask_phase_policy(b, m, pc, pc - m.pixel_start, out0, p) {
                        Ok((end, out2)) => { lemma_mp_mask_phase_policy_len(b, m, pc, pc - m.pixel_start, out0, p, end, out2); },
                        Err(_) => {},
                    },
                    Err(_) => {},
                }
            } else {
                match mr_out(b, m, m.pixel_start, 0, 0) {
                    Ok((pc, out0)) => {
                        lemma_mr_out_len(b, m, m.pixel_start, pc, out0);
                        match mp_mask_phase_policy(b, m, pc, pc - m.pixel_start, out0, p) {
                            Ok((end, out2)) => {
                                lemma_mp_mask_phase_policy_len(b, m, pc,
                                    pc - m.pixel_start, out0, p, end, out2);
                                assert(s == out2);
                            },
                            Err(_) => {},
                        }
                    },
                    Err(_) => {},
                }
            }
        }
    }
}

/// The `Ok` output of the policy mask phase keeps the XOR plane's length:
/// the two BMP modes return it unchanged.
pub proof fn lemma_mp_mask_phase_policy_len(
    b: Seq<u8>, m: MHRecord, pos: int, x_len: int, out: Seq<u8>, p: Policy, end: int, out2: Seq<u8>,
)
    requires
        mp_mask_phase_policy(b, m, pos, x_len, out, p) == Ok((end, out2)),
    ensures
        out2.len() == out.len(),
{
    reveal(mp_mask_phase_policy);
}

/// The visibility and output caps hold for every `MH_policy` success: the
/// guard chain's width/height rules and the output cap, read off the record's
/// construction the same way `lemma_mh_ok_bounds` reads `MH`'s. The policy
/// prefix is unfolded in place of `r1_prefix`; the R2–R7 chain is shared.
pub proof fn lemma_mh_policy_bounds(b: Seq<u8>, bm: BmpMode, p: Policy, m: MHRecord)
    requires
        mh_policy(b, bm, p) == Ok(m),
    ensures
        mh_bounds(m),
{
    reveal(mh_policy);
    reveal(r2r7);
    reveal(r1_prefix_policy);
    reveal(crate::dib::bmp_mode_to_mode);
    reveal(mh_bounds);
    reveal(visible_height_of);
    reveal(stride_of);
    reveal(pow2);
    reveal(abs_int);
    reveal(depth_ok);
    reveal(hsize_ok);
    reveal(within_bits);
    reveal(contiguous);
    reveal(low_of);
    reveal(masks_ok);
    assert(m.stride == 0 || m.stride == stride_of(m.width, m.depth));
    if m.stride != 0 {
        lemma_stride_bound(m.width, m.depth);
    }
    assert(1 <= m.width <= max_dimension());
    assert(1 <= m.height <= max_dimension());
    assert(m.n == m.width * m.height);
    assert(1 <= m.n) by (nonlinear_arith)
        requires
            m.n == m.width * m.height,
            1 <= m.width,
            1 <= m.height,
    ;
    assert(4 * m.n <= max_output());
    assert(m.n <= 16777216) by (nonlinear_arith)
        requires
            m.n == m.width * m.height,
            1 <= m.width,
            1 <= m.height,
            4 * m.n <= max_output(),
    ;
    assert(0 <= m.stride <= 32768);
}

} // verus!
