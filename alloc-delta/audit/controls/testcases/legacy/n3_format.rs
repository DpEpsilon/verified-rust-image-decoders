#![no_std]
#![allow(unused_imports)]
// Ported from alloc-delta/proto/alloc_audit/testcases/n3_format.rs for the production
// tool: identical items, compiled as a no_std crate (the production audit's
// crate-origin rule, SPEC §9, makes any std instance a finding of its own).
//! Negative control N3: `format!` allocates a `String`.
//! Expected: allocation reached through `alloc::fmt::format` /
//! `String` growth, from `n3_format`.

extern crate alloc;
use alloc::{boxed::Box, format, string::String, string::ToString, vec, vec::Vec};
pub fn n3_format(x: u32) -> String {
    format!("value={x}")
}
