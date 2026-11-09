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
