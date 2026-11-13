//! x21_inline_asm: inline assembly in a root. Must fail with `inline-asm`.
#![no_std]
pub fn x21_root(x: u64) -> u64 {
    let y: u64;
    unsafe { core::arch::asm!("mov {0}, {1}", out(reg) y, in(reg) x) };
    y
}
