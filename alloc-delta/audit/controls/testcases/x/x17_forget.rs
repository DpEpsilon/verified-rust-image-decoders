//! x17_forbidden: crate code calls mem::forget and Rc::new (SPEC §2: no
//! mem::forget, Box::leak, ManuallyDrop, Rc, Arc). Must fail with
//! `forbidden-item`.
#![no_std]
extern crate alloc;
pub fn x17_root(n: usize) -> usize {
    let r = alloc::rc::Rc::new(n);
    let m = *r;
    core::mem::forget(r);
    m
}
