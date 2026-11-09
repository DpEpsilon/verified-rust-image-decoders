//! `dib_verus`: the DIB/BMP decoder core of DIB-CORE-SPEC 1.0.
//!
//! M2 moves the whole implementation inside `verus!` ([`dib`]) with contracts and
//! proof, and adds the independent reference model [`model`] (`MH` and its
//! lemmas). The M1 plain implementation is preserved as `src/dib.rs.m1`.

pub mod dib;
pub mod model;
pub mod model_vectors;

pub use dib::{decode, Decoded, Error, Mode};

use vstd::prelude::*;

verus! {

fn skeleton() {
    assert(1 == 0 + 1);
}

} // verus!
