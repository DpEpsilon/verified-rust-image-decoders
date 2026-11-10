# XBM leniency matrix (public subset)

Seed 731; 9904 synthetic inputs (89 valid/unmutated seeds from 89 seed files; 7166 mutations changed the acceptance/pixel-equality pattern relative to their seed); 109.9 s for the comparison pass. Inputs whose run involved a crash or timeout are excluded from every count and row below.

Classification | Count
--- | ---:
all-accept-identical-pixels | 286
accept-different-pixels | 0
accept-vs-reject | 3823
all-reject | 5795
crash-or-timeout | 0

## Decoders

| Decoder | ok | error | of which caught Rust panic | crash | timeout | top error classes |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| glycin | 415 | 9489 | 0 | - | - | ExpectedError (9489) |
| image-extras | 415 | 9489 | 0 | - | - | Format error decoding `XBM` (9489) |
| xbm-crate | 452 | 9452 | 125 | - | - | invalid header (5398), invalid termination string (588), invalid digit found in string (555) |
| libx11 | 3916 | 5988 | 0 | - | - | BitmapFileInvalid (5840), BitmapNoMemory (104), DimensionsOutsideHarnessLimit (44) |
| gdk-file | 3834 | 6070 | 0 | - | - | gdk (6070) |
| gdk-forced-whole | 4071 | 5833 | 0 | - | - | gdk (5833) |
| gdk-forced-byte | 4071 | 5833 | 0 | - | - | gdk (5833) |
| imagemagick/netpbm | unavailable: no convert/xbmtopbm CLI installed; PyPI (Pillow) and distro packages are outside the allowed network | | | | | |
| gdk-auto-detect | unavailable: GDK has no content magic for XBM: loader auto-detection rejects even valid XBM; only filename (gdk-file) or forced type decodes. Excluded to avoid a constant outlier. | | | | | |

A caught Rust panic is reported as a rejection (`rust_panic`), not as a crash. Independence groups (decoders in one group share code and are not independent votes): {glycin, image-extras}; {xbm-crate}; {libx11}; {gdk-file, gdk-forced-whole, gdk-forced-byte}.

### Votes per independence group

A group votes accept when a strict majority of its members accept. "Alone accepts" = this group accepts while at least two other groups reject (needs 3 or more groups).

| Group | alone accepts | alone rejects | intra-group accept/reject disagreements |
| --- | ---: | ---: | ---: |
| image-extras | 4 | 132 | 0 |
| xbm-crate | 33 | 124 | 0 |
| libX11 | 0 | 0 | 0 |
| gdk-pixbuf | 155 | 0 | 237 |

## Pairwise disagreement (inputs where both ran without crash)

| Pair | comparable | accept/reject differs | both accept, pixels differ |
| --- | ---: | ---: | ---: |
| gdk-file vs gdk-forced-byte | 9904 | 237 | 0 |
| gdk-file vs gdk-forced-whole | 9904 | 237 | 0 |
| gdk-forced-whole vs gdk-forced-byte | 9904 | 0 | 0 |
| glycin vs gdk-file | 9904 | 3429 | 0 |
| glycin vs gdk-forced-byte | 9904 | 3666 | 0 |
| glycin vs gdk-forced-whole | 9904 | 3666 | 0 |
| glycin vs image-extras | 9904 | 0 | 0 |
| glycin vs libx11 | 9904 | 3511 | 0 |
| glycin vs xbm-crate | 9904 | 293 | 0 |
| image-extras vs gdk-file | 9904 | 3429 | 0 |
| image-extras vs gdk-forced-byte | 9904 | 3666 | 0 |
| image-extras vs gdk-forced-whole | 9904 | 3666 | 0 |
| image-extras vs libx11 | 9904 | 3511 | 0 |
| image-extras vs xbm-crate | 9904 | 293 | 0 |
| libx11 vs gdk-file | 9904 | 392 | 322 |
| libx11 vs gdk-forced-byte | 9904 | 155 | 322 |
| libx11 vs gdk-forced-whole | 9904 | 155 | 322 |
| xbm-crate vs gdk-file | 9904 | 3450 | 0 |
| xbm-crate vs gdk-forced-byte | 9904 | 3687 | 0 |
| xbm-crate vs gdk-forced-whole | 9904 | 3687 | 0 |
| xbm-crate vs libx11 | 9904 | 3532 | 0 |

## Pixel disagreement among decoders that accept (322 inputs)

Same bytes, two or more decoders accept, RGBA8 (or dimensions) differ. The partition shows which groups produced identical output (`||` separates output classes). These inputs are often classed `accept-vs-reject` overall because a third decoder rejects.

| Partition | inputs | top clusters | witness | pixel difference | glycin | image-extras | xbm-crate | libx11 | gdk-file | gdk-forced-whole | gdk-forced-byte |
| --- | ---: | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| gdk-pixbuf || libX11 | 322 | byte-flip (34), token-duplicate (17), bad-char-replace (15) | [1](witnesses/98-pixels-gdk-pixbuf-libX11.xbm) | gdk-file vs libx11: 15/91 px; gdk-forced-whole vs libx11: 15/91 px; gdk-forced-byte vs libx11: 15/91 px | rej `ExpectedError` | rej `Format error decodin` | rej `invalid header` | 13x7 `206634` | 13x7 `95c2a4` | 13x7 `95c2a4` | 13x7 `95c2a4` |

