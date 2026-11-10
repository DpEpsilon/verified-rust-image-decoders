# JPEG XL: what was available (surveyed 2026-11-10)

| Candidate | Status | Used as |
| --- | --- | --- |
| glycin-image-rs special loader `loader::jxl` (`jxl` crate 0.8.0 = jxl-rs) | in-tree, builds; frames via `specific_frame`, applies orientation, 8/16-bit/float output by bit depth | column `glycin` |
| jxl-oxide 0.12.6 | crates.io, builds; independent of jxl-rs | column `jxl-oxide` |
| libjxl | not installed; built from github.com/libjxl/libjxl at eaae1abb (0.13.0-dev, vendored highway/skcms, system brotli, tools off, static, cmake 3.31.6 from a github release asset, a few minutes) | column `libjxl` (C API, single-threaded); also the encoder for seeds (`runners/mkseeds.c`) |
| gdk-pixbuf JXL module | absent | unavailable |
| `jpegxl-rs` / `jpegxl-sys` 0.14 / 0.12 (libjxl 0.11.2) | in the cargo cache; bind libjxl | not built: same code as the libjxl column |
| `zune-jpegxl` etc. | encoder-only | not needed |

Three independent implementations (jxl-rs, jxl-oxide, libjxl). libjxl runner: float RGBA output, round(clamp*255), unpremultiplied alpha, default orientation handling (applied). jxl-oxide runner converts its f32 image the same way. CMYK/spot output is `skip`.
No Python JXL tooling exists in the image.
