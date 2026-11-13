#![no_std]
#[cfg(x8_from_build_script)]
pub fn x8_root(n: usize) -> usize {
    n + 8
}
#[cfg(not(x8_from_build_script))]
pub fn x8_root(n: usize) -> usize {
    n
}
