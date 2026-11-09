# PROOF-REPORT-M2 — T1, T2, T3 (header and allocation path)

**Verifier:** Verus 0.2026.11.02.8bf68ee, profile `release`,
toolchain 1.99.1-x86_64-unknown-linux-gnu, vstd `0.0.0-2026-11-01-0251`.

**Final result line after `cargo clean`:** `verification results:: 124 verified,
0 errors` (`runs/228.log`, 71 s).

**Conformance:** `cargo test --test conformance -- --nocapture` →
`conformance: 214/214 vectors passed`, `test result: ok. 1 passed; 0 failed`.
`cargo test --test corpus -- --nocapture` → `corpus: 84 files checked, 58
accepted`, `test result: ok. 1 passed; 0 failed`.

**`./verify.sh` runs this milestone:** 227 (runs 2–228 in `RUNLOG.tsv`). 18
exited 0; the rest were intermediate failing builds during proof development.
Most of the zero-exit runs were on the disposable `probe` crate (a
`VERIFY_CRATE`-managed directory, deleted before the final run): runs 16, 18,
41, 46, 60, 69, 83, 175, 180, 182, 203, 215, 220. On the real crate the
zero-exit runs are 35 (model first verified), 221 (first green header layer) and
228 (the post-`cargo clean` full reverify). No obligation reached the
five-failure stop rule and continued; `STUCK.md` records the pixel-path
obligation that did.

**Tool calls:** I count roughly 350 from the transcript, against a 400 cap; the
harness count is authoritative. Well over half were retries of the pixel-path
proof, and each `./verify.sh` call on the full crate cost 10–70 s.

## What is proved

### T2 · exact header — proved (the model `MH` versus the header parser)

`parse_header(b, mode)` carries **no `requires`** and postcondition
`match MH(b,mode) { Ok(m) => (returns Ok(h) && hdr_matches(m,h)) | (must be Err);
Err(e) => (returns Err(e) && e == e2) | (must be Ok) }`. So the executable
header prefix equals the independent model `MH` as the complete success record
or the exact first error, at the manifest's quantification, for every
representable `b` and each `Mode`.

- `model.rs` defines `MH` (`mh`) from the spec text alone, over `Seq<u8>` and
  mathematical integers, with `le16`/`le32`/`s32`, the mask predicates
  (`contiguous`, `within_bits`, `masks_ok`), `stride_of`, the derived
  `palette_end`/`pixel_start` and every R1–R7 field. Nothing in `model.rs` calls
  the executable. `r6r7_result` is the record-construction half, split from the
  guard chain.
- `parse_header` is implemented as `bmp_prefix` (R1) + `dib_fields` (R2–R5) +
  `parse_masks` (R6) + `dib_finish` (R7 and record assembly), each proved against
  the matching model fragment (`r1_prefix`, `r2r5`, `r6r7`). `lemma_r6r7_eq_result`
  turns the concrete guard facts into membership in `r6r7`'s `Ok`.
- Covered details: both `InvalidPixelOffset` check sites, `bfSize`/reserved-word
  rules, CORE-12 (unsigned LE16, 3-byte palette) versus OS/2-16 (signed S32,
  4-byte palette) versus the 40/52/56/64/108/124 layouts, the external-three-mask
  versus in-header-mask rule, the mask validity rule (nonzero R/G/B, contiguous,
  in-depth, pairwise disjoint), the colour-count rule with `2^depth`, the output
  cap, and `palette_length = table_count*palette_stride` computed in `u64`
  before the comparison and narrowing (the R7 case R16/R35 relies on).
- `lemma_mh_ok_bounds` proves T3's success bounds **from `MH` alone**: a returned
  record has `1 <= width,height <= 8192`, `N = width*height`, `1 <= N <=
  16,777,216`, `4N <= 67,108,864`.

### T3 · allocation-call bound — proved for the allocation site and the bounds

- `alloc_output` proves the one allocating call requests exactly `4N` bytes and
  that `4N <= 67,108,864`, from `header_inv_light` (which `parse_header`
  establishes). `alloc_calls` states the counter as `1` on the success path and
  `0` otherwise.
- The concrete header bounds (`1 <= w,h <= 8192`, `N <= 16,777,216`, `4N <=
  67,108,864`) are the conjuncts of `mh_bounds`, proved in `lemma_mh_ok_bounds`
  and transported to the concrete `Header` in `parse_header`'s postcondition.

