# DIB draft 2 changes — independent review response

This remains **DIB-CORE-SPEC 1.0 draft**, not frozen. Draft-one snapshots sit beside every revised deliverable (`*-draft1.md`, `VECTORS-draft1.json`, `check_vectors-draft1.py`, `run_corpus-draft1.py`). This file maps every numbered finding in `/__modal/volumes/vo-Iyh3VLkOB7cEHrIp2RkTs7/projects/dib/REVIEW.md`; no finding was silently dropped. At the initial draft-two point, `python3 -I spec/check_vectors.py` passed **200/200** revised vectors. Independent reviewer agreement at that point applied to **draft one only** (186 vectors); the second review subsequently checked all 200.

| Finding | Draft-two disposition |
| --- | --- |
| B1 | Accepted ICO `biSizeImage` values 0, XOR bytes, XOR+AND bytes; compute AND length before comparison; reject other values before mask availability. Added `ico-image-size-{xor-only,xor-and,bad}` and the image-size-versus-short-AND pair. Corpus now accepts `python.ico`, `deerstalker.cur`, and both cursor samples once the measurement shim handles their malformed directory lengths. |
| B2 | Added 1/4/8-bpp 3-row width 3/5/9/33 patterns; top-down indexed, top-down BITFIELDS, all BMP depths and both RLE modes/bitfield depths with an offset gap, 16-bit A4R4G4B4 alpha and mid-range mask scaling. Added second-offset/mask tests and explicit unused-bit/padding vectors. To keep the 200-vector ceiling, removed redundant mode×header combinations and single-error fixtures while retaining each of the eight header sizes and each depth. |
| S1 | Added simultaneous-defect precedence vectors: oversized BmpFile before bad magic; size before offset; basic offset before unsupported header; color/output before short external masks; raw truncation before image size; image size before short AND or suffix. Removed vacuous `compression-before-combo` and `mask-before-palette` vectors from the non-vacuity set; R15 states mutually exclusive error sites are not compared. |
| S2 | Added `bfSize>|b|`, second reserved word, offset at EOF, 17/18 basic offset with the 18-byte case reaching header validation, exact height 8192, CORE high-bit height, OS/2-short negative height, oversized `biSizeImage`, alpha/RGB mask overlap, ICO BITFIELDS, BI_RGB V4 nonzero ignored masks, nonzero absolute pad, zero delta, missing EOB after final EOL, unused odd RLE4 low nibble and unused raw bit. |
| S3 | Applied owner's D2: high-depth `biClrUsed` optimal tables only in BmpFile; skip `4*used` and require offset past the table. BareDib/ICO remain strict. Three high-depth corpus files now pass. |
| S4 | Applied owner's D1, which subsumes D3's exact profile-span acceptance without parsing V5 profile fields; BMP profile suffixes now pass, with no colour management. BareDib/ICO still reject suffixes. |
| S5 | T3 now states a checkable **one allocating call requesting exactly `4N`** on success, with an allocation-call ghost counter, guards before the call and no allocation in loops; no purported Verus proof of peak heap residence or stack bytes. Source review prohibits recursion and input-sized stack locals. |
| S6 | MR's error result retains exact error/cursor/x/y but not unobservable partial pixels. T6 similarly requires exact error/cursor on failure and full bytes only on success. |
| S7 | L3 is now a parameterised 2-pixel-wide row lemma for 1..16 rows, with `rle8-small-red-rows` as a concrete witness; 64 MiB output is a mandatory runtime vector with red **painted** on black initialization. Only its header and size arithmetic are symbolic; no 266 KiB repeated-sequence Verus obligation. T9's universal correctness statement remains unchanged. |
| S8 | Mutations 23–31 now target AND bit order, RLE4 nibble order, delta, early EOB, `bfOffBits` gap, mask contiguity, 555 high bit, ICO `biSizeImage=X+A`, and 32-bit ICO alpha/mask global selection. Existing mutation 10 now tests fallback rather than the newly legitimate ignore-AND-when-alpha-present case. |
| S9 | Corrected DESIGN-NOTES: `io-ico.c:366–388` skips BITFIELDS ICO before its converter; this core's support is a new behavior, not altered converter output. |
| S10 | Applied owner's D4: any nonzero 32-bpp XOR alpha makes AND irrelevant; an all-zero XOR alpha plane falls back to AND-derived binary alpha. Added mixed-alpha and zero-alpha vectors; documented gdk-pixbuf's unconditional 32-bit AND skip. |
| S11 | Kept exact-slice ICO behavior per D5; `hopper_mask.ico` and `pillow.ico` remain `TrailingBytes`. BMP's D1 suffix rule does not apply to ICO. |
| N12 | Measurement shim recognizes the two single-entry CUR fixtures that falsely declare eight bytes and measures from their directory offset to EOF; no production container policy follows from this workaround. |
| N13 | R15 identifies `TruncatedDibHeader`, `TruncatedPalette`, and `TrailingBytes` as unreachable in BmpFile, with the reason for each. |
| N14 | Fixed the directory-size comparison cite to `io-ico.c:421–428`; SOURCES labels `io-ico.c:1235–1254` as **writer** evidence, not loader behavior. |
| N15 | R8 explicitly applies R6's rounded component scaling to BI_RGB16 555; intermediate sample value three yields 25. |
| N16 | R3 marks unsigned OS/2 2.x fields RECALLED and states the decoder's signed interpretation as a strict policy. |
| N17 | R1 explains `bfOffBits>=18` only protects the four-byte size word; R7 checks the actual mask/table end. |
| N18 | DESIGN-NOTES explicitly states the final raw row's padding is required, with a compatibility cost for producers omitting it. |
| N19 | DESIGN-NOTES distinguishes strict delta destinations (`x<width,y<height`) from EOL's permitted `y=height`; retained per D6. |

