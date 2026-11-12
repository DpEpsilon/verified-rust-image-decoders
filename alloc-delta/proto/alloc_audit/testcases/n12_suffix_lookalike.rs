//! Negative control N12: a *suffix lookalike* module path must not be admitted
//! by an allowlist entry that names the guard.
//!
//! The allowlist admits `guard::try_zeroed` as the caller of the reservation
//! item. The root calls both the admitted `guard::try_zeroed` AND
//! `unapproved_guard::try_zeroed` — a *different* function whose def-path ends
//! with the admitted name (`"unapproved_guard::try_zeroed".ends_with("guard::try_zeroed")`).
//! The old matcher accepted a caller when `c.ends_with(allow)` or
//! `allow.ends_with(c)`, so the lookalike passed; the whole-segment matcher must
//! refuse it while still admitting the real guard.
//!
//! Expected: the tool fails, with the lookalike's path named as
//! `unauthorized-alloc-path`, and the guard's own path still shown as justified.

pub mod guard {
    /// The admitted function.
    pub fn try_zeroed(n: usize) -> Vec<u8> {
        let mut v = Vec::new();
        v.reserve_exact(n);
        v
    }
}

/// A module whose name is `unapproved_guard`, not `guard`.
pub mod unapproved_guard {
    /// Same last segment as the admitted function, different def-path.
    pub fn try_zeroed(m: usize) -> Vec<u32> {
        let mut v = Vec::new();
        v.reserve_exact(m);
        v
    }
}

/// The public root: reaches the shared `reserve_exact` instance through both the
/// admitted guard and the unadmitted lookalike.
pub fn n12_suffix_lookalike(n: usize, m: usize) -> usize {
    let a = guard::try_zeroed(n);
    let b = unapproved_guard::try_zeroed(m);
    a.len() + b.len()
}
