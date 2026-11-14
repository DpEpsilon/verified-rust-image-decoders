# Brief: verified PNG decoder (Verus) — scope

Status: specification phase. The draft spec is `spec-draft/` (PNG-CORE-SPEC 0.1: SPEC.md P1–P21, a 62-name closed error list, the Strict oracle `png_oracle.py` with a hand-written inflate, 592 vectors exercising all 62 errors), the leniency catalogue is `policy-draft/` (30-field record, four profiles, closed `recovered` events R1–R5, 16 owner questions), and the proof plan is `PLAN.md`. Nothing is frozen and no Rust exists yet. Once the owner answers `policy-draft/QUESTIONS.md` PNG-Q1–Q13, `spec-draft/` and `policy-draft/` are frozen (hashes in a `FREEZE.txt` each) and this brief gains the binding rules of `../dib/BRIEF.md`, `../dib/BRIEF-POLICY.md` and `../alloc-delta/SPEC.md` unchanged.

## Goal

A feature-complete PNG decoder with the guarantees the DIB/BMP, TGA and (planned) GIF cores have: safety (T1) for every input and every policy record; exact equality with the frozen model (T2 and the integrated entry-point theorems), including the first error; allocation bounds (T3) through the sealed `alloc_guard` with `try_*` entry points, `OutOfMemory` outside the format error set, and the G/Prog/PreAlloc theorem shape of `../alloc-delta/THEOREMS-ALLOC-DELTA.md`; termination; a linear work bound; and the policy machinery: a closed `recovered` event list (R1–R5), the Strict bridge (`policy(Strict) = core model` for every input), obligation E (`recovered` iff an event occurred) and obligation R (RecommendedBeta returns identical RGBA with `recovered=false` on every Strict-accepted input). Both crates are `no_std + alloc`; the inflate crate needs no `alloc` at all.

## Features in scope

- **Datastream.** Signature, chunk layout, CRC-32 on every chunk, chunk-type letters, critical vs ancillary (unknown critical rejected, unknown ancillary skipped, reserved bit treated as unknown), the third edition's ordering table including the APNG chunks, duplicates, IEND, trailing bytes.
- **IHDR.** All fifteen colour-type/bit-depth pairs: greyscale 1, 2, 4, 8, 16; truecolour 8, 16; indexed 1, 2, 4, 8; greyscale+alpha 8, 16; RGBA 8, 16. Compression and filter method 0, interlace 0 or 1.
- **PLTE and tRNS.** Palette length and entry count rules; suggested palettes in truecolour images (validated, not applied); tRNS for colour types 0, 2 and 3 with exact key comparison at native depth and masking of unused high bits; tRNS forbidden for types 4 and 6.
- **zlib/DEFLATE.** RFC 1950 header (CM = 8, CINFO ≤ 7, FCHECK, FDICT rejected), stored, fixed and dynamic blocks, canonical Huffman construction with zlib's completeness rule, length/distance decoding with the declared window, Adler-32, nothing after the Adler-32, the image data as one stream over any number of IDAT (or fdAT) chunks of any size including zero, output size exactly `R(w,h)` (too little: `TruncatedImageData`; one byte more: `ExcessImageData`).
- **Scanlines.** The five filter types (None, Sub, Up, Average, Paeth), filter byte distance `max(1, bits/8)`, Adam7 with empty passes contributing no bytes, sub-byte packing.
- **Conversion.** RGBA8 (exact scaling for depths 1, 2, 4; rounding for 16, PNG-Q3) and RGBA16 (exact scaling from every depth).
- **Ancillary chunks.** Validated and exposed raw: gAMA, cHRM, sRGB, iCCP (name and method; profile through `icc_profile`), sBIT, cICP, mDCV, cLLI, bKGD, hIST, pHYs, sPLT, eXIf (byte range), tIME; tEXt, zTXt, iTXt validated structurally (keyword rules, separators, method bytes) and never inflated. None of them changes a pixel except tRNS (PNG-Q4: no gamma or colour management in the core, as in every measured incumbent).
- **APNG.** acTL, fcTL, fdAT with the shared sequence counter, frame rectangles inside the canvas, dispose ops NONE, BACKGROUND, PREVIOUS (PREVIOUS on the first frame acts as BACKGROUND), blend ops SOURCE and OVER (exact integer OVER, PNG-Q8), the default image as frame 0 or outside the animation, frame-count check, raw `num_plays` and `delay_num/delay_den`.

## API

