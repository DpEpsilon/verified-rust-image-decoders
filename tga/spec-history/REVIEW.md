# Independent review of TGA-CORE-SPEC 1.0 draft, VECTORS.json, SOURCES.md, DESIGN-NOTES.md, THEOREMS-DRAFT.md

Reviewer: independent of the author. Marking: RECALLED = from memory of the Truevision TGA 2.0 spec or of other decoders, not read from a primary source here. Primary sources actually read: `io-tga.c` (all 769 lines), the five spec inputs, the ICNS reviews and spec. No TGA specification was fetched (neither did the author: SOURCES.md says so).

Counts: blocking 2, should-fix 10, nit 6.

Verdict: **freeze after listed fixes.** SPEC.md is internally consistent and the error order is total; no vector is wrong. The two blocking items are one policy decision that freezing would lock in (B1) and a vector gap in the most error-prone conversion rule (B2). Both must land before the vector hash is taken.

## Checks run

- `python3 -I check_vectors.py`: 118 passed.
- Own oracle, written from SPEC.md R1-R16 (`/tmp/claude-0/---modal-volumes-vo-Iyh3VLkOB7cEHrIp2RkTs7/07bf9765-af64-477d-a906-dbb14d47e2dd/scratchpad/myoracle.py`; I had read `check_vectors.py` first, so independence is of code, not of reading): **0 disagreements on 118/118**, including the 64 MiB input vectors (expanded) and both 64 MiB output vectors (full decode). Neither the spec text nor any vector is wrong.
- Hand decodes of 8 vectors (raw-15-red 0x7c00 gives ff0000ff; palette-two-entries first=5 indices 5,6; palette-16-attr-1 bit 15 clear gives alpha 0; raw-32-alpha-zero-ignores-byte `12 34 56 00` gives 563412ff; origin-top-right; rle-crosses-row; output-too-large is 8192x2049, one row over the cap; L3 and L4 sizes). L3: 8192*2048/128 = 131,072 packets x 2 bytes + 18 = 262,162. L4: 131,072 x 3 + 18 = 393,234; `ff 00 fc` is word 0xfc00, red 31, bit 15 set, so `ff0000ff`. All match the stated sizes. Of the 118 vectors, 114 are at most 56 bytes expanded; the other four are the stated large ones. The 40 witness names are all real vector names (a regex over the section also matches the word `expected`, which is not one).
- Mutation test of the vector suite: 53 single-point mutants of my oracle, run against all vectors except the two 64 MiB output ones (too slow in Python; they exist to kill output-cap `>=`, which I did not run). 40 killed by a disagreement, 2 killed by an out-of-bounds crash in the oracle (index `<=` upper bound; control-byte test `>` for `>=`), 1 (output cap `>=`) expected killed by the excluded vectors, 2 equivalent (15-bit alpha from bit 15, because R5 forces attribute count 0; reversed write order inside a literal packet, because the error name is the same), and 8 real survivors, covered by B2, S1, S2, S3: `c*255/31` rounding, green without replication, blue without replication, blue extracted from green bits, map read at offset 18 ignoring the ID, ID 255 off by one, map stride fixed at 3 bytes, range check without the flag.

## Blocking

### B1. Non-indexed images with a colour map are rejected (SPEC.md:11; DESIGN-NOTES.md:7)
R4 returns `InvalidColorMap` for type 2/3/10/11 with flag 1, or with nonzero first/length/entry depth. gdk-pixbuf accepts such files: it sizes the map from the header, loads and discards it (`io-tga.c:341-343,486-526`). RECALLED: TGA 2.0 allows a colour map in true-colour files (as quantisation palette) and says nothing that forbids it in greyscale; real writers produce them. Concrete input: type 2, flag 1, length 1, entry depth 24, depth 24, 1x1, then 3 map bytes then 3 pixel bytes. Baseline decodes it; the spec returns `InvalidColorMap`.
The stated rationale, "no ambiguous payload cursor", is wrong: R7 already gives the map length as `length*ceil(entry_depth/8)`, so the cursor is unambiguous. This is a compatibility decision, and DESIGN-NOTES.md:11 asks the reviewer to decide the compatibility questions without listing this one.
Fix, either: (a) accept flag 1 on non-indexed images, require the same depth and length checks as for indexed (or only skip `length*ceil(depth/8)` bytes), and ignore the contents; add vectors; or (b) keep the rejection and rewrite DESIGN-NOTES.md:7 as "rejects real TGA 2.0 files that carry an unused palette; baseline accepts them". Recommend (a): rejection buys no safety.

