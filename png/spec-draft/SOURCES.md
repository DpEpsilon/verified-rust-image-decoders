# Sources for PNG-CORE-SPEC draft 0.1

## Format texts (local copies in `sources/`, not committed: see `../.gitignore`)

| File | What | SHA-256 | How to re-fetch |
|---|---|---|---|
| `png-3e-index.html` | Portable Network Graphics (PNG) Specification (Third Edition), W3C Recommendation 24 June 2025: the ReSpec source at tag `Third-Edition` (commit `037f3c3da08dad7b02ec14b816a561e7d063cb70`, message "Changes for 2025-06-24 W3C Recommendation") of `github.com/w3c/png` | `eef4ec90eadc719812498129a4ab6bbbed5c3446b982d5a37ac417d5cd864c68` | `curl -sSfL -o png-3e-index.html https://raw.githubusercontent.com/w3c/png/Third-Edition/index.html` |
| `png-3e.txt` | plain-text rendering of the file above (script in this table's last row) | `ef69a95f586f5edd20b9d6c8b466058e66866fe28161d54a67efa12e1f46a51a` | `../tools/spec_section.py ID` prints one section by its anchor; the full text was produced by stripping tags (scripts, styles removed; `<br>`, block ends → newlines; HTML entities unescaped) |
| `rfc1950.txt` | RFC 1950, ZLIB Compressed Data Format 3.3, as shipped in zlib 1.3.1 `doc/` | `8f0475a5c984657bf26277f73df9456c9b97f175084f0c1748f1eb1f0b9b10b9` | `curl -sSfL https://raw.githubusercontent.com/madler/zlib/v1.3.1/doc/rfc1950.txt` |
| `rfc1951.txt` | RFC 1951, DEFLATE 1.3, same origin | `5ebf4b5b7fe1c3a0c0ab9aa3ac8c0f3853a7dc484905e76e03b0b0f301350009` | `curl -sSfL https://raw.githubusercontent.com/madler/zlib/v1.3.1/doc/rfc1951.txt` |

The canonical hosts `www.w3.org` and `www.rfc-editor.org` refused connections from this sandbox (curl: connection reset), as `w3.org` did for the GIF phase. Identity of the GitHub tag with the published W3C document is therefore UNVERIFIED beyond the ReSpec metadata in the file (`specStatus: "REC"`, `publishDate: "2025-06-24"`); compare the rendered text with `https://www.w3.org/TR/2025/REC-png-3-20250624/` when reachable. The ReSpec source has no section numbers, so SPEC.md cites anchors (`[3e #fcTL-chunk]`), which are stable across renderings.

## Incumbent sources read

| Library | Version | Files (SHA-256 of the copy read) |
|---|---|---|
| gdk-pixbuf | tag `2.42.12` of `github.com/GNOME/gdk-pixbuf` (local clone `~/projects/spec/gdk-pixbuf`, `git show 2.42.12:…`), copies in `../sources/gdk-pixbuf-2.42.12/` | `io-png.c` a1ad64da…, `gdk-pixbuf-loader.c` eec9f41d…, `gdk-pixbuf-io.c` 0984f9cf… |
| libpng | 1.6.48 (Debian `1.6.48-1+deb13u6`, the library gdk-pixbuf links); headers only (`/usr/include/libpng16`), the `.c` sources were not available, so libpng behaviour is taken from measurements and from `png.h` documentation | — |
| png (crate) | 0.18.1 (the version `image` 0.25.10 resolves from crates.io; `~/.cargo/registry`) | `src/decoder/stream.rs` af87e912…, `src/decoder/zlib.rs` fb97c270…, `src/decoder/mod.rs` 8c15bb20…, `src/decoder/transform/palette.rs` 8eed7854… |
| fdeflate | 0.3.7 (png's inflate) | `src/decompress.rs` 26eea8e9… |
| image (crate) | 0.25.10 | `src/codecs/png.rs` 0bdbbd9f…, `src/color.rs` ba717487… |
| glycin | `~/projects/glycin-plan/glycin` at 7344b95; it patches `png` to the image-png git revision `e917d6a` (`~/.cargo/git/checkouts/image-png-1d85973eec6d3c42/e917d6a`) | `glycin-loaders/glycin-image-rs/src/{lib.rs,animated.rs}`, `glycin-utils/src/image_rs.rs` |

The source reading (with line citations) was done by a subagent and is summarised in `../policy-draft/OPTIONS.md`; every catalogued behaviour was also measured unless marked UNVERIFIED.

## Runners measured

`../policy-draft/probes/run_probes.py` lists them: `image`, `image-still`, `pngcrate` (built from `../policy-draft/probes/runner`, image 0.25.10 + png 0.18.1 from crates.io, no git patch), `libpng`, `spng` (libspng `46c81dc`), `stb` (stb_image `4e9b368`) from `~/projects/leniency/gif/runners/png_native.c`, and the gdk-pixbuf runners `gdk-loader`, `gdk-loader-byte`, `gdk-anim` (`~/projects/leniency/gif/runners/gdk_anim.c`) and `gdk-file` (`~/projects/leniency/shared/gdk.c`, `gdk_pixbuf_new_from_file`). glycin itself was not rebuilt (its workspace build is heavy for this sandbox); its PNG behaviour is image 0.25.10 plus the png git revision, with the differences listed in OPTIONS.md, and the earlier leniency run (`~/projects/leniency/png/LENIENCY.md`) measured it.
