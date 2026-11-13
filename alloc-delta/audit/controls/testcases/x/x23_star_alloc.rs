//! x23_star_alloc: an allowlist that admits Vec::push from any caller
//! (`allow = "*"`). `*` admits deallocation sinks only, so the allocating
//! paths must still fail with `unauthorized-alloc-path`.
#![no_std]
extern crate alloc;
use alloc::vec::Vec;
pub fn x23_root(n: usize) -> usize {
    let mut v: Vec<u8> = Vec::new();
    v.push(n as u8);
    v.len()
}
