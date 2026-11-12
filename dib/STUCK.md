> HISTORICAL: resolved by gpt-6-sol in M2b (see PROOF-REPORT-M2b.md). Kept for the record.

# STUCK — M2, T1/T3 pixel-path obligations in `decode`

## Obligation that would not discharge

`decode_raw` (the R8/R9 raw/BITFIELDS pixel loop), and with it the connection
from the proved `parse_header`/`alloc_output` layer to the public `decode`
entry point, which T1 (safety for every input and mode) and T3 (the
allocation-call bound "before the single allocating call prove the concrete
header has ...") both quantify over.

Concretely, the failing obligations were, in `fn decode_raw`:

1. the loop invariant `final(out)@.len() == 4 * h.spec_n()` not holding before
   the outer `while y < height` (Verus reports `invariant not satisfied before
   loop`);
2. `assert(x * depth / 8 < line)` inside the `depth <= 8` branch;
3. `possible arithmetic underflow/overflow` on `pos + x * depth / 8` and on the
   24/32-bit `pos + x * 3 + 2` / `pos + x * 4 + 3` reads;
4. `channel`'s precondition `crate::model::contiguous(mask)` not derivable at
   the BITFIELDS call site from the `masks_ok` precondition.

## Last error text (representative)

```
error: possible arithmetic underflow/overflow
    --> src/dib.rs:2224:25
     |
2224 |                 let p = pos + (x * (depth / 8));
     |                         ^^^^^^^^^^^^^^^^^^^^^^^

error: assertion failed
    --> src/dib.rs:2230:28
     |
2230 |                     assert(crate::model::contiguous(h.masks[3]) || h.masks[3] == 0u32);
     |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ assertion failed
```

and, on the earlier attempt at the same file, `resource limit (rlimit) exceeded`
on `fn dib_finish` (the whole R6/R7 second phase) before that function was
split into `parse_masks` + `dib_finish`.

## What I tried (each a `./verify.sh` run, all logged in `RUNLOG.tsv`)

- Splitting the single `parse_header` into `bmp_prefix` + `dib_fields` +
  `dib_finish`, then `dib_finish` into `parse_masks` + palette phase, to get
  under the prover's per-function resource limit. **This worked**: those
  functions now verify (124 obligations, 0 errors).
- Moving the model equality out of the exec body into a named `proof fn`
  `dib_finish_eq`, and adding `lemma_r6r7_eq_result` and
  `lemma_r6r7_ok_bounds` in `model.rs` so the membership in `r6r7`'s `Ok` is
  discharged from explicit guard hypotheses instead of unfolding the model's
  if-chain.
- Adding model-side arithmetic lemmas for the pixel path:
  `lemma_stride_times_8`, `lemma_sample_within_stride`,
  `lemma_subbyte_within_stride`, `lemma_subbyte_shift`. The byte-shift lemma
  makes `raw_index` verify; the stride lemmas do not yet let the solver see
  `x*depth/8 + depth/8 <= line` at the widened types in `decode_raw`.
- Making model spec functions `#[verifier::opaque]` selectively and then
  reverting; the opacity interacted badly with the `by (nonlinear_arith)`
  hypotheses that need the definitions unfolded, and did not remove the rlimit
  pressure in the exec function.

## Honest statement of where this leaves M2

The verified layer that *does* check is real and non-vacuous:

- `MH` (in `model.rs`) is the total R1–R7 header model, written independently of
  the executable; `lemma_mh_ok_bounds` proves T3's success bounds **from `MH`
  alone**.
- `bmp_prefix`, `dib_fields`, `parse_masks`, `dib_finish`, `parse_header` carry
  the T2 contract `concrete_header_prefix(b, mode) = MH(b, mode)` as the exact
  record or the exact first error, at the manifest's quantification (no
  `requires`), including both `InvalidPixelOffset` sites, CORE-12 versus OS/2-16,
  bitfields, palette counts, caps and the `u64` widening of `palette_length`.
- `alloc_output` proves the `4N` allocation request and its `<= 67,108,864`
  bound; `out_offset`, `map_row_spec`, `lemma_map_row_bound` prove the R9
  orientation offset and its `+4 <= 4N` cell bound.

What is **not** proved, and this is the STUCK condition: **`decode` is not yet
the verified function.** The public `decode` in `src/dib.rs` is still the M1
plain implementation, placed outside `verus!` (renamed helpers `rd_p`,
`stride_p`, ...). So:

- T2 is proved for the header parser, but T2 as stated on `concrete_header_prefix`
  *of `decode`* is not connected: the proof exists in isolation.
