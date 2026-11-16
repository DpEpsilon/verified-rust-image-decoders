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
mod chain;
mod compose;
/// M4: the compositor's total models `IR`/`MC` (`compose_proof/src/model.rs`).
mod mc;
/// M4: the executable `interlace_row` and the T5 interlace theory
/// (`compose_proof/src/interlace.rs`).
mod interlace;
/// M4: the compositor's proved per-index sink `draw_index` and its helpers
/// (`compose_proof/src/compose.rs`).
mod compose_new;
mod dictionary;
mod emission_compose;
mod frame_model;
mod work_model;
pub mod gif;
pub mod limits;
mod lzw;
mod lzw_model;
pub(crate) mod model;
mod parse;
mod refinement;
pub mod policy;

pub use alloc_guard::sites::TryError;
pub use alloc_guard::{AllocSite, OutOfMemory};
pub use gif::{
    plays, try_decode_still, try_decode_still_with_policy, try_decode_still_logged,
    try_decode_still_with_policy_logged, Error, FrameView, Frames, Info, Plays,
    Still, Version,
};
pub use policy::Profile;
