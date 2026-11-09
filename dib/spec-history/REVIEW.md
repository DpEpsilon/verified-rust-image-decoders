# Independent review of DIB-CORE-SPEC 1.0 draft, VECTORS.json, SOURCES.md, DESIGN-NOTES.md, THEOREMS-DRAFT.md

Reviewer: independent of the author. RECALLED = from memory, not read from a primary source here. Read in full: `io-bmp.c` (1548 lines), `io-ico.c` (1427 lines), the five spec inputs, `THEOREMS-DRAFT.md`, `run_corpus.py`, the TGA reviews. The Microsoft docs could not be fetched (network reset), so every Microsoft claim below is RECALLED; the author's "READ" marks on Microsoft pages are unverified by me.

Counts: blocking 2, should-fix 11, nit 7.

Verdict: **freeze after listed fixes.** Spec text and vectors agree (0 disagreements with my oracle on 186/186 and on 82 decodable corpus files, pixel-exact). B1 is a spec bug that rejects real Windows icons. B2 is a vector blind spot: a mutation sweep shows the vectors never test multi-pixel raw 1/4/8-bpp decoding, and most `BmpFile` paths. Both must land before the vector hash is taken. Section "Owner decisions" lists six strictness choices that need a ruling before freeze.

## Checks run

- `python3 -I check_vectors.py`: 186 passed.
- Own oracle written from SPEC.md R1-R12 (`/tmp/claude-0/---modal-volumes-vo-Iyh3VLkOB7cEHrIp2RkTs7/07bf9765-af64-477d-a906-dbb14d47e2dd/scratchpad/myoracle.py`; I had read `check_vectors.py` first, so independence is of code, not of reading): **0 disagreements on 186/186**, including the 64 MiB input vectors (expanded) and the 64 MiB output vector (full decode). No spec text or vector is wrong.
- Corpus: `run_corpus.py` reproduces the DESIGN-NOTES numbers exactly (49 accepted, 33 rejected, 2 not assessable; per-error counts match). My oracle gives the same verdict and identical RGBA bytes on all 82 decodable files.
- Mutation testing of my oracle: **125 single-point mutants** (header sizes, field layouts, every check boundary, adjacent-error order swaps, RLE escapes, padding, bit order, alpha and mask policies). **42 survive; 3 are false (see below), so 39 are real blind spots.** Survivors are items S1-S3 and B2.
  - False survivors: `out_ge` (killed by `output-exact-64m-rle8`, which I excluded from the sweep for speed), `swap3` (equivalent: the combo check only fires for compression 1/2/3, which already passed the compression check), `h16_unsigned` (equivalent: a width with the sign bit set is invalid either way).
- Citations: 27 checked against `io-bmp.c`/`io-ico.c` (list under finding 14).

## Blocking

**B1. ICO biSizeImage is XOR+AND in real files; R12 rejects them.** SPEC.md:27 requires nonzero `biSizeImage` to equal the XOR bytes. Four corpus files from ICO/CUR producers carry XOR+AND instead:
- `python.ico` (all 3 entries): 16x16x32bpp, `biSizeImage`=1088 = 1024 XOR + 64 AND. Same pattern for 32x32 and 48x48.
- `deerstalker.cur`: 32x32x32bpp, 4224 = 4096 + 128.
- `mask_1.cur`: 1x4 1bpp, 32 = 16 XOR + 16 AND (decoding from the directory offset to end of file also gives `InvalidImageSize`).
- DESIGN-NOTES counts one CUR among the three `InvalidImageSize` rejections; it is actually two ICO/CUR files in the shim's output plus `python.ico`, and `mask_*.cur` fail earlier for a shim reason (finding 12).
This is the layout Windows icon tools write (RECALLED), so the rule would reject mainstream icons. Fix: in `IcoEntryDib`, accept `biSizeImage` in {0, XOR bytes, XOR+AND bytes}, or ignore it. Whichever is chosen needs three vectors (equal XOR, equal XOR+AND, other value rejected); today no ICO vector has a nonzero `biSizeImage` (mutant `img_incl_mask` survives). Also move the check after the mask-length computation, and update R14, `check_vectors.py`, and mutation 21's neighbours.

