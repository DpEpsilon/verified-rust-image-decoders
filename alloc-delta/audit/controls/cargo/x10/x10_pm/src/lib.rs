//! x10_proc_macro_untracked_read (SPEC §6 required control): a proc macro
//! that, during expansion, reads an undeclared file through std::fs and an
//! undeclared variable through std::env. Neither read is in dep-info.
extern crate proc_macro;
use proc_macro::TokenStream;
#[proc_macro]
pub fn x10_len(_: TokenStream) -> TokenStream {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../x10-undeclared-input.txt");
    let n = std::fs::read(path).map(|v| v.len()).unwrap_or(0);
    let s = std::env::var("X10_SECRET").unwrap_or_default();
    format!("{}usize", n + s.len()).parse().unwrap()
}
