# DDS leniency matrix (public: synthetic inputs, crash/timeout inputs excluded)

Seed 731; 9986 inputs (synthetic only; no third-party files); 84 valid seeds; 278.6 s for the decoder pass.

Decoded surface compared: **first mip level, array layer 0, volume slice 0, straight top-first RGBA8** (SHA-256 of the RGBA bytes). `image` and `glycin` are the same implementation (image::DdsDecoder) and count as one vote. `image` supports only DXT1/3/5 (and DX10 BC1-3) and returns RGB for DXT1. `ddsfile-*` columns are our glue around the ddsfile container parser: BC4 is shown as (r,0,0,255), BC5 as (r,g,0,255), alpha-only as (0,0,0,a), luminance as (l,l,l,a), bitmask channels scaled to 8 bits by truncating division; texpresso covers BC1-5 only (others show `UnsupportedByCandidate`, which is a coverage gap of that column, not a rejection judgement). The mip/cube/array/volume handling of each decoder is stored per input in the results JSONL (`mips`, `layers`, `cube`, `depth`, `policy`).

Classification | Count
--- | ---:
all-accept-identical-pixels | 76
accept-different-pixels | 1580
accept-vs-reject | 5521
all-reject | 2809
crash-or-timeout (excluded from this file) | 14

Convention-adjusted view (decoders whose RGBA8 outputs differ by at most 1 on every channel (BCn endpoint-expansion / rounding convention) are treated as equal; shows what remains once +-1 rounding is set aside):

Classification (adjusted) | Count
--- | ---:
all-accept-identical-pixels | 1578
accept-different-pixels | 78
accept-vs-reject | 5521
all-reject | 2809

## Per decoder

| Decoder | ok | error | n/a | crash | timeout | only acceptor | only rejector |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| image | 1829 | 8157 | 0 | - | - | 0 | 0 |
| glycin | 1829 | 8157 | 0 | - | - | 0 | 0 |
| image-dds | 5606 | 4380 | 0 | - | - | 184 | 6 |
| ddsfile-bcdec | 6826 | 3160 | 0 | - | - | 1552 | 0 |
| ddsfile-texpresso | 3662 | 2692 | 3632 | - | - | 0 | 0 |

Unavailable candidates: `gdk-pixbuf DDS module` (not installed (gdk-pixbuf-query-loaders lists none)); `DirectXTex` (C++ library not fetched; would need a CMake build); `stb_image` (has no DDS support).

only acceptor: the decoder accepted while every other decoder rejected. only rejector: it rejected while every other decoder accepted. Both exclude inputs with a crash or timeout.

## Provenance families (seed variant family, all mutations of its seeds)

| Family | inputs | all-accept-identical | accept-different-pixels | accept-vs-reject | all-reject |
| --- | ---: | ---: | ---: | ---: | ---: |
| array | 238 | 0 | 68 | 106 | 64 |
| cube | 473 | 0 | 299 | 57 | 117 |
| dx10 | 2021 | 0 | 213 | 1257 | 551 |
| dx10-random-blocks | 119 | 0 | 0 | 87 | 32 |
| legacy-bc45 | 476 | 0 | 0 | 363 | 113 |
| legacy-bcn | 2021 | 76 | 543 | 902 | 500 |
| legacy-random-blocks | 594 | 0 | 235 | 221 | 138 |
| mipmaps | 476 | 0 | 149 | 211 | 116 |
| uncompressed-bitmask | 3213 | 0 | 0 | 2146 | 1067 |
| volume | 355 | 0 | 73 | 171 | 111 |

## Unmutated valid seeds: acceptance per decoder (ok / seeds in family)