```rust
pub fn try_decode_still(b: &[u8]) -> Result<Still, TryError>;            // default image, RGBA8 (P15)
pub fn try_decode_still16(b: &[u8]) -> Result<Still16, TryError>;        // RGBA16, any source depth
pub fn try_decode_still_with_policy(b: &[u8], p: Profile) -> Result<(Still, bool), TryError>;
pub struct Frames<'a> { /* borrowed input, canvas, Raw buffer, Saved, walk cursor */ }
impl<'a> Frames<'a> {
    pub fn try_new(b: &'a [u8], p: Profile) -> Result<Frames<'a>, TryError>;   // walk + default image
    pub fn info(&self) -> &Info;                                               // IHDR, colour chunks, num_plays
    pub fn try_next(&mut self) -> Result<Option<FrameView<'_>>, TryError>;     // fused after Err
}
pub struct FrameView<'f> { pub rgba: &'f [u8], pub rect: (u32, u32, u32, u32), pub delay: Option<(u16, u16)>,
                           pub dispose: u8, pub blend: u8, pub recovered: bool }
pub fn try_icc_profile(b: &[u8]) -> Result<Option<Vec<u8>>, TryError>;   // P17
pub fn chunks(b: &[u8]) -> ChunkIter<'_>;                                // borrowed (type, data) after a walk; text for glue
```

`Info` carries the IHDR fields, palette entry count, tRNS presence, raw gAMA, cHRM, sRGB, cICP, mDCV, cLLI, sBIT, bKGD, pHYs and tIME values, the iCCP profile name and the eXIf byte range. `TryError` is `Format(Error) | OutOfMemory(OutOfMemory)` with the 62 error names of P18. Delay mapping (image: `den = 0 → 100`, milliseconds; glycin additionally `0 ms → 100 ms`) and loop translation belong to the glue.

How the output model meets the consumers:

- **gdk-pixbuf** wants RGB8 or RGBA8 (`io-png.c` strips 16-bit samples to their high byte and expands everything else) plus the options `icc-profile` (base64), `x-dpi`/`y-dpi` from pHYs and `tEXt::key` from text chunks. The glue copies `Still` into a pixbuf (dropping alpha when the source has none and no tRNS), base64-encodes `try_icc_profile`, converts pHYs and reads text through `chunks`. gdk-pixbuf has no APNG support (the loader shows the default image); a module could wrap `Frames` in a `GdkPixbufAnimation` as the GIF loader does.
- **image 0.25.10 / png 0.18.1** returns native depth (`L8 … Rgba16`) with `EXPAND`; `into_rgba8` rounds 16-bit samples exactly as P13.2 does. Its APNG decoder yields full-canvas RGBA8 frames, as `Frames` does.
- **glycin** passes 16-bit formats through (`R16g16b16a16` etc.) and needs the ICC profile, cICP, Exif and XMP. It would use `try_decode_still16` for 16-bit sources, `Info.cicp`, `try_icc_profile`, the eXIf range and the XMP iTXt through `chunks`; its animation path maps to `Frames`.

## Resource caps and allocation

Caps are the constants of SPEC P1 (one section; theorems are stated over the names, PNG-Q5): input 256 MiB; width and height ≤ 16,384; `4·W·H ≤ 64 MiB`; at most 65,536 frames and 2^30 frame pixels; ICC profile ≤ 16 MiB. Chunk lengths are bounded by `2^31 − 1` and the input; text chunk sizes need no cap because the core never inflates them; inflate output never exceeds the declared size `R(w,h)` (the decompression-bomb bound: work and memory are linear in the output the header already promised, whatever the compression ratio). Allocation (P20), all through the guard, none before the structural walk succeeds:

