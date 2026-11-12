//! Probe 1: `#![no_std]` + `extern crate alloc` + vstd with `default-features =
//! false, features = ["alloc"]`.
//!
//! The toy function reserves exactly with `Vec::try_reserve_exact` and pushes,
//! the same shape as the alloc-delta guard (`alloc_guard::try_zeroed`), so that
//! vstd's `assume_specification` for `Vec::try_reserve_exact` and `Vec::push`
//! (std_specs/vec.rs) must resolve for the proof to go through.

#![no_std]

extern crate alloc;

use vstd::prelude::*;
use alloc::vec::Vec;

verus! {

/// Reserve `n` elements exactly, then push one value. Returns `None` when the
/// reservation fails (the out-of-memory arm), `Some(len)` otherwise.
///
/// vstd's `try_reserve_exact` spec fixes the *view* (contents) but says nothing
/// about capacity; `push` appends. So after one successful push the view is
/// exactly `[v]`, independent of `n`. That is the whole content of the two
/// assumed specs this toy exercises.
pub fn try_zeroed_push(n: usize, v: u8) -> (r: Option<usize>)
    ensures
        r is Some ==> r.unwrap() == 1,
{
    let mut out: Vec<u8> = Vec::new();
    match out.try_reserve_exact(n) {
        Ok(()) => {
            let ghost before = out@;
            out.push(v);
            assert(out@ == before.push(v));
            assert(out@.len() == 1);
            Some(out.len())
        }
        Err(_e) => None,
    }
}

fn skeleton() { assert(1 == 0 + 1); }

} // verus!
