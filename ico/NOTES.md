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

## M3 (T5, T6, T8)

`ico_verus/src/model.rs` gains the total decode model `MD(b,E,i)` (R8/R11: the
ICO-controlled `md_ico` plus, on the selected valid DIB branch,
`dib_verus::model::mp(b.subrange(start,end), IcoEntryDibSlack)` mapped to
`Ok(dib_wrap)`/`Err`), the exact statement `md_eq`, the composition discharger
`lemma_md_eq_dib`, the T6 precedence predicates/lemmas and the T8/T7 lemmas.

`decode_entry`'s postcondition is now `model::md_eq`: exact for header/guard
errors, `IndexOutOfRange`, the forged-record `InvalidEntryRecord` check, the
entry defects, `PngDelegation`, and the composition with the DIB model on the
validated declared slice. The selected-DIB branch slices exactly
`[payload[0], payload[1])`, proves `slice@ == bytes@.subrange(start,end)`, calls

    let (r0, _steps) = dib_verus::decode_with_steps(slice, Mode::IcoEntryDibSlack);

once, and discharges `md_eq` from the returned `decode_mp_eq`. This is the DIB
1.1 public tracked call M4a's bound needs, called now so M4a leaves the executable
alone; `_steps` (a `Ghost<nat>`) is discarded and erases, so behaviour is
byte-identical.

Proved: T5 (exact total `decode_entry` incl. the composition), T6 (exact
precedence on both public calls; decode reads only `entries.len()` and
`entries[index]`), T8 (variants and the count-cap-returns-before-loop fact), plus
the T7 clause M2 deferred (`4*w*h <= 67,108,864` on a DIB success). Witnesses:
`t5_decode_exact`, `t6_precedence`, `t6_precedence_full`,
`t6_parse_directory_priority`, `t6_decode_ordering`, `t7_dib_output_bound`,
`t8_termination`; `t1_decode_ico` now discharges from `md_eq` via
`lemma_md_eq_implies_decode_pre`.

Three clean sensitivity kills (disposable copies): skip the selected-record
comparison, skip the list-length check, and pass `bytes[start..]` instead of the
declared slice. See `PROOF-REPORT-M3.md`.

Clean full reverify: `verification results:: 101 verified, 0 errors` (after
`cargo clean`). Executable behaviour byte-identical (`cargo test --release` →
`conformance: 217/217 vectors passed`).

## M4a (T9, T10, T11)

A ghost step counter (`&mut Ghost<nat>`) is charged on every ICO-controlled loop
body: the record loop, the eight-byte signature comparison, and the bounded list
initialisation (`parse_step_bound`); the wrapper charges its single signature
comparison. `parse_steps`/`parse_tracked` and `decode_entry`/`decode_entry_impl`
are split so the public bodies stay `parse`/`decode_entry` (now thin wrappers) with
unchanged postconditions; new public tracked exports `parse_with_steps` and
`decode_entry_with_steps` state `steps <= 16*(|b|+1)` for every `b,E,i`. The
combined bound `t9_step_cap = wrapper_own_bound + dib_steps_bound` is stated on the
executable, using the DIB 1.1 `decode_with_steps` published bound
`steps_DIB <= 16*(|slice|+N)` on the same call; off the selected-DIB decision no DIB
call is made.

A ghost allocation counter charges `Vec::with_capacity(count)` once in
`parse_tracked` (zero on a directory error) and nothing in the wrapper; labelled
honestly that it cannot catch an uncharged allocation. `tools/audit_alloc.sh`
(grep of the allowed site) is the enforcement and fails on injected
`vec!`/`Vec::with_capacity`/`to_vec`/`clone`/`collect`.

T11: `src/model_vectors.rs` (generated by `tools/gen_model_vectors.py`, expected
values from `spec-frozen/VECTORS.json` only) proves `mc`/`md` equal to the fixture
`Result` for every small vector, closed by `by (compute)`; 41 DIB-branch decode
vectors are excluded by the manifest's "no concrete DIB MP" rule and stay runtime.
`src/witnesses.rs` proves three public `decode_entry` calls (valid selected beside
invalid unselected → `PngDelegation`; forged selected → `InvalidEntryRecord`;
forged unselected ignored). All 31 named non-vacuity witnesses have proof items;
`src/model.rs` proves the symbolic L1-L4 lemmas.

Clean full reverify: `verification results:: 303 verified, 0 errors` (after
`cargo clean`). Executable behaviour byte-identical (`cargo test --release` →
`conformance: 217/217 vectors passed`). See `PROOF-REPORT-M4a.md`.
