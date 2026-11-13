//! ICO's per-core site declarations, including `import_dib` (SPEC.md §3.1).
//!
//! `import_dib` is the one guard function that appends a whole imported DIB log.
//! It lives in `sites.rs` for the same reason as in DIB's guard: only a module
//! that can name ICO's `AllocLog` private field may write it. The shape part of
//! its contract (A-ICO-2) is proved here too, because the `OutOfMemory` it
//! returns can only be constructed in this module.

use vstd::prelude::*;

use guard_proto::alloc_guard::{AllocSite as DibSite, AllocLog as DibLog, TryError as DibTryError};
use guard_proto::toy as dib_toy;

use super::{AllocLog, IcoError, OutOfMemory, TryError};
use crate::ico::{g_ico, prog_ico, req_d, eq_d, map_err, LogSeq};

verus! {

/// ICO's site enum: `EntryList | Dib(dib::AllocSite)` (SPEC.md §4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ICOSite {
    EntryList,
    Dib(DibSite),
}

/// `map_site` (SPEC.md §3.1): rename DIB's sites into ICO's.
pub open spec fn map_site(s: DibSite) -> ICOSite {
    ICOSite::Dib(s)
}

/// `map_dib` on full log entries.
pub open spec fn map_dib(s: Seq<(DibSite, nat, bool)>) -> Seq<(ICOSite, nat, bool)> {
    Seq::new(s.len(), |j: int| (map_site(s[j].0), s[j].1, s[j].2))
}

/// `map_dib` on the request list, with the `ok` flag already dropped.
pub open spec fn map_dib_req(s: Seq<(DibSite, nat)>) -> Seq<(ICOSite, nat)> {
    Seq::new(s.len(), |j: int| (map_site(s[j].0), s[j].1))
}

/// `req(L)` for DIB's element type.
pub open spec fn req_of_log_dib(l: Seq<(DibSite, nat, bool)>) -> Seq<(DibSite, nat)> {
    Seq::new(l.len(), |j: int| (l[j].0, l[j].1))
}

/// `req(L)` for ICO's element type.
pub open spec fn req_of_log(l: Seq<(ICOSite, nat, bool)>) -> Seq<(ICOSite, nat)> {
    Seq::new(l.len(), |j: int| (l[j].0, l[j].1))
}

/// Every entry of `l` is `Ok`, for ICO's site type.
pub open spec fn all_ok(l: Seq<(ICOSite, nat, bool)>) -> bool {
    forall|j: int| 0 <= j < l.len() ==> l[j].2
}

/// Is this site DIB's `Output` reservation, as seen from ICO?
pub open spec fn is_dib_output(s: ICOSite) -> bool {
    match s {
        ICOSite::EntryList => false,
        ICOSite::Dib(d) => d == DibSite::Output,
    }
}

/// Exec counterpart of `map_site`, used to build the wrapped `OutOfMemory`.
pub fn map_site_exec(s: DibSite) -> (r: ICOSite)
    ensures r == map_site(s),
{
    ICOSite::Dib(s)
}

/// Exec counterpart of `map_err`, used by `import_dib`'s `Format` arm.
pub fn map_err_exec(e: guard_proto::alloc_guard::DecodeError) -> (r: IcoError)
    ensures r == map_err(e),
{
    match e {
        guard_proto::alloc_guard::DecodeError::TruncatedPixels => IcoError::TruncatedDirectory,
        guard_proto::alloc_guard::DecodeError::InvalidImageSize => IcoError::InvalidKind,
        guard_proto::alloc_guard::DecodeError::RleTruncated => IcoError::TruncatedDirectory,
    }
}

// ---------------------------------------------------------------------------
// Facts about map_dib: A-ICO-2's steps
// ---------------------------------------------------------------------------

/// Requests commute with the site mapping.
pub proof fn lemma_req_of_map_dib(dl: Seq<(DibSite, nat, bool)>)
    ensures
        req_of_log(map_dib(dl)) == map_dib_req(req_of_log_dib(dl)),
{
    reveal(req_of_log);
    reveal(req_of_log_dib);
    reveal(map_dib);
    reveal(map_dib_req);
    assert(req_of_log(map_dib(dl)) =~= map_dib_req(req_of_log_dib(dl)));
}

