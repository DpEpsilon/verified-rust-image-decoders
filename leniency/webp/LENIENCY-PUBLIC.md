# WebP (image/webp) leniency matrix (public: synthetic inputs, crash rows excluded)

Seed 731; 6979 inputs (30 unmutated synthetic seeds from 30 distinct seed files, the rest mutations); 5537 mutations changed the acceptance or cross-decoder pixel relation relative to their seed; 180.3 s comparison pass, 238.6 s total including witness minimisation.

All inputs are synthetic (produced by the seed tool in this directory from closed-form pixel patterns, then mutated); no third-party file is involved. Inputs on which any decoder crashed or timed out are excluded from this public summary.

## Decoders

Column | Implementation group (one vote per group) | Status counts | What it is
--- | --- | --- | ---
glycin | image-webp | error 3240, ok 3739 | glycin-image-rs `ImgLoader` for `image/webp` (image::WebPDecoder, image-webp 0.2.4), frames pulled with `specific_frame` until `NoMoreFrames`
image-webp | image-webp | error 3240, ok 3739 | `image::codecs::webp::WebPDecoder` called directly (same code as glycin; `into_frames()` for animation)
libwebp-simple | libwebp | error 2864, ok 1277, skip 2838 | system libwebp 1.5.0 `WebPDecodeRGBA` (still images; animated files unsupported by this API)
libwebp-anim | libwebp | error 4398, ok 2581 | system libwebp 1.5.0 `WebPAnimDecoder` (demux validation + canvas composition, every frame; also used for stills)

Unavailable (not counted as rejecting):

- **gdk-pixbuf WebP module**: not installed (module list: ani bmp gif icns ico pnm qtif tga tiff xbm xpm)
- **`webp` crate**: wraps libwebp-sys, i.e. the same libwebp code: not an independent vote, not built
- **independent Rust WebP decoder other than image-webp**: none found among the crates reachable in this survey; see SURVEY.md

Pixel comparison: exact RGBA8 SHA-256 first; otherwise the maximum absolute channel difference over all frames (alpha included) decides: at most 2 counts as within tolerance (documented per format), larger is a pixel disagreement. Differences confined to RGB under alpha 0 on both sides are reported separately (`transparent-rgb`). Dimension and frame-count differences are never tolerated.

## Classification

Classification | Count
--- | ---:
all-reject | 2875
accept-vs-reject | 1824
all-accept-identical-pixels | 1249
accept-different-pixels | 739
all-accept-within-tolerance | 292

## Seeds (unmutated)

