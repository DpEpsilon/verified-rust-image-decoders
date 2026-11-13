#![no_std]
#![allow(unused_imports)]
// Ported from alloc-delta/proto/alloc_audit/testcases/n7_dyn.rs for the production
// tool: identical items, compiled as a no_std crate (the production audit's
// crate-origin rule, SPEC §9, makes any std instance a finding of its own).
//! Negative control N7: a trait-object call hides an allocating impl.
//! Expected: `n7_dyn` makes a `dyn` call; the tool records a `dyn-call` finding
//! and follows the vtable methods, so it fails closed rather than missing the
//! allocation in `Allocating as Trait::make`.

extern crate alloc;
use alloc::{boxed::Box, format, string::String, string::ToString, vec, vec::Vec};
pub trait Make {
    fn make(&self) -> Vec<u8>;
}

pub struct Allocating;

impl Make for Allocating {
    fn make(&self) -> Vec<u8> {
        let mut v = Vec::new();
        v.push(9u8);
        v
    }
}

pub fn n7_dyn(m: &dyn Make) -> Vec<u8> {
    m.make()
}