/// Prefix is preserved, which carries clause 2's "prefix of `Req`" across.
pub proof fn lemma_map_dib_req_prefix(a: Seq<(DibSite, nat)>, whole: Seq<(DibSite, nat)>)
    requires a.is_prefix_of(whole),
    ensures map_dib_req(a).is_prefix_of(map_dib_req(whole)),
{
    reveal(map_dib_req);
    assert(map_dib_req(a).len() == a.len());
    assert(map_dib_req(whole).len() == whole.len());
    assert forall|j: int| 0 <= j < map_dib_req(a).len() implies
        map_dib_req(a)[j] == map_dib_req(whole)[j] by {
        assert(a[j] == whole[j]);
    }
}

/// The `ok` flag is untouched, which carries `all_ok` and the failed-last-entry
/// facts across.
pub proof fn lemma_all_ok_map_dib(dl: Seq<(DibSite, nat, bool)>)
    ensures
        all_ok(map_dib(dl)) == guard_proto::alloc_guard::all_ok(dl),
{
    reveal(all_ok);
    reveal(guard_proto::alloc_guard::all_ok);
    reveal(map_dib);
    assert forall|j: int| 0 <= j < map_dib(dl).len() implies map_dib(dl)[j].2 == dl[j].2 by {};
}

/// The last entry of `map_dib(dl)` is DIB's last entry with its site renamed.
pub proof fn lemma_map_dib_last(dl: Seq<(DibSite, nat, bool)>)
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
// import_dib
// ---------------------------------------------------------------------------

/// The `reveal` set the three arm lemmas of A-ICO-2 need, in one place.
proof fn reveal_g_ico()
    ensures true,
{
    reveal(g_ico);
    reveal(prog_ico);
    reveal(req_d);
    reveal(req_of_log);
    reveal(eq_d);
    reveal(all_ok);
    reveal(map_err);
    reveal(map_dib);
    reveal(map_dib_req);
    reveal(map_site);
    reveal(req_of_log_dib);
    reveal(dib_toy::g);
    reveal(dib_toy::g_clause1);
    reveal(dib_toy::g_clause2);
    reveal(dib_toy::eq_f);
    reveal(dib_toy::decode_eq);
    reveal(dib_toy::req);
    reveal(dib_toy::model_out);
    reveal(guard_proto::alloc_guard::fmt_part);
}

/// A-ICO-2 for DIB's `Ok` arm.
pub proof fn lemma_g_ico_ok(b: &[u8], d: Vec<u8>, dl: Seq<(DibSite, nat, bool)>, il: LogSeq)
    requires
        dib_toy::g(b, Ok::<Vec<u8>, DibTryError>(d), dl),
        il == map_dib(dl),
    ensures
        g_ico(b, Ok::<Vec<u8>, TryError>(d), il),
        prog_ico(b, Ok::<Vec<u8>, TryError>(d), il),
{
    reveal_g_ico();
    assert(dib_toy::g_clause1(b, Ok::<Vec<u8>, DibTryError>(d), dl));
    assert(dib_toy::eq_f(b, Ok::<Vec<u8>, DibTryError>(d)));
    assert(dib_toy::decode_eq(
        guard_proto::alloc_guard::fmt_part(Ok::<Vec<u8>, DibTryError>(d)), b));
    assert(d@ == dib_toy::model_out(b));
    assert(eq_d(b, Ok::<Vec<u8>, TryError>(d)));
    assert(req_of_log_dib(dl) =~= dib_toy::req(b));
    lemma_req_of_map_dib(dl);
    assert(req_of_log(il) =~= map_dib_req(req_of_log_dib(dl)));
    assert(req_of_log(il) =~= req_d(b));
    lemma_all_ok_map_dib(dl);
    assert(all_ok(il));
}

/// A-ICO-2 for DIB's `Format` arm.
pub proof fn lemma_g_ico_format(
    b: &[u8],
    e: guard_proto::alloc_guard::DecodeError,
    dl: Seq<(DibSite, nat, bool)>,
    il: LogSeq,
)
    requires
        dib_toy::g(b, Err::<Vec<u8>, DibTryError>(DibTryError::Format(e)), dl),
        il == map_dib(dl),
    ensures
        g_ico(b, Err::<Vec<u8>, TryError>(TryError::Format(map_err(e))), il),
        prog_ico(b, Err::<Vec<u8>, TryError>(TryError::Format(map_err(e))), il),
{
    reveal_g_ico();
    assert(dib_toy::g_clause1(b, Err::<Vec<u8>, DibTryError>(DibTryError::Format(e)), dl));
    assert(dib_toy::eq_f(b, Err::<Vec<u8>, DibTryError>(DibTryError::Format(e))));
    assert(eq_d(b, Err::<Vec<u8>, TryError>(TryError::Format(map_err(e)))));
    assert(req_of_log_dib(dl) =~= dib_toy::req(b));
    lemma_req_of_map_dib(dl);
    assert(req_of_log(il) =~= req_d(b));
    lemma_all_ok_map_dib(dl);
    assert(all_ok(il));
}

