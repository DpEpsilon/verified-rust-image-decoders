# EXR availability survey

**Status: implemented.** Candidates actually run: glycin path, `image` crate, tinyexr. Native OpenEXR is unavailable.

## What glycin uses

`image/x-exr` has no `glycin-*` loader of its own. `glycin-image-rs/src/lib.rs` dispatches it to `src/exr.rs`: `exr::meta::MetaData::read_from_buffered` for dimensions, then `exr::image::read::read().no_deep_data().largest_resolution_level().rgba_channels(...).first_valid_layer()` into half- or single-float RGB(A). The runner calls that loader (`glycin_image_rs::ImgLoader::load` + `specific_frame`) from the in-tree clone, not a reimplementation.

## Installed here

- No OpenEXR/Imath (`pkg-config OpenEXR Imath` fail, no `exrheader`, no libOpenEXR in `ldconfig -p`). No GDK EXR module (installed GDK loaders: ani bmp gif icns ico pnm qtif tga tiff xbm xpm).
- No ImageMagick, ffmpeg, Python PIL/numpy.
- Present: gcc/g++, make, meson, ninja, pkg-config, cargo 1.99.1 (`/opt/rustup`).
- Not present: cmake (downloaded from github.com release assets into `/tmp/claude-0/tools` for the HEIF work; not needed for EXR).

## Buildable from allowed domains

| candidate | source | status | independent of glycin path? |
| --- | --- | --- | --- |
| `exr` crate 1.74.x | crates.io | used by glycin | is the glycin path |
| `image` 0.25.10 `OpenExrDecoder` | crates.io | built (`image-exr`) | **no**: wraps the same `exr` crate with different glue (channel policy, display/data window) |
| tinyexr (C++ single header + miniz) | github.com/syoyo/tinyexr, branch `release`, tarball fetched by `build.sh` | built (`tinyexr`) | **yes** |
| OpenEXR + Imath | github.com/AcademySoftwareFoundation (cmake needed; cmake is obtainable) | **not built** (time/call budget); would be the reference decoder | yes |
| `openexr` / `openexr-sys` crates | crates.io | not built (cmake + OpenEXR source vendoring) | yes |

## Encoders for seeds

The `exr` crate writer (`src/seeds.rs`, binary `exr-seeds`) writes 37 deterministic seeds: compression none/RLE/ZIPS/ZIP/PIZ/PXR24/B44/B44A, f16/f32/u32 samples, RGB/RGBA/Y/YA/XYZ/extra channel, scanline and 8x8/4x4 tiles, line orders, 1x1 to 64x64, NaN/Inf samples. Not written: DWAA/DWAB, deep data, multipart, mipmap/ripmap levels (the writer path used here only emits single-level images).

## Gaps (what is missing)

- A native OpenEXR column. Needs OpenEXR + Imath sources from github.com and cmake (already downloadable); no apt package is required. With apt: `libopenexr-dev` (+ `openexr` for `exrheader`).
- DWA compression, deep, multipart and mipmap seeds (writer limitation, not an availability problem).

## Harness note: address-space cap and thread pools

`engine.invoke` runs each decoder under `prlimit --as=768MiB`. Under that cap rayon's default global thread pool cannot start, which made decoders that use rayon (`rawler`, `librsvg`, the `exr` crate's callers) report spurious errors or panics in a first run. The Rust runners now set `RAYON_NUM_THREADS=1` before any decoder code runs, and the numbers in this directory come from the second run. The machine has one CPU shared with other jobs, so crash and timeout counts are counts under load.

Note on tinyexr: its runner is a plain C++ program; it is compiled from the `release` branch tarball at build time (no commit pin), so a later run may fetch different tinyexr code.
