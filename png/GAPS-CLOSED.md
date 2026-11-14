# PNG specification phase: gaps closed

The phase report (commit 3ef5dc4) listed nine gaps. Seven are closed, two are partly closed (gap 1: glycin's loader code runs, but in-process and without its sandbox; gap 3: the published W3C text could not be fetched). The oracle changes leave Strict untouched and raise ImageCrateCompat's fitted agreement; on a held-out set frozen after the changes, GdkCompat agrees with the gdk-pixbuf loader on 389 of 395 inputs and ImageCrateCompat with image on 395 (still) and 394 (frames).

## Freeze records (gap 6)

Held-out set A frozen 2026-11-14 before any runner saw it: `policy-draft/probes/heldout_def.py`, seed 20261114, 400 inputs, digest `e45b8d18bad0f0505896de8cb297cc1b41002466ea2e67f29fa45ab2986f911d` (SHA-256 over name, NUL and SHA-256 of each input, in order).

Held-out set B frozen 2026-11-14 after the gap-2 and gap-5 oracle changes and before any runner saw it: same generator, seed 20261115, 400 inputs disjoint from set A, digest `7379a01783d1b321c74fda5a6d2f82c9f6ff36f04414d84274902c2ac1f18330`; policy_oracle.py at that moment: SHA-256 `d783e367c40afe56f1304b4b2e3e3a8c7ce31d4db0904bf6e6c4bee508525929`. Final policy_oracle.py: `c6d9895416a0f5ac03a56379c35510cf17962382bca98775e720246e92dc0677`.

## Check results before and after

| Check | Before (3ef5dc4) | After |
|---|---|---|
| `spec-draft/check_vectors.py` | 592/592 vectors, 62/62 errors, inflate vs zlib 741/0 | unchanged (`png_oracle.py` was not touched) |
| Strict bridge (`validate.py`) | 737 inputs, 0 differences | 737 inputs, 0 differences |
| Obligation R | 364 Strict-accepted inputs, 0 violations | 364, 0 violations |
| GdkCompat vs GDK loader, fitted set | 721/721 | 721/721 |
| ImageCrateCompat vs image still, fitted | 720/721 | 721/721 |
| ImageCrateCompat vs image frames, fitted | 718/721 | 721/721 |
| ImageCrateCompat vs glycin frames, fitted | not measured | 720/721 (glycin's png fixes the interlaced-subframe bug) |
| GdkCompat vs `gdk_pixbuf_new_from_file` (not targeted) | 616/721 | 616/721 |
| RecommendedBeta vs image still / GDK loader | 686/721, 608/721 | unchanged |
| Strict vs image still / GDK loader | 495/721, 378/721 | unchanged |
| RecommendedBeta acceptance gain | 231 of 289 | unchanged |
| Inputs above 4 KiB, all four compat comparisons | not compared | 14/14 each |
| Held-out set A: GdkCompat; ImageCrateCompat still, frames, glycin | not measured | committed oracle 378, 388, 387, 387 of 390; final 388, 389, 388, 388 |
| Held-out set B: same four | not measured | at freeze 389, 395, 394, 393 of 395; final 393, 395, 395, 394 |

The set-B figures at freeze are the clean hold-out. Two changes came after them: the stored-block fix in the GDK model (gap 2, prompted by four set-B misses) and a comparison fix in `validate.py` (a frame iterator whose first item is an error now counts as an error, as the runners report it).

## Gap 1: glycin was not run. Partly closed

`policy-draft/probes/glycin-runner/` builds a runner from a PNG-only copy of glycin 7344b95's `glycin-image-rs` loader (`load`, `specific_frame`, `ImageRsFormat`, `animated.rs`), linked against glycin's own `glycin-utils` and `gufo`, with the crate versions of glycin's `Cargo.lock` (image 0.25.10, png patched to git `e917d6a`). It drives the loader the way glycin's frame loop does. Build: `CARGO_TARGET_DIR=/tmp/claude-0/png-glycin-target cargo build --release --offline -j1` (2 min 39 s on one CPU). Results for the 748 fitted inputs are in `policy-draft/probes/results-glycin.json` (1.0 MB, not ignored); the held-out runs include glycin.

Outcome: glycin's frames equal image's frame loop on 746 of 748 inputs. The exceptions:

- `probe:apng-interlaced`: glycin is correct, image is not.
- `vec:ihdr/width-2^31`: glycin returns "Dimension too large for system", image aborts on a 6 GiB allocation.

Both runners abort on `probe:dims-65535-square-tiny-data` and `dims-2^31`. On held-out set B glycin also differs from image only on an interlaced APNG (`heldB:031`). glycin's delay mapping (0 ms becomes 100 ms) is now measured.

Still unverified:

- glycin's process isolation (sandboxed loader process, seccomp, memory limits), which might contain the aborts.
- Its MIME sniffing.
- `image_extras::register()`, which registers XBM/XPM hooks and is not on the PNG path.

## Gap 2: libpng not read. Closed

libpng `v1.6.48` (commit `ea12796`) is in `sources/libpng-1.6.48/`; Debian's installed library reports 1.6.48 (`png.h`, package `1.6.48-1+deb13u6`). zlib 1.3.1's `inflate.c`/`inffast.c` are in `sources/zlib-1.3.1/`. Re-fetch commands are in `spec-draft/SOURCES.md`.

The derived rule (OPTIONS.md `image_data_error`, `eof`; DESIGN-NOTES N17):

- libpng inflates the image data into a buffer of exactly one filtered row. It hands a row to gdk-pixbuf only after an `inflate()` call returns `Z_OK` or `Z_STREAM_END` with that buffer full.
- zlib returns early when it needs output room or more input. It decodes the next block header, code-length table, literal/length code, distance code and Adler-32 *before* it checks for output room. It checks a distance against the history in MATCH, *after* that check.
- Every `Z_DATA_ERROR` is a benign error, which by default is a warning, so the rows already handed over stay. A preset dictionary (`Z_NEED_DICT`) is fatal. So is an IDAT run that ends before the zlib stream ends ("Not enough compressed data").
- The whole-buffer loader passes each IDAT chunk's data to one call. A completed row is therefore lost exactly when the error is detected in the same chunk with no output byte in between, and the error is not a distance too far back. A lost row is never unfiltered.
- For interlaced images, gdk-pixbuf enables libpng's interlace handling. libpng then delivers each pass row to 8, 8, 4, 4, 2, 2 or 1 image rows, replicates pixels to the right and combines in "blocky display" mode. So a damaged interlaced image shows a pixelated preview, not grey rows.
- When the file ends inside a stored block, zlib has already copied the bytes that are present.

The fitted surrogate was right on all 748 fitted inputs and wrong in four places:

- distance errors;
- errors whose detection needs bytes of the next IDAT chunk;
- interlaced images;
- truncated stored blocks.

The derived rule matches the loader on 10 set-A inputs the surrogate missed and on the four set-B stored-block cases. `policy_oracle.py` replaces the surrogate. Still unverified: Debian's `+deb13u6` patches to libpng (sources.debian.org is unreachable). Zlib's exact byte position for an invalid Huffman code is approximated by the oracle's bit position, which matters only when the error sits at an IDAT chunk boundary.

## Gap 3: spec copy identity. Partly closed (UNVERIFIED at the text level)

www.w3.org, web.archive.org, archive.ph, w3c.github.io, jsdelivr, unpkg and cdnjs all reset the connection. Reachable evidence is GitHub only:

- The local file is byte-identical to the `Third-Edition` tag.
- The tag commit changes only ReSpec metadata after the CR text.
- The W3C transition requests say "no changes".
- `w3c/webref`'s crawl of the published REC (2025-06-25) contains all 335 ids of the local source and the 16 anchors SPEC.md cites, each on a numbered heading. Its headings (183) and definitions (69) match a ReSpec rendering of the tag.

One difference shows the REC was not rendered from the tag unchanged: the published page lacks a NASA link in the `luminance` definition and the informative reference `Luminance-Chromaticity`. Both are outside the cited sections. The body text of the 16 cited sections could not be compared. `main` differs from the tag in 4 cited sections, so SPEC.md must keep citing the tag. Details, hashes and re-fetch commands are in `spec-draft/SOURCES.md`; the crawls and comparison scripts are in `sources/spec-identity/`.

## Gap 4: two image behaviours untested. Closed

`policy-draft/probes/icc_probes.py` puts every runner process under a 3,000,000 KiB address-space limit; results are in `results-icc.json`.

(a) Limits on `probe:dims-65535-square-tiny-data`, which is 69 bytes, not 100 as QUESTIONS.md said (corrected):

- `image::load_from_memory` returns "Memory limit exceeded" without allocating.
- `PngDecoder` + `DynamicImage::from_decoder`, the APNG path and glycin abort on a failed 17,179,344,900-byte allocation.

(b) Damaged ICC profiles. png swallows every iCCP error, so the pixels are identical in all 16 iCCP cases.

- `icc_profile()` returns `None` for:
  - bad zlib data, a truncated stream or a wrong Adler-32;
  - compression method 1;
  - an empty name, no NUL, or no method byte.
- It returns the profile for:
  - a name with a leading space or a control character;
  - trailing bytes after the zlib stream.
- An 80-byte name, one over the limit: png 0.18.1 accepts it, glycin's png revision reports no profile. This is a new divergence between the two png versions.
- A 1 MB iCCP chunk inflating to 1 GiB comes back from image and glycin as a 1 GiB profile (no limit is applied). With png's default limits (the `pngcrate` runner) the profile is dropped.
- GDK's `icc-profile` option was not measured.

## Gap 5: three image behaviours not modelled. Closed

All three are bounded rules in ImageCrateCompat-0.25.10.

- New field `subframe_interlace=CanvasStride`. png 0.18.1 expands an interlaced fdAT frame with the canvas row stride (`decoder/mod.rs:453–466`) into a canvas-sized buffer (image `png.rs:420–430`). image then reads the first `fw·fh` pixels.
- `apng_errors=Prefix` and `apng_in_still=ImageRs` now let a valid fcTL replace a preceding fcTL that has no image data (png keeps only the latest frame control). This covers both `vec:apng-bad/fctl-no-fdat-then-fctl` and `vec:apng-bad/two-fctl-before-idat`.

Fitted agreement rose from 720 to 721 (still) and 718 to 721 (frames). Neither rule was tuned on held-out results, although `held:105` (set A) shows the second quirk and was looked at before the change.

## Gap 6: no hold-out or necessity check. Closed

(a) Held-out sets A and B are deterministic mutations of the fitted vectors and probes. The mutations are byte flips with and without CRC repair, zlib-data flips, chunk deletion, duplication, moves and swaps, length edits, truncation, IDAT splits and appended bytes. Each set has 400 inputs of at most 2,077 bytes. Both were frozen before any runner saw them; the figures are in the table above.

Set A's GdkCompat misses guided the gap-2 reading, so set B, frozen afterwards, is the clean measurement: GdkCompat 389/395 against 721/721 on the fitted set, and ImageCrateCompat 395/395 (still) and 394/395 (frames) against 721/721. New divergences, recorded as residuals in OPTIONS.md and not modelled:

- libpng does not check the type of the chunk that ends the IDAT run (`held:075`).
- libpng stops with "Not enough compressed data" when that chunk's header arrives before the zlib stream ends, even if the file then ends (`heldB:269`).
- libpng ignores every PLTE error in non-palette images, including a duplicate PLTE in an RGB image (`held:279`, `heldB:274`).
- image delivers an APNG frame whose zlib stream is complete even if the next chunk header is broken (`held:047`).
- png rejects an interlaced stream that overruns the raster and is then truncated (`held:156`).

(b) `policy-draft/necessity.py` covers 1,520 inputs (fitted, large and both held-out sets; 28 beyond a cap excluded). Fields whose reversion to Strict breaks nothing:

- GdkCompat's `missing_iend`, `apng_in_still`, `apng_errors` and `apng`.
- ImageCrateCompat's `iend`, `trailing` and `zlib_tail`.

Each is subsumed by another field (OPTIONS.md, Named profiles). Every other compat field breaks between 1 and 168 inputs.

## Gap 7: 19 inputs above 4 KiB not compared. Closed, with a correction

Only 14 inputs exceed 4 KiB: three vectors, five probes (`iccp-bomb-100m`, `ztxt-bomb-64m`, `itxt-bomb-64m`, `text-many-20000`, `text-large-8m`) and six witnesses. `probe:dims-100000x1` (177 bytes) and `dims-16385x16` (334 bytes) always had their bytes stored. They are excluded from agreement as beyond the P1 caps, not for size.

`run_probes.py` now stores inputs above 4 KiB by reference: `hex` is null, `ref` names the generator, and `resolve()` regenerates the bytes and checks the SHA-256. `validate.py` compares the 14: all four compat comparisons agree on 14/14.

`run_probes.py --large --as-kib 3000000` reran the 14 inputs and the two dims probes with all twelve runners under a 3,000,000 KiB address-space limit (`results-large.json`). Every outcome equals the 768 MiB run; every runner accepts the dims probes.

## Gap 8: corpus licences. Closed for what GitHub holds

`policy-draft/CORPUS-LICENCES.md` has a 16-row table from licence texts fetched from the source repositories; the copies are in `sources/licences/`. Corrections to BRIEF.md's licence column:

- libpng `contrib/pngsuite` is van Schaik's freeware grant, not the libpng licence.
- image-png `tests/` mixes PngSuite freeware, a CC-BY-2.0 file and CC-BY-SA-3.0 bench images. There is no `v0.18.1` tag; the release is commit `2a3f980`.
- Wuffs is Apache-2.0 OR MIT, with third-party exceptions.
- The APNG suite is by Philip Taylor, not Philip Jägenstedt. Its generator in the web-platform-tests port is MIT.
- Adwaita is LGPL-3.0-only OR CC-BY-SA-3.0-US.

BRIEF.md now points to the file. Still unverified, because the hosts are blocked:

- schaik.com's own PngSuite page;
- Philip Taylor's original page;
- upstream hicolor-icon-theme;
- per-file provenance of the Chromium, Mozilla and gdk-pixbuf test PNGs.

## Gap 9: OPTIONS.md residual statement. Closed

"Three vectors and six witnesses" plus the five named probes was a correct count of the 14 uncompared inputs. The dims probes were never among them. The residual now says that the 14 inputs are compared and why the dims probes are excluded.

## Files

Changed:

- `png/.gitignore`
- `BRIEF.md`
- `policy-draft/OPTIONS.md`
- `policy-draft/QUESTIONS.md` (facts and gap-closed notes only; questions and recommendations unchanged)
- `policy-draft/policy_oracle.py`
- `policy-draft/validate.py`
- `policy-draft/probes/run_probes.py`
- `policy-draft/probes/runner/src/main.rs` (selectors `image-load`, `image-icc`)
- `spec-draft/DESIGN-NOTES.md` (N16 corrected, N17 added)
- `spec-draft/SOURCES.md`

New:

- `GAPS-CLOSED.md`
- `policy-draft/CORPUS-LICENCES.md`
- `policy-draft/necessity.py`
- `policy-draft/probes/heldout_def.py`
- `policy-draft/probes/icc_probes.py`
- `policy-draft/probes/glycin-runner/` (`Cargo.toml`, `Cargo.lock`, `src/main.rs`, `README.md`; `src/glycin_png.rs` is a copy of glycin code under MPL-2.0 OR LGPL-2.1-or-later, so it is git-ignored and recreated as the README describes)
- `policy-draft/probes/results-glycin.json`, `results-icc.json`, `results-large.json`

Ignored (regenerable):

- `results-heldout.json` (1.25 MB) and `results-heldoutB.json` (1.19 MB)
- `sources/`: libpng, zlib, licence texts, spec-identity crawls

Rerun everything:

```sh
cd policy-draft/probes
python3 -I run_probes.py
python3 -I run_probes.py --runners glycin,image-load --out results-glycin.json
python3 -I run_probes.py --set heldout --runners image,image-still,pngcrate,libpng,spng,stb,gdk-loader,gdk-loader-byte,gdk-file,gdk-anim,glycin,image-load
python3 -I run_probes.py --set heldoutB --runners image,image-still,pngcrate,libpng,spng,stb,gdk-loader,gdk-loader-byte,gdk-file,gdk-anim,glycin,image-load
python3 -I run_probes.py --large --as-kib 3000000 --runners image,image-still,pngcrate,libpng,spng,stb,gdk-loader,gdk-loader-byte,gdk-file,gdk-anim,glycin,image-load --out results-large.json
python3 -I icc_probes.py
cd .. && python3 -I validate.py && python3 -I necessity.py
```
