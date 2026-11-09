# PROOF-REPORT-M4d — non-vacuity and mutation protocol (Creusot): COMPLETE

## Headline

M4c's two open items are both closed. Sol's runs 214/215 proved **all 102 model-vs-vector
agreement obligations** (green, `Proved (7077 files) ✔`, the second from a cache-free copy), and the
**ten manifest mutations all fail verification** (runs 216-225, exit=1 each) — no vacuity finding, so
the manifest's fallback wording for mutations 5 and 10 was not needed. The live crate is green,
`cargo test` is 111/111, and no theorem was weakened, no `why3find.json` budget changed, and no
`admit`/`trusted`/`external_body`/`unsafe`/`todo`/`requires(false)` appears anywhere.

The mutation runs 216-225 were produced by `tools/run_mutations.py` (each case a fresh copy of
`mutation/base/`, rebuilt per `THEOREMS.md`). The driver crashed at mutation 10's `copytree` with
`ENOSPC` — the volume's hard **inode** cap (500,000), not a block shortage — so I freed inodes from
the finished copies' regenerable caches (`verif/`, `.why3find/`, `target/`) while keeping every
`failure.log`/`result.txt`, and ran mutation 10 myself with the driver's own `mutate(10)` and
`verify_case(10)`. See "Mutation-10 recovery" below.

## Tick sheet

| Item | Status | Evidence |
| --- | --- | --- |
| T1-T13 | proved (unchanged from M2/M3/M4a/M4b), re-confirmed in the green runs | runs 214/215, `Proved (7077 files)`; T7/T8 in `parse`'s contract + `parse_iff_p`/`parse_p_forward`; T9 in `rle_decode`; T10 `t10_rgb`, T11 `t11_mask`, T12 `t12_combine`, T13 `t13_combine_at_parse`; T1-T6 safety via `parse`/`rgb`/`mask`/`combine`/`rle_channel` contracts and the `rgb_at_parse`/`mask_at_parse`/`combine_at_parse` call sites |
| L1-L6 | proved (from M3c/M4a), all nine instantiations present | `l1`, `l2`, `l4`, `l5`, `l6_short`/`l6_oob`, `l3_mp`/`l3_ma`; `inst_*` items listed below |
| 102-vector agreement | **102/102 proved** | run 214 (live), run 215 (clean cache); 102 `agree_*` items |
| 40 named witnesses | **40/40 proved**, one proof item each | `agree_<name>` (name with `-`→`_`), see table |
| Mutations 1-10 | **all 10 fail** (exit=1) | runs 216-225, logs `mutation/mN/failure.log` |

## 102 model-vs-vector obligations

`tools/gen_model_vectors.py` (769 lines of emitter, 2.3 MB of output) reads
`spec-frozen/VECTORS.json` and emits, for each of the **102** vectors whose expanded input is at most
4,096 bytes, one `#[logic]` obligation `agree_<name>` asserting the crate's formal model applied to
the vector's expanded bytes equals the fixture's `expected` value. Expected values come only from the
fixture; the executable decoder is never consulted. Decoding vectors are phrased
`match model::mp(b) { Ok(ms) => <mc_model|ma_model|mj_model>(b, ms, idx) == <expected>, Err(_) => false }`,
so the entry list the decoder model consumes is `mp`'s own parse result and a parse failure makes the
obligation false, not vacuous. Family counts: **parse 54, rgb 33, mask 8, combine 7 = 102**.

`tools/inject.sh` splices the generated module into `icns_creusot/src/lib.rs` under
`#[cfg(creusot)] pub mod model_vectors` (line 3023). The 102 obligations are the goals whose names
appear as `vc_agree_<name>`; they are among the 7,077 proved files of runs 214 and 215.

- **run 214** — `M4d exact fixture RGB bytes and all 102 model-agreement obligations`, live crate,
  exit 0, `Proved (7077 files) ✔`, 343 s.
- **run 215** — `M4d clean-cache exact 102 frozen expected model outputs`, crate
  `mutation/clean-exact/icns_creusot` (source + config copied in, **no proof cache**), exit 0,
  `Proved (7077 files) ✔`, 657 s. The ~1.9× time over run 214 (657 s vs 343 s) is the expected cost
  of proving every goal from scratch rather than hitting `.why3find`.

Every obligation is a chain of small lemmas instantiating proved single-step facts at concrete `Int`
positions (`model::step_acc`, `loop_done`, `loop_hdr`, `loop_short`, `loop_oob`, `loop_many`,
`rmodel::ch_step`, `ch_done`, `ch_trunc`, `ch_over`, `ch_data`), following the M3c encoding rules
(byte-at-index via `Seq::create`, `Int` not `u32`, no concat/repeat constructors).

