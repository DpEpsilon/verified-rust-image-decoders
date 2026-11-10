# XPM leniency matrix (public subset)

Seed 731; 9985 synthetic inputs (105 valid/unmutated seeds from 105 seed files; 6910 mutations changed the acceptance/pixel-equality pattern relative to their seed); 270.1 s for the comparison pass. Inputs whose run involved a crash or timeout are excluded from every count and row below.

Classification | Count
--- | ---:
all-accept-identical-pixels | 365
accept-different-pixels | 381
accept-vs-reject | 5818
all-reject | 3421
crash-or-timeout | 0

## Decoders

| Decoder | ok | error | of which caught Rust panic | crash | timeout | top error classes |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| glycin | 960 | 9025 | 16 | - | - | ExpectedError (9009), rust_panic (16) |
| image-extras | 960 | 9025 | 16 | - | - | Format error decoding `XPM` (8991), output dimensions outside harness limit (18), rust_panic (16) |
| ez-pixmap | 3512 | 6473 | 491 | - | - | Internal (4691), rust_panic (491), NoStrings (452) |
| gdk-file | 5473 | 4512 | 0 | - | - | gdk (4512) |
| gdk-auto-whole | 5248 | 4737 | 0 | - | - | gdk (4737) |
| gdk-forced-whole | 6010 | 3975 | 0 | - | - | gdk (3975) |
| gdk-forced-byte | 6010 | 3975 | 0 | - | - | gdk (3975) |
| libXpm | unavailable: headers/library not installed (no X11/xpm.h, no libXpm.so) and its source host (freedesktop.org) is outside the allowed network; github.com mirror was not accessible | | | | | |
| imagemagick/netpbm | unavailable: no convert/xpmtoppm CLI installed | | | | | |
| sdl-image/imlib2 | unavailable: not installed | | | | | |

A caught Rust panic is reported as a rejection (`rust_panic`), not as a crash. Independence groups (decoders in one group share code and are not independent votes): {glycin, image-extras}; {ez-pixmap}; {gdk-file, gdk-auto-whole, gdk-forced-whole, gdk-forced-byte}.

### Votes per independence group

A group votes accept when a strict majority of its members accept. "Alone accepts" = this group accepts while at least two other groups reject (needs 3 or more groups).

| Group | alone accepts | alone rejects | intra-group accept/reject disagreements |
| --- | ---: | ---: | ---: |
| image-extras | 0 | 1924 | 0 |
| ez-pixmap | 842 | 214 | 0 |
| gdk-pixbuf | 2589 | 0 | 762 |

## Pairwise disagreement (inputs where both ran without crash)

| Pair | comparable | accept/reject differs | both accept, pixels differ |
| --- | ---: | ---: | ---: |
| ez-pixmap vs gdk-auto-whole | 9985 | 3756 | 1342 |
| ez-pixmap vs gdk-file | 9985 | 3645 | 1417 |
| ez-pixmap vs gdk-forced-byte | 9985 | 3606 | 1555 |
| ez-pixmap vs gdk-forced-whole | 9985 | 3606 | 1555 |
| gdk-auto-whole vs gdk-forced-byte | 9985 | 762 | 0 |
| gdk-auto-whole vs gdk-forced-whole | 9985 | 762 | 0 |
| gdk-file vs gdk-auto-whole | 9985 | 225 | 0 |
| gdk-file vs gdk-forced-byte | 9985 | 537 | 0 |
| gdk-file vs gdk-forced-whole | 9985 | 537 | 0 |
| gdk-forced-whole vs gdk-forced-byte | 9985 | 0 | 0 |
| glycin vs ez-pixmap | 9985 | 2980 | 254 |
| glycin vs gdk-auto-whole | 9985 | 4288 | 272 |
| glycin vs gdk-file | 9985 | 4513 | 272 |
| glycin vs gdk-forced-byte | 9985 | 5050 | 272 |
| glycin vs gdk-forced-whole | 9985 | 5050 | 272 |
| glycin vs image-extras | 9985 | 0 | 0 |
| image-extras vs ez-pixmap | 9985 | 2980 | 254 |
| image-extras vs gdk-auto-whole | 9985 | 4288 | 272 |
| image-extras vs gdk-file | 9985 | 4513 | 272 |
| image-extras vs gdk-forced-byte | 9985 | 5050 | 272 |
| image-extras vs gdk-forced-whole | 9985 | 5050 | 272 |

## Pixel disagreement among decoders that accept (1630 inputs)

Same bytes, two or more decoders accept, RGBA8 (or dimensions) differ. The partition shows which groups produced identical output (`||` separates output classes). These inputs are often classed `accept-vs-reject` overall because a third decoder rejects.

