# JPEG XL (image/jxl) leniency matrix (public: synthetic inputs, crash rows excluded)

Seed 731; 7000 inputs (39 unmutated synthetic seeds from 39 distinct seed files, the rest mutations); 5884 mutations changed the acceptance or cross-decoder pixel relation relative to their seed; 151.9 s comparison pass, 201.6 s total including witness minimisation.

All inputs are synthetic (produced by the seed tool in this directory from closed-form pixel patterns, then mutated); no third-party file is involved. Inputs on which any decoder crashed or timed out are excluded from this public summary.

## Decoders

Column | Implementation group (one vote per group) | Status counts | What it is
--- | --- | --- | ---
glycin | jxl-rs | error 5758, ok 1242 | glycin-image-rs `ImgLoader` for `image/jxl` (special loader::jxl -> `jxl` crate 0.8.0 = jxl-rs), frames via `specific_frame` until `NoMoreFrames`
jxl-oxide | jxl-oxide | error 5249, ok 1751 | jxl-oxide 0.12.6 `JxlImage::render_frame` for every keyframe; f32 -> RGBA8 by round(clamp*255); independent of jxl-rs
libjxl | libjxl | error 5466, ok 1534 | libjxl (github master 0.13.0-dev, built with cmake, single-threaded) `JxlDecoder`, float RGBA output, unpremultiplied alpha, orientation applied, round(clamp*255)

Unavailable (not counted as rejecting):

- **gdk-pixbuf JXL module**: not installed
- **`jpegxl-rs` / `jpegxl-sys` crates**: bind libjxl: not independent of the libjxl column, not built
- **CMYK and spot-colour output in the native/oxide runners**: reported as `skip` (n/a); no colour conversion is implemented

Pixel comparison: exact RGBA8 SHA-256 first; otherwise the maximum absolute channel difference over all frames (alpha included) decides: at most 2 counts as within tolerance (documented per format), larger is a pixel disagreement. Differences confined to RGB under alpha 0 on both sides are reported separately (`transparent-rgb`). Dimension and frame-count differences are never tolerated.

## Classification

Classification | Count
--- | ---:
all-reject | 5155
accept-vs-reject | 638
all-accept-within-tolerance | 615
all-accept-identical-pixels | 576
accept-different-pixels | 16

## Seeds (unmutated)

