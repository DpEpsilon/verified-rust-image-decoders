# M2 notes — proofs of T1 to T6

## Status

All of T1–T6 are proved. `./verify.sh` reports `verification results:: 47 verified, 0 errors`
(run 72, forced full reverify). `cargo test` reports `conformance: 111/111 vectors passed`
and `test result: ok. 1 passed; 0 failed`. The executable behaviour is byte-identical to M1:
the only changes to the algorithms are (a) three std calls Verus cannot model, replaced by
hand-written equivalents, and (b) loop `invariant`/`decreases` clauses.

## Files

- `icns_verus/src/icns.rs` — the implementation, now inside `verus! { ... }`, with the M2
  contracts. The public API (`parse`, `decode_rgb`, `decode_mask`, `combine`), the records
  and the error names are unchanged.
- `icns_verus/src/model.rs` — the formal reference model written independently of the
  executable code: `hex4`/`hex_val`/`code_of`, R8 `legacy_side`, R9 `nominal_side`, the PNG
  and JP2 signature predicates, `classify`, `entry_inv`, `parsed_inv`, and the hex lemmas.
- `icns_verus/src/lib.rs` — unchanged except for `pub mod model;`.

## What the contracts say (and where each theorem is discharged)

- **T1** — `parse` has no `requires`, and its postcondition is `parsed_inv(bytes@, E@)` on
  success. Verus proves, for every `bytes` at every representable length, that no indexing,
  arithmetic or narrowing can fail: every read is guarded by the `declared`-vs-`len` equality
  and the per-entry bounds check, and every offset is computed in `u64` before narrowing. The
  entry loop has `decreases declared - offset`, and `offset` grows by `length >= 8`. The entry
  list is capped at `MAX_ENTRIES = 4096`, and each record's `type_hex` is eight hex digits, so
  the only allocation `parse` requests is bounded. `parse` never allocates a pixel buffer.
- **T2** — `rle_decode` has `requires n <= MAX_LEGACY_SIDE * MAX_LEGACY_SIDE` (16384, which
  covers the required 256/1024/2304/16384) and allocates exactly three `n`-byte channel
  buffers. Every payload read is guarded by `cursor`/`payload.len()` checks, the per-packet
  copy is a helper (`rle_copy`) with its own bounds, `produced` increases by `count >= 1`, and
  the outer and inner loops carry `decreases` clauses. It never indexes past `payload`.
- **T3** — `decode_rgb` has `requires parsed_inv(bytes@, entries@)`; on success it returns
  `width == height <= 128` and `data.len() == width*width*3 <= 49152`. The `side <= 128` fact
  is a proved lemma over the R8 table (each legacy side is 16/32/48/128), so `n <= 16384` and
  the channel and output allocations are bounded.
- **T4** — `decode_mask`, same precondition; success gives `data.len() == width*width <= 16384`
  and `width <= 128`.
- **T5** — `combine`, same precondition; success gives `data.len() == width*width*4 <= 65536`.
  Its two internal calls `decode_rgb`/`decode_mask` each carry `parsed_inv`, and those
  preconditions are discharged by the same `parsed_inv(bytes@, entries@)` under which `combine`
  itself is verified — no axiom is introduced at the call sites.
- **T6** — the success invariant is `parsed_inv` (see `model.rs`): `|b| <= 67,108,864`,
  `|E| <= 4096`, and for every entry `8 <= start <= end <= |b|`, an eight-digit lowercase
  `type_hex`, `type_hex == hex4(type_code)`, and `opaque_candidate`/`nominal_side` equal to
  `classify` of the actual parsed bytes. `parse` proves it, and the three wrappers
  `parse_success_invariant`, `decode_rgb_after_parse`, `decode_mask_after_parse` and
  `combine_after_parse` show a concrete successful `parse` discharging each decoder's
  precondition. The empty file is covered (`E = []` satisfies the `forall`).

## Surprises and tool obstacles

