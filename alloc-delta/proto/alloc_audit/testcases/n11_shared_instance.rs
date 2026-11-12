//! Negative control N11: an allocation-capable instance shared by the allowed
//! guard and an unapproved helper.
//!
//! `guard::try_zeroed` reserves on a `Vec<u8>`; `unapproved_reserve` reserves on
//! a `Vec<u32>`. `RawVecInner<A>::reserve_exact` is parameterised by the
//! allocator only, so both calls resolve to the *same* monomorphised instance
//! `alloc::raw_vec::RawVecInner::<A>::reserve_exact<std::alloc::Global>`.
//! `unapproved_grow` grows a `Vec<u8>` through `Vec::push_mut`, the shared
//! growth instance the guard's own `push` loop also uses.
//!
//! The guard path is reachable first from the root. The tool keeps only the
//! first BFS parent of the shared instance, so it must not conclude from that
//! single path that the whole instance is allowed: the helper's incoming path
//! reaches the same allocator entry points and is not admitted by an allowlist
//! that names `guard::try_zeroed`.
//!
//! Expected: the tool fails, naming the helper's path(s) to the shared
//! allocation instance.

pub mod guard {
    /// Reserve `n` zeroed bytes. Admitted by the allowlist as a caller of
    /// `RawVecInner::reserve_exact`.
    pub fn try_zeroed(n: usize) -> Vec<u8> {
        let mut v = Vec::new();
        v.reserve_exact(n);
        let mut i = 0usize;
        while i < n {
            v.push(0u8);
            i += 1;
        }
        v
    }
}

/// An unapproved helper that reaches the *same* `reserve_exact` instance as the
/// guard, on a different element type. No allowlist entry admits this caller.
fn unapproved_reserve(m: usize) -> Vec<u32> {
    let mut v = Vec::new();
    v.reserve_exact(m);
    v
}

/// An unapproved helper that reaches the shared growth instance through
/// `Vec::push_mut` — the *same* instance the guard's own `push` loop uses,
/// because `push_mut<T, A>` is parameterised by `T = u8` and the allocator only,
/// and the guard fills a `Vec<u8>` too. (An earlier version of this helper
/// called `resize`, which reaches `reserve -> grow_amortized` instead and so did
/// not exercise the shared `push_mut` instance at all.)
fn unapproved_grow(k: usize) -> Vec<u8> {
    let mut v = Vec::new();
    let mut i = 0usize;
    while i < k {
        v.push(1u8);
        i += 1;
    }
    v
}

/// The public root. `guard::try_zeroed` is reached first, so the shared
/// instances' first BFS parent is on the guard path.
pub fn n11_shared_instance(n: usize, m: usize, k: usize) -> usize {
    let a = guard::try_zeroed(n);
    let b = unapproved_reserve(m);
    let c = unapproved_grow(k);
    a.len() + b.len() + c.len()
}
