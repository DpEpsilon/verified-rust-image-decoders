//! ICO's per-core site declarations (alloc-delta SPEC.md §3, §3.1).
//!
//! This file is a child of `alloc_guard`, so it may read and write the private
//! `ev` field of `AllocLog` and construct `OutOfMemory`. It carries the per-core
//! part of the guard text: `AllocSite`, the frozen-`Error`-carrying `TryError`,
//! the sealed log's public predicates, ICO's one own reservation
//! (`try_with_room` for the entry list), and `import_dib` (§3.1).
//!
//! The sibling `mod.rs` is byte-identical across the four cores; only this file
//! differs, so `Error`, `Decoded` and `Entry` are reached through
//! `super::super::ico` (from `alloc_guard::sites`, `super::super` is the crate
//! root) rather than named in `mod.rs`.
//!
//! `try_with_room` and `Reserved<T>` live here rather than in `mod.rs` because
//! the shipped `mod.rs` (hash-pinned, byte-identical across DIB and TGA) carries
//! only `try_zeroed`: DIB and TGA each make one byte-oriented reservation, while
//! ICO's entry list is a `Vec<Entry>`. Keeping the element-list reservation in
//! the per-core child module preserves the D6 byte-identity of `mod.rs` and
//! keeps every allocation inside `alloc_guard` (SPEC.md §3, §6).

use alloc::vec::Vec;
use dib_verus::Decoded;
use vstd::prelude::*;

use super::super::ico::{Entry, Error};
use super::{AllocLog, OutOfMemory};

verus! {

/// Upper bound on a `Reserved<T>` room, in elements (SPEC.md §3). ICO's entry
/// list is at most 1,024 records; the guard's cap is the prototype's 4,096.
pub const MAX_ROOM: usize = 4_096;

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
    /// The `ensures` ties the executable spelling to `name_spec` for every
    /// variant except `Format(Error::Dib(_))`. DIB's `Error::name` exports no
    /// contract, so the DIB spelling is DIB's own R15 obligation and cannot be
    /// discharged here; the `Dib` arm is therefore left unconstrained rather
    /// than stated and unproved.
    ///
    /// No `#[verifier::when_used_as_spec]`: the attribute disturbs the
    /// crate-wide encoding and makes an unrelated function report a
    /// resource-limit failure (DIB `alloc-wip-1` run 504 onward; see that
    /// project's `STUCK.md`). `name` is called in exec position only (the test
    /// binaries), so this `ensures` is the spec-position statement.
    pub fn name(&self) -> (s: &'static str)
        ensures
            match self {
                TryError::Format(Error::Dib(_)) => true,
                _ => s@ == self.name_spec()@,
            },
    {
        match self {
            TryError::Format(e) => e.name(),
            TryError::OutOfMemory(_) => "OutOfMemory",
        }
    }
}

/// The closed per-core site enum, one variant per reservation (SPEC.md §4).
/// ICO makes one own reservation (the entry list) and imports DIB's one
/// reservation (the RGBA output) through `import_dib`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocSite {
    /// The entry list: at most 1,024 `Entry` records.
    EntryList,
    /// A reservation made by the DIB core on the selected slice.
    Dib(dib_verus::alloc_guard::AllocSite),
}

/// `fmt_part(r)` for a `parse` result: the frozen-typed result of a `try_*`
/// result that is not `OutOfMemory` (SPEC.md §2, THEOREMS-ALLOC-DELTA.md §0):
/// `Ok(v) ↦ Ok(v)`, `Err(Format(e)) ↦ Err(e)`. The third arm is unreachable in
/// every clause that uses this function, which is why it may return an
/// unspecified value.
pub open spec fn fmt_part(r: Result<Vec<Entry>, TryError>) -> Result<Vec<Entry>, Error> {
    match r {
        Ok(v) => Ok(v),
        Err(TryError::Format(e)) => Err(e),
        Err(TryError::OutOfMemory(_)) => arbitrary(),
    }
}

/// `fmt_part` for a `decode_entry` result (SPEC.md §2): the same mapping on the
/// `Decoded` payload.
pub open spec fn fmt_part_decoded(r: Result<Decoded, TryError>) -> Result<Decoded, Error> {
    match r {
        Ok(v) => Ok(v),
        Err(TryError::Format(e)) => Err(e),
        Err(TryError::OutOfMemory(_)) => arbitrary(),
    }
}

