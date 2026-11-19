//! Per-core site declarations (alloc-delta SPEC.md §3) for the PNG core. This
//! file is a child of `alloc_guard`, so it may read and write the private `ev`
//! field of `AllocLog` and construct `OutOfMemory`. It holds the per-core part
//! of the guard text: `AllocSite`, the `Error`-carrying `TryError`, the log
//! predicate `log_tied` and `try_empty`, the zero-element request of D8.
//!
//! The sibling `mod.rs` is byte-identical across the cores (decision D6); only
//! this file differs. `Error` is reached through `super::super::png`.
//!
//! PNG's reservations (SPEC P20), byte-counted `try_zeroed` calls of the
//! hash-pinned guard, plus one zero-element request:
//!
//! | Site | Request | When |
//! |---|---|---|
//! | `Raw` | `try_zeroed(Raw, R(W,H))` | `try_decode_still*`, `Frames::try_new`, after the walk |
//! | `Output` | `try_zeroed(Output, 4·W·H)` (`8·W·H` for the RGBA16 still) | same call, after `Raw` |
//! | `Saved` | `try_zeroed(Saved, 4·W·H)` | step S: the first reached frame `k > 0` with `dispose_op = 2`, once |
//! | `IccWindow` | `try_zeroed(IccWindow, WINDOW_MAX)` | `try_icc_profile`, after the walk, when iCCP exists |
//! | `Icc` | `try_zeroed(Icc, m)`, or `try_empty(Icc)` when `m = 0` | `try_icc_profile`, after a measuring pass that ends `Done` |

use vstd::prelude::*;
use alloc::vec::Vec;

use super::{AllocLog, OutOfMemory};
use super::super::png::Error;

verus! {

/// The outcome type of every `try_*` entry point (alloc-delta SPEC.md §1).
/// `Format` carries the closed P18 `Error`; `OutOfMemory` is outside that list
/// and is never a byte-format error (P18, P20).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TryError {
    Format(Error),
    OutOfMemory(OutOfMemory),
}

impl TryError {
    /// The P18 `Error::name()` spelling for `Format(e)`, `"OutOfMemory"`
    /// otherwise. `'static`, never allocates (alloc-delta SPEC.md §1).
    pub fn name(&self) -> &'static str {
        match self {
            TryError::Format(e) => e.name(),
            TryError::OutOfMemory(_) => "OutOfMemory",
        }
    }
}

/// The closed per-core site enum, one variant per reservation (SPEC P20).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocSite {
    /// `R(W,H)` bytes: the filtered image data, inflate's output and window.
    Raw,
    /// `4·W·H` (RGBA8) or `8·W·H` (RGBA16 still) zero bytes: the output or canvas.
    Output,
    /// `4·W·H` bytes: the disposal-PREVIOUS restore buffer.
    Saved,
    /// `WINDOW_MAX` bytes: the measuring ring of `try_icc_profile`.
    IccWindow,
    /// `m` bytes: the ICC profile.
    Icc,
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

/// D8: a request of zero elements for `site`. It is logged like any other
/// request, never reaches the allocator (`Vec::new` does not allocate) and
/// never fails.
#[inline(never)]
pub fn try_empty(site: AllocSite, Tracked(log): Tracked<&mut AllocLog>)
    -> (r: Result<Vec<u8>, OutOfMemory>)
    ensures
        final(log).view() == old(log).view().push((site, 0nat, true)),
        r is Ok && r->Ok_0@.len() == 0,
{
    let v: Vec<u8> = Vec::new();
    proof { log.ev = log.ev.push((site, 0nat, true)); }
    Ok(v)
}

} // verus!
