# GIF-Q6: LZW minimum code sizes in the real-file corpus

Recommendation: (b). Accept minimum code sizes 1-11 in RecommendedBeta and reject any literal code above 255 as `ColorIndexOutOfRange`. Keeping 2-8 (a) rejects five corpus files that gdk-pixbuf 2.42.12 (animation path and forced loader) and `image` 0.25.10 / `gif` 0.14.2 all decode, with first frames identical to each other and to the first frame of a 1-11 decoder. Checked literals (b) and wrapping modulo 256 (c) give the same result on every file, so the corpus cannot choose between them; (b) wins on the argument that a literal above 255 cannot be an 8-bit pixel value, so no conforming encoder emits one and rejecting it costs nothing.

## Numbers

- The lenient scan (`gif_stats.scan`, which does not stop at LZW errors) reaches 5,452 image descriptors in 1,407 files; 5,283 (96.9%) have size 2-8. 98 files have another value: 83 of the 326 fuzzer-derived files (25%) and 15 of the 1,081 others (1.4%). None of the 303 incidental assets (site, documentation and demo images) has one.
- The lenient scan also reads size bytes out of garbage after a structural break. Counting only frames that the Strict block grammar reaches without error leaves 5,039 frames, 21 of them (0.4%) outside 2-8, in 20 files. Those are exactly the 20 files whose first Strict error is `InvalidLzwMinCodeSize`; 9 are not fuzzer-derived and 11 are.
- Of the 15 files that are not fuzzer-derived with a size outside 2-8, five are decodable images: size 1 in two files that share one 42-byte 1x1 transparent GIF (one copy has 12 trailing bytes), size 10 in a 3x1 image, size 11 in two 100x100 images. Four are invalid inputs from failure or regression tests (sizes 12, 12, 154, 255) and all three incumbents reject them. Six carry a size byte behind a header or frame descriptor that is invalid on other grounds (zero or oversize dimensions, frame outside the screen), and all three reject them.
- All three incumbents accept the five decodable files. Their first frames are equal to each other (RGB zeroed where alpha is 0) and equal to the first frame of the 1-11 checked decoder and of the 1-11 wrapping decoder. Four of the five are otherwise clean (the 1-11 checked decoder accepts the whole file); `chromium` `0.gif` has 12 bytes after the trailer.
- Over all 1,407 files, widening Recommended's `lzw_min` from 2-8 to 1-11 turns exactly those five files from reject to accept, with no other relaxation needed. Six fuzz mutants change error name only (`TruncatedImageData` four times, `InvalidLzwCode`, `TooFewPixels`) and stay rejected.
- No decode of the 99 examined files consumes a literal code in 256..2^m-1 before it ends (at the trailer or at another error), so zero files change between rejected and a different pixel result when going from checked to wrapping. A synthetic 2x1 stream (not corpus bytes: size 9, codes clear, 300, 7, EOI) shows what the incumbents do when one appears: gdk-pixbuf on both paths and `image` all output palette[300 mod 256], that is palette[44].
- Size 0 occurs at three Strict-reached frames in three files, all fuzzer-derived; `image` rejects all three, and gdk-pixbuf accepts one of them on its animation path only (the forced loader rejects it). Sizes 12 and above occur at ten Strict-reached frames in ten files; all three incumbents reject nine of them, and the tenth (`8ab2ff8208`, which also contains a size-0 frame) is accepted only by gdk-pixbuf's animation path.

## What the corpus cannot show

- Every source is a decoder test suite or image library, and test suites keep unusual files because they once broke something. All five decodable odd-size files sit in test directories, and none of the 303 incidental assets uses one. The corpus shows that sizes 1, 10 and 11 exist and that both incumbents decode them; it does not show how common they are on the web. There are four distinct images, not five.
- 326 files (23%) are fuzzer-derived and they hold 83 of the 98 files with an odd size. Mutation produces sizes 0 and 12+ at a rate no encoder does, so those counts say nothing about encoders. The two files where an incumbent accepts something the 1-11 checked decoder rejects are both fuzzer-derived, and both are gdk-pixbuf recovery rules rather than sizes 1 or 9-11: size 0, and stopping a frame at an invalid code. The policy oracle's `GdkCompat-2.42.12` profile reproduces gdk-pixbuf's first frame on both, and `ImageCrateCompat-0.25.10` rejects both as `image` does.
- No file here exercises a high literal, so the corpus cannot say whether a real producer emits one. If one does, (b) rejects a file that all three incumbents display with wrapped colours and (c) would reproduce those colours. The corpus gives no reason to pay for (c).
- The runners dump only the first composited frame. All five decodable files have a single frame, so first-frame equality is whole-file equality for them.
- No file accepted by all three uses size 0, so gdk-pixbuf's size-0 behaviour (not reproduced by the policy oracle) is untested here.

