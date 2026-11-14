//! Per-core site declarations (alloc-delta SPEC.md §3). This file is a child of
//! `alloc_guard`, so it may read and write the private `ev` field of `AllocLog`
//! and construct `OutOfMemory`. It is where the per-core part of the guard text
//! lives: `AllocSite`, the frozen-`Error`-carrying `TryError`, the sealed log's
//! public predicates that name `Error`, the two reservation shapes ICNS needs
//! beyond `try_zeroed` (`try_copy`, `try_with_room` with `Reserved<T>`), and the
//! per-target `Entry` layout fact.
//!
//! The sibling `mod.rs` is byte-identical across the four cores (decision D6);
//! only this file differs. `Error`, `Decoded` and `Entry` are therefore reached
//! through `super::super::icns` (from `alloc_guard::sites`, `super::super` is the
//! crate root) rather than named in `mod.rs`.
//!
//! ICNS is the first core to need `try_copy` and `try_with_room`: `MaskData` is
//! a copy of an existing slice, and the `EntryList` is a `Reserved<Entry>` filled
//! by `push`. They live here rather than in `mod.rs` for the same reason DIB's
//! `TryError` items do: putting core-specific element types in `mod.rs` would
//! break the four-way byte identity D6 pins.

use vstd::prelude::*;
use alloc::vec::Vec;

use super::{AllocLog, OutOfMemory};
use super::super::icns::{Decoded, Entry, Error};

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
    /// The frozen `Error::name()` spelling for `Format(e)`, `"OutOfMemory"`
    /// otherwise. `open` and field-free, so it is well-formed everywhere.
    pub open spec fn name_spec(&self) -> &'static str {
        match self {
            TryError::Format(e) => e.name_spec(),
            TryError::OutOfMemory(_) => "OutOfMemory",
        }
    }

    /// `'static`, never allocates (SPEC.md §1).
    ///
    /// The `ensures` ties the executable spelling to `name_spec` for every
    /// variant: `Format(e)` inherits `Error::name`'s own verified bridge to
    /// `Error::name_spec`, and the `OutOfMemory` arm is the literal that
    /// `name_spec` gives. Without it, a mutation of one executable spelling
    /// would still verify.
    ///
    /// No `#[verifier::when_used_as_spec]`: the attribute disturbs the
    /// crate-wide encoding and makes an unrelated function report a
    /// resource-limit failure (DIB `alloc-wip-1` run 504 onward; see that
    /// project's `STUCK.md`). `name` is called in exec position only (the test
    /// binaries), so this `ensures` is the spec-position statement.
    pub fn name(&self) -> (s: &'static str)
        ensures s@ == self.name_spec()@,
    {
        match self {
            TryError::Format(e) => e.name(),
            TryError::OutOfMemory(_) => "OutOfMemory",
        }
    }
}

/// The closed per-core site enum, one variant per reservation (SPEC.md §4). ICNS
/// makes seven reservations: one entry-list reservation, three channel buffers,
/// the RGB output, the mask copy and the combine output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocSite {
    /// One `try_with_room(EntryList, min(4096, (|b|-8)/8))`, before the loop.
    EntryList,
    /// `try_zeroed(Channel(k), n)`, `k` = 0, 1, 2, three straight-line calls.
    Channel(u8),
    /// `try_zeroed(RgbOut, 3n)`.
    RgbOut,
    /// `try_copy(MaskData, payload)`.
    MaskData,
    /// `try_zeroed(CombineOut, 4n)`.
    CombineOut,
}

/// The ICNS `Entry` after decision D2 (`type_hex: [u8; 8]`) is declared in
/// `crate::icns`; this module only records its x86_64 layout. Size and alignment
/// are asserted below with a `global layout` declaration; rustc checks it as a
/// static assertion, Verus turns it into an axiom for `size_of`/`align_of`.
///
/// Gate F2, re-checked on the production shape: the D2-shaped `Entry` is 28 bytes
/// with alignment 4 on x86_64. rustc checks the declaration as a static
/// assertion (`E0080` on a wrong size); on i686 the declaration is re-checked by
/// that target's build (see NO_STD.md).
global layout Entry is size == 28, align == 4;

