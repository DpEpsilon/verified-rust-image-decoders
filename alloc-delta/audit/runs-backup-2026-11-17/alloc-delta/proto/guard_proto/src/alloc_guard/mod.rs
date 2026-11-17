//! alloc_guard: the sealed reservation module (SPEC.md §3).
//!
//! Text-identical in every core in the real delta; here it is generic over the
//! per-core `AllocSite` declared in the child module `sites`.
//!
//! The log is sealed: `AllocLog`'s only field is `ghost ev`, private to this
//! module (Rust privacy, which Verus enforces in ghost and proof code as well),
//! so only `new_log` and the reservation functions in this file and its child
//! module can produce or extend a log. `OutOfMemory`'s fields are private to the
//! same module, so only a `try_reserve_exact` `Err` arm can construct one.
//!
//! Trust item: A-STD-1/2/3 (SPEC.md §3). The guard body is the binding point
//! between the returned `Result` and vstd's unconstrained `try_reserve_exact`
//! spec; that binding is hash-pinned text, not an `ensures`.

use vstd::prelude::*;

pub mod sites;

pub use sites::{AllocSite, Entry, IcoEntry};

verus! {

/// Upper bound on a single byte-oriented reservation (SPEC.md §4). 67,108,864 B.
pub const MAX_BYTES: usize = 67_108_864;

/// Upper bound on a `Reserved<T>` room, in elements (SPEC.md §3).
pub const MAX_ROOM: usize = 4_096;

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

/// The outcome type of every `try_*` entry point (SPEC.md §1). `Format` carries
/// the frozen error; `OutOfMemory` is outside every frozen error list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TryError {
    Format(DecodeError),
    OutOfMemory(OutOfMemory),
}

/// The frozen error enum, unchanged (SPEC.md §1). Truncated here to the
/// variants the toy decoder can return.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    TruncatedPixels,
    InvalidImageSize,
    RleTruncated,
}

/// A reservation failure. Constructed only by the guard's `try_*` functions,
/// from the `Err` arm of `try_reserve_exact`, and by ICO's `import_dib`.
/// Fields private to this module.
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
    /// requires.
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

/// `TryError::name()`: the frozen spelling for `Format(e)`, `"OutOfMemory"`
/// otherwise. `'static`, never allocates.
pub fn try_error_name(e: TryError) -> (s: &'static str)
    ensures
        match e {
            TryError::Format(DecodeError::TruncatedPixels) => s == "TruncatedPixels",
            TryError::Format(DecodeError::InvalidImageSize) => s == "InvalidImageSize",
            TryError::Format(DecodeError::RleTruncated) => s == "RleTruncated",
            TryError::OutOfMemory(_) => s == "OutOfMemory",
        },
{
    match e {
        TryError::Format(DecodeError::TruncatedPixels) => "TruncatedPixels",
        TryError::Format(DecodeError::InvalidImageSize) => "InvalidImageSize",
        TryError::Format(DecodeError::RleTruncated) => "RleTruncated",
        TryError::OutOfMemory(_) => "OutOfMemory",
    }
}

/// The frozen-typed result of a `try_*` result that is not `OutOfMemory`
/// (SPEC.md §2): `Ok(v) ↦ Ok(v)`, `Err(Format(e)) ↦ Err(e)`. The third arm is
/// unreachable in every clause that uses this function, which is why it may
/// return an unspecified value.
pub open spec fn fmt_part(r: Result<Vec<u8>, TryError>) -> Result<Vec<u8>, DecodeError> {
    match r {
        Ok(v) => Ok(v),
        Err(TryError::Format(e)) => Err(e),
        Err(TryError::OutOfMemory(_)) => arbitrary(),
    }
}

/// The eight-byte type marker the toy entry decoder writes into every entry
/// (the `[u8; 8]` type_hex of decision D2). Spec and exec forms, so the model
/// and the code agree by construction.
pub open spec fn marker_hex() -> [u8; 8] {
    [b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0']
}

pub fn marker_hex_exec() -> (a: [u8; 8])
    ensures a == marker_hex(),
{
    [b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0']
}

/// Every entry of `l` is `Ok`.
pub open spec fn all_ok(l: Seq<(AllocSite, nat, bool)>) -> bool {
    forall|j: int| 0 <= j < l.len() ==> l[j].2
}

/// `l` with the `ok` flag dropped.
pub open spec fn req_of_log(l: Seq<(AllocSite, nat, bool)>) -> Seq<(AllocSite, nat)> {
    Seq::new(l.len(), |j: int| (l[j].0, l[j].1))
}

/// The log-only part of theorem shape `G` clauses 1-2 (THEOREMS-ALLOC-DELTA §0).
/// Exported by DIB as `dib::dib::log_tied` for ICO's `import_dib`.
pub open spec fn log_tied(r: Result<Vec<u8>, TryError>, l: Seq<(AllocSite, nat, bool)>) -> bool {
    match r {
        Err(TryError::OutOfMemory(o)) =>
            l.len() >= 1
            && (forall|j: int| 0 <= j < l.len() - 1 ==> l[j].2)
            && !l[l.len() - 1].2
            && l[l.len() - 1].0 == o.site_spec()
            && l[l.len() - 1].1 == o.elements_spec() as nat,
        _ => all_ok(l),
    }
}

/// Reserve `n` zero bytes. `n >= 1`; a zero request is the caller's job to skip.
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

/// Copy `s` into a fresh vector.
pub fn try_copy(site: AllocSite, s: &[u8], Tracked(log): Tracked<&mut AllocLog>)
    -> (r: Result<Vec<u8>, OutOfMemory>)
    requires
        1 <= s.len() <= MAX_BYTES,
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
pub struct Reserved<T> {
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
    pub fn push(&mut self, x: T)
        requires
            old(self).view().len() < old(self).room(),
        ensures
            final(self).view() == old(self).view().push(x),
            final(self).room() == old(self).room(),
    {
        self.v.push(x);
    }

    pub fn into_vec(self) -> (v: Vec<T>)
        ensures v@ == self.view(),
    {
        self.v
    }
}

/// Reserve room for `n` elements of `T` without filling them. `n == 0` returns
/// `Ok` without touching the allocator. `size_of::<T>() >= 1` and
/// `n * size_of::<T>() <= isize::MAX` are discharged at each call site from a
/// `global layout` declaration for the concrete `T` (SPEC.md §3, gate F2).
pub fn try_with_room<T>(site: AllocSite, n: usize, Tracked(log): Tracked<&mut AllocLog>)
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
