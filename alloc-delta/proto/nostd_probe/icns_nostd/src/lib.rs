#![no_std]

extern crate alloc;

use vstd::prelude::*;

verus! {

fn foo() { assert(1 == 0 + 1); }

} // verus!

pub mod icns;
pub mod model;
pub mod model_vectors;
pub use icns::*;