## The 40 named witnesses

Each is a vector at or below 4,096 bytes; the proof item is `agree_<name>` with `-` replaced by `_`
(`ident()` in `gen_model_vectors.py` replaces every non-alphanumeric character with `_`). All 40 are
present and proved in runs 214/215.

| Group | Witness | Proof item |
| --- | --- | --- |
| Container errors/precedence | short-header | `agree_short_header` |
| | bad-magic | `agree_bad_magic` |
| | empty-input | `agree_empty_input` |
| | bad-magic-before-length | `agree_bad_magic_before_length` |
| | invalid-file-length | `agree_invalid_file_length` |
| | declared-longer | `agree_declared_longer` |
| | declared-shorter | `agree_declared_shorter` |
| | truncated-entry-header | `agree_truncated_entry_header` |
| | entry-size-seven | `agree_entry_size_seven` |
| | entry-length-one-too-long | `agree_entry_length_one_too_long` |
| | trailing-after-entry | `agree_trailing_after_entry` |
| | entry-size-zero | `agree_entry_size_zero` |
| Container successes/classification | minimal-empty | `agree_minimal_empty` |
| | entry-length-equals-remaining | `agree_entry_length_equals_remaining` |
| | unknown-type | `agree_unknown_type` |
| | duplicate-order | `agree_duplicate_order` |
| | opaque-ic10 | `agree_opaque_ic10` |
| | jp2-signature-unknown-type | `agree_jp2_signature_unknown_type` |
| | png-signature-legacy-mask-stays-mask | `agree_png_signature_legacy_mask_stays_mask` |
| RLE and colour | rgb-is32 | `agree_rgb_is32` |
| | rgb-it32 | `agree_rgb_it32` |
| | literal-128-boundary | `agree_literal_128_boundary` |
| | literal-nonuniform-rgb | `agree_literal_nonuniform_rgb` |
| | it32-eight-zero-prefix-then-literals | `agree_it32_eight_zero_prefix_then_literals` |
| | is32-four-zero-literals | `agree_is32_four_zero_literals` |
| | rle-empty | `agree_rle_empty` |
| | repeat-overrun-before-missing-value | `agree_repeat_overrun_before_missing_value` |
| | run-crosses-channel-boundary | `agree_run_crosses_channel_boundary` |
| | rle-trailing-data | `agree_rle_trailing_data` |
| Mask and composition | mask-h8mk | `agree_mask_h8mk` |
| | mask-too-short | `agree_mask_too_short` |
| | mask-index-equal-entry-count | `agree_mask_index_equal_entry_count` |
| | unsupported-rgb-type | `agree_unsupported_rgb_type` |
| | unsupported-mask-colour-type | `agree_unsupported_mask_colour_type` |
| | mask-without-colour | `agree_mask_without_colour` |
| | colour-without-mask | `agree_colour_without_mask` |
| | combine-mask-before-colour | `agree_combine_mask_before_colour` |
| | combine-nonuniform-alpha | `agree_combine_nonuniform_alpha` |
| | mismatched-mask-dimensions | `agree_mismatched_mask_dimensions` |
| | combine-colour-error-before-mask-error | `agree_combine_colour_error_before_mask_error` |

## Coverage of the nine large vectors by L1-L6

`VECTORS.json` splits 111 = 102 (≤4,096 B expanded) + 9 (larger). The nine are not expanded inside
the verifier; each is an instantiation of a proved symbolic lemma, whose fixture bytes the generator
checks equal the manifest's arithmetic construction.

| Vector | Expanded bytes | Lemma | Instantiation item |
| --- | --- | --- | --- |
| file-too-large | 67,108,865 | L1 (`l1`) | `l1` |
| file-size-at-limit | 67,108,864 | L2 (`l2`) | `l2` |
| mask-t8mk | 16,400 | L3 (`l3_mp`, `l3_ma`) | `inst_mask_t8mk` |
| entry-count-exactly-4096 | 32,776 | L4 (`l4`) | `inst_entry_count_exactly_4096` |
| entry-count-limit | 32,784 | L5 (`l5`) | `inst_entry_count_limit` |
| count-limit-before-stray-suffix | 32,787 | L5 (`l5`) | `inst_count_limit_before_stray_suffix` |
| entry-count-before-later-malformed-entry | 32,792 | L5 (`l5`) | `inst_entry_count_before_later_malformed_entry` |
| invalid-4097th-entry-before-limit | 32,784 | L6, `q=7` (`l6_short`) | `inst_invalid_4097th_entry_before_limit` |
| entry-count-followed-by-overflowing-4097th | 32,784 | L6, `q=2³²-1` (`l6_oob`) | `inst_entry_count_followed_by_overflowing_4097th` |

