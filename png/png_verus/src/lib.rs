//! PNG/APNG decoder core (PNG-CORE-SPEC 1.0, Stage 1, `../../spec-frozen/SPEC.md`),
//! with the Strict and RecommendedBeta profiles of `../../policy-draft/OPTIONS.md`.
//!
//! Milestone M1b (`../../PLAN.md`): the plain implementation. The decoder is
//! ordinary Rust outside `verus!` blocks; only the data types the sealed
//! allocation guard names (`png::Error`), the cap lemma and the guard itself are
//! inside `verus!`. zlib/DEFLATE is the verified `inflate_verus` crate. Later
//! milestones (M3–M6, P1) add the model and the proofs.
//!
//! `no_std + alloc`, no `unsafe`, no recursion. Every heap allocation goes
//! through `alloc_guard` (`Raw`, `Output`, `Saved`, `IccWindow`, `Icc`).

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

pub mod alloc_guard;
mod crc;
mod image;
pub mod limits;
pub mod png;
pub mod policy;
mod reader;
mod walk;

pub use alloc_guard::sites::TryError;
pub use alloc_guard::{AllocSite, OutOfMemory};
pub use png::{
    try_decode_still, try_decode_still16, try_decode_still16_with_policy, try_decode_still_with_policy,
    try_icc_profile, Error, FrameView, Frames, Info, Meta, Still, Still16, Time,
};
#[cfg(feature = "test-records")]
pub use png::{try_decode_still16_traced, try_decode_still_traced, try_icc_profile_traced};
pub use policy::Profile;
