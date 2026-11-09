use vstd::prelude::*;

pub mod model;
pub mod model_vectors;
pub mod tga;

verus! {

fn skeleton() { assert(1 == 0 + 1); }

} // verus!
