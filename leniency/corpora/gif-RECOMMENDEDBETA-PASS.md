# RecommendedBeta on the pinned real GIF corpus (Q9, PLAN task M0-corpus)

RecommendedBeta accepts 1,016 of 1,407 corpus files as whole streams (72.2%) and 1,027 as stills; Strict accepts 915 (65.0%) and 974. 101 files flip from rejected to accepted, 0 Strict-accepted files decode differently (obligation R), and on every file that RecommendedBeta and an incumbent both accept, the first frame equals the incumbent's (1,016 of 1,016 against gdk-pixbuf's animation path, 1,014 of 1,014 against its forced loader, 1,023 of 1,023 against image 0.25.10). The recording half of the Q9 gate is met; the data cannot support more than that (last section).

Corpus: the manifest `corpora/manifests/gif.MANIFEST.tsv` (SHA-256 `122e6ecacdbe081c7bf32209ede195833d2fcda4553381dd39695631b39ccf23`, checked by the script before it reads a file), 1,407 files, of which 1,318 carry a GIF signature, 326 are fuzzer-derived (path contains afl, fuzz or crash) and 303 are incidental assets (documentation, website and demo images; `gif_stats.py`'s path heuristic). Oracle: `gif/policy-draft/policy_oracle.py` (draft 0.2) with `PROFILES['Strict']` and `PROFILES['RecommendedBeta']` (minimum code size 1 accepted only for a one-pixel frame); "stream" means `decode_frames` ends at the Trailer, "still" means `decode_still` returns the first frame. No oracle run timed out. Per-file outcomes: `gif-recommendedbeta-pass.json` (names, error names and numbers only). Script: `gif_recommendedbeta_pass.py`.

## Acceptance

| subset | files | Strict stream | RecommendedBeta stream | Strict still | RecommendedBeta still |
|---|---|---|---|---|---|
| all | 1,407 | 915 (65.0%) | 1,016 (72.2%) | 974 | 1,027 |
| GIF signature | 1,318 | 915 (69.4%) | 1,016 (77.1%) | 974 | 1,027 |
| not fuzzer-derived | 1,081 | 904 (83.6%) | 996 (92.1%) | 962 | 1,009 |
| fuzzer-derived | 326 | 11 (3.4%) | 20 (6.1%) | 12 | 18 |
| incidental assets | 303 | 283 (93.4%) | 303 (100.0%) | 296 | 303 |

Among the 1,074 files that are not fuzzer-derived and have a GIF signature (mostly decoder test inputs), Strict accepts 904 (84.2%) and RecommendedBeta 996 (92.7%). The earlier statistics (`gif-STATS.md`) say Strict accepts 910 streams and 969 stills; the current oracle accepts 915 and 974. The five extra files are `treeview-sprite.gif` (18x8888), `max-height.gif`, `max-width.gif`, `100x32768.gif` and `32768x100.gif`: draft 0.2 dropped the per-side cap of 8,192 (Q8) and they are well-formed. The Strict implementation used here equals `gif_oracle.py` on those five files and on a random sample of 150 others (identical end state, error and frame hashes; the bridge in `validate.py` covers the vectors).

101 files are accepted as streams by RecommendedBeta and rejected by Strict (53 as stills); 9 of the 101 are fuzzer-derived and 20 are incidental assets. Strict's first errors on those 101 files: DataAfterEndOfInformation 23, DuplicateGraphicControl 1, FrameOutsideScreen 21, GraphicControlWithoutGraphic 3, InvalidApplicationExtension 3, InvalidLzwMinCodeSize 5, MissingEndOfInformation 12, MissingTrailer 2, TooManyPixels 1, TrailingBytes 24, UndefinedDisposal 6. RecommendedBeta accepts every file Strict accepts (0 reversals).

## Which fields fired

A field "fired" on a file when reverting that field alone to its Strict value changes the file's outcome (ablation over the ten non-Strict fields, `gif_recommendedbeta_pass.py`). Of the 101 stream flips, 95 need one field, 5 two and 1 three; no flip is unexplained by single fields. Columns: stream flips (still flips), of which not fuzzer-derived, incidental assets, the Strict error the field replaced, and how many of the stream-flip files gdk-pixbuf's animation path and image accept as whole streams.

| field | stream flips (still) | not fuzz | incidental | error replaced | gdk anim / image accept | examples |
|---|---|---|---|---|---|---|
| `frame_bounds` | 21 (12) | 20 | 1 | FrameOutsideScreen 21 | 20 / 20 | `congrats.gif`, `animated_transparent_loop_frames_out_of_bounds.gif`, `randPixelsAnim.gif` |
| `disposal_undefined` | 6 (5) | 3 | 1 | UndefinedDisposal 6 | 6 / 6 | `backblue.gif`, `invalid-disposal-method.gif`, `invalid-disposal-method-1.gif` |
| `gce_duplicate` | 1 (1) | 1 | 0 | DuplicateGraphicControl 1 | 1 / 1 | `multiple-graphic-controls.gif` |
| `gce_consumers` | 0 (0) | 0 | 0 | none (replaced `UnsupportedGraphicControlTarget` in 3 fuzzer-derived files, which still fail at `UnknownBlock`) | n/a | |
| `gce_orphan` | 5 (0) | 2 | 0 | GraphicControlWithoutGraphic 5 | 2 / 2 | `cogs.gif`, `1509998.gif`, `threeframes-start.gif` |
| `ext_sizes` | 3 (3) | 3 | 0 | InvalidApplicationExtension 3 | 3 / 3 | `gradient.gif`, `apple.gif`, `short-app-extension-string.gif` |
| `lzw_min` | 5 (5) | 5 | 0 | InvalidLzwMinCodeSize 5 | 5 / 5 | `0.gif`, `max-codes.gif`, `set_hsts.gif` |
| `lzw_tail` | 36 (27) | 35 | 14 | DataAfterEndOfInformation 23, MissingEndOfInformation 12, TooManyPixels 1 | 36 / 34 | `tile.gif`, `orientation_widget_demo.gif`, `orientation_widget_states.gif` |
| `missing_trailer` | 3 (0) | 1 | 0 | MissingTrailer 3 | 0 / 0 | `no_trailer.gif`, `1509998.gif`, `threeframes-start.gif` |
| `trailing` | 28 (0) | 25 | 4 | TrailingBytes 28 | 25 / 25 | `spinner.gif`, `BeanEater-1s-200px.gif`, `loader.gif` |

Notes on the table.

- `gce_consumers` never decides a file. The three files whose Strict error is `UnsupportedGraphicControlTarget` are libnsgif mutants and RecommendedBeta rejects them at `UnknownBlock`. Q7(c) rests on decoder sources, not on a corpus file.
- A field can fire and the file still be rejected later. Counting by the Strict error a field replaced (not by ablation): `disposal_undefined` 34 files (29 then fail `NoColorTable`, 4 `TruncatedImageData`, 1 `UnknownBlock`), `frame_bounds` 21, `ext_sizes` 8, `gce_consumers` 3, `lzw_min` 3, `lzw_tail` 1. These are all test or fuzzer inputs with a second defect.
- `lzw_min` flips exactly the five files of the Q6 analysis (`set_hsts.gif` and chromium `0.gif` at size 1, `lzw-code-three-subblocks.gif` at 10, `max-codes.gif` and `issue_2811.gif` at 11).
- `missing_trailer` has no gdk-pixbuf support (it rejects all 3; image decodes the first frame and reports an error after it). It stays a design choice (Q1).
- Recovery events (OPTIONS.md): 22 accepted streams carry `recovered` frames (32 frames): `E1` (frame clipped) in 21 streams and `E4` (surplus pixels) in 1 (`pixel-data-too-much-good-lzw.gif`). `decode_still` returns a recovered first frame for 13 files. `E2` and `E3` cannot occur under RecommendedBeta. `E1` examples: `congrats.gif`, `animated_transparent_loop_frames_out_of_bounds.gif`, `randPixelsAnim.gif`. One rejected stream has a recovered frame before its error. `lzw_tail` sets `recovered` only for surplus pixels, not for a missing or early End of Information; no other field sets it.

## Files RecommendedBeta still rejects

391 files: 306 fuzzer-derived and 85 not (78 of those have a GIF signature); 89 of the 391 have no GIF signature at all. No incidental asset is rejected. First error as a stream (top 15 of 18 kinds):

| first error | files | examples |
|---|---|---|
| `NoColorTable` | 90 | `skbug5883.gif`, `empty-palette.gif` |
| `UnsupportedVersion` | 79 | `bad_magic.gif`, `id:000040,orig:id:000471,src:000005,op:havoc,rep:4,+cov.gif` |
| `InvalidDimensions` | 46 | `zero-dims.gif`, `xOffsetTooBig.gif` |
| `TruncatedImageData` | 29 | `invalid.4.gif`, `hippopotamus.interlaced.truncated.gif` |
| `InvalidLzwMinCodeSize` | 25 | `overflow-codes-max.gif`, `overflow-codes.gif` |
| `UnknownBlock` | 22 | `bad-trailer.gif`, `broken.gif` |
| `InvalidLzwCode` | 18 | `bad-code.gif`, `bad-initial-code.gif` |
| `InvalidFrameDimensions` | 16 | `skbug5887.gif`, `image-zero-height.gif` |
| `TooFewPixels` | 13 | `gif-frame-outside-logical-screen.gif`, `first-frame-has-greater-size-than-screen-size.gif` |
| `OutputTooLarge` | 13 | `size-failure.gif`, `large-size-image-crash.gif` |
| `InvalidGraphicControl` | 9 | `bad_lzw_data.gif`, `ossfuzz6274.gif` |
| `BadSignature` | 8 | `aboutHeader.gif`, `bkgd_yellow.gif` |
| `ColorIndexOutOfRange` | 7 | `alphabetAnim.gif`, `invalid-colors.gif` |
| `TruncatedExtension` | 5 | `trunc-ext-subblock.gif`, `id:000010,orig:id:000547,src:000080,op:havoc,rep:2.gif` |
| `TruncatedColorTable` | 4 | `trunc-gct.gif`, `id:000029,orig:id:000453,src:000005,op:flip1,pos:10,+cov.gif` |

Five of the 25 `InvalidLzwMinCodeSize` files are libnsgif `ns-afl-gif` fuzzer mutants that carry minimum code size 1 on a frame of more than one pixel. Before the size-1 rule (RecommendedBeta accepts `m = 1` only for a one-pixel frame, Q6 refinement) they failed later, four at `TruncatedImageData` and one at `TooFewPixels`; gdk-pixbuf's two paths and image reject all five either way, and no accept/reject outcome changed.

`NoColorTable` (90) is high because RecommendedBeta forgives `UndefinedDisposal` in 29 fuzzer-derived files and `FrameOutsideScreen` in 3 more, and then reaches the missing colour table; Strict reports the other 58 there.

## Obligation R on the corpus

Every Strict-accepted file decodes identically under RecommendedBeta: 974 stills, 915 streams and 3,538 frames (the prefix form, which also covers frames before a Strict error) compared field by field (RGBA of the whole canvas, rectangle, delay, disposal, user input, transparent index, interlace, loop count), `recovered` false on every frame. Violations: 0. This extends `validate.py`'s 163 inputs (synthetic) to real files.

## Agreement with the incumbents

Incumbent runners: gdk-pixbuf 2.42.12 (`gdk-anim gif anim`: animation path; `gdk-anim gif forced-whole`: loader forced to GIF, whole buffer, first frame) and image 0.25.10 / gif 0.14.2 (`rust-image gif:image`, `build-min.sh`). The runners were run once for this pass, because the old JSON lacks the image runner's `tail_error` (an error after its first frame; 22 files); statuses and first-frame hashes then equalled `gif-incumbents.json` on 1,407 of 1,407 files for all three runners. The rerun after the size-1 rule changed only the oracle half: the incumbent results (statuses, frame counts, first-frame hashes and `tail_error`) are the stored ones, since neither the runners nor the corpus changed. First frames are compared with RGB zeroed where alpha is 0. Runners stop at 64 frames, so a whole-stream "accept" means the first 64 frames.

| incumbent | first frame, both accept | equal | RB accepts, incumbent rejects (stream / still) | incumbent accepts, RB rejects (stream / still) |
|---|---|---|---|---|
| gdk-pixbuf animation path | 1,016 | 1,016 | 9 / 11 | 40 / 31 |
| gdk-pixbuf forced loader | 1,014 | 1,014 | 9 / 13 | 32 / 25 |
| image 0.25.10 (stream: no tail error) | 1,023 | 1,023 | 15 / 4 | 4 / 4 |

Whole streams: 1,001 files are accepted by RecommendedBeta, the animation path and image together. 9 files are accepted by RecommendedBeta and rejected by both: `no-data.gif`, `no-frames.gif`, `empty.gif` (no image block; RecommendedBeta's stream is empty and its still is `NoImage`), `no_trailer.gif`, `1509998.gif`, `threeframes-start.gif` (missing trailer; `missing_trailer=AtEof` and `gce_orphan`), `856616.gif` and two libnsgif mutants. 4 files are rejected by RecommendedBeta and accepted by both: `invalid-colors.gif`, `unitialized_memory_read.gif`, `alphabetAnim.gif` (out-of-range index, Q5) and chromium `error_frame.gif` (an invalid code in a later frame, Q4). Where gdk-pixbuf accepts and RecommendedBeta rejects, the reasons are `InvalidLzwCode`, `TooFewPixels`, `ColorIndexOutOfRange`, `InvalidGraphicControl`, `UnknownBlock` and `NoColorTable`: Q4, Q5 and the Strict structure checks that RecommendedBeta keeps (gdk-pixbuf skips an unknown block byte and draws partial frames).

## The 20 incidental assets that Strict rejects

All 20 are accepted by RecommendedBeta as streams and as stills, and by gdk-pixbuf's animation path. image decodes the first frame of all 20 and accepts 19 as whole streams (`reset-no-gce.gif` fails after its first frame). The five Strict errors are those listed in `gif-STATS-interpretation.md`.

| file | Strict error | field |
|---|---|---|
| `ImageMagick/ImageMagick:PerlMagick/demo/tile.gif` | DataAfterEndOfInformation | `lzw_tail` |
| `ImageMagick/ImageMagick:PerlMagick/t/x11/congrats.gif` | FrameOutsideScreen | `frame_bounds` (`E1`) |
| `ImageMagick/ImageMagick:www/backblue.gif` | UndefinedDisposal | `disposal_undefined` |
| `Kitware/VTK:Documentation/release/imgs/9.3/orientation_widget_demo.gif` | DataAfterEndOfInformation | `lzw_tail` |
| `Kitware/VTK:Documentation/release/imgs/9.3/orientation_widget_states.gif` | DataAfterEndOfInformation | `lzw_tail` |
| `WebKit/WebKit:Source/ThirdParty/skia/demos.skia.org/demos/mesh2d/BeanEater-1s-200px.gif` | TrailingBytes | `trailing` |
| `chromium/chromium:docs/ui/ui_devtools/images/edit_property.gif` | DataAfterEndOfInformation | `lzw_tail` |
| `chromium/chromium:docs/ui/ui_devtools/images/expand_elements.gif` | DataAfterEndOfInformation | `lzw_tail` |
| `chromium/chromium:docs/ui/ui_devtools/images/hover_and_inspect.gif` | DataAfterEndOfInformation | `lzw_tail` |
| `chromium/chromium:docs/ui/ui_devtools/images/hovering_distances.gif` | DataAfterEndOfInformation | `lzw_tail` |
| `chromium/chromium:docs/ui/ui_devtools/images/hovering_over_elements.gif` | DataAfterEndOfInformation | `lzw_tail` |
| `chromium/chromium:docs/ui/ui_devtools/images/ui_element_tree_search.gif` | DataAfterEndOfInformation | `lzw_tail` |
| `imagej/ImageJ:images/microscope.gif` | TrailingBytes | `trailing` |
| `mirrorer/giflib:pic/porsche.gif` | TrailingBytes | `trailing` |
| `mozilla/gecko-dev:editor/composer/res/grabber.gif` | DataAfterEndOfInformation | `lzw_tail` |
| `mozilla/gecko-dev:toolkit/components/pdfjs/content/web/images/loading-icon.gif` | TrailingBytes | `trailing` |
| `opencv/opencv:doc/images/camshift_face.gif` | DataAfterEndOfInformation | `lzw_tail` |
| `opencv/opencv:doc/images/meanshift_face.gif` | DataAfterEndOfInformation | `lzw_tail` |
| `opencv/opencv:doc/py_tutorials/py_imgproc/py_houghlines/images/houghlinesdemo.gif` | MissingEndOfInformation | `lzw_tail` |
| `web-platform-tests/wpt:gif/reset-no-gce.gif` | MissingEndOfInformation | `lzw_tail` |

## What the pass shows and cannot show; the Q9 gate

It shows which Strict errors files in this corpus hit and which field replaces each, that no relaxation rejects anything Strict accepts, that RecommendedBeta's result agrees with both incumbents' first frame on every file all three accept, and that no file exists on which RecommendedBeta and an incumbent both accept and show different first frames.

It cannot show how common these defects are on the web. Every file comes from a test suite, a documentation tree or a demo directory (no crawl); 326 files (23%) are fuzzer-derived; 1,081 are not, of which 303 are incidental assets and the other 778 have a test-like token in their path; the corpus spans 32 repositories, the incidental label is a path heuristic, and the encoder mix is unknown. A rate such as "303 of 303 incidental assets accepted" is a property of this sample. Further limits:

- Only the first frame is compared with the incumbents. Later frames, delays, loop counts and the compositing of disposal 3 are not compared in this pass.
- The subject is the Python oracle. No Rust exists yet; M1's conformance run repeats this pass against the implementation.
- Four fields have incidental-asset support: `lzw_tail` (14 files), `trailing` (4), `frame_bounds` (1), `disposal_undefined` (1). Five have test inputs only: `lzw_min` (5 images that both incumbents decode), `gce_duplicate` (1 file, Wuffs artificial), `ext_sizes` (3 chromium test files), `gce_orphan` (5 files, 2 not fuzzer-derived) and `missing_trailer` (3 files, 1 not fuzzer-derived, no gdk-pixbuf support). `gce_consumers` has none.

Gate ("block the RecommendedBeta freeze until a real-corpus pass records which Strict errors real files hit and each field's outcomes"): the recording condition is met, because the pass ran on the hash-pinned corpus and records per file the Strict and RecommendedBeta outcome, the fields that fired, the replaced error and both incumbents' outcomes, summarised above. M0-corpus also says the freeze needs the record to be reviewed, which has not happened, and the data do not decide whether the six fields without incidental-asset support (`lzw_min`, `gce_duplicate`, `ext_sizes`, `gce_orphan`, `missing_trailer`, `gce_consumers`) are worth keeping: each costs proof effort. That is the owner's call (Q1, Q7), now with the counts above.

## Reproduce

```
cd leniency/corpora
python3 -I gif_recommendedbeta_pass.py oracle     --cache DIR   # resumable (`--budget SECONDS` per call); about 670 s of oracle time on one core
python3 -I gif_recommendedbeta_pass.py incumbents --cache DIR   # runners from ../gif/build-min.sh in /tmp/claude-0/leniency-gifpng-bin
python3 -I gif_recommendedbeta_pass.py report     --cache DIR   # writes gif-recommendedbeta-pass.json
```
