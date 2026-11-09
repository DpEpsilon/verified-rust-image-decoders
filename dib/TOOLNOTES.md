# Verus notes
- Verus release 0.2026.11.02 (binary /opt/verus), Rust 1.99.1. `cargo verus verify` runs inside
  dib_verus via ./verify.sh. vstd is the support crate; its source/docs are not installed. For
  examples and the guide you can `git clone --depth 1 https://github.com/verus-lang/verus` into
  a directory under this project (source/vstd, examples/, source/docs/guide).
- Verified code lives inside `verus! { ... }`. For M1 keep the plain implementation OUTSIDE it
  (ordinary Rust functions in the same crate); later milestones move or re-state it inside.
- Specs: `requires`/`ensures`, `spec fn`, `proof fn`, `Seq<u8>` views `bytes@`, `decreases`.
- Plain `cargo test` works for tests of non-verified code; tests go in `tests/`.
