# Second review of DIB-CORE-SPEC draft 2 (200 vectors, 76 witnesses)

Reviewer: same independent reviewer as REVIEW.md. Scratch files are under `/tmp/claude-0/---modal-volumes-vo-Iyh3VLkOB7cEHrIp2RkTs7/07bf9765-af64-477d-a906-dbb14d47e2dd/scratchpad/` (`myoracle.py` is the updated oracle, `mutate.py` the sweep, `mut2.out` its output). Microsoft docs still could not be fetched; Microsoft claims remain RECALLED.

Counts (new findings only): blocking 0, should-fix 4 (N1, N2, N3, N5), nit 4 (N4, N6-N8).

Verdict: **not ready to freeze; freeze after N1, N2, N3 and N5 are fixed.** Every draft-1 finding is resolved or consciously kept, and the D1-D4 rules are coherent and unambiguous. But in making room under the 200-vector ceiling the author deleted three single-error vectors, so three RLE/BITFIELDS checks that draft 1 covered are untested again. The new D2 and D4 rules also have boundary cases with no vector. All are vector additions; no spec text changes are required except two wording nits.

## Checks run

- `python3 -I check_vectors.py`: 200 passed.
- Updated oracle (from the new SPEC.md text only; changes: BmpFile `bfSize` rule, high-depth optimal table, R12 `biSizeImage` set and the D4 whole-image alpha rule, BmpFile no-trailing-check): **0 disagreements on 200/200**, including the 64 MiB input vectors expanded (the too-large BmpFile vector is now genuinely expanded and decoded, not short-circuited) and the 8192x2048 output vector decoded in full.
- Corpus differential: my oracle and `check_vectors.py` agree on all 82 decodable files, pixel for pixel.
- Mutation sweep: **142 single-point mutants** (125 from draft 1 adapted, 17 new for D1/D2/D4 and the new `biSizeImage` rule). 18 survive; 6 are equivalent, no-op or killed by the large vector, leaving **12 real survivors in 8 distinct gaps** (N1-N5).

## Status of draft-1 findings

| Finding | Status | Evidence |
| --- | --- | --- |
| B1 ICO `biSizeImage` | **Fixed** | R12 accepts {0, X, X+A} in ICO, X only elsewhere, A computed before the check. Mutants `ico_img_xor_only`, `img_noA`, `img_ge` are killed by `ico-image-size-xor-and`, `ico-image-size-bad`, `image-size-above-consumed`. `python.ico`, `deerstalker.cur`, `mask_1.cur`, `mask_L.cur` now accepted. |
| B2 multi-pixel raw, BmpFile paths | **Fixed except top-down BITFIELDS** | Killed now: `nib_x_odd_high`, `bit_x_msb_only`, `byte_idx_ceil`, `td_indexed_ignored`, `bmp_rle/indexed/bf_ignores_offset`, `bmp_off_ignores_masks`, `alpha16_ignore`, `scale_halfup`. Widths 3, 5, 9, 33 at 1/4/8 bpp are present; BmpFile has indexed, RLE and BITFIELDS-with-gap vectors. Survivor `td_bf_ignored`: `fields-topdown-16/32` have two identical rows (N3). |
| S1 precedence pairs | **Fixed** | All draft-1 order-swap survivors (`toolarge_after_hdr`, `off_before_size`, `off_basic_late`, `tm_before_color`, `tm_before_out`, `img_before_pixels_raw`, `and_before_img`, `trail_before_img`) are killed. Vacuous witnesses removed; R15 states mutually exclusive sites are not compared. |
| S2 single-point gaps | **Fixed except `h16_height_unsigned`** | `size_gt_ok`, `res2_ok`, `off_hi_ge`, `off_lo17/19`, `hcap_ge`, `core_signed`, `img_ge`, `mask_overlap_rgb_only`, `ico_bf_reject`, `bf_neg_reject`, `v4_masks_rgb32`, `abs_pad_zero`, `delta_zero_reject`, `eob_implicit_full`, `rle4_*_low_odd`, `unused_bits_zero` all killed. `h16_height_unsigned` (accepted OS/2-short negative height) still has no accepting vector; `height-8193-os2short` only covers the rejection. Minor. |
| S3 high-depth `biClrUsed` | **Applied (D2)**; new boundary gaps, N2 | |
| S4 V5 profile / trailing | **Applied via D1**; coherent, see below | |
| S5 T3 | **Fixed** | "Exactly one allocating call requesting `4N`", ghost call counter, no stack or peak-heap claim; R17 matches. |
| S6 MR partial output | **Fixed** | `MR` and T6 return exact error and cursor/`(x,y)` only. |
| S7 L3 | **Fixed** with a wording nit (N4) | Parameterised 2-pixel-wide, k<=16 rows lemma; `rle8-small-red-rows` (2x3, six red pixels) matches it; the 64 MiB vector is a runtime test and now **paints** red over a black initial fill (16,777,216 red pixels, input 48+130*2048+2=266,290 bytes, arithmetic checked). |
| S8 mutation gaps | **Fixed** | Mutations 23-31 added. I re-ran their code changes on my oracle: AND bit order (`mask_lsb`, 9 vectors), RLE4 nibble order (7), delta (2), early EOB (`eob_requires_full`, 5), BmpFile gap (13), contiguity (`masks-hole`), 555 bit 15 (7), ICO `X+A` (1), D4 veto (`ico-32-and-1`, `ico-alpha-nonzero-ignores-and`) are each killed by their cited witness. |
| S9 DESIGN-NOTES ICO BITFIELDS | **Fixed** | Now says `io-ico.c:366-388` skips BITFIELDS entries and the support is an extension. |
| S10 ICO 32-bpp alpha | **Applied (D4)**; BITFIELDS gaps, N2 | |
| S11 ICO slack | **Kept per D5** | `hopper_mask.ico`, `pillow.ico` remain `TrailingBytes`. |
| N12-N19 nits | **Fixed** | Shim reads to EOF for the two single-entry `.cur` files with declared length 8; R15 lists the unreachable errors; citation `io-ico.c:421-428` and the writer label are corrected; R8 states rounded scale (3 -> 25); R3 flags the signed OS/2 reading; R1 explains the 18 bound; DESIGN-NOTES covers final-row padding (line 8) and delta strictness (line 15). |