Seed | Class | glycin | image-webp | libwebp-simple | libwebp-anim
--- | --- | --- | --- | --- | ---
anim-ll-2f-8x8 | all-accept-identical-pixels | 8×8×2f `3c0d1396` | 8×8×2f `3c0d1396` | n/a | 8×8×2f `3c0d1396`
anim-ll-3f-16x16 | all-accept-within-tolerance | 16×16×3f `f2304f5f` | 16×16×3f `f2304f5f` | n/a | 16×16×3f `f2304f5f`
anim-ll-alpha-3f-20x12 | accept-different-pixels | 20×12×3f `2239df38` | 20×12×3f `2239df38` | n/a | 20×12×3f `2239df38`
anim-lossy-3f-16x16 | all-accept-identical-pixels | 16×16×3f `fb7b3c25` | 16×16×3f `fb7b3c25` | n/a | 16×16×3f `fb7b3c25`
anim-mixed-alpha-4f-24x16 | accept-different-pixels | 24×16×4f `2c50dd97` | 24×16×4f `2c50dd97` | n/a | 24×16×4f `2c50dd97`
ll-16x16-exact-alpha | all-accept-identical-pixels | 16×16 `d3d53d4b` | 16×16 `d3d53d4b` | 16×16 `d3d53d4b` | 16×16 `d3d53d4b`
ll-16x16-nearlossless | all-accept-identical-pixels | 16×16 `579eab5e` | 16×16 `579eab5e` | 16×16 `579eab5e` | 16×16 `579eab5e`
ll-16x16-rgb | all-accept-identical-pixels | 16×16 `2e67fad2` | 16×16 `2e67fad2` | 16×16 `2e67fad2` | 16×16 `2e67fad2`
ll-16x16-rgba | all-accept-identical-pixels | 16×16 `4badb2c2` | 16×16 `4badb2c2` | 16×16 `4badb2c2` | 16×16 `4badb2c2`
ll-17x13-rgba | all-accept-identical-pixels | 17×13 `f13b82fd` | 17×13 `f13b82fd` | 17×13 `f13b82fd` | 17×13 `f13b82fd`
ll-1x1 | all-accept-identical-pixels | 1×1 `e3820096` | 1×1 `e3820096` | 1×1 `e3820096` | 1×1 `e3820096`
ll-24x8-m0 | all-accept-identical-pixels | 24×8 `22e0a2ef` | 24×8 `22e0a2ef` | 24×8 `22e0a2ef` | 24×8 `22e0a2ef`
ll-2x2-rgb | all-accept-identical-pixels | 2×2 `384f7c86` | 2×2 `384f7c86` | 2×2 `384f7c86` | 2×2 `384f7c86`
ll-32x32-m6 | all-accept-identical-pixels | 32×32 `f27b17bc` | 32×32 `f27b17bc` | 32×32 `f27b17bc` | 32×32 `f27b17bc`
lossy-16x16-nofilter | all-accept-identical-pixels | 16×16 `ae1525f8` | 16×16 `ae1525f8` | 16×16 `ae1525f8` | 16×16 `ae1525f8`
lossy-16x16-q75 | all-accept-identical-pixels | 16×16 `66337ad2` | 16×16 `66337ad2` | 16×16 `66337ad2` | 16×16 `66337ad2`
lossy-16x16-sharp | all-accept-identical-pixels | 16×16 `bb04a0ea` | 16×16 `bb04a0ea` | 16×16 `bb04a0ea` | 16×16 `bb04a0ea`
lossy-16x16-strongfilter | all-accept-identical-pixels | 16×16 `a563ed57` | 16×16 `a563ed57` | 16×16 `a563ed57` | 16×16 `a563ed57`
lossy-17x13-q50 | all-accept-identical-pixels | 17×13 `1aa46bb4` | 17×13 `1aa46bb4` | 17×13 `1aa46bb4` | 17×13 `1aa46bb4`
lossy-1x1 | all-accept-identical-pixels | 1×1 `e3820096` | 1×1 `e3820096` | 1×1 `e3820096` | 1×1 `e3820096`
lossy-33x33-q10 | all-accept-identical-pixels | 33×33 `a1f67453` | 33×33 `a1f67453` | 33×33 `a1f67453` | 33×33 `a1f67453`
lossy-64x48-parts | all-accept-identical-pixels | 64×48 `627d030c` | 64×48 `627d030c` | 64×48 `627d030c` | 64×48 `627d030c`
lossy-alpha-16x16 | all-accept-identical-pixels | 16×16 `ef0ffe4b` | 16×16 `ef0ffe4b` | 16×16 `ef0ffe4b` | 16×16 `ef0ffe4b`
lossy-alpha-filter-16x16 | all-accept-identical-pixels | 16×16 `9b1fa175` | 16×16 `9b1fa175` | 16×16 `9b1fa175` | 16×16 `9b1fa175`
lossy-alpha-q20-24x24 | all-accept-identical-pixels | 24×24 `95276b28` | 24×24 `95276b28` | 24×24 `95276b28` | 24×24 `95276b28`
lossy-alpha-raw-17x13 | all-accept-identical-pixels | 17×13 `5348a174` | 17×13 `5348a174` | 17×13 `5348a174` | 17×13 `5348a174`
meta-anim-xmp | all-accept-within-tolerance | 16×16×3f `f2304f5f` | 16×16×3f `f2304f5f` | n/a | 16×16×3f `f2304f5f`
meta-ll-icc | all-accept-identical-pixels | 16×16 `4badb2c2` | 16×16 `4badb2c2` | 16×16 `4badb2c2` | 16×16 `4badb2c2`
meta-lossy-alpha-exif | all-accept-identical-pixels | 16×16 `ef0ffe4b` | 16×16 `ef0ffe4b` | 16×16 `ef0ffe4b` | 16×16 `ef0ffe4b`
meta-lossy-icc-exif-xmp | all-accept-identical-pixels | 16×16 `66337ad2` | 16×16 `66337ad2` | 16×16 `66337ad2` | 16×16 `66337ad2`

