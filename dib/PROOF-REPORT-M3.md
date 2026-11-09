# PROOF-REPORT-M3 — T4 (conversion), T5 (orientation and row arithmetic), T11 (termination); the M2b `read_at` limit closed

**Verifier:** Verus 0.2026.11.02.8bf68ee (binary `/opt/verus`), profile `release`,
toolchain 1.99.1-x86_64-unknown-linux-gnu, vstd `0.0.0-2026-11-01-0251`.

**Final result line after `cargo clean`:** `verification results:: 264 verified,
0 errors` (`runs/287.log`, 87 s; the earlier identical run after a first
`cargo clean` is `runs/286.log`, 84 s).

**Conformance:** `cargo test --test conformance -- --nocapture` →
`conformance: 214/214 vectors passed`, `test result: ok. 1 passed; 0 failed`.
`cargo test --test corpus -- --nocapture` → `corpus: 84 files checked, 58
accepted`, `test result: ok. 1 passed; 0 failed`. `cargo build --release` clean.

**`./verify.sh` runs this milestone:** 52, numbered 236–287 in `RUNLOG.tsv`
(287 total runs on the project); 13 exited 0. Runs 236–248 were the `read_at`
closure, 249–266 the model lemmas and the `Conv` connection, 267–285 the T4/T5/T11
witnesses, 286–287 the post-`cargo clean` full reverifies. No obligation reached
the five-failure stop rule.

**Tool calls:** approximately 155 (52 of them `./verify.sh` runs, the rest file
reads, edits and test/build commands). The harness count is authoritative. Under
the 400 cap.

## The M2b limit is closed

M2b left `read_at(b, p)` a guarded total reader that returned `0` for an
out-of-slice `p`, so M2 never proved that a computed raw/AND/palette sample
address lies inside its checked row, mask or palette. M3 removes `read_at`
entirely (`grep -n read_at src/dib.rs` finds only a comment) and replaces every
call site with the precondition-carrying `rd(b, p)` (`requires p < b@.len()`), so
the byte read is a verified in-slice access on every path:

- **Raw samples.** `raw_sample` now takes the row stride `line` and
  `pos + line <= b@.len()`, and computes the sample's byte offset from
  `x` and `depth`; `model::lemma_subbyte_within_stride` (sub-byte depths) and
  `row_last_sample_within_stride` (`depth % 8 == 0`) prove the offset plus the
  whole sample lies within the stride, hence within the checked row. The 16/32-bit
  word reads use the model-proved `rd16`/`rd32` (`LE16`/`LE32`).
- **Palette reads.** `colour`'s call site proves the borrowed palette ends inside
  `b` (`header_inv`), and `model::palette_stride_bounds` gives the indexed entry's
  last byte is inside the slice; the B/G/R reads use the precondition `rd`.
- **AND mask.** `apply_and` carries `row + mask_stride <= b@.len()` per row and
  `model::and_bit_offset_bound` (a new named nonlinear-arithmetic lemma) proves the
  MSB-first bit byte `floor(x/8)` is one below the padded 1-bpp row stride, so the
  mask read is in range. The row bound `row + mask_stride <= pos + and_bytes <=
  b@.len()` is carried as a loop invariant.
- **RLE bytes.** The command bytes, the delta bytes and the absolute-packet bytes
  all read through `rd` under proved `pos + 2 <= len` / `pos + padded <= len`
  guards; the payload index `j/2` is bounded by `payload - 1` (with
  `payload == (q+1)/2`) and the padded payload covers it.

Behaviour is byte-identical: 214/214 conformance and 84-file corpus (58 accepted)
unchanged, and a 1500-case differential fuzz against the frozen oracle
(`/home/ws/projects/dib/spec/check_vectors.py`) — random bytes and mutated BMP
skeletons in all three modes — agreed 1500/1500.

## T4 · conversion — proved

The independent model gained `Conv`: `pal_entry` (R at 0, G at 1, B at 2, alpha
255, reserved byte ignored), `rgb_byte`/`rgb_seq` (24/32-bit byte order),
`rgb555_seq` (BI_RGB 16 rounded 5:5:5, bit 15 ignored), `bitfield_seq` (R6 masks),
`conv_word`, `conv`, and the byte-level `conv_rgb`/`raw_conv` that mirror the
executable's own reads (`sample_word`, `sub_index`, `sub_shift`, `sub_mask;
`sample_word_len`). These are transcriptions of R6/R8 and never call `crate::dib`.

