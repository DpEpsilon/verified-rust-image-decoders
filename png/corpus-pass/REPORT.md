# M0-corpus: the real-corpus pass under the 2026-11-17 caps (PNG-Q7)

## Conclusions

Under the 2026-11-17 caps, Strict and RecommendedBeta ran to completion on all 21,966 files of the pinned real corpus and on 25 project-authored synthetic files: no timeout, crash or SHA-256 mismatch. RecommendedBeta rejects no file that Strict accepts, and obligation R holds with 0 violations over 21,401 Strict-accepted real stills and 22,555 Strict frames. No real file reaches a reservation above 464.9 MiB, and no capped input in the synthetic set reaches one above 576 MiB, so `MAX_BYTES` = 2^30 is never the binding limit. Counts are files. Tables T1 to T13 follow the method.

**Composition.** There are 8,296 real incidental assets (web/doc images 4,750, icons 2,109, wallpapers 1,210, screenshots 227; this split uses a path heuristic and no decision relies on it), 13,323 test files and 347 fuzz files. The hold-out (`HOLDOUT.tsv`, PNG-Q14) holds 5,059 of those files from 10 whole repositories: 3,403 incidental, 1,626 test and 30 fuzz. The other 16,907 files are the development set. The 25 synthetic files are separate and never counted as real: 9 multi-monitor screenshots, 4 cap-boundary images, 7 long thin images, 3 stored near-cap images and 2 files at the input-cap boundary. Every synthetic file's outcome matched the outcome written into its manifest before the run.

**Strict errors on real files (T2).** Strict rejects 565 real files: 179 incidental, 280 test and 106 fuzz. 44 of the 62 errors occur on at least one real file. On incidental assets only four errors occur: `TrailingBytes` 107, `ChunkOutOfOrder` 64 (every one an eXIf after IDAT), `BadSignature` 7 (files that are not PNGs) and `InvalidExif` 1. Over the whole corpus the most frequent are `TrailingBytes` 147, `TruncatedChunk` 133, `ChunkOutOfOrder` 99, `CrcMismatch` 40 and `BadSignature` 22. Eighteen errors occur on no real file, among them all the deflate code errors (`InvalidSymbol`, `InvalidCodeLengths`, `InvalidBlockType`), `NonConsecutiveIdat`, `UnknownCriticalChunk`, `PixelBudgetExceeded` and `IccProfileTooLarge`; the vectors still exercise them.

**RecommendedBeta fields (T5 still, T6 stream).** RecommendedBeta accepts 313 files that Strict rejects (172 incidental, 107 test, 34 fuzz; 22 of them in the hold-out). Every one of these flips is explained by the field reversions: 308 need a single field, 5 need two or three fields, and none is unexplained. A field "acts" on a file when reverting that field alone to its Strict value changes RecommendedBeta's outcome. Final outcomes per field on real files:
- `trailing`: acts on 147 (107 incidental), all accepted.
- `ancillary_order`: acts on 97 (64 incidental), all accepted.
- `ancillary_values`: acts on 26 (1 incidental); 14 accepted, 12 rejected later, 6 of them by `CrcMismatch`.
- `apng_in_still` (still) and `apng_errors=Static` (stream): act on 20 and 19 test files; 18 and 17 accepted (R4 on the 17 streams).
- `crc_ancillary=SkipExceptFdat`: acts on 13 test and fuzz files; 9 accepted.
- `ancillary_duplicate`: acts on 12; 11 accepted in the still view (12 in the stream view).
- `palette_index`: acts on 11 test files, all accepted with R3.
- `palette_size`: acts on 7; 1 accepted.
- `missing_iend=AtEof`: acts on 7 fuzz files; 5 accepted.
- `window` and `zlib_tail=IgnoreTail`: act on 2 test files each, all accepted.
- `plte_unexpected`, `trns_invalid`: act on 2 each; 1 accepted each.
- `iend=Lenient`: acts on no file.

Only `trailing`, `ancillary_order` and `ancillary_values` act on any incidental asset. The 252 files that RecommendedBeta still rejects contain no incidental asset except the 7 non-PNG files.

