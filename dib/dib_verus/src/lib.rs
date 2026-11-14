//! `dib_verus`: the DIB/BMP decoder core of DIB-CORE-SPEC 1.1.
//!
//! M2 moved the whole implementation inside `verus!` (`dib`) with contracts and
//! proof, and added the independent reference model (`model`, `MH` and its
//! lemmas). The M1 plain implementation is preserved as `src/dib.rs.m1`.
//!
//! Version 0.2.0 (milestone M1.1a) adds the fourth public mode
//! `Mode::IcoEntryDibSlack` (R19): identical to `IcoEntryDib` except that bytes
//! after a complete AND mask are ignored instead of `TrailingBytes`. This is a
//! semver-breaking change to the public `Mode` enum.
//!
//! The intended public API is the four `try_*` entry points, the result type
//! `Decoded`, the error type `TryError` (with `Error` and `OutOfMemory`), the
//! `Mode` enum, the policy surface (`BmpMode`, `BmpProfile`, `Policy` and its
//! option enums, and the `*_policy` constructors), the `MAX_INPUT`/`MAX_OUTPUT`
//! caps and the R15 `Error::name` spelling. Everything else is implementation
//! detail: `model_vectors` is crate-private, `model` is `#[doc(hidden)]` (ICO's
//! spec code names four of its items, below), and the proof items DIB's own
//! contracts use are `pub` but `#[doc(hidden)]` — they cannot be hidden at the
//! item level, because a `pub` `try_*` contract names them and Verus refuses a
//! contract that refers to a non-`pub` item.

#![no_std]

extern crate alloc;
#[cfg(test)]
#[macro_use]
extern crate std;

pub mod alloc_guard;
pub mod dib;
// `model` stays reachable from another crate: `ico_verus`'s allocation proof
// names `dib_verus::model::{mp, max_dimension, max_output, lemma_mp_ok_shape}` in
// spec position (SPEC 3.1), and Verus requires those items to be `pub`. It is
// hidden from the documented surface instead.
#[doc(hidden)]
pub mod model;
pub(crate) mod model_vectors;

#[cfg(test)]
mod policy_tests;

pub use dib::{
    try_decode, try_decode_with_policy, try_decode_with_steps, try_decode_with_steps_logged,
    BmpMode, BmpProfile, Decoded, Error, Mode, TryError,
};

/// The input and output caps (SPEC R1, R2), re-exported so a caller can
/// size a buffer or reject an oversized file before calling `try_decode`.
pub use dib::{MAX_INPUT, MAX_OUTPUT};

use vstd::prelude::*;

verus! {

fn skeleton() {
    assert(1 == 0 + 1);
}

} // verus!
