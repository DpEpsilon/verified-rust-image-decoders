# Re-review of TGA-CORE-SPEC 1.0 draft 2 against REVIEW.md

Inputs: SPEC.md, VECTORS.json (167), SOURCES.md, DESIGN-NOTES.md, check_vectors.py, THEOREMS-DRAFT.md, CHANGELOG-draft2.md. The draft-1 copies were used only to diff.

Remaining: blocking 0, should-fix 1, nit 4.

Verdict: **freeze after one small fix (N-1 below: one sentence in R5 and two vectors).** Every draft-1 finding is fixed with evidence. Take the SHA-256 after the N-1 vectors land, since the vector file changes.

## 1. Status of each earlier finding

| ID | Status | Evidence |
| --- | --- | --- |
| B1 colour map on non-indexed images | Fixed | R4 (SPEC.md:11) validates a flag-one map for any image type and applies the range check to all of them; R7 (:17) consumes `length*ceil(depth/8)` bytes whenever flag is one; DESIGN-NOTES.md:6 gives the baseline-compatible rationale. 19 new `unused-map-*` vectors (16 success, 3 invalid-depth, range-overflow and truncated). Mutants "unused map not skipped", "not validated", "flag-0 fields ignored" (three variants), "range check only for indexed" are all killed. |
| B2 15/16-bit conversion untested | Fixed | New vectors `raw-16-mixed-0444`, `raw-15-mixed-0c63`, `raw-16-distinct-453e`, `palette-15-mixed-0c63`, `palette-16-mixed-0444`, `palette-16-distinct-453e`. Hand-checked: 0x0444 gives `081021ff`; 0x453e (r17,g9,b30) gives `8c4af7ff` (140, 74, 247). Mutants killed: `c*255/31` with and without +15, green or blue without replication, blue from green bits, red/green/blue taken from bits shifted by one, R/B swap. |
| S1 map base after ID | Fixed | `indexed-map-after-id-3`, `indexed-map-after-id-255`, `indexed-map-short-after-id`; mutants "map base at 18" and "ID 255 off by one" killed. |
| S2 map stride and index>0 | Fixed | `palette-{15,16,32}-index-6-type-{1,9}`, `palette-16-index16-second`; stride 2, 3 and 4 mutants all killed. |
| S3 ColorMapRange wording | Fixed | R4 now says flag-one coherence precedes range for **any** flag-one image and names the flag-zero example; `map-range-with-flag-zero` pins `InvalidColorMap`. Mutant ordering range before coherence is killed by `map-invalid-depth-before-range` and `unused-map-invalid-depth-before-range`. |
| S4 alpha policy | Fixed (decisions applied) | R8 (:19): true-colour 16 is always opaque; map entries with count one use bit 15. DESIGN-NOTES.md:8-12 names each case against the baseline: TC16 count-one opaque, map-entry 16 count-one bit-controlled, 32-bit count 0 drops A, grey16 count 0 drops the byte, 24/count 8 and 32/count 1..7 rejected, 32/count 8 all-zero A stays transparent. The two vectors `raw-16-alpha-zero` and `rle-16-alpha` changed to match; both re-derived by hand. |
| S5 RLE overrun vs clamp | Fixed | DESIGN-NOTES.md:14. |
| S6 baseline R/B swap | Fixed | SOURCES.md R5/R8 row and DESIGN-NOTES.md:13: red gets bits 0..4, blue gets bits 10..14, no replication; verified against io-tga.c:499-506 and 299-305. |
| S7 MR | Fixed | THEOREMS-DRAFT.md:13: MR takes the MH record, checked map, `cursor0`, `produced0`; returns converted RGBA pixels, final cursor and produced count, and on failure the error with ghost state. T6 (:26) compares exactly those. Statable. |
| S8 mutation list | Fixed | Old 10 split into 10 and 11; 12-20 added (see section 4). |
| S9 RECALLED labels | Fixed | SOURCES.md R4 (`first` absolute), R5/R8 (A1R5G5B5, count nibble, extension alpha type), R10/R11 (scanline-crossing) all tagged RECALLED; footer layout tagged READ with the right lines. |
| S10 feasibility | Fixed in text, one over-constraint | Bijection lemma (:35), separate conversion lemma (:37), repetition lemma (:39). L3/L4 need no 16 MiB evaluation. See N-3. |
| N1 R5 wording | Fixed | SPEC.md:13. |
| N2 boundary vectors | Fixed | `unsupported-type-4/8/12/32`, `interleave-128`, `interleave-192`, `rle-literal-crosses-row`, `gray-origin-bottom-right`, `indexed-origin-top-right`, `dimensions-before-interleave`, `depth-before-attribute`. |
| N3 lazy allocation | Fixed | SPEC.md:15 permits it and keeps R14 priority. |
| N4 map bound | Fixed | SPEC.md:17 derives 262,140 from the 16-bit length and 4-byte entry. |
| N5 L2 wording | Fixed | THEOREMS-DRAFT.md:54. |
| N6 RLE indexed attribute vectors | Fixed | `attribute-rle-indexed-15-one`, `attribute-rle-indexed-24-one`. |

