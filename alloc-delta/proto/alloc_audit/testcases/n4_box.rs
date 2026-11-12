//! Negative control N4: `Box::new` allocates via `exchange_malloc`.
//! Expected: `n4_box` reaches `alloc::alloc::exchange_malloc`.

pub fn n4_box(v: u64) -> Box<u64> {
    Box::new(v)
}