| Site | Bytes | Reserved |
|---|---|---|
| `Raw` | `R(W,H) ≤ H·(1 + 8W)` (filtered data; inflate's output and its own window) | once per decoder |
| `Output` | `4·W·H` (`8·W·H` for `try_decode_still16`), zero-filled | once per decoder |
| `Saved` | `4·W·H` | at the first frame (k > 0) whose disposal is PREVIOUS, once |
| `IccWindow`, `Icc` | 32,768, then the measured profile length | `try_icc_profile` only |

Peak live heap per decoder: `R + 8·W·H` for an animation (at most 192 MiB plus `H` at the caps for 8-bit RGBA, 256 MiB plus `H` for 16-bit RGBA). The walk keeps a fixed-size record (no chunk list, no frame list: data chunks are re-walked by offset), the Huffman tables are fixed arrays in the decoder state, inflate allocates nothing. `PreAlloc` holds exactly when the walk fails. `OutOfMemory` from `Saved` can displace a later format error of that frame (accepted for ICNS, alloc-delta D5). The `try_*` API, the sealed log, the removed non-`try` names and the `no_std + alloc` certification follow `../alloc-delta/SPEC.md` §1–§9 unchanged; A-STD-1/2 are the only trusted items.

## Decomposition (PNG-Q12)

Two crates: `inflate_verus` (zlib + DEFLATE; `no_std`, no `alloc`; API `inflate_zlib(src: &mut impl Segments, out: &mut [u8]) -> Result<usize, InflateError>` with `OutputFull` as the limit outcome; model `Inflate(Z, L)` = output prefix of length ≤ L or the first error; `Segments` abstracts IDAT/fdAT payload runs with a spec function `concat()`) and `png_verus` (chunks, filters, Adam7, conversion, APNG compositor, policy, guard). The split costs no speed (generic, inlined), no memory (output-as-window, zero allocation, so no cross-crate allocation log as ICO needed for DIB) and no PNG-side proof complexity (one frozen model function); it gives ICO, ICNS (both embed PNG), TIFF Deflate, zTXt/iCCP and any later format a verified inflate. This is consistent with the owner's GIF Q10 verdict, whose three costs do not arise here. The APNG compositor stays in `png_verus` (owner preference), as its own module and model.

## Evidence base

- Spec texts: PNG third edition (W3C REC 2025-06-24, GitHub tag `Third-Edition`), RFC 1950/1951 (zlib 1.3.1 copies); local copies and re-fetch commands with SHA-256 in `spec-draft/SOURCES.md`; not committed (`.gitignore`).
- Incumbents: gdk-pixbuf 2.42.12 source (`sources/gdk-pixbuf-2.42.12/`) and the installed Debian build with libpng 1.6.48; png 0.18.1, fdeflate 0.3.7 and image 0.25.10 from `~/.cargo/registry`; glycin's image-rs loader (measured since GAPS-CLOSED.md gap 1 with `policy-draft/probes/glycin-runner`, its PNG loader code in-process; it patches png to git revision e917d6a). Runners: `policy-draft/probes/runner` (Rust; build: `CARGO_TARGET_DIR=/tmp/claude-0/png-probe-target cargo build --release --offline --locked -j1`) and the C runners of `~/projects/leniency/gif/runners` and `~/projects/leniency/shared/gdk.c` built into `/tmp/claude-0/png-probe-bin` (libspng `46c81dc`, stb `4e9b368` cloned to `/tmp/claude-0/png-probe-src`; commands in `policy-draft/probes/run_probes.py`'s docstring and `~/projects/leniency/gif/build.sh`).
- Results: `policy-draft/probes/results.json` (748 inputs × 10 runners), `policy-draft/OPTIONS.md`, `policy-draft/QUESTIONS.md`, `spec-draft/DESIGN-NOTES.md`.

## Corpus plan (PNG-Q7)

Collect into `$CORPUS_DIR/png/files/` with a `MANIFEST.tsv` (source URL, revision, SHA-256, licence per file), as for the other formats; nothing is committed. The licence column below was checked against the repositories' licence texts in `policy-draft/CORPUS-LICENCES.md`, which corrects five rows (libpng `contrib/pngsuite`, image-png `tests/`, Wuffs, the APNG suite's author and licence, Adwaita).

| Source | What it covers | Licence |
|---|---|---|
| PngSuite (Willem van Schaik, `schaik.com/pngsuite`, also vendored in libpng `contrib/pngsuite` and image-png `tests/pngsuite`) | every colour type and depth, interlacing, filters, ancillary chunks, 14 deliberately corrupt `x*.png` files | freeware: "permission to use, copy, modify and distribute … for any purpose and without fee" |
| libpng `contrib/testpngs`, `contrib/pngsuite` (tag `v1.6.48`) | gamma, sBIT, tRNS and palette variants generated by `makepng` | PNG Reference Library License v2 |
| image-png `tests/` (bugfixes, fuzz regressions, APNG samples) at the e917d6a revision and 0.18.1 | regressions for the decoder image and glycin use | MIT OR Apache-2.0 (per-file provenance to confirm) |
| gdk-pixbuf `tests/test-images/` (tag 2.42.12) | the files gdk-pixbuf's own tests decode, including fuzz reproducers | LGPL-2.1-or-later |
| Wuffs `test/data/` (png, zlib, deflate) | inflate edge cases, bad checksums, Adam7 corner sizes | Apache-2.0 |
| APNG: Philip Jägenstedt's APNG test suite; Chromium `web_tests/images/resources/*.png` (APNG); Mozilla `image/test/reftest/apng` | dispose/blend semantics, broken animations | licences UNVERIFIED for the first; BSD-3-Clause (Chromium); MPL-2.0 (Mozilla) |
| Real-world sample: Adwaita and hicolor icon themes, a Wikimedia Commons random sample (per-file CC licences recorded), screenshots from the owner's machines | what PNG files in the wild look like (iCCP names, text chunks, surplus zlib data, sizes) | per file |

The pass records, per Strict error, how many real files hit it, and re-measures GdkCompat and ImageCrateCompat on files that were not used to fit them.
