# JPEG 2000 (image/jp2, image/x-jp2-codestream) leniency matrix (public: synthetic inputs, crash rows excluded)

Seed 731; 6986 inputs (53 unmutated synthetic seeds from 53 distinct seed files, the rest mutations); 5423 mutations changed the acceptance or cross-decoder pixel relation relative to their seed; 360.7 s comparison pass, 571.5 s total including witness minimisation.

All inputs are synthetic (produced by the seed tool in this directory from closed-form pixel patterns, then mutated); no third-party file is involved. Inputs on which any decoder crashed or timed out are excluded from this public summary.

## Decoders

Column | Implementation group (one vote per group) | Status counts | What it is
--- | --- | --- | ---
glycin | hayro_jpeg2000 | error 3490, ok 3496 | glycin-image-rs `ImgLoader` for `image/jp2` / `image/x-jp2-codestream` (hayro_jpeg2000 0.3.5 `integration::Jp2Decoder`, default settings)
hayro | hayro_jpeg2000 | error 3572, ok 3414 | hayro-jpeg2000 0.3.5 called directly, `strict=false` (glycin default)
hayro-strict | hayro_jpeg2000 | error 4383, ok 2603 | hayro-jpeg2000 with `strict=true`
openjpeg | openjpeg | error 4455, ok 2240, skip 291 | OpenJPEG 2.5.x (github master, built with gcc/cmake) `opj_decode`, default strict mode; runner-side RGBA8 normalisation
openjpeg-nonstrict | openjpeg | error 4271, ok 2407, skip 308 | OpenJPEG with `opj_decoder_set_strict_mode(false)` (accepts truncated codestreams)
jasper | jasper | error 4827, ok 1751, skip 408 | JasPer 4.2.9 `jas_image_decode`, gray/RGB(+opacity) only; runner-side RGBA8 normalisation

Unavailable (not counted as rejecting):

- **gdk-pixbuf JP2/J2K module**: not installed
- **subsampled/YCC/CMYK colour conversion in the native runners**: OpenJPEG and JasPer rows report `skip` (n/a) for sYCC/eYCC/CMYK/ICC-only/>4-component outputs: no colour conversion is implemented in the runners, so those inputs do not vote for those columns
- **`jpeg2k` / `openjp2` Rust crates**: wrap or port OpenJPEG: not independent of the OpenJPEG column, not built

Pixel comparison: exact RGBA8 SHA-256 first; otherwise the maximum absolute channel difference over all frames (alpha included) decides: at most 3 counts as within tolerance (documented per format), larger is a pixel disagreement. Differences confined to RGB under alpha 0 on both sides are reported separately (`transparent-rgb`). Dimension and frame-count differences are never tolerated.

## Classification

Classification | Count
--- | ---:
accept-vs-reject | 2987
all-reject | 2881
all-accept-identical-pixels | 845
accept-different-pixels | 191
all-accept-within-tolerance | 82

## Seeds (unmutated)

