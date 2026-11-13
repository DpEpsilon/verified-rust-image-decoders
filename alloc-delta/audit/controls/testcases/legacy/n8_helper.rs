#![no_std]
#![allow(unused_imports)]
// Ported from alloc-delta/proto/alloc_audit/testcases/n8_helper.rs for the production
// tool: identical items, compiled as a no_std crate (the production audit's
// crate-origin rule, SPEC §9, makes any std instance a finding of its own).
//! Negative control N8: an allocation in a user helper, reached transitively.
//! Expected: `n8_caller` -> `helper_grow` -> `Vec::push` growth; the tool must
//! follow the user helper (a direct call) and report the allocation.

extern crate alloc;
use alloc::{boxed::Box, format, string::String, string::ToString, vec, vec::Vec};
fn helper_grow(n: usize) -> Vec<u32> {
    let mut v = Vec::new();
    let mut i = 0usize;
    while i < n {
        v.push(i as u32);
        i += 1;
    }
    v
}

pub fn n8_caller(n: usize) -> Vec<u32> {
    helper_grow(n)
}