**B2. Vector blind spots in the most error-prone arithmetic.** Every raw (BI_RGB) 1/4/8-bpp vector is 1x1 (`indexed-*-header-*`, `ico-*`). Widest accepted raw vectors: 24-bpp 2x1, ICO 24-bpp 9x1 (only the AND mask is wide). Consequently these mutants all survive:
- 4-bpp nibble parity for odd x (`nib_x_odd_high`: always high nibble), 1-bpp bit position for x>0 (`bit_x_msb_only`), byte index `ceil` instead of `floor` for x>0 (`byte_idx_ceil`);
- any row padding for indexed images, since a 1x1 indexed image has one row and the padded stride only matters if bytes follow;
- top-down for indexed and for BITFIELDS (`td_indexed_ignored`, `td_bf_ignored`; top-down only appears for 16/24/32 BI_RGB at 1 pixel);
- `BmpFile` with palette, RLE, BITFIELDS or 16/32-bpp: the only accepted `BmpFile` vectors are 24-bpp 1x1 plus `bmp-gap-before-pixels`, so using `bfOffBits` vs "right after palette" for indexed raw, RLE and BITFIELDS is untested (`bmp_indexed_ignores_offset`, `bmp_rle_ignores_offset`, `bmp_bf_ignores_offset`);
- `bfOffBits` that points inside the 12 external mask bytes (`bmp_off_ignores_masks`; the "two InvalidPixelOffset check sites" claim in T2 has one site untested for masks);
- 16-bit alpha masks (`alpha16_ignore`: 16-bit vectors have alpha mask 0 or saturated), and mask scaling other than 5-bit max=31 and max=63 extremes (`scale_halfup`: `floor(max/2)` vs `ceil(max/2)` differ for 5-bit c=11, 255*11+15=2820 -> 90, +16 -> 91).
Add: raw 1/4/8-bpp with widths 3, 5, 9, 33 and 3 rows, bottom-up and top-down; `BmpFile` versions of indexed, RLE8, RLE4, BITFIELDS 16 and 32 with a gap; BITFIELDS top-down; 4-4-4-4 16-bpp alpha; 565 with a mid-range green (c=11 or 21).

## Should-fix

**S1. Precedence pairs never separated by a vector** (each is a surviving order-swap mutant; R14 asserts the order, so each needs one input with both defects):
- `FileTooLarge` before `BmpFile` checks (`toolarge_after_hdr`): needs a >64 MiB `BmpFile` vector (symbolic repeat is already supported).
- `InvalidFileSize` before basic `InvalidPixelOffset` (`off_before_size`); basic offset check before header errors (`off_basic_late`): a `BmpFile` with bad offset and `UnsupportedHeaderSize`.
- `TruncatedMasks` before `InvalidColorCount` (`tm_before_color`) and before `OutputTooLarge` (`tm_before_out`).
- Pixel errors before `InvalidImageSize` (`img_before_pixels_raw`); `InvalidImageSize` before `TruncatedAndMask` (`and_before_img`); `InvalidImageSize` before `TrailingBytes` (`trail_before_img`).
- Witness `compression-before-combo` and `mask-before-palette` are vacuous: BITFIELDS requires depth 16/32 so there is never a palette to truncate; invalid compression values never reach the combo check. Remove them from the T-list or state in R14 that the pair cannot interact.

**S2. Other single-point vector gaps** (each a surviving mutant):
- `bfSize` greater than `|b|` accepted (`size_gt_ok`); second reserved field at offset 8 nonzero (`res2_ok`; only the field at 6 is tested).
- `bfOffBits` equal to `|b|` (`off_hi_ge`), 17 and 18 boundaries (`off_lo17`, `off_lo19`).
- Height exactly 8192 accepted (`hcap_ge`); CORE height 0xFFFF must give `InvalidDimensions`, a signed reading would give a valid top-down 1-row image (`core_signed`); `H=16` with negative height (`h16_height_unsigned`).
- `biSizeImage` larger than consumed (`img_ge`).
- Alpha mask overlapping an RGB mask (`mask_overlap_rgb_only`); BITFIELDS top-down (`bf_neg_reject`); BITFIELDS inside `IcoEntryDib` (`ico_bf_reject`; no vector at all).
- V4/V5 header with a nonzero alpha mask under BI_RGB 32-bpp must be ignored (`v4_masks_rgb32`; R6 states it, no vector).
- RLE: nonzero absolute pad byte (`abs_pad_zero`, DESIGN-NOTES promises it is accepted); `00 02 00 00` zero delta (`delta_zero_reject`, T11 mentions it); stream ending right after the last EOL with no EOB must give `RleTruncated` (`eob_implicit_full`); invalid unused low nibble of an odd RLE4 run or absolute packet must be accepted (`rle4_check_low_odd`, `rle4_abs_check_low_odd`).
- Nonzero unused pixel bits in a row (`unused_bits_zero`).

