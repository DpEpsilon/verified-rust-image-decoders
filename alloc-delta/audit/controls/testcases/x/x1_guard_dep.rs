//! External crate named `guard` for x1_extern_namesake. Its `try_zeroed`
//! reaches the same allocation-capable instances as the audited crate's local
//! `guard::try_zeroed`.
#![no_std]
extern crate alloc;
use alloc::vec::Vec;
#[inline]
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
// `#[inline]` makes rustc export this body's MIR, so the audit walks into it
// (a non-inline upstream body would be a no-MIR leaf and fail closed earlier).
