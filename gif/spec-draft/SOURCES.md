# Sources for GIF-CORE-SPEC draft 0.2 (unchanged since draft 0.1)

## Format text

`sources/spec-gif89a.txt`: "Graphics Interchange Format (sm) Version 89a", CompuServe Incorporated, 31 July 1990 (2,476 lines, CR stripped), SHA-256 `c0e48c12e3e91b1e78090caffb1d1ffe8e7d0face4350da675044a9a4be02aa3`. The canonical host, `https://www.w3.org/Graphics/GIF/spec-gif89a.txt`, refused connections from this sandbox (curl: connection reset; WebFetch: ECONNRESET). The copy was fetched from two unrelated GitHub repositories found by code search, and the two were byte-identical after removing carriage returns:

- `https://raw.githubusercontent.com/SixLabors/ImageSharp/407ce6b081ce5faf7c0bf4966b6a7f4b96a60c2d/src/ImageSharp/Formats/Gif/spec-gif89a.txt`
- `https://raw.githubusercontent.com/tonycoz/imager/60d5b5e9744f4722674d2bbd79f9ffd6337e2cac/fileformatdocs/spec-gif89a.txt`

Identity with the W3C copy is therefore UNVERIFIED; compare the hash when w3.org is reachable. The GIF87a document (`spec-gif87.txt`) was not obtained; 87a behaviour is taken from the 89a text's version notes (§6, §17, Appendix A).

## Incumbent sources read

| Library | Version | Files (SHA-256 of the copy read) |
|---|---|---|
| gdk-pixbuf | tag `2.42.12` of `github.com/GNOME/gdk-pixbuf` (local clone `~/projects/spec/gdk-pixbuf`), copies in `../sources/gdk-pixbuf-2.42.12/` | `io-gif.c` 59a2f4bf…, `io-gif-animation.c` 3383edd6…, `io-gif-animation.h` 21a15283…, `lzw.c` 09a8e6a2…, `lzw.h` 457706e4…; also `gdk-pixbuf-loader.c`, `gdk-pixbuf-io.c`, `gdk-pixbuf-animation.c` at the same tag |
| gif (crate) | 0.14.2 (the version `image` 0.25.10 resolves; `~/.cargo/registry`) | `src/reader/decoder.rs` 89d06df6…, `src/reader/mod.rs` d5a1d2f6…, `src/reader/converter.rs` d6963365… |
| image (crate) | 0.25.10 | `src/codecs/gif.rs` 696004f6… |
| weezl (crate) | 0.1.12 | `src/decode.rs` cfbeff7f… |
| glycin | `~/projects/glycin-plan/glycin` at 7344b95 | `glycin-loaders/glycin-image-rs/src/animated.rs` (delay mapping lines 96–104) |

The installed gdk-pixbuf is Debian `2.42.12+dfsg-4+deb13u1`. Its Debian patch series could not be fetched (sources.debian.org and salsa.debian.org are unreachable). Upstream master changed one line of `lzw.c` since the tag (`return output_length` → `return n_written` on an invalid code, the CVE-2025-6199 fix); the installed library's output on probe `lzw-invalid-code-first` equals the post-fix behaviour, so the Debian package most likely carries that fix. UNVERIFIED beyond that one measurement.

giflib 5.1.4 (`github.com/mirrorer/giflib` fa37672) and libnsgif (`github.com/netsurf-browser/libnsgif` c26c7aa) were run through the leniency harness only; their sources were not read for this draft.
