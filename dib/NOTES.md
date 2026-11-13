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

## alloc-delta (fallible allocation) — FIRST SLICE, **INCOMPLETE / NOT GREEN** (2026-11-12)

The coordinator restarted the sandbox mid-step; this section records exactly
what is done, what is not, and the next step. **The live tree is the P2d
baseline (green).** The `try_*` rewrite described below is set aside in
`dib/alloc-wip-1/` (not built, not verified).

### Tree state right now

- `dib_verus/src/{dib.rs,model.rs,lib.rs,policy_tests.rs}` and
  `dib_verus/tests/{conformance.rs,corpus.rs,policy_conformance.rs}` are the
  P2d baseline, restored from the `.alloc-0` backups (SHA-256
  `dib.rs = 6cef9186bc7a2165ebd0656cc1371b5d2051b6afa7abb644c43718c26890f68b`,
  `model.rs = f3dd49ca95b325ab7ee5c4a89746871c8b62f6f32c48a63f217dc29262288845`).
- `dib_verus/src/alloc_guard/` does **not** exist in the live tree (it lives in
  `alloc-wip-1/alloc_guard/`).
- Clean reverify of the restored tree: **run 519, 1722 verified, 0 errors, exit 0**.
- `cargo test` on the restored tree: 302/302 frozen vectors, four policy
  profiles 102/102 each, corpus 84 files / 58 accepted Strict, `66/66` custom
  vectors, `custom_policies_match_python_oracle` 26,238 profile rows, GdkCompat
  corpus alpha check — all pass. `tools/audit_alloc.sh` OK (exactly one `vec!`).
- The tree therefore compiles, verifies and passes its tests: any other agent
  and the audit prototype see a working DIB.
- Backups: `*.alloc-0` beside each source is the frozen P2d baseline. The WIP
  set is `alloc-wip-1/`.

### What the WIP implements (in `alloc-wip-1/`)

Part (1) of the task — the production `alloc_guard` — is written and **verifies
in isolation**: `alloc-wip-1/alloc_guard/mod.rs` (sealed `AllocLog`, `new_log`,
`OutOfMemory` with private fields and `when_used_as_spec` accessors, `all_ok`,
`req_of_log`, `MAX_BYTES`, `#[inline(never)] try_zeroed` with the prototype's
exact push-fill body) and `alloc-wip-1/alloc_guard/sites.rs` (DIB `AllocSite
{ Output }`, `TryError { Format(Error), OutOfMemory(..) }` with matching
`name_spec`, `fmt_part`, `log_tied`, reaching `Error`/`Decoded` through
`super::super::dib`). The `mod.rs` is DIB-adapted from the prototype: DIB has one
site, so `Reserved`/`try_copy`/`try_with_room`/`MAX_ROOM` are dropped, and the
per-core `Error`-carrying items live in `sites.rs` so `mod.rs` stays
core-independent (SPEC §3, D6). SHA-256 of the WIP set:

> Superseded by ERRATA E6 (2026-11-13): the guard fill is now `resize(n, 0u8)`, not a `push` loop; see the E6 section at the end of this file.

```
5d145f45adcf8d1c5ab398f2ac2ec0d50eb3bcbd54a542b277d7b86a41ae33b1  alloc-wip-1/alloc_guard/mod.rs
21455401aefc2fbdacf5cae33c28e8ede4ac609d0b42f684932ae638be8e4875  alloc-wip-1/alloc_guard/sites.rs
ea1345c6370f70401256b174e4877b8fb7b2a5b6442f20f6e948051cd0b83eb0  alloc-wip-1/dib.rs
20d864d667dda63b811ac66c50b739f558226ef6b76b7c2b08ec5f0c1f254b58  alloc-wip-1/lib.rs
6b6489263221f1b7021accd9cbda4d75738032dadd677950882b90506c2db628  alloc-wip-1/policy_tests.rs
d848d2132d53c0fee90d8bab2dd928d1c66d776152ee2e15d0dab8e0a9f3f019  alloc-wip-1/conformance.rs
8f771e50ff23b3b980021d2189d5ce1fb320372e0342bec713dc205fa4f4f3fc  alloc-wip-1/corpus.rs
8369fd590e00ae5753bfc5ac81fcac2bbdfa34009db115bf8febd5b99a96fa85  alloc-wip-1/policy_conformance.rs
```

