//! `dib_verus`: the DIB/BMP decoder core of DIB-CORE-SPEC 1.1.
//!
//! M2 moved the whole implementation inside `verus!` ([`dib`]) with contracts and
//! proof, and added the independent reference model [`model`] (`MH` and its
//! lemmas). The M1 plain implementation is preserved as `src/dib.rs.m1`.
//!
//! Version 0.2.0 (milestone M1.1a) adds the fourth public mode
//! `Mode::IcoEntryDibSlack` (R19): identical to `IcoEntryDib` except that bytes
//! after a complete AND mask are ignored instead of `TrailingBytes`. This is a
//! semver-breaking change to the public `Mode` enum.

pub mod dib;
pub mod model;
pub mod model_vectors;

#[cfg(test)]
mod policy_tests;

pub use dib::{
    decode, decode_with_policy, decode_with_steps, BmpMode, BmpProfile, Decoded, Error, Mode,
};

use vstd::prelude::*;

verus! {

fn skeleton() {
    assert(1 == 0 + 1);
}

} // verus!
