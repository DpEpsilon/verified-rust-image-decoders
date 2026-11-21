//! The policy record (`../../policy-draft/OPTIONS.md`, policy draft 0.2): 30
//! fields, and the profiles that select a record.
//!
//! M1b implements two profiles, `Strict` (the core of SPEC.md) and
//! `RecommendedBeta`. Each field enum lists only the values those two records
//! use; the compatibility profiles (`GdkCompat-2.42.12`,
//! `ImageCrateCompat-0.25.10`) and their extra values arrive with milestone P2,
//! which is why `Profile` is `#[non_exhaustive]`. A record is not constructible
//! outside the crate except through `Profile` (and, for this crate's own
//! tests, through the `test-records` feature).
//!
//! **Held out of Stage 1.** RecommendedBeta is not frozen (PNG-Q7: it freezes
//! at Stage 2, after the corpus pass). Its fifteen non-Strict rows are written
//! once, in `RECOMMENDED_BETA` below, and nowhere else; a Stage-2 change to a
//! row edits that constant (and, for a value no profile used before, the
//! field's enum).

use vstd::prelude::*;

verus! {

/// A decoding profile. `Strict` is PNG-CORE-SPEC 1.0; `RecommendedBeta` is the
/// provisional lenient profile (unfrozen). Obligation R: on every input Strict
/// accepts, RecommendedBeta returns the identical RGBA with `recovered = false`.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Profile {
    #[default]
    Strict,
    RecommendedBeta,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrcCritical { Reject }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrcAncillary { Reject, SkipExceptFdat }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iend { Strict, Lenient }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChunkType { Reject }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReservedBit { Accept }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AfterIdat { Full }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MissingIend { Reject, AtEof }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eof { Reject }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trailing { Reject, Ignore }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AncillaryOrder { Reject, Drop }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AncillaryDuplicate { Reject, FirstWins }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AncillaryValues { Reject, Drop }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrnsInvalid { Reject, Drop }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlteUnexpected { Reject, Ignore }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PaletteSize { Reject, Allow }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PaletteIndex { Reject, Black }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adler { Check }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StreamEnd { Require }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ZlibTail { Reject, IgnoreTail }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Window { Enforce, Ignore }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SingleCode { Accept }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImageDataError { Reject }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Depth16 { Round }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApngInStill { Validate, Ignore }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApngErrors { Reject, Static }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blend { Exact }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisposePrevious { Spec }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Apng16 { Composite }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SubframeInterlace { Exact }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Apng { On }

/// The 30-field policy record. Field order and names follow OPTIONS.md.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Policy {
    pub crc_critical: CrcCritical,
    pub crc_ancillary: CrcAncillary,
    pub iend: Iend,
    pub chunk_type: ChunkType,
    pub reserved_bit: ReservedBit,
    pub after_idat: AfterIdat,
    pub missing_iend: MissingIend,
    pub eof: Eof,
    pub trailing: Trailing,
    pub ancillary_order: AncillaryOrder,
    pub ancillary_duplicate: AncillaryDuplicate,
    pub ancillary_values: AncillaryValues,
    pub trns_invalid: TrnsInvalid,
    pub plte_unexpected: PlteUnexpected,
    pub palette_size: PaletteSize,
    pub palette_index: PaletteIndex,
    pub adler: Adler,
    pub stream_end: StreamEnd,
    pub zlib_tail: ZlibTail,
    pub window: Window,
    pub single_code: SingleCode,
    pub image_data_error: ImageDataError,
    pub depth16: Depth16,
    pub apng_in_still: ApngInStill,
    pub apng_errors: ApngErrors,
    pub blend: Blend,
    pub dispose_previous: DisposePrevious,
    pub apng16: Apng16,
    pub subframe_interlace: SubframeInterlace,
    pub apng: Apng,
}