**Caps and `MAX_BYTES` (T8 to T10).** Raising the caps changed the outcome of 45 real files and of no other real file: 49 real files exceed a 2026-11-16 cap, and 4 of them keep their outcome. Of the 45, 5 changed only the error name, from `InvalidDimensions` to `OutputTooLarge`. File by file against the 2026-11-16 pass (T11), 20,497 of the 20,502 files that both passes ran have the same outcome and the same Strict RGBA. The 5 that changed are the test files with a side above 16,384 (65,535 × 1, 1 × 65,535, 32,768 × 100, 524,288 × 2, 2 × 524,288), which Strict now accepts; all of them have long been decoded by every incumbent. The oracle repairs of 2026-11-17 (D4, D5, D20) changed no real file. 1,464 files ran for the first time (the old pass's unsampled 1 to 16.7 Mpx files and the files over the old caps).
- **Output.** Strict now accepts 35 real files that the old caps rejected (30 over the 64 MiB output cap, 5 over the side cap); RecommendedBeta accepts 40, the extra 5 being `TrailingBytes` or `ChunkOutOfOrder` wallpapers. The largest accepted real image is 60.9 Mpx, 232.4 MiB as RGBA8 (an incidental asset in the hold-out). The largest in the development set is a 10,000 × 5,625 wallpaper, 214.6 MiB. Two real files exceed 128 MiB and none exceeds 256 MiB. Under the 256 MiB cap Strict rejects six real files with `OutputTooLarge`. Four of them declare more than 1 Gpx and are bombs or crashers. The other two are valid test images that the incumbents decode: `gdk-pixbuf tests/large.png` (839 Mpx) and the hold-out file `opencv_extra read.png` (181 Mpx). No permitted `MAX_OUTPUT` covers either one: the envelope's upper bound, 429,496,728 bytes, is 107 Mpx.
- **Side.** The longest accepted side is 524,288 pixels (2 × 524,288, test). Among incidental assets it is 16,384. Of 41 thin real files (aspect at least 32:1), 37 are accepted. The other four fail for non-cap reasons (`CrcMismatch` 2, `InvalidChunkType` 1) or for the intended `OutputTooLarge` of `malformed-size.apng` (524,288 × 2,048). The synthetic files show that the boundary sits where the caps put it: 67,108,864 × 1 and 1 × 67,108,864 RGBA8 are accepted and 1 × 67,108,865 is `OutputTooLarge`; 2^31 − 1 × 1 is `OutputTooLarge` and 2^31 × 1 is `InvalidDimensions`.
- **Input.** The largest real file is 44.2 MiB (46,397,113 bytes), so no real file comes near either input cap. Stored synthetic images within the output cap are 256.1 MiB (8,192 × 8,192 RGBA8), 512.1 MiB (RGBA16) and 576.1 MiB (1 × 2^26 RGBA16 Adam7). The old 256 MiB `MAX_INPUT` rejected all three with `FileTooLarge`; 2^30 accepts them. A file of exactly 2^30 bytes is accepted and one of 2^30 + 1 bytes is `FileTooLarge`.
- **Multi-monitor screenshots.** The real corpus has none; its 227 files with "screen" in the path are at most 4K. All 8 synthetic layouts up to 15,360 × 4,320 (4 × 2 4K panels, 253.1 MiB RGBA8) are accepted. The 23,040 × 4,320 layout (3 × 8K, 379.7 MiB) is `OutputTooLarge`. The synthetic screenshots repeat every 256 rows, so they test sizes, not compression ratios.
- **`MAX_BYTES`.** Among accepted real files the largest single reservations are `R` = 232.4 MiB and the RGBA16 output (`decode_still16`, 8N) = 464.9 MiB. Among synthetic files they are `R` = 576.0 MiB (1 × 2^26 RGBA16 Adam7, the worst shape in `LIMITS.md`) and the RGBA16 output = 512.0 MiB. No reservation exceeds 2^30. The live-heap sum `R + 8N` for that worst shape is 1,088 MiB, which matches `LIMITS.md`. It exceeds 2^30 as an aggregate, but `MAX_BYTES` bounds each reservation, not the sum.
- **Other caps.** The largest values among accepted real files are 128 frames (`MAX_FRAMES` 65,536), an fcTL pixel sum of 112.5 Mpx (about a tenth of `MAX_TOTAL_PIXELS` = 2^30) and an inflated iCCP of 1.4 MB (about a twelfth of `MAX_ICC` = 16 MiB). One test file declares 2,147,483,649 frames, and Strict rejects it with `TooManyFrames`, as intended. These three caps were not changed on 2026-11-17 and were not stressed by synthetic files.

**Count reconciliation (T13).** All six numbers recompute exactly from the files on disk. 748 is the number of records in `probes/results.json` (516 vector, 177 probe, 55 witness; 14 above 4 KiB stored by reference). The Strict bridge counts distinct non-padded inputs: 737 = 734 inline + 533 vectors − 530 shared; then +3 OVER corner vectors = 740; +198 profile vectors, of which only 27 are new byte strings, = 767; +6 cap-boundary vectors = 773; +14 review and council vectors, 11 of them new byte strings, = 784. The step from 740 to 767 adds 27 inputs, not 198, because most profile vectors repeat bytes the bridge already holds. PLAN.md's wording "767 after the 198 profile vectors" is correct about the vectors and silent on that.

## Recommendations for the Stage-2 items (recommendations only; nothing in spec-frozen, limits-draft, policy-draft, THEOREMS.md or ERRATA.md was edited)

These recommendations are for the owner and the council. The cap evidence is quoted for the development set, with hold-out files named where they would change a number. The hold-out was run through Strict and RecommendedBeta only, as on 2026-11-16, and not through any compatibility profile.

1. *Recommendation:* freeze `MAX_OUTPUT` = 256 MiB. It covers every accepted development file with 41.4 MiB of headroom (214.6 MiB at most), and every accepted real file including the hold-out (232.4 MiB). The two valid images it rejects (839 Mpx and 181 Mpx test files) are beyond the largest permitted value, so raising the cap within the envelope would gain nothing. A real multi-monitor screenshot beyond 4 × 2 4K (more than 67 Mpx) would still be rejected; no corpus file shows one.
2. *Recommendation:* freeze `MAX_DIMENSION` = 2^31 − 1. It admits 5 valid test files that 16,384 rejected, and changes no other outcome. Because `MAX_OUTPUT` bounds the area, the effective longest side is 2^26.
3. *Recommendation:* freeze `MAX_INPUT` = 2^30, confirming the coordinator's call of 2026-11-17. Real files need only 44.2 MiB, but stored images within the output cap need up to 576.1 MiB, which 256 MiB rejects.
4. *Recommendation:* the corpus asks for no change to `MAX_BYTES` = 2^30 (E16, already approved). The largest reservation is 576 MiB (synthetic) and 464.9 MiB (real).
5. *Recommendation:* freeze `MAX_FRAMES`, `MAX_TOTAL_PIXELS` and `MAX_ICC` at their draft values. The corpus maxima are 128 frames, 112.5 Mpx and 1.4 MB, and nothing argues for another value.
6. *Recommendation:* freeze RecommendedBeta's 15 fields as drafted (policy draft 0.2). No field rejects a Strict-accepted file, obligation R holds, and every field except `iend` and `apng_errors` (in the still view) acts on at least one real file. Small calls for the council:
   - `iend=Lenient` acts on no real file. Keeping it costs nothing measurable here, and the probes show every incumbent accepts a damaged IEND. The corpus neither supports nor contradicts it.
   - `missing_iend=AtEof` acts only on 7 fuzz files (5 accepted). It is already recorded as a design choice that `image` does not share.
   - `palette_size`, `plte_unexpected` and `trns_invalid` each lead to acceptance of one real file, and none is incidental.
   - The incidental-asset case rests on `trailing` and `ancillary_order`. Their incidental flips are concentrated: on 2026-11-16, 95 of the `trailing` flips came from one wallpaper repository and 51 of the `ancillary_order` flips from another.
7. The hold-out is still unspent for the compatibility profiles. GdkCompat and ImageCrateCompat have not been run on any file, so PNG-Q14's re-measurement can use `HOLDOUT.tsv` as it stands.

## Method

- **Corpus.** The pinned real corpus of 2026-11-16: `~/corpora/png/MANIFEST.tsv` (SHA-256 `f73984e8…62996`, checked on every run), 21,966 distinct files from 87 repository trees; every file's SHA-256 is re-checked against the manifest when it is read. Classes and the hold-out are those of `../../leniency/corpora/png_stats.py` and `holdout-png.tsv`, unchanged: *incidental* (asset or mixed repositories, path without a test word), *test*, *fuzz*. A path heuristic (`corpus_pass.incidental_sub`: icon, screenshot, wallpaper, other web/doc image) splits the incidental class further as a reading aid; no decision uses it. No file bytes are in this directory; files are named `repo:path` and by SHA-256.
- **Synthetic set.** The real corpus has no multi-monitor screenshot, no file above 64 MiB (the 2026-11-16 fetcher capped files at 64 MiB and dropped none) and no stored image near the output cap. `gen_synthetic.py` (standard library only, deterministic) writes 25 project-authored files to `~/corpora/png-synthetic/` (3.6 GiB, outside the repository because the largest is 1 GiB): nine multi-monitor screenshot layouts, four cap-boundary images, seven long thin images, three stored (uncompressed-deflate) images near the output cap and two files at the input-cap boundary. Licence: project-authored, MIT; `MANIFEST.tsv` there records origin, licence, SHA-256 and the outcome intended before any oracle run. Synthetic files have their own column and never enter the real counts. Their pixels are screenshot-like but repeat every 256 rows, so they compress far better than real screenshots (3.9 MB for 66 Mpx); the stored files carry the worst-case input size instead. A scaled-down variant of every generated file decodes under the oracle to exactly the pixels the generator intended (checked before the full-size run).
- **Oracles.** `../spec-frozen/png_oracle.py` (asserted byte-identical to `../spec-draft/png_oracle.py`, which `policy_oracle.py` imports) and `../policy-draft/policy_oracle.py`, under `python3 -I`, with the caps of `../limits-draft/limits.py` (the 2026-11-17 values). No oracle, specification or limits file was edited.
- **What ran.** `corpus_pass.py oracle` runs Strict and RecommendedBeta on every file at its own size (the 2026-11-16 pass sampled 1 in 8 files between 1 Mpx and 16.7 Mpx and ran files over the old caps only in a raised-cap side run): `decode_still`, plus `decode_frames` for files with APNG chunks; obligation R in the still, stream and prefix forms; single-field reversion of each of RecommendedBeta's 15 non-Strict fields on every Strict-rejected file at every size (the old pass ablated only flips, and files above 1 Mpx only for the fields matching Strict's first error), with pairs for flips that no single field explains. Each file whose header exceeds a 2026-11-16 cap (16,384 per side, 64 MiB RGBA8 output, 256 MiB input) is run a second time with the old caps patched into both oracle modules, so the effect of the raise is measured rather than inferred. The still, stream and ablation functions are imported from the 2026-11-16 script (`../../leniency/corpora/png_recommendedbeta_pass.py`), not copied.
- **Final outcome of a field.** Field *f* *acts* on a file when reverting *f* alone to its Strict value changes RecommendedBeta's outcome (acceptance, first error, or output). Its final outcome on that file is RecommendedBeta's own outcome: *accept*, or the error that still rejects the file. *Needed* means the file is a flip (Strict rejects, RecommendedBeta accepts) that becomes rejected when *f* is reverted.
- **Allocation sizes.** For every file the reservation sizes of SPEC P20 come from IHDR through the oracle's own `raw_size`: Raw `R`, RGBA8 output `4N`, RGBA16 output `8N`, the APNG canvas and PREVIOUS buffer `4N`; they are compared with the guard's `MAX_BYTES` = 2^30 (and the old 2^26). `acTL num_frames`, the fcTL pixel sum and the inflated iCCP length (standard-library zlib, statistics only) are scanned for the other three caps.
- **Incumbents.** The 2026-11-16 runner binaries lived under `/tmp` and are gone; their cached results (`~/corpora/png/pass-cache/incumbents*.jsonl`: libpng 1.6.48, gdk-pixbuf 2.42.12, png 0.18.1 via image 0.25.10, glycin) are joined to the new oracle results. Neither the runners nor their inputs changed, so the cached results stand; the synthetic files have no incumbent results.
- **Compatibility profiles were not run** on any file: PNG-Q14 defers GdkCompat and ImageCrateCompat until Strict and RecommendedBeta are proved, and the hold-out must stay unseen by their fitting.
- **Reproduce.** `python3 -I gen_synthetic.py`; `python3 -I corpus_pass.py oracle --set synthetic`; `python3 -I corpus_pass.py oracle --set real --shard 0/1 --workers 3` (the machine has two cores; the first run of 2026-11-19 used `--shard K/2 --workers 7`, whose two cache files are kept and read); `python3 -I corpus_pass.py oldfix --set synthetic`; `python3 -I corpus_pass.py report`; `python3 -I reconcile_counts.py --json counts.json`; `python3 -I render_report.py`. Each oracle phase resumes from its cache under `~/corpora/png/pass-cache-2026-11-19/`.
- **Resuming after an interruption.** The sandbox can stop without warning, so the oracle phase writes one JSON line per finished file to `~/corpora/png/pass-cache-2026-11-19/oracle.<set>.shard<K>of<N>.jsonl` and flushes it at once (the cache is derived from third-party bytes, so it lives beside the corpus, not here). On start, `corpus_pass.py oracle` reads every `oracle.<set>*.jsonl` in that directory, whatever shard wrote it, skips each file whose SHA-256 already has a line, and appends to its own shard file; a line cut off by a crash fails to parse and is ignored, so that file simply runs again. A restart therefore loses only the files that were in progress (at most one per worker; the largest real file took about half an hour). To resume, re-run the same command, wrapped in `timeout`, e.g. `timeout 9h python3 -I corpus_pass.py oracle --set real --shard 0/1 --workers 3 --timeout 3600`; when it prints `0 files this call`, run `report`, then `render_report.py`. `report` refuses to run while any file lacks a record unless given `--partial`.

## Files