### B2. 15/16-bit conversion is verified only at 0 and 31 in the red channel (VECTORS.json; SPEC.md:19)
Every 15/16-bit vector (raw-15-red, raw-15-bit15-ignored, raw-16-*, rle-15-repeat, rle-16-alpha, palette-15/16-*) has green=0 and blue=0 and red in {0,31}. Surviving mutants: expansion `(c*255+15)/31` instead of `(c<<3)|(c>>2)` (differs from c=3: 25 vs 24); green or blue without the `|c>>2` term; green and blue extracted from the same bits; green shift 5 changed within the zero range. All 118 vectors pass these. The manifest's mutation list (THEOREMS-DRAFT.md:54-63) has no rounding mutation either.
Add at least: true-colour 16, word 0x0444 (r=1,g=2,b=4) gives `08 10 21 ff`; true-colour 15 word 0x0c63 (r=g=b=3) gives `18 18 18 ff`; a word with all three channels at distinct values such as r=17, g=9, b=30; the same words through a 15-bit and a 16-bit palette entry. Add mutation 11 (replace expansion by `c*255/31`) to the manifest, mapped to the new vector and T7.

## Should-fix

### S1. Colour map position after the ID field is untested (SPEC.md:17)
No vector has a nonzero ID length together with a colour map (id-skip-and-coordinates-ignored has ID 3 and no map; every indexed vector has ID 0). A decoder that reads the map at offset 18 instead of `18+ID_length` passes all 118. Also no vector uses ID length 255, and none puts a truncation exactly at the ID/map boundary with ID>0 (`id-before-map` has ID short, not map short). Add: type 1, ID 3 bytes `61 62 63`, one 24-bit entry, one pixel; and ID 255.

### S2. Map entry stride and index>0 are untested for 15/16/32-bit entries (SPEC.md:11,19)
Every multi-entry or index>0 palette vector uses 24-bit entries (palette-two-entries, palette-index-16-upper). The remaining palette vectors use index 0 or the single entry. A decoder that hard-codes a 3-byte stride, or `ceil(depth/8)` rounded wrongly for 15, passes unless the index is 0. Mutant `(ix-first)*3` survives. RLE-indexed (types 9) also only appears with 24-bit entries. Add: type 1 with two 32-bit entries and a pixel using index 1; same with two 16-bit entries; a type 9 vector with 16-bit and 32-bit entries.

### S3. `ColorMapRange` condition omits the flag (SPEC.md:11)
The exception reads "a nonzero-length, supported-depth indexed map whose first+length>65,536", with no mention of flag 1. The only resolution is the following sentence "Check coherence/depth before range". Two implementers can read "indexed map" as "an indexed image with length and depth in range" and return `ColorMapRange` for: type 1, flag 0, first 65535, length 2, entry depth 24. check_vectors.py returns `InvalidColorMap`; no vector pins it (mutant "range check without flag" survives). Rewrite the sentence as "if flag=1, length>0 and depth is supported, then `first+length>65,536` gives `ColorMapRange`; any other violation gives `InvalidColorMap`", and add the vector.