## Implementation note for (b)

`policy_oracle` already has `lzw_min=Range1to11`, which wraps (option c); (b) needs a third value. The checked decoder in `gif_lzwmin.py` (`lzw_decode_1to11`) keeps Strict's order of checks and raises `ColorIndexOutOfRange` when a code in 256..2^m-1 is read, after the "raster complete without End-of-Information" check and before the code is emitted. Its wrapping variant equals `policy_oracle` with `STRICT` plus `Range1to11` on all 99 files (whole-file result and first frame, 0 mismatches).

## Histograms

LZW minimum code size byte of every image descriptor reached, counted in frames and in files (cell = frames / files; values that never occur are omitted, so every value in 0..255 not listed has count 0). "Fuzzer-derived" is `gif_stats.is_fuzz` (path contains afl, fuzz or crash, which also catches the `crashtest` and `crash_repro` bug reproductions); "incidental asset" is `gif_stats.is_incidental` (no test-like token in the path). "Not fuzzer-derived" includes the incidental assets.

**lenient scan**

| class | files | frames | frames, size 2..8 | frames, size 1 or 9..11 | frames, size 0 or 12..255 | files with any size outside 2..8 | files with a 1 or 9..11 |
|---|---|---|---|---|---|---|---|
| all files | 1407 | 5452 | 5283 | 84 | 85 | 98 | 28 |
| fuzzer-derived | 326 | 481 | 328 | 78 | 75 | 83 | 22 |
| not fuzzer-derived | 1081 | 4971 | 4955 | 6 | 10 | 15 | 6 |
| incidental asset | 303 | 1884 | 1884 | 0 | 0 | 0 | 0 |

**structure-intact**

| class | files | frames | frames, size 2..8 | frames, size 1 or 9..11 | frames, size 0 or 12..255 | files with any size outside 2..8 | files with a 1 or 9..11 |
|---|---|---|---|---|---|---|---|
| all files | 1407 | 5039 | 5018 | 8 | 13 | 20 | 8 |
| fuzzer-derived | 326 | 152 | 140 | 3 | 9 | 11 | 3 |
| not fuzzer-derived | 1081 | 4887 | 4878 | 5 | 4 | 9 | 5 |
| incidental asset | 303 | 1882 | 1882 | 0 | 0 | 0 | 0 |

**Lenient scan (every image descriptor the scan reaches)**

| class | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 59 | 70 | 72 | 90 | 102 | 143 | 154 | 156 | 171 | 255 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| all files (1407) | 55 / 55 | 77 / 21 | 920 / 414 | 217 / 133 | 318 / 115 | 361 / 89 | 687 / 106 | 295 / 67 | 2485 / 372 | 1 / 1 | 2 / 2 | 4 / 4 | 18 / 15 | 1 / 1 | 1 / 1 | 1 / 1 | 1 / 1 | 1 / 1 | 2 / 2 | 1 / 1 | 1 / 1 | 1 / 1 | 2 / 2 |
| fuzzer-derived (326) | 54 / 54 | 74 / 18 | 214 / 96 | 26 / 11 | 17 / 14 | 0 / 0 | 47 / 39 | 4 / 4 | 20 / 8 | 1 / 1 | 1 / 1 | 2 / 2 | 15 / 12 | 0 / 0 | 1 / 1 | 1 / 1 | 1 / 1 | 0 / 0 | 0 / 0 | 0 / 0 | 1 / 1 | 1 / 1 | 1 / 1 |
| not fuzzer-derived (1081) | 1 / 1 | 3 / 3 | 706 / 318 | 191 / 122 | 301 / 101 | 361 / 89 | 640 / 67 | 291 / 63 | 2465 / 364 | 0 / 0 | 1 / 1 | 2 / 2 | 3 / 3 | 1 / 1 | 0 / 0 | 0 / 0 | 0 / 0 | 1 / 1 | 2 / 2 | 1 / 1 | 0 / 0 | 0 / 0 | 1 / 1 |
| incidental asset (303) | 0 / 0 | 0 / 0 | 55 / 45 | 16 / 15 | 26 / 23 | 183 / 36 | 109 / 22 | 102 / 23 | 1393 / 168 | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 0 |

