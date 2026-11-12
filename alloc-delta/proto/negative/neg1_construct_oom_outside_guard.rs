//! NEGATIVE CHECK 1 (must FAIL to verify).
//!
//! Included as `guard_proto::negative_checks` by `run_negatives.sh`.
//!
//! Expectation: rustc rejects this file — `OutOfMemory`'s fields are private to
//! `alloc_guard`, so no module outside it can construct one. SPEC.md §1 and §2
//! rest on this: "Code outside `alloc_guard` cannot construct an `OutOfMemory`".
//! If this file compiles, A5/A17/A22's build-level kills are void.
//!
//! Failure mode expected: E0603 / E0616 "field ... is private".

use vstd::prelude::*;

use crate::alloc_guard::{AllocSite, OutOfMemory, TryError};

verus! {

/// A format-error path that tries to fabricate `OutOfMemory` (mutation A5).
pub fn pretend_oom(b: &[u8]) -> (r: Result<Vec<u8>, TryError>)
    ensures b.len() == 0 ==> matches!(r, Err(TryError::OutOfMemory(_))),
{
    if b.len() == 0 {
        Err(TryError::OutOfMemory(OutOfMemory {
            site: AllocSite::Output,
            elements: 4,
        }))
    } else {
        Ok(Vec::new())
    }
}

} // verus!
