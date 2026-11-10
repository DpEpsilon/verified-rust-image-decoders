# HEIF/AVIF availability survey

**Status: AVIF implemented (libheif vs libavif). HEIC/HEVC and other HEIF codecs: only libheif exists here, no comparison.**

## What glycin uses

`glycin-loaders/glycin-heif/src/main.rs`: `libheif-rs` 2.7 (features `v1_20`, default features off) over the system libheif. `HeifContext::read_from_reader`, `primary_image_handle`, `LibHeif::new().decode(&handle, ColorSpace::Rgb(chroma), None)`; 8-bit for 8-bit sources, 16-bit scaled for higher depths; EXIF/ICC/NCLX read from the handle. Registered: `image/avif` (identifier `ftypavif`), `image/heif`. The C runner calls the same libheif C API (`heif_context_read_from_memory_without_copy`, primary handle, `heif_decode_image` to interleaved RGBA 8-bit) on a libheif 1.20.2 built here; it does not use the Rust wrapper, and the 10-bit path glycin takes is flattened to 8-bit.

## Installed here (before any building)

No libheif, libde265, x265, aom, dav1d, libavif, ffmpeg (`pkg-config` and `ldconfig -p` empty); no `heif-*`/`avif*` tools; no GDK HEIF/AVIF module; no cmake, no nasm. libpng and libjpeg headers present (needed by the libheif/libavif example tools).

## Built under /tmp/claude-0/prefix from allowed domains

| component | source | purpose |
| --- | --- | --- |
| cmake 3.31.0 | github.com Kitware release tarball (reachable through github.com's asset redirect) | build system for the rest |
| dav1d 1.5.1 (asm off) | github.com/videolan/dav1d mirror, meson | AV1 pixel decoder for both candidates |
| libde265 1.0.15 | github.com/strukturag/libde265 | HEVC decoder inside libheif (HEIC not compared) |
| libheif 1.20.2 | github.com/strukturag/libheif, cmake | **glycin-path candidate** |
| libavif 1.3.0 | github.com/AOMediaCodec/libavif, cmake, dav1d codec | **independent container parser / YUV->RGB** |
| ravif 0.13 / rav1e 0.8 | crates.io | AVIF **encoder** for seeds (pure Rust, no asm) |
| libxml2 2.14.5 | gitlab.gnome.org | (SVG adapter only) |

## Seeds

`avif-seeds` writes 22 AVIF files: 8-bit/10-bit, RGB and RGBA (separate alpha item), 1x1 to 64x48, identity vs YCbCr colour model, BT.601/709/2020-NCL x full/limited range, quality 10/85/100, premultiplied and clean alpha, EXIF item. `harness.py` derives irot/imir/clap seeds by inserting those properties (ipco/ipma rewrite plus iloc offset shift). HEIC seeds are **not** available: no HEVC encoder was built.

## Gaps

- HEVC/HEIC second decoder: needs apt `libavcodec-dev`/`ffmpeg` or another independent HEVC-HEIF reader; and an encoder for HEIC seeds (x265: github.com/videolan/x265 builds with cmake, about 10 minutes single core; not done).
- Only one AV1 pixel decoder (dav1d) is exercised in both columns; an independent one would be `rav1d` (crates.io) wired to a Rust AVIF parser (`avif-parse`), not done.
- Grid, multi-image, uncompressed (`unci`) and JPEG-in-HEIF files can be produced with the built `heif-enc` (libheif example tool) but libavif cannot decode them, so they would only populate libheif.
- Exact glycin binding (libheif-rs on a distribution libheif): apt `libheif-dev` plus plugins `libheif-plugin-dav1d` or `-aomdec`, `libheif-plugin-libde265`.

## Harness note: address-space cap and threads

`engine.invoke` runs each decoder under `prlimit --as=768MiB`. The runner therefore disables libheif's decoding threads (`heif_context_set_max_decoding_threads(ctx, 0)`) and libavif's (`maxThreads = 1`). The machine has one CPU shared with other jobs, so crash and timeout counts are counts under load.