**Structure-intact frames (reached by the Strict block grammar)**

| class | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 154 | 255 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| all files (1407) | 3 / 3 | 4 / 4 | 760 / 312 | 209 / 126 | 302 / 104 | 359 / 87 | 641 / 72 | 294 / 66 | 2453 / 360 | 1 / 1 | 1 / 1 | 2 / 2 | 7 / 7 | 1 / 1 | 2 / 2 |
| fuzzer-derived (326) | 3 / 3 | 2 / 2 | 88 / 8 | 25 / 10 | 9 / 8 | 0 / 0 | 6 / 6 | 3 / 3 | 9 / 4 | 1 / 1 | 0 / 0 | 0 / 0 | 5 / 5 | 0 / 0 | 1 / 1 |
| not fuzzer-derived (1081) | 0 / 0 | 2 / 2 | 672 / 304 | 184 / 116 | 293 / 96 | 359 / 87 | 635 / 66 | 291 / 63 | 2444 / 356 | 0 / 0 | 1 / 1 | 2 / 2 | 2 / 2 | 1 / 1 | 1 / 1 |
| incidental asset (303) | 0 / 0 | 0 / 0 | 55 / 45 | 15 / 14 | 26 / 23 | 182 / 35 | 109 / 22 | 102 / 23 | 1393 / 168 | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 0 |

## Files that are not fuzzer-derived with a size outside 2-8 (15)

The size columns give the lenient-scan values, then the values at frames the Strict grammar reaches (`-` = none reached: the header or a frame descriptor is invalid first). "with 1-11 checked" is the whole-file result of Strict with the size range widened and literals above 255 rejected; `ok` means the rest of the file is well-formed. Incumbent cells: `eq` = accepts and the first frame equals the 1-11 checked decoder's (RGB zeroed where alpha is 0, canvas size equal); `acc (hyp. rejects)` = accepts a file whose first frame the 1-11 decoder rejects; `rej` = rejects.