## New findings

**N1. Three draft-1 RLE/BITFIELDS vectors were deleted and their checks are untested again (should-fix).** `rle4/8-run-over-row`, `rle4/8-eol-past-height`, `masks-outside` no longer exist. Surviving mutants:
- `run_clip`: an encoded run that passes the row end is clipped instead of `RleBounds` (e.g. RLE8, width 3, `04 00` at `x=0` must give `RleBounds`). The large vector does not kill it (I ran it).
- `eol_nobounds`: a second EOL after `y=height` must give `RleBounds`.
- `mask_outside_ok`: a BITFIELDS 16-bpp mask with a bit above bit 15 (for example green `0x0003e000`... any `m>>16 != 0`) must give `InvalidBitfields`.
Restore one vector each (RLE8 is enough for the first two). Only the absolute-mode overrun vectors remain for the row-bounds rule.

**N2. The new D2 and D4 rules have untested boundaries (should-fix).**
- D2 table size and location: `table_ps3` (3-byte instead of 4-byte table entries), `table_offset_unchecked`/`hi_used_ignore_table` (offset need not clear the table) survive, since the three `bmp-optimal-palette-*` vectors use `used=1` and an offset exactly at the table end. Add `bfOffBits = p+4*used-1` (must give `InvalidPixelOffset`) for BI_RGB, and one BITFIELDS 16-bpp H=40 BmpFile vector with masks then table (the `rgb16-565pal.bmp` layout; `table_before_masks` is only killed incidentally by the offset-inside-masks vectors).
- D2 and the 32-bit slice domain (`plen_wrap32` survives): `biClrUsed=0x40000000` at 24 bpp in `BmpFile` makes `4*used` wrap to 0 in a 32-bit `usize`; the correct result is `InvalidPixelOffset`. THEOREMS requires the proof on 32-bit targets, so this is the case an implementer will get wrong.
- D4 on BITFIELDS (`explicit_bf_always`, `explicit_bf_never`, `explicit_all32` survive): there is exactly one ICO BITFIELDS vector (`ico-fields-32`) and it does not discriminate. Add two 32-bpp ICO BITFIELDS vectors: no alpha mask with an AND bit set (AND must force alpha 0 even though the implied alpha 255 is nonzero), and an alpha mask with all alpha pixels zero (AND fallback must apply).

**N3. `fields-topdown-16/32` cannot detect a missing top-down flip (should-fix).** Both have two identical rows, so `td_bf_ignored` survives. Use two different pixels per vector.

**N4. L3 wording (nit).** THEOREMS says the first `2j` output pixels are red after `j` rows. RLE rows are painted bottom-up, so it is the last `2j` pixels in top-first output order (equivalently the first `2j` source-order pixels). The final result `red^(2k)` is right.

**N5. D1 leaves `bfSize` otherwise unconstrained (should-fix to document, not to change).** `bfSize` nonzero and `<=|b|` is accepted even if it is smaller than `bfOffBits` or the real pixel end (for example `bfSize=1`). That is what D1 says (`bfSize` is not a read bound) and `bmp-size-short` covers it, but R1 still lists "reject inconsistent metadata" as its rationale. Reword so the BmpFile exception is explicit. The `bfSize>|b|` rejection (kept) means `badfilesize.bmp` (complete data, stale huge `bfSize`) is still refused; that matches the owner's D1.

