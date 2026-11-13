#![no_std]
#![feature(core_intrinsics)]
#[inline(never)]
fn guard<T>(i: usize) -> u8 {
    let xs = [core::mem::size_of::<T>() as u8];
    core::intrinsics::black_box(xs[i])
}
pub fn root(i:usize) -> u8 { guard::<u8>(i) ^ guard::<u16>(i) }
