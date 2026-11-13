//! Per-core site declarations (alloc-delta SPEC.md §3). This file is a child of
//! `alloc_guard`, so it may read and write the private `ev` field of `AllocLog`
//! and construct `OutOfMemory`. It is where the per-core part of the guard text
//! lives: `AllocSite`, the frozen-`Error`-carrying `TryError`, and the sealed
//! log's public predicates that name `Error`.
//!
//! The sibling `mod.rs` is byte-identical across the four cores; only this file
//! differs, so `Error` and `Decoded` are reached through `super::super::tga`
//! (from `alloc_guard::sites`, `super::super` is the crate root) rather than
//! named in `mod.rs`.

use vstd::prelude::*;

use super::{AllocLog, OutOfMemory};
use super::super::tga::{Decoded, Error};

verus! {

/// The outcome type of every `try_*` entry point (SPEC.md §1). `Format` carries
/// the frozen `Error`; `OutOfMemory` is outside every frozen error list and is
/// never a byte-format error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TryError {
    Format(Error),
    OutOfMemory(OutOfMemory),
}

/// `TryError::name()`: the frozen `Error::name()` spelling for `Format(e)`,
/// `"OutOfMemory"` otherwise. It is `open`, reads no private field and is
/// well-formed everywhere; the public exec accessor's `ensures` names it.
impl TryError {
    pub open spec fn name_spec(&self) -> &'static str {
        match self {
            TryError::Format(e) => e.name_spec(),
            TryError::OutOfMemory(_) => "OutOfMemory",
        }
    }

    /// `'static`, never allocates (SPEC.md §1).
    ///
    /// No `#[verifier::when_used_as_spec]`, matching DIB's shipped guard: tying
    /// the exec accessor to `name_spec` disturbs the crate-wide encoding and
    /// makes an unrelated function report a resource-limit failure (`dib`
    /// `alloc-wip-1` run 504 onward; see that project's `STUCK.md`). Both `name`
    /// functions are called in exec position only (the test binaries), so the
    /// spelling tables `name_spec` stand alone as the spec-position statement.
    pub fn name(&self) -> (s: &'static str)
    {
        match self {
            TryError::Format(e) => e.name(),
            TryError::OutOfMemory(_) => "OutOfMemory",
        }
    }
}

/// The closed per-core site enum, one variant per reservation (SPEC.md §4). TGA
/// makes exactly one reservation, the RGBA output buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocSite {
    /// The `4N`-byte zero-filled RGBA output.
    Output,
}

/// `fmt_part(r)`: the frozen-typed result of a `try_*` result that is not
/// `OutOfMemory` (SPEC.md §2, THEOREMS-ALLOC-DELTA.md §0): `Ok(v) ↦ Ok(v)`,
/// `Err(Format(e)) ↦ Err(e)`. The third arm is unreachable in every clause that
/// uses this function, which is why it may return an unspecified value.
pub open spec fn fmt_part(r: Result<Decoded, TryError>) -> Result<Decoded, Error> {
    match r {
        Ok(v) => Ok(v),
        Err(TryError::Format(e)) => Err(e),
        Err(TryError::OutOfMemory(_)) => arbitrary(),
    }
}

/// The log-only part of theorem shape `G` clauses 1-2 (THEOREMS-ALLOC-DELTA.md
/// §0), exported alongside `fmt_part`: an `OutOfMemory` result has a non-empty
/// log whose last entry is the failed reservation and whose earlier entries are
/// all `Ok`; any other result has an all-`Ok` log.
pub open spec fn log_tied(
    r: Result<Decoded, TryError>,
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