The live `dib_verus/src/alloc_guard/` and the live sources are the P2d baseline;
the WIP files above are the continuation's starting point.

Part (2) — the `try_*` rewrite — is **partial**:
- `decode_tracked` takes the log, calls `try_zeroed` through a guarded
  `alloc_output`, and returns `Result<Decoded, TryError>`. The `vec!` site is
  gone from the WIP.
- `try_decode_with_steps_logged` / `try_decode_with_steps` / `try_decode`
  replace `decode_with_steps` / `decode`; the log is created with `new_log`,
  returned from the `_logged` form.
- The DIB spec surface (`req_dib`, `g`, `prog`, `pre_alloc`, `log_tied`,
  `at_site_log` with helper lemmas) is written.
- **Not done:** the policy path is only half rewritten (`alloc_output_n` and
  `decode_partial_metadata` are threaded and the `_policy` spec surface
  `policy_shape`/`g_policy`/`prog_policy`/`req_dibp`/`policy_reach`/
  `partial_revalidated`/`lemma_blank_second_pass` is written, but
  `decode_policy_tracked`, `decode_complete_policy` and
  `finish_policy_after_alloc` do not yet discharge their contracts), and the
  witness section is only partly retargeted.

### Exact verifier state of the WIP

Best WIP run: **run 517 — 1737 verified, 14 errors** (log `runs/517.log`).
`runs/518` is a one-line compile slip on top of 517. The remaining errors are
all in the policy half and are enumerated from `runs/517.log`; the decode half
(1 site, `G`, `Prog`, `PreAlloc`, `log_tied`) is essentially through.

### Next step for a continuation agent

1. Copy `alloc-wip-1/*` back over the live sources, and
   `alloc-wip-1/alloc_guard/` back to `dib_verus/src/alloc_guard/`.
2. `./verify.sh` and read `runs/517.log` (or the newest log) for the current
   error list. It is: `finish_raw_zero`/`finish_rle_clipped` (add the
   `policy_shape` discharge at their `InvalidImageSize`/`TrailingBytes` exits),
   `finish_policy_after_alloc` (the precondition `alpha == mh_policy_rle_or_32bpp`
   and the `steps` arithmetic), and `try_decode_with_steps`'s `OutOfMemory`
   clause (state the site only, as it now does).
3. **Watch the per-function resource limit.** Naming a model predicate in a
   `_policy` helper's `ensures` made the body exceed its default rlimit. The
   fix, applied in P2d and needed again here, is the `alpha: bool` threading
   pattern (NOTES.md P2d): compute the model value in a small function, pass the
   concrete value into the large one. `raw_sample_scaled` also reports an rlimit
   in the WIP runs although its body is byte-identical to the baseline; the
   likely cause is the `#[verifier::when_used_as_spec]` added to `Error::name`
   changing the crate-wide encoding. Try dropping that attribute and stating
   `TryError::name`'s postcondition with the `Error` spelling table inlined in
   `sites.rs` instead.
4. Parts (3)–(5) — the full A-DIB-1..8 obligations, the test migration to
   `try_*`, the fault-injection tests and the `audit_alloc.sh` extension — are
   **not started** beyond the partial witness retargeting in the WIP.

`alloc-delta/SPEC.md`, `THEOREMS-ALLOC-DELTA.md` and `TABLE.md` were read and
were **not** edited, as instructed. No spec edit was found necessary; the one
divergence worth reporting is that DIB's per-core `AllocSite` had to live in
`sites.rs` (not `mod.rs`) so that `mod.rs` stays byte-identical across cores,
which is what SPEC §3/D6 wants but `SPEC.md` §3's prototype listing does not say
explicitly.


## alloc-delta continuation (2nd agent, 2026-11-12) — STALLED on the policy rlimit

**Live tree is the green P2d baseline.** The WIP is `dib/alloc-wip-2/` (the
run-530/531 state; see below), not built and not verified. Do **not** treat any
`try_*` obligation below as delivered.

### What was done in this session

1. Copied `alloc-wip-1/{dib.rs,lib.rs,policy_tests.rs}` over `dib_verus/src/`,
   `alloc-wip-1/{conformance.rs,corpus.rs,policy_conformance.rs}` into
   `dib_verus/tests/`, and `alloc-wip-1/alloc_guard/*` to
   `dib_verus/src/alloc_guard/`.
