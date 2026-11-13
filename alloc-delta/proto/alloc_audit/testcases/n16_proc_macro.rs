//! Negative control N16: the crate links a foreign proc-macro crate.
//! The source hash covers this crate's `.rs` text only. The tokens
//! `pmtoy::passthrough!` expands to are produced by a foreign proc macro and
//! are not in the source map, so the tool must fail closed unless the allowlist
//! declares the proc-macro crate.
//!
//! `pmtoy` is a toy proc-macro crate built offline from this directory's
//! `pmtoy.rs` (it depends only on `proc_macro`, which rustc provides from the
//! sysroot); `run_tests.sh` builds it before this control. It generates the
//! `pub fn n16_proc_macro` item below, so the expansion is a real build input
//! and not merely a linked-but-unused dependency.
//!
//! Expected: exit 1 with an `undeclared-proc-macro` finding naming `pmtoy`.

extern crate pmtoy;

pmtoy::passthrough! {
    pub fn n16_proc_macro() -> usize {
        0
    }
}