L5 covers three, L6 two; L4's empty base case (`k=0`) is proved too, so it is non-vacuous.

## Mutations 1-10 (all failed verification, as required)

Each case is a disposable copy `mutation/mN/icns_creusot` rebuilt from the pristine
`mutation/base/icns_creusot`, run through `VERIFY_CRATE=mutation/mN/icns_creusot ./verify.sh`. `base`
is byte-identical to the live crate on `src/lib.rs`, `Cargo.toml`, `Cargo.lock`, `why3find.json`.
All T1-T13 and L1-L6 statements stay enabled.

| # | Change (vs live) | Result | Failed goals | Log |
| --- | --- | --- | --- | --- |
| 1 | `|b| > 67108864` → `>=` in `parse`'s length check; L2 and T7 untouched | run 216, exit=1 | `vc_l2`, `vc_parse`, `vc_l5`, `vc_map_push` | `mutation/m1/failure.log` |
| 2 | entry bound `q > l - off` → `q > l - off - 8`; `entry-length-equals-remaining` untouched | run 217, exit=1 | `vc_step_none`, `vc_good_many`, `vc_loop_many`, `vc_good_step`, `vc_step_acc`, `vc_step_l3`, `vc_map_push` | `mutation/m2/failure.log` |
| 3 | move `TooManyEntries` before the 4,097th length check; L6 and T7 untouched | run 218, exit=1 | `vc_loop_short`, `vc_loop_oob`, `vc_good_len`, `vc_good_bounds`, `vc_map_push` | `mutation/m3/failure.log` |
| 4 | delete the `legacy(c)=None` guard from signature classification | run 219, exit=1 | 23 goals incl. `vc_k_png_signature_legacy_mask_stays_mask_8`, `vc_classify_opaque`, `vc_t8mk_opaque` | `mutation/m4/failure.log` |
| 5 | delete `parse`'s `parser_ok` postcondition; T3-T6/T13 untouched | run 220, exit=1 | `vc_rgb_at_parse`, `vc_mask_at_parse`, `vc_combine_at_parse`, `vc_map_push` | `mutation/m5/failure.log` |
| 6 | repeat control `control-125` → `control-124` | run 221, exit=1 | `vc_rle_channel`, `vc_map_push` | `mutation/m6/failure.log` |
| 7 | swap RLE's overrun and payload-availability checks | run 222, exit=1 | `vc_rle_channel`, `vc_map_push` | `mutation/m7/failure.log` |
| 8 | strip eight zeros rather than four for `it32` | run 223, exit=1 | `vc_strip4_prog` | `mutation/m8/failure.log` |
| 9 | reverse missing-colour/missing-mask priority in `combine` | run 224, exit=1 | `vc_combine` | `mutation/m9/failure.log` |
| 10 | drop the alpha-byte equality from the **modular** `combine` postcondition; T12's full pixel equality and the dependent final assertion unchanged | run 225, exit=1 | `vc_t12_combine`, `vc_t13_combine_at_parse` | `mutation/m10/failure.log` |

No mutation proved; there is no vacuity finding, and the manifest's fallback wording for mutations 5
and 10 was not needed (the direct forms failed). Each failure is the *symbolic/model* obligation the
manifest names, not a concrete large-input evaluation:

- **1** fails the symbolic `vc_l2` (exact-limit bad magic), not a 64 MiB concrete evaluation.
- **3** fails the `mp_loop` unfolding facts `vc_loop_short`/`vc_loop_oob`/`vc_good_len`/`vc_good_bounds`
  (the L6 `q=7` chain), not a 32 KiB concrete evaluation.
- **5** fails exactly the T3-T6 call-site derivations `vc_rgb_at_parse`/`vc_mask_at_parse`/
  `vc_combine_at_parse` (the T13 site too), i.e. the derivation that reads `parse`'s postcondition.
- **8** fails `vc_strip4_prog`, the private helper whose `#[ensures]` is exactly R11's "remove exactly
  four bytes" (`result@ == p@.subsequence(4, p@.len())`). The verifier catches the eight-zero strip on
  the it32 path (the helper is called only when `is_it32_code(ty) && starts_zero4_prog(payload)`).
  Note: `agree_it32_eight_zero_prefix_then_literals` is stated purely over the model (no call into
  executable code, verified: it references only `b_*`/`ms_*`/`mc_model`/`mc_entry`/`msg_*`/`rgb_expected_*`),
  so it is unaffected by a change to the executable helper; the executable contract that R11 governs
  is the `strip4_prog` goal, which is what fails here.
