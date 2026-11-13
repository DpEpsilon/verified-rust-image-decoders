//! x20_const_dyn: a trait-object call whose vtable comes from a const, not
//! from an unsizing in the calling body. Must fail with `virtual-dispatch`.
#![no_std]
extern crate alloc;
use alloc::vec::Vec;
pub trait Mk {
    fn mk(&self) -> usize;
}
struct A;
impl Mk for A {
    fn mk(&self) -> usize {
        let mut v: Vec<u8> = Vec::new();
        v.push(1);
        v.len()
    }
}
const OBJ: &dyn Mk = &A;
pub fn x20_root() -> usize {
    OBJ.mk()
}
