//! Allocation-free zlib/DEFLATE, PNG-CORE-SPEC 1.0 P9–P10.
//!
//! Both verified public entries refine one independent sequence model, retain
//! produced prefixes on errors and return erased per-call work observations.
#![no_std]
#![forbid(unsafe_code)]

mod adler;
mod bits;
mod canonical;
mod classification;
mod huffman;
mod inflate;
#[cfg(verus_only)]
pub mod limit_mono;
pub mod limits;
#[cfg(verus_only)]
pub mod model;
pub mod segments;
#[cfg(verus_only)]
#[cfg(any(not(feature = "split"), feature = "split-fixed-policy"))]
mod witness_cases;
#[cfg(verus_only)]
#[cfg(any(not(feature = "split"), feature = "split-classes-blocks"))]
mod witness_classes;
#[cfg(verus_only)]
mod witness_checksum;
#[cfg(verus_only)]
mod witness_config;
#[cfg(verus_only)]
#[cfg(any(not(feature = "split"), feature = "split-config-cases"))]
mod witness_config_cases;
#[cfg(verus_only)]
mod witness_profile_bridge;
#[cfg(verus_only)]
#[cfg(any(not(feature = "split"), feature = "split-dynamic"))]
mod witness_profiles;
#[cfg(verus_only)]
mod witness_block_bridge;
#[cfg(verus_only)]
#[cfg(any(not(feature = "split"), feature = "split-classes-blocks"))]
mod witness_blocks;
#[cfg(verus_only)]
#[cfg(any(not(feature = "split"), feature = "split-dynamic"))]
mod witness_d4;
#[cfg(verus_only)]
mod witness_dynamic;
#[cfg(verus_only)]
#[cfg(any(not(feature = "split"), feature = "split-dynamic"))]
mod witness_dynamic_cases;
#[cfg(verus_only)]
#[cfg(any(not(feature = "split"), feature = "split-extended"))]
mod witness_extended;
#[cfg(verus_only)]
mod witness_stored_bridge;
#[cfg(verus_only)]
#[cfg(any(not(feature = "split"), feature = "split-fixed-policy"))]
mod witness_finish;
#[cfg(verus_only)]
mod witness_layout;
#[cfg(verus_only)]
#[cfg(any(not(feature = "split"), feature = "split-dynamic"))]
mod witness_invalid_dynamic;
#[cfg(verus_only)]
mod witness_literal_class;
#[cfg(verus_only)]
mod witness_match;
#[cfg(verus_only)]
#[cfg(any(not(feature = "split"), feature = "split-fixed-policy"))]
mod witness_overlap;
#[cfg(verus_only)]
mod witness_policy;
#[cfg(verus_only)]
#[cfg(any(not(feature = "split"), feature = "split-fixed-policy"))]
mod witness_policy_cases;
#[cfg(verus_only)]
#[cfg(any(not(feature = "split"), feature = "split-dynamic"))]
mod witness_repeats;
#[cfg(verus_only)]
#[cfg(any(not(feature = "split"), feature = "split-dynamic"))]
mod witness_single_literal;
#[cfg(verus_only)]
#[cfg(any(not(feature = "split"), feature = "split-fixed-policy"))]
mod witness_special;
#[cfg(verus_only)]
mod witness_window;
#[cfg(verus_only)]
#[cfg(any(not(feature = "split"), feature = "split-fixed-policy"))]
mod witness_window_cases;
#[cfg(verus_only)]
mod witnesses;

#[cfg(verus_only)]
pub use inflate::{fault_eq, ring_repr, status_eq};
pub use inflate::{inflate_zlib_into, inflate_zlib_measure};
pub use segments::{SegmentSlices, Segments};

vstd::prelude::verus! {
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InflateError {
    TruncatedZlib,
    InvalidZlibHeader,
    UnsupportedZlibMethod,
    ZlibPresetDictionary,
    InvalidBlockType,
    StoredLengthMismatch,
    InvalidCodeLengths,
    InvalidSymbol,
    DistanceTooFar,
    AdlerMismatch,
    TrailingZlibData,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Done,
    Full,
    Err(InflateError),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdlerOption {
    Check,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StreamEnd {
    Require,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ZlibTail {
    Reject,
    IgnoreTail,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Window {
    Enforce,
    Ignore,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SingleCode {
    Accept,
}

/// Only Strict/RecommendedBeta field values exist at M1a. Stage-2 compatibility
/// values are deliberately unrepresentable rather than silently approximated.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InflateOptions {
    pub adler: AdlerOption,
    pub stream_end: StreamEnd,
    pub zlib_tail: ZlibTail,
    pub window: Window,
    pub single_code: SingleCode,
}
impl InflateOptions {
    pub const STRICT: Self = Self {
        adler: AdlerOption::Check,
        stream_end: StreamEnd::Require,
        zlib_tail: ZlibTail::Reject,
        window: Window::Enforce,
        single_code: SingleCode::Accept,
    };
    pub const RECOMMENDED_BETA: Self = Self {
        zlib_tail: ZlibTail::IgnoreTail,
        window: Window::Ignore,
        ..Self::STRICT
    };
}
} // verus!

use vstd::prelude::*;

verus! {
/// Both work observations are erased, exact per-call loop counts.
#[derive(Clone, Copy)]
pub struct InflateResult {
    pub status: Status,
    pub produced: u64,
    pub core_steps: Ghost<nat>,
    pub seg_steps: Ghost<nat>,
}

pub(crate) struct Work {
    pub observed: Ghost<nat>,
}
impl Work {
    pub(crate) closed spec fn wf(&self) -> bool { true }

    pub(crate) fn new() -> (w: Self)
        ensures w.wf(), w.observed@ == 0,
    { Self { observed: Ghost(0nat) } }

    /// Charge only the erased counter at the actual production loop boundary.
    pub(crate) fn add(&mut self, n: usize)
        requires old(self).wf(),
        ensures final(self).wf(), final(self).observed@ == old(self).observed@ + n,
    {
        proof { self.observed = Ghost(self.observed@ + n as nat); }
    }
}
} // verus!

impl Default for Work {
    fn default() -> Self {
        Self::new()
    }
}