Connection to the executable:

- `channel` postcondition: `r = channel_of(word, mask)` (M2), unchanged.
- `colour` postcondition strengthened to `c@ == pal_entry(b@, palette_start,
  pal_stride, ix)`.
- `raw_sample` postcondition: on `Ok(c)`, `raw_conv(b@, model_record, pos, x) ==
  c@`, proved in every branch (sub-byte index and shift, BITFIELDS 16/32, BI_RGB
  16/24/32 with ICO alpha split).
- `model::lemma_conv_rgb_eq_conv_word` proves `conv_rgb = conv_word ∘
  sample_word` (with the byte-extraction lemmas `lemma_rgb_byte_three`,
  `lemma_le32_top_byte`); `model::lemma_raw_conv_eq_conv` composes it to the
  manifest's `Conv`. Thus for every reachable `MH` record and every guarded sample
  the concrete conversion equals `Conv` byte for byte: BGR order, palette stride
  3/4, 555 rounding, 16/32-bitfield scaling with absent alpha 255, BI_RGB32 alpha
  split by mode and 32-bit ICO XOR alpha preservation. The witness
  `t4_conversion` states the palette-read containment at the manifest's
  quantification (`forall ix < count`).

## T5 · orientation and row arithmetic — proved

Named manifest lemmas, all in `src/model.rs`, proved for valid `w,h,depth`:

- `stride_roundup_safe` — `stride = 4*ceil(w*depth/32)`, `0 <= stride <= 32,768`,
  `stride % 4 == 0`.
- `row_last_sample_within_stride` — the last raw sample byte stays inside the
  stride (`stride*8 >= w*depth`).
- `product_width_height_bound` — `1 <= w*h <= 16,777,216` and `4*(w*h) <=
  67,108,864`.
- `pixel_byte_offset_bound` — `0 <= orient_cell(...) < N` and `4*cell + 4 <= 4N`.
- `vertical_flip_bijection` — the flip is injective, is its own inverse, and the
  composed cell map over `(x, r)` is a bijection `[0, N) -> [0, N)` (injectivity
  plus an explicit `exists` witness for surjectivity), so distinct `(x, r)` map to
  distinct pixel cells.
- `and_bit_offset_bound` — the 1-bpp AND stride and `floor(x/8)` MSB-first offset
  stay inside the checked mask.
- `bit_mask_contiguous_extract` and `scale_round_bounds` — the mask's `max` is
  nonzero with low bit set and `component <= max`; `scale_of(c, max) <= 255` and
  `255*c + max/2` is nonnegative.
- `palette_stride_bounds` — stride 3 or 4, and an indexed entry `ix < count` ends
  inside the borrowed slice.

`out_offset`'s postcondition now also gives `r = 4*orient_cell(...)`, so the
orientation offset and the model's cell map agree. The witness `t5_rows` states
the stride, last-sample, per-write `4N` bound, bijection, distinctness and AND-bit
clauses on a concrete accepted header.

## T11 · termination — proved

The manifest asks for explicit decreasing variants for every loop, including the
RLE command/absolute-packet parsers, plus a model-level argument.

- **Raw loop:** outer `decreases height - y`, inner `decreases width - x`; each
  row consumes exactly one stride, so on success the cursor is `pixel_start +
  height*stride`.
- **RLE command loop:** `decreases b.len() - pos`. The model defines
  `rle_cmd_len` (EOB and EOL consume 2, delta 4, a run 2, an absolute packet
  `2 + padded`; `0` means incomplete) and `rle_cmd_count`. The named lemma
  `lemma_rle_cmd_progress` proves every *complete* command consumes at least two
  bytes and stays inside the slice, so the cursor strictly increases and the
  measure strictly decreases; `lemma_rle_cmd_count_bound` proves
  `p + 2*count <= b.len()` and `count <= (b.len()-p)/2`.
  `lemma_rle_cmd_no_retry` states an incomplete command is rejected before any
  payload byte is read, so no zero-progress packet is retried.
- **Nested `<= 255` painters/parsers:** `decreases quantity - j` (repetition) and
  `decreases q - j` (absolute); `lemma_rle_paint_terminates` proves `q - j`
  strictly decreases with `0 <= j <= q <= 255`.
