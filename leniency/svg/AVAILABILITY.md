# SVG/SVGZ availability survey

**Status: implemented.** Candidates run: librsvg (Rust crate 2.63.4, same code base as glycin's library) and resvg 0.49. Neither is the glycin binary.

## What glycin uses

`glycin-loaders/glycin-svg/src/bin/glycin-svg.rs`: `gufo_svg::Svg::new` (XMP extraction, SVGZ handling), then `rsvg::Handle::from_stream_sync` with `HandleFlags::FLAG_UNLIMITED` (crate `librsvg-rebind` 0.3.0, linking the C `librsvg-2`). Dimensions come from `intrinsic_size_in_pixels()`, then percent-times-viewBox, then 300x300. Rendering uses `render_document` into a cairo ARGB32 surface. The runner reproduces the dimension chain and rendering call with the `librsvg` crate; it does **not** use `FLAG_UNLIMITED` (the crate's `Loader` default limits apply) and does not go through gufo-svg, so SVGZ inputs reach librsvg's own gzip sniffing.

## Installed here

- No librsvg (`pkg-config librsvg-2.0` fails), no `rsvg-convert`, no resvg, no inkscape, no GDK SVG module.
- Installed and usable: cairo 1.18.4, pango/pangocairo 1.56.3, glib/gio 2.84.4, freetype, fontconfig, harfbuzz, gdk-pixbuf 2.42.12; fonts: DejaVu only.
- libxml2 has the runtime library only (`libxml2.so.2`), no headers/`.pc`; the librsvg crate requires libxml2 >= 2.9 through system-deps.

## Buildable

| candidate | source | status | independent? |
| --- | --- | --- | --- |
| libxml2 2.14.5 | gitlab.gnome.org archive (cmake + ninja) | built into `/tmp/claude-0/prefix` | dependency of librsvg |
| `librsvg` crate 2.63.4 | crates.io | built (`librsvg`) | glycin's renderer family (same source tree as the C library; version differs from whatever glycin is deployed against) |
| `resvg` 0.49 / `usvg` / `tiny-skia` | crates.io | built (`resvg`) | **yes** |
| `librsvg-rebind` + system librsvg-2 | needs `librsvg2-dev` | **not available** | exactly glycin's binding |
| cairosvg, inkscape | PyPI / apt | not available | - |

## Seeds

Hand-written in `harness.py` (68 synthetic SVGs and gzip variants): size/viewBox/unit/percent/aspect combinations, gradients, paths, clip/mask/filter/pattern/CSS/nested svg/`use`, data-URI PNG, DOCTYPE/entity forms, encodings (ISO-8859-1, UTF-8 BOM, UTF-16), namespace variants, gzip wrapping variants. External references (`http://example.invalid/...`, relative file names, external entity) are present only to measure accept/reject; neither runner supplies a base URL and nothing is fetched.

## Gaps

- The exact glycin binding (`librsvg-rebind` against the C library, with `FLAG_UNLIMITED` and gufo-svg): needs apt `librsvg2-dev` (+ `libxml2-dev`), or a Dockerfile step building librsvg from gitlab.gnome.org.
- Text rendering is excluded from comparison value: resvg is given no fonts, librsvg uses fontconfig. One text seed (`text-basic`) records the resulting difference.

## Harness note: address-space cap and thread pools

`engine.invoke` runs each decoder under `prlimit --as=768MiB`. Under that cap rayon's default global thread pool cannot start, which made decoders that use rayon (`rawler`, `librsvg`, the `exr` crate's callers) report spurious errors or panics in a first run. The Rust runners now set `RAYON_NUM_THREADS=1` before any decoder code runs, and the numbers in this directory come from the second run. The machine has one CPU shared with other jobs, so crash and timeout counts are counts under load.