Seed | Class | glycin | hayro | hayro-strict | openjpeg | openjpeg-nonstrict | jasper
--- | --- | --- | --- | --- | --- | --- | ---
j2k-5comp-unspec | accept-vs-reject | reject `ExpectedError` | reject `Validation` | reject `Validation` | n/a | n/a | 16×16 `3e9a9ba0`
j2k-cmyk8-ll | accept-vs-reject | 16×16 `0a2cc235` | 16×16 `0a2cc235` | reject `unsupported channel la` | 16×16 `0a2cc235` | 16×16 `0a2cc235` | 16×16 `3e9a9ba0`
j2k-ga8-ll | accept-vs-reject | 16×16 `7157c8a1` | 16×16 `7157c8a1` | reject `Validation` | 16×16 `7157c8a1` | 16×16 `7157c8a1` | 16×16 `4102f1f4`
j2k-gray1-ll | all-accept-identical-pixels | 16×16 `b3d08689` | 16×16 `b3d08689` | 16×16 `b3d08689` | 16×16 `b3d08689` | 16×16 `b3d08689` | 16×16 `b3d08689`
j2k-gray10-ll | all-accept-identical-pixels | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4`
j2k-gray12-ll | all-accept-identical-pixels | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4`
j2k-gray16-ll | all-accept-identical-pixels | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4`
j2k-gray16-lossy | accept-different-pixels | 24×24 `4ef1a708` | 24×24 `4ef1a708` | 24×24 `4ef1a708` | 24×24 `4791a645` | 24×24 `4791a645` | 24×24 `fb8120a9`
j2k-gray4-ll | all-accept-identical-pixels | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4`
j2k-gray8-17x13 | all-accept-identical-pixels | 17×13 `ca40751a` | 17×13 `ca40751a` | 17×13 `ca40751a` | 17×13 `ca40751a` | 17×13 `ca40751a` | 17×13 `ca40751a`
j2k-gray8-1x1 | all-accept-identical-pixels | 1×1 `e3820096` | 1×1 `e3820096` | 1×1 `e3820096` | 1×1 `e3820096` | 1×1 `e3820096` | 1×1 `e3820096`
j2k-gray8-bypass | all-accept-identical-pixels | 32×32 `596605d7` | 32×32 `596605d7` | 32×32 `596605d7` | 32×32 `596605d7` | 32×32 `596605d7` | 32×32 `596605d7`
j2k-gray8-cblk4x4 | all-accept-identical-pixels | 32×32 `596605d7` | 32×32 `596605d7` | 32×32 `596605d7` | 32×32 `596605d7` | 32×32 `596605d7` | 32×32 `596605d7`
j2k-gray8-cblk64x64-nres1 | all-accept-identical-pixels | 64×64 `746e3133` | 64×64 `746e3133` | 64×64 `746e3133` | 64×64 `746e3133` | 64×64 `746e3133` | 64×64 `746e3133`
j2k-gray8-layers3-lossy | all-accept-within-tolerance | 32×32 `f69bac4b` | 32×32 `f69bac4b` | 32×32 `f69bac4b` | 32×32 `5dcf09ab` | 32×32 `5dcf09ab` | 32×32 `8586403f`
j2k-gray8-ll | all-accept-identical-pixels | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4`
j2k-gray8-lossy-r10 | accept-different-pixels | 32×32 `5d977c50` | 32×32 `5d977c50` | 32×32 `5d977c50` | 32×32 `967b0f26` | 32×32 `967b0f26` | 32×32 `e039989e`
j2k-gray8-pcrl | all-accept-identical-pixels | 32×32 `596605d7` | 32×32 `596605d7` | 32×32 `596605d7` | 32×32 `596605d7` | 32×32 `596605d7` | 32×32 `596605d7`
j2k-gray8-rpcl | all-accept-identical-pixels | 32×32 `596605d7` | 32×32 `596605d7` | 32×32 `596605d7` | 32×32 `596605d7` | 32×32 `596605d7` | 32×32 `596605d7`
j2k-gray8-signed-ll | all-accept-identical-pixels | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4`
j2k-rgb12-ll | all-accept-identical-pixels | 16×16 `41f76705` | 16×16 `41f76705` | 16×16 `41f76705` | 16×16 `41f76705` | 16×16 `41f76705` | 16×16 `41f76705`
j2k-rgb16-ll | all-accept-identical-pixels | 16×16 `ccb19b2e` | 16×16 `ccb19b2e` | 16×16 `ccb19b2e` | 16×16 `ccb19b2e` | 16×16 `ccb19b2e` | 16×16 `ccb19b2e`
j2k-rgb8-cprl | all-accept-identical-pixels | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0`
j2k-rgb8-ll-mct | all-accept-identical-pixels | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0`
j2k-rgb8-ll-nomct | all-accept-identical-pixels | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0`
j2k-rgb8-lossy-r30 | all-accept-within-tolerance | 32×32 `bdea49a5` | 32×32 `bdea49a5` | 32×32 `bdea49a5` | 32×32 `d65b6475` | 32×32 `d65b6475` | 32×32 `d65b6475`
j2k-rgb8-lossy-r8 | accept-different-pixels | 32×32 `a9d8dfb1` | 32×32 `a9d8dfb1` | 32×32 `a9d8dfb1` | 32×32 `b5ce0ce4` | 32×32 `b5ce0ce4` | 32×32 `52ac6144`
j2k-rgb8-prec-sop-eph | all-accept-identical-pixels | 32×32 `a0edf012` | 32×32 `a0edf012` | 32×32 `a0edf012` | 32×32 `a0edf012` | 32×32 `a0edf012` | 32×32 `a0edf012`
j2k-rgb8-rlcp | all-accept-identical-pixels | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0`
j2k-rgb8-tiles8 | all-accept-identical-pixels | 24×16 `1f06785c` | 24×16 `1f06785c` | 24×16 `1f06785c` | 24×16 `1f06785c` | 24×16 `1f06785c` | 24×16 `1f06785c`
j2k-rgba8-ll | accept-vs-reject | 16×16 `2fa147b9` | 16×16 `2fa147b9` | reject `unsupported channel la` | 16×16 `2fa147b9` | 16×16 `2fa147b9` | 16×16 `3e9a9ba0`
jp2-cmyk8 | accept-vs-reject | 16×16 `97830e72` | reject `unsupported channel la` | reject `unsupported channel la` | n/a | n/a | n/a
jp2-ga8-cdef | all-accept-identical-pixels | 16×16 `7157c8a1` | 16×16 `7157c8a1` | 16×16 `7157c8a1` | 16×16 `7157c8a1` | 16×16 `7157c8a1` | 16×16 `7157c8a1`
jp2-gray12-lossy | accept-different-pixels | 24×24 `97d36c0b` | 24×24 `97d36c0b` | 24×24 `97d36c0b` | 24×24 `3c61b977` | 24×24 `3c61b977` | 24×24 `3c61b977`
jp2-gray16-ll | all-accept-identical-pixels | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4`
jp2-gray8-1x1 | all-accept-identical-pixels | 1×1 `e3820096` | 1×1 `e3820096` | 1×1 `e3820096` | 1×1 `e3820096` | 1×1 `e3820096` | 1×1 `e3820096`
jp2-gray8-ll | all-accept-identical-pixels | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4`
jp2-rgb8-icc | all-accept-identical-pixels | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | n/a
jp2-rgb8-ll | all-accept-identical-pixels | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0`
jp2-rgb8-lossy | all-accept-within-tolerance | 32×32 `bdea49a5` | 32×32 `bdea49a5` | 32×32 `bdea49a5` | 32×32 `d65b6475` | 32×32 `d65b6475` | 32×32 `d65b6475`
jp2-rgb8-tiles-sop | all-accept-identical-pixels | 24×16 `1f06785c` | 24×16 `1f06785c` | 24×16 `1f06785c` | 24×16 `1f06785c` | 24×16 `1f06785c` | 24×16 `1f06785c`
jp2-rgba8-ll-cdef | all-accept-identical-pixels | 16×16 `2fa147b9` | 16×16 `2fa147b9` | 16×16 `2fa147b9` | 16×16 `2fa147b9` | 16×16 `2fa147b9` | 16×16 `2fa147b9`
jp2-sycc8 | all-accept-identical-pixels | 16×16 `9b71770b` | 16×16 `9b71770b` | 16×16 `9b71770b` | n/a | n/a | n/a
jp2-unspec-3c | accept-vs-reject | reject `ExpectedError` | reject `Format` | reject `Format` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | n/a
py-jp2-palette-gray8 | all-accept-identical-pixels | 16×16 `25e9ae4e` | 16×16 `25e9ae4e` | 16×16 `25e9ae4e` | 16×16 `25e9ae4e` | 16×16 `25e9ae4e` | 16×16 `25e9ae4e`
py-jp2-gray-as-srgb | accept-vs-reject | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | n/a | n/a | reject `layout`
py-jp2-no-ftyp | all-reject | reject `ExpectedError` | reject `Format` | reject `Format` | reject `read_header` | reject `read_header` | reject `decode`
py-jp2-jp2c-before-jp2h | accept-vs-reject | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | reject `read_header` | reject `read_header` | reject `decode`
py-jp2-bpcc-res | accept-vs-reject | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | reject `decode`
py-jp2-colr-icc-junk | accept-vs-reject | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | reject `decode`
py-jp2-colr-meth3 | all-accept-identical-pixels | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4`
py-jp2-rgb-cdef-reversed | all-accept-identical-pixels | 16×16 `8f40594e` | 16×16 `8f40594e` | 16×16 `8f40594e` | 16×16 `8f40594e` | 16×16 `8f40594e` | 16×16 `8f40594e`
py-jp2-rgb-ihdr-wrong-nc | accept-vs-reject | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | reject `decode`

