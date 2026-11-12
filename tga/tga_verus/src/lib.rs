use vstd::prelude::*;

pub mod model;
pub mod model_vectors;
pub mod policy;
pub mod policy_model;
pub mod tga;

#[cfg(test)]
mod policy_tests;

verus! {

fn skeleton() { assert(1 == 0 + 1); }

} // verus!
