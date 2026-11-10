# RAW availability survey

**Status: implemented for DNG only.** Candidates run: libopenraw (the crate glycin-raw uses), rawloader, rawler. Other registered RAW MIME types cannot be synthesised and are not covered.

## What glycin uses

`glycin-loaders/glycin-raw/src/bin/glycin-raw.rs` registers `image/x-canon-cr2`, `x-adobe-dng`, `x-epson-erf`, `x-minolta-mrw`, `x-olympus-orf`, `x-panasonic-rw`, `x-panasonic-rw2`, `x-pentax-pef`, `x-sony-srf`. The decoder is the **pure-Rust** crate `libopenraw =0.4.0-alpha.12` (not the C library): `rawfile_from_memory(buf, None)`, `raw_data(false)`, dimensions from the raw plane, orientation, then `rendered_image(&RenderingOptions::default())` to 16-bit RGB. The runner replicates exactly these calls (selector `libopenraw-glycin`).

## Installed here

No libraw, dcraw, darktable, rawtherapee, exiftool, ImageMagick. `libopenraw` C library not installed (and not what glycin uses). gcc/g++ and cargo are present.

## Buildable from allowed domains

| candidate | source | status | independent? |
| --- | --- | --- | --- |
| `libopenraw` 0.4.0-alpha.12 | crates.io (already in the cargo cache) | built | is the glycin path |
| `rawloader` 0.37.2 | crates.io | built | **yes** |
| `rawler` 0.8.0 (dnglab) | crates.io | built | **yes** (shares ancestry with rawloader but is a separate rewrite) |
| LibRaw | github.com/LibRaw/LibRaw (make/autotools or cmake) | **not built** (call budget); would be a fourth, C++ implementation | yes |
| libopenraw C (gitlab.freedesktop.org) | domain not allowed | unavailable | - |

## Synthetic seeds

RAW files are normally camera files, which this repository must not contain. A **TIFF-based DNG can be written** from scratch (`harness.py`, `make_dng`): little-endian TIFF with DNGVersion, CFA pattern/layout, black/white levels, ColorMatrix1, AsShotNeutral. 30 seeds: four CFA orders; 8/12 (packed)/14-in-16/16-bit integer and 32-bit float samples; strips, multiple strips, tiles; Deflate compression; flat IFD0-raw and IFD0-thumbnail plus SubIFD-raw layouts; ActiveArea/crop; LinearRaw 3-sample; monochrome; orientation 3/6/8; per-channel black levels; tiny/odd sizes; DNG files declaring Canon/Nikon/Sony/Leica as Make; missing ColorMatrix/AsShotNeutral; plain-TIFF CFA without DNG tags.

Not synthesisable here and therefore not generated: CR2 (Canon maker note + lossless JPEG), ERF, MRW, ORF, RW2, PEF, SRF (vendor containers, proprietary compression, camera-model tables). Lossless-JPEG DNG (compression 7) would need a lossless JPEG encoder, which is small but was not written. The `make-*` seeds only test vendor dispatch from DNG metadata, not those formats' decoders.

## Gaps

- Real vendor formats need camera files; the operator would have to supply a licence-cleared corpus outside this repository.
- LibRaw column: apt `libraw-dev` (or github.com LibRaw source + g++).

## Harness note: address-space cap and thread pools

`engine.invoke` runs each decoder under `prlimit --as=768MiB`. Under that cap rayon's default global thread pool cannot start, which made decoders that use rayon (`rawler`, `librsvg`, the `exr` crate's callers) report spurious errors or panics in a first run. The Rust runners now set `RAYON_NUM_THREADS=1` before any decoder code runs, and the numbers in this directory come from the second run. The machine has one CPU shared with other jobs, so crash and timeout counts are counts under load.
