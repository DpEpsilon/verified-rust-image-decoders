//! The policy record (`../policy-draft/OPTIONS.md`, policy draft 0.2): 22
//! pixel fields plus the metadata field `loop_scan`, and the profiles that
//! select a record.
//!
//! Release 0.1 implements two profiles, `Strict` (the core of SPEC.md) and
//! `RecommendedBeta`. Each field enum lists only the values those two records
//! use; the compatibility profiles (`GdkCompat-2.42.12`,
//! `ImageCrateCompat-0.25.10`) and their extra values arrive with milestone P2,
//! which is why `Profile` is `#[non_exhaustive]`. A record is not constructible
//! outside the crate except through `Profile` (and, for this crate's own
//! tests, through the `test-records` feature).

use vstd::prelude::*;

verus! {

/// A decoding profile. `Strict` is the supported GIF subset of SPEC.md;
/// `RecommendedBeta` is the provisional lenient profile (unfrozen until its
/// corpus pass is reviewed, Q9). Obligation R: on every input Strict accepts,
/// RecommendedBeta returns the identical result with `recovered = false`.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Profile {
    Strict,
    RecommendedBeta,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VersionRule { Known }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrameBounds { Reject, Clip }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RestoreOutside { Restore }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisposalUndefined { Reject, Keep }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GceSize { Exact }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GceDuplicate { Reject, LastWins }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GceScope { NextGraphic }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GceConsumers { Reject, ImageOnly }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GceOrphan { Reject, Ignore }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExtSizes { Exact, Ignore }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnknownExtension { Skip }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnknownBlock { Reject }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LzwMin { Spec, Range1to11Checked }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LzwInvalid { Reject }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LzwTail { Strict, StopAtRaster }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShortFrame { Reject }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Truncation { Reject }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IndexRange { Reject }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MissingTrailer { Reject, AtEof }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trailing { Reject, Ignore }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImageDataMode { Whole }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmptyFirstSubblock { Terminator }
/// Metadata field (Q15): which NETSCAPE2.0 sub-blocks set `loop_count`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoopScan { EverySubBlock, FirstSubBlockOnly }

/// The 22-field policy record plus `loop_scan`. Field order and names follow
/// OPTIONS.md.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Policy {
    pub version: VersionRule,
    pub frame_bounds: FrameBounds,
    pub restore_outside: RestoreOutside,
    pub disposal_undefined: DisposalUndefined,
    pub gce_size: GceSize,
    pub gce_duplicate: GceDuplicate,
    pub gce_scope: GceScope,
    pub gce_consumers: GceConsumers,
    pub gce_orphan: GceOrphan,
    pub ext_sizes: ExtSizes,
    pub unknown_extension: UnknownExtension,
    pub unknown_block: UnknownBlock,
    pub lzw_min: LzwMin,
    pub lzw_invalid: LzwInvalid,
    pub lzw_tail: LzwTail,
    pub short_frame: ShortFrame,
    pub truncation: Truncation,
    pub index_range: IndexRange,
    pub missing_trailer: MissingTrailer,
    pub trailing: Trailing,
    pub image_data: ImageDataMode,
    pub empty_first_subblock: EmptyFirstSubblock,
    pub loop_scan: LoopScan,
}

/// Strict: the core of SPEC.md (`policy_oracle.STRICT`).
pub(crate) const STRICT: Policy = Policy {
    version: VersionRule::Known,
    frame_bounds: FrameBounds::Reject,
    restore_outside: RestoreOutside::Restore,
    disposal_undefined: DisposalUndefined::Reject,
    gce_size: GceSize::Exact,
    gce_duplicate: GceDuplicate::Reject,
    gce_scope: GceScope::NextGraphic,
    gce_consumers: GceConsumers::Reject,
    gce_orphan: GceOrphan::Reject,
    ext_sizes: ExtSizes::Exact,
    unknown_extension: UnknownExtension::Skip,
    unknown_block: UnknownBlock::Reject,
    lzw_min: LzwMin::Spec,
    lzw_invalid: LzwInvalid::Reject,
    lzw_tail: LzwTail::Strict,
    short_frame: ShortFrame::Reject,
    truncation: Truncation::Reject,
    index_range: IndexRange::Reject,
    missing_trailer: MissingTrailer::Reject,
    trailing: Trailing::Reject,
    image_data: ImageDataMode::Whole,
    empty_first_subblock: EmptyFirstSubblock::Terminator,
    loop_scan: LoopScan::EverySubBlock,
};

/// RecommendedBeta: Strict with Q1's eight relaxations, `gce_consumers =
/// ImageOnly` and `lzw_min = Range1to11Checked` (`policy_oracle.RECOMMENDED_BETA`).
pub(crate) const RECOMMENDED_BETA: Policy = Policy {
    frame_bounds: FrameBounds::Clip,
    disposal_undefined: DisposalUndefined::Keep,
    gce_duplicate: GceDuplicate::LastWins,
    gce_consumers: GceConsumers::ImageOnly,
    gce_orphan: GceOrphan::Ignore,
    ext_sizes: ExtSizes::Ignore,
    lzw_min: LzwMin::Range1to11Checked,
    lzw_tail: LzwTail::StopAtRaster,
    missing_trailer: MissingTrailer::AtEof,
    trailing: Trailing::Ignore,
    ..STRICT
};

pub closed spec fn profile_record(p:Profile)->Policy {
    match p {Profile::Strict=>STRICT,Profile::RecommendedBeta=>RECOMMENDED_BETA}
}

pub(crate) proof fn lemma_profile_records()
    ensures profile_record(Profile::Strict)==STRICT,
        profile_record(Profile::RecommendedBeta)==RECOMMENDED_BETA,
{}

impl Profile {
    /// The record this profile selects.
    pub(crate) fn policy(self) -> (pol: Policy)
        ensures pol==profile_record(self),
    {
        match self {
            Profile::Strict => STRICT,
            Profile::RecommendedBeta => RECOMMENDED_BETA,
        }
    }
}

/// Test-only record constructors (feature `test-records`, enabled only by this
/// crate's own dev-dependency on itself).
#[cfg(feature = "test-records")]
impl Policy {
    #[doc(hidden)]
    pub fn of_profile(p: Profile) -> Policy {
        p.policy()
    }

    #[doc(hidden)]
    pub fn with_loop_scan_first_only(self) -> Policy {
        Policy { loop_scan: LoopScan::FirstSubBlockOnly, ..self }
    }
}

} // verus!
