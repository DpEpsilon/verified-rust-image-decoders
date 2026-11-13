//! x12c: two crates named `helperdep` in the graph (a renamed second copy).
#![no_std]
extern crate helperdep;
extern crate hd2;
pub fn x12c_root(n: usize) -> usize {
    helperdep::helper(n) + hd2::helper(n)
}