## Who accepts what (accept-vs-reject partitions)

Accepting decoders | Rejecting | Count | Top rules | Witness | glycin | hayro | hayro-strict | openjpeg | openjpeg-nonstrict | jasper
--- | --- | ---: | --- | --- | --- | --- | --- | --- | --- | ---
glycin, hayro, hayro-strict | openjpeg, openjpeg-nonstrict, jasper | 721 | field: )  _TBox 78; field:SOT_TPsot 71; field:COD_Lseg 70; field:SIZ_Lseg 68 | [lenient-01-1006.jp2](witnesses/lenient-01-1006.jp2) | 1×1 `e3820096` | 1×1 `e3820096` | 1×1 `e3820096` | reject `decode` | reject `decode` | reject `decode`
glycin, hayro, hayro-strict, openjpeg, openjpeg-nonstrict | jasper | 390 | field:colr_TBox 69; field:ihdr_bpc 57; field:ihdr_c 55; field:ihdr_nc 41 | [lenient-02-1913.jp2](witnesses/lenient-02-1913.jp2) | 16×16 `b3d08689` | 16×16 `b3d08689` | 16×16 `b3d08689` | 16×16 `b3d08689` | 16×16 `b3d08689` | reject `decode`
glycin, hayro | hayro-strict, openjpeg, openjpeg-nonstrict, jasper | 294 | field:Mff93_marker 61; field:SIZ_XRsiz 26; field:SIZ_YRsiz 25; field:COD_cbstyle 23 | [lenient-03-427.jp2](witnesses/lenient-03-427.jp2) | 1×1 `79dfad35` | 1×1 `79dfad35` | reject `Tile` | reject `read_header` | reject `read_header` | reject `decode`
glycin, hayro, hayro-strict, jasper | openjpeg, openjpeg-nonstrict | 274 | field:ihdr_width 45; field:ihdr_height 44; field:COD_cbh 33; field:COD_cbw 27 | [lenient-04-6083.jp2](witnesses/lenient-04-6083.jp2) | 1×1 `e3820096` | 1×1 `e3820096` | 1×1 `e3820096` | reject `read_header` | reject `read_header` | 1×1 `e3820096`
glycin, hayro, openjpeg, openjpeg-nonstrict | hayro-strict, jasper | 192 | field:SIZ_YRsiz 18; sot-psot 16; field:COD_levels 16; field:cdef_typ0 15 | [lenient-05-2261.jp2](witnesses/lenient-05-2261.jp2) | 1×1 `79dfad35` | 1×1 `79dfad35` | reject `Tile` | 1×1 `79dfad35` | 1×1 `79dfad35` | reject `decode`
glycin, hayro, openjpeg, openjpeg-nonstrict, jasper | hayro-strict | 188 | field:QCD_SPqcd0 37; field:QCD_SPqcd1 29; trailing-bytes 18; eoc 15 | [lenient-06-1277.jp2](witnesses/lenient-06-1277.jp2) | 1×1 `79dfad35` | 1×1 `79dfad35` | reject `Decoding` | 1×1 `79dfad35` | 1×1 `79dfad35` | 1×1 `79dfad35`
openjpeg, openjpeg-nonstrict | glycin, hayro, hayro-strict | 155 | field:colr_enumcs 60; wrap-colr-enumcs 40; field:ihdr_ipr 6; box-insert 6 | [lenient-07-2180.jp2](witnesses/lenient-07-2180.jp2) | reject `ExpectedError` | reject `Format` | reject `Format` | 1×1 `e3820096` | 1×1 `e3820096` | n/a
openjpeg, openjpeg-nonstrict | glycin, hayro, hayro-strict, jasper | 126 | marker-insert 29; field:COD_levels 20; field:jp2c_LBox 15; field:QCD_Sqcd 15 | [lenient-08-5349.jp2](witnesses/lenient-08-5349.jp2) | reject `ExpectedError` | reject `Validation` | reject `Validation` | 1×1 `e3820096` | 1×1 `e3820096` | reject `decode`
openjpeg, openjpeg-nonstrict, jasper | glycin, hayro, hayro-strict | 119 | field:jp2c_LBox 35; field:COD_layers 30; marker-insert 27; marker-duplicate 12 | [lenient-09-4396.jp2](witnesses/lenient-09-4396.jp2) | reject `ExpectedError` | reject `Marker` | reject `Marker` | 1×1 `e3820096` | 1×1 `e3820096` | 1×1 `e3820096`
openjpeg-nonstrict | glycin, hayro, hayro-strict, openjpeg, jasper | 97 | tile-data-truncate 48; field:SOT_Psot 23; truncation 14; field:COD_levels 3 | [lenient-10-2613.jp2](witnesses/lenient-10-2613.jp2) | reject `ExpectedError` | reject `Tile` | reject `Tile` | reject `decode` | 1×1 `e3820096` | reject `decode`
jasper | glycin, hayro, hayro-strict, openjpeg, openjpeg-nonstrict | 87 | field:jp2h_LBox 36; box-length-zero 11; box-duplicate 10; field:QCD_Sqcd 9 | [lenient-11-5338.jp2](witnesses/lenient-11-5338.jp2) | reject `ExpectedError` | reject `Validation` | reject `Validation` | reject `read_header` | reject `read_header` | 1×1572865 `3125ac5b`
glycin, hayro, hayro-strict | jasper | 75 | field:ihdr_bpc 9; field:ihdr_c 8; wrap-bare 8; field:cmap_TBox 6 | [lenient-12-4657.jp2](witnesses/lenient-12-4657.jp2) | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | n/a | n/a | reject `layout`
glycin, hayro, openjpeg-nonstrict | hayro-strict, openjpeg, jasper | 70 | tile-data-truncate 19; field:COD_cbstyle 13; sot-psot 11; field:SIZ_XRsiz 7 | [lenient-13-5128.jp2](witnesses/lenient-13-5128.jp2) | 1×1 `e3820096` | 1×1 `e3820096` | reject `Marker` | reject `decode` | 1×1 `e3820096` | reject `decode`
glycin, hayro, jasper | hayro-strict, openjpeg, openjpeg-nonstrict | 40 | sot-psot 18; field:SOT_Psot 5; field:COD_cbw 4; wrap-inconsistent 3 | [lenient-14-6311.jp2](witnesses/lenient-14-6311.jp2) | 1×1 `79dfad35` | 1×1 `79dfad35` | reject `Marker` | reject `decode` | reject `decode` | 1×1 `e3820096`
glycin | hayro, hayro-strict | 34 | field:ihdr_ipr 5; box-insert 5; field:colr_prec 4; box-duplicate 2 | #31 | 16×16 `97830e72` | reject `unsupported channel la` | reject `unsupported channel la` | n/a | n/a | n/a
jasper | glycin, hayro, hayro-strict | 25 | field:COD_layers 3; packet-data-byte 2; bit-flip 2; field:SIZ_Rsiz 2 | #3459 | reject `ExpectedError` | reject `Validation` | reject `Validation` | n/a | n/a | 16×16 `3e9a9ba0`
glycin, hayro, hayro-strict | openjpeg, openjpeg-nonstrict | 25 | field:ihdr_width 7; field:ihdr_height 6; field:COD_cbh 4; field:COD_mct 3 | #544 | 16×16 `9b71770b` | 16×16 `9b71770b` | 16×16 `9b71770b` | reject `read_header` | reject `read_header` | n/a
glycin | hayro, hayro-strict, jasper | 19 | field:SIZ_YRsiz 4; field:ihdr_nc 4; field:ihdr_c 2; field:SIZ_Ssiz 2 | #101 | 16×16 `7af360d2` | reject `unsupported channel la` | reject `Tile` | n/a | n/a | reject `decode`
glycin, hayro | hayro-strict, jasper | 14 | field:SIZ_XRsiz 3; field:QCD_SPqcd1 3; field:QCD_SPqcd0 2; field:COD_cbh 1 | #973 | 16×16 `4ccc172f` | 16×16 `4ccc172f` | reject `Tile` | n/a | n/a | reject `decode`
glycin | hayro, hayro-strict, openjpeg, openjpeg-nonstrict | 12 | field:ihdr_width 2; field:ihdr_height 2; field:SIZ_Ysiz 2; field:jp2h_LBox 1 | #150 | 16×16 `97830e72` | reject `unsupported channel la` | reject `unsupported channel la` | reject `read_header` | reject `read_header` | n/a
glycin | hayro, hayro-strict, openjpeg, openjpeg-nonstrict, jasper | 9 | field:ihdr_TBox 5; field:SOT_TPsot 2; field:COD_cbstyle 1; field:SIZ_Lseg 1 | #110 | 16×16 `97830e72` | reject `unsupported channel la` | reject `unsupported channel la` | reject `decode` | reject `decode` | reject `decode`
glycin, hayro, openjpeg, openjpeg-nonstrict | hayro-strict | 9 | wrap-colr-enumcs 6; field:QCD_SPqcd1 1; eoc 1; field:QCD_SPqcd0 1 | #1631 | 16×16 `8fc72ce4` | 16×16 `9426fe4f` | reject `unsupported channel la` | 16×16 `0a2cc235` | 16×16 `0a2cc235` | n/a
glycin | hayro, hayro-strict, openjpeg, jasper | 2 | field:COD_cbstyle 1; sot-psot 1 | #1166 | 16×16 `5bb853d0` | reject `unsupported channel la` | reject `Tile` | reject `decode` | n/a | reject `decode`
glycin, openjpeg, openjpeg-nonstrict, jasper | hayro, hayro-strict | 2 | field:colr_meth 2 | #2239 | 16×16 `97830e72` | reject `unsupported channel la` | reject `unsupported channel la` | 16×16 `0a2cc235` | 16×16 `0a2cc235` | 16×16 `3e9a9ba0`
glycin, hayro | hayro-strict | 2 | eoc 1; wrap-bare 1 | #4750 | 16×16 `9b71770b` | 16×16 `9b71770b` | reject `Format` | n/a | n/a | n/a