## Intra-group disagreement

Members of one library family given the same bytes by different entry points (GDK: filename-sniffed file vs forced module, whole vs byte-wise writes; glycin vs direct image-extras).

| Group | Cluster (inputs) | Witness | glycin | image-extras | xbm-crate | libx11 | gdk-file | gdk-forced-whole | gdk-forced-byte |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| gdk-pixbuf | byte-flip (26) | [464](witnesses/99-intra-gdk-pixbuf-byte-flip.xbm) | rej `ExpectedError` | rej `Format error decodin` | rej `invalid header` | 13x7 `a4449a` | rej `gdk` | 13x7 `a4449a` | 13x7 `a4449a` |
| gdk-pixbuf | token-duplicate (12) | [1087](witnesses/100-intra-gdk-pixbuf-token-duplicate.xbm) | rej `ExpectedError` | rej `Format error decodin` | rej `invalid header` | 13x7 `a4449a` | rej `gdk` | 13x7 `a4449a` | 13x7 `a4449a` |
| gdk-pixbuf | bad-char-replace (10) | [1328](witnesses/101-intra-gdk-pixbuf-bad-char-replace.xbm) | rej `ExpectedError` | rej `Format error decodin` | rej `invalid header` | 8x7 `e47edc` | rej `gdk` | 8x7 `e47edc` | 8x7 `e47edc` |

## Representative matrix (most frequent outcome per mutation rule)

Cluster = mutation rule, split by mutated value when the rule has at most 40 distinct values. Cell: `WxH` (a = some alpha<255) and the first six hex digits of the RGBA8 SHA-256, or `rej` + error class. RGB under alpha=0 is zeroed before hashing.