## Who accepts what (accept-vs-reject partitions)

Accepting decoders | Rejecting | Count | Top rules | Witness | glycin | image-webp | libwebp-simple | libwebp-anim
--- | --- | ---: | --- | --- | --- | --- | --- | ---
glycin, image-webp | libwebp-anim | 723 | field:anmf_x 78; field:anmf_y 72; field:vp8l_signature 70; field:VP8L_size 64 | [lenient-01-6273.webp](witnesses/lenient-01-6273.webp) | 20×12 `2239df38` | 20×12 `2239df38` | n/a | reject `AnimDecoderNew`
glycin, image-webp | libwebp-simple, libwebp-anim | 606 | riff-size-delta 213; field:riff_size 148; field:VP8_size 47; chunk-size-delta 47 | [lenient-02-456.webp](witnesses/lenient-02-456.webp) | 1×1 `e3820096` | 1×1 `e3820096` | reject `NOT_ENOUGH_DATA` | reject `AnimDecoderNew`
libwebp-anim | glycin, image-webp | 200 | field:anmf_w_minus1 42; field:anmf_h_minus1 42; field:VP8L_fourcc 35; chunk-size-delta 15 | [lenient-03-32.webp](witnesses/lenient-03-32.webp) | reject `ExpectedError` | reject `Format error decoding ` | n/a | 8×8 `b34e94dd`
glycin, image-webp, libwebp-simple | libwebp-anim | 130 | chunk-insert 26; padding-extra 19; chunk-duplicate 19; vp8x-flag-bit 17 | [lenient-04-1303.webp](witnesses/lenient-04-1303.webp) | 1×1 `e3820096` | 1×1 `e3820096` | 1×1 `e3820096` | reject `AnimDecoderNew`
libwebp-simple, libwebp-anim | glycin, image-webp | 101 | field:ALPH_fourcc 35; field:EXIF_fourcc 14; vp8x-flag-bit 12; wrap-vp8x 8 | [lenient-05-1941.webp](witnesses/lenient-05-1941.webp) | reject `ExpectedError` | reject `Format error decoding ` | 1×1 `e3820096` | 1×1 `e3820096`
libwebp-simple | glycin, image-webp, libwebp-anim | 64 | field:VP8L_size 35; field:VP8_size 13; field:EXIF_size 7; chunk-size-delta 5 | [lenient-06-1533.webp](witnesses/lenient-06-1533.webp) | reject `ExpectedError` | reject `Format error decoding ` | 1×1 `e3820096` | reject `AnimDecoderNew`

## Provenance clusters (mutation rule; changed-relation counts)

A mutation joins a cluster only if it changed acceptance or the cross-decoder pixel relation relative to its unmutated seed. The row shows the most frequent outcome pattern.

