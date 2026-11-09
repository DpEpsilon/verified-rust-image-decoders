# Proof report — M1.1a (DIB-CORE-SPEC 1.1, R19 enum + helpers + models)

Delivered by DeepSeek V4.1 Flash (Verus). Branch `dib-1.1`. Verus
0.2026.11.02. No commits made (coordinator commits).

## Headline

- Clean full reverify after `cargo clean`: `verification results:: 1398 verified, 0 errors` (`runs/038.log`, 100 s). The same crate verifies again at `runs/040.log` on the final shipped source.
- `cargo test` (release and debug): conformance **302/302** vectors (`../spec-1.1-frozen/VECTORS.json`), corpus **84 files, 58 accepted**.
- Crate version bumped 0.1.0 → 0.2.0 (semver break: `Mode` gains a variant). Noted in `NOTES.md`.
- verify.sh runs used for M1.1a: **runs 1–40** (40 runs; green: 6, 37, 38, 39, 40). RUNLOG.tsv now has 1,045 rows.
- Backups made before editing: `dib.rs.m11`, `model.rs.m11`, `model_vectors.rs.m11`.
- No escapes (no `assume`/`admit`/`external_body`/`trusted`/`unsafe`/`todo!`/`unimplemented!`), no `#[verifier::rlimit]` or other solver-budget attribute. Frozen 1.1 hashes match `spec-1.1-frozen/FREEZE.txt`; `spec-frozen/` and `THEOREMS.md` unchanged.

## What M1.1a asked for, and what was done

### (1) Fourth public mode + helpers + site classification

- `Mode` gains `IcoEntryDibSlack` in both `dib.rs` and `model.rs` (shared: `mode.rs` `use crate::dib::{Error, Mode}`).
- Helpers, in **both** files (dib.rs as `pub open spec fn`, model.rs as `pub open spec fn`):
  - `is_ico(mode)` — "either ICO mode" (`IcoEntryDib` or `IcoEntryDibSlack`).
  - `exact_tail(mode)` — "rejects bytes after the structure" (`BareDib` or `IcoEntryDib`).
  - model.rs extras: `old_mode` (the three 1.0 modes) and `with_mode(m, mode)` (record with only `mode` replaced).
- **Site classification** (delta: 32 in dib.rs, 16 in model.rs). Every 1.0 `x == Mode::IcoEntryDib` test was classified:

  *Shared ICO handling* → `is_ico(x)` or the `A | B` pattern:
  - `dib.rs`: `visible_height` dispatch (1117); R3 even-height rule (1301); R3 visible-height arithmetic (1325, 1343); R4 orientation `ico` local (1430); `spec_is_ico` header predicate (748); 32-bpp alpha read in `raw_sample` (2465); `mask_stride`/`and_bytes` (4724); `is_ico`/`ico` locals in `finish_mask` (4771, 4840); mask-phase case in `decode_tracked` (5396); header invariant `is_ico(h.mode) ==> !top_down` (775); the three `apply_and` contracts/invariants (`is_ico(h.mode)`); `lemma_ma_mp_mask_phase` guard; `mask_done_eq`'s ICO branch; `lemma_mask_done_ok`'s ICO case.
  - `model.rs`: `visible_height_of` (83); `r2r5` even-height (364) and orientation (398); `conv_word` (1230) and `conv_rgb` (1265) 32-bpp alpha; `ma_reachable` (4690); `ma_domain` (5040); `mp_image_size_ok` (5241); `mp_mask_phase` ICO branch (5263); `lemma_mp_mask_phase_ico_ok` guard.

  *Exact-tail handling* → `exact_tail(x)` — the **sole** behavioural difference:
  - `dib.rs`: the final `end != b.len()` test in `finish_mask`, guarded by the new exec boolean `exact` (`exact && end != b.len()`); its mirrors in `mask_done_eq` (`exact_tail(m.mode) && pos+a != b.len()`) and the `decode_tracked` error-branch reasoning (`assert(exact_tail(m.mode))`).
  - `model.rs`: `mp_mask_phase`'s `exact_tail(m.mode) && end != b.len()` test (5256).
  - `ma_domain`/`ma_reachable` do **not** consult the tail rule (MA ends at the complete mask), so they are shared.

### (2) Models MH/MP/MA/MR/Conv extended; new model theorems

