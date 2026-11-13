//! x15_generic_export: an exported generic function (SPEC §6 step 1: no core
//! exports one). Must fail with `generic-pub-fn` unless declared.
#![no_std]
pub fn x15_generic<T: Copy>(x: T) -> T {
    x
}
pub fn x15_root(n: usize) -> usize {
    x15_generic(n)
}
