//! Negative control N10: an FFI call. Expected: the tool records an `ffi` leaf
//! for the `extern` item and fails closed; it cannot see what a foreign
//! function allocates, so it does not attempt to reason about it.

extern "C" {
    fn getpid() -> i32;
}

pub fn n10_ffi() -> i32 {
    unsafe { getpid() }
}