| Family | seeds | image | glycin | image-dds | ddsfile-bcdec | ddsfile-texpresso |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| array | 2 | 1 | 1 | 2 | 2 | 1 (1 n/a) |
| cube | 4 | 4 | 4 | 4 | 4 | 4 |
| dx10 | 17 | 3 | 3 | 15 | 17 | 10 (7 n/a) |
| dx10-random-blocks | 1 | 0 | 0 | 1 | 1 | 0 (1 n/a) |
| legacy-bc45 | 4 | 0 | 0 | 4 | 4 | 4 |
| legacy-bcn | 17 | 9 | 9 | 17 | 16 | 16 |
| legacy-random-blocks | 5 | 3 | 3 | 5 | 5 | 5 |
| mipmaps | 4 | 2 | 2 | 4 | 4 | 2 (2 n/a) |
| uncompressed-bitmask | 27 | 0 | 0 | 11 | 25 | 0 (25 n/a) |
| volume | 3 | 1 | 1 | 3 | 2 | 1 (2 n/a) |

## Representative matrix (one row per rule; modal outcome among inputs that changed the seed relation)

| Rule (inputs; patterns) | Witness | image | glycin | image-dds | ddsfile-bcdec | ddsfile-texpresso |
| --- | --- | --- | --- | --- | --- | --- |
| pitch_or_linear_size (163; 47) | [#93](witnesses/01-pitch_or_linear_size.dds) | reject `The Image's dimensions are` | reject `ExpectedError` | 7x5 `ecdce5f8` | reject `Truncated` | reject `Truncated` |
| flags-bit (92; 35) | [#298](witnesses/02-flags-bit.dds) | reject `Format error decoding Dds` | reject `ExpectedError` | 8x8 `f1099aed` | 8x8 `f1099aed` | 8x8 `02310379` |
| flags (90; 30) | [#502](witnesses/03-flags.dds) | reject `Format error decoding Dds` | reject `ExpectedError` | 8x8 `2c893cd4` | 8x8 `2c893cd4` | 8x8 `2c893cd4` |
| pf_b_mask (47; 46) | [#954](witnesses/04-pf_b_mask.dds) | reject `The image format `DDS` is ` | reject `ExpectedError` | reject `Surface` | 8x8 `8e99a8ee` | n/a |
| pf_g_mask (47; 46) | [#4482](witnesses/05-pf_g_mask.dds) | reject `The image format `DDS` is ` | reject `ExpectedError` | reject `Surface` | 8x8 `05d3c6ca` | n/a |
| pf_r_mask (44; 44) | [#445](witnesses/06-pf_r_mask.dds) | reject `The image format `DDS` is ` | reject `ExpectedError` | reject `Surface` | 5x3 `b981666e` | n/a |
| pf_a_mask (38; 38) | [#619](witnesses/07-pf_a_mask.dds) | reject `The image format `DDS` is ` | reject `ExpectedError` | reject `Surface` | 5x3 `85aa482f` | n/a |
| seed:uncompressed-bitmask (25; 23) | [#34](witnesses/08-seed-uncompressed-bitmask.dds) | reject `The image format `DDS` is ` | reject `ExpectedError` | reject `Surface` | 8x8 `2c893cd4` | n/a |
| dx10_misc_flag-bit (21; 6) | [#151](witnesses/09-dx10_misc_flag-bit.dds) | reject `Format error decoding Dds` | reject `ExpectedError` | 8x8 `dc1bccb4` | 8x8 `dc1bccb4` | 8x8 `ef7a753e` |
| width (20; 20) | [#429](witnesses/10-width.dds) | reject `The Image's dimensions are` | reject `ExpectedError` | 4x5 `4d84201d` | 4x5 `4d84201d` | 4x5 `4d84201d` |
| height (20; 20) | [#580](witnesses/11-height.dds) | reject `The Image's dimensions are` | reject `ExpectedError` | 8x1 `dc335345` | 8x1 `dc335345` | 8x1 `d8651ca4` |
| dx10_misc_flag (18; 6) | [#576](witnesses/12-dx10_misc_flag.dds) | reject `Format error decoding Dds` | reject `ExpectedError` | 8x8 `427deded` | 8x8 `427deded` | 8x8 `0219bbe7` |
| seed:dx10 (17; 17; rgb-only(max 1)) | [#44](witnesses/13-seed-dx10.dds) | 8x8 `1db92e61` | 8x8 `1db92e61` | 8x8 `cfc77059` | 8x8 `cfc77059` | 8x8 `2f3fc90e` |
| seed:legacy-bcn (16; 16; rgb-only(max 1)) | [#0](witnesses/14-seed-legacy-bcn.dds) | 8x8 `8e0e05cd` | 8x8 `8e0e05cd` | 8x8 `550d1321` | 8x8 `550d1321` | 8x8 `93515b13` |
| byte-flip-header (14; 14) | [#358](witnesses/15-byte-flip-header.dds) | reject `The image format `DDS` is ` | reject `ExpectedError` | reject `Surface` | 8x8 `8a4babfd` | n/a |
| mip_count (11; 7) | [#398](witnesses/16-mip_count.dds) | 16x16 `92ae9785` | 16x16 `92ae9785` | reject `Decode` | reject `DdsfileLayout` | reject `DdsfileLayout` |
| seed:legacy-random-blocks (5; 5; rgb(max 1)+alpha(max 255)) | [#74](witnesses/17-seed-legacy-random-blocks.dds) | 8x8 `2518d062` | 8x8 `2518d062` | 8x8 `5fb5e337` | 8x8 `5fb5e337` | 8x8 `158def82` |
| dx10_dxgi_format (5; 5) | [#1571](witnesses/18-dx10_dxgi_format.dds) | reject `The decoder for Dds does n` | reject `ExpectedError` | 8x8 `ac525466` | reject `UnsupportedDxgi` | reject `UnsupportedDxgi` |
| seed:legacy-bc45 (4; 4) | [#14](witnesses/19-seed-legacy-bc45.dds) | reject `The decoder for Dds does n` | reject `ExpectedError` | 8x8 `2caea0bf` | 8x8 `a88f1099` | 8x8 `2caea0bf` |
| seed:mipmaps (4; 4; rgb-only(max 1)) | [#61](witnesses/20-seed-mipmaps.dds) | 16x16 `e7ff2122` | 16x16 `e7ff2122` | 16x16 `936d318a` | 16x16 `936d318a` | 16x16 `e5af22ea` |
| seed:cube (4; 4; rgb-only(max 1)) | [#65](witnesses/21-seed-cube.dds) | 8x8 `0c05bb9d` | 8x8 `0c05bb9d` | 8x8 `19f7a644` | 8x8 `19f7a644` | 8x8 `4bf3cbc6` |
| depth (4; 1) | [#995](witnesses/22-depth.dds) | 8x8 `7504f71f` | 8x8 `7504f71f` | reject `Decode` | 8x8 `3e5994c0` | 8x8 `30a97097` |
| seed:volume (3; 3) | [#69](witnesses/23-seed-volume.dds) | reject `The image format `DDS` is ` | reject `ExpectedError` | 4x4 `1aae7d58` | 4x4 `1aae7d58` | n/a |
| dx10_array_size (3; 3) | [#997](witnesses/24-dx10_array_size.dds) | reject `The decoder for Dds does n` | reject `ExpectedError` | 8x8 `3a40acf7` | reject `DdsfileLayout` | n/a |
| pf_flags-bit (3; 3) | [#1876](witnesses/25-pf_flags-bit.dds) | reject `The image format `DDS` is ` | reject `ExpectedError` | reject `Surface` | 8x8 `3151ec15` | n/a |
| truncation (3; 2) | [#4767](witnesses/26-truncation.dds) | reject `The image format `DDS` is ` | reject `ExpectedError` | 8x8 `1de75bfe` | reject `DdsfileLayout` | n/a |
| seed:array (2; 2; rgb-only(max 1)) | [#72](witnesses/27-seed-array.dds) | 8x8 `a7f7546a` | 8x8 `a7f7546a` | 8x8 `427deded` | 8x8 `427deded` | 8x8 `0219bbe7` |
| trailing-bytes (2; 1) | [#3682](witnesses/28-trailing-bytes.dds) | reject `The image format `DDS` is ` | reject `ExpectedError` | 4x4 `e16f3a1b` | 4x4 `e16f3a1b` | n/a |
| dx10_resource_dimension (2; 2) | [#6960](witnesses/29-dx10_resource_dimension.dds) | reject `Format error decoding Dds` | reject `ExpectedError` | 8x8 `427deded` | 8x8 `427deded` | 8x8 `0219bbe7` |
| seed:dx10-random-blocks (1; 1) | [#79](witnesses/30-seed-dx10-random-blocks.dds) | reject `The decoder for Dds does n` | reject `ExpectedError` | 8x8 `f336ce0b` | 8x8 `f336ce0b` | n/a |
| pf_bit_count (1; 1) | [#5825](witnesses/31-pf_bit_count.dds) | reject `The image format `DDS` is ` | reject `ExpectedError` | reject `Surface` | 5x3 `485baeec` | n/a |
| pf_flags (1; 1) | [#6783](witnesses/32-pf_flags.dds) | reject `The image format `DDS` is ` | reject `ExpectedError` | reject `Surface` | 8x8 `c8c9e3d5` | n/a |
| byte-flip-payload (1; 1; rgb-only(max 1)) | [#7474](witnesses/33-byte-flip-payload.dds) | 8x8 `67450faf` | 8x8 `67450faf` | 8x8 `5a1c3d22` | 8x8 `5a1c3d22` | 8x8 `5a1c3d22` |

## Same acceptance, different pixels

| Rule | Witness | Difference | Outcomes |
| --- | --- | --- | --- |
| seed:legacy-bcn (7) | [#0](witnesses/pixel-01.dds) | rgb-only(max 1) | image, glycin: accept 8x8 `8e0e05cd`; image-dds, ddsfile-bcdec: accept 8x8 `550d1321`; ddsfile-texpresso: accept 8x8 `93515b13` |
| seed:cube (4) | [#65](witnesses/pixel-02.dds) | rgb-only(max 1) | image, glycin: accept 8x8 `0c05bb9d`; image-dds, ddsfile-bcdec: accept 8x8 `19f7a644`; ddsfile-texpresso: accept 8x8 `4bf3cbc6` |
| seed:dx10 (3) | [#44](witnesses/pixel-03.dds) | rgb-only(max 1) | image, glycin: accept 8x8 `1db92e61`; image-dds, ddsfile-bcdec: accept 8x8 `cfc77059`; ddsfile-texpresso: accept 8x8 `2f3fc90e` |
| seed:legacy-random-blocks (3) | [#74](witnesses/pixel-04.dds) | rgb(max 1)+alpha(max 255) | image, glycin: accept 8x8 `2518d062`; image-dds, ddsfile-bcdec: accept 8x8 `5fb5e337`; ddsfile-texpresso: accept 8x8 `158def82` |
| seed:mipmaps (2) | [#61](witnesses/pixel-05.dds) | rgb-only(max 1) | image, glycin: accept 16x16 `e7ff2122`; image-dds, ddsfile-bcdec: accept 16x16 `936d318a`; ddsfile-texpresso: accept 16x16 `e5af22ea` |
| seed:volume (1) | [#71](witnesses/pixel-06.dds) | rgb-only(max 1) | image, glycin: accept 8x8 `7504f71f`; image-dds, ddsfile-bcdec: accept 8x8 `3e5994c0`; ddsfile-texpresso: accept 8x8 `30a97097` |
| seed:array (1) | [#72](witnesses/pixel-07.dds) | rgb-only(max 1) | image, glycin: accept 8x8 `a7f7546a`; image-dds, ddsfile-bcdec: accept 8x8 `427deded`; ddsfile-texpresso: accept 8x8 `0219bbe7` |
| byte-flip-payload (1) | [#7474](witnesses/pixel-08.dds) | rgb-only(max 1) | image, glycin: accept 8x8 `67450faf`; image-dds, ddsfile-bcdec, ddsfile-texpresso: accept 8x8 `5a1c3d22` |

## Ten most significant leniency findings (ranked: pixel disagreement, then accept-vs-reject, by input count; rows whose only difference is the convention-adjusted equivalence are excluded)

- `seed:legacy-random-blocks` (5 inputs, accept-different-pixels): image, glycin: accept 8x8 `2518d062`; image-dds, ddsfile-bcdec: accept 8x8 `5fb5e337`; ddsfile-texpresso: accept 8x8 `158def82`. Witness witnesses/17-seed-legacy-random-blocks.dds.
- `pitch_or_linear_size` (163 inputs, accept-vs-reject): image: reject (The Image's dimensions are eit); glycin: reject (ExpectedError); image-dds: accept 7x5 `ecdce5f8`; ddsfile-bcdec, ddsfile-texpresso: reject (Truncated). Witness witnesses/01-pitch_or_linear_size.dds.
- `flags-bit` (92 inputs, accept-vs-reject): image: reject (Format error decoding Dds); glycin: reject (ExpectedError); image-dds, ddsfile-bcdec: accept 8x8 `f1099aed`; ddsfile-texpresso: accept 8x8 `02310379`. Witness witnesses/02-flags-bit.dds.
- `flags` (90 inputs, accept-vs-reject): image: reject (Format error decoding Dds); glycin: reject (ExpectedError); image-dds, ddsfile-bcdec, ddsfile-texpresso: accept 8x8 `2c893cd4`. Witness witnesses/03-flags.dds.
- `pf_b_mask` (47 inputs, accept-vs-reject): image: reject (The image format `DDS` is not ); glycin: reject (ExpectedError); image-dds: reject (Surface); ddsfile-bcdec: accept 8x8 `8e99a8ee`; ddsfile-texpresso: na. Witness witnesses/04-pf_b_mask.dds.
- `pf_g_mask` (47 inputs, accept-vs-reject): image: reject (The image format `DDS` is not ); glycin: reject (ExpectedError); image-dds: reject (Surface); ddsfile-bcdec: accept 8x8 `05d3c6ca`; ddsfile-texpresso: na. Witness witnesses/05-pf_g_mask.dds.
- `pf_r_mask` (44 inputs, accept-vs-reject): image: reject (The image format `DDS` is not ); glycin: reject (ExpectedError); image-dds: reject (Surface); ddsfile-bcdec: accept 5x3 `b981666e`; ddsfile-texpresso: na. Witness witnesses/06-pf_r_mask.dds.
- `pf_a_mask` (38 inputs, accept-vs-reject): image: reject (The image format `DDS` is not ); glycin: reject (ExpectedError); image-dds: reject (Surface); ddsfile-bcdec: accept 5x3 `85aa482f`; ddsfile-texpresso: na. Witness witnesses/07-pf_a_mask.dds.
- `seed:uncompressed-bitmask` (25 inputs, accept-vs-reject): image: reject (The image format `DDS` is not ); glycin: reject (ExpectedError); image-dds: reject (Surface); ddsfile-bcdec: accept 8x8 `2c893cd4`; ddsfile-texpresso: na. Witness witnesses/08-seed-uncompressed-bitmask.dds.
- `dx10_misc_flag-bit` (21 inputs, accept-vs-reject): image: reject (Format error decoding Dds); glycin: reject (ExpectedError); image-dds, ddsfile-bcdec: accept 8x8 `dc1bccb4`; ddsfile-texpresso: accept 8x8 `ef7a753e`. Witness witnesses/09-dx10_misc_flag-bit.dds.

Seeds: legacy FourCC DXT1/2/3/4/5 and ATI1/ATI2/BC4U/BC5U, 13 uncompressed bitmask layouts (RGB, RGBA, luminance, alpha, 8/16/24/32-bit), DX10 extended header (BC1-5,7, RGBA8, BGRA8, R8, A8, BGRX8), mipmap chains, cube maps (legacy caps2 and DX10), volume textures, DX10 arrays. Mutations: truncation at structural boundaries, trailing bytes, byte flips, every header field at boundary values, flag-bit toggles. Provenance (seed name, rule, field, old and new value) per input is in the manifest.

## Run

`./run.sh --inputs 10000 --seed 731 --jobs 4` from `/__modal/volumes/vo-Iyh3VLkOB7cEHrIp2RkTs7/projects/leniency/dds`. `python3 reproduce.py FILE` runs one file through every decoder. Per-input results and manifest: `/tmp/claude-0/leniency-dds`.
