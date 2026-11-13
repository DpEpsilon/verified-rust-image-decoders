//! x25_extern_malloc: crate code declares and calls libc `malloc` itself. An
//! allocator-shaped foreign symbol is judged as an allocation sink (never an
//! ffi leaf a reviewer could admit as "none"); must fail with
//! `unauthorized-alloc-path`.
#![no_std]
extern "C" {
    fn malloc(n: usize) -> *mut u8;
}
pub fn x25_root(n: usize) -> usize {
    unsafe { malloc(n) as usize }
}