- `MH` retains the exact typed mode (`MHRecord.mode`, set from the mode argument in `r6r7_result`). `mp_mask_phase` skips only the final `cursor==|b|` test for `IcoEntryDibSlack`, after the complete AND-mask availability check.
- **Proved new theorems** (no `requires` weakening; stated at the manifest's quantification):
  - `old_modes_unchanged(b, mode)` — for the three 1.0 modes `is_ico(mode) == (mode == IcoEntryDib)` and `exact_tail(mode) == (mode != BmpFile)` hold *definitionally* (case split on the variant), so the models are the 1.0 models there. This is a proof of the predicate reductions; the byte-level "no result change" follows because every clause reads the mode only through these two predicates (plus `== BmpFile`).
  - `slack_mode_same_prefix(b, m)` — `mh(b, IcoEntryDibSlack) == Ok(with_mode(m, IcoEntryDibSlack))` when `mh(b, IcoEntryDib) == Ok(m)`; the two ICO modes share R1–R7.
  - `slack_mode_suffix_phase(b, m, pos, x_len, out0, r)` — with `m.mode == IcoEntryDib` and `r = mp_mask_phase(b,m,...)`, the two modes' mask phases differ **only** when `r = Err(TrailingBytes)`, and then `mp_mask_phase(b, slack) = Ok((pos+and_bytes, ma_output))`; every other result (`Ok` payload, `InvalidImageSize`, `TruncatedAndMask`) is identical.
  - `slack_mode_suffix_result(b, r)` (the whole-`MP` level, the name the delta gives) — `MP(b,Slack)` and `MP(b,IcoEntryDib)` differ only when the latter is `Err(TrailingBytes)`, in which case the former is `Ok` (a completed image); every other result — success image and every other error — is identical. Lifted through the raw/RLE phases by the `ico_pair` congruence lemmas (`conv`/`raw_conv`/`raw_full`/`mr_full`/`mr_out`/`mr_raw_out` all produce identical outputs; the raw and RLE phases never return `TrailingBytes`, proved by `mr_cur_err_kind`).
- `MP(b, IcoEntryDib)` and `MP(b, IcoEntryDibSlack)` are both stated over the full `forall b : Seq<u8>`; `accepted` also quantifies over mode.

### (3) Executable accepts the new mode; all existing theorems verify

- `parse_header`/`decode` accept `IcoEntryDibSlack` through the shared-ICO `is_ico` sites; the new mode reaches the **same code** as `IcoEntryDib` everywhere except the one final cursor test, isolated in `finish_mask` behind the single exec boolean `exact` and one `if exact && end != b.len()`.
- **T9 is preserved universally**: `decode_mp_eq` quantifies over `mode` with no precondition, and `finish_mask`'s `mask_done_eq` now carries the slack branch, so `decode = MP` holds for **all four** modes. **No STUCK.md entry and no dropped T-name.** T1–T13, L1–L3 all verify.
- T10 (`t10_error_precedence`) and T13 (`t13_success_bounds`) remain at their manifest quantification over `mode`; `mask_done_eq`'s documented error set (`InvalidImageSize`, `TruncatedAndMask`, `TrailingBytes`) is now correctly mode-dependent (slack never yields `TrailingBytes`).

### (4) Conformance on the 302 vectors

- `tests/conformance.rs` reads `../spec-1.1-frozen/VECTORS.json` (was `spec-frozen/`), asserts `vectors.len() == 302` and `passed == 302`, and its `mode_of` parser handles `IcoEntryDibSlack`. Runs **302/302**. The corpus test is unchanged: **84 files checked, 58 accepted**.
- `model_vectors.rs` was **not edited**: its 209 `agree_*` obligations (214 old vectors) still verify against the extended model — regeneration for the 88 new vectors is deferred to M1.1c as the brief allows.

## Exact verification output

```
$ ./verify.sh "M1.1a: clean full reverify"     # after cargo clean
verification results:: 1398 verified, 0 errors          (runs/038.log, 100 s)

$ ./verify.sh "M1.1a: final shipped-source reverify"
verification results:: 1398 verified, 0 errors          (runs/040.log, 105 s)

$ cargo test --release
conformance: 302/302 vectors passed
corpus: 84 files checked, 58 accepted
```

Obligation count moved 1,367 (1.0) → 1,398 (+31: the new model lemmas and congruence helpers; the 209 `agree_*` and all old obligations are among the 1,398).

## Deferred to later milestones (honest gaps)

- **M1.1c — `model_vectors.rs`**: not regenerated for the 88 new vectors (delta-explained, explicitly allowed by the dispatch). The 214 old `agree_*` verify unchanged.
- **M1.1b/M1.1c — non-vacuity witnesses**: the six new proof-gate witnesses (`slack-1-new`, `slack-1-old`, `image-size-9-slack`, `mask-short-3-new`, `invalid-size-and-short-new`, `alpha32-7f-mask80-new`) are **not** added here. The model theorems are proved at full `forall b` quantification (not vacuously), but the named concrete witnesses are M1.1b/M1.1c.
- **M1.1c — mutations and `decode_with_steps`**: the delta's implementation mutations and the public `decode_with_steps` export (the private `t12_linear_work` body) are M1.1c.
- No 32-bit verifier run (argument by hand, as for 1.0).

## Notes on the proof work (engineer log)

- The mask-phase bridge (`slack_mode_suffix_phase`) and the `MP`-level lift were the bulk of the effort: the phase body's `exact_tail` test only affects the `end != b.len()` branch, so both modes' unfolding is definitional once the shared `is_ico` guard and the AND-availability test are discharged.
- `ico_pair` (two ICO records agreeing on all fields but `mode`) plus one congruence lemma per model function (`conv`, `raw_conv`, `raw_full`, `mr_full`, `mr_out`, `mr_raw_out`, `run_writes`, `abs_writes`, `and_writes`, `pal_zero_bytes`, `mp_decoded`) carry the raw/RLE phases across the mode change. Transitive dependence on `mode` is limited to `cell_of` (via `top_down`), `conv`/`conv_word` (via `is_ico`), `index_ok`/`raw_index_ok` (via `count`/`depth`), and `and_bit`/`and_stride` (via `width`/`height`) — all captured field-for-field by `ico_pair`.
- Four runs (7, 10, 11, 14) were parse/type errors from early drafts of the congruence section and do not count as verifier body failures. The only obligation that needed more than five body-check attempts was none: each failing obligation (the `MP`-level lift) was reorganized into smaller lemmas rather than re-attempted unchanged, so no STUCK.md entry was warranted.