/// Strict: PNG-CORE-SPEC 1.0 (frozen).
pub const STRICT: Policy = Policy {
    crc_critical: CrcCritical::Reject,
    crc_ancillary: CrcAncillary::Reject,
    iend: Iend::Strict,
    chunk_type: ChunkType::Reject,
    reserved_bit: ReservedBit::Accept,
    after_idat: AfterIdat::Full,
    missing_iend: MissingIend::Reject,
    eof: Eof::Reject,
    trailing: Trailing::Reject,
    ancillary_order: AncillaryOrder::Reject,
    ancillary_duplicate: AncillaryDuplicate::Reject,
    ancillary_values: AncillaryValues::Reject,
    trns_invalid: TrnsInvalid::Reject,
    plte_unexpected: PlteUnexpected::Reject,
    palette_size: PaletteSize::Reject,
    palette_index: PaletteIndex::Reject,
    adler: Adler::Check,
    stream_end: StreamEnd::Require,
    zlib_tail: ZlibTail::Reject,
    window: Window::Enforce,
    single_code: SingleCode::Accept,
    image_data_error: ImageDataError::Reject,
    depth16: Depth16::Round,
    apng_in_still: ApngInStill::Validate,
    apng_errors: ApngErrors::Reject,
    blend: Blend::Exact,
    dispose_previous: DisposePrevious::Spec,
    apng16: Apng16::Composite,
    subframe_interlace: SubframeInterlace::Exact,
    apng: Apng::On,
};

/// RecommendedBeta (provisional, policy draft 0.2, NOT frozen; held out of
/// Stage 1): Strict with thirteen fields changed, twelve full relaxations and
/// the narrowed `zlib_tail = IgnoreTail` (PNG-Q1, Q2, Q11; council RB-S2 kept
/// `missing_iend` and `trns_invalid` at Strict's `Reject`). The only place
/// these rows are written.
pub const RECOMMENDED_BETA: Policy = Policy {
    crc_ancillary: CrcAncillary::SkipExceptFdat,
    iend: Iend::Lenient,
    trailing: Trailing::Ignore,
    ancillary_order: AncillaryOrder::Drop,
    ancillary_duplicate: AncillaryDuplicate::FirstWins,
    ancillary_values: AncillaryValues::Drop,
    plte_unexpected: PlteUnexpected::Ignore,
    palette_size: PaletteSize::Allow,
    palette_index: PaletteIndex::Black,
    zlib_tail: ZlibTail::IgnoreTail,
    window: Window::Ignore,
    apng_in_still: ApngInStill::Ignore,
    apng_errors: ApngErrors::Static,
    ..STRICT
};

} // verus!

impl Profile {
    /// The record a profile selects.
    pub const fn policy(self) -> Policy {
        match self {
            Profile::Strict => STRICT,
            Profile::RecommendedBeta => RECOMMENDED_BETA,
        }
    }
}

verus! {

impl Profile {
    pub closed spec fn walk_model(self) -> crate::document_model::Policy {
        match self {Self::Strict=>STRICT.walk_view(),Self::RecommendedBeta=>RECOMMENDED_BETA.walk_view()}
    }
    pub(crate) fn checked_policy(self) -> (r: Policy)
        ensures r==match self {Self::Strict=>STRICT,Self::RecommendedBeta=>RECOMMENDED_BETA},r.walk_view()==self.walk_model(),
    {match self {Self::Strict=>STRICT,Self::RecommendedBeta=>RECOMMENDED_BETA}}
}

impl Policy {
    /// The five inflate fields (SPEC P10.6, PNG-Q12) as `inflate_verus` options.
    pub(crate) fn inflate_options(&self) -> (r: inflate_verus::InflateOptions)
        ensures (r.window==inflate_verus::Window::Enforce)==(self.window==Window::Enforce),
            (r.zlib_tail==inflate_verus::ZlibTail::IgnoreTail)==(self.zlib_tail==ZlibTail::IgnoreTail),
    {
        let mut o = inflate_verus::InflateOptions::STRICT;
        o.zlib_tail = match self.zlib_tail {
            ZlibTail::Reject => inflate_verus::ZlibTail::Reject,
            ZlibTail::IgnoreTail => inflate_verus::ZlibTail::IgnoreTail,
        };
        o.window = match self.window {
            Window::Enforce => inflate_verus::Window::Enforce,
            Window::Ignore => inflate_verus::Window::Ignore,
        };
        o
    }
}

} // verus!
