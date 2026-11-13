#![no_std]
#![allow(unused_imports)]
// Ported from alloc-delta/proto/alloc_audit/testcases/n4_box.rs for the production
// tool: identical items, compiled as a no_std crate (the production audit's
// crate-origin rule, SPEC §9, makes any std instance a finding of its own).
//! Negative control N4: `Box::new` allocates via `exchange_malloc`.
//! Expected: `n4_box` reaches `alloc::alloc::exchange_malloc`.

extern crate alloc;
use alloc::{boxed::Box, format, string::String, string::ToString, vec, vec::Vec};
pub fn n4_box(v: u64) -> Box<u64> {
    Box::new(v)
}
