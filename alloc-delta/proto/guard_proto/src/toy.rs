//! Toy decoder: uses the guard for one output reservation of `4N` bytes, filled
//! by a push loop (SPEC.md §2, §4 DIB `Output`), and for one entry-list
//! reservation filled by `Reserved::push` (SPEC.md §4 ICNS `EntryList`).
//!
//! Proves theorem shape `G`, the progress theorem `Prog`, and the `PreAlloc`
//! corollary (THEOREMS-ALLOC-DELTA §0).

use vstd::prelude::*;

use crate::alloc_guard::{self, AllocLog, AllocSite, DecodeError, OutOfMemory, TryError};
use crate::alloc_guard::{all_ok, fmt_part, log_tied, req_of_log};

verus! {

/// The log's view type. Each element is `(site, elements, ok)`.
pub type LogSeq = Seq<(AllocSite, nat, bool)>;

// ---------------------------------------------------------------------------
// The frozen model and its result relation
// ---------------------------------------------------------------------------

/// The frozen model output for input `b`: `4N` zero bytes, `N = |b|`.
pub open spec fn model_out(b: &[u8]) -> Seq<u8> {
    Seq::new(4 * b.len() as nat, |_i: int| 0u8)
}

/// `Reach(b)`: the header check passes, so the entry point goes on to reserve.
/// `1 <= N` and `4N <= MAX_BYTES` is exactly the header check's arithmetic.
pub open spec fn reach(b: &[u8]) -> bool {
    1 <= b.len() && 4 * b.len() <= 67_108_864
}

/// The frozen executable-to-model predicate (SPEC.md §2, standing in for DIB's
/// `decode_mp_eq`). It is a predicate on `fmt_part` of the result, never an
/// equation between an executable and a model value.
pub open spec fn decode_eq(r: Result<Vec<u8>, DecodeError>, b: &[u8]) -> bool {
    match r {
        Ok(v) => v@ == model_out(b),
        Err(_) => !reach(b),
    }
}

/// `Eq_f(x, r)`: the frozen predicate on `fmt_part(r)`.
pub open spec fn eq_f(b: &[u8], r: Result<Vec<u8>, TryError>) -> bool {
    decode_eq(fmt_part(r), b)
}

/// `Req(x)`: the ordered requests the entry point issues when every reservation
/// succeeds (SPEC.md §2). One request, `(Output, 4N)`, iff `Reach`.
pub open spec fn req(b: &[u8]) -> Seq<(AllocSite, nat)> {
    if reach(b) {
        seq![(AllocSite::Output, 4 * b.len() as nat)]
    } else {
        Seq::empty()
    }
}

/// `PreAlloc(x)`: `Req(x) == []`.
pub open spec fn pre_alloc(b: &[u8]) -> bool {
    req(b).len() == 0
}

// ---------------------------------------------------------------------------
// Theorem shape G
// ---------------------------------------------------------------------------

/// Clause 1: a non-`OutOfMemory` result satisfies `Eq_f`, has exactly the
/// requests of `Req`, and every log entry is `Ok`.
pub open spec fn g_clause1(b: &[u8], r: Result<Vec<u8>, TryError>, l: Seq<(AllocSite, nat, bool)>) -> bool {
    match r {
        Err(TryError::OutOfMemory(_)) => true,
        Ok(_) => eq_f(b, r) && req_of_log(l) == req(b) && all_ok(l),
        Err(TryError::Format(_)) => eq_f(b, r) && req_of_log(l) == req(b) && all_ok(l),
    }
}

/// Clause 2: an `OutOfMemory` result has a non-empty log whose requests are a
/// prefix of `Req`, every entry but the last `Ok`, and the last entry not `Ok`
/// and equal to the returned `(site, elements)`.
pub open spec fn g_clause2(b: &[u8], r: Result<Vec<u8>, TryError>, l: Seq<(AllocSite, nat, bool)>) -> bool {
    match r {
        Err(TryError::OutOfMemory(o)) =>
            l.len() >= 1
            && req_of_log(l).is_prefix_of(req(b))
            && (forall|j: int| 0 <= j < l.len() - 1 ==> l[j].2)
            && !l[l.len() - 1].2
            && l[l.len() - 1].0 == o.site_spec()
            && l[l.len() - 1].1 == o.elements_spec() as nat,
        _ => true,
    }
}

/// Clause 3: every request is within its site's maximum. Proved from checked
/// values (`Reach`) before the guard call.
pub open spec fn g_clause3(b: &[u8]) -> bool {
    forall|j: int| 0 <= j < req(b).len() ==> req(b)[j].1 <= alloc_guard::MAX_BYTES
}

/// Theorem shape `G` (THEOREMS-ALLOC-DELTA §0).
pub open spec fn g(b: &[u8], r: Result<Vec<u8>, TryError>, l: Seq<(AllocSite, nat, bool)>) -> bool {
    g_clause1(b, r, l) && g_clause2(b, r, l) && g_clause3(b)
}

/// `Prog`: if every entry of the log is `Ok`, the result is not `OutOfMemory`,
/// `Eq_f` holds and `req(L) == Req(x)`.
pub open spec fn prog(b: &[u8], r: Result<Vec<u8>, TryError>, l: Seq<(AllocSite, nat, bool)>) -> bool {
    all_ok(l) ==> !matches!(r, Err(TryError::OutOfMemory(_))) && eq_f(b, r) && req_of_log(l) == req(b)
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// The one output site's request size in bytes: `4N`, within `MAX_BYTES`.
pub fn checked_n(b: &[u8]) -> (n: usize)
    requires
        1 <= b.len() <= 16_777_216,
    ensures
        n == 4 * b.len(),
        1 <= n <= alloc_guard::MAX_BYTES,
{
    4 * b.len()
}

pub proof fn lemma_g_clause3(b: &[u8])
    ensures g_clause3(b),
{
    reveal(req);
    if reach(b) {
        assert(req(b).len() == 1);
        assert(req(b)[0].1 == 4 * b.len() as nat);
        assert(4 * b.len() <= alloc_guard::MAX_BYTES);
    } else {
        assert(req(b).len() == 0);
    }
}

// ---------------------------------------------------------------------------
// `G`, `Prog` and the `PreAlloc` corollary, one lemma per exit site
// ---------------------------------------------------------------------------

/// The header-error exit: no reservation was made, so the log is empty and
/// `Req(b)` is empty.
pub proof fn lemma_g_head_error(b: &[u8], l: LogSeq)
    requires
        !reach(b),
        l == Seq::<(AllocSite, nat, bool)>::empty(),
    ensures
        g(b, Err(TryError::Format(DecodeError::TruncatedPixels)), l),
        prog(b, Err(TryError::Format(DecodeError::TruncatedPixels)), l),
        pre_alloc(b),
        log_tied(Err::<Vec<u8>, TryError>(TryError::Format(DecodeError::TruncatedPixels)), l),
{
    reveal(req);
    reveal(all_ok);
    reveal(fmt_part);
    reveal(decode_eq);
    reveal(eq_f);
    reveal(log_tied);
    lemma_g_clause3(b);
    assert(req(b) == Seq::<(AllocSite, nat)>::empty());
    assert(req_of_log(l) =~= Seq::<(AllocSite, nat)>::empty());
    assert(all_ok(l));
    assert(eq_f(b, Err::<Vec<u8>, TryError>(TryError::Format(DecodeError::TruncatedPixels))));
}

/// The `Ok` exit: the one reservation succeeded, so the log holds one `Ok`
/// entry equal to `Req(b)`.
pub proof fn lemma_g_ok(b: &[u8], v: Vec<u8>, n: usize, l: LogSeq)
    requires
        reach(b),
        n == 4 * b.len(),
        v@ == model_out(b),
        l == Seq::<(AllocSite, nat, bool)>::empty().push((AllocSite::Output, n as nat, true)),
    ensures
        g(b, Ok::<Vec<u8>, TryError>(v), l),
        prog(b, Ok::<Vec<u8>, TryError>(v), l),
        log_tied(Ok::<Vec<u8>, TryError>(v), l),
{
    reveal(req);
    lemma_g_clause3(b);
    assert(req_of_log(l) =~= req(b));
    assert(all_ok(l));
}

/// The `OutOfMemory` exit: the one reservation failed, so the log holds one
/// failed entry, whose `(site, elements)` is the returned one and the requested
/// one.
pub proof fn lemma_g_oom(b: &[u8], o: OutOfMemory, n: usize, l: LogSeq)
    requires
        reach(b),
        n == 4 * b.len(),
        o.site_spec() == AllocSite::Output,
        o.elements_spec() == n,
        l == Seq::<(AllocSite, nat, bool)>::empty().push((AllocSite::Output, n as nat, false)),
    ensures
        g(b, Err::<Vec<u8>, TryError>(TryError::OutOfMemory(o)), l),
        prog(b, Err::<Vec<u8>, TryError>(TryError::OutOfMemory(o)), l),
        (o.site_spec(), o.elements_spec() as nat) == req(b)[0],
        log_tied(Err::<Vec<u8>, TryError>(TryError::OutOfMemory(o)), l),
{
    reveal(req);
    lemma_g_clause3(b);
    assert(req_of_log(l) =~= req(b));
    assert(l.len() >= 1);
    assert(req_of_log(l).is_prefix_of(req(b)));
    assert(!l[l.len() - 1].2);
    assert(!all_ok(l));
}


// ---------------------------------------------------------------------------
// The toy decoder, logged
// ---------------------------------------------------------------------------

/// The toy decoder, logged: creates the log, makes at most one reservation.
///
/// Reachability is exactly `Reach(b)`: the header check passes and the guard is
/// called; otherwise the header error returns before any reservation.
pub fn try_decode_logged(b: &[u8]) -> (r: (Result<Vec<u8>, TryError>, Tracked<AllocLog>))
    ensures
        g(b, r.0, r.1@.view()),
        prog(b, r.0, r.1@.view()),
        // A-DIB-8: the log-only part of clauses 1-2, exported for ICO.
        log_tied(r.0, r.1@.view()),
        pre_alloc(b) ==> !matches!(r.0, Err(TryError::OutOfMemory(_))) && eq_f(b, r.0)
            && r.1@.view().len() == 0,
        match r.0 {
            Err(TryError::OutOfMemory(o)) => (o.site_spec(), o.elements_spec() as nat) == req(b)[0],
            _ => true,
        },
{
    let tracked mut log = alloc_guard::new_log();
    // The header check: `Reach(b)` is exactly this predicate.
    if b.len() < 1 || b.len() > 16_777_216 {
        proof {
            assert(!reach(b));
            lemma_g_head_error(b, log.view());
        }
        return (Err(TryError::Format(DecodeError::TruncatedPixels)), Tracked(log));
    }
    let n = checked_n(b);
    proof {
        assert(reach(b));
    }
    match alloc_guard::try_zeroed(AllocSite::Output, n, Tracked(&mut log)) {
        Ok(v) => {
            proof {
                assert(v@ == Seq::new(n as nat, |_i: int| 0u8));
                assert(n == 4 * b.len());
                assert(v@ =~= model_out(b));
                lemma_g_ok(b, v, n, log.view());
            }
            (Ok(v), Tracked(log))
        },
        Err(o) => {
            proof {
                assert(o.site_spec() == AllocSite::Output);
                assert(o.elements_spec() == n);
                assert(n == 4 * b.len());
                lemma_g_oom(b, o, n, log.view());
            }
            (Err(TryError::OutOfMemory(o)), Tracked(log))
        },
    }
}

/// The public `try_*`: the logged function with the log discarded.
pub fn try_decode(b: &[u8]) -> (r: Result<Vec<u8>, TryError>)
    ensures
        !matches!(r, Err(TryError::OutOfMemory(_))) ==> eq_f(b, r),
        match r {
            Err(TryError::OutOfMemory(o)) => (o.site_spec(), o.elements_spec() as nat) == req(b)[0],
            _ => true,
        },
{
    let (r, _log) = try_decode_logged(b);
    r
}

} // verus!
