//! x2c: a `macro_rules!` macro from another crate that expands to
//! include_bytes!. Must fail with `undeclared-include`.
#![no_std]
#[macro_use]
extern crate x2_incmac;
pub fn x2c_root() -> usize {
    let p: &[u8] = embed!();
    p.len()
}
