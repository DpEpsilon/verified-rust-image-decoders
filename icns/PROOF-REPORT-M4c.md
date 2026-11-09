# PROOF-REPORT-M4c — non-vacuity and mutation protocol

**Milestone:** M4c, the last one: the non-vacuity and mutation protocol of `THEOREMS.md`.
**Verifier:** Verus 0.2026.11.02.8bf68ee, release profile, toolchain
1.99.1-x86_64-unknown-linux-gnu (`/opt/verus/version.json`).

## Contents

1. What was added
2. Tick sheet: T1–T13
3. Tick sheet: L1–L6 and the nine large vectors
4. The 102 model-vs-vector agreement obligations
5. The 40 named witnesses
6. Mutations 1–10
7. Assurance boundary
8. Things worth knowing

## 1. What was added

Nothing in the existing model or the executable changed except two additions:

- `icns_verus/src/model.rs` — `mj_ms`, the R14 combine model over `MpEntry` records (the form
  the fixture's `entries` field can be compared against), and `lemma_mj_ms_eq`, which carries
  `mj` to `mj_ms` on the same model entries (`mc_ms`/`ma_ms` already existed from M4a).
- `icns_verus/src/model_vectors.rs` — generated, 3,260 lines, 102 proof obligations, one per
  small vector. Registered in `lib.rs` as `pub mod model_vectors;`.

`tools/gen_model_vectors.py` emits that file from `spec-frozen/VECTORS.json`. It reads the
expected result only from the fixture's `expected` field; it never calls the production decoder,
and it refuses to run if a large vector's expanded bytes differ from the manifest's symbolic
construction (`check_large`).

No `assume`, `admit`, `external_body`, `trusted`, `unsafe`, `todo!` or `unimplemented!` appears in
`src/icns.rs`, `src/model.rs`, `src/model_vectors.rs` or `src/lib.rs`:
`grep -rnE "admit\(|assume\(|assume_specification|external_body|trusted|unsafe|todo!|unimplemented!|verifier::external"` on those four files returns nothing.

## 2. Tick sheet: T1–T13

All thirteen are proved in the real crate. Each is a postcondition on the concrete function named
in `THEOREMS.md`, discharged by the same verifier run that checks everything else.

| Theorem | Where it lives | Status |
| --- | --- | --- |
| T1 | `parse`'s no-precondition safety postcondition | proved |
| T2 | `rle_decode` (`n <= 16384`, three `n`-byte buffers) | proved |
| T3 | `decode_rgb` bounds (`3d² ≤ 49,152`) | proved |
| T4 | `decode_mask` bounds (`d² ≤ 16,384`) | proved |
| T5 | `combine` bounds (`4d² ≤ 65,536`) | proved |
| T6 | `parsed_inv` from `parse` (`parse_success_invariant`) | proved |
| T7 | `parse` postcondition `parses_to(bytes@, r)` | proved |
| T8 | `lemma_t8` / `lemma_t8_result` / `lemma_parse_ok_gives_p` | proved |
| T9 | `rle_decode` postcondition `rle_result_eq(r, mr(...))`; wrapper `t9_rle_exact` | proved |
| T10 | `decode_rgb` postcondition `rgb_decoded_eq(...)`; wrapper `t10_rgb_exact_for_p` | proved |
| T11 | `decode_mask` postcondition `mask_decoded_eq(...)`; wrapper `t11_mask_exact_for_p` | proved |
| T12 | `combine` postcondition `combine_decoded_eq(...)`; wrapper `t12_combine_exact_for_p` | proved |
| T13 | `t13_combine_after_parse` (P derived from T7/T8, not assumed) | proved |

## 3. Tick sheet: L1–L6 and the nine large vectors

Each large vector is covered exactly once, by the lemma the manifest names. The generator checks
that each fixture's expanded bytes equal the manifest's symbolic construction:

| Vector | Bytes | Manifest form | Lemma | Status |
| --- | --- | --- | --- | --- |
| `file-too-large` | 67,108,865 | L1: `\|b\| = 67,108,864 + 1` | `lemma_l1_file_too_large` | proved |
| `file-size-at-limit` | 67,108,864 | L2: `\|b\| = 67,108,864`, bad magic | `lemma_l2_file_size_at_limit` | proved |
| `mask-t8mk` | 16,400 | L3: `icns·BE32(16,400)·"t8mk"·BE32(16,392)·a`, `a = 0x7f^16384` | `lemma_l3_mask_t8mk` | proved |
| `entry-count-exactly-4096` | 32,776 | L4 at `k = 4,096`: `H(4096, empty)` | `lemma_l4_entry_count_exactly_4096` | proved |
| `entry-count-limit` | 32,784 | L5, `s = empty`: `H(4097, empty)` | `lemma_l5_vectors` | proved |
| `count-limit-before-stray-suffix` | 32,787 | L5, `s = "BAD"` | `lemma_l5_vectors` | proved |
| `entry-count-before-later-malformed-entry` | 32,792 | L5, `s = "none"·BE32(7)` | `lemma_l5_vectors` | proved |
| `invalid-4097th-entry-before-limit` | 32,784 | L6, `q = 7` | `lemma_l6_vectors` | proved |
| `entry-count-followed-by-overflowing-4097th` | 32,784 | L6, `q = 4,294,967,295` | `lemma_l6_vectors` | proved |

The general statements are `lemma_l1_oversized` (`∀b, |b| > 67,108,864`),
`lemma_l2_exact_limit_bad_magic` (`∀b, |b| = 67,108,864 ∧ b[0:4] ≠ "icns"`), `lemma_l3_large_mask`
(`∀a, |a| = 16,384`), `lemma_l4_repeated_entries` (`∀k ≤ 4096`), `lemma_l5_count_before_suffix`
(`∀s, |s| ≤ 67,108,864 − 32,784`) and `lemma_l6_invalid_before_count` / `lemma_l6_oob_before_count`
(`∀q < 8` / `∀q > 8`). L6's `q = 8` case is `lemma_l6_eight_before_count`, which gives
`TooManyEntries`.

## 4. The 102 model-vs-vector agreement obligations

`icns_verus/src/model_vectors.rs` contains one `proof fn agree_<vector>` per vector whose expanded
input is at most 4,096 bytes: 102 obligations over 54 `parse`, 33 `rgb`, 8 `mask` and 7 `combine`
vectors, 11,819 input bytes in all. Each states the model application equals the fixture's
`expected`, and each is discharged by `assert(...) by (compute)`. They verify: the run that
includes this file reports `315 verified, 0 errors` (212 before, plus the 102 vector obligations
and `lemma_mj_ms_eq`).

Two forms appear:

- `parse` vectors: `mp(b_v()) == Ok(e_v())` (or `== Err(MpError::X)`), where `e_v()` is built
  from the fixture's `entries` array — `type_hex`, `payload`, `opaque_candidate`, `nominal_side`.
- decoding vectors: `match mp(b_v()) { Ok(ms) => <mc_ms|ma_ms|mj_ms>(b_v(), ms, idx) == Ok(...) | Err(_) => false }`.
  The entries are taken from the model's own `mp` result, so the statement covers the whole chain
  parse-then-decode. The expected pixel bytes are the fixture's `rgb_runs`/`alpha_runs`/`rgba_runs`
  expanded into `rep_pat(pattern, count)` repetitions.

## 5. The 40 named witnesses

All forty exist in `spec-frozen/VECTORS.json` at or below 4,096 bytes, and each is discharged by
the named obligation in section 4 (all of which verify):

| Witness | Operation | Expected | Proof item |
| --- | --- | --- | --- |
| `short-header` | parse | TruncatedHeader | `agree_short_header` |
| `bad-magic` | parse | BadMagic | `agree_bad_magic` |
| `empty-input` | parse | TruncatedHeader | `agree_empty_input` |
| `bad-magic-before-length` | parse | BadMagic | `agree_bad_magic_before_length` |
| `invalid-file-length` | parse | InvalidFileLength | `agree_invalid_file_length` |
| `declared-longer` | parse | TruncatedFile | `agree_declared_longer` |
| `declared-shorter` | parse | TrailingFileBytes | `agree_declared_shorter` |
| `truncated-entry-header` | parse | TruncatedEntryHeader | `agree_truncated_entry_header` |
| `entry-size-seven` | parse | InvalidEntryLength | `agree_entry_size_seven` |
| `entry-length-one-too-long` | parse | EntryOutOfBounds | `agree_entry_length_one_too_long` |
| `trailing-after-entry` | parse | TruncatedEntryHeader | `agree_trailing_after_entry` |
| `entry-size-zero` | parse | InvalidEntryLength | `agree_entry_size_zero` |
| `minimal-empty` | parse | Ok: 0 entries | `agree_minimal_empty` |
| `entry-length-equals-remaining` | parse | Ok: 1 entry | `agree_entry_length_equals_remaining` |
| `unknown-type` | parse | Ok: 1 entry | `agree_unknown_type` |
| `duplicate-order` | parse | Ok: 2 entries | `agree_duplicate_order` |
| `opaque-ic10` | parse | Ok: 1 entry | `agree_opaque_ic10` |
| `jp2-signature-unknown-type` | parse | Ok: 1 entry | `agree_jp2_signature_unknown_type` |
| `png-signature-legacy-mask-stays-mask` | parse | Ok: 1 entry | `agree_png_signature_legacy_mask_stays_mask` |
| `rgb-is32` | rgb | Ok: side 16 | `agree_rgb_is32` |
| `rgb-it32` | rgb | Ok: side 128 | `agree_rgb_it32` |
| `literal-128-boundary` | rgb | Ok: side 16 | `agree_literal_128_boundary` |
| `literal-nonuniform-rgb` | rgb | Ok: side 16 | `agree_literal_nonuniform_rgb` |
| `it32-eight-zero-prefix-then-literals` | rgb | Ok: side 128 | `agree_it32_eight_zero_prefix_then_literals` |
| `is32-four-zero-literals` | rgb | Ok: side 16 | `agree_is32_four_zero_literals` |
| `rle-empty` | rgb | RleTruncated | `agree_rle_empty` |
| `repeat-overrun-before-missing-value` | rgb | RleOverrun | `agree_repeat_overrun_before_missing_value` |
| `run-crosses-channel-boundary` | rgb | RleOverrun | `agree_run_crosses_channel_boundary` |
| `rle-trailing-data` | rgb | RleTrailingBytes | `agree_rle_trailing_data` |
| `mask-h8mk` | mask | Ok: side 48 | `agree_mask_h8mk` |
| `mask-too-short` | mask | InvalidMaskLength | `agree_mask_too_short` |
| `mask-index-equal-entry-count` | mask | IndexOutOfRange | `agree_mask_index_equal_entry_count` |
| `unsupported-rgb-type` | rgb | UnsupportedType | `agree_unsupported_rgb_type` |
| `unsupported-mask-colour-type` | mask | UnsupportedType | `agree_unsupported_mask_colour_type` |
| `mask-without-colour` | combine | MissingColor | `agree_mask_without_colour` |
| `colour-without-mask` | combine | MissingMask | `agree_colour_without_mask` |
| `combine-mask-before-colour` | combine | Ok: side 16 | `agree_combine_mask_before_colour` |
| `combine-nonuniform-alpha` | combine | Ok: side 16 | `agree_combine_nonuniform_alpha` |
| `mismatched-mask-dimensions` | combine | TypeMismatch | `agree_mismatched_mask_dimensions` |
| `combine-colour-error-before-mask-error` | combine | IndexOutOfRange | `agree_combine_colour_error_before_mask_error` |

## 6. Mutations 1–10

| # | Mutation | Required failing obligation, and where it failed | Result | Log |
| --- | --- | --- | --- | --- |
| 1 | `|b| > 67,108,864` → `>=` | lemma_l2_exact_limit_bad_magic (model.rs:1465); `lemma_mp_bad_magic`/`_invalid_len`/`_truncated_file`/`_trailing`; `lemma_mp_eq_mp_at` | 309 verified, 6 errors | `mutation/m1/failure.log` (from `runs/193.log`) |
| 2 | `q > L − off` → `q > L − off − 8` | `agree_trailing_after_entry` (model_vectors.rs:242) reports `Err(EntryOutOfBounds) == Err(TruncatedEntryHeader)` false; `lemma_scan_walk` (model.rs:534); L3 (model.rs:2203) | compile aborted after 3 errors (no crate summary) | `mutation/m2/failure.log` (from `runs/196.log`) |
| 3 | `TooManyEntries` moved before the 4,097th entry's length check, in both `scan` and `walk` | `lemma_pref_error_iel` (model.rs:1266) and `lemma_pref_error_oob` (model.rs:1280), the walk-level statements L6 uses | 313 verified, 2 errors | `mutation/m3/failure.log` (from `runs/205.log`) |
| 4 | `legacy(c) = None` guard deleted from `classify` | `classify`'s postcondition (icns.rs:292, the `s8mk` exit), L3's `entry_at` (model.rs:2191), and `agree_png_signature_legacy_mask_stays_mask` (model_vectors.rs:2646) with `opaque_candidate: true` vs `false` | compile aborted after 3 errors (no crate summary) | `mutation/m4/failure.log` (from `runs/198.log`) |
| 5 | range conjunct `8 <= start <= end <= |b|` deleted from `entry_inv` | `payload_range`'s `entry_payload_safe` precondition at both decoder call sites (icns.rs:982 `decode_rgb`, icns.rs:1118 `decode_mask`) | 313 verified, 2 errors | `mutation/m5/failure.log` (from `runs/199.log`) |
| 6 | `control − 125` → `control − 124` in `rle_count` | `agree_rgb_is32` (model_vectors.rs:317); `lemma_rle_count_bounds` (model.rs:2656); `rle_decode`'s own count assertion (icns.rs:778) | compile aborted after 4 errors (no crate summary) | `mutation/m6/failure.log` (from `runs/200.log`) |
| 7 | overrun and payload-availability checks swapped in `rle_chan_from` | `lemma_rle_chan_from_overrun` (model.rs:2589); `agree_repeat_overrun_before_missing_value` (model_vectors.rs:2367) | compile aborted after 2 errors (no crate summary) | `mutation/m7/failure.log` (from `runs/201.log`) |
| 8 | `it32` prefix strips eight zeros rather than four | `agree_rgb_it32` (model_vectors.rs:466); the `it32` prefix equality inside `decode_rgb` (icns.rs:995) | compile aborted after 2 errors (no crate summary) | `mutation/m8/failure.log` (from `runs/202.log`) |
| 9 | `MissingMask` tested before `MissingColor` in `mj` | `combine`'s `MissingColor` assertion (icns.rs:1184); `lemma_mj_missing_color` (model.rs:3806); `lemma_mj_ms_eq` (model.rs:4018) | 312 verified, 3 errors | `mutation/m9/failure.log` (from `runs/203.log`) |
| 10 | alpha equality `out[4k+3] = alpha[k]` removed from combine's loop postcondition | `lemma_interleave_rgba_exact`'s alpha conjunct (model.rs:3748) is no longer dischargeable at the call in `combine` (icns.rs:1276) | 314 verified, 1 errors | `mutation/m10/failure.log` (from `runs/204.log`) |

## 7. Assurance boundary

No additional trust assumption was introduced. The trusted base is the verifier, `vstd`'s
contracts and the compiler's faithful execution of verified code — the same base as M4b. The
`by (compute)` obligations are decided by Verus's normalizer, whose soundness is part of that
same trusted base; they introduce no axiom and no escape.

## 8. Things worth knowing

1. **`compute` decides these obligations, and it is sound but expensive.** `assert(p) by (compute)`
   makes Verus's own normalizer evaluate the closing term; no axiom enters the trusted base. The
   cost is time: the whole crate now takes 252 s to verify (was 64 s), because six vectors have
   outputs up to 49,152 bytes (`rgb-it32`, `it32-without-prefix`,
   `it32-eight-zero-prefix-then-literals`, `rgb-il32`, `rgb-ih32`, `mask-h8mk`) and `compute` walks
   them byte by byte.

2. **`type_hex` is the hex encoding, not the raw bytes.** The model's `MpEntry.type_hex` is
   `Seq<char>` of the eight lowercase hex digits (`hex4(code)`), not the four raw bytes; the
   generator builds the expected list with `'6', '9', '7', '3', ...` accordingly. My first draft
   used `chr(byte)`, which `compute` rejected by simplifying both sides and printing the mismatch.

3. **Decoding vectors run parse first.** Each one is stated as
   `match mp(b()) { Ok(ms) => <model>(b(), ms, idx) == Ok(...) | Err(_) => false }`, so the entries
   the decoder sees are `mp`'s own, and a parse failure makes the obligation false rather than
   vacuous. That is stronger than comparing against a separately presupplied list.

4. **Mutation 3 needed two edits, not one.** Moving `TooManyEntries` in `scan` alone still leaves
   `walk` (the count-indexed twin used by the T7 bridge) in the old order, so the first attempt
   failed at `lemma_scan_walk` rather than at L6's `q = 7` statement. The copy now moves the check
   in both `scan` and `walk`, and the failure lands at `lemma_pref_error_iel` /
   `lemma_pref_error_oob` — the two lemmas `lemma_l6_vectors` composes. The original single-site
   attempt is preserved in `runs/197.log`; the corrected run is `runs/205.log`.

5. **Two runs were aborted by me and are not results.** Runs 194 and 195 are SIGTERM failures
   caused by my own `pkill` while a real-crate verification and a mutation run overlapped. They
   are not mutation outcomes; the mutation logs are 196 and 205 for mutation 2 and 3
   respectively, and run 195 has no bearing on any claim here. Every mutation log in
   `mutation/mN/failure.log` comes from a clean `./verify.sh` invocation.

6. **Mutation 5 did not need its fallback.** Deleting the range conjunct from `entry_inv` broke
   the derivation at `payload_range`'s precondition in both `decode_rgb` and `decode_mask`: the
   verifier did not reconstruct `8 <= start <= end <= |b|` by unfolding `parse`, so the mandated
   false `start > end` assertion was not applied. The fallback is implemented
   (`tools/mutations.py`, `M5_FALLBACK`) and unused.

7. **Mutation 10 did not need its fallback either.** Removing the alpha equality from the loop
   postcondition left `lemma_interleave_rgba_exact`'s alpha conjunct underivable at the `combine`
   call, so T12 failed directly. The manifest's alternative — replacing a nonzero alpha with zero
   in the implementation — was not needed.

8. **Five of the ten mutations abort compilation after reporting errors (2, 4, 6, 7, 8), so
their logs carry no crate summary line.** `cargo verus` prints `verification results:: 2159 verified, 0 errors` for
   `vstd` first; that line is about the support crate, not this one. `tools/mutation_summary.py`
   therefore records either the crate's own summary (mutations 1, 3, 5, 9, 10) or the fact that
   compilation aborted (2, 4, 6, 7, 8) plus the individual `error:` diagnostics.

9. **Interrupted work estimate.** Verifying this crate now takes about 4 minutes per run because
   of point 1; the ten mutations alone are 40 minutes of verifier time. The overall elapsed time
   is dominated by that, not by proof search.

10. **Housekeeping.** I deleted `src/icns.rs.m2`, a stale M2 snapshot that nothing references
    (the M3/M4a snapshots `icns.rs.m3`, `icns.rs.m4a` and `icns.rs.wip-m3b` remain). The mutation
    copies under `mutation/mN/` keep their own `src/icns.rs`, so the deletion does not affect any
    mutation log. I also removed the throwaway probe crate `mutation/_exp` and the mutation
    copies' `target/` directories (1.8 GB).
11. **The executable is untouched.** `icns.rs` in the real crate is byte-identical to
    `mutation/base/src/icns.rs` (the pristine copy taken before any mutation), and `cargo test`
    still reports `conformance: 111/111 vectors passed`.

### Runs and counts

`./verify.sh` runs this milestone: 33, numbered 176-208 (runs 194 and 195 were aborted by my
`pkill` and produced no result, so 31 carry a verifier outcome). Of these:

- 14 were exploratory probes in a throwaway copy (`mutation/_exp`, since deleted): 176-189.
- 3 were real-crate generations of `model_vectors.rs`: 190, 191 (both failing, on the import and
  `type_hex` issues above) and 192 (315 verified, 0 errors).
- 10 were the mutation outcome runs: 193, 196, 197, 198, 199, 200, 201, 202, 203, 204. Run 197
  was mutation 3's first attempt with a single-site edit; it failed at the scan/walk bridge rather
  than at L6, so mutation 3 was rebuilt and rerun as 205.
- 2 were aborted by my `pkill` when two verifications overlapped: 194 (a first attempt at
  mutation 2) and 195 (a real-crate run). Neither is a result.
- 1 was the corrected mutation-3 rerun: 205.
- 1 was a real-crate verification after cleanup: 206.
- 1 was a cache-only confirmation (207, 0 s — no verification happened).
- 1 was the definitive full reverify after `cargo clean`: 208.

No obligation hit the five-failure stop rule; no `STUCK.md` was written.

### Tool calls

Approximately 250 by my own count, most of them short wait/poll calls while the verifier ran (a
run takes 4-5 minutes now). The distinct work is about 30 tool calls' worth of reads, about 25 of
generator/probe iterations, and 33 logged verifier invocations. I did not hit the 400 cap.

### Final result lines

- Real crate, full reverify after `cargo clean`:
  `verification results:: 315 verified, 0 errors` (`runs/208.log`), 317 s.
- `cargo test --test conformance -- --nocapture`:
  `conformance: 111/111 vectors passed`, `test result: ok. 1 passed; 0 failed`.
- Absolute paths: `/__modal/volumes/vo-Iyh3VLkOB7cEHrIp2RkTs7/projects/icns-verus/runs/208.log`,
  `/__modal/volumes/vo-Iyh3VLkOB7cEHrIp2RkTs7/projects/icns-verus/icns_verus/src/model_vectors.rs`,
  `/__modal/volumes/vo-Iyh3VLkOB7cEHrIp2RkTs7/projects/icns-verus/tools/gen_model_vectors.py`,
  `/__modal/volumes/vo-Iyh3VLkOB7cEHrIp2RkTs7/projects/icns-verus/tools/mutations.py`,
  `/__modal/volumes/vo-Iyh3VLkOB7cEHrIp2RkTs7/projects/icns-verus/tools/run_mutations.sh`,
  `/__modal/volumes/vo-Iyh3VLkOB7cEHrIp2RkTs7/projects/icns-verus/tools/mutation_summary.py`.
