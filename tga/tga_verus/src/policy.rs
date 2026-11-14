//! The TGA policy layer (POLICY-SPEC.md): the typed record of the thirteen
//! option fields, the four frozen named profiles, and the record constructors.
//!
//! `Policy`'s fields are `pub` because Verus treats a struct with any non-`pub`
//! field as opaque everywhere, and `crate::policy_model`'s spec functions read
//! those fields. Cross-crate construction is nonetheless impossible: the struct
//! is `#[non_exhaustive]` (an external literal is rejected) and the only
//! constructors, `strict_policy_exec`/`profile_policy_exec`, are `pub(crate)`. A
//! release caller can therefore not obtain a record with custom field values nor
//! mutate one; the only release decoder is `decode_with_policy`, which takes a
//! `Profile`, and the custom-record decoder `decode_custom_policy` exists only
//! under `#[cfg(test)]`.
//!
//! The profile selector is `#[non_exhaustive]` too, and `Default` is `Strict`.

use vstd::prelude::*;

verus! {

// ---------------------------------------------------------------------------
// The thirteen option enums. The first value of each is Strict's value
// (POLICY-SPEC "Named profiles", column Strict).
// ---------------------------------------------------------------------------

/// R12 `trailing`: what may follow the last pixel. `FooterOnly` is 1.0 (the
/// suffix must be empty or the 26-byte footer structure); `Ignore` accepts any
/// suffix, including a partial footer, and reads none of it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum TrailingOptions {
    FooterOnly,
    Ignore,
}

/// R11 `rle_overrun`: the over-long-packet rule. `Reject` is 1.0 (`RleOverrun`
/// before reading packet data); `ClipPacket` requires the whole packet and
/// paints `min(count, N-produced)` samples; `ClipSamples` requires only the
/// samples it paints for a literal packet (a repeat packet still requires its
/// one sample).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum RleOverrunOptions {
    Reject,
    ClipPacket,
    ClipSamples,
}

/// R5 `attribute_bits`: the descriptor attribute count. `Exact` is 1.0;
/// `Lenient` replaces a rejected count by 8 when the element has an 8-bit
/// attribute slot (32-bit colour, 16-bit grey) and by 0 otherwise;
/// `ImageTable` is 1.0's set except that 8-bit grey allows {0,8} (8 treated
/// like 0) and 16-bit grey allows only {8}.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum AttributeBitsOptions {
    Exact,
    Lenient,
    ImageTable,
}

/// R2/R4 `map_header`: how the colour-map flag and header are read. `Core` is
/// 1.0; `IgnoreAbsent` skips the flag-0 zero-field check on a non-indexed
/// image; `Gdk` treats any nonzero flag as present with no range check;
/// `Image` treats only flag 1 as present, with no range check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum MapHeaderOptions {
    Core,
    IgnoreAbsent,
    Gdk,
    Image,
}

/// R3 `depths`: the supported-depth table. `Core` is 1.0; `Gdk` is
/// indexed 8, true-colour 16/24/32, grey 8/16; `Image` is Core plus an
/// `UnsupportedPixelDepth` for an indexed image whose index depth exceeds its
/// entry depth (checked immediately after the map checks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DepthsOptions {
    Core,
    Gdk,
    Image,
}

/// R8 `alpha`: which stored bytes carry alpha. `AttributeBits` is 1.0 (stored
/// alpha only when the effective count is 8); `Stored` always takes the stored
/// byte for 32-bit true-colour, 32-bit palette entries and 16-bit grey.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum AlphaOptions {
    AttributeBits,
    Stored,
}

/// R8 `map16_alpha`: a 16-bit palette entry with effective count 1.
/// `Bit15` is 1.0 (alpha 255 iff bit 15 is set); `Opaque` is always 255.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Map16AlphaOptions {
    Bit15,
    Opaque,
}

