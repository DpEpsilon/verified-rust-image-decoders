# GIF proof phase: work breakdown

Conclusion first: one crate (`gif_verus`), `no_std + alloc` and `try_*`-only from the first line, proved in six core milestones (M0–M5) and two policy milestones (P1–P2). Estimated size, calibrated on the earlier cores: 1,600–2,400 verified obligations for the core plus Strict/Recommended, and 40–60% more for the two compatibility profiles; 1,200–2,000 `verify.sh` runs; 14–18 dispatches. The three obstacles most likely to cost runs are the LZW table representation, the bit reader over sub-blocks and the multi-frame iterator invariant.

Calibration: DIB reached 1,557 obligations over 1,560 logged runs (core 1.0, the 1.1 mode, policy P1–P2 and allocation work); TGA 947 obligations over 720 runs; ICNS 315 over 258; ICO 303 over 174. GIF's header and block parser are about TGA's size; its LZW decoder is harder than any RLE so far (a dictionary with a representation invariant); the compositor and the stateful iterator have no precedent in these cores.

## Structure

- `gif_verus/src/model.rs`: the frozen model, transcribed from `spec-draft/SPEC.md` independently of the executable: `MH` (G2–G4), `MB` (the block walk G5–G9 without pixels, returning the per-frame records and the concatenated image data `D` as a ghost `Seq<u8>`), `ML(m, D, A)` (G10, the index sequence or first error), `IR(r, h)` (G11), `MC(canvas, frame, gce, prev)` (G12), and `MS(b)`/`MF(b)` (G14/G15). Policy-indexed versions take the record (P1).
- `gif_verus/src/lzw.rs`, `parse.rs`, `compose.rs`, `gif.rs` (entry points and iterator), `alloc_guard/{mod.rs,sites.rs}` (the hash-pinned guard of `../alloc-delta/SPEC.md` §3, sites `Canvas`, `LzwTable`, `Saved`).
- Tests: `tests/conformance.rs` over `spec-draft/VECTORS.json` (all 312), the policy vectors once generated, and the probe inputs.

The guard and `no_std` are adopted from the start (alloc-delta §1.2 removes non-`try` names; retrofitting cost DIB and TGA separate campaigns).

## Milestones

**M0 — freeze.** Owner answers `policy-draft/QUESTIONS.md` Q1–Q10. Write `THEOREMS.md` in the shape of `../dib/THEOREMS.md` (T1–T13 restated for GIF, plus T-LZW and T-ITER below), have it reviewed by an independent model (as `REVIEW-SOL-*.md`), then freeze `spec-draft/` → `spec-frozen/` with `FREEZE.txt` hashes. Run a real-corpus pass first (Q9) so the freeze does not wait on a later surprise.

**M1 — plain implementation.** Ordinary Rust outside `verus!`, with the guard already in place. Done when all 312 vectors pass (the 65,536-frame records and the 64 MiB padding records as runtime tests), every probe input matches `gif_oracle.py` on accept/reject and pixels, and the leniency harness gains a `verified` GIF runner agreeing with Strict.

**M2 — LZW core (T-LZW).** For all `m ∈ [2,8]`, all `D`, all `A ≥ 1`: the executable decoder emits exactly `ML(m, D, A)`'s index sequence or its first error, with the bit position at the error; table indices stay below 4096; the string stack never overflows; termination by the variant `8·|D| − pos`. Proof plan: the table invariant `∀c ∈ [clear+2, next): prefix[c] < c ∧ len[c] = len[prefix[c]] + 1 ≤ 4096 ∧ first[c] = first[prefix[c]]`, plus the representation lemma `chain(c) = model_table[c]` (the model holds strings; the executable holds prefix links and writes strings backwards into the stack); the width lemma `next < 2^width ∨ width = 12`; the KwKwK case where the new entry's string is needed before it is materialised. Witness gate: about 12 small streams (Clear, EOI, KwKwK, width change at 7→8 entries for m=2, table full, deferred clear, each error).

**M3 — header and block parser (T1–T3, T10 prefix).** `MH` and `MB` equality with exact first error for every input: header, both colour tables, extensions (GCE size and terminator, duplicates, undefined disposal, Application and Plain Text sizes, unknown labels and GCE scope), descriptors and caps, the two-phase image-data scan. The bit reader over sub-blocks: prove that reading `width` bits at reader state `s` (sub-block index, offset, bit buffer) equals reading `D` at model position `pos(s)`, where `D` is the ghost concatenation built by the structural scan. Allocation: `Canvas` and `LzwTable` reserved exactly when `MH` succeeds (A-…-1/2 shape: G, Prog, the `PreAlloc` corollary).