| sha256 | file | sizes (lenient; Strict-reached) | canvas | Strict first error | with 1..11 checked: whole file | gdk anim | gdk forced | image |
|---|---|---|---|---|---|---|---|---|
| 1a564832df | `Pillow:Tests/images/issue_2811.gif` | 11; 11 | 100x100 | InvalidLzwMinCodeSize | ok | eq | eq | eq |
| bf19f43656 | `libnsgif:test/data/lzw-code-three-subblocks.gif` | 10; 10 | 3x1 | InvalidLzwMinCodeSize | ok | eq | eq | eq |
| 6d8ba81d1b | `image-gif:tests/samples/set_hsts.gif` | 1; 1 | 1x1 | InvalidLzwMinCodeSize | ok | eq | eq | eq |
| 27b2355413 | `gdk-pixbuf:tests/test-images/gif-test-suite/max-codes.gif` | 11; 11 | 100x100 | InvalidLzwMinCodeSize | ok | eq | eq | eq |
| d8d16484a4 | `chromium:third_party/blink/web_tests/tables/mozilla/images/0.gif` | 1; 1 | 1x1 | InvalidLzwMinCodeSize | TrailingBytes | eq | eq | eq |
| 603447693c | `libnsgif:test/data/lzw-min-code-12.gif` | 12; 12 | 1x1 | InvalidLzwMinCodeSize | InvalidLzwMinCodeSize | rej | rej | rej |
| 5ad4ca4500 | `libgd:tests/gif/bug00005_3.gif` | 154; 154 | 130x130 | InvalidLzwMinCodeSize | InvalidLzwMinCodeSize | rej | rej | rej |
| 9cfb8f3b3f | `gdk-pixbuf:tests/test-images/fail/overflow-codes-max.gif` | 255; 255 | 2x2 | InvalidLzwMinCodeSize | InvalidLzwMinCodeSize | rej | rej | rej |
| bcd25a260f | `gdk-pixbuf:tests/test-images/fail/overflow-codes.gif` | 12; 12 | 2x2 | InvalidLzwMinCodeSize | InvalidLzwMinCodeSize | rej | rej | rej |
| 4fde50dda1 | `WebKit:Source/ThirdParty/skia/resources/invalid_images/skbug6046.gif` | 1,6,102; - | 32x0 | InvalidDimensions | InvalidDimensions | rej | rej | rej |
| 7357284ef3 | `Pillow:Tests/images/decompression_bomb.gif` | 143; - | 32x32 | FrameOutsideScreen | FrameOutsideScreen | rej | rej | rej |
| f5aa13350b | `Pillow:Tests/images/zero_width.gif` | 143; - | 0x32 | InvalidDimensions | InvalidDimensions | rej | rej | rej |
| e7ff8f70ed | `libnsgif:test/data/lzw-oob.gif` | 12; - | 4x13060 | InvalidDimensions | InvalidDimensions | rej | rej | rej |
| 0a32d7e7b6 | `image-gif:tests/stall/issue-101-infinite-empty-loop.gif` | 0; - | 6727x65351 | InvalidDimensions | InvalidDimensions | rej | rej | rej |
| 4cde31fe4b | `gdk-pixbuf:tests/test-images/gif-test-suite/image-zero-width.gif` | 59; - | 1x1 | InvalidFrameDimensions | InvalidFrameDimensions | rej | rej | rej |

## Fuzzer-derived files with a size outside 2-8 at a Strict-reached frame (11)

| sha256 | file | sizes (lenient; Strict-reached) | canvas | Strict first error | with 1..11 checked: whole file | gdk anim | gdk forced | image |
|---|---|---|---|---|---|---|---|---|
| 8ab2ff8208 | `gecko-dev:image/test/crashtests/1443232-1.gif` | 0,8x3,255; 0,8x3,255 | 120x124 | InvalidLzwMinCodeSize | InvalidLzwMinCodeSize | acc (hyp. rejects) | rej | rej |
| cf316c9d99 | `libnsgif:ns-afl-gif/id:000005` | 12; 12 | 32x32 | InvalidLzwMinCodeSize | InvalidLzwMinCodeSize | rej | rej | rej |
| a72658c3f4 | `libnsgif:ns-afl-gif/id:000007` | 1; 1 | 32x32 | InvalidLzwMinCodeSize | TruncatedImageData | rej | rej | rej |
| 16848b8ff6 | `libnsgif:ns-afl-gif/id:000008` | 0; 0 | 32x32 | InvalidLzwMinCodeSize | InvalidLzwMinCodeSize | rej | rej | rej |
| 1eee7b583f | `libnsgif:ns-afl-gif/id:000015` | 12; 12 | 32x32 | InvalidLzwMinCodeSize | InvalidLzwMinCodeSize | rej | rej | rej |
| f987619a76 | `libnsgif:ns-afl-gif/id:000017` | 12; 12 | 32x32 | InvalidLzwMinCodeSize | InvalidLzwMinCodeSize | rej | rej | rej |
| ad5d18c7d0 | `libnsgif:ns-afl-gif/id:000021` | 9; 9 | 32x32 | InvalidLzwMinCodeSize | InvalidLzwCode | acc (hyp. rejects) | acc (hyp. rejects) | rej |
| 1c93ec8b9f | `libnsgif:ns-afl-gif/id:000036` | 0; 0 | 32x32 | InvalidLzwMinCodeSize | InvalidLzwMinCodeSize | rej | rej | rej |
| 231df5124c | `libnsgif:ns-afl-gif/id:000092` | 1; 1 | 32x32 | InvalidLzwMinCodeSize | TruncatedImageData | rej | rej | rej |
| 1dcaf4e38f | `libnsgif:ns-afl-gif/id:000135` | 12; 12 | 32x32 | InvalidLzwMinCodeSize | InvalidLzwMinCodeSize | rej | rej | rej |
| a75d454113 | `libnsgif:ns-afl-gif/id:000324` | 1x4,12x3; 12 | 32x32 | InvalidLzwMinCodeSize | InvalidLzwMinCodeSize | rej | rej | rej |