/// The log-only part of theorem shape `G` clauses 1-2 (THEOREMS-ALLOC-DELTA.md
/// §0) for a `parse` result: an `OutOfMemory` result has a non-empty log whose
/// last entry is the failed reservation and whose earlier entries are all `Ok`;
/// any other result has an all-`Ok` log.
pub open spec fn log_tied(r: Result<Vec<Entry>, TryError>, l: Seq<(AllocSite, nat, bool)>) -> bool {
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

/// `log_tied` for a `decode_entry` result.
pub open spec fn log_tied_decoded(
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

// ---------------------------------------------------------------------------
// The entry-list reservation (SPEC.md §4 ICO `EntryList`)
// ---------------------------------------------------------------------------

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
/// `Ok` without touching the allocator. `size_of::<T>() >= 1` and
/// `n * size_of::<T>() <= isize::MAX` are discharged at each call site from the
/// `global layout` declaration below, through `lemma_entry_layout` (SPEC.md §3,
/// gate F2).
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

// Gate F2: the per-target `Entry` layout, checked by rustc as a static
// assertion. `Entry` has no pointer-sized field, so the layout is the same on
// x86_64, i686, thumbv7m and x86_64-unknown-none (measured: 44 bytes, align 4).
global layout Entry is size == 44, align == 4;

/// Gate F2: the layout facts, exported so modules and crates outside `sites.rs`
/// can use the numbers. A `global layout` declaration's axiom is only usable
/// inside the module that declares it, so every other user (the entry-list
/// reservation's `size_of::<T>() >= 1` and `n * size_of::<T>() <= isize::MAX`
/// preconditions among them) goes through this lemma. rustc checks the
/// declaration itself as a static assertion, so the numbers cannot drift.
pub proof fn lemma_entry_layout()
    ensures
        size_of::<Entry>() == 44,
        align_of::<Entry>() == 4,
{
    assert(size_of::<Entry>() == 44);
    assert(align_of::<Entry>() == 4);
}

// ---------------------------------------------------------------------------
// The request models (SPEC.md §4)
// ---------------------------------------------------------------------------

/// `Req_P(b)`: the entry-list request `[(EntryList, count)]` iff `H(b)` is `Ok`.
pub open spec fn req_p(b: Seq<u8>) -> Seq<(AllocSite, nat)> {
    match crate::model::h(b) {
        Ok(hr) => seq![(AllocSite::EntryList, hr.count as nat)],
        Err(_) => Seq::empty(),
    }
}

/// `map_site` (SPEC.md §3.1): rename DIB's sites into ICO's.
pub open spec fn map_site(s: dib_verus::alloc_guard::AllocSite) -> AllocSite {
    AllocSite::Dib(s)
}

/// `map_dib` on full log entries.
pub open spec fn map_dib(
    s: Seq<(dib_verus::alloc_guard::AllocSite, nat, bool)>,
) -> Seq<(AllocSite, nat, bool)> {
    Seq::new(s.len(), |j: int| (map_site(s[j].0), s[j].1, s[j].2))
}

/// `map_dib` on the request list, with the `ok` flag already dropped.
pub open spec fn map_dib_req(
    s: Seq<(dib_verus::alloc_guard::AllocSite, nat)>,
) -> Seq<(AllocSite, nat)> {
    Seq::new(s.len(), |j: int| (map_site(s[j].0), s[j].1))
}

/// `Req_D(b, E, i)`: `[]` if `MD` returns before the DIB call, and otherwise
/// DIB's `Req_DIB(b[a..z], IcoEntryDibSlack)` with `Output ↦ Dib(Output)`.
pub open spec fn req_d(b: Seq<u8>, es: Seq<Entry>, index: u32) -> Seq<(AllocSite, nat)> {
    match crate::model::md_ico(b, es, index) {
        crate::model::IcoDecision::DibSlice(start, end) => map_dib_req(
            dib_verus::dib::req_dib(b.subrange(start, end), dib_verus::Mode::IcoEntryDibSlack),
        ),
        _ => Seq::empty(),
    }
}

// ---------------------------------------------------------------------------
// Theorem shape `G` and `Prog` for the two ICO entry points
// ---------------------------------------------------------------------------

/// Clause 1 of `G` for `parse` (THEOREMS-ALLOC-DELTA.md §0): a non-`OutOfMemory`
/// result satisfies `parse_eq` on `fmt_part(r)`, has exactly the requests of
/// `Req_P`, and every log entry is `Ok`.
pub open spec fn g_parse_clause1(
    b: Seq<u8>,
    r: Result<Vec<Entry>, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    match r {
        Err(TryError::OutOfMemory(_)) => true,
        _ => crate::model::parse_eq(b, fmt_part(r)) && crate::alloc_guard::req_of_log(l) == req_p(b) && crate::alloc_guard::all_ok(l),
    }
}

/// Clause 2 of `G` for `parse`: an `OutOfMemory` result has a non-empty log
/// whose requests are a prefix of `Req_P`, every entry but the last `Ok`, and
/// the last entry not `Ok` and equal to the returned `(site, elements)`.
pub open spec fn g_parse_clause2(
    b: Seq<u8>,
    r: Result<Vec<Entry>, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    match r {
        Err(TryError::OutOfMemory(o)) =>
            l.len() >= 1
            && crate::alloc_guard::req_of_log(l).is_prefix_of(req_p(b))
            && (forall|j: int| 0 <= j < l.len() - 1 ==> l[j].2)
            && !l[l.len() - 1].2
            && l[l.len() - 1].0 == o.site_spec()
            && l[l.len() - 1].1 == o.elements_spec() as nat,
        _ => true,
    }
}

/// Clause 3 of `G` for `parse`: every request is within the site's maximum,
/// proved from the checked header before the guard call.
pub open spec fn g_parse_clause3(b: Seq<u8>) -> bool {
    forall|j: int| 0 <= j < req_p(b).len() ==> req_p(b)[j].1 <= MAX_ROOM
}

/// Theorem shape `G` for `try_parse` (THEOREMS-ALLOC-DELTA.md A-ICO-1).
pub open spec fn g_parse(
    b: Seq<u8>,
    r: Result<Vec<Entry>, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    g_parse_clause1(b, r, l) && g_parse_clause2(b, r, l) && g_parse_clause3(b)
}

/// `Prog` for `parse` (THEOREMS-ALLOC-DELTA.md §0): if every log entry is `Ok`,
/// the result is not `OutOfMemory`, `parse_eq` holds, and `req(L) == Req_P`.
pub open spec fn prog_parse(
    b: Seq<u8>,
    r: Result<Vec<Entry>, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    crate::alloc_guard::all_ok(l) ==> !matches!(r, Err(TryError::OutOfMemory(_))) && crate::model::parse_eq(b, fmt_part(r))
        && crate::alloc_guard::req_of_log(l) == req_p(b)
}

/// Clause 1 of `G` for `decode_entry`.
pub open spec fn g_decode_clause1(
    b: Seq<u8>,
    es: Seq<Entry>,
    index: u32,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    match r {
        Err(TryError::OutOfMemory(_)) => true,
        _ => crate::model::md_eq(b, es, index, fmt_part_decoded(r)) && crate::alloc_guard::req_of_log(l) == req_d(
            b,
            es,
            index,
        ) && crate::alloc_guard::all_ok(l),
    }
}

/// Clause 2 of `G` for `decode_entry`.
pub open spec fn g_decode_clause2(
    b: Seq<u8>,
    es: Seq<Entry>,
    index: u32,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    match r {
        Err(TryError::OutOfMemory(o)) =>
            l.len() >= 1
            && crate::alloc_guard::req_of_log(l).is_prefix_of(req_d(b, es, index))
            && (forall|j: int| 0 <= j < l.len() - 1 ==> l[j].2)
            && !l[l.len() - 1].2
            && l[l.len() - 1].0 == o.site_spec()
            && l[l.len() - 1].1 == o.elements_spec() as nat,
        _ => true,
    }
}

/// Clause 3 of `G` for `decode_entry`: every request is within its site's
/// maximum. The only request is DIB's `(Dib(Output), 4N)` with `4N <= MAX_BYTES`.
pub open spec fn g_decode_clause3(b: Seq<u8>, es: Seq<Entry>, index: u32) -> bool {
    forall|j: int| 0 <= j < req_d(b, es, index).len()
        ==> req_d(b, es, index)[j].1 <= crate::alloc_guard::MAX_BYTES
}

/// Theorem shape `G` for `try_decode_entry` (THEOREMS-ALLOC-DELTA.md A-ICO-2).
pub open spec fn g_decode(
    b: Seq<u8>,
    es: Seq<Entry>,
    index: u32,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    g_decode_clause1(b, es, index, r, l) && g_decode_clause2(b, es, index, r, l) && g_decode_clause3(
        b,
        es,
        index,
    )
}

/// `Prog` for `decode_entry`.
pub open spec fn prog_decode(
    b: Seq<u8>,
    es: Seq<Entry>,
    index: u32,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    crate::alloc_guard::all_ok(l) ==> !matches!(r, Err(TryError::OutOfMemory(_))) && crate::model::md_eq(
        b,
        es,
        index,
        fmt_part_decoded(r),
    ) && crate::alloc_guard::req_of_log(l) == req_d(b, es, index)
}

// ---------------------------------------------------------------------------
// Facts about map_dib: A-ICO-2's steps
// ---------------------------------------------------------------------------

/// Requests commute with the site mapping.
pub proof fn lemma_req_of_map_dib(dl: Seq<(dib_verus::alloc_guard::AllocSite, nat, bool)>)
    ensures
        crate::alloc_guard::req_of_log(map_dib(dl)) == map_dib_req(dib_verus::alloc_guard::req_of_log(dl)),
{
    reveal(crate::alloc_guard::req_of_log);
    reveal(dib_verus::alloc_guard::req_of_log);
    reveal(map_dib);
    reveal(map_dib_req);
    assert(crate::alloc_guard::req_of_log(map_dib(dl)) =~= map_dib_req(dib_verus::alloc_guard::req_of_log(dl)));
}

/// Prefix is preserved, which carries clause 2's "prefix of `Req`" across.
pub proof fn lemma_map_dib_req_prefix(
    a: Seq<(dib_verus::alloc_guard::AllocSite, nat)>,
    whole: Seq<(dib_verus::alloc_guard::AllocSite, nat)>,
)
    requires a.is_prefix_of(whole),
    ensures map_dib_req(a).is_prefix_of(map_dib_req(whole)),
{
    reveal(map_dib_req);
    assert(map_dib_req(a).len() == a.len());
    assert(map_dib_req(whole).len() == whole.len());
    assert forall|j: int| 0 <= j < map_dib_req(a).len() implies map_dib_req(a)[j]
        == map_dib_req(whole)[j] by {
        assert(a[j] == whole[j]);
    }
    assert(map_dib_req(a).len() <= map_dib_req(whole).len());
}

/// The `ok` flag is untouched, which carries `all_ok` and the failed-last-entry
/// facts across.
pub proof fn lemma_all_ok_map_dib(dl: Seq<(dib_verus::alloc_guard::AllocSite, nat, bool)>)
    ensures
        crate::alloc_guard::all_ok(map_dib(dl)) == dib_verus::alloc_guard::all_ok(dl),
{
    reveal(crate::alloc_guard::all_ok);
    reveal(crate::alloc_guard::all_ok);
    reveal(map_dib);
    assert forall|j: int| 0 <= j < map_dib(dl).len() implies map_dib(dl)[j].2 == dl[j].2 by {};
}

/// The last entry of `map_dib(dl)` is DIB's last entry with its site renamed.
pub proof fn lemma_map_dib_last(dl: Seq<(dib_verus::alloc_guard::AllocSite, nat, bool)>)
    requires dl.len() >= 1,
    ensures
        map_dib(dl).len() == dl.len(),
        map_dib(dl)[map_dib(dl).len() - 1].0 == map_site(dl[dl.len() - 1].0),
        map_dib(dl)[map_dib(dl).len() - 1].1 == dl[dl.len() - 1].1,
        map_dib(dl)[map_dib(dl).len() - 1].2 == dl[dl.len() - 1].2,
{
    reveal(map_dib);
    assert(map_dib(dl).len() == dl.len());
}

// ---------------------------------------------------------------------------
// import_dib (SPEC.md §3.1)
// ---------------------------------------------------------------------------

/// Exec counterpart of `map_site`, used to build the wrapped `OutOfMemory`.
pub fn map_site_exec(s: dib_verus::alloc_guard::AllocSite) -> (r: AllocSite)
    ensures r == map_site(s),
{
    AllocSite::Dib(s)
}

/// A-ICO-2's `G` on the DIB arm, stated over `import_dib`'s returned value and
/// its final log alone, because `import_dib` consumes DIB's result and log. The
/// success side of the nested result satisfies DIB's `decode_mp_eq` on the
/// slice; the `Err` side is the imported `OutOfMemory`. `Req` is
/// `map_dib(Req_DIB(slice, IcoEntryDibSlack))`.
pub open spec fn g_dib_arm(
    s: Seq<u8>,
    out: Result<Result<Decoded, dib_verus::Error>, OutOfMemory>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    match out {
        Err(o) =>
            l.len() >= 1
            && crate::alloc_guard::req_of_log(l).is_prefix_of(
                map_dib_req(dib_verus::dib::req_dib(s, dib_verus::Mode::IcoEntryDibSlack)),
            )
            && (forall|j: int| 0 <= j < l.len() - 1 ==> l[j].2)
            && !l[l.len() - 1].2
            && l[l.len() - 1].0 == o.site_spec()
            && l[l.len() - 1].1 == o.elements_spec() as nat,
        Ok(inner) => dib_verus::dib::decode_mp_eq(
            inner,
            s,
            dib_verus::Mode::IcoEntryDibSlack,
        ) && crate::alloc_guard::req_of_log(l) == map_dib_req(
            dib_verus::dib::req_dib(s, dib_verus::Mode::IcoEntryDibSlack),
        ) && crate::alloc_guard::all_ok(l),
    }
}

/// `Prog` on the DIB arm, alongside `g_dib_arm`.
pub open spec fn prog_dib_arm(
    s: Seq<u8>,
    out: Result<Result<Decoded, dib_verus::Error>, OutOfMemory>,
    l: Seq<(AllocSite, nat, bool)>,
) -> bool {
    crate::alloc_guard::all_ok(l) ==> match out {
        Err(_) => false,
        Ok(inner) => dib_verus::dib::decode_mp_eq(
            inner,
            s,
            dib_verus::Mode::IcoEntryDibSlack,
        ) && crate::alloc_guard::req_of_log(l) == map_dib_req(
            dib_verus::dib::req_dib(s, dib_verus::Mode::IcoEntryDibSlack),
        ),
    }
}

/// The `reveal` set the three arm lemmas of A-ICO-2 need, in one place.
proof fn reveal_g_dib_arm()
    ensures true,
{
    reveal(g_dib_arm);
    reveal(prog_dib_arm);
    reveal(crate::alloc_guard::req_of_log);
    reveal(dib_verus::alloc_guard::req_of_log);
    reveal(map_dib);
    reveal(map_dib_req);
    reveal(map_site);
    reveal(crate::alloc_guard::all_ok);
    reveal(crate::model::h);
    reveal(crate::model::parse_eq);
    reveal(crate::model::md_ico);
    reveal(crate::model::md_eq);
    reveal(dib_verus::dib::g_logged);
    reveal(dib_verus::dib::g);
    reveal(dib_verus::dib::g_clause1);
    reveal(dib_verus::dib::g_clause2);
    reveal(dib_verus::dib::prog_logged);
    reveal(dib_verus::dib::prog);
    reveal(dib_verus::dib::prog_concl);
    reveal(dib_verus::dib::req_dib);
    reveal(dib_verus::dib::fmt_part);
    reveal(dib_verus::dib::decode_mp_eq);
    reveal(crate::alloc_guard::all_ok);
    reveal(crate::alloc_guard::req_of_log);
}

/// `G`'s clauses fix the log length: on `Ok`/`Format` `req(L) == Req_P`, and on
/// `OutOfMemory` `L` is a non-empty prefix of `Req_P` and `Req_P` has at most one
/// element, so `L.len() == Req_P.len()` in every case. This is A-ICO-4's
/// `r.2@ == L.len() <= 1`.
pub proof fn lemma_g_parse_log_len(
    b: Seq<u8>,
    r: Result<Vec<Entry>, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
)
    requires
        g_parse(b, r, l),
    ensures
        l.len() == req_p(b).len(),
{
    reveal(g_parse);
    reveal(g_parse_clause1);
    reveal(g_parse_clause2);
    reveal(req_p);
    reveal(crate::alloc_guard::req_of_log);
    reveal(crate::model::h);
    assert(req_p(b).len() <= 1);
    match r {
        Err(TryError::OutOfMemory(o)) => {
            assert(crate::alloc_guard::req_of_log(l).is_prefix_of(req_p(b)));
            assert(crate::alloc_guard::req_of_log(l).len() == l.len());
            assert(l.len() <= req_p(b).len());
        },
        _ => {
            assert(crate::alloc_guard::req_of_log(l) == req_p(b));
            assert(crate::alloc_guard::req_of_log(l).len() == l.len());
        },
    }
}

/// The header-error exit of `try_parse_logged`: `H(b)` failed before any
/// reservation, so the log is empty and the result is a format error.
pub proof fn lemma_g_parse_head_error(b: Seq<u8>, e: Error, l: Seq<(AllocSite, nat, bool)>)
    requires
        crate::model::h(b) == Err(e),
        l == Seq::<(AllocSite, nat, bool)>::empty(),
    ensures
        g_parse(b, Err::<Vec<Entry>, TryError>(TryError::Format(e)), l),
        prog_parse(b, Err::<Vec<Entry>, TryError>(TryError::Format(e)), l),
{
    reveal(g_parse);
    reveal(g_parse_clause1);
    reveal(g_parse_clause2);
    reveal(g_parse_clause3);
    reveal(prog_parse);
    reveal(req_p);
    reveal(fmt_part);
    reveal(crate::model::parse_eq);
    reveal(crate::model::mc);
    reveal(crate::alloc_guard::req_of_log);
    reveal(crate::alloc_guard::all_ok);
    assert(crate::model::parse_eq(b, Err::<Vec<Entry>, Error>(e)));
    assert(req_p(b) == Seq::<(AllocSite, nat)>::empty());
    assert(crate::alloc_guard::req_of_log(l) =~= Seq::<(AllocSite, nat)>::empty());
    assert(crate::alloc_guard::all_ok(l));
}

/// The `Ok` exit of `try_parse_logged`: the reservation succeeded, every record
/// was pushed into its room, and the log holds one `Ok` entry.
pub proof fn lemma_g_parse_ok(
    b: Seq<u8>,
    v: Vec<Entry>,
    count: usize,
    l: Seq<(AllocSite, nat, bool)>,
)
    requires
        crate::model::parse_eq(b, Ok::<Vec<Entry>, Error>(v)),
        crate::alloc_guard::sites::req_p(b) == seq![(AllocSite::EntryList, count as nat)],
        l == Seq::<(AllocSite, nat, bool)>::empty().push((AllocSite::EntryList, count as nat, true)),
    ensures
        g_parse(b, Ok::<Vec<Entry>, TryError>(v), l),
        prog_parse(b, Ok::<Vec<Entry>, TryError>(v), l),
{
    reveal(g_parse);
    reveal(g_parse_clause1);
    reveal(g_parse_clause2);
    reveal(g_parse_clause3);
    reveal(prog_parse);
    reveal(fmt_part);
    reveal(req_p);
    reveal(crate::model::parse_eq);
    reveal(crate::model::h);
    reveal(crate::alloc_guard::req_of_log);
    reveal(crate::alloc_guard::all_ok);
    assert(fmt_part(Ok::<Vec<Entry>, TryError>(v)) == Ok::<Vec<Entry>, Error>(v));
    assert(req_p(b) == seq![(AllocSite::EntryList, count as nat)]);
    assert(crate::alloc_guard::req_of_log(l) =~= req_p(b));
    assert(crate::alloc_guard::all_ok(l));
    assert(g_parse_clause3(b));
}

/// The `OutOfMemory` exit of `try_parse_logged`: the reservation failed and the
/// log holds one failed entry equal to the returned `(site, elements)`.
pub proof fn lemma_g_parse_oom(
    b: Seq<u8>,
    o: OutOfMemory,
    count: usize,
    l: Seq<(AllocSite, nat, bool)>,
)
    requires
        crate::alloc_guard::sites::req_p(b) == seq![(AllocSite::EntryList, count as nat)],
        o.site_spec() == AllocSite::EntryList,
        o.elements_spec() == count,
        l == Seq::<(AllocSite, nat, bool)>::empty().push((AllocSite::EntryList, count as nat, false)),
    ensures
        g_parse(b, Err::<Vec<Entry>, TryError>(TryError::OutOfMemory(o)), l),
        prog_parse(b, Err::<Vec<Entry>, TryError>(TryError::OutOfMemory(o)), l),
        (o.site_spec(), o.elements_spec() as nat) == req_p(b)[0],
{
    reveal(g_parse);
    reveal(g_parse_clause1);
    reveal(g_parse_clause2);
    reveal(g_parse_clause3);
    reveal(prog_parse);
    reveal(fmt_part);
    reveal(req_p);
    reveal(crate::alloc_guard::req_of_log);
    reveal(crate::alloc_guard::all_ok);
    assert(req_p(b) == seq![(AllocSite::EntryList, count as nat)]);
    assert(crate::alloc_guard::req_of_log(l) =~= seq![(AllocSite::EntryList, count as nat)]);
    assert(o.site_spec() == AllocSite::EntryList);
    assert(o.elements_spec() as nat == count as nat);
    assert((o.site_spec(), o.elements_spec() as nat) == req_p(b)[0]);
    assert(crate::alloc_guard::req_of_log(l).is_prefix_of(req_p(b)));
    assert(l.len() >= 1);
    assert(!l[l.len() - 1].2);
    assert(crate::alloc_guard::all_ok(l) == false);
}

// ---------------------------------------------------------------------------
// Bridging `G` to concrete outcomes (used by the public witnesses)
// ---------------------------------------------------------------------------

/// `G` for `decode_entry` with an empty `Req_D` rules out `OutOfMemory`: clause 2
/// would need a non-empty log that is a prefix of the empty `Req_D`.
pub proof fn lemma_g_decode_prealloc_not_oom(
    b: Seq<u8>,
    es: Seq<Entry>,
    index: u32,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
)
    requires
        g_decode(b, es, index, r, l),
        req_d(b, es, index).len() == 0,
    ensures
        !matches!(r, Err(TryError::OutOfMemory(_))),
{
    reveal(g_decode);
    reveal(g_decode_clause2);
    reveal(crate::alloc_guard::req_of_log);
    if let Err(TryError::OutOfMemory(_)) = r {
        assert(crate::alloc_guard::req_of_log(l).is_prefix_of(req_d(b, es, index)));
        assert(crate::alloc_guard::req_of_log(l).len() == l.len());
        assert(l.len() <= req_d(b, es, index).len());
        assert(l.len() >= 1);
        assert(false);
    }
}

/// The frozen `PreAlloc` corollary for `decode_entry` (THEOREMS-ALLOC-DELTA.md
/// §0): `Req_D(b, E, i) == []` implies the result is not `OutOfMemory`, `md_eq`
/// holds on `fmt_part_decoded(r)`, and the returned log is empty.
pub proof fn lemma_g_decode_prealloc(
    b: Seq<u8>,
    es: Seq<Entry>,
    index: u32,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
)
    requires
        g_decode(b, es, index, r, l),
        req_d(b, es, index).len() == 0,
    ensures
        !matches!(r, Err(TryError::OutOfMemory(_))),
        crate::model::md_eq(b, es, index, fmt_part_decoded(r)),
        l == Seq::<(AllocSite, nat, bool)>::empty(),
{
    lemma_g_decode_prealloc_not_oom(b, es, index, r, l);
    reveal(g_decode);
    reveal(g_decode_clause1);
    reveal(fmt_part_decoded);
    reveal(crate::alloc_guard::req_of_log);
    match r {
        Err(TryError::OutOfMemory(_)) => { assert(false); },
        _ => {
            assert(crate::model::md_eq(b, es, index, fmt_part_decoded(r)));
            assert(crate::alloc_guard::req_of_log(l) == req_d(b, es, index));
            assert(crate::alloc_guard::req_of_log(l).len() == l.len());
            assert(l.len() == 0);
            assert(l =~= Seq::<(AllocSite, nat, bool)>::empty());
        },
    }
}

/// The public `try_decode_entry` contract implies the `PreAlloc` corollary: its
/// existential `G` witness is the log the corollary constrains.
pub proof fn lemma_public_decode_prealloc(
    b: Seq<u8>,
    es: Seq<Entry>,
    index: u32,
    r: Result<Decoded, TryError>,
)
    requires
        exists|l: Seq<(AllocSite, nat, bool)>| g_decode(b, es, index, r, l),
        req_d(b, es, index).len() == 0,
    ensures
        !matches!(r, Err(TryError::OutOfMemory(_))),
        crate::model::md_eq(b, es, index, fmt_part_decoded(r)),
{
    let l = choose|l: Seq<(AllocSite, nat, bool)>| g_decode(b, es, index, r, l);
    lemma_g_decode_prealloc(b, es, index, r, l);
}

/// `G` for `decode_entry`, an empty `Req_D`, and `MD = Err(e)`: the result is
/// exactly `Err(TryError::Format(e))`.
pub proof fn lemma_g_decode_format(
    b: Seq<u8>,
    es: Seq<Entry>,
    index: u32,
    r: Result<Decoded, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
    e: Error,
)
    requires
        g_decode(b, es, index, r, l),
        req_d(b, es, index).len() == 0,
        crate::model::md(b, es, index) == Err(e),
    ensures
        r == Err(TryError::Format(e)),
{
    lemma_g_decode_prealloc_not_oom(b, es, index, r, l);
    reveal(g_decode);
    reveal(g_decode_clause1);
    reveal(fmt_part_decoded);
    reveal(crate::model::md_eq);
    reveal(crate::model::md);
    reveal(crate::model::decoded_matches);
    assert(crate::model::md_eq(b, es, index, fmt_part_decoded(r)));
    match r {
        Ok(d) => {
            assert(false);
            assert(fmt_part_decoded(Ok::<Decoded, TryError>(d)) == Ok::<Decoded, Error>(d));
            assert(false);
        },
        Err(TryError::OutOfMemory(_)) => { assert(false); },
        Err(TryError::Format(e2)) => {
            assert(fmt_part_decoded(Err::<Decoded, TryError>(TryError::Format(e2)))
                == Err::<Decoded, Error>(e2));
            assert(e2 == e);
        },
    }
}

/// `G` for `parse` with an empty `Req_P` rules out `OutOfMemory`.
pub proof fn lemma_g_parse_prealloc_not_oom(
    b: Seq<u8>,
    r: Result<Vec<Entry>, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
)
    requires
        g_parse(b, r, l),
        req_p(b).len() == 0,
    ensures
        !matches!(r, Err(TryError::OutOfMemory(_))),
{
    reveal(g_parse);
    reveal(g_parse_clause2);
    reveal(crate::alloc_guard::req_of_log);
    if let Err(TryError::OutOfMemory(_)) = r {
        assert(crate::alloc_guard::req_of_log(l).is_prefix_of(req_p(b)));
        assert(crate::alloc_guard::req_of_log(l).len() == l.len());
        assert(l.len() <= req_p(b).len());
        assert(l.len() >= 1);
        assert(false);
    }
}

/// The frozen `PreAlloc` corollary for `parse` (THEOREMS-ALLOC-DELTA.md §0):
/// `Req_P(b) == []` implies the result is not `OutOfMemory`, `parse_eq` holds on
/// `fmt_part(r)`, and the returned log is empty.
pub proof fn lemma_g_parse_prealloc(
    b: Seq<u8>,
    r: Result<Vec<Entry>, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
)
    requires
        g_parse(b, r, l),
        req_p(b).len() == 0,
    ensures
        !matches!(r, Err(TryError::OutOfMemory(_))),
        crate::model::parse_eq(b, fmt_part(r)),
        l == Seq::<(AllocSite, nat, bool)>::empty(),
{
    lemma_g_parse_prealloc_not_oom(b, r, l);
    reveal(g_parse);
    reveal(g_parse_clause1);
    reveal(fmt_part);
    reveal(crate::alloc_guard::req_of_log);
    match r {
        Err(TryError::OutOfMemory(_)) => { assert(false); },
        _ => {
            assert(crate::model::parse_eq(b, fmt_part(r)));
            assert(crate::alloc_guard::req_of_log(l) == req_p(b));
            assert(crate::alloc_guard::req_of_log(l).len() == l.len());
            assert(l.len() == 0);
            assert(l =~= Seq::<(AllocSite, nat, bool)>::empty());
        },
    }
}

/// The public `try_parse` contract implies the `PreAlloc` corollary: its
/// existential `G` witness is the log the corollary constrains.
pub proof fn lemma_public_parse_prealloc(
    b: Seq<u8>,
    r: Result<Vec<Entry>, TryError>,
)
    requires
        exists|l: Seq<(AllocSite, nat, bool)>| g_parse(b, r, l),
        req_p(b).len() == 0,
    ensures
        !matches!(r, Err(TryError::OutOfMemory(_))),
        crate::model::parse_eq(b, fmt_part(r)),
{
    let l = choose|l: Seq<(AllocSite, nat, bool)>| g_parse(b, r, l);
    lemma_g_parse_prealloc(b, r, l);
}

/// `G` for `parse`, an empty `Req_P`, and `MC = Err(e)`: the result is exactly
/// `Err(TryError::Format(e))`.
pub proof fn lemma_g_parse_format(
    b: Seq<u8>,
    r: Result<Vec<Entry>, TryError>,
    l: Seq<(AllocSite, nat, bool)>,
    e: Error,
)
    requires
        g_parse(b, r, l),
        req_p(b).len() == 0,
        crate::model::mc(b) == Err(e),
    ensures
        r == Err(TryError::Format(e)),
{
    lemma_g_parse_prealloc_not_oom(b, r, l);
    reveal(g_parse);
    reveal(g_parse_clause1);
    reveal(fmt_part);
    reveal(crate::model::parse_eq);
    assert(crate::model::parse_eq(b, fmt_part(r)));
    match r {
        Ok(v) => {
            assert(fmt_part(Ok::<Vec<Entry>, TryError>(v)) == Ok::<Vec<Entry>, Error>(v));
            assert(false);
        },
        Err(TryError::OutOfMemory(_)) => { assert(false); },
        Err(TryError::Format(e2)) => {
            assert(fmt_part(Err::<Vec<Entry>, TryError>(TryError::Format(e2)))
                == Err::<Vec<Entry>, Error>(e2));
            assert(e2 == e);
        },
    }
}

/// `req_p(b).len() == 0` whenever `H(b)` is an error.
pub proof fn lemma_req_p_empty_on_header_error(b: Seq<u8>, e: Error)
    requires
        crate::model::h(b) == Err(e),
    ensures
        req_p(b).len() == 0,
{
    reveal(req_p);
}

/// `req_d(b, es, index).len() == 0` whenever `MD` does not reach the DIB call.
pub proof fn lemma_req_d_empty_off_dib(b: Seq<u8>, es: Seq<Entry>, index: u32)
    requires
        !(crate::model::md_ico(b, es, index) is DibSlice),
    ensures
        req_d(b, es, index).len() == 0,
{
    reveal(req_d);
    reveal(crate::model::md_ico);
}

/// `to_ico_result(out)`: the ICO result `import_dib` produces from DIB's nested
/// result (SPEC.md §3.1).
pub open spec fn to_ico_result(
    out: Result<Result<Decoded, dib_verus::Error>, OutOfMemory>,
) -> Result<Decoded, TryError> {
    match out {
        Ok(inner) => match inner {
            Ok(d) => Ok(d),
            Err(e) => Err(TryError::Format(Error::Dib(e))),
        },
        Err(o) => Err(TryError::OutOfMemory(o)),
    }
}

/// `Req_D`'s only element is within its site maximum, by DIB's clause 3.
pub proof fn lemma_req_d_clause3(b: Seq<u8>, es: Seq<Entry>, index: u32)
    requires
        crate::model::md_ico(b, es, index) is DibSlice,
    ensures
        g_decode_clause3(b, es, index),
{
    let (start, end) = match crate::model::md_ico(b, es, index) {
        crate::model::IcoDecision::DibSlice(s, e) => (s, e),
        _ => (0, 0),
    };
    reveal(g_decode_clause3);
    reveal(req_d);
    reveal(map_dib_req);
    reveal(map_site);
    reveal(dib_verus::dib::req_dib);
    reveal(crate::alloc_guard::MAX_BYTES);
    dib_verus::dib::lemma_req_dib_clause3(b.subrange(start, end), dib_verus::Mode::IcoEntryDibSlack);
    reveal(dib_verus::dib::g_clause3);
    assert forall|j: int| 0 <= j < req_d(b, es, index).len() implies req_d(b, es, index)[j].1
        <= crate::alloc_guard::MAX_BYTES by {
        assert(req_d(b, es, index) == map_dib_req(
            dib_verus::dib::req_dib(b.subrange(start, end), dib_verus::Mode::IcoEntryDibSlack),
        ));
        let k = dib_verus::dib::req_dib(b.subrange(start, end), dib_verus::Mode::IcoEntryDibSlack);
        assert(req_d(b, es, index)[j].1 == k[j].1);
    }
}

/// `G` for `decode_entry` with an `OutOfMemory` result: the returned
/// `(site, elements)` is `Req_D`'s single element.
pub proof fn lemma_g_decode_oom_site(
    b: Seq<u8>,
    es: Seq<Entry>,
    index: u32,
    o: OutOfMemory,
    l: Seq<(AllocSite, nat, bool)>,
)
    requires
        g_decode(b, es, index, Err::<Decoded, TryError>(TryError::OutOfMemory(o)), l),
    ensures
        (o.site_spec(), o.elements_spec() as nat) == req_d(b, es, index)[0],
{
    reveal(g_decode);
    reveal(g_decode_clause2);
    reveal(req_d);
    reveal(map_dib_req);
    reveal(map_site);
    reveal(crate::alloc_guard::req_of_log);
    reveal(crate::model::md_ico);
    reveal(dib_verus::dib::req_dib);
    let l2 = crate::alloc_guard::req_of_log(l);
    assert(l2.is_prefix_of(req_d(b, es, index)));
    assert(l2.len() == l.len());
    assert(l.len() >= 1);
    assert(crate::alloc_guard::req_of_log(l)[l.len() - 1] == (
        l[l.len() - 1].0,
        l[l.len() - 1].1,
    ));
    assert(l[l.len() - 1].0 == o.site_spec());
    assert(l[l.len() - 1].1 == o.elements_spec() as nat);
    assert(crate::alloc_guard::req_of_log(l)[l.len() - 1] == req_d(b, es, index)[l.len() - 1]);
    assert((o.site_spec(), o.elements_spec() as nat) == req_d(b, es, index)[l.len() - 1]);
    if crate::model::md_ico(b, es, index) is DibSlice {
        let (a, z) = match crate::model::md_ico(b, es, index) {
            crate::model::IcoDecision::DibSlice(a, z) => (a, z),
            _ => (0, 0),
        };
        dib_verus::dib::lemma_req_dib_len(b.subrange(a, z), dib_verus::Mode::IcoEntryDibSlack);
        assert(req_d(b, es, index).len() == dib_verus::dib::req_dib(
            b.subrange(a, z),
            dib_verus::Mode::IcoEntryDibSlack,
        ).len());
        assert(req_d(b, es, index).len() <= 1);
        assert(l.len() == 1);
    } else {
        assert(req_d(b, es, index) =~= Seq::<(AllocSite, nat)>::empty());
        assert(l.len() <= 0);
        assert(false);
    }
}

/// `G` and `Prog` for a `decode_entry` outcome chosen before the DIB call
/// (directory error, index, bad list, defect, PNG): `req_d(b, es, index)` is
/// empty, the log is empty, and `md_eq` must already hold for `err`.
pub proof fn lemma_g_decode_off_dib(
    b: Seq<u8>,
    es: Seq<Entry>,
    index: u32,
    err: Error,
    l: Seq<(AllocSite, nat, bool)>,
)
    requires
        req_d(b, es, index).len() == 0,
        l == Seq::<(AllocSite, nat, bool)>::empty(),
        crate::model::md_eq(b, es, index, Err::<Decoded, Error>(err)),
    ensures
        g_decode(b, es, index, Err::<Decoded, TryError>(TryError::Format(err)), l),
        prog_decode(b, es, index, Err::<Decoded, TryError>(TryError::Format(err)), l),
{
    reveal(g_decode);
    reveal(g_decode_clause1);
    reveal(g_decode_clause2);
    reveal(g_decode_clause3);
    reveal(prog_decode);
    reveal(fmt_part_decoded);
    reveal(req_d);
    reveal(crate::alloc_guard::req_of_log);
    reveal(crate::alloc_guard::all_ok);
    assert(fmt_part_decoded(Err::<Decoded, TryError>(TryError::Format(err)))
        == Err::<Decoded, Error>(err));
    assert(crate::alloc_guard::req_of_log(l) =~= Seq::<(AllocSite, nat)>::empty());
    assert(req_d(b, es, index) =~= Seq::<(AllocSite, nat)>::empty());
}

/// `G` for DIB's `Ok(Ok(d))` arm.
/// `ensures`), derive `g_decode` on `to_ico_result(out)`. Takes `out` by
/// reference so the caller keeps ownership for its own result match.
pub proof fn lemma_g_decode_of_dib_arm(
    b: Seq<u8>,
    es: Seq<Entry>,
    index: u32,
    s: Seq<u8>,
    a: int,
    z: int,
    out: &Result<Result<Decoded, dib_verus::Error>, OutOfMemory>,
    l: Seq<(AllocSite, nat, bool)>,
)
    requires
        crate::model::md_ico(b, es, index) == crate::model::IcoDecision::DibSlice(a, z),
        s == b.subrange(a, z),
        g_dib_arm(s, *out, l),
    ensures
        g_decode(b, es, index, to_ico_result(*out), l),
        prog_decode(b, es, index, to_ico_result(*out), l),
{
    reveal(g_decode);
    reveal(g_decode_clause1);
    reveal(g_decode_clause2);
    reveal(g_decode_clause3);
    reveal(prog_decode);
    reveal(fmt_part_decoded);
    reveal(to_ico_result);
    reveal(req_d);
    reveal(map_dib_req);
    reveal(map_site);
    reveal(g_dib_arm);
    reveal(prog_dib_arm);
    reveal(crate::alloc_guard::req_of_log);
    reveal(crate::alloc_guard::all_ok);
    reveal(crate::model::md_ico);
    reveal(crate::model::md_eq);
    reveal(crate::model::md);
    reveal(crate::model::dib_wrap);
    reveal(dib_verus::dib::req_dib);
    reveal(dib_verus::dib::decode_mp_eq);
    reveal(dib_verus::dib::fmt_part);
    assert(req_d(b, es, index) == map_dib_req(
        dib_verus::dib::req_dib(s, dib_verus::Mode::IcoEntryDibSlack),
    ));
    lemma_req_d_clause3(b, es, index);
    match *out {
        Ok(inner) => {
            match inner {
                Ok(d) => {
                    crate::model::lemma_md_eq_dib(
                        b,
                        es,
                        index,
                        Ok::<Decoded, dib_verus::Error>(d),
                        a,
                        z,
                        s,
                    );
                    assert(to_ico_result(*out) == Ok::<Decoded, TryError>(d));
                    assert(fmt_part_decoded(to_ico_result(*out)) == Ok::<Decoded, Error>(d));
                },
                Err(e) => {
                    crate::model::lemma_md_eq_dib(
                        b,
                        es,
                        index,
                        Err::<Decoded, dib_verus::Error>(e),
                        a,
                        z,
                        s,
                    );
                    assert(to_ico_result(*out) == Err::<Decoded, TryError>(TryError::Format(
                        Error::Dib(e),
                    )));
                    assert(fmt_part_decoded(to_ico_result(*out)) == Err::<Decoded, Error>(Error::Dib(e)));
                },
            }
        },
        Err(o) => {
            assert(to_ico_result(*out) == Err::<Decoded, TryError>(TryError::OutOfMemory(o)));
            assert(!crate::alloc_guard::all_ok(l));
        },
    }
}

/// A-ICO-2 for DIB's `Ok` arm.
pub proof fn lemma_g_ico_ok(
    s: Seq<u8>,
    d: Decoded,
    dl: Seq<(dib_verus::alloc_guard::AllocSite, nat, bool)>,
    il: Seq<(AllocSite, nat, bool)>,
)
    requires
        dib_verus::dib::g_logged(s, Ok::<Decoded, dib_verus::TryError>(d), dl),
        il == map_dib(dl),
    ensures
        g_dib_arm(s, Ok::<Result<Decoded, dib_verus::Error>, OutOfMemory>(Ok(d)), il),
        prog_dib_arm(s, Ok::<Result<Decoded, dib_verus::Error>, OutOfMemory>(Ok(d)), il),
{
    reveal_g_dib_arm();
    assert(dib_verus::dib::g_clause1(
        s,
        dib_verus::Mode::IcoEntryDibSlack,
        Ok::<Decoded, dib_verus::TryError>(d),
        dl,
    ));
    assert(dib_verus::dib::decode_mp_eq(
        dib_verus::dib::fmt_part(Ok::<Decoded, dib_verus::TryError>(d)),
        s,
        dib_verus::Mode::IcoEntryDibSlack,
    ));
    assert(dib_verus::dib::fmt_part(Ok::<Decoded, dib_verus::TryError>(d))
        == Ok::<Decoded, dib_verus::Error>(d));
    assert(dib_verus::alloc_guard::req_of_log(dl) =~= dib_verus::dib::req_dib(s, dib_verus::Mode::IcoEntryDibSlack));
    lemma_req_of_map_dib(dl);
    assert(crate::alloc_guard::req_of_log(il) =~= map_dib_req(dib_verus::alloc_guard::req_of_log(dl)));
    lemma_all_ok_map_dib(dl);
    assert(crate::alloc_guard::all_ok(il));
}

/// A-ICO-2 for DIB's `Format` arm.
pub proof fn lemma_g_ico_format(
    s: Seq<u8>,
    e: dib_verus::Error,
    dl: Seq<(dib_verus::alloc_guard::AllocSite, nat, bool)>,
    il: Seq<(AllocSite, nat, bool)>,
)
    requires
        dib_verus::dib::g_logged(
            s,
            Err::<Decoded, dib_verus::TryError>(dib_verus::TryError::Format(e)),
            dl,
        ),
        il == map_dib(dl),
    ensures
        g_dib_arm(s, Ok::<Result<Decoded, dib_verus::Error>, OutOfMemory>(Err(e)), il),
        prog_dib_arm(s, Ok::<Result<Decoded, dib_verus::Error>, OutOfMemory>(Err(e)), il),
{
    reveal_g_dib_arm();
    assert(dib_verus::dib::g_clause1(
        s,
        dib_verus::Mode::IcoEntryDibSlack,
        Err::<Decoded, dib_verus::TryError>(dib_verus::TryError::Format(e)),
        dl,
    ));
    assert(dib_verus::dib::decode_mp_eq(
        dib_verus::dib::fmt_part(
            Err::<Decoded, dib_verus::TryError>(dib_verus::TryError::Format(e)),
        ),
        s,
        dib_verus::Mode::IcoEntryDibSlack,
    ));
    assert(dib_verus::dib::fmt_part(
        Err::<Decoded, dib_verus::TryError>(dib_verus::TryError::Format(e)),
    ) == Err::<Decoded, dib_verus::Error>(e));
    assert(dib_verus::alloc_guard::req_of_log(dl) =~= dib_verus::dib::req_dib(s, dib_verus::Mode::IcoEntryDibSlack));
    lemma_req_of_map_dib(dl);
    assert(crate::alloc_guard::req_of_log(il) =~= map_dib_req(dib_verus::alloc_guard::req_of_log(dl)));
    lemma_all_ok_map_dib(dl);
    assert(crate::alloc_guard::all_ok(il));
}

/// A-ICO-2 for DIB's `OutOfMemory` arm. The wrapped value is exactly
/// `(Dib(o.site()), o.elements())`, and ICO's last entry is DIB's last entry
/// renamed.
pub proof fn lemma_g_ico_oom(
    s: Seq<u8>,
    o: dib_verus::alloc_guard::OutOfMemory,
    dl: Seq<(dib_verus::alloc_guard::AllocSite, nat, bool)>,
    il: Seq<(AllocSite, nat, bool)>,
    out: Result<Result<Decoded, dib_verus::Error>, OutOfMemory>,
)
    requires
        dib_verus::dib::g_logged(
            s,
            Err::<Decoded, dib_verus::TryError>(dib_verus::TryError::OutOfMemory(o)),
            dl,
        ),
        il == map_dib(dl),
        // The returned value's shape, stated through the accessors rather than a
        // constructor: an `OutOfMemory` literal may appear only in this module,
        // and this is that module.
        out is Err,
        match out {
            Err(o2) => o2.site_spec() == map_site(o.site_spec()) && o2.elements_spec()
                == o.elements_spec(),
            _ => false,
        },
    ensures
        g_dib_arm(s, out, il),
        prog_dib_arm(s, out, il),
{
    reveal_g_dib_arm();
    assert(dib_verus::dib::g_clause2(
        s,
        dib_verus::Mode::IcoEntryDibSlack,
        Err::<Decoded, dib_verus::TryError>(dib_verus::TryError::OutOfMemory(o)),
        dl,
    ));
    assert(dl.len() >= 1);
    lemma_map_dib_last(dl);
    assert(il.len() >= 1);
    assert(il[il.len() - 1].0 == map_site(o.site_spec()));
    assert(il[il.len() - 1].1 == o.elements_spec() as nat);
    assert(!il[il.len() - 1].2);
    lemma_all_ok_map_dib(dl);
    assert(!crate::alloc_guard::all_ok(il));
    lemma_req_of_map_dib(dl);
    assert(crate::alloc_guard::req_of_log(il) =~= map_dib_req(dib_verus::alloc_guard::req_of_log(dl)));
    assert(dib_verus::alloc_guard::req_of_log(dl) =~= dib_verus::dib::req_dib(s, dib_verus::Mode::IcoEntryDibSlack));
    lemma_map_dib_req_prefix(
        dib_verus::alloc_guard::req_of_log(dl),
        dib_verus::dib::req_dib(s, dib_verus::Mode::IcoEntryDibSlack),
    );
}

/// `import_dib` (SPEC.md §3.1). Consumes DIB's returned log, appends its events
/// to ICO's log through `map_dib`, and wraps a DIB `OutOfMemory`. The nested
/// result leaves the format-error wrapping to ICO's existing code.
#[inline(never)]
pub fn import_dib(
    s: &[u8],
    r: Result<Decoded, dib_verus::TryError>,
    Tracked(dlog): Tracked<dib_verus::alloc_guard::AllocLog>,
    Tracked(log): Tracked<&mut AllocLog>,
) -> (out: Result<Result<Decoded, dib_verus::Error>, OutOfMemory>)
    requires
        old(log).view() == Seq::<(AllocSite, nat, bool)>::empty(),
        dib_verus::dib::log_tied(r, dlog.view()),
        // DIB's exported graceful-failure shape on the same result and log.
        // `try_decode_with_steps_logged` (A-DIB-8) provides it.
        dib_verus::dib::g_logged(s@, r, dlog.view()),
    ensures
        final(log).view() == map_dib(dlog.view()),
        match r {
            Ok(d) => out == Ok(Ok::<Decoded, dib_verus::Error>(d)),
            Err(dib_verus::TryError::Format(e)) => out == Ok(Err::<Decoded, dib_verus::Error>(e)),
            Err(dib_verus::TryError::OutOfMemory(o)) => out is Err && out->Err_0.site_spec()
                == AllocSite::Dib(o.site_spec()) && out->Err_0.elements_spec() == o.elements_spec()
                && final(log).view().last() == (
                AllocSite::Dib(o.site_spec()),
                o.elements_spec() as nat,
                false,
            ),
        },
        // A-ICO-2, stated over the returned value and the final log only, so the
        // caller can use it after `r` and `dlog` are consumed (they are linear).
        g_dib_arm(s@, out, final(log).view()),
        prog_dib_arm(s@, out, final(log).view()),
{
    proof { log.ev = map_dib(dlog.view()); }
    match r {
        Ok(d) => {
            proof { lemma_g_ico_ok(s@, d, dlog.view(), log@); }
            Ok(Ok(d))
        },
        Err(dib_verus::TryError::Format(e)) => {
            proof { lemma_g_ico_format(s@, e, dlog.view(), log@); }
            Ok(Err(e))
        },
        Err(dib_verus::TryError::OutOfMemory(o)) => {
            let out = Err(OutOfMemory { site: map_site_exec(o.site()), elements: o.elements() });
            proof { lemma_g_ico_oom(s@, o, dlog.view(), log@, out); }
            out
        },
    }
}

} // verus!