## Provenance clusters (mutation rule; changed-relation counts)

A mutation joins a cluster only if it changed acceptance or the cross-decoder pixel relation relative to its unmutated seed. The row shows the most frequent outcome pattern.

Rule (changed; patterns) | Witness | glycin | hayro | hayro-strict | openjpeg | openjpeg-nonstrict | jasper
--- | --- | --- | --- | --- | --- | --- | ---
field: )  _LBox (79; 1) | [rule-01-2706.jp2](witnesses/rule-01-2706.jp2) | reject `ExpectedError` | reject `Format` | reject `Format` | reject `read_header` | reject `read_header` | reject `decode`
field:SIZ_XOsiz (79; 16) | [rule-02-331.jp2](witnesses/rule-02-331.jp2) | reject `ExpectedError` | reject `Marker` | reject `Marker` | reject `read_header` | reject `read_header` | reject `decode`
field: )  _TBox (78; 6) | [rule-03-3392.jp2](witnesses/rule-03-3392.jp2) | 16×16 `b3d08689` | 16×16 `b3d08689` | 16×16 `b3d08689` | reject `read_header` | reject `read_header` | reject `decode`
field:COD_marker (78; 1) | [rule-04-1268.jp2](witnesses/rule-04-1268.jp2) | reject `ExpectedError` | reject `Marker` | reject `Marker` | reject `read_header` | reject `read_header` | reject `decode`
field:COM_marker (78; 5) | [rule-05-6094.jp2](witnesses/rule-05-6094.jp2) | reject `ExpectedError` | reject `Marker` | reject `Marker` | reject `read_header` | reject `read_header` | reject `decode`
field:QCD_Lseg (78; 1) | [rule-06-6015.jp2](witnesses/rule-06-6015.jp2) | reject `ExpectedError` | reject `Marker` | reject `Marker` | reject `read_header` | reject `read_header` | reject `decode`
field:SIZ_Csiz (78; 1) | [rule-07-444.jp2](witnesses/rule-07-444.jp2) | reject `ExpectedError` | reject `Marker` | reject `Marker` | reject `read_header` | reject `read_header` | reject `decode`
field:SIZ_XTOsiz (78; 1) | [rule-08-5781.jp2](witnesses/rule-08-5781.jp2) | reject `ExpectedError` | reject `Validation` | reject `Validation` | reject `read_header` | reject `read_header` | reject `decode`
field:SIZ_YOsiz (78; 19) | [rule-09-2997.jp2](witnesses/rule-09-2997.jp2) | reject `ExpectedError` | reject `Marker` | reject `Marker` | reject `read_header` | reject `read_header` | reject `decode`
field:SIZ_marker (78; 1) | [rule-10-4024.jp2](witnesses/rule-10-4024.jp2) | reject `ExpectedError` | reject `Format` | reject `Format` | reject `read_header` | reject `read_header` | reject `decode`
field:ftyp_LBox (78; 1) | [rule-11-3272.jp2](witnesses/rule-11-3272.jp2) | reject `ExpectedError` | reject `Format` | reject `Format` | reject `read_header` | reject `read_header` | reject `decode`
field:ftyp_TBox (78; 1) | [rule-12-254.jp2](witnesses/rule-12-254.jp2) | reject `ExpectedError` | reject `Format` | reject `Format` | reject `read_header` | reject `read_header` | reject `decode`
tile-data-truncate (78; 12) | [rule-13-3804.jp2](witnesses/rule-13-3804.jp2) | reject `ExpectedError` | reject `Tile` | reject `Tile` | reject `decode` | 16×16 `da573f3d` | reject `decode`
field:COD_Lseg (77; 7) | [rule-14-71.jp2](witnesses/rule-14-71.jp2) | 16×16 `b3d08689` | 16×16 `b3d08689` | 16×16 `b3d08689` | reject `read_header` | reject `read_header` | reject `decode`
field:COM_Lseg (77; 1) | #436 | reject `ExpectedError` | reject `Marker` | reject `Marker` | reject `read_header` | reject `read_header` | reject `decode`
field:QCD_marker (77; 1) | #5194 | reject `ExpectedError` | reject `Marker` | reject `Marker` | reject `read_header` | reject `read_header` | reject `decode`
field:SIZ_XRsiz (77; 22) | #3675 | 16×16 `91903e99` | 16×16 `91903e99` | reject `Tile` | reject `decode` | reject `decode` | reject `decode`
field:SOT_marker (77; 1) | #2836 | reject `ExpectedError` | reject `Marker` | reject `Marker` | reject `read_header` | reject `read_header` | reject `decode`
wrap-colr-enumcs (77; 22) | #1823 | reject `ExpectedError` | reject `Format` | reject `Format` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | n/a
field:COD_precinct0 (76; 4) | #1654 | reject `ExpectedError` | reject `Marker` | reject `Marker` | reject `read_header` | reject `read_header` | reject `decode`
field:Mff93_marker (76; 8) | #2810 | 16×16 `5f4ecdb7` | 16×16 `5f4ecdb7` | reject `Marker` | reject `decode` | reject `decode` | reject `decode`
field:SIZ_Lseg (76; 9) | #5776 | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | reject `read_header` | reject `read_header` | reject `decode`
field:SIZ_YTOsiz (76; 1) | #6598 | reject `ExpectedError` | reject `Validation` | reject `Validation` | reject `read_header` | reject `read_header` | reject `decode`
field:SOT_Isot (76; 2) | #4025 | reject `ExpectedError` | reject `Tile` | reject `Tile` | reject `decode` | reject `decode` | reject `decode`
field:SOT_Psot (76; 16) | #3857 | reject `ExpectedError` | reject `Tile` | reject `Tile` | reject `decode` | reject `decode` | reject `decode`
field:jP_LBox (76; 1) | #838 | reject `ExpectedError` | reject `Format` | reject `Format` | reject `read_header` | reject `read_header` | reject `decode`
field:jp2c_TBox (76; 1) | #1233 | reject `ExpectedError` | reject `Format` | reject `Format` | reject `read_header` | reject `read_header` | reject `decode`
truncation (76; 6) | #519 | reject `ExpectedError` | reject `Marker` | reject `Marker` | reject `read_header` | reject `read_header` | reject `decode`
box-delete (75; 2) | #2442 | reject `ExpectedError` | reject `Format` | reject `Format` | reject `read_header` | reject `read_header` | reject `decode`
field:COD_levels (75; 13) | #5013 | reject `ExpectedError` | reject `Marker` | reject `Marker` | reject `read_header` | reject `read_header` | reject `decode`
field:colr_LBox (75; 3) | #4201 | reject `ExpectedError` | reject `Format` | reject `Format` | reject `read_header` | reject `read_header` | reject `decode`
field:ihdr_LBox (75; 5) | #3954 | reject `ExpectedError` | reject `Format` | reject `Format` | reject `read_header` | reject `read_header` | reject `decode`
sot-psot (75; 26) | #3633 | reject `ExpectedError` | reject `Tile` | reject `Tile` | reject `decode` | reject `decode` | reject `decode`
field:COD_transform (75; 8) | #1752 | reject `ExpectedError` | reject `Marker` | reject `Marker` | reject `read_header` | reject `read_header` | reject `decode`
field:QCD_Sqcd (74; 18) | #2986 | reject `ExpectedError` | reject `Marker` | reject `Marker` | reject `read_header` | reject `read_header` | reject `decode`
field:SOT_Lseg (74; 9) | #3004 | 16×16 `b3d08689` | 16×16 `b3d08689` | 16×16 `b3d08689` | reject `decode` | reject `decode` | reject `decode`
field:SOT_TPsot (74; 9) | #1780 | 16×16 `b3d08689` | 16×16 `b3d08689` | 16×16 `b3d08689` | reject `decode` | reject `decode` | reject `decode`
field:ihdr_width (74; 9) | #145 | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4` | reject `read_header` | reject `read_header` | 16×16 `4102f1f4`
field:jp2h_TBox (74; 1) | #621 | reject `ExpectedError` | reject `Format` | reject `Format` | reject `read_header` | reject `read_header` | reject `decode`
field:jP_TBox (73; 1) | #2166 | reject `ExpectedError` | reject `Format` | reject `Format` | reject `read_header` | reject `read_header` | reject `decode`

