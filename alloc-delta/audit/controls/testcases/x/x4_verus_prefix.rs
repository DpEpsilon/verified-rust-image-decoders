//! x4_verus_prefix (SPEC §6 required control; defect 3): an undeclared
//! proc-macro crate whose name begins with `verus`. The prototype admitted any
//! `verus*` crate by name; must fail with `undeclared-proc-macro`.
#![no_std]
extern crate verus_evil;
verus_evil::passthrough! {
    pub fn x4_root() -> usize {
        0
    }
}