2. **Applied the hazard fix.** Dropped `#[verifier::when_used_as_spec]` from
   `Error::name` (`dib.rs`) and from `TryError::name`
   (`alloc_guard/sites.rs`), and gave both `name` exec functions no spec clause.
   The `name_spec` spelling tables stand alone (the tests call `name()` in exec
   position only). No `raw_sample_scaled` rlimit appeared in any run after this
   — the hazard is closed. Also removed a stray
   `reveal(crate::alloc_guard::AllocSite);` line (E0423: `AllocSite` is a type,
   not a module path) that was the run-518 compile slip.
3. Walked the 14-error list of run 517 down to **2 errors** (run 530: 1749
   verified, 2 errors):

   - `try_decode_with_steps`'s `OutOfMemory` clause: fixed (the site-only form
     the prior agent had already written).
   - `finish_raw_zero` / `finish_rle_clipped`'s `policy_shape(...false...)`
     postcondition: fixed. The two helpers' opening proof blocks now establish
     `policy_checked_n(b,bm,p) == h.spec_n()` (from `hdr_matches` + `mh_partial
     is None`) and call `lemma_shape_pushed_ok` once on `old(log).view()`; the
     log is never touched by any pixel/RLE helper, so `policy_shape` holds at
     every exit.
   - `finish_policy_after_alloc`'s `alpha == mh_policy_rle_or_32bpp(..)`
     precondition: fixed by taking `alpha` (the concrete header value) and the
     `alpha == rle_or_32bpp(spec_depth, spec_compression)` tie, exactly the P2d
     pattern; `decode_complete_policy` derives it from `lemma_alpha_value`.
   - `decode_complete_policy`'s `steps` arithmetic: fixed with `pre =
     start_steps` and a direct re-derivation.
   - The `InvalidPixelOffset` (`from_gdk_rgb`) exit and the header-error exit:
     `policy_shape` established via `lemma_shape_prealloc`.
   - Witnesses `t10_error_precedence`, `t13_success_bounds`, `t13_alpha_field`,
     `t10_policy_precedence`, `t13_policy_success_bounds`: the manifest's
     quantification forces the `!OutOfMemory` antecedent (SPEC §1 clause 1 is
     only about non-`OutOfMemory` results). T13's bounds and the `has_alpha`
     field now also appear on `try_decode`/`try_decode_with_steps_logged`, and
     each witness uses a by-reference `match` to case-split `r` without moving
     it.
   - `decode_partial_metadata`: `revalidate_partial` now exports
     `policy_checked_n == f.spec_n()` and `1 <= f.n <= 16777216`;
     `alloc_output_n` requires an empty log.
   - `decode_complete_policy`/`finish_policy_after_alloc`'s log precondition
     is now the one-`Ok`-entry sequence (the reservation already happened),
     not `empty()`.

### The STALL: per-function rlimit on the policy pair

The last 2 failures (still present in run 530) are **per-function resource-limit
failures**, not false statements:

```
error: function body check: resource limit (rlimit) exceeded
    --> src/dib.rs:9691  fn decode_complete_policy
    --> src/dib.rs:9858  fn finish_policy_after_alloc
```

Runs 531–535 (five runs) each tried to split these bodies under the default
budget, never adding `#[verifier::rlimit]` (forbidden). Per the brief's stall
rule I stopped at five and reverted to the run-530/531 state.

- **531** split the `from_gdk` blank branch into `finish_policy_blank`: 1750
  verified, 2 errors. `finish_policy_after_alloc` still rlimit; the new
  `finish_policy_blank` verified. This is the best structural state.
- **532** added `policy_gdk_flags` to shrink `decode_complete_policy`: added a
  precondition failure (`policy_indexed_gap` needs `header_inv_light`) and an
  `h.is_bmp == (h.mode == BmpFile)` assert failure; net worse.
- **533/534** added named `lemma_policy_phase_success` for the final phase
  assembly: the raw/RLE arm needed `raw_done_eq`/`rle_exact` hypotheses and the
  `mr_out`/`mr_raw_out` derivations re-asserted; 1751/3, worse.
- **535** extracted `finish_policy_plain` (the plain decode+mask tail):
  1751/4, worse.

