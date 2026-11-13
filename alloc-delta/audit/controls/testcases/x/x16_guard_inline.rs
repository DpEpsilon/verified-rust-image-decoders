//! x16_guard_inline_never: an allocation-capable function in a module named
//! `alloc_guard` without #[inline(never)] (SPEC §3). Must fail with
//! `guard-inline-never`.
#![no_std]
extern crate alloc;
pub mod alloc_guard {
    use alloc::vec::Vec;
    pub fn try_zeroed(n: usize) -> Vec<u8> {
        let mut v = Vec::new();
        let _ = v.try_reserve_exact(n);
        v
    }
}
pub fn x16_root(n: usize) -> usize {
    alloc_guard::try_zeroed(n).len()
}
