# Creusot notes
- Creusot v0.14.0, sources and examples in /opt/creusot-src (examples/, tests/should_succeed/,
  the guide under /opt/creusot-src if present). `cargo creusot` (run via ./verify.sh) translates
  to Why3 and proves with Alt-Ergo, Z3, CVC4, CVC5. The support crate is `creusot-std`
  (`use creusot_std::prelude::*;`). Contracts: `#[requires(..)]`, `#[ensures(..)]`,
  `#[variant(..)]`, `#[invariant(..)]` in Pearlite; `x@` is the logical view.
- Creusot proves ordinary safe Rust; contracts are erased in normal builds, so for M1 add no
  contracts and `cargo test` works as usual. Tests go in `tests/`.
