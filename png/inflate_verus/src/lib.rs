//! Allocation-free zlib/DEFLATE, PNG-CORE-SPEC 1.0 P9–P10.
//!
//! M1a ordinary Rust, not yet verified. Both entry points retain the produced
//! count on errors and distinguish an exactly full output from a surplus byte.
#![no_std]
#![forbid(unsafe_code)]

mod adler;
mod bits;
mod canonical;
mod classification;
mod huffman;
mod inflate;
pub mod limits;
#[cfg(verus_only)]
pub mod model;
pub mod segments;

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
} // verus!

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

/// Runtime diagnostics at M1a; M2 must replace these with erased Ghost<nat>
/// counters connected to the same concrete loops. No work inequality is proved.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InflateResult {
    pub status: Status,
    pub produced: u64,
    pub core_steps: u128,
    pub seg_steps: u128,
}

use vstd::prelude::*;

verus! {
pub(crate) struct Work {
    pub core: u128,
    pub observed: Ghost<nat>,
}
impl Work {
    pub(crate) closed spec fn wf(&self) -> bool {
        self.core == if self.observed@ <= u128::MAX { self.observed@ } else { u128::MAX as nat }
    }

    pub(crate) fn new() -> (w: Self)
        ensures w.wf(), w.core == 0, w.observed@ == 0,
    { Self { core: 0, observed: Ghost(0nat) } }

    /// The erased observation is exact; the legacy runtime diagnostic clamps.
    pub(crate) fn add(&mut self, n: usize)
        requires old(self).wf(),
        ensures final(self).wf(), old(self).core <= final(self).core <= old(self).core + n,
            final(self).observed@ == old(self).observed@ + n,
    {
        if self.core > u128::MAX - n as u128 { self.core = u128::MAX; }
        else { self.core += n as u128; }
        proof { self.observed = Ghost(self.observed@ + n as nat); }
    }
}
} // verus!

impl Default for Work {
    fn default() -> Self {
        Self::new()
    }
}