/// R8 `rgb5`: five-bit channel expansion for 15/16-bit true-colour.
/// `Replicate` is 1.0 (`(c<<3)|(c>>2)`); `Round` is `(255c+15) div 31`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Rgb5Options {
    Replicate,
    Round,
}

/// R8 `map5`: as `rgb5`, for 15/16-bit palette entries. `Replicate` is 1.0;
/// `Round` is `(255c+15) div 31`; `GdkSwapShift` is `(v&31)<<3`,
/// `((v>>5)&31)<<3`, `((v>>10)&31)<<3`, with no low-bit replication.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Map5Options {
    Replicate,
    Round,
    GdkSwapShift,
}

/// R4/R8 `map_index`: entry `j = index - first` (1.0, `Absolute`) or
/// `j = index` with `first` ignored (`ZeroBased`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum MapIndexOptions {
    Absolute,
    ZeroBased,
}

/// R8 `index_range`: `Reject` is 1.0 (`ColorIndexOutOfRange` unless
/// `0 <= j < length`); `Black` substitutes `(0,0,0,0)` for a 32-bit palette
/// entry and `(0,0,0,255)` otherwise, and sets `recovered`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum IndexRangeOptions {
    Reject,
    Black,
}

/// R2 `interleave`: `Reject` is 1.0 (`UnsupportedInterleave` when descriptor
/// bits 6-7 are nonzero); `Ignore` decodes as if they were zero.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum InterleaveOptions {
    Reject,
    Ignore,
}

/// R1 `image_type`: `Core` is 1.0's six types; `GreyFallback` decodes every
/// other type byte, including 0, as raw greyscale.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ImageTypeOptions {
    Core,
    GreyFallback,
}

/// All thirteen retained option fields (POLICY-SPEC "Option fields"). Every
/// combination has the same safety, termination, allocation and work bounds;
/// only the four named profiles have exact-semantics proofs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct Policy {
    pub trailing: TrailingOptions,
    pub rle_overrun: RleOverrunOptions,
    pub attribute_bits: AttributeBitsOptions,
    pub map_header: MapHeaderOptions,
    pub depths: DepthsOptions,
    pub alpha: AlphaOptions,
    pub map16_alpha: Map16AlphaOptions,
    pub rgb5: Rgb5Options,
    pub map5: Map5Options,
    pub map_index: MapIndexOptions,
    pub index_range: IndexRangeOptions,
    pub interleave: InterleaveOptions,
    pub image_type: ImageTypeOptions,
}

/// The profile selector. `Strict` is TGA-CORE-SPEC 1.0 exactly; `RecommendedBeta`
/// is Strict plus the four error-site relaxations (POLICY-SPEC D2). The two
/// compatibility profiles `GdkCompat-2.42.12` and `ImageCrateCompat-0.25.10`
/// reproduce the measured gdk-pixbuf and image-rs byte semantics on their
/// versioned records (POLICY-SPEC "Named profiles"); POLICY-SPEC D5 keeps their
/// names versioned. `Default` is `Strict`. The matches in `profile_policy` and
/// `profile_policy_exec` are exhaustive over this variant set, so a new profile
/// is a compile error there rather than a silent default.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Profile {
    Strict,
    RecommendedBeta,
    GdkCompat,
    ImageCrateCompat,
}

impl core::default::Default for Profile {
    fn default() -> Self {
        Profile::Strict
    }
}

// ---------------------------------------------------------------------------
// The four named profile records (POLICY-SPEC "Named profiles").
// ---------------------------------------------------------------------------

/// `Strict`'s record: every option keeps its 1.0 value.
pub open spec fn strict_policy() -> Policy {
    Policy {
        trailing: TrailingOptions::FooterOnly,
        rle_overrun: RleOverrunOptions::Reject,
        attribute_bits: AttributeBitsOptions::Exact,
        map_header: MapHeaderOptions::Core,
        depths: DepthsOptions::Core,
        alpha: AlphaOptions::AttributeBits,
        map16_alpha: Map16AlphaOptions::Bit15,
        rgb5: Rgb5Options::Replicate,
        map5: Map5Options::Replicate,
        map_index: MapIndexOptions::Absolute,
        index_range: IndexRangeOptions::Reject,
        interleave: InterleaveOptions::Reject,
        image_type: ImageTypeOptions::Core,
    }
}

