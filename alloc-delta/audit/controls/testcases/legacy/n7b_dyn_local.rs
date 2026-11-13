#![no_std]
#![allow(unused_imports)]
// Ported from alloc-delta/proto/alloc_audit/testcases/n7b_dyn_local.rs for the production
// tool: identical items, compiled as a no_std crate (the production audit's
// crate-origin rule, SPEC §9, makes any std instance a finding of its own).
//! Negative control N7b: a trait-object call where the unsizing is local.
//! Expected: `n7b_dyn_local` unsizes `Allocating` to `&dyn Make` in its own
//! body, so the tool follows the vtable methods (reaching `Allocating::make`,
//! which allocates) *and* records the `dyn-call`/`vtable-unsize` finding. It
//! fails closed and reports the allocation.

extern crate alloc;
use alloc::{boxed::Box, format, string::String, string::ToString, vec, vec::Vec};
pub trait Make2 {
    fn make(&self) -> Vec<u8>;
}

pub struct Allocating2;

impl Make2 for Allocating2 {
    fn make(&self) -> Vec<u8> {
        let mut v = Vec::new();
        v.push(9u8);
        v
    }
}

pub fn n7b_dyn_local() -> Vec<u8> {
    let a = Allocating2;
    let m: &dyn Make2 = &a;
    m.make()
}