**S3. High-depth `biClrUsed` rejects four real-producer-style files.** SPEC.md:13 gives `InvalidColorCount` for `biClrUsed>0` at 16/24/32 bpp: `rgb24pal.bmp` (used=256, `bfOffBits`=1078=54+1024), `rgb24largepal.bmp` (300), `rgb16-565pal.bmp` (BITFIELDS, used=256, offset 1090=54+12+1024). Microsoft permits this optimal table (RECALLED; SOURCES cites MS-WMF). DESIGN-NOTES says skipping is not enough, but in `BmpFile` it is: `bfOffBits >= p + 4*used` is the same check R7 already makes. See owner decision D2.

**S4. V5 profile data and trailing bytes.** `rgb24prof.bmp` and `rgb24lprof.bmp` (V5, 24 bpp) fail only on `TrailingBytes`; the pixel array is fully valid. See D3.

**S5. THEOREMS T3 contains unverifiable claims.** `peak_decoder_stack<=512` cannot be stated in Verus (no stack model). `peak_decoder_heap<=4N` needs an allocator model that vstd lacks. Replace with checkable statements: exactly one allocating call (`Vec::with_capacity`/zeroed vec of `4N`), none in loops, no `clone`/`to_vec`/`collect`/`format!`; stack as a code-review rule (no recursion, no large locals). Same for the "implicit format I/O" clause in T1.

**S6. THEOREMS T7/MR over-specifies.** `MR` returns the ghost partial output `out'` at the failing check, but errors discard output (R12 last sentence). Requiring the concrete helper to match partially painted bytes at an error point adds a loop-invariant obligation with no observable consequence. Keep error name, cursor, `(x,y)`; drop `out'` on `Err`. Same trim for the partial-state parts of T6.

**S7. Lemma L3 (SPEC symbolic RLE, THEOREMS:46) is likely infeasible for a Flash-class implementer.** It needs an inductive proof over a 266 KB repeated byte string with `Seq` repeat/concat reasoning and a pixel-sequence invariant over `8192*k` elements; the expected output is also all black, so it equals the initial fill and cannot distinguish "paints nothing" from correct painting. Replace with a parameterised lemma on a small width (for example a 2-pixel-wide, k-row RLE8 stream: cursor `= 6k`, `y=k`) plus the generic row lemma, and treat the 64 MiB vector as a runtime test only. Also make the big vector paint something non-black.