| Rule (inputs; patterns) | Witness | class | glycin | image-extras | xbm-crate | libx11 | gdk-file | gdk-forced-whole | gdk-forced-byte |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| byte-flip (197; 93) | [441](witnesses/01-byte-flip.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `stream did not conta` | 1x7 `918050` | 1x7 `918050` | 1x7 `918050` | 1x7 `918050` |
| token-duplicate (134; 54) | [1024](witnesses/02-token-duplicate.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid hex byte `0x` | 13x7 `22aa90` | 13x7 `22aa90` | 13x7 `22aa90` | 13x7 `22aa90` |
| bad-char-replace (108; 71) | [1276](witnesses/03-bad-char-replace.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `stream did not conta` | 13x7 `ab35de` | 13x7 `ab35de` | 13x7 `ab35de` | 13x7 `ab35de` |
| bad-char-insert (99; 66) | [2002](witnesses/04-bad-char-insert.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid header` | 13x7 `186bd7` | 13x7 `186bd7` | 13x7 `186bd7` | 13x7 `186bd7` |
| long-run (80; 75) | [1365](witnesses/05-long-run.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid hex byte `0x` | 13x7 `ab35de` | 13x7 `ab35de` | 13x7 `ab35de` | 13x7 `ab35de` |
| truncate (57; 50) | [6538](witnesses/06-truncate.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid termination ` | 13x7 `cfaff8` | 13x7 `cfaff8` | 13x7 `cfaff8` | 13x7 `cfaff8` |
| newlines=double (54; 54) | [2692](witnesses/07-newlines-double.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid header` | 13x7 `4505ee` | 13x7 `4505ee` | 13x7 `4505ee` | 13x7 `4505ee` |
| newlines=crlf (53; 53) | [668](witnesses/08-newlines-crlf.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 13x7 `22aa90` | 13x7 `22aa90` | 13x7 `22aa90` | 13x7 `22aa90` | 13x7 `22aa90` |
| token-replace (47; 45) | [2686](witnesses/09-token-replace.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid digit found ` | 13x7 `b0247d` | 13x7 `b0247d` | 13x7 `b0247d` | 13x7 `b0247d` |
| syntax:trailing-define=define after data (43; 43) | [289](witnesses/10-syntax-trailing-define-define-after-data.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid hex byte `};` | 13x7 `4505ee` | 13x7 `4505ee` | 13x7 `4505ee` | 13x7 `4505ee` |
| syntax:leading-blank=blank lines (42; 42) | [290](witnesses/11-syntax-leading-blank-blank-lines.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid header` | 13x7 `149794` | 13x7 `149794` | 13x7 `149794` | 13x7 `149794` |
| syntax:extra-define=extra-define (42; 42) | [1023](witnesses/12-syntax-extra-define-extra-define.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid header` | 13x7 `186bd7` | 13x7 `186bd7` | 13x7 `186bd7` | 13x7 `186bd7` |
| syntax:ifndef=ifndef (39; 39) | [267](witnesses/13-syntax-ifndef-ifndef.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid header` | 13x7 `b0bd8b` | 13x7 `b0bd8b` | 13x7 `b0bd8b` | 13x7 `b0bd8b` |
| syntax:no-open-brace=no-open-brace (38; 38) | [281](witnesses/14-syntax-no-open-brace-no-open-brace.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid header` | 13x7 `1b18de` | 13x7 `1b18de` | 13x7 `1b18de` | 13x7 `1b18de` |
| syntax:no-close-brace=no-close-brace (38; 38) | [313](witnesses/15-syntax-no-close-brace-no-close-brace.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid header` | 13x7 `6c6c02` | 13x7 `6c6c02` | 13x7 `6c6c02` | 13x7 `6c6c02` |
| data-extra=2 (37; 37) | [254](witnesses/16-data-extra-2.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid hex byte ``` | 13x7 `c0819f` | 13x7 `c0819f` | 13x7 `c0819f` | 13x7 `c0819f` |
| syntax:no-semicolon=no-semicolon (37; 37) | [1021](witnesses/17-syntax-no-semicolon-no-semicolon.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid termination ` | 13x7 `548e0e` | 13x7 `548e0e` | 13x7 `548e0e` | 13x7 `548e0e` |
| data-token=0xfffff (36; 36) | [1617](witnesses/18-data-token-0xfffff.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid hex byte `0x` | 13x7 `4d7644` | 13x7 `4d7644` | 13x7 `4d7644` | 13x7 `4d7644` |
| token-swap-next (36; 25) | [1808](witnesses/19-token-swap-next.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid hex byte ``` | 13x7 `a716df` | 13x7 `a716df` | 13x7 `a716df` | 13x7 `a716df` |
| data-token=0x0g (35; 35) | [178](witnesses/20-data-token-0x0g.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid digit found ` | 13x7 `2cacde` | 13x7 `2cacde` | 13x7 `2cacde` | 13x7 `2cacde` |
| syntax:doubled=file twice (35; 35) | [1002](witnesses/21-syntax-doubled-file-twice.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid hex byte `0X` | 13x7 `149794` | 13x7 `149794` | 13x7 `149794` | 13x7 `149794` |
| syntax:trailing-nul=nul (34; 34) | [2541](witnesses/22-syntax-trailing-nul-nul.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid hex byte `};` | 13x7 `30e037` | 13x7 `30e037` | 13x7 `30e037` | 13x7 `30e037` |
| data-extra=50 (33; 33) | [174](witnesses/23-data-extra-50.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid hex byte ``` | 1x7 `918050` | 1x7 `918050` | 1x7 `918050` | 1x7 `918050` |
| data-token=0xA (33; 33) | [236](witnesses/24-data-token-0xA.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid hex byte `0x` | 100x7 `d11c8b` | 100x7 `d11c8b` | 100x7 `d11c8b` | 100x7 `d11c8b` |
| syntax:no-equals=no-equals (33; 33) | [325](witnesses/25-syntax-no-equals-no-equals.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid header` | 100x7 `56c94e` | 100x7 `56c94e` | 100x7 `56c94e` | 100x7 `56c94e` |
| data-lead-comma=lead-comma (33; 33) | [907](witnesses/26-data-lead-comma-lead-comma.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 13x7 `a1ff8e` | 13x7 `a1ff8e` | 13x7 `a1ff8e` | 13x7 `a1ff8e` | 13x7 `a1ff8e` |
| syntax:no-unsigned=no-unsigned (33; 33) | [1029](witnesses/27-syntax-no-unsigned-no-unsigned.xbm) | accept-vs-reject | 1x7 `cbcfde` | 1x7 `cbcfde` | rej `invalid header` | 1x7 `cbcfde` | 1x7 `cbcfde` | 1x7 `cbcfde` | 1x7 `cbcfde` |
| data-token="a" (32; 32) | [201](witnesses/28-data-token-a-.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid hex byte `0X` | 13x7 `200193` | 13x7 `200193` | 13x7 `200193` | 13x7 `200193` |
| data-token=-1 (32; 32) | [225](witnesses/29-data-token--1.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid hex byte `-1` | 13x7 `ee3d1e` | 13x7 `ee3d1e` | 13x7 `ee3d1e` | 13x7 `ee3d1e` |
| data-token=a (32; 32) | [265](witnesses/30-data-token-a.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid hex byte `a`` | 13x7 `0af247` | 13x7 `0af247` | 13x7 `0af247` | 13x7 `0af247` |
| syntax:trailing-garbage=garbage (32; 32) | [1041](witnesses/31-syntax-trailing-garbage-garbage.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid hex byte `};` | 13x33 `8b8d40` | 13x33 `8b8d40` | 13x33 `8b8d40` | 13x33 `8b8d40` |
| data-extra=1 (31; 31) | [134](witnesses/32-data-extra-1.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid hex byte ``` | 13x7 `22aa90` | 13x7 `22aa90` | 13x7 `22aa90` | 13x7 `22aa90` |
| data-token=256 (31; 31) | [237](witnesses/33-data-token-256.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid hex byte `25` | 13x1 `9d978d` | 13x1 `9d978d` | 13x1 `9d978d` | 13x1 `9d978d` |
| data-extra=12 (31; 31) | [1615](witnesses/34-data-extra-12.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid hex byte ``` | 13x7 `5c1180` | 13x7 `5c1180` | 13x7 `5c1180` | 13x7 `5c1180` |
| data-token=0x (30; 30) | [180](witnesses/35-data-token-0x.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid hex byte `0x` | 13x7 `a26cd0` | 13x7 `a26cd0` | 13x7 `a26cd0` | 13x7 `a26cd0` |
| data-comment-inside=comment-inside (30; 30) | [206](witnesses/36-data-comment-inside-comment-inside.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid hex byte `/*` | 13x7 `9219f3` | 13x7 `9219f3` | 13x7 `9219f3` | 13x7 `9219f3` |
| data-double-commas=double-commas (30; 30) | [846](witnesses/37-data-double-commas-double-commas.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid hex byte ``` | 13x7 `22aa90` | 13x7 `22aa90` | 13x7 `22aa90` | 13x7 `22aa90` |
| data-token=08 (29; 29) | [216](witnesses/38-data-token-08.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid hex byte `08` | 13x7 `1903ef` | 13x7 `1903ef` | 13x7 `1903ef` | 13x7 `1903ef` |
| data-token=0x1ff (29; 29) | [2463](witnesses/39-data-token-0x1ff.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid hex byte `0x` | 13x2 `a49cc0` | 13x2 `a49cc0` | 13x2 `a49cc0` | 13x2 `a49cc0` |
| data-token=1.5 (28; 28) | [188](witnesses/40-data-token-1.5.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid header` | 13x7 `91ab55` | 13x7 `91ab55` | 13x7 `91ab55` | 13x7 `91ab55` |
| data-token='a' (28; 28) | [212](witnesses/41-data-token-a-.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid hex byte `'a` | 13x7 `676336` | 13x7 `676336` | 13x7 `676336` | 13x7 `676336` |
| data-no-commas=no-commas (28; 28) | [229](witnesses/42-data-no-commas-no-commas.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid hex byte `0x` | 7x7 `b4acec` | 7x7 `b4acec` | 7x7 `b4acec` | 7x7 `b4acec` |
| data-token=07 (28; 28) | [241](witnesses/43-data-token-07.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid hex byte `07` | 13x7 `54252d` | 13x7 `54252d` | 13x7 `54252d` | 13x7 `54252d` |
| data-token=0 (27; 27) | [191](witnesses/44-data-token-0.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid hex byte `0`` | 13x7 `2c00ec` | 13x7 `2c00ec` | 13x7 `2c00ec` | 13x7 `2c00ec` |
| token-delete (27; 18) | [388](witnesses/45-token-delete.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 13x7 `9c33bc` | 13x7 `9c33bc` | 13x7 `9c33bc` | 13x7 `9c33bc` | 13x7 `9c33bc` |
| data-token=0xabc (27; 27) | [921](witnesses/46-data-token-0xabc.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid hex byte `0x` | 13x7 `adaa4d` | 13x7 `adaa4d` | 13x7 `adaa4d` | 13x7 `adaa4d` |
| data-token=0x-1 (26; 26) | [219](witnesses/47-data-token-0x-1.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid digit found ` | 13x7 `71f70c` | 13x7 `71f70c` | 13x7 `71f70c` | 13x7 `71f70c` |
| data-uppercase-hex=uppercase-hex (26; 26) | [231](witnesses/48-data-uppercase-hex-uppercase-hex.xbm) | accept-vs-reject | 9x7 `6faf42` | 9x7 `6faf42` | rej `invalid hex byte `0X` | 9x7 `6faf42` | 9x7 `6faf42` | 9x7 `6faf42` | 9x7 `6faf42` |
| data-token=0x100 (25; 25) | [192](witnesses/49-data-token-0x100.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid hex byte `0x` | 13x7 `81e13b` | 13x7 `81e13b` | 13x7 `81e13b` | 13x7 `81e13b` |
| num-boundary:width=7 (25; 25) | [847](witnesses/50-num-boundary-width-7.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid header` | 7x7 `8404ec` | 7x7 `8404ec` | 7x7 `8404ec` | 7x7 `8404ec` |
| data-extra=3 (24; 24) | [215](witnesses/51-data-extra-3.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid hex byte ``` | 13x7 `9aad8f` | 13x7 `9aad8f` | 13x7 `9aad8f` | 13x7 `9aad8f` |
| data-token=0xZZ (24; 24) | [918](witnesses/52-data-token-0xZZ.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid digit found ` | 13x7 `13f95c` | 13x7 `13f95c` | 13x7 `13f95c` | 13x7 `13f95c` |
| num-boundary:width=2 (17; 17) | [104](witnesses/53-num-boundary-width-2.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `rust_panic` | 2x7 `33b0ee` | 2x7 `33b0ee` | 2x7 `33b0ee` | 2x7 `33b0ee` |
| num-boundary:height=1 (17; 17) | [117](witnesses/54-num-boundary-height-1.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `rust_panic` | 13x1 `3053b9` | 13x1 `3053b9` | 13x1 `3053b9` | 13x1 `3053b9` |
| num-boundary:width=1 (15; 13) | [8241](witnesses/55-num-boundary-width-1.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `rust_panic` | 1x7 `c94959` | 1x7 `c94959` | 1x7 `c94959` | 1x7 `c94959` |
| num-boundary:height=2 (13; 13) | [2371](witnesses/56-num-boundary-height-2.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `rust_panic` | 64x2 `694b32` | 64x2 `694b32` | 64x2 `694b32` | 64x2 `694b32` |
| num-boundary:width=8 (12; 12) | [162](witnesses/57-num-boundary-width-8.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid header` | 8x7 `e731f8` | 8x7 `e731f8` | 8x7 `e731f8` | 8x7 `e731f8` |
| num-format:width=8 8 (11; 11) | [168](witnesses/58-num-format-width-8-8.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid header` | 8x7 `a10789` | 8x7 `a10789` | 8x7 `a10789` | 8x7 `a10789` |
| num-format:width=8L (8; 8) | [136](witnesses/59-num-format-width-8L.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid digit found ` | 8x7 `7510a1` | 8x7 `7510a1` | 8x7 `7510a1` | 8x7 `7510a1` |
| num-format:width=+8 (6; 6) | [99](witnesses/60-num-format-width-8.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid header` | 8x7 `3fb343` | 8x7 `3fb343` | 8x7 `3fb343` | 8x7 `3fb343` |
| data-semicolons=semicolons (6; 6) | [969](witnesses/61-data-semicolons-semicolons.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid hex byte `0x` | 13x7 `c27e84` | 13x7 `c27e84` | 13x7 `c27e84` | 13x7 `c27e84` |
| num-format:width=010 (6; 6) | [3041](witnesses/62-num-format-width-010.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 10x7 `e993aa` | 10x7 `e993aa` | 10x7 `e993aa` | 10x7 `e993aa` | 10x7 `e993aa` |
| num-format:width=8.0 (5; 5) | [138](witnesses/63-num-format-width-8.0.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid digit found ` | 8x7 `c86d8e` | 8x7 `c86d8e` | 8x7 `c86d8e` | 8x7 `c86d8e` |
| syntax:hot-rename=hot-rename (5; 5) | [282](witnesses/64-syntax-hot-rename-hot-rename.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid header` | 13x7 `d3d790` | 13x7 `d3d790` | 13x7 `d3d790` | 13x7 `d3d790` |
| num-format:width=8, (5; 5) | [829](witnesses/65-num-format-width-8-.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid digit found ` | 8x7 `316ac6` | 8x7 `316ac6` | 8x7 `316ac6` | 8x7 `316ac6` |
| num-format:width=1e1 (5; 5) | [1589](witnesses/66-num-format-width-1e1.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid digit found ` | 1x7 `75b082` | 1x7 `75b082` | 1x7 `75b082` | 1x7 `75b082` |
| num-format:height=1e1 (4; 4) | [118](witnesses/67-num-format-height-1e1.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid digit found ` | 13x1 `461835` | 13x1 `461835` | 13x1 `461835` | 13x1 `461835` |
| num-format:width= 8  (4; 4) | [1542](witnesses/68-num-format-width-8-.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `rust_panic` | 8x7 `16f64f` | 8x7 `16f64f` | 8x7 `16f64f` | 8x7 `16f64f` |
| num-format:width=8u (4; 4) | [6158](witnesses/69-num-format-width-8u.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid digit found ` | 8x7 `17c39d` | 8x7 `17c39d` | 8x7 `17c39d` | 8x7 `17c39d` |
| num-format:width=8; (3; 3) | [862](witnesses/70-num-format-width-8-.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid digit found ` | 8x16 `f16cc9` | 8x16 `f16cc9` | 8x16 `f16cc9` | 8x16 `f16cc9` |
| num-format:x_hot=010 (2; 2) | [2301](witnesses/71-num-format-x_hot-010.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 13x7 `c0819f` | 13x7 `c0819f` | 13x7 `c0819f` | 13x7 `c0819f` | 13x7 `c0819f` |
| num-format:y_hot=8L (2; 2) | [2329](witnesses/72-num-format-y_hot-8L.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid digit found ` | 13x7 `d3d790` | 13x7 `d3d790` | 13x7 `d3d790` | 13x7 `d3d790` |
| num-format:width=8_0 (2; 2) | [3702](witnesses/73-num-format-width-8_0.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid digit found ` | 8x7 `ba8a30` | 8x7 `ba8a30` | 8x7 `ba8a30` | 8x7 `ba8a30` |
| num-format:x_hot=+8 (2; 2) | [4259](witnesses/74-num-format-x_hot-8.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 13x7 `c0819f` | 13x7 `c0819f` | 13x7 `c0819f` | 13x7 `c0819f` | 13x7 `c0819f` |
| num-format:y_hot='8' (2; 2) | [4288](witnesses/75-num-format-y_hot-8-.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid digit found ` | 13x7 `b0247d` | 13x7 `b0247d` | 13x7 `b0247d` | 13x7 `b0247d` |
| num-format:y_hot=+8 (2; 2) | [7296](witnesses/76-num-format-y_hot-8.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 13x7 `aa1f8a` | 13x7 `aa1f8a` | 13x7 `aa1f8a` | 13x7 `aa1f8a` | 13x7 `aa1f8a` |
| num-format:x_hot=8, (2; 2) | [7641](witnesses/77-num-format-x_hot-8-.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid digit found ` | 13x7 `c0819f` | 13x7 `c0819f` | 13x7 `c0819f` | 13x7 `c0819f` |
| num-format:y_hot=8, (2; 2) | [8292](witnesses/78-num-format-y_hot-8-.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid digit found ` | 13x7 `d3d790` | 13x7 `d3d790` | 13x7 `d3d790` | 13x7 `d3d790` |
| seed:style='x10' (1; 1) | [1](witnesses/79-seed-style-x10-.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid header` | 13x7 `206634` | 13x7 `95c2a4` | 13x7 `95c2a4` | 13x7 `95c2a4` |
| seed:ctype='const unsigned char' (1; 1) | [3](witnesses/80-seed-ctype-const-unsigned-char-.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid header` | rej `BitmapFileInvalid` | 13x7 `accdd0` | 13x7 `accdd0` | 13x7 `accdd0` |
| seed:ctype='const char' (1; 1) | [4](witnesses/81-seed-ctype-const-char-.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `invalid header` | rej `BitmapFileInvalid` | 13x7 `aea783` | 13x7 `aea783` | 13x7 `aea783` |
| seed:static='static  ' (1; 1) | [10](witnesses/82-seed-static-static-.xbm) | accept-vs-reject | 13x7 `9b80dc` | 13x7 `9b80dc` | rej `invalid header` | 13x7 `9b80dc` | 13x7 `9b80dc` | 13x7 `9b80dc` | 13x7 `9b80dc` |
| seed:nl='\r\n' (1; 1) | [11](witnesses/83-seed-nl-r-n-.xbm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 13x7 `062557` | 13x7 `062557` | 13x7 `062557` | 13x7 `062557` | 13x7 `062557` |
| seed:hot=(-1, -1) (1; 1) | [16](witnesses/84-seed-hot--1--1-.xbm) | accept-vs-reject | 13x7 `b0247d` | 13x7 `b0247d` | rej `invalid digit found ` | 13x7 `b0247d` | 13x7 `b0247d` | 13x7 `b0247d` | 13x7 `b0247d` |

## Single-decoder outliers

An independence group that alone accepts (or alone rejects) while at least two other groups disagree with it, grouped by cluster; witnesses are minimised preserving every decoder outcome.

| Group | Kind | Cluster (inputs) | Witness | glycin | image-extras | xbm-crate | libx11 | gdk-file | gdk-forced-whole | gdk-forced-byte |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| image-extras | only-rejects | newlines=crlf (45) | [668](witnesses/85-image-extras-only-rejects-newlines-crlf.xbm) | rej `ExpectedError` | rej `Format error decodin` | 13x7 `22aa90` | 13x7 `22aa90` | 13x7 `22aa90` | 13x7 `22aa90` | 13x7 `22aa90` |
| xbm-crate | only-rejects | syntax:no-unsigned=no-unsigned (36) | [1029](witnesses/86-xbm-crate-only-rejects-syntax-no-unsigned-no-uns.xbm) | 1x7 `cbcfde` | 1x7 `cbcfde` | rej `invalid header` | 1x7 `cbcfde` | 1x7 `cbcfde` | 1x7 `cbcfde` | 1x7 `cbcfde` |
| image-extras | only-rejects | data-lead-comma=lead-comma (30) | [907](witnesses/87-image-extras-only-rejects-data-lead-comma-lead-c.xbm) | rej `ExpectedError` | rej `Format error decodin` | 13x7 `a1ff8e` | 13x7 `a1ff8e` | 13x7 `a1ff8e` | 13x7 `a1ff8e` | 13x7 `a1ff8e` |
| xbm-crate | only-rejects | data-uppercase-hex=uppercase-hex (28) | [231](witnesses/88-xbm-crate-only-rejects-data-uppercase-hex-upperc.xbm) | 9x7 `6faf42` | 9x7 `6faf42` | rej `invalid hex byte `0X` | 9x7 `6faf42` | 9x7 `6faf42` | 9x7 `6faf42` | 9x7 `6faf42` |
| image-extras | only-rejects | bad-char-insert (16) | [4383](witnesses/89-image-extras-only-rejects-bad-char-insert.xbm) | rej `ExpectedError` | rej `Format error decodin` | 13x7 `4505ee` | 13x7 `4505ee` | 13x7 `4505ee` | 13x7 `4505ee` | 13x7 `4505ee` |
| xbm-crate | only-accepts | long-run (14) | [636](witnesses/90-xbm-crate-only-accepts-long-run.xbm) | rej `ExpectedError` | rej `Format error decodin` | 13x7 `5c1180` | rej `BitmapFileInvalid` | rej `gdk` | rej `gdk` | rej `gdk` |
| gdk-pixbuf | only-accepts | byte-flip (13) | [448](witnesses/91-gdk-pixbuf-only-accepts-byte-flip.xbm) | rej `ExpectedError` | rej `Format error decodin` | rej `invalid header` | rej `BitmapFileInvalid` | 13x7 `accdd0` | 13x7 `accdd0` | 13x7 `accdd0` |
| xbm-crate | only-rejects | long-run (11) | [2075](witnesses/92-xbm-crate-only-rejects-long-run.xbm) | 13x7 `a716df` | 13x7 `a716df` | rej `invalid hex byte `};` | 13x7 `a716df` | 13x7 `a716df` | 13x7 `a716df` | 13x7 `a716df` |
| gdk-pixbuf | only-accepts | token-duplicate (9) | [1071](witnesses/93-gdk-pixbuf-only-accepts-token-duplicate.xbm) | rej `ExpectedError` | rej `Format error decodin` | rej `invalid header` | rej `BitmapFileInvalid` | 13x7 `accdd0` | 13x7 `accdd0` | 13x7 `accdd0` |
| gdk-pixbuf | only-accepts | truncate (7) | [7568](witnesses/94-gdk-pixbuf-only-accepts-truncate.xbm) | rej `ExpectedError` | rej `Format error decodin` | rej `invalid header` | rej `BitmapFileInvalid` | 13x7 `accdd0` | 13x7 `accdd0` | 13x7 `accdd0` |
| image-extras | only-accepts | long-run (4) | [1349](witnesses/95-image-extras-only-accepts-long-run.xbm) | 13x7 `1b18de` | 13x7 `1b18de` | rej `invalid header` | rej `BitmapFileInvalid` | rej `gdk` | rej `gdk` | rej `gdk` |
| xbm-crate | only-accepts | newlines=crlf (2) | [760](witnesses/96-xbm-crate-only-accepts-newlines-crlf.xbm) | rej `ExpectedError` | rej `Format error decodin` | 13x7 `205f04` | rej `BitmapFileInvalid` | rej `gdk` | rej `gdk` | rej `gdk` |
| xbm-crate | only-accepts | bad-char-insert (2) | [4420](witnesses/97-xbm-crate-only-accepts-bad-char-insert.xbm) | rej `ExpectedError` | rej `Format error decodin` | 13x1 `365fd5` | rej `BitmapFileInvalid` | rej `gdk` | rej `gdk` | rej `gdk` |

## Ten most frequent outlier behaviours

- `image-extras` only rejects on rule `newlines=crlf` (45 inputs): [witnesses/85-image-extras-only-rejects-newlines-crlf.xbm](witnesses/85-image-extras-only-rejects-newlines-crlf.xbm)
- `xbm-crate` only rejects on rule `syntax:no-unsigned=no-unsigned` (36 inputs): [witnesses/86-xbm-crate-only-rejects-syntax-no-unsigned-no-uns.xbm](witnesses/86-xbm-crate-only-rejects-syntax-no-unsigned-no-uns.xbm)
- `image-extras` only rejects on rule `data-lead-comma=lead-comma` (30 inputs): [witnesses/87-image-extras-only-rejects-data-lead-comma-lead-c.xbm](witnesses/87-image-extras-only-rejects-data-lead-comma-lead-c.xbm)
- `xbm-crate` only rejects on rule `data-uppercase-hex=uppercase-hex` (28 inputs): [witnesses/88-xbm-crate-only-rejects-data-uppercase-hex-upperc.xbm](witnesses/88-xbm-crate-only-rejects-data-uppercase-hex-upperc.xbm)
- `xbm-crate` only accepts on rule `long-run` (14 inputs): [witnesses/90-xbm-crate-only-accepts-long-run.xbm](witnesses/90-xbm-crate-only-accepts-long-run.xbm)
- `gdk-pixbuf` only accepts on rule `byte-flip` (13 inputs): [witnesses/91-gdk-pixbuf-only-accepts-byte-flip.xbm](witnesses/91-gdk-pixbuf-only-accepts-byte-flip.xbm)
- `gdk-pixbuf` only accepts on rule `token-duplicate` (9 inputs): [witnesses/93-gdk-pixbuf-only-accepts-token-duplicate.xbm](witnesses/93-gdk-pixbuf-only-accepts-token-duplicate.xbm)
- `image-extras` only accepts on rule `long-run` (4 inputs): [witnesses/95-image-extras-only-accepts-long-run.xbm](witnesses/95-image-extras-only-accepts-long-run.xbm)
- `xbm-crate` only accepts on rule `newlines=crlf` (2 inputs): [witnesses/96-xbm-crate-only-accepts-newlines-crlf.xbm](witnesses/96-xbm-crate-only-accepts-newlines-crlf.xbm)

## Ten leniency findings (XBM, seed 731)

Votes: image-extras (which glycin's in-tree path wraps), the `xbm` crate, libX11's `XReadBitmapFileData`, and GDK-Pixbuf (file, forced-whole and forced-byte entry points). Counts are inputs out of 10,000; "accepts" means the decoder returned pixels.

1. **libX11 and GDK accept roughly 3,900 inputs; image-extras/glycin accept 415 and the `xbm` crate 452.** Both of the latter are strict C-syntax parsers; libX11 and GDK scan for the `_width`/`_height` defines and the `static ... {` line and then read whatever hex-like tokens follow. Glycin and image-extras never disagree (0 of 9,914 comparable inputs).
2. **Excess or malformed data tokens are ignored by libX11 and GDK and rejected by image-extras and the `xbm` crate.** Examples that libX11 and GDK decode with identical pixels (the `xbm` crate panics on extra data entries, recorded as a rejection): 1 to 50 extra `0xff` entries after the array, a token replaced by `a`, `"a"`, `1.5`, `0x0g`, `0xabc`, `0x1ff`, `-1`, a `/* c */` comment inside the array, doubled commas, `;` after the array is missing, `}` is missing, a trailing `#define` after the data, a doubled file.
3. **Numeric defines are parsed by prefix in libX11 and GDK (`sscanf`-style), by whole token elsewhere.** `#define img_width 8u`, `8L`, `8.0`, `1e1` (width 1), `8 8` and ` 8 ` decode in libX11 and GDK and are rejected by image-extras and the `xbm` crate. `0x8`, `0X8`, `0b1000`, `'8'`, `-0` and a full-width digit are rejected by all four. `010` is accepted by the `xbm` crate, libX11 and GDK but rejected by image-extras; `+8` is accepted by libX11 and GDK, rejected by image-extras, and makes the `xbm` crate panic (recorded as a rejection) or accept depending on the seed.
4. **X10 `short` arrays decode only in libX11 and GDK, and they disagree on pixels.** In the 13x7 X10 seed, GDK output differs from libX11 in 15 of 91 pixels. This is the only pixel-level disagreement among accepting decoders in the run (322 inputs, all of the same libX11-versus-GDK shape, mostly from byte flips and token edits of X10-style or `0x%04x` seeds). image-extras and the `xbm` crate reject the X10 form.
5. **`static` is mandatory in every decoder.** libX11 matches the line with a fixed `static unsigned char NAME = {` / `static char` / `static short` pattern, so `const static`, `extern`, no `static`, `unsigned short` and `signed char` are rejected by all of them. GDK additionally accepts `static const char` / `static const unsigned char`, which libX11, image-extras and the `xbm` crate reject.
6. **Leading comments split the decoders three ways.** A `/* ... */` or `//` line before the first `#define` is accepted by libX11, by GDK's forced-type loaders, and rejected by GDK's filename-sniffing `gdk_pixbuf_new_from_file`, by image-extras and by the `xbm` crate. GDK file and forced loaders disagree on accept/reject for 237 inputs. GDK content auto-detection never identifies XBM (no magic), so only filename or forced type decode.
7. **Define order and naming.** Height before width is rejected by image-extras and the `xbm` crate, accepted by libX11 and GDK. An empty name prefix (`#define width 13`) is accepted only by libX11 and GDK. `a-b` and `1x` prefixes are accepted by image-extras, libX11 and GDK and rejected by the `xbm` crate. A 300-character prefix is accepted only by the `xbm` crate (libX11 and GDK reject it, presumably a line-length limit).
8. **Line endings.** CRLF files are accepted by the `xbm` crate, libX11 and GDK and rejected by image-extras (and so glycin). Bare CR line endings are rejected by every decoder, and so is a file with every newline replaced by a space: all four parsers are line-oriented in the header.
9. **Case and format of hex values.** `0X1F` is rejected by the `xbm` crate only. Decimal and octal array entries (`%d`, `0%o`) are accepted by libX11 and GDK and rejected by image-extras and the `xbm` crate. A leading comma in the array is accepted by libX11 and GDK and rejected by image-extras; the `xbm` crate accepts it for about half the seeds (30 of 56 mutated seeds) and rejects it for the rest.
10. **Hotspot values never change the pixels**, but a negative hotspot (`-1`) is accepted by image-extras, libX11 and GDK and rejected by the `xbm` crate (`invalid digit`). The glycin path does not expose the hotspot at all (metadata only, excluded from the pixel hash).

Counts only (not analysed): crash or timeout in at least one decoder on 96 inputs: glycin 86, GDK 31 to 34 per entry point (all status `crash`, reproduced on a serial rerun with a 10 s limit). The `xbm` crate returned a caught panic on 125 inputs, which the matrix records as a rejection.

## Run

`./run.sh --inputs 10000 --seed 731 --jobs 6` from `/__modal/volumes/vo-Iyh3VLkOB7cEHrIp2RkTs7/projects/leniency/xbm`; outputs in `/tmp/claude-0/leniency-xbm`.
