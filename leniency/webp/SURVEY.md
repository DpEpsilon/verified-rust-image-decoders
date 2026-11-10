# WebP: what was available (surveyed 2026-11-10)

| Candidate | Status | Used as |
| --- | --- | --- |
| glycin-image-rs `ImgLoader` (`image/webp` -> `image::WebPDecoder`, image-webp 0.2.4) | in-tree path, builds from `~/projects/glycin-plan/glycin` | column `glycin` |
| `image::codecs::webp::WebPDecoder` direct | same code as glycin (ONE vote); catches glue differences (frame loop, conversion) | column `image-webp` |
| libwebp 1.5.0 (apt: libwebp7, libwebpdemux2, libwebpmux3, headers + static libs) | installed, no `cwebp`/`dwebp` CLIs, so called through C | columns `libwebp-simple` (`WebPDecodeRGBA`, stills only) and `libwebp-anim` (`WebPAnimDecoder`); ONE vote |
| libwebp encoder / mux / anim encoder | installed | seed generator `runners/mkseeds.c` (lossless, lossy, near-lossless, exact, sharp YUV, alpha variants, ICCP/EXIF/XMP, animation with mixed frames) |
| gdk-pixbuf WebP module | absent (installed modules: ani bmp gif icns ico pnm qtif tga tiff xbm xpm) | unavailable |
| `webp` 0.3.1 crate (libwebp-sys 0.9.6) | in the cargo cache, wraps libwebp | not built: same code as libwebp |
| independent pure-Rust or other WebP decoder | none found | unavailable; the matrix has two independent implementations (image-webp, libwebp) |

No cmake, cwebp, dwebp, or Pillow in the base image. The harness needs none of them.
