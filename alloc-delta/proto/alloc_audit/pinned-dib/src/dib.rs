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
///
/// `rle_or_32bpp` is the gdk-pixbuf 2.42.12 `has_alpha` rule, computed from the
/// validated header (depth 32, or RLE8/RLE4 compression) and never from a pixel
/// value. The profiled entry point ties it to the header record (T9's companion
/// postcondition below); the policy-free `decode` fills it from `MH`'s record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decoded {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
    pub rle_or_32bpp: bool,
}

// ---------------------------------------------------------------------------
// P1 · the policy record, the profile selector and the profiled entry point's
// types (POLICY-SPEC, "Independent options" and "Named profiles").
// ---------------------------------------------------------------------------

/// One retained metadata option. `Declared` keeps DIB-CORE-SPEC 1.1's check;
/// `Ignore` suppresses exactly the check that names the field, and nothing else
/// (POLICY-SPEC: "An `Ignore` field suppresses only its named check; it never
/// relaxes the input/output caps"). In particular neither value makes either
/// declaration a read bound: `bfSize` and `biSizeImage` are validated or
/// skipped, never used to bound an input read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetadataOptions {
    Declared,
    Ignore,
}

/// R3 `planes`. `Must1` is 1.1's rule (`planes == 1`); `Ignore` accepts any
/// LE16 value including zero, and `planes` is never used in a stride or an
/// offset (POLICY-SPEC: "accept any LE16 value, including zero, without using it
/// in stride").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum PlanesOptions {
    Must1,
    Ignore,
}

/// R1 `reserved`. `Zero` is 1.1's rule (both LE16 reserved words zero);
/// `Ignore` suppresses exactly those two tests and nothing else.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ReservedOptions {
    Zero,
    Ignore,
}

/// R1/R7 `offset`. `Declared` is 1.1: R1's `[18,|b|]`, then R7's
/// `bfOffBits >= palette_end`, then pixels start at `bfOffBits`. `AllowAny`
/// seeks to the declared offset even when it falls inside metadata or beyond
/// EOF, leaving the guarded reads to fail on short data. `Gdk` repairs an
/// invalid offset on GDK's indexed and BI_RGB paths (POLICY-SPEC "Independent
/// options", `offset`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum OffsetOptions {
    Declared,
    AllowAny,
    Gdk,
}

/// R5/R7 `high_count`, the high-depth `biClrUsed` table rule. `BmpTable`
/// skips four bytes per `biClrUsed` at depth 16/24/32; `Ignore` skips no
/// optional table; `GdkBound` skips no table but rejects a count above the low
/// 32 bits of `2^depth`. All three reject a nonzero high-depth count in
/// `BareDib` (that check is R5's and is not a `high_count` value).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum HighCountOptions {
    BmpTable,
    Ignore,
    GdkBound,
}

/// R2 `headers`, the accepted DIB header sizes. `All1_1` is 1.1's set
/// `{12,16,40,52,56,64,108,124}`; `No16` removes 16; `No16No64` removes 16 and
/// 64. Rejection precedes any read of the remaining header.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum HeadersOptions {
    All1_1,
    No16,
    No16No64,
}

/// R6 `masks`, the BITFIELDS mask validity rule. `Reject` is 1.1: R,G,B
/// nonzero, contiguous and disjoint R/G/B/A, wholly inside depth. `Independent`
/// requires each RGB mask nonzero/contiguous/inside depth and checks the
/// optional alpha mask for contiguity/inside-depth too, but allows overlaps
/// between channels. `DefaultRGB` substitutes 555 opaque for 16-bit or BGRA
/// 8888 for 32-bit when any RGB mask is zero, otherwise accepts overlaps and
/// holes but rejects any bit above depth.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum MasksOptions {
    Reject,
    Independent,
    DefaultRGB,
}

/// R9 `short_pixels`. `Reject` is 1.1: each entire padded row must be present.
/// `ZeroFill` keeps the R8-initialized black in a short row and every later row,
/// advancing the cursor logically by the stride without reading absent bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ShortPixelsOptions {
    Reject,
    ZeroFill,
}

/// R11 `rle_eol`. `OnlyFinal` is 1.1: one final EOL raising `y` to `height`,
/// then EOB required. `StopAtHeight` ends decoding when an EOL starting at
/// `y < height` raises `y` to `height`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum RleEolOptions {
    OnlyFinal,
    StopAtHeight,
}

/// R10/R11 `rle_overrun`. `Reject` is 1.1; `Clip` consumes the complete packet
/// and paints only positions with `x < width`; `Image` clips encoded RLE8 runs
/// only and rejects RLE4 encoded and both absolute overrun packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum RleOverrunOptions {
    Reject,
    Clip,
    Image,
}

/// R11 `rle_delta`. `Inside` is 1.1: a delta must stay strictly inside the
/// image. `ClampStop` clamps each coordinate to `[0,width]`/`[0,height]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum RleDeltaOptions {
    Inside,
    ClampStop,
}

/// R10 `rle_eob`. `Required` is 1.1: an explicit `00 01`. `AtInputEnd` also
/// stops when an EOL/delta reached height or when fewer than two bytes remain.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum RleEobOptions {
    Required,
    AtInputEnd,
}

/// R8 `rle_fill`: the pixel an RLE image is prefilled with. `Palette0` is 1.1
/// (palette entry zero, alpha 255); `OpaqueBlack`/`TransparentBlack` are
/// `(0,0,0,255)`/`(0,0,0,0)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum RleFillOptions {
    Palette0,
    OpaqueBlack,
    TransparentBlack,
}

/// R7/R8/R10/R11 `palette_index`. `Reject` is 1.1: an index `>= count` is
/// `PaletteIndexOutOfRange`. `Black` substitutes `(0,0,0,255)` without a read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum PaletteIndexOptions {
    Reject,
    Black,
}

/// R6 `scaling`: how a component value is scaled to 8 bits. `Round` is 1.1's
/// exact integer rounding; `Image` keeps the most significant 8 extracted bits
/// for masks wider than 8; `Gdk` follows GDK's bounded popcount shifts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ScalingOptions {
    Round,
    Image,
    Gdk,
}

/// R4 `unsupported`. `Reject` is 1.1's six depths; `Rgb2` adds 2 bpp for
/// headers of at least 40 bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum UnsupportedOptions {
    Reject,
    Rgb2,
}

/// R1/R7 `bitfield_offset`: where BITFIELDS pixels start. `DeclaredOffset` is
/// 1.1 (the raw `bfOffBits`); `MetadataEnd` starts immediately after the
/// masks/optional table, ignoring `bfOffBits` after header validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum BitfieldOffsetOptions {
    DeclaredOffset,
    MetadataEnd,
}

/// R8 `bitfield16_alpha`: 16-bit BITFIELDS alpha. `Mask` is 1.1 (expose a
/// nonzero alpha mask); `Opaque` writes 255 regardless of that mask.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Bitfield16AlphaOptions {
    Mask,
    Opaque,
}

/// R6/R7 `incomplete_metadata`. `Reject` is 1.1. `Blank` returns initialized
/// black output for a short external mask or nonempty palette/table after
/// validated dimensions and the output cap, bypassing later pixel/size checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum IncompleteMetadataOptions {
    Reject,
    Blank,
}

/// R7 `short_indexed_gap`. `Decode` is 1.1. `Blank` returns initialized black
/// raw (opaque) or RLE (transparent) pixels when `0 < biClrUsed < 2^depth` and
/// `bfOffBits > palette_end`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ShortIndexedGapOptions {
    Decode,
    Blank,
}

/// R2 `header_bytes`. `Full` is 1.1: all `H` declared bytes must be present.
/// `ParsedFields` requires only the fields actually read (12/16 for those
/// headers, the first 40 bytes for `H >= 40`), then guards each later read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum HeaderBytesOptions {
    Full,
    ParsedFields,
}

/// All twenty-two retained policy fields, including metadata and pixel recovery.
/// Every combination has the same safety, termination, allocation and work bounds.
///
/// POLICY-SPEC calls the record of individual field values internal and
/// experimental, not a public constructor. The fields must be `pub` for
/// `model.rs`'s `pub open spec fn`s to be well-formed outside the crate: Verus
/// treats a struct with any non-`pub` field as opaque everywhere and then rejects
/// the spec-fn field expressions this crate relies on (a `pub(crate)` field is
/// not enough — the public entry point's own `ensures` clause mentions the model).
/// Cross-crate construction is still impossible: `#[non_exhaustive]` rejects an
/// external literal, and the only constructors, `strict_policy_exec`,
/// `profile_policy_exec`, are `pub(crate)`. A release caller therefore cannot
/// obtain a record with custom field values nor mutate one; the only release
/// decoder is `decode_with_policy`, which takes a `BmpProfile`, and the
/// custom-record decoder `decode_custom_policy` exists only under `cfg(test)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct Policy {
    pub file_size: MetadataOptions,
    pub image_size: MetadataOptions,
    /// R3: `Must1` keeps 1.1's `planes == 1`; `Ignore` accepts any LE16 value.
    pub planes: PlanesOptions,
    /// R1: `Zero` keeps 1.1's two reserved words; `Ignore` suppresses only those.
    pub reserved: ReservedOptions,
    /// R1/R7: `Declared` keeps 1.1's `bfOffBits` rules; `AllowAny` seeks to the
    /// declared offset; `Gdk` repairs an invalid offset the way GDK's indexed /
    /// BI_RGB paths do.
    pub offset: OffsetOptions,
    /// R5/R7: the high-depth `biClrUsed` table rule.
    pub high_count: HighCountOptions,
    /// R2: the accepted DIB header sizes.
    pub headers: HeadersOptions,
    /// R6: the BITFIELDS mask validity rule.
    pub masks: MasksOptions,
    /// R9: the short-row rule.
    pub short_pixels: ShortPixelsOptions,
    /// R11: the terminal-EOL rule.
    pub rle_eol: RleEolOptions,
    /// R10/R11: the over-long-run rule.
    pub rle_overrun: RleOverrunOptions,
    /// R11: the delta coordinate rule.
    pub rle_delta: RleDeltaOptions,
    /// R10: the explicit-end-of-bitmap rule.
    pub rle_eob: RleEobOptions,
    /// R8: the RLE prefill pixel.
    pub rle_fill: RleFillOptions,
    /// R7/R8/R10/R11: the out-of-range palette index rule.
    pub palette_index: PaletteIndexOptions,
    /// R6: the component-scaling rule.
    pub scaling: ScalingOptions,
    /// R4: the accepted pixel depths.
    pub unsupported: UnsupportedOptions,
    /// R1/R7: where BITFIELDS pixels start.
    pub bitfield_offset: BitfieldOffsetOptions,
    /// R8: the 16-bit BITFIELDS alpha rule.
    pub bitfield16_alpha: Bitfield16AlphaOptions,
    /// R6/R7: the short external-mask / nonempty-palette rule.
    pub incomplete_metadata: IncompleteMetadataOptions,
    /// R7: the short-indexed-palette gap rule.
    pub short_indexed_gap: ShortIndexedGapOptions,
    /// R2: the declared-header-bytes rule.
    pub header_bytes: HeaderBytesOptions,
}

/// The profile selector. All four frozen named profiles of POLICY-SPEC
/// "Named profiles". `Strict` is DIB-CORE-SPEC 1.1 exactly; `Recommended`
/// differs from it only by `file_size = Ignore` and `image_size = Ignore`;
/// `GdkCompat` and `ImageCrateCompat` are the measured gdk-pixbuf 2.42.12 and
/// image 0.25.10 assignments. `Default` is `Strict`. The matches in
/// `profile_policy` and `profile_policy_exec` are deliberately exhaustive, so a
/// future variant is a compile error there rather than a silent default to
/// Strict.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum BmpProfile {
    Strict,
    Recommended,
    GdkCompat,
    ImageCrateCompat,
}

impl std::default::Default for BmpProfile {
    fn default() -> Self {
        BmpProfile::Strict
    }
}

/// The mode of the profiled entry point. POLICY-SPEC gives
/// `decode_with_policy` only the two BMP modes; the two ICO modes keep the
/// policy-free `decode`, so no ICO input can reach a policy branch and no mode
/// needs to be rejected at run time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum BmpMode {
    BmpFile,
    BareDib,
}

/// The `Mode` a profiled `BmpMode` decodes as.
pub open spec fn bmp_mode_to_mode(bm: BmpMode) -> Mode {
    match bm {
        BmpMode::BmpFile => Mode::BmpFile,
        BmpMode::BareDib => Mode::BareDib,
    }
}

/// `Recommended`'s policy record (POLICY-SPEC "Named profiles"): Strict except
/// `file_size = Ignore` and `image_size = Ignore`.
pub open spec fn recommended_policy() -> Policy {
    Policy {
        file_size: MetadataOptions::Ignore,
        image_size: MetadataOptions::Ignore,
        planes: PlanesOptions::Must1,
        reserved: ReservedOptions::Zero,
        offset: OffsetOptions::Declared,
        high_count: HighCountOptions::BmpTable,
        headers: HeadersOptions::All1_1,
        masks: MasksOptions::Reject,
        short_pixels: ShortPixelsOptions::Reject,
        rle_eol: RleEolOptions::OnlyFinal,
        rle_overrun: RleOverrunOptions::Reject,
        rle_delta: RleDeltaOptions::Inside,
        rle_eob: RleEobOptions::Required,
        rle_fill: RleFillOptions::Palette0,
        palette_index: PaletteIndexOptions::Reject,
        scaling: ScalingOptions::Round,
        unsupported: UnsupportedOptions::Reject,
        bitfield_offset: BitfieldOffsetOptions::DeclaredOffset,
        bitfield16_alpha: Bitfield16AlphaOptions::Mask,
        incomplete_metadata: IncompleteMetadataOptions::Reject,
        short_indexed_gap: ShortIndexedGapOptions::Decode,
        header_bytes: HeaderBytesOptions::Full,
    }
}

/// `Strict`'s policy record: every option group keeps its 1.1 value.
pub open spec fn strict_policy() -> Policy {
    Policy {
        file_size: MetadataOptions::Declared,
        image_size: MetadataOptions::Declared,
        planes: PlanesOptions::Must1,
        reserved: ReservedOptions::Zero,
        offset: OffsetOptions::Declared,
        high_count: HighCountOptions::BmpTable,
        headers: HeadersOptions::All1_1,
        masks: MasksOptions::Reject,
        short_pixels: ShortPixelsOptions::Reject,
        rle_eol: RleEolOptions::OnlyFinal,
        rle_overrun: RleOverrunOptions::Reject,
        rle_delta: RleDeltaOptions::Inside,
        rle_eob: RleEobOptions::Required,
        rle_fill: RleFillOptions::Palette0,
        palette_index: PaletteIndexOptions::Reject,
        scaling: ScalingOptions::Round,
        unsupported: UnsupportedOptions::Reject,
        bitfield_offset: BitfieldOffsetOptions::DeclaredOffset,
        bitfield16_alpha: Bitfield16AlphaOptions::Mask,
        incomplete_metadata: IncompleteMetadataOptions::Reject,
        short_indexed_gap: ShortIndexedGapOptions::Decode,
        header_bytes: HeaderBytesOptions::Full,
    }
}

/// `GdkCompat-2.42.12`'s policy record (POLICY-SPEC "Named profiles"): the
/// measured gdk-pixbuf 2.42.12 assignment on all twenty-two retained fields.
pub open spec fn gdk_compat_policy() -> Policy {
    Policy {
        file_size: MetadataOptions::Ignore,
        image_size: MetadataOptions::Ignore,
        planes: PlanesOptions::Ignore,
        reserved: ReservedOptions::Ignore,
        offset: OffsetOptions::Gdk,
        high_count: HighCountOptions::GdkBound,
        headers: HeadersOptions::No16,
        masks: MasksOptions::DefaultRGB,
        short_pixels: ShortPixelsOptions::ZeroFill,
        rle_eol: RleEolOptions::StopAtHeight,
        rle_overrun: RleOverrunOptions::Clip,
        rle_delta: RleDeltaOptions::ClampStop,
        rle_eob: RleEobOptions::AtInputEnd,
        rle_fill: RleFillOptions::TransparentBlack,
        palette_index: PaletteIndexOptions::Black,
        scaling: ScalingOptions::Gdk,
        unsupported: UnsupportedOptions::Reject,
        bitfield_offset: BitfieldOffsetOptions::MetadataEnd,
        bitfield16_alpha: Bitfield16AlphaOptions::Opaque,
        incomplete_metadata: IncompleteMetadataOptions::Blank,
        short_indexed_gap: ShortIndexedGapOptions::Blank,
        header_bytes: HeaderBytesOptions::Full,
    }
}

/// `ImageCrateCompat-0.25.10`'s policy record (POLICY-SPEC "Named profiles"):
/// the measured image 0.25.10 assignment on all twenty-two retained fields.
pub open spec fn image_crate_compat_policy() -> Policy {
    Policy {
        file_size: MetadataOptions::Ignore,
        image_size: MetadataOptions::Ignore,
        planes: PlanesOptions::Must1,
        reserved: ReservedOptions::Ignore,
        offset: OffsetOptions::AllowAny,
        high_count: HighCountOptions::Ignore,
        headers: HeadersOptions::No16No64,
        masks: MasksOptions::Independent,
        short_pixels: ShortPixelsOptions::Reject,
        rle_eol: RleEolOptions::StopAtHeight,
        rle_overrun: RleOverrunOptions::Image,
        rle_delta: RleDeltaOptions::Inside,
        rle_eob: RleEobOptions::Required,
        rle_fill: RleFillOptions::OpaqueBlack,
        palette_index: PaletteIndexOptions::Black,
        scaling: ScalingOptions::Image,
        unsupported: UnsupportedOptions::Rgb2,
        bitfield_offset: BitfieldOffsetOptions::DeclaredOffset,
        bitfield16_alpha: Bitfield16AlphaOptions::Mask,
        incomplete_metadata: IncompleteMetadataOptions::Reject,
        short_indexed_gap: ShortIndexedGapOptions::Decode,
        header_bytes: HeaderBytesOptions::ParsedFields,
    }
}

/// The profile selector's policy record (POLICY-SPEC "Named profiles").
pub open spec fn profile_policy(p: BmpProfile) -> Policy {
    match p {
        BmpProfile::Strict => strict_policy(),
        BmpProfile::Recommended => recommended_policy(),
        BmpProfile::GdkCompat => gdk_compat_policy(),
        BmpProfile::ImageCrateCompat => image_crate_compat_policy(),
    }
}

/// The executable wrapper for `bmp_mode_to_mode`: the profiled entry point calls
/// this so its postcondition carries the spec-fn identity.
pub fn bmp_mode_mode(bm: BmpMode) -> (r: Mode)
    ensures
        r == bmp_mode_to_mode(bm),
{
    match bm {
        BmpMode::BmpFile => Mode::BmpFile,
        BmpMode::BareDib => Mode::BareDib,
    }
}

/// The executable wrapper for `strict_policy`: `parse_header` calls this so its
/// calls to the policy-parameterised field helpers carry the spec-fn identity.
/// Not part of the release API: `pub(crate)` so the `cfg(test)` custom-record
/// harness can start from the Strict record (POLICY-SPEC: the field record is
/// internal and experimental, not a public constructor).
pub(crate) fn strict_policy_exec() -> (r: Policy)
    ensures
        r == strict_policy(),
{
    Policy {
        file_size: MetadataOptions::Declared,
        image_size: MetadataOptions::Declared,
        planes: PlanesOptions::Must1,
        reserved: ReservedOptions::Zero,
        offset: OffsetOptions::Declared,
        high_count: HighCountOptions::BmpTable,
        headers: HeadersOptions::All1_1,
        masks: MasksOptions::Reject,
        short_pixels: ShortPixelsOptions::Reject,
        rle_eol: RleEolOptions::OnlyFinal,
        rle_overrun: RleOverrunOptions::Reject,
        rle_delta: RleDeltaOptions::Inside,
        rle_eob: RleEobOptions::Required,
        rle_fill: RleFillOptions::Palette0,
        palette_index: PaletteIndexOptions::Reject,
        scaling: ScalingOptions::Round,
        unsupported: UnsupportedOptions::Reject,
        bitfield_offset: BitfieldOffsetOptions::DeclaredOffset,
        bitfield16_alpha: Bitfield16AlphaOptions::Mask,
        incomplete_metadata: IncompleteMetadataOptions::Reject,
        short_indexed_gap: ShortIndexedGapOptions::Decode,
        header_bytes: HeaderBytesOptions::Full,
    }
}

