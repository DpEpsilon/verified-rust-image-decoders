# M4: resolved

**Status: complete.** Run 300 gives 123 verified, 0 errors with default solver
limits and `--no-cheating`; `cargo test` passes 4 tests. T4, T5, T7 and T8 are
proved for Strict's `lzw_tail`/`index_range`, with the T12 per-frame charges and
the T13/T4 pixel clause. No open obligation remains in this crate's scope.

## The one statement that had to change (not weakened: corrected)

`lemma_ir_eq_iv_any` was drafted to say `ir(r, h) == iv(r, h)` for **every**
integer `r`. It is false. On `r >= h` the model's `IR` (`ir_at`) walks the four
passes and, in the fourth, takes `pass_start(3) + r·pass_step(3)` only for the
rows pass 3 owns; the executable's `iv_at` assigns pass 3 `1 + r·2` for any
residual. `IR`'s fourth pass holds only its own rows, by G11's definition. On the
reachable domain `0 <= r < h` the two agree — that is `lemma_ir_eq_iv`, proved at
run 34 and unchanged — and the equality is only ever used there.

The replacement states what the model actually needs: `lemma_iv_total` for
`0 <= r` (used to prove `ir_at` non-negative on the drawing domain), and
`lemma_ir_at_nonneg_step`, an explicit four-case induction on the pass index. The
Clip clauses were then restated on the drawing loop's domain
`0 <= q < w·h`, which is where the executable's positions live.

## History

- Runs 4–6: Verus parses `EXPR as int < EXPR` as a cast of `EXPR as int < ...`;
  every comparison with a cast needs the cast parenthesized or named first
  (`let ghost qv: int = q as int;`). This bit the first `interlace_row`,
  `draw_index` and the count lemmas.
- Runs 39–46: the interlace bijection. The route that worked is the explicit
  **inverse** `back(y, h)` = the pass's prefix + the row's index inside the pass,
  with the class of `y` (multiples of 8, `≡ 4 mod 8`, `≡ 2 mod 4`, odd) choosing
  the pass. `back(iv(r)) == r` gives in-range, injectivity and surjectivity at
  once, and avoids any modular arithmetic on the forward direction beyond the
  four pass-disjointness facts.
- Runs 69–162: `clear_rect`/`copy_rect`. The recursive row fold (`clear_rows` over
  rows, `clear_cols` within a row) is what the executable's two loops do, but its
  per-position effect at the row-loop head is awkward to state. The definition was
  switched to the fold form *and* the loop invariant made the fold-shaped
  statement; the pointwise `Seq::new` form was tried first and abandoned, because
  the inner loop's step then needs a per-byte slab argument. The fold form proved
  in three runs.
- Runs 101–108: the `Seq::new` per-byte model was kept for `clear_rect`/`copy_rect`
  after all, because the **empty-clip identity** (Q18) is one `forall` over bytes
  rather than a two-level recursion. Both forms exist in the model: the folds for
  the loops, the `Seq::new` forms for the empty case. They are linked by the
  loops' invariants at `x = x0`.
- Runs 163–197: `MC` was factored into `mc_disposed`, `mc_saved` and `mc_from` so
  `composite` can take its three steps as three calls; `draw_run`/`draw_run_at`
  (canvas through the first error) was added because the executable's drawing loop
  needs the canvas **at** the error, which `draw_fold` (error only) does not give.
- Runs 198–251: the frame sequence and the Clip clauses. `mc_frames_from` carries
  `(canvas, Saved, Option<ComposeError>)` and stops after the first error, which
  makes "the last frame's disposal is never applied" and "an error leaves both
  buffers alone" direct.
- Runs 252–266: plain `rustc` builds `tests/` without `vstd`'s proof functions, so
  every `proof fn` import is `#[cfg(verus_only)]`. `cargo verus verify` sets that
  cfg; `cargo test` does not.
- Run 267: `rlimit` on `lemma_ir_eq_iv_any` — the statement was false (above).
- Runs 268–300: the T12 work statement. `lemma_disposal_bytes_bound` needed the
  reachable domain's `left <= sw`, `top <= sh`; without them the clip's spans are
  not provably non-negative.

## Nothing unproved in scope

`assert`-only "lemmas" left in the file: none. Every `proof fn` has a real body.
No `assume`, `admit`, `external_body`, `unsafe`, `rlimit` attribute, new axiom or
trusted FFI. The deferred interfaces (the parser's frame header, the allocator's
reservations, the policy forms) are named in `README.md` and `LEDGER.md`'s scope
section; they belong to M5, not to this milestone's T4/T5/T7/T8 rows.
