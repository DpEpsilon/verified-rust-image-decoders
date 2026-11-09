# PROOF-REPORT-M4c — closing the Sol review: T6 range, T8 projection, OutsideDomain, executables-only mutations

**Milestone:** M4c. The Sol review (`REVIEW-SOL-TGA.md`) found that the M4 package
did not establish everything the frozen manifest claims. This milestone closes
Findings 1, 3, 4, 6 and 2/7 at the manifest's exact quantification, without
weakening anything and without editing `THEOREMS.md`, `spec-frozen/` or the
vectors.

**Verifier:** Verus 0.2026.11.02 (binary `/opt/verus`, `rust_verify`), release
profile, toolchain 1.99.1-x86_64-unknown-linux-gnu, vstd
`0.0.0-2026-11-01-0251`.

**Final result line:** `verification results:: 636 verified, 0 errors`
(`runs/460.log`, a full reverify after `cargo clean`, 85 s). M4b was 578.

**Conformance:** `cargo test` → `conformance: 169/169 vectors passed`,
`test result: ok. 1 passed; 0 failed`.

**Frozen files:** `spec-frozen/*` and `THEOREMS.md` unchanged (the vectors, the
generator's expected values and `model_vectors.rs` are untouched).

**Runs this milestone:** 297–460 (164 logged runs, 24 exited 0). Every run went
through `./verify.sh`; no scratch crate, no direct `cargo verus verify`.

## Finding 1 · T6 at arbitrary starting cursor and produced count

T6 previously held only at `produced0 = 0`. Now:

- **`decode_rle` takes an entry produced count.** Its signature gained `p0g:
  usize`; `produced` starts at `p0g`. The postconditions are stated from the
  entry state `(c0g, p0g)`: `rle_result_eq_from`, `rle_pixels_eq_from`,
  `rle_full_eq_from`, the `MR(s,H,map,cursor0,produced0)` projections.
- **The pixel model is offset.** `model::wrote_px_off` (with `rle_quad_ok_off`)
  writes the loop's `k` produced pixels at oriented cells `f(p0g + j)`,
  generalizing `wrote_px` (`lemma_wrote_px_off_zero`); the loop maintains
  `wrote_px_off(out@, acc, ..., p0g, produced - p0g)` and its per-step lemma
  `lemma_wrote_px_off_write_step`.
- **The witness takes `produced0`.** `t6_rle_exact(bytes, h, map, indexed, out,
  cursor, n, p0)` with `p0 <= n` states `rle_full_eq_from` from
  `(*old(cursor), p0)`.
- **The failure case carries the ghost cursor.** `model::mr_gcur` is `MR`'s ghost
  cursor projection (final cursor on `Ok`, the cursor held at the failing check on
  `Err`); the loop maintains `mr_gcur(c0g,p0g) == mr_gcur(cur,produced)`, with
  the per-branch step lemmas `lemma_mr_gcur_step_rep`, `_step_lit`,
  `_step_lit_err`. `decode_rle`'s postcondition states the concrete final cursor
  equals `MR`'s ghost cursor on failure.
- **The failure case's first error is exact** (`rle_result_eq_from`), and
  `rle_fail_eq_from` (error, ghost cursor, partial pixels at oriented cells) is
  proved at all five error exits by `lemma_rle_fail_from_split` over the model's
  per-branch exact error tuples (`lemma_mr_err_ctrl_absent`, `_overrun`,
  `_rep_missing`, `_rep_index`, `_lit_missing`, `_lit_index`).

**Not closed, stated precisely.** The partial-pixel half of the failure ghost
state is proved as `rle_fail_eq_from` at every exit (so the model's ghost pixels
and the concrete partial output are tied), but the public `decode_rle`
postcondition states only the ghost-cursor clause directly, because Verus's
`final(out)@` at the loop's error exits could not be bridged to the invariant's
`out@` within the solver budget (five failed runs on that obligation). The
error tag and the ghost cursor *are* in the public postcondition. This is the one
place M4c's T6 is stronger than M4a but short of the literal manifest text; it is
recorded here rather than narrowed silently.

The public decode proof (T7, `t7_decode_exact`) stays unconditional: `decode`
calls `decode_rle` at `p0g = 0`, where `rle_full_eq_from` reduces to the previous
`rle_full_eq` (`lemma_rle_full_eq_from_zero`), so `decode_eq` is unchanged in
content and in quantification.

## Finding 3 · the T8 projection lemma and the per-cell statement

- `model::lemma_mp_accepted_shape` proves `MP(b) = Ok((w,h,out)) ==>
  mp_ok_shape(b,w,h,out)` (MP's own success shape, record supplied by `choose`).
- `model::t8_cell` states the manifest's per-cell claim over `MP(b) = Ok(w,h,out)`
  (not `mp_ok_shape`); `model::lemma_mp_t8_cell` discharges it for every accepted
  `b` and every `q < N`, via `lemma_fcoord_inverse` and
  `lemma_orient_bytes_index`.
- `tga::t8_output_cells` now takes the hypothesis `model::mp(b) == Ok((w,h,out))`
  and concludes `t8_cell`, calling `lemma_mp_t8_cell`.
- `t7_decode_exact` — the public result proof — now asserts `t8_cell` for every
  output cell `q < N` on `Ok`, so the per-cell statement is called from the
  public proof, not left as an orphan witness.

## Finding 4 · `OutsideDomain` as a distinct marker

`model::MrErr { Api(Error), Outside }` is the model's result type: `Api` wraps
exactly one of R15's 18 errors, `Outside` is THEOREMS.md:13's internal proof
marker, never a nineteenth error. `mr_at`, `mr`, `mr_cur` and `mr_gcur` return
`MrErr`; the out-of-domain branch is `Err(MrErr::Outside)`. `MP`'s `mp_after`
unwraps `MrErr::Api(e)` to the API `Error`; its `Outside` arm (unreachable, since
`mp_after` only calls `MR` in-domain) returns an ordinary error only to keep the
model total, and is documented as such. The executable `Error` enum is unchanged;
`mp_after`'s calls are all in-domain, so `MP` never observes `Outside`.

## Finding 6 · R1 x/y origin fields

`MHModel` gained `x_origin`/`y_origin` (`LE16(8)`, `LE16(10)`), the executable
`Header` carries the same two fields built in `parse_header`, and `hdr_matches`
relates them (proved in `parse_header`'s T5 postcondition). They are metadata
only (R1: "Coordinates are metadata, not an extra crop or offset"; R9's flips use
descriptor bits 4/5), so no pixel changes and no fixture changes.

## Findings 2 and 7 · the executable-side mutation campaign

The 13 model-only mutations (m1–m7, m9, m10, m12, m18–m20) are kept as
**model-sensitivity** evidence under `mutation/model-side/mN/` (their
`failure.log` and `result.txt` are untouched). Each now also has an
**executable-side** mutation under `mutation/exec-mN/`, rebuilt from the pristine
`mutation/base2/` (a fresh copy of the current live `src/`), editing only
`src/tga.rs`; the frozen model, `model_vectors.rs` and the vectors are untouched.
`tools/mutations_exec.py` applies them; `tools/run_exec_mutations.sh` runs one
copy through `VERIFY_CRATE=mutation/exec-mN/tga_verus ./verify.sh`, keeping
`result.txt` and `failure.log` and deleting the copy's caches.

All 15 executable mutations fail with a **verifier** diagnostic (never a
compile/type error and never a runtime-test failure); each run's crate line is
`636 verified, ... errors` with the diagnostic counted above it:

| # | executable mutation | diagnostic (verifier) | log |
| --- | --- | --- | --- |
| exec-m1 | input cap `>` → `>=` | postcondition not satisfied (T7/T9, L2) | `mutation/exec-m1/failure.log` |
| exec-m2 | map-flag check before type check | postcondition not satisfied (T5/T9) | `mutation/exec-m2/failure.log` |
| exec-m3 | dimension cap 8192 → 65535 | postcondition + requires not satisfied (T5/T8) | `mutation/exec-m3/failure.log` |
| exec-m4 | map `first` ignored in `convert_index` | assertion + possible arithmetic underflow (T4/T7) | `mutation/exec-m4/failure.log` |
| exec-m5 | bit 15 as alpha for 16-bit attr 0 | postcondition not satisfied (T7) | `mutation/exec-m5/failure.log` |
| exec-m6 | RLE count `(control&0x7f)` | assertion + loop variant (T3/T6/T7/T10) | `mutation/exec-m6/failure.log` |
| exec-m7 | availability before overrun | assertion not satisfied (T6/T9) | `mutation/exec-m7/failure.log` |
| exec-m9 | vertical flip reversed | assertion not satisfied (T7/T8) | `mutation/exec-m9/failure.log` |
| exec-m10 | any suffix accepted (no footer) | precondition not satisfied (T7/T9) | `mutation/exec-m10/failure.log` |
| exec-m12 | rounded scaling instead of replication | requires not satisfied (conversion/T7) | `mutation/exec-m12/failure.log` |
| exec-m18 | stored A byte at 32-bit attr 0 | assertion not satisfied (conversion/T7) | `mutation/exec-m18/failure.log` |
| exec-m19 | greyscale-16 second byte at attr 0 | assertion not satisfied (conversion/T7) | `mutation/exec-m19/failure.log` |
| exec-m20 | bit 15 clear transparent at attr 1 | postcondition not satisfied (conversion/T7) | `mutation/exec-m20/failure.log` |
| exec-m15 | whole-literal guard removed | requires not satisfied at the packet slice (T1/T3, the out-of-bounds read) | `mutation/exec-m15/failure.log` |
| exec-m17 | palette range guard removed | precondition not satisfied at `convert_index` `map_first <= index` (T1/T4, the unchecked map index) | `mutation/exec-m17/failure.log` |

How the "verifier diagnostic, not a compile/type error" check was made: each
`result.txt` records the `error:` lines; the diagnostics are
`postcondition not satisfied`, `assertion failed`, `requires not satisfied`,
`precondition not satisfied`, `possible arithmetic underflow/overflow` and
`while loop: resource limit exceeded` — all Verus verification failures, with the
crate line `N verified, M errors` where compilation reached it. No log shows a
`error[E....]` rustc type error or an `error: expected` parse error. exec-m15 and
exec-m17 are the Finding 7 adjustment: the old model-only m15/m17 failed on a
pre-existing assertion's premise and an `index_in_range` postcondition, so their
new executable forms remove the guard itself and the diagnostic is the concrete
`convert_index` index precondition / the packet-slice read precondition, the
out-of-bounds/overflow obligation the manifest names.

## Finding 5 · no 32-bit verifier run (documented)

Only `x86_64` is installed, so no 32-bit run exists. `NOTES.md` now records the
exact bounds that make the proof width-independent: every value is below `2^31`
(R2 dimensions ≤ 8,192 so `N ≤ 16,777,216` and `4N ≤ 67,108,864`; R4 map
`≤ 262,140`; R1 input `≤ 67,108,864`; every cursor `≤ |b|`), all below
`isize::MAX`/`usize::MAX` on the 32-bit target, and Verus checks the same
`usize`/`isize` arithmetic, so a 32-bit run would discharge the same obligations.

## Assurance boundary

`grep -rnE "admit\(|assume\(|assume_specification|external_body|trusted|unsafe|todo!|unimplemented!|verifier::external|requires\(false\)" tga_verus/src/`
finds nothing. No new solver-budget attribute was added; the only one remains
M3's single `#[verifier::rlimit(80)]`. No `THEOREMS.md` or spec-frozen edit; no
vector change.

## Not closed

- Finding 1's failure-ghost **partial pixels** are a proved exit lemma
  (`rle_fail_eq_from`) but not a clause of the public `decode_rle`
  postcondition (the `final(out)@` bridge at the loop's error exits hit the
  solver budget; five failed runs, recorded here rather than narrowed).
- Finding 8 (nit) was not attempted: the bounded `4*N` output is still allocated
  before the ID/map availability checks, as before (R6 and the manifest bound
  permit it), to keep R14 error priority and the proof unchanged.
