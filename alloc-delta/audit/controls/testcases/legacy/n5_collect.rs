#![no_std]
#![allow(unused_imports)]
// Ported from alloc-delta/proto/alloc_audit/testcases/n5_collect.rs for the production
// tool: identical items, compiled as a no_std crate (the production audit's
// crate-origin rule, SPEC §9, makes any std instance a finding of its own).
//! Negative control N5: `collect` allocates for the target `Vec`.
//! Expected: allocation reached through `FromIterator`/`RawVec` growth, from
//! `n5_collect`.

extern crate alloc;
use alloc::{boxed::Box, format, string::String, string::ToString, vec, vec::Vec};
pub fn n5_collect(src: &[u16]) -> Vec<u16> {
    src.iter().map(|x| x.wrapping_mul(3)).collect()
}
