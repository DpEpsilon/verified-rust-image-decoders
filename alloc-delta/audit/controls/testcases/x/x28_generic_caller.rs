#![no_std]
extern crate alloc;
#[inline(never)]
fn guard<T>() -> alloc::vec::Vec<u8> {
    let mut v=alloc::vec::Vec::new();
    let _ = v.try_reserve_exact(core::mem::size_of::<T>());
    v
}
pub fn root() -> usize { guard::<u8>().capacity() + guard::<u16>().capacity() }
