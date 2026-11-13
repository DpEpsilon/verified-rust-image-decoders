//! x2b: the include macro under a renamed import. Must fail with
//! `undeclared-include`.
#![no_std]
use core::include_bytes as ib;
pub fn x2b_root() -> usize {
    let p: &[u8] = ib!("p.bin");
    p.len()
}
