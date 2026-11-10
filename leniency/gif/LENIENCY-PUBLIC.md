# GIF leniency matrix

Seed 731; 9938 synthetic inputs (crash/timeout inputs excluded); 51 unmutated generator seeds; 51 seeds received mutations; 5385 mutations changed the acceptance/pixel relation relative to their seed; decoder pass 423.4 s (one CPU), witness minimisation afterwards.

Classification (first displayed frame, top-first straight RGBA8) | Count
--- | ---:
all-accept-identical-pixels | 3862
accept-different-pixels | 611
accept-vs-reject | 4568
all-reject | 897

Animation comparison among accepting animation-capable runners (glycin, image, gifcrate, gdk-anim, giflib, libnsgif, stb, gift, tinygif) | Count
--- | ---:
canvas-differs | 1545
frame-count-differs | 158
identical | 5094
n/a | 1733
sequence-pixels-differ | 1470

## Decoders

`glycin` calls the in-tree `glycin_image_rs::ImgLoader` (mime `image/gif`) and requests frames with `loop_animation=false` until `NoMoreFrames`; it wraps `image::GifDecoder` =0.25.10, which wraps the `gif` crate 0.14.2, so `glycin`, `image` and `gifcrate` are one vote. `gifcrate` uses the gif crate's indexed frames with the harness reference compositor (transparent-black canvas; disposal 2 clears the rectangle to transparent; 3 restores; 0, 1 and undefined 4–7 keep; out-of-palette indexes paint opaque black); `giflib` (5.1.4 via the github.com/mirrorer/giflib mirror, the only allowed source) uses `DGifSlurp` with the same compositor. `libnsgif` (NetSurf, c26c7aa), `stb` (stb_image 2.30, 4e9b368), `gift` 0.13 and the GDK animation iterator composite with their own policies. `tinygif` 0.0.4 is an embedded decoder: it only yields frames introduced by a Graphic Control Extension, draws without disposal and without alpha. `gdk-anim` = `gdk_pixbuf_animation_new_from_file` with synthetic time advanced by each frame delay (GDK clamps delays: 0 → 100 ms, < 20 ms → 20 ms); `gdk-forced-*` = `GdkPixbufLoader` forced to `gif`, whole buffer or one byte per write, static pixbuf. Every hash is over straight RGBA8; GIF has no gamma/ICC handling in any runner. The family vote of a family is its first listed runner.

| Runner | Family (one vote) | ok | error | lenient (accepts, independent majority rejects) | strict (rejects, majority accepts) | first-frame pixel outlier | tail error after pixels |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| glycin | image-rs/gif-crate | 6768 | 3170 | 24 | 230 | 54 | 553 |
| image | image-rs/gif-crate | 6768 | 3170 | 24 | 230 | 54 | 553 |
| gifcrate | image-rs/gif-crate | 6768 | 3170 | 24 | 230 | 82 | 553 |
| gdk-anim | gdk-pixbuf | 7747 | 2191 | 973 | 415 | 29 | 0 |
| gdk-forced-whole | gdk-pixbuf | 7730 | 2208 | 972 | 423 | 29 | 0 |
| gdk-forced-byte | gdk-pixbuf | 7523 | 2415 | 959 | 610 | 29 | 0 |
| giflib | giflib | 6508 | 3430 | 232 | 650 | 39 | 0 |
| libnsgif | libnsgif | 8371 | 1567 | 1043 | 0 | 1209 | 392 |
| stb | stb_image | 6249 | 3689 | 301 | 1320 | 147 | 0 |
| gift | gift | 5475 | 4463 | 18 | 1449 | 610 | 439 |
| tinygif | tinygif | 2619 | 7319 | 245 | 4603 | 325 | 0 |

Frame-delay lists differing from `gifcrate` (raw file delays) when frame counts agree: glycin 803, gdk-anim 747, giflib 6, libnsgif 157, stb 1, gift 757.

Unavailable or not integrated candidates:

- ImageMagick, ffmpeg, gifsicle: not installed; apt is not usable at runtime and they were not built from source.
- giflib 5.2.x: upstream lives on SourceForge (not an allowed host); the GitHub mirror provides 5.1.4.
- wuffs (github.com/google/wuffs): buildable, not integrated in this pass.
- image-rs `gif` crate as an independent vote: not independent (image-rs uses it); shown as `gifcrate` with the reference compositor.

## Ten significant observations

- Unknown extension labels (0x21 followed by an unregistered label): the image-rs family (glycin, image, gif crate) rejects them because the gif crate defaults to `allow_unknown_blocks=false`; giflib, libnsgif, GDK, stb and gift skip them (struct:insert-unknown-ext 0 of 51 accepted by glycin, 51 of 51 by the others).
- Missing trailer (no 0x3B): glycin, libnsgif, stb and gift accept; giflib and every GDK path reject (struct:no-trailer, 51 inputs).
- GDK is the most lenient on LZW streams: it accepts every invalid code, TIFF-style early code-width change and mismatched LZW minimum-code-size byte that the others mostly reject (lzw:invalid-code 51/51 vs giflib 11, glycin 3; lzw:early-change 51/51 vs glycin 10; lzw:min-code-byte 428/428 vs glycin 250).
- Short pixel data (the LZW stream ends before width×height pixels): glycin and giflib reject; GDK, stb and libnsgif decode a partial frame (lzw:pixels-short, -one-short, -none).
- Frames that extend past the logical screen: glycin, giflib, libnsgif and GDK clip (or grow the canvas, libnsgif) and accept; stb and gift reject (field:left 5 of 384 accepted by stb and gift, 378–384 by the rest).
- Header version: giflib and libnsgif accept any three bytes after `GIF` (88a, 89b, spaces); glycin, GDK, stb and gift accept only 87a/89a (header:version).
- No colour table at all: giflib and libnsgif accept, glycin, GDK, stb and gift reject (struct:no-palette 37 inputs). A zero-sized logical screen is accepted only by libnsgif, which grows the canvas to the frame (struct:canvas-zero).
- Interlace: gift ignores the interlace flag and renders rows in stream order (every interlaced seed: rgb-only pixel difference against all other decoders); stb fills uncovered canvas with the background colour instead of transparency (frame-inset seed, field:frame_width pixel row).
- Incremental vs whole-buffer GDK: with one-byte writes the GDK GIF loader rejects an XMP application extension that whole-buffer loading accepts (struct:insert-xmp 0 of 51 vs 51 of 51), and is stricter on other fields (gdk-forced-byte 611 strict vs 423 for whole-buffer).
- Frame timing policy (not pixels): GDK maps delay 0 to 100 ms and anything below 20 ms to 20 ms; glycin maps delay 0 to 100 ms but keeps 10 ms; libnsgif reports 100 ms for frames without a GCE; image/gif crate/giflib/stb report raw centiseconds×10; gift exposes no delay. GDK frame counts come from its iterator, so identical consecutive frames can collapse.