Seed | Class | glycin | jxl-oxide | libjxl
--- | --- | --- | --- | ---
anim-ml-rgb8-2f | all-accept-identical-pixels | 12×12×2f `e8224d4b` | 12×12×2f `e8224d4b` | 12×12×2f `e8224d4b`
anim-ml-rgb8-3f-container | all-accept-identical-pixels | 16×16×3f `3e9a9ba0` | 16×16×3f `3e9a9ba0` | 16×16×3f `3e9a9ba0`
anim-ml-rgba8-3f | all-accept-identical-pixels | 16×16×3f `ff5710ba` | 16×16×3f `ff5710ba` | 16×16×3f `ff5710ba`
anim-vd-rgb8-3f | all-accept-within-tolerance | 24×24×3f `f7309f1e` | 24×24×3f `7e075a00` | 24×24×3f `3fd7ff8d`
ml-ga8-16x16 | all-accept-identical-pixels | 16×16 `1beac8cc` | 16×16 `1beac8cc` | 16×16 `1beac8cc`
ml-gray16-16x16 | all-accept-identical-pixels | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4`
ml-gray8-16x16 | all-accept-identical-pixels | 16×16 `4102f1f4` | 16×16 `4102f1f4` | 16×16 `4102f1f4`
ml-rgb16-16x16 | all-accept-identical-pixels | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0`
ml-rgb16-pq | all-accept-identical-pixels | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0`
ml-rgb8-16x16 | all-accept-identical-pixels | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0`
ml-rgb8-17x13 | all-accept-identical-pixels | 17×13 `8b7ce752` | 17×13 `8b7ce752` | 17×13 `8b7ce752`
ml-rgb8-1x1 | all-accept-identical-pixels | 1×1 `9b24e70d` | 1×1 `9b24e70d` | 1×1 `9b24e70d`
ml-rgb8-container | all-accept-identical-pixels | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0`
ml-rgb8-effort1 | all-accept-identical-pixels | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0`
ml-rgb8-effort7 | all-accept-identical-pixels | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0`
ml-rgb8-exif-brob | all-accept-identical-pixels | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0`
ml-rgb8-exif-xmp | all-accept-identical-pixels | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0`
ml-rgb8-linear | all-accept-identical-pixels | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0`
ml-rgb8-orient6 | all-accept-identical-pixels | 12×16 `b00615c1` | 12×16 `b00615c1` | 12×16 `b00615c1`
ml-rgb8-p3 | all-accept-identical-pixels | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0` | 16×16 `3e9a9ba0`
ml-rgb8-squeeze | all-accept-identical-pixels | 24×24 `fabe0c38` | 24×24 `fabe0c38` | 24×24 `fabe0c38`
ml-rgba16-16x16 | all-accept-identical-pixels | 16×16 `8a76a27a` | 16×16 `8a76a27a` | 16×16 `8a76a27a`
ml-rgba8-16x16 | all-accept-identical-pixels | 16×16 `ff5710ba` | 16×16 `ff5710ba` | 16×16 `ff5710ba`
mlossy-rgb8-d2 | all-accept-within-tolerance | 24×24 `5540d56a` | 24×24 `49361c81` | 24×24 `928bcece`
vd-gray8-d1 | all-accept-within-tolerance | 32×32 `63f58ab7` | 32×32 `a6b0de38` | 32×32 `36c8c0ea`
vd-rgb16-d1 | all-accept-within-tolerance | 24×24 `a4b33847` | 24×24 `e69f1164` | 24×24 `85a696b5`
vd-rgb8-d0p3-e7 | all-accept-within-tolerance | 32×32 `d5c1240c` | 32×32 `1d98ca9e` | 32×32 `9334f16f`
vd-rgb8-d1-17x13 | all-accept-within-tolerance | 17×13 `ff67f5e8` | 17×13 `c65d68ff` | 17×13 `630e93c5`
vd-rgb8-d1-80x72 | all-accept-within-tolerance | 80×72 `c54f2e04` | 80×72 `47a580cc` | 80×72 `50d581c9`
vd-rgb8-d1-decspeed4 | all-accept-within-tolerance | 32×32 `cced147d` | 32×32 `cc1cd32f` | 32×32 `fda760be`
vd-rgb8-d1-effort1 | all-accept-within-tolerance | 32×32 `cced147d` | 32×32 `cc1cd32f` | 32×32 `fda760be`
vd-rgb8-d1-effort9 | all-accept-within-tolerance | 32×32 `d0f4ee92` | 32×32 `41a44846` | 32×32 `c3036f08`
vd-rgb8-d1-exif | all-accept-within-tolerance | 32×32 `cced147d` | 32×32 `cc1cd32f` | 32×32 `fda760be`
vd-rgb8-d1-noise | all-accept-within-tolerance | 32×32 `a24a16a4` | 32×32 `1ec0bcdd` | 32×32 `7fa5e0ad`
vd-rgb8-d1-prog | all-accept-within-tolerance | 32×32 `cced147d` | 32×32 `cc1cd32f` | 32×32 `fda760be`
vd-rgb8-d1-resamp2 | all-accept-within-tolerance | 32×32 `cced147d` | 32×32 `cc1cd32f` | 32×32 `fda760be`
vd-rgb8-d1 | all-accept-within-tolerance | 32×32 `cced147d` | 32×32 `cc1cd32f` | 32×32 `fda760be`
vd-rgb8-d4 | all-accept-within-tolerance | 32×32 `a398358d` | 32×32 `d59fa81f` | 32×32 `cd899bf6`
vd-rgba8-d1 | all-accept-within-tolerance | 32×32 `061817a6` | 32×32 `4f21cd47` | 32×32 `06d9591e`

## Who accepts what (accept-vs-reject partitions)

Accepting decoders | Rejecting | Count | Top rules | Witness | glycin | jxl-oxide | libjxl
--- | --- | ---: | --- | --- | --- | --- | ---
jxl-oxide | glycin, libjxl | 308 | field:ftyp_TBox 60; field:ftyp_brand 60; field:ftyp_minor 57; ftyp-brand 30 | [lenient-01-3250.jxl](witnesses/lenient-01-3250.jxl) | reject `ExpectedError` | 1×1 `9b24e70d` | reject `decoder-error`
jxl-oxide, libjxl | glycin | 233 | truncation 69; trailing-bytes 60; field:xml_LBox 22; field:jxlc_LBox 19 | [lenient-02-248.jxl](witnesses/lenient-02-248.jxl) | reject `ExpectedError` | 16×16 `4102f1f4` | 16×16 `4102f1f4`
libjxl | glycin, jxl-oxide | 62 | field:brob_LBox 12; field:jxlp_index 12; trailing-bytes 10; field:jxlp_LBox 7 | [lenient-03-6009.jxl](witnesses/lenient-03-6009.jxl) | reject `ExpectedError` | reject `invalid container` | 16×16 `4102f1f4`
glycin, libjxl | jxl-oxide | 32 | box-insert 20; box-duplicate 9; box-length-zero 2; field:cs_byte7 1 | [lenient-04-560.jxl](witnesses/lenient-04-560.jxl) | 16×16 `4102f1f4` | reject `Invalid Data` | 16×16 `4102f1f4`
glycin, jxl-oxide | libjxl | 3 | field:cs_byte6 1; cs-header-byte-sweep 1; field:cs_byte7 1 | [lenient-05-5152.jxl](witnesses/lenient-05-5152.jxl) | 32×32 `d53fb63e` | 32×32 `56b4d6e8` | reject `decoder-error`

## Provenance clusters (mutation rule; changed-relation counts)

A mutation joins a cluster only if it changed acceptance or the cross-decoder pixel relation relative to its unmutated seed. The row shows the most frequent outcome pattern.

Rule (changed; patterns) | Witness | glycin | jxl-oxide | libjxl
--- | --- | --- | --- | ---
truncation (573; 9) | [rule-01-6245.jxl](witnesses/rule-01-6245.jxl) | reject `ExpectedError` | reject `reader ended before pa` | reject `decoder-error`
cs-header-byte-sweep (569; 2) | [rule-02-451.jxl](witnesses/rule-02-451.jxl) | reject `ExpectedError` | reject `bitstream validation f` | reject `decoder-error`
bit-flip (490; 18) | [rule-03-3395.jxl](witnesses/rule-03-3395.jxl) | reject `ExpectedError` | reject `bitstream validation f` | reject `decoder-error`
byte-replace (370; 16) | [rule-04-4917.jxl](witnesses/rule-04-4917.jxl) | reject `ExpectedError` | reject `frame error` | reject `decoder-error`
field:cs_byte0 (178; 1) | [rule-05-112.jxl](witnesses/rule-05-112.jxl) | reject `ExpectedError` | reject `bitstream validation f` | reject `decoder-error`
field:cs_byte2 (178; 1) | [rule-06-2615.jxl](witnesses/rule-06-2615.jxl) | reject `ExpectedError` | reject `Enum` | reject `decoder-error`
field:cs_byte1 (177; 1) | [rule-07-345.jxl](witnesses/rule-07-345.jxl) | reject `ExpectedError` | reject `bitstream validation f` | reject `decoder-error`
field:cs_byte9 (175; 4) | [rule-08-2653.jxl](witnesses/rule-08-2653.jxl) | reject `ExpectedError` | reject `frame error` | reject `decoder-error`
field:cs_byte4 (174; 2) | [rule-09-411.jxl](witnesses/rule-09-411.jxl) | reject `ExpectedError` | reject `Enum` | reject `decoder-error`
field:cs_byte3 (170; 3) | [rule-10-179.jxl](witnesses/rule-10-179.jxl) | reject `ExpectedError` | reject `no frames decoded` | reject `decoder-error`
field:cs_byte14 (162; 6) | [rule-11-1777.jxl](witnesses/rule-11-1777.jxl) | reject `ExpectedError` | reject `frame error` | reject `decoder-error`
field:cs_byte15 (162; 4) | [rule-12-1055.jxl](witnesses/rule-12-1055.jxl) | reject `ExpectedError` | reject `frame error` | reject `decoder-error`
field:cs_byte8 (160; 1) | [rule-13-750.jxl](witnesses/rule-13-750.jxl) | reject `ExpectedError` | reject `no frames decoded` | reject `decoder-error`
field:cs_byte10 (159; 5) | [rule-14-403.jxl](witnesses/rule-14-403.jxl) | reject `ExpectedError` | reject `no frames decoded` | reject `decoder-error`
field:cs_byte6 (159; 3) | #1863 | reject `ExpectedError` | reject `frame error` | reject `decoder-error`
field:cs_byte7 (158; 5) | #67 | reject `ExpectedError` | reject `frame error` | reject `decoder-error`
field:cs_byte13 (153; 3) | #1523 | reject `ExpectedError` | reject `frame error` | reject `decoder-error`
field:cs_byte11 (142; 3) | #289 | reject `ExpectedError` | reject `frame error` | reject `decoder-error`
field:cs_byte5 (136; 1) | #903 | reject `ExpectedError` | reject `Enum` | reject `decoder-error`
field:cs_byte12 (133; 1) | #1815 | reject `ExpectedError` | reject `frame error` | reject `decoder-error`
jxlp-split (72; 3) | #770 | reject `ExpectedError` | reject `invalid container` | reject `decoder-error`
trailing-bytes (70; 6) | #5023 | reject `ExpectedError` | 16×16 `4102f1f4` | 16×16 `4102f1f4`
field:JXL_LBox (60; 1) | #568 | reject `ExpectedError` | reject `bitstream validation f` | reject `decoder-error`
field:JXL_TBox (60; 1) | #455 | reject `ExpectedError` | reject `bitstream validation f` | reject `decoder-error`
field:ftyp_LBox (60; 1) | #128 | reject `ExpectedError` | reject `reader ended before pa` | reject `decoder-error`
field:ftyp_TBox (60; 3) | #857 | reject `ExpectedError` | 16×16 `4102f1f4` | reject `decoder-error`
field:ftyp_brand (60; 3) | #646 | reject `ExpectedError` | 16×16 `4102f1f4` | reject `decoder-error`
prefix (58; 1) | #1715 | reject `ExpectedError` | reject `bitstream validation f` | reject `decoder-error`
wrap (58; 11) | #1330 | reject `ExpectedError` | reject `bitstream validation f` | reject `decoder-error`
field:ftyp_minor (57; 3) | #73 | reject `ExpectedError` | 16×16 `4102f1f4` | reject `decoder-error`
field:jxlc_TBox (54; 1) | #365 | reject `ExpectedError` | reject `reader ended before pa` | reject `decoder-error`
field:jxlc_cs_byte0 (54; 1) | #480 | reject `ExpectedError` | reject `bitstream validation f` | reject `decoder-error`
field:jxlc_cs_byte1 (54; 1) | #251 | reject `ExpectedError` | reject `bitstream validation f` | reject `decoder-error`
field:jxlc_cs_byte2 (54; 1) | #1170 | reject `ExpectedError` | reject `frame error` | reject `decoder-error`
field:jxlc_LBox (52; 3) | #1708 | reject `ExpectedError` | reject `no frames decoded` | reject `decoder-error`
signature (50; 1) | #2159 | reject `ExpectedError` | reject `bitstream validation f` | reject `decoder-error`
field:jxlc_cs_byte3 (49; 1) | #137 | reject `ExpectedError` | reject `frame error` | reject `decoder-error`
box-duplicate (36; 9) | #1595 | reject `ExpectedError` | 16×16 `4102f1f4` | reject `decoder-error`
ftyp-brand (30; 3) | #491 | reject `ExpectedError` | 16×16 `4102f1f4` | reject `decoder-error`
box-swap (26; 6) | #1512 | reject `ExpectedError` | reject `bitstream validation f` | reject `decoder-error`

## Same acceptance, different pixels or animation

Class | Differing decoder pairs | Count | Max abs diff (witness) | Top rules | Witness | glycin | jxl-oxide | libjxl
--- | --- | ---: | --- | --- | --- | --- | --- | ---
all-accept-within-tolerance | glycin~jxl-oxide: within-tol; glycin~libjxl: within-tol; jxl-oxide~libjxl: within-tol | 585 | 1 | trailing-bytes 156; wrap 59; bit-flip 51 | [pixel-01-24.jxl](witnesses/pixel-01-24.jxl) | 32×32 `63f58ab7` | 32×32 `a6b0de38` | 32×32 `36c8c0ea`
all-accept-within-tolerance | glycin~jxl-oxide: within-tol; jxl-oxide~libjxl: within-tol | 21 | 1 | field:cs_byte14 6; field:cs_byte7 5; byte-replace 4 | [pixel-02-4554.jxl](witnesses/pixel-02-4554.jxl) | 32×32 `36c8c0ea` | 32×32 `a6b0de38` | 32×32 `36c8c0ea`
all-accept-within-tolerance | glycin~jxl-oxide: within-tol; glycin~libjxl: within-tol | 9 | 1 | field:cs_byte15 6; field:cs_byte14 1; field:cs_byte11 1 | [pixel-03-2645.jxl](witnesses/pixel-03-2645.jxl) | 32×32 `3ddcbde1` | 32×32 `170b31df` | 32×32 `170b31df`
accept-different-pixels | glycin~jxl-oxide: different-pixels; glycin~libjxl: different-pixels | 7 | 240 | byte-replace 4; bit-flip 2; field:cs_byte6 1 | [pixel-04-1927.jxl](witnesses/pixel-04-1927.jxl) | 16×16 `44402a4a` | 16×16 `647d369e` | 16×16 `647d369e`
accept-different-pixels | glycin~libjxl: different-pixels; jxl-oxide~libjxl: different-pixels | 4 | 255 | bit-flip 3; byte-replace 1 | [pixel-05-5416.jxl](witnesses/pixel-05-5416.jxl) | 16×16 `6ce21a0b` | 16×16 `6ce21a0b` | 16×16 `9b6ace11`
accept-different-pixels | glycin~jxl-oxide: different-pixels; glycin~libjxl: within-tol; jxl-oxide~libjxl: different-pixels | 2 | 57 | byte-replace 2 | [pixel-06-888.jxl](witnesses/pixel-06-888.jxl) | 32×32 `e29aa260` | 32×32 `14a6dd57` | 32×32 `32a91278`
accept-different-pixels | glycin~jxl-oxide: different-frame-count; jxl-oxide~libjxl: different-frame-count | 1 | 0 | field:cs_byte9 1 | [pixel-07-3174.jxl](witnesses/pixel-07-3174.jxl) | 16×16 `ff5710ba` | 16×16×3f `ff5710ba` | 16×16 `ff5710ba`
accept-different-pixels | glycin~jxl-oxide: different-pixels; jxl-oxide~libjxl: different-pixels | 1 | 245 | field:cs_byte13 1 | [pixel-08-3560.jxl](witnesses/pixel-08-3560.jxl) | 24×24 `8b95c503` | 24×24 `cc4d40ce` | 24×24 `8b95c503`
accept-different-pixels | glycin~jxl-oxide: different-pixels; glycin~libjxl: different-pixels; jxl-oxide~libjxl: different-pixels | 1 | 160 | byte-replace 1 | #3949 | 12×16 `852a07fd` | 12×16 `9fdedeeb` | 12×16 `5772afc0`

## Per-group lone votes and same-group divergence

Lone acceptor (only this implementation group accepts): {'jxl-oxide': 308, 'libjxl': 62}

Lone rejecter (only this group rejects): {'jxl-oxide': 32, 'jxl-rs': 233, 'libjxl': 3}

Same-group divergence (columns that share an implementation but disagree; indicates glue or API-path differences): none


## Acceptance disagreement counts (inputs where the two columns differ in accept/reject)

| | glycin | jxl-oxide | libjxl
--- | --- | --- | ---
glycin | 0 | 573 | 298
jxl-oxide | 573 | 0 | 405
libjxl | 298 | 405 | 0

## Significant observations (seed 731, 7,000 inputs; witnesses linked above)

1. jxl-oxide alone accepts 308 inputs that glycin (jxl-rs) and libjxl reject; led by ftyp box mutations (TBox, brand, minor version): it does not require the declared ftyp brand.
2. jxl-oxide and libjxl accept 233 inputs that jxl-rs (glycin) rejects: truncations (69), appended trailing bytes (60) and xml box length changes. Cause not investigated.
3. libjxl alone accepts 62 inputs (brob LBox, jxlp index, trailing bytes): it tolerates container-level inconsistencies that both Rust decoders reject.
4. glycin and libjxl accept 32 inputs that jxl-oxide rejects (box insert 20, duplicate 9, zero-length box 2).
5. glycin and jxl-oxide accept 3 inputs that libjxl rejects, all codestream header byte mutations (bytes 6 and 7).
6. Lossy VarDCT agreement: all 17 lossy seeds decode within 1 level of each other across three independent implementations (mean difference about 0.2 for jxl-rs vs the others, 0.005 for jxl-oxide vs libjxl); no tolerance above 1 was needed on unmutated seeds.
7. Modular lossless seeds (gray/RGB/RGBA, 8/16-bit, palette, squeeze, linear/P3/PQ encodings) and animated seeds are bit-identical in all three, including frame counts; orientation 6 is applied by all three (output size swaps), as glycin's loader leaves rotation to the decoder.
8. Pixel disagreements are rare: 16 inputs, all from bit flips or byte replacement in the codestream, with maxima 57 to 255; the same corrupted stream decodes differently, e.g. 7 inputs where glycin differs from both others.
9. Container wrapping of a bare codestream (jxlc, jxlp with correct indices, 2 parts) is accepted by all three and gives identical pixels; incorrect jxlp last-flag, index gap, reversed order produce split outcomes recorded in the partition table.
10. 5,155 of 7,000 inputs are rejected by all three; 576 identical; 615 within tolerance (mostly trailing-bytes and wrap variants of lossy seeds).


## Notes

- Seeds: 40 libjxl-encoded files (`seeds/`): modular lossless (gray/RGB/RGBA/GA, 8/16-bit, 1x1, odd sizes, efforts, squeeze, palette, orientation 6, linear/P3/PQ colour encodings), modular lossy, VarDCT (distances 0.3 to 4, efforts 1 to 9, progressive AC, resampling 2, photon noise, decoding speed 4, 16-bit, gray, alpha, multi-group 80x72), containers with Exif/XMP boxes (plain and Brotli `brob`), and animations (modular and VarDCT, 2 to 3 frames, with and without container).
- Mutation families: container box LBox/TBox/XLBox, ftyp brand, jxlp index and last-flag, jxlc <-> jxlp splitting and reordering, bare <-> container wrapping, signature variants, box insert/delete/duplicate/swap, 10 sampled values for each of the first 10 codestream bytes (the header is bit-packed; there is no bit-level parser), truncation at box boundaries, trailing bytes, random bit/byte flips.
- Lossy pixel tolerance: maximum absolute channel difference of at most 2 (RGBA8) is `within-tol`. Observed on the VarDCT seeds: at most 1.
- libjxl output is 8-bit rounded from float; glycin's jxl-rs path is asked for U8/U16/F32 by bit depth and converted by the runner; all three apply orientation.
- Crashes/timeouts: input saved under `/tmp/claude-0/leniency-jxl/anomalies/`, counted only; not investigated.