## Same acceptance, different pixels or animation

Class | Differing decoder pairs | Count | Max abs diff (witness) | Top rules | Witness | glycin | hayro | hayro-strict | openjpeg | openjpeg-nonstrict | jasper
--- | --- | ---: | --- | --- | --- | --- | --- | --- | --- | --- | ---
accept-different-pixels | glycin~jasper: different-pixels; glycin~openjpeg: different-pixels; glycin~openjpeg-nonstrict: different-pixels; hayro~jasper: different-pixels | 88 | 255 | field:SIZ_Ysiz 14; field:SIZ_Xsiz 11; field:SIZ_XTsiz 8 | [pixel-01-568.jp2](witnesses/pixel-01-568.jp2) | 16×16 `f27d332f` | 16×16 `f27d332f` | 16×16 `f27d332f` | 16×16 `9503245a` | 16×16 `9503245a` | 16×16 `9503245a`
all-accept-within-tolerance | glycin~jasper: within-tol; glycin~openjpeg: within-tol; glycin~openjpeg-nonstrict: within-tol; hayro~jasper: within-tol | 60 | 1 | field:SOT_TNsot 6; field:ihdr_ipr 4; field:QCD_SPqcd1 4 | [pixel-02-4171.jp2](witnesses/pixel-02-4171.jp2) | 16×16 `d307b31b` | 16×16 `d307b31b` | 16×16 `d307b31b` | 16×16 `56e4792c` | 16×16 `56e4792c` | 16×16 `56e4792c`
accept-different-pixels | glycin~jasper: different-pixels; glycin~openjpeg: different-pixels; glycin~openjpeg-nonstrict: different-pixels; hayro~jasper: different-pixels | 44 | 11 | field:QCD_SPqcd1 5; field:SIZ_YTsiz 4; trailing-bytes 3 | [pixel-03-16.jp2](witnesses/pixel-03-16.jp2) | 32×32 `5d977c50` | 32×32 `5d977c50` | 32×32 `5d977c50` | 32×32 `967b0f26` | 32×32 `967b0f26` | 32×32 `e039989e`
accept-different-pixels | glycin~jasper: different-pixels; glycin~openjpeg: within-tol; glycin~openjpeg-nonstrict: within-tol; hayro~jasper: different-pixels | 26 | 128 | field:SIZ_Rsiz 3; field:QCD_SPqcd0 2; trailing-bytes 2 | [pixel-04-1526.jp2](witnesses/pixel-04-1526.jp2) | 24×24 `4ef1a708` | 24×24 `4ef1a708` | 24×24 `4ef1a708` | 24×24 `4791a645` | 24×24 `4791a645` | 24×24 `fb8120a9`
all-accept-within-tolerance | glycin~jasper: within-tol; glycin~openjpeg: within-tol; glycin~openjpeg-nonstrict: within-tol; hayro~jasper: within-tol | 19 | 2 | field:QCD_SPqcd1 2; field:SIZ_XTsiz 2; field:COD_cbstyle 2 | [pixel-05-3057.jp2](witnesses/pixel-05-3057.jp2) | 32×32 `e96962ba` | 32×32 `e96962ba` | 32×32 `e96962ba` | 32×32 `c8c89f1e` | 32×32 `c8c89f1e` | 32×32 `8161cafb`
accept-different-pixels | glycin~jasper: different-pixels; hayro~jasper: different-pixels; hayro-strict~jasper: different-pixels; openjpeg~jasper: different-pixels | 7 | 158 | field:pclr_b0 3; field:colr_meth 2; field:cdef_asoclast 1 | [pixel-06-2205.jp2](witnesses/pixel-06-2205.jp2) | 16×16 `01e93020` | 16×16 `01e93020` | 16×16 `01e93020` | 16×16 `01e93020` | 16×16 `01e93020` | 16×16 `bbe5236a`
accept-different-pixels | glycin~jasper: different-pixels; hayro~jasper: different-pixels; hayro-strict~jasper: different-pixels | 6 | 255 | wrap-bare 4; field:colr_enumcs 1; wrap-colr-enumcs 1 | [pixel-07-4742.jp2](witnesses/pixel-07-4742.jp2) | 16×16 `41f76705` | 16×16 `41f76705` | 16×16 `41f76705` | n/a | n/a | 16×16 `4102f1f4`
accept-different-pixels | glycin~jasper: different-pixels; glycin~openjpeg: different-pixels; glycin~openjpeg-nonstrict: different-pixels; hayro~jasper: different-pixels | 5 | 255 | field:QCD_SPqcd0 1; field:SIZ_Xsiz 1; field:COD_transform 1 | [pixel-08-3746.jp2](witnesses/pixel-08-3746.jp2) | 16×16 `6b6193b9` | 16×16 `6b6193b9` | 16×16 `6b6193b9` | 16×16 `9503245a` | 16×16 `9503245a` | 16×16 `b3d08689`
accept-different-pixels | glycin~hayro: different-pixels; glycin~hayro-strict: different-pixels; glycin~openjpeg: different-pixels; glycin~openjpeg-nonstrict: different-pixels | 3 | 91 | wrap-colr-enumcs 3 | #5164 | 32×32 `5a961c18` | 32×32 `c101fef4` | 32×32 `c101fef4` | 32×32 `d65b6475` | 32×32 `d65b6475` | n/a
accept-different-pixels | glycin~jasper: different-dimensions; glycin~openjpeg: different-dimensions; glycin~openjpeg-nonstrict: different-dimensions; hayro~jasper: different-dimensions | 3 | 0 | field:SIZ_XRsiz 2; field:SIZ_YRsiz 1 | #796 | 3×13 `dc1a5f0b` | 3×13 `dc1a5f0b` | 3×13 `dc1a5f0b` | 17×13 `ef5a59df` | 17×13 `ef5a59df` | 15×13 `229622db`
accept-different-pixels | glycin~hayro: different-pixels; glycin~hayro-strict: different-pixels; glycin~openjpeg: different-pixels; glycin~openjpeg-nonstrict: different-pixels | 3 | 136 | wrap-colr-enumcs 3 | #3893 | 16×16 `715f7abc` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | n/a
accept-different-pixels | glycin~openjpeg: different-pixels; glycin~openjpeg-nonstrict: different-pixels; hayro~openjpeg: different-pixels; hayro~openjpeg-nonstrict: different-pixels | 2 | 7 | wrap-colr-enumcs 2 | #1147 | 24×24 `97d36c0b` | 24×24 `97d36c0b` | 24×24 `97d36c0b` | 24×24 `3c61b977` | 24×24 `3c61b977` | n/a
accept-different-pixels | glycin~openjpeg: different-dimensions; glycin~openjpeg-nonstrict: different-dimensions; hayro~openjpeg: different-dimensions; hayro~openjpeg-nonstrict: different-dimensions | 2 | 0 | field:SIZ_XRsiz 2 | #4780 | 1×16 `05922607` | 1×16 `05922607` | 1×16 `05922607` | 16×16 `0b6476c9` | 16×16 `0b6476c9` | 1×16 `05922607`
all-accept-within-tolerance | glycin~openjpeg: within-tol; glycin~openjpeg-nonstrict: within-tol; hayro~openjpeg: within-tol; hayro~openjpeg-nonstrict: within-tol | 2 | 1 | field:COD_transform 2 | #5602 | 17×13 `c0b9d969` | 17×13 `c0b9d969` | 17×13 `c0b9d969` | 17×13 `38a0c526` | 17×13 `38a0c526` | 17×13 `c0b9d969`
accept-different-pixels | glycin~hayro: different-pixels; glycin~hayro-strict: different-pixels; glycin~openjpeg: different-pixels; glycin~openjpeg-nonstrict: different-pixels | 1 | 21 | wrap-colr-enumcs 1 | #755 | 32×32 `d4919cd3` | 32×32 `bdea49a5` | 32×32 `bdea49a5` | 32×32 `d65b6475` | 32×32 `d65b6475` | n/a
accept-different-pixels | glycin~jasper: within-tol; glycin~openjpeg: different-dimensions; glycin~openjpeg-nonstrict: different-dimensions; hayro~jasper: within-tol | 1 | 1 | field:SIZ_YRsiz 1 | #1291 | 32×1 `2cf307cc` | 32×1 `2cf307cc` | 32×1 `2cf307cc` | 32×32 `6a5e1dde` | 32×32 `6a5e1dde` | 32×1 `deacc9e1`
all-accept-within-tolerance | glycin~openjpeg: within-tol; glycin~openjpeg-nonstrict: within-tol; hayro~openjpeg: within-tol; hayro~openjpeg-nonstrict: within-tol | 1 | 3 | wrap-colr-enumcs 1 | #2783 | 32×32 `bdea49a5` | 32×32 `bdea49a5` | 32×32 `bdea49a5` | 32×32 `d65b6475` | 32×32 `d65b6475` | n/a