- `gen_synthetic.py`, `corpus_pass.py`, `reconcile_counts.py`, `render_report.py`: the scripts (this report's prose is in `render_report.py`).
- `results.json`: machine-readable results (a summary, then one record per file: outcomes, acting and needed fields, events, old-cap outcome, `R`).
- `counts.json`: the count reconciliation.
- `HOLDOUT.tsv`: the hold-out list (PNG-Q14).

## Tables

<!-- tables:begin -->

Generated by `render_report.py` from `results.json` (2026-11-20). Real corpus manifest SHA-256 `f73984e8c5f30fb959d6fae697ad267be4da3431330714ba111e2fd9baf62996`; synthetic manifest `c80a0d52444578b0fb5429fe6fe6a118b9a95237856ba748add56d14272d593d`; `png_oracle.py` (spec-frozen) `5a6a4ce2cf56219b7f4a9a1ef77201fe61a694f16e80b7941482f3f1a6452d2c`; `policy_oracle.py` `6e012ea2150eb5208bd68fd6526f2baf3f18eff69204f55c343dde3d94699291`; `limits.py` `8a5f9b43b3b845b47e3e5693284cfd3c31d2927ca9228dc498c67b1ba42c7b08`.

### T1. Composition and coverage

| | incidental | test | fuzz | real | development | holdout | synthetic |
|---|---|---|---|---|---|---|---|
| files | 8296 | 13323 | 347 | 21966 | 16907 | 5059 | 25 |
| ran | 8296 | 13323 | 347 | 21966 | 16907 | 5059 | 25 |
| S_still | 8117 | 13043 | 241 | 21401 | 16403 | 4998 | 19 |
| RB_still | 8289 | 13150 | 275 | 21714 | 16694 | 5020 | 19 |
| S_stream | 8117 | 13041 | 241 | 21399 | 16401 | 4998 | 19 |
| RB_stream | 8289 | 13147 | 275 | 21711 | 16691 | 5020 | 19 |
| flip_still | 172 | 107 | 34 | 313 | 291 | 22 | 0 |
| flip_stream | 172 | 106 | 34 | 312 | 290 | 22 | 0 |
| rb_rejects_strict_accepts_still | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| rb_rejects_strict_accepts_stream | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

Incidental assets by kind (path heuristic, `corpus_pass.incidental_sub`): web/doc image 4750, icon 2109, wallpaper 1210, screenshot 227; of them in the hold-out: web/doc image 2295, icon 1070, screenshot 27, wallpaper 11.

Synthetic kinds: cap-boundary 4, input-cap 2, long-thin 7, screenshot-multimonitor 9, stored-near-cap 3. Coverage: {'ran': 21966} real, {'ran': 25} synthetic; failures: none; SHA-256 mismatches against the manifests: 0.

Hold-out (`HOLDOUT.tsv`, SHA-256 `a20541f166cadc5fc46ccb02b1ce498e84b02d535d8284c05f97b40bd3c39d39`): 5059 files, fuzz 30, incidental 3403, test 1626; repositories: `AFLplusplus/AFLplusplus`, `elenapan/dotfiles`, `github/docs`, `gitlab.gnome.org/GNOME/gnome-user-docs`, `godotengine/godot-docs`, `google/skia`, `kubernetes/website`, `mozilla/gecko-dev`, `opencv/opencv_extra`, `servo/servo`.

### T2. Strict first error, still (decode_still), files per error

| error | incidental | test | fuzz | real | development | holdout | synthetic |
|---|---|---|---|---|---|---|---|
| TrailingBytes | 107 | 40 | 0 | 147 | 140 | 7 | 0 |
| TruncatedChunk | 0 | 113 | 20 | 133 | 108 | 25 | 0 |
| ChunkOutOfOrder | 64 | 23 | 12 | 99 | 86 | 13 | 0 |
| CrcMismatch | 0 | 17 | 23 | 40 | 36 | 4 | 0 |
| BadSignature | 7 | 15 | 0 | 22 | 19 | 3 | 0 |
| DuplicateChunk | 0 | 3 | 10 | 13 | 13 | 0 | 0 |
| InvalidColorTypeDepth | 0 | 5 | 6 | 11 | 11 | 0 | 0 |
| PaletteIndexOutOfRange | 0 | 11 | 0 | 11 | 11 | 0 | 0 |
| InvalidText | 0 | 3 | 5 | 8 | 6 | 2 | 0 |
| SequenceError | 0 | 8 | 0 | 8 | 8 | 0 | 0 |
| InvalidPalette | 0 | 2 | 5 | 7 | 6 | 1 | 0 |
| MissingIhdr | 0 | 7 | 0 | 7 | 7 | 0 | 0 |
| OutputTooLarge | 0 | 5 | 1 | 6 | 5 | 1 | 4 |
| InvalidChunkType | 0 | 4 | 1 | 5 | 4 | 1 | 0 |
| InvalidDimensions | 0 | 4 | 1 | 5 | 4 | 1 | 1 |
| InvalidChunkLength | 0 | 0 | 4 | 4 | 4 | 0 | 0 |
| FrameCountMismatch | 0 | 3 | 0 | 3 | 3 | 0 | 0 |
| DistanceTooFar | 0 | 2 | 0 | 2 | 1 | 1 | 0 |
| InvalidAnimationControl | 0 | 2 | 0 | 2 | 2 | 0 | 0 |
| InvalidExif | 1 | 1 | 0 | 2 | 2 | 0 | 0 |
| InvalidGamma | 0 | 1 | 1 | 2 | 1 | 1 | 0 |
| InvalidSplt | 0 | 0 | 2 | 2 | 2 | 0 | 0 |
| InvalidTime | 0 | 0 | 2 | 2 | 2 | 0 | 0 |
| InvalidTransparency | 0 | 0 | 2 | 2 | 2 | 0 | 0 |
| MissingAnimationControl | 0 | 2 | 0 | 2 | 2 | 0 | 0 |
| MissingIend | 0 | 1 | 1 | 2 | 2 | 0 | 0 |
| TrailingZlibData | 0 | 2 | 0 | 2 | 2 | 0 | 0 |
| AdlerMismatch | 0 | 0 | 1 | 1 | 1 | 0 | 0 |
| ExcessImageData | 0 | 0 | 1 | 1 | 0 | 1 | 0 |
| InvalidBackground | 0 | 0 | 1 | 1 | 1 | 0 | 0 |
| InvalidFrameControl | 0 | 1 | 0 | 1 | 1 | 0 | 0 |
| InvalidHistogram | 0 | 0 | 1 | 1 | 1 | 0 | 0 |
| InvalidPhys | 0 | 0 | 1 | 1 | 1 | 0 | 0 |
| InvalidSbit | 0 | 0 | 1 | 1 | 1 | 0 | 0 |
| MissingFrameData | 0 | 1 | 0 | 1 | 1 | 0 | 0 |
| MissingImageData | 0 | 1 | 0 | 1 | 1 | 0 | 0 |
| MissingPalette | 0 | 0 | 1 | 1 | 1 | 0 | 0 |
| TooManyFrames | 0 | 1 | 0 | 1 | 1 | 0 | 0 |
| TruncatedImageData | 0 | 0 | 1 | 1 | 1 | 0 | 0 |
| TruncatedZlib | 0 | 1 | 0 | 1 | 1 | 0 | 0 |
| UnexpectedFrameData | 0 | 1 | 0 | 1 | 1 | 0 | 0 |
| UnexpectedPalette | 0 | 0 | 1 | 1 | 1 | 0 | 0 |
| UnsupportedFilterMethod | 0 | 0 | 1 | 1 | 1 | 0 | 0 |
| FileTooLarge | 0 | 0 | 0 | 0 | 0 | 0 | 1 |

Errors no file hits (18 of 62): ChunkTooLong, IccProfileTooLarge, InvalidBlockType, InvalidCicp, InvalidCodeLengths, InvalidFilterType, InvalidIccp, InvalidSrgb, InvalidSymbol, InvalidZlibHeader, NonConsecutiveIdat, PixelBudgetExceeded, StoredLengthMismatch, UnknownCriticalChunk, UnsupportedCompressionMethod, UnsupportedInterlaceMethod, UnsupportedZlibMethod, ZlibPresetDictionary.

### T3. Strict first error, whole stream (decode_frames to IEND); differs from T2 only on APNG files

| error | incidental | test | fuzz | real | development | holdout | synthetic |
|---|---|---|---|---|---|---|---|
| TrailingBytes | 107 | 40 | 0 | 147 | 140 | 7 | 0 |
| TruncatedChunk | 0 | 113 | 20 | 133 | 108 | 25 | 0 |
| ChunkOutOfOrder | 64 | 23 | 12 | 99 | 86 | 13 | 0 |
| CrcMismatch | 0 | 17 | 23 | 40 | 36 | 4 | 0 |
| BadSignature | 7 | 15 | 0 | 22 | 19 | 3 | 0 |
| DuplicateChunk | 0 | 3 | 10 | 13 | 13 | 0 | 0 |
| InvalidColorTypeDepth | 0 | 5 | 6 | 11 | 11 | 0 | 0 |
| PaletteIndexOutOfRange | 0 | 11 | 0 | 11 | 11 | 0 | 0 |
| InvalidText | 0 | 3 | 5 | 8 | 6 | 2 | 0 |
| SequenceError | 0 | 8 | 0 | 8 | 8 | 0 | 0 |
| InvalidPalette | 0 | 2 | 5 | 7 | 6 | 1 | 0 |
| MissingIhdr | 0 | 7 | 0 | 7 | 7 | 0 | 0 |
| OutputTooLarge | 0 | 5 | 1 | 6 | 5 | 1 | 4 |
| InvalidChunkType | 0 | 4 | 1 | 5 | 4 | 1 | 0 |
| InvalidDimensions | 0 | 4 | 1 | 5 | 4 | 1 | 1 |
| InvalidChunkLength | 0 | 0 | 4 | 4 | 4 | 0 | 0 |
| FrameCountMismatch | 0 | 3 | 0 | 3 | 3 | 0 | 0 |
| DistanceTooFar | 0 | 2 | 0 | 2 | 1 | 1 | 0 |
| ExcessImageData | 0 | 1 | 1 | 2 | 1 | 1 | 0 |
| InvalidAnimationControl | 0 | 2 | 0 | 2 | 2 | 0 | 0 |
| InvalidExif | 1 | 1 | 0 | 2 | 2 | 0 | 0 |
| InvalidGamma | 0 | 1 | 1 | 2 | 1 | 1 | 0 |
| InvalidSplt | 0 | 0 | 2 | 2 | 2 | 0 | 0 |
| InvalidTime | 0 | 0 | 2 | 2 | 2 | 0 | 0 |
| InvalidTransparency | 0 | 0 | 2 | 2 | 2 | 0 | 0 |
| MissingAnimationControl | 0 | 2 | 0 | 2 | 2 | 0 | 0 |
| MissingIend | 0 | 1 | 1 | 2 | 2 | 0 | 0 |
| TrailingZlibData | 0 | 2 | 0 | 2 | 2 | 0 | 0 |
| TruncatedImageData | 0 | 1 | 1 | 2 | 2 | 0 | 0 |
| AdlerMismatch | 0 | 0 | 1 | 1 | 1 | 0 | 0 |
| InvalidBackground | 0 | 0 | 1 | 1 | 1 | 0 | 0 |
| InvalidFrameControl | 0 | 1 | 0 | 1 | 1 | 0 | 0 |
| InvalidHistogram | 0 | 0 | 1 | 1 | 1 | 0 | 0 |
| InvalidPhys | 0 | 0 | 1 | 1 | 1 | 0 | 0 |
| InvalidSbit | 0 | 0 | 1 | 1 | 1 | 0 | 0 |
| MissingFrameData | 0 | 1 | 0 | 1 | 1 | 0 | 0 |
| MissingImageData | 0 | 1 | 0 | 1 | 1 | 0 | 0 |
| MissingPalette | 0 | 0 | 1 | 1 | 1 | 0 | 0 |
| TooManyFrames | 0 | 1 | 0 | 1 | 1 | 0 | 0 |
| TruncatedZlib | 0 | 1 | 0 | 1 | 1 | 0 | 0 |
| UnexpectedFrameData | 0 | 1 | 0 | 1 | 1 | 0 | 0 |
| UnexpectedPalette | 0 | 0 | 1 | 1 | 1 | 0 | 0 |
| UnsupportedFilterMethod | 0 | 0 | 1 | 1 | 1 | 0 | 0 |
| FileTooLarge | 0 | 0 | 0 | 0 | 0 | 0 | 1 |

Errors no file hits (18 of 62): ChunkTooLong, IccProfileTooLarge, InvalidBlockType, InvalidCicp, InvalidCodeLengths, InvalidFilterType, InvalidIccp, InvalidSrgb, InvalidSymbol, InvalidZlibHeader, NonConsecutiveIdat, PixelBudgetExceeded, StoredLengthMismatch, UnknownCriticalChunk, UnsupportedCompressionMethod, UnsupportedInterlaceMethod, UnsupportedZlibMethod, ZlibPresetDictionary.

### T4. RecommendedBeta first error, still

| error | incidental | test | fuzz | real | development | holdout | synthetic |
|---|---|---|---|---|---|---|---|
| TruncatedChunk | 0 | 114 | 22 | 136 | 110 | 26 | 0 |
| CrcMismatch | 0 | 10 | 28 | 38 | 35 | 3 | 0 |
| BadSignature | 7 | 15 | 0 | 22 | 19 | 3 | 0 |
| InvalidColorTypeDepth | 0 | 5 | 6 | 11 | 11 | 0 | 0 |
| MissingIhdr | 0 | 7 | 0 | 7 | 7 | 0 | 0 |
| InvalidChunkType | 0 | 5 | 1 | 6 | 4 | 2 | 0 |
| OutputTooLarge | 0 | 5 | 1 | 6 | 5 | 1 | 4 |
| ExcessImageData | 0 | 2 | 3 | 5 | 2 | 3 | 0 |
| InvalidDimensions | 0 | 4 | 1 | 5 | 4 | 1 | 1 |
| InvalidPalette | 0 | 0 | 3 | 3 | 3 | 0 | 0 |
| MissingImageData | 0 | 2 | 0 | 2 | 2 | 0 | 0 |
| TruncatedImageData | 0 | 1 | 1 | 2 | 2 | 0 | 0 |
| TruncatedZlib | 0 | 1 | 1 | 2 | 2 | 0 | 0 |
| AdlerMismatch | 0 | 0 | 1 | 1 | 1 | 0 | 0 |
| DuplicateChunk | 0 | 0 | 1 | 1 | 1 | 0 | 0 |
| MissingIend | 0 | 1 | 0 | 1 | 1 | 0 | 0 |
| MissingPalette | 0 | 0 | 1 | 1 | 1 | 0 | 0 |
| NonConsecutiveIdat | 0 | 1 | 0 | 1 | 1 | 0 | 0 |
| UnknownCriticalChunk | 0 | 0 | 1 | 1 | 1 | 0 | 0 |
| UnsupportedFilterMethod | 0 | 0 | 1 | 1 | 1 | 0 | 0 |
| FileTooLarge | 0 | 0 | 0 | 0 | 0 | 0 | 1 |

Errors no file hits (41 of 62): ChunkOutOfOrder, ChunkTooLong, DistanceTooFar, FrameCountMismatch, IccProfileTooLarge, InvalidAnimationControl, InvalidBackground, InvalidBlockType, InvalidChunkLength, InvalidCicp, InvalidCodeLengths, InvalidExif, InvalidFilterType, InvalidFrameControl, InvalidGamma, InvalidHistogram, InvalidIccp, InvalidPhys, InvalidSbit, InvalidSplt, InvalidSrgb, InvalidSymbol, InvalidText, InvalidTime, InvalidTransparency, InvalidZlibHeader, MissingAnimationControl, MissingFrameData, PaletteIndexOutOfRange, PixelBudgetExceeded, SequenceError, StoredLengthMismatch, TooManyFrames, TrailingBytes, TrailingZlibData, UnexpectedFrameData, UnexpectedPalette, UnsupportedCompressionMethod, UnsupportedInterlaceMethod, UnsupportedZlibMethod, ZlibPresetDictionary.

### T5. RecommendedBeta fields, final outcome (still)

"acts" = files on which reverting the field alone to Strict changes RecommendedBeta's outcome; "final accept" / "final reject" = RecommendedBeta's own outcome on those files; "needed" = flip files (Strict rejects, RecommendedBeta accepts) that become rejected when the field alone is reverted (or, marked, only with a second field). Cells are real files (incidental / test / fuzz, hold-out) and synthetic files.

| field | acts: inc / test / fuzz (hold-out) | final accept (inc) | final reject: errors | needed: inc / test / fuzz (hold-out) | needed only as pair | revert gives | synthetic acts | example |
|---|---|---|---|---|---|---|---|---|
| `ancillary_duplicate` | 0 / 2 / 10 (0) | 11 (0) | 1: NonConsecutiveIdat 1 | 0 / 2 / 10 (0) | 1 | DuplicateChunk 12 | 0 | `image-rs/image-png:tests/bugfixes/issue#1825.png` |
| `ancillary_order` | 64 / 21 / 12 (13) | 97 (64) | 0 | 64 / 21 / 12 (13) | 0 | ChunkOutOfOrder 97 | 0 | `AbegAshford/rice-walls:Aesthetic/04)Cold Star Unfolding.png` |
| `ancillary_values` | 1 / 5 / 20 (3) | 14 (1) | 12: CrcMismatch 6, ExcessImageData 2, InvalidChunkType 1, TruncatedChunk 1, TruncatedZlib 1, UnknownCriticalChunk 1 | 1 / 3 / 10 (1) | 0 | InvalidBackground 1, InvalidChunkLength 5, InvalidExif 2, InvalidGamma 2, InvalidHistogram 1, InvalidPhys 1, InvalidSbit 1, InvalidSplt 2, InvalidText 9, InvalidTime 2 | 0 | `dharmx/walls:apeiros/looking_up_at_a_building.png` |
| `apng_errors` | 0 / 0 / 0 (0) | 0 (0) | 0 | 0 / 0 / 0 (0) | 0 |  | 0 | `` |
| `apng_in_still` | 0 / 20 / 0 (0) | 18 (0) | 2: MissingImageData 1, TruncatedImageData 1 | 0 / 19 / 0 (0) | 1 | ChunkOutOfOrder 1, FrameCountMismatch 3, InvalidAnimationControl 2, InvalidFrameControl 1, MissingAnimationControl 2, MissingFrameData 1, SequenceError 8, TooManyFrames 1, UnexpectedFrameData 1 | 0 | `onevcat/APNGKit:Tests/APNGKitTests/Resources/SpecTesting/049.png` |
| `crc_ancillary` | 0 / 7 / 6 (1) | 9 (0) | 4: ExcessImageData 1, TruncatedChunk 2, TruncatedZlib 1 | 0 / 6 / 3 (0) | 0 | CrcMismatch 13 | 0 | `google/skia:resources/invalid_images/bad_palette.png` |
| `iend` | 0 / 0 / 0 (0) | 0 (0) | 0 | 0 / 0 / 0 (0) | 0 |  | 0 | `` |
| `missing_iend` | 0 / 0 / 7 (0) | 5 (0) | 2: ExcessImageData 1, TruncatedZlib 1 | 0 / 0 / 5 (0) | 0 | MissingIend 7 | 0 | `strongcourage/fuzzing-corpus:png/mozilla/94e1bdbb03c42581d8407602634636ea.png` |
| `palette_index` | 0 / 11 / 0 (0) | 11 (0) | 0 | 0 / 11 / 0 (0) | 0 | PaletteIndexOutOfRange 11 | 0 | `image-rs/image-png:tests/bugfixes/invalid_palette_index.png` |
| `palette_size` | 0 / 3 / 4 (2) | 1 (0) | 6: CrcMismatch 4, ExcessImageData 2 | 0 / 1 / 0 (0) | 0 | InvalidPalette 7 | 0 | `google/skia:resources/images/crbug445556737.png` |
| `plte_unexpected` | 0 / 1 / 1 (0) | 1 (0) | 1: ExcessImageData 1 | 0 / 1 / 0 (0) | 0 | ChunkOutOfOrder 1, UnexpectedPalette 1 | 0 | `AOMediaCodec/libavif:tests/data/circle-trns-before-plte.png` |
| `trailing` | 107 / 40 / 0 (7) | 147 (107) | 0 | 107 / 40 / 0 (7) | 0 | TrailingBytes 147 | 0 | `godotengine/godot-docs:tutorials/assets_pipeline/img/gltf_godot_export.png` |
| `trns_invalid` | 0 / 0 / 2 (0) | 1 (0) | 1: CrcMismatch 1 | 0 / 0 / 1 (0) | 0 | InvalidTransparency 2 | 0 | `strongcourage/fuzzing-corpus:png/mozilla/4f14b7aab3a41855378c5517342598b9.png` |
| `window` | 0 / 2 / 0 (1) | 2 (0) | 0 | 0 / 2 / 0 (1) | 0 | DistanceTooFar 2 | 0 | `google/skia:resources/images/crbug807324.png` |
| `zlib_tail` | 0 / 2 / 0 (0) | 2 (0) | 0 | 0 / 2 / 0 (0) | 0 | TrailingZlibData 2 | 0 | `python-pillow/Pillow:Tests/images/hopper_idat_after_image_end.png` |

Flips (still): incidental 172, test 107, fuzz 34, real 313, development 291, holdout 22, synthetic 0; files by number of needed fields {'1': 308, '2': 3, '3': 2}; unexplained 0; pair-only [['python-pillow/Pillow:Tests/images/apng/chunk_multi_actl.png', ['ancillary_duplicate+apng_in_still']]].

### T6. RecommendedBeta fields, final outcome (stream)

"acts" = files on which reverting the field alone to Strict changes RecommendedBeta's outcome; "final accept" / "final reject" = RecommendedBeta's own outcome on those files; "needed" = flip files (Strict rejects, RecommendedBeta accepts) that become rejected when the field alone is reverted (or, marked, only with a second field). Cells are real files (incidental / test / fuzz, hold-out) and synthetic files.

| field | acts: inc / test / fuzz (hold-out) | final accept (inc) | final reject: errors | needed: inc / test / fuzz (hold-out) | needed only as pair | revert gives | synthetic acts | example |
|---|---|---|---|---|---|---|---|---|
| `ancillary_duplicate` | 0 / 3 / 10 (0) | 12 (0) | 1: NonConsecutiveIdat 1 | 0 / 2 / 10 (0) | 0 | DIFFERS 1, DuplicateChunk 12 | 0 | `image-rs/image-png:tests/bugfixes/issue#1825.png` |
| `ancillary_order` | 64 / 21 / 12 (13) | 97 (64) | 0 | 64 / 21 / 12 (13) | 0 | ChunkOutOfOrder 97 | 0 | `AbegAshford/rice-walls:Aesthetic/04)Cold Star Unfolding.png` |
| `ancillary_values` | 1 / 5 / 20 (3) | 14 (1) | 12: CrcMismatch 6, ExcessImageData 2, InvalidChunkType 1, TruncatedChunk 1, TruncatedZlib 1, UnknownCriticalChunk 1 | 1 / 3 / 10 (1) | 0 | InvalidBackground 1, InvalidChunkLength 5, InvalidExif 2, InvalidGamma 2, InvalidHistogram 1, InvalidPhys 1, InvalidSbit 1, InvalidSplt 2, InvalidText 9, InvalidTime 2 | 0 | `dharmx/walls:apeiros/looking_up_at_a_building.png` |
| `apng_errors` | 0 / 19 / 0 (0) | 17 (0) | 2: MissingImageData 1, TruncatedImageData 1 | 0 / 17 / 0 (0) | 0 | ChunkOutOfOrder 1, FrameCountMismatch 3, InvalidAnimationControl 2, InvalidFrameControl 1, MissingAnimationControl 2, MissingFrameData 1, SequenceError 8, UnexpectedFrameData 1 | 0 | `onevcat/APNGKit:Tests/APNGKitTests/Resources/SpecTesting/049.png` |
| `apng_in_still` | 0 / 0 / 0 (0) | 0 (0) | 0 | 0 / 0 / 0 (0) | 0 |  | 0 | `` |
| `crc_ancillary` | 0 / 7 / 6 (1) | 9 (0) | 4: ExcessImageData 1, TruncatedChunk 2, TruncatedZlib 1 | 0 / 6 / 3 (0) | 0 | CrcMismatch 13 | 0 | `google/skia:resources/invalid_images/bad_palette.png` |
| `iend` | 0 / 0 / 0 (0) | 0 (0) | 0 | 0 / 0 / 0 (0) | 0 |  | 0 | `` |
| `missing_iend` | 0 / 0 / 7 (0) | 5 (0) | 2: ExcessImageData 1, TruncatedZlib 1 | 0 / 0 / 5 (0) | 0 | MissingIend 7 | 0 | `strongcourage/fuzzing-corpus:png/mozilla/94e1bdbb03c42581d8407602634636ea.png` |
| `palette_index` | 0 / 11 / 0 (0) | 11 (0) | 0 | 0 / 11 / 0 (0) | 0 | PaletteIndexOutOfRange 11 | 0 | `image-rs/image-png:tests/bugfixes/invalid_palette_index.png` |
| `palette_size` | 0 / 3 / 4 (2) | 1 (0) | 6: CrcMismatch 4, ExcessImageData 2 | 0 / 1 / 0 (0) | 0 | InvalidPalette 7 | 0 | `google/skia:resources/images/crbug445556737.png` |
| `plte_unexpected` | 0 / 1 / 1 (0) | 1 (0) | 1: ExcessImageData 1 | 0 / 1 / 0 (0) | 0 | ChunkOutOfOrder 1, UnexpectedPalette 1 | 0 | `AOMediaCodec/libavif:tests/data/circle-trns-before-plte.png` |
| `trailing` | 107 / 40 / 0 (7) | 147 (107) | 0 | 107 / 40 / 0 (7) | 0 | TrailingBytes 147 | 0 | `godotengine/godot-docs:tutorials/assets_pipeline/img/gltf_godot_export.png` |
| `trns_invalid` | 0 / 0 / 2 (0) | 1 (0) | 1: CrcMismatch 1 | 0 / 0 / 1 (0) | 0 | InvalidTransparency 2 | 0 | `strongcourage/fuzzing-corpus:png/mozilla/4f14b7aab3a41855378c5517342598b9.png` |
| `window` | 0 / 2 / 0 (1) | 2 (0) | 0 | 0 / 2 / 0 (1) | 0 | DistanceTooFar 2 | 0 | `google/skia:resources/images/crbug807324.png` |
| `zlib_tail` | 0 / 2 / 0 (0) | 2 (0) | 0 | 0 / 2 / 0 (0) | 0 | TrailingZlibData 2 | 0 | `python-pillow/Pillow:Tests/images/hopper_idat_after_image_end.png` |

Flips (stream): incidental 172, test 106, fuzz 34, real 312, development 290, holdout 22, synthetic 0; files by number of needed fields {'1': 307, '2': 3, '3': 2}; unexplained 0; pair-only [].

### T7. Obligation R and recovered events

Strict-accepted stills 21401 real + 19 synthetic; streams 21399 + 19; Strict frames checked (prefix form) 22555; **violations: 0**.

| entry | event | incidental | test | fuzz | real | development | holdout | synthetic |
|---|---|---|---|---|---|---|---|---|
| still | R1 | 0 | 0 | 3 | 3 | 3 | 0 | 0 |
| still | R3 | 0 | 11 | 0 | 11 | 11 | 0 | 0 |
| stream | R1 | 0 | 0 | 3 | 3 | 3 | 0 | 0 |
| stream | R3 | 0 | 11 | 0 | 11 | 11 | 0 | 0 |
| stream | R4 | 0 | 17 | 0 | 17 | 17 | 0 | 0 |

### T8. Caps: every file whose header exceeds a 2026-11-16 cap (16,384 per side, 64 MiB RGBA8 output, 256 MiB input)

"old" = the oracle re-run with the old caps patched in (Strict / RecommendedBeta); "now" = under the current caps. R = Raw (filtered scanlines) reservation.

| file | set / class | dims | ct/depth/il | size | R | old Strict / RB | now Strict / RB | oracle s |
|---|---|---|---|---|---|---|---|---|
| `image-rs/image-png:tests/bugfixes/x_issue#230.1.png` | real / test | 4294967295 x 536883232 | 2/2/0 | 0.0 MiB | - | InvalidDimensions / InvalidDimensions | InvalidDimensions / InvalidDimensions | 0.0 |
| `gitlab.gnome.org/GNOME/gdk-pixbuf:tests/test-images/fail/invalid.4.png` | real / test | 538968589 x 538968673 | 2/8/32 | 0.3 MiB | - | TruncatedChunk / TruncatedChunk | TruncatedChunk / TruncatedChunk | 0.0 |
| `gitlab.gnome.org/GNOME/gdk-pixbuf:tests/test-images/fail/bug793470-crasher.png` | real / fuzz | 935820 x 854554 | 0/8/0 | 0.0 MiB | 762,662.5 MiB | InvalidDimensions / InvalidDimensions | OutputTooLarge / OutputTooLarge | 0.0 |
| `lovell/sharp:test/fixtures/65536-uint32-limit.png` | real / test | 65536 x 65536 | 3/1/0 | 0.5 MiB | 512.1 MiB | InvalidDimensions / InvalidDimensions | OutputTooLarge / OutputTooLarge | 0.0 |
| `OpenImageIO/oiio:testsuite/png-bomb/src/bomb.png` | real / test | 46340 x 46340 | 2/8/0 | 0.0 MiB | 6,143.8 MiB | InvalidDimensions / InvalidDimensions | OutputTooLarge / OutputTooLarge | 0.0 |
| `onevcat/APNGKit:Tests/APNGKitTests/Resources/General/malformed-size.apng` | real / test | 524288 x 2048 | 2/8/0 | 0.1 MiB | 3,072.0 MiB | InvalidDimensions / InvalidDimensions | OutputTooLarge / OutputTooLarge | 0.0 |
| `gitlab.gnome.org/GNOME/gdk-pixbuf:tests/large.png` | real / test | 25200 x 33300 | 2/8/0 | 2.4 MiB | 2,400.9 MiB | InvalidDimensions / InvalidDimensions | OutputTooLarge / OutputTooLarge | 0.0 |
| `opencv/opencv_extra:testdata/highgui/readwrite/read.png` | real / test (hold-out) | 13043 x 13917 | 6/8/0 | 0.7 MiB | 692.5 MiB | OutputTooLarge / OutputTooLarge | OutputTooLarge / OutputTooLarge | 0.0 |
| `mozilla/gecko-dev:third_party/python/aiohttp/vendor/llhttp/images/http-strict-none.png` | real / incidental (hold-out) | 6868 x 8872 | 6/8/0 | 4.0 MiB | 232.4 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 438.37 |
| `neonvoidx/pics:ultrawide/wallhaven-yq8yex.png` | real / incidental | 10000 x 5625 | 3/1/0 | 1.2 MiB | 6.7 MiB | OutputTooLarge / OutputTooLarge | TrailingBytes / accept | 993.57 |
| `dharmx/walls:nord/a_blue_and_grey_background.png` | real / incidental | 7680 x 4320 | 2/8/0 | 9.7 MiB | 94.9 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 245.52 |
| `dharmx/walls:nord/a_cartoon_of_a_couple_of_tentacles.png` | real / incidental | 7680 x 4320 | 6/8/0 | 0.2 MiB | 126.6 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 225.46 |
| `dharmx/walls:nord/a_pattern_of_circles_and_lines.png` | real / incidental | 7680 x 4320 | 2/8/0 | 6.7 MiB | 94.9 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 233.22 |
| `dharmx/walls:nord/nord4.png` | real / incidental | 7680 x 4320 | 2/8/0 | 22.5 MiB | 94.9 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 322.93 |
| `k1ng440/Wallpapers:wallpapers/099.png` | real / incidental | 7680 x 4320 | 6/8/0 | 10.7 MiB | 126.6 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 276.65 |
| `k1ng440/Wallpapers:wallpapers/101.png` | real / incidental | 7680 x 4320 | 6/8/0 | 14.6 MiB | 126.6 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 307.78 |
| `dharmx/walls:centered/a_palm_tree_and_cactus_on_a_pink_background.png` | real / incidental | 7424 x 4176 | 6/8/0 | 1.3 MiB | 118.3 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 215.74 |
| `dharmx/walls:abstract/a_group_of_buildings_with_blue_awnings.png` | real / incidental | 7108 x 3998 | 6/8/0 | 3.2 MiB | 108.4 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 223.93 |
| `neonvoidx/pics:ultrawide/radahn.png` | real / incidental | 7680 x 3268 | 6/8/0 | 17.1 MiB | 95.7 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 289.57 |
| `ruffle-rs/ruffle:tests/tests/swfs/avm2/large_preload_image_from_bytes/5000x5000.png` | real / test | 5000 x 5000 | 3/4/0 | 0.0 MiB | 11.9 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 63.88 |
| `dharmx/walls:gruvbox/27r8pdbrev0a1.png` | real / incidental | 6000 x 4000 | 2/8/0 | 20.8 MiB | 68.7 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 297.84 |
| `dharmx/walls:nord/a_close_up_of_a_rose.png` | real / incidental | 6000 x 4000 | 2/8/0 | 0.4 MiB | 68.7 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 148.28 |
| `dharmx/walls:nord/a_group_of_pink_flowers.png` | real / incidental | 6000 x 4000 | 2/8/0 | 2.7 MiB | 68.7 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 154.82 |
| `dharmx/walls:nord/a_group_of_pink_flowers_02.png` | real / incidental | 6000 x 4000 | 2/8/0 | 1.8 MiB | 68.7 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 156.93 |
| `neonvoidx/pics:vertical/abstractacid.png` | real / incidental | 4000 x 6000 | 3/8/0 | 15.0 MiB | 22.9 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 158.49 |
| `dharmx/walls:nord/a_red_sports_car_parked_in_a_garage.png` | real / incidental | 6361 x 3578 | 2/8/0 | 7.4 MiB | 65.1 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 173.18 |
| `dharmx/walls:outrun/a_sports_car_with_neon_lights.png` | real / incidental | 6058 x 3408 | 2/8/0 | 9.4 MiB | 59.1 MiB | OutputTooLarge / OutputTooLarge | ChunkOutOfOrder / accept | 1298.89 |
| `dharmx/walls:manga/a_cartoon_of_a_girl_with_short_hair.png` | real / incidental | 5990 x 3369 | 6/8/0 | 5.5 MiB | 77.0 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 180.25 |
| `dharmx/walls:nord/a_group_of_pink_flowers_01.png` | real / incidental | 5472 x 3648 | 2/8/0 | 1.5 MiB | 57.1 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 121.87 |
| `markpendlebury/ultrawide-wallpapers:6880x2880/Black_Myth_Wukong_6880x2880.png` | real / incidental | 6880 x 2880 | 6/8/0 | 16.6 MiB | 75.6 MiB | OutputTooLarge / OutputTooLarge | TrailingBytes / accept | 1730.29 |
| `markpendlebury/ultrawide-wallpapers:6880x2880/Here_is_my_first_abstract_design_6880_x_2880_21_9.png` | real / incidental | 6880 x 2880 | 2/8/0 | 1.7 MiB | 56.7 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 109.81 |
| `dharmx/walls:nord/a_group_of_people_standing_at_a_bar.png` | real / incidental | 6000 x 3274 | 2/8/0 | 1.7 MiB | 56.2 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 108.91 |
| `OpenImageIO/oiio:ASWF/logos/openimageio-stacked-gradient-light.png` | real / test | 5213 x 3668 | 6/8/0 | 0.3 MiB | 72.9 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 123.92 |
| `OpenImageIO/oiio:ASWF/logos/openimageio-stacked-gradient.png` | real / test | 5213 x 3668 | 6/8/0 | 0.3 MiB | 72.9 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 125.36 |
| `neonvoidx/pics:ultrawide/wallhaven-w5m66p.png` | real / incidental | 5760 x 3240 | 6/8/0 | 0.1 MiB | 71.2 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 108.92 |
| `neonvoidx/pics:ultrawide/asciisaturn.png` | real / incidental | 5760 x 3232 | 6/8/0 | 0.8 MiB | 71.0 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 109.6 |
| `dharmx/walls:nord/a_group_of_small_flowers.png` | real / incidental | 5238 x 3492 | 2/8/0 | 0.9 MiB | 52.3 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 109.03 |
| `markpendlebury/ultrawide-wallpapers:6468x2772/felix_the_cat_juggling_skulls_6468x2772.png` | real / incidental | 6468 x 2772 | 2/8/0 | 9.9 MiB | 51.3 MiB | OutputTooLarge / OutputTooLarge | TrailingBytes / accept | 1128.68 |
| `dharmx/walls:flowers/a_group_of_flowers_on_a_black_background.png` | real / incidental | 5184 x 3456 | 2/8/0 | 4.4 MiB | 51.3 MiB | OutputTooLarge / OutputTooLarge | ChunkOutOfOrder / accept | 884.64 |
| `dharmx/walls:nord/a_blue_and_white_dots.png` | real / incidental | 5184 x 3456 | 2/8/0 | 3.5 MiB | 51.3 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 112.74 |
| `opencv/opencv_extra:testdata/cv/cameracalibration/chromatic_aberration/chromatic_aberration_pattern_a3.png` | real / test (hold-out) | 4961 x 3508 | 2/8/0 | 0.3 MiB | 49.8 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 88.58 |
| `neonvoidx/pics:ultrawide/wallhaven-gwqxkq.png` | real / incidental | 5556 x 3125 | 6/8/0 | 16.5 MiB | 66.2 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 200.76 |
| `markpendlebury/ultrawide-wallpapers:7860x2160/Black_Skull_32_9_7860x2160.png` | real / incidental | 7860 x 2160 | 2/8/0 | 4.2 MiB | 48.6 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 112.49 |
| `mozilla/gecko-dev:image/test/reftest/downscaling/32768x100.png` | real / test (hold-out) | 32768 x 100 | 6/8/0 | 0.0 MiB | 12.5 MiB | InvalidDimensions / InvalidDimensions | accept / accept | 10.86 |
| `ruffle-rs/ruffle:tests/tests/swfs/visual/define_bits_jpeg2_huge/image_height.png` | real / test | 2 x 524288 | 2/8/0 | 0.0 MiB | 3.5 MiB | InvalidDimensions / InvalidDimensions | accept / accept | 5.75 |
| `ruffle-rs/ruffle:tests/tests/swfs/visual/define_bits_jpeg2_huge/image_width.png` | real / test | 524288 x 2 | 2/8/0 | 0.0 MiB | 3.0 MiB | InvalidDimensions / InvalidDimensions | accept / accept | 3.16 |
| `gitlab.gnome.org/GNOME/gdk-pixbuf:tests/test-images/gif-test-suite/max-height.png` | real / test | 1 x 65535 | 6/8/0 | 0.0 MiB | 0.3 MiB | InvalidDimensions / InvalidDimensions | accept / accept | 0.44 |
| `gitlab.gnome.org/GNOME/gdk-pixbuf:tests/test-images/gif-test-suite/max-width.png` | real / test | 65535 x 1 | 6/8/0 | 0.0 MiB | 0.2 MiB | InvalidDimensions / InvalidDimensions | accept / accept | 0.22 |
| `image-rs/image-png:tests/bugfixes/x_issue#230.0.png` | real / test | 4293328897 x 0 | 0/16/0 | 0.0 MiB | - | InvalidDimensions / InvalidDimensions | InvalidDimensions / InvalidDimensions | 0.0 |
| `synthetic:thin-2147483648x1-grey1` | synthetic / synthetic | 2147483648 x 1 | 0/1/0 | 0.2 MiB | 256.0 MiB | InvalidDimensions / InvalidDimensions | InvalidDimensions / InvalidDimensions | 0.0 |
| `synthetic:thin-2147483647x1-grey1` | synthetic / synthetic | 2147483647 x 1 | 0/1/0 | 0.2 MiB | 256.0 MiB | InvalidDimensions / InvalidDimensions | OutputTooLarge / OutputTooLarge | 0.0 |
| `synthetic:screen-3x8k-rgba8-overcap` | synthetic / synthetic | 23040 x 4320 | 6/8/0 | 13.3 MiB | 379.7 MiB | InvalidDimensions / InvalidDimensions | OutputTooLarge / OutputTooLarge | 0.0 |
| `synthetic:cap-8192x8193-rgba8` | synthetic / synthetic | 8192 x 8193 | 6/8/0 | 8.9 MiB | 256.0 MiB | OutputTooLarge / OutputTooLarge | OutputTooLarge / OutputTooLarge | 0.0 |
| `synthetic:thin-1x67108865-rgba8` | synthetic / synthetic | 1 x 67108865 | 6/8/0 | 40.1 MiB | 320.0 MiB | InvalidDimensions / InvalidDimensions | OutputTooLarge / OutputTooLarge | 0.0 |
| `synthetic:cap-8192x8192-rgba16-adam7` | synthetic / synthetic | 8192 x 8192 | 6/16/1 | 156.6 MiB | 512.0 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 1539.6 |
| `synthetic:cap-8192x8192-rgba16` | synthetic / synthetic | 8192 x 8192 | 6/16/0 | 39.4 MiB | 512.0 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 929.68 |
| `synthetic:cap-8192x8192-rgba8` | synthetic / synthetic | 8192 x 8192 | 6/8/0 | 3.2 MiB | 256.0 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 440.96 |
| `synthetic:stored-1x67108864-rgba16-adam7` | synthetic / synthetic | 1 x 67108864 | 6/16/1 | 576.1 MiB | 576.0 MiB | FileTooLarge / FileTooLarge | accept / accept | 1036.04 |
| `synthetic:stored-8192x8192-rgba16` | synthetic / synthetic | 8192 x 8192 | 6/16/0 | 512.1 MiB | 512.0 MiB | FileTooLarge / FileTooLarge | accept / accept | 785.14 |
| `synthetic:stored-8192x8192-rgba8` | synthetic / synthetic | 8192 x 8192 | 6/8/0 | 256.1 MiB | 256.0 MiB | FileTooLarge / FileTooLarge | accept / accept | 476.45 |
| `synthetic:thin-1048576x64-grey8` | synthetic / synthetic | 1048576 x 64 | 0/8/0 | 0.1 MiB | 64.0 MiB | InvalidDimensions / InvalidDimensions | accept / accept | 162.06 |
| `synthetic:thin-1x67108864-rgba8` | synthetic / synthetic | 1 x 67108864 | 6/8/0 | 2.6 MiB | 320.0 MiB | InvalidDimensions / InvalidDimensions | accept / accept | 850.85 |
| `synthetic:thin-65536x1024-grey8` | synthetic / synthetic | 65536 x 1024 | 0/8/0 | 0.1 MiB | 64.0 MiB | InvalidDimensions / InvalidDimensions | accept / accept | 169.62 |
| `synthetic:thin-67108864x1-rgba8` | synthetic / synthetic | 67108864 x 1 | 6/8/0 | 1.1 MiB | 256.0 MiB | InvalidDimensions / InvalidDimensions | accept / accept | 427.54 |
| `synthetic:screen-4x2-4k-rgba8` | synthetic / synthetic | 15360 x 4320 | 6/8/0 | 3.7 MiB | 253.1 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 444.3 |
| `synthetic:screen-2x2-4k-rgba8` | synthetic / synthetic | 7680 x 4320 | 6/8/0 | 1.9 MiB | 126.6 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 219.7 |
| `synthetic:screen-4x4k-row-rgba8` | synthetic / synthetic | 15360 x 2160 | 6/8/0 | 1.8 MiB | 126.6 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 210.46 |
| `synthetic:screen-2x5k-rgb8` | synthetic / synthetic | 10240 x 2880 | 2/8/0 | 1.4 MiB | 84.4 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 170.3 |
| `synthetic:screen-3x4k-rgba8` | synthetic / synthetic | 11520 x 2160 | 6/8/0 | 1.4 MiB | 94.9 MiB | OutputTooLarge / OutputTooLarge | accept / accept | 161.43 |
| `synthetic:input-2^30-plus-1` | synthetic / synthetic | 64 x 64 | 6/8/0 | 1,024.0 MiB | 0.0 MiB | FileTooLarge / FileTooLarge | FileTooLarge / FileTooLarge | 0.0 |
| `synthetic:input-exactly-2^30` | synthetic / synthetic | 64 x 64 | 6/8/0 | 1,024.0 MiB | 0.0 MiB | FileTooLarge / FileTooLarge | accept / accept | 265.53 |

Summary, Strict old -> now: real {'InvalidDimensions -> InvalidDimensions': 2, 'InvalidDimensions -> OutputTooLarge': 5, 'InvalidDimensions -> accept': 5, 'OutputTooLarge -> ChunkOutOfOrder': 2, 'OutputTooLarge -> OutputTooLarge': 1, 'OutputTooLarge -> TrailingBytes': 3, 'OutputTooLarge -> accept': 30, 'TruncatedChunk -> TruncatedChunk': 1}; synthetic {'FileTooLarge -> FileTooLarge': 1, 'FileTooLarge -> accept': 4, 'InvalidDimensions -> InvalidDimensions': 1, 'InvalidDimensions -> OutputTooLarge': 3, 'InvalidDimensions -> accept': 4, 'OutputTooLarge -> OutputTooLarge': 1, 'OutputTooLarge -> accept': 8}. RecommendedBeta old -> now: real {'InvalidDimensions -> InvalidDimensions': 2, 'InvalidDimensions -> OutputTooLarge': 5, 'InvalidDimensions -> accept': 5, 'OutputTooLarge -> OutputTooLarge': 1, 'OutputTooLarge -> accept': 35, 'TruncatedChunk -> TruncatedChunk': 1}; synthetic {'FileTooLarge -> FileTooLarge': 1, 'FileTooLarge -> accept': 4, 'InvalidDimensions -> InvalidDimensions': 1, 'InvalidDimensions -> OutputTooLarge': 3, 'InvalidDimensions -> accept': 4, 'OutputTooLarge -> OutputTooLarge': 1, 'OutputTooLarge -> accept': 8}.

Cap errors under the current caps (Strict, still): FileTooLarge 0 real / 1 synthetic; IccProfileTooLarge 0 real / 0 synthetic; InvalidDimensions 5 real / 1 synthetic; OutputTooLarge 6 real / 4 synthetic; PixelBudgetExceeded 0 real / 0 synthetic; TooManyFrames 1 real / 0 synthetic.

- `OpenImageIO/oiio:testsuite/png-bomb/src/bomb.png` (test): 46340 x 46340, OutputTooLarge
- `gitlab.gnome.org/GNOME/gdk-pixbuf:tests/large.png` (test): 25200 x 33300, OutputTooLarge
- `gitlab.gnome.org/GNOME/gdk-pixbuf:tests/test-images/fail/bug793470-crasher.png` (fuzz): 935820 x 854554, OutputTooLarge
- `image-rs/image-png:tests/bugfixes/x_issue#214.png` (test): 0 x 3, InvalidDimensions
- `image-rs/image-png:tests/bugfixes/x_issue#230.0.png` (test): 4293328897 x 0, InvalidDimensions
- `image-rs/image-png:tests/bugfixes/x_issue#230.1.png` (test): 4294967295 x 536883232, InvalidDimensions
- `lovell/sharp:test/fixtures/65536-uint32-limit.png` (test): 65536 x 65536, OutputTooLarge
- `mozilla/gecko-dev:tools/lint/test/files/file-perm/no-shebang/bad.png` (test): 0 x 0, InvalidDimensions
- `onevcat/APNGKit:Tests/APNGKitTests/Resources/General/malformed-size.apng` (test): 524288 x 2048, OutputTooLarge
- `opencv/opencv_extra:testdata/highgui/readwrite/read.png` (test): 13043 x 13917, OutputTooLarge
- `python-pillow/Pillow:Tests/images/apng/syntax_num_frames_invalid.png` (test): 128 x 64, TooManyFrames
- `randy408/libspng:tests/crashers/zero_width.png` (fuzz): 0 x 32, InvalidDimensions
- `synthetic:cap-8192x8193-rgba8` (synthetic): 8192 x 8193, OutputTooLarge
- `synthetic:input-2^30-plus-1` (synthetic): 64 x 64, FileTooLarge
- `synthetic:screen-3x8k-rgba8-overcap` (synthetic): 23040 x 4320, OutputTooLarge
- `synthetic:thin-1x67108865-rgba8` (synthetic): 1 x 67108865, OutputTooLarge
- `synthetic:thin-2147483647x1-grey1` (synthetic): 2147483647 x 1, OutputTooLarge
- `synthetic:thin-2147483648x1-grey1` (synthetic): 2147483648 x 1, InvalidDimensions

### T9. Synthetic files (project-authored, `gen_synthetic.py`)

| file | kind | dims | ct/depth/il | encoding | size | R | out16 | expected | Strict / RB | old caps (measured) | oracle s |
|---|---|---|---|---|---|---|---|---|---|---|---|
| `cap-8192x8192-rgba16` | cap-boundary | 8192 x 8192 | 6/16/0 | z6 | 39.4 MiB | 512.0 MiB | 512.0 MiB | accept | accept / accept | OutputTooLarge | 929.68 |
| `cap-8192x8192-rgba16-adam7` | cap-boundary | 8192 x 8192 | 6/16/1 | z6 | 156.6 MiB | 512.0 MiB | 512.0 MiB | accept | accept / accept | OutputTooLarge | 1539.6 |
| `cap-8192x8192-rgba8` | cap-boundary | 8192 x 8192 | 6/8/0 | z6 | 3.2 MiB | 256.0 MiB | 512.0 MiB | accept | accept / accept | OutputTooLarge | 440.96 |
| `cap-8192x8193-rgba8` | cap-boundary | 8192 x 8193 | 6/8/0 | z1 | 8.9 MiB | 256.0 MiB | 512.1 MiB | OutputTooLarge | OutputTooLarge / OutputTooLarge | OutputTooLarge | 0.0 |
| `input-2^30-plus-1` | input-cap | 64 x 64 | 6/8/0 | z6 | 1,024.0 MiB | 0.0 MiB | 0.0 MiB | FileTooLarge | FileTooLarge / FileTooLarge | FileTooLarge | 0.0 |
| `input-exactly-2^30` | input-cap | 64 x 64 | 6/8/0 | z6 | 1,024.0 MiB | 0.0 MiB | 0.0 MiB | accept | accept / accept | FileTooLarge | 265.53 |
| `screen-2x2-4k-rgba8` | screenshot-multimonitor | 7680 x 4320 | 6/8/0 | z6 | 1.9 MiB | 126.6 MiB | 253.1 MiB | accept | accept / accept | OutputTooLarge | 219.7 |
| `screen-2x4k-rgb8` | screenshot-multimonitor | 7680 x 2160 | 2/8/0 | z6 | 0.9 MiB | 47.5 MiB | 126.6 MiB | accept | accept / accept | accept (intent) | 90.29 |
| `screen-2x4k-rgba16` | screenshot-multimonitor | 7680 x 2160 | 6/16/0 | z6 | 10.1 MiB | 126.6 MiB | 126.6 MiB | accept | accept / accept | accept (intent) | 222.73 |
| `screen-2x5k-rgb8` | screenshot-multimonitor | 10240 x 2880 | 2/8/0 | z6 | 1.4 MiB | 84.4 MiB | 225.0 MiB | accept | accept / accept | OutputTooLarge | 170.3 |
| `screen-3x4k-rgba8` | screenshot-multimonitor | 11520 x 2160 | 6/8/0 | z6 | 1.4 MiB | 94.9 MiB | 189.8 MiB | accept | accept / accept | OutputTooLarge | 161.43 |
| `screen-3x8k-rgba8-overcap` | screenshot-multimonitor | 23040 x 4320 | 6/8/0 | z1 | 13.3 MiB | 379.7 MiB | 759.4 MiB | OutputTooLarge | OutputTooLarge / OutputTooLarge | InvalidDimensions | 0.0 |
| `screen-4k-plus-1440p-rgba8` | screenshot-multimonitor | 6400 x 2160 | 6/8/0 | z6 | 0.8 MiB | 52.7 MiB | 105.5 MiB | accept | accept / accept | accept (intent) | 81.48 |
| `screen-4x2-4k-rgba8` | screenshot-multimonitor | 15360 x 4320 | 6/8/0 | z6 | 3.7 MiB | 253.1 MiB | 506.2 MiB | accept | accept / accept | OutputTooLarge | 444.3 |
| `screen-4x4k-row-rgba8` | screenshot-multimonitor | 15360 x 2160 | 6/8/0 | z6 | 1.8 MiB | 126.6 MiB | 253.1 MiB | accept | accept / accept | OutputTooLarge | 210.46 |
| `stored-1x67108864-rgba16-adam7` | stored-near-cap | 1 x 67108864 | 6/16/1 | stored | 576.1 MiB | 576.0 MiB | 512.0 MiB | accept | accept / accept | FileTooLarge | 1036.04 |
| `stored-8192x8192-rgba16` | stored-near-cap | 8192 x 8192 | 6/16/0 | stored | 512.1 MiB | 512.0 MiB | 512.0 MiB | accept | accept / accept | FileTooLarge | 785.14 |
| `stored-8192x8192-rgba8` | stored-near-cap | 8192 x 8192 | 6/8/0 | stored | 256.1 MiB | 256.0 MiB | 512.0 MiB | accept | accept / accept | FileTooLarge | 476.45 |
| `thin-1048576x64-grey8` | long-thin | 1048576 x 64 | 0/8/0 | z6 | 0.1 MiB | 64.0 MiB | 512.0 MiB | accept | accept / accept | InvalidDimensions | 162.06 |
| `thin-1x67108864-rgba8` | long-thin | 1 x 67108864 | 6/8/0 | z6 | 2.6 MiB | 320.0 MiB | 512.0 MiB | accept | accept / accept | InvalidDimensions | 850.85 |
| `thin-1x67108865-rgba8` | long-thin | 1 x 67108865 | 6/8/0 | z1 | 40.1 MiB | 320.0 MiB | 512.0 MiB | OutputTooLarge | OutputTooLarge / OutputTooLarge | InvalidDimensions | 0.0 |
| `thin-2147483647x1-grey1` | long-thin | 2147483647 x 1 | 0/1/0 | z6 | 0.2 MiB | 256.0 MiB | 16,384.0 MiB | OutputTooLarge | OutputTooLarge / OutputTooLarge | InvalidDimensions | 0.0 |
| `thin-2147483648x1-grey1` | long-thin | 2147483648 x 1 | 0/1/0 | z6 | 0.2 MiB | 256.0 MiB | 16,384.0 MiB | InvalidDimensions | InvalidDimensions / InvalidDimensions | InvalidDimensions | 0.0 |
| `thin-65536x1024-grey8` | long-thin | 65536 x 1024 | 0/8/0 | z6 | 0.1 MiB | 64.0 MiB | 512.0 MiB | accept | accept / accept | InvalidDimensions | 169.62 |
| `thin-67108864x1-rgba8` | long-thin | 67108864 x 1 | 6/8/0 | z6 | 1.1 MiB | 256.0 MiB | 512.0 MiB | accept | accept / accept | InvalidDimensions | 427.54 |

### T10. Largest quantities among RecommendedBeta-accepted files, and reservations against MAX_BYTES

| quantity | real (all classes) | real incidental |
|---|---|---|
| area_px | 60,932,896 (`mozilla/gecko-dev:third_party/python/aiohttp/vendor/llhttp/images/http-strict-none.png`) | 60,932,896 (`mozilla/gecko-dev:third_party/python/aiohttp/vendor/llhttp/images/http-strict-none.png`) |
| side | 524,288 (`ruffle-rs/ruffle:tests/tests/swfs/visual/define_bits_jpeg2_huge/image_height.png`) | 16,384 (`godotengine/godot-demo-projects:3d/sky_shaders/perlworlnoise.png`) |
| size_bytes | 46,397,113 (`dharmx/walls:nord/cherry-blossoms.png`) | 46,397,113 (`dharmx/walls:nord/cherry-blossoms.png`) |
| R | 243,740,456 (`mozilla/gecko-dev:third_party/python/aiohttp/vendor/llhttp/images/http-strict-none.png`) | - |
| out16_bytes_depth16 | 84,000,000 (`opencv/opencv_extra:testdata/dnn/caffe_fcn8s_prob.png`) | - |
| num_frames | 2,147,483,649 (`python-pillow/Pillow:Tests/images/apng/syntax_num_frames_invalid.png`) | - |
| fctl_pixel_sum | 112,457,160 (`penfeizhou/APNG4Android:app/src/main/assets/wheel.png`) | - |
| iccp_inflated | 1,401,220 (`gitlab.gnome.org/GNOME/gimp-help:images/it/filters/animation/waves.png`) | - |
| aspect | 262,144.0 (`ruffle-rs/ruffle:tests/tests/swfs/visual/define_bits_jpeg2_huge/image_height.png`) | - |

Largest single reservation among accepted files: real: R 232.4 MiB (`mozilla/gecko-dev:third_party/python/aiohttp/vendor/llhttp/images/http-strict-none.png`), out16 464.9 MiB (`mozilla/gecko-dev:third_party/python/aiohttp/vendor/llhttp/images/http-strict-none.png`), out8 232.4 MiB (`mozilla/gecko-dev:third_party/python/aiohttp/vendor/llhttp/images/http-strict-none.png`), saved 7.9 MiB (`penfeizhou/APNG4Android:app/src/main/assets/wheel.png`); synthetic: R 576.0 MiB (`synthetic:stored-1x67108864-rgba16-adam7`), out16 512.0 MiB (`synthetic:thin-67108864x1-rgba8`), out8 256.0 MiB (`synthetic:thin-67108864x1-rgba8`).

Accepted files with a reservation above the old guard (64 MiB): {'real': {'R': 24, 'out16': 194, 'out8': 35}, 'synthetic': {'R': 16, 'out16': 18, 'out8': 15}}; above MAX_BYTES (1 GiB): {'real': 0, 'synthetic': 0}. Largest R + RGBA16 output (decode_still16 peak): {'real': [731203624, 'mozilla/gecko-dev:third_party/python/aiohttp/vendor/llhttp/images/http-strict-none.png'], 'synthetic': [1140850688, 'synthetic:stored-1x67108864-rgba16-adam7']}.

Largest real files by declared area (all outcomes):

| area | dims | file | class | Strict | RB |
|---|---|---|---|---|---|
| 2,305,895,922,673.9 Mpx | 4294967295 x 536883232 | `image-rs/image-png:tests/bugfixes/x_issue#230.1.png` | test | InvalidDimensions | InvalidDimensions |
| 290,487,185,202.0 Mpx | 538968589 x 538968673 | `gitlab.gnome.org/GNOME/gdk-pixbuf:tests/test-images/fail/invalid.4.png` | test | TruncatedChunk | TruncatedChunk |
| 799,708.7 Mpx | 935820 x 854554 | `gitlab.gnome.org/GNOME/gdk-pixbuf:tests/test-images/fail/bug793470-crasher.png` | fuzz | OutputTooLarge | OutputTooLarge |
| 4,295.0 Mpx | 65536 x 65536 | `lovell/sharp:test/fixtures/65536-uint32-limit.png` | test | OutputTooLarge | OutputTooLarge |
| 2,147.4 Mpx | 46340 x 46340 | `OpenImageIO/oiio:testsuite/png-bomb/src/bomb.png` | test | OutputTooLarge | OutputTooLarge |
| 1,073.7 Mpx | 524288 x 2048 | `onevcat/APNGKit:Tests/APNGKitTests/Resources/General/malformed-size.apng` | test | OutputTooLarge | OutputTooLarge |
| 839.2 Mpx | 25200 x 33300 | `gitlab.gnome.org/GNOME/gdk-pixbuf:tests/large.png` | test | OutputTooLarge | OutputTooLarge |
| 181.5 Mpx | 13043 x 13917 | `opencv/opencv_extra:testdata/highgui/readwrite/read.png` | test | OutputTooLarge | OutputTooLarge |
| 60.9 Mpx | 6868 x 8872 | `mozilla/gecko-dev:third_party/python/aiohttp/vendor/llhttp/images/http-strict-none.png` | incidental | accept | accept |
| 56.2 Mpx | 10000 x 5625 | `neonvoidx/pics:ultrawide/wallhaven-yq8yex.png` | incidental | TrailingBytes | accept |
| 33.2 Mpx | 7680 x 4320 | `k1ng440/Wallpapers:wallpapers/101.png` | incidental | accept | accept |
| 33.2 Mpx | 7680 x 4320 | `k1ng440/Wallpapers:wallpapers/099.png` | incidental | accept | accept |
| 33.2 Mpx | 7680 x 4320 | `dharmx/walls:nord/nord4.png` | incidental | accept | accept |
| 33.2 Mpx | 7680 x 4320 | `dharmx/walls:nord/a_pattern_of_circles_and_lines.png` | incidental | accept | accept |
| 33.2 Mpx | 7680 x 4320 | `dharmx/walls:nord/a_cartoon_of_a_couple_of_tentacles.png` | incidental | accept | accept |

Thin real files (aspect at least 32:1, long side at least 1,024): 41 (incidental 25); Strict {'CrcMismatch': 2, 'InvalidChunkType': 1, 'OutputTooLarge': 1, 'accept': 37}; RecommendedBeta {'CrcMismatch': 2, 'InvalidChunkType': 1, 'OutputTooLarge': 1, 'accept': 37}.

### T11. Against the 2026-11-16 pass (same manifest; that pass ran the old oracles at the old caps, 20,502 files)

Files run in both passes with the same Strict and RecommendedBeta outcome (and the same Strict RGBA): 20497; changed: 5; not run on 2026-11-16: 1464.

- `gitlab.gnome.org/GNOME/gdk-pixbuf:tests/test-images/gif-test-suite/max-height.png` (test): Strict/RB InvalidDimensions / InvalidDimensions then, accept / accept now
- `gitlab.gnome.org/GNOME/gdk-pixbuf:tests/test-images/gif-test-suite/max-width.png` (test): Strict/RB InvalidDimensions / InvalidDimensions then, accept / accept now
- `mozilla/gecko-dev:image/test/reftest/downscaling/32768x100.png` (test): Strict/RB InvalidDimensions / InvalidDimensions then, accept / accept now
- `ruffle-rs/ruffle:tests/tests/swfs/visual/define_bits_jpeg2_huge/image_height.png` (test): Strict/RB InvalidDimensions / InvalidDimensions then, accept / accept now
- `ruffle-rs/ruffle:tests/tests/swfs/visual/define_bits_jpeg2_huge/image_width.png` (test): Strict/RB InvalidDimensions / InvalidDimensions then, accept / accept now

### T12. Incumbents (cached 2026-11-16 runner results, real files only), against RecommendedBeta still

| runner | both accept | RB only | incumbent only (incidental) | incumbent only, RB error | first frame equal | first frame differs, by depth |
|---|---|---|---|---|---|---|
| gdk-file | 21601 | 113 | 12 (6) | BadSignature 7, ExcessImageData 2, InvalidPalette 1, OutputTooLarge 2 | 21355 | {'16': 246} |
| gdk-loader | 21714 | 0 | 124 (0) | AdlerMismatch 1, ExcessImageData 4, InvalidPalette 1, OutputTooLarge 2, TruncatedChunk 115, TruncatedImageData 1 | 21465 | {'8': 2, '16': 247} |
| glycin | 21705 | 9 | 14 (0) | AdlerMismatch 1, CrcMismatch 3, DuplicateChunk 1, ExcessImageData 4, InvalidChunkType 2, OutputTooLarge 2, UnknownCriticalChunk 1 | 21662 | {'1': 1, '2': 1, '8': 41} |
| image | 21705 | 9 | 14 (0) | AdlerMismatch 1, CrcMismatch 3, DuplicateChunk 1, ExcessImageData 4, InvalidChunkType 2, OutputTooLarge 2, UnknownCriticalChunk 1 | 21662 | {'1': 1, '2': 1, '8': 41} |
| image-still | 21713 | 1 | 14 (0) | AdlerMismatch 1, CrcMismatch 3, DuplicateChunk 1, ExcessImageData 4, InvalidChunkType 2, OutputTooLarge 2, UnknownCriticalChunk 1 | 21712 | {'1': 1} |
| libpng | 21712 | 2 | 13 (0) | CrcMismatch 4, DuplicateChunk 1, ExcessImageData 3, InvalidChunkType 1, InvalidPalette 1, OutputTooLarge 1, TruncatedChunk 1, UnknownCriticalChunk 1 | 21465 | {'16': 247} |

### T13. Input-count reconciliation (`reconcile_counts.py`)

| count | recomputed | definition |
|---|---|---|
| 748 | 748 | records in `policy-draft/probes/results.json`: 516 vector, 177 probe, 55 witness; 14 above 4 KiB stored by reference, 734 inline |
| 737 | 737 | Strict bridge of `validate.py` (distinct bytes over non-padded vectors and inline results.json records), also without the 3 OVER corner vectors of 2026-11-16 |
| 740 | 740 | Strict bridge of `validate.py` (distinct bytes over non-padded vectors and inline results.json records), also without the 198 profile vectors of VECTORS-POLICY.json |
| 767 | 767 | Strict bridge of `validate.py` (distinct bytes over non-padded vectors and inline results.json records), also without the 6 cap-boundary vectors of 2026-11-17 |
| 773 | 773 | Strict bridge of `validate.py` (distinct bytes over non-padded vectors and inline results.json records), without the 14 review/council vectors (11 distinct non-padded inputs) |
| 784 | 784 | Strict bridge of `validate.py` (distinct bytes over non-padded vectors and inline results.json records), now: Strict bridge after the review and council vectors (2026-11-17) |

Parts: 734 distinct inline records; 533 distinct non-padded vectors of today's core and caps files without the 17 later vectors, 530 of them also inline (734 + 533 - 530 = 737); the 198 profile vectors add 27 distinct inputs; the 14 review/council vectors add 11; 2 vector records are padded (`pad_to`) and never bridged. Vector records now: 810 (spec-frozen/VECTORS.json 599, limits-draft/VECTORS-CAPS.json 13, policy-draft/VECTORS-POLICY.json 198).

Timing: 137,968 s of oracle wall time summed over workers; slowest: 1,731 s `markpendlebury/ultrawide-wallpapers:6880x2880/Black_Myth_Wukong_6880x2880.png`; 1,544 s `synthetic:cap-8192x8192-rgba16-adam7`; 1,355 s `k1ng440/Wallpapers:wallpapers/098.png`; 1,299 s `dharmx/walls:outrun/a_sports_car_with_neon_lights.png`; 1,160 s `markpendlebury/ultrawide-wallpapers:7680x2160/Aurora_7680x2160.png`.

<!-- tables:end -->