**M4 — interlace and compositor.** M4a: `interlace_row` is a bijection on `[0, h)` for all `1 ≤ h ≤ 65,535` (pass sizes `⌈(h−s)/k⌉`; nonlinear facts as explicit lemmas); every pixel write lands inside `4·W·H`; under Strict the frame rectangle is inside the screen, so `4·((top+r)·W + left+c) + 3 < 4N`. M4b: disposal 2 and 3 on rectangles, the `Saved` buffer (reserved at the first disposal-3 frame) and its row layout, the canvas equality `canvas_k = MC(canvas_{k−1}, …)` as a whole `Seq`, stated per position (inside the rectangle and drawn, or unchanged).

**M5 — integration (T9–T13, T-ITER).** `try_decode_still(b) = MS(b)` exactly, including every RGBA byte and the first error; for the iterator, T-ITER: for every input and every `k`, the `k`-th `try_next` returns the `k`-th element of `MF(b)` (frame record and canvas, then `None` or the error, then the same result again), stated over arbitrary reachable decoder states and all finite call sequences, as DIB's T7 was. Work bound T12: `steps ≤ c·(|b| + N + P)` for the still call and summed over iterator calls; every code costs at least 3 bits of input, every emitted index is charged to `P`, the chain walk of one code is charged to the indices it emits (at most 4,095 discarded per frame under `StopAtRaster`, a per-frame constant). Witnesses (`model_vectors.rs`, about 20 small gates), runtime conformance, and an implementation-side mutation campaign (below).

**P1 — Strict/Recommended.** Typed `#[non_exhaustive]` `Profile`; the 22-field record internal and `#[cfg(test)]`-constructible; the spec-level Strict bridge `M*(Strict) = M*` for every input; Recommended's eight fields with obligations E (E1, E4 are the only events reachable in Recommended) and R; T1/T3/T11/T12 universal over every record. Policy vectors generated by a policy generator in `policy-draft` style (expected values from intent, checked by `policy_oracle.py`).

**P2 — compatibility profiles.** GdkCompat-2.42.12 (sticky Graphic Control, `GdkTable` state, `KeepCanvas`, `StopFrame`, `SkipByte`, `restore_outside=Keep`, size-0 minimum code surrogate) and ImageCrateCompat-0.25.10 (`Streaming`, `empty_first_subblock`, unknown-extension rejection, `Transparent` indices). Gate each field separately: if `image_data=Streaming` (a second, interleaved model of the image data) stalls, drop it from the profile and record the residual rather than delaying the rest.

## Hardest verification obstacles

1. **LZW table and termination.** The prefix chain must be acyclic (`prefix[c] < c`) and bounded (`len ≤ 4096`) for the chain walk to terminate and the stack writes to stay in bounds; the KwKwK entry; the deferred clear at width 12 (no entry added, width unchanged, `next = 4096` never readable as a code); the model–executable bridge from `Seq<Seq<u8>>` to three parallel arrays.
2. **Bit reader across sub-block boundaries.** Codes straddle sub-blocks; the reader's local state must be related to a single ghost bit position in the concatenated payload, and the structural scan that defines that concatenation runs before decoding (G9).
3. **Interlace row mapping.** A bijection over four arithmetic progressions with empty passes for small `h`; it feeds every pixel-address proof.
4. **Compositing arithmetic.** `u32` sums of `u16` coordinates under `Clip` (up to 131,070), clipped rectangles for disposal and restore, `Saved` row strides, and a canvas-equality invariant over every position after every frame.
5. **The iterator invariant.** Decoder state (cursor, pending Graphic Control, previous frame's disposal and rectangle, `Saved`, frame and pixel counters) must equal the model's state after `k` frames for every `k`, with the fused error state.
6. **Linear work with long LZW strings.** One code can emit up to 4,095 indices; the bound charges them to `P`, and `MAX_TOTAL_PIXELS` keeps `P` at most 2^30.

## Mutation candidates (implementation-side, for M5)

Widen one code early (TIFF style) or late; drop the KwKwK case; read codes MSB-first; swap interlace passes 2 and 3; skip the empty-pass adjustment for `h < 5`; fill disposal 2 with the background colour; restore the whole canvas for disposal 3; let the Graphic Control persist to the next frame; accept a duplicate Graphic Control; treat a zero first sub-block as empty; check the colour index before the transparent index; accept `m = 9`; omit `DataAfterEndOfInformation`; reserve `Saved` per frame (T3 must kill it); rescan the remaining input per code (T12 must kill it).

## Compositor in the same crate?

Yes (QUESTIONS.md Q10). Indices are written straight to the canvas through the interlace map, so the compositor and the LZW emitter share the pixel-address lemmas; three of the 22 policy fields (`frame_bounds`, `index_range`, `short_frame`) act inside the emit loop; and a split would need an index buffer of up to 16.7 megapixels and a cross-crate allocation log (as ICO needed for DIB). Keep it a separate module with its own model function (`MC`) so its proofs run after M2–M3 without reopening them.