Rule (changed; patterns) | Witness | glycin | image-webp | libwebp-simple | libwebp-anim
--- | --- | --- | --- | --- | ---
truncation (314; 14) | [rule-01-6300.webp](witnesses/rule-01-6300.webp) | reject `ExpectedError` | reject `failed to fill whole b` | reject `NOT_ENOUGH_DATA` | reject `AnimDecoderNew`
riff-size-delta (276; 17) | [rule-02-533.webp](witnesses/rule-02-533.webp) | 16×16 `bb04a0ea` | 16×16 `bb04a0ea` | reject `BITSTREAM_ERROR` | reject `AnimDecoderNew`
alph-header-byte (227; 4) | [rule-03-30.webp](witnesses/rule-03-30.webp) | reject `ExpectedError` | reject `Format error decoding ` | reject `OUT_OF_MEMORY` | reject `decode`
vp8l-header-bits (217; 38) | [rule-04-5230.webp](witnesses/rule-04-5230.webp) | reject `ExpectedError` | reject `Format error decoding ` | reject `BITSTREAM_ERROR` | reject `AnimDecoderNew`
field:riff_size (210; 23) | [rule-05-1088.webp](witnesses/rule-05-1088.webp) | 16×16 `bb04a0ea` | 16×16 `bb04a0ea` | reject `BITSTREAM_ERROR` | reject `AnimDecoderNew`
bit-flip (196; 30) | [rule-06-4974.webp](witnesses/rule-06-4974.webp) | reject `ExpectedError` | reject `Format error decoding ` | reject `BITSTREAM_ERROR` | reject `AnimDecoderNew`
byte-replace (185; 29) | [rule-07-1724.webp](witnesses/rule-07-1724.webp) | reject `ExpectedError` | reject `Format error decoding ` | reject `BITSTREAM_ERROR` | reject `AnimDecoderNew`
riff-magic (180; 1) | [rule-08-215.webp](witnesses/rule-08-215.webp) | reject `ExpectedError` | reject `Format error decoding ` | reject `BITSTREAM_ERROR` | reject `AnimDecoderNew`
field:VP8L_fourcc (175; 13) | [rule-09-314.webp](witnesses/rule-09-314.webp) | reject `ExpectedError` | reject `Format error decoding ` | reject `BITSTREAM_ERROR` | reject `AnimDecoderNew`
field:vp8l_dims_flags (175; 38) | [rule-10-5115.webp](witnesses/rule-10-5115.webp) | reject `ExpectedError` | reject `Format error decoding ` | reject `BITSTREAM_ERROR` | reject `AnimDecoderNew`
field:vp8l_signature (175; 10) | [rule-11-272.webp](witnesses/rule-11-272.webp) | reject `ExpectedError` | reject `Format error decoding ` | reject `BITSTREAM_ERROR` | reject `AnimDecoderNew`
field:VP8L_size (173; 27) | [rule-12-1227.webp](witnesses/rule-12-1227.webp) | reject `ExpectedError` | reject `Format error decoding ` | n/a | reject `AnimDecoderNew`
chunk-insert (170; 20) | [rule-13-1574.webp](witnesses/rule-13-1574.webp) | reject `ExpectedError` | reject `Format error decoding ` | reject `BITSTREAM_ERROR` | reject `AnimDecoderNew`
chunk-size-delta (151; 37) | [rule-14-107.webp](witnesses/rule-14-107.webp) | reject `ExpectedError` | reject `Format error decoding ` | reject `BITSTREAM_ERROR` | reject `AnimDecoderNew`
field:ANMF_fourcc (126; 11) | #737 | 16×16×2f `46fdd627` | 16×16×2f `46fdd627` | n/a | 16×16×2f `2974ed8f`
field:ANMF_size (126; 21) | #1378 | reject `ExpectedError` | reject `Format error decoding ` | n/a | reject `AnimDecoderNew`
field:anmf_h_minus1 (126; 12) | #696 | 16×16 `fb7b3c25` | 16×16 `fb7b3c25` | n/a | 16×16×3f `fb7b3c25`
field:anmf_w_minus1 (126; 12) | #1442 | reject `ExpectedError` | reject `Format error decoding ` | n/a | 16×16×3f `fb7b3c25`
field:VP8_fourcc (119; 4) | #58 | reject `ExpectedError` | reject `Format error decoding ` | reject `BITSTREAM_ERROR` | reject `AnimDecoderNew`
field:vp8_frame_tag (119; 8) | #761 | reject `ExpectedError` | reject `Format error decoding ` | reject `BITSTREAM_ERROR` | reject `AnimDecoderNew`
field:vp8_height_scale (119; 24) | #986 | reject `ExpectedError` | reject `Format error decoding ` | reject `NOT_ENOUGH_DATA` | reject `decode`
field:vp8_start_code (119; 4) | #139 | reject `ExpectedError` | reject `Format error decoding ` | reject `BITSTREAM_ERROR` | reject `AnimDecoderNew`
field:vp8_width_scale (119; 22) | #75 | reject `ExpectedError` | reject `Format error decoding ` | reject `NOT_ENOUGH_DATA` | reject `decode`
field:anmf_x (118; 9) | #1494 | reject `ExpectedError` | reject `Format error decoding ` | n/a | reject `AnimDecoderNew`
field:VP8_size (116; 17) | #189 | reject `ExpectedError` | reject `failed to fill whole b` | reject `BITSTREAM_ERROR` | reject `AnimDecoderNew`
field:anmf_y (112; 9) | #639 | reject `ExpectedError` | reject `Format error decoding ` | n/a | reject `AnimDecoderNew`
field:VP8X_fourcc (91; 1) | #503 | reject `ExpectedError` | reject `Format error decoding ` | reject `BITSTREAM_ERROR` | reject `AnimDecoderNew`
field:VP8X_size (91; 2) | #1024 | reject `ExpectedError` | reject `Format error decoding ` | reject `BITSTREAM_ERROR` | reject `AnimDecoderNew`
field:vp8x_flags (87; 16) | #3110 | reject `ExpectedError` | reject `Format error decoding ` | n/a | reject `AnimDecoderNew`
field:vp8x_canvas_h_minus1 (84; 26) | #647 | reject `ExpectedError` | reject `Format error decoding ` | reject `BITSTREAM_ERROR` | reject `AnimDecoderNew`
field:vp8x_canvas_w_minus1 (83; 26) | #1759 | reject `ExpectedError` | reject `Format error decoding ` | reject `BITSTREAM_ERROR` | reject `AnimDecoderNew`
vp8-dimension (83; 16) | #2350 | reject `ExpectedError` | reject `Format error decoding ` | reject `NOT_ENOUGH_DATA` | reject `decode`
vp8-frame-tag (74; 12) | #1156 | reject `ExpectedError` | reject `failed to fill whole b` | reject `BITSTREAM_ERROR` | reject `AnimDecoderNew`
vp8x-flag-bit (69; 19) | #3615 | 16×16 `9b1fa175` | 16×16 `9b1fa175` | 16×16 `9b1fa175` | reject `AnimDecoderNew`
wrap-vp8x (65; 10) | #3776 | reject `ExpectedError` | reject `Format error decoding ` | reject `BITSTREAM_ERROR` | reject `AnimDecoderNew`
padding-extra (64; 22) | #2860 | reject `ExpectedError` | reject `Format error decoding ` | n/a | reject `AnimDecoderNew`
chunk-delete (52; 12) | #1215 | reject `ExpectedError` | reject `failed to fill whole b` | reject `BITSTREAM_ERROR` | reject `AnimDecoderNew`
field:anmf_flags (51; 8) | #66 | 20×12×3f `2239df38` | 20×12×3f `2239df38` | n/a | 20×12×3f `2239df38`
chunk-duplicate (43; 12) | #2661 | 16×16 `9b1fa175` | 16×16 `9b1fa175` | 16×16 `9b1fa175` | reject `AnimDecoderNew`
field:ANIM_fourcc (42; 1) | #369 | reject `ExpectedError` | reject `Format error decoding ` | n/a | reject `AnimDecoderNew`