**N6-N8. Nits.** (6) `h16_height_unsigned` as above. (7) The sweep's equivalents (`swap3`, `h16_unsigned`, `img_xa_any_mode`) show the compression-vs-combo order and unsigned OS/2 width are unobservable; no action. (8) THEOREMS names 20 small witnesses as the proof gate and the rest as runtime tests, which is proportionate; state which 20 so the target is not ambiguous.

## D1/D3 interaction and precedence of the new rules

- **D3 subsumed by D1 coherently.** BmpFile ignores every byte after the complete pixel array: RLE data after the EOB (still `RleTruncated` if EOB is missing), raw data after the last row, and so V5 profile spans without parsing `bV5ProfileData`/`bV5ProfileSize`. `TrailingBytes` is unreachable in BmpFile and R15/R14 say so. `rgb24prof.bmp` and `rgb24lprof.bmp` are accepted. The cost is that a BMP with arbitrary appended junk is accepted, which is the owner's choice.
- **Unambiguous.** BareDib and ICO keep `cursor=|b|`. `biSizeImage` in BmpFile is still compared with consumed bytes X (excluding the suffix) and with `X+A` never accepted since `A=0`; `InvalidImageSize` still precedes everything after the pixel array. `bfSize>|b|` is checked immediately after `BadMagic`, before `bfOffBits`.
- **Total precedence.** R14 now reads: pixel errors, `InvalidImageSize`, `TruncatedAndMask` (AND span computed first, bytes not read until after the size check), `TrailingBytes` (BareDib/ICO). The D4 scan and the AND application happen after all errors are decided, so no new error site exists. My independently written oracle took the same order on all 200 vectors.
- **Remaining ambiguity risk, covered by N2:** whether the D4 "explicit alpha" test applies to BITFIELDS without an alpha mask and what counts as "nonzero BITFIELDS A mask" (the text is clear; the vectors do not enforce it).

## Corpus (58 accepted, 24 core rejections, 2 container-only)

