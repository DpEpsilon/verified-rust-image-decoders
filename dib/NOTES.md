# Notes

## M1 (done)
- `dib_verus/src/dib.rs`: whole DIB-CORE-SPEC 1.0 as ordinary Rust outside `verus!`.
  `pub fn decode(bytes: &[u8], mode: Mode) -> Result<Decoded, Error>`, `Mode {BmpFile,
  BareDib, IcoEntryDib}`, `Decoded {width, height, rgba}` (all `u32`/`Vec<u8>`), and the
  26 R15 error names with `Error::name()`.
- Safety: no `unsafe`; input reads go through `rd`/`rd16`/`rd32`, which return 0 past the
  end, so no input can reach a slice-index panic. All width/height/palette/offset products
  are computed in `u64` before narrowing, so the code holds on a 32-bit `usize`
  (`palette_length = table_count * palette_stride` is the R7 case). One `vec![0u8; 4*n]`
  allocation, `4N <= 64 MiB`, made only after R1..R7. The RLE paint buffer is `[u8; 255]`,
  a fixed-size stack local, not input-sized.
- `dib_verus/tests/conformance.rs`: loads `../spec-frozen/VECTORS.json`, expands whole-input
  and suffix repeats per R16, runs all 214, prints `conformance: N/214 vectors passed`.
- `dib_verus/tests/corpus.rs` compares against `tests/corpus_expected.json`, generated once
  by `tools/corpus_expected.py` (imports the frozen oracle `/home/ws/projects/dib/spec/
  check_vectors.py`, hash `01029bae...` matches `FREEZE.txt`; first-DIB shim copied from
  `run_corpus.py`).
- Checks run: `cargo test` -> 214/214 vectors, 84/84 corpus (58 accepted) byte-identical to
  the oracle; `cargo build --release` clean; `./verify.sh "M1: skeleton still verifies"` ->
  run 001, `verification results:: 1 verified, 0 errors`. A 6000-case differential fuzz
  against the oracle (mutated vectors + random bytes, all three modes) agreed 6000/6000 with
  no panic; scratch CLI removed afterwards.

## Not done (later milestones)
- M2 did NOT finish. The public `decode` is still the M1 plain implementation
  outside `verus!`; the pixel loops are unproven. See `STUCK.md` and
  `PROOF-REPORT-M2.md`.

## M2 (partial — header and allocation path)
- `dib_verus/src/model.rs`: the total header model `MH` (`mh`), written
  independently of the executable from the spec text over `Seq<u8>` and
  mathematical integers: `le16`/`le32`/`s32`, `contiguous`/`within_bits`/
  `masks_ok`, `stride_of`, `r1_prefix`/`r2r5`/`r6r7`/`r6r7_result`, and the
  lemmas `lemma_le16_lt`, `lemma_le32_lt`, `lemma_stride_bound`,
  `lemma_mh_ok_bounds`, `lemma_r6r7_ok_bounds`, `lemma_r6r7_eq_result`,
  `lemma_stride_times_8`, `lemma_sample_within_stride`,
  `lemma_subbyte_within_stride`, `lemma_subbyte_shift`.
- `dib_verus/src/dib.rs` inside `verus!`: `parse_header` = `bmp_prefix` +
  `dib_fields` + `parse_masks` + `dib_finish`, carrying **T2** (the parser equals
  `MH` as the exact record or the exact first error, no `requires`); and
  `alloc_output`/`alloc_calls`/`out_offset`/`map_row_spec`/`lemma_map_row_bound`
  carrying the **T3** allocation bound and the R9 orientation offset. `rd`/`rd16`/
  `rd32`/`rd32s`/`rem` gained preconditions instead of returning 0 past the end.
- `dib_verus/src/dib.rs` outside `verus!`: the M1 plain `decode` (helpers
  renamed `rd_p`/`stride_p`/...), byte-identical, so `cargo test` still runs.
- Checks: clean full reverify `verification results:: 124 verified, 0 errors`
  (`runs/228.log`); `cargo test` -> 214/214 vectors, 84/84 corpus (58 accepted).

## Not done (M2's remainder)
- The pixel loops (R8 conversion, R9 rows, R10–R11 RLE, R12 AND mask) are not
  inside `verus!`, and `decode` is not connected to the proved header layer, so
  T1's no-panic/OOB/overflow clauses and T3's "exactly one call on the success
  path" are not discharged for the public entry point. `decode_raw` exceeded the
  per-function resource limit; `STUCK.md` records it. (Resolved in M2b.)

## M2b (Sol handoff)
- Public `decode` is inside `verus!` and calls the MH-exact `parse_header` on its
  actual input; 151 verified, 0 errors; 214/214 + corpus. Limit: `read_at` was a
  guarded total reader returning 0 past the end, so sample-address containment was
  not proved.

## M3 (done — T4, T5, T11; the M2b `read_at` limit closed)
- `read_at` removed. Every pixel-path byte read is now the precondition-carrying
  `rd` (`requires p < b@.len()`); `raw_sample` takes the row stride and proves the
  sample offset+width lies within the checked stride (`subbyte_within_stride`,
  `row_last_sample_within_stride`); `colour`/`palette_stride_bounds` prove the
  palette entry ends inside the slice; `apply_and` carries a per-row bound and
  `and_bit_offset_bound` proves the 1-bpp MSB-first bit byte is inside the mask.