/// The layout facts, exported so modules outside `sites.rs` can use the numbers.
/// A `global layout` axiom is usable only inside the module that declares it, so
/// the entry-list reservation's `size_of::<Entry>() >= 1` and
/// `n * size_of::<Entry>() <= isize::MAX` preconditions are discharged here and
/// the numbers leave through this lemma (SPEC.md §3, gate F2).
pub proof fn lemma_entry_layout()
    ensures
        size_of::<Entry>() == 28,
        align_of::<Entry>() == 4,
{
    assert(size_of::<Entry>() == 28);
    assert(align_of::<Entry>() == 4);
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

/// `fmt_part` for the parse entry point's result type. Same projection, a second
/// name because Verus has no overloading (SPEC.md §2).
pub open spec fn fmt_part_entries(r: Result<Vec<Entry>, TryError>) -> Result<Vec<Entry>, Error> {
    match r {
        Ok(v) => Ok(v),
        Err(TryError::Format(e)) => Err(e),
        Err(TryError::OutOfMemory(_)) => arbitrary(),
    }
}

/// `fmt_part` for the RLE helper's result type (SPEC.md §2, A-ICNS-3).
pub open spec fn fmt_part_channels(
    r: Result<[Vec<u8>; 3], TryError>,
) -> Result<[Vec<u8>; 3], Error> {
    match r {
        Ok(v) => Ok(v),
        Err(TryError::Format(e)) => Err(e),
        Err(TryError::OutOfMemory(_)) => arbitrary(),
    }
}

/// The log-only part of theorem shape `G` clauses 1-2 (THEOREMS-ALLOC-DELTA.md
/// §0) for the decoders' result type: an `OutOfMemory` result has a non-empty log
/// whose last entry is the failed reservation and whose earlier entries are all
/// `Ok`; any other result has an all-`Ok` log.
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

/// `log_tied` for `try_parse`'s result type.
pub open spec fn log_tied_entries(
    r: Result<Vec<Entry>, TryError>,
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

/// `log_tied` for the RLE helper's result type.
pub open spec fn log_tied_channels(
    r: Result<[Vec<u8>; 3], TryError>,
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

/// Upper bound on a `Reserved<T>` room, in elements (SPEC.md §3). ICNS's one
/// `Reserved` site is the entry list, capped at `MAX_ENTRIES = 4096`.
pub const MAX_ROOM: usize = 4_096;

/// Copy `s` into a fresh vector. The `MaskData` reservation (A-ICNS-5).
#[inline(never)]
pub fn try_copy(site: AllocSite, s: &[u8], Tracked(log): Tracked<&mut AllocLog>)
    -> (r: Result<Vec<u8>, OutOfMemory>)
    requires
        1 <= s.len() <= crate::alloc_guard::MAX_BYTES,
    ensures
        final(log).view() == old(log).view().push((site, s.len() as nat, r is Ok)),
        r is Ok ==> r->Ok_0@ == s@,
        r is Err ==> r->Err_0.site_spec() == site && r->Err_0.elements_spec() == s.len(),
{
    let mut v: Vec<u8> = Vec::new();
    match v.try_reserve_exact(s.len()) {
        Ok(()) => {
            let mut i: usize = 0;
            while i < s.len()
                invariant
                    0 <= i <= s.len(),
                    v@ == s@.subrange(0, i as int),
                decreases s.len() - i,
            {
                v.push(s[i]);
                i = i + 1;
            }
            proof { log.ev = log.ev.push((site, s.len() as nat, true)); }
            Ok(v)
        },
        Err(_) => {
            proof { log.ev = log.ev.push((site, s.len() as nat, false)); }
            Err(OutOfMemory { site, elements: s.len() })
        },
    }
}

/// A reserved block with `room` free slots, filled only by `push`, which
/// requires `len < room`. Invariant A-STD-1: `v@.len() <= room <= v.capacity()`.
pub(crate) struct Reserved<T> {
    v: Vec<T>,
    room: Ghost<nat>,
}

impl<T> Reserved<T> {
    pub closed spec fn view(&self) -> Seq<T> {
        self.v@
    }

    pub closed spec fn room(&self) -> nat {
        self.room@
    }

    /// Push one element. The precondition is the `len < room` guard; A-STD-2
    /// (the capacity invariant) is what makes this `push` allocation-free. The
    /// verifier cannot see that; it sees only `final(v)@ == old(v)@.push(x)`.
    #[inline(never)]
    pub(crate) fn push(&mut self, x: T)
        requires
            old(self).view().len() < old(self).room(),
        ensures
            final(self).view() == old(self).view().push(x),
            final(self).room() == old(self).room(),
    {
        self.v.push(x);
    }

    #[inline(never)]
    pub(crate) fn into_vec(self) -> (v: Vec<T>)
        ensures v@ == self.view(),
    {
        self.v
    }
}

/// Reserve room for `n` elements of `T` without filling them. `n == 0` returns
/// `Ok` without touching the allocator (SPEC.md §3: a zero-element request is a
/// successful logical event and reaches no allocator). `size_of::<T>() >= 1` and
/// `n * size_of::<T>() <= isize::MAX` are discharged at the call site from
/// `lemma_entry_layout` (gate F2).
#[inline(never)]
pub(crate) fn try_with_room<T>(site: AllocSite, n: usize, Tracked(log): Tracked<&mut AllocLog>)
    -> (r: Result<Reserved<T>, OutOfMemory>)
    requires
        n <= MAX_ROOM,
        1 <= size_of::<T>(),
        n * size_of::<T>() <= isize::MAX,
    ensures
        final(log).view() == old(log).view().push((site, n as nat, r is Ok)),
        n == 0 ==> r is Ok,
        r is Ok ==> r->Ok_0.view() == Seq::<T>::empty() && r->Ok_0.room() == n as nat,
        r is Err ==> r->Err_0.site_spec() == site && r->Err_0.elements_spec() == n,
{
    let mut v: Vec<T> = Vec::new();
    if n == 0 {
        proof { log.ev = log.ev.push((site, n as nat, true)); }
        return Ok(Reserved { v, room: Ghost(n as nat) });
    }
    match v.try_reserve_exact(n) {
        Ok(()) => {
            proof { log.ev = log.ev.push((site, n as nat, true)); }
            Ok(Reserved { v, room: Ghost(n as nat) })
        },
        Err(_) => {
            proof { log.ev = log.ev.push((site, n as nat, false)); }
            Err(OutOfMemory { site, elements: n })
        },
    }
}

} // verus!