### S4. Alpha policy: what real files do, and what DESIGN-NOTES.md does not say (SPEC.md:13,19; DESIGN-NOTES.md:6)
The strict choice does not mangle any file that carries a consistent descriptor, and rejects or drops alpha for these (baseline in brackets):
- 32-bit, attribute count 0, real alpha bytes: alpha discarded silently, output opaque [baseline keeps the byte, `io-tga.c:310-314`]. Pre-2.0 writers and some current ones write descriptor 0x00/0x20 here (RECALLED).
- 16-bit true-colour or 16-bit palette entries, count 1, bit 15 clear: output **fully transparent** [baseline: alpha 255 always, `:306`, `:495`]. This is the sharpest edge: a writer that sets count 1 but leaves bit 15 zero produces an invisible image.
- 24-bit with count 8, 32-bit with counts 1..7, 8-bit grey with count 8: `InvalidAttributeBits` [baseline ignores the nibble]. RECALLED that 24-bit files with descriptor 0x08 exist in the wild; not confirmed.
- 32-bit, count 8, alpha all zero (a "no alpha" file whose TGA 2.0 extension area says alpha type 0 or 1): output transparent; the spec ignores the extension field that exists to disambiguate this (RECALLED). The baseline does the same.
DESIGN-NOTES.md:6 says "rejects unconventional descriptors and may differ from legacy producer intent" but does not name the 16-bit invisible-image case or say the baseline differs for 32-bit/grey16 count 0. Add one bullet per case. Decision for the owner: whether 16-bit count 1 with bit clear should be opaque (compatible, and what 5-5-5 writers mostly intend) or transparent (strict). The spec is internally consistent either way.

### S5. Baseline differences present in SOURCES.md but absent from DESIGN-NOTES.md
Checked against io-tga.c, each strict choice and its documentation:

| Difference | io-tga.c | In DESIGN-NOTES? |
| --- | --- | --- |
| `first` honoured, bad index is an error (baseline: transparent black) | `:154-164` | yes, line 4 |
| 15-bit TC, 16-bit index added | `:257-281` | yes, line 5 |
| descriptor count authoritative | `:310-326` | partly, see S4 |
| map metadata must be zero on non-indexed (baseline accepts) | `:341-343,486` | line 7, wrong rationale, B1 |
| trailing bytes rejected (baseline discards) | `:183-190` | yes, line 8 |
| 8,192 dimension cap, 64 MiB caps | `:367` | yes, line 9 |
| RLE packet overrun is `RleOverrun` (baseline clamps and writes the prefix) | `:439,453` | **no**; line 10 covers only ordering. Only SOURCES.md R10/R11 mentions it |
| RLE/raw truncation: no partial image (baseline emits pixbuf progressively and fails at stop_load) | `:707-714` | n/a to a pure API |
| bad palette index in RLE repeat packets: error (baseline: transparent) | `:292,436` | covered by line 4 |
| 16-bit palette channels: baseline swaps R and B and does not expand | `:499-506` | **no** (S6) |

Add the RLE overrun row: a final packet whose count exceeds the pixels left is accepted by gdk-pixbuf and by many encoders that pad the last packet (RECALLED) and rejected here.

### S6. Baseline's 16-bit palette decoding is buggy and SOURCES.md does not say how (SOURCES.md:21; io-tga.c:499-506)
`color.b = (col>>7)&0xf8; color.r = col<<3`: the baseline assigns the red bits (10..14) to blue and the blue bits (0..4) to red, and does not replicate the top bits. SOURCES.md says only "neither expands five-bit channels". An implementer using the baseline as a cross-check would see swapped channels on any non-grey 16-bit palette (all current vectors are pure red, so this is also invisible in the vectors: B2). State the swap in SOURCES.md and DESIGN-NOTES.md. For true-colour 16 the baseline's expansion (`:299-305`) is `(c<<3)|(c>>2)`, identical to R8; say so, it is evidence for the formula.

### S7. `MR` is underspecified for a total packet model (THEOREMS-DRAFT.md:13)
`MR(s,N,u,indexed,first,length,map)` has no image depth, attribute count, or map entry depth, so it cannot produce R8 colours; its result is "samples", but T6 (line 26) equates the concrete helper with MR "including consumed cursor", which MR does not return. T6 as stated is not statable. Give MR the header record from MH, and return `(cursor', produced', decoded pixels | Err)`. Orientation stays in MP.