- `model.rs`: `Conv` (`pal_entry`, `rgb_seq`, `rgb555_seq`, `bitfield_seq`,
  `conv_word`, `conv`, byte-level `conv_rgb`/`raw_conv`), the orientation cell map
  (`map_row_of`, `orient_cell`), the eleven named manifest lemmas
  (`stride_roundup_safe`, `row_last_sample_within_stride`,
  `product_width_height_bound`, `pixel_byte_offset_bound`,
  `vertical_flip_bijection`, `bit_mask_contiguous_extract`, `scale_round_bounds`,
  `palette_stride_bounds`, `rle_monotone_position`, `rle_painted_at_most_N`,
  `and_bit_offset_bound`), and the T11 RLE command-parser model
  (`rle_cmd_len`/`rle_cmd_count` + progress/count-bound/no-retry lemmas).
- Connect: `colour` post = `pal_entry`; `raw_sample` post = `raw_conv`;
  `lemma_conv_rgb_eq_conv_word` and `lemma_raw_conv_eq_conv` identify `raw_conv`
  with the manifest's `Conv`.
- Witnesses: `t4_conversion`, `t4_raw_sample_eq`, `t5_rows`, `t11_termination`,
  `t11_rle_cmd_model`, `t11_decode_terminates` (runs the real decoders).
- Checks: clean full reverify `verification results:: 264 verified, 0 errors`
  (`runs/287.log`); `cargo test` 214/214 vectors, 84/84 corpus (58 accepted); a
  1500-case differential fuzz vs the frozen oracle agreed 1500/1500. Backups
  `dib.rs.m2b`, `model.rs.m2b`. See `PROOF-REPORT-M3.md`.

## Not done (M3's remainder -> M4a/M4b/M4c)
- T6, T7, T8 (exact raw/RLE/AND transitions and ghost state) are M4a. T9, T10,
  T12, T13 and the vector/mutation campaign are M4b/M4c. No 32-bit verifier run
  (argument by hand). The full `decode = MP` equivalence is M4b.

## M4a (partial — T6 proved; T7/T8 not)
- `model.rs`: the M4a models. `Write`, `replay_bytes_at`/`bytes_replay` (the
  exact byte level of the executable's `put`), `zero_bytes`/`pal_zero_bytes`,
  `cell_of`, `Img`/`well_formed`, and the raw model `raw_full`/`raw_gcur`/
  `raw_writes` + `lemma_raw_full_proj`/`lemma_raw_gcur_ok_shape`. The RLE machine
  `run_index`/`abs_index`/`run_good`/`abs_good`/`run_writes`/`abs_writes`/
  `rle_reachable` and `mr_full`/`mr_cur`/`mr_writes` + `lemma_mr_full_proj`.
  `MA`: `and_stride`/`and_bytes`/`image_size_ok`/`and_bit`/`and_writes`/`ma_mask`.
  `MP`: `mp_raw`/`mp_rle`/`mp_mask_phase`/`mp`/`accepted`. Plus the algebra
  lemmas `lemma_bytes_replay_{push,len,assoc}`, `lemma_run_writes_{unfold,empty,
  single,snoc,snoc_at}`, `lemma_abs_writes_{...}`, `lemma_run_good_{unfold,bound,
  all,prefix,from,all_ok,is_run_ok}`, `lemma_abs_good_{...}`, `lemma_quad_at_
  index`, `lemma_pal_zero_bytes_len`, `lemma_pal_zero_put`.
- `dib.rs`: `put` now carries `final(out)@ == replay_bytes_at(old, o, c)` (the
  frame condition the ghost write accumulator needs); `alloc_output` ensures the
  zero buffer; `raw_loop` (T6) has the manifest's quantification — arbitrary
  reachable start state, every finite stream, exact first error **and** ghost
  final cursor/position on failure, and `final(out)@ == bytes_replay(zero fill,
  raw_writes(...))`. `paint_run` is the T7 per-run half with the same ghost
  accumulator and the `run_good` partial-paint clause.
- Checks: clean full reverify `verification results:: 336 verified, 0 errors`
  (`runs/403.log`, 88 s); `cargo test` 214/214 vectors, 84/84 corpus (58
  accepted); frozen hashes unchanged; no escapes, no rlimit. Backups
  `dib.rs.m4a`, `model.rs.m4a` (and the required `*.m3`).
- **Not done:** T7's command loop (`decode_rle`) and T8 (`apply_and`/
  `finish_mask`) are not proved; `MA`/`mr_*` are defined and their standalone
  lemmas prove, but the exec contract is incomplete. See `STUCK.md`.
  (Closed in M4a3 below.)

