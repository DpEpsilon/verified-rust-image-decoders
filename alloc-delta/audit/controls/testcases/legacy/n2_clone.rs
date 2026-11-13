#![no_std]
#![allow(unused_imports)]
// Ported from alloc-delta/proto/alloc_audit/testcases/n2_clone.rs for the production
// tool: identical items, compiled as a no_std crate (the production audit's
// crate-origin rule, SPEC §9, makes any std instance a finding of its own).
//! Negative control N2: `clone` of a `Vec` (a non-`Copy` type).
//! Expected: allocation reached through `Vec::clone` /
//! `<[T]>::to_vec` -> `RawVec` growth, from `n2_clone`.

extern crate alloc;
use alloc::{boxed::Box, format, string::String, string::ToString, vec, vec::Vec};
pub fn n2_clone(v: &[u8]) -> Vec<u8> {
    let owned: Vec<u8> = v.to_vec();
    owned.clone()
}