## 2. Independent oracle over all 167 vectors

I rewrote my oracle for the new text (`/tmp/claude-0/---modal-volumes-vo-Iyh3VLkOB7cEHrIp2RkTs7/07bf9765-af64-477d-a906-dbb14d47e2dd/scratchpad/myoracle2.py`; the draft-1 file was lost to a scratch cleanup, so it is a rewrite from the new SPEC.md plus my earlier code). Changes taken from the text: R4 flag-zero/flag-one branches with range check for any flag-one image; R7 map consumed whenever flag is one; R8 true-colour 16 always opaque, map-entry 16 bit-controlled at count one. Result: **0 disagreements on 167/167**, with the 64 MiB input vectors expanded and both 64 MiB output vectors fully decoded. 49 new vectors, 0 removed; 2 old vectors changed (the two alpha ones, S4). 163 inputs are at most 277 bytes (the longest carries a 255-byte ID); four are the large ones. All 54 witness names, and every vector name used in the mutation list, exist in VECTORS.json; 54 = 40 + 14 and "the other 109" = 163 - 54 are both correct.

So the 49 new vectors now do have an independent second oracle result, contrary to the last sentence of CHANGELOG-draft2.md and SOURCES.md:39 ("the earlier 118 fixtures also passed an independent reviewer-written oracle"); update both after this review.

## 3. Mutant rerun

62 single-point mutants of my oracle (the 53 from REVIEW.md retargeted at the new text, plus 9 for non-indexed maps, flag-zero fields and bit positions), run against all vectors except the two 64 MiB output ones (Python is too slow; they exist to kill output-cap `>=`, not run here). 57 killed by a disagreement, 3 killed by an out-of-bounds crash (index `<=` upper bound, control-byte `>` for `>=`, and one disabled-branch test), 2 survivors:

- "range check before coherence" as I coded it is an equivalent mutant (it only fires when coherence already passes). The real ordering mutant (range reported even for incoherent depth) is killed by two vectors.
- **"Attribute policy for a non-indexed image with a map taken from the map's entry depth": survives.** This is N-1.

Previously surviving mutants (rounding, green/blue replication, blue from green, ID+map base, ID 255, map stride, range check without the flag) are all killed now. For 15/16-bit conversion, ID plus colour map, and map index>0, no survivor remains.

## 4. Declined mutation and arithmetic claim

The claim is correct. R2 runs before R6 and caps both dimensions at 8,192, so `N<=67,108,864` and `4*N<=268,435,456 < 4,294,967,296 = 2^32`. `4*(Y*w+X)+4<=4*N` is also below `2^32`, and on a 32-bit target `usize` holds it. The `u32` mutant would therefore not fail T1 and the manifest rightly drops it. The replacement, `N` in `u16`, is a real overflow: 256x256 gives 65,536. It rests on the implementation computing in the R2-then-R6 order; an implementation that multiplied before the R2 test (up to 65,535x65,535) could overflow `u32`, and T5 (header equals MH) is what catches that, so the manifest's text is right as written. One small point: mutation 16 lists `output-at-64m-8192x2048` as the failing vector, but a runtime test with that vector would hit the width-narrowing bug at 256x256 or above; any `N>65,535` vector such as `rle-128-boundary` is too small. Only the two large vectors exercise it, so the verifier obligations (T1/T2/T8) carry the mutation, as the text says.

## 5. THEOREMS-DRAFT.md check

- T6 is statable (S7).
- Mutation coverage: T1-T4: 15 (literal guard, T3/T1), 16 (width, T1/T2/T8), 17 (palette guard, T4/T1), 11 (footer dereference, T1); conversion: 12, 18, 19, 20 (+ 5); map placement and stride: 13, 14; suffix split: 10, 11; termination: 6 now names T3/T10. Each named vector fails the matching mutant in my run (mutations 1-10, 12-14, 17-20 checked; 15, 16 are verifier-only by construction).
- 54 witness names are all real (checked by script).
- Feasibility: no obligation needs 16 MiB-scale evaluation (L3/L4 by the general repetition lemma, instantiated once each). The remaining heavy items are the bijection lemma and the 32/64-bit parameterisation; see N-3.

