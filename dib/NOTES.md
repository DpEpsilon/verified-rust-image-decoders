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