## Same acceptance, different pixels or animation

Class | Differing decoder pairs | Count | Max abs diff (witness) | Top rules | Witness | glycin | image-webp | libwebp-simple | libwebp-anim
--- | --- | ---: | --- | --- | --- | --- | --- | --- | ---
all-accept-within-tolerance | glycin~libwebp-anim: within-tol; image-webp~libwebp-anim: within-tol | 292 | 1 | field:anmf_duration 42; field:anmf_flags 38; field:ANMF_fourcc 28 | [pixel-01-926.webp](witnesses/pixel-01-926.webp) | 8×8×2f `a63a2ebd` | 8×8×2f `a63a2ebd` | n/a | 8×8×2f `3c0d1396`
accept-different-pixels | glycin~image-webp: different-pixels; image-webp~libwebp-anim: different-pixels | 291 | 255 | field:anmf_duration 49; field:anmf_flags 27; field:ANMF_fourcc 21 | [pixel-02-1827.webp](witnesses/pixel-02-1827.webp) | 20×12×2f `2239df38` | 20×12×2f `2239df38` | n/a | 20×12×2f `2239df38`
accept-different-pixels | glycin~libwebp-anim: different-frame-count; image-webp~libwebp-anim: different-frame-count | 280 | 0 | field:anmf_h_minus1 63; field:anmf_w_minus1 63; field:ANMF_fourcc 35 | [pixel-03-197.webp](witnesses/pixel-03-197.webp) | 8×8 `3c0d1396` | 8×8 `3c0d1396` | n/a | 8×8×2f `3c0d1396`
accept-different-pixels | glycin~image-webp: different-pixels; glycin~libwebp-anim: different-frame-count; image-webp~libwebp-anim: different-frame-count | 81 | 255 | field:anmf_w_minus1 21; field:anmf_h_minus1 21; chunk-size-delta 8 | [pixel-04-133.webp](witnesses/pixel-04-133.webp) | 20×12×2f `2239df38` | 20×12×2f `2239df38` | n/a | 20×12×3f `2239df38`
accept-different-pixels | glycin~image-webp: different-pixels; glycin~libwebp-anim: within-tol; image-webp~libwebp-anim: different-pixels | 25 | 255 | field:anmf_flags 25 | [pixel-05-66.webp](witnesses/pixel-05-66.webp) | 20×12×3f `2239df38` | 20×12×3f `2239df38` | n/a | 20×12×3f `2239df38`
accept-different-pixels | glycin~libwebp-anim: different-pixels; image-webp~libwebp-anim: different-pixels | 25 | 255 | field:vp8x_canvas_h_minus1 7; field:vp8x_canvas_w_minus1 7; field:ANMF_fourcc 7 | [pixel-06-143.webp](witnesses/pixel-06-143.webp) | 8×15×2f `96102f5f` | 8×15×2f `96102f5f` | n/a | 8×15×2f `60be3f37`
accept-different-pixels | glycin~libwebp-anim: different-pixels; glycin~libwebp-simple: different-pixels; image-webp~libwebp-anim: different-pixels; image-webp~libwebp-simple: different-pixels | 24 | 255 | wrap-vp8x 8; field:vp8l_dims_flags 8; vp8l-header-bits 4 | [pixel-07-3333.webp](witnesses/pixel-07-3333.webp) | 16×16 `2be86413` | 16×16 `2be86413` | 16×16 `8fc3e9d2` | 16×16 `8fc3e9d2`
accept-different-pixels | glycin~libwebp-simple: different-pixels; image-webp~libwebp-simple: different-pixels; libwebp-simple~libwebp-anim: different-pixels | 7 | 255 | field:vp8x_flags 5; vp8x-flag-bit 2 | [pixel-08-1709.webp](witnesses/pixel-08-1709.webp) | 16×16 `24137068` | 16×16 `24137068` | 16×16 `9b1fa175` | 16×16 `24137068`
accept-different-pixels | glycin~image-webp: different-pixels; glycin~libwebp-anim: different-pixels; image-webp~libwebp-anim: different-pixels | 6 | 255 | field:anmf_flags 4; field:vp8x_flags 2 | #709 | 20×12×3f `57f7914a` | 20×12×3f `57f7914a` | n/a | 20×12×3f `2239df38`