## Findings

### N-1 (should-fix). Attribute policy for non-indexed images that carry a map is unstated and untested
R5 (SPEC.md:13) says only indexed images take their policy from the map entry depth; for the non-indexed rows it lists image depth. That reading is clearly intended, but nothing pins it now that these images can have maps. Mutant "non-indexed policy from map depth" passes all 167. Concrete inputs: type 2, depth 24, flag 1, one 32-bit map entry, descriptor 0x28 (count 8): `InvalidAttributeBits` (image depth 24 requires zero). Type 2, depth 32, flag 1, one 24-bit entry, descriptor 0x28: success, alpha from the pixel (map depth irrelevant). Add one sentence to R5 ("for non-indexed images the count is governed by the image depth; an attached map's entry depth is irrelevant") and these two vectors.

### N-2 (nit). Flag one with length zero is rejected; the baseline appears to accept it
R4 requires nonzero length when flag is one. The baseline's loader loops over `n_colors` and would accept length 0 (io-tga.c:492-524); I did not verify what `gdk_pixbuf_buffer_queue_pull(…, 0)` returns, so this is uncertain. DESIGN-NOTES.md:6 says the validation is "the same as for indexed maps" but names no divergence. Either say so there, or accept length 0 on non-indexed images and skip zero bytes (the entry depth would then be unchecked).

### N-3 (nit). THEOREMS-DRAFT.md:35 forbids nonlinear arithmetic automation
"The proof must not rely on nonlinear arithmetic automation for variable dimensions." For Verus the usual route is an explicit lemma proved with `by(nonlinear_arith)` or the library multiplication lemmas; banning it makes the lemma harder for a Flash-class implementer without adding soundness. Say "state each nonlinear fact as a named lemma" instead.

### N-4 (nit). Stale provenance sentences
CHANGELOG-draft2.md (last paragraph) and SOURCES.md:39 say no independent second oracle result exists for the 49 new vectors; see section 2.

### N-5 (nit). Mutation 16 vector attribution
See section 4: neither small vector can fail the `u16` mutant at run time; the large output vectors and the verifier obligations do. Reword the "must fail" clause to say which.

## Fixes before freeze

1. N-1: add the R5 sentence and two vectors; regenerate `check_vectors.py` expectations; update the 163/54/109 counts if witnesses are added.
2. N-2 to N-5: wording.
3. Then take the two SHA-256 hashes (SPEC.md, VECTORS.json) as for ICNS.

## REVIEW2 addendum (after N-1..N-5)

- **Oracle:** `myoracle2.py` on all 169 vectors: 0 disagreements. `check_vectors.py` also passes 169/169. The two new vectors, `unused-map-32-entry-24-image-attr-eight` (type 2, depth 24, 32-bit map, descriptor 0x28) and `unused-map-24-entry-32-image-attr-eight` (type 2, depth 32, 24-bit map, descriptor 0x28), match my N-1 cases exactly: `InvalidAttributeBits`, and success with pixel `33221144`. The mutant "non-indexed policy from map depth", the one REVIEW2 survivor, is now killed by the first of them.
- **N-1:** the R5 sentence ("For non-indexed images, the count is governed by the image depth; an attached map's entry depth is irrelevant.") says what I proposed.
- **N-2:** DESIGN-NOTES.md:6 is accurate. `colormap_new(0)` allocates `MAX(n,1)` entries (io-tga.c:145); the load loop at :497 runs zero times; `gdk_pixbuf_buffer_queue_pull(…,0)` returns non-NULL because `peek` special-cases length 0 with `g_bytes_new(NULL,0)` (gdk-pixbuf-buffer-queue.c, peek and pull bodies), so `tga_load_colormap` does not return early at :489. The baseline therefore accepts flag one with length 0. Keeping the strict rejection is a documented decision, not a defect.
- **N-3 to N-5:** CHANGELOG-draft2.md records them as applied; N-4's note is now superseded by this addendum (the oracle was rerun on the two N-1 vectors). I did not re-read THEOREMS-DRAFT.md line by line for these three wording changes.
- **Verdict: ready to freeze.** Blocking 0, should-fix 0, nit 0 outstanding. Take the SHA-256 hashes of SPEC.md and VECTORS.json as the next step.
