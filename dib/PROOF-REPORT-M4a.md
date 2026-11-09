# PROOF-REPORT-M4a — the total models MR (raw), MR (RLE), MA (ICO AND), MP; T6, T7, T8

**Verifier:** Verus 0.2026.11.02.8bf68ee (binary `/opt/verus`), profile `release`,
toolchain 1.99.1-x86_64-unknown-linux-gnu.

**Final result line after `cargo clean`:** `verification results:: 336 verified,
0 errors` (`runs/403.log`, 88 s; the pre-clean green run is `runs/402.log`).

**Conformance:** `cargo test --test conformance -- --nocapture` →
`conformance: 214/214 vectors passed`, `test result: ok. 1 passed; 0 failed`.
`cargo test --test corpus -- --nocapture` → `corpus: 84 files checked, 58
accepted`. Behaviour is byte-identical to M3.

**`./verify.sh` runs this milestone:** 289–403 (115 runs, 12 exited 0). Runs
382–397 were the unfinished T7 command-loop construction; 398–403 the revert to
the last green state and the final clean reverify. No obligation hit the
five-failure stop rule inside any single attempt before the budget ran out;
`STUCK.md` records the T7/T8 remainder.

**Tool calls:** the harness count is authoritative; by my own count this exceeded
the 400 cap (the T7 loop alone consumed ~45 runs/edits).

**Frozen files:** `spec-frozen/SPEC.md` (`94794534…`), `VECTORS.json`
(`c284c4ff…`), `SOURCES.md`, `DESIGN-NOTES.md` — `sha256sum` matches
`FREEZE.txt`, unchanged.

## What is proved

### T6 · exact raw — proved

`decode_raw` is a thin wrapper over `raw_loop`, whose postcondition is the
manifest's claim at its own quantification: an **arbitrary reachable start
state** `(pixel_start, 0, 0)` (`raw_reachable`), **every finite stream**, and on
**both** exits