So the only split that helped was **`finish_policy_blank`**; the two functions
are still over budget. The WIP set saved is `alloc-wip-2/` at the run-531
structure (== run 530 plus `finish_policy_blank`).

### Exact next step (for the next tier)

In `dib/alloc-wip-2/dib.rs`, the two over-budget functions are:

1. `decode_complete_policy` — its body is the `ghost_mh_policy` +
   `lemma_policy_header_geometry` opening proof block, the Gdk-flag
   computation, `alloc_output`, and the call to `finish_policy_after_alloc`.
   The opening proof block (the `hrec`/`hdr_matches`/palette-geometry `assert`
   chain) is the bulk. **Move that whole block into a named `proof fn`
   `lemma_complete_policy_geometry(b, bm, p, h, m)`** whose `ensures` carry the
   equalities it currently `assert`s (`hrec(h) == m`, `m.palette_end == h.spec_palette_end()`,
   `policy_checked_n(..) == h.spec_n()`, the palette-fit disjunction). That is
   the same move that fixed `decode_tracked` in T9.
2. `finish_policy_after_alloc` — its body now has (a) the opening
   `policy_shape` block, (b) the `from_gdk` dispatch (already split to
   `finish_policy_blank`), (c) the raw-dispatch to `finish_raw_zero`, (d) the
   truncation exit, (e) the RLE dispatch to `finish_rle_clipped`, (f) the plain
   decode + `finish_mask_policy` tail. **Split (f) into `finish_policy_plain`
   *with its own opening `lemma_mh_partial_none`/`lemma_shape_pushed_ok` block*
   (run 535's version failed only because it re-derived nothing and the model
   assertions in `lemma_policy_phase_success` were not re-established there).**
   Keep the phase assembly inline or move it to a named `proof fn` whose
   `requires` include `raw_done_eq(...)`/`rle_exact(...)` for the matching arm
   (run 533's `lemma_policy_phase_success` had the right shape but wrong
   preconditions; the raw arm needs `mr_raw_out(..) == Ok((pos,out_xor))` derived
   from `lemma_raw_full_proj` on `hrec(h)`, and the RLE arm needs
   `mr_out(b,m,m.pixel_start,0,0) == Ok((pos,out_xor))` from `rle_exact`).

No `#[verifier::rlimit]`. The alternative when splitting stalls: fold the
`recovered`/`mp_policy` model equation (the `by { reveal(recovered_policy); ... }`
block in `finish_policy_blank` and the tail) into a `proof fn` taking
`m: MHRecord as a value`, per the P2b note's "fold into a helper" advice.

### State of the rest

`alloc-wip-2/{dib.rs,lib.rs,policy_tests.rs,alloc_guard/*}` and the three
`tests/*.rs` are on disk. The tests are already migrated to `try_*` in the WIP
(`tests/conformance.rs`, `tests/corpus.rs`, `tests/policy_conformance.rs`,
`src/policy_tests.rs` all call `try_decode*` and fold `TryError` with a
`fmt_part` helper that panics on `OutOfMemory`). Not started: the fault-injection
tests, the `tools/audit_alloc.sh` extension, and the no_std conversion.

### WIP-2 hashes (the state that verified at run 536: 1750/2)

```
2451b1656e71a5b043dd17da21b493c0ff0c78b5febeb6b3c330a17740137709  alloc-wip-2/dib.rs
5d145f45adcf8d1c5ab398f2ac2ec0d50eb3bcbd54a542b277d7b86a41ae33b1  alloc-wip-2/alloc_guard/mod.rs
33465c740e10162d0d6634afd0a6033e26f3d0f523b40e375f4b1d46bcdf000a  alloc-wip-2/alloc_guard/sites.rs
20d864d667dda63b811ac66c50b739f558226ef6b76b7c2b08ec5f0c1f254b58  alloc-wip-2/lib.rs
6b6489263221f1b7021accd9cbda4d75738032dadd677950882b90506c2db628  alloc-wip-2/policy_tests.rs
d848d2132d53c0fee90d8bab2dd928d1c66d776152ee2e15d0dab8e0a9f3f019  alloc-wip-2/conformance.rs
8f771e50ff23b3b980021d2189d5ce1fb320372e0342bec713dc205fa4f4f3fc  alloc-wip-2/corpus.rs
8369fd590e00ae5753bfc5ac81fcac2bbdfa34009db115bf8febd5b99a96fa85  alloc-wip-2/policy_conformance.rs
```

