//! NEGATIVE CHECK 3 (must FAIL to verify).
//!
//! Expectation: every exit satisfies `G` except one, which returns an
//! `OutOfMemory` stashed from a reservation made against a *different* log while
//! the log it returns has an `Ok` last entry. `G` clause 2 needs `¬L.last().ok`
//! and the last entry to match the returned value, so that exit fails and the
//! others pass. This is mutation A17 ("return `OutOfMemory` after the guard
//! returned `Ok`").
//!
//! Failure mode expected: a Verus postcondition error on `g` at the stashed-OOM
//! exit.

use vstd::prelude::*;

use crate::alloc_guard::{self, AllocLog, AllocSite, OutOfMemory, TryError};
use crate::toy::*;

verus! {

pub fn try_decode_logged_bad(b: &[u8]) -> (r: (Result<Vec<u8>, TryError>, Tracked<AllocLog>))
    requires
        1 <= b.len() <= 16_777_216,
    ensures
        g(b, r.0, r.1@.view()),
{
    // A second log, used only to obtain an `OutOfMemory` value to stash. The
    // only constructor outside `alloc_guard` is a guard `Err`.
    let tracked mut stash_log = alloc_guard::new_log();
    proof { crate::alloc_guard::sites::lemma_entry_layout(); }
    let stash = alloc_guard::try_with_room::<crate::alloc_guard::Entry>(
        AllocSite::EntryList, 4_096, Tracked(&mut stash_log));
    // The log this function returns and reports on.
    let tracked mut log = alloc_guard::new_log();
    proof {
        assert(reach(b));
        lemma_g_clause3(b);
    }
    let n = checked_n(b);
    match alloc_guard::try_zeroed(AllocSite::Output, n, Tracked(&mut log)) {
        Ok(v) => {
            match stash {
                // Mutation: return the stashed `OutOfMemory` although the log's
                // only (hence last) entry is `Ok`. Clause 2 fails here.
                Err(o) => {
                    let _ = o;
                    let o2: OutOfMemory = o;
                    (Err(TryError::OutOfMemory(o2)), Tracked(log))
                },
                Ok(_) => {
                    proof {
                        assert(v@ =~= model_out(b));
                        lemma_g_ok(b, v, n, log.view());
                    }
                    (Ok(v), Tracked(log))
                },
            }
        },
        Err(o) => {
            proof {
                assert(o.site_spec() == AllocSite::Output);
                assert(o.elements_spec() == n);
                lemma_g_oom(b, o, n, log.view());
            }
            (Err(TryError::OutOfMemory(o)), Tracked(log))
        },
    }
}

} // verus!