**S8. Mutation list gaps.** The 22 mutations all map to real vectors and plausible failing theorems (I re-ran the vector-detectable ones on my oracle: all are killed by their cited witness; #13 and #14 share one witness but are separate code changes, fine). Missing mutations for rules the vectors do kill: AND mask bit order (`ico-mask-msb-and-padding` is a witness with no mutation), RLE4 nibble order (`rle4-encoded`), delta, early EOB, `bfOffBits` gap (`bmp-gap-before-pixels`), contiguity (`masks-hole`), bit 15 ignored in 555. Add them. Also add the new vectors from B2/S1/S2 as witnesses.

**S9. DESIGN-NOTES factual error.** DESIGN-NOTES.md:17 says the baseline ICO converter would misrender BITFIELDS entries. `io-ico.c:366-371` skips every entry whose compression field is nonzero, which includes BITFIELDS (3), so BITFIELDS ICO never reaches the converter; the baseline simply rejects it. The spec accepting ICO BITFIELDS is a new behaviour with zero vectors (S2).

**S10. ICO 32-bpp alpha policy.** R12 applies AND as a veto on top of XOR alpha. Windows reportedly ignores the AND mask when any XOR alpha is nonzero, and uses the mask when all alpha is zero (RECALLED). The spec makes an all-zero-alpha 32-bpp icon with AND=0 fully transparent, which is the opposite of the Windows result. Corpus is indifferent (no 32-bpp ICO entry in the corpus has a pixel with alpha>0 and AND=1; `ico32_win` mutant only changes `ico-32-and-1`). See D4.

**S11. Slack and wrong-length ICO entries.** `hopper_mask.ico` has 64 bytes beyond the AND mask inside the directory-declared entry; `pillow.ico` entries 1 and 2 have an AND mask stride that is 4 bytes per row too wide (48x48: 192 extra bytes). Strict `TrailingBytes` rejects both. D5.

## Nit

12. `mask_1.cur`/`mask_L.cur`: the directory length is 8 for an 80-byte and 1096-byte DIB, so `run_corpus.py` hands the core 8 bytes and reports `TruncatedHeader`. This is a container/shim artifact, not a core verdict; DESIGN-NOTES counts them among core rejections (2 `TruncatedHeader`). Re-measured from the directory offset to end of file they still fail (`InvalidImageSize`, B1).
13. `TruncatedDibHeader` and `TruncatedPalette` are unreachable in `BmpFile` (offset must be in [18,|b|] and >= palette end), so the all-26-errors-in-every-mode reading is false; say so in R15.
14. Citation check (27): `io-bmp.c` 269-286, 292-342, 305-323, 353-375, 442-445, 447-510, 535-566, 553-567, 583-631, 604-610, 633-647, 649-736, 688-711, 713-728, 799-803, 829-893, 874-880, 884-892, 918-979, 956-975, 981-1067, 1092-1101, 1105-1268; `io-ico.c` 214-394, 366-388, 399-410, 543-547, 619-623, 637-658, 790-815, 817-850 (skip at 823-825), 893-905, 1235-1254 all match their claims. Exceptions: DESIGN-NOTES' `io-ico.c:401-429,524-534` is partly wrong (the directory-size comparison is 421-428; 524-534 is pixbuf allocation); SOURCES' `io-ico.c:1235-1254` is the saver, usable as evidence only of the doubled height the writer emits.
15. R8 (SPEC.md:19) says "use 5:5:5 masks" for BI_RGB 16 but never says the R6 rounding `scale` applies; DESIGN-NOTES implies it. State it.
16. CORE (12-byte) headers accept depths 16 and 32 and OS/2 2.x 16/64 headers use signed width/height; the OS/2 fields are unsigned 32-bit (RECALLED). Harmless (invalid values are rejected either way) but note it.
17. The basic lower bound `bfOffBits>=18` is arbitrary (14+4); it is observable only through error precedence (S1/S2). Say that it only guarantees the size word exists.
18. `TruncatedPixels` demands the final row's padding; writers that drop trailing pad bytes are rejected. Mention in DESIGN-NOTES.
19. R11 delta (`x+dx<width`, `y+dy<height`) is stricter than the model's own states: after a run that ends at `x=width`, `00 02 00 01` is `RleBounds`, while EOL may set `y=height`. Intentional per vectors (`delta_le` is killed), but not discussed in DESIGN-NOTES' baseline comparison. D6.

## Error order: can two implementers disagree?

No contradictions found. R14 is total and my independently written oracle matched all 186 vectors on first run. Points where text alone leaves room, all resolved by reading R3/R5/R14 together: ICO `N` uses visible height (R5 says `width*height`; stated by R3); `InvalidImageSize` for `BmpFile` counts from `bfOffBits`, not from the palette end; the unused low nibble of an odd RLE4 run is not an index (S2 has no vector); the 16-bit scale rule (nit 15).

## Corpus: strictness decisions and owner decisions

Strict rejection justified (corrupt or hostile by design): `badpalettesize` (used=305402420), `pal8oversizepal` (used 300 at 8 bpp, also rejected by the baseline), `badwidth`, `decompression_bomb.ico`, `badfilesize` (but see D1), `l2rgb_read`, `reallybig`, `shortfile`, `mmap_error` (file is 1 byte short of `bfSize`, pixels also short), `badplanes`, `pal8badindex`, `badrle` and `hopper_rle8_row_overflow` (a 255-run at x=127 of a 128-wide row, injected into otherwise valid `hopper_rle8.bmp`), `badheadersize` (66), `badbitcount`, `rgb24jpeg`/`rgb24png` (depth 0, compression 4/5), `rletopdown`, `badbitssize` (img=2129587950).

| # | Rejected files | Rule | Recommendation | Owner decision? |
| --- | --- | --- | --- | --- |
| B1 | `python.ico`, `deerstalker.cur`, `mask_1.cur` | ICO `biSizeImage` must equal XOR bytes | **Relax**: accept 0, XOR, XOR+AND | No, spec bug |
| D1 | `badfilesize` (bfSize 2111692253, data complete); stale/appended-byte BMPs | `bfSize==|b|` and trailing bytes (`InvalidFileSize`, `TrailingBytes`) | Keep strict, or accept `bfSize==0` and `bfSize<|b|` for BMPs whose pixel array is complete. Windows ignores `bfSize`; real producers write 0 or stale values | **Yes** |
| D2 | `rgb24pal`, `rgb24largepal`, `rgb16-565pal` | `biClrUsed>0` at 16/24/32 bpp | **Relax** in `BmpFile`: skip `4*used` bytes after header/masks, require `bfOffBits>=` that end. Keep reject in `BareDib`/ICO | **Yes** |
| D3 | `rgb24prof`, `rgb24lprof` | Trailing bytes after V5 pixel array | Relax for `H=124` when the remainder is exactly the span named by `bV5ProfileData`/`bV5ProfileSize`, else keep reject; costs three more header fields | **Yes** |
| - | `pal8os2sp` | CORE header, palette shorter than 2^depth (`bfOffBits` inside the table; 252 entries) | Keep strict (CORE has no count; inferring from offset is a heuristic) | Yes if wanted |
| - | `rgba32abf` | BI_ALPHABITFIELDS (6), 40-byte header, 4 external masks | Keep strict, or add (one extra mask word, `bfOffBits`=70 matches); rare | Optional |
| - | `pal2` | 2 bpp (Windows CE) | Keep strict | No |
| D4 | none in corpus | ICO 32-bpp: AND as veto, all-zero alpha preserved | Prefer Windows rule: if any XOR alpha is nonzero ignore AND, else use AND; or at least document risk | **Yes** |
| D5 | `hopper_mask.ico`, `pillow.ico` | Bytes beyond AND mask inside the entry | Keep strict (the core gets exactly the entry; slack means a producer bug) | Yes if wanted |
| D6 | none | Delta to `x=width` or `y=height` | Keep strict | No, document |
| - | `mask_1.cur`, `mask_L.cur`, `no_cursors.cur`, `hopper_draw.ico` | Container-level (wrong directory length, empty, PNG-only) | Not core decisions; fix the shim to read to end of file for a measurement | No |
| - | `decompression_bomb.ico` and the 64 MiB/8192 caps | Resource policy | Keep; note a 600 dpi A4 scan (about 35 MP) is `OutputTooLarge` | Optional |

## THEOREMS-DRAFT.md notes

- **Functional correctness:** T2/T4-T10/T13 state exact equality with total models `MH`/`MP`/`MR`/`MA` including every output byte, so this is not a safety-only manifest. Good.
- **Allocation bound (T3):** the `4N` heap part is precisely stated and provable from the code shape; the stack part and "peak" accounting are not (S5).
- **Work bound (T12):** well formed. Counting steps with R18's accounting, the worst case is raw ICO at w=1: `4N` init + `N` rows/pixels XOR + `N` rows/pixels AND... about `8N`, RLE about `4N + N + |b|/2`. `c=16` leaves a factor 2 or more; sensible. Empty input has zero loop iterations so the bound holds at `|b|=0`. The invariant `steps<=8*cursor+8*N` is tight at 1xN ICO and needs the per-phase split the draft already asks for.
- **L1, L2:** hold (`|b|>cap`; zero size word at exactly the cap). **L3:** arithmetic holds (130*2048=266,240; 44+266,240+2=266,286; 8192*2048 = 2^24; 4N = 67,108,864 exactly) and my oracle decodes the full vector in agreement, but see S7.
- **Witnesses:** all 54 names exist as vectors; no name is wrong. Two are vacuous (S1).
- **22 mutations:** meaningful; each fixture fails under the matching change on my oracle (#1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 21, 22; 17 and 18-20 are proof-side). Add those in S8.
- **Proportionality for a DeepSeek-Flash-class implementer in Verus:** likely infeasible or very costly: L3 (S7); the ghost partial-output equality in MR (S6); stack bound (S5); evaluating 183 spec-level models concretely by `assert` with spec-function fuel (set per-fixture fuel or use exec tests + a smaller subset of 15-20 representative proof witnesses); the RLE state machine (cursor, `x`, `y`, absolute packets, nibble parity, padding) proven against `MR` in one function. Recommend splitting the RLE helper into command-decoder (pure) and painter and proving each against its model; stating `stride_roundup_safe` etc. with bounded `w<=8192` so `by(nonlinear_arith)` or `bit_vector` suffices; and the nonlinear row-padding facts via the named lemmas the draft already lists (feasible with `w*d<=2^18`).

## Files

- `/__modal/volumes/vo-Iyh3VLkOB7cEHrIp2RkTs7/projects/dib/REVIEW.md` (this file)
- scratch: `/tmp/claude-0/---modal-volumes-vo-Iyh3VLkOB7cEHrIp2RkTs7/07bf9765-af64-477d-a906-dbb14d47e2dd/scratchpad/{myoracle,mutate,mut2,cov,corp,dump,ico,ico2,rle,big}.py`, `mut.out`