`alloc-wip-2/dib.rs` is the run-531 structure (run 530 plus the
`finish_policy_blank` split). Run 536 (`VERIFY_CRATE=alloc-verify-1`) verified
it at 1750 verified, 2 errors; the two are per-function rlimits on
`decode_complete_policy` and `finish_policy_after_alloc`.

## alloc-delta continuation (Sol, 2026-11-12) — stalled after five runs

The live `dib_verus/` tree is restored byte-for-byte to its P2d sources (`dib.rs` SHA-256 `6cef9186bc7a2165ebd0656cc1371b5d2051b6afa7abb644c43718c26890f68b`). The third try_* snapshot is in `alloc-wip-3/`, including its `alloc_guard` and tests; its `dib.rs` SHA-256 is `52bcb266b432dab68a2b55a6e112ed29aa9ab85c6d35e771305dcc3083aa1e64`. Green backups for each replaced source and test are `*.alloc-s1` in the live crate. The older `alloc-wip-2/` remains intact.

Runs 538–542 each failed on the same two per-function resource limits. Run 542 reached 1757 verified, 2 errors; it also exposed a special raw work-bound assertion failure. Work preserved in `alloc-wip-3/dib.rs`: `lemma_complete_policy_geometry`, `lemma_complete_policy_gdk_flags`, `lemma_complete_policy_rgb_prealloc`, `lemma_complete_policy_pushed`, and the split `finish_policy_plain` with explicit raw/RLE model-result derivations. The plain helper now verifies; neither large dispatcher verifies. `STUCK.md` records all five attempts and the needed control-flow split. **Do not use the WIP as a verified crate.**

Audit finding, not fixed in the WIP: both allocation helpers charge `4N` on failed reservations, contrary to the frozen successful-reservation-only step rule. Fix `alloc_output` and `alloc_output_n` in both their postconditions and their `Err` bodies before claiming A-DIB-6, and adapt callers. No fault-injection test, source audit extension or no_std conversion was attempted because the stall rule requires restoring green and handing back after five failures on this obligation. No frozen alloc-delta document was edited.

Run 543 reverified the restored baseline: **1722 verified, 0 errors**. `cargo test --offline` passed all baseline test binaries (the conformance, corpus, policy and custom-policy tests); the first attempt omitted `/opt/cargo/bin` from PATH and did not run, the second used the verifier's PATH and passed. Removed `dib_verus/target/` after the check. The try_* snapshot was not run through the full suite and has no green verifier run.

## alloc-delta continuation (Astra, 2026-11-12) — live try_* and no_std

The live `dib_verus/` is now the fallible-allocation implementation, not the restored P2d baseline. Final **run 552: 1,765 verified, 0 errors**. The original two resource-limit failures closed in run 545 after splitting executable control flow: `reserve_complete_policy` owns pre-reservation Gdk rejection and allocation; `finish_policy_after_alloc` handles Blank dispatch and delegates the remaining dispatch to `finish_policy_pixels`. The previous tier's geometry lemmas and `finish_policy_plain` remain in use. Concrete `alpha: bool` threading remains intact. No budget attribute or proof escape was introduced.

The post-reservation helpers also needed an explicit non-OOM postcondition. Their old `fmt_part` relation and successful-log shape did not exclude an OOM result, because `fmt_part(OutOfMemory)` is unspecified. Both allocation helpers now charge initialization only on `Ok`; failed reservations add no `4N` charge. The special raw branch includes the initialization charge when composing its helper bound. `alloc_output_n` contains the sole production `try_zeroed` call; `alloc_output` delegates to it.

`try_decode_policy_tracked` is now part of the production verification rather than hidden behind `cfg(test)`. Its contracts quantify over every `Policy` record and expose full G/Prog, non-OOM format/alpha/recovery equality, bounded output, one-request shape, OOM site/size, and PreAlloc. The internal `decode_policy_tracked` carries the all-outcome `16*(|b|+N)` work bound; the logged wrapper explicitly asserts it before discarding its ghost counter. `revalidate_partial` invokes the four-exit Blank equivalence lemma. The Strict bridge compares independent calls only when both are non-OOM. The policy-free logged export gives ICO the returned tracked log, counter, `log_tied`, and the specialized `g_logged(b,r,L)`/`prog_logged(b,r,L)` predicates.