- `lemma_rle_monotone_position` and `rle_painted_at_most_N` record the position and
  painted-count invariants (`x <= w`, `y <= h`, `y` nondecreasing, `painted <= N`).
- Witnesses: `t11_termination` (the nested variant), `t11_rle_cmd_model` (the
  model-level `|b| - cursor`), and `t11_decode_terminates`, which *runs* the
  concrete `decode_raw`/`decode_rle` helpers on a validated record and returns
  their cursor — the real decoders' own `decreases` clauses, not a restatement.

## What is NOT proved (deferred, with the reason)

- **T6, T7, T8** (exact raw/RLE/AND transitions including ghost cursor/error
  state) are M4a, as the brief schedules. `raw_sample` proves the per-sample
  conversion (T4) but the raw *loop's* full `MP`-level cursor/ghost equality is
  M4a.
- **T9, T10, T12, T13** (integrated `decode = MP`, error precedence, linear work,
  resource bounds) and the vector/mutation campaign are M4b/M4c.
- **T2 as stated of `decode`** is carried by `parse_header`'s postcondition on the
  actual first call (M2b); the full `decode = MP` is M4b.
- A 32-bit verifier run: only x86_64 is installed. The 32-bit argument is by hand
  (the bounds `4N <= 67,108,864 <= 2^26`, `stride <= 32,768`, `input <=
  67,108,864` fit a 32-bit `usize`), not verifier-checked.
- No theorem is stated under a T-name in a weakened form: `t4_conversion`,
  `t4_raw_sample_eq`, `t5_rows`, `t11_*` carry the manifest's quantification and
  are connected to the public helpers. Nothing is proved only for vector-shaped
  inputs.

## Assurance boundary (no additional trust)

`grep -rnE "admit\(|assume\(|assume_specification|external_body|trusted|unsafe|
todo!|unimplemented!|verifier::external|requires\(false\)|rlimit"
dib_verus/src/*.rs` (excluding the preserved `*.m1`, `*.m2a`, `*.m2b` copies)
finds nothing. No `#[verifier::rlimit]` or other solver-budget attribute is
present. The only trusted base is the verifier, its `vstd` contracts and the
compiler's execution of verified code. Frozen `spec-frozen/` hashes unchanged
(`SPEC.md 94794534…`, `VECTORS.json c284c4ff…`, matching `FREEZE.txt`).

## Surprises and tool obstacles

1. **`b.len()` does not unify with `b@.len()` in `by (nonlinear_arith)`
   hypotheses.** Every bound on a slice read had to be restated over `b@.len()`
   (or the concrete index first cast to `int`) before the arithmetic solver would
   fire. This is the single most repeated obstacle.
2. **`by (bit_vector)` rejects calls in its hypotheses.** Trailing-zero reasoning
   had to bind `shift` to a `let` before the bit-vector block;
   `u32_trailing_zeros(mask)` cannot appear inside one.
3. **A nested `if` in a spec function is opaque to `by (nonlinear_arith)`.**
   `padded = if payload % 2 == 0 { payload } else { payload + 1 }` did not give
   `payload <= padded`; rewriting it as a total `payload + payload % 2` (byte-
   identical, one extra `%`) made the fact flow.
4. **One extra postcondition clause on `raw_sample` exceeded the per-function
   resource limit** (the Binding limit from M2). The `Conv` equality is instead
   delivered by the witness `t4_raw_sample_eq`, which calls the helper and applies
   `model::lemma_raw_conv_eq_conv` — no budget attribute, no weaker statement.
5. **`let` bindings in spec `ensures` need a `{ }` block**, and a bare
   `x as int < y` still needs the parentheses (`(x as int) < y`).

## Deliverables

- `dib_verus/src/model.rs` (~2,330 lines) — `M2`'s model plus `Conv`, the
  orientation cell map, the eleven named manifest lemmas, the T11 RLE
  command-parser termination, and the `read_at`-closing arithmetic lemmas.
- `dib_verus/src/dib.rs` (~3,140 lines) — `read_at` removed; `raw_sample`,
  `colour`, `out_offset`, `apply_and`, `decode_rle` carry the containment
  preconditions/postconditions; the `t4_*`, `t5_rows`, `t11_*` witnesses.
- `dib_verus/src/dib.rs.m2b`, `dib_verus/src/model.rs.m2b` — the M2b files,
  preserved per the dispatch.
- `PROOF-REPORT-M3.md`, updated `NOTES.md`.