1. Exec functions cannot be called from spec/proof code in Verus, so the model cannot be a
   "spec copy" that mirrors the executable and then be *called* on it. The executable instead
   carries a proven postcondition phrased in model terms, and spec-level reasoning stays inside
   the model. The bridge is `hex4(code)` built by executable `push` calls, with the postcondition
   `s@ == model::hex4(code)`.
2. `format!("{:02x}", ...)` carries no Verus postcondition, so R1's `type_hex` is built by
   pushing characters, and `parse`'s invariant is phrased over `hex4`. `Entry::type_code` is
   proved to recover the four bytes (`code_of(hex4(code)) == code`).
3. Three std calls are unsupported inside `verus!` and were replaced, keeping behaviour
   identical: `u32::from_be_bytes` → explicit shifts-free arithmetic; `String::as_bytes` →
   `String::as_str().as_bytes()`; `slice::to_vec` → a copy loop. Array `for` loops and
   byte-string `match` patterns on `[u8; 4]` are also unsupported, so loops are `while` with
   index variables and the code tables compare the four indices explicitly.
4. `is_ascii_spec_bytes` is a broadcast lemma, not a callable one; the ASCII premise for
   `str`→`bytes` needs `broadcast use`, plus a proved lemma that an eight-digit hex string is
   ASCII.
5. Array `==` in a *spec* does not discharge by itself; tables compare indices.

## Not done (M3/M4)

- No `MP`/`MR`/`MC`/`MA`/`MJ` reference model yet; T6's invariant is phrased directly over the
  parser, which is what M2 requires. M3 supplies the `parse(b)=Ok(E) ⇔ P(b,E)` bridge.
- No non-vacuity/mutation protocol yet (M4).

## M3c status — exact parser/model bridge

T7 and T8 now verify with the parser's live `parses_to` postcondition and concrete decoder call-site derivations. L4, L5 and L6, including their compact vector instantiations, verify; L1 and L2 were already proved. `runs/121.log` records `verification results:: 142 verified, 0 errors`; the M3c dispatch used ten `./verify.sh` runs (112–121). `cargo test --test conformance -- --nocapture` reports 111/111 vectors. See `PROOF-REPORT-M3c.md`. L3 and the remaining full non-vacuity protocol belong to later work.

## M3 notes — the MP reference model (historical, incomplete at that time)

The MP model is now defined and internally proved in `src/model.rs` (94 obligations verify,
`runs/077.log`): `mp`, `scan`, `walk`, `entry_at`, the R15 error order, `classify` and the
`Accepted`/`p_of` predicates, plus the walk/scan bridge, the `be32` round-trip, the `H(k,s)`
subrange lemmas and the all-empty dot-string entry lemmas (`big_e`, `entries_big_e`).

T7 and T8 are **not** proved: `parse`'s live postcondition is still M2's `parsed_inv`; the
`parses_to` statement and its per-branch early-return lemmas exist, but the entry loop was not
rewritten to carry the ghost offset sequence before the tool-call cap was reached. L1/L2/L4 have
their core lemmas proved but are not assembled into the manifest's stated form; L5 and L6 are not
started. See `PROOF-REPORT-M3.md`.

## M4c status — non-vacuity and mutation protocol

All of M4c is done. `./verify.sh` reports `verification results:: 315 verified, 0 errors` (run 206,
252 s; 212 before this milestone plus 102 vector obligations and `lemma_mj_ms_eq`).
`cargo test --test conformance` reports `conformance: 111/111 vectors passed`,
`test result: ok. 1 passed; 0 failed`.

- `src/model_vectors.rs` (generated, 3,260 lines): 102 proof obligations, one per vector at or
  below 4,096 bytes, each stating that the model application equals the fixture's `expected` and
  discharged by `by (compute)`. Generated by `tools/gen_model_vectors.py`, which reads expected
  values only from `spec-frozen/VECTORS.json` and checks each of the nine large vectors' expanded
  bytes against the manifest's symbolic construction.
