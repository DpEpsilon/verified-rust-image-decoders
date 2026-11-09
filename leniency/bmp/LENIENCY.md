# BMP/DIB leniency matrix

Inputs use identical bytes; each cell describes the **representative** witness for a provenance cluster, not every mutation in that cluster. See `summary.json` for cluster variability and `/tmp/claude-0/leniency-bmp/results.jsonl` for per-input observations. Pixel hashes cover top-first, straight RGBA8.

## Run and coverage

Seed 731; 10000 inputs (260 frozen/corpus, 9740 mutations) from 42 distinct header/depth/compression seeds; 6279 mutations change the decoder acceptance/pixel-equality pattern relative to their unmutated seed; elapsed 109.41 seconds.

Classification | Count
--- | ---:
all-accept-identical-pixels | 1522
accept-different-pixels | 282
accept-vs-reject | 3699
all-reject | 4497
crash-or-timeout | 0
All-reject error classes: same / different | 0 / 4497

Error-class agreement is literal; libraries use different error taxonomies, so different labels do not establish different validation rules.

## Representative matrix

| Rule (differing inputs; patterns) | Witness | verified | image-explicit | image-guess | glycin-image-rs | gdk-file | gdk-stream | gdk-whole | gdk-byte | gdk-chunk |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| planes (560; 10) | [#4046](witnesses/01-planes.bmp) | reject `InvalidPlanes` | reject `Format error decoding Bmp` | reject `Format error decoding Bmp` | reject `ExpectedError` | accept 1×1 `34aaa746` | accept 1×1 `34aaa746` | accept 1×1 `34aaa746` | accept 1×1 `34aaa746` | accept 1×1 `34aaa746` |
| biSizeImage (476; 10) | [#2366](witnesses/02-biSizeImage.bmp) | reject `InvalidImageSize` | accept 1×1 `34aaa746` | accept 1×1 `34aaa746` | accept 1×1 `34aaa746` | accept 1×1 `34aaa746` | accept 1×1 `34aaa746` | accept 1×1 `34aaa746` | accept 1×1 `34aaa746` | accept 1×1 `34aaa746` |
| height (475; 424) | [#5220](/tmp/claude-0/leniency-bmp/witnesses/03-height.bmp) | reject `InvalidImageSize` | reject `The decoder for Bmp does not` | reject `The decoder for Bmp does not` | reject `ExpectedError` | accept 127×1 `d8aa451d` | accept 127×1 `d8aa451d` | accept 127×1 `d8aa451d` | accept 127×1 `d8aa451d` | accept 127×1 `d8aa451d` |
| bfOffBits (426; 137) | [#3372](/tmp/claude-0/leniency-bmp/witnesses/04-bfOffBits.bmp) | reject `InvalidPixelOffset` | reject `The decoder for Bmp does not` | reject `The decoder for Bmp does not` | reject `ExpectedError` | accept 127×64 `ba4b375b` | accept 127×64 `ba4b375b` | accept 127×64 `ba4b375b` | accept 127×64 `ba4b375b` | accept 127×64 `ba4b375b` |
| width (330; 133) | [#2726](witnesses/05-width.bmp) | reject `TruncatedPixels` | reject `failed to fill whole buffer` | reject `failed to fill whole buffer` | reject `ExpectedError` | accept 39×1 `dc1a5f0b` | accept 39×1 `dc1a5f0b` | accept 39×1 `dc1a5f0b` | accept 39×1 `dc1a5f0b` | accept 39×1 `dc1a5f0b` |
| biClrUsed (199; 30) | [#6687](witnesses/06-biClrUsed.bmp) | reject `InvalidPixelOffset` | accept 1×1 `7a7bf454` | accept 1×1 `7a7bf454` | accept 1×1 `7a7bf454` | accept 1×1 `7a7bf454` | accept 1×1 `7a7bf454` | accept 1×1 `7a7bf454` | accept 1×1 `7a7bf454` | accept 1×1 `7a7bf454` |
| random-bitflip (180; 83) | [#1348](witnesses/07-random-bitflip.bmp) | reject `InvalidFileSize` | accept 1×1 `34aaa746` | accept 1×1 `34aaa746` | accept 1×1 `34aaa746` | accept 1×1 `34aaa746` | accept 1×1 `34aaa746` | accept 1×1 `34aaa746` | accept 1×1 `34aaa746` | accept 1×1 `34aaa746` |
| bitfield-mask (168; 89) | [#273](witnesses/08-bitfield-mask.bmp) | reject `InvalidBitfields` | reject `Format error decoding Bmp` | reject `Format error decoding Bmp` | reject `ExpectedError` | accept 1×1 `48f7fc19` | accept 1×1 `48f7fc19` | accept 1×1 `48f7fc19` | accept 1×1 `48f7fc19` | accept 1×1 `48f7fc19` |
| bfSize (124; 8) | [#8540](witnesses/09-bfSize.bmp) | reject `InvalidFileSize` | accept 1×1 `34aaa746` | accept 1×1 `34aaa746` | accept 1×1 `34aaa746` | accept 1×1 `34aaa746` | accept 1×1 `34aaa746` | accept 1×1 `34aaa746` | accept 1×1 `34aaa746` | accept 1×1 `34aaa746` |
| truncation (118; 15) | [#794](witnesses/10-truncation.bmp) | reject `InvalidFileSize` | reject `failed to fill whole buffer` | reject `failed to fill whole buffer` | reject `ExpectedError` | accept 1×1 `e3820096` | accept 1×1 `e3820096` | accept 1×1 `e3820096` | accept 1×1 `e3820096` | accept 1×1 `e3820096` |
| biSize (60; 36) | [#2869](witnesses/11-biSize.bmp) | reject `InvalidPixelOffset` | reject `failed to fill whole buffer` | reject `failed to fill whole buffer` | reject `ExpectedError` | accept 1×1 `e3820096` | accept 1×1 `e3820096` | accept 1×1 `e3820096` | accept 1×1 `e3820096` | accept 1×1 `e3820096` |
| frozen:BareDib (54; 25) | [#27](witnesses/12-frozen-BareDib.bmp) | reject `InvalidPixelOffset` | reject `failed to fill whole buffer` | reject `failed to fill whole buffer` | reject `ExpectedError` | accept 1×1 `e3820096` | accept 1×1 `e3820096` | accept 1×1 `e3820096` | accept 1×1 `e3820096` | accept 1×1 `e3820096` |
| inconsistent-pair (49; 29) | [#2324](witnesses/13-inconsistent-pair.bmp) | reject `InvalidImageSize` | accept 1×1 `34aaa746` | accept 1×1 `34aaa746` | accept 1×1 `34aaa746` | accept 1×1 `34aaa746` | accept 1×1 `34aaa746` | accept 1×1 `34aaa746` | accept 1×1 `34aaa746` | accept 1×1 `34aaa746` |
| rle-escape (32; 14) | [#1556](/tmp/claude-0/leniency-bmp/witnesses/14-rle-escape.bmp) | reject `RleBounds` | accept 256×10 `79c0e2de` | accept 256×10 `79c0e2de` | accept 256×10 `79c0e2de` | accept 256×10 `79c0e2de` | accept 256×10 `79c0e2de` | accept 256×10 `79c0e2de` | accept 256×10 `79c0e2de` | accept 256×10 `79c0e2de` |
| frozen:BmpFile (24; 16) | [#67](witnesses/15-frozen-BmpFile.bmp) | reject `TruncatedPixels` | reject `failed to fill whole buffer` | reject `failed to fill whole buffer` | reject `ExpectedError` | accept 1×1 `e3820096` | accept 1×1 `e3820096` | accept 1×1 `e3820096` | accept 1×1 `e3820096` | accept 1×1 `e3820096` |
| local-corpus (21; 20) | [#181](/tmp/claude-0/leniency-bmp/witnesses/16-local-corpus.bmp) | reject `InvalidImageSize` | accept 127×64 `54483daf` | accept 127×64 `54483daf` | accept 127×64 `54483daf` | accept 127×64 `54483daf` | accept 127×64 `54483daf` | accept 127×64 `54483daf` | accept 127×64 `54483daf` | accept 127×64 `54483daf` |
| bitCount (11; 7) | [#3421](witnesses/17-bitCount.bmp) | reject `InvalidPixelOffset` | reject `failed to fill whole buffer` | reject `failed to fill whole buffer` | reject `ExpectedError` | accept 1×1 `e3820096` | accept 1×1 `e3820096` | accept 1×1 `e3820096` | accept 1×1 `e3820096` | accept 1×1 `e3820096` |
| compression (7; 7) | [#3908](/tmp/claude-0/leniency-bmp/witnesses/18-compression.bmp) | reject `InvalidImageSize` | accept 256×10 `9e227177` | accept 256×10 `9e227177` | accept 256×10 `9e227177` | accept 256×10 `9e227177` | accept 256×10 `9e227177` | accept 256×10 `9e227177` | accept 256×10 `9e227177` | accept 256×10 `9e227177` |

## Ten significant observations

- `planes=0`: GDK accepts the image; verified core and image-rs reject it. [Witness](witnesses/01-planes.bmp).
- `biSizeImage=1` instead of zero: verified core rejects; image-rs and GDK decode identical pixels. [Witness](witnesses/02-biSizeImage.bmp).
- `bfSize=63` for a 58-byte file: verified core rejects the inflated declared size; image-rs and GDK accept. [Witness](witnesses/09-bfSize.bmp).
- A zero red bitfield mask: GDK decodes the image; verified core and image-rs reject it. [Witness](witnesses/08-bitfield-mask.bmp).
- Replacing the terminal RLE escape with end-of-line: verified core reports `RleBounds`; image-rs and GDK accept. [Witness](/tmp/claude-0/leniency-bmp/witnesses/14-rle-escape.bmp).
- One 16-bit component scales to 25 in verified core/image-rs but 24 in GDK. [Witness](witnesses/pixel-01-frozen-BareDib.bmp).
- An indexed BMP with a palette gap retains colored pixels in verified core/image-rs; GDK outputs black. [Witness](witnesses/pixel-02-frozen-BmpFile.bmp).
- An RLE escape produces opaque black in verified core/image-rs and transparent black in GDK. [Witness](witnesses/pixel-03-rle-escape.bmp).
- Verified core accepts a wrapped 16-byte OS/2 DIB header that image-rs and GDK reject. [Witness](witnesses/verified-only-header16.bmp).
- GDK's `.bmp` filename path accepts a damaged signature that its stream and all incremental chunk patterns reject; renaming the file `.dat` makes the filename path reject it. [Witness](witnesses/gdk-path-01.bmp).

## Same acceptance, different pixels

These witnesses keep the same accept/size/hash pattern through bounded delta-debugging. The category names describe byte-level differences, not presumed decoder intent.
- frozen:BareDib bi-rgb-16-630c: 5-bit-scaling; verified `5fa6a5d9` versus GDK `2cf80195`; [witness](witnesses/pixel-01-frozen-BareDib.bmp) (input #13).
- frozen:BmpFile bmp-indexed-gap-1: color-values-or-layout; verified `e40be0f0` versus GDK `b3b2d513`; [witness](witnesses/pixel-02-frozen-BmpFile.bmp) (input #125).
- rle-escape escape@70=0: alpha-only; verified `b3b2d513` versus GDK `15ec7bf0`; [witness](witnesses/pixel-03-rle-escape.bmp) (input #262).
- bitfield-mask blueMask=1: color-values-or-layout; verified `1eb1feba` versus GDK `4bb8e4ac`; [witness](witnesses/pixel-04-bitfield-mask.bmp) (input #819).
- width width=7: alpha-only; verified `68f48b45` versus GDK `2f638cf4`; [witness](witnesses/pixel-05-width.bmp) (input #8299).

## where our core's choices disagree with ALL or MOST other decoders

- `biSizeImage`: verified rejects while image-rs, glycin-image-rs and gdk-file all disagree; [witness](witnesses/02-biSizeImage.bmp).
- `biClrUsed`: verified rejects while image-rs, glycin-image-rs and gdk-file all disagree; [witness](witnesses/06-biClrUsed.bmp).
- `random-bitflip`: verified rejects while image-rs, glycin-image-rs and gdk-file all disagree; [witness](witnesses/07-random-bitflip.bmp).
- `bfSize`: verified rejects while image-rs, glycin-image-rs and gdk-file all disagree; [witness](witnesses/09-bfSize.bmp).
- `inconsistent-pair`: verified rejects while image-rs, glycin-image-rs and gdk-file all disagree; [witness](witnesses/13-inconsistent-pair.bmp).
- `rle-escape`: verified rejects while image-rs, glycin-image-rs and gdk-file all disagree; [witness](/tmp/claude-0/leniency-bmp/witnesses/14-rle-escape.bmp).
- `local-corpus`: verified rejects while image-rs, glycin-image-rs and gdk-file all disagree; [witness](/tmp/claude-0/leniency-bmp/witnesses/16-local-corpus.bmp).
- `compression`: verified rejects while image-rs, glycin-image-rs and gdk-file all disagree; [witness](/tmp/claude-0/leniency-bmp/witnesses/18-compression.bmp).
- The verified core **accepts** a valid wrapped 16-byte OS/2 header that image-rs and GDK reject; [witness](witnesses/verified-only-header16.bmp). This is a verified-core leniency/format-coverage choice, not an error exposed by a mutation.
Image-rs and glycin-image-rs share the BMP decoder implementation; their acceptance/dimensions/pixel hashes differ on 0 of 10000 inputs. Treat their agreement as one implementation, not two independent votes.

## GDK incremental versus whole-file

The installed BMP loader is `/usr/lib/x86_64-linux-gnu/gdk-pixbuf-2.0/2.10.0/loaders/libpixbufloader-bmp.so` (module, not built-in). The runner compares `new_from_file`, `new_from_stream`, and `GdkPixbufLoader` with whole-buffer, byte and 4096-byte writes.
Different statuses, dimensions or pixels on 3 input(s): [2345, 2451, 3165].
- Input #2345: [witness](witnesses/gdk-path-01.bmp); compare `gdk-file` with `gdk-stream` and all loader chunk sizes.
- Input #2451: [witness](witnesses/gdk-path-02.bmp); compare `gdk-file` with `gdk-stream` and all loader chunk sizes.
In this run the filename path accepts two distinct damaged BMP signatures because the `.bmp` suffix selects BMP; the stream and all loader chunk patterns reject them as unrecognized. Renaming the first witness to `.dat` makes the file path reject it too.

## Crashes, timeouts, and anomalies

0 inputs have a crash or timeout. Up to 30 inputs are saved under `/tmp/claude-0/leniency-bmp/anomalies/`; no exploitation or upstream reporting was attempted.

## Reproduce

Run `python3 harness.py --inputs 10000 --seed 731` from this directory after `./build.sh`. A single witness: `python3 reproduce.py PATH_FROM_MATRIX`. Witness numbers and paths may change when the run size changes. The manifest and all input files are in `/tmp/claude-0/leniency-bmp/`, and are regenerated for a new run.