| Partition | inputs | top clusters | witness | pixel difference | glycin | image-extras | ez-pixmap | gdk-file | gdk-auto-whole | gdk-forced-whole | gdk-forced-byte |
| --- | ---: | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ez-pixmap || gdk-pixbuf | 1174 | byte-flip (138), bad-char-insert (98), long-run (85) | [23](witnesses/103-pixels-ez-pixmap-gdk-pixbuf.xpm) | gdk-file vs ez-pixmap: 20/20 px; gdk-auto-whole vs ez-pixmap: 20/20 px; gdk-forced-whole vs ez-pixmap: 20/20 px | rej `ExpectedError` | rej `Format error decodin` | 5x4 `b1afc3` | 5x4a `0a3d10` | 5x4a `0a3d10` | 5x4a `0a3d10` | 5x4a `0a3d10` |
| ez-pixmap || gdk-pixbuf+image-extras | 184 | color-value=NONE (12), color-value=red  (11), color-key=c  (10) | [0](witnesses/104-pixels-ez-pixmap-gdk-pixbuf-image-extras.xpm) | ez-pixmap vs glycin: 9/20 px | 5x4a `6011e4` | 5x4a `6011e4` | 5x4a `83c295` | 5x4a `6011e4` | 5x4a `6011e4` | 5x4a `6011e4` | 5x4a `6011e4` |
| ez-pixmap+image-extras || gdk-pixbuf | 127 | newlines=double (9), pixels:row-swap=row-swap (7), color-named=green (6) | [19](witnesses/105-pixels-ez-pixmap-image-extras-gdk-pixbuf.xpm) | gdk-file vs glycin: 13/20 px; gdk-auto-whole vs glycin: 13/20 px; gdk-forced-whole vs glycin: 13/20 px | 5x4 `6a1174` | 5x4 `6a1174` | 5x4 `6a1174` | 5x4 `82de4f` | 5x4 `82de4f` | 5x4 `82de4f` | 5x4 `82de4f` |
| gdk-pixbuf || image-extras | 75 | byte-flip (7), newlines=double (4), color-line:multi-space=multi-space (4) | [7](witnesses/106-pixels-gdk-pixbuf-image-extras.xpm) | gdk-file vs glycin: 8/20 px; gdk-auto-whole vs glycin: 8/20 px; gdk-forced-whole vs glycin: 8/20 px | 5x4a `3fe0c1` | 5x4a `3fe0c1` | rej `Internal` | 5x4a `f306e9` | 5x4a `f306e9` | 5x4a `f306e9` | 5x4a `f306e9` |
| ez-pixmap || gdk-pixbuf || image-extras | 70 | color-line:key-after-none=key-after-none (12), truncate (4), long-run (4) | [14](witnesses/107-pixels-ez-pixmap-gdk-pixbuf-image-extras.xpm) | ez-pixmap vs glycin: 13/20 px; gdk-file vs glycin: 7/20 px; gdk-auto-whole vs glycin: 7/20 px | 5x4a `db82b8` | 5x4a `db82b8` | 5x4 `7e7881` | 5x4a `9ba74b` | 5x4a `9ba74b` | 5x4a `9ba74b` | 5x4a `9ba74b` |

## Intra-group disagreement

Members of one library family given the same bytes by different entry points (GDK: filename-sniffed file vs forced module, whole vs byte-wise writes; glycin vs direct image-extras).

| Group | Cluster (inputs) | Witness | glycin | image-extras | ez-pixmap | gdk-file | gdk-auto-whole | gdk-forced-whole | gdk-forced-byte |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| gdk-pixbuf | bad-char-insert (123) | [685](witnesses/108-intra-gdk-pixbuf-bad-char-insert.xpm) | rej `ExpectedError` | rej `Format error decodin` | 5x4a `ec3e42` | 5x4a `ec3e42` | rej `gdk` | 5x4a `ec3e42` | 5x4a `ec3e42` |
| gdk-pixbuf | long-run (107) | [815](witnesses/109-intra-gdk-pixbuf-long-run.xpm) | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4a `ca42f5` | rej `gdk` | 5x4a `ca42f5` | 5x4a `ca42f5` |
| gdk-pixbuf | byte-flip (60) | [632](witnesses/110-intra-gdk-pixbuf-byte-flip.xpm) | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4a `416c0f` | rej `gdk` | 5x4a `416c0f` | 5x4a `416c0f` |

## Representative matrix (most frequent outcome per mutation rule)

Cluster = mutation rule, split by mutated value when the rule has at most 40 distinct values. Cell: `WxH` (a = some alpha<255) and the first six hex digits of the RGBA8 SHA-256, or `rej` + error class. RGB under alpha=0 is zeroed before hashing.

