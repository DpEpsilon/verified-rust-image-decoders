//! x2_include_after_url (SPEC §6 required control; defect 2): an include after
//! a `"https://..."` string literal on the same line, which the prototype's
//! `//`-stripping text scan hid. Must fail with `undeclared-include`.
#![no_std]
pub fn x2_root() -> usize { let u = "https://x"; let p: &[u8] = include_bytes!("p.bin"); u.len() + p.len() }