The spec exports `fmt_part` and `log_tied` are forwarding spec functions, not `pub use` aliases: ordinary Rust erases the aliased definitions but retains such imports, which caused the first landed cargo test to fail to compile. The forwarding definitions fixed that mismatch.

| Run | Result | Change or failure |
|---|---|---|
| 545 | 1,761 / 0 | Control-flow split, explicit non-OOM contracts, successful-only initialization charge; disposable copy |
| 546 | 1,761 / 1 | Production full policy G/Prog needed explicit OOM request projection |
| 547 | 1,764 / 1 | Strict bridge needed pointwise equality before sequence extensionality |
| 548 | 1,765 / 0 | Completed allocation theorem surface in disposable copy |
| 549 | 1,765 / 0 | Live std try_* tree with runtime-erased spec export wrappers |
| 550 | 1,764 / 1 | Alloc-only vstd exposed a resource-limit failure in `dib_finish` |
| 551 | 1,765 / 0 | Replaced broad header-model unfolding with `lemma_r2r5_policy_ok` |
| 552 | 1,765 / 0 | Final no_std tree, full-G cleanup and explicit policy work assertion |

Every verifier invocation used `verify.sh`, serially and offline. `*.alloc-a1` preserves the original live state; `*.alloc-a3` preserves the fully tested std try_* state; `*.alloc-a4` preserves run 551; `*.alloc-a5` preserves the final run-552 source/test/configuration state. Earlier `alloc-wip-*` snapshots remain historical, unverified snapshots. The disposable `alloc-verify-a1` crate and temporary transformation scripts were removed after landing; no numbered run log was deleted or renamed.

The final serial host suite passed all eight tests: 302 frozen vectors, four profiles at 102 each, BMP acceptance 50/52/58/55 out of 71, original corpus 84 checked/58 accepted, 66 custom vectors, and `custom_policies_match_python_oracle` including 26,238 public/custom comparisons. `alloc-a5-tests.log` retains the output. The added serial fault test checks 302 core + 408 named + 66 custom + 355 corpus cases under seven measured schedules each (7,917 measured calls). It checks exact OOM site/size, false recovery, no retained partial output, no retry/reallocation, equal uninjected results, and zero live measured bytes after drop. Its allocator counts bytes only after successful underlying allocation; all unsafe test allocator code is in `tests/support/fault_alloc.rs`.

`tools/audit_alloc.sh` now delegates to a Python source lint that strips spec/proof regions, flags executable allocating constructs including `.push(` and abort calls, pins both guard files, checks the production-module inventory, and runs regression controls. It passes with one executable guard call (`alloc-a5-lint.log`). This is a source falsification check, not the frozen production typed/transitive call-graph audit. A-DIB-4's physical no-other-allocation audit requirement remains outside this delivery; the vector-length invariants and logical-log contracts are verified, while physical allocation behavior is tested.

The crate is `#![no_std]`, with alloc-only vstd, explicit alloc imports, core Default, and test-only std. Native release plus `x86_64-unknown-none`, `thumbv7m-none-eabi`, and `i686-unknown-linux-gnu` library builds passed offline. Dependency-tree and release-rlib metadata contain no runtime std dependency. `NO_STD.md`, `alloc-a5-builds.log`, and `alloc-a5-dependencies.log` record commands, results, trust boundaries, feature-unification caveats, and the absence of target linking/execution/verification. The library targets were removed after collecting evidence.

Frozen policy (six files), DIB 1.1 (six files), and alloc-delta (four files) hashes matched their FREEZE manifests. No frozen file was edited; no git state-changing command or commit was run. Final `src/dib.rs` SHA-256: `bb22ce61ab94562aa036fb6c7a8cf9d0a45d3198b1d0807aaba60088854fe966`.

## tooling hardening after Sol review (2026-11-12)

Test and lint tooling only. No file under `dib_verus/src` changed except the cfg(test) module `policy_tests.rs` (one added call). No git state change, no commit.