## Fuzzer-derived files where the size is read behind a structural break or an invalid header (73)

Included for completeness. The size byte is an artefact of mutation or of misaligned reading, and the file fails Strict on a header, frame descriptor, extension or disposal error before any LZW data matters. The `3e9b7e56ab` row is found only by the Recommended policy walk, which skips the malformed Plain Text block size that the lenient scan misreads.

| sha256 | file | sizes (lenient) | canvas | Strict first error |
|---|---|---|---|---|
| beb99ae985 | `gecko-dev:image/test/crashtests/1235605.gif` | 4,171 | 65534x65531 | InvalidDimensions |
| 30f8191b7b | `gecko-dev:image/test/crashtests/1262549-1.gif` | 70 | 32x32 | FrameOutsideScreen |
| 27edfa6bb4 | `gecko-dev:image/test/crashtests/1413762-1.gif` | 72 | 43x43 | InvalidFrameDimensions |
| bac620b664 | `imageflow:imageflow_core/tests/crash_repro/gif_frame_buffer_oob.gif` | 10 | 0x256 | InvalidDimensions |
| 4efd39a7f3 | `libnsgif:ns-afl-gif/id:000033` | 90 | 32x32 | InvalidGraphicControl |
| 8793aa3c55 | `libnsgif:ns-afl-gif/id:000097` | 0 | 32x32 | FrameOutsideScreen |
| 4d4ea357a1 | `libnsgif:ns-afl-gif/id:000098` | 1 | 32x32 | UndefinedDisposal |
| 33c36e5390 | `libnsgif:ns-afl-gif/id:000130` | 0 | 32x32 | InvalidFrameDimensions |
| 3e9b7e56ab | `libnsgif:ns-afl-gif/id:000140` | (policy walk: 1) | 32x32 | InvalidPlainText |
| fba312d931 | `libnsgif:ns-afl-gif/id:000231` | 0,7 | 62239x32 | InvalidDimensions |
| 84feb9ebc7 | `libnsgif:ns-afl-gif/id:000243` | 0,1x3,156 | 32x32 | UndefinedDisposal |
| 22f333ffb2 | `libnsgif:ns-afl-gif/id:000281` | 0 | 32x59136 | InvalidDimensions |
| 04ce76b1e1 | `libnsgif:ns-afl-gif/id:000312` | 1x12,12 | 232x1 | FrameOutsideScreen |
| c3faf69fde | `libnsgif:ns-afl-gif/id:000313` | 1x5,12x2 | 232x1 | FrameOutsideScreen |
| cef123a7f4 | `libnsgif:ns-afl-gif/id:000323` | 0,1x2 | 32x32 | FrameOutsideScreen |
| 81adb0bd5d | `libnsgif:ns-afl-gif/id:000325` | 1x4 | 32x32 | InvalidFrameDimensions |
| 009adf0c4e | `libnsgif:ns-afl-gif/id:000368` | 0,6 | 32x65312 | InvalidDimensions |
| 2f24ad165e | `libnsgif:ns-afl-gif/id:000371` | 0,6 | 32x65312 | InvalidDimensions |
| 983b3a2a99 | `libnsgif:ns-afl-gif/id:000372` | 0,6 | 32x65312 | InvalidDimensions |
| 768f476ed3 | `libnsgif:ns-afl-gif/id:000373` | 0,6 | 32x65312 | InvalidDimensions |
| d45d54f8a8 | `libnsgif:ns-afl-gif/id:000375` | 0,6 | 32x65312 | InvalidDimensions |
| c31cc9f54d | `libnsgif:ns-afl-gif/id:000376` | 0,6 | 32x65312 | InvalidDimensions |
| 5dd46a47fe | `libnsgif:ns-afl-gif/id:000377` | 0,6 | 32x65312 | InvalidDimensions |
| bbb312bb03 | `libnsgif:ns-afl-gif/id:000390` | 0,6 | 32x65312 | InvalidDimensions |
| a3aa46d580 | `libnsgif:ns-afl-gif/id:000390` | 0 | 32x65312 | InvalidDimensions |
| 522e25a9fb | `libnsgif:ns-afl-gif/id:000391` | 0,6 | 32x65312 | InvalidDimensions |
| c2d5ada481 | `libnsgif:ns-afl-gif/id:000391` | 0 | 32x65312 | InvalidDimensions |
| 20b980ef9f | `libnsgif:ns-afl-gif/id:000392` | 0,6x2 | 32x65312 | InvalidDimensions |
| fad6a210dd | `libnsgif:ns-afl-gif/id:000394` | 0,6 | 32x65312 | InvalidDimensions |
| 8f22347c13 | `libnsgif:ns-afl-gif/id:000394` | 0 | 32x65312 | InvalidDimensions |
| 864f8d69fd | `libnsgif:ns-afl-gif/id:000396` | 0,6 | 32x65312 | InvalidDimensions |
| ac4f8c80bc | `libnsgif:ns-afl-gif/id:000397` | 0,1,6 | 32x65312 | InvalidDimensions |
| 69568968c5 | `libnsgif:ns-afl-gif/id:000398` | 12 | 32x65312 | InvalidDimensions |
| 1507e73224 | `libnsgif:ns-afl-gif/id:000409` | 0 | 32x65312 | InvalidDimensions |
| da16f52b11 | `libnsgif:ns-afl-gif/id:000418` | 0 | 32x65312 | InvalidDimensions |
| fe7c89cb32 | `libnsgif:ns-afl-gif/id:000425` | 0 | 32x65312 | InvalidDimensions |
| b3569e818b | `libnsgif:ns-afl-gif/id:000428` | 0 | 32x65312 | InvalidDimensions |
| e718f1500d | `libnsgif:ns-afl-gif/id:000433` | 0 | 32x65312 | InvalidDimensions |
| e826befc06 | `libnsgif:ns-afl-gif/id:000438` | 0 | 32x65312 | InvalidDimensions |
| 8aa08c720e | `libnsgif:ns-afl-gif/id:000450` | 0 | 32x65312 | InvalidDimensions |
| 886d46949a | `libnsgif:ns-afl-gif/id:000463` | 0,6 | 32x65312 | InvalidDimensions |
| fee545015a | `libnsgif:ns-afl-gif/id:000468` | 0,6x2 | 32x65312 | InvalidDimensions |
| 3d8843f26e | `libnsgif:ns-afl-gif/id:000479` | 0,6 | 32x65312 | InvalidDimensions |
| a1085d265d | `libnsgif:ns-afl-gif/id:000485` | 0,6 | 32x65312 | InvalidDimensions |
| b232148679 | `libnsgif:ns-afl-gif/id:000495` | 0,6x2 | 32x65312 | InvalidDimensions |
| 5af8386a5c | `libnsgif:ns-afl-gif/id:000496` | 12 | 32x65312 | InvalidDimensions |
| 2495955e9a | `libnsgif:ns-afl-gif/id:000498` | 1x2,2x2,12 | 32x65312 | InvalidDimensions |
| 232686ce07 | `libnsgif:ns-afl-gif/id:000499` | 12 | 232x1 | FrameOutsideScreen |
| 42fe194a5b | `libnsgif:ns-afl-gif/id:000532` | 1x30,4x3 | 32x32 | InvalidFrameDimensions |
| c0fc0e5690 | `libnsgif:ns-afl-gif/id:000553` | 0,6x2 | 32x32 | UndefinedDisposal |
| 7a203792c2 | `libnsgif:ns-afl-gif/id:000567` | 0 | 32x65312 | InvalidDimensions |
| bab60145c3 | `libnsgif:ns-afl-gif/id:000569` | 0,6 | 32x65312 | InvalidDimensions |
| ab6352166d | `libnsgif:ns-afl-gif/id:000572` | 0,6 | 32x65312 | InvalidDimensions |
| 7a466f0d07 | `libnsgif:ns-afl-gif/id:000578` | 1,2 | 32x65312 | InvalidDimensions |
| 9f30ffe291 | `libnsgif:ns-afl-gif/id:000599` | 0,1 | 32x65312 | InvalidDimensions |
| d3775088c5 | `libnsgif:ns-afl-gif/id:000604` | 11 | 17993x0 | InvalidDimensions |
| 19c7296471 | `libnsgif:ns-afl-gif/id:000610` | 0,6 | 32x65312 | InvalidDimensions |
| a4edea4ddf | `libnsgif:ns-afl-gif/id:000612` | 1 | 32x32 | InvalidFrameDimensions |
| 7cc0c0914f | `libnsgif:ns-afl-gif/id:000613` | 0,6x2 | 32x65312 | InvalidDimensions |
| 8b941c3a6b | `libnsgif:ns-afl-gif/id:000616` | 11 | 17993x0 | InvalidDimensions |
| f8f7205535 | `libnsgif:ns-afl-gif/id:000621` | 0 | 32x65312 | InvalidDimensions |
| 8c0229fbb9 | `libnsgif:ns-afl-gif/id:000623` | 0 | 32x65312 | InvalidDimensions |
| 2496b950e0 | `libnsgif:ns-afl-gif/id:000627` | 0,6 | 32x65312 | InvalidDimensions |
| 204e9ef488 | `libnsgif:ns-afl-gif/id:000631` | 0,6 | 32x65312 | InvalidDimensions |
| 11f94d4181 | `libnsgif:ns-afl-gif/id:000655` | 0,6 | 32x65312 | InvalidDimensions |
| d30c67397e | `libnsgif:ns-afl-gif/id:000658` | 1x2,2,6 | 32x65312 | InvalidDimensions |
| 1ea8f0981b | `libnsgif:ns-afl-gif/id:000663` | 1,2,6 | 32x65312 | InvalidDimensions |
| 13e739e61e | `libnsgif:ns-afl-gif/id:000666` | 0,1x2,6 | 32x32 | UndefinedDisposal |
| 2bc556ab97 | `libnsgif:ns-afl-gif/id:000668` | 0,6 | 32x65312 | InvalidDimensions |
| 98f877e69b | `image-gif:tests/crashtest/issue-117-out-of-range.gif` | 0 | 3x23850 | InvalidDimensions |
| 8863bf6bd6 | `image-gif:tests/crashtest/issue-96-small-codesize-2.gif` | 0 | 11390x12595 | InvalidDimensions |
| 1464bdf674 | `image-gif:tests/crashtest/issue-96-small-codesize.gif` | 0 | 185x0 | InvalidDimensions |
| 6c6bd4bf46 | `libgd:tests/fuzzing/corpus/gif/ossfuzz5700.gif` | 12 | 8224x8224 | InvalidDimensions |

