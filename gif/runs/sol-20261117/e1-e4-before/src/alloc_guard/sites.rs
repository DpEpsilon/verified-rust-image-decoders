//! Per-core site declarations (alloc-delta SPEC.md §3) for the GIF core. This
//! file is a child of `alloc_guard`, so it may read and write the private `ev`
//! field of `AllocLog` and construct `OutOfMemory`. It holds the per-core part
//! of the guard text: `AllocSite`, the `Error`-carrying `TryError`, and the log
//! predicate `log_tied`.
//!
//! The sibling `mod.rs` is byte-identical across the cores (decision D6); only
//! this file differs. `Error` is reached through `super::super::gif` (from
//! `alloc_guard::sites`, `super::super` is the crate root) rather than named in
//! `mod.rs`.
//!
//! GIF's reservations (SPEC.md G18), all three byte-counted `try_zeroed` calls
//! of the hash-pinned guard:
//!
//! | Site | Request | When |
//! |---|---|---|
//! | `Canvas` | `try_zeroed(Canvas, 4·W·H)` | `try_decode_still*` and `Frames::try_new*`, after G2–G4 |
//! | `LzwTable` | `try_zeroed(LzwTable, 20_480)` | same call, after `Canvas` |
//! | `Saved` | `try_zeroed(Saved, 4·W·H)` | the first frame with disposal 3, once per `Frames` |

use vstd::prelude::*;

use super::OutOfMemory;
use super::super::gif::Error;

verus! {

/// The outcome type of every `try_*` entry point (alloc-delta SPEC.md §1).
/// `Format` carries the closed G16 `Error`; `OutOfMemory` is outside that list
/// and is never a byte-format error (G16, G18).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TryError {
    Format(Error),
    OutOfMemory(OutOfMemory),
}

impl TryError {
    /// The G16 `Error::name()` spelling for `Format(e)`, `"OutOfMemory"`
    /// otherwise. `open` and field-free, so it is well-formed everywhere.
    pub open spec fn name_spec(&self) -> &'static str {
        match self {
            TryError::Format(e) => e.name_spec(),
            TryError::OutOfMemory(_) => "OutOfMemory",
        }
    }

    /// `'static`, never allocates (alloc-delta SPEC.md §1).
    pub fn name(&self) -> (s: &'static str)
        ensures s@ == self.name_spec()@,
    {
        match self {
            TryError::Format(e) => e.name(),
            TryError::OutOfMemory(_) => "OutOfMemory",
        }
    }
}

/// The closed per-core site enum, one variant per reservation (G18).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocSite {
    /// `try_zeroed(Canvas, 4·W·H)`: the RGBA8 canvas (also `Still::rgba`).
    Canvas,
    /// `try_zeroed(LzwTable, 20_480)`: G18's one LZW table region (prefix
    /// codes as LE16 pairs, suffix bytes, string lengths as LE16 pairs).
    LzwTable,
    /// `try_zeroed(Saved, 4·W·H)`: the disposal-3 restore buffer.
    Saved,
}

/// Theorem shape `G` clauses 1–2 (THEOREMS-ALLOC-DELTA.md §0), log part, for any
/// `try_*` result: an `OutOfMemory` result has a non-empty log whose last entry
/// is the failed reservation and whose earlier entries are all `Ok`; any other
/// result has an all-`Ok` log.
pub open spec fn log_tied<T>(
    r: Result<T, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    match r {
        Err(TryError::OutOfMemory(o)) =>
            l.len() >= 1
            && (forall|j: int| 0 <= j < l.len() - 1 ==> l[j].2)
            && !l[l.len() - 1].2
            && l[l.len() - 1].0 == o.site_spec()
            && l[l.len() - 1].1 == o.elements_spec() as nat,
        _ => crate::alloc_guard::all_ok(l),
    }
}

} // verus!