## Per-group lone votes and same-group divergence

Lone acceptor (only this implementation group accepts): {'hayro_jpeg2000': 1027, 'jasper': 87, 'openjpeg': 223}

Lone rejecter (only this group rejects): {'hayro_jpeg2000': 119, 'jasper': 654, 'openjpeg': 315}

Same-group divergence (columns that share an implementation but disagree; indicates glue or API-path differences): {'glycin/hayro-strict:different-pixels': 7, 'glycin/hayro-strict:status': 893, 'glycin/hayro:different-pixels': 11, 'glycin/hayro:status': 82, 'hayro/hayro-strict:status': 811, 'openjpeg/openjpeg-nonstrict:status': 167}


## Acceptance disagreement counts (inputs where the two columns differ in accept/reject)

| | glycin | hayro | hayro-strict | openjpeg | openjpeg-nonstrict | jasper
--- | --- | --- | --- | --- | --- | ---
glycin | 0 | 82 | 893 | 1850 | 1874 | 2020
hayro | 82 | 0 | 811 | 1830 | 1856 | 1992
hayro-strict | 893 | 811 | 0 | 1813 | 1980 | 1649
openjpeg | 1850 | 1830 | 1813 | 0 | 167 | 1112
openjpeg-nonstrict | 1874 | 1856 | 1980 | 167 | 0 | 1279
jasper | 2020 | 1992 | 1649 | 1112 | 1279 | 0