/// A-ICO-2 for DIB's `OutOfMemory` arm. The wrapped value is exactly
/// `(Dib(o.site()), o.elements())`, and ICO's last entry is DIB's last entry
/// renamed.
pub proof fn lemma_g_ico_oom(
    b: &[u8],
    o: guard_proto::alloc_guard::OutOfMemory,
    dl: Seq<(DibSite, nat, bool)>,
    il: LogSeq,
    out: Result<Vec<u8>, TryError>,
)
    requires
        dib_toy::g(b, Err::<Vec<u8>, DibTryError>(DibTryError::OutOfMemory(o)), dl),
        il == map_dib(dl),
        // The returned value's shape, stated through the accessors rather than a
        // constructor: an `OutOfMemory` literal may appear only in this module,
        // and this is that module.
        matches!(out, Err(TryError::OutOfMemory(_))),
        match out {
            Err(TryError::OutOfMemory(o2)) =>
                o2.site_spec() == map_site(o.site_spec())
                && o2.elements_spec() == o.elements_spec(),
            _ => false,
        },
    ensures
        g_ico(b, out, il),
        prog_ico(b, out, il),
{
    reveal_g_ico();
    assert(dib_toy::g_clause2(b, Err::<Vec<u8>, DibTryError>(DibTryError::OutOfMemory(o)), dl));
    assert(dl.len() >= 1);
    lemma_map_dib_last(dl);
    assert(il.len() >= 1);
    assert(il[il.len() - 1].0 == map_site(o.site_spec()));
    assert(il[il.len() - 1].1 == o.elements_spec() as nat);
    assert(!il[il.len() - 1].2);
    lemma_all_ok_map_dib(dl);
    assert(!all_ok(il));
    lemma_req_of_map_dib(dl);
    assert(req_of_log(il) =~= map_dib_req(req_of_log_dib(dl)));
    assert(req_of_log_dib(dl) =~= dib_toy::req(b));
    lemma_map_dib_req_prefix(req_of_log_dib(dl), dib_toy::req(b));
}

/// `import_dib` (SPEC.md §3.1). Consumes DIB's returned log, appends its events
/// to ICO's log through `map_dib`, and wraps a DIB `OutOfMemory`.
pub fn import_dib(
    b: &[u8],
    r: Result<Vec<u8>, DibTryError>,
    Tracked(dlog): Tracked<DibLog>,
    Tracked(log): Tracked<&mut AllocLog>,
) -> (out: Result<Vec<u8>, TryError>)
    requires
        old(log).view() == Seq::<(ICOSite, nat, bool)>::empty(),
        guard_proto::alloc_guard::log_tied(r, dlog.view()),
        // DIB's exported graceful-failure shape on the same result and log.
        // `try_decode_with_steps_logged` (A-DIB-8) provides it.
        dib_toy::g(b, r, dlog.view()),
    ensures
        final(log).view() == map_dib(dlog.view()),
        match r {
            Ok(d) => out == Ok(d),
            Err(DibTryError::Format(e)) => out == Err(TryError::Format(map_err(e))),
            Err(DibTryError::OutOfMemory(o)) =>
                match out {
                    Err(TryError::OutOfMemory(o2)) =>
                        o2.site_spec() == map_site(o.site_spec())
                        && o2.elements_spec() == o.elements_spec()
                        && final(log).view().last()
                            == (map_site(o.site_spec()), o.elements_spec() as nat, false),
                    _ => false,
                },
        },
        // A-ICO-2, stated over the returned value and the final log only, so the
        // caller can use it after `r` and `dlog` are consumed (they are linear).
        g_ico(b, out, final(log).view()),
        prog_ico(b, out, final(log).view()),
{
    proof { log.ev = map_dib(dlog.view()); }
    match r {
        Ok(d) => {
            proof { lemma_g_ico_ok(b, d, dlog.view(), log@); }
            Ok(d)
        },
        Err(DibTryError::Format(e)) => {
            proof { lemma_g_ico_format(b, e, dlog.view(), log@); }
            Err(TryError::Format(map_err_exec(e)))
        },
        Err(DibTryError::OutOfMemory(o)) => {
            let out = Err(TryError::OutOfMemory(OutOfMemory {
                site: map_site_exec(o.site()),
                elements: o.elements(),
            }));
            proof { lemma_g_ico_oom(b, o, dlog.view(), log@, out); }
            out
        },
    }
}

} // verus!