- the model's exact first error (or success cursor/position): `r is Err(e) ==>
  raw_gcur(b, hrec, pos0, 0, 0) == Err((RawErr::Api(e), gcur.0, gcur.1, gcur.2))`
  and `r is Ok ==> raw_gcur(...) == Ok((gcur.0, 0, height))` — the ghost final
  cursor/position is a *clause of the helper's contract*, not a separate exit
  lemma; and
- `final(out)@ == bytes_replay(zero fill, raw_writes(b, hrec, pos0, 0, 0))` — the
  concrete pixels written so far **are** the model's partial output through the
  first failing check, via the ghost write accumulator `gw` that tracks each
  concrete `put`.

`raw_full`/`raw_gcur`/`raw_writes` are written from SPEC R8/R9/R14 alone;
`lemma_raw_full_proj` ties `raw_full` to its two projections, and
`lemma_raw_gcur_ok_shape` proves the success shape. No `MR`-projection hypothesis
is left undischarged.

### T7 · run-painter half — proved

`paint_run` (the per-run/absolute pixel painter) carries the same ghost
accumulator and, on `PaletteIndexOutOfRange`, ties the concrete partial output
to the model's `run_good` prefix (`final(out)@ == bytes_replay(fill, gw +
run_writes(0..run_good))`), and asserts `run_ok` on success.

### The models (independently written)

`model.rs` gained, from the frozen text alone and never calling `crate::dib`:

- the byte-level write semantics `Write`, `replay_bytes_at`/`bytes_replay` (the
  exact bytes the executable's `put` writes), `zero_bytes`/`pal_zero_bytes`,
  `cell_of`, `Img`/`well_formed`;
- **MR (raw)**: `raw_full`/`raw_gcur`/`raw_writes` (R9/R8 order, with
  `TruncatedPixels` at the row start and `PaletteIndexOutOfRange` at the
  row-start cursor);
- **MR (RLE state machine)**: `rle_reachable`, `run_index`/`abs_index`/
  `run_good`/`abs_good`/`run_writes`/`abs_writes`, and `mr_full`/`mr_cur`/
  `mr_writes` (R10/R11 order: command presence, run bounds, EOL, delta presence
  and bounds, absolute bounds, whole padded payload, then each index), with
  `lemma_mr_full_proj`;
- **MA (ICO AND)**: `and_stride`/`and_bytes`/`image_size_ok`/`and_bit`/
  `and_writes`/`ma_mask` (R12: size check before mask availability, then the
  whole-image alpha policy, AND bit one vetoes);
- **MP**: `mp_raw`/`mp_rle`/`mp_mask_phase`/`mp_decoded`/`mp`/`accepted`;
- the algebra: `lemma_bytes_replay_{push,len,assoc}`,
  `lemma_run_writes_{unfold,empty,single,snoc,snoc_at}`,
  `lemma_abs_writes_{unfold,empty,single,snoc_at,full}`,
  `lemma_run_good_{unfold,bound,all,g_all,prefix,from,all_ok,is_run_ok,index_eq}`,
  `lemma_abs_good_{unfold,bound,all,g_all,prefix,from,all_ok,is_abs_ok}`,
  `lemma_quad_at_index`, `lemma_pal_zero_bytes_{index,len}`, `lemma_pal_zero_put`,
  `lemma_raw_{full_proj,gcur_ok_shape,writes_step}`.

## What is NOT proved (STUCK.md)

- **T7's command loop** (`decode_rle`'s `rle_loop`) — the model and all its
  standalone lemmas are defined and proved, `paint_run` is proved, but the loop's
  postcondition (connecting the running ghost `gw` back to `mr_writes(pos0,0,0)`
  at every exit, plus the EOB success shape) did not close before the budget ran
  out. The missing pieces are the model's `mr_writes` recursive-step lemmas.
- **T8** — `MA` is fully defined in the model; `apply_and`/`finish_mask` were not
  given the matching exact contract, so no T8 obligation is stated on the
  executable.
- T9, T10, T12, T13, L1–L3 and the vector/mutation campaign are M4b/M4c, as
  scheduled. Nothing weaker carries a T-name: `t6_*` is not a stub.

## Assurance boundary (no additional trust)

`grep -rnE "admit\(|assume\(|assume_specification|external_body|trusted|unsafe|
todo!|unimplemented!|verifier::external|requires\(false\)|rlimit" dib_verus/src/*.rs`
(excluding the preserved `*.m1`, `*.m2*`, `*.m3`, `*.m4a` copies) finds nothing.
No solver-budget attribute. `target/` removed after the final run.

## Surprises and tool obstacles

1. **`old(out)` inside a loop body is iteration-relative**, so a loop's
   postcondition cannot name the function-entry buffer. The fix (as in TGA) is a
   ghost local initialised from `old(out)` before the loop plus a byte-level
   frame condition on `put`.
2. **In-loop `proof` blocks may not contain `let` bindings of exec values**
   (`let byte = rd(b, p)` is a mode error); the exec computation must be hoisted
   out of the proof block, and `by (bit_vector)` cannot reference `Seq` indices
   (bind the byte to a local first).
3. **`raw_writes`' success shape is not `x == width` but `(0, height)`** — the
   `x >= width` branch re-enters at `x = 0`; `lemma_raw_gcur_ok_shape` had to be
   stated with the invariant `y >= height ==> x == 0`.
4. **`m.n` (the model's `int`) is concrete inside loops, so plain
   `by (nonlinear_arith)` on it fails**; hypotheses must be restated over the
   executable's `h.n as int`.

## Deliverables

- `dib_verus/src/model.rs` (4,134 lines) — the M3 model plus the M4a models and
  their lemmas.
- `dib_verus/src/dib.rs` (3,783 lines) — `put`'s frame condition, `alloc_output`'s
  zero-buffer postcondition, `raw_loop` (T6), `paint_run` (T7 part), and the
  M4a helpers.
- `dib_verus/src/{dib,model}.rs.m3` and `.m4a` — the previous and current files.
- `STUCK.md` (extended with the M4a T7/T8 remainder), updated `NOTES.md`.