## Significant observations (seed 731, 7,000 inputs; witnesses linked above)

1. Lossy 9/7 pixels: hayro (and so glycin) differs from both OpenJPEG and JasPer on 8-bit lossy seeds by up to 10 (gray, 10:1) and 42 (RGB, 8:1) levels, mean 3.5 to 5.5, while OpenJPEG and JasPer agree within 1. For 12/16-bit lossy gray glycin matches OpenJPEG within 1; hayro's 8-bit path is the outlier.
2. hayro non-strict accepts 721 inputs (plus those hayro-strict rejects: 893 glycin/strict status differences) where hayro-strict rejects: marker and segment-length mutations (SOT TPsot, COD Lseg, unknown segment markers) are tolerated by default in glycin's path.
3. A 4-component bare codestream without cdef: hayro non-strict and OpenJPEG (runner policy) treat the fourth component as alpha; hayro-strict rejects; JasPer ignores it (different pixels, max 255).
4. JP2 colour-space declaration: an unspecified 3-component JP2 (jp2-unspec-3c) is rejected by glycin/hayro and decoded by OpenJPEG; a gray codestream declared sRGB (py-jp2-gray-as-srgb) is decoded by glycin/hayro and rejected by JasPer.
5. Box-layer leniency: jp2c before jp2h, bpcc/res boxes, and a junk ICC colr box (method 2) are accepted by glycin/hayro and OpenJPEG but rejected by JasPer in the seed set; the colr enumCS sweep (60 + 40 inputs) is accepted only by OpenJPEG in its partition (155 inputs).
6. Glycin/hayro accept ihdr width/height/bpc/colr-TBox inconsistencies that other decoders reject (390 inputs accepted by hayro and OpenJPEG; 274 by hayro and JasPer): the ihdr box is largely ignored in favour of the codestream SIZ.
7. OpenJPEG alone (strict and non-strict) accepts 155 + 126 inputs the other families reject (marker-insert, COD levels, jp2c LBox); OpenJPEG strict and non-strict differ on 167 inputs (truncated codestreams).
8. Lone votes per implementation family on accept/reject: hayro 1,027 lone acceptances and 119 lone rejections; OpenJPEG 223 and 315; JasPer 87 and 654 (JasPer is the strictest).
9. Pixel disagreements among accepting decoders: 191 inputs different, 82 within tolerance; the largest are SIZ Xsiz/Ysiz mutations (hayro vs the other two, up to 255) and subsampling (XRsiz/YRsiz) where the runners replicate nearest-neighbour.


