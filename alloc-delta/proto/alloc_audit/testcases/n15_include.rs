//! Negative control N15: the crate embeds a non-source file with `include_str!`.
//! The source hash covers the `.rs` text only; the embedded file's bytes are a
//! build input the hash does not cover, so the tool must fail closed unless the
//! allowlist declares the include.
//!
//! Expected (no declaration): exit 1 with an `undeclared-include` finding.

pub const EMBEDDED: &str = include_str!("n15_payload.txt");

pub fn n15_include() -> usize {
    EMBEDDED.len()
}
