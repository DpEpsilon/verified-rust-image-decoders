//! NEGATIVE CHECK 4 (crate `guard_proto_dep`; must FAIL to verify).
//!
//! Expectation: Verus rejects this because ICO's log would stay empty while
//! `Req_D` is non-empty, so A-ICO-2 clause 1 (`req(L) == Req_D`) fails; and with
//! a DIB `OutOfMemory` there is no constructor for ICO's own `OutOfMemory` at
//! all. This is mutation A22.
//!
//! Failure mode expected: a Verus postcondition error on `g_ico`/`prog_ico`, or a
//! rustc error if the author tries to build the ICO `OutOfMemory` directly.

use vstd::prelude::*;

use guard_proto::toy as dib_toy;
use guard_proto::alloc_guard::TryError as DibTryError;

use crate::alloc_guard::{self as ico_guard, AllocLog, TryError};
use crate::ico::{g_ico, prog_ico};

verus! {

/// Drop `import_dib`: pass DIB's result through with ICO's log left empty.
pub fn try_decode_entry_bad(b: &[u8]) -> (r: (Result<Vec<u8>, TryError>, Tracked<AllocLog>))
    ensures
        g_ico(b, r.0, r.1@.view()),
        prog_ico(b, r.0, r.1@.view()),
{
    let tracked log = ico_guard::new_log();
    let (r0, dlog) = dib_toy::try_decode_logged(b);
    // No `import_dib`: read DIB's result and hand back an ICO-shaped one.
    let out = match r0 {
        Ok(d) => Ok(d),
        // Nothing here can build an ICO `OutOfMemory`, which is the second half
        // of the expectation.
        Err(DibTryError::Format(_)) => Err(TryError::Format(crate::alloc_guard::IcoError::TruncatedDirectory)),
        Err(DibTryError::OutOfMemory(_)) => {
            Err(TryError::Format(crate::alloc_guard::IcoError::InvalidKind))
        },
    };
    (out, Tracked(log))
}

} // verus!