## M4a3 (Fable — T7 and T8 closed)
- `model.rs`: `mr_gcur` (cursor/position projection of `mr_full` on both
  exits) + `lemma_mr_gcur_proj`; one single-unfolding lemma per command exit
  and per continuing command (`lemma_mr_exit_{truncated,run_bounds,run_index,
  eob,eol_bounds,delta_trunc,delta_bounds,abs_bounds,abs_trunc,abs_index}`,
  `lemma_mr_step_{eol,delta,run_gcur,abs_gcur}`); `mr` now returns the
  `Outside` marker off `rle_reachable`. MA: `explicit_alpha`, `ma_domain`,
  `ma` (the manifest's `MA`, `Outside` off-domain, else `ma_exact`),
  `alpha_one`/`alpha_replay` (the AND write list acting on alpha bytes only),
  `lemma_and_writes_prefix` + `lemma_ma_mask_connect` (ties `ma_mask`/
  `and_writes` to `ma`'s byte output; errors agree). `lemma_replay_bytes_at_
  other` (frame of one `put`). `lemma_ma_prefix_step`'s brittle nonlinear
  assert (the run-419 rlimit site) now calls `lemma_ma_source_cell`.
  `mp_mask_phase` applies `ma_output` in ICO (it returned the XOR plane
  unchanged — a latent MP bug M4b would have hit).
- `dib.rs`: `fill_rle` ensures `out == pal_zero_bytes`; `paint_abs` (absolute
  painter, mirror of `paint_run`, reads indices from the slice); `rle_command`
  (one command, every R14 exit via the model exit lemmas) → `rle_abs_packet`;
  `rle_loop` (T7 at an arbitrary reachable start state `(*old(pos), *old(x),
  *old(y))` with a ghost accumulator; variant `|b| - pos`); `decode_rle` =
  fill + `rle_loop` from `(pixel_start, 0, 0)`, postcondition `rle_exact` on
  `mr`'s own tuple: result, ghost `*gcur` and buffer = `MR`'s on both exits;
  `decode` passes a `gcur` local. `finish_mask` is now stated against `ma`
  (Outside branch `=> false`, discharged by `ma_domain`) plus the write-list
  clause from `lemma_ma_mask_connect`. Dead `paint_rle`/`paint` removed.
- Checks: clean full reverify `verification results:: 378 verified, 0 errors`
  (`runs/428.log`, 95 s); `cargo test --release` 214/214 vectors, corpus 84
  files / 58 accepted; frozen hashes unchanged; no escapes, no rlimit. Runs
  422–428 (422 parse error, 423/425/426 one error each, 424/427/428 green).
  Backups `dib.rs.m4a3`, `model.rs.m4a3` (= the run-421 state).
- Not done (as scheduled): T9/T10/T12/T13, L1–L3, vector agreements, mutations.

## M4b (partial — T9, T10, T13, L1, L2 proved; T12 not)
- `dib.rs`: the public `decode` carries `decode_mp_eq(r, bytes@, mode)` with no
  `requires` (T9): it ties `parse_header` (T2, `hdr_matches`/`hrec`), the raw
  helper's `raw_done_eq` (T6), the RLE helper's `rle_exact` (T7) and
  `finish_mask`'s `mask_done_eq` (T8) to `mp`'s own phases via
  `lemma_mp_raw_assemble`/`lemma_mp_rle_assemble`/`lemma_mask_done_ok`, and
  folds every error route back. Witnesses `t10_error_precedence` (T10) and
  `t13_success_bounds` (T13); helper `lemma_hrec_matches`.
- `model.rs`: bridges `lemma_mp_mh_err`, `lemma_mh_palette_end`,
  `lemma_mp_ok_shape`, `lemma_mp_mask_phase_ico_ok`,
  `lemma_mp_mask_phase_size_err`, `lemma_mp_raw_err`, `lemma_mp_rle_err`,
  `lemma_mp_mask_phase_err_raw`, `lemma_mp_mask_phase_err_rle`, plus the L1/L2
  lemmas `lemma_mp_l1` (oversized input) and `lemma_mp_l2` (the exact-cap zero
  input), and the length helpers `lemma_ma_output_len`,
  `lemma_mp_mask_phase_len`, `lemma_mr_raw_out_len`, `lemma_mr_out_len`.
- Checks: clean full reverify `verification results:: 403 verified, 0 errors`
  (`runs/626.log`, 87 s); `cargo test --release` 214/214 vectors, corpus 84
  files / 58 accepted; frozen hashes unchanged; no escapes, no rlimit.
  Backups `dib.rs.m4a3b`, `model.rs.m4a3b`.
- **Not done:** T12 (linear work) on the executable. The model-side bounds and
  the leaf-loop ghost counters were written and discharged in intermediate
  runs, but the RLE command/loop coupling did not close and the whole T12
  instrumentation was reverted to the last green checkpoint (so the delivered
  files carry no step-counter clause). See `STUCK.md` — obstacle 1: a
  `&mut Ghost(local)` temporary does not alias the caller's ghost local
  (probe runs 559-561); obstacle 2: the exact RLE coupling pushed
  `decode_tracked` past the per-function verifier resource limit (runs 622-624),
  and no `#[verifier::rlimit]` is permitted.

## M4c (partial — vector agreements, mutations; L3 not closed)
- `dib_verus/src/model_vectors.rs` (generated by `tools/gen_model_vectors.py`,
  registered in `lib.rs`): 209 small-vector model-vs-expected obligations.
  181 are direct `by (compute)`; 28 go through the model's `channel_of` terms
  because Verus's `compute` normalizer will not unfold the `closed`
  `u32_trailing_zeros` that `channel_of` uses for 16-bit and BITFIELDS pixels
  (a `channel_lit` lemma relates each term to its concrete byte). The nine ICO
  32-bit successes are staged with a concrete `MHRecord`, `mr_raw_out` /
  `ma_output` steps; two of the nine (`ico-alpha-zero-fallback-mask`,
  `ico-fields-32-explicit-alpha-zero-and-fallback`) could not be closed and are
  **excluded** (they remain mandatory runtime conformance tests).
- `tools/gen_ico32.py` emits the nine staged ICO-32 obligations.
- Implementation mutations: `tools/mutations.py`, `tools/run_mutations.sh`;
  copies under `mutation/mN/` rebuilt from `mutation/base/` (== live src).
- Checks: clean reverify `verification results:: 1294 verified, 0 errors`;
  `cargo test --release` 214/214 vectors, corpus 84 files / 58 accepted;
  frozen hashes unchanged; no escapes, no rlimit.
- Not done: L3 (attempted as a `compute` obligation and as a `mr_gcur`
  induction; the `compute` form times out and the induction needs
  `run_good`/`index_ok` rewrites this agent did not complete). Two ICO-32
  fallback witnesses are runtime-only.

## usize width (coordinator note, 2026-11-09)
Checked on a standalone Verus file with the same toolchain (0.2026.11.02): `assert(usize::MAX == 0xffff_ffff_ffff_ffff)` FAILS while `assert(usize::MAX >= 0xffff_ffff)` verifies, i.e. Verus treats `usize` as a 32-or-64-bit type by default unless a crate declares `global size_of usize == 8` (none of the crates here do: grep for `global size_of`/`size_of usize` is empty). The proofs therefore hold for both widths in the sense of the manifest's 'parameterised by a word width'. This addresses the 'no 32-bit verifier run' review finding as a width-generic proof; it is not an actual i686 build or test run.

## M1.1a (done — R19 enum, helpers, models; crate version 0.2.0)
- **Semver break:** `Cargo.toml` version bumped 0.1.0 -> 0.2.0 (public `Mode`
  gains a variant; downstream exhaustive matches break).
- `dib.rs` and `model.rs` each gain `is_ico(mode)` ("either ICO mode") and
  `exact_tail(mode)` ("rejects bytes after the structure", i.e. `BareDib` or
  `IcoEntryDib`); `model.rs` adds `old_mode` and `with_mode`. `Mode` now has the
  fourth variant `IcoEntryDibSlack`.
- **Site classification.** Every 1.0 `x == Mode::IcoEntryDib` test was classified
  as *shared ICO handling* (rewritten to `is_ico(x)`) or *exact-tail handling*
  (rewritten to `exact_tail(x)`). In `dib.rs` the 32 IcoEntryDib sites are: shared
  ICO handling at the `visible_height` dispatch, the R3 even-height rule, the R3
  visible-height arithmetic, the R4 orientation rule (`ico` local), the
  `spec_is_ico` header predicate, the 32-bpp alpha read in `raw_sample`, the
  `mask_stride`/`and_bytes` computation, the whole `finish_mask` ICO body
  (shared, storage), and the mask-phase reasoning in `decode_tracked`; the only
  *exact-tail* site is the final `end != b.len()` cursor test in `finish_mask`
  (and its `mask_done_eq`/`decode_tracked` error-branch mirrors). In `model.rs`
  the 16 sites are the shared `visible_height_of`, `r2r5` (even-height,
  orientation), `conv_word`/`conv_rgb` 32-bpp alpha, `ma_reachable`, `ma_domain`,
  `mp_image_size_ok`, and the `mp_mask_phase` ICO branch; the only *exact-tail*
  site is `mp_mask_phase`'s `exact_tail(m.mode) && end != b.len()` test.
- **Models MH/MP/MA/MR/Conv extended.** `MH`'s record keeps the exact typed mode
  (a new field, `with_mode` relates the two ICO records field-for-field);
  `MP` skips *only* the final `cursor==|b|` test for `IcoEntryDibSlack`, after a
  complete AND mask; `MA`/`MR`/`Conv` share the old ICO rules.
- **New model theorems (all prove, no `requires` escape):**
  `old_modes_unchanged` (on the three 1.0 modes `is_ico`/`exact_tail` reduce
  definitionally to the 1.0 tests, so the models are unchanged);
  `slack_mode_same_prefix` (the `MH` record for Slack is the `IcoEntryDib`
  record with only `mode` changed); `slack_mode_suffix_phase` (the two modes'
  mask phases differ **only** where the exact-tail mode returns `TrailingBytes`,
  then Slack returns the completed `Ok((end, ma_output))`); and
  `slack_mode_suffix_result` at the whole-`MP` level (same statement lifted
  through the raw/RLE phases, via `ico_pair` congruence lemmas).
- **Executable preservation.** `finish_mask` keeps the shared ICO body; the new
  mode reaches the same code as `IcoEntryDib` with the exact-tail difference
  isolated in one exec boolean `exact` and one `if exact && end != b.len()`.
  All T1-T13, L1-L3 still verify.
- **Conformance:** `tests/conformance.rs` now reads
  `../spec-1.1-frozen/VECTORS.json` (302 vectors) and teaches its mode parser
  `IcoEntryDibSlack`; asserts 302/302.
- Checks: clean full reverify `verification results:: 1398 verified, 0 errors`
  (`runs/038.log`, after `cargo clean`); `cargo test` conformance 302/302, corpus
  84 files / 58 accepted; frozen 1.1 hashes match FREEZE.txt; spec-frozen and
  THEOREMS.md unchanged; no escapes, no solver-budget attribute. Backups
  `dib.rs.m11`, `model.rs.m11`, `model_vectors.rs.m11`.
- **Deferred to M1.1b/M1.1c (not done here):** (a) *integrated* T9/T10/T13 for
  the new mode is already complete — `decode_mp_eq` quantifies over `mode`
  universally and `finish_mask`'s `mask_done_eq` now covers the slack branch, so
  no gap remains for T9/T10/T13; (b) `model_vectors.rs` is unchanged (the 214
  old `agree_*` proofs still verify against the extended model; regeneration for
  the 88 new vectors is M1.1c); (c) the named non-vacuity witnesses
  `slack-1-new`, `slack-1-old`, `image-size-9-slack`, `mask-short-3-new`,
  `invalid-size-and-short-new`, `alpha32-7f-mask80-new` are M1.1b/M1.1c;
  (d) the delta's implementation mutations and the public `decode_with_steps`
  export are M1.1c. `model_vectors.rs` was therefore NOT edited (no regeneration
  needed to keep its 209 `agree_*` obligations verifying).

## M1.1c (done — witnesses, tracked-decoder export, mutations)
- **`model_vectors.rs` regenerated** from `spec-1.1-frozen/VECTORS.json` (the
  generator now reads the 1.1 file and knows both ICO modes): **294** `agree_*`
  obligations (254 literal, 24 term-form, 16 staged 32-bpp ICO), up from 209.
  `tools/gen_ico32.py` is mode-parameterized, so the staged 32-bpp form serves
  `IcoEntryDib` and `IcoEntryDibSlack`. The 20 proof-gate witnesses are kept; the
  six new non-vacuity witnesses are proved (`agree_slack_1_new`, `agree_slack_1_old`,
  `agree_image_size_9_slack`, `agree_mask_short_3_new`,
  `agree_invalid_size_and_short_new`, `agree_alpha32_7f_mask80_new`). The seven
  large vectors are covered by L1/L2 or runtime conformance; no 64 MiB input is
  expanded into verifier terms.
- **`decode_with_steps` exported** (`dib.rs`): the former private
  `t12_linear_work` body, with the ghost step result and the proved
  `decode_mp_eq` and `steps <= 16*(|b|+N)` postconditions; `t12_checked_n` became
  `pub open spec fn`; `lib.rs` re-exports it; `tests/conformance.rs` decodes
  through it. `decode` is unchanged and still carries `decode_mp_eq`.
- **44 implementation mutants** (35 historic + 8 delta items 36-44; item (5) has
  two arms). Six stale anchors were re-pinned after the M1.1a re-indentation.
  43 are verifier kills; mutant 20 (second `vec!`) is an **audit** kill
  (`tools/audit_alloc.sh`). m38 was corrected (it called the spec fn `exact_tail`
  from exec code, a rustc mode error) and rerun as a verifier kill. Previous
  campaign preserved in `mutation-1.0/`; `mutation/base` refreshed and byte-identical
  to the live sources.
- Checks: clean full reverify `verification results:: 1557 verified, 0 errors`
  (runs/098.log 139 s; runs/101.log 138 s, both after `cargo clean`); `cargo test`
  conformance 302/302, corpus 84/58; frozen 1.1 hashes intact; spec-frozen and
  THEOREMS.md untouched; no escapes, no solver-budget attribute; `audit_alloc.sh` OK.
  Backups `dib.rs.m11a`, `model.rs.m11a`, `model_vectors.rs.m11a`. See
  PROOF-REPORT-M1.1c.md.

## P1 (policy refactor, first slice; crate version 0.3.0)
- **`dib.rs` policy types.** `MetadataOptions {Declared, Ignore}`, a two-field
  `Policy {file_size, image_size}` (room for later option groups), the
  `BmpProfile {Strict, Recommended}` selector (`Default` is `Strict`, the match in
  `profile_policy` is exhaustive so a new profile is a compile error, not a silent
  Strict default), and `BmpMode {BmpFile, BareDib}` (POLICY-SPEC gives the
  profiled entry point only the two BMP modes, so no ICO input can reach a policy
  branch and no mode is rejected at run time). Spec fns `strict_policy`,
  `recommended_policy`, `profile_policy`; exec wrappers `bmp_mode_mode`,
  `profile_policy_exec`.
- **`model.rs` policy layer.** `r1_prefix_policy` (R1's `bfSize` test governed by
  `file_size`), `mh_policy` (the R1–R7 model with that prefix, same `r2r7` chain
  and the same `MHRecord` type), `image_size_ok_policy`/`mp_image_size_ok_policy`
  (R12 with `image_size`), `mp_mask_phase_policy`, `mp_raw_policy`/`mp_rle_policy`,
  and `mp_policy`. `recovered` is not a model field: it is `false` for both P1
  profiles, so it cannot depend on input. `mask_done_policy_eq` and the lemmas
  `lemma_mp_policy_{mh_err,raw_err,rle_err,phase_err_raw,phase_err_rle,
  raw_assemble,rle_assemble,ok_shape}`, `lemma_mh_policy_bounds`,
  `lemma_mp_mask_phase_policy_len`, `lemma_mp_policy_phase`, plus the Strict bridge
  `lemma_{r1_prefix_policy_strict, mh_policy_strict, image_size_ok_policy_strict,
  mp_image_size_ok_policy_strict, mp_mask_phase_policy_strict, mp_raw_policy_strict,
  mp_rle_policy_strict, mp_policy_strict}` (each a definitional reduction at
  `strict_policy()`; no existing theorem is weakened).
- **`dib.rs` executable.** `bmp_prefix_policy` (R1 with the option) and
  `parse_header_policy` (reuse `dib_fields` + `dib_finish`, so they generalize to
  any policy and the record is `MH_policy`'s); `finish_mask_policy` (R12 with the
  option; no AND work in either BMP mode; unknown modes are unreachable and the
  proof says so); the public `decode_with_policy(bytes, bm, profile) ->
  (Result<Decoded, Error>, bool)` with `recovered == false`, `recovered_ok`,
  `decode_policy_mp_eq` (T9 = `MP_policy` on every byte) and the T3/T13 shape
  bounds. Its body reuses the existing `alloc_output`/`decode_raw`/`decode_rle`.
  `decode`, `decode_with_steps`, every existing theorem and the 302 vectors are
  untouched; `lib.rs` re-exports the new names.
- **Conformance.** `tests/policy_conformance.rs` runs `policy/POLICY-VECTORS.json`
  for `Strict` and `Recommended` (102 each, `recovered` checked); it sits next to
  the unchanged `tests/conformance.rs` (302) and `tests/corpus.rs`.
- **Deferred to later milestones (not P1).** GdkCompat-2.42.12 and
  ImageCrateCompat-0.25.10 (and the other sixteen options); any input that sets
  `recovered = true` (P1 proves `false` for both profiles); `#[cfg(test)]` custom
  records.

## P2a (policy refactor, second slice; INCOMPLETE)
- **Delivered tree is the P1 baseline.** `dib_verus/src/dib.rs` and `model.rs`
  are byte-identical to the P1 sources (verified run 280: 1594 verified, 0
  errors). The P2a work is preserved in `dib_verus/src/dib.rs.p2a-wip2` and
  `model.rs.p2a-wip2` (best state: 1633 verified, 3 errors) and the earlier
  `*.p2a-wip` (1724 verified, 10 errors).
- **Built in `*.p2a-wip2`.** Six new option enums (`PlanesOptions`,
  `ReservedOptions`, `OffsetOptions`, `HighCountOptions`, `HeadersOptions`,
  `MasksOptions`) and their `pub` fields on `Policy`; `strict_policy`/
  `recommended_policy`/`profile_policy`/`profile_policy_exec` carrying all
  eight fields; the model's `r1_prefix_policy` (reserved, offset), the
  accessors `width_i_of`/`stored_of`/`planes_of`/`depth_of`/`compression_of`/
  `used_of`/`comp_of`, `hsize_ok_policy`, `gdk_high_bound`,
  `table_count_of_policy`, `masks_ok_policy`/`default_rgb_masks`/
  `masks_after_policy`/`raw_masks_policy`, a new `r2r5_policy` (planes,
  headers, high_count) and `r6r7_policy`/`r6r7_result_policy` (masks,
  high_count, offset), `offset_effective`/`gdk_blank`/`gdk_rgb_reject`,
  `mh_recovered_so_far`/`recovered_policy`, and the Gdk branches in
  `mp_raw_policy`/`mp_rle_policy`/`mp_policy`; the executable's
  `dib_fields`/`dib_finish`/`bmp_prefix_policy` honouring the options,
  `policy_masks`, `decode_raw` parameterised by the pixel start `p0`,
  `fill_blank`, and `decode_with_policy` returning the model's
  `recovered_policy`. All the Strict-bridge lemmas for R1/R2–R5, the
  `r2r5_policy` per-branch error lemmas, `lemma_mp_policy_gdk_rgb`, the
  truncation lemmas and the new `lemma_mp_policy_ok_shape` verify.
- **Not done.** `dib_finish` → `r6r7_policy` on the success path and
  `lemma_r6r7_policy_strict` did not close; two per-function resource-limit
  failures on `dib_fields`/`dib_finish` accompany them. The P2a conformance
  extension (per-option oracle probes, generated vectors, per-option corpus
  acceptance) was not written. See `STUCK.md` and `PROOF-REPORT-P2a.md`.
- **Checks on the delivered baseline.** clean reverify 1594 verified, 0 errors
  (runs 276 and 280); `cargo test --release` 302/302, corpus 84/58, policy
  Strict/Recommended 102/102; `tools/audit_alloc.sh` OK; frozen inputs intact;
  no escapes, no `#[verifier::rlimit]`.

## P2b (policy refactor, third slice; INCOMPLETE — stopped at the escalation budget)
- **Delivered tree is the run-387 green state** (`dib_verus/src/dib.rs`,
  `model.rs`, `lib.rs`; backups `*.p2b-base2`): the P2a baseline (1658 verified,
  0 errors) plus the reviewer-note exposure fix and the fourteen new option
  enums with their `Policy` fields, every one at its Strict (bold) value. **No
  option beyond the P2a six is behaviourally implemented.**
- **Exposure fix (reviewer note).** `Policy`'s fields must stay `pub` for
  `model.rs`'s `pub open spec fn`s to be well-formed outside the crate (Verus
  treats any non-`pub` field as opaque and then rejects the spec-fn field
  expressions the public entry point's own `ensures` mentions — tried
  `pub(crate)`, 26 errors, run 295). The actual restriction is the constructors:
  `strict_policy_exec` and `profile_policy_exec` are now `pub(crate)`, and
  `Policy`/`MetadataOptions` are dropped from `lib.rs`'s exports, so no release
  caller can obtain or build a custom record; `#[non_exhaustive]` still blocks
  external literals. Flagged in `PROOF-REPORT`.
- **New types.** `ShortPixelsOptions`, `RleEolOptions`, `RleOverrunOptions`,
  `RleDeltaOptions`, `RleEobOptions`, `RleFillOptions`, `PaletteIndexOptions`,
  `ScalingOptions`, `UnsupportedOptions`, `BitfieldOffsetOptions`,
  `Bitfield16AlphaOptions`, `IncompleteMetadataOptions`, `ShortIndexedGapOptions`,
  `HeaderBytesOptions`, each `#[non_exhaustive]`, plus the fourteen matching
  `Policy` fields. All four `Policy` constructors carry them at the Strict value,
  so `strict_policy()`/`recommended_policy()`/`profile_policy_exec` are unchanged
  and `parse_header`'s Strict instance is byte-identical.
- **rlimit recovery.** The fourteen extra fields pushed `parse_header` over the
  per-function solver limit through its five redundant `reveal(strict_policy)`
  calls (the strict-bridge lemmas redo those reveals); removing the redundant
  reveals restored green (runs 297→298). The `model::strict_policy` spec-fn
  gaining fourteen fields is what widened those terms.
- **Not done.** `bitfield_offset = MetadataEnd` (the first of the fourteen) was
  attempted across runs 299–388 and is **not** proved: the `&Header`-reference
  field reads inside exec `if` conditions would not reduce to the same term as
  the `as int` projections the record asserts use, and the whole
  `decode_policy_tracked` then exceeded the per-function resource limit. See
  `STUCK.md` (P2b section) for the ten-plus variants and the suggested fix
  (compute the start from four plain values, or fold the option into the
  record's own `pixel_start` at construction). The other thirteen options are
  unimplemented. The `cfg(test)` oracle harness, the 26 unused
  `CUSTOM-VECTORS` rows, and per-option corpus counts are likewise not extended.
- **Checks on the delivered state.** clean reverify 1658 verified, 0 errors
  (run 387); `cargo test --release` 302/302 vectors, corpus 84 files / 58
  accepted, policy Strict/Recommended 102/102, the P2a `custom_policies_match_
  python_oracle` harness green; frozen inputs intact; no escapes, no
  `#[verifier::rlimit]`.

## P2c (policy refactor, fourth slice — the two compatibility profiles)
- **Delivered tree** (`dib_verus/src/dib.rs`, `model.rs` unchanged, `lib.rs`,
  `src/policy_tests.rs`, `tests/policy_conformance.rs`; backups `*.p2c-final`):
  run-442 green state, **1714 verified, 0 errors** (was 1712). All four frozen
  named profiles are selectable from release code.
- **`BmpProfile`.** `GdkCompat` and `ImageCrateCompat` variants added after
  `Recommended`; the enum was already `#[non_exhaustive]`, `Default` stays
  `Strict`.
- **Spec records.** `gdk_compat_policy()` and `image_crate_compat_policy()` carry
  every one of the twenty-two retained fields from the frozen POLICY-SPEC "Named
  profiles" table; `profile_policy` gained the two arms. `profile_policy_exec`
  gained the matching two arms with the same field values; the `Strict` and
  `Recommended` arms are **byte-identical** to the run-437 file (checked), so the
  P1 behaviour and its proofs are untouched. Six new verified obligations: the
  two spec-fn bodies and the two `profile_policy_exec` arms each discharge their
  `ensures` against `profile_policy` (the exec-constructor-equals-model equality,
  as for `Strict`/`Recommended`). The constructors stay `pub(crate)`.
- **Witness fix (task 3).** `t1_policy_totality` claimed `r.1 == false` for
  *every* profile; that is false for the two compatibility profiles, which carry
  recovery-producing options. Split into `t1_policy_recovery_flag` (all four,
  `r.1 == recovered_policy(..)`, the frozen closed-event semantics) plus
  `t1_policy_recovery_strict`/`_recommended` (concrete `BmpProfile::Strict` /
  `Recommended`, `r.1 == false`, the original P1 claim at its original
  quantification). No `model.rs` change was needed: `recovered_policy`,
  `mh_recovered_so_far`, `pixel_recovered`, `gdk_blank` were already written over
  an arbitrary `Policy`, and `lemma_mh_recovered_so_far_strict` stays as is.
- **Conformance (`tests/policy_conformance.rs`).** `policy_conformance_all_profiles`
  runs all four `POLICY-VECTORS.json` profiles, **102/102 each**, RGBA and
  `recovered` checked. `policy_conformance_corpus_acceptance` checks the 71-file
  BMP corpus per profile — **Strict 50, Recommended 52, GdkCompat-2.42.12 58,
  ImageCrateCompat-0.25.10 55** — and, for Strict, the frozen
  `corpus_expected.json` answer byte for byte.
- **Public-vs-custom equality (task 3, the fixture half).** In
  `policy_tests::custom_policies_match_python_oracle`, every oracle row whose
  group is `profile-fixtures/*`, `profile-prefixes/*` or `profile-corpus/*`
  (26,238 rows over the two compatibility profiles) asserts both that
  `profile_policy_exec(profile)` equals the record `policy_of` builds from the
  oracle's own assignment (so the release and test-only full assignments cannot
  diverge) and that `decode_with_policy(bytes, mode, profile)` equals
  `decode_custom_policy(bytes, mode, that_record)`. This is a crate unit test, not
  an integration test: `decode_custom_policy` and the constructors are
  `pub(crate)` + `#[cfg(test)]`.
- **Task 4, flagged not done.** The `(Result<Decoded, Error>, bool)` return shape
  is unchanged; a note on `decode_with_policy` records that folding `recovered`
  into a `ProfiledDecoded` struct is a proof-neutral API-shape change.
- **Checks on the delivered state.** clean reverify **1714 verified, 0 errors**
  (runs 438/440/441/442); `cargo test --release` 302/302 vectors, corpus 84 files
  / 58 accepted (Strict's answer), the four-profile policy conformance above, and
  the extended `custom_policies_match_python_oracle` harness green;
  `tools/audit_alloc.sh` OK (one `vec!`); frozen policy hashes match
  `policy/FREEZE.txt`; no escapes, no `#[verifier::rlimit]`. `model.rs` and
  `lib.rs` sources are unchanged from the run-437 green state.

## P2d (the loader's `has_alpha` field on the verified result)
- **What the loader needs.** Stock gdk-pixbuf 2.42.12 sets `has_alpha` from the
  header alone: TRUE exactly when `biBitCount == 32` or `biCompression` is RLE8
  (1) or RLE4 (2); it never inspects a pixel. `gdkpixbuf-loaders/src/bmp.rs`
  currently derives `has_alpha` from `d.rgba` (any non-255 alpha) because
  `Decoded` carried no header field.
- **Delivered tree** (`dib_verus/src/dib.rs`, `model.rs`, `src/policy_tests.rs`,
  `tests/conformance.rs`, `tests/policy_conformance.rs`; backups `*.p2d-0`
  pre-change, `*.p2d-1` green): run-469 clean state, **1722 verified, 0 errors**
  (was 1714, +8).
- **The field.** `Decoded` gains `pub rle_or_32bpp: bool`. It is the small change
  the brief asks for: `model.rs`'s `gdk_blank`/`partial_fill` already select the
  transparent fill with `bits == 32 || compression == 1 || compression == 2`, so
  the same predicate is now also a returned value. The Strict/Recommended/Gdk/
  Image RGBA bytes are untouched: no pixel path reads the field, and every
  existing postcondition (`decode_mp_eq`, `decode_policy_mp_eq`,
  `recovered_policy`, the T12 work bound, the one-`vec!` allocation) is kept.
- **Model predicates.** `rle_or_32bpp(depth, compression)`,
  `mh_rle_or_32bpp(b, mode)` (the policy-free tie) and
  `mh_policy_rle_or_32bpp(b, bm, p)` (the profiled selection: the partial
  `RawFields` under `incomplete_metadata = Blank`, else the complete `MH_policy`
  record, `false` on error). The two branch unfolds are named lemmas
  (`lemma_mh_policy_rle_or_32bpp_partial`/`_complete`), as is
  `lemma_mh_partial_none`.
- **Postconditions.**
  `decode`: `Ok(d) => d.rle_or_32bpp == mh_rle_or_32bpp(bytes@, mode)`.
  `decode_with_policy` and its helpers: the new spec fn
  `decode_policy_alpha_eq(r.0, b, bm, p)`; the profiled entry point, the tracked
  and complete helpers, `decode_partial_metadata`, the two `finish_*` helpers,
  and `decode_custom_policy` all carry it. Witnesses `t13_alpha_field` (policy
  free) and `t9_policy_alpha` (all four profiles) state the same clauses at the
  manifest's full quantification; `t13_policy_success_bounds` gained the clause.
- **One budget note (no `#[verifier::rlimit]`).** Naming
  `mh_policy_rle_or_32bpp` directly in `decode_complete_policy`'s `ensures` made
  the solver unfold `mh_policy` at every return site and pushed that
  (already-large) body past its default rlimit. The fix keeps the model tie but
  moves it: `decode_policy_tracked` computes `alpha = header_alpha(&h)` and runs
  `lemma_alpha_value` (with `lemma_complete_alpha`) to get
  `alpha == mh_policy_rle_or_32bpp(...)`, then passes `alpha: bool` into
  `decode_complete_policy` and the two `finish_*` helpers, whose clauses read
  `d.rle_or_32bpp == alpha`. So the model predicate appears only in the small
  functions, the .rs has exactly one `reveal` of it, and no function needs a
  larger solver budget.
- **Conformance.** `tests/conformance.rs` (302 vectors) and
  `tests/policy_conformance.rs` (102 per profile) each recompute the rule from
  the input's own `biBitCount`/`biCompression` bytes (DIB start 14/0; CORE and
  16-byte layouts imply `biCompression = 0`) and assert it equals
  `d.rle_or_32bpp` on every success. `src/policy_tests.rs`'s `check` does the
  same on every custom-oracle row. The new
  `gdkcompat_alpha_field_matches_header_rule_on_corpus` decodes all 71 BMP
  corpus files under `GdkCompat` and asserts the field matches the independently
  read rule on each of the 58 the profile accepts (the other 13 return no
  `Decoded`).
- **Checks on the delivered state.** clean reverify run 469 **1722 verified, 0
  errors**; `cargo test --release` 302/302 vectors, four-profile policy
  conformance 102/102 each, corpus acceptance Strict 50 / Recommended 52 / Gdk
  58 / ImageCrate 55, `custom_policies_match_python_oracle` 26,238 profile rows
  green; `tools/audit_alloc.sh` OK (one `vec!`); frozen policy hashes match
  `policy/FREEZE.txt`; no escapes, no `#[verifier::rlimit]`.
  `gdkpixbuf-loaders/` was not touched (a later step wires the module to the
  field).
