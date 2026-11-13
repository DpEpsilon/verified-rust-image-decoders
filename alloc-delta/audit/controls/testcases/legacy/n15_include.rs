#![no_std]
#![allow(unused_imports)]
// Ported from alloc-delta/proto/alloc_audit/testcases/n15_include.rs for the production
// tool: identical items, compiled as a no_std crate (the production audit's
// crate-origin rule, SPEC §9, makes any std instance a finding of its own).
//! Negative control N15: the crate embeds a non-source file with `include_str!`.
//! The source hash covers the `.rs` text only; the embedded file's bytes are a
//! build input the hash does not cover, so the tool must fail closed unless the
//! allowlist declares the include.
//!
//! Expected (no declaration): exit 1 with an `undeclared-include` finding.

extern crate alloc;
use alloc::{boxed::Box, format, string::String, string::ToString, vec, vec::Vec};
pub const EMBEDDED: &str = include_str!("n15_payload.txt");

pub fn n15_include() -> usize {
    EMBEDDED.len()
}