**1. `tools/audit_alloc.py`.** `BAD` now also flags `.split_off(`, `.clone_from(`, `.push_str(` and other `push*`/`insert*`, `shrink*`, `splice`, `into_vec`, `into_string`, `concat`, `join`, and bare `with_capacity(`. The existing entries already covered `extend*`, `append`, `resize*`, `reserve*`, `into_boxed*`, `to_vec`, `to_owned`, `to_string`, `clone`, `collect`, `format!`, `vec!`, `Box::`/`Rc::`/`Arc::`/`Vec::`/`String::` paths. The self-test has about 30 negative controls (each construct must be flagged), a set of non-allocating look-alikes that must stay unflagged (`split_at`, `copy_from_slice`, `clone_from_slice`, `truncate`, `swap`), and a proof-only case showing `Seq::push`/`Seq::insert` in `ensures`, `proof {}` and spec functions stay unflagged. `tools/audit_alloc.sh` passes on the real sources (one `try_zeroed` call site). It is a **falsification lint, not the typed call-graph audit**: it matches method names in text, so an allocating method not in the list, or allocation behind a callee, is invisible to it. A clean run does not prove absence of allocation; the docstring says so.

**2. Alignment.** `tests/support/fault_alloc.rs` records `Layout::align()` per event (`last_align`, `failed_align`). The fault test asserts, for the single reservation, align == `align_of::<u8>()` == 1, and, when the baseline decode succeeds, size >= 4*width*height; the failed event's align is also 1.

**3. Suite-level coverage.** `fault_alloc::assert_suite_exercised()` runs at the end of the fault test, prints per-schedule counts, and asserts: at least one case allocates; the control injects nothing; the allocation-2 schedule is reached by allocating cases and injects nothing (the decoder makes at most one allocation, so it cannot fire); thresholds 1, 4096, 65536 each injected at least once; and the 64 MiB threshold either injected or the test prints that the corpus cannot reach it. Observed: 1131 cases, 820 allocate; injections: nth=1 820, threshold 1 820, 4096 281, 65536 26, 67108864 1 (a frozen vector reserves exactly 64 MiB, so that threshold is reachable). Log: `tooling-hardening-tests.log`.

Results unchanged: 302/302 frozen vectors; four profiles 102/102; corpus 50/52/58/55 of 71; custom 66/66; 26,238-row oracle comparison; fault injection green. TGA copies not touched (see the report: `tga_verus/tests/fault_injection.rs` was modified minutes earlier).

## ERRATA E6 update (2026-11-13): `try_zeroed` fills with `resize`

`alloc-delta/ERRATA.md` E6 changed the guard body in all four cores: `try_zeroed` now calls `try_reserve_exact(n)` and, on `Ok`, `v.resize(n, 0u8)`, instead of a `push` loop. `alloc_guard/mod.rs` is byte-identical in the four cores with SHA-256 `26e4c832d2df80b782d157e68ade31066cc82ca2774b4c714a6df0f4edaf86b7`; the pin in `tools/audit_alloc.*` was updated. The trusted list gains A-STD-4: `Vec::<u8>::resize(n, 0)` on a `Vec` with `len == 0` and `capacity() >= n` does not allocate or reallocate. It is observed behaviour of std 1.99.1 (pinned source, counting-allocator and fault-injection tests: one allocation, zero reallocations), not a documented guarantee; `Vec`'s docs say bulk insertion methods "may reallocate, even when not necessary". A reallocation inside `resize` would be infallible and would abort, so it cannot be detected after the fact. The reason is cost: for a 64 MiB buffer the `push` loop takes 47.8 ms and `resize(n, 0)` 6.6 ms (`bench/fillbench/RESULTS.md`).

Latest verifier result: run 556, **1,764 verified, 0 errors** (before E6: 1,765 (run 552)). The live source now carries two `#[verifier::rlimit(20)]` attributes, on `parse_header` and `dib_finish` (RUNLOG runs 555-556); statements in this file that no solver-budget attribute was added describe run 552 only. `alloc_guard/sites.rs` is unchanged (SHA-256 `33465c740e10162d0d6634afd0a6033e26f3d0f523b40e375f4b1d46bcdf000a`). DIB has no other use of A-STD-2: its guard makes one byte reservation and has no element list.

Statements elsewhere in this file that describe the guard as a `push` loop, that use A-STD-2 for `try_zeroed`, that quote the old `mod.rs` hash `018ed17a...` or that give the pre-E6 verified count describe the cores before E6 and are superseded by this section. Test and lint results recorded in this file predate E6 unless dated 2026-11-13; a post-E6 rerun of them is not recorded here.
