#![no_std]
#![allow(unused_imports)]
// Ported from alloc-delta/proto/alloc_audit/testcases/n1_push_unreserved.rs for the production
// tool: identical items, compiled as a no_std crate (the production audit's
// crate-origin rule, SPEC §9, makes any std instance a finding of its own).
//! Negative control N1: `Vec::new` + `push` without a prior reservation.
//! Expected: the tool reports an allocator entry reached through `Vec::push`
//! (`alloc::raw_vec::RawVecInner::grow_amortized` / `finish_grow`) from
//! `n1_push_unreserved`, and fails closed (unexplained allocation).

extern crate alloc;
use alloc::{boxed::Box, format, string::String, string::ToString, vec, vec::Vec};
pub fn n1_push_unreserved(n: usize) -> Vec<u8> {
    let mut v = Vec::new();
    let mut i = 0usize;
    while i < n {
        v.push(i as u8);
        i += 1;
    }
    v
}
