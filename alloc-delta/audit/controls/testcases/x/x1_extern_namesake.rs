//! x1_extern_namesake (SPEC §6, required control; defect 1). A local module
//! `guard` with an admitted `guard::try_zeroed`, plus a dependency crate named
//! `guard` whose `try_zeroed` reaches the same allocation-capable instances.
//! An allowlist value `guard::try_zeroed` must be `allowlist-ambiguous`; with
//! `crate::guard::try_zeroed` the external function's paths must be
//! `unauthorized-alloc-path`, and the two functions must be distinct nodes.
#![no_std]
extern crate alloc;
extern crate guard as ext_guard;
use alloc::vec::Vec;
pub mod guard {
    use alloc::vec::Vec;
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
pub fn x1_root(n: usize) -> usize {
    let a: Vec<u8> = guard::try_zeroed(n);
    let b: Vec<u8> = ext_guard::try_zeroed(n);
    a.len() + b.len()
}