## Notes

- Seeds: 44 libopenjp2-encoded files (`seeds/`, bare codestream `j2k-*` and boxed `jp2-*`; 1/4/8/10/12/16-bit, signed, 1 to 5 components, gray/sRGB/sYCC/CMYK/unspecified colour spaces, alpha via cdef, lossless 5/3 and lossy 9/7 with multiple rates and layers, tiles, precincts, SOP/EPH, all five progression orders, code-block sizes 4x4 to 64x64, bypass mode, ICC colr) plus 13 JP2 wrappers built in Python (palette pclr/cmap, bpcc, res, colr variants, reversed cdef, wrong ihdr, no ftyp, jp2c before jp2h).
- Mutation families: JP2 box LBox/TBox and ihdr/colr/pclr/cmap/cdef/bpcc fields, SIZ/COD/QCD/SOT field boundary values, marker delete/duplicate/insert, EOC removal, Psot variants, packet-data byte replacement, wrapper add/strip, inconsistent ihdr vs SIZ, enumCS sweep, truncation at box/marker boundaries, trailing bytes, random bit/byte flips.
- Lossy pixel tolerance: maximum absolute channel difference of at most 3 (RGBA8) is `within-tol`. The lossy 9/7 seeds are the cases it exists for; the histogram of observed differences is in `summary.json` (`visible_diff_histogram_pairs`).
- Columns `glycin`, `hayro`, `hayro-strict` are one implementation; `openjpeg`, `openjpeg-nonstrict` one; `jasper` one. Runner-side RGBA8 normalisation for OpenJPEG/JasPer is a harness policy (see SURVEY.md), so a disagreement on alpha detection for bare 4-component codestreams reflects that policy plus each library's cdef handling, not only the library.
- glycin MIME: the runner claims `image/x-jp2-codestream` when the data starts FF 4F, else `image/jp2`; both map to the same decoder in glycin-image-rs.
- OpenJPEG and JasPer print `skip` for colour spaces needing conversion (sYCC, eYCC, CMYK, ICC-only) or more than one extra component; those inputs do not vote in those columns.
- Crashes/timeouts: input saved under `/tmp/claude-0/leniency-jp2/anomalies/`, counted only; not investigated.