- `src/model.rs` gained `mj_ms` (MJ over `MpEntry`) and `lemma_mj_ms_eq`.
- Mutations 1-10 run in `mutation/mN/` from the pristine `mutation/base/`; each fails the
  verifier on the obligation `THEOREMS.md` names. Logs: `mutation/mN/failure.log`; summaries:
  `mutation/mN/result.txt`. Tooling: `tools/mutations.py`, `tools/run_mutations.sh`,
  `tools/mutation_summary.py`.
- Mutation 5 and mutation 10 both failed directly, so neither manifest fallback was applied.
- Mutation 3 needed the check moved in *both* `scan` and `walk`; the single-site attempt failed at
  the scan/walk bridge instead of L6 (`runs/197.log`, superseded by `runs/205.log`).
- See `PROOF-REPORT-M4c.md` for the full tick sheet.

## M4b status — MJ; T12, T13

T12 and T13 are proved. `./verify.sh` reports `verification results:: 212 verified, 0 errors`
(run 175, full reverify after `cargo clean`). `cargo test --test conformance` reports
`conformance: 111/111 vectors passed`, `test result: ok. 1 passed; 0 failed`. The M4b additions
live in `src/model.rs`: `interleave_rgba`, `mj`, `combine_decoded_eq`, the `MJ` case lemmas
(`lemma_mj_missing_color`, `lemma_mj_missing_mask`, `lemma_mj_mc_err`, `lemma_mj_ma_err`,
`lemma_mj_mismatch`, `lemma_mj_ok`), `lemma_interleave_rgba_len`/`_exact`, the result-unpacking
lemmas `lemma_decoded_eq_ok`/`_err`, the field-transfer lemmas
`lemma_mc_fields_eq`/`lemma_ma_fields_eq`/`lemma_mj_fields_eq`, and the two exec wrappers
`t12_combine_exact_for_p` and `t13_combine_after_parse`. `combine` in `src/icns.rs` carries the
postcondition `combine_decoded_eq(r, bytes@, entries@, color_index, mask_index)`; its executable
statements are unchanged (the `?` operators became explicit `match`es, and the RGBA loop gained
four ghost `forall` invariants). A working M4a copy of icns.rs is kept as `src/icns.rs.m4a`
(not compiled). 10 `./verify.sh` runs (166–175).

## M4a status — MR, MC, MA; T9, T10, T11; L3

T9, T10, T11 and L3 are proved. `./verify.sh` reports `verification results:: 193 verified, 0
errors` (run 164, full reverify after `cargo clean`). `cargo test --test conformance` reports
111/111. The M4a model lives in `src/model.rs`: `rle_count`/`rle_needed`/`rle_literal`/`rle_bytes`,
`rle_chan_from`, `rle_rest`/`rle_run`, `mr`, `it32_prefix`/`strip_it32`, `interleave_rgb`, `mc`/`ma`
(over executable `Entry`), `mc_ms`/`ma_ms` (over `MpEntry`), `decoded_eq`/`rle_result_eq`, and the
L3 lemmas `l3_string`/`l3_entry`/`lemma_l3_large_mask`/`lemma_l3_mask_t8mk`. Executable
postconditions: `rle_decode` carries `rle_result_eq(r, mr(payload@, n))`; `decode_rgb`/`decode_mask`
carry `rgb_decoded_eq`/`mask_decoded_eq`; `rle_copy` carries the `write_seq` packet equality. A
working M3 copy of icns.rs is kept as `src/icns.rs.m3`. 44 `./verify.sh` runs (122–165).
T12, T13 and the non-vacuity protocol are M4b.

## usize width (coordinator note, 2026-11-09)
Checked on a standalone Verus file with the same toolchain (0.2026.11.02): `assert(usize::MAX == 0xffff_ffff_ffff_ffff)` FAILS while `assert(usize::MAX >= 0xffff_ffff)` verifies, i.e. Verus treats `usize` as a 32-or-64-bit type by default unless a crate declares `global size_of usize == 8` (none of the crates here do: grep for `global size_of`/`size_of usize` is empty). The proofs therefore hold for both widths in the sense of the manifest's 'parameterised by a word width'. This addresses the 'no 32-bit verifier run' review finding as a width-generic proof; it is not an actual i686 build or test run.
