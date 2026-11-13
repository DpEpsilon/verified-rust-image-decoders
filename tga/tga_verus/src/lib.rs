#![no_std]

extern crate alloc;

// Unit tests (cfg(test) only) use std, including its macros (format!, println!).
#[cfg(test)]
#[macro_use]
extern crate std;

use vstd::prelude::*;

pub mod model;
pub mod model_vectors;
pub mod policy;
pub mod policy_model;
pub mod tga;
pub mod alloc_guard;

#[cfg(test)]
mod policy_tests;

verus! {

fn skeleton() { assert(1 == 0 + 1); }

} // verus!