/// `RecommendedBeta`'s record (POLICY-SPEC D2): Strict except the four error-site
/// relaxations `trailing = Ignore`, `rle_overrun = ClipPacket`,
/// `attribute_bits = Lenient`, `map_header = IgnoreAbsent`. Every pixel-rule
/// field stays at its Strict value.
pub open spec fn recommended_beta_policy() -> Policy {
    Policy {
        trailing: TrailingOptions::Ignore,
        rle_overrun: RleOverrunOptions::ClipPacket,
        attribute_bits: AttributeBitsOptions::Lenient,
        map_header: MapHeaderOptions::IgnoreAbsent,
        depths: DepthsOptions::Core,
        alpha: AlphaOptions::AttributeBits,
        map16_alpha: Map16AlphaOptions::Bit15,
        rgb5: Rgb5Options::Replicate,
        map5: Map5Options::Replicate,
        map_index: MapIndexOptions::Absolute,
        index_range: IndexRangeOptions::Reject,
        interleave: InterleaveOptions::Reject,
        image_type: ImageTypeOptions::Core,
    }
}

/// `GdkCompat-2.42.12`'s record: the measured gdk-pixbuf 2.42.12 assignment on
/// all thirteen retained fields.
pub open spec fn gdk_compat_policy() -> Policy {
    Policy {
        trailing: TrailingOptions::Ignore,
        rle_overrun: RleOverrunOptions::ClipPacket,
        attribute_bits: AttributeBitsOptions::Lenient,
        map_header: MapHeaderOptions::Gdk,
        depths: DepthsOptions::Gdk,
        alpha: AlphaOptions::Stored,
        map16_alpha: Map16AlphaOptions::Opaque,
        rgb5: Rgb5Options::Replicate,
        map5: Map5Options::GdkSwapShift,
        map_index: MapIndexOptions::ZeroBased,
        index_range: IndexRangeOptions::Black,
        interleave: InterleaveOptions::Reject,
        image_type: ImageTypeOptions::Core,
    }
}

/// `ImageCrateCompat-0.25.10`'s record: the measured image 0.25.10 assignment
/// on all thirteen retained fields.
pub open spec fn image_crate_compat_policy() -> Policy {
    Policy {
        trailing: TrailingOptions::Ignore,
        rle_overrun: RleOverrunOptions::ClipSamples,
        attribute_bits: AttributeBitsOptions::ImageTable,
        map_header: MapHeaderOptions::Image,
        depths: DepthsOptions::Image,
        alpha: AlphaOptions::Stored,
        map16_alpha: Map16AlphaOptions::Opaque,
        rgb5: Rgb5Options::Round,
        map5: Map5Options::Round,
        map_index: MapIndexOptions::Absolute,
        index_range: IndexRangeOptions::Reject,
        interleave: InterleaveOptions::Ignore,
        image_type: ImageTypeOptions::GreyFallback,
    }
}

/// The profile selector's record (POLICY-SPEC "Named profiles").
pub open spec fn profile_policy(p: Profile) -> Policy {
    match p {
        Profile::Strict => strict_policy(),
        Profile::RecommendedBeta => recommended_beta_policy(),
        Profile::GdkCompat => gdk_compat_policy(),
        Profile::ImageCrateCompat => image_crate_compat_policy(),
    }
}

// ---------------------------------------------------------------------------
// The executable record constructors (pub(crate): the record is internal).
// ---------------------------------------------------------------------------