The reviewer counted seven nits but numbered findings **12 through 19**, eight distinct items; all eight are mapped above. Owner decisions D1–D6 were applied: D1 and D2 relax BmpFile only; D3 is subsumed by D1 without V5 profile-field parsing; D4 uses the global XOR-alpha test; D5 and D6 remain strict. The owner also kept OS/2 sparse palettes (`pal8os2sp.bmp`), 2-bpp (`pal2.bmp`) and BI_ALPHABITFIELDS (`rgba32abf.bmp`) unsupported. Microsoft documentation fetches returned `ECONNRESET`; SOURCES now marks those claims RECALLED rather than claiming complete primary reading.

The same local Pillow corpus of **84 files** now yields **58 accepted, 24 core rejections, 2 container-only cases**; exact per-error counts are in DESIGN-NOTES. The corpus intentionally includes corrupt fixtures and does not estimate the population compatibility rate. An independent reviewer should rerun its oracle and mutation sweep before any freeze.

## Draft 2b — response to REVIEW2.md

The second independent review matched its separate oracle against **all 200 draft-two vectors** and all 82 decodable local corpus files, pixel for pixel. Its 142-point mutation sweep found no blocking defect, four should-fix findings and four nits. The previous 200-vector count was a drafting choice, **not** a cap: draft 2b restores five regressions and adds nine boundary fixtures. `spec/VECTORS.json` now has **214** vectors. The draft remains unfrozen; the reviewer has not yet independently evaluated the new 14 vectors or rerun its mutation sweep on them.

| REVIEW2 finding | Draft-2b disposition |
| --- | --- |
| N1 | Restored the unchanged draft-one `rle4/8-run-over-row`, `rle4/8-eol-past-height`, and `masks-outside` vectors. The encoded RLE8 overrun distinguishes a clipped run; a second EOL after `y=height` and a 16-bpp mask above bit 15 each reject. |
| N2 | Added one- and two-entry BI_RGB optimal-table offsets ending one byte inside the table, a 16-bpp H=40 BITFIELDS BMP with three external masks followed by two four-byte optimal-table entries and an offset gap, a corresponding inside-table rejection, and `biClrUsed=0x40000000` at 24 bpp, which must give `InvalidPixelOffset` instead of wrapping on a 32-bit target. R7 and T2 require `u64` for palette length and end before narrowing. Added ICO BITFIELDS without an A mask and AND=1, with an all-zero explicit A plane and mixed AND bits, and with nonzero explicit A and AND=1; these distinguish the no-mask veto, whole-image fallback and XOR-alpha preservation. |
| N3 | Changed `fields-topdown-16` and `fields-topdown-32` to distinct red/green rows; a missing top-down branch now reverses their expected output. These existing top-down BITFIELDS vectors cover B2's remaining orientation gap. |
| N4 | Corrected L3: after `j` bottom-up RLE rows, the **last** `2j` top-first output pixels are red. |
| N5 | Clarified R1: BmpFile accepts nonzero `bfSize<=|b|` even when that value lies before `bfOffBits` or the complete raster end; it still rejects `bfSize>|b|`, and no other metadata check changes. |
| N6 | Added an accepted OS/2 16-byte BareDib with negative stored height and two distinct top-down rows; `height-8193-os2short` retains the rejection boundary. |
| N7 | No change to format rules for the equivalent/no-op compression-order, unsigned-width and mode-specific image-size mutants identified by the reviewer. |
| N8 | Named the exact 20 small formal proof-gate vectors in THEOREMS-DRAFT.md; all 214 remain runtime conformance tests, with large repeats handled separately as specified. |

The local oracle passes **214/214** (`python3 -I spec/check_vectors.py`). The unchanged 84-file Pillow corpus remains **58 accepted, 24 core rejections, 2 container-only cases** (`python3 -I tools/run_corpus.py`). No verifier proof or post-2b independent oracle/mutation result is claimed. Do not freeze.