## Per-group lone votes and same-group divergence

Lone acceptor (only this implementation group accepts): none

Lone rejecter (only this group rejects): none

Same-group divergence (columns that share an implementation but disagree; indicates glue or API-path differences): {'glycin/image-webp:different-pixels': 592, 'libwebp-simple/libwebp-anim:different-pixels': 11, 'libwebp-simple/libwebp-anim:status': 194}


## Acceptance disagreement counts (inputs where the two columns differ in accept/reject)

| | glycin | image-webp | libwebp-simple | libwebp-anim
--- | --- | --- | --- | ---
glycin | 0 | 0 | 771 | 1760
image-webp | 0 | 0 | 771 | 1760
libwebp-simple | 771 | 771 | 0 | 194
libwebp-anim | 1760 | 1760 | 194 | 0

## Significant observations (seed 731, 7,000 inputs; witnesses linked above)

1. Animated frames: on the unmutated anim-ll-3f seed, image-webp and glycin agree with libwebp-anim on frame count and on the first frame, but frame 2 differs by one level (0x1d vs 0x1e on the opaque background); the pair is `within-tol`, not equal.
2. glycin and a direct image-webp call (the same code) disagree on later-frame pixels for 592 mutated animation inputs (mainly ANMF flags, duration, offsets, fourcc, size). In witness 66 glycin returns a transparent pixel at (0,0) of frames 2 and 3 where the image-crate path returns an opaque pixel. Seeds are identical in both, so the divergence is in the glycin animation loop on perturbed ANMF headers.
3. glycin/image-webp accept and libwebp-anim rejects on 723 inputs, led by ANMF x/y offset mutations, VP8L signature and size fields: libwebp's demuxer validates frame geometry and chunk consistency that image-webp does not.
4. A wrong RIFF size (delta -9..+1000, or boundary values) is accepted by image-webp/glycin and libwebp-anim rejects it; libwebp-simple also rejects (606 inputs where both libwebp APIs reject and both Rust paths accept).
5. libwebp-anim alone accepts 200 inputs (ANMF width/height and VP8L fourcc mutations) that both Rust paths reject.
6. libwebp-simple alone accepts 64 inputs (VP8L/VP8/EXIF chunk size mutations) that every other column rejects; libwebp-simple and libwebp-anim, one library, disagree on acceptance for 194 inputs (the demux validation of AnimDecoder versus the incremental parser of WebPDecode).
7. Inserted, duplicated or padded extra chunks: the Rust paths and libwebp-simple accept (130 inputs) while libwebp-anim rejects (demux strictness).
8. VP8X flag bits and VP8X wrapping of simple files change what libwebp-simple/libwebp-anim accept against the Rust paths (101 inputs where only libwebp accepts, e.g. ALPH/EXIF fourcc mutations); a falsely set animation or alpha flag yields different pixels in 31 inputs.
9. Lossless and lossy still images (30 seeds, including lossy+ALPH, near-lossless, exact) decode bit-identically in all four columns; lossy needs no tolerance on unmutated seeds.
10. All-reject is 2,875 of 7,000 inputs and 1,249 are identical-pixel accepts; truncation, bit flips and magic changes are rejected by every column.


