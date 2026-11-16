//! GIF decoder core (GIF-CORE-SPEC draft 0.2, `../spec-draft/SPEC.md`).
//!
//! Milestone M1 (`../PLAN.md`): the plain implementation. The decoder is
//! ordinary Rust outside `verus!` blocks; only the data types that the sealed
//! allocation guard names (`gif::Error`, `lzw::LzwTable`) and the guard itself
//! are inside `verus!`. Later milestones add the model and the proofs.
//!
//! `no_std + alloc`. Every heap allocation goes through `alloc_guard`
//! (`Canvas`, `LzwTable`, `Saved`); see `README.md` for the public API.

#![no_std]

extern crate alloc;

pub mod alloc_guard;
mod compose;
pub mod gif;
pub mod limits;
mod lzw;
pub(crate) mod model;
mod parse;
pub mod policy;

pub use alloc_guard::sites::TryError;
pub use alloc_guard::{AllocSite, OutOfMemory};
pub use gif::{
    plays, try_decode_still, try_decode_still_with_policy, Error, FrameView, Frames, Info, Plays,
    Still, Version,
};
pub use policy::Profile;
