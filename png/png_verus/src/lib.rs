//! PNG/APNG decoder core (PNG-CORE-SPEC 1.0, Stage 1, `../../spec-frozen/SPEC.md`),
//! with the Strict and RecommendedBeta profiles of `../../policy-draft/OPTIONS.md`.
//!
//! M3 has begun (`../../PLAN.md`): the CRC, wire framing, header prefix,
//! keyword helpers and segmented re-walk adapter are inside `verus!` and tied
//! to independent sequence models. The policy ordering walk, image stages and
//! public entry-point bodies remain ordinary Rust; a green run does not claim
//! their verification. zlib/DEFLATE is the verified `inflate_verus` dependency.
//!
//! `no_std + alloc`, no `unsafe`, no recursion. Every heap allocation goes
//! through `alloc_guard` (`Raw`, `Output`, `Saved`, `IccWindow`, `Icc`).

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

pub mod alloc_guard;
mod crc;
mod framing;
mod header;
mod image;
mod keywords;
mod kind;
mod ordering;
#[cfg(verus_only)]
pub mod walk_model;
#[cfg(verus_only)]
pub mod model;
pub mod limits;
pub mod png;
pub mod policy;
mod reader;
mod walk;
#[cfg(verus_only)]
mod witness_m3;

pub use alloc_guard::sites::TryError;
pub use alloc_guard::{AllocSite, OutOfMemory};
pub use png::{
    try_decode_still, try_decode_still16, try_decode_still16_with_policy, try_decode_still_with_policy,
    try_icc_profile, Error, FrameView, Frames, Info, Meta, Still, Still16, Time,
};
#[cfg(feature = "test-records")]
pub use png::{try_decode_still16_traced, try_decode_still_traced, try_icc_profile_traced};
pub use policy::Profile;
