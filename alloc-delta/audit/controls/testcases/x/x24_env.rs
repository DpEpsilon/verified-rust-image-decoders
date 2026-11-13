//! x24_env_read: an `env!` read, tracked by rustc's dep-info; must fail with
//! `undeclared-env` and pass once declared.
#![no_std]
pub fn x24_root() -> usize {
    env!("ALLOC_AUDIT_X24").len()
}