- **9** fails `vc_combine`, whose body is the absent-index order.
- **10** fails exactly `vc_t12_combine` and `vc_t13_combine_at_parse`, matching M4b's run-119 finding:
  a caller sees only `combine`'s own postcondition, so the mutation must be on that contract to reach
  the blanket theorems. The fallback ("replace a nonzero alpha with zero in the implementation") was
  not needed because the direct form failed.

### Mutation-10 recovery (inode exhaustion)

`tools/run_mutations.py` (started 23:41 by the prior Sol session) ran m1-recheck through m9 and died in
`shutil.copytree` for m10 with `[Errno 28] No space left on device`. The volume
(`/__modal/volumes/vo-Iyh3VLkOB7cEHrIp2RkTs7`) has a hard cap of **500,000 inodes, 0 free** (`v9fs`,
`stat -f`), while ≥382 GB of blocks were free. Each mutation copy holds ~28,000 files (≈14,154 in
`verif/` plus ≈14,500 in `.why3find/`), so eleven copies plus the live crate filled the cap. I removed
only the regenerable caches (`verif/`, `.why3find/`, `target/`) from m1-m9 and `clean`/`clean-exact`,
keeping **every** `failure.log`, `result.txt`, `direct.log`, `recheck.log`, `preflight-failure.log`;
`base` was untouched. I then ran mutation 10 by importing the driver's own `mutate`/`verify_case`, so
the mutation text, the copy, the `./verify.sh` wrapper and the logging are identical to the crashed
run's intent. Mutation 10 is run 225.

## Clean-cache verifier line, tests, versions

- Clean-cache reproduction: **run 215**, crate `mutation/clean-exact/icns_creusot` (source + config,
  no proof cache), exit 0, final line `Proved (7077 files) ✔`.
- Live green: **run 214**, exit 0, `Proved (7077 files) ✔`. (Runs 211/212, `Proved (7065 files)`, are
  the pre-final-fixture greens; run 212 is the same-102 clean-cache run before the explicit RGB fixture
  equality was added.)
- `cargo test` on the live crate: `conformance: 111/111 vectors passed`, `test result: ok. 1 passed; 0
  failed; 0 ignored` (5.1 s). Vector split confirmed from `VECTORS.json`: 111 total, 102 ≤ 4,096 B, 9
  larger.
- Trusted computing base: `cargo-creusot 0.14.0`, `creusot-std 0.14.0`, Rust toolchain
  `nightly-2026-08-03` (`rustc 1.99.0-nightly 11177f223`, per M4b; the wrapper `rustc` reports
  1.99.1); Why3 `1.8.2+git`, `why3find v1.3.0+dev`; provers Alt-Ergo `2.6.4`, Z3 `5.1.0`, CVC5
  `1.4.1`, CVC4 `1.8`.
- `why3find.json` is unchanged: `fast 0.2`, `time 1.0`, `depth 6`, provers
  `alt-ergo, z3, cvc5, cvc4`.

## Runs and tool calls

M4d verifier runs (RUNLOG rows; run numbers and durations): 209 baseline (red, 395 s), 210 (red,
427 s), 211 (green 7065, 327 s), 212 (clean-cache green 7065, 622 s), 213 mutation-1 preflight (red,
490 s), 214 (green 7077, 343 s), 215 (clean-cache green 7077, 657 s), then mutations 216 (m1 recheck,
473 s), 217 (m2, 452 s), 218 (m3, 441 s), 219 (m4, 457 s), 220 (m5, 463 s), 221 (m6, 468 s), 222 (m7,
465 s), 223 (m8, 417 s), 224 (m9, 420 s), 225 (m10, 433 s). Run number 209 carries six M4c rows (the
M4c numbering bug); every run 210-225 is unique, and 222 distinct run numbers appear in the 227
RUNLOG rows. `runs/` holds 224 logs (all numbers 1-225 except 208, which M4c deleted); 22 M4d rows
span numbers 209-225.

Tool calls: 149 (my own transcript's `tool_use` count at report time), within the 150 cap.

## Assurance boundary

No `assume`, `admit`, `trusted`, `external_body`, `unsafe`, `todo!`, `unimplemented!`,
`requires(false)` or test-only axiom was added to the live crate or to any mutation copy; no theorem
was weakened; no run log or `result.txt` was deleted or renamed. The only deletions this dispatch were
the regenerable verifier/build caches inside the finished disposable mutation copies, forced by the
volume's inode cap. The Verus/Creusot assurance boundary stands as in `THEOREMS.md`: the verifier's
soundness, its standard-library contracts and the compiler's faithful execution of verified code are
the unavoidable trusted base.
