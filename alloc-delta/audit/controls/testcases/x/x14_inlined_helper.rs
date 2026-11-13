//! x14_inlined_helper_in_guard: the admitted guard calls a small local helper
//! that the MIR inliner folds into it, and the helper grows a Vec. The
//! allowlist admits Vec::push only from `crate::guard::try_zeroed`. The tool
//! attributes calls through rustc's inlined-scope records, so the push is the
//! helper's, and the run must fail naming `helper_push`. (A tool that read the
//! optimised MIR without undoing inlining would see the push in try_zeroed and
//! pass it.)
#![no_std]
extern crate alloc;
use alloc::vec::Vec;
pub mod guard {
    use alloc::vec::Vec;
    #[inline]
    fn helper_push(v: &mut Vec<u8>) {
        v.push(1u8);
    }
    #[inline(never)]
    pub fn try_zeroed(n: usize) -> Vec<u8> {
        let mut v = Vec::new();
        v.reserve_exact(n);
        helper_push(&mut v);
        v
    }
}
pub fn x14_root(n: usize) -> usize {
    let v: Vec<u8> = guard::try_zeroed(n);
    v.len()
}
