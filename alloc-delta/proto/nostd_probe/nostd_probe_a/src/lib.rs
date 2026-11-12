//! Negative control: each marked line must be rejected in a `#![no_std]` crate.
#![no_std]

extern crate alloc;

use vstd::prelude::*;
use alloc::vec::Vec;

// NEG-1: std path in an item position.
pub struct A { pub v: std::vec::Vec<u8> }

// NEG-2: std::default::Default impl (the real crates' pattern).
pub enum P { X }
impl std::default::Default for P { fn default() -> Self { P::X } }

// NEG-3: unimported vec! (not in the no_std prelude).
pub fn z(n: usize) -> Vec<u8> { vec![0u8; n] }

// NEG-4: String without importing alloc::string::String.
pub fn s() -> String { String::new() }

verus! {
fn skeleton() { assert(1 == 0 + 1); }
}
