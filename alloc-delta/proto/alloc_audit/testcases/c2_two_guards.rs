//! Positive control C2: *two* admitted guard functions that call the SAME
//! allocation-capable instance, each with its own `[[entry]]`.
//!
//! `guard_a::try_zeroed` reserves on a `Vec<u8>` and `guard_b::try_zeroed`
//! reserves on a `Vec<u32>`. `RawVecInner<A>::reserve_exact` is parameterised by
//! the allocator only, so both calls resolve to the same monomorphised instance
//! `alloc::raw_vec::RawVecInner::<A>::reserve_exact<std::alloc::Global>`. The
//! allowlist gives that item TWO entries, one allowing `guard_a::try_zeroed` and
//! one allowing `guard_b::try_zeroed`.
//!
//! The old matcher selected a single entry with `entries.iter().find` and judged
//! only its `allow`, so a second entry for the same item was never consulted and
//! this crate would fail spuriously (the path found first, through one guard,
//! would be judged against the other guard's entry). The whole-segment matcher
//! checks every matching entry, so this crate passes.
//!
//! Expected: exit 0 with an allowlist; every path is justified.

pub mod guard_a {
    pub fn try_zeroed(n: usize) -> Vec<u8> {
        let mut v = Vec::new();
        v.reserve_exact(n);
        v
    }
}

pub mod guard_b {
    pub fn try_zeroed(m: usize) -> Vec<u32> {
        let mut v = Vec::new();
        v.reserve_exact(m);
        v
    }
}

/// The public root: reaches the shared `reserve_exact` instance through both
/// admitted guards. No other path allocates.
pub fn c2_two_guards(n: usize) -> usize {
    let a = guard_a::try_zeroed(n);
    let b = guard_b::try_zeroed(n);
    a.len() + b.len()
}
