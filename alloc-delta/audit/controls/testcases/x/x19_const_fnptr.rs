//! x19_const_fnptr: a call through a function pointer read from a const
//! table. The target is not visible as a cast in the calling body; the call
//! must still fail closed with `fn-ptr-call`.
#![no_std]
extern crate alloc;
use alloc::vec::Vec;
fn allocates(n: usize) -> usize {
    let mut v: Vec<u8> = Vec::new();
    v.push(n as u8);
    v.len()
}
const TABLE: [fn(usize) -> usize; 1] = [allocates];
pub fn x19_root(i: usize, n: usize) -> usize {
    (TABLE[i % 1])(n)
}