| Rule (inputs; patterns) | Witness | class | glycin | image-extras | ez-pixmap | gdk-file | gdk-auto-whole | gdk-forced-whole | gdk-forced-byte |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| byte-flip (421; 286) | [769](witnesses/01-byte-flip.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 5x4a `4855ed` | 5x4a `7880f1` | 5x4a `7880f1` | 5x4a `7880f1` | 5x4a `7880f1` |
| bad-char-insert (294; 258) | [737](witnesses/02-bad-char-insert.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `NonUtf8Input` | 5x4a `416c0f` | 5x4a `416c0f` | 5x4a `416c0f` | 5x4a `416c0f` |
| bad-char-replace (226; 179) | [4011](witnesses/03-bad-char-replace.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4a `b2cc88` | 5x4a `b2cc88` | 5x4a `b2cc88` | 5x4a `b2cc88` |
| long-run (194; 179) | [948](witnesses/04-long-run.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4a `d32789` | 5x4a `d32789` | 5x4a `d32789` | 5x4a `d32789` |
| token-duplicate (149; 121) | [607](witnesses/05-token-duplicate.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `rust_panic` | 5x4a `5b6fb5` | 5x4a `5b6fb5` | 5x4a `5b6fb5` | 5x4a `5b6fb5` |
| token-swap-next (132; 119) | [1539](witnesses/06-token-swap-next.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 5x4 `54486c` | 5x4 `54486c` | 5x4 `54486c` | 5x4 `54486c` | 5x4 `54486c` |
| token-delete (99; 81) | [3643](witnesses/07-token-delete.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4a `3c354b` | 5x4a `3c354b` | 5x4a `3c354b` | 5x4a `3c354b` |
| token-replace (96; 80) | [636](witnesses/08-token-replace.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4a `98897f` | 5x4a `98897f` | 5x4a `98897f` | 5x4a `98897f` |
| pixels:row-unknown-code=row-unknown-code (61; 61) | [322](witnesses/09-pixels-row-unknown-code-row-unknown-code.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4a `f64d18` | 5x4a `f64d18` | 5x4a `f64d18` | 5x4a `f64d18` |
| pixels:row-dup=row-dup (61; 61) | [323](witnesses/10-pixels-row-dup-row-dup.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4a `0f265a` | 5x4a `0f265a` | 5x4a `0f265a` | 5x4a `0f265a` |
| newlines=none (58; 58) | [895](witnesses/11-newlines-none.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 5x4a `ec3e42` | 5x4a `ec3e42` | 5x4a `ec3e42` | 5x4a `ec3e42` | 5x4a `ec3e42` |
| newlines=cr (58; 58) | [947](witnesses/12-newlines-cr.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4a `416c0f` | 5x4a `416c0f` | 5x4a `416c0f` | 5x4a `416c0f` |
| newlines=crlf (58; 58) | [972](witnesses/13-newlines-crlf.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 5x4a `854d37` | 5x4a `854d37` | 5x4a `854d37` | 5x4a `854d37` | 5x4a `854d37` |
| truncate (57; 54) | [2646](witnesses/14-truncate.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 5x4a `b2cc88` | 5x4a `b2cc88` | 5x4a `b2cc88` | 5x4a `b2cc88` | 5x4a `b2cc88` |
| pixels:row-extra-at-end=row-extra-at-end (55; 55) | [285](witnesses/15-pixels-row-extra-at-end-row-extra-at-end.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4a `68b07d` | 5x4a `68b07d` | 5x4a `68b07d` | 5x4a `68b07d` |
| pixels:row-backslash=row-backslash (54; 54) | [315](witnesses/16-pixels-row-backslash-row-backslash.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4a `7cf5ec` | 5x4a `7cf5ec` | 5x4a `7cf5ec` | 5x4a `7cf5ec` |
| pixels:row-tab=row-tab (54; 54) | [330](witnesses/17-pixels-row-tab-row-tab.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `rust_panic` | 5x4 `0dbc37` | 5x4 `0dbc37` | 5x4 `0dbc37` | 5x4 `0dbc37` |
| pixels:row-long-1=row-long-1 (53; 53) | [338](witnesses/18-pixels-row-long-1-row-long-1.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4a `0a3d10` | 5x4a `0a3d10` | 5x4a `0a3d10` | 5x4a `0a3d10` |
| pixels:row-long-char=row-long-char (48; 48) | [337](witnesses/19-pixels-row-long-char-row-long-char.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 5x4 `d72108` | 5x4 `4a9603` | 5x4 `4a9603` | 5x4 `4a9603` | 5x4 `4a9603` |
| pixels:row-one-unknown=row-one-unknown (47; 47) | [386](witnesses/20-pixels-row-one-unknown-row-one-unknown.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4a `bd7100` | 5x4a `bd7100` | 5x4a `bd7100` | 5x4a `bd7100` |
| pixels:row-nonascii=row-nonascii (45; 45) | [321](witnesses/21-pixels-row-nonascii-row-nonascii.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4a `70f18c` | 5x4a `70f18c` | 5x4a `70f18c` | 5x4a `70f18c` |
| pixels:row-trailing-space=row-trailing-space (42; 42) | [318](witnesses/22-pixels-row-trailing-space-row-trailing-space.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4a `d32789` | 5x4a `d32789` | 5x4a `d32789` | 5x4a `d32789` |
| syntax:magic=/* XPM 3 */ (32; 32) | [437](witnesses/23-syntax-magic-XPM-3-.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 5x4 `806f22` | rej `gdk` | rej `gdk` | 5x4 `1afc34` | 5x4 `1afc34` |
| syntax:magic=// XPM (32; 32) | [465](witnesses/24-syntax-magic-XPM.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 5x4a `2fcba3` | rej `gdk` | rej `gdk` | 5x4a `346302` | 5x4a `346302` |
| syntax:magic=/*  XPM  */ (31; 31) | [493](witnesses/25-syntax-magic-XPM-.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | rej `gdk` | rej `gdk` | 5x4a `3c354b` | 5x4a `3c354b` |
| syntax:leading-define=define (28; 27) | [5263](witnesses/26-syntax-leading-define-define.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4a `5b6fb5` | rej `gdk` | 5x4a `5b6fb5` | 5x4a `5b6fb5` |
| color-key=g4 (28; 28) | [1274](witnesses/27-color-key-g4.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4a `11e16c` | 5x4a `11e16c` | 5x4a `11e16c` | 5x4a `11e16c` |
| syntax:doubled=twice (27; 27) | [423](witnesses/28-syntax-doubled-twice.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4a `d32789` | 5x4a `d32789` | 5x4a `d32789` | 5x4a `d32789` |
| color-line:sym-only=sym-only (26; 7) | [212](witnesses/29-color-line-sym-only-sym-only.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4a `5b6fb5` | 5x4a `5b6fb5` | 5x4a `5b6fb5` | 5x4a `5b6fb5` |
| syntax:magic=/* XPM2 */ (25; 25) | [447](witnesses/30-syntax-magic-XPM2-.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 5x4a `854d37` | rej `gdk` | rej `gdk` | rej `gdk` | rej `gdk` |
| color-line:code-longer=code-longer (24; 24) | [228](witnesses/31-color-line-code-longer-code-longer.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4a `bf0d93` | 5x4a `bf0d93` | 5x4a `bf0d93` | 5x4a `bf0d93` |
| color-line:code-space=code-space (24; 24) | [237](witnesses/32-color-line-code-space-code-space.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4a `9bb73d` | 5x4a `9bb73d` | 5x4a `9bb73d` | 5x4a `9bb73d` |
| syntax:ext-bad-name=ext-bad-name (24; 24) | [440](witnesses/33-syntax-ext-bad-name-ext-bad-name.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4a `5b6fb5` | 5x4a `5b6fb5` | 5x4a `5b6fb5` | 5x4a `5b6fb5` |
| syntax:ext-without-flag=ext-without-flag (24; 24) | [1485](witnesses/34-syntax-ext-without-flag-ext-without-flag.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `rust_panic` | 5x4 `0dbc37` | 5x4 `0dbc37` | 5x4 `0dbc37` | 5x4 `0dbc37` |
| color-line:mono-only=mono-only (23; 6) | [247](witnesses/35-color-line-mono-only-mono-only.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4 `b1afc3` | 5x4 `b1afc3` | 5x4 `b1afc3` | 5x4 `b1afc3` |
| color-line:dup-code-new-color=dup-code-new-color (22; 22) | [229](witnesses/36-color-line-dup-code-new-color-dup-code-new-color.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4 `ddf38e` | 5x4 `ddf38e` | 5x4 `ddf38e` | 5x4 `ddf38e` |
| syntax:ext-no-end=ext-no-end (21; 21) | [454](witnesses/37-syntax-ext-no-end-ext-no-end.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4a `7880f1` | 5x4a `7880f1` | 5x4a `7880f1` | 5x4a `7880f1` |
| syntax:ext-flag-without-ext=ext-flag-without-ext (21; 21) | [479](witnesses/38-syntax-ext-flag-without-ext-ext-flag-without-ext.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 5x4a `f0077f` | 5x4a `f0077f` | 5x4a `f0077f` | 5x4a `f0077f` | 5x4a `f0077f` |
| color-line:no-value=no-value (20; 20) | [215](witnesses/39-color-line-no-value-no-value.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4a `aa100b` | 5x4a `aa100b` | 5x4a `aa100b` | 5x4a `aa100b` |
| color-key=m (20; 20) | [226](witnesses/40-color-key-m.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4 `39a961` | 5x4 `39a961` | 5x4 `39a961` | 5x4 `39a961` |
| syntax:decl=4 (20; 20) | [390](witnesses/41-syntax-decl-4.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 5x4a `68b07d` | 5x4a `68b07d` | 5x4a `68b07d` | 5x4a `68b07d` | 5x4a `68b07d` |
| syntax:no-magic=no-magic (20; 20) | [458](witnesses/42-syntax-no-magic-no-magic.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 5x4a `b49ecf` | rej `gdk` | rej `gdk` | rej `gdk` | rej `gdk` |
| syntax:magic=/* xpm */ (20; 20) | [1512](witnesses/43-syntax-magic-xpm-.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 5x4a `a9c194` | rej `gdk` | rej `gdk` | rej `gdk` | rej `gdk` |
| syntax:no-trailing=no-trailing (19; 19) | [436](witnesses/44-syntax-no-trailing-no-trailing.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 5x4 `39a961` | 5x4 `39a961` | 5x4 `39a961` | 5x4 `39a961` | 5x4 `39a961` |
| color-key=G4 (18; 17) | [7790](witnesses/45-color-key-G4.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4a `5b6fb5` | 5x4a `5b6fb5` | 5x4a `5b6fb5` | 5x4a `5b6fb5` |
| color-key= (18; 18) | [243](witnesses/46-color-key-.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4a `98280c` | 5x4a `98280c` | 5x4a `98280c` | 5x4a `98280c` |
| color-line:only-code=only-code (18; 17) | [4326](witnesses/47-color-line-only-code-only-code.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `rust_panic` | 5x4a `5b6fb5` | 5x4a `5b6fb5` | 5x4a `5b6fb5` | 5x4a `5b6fb5` |
| syntax:magic= (18; 18) | [445](witnesses/48-syntax-magic-.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 5x4a `e264fb` | rej `gdk` | rej `gdk` | rej `gdk` | rej `gdk` |
| syntax:trailing-garbage=x (18; 18) | [460](witnesses/49-syntax-trailing-garbage-x.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 5x4a `6be749` | 5x4a `6be749` | 5x4a `6be749` | 5x4a `6be749` | 5x4a `6be749` |
| syntax:semicolon-row-sep=semicolon-row-sep (18; 18) | [1369](witnesses/50-syntax-semicolon-row-sep-semicolon-row-sep.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4a `482bb9` | 5x4a `482bb9` | 5x4a `482bb9` | 5x4a `482bb9` |
| syntax:string-concat=adjacent literals (18; 18) | [1473](witnesses/51-syntax-string-concat-adjacent-literals.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4a `d32789` | 5x4a `d32789` | 5x4a `d32789` | 5x4a `d32789` |
| syntax:magic=/*XPM*/ (18; 18) | [2441](witnesses/52-syntax-magic-XPM-.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 5x4a `c14dd7` | rej `gdk` | rej `gdk` | rej `gdk` | rej `gdk` |
| color-key=cc (16; 16) | [213](witnesses/53-color-key-cc.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4a `4a3061` | 5x4a `4a3061` | 5x4a `4a3061` | 5x4a `4a3061` |
| color-value=#+f00 (16; 16) | [304](witnesses/54-color-value-f00.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `ParseError` | 5x4a `7abfcb` | 5x4a `7abfcb` | 5x4a `7abfcb` | 5x4a `7abfcb` |
| syntax:decl=2 (16; 16) | [319](witnesses/55-syntax-decl-2.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4a `482bb9` | 5x4a `482bb9` | 5x4a `482bb9` | 5x4a `482bb9` |
| color-line:multi-space=multi-space (16; 16) | [2376](witnesses/56-color-line-multi-space-multi-space.xpm) | accept-vs-reject | 5x2a `abf9fa` | 5x2a `abf9fa` | rej `Internal` | 5x2a `abf9fa` | 5x2a `abf9fa` | 5x2a `abf9fa` | 5x2a `abf9fa` |
| color-line:code-shorter=code-shorter (15; 15) | [218](witnesses/57-color-line-code-shorter-code-shorter.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4a `50059e` | 5x4a `50059e` | 5x4a `50059e` | 5x4a `50059e` |
| color-key=C (15; 15) | [225](witnesses/58-color-key-C.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `rust_panic` | 5x4a `b9475a` | 5x4a `b9475a` | 5x4a `b9475a` | 5x4a `b9475a` |
| color-key=x (15; 15) | [246](witnesses/59-color-key-x.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4a `04ca72` | 5x4a `04ca72` | 5x4a `04ca72` | 5x4a `04ca72` |
| color-key=g5 (15; 15) | [270](witnesses/60-color-key-g5.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 1x4a `bce718` | 1x4a `bce718` | 1x4a `bce718` | 1x4a `bce718` |
| color-line:key-after-none=key-after-none (15; 15) | [307](witnesses/61-color-line-key-after-none-key-after-none.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 16x4a `090a1d` | 16x4a `b52c32` | 16x4a `b52c32` | 16x4a `b52c32` | 16x4a `b52c32` |
| syntax:decl=5 (15; 15) | [446](witnesses/62-syntax-decl-5.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 5x4a `c14dd7` | 5x4a `c14dd7` | 5x4a `c14dd7` | 5x4a `c14dd7` | 5x4a `c14dd7` |
| color-key=c  (15; 15; ez-pixmap vs glycin: 67/160 px; gdk-file vs glycin: 67/160 px; gdk-auto-whole vs glycin: 67/160 px) | [1150](witnesses/63-color-key-c-.xpm) | accept-different-pixels | 40x4a `eddeb7` | 40x4a `eddeb7` | 40x4a `f68f4c` | 40x4a `29018e` | 40x4a `29018e` | 40x4a `29018e` | 40x4a `29018e` |
| color-key=s (14; 14) | [257](witnesses/64-color-key-s.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | rej `gdk` | rej `gdk` | 5x4a `af4553` | 5x4a `af4553` |
| color-value=# (14; 14) | [314](witnesses/65-color-value-.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `ParseError` | 5x4a `2dd45f` | 5x4a `2dd45f` | 5x4a `2dd45f` | 5x4a `2dd45f` |
| syntax:decl=9 (14; 14) | [420](witnesses/66-syntax-decl-9.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 5x4a `83c295` | 5x4a `6011e4` | 5x4a `6011e4` | 5x4a `6011e4` | 5x4a `6011e4` |
| syntax:no-row-sep=no-row-sep (14; 14) | [1433](witnesses/67-syntax-no-row-sep-no-row-sep.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 5x16a `3e501d` | 5x16a `3e501d` | 5x16a `3e501d` | 5x16a `3e501d` | 5x16a `3e501d` |
| hdr-extra-token=c (13; 13) | [126](witnesses/68-hdr-extra-token-c.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 5x4a `b2cc88` | 5x4a `b2cc88` | 5x4a `b2cc88` | 5x4a `b2cc88` | 5x4a `b2cc88` |
| hdr-extra-token=1 (13; 13) | [142](witnesses/69-hdr-extra-token-1.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 5x4a `2934e2` | rej `gdk` | rej `gdk` | rej `gdk` | rej `gdk` |
| color-line:dup-color-line=dup-color-line (13; 13) | [252](witnesses/70-color-line-dup-color-line-dup-color-line.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4 `0d2a2f` | 5x4 `0d2a2f` | 5x4 `0d2a2f` | 5x4 `0d2a2f` |
| color-value=rgbi:1.0/0/0 (13; 13) | [262](witnesses/71-color-value-rgbi-1.0-0-0.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 5x4a `dbe3f0` | 5x4a `d19e76` | 5x4a `d19e76` | 5x4a `d19e76` | 5x4a `d19e76` |
| color-key=M (13; 13) | [283](witnesses/72-color-key-M.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4a `0ecfe2` | 5x4a `0ecfe2` | 5x4a `0ecfe2` | 5x4a `0ecfe2` |
| color-value=#GGGGGG (13; 13) | [1363](witnesses/73-color-value-GGGGGG.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `ParseError` | rej `gdk` | rej `gdk` | 5x4a `464d20` | 5x4a `464d20` |
| color-line:code-hash=code-hash (13; 13) | [2309](witnesses/74-color-line-code-hash-code-hash.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4a `409f08` | 5x4a `409f08` | 5x4a `409f08` | 5x4a `409f08` |
| hdr-ncolors-off-by=-1 (13; 13) | [3113](witnesses/75-hdr-ncolors-off-by--1.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x16 `5c5f7d` | 5x16 `5c5f7d` | 5x16 `5c5f7d` | 5x16 `5c5f7d` |
| color-value=#ABC (12; 12) | [264](witnesses/76-color-value-ABC.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `rust_panic` | 5x4a `05d31f` | 5x4a `05d31f` | 5x4a `05d31f` | 5x4a `05d31f` |
| color-value=NONE (12; 12) | [1298](witnesses/77-color-value-NONE.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 5x4a `646819` | 5x4a `9b0205` | 5x4a `9b0205` | 5x4a `9b0205` | 5x4a `9b0205` |
| hdr-extra-token=7 (12; 12) | [3183](witnesses/78-hdr-extra-token-7.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 5x4a `6cc602` | rej `gdk` | rej `gdk` | rej `gdk` | rej `gdk` |
| color-value=gray101 (11; 11) | [230](witnesses/79-color-value-gray101.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 5x4a `63a0ad` | 5x4a `5b6fb5` | 5x4a `5b6fb5` | 5x4a `5b6fb5` | 5x4a `5b6fb5` |
| color-value=red  (11; 11; ez-pixmap vs glycin: 11/40 px) | [277](witnesses/80-color-value-red-.xpm) | accept-different-pixels | 5x8a `10e777` | 5x8a `10e777` | 5x8a `0e3180` | 5x8a `10e777` | 5x8a `10e777` | 5x8a `10e777` | 5x8a `10e777` |
| color-line:extra-key-pair=extra-key-pair (11; 11; ez-pixmap vs glycin: 7/20 px) | [1303](witnesses/81-color-line-extra-key-pair-extra-key-pair.xpm) | accept-different-pixels | 5x4a `f6dcf7` | 5x4a `f6dcf7` | 5x4a `b63eec` | 5x4a `f6dcf7` | 5x4a `f6dcf7` | 5x4a `f6dcf7` | 5x4a `f6dcf7` |
| color-value=#12345 (11; 10) | [7476](witnesses/82-color-value-12345.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `rust_panic` | 5x4a `5b6fb5` | 5x4a `5b6fb5` | 5x4a `5b6fb5` | 5x4a `5b6fb5` |
| color-value=None# (10; 10) | [241](witnesses/83-color-value-None-.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 5x4a `b5fd20` | 5x4a `997221` | 5x4a `997221` | 5x4a `997221` | 5x4a `997221` |
| color-value=#1234 (10; 10) | [281](witnesses/84-color-value-1234.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `rust_panic` | 5x4a `bd7100` | 5x4a `bd7100` | 5x4a `bd7100` | 5x4a `bd7100` |
| color-key=g (10; 10) | [1284](witnesses/85-color-key-g.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `rust_panic` | 5x4 `814eac` | 5x4 `814eac` | 5x4 `814eac` | 5x4 `814eac` |
| color-value=transparent (10; 10) | [1324](witnesses/86-color-value-transparent.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 40x4a `0578a5` | 40x4a `279185` | 40x4a `279185` | 40x4a `279185` | 40x4a `279185` |
| color-value=nosuchcolor (10; 10) | [1325](witnesses/87-color-value-nosuchcolor.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 5x1 `c30cd5` | 5x1a `4389a6` | 5x1a `4389a6` | 5x1a `4389a6` | 5x1a `4389a6` |
| syntax:decl=3 (10; 10) | [1435](witnesses/88-syntax-decl-3.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 5x4 `36f64f` | 5x4 `36f64f` | 5x4 `36f64f` | 5x4 `36f64f` | 5x4 `36f64f` |
| hdr-extra-token=1 2 3 (10; 10) | [2205](witnesses/89-hdr-extra-token-1-2-3.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | 5x4a `83c295` | 5x4a `6011e4` | 5x4a `6011e4` | 5x4a `6011e4` | 5x4a `6011e4` |
| color-value=#12 (9; 9) | [217](witnesses/90-color-value-12.xpm) | accept-vs-reject | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4a `f306e9` | 5x4a `f306e9` | 5x4a `f306e9` | 5x4a `f306e9` |

## Single-decoder outliers

An independence group that alone accepts (or alone rejects) while at least two other groups disagree with it, grouped by cluster; witnesses are minimised preserving every decoder outcome.

| Group | Kind | Cluster (inputs) | Witness | glycin | image-extras | ez-pixmap | gdk-file | gdk-auto-whole | gdk-forced-whole | gdk-forced-byte |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| gdk-pixbuf | only-accepts | byte-flip (275) | [605](witnesses/91-gdk-pixbuf-only-accepts-byte-flip.xpm) | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4a `ca42f5` | 5x4a `ca42f5` | 5x4a `ca42f5` | 5x4a `ca42f5` |
| image-extras | only-rejects | byte-flip (216) | [580](witnesses/92-image-extras-only-rejects-byte-flip.xpm) | rej `ExpectedError` | rej `Format error decodin` | 5x4a `ec3e42` | 5x4a `ec3e42` | 5x4a `ec3e42` | 5x4a `ec3e42` | 5x4a `ec3e42` |
| gdk-pixbuf | only-accepts | bad-char-replace (173) | [710](witnesses/93-gdk-pixbuf-only-accepts-bad-char-replace.xpm) | rej `ExpectedError` | rej `Format error decodin` | rej `Internal` | 5x4a `ca42f5` | 5x4a `ca42f5` | 5x4a `ca42f5` | 5x4a `ca42f5` |
| image-extras | only-rejects | bad-char-insert (167) | [685](witnesses/94-image-extras-only-rejects-bad-char-insert.xpm) | rej `ExpectedError` | rej `Format error decodin` | 5x4a `ec3e42` | 5x4a `ec3e42` | rej `gdk` | 5x4a `ec3e42` | 5x4a `ec3e42` |
| gdk-pixbuf | only-accepts | bad-char-insert (161) | [711](witnesses/95-gdk-pixbuf-only-accepts-bad-char-insert.xpm) | rej `ExpectedError` | rej `Format error decodin` | rej `NonUtf8Input` | 5x16a `1d8ce8` | 5x16a `1d8ce8` | 5x16a `1d8ce8` | 5x16a `1d8ce8` |
| image-extras | only-rejects | long-run (151) | [835](witnesses/96-image-extras-only-rejects-long-run.xpm) | rej `ExpectedError` | rej `Format error decodin` | 40x4a `eddeb7` | 40x4a `29018e` | rej `gdk` | 40x4a `29018e` | 40x4a `29018e` |
| ez-pixmap | only-accepts | byte-flip (82) | [617](witnesses/97-ez-pixmap-only-accepts-byte-flip.xpm) | rej `ExpectedError` | rej `Format error decodin` | 2x4a `871f0e` | rej `gdk` | rej `gdk` | rej `gdk` | rej `gdk` |
| ez-pixmap | only-accepts | long-run (40) | [827](witnesses/98-ez-pixmap-only-accepts-long-run.xpm) | rej `ExpectedError` | rej `Format error decodin` | 2x4a `871f0e` | rej `gdk` | rej `gdk` | rej `gdk` | rej `gdk` |
| ez-pixmap | only-accepts | bad-char-insert (38) | [779](witnesses/99-ez-pixmap-only-accepts-bad-char-insert.xpm) | rej `ExpectedError` | rej `Format error decodin` | 5x4a `625324` | rej `gdk` | rej `gdk` | rej `gdk` | rej `gdk` |
| ez-pixmap | only-rejects | color-line:multi-space=multi-space (14) | [2376](witnesses/100-ez-pixmap-only-rejects-color-line-multi-space-mu.xpm) | 5x2a `abf9fa` | 5x2a `abf9fa` | rej `Internal` | 5x2a `abf9fa` | 5x2a `abf9fa` | 5x2a `abf9fa` | 5x2a `abf9fa` |
| ez-pixmap | only-rejects | byte-flip (13) | [812](witnesses/101-ez-pixmap-only-rejects-byte-flip.xpm) | 5x4a `72d142` | 5x4a `72d142` | rej `NonUtf8Input` | 5x4a `72d142` | 5x4a `72d142` | 5x4a `72d142` | 5x4a `72d142` |
| ez-pixmap | only-rejects | newlines=double (12) | [949](witnesses/102-ez-pixmap-only-rejects-newlines-double.xpm) | 5x4a `482bb9` | 5x4a `482bb9` | rej `Internal` | 5x4a `482bb9` | 5x4a `482bb9` | 5x4a `482bb9` | 5x4a `482bb9` |

## Ten most frequent outlier behaviours

- `gdk-pixbuf` only accepts on rule `byte-flip` (275 inputs): [witnesses/91-gdk-pixbuf-only-accepts-byte-flip.xpm](witnesses/91-gdk-pixbuf-only-accepts-byte-flip.xpm)
- `image-extras` only rejects on rule `byte-flip` (216 inputs): [witnesses/92-image-extras-only-rejects-byte-flip.xpm](witnesses/92-image-extras-only-rejects-byte-flip.xpm)
- `gdk-pixbuf` only accepts on rule `bad-char-replace` (173 inputs): [witnesses/93-gdk-pixbuf-only-accepts-bad-char-replace.xpm](witnesses/93-gdk-pixbuf-only-accepts-bad-char-replace.xpm)
- `image-extras` only rejects on rule `bad-char-insert` (167 inputs): [witnesses/94-image-extras-only-rejects-bad-char-insert.xpm](witnesses/94-image-extras-only-rejects-bad-char-insert.xpm)
- `ez-pixmap` only accepts on rule `byte-flip` (82 inputs): [witnesses/97-ez-pixmap-only-accepts-byte-flip.xpm](witnesses/97-ez-pixmap-only-accepts-byte-flip.xpm)
- `ez-pixmap` only accepts on rule `long-run` (40 inputs): [witnesses/98-ez-pixmap-only-accepts-long-run.xpm](witnesses/98-ez-pixmap-only-accepts-long-run.xpm)
- `ez-pixmap` only rejects on rule `color-line:multi-space=multi-space` (14 inputs): [witnesses/100-ez-pixmap-only-rejects-color-line-multi-space-mu.xpm](witnesses/100-ez-pixmap-only-rejects-color-line-multi-space-mu.xpm)
- `ez-pixmap` only rejects on rule `byte-flip` (13 inputs): [witnesses/101-ez-pixmap-only-rejects-byte-flip.xpm](witnesses/101-ez-pixmap-only-rejects-byte-flip.xpm)

## Ten leniency findings (XPM, seed 731)

Votes: image-extras (which glycin's in-tree path wraps), `ez-pixmap` (a deliberately naive crate: chars-per-pixel 1, `c` key only), and GDK-Pixbuf's XPM module through four entry points. libXpm, the reference implementation, is unavailable here, so there is no reference vote; GDK is the most lenient decoder and image-extras the strictest. Counts are inputs out of 10,000. Colour results below come from 1x1 probe files, not from the random mutations.

1. **GDK forced-type loaders accept 6,010 inputs, `ez-pixmap` 3,512, image-extras (and glycin) 960.** Glycin and image-extras never disagree. image-extras documents itself as a strict parser: no XPM2, no XPMEXT, no HSV, exact `/* XPM */` header, trailing comma required on each string.
2. **GDK turns an unparseable colour into transparent pixels; image-extras rejects the file; `ez-pixmap` paints opaque black.** Applies to `#12345`, `#GGG`, `#FFFF`, `rgb:ff/00/00`, `%ff0000`, `transparent`, `rebeccapurple`, `red x`, and a lone `s foo`. `#1234567` is transparent in GDK, rejected by image-extras, and `#123456` opaque in `ez-pixmap` (it reads the first six digits).
3. **Named colours come from different tables.** `gray` is (190,190,190) in image-extras and `ez-pixmap` (X11R6) and (128,128,128) in GDK. `crimson` (a CSS-only name) is accepted by GDK, rejected by image-extras, and black in `ez-pixmap`. `dark slate gray` (spaces) is accepted by image-extras and `ez-pixmap` but transparent in GDK. `Red` and `RED` match in image-extras and GDK, and become opaque black in `ez-pixmap` (case-sensitive lookup). `NONE` is transparent in image-extras and GDK, opaque black in `ez-pixmap`.
4. **Only GDK uses the monochrome, grey and 4-level-grey keys.** A colour line with only `m black`, `g gray50` or `g4 gray25` decodes in GDK and is rejected by image-extras ("no specified value for color visual") and `ez-pixmap`. `s foo c red` decodes in image-extras and GDK but not in `ez-pixmap` (which requires `c` as the first key).
5. **Hex colours of 1 to 4 digits per channel.** image-extras and GDK agree on all four widths (`#F00` and `#FFFF00000000` are both red). `ez-pixmap` rejects 1-digit channels and reads 3- and 4-digit channels as the first two digits (`#FFF000000` becomes `#FFF000`).
6. **GDK ignores pixel-grid defects that image-extras and `ez-pixmap` reject.** An undefined pixel code is drawn with colour table entry 0; extra rows or over-long rows are silently dropped; a header ncolors one smaller than the number of colour lines is accepted (the extra colour line is skipped); a duplicate colour code takes the later definition. A short row, an ncolors that is too large, width or height 0 are rejected by every decoder.
7. **XPM2 (`! XPM2`) is rejected by all three decoders**, so it is not covered by any vote here (libXpm would read it). XPMEXT blocks and the `XPMEXT` header flag are accepted by GDK and `ez-pixmap` (extension data ignored) and rejected by image-extras. A header hotspot (`xhot yhot`, including `-1 -1`) is accepted by all decoders and never changes the pixels.
8. **Chars per pixel.** image-extras and GDK accept 1 to 4 (including a space as the pixel code); `ez-pixmap` rejects anything but 1; a cpp of 8 or more is rejected by all.
9. **C syntax tolerance.** GDK and `ez-pixmap` accept a missing final `;` or `}`, a missing comma after a string, `static char **name = {`, `char *name[]={`, `const char * const`, and CR or CRLF line endings; image-extras rejects each of these. Adjacent string literals (`"a" "b"`) are accepted only by `ez-pixmap` (it extracts quoted strings itself), not by GDK or image-extras.
10. **GDK detection and entry points disagree.** Content auto-detection needs the exact `/* XPM */` prefix and rejects 762 inputs that the forced-type loader accepts (for example `/*  XPM  */`, `// XPM`, `/* XPM 3 */`, or a stray byte before the comment); filename-based `gdk_pixbuf_new_from_file` differs from the forced loader on 537 inputs. Whole-buffer and byte-by-byte writes never disagree.

Counts only (not analysed): crash or timeout in at least one decoder on 15 inputs, all in glycin (status `crash`, reproduced on a serial rerun with a 10 s limit). `ez-pixmap` returned a caught panic on 491 inputs and glycin/image-extras on 16, which the matrix records as rejections.

## Run

`./run.sh --inputs 10000 --seed 731 --jobs 6` from `/__modal/volumes/vo-Iyh3VLkOB7cEHrIp2RkTs7/projects/leniency/xpm`; outputs in `/tmp/claude-0/leniency-xpm`.
