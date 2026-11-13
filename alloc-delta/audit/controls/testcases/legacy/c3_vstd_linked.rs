#![no_std]
#![allow(unused_imports)]
// Ported from alloc-delta/proto/alloc_audit/testcases/c3_vstd_linked.rs for the production
// tool: identical items, compiled as a no_std crate (the production audit's
// crate-origin rule, SPEC §9, makes any std instance a finding of its own).
//! A minimal crate that links vstd, used only to exercise the vstd-version
//! provenance check. The root does not call into vstd, so the walk stays small;
//! the point is that `vstd` is a linked dependency, which the tool must see and
//! compare against the allowlist's recorded `vstd_version`.
extern crate alloc;
use alloc::{boxed::Box, format, string::String, string::ToString, vec, vec::Vec};
extern crate vstd;
use vstd::prelude::*;

pub fn c3_vstd_root(n: usize) -> usize {
    n
}
