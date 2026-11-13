#![no_std]
#![allow(unused_imports)]
// Ported from alloc-delta/proto/alloc_audit/testcases/n9_drop.rs for the production
// tool: identical items, compiled as a no_std crate (the production audit's
// crate-origin rule, SPEC §9, makes any std instance a finding of its own).
//! Negative control N9: a `Drop` impl that allocates.
//! Expected: `n9_drop` stores a `Dropper` in a `Box`; dropping it runs the
//! `Drop::drop` glue, whose body allocates (`Vec::new` + `push`). The tool must
//! follow drop glue into the `Drop` impl and report the allocation.

extern crate alloc;
use alloc::{boxed::Box, format, string::String, string::ToString, vec, vec::Vec};
pub struct Dropper;

impl Drop for Dropper {
    fn drop(&mut self) {
        let mut v = Vec::new();
        v.push(1u8);
        core::hint::black_box(&v);
    }
}

pub fn n9_drop() {
    let d = Dropper;
    core::hint::black_box(&d);
    // `d` is dropped here, running `Dropper::drop`.
}