Recomputed with my oracle: 58 accepted, 24 rejected, 2 outside core. Newly accepted versus draft 1: `rgb24pal`, `rgb24largepal`, `rgb16-565pal` (D2); `rgb24prof`, `rgb24lprof` (D1); `python.ico`, `deerstalker.cur`, `mask_1.cur`, `mask_L.cur` (B1; the `.cur` pair via the shim's read-to-EOF workaround). Remaining rejections, all still justified:

| Error | Files | Justification |
| --- | --- | --- |
| `InvalidColorCount` 2 | `badpalettesize` (used=305402420), `pal8oversizepal` (300 at 8 bpp) | indexed `biClrUsed>2^depth` |
| `InvalidDimensions` 2 | `badwidth` (-127), `decompression_bomb.ico` (65535x65535) | corrupt or over the 8192 cap |
| `InvalidFileSize` 5 | `badfilesize` (bfSize 2111692253, data complete), `l2rgb_read`, `mmap_error` (1 byte short of `bfSize`), `reallybig`, `shortfile` | `bfSize>|b|`; `badfilesize` is the one a relaxed rule would accept, owner kept it |
| `InvalidImageSize` 1 | `badbitssize` (img=2129587950, actual 1024) | garbage metadata |
| `InvalidPixelOffset` 1 | `pal8os2sp` | CORE with palette cut by `bfOffBits`, kept strict by owner |
| `InvalidPlanes` 1, `PaletteIndexOutOfRange` 1 | `badplanes` (30000), `pal8badindex` | corrupt |
| `RleBounds` 2 | `badrle`, `hopper_rle8_row_overflow` (255-run at x=127 of 128) | crafted overflows |
| `TrailingBytes` 2 | `hopper_mask.ico`, `pillow.ico` | slack inside ICO entries (D5, strict) |
| `Unsupported*` 6 | `rgba32abf` (BI_ALPHABITFIELDS), `badheadersize` (66), `rletopdown`, `badbitcount`, `pal2`, `rgb24jpeg`/`rgb24png` (depth 0) | policy or corrupt |

Per-error counts match DESIGN-NOTES. `shortfile.bmp` and `mmap_error.bmp` would be rejected by pixel truncation as well, so they stay rejected under any D1 relaxation. In the accepted ICO files D4 changes nothing observable (all 32-bpp entries carry nonzero alpha), so the corpus does not exercise the new rule.

## THEOREMS-DRAFT.md

- All **76** witness names are real vectors (checked by script); **31** mutations, every cited vector exists and the code changes for #23-31 are killed by their witnesses on my oracle.
- L1/L2 hold (`precedence-too-large-bmp-bad-magic` is a good instance: 67,108,865 bytes of BM-less data gives `FileTooLarge`, not `BadMagic`). L3 replacement is provable by induction on a 48+4j cursor invariant (N4 wording aside); the large vector's coverage is runtime plus symbolic header and cap arithmetic, which a Flash-class implementer can do.
- T3 is now a checkable allocation-call count; T12 is unchanged and `c=16` still holds with the D4 scan (ICO 32-bpp worst case: `4N` init + `N` XOR + `N` alpha scan + `N` AND + row loops, about `7N+2h <= 9N`); the `plen` product for D2 must be computed in `u64` (N2).
- Remaining proportionality risk is only the RLE state machine; the factoring into a packet parser and a <=255-pixel painter is the right shape.

## Files

- `/__modal/volumes/vo-Iyh3VLkOB7cEHrIp2RkTs7/projects/dib/REVIEW2.md`

## REVIEW2 addendum 2: draft 2b (214 vectors)

Counts (new findings): blocking 0, should-fix 0, nit 0.

Verdict: **ready to freeze** from the independent-review side, assuming the owner accepts the standing decisions D1-D6 and the unsupported items (`pal8os2sp`, `pal2`, `rgba32abf`) as recorded. Hashing the vectors and freezing the manifest can proceed.

- **Spec text:** the draft-2b edits (bfSize exception wording in R1, `u64` palette arithmetic in R7/T2) change no rule. My oracle needed no change.
- **Oracle:** `python3 -I check_vectors.py` passes 214/214. My independently written oracle has **0 disagreements on all 214 vectors** (64 MiB inputs expanded; the 8192x2048 vector decoded in full).
- **Mutation sweep:** the same 142 mutants on the 214 vectors leave 6 survivors, all false or equivalent: `out_ge` (killed by `output-exact-64m-rle8`, which the sweep excludes for speed; re-run separately), `swap3`, `h16_unsigned`, `img_xa_any_mode`, `img_only_xa`, and `hi_used_ico_ok` (a no-op). **All 12 previously real survivors are killed:** `run_clip`, `eol_nobounds`, `mask_outside_ok`, `td_bf_ignored`, `explicit_all32`, `explicit_bf_always`, `explicit_bf_never`, `hi_used_ignore_table`, `table_offset_unchecked`, `table_ps3`, `plen_wrap32`, `h16_height_unsigned`. Every other mutant killed in the first sweep is still killed.
- **Hand derivation of the new fixtures** (done from field slices, not from the oracle):
  - `bmp-optimal-table-u32-wrap`: 24 bpp, `biClrUsed=0x40000000`, `p=54`, `bfOffBits=54`, `|b|=58`. The table is 2^32 bytes, so `54 < 54+2^32` gives `InvalidPixelOffset`; a 32-bit wrap to 0 would accept and decode a red pixel. Correct.
  - `ico-fields-32-no-alpha-and-veto`: masks R `0xff0000`, G `0xff00`, B `0xff`, no A. Pixel word `0x00ff0000` gives red, alpha 255 from the absent mask, and AND bit `0x80` set forces alpha 0: `ff000000`. Correct.
  - `ico-fields-32-explicit-alpha-zero-and-fallback`: A mask `0xff000000`, both pixel alphas 0, AND byte `0x40`. Pixel 0 (bit clear) alpha 255, pixel 1 (bit set) alpha 0: `ff0000ff`, `00ff0000`. Correct.
  - `ico-fields-32-explicit-alpha-keeps-xor`: alpha `0x44` nonzero, AND bit set ignored: `ff000044`. Correct.
  - `fields-topdown-16/32` and `header-16-negative-height-topdown`: the first stored row is the top row; red then green (BITFIELDS 7c00/03e0, ARGB with alpha 0x33) and red then blue for the OS/2 16-byte header. Correct, and the rows now differ so a missing flip changes the output.
  - `bmp-fields-40-masks-then-optimal-table-gap`: masks end at 66, table of two entries ends at 74, `bfOffBits=76` accepts (two gap bytes), pixel `0xf800` is red. Its inside-table twin (`bfOffBits=73`) and the two BI_RGB table vectors (57 < 58; 61 < 62, which a 3-byte stride would accept at 60) give `InvalidPixelOffset`. Correct.
  - The five restored vectors (`rle4/8-run-over-row`, `rle4/8-eol-past-height`, `masks-outside`) are byte-identical in behaviour to draft 1 and are killed-sensitive as listed above.
- **Proof gate:** `THEOREMS-DRAFT.md` names exactly 20 vectors (checked by script), all real. The 76 broader witnesses are still real. The 35 mutations all reference real vectors, and the code changes for #32-#35 are each killed by their cited witness on my oracle. L3 now says the last `2j` top-first pixels are red, which is right.
- **Not verified by me:** the Microsoft documentation claims (still RECALLED; the network was unavailable) and any Verus proof (none exists yet).
