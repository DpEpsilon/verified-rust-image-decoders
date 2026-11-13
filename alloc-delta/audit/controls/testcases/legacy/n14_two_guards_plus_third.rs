#![no_std]
#![allow(unused_imports)]
// Ported from alloc-delta/proto/alloc_audit/testcases/n14_two_guards_plus_third.rs for the production
// tool: identical items, compiled as a no_std crate (the production audit's
// crate-origin rule, SPEC §9, makes any std instance a finding of its own).
//! Negative control N14: two admitted guard functions that call the same
//! allocation-capable instance, PLUS a third, unapproved caller. The
//! per-caller allowlist entries admit the two guards; the third must still fail.
//!
//! This is the negative companion to C2. `guard_a` and `guard_b` are each
//! admitted for `RawVecInner::<A>::reserve_exact` by their own `[[entry]]`. The
//! helper `unapproved_third` calls the same instance from a third caller, which
//! no entry admits.
//!
//! Expected: the tool fails, naming `unapproved_third`'s path, while the two
//! guards' paths are present. The two-guard entries must not be read as "any
//! caller of the item is allowed".

extern crate alloc;
use alloc::{boxed::Box, format, string::String, string::ToString, vec, vec::Vec};
pub mod guard_a {
    use alloc::vec::Vec;
    pub fn try_zeroed(n: usize) -> Vec<u8> {
        let mut v = Vec::new();
        v.reserve_exact(n);
        v
    }
}

pub mod guard_b {
    use alloc::vec::Vec;
    pub fn try_zeroed(m: usize) -> Vec<u32> {
        let mut v = Vec::new();
        v.reserve_exact(m);
        v
    }
}

/// A third caller of the same shared instance; not admitted by any entry.
fn unapproved_third(k: usize) -> Vec<u64> {
    let mut v = Vec::new();
    v.reserve_exact(k);
    v
}

pub fn n14_two_guards_plus_third(n: usize, k: usize) -> usize {
    let a = guard_a::try_zeroed(n);
    let b = guard_b::try_zeroed(n);
    let c = unapproved_third(k);
    a.len() + b.len() + c.len()
}
