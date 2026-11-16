# compose_proof: the G11 interlace and G12 compositor (milestone M4)

Standalone Verus crate proving `THEOREMS.md`'s **T4, T5, T7 and T8** (with the
A-GIF-3/5 `Saved` reservation shape): the interlace `IR` and its executability,
the drawing, the disposals 0–3 with the `Saved` buffer, clipping under
`frame_bounds = Clip`, the output pixel clause, and the frame sequence
`canvas_k = MC(canvas_{k-1}, …)`, all against `SPEC.md` G11/G12 and the models
`IR`/`MC`.

It is standalone so it does not touch the parser proof in `gif/gif_verus/` or the
finished LZW crate in `gif/lzw_proof/`; the module moves into `gif_verus` at M5
with its own model intact.

| file | responsibility |
|---|---|
| `src/limits.rs` | G1's caps, by name only (`MAX_INPUT`, `MAX_DIMENSION`, `MAX_OUTPUT`) |
| `src/model.rs` | `IR` (G11) and `MC` (G12) as total models: `ir`/`iv`, `clip`, `target_cell`, `draw_step`/`draw_fold`/`draw_run`/`draw_all`, `clear_rect`, `copy_rect`, `dispose_prev`, `mc`, the pixel clause, and the `MC` frame fold |
| `src/interlace.rs` | the executable `interlace_row`, its equality with `IR`, the bijection (T5), the T5 offset/address lemmas and the `interlace_pass_sizes` arithmetic |
| `src/compose.rs` | `draw_index` (the per-index sink), `clear_rect`/`copy_rect`, `dispose`, `save_rect`, `draw_all`, `composite`/`next_frame`, the output pixel clause, the Clip clauses and the T12 work statement |
| `tests/compositor.rs` | the compositor against `gif_oracle.py` on the 30 M4 vector records, the per-index sink against `composite`, `interlace_row` against G11, and the Q18 regression |

The caps are the named constants of `src/limits.rs`: `MAX_INPUT` 256 MiB (2^28); `MAX_OUTPUT` stays at 64 MiB (2^26; `W·H ≤ 16,777,216` pixels) because it must not exceed the shared alloc guard's `MAX_BYTES` (67,108,864), which is copied byte-identically into every core. The owner expects to raise the canvas cap and `MAX_BYTES` together later, as a strict widening through an alloc-delta erratum with re-verification of every core.

From the repository root:

```sh
timeout 25m bash gif/compose_proof/verify.sh "what I am proving"
```

The script records every verifier run in `RUNLOG.tsv`, uses two verifier threads
and enables `--no-cheating`. Do not run two verifiers at once. From this crate:

```sh
PATH=/opt/cargo/bin:/opt/verus:$PATH timeout 25m cargo test
python3 -I tools/gen_compositor_cases.py     # regenerate tests/expected/compositor.json
```

## The interface the LZW core feeds (for M5)

`draw_index(canvas, state, frame, table, sw, sh, idx)` is called **once per
decoded index, in raster order**, with `state.q` running 0, 1, 2, … and
`state.q < w·h` as the caller's invariant (the LZW core emits indices only while
`q < A`). It is closure-free and sink-free: the caller keeps the state, so the
LZW loop can call it straight from its own raster loop. `composite`/`next_frame`
takes the frame's whole index sequence and runs the three G12.2 steps (disposal,
save, draw) in order.

`draw_index` returns `ComposeError::ColorIndexOutOfRange` for a non-transparent
index `>= C`; the transparent index is compared first, so a transparent index
`>= C` writes nothing. On an error the canvas is unchanged (G12.4).

## What is proved

- `interlace_row` equals the model's `IR` on `1 <= h <= MAX_DIMENSION`,
  `0 <= r < h`, and `IR` is a bijection on `[0, h)` (`lemma_ir_bijective`), with
  the pass counts as `interlace_pass_sizes` (`lemma_nk_pass_len`) and the
  inverse `back` for the surjectivity and injectivity legs.
- Every pixel of a frame inside the screen has its four bytes **strictly** inside
  the canvas (`lemma_frame_offset_strict`), distinct stream positions give
  distinct cells (`lemma_frame_address_injective`), and all coordinate sums are
  computed without narrowing (`lemma_rect_sum_unnarrowed`).
- Drawing one index equals `draw_step`; drawing a whole frame equals
  `draw_fold`/`draw_run`/`draw_all` (`draw_index`, `draw_all`); `composite`
  equals `MC` as a whole `Seq` for the canvas and for `Saved`, with the first
  error and no partial frame.
- Disposal 0 and 1 leave the canvas, 2 clears the clipped rectangle to
  `(0,0,0,0)`, 3 restores the `Saved` bytes at the same offsets
  (`clear_rect`, `copy_rect`, `dispose`); the last frame's disposal is never
  applied (the caller's sequence stops at it), and an empty clipped rectangle
  writes nothing (Q18).
- Under `Clip` a position is written iff its unnarrowed coordinates are inside
  the screen (`lemma_clip_skips_outside`), and a rectangle entirely beyond the
  screen clips every position away (`lemma_clip_all_outside`).
- The output pixel clause: every alpha-255 pixel carries a colour-table entry's
  RGB and every other pixel is `(0,0,0,0)`, preserved by each `MC` step
  (`lemma_draw_run_pixels`, `lemma_mc_pixels`).
- T12's step accounting for the compositor's own loops: the drawing loop runs
  `w·h` iterations, inside `C_CALL·(Δ + A)` because `Δ + A >= 13` and
  `C_CALL = 32` (`lemma_draw_steps_bound`); disposal and restore touch at most
  the clipped rectangle, within the previous frame's `w·h`
  (`lemma_disposal_bytes_bound`, D6); the `Saved` fill is the one `4·N` charge of
  the reserving call (`lemma_saved_fill_cost`, D6).
- Termination and no panics: every loop has an explicit variant (`y1 - y`, `x1 - x`,
  `idx.len() - q`, `idx.len() - k`), the interlace walk decreases on its pass
  index, `draw_all` on `|idx| - q`, and every index is proved in bounds before it
  is used. No recursion, no `unsafe`, no `assume`/`admit`/`external_body`, no
  `rlimit` attribute, no new axiom.

## Standalone scope (deferred to M5)

Not proved here, and named as such in every report:

- The raw-byte table accessors and the parser: `MB`'s frame header record, the
  colour table's borrowed range, and the `C` and rectangle a frame's descriptor
  gives. `compose_proof` takes a `Frame` record whose `frame_ok` facts (`1 <= w,
  h`, `C <= 256`, disposal `0..=3`, the coordinate sums inside `2·MAX_DIMENSION`)
  are the reachable domain's, supplied by the caller.
- The `LzwTable` and `Canvas` reservations and the `alloc_guard` log (A-GIF-1–6);
  only `Saved`'s `4·N` charge is stated here, since it is this milestone's shape.
- SUBTLE: the M4b canvas is passed as `&mut [u8]` and `Saved` as a second slice.
  The `Saved` reservation belongs to `Frames::next_frame` (G18); this crate
  receives both and proves their effects.
- The `StopAtRaster` and `index_range` policy values: the theorems here are for
  Strict's `lzw_tail` and `index_range = Reject`, so an emitted index is either
  transparent or below `C` (G10.2/G12.2). The policy forms are P1's.

## Trusted, and named

vstd's specifications; Verus's soundness; rustc's faithful compilation. The
Python oracle and `tools/gen_compositor_cases.py` are evidence for the vectors,
not part of any proof.
