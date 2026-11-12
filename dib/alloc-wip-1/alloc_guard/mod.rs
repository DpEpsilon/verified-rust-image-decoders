//! alloc_guard: the sealed reservation module (alloc-delta SPEC.md §3).
//!
//! This is the production copy for `dib_verus`, adapted from the verified
//! prototype `alloc-delta/proto/guard_proto/src/alloc_guard/mod.rs`. Two
//! adaptations:
//!
//! - DIB has exactly one reservation site, `AllocSite::Output` (4N zero bytes),
//!   so the prototype's `Reserved<T>`, `try_copy`, `try_with_room` and
//!   `MAX_ROOM` are omitted; `try_zeroed` keeps the prototype's exact body.
//! - The prototype's toy `DecodeError` and the `TryError` / `fmt_part` /
//!   `log_tied` items that name the core's frozen error type live in the
//!   per-core child module `sites` instead, here in DIB's own `dib.rs`. Without
//!   that move this file would have to name `crate::dib::Error` and could not be
//!   byte-identical across the four cores (SPEC.md §3, decision D6).
//!
//! `#[inline(never)]` is on the exec function, so the guard's growth path is a
//! distinct function in the shipped binary (SPEC.md §6, item 4).
//!
//! The log is sealed: `AllocLog`'s only field is `ghost ev`, private to this
//! module (Rust privacy, which Verus enforces in ghost and proof code as well),
//! so only `new_log`, `try_zeroed` here and the child module `sites` can produce
//! or extend a log. `OutOfMemory`'s fields are private to the same module, so
//! only a `try_reserve_exact` `Err` arm can construct one.
//!
//! Trust item: A-STD-1/2/3 (alloc-delta SPEC.md §3). The guard body is the
//! binding point between the returned `Result` and vstd's unconstrained
//! `try_reserve_exact` spec; that binding is hash-pinned text, not an `ensures`.

use vstd::prelude::*;

pub mod sites;

pub use sites::{AllocSite, TryError};

verus! {

/// Upper bound on a single byte-oriented reservation (SPEC.md §4). 67,108,864 B.
pub const MAX_BYTES: usize = 67_108_864;

/// The sealed logical reservation trace (SPEC.md §2, §3). One field, private to
/// this module. Each element is `(site, elements, ok)`; `ok == (r is Ok)` for
/// the guard call that appended it.
pub tracked struct AllocLog {
    ghost ev: Seq<(AllocSite, nat, bool)>,
}

impl AllocLog {
    pub closed spec fn view(self) -> Seq<(AllocSite, nat, bool)> {
        self.ev
    }
}

/// The only way to make a log. It is a `proof fn`, not an `exec fn`: `new_log`
/// is called once at the entry of each `*_logged` function, in ghost code.
pub proof fn new_log() -> (tracked l: AllocLog)
    ensures l.view() == Seq::<(AllocSite, nat, bool)>::empty(),
{
    AllocLog { ev: Seq::empty() }
}

/// A reservation failure. Constructed only by `try_zeroed`, from the `Err` arm
/// of `try_reserve_exact`, and by ICO's `import_dib`. Fields private to this
/// module (and its child `sites`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutOfMemory {
    site: AllocSite,
    elements: usize,
}

impl OutOfMemory {
    /// `#[verifier::when_used_as_spec]` so `ensures` clauses in this and other
    /// crates can name the accessors. A caller outside `alloc_guard` writes
    /// `o.site()`; in spec position that resolves to the sibling `site_spec`,
    /// whose body is in this `impl` and may read the private field.
    ///
    /// The spec counterparts must be `closed`, not `open`: an `open` body would
    /// have to be well-formed everywhere, including outside `alloc_guard`, where
    /// the field is private. `closed` keeps the body inside the module, which is
    /// exactly the sealing SPEC.md §2 asks for. The parameters must match the
    /// exec accessor's receiver (`&self`), as `#[verifier::when_used_as_spec]`
    /// requires (gate F3).
    #[verifier::when_used_as_spec(site_spec)]
    pub fn site(&self) -> (s: AllocSite)
        ensures s == Self::site_spec(self),
    {
        self.site
    }

    #[verifier::when_used_as_spec(elements_spec)]
    pub fn elements(&self) -> (n: usize)
        ensures n == Self::elements_spec(self),
    {
        self.elements
    }

    pub closed spec fn site_spec(&self) -> AllocSite {
        self.site
    }

    pub closed spec fn elements_spec(&self) -> usize {
        self.elements
    }
}

/// Every entry of `l` is `Ok`.
pub open spec fn all_ok(l: Seq<(AllocSite, nat, bool)>) -> bool {
    forall|j: int| 0 <= j < l.len() ==> l[j].2
}

/// `l` with the `ok` flag dropped.
pub open spec fn req_of_log(l: Seq<(AllocSite, nat, bool)>) -> Seq<(AllocSite, nat)> {
    Seq::new(l.len(), |j: int| (l[j].0, l[j].1))
}

/// Reserve `n` zero bytes for `site`. `n >= 1`; a zero request is the caller's
/// job to skip. The reservation is `try_reserve_exact(n)` on an empty `Vec`, so
/// on `Ok` the capacity is at least `n` (A-STD-1) and the push loop below never
/// reallocates (A-STD-2); on `Err` the only constructor of `OutOfMemory` builds
/// the value returned here. Either way exactly one log entry is appended, with
/// `ok == (r is Ok)`.
#[inline(never)]
pub fn try_zeroed(site: AllocSite, n: usize, Tracked(log): Tracked<&mut AllocLog>)
    -> (r: Result<Vec<u8>, OutOfMemory>)
    requires
        1 <= n <= MAX_BYTES,
    ensures
        final(log).view() == old(log).view().push((site, n as nat, r is Ok)),
        r is Ok ==> r->Ok_0@ == Seq::new(n as nat, |_i: int| 0u8),
        r is Err ==> r->Err_0.site_spec() == site && r->Err_0.elements_spec() == n,
{
    let mut v: Vec<u8> = Vec::new();
    match v.try_reserve_exact(n) {
        Ok(()) => {
            let mut i: usize = 0;
            while i < n
                invariant
                    0 <= i <= n,
                    v@ == Seq::new(i as nat, |_j: int| 0u8),
                decreases n - i,
            {
                v.push(0u8);
                i = i + 1;
            }
            proof { log.ev = log.ev.push((site, n as nat, true)); }
            Ok(v)
        },
        Err(_) => {
            proof { log.ev = log.ev.push((site, n as nat, false)); }
            Err(OutOfMemory { site, elements: n })
        },
    }
}

} // verus!
