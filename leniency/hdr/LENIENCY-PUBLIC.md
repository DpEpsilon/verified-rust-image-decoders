# HDR leniency matrix (public: synthetic inputs, crash/timeout inputs excluded)

Seed 731; 9855 inputs (synthetic only; no third-party files); 52 valid seeds; 370.8 s for the decoder pass.

Decoded quantity compared: **linear float RGB samples exactly as returned** (no tone mapping, no reorientation, no EXPOSURE/COLORCORR scaling by the harness; per-input hash = SHA-256 of little-endian f32 bit patterns, R,G,B per pixel). Tolerance rule: when accepting decoders have equal dimensions but different bits, samples are re-dumped and counted equal iff |a-b| <= 1e-6 * max(|a|,|b|) (about 8 f32 ulp; NaN matches NaN), so `accept-different-pixels` means a larger difference. `image`, `image-nonstrict` and `glycin` are the same implementation (image::HdrDecoder; glycin uses the non-strict constructor) and count as one vote; radiant, hdrldr and stb_image v2.30 are independent. hdrldr and radiant compute `m/255 * 2^(e-128)`; image and stb compute `m * 2^(e-136)` (the Radiance convention without the +0.5 rounding term); the first form is 256/255 larger.

Classification | Count
--- | ---:
all-accept-identical-pixels | 1
accept-different-pixels | 2892
accept-vs-reject | 3039
all-reject | 3923
crash-or-timeout (excluded from this file) | 145

Convention-adjusted view (decoders whose samples differ from another decoder by exactly one uniform factor of 256/255 are treated as equal; shows what remains once that known normalisation difference is set aside):

Classification (adjusted) | Count
--- | ---:
all-accept-identical-pixels | 2256
accept-different-pixels | 637
accept-vs-reject | 3039
all-reject | 3923

## Per decoder

| Decoder | ok | error | n/a | crash | timeout | only acceptor | only rejector |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| image | 3910 | 5945 | 0 | - | - | 0 | 205 |
| image-nonstrict | 4835 | 5020 | 0 | - | - | 0 | 0 |
| glycin | 4835 | 5020 | 0 | - | - | 0 | 0 |
| radiant | 4602 | 5253 | 0 | - | - | 11 | 55 |
| hdrldr | 4613 | 5242 | 0 | - | - | 46 | 0 |
| stb | 4038 | 5817 | 0 | - | - | 490 | 800 |

Unavailable candidates: `gdk-pixbuf Radiance module` (not installed (gdk-pixbuf-query-loaders lists none)); `Radiance reference tools (ra_*)` (not fetched); `DirectXTex` (does not read Radiance HDR).

only acceptor: the decoder accepted while every other decoder rejected. only rejector: it rejected while every other decoder accepted. Both exclude inputs with a crash or timeout.

## Provenance families (seed variant family, all mutations of its seeds)

| Family | inputs | all-accept-identical | accept-different-pixels | accept-vs-reject | all-reject |
| --- | ---: | ---: | ---: | ---: | ---: |
| exponent-extremes | 381 | 0 | 180 | 109 | 92 |
| flat-scanlines | 1265 | 1 | 569 | 381 | 314 |
| header-lines | 4762 | 0 | 1426 | 1993 | 1343 |
| orientation | 1911 | 0 | 26 | 41 | 1844 |
| rle-new | 1152 | 0 | 530 | 361 | 261 |
| rle-old | 384 | 0 | 161 | 154 | 69 |

## Unmutated valid seeds: acceptance per decoder (ok / seeds in family)

| Family | seeds | image | image-nonstrict | glycin | radiant | hdrldr | stb |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| exponent-extremes | 2 | 2 | 2 | 2 | 2 | 2 | 2 |
| flat-scanlines | 7 | 7 | 7 | 7 | 7 | 7 | 7 |
| header-lines | 25 | 20 | 22 | 22 | 22 | 20 | 17 |
| orientation | 10 | 0 | 0 | 0 | 0 | 0 | 0 |
| rle-new | 6 | 6 | 6 | 6 | 6 | 6 | 6 |
| rle-old | 2 | 2 | 2 | 2 | 2 | 2 | 2 |

