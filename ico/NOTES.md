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