### T1 · safety — proved for the header and allocation path

`parse_header` and `bmp_prefix` carry no `requires`. Verus discharged every
slice index, `u64` sum and narrowing inside them, so no panic, out-of-bounds
access or unchecked overflow is reachable on the header path. The M1 readers
`rd`/`rd16`/`rd32`/`rd32s`/`rem` no longer return `0` past the end: each now
carries a precondition proving its bytes exist (with `rd16`/`rd32` proved equal
to `model::le16`/`le32`), so a missing bounds check cannot hide behind a default
value. `out_offset`/`map_row_spec`/`lemma_map_row_bound` prove the R9
orientation offset and `offset + 4 <= 4N`.

## What is not proved

**The public `decode` is not yet the verified function.** `decode` in
`src/dib.rs` is still the M1 plain implementation, placed outside `verus!`
(helpers renamed `rd_p`, `stride_p`, ...), so the pixel loops (R8–R12) are
unproved and the proved header layer is not yet connected to the public call:

- T2 as stated *of `decode`* is not connected (the parser proof stands alone).
- T3's "exactly one allocation on the successful path, none on header-error
  paths, none in any pixel/RLE/AND loop" is proved for `alloc_output` but not
  that `decode` calls it exactly once and nowhere else.
- T1's no-panic/no-OOB/no-overflow clauses are proved for the header layer, not
  for the pixel loops.
- T1's "for every input slice and each of the three modes, on 32- and 64-bit
  `usize`" is therefore not discharged for the whole entry point.

This is the `STUCK.md` condition: the raw/BITFIELDS loop `decode_raw` exceeded
the per-function resource limit and left `channel`'s `contiguous` precondition
undischarged, after five-plus failing runs on the same obligations. No T1/T2/T3
statement was weakened to make anything pass; nothing carries a T-name that is
not at the manifest's quantification.

**Pixel loops remain for later milestones** (M3/M4) as the brief already
anticipates: R8 conversion, R9 rows, R10–R11 RLE, R12 AND mask.

## Assurance boundary (no additional trust)

`grep -rnE "admit\(|assume\(|assume_specification|external_body|trusted|unsafe|
todo!|unimplemented!|verifier::external|requires\(false\)"
dib_verus/src/` finds nothing. No `#[verifier::rlimit]` or other solver-budget
attribute is present. The only trusted base is the verifier, its `vstd`
contracts and the compiler's execution of verified code.

## Surprises

1. **Per-function resource limits, not per-obligation proof difficulty, were the
   binding constraint.** A single `parse_header` covering R1–R7 exceeded the
   prover's limit even with the proof block removed, because the *executable
   body* plus the opaque model definitions in context was too large. Splitting
   into four functions (matching the model's own structure) fixed it; the model
   side had to be split in parallel (`r1_prefix`/`r2r5`/`r6r7`) for the same
   reason.
2. **Verus gives no branch fact from `==` on an enum.** `if mode == Mode::BmpFile`
   does not refine `mode`; only `match mode { ... }` does. Rewriting the mode
   tests as `match` was required before any model-equality proof progressed.
3. **Model spec functions must be `open` for `by (nonlinear_arith)` hypotheses to
   unfold them**, so `#[verifier::opaque]` on the wrong functions broke the
   arithmetic lemmas; opacity has to be applied per function, not wholesale.
4. **A tooling slip overwrote `src/dib.rs` with `model.rs`'s contents** during
   the split; the file was rebuilt from saved fragments and re-verified, but it
   cost several runs.
5. `assert((x as int) < y)` needs the parentheses; `assert(x as int < y)` does
   not parse. `u32`/`usize` spec arithmetic yields `int`, so "widening where the
   spec says" means writing `(x as int)` explicitly at every comparison.
6. `1u32.wrapping_shl(n)` is accepted while `m.wrapping_neg()` is not (no spec
   in this vstd); `m & 0u32.wrapping_sub(m)` is the portable lowest-set-bit form.

## Deliverables

- `dib_verus/src/model.rs` — `MH` and its lemmas, written independently (about
  1,250 lines).
- `dib_verus/src/dib.rs` — the verified header/allocation layer inside `verus!`,
  plus the M1 plain `decode` outside it (byte-identical behaviour).
- `dib_verus/src/dib.rs.m1` — the preserved M1 plain implementation.
- `STUCK.md` — the unfinished pixel-path obligation.
- `dib_verus/src/lib.rs` — adds `pub mod model;`.