## Method and reproduction

- Script: `gif_lzwmin.py` in this directory (`python3 -I gif_lzwmin.py --json OUT.json > tables.md`, about three minutes on one CPU; the incumbent runners must be present in `/tmp/claude-0/leniency-gifpng-bin`, built by `../gif/build-min.sh -j1`). The tables above are its output; the prose is hand-written from its derived-facts block. `gif_oracle.py` and `policy_oracle.py` are not edited: the 1-11 decoders replace `read_image_data` and `lzw_decode` inside the oracle module for one call.
- Hypothetical decoders. Checked: size 1-11, a literal code in 256..2^m-1 raises `ColorIndexOutOfRange`. Wrap: size 1-11, such a code yields index code mod 256. Everything else is the Strict oracle.
- The 99 examined files are those with a size outside 2-8 in the lenient scan (98) plus the one the Recommended policy walk reads an odd size from that the scan misses (`3e9b7e56ab`). Whether the range matters under Recommended was checked on all 1,407 files by running the policy oracle (Recommended with `Range1to11`) and recording every size byte it reads; only files where it reads a size outside 2-8 can change.
- Incumbents: gdk-pixbuf 2.42.12 `gdk_pixbuf_animation_new_from_file` ("gdk anim"), the loader forced to GIF with a whole-buffer write ("gdk forced"), and `image` 0.25.10 `GifDecoder::into_frames` using `gif` 0.14.2. Statuses and first-frame hashes for the 99 files equal those in `gif-incumbents.json` (0 mismatches).
