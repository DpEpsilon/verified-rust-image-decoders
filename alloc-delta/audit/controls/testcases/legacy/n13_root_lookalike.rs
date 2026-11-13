#![no_std]
#![allow(unused_imports)]
// Ported from alloc-delta/proto/alloc_audit/testcases/n13_root_lookalike.rs for the production
// tool: identical items, compiled as a no_std crate (the production audit's
// crate-origin rule, SPEC §9, makes any std instance a finding of its own).
//! Negative control N13: a *root-level* function whose name equals the admitted
//! function's last segment must not be admitted.
//!
//! The allowlist admits `guard::try_zeroed`. This crate also defines a
//! root-level `try_zeroed` (def-path just `try_zeroed`). The old matcher
//! accepted a caller when `allow.ends_with(c)` — and
//! `"guard::try_zeroed".ends_with("try_zeroed")` is true — so the root-level
//! function was admitted. The whole-segment matcher must refuse it (it is not
//! `guard::try_zeroed`, and `try_zeroed` is not a generic-argument suffix of it).
//!
//! Expected: the tool fails, naming the root-level function's path as
//! `unauthorized-alloc-path`, while the real guard's path stays justified.

extern crate alloc;
use alloc::{boxed::Box, format, string::String, string::ToString, vec, vec::Vec};
pub mod guard {
    use alloc::vec::Vec;
    /// The admitted function.
    pub fn try_zeroed(n: usize) -> Vec<u8> {
        let mut v = Vec::new();
        v.reserve_exact(n);
        v
    }
}

/// A root-level function whose def-path is just `try_zeroed`, not
/// `guard::try_zeroed`. It reaches the same shared `reserve_exact` instance.
pub fn try_zeroed(m: usize) -> Vec<u32> {
    let mut v = Vec::new();
    v.reserve_exact(m);
    v
}

/// The public root: reaches the shared instance through both the admitted guard
/// and the unadmitted root-level function.
pub fn n13_root_lookalike(n: usize, m: usize) -> usize {
    let a = guard::try_zeroed(n);
    let b = try_zeroed(m);
    a.len() + b.len()
}