### S8. Mutation list: all ten map to a failing vector and theorem, but coverage is narrow (THEOREMS-DRAFT.md:54-63)
Checked with my oracle: mutations 1-10 each fail the vector named (1: input-at-64m; 2: type-before-flag-and-dimensions; 3: dimensions-8193-1; 4: palette-first-65535; 5: raw-16-opaque-bit-off; 6: rle-true-repeat and 20 others; 7: rle-repeat-overrun-no-sample; 8: rle-crosses-row; 9: origin-bottom-left; 10: trailing-one-byte, footer-opaque-offsets for the dereference variant). Mutation 6 (count 0) also breaks termination, so T10/T3 should be listed, not only T6/T7. Problems:
- Mutation 10 is two mutations (accept any suffix; dereference a footer offset). The second is a safety failure (T1), the first a semantic one (T7/T9). Split them.
- None targets T1-T4. Add: drop the `len-pos>=needed` check before a literal packet (T3 fails, OOB read); compute `4*N` in `u32` (T1/T2 fail on 8192x2048 at 32-bit usize); index the palette without the range test (T4).
- None targets conversion. Add rounding (B2), map base after ID (S1), map stride (S2), alpha policy for 32-bit and grey16 (vectors raw-32-alpha-zero-ignores-byte, gray16-alpha-zero already kill these).