/// The executable wrapper for `strict_policy`.
pub(crate) fn strict_policy_exec() -> (r: Policy)
    ensures
        r == strict_policy(),
{
    Policy {
        trailing: TrailingOptions::FooterOnly,
        rle_overrun: RleOverrunOptions::Reject,
        attribute_bits: AttributeBitsOptions::Exact,
        map_header: MapHeaderOptions::Core,
        depths: DepthsOptions::Core,
        alpha: AlphaOptions::AttributeBits,
        map16_alpha: Map16AlphaOptions::Bit15,
        rgb5: Rgb5Options::Replicate,
        map5: Map5Options::Replicate,
        map_index: MapIndexOptions::Absolute,
        index_range: IndexRangeOptions::Reject,
        interleave: InterleaveOptions::Reject,
        image_type: ImageTypeOptions::Core,
    }
}

/// The executable wrapper for `profile_policy`: `decode_with_policy` calls it,
/// so the record never escapes the crate.
pub(crate) fn profile_policy_exec(p: Profile) -> (r: Policy)
    ensures
        r == profile_policy(p),
{
    match p {
        Profile::Strict => Policy {
            trailing: TrailingOptions::FooterOnly,
            rle_overrun: RleOverrunOptions::Reject,
            attribute_bits: AttributeBitsOptions::Exact,
            map_header: MapHeaderOptions::Core,
            depths: DepthsOptions::Core,
            alpha: AlphaOptions::AttributeBits,
            map16_alpha: Map16AlphaOptions::Bit15,
            rgb5: Rgb5Options::Replicate,
            map5: Map5Options::Replicate,
            map_index: MapIndexOptions::Absolute,
            index_range: IndexRangeOptions::Reject,
            interleave: InterleaveOptions::Reject,
            image_type: ImageTypeOptions::Core,
        },
        Profile::RecommendedBeta => Policy {
            trailing: TrailingOptions::Ignore,
            rle_overrun: RleOverrunOptions::ClipPacket,
            attribute_bits: AttributeBitsOptions::Lenient,
            map_header: MapHeaderOptions::IgnoreAbsent,
            depths: DepthsOptions::Core,
            alpha: AlphaOptions::AttributeBits,
            map16_alpha: Map16AlphaOptions::Bit15,
            rgb5: Rgb5Options::Replicate,
            map5: Map5Options::Replicate,
            map_index: MapIndexOptions::Absolute,
            index_range: IndexRangeOptions::Reject,
            interleave: InterleaveOptions::Reject,
            image_type: ImageTypeOptions::Core,
        },
        Profile::GdkCompat => Policy {
            trailing: TrailingOptions::Ignore,
            rle_overrun: RleOverrunOptions::ClipPacket,
            attribute_bits: AttributeBitsOptions::Lenient,
            map_header: MapHeaderOptions::Gdk,
            depths: DepthsOptions::Gdk,
            alpha: AlphaOptions::Stored,
            map16_alpha: Map16AlphaOptions::Opaque,
            rgb5: Rgb5Options::Replicate,
            map5: Map5Options::GdkSwapShift,
            map_index: MapIndexOptions::ZeroBased,
            index_range: IndexRangeOptions::Black,
            interleave: InterleaveOptions::Reject,
            image_type: ImageTypeOptions::Core,
        },
        Profile::ImageCrateCompat => Policy {
            trailing: TrailingOptions::Ignore,
            rle_overrun: RleOverrunOptions::ClipSamples,
            attribute_bits: AttributeBitsOptions::ImageTable,
            map_header: MapHeaderOptions::Image,
            depths: DepthsOptions::Image,
            alpha: AlphaOptions::Stored,
            map16_alpha: Map16AlphaOptions::Opaque,
            rgb5: Rgb5Options::Round,
            map5: Map5Options::Round,
            map_index: MapIndexOptions::Absolute,
            index_range: IndexRangeOptions::Reject,
            interleave: InterleaveOptions::Ignore,
            image_type: ImageTypeOptions::GreyFallback,
        },
    }
}

} // verus!
