# ICO core — working notes

## M1 (plain implementation)

`ico_verus/src/ico.rs` implements R1–R10 as ordinary Rust outside `verus!`.
`src/lib.rs` keeps the cross-crate DIB skeleton (still verifying) and declares
`pub mod ico;`. `tests/conformance.rs` runs all 217 vectors: `parse` vectors
compare the whole R1 record list (typed record rendered back to JSON, so every
field — including absent optionals as `null` — is compared); `decode` vectors run
`parse` first unless `entries_override` is supplied, then `decode_entry`, and
compare exact dimensions and RGBA bytes or the exact error name (`Dib(ErrorName)`
for the DIB wrap).

Arithmetic: `dir_end = 6 + 16*count` and `offset + size` are computed in `u64`;
the range test is `offset > |b| || size > |b| - offset`, so a `u32` wrap
(`wide-sum-wrap`) cannot pass. Reads go through guarded helpers that bounds-check
before indexing; no `unsafe`, `unwrap`, `expect` or `panic!` is reachable.

Two vectors expand past the 64 MiB cap (`file-size-67108865`,
`too-large-before-bad-header`). R2 tests the length before reading, so the
harness materialises a zeroed buffer of the exact expanded length and requires
`FileTooLarge` from both entry points; it never builds the under-cap 64 MiB
fixture (`file-size-67108864`), which is decided by the `len > MAX_INPUT` test
alone.

Result: `conformance: 217/217 vectors passed`.

## M2 (verus! with contracts and proof)

`ico_verus/src/ico.rs` is now inside `verus!`; `src/ico.rs.m1` preserves the M1
plain file. `ico_verus/src/model.rs` holds the independent reference model
`H`/`ME`/`MC` (over `Seq<u8>` and mathematical integers), the equality model
(`opt_u16_eq`/`status_eq`/`entry_eq`), the decode-side decision `md_ico` and the
arithmetic lemmas; nothing in it calls the executable. `lib.rs` declares
`pub mod model;`.

Proved: T1 (safety of `parse` and the ICO side of `decode_entry`), T2 (`parse`
equals `MC` as the whole `Result`), T3 (record validity), T4 (signature
classification on valid entries), T7 (arithmetic and output bounds) and their
witness lemmas (`t1_decode_ico`, `t1_index_u32`, `t2_parse_exact`,
`t3_record_validity`, `t4_signature_classification`, `t7_arithmetic_bounds`).
Deferred to M3: T5, T6, T8, the model `MD` and the DIB composition. The
`decode_entry` postcondition is `model::decode_pre`, which fixes the whole
ICO-controlled decision exactly and the DIB-result shape (success or
`Dib(_)`), but not the DIB bytes.

Executable behaviour is byte-identical (`cargo test --release` →
`conformance: 217/217 vectors passed`). Three proof-driven edits: the
`Option`-returning reads became preconditioned `rd8`/`rd16`/`rd32`; the M1
defensive `dir_end > u32::MAX` branch was dropped as provably dead; `reconstruct`
returns `Entry` and `parse` has no unreachable arm.

Clean full reverify: `verification results:: 79 verified, 0 errors` (after
`rm -rf ico_verus/target`). See `PROOF-REPORT-M2.md`.
