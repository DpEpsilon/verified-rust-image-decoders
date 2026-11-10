# PNG leniency matrix

Seed 731; 9986 synthetic inputs (crash/timeout inputs excluded); 70 unmutated generator seeds; 70 seeds received mutations; 8133 mutations changed the acceptance/pixel relation relative to their seed; decoder pass 478.8 s (one CPU), witness minimisation afterwards.

Classification (first displayed frame, top-first straight RGBA8) | Count
--- | ---:
all-accept-identical-pixels | 1187
accept-different-pixels | 102
accept-vs-reject | 4664
all-reject | 4033

Animation comparison among accepting animation-capable runners (glycin, glycin-apng, image, pngcrate-apng) | Count
--- | ---:
identical | 3455
n/a | 6498
sequence-pixels-differ | 47

## Decoders

`glycin` / `glycin-apng` call the in-tree `glycin_image_rs::ImgLoader` with mime `image/png` / `image/apng` and request frames with `loop_animation=false` until `NoMoreFrames` (glycin animates any PNG whose `is_apng()` is true, under either mime). glycin, `image` (=0.25.10 `PngDecoder`, APNG via `apng().into_frames()`) and `pngcrate` share the `png` crate built from the same image-png git revision glycin's workspace patches in (e917d6a): **one vote**. `pngcrate` = default image with `Transformations::EXPAND`; `pngcrate-apng` = the png crate's fcTL frames through the harness APNG compositor (transparent canvas; dispose 0/1/2, first-frame PREVIOUS treated as BACKGROUND; blend SOURCE, or OVER on straight 8-bit samples in f32 with truncation, the arithmetic image-rs uses). `libpng` = system libpng 1.6.48 (png_read_row, expand, gray→RGB, add alpha, expand_16, default CRC and benign-error policy); gdk-pixbuf 2.42.12's built-in PNG loader is also libpng, so GDK and libpng are one family. `spng` (libspng 0.7.x, 46c81dc), `lodepng` crate 3.12 (Rust LodePNG), `stb` (stb_image 2.30), `zune` (zune-png 0.5.2) and `minipng` 1.0 are independent; none of the non-image-rs decoders implement APNG, so they return the default (IDAT) image. **16-bit normalisation:** every runner that sees 16-bit samples reports `rgba16_sha256` over exact big-endian RGBA16 and reduces to 8 bits by the high byte (`v >> 8`, libpng `png_set_strip_16`, which GDK uses); minipng and GDK only expose 8 bits. **Colour management:** no runner applies gAMA, cHRM, sRGB or iCCP; hashes compare raw decoded samples. A `tail_error` means the decoder returned pixels but reported an error after them (libpng `png_read_end`, spng `spng_decode_chunks`). lodepng: 16-bit sources use its RGBA16 output (byte-swapped back from the crate's big-endian pairs), sources up to 8 bits use `decode32`.

| Runner | Family (one vote) | ok | error | lenient (accepts, independent majority rejects) | strict (rejects, majority accepts) | first-frame pixel outlier | tail error after pixels |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| glycin | image-rs/png-crate | 3502 | 6484 | 301 | 407 | 226 | 105 |
| glycin-apng | image-rs/png-crate | 3502 | 6484 | 301 | 407 | 226 | 105 |
| image | image-rs/png-crate | 3502 | 6484 | 301 | 407 | 226 | 105 |
| pngcrate | image-rs/png-crate | 3691 | 6295 | 314 | 231 | 1 | 0 |
| pngcrate-apng | image-rs/png-crate | 3570 | 6416 | 309 | 347 | 227 | 667 |
| libpng | libpng | 3758 | 6228 | 386 | 236 | 0 | 647 |
| gdk-anim | libpng | 3316 | 6670 | 678 | 970 | 250 | 0 |
| gdk-forced-whole | libpng | 4087 | 5899 | 767 | 288 | 283 | 0 |
| gdk-forced-byte | libpng | 4087 | 5899 | 767 | 288 | 213 | 0 |
| spng | spng | 4142 | 5844 | 592 | 58 | 0 | 1056 |
| lodepng | lodepng | 3204 | 6782 | 762 | 1166 | 0 | 0 |
| stb | stb_image | 3824 | 6162 | 312 | 96 | 18 | 0 |
| zune | zune-png | 3856 | 6130 | 720 | 472 | 62 | 0 |
| minipng | minipng | 3217 | 6769 | 522 | 913 | 105 | 0 |

Frame-delay lists differing from `pngcrate-apng` (raw file delays) when frame counts agree: glycin 58, glycin-apng 58.

Unavailable or not integrated candidates:

- ImageMagick, ffmpeg, pngcheck: not installed; not built from source.
- wuffs PNG (github.com/google/wuffs): buildable, not integrated in this pass.
- APNG in libpng/spng/stb/lodepng/zune/minipng: not supported by these builds (system libpng has no APNG patch); they show the default image.
- The `png` crate as an independent vote: not independent (image-rs and glycin use it).

## Ten significant observations

