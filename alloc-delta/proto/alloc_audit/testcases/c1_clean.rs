//! Clean control C1: reserve-then-push through a guard-like module.
//! Expected: every allocation on every path from the public root is reached
//! through `guard::try_zeroed`, which the allowlist names as an
//! allocation-capable item with `allow = "*"`. No other path allocates. The
//! tool passes when the allowlist is supplied; the report shows the (expected)
//! allocation, justified.
//!
//! The module mirrors the DIB guard's shape: a single reservation entry point
//! that returns a `Vec` the caller then fills with `push` (which cannot regrow
//! because capacity was reserved first).

pub mod guard {
    /// Reserve `n` zeroed bytes up front. The only allocation site in this crate.
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

/// A pointer holder with a `Drop` impl that does not allocate.
pub struct Holder {
    v: Vec<u8>,
}

impl Holder {
    pub fn new(n: usize) -> Holder {
        Holder { v: guard::try_zeroed(n) }
    }

    pub fn len(&self) -> usize {
        self.v.len()
    }
}

impl Drop for Holder {
    fn drop(&mut self) {
        // Deallocation only; no allocation on this path.
        self.v.clear();
    }
}

/// The public root. Every path reaches the allocator only through
/// `guard::try_zeroed`.
pub fn c1_clean_root(n: usize) -> usize {
    let h = Holder::new(n);
    h.len()
}
