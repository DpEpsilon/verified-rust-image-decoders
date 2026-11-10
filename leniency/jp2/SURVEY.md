# JPEG 2000: what was available (surveyed 2026-11-10)

| Candidate | Status | Used as |
| --- | --- | --- |
| glycin-image-rs `ImgLoader` (`image/jp2`, `image/x-jp2-codestream` -> `hayro_jpeg2000::integration::Jp2Decoder`, hayro-jpeg2000 0.3.5, default settings) | in-tree, builds | column `glycin` |
| hayro-jpeg2000 0.3.5 direct, `strict=false` / `strict=true` | same code as glycin (ONE vote); isolates glue and the strict switch | columns `hayro`, `hayro-strict` |
| OpenJPEG 2.5.x | not installed (no libopenjp2, no pkg-config file); built from github.com/uclouvain/openjpeg at 8314119b (static, cmake 3.31.6 fetched from a github release asset, about 2 min on this machine) | columns `openjpeg` (default strict mode) and `openjpeg-nonstrict`; also the encoder for seeds (`runners/mkseeds.c`) |
| JasPer 4.2.9 | not installed; built from github.com/jasper-software/jasper at 5ce57f67 | column `jasper` (third independent implementation); gray/RGB(+opacity) only |
| gdk-pixbuf JP2/J2K module | absent | unavailable |
| `jpeg2k` (openjpeg-sys), `openjp2` (Rust port of OpenJPEG) crates | not built | same code or same lineage as OpenJPEG, not an independent vote |
| Grok (GrokImageCompression) | not tried | skipped: heavy cmake build with extra deps, same lineage as OpenJPEG |

Runner-side normalisation (OpenJPEG, JasPer): colour channels from the colour space, first extra component is alpha, unsigned scale round(v*255/max), signed components shifted by +2^(prec-1), nearest-neighbour replication of subsampled components. sYCC/eYCC/CMYK/ICC-only/more than one extra component are `skip` (no vote) because no conversion is implemented. hayro scales to 8 bit with round(v/(2^p-1)*255), the same formula.
Palette and cdef-reordered JP2s are built in Python around a libopenjp2 codestream (the OpenJPEG encoder API cannot write pclr/cmap).