- T3's allocation bound is proved for `alloc_output`, but not that `decode`
  itself calls it exactly once on the success path and nowhere else.
- T1's no-panic / no-OOB / no-overflow clauses are proved for the verified
  header layer, not for the pixel loops of the executable `decode`.

The correct next step is to move the pixel loops inside `verus!` and prove them
against `parse_header`/`alloc_output`; the arithmetic lemmas they need are
already in `model.rs`, but the widened-type index bounds (`usize` versus the
model's `int`) repeatedly exceeded the per-function resource limit and, in the
bits-1/4/8 and 16/24/32 branches, left `channel`'s `contiguous` precondition
undischarged.

## Escalation

Per BRIEF.md rule 5 this is handed on. The implementer was
deepseek-v4.1-flash-verus; the ledger says the next tier is gpt-6-sol.

---

# STUCK — M4a, T7 (exact RLE) and T8 (exact ICO AND) at the manifest's quantification

## State at the time of writing

T6 (exact raw/BITFIELDS) is **proved**: `verification results:: 336 verified,
0 errors` (`runs/402.log`), `cargo test` 214/214 vectors and 84-file corpus (58
accepted). The T6 half is real, not a projection stub:

- `model.rs` defines `Write`, `bytes_replay`/`replay_bytes_at` (the exact byte
  level of the executable's `put`), `raw_full`/`raw_gcur`/`raw_writes` and their
  projection lemma `lemma_raw_full_proj`, all from SPEC R8/R9/R14 alone.
- `decode_raw` = `raw_loop` (src/dib.rs) carries the manifest's quantification: an
  arbitrary reachable start state `(pixel_start, 0, 0)`, every finite stream, and
  on **both** exits the model's exact first error (or success cursor) **and** the
  ghost final cursor/position, and `final(out)@ == bytes_replay(zero fill,
  raw_writes(...))` — the concrete pixels written so far *are* the model's
  partial output through the first failing check, not a separate exit lemma.
- The `paint_run` RLE run-painter (T7's per-run half) is also proved with the same
  ghost write accumulator and the `run_good` partial-paint clause.

## Obligation that would not discharge

T7's command loop and T8's AND helper, at the manifest's quantification.

1. **T7 command loop.** The full `rle_loop` (with the ghost cursor/position and
   `mr_writes` accumulator, mirrored on the proved `raw_loop`) was written and
   iterated through runs 382–397. It did not converge before the budget ran out.
   The residual failures were: (a) connecting the freshly-accumulated ghost
   `gw = gw + run_writes(...)`/`+ abs_writes(...)` back to `mr_writes(pos0,0,0)`
   at every exit, which needs the model's *recursive step* for `mr_writes` (a
   `mr_writes(p) == run_writes(p) + mr_writes(p')` lemma and its absolute twin)
   that was not yet proved; and (b) the EOB/success-shape conjuncts
   (`gst.1 == 0`, `gst.2 == height`).
2. **T8 AND helper.** `ma_mask`/`and_writes` are defined in `model.rs` (from R12
   alone), but `apply_and`/`finish_mask` were not given the matching exact
   contract, so no T8 obligation is stated on the executable yet.

## What was tried (each a `./verify.sh` run, logged)

- Runs 382–397: the `rle_loop`/`paint_abs`/`fill_rle` construction; the model
  gained `lemma_run_writes_{unfold,empty,single,snoc,snoc_at}`,
  `lemma_abs_writes_{unfold,empty,single,snoc_at,full}`, `run_good`/`abs_good`
  prefix lemmas, `lemma_mr_full_proj`, `lemma_run_good_all_ok` and the pack-index
  lemmas. These compile and prove individually (runs 343, 380, 381 green); the
  loop-level connection is what does not close in the remaining budget.
- Runs 396–402: reverting the unfinished T7 exec block to the last green state
  (`runs/380`, `380`, then `402` clean). The model-side T7/T8 *definitions* and
  all their standalone lemmas are kept; only the exec contract is incomplete.

## Honest boundary

- **T6: proved** (manifest quantification, error + ghost state + partial output).
- **T7: not proved** on `decode_rle`; the run-painter half (`paint_run`) is
  proved, the command-loop half is defined and its lemmas are proved, but the
  loop's postcondition is not discharged. Nothing weaker carries a T-name.
- **T8: not proved**; `MA` is defined in the model, no exec contract yet.
- No `assume`/`admit`/`external_body`/`trusted`/`unsafe`/`todo!`/`requires(false)`
  and no solver-budget attribute anywhere in `src/`. Frozen `spec-frozen/` hashes
  unchanged.

## Escalation

Per BRIEF.md rule 5 the T7/T8 remainder should be handed on: the implementer was
deepseek-v4.1-flash-verus; the ledger's next tier is gpt-6-sol. The remaining work
is mechanical given the proved `raw_loop` template: prove the `mr_writes`
recursive-step lemmas, then replay the `raw_loop` structure for `rle_loop`.

> HISTORICAL: the M4a T7/T8 remainder below (both sections) was closed by Fable in M4a3
> (`runs/428.log`: 378 verified, 0 errors; see NOTES.md "M4a3"). Kept for the record.

## M4a2 (gpt-6-sol, saved by the coordinator from Sol's reply): T7 still open, T8 partial

Backups src/dib.rs.m4a2, src/model.rs.m4a2. Clean reverify run 421: 363 verified, 0 errors; 214/214; corpus 84/58; runs 404-421 (green: 404, 407, 411, 414, 418, 420, 421). No budget attribute or escape added.
- T7 NOT proved on executable decode_rle: Sol proved the encoded-run and absolute-packet recursive-step equalities for mr_writes and mr_cur in model.rs (these are lemmas, not T7), but the command loop still has neither a model-error/ghost-position postcondition nor the partial-output clause. paint_run is proved but unused by decode_rle.
- T8 gained an executable exact-output contract: apply_and proves byte-for-byte equality to ma_output (unchanged RGB, bottom-up MSB mask selection, alpha writes); alpha_present proves the whole-image XOR-alpha predicate; finish_mask states and proves equality to the new ma_exact model on both image-size/mask errors (buffer unchanged) and success/trailing-byte failure. NOT claimable as manifest T8: the older authoritative ma_mask/and_writes models (write list with zero RGB components) are not connected to ma_exact, and ma_exact returns API errors rather than Outside for out-of-domain states. Run 419 tried an Outside-domain guard; an unrelated nonlinear mask-prefix proof hit a solver resource limit; reverted (no budget attribute).

---

# STUCK — M4b, T12 (linear work) at the manifest's quantification

## What was closed in M4b (not stuck)

- **T9** integrated correctness: the public `decode` now proves
  `decode_mp_eq(r, bytes@, mode)` with no `requires` — `decode(bytes, mode)`
  equals `MP(bytes, mode)` as the exact `Result` including every output byte,
  in all three modes. The proof threads `parse_header` (T2,
  `hdr_matches`/`hrec`), the raw helper (`raw_done_eq`, T6), the RLE helper
  (`rle_exact`, T7) and the mask helper (`mask_done_eq`, T8) into `mp`'s own
  phases; `model::lemma_mp_mh_err`, `lemma_mp_ok_shape`,
  `lemma_mp_mask_phase_ico_ok`, `lemma_mp_mask_phase_size_err`,
  `lemma_mp_raw_err`, `lemma_mp_rle_err`, `lemma_mp_mask_phase_err_raw/rle`
  are the bridge lemmas. All are proved (403 verified, 0 errors, `runs/626.log`).
- **T10** error precedence: `t10_error_precedence` states
  `(r == Err(e)) == (MP(b,mode) == Err(e))` on the actual entry point, proved
  from T9. R14's operational ordering is carried by the T2/T6/T7/T8 helper
  postconditions (each returns the model's exact first error).
- **T13** success/resource: `t13_success_bounds` states the exact
  `MP(b,mode) = Ok(w,h,p)` iff conditions and `|p| = 4wh <= 67,108,864` on the
  public entry point; the per-success clauses of THEOREMS T13 (cells
  initialized before read, each raw cell assigned once, each RLE cell painted
  at most once after palette-zero init, alpha non-premultiplied, cursor
  conventions) are proved inside the T6/T7/T8 helper postconditions.
- **L1** (`lemma_mp_l1`): every `|b| > 67,108,864` is `FileTooLarge` in every
  mode, for an arbitrary oversized byte sequence (the invalid `BM` header of
  `precedence-too-large-bmp-bad-magic` need never be expanded).
- **L2** (`lemma_mp_l2`): `MP(0^67,108,864, BareDib) = Err(UnsupportedHeaderSize)`
  — the input cap is `>` not `>=`, at the exact boundary.

## Obligation that would not discharge: T12 on the executable

`forall b,mode: steps(decode(b,mode)) <= 16*(|b|+N)`, with `steps` a ghost
counter incremented once per decoder-controlled loop-body iteration (R18's
accounting, including the `4N`-byte output fill at `4N` and the ICO alpha scan
at `N`).

The model-side half was written and proved (a `step_cap`, and the phase
budgets `raw_step_bound(n,h) = n+h+1`, `rle_step_bound(l,n) = l/2+n`), and the
leaf loops were instrumented and proved:

- `alloc_output` charges exactly `4N`;
- `raw_loop`/`decode_raw` charge one step per sample, one per row advance and
  one final height check (`steps <= old + N + height + 1`) — **proved**;
- `fill_rle` charges exactly `N` — **proved**;
- `paint_run`/`paint_abs` charge exactly one step and one paint per painted
  pixel — **proved**;
- `alpha_present` charges at most `N` — **proved**;
- `apply_and` charges at most `N` — **proved**.

The RLE **command** layer (`rle_command`/`rle_loop`) is where it does not
close. The coupling needed is
`steps_new == steps_old + 1 + (painted_new - painted_old)` per command (one
command step plus one step per painted pixel), carried so that
`steps <= s_start + (#commands) + painted` with `#commands <= |b|/2 + 1` and
`painted <= N`. Two obstacles, both recorded:

1. **The ghost counters must be `&mut Ghost<nat>` parameters, not ghost locals
   threaded via `&mut Ghost(local)`.** A probe (`runs/559/560/561`) showed that
   `inner(k, &mut Ghost(painted))` does **not** write back to the caller's ghost
   local `painted`: the temporary is a fresh `Ghost`, and reads of the caller's
   `painted@` afterwards see the un-mutated value. So the counter must be a
   `&mut Ghost<nat>` parameter of every function on the RLE path, exactly as
   `steps` already is. `paint_run`/`paint_abs` were given their `painted`
   parameter this way and prove; `rle_command`/`rle_abs_packet`/`rle_loop`
   were converted the same way.
2. **`rle_command` (and then `decode_tracked`) exceeded the per-function
   verifier resource limit.** Once the exact `+1` coupling and the painted
   position bound were added, `fn body check: resource limit (rlimit) exceeded`
   appeared on `decode_tracked` (`runs/622-624`, log lines `error: function
   body check: resource limit (rlimit) exceeded; consider rerunning with
   --profile`). Adding `#[verifier::rlimit]` is forbidden by the brief, and
   splitting the RLE command body further did not remove the pressure before
   the budget ran out.

## What was tried (each a `./verify.sh` run, all logged in RUNLOG.tsv)

- Runs 452-560: instrumenting `alloc_output`, `raw_loop`, `fill_rle`,
  `paint_run`, `paint_abs`, `alpha_present`, `apply_and` with ghost counters
  (all green: `runs/626` shows the raw/fill/paint/alpha/and counters prove).
  The raw-loop arithmetic (`M = y*w + x + y <= N + height`, the `+1` boundary)
  took ~30 runs to state so Z3 would close it.
- Runs 561-596: the RLE command coupling. The
  `steps_new == gs_before + 1 + (painted_new - gp_before)` assert never
  discharged at the `rle_loop` call site because the `&mut Ghost(gpaint)`
  temporary did not alias the caller's local (probe `runs/559-561`).
- Runs 597-625: converting `painted` to a `&mut Ghost<nat>` parameter and
  re-deriving the invariant `steps <= s_start + gcmds + painted`,
  `gcmds <= |b|`, `painted <= N`; measured and re-measured the required slack.
  It ended in the `rlimit` error above rather than a stated obligation.

## Reverted state

The `T12` work was reverted to the last green checkpoint
(`dib.rs.m4a3b`/`model.rs.m4a3b`-based `dib.rs`/`model.rs`, the run-426+ state
plus the M4b T9/T10/T13/L1/L2 additions). The delivered files prove T9, T10,
T13, L1, L2 and the leaf-loop step counters for the raw/fill/paint/alpha/and
helpers; they do **not** carry a `T12`-named witness. Nothing weaker is
labelled `t12_*`: there is no `t12_*` item. The `steps`-counter clause on the
public `decode` is **not** stated (it was reverted), so the manifest's T12 is
**not claimed**.

## Escalation

Per BRIEF.md rule 5 this is handed on. The implementer was
deepseek-v4.1-flash-verus; the ledger's next tier is gpt-6-sol. The remaining
work is the RLE command/loop ghost-counter threading with R18's per-command
`+1` and per-pixel accounting, keeping each function under the per-function
resource limit (the likely fix is to move the per-pixel charge into the
already-proved painters' `painted` parameter and to keep `rle_command`'s
coupling as a named `proof fn` so the exec body stays small).

---

# P2a · the `r6r7_policy` connection (unfinished; work preserved)

Status: **stopped after far more than the 5-failed-run escalation budget.** The
delivered `dib_verus/src/dib.rs` and `model.rs` are the P1 baseline (run 276:
1594 verified, 0 errors); all P2a work is preserved in
`dib_verus/src/dib.rs.p2a-wip2` and `dib_verus/src/model.rs.p2a-wip2`
(state: 1633 verified, 3 errors) plus the earlier `*.p2a-wip` (1724 verified,
10 errors).

## The obligation

`fn dib_finish` (`dib.rs`, the two exit proofs at `dib.rs:2919` and the
`hdr_matches` bridge at `dib.rs:2919`/`model.rs:8644`) plus the spec-level
Strict bridge `lemma_r6r7_policy_strict`. The stubborn part is that the
executable's `dib_finish` returns `Header`-shaped data with mask words that read
BITFIELDS masks from **two different offsets** depending on `hsize`:

* `hsize == 40 || hsize == 64`: three external masks immediately after the
  header, at `start + size` (`s + rf.hsize`) — four-byte words with alpha zero;
* `hsize >= 56`: four internal masks at `start + 40 .. start + 56`;
* `hsize == 52`: three internal masks at `start + 40 .. start + 52`, alpha zero.

The model's `raw_masks_policy` duplicates those offsets, and the initial version
used `rf.pos0` (which is `s + hsize`) for the external case and `s + 40` for the
internal case. The mismatch (external words computed from `rf.pos0` vs the
executable's `start + size`) produced silent word-difference assertion failures
whenever the two coincided numerically but the prover could not see it.

## Attempts (all after the model was restructured to expose *exactly* the
## executable's raw words)

1. `lemma_r6r7_policy_record` with `ms0..ms3` hypotheses — 2 errors, the mask
   words did not line up.
2. `lemma_r6r7_policy_none` with a `raw_masks_policy` hypothesis — 4 errors,
   the `okv` validity flag could not be tied to the substituted words.
3. `lemma_r6r7_policy_none_raw` with the raw words as parameters and the
   `masks_after_policy`/`masks_ok_policy` pair — the record equation held but
   the guard hypotheses (`rf.compression == 3`, `is_bmp`, `p.offset == …`) kept
   failing to instantiate on the executable side.
4. `lemma_r6r7_policy_trunc_masks`/`_masks_err`/`_offset_err`/`_palette_err`/
   `_ok`, one per branch, with `pe` (the palette end) passed explicitly —
   offset and palette branches now pass; the `ok` branch (2 errors) and
   `lemma_r6r7_policy_strict` remain.
5. `if`-nesting of the guard conjunctions in `dib_finish` to expose each
   conjunct — worked for the planes/dimensions/colour-count guards (`dib_fields`
   now proves, 1594→1633 verified), did **not** close the last two.

## Diagnostics (last run, `runs/276`-adjacent WIP state, 1633 verified, 3 errors)

```
error: assertion failed                 src/dib.rs:2919  (the r6r7_policy == Ok(m) assert)
error: precondition not satisfied       src/dib.rs:2752  (lemma_r6r7_policy_ok: a guard hypothesis)
error: assertion failed                 src/model.rs:8644 (lemma_r6r7_policy_strict)
```

## Likely fix (for the next tier)

`lemma_r6r7_policy_strict` is the load-bearing one. The clean route is to prove
it **by the same `r6r7`-guard decomposition the executable uses**, with
`r6r7_policy` re-stated as `if r6r7_policy_err(..).is_Some then Err(..) else Ok(r6r7_result_policy(..))`,
and to prove `r6r7_result_policy` equal to `r6r7_result` **given** the four mask
words and the table count as hypotheses (`raw_masks_policy(..) ==
(r6r7_result's own raw words)`), rather than trying to normalize both sides. The
executable side then discharges those four equalities from its own `rd32` reads
at `start + 40`, `start + 44`, `start + 48`, `start + 52` (or `start + size` for
the external case), each an `as_u32(le32(..)) == mask_fields[i]` bridge of the
kind the other fields already use (`hdr_matches`'s `m.hsize == h.spec_hsize()`
pattern). The per-function resource-limit failures on `dib_finish`/`dib_fields`
(surfaced repeatedly) also point at shrinking each proof block into a named
`proof fn`, as `decode_tracked` did for T9.

## Escalation

Per BRIEF-POLICY (P2a). Implementer deepseek-v4.1-flash. Hand to gpt-6-sol.
Nothing weaker is labelled: the six options' `Policy` fields, their enum types,
the model's `r2r5_policy`/`r6r7_policy`/`mp_policy` extensions and the
`dib_fields` proof (all in `*.p2a-wip2`) are working; only the `dib_finish` →
`r6r7_policy` connection and its Strict bridge are open.

---

# P2a escalation · Sol handoff (runs 281–287)

Status: **incomplete**. The new `dib.rs.p2a-wip3` and `model.rs.p2a-wip3`
preserve this continuation; the live `src/dib.rs` and `src/model.rs` were
restored to the verified P1 baseline. Run 287 reports **1634 verified, 2
errors**, both per-function resource-limit failures on `dib_fields` and
`dib_finish`. There are no remaining reported assertion or precondition
failures. Do not call that a completed verification: a resource-limit failure
leaves the function unproved.

The earlier strict bridge was in fact false for arbitrary `RawFields`:
`raw_masks_policy` read external masks at `s + rf.hsize`, whereas the frozen
`r6r7` reads at `rf.pos0`; the lemma has no precondition tying those two
positions together. Reading at `rf.pos0` restores the exact Strict bridge
without weakening its quantification. The executable's `f.spec_raw().pos0`
equals `start + size`, so the external-mask proof also has the concrete offset
it needs. `lemma_r6r7_policy_strict` now verifies.

A separate real model mismatch surfaced in the `hdr_matches` proof:
`masks_of_policy_rf` applied `DefaultRGB` to zero masks even when
`compression != 3`, but the executable applies mask substitution only for
BITFIELDS. The model now returns zero masks outside BITFIELDS. The exact
`dib_finish` header/mask correspondence verifies. The policy success-output
length lemma now proves both Gdk blank and ordinary raw/RLE paths. Replacing
the declared-offset executable equality test with an exhaustive variant match
removed the offset guard failures; the extra model unfolding shifts the two
large functions over the solver resource limit. Runs 281–287 were serial and
went exclusively through `./verify.sh`; neither frozen spec nor hash changed.

Next proof work: split the `dib_finish` R6 mask read/validation from the R7
palette/record assembly as already done in the original M2 parser, and move
its large success-path field-by-field `hdr_matches` proof into a small named
proof function. Likewise move `dib_fields`'s large R2–R5 proof block into
per-branch lemmas, or trim redundant unfolding. Do not increase solver budget.
After both functions verify, check `mh_recovered_so_far` against the frozen
closed events: its `DefaultRGB` condition currently tests post-substitution
`m.mask_r/g/b == 0`, which misses the fallback event after replacing a zero
raw RGB mask. Also test each option against `CUSTOM-VECTORS.json` and the
universal resource/work bounds; neither was established by these runs.

---

# P2b · the `bitfield_offset` (and the `offset_effective` refactor) — stopped at the escalation budget

Status: **stopped after far more than the 5-failed-run escalation budget.** The
delivered `dib_verus/src/dib.rs` and `model.rs` are the run-387 green state
(`1658 verified, 0 errors`): the P2a baseline plus (a) the reviewer-note exposure
fix and (b) the fourteen new option enums and their `Policy` fields, every one at
its Strict (bold) value. No option is behaviourally implemented yet; `decode`
and `decode_with_policy` are byte-identical to P2a.

## The obligation

The first option of the fourteen, `bitfield_offset = MetadataEnd`: for a BITFIELDS
record the pixel cursor starts at the metadata end (`palette_end`) instead of the
declared `bfOffBits`, and any case where that differs is a `recovered = true`
event. In the model this is `offset_effective` gaining one pre-layer:

```
let start = if p.bitfield_offset == MetadataEnd && m.has_masks { m.palette_end } else { m.pixel_start };
match p.offset { Declared|AllowAny => start, Gdk => if m.compression == 0 && start < m.palette_end { m.palette_end } else { start } }
```

and the executable's `decode_policy_tracked` must compute the same `start`
(`bstart`) and pass it to `decode_raw` as `p0`.

## What was tried (runs 296–388; every run through `./verify.sh`, all in `RUNLOG.tsv`)

Every variant below either left an `assertion failed` on a record/field equality
or a per-function `rlimit exceeded` on `decode_policy_tracked`/the helper:

1. **Inline in `decode_policy_tracked`** with `hdr_matches(m, h)` projections
   (`m.has_masks == (h.compression == 3u32)` etc.). The record asserts and the
   `bstart == offset_start` assert **passed** (run 363), but the whole function
   then hit `rlimit exceeded`.
2. **Extracted `policy_pixel_starts` helpers** (runs 364–377): took `m` as a
   value (rejected: "cannot access spec-mode place in executable context"),
   then as `Ghost<MHRecord>` with `hdr_matches(gm@, h)` as a precondition. The
   helper still failed its own assertions: `assert(bstart == h.palette_end)`
   failed inside the taken `if` branch even though the branch condition is
   exactly `meta && c3` and the identical condition outside a `proof` block
   reduced fine. Reading `h.pixel_start`/`h.palette_end` through the `&Header`
   reference inside the *exec* `if` condition does not produce the same term as
   the `as int` projection used in the `assert`, and Verus did not accept the
   equality even after binding the values to `let` locals first.
3. **Enum `==` vs `match`**: the codebase never uses `==` on the option enums;
   every branch is a `match`. `h.compression == 3u32` (a `u32`) is compared with
   `==` elsewhere and works, but `p.bitfield_offset == BitfieldOffsetOptions::MetadataEnd`
   inside an exec `if` did not reduce the way the `match p.offset` branches do.
4. **Model primitive split**: `offset_start_of(meta, has_masks, ps, pe)` on
   `int` (run 344), on `usize` `offset_start_of_bool` (run 378 verified in
   isolation, 0 errors, but failed once wired into the helper), and an `of`-form
   plus an `if`-expansion lemma. A minimal standalone probe of the exact idiom
   (`fn probe(b: bool, x: usize, y: usize) -> (r: usize) ensures r == (if b { x } else { y })`)
   verified (run 378); the same idiom instantiated with the enum condition and
   `&Header` field reads did not.
5. **`lemma_offset_start_fields` / `lemma_offset_start_branches`** implication
   lemmas (runs 366, 384): the lemmas themselves are fine; the caller could not
   supply their `m.pixel_start == ps` / `m.has_masks == hm` preconditions because
   the exec-side `bstart` computation and the spec-side `m` projection would not
   be brought to a common term.

## Honest statement

The fourteen option enums and their `Policy` fields, the exposure fix
(`policy_header_recovered`… no; `Policy` fields kept `pub` for Verus opacity,
`strict_policy_exec`/`profile_policy_exec` narrowed to `pub(crate)`, `Policy`
and `MetadataOptions` dropped from `lib.rs`), and the `parse_header` rlimit
recovery all verify (run 387: 1658 verified, 0 errors; `cargo test --release`
302/302 vectors, corpus 84/58, policy Strict/Recommended 102/102, and the P2a
`cfg(test)` oracle harness green). **No option beyond the P2a six is
behaviourally implemented.** `bitfield_offset` is the one that was attempted and
it is **not** proved.

## Likely fix (for the next tier)

The `&Header`-reference reads inside exec conditions are the recurring friction.
The clean route is to compute `bstart` in a helper that takes the four plain
values (`meta: bool, has_masks: bool, ps: usize, pe: usize`) and returns a value
whose postcondition is a *model spec function over those same four plain values*
(the isolated probe of exactly this shape verified, run 378), and to have
`decode_policy_tracked` pass `h.pixel_start`/`h.palette_end` copied into `let`
locals first, with the `m.* == local as int` equalities asserted in a proof
block *before* the call. The alternative that avoids the whole class is to fold
`bitfield_offset` into the record's own `pixel_start` at `r6r7_policy`/
`r6r7_result_policy` construction time (model) and into the `Header`'s
`pixel_start` at `dib_finish` construction time (exec), leaving
`offset_effective` and `decode_policy_tracked` byte-for-byte as P2a; that keeps
every existing proof about `offset_effective`, `mp_raw_policy`, `mr_raw_out` and
the truncation lemmas untouched, at the cost of proving the record's
`pixel_start` field separately.

## Escalation

Per BRIEF-POLICY P2b. Implementer deepseek-v4.1-flash. Hand to gpt-6-sol (then
Astra 6 if Sol also stops). Nothing weaker is labelled: no option-11
`bitfield_offset` witness exists.

---

# P2b · Sol escalation: `unsupported=Rgb2` header proof

Stopped after five failed verifier runs (395–399) on the same two-bit-depth
extension. The latest WIP sources are `dib_verus/src/dib.rs.p2b-sol-rgb2-wip`
and `dib_verus/src/model.rs.p2b-sol-rgb2-wip`; the live sources are restored
byte-for-byte to the run-394 green checkpoint (`*.p2b-sol-two`, 1662 verified,
0 errors). **No new option is verified or shipped by this attempt.**

The WIP adds `Rgb2` to the policy header model's depth guard only for H>=40,
updates the executable depth check, permits two-bit indexed samples in the
pixel helper, derives the four-colour mask, and extends sub-byte shifts and
palette counts. The frozen Strict `depth_ok` and original `decode` depth check
remain unchanged. Runs 395–397 exposed concrete proof obligations in the
palette count, `header_inv` and raw conversion and those were discharged.
Trimming eager model reveals in `validate_dib_fields` and `parse_header`
removed their resource-limit failures. Run 399 reaches 1664 verified, one
error: `fn dib_finish` exceeds its per-function solver resource limit at
`src/dib.rs:2843`. No solver-budget attribute was added. These intermediate
numbers do not constitute a verified option.

The model's policy-specific R2–R5 guard introduces an additional successful
indexed depth into `dib_finish`'s already-large R6/R7 and record-equivalence
proof. The next step is to split `dib_finish`'s palette geometry from its
success-record construction, or extract the final `hdr_matches` field proof
into a named proof function, without increasing solver budget. Then check
`Rgb2` against the oracle's 2-bpp successes and compression error order,
including test-only custom records. Other eleven requested options remain
untouched. The 66-vector harness has not been extended, and neither
compatibility profile has been wired into release code.

An independent oracle calculation on the 71 BMP corpus files accepts 50 under
Strict, 51 with `unsupported=Rgb2` alone, 58 under full GdkCompat-2.42.12,
and 55 under full ImageCrateCompat-0.25.10. These are reference-model counts,
not executable acceptance measurements.
# STUCK — alloc-delta DIB step, policy half (2026-11-12)

Status: **not green; the live tree is the P2d baseline and green.** The WIP is in
`dib/alloc-wip-1/`. No `#[verifier::rlimit]` was added, and none is needed — the
failed bodies fail their postconditions, they do not need a bigger budget for a
true statement.

## Obligation that would not discharge

`decode_policy_tracked` → `decode_complete_policy` → `finish_policy_after_alloc`
and the two clipping helpers `finish_raw_zero` / `finish_rle_clipped`, i.e. the
A-DIB-2 policy half of the theorem shape `G`/`Prog` (THEOREMS-ALLOC-DELTA.md §1).
The decode half (A-DIB-1 for the four policy-free modes) is through.

The remaining failures in the best WIP run (517, 1737 verified, 14 errors) are:

1. `try_decode_with_steps`'s `OutOfMemory` postcondition: `o.elements_spec() ==
   4 * t12_checked_n(..)` is not derivable at the wrapper. Fixed in the newest
   WIP edit (state the site only; `t12_checked_n` is provably the request size
   only through `hdr_matches`); not yet re-run.
2. `finish_raw_zero` / `finish_rle_clipped`'s `policy_shape(.., false, ..)`
   postcondition at their `InvalidImageSize` / `TrailingBytes` format-error
   exits: the log is the single `Ok` entry (the reservation succeeded, and the
   error is a later phase error), so `policy_shape`'s `false` case holds with
   `all_ok`, but the exit's proof block does not yet establish it. The
   `old(log) == final(log)` assert and the `AllocSite::Output` reveal are in;
   what is missing is the `req_of_log(l) == req_dibp` step, which needs
   `policy_checked_n(b, bm, p) == h.spec_n()` (from `hdr_matches` and
   `mh_partial is None`).
3. `finish_policy_after_alloc`'s precondition
   `alpha == mh_policy_rle_or_32bpp(bytes@, bm, policy)` and its `steps`
   arithmetic. The `alpha` tie is what P2d solved with the `alpha: bool`
   threading; the WIP still states the model predicate in this helper's
   `requires`, which is the P2d rlimit trap.

## The one substantive hazard

Adding `#[verifier::when_used_as_spec]` to `Error::name` (needed so
`TryError::name`'s postcondition can name the `Error` spelling) appears to have
disturbed the crate-wide encoding: `fn raw_sample_scaled`, whose body is
byte-identical to the baseline (verified by `diff`), began reporting
`function body check: resource limit (rlimit) exceeded` from the run where the
attribute first appeared (run 504 onward), and only there. The baseline runs it
in the same configuration with no such error. Next agent: try removing the
attribute and writing `Error::name`'s postcondition from `sites.rs` with the
spelling table inlined, or give the two `name` functions no spec clause at all
(the tests call `name()` in exec position only).

## What to do next

See `NOTES.md`, "alloc-delta (fallible allocation) — FIRST SLICE", for the
ordered continuation steps. The short version: copy `alloc-wip-1/*` back over
the live sources and `alloc-wip-1/alloc_guard/` to `dib_verus/src/alloc_guard/`,
`./verify.sh`, and work the list above; the resource-limit note in `NOTES.md`
P2d is binding for the policy half.

Not attempted at all: A-DIB-4..A-DIB-8 as separate obligations beyond what
`decode_tracked`'s `ensures` already states, the test migration to `try_*`, the
fault-injection tests, and the `tools/audit_alloc.sh` extension.
