//! PNG/APNG decoder core (PNG-CORE-SPEC 1.0, Stage 1, `../../spec-frozen/SPEC.md`),
//! with the Strict and RecommendedBeta profiles of `../../policy-draft/OPTIONS.md`.
//!
//! M3 has begun (`../../PLAN.md`): the actual structural walk, local value
//! and APNG checks, raw Info storage and policy projection are verified
//! against complete independent MW/MW_p/MWF_p transition/trace models.
//! The segmented adapter, IDAT/fdAT transport and validated boundary selectors
//! feed checked source-state transitions and fresh-event walk gates. The Strict
//! ICC entry composes bounded measurement, exact-sized second-pass bytes and its
//! returned sealed log. The image continuation and public still entries have
//! checked weak safety, walk-error and Info contracts. Pixel equality, public
//! frame continuation and complete logged receiver equations remain open. A green run
//! does not sign off M3. zlib/DEFLATE is the verified `inflate_verus` dependency.
//!
//! `no_std + alloc`, no `unsafe`, no recursion. Every heap allocation goes
//! through `alloc_guard` (`Raw`, `Output`, `Saved`, `IccWindow`, `Icc`).

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

pub mod alloc_guard;
mod apng;
#[cfg(verus_only)]
pub mod apng_model;
mod crc;
mod callers;
#[cfg(verus_only)]
pub mod allocation_model;
mod frame_state;
mod critical;
#[cfg(verus_only)]
pub mod critical_model;
#[cfg(verus_only)]
pub mod document_model;
#[cfg(verus_only)]
pub mod transport_model;
#[cfg(verus_only)]
pub mod frame_transport_model;
mod framing;
#[cfg(verus_only)]
pub mod filter_model;
#[cfg(verus_only)]
pub mod raster_model;
mod header;
mod image;
#[cfg(verus_only)]
pub mod icc_model;
mod info;
mod keywords;
#[cfg(verus_only)]
pub mod layout_model;
mod kind;
mod metadata;
#[cfg(verus_only)]
pub mod metadata_model;
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
mod walk_policy;
mod walk_state;
mod walk_own;
mod walk_step;
#[cfg(verus_only)]
mod m4_fixed_witness_model;
#[cfg(verus_only)]
mod witness_m4_complete;
#[cfg(verus_only)]
mod witness_m3;
#[cfg(verus_only)]
mod witness_m3_frozen;
#[cfg(verus_only)]
mod witness_m3_walk_frozen;
#[cfg(verus_only)]
mod witness_m3_projection;
#[cfg(verus_only)]
mod witness_metadata;
#[cfg(verus_only)]
mod witness_critical;
#[cfg(verus_only)]
mod witness_document;
#[cfg(verus_only)]
mod witness_walk;
#[cfg(verus_only)]
mod witness_frame;

pub use alloc_guard::sites::TryError;
pub use alloc_guard::{AllocSite, OutOfMemory};
pub use png::{
    try_decode_still, try_decode_still16, try_decode_still16_with_policy, try_decode_still_with_policy,
    try_icc_profile, try_icc_profile_logged, Error, FrameView, Frames, Info, Meta, Still, Still16, Time,
};
#[cfg(feature = "test-records")]
pub use png::{try_decode_still16_traced, try_decode_still_traced, try_icc_profile_traced};
pub use policy::Profile;