- GDK returns a pixbuf when IDAT decompression fails: with a bad zlib header, raw deflate without zlib wrapper, too little image data or a bad compression-method byte, `GdkPixbufLoader` (whole and byte writes) closes without error and yields an image filled with a constant grey, while libpng itself, glycin, spng and stb reject (zlib:bad-zlib-header, zlib:raw-deflate, zlib:too-little-data, zlib:bad-cm: 70 of 70 accepted by GDK).
- Adler-32: glycin/image-rs, GDK, stb, zune and minipng ignore a wrong zlib Adler-32; libpng, spng and lodepng reject (zlib:bad-adler32). A missing Adler-32 is accepted by glycin (64/70), spng, stb, zune and minipng.
- CRC on critical chunks: stb and zune never check CRCs (229 of 229 accepted), minipng accepts 177 and spng 135; libpng (69), glycin (66) and lodepng (40) accept mainly cases where the corrupted CRC belongs to a chunk they do not need (crc:critical).
- IHDR compression and filter-method bytes must be 0: lodepng accepts any value (348/348 and 347/347) and zune accepts nonzero filter methods (268/347); every other decoder rejects.
- Missing IEND: libpng and GDK accept all 70; glycin accepts 13, lodepng, stb, zune and minipng reject (order:missing-iend). Conversely a wrong IEND length is accepted by glycin, libpng, spng and stb but rejected by lodepng and zune (length:IEND).
- Palette images without PLTE (or with PLTE after IDAT, or empty): lodepng, zune and minipng accept and invent colours; glycin, libpng, GDK, spng and stb reject (order:missing-plte). zune also applies a PLTE that appears in a greyscale image to the grey samples (order:plte, rgb-only difference).
- APNG: glycin rejects an acTL without any fcTL (58 of 58) and every fcTL sequence-number, size or offset error, whereas non-APNG decoders show the default image; image-rs/glycin reject 16-bit APNG (“does not support the color type Rgba16”); GDK’s file/animation path rejects every APNG (content sniffing yields image/apng, which no GDK loader claims) while the forced PNG loader shows the default image.
- tRNS: minipng ignores a greyscale tRNS key (alpha-only difference on gray2/gray8 seeds); lodepng, stb and zune reject a tRNS of wrong length, tRNS in a colour type with alpha or tRNS before PLTE, which libpng, GDK, spng and image-rs accept (order:trns-*). The lodepng crate’s RGBA16 conversion also drops a grey key below 8 bits; the harness uses its RGBA8 path for such sources.
- Chunk-type bits: a lower-case third letter (reserved bit) is accepted by glycin, spng and libpng for 52–94 inputs but rejected by GDK (2); turning IDAT/PLTE ancillary by case makes stb, lodepng, zune and minipng reject where libpng-based and image-rs decoders accept (type-bit:*).
- Declared IHDR height larger than the data: GDK accepts 225 of 343 (partially filled image) vs 114 for libpng/spng/stb and 91 for glycin; lodepng accepts none (ihdr:height). 16-bit samples agree bit-exactly across every decoder that exposes them; with the high-byte rule the 8-bit hashes also agree, so no 16-bit normalisation disagreement was observed.

## Representative leniency rows

