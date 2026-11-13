#![no_std]
#![allow(unused_imports)]
// Ported from alloc-delta/proto/alloc_audit/testcases/n10_ffi.rs for the production
// tool: identical items, compiled as a no_std crate (the production audit's
// crate-origin rule, SPEC §9, makes any std instance a finding of its own).
//! Negative control N10: an FFI call. Expected: the tool records an `ffi` leaf
//! for the `extern` item and fails closed; it cannot see what a foreign
//! function allocates, so it does not attempt to reason about it.

extern crate alloc;
use alloc::{boxed::Box, format, string::String, string::ToString, vec, vec::Vec};
extern "C" {
    fn getpid() -> i32;
}

pub fn n10_ffi() -> i32 {
    unsafe { getpid() }
}
