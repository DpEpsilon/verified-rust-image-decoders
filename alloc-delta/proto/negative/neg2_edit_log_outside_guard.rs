//! NEGATIVE CHECK 2 (must FAIL to verify).
//!
//! Expectation: rustc rejects this file — `AllocLog`'s `ev` field is private to
//! `alloc_guard`, so no module outside it can append, replace or truncate an
//! entry. This is what makes the log "sealed" (SPEC.md §2) and what A18/A19
//! would have to defeat.
//!
//! Failure mode expected: exactly one rustc error, E0616 "field `ev` of struct
//! `AllocLog` is private". (An earlier second route through `borrow_mut` was
//! removed: it failed with an unrelated E0599, not with field privacy.)

use vstd::prelude::*;

use crate::alloc_guard::{AllocLog, AllocSite};

verus! {

/// Hide a reservation by rewriting the log's ghost field (mutation A18/A19).
pub fn scrub_log(site: AllocSite, Tracked(log): Tracked<&mut AllocLog>)
    ensures final(log).view().len() == 0,
{
    log.ev = Seq::empty();
}

} // verus!
