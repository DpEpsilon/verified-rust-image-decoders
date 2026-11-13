//! x12_foreign_crate (SPEC §9 required control): a root reaching a function in
//! an unlisted dependency crate. Fails with `foreign-crate-instance`; passes
//! once a kind = "crate" entry (with its dependency contract) admits it; with
//! two linked crates of that name the entry is `allowlist-ambiguous-crate`.
#![no_std]
extern crate helperdep;
pub fn x12_root(n: usize) -> usize {
    helperdep::helper(n)
}
