//! Toy entry-list decoder: one `try_with_room` reservation for `min(4096,
//! (|b|-8)/8)` entries, each pushed through `Reserved::push` with the
//! `len < room` precondition (SPEC.md §4 ICNS `EntryList`, A-ICNS-1/2).
//!
//! This exercises the second reservation shape: a `Reserved<T>` whose room is
//! fixed by the reservation, filled only by `push`, and a zero-element request
//! that returns `Ok` without touching the allocator.

use vstd::prelude::*;

use crate::alloc_guard::{self, AllocLog, AllocSite, DecodeError, Entry, OutOfMemory, TryError};
use crate::alloc_guard::{marker_hex, marker_hex_exec};
use crate::alloc_guard::{all_ok, fmt_part, req_of_log};

verus! {

pub type LogSeq = crate::toy::LogSeq;

/// The frozen model: the list of `count` entries, each with the eight-byte type
/// marker the decoder writes.
pub open spec fn model_entries(b: &[u8], count: nat) -> Seq<Entry> {
    Seq::new(count, |i: int| built_entry(i as nat))
}

/// `Reach_P(b)`: R2-R4 pass, so the entry list is reserved. The count is
/// `min(4096, (|b|-8)/8)`, computed from checked values before the guard call.
pub open spec fn reach_p(b: &[u8]) -> bool {
    8 <= b.len() && b.len() <= 67_108_864
}

/// The request count, from checked values: `min(4096, (|b|-8)/8)`.
pub open spec fn entry_count(b: &[u8]) -> nat {
    if b.len() >= 8 {
        if (b.len() - 8) / 8 <= 4_096 { ((b.len() - 8) / 8) as nat } else { 4_096 }
    } else {
        0
    }
}

pub open spec fn req_p(b: &[u8]) -> Seq<(AllocSite, nat)> {
    if reach_p(b) {
        Seq::empty().push((AllocSite::EntryList, entry_count(b)))
    } else {
        Seq::empty()
    }
}

pub open spec fn pre_alloc_p(b: &[u8]) -> bool {
    req_p(b).len() == 0
}

/// The frozen executable-to-model predicate for this entry point (SPEC.md §2,
/// standing in for ICNS's `parses_to`).
pub open spec fn parse_eq(r: Result<Vec<Entry>, DecodeError>, b: &[u8]) -> bool {
    match r {
        Ok(v) => v@ == model_entries(b, entry_count(b)),
        Err(_) => !reach_p(b),
    }
}

pub open spec fn eq_f_p(b: &[u8], r: Result<Vec<Entry>, TryError>) -> bool {
    parse_eq(fmt_part_p(r), b)
}

pub open spec fn fmt_part_p(r: Result<Vec<Entry>, TryError>) -> Result<Vec<Entry>, DecodeError> {
    match r {
        Ok(v) => Ok(v),
        Err(TryError::Format(e)) => Err(e),
        Err(TryError::OutOfMemory(_)) => arbitrary(),
    }
}

pub open spec fn all_ok_p(l: LogSeq) -> bool {
    all_ok(l)
}

pub open spec fn g_p(b: &[u8], r: Result<Vec<Entry>, TryError>, l: LogSeq) -> bool {
    match r {
        Err(TryError::OutOfMemory(o)) =>
            l.len() >= 1
            && req_of_log(l).is_prefix_of(req_p(b))
            && (forall|j: int| 0 <= j < l.len() - 1 ==> l[j].2)
            && !l[l.len() - 1].2
            && l[l.len() - 1].0 == o.site_spec()
            && l[l.len() - 1].1 == o.elements_spec() as nat,
        _ => eq_f_p(b, r) && req_of_log(l) == req_p(b) && all_ok(l),
    }
}

/// One decoder-built entry, the same expression in model and in code.
pub open spec fn built_entry(i: nat) -> Entry {
    Entry {
        type_hex: marker_hex(),
        payload: [i as u32, i as u32],
        opaque_candidate: false,
        nominal_side: None,
    }
}

/// The exec constructor of one entry; its `ensures` is the model expression, so
/// code and model agree by construction.
pub fn mk_entry(i: usize) -> (e: Entry)
    ensures e == built_entry(i as nat),
{
    Entry {
        type_hex: marker_hex_exec(),
        payload: [i as u32, i as u32],
        opaque_candidate: false,
        nominal_side: None,
    }
}

/// The loop invariant is preserved by one push: the pushed entry is the model's
/// entry at index `i`, and `Seq::new` picks it up at that index.
pub proof fn lemma_push_preserves_model(b: &[u8], entries: Seq<Entry>, i: nat)
    requires
        entries =~= model_entries(b, i),
        i < entry_count(b),
    ensures
        entries.push(built_entry(i)) =~= model_entries(b, i + 1),
{
    use vstd::seq::{lemma_seq_new_len, lemma_seq_push_index_different,
        lemma_seq_push_index_same};

    broadcast use vstd::seq::lemma_seq_push_len;

    reveal(model_entries);
    assert(model_entries(b, i + 1).len() == i + 1) by {
        lemma_seq_new_len(i + 1, |j: int| built_entry(j as nat));
    }
    assert forall|j: int| 0 <= j < i + 1 implies
        entries.push(built_entry(i))[j] == model_entries(b, i + 1)[j] by {
        if j < i {
            lemma_seq_push_index_different(entries, built_entry(i), j);
        } else {
            lemma_seq_push_index_same(entries, built_entry(i), i as int);
        }
    }
}

/// A-ICNS-2's room invariant, stated as an `ensures` on the entry-list loop.
pub open spec fn room_inv(count: nat, entries: Seq<Entry>) -> bool {
    entries.len() <= count
}

/// The guard call's precondition for the entry list: `count <= MAX_ROOM` and
/// `count * size_of::<Entry>() <= isize::MAX`, the latter discharged from the
/// `global layout Entry is size == 28, align == 4` declaration in `sites.rs`.
pub proof fn lemma_room_fits(count: usize)
    requires
        count <= alloc_guard::MAX_ROOM,
    ensures
        count as nat * size_of::<Entry>() <= isize::MAX,
        1 <= size_of::<Entry>(),
{
    // Gate F2: the `global layout` declaration in `sites.rs` makes
    // `size_of::<Entry>() == 28` an axiom, but only inside that module, so the
    // numbers leave it through the exported lemma.
    crate::alloc_guard::sites::lemma_entry_layout();
    assert(count <= 4_096);
    assert(count as nat * size_of::<Entry>() <= 4_096 * 28) by (nonlinear_arith)
        requires
            count <= 4_096,
            size_of::<Entry>() == 28;
    assert(4_096 * 28 <= isize::MAX) by (nonlinear_arith)
        requires
            size_of::<Entry>() == 28;
}

pub fn checked_count(b: &[u8]) -> (n: usize)
    requires
        8 <= b.len() <= 67_108_864,
    ensures
        n == entry_count(b),
        n <= alloc_guard::MAX_ROOM,
{
    let n = (b.len() - 8) / 8;
    if n <= 4_096 {
        n
    } else {
        4_096
    }
}

pub proof fn lemma_g_p_clause3(b: &[u8])
    ensures
        forall|j: int| 0 <= j < req_p(b).len() ==> req_p(b)[j].1 <= 4_096,
{
    reveal(req_p);
    reveal(entry_count);
    if reach_p(b) {
        assert(req_p(b).len() == 1);
        assert(req_p(b)[0].1 == entry_count(b));
        assert(entry_count(b) <= 4_096);
    } else {
        assert(req_p(b).len() == 0);
    }
}

/// The header-error exit of the entry-list decoder.
pub proof fn lemma_g_p_head_error(b: &[u8], l: LogSeq)
    requires
        !reach_p(b),
        l == Seq::<(AllocSite, nat, bool)>::empty(),
    ensures
        g_p(b, Err(TryError::Format(DecodeError::InvalidImageSize)), l),
        pre_alloc_p(b),
{
    reveal(req_p);
    reveal(eq_f_p);
    reveal(fmt_part_p);
    reveal(parse_eq);
    reveal(all_ok);
    assert(req_p(b) == Seq::<(AllocSite, nat)>::empty());
    assert(req_of_log(l) =~= Seq::<(AllocSite, nat)>::empty());
    assert(all_ok(l));
}

/// The `Ok` exit: the reservation succeeded, every `push` fit in the room, and
/// the log holds one `Ok` entry.
pub proof fn lemma_g_p_ok(b: &[u8], v: Vec<Entry>, count: usize, l: LogSeq)
    requires
        reach_p(b),
        count == entry_count(b),
        v@ == model_entries(b, count as nat),
        l == Seq::<(AllocSite, nat, bool)>::empty().push((AllocSite::EntryList, count as nat, true)),
    ensures
        g_p(b, Ok::<Vec<Entry>, TryError>(v), l),
        room_inv(count as nat, v@),
{
    reveal(req_p);
    reveal(eq_f_p);
    reveal(fmt_part_p);
    reveal(parse_eq);
    reveal(all_ok);
    assert(req_of_log(l) =~= req_p(b));
    assert(all_ok(l));
}

/// The `OutOfMemory` exit: the reservation failed and the log holds one failed
/// entry equal to the returned `(site, elements)`.
pub proof fn lemma_g_p_oom(b: &[u8], o: OutOfMemory, count: usize, l: LogSeq)
    requires
        reach_p(b),
        count == entry_count(b),
        o.site_spec() == AllocSite::EntryList,
        o.elements_spec() == count,
        l == Seq::<(AllocSite, nat, bool)>::empty().push((AllocSite::EntryList, count as nat, false)),
    ensures
        g_p(b, Err::<Vec<Entry>, TryError>(TryError::OutOfMemory(o)), l),
        (o.site_spec(), o.elements_spec() as nat) == req_p(b)[0],
{
    reveal(req_p);
    assert(req_of_log(l) =~= req_p(b));
    assert(l.len() >= 1);
    assert(req_of_log(l).is_prefix_of(req_p(b)));
    assert(!l[l.len() - 1].2);
}

/// The entry-list decoder, logged. The room is fixed once; the loop pushes one
/// entry per accepted record, and `len < room` follows from the loop bound, not
/// from a regrowth.
pub fn try_parse_logged(b: &[u8]) -> (r: (Result<Vec<Entry>, TryError>, Tracked<AllocLog>))
    ensures
        g_p(b, r.0, r.1@.view()),
        match r.0 {
            Err(TryError::OutOfMemory(o)) => (o.site_spec(), o.elements_spec() as nat) == req_p(b)[0],
            _ => true,
        },
{
    let tracked mut log = alloc_guard::new_log();
    if b.len() < 8 || b.len() > 67_108_864 {
        proof {
            assert(!reach_p(b));
            lemma_g_p_head_error(b, log.view());
        }
        return (Err(TryError::Format(DecodeError::InvalidImageSize)), Tracked(log));
    }
    let count = checked_count(b);
    proof {
        assert(reach_p(b));
        assert(count == entry_count(b));
        lemma_room_fits(count);
        lemma_g_p_clause3(b);
    }
    match alloc_guard::try_with_room::<Entry>(AllocSite::EntryList, count, Tracked(&mut log)) {
        Ok(mut res) => {
            let mut i: usize = 0;
            while i < count
                invariant
                    i <= count,
                    count == entry_count(b),
                    res.view().len() == i,
                    res.room() == count as nat,
                    res.view() =~= model_entries(b, i as nat),
                decreases count - i,
            {
                // `len < room`: the ICNS `TooManyEntries` check, proved from the
                // loop bound `i < count == room`.
                let e = mk_entry(i);
                proof {
                    reveal(model_entries);
                    lemma_push_preserves_model(b, res.view(), i as nat);
                }
                res.push(e);
                i = i + 1;
            }
            let v = res.into_vec();
            proof {
                assert(v@ =~= model_entries(b, count as nat));
                lemma_g_p_ok(b, v, count, log.view());
            }
            (Ok(v), Tracked(log))
        },
        Err(o) => {
            proof {
                assert(o.site_spec() == AllocSite::EntryList);
                assert(o.elements_spec() == count);
                lemma_g_p_oom(b, o, count, log.view());
            }
            (Err(TryError::OutOfMemory(o)), Tracked(log))
        },
    }
}

/// The zero-entry case: `8 <= |b| <= 15` gives `count == 0`, and the guard
/// returns `Ok` without touching the allocator (SPEC.md §3).
pub fn try_parse_zero_entry(b: &[u8]) -> (r: Result<Vec<Entry>, TryError>)
    requires
        8 <= b.len() <= 15,
    ensures
        r is Ok,
        r->Ok_0@ == Seq::<Entry>::empty(),
{
    let tracked mut log = alloc_guard::new_log();
    let count: usize = 0;
    proof {
        lemma_room_fits(count);
    }
    let res = alloc_guard::try_with_room::<Entry>(AllocSite::EntryList, count, Tracked(&mut log));
    match res {
        Ok(r0) => {
            let v = r0.into_vec();
            proof {
                assert(v@ == Seq::<Entry>::empty());
            }
            Ok(v)
        },
        Err(_) => {
            proof { assert(false); }
            Ok(Vec::new())
        },
    }
}

} // verus!