/// The executable wrapper for `profile_policy`. Not public: only
/// `decode_with_policy` calls it, so the record never escapes the crate. The two
/// original named-profile arms are unchanged; the two compatibility profiles
/// carry every field of the frozen table. Each arm's value must equal its
/// `profile_policy` arm term for the `ensures` clause to close.
pub(crate) fn profile_policy_exec(p: BmpProfile) -> (r: Policy)
    ensures
        r == profile_policy(p),
{
    match p {
        BmpProfile::Strict => Policy {
            file_size: MetadataOptions::Declared,
            image_size: MetadataOptions::Declared,
            planes: PlanesOptions::Must1,
            reserved: ReservedOptions::Zero,
            offset: OffsetOptions::Declared,
            high_count: HighCountOptions::BmpTable,
            headers: HeadersOptions::All1_1,
            masks: MasksOptions::Reject,
        short_pixels: ShortPixelsOptions::Reject,
        rle_eol: RleEolOptions::OnlyFinal,
        rle_overrun: RleOverrunOptions::Reject,
        rle_delta: RleDeltaOptions::Inside,
        rle_eob: RleEobOptions::Required,
        rle_fill: RleFillOptions::Palette0,
        palette_index: PaletteIndexOptions::Reject,
        scaling: ScalingOptions::Round,
        unsupported: UnsupportedOptions::Reject,
        bitfield_offset: BitfieldOffsetOptions::DeclaredOffset,
        bitfield16_alpha: Bitfield16AlphaOptions::Mask,
        incomplete_metadata: IncompleteMetadataOptions::Reject,
        short_indexed_gap: ShortIndexedGapOptions::Decode,
        header_bytes: HeaderBytesOptions::Full,
        },
        BmpProfile::Recommended => Policy {
            file_size: MetadataOptions::Ignore,
            image_size: MetadataOptions::Ignore,
            planes: PlanesOptions::Must1,
            reserved: ReservedOptions::Zero,
            offset: OffsetOptions::Declared,
            high_count: HighCountOptions::BmpTable,
            headers: HeadersOptions::All1_1,
            masks: MasksOptions::Reject,
        short_pixels: ShortPixelsOptions::Reject,
        rle_eol: RleEolOptions::OnlyFinal,
        rle_overrun: RleOverrunOptions::Reject,
        rle_delta: RleDeltaOptions::Inside,
        rle_eob: RleEobOptions::Required,
        rle_fill: RleFillOptions::Palette0,
        palette_index: PaletteIndexOptions::Reject,
        scaling: ScalingOptions::Round,
        unsupported: UnsupportedOptions::Reject,
        bitfield_offset: BitfieldOffsetOptions::DeclaredOffset,
        bitfield16_alpha: Bitfield16AlphaOptions::Mask,
        incomplete_metadata: IncompleteMetadataOptions::Reject,
        short_indexed_gap: ShortIndexedGapOptions::Decode,
        header_bytes: HeaderBytesOptions::Full,
        },
        BmpProfile::GdkCompat => Policy {
            file_size: MetadataOptions::Ignore,
            image_size: MetadataOptions::Ignore,
            planes: PlanesOptions::Ignore,
            reserved: ReservedOptions::Ignore,
            offset: OffsetOptions::Gdk,
            high_count: HighCountOptions::GdkBound,
            headers: HeadersOptions::No16,
            masks: MasksOptions::DefaultRGB,
            short_pixels: ShortPixelsOptions::ZeroFill,
            rle_eol: RleEolOptions::StopAtHeight,
            rle_overrun: RleOverrunOptions::Clip,
            rle_delta: RleDeltaOptions::ClampStop,
            rle_eob: RleEobOptions::AtInputEnd,
            rle_fill: RleFillOptions::TransparentBlack,
            palette_index: PaletteIndexOptions::Black,
            scaling: ScalingOptions::Gdk,
            unsupported: UnsupportedOptions::Reject,
            bitfield_offset: BitfieldOffsetOptions::MetadataEnd,
            bitfield16_alpha: Bitfield16AlphaOptions::Opaque,
            incomplete_metadata: IncompleteMetadataOptions::Blank,
            short_indexed_gap: ShortIndexedGapOptions::Blank,
            header_bytes: HeaderBytesOptions::Full,
        },
        BmpProfile::ImageCrateCompat => Policy {
            file_size: MetadataOptions::Ignore,
            image_size: MetadataOptions::Ignore,
            planes: PlanesOptions::Must1,
            reserved: ReservedOptions::Ignore,
            offset: OffsetOptions::AllowAny,
            high_count: HighCountOptions::Ignore,
            headers: HeadersOptions::No16No64,
            masks: MasksOptions::Independent,
            short_pixels: ShortPixelsOptions::Reject,
            rle_eol: RleEolOptions::StopAtHeight,
            rle_overrun: RleOverrunOptions::Image,
            rle_delta: RleDeltaOptions::Inside,
            rle_eob: RleEobOptions::Required,
            rle_fill: RleFillOptions::OpaqueBlack,
            palette_index: PaletteIndexOptions::Black,
            scaling: ScalingOptions::Image,
            unsupported: UnsupportedOptions::Rgb2,
            bitfield_offset: BitfieldOffsetOptions::DeclaredOffset,
            bitfield16_alpha: Bitfield16AlphaOptions::Mask,
            incomplete_metadata: IncompleteMetadataOptions::Reject,
            short_indexed_gap: ShortIndexedGapOptions::Decode,
            header_bytes: HeaderBytesOptions::ParsedFields,
        },
    }
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
/// `shift = trailing_zeros(m)`, `max = (1 << bit_length(m >> shift)) - 1`,
/// `component = (word & m) >> shift`. Contiguous Strict masks retain their values.
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
    let compact = mask >> shift;
    let x = compact | (compact >> 1);
    let x = x | (x >> 2);
    let x = x | (x >> 4);
    let x = x | (x >> 8);
    let max = x | (x >> 16);
    let component = (word & mask) >> shift;
    proof {
        use vstd::std_specs::bits::u32_trailing_zeros;
        assert(shift == u32_trailing_zeros(mask));
        assert(shift < 32u32);
        assert((compact & 1u32) == 1u32);
        assert(compact != 0u32) by (bit_vector)
            requires (compact & 1u32) == 1u32;
        assert(max == crate::model::channel_span(compact));
        crate::model::lemma_channel_span_bounds(compact);
        assert((word & mask) <= mask) by (bit_vector);
        assert(component <= compact) by (bit_vector)
            requires (word & mask) <= mask,
                component == (word & mask) >> shift, compact == mask >> shift;
        assert(component <= max && max != 0u32);
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

/// R6 `masks` validity, one arm per option value. `Reject` is `masks_valid`;
/// `Independent` drops the pairwise-disjoint tests only; `DefaultRGB` keeps the
/// within-depth test only (the RGB masks are already defaulted when zero, so the
/// nonzero/contiguous/overlap rules no longer apply).
fn masks_valid_policy(m0: u32, m1: u32, m2: u32, m3: u32, bits: u32, o: MasksOptions) -> (r: bool)
    ensures
        r == crate::model::masks_ok_policy(m0, m1, m2, m3, bits as int, o),
{
    proof {
        reveal(crate::model::masks_ok_policy);
        reveal(crate::model::masks_ok);
        reveal(crate::model::within_bits);
        reveal(crate::model::contiguous);
        reveal(crate::model::low_of);
    }
    match o {
        MasksOptions::Reject => masks_valid(m0, m1, m2, m3, bits),
        MasksOptions::Independent => {
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
            true
        },
        MasksOptions::DefaultRGB => {
            inside(m0, bits) && inside(m1, bits) && inside(m2, bits) && inside(m3, bits)
        },
    }
}

/// R6 `masks`: the four post-substitution mask words and the policy validity
/// verdict, as one function. `DefaultRGB` replaces all four when any RGB mask
/// is zero; the other two values leave them unchanged. The postcondition is the
/// model's `masks_after_policy` and `masks_ok_policy`.
fn policy_masks(
    raw0: u32,
    raw1: u32,
    raw2: u32,
    raw3: u32,
    bits: u32,
    o: MasksOptions,
) -> (r: ([u32; 4], bool))
    ensures
        (r.0[0], r.0[1], r.0[2], r.0[3]) == crate::model::masks_after_policy(
            raw0, raw1, raw2, raw3, bits as int, o,
        ),
        r.1 == crate::model::masks_ok_policy(r.0[0], r.0[1], r.0[2], r.0[3], bits as int, o),
{
    proof {
        reveal(crate::model::masks_after_policy);
        reveal(crate::model::masks_ok_policy);
        reveal(crate::model::default_rgb_masks);
        reveal(crate::model::masks_ok);
    }
    let sub: [u32; 4] = match o {
        MasksOptions::DefaultRGB => {
            if raw0 == 0 || raw1 == 0 || raw2 == 0 {
                if bits == 16u32 {
                    [0x7c00u32, 0x03e0u32, 0x001fu32, 0u32]
                } else {
                    [0x00ff0000u32, 0x0000ff00u32, 0x000000ffu32, 0xff000000u32]
                }
            } else {
                [raw0, raw1, raw2, raw3]
            }
        },
        _ => [raw0, raw1, raw2, raw3],
    };
    proof {
        reveal(crate::model::masks_after_policy);
        reveal(crate::model::default_rgb_masks);
    }
    let ok = masks_valid_policy(sub[0], sub[1], sub[2], sub[3], bits, o);
    proof {
        reveal(crate::model::masks_ok_policy);
    }
    (sub, ok)
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
    &&& (count > 0 ==> (palette_start as int) + (count as int) * (pal_stride as int) <= b@.len())
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

/// The gdk-pixbuf `has_alpha` rule on the parsed header, as an executable `bool`:
/// the same condition `fill_blank`'s `transparent` uses. The postcondition ties it
/// to the record-level predicate `model::rle_or_32bpp`.
fn header_alpha(h: &Header) -> (r: bool)
    ensures r == crate::model::rle_or_32bpp(h.spec_depth(), h.spec_compression()),
{
    proof { reveal(crate::model::rle_or_32bpp); }
    h.bits == 32u32 || h.compression == 1u32 || h.compression == 2u32
}

/// The success invariant `parse_header` establishes, specialised to the bounds
/// the pixel decoder needs (T2/T3 for the concrete `Header`).
spec fn header_inv_light(h: &Header) -> bool {
    &&& 1 <= h.spec_width() <= 8192
    &&& 1 <= h.spec_height() <= 8192
    &&& (crate::model::depth_ok(h.spec_depth()) || h.spec_depth() == 2)
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
    &&& (h.table_count > 0 ==> h.palette_end <= b.len())
    &&& (h.count > 0 ==> h.palette_start + (h.count as int) * (h.pal_stride as int) <= b.len())
    &&& (h.pal_stride == 3 || h.pal_stride == 4)
    &&& h.n as int == h.spec_width() * h.spec_height()
    &&& (h.compression == 0u32 || h.compression == 3u32) ==> h.spec_stride() <= 32768
    &&& (h.compression == 1u32 ==> h.spec_depth() == 8)
    &&& (h.compression == 2u32 ==> h.spec_depth() == 4)
    &&& (h.compression == 1u32 || h.compression == 2u32) ==> h.count >= 1u32
    &&& h.bits == 1u32 || h.bits == 2u32 || h.bits == 4u32 || h.bits == 8u32 || h.bits == 16u32 || h.bits == 24u32
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
            assert((bf_size != 0 && bf_size > (len as u64)) == (crate::model::le32(bytes@, 2) != 0
                && crate::model::le32(bytes@, 2) > bytes@.len())) by { reveal(crate::model::le32); };
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
/// R1/R2 with the `file_size` option: the BMP prefix (if any) and the R2–R5
/// field block, then the shared `dib_finish`. `MH_policy`'s exact record or
/// first error is the postcondition. The two option values touch one test each:
/// `Declared` keeps 1.1's `bfSize` bound and `Ignore` drops it; `biSizeImage`,
/// the reserved words and `bfOffBits` are unaffected.
fn parse_header_policy(
    bytes: &[u8],
    bm: BmpMode,
    policy: Policy,
) -> (r: Result<Header, Error>)
    ensures
        match crate::model::mh_policy(bytes@, bm, policy) {
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
            reveal(crate::model::mh_policy);
        }
        return Err(Error::FileTooLarge);
    }
    proof {
        reveal(crate::model::mh_policy);
        reveal(crate::model::max_input);
        assert(bytes@.len() <= crate::model::max_input());
    }
    let mode = bmp_mode_mode(bm);
    let is_bmp = match bm {
        BmpMode::BmpFile => true,
        BmpMode::BareDib => false,
    };
    let start: usize;
    let file_offset: u64;
    match bm {
        BmpMode::BmpFile => {
            let p = match bmp_prefix_policy(bytes, policy) {
                Ok(p) => p,
                Err(e) => {
                    proof {
                        reveal(crate::model::mh_policy);
                            }
                    return Err(e);
                },
            };
            start = p.0;
            file_offset = p.1;
            proof {
                reveal(crate::model::mh_policy);
                assert(crate::model::r1_prefix_policy(bytes@, policy) == Ok((
                    start as int,
                    file_offset as int,
                )));
            }
        }
        BmpMode::BareDib => {
            proof {
                reveal(crate::model::mh_policy);
            }
            start = 0;
            file_offset = 0;
        }
    }
    proof {
        reveal(crate::model::mh_policy);
        reveal(crate::dib::bmp_mode_to_mode);
    }
    let f = match dib_fields(bytes, mode, start, file_offset, is_bmp, policy) {
        Ok(f) => f,
        Err(e) => {
            proof {
                reveal(crate::model::mh_policy);
            }
            return Err(e);
        },
    };
    proof {
        reveal(crate::model::r2r7_policy);
        reveal(crate::model::r6r7_policy);
        assert(bytes@.len() <= crate::model::max_input());
        assert(bytes@.len() >= f.start as int + crate::model::header_bytes_needed(f.size as int, policy));
        assert(f.spec_raw().compression == 3 && !(f.spec_raw().hsize == 40 || f.spec_raw().hsize
            == 64) ==> f.spec_raw().hsize >= 52) by {
            reveal(crate::model::hsize_ok_policy);
            reveal(crate::model::hsize_ok);
        }
        if is_bmp {
            assert((file_offset as int) == crate::model::le32(bytes@, 10));
            if policy.offset == OffsetOptions::Declared {
                assert((file_offset as int) <= bytes@.len());
            }
        }
    }
    match dib_finish(bytes, &f, file_offset, is_bmp, policy) {
        Ok(h) => {
            proof {
                reveal(crate::model::mh_policy);
            }
            Ok(h)
        }
        Err(e) => {
            proof {
                reveal(crate::model::mh_policy);
            }
            Err(e)
        }
    }
}

/// R1 with the `file_size` option: the BMP file-header prefix. `Ok` carries
/// `(s, bfOffBits)`. The postcondition is the model's `r1_prefix_policy`.
fn bmp_prefix_policy(bytes: &[u8], policy: Policy) -> (r: Result<(usize, u64), Error>)
    ensures
        match crate::model::r1_prefix_policy(bytes@, policy) {
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
            reveal(crate::model::r1_prefix_policy);
        }
        return Err(Error::TruncatedFileHeader);
    }
    proof {
        assert(len as int == bytes@.len());
    }
    if bytes[0] != b'B' || bytes[1] != b'M' {
        proof {
            reveal(crate::model::r1_prefix_policy);
            assert(bytes@[0] == bytes[0]);
            assert(bytes@[1] == bytes[1]);
            assert(bytes@[0] != 66u8 || bytes@[1] != 77u8);
        }
        return Err(Error::BadMagic);
    }
    let bf_size = rd32(bytes, 2) as u64;
    let res6 = rd16(bytes, 6);
    let res8 = rd16(bytes, 8);
    proof {
        assert(len as int == bytes@.len());
        assert((len as u64) as int == len as int);
        assert(bf_size as int == crate::model::le32(bytes@, 2));
        assert(res6 as int == crate::model::le16(bytes@, 6));
        assert(res8 as int == crate::model::le16(bytes@, 8));
        crate::model::lemma_le32_lt(bytes@, 2);
    }
    proof {
        reveal(crate::model::r1_size_bad);
        reveal(crate::model::r1_offset_bad);
        reveal(crate::model::le32);
        reveal(crate::model::le16);
    }
    // R1 `file_size`/`reserved`: four combinations, each with the model's own
    // expression for that combination.
    match policy.file_size {
        MetadataOptions::Declared => {
            match policy.reserved {
                ReservedOptions::Zero => {
                    if (bf_size != 0 && bf_size > len as u64) || res6 != 0 || res8 != 0 {
                        proof {
                                        assert(crate::model::r1_size_bad(bytes@, policy));
                            assert(crate::model::r1_prefix_policy(bytes@, policy) == Err(
                                Error::InvalidFileSize));
                        }
                        return Err(Error::InvalidFileSize);
                    }
                    proof {
                        assert(!crate::model::r1_size_bad(bytes@, policy));
                    }
                },
                ReservedOptions::Ignore => {
                    if bf_size != 0 && bf_size > len as u64 {
                        proof {
                                        assert(crate::model::r1_size_bad(bytes@, policy));
                            assert(crate::model::r1_prefix_policy(bytes@, policy) == Err(
                                Error::InvalidFileSize));
                        }
                        return Err(Error::InvalidFileSize);
                    }
                    proof {
                        assert(!crate::model::r1_size_bad(bytes@, policy));
                    }
                },
            }
        },
        MetadataOptions::Ignore => {
            match policy.reserved {
                ReservedOptions::Zero => {
                    if res6 != 0 || res8 != 0 {
                        proof {
                                        assert(crate::model::r1_size_bad(bytes@, policy));
                            assert(crate::model::r1_prefix_policy(bytes@, policy) == Err(
                                Error::InvalidFileSize));
                        }
                        return Err(Error::InvalidFileSize);
                    }
                    proof {
                        assert(!crate::model::r1_size_bad(bytes@, policy));
                    }
                },
                ReservedOptions::Ignore => {
                    proof {
                        assert(!crate::model::r1_size_bad(bytes@, policy));
                    }
                },
            }
        },
    }
    proof {
        reveal(crate::model::r1_prefix_policy);
        assert(!crate::model::r1_size_bad(bytes@, policy));
    }
    let off = rd32(bytes, 10) as u64;
    proof {
        assert(off as int == crate::model::le32(bytes@, 10));
        assert((len as u64) as int == len as int);
        reveal(crate::model::r1_offset_bad);
        reveal(crate::model::le32);
    }
    match policy.offset {
        OffsetOptions::Declared => {
            if off < 18 || off > len as u64 {
                proof {
                        assert(crate::model::r1_offset_bad(bytes@, policy)) by {
                        reveal(crate::model::r1_offset_bad);
                    }
                    assert(crate::model::r1_prefix_policy(bytes@, policy) == Err(
                        Error::InvalidPixelOffset));
                }
                return Err(Error::InvalidPixelOffset);
            }
            proof {
                assert(!crate::model::r1_offset_bad(bytes@, policy));
            }
        },
        _ => {
            proof {
                assert(!crate::model::r1_offset_bad(bytes@, policy));
            }
        },
    }
    proof {
        reveal(crate::model::r1_prefix_policy);
        assert(!crate::model::r1_offset_bad(bytes@, policy));
        assert(crate::model::r1_prefix_policy(bytes@, policy) == Ok((
            14int,
            crate::model::le32(bytes@, 10),
        )));
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
                            }
                    return Err(e);
                },
            };
            start = p.0;
            file_offset = p.1;
            proof {
                reveal(crate::model::mh);
                assert(crate::model::r1_prefix(bytes@) == Ok((start as int, file_offset as int)));
            }
        }
        _ => {
            proof {
                reveal(crate::model::mh);
            }
            start = 0;
            file_offset = 0;
        }
    }
    let sp = strict_policy_exec();
    proof {
        reveal(crate::model::mh);
    }
    let f = match dib_fields(bytes, mode, start, file_offset, is_bmp, sp) {
        Ok(f) => f,
        Err(e) => {
            proof {
                reveal(crate::model::mh);
                reveal(crate::model::r2r7);
                crate::model::lemma_r1_prefix_policy_strict(bytes@);
                crate::model::lemma_r2r5_policy_strict(bytes@, mode, start as int);
                crate::model::lemma_r2r7_policy_strict(bytes@, mode, start as int, file_offset as int, is_bmp);
            }
            return Err(e);
        },
    };
    proof {
        reveal(crate::model::mh);
        reveal(crate::model::r2r7);
        reveal(crate::model::r1_prefix);
        crate::model::lemma_r1_prefix_policy_strict(bytes@);
        crate::model::lemma_r2r5_policy_strict(bytes@, mode, start as int);
        crate::model::lemma_r2r7_policy_strict(bytes@, mode, start as int, file_offset as int, is_bmp);
        assert(crate::model::r2r5(bytes@, mode, start as int) == Ok(f.spec_raw()));
        assert(bytes@.len() <= crate::model::max_input());
        assert(bytes@.len() >= f.spec_raw().pos0);
        assert(f.spec_raw().compression == 3 && !(f.spec_raw().hsize == 40 || f.spec_raw().hsize
            == 64) ==> f.spec_raw().hsize >= 52) by {
            reveal(crate::model::hsize_ok_policy);
            reveal(crate::model::hsize_ok);
        }
        if is_bmp {
            assert((file_offset as int) == crate::model::le32(bytes@, 10));
            assert((file_offset as int) <= bytes@.len());
        }
    }
    match dib_finish(bytes, &f, file_offset, is_bmp, sp) {
        Ok(h) => {
            proof {
                reveal(crate::model::mh);
                reveal(crate::model::r2r7);
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

/// R2..R5: parse and validate the header fields up to the output cap, under the
/// `planes`, `headers` and `high_count` options. The postcondition is the
/// model's `r2r5_policy`; at `strict_policy()` the options sit at their 1.1
/// values, so this is `r2r5` (the caller uses the bridge lemma).
fn dib_fields(
    bytes: &[u8],
    mode: Mode,
    start: usize,
    file_offset: u64,
    is_bmp: bool,
    policy: Policy,
) -> (r: Result<Fields, Error>)
    requires
        is_bmp == (mode == Mode::BmpFile),
        is_bmp ==> start == 14,
        !is_bmp ==> start == 0 && file_offset == 0,
    ensures
        match crate::model::r2r5_policy(bytes@, mode, start as int, policy) {
            Ok(m) => match r {
                Ok(f) => f.mode == mode && f.is_bmp == (mode == Mode::BmpFile) && fields_matches(m, f)
                    && f.spec_n() == f.width_i as int * (f.height_v as int) && crate::model::hsize_ok(
                    f.size as int,
                ) && m.pos0 == f.start as int + f.size as int && bytes@.len() >= f.start as int + crate::model::header_bytes_needed(f.size as int, policy),
                Err(_) => false,
            },
            Err(e2) => match r {
                Ok(_) => false,
                Err(e) => e == e2,
            },
        },
{
    let len = bytes.len();
    proof {
        reveal(crate::model::r2r5_policy);
        reveal(crate::model::hsize_ok_policy);
        reveal(crate::model::hsize_ok);
        reveal(crate::model::visible_height_of);
        reveal(crate::model::pow2);
        reveal(crate::model::depth_ok);
        reveal(crate::model::is_ico);
        reveal(crate::model::s32_min);
        reveal(crate::model::abs_int);
        reveal(crate::model::max_dimension);
        reveal(crate::model::max_output);
        reveal(crate::model::gdk_high_bound);
    }

    // -----------------------------------------------------------------------
    // R2: DIB header size and layout.
    // -----------------------------------------------------------------------
    if len < start + 4 {
        proof {
            reveal(crate::model::r2r7_policy);
            reveal(crate::model::r2r5_policy);
        }
        return Err(Error::TruncatedDibHeader);
    }
    let size = rd32(bytes, start);
    proof {
        assert(size as int == crate::model::le32(bytes@, start as int));
    }
    let core = match size {
        12 => true,
        16 => {
            match policy.headers {
                HeadersOptions::All1_1 => false,
                _ => {
                    proof {
                                        assert(!crate::model::hsize_ok_policy(16, HeadersOptions::No16) && !
                            crate::model::hsize_ok_policy(16, HeadersOptions::No16No64));
                        reveal(crate::model::mh_policy);
                                    }
                    return Err(Error::UnsupportedHeaderSize);
                },
            }
        },
        64 => {
            match policy.headers {
                HeadersOptions::No16No64 => {
                    proof {
                                        assert(!crate::model::hsize_ok_policy(64, HeadersOptions::No16No64));
                        reveal(crate::model::mh_policy);
                                    }
                    return Err(Error::UnsupportedHeaderSize);
                },
                _ => false,
            }
        },
        40 | 52 | 56 | 108 | 124 => false,
        _ => {
            proof {
                assert(size != 12u32 && size != 16u32 && size != 40u32 && size != 52u32 && size
                    != 56u32 && size != 64u32 && size != 108u32 && size != 124u32);
                assert(!crate::model::hsize_ok_policy(size as int, policy.headers));
                reveal(crate::model::mh_policy);
            }
            return Err(Error::UnsupportedHeaderSize);
        },
    };
    proof {
        reveal(crate::model::hsize_ok_policy);
        reveal(crate::model::hsize_ok);
        assert(crate::model::hsize_ok_policy(size as int, policy.headers));
    }
    let needed = match policy.header_bytes { HeaderBytesOptions::ParsedFields => if size >= 40 { 40 } else { size }, _ => size };
    if len < start + needed as usize {
        proof {
            reveal(crate::model::mh_policy);
            reveal(crate::model::r2r7_policy);
            reveal(crate::model::r2r5_policy);
        }
        return Err(Error::TruncatedHeader);
    }
    proof {
        assert(bytes@.len() >= start as int + crate::model::header_bytes_needed(size as int, policy));
        // Pin `r2r5_policy`'s three early guards as decided, so every later exit
        // sees only the guard it is actually about.
        reveal(crate::model::r2r5_policy);
        reveal(crate::model::hsize_ok_policy);
        reveal(crate::model::hsize_ok);
        assert(size as int == crate::model::le32(bytes@, start as int));
        assert(!(bytes@.len() < start as int + 4));
        assert(!(bytes@.len() < start as int + crate::model::header_bytes_needed(crate::model::le32(bytes@, start as int), policy)));
        assert(crate::model::hsize_ok_policy(size as int, policy.headers));
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
            assert(bytes@.len() >= start as int + crate::model::header_bytes_needed(size as int, policy));
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

    validate_dib_fields(bytes, mode, start, is_bmp, policy, size, core,
        width_i, signed_height, planes, bits, compression, declared_image, used)
}

/// Validate the already-read R3–R5 fields without carrying the reader branches.
fn validate_dib_fields(
    bytes: &[u8], mode: Mode, start: usize, is_bmp: bool, policy: Policy,
    size: u32, core: bool, width_i: i32, signed_height: i32, planes: u32,
    bits: u32, compression: u32, declared_image: u32, used: u32,
) -> (r: Result<Fields, Error>)
    requires
        is_bmp == (mode == Mode::BmpFile),
        is_bmp ==> start == 14,
        !is_bmp ==> start == 0,
        bytes@.len() >= start as int + 4,
        size as int == crate::model::le32(bytes@, start as int),
        crate::model::hsize_ok_policy(size as int, policy.headers),
        bytes@.len() >= start as int + crate::model::header_bytes_needed(size as int, policy),
        core == (size == 12),
        width_i as int == crate::model::width_i_of(bytes@, start as int),
        signed_height as int == crate::model::stored_of(bytes@, start as int),
        planes as int == crate::model::planes_of(bytes@, start as int),
        bits as int == crate::model::depth_of(bytes@, start as int),
        compression as int == crate::model::compression_of(bytes@, start as int),
        declared_image as int == crate::model::declared_of(bytes@, start as int),
        used as int == crate::model::used_of(bytes@, start as int),
    ensures
        match crate::model::r2r5_policy(bytes@, mode, start as int, policy) {
            Ok(m) => match r {
                Ok(f) => f.mode == mode && f.is_bmp == (mode == Mode::BmpFile) && fields_matches(m, f)
                    && f.spec_n() == f.width_i as int * (f.height_v as int) && crate::model::hsize_ok(
                    f.size as int,
                ) && m.pos0 == f.start as int + f.size as int && bytes@.len() >= f.start as int + crate::model::header_bytes_needed(f.size as int, policy),
                Err(_) => false,
            },
            Err(e2) => match r {
                Ok(_) => false,
                Err(e) => e == e2,
            },
        },
{
    proof {
        reveal(crate::model::hsize_ok_policy);
        reveal(crate::model::hsize_ok);
        reveal(crate::model::visible_height_of);
        reveal(crate::model::pow2);
        reveal(crate::model::is_ico);
        reveal(crate::model::s32_min);
        reveal(crate::model::abs_int);
        reveal(crate::model::max_dimension);
        reveal(crate::model::max_output);
        reveal(crate::model::gdk_high_bound);
    }

    // -----------------------------------------------------------------------
    // R3: planes, dimensions, ICO stored-height rule, dimension caps.
    // -----------------------------------------------------------------------
    match policy.planes {
      PlanesOptions::Must1 => {
      if planes != 1 {
        proof {
            reveal(crate::model::r2r5_policy);
            reveal(crate::model::hsize_ok_policy);
            reveal(crate::model::hsize_ok);
            reveal(crate::model::visible_height_of);
            reveal(crate::model::pow2);
            reveal(crate::model::depth_ok);
            assert(planes as int == (if core {
                crate::model::le16(bytes@, start as int + 8)
            } else {
                crate::model::le16(bytes@, start as int + 12)
            }));
            assert((if crate::model::le32(bytes@, start as int) == 12 {
                crate::model::le16(bytes@, start as int + 8)
            } else {
                crate::model::le16(bytes@, start as int + 12)
            }) == planes as int);
            assert((if crate::model::le32(bytes@, start as int) == 12 {
                crate::model::le16(bytes@, start as int + 4) as int
            } else {
                crate::model::s32(bytes@, start as int + 4)
            }) == width_i as int);
            assert((if crate::model::le32(bytes@, start as int) == 12 {
                crate::model::le16(bytes@, start as int + 6) as int
            } else {
                crate::model::s32(bytes@, start as int + 8)
            }) == signed_height as int);
            crate::model::lemma_r2r5_policy_planes_err(bytes@, mode, start as int, policy);
        }
        return Err(Error::InvalidPlanes);
      }
      },
      PlanesOptions::Ignore => {},
    }
    if width_i <= 0 || signed_height == 0 || signed_height == i32::MIN {
        proof {
            reveal(crate::model::r2r5_policy);
            reveal(crate::model::s32_min);
            assert(signed_height as int == 0 || signed_height as int == crate::model::s32_min()
                || width_i <= 0);
            assert((if crate::model::le32(bytes@, start as int) == 12 {
                crate::model::le16(bytes@, start as int + 8)
            } else {
                crate::model::le16(bytes@, start as int + 12)
            }) == planes as int);
            assert((if crate::model::le32(bytes@, start as int) == 12 {
                crate::model::le16(bytes@, start as int + 4) as int
            } else {
                crate::model::s32(bytes@, start as int + 4)
            }) == width_i as int);
            assert((if crate::model::le32(bytes@, start as int) == 12 {
                crate::model::le16(bytes@, start as int + 6) as int
            } else {
                crate::model::s32(bytes@, start as int + 8)
            }) == signed_height as int);
            assert(width_i <= 0 || signed_height == 0 || signed_height == i32::MIN);
            crate::model::lemma_r2r5_policy_dims_err(bytes@, mode, start as int, policy);
        }
        return Err(Error::InvalidDimensions);
    }
    match mode {
        Mode::IcoEntryDib | Mode::IcoEntryDibSlack => {
            if signed_height < 0 || signed_height % 2 != 0 {
                proof {
                        reveal(crate::model::is_ico);
            assert((if crate::model::le32(bytes@, start as int) == 12 {
                crate::model::le16(bytes@, start as int + 8)
            } else {
                crate::model::le16(bytes@, start as int + 12)
            }) == planes as int);
            assert((if crate::model::le32(bytes@, start as int) == 12 {
                crate::model::le16(bytes@, start as int + 4) as int
            } else {
                crate::model::s32(bytes@, start as int + 4)
            }) == width_i as int);
            assert((if crate::model::le32(bytes@, start as int) == 12 {
                crate::model::le16(bytes@, start as int + 6) as int
            } else {
                crate::model::s32(bytes@, start as int + 8)
            }) == signed_height as int);
                    assert(signed_height < 0 || signed_height % 2 != 0);
                    crate::model::lemma_r2r5_policy_ico_err(bytes@, mode, start as int, policy);
                }
                return Err(Error::InvalidDimensions);
            }
        },
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
            reveal(crate::model::mh_policy);
            reveal(crate::model::r2r7_policy);
            reveal(crate::model::r2r5_policy);
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
    let rgb2 = match policy.unsupported {
        UnsupportedOptions::Rgb2 => size >= 40,
        UnsupportedOptions::Reject => false,
    };
    if bits != 1 && bits != 4 && bits != 8 && bits != 16 && bits != 24 && bits != 32
        && !(rgb2 && bits == 2) {
        proof {
            reveal(crate::model::mh_policy);
            reveal(crate::model::r2r7_policy);
            reveal(crate::model::r2r5_policy);
            reveal(crate::model::depth_ok);
        }
        return Err(Error::UnsupportedPixelDepth);
    }
    if compression > 3 {
        proof {
            reveal(crate::model::mh_policy);
            reveal(crate::model::r2r7_policy);
            reveal(crate::model::r2r5_policy);
        }
        return Err(Error::UnsupportedCompression);
    }
    let rle = compression == 1 || compression == 2;
    if (compression == 1 && bits != 8) || (compression == 2 && bits != 4) || (compression == 3
        && bits != 16 && bits != 32) {
        proof {
            reveal(crate::model::mh_policy);
            reveal(crate::model::r2r7_policy);
            reveal(crate::model::r2r5_policy);
        }
        return Err(Error::InvalidCompressionDepth);
    }
    let ico = match mode {
        Mode::IcoEntryDib | Mode::IcoEntryDibSlack => true,
        _ => false,
    };
    if (signed_height < 0 && rle) || (ico && rle) {
        proof {
            reveal(crate::model::mh_policy);
            reveal(crate::model::r2r7_policy);
            reveal(crate::model::r2r5_policy);
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
            } else if bits == 2u32 {
                assert(crate::model::pow2(bits as int) == 4);
            } else if bits == 4u32 {
                assert(crate::model::pow2(bits as int) == 16);
            } else {
                assert(bits == 8u32);
                assert(crate::model::pow2(bits as int) == 256);
            }
            assert(limit as int == crate::model::pow2(bits as int)) by (bit_vector)
                requires
                    bits == 1u32 || bits == 2u32 || bits == 4u32 || bits == 8u32,
                    limit == 1u32 << bits,
            ;
        }
        if used > limit {
            proof {
                reveal(crate::model::pow2);
                assert((bits as int) == (if core { crate::model::le16(bytes@, start as int + 10) }
                    else { crate::model::le16(bytes@, start as int + 14) }));
                assert((used as int) == (if size == 12 || size == 16 { 0 }
                    else { crate::model::le32(bytes@, start as int + 32) }));
                crate::model::lemma_r2r5_policy_low_count_err(bytes@, mode, start as int, policy);
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
                reveal(crate::model::pow2);
                assert((bits as int) == (if core { crate::model::le16(bytes@, start as int + 10) }
                    else { crate::model::le16(bytes@, start as int + 14) }));
                assert((used as int) == (if size == 12 || size == 16 { 0 }
                    else { crate::model::le32(bytes@, start as int + 32) }));
                assert(mode != Mode::BmpFile);
                crate::model::lemma_r2r5_policy_bare_high_err(bytes@, mode, start as int, policy);
            }
            return Err(Error::InvalidColorCount);
        }
        // R7 `high_count = GdkBound`: a nonzero BMP high-depth count above the
        // low 32 bits of `2^depth` is rejected (the 32-bpp threshold is zero).
        match policy.high_count {
            HighCountOptions::GdkBound => {
                if used != 0 && bmp {
                    let bound: u64 = if bits == 16 {
                        65536u64
                    } else if bits == 24 {
                        16777216u64
                    } else {
                        0u64
                    };
                    proof {
                        reveal(crate::model::gdk_high_bound);
                        assert(bound as int == crate::model::gdk_high_bound(bits as int));
                    }
                    if (used as u64) > bound {
                        proof {
                                        reveal(crate::model::gdk_high_bound);
                            assert((bits as int) == (if core {
                                crate::model::le16(bytes@, start as int + 10)
                            } else {
                                crate::model::le16(bytes@, start as int + 14)
                            }));
                            assert((used as int) == (if size == 12 || size == 16 {
                                0
                            } else {
                                crate::model::le32(bytes@, start as int + 32)
                            }));
                            assert(used as int > crate::model::gdk_high_bound(bits as int));
                            crate::model::lemma_r2r5_policy_gdk_count_err(
                                bytes@, mode, start as int, policy);
                        }
                        return Err(Error::InvalidColorCount);
                    }
                }
            },
            _ => {},
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
            reveal(crate::model::r2r5_policy);
            reveal(crate::model::pow2);
            reveal(crate::model::visible_height_of);
            reveal(crate::model::abs_int);
            assert(4 * n64 as int == 4 * (width_i as int * (height_v as int)));
            assert((width_i as int) == (if core { crate::model::le16(bytes@, start as int + 4)
                as int } else { crate::model::s32(bytes@, start as int + 4) }));
            assert((signed_height as int) == (if core { crate::model::le16(bytes@, start as int + 6)
                as int } else { crate::model::s32(bytes@, start as int + 8) }));
            crate::model::lemma_r2r5_policy_output_err(bytes@, mode, start as int, policy);
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

/// R6..R7: parse masks and palette geometry, then build the record, under the
/// `masks`, `high_count` and `offset` options. Given a `Fields` that is exactly
/// the model's `r2r5_policy` record, the postcondition is the model's
/// `r6r7_policy`.
/// Read and validate R6 masks before assembling the palette/header record.
fn dib_policy_masks(bytes: &[u8], f: &Fields, policy: Policy)
    -> (r: Result<(usize, [u32; 4]), Error>)
    requires
        crate::model::r2r5_policy(bytes@, f.mode, f.start as int, policy) == Ok(f.spec_raw()),
        f.start <= 14,
        crate::model::hsize_ok(f.size as int),
        f.spec_raw().compression == 3 && !(f.spec_raw().hsize == 40 || f.spec_raw().hsize == 64)
            ==> f.spec_raw().hsize >= 52,
    ensures
        match r {
            Ok((pos, masks)) => {
                &&& pos as int == crate::model::palette_start_of(f.spec_raw())
                &&& pos <= 150
                &&& (masks[0], masks[1], masks[2], masks[3])
                    == crate::model::masks_of_policy_rf(bytes@, f.start as int, f.spec_raw(), policy.masks)
                &&& !crate::model::policy_masks_missing(bytes@, f.start as int, f.spec_raw(), policy)
                &&& !(f.spec_raw().compression == 3
                    && !crate::model::masks_ok_policy_rf(bytes@, f.start as int, f.spec_raw(), policy.masks))
            },
            Err(e) => forall|off: int, bmp: bool| crate::model::r6r7_policy(
                bytes@, f.mode, f.start as int, off, bmp, f.spec_raw(), policy) == Err(e),
        },
{
    let len = bytes.len();
    let start = f.start;
    let size = f.size;
    let compression = f.compression;
    let bits = f.bits;
    let mut pos = start + size as usize;
    let mut masks = [0u32; 4];
    let mut mask_fields: [u32; 4] = [0u32; 4];
    let mut masks_okv: bool = true;
    if compression == 3 {
        if size == 40 || size == 64 {
            // R6: three external masks immediately after the header.
            if pos > len || len - pos < 12 {
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
            // Internal masks still require their complete physical span.
            let span = if size >= 56 { 56usize } else { 52usize };
            if len < start + span {
                proof { reveal(crate::model::r6r7_policy); reveal(crate::model::r2r5_policy); }
                return Err(Error::TruncatedMasks);
            }
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
            let mf0: u32 = mask_fields[0];
            let mf1: u32 = mask_fields[1];
            let mf2: u32 = mask_fields[2];
            let mf3: u32 = mask_fields[3];
            if size == 40 || size == 64 {
                let mw0: int = crate::model::le32(bytes@, start as int + size as int);
                let mw1: int = crate::model::le32(bytes@, start as int + size as int + 4);
                let mw2: int = crate::model::le32(bytes@, start as int + size as int + 8);
                assert(mf0 as int == mw0);
                assert(mf1 as int == mw1);
                assert(mf2 as int == mw2);
                assert(crate::model::as_u32(mw0) == mf0) by { reveal(crate::model::as_u32); };
                assert(crate::model::as_u32(mw1) == mf1) by { reveal(crate::model::as_u32); };
                assert(crate::model::as_u32(mw2) == mf2) by { reveal(crate::model::as_u32); };
                assert(mask_fields[0] == crate::model::as_u32(mw0));
                assert(mask_fields[1] == crate::model::as_u32(mw1));
                assert(mask_fields[2] == crate::model::as_u32(mw2));
            } else {
                let mw0: int = crate::model::le32(bytes@, start as int + 40);
                let mw1: int = crate::model::le32(bytes@, start as int + 44);
                let mw2: int = crate::model::le32(bytes@, start as int + 48);
                assert(mf0 as int == mw0);
                assert(mf1 as int == mw1);
                assert(mf2 as int == mw2);
                assert(crate::model::as_u32(mw0) == mf0) by { reveal(crate::model::as_u32); };
                assert(crate::model::as_u32(mw1) == mf1) by { reveal(crate::model::as_u32); };
                assert(crate::model::as_u32(mw2) == mf2) by { reveal(crate::model::as_u32); };
                assert(mask_fields[0] == crate::model::as_u32(mw0));
                assert(mask_fields[1] == crate::model::as_u32(mw1));
                assert(mask_fields[2] == crate::model::as_u32(mw2));
                if size >= 56 {
                    let mw3: int = crate::model::le32(bytes@, start as int + 52);
                    assert(mf3 as int == mw3);
                    assert(crate::model::as_u32(mw3) == mf3) by { reveal(crate::model::as_u32); };
                    assert(mask_fields[3] == crate::model::as_u32(mw3));
                } else {
                    assert(mf3 == 0u32);
                }
            }
        }
        let ms0v: u32 = mask_fields[0];
        let ms1v: u32 = mask_fields[1];
        let ms2v: u32 = mask_fields[2];
        let ms3v: u32 = mask_fields[3];
        let (sub_masks, ok) = policy_masks(mask_fields[0], mask_fields[1], mask_fields[2],
            mask_fields[3], bits, policy.masks);
        proof {
            reveal(crate::model::masks_after_policy);
            reveal(crate::model::masks_ok_policy);
        }
        mask_fields = sub_masks;
        masks_okv = ok;
        proof {
            reveal(crate::model::masks_of_policy_rf);
            reveal(crate::model::raw_masks_policy);
            reveal(crate::model::masks_after_policy);
            reveal(crate::model::masks_ok_policy);
            assert(f.spec_raw().pos0 == start as int + size as int);
            assert(crate::model::raw_masks_policy(bytes@, start as int, f.spec_raw())
                == (ms0v, ms1v, ms2v, ms3v));
            assert((sub_masks[0], sub_masks[1], sub_masks[2], sub_masks[3])
                == crate::model::masks_after_policy(
                ms0v, ms1v, ms2v, ms3v, bits as int, policy.masks,
            ));
            assert(ok == crate::model::masks_ok_policy(sub_masks[0], sub_masks[1], sub_masks[2],
                sub_masks[3], bits as int, policy.masks));
        }
        if !ok {
            proof {
                reveal(crate::model::mh_policy);
            }
            return Err(Error::InvalidBitfields);
        }
    }

    Ok((pos, mask_fields))
}

// Keep sub-byte count arithmetic separate from R6/R7 record assembly.
fn policy_palette_counts(bits: u32, used: u32, high_count: HighCountOptions)
    -> (r: (u32, u32))
    requires bits == 1 || bits == 2 || bits == 4 || bits == 8
        || bits == 16 || bits == 24 || bits == 32,
    ensures
        r.0 as int == if bits <= 8 {
            if used != 0 { used as int } else { crate::model::pow2(bits as int) }
        } else { 0int },
        r.1 == if bits <= 8 { r.0 } else {
            if high_count == HighCountOptions::BmpTable { used } else { 0u32 }
        },
{
    let count = if bits <= 8 { if used != 0 { used } else { 1u32 << bits } } else { 0u32 };
    proof {
        reveal(crate::model::pow2);
        if bits <= 8 && used == 0 {
            assert(count as int == crate::model::pow2(bits as int)) by (bit_vector)
                requires bits == 1u32 || bits == 2u32 || bits == 4u32 || bits == 8u32,
                    count == 1u32 << bits;
        }
    }
    let table_count = if bits <= 8 { count } else {
        match high_count {
            HighCountOptions::BmpTable => used,
            HighCountOptions::Ignore | HighCountOptions::GdkBound => 0u32,
        }
    };
    (count, table_count)
}

fn dib_finish(
    bytes: &[u8],
    f: &Fields,
    file_offset: u64,
    is_bmp: bool,
    policy: Policy,
) -> (r: Result<Header, Error>)
    requires
        f.is_bmp == is_bmp,
        f.is_bmp ==> f.start == 14,
        !f.is_bmp ==> f.start == 0 && file_offset == 0,
        is_bmp == (f.mode == Mode::BmpFile),
        crate::model::r2r5_policy(bytes@, f.mode, f.start as int, policy) == Ok(f.spec_raw()),
        f.spec_n() == f.width_i as int * (f.height_v as int),
        bytes@.len() <= crate::model::max_input(),
        is_bmp ==> (file_offset as int) == crate::model::le32(bytes@, 10),
        is_bmp ==> (file_offset as int) <= usize::MAX,
        (policy.offset == OffsetOptions::Declared && is_bmp) ==> (file_offset as int)
            <= bytes@.len(),
        f.start <= 14,
        crate::model::hsize_ok(f.size as int),
        f.spec_raw().compression == 3 && !(f.spec_raw().hsize == 40 || f.spec_raw().hsize == 64)
            ==> f.spec_raw().hsize >= 52,
    ensures
        match crate::model::r6r7_policy(bytes@, f.mode, f.start as int, file_offset as int, is_bmp, f.spec_raw(), policy) {
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
        match r {
            Ok(h) => policy.header_bytes != HeaderBytesOptions::Full || policy.offset != OffsetOptions::Declared || h.pixel_start <= bytes@.len(),
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
    let (mut pos, mask_fields) = match dib_policy_masks(bytes, f, policy) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    let ghost masks_okv = true;
    // -----------------------------------------------------------------------
    // R7: palette geometry (computed in u64), offset check, availability.
    // -----------------------------------------------------------------------
    let (count, table_count) = policy_palette_counts(bits, used, policy.high_count);
    let pal_stride: usize = if core { 3 } else { 4 };
    proof {
        reveal(crate::model::table_count_of_policy);
        reveal(crate::model::table_count_of);
        assert(table_count as int == crate::model::table_count_of_policy(f.spec_raw(), policy.high_count));
    }
    let palette_length: u64 = table_count as u64 * pal_stride as u64;
    let palette_end64: u64 = pos as u64 + palette_length;
    proof {
        reveal(crate::model::palette_start_of);
        reveal(crate::model::palette_stride_of);
        reveal(crate::model::table_count_of_policy_rf);
        reveal(crate::model::table_count_of_policy);
        assert(crate::model::palette_start_of(f.spec_raw()) == pos as int);
        assert(crate::model::palette_stride_of(f.spec_raw()) == pal_stride as int);
        assert(crate::model::table_count_of_policy_rf(f.spec_raw(), policy.high_count)
            == table_count as int);
        assert(crate::model::palette_start_of(f.spec_raw()) + crate::model::table_count_of_policy_rf(
            f.spec_raw(), policy.high_count) * crate::model::palette_stride_of(f.spec_raw())
            == palette_end64 as int);
        reveal(crate::model::masks_ok_policy_rf);
        reveal(crate::model::masks_of_policy_rf);
        reveal(crate::model::raw_masks_policy);
        reveal(crate::model::masks_after_policy);
        assert(!(f.spec_raw().compression == 3 && !crate::model::masks_ok_policy_rf(bytes@, start
            as int, f.spec_raw(), policy.masks))) by {
            if compression == 3 {
                assert(masks_okv);
            }
        };
        assert(!(f.spec_raw().compression == 3 && (f.spec_raw().hsize == 40 || f.spec_raw().hsize
            == 64) && bytes@.len() - f.spec_raw().pos0 < 12));
    }
    match policy.offset {
     OffsetOptions::Declared => {
      if is_bmp {
       if (file_offset as u64) < palette_end64 {
        proof {
            reveal(crate::model::r2r7_policy);
            reveal(crate::model::r2r5_policy);
            reveal(crate::model::r6r7_policy);
            reveal(crate::model::masks_ok_policy_rf);
            reveal(crate::model::masks_of_policy_rf);
            reveal(crate::model::raw_masks_policy);
            assert(crate::model::palette_start_of(f.spec_raw()) + crate::model::table_count_of_policy_rf(
                f.spec_raw(), policy.high_count) * crate::model::palette_stride_of(f.spec_raw())
                == palette_end64 as int) by {
                reveal(crate::model::palette_start_of);
                reveal(crate::model::palette_stride_of);
                reveal(crate::model::table_count_of_policy_rf);
                reveal(crate::model::table_count_of_policy);
            };
            assert(!(f.spec_raw().compression == 3 && !crate::model::masks_ok_policy_rf(bytes@,
                start as int, f.spec_raw(), policy.masks))) by {
                reveal(crate::model::masks_ok_policy_rf);
                reveal(crate::model::masks_of_policy_rf);
                reveal(crate::model::raw_masks_policy);
                reveal(crate::model::masks_after_policy);
            };
            assert(!(f.spec_raw().compression == 3 && (f.spec_raw().hsize == 40 || f.spec_raw().hsize
                == 64) && bytes@.len() - f.spec_raw().pos0 < 12));
            assert((file_offset as int) < palette_end64 as int);
            assert(is_bmp && policy.offset == OffsetOptions::Declared);
            let pe_ico: bool = is_bmp && policy.offset == OffsetOptions::Declared;
            assert(pe_ico);
            crate::model::lemma_r6r7_policy_offset_err(
                bytes@, mode, start as int, file_offset as int, is_bmp, f.spec_raw(),
                policy, palette_end64 as int, pe_ico);
        }
        return Err(Error::InvalidPixelOffset);
       }
      }
     },
     OffsetOptions::AllowAny | OffsetOptions::Gdk => {},
    }
    proof {
        assert(!(is_bmp && policy.offset == OffsetOptions::Declared && (file_offset as u64)
            < palette_end64));
    }
    let full = match policy.header_bytes { HeaderBytesOptions::Full => true, _ => false };
    if (full || table_count != 0) && (len as u64) < palette_end64 {
        proof {
            reveal(crate::model::r2r7_policy);
            reveal(crate::model::r2r5_policy);
            reveal(crate::model::r6r7_policy_err);
            reveal(crate::model::r6r7_policy);
            assert(crate::model::palette_start_of(f.spec_raw()) + crate::model::table_count_of_policy_rf(
                f.spec_raw(), policy.high_count) * crate::model::palette_stride_of(f.spec_raw())
                == palette_end64 as int) by {
                reveal(crate::model::palette_start_of);
                reveal(crate::model::palette_stride_of);
                reveal(crate::model::table_count_of_policy_rf);
                reveal(crate::model::table_count_of_policy);
            };
            assert(!(f.spec_raw().compression == 3 && !crate::model::masks_ok_policy_rf(bytes@,
                start as int, f.spec_raw(), policy.masks))) by {
                reveal(crate::model::masks_ok_policy_rf);
                reveal(crate::model::masks_of_policy_rf);
                reveal(crate::model::raw_masks_policy);
                reveal(crate::model::masks_after_policy);
            };
            assert(!(f.spec_raw().compression == 3 && (f.spec_raw().hsize == 40 || f.spec_raw().hsize
                == 64) && bytes@.len() - f.spec_raw().pos0 < 12));
            crate::model::lemma_r6r7_policy_palette_err(
                bytes@, mode, start as int, file_offset as int, is_bmp, f.spec_raw(),
                policy, palette_end64 as int);
        }
        return Err(Error::TruncatedPalette);
    }
    proof {
        assert(table_count == 0 || palette_end64 <= len as u64);
        assert(palette_end64 <= usize::MAX as u64) by (nonlinear_arith)
            requires table_count == 0 || palette_end64 <= len as u64,
                palette_end64 as int == pos as int + table_count as int * pal_stride as int,
                pos <= 150, len as u64 <= usize::MAX as u64;
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
        reveal(crate::model::mh_policy);
        reveal(crate::model::r2r5_policy);
        reveal(crate::model::r6r7_policy);
        reveal(crate::model::r6r7_result_policy);
        reveal(crate::model::r2r7_policy);
        reveal(crate::model::pow2);
        reveal(crate::model::as_u32);
        reveal(crate::model::stride_of);
        reveal(crate::model::is_ico);
        reveal(crate::model::exact_tail);
        reveal(crate::model::visible_height_of);
        reveal(crate::model::masks_ok_policy);
        reveal(crate::model::masks_after_policy);
        reveal(crate::model::masks_ok);
        reveal(crate::model::table_count_of_policy);
        reveal(crate::model::table_count_of);
        reveal(crate::model::palette_start_of);
        reveal(crate::model::palette_stride_of);
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
        let rawv: (u32, u32, u32, u32) = crate::model::raw_masks_policy(bytes@, start as int, rf);
        assert(rawv == (0u32, 0u32, 0u32, 0u32) || compression == 3);
        assert(compression != 3 || masks_okv);
        let ms: (u32, u32, u32, u32) = crate::model::masks_of_policy_rf(bytes@,
            start as int, rf, policy.masks);
        assert(ms == (mask_fields[0], mask_fields[1], mask_fields[2], mask_fields[3])) by {
            reveal(crate::model::masks_of_policy_rf);
            if compression == 3 {
                reveal(crate::model::raw_masks_policy);
                reveal(crate::model::masks_after_policy);
            } else {
                assert((mask_fields[0], mask_fields[1], mask_fields[2], mask_fields[3])
                    == (0u32, 0u32, 0u32, 0u32));
            }
        };
        let m = crate::model::r6r7_result_policy(
            bytes@,
            mode,
            start as int,
            file_offset as int,
            is_bmp,
            rf,
            ms,
            table_count as int,
        );
        assert(!(rf.compression == 3 && (rf.hsize == 40 || rf.hsize == 64) && bytes@.len() - rf.pos0
            < 12));
        assert(compression == 3 ==> ms == crate::model::masks_after_policy(rawv.0, rawv.1, rawv.2,
            rawv.3, rf.depth, policy.masks));
        assert(compression != 3 ==> masks_okv);
        assert(compression == 3 ==> masks_okv == crate::model::masks_ok_policy(ms.0, ms.1, ms.2,
            ms.3, rf.depth, policy.masks));
        assert((table_count as int) == crate::model::table_count_of_policy_rf(rf, policy.high_count)) by {
            reveal(crate::model::table_count_of_policy_rf);
            reveal(crate::model::table_count_of_policy);
        };
        assert(crate::model::palette_start_of(rf) + crate::model::table_count_of_policy_rf(rf,
            policy.high_count) * crate::model::palette_stride_of(rf) == palette_end64 as int) by {
            reveal(crate::model::palette_start_of);
            reveal(crate::model::palette_stride_of);
            reveal(crate::model::table_count_of_policy_rf);
        };
        assert(!(is_bmp && policy.offset == OffsetOptions::Declared && (file_offset as u64)
            < palette_end64));
        assert((policy.header_bytes == HeaderBytesOptions::ParsedFields && table_count == 0) || (len as u64) >= palette_end64);
        assert(!(is_bmp && policy.offset == OffsetOptions::Declared && (file_offset as int)
            < crate::model::palette_start_of(rf) + crate::model::table_count_of_policy_rf(rf,
            policy.high_count) * crate::model::palette_stride_of(rf)));
        assert((policy.header_bytes == HeaderBytesOptions::ParsedFields && table_count == 0) || !((len as int) < crate::model::palette_start_of(rf)
            + crate::model::table_count_of_policy_rf(rf, policy.high_count)
            * crate::model::palette_stride_of(rf)));
        crate::model::lemma_r6r7_policy_ok(
            bytes@, mode, start as int, file_offset as int, is_bmp, rf, policy, m,
        );
        assert(crate::model::r6r7_policy(bytes@, mode, start as int, file_offset as int, is_bmp, rf, policy)
            == Ok(m));
        reveal(hdr_matches);
        reveal(crate::model::r6r7_result_policy);
        assert(hdr_matches(m, h)) by {
            reveal(crate::model::r6r7_result_policy);
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
                reveal(crate::model::r6r7_result_policy);
                if is_bmp {
                    assert((file_offset as usize) as int == file_offset as int) by (nonlinear_arith)
                        requires
                            file_offset as int <= usize::MAX,
                    ;
                    assert(m.pixel_start == file_offset as int);
                    assert(h.pixel_start == file_offset as usize);
                } else {
                    assert(m.pixel_start == m.palette_end);
                    assert(h.pixel_start == h.palette_end);
                }
            }
            reveal(crate::model::masks_of_policy_rf);
            reveal(crate::model::raw_masks_policy);
            assert(m.mask_r == h.spec_mask_r()) by {
                reveal(crate::model::raw_masks_policy);
            }
            assert(m.mask_g == h.spec_mask_g()) by {
                reveal(crate::model::raw_masks_policy);
            }
            assert(m.mask_b == h.spec_mask_b()) by {
                reveal(crate::model::raw_masks_policy);
            }
            assert(m.mask_a == h.spec_mask_a()) by {
                reveal(crate::model::raw_masks_policy);
            }
            assert(m.has_masks == (h.compression == 3u32));
            assert(m.core == h.core);
            assert(m.is_bmp == h.is_bmp);
            assert(m.top_down == h.top_down);
            assert(m.n == h.spec_n());
            assert(m.stride == h.spec_stride());
        }
        crate::model::lemma_r6r7_policy_ok_bounds(
            bytes@,
            mode,
            start as int,
            file_offset as int,
            is_bmp,
            rf,
            policy,
            m,
        );
        assert(crate::model::mh_bounds(m));
        assert(header_inv(bytes@, h)) by {
            reveal(header_inv);
            reveal(header_record);
            assert(crate::model::mh_bounds(header_record(h)));
            assert(h.table_count == 0 || h.palette_end <= bytes@.len());
            assert(h.count <= h.table_count);
            assert(h.count > 0 ==> h.palette_start + (h.count as int) * (h.pal_stride as int) <= bytes@.len()) by (nonlinear_arith)
                requires h.count <= h.table_count, h.pal_stride >= 0,
                    h.palette_start as int + h.table_count as int * h.pal_stride as int == h.palette_end as int,
                    h.table_count == 0 || h.palette_end <= bytes@.len();
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
            assert(crate::model::depth_ok(bits as int) || bits == 2u32);
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
        assert(policy.header_bytes != HeaderBytesOptions::Full || policy.offset != OffsetOptions::Declared || h.pixel_start <= bytes@.len()) by {
            if is_bmp {
                assert(h.pixel_start == file_offset as usize);
                if policy.offset == OffsetOptions::Declared {
                    assert((file_offset as int) <= bytes@.len());
                }
            } else {
                assert(h.pixel_start == palette_start + (table_count as usize) * pal_stride);
                assert(h.pixel_start == h.palette_end);
                assert(h.table_count == 0 || h.palette_end <= bytes@.len());
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
    alloc_output_n(h.n, steps)
}

/// R8 `offset = Gdk` blank fill: overwrite every output pixel with an
/// initialized black — transparent (alpha 0) when `transparent`, else opaque
/// (alpha 255). The result is exactly the model's `zero_bytes` (transparent) or
/// `opaque_fill` (opaque) over the `4N` buffer, and the write is charged to the
/// same ghost counter as the rest of the fill work.
fn fill_blank(out: &mut Vec<u8>, transparent: bool, steps: &mut Ghost<nat>)
    requires
        old(out)@.len() % 4 == 0,
    ensures
        final(out)@.len() == old(out)@.len(),
        final(out)@ == (if transparent {
            crate::model::zero_bytes(old(out)@.len() as int)
        } else {
            crate::model::opaque_fill(old(out)@.len() as int)
        }),
        old(steps)@ <= final(steps)@ <= old(steps)@ + old(out)@.len(),
{
    let n = out.len();
    proof {
        reveal(crate::model::zero_bytes);
        reveal(crate::model::opaque_fill);
    }
    let fill: u8 = if transparent {
        0u8
    } else {
        255u8
    };
    let mut i: usize = 0;
    while i < n
        invariant
            i <= n,
            n == out@.len(),
            old(out)@.len() == n,
            forall|k: int| 0 <= k < i ==> #[trigger] out@[k] == (if transparent {
                0u8
            } else if k % 4 == 3 {
                255u8
            } else {
                0u8
            }),
            forall|k: int| i <= k < n ==> out@[k] == old(out)@[k],
            old(steps)@ <= steps@ <= old(steps)@ + i,
        decreases n - i,
    {
        if transparent {
            out.set(i, 0u8);
        } else if i % 4 == 3 {
            out.set(i, 255u8);
        } else {
            out.set(i, 0u8);
        }
        proof { steps@ = steps@ + 1; }
        i = i + 1;
    }
    proof {
        broadcast use vstd::seq::group_seq_lemmas;
        assert(final(out)@ =~= (if transparent {
            crate::model::zero_bytes(n as int)
        } else {
            crate::model::opaque_fill(n as int)
        })) by {
            assert forall|k: int| 0 <= k < n implies #[trigger] final(out)@[k] == (if transparent {
                crate::model::zero_bytes(n as int)
            } else {
                crate::model::opaque_fill(n as int)
            })[k] by {
                reveal(crate::model::zero_bytes);
                reveal(crate::model::opaque_fill);
            }
        }
    }
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
        assert(bits == 1u32 || bits == 2u32 || bits == 4u32 || bits == 8u32 || bits == 16u32 || bits == 24u32
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
        let mask = if bits == 1 { 1u32 } else if bits == 2 { 3u32 } else if bits == 4 { 15u32 } else { 255u32 };
        let byte = rd(b, p);
        let ix = ((byte as u32) >> shift) & mask;
        proof {
            reveal(header_record);
            reveal(crate::model::raw_conv);
            reveal(crate::model::sub_index);
            reveal(crate::model::sub_shift);
            reveal(crate::model::sub_mask);
            assert(bits == 1u32 || bits == 2u32 || bits == 4u32 || bits == 8u32);
            assert(crate::model::sub_mask(bits as int) == mask) by {
                if bits == 1 {
                    assert(crate::model::sub_mask(bits as int) == 1u32);
                } else if bits == 2 {
                    assert(crate::model::sub_mask(bits as int) == 3u32);
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
        final(gcur).0 as int >= pos0 as int,
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
            pos as int <= b@.len(), pos as int >= pos0 as int,
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

fn decode_raw(b: &[u8], h: &Header, out: &mut Vec<u8>, p0: usize, steps: &mut Ghost<nat>) -> (r: Result<usize, Error>)
    requires
        header_inv(b@, *h), header_inv_light(h),
        old(out)@.len() == 4 * h.spec_n(),
        old(out)@ == crate::model::zero_bytes(4 * h.spec_n()),
        h.compression == 0 || h.compression == 3,
        0 <= p0 <= b@.len(),
        crate::model::raw_reachable(b@, hrec(h), p0 as int, 0, 0),
    ensures
        final(steps)@ <= old(steps)@ + h.spec_n() + h.spec_height() + 1,
        final(out)@.len() == 4 * h.spec_n(),
        match r { Ok(p) => p0 <= p <= b@.len(), Err(_) => true },
        // T9/T6: the returned result and buffer are exactly `MR`'s raw
        // transition replayed onto the zero fill.
        raw_done_eq(b@, hrec(h), p0 as int, final(out)@, r),
        // T6 on the whole image, at the manifest's quantification (start state
        // `(pixel_start, 0, 0)`): the exact first error and ghost cursor/position
        // or the exact success cursor, and the exact pixels.
        match r {
            Ok(p) => crate::model::raw_gcur(
                b@,
                hrec(h),
                p0 as int,
                0,
                0,
            ) == Ok((p as int, 0, h.spec_height())) && p as int <= b@.len(),
            Err(e) => exists|gp: int, gx: int, gy: int|
                crate::model::raw_gcur(b@, hrec(h), p0 as int, 0, 0) == Err((
                    crate::model::RawErr::Api(e),
                    gp,
                    gx,
                    gy,
                )),
        },
        final(out)@ == crate::model::bytes_replay(
            crate::model::zero_bytes(4 * h.spec_n()),
            crate::model::raw_writes(b@, hrec(h), p0 as int, 0, 0),
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
        reveal(header_inv);
        reveal(header_inv_light);
        reveal(header_record);
        assert(crate::model::raw_reachable(b@, hrec(h), p0 as int, 0, 0)) by {
            reveal(crate::model::raw_reachable);
            reveal(crate::model::stride_of);
            reveal(crate::model::mask_depth_ok);
            assert(hrec(h).stride == h.spec_stride());
            assert(hrec(h).pixel_start == h.spec_pixel_start());
            assert(p0 as int <= b@.len());
        };
    }
    let mut gcur: (usize, usize, usize) = (p0, 0, 0);
    let r = raw_loop(b, h, out, p0, &mut gcur, steps);
    proof {
        reveal(crate::model::raw_full);
        reveal(crate::model::raw_gcur);
        match r {
            Ok(()) => {
                assert(crate::model::raw_gcur(b@, hrec(h), p0 as int, 0, 0) == Ok((
                    gcur.0 as int,
                    gcur.1 as int,
                    gcur.2 as int,
                )));
                crate::model::lemma_raw_gcur_ok_shape(b@, hrec(h), p0 as int, 0, 0);
                assert(exists|pc: int| crate::model::raw_gcur(
                    b@,
                    hrec(h),
                    p0 as int,
                    0,
                    0,
                ) == Ok((pc, 0, h.spec_height())));
                assert(crate::model::raw_gcur(b@, hrec(h), p0 as int, 0, 0) == Ok((
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
        assert(gcur.0 as int >= p0 as int);
    }
    proof {
        // T9/T6 bridge: fold `raw_loop`'s two projections back into `mr_raw_out`.
        reveal(raw_done_eq);
        reveal(crate::model::mr_raw_out);
        reveal(crate::model::raw_full);
        crate::model::lemma_raw_full_proj(b@, hrec(h), p0 as int, 0, 0);
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
    Ghost(base): Ghost<Seq<u8>>,
) -> (r: Result<(), Error>)
    requires
        header_inv(b@, *h), header_inv_light(h),
        (y as int) < h.spec_height(),
        x + q <= h.spec_width(), q <= 255,
        old(out)@.len() == 4 * h.spec_n(),
        old(out)@ == crate::model::bytes_replay(base, gw),
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
                    base,
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
                    base,
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
                base,
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
                base,
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
    Ghost(base): Ghost<Seq<u8>>,
) -> (r: Result<(), Error>)
    requires
        header_inv(b@, *h), header_inv_light(h),
        (y as int) < h.spec_height(),
        x + q <= h.spec_width(), 1 <= q <= 255,
        old(out)@.len() == 4 * h.spec_n(),
        old(out)@ == crate::model::bytes_replay(base, gw),
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
                    base,
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
                    base,
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
            out@ == crate::model::bytes_replay(base, gj),
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
            crate::model::lemma_bytes_replay_push(base, gj, w);
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
    let pr = paint_abs(b, h, out, pay, *x, *y, q, Ghost(gw0), steps, Ghost(crate::model::pal_zero_bytes(b@, hrec(h))));
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
        let pr = paint_run(b, h, out, code, *x, *y, quantity, Ghost(gw0), steps, Ghost(crate::model::pal_zero_bytes(b@, hrec(h))));
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

fn count_byte_bits(v: u8) -> (r: u32)
    ensures r == crate::model::popcount_byte(v), r <= 8, (r == 0) == (v == 0),
{
    let r = (v & 1) as u32 + ((v >> 1) & 1) as u32 + ((v >> 2) & 1) as u32 + ((v >> 3) & 1) as u32
        + ((v >> 4) & 1) as u32 + ((v >> 5) & 1) as u32 + ((v >> 6) & 1) as u32 + ((v >> 7) & 1) as u32;
    proof {
        reveal(crate::model::popcount_byte);
        assert(r <= 8 && (r == 0) == (v == 0)) by (bit_vector)
            requires r == (v & 1) as u32 + ((v >> 1) & 1) as u32 + ((v >> 2) & 1) as u32 + ((v >> 3) & 1) as u32
                + ((v >> 4) & 1) as u32 + ((v >> 5) & 1) as u32 + ((v >> 6) & 1) as u32 + ((v >> 7) & 1) as u32;
    }
    r
}

fn count_mask_bits(mask: u32) -> (r: u32)
    ensures r == crate::model::popcount_word(mask), r <= 32, (r == 0) == (mask == 0),
{
    let a = count_byte_bits(mask as u8);
    let b = count_byte_bits((mask >> 8) as u8);
    let c = count_byte_bits((mask >> 16) as u8);
    let d = count_byte_bits((mask >> 24) as u8);
    proof {
        reveal(crate::model::popcount_word);
        assert(((mask as u8) == 0 && ((mask >> 8) as u8) == 0
            && ((mask >> 16) as u8) == 0 && ((mask >> 24) as u8) == 0) == (mask == 0)) by (bit_vector);
    }
    a + b + c + d
}

fn channel_policy(word: u32, mask: u32, scaling: ScalingOptions) -> (r: u8)
    ensures r == crate::model::channel_scaled(word, mask, scaling),
{
    proof { reveal(crate::model::channel_scaled); }
    if mask == 0 { return 255; }
    match scaling {
        ScalingOptions::Round => channel(word, mask),
        ScalingOptions::Image => {
            let shift = mask.trailing_zeros();
            let compact = mask >> shift;
            let width = 32 - compact.leading_zeros();
            if width <= 8 { channel(word, mask) }
            else { (((word & mask) >> shift) >> (width - 8)) as u8 }
        },
        ScalingOptions::Gdk => {
            let shift = mask.trailing_zeros();
            let count = count_mask_bits(mask);
            let n = if count > 8 { 8u32 } else { count };
            let skip = if count > 8 { count - 8 } else { 0u32 };
            let value = ((word & mask) as u64) >> (shift + skip);
            let high = value << (8 - n);
            let low = if n >= 4 { value >> (2 * n - 8) } else { 0u64 };
            (high | low) as u8
        },
    }
}

fn raw_policy_word(b: &[u8], h: &Header, pos: usize, x: usize, line: usize) -> (word: u32)
    requires header_inv(b@, *h), header_inv_light(h), h.bits == 16 || h.bits == 32,
        h.compression == 0 || h.compression == 3,
        x < h.spec_width(), pos + line <= b@.len(), line as int == h.spec_stride(),
    ensures word as int == if h.bits == 16 {
        crate::model::le16(b@, pos as int + x as int * h.spec_depth() / 8)
    } else { crate::model::le32(b@, pos as int + x as int * h.spec_depth() / 8) },
{
    proof {
        reveal(header_inv_light);
        crate::model::lemma_sample_within_stride(h.spec_width(), h.spec_depth(), x as int);
        assert(x as int * (h.bits as int / 8) == x as int * h.spec_depth() / 8) by (nonlinear_arith)
            requires h.bits == 16 || h.bits == 32;
        assert(x as int * (h.bits as int / 8) + h.bits as int / 8 <= line as int);
    }
    let at = pos + x * (h.bits as usize / 8);
    if h.bits == 16 { rd16(b, at) as u32 } else { rd32(b, at) }
}

fn raw_sample_scaled(b: &[u8], h: &Header, pos: usize, x: usize, line: usize, scaling: ScalingOptions, opaque: bool)
    -> (r: Result<[u8; 4], Error>)
    requires header_inv(b@, *h), header_inv_light(h), h.compression == 0 || h.compression == 3,
        x < h.spec_width(), pos + line <= b@.len(), line as int == h.spec_stride(),
    ensures match r {
        Ok(c) => c@ == crate::model::raw_scaled_conv(b@, hrec(h), pos as int, x as int, scaling, opaque)
            && crate::model::raw_index_ok(b@, hrec(h), pos as int, x as int),
        Err(e) => e == Error::PaletteIndexOutOfRange && !crate::model::raw_index_ok(b@, hrec(h), pos as int, x as int),
    },
{
    proof { reveal(crate::model::raw_scaled_conv); reveal(hrec); reveal(header_record); reveal(header_inv_light); }
    if h.compression == 3 || h.bits == 16 {
        proof { reveal(crate::model::raw_index_ok); }
        let word = raw_policy_word(b, h, pos, x, line);
        let c = if h.compression == 3 {
            [channel_policy(word, h.masks[0], scaling), channel_policy(word, h.masks[1], scaling),
                channel_policy(word, h.masks[2], scaling), if opaque && h.bits == 16 { 255u8 } else { channel_policy(word, h.masks[3], scaling) }]
        } else {
            [channel_policy(word, 0x7c00, scaling), channel_policy(word, 0x3e0, scaling),
                channel_policy(word, 0x1f, scaling), 255u8]
        };
        proof {
            let want = crate::model::raw_scaled_conv(b@, hrec(h), pos as int, x as int, scaling, opaque);
            assert(c@ =~= want);
        }
        Ok(c)
    } else { raw_sample(b, h, pos, x, line) }
}

// Locate a complete source row without adding a possibly out-of-file offset.
fn policy_row_start(len: usize, start: usize, advance: u64, line: usize) -> (r: Option<usize>)
    ensures match r {
        Some(pos) => pos as int == start as int + advance as int && pos + line <= len,
        None => start as int + advance as int + line as int > len,
    },
{
    if start > len { return None; }
    let remaining = len - start;
    if advance > remaining as u64 { return None; }
    let used = advance as usize;
    if line > remaining - used { return None; }
    Some(start + used)
}

fn decode_raw_zero(b: &[u8], h: &Header, out: &mut Vec<u8>, start: usize, zero: bool, black: bool, scaling: ScalingOptions, opaque: bool, steps: &mut Ghost<nat>)
    -> (r: (Result<u64, Error>, bool))
    requires header_inv(b@, *h), header_inv_light(h),
        h.compression == 0 || h.compression == 3, old(out)@.len() == 4 * h.spec_n(),
    ensures
        final(out)@.len() == 4 * h.spec_n(),
        final(steps)@ as int <= old(steps)@ as int + 4 * h.spec_n() + h.spec_n() + h.spec_height() + 1,
        r.1 == crate::model::raw_zero_out(b@, hrec(h), start as int, zero, black, scaling, opaque).1,
        match r.0 {
            Ok(advance) => advance <= 268435456
                && crate::model::raw_zero_out(b@, hrec(h), start as int, zero, black, scaling, opaque).0
                    == Ok((start as int + advance as int, final(out)@)),
            Err(e) => crate::model::raw_zero_out(b@, hrec(h), start as int, zero, black, scaling, opaque).0 == Err(e),
        },
{
    let ghost s0 = steps@;
    let ghost m = hrec(h);
    fill_blank(out, !zero || h.bits == 32, steps);
    let line = stride(h.width, h.bits);
    let ghost initial = crate::model::raw_zero_fill(m, zero);
    let ghost full = crate::model::raw_zero_full(b@, m, start as int, 0, 0, zero, black, scaling, opaque);
    let ghost mut gw = Seq::<crate::model::Write>::empty();
    let mut x = 0usize;
    let mut y = 0usize;
    let mut advance = 0u64;
    let mut recovered = false;
    proof {
        reveal(hrec); reveal(header_record); reveal(header_inv_light);
        reveal(crate::model::raw_zero_fill);
        assert(initial == out@);
        assert(gw + full.1 == full.1) by { broadcast use vstd::seq_lib::group_seq_lib_default; }
    }
    loop
        invariant
            header_inv(b@, *h), header_inv_light(h), m == hrec(h),
            h.compression == 0 || h.compression == 3,
            line as int == h.spec_stride(), line <= 32768,
            x <= h.spec_width(), y <= h.spec_height(),
            advance as int == y as int * line as int,
            advance <= 268435456,
            s0 == old(steps)@,
            steps@ as int <= s0 as int + 4 * h.spec_n() + y as int * (h.spec_width() + 1) + x as int,
            out@.len() == 4 * h.spec_n(),
            out@ == crate::model::bytes_replay(initial, gw),
            initial == crate::model::raw_zero_fill(m, zero),
            full == crate::model::raw_zero_full(b@, m, start as int, 0, 0, zero, black, scaling, opaque),
            full.0 == crate::model::raw_zero_full(b@, m, start as int + advance as int, x as int, y as int, zero, black, scaling, opaque).0,
            full.1 == gw + crate::model::raw_zero_full(b@, m, start as int + advance as int, x as int, y as int, zero, black, scaling, opaque).1,
            full.2 == (recovered || crate::model::raw_zero_full(b@, m, start as int + advance as int, x as int, y as int, zero, black, scaling, opaque).2),
            y == h.spec_height() ==> x == 0,
        decreases h.spec_height() - y, h.spec_width() - x,
    {
        proof {
            reveal_with_fuel(crate::model::raw_zero_full, 1);
            reveal(crate::model::raw_zero_out);
            reveal(header_inv_light);
            *steps = Ghost(steps@ + 1);
            assert(steps@ as int <= s0 as int + 4 * h.spec_n() + h.spec_n() + h.spec_height() + 1)
                by (nonlinear_arith) requires
                    steps@ as int <= s0 as int + 4 * h.spec_n() + y as int * (h.spec_width() + 1) + x as int + 1,
                    y <= h.spec_height(), x <= h.spec_width(),
                    y == h.spec_height() ==> x == 0,
                    h.spec_width() >= 1, h.spec_n() == h.spec_width() * h.spec_height();
        }
        if y >= h.height as usize { return (Ok(advance), recovered); }
        let row = policy_row_start(b.len(), start, advance, line as usize);
        if x >= h.width as usize || row.is_none() {
            if x < h.width as usize {
                if !zero { return (Err(Error::TruncatedPixels), recovered); }
                recovered = true;
            }
            proof {
                assert(advance as int + line as int == (y as int + 1) * line as int) by (nonlinear_arith)
                    requires advance as int == y as int * line as int;
                assert(advance as int + line as int <= 268435456) by (nonlinear_arith)
                    requires y < h.spec_height(), h.spec_height() <= 8192, line <= 32768,
                        advance as int == y as int * line as int;
                assert(y as int * (h.spec_width() + 1) + x as int + 1
                    <= (y as int + 1) * (h.spec_width() + 1)) by (nonlinear_arith)
                    requires x <= h.spec_width();
            }
            advance = advance + line as u64;
            y = y + 1; x = 0;
        } else {
            let pos = row.unwrap();
            let c = match raw_sample_scaled(b, h, pos, x, line as usize, scaling, opaque) {
                Ok(c) => c,
                Err(e) => {
                    if !black { return (Err(e), recovered); }
                    recovered = true;
                    [0u8, 0u8, 0u8, 255u8]
                },
            };
            let offset = out_offset(h, x, y);
            let ghost w = crate::model::Write {
                cell: crate::model::cell_of(m, x as int, y as int), px: c@,
            };
            let ghost tail = crate::model::raw_zero_full(b@, m, start as int + advance as int, x as int + 1, y as int, zero, black, scaling, opaque);
            put(out, offset, c);
            proof {
                reveal(crate::model::replay_one); reveal(crate::model::replay_bytes_at);
                reveal(crate::model::cell_of); reveal(crate::model::orient_cell); reveal(map_row_spec);
                crate::model::lemma_bytes_replay_push(initial, gw, w);
                vstd::seq_lib::lemma_concat_associative(gw, seq![w], tail.1);
                assert(gw.push(w) == gw + seq![w]) by { broadcast use vstd::seq_lib::group_seq_lib_default; }
                gw = gw.push(w);
            }
            x = x + 1;
        }
    }
}

// A scalar quantity bound keeps clipped positions out of all painter loops.
fn rle_clip_count(width: usize, x: usize, q: usize) -> (r: usize)
    requires x <= width,
    ensures r <= q, x + r <= width,
        r as int == crate::model::clip_paint_count(width as int, x as int, q as int),
{
    if q > width - x { width - x } else { q }
}

fn policy_packet_index(b: &[u8], bits: u32, absolute: bool, value: usize, j: usize, q: usize)
    -> (r: u32)
    requires bits == 4 || bits == 8, j < q <= 255, b@.len() <= 67108864,
        !absolute ==> value <= 255,
        absolute ==> value as int + crate::model::rle_payload(q as int, bits as int) <= b@.len(),
    ensures r <= 255,
        r as int == crate::model::packet_index(b@, bits as int, absolute, value as int, j as int),
{
    proof { reveal(crate::model::rle_payload); reveal(crate::model::packet_index); }
    let byte = if absolute {
        if bits == 8 { rd(b, value + j) } else {
            proof {
                assert(j / 2 < (q + 1) / 2) by (nonlinear_arith) requires j < q;
            }
            rd(b, value + j / 2)
        }
    } else { value as u8 };
    let ix = if bits == 8 { byte as u32 } else {
        ((byte >> (if j % 2 == 0 { 4 } else { 0 })) & 15) as u32
    };
    proof {
        reveal(crate::model::abs_index); reveal(crate::model::run_index);
        if bits == 4 {
            if j % 2 == 0 {
                assert(((byte >> 4) & 15) as int == byte as int / 16) by (bit_vector);
            } else {
                assert(((byte >> 0) & 15) as int == byte as int % 16) by (bit_vector);
            }
        }
    }
    ix
}

fn paint_black_packet(b: &[u8], h: &Header, out: &mut Vec<u8>, absolute: bool,
    value: usize, x: usize, y: usize, q: usize,
    Ghost(gw): Ghost<Seq<crate::model::Write>>, steps: &mut Ghost<nat>, Ghost(initial): Ghost<Seq<u8>>) -> (event: bool)
    requires header_inv(b@, *h), header_inv_light(h),
        h.bits == 4 || h.bits == 8, x + q <= h.spec_width(), y < h.spec_height(), q <= 255,
        !absolute ==> value <= 255,
        absolute ==> value as int + crate::model::rle_payload(q as int, h.spec_depth()) <= b@.len(),
        old(out)@.len() == 4 * h.spec_n(),
        old(out)@ == crate::model::bytes_replay(initial, gw),
    ensures
        final(out)@.len() == 4 * h.spec_n(), final(steps)@ == old(steps)@ + q,
        event == crate::model::black_packet_event(b@, hrec(h), absolute, value as int, q as int),
        final(out)@ == crate::model::bytes_replay(initial,
            gw + crate::model::black_packet_writes(b@, hrec(h), absolute, value as int, x as int, y as int, q as int)),
{
    let ghost m = hrec(h);
    let ghost base = initial;
    let ghost s0 = steps@;
    let mut j = 0usize;
    let mut recovered = false;
    proof { reveal(crate::model::black_packet_writes); reveal(crate::model::black_packet_event); }
    while j < q
        invariant
            header_inv(b@, *h), header_inv_light(h), m == hrec(h),
            h.bits == 4 || h.bits == 8, x + q <= h.spec_width(), y < h.spec_height(), q <= 255,
            !absolute ==> value <= 255,
            absolute ==> value as int + crate::model::rle_payload(q as int, h.spec_depth()) <= b@.len(),
            j <= q, out@.len() == 4 * h.spec_n(), s0 == old(steps)@, steps@ == s0 + j,
            base == initial,
            recovered == crate::model::black_packet_event(b@, m, absolute, value as int, j as int),
            out@ == crate::model::bytes_replay(base,
                gw + crate::model::black_packet_writes(b@, m, absolute, value as int, x as int, y as int, j as int)),
        decreases q - j,
    {
        proof { *steps = Ghost(steps@ + 1); reveal(hrec); reveal(header_record); }
        let ix = policy_packet_index(b, h.bits, absolute, value, j, q);
        let c = match colour(b, h.palette_start, h.pal_stride, h.count, ix) {
            Ok(c) => c,
            Err(_) => { recovered = true; [0u8, 0u8, 0u8, 255u8] },
        };
        let offset = out_offset(h, x + j, y);
        let ghost w = crate::model::Write { cell: crate::model::cell_of(m, x as int + j as int, y as int), px: c@ };
        let ghost before = crate::model::black_packet_writes(b@, m, absolute, value as int, x as int, y as int, j as int);
        put(out, offset, c);
        proof {
            reveal(crate::model::black_packet_writes); reveal(crate::model::black_packet_event);
            reveal(crate::model::replay_one); reveal(crate::model::cell_of);
            reveal(crate::model::orient_cell); reveal(map_row_spec);
            let after = crate::model::black_packet_writes(b@, m, absolute, value as int, x as int, y as int, j as int + 1);
            assert(after == before.push(w)) by {
                assert forall|k: int| 0 <= k < after.len() implies after[k] == #[trigger] before.push(w)[k] by {
                    if k == j as int {} else {}
                }
                vstd::seq::lemma_seq_ext_equal(after, before.push(w));
            }
            assert((gw + before).push(w) == gw + after) by { broadcast use vstd::seq_lib::group_seq_lib_default; }
            crate::model::lemma_bytes_replay_push(base, gw + before, w);
            assert(recovered == crate::model::black_packet_event(b@, m, absolute, value as int, j as int + 1)) by {
                if ix >= h.count {
                    assert(crate::model::packet_index(b@, m.depth, absolute, value as int, j as int) >= m.count);
                } else {
                    assert forall|k: int| 0 <= k < j as int + 1 && crate::model::packet_index(b@, m.depth, absolute, value as int, k) >= m.count
                        implies k < j by { }
                }
            }
        }
        j = j + 1;
    }
    recovered
}

spec fn rle_position(width: int, height: int, x: int, y: int) -> int {
    if y >= height { width * height } else { y * width + x }
}

fn policy_delta_target(width: usize, height: usize, x: usize, y: usize, dx: usize, dy: usize,
    delta: RleDeltaOptions) -> (r: Result<(usize, usize, bool), Error>)
    requires 1 <= width <= 8192, 1 <= height <= 8192, x <= width, y <= height, dx <= 255, dy <= 255,
    ensures match r {
        Err(e) => e == Error::RleBounds && delta == RleDeltaOptions::Inside
            && (x + dx >= width || y + dy >= height),
        Ok((nx, ny, event)) => {
            &&& x <= nx <= width && y <= ny <= height
            &&& nx as int == if x + dx > width { width as int } else { x + dx }
            &&& ny as int == if y + dy > height { height as int } else { y + dy }
            &&& (delta == RleDeltaOptions::Inside ==> nx < width && ny < height)
            &&& event == (x + dx > width || y + dy > height)
            &&& rle_position(width as int, height as int, x as int, y as int)
                <= rle_position(width as int, height as int, nx as int, ny as int)
        },
    },
{
    match delta {
        RleDeltaOptions::Inside => {
            if x + dx >= width || y + dy >= height { return Err(Error::RleBounds); }
        },
        RleDeltaOptions::ClampStop => {},
    }
    let nx = if x + dx > width { width } else { x + dx };
    let ny = if y + dy > height { height } else { y + dy };
    let event = x + dx > width || y + dy > height;
    proof {
        reveal(rle_position);
        assert(rle_position(width as int, height as int, x as int, y as int)
            <= rle_position(width as int, height as int, nx as int, ny as int)) by (nonlinear_arith)
            requires x <= nx <= width, y <= ny <= height, width >= 1,
                rle_position(width as int, height as int, x as int, y as int)
                    == if y >= height { width as int * height as int } else { y as int * width as int + x as int },
                rle_position(width as int, height as int, nx as int, ny as int)
                    == if ny >= height { width as int * height as int } else { ny as int * width as int + nx as int };
    }
    Ok((nx, ny, event))
}

fn rle_clip_absolute(b: &[u8], h: &Header, out: &mut Vec<u8>, p: usize, x: usize, y: usize,
    overrun: RleOverrunOptions, eol: RleEolOptions, eob: RleEobOptions, black: bool, delta: RleDeltaOptions, Ghost(gw): Ghost<Seq<crate::model::Write>>, steps: &mut Ghost<nat>, Ghost(initial): Ghost<Seq<u8>>)
    -> (r: (Result<bool, Error>, usize, usize, usize, bool, Ghost<Seq<crate::model::Write>>, usize))
    requires
        p + 2 <= b@.len(), b@[p as int] == 0, b@[p as int + 1] >= 3,
        eob == RleEobOptions::AtInputEnd ==> y < h.spec_height(),
        header_inv(b@, *h), header_inv_light(h),
        h.compression == 1 || h.compression == 2,
        p <= b@.len(), x <= h.spec_width(), y <= h.spec_height(),
        old(out)@.len() == 4 * h.spec_n(),
        old(out)@ == crate::model::bytes_replay(initial, gw),
    ensures
        r.0 is Ok ==> r.6 as int == r.5@.len(),
        r.0 == Ok(true) ==> r.6 == 0,
        r.0 is Ok ==> r.6 as int + rle_position(h.spec_width(), h.spec_height(), x as int, y as int)
            <= rle_position(h.spec_width(), h.spec_height(), r.2 as int, r.3 as int),
        p <= r.1 <= b@.len(), r.2 <= h.spec_width(), r.3 <= h.spec_height(),
        r.0 == Ok(false) ==> r.1 >= p + 2,
        old(steps)@ <= final(steps)@,
        y == h.spec_height() ==> final(steps)@ == old(steps)@,
        match r.0 {
            Ok(true) => final(steps)@ == old(steps)@,
            Ok(false) => final(steps)@ as int + rle_position(h.spec_width(), h.spec_height(), x as int, y as int)
                <= old(steps)@ as int + rle_position(h.spec_width(), h.spec_height(), r.2 as int, r.3 as int),
            Err(_) => final(steps)@ as int <= old(steps)@ as int + h.spec_width() - x as int,
        },
        final(out)@.len() == 4 * h.spec_n(),
        final(out)@ == crate::model::bytes_replay(initial, gw + r.5@),
        crate::model::rle_clip_step(b@, hrec(h), p as int, x as int, y as int, overrun, eol, eob, black, delta)
            == (r.0, r.1 as int, r.2 as int, r.3 as int, r.5@, r.4),
{
    let ghost m = hrec(h);
    let ghost empty = Seq::<crate::model::Write>::empty();
    proof {
        reveal(header_inv_light); reveal(hrec); reveal(header_record);
        reveal(crate::model::rle_clip_step); reveal(rle_position);
        assert(gw + empty == gw) by { broadcast use vstd::seq_lib::group_seq_lib_default; }
    }
    let code = rd(b, p + 1) as usize;
    let pos = p + 2;
        let reject = match overrun {
            RleOverrunOptions::Clip => false,
            RleOverrunOptions::Reject | RleOverrunOptions::Image => true,
        };
        if y >= h.height as usize || (x + code > h.width as usize && reject) {
            return (Err(Error::RleBounds), pos, x, y, false, Ghost(empty), 0);
        }
        let recovered = x + code > h.width as usize;
        let payload = if h.bits == 8 { code } else { (code + 1) / 2 };
        let padded = payload + payload % 2;
        proof { reveal(crate::model::rle_payload); reveal(crate::model::rle_padded); }
        if b.len() - pos < padded {
            return (Err(Error::RleTruncated), pos, x, y, recovered, Ghost(empty), 0);
        }
        let count = rle_clip_count(h.width as usize, x, code);
        if black {
            proof {
                reveal(crate::model::rle_payload);
                assert((count as int + 1) / 2 <= (code as int + 1) / 2) by (nonlinear_arith) requires count <= code;
            }
            proof { reveal(crate::model::black_packet_writes); }
            let event = paint_black_packet(b, h, out, true, pos, x, y, count, Ghost(gw), steps, Ghost(initial));
            let ghost ws = crate::model::black_packet_writes(b@, m, true, pos as int, x as int, y as int, count as int);
            return (Ok(false), pos + padded, x + count, y, recovered || event, Ghost(ws), count);
        }
        if count == 0 {
            proof {
                reveal(crate::model::abs_good); reveal(crate::model::abs_writes);
            }
            return (Ok(false), pos + padded, x, y, recovered, Ghost(empty), 0);
        }
        proof {
            assert((count as int + 1) / 2 <= (code as int + 1) / 2) by (nonlinear_arith)
                requires count <= code;
        }
        let pr = paint_abs(b, h, out, pos, x, y, count, Ghost(gw), steps, Ghost(initial));
        match pr {
            Ok(()) => {
                proof {
                    crate::model::lemma_abs_good_all_ok(b@, m, pos as int, count as int);
                    crate::model::lemma_packet_write_lengths(b@, m, x as int, y as int, pos as int, 0, count as int);
                }
                let ghost ws = crate::model::abs_writes(b@, m, x as int, y as int, pos as int, 0, count as int);
                (Ok(false), pos + padded, x + count, y, recovered, Ghost(ws), count)
            },
            Err(e) => {
                let ghost ws = crate::model::abs_writes(b@, m, x as int, y as int, pos as int,
                    0, crate::model::abs_good(b@, m, pos as int, count as int, 0));
                (Err(e), pos + padded, x, y, recovered, Ghost(ws), 0)
            },
        }
}

fn rle_clip_encoded(b: &[u8], h: &Header, out: &mut Vec<u8>, p: usize, x: usize, y: usize,
    overrun: RleOverrunOptions, eol: RleEolOptions, eob: RleEobOptions, black: bool, delta: RleDeltaOptions, Ghost(gw): Ghost<Seq<crate::model::Write>>, steps: &mut Ghost<nat>, Ghost(initial): Ghost<Seq<u8>>)
    -> (r: (Result<bool, Error>, usize, usize, usize, bool, Ghost<Seq<crate::model::Write>>, usize))
    requires
        p + 2 <= b@.len(), b@[p as int] != 0,
        eob == RleEobOptions::AtInputEnd ==> y < h.spec_height(),
        header_inv(b@, *h), header_inv_light(h),
        h.compression == 1 || h.compression == 2,
        p <= b@.len(), x <= h.spec_width(), y <= h.spec_height(),
        old(out)@.len() == 4 * h.spec_n(),
        old(out)@ == crate::model::bytes_replay(initial, gw),
    ensures
        r.0 is Ok ==> r.6 as int == r.5@.len(),
        r.0 == Ok(true) ==> r.6 == 0,
        r.0 is Ok ==> r.6 as int + rle_position(h.spec_width(), h.spec_height(), x as int, y as int)
            <= rle_position(h.spec_width(), h.spec_height(), r.2 as int, r.3 as int),
        p <= r.1 <= b@.len(), r.2 <= h.spec_width(), r.3 <= h.spec_height(),
        r.0 == Ok(false) ==> r.1 >= p + 2,
        old(steps)@ <= final(steps)@,
        y == h.spec_height() ==> final(steps)@ == old(steps)@,
        match r.0 {
            Ok(true) => final(steps)@ == old(steps)@,
            Ok(false) => final(steps)@ as int + rle_position(h.spec_width(), h.spec_height(), x as int, y as int)
                <= old(steps)@ as int + rle_position(h.spec_width(), h.spec_height(), r.2 as int, r.3 as int),
            Err(_) => final(steps)@ as int <= old(steps)@ as int + h.spec_width() - x as int,
        },
        final(out)@.len() == 4 * h.spec_n(),
        final(out)@ == crate::model::bytes_replay(initial, gw + r.5@),
        crate::model::rle_clip_step(b@, hrec(h), p as int, x as int, y as int, overrun, eol, eob, black, delta)
            == (r.0, r.1 as int, r.2 as int, r.3 as int, r.5@, r.4),
{
    let ghost m = hrec(h);
    let ghost empty = Seq::<crate::model::Write>::empty();
    proof {
        reveal(header_inv_light); reveal(hrec); reveal(header_record);
        reveal(crate::model::rle_clip_step); reveal(rle_position);
        assert(gw + empty == gw) by { broadcast use vstd::seq_lib::group_seq_lib_default; }
    }
    let q = rd(b, p) as usize;
    let code = rd(b, p + 1) as usize;
    let pos = p + 2;
        let reject = match overrun {
            RleOverrunOptions::Reject => true,
            RleOverrunOptions::Clip => false,
            RleOverrunOptions::Image => h.bits == 4,
        };
        if y >= h.height as usize || (x + q > h.width as usize && reject) {
            return (Err(Error::RleBounds), pos, x, y, false, Ghost(empty), 0);
        }
        let count = rle_clip_count(h.width as usize, x, q);
        let recovered = x + q > h.width as usize;
        if black {
            proof { reveal(crate::model::black_packet_writes); }
            let event = paint_black_packet(b, h, out, false, code, x, y, count, Ghost(gw), steps, Ghost(initial));
            let ghost ws = crate::model::black_packet_writes(b@, m, false, code as int, x as int, y as int, count as int);
            return (Ok(false), pos, x + count, y, recovered || event, Ghost(ws), count);
        }
        let pr = paint_run(b, h, out, code, x, y, count, Ghost(gw), steps, Ghost(initial));
        match pr {
            Ok(()) => {
                proof {
                    crate::model::lemma_run_good_all_ok(m, code as int, count as int);
                    crate::model::lemma_packet_write_lengths(b@, m, x as int, y as int, code as int, 0, count as int);
                }
                let ghost ws = crate::model::run_writes(b@, m, x as int, y as int, code as int, 0, count as int);
                (Ok(false), pos, x + count, y, recovered, Ghost(ws), count)
            },
            Err(e) => {
                let ghost ws = crate::model::run_writes(b@, m, x as int, y as int, code as int,
                    0, crate::model::run_good(m, code as int, count as int, 0));
                (Err(e), pos, x, y, recovered, Ghost(ws), 0)
            },
        }
}

fn rle_clip_command(b: &[u8], h: &Header, out: &mut Vec<u8>, p: usize, x: usize, y: usize,
    overrun: RleOverrunOptions, eol: RleEolOptions, eob: RleEobOptions, black: bool, delta: RleDeltaOptions, Ghost(gw): Ghost<Seq<crate::model::Write>>, steps: &mut Ghost<nat>, Ghost(initial): Ghost<Seq<u8>>)
    -> (r: (Result<bool, Error>, usize, usize, usize, bool, Ghost<Seq<crate::model::Write>>, usize))
    requires
        header_inv(b@, *h), header_inv_light(h),
        h.compression == 1 || h.compression == 2,
        p <= b@.len(), x <= h.spec_width(), y <= h.spec_height(),
        old(out)@.len() == 4 * h.spec_n(),
        old(out)@ == crate::model::bytes_replay(initial, gw),
    ensures
        r.0 is Ok ==> r.6 as int == r.5@.len(),
        r.0 == Ok(true) ==> r.6 == 0,
        r.0 is Ok ==> r.6 as int + rle_position(h.spec_width(), h.spec_height(), x as int, y as int)
            <= rle_position(h.spec_width(), h.spec_height(), r.2 as int, r.3 as int),
        p <= r.1 <= b@.len(), r.2 <= h.spec_width(), r.3 <= h.spec_height(),
        r.0 == Ok(false) ==> r.1 >= p + 2,
        old(steps)@ + 1 <= final(steps)@,
        y == h.spec_height() ==> final(steps)@ == old(steps)@ + 1,
        match r.0 {
            Ok(true) => final(steps)@ == old(steps)@ + 1,
            Ok(false) => final(steps)@ as int + rle_position(h.spec_width(), h.spec_height(), x as int, y as int)
                <= old(steps)@ as int + 1 + rle_position(h.spec_width(), h.spec_height(), r.2 as int, r.3 as int),
            Err(_) => final(steps)@ as int <= old(steps)@ as int + 1 + h.spec_width() - x as int,
        },
        final(out)@.len() == 4 * h.spec_n(),
        final(out)@ == crate::model::bytes_replay(initial, gw + r.5@),
        crate::model::rle_clip_step(b@, hrec(h), p as int, x as int, y as int, overrun, eol, eob, black, delta)
            == (r.0, r.1 as int, r.2 as int, r.3 as int, r.5@, r.4),
{
    let ghost m = hrec(h);
    let ghost empty = Seq::<crate::model::Write>::empty();
    proof {
        reveal(header_inv_light); reveal(hrec); reveal(header_record);
        reveal(crate::model::rle_clip_step); reveal(rle_position);
        assert(gw + empty == gw) by { broadcast use vstd::seq_lib::group_seq_lib_default; }
        *steps = Ghost(steps@ + 1);
    }
    let allow_end = match eob {
        RleEobOptions::Required => false,
        RleEobOptions::AtInputEnd => true,
    };
    if allow_end && (y >= h.height as usize || b.len() - p < 2) {
        return (Ok(true), p, x, y, false, Ghost(empty), 0);
    }
    if b.len() - p < 2 {
        return (Err(Error::RleTruncated), p, x, y, false, Ghost(empty), 0);
    }
    let q = rd(b, p) as usize;
    let code = rd(b, p + 1) as usize;
    let pos = p + 2;
    if q != 0 {
        rle_clip_encoded(b, h, out, p, x, y, overrun, eol, eob, black, delta, Ghost(gw), steps, Ghost(initial))
    } else if code == 1 {
        (Ok(true), pos, x, y, false, Ghost(empty), 0)
    } else if code == 0 {
        if y >= h.height as usize {
            return (Err(Error::RleBounds), pos, x, y, false, Ghost(empty), 0);
        }
        proof {
            assert(y as int * h.spec_width() + x as int <= (y + 1) as int * h.spec_width())
                by (nonlinear_arith) requires x <= h.spec_width();
        }
        proof {
            assert(rle_position(h.spec_width(), h.spec_height(), x as int, y as int)
                <= rle_position(h.spec_width(), h.spec_height(), 0, y as int + 1)) by (nonlinear_arith)
                requires y < h.spec_height(), x <= h.spec_width(), h.spec_width() >= 1,
                    rle_position(h.spec_width(), h.spec_height(), x as int, y as int) == y as int * h.spec_width() + x as int,
                    rle_position(h.spec_width(), h.spec_height(), 0, y as int + 1)
                        == if y as int + 1 >= h.spec_height() { h.spec_width() * h.spec_height() } else { (y as int + 1) * h.spec_width() };
        }
        let stop = match eol {
            RleEolOptions::OnlyFinal => false,
            RleEolOptions::StopAtHeight => y + 1 == h.height as usize,
        };
        (Ok(stop), pos, 0, y + 1, false, Ghost(empty), 0)
    } else if code == 2 {
        if b.len() - pos < 2 {
            return (Err(Error::RleTruncated), pos, x, y, false, Ghost(empty), 0);
        }
        let dx = rd(b, pos) as usize;
        let dy = rd(b, pos + 1) as usize;
        match policy_delta_target(h.width as usize, h.height as usize, x, y, dx, dy, delta) {
            Err(e) => (Err(e), pos + 2, x, y, false, Ghost(empty), 0),
            Ok((nx, ny, event)) => (Ok(false), pos + 2, nx, ny, event, Ghost(empty), 0),
        }
    } else {
        rle_clip_absolute(b, h, out, p, x, y, overrun, eol, eob, black, delta, Ghost(gw), steps, Ghost(initial))
    }
}

proof fn lemma_policy_rle_position(width: int, height: int, x: int, y: int)
    requires width >= 1, height >= 1, 0 <= x <= width, 0 <= y <= height,
    ensures 0 <= rle_position(width, height, x, y) <= width * height,
{
    reveal(rle_position);
    assert(0 <= rle_position(width, height, x, y) <= width * height) by (nonlinear_arith)
        requires width >= 1, height >= 1, 0 <= x <= width, 0 <= y <= height,
            rle_position(width, height, x, y) == if y >= height { width * height } else { y * width + x };
}

proof fn lemma_policy_rle_error_budget(width: int, height: int, x: int, y: int,
    start: int, before: int, after: int, commands: int, len: int)
    requires width >= 1, height >= 1, 0 <= x <= width, 0 <= y <= height,
        commands <= len,
        before <= start + 4 * (width * height) + commands + rle_position(width, height, x, y),
        after <= before + 1 + width - x,
        y == height ==> after == before + 1,
    ensures after <= start + 5 * (width * height) + len + 1,
{
    reveal(rle_position);
    assert(after <= start + 5 * (width * height) + len + 1) by (nonlinear_arith)
        requires width >= 1, height >= 1, 0 <= x <= width, 0 <= y <= height,
            commands <= len,
            before <= start + 4 * (width * height) + commands + rle_position(width, height, x, y),
            after <= before + 1 + width - x,
            y == height ==> after == before + 1,
            rle_position(width, height, x, y) == if y >= height { width * height } else { y * width + x };
}

fn decode_rle_clipped(b: &[u8], h: &Header, out: &mut Vec<u8>, overrun: RleOverrunOptions, eol: RleEolOptions, eob: RleEobOptions, black: bool, delta: RleDeltaOptions,
    steps: &mut Ghost<nat>, fill: RleFillOptions) -> (r: (Result<usize, Error>, bool))
    requires header_inv(b@, *h), header_inv_light(h),
        h.compression == 1 || h.compression == 2,
        old(out)@.len() == 4 * h.spec_n(),
    ensures
        final(out)@.len() == old(out)@.len(),
        final(steps)@ as int <= old(steps)@ as int + 5 * h.spec_n() + b@.len() + 1,
        match r.0 { Ok(pos) => h.pixel_start <= pos && (h.pixel_start <= b@.len() ==> pos <= b@.len()), Err(_) => true },
        r.1 == crate::model::rle_clip_out(b@, hrec(h), overrun, eol, eob, black, delta, fill).1,
        match r.0 {
            Ok(pos) => crate::model::rle_clip_out(b@, hrec(h), overrun, eol, eob, black, delta, fill).0 == Ok((pos as int, final(out)@)),
            Err(e) => crate::model::rle_clip_out(b@, hrec(h), overrun, eol, eob, black, delta, fill).0 == Err(e),
        },
{
    let ghost s0 = steps@;
    let nonpalette = match fill { RleFillOptions::Palette0 => false, _ => true };
    match fill {
        RleFillOptions::Palette0 => fill_rle(b, h, out, steps),
        RleFillOptions::OpaqueBlack => fill_blank(out, false, steps),
        RleFillOptions::TransparentBlack => fill_blank(out, true, steps),
    }
    let ghost m = hrec(h);
    let ghost initial = crate::model::rle_initial(b@, m, fill);
    proof { reveal(crate::model::rle_initial); reveal(header_inv_light); reveal(hrec); reveal(header_record); }
    let ghost full = crate::model::rle_clip_full(b@, m, h.pixel_start as int, 0, 0, overrun, eol, eob, black, delta);
    if h.pixel_start > b.len() {
        proof {
            reveal(crate::model::rle_clip_out); reveal(crate::model::rle_clip_full);
            reveal(crate::model::rle_clip_step); reveal(rle_position); reveal(crate::model::bytes_replay);
        }
        return match eob {
            RleEobOptions::Required => (Err(Error::RleTruncated), false),
            RleEobOptions::AtInputEnd => (Ok(h.pixel_start), nonpalette),
        };
    }
    let mut pos = h.pixel_start;
    let mut x = 0usize;
    let mut y = 0usize;
    let mut recovered = false;
    let mut painted = 0usize;
    let ghost mut gw = Seq::<crate::model::Write>::empty();
    let ghost mut commands: nat = 0;
    proof {
        reveal(header_inv_light); reveal(hrec); reveal(header_record);
        assert(gw + full.1 == full.1) by { broadcast use vstd::seq_lib::group_seq_lib_default; }
    }
    loop
        invariant
            header_inv(b@, *h), header_inv_light(h), m == hrec(h),
            h.compression == 1 || h.compression == 2,
            out@.len() == 4 * h.spec_n(), old(out)@.len() == 4 * h.spec_n(),
            h.pixel_start <= pos <= b@.len(), x <= h.spec_width(), y <= h.spec_height(),
            s0 == old(steps)@,
            initial == crate::model::rle_initial(b@, m, fill),
            nonpalette == (fill != RleFillOptions::Palette0),
            painted as int == gw.len(),
            painted as int <= rle_position(h.spec_width(), h.spec_height(), x as int, y as int),
            commands <= pos - h.pixel_start,
            steps@ as int <= s0 as int + 4 * h.spec_n() + commands as int + rle_position(h.spec_width(), h.spec_height(), x as int, y as int),
            full == crate::model::rle_clip_full(b@, m, h.pixel_start as int, 0, 0, overrun, eol, eob, black, delta),
            full.0 == crate::model::rle_clip_full(b@, m, pos as int, x as int, y as int, overrun, eol, eob, black, delta).0,
            full.1 == gw + crate::model::rle_clip_full(b@, m, pos as int, x as int, y as int, overrun, eol, eob, black, delta).1,
            full.2 == (recovered || crate::model::rle_clip_full(b@, m, pos as int, x as int, y as int, overrun, eol, eob, black, delta).2),
            out@ == crate::model::bytes_replay(initial, gw),
        decreases b@.len() - pos,
    {
        let ghost sb = steps@;
        let ghost gwb = gw;
        let ghost previous = crate::model::rle_clip_full(b@, m, pos as int, x as int, y as int, overrun, eol, eob, black, delta);
        proof {
            reveal_with_fuel(crate::model::rle_clip_full, 1);
            reveal(header_inv_light);
            lemma_policy_rle_position(h.spec_width(), h.spec_height(), x as int, y as int);
        }
        let (result, next, nx, ny, event, Ghost(ws), count) = rle_clip_command(b, h, out,
            pos, x, y, overrun, eol, eob, black, delta, Ghost(gw), steps, Ghost(initial));
        recovered = recovered || event;
        proof {
            gw = gw + ws;
        }
        match result {
            Err(e) => {
                proof {
                    reveal(crate::model::rle_clip_out);
                    lemma_policy_rle_error_budget(h.spec_width(), h.spec_height(), x as int, y as int,
                        s0 as int, sb as int, steps@ as int, commands as int, b@.len() as int);
                }
                return (Err(e), recovered);
            },
            Ok(true) => { proof { reveal(crate::model::rle_clip_out); } return (Ok(next), recovered || (nonpalette && painted < h.n)); },
            Ok(false) => {
                proof {
                    let tail = crate::model::rle_clip_full(b@, m, next as int, nx as int, ny as int, overrun, eol, eob, black, delta);
                    vstd::seq_lib::lemma_concat_associative(gwb, ws, tail.1);
                    // Associativity relates the consumed packet to the remaining write list.
                    assert(full.1 == gw + tail.1) by {
                        broadcast use vstd::seq_lib::group_seq_lib_default;
                    }
                    commands = commands + 1;
                }
                proof {
                    reveal(header_inv_light);
                    lemma_policy_rle_position(h.spec_width(), h.spec_height(), nx as int, ny as int);
                }
                painted = painted + count;
                pos = next; x = nx; y = ny;
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
        h.pixel_start <= b@.len(),
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
    reveal(crate::model::mh); reveal(crate::model::r2r7); reveal(crate::model::r6r7);
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

/// R12 for the two BMP modes with the `image_size` option: the `biSizeImage`
/// equality (`Declared`) or no size check at all (`Ignore`), then the `BmpFile`
/// suffix-ignoring rule or `BareDib`'s exact-tail `TrailingBytes` test. Like
/// `finish_mask`, it performs no AND-mask work (there is no AND plane in either
/// BMP mode). Its `mask_done_policy_eq` postcondition is
/// `finish_mask`'s `mask_done_eq` with `mp_mask_phase`'s size test replaced.
fn finish_mask_policy(b: &[u8], h: &Header, out: &mut Vec<u8>, pos: usize, p0: usize,
    steps: &mut Ghost<nat>, policy: Policy) -> (r: Result<(), Error>)
    requires
        header_inv(b@, *h),
        header_inv_light(h),
        h.mode == Mode::BmpFile || h.mode == Mode::BareDib,
        old(out)@.len() == 4 * h.spec_n(),
        p0 <= pos,
        p0 as int == crate::model::offset_effective(hrec(h), policy),
        b@.len() <= 67108864,
    ensures
        old(steps)@ == final(steps)@,
        final(out)@.len() == old(out)@.len(),
        match r { Ok(()) => true, Err(e) => e == Error::InvalidImageSize
            || e == Error::TrailingBytes },
        crate::model::mask_done_policy_eq(b@, hrec(h), pos as int, old(out)@, final(out)@, r, policy),
        final(out)@ == old(out)@,
{
    proof {
        reveal(crate::model::mask_done_policy_eq);
        reveal(crate::model::mp_mask_phase_policy);
        reveal(crate::model::mp_image_size_ok_policy);
        reveal(crate::model::image_size_ok_policy);
        reveal(crate::model::image_size_ok);
        reveal(crate::model::and_bytes);
        reveal(crate::model::and_stride);
        reveal(hrec);
        reveal(crate::model::is_ico);
        reveal(crate::model::exact_tail);
        lemma_hrec_mode(h);
        assert(hrec(h).mode == h.mode);
        assert(hrec(h).mode == Mode::BmpFile || hrec(h).mode == Mode::BareDib);
    }
    let image_len = pos - p0;
    let skip_size = match policy.image_size {
        MetadataOptions::Declared => false,
        MetadataOptions::Ignore => true,
    };
    if !skip_size && h.declared != 0 {
        let valid = h.declared as u64 == image_len as u64;
        if !valid {
            proof {
                reveal(crate::model::mask_done_policy_eq);
                reveal(crate::model::mp_mask_phase_policy);
                reveal(crate::model::mp_image_size_ok_policy);
                reveal(crate::model::image_size_ok_policy);
                reveal(crate::model::image_size_ok);
                reveal(crate::dib::is_ico);
                reveal(hrec);
                assert(image_len as int == pos as int - p0 as int);
                assert(policy.image_size == MetadataOptions::Declared);
                assert(h.declared != 0);
                assert(h.declared as int != 0);
                assert(hrec(h).declared_image == h.declared as int);
                assert(hrec(h).pixel_start == h.pixel_start as int);
                assert(hrec(h).declared_image != image_len as int);
                assert(!crate::model::mp_image_size_ok_policy(
                    hrec(h), image_len as int, policy));
            }
            return Err(Error::InvalidImageSize);
        }
    }
    proof {
        reveal(crate::model::mask_done_policy_eq);
        reveal(crate::model::mp_mask_phase_policy);
        reveal(crate::model::mp_image_size_ok_policy);
        reveal(crate::model::image_size_ok_policy);
        reveal(crate::model::image_size_ok);
        reveal(crate::model::is_ico);
        reveal(hrec);
        assert(image_len as int == pos as int - p0 as int);
        assert(hrec(h).declared_image == h.declared as int);
        // The image-size test passes in the model at this point.
        assert(crate::model::mp_image_size_ok_policy(hrec(h), image_len as int, policy));
    }
    match h.mode {
        Mode::BareDib => {
            let reject_short = match policy.short_pixels {
                ShortPixelsOptions::Reject => true,
                ShortPixelsOptions::ZeroFill => false,
            };
            if pos < b.len() || (pos > b.len() && reject_short) {
                proof {
                    reveal(crate::model::mask_done_policy_eq);
                    reveal(crate::model::mp_mask_phase_policy);
                    reveal(crate::model::mp_image_size_ok_policy);
                    reveal(crate::model::image_size_ok_policy);
                    reveal(hrec);
                    reveal(header_inv_light);
                    lemma_hrec_mode(h);
                    assert(hrec(h).mode == Mode::BareDib);
                    assert(pos as int != b@.len());
                }
                return Err(Error::TrailingBytes);
            }
            proof {
                reveal(crate::model::mask_done_policy_eq);
                reveal(crate::model::mp_mask_phase_policy);
                reveal(crate::model::mp_image_size_ok_policy);
                reveal(crate::model::image_size_ok_policy);
                reveal(hrec);
                reveal(header_inv_light);
                lemma_hrec_mode(h);
                assert(hrec(h).mode == Mode::BareDib);
                assert(pos >= b.len());
            }
        },
        Mode::BmpFile => {
            proof {
                reveal(crate::model::mask_done_policy_eq);
                reveal(crate::model::mp_mask_phase_policy);
                reveal(crate::model::mp_image_size_ok_policy);
                reveal(crate::model::image_size_ok_policy);
                reveal(hrec);
                reveal(header_inv_light);
                lemma_hrec_mode(h);
                assert(hrec(h).mode == Mode::BmpFile);
            }
        },
        Mode::IcoEntryDib | Mode::IcoEntryDibSlack => {
            proof {
                // The profiled entry point has no ICO mode: `m.mode` is the
                // `BmpMode` converted, so these arms are unreachable.
                lemma_hrec_mode(h);
                assert(h.mode == Mode::BmpFile || h.mode == Mode::BareDib);
                assert(false);
            }
        },
    }
    Ok(())
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
    assert(h.spec_count() > 0 ==> h.spec_palette_start() + h.spec_count() * h.spec_pal_stride() <= b.len());
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
        h.pixel_start <= b@.len(),
    ensures
        final(out)@.len() == 4 * h.spec_n(),
        match r { Ok(p) => h.pixel_start <= p <= b@.len(), Err(_) => true },
{
    if h.compression == 0 || h.compression == 3 {
        let mut steps = Ghost(0nat);
        decode_raw(b, h, out, h.pixel_start, &mut steps)
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
        // The gdk-pixbuf `has_alpha` field: on a success it is the 32-bpp/RLE
        // rule on `MH`'s validated record, an error carries no image.
        match r {
            Ok(d) => d.rle_or_32bpp == crate::model::mh_rle_or_32bpp(bytes@, mode),
            Err(_) => true,
        },
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
        decode_raw(bytes, &h, &mut out, h.pixel_start, steps)
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
        reveal(crate::model::mh_rle_or_32bpp);
        assert((h.bits == 32u32 || h.compression == 1u32 || h.compression == 2u32)
            == crate::model::mh_rle_or_32bpp(bytes@, mode));
    }
    Ok(Decoded { width: h.width, height: h.height, rgba: out, rle_or_32bpp: header_alpha(&h) })
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
        // The `has_alpha` field on `mh`'s validated record; an error carries none.
        match r {
            Ok(d) => d.rle_or_32bpp == crate::model::mh_rle_or_32bpp(bytes@, mode),
            Err(_) => true,
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

/// The profiled entry point's ghost header record: `MH_policy`'s `Ok` record,
/// tied to the concrete `Header` by `hdr_matches`. An exec spec fn so the ghost
/// local can be introduced by `let ghost m = ...` and still be passed to
/// `lemma` proof functions as a value.
spec fn ghost_mh_policy(b: Seq<u8>, bm: BmpMode, policy: Policy, h: &Header)
    -> crate::model::MHRecord {
    choose|m: crate::model::MHRecord| crate::model::mh_policy(b, bm, policy) == Ok(m)
        && hdr_matches(m, *h)
}

/// T9 for the profiled entry point: the returned `Result` equals
/// `MP_policy(b, bm, policy)` as the exact `Result`, including every output
/// byte. `Policy` is the concrete record `profile_policy(profile)` selects.
pub open spec fn decode_policy_mp_eq(
    r: Result<Decoded, Error>,
    b: Seq<u8>,
    bm: BmpMode,
    policy: Policy,
) -> bool {
    match r {
        Ok(d) => crate::model::mp_policy(b, bm, policy)
            == Ok((d.width as int, d.height as int, d.rgba@)),
        Err(e) => crate::model::mp_policy(b, bm, policy) == Err(e),
    }
}

/// The `has_alpha` postcondition of the profiled entry point and its helpers: on
/// a success the `Decoded.rle_or_32bpp` field equals the gdk-pixbuf 32-bpp/RLE
/// rule on the validated header selection `MH_policy` (or its `MH_policy`-backed
/// partial fallback). An error carries no image, so the clause is vacuous.
pub open spec fn decode_policy_alpha_eq(
    r: Result<Decoded, Error>,
    b: Seq<u8>,
    bm: BmpMode,
    policy: Policy,
) -> bool {
    match r {
        Ok(d) => d.rle_or_32bpp == crate::model::mh_policy_rle_or_32bpp(b, bm, policy),
        Err(_) => true,
    }
}

/// Select the BITFIELDS pixel location from copied scalar header fields.
fn bitfield_pixel_start(meta: bool, masks: bool, ps: usize, pe: usize) -> (r: usize)
    ensures r as int == crate::model::bitfield_pixel_start(meta, masks, ps as int, pe as int),
{
    if meta && masks { pe } else { ps }
}

/// Keep executable field reads and the model's record projections in one small proof.
fn policy_pixel_start(h: &Header, policy: Policy, Ghost(m): Ghost<crate::model::MHRecord>)
    -> (r: usize)
    requires hdr_matches(m, *h),
    ensures r as int == crate::model::offset_effective(m, policy),
{
    let ps = h.pixel_start;
    let pe = h.palette_end;
    let compression = h.compression;
    let masks = compression == 3u32;
    let meta = match policy.bitfield_offset {
        BitfieldOffsetOptions::MetadataEnd => true,
        _ => false,
    };
    proof {
        reveal(hdr_matches);
        assert(m.has_masks == masks);
        assert(m.pixel_start == ps as int);
        assert(m.palette_end == pe as int);
        assert(m.compression == compression as int);
        assert(meta == (policy.bitfield_offset == BitfieldOffsetOptions::MetadataEnd));
    }
    let start = bitfield_pixel_start(meta, masks, ps, pe);
    let result = match policy.offset {
        OffsetOptions::Gdk => if compression == 0u32 && start < pe { pe } else { start },
        _ => start,
    };
    proof {
        reveal(crate::model::offset_effective);
        assert(result as int == crate::model::offset_effective(m, policy));
    }
    result
}

/// A short indexed table followed by an offset gap selects initialized output.
fn policy_indexed_gap(h: &Header, p: Policy) -> (r: bool)
    requires header_inv_light(h),
    ensures r == crate::model::indexed_gap_blank(hrec(h), p),
{
    let bits = h.bits;
    let gap = match p.short_indexed_gap {
        ShortIndexedGapOptions::Blank => h.is_bmp && bits <= 8u32 && h.used > 0u32
            && h.used < (1u32 << bits) && h.pixel_start > h.palette_end,
        _ => false,
    };
    proof {
        reveal(hrec);
        reveal(header_record);
        reveal(crate::model::indexed_gap_blank);
        reveal(crate::model::pow2);
        reveal(header_inv_light);
        if bits <= 8u32 {
            assert(bits == 1u32 || bits == 2u32 || bits == 4u32 || bits == 8u32);
            assert((1u32 << bits) as int == crate::model::pow2(bits as int))
                by (bit_vector) requires bits == 1u32 || bits == 2u32 || bits == 4u32 || bits == 8u32;
        }
    }
    gap
}

/// Inspect the raw RGB words before substitution; do not infer fallback from output masks.
fn policy_header_recovered(b: &[u8], h: &Header, p: Policy) -> (r: bool)
    requires header_inv_light(h),
    ensures r == crate::model::mh_recovered_so_far(b@, hrec(h), p),
{
    let mut recovered = false;
    if h.is_bmp {
        match p.offset {
            OffsetOptions::Declared => {},
            OffsetOptions::AllowAny => {
                recovered = h.pixel_start < 18 || h.pixel_start > b.len();
            },
            OffsetOptions::Gdk => {
                recovered = h.pixel_start < 18 || h.pixel_start > b.len()
                    || (h.bits <= 8 && h.pixel_start < h.palette_end)
                    || (h.compression == 0 && h.pixel_start < h.palette_end);
            },
        }
    }
    match p.bitfield_offset {
        BitfieldOffsetOptions::MetadataEnd => {
            if h.is_bmp && h.compression == 3 && h.pixel_start != h.palette_end {
                recovered = true;
            }
        },
        _ => {},
    }
    recovered = recovered || policy_indexed_gap(h, p);
    match p.masks {
        MasksOptions::DefaultRGB => {
            if h.compression == 3 {
                let off = if h.hsize == 40 || h.hsize == 64 { h.hsize } else { 40 };
                if h.s <= b.len() && off <= b.len() - h.s {
                    let q = h.s + off;
                    if b.len() - q >= 12 {
                        let r = rd32(b, q);
                        let g = rd32(b, q + 4);
                        let blue = rd32(b, q + 8);
                        recovered = recovered || r == 0 || g == 0 || blue == 0;
                    }
                }
            }
        },
        _ => {},
    }
    match p.header_bytes {
        HeaderBytesOptions::ParsedFields => {
            recovered = recovered || h.s > b.len() || h.hsize > b.len() - h.s;
        },
        _ => {},
    }
    recovered
}

/// The profiled entry point (POLICY-SPEC "Named profiles", THEOREMS-POLICY-DELTA):
/// `decode_with_policy(bytes, bm, profile)` for the two BMP modes and all four
/// named profiles, returning the result and the recovery flag.
///
/// * T9: the `Result` equals `MP_policy(bytes, bm, profile_policy(profile))` on
///   every error and every RGBA byte, for every slice, both modes and every
///   profile.
/// * The gdk-pixbuf `has_alpha` field: on a success `d.rle_or_32bpp` is the
///   frozen predicate `bits == 32 || compression == 1 || compression == 2` on the
///   validated header record (`MH_policy`, or the `MH_policy`-backed partial
///   fallback), the same predicate the `offset = Gdk` blank fill selects. It never
///   reads a pixel value.
/// * `recovered` is the closed-event flag of POLICY-SPEC on the selected record.
///   For `Strict` and `Recommended` it is `false` on every input (only `Ignore`d
///   `bfSize`/`biSizeImage` values are tolerated there, and none of those is a
///   listed data-affecting event). `GdkCompat` and `ImageCrateCompat` carry
///   recovery-producing options, so their flag follows `recovered_policy`.
/// * T3/T13: the sole allocation is the one `4N` output buffer, made once after
///   the header phase succeeds; a header error allocates nothing, and any
///   success has the exact `4*w*h <= 64 MiB` output.
/// * T1/T11/T12: no read beyond the slice, no panic, termination, and the same
///   charged-work bound as `decode`.
///
/// Flagged, not changed: the return type is the tuple `(Result<Decoded, Error>,
/// bool)`. THEOREMS-POLICY-DELTA sketches a named `ProfiledDecoded` record
/// carrying dimensions, RGBA bytes and `recovered`. Folding the flag into a
/// struct is a pure API shape change with no proof content — the two
/// postconditions above are stated on `r.0`/`r.1` and would read the same
/// through field projections — so it is left for whichever milestone fixes the
/// public surface, at which point every caller and the `t*_policy*` witnesses
/// update mechanically.
pub fn decode_with_policy(bytes: &[u8], bm: BmpMode, profile: BmpProfile)
    -> (r: (Result<Decoded, Error>, bool))
    ensures
        r.1 == crate::model::recovered_policy(bytes@, bm, profile_policy(profile)),
        decode_policy_mp_eq(r.0, bytes@, bm, profile_policy(profile)),
        decode_policy_alpha_eq(r.0, bytes@, bm, profile_policy(profile)),
        match r.0 {
            Ok(d) => 1 <= d.width as int <= 8192 && 1 <= d.height as int <= 8192
                && d.rgba@.len() == 4 * (d.width as int * d.height as int)
                && d.rgba@.len() <= 67108864,
            Err(_) => true,
        },
{
    let policy = profile_policy_exec(profile);
    let mut steps = Ghost(0nat);
    decode_policy_tracked(bytes, bm, policy, &mut steps)
}

/// The header-validated pixel count, zero precisely on a failed header.
spec fn policy_checked_n(b: Seq<u8>, bm: BmpMode, p: Policy) -> int {
    match crate::model::mh_partial(b, bm, p) {
        Some(rf) => rf.width * rf.height,
        None => match crate::model::mh_policy(b, bm, p) { Ok(m) => m.n, Err(_) => 0 },
    }
}

fn alloc_output_n(n: usize, steps: &mut Ghost<nat>) -> (out: Vec<u8>)
    requires 1 <= n <= 16777216,
    ensures out@ == crate::model::zero_bytes(4 * n), out@.len() == 4 * n,
        final(steps)@ == old(steps)@ + 4 * n,
{
    let out = vec![0u8; 4 * n];
    proof {
        *steps = Ghost(steps@ + (4 * n) as nat);
        reveal(crate::model::zero_bytes);
        assert(out@ =~= crate::model::zero_bytes(4 * n));
    }
    out
}

fn decode_partial_metadata(bytes: &[u8], bm: BmpMode, policy: Policy, original: Error,
    steps: &mut Ghost<nat>) -> (r: (Result<Decoded, Error>, bool))
    requires crate::model::mh_policy(bytes@, bm, policy) == Err(original),
        original == Error::TruncatedMasks || original == Error::TruncatedPalette,
        policy.incomplete_metadata == IncompleteMetadataOptions::Blank,
    ensures
        decode_policy_mp_eq(r.0, bytes@, bm, policy),
        decode_policy_alpha_eq(r.0, bytes@, bm, policy),
        r.1 == crate::model::recovered_policy(bytes@, bm, policy),
        final(steps)@ as int <= old(steps)@ as int + 8 * policy_checked_n(bytes@, bm, policy),
        match r.0 {
            Ok(d) => 1 <= d.width <= 8192 && 1 <= d.height <= 8192
                && d.rgba@.len() == 4 * (d.width as int * d.height as int) && d.rgba@.len() <= 67108864,
            Err(_) => true,
        },
{
    proof {
        reveal(crate::model::mh_policy); reveal(crate::model::r2r7_policy);
        reveal(crate::model::mh_partial); reveal(crate::model::mp_policy);
        reveal(crate::model::recovered_policy); reveal(decode_policy_mp_eq);
        reveal(decode_policy_alpha_eq);
        reveal(policy_checked_n); reveal(crate::dib::bmp_mode_to_mode);
    }
    // No allocation occurs until the second validation yields a checked field record.
    if bytes.len() > MAX_INPUT { return (Err(original), false); }
    let mode = bmp_mode_mode(bm);
    let is_bmp = match bm { BmpMode::BmpFile => true, _ => false };
    let (start, file_offset) = match bm {
        BmpMode::BmpFile => match bmp_prefix_policy(bytes, policy) {
            Ok(pair) => pair,
            Err(_) => return (Err(original), false),
        },
        BmpMode::BareDib => (0usize, 0u64),
    };
    let f = match dib_fields(bytes, mode, start, file_offset, is_bmp, policy) {
        Ok(f) => f,
        Err(_) => return (Err(original), false),
    };
    let masks_missing = match original { Error::TruncatedMasks => true, _ => false };
    if masks_missing && f.size != 40 && f.size != 64 {
        return (Err(original), false);
    }
    proof {
        crate::model::lemma_r2r5_policy_ok(bytes@, mode, start as int, policy, f.spec_raw());
        assert(crate::model::mh_partial(bytes@, bm, policy) == Some(f.spec_raw()));
        assert(1 <= f.n <= 16777216) by (nonlinear_arith)
            requires f.n as int == f.width_i as int * f.height_v as int,
                f.width_i >= 1, f.height_v >= 1, 4 * (f.width_i as int * f.height_v as int) <= 67108864;
    }
    let mut out = alloc_output_n(f.n, steps);
    let transparent = f.bits == 32 || f.compression == 1 || f.compression == 2;
    fill_blank(&mut out, transparent, steps);
    proof {
        reveal(crate::model::partial_fill);
        crate::model::lemma_mh_policy_rle_or_32bpp_partial(bytes@, bm, policy, f.spec_raw());
        assert(crate::model::mh_policy_rle_or_32bpp(bytes@, bm, policy)
            == (f.bits == 32u32 || f.compression == 1u32 || f.compression == 2u32));
        assert(transparent == crate::model::rle_or_32bpp(f.spec_raw().depth, f.spec_raw().compression));
    }
    (Ok(Decoded { width: f.width_i as u32, height: f.height_v, rgba: out, rle_or_32bpp: transparent }), true)
}

fn finish_raw_zero(b: &[u8], bm: BmpMode, h: &Header, mut out: Vec<u8>, start: usize,
    policy: Policy, alpha: bool, steps: &mut Ghost<nat>) -> (r: (Result<Decoded, Error>, bool))
    requires
        header_inv(b@, *h), header_inv_light(h),
        h.mode == Mode::BmpFile || h.mode == Mode::BareDib,
        crate::model::mh_policy(b@, bm, policy) == Ok(hrec(h)),
        h.compression == 0 || h.compression == 3,
        !crate::model::gdk_blank(b@, hrec(h), policy),
        !crate::model::gdk_rgb_reject(b@, hrec(h), policy),
        !crate::model::raw_legacy(policy),
        start as int == crate::model::offset_effective(hrec(h), policy),
        out@.len() == 4 * h.spec_n(), b@.len() <= 67108864,
    ensures
        final(steps)@ as int <= old(steps)@ as int + 5 * h.spec_n() + h.spec_height() + 1,
        decode_policy_mp_eq(r.0, b@, bm, policy),
        match r.0 {
            Ok(d) => d.rle_or_32bpp == alpha,
            Err(_) => true,
        },
        r.1 == crate::model::recovered_policy(b@, bm, policy),
        match r.0 {
            Ok(d) => d.width == h.width && d.height == h.height && d.rgba@.len() == 4 * h.spec_n(),
            Err(_) => true,
        },
{
    proof {
        reveal(header_inv); reveal(header_inv_light); reveal(hrec); reveal(header_record);
        reveal(decode_policy_mp_eq);
        reveal(crate::model::mp_policy); reveal(crate::model::mp_raw_policy);
        reveal(crate::model::mp_decoded); reveal(crate::model::mp_mask_phase_policy);
        reveal(crate::model::mp_image_size_ok_policy); reveal(crate::model::image_size_ok_policy);
        reveal(crate::model::image_size_ok); reveal(crate::model::is_ico);
        reveal(crate::model::recovered_policy); reveal(crate::model::pixel_recovered);
    }
    let zero = match policy.short_pixels { ShortPixelsOptions::ZeroFill => true, _ => false };
    let black = match policy.palette_index { PaletteIndexOptions::Black => true, _ => false };
    let opaque = match policy.bitfield16_alpha { Bitfield16AlphaOptions::Opaque => true, _ => false };
    let (result, event) = decode_raw_zero(b, h, &mut out, start, zero, black, policy.scaling, opaque, steps);
    let advance = match result {
        Err(e) => return (Err(e), false),
        Ok(advance) => advance,
    };
    let exact = match policy.image_size {
        MetadataOptions::Declared => true,
        MetadataOptions::Ignore => false,
    };
    if exact && h.declared != 0 && h.declared as u64 != advance {
        return (Err(Error::InvalidImageSize), false);
    }
    match h.mode {
        Mode::BareDib => {
            if (start <= b.len() && advance < (b.len() - start) as u64)
                || (!zero && (start > b.len() || advance > (b.len() - start) as u64)) {
                return (Err(Error::TrailingBytes), false);
            }
        },
        _ => {},
    }
    let recovered = policy_header_recovered(b, h, policy);
    (Ok(Decoded { width: h.width, height: h.height, rgba: out, rle_or_32bpp: alpha }), recovered || event)
}

fn finish_rle_clipped(b: &[u8], bm: BmpMode, h: &Header, mut out: Vec<u8>, policy: Policy,
    alpha: bool, steps: &mut Ghost<nat>) -> (r: (Result<Decoded, Error>, bool))
    requires
        header_inv(b@, *h), header_inv_light(h),
        h.mode == Mode::BmpFile || h.mode == Mode::BareDib,
        crate::model::mh_policy(b@, bm, policy) == Ok(hrec(h)),
        h.compression == 1 || h.compression == 2,
        !crate::model::gdk_blank(b@, hrec(h), policy),
        !crate::model::rle_legacy(policy),
        out@.len() == 4 * h.spec_n(), b@.len() <= 67108864,
    ensures
        final(steps)@ as int <= old(steps)@ as int + 5 * h.spec_n() + b@.len() + 1,
        decode_policy_mp_eq(r.0, b@, bm, policy),
        match r.0 {
            Ok(d) => d.rle_or_32bpp == alpha,
            Err(_) => true,
        },
        r.1 == crate::model::recovered_policy(b@, bm, policy),
        match r.0 {
            Ok(d) => d.width == h.width && d.height == h.height && d.rgba@.len() == 4 * h.spec_n(),
            Err(_) => true,
        },
{
    proof {
        reveal(header_inv); reveal(header_inv_light); reveal(hrec); reveal(header_record);
        reveal(decode_policy_mp_eq);
        reveal(crate::model::mp_policy); reveal(crate::model::mp_rle_policy);
        reveal(crate::model::mp_decoded); reveal(crate::model::gdk_rgb_reject);
        reveal(crate::model::offset_effective);
        reveal(crate::model::recovered_policy); reveal(crate::model::pixel_recovered);
    }
    let black = match policy.palette_index { PaletteIndexOptions::Black => true, _ => false };
    let (result, event) = decode_rle_clipped(b, h, &mut out, policy.rle_overrun, policy.rle_eol, policy.rle_eob, black, policy.rle_delta, steps, policy.rle_fill);
    let pos = match result {
        Err(e) => return (Err(e), false),
        Ok(pos) => pos,
    };
    let end = finish_mask_policy(b, h, &mut out, pos, h.pixel_start, steps, policy);
    proof { reveal(crate::model::mask_done_policy_eq); }
    match end {
        Err(e) => return (Err(e), false),
        Ok(()) => {},
    }
    let recovered = policy_header_recovered(b, h, policy);
    (Ok(Decoded { width: h.width, height: h.height, rgba: out, rle_or_32bpp: alpha }), recovered || event)
}

/// All retained policy combinations use this executable and its charged-work proof.
proof fn lemma_policy_header_geometry(b: Seq<u8>, bm: BmpMode, p: Policy, m: crate::model::MHRecord)
    requires crate::model::mh_policy(b, bm, p) == Ok(m),
    ensures
        m.mode == bmp_mode_to_mode(bm),
        m.palette_end == m.palette_start + m.table_count * m.palette_stride,
        (p.header_bytes == HeaderBytesOptions::ParsedFields && m.table_count == 0) || m.palette_end <= b.len(),
{
    reveal(crate::model::mh_policy);
    reveal(crate::model::r2r7_policy);
    reveal(crate::model::r6r7_policy);
    reveal(crate::model::r6r7_result_policy);
}

fn decode_policy_tracked(bytes: &[u8], bm: BmpMode, policy: Policy, steps: &mut Ghost<nat>)
    -> (r: (Result<Decoded, Error>, bool))
    ensures
        final(steps)@ as int <= old(steps)@ as int
            + 16 * (bytes@.len() + policy_checked_n(bytes@, bm, policy)),
        r.1 == crate::model::recovered_policy(bytes@, bm, policy),
        decode_policy_mp_eq(r.0, bytes@, bm, policy),
        decode_policy_alpha_eq(r.0, bytes@, bm, policy),
        match r.0 {
            Ok(d) => 1 <= d.width as int <= 8192 && 1 <= d.height as int <= 8192
                && d.rgba@.len() == 4 * (d.width as int * d.height as int)
                && d.rgba@.len() <= 67108864,
            Err(_) => true,
        },
{
    let ghost start_steps = steps@;
    let recovered: bool = false;
    let h = match parse_header_policy(bytes, bm, policy) {
        Ok(h) => h,
        Err(e) => {
            let blank = match policy.incomplete_metadata { IncompleteMetadataOptions::Blank => true, _ => false };
            let recoverable = match e { Error::TruncatedMasks | Error::TruncatedPalette => true, _ => false };
            if blank && recoverable {
                return decode_partial_metadata(bytes, bm, policy, e, steps);
            }

            proof {
                reveal(decode_policy_mp_eq);
                reveal(decode_policy_alpha_eq);
                match crate::model::mh_policy(bytes@, bm, policy) {
                    Ok(_) => {},
                    Err(e2) => {
                        crate::model::lemma_mp_policy_mh_err(bytes@, bm, policy, e2);
                    },
                }
            }
            return (Err(e), recovered);
        },
    };
    let alpha = header_alpha(&h);
    proof { lemma_alpha_value(bytes@, bm, policy, &h, alpha); }
    let r = decode_complete_policy(bytes, bm, policy, h, alpha, steps);
    r
}

/// The complete-header half of the `has_alpha` tie, isolated from
/// `decode_complete_policy`'s near-budget body. `MH_policy` has no partial
/// fallback here (`mh_partial` is `None`), so the field's rule is
/// `rle_or_32bpp` on `MH_policy`'s record `m`, whose `depth`/`compression`
/// `hdr_matches` pins to the executable `Header`'s `spec_depth`/`spec_compression`.
proof fn lemma_complete_alpha(b: Seq<u8>, bm: BmpMode, p: Policy, h: &Header, m: crate::model::MHRecord)
    requires
        crate::model::mh_policy(b, bm, p) == Ok(m),
        crate::model::mh_partial(b, bm, p) is None,
        hdr_matches(m, *h),
    ensures
        crate::model::mh_policy_rle_or_32bpp(b, bm, p)
            == crate::model::rle_or_32bpp(h.spec_depth(), h.spec_compression()),
{
    reveal(crate::model::mh_policy_rle_or_32bpp);
    reveal(crate::model::rle_or_32bpp);
    crate::model::lemma_mh_policy_rle_or_32bpp_complete(b, bm, p, m);
    reveal(hdr_matches);
    assert(m.depth == h.spec_depth() && m.compression == h.spec_compression());
}

/// The complete-header `has_alpha` value tie: given `hdr_matches` and the
/// header's own `rle_or_32bpp` value `alpha`, the model selection
/// `mh_policy_rle_or_32bpp` equals `alpha`. A pure value lemma (no `Decoded`), so
/// `decode_policy_tracked` runs it before the `Header` moves into
/// `decode_complete_policy` and then combines it with that call's
/// `d.rle_or_32bpp == alpha` postcondition.
proof fn lemma_alpha_value(bytes: Seq<u8>, bm: BmpMode, policy: Policy, h: &Header, alpha: bool)
    requires
        match crate::model::mh_policy(bytes, bm, policy) { Ok(m) => hdr_matches(m, *h), Err(_) => false },
        alpha == crate::model::rle_or_32bpp(h.spec_depth(), h.spec_compression()),
    ensures alpha == crate::model::mh_policy_rle_or_32bpp(bytes, bm, policy),
{
    reveal(crate::model::mh_policy_rle_or_32bpp);
    match crate::model::mh_policy(bytes, bm, policy) {
        Ok(m) => {
            crate::model::lemma_mh_partial_none(bytes, bm, policy, m);
            lemma_complete_alpha(bytes, bm, policy, h, m);
        },
        Err(_) => {},
    }
}

fn decode_complete_policy(bytes: &[u8], bm: BmpMode, policy: Policy, h: Header, alpha: bool,
    steps: &mut Ghost<nat>)
    -> (r: (Result<Decoded, Error>, bool))
    requires header_inv(bytes@, h), header_inv_light(&h),
        match crate::model::mh_policy(bytes@, bm, policy) { Ok(m) => hdr_matches(m, h), Err(_) => false },
    ensures
        final(steps)@ as int <= old(steps)@ as int
            + 16 * (bytes@.len() + policy_checked_n(bytes@, bm, policy)),
        r.1 == crate::model::recovered_policy(bytes@, bm, policy),
        decode_policy_mp_eq(r.0, bytes@, bm, policy),
        // Stated with the passed-in concrete value rather than `MH_policy`:
        // naming the model predicate here unfolds `mh_policy` at every return
        // site and pushes this body past its rlimit. `decode_policy_tracked`
        // computes `alpha` from the header and ties it to the model with
        // `lemma_alpha_value` before this call.
        match r.0 {
            Ok(d) => d.rle_or_32bpp == alpha,
            Err(_) => true,
        },
        match r.0 {
            Ok(d) => 1 <= d.width as int <= 8192 && 1 <= d.height as int <= 8192
                && d.rgba@.len() == 4 * (d.width as int * d.height as int)
                && d.rgba@.len() <= 67108864,
            Err(_) => true,
        },
 {
    let ghost start_steps = steps@;
    let recovered = false;
    proof {
        reveal(decode_policy_mp_eq);
    }
    let ghost m = ghost_mh_policy(bytes@, bm, policy, &h);
    proof {
        lemma_policy_header_geometry(bytes@, bm, policy, m);
        assert(crate::model::mh_policy(bytes@, bm, policy) == Ok(m));
        assert(hdr_matches(m, h));
        lemma_hrec_matches(m, &h);
        assert(hrec(&h) == m);
        reveal(hdr_matches);
        reveal(hrec);
        reveal(header_record);
        reveal(crate::dib::bmp_mode_to_mode);
        assert(h.mode == m.mode);
        assert(m.mode == Mode::BmpFile || m.mode == Mode::BareDib);
        assert(h.mode == Mode::BmpFile || h.mode == Mode::BareDib);
        // R7 palette geometry: the record's own `palette_end` and the
        // executable's `header_inv` palette bound name the same sum.

        reveal(header_inv);
        assert(hrec(&h) == m);
        assert(m.palette_end == h.spec_palette_end());
        assert(h.spec_palette_end() == h.palette_end as int);
        assert(m.palette_end == m.palette_start + m.table_count * m.palette_stride);
        assert((policy.header_bytes == HeaderBytesOptions::ParsedFields && m.table_count == 0) || m.palette_end <= bytes@.len());
        assert((policy.header_bytes == HeaderBytesOptions::ParsedFields && m.table_count == 0) || m.palette_start + m.table_count * m.palette_stride <= bytes@.len());
    }
    proof {
        assert(policy_checked_n(bytes@, bm, policy) == h.spec_n());
        reveal(header_inv_light);
        assert(h.spec_height() <= h.spec_n()) by (nonlinear_arith)
            requires 1 <= h.spec_width(), 1 <= h.spec_height(),
                h.spec_n() == h.spec_width() * h.spec_height();
        lemma_t12_phase_budget(bytes@.len() as int, h.spec_n(), h.spec_height());
    }
    let raw = h.compression == 0 || h.compression == 3;
    let from_gap = policy_indexed_gap(&h, policy);
    let from_gdk_offset: bool = match policy.offset {
        OffsetOptions::Gdk => h.is_bmp && h.bits <= 8u32 && (h.pixel_start < h.palette_end || h.pixel_start
            > bytes.len()),
        _ => false,
    };
    let from_gdk = from_gap || from_gdk_offset;
    let from_gdk_rgb: bool = match policy.offset {
        OffsetOptions::Gdk => h.is_bmp && h.bits > 8u32 && h.compression == 0u32 && h.pixel_start
            > bytes.len(),
        _ => false,
    };
    proof {
        reveal(crate::model::gdk_blank);
        reveal(crate::model::gdk_rgb_reject);
        assert(from_gap == crate::model::indexed_gap_blank(m, policy));
        reveal(hdr_matches);
        assert(m.depth == h.spec_depth() && m.pixel_start == h.spec_pixel_start() && m.palette_end
            == h.spec_palette_end() && m.compression == h.spec_compression());
        assert(h.spec_depth() == h.bits as int && h.spec_pixel_start() == h.pixel_start as int
            && h.spec_palette_end() == h.palette_end as int && h.spec_compression()
            == h.compression as int);
        assert(bytes@.len() == bytes.len());
        if from_gdk {
            assert(crate::model::gdk_blank(bytes@, m, policy));
        }
        if from_gdk_rgb {
            assert(crate::model::gdk_rgb_reject(bytes@, m, policy));
        }
        if !from_gdk {
            assert(!crate::model::gdk_blank(bytes@, m, policy));
        }
        if !from_gdk_rgb {
            assert(!crate::model::gdk_rgb_reject(bytes@, m, policy));
        }
    }
    if from_gdk_rgb {
        proof {
            reveal(decode_policy_mp_eq);
            crate::model::lemma_mp_policy_gdk_rgb(bytes@, bm, policy, m);
        }
        return (Err(Error::InvalidPixelOffset), recovered);
    }
    let mut out = alloc_output(&h, steps);
    proof { assert(steps@ == start_steps + 4 * h.spec_n()); }
    if from_gdk {
        // POLICY-SPEC `offset = Gdk`: an invalid indexed offset blanks the image.
        let transparent = h.compression == 1u32 || h.compression == 2u32;
        fill_blank(&mut out, transparent, steps);
        proof {
            assert(out@ == (if transparent {
                crate::model::zero_bytes(4 * m.n)
            } else {
                crate::model::opaque_fill(4 * m.n)
            }));
            if transparent {
                assert(out@ == crate::model::zero_bytes(4 * m.n));
            }
            let recovered = crate::model::mh_recovered_so_far(bytes@, m, policy);
            assert(recovered == crate::model::recovered_policy(bytes@, bm, policy)) by {
                reveal(crate::model::recovered_policy);
        reveal(crate::model::pixel_recovered);
                reveal(crate::model::mh_recovered_so_far);
                reveal(crate::model::gdk_blank);
                reveal(crate::model::mp_policy);
                assert((policy.header_bytes == HeaderBytesOptions::ParsedFields && m.table_count == 0) || m.palette_start + m.table_count * m.palette_stride <= bytes@.len());
                assert(crate::model::gdk_blank(bytes@, m, policy));
                if crate::model::mp_policy(bytes@, bm, policy) == Ok((
                    m.width,
                    m.height,
                    (if transparent {
                        crate::model::zero_bytes(4 * m.n)
                    } else {
                        crate::model::opaque_fill(4 * m.n)
                    }),
                )) {
                }

            }
            reveal(decode_policy_mp_eq);
            assert(decode_policy_mp_eq(
                Ok(Decoded { width: h.width, height: h.height, rgba: out, rle_or_32bpp: alpha }),
                bytes@,
                bm,
                policy,
            ));
        }
        return (Ok(Decoded { width: h.width, height: h.height, rgba: out, rle_or_32bpp: alpha }), from_gdk);
    }
    let p0 = policy_pixel_start(&h, policy, Ghost(m));
    let zero_fill = match policy.short_pixels {
        ShortPixelsOptions::Reject => false,
        ShortPixelsOptions::ZeroFill => true,
    };
    let black = match policy.palette_index { PaletteIndexOptions::Black => true, _ => false };
    let scaled = match policy.scaling { ScalingOptions::Round => false, _ => true };
    let opaque = match policy.bitfield16_alpha { Bitfield16AlphaOptions::Opaque => true, _ => false };
    if raw && (zero_fill || black || scaled || opaque) {
        return finish_raw_zero(bytes, bm, &h, out, p0, policy, alpha, steps);
    }

    let allow_end = match policy.rle_eob {
        RleEobOptions::Required => false,
        RleEobOptions::AtInputEnd => true,
    };
    if p0 > bytes.len() && (raw || !allow_end) {
        proof {
            reveal(decode_policy_mp_eq);
            reveal(crate::model::offset_effective);
            if raw {
                crate::model::lemma_mp_policy_raw_trunc(bytes@, bm, policy, m);
            } else {
                crate::model::lemma_mp_policy_rle_trunc(bytes@, bm, policy, m);
            }
        }
        if raw {
            return (Err(Error::TruncatedPixels), recovered);
        } else {
            return (Err(Error::RleTruncated), recovered);
        }
    }
    proof {
        assert(p0 <= bytes.len() || (!raw && allow_end));
        if !raw {
            reveal(crate::model::offset_effective);
            assert(h.pixel_start == p0);
        }
    }
    let clipped = match policy.rle_overrun {
        RleOverrunOptions::Reject => false,
        RleOverrunOptions::Clip | RleOverrunOptions::Image => true,
    };
    let stop_eol = match policy.rle_eol {
        RleEolOptions::OnlyFinal => false,
        RleEolOptions::StopAtHeight => true,
    };
    let clamp_delta = match policy.rle_delta { RleDeltaOptions::ClampStop => true, _ => false };
    let nonpalette = match policy.rle_fill { RleFillOptions::Palette0 => false, _ => true };
    if !raw && (clipped || stop_eol || allow_end || black || clamp_delta || nonpalette) {
        return finish_rle_clipped(bytes, bm, &h, out, policy, alpha, steps);
    }
    let mut gcur: (usize, usize, usize) = (h.pixel_start, 0, 0);
    let p = if raw {
        decode_raw(bytes, &h, &mut out, p0, steps)
    } else {
        decode_rle(bytes, &h, &mut out, &mut gcur, steps)
    };
    proof {
        assert((bytes@.len() - p0 as int) / 2 <= bytes@.len() / 2)
            by (nonlinear_arith) requires 0 <= p0 <= bytes@.len();
        assert(steps@ as int <= start_steps as int
            + 16 * (bytes@.len() + policy_checked_n(bytes@, bm, policy)));
    }
    let pos = match p {
        Ok(pos) => pos,
        Err(e) => {
            proof {
                reveal(decode_policy_mp_eq);
                reveal(crate::model::offset_effective);
                reveal(crate::model::gdk_blank);
                reveal(crate::model::gdk_rgb_reject);
                if raw {
                    reveal(raw_done_eq);
                    reveal(crate::model::mr_raw_out);
                    crate::model::lemma_raw_full_proj(
                        bytes@, hrec(&h), p0 as int, 0, 0);
                    assert(crate::model::mr_raw_out(bytes@, m, crate::model::offset_effective(m, policy))
                        is Err);
                    assert((crate::model::mr_raw_out(bytes@, m, crate::model::offset_effective(m, policy))->Err_0).0
                        == crate::model::RawErr::Api(e));
                    crate::model::lemma_mp_policy_raw_err(bytes@, bm, policy, m, e);
                } else {
                    reveal(rle_exact);
                    reveal(crate::model::mr_out);
                    assert(crate::model::mr_out(bytes@, m, m.pixel_start, 0, 0) is Err);
                    assert((crate::model::mr_out(bytes@, m, m.pixel_start, 0, 0)->Err_0).0
                        == crate::model::RleErr::Api(e));
                    crate::model::lemma_mp_policy_rle_err(bytes@, bm, policy, m, e);
                }
            }
            return (Err(e), recovered);
        },
    };
    let ghost out_xor = out@;
    let mask_result = finish_mask_policy(bytes, &h, &mut out, pos, p0, steps, policy);
    match mask_result {
        Ok(()) => {},
        Err(e) => {
            proof {
                reveal(decode_policy_mp_eq);
                reveal(crate::model::mask_done_policy_eq);
                reveal(crate::model::offset_effective);
                reveal(crate::model::gdk_blank);
                reveal(crate::model::gdk_rgb_reject);
                assert(crate::model::mask_done_policy_eq(
                    bytes@, m, pos as int, out_xor, out@, Err(e), policy));
                crate::model::lemma_mp_policy_phase(
                    bytes@, m, pos as int, out_xor, out@, Err(e), policy);
                if raw {
                    crate::model::lemma_mp_policy_phase_err_raw(
                        bytes@, bm, policy, m, pos as int, out_xor, e);
                } else {
                    crate::model::lemma_mp_policy_phase_err_rle(
                        bytes@, bm, policy, m, pos as int, out_xor, e);
                }
            }
            return (Err(e), recovered);
        },
    }
    proof {
        reveal(decode_policy_mp_eq);
        reveal(crate::model::mask_done_policy_eq);
        reveal(crate::model::offset_effective);
        reveal(crate::model::gdk_blank);
        reveal(crate::model::gdk_rgb_reject);
        crate::model::lemma_mp_policy_phase(
            bytes@, m, pos as int, out_xor, out@, Ok(()), policy);
        let end = choose|end: int| crate::model::mp_mask_phase_policy(
            bytes@, m, pos as int, pos as int - crate::model::offset_effective(m, policy), out_xor, policy)
            == Ok((end, out@));
        if raw {
            reveal(raw_done_eq);
            crate::model::lemma_raw_full_proj(
                bytes@, hrec(&h), p0 as int, 0, 0);
            assert(crate::model::mr_raw_out(bytes@, m, crate::model::offset_effective(m, policy))
                == Ok((pos as int, out_xor)));
            crate::model::lemma_mp_policy_raw_assemble(
                bytes@, bm, policy, m, pos as int, out_xor, out@, end);
        } else {
            reveal(rle_exact);
            reveal(crate::model::mr_out);
            assert(crate::model::mr_out(bytes@, m, m.pixel_start, 0, 0)
                == Ok((pos as int, out_xor)));
            crate::model::lemma_mp_policy_rle_assemble(
                bytes@, bm, policy, m, pos as int, out_xor, out@, end);
        }
    }
    let recovered2 = policy_header_recovered(bytes, &h, policy);
    proof {
        reveal(crate::model::recovered_policy);
        reveal(crate::model::pixel_recovered);
        assert(recovered2 == crate::model::mh_recovered_so_far(bytes@, m, policy));
    }
    (Ok(Decoded { width: h.width, height: h.height, rgba: out, rle_or_32bpp: alpha }), recovered2)
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

/// The policy-free `has_alpha` witness, at the same quantification as
/// `t13_success_bounds`: on every success `d.rle_or_32bpp` is the 32-bpp/RLE rule
/// on `MH`'s validated record; an error carries no image. Discharged by
/// `decode`'s own postcondition.
#[allow(dead_code)]
fn t13_alpha_field(bytes: &[u8], mode: Mode) -> (r: Result<Decoded, Error>)
    ensures
        match r {
            Ok(d) => d.rle_or_32bpp == crate::model::mh_rle_or_32bpp(bytes@, mode),
            Err(_) => true,
        },
{
    decode(bytes, mode)
}

// --- Statement witnesses for the profiled entry point, at the same
// quantification as the `decode` witnesses above: every slice, both BMP modes
// and all four named profiles.

/// T9 · the profiled entry point equals `MP_policy` on every error, dimension
/// and RGBA byte, for every slice, both modes and every profile. Discharged by
/// `decode_with_policy`'s own `decode_policy_mp_eq` postcondition.
#[allow(dead_code)]
fn t9_policy(bytes: &[u8], bm: BmpMode, profile: BmpProfile) -> (r: (Result<Decoded, Error>, bool))
    ensures
        decode_policy_mp_eq(r.0, bytes@, bm, profile_policy(profile)),
{
    decode_with_policy(bytes, bm, profile)
}

/// The profiled `has_alpha` witness, at the same quantification as `t9_policy`:
/// on every success `d.rle_or_32bpp` is the 32-bpp/RLE rule on the validated
/// header selection `MH_policy`; an error carries no image. Discharged by
/// `decode_with_policy`'s own `decode_policy_alpha_eq` postcondition.
#[allow(dead_code)]
fn t9_policy_alpha(bytes: &[u8], bm: BmpMode, profile: BmpProfile) -> (r: (Result<Decoded, Error>, bool))
    ensures
        decode_policy_alpha_eq(r.0, bytes@, bm, profile_policy(profile)),
{
    decode_with_policy(bytes, bm, profile)
}

/// T10 · error precedence for the profiled entry point: for every error `e`,
/// `decode_with_policy(..).0 = Err(e)` iff `MP_policy(..) = Err(e)`.
#[allow(dead_code)]
fn t10_policy_precedence(bytes: &[u8], bm: BmpMode, profile: BmpProfile, e: Error)
    -> (r: (Result<Decoded, Error>, bool))
    ensures
        (r.0 == Err(e)) == (crate::model::mp_policy(bytes@, bm, profile_policy(profile)) == Err(e)),
{
    decode_with_policy(bytes, bm, profile)
}

/// T13 · success and resource bounds for the profiled entry point: every success
/// has `MP_policy`'s exact `(w, h, p)`, `1 <= w, h <= 8192`, a `4*w*h`
/// output of at most 64 MiB, and the header-derived `rle_or_32bpp` field.
#[allow(dead_code)]
fn t13_policy_success_bounds(bytes: &[u8], bm: BmpMode, profile: BmpProfile)
    -> (r: (Result<Decoded, Error>, bool))
    ensures
        decode_policy_alpha_eq(r.0, bytes@, bm, profile_policy(profile)),
        match r.0 {
            Ok(d) => {
                &&& crate::model::mp_policy(bytes@, bm, profile_policy(profile))
                    == Ok((d.width as int, d.height as int, d.rgba@))
                &&& 1 <= d.width as int <= 8192
                &&& 1 <= d.height as int <= 8192
                &&& d.rgba@.len() == 4 * (d.width as int * d.height as int)
                &&& d.rgba@.len() <= 67108864
            },
            Err(_) => true,
        },
{
    decode_with_policy(bytes, bm, profile)
}

/// Named-profile recovery-flag witness, at full quantification: for every
/// slice, both BMP modes and all four named profiles, the flag is exactly the
/// frozen spec's closed-event indicator on the selected record. The universal
/// charged-work bound is stated on `decode_policy_tracked`, the executable called
/// by all four named profiles and by test-only custom records; it has no policy
/// precondition.
#[allow(dead_code)]
fn t1_policy_recovery_flag(bytes: &[u8], bm: BmpMode, profile: BmpProfile)
    -> (r: (Result<Decoded, Error>, bool))
    ensures
        r.1 == crate::model::recovered_policy(bytes@, bm, profile_policy(profile)),
        decode_policy_mp_eq(r.0, bytes@, bm, profile_policy(profile)),
{
    decode_with_policy(bytes, bm, profile)
}

/// The P1 statement, kept as is for exactly the two profiles it covers.
/// `Strict` and `Recommended` tolerate only `Ignore`d `bfSize`/`biSizeImage`
/// declarations, so no entry of the closed data-affecting event list is
/// reachable: `recovered_policy` is `false` on every input, and the flag with
/// it. This is the same quantification (every slice, both BMP modes) the P1
/// witness had; the two compatibility profiles are covered by
/// `t1_policy_recovery_flag` above, which states the general closed-list
/// equality instead of `false`.
#[allow(dead_code)]
fn t1_policy_recovery_strict(bytes: &[u8], bm: BmpMode) -> (r: (Result<Decoded, Error>, bool))
    ensures
        r.1 == false,
        decode_policy_mp_eq(r.0, bytes@, bm, strict_policy()),
{
    decode_with_policy(bytes, bm, BmpProfile::Strict)
}

/// As `t1_policy_recovery_strict`, for `Recommended`.
#[allow(dead_code)]
fn t1_policy_recovery_recommended(bytes: &[u8], bm: BmpMode)
    -> (r: (Result<Decoded, Error>, bool))
    ensures
        r.1 == false,
        decode_policy_mp_eq(r.0, bytes@, bm, recommended_policy()),
{
    decode_with_policy(bytes, bm, BmpProfile::Recommended)
}

/// Experimental custom records cannot enter a release decoder API.
#[cfg(test)]
pub(crate) fn decode_custom_policy(bytes: &[u8], bm: BmpMode, policy: Policy)
    -> (r: (Result<Decoded, Error>, bool))
    ensures decode_policy_mp_eq(r.0, bytes@, bm, policy),
        decode_policy_alpha_eq(r.0, bytes@, bm, policy),
        r.1 == crate::model::recovered_policy(bytes@, bm, policy),
{
    let mut steps = Ghost(0nat);
    decode_policy_tracked(bytes, bm, policy, &mut steps)
}

} // verus!
