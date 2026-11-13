#![no_std]
#![allow(unused_imports)]
// Ported from alloc-delta/proto/alloc_audit/testcases/c1_mutated.rs for the production
// tool: identical items, compiled as a no_std crate (the production audit's
// crate-origin rule, SPEC §9, makes any std instance a finding of its own).
//! Mutation of the clean control: a second allocation site is added outside
//! the guard. With the c1_clean allowlist supplied, the tool must still fail:
//! the new allocation is reached through `Vec::push` growth, which the
//! allowlist does not name.
//!
//! This is the audit analogue of mutation A1 ("ignore the reservation result")
//! and A11 ("build the entry list with Vec::new + push"): a change that adds an
//! allocation outside the guard must turn a passing audit into a failing one.

extern crate alloc;
use alloc::{boxed::Box, format, string::String, string::ToString, vec, vec::Vec};
pub mod guard {
    use alloc::vec::Vec;
    pub fn try_zeroed(n: usize) -> Vec<u8> {
        let mut v = Vec::new();
        v.reserve_exact(n);
        let mut i = 0usize;
        while i < n {
            v.push(0u8);
            i += 1;
        }
        v
    }
}

pub struct Holder {
    v: Vec<u8>,
}

impl Holder {
    pub fn new(n: usize) -> Holder {
        Holder { v: guard::try_zeroed(n) }
    }

    pub fn len(&self) -> usize {
        self.v.len()
    }
}

impl Drop for Holder {
    fn drop(&mut self) {
        self.v.clear();
    }
}

/// A scratch vector built without a reservation. The mutation: this allocates
/// outside `guard::try_zeroed`.
fn scratch_mutated(m: usize) -> Vec<u8> {
    let mut s = Vec::new();
    let mut i = 0usize;
    while i < m {
        s.push(1u8);
        i += 1;
    }
    s
}

pub fn c1_mutated_root(n: usize, m: usize) -> usize {
    let h = Holder::new(n);
    let s = scratch_mutated(m);
    h.len() + s.len()
}