### S9. SOURCES.md has no primary-format evidence for the format semantics the spec asserts
READ items are accurate (below). RECALLED items that the spec states as fact and SOURCES.md does not label: first-entry-index is an absolute index offset (R4, R8; Pillow's TGA plugin pads `start` entries, RECALLED, consistent); pixel depth 16 is A1R5G5B5 with bit 15 as the attribute; descriptor bits 3..0 are the attribute count; footer is 8 offset bytes + 18-byte signature (this one is READ at `io-tga.c:92-104`); the TGA 2.0 extension alpha-type field exists and R8 ignores it; RLE packets "should not" cross scanlines in the original spec and the decoder accepts crossing (RECALLED). Add RECALLED tags; none of them changes the spec text.

### S10. Proof feasibility for a Flash-class Verus implementer (THEOREMS-DRAFT.md:21-48)
No obligation needs 16 MiB-scale evaluation: L1-L4 are symbolic, and the 114 small vectors expand to at most 56 bytes. Risks, in order:
- T4/T7 write in orientation order, not sequentially. The loop invariant over a `Vec<u8>` must say `out[pos(j)]=pixel(j)` for all `j<produced`, which needs injectivity of `k -> 4*(Y*w+X)` and, for L3/L4, surjectivity (every output cell is written). That is nonlinear integer arithmetic with variable `w`, `h`; expect it to need explicit lemmas. Recommend the manifest name it as a standalone lemma (bijection of the four orientation maps on `[0,N)`), proved once and used by T4, T7, L3, L4.
- L3/L4 need a repetition lemma: `MP(H + P^k)` for a fixed packet `P` equals `k*128` copies of one pixel when `128k=N`. State it generally (induction on `k`), not per vector.
- T7+T11 are one large equivalence (6 types x 4 orientations x 5 colour conversions). A model split by colour function `conv(depth, attr, bytes)` proved separately from the loop will help; the manifest should say so.
- `|b|<=usize::MAX` in T1 (line 21) is vacuous for a Rust slice (at most `isize::MAX`); and SPEC.md:3 promises 32-bit safety that Verus normally cannot state unless `usize` width is a parameter. Say which width the proof assumes.

## Nit

- N1. SPEC.md:13 last sentence of R5 ("thus indexed depth 8/16 accepts the corresponding map-entry policy instead of requiring zero independent of map depth") is hard to parse. Say: "for indexed images the allowed counts depend only on the map entry depth; the index depth is irrelevant."
- N2. Boundary vectors that are missing but cheap: image types 4, 8, 12, 32 (only 0 and 255 tested); interleave 0x80 and 0xc0 (only 0x40, so "check only 0x40" survives); a literal packet (not repeat) that crosses a row; greyscale and indexed images under a non-default origin (origin vectors are 24-bit raw, plus one RLE).
- N3. SPEC.md:15 / R6: a header-only 18-byte file with output exactly at the cap makes the implementation request 64 MiB before the first pixel read (R14 puts `OutputTooLarge` before truncation, but does not forbid a cheaper order of allocation). State that the output may be allocated lazily or after a cheap minimum-length check, as long as the returned error is the R14 one.
- N4. SPEC.md:17 says "length*ceil(depth/8) is at most 262,140", true only for `length<=65,535`; since LE16 cannot exceed that it is correct, but say so.
- N5. THEOREMS-DRAFT.md:46 L2 "provided its 18 bytes exist" is redundant at length 64 MiB.
- N6. check_vectors.py computes its pixel conversion for indexed images from `map_depth` and `attr`, as the spec says; there is no separate check that the `attr` policy for indexed images with 15/24-bit entries rejects count 1 for all of types 1, 9 (vectors exist only for type 1, e.g. attribute-1-16-1-15).

## SPEC.md fidelity summary

- Header layout, six types, depths 8/15/16/24/32, interleave bits, origin bits and the flip rule match `io-tga.c:44-90,192-212,257-281,605-617`. Orientation rule is identical (`:201-202`).
- RLE semantics (bit 7 repeat, 7-bit count + 1, continuous stream across scanlines) match `:412-475`; runs crossing rows are accepted by both. Overrun and a packet cut off at the end of input differ (S5): baseline `:431-433,449-451` waits for more data, then fails at `stop_load`; spec returns `RleTruncated`.
- ID skipped, map consumed after the ID: matches `:550-561,477-526`; wording consistent.
- Output is always RGBA; the baseline emits RGB for some types. Pure-API difference, covered by DESIGN-NOTES.md:3.
- Footer: spec accepts suffix >= 26 bytes ending in the signature; baseline discards everything (`:183-190`). 2.0 files with extension area, developer area, scan-line table, postage stamp, correction table before the footer are accepted (suffix is opaque). Pre-2.0 files with arbitrary trailing bytes (for example a DOS EOF byte, RECALLED) are rejected with `TrailingBytes`; documented at DESIGN-NOTES.md:8.

## Error precedence

R14, R15, SPEC.md:5-31 and check_vectors.py agree on all 18 names and on order, and my oracle (written from the text) matches 118/118. The only place two implementers could differ is S3. Pairs that ought to be pinned and are: type/flag/dimensions, flag/dimensions, interleave/depth, depth/map, map/attribute, range/attribute, attribute/output, output/ID, ID/map, map/pixel, raw invalid index before later truncation, RLE packet bounds before first invalid index, overrun before data, pixel/trailing. Pairs not pinned: dimensions before interleave (only the reverse position is tested by `dimension-before-output`), `UnsupportedPixelDepth` before attribute count, and both interleave bits other than 0x40.

## SOURCES.md citation spot-check (12 checked, 12 accurate)

`io-tga.c:53-63` (type enum), `73-90` (header), `257-281` (format table), `564-624` (header load), `192-205` (write_pixel and flips), `605-617` (dimension and interleave rejects), `341-343` (cmap_size and map allocation for all types), `154-164` (transparent black), `299-306` (16-bit true-colour), `499-506` (16-bit palette, no expansion), `439,453` (clamp of RLE counts), `700-714` (stop_load incomplete error), `92-104` (footer struct), `183-190,407-409,470-474` (skip rest). Line ranges are exact; the paraphrases are accurate except the omission in S6.

## Fixes needed before freeze

1. B1: decide and rewrite R4 or DESIGN-NOTES.md:7; add vectors.
2. B2, S1, S2, S3: add the vectors listed; recompute `check_vectors.py` and the witness list (the manifest's 118/114/74 counts change).
3. S4, S5, S6: DESIGN-NOTES.md and SOURCES.md rows; decide the 16-bit count-1 policy.
4. S7, S8, S10: rewrite MR, split mutation 10, add mutations 11-14, name the orientation-bijection and repetition lemmas.
5. S9, nits: labels and wording.