## Representative leniency rows

A row is the most frequent full outcome signature among *changed* mutations of one rule (a mutation whose acceptance/pixel-equality pattern equals its seed's is not counted). `n; p` = changed inputs; distinct patterns. Cells: `W×H hash8` or `reject class`. Witnesses are delta-debugged under the same signature (bounded, not globally minimal).

Runners outside the classification and majority votes (observed only): tinygif.

| Rule (n; p) | Witness | glycin | image | gifcrate | gdk-anim | gdk-forced-whole | gdk-forced-byte | giflib | libnsgif | stb | gift | tinygif |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| field:left (379; 379) | [#2144](witnesses/01-field-left.gif) 41B | 9×6 `d5638b` | 9×6 `d5638b` | 9×6 `d5638b` | 9×6 `d5638b` | 9×6 `d5638b` | 9×6 `d5638b` | 9×6 `d5638b` | 10×6 `ca382f` | ✗ `stbi` | ✗ `InvalidFrameDimens` | ✗ `NoFrames` |
| field:trailer (378; 51) | [#2501](witnesses/02-field-trailer.gif) 41B | 9×6 `1af928` | 9×6 `1af928` | 9×6 `1af928` | ✗ `gdk` | ✗ `gdk` | ✗ `gdk` | ✗ `DGifSlurp` | 9×6 `1af928` | 9×6 `1af928` | 9×6 `1af928` | ✗ `NoFrames` |
| field:top (377; 377) | [#2195](witnesses/03-field-top.gif) 41B | 9×6 `25ebfc` | 9×6 `25ebfc` | 9×6 `25ebfc` | 9×6 `25ebfc` | 9×6 `25ebfc` | 9×6 `25ebfc` | 9×6 `25ebfc` | 9×7 `b2b831` | ✗ `stbi` | ✗ `InvalidFrameDimens` | ✗ `NoFrames` |
| field:first_sub_block_len (324; 262) | [#4826](witnesses/04-field-first_sub_block_len.gif) 130B | ✗ `ExpectedError` | ✗ `Format error decod` | ✗ `invalid code in LZ` | 12×12 `03b57e` | 12×12 `03b57e` | 12×12 `03b57e` | ✗ `DGifSlurp` | ✗ `nsgif_frame_decode` | ✗ `stbi` | ✗ `InvalidLzwData` | ✗ `NoFrames` |
| field:lzw_min (304; 251) | [#6825](witnesses/05-field-lzw_min.gif) 77B | 1×1 `df3f61` | 1×1 `df3f61` | 1×1 `e38200` | 1×1 `e38200` | 1×1 `e38200` | 1×1 `e38200` | 1×1 `e38200` | 1×1 `df3f61` | ✗ `stbi` | ✗ `InvalidColorIndex` | ✗ `NoFrames` |
| field:frame_height (222; 207) | [#2349](witnesses/06-field-frame_height.gif) 80B | ✗ `ExpectedError` | ✗ `Format error decod` | ✗ `image truncated` | ✗ `gdk` | ✗ `gdk` | ✗ `gdk` | ✗ `DGifSlurp` | 2×3 `9d908e` | 2×3 `9d908e` | ✗ `InvalidFrameDimens` | ✗ `NoFrames` |
| field:screen_width (217; 217) | [#1889](witnesses/07-field-screen_width.gif) 41B | ✗ `Dimensions` | ✗ `Dimensions` | ✗ `Dimensions` | ✗ `gdk` | ✗ `gdk` | ✗ `gdk` | ✗ `Dimensions` | 9×6 `1af928` | ✗ `stbi` | ✗ `InvalidFrameDimens` | ✗ `Dimensions` |
| field:screen_height (213; 213) | [#1940](witnesses/08-field-screen_height.gif) 41B | ✗ `Dimensions` | ✗ `Dimensions` | ✗ `Dimensions` | ✗ `gdk` | ✗ `gdk` | ✗ `gdk` | ✗ `Dimensions` | 9×6 `1af928` | ✗ `stbi` | ✗ `InvalidFrameDimens` | ✗ `Dimensions` |
| lzw:min-code-byte (212; 196) | [#3891](witnesses/09-lzw-min-code-byte.gif) 223B | ✗ `ExpectedError` | ✗ `Format error decod` | ✗ `invalid code in LZ` | 2×3 `9d908e` | 2×3 `9d908e` | 2×3 `9d908e` | ✗ `DGifSlurp` | ✗ `nsgif_frame_decode` | ✗ `stbi` | ✗ `InvalidLzwData` | ✗ `NoFrames` |
| field:frame_width (203; 188) | [#2298](witnesses/10-field-frame_width.gif) 80B | ✗ `ExpectedError` | ✗ `Format error decod` | ✗ `image truncated` | ✗ `gdk` | ✗ `gdk` | ✗ `gdk` | 2×3 `9d908e` | 2×3 `9d908e` | 2×3 `9d908e` | ✗ `InvalidFrameDimens` | ✗ `NoFrames` |
| header:version (153; 51) | [#3062](witnesses/11-header-version.gif) 41B | ✗ `ExpectedError` | ✗ `Format error decod` | ✗ `malformed GIF head` | ✗ `gdk` | ✗ `gdk` | ✗ `gdk` | 9×6 `1af928` | 9×6 `1af928` | ✗ `stbi` | ✗ `UnsupportedVersion` | ✗ `InvalidFileSignatu` |
| lzw:encoded-min-code (133; 51) | [#845](witnesses/12-lzw-encoded-min-code.gif) 1119B | 16×16 `86170d` | 16×16 `86170d` | 16×16 `86170d` | 16×16 `86170d` | 16×16 `86170d` | 16×16 `86170d` | ✗ `DGifSlurp` | 16×16 `86170d` | 16×16 `86170d` | ✗ `InvalidLzwData` | ✗ `NoFrames` |
| byte-flip (130; 122) | [#2625](witnesses/13-byte-flip.gif) 74B | 8×8 `810bd3` | 8×8 `810bd3` | 8×8 `810bd3` | 8×8 `810bd3` | 8×8 `810bd3` | 8×8 `810bd3` | ✗ `DGifSlurp` | 8×8 `810bd3` | 8×8 `810bd3` | 8×8 `810bd3` | ✗ `NoFrames` |
| truncation (91; 19) | [#8113](witnesses/14-truncation.gif) 38B | ✗ `ExpectedError` | ✗ `Format error decod` | ✗ `Unexpected End of ` | ✗ `gdk` | ✗ `gdk` | ✗ `gdk` | ✗ `DGifSlurp` | ✗ `nsgif_data_scan` | 64×48 `f3cc10` | ✗ `UnexpectedEndOfFil` | ✗ `NoFrames` |
| field:gce_size (63; 39) | [#2292](witnesses/15-field-gce_size.gif) 50B | ✗ `ExpectedError` | ✗ `Format error decod` | ✗ `control extension ` | 6×6 `518318` | 6×6 `518318` | 6×6 `518318` | ✗ `DGifSlurp` | ✗ `nsgif_data_scan` | ✗ `stbi` | ✗ `InvalidBlockCode` | ✗ `NoFrames` |
| field:ext_label (52; 20) | [#2210](witnesses/16-field-ext_label.gif) 75B | ✗ `ExpectedError` | ✗ `Format error decod` | ✗ `unknown extension ` | 7×5 `e98799` | 7×5 `e98799` | 7×5 `e98799` | 7×5 `e98799` | 7×5 `e98799` | 7×5 `e98799` | 7×5 `e98799` | ✗ `NoFrames` |
| field:image_flags (52; 26) | [#7652](witnesses/17-field-image_flags.gif) 30B | ✗ `ExpectedError` | ✗ `Format error decod` | ✗ `Unexpected End of ` | ✗ `gdk` | ✗ `gdk` | ✗ `gdk` | ✗ `DGifSlurp` | ✗ `nsgif_data_scan` | 9×6 `a5645e` | ✗ `UnexpectedEndOfFil` | ✗ `NoFrames` |
| lzw:first-subblock-zero (51; 51) | [#459](witnesses/18-lzw-first-subblock-zero.gif) 90B | 12×10 `cc8f1f` | 12×10 `cc8f1f` | 12×10 `cc8f1f` | 12×10 `4b48f2` | 12×10 `4b48f2` | 12×10 `4b48f2` | ✗ `DGifSlurp` | 12×10 `4b48f2` | 12×10 `4b48f2` | ✗ `IncompleteImageDat` | 12×10 `4b48f2` |
| lzw:no-block-terminator (51; 51) | [#510](witnesses/19-lzw-no-block-terminator.gif) 88B | 12×10 `cc8f1f` | 12×10 `cc8f1f` | 12×10 `cc8f1f` | ✗ `gdk` | ✗ `gdk` | ✗ `gdk` | ✗ `DGifSlurp` | 12×10 `cc8f1f` | 12×10 `cc8f1f` | ✗ `UnexpectedEndOfFil` | 12×10 `4b48f2` |
| lzw:pixels-short (51; 50) | [#580](witnesses/20-lzw-pixels-short.gif) 125B | ✗ `ExpectedError` | ✗ `Format error decod` | ✗ `image truncated` | 1×1 `df3f61` | 1×1 `df3f61` | 1×1 `df3f61` | ✗ `DGifSlurp` | ✗ `nsgif_frame_decode` | 1×1 `df3f61` | ✗ `IncompleteImageDat` | ✗ `NoFrames` |
| lzw:pixels-one-short (51; 50) | [#631](witnesses/21-lzw-pixels-one-short.gif) 125B | ✗ `ExpectedError` | ✗ `Format error decod` | ✗ `image truncated` | 1×1 `df3f61` | 1×1 `df3f61` | 1×1 `df3f61` | ✗ `DGifSlurp` | ✗ `nsgif_frame_decode` | 1×1 `df3f61` | ✗ `IncompleteImageDat` | ✗ `NoFrames` |
| lzw:pixels-none (51; 27) | [#714](witnesses/22-lzw-pixels-none.gif) 60B | ✗ `ExpectedError` | ✗ `Format error decod` | ✗ `image truncated` | 12×10 `4b48f2` | 12×10 `4b48f2` | 12×10 `4b48f2` | ✗ `DGifSlurp` | ✗ `nsgif_frame_decode` | 12×10 `4b48f2` | ✗ `IncompleteImageDat` | 12×10 `4b48f2` |
| struct:no-trailer (51; 51) | [#867](witnesses/23-struct-no-trailer.gif) 88B | 12×10 `cc8f1f` | 12×10 `cc8f1f` | 12×10 `cc8f1f` | ✗ `gdk` | ✗ `gdk` | ✗ `gdk` | ✗ `DGifSlurp` | 12×10 `cc8f1f` | 12×10 `cc8f1f` | 12×10 `cc8f1f` | 12×10 `cc8f1f` |
| lzw:no-leading-clear (50; 50) | [#51](witnesses/24-lzw-no-leading-clear.gif) 89B | 12×10 `cc8f1f` | 12×10 `cc8f1f` | 12×10 `cc8f1f` | 12×10 `cc8f1f` | 12×10 `cc8f1f` | 12×10 `cc8f1f` | 12×10 `cc8f1f` | 12×10 `cc8f1f` | ✗ `stbi` | 12×10 `cc8f1f` | 12×10 `62186a` |
| struct:canvas-zero (50; 50) | [#1073](witnesses/25-struct-canvas-zero.gif) 41B | ✗ `Dimensions` | ✗ `Dimensions` | ✗ `Dimensions` | ✗ `gdk` | ✗ `gdk` | ✗ `gdk` | ✗ `Dimensions` | 9×6 `1af928` | ✗ `stbi` | ✗ `InvalidFrameDimens` | ✗ `Dimensions` |
| struct:insert-unknown-ext (50; 50) | [#1583](witnesses/26-struct-insert-unknown-ext.gif) 58B | ✗ `ExpectedError` | ✗ `Format error decod` | ✗ `unknown extension ` | 9×6 `1af928` | 9×6 `1af928` | 9×6 `1af928` | 9×6 `1af928` | 9×6 `1af928` | 9×6 `1af928` | 9×6 `1af928` | ✗ `NoFrames` |
| struct:insert-xmp (50; 50) | [#1634](witnesses/27-struct-insert-xmp.gif) 94B | 9×6 `1af928` | 9×6 `1af928` | 9×6 `1af928` | 9×6 `1af928` | 9×6 `1af928` | ✗ `gdk` | 9×6 `1af928` | 9×6 `1af928` | 9×6 `1af928` | 9×6 `1af928` | ✗ `NoFrames` |
| lzw:invalid-code (48; 48) | [#306](witnesses/28-lzw-invalid-code.gif) 90B | ✗ `ExpectedError` | ✗ `Format error decod` | ✗ `invalid code in LZ` | 12×10 `3f4149` | 12×10 `3f4149` | 12×10 `3f4149` | ✗ `DGifSlurp` | ✗ `nsgif_frame_decode` | ✗ `stbi` | ✗ `InvalidLzwData` | 12×10 `3f4149` |
| struct:frame-beyond-canvas (48; 48) | [#971](witnesses/29-struct-frame-beyond-canvas.gif) 41B | 9×6 `5e158b` | 9×6 `5e158b` | 9×6 `5e158b` | 9×6 `5e158b` | 9×6 `5e158b` | 9×6 `5e158b` | 9×6 `5e158b` | 17×11 `645a90` | ✗ `stbi` | ✗ `InvalidFrameDimens` | ✗ `NoFrames` |
| struct:frame-larger-than-canvas (48; 48) | [#1022](witnesses/30-struct-frame-larger-than-canvas.gif) 41B | 4×3 `76bdbd` | 4×3 `76bdbd` | 4×3 `76bdbd` | 4×3 `76bdbd` | 4×3 `76bdbd` | 4×3 `76bdbd` | 4×3 `76bdbd` | 9×6 `1af928` | ✗ `stbi` | ✗ `InvalidFrameDimens` | ✗ `NoFrames` |
| lzw:early-change (42; 42) | [#357](witnesses/31-lzw-early-change.gif) 89B | ✗ `ExpectedError` | ✗ `Format error decod` | ✗ `invalid code in LZ` | 12×10 `99f05d` | 12×10 `99f05d` | 12×10 `99f05d` | ✗ `DGifSlurp` | ✗ `nsgif_frame_decode` | ✗ `stbi` | ✗ `InvalidLzwData` | 12×10 `99f05d` |
| struct:interlace-toggle (42; 42; rgb-only: 18 of 54 px (glycin+image+gifcrate vs gift)) | [#1328](witnesses/32-struct-interlace-toggle.gif) 43B | 9×6 `1af928` | 9×6 `1af928` | 9×6 `1af928` | 9×6 `1af928` | 9×6 `1af928` | 9×6 `1af928` | 9×6 `1af928` | 9×6 `1af928` | 9×6 `1af928` | 9×6 `08c2d8` | ✗ `NoFrames` |
| struct:no-palette (37; 33) | [#960](witnesses/33-struct-no-palette.gif) 29B | ✗ `ExpectedError` | ✗ `Format error decod` | ✗ `no color table ava` | ✗ `gdk` | ✗ `gdk` | ✗ `gdk` | 1×1 `e38200` | 1×1 `e38200` | ✗ `stbi` | ✗ `MissingColorTable` | ✗ `NoFrames` |
| field:lsd_flags (36; 31) | [#2086](witnesses/34-field-lsd_flags.gif) 127B | ✗ `ExpectedError` | ✗ `Format error decod` | ✗ `unknown block type` | 10×10 `6a9df3` | 10×10 `6a9df3` | 10×10 `6a9df3` | ✗ `DGifSlurp` | ✗ `nsgif_data_scan` | ✗ `stbi` | ✗ `InvalidBlockCode` | ✗ `NoFrames` |
| struct:short-global-palette (30; 29) | [#984](witnesses/35-struct-short-global-palette.gif) 40B | 2×3 `9d908e` | 2×3 `9d908e` | 2×3 `c77339` | 2×3 `c77339` | 2×3 `c77339` | 2×3 `c77339` | 2×3 `c77339` | 2×3 `9d908e` | 2×3 `9d908e` | ✗ `InvalidColorIndex` | ✗ `NoFrames` |
| field:gce_terminator (25; 6) | [#2496](witnesses/36-field-gce_terminator.gif) 150B | ✗ `ExpectedError` | ✗ `Format error decod` | ✗ `control extension ` | 6×6 `518318` | 6×6 `518318` | 6×6 `518318` | ✗ `DGifSlurp` | ✗ `nsgif_data_scan` | ✗ `stbi` | ✗ `MalformedGraphicCo` | ✗ `NoFrames` |
| struct:gce-without-image (15; 15) | [#1428](witnesses/37-struct-gce-without-image.gif) 97B | 12×10 `cc8f1f` | 12×10 `cc8f1f` | 12×10 `cc8f1f` | 12×10 `cc8f1f` | 12×10 `cc8f1f` | 12×10 `cc8f1f` | 12×10 `31af79` | 12×10 `cc8f1f` | 12×10 `cc8f1f` | ✗ `InvalidBlockSequen` | 12×10 `4b48f2` |
| lzw:garbage-after-eoi (14; 14) | [#155](witnesses/38-lzw-garbage-after-eoi.gif) 45B | 9×6 `1af928` | 9×6 `1af928` | 9×6 `1af928` | 9×6 `1af928` | 9×6 `1af928` | 9×6 `1af928` | 9×6 `1af928` | 9×6 `1af928` | 9×6 `1af928` | ✗ `InvalidLzwData` | ✗ `NoFrames` |
| seed:interlace (5; 5; rgb-only: 40 of 64 px (glycin+image+gifcrate vs gift)) | [#22](witnesses/39-seed-interlace.gif) 73B | 8×8 `954b63` | 8×8 `954b63` | 8×8 `954b63` | 8×8 `954b63` | 8×8 `954b63` | 8×8 `954b63` | 8×8 `954b63` | 8×8 `954b63` | 8×8 `954b63` | 8×8 `24a751` | ✗ `NoFrames` |
| seed:ext (2; 2) | [#42](witnesses/40-seed-ext.gif) 109B | 7×5 `7cc624` | 7×5 `7cc624` | 7×5 `7cc624` | 7×5 `7cc624` | 7×5 `7cc624` | ✗ `gdk` | 7×5 `7cc624` | 7×5 `7cc624` | 7×5 `7cc624` | 7×5 `7cc624` | 7×5 `7cc624` |

## Same acceptance, different first-frame pixels

Rules with the most `accept-different-pixels` inputs (smallest input per rule, not minimised; difference from raw RGBA dumps of the two largest hash groups).

| Rule (n) | Witness | Difference | Hash groups |
| --- | --- | --- | --- |
| field:image_flags (68) | [#6777](witnesses/pixel-01-field-image_flags.gif) | rgb-only: 8 of 9 px (glycin+image+gifcrate vs gift) | 1×9 12763255: glycin+image+gifcrate+gdk-anim+gdk-forced-whole+gdk-forced-byte+giflib+libnsgif+stb; 1×9 964ce50c: gift |
| struct:disposal (48) | [#1320](witnesses/pixel-02-struct-disposal.gif) | rgb-only: 8 of 9 px (glycin+image+gifcrate vs gift+tinygif) | 1×9 12763255: glycin+image+gifcrate+gdk-anim+gdk-forced-whole+gdk-forced-byte+giflib+libnsgif+stb; 1×9 964ce50c: gift+tinygif |
| field:aspect (48) | [#2187](witnesses/pixel-03-field-aspect.gif) | rgb-only: 8 of 9 px (glycin+image+gifcrate vs gift) | 1×9 12763255: glycin+image+gifcrate+gdk-anim+gdk-forced-whole+gdk-forced-byte+giflib+libnsgif+stb; 1×9 964ce50c: gift |
| field:background (47) | [#2136](witnesses/pixel-04-field-background.gif) | rgb-only: 8 of 9 px (glycin+image+gifcrate vs gift) | 1×9 12763255: glycin+image+gifcrate+gdk-anim+gdk-forced-whole+gdk-forced-byte+giflib+libnsgif+stb; 1×9 964ce50c: gift |
| struct:interlace-toggle (37) | [#1328](witnesses/pixel-05-struct-interlace-toggle.gif) | rgb-only: 18 of 54 px (glycin+image+gifcrate vs gift) | 9×6 1af928fe: glycin+image+gifcrate+gdk-anim+gdk-forced-whole+gdk-forced-byte+giflib+libnsgif+stb; 9×6 08c2d87d: gift |
| trailing-data (36) | [#1830](witnesses/pixel-06-trailing-data.gif) | rgb-only: 8 of 9 px (glycin+image+gifcrate vs gift) | 1×9 12763255: glycin+image+gifcrate+gdk-anim+gdk-forced-whole+gdk-forced-byte+giflib+libnsgif+stb; 1×9 964ce50c: gift |
| lzw:min-code-byte (33) | [#4992](witnesses/pixel-07-lzw-min-code-byte.gif) | rgb-only: 8 of 9 px (glycin+image+gifcrate vs gift) | 1×9 12763255: glycin+image+gifcrate+gdk-anim+gdk-forced-whole+gdk-forced-byte+giflib+libnsgif+stb; 1×9 964ce50c: gift |
| field:screen_width (33) | [#3207](witnesses/pixel-08-field-screen_width.gif) | rgb-only: 8 of 18 px (glycin+image+gifcrate vs gift) | 2×9 8ae67f52: glycin+image+gifcrate+gdk-anim+gdk-forced-whole+gdk-forced-byte+giflib+libnsgif+stb; 2×9 8738fd2d: gift |
| field:screen_height (33) | [#6369](witnesses/pixel-09-field-screen_height.gif) | rgb-only: 8 of 10 px (glycin+image+gifcrate vs gift) | 1×10 7a5bdec4: glycin+image+gifcrate+gdk-anim+gdk-forced-whole+gdk-forced-byte+giflib+libnsgif+stb; 1×10 67d68bf1: gift |
| lzw:encoded-min-code (30) | [#861](witnesses/pixel-10-lzw-encoded-min-code.gif) | rgb-only: 8 of 9 px (glycin+image+gifcrate vs gift) | 1×9 12763255: glycin+image+gifcrate+gdk-anim+gdk-forced-whole+gdk-forced-byte+giflib+libnsgif+stb; 1×9 964ce50c: gift |
| struct:transparent-index (24) | [#1371](witnesses/pixel-11-struct-transparent-index.gif) | rgb-and-alpha: 8 of 9 px (glycin+image+gifcrate vs gift+tinygif) | 1×9 827b5ea3: glycin+image+gifcrate+gdk-anim+gdk-forced-whole+gdk-forced-byte+giflib+libnsgif+stb; 1×9 5ad9362b: gift+tinygif |
| byte-flip (24) | [#6012](witnesses/pixel-12-byte-flip.gif) | rgb-only: 8 of 9 px (glycin+image+gifcrate vs gift) | 1×9 f78ea129: glycin+image+gifcrate+gdk-anim+gdk-forced-whole+gdk-forced-byte+giflib+libnsgif+stb; 1×9 82f35991: gift |
| field:frame_width (19) | [#3546](witnesses/pixel-13-field-frame_width.gif) | rgb-and-alpha: 295 of 300 px (glycin+image+gifcrate vs stb) | 20×15 ecf7f3b3: glycin+image+gifcrate+gdk-anim+gdk-forced-whole+gdk-forced-byte+giflib+libnsgif+gift; 20×15 beaa15bd: stb |
| lzw:clear-every (12) | [#249](witnesses/pixel-14-lzw-clear-every.gif) | rgb-only: 8 of 9 px (glycin+image+gifcrate vs gift) | 1×9 12763255: glycin+image+gifcrate+gdk-anim+gdk-forced-whole+gdk-forced-byte+giflib+libnsgif+stb; 1×9 964ce50c: gift |
| lzw:subblock (12) | [#2748](witnesses/pixel-15-lzw-subblock.gif) | rgb-only: 8 of 9 px (glycin+image+gifcrate vs gift) | 1×9 12763255: glycin+image+gifcrate+gdk-anim+gdk-forced-whole+gdk-forced-byte+giflib+libnsgif+stb; 1×9 964ce50c: gift |

## Animation disagreements

Inputs where accepting animation-capable runners agree on the first frame or not, but disagree on canvas, frame count or composited sequence, clustered by rule (changed relative to the seed). Witnesses are not minimised.

| Rule (n) | Witness | Kinds | Frames per runner |
| --- | --- | --- | --- |
| field:left (379) | [#2144](witnesses/anim-01-field-left.gif) | canvas-differs 379 | glycin 1; image 1; gifcrate 1; gdk-anim 1; giflib 1; libnsgif 1; stb ✗; gift ✗; tinygif ✗ |
| field:top (377) | [#2195](witnesses/anim-02-field-top.gif) | canvas-differs 377 | glycin 1; image 1; gifcrate 1; gdk-anim 1; giflib 1; libnsgif 1; stb ✗; gift ✗; tinygif ✗ |
| field:frame_height (170) | [#6632](witnesses/anim-03-field-frame_height.gif) | canvas-differs 170 | glycin ✗; image ✗; gifcrate ✗; gdk-anim 1; giflib ✗; libnsgif 1; stb ✗; gift ✗; tinygif ✗ |
| field:screen_width (167) | [#3113](witnesses/anim-04-field-screen_width.gif) | canvas-differs 167 | glycin 1; image 1; gifcrate 1; gdk-anim 1; giflib 1; libnsgif 1; stb ✗; gift ✗; tinygif ✗ |
| field:frame_width (165) | [#8468](witnesses/anim-05-field-frame_width.gif) | canvas-differs 165 | glycin ✗; image ✗; gifcrate ✗; gdk-anim 1; giflib ✗; libnsgif 1; stb ✗; gift ✗; tinygif ✗ |
| field:screen_height (163) | [#3164](witnesses/anim-06-field-screen_height.gif) | canvas-differs 162, sequence-pixels-differ 1 | glycin 1; image 1; gifcrate 1; gdk-anim 1; giflib 1; libnsgif 1; stb ✗; gift ✗; tinygif ✗ |
| field:first_sub_block_len (64) | [#8672](witnesses/anim-07-field-first_sub_block_len.gif) | sequence-pixels-differ 57, frame-count-differs 7 | glycin 1; image 1; gifcrate 1; gdk-anim ✗; giflib ✗; libnsgif 1; stb 1; gift ✗; tinygif ✗ |
| field:lzw_min (52) | [#2399](witnesses/anim-08-field-lzw_min.gif) | sequence-pixels-differ 52 | glycin ✗; image ✗; gifcrate ✗; gdk-anim 1; giflib ✗; libnsgif 1; stb ✗; gift 1; tinygif ✗ |
| struct:frame-beyond-canvas (48) | [#971](witnesses/anim-09-struct-frame-beyond-canvas.gif) | canvas-differs 48 | glycin 1; image 1; gifcrate 1; gdk-anim 1; giflib 1; libnsgif 1; stb ✗; gift ✗; tinygif ✗ |
| struct:frame-larger-than-canvas (48) | [#1022](witnesses/anim-10-struct-frame-larger-than-canvas.gif) | canvas-differs 48 | glycin 1; image 1; gifcrate 1; gdk-anim 1; giflib 1; libnsgif 1; stb ✗; gift ✗; tinygif ✗ |
| byte-flip (47) | [#2552](witnesses/anim-11-byte-flip.gif) | canvas-differs 29, sequence-pixels-differ 12, frame-count-differs 6 | glycin 1; image 1; gifcrate 1; gdk-anim 1; giflib 1; libnsgif 1; stb ✗; gift ✗; tinygif ✗ |
| field:image_flags (46) | [#8627](witnesses/anim-12-field-image_flags.gif) | sequence-pixels-differ 46 | glycin ✗; image ✗; gifcrate ✗; gdk-anim 1; giflib 1; libnsgif 1; stb 1; gift 1; tinygif ✗ |
| struct:duplicate-frame (44) | [#1175](witnesses/anim-13-struct-duplicate-frame.gif) | frame-count-differs 44 | glycin 2; image 2; gifcrate 2; gdk-anim 1; giflib 2; libnsgif 2; stb 2; gift 2; tinygif ✗ |
| lzw:first-subblock-zero (43) | [#461](witnesses/anim-14-lzw-first-subblock-zero.gif) | sequence-pixels-differ 37, frame-count-differs 6 | glycin 1; image 1; gifcrate 1; gdk-anim 1; giflib ✗; libnsgif 1; stb 1; gift ✗; tinygif ✗ |
| struct:interlace-toggle (36) | [#1328](witnesses/anim-15-struct-interlace-toggle.gif) | sequence-pixels-differ 36 | glycin 1; image 1; gifcrate 1; gdk-anim 1; giflib 1; libnsgif 1; stb 1; gift 1; tinygif ✗ |
| lzw:min-code-byte (27) | [#2705](witnesses/anim-16-lzw-min-code-byte.gif) | sequence-pixels-differ 27 | glycin ✗; image ✗; gifcrate ✗; gdk-anim 1; giflib 1; libnsgif ✗; stb 1; gift ✗; tinygif ✗ |
| struct:no-palette (27) | [#921](witnesses/anim-17-struct-no-palette.gif) | sequence-pixels-differ 27 | glycin ✗; image ✗; gifcrate ✗; gdk-anim ✗; giflib 1; libnsgif 1; stb ✗; gift ✗; tinygif ✗ |
| struct:short-global-palette (23) | [#972](witnesses/anim-18-struct-short-global-palette.gif) | sequence-pixels-differ 23 | glycin 1; image 1; gifcrate 1; gdk-anim 1; giflib 1; libnsgif 1; stb 1; gift ✗; tinygif ✗ |
| lzw:invalid-code (7) | [#309](witnesses/anim-19-lzw-invalid-code.gif) | sequence-pixels-differ 7 | glycin ✗; image ✗; gifcrate ✗; gdk-anim 1; giflib 1; libnsgif ✗; stb ✗; gift ✗; tinygif ✗ |
| struct:disposal (7) | [#4002](witnesses/anim-20-struct-disposal.gif) | sequence-pixels-differ 7 | glycin 2; image 2; gifcrate 2; gdk-anim 2; giflib 2; libnsgif 2; stb 2; gift 2; tinygif 2 |

## Acceptance by rule

Accepted inputs per rule (crash/timeout inputs excluded).

| Rule | n | glycin | image | gifcrate | gdk-anim | gdk-forced-whole | gdk-forced-byte | giflib | libnsgif | stb | gift | tinygif |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| byte-flip | 377 | 269 | 269 | 269 | 346 | 346 | 337 | 284 | 334 | 284 | 254 | 74 |
| field:aspect | 390 | 384 | 384 | 384 | 390 | 390 | 384 | 390 | 390 | 381 | 390 | 88 |
| field:background | 393 | 387 | 387 | 387 | 393 | 393 | 387 | 393 | 393 | 384 | 393 | 88 |
| field:delay | 90 | 84 | 84 | 84 | 90 | 90 | 84 | 90 | 90 | 90 | 90 | 84 |
| field:ext_label | 66 | 10 | 10 | 10 | 66 | 66 | 56 | 66 | 66 | 66 | 66 | 7 |
| field:first_sub_block_len | 378 | 79 | 79 | 79 | 79 | 70 | 69 | 0 | 296 | 168 | 2 | 84 |
| field:frame_height | 382 | 162 | 162 | 162 | 331 | 331 | 326 | 161 | 382 | 208 | 160 | 84 |
| field:frame_width | 368 | 161 | 161 | 161 | 331 | 331 | 326 | 197 | 368 | 199 | 165 | 70 |
| field:gce_flags | 92 | 86 | 86 | 86 | 92 | 92 | 86 | 92 | 92 | 92 | 92 | 86 |
| field:gce_size | 94 | 0 | 0 | 0 | 48 | 48 | 45 | 0 | 0 | 15 | 0 | 0 |
| field:gce_terminator | 90 | 0 | 0 | 0 | 25 | 25 | 25 | 0 | 0 | 0 | 0 | 0 |
| field:image_flags | 378 | 345 | 345 | 345 | 355 | 355 | 349 | 355 | 355 | 351 | 355 | 84 |
| field:left | 384 | 378 | 378 | 378 | 384 | 384 | 378 | 384 | 384 | 5 | 5 | 84 |
| field:lsd_flags | 405 | 110 | 110 | 110 | 146 | 146 | 145 | 110 | 110 | 107 | 110 | 4 |
| field:lzw_min | 373 | 20 | 20 | 20 | 296 | 296 | 291 | 35 | 123 | 0 | 53 | 79 |
| field:screen_height | 414 | 358 | 358 | 358 | 363 | 363 | 358 | 363 | 414 | 199 | 204 | 74 |
| field:screen_width | 417 | 362 | 362 | 362 | 367 | 367 | 362 | 367 | 417 | 198 | 203 | 74 |
| field:top | 382 | 376 | 376 | 376 | 382 | 382 | 376 | 382 | 382 | 5 | 5 | 84 |
| field:trailer | 378 | 372 | 372 | 372 | 0 | 0 | 0 | 0 | 378 | 370 | 378 | 84 |
| field:transparent_index | 90 | 84 | 84 | 84 | 90 | 90 | 84 | 90 | 90 | 90 | 90 | 84 |
| header:signature | 51 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| header:version | 204 | 50 | 50 | 50 | 51 | 51 | 50 | 204 | 204 | 50 | 51 | 14 |
| lzw:clear-every | 102 | 100 | 100 | 100 | 102 | 102 | 100 | 102 | 102 | 102 | 102 | 28 |
| lzw:deferred-clear | 51 | 50 | 50 | 50 | 51 | 51 | 50 | 51 | 51 | 49 | 51 | 14 |
| lzw:early-change | 51 | 10 | 10 | 10 | 51 | 51 | 50 | 12 | 11 | 9 | 9 | 14 |
| lzw:encoded-min-code | 349 | 342 | 342 | 342 | 349 | 349 | 342 | 216 | 349 | 346 | 225 | 98 |
| lzw:first-subblock-zero | 51 | 50 | 50 | 50 | 48 | 40 | 39 | 0 | 51 | 51 | 0 | 14 |
| lzw:garbage-after-eoi | 51 | 50 | 50 | 50 | 51 | 51 | 50 | 51 | 51 | 51 | 37 | 14 |
| lzw:invalid-code | 51 | 3 | 3 | 3 | 51 | 51 | 50 | 11 | 3 | 3 | 3 | 14 |
| lzw:min-code-byte | 428 | 250 | 250 | 250 | 428 | 428 | 421 | 228 | 363 | 363 | 225 | 98 |
| lzw:no-block-terminator | 51 | 37 | 37 | 37 | 1 | 1 | 1 | 0 | 51 | 50 | 0 | 14 |
| lzw:no-eoi | 51 | 50 | 50 | 50 | 51 | 51 | 50 | 51 | 51 | 51 | 51 | 14 |
| lzw:no-leading-clear | 51 | 50 | 50 | 50 | 51 | 51 | 50 | 51 | 51 | 0 | 51 | 14 |
| lzw:pixels-long | 51 | 50 | 50 | 50 | 51 | 51 | 50 | 51 | 51 | 50 | 51 | 14 |
| lzw:pixels-none | 51 | 0 | 0 | 0 | 51 | 51 | 50 | 0 | 0 | 51 | 1 | 14 |
| lzw:pixels-one-short | 51 | 0 | 0 | 0 | 51 | 51 | 50 | 0 | 48 | 50 | 8 | 14 |
| lzw:pixels-short | 51 | 0 | 0 | 0 | 51 | 51 | 50 | 0 | 48 | 51 | 1 | 14 |
| lzw:subblock | 102 | 100 | 100 | 100 | 102 | 102 | 100 | 102 | 102 | 100 | 102 | 28 |
| seed:anim | 7 | 7 | 7 | 7 | 7 | 7 | 7 | 7 | 7 | 7 | 7 | 6 |
| seed:basic | 14 | 14 | 14 | 14 | 14 | 14 | 14 | 14 | 14 | 14 | 14 | 0 |
| seed:big | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 2 | 3 | 0 |
| seed:ext | 5 | 4 | 4 | 4 | 5 | 5 | 4 | 5 | 5 | 5 | 5 | 4 |
| seed:frame | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 0 |
| seed:interlace | 5 | 5 | 5 | 5 | 5 | 5 | 5 | 5 | 5 | 5 | 5 | 0 |
| seed:local | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 0 |
| seed:size | 8 | 8 | 8 | 8 | 8 | 8 | 8 | 8 | 8 | 8 | 8 | 0 |
| seed:small | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 0 |
| seed:transparent | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 |
| struct:canvas-larger | 51 | 50 | 50 | 50 | 51 | 51 | 50 | 51 | 51 | 50 | 51 | 14 |
| struct:canvas-zero | 50 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 50 | 0 | 0 | 0 |
| struct:disposal | 385 | 378 | 378 | 378 | 385 | 385 | 378 | 385 | 385 | 385 | 385 | 378 |
| struct:duplicate-frame | 51 | 50 | 50 | 50 | 51 | 51 | 50 | 51 | 51 | 50 | 51 | 14 |
| struct:frame-beyond-canvas | 51 | 50 | 50 | 50 | 51 | 51 | 50 | 51 | 51 | 3 | 3 | 14 |
| struct:frame-larger-than-canvas | 51 | 50 | 50 | 50 | 51 | 51 | 50 | 51 | 51 | 3 | 3 | 14 |
| struct:gce-without-image | 50 | 49 | 49 | 49 | 50 | 50 | 49 | 50 | 50 | 50 | 35 | 49 |
| struct:insert-comment | 51 | 50 | 50 | 50 | 51 | 51 | 50 | 51 | 51 | 50 | 51 | 14 |
| struct:insert-netscape | 102 | 100 | 100 | 100 | 102 | 102 | 100 | 102 | 102 | 100 | 102 | 28 |
| struct:insert-plaintext | 51 | 50 | 50 | 50 | 51 | 51 | 50 | 51 | 51 | 50 | 51 | 14 |
| struct:insert-unknown-ext | 51 | 0 | 0 | 0 | 51 | 51 | 50 | 51 | 51 | 50 | 51 | 0 |
| struct:insert-xmp | 51 | 50 | 50 | 50 | 51 | 51 | 0 | 51 | 51 | 50 | 51 | 14 |
| struct:interlace-toggle | 51 | 48 | 48 | 48 | 51 | 51 | 50 | 51 | 51 | 50 | 51 | 14 |
| struct:no-palette | 37 | 0 | 0 | 0 | 0 | 0 | 0 | 37 | 37 | 0 | 0 | 0 |
| struct:no-trailer | 51 | 50 | 50 | 50 | 0 | 0 | 0 | 0 | 51 | 50 | 51 | 14 |
| struct:short-global-palette | 35 | 34 | 34 | 34 | 35 | 35 | 35 | 35 | 35 | 34 | 5 | 0 |
| struct:transparent-index | 194 | 190 | 190 | 190 | 194 | 194 | 190 | 194 | 194 | 194 | 194 | 190 |
| trailing-data | 306 | 300 | 300 | 300 | 306 | 306 | 300 | 306 | 306 | 300 | 306 | 84 |
| truncation | 429 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 91 | 0 | 0 |

## Crashes, timeouts and anomalies

Crash and timeout inputs are excluded from this public view; they are retained and counted only in the local matrix.

## Run

`./run.sh --inputs 10000 --seed 731` in `/__modal/volumes/vo-Iyh3VLkOB7cEHrIp2RkTs7/projects/leniency/gif` builds every runner under /tmp/claude-0 and regenerates this file; `python3 reproduce.py FILE` runs one file through every runner. Per-input JSONL and the manifest (provenance: seed, kind, rule, field, old/new value) live in `/tmp/claude-0/leniency-gif`.