## Notes

- Seeds (`seeds/`, 30 files, 18 KB) come from `runners/mkseeds.c` (libwebp 1.5.0 encoder, mux, anim encoder) with closed-form gradient pixels: lossless (RGB/RGBA, 1x1 to 32x32, near-lossless, exact), lossy (q10 to q75, sharp YUV, filter/segment/partition variants, lossy+ALPH with raw/filtered/lossy alpha), VP8X metadata (ICCP/EXIF/XMP), animations (lossless, lossy, mixed, with alpha, sub-rectangle frames).
- Mutation families: RIFF size and magic, per-chunk FourCC/size boundary values, VP8X flags/canvas, ANIM, ANMF geometry/duration/flags, ALPH header byte (48 sampled of 256 values), VP8 frame tag/scale/dimensions, VP8L header bit fields, chunk insert/delete/duplicate/swap, padding variants, simple<->extended wrapping, truncation at chunk boundaries, trailing bytes, random bit/byte flips.
- Lossy pixel tolerance: maximum absolute channel difference (RGBA8, all frames) of at most 2 is `within-tol`. All lossy seeds decode bit-identically between image-webp and libwebp in this run, so the threshold only matters for mutated streams.
- `libwebp-simple` is `skip` (no vote) when `WebPGetFeatures` reports animation: the still-image API is not applicable. It is still a vote for everything else, including files whose VP8X flags claim animation falsely.
- Animation: `glycin` and `image-webp` return frames via the image crate's composited canvas; `libwebp-anim` returns canvases from `WebPAnimDecoder`. Hash lines compare every frame.
- glycin column: frames pulled with `specific_frame(loop_animation=false)` until `NoMoreFrames` (max 32).
- Crashes/timeouts: input saved under `/tmp/claude-0/leniency-webp/anomalies/`, counted in the matrix only; not investigated.