A row is the most frequent full outcome signature among *changed* mutations of one rule (a mutation whose acceptance/pixel-equality pattern equals its seed's is not counted). `n; p` = changed inputs; distinct patterns. Cells: `W×H hash8` or `reject class`. Witnesses are delta-debugged under the same signature (bounded, not globally minimal).



| Rule (n; p) | Witness | glycin | glycin-apng | image | pngcrate | pngcrate-apng | libpng | gdk-anim | gdk-forced-whole | gdk-forced-byte | spng | lodepng | stb | zune | minipng |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ihdr:compression (348; 70) | [#350](witnesses/01-ihdr-compression.png) 110B | ✗ `ExpectedError` | ✗ `ExpectedError` | ✗ `Format error decod` | ✗ `Unknown compressio` | ✗ `Unknown compressio` | ✗ `libpng` | ✗ `gdk` | ✗ `gdk` | ✗ `gdk` | ✗ `spng` | 9×7 `10a50d` | ✗ `stbi` | ✗ `"Unknown compressi` | ✗ `BadIhdr` |
| ihdr:filter_method (347; 122) | [#420](witnesses/02-ihdr-filter_method.png) 110B | ✗ `ExpectedError` | ✗ `ExpectedError` | ✗ `Format error decod` | ✗ `Unknown filter met` | ✗ `Unknown filter met` | ✗ `libpng` | ✗ `gdk` | ✗ `gdk` | ✗ `gdk` | ✗ `spng` | 9×7 `10a50d` | ✗ `stbi` | 9×7 `10a50d` | ✗ `BadIhdr` |
| length:IEND (309; 181) | [#1053](witnesses/03-length-IEND.png) 37356B | 150×130 `55863e` | 150×130 `55863e` | 150×130 `55863e` | 150×130 `55863e` | 150×130 `55863e` | 150×130 `55863e` | 150×130 `55863e` | 150×130 `55863e` | 150×130 `55863e` | 150×130 `55863e` | ✗ `the length of the ` | 150×130 `55863e` | ✗ `I/O error Not enou` | ✗ `UnsupportedInterla` |
| length:IDAT (233; 110) | [#5393](witnesses/04-length-IDAT.png) 37356B | ✗ `ExpectedError` | ✗ `ExpectedError` | ✗ `Format error decod` | ✗ `CRC error` | ✗ `CRC error` | ✗ `libpng` | ✗ `gdk` | ✗ `gdk` | ✗ `gdk` | 150×130 `55863e` | ✗ `invalid CRC encoun` | ✗ `stbi` | ✗ `I/O error Not enou` | ✗ `UnsupportedInterla` |
| ihdr:height (225; 225) | [#4560](witnesses/05-ihdr-height.png) 67B | ✗ `ExpectedError` | ✗ `ExpectedError` | ✗ `Format error decod` | ✗ `IDAT or fDAT chunk` | ✗ `IDAT or fDAT chunk` | ✗ `libpng` | 1×2 `bd4534` | 1×2 `bd4534` | 1×2 `bd4534` | ✗ `spng` | ✗ `invalid decompress` | ✗ `stbi` | ✗ `"Not enough pixels` | 1×2 `6639c2` |
| crc:critical (209; 187) | [#586](witnesses/06-crc-critical.png) 1345B | ✗ `ExpectedError` | ✗ `ExpectedError` | ✗ `Format error decod` | ✗ `CRC error` | ✗ `CRC error` | ✗ `libpng` | ✗ `gdk` | ✗ `gdk` | ✗ `gdk` | ✗ `spng` | ✗ `invalid CRC encoun` | 20×20 `83c0f1` | 20×20 `83c0f1` | 20×20 `83c0f1` |
| byte-flip (193; 172) | [#4522](witnesses/07-byte-flip.png) 656B | ✗ `ExpectedError` | ✗ `ExpectedError` | ✗ `Format error decod` | ✗ `CRC error` | ✗ `CRC error` | ✗ `libpng` | ✗ `gdk` | ✗ `gdk` | ✗ `gdk` | ✗ `spng` | ✗ `end of in buffer m` | 12×12 `5ef38e` | 12×12 `5ef38e` | 12×12 `5ef38e` |
| type-bit:reserved (92; 88) | [#5609](witnesses/08-type-bit-reserved.png) 264B | 7×7 `7d2f9c` | 7×7 `7d2f9c` | 7×7 `7d2f9c` | 7×7 `7d2f9c` | 7×7 `7d2f9c` | ✗ `libpng` | ✗ `gdk` | ✗ `gdk` | ✗ `gdk` | 7×7 `7d2f9c` | 7×7 `7d2f9c` | 7×7 `7d2f9c` | 7×7 `7d2f9c` | 7×7 `7d2f9c` |
| order:missing-iend (70; 70) | [#1260](witnesses/09-order-missing-iend.png) 98B | ✗ `ExpectedError` | ✗ `ExpectedError` | ✗ `unexpected end of ` | ✗ `unexpected end of ` | ✗ `unexpected end of ` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | ✗ `jumped past memory` | ✗ `stbi` | ✗ `I/O error Not enou` | ✗ `UnexpectedEof` |
| order:ztxt-bad-zlib (70; 70) | [#2660](witnesses/10-order-ztxt-bad-zlib.png) 129B | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | ✗ `invalid FCHECK in ` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` |
| order:phys-short (70; 70) | [#2730](witnesses/11-order-phys-short.png) 124B | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | ✗ `invalid pHYs chunk` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` |
| zlib:bad-zlib-header (70; 21) | [#3641](witnesses/12-zlib-bad-zlib-header.png) 102B | ✗ `ExpectedError` | ✗ `ExpectedError` | ✗ `Format error decod` | ✗ `Corrupt deflate st` | ✗ `Corrupt deflate st` | ✗ `libpng` | 7×7 `5bdc59` | 7×7 `5bdc59` | 7×7 `5bdc59` | ✗ `spng` | ✗ `end of in buffer m` | ✗ `stbi` | ✗ `Error decoding ida` | ✗ `BadZlibHeader` |
| zlib:zlib-fdict (70; 70) | [#3010](witnesses/13-zlib-zlib-fdict.png) 110B | ✗ `ExpectedError` | ✗ `ExpectedError` | ✗ `Format error decod` | ✗ `Corrupt deflate st` | ✗ `Corrupt deflate st` | ✗ `libpng` | ✗ `gdk` | ✗ `gdk` | ✗ `gdk` | ✗ `spng` | ✗ `end of in buffer m` | ✗ `stbi` | 9×7 `10a50d` | 9×7 `10a50d` |
| zlib:bad-cm (70; 21) | [#3781](witnesses/14-zlib-bad-cm.png) 102B | ✗ `ExpectedError` | ✗ `ExpectedError` | ✗ `Format error decod` | ✗ `Corrupt deflate st` | ✗ `Corrupt deflate st` | ✗ `libpng` | 7×7 `5bdc59` | 7×7 `5bdc59` | 7×7 `5bdc59` | ✗ `spng` | ✗ `end of in buffer m` | ✗ `stbi` | ✗ `Error decoding ida` | ✗ `BadZlibHeader` |
| zlib:bad-adler32 (70; 70) | [#3150](witnesses/15-zlib-bad-adler32.png) 110B | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | ✗ `libpng` | 9×7 `93f6dc` | 9×7 `93f6dc` | 9×7 `10a50d` | ✗ `spng` | ✗ `end of in buffer m` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` |
| zlib:missing-adler32 (70; 70) | [#3220](witnesses/16-zlib-missing-adler32.png) 106B | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | ✗ `libpng` | ✗ `gdk` | ✗ `gdk` | ✗ `gdk` | 9×7 `10a50d` | ✗ `end of in buffer m` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` |
| zlib:extra-after-zlib (70; 70) | [#3360](witnesses/17-zlib-extra-after-zlib.png) 116B | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | ✗ `end of in buffer m` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` |
| zlib:raw-deflate (70; 21) | [#4131](witnesses/18-zlib-raw-deflate.png) 96B | ✗ `ExpectedError` | ✗ `ExpectedError` | ✗ `Format error decod` | ✗ `Corrupt deflate st` | ✗ `Corrupt deflate st` | ✗ `libpng` | 7×7 `5bdc59` | 7×7 `5bdc59` | 7×7 `5bdc59` | ✗ `spng` | ✗ `end of in buffer m` | ✗ `stbi` | ✗ `Error decoding ida` | ✗ `BadZlibHeader` |
| zlib:too-little-data (70; 69) | [#3609](witnesses/19-zlib-too-little-data.png) 54B | ✗ `ExpectedError` | ✗ `ExpectedError` | ✗ `Format error decod` | ✗ `IDAT or fDAT chunk` | ✗ `IDAT or fDAT chunk` | ✗ `libpng` | 1×1 `be63b3` | 1×1 `be63b3` | 1×1 `be63b3` | ✗ `spng` | ✗ `invalid decompress` | ✗ `stbi` | ✗ `"Too short data"` | ✗ `UnsupportedInterla` |
| zlib:one-byte-short (70; 69) | [#3679](witnesses/20-zlib-one-byte-short.png) 54B | ✗ `ExpectedError` | ✗ `ExpectedError` | ✗ `Format error decod` | ✗ `IDAT or fDAT chunk` | ✗ `IDAT or fDAT chunk` | ✗ `libpng` | 1×1 `be63b3` | 1×1 `be63b3` | 1×1 `be63b3` | ✗ `spng` | ✗ `invalid decompress` | ✗ `stbi` | ✗ `"Too short data"` | ✗ `UnsupportedInterla` |
| zlib:one-byte-long (70; 70) | [#3640](witnesses/21-zlib-one-byte-long.png) 111B | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | ✗ `invalid decompress` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` |
| zlib:extra-row (70; 70) | [#3710](witnesses/22-zlib-extra-row.png) 112B | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | ✗ `invalid decompress` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` |
| zlib:two-zlib-streams (70; 70) | [#3780](witnesses/23-zlib-two-zlib-streams.png) 163B | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | ✗ `end of in buffer m` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` |
| type-bit:ancillary (68; 68) | [#5136](witnesses/24-type-bit-ancillary.png) 255B | ✗ `ExpectedError` | ✗ `ExpectedError` | ✗ `Format error decod` | ✗ `Corrupt deflate st` | ✗ `Corrupt deflate st` | ✗ `libpng` | 20×20 `9eefb9` | 20×20 `9eefb9` | 20×20 `9eefb9` | ✗ `spng` | ✗ `end of in buffer m` | ✗ `stbi` | ✗ `Error decoding ida` | ✗ `BadZlibHeader` |
| type-bit:safe-to-copy (60; 60) | [#6653](witnesses/25-type-bit-safe-to-copy.png) 37356B | 150×130 `55863e` | 150×130 `55863e` | 150×130 `55863e` | 150×130 `55863e` | 150×130 `55863e` | 150×130 `55863e` | ✗ `gdk` | ✗ `gdk` | ✗ `gdk` | 150×130 `55863e` | ✗ `unknown chunk type` | ✗ `stbi` | ✗ `"Marker IENd unkno` | ✗ `UnsupportedInterla` |
| type-bit:private (60; 60) | [#6723](witnesses/26-type-bit-private.png) 37356B | 150×130 `55863e` | 150×130 `55863e` | 150×130 `55863e` | 150×130 `55863e` | 150×130 `55863e` | 150×130 `55863e` | ✗ `gdk` | ✗ `gdk` | ✗ `gdk` | 150×130 `55863e` | ✗ `unknown chunk type` | ✗ `stbi` | ✗ `"Marker IeND unkno` | ✗ `UnsupportedInterla` |
| apng:actl-without-fctl (58; 58) | [#4133](witnesses/27-apng-actl-without-fctl.png) 37376B | ✗ `ExpectedError` | ✗ `ExpectedError` | ✗ `The encoder or dec` | 150×130 `55863e` | ✗ `IDAT or fdAT chunk` | 150×130 `55863e` | ✗ `gdk` | 150×130 `55863e` | 150×130 `55863e` | 150×130 `55863e` | 150×130 `55863e` | 150×130 `55863e` | 150×130 `55863e` | ✗ `UnsupportedInterla` |
| order:trns-wrong-length (56; 56) | [#2870](witnesses/28-order-trns-wrong-length.png) 123B | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | 9×7 `10a50d` | ✗ `chunk length too l` | ✗ `stbi` | ✗ `I/O error Not enou` | 9×7 `10a50d` |
| ihdr:width (50; 40) | [#9450](witnesses/29-ihdr-width.png) 42B | ✗ `ExpectedError` | ✗ `ExpectedError` | ✗ `Format error decod` | ✗ `IDAT or fDAT chunk` | ✗ `IDAT or fDAT chunk` | ✗ `libpng` | 65536×7 `6ce585` | 65536×7 `6ce585` | 65536×7 `6ce585` | ✗ `spng` | ✗ `invalid decompress` | ✗ `stbi` | ✗ `"Image width 65536` | ✗ `CompressedSizeTooS` |
| fctl:seq (35; 12) | [#4627](witnesses/30-fctl-seq.png) 385B | ✗ `ExpectedError` | ✗ `ExpectedError` | ✗ `Format error decod` | ✗ `Sequence is not in` | ✗ `Sequence is not in` | 8×8 `3250b0` | ✗ `gdk` | 8×8 `3250b0` | 8×8 `3250b0` | 8×8 `3250b0` | 8×8 `3250b0` | 8×8 `3250b0` | 8×8 `3250b0` | 8×8 `3250b0` |
| fctl:width (35; 12) | [#4697](witnesses/31-fctl-width.png) 385B | ✗ `ExpectedError` | ✗ `ExpectedError` | ✗ `Format error decod` | ✗ `Invalid image dime` | ✗ `Invalid image dime` | 8×8 `3250b0` | ✗ `gdk` | 8×8 `3250b0` | 8×8 `3250b0` | 8×8 `3250b0` | 8×8 `3250b0` | 8×8 `3250b0` | ✗ `"Unknown filter 21` | 8×8 `3250b0` |
| ihdr:color_type (33; 33; rgb-only: 24 of 49 px (glycin+glycin-apng+image vs zune)) | [#281](witnesses/32-ihdr-color_type.png) 133B | 7×7 `7a6ad7` | 7×7 `7a6ad7` | 7×7 `7a6ad7` | 7×7 `7a6ad7` | 7×7 `7a6ad7` | 7×7 `7a6ad7` | 7×7 `7a6ad7` | 7×7 `7a6ad7` | 7×7 `7a6ad7` | 7×7 `7a6ad7` | 7×7 `7a6ad7` | 7×7 `7a6ad7` | 7×7 `e2b899` | 7×7 `7a6ad7` |
| apng:add-single-frame (33; 33) | [#4063](witnesses/33-apng-add-single-frame.png) 37414B | ✗ `ExpectedError` | ✗ `ExpectedError` | ✗ `The encoder or dec` | 150×130 `55863e` | 150×130 `55863e` | 150×130 `55863e` | ✗ `gdk` | 150×130 `55863e` | 150×130 `55863e` | 150×130 `55863e` | 150×130 `55863e` | 150×130 `55863e` | 150×130 `55863e` | ✗ `UnsupportedInterla` |
| length:fdAT (32; 12) | [#1262](witnesses/34-length-fdAT.png) 519B | 8×8 `aa4e58` | 8×8 `aa4e58` | 8×8 `aa4e58` | 8×8 `aa4e58` | 8×8 `aa4e58` | 8×8 `aa4e58` | ✗ `gdk` | ✗ `gdk` | ✗ `gdk` | 8×8 `aa4e58` | ✗ `the length of the ` | ✗ `stbi` | ✗ `I/O error Not enou` | ✗ `BadChunkSize` |
| fctl:height (32; 31) | [#4868](witnesses/35-fctl-height.png) 1161B | ✗ `ExpectedError` | ✗ `ExpectedError` | ✗ `Format error decod` | 12×10 `0431ca` | ✗ `Invalid image dime` | 12×10 `0431ca` | ✗ `gdk` | 12×10 `0431ca` | 12×10 `0431ca` | 12×10 `0431ca` | 12×10 `0431ca` | 12×10 `0431ca` | 12×10 `0431ca` | 12×10 `0431ca` |
| crc:ancillary (28; 26) | [#996](witnesses/36-crc-ancillary.png) 232B | 7×7 `6e5374` | 7×7 `6e5374` | 7×7 `6e5374` | 7×7 `6e5374` | 7×7 `6e5374` | 7×7 `6e5374` | 7×7 `6e5374` | 7×7 `6e5374` | 7×7 `6e5374` | 7×7 `6e5374` | ✗ `invalid CRC encoun` | 7×7 `6e5374` | 7×7 `6e5374` | 7×7 `6e5374` |
| order:trns-with-alpha (26; 26) | [#2944](witnesses/37-order-trns-with-alpha.png) 325B | 9×7 `49b805` | 9×7 `49b805` | 9×7 `49b805` | 9×7 `49b805` | 9×7 `49b805` | 9×7 `49b805` | 9×7 `49b805` | 9×7 `49b805` | 9×7 `49b805` | 9×7 `49b805` | ✗ `tRNS chunk appeare` | ✗ `stbi` | ✗ `"A tRNS chunk shal` | 9×7 `49b805` |
| fctl:x_offset (24; 12) | [#4837](witnesses/38-fctl-x_offset.png) 385B | ✗ `ExpectedError` | ✗ `ExpectedError` | ✗ `Format error decod` | ✗ `Sub frame is out-o` | ✗ `Sub frame is out-o` | 8×8 `3250b0` | ✗ `gdk` | 8×8 `3250b0` | 8×8 `3250b0` | 8×8 `3250b0` | 8×8 `3250b0` | 8×8 `3250b0` | 8×8 `3250b0` | 8×8 `3250b0` |
| fctl:blend_op (24; 23) | [#7497](witnesses/39-fctl-blend_op.png) 385B | ✗ `ExpectedError` | ✗ `ExpectedError` | ✗ `Format error decod` | ✗ `Invalid blend op 2` | ✗ `Invalid blend op 2` | 8×8 `3250b0` | ✗ `gdk` | 8×8 `3250b0` | 8×8 `3250b0` | 8×8 `3250b0` | 8×8 `3250b0` | 8×8 `3250b0` | ✗ `"Unknown blend ope` | 8×8 `3250b0` |
| fctl:y_offset (23; 11) | [#4907](witnesses/40-fctl-y_offset.png) 385B | ✗ `ExpectedError` | ✗ `ExpectedError` | ✗ `Format error decod` | ✗ `Sub frame is out-o` | ✗ `Sub frame is out-o` | 8×8 `3250b0` | ✗ `gdk` | 8×8 `3250b0` | 8×8 `3250b0` | 8×8 `3250b0` | 8×8 `3250b0` | 8×8 `3250b0` | 8×8 `3250b0` | 8×8 `3250b0` |

## Same acceptance, different first-frame pixels

Rules with the most `accept-different-pixels` inputs (smallest input per rule, not minimised; difference from raw RGBA dumps of the two largest hash groups).

| Rule (n) | Witness | Difference | Hash groups |
| --- | --- | --- | --- |
| order:plte (9) | [#2848](witnesses/pixel-01-order-plte.png) | rgb-only: 56 of 63 px (glycin+glycin-apng+image vs zune) | 9×7 3e2b2031: glycin+glycin-apng+image+pngcrate+pngcrate-apng+libpng+gdk-anim+gdk-forced-whole+gdk-forced-byte+spng+lodepng+stb+minipng; 9×7 6c5dd92f: zune |
| trailing-data (9) | [#4618](witnesses/pixel-02-trailing-data.png) | alpha-only: 16 of 64 px (glycin+glycin-apng+image vs minipng) | 8×8 a83050ec: glycin+glycin-apng+image+pngcrate+pngcrate-apng+libpng+gdk-anim+gdk-forced-whole+gdk-forced-byte+spng+lodepng+stb+zune; 8×8 7cadb2b6: minipng |
| order:unknown-ancillary (6) | [#2098](witnesses/pixel-03-order-unknown-ancillary.png) | alpha-only: 16 of 64 px (glycin+glycin-apng+image vs minipng) | 8×8 a83050ec: glycin+glycin-apng+image+pngcrate+pngcrate-apng+libpng+gdk-anim+gdk-forced-whole+gdk-forced-byte+spng+lodepng+stb+zune; 8×8 7cadb2b6: minipng |
| order:double-gama (6) | [#2308](witnesses/pixel-04-order-double-gama.png) | alpha-only: 16 of 64 px (glycin+glycin-apng+image vs minipng) | 8×8 a83050ec: glycin+glycin-apng+image+pngcrate+pngcrate-apng+libpng+gdk-anim+gdk-forced-whole+gdk-forced-byte+spng+lodepng+stb+zune; 8×8 7cadb2b6: minipng |
| order:srgb-and-iccp (6) | [#2378](witnesses/pixel-05-order-srgb-and-iccp.png) | alpha-only: 16 of 64 px (glycin+glycin-apng+image vs minipng) | 8×8 a83050ec: glycin+glycin-apng+image+pngcrate+pngcrate-apng+libpng+gdk-anim+gdk-forced-whole+gdk-forced-byte+spng+lodepng+stb+zune; 8×8 7cadb2b6: minipng |
| order:iccp-bad-method (6) | [#2448](witnesses/pixel-06-order-iccp-bad-method.png) | alpha-only: 16 of 64 px (glycin+glycin-apng+image vs minipng) | 8×8 a83050ec: glycin+glycin-apng+image+pngcrate+pngcrate-apng+libpng+gdk-anim+gdk-forced-whole+gdk-forced-byte+spng+lodepng+stb+zune; 8×8 7cadb2b6: minipng |
| order:iccp-bad-zlib (6) | [#2518](witnesses/pixel-07-order-iccp-bad-zlib.png) | alpha-only: 16 of 64 px (glycin+glycin-apng+image vs minipng) | 8×8 a83050ec: glycin+glycin-apng+image+pngcrate+pngcrate-apng+libpng+gdk-anim+gdk-forced-whole+gdk-forced-byte+spng+lodepng+stb+zune; 8×8 7cadb2b6: minipng |
| order:iccp-no-name (6) | [#2588](witnesses/pixel-08-order-iccp-no-name.png) | alpha-only: 16 of 64 px (glycin+glycin-apng+image vs minipng) | 8×8 a83050ec: glycin+glycin-apng+image+pngcrate+pngcrate-apng+libpng+gdk-anim+gdk-forced-whole+gdk-forced-byte+spng+lodepng+stb+zune; 8×8 7cadb2b6: minipng |
| order:gama-zero (6) | [#2658](witnesses/pixel-09-order-gama-zero.png) | alpha-only: 16 of 64 px (glycin+glycin-apng+image vs minipng) | 8×8 a83050ec: glycin+glycin-apng+image+pngcrate+pngcrate-apng+libpng+gdk-anim+gdk-forced-whole+gdk-forced-byte+spng+lodepng+stb+zune; 8×8 7cadb2b6: minipng |
| order:srgb-intent-9 (6) | [#2728](witnesses/pixel-10-order-srgb-intent-9.png) | alpha-only: 16 of 64 px (glycin+glycin-apng+image vs minipng) | 8×8 a83050ec: glycin+glycin-apng+image+pngcrate+pngcrate-apng+libpng+gdk-anim+gdk-forced-whole+gdk-forced-byte+spng+lodepng+stb+zune; 8×8 7cadb2b6: minipng |
| order:text-no-null (6) | [#2798](witnesses/pixel-11-order-text-no-null.png) | alpha-only: 16 of 64 px (glycin+glycin-apng+image vs minipng) | 8×8 a83050ec: glycin+glycin-apng+image+pngcrate+pngcrate-apng+libpng+gdk-anim+gdk-forced-whole+gdk-forced-byte+spng+lodepng+stb+zune; 8×8 7cadb2b6: minipng |
| order:plte-short (4) | [#3151](witnesses/pixel-12-order-plte-short.png) | rgb-and-alpha: 19 of 49 px (glycin+glycin-apng+image vs stb) | 7×7 0516c600: glycin+glycin-apng+image+pngcrate+pngcrate-apng+libpng+gdk-anim+gdk-forced-whole+gdk-forced-byte+spng+lodepng+zune+minipng; 7×7 5380acf3: stb |
| seed:trns (2) | [#32](witnesses/pixel-13-seed-trns.png) | alpha-only: 16 of 64 px (glycin+glycin-apng+image vs minipng) | 8×8 a83050ec: glycin+glycin-apng+image+pngcrate+pngcrate-apng+libpng+gdk-anim+gdk-forced-whole+gdk-forced-byte+spng+lodepng+stb+zune; 8×8 7cadb2b6: minipng |
| ihdr:color_type (2) | [#281](witnesses/pixel-14-ihdr-color_type.png) | rgb-only: 24 of 49 px (glycin+glycin-apng+image vs zune) | 7×7 7a6ad787: glycin+glycin-apng+image+pngcrate+pngcrate-apng+libpng+gdk-anim+gdk-forced-whole+gdk-forced-byte+spng+lodepng+stb+minipng; 7×7 e2b8992d: zune |
| order:iend-with-data (2) | [#1538](witnesses/pixel-15-order-iend-with-data.png) | alpha-only: 16 of 64 px (glycin+glycin-apng+image vs minipng) | 8×8 a83050ec: glycin+glycin-apng+image+pngcrate+pngcrate-apng+libpng+gdk-anim+gdk-forced-whole+gdk-forced-byte+spng+lodepng+stb+zune; 8×8 7cadb2b6: minipng |

## Animation disagreements

Inputs where accepting animation-capable runners agree on the first frame or not, but disagree on canvas, frame count or composited sequence, clustered by rule (changed relative to the seed). Witnesses are not minimised.

| Rule (n) | Witness | Kinds | Frames per runner |
| --- | --- | --- | --- |
| fctl:dispose_op (2) | [#7953](witnesses/anim-01-fctl-dispose_op.png) | sequence-pixels-differ 2 | glycin 3; glycin-apng 3; image 3; pngcrate-apng 3 |
| seed:apng (1) | [#69](witnesses/anim-02-seed-apng.png) | sequence-pixels-differ 1 | glycin 4; glycin-apng 4; image 4; pngcrate-apng 4 |

## Acceptance by rule

Accepted inputs per rule (crash/timeout inputs excluded).

| Rule | n | glycin | glycin-apng | image | pngcrate | pngcrate-apng | libpng | gdk-anim | gdk-forced-whole | gdk-forced-byte | spng | lodepng | stb | zune | minipng |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| actl:num_frames | 35 | 33 | 33 | 33 | 35 | 35 | 35 | 2 | 35 | 35 | 35 | 35 | 35 | 35 | 35 |
| actl:num_plays | 12 | 12 | 12 | 12 | 12 | 12 | 12 | 1 | 12 | 12 | 12 | 12 | 12 | 12 | 12 |
| apng:actl-after-idat | 12 | 11 | 11 | 11 | 12 | 12 | 12 | 1 | 12 | 12 | 12 | 12 | 12 | 12 | 12 |
| apng:actl-without-fctl | 58 | 0 | 0 | 0 | 58 | 0 | 58 | 28 | 58 | 58 | 58 | 58 | 58 | 58 | 42 |
| apng:add-single-frame | 58 | 45 | 45 | 45 | 58 | 58 | 58 | 28 | 58 | 58 | 58 | 58 | 58 | 58 | 42 |
| apng:drop-frame | 11 | 9 | 9 | 9 | 11 | 10 | 11 | 1 | 11 | 11 | 11 | 11 | 11 | 11 | 11 |
| apng:fdat-before-fctl | 11 | 9 | 9 | 9 | 11 | 10 | 11 | 1 | 11 | 11 | 11 | 11 | 11 | 11 | 11 |
| apng:fdat-empty | 11 | 9 | 9 | 9 | 11 | 10 | 11 | 1 | 11 | 11 | 11 | 11 | 11 | 11 | 11 |
| apng:fdat-truncated-zlib | 11 | 9 | 9 | 9 | 11 | 10 | 11 | 1 | 11 | 11 | 11 | 11 | 11 | 11 | 11 |
| apng:missing-actl | 12 | 12 | 12 | 12 | 12 | 12 | 12 | 12 | 12 | 12 | 12 | 12 | 12 | 12 | 12 |
| byte-flip | 281 | 34 | 34 | 34 | 36 | 34 | 35 | 24 | 34 | 34 | 46 | 17 | 184 | 179 | 139 |
| crc:ancillary | 59 | 47 | 47 | 47 | 59 | 48 | 59 | 27 | 59 | 59 | 59 | 47 | 59 | 59 | 59 |
| crc:critical | 229 | 66 | 66 | 66 | 67 | 67 | 69 | 58 | 69 | 69 | 135 | 40 | 229 | 229 | 177 |
| fctl:blend_op | 25 | 11 | 11 | 11 | 13 | 12 | 25 | 2 | 25 | 25 | 25 | 25 | 25 | 12 | 25 |
| fctl:delay_den | 25 | 23 | 23 | 23 | 25 | 25 | 25 | 2 | 25 | 25 | 25 | 25 | 25 | 25 | 25 |
| fctl:delay_num | 25 | 23 | 23 | 23 | 25 | 25 | 25 | 2 | 25 | 25 | 25 | 25 | 25 | 25 | 25 |
| fctl:dispose_op | 25 | 22 | 22 | 22 | 24 | 24 | 25 | 2 | 25 | 25 | 25 | 25 | 25 | 24 | 25 |
| fctl:height | 33 | 1 | 1 | 1 | 3 | 1 | 33 | 2 | 33 | 33 | 33 | 33 | 33 | 25 | 33 |
| fctl:seq | 35 | 0 | 0 | 0 | 3 | 0 | 35 | 2 | 35 | 35 | 35 | 35 | 35 | 35 | 35 |
| fctl:width | 35 | 0 | 0 | 0 | 3 | 0 | 35 | 2 | 35 | 35 | 35 | 35 | 35 | 3 | 35 |
| fctl:x_offset | 25 | 1 | 1 | 1 | 2 | 1 | 25 | 2 | 25 | 25 | 25 | 25 | 25 | 25 | 25 |
| fctl:y_offset | 25 | 2 | 2 | 2 | 2 | 2 | 25 | 2 | 25 | 25 | 25 | 25 | 25 | 25 | 25 |
| fdat:seq | 11 | 9 | 9 | 9 | 11 | 10 | 11 | 1 | 11 | 11 | 11 | 11 | 11 | 11 | 11 |
| filter-type | 309 | 28 | 28 | 28 | 28 | 28 | 28 | 28 | 28 | 28 | 28 | 28 | 28 | 28 | 0 |
| ihdr:bit_depth | 349 | 12 | 12 | 12 | 12 | 12 | 12 | 19 | 19 | 19 | 11 | 6 | 18 | 18 | 10 |
| ihdr:color_type | 348 | 18 | 18 | 18 | 18 | 18 | 18 | 25 | 25 | 25 | 18 | 20 | 17 | 28 | 24 |
| ihdr:compression | 348 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 348 | 0 | 0 | 0 |
| ihdr:filter_method | 347 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 347 | 0 | 268 | 0 |
| ihdr:height | 343 | 91 | 91 | 91 | 92 | 91 | 114 | 203 | 225 | 225 | 114 | 0 | 114 | 72 | 114 |
| ihdr:interlace | 345 | 6 | 6 | 6 | 6 | 6 | 6 | 9 | 9 | 9 | 6 | 4 | 6 | 6 | 4 |
| ihdr:width | 344 | 23 | 23 | 23 | 23 | 23 | 23 | 64 | 64 | 64 | 23 | 14 | 23 | 23 | 27 |
| length:IDAT | 321 | 0 | 0 | 0 | 0 | 0 | 0 | 42 | 42 | 42 | 191 | 0 | 0 | 0 | 0 |
| length:IEND | 309 | 306 | 306 | 306 | 309 | 309 | 309 | 176 | 209 | 209 | 309 | 0 | 309 | 0 | 108 |
| length:IHDR | 342 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| length:PLTE | 65 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| length:acTL | 35 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| length:cHRM | 9 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| length:eXIf | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| length:fcTL | 35 | 0 | 0 | 0 | 3 | 0 | 3 | 0 | 0 | 0 | 3 | 0 | 0 | 0 | 0 |
| length:fdAT | 32 | 28 | 28 | 28 | 32 | 31 | 32 | 0 | 0 | 0 | 32 | 0 | 0 | 0 | 0 |
| length:gAMA | 23 | 5 | 5 | 5 | 5 | 5 | 5 | 1 | 1 | 1 | 5 | 0 | 0 | 20 | 0 |
| length:iCCP | 9 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| length:iTXt | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 0 | 0 | 0 | 4 | 0 | 0 | 0 | 0 |
| length:leNy | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| length:pHYs | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| length:sRGB | 9 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| length:tEXt | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| length:tIME | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 0 | 0 | 0 | 4 | 0 | 0 | 0 | 0 |
| length:tRNS | 35 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 20 | 0 |
| length:zTXt | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 0 | 0 | 0 | 4 | 0 | 0 | 0 | 0 |
| order:ancillary-before-ihdr | 70 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| order:ancillary-between-idat | 70 | 67 | 67 | 67 | 68 | 68 | 68 | 57 | 68 | 68 | 68 | 70 | 70 | 70 | 54 |
| order:chunk-after-iend | 70 | 69 | 69 | 69 | 70 | 70 | 70 | 59 | 70 | 70 | 70 | 70 | 70 | 70 | 54 |
| order:double-gama | 70 | 69 | 69 | 69 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 54 |
| order:duplicate-ihdr | 70 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| order:duplicate-plte | 14 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 14 | 14 | 14 | 14 | 10 |
| order:empty-idat-inserted | 70 | 69 | 69 | 69 | 70 | 70 | 70 | 59 | 70 | 70 | 70 | 70 | 70 | 70 | 54 |
| order:empty-idat-only | 70 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| order:gama-after-idat | 70 | 69 | 69 | 69 | 70 | 70 | 70 | 59 | 70 | 70 | 70 | 70 | 70 | 70 | 54 |
| order:gama-zero | 70 | 69 | 69 | 69 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 54 |
| order:iccp-bad-method | 70 | 69 | 69 | 69 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 54 |
| order:iccp-bad-zlib | 70 | 69 | 69 | 69 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 54 |
| order:iccp-no-name | 70 | 69 | 69 | 69 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 54 |
| order:idat-after-iend | 70 | 69 | 69 | 69 | 70 | 70 | 70 | 59 | 70 | 70 | 70 | 70 | 70 | 70 | 54 |
| order:iend-with-data | 70 | 69 | 69 | 69 | 70 | 70 | 70 | 59 | 70 | 70 | 70 | 70 | 70 | 70 | 54 |
| order:ihdr-not-first | 70 | 0 | 0 | 0 | 0 | 0 | 11 | 11 | 11 | 11 | 0 | 0 | 0 | 0 | 0 |
| order:missing-idat | 70 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| order:missing-iend | 70 | 13 | 13 | 13 | 14 | 14 | 70 | 59 | 70 | 70 | 70 | 0 | 0 | 0 | 0 |
| order:missing-plte | 14 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 11 | 0 | 11 | 10 |
| order:phys-short | 70 | 69 | 69 | 69 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 0 | 70 | 70 | 54 |
| order:plte | 56 | 53 | 53 | 53 | 54 | 54 | 56 | 56 | 56 | 56 | 56 | 56 | 56 | 56 | 44 |
| order:plte-257 | 14 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| order:plte-after-idat | 14 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 11 | 0 | 11 | 10 |
| order:plte-empty | 14 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 11 | 0 | 14 | 10 |
| order:plte-not-multiple-of-3 | 14 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 14 | 0 | 0 | 0 |
| order:plte-short | 14 | 14 | 14 | 14 | 14 | 14 | 14 | 14 | 14 | 14 | 14 | 11 | 11 | 14 | 10 |
| order:srgb-and-iccp | 70 | 69 | 69 | 69 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 54 |
| order:srgb-intent-9 | 70 | 69 | 69 | 69 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 54 |
| order:text-no-null | 70 | 69 | 69 | 69 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 54 |
| order:trns-before-plte | 14 | 14 | 14 | 14 | 14 | 14 | 14 | 14 | 14 | 14 | 14 | 0 | 0 | 0 | 10 |
| order:trns-longer-than-plte | 14 | 14 | 14 | 14 | 14 | 14 | 14 | 14 | 14 | 14 | 14 | 0 | 0 | 12 | 8 |
| order:trns-with-alpha | 26 | 25 | 25 | 25 | 26 | 26 | 26 | 26 | 26 | 26 | 26 | 0 | 0 | 0 | 22 |
| order:trns-wrong-length | 56 | 55 | 55 | 55 | 56 | 56 | 56 | 56 | 56 | 56 | 56 | 0 | 0 | 0 | 44 |
| order:unknown-ancillary | 70 | 69 | 69 | 69 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 54 |
| order:unknown-critical | 70 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| order:ztxt-bad-zlib | 70 | 69 | 69 | 69 | 70 | 70 | 70 | 70 | 70 | 70 | 70 | 0 | 70 | 70 | 54 |
| row-padding-bits | 10 | 10 | 10 | 10 | 10 | 10 | 10 | 10 | 10 | 10 | 10 | 10 | 10 | 10 | 10 |
| seed:anc | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 |
| seed:apng | 12 | 11 | 11 | 11 | 12 | 12 | 12 | 1 | 12 | 12 | 12 | 12 | 12 | 12 | 12 |
| seed:big | 2 | 2 | 2 | 2 | 2 | 2 | 2 | 2 | 2 | 2 | 2 | 2 | 2 | 2 | 1 |
| seed:color | 6 | 6 | 6 | 6 | 6 | 6 | 6 | 6 | 6 | 6 | 6 | 6 | 6 | 6 | 6 |
| seed:ct0 | 10 | 10 | 10 | 10 | 10 | 10 | 10 | 10 | 10 | 10 | 10 | 10 | 10 | 10 | 5 |
| seed:ct2 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 2 |
| seed:ct3 | 8 | 8 | 8 | 8 | 8 | 8 | 8 | 8 | 8 | 8 | 8 | 8 | 8 | 8 | 4 |
| seed:ct4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 2 |
| seed:ct6 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 2 |
| seed:filter | 2 | 2 | 2 | 2 | 2 | 2 | 2 | 2 | 2 | 2 | 2 | 2 | 2 | 2 | 2 |
| seed:idat | 2 | 2 | 2 | 2 | 2 | 2 | 2 | 2 | 2 | 2 | 2 | 2 | 2 | 2 | 2 |
| seed:plte | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 |
| seed:trns | 7 | 7 | 7 | 7 | 7 | 7 | 7 | 7 | 7 | 7 | 7 | 7 | 7 | 7 | 7 |
| seed:zlib | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
| signature | 299 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| trailing-data | 295 | 293 | 293 | 293 | 295 | 295 | 295 | 272 | 295 | 295 | 295 | 295 | 295 | 295 | 219 |
| truncation | 297 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| type-bit:ancillary | 243 | 54 | 54 | 54 | 54 | 54 | 54 | 54 | 54 | 54 | 54 | 13 | 2 | 13 | 12 |
| type-bit:private | 243 | 84 | 84 | 84 | 94 | 84 | 94 | 34 | 44 | 44 | 94 | 44 | 44 | 44 | 44 |
| type-bit:reserved | 243 | 84 | 84 | 84 | 94 | 84 | 52 | 2 | 2 | 2 | 94 | 44 | 44 | 44 | 44 |
| type-bit:safe-to-copy | 243 | 84 | 84 | 84 | 94 | 84 | 94 | 34 | 44 | 44 | 94 | 44 | 44 | 44 | 44 |
| zlib:bad-adler32 | 70 | 69 | 69 | 69 | 70 | 70 | 0 | 59 | 70 | 70 | 0 | 0 | 70 | 70 | 54 |
| zlib:bad-cm | 70 | 0 | 0 | 0 | 0 | 0 | 0 | 59 | 70 | 70 | 0 | 0 | 0 | 0 | 0 |
| zlib:bad-zlib-header | 70 | 0 | 0 | 0 | 0 | 0 | 0 | 59 | 70 | 70 | 0 | 0 | 0 | 0 | 0 |
| zlib:extra-after-zlib | 70 | 69 | 69 | 69 | 70 | 70 | 70 | 59 | 70 | 70 | 70 | 0 | 70 | 70 | 54 |
| zlib:extra-row | 70 | 69 | 69 | 69 | 70 | 70 | 70 | 59 | 70 | 70 | 70 | 0 | 70 | 63 | 30 |
| zlib:level0 | 70 | 69 | 69 | 69 | 70 | 70 | 70 | 59 | 70 | 70 | 70 | 70 | 70 | 70 | 54 |
| zlib:level9 | 70 | 69 | 69 | 69 | 70 | 70 | 70 | 59 | 70 | 70 | 70 | 70 | 70 | 70 | 54 |
| zlib:missing-adler32 | 70 | 64 | 64 | 64 | 65 | 65 | 0 | 0 | 0 | 0 | 70 | 0 | 70 | 70 | 54 |
| zlib:one-byte-long | 70 | 69 | 69 | 69 | 70 | 70 | 70 | 59 | 70 | 70 | 70 | 0 | 70 | 70 | 32 |
| zlib:one-byte-short | 70 | 0 | 0 | 0 | 0 | 0 | 0 | 59 | 70 | 70 | 0 | 0 | 0 | 0 | 54 |
| zlib:raw-deflate | 70 | 0 | 0 | 0 | 0 | 0 | 0 | 59 | 70 | 70 | 0 | 0 | 0 | 0 | 0 |
| zlib:too-little-data | 70 | 0 | 0 | 0 | 0 | 0 | 0 | 59 | 70 | 70 | 0 | 0 | 0 | 0 | 54 |
| zlib:truncated-deflate | 70 | 1 | 1 | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | 0 | 21 |
| zlib:two-zlib-streams | 70 | 69 | 69 | 69 | 70 | 70 | 70 | 59 | 70 | 70 | 70 | 0 | 70 | 70 | 54 |
| zlib:zlib-fdict | 70 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 70 | 54 |

## Crashes, timeouts and anomalies

Crash and timeout inputs are excluded from this public view; they are retained and counted only in the local matrix.

## Run

`./run.sh --inputs 10000 --seed 731` in `/__modal/volumes/vo-Iyh3VLkOB7cEHrIp2RkTs7/projects/leniency/png` builds every runner under /tmp/claude-0 and regenerates this file; `python3 reproduce.py FILE` runs one file through every runner. Per-input JSONL and the manifest (provenance: seed, kind, rule, field, old/new value) live in `/tmp/claude-0/leniency-png`.
