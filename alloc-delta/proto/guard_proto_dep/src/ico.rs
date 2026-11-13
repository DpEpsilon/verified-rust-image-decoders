//! A toy ICO decode path that reserves nothing of its own and reaches DIB's
//! `Output` reservation on the selected slice, then imports DIB's log. It
//! states A-ICO-2's graceful-failure shape across the crate boundary.

use vstd::prelude::*;

use guard_proto::alloc_guard::{self as dib_guard, AllocSite as DibSite, TryError as DibTryError};
use guard_proto::toy::{self as dib_toy};

use crate::alloc_guard::{self as ico_guard, AllocLog, IcoError, TryError};
use crate::alloc_guard::sites::{ICOSite, all_ok, is_dib_output, map_dib, map_dib_req, map_site, req_of_log, req_of_log_dib};

verus! {

pub type LogSeq = Seq<(ICOSite, nat, bool)>;

/// DIB's `fmt_part`, re-exported so this crate can name the frozen predicate's
/// argument.
pub open spec fn fmt_part_dib(r: Result<Vec<u8>, DibTryError>) -> Result<Vec<u8>, guard_proto::alloc_guard::DecodeError> {
    guard_proto::alloc_guard::fmt_part(r)
}

/// `Req_D`: `[]` until `MD` reaches the DIB call, then DIB's `Req` on the
/// selected slice with `Output ↦ Dib(Output)` (SPEC.md §4, THEOREMS-ALLOC-DELTA
/// A-ICO-2).
pub open spec fn req_d(b: &[u8]) -> Seq<(ICOSite, nat)> {
    if dib_toy::reach(b) {
        Seq::empty().push((ICOSite::Dib(DibSite::Output), 4 * b.len() as nat))
    } else {
        Seq::empty()
    }
}

/// `map_dib` of DIB's `Req`: the identity that A-ICO-2 clause 1 uses to turn
/// DIB's request list into ICO's.
pub proof fn lemma_map_dib_req(b: &[u8])
    ensures req_d(b) == map_dib_req(dib_toy::req(b)),
{
    reveal(req_d);
    reveal(dib_toy::req);
    reveal(map_dib_req);
    reveal(map_site);
    if dib_toy::reach(b) {
        assert(map_dib_req(dib_toy::req(b)) =~=
            Seq::empty().push((ICOSite::Dib(DibSite::Output), 4 * b.len() as nat)));
    }
}

/// `Eq_D`: the frozen ICO predicate (SPEC.md §2, standing in for `md_eq`). The
/// toy content is DIB's model output.
pub open spec fn eq_d(b: &[u8], r: Result<Vec<u8>, TryError>) -> bool {
    match ico_guard::fmt_part(r) {
        Ok(v) => v@ == dib_toy::model_out(b),
        Err(_) => !dib_toy::reach(b),
    }
}

/// A-ICO-2, clause 1 and clause 2, plus `Prog` for the ICO entry point.
pub open spec fn g_ico(b: &[u8], r: Result<Vec<u8>, TryError>, l: LogSeq) -> bool {
    match r {
        Err(TryError::OutOfMemory(o)) =>
            l.len() >= 1
            && req_of_log(l).is_prefix_of(req_d(b))
            && (forall|j: int| 0 <= j < l.len() - 1 ==> l[j].2)
            && !l[l.len() - 1].2
            && l[l.len() - 1].0 == o.site_spec()
            && l[l.len() - 1].1 == o.elements_spec() as nat,
        _ => eq_d(b, r) && req_of_log(l) == req_d(b) && all_ok(l),
    }
}

pub open spec fn prog_ico_conj0(b: &[u8], r: Result<Vec<u8>, TryError>, l: LogSeq) -> bool {
    all_ok(l) ==> !matches!(r, Err(TryError::OutOfMemory(_)))
}
pub open spec fn prog_ico_conj1(b: &[u8], r: Result<Vec<u8>, TryError>, l: LogSeq) -> bool {
    all_ok(l) ==> eq_d(b, r)
}
pub open spec fn prog_ico_conj2(b: &[u8], r: Result<Vec<u8>, TryError>, l: LogSeq) -> bool {
    all_ok(l) ==> req_of_log(l) == req_d(b)
}
pub open spec fn prog_ico_conj3(b: &[u8], r: Result<Vec<u8>, TryError>, l: LogSeq) -> bool {
    match r {
        Err(TryError::OutOfMemory(_)) =>
            l.len() >= 1 && req_of_log(l).is_prefix_of(req_d(b)),
        _ => true,
    }
}

pub open spec fn prog_ico(b: &[u8], r: Result<Vec<u8>, TryError>, l: LogSeq) -> bool {
    all_ok(l)
        ==> !matches!(r, Err(TryError::OutOfMemory(_))) && eq_d(b, r) && req_of_log(l) == req_d(b)
}

/// The frozen DIB error carried into ICO's error enum, the analogue of ICO's
/// `Error::Dib(_)` wrapper.
pub open spec fn map_err(e: guard_proto::alloc_guard::DecodeError) -> IcoError {
    match e {
        guard_proto::alloc_guard::DecodeError::TruncatedPixels => IcoError::TruncatedDirectory,
        guard_proto::alloc_guard::DecodeError::InvalidImageSize => IcoError::InvalidKind,
        guard_proto::alloc_guard::DecodeError::RleTruncated => IcoError::TruncatedDirectory,
    }
}

/// The `OutOfMemory` site of an ICO decode result is DIB's `Output`. `Req_D` has
/// at most one element, `(Dib(Output), 4N)`, so a non-empty prefix of it is that
/// element, and clause 2 pins the returned site to the log's last entry.
pub proof fn lemma_oom_site_is_dib(b: &[u8], r: Result<Vec<u8>, TryError>, l: LogSeq)
    requires g_ico(b, r, l),
    ensures
        match r {
            Err(TryError::OutOfMemory(o)) => is_dib_output(o.site_spec()),
            _ => true,
        },
{
    reveal(g_ico);
    reveal(req_d);
    reveal(req_of_log);
    reveal(is_dib_output);
    reveal(map_site);
    if let Err(TryError::OutOfMemory(o)) = r {
        assert(l.len() >= 1);
        assert(req_of_log(l).is_prefix_of(req_d(b)));
        assert(req_of_log(l).len() == l.len());
        assert(req_d(b).len() >= 1);
        assert(dib_toy::reach(b));
        assert(req_d(b) =~= Seq::empty().push((ICOSite::Dib(DibSite::Output), 4 * b.len() as nat)));
        assert(req_of_log(l).len() <= req_d(b).len());
        assert(l.len() <= 1);
        assert(l.len() == 1);
        assert(l[l.len() - 1].0 == o.site_spec());
        assert(req_of_log(l)[l.len() - 1] == req_d(b)[l.len() - 1]);
        assert(o.site_spec() == ICOSite::Dib(DibSite::Output));
    }
}

pub fn try_decode_entry_logged(b: &[u8]) -> (r: (Result<Vec<u8>, TryError>, Tracked<AllocLog>))
    ensures
        g_ico(b, r.0, r.1@.view()),
        prog_ico(b, r.0, r.1@.view()),
{
    let tracked mut ico_log = ico_guard::new_log();
    // The DIB call: `dib_verus::try_decode_with_steps_logged` at `ico.rs:1192`.
    let (r0, dlog) = dib_toy::try_decode_logged(b);
    proof {
        lemma_map_dib_req(b);
        // A-DIB-8 gives `log_tied(r0, dlog)`, which is `import_dib`'s `requires`,
        // and `try_decode_logged` also gives DIB's `G` on the same result and log.
        assert(dib_guard::log_tied(r0, dlog@.view()));
        assert(dib_toy::g(b, r0, dlog@.view()));
        assert(ico_log.view() == Seq::<(ICOSite, nat, bool)>::empty());
    }
    // `import_dib` consumes `r0` and the tracked DIB log, writes ICO's log, and
    // wraps a DIB `OutOfMemory`. It is the only route from a DIB result to an
    // ICO result.
    let out = crate::alloc_guard::sites::import_dib(b, r0, dlog, Tracked(&mut ico_log));
    (out, Tracked(ico_log))
}

/// A-ICO-2's public corollary: on `OutOfMemory` the returned site is
/// `Dib(Output)`, and on a non-`OutOfMemory` result `Eq_D` holds.
pub fn try_decode_entry(b: &[u8]) -> (r: Result<Vec<u8>, TryError>)
    ensures
        !matches!(r, Err(TryError::OutOfMemory(_))) ==> eq_d(b, r),
        match r {
            Err(TryError::OutOfMemory(o)) => is_dib_output(o.site_spec()),
            _ => true,
        },
{
    let (r, log) = try_decode_entry_logged(b);
    proof {
        assert(g_ico(b, r, log@.view()));
        lemma_oom_site_is_dib(b, r, log@.view());
    }
    r
}

} // verus!
