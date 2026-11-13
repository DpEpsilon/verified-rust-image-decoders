//! The dependency crate for x12_foreign_crate; allocation-free.
#![no_std]
pub fn helper(n: usize) -> usize {
    n.wrapping_mul(3)
}
