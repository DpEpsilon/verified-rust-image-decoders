//! Toy proc-macro crate for negative control N16.
//!
//! It depends only on `proc_macro` (provided by rustc from the sysroot, so it
//! builds offline with no crates.io dependency). `run_tests.sh` compiles it to
//! a `.so` and passes the `.so` to the audit run with `--extern pmtoy=...`.
//! The single macro `passthrough!` returns its input tokens unchanged, so the
//! macro-generated item is present in the consumer's expanded source.

extern crate proc_macro;

use proc_macro::TokenStream;

/// Return the input tokens unchanged.
#[proc_macro]
pub fn passthrough(input: TokenStream) -> TokenStream {
    input
}