## Representative matrix (one row per rule; modal outcome among inputs that changed the seed relation)

| Rule (inputs; patterns) | Witness | image | image-nonstrict | glycin | radiant | hdrldr | stb |
| --- | --- | --- | --- | --- | --- | --- | --- |
| magic (449; 54) | [#85](witnesses/01-magic.hdr) | reject `Format error decoding Hdr` | 8x4 `d19d3168` | 8x4 `d19d3168` | reject `the file is not a Radiance` | reject `FileFormat` | reject `unknown image type` |
| truncation (336; 78) | [#2316](witnesses/02-truncation.hdr) | reject `failed to fill whole buffe` | reject `failed to fill whole buffe` | reject `ExpectedError` | reject `failed to fill whole buffe` | reject `Io` | 5x3 `10b2a668` |
| rle-width-field (271; 39) | [#99](witnesses/03-rle-width-field.hdr) | 8x4 `d19d3168` | 8x4 `d19d3168` | 8x4 `d19d3168` | 8x4 `2592e096` | 8x4 `2592e096` | reject `invalid decoded scanline l` |
| dims-line (259; 99) | [#1108](witnesses/04-dims-line.hdr) | 5x3 `4235d79b` | 5x3 `4235d79b` | 5x3 `4235d79b` | reject `the image header is invali` | 5x3 `8207b5d9` | 5x3 `4235d79b` |
| rle-marker-bytes (238; 126; image vs radiant: sample values (max rel err 0.00392); image vs hdrldr: uniform scale x1.003922; image vs stb: sample values (max rel err 1)) | [#397](witnesses/05-rle-marker-bytes.hdr) | 8x4 `3560b6d2` | 8x4 `3560b6d2` | 8x4 `3560b6d2` | 8x4 `1b73b9a2` | 8x4 `97a99abe` | 8x4 `a202151a` |
| byte-flip-header (209; 55) | [#706](witnesses/06-byte-flip-header.hdr) | reject `The image format `3"-bit_r` | reject `The image format `3"-bit_r` | reject `ExpectedError` | 5x3 `212bfe7a` | 5x3 `212bfe7a` | reject `unsupported format` |
| format-line (168; 25) | [#495](witnesses/07-format-line.hdr) | reject `The image format `rgbe` is` | reject `The image format `rgbe` is` | reject `ExpectedError` | 8x4 `2592e096` | 8x4 `2592e096` | 8x4 `d19d3168` |
| colorcorr-line (156; 22) | [#151](witnesses/08-colorcorr-line.hdr) | reject `Format error decoding Hdr` | 8x4 `d19d3168` | 8x4 `d19d3168` | 8x4 `2592e096` | 8x4 `2592e096` | 8x4 `d19d3168` |
| byte-value (118; 33) | [#391](witnesses/09-byte-value.hdr) | 8x4 `d19d3168` | 8x4 `d19d3168` | 8x4 `d19d3168` | 8x4 `2592e096` | 8x4 `2592e096` | reject `invalid decoded scanline l` |
| exponent-byte-flat (98; 56; image vs radiant: sample values (max rel err 0.00391); image vs hdrldr: uniform scale x1.003922) | [#142](witnesses/10-exponent-byte-flat.hdr) | 5x3 `43d4a618` | 5x3 `43d4a618` | 5x3 `43d4a618` | 5x3 `e29a51b9` | 5x3 `01c57d4a` | 5x3 `43d4a618` |
| exposure-line (96; 22) | [#235](witnesses/11-exposure-line.hdr) | reject `Format error decoding Hdr` | 8x4 `d19d3168` | 8x4 `d19d3168` | 8x4 `2592e096` | 8x4 `2592e096` | 8x4 `d19d3168` |
| flat-first-pixel-rle-lookalike (62; 34) | [#100](witnesses/12-flat-first-pixel-rle-lookalike.hdr) | reject `Format error decoding Hdr` | reject `Format error decoding Hdr` | reject `ExpectedError` | 5x3 `60492ec7` | 5x3 `60492ec7` | 5x3 `70cd509a` |
| header-end (42; 19) | [#761](witnesses/13-header-end.hdr) | reject `Format error decoding Hdr` | reject `Format error decoding Hdr` | reject `ExpectedError` | reject `the image header is invali` | 8x4 `3782a00f` | reject `unsupported format` |
| flat-first-pixel-old-rle-marker (36; 18) | [#186](witnesses/14-flat-first-pixel-old-rle-marker.hdr) | reject `Format error decoding Hdr` | reject `Format error decoding Hdr` | reject `ExpectedError` | 5x3 `a39ffd72` | reject `Rle` | 5x3 `f4d500a7` |
| rle-insert-byte (32; 8) | [#131](witnesses/15-rle-insert-byte.hdr) | 8x4 `d19d3168` | 8x4 `d19d3168` | 8x4 `d19d3168` | 8x4 `2592e096` | 8x4 `2592e096` | reject `corrupt` |
| extra-header-line (32; 2) | [#149](witnesses/16-extra-header-line.hdr) | 8x4 `d19d3168` | 8x4 `d19d3168` | 8x4 `d19d3168` | 8x4 `2592e096` | 8x4 `2592e096` | reject `unsupported format` |
| old-rle-count (30; 29) | [#8382](witnesses/17-old-rle-count.hdr) | reject `failed to fill whole buffe` | reject `failed to fill whole buffe` | reject `ExpectedError` | reject `unexpected end of file` | reject `Io` | 16x2 `bd5721cd` |
| seed:header-lines (24; 10; image vs radiant: uniform scale x1.003922) | [#27](witnesses/18-seed-header-lines.hdr) | 8x4 `d19d3168` | 8x4 `d19d3168` | 8x4 `d19d3168` | 8x4 `2592e096` | 8x4 `2592e096` | 8x4 `d19d3168` |
| byte-flip-data (11; 8) | [#969](witnesses/19-byte-flip-data.hdr) | 8x4 `d19d3168` | 8x4 `d19d3168` | 8x4 `d19d3168` | 8x4 `2592e096` | 8x4 `2592e096` | reject `invalid decoded scanline l` |
| seed:flat-scanlines (7; 7; image vs radiant: uniform scale x1.003922) | [#0](witnesses/20-seed-flat-scanlines.hdr) | 1x1 `a2c33d48` | 1x1 `a2c33d48` | 1x1 `a2c33d48` | 1x1 `729f30ba` | 1x1 `729f30ba` | 1x1 `a2c33d48` |
| seed:rle-new (6; 6; image vs radiant: uniform scale x1.003922) | [#7](witnesses/21-seed-rle-new.hdr) | 8x4 `d19d3168` | 8x4 `d19d3168` | 8x4 `d19d3168` | 8x4 `2592e096` | 8x4 `2592e096` | 8x4 `d19d3168` |
| many-header-lines (6; 2) | [#358](witnesses/22-many-header-lines.hdr) | 5x3 `4235d79b` | 5x3 `4235d79b` | 5x3 `4235d79b` | 5x3 `212bfe7a` | 5x3 `212bfe7a` | reject `unsupported format` |
| old-rle-first-pixel-marker (4; 4) | [#221](witnesses/23-old-rle-first-pixel-marker.hdr) | reject `Format error decoding Hdr` | reject `Format error decoding Hdr` | reject `ExpectedError` | 16x2 `e6de595f` | reject `Rle` | 16x2 `02873420` |
| seed:rle-old (2; 2; image vs radiant: uniform scale x1.003922; image vs stb: sample values (max rel err 1)) | [#13](witnesses/24-seed-rle-old.hdr) | 16x2 `94507a92` | 16x2 `94507a92` | 16x2 `94507a92` | 16x2 `958c47de` | 16x2 `958c47de` | 16x2 `c5a5cb19` |
| seed:exponent-extremes (2; 2; image vs radiant: sample values (max rel err 0.00391); image vs hdrldr: uniform scale x1.003922) | [#25](witnesses/25-seed-exponent-extremes.hdr) | 8x4 `48308992` | 8x4 `48308992` | 8x4 `48308992` | 8x4 `359fe533` | 8x4 `c2b7ebe5` | 8x4 `48308992` |

## Same acceptance, different pixels

| Rule | Witness | Difference | Outcomes |
| --- | --- | --- | --- |
| rle-marker-bytes (175) | [#320](witnesses/pixel-01.hdr) | image vs radiant: uniform scale x1.003922; image vs stb: sample values (max rel err 1) | image, image-nonstrict, glycin: accept 16x16 `72c1497d`; radiant, hdrldr: accept 16x16 `479f837a`; stb: accept 16x16 `33ef2cea` |
| exponent-byte-flat (86) | [#142](witnesses/pixel-02.hdr) | image vs radiant: sample values (max rel err 0.00391); image vs hdrldr: uniform scale x1.003922 | image, image-nonstrict, glycin, stb: accept 5x3 `43d4a618`; radiant: accept 5x3 `e29a51b9`; hdrldr: accept 5x3 `01c57d4a` |
| rle-width-field (39) | [#705](witnesses/pixel-03.hdr) | image vs radiant: sample values (max rel err 0.00392); image vs hdrldr: uniform scale x1.003922; image vs stb: sample values (max rel err 1) | image, image-nonstrict, glycin: accept 8x4 `0d657628`; radiant: accept 8x4 `0dbf9502`; hdrldr: accept 8x4 `c4f169cf`; stb: accept 8x4 `af46f51b` |
| dims-line (26) | [#281](witnesses/pixel-04.hdr) | image vs radiant: uniform scale x1.003922 | image, image-nonstrict, glycin, stb: accept 4x3 `0f539f75`; radiant, hdrldr: accept 4x3 `9b2d804d` |
| seed:header-lines (16) | [#27](witnesses/pixel-05.hdr) | image vs radiant: uniform scale x1.003922 | image, image-nonstrict, glycin, stb: accept 8x4 `d19d3168`; radiant, hdrldr: accept 8x4 `2592e096` |
| byte-value (9) | [#3498](witnesses/pixel-06.hdr) | image vs radiant: sample values (max rel err 0.00392); image vs hdrldr: uniform scale x1.003922; image vs stb: sample values (max rel err 1) | image, image-nonstrict, glycin: accept 20x3 `0d8b89a2`; radiant: accept 20x3 `bed0bac5`; hdrldr: accept 20x3 `54910881`; stb: accept 20x3 `2f343e8b` |
| seed:flat-scanlines (7) | [#0](witnesses/pixel-07.hdr) | image vs radiant: uniform scale x1.003922 | image, image-nonstrict, glycin, stb: accept 1x1 `a2c33d48`; radiant, hdrldr: accept 1x1 `729f30ba` |
| seed:rle-new (6) | [#7](witnesses/pixel-08.hdr) | image vs radiant: uniform scale x1.003922 | image, image-nonstrict, glycin, stb: accept 8x4 `d19d3168`; radiant, hdrldr: accept 8x4 `2592e096` |
| old-rle-count (5) | [#1782](witnesses/pixel-09.hdr) | image vs radiant: sample values (max rel err 0.00392); image vs hdrldr: uniform scale x1.003922; image vs stb: sample values (max rel err 1) | image, image-nonstrict, glycin: accept 20x3 `0d8b89a2`; radiant: accept 20x3 `bed0bac5`; hdrldr: accept 20x3 `54910881`; stb: accept 20x3 `2f343e8b` |
| format-line (4) | [#4152](witnesses/pixel-10.hdr) | image vs radiant: uniform scale x1.003922 | image, image-nonstrict, glycin, stb: accept 5x3 `4235d79b`; radiant, hdrldr: accept 5x3 `212bfe7a` |

## Ten most significant leniency findings (ranked: pixel disagreement, then accept-vs-reject, by input count; rows whose only difference is the convention-adjusted equivalence are excluded)

- `rle-marker-bytes` (238 inputs, accept-different-pixels): image, image-nonstrict, glycin: accept 8x4 `3560b6d2`; radiant: accept 8x4 `1b73b9a2`; hdrldr: accept 8x4 `97a99abe`; stb: accept 8x4 `a202151a`. Witness witnesses/05-rle-marker-bytes.hdr.
- `exponent-byte-flat` (98 inputs, accept-different-pixels): image, image-nonstrict, glycin, stb: accept 5x3 `43d4a618`; radiant: accept 5x3 `e29a51b9`; hdrldr: accept 5x3 `01c57d4a`. Witness witnesses/10-exponent-byte-flat.hdr.
- `seed:rle-old` (2 inputs, accept-different-pixels): image, image-nonstrict, glycin: accept 16x2 `94507a92`; radiant, hdrldr: accept 16x2 `958c47de`; stb: accept 16x2 `c5a5cb19`. Witness witnesses/24-seed-rle-old.hdr.
- `seed:exponent-extremes` (2 inputs, accept-different-pixels): image, image-nonstrict, glycin, stb: accept 8x4 `48308992`; radiant: accept 8x4 `359fe533`; hdrldr: accept 8x4 `c2b7ebe5`. Witness witnesses/25-seed-exponent-extremes.hdr.
- `magic` (449 inputs, accept-vs-reject): image: reject (Format error decoding Hdr); image-nonstrict, glycin: accept 8x4 `d19d3168`; radiant: reject (the file is not a Radiance HDR); hdrldr: reject (FileFormat); stb: reject (unknown image type). Witness witnesses/01-magic.hdr.
- `truncation` (336 inputs, accept-vs-reject): image, image-nonstrict, radiant: reject (failed to fill whole buffer); glycin: reject (ExpectedError); hdrldr: reject (Io); stb: accept 5x3 `10b2a668`. Witness witnesses/02-truncation.hdr.
- `rle-width-field` (271 inputs, accept-vs-reject): image, image-nonstrict, glycin: accept 8x4 `d19d3168`; radiant, hdrldr: accept 8x4 `2592e096`; stb: reject (invalid decoded scanline lengt). Witness witnesses/03-rle-width-field.hdr.
- `dims-line` (259 inputs, accept-vs-reject): image, image-nonstrict, glycin, stb: accept 5x3 `4235d79b`; radiant: reject (the image header is invalid); hdrldr: accept 5x3 `8207b5d9`. Witness witnesses/04-dims-line.hdr.
- `byte-flip-header` (209 inputs, accept-vs-reject): image, image-nonstrict: reject (The image format `3"-bit_rle_r); glycin: reject (ExpectedError); radiant, hdrldr: accept 5x3 `212bfe7a`; stb: reject (unsupported format). Witness witnesses/06-byte-flip-header.hdr.
- `format-line` (168 inputs, accept-vs-reject): image, image-nonstrict: reject (The image format `rgbe` is not); glycin: reject (ExpectedError); radiant, hdrldr: accept 8x4 `2592e096`; stb: accept 8x4 `d19d3168`. Witness witnesses/07-format-line.hdr.

Seeds: flat (uncompressed) scanlines, new-style adaptive RLE (runs and literals), old-style RLE, all eight orientation lines, exponent extremes (e=0/1/255), header variants (comments, FORMAT, EXPOSURE, COLORCORR, PRIMARIES, CRLF, missing magic). Mutations: dimensions-line text, magic, FORMAT/EXPOSURE/COLORCORR/extra attribute lines, header termination, RLE scanline header and control bytes, old-RLE counts, flat first-pixel RLE look-alikes, truncation, trailing data, byte flips.

## Run

`./run.sh --inputs 10000 --seed 731 --jobs 4` from `/__modal/volumes/vo-Iyh3VLkOB7cEHrIp2RkTs7/projects/leniency/hdr`. `python3 reproduce.py FILE` runs one file through every decoder. Per-input results and manifest: `/tmp/claude-0/leniency-hdr`.
