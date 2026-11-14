# PNG policy option catalogue (draft 0.1, not frozen)

This draft extends PNG-CORE-SPEC draft 0.1 (`../spec-draft/SPEC.md`, P1–P21) the way the frozen DIB and TGA policy specs and the GIF policy draft extend their cores. The stable entry points `decode_still`/`decode_still16`/`decode_frames`/`icc_profile` stay Strict. Profiled entry points take one of four named profiles (`Strict`, `RecommendedBeta`, `GdkCompat-2.42.12`, `ImageCrateCompat-0.25.10`) and add `recovered: bool` to the still result and to every frame. `Policy::default()` is Strict; the profile selector is `#[non_exhaustive]`; the 29-field record below is internal and constructible only under `#[cfg(test)]`. The reference model is `policy_oracle.py`; `validate.py` checks the Strict bridge, obligation R and agreement with the incumbents.

**Non-negotiable for every policy value**, including test-only records: P1's caps (`MAX_INPUT`, `MAX_DIMENSION`, `MAX_OUTPUT`, `MAX_FRAMES`, `MAX_TOTAL_PIXELS`, `MAX_ICC`) with their check order; P20's reservation sites and no other input-dependent allocation; every inflate bounded by its output limit (a relaxed tail check stops at the limit, it never raises it); no read beyond the slice, unchecked overflow or narrowing, panic or nontermination; work `steps ≤ c·(|b| + R + N + P)`. A native decoder's acceptance beyond these caps is not part of any profile (`probe:dims-100000x1`, `-16385x16`, `vec:ihdr/width-16385`, `vec:apng-bad/actl-too-many-frames` are accepted by the incumbents and excluded from agreement counts). The Python oracle allocates freely; its memory behaviour is not evidence about a Rust implementation.

## The `recovered` flag

For every retained record, the still result's `recovered` (and a frame's) is true **iff** at least one of these data-affecting events occurred while producing it:

- **R1 transparency dropped:** in a colour type 0, 2 or 3 image, a tRNS chunk was discarded or shortened under `crc_ancillary=Skip`, `ancillary_order`, `ancillary_duplicate=FirstWins` (a later tRNS) or `trns_invalid`.
- **R2 surplus image data discarded:** the image or frame's zlib stream produced more than `R` bytes and the surplus was discarded (`zlib_tail ∈ {Ignore, StopAtRaster}`, `stream_end=AtRaster`).
- **R3 palette substitution:** an index `≥ E` was drawn as `(0,0,0,255)` (`palette_index=Black`).
- **R4 animation fallback:** `apng_errors=Static` replaced an invalid animation by the default image (frames entry only).
- **R5 partial image:** rows that were never decoded were filled with `(0x97,0x98,0x99,0xFF)` (`image_data_error=GdkGrey`, `eof=GdkPartial`).

No other tolerance sets the flag: an ignored ancillary CRC on a chunk other than tRNS, dropped misplaced, duplicate or invalid metadata, trailing bytes, a missing or damaged IEND, a CINFO window violation, an ignored Adler-32 under `adler=Ignore`, APNG chunks ignored by `decode_still`, extra frames beyond `num_frames`. Strict never sets it. So `recovered=false` does not imply that Strict accepts the file.

## Option fields

Values in **bold** are Strict. Evidence: gdk-pixbuf 2.42.12 source copies in `../sources/gdk-pixbuf-2.42.12/` (cited `io-png.c:N`); png 0.18.1, fdeflate 0.3.7 and image 0.25.10 from `~/.cargo/registry` (cited `png stream.rs:N`, `image png.rs:N`); `probe:`, `vec:` and `wit:` names refer to `probes/results.json` (`wit:` are the 57 witness files of the earlier leniency run, `~/projects/leniency/png/witnesses/`). "GDK" means the `GdkPixbufLoader` whole-buffer runner unless named otherwise; "image" means `PngDecoder` with `DynamicImage::from_decoder` (still) or the APNG frame loop (frames). libpng is the library gdk-pixbuf links, so GDK and libpng are one family; libpng's own `.c` was not read (UNVERIFIED reasons, measured results).

| Field | Values and exact delta from Strict | Incumbent behaviour, evidence |
|---|---|---|
| `crc_critical` | **`Reject`**. `Ignore`: use a critical chunk whose CRC fails. | GDK, libpng, image reject (`probe:crc-idat`, `-plte`, `-ihdr`); stb ignores all CRCs, libspng ignores the IDAT CRC. No profile uses `Ignore`. |
| `crc_ancillary` | **`Reject`**. `Skip`: an ancillary chunk with a bad CRC is discarded as if absent (R1 for tRNS). `Ignore`: it is used. | libpng/GDK (`png.h:1440–1454`, `PNG_CRC_DEFAULT`, gdk-pixbuf never calls `png_set_crc_action`) and png (`skip_ancillary_crc_failures=true`, `stream.rs:474–484`) skip: `probe:crc-text`, `-gama`, `-unknown-ancillary`, `-trns` (alpha lost in every decoder but stb). |
| `iend` | **`Strict`**: IEND CRC and zero length checked. `Lenient`: IEND's CRC and length are ignored. | Every runner accepts `probe:crc-iend`, `vec:container/iend-length-1`, `probe:trailing-iend-length-1`, `wit:03-length-IEND` (lodepng and zune reject in the earlier run). |
| `chunk_type` | **`Reject`**: P3 step 2. `Ancillary`: a type with non-letter bytes is an unknown chunk, critical iff bit 5 of byte 0 is clear. | libpng/GDK reject ("bad header"); image, libspng, stb accept (`probe:type-nonletter`; image rejects `IDA ` as an unknown critical chunk, `vec:container/type-space`). |
| `reserved_bit` | **`Accept`** ([3e #13Chunking]). `Reject`. `RejectBeforeIdat`: reject a set reserved bit before the first IDAT only. | GDK rejects before IDAT (`probe:reserved-bit-ancillary`) but accepts after it (`vec:container/unknown-ancillary-everywhere`, `probe:crc-text-after-idat-image`); image, libspng, stb accept. |
| `after_idat` | **`Full`**: P5 to IEND. `ImageRs`: for a static image, once the IDAT run ends nothing is examined; 8 bytes of a following chunk header must exist, else `TruncatedChunk`. | png `read_decoder.rs:128–134`, `stream.rs:876–900`: `probe:crc-iend`, `-text-after-idat`, `unknown-critical-after-idat`, `idat-then-iend-no-crc` accepted; `probe:truncated-after-idat`, `idat-then-4-bytes` rejected ("unexpected end of file"). |
| `missing_iend` | **`Reject`**. `AtEof`: EOF at a chunk boundary after the first IDAT acts as IEND. | GDK and libpng accept (`probe:missing-iend-after-text`, `truncated-after-idat`; libpng reports a tail error); `gdk_pixbuf_new_from_file` rejects (`io-png.c:344–345`, `png_read_end`); image: see `after_idat`. |
| `eof` | **`Reject`**. `GdkPartial`: EOF anywhere after an IDAT chunk header acts as IEND; rows not decoded become grey (R5). | `stop_load` succeeds whenever a pixbuf was allocated (`io-png.c:555–566`, "FIXME … unused image data"); prefill `0x979899ff` (`io-png.c:41, 721`): `probe:truncated-mid-idat`, `idat-then-4-bytes`. |
| `trailing` | **`Reject`**: `TrailingBytes`. `Ignore`. | Every runner ignores bytes after IEND (`probe:trailing-garbage`, `-second-png`). |
| `ancillary_order` | **`Reject`**: `ChunkOutOfOrder`. `Drop`: a misplaced ancillary chunk is discarded (R1 for tRNS). `ImageRs`: as `Drop`, except bKGD before PLTE in a palette image is an error. | libpng/GDK and png drop: `probe:order-gama-after-plte`, `-gama-after-idat`, `-srgb-after-idat`, `-iccp-after-idat`, `-trns-before-plte`, `-trns-after-idat-*`, `-phys-after-idat` (identical pixels; tRNS ignored). png rejects bKGD before PLTE (`stream.rs:1939–1943`; `probe:order-bkgd-before-plte`). stb rejects misplaced tRNS. |
| `ancillary_duplicate` | **`Reject`**: `DuplicateChunk`. `FirstWins`. | `probe:dup-gama`, `-srgb`, `-time`, `-phys` accepted everywhere; `probe:dup-trns-palette`: GDK, libpng, image use the first tRNS, stb the second. |
| `ancillary_values` | **`Reject`**: P8.1 errors and wrong fixed lengths of ancillary chunks. `Drop`: the chunk is discarded. `ImageRs`: `Drop`, except `InvalidCicp` stays an error. | All accept: `probe:gamma-0`, `srgb-intent-4`, `iccp-method-1`, `cicp-len-5`, `bkgd-index-oob`, `phys-unit-2`, `exif-bad-magic`, `time-invalid`, `sbit-zero`, `hist-wrong-length`, `text-*`, `ztxt-method-1`, `itxt-flag-2`. png treats an invalid cICP value as a fatal I/O error (`stream.rs:1556–1578`; `probe:cicp-matrix-1`). |
| `trns_invalid` | **`Reject`**: `InvalidTransparency`. `Drop` (R1). `ImageRs`: a grey or RGB tRNS longer than required is truncated to its first 2 or 6 bytes (R1); any other invalid tRNS is dropped (R1). | GDK/libpng drop (`probe:trns-palette-long`, `-palette-empty`, `-rgb-len-4`, `-rgb-len-8`, `-grey-len-1`, `-in-rgba`, `-in-grey-alpha`); png keeps a prefix (`stream.rs:1361–1384`; `probe:trns-rgb-len-8`: pixel 0 transparent in image only). stb rejects. |
| `plte_unexpected` | **`Reject`**: PLTE in colour types 0/4 (`UnexpectedPalette`) and PLTE after IDAT in types 2/6 (`ChunkOutOfOrder`). `Ignore`: such a PLTE is ignored. | All accept: `probe:plte-in-grey`, `order-plte-after-idat-rgb`. |
| `palette_size` | **`Reject`**: more than `2^d` entries is `InvalidPalette`. `Allow` (still at most 256). | All but libspng accept `probe:plte-more-than-depth` (indices cannot exceed `2^d − 1`, so pixels are unaffected). |
| `palette_index` | **`Reject`**: `PaletteIndexOutOfRange`. `Black`: `(0,0,0,255)`, R3. | GDK, libpng and image agree on opaque black (`probe:index-oob-8bit`, `-1bit`, `-with-trns`, `-2bit-trns`; png `palette.rs:57, 86–88`); stb differs. |
| `adler` | **`Check`**. `Ignore`. | png ignores by default (`ignore_adler32=true`, `zlib.rs:64–78`): `probe:adler-bad` accepted by image; libpng and libspng reject; the GDK loader loses the last row (see `image_data_error`) and with one-byte writes accepts the full image (path dependence, not reproduced). |
| `stream_end` | **`Require`**: P9 steps 5–6. `AtRaster`: once `R` bytes are produced, a missing or partial Adler-32, an unfinished final block and any following data are accepted; surplus output is discarded (R2). | png stops when the output buffer is full (`zlib.rs:146–152`): `probe:adler-missing`, `-partial` accepted by image only. |
| `zlib_tail` | **`Reject`**: `TrailingZlibData`, `ExcessImageData`. `Ignore`: bytes after the Adler-32 are ignored and surplus output is discarded (R2) but the Adler-32 still covers the whole output. `StopAtRaster`: inflating stops at the first surplus byte (R2); the rest of the stream, including its Adler-32, is not examined. | Every runner accepts `probe:zlib-trailing-in-idat`, `-extra-idat-zeros`, `-second-stream`, `excess-data-byte`, `-row`. GDK accepts `vec:deflate/match-overruns-image-is-excess` although its Adler-32 is wrong, so libpng stops at the raster (`StopAtRaster`, inferred from that probe). |
| `window` | **`Enforce`**: `DistanceTooFar` beyond `2^(CINFO+8)`. `Ignore`: only the output so far bounds the distance. | Every runner accepts `probe:zlib-window-too-small` (the byte-wise GDK loader with different pixels). |
| `single_code` | **`Accept`** (zlib's rule, P10.3). `Reject`: a literal/length code with a single code is `InvalidCodeLengths`. | fdeflate rejects (`probe:deflate-single-literal-code`: image ✗, libpng/GDK/libspng/stb accept). |
| `image_data_error` | **`Reject`**. `GdkGrey`: a zlib format error in the image data (header, block type, stored length, code lengths, symbol, distance, Adler-32) or `TruncatedImageData` keeps the rows completed before it, except that after a format error the last completed row is also lost; the other rows are grey (R5). `ZlibPresetDictionary` and a non-EOF `TruncatedZlib` stay errors. | **Surrogate** fitted to the GDK loader: `probe:zlib-fcheck`, `-cm7`, `-cinfo8`, `-raw-deflate`, `deflate-btype3`, `-incomplete-dist-code`, `-empty-fixed-block`, `too-little-data`, `-one-row`, `adler-bad`, `vec:deflate/length-with-no-distance-codes`; `probe:zlib-fdict` and `adler-missing` rejected. The libpng mechanism behind the lost row is UNVERIFIED (libpng's progressive reader was not read). |
| `depth16` | **`Round`**: P13.2. `HighByte`: `v >> 8`. | GDK uses `png_set_strip_16` (`io-png.c:108–110`, `png.h:1333`): `probe:depth16-rgba`; image rounds (`color.rs:549`). Pixel rule, not a leniency: RecommendedBeta must keep Strict's value (obligation R). |
| `apng_in_still` | **`Validate`**: `decode_still` runs P14.1. `Ignore`: `decode_still` treats acTL, fcTL, fdAT as unknown ancillary chunks. `ImageRs`: APNG chunks before the first IDAT are validated (an fcTL there must match IHDR, sequence 0, length 26; fdAT before IDAT is an error), invalid acTL values make acTL ignored, later chunks are never read (`after_idat`). | Static decoders ignore APNG: libpng, GDK loader, libspng, stb show the default image on every `apng-bad` input. image's still path rejects only the pre-IDAT cases (`vec:apng-bad/seq-starts-at-1`, `fdat-before-idat`, `fctl-len-25`, `default-fctl-smaller`; png `stream.rs:1962–1981`). |
| `apng_errors` | **`Reject`**: P14.1 errors fail `decode_frames`. `Static`: on any APNG structure error the animation is replaced by the default image (R4). `Prefix`: acTL with `num_frames = 0` or a value above `2^31−1`, a misplaced or duplicate acTL, and fcTL/fdAT without a valid acTL are ignored; frames beyond `num_frames` are not read; any other error after the first IDAT ends the sequence after the frames whose data was complete; an error before any frame completes fails the call. | png and image (`mod.rs:229–242`, `read_decoder.rs:96–100`, `image png.rs:344–347`): `probe:apng-seq-gap`, `-seq-fdat-dup`, `-fctl-no-fdat`, `-frame-outside`, `-dispose-3`, `-blend-2`, `-fewer-frames`, `-more-frames`, `-actl-zero`, `-actl-after-idat`, `-fctl-without-actl`, `-actl-no-fctl`, `-fdat-before-fctl`, `wit:34-length-fdAT`. `Static` is what a decoder without APNG support shows; no measured APNG decoder does it. |
| `blend` | **`Exact`**: P14.4. `ImageF32`: image's `Rgba::blend` in f32 with truncating casts. | `color.rs:868–925`; `probe:apng-blend-over`, `apng-ok-3-frames`. The f32 model is a Python emulation; a verified f32 bridge is not planned (PLAN.md P2). |
| `dispose_previous` | **`Spec`**: P14.3. `ImageRs`: frame 0's PREVIOUS is kept; PREVIOUS restores from a buffer that image updates only when the preceding disposal was NONE or BACKGROUND, and then before clearing. | `image png.rs:375–412, 450`: `vec:apng/dispose-previous-first-frame-and-later` differs from the spec. `probe:apng-bg-then-previous-4` and `apng-previous-chain` happen to agree. |
| `apng16` | **`Composite`**: P14.4 after P13.2. `Reject`: a 16-bit APNG fails `decode_frames`. | image rejects 16-bit APNG (`png.rs:502–511`; `probe:apng-16bit`). |
| `apng` | **`On`**. `Off`: acTL, fcTL, fdAT are unknown ancillary chunks everywhere; `decode_frames` returns the default image. | GDK: libpng 1.6.48 has no APNG patch (`png.h`); the loader shows the default image; `gdk_pixbuf_new_from_file` and the animation API reject APNG files by MIME sniffing (`probe:apng-ok-3-frames`: "Couldn't recognize"). |

Operational order is P18's wherever a check still exists. A relaxed check never moves another check earlier. `Drop`/`Skip` act after the chunk's CRC and length have been read, so the walk stays aligned.

### Presentation and metadata fields (not part of the pixel record)

| Field | Values | Evidence |
|---|---|---|
| `delay_map` | **`Raw`**: `(delay_num, delay_den)`. `Image`: `den = 0 → 100`, then milliseconds. `Glycin`: as `Image`, then `0 ms → 100 ms`. | `image png.rs:541–547`; glycin `animated.rs:94–103` (source reading); `probe:apng-delay-den-0`: image reports 70 ms and 0 ms. GDK has no APNG. |
| `icc_errors` | **`Reject`**: `icc_profile` returns the error. `None`: a damaged profile is reported as absent. | png swallows every iCCP error, including its allocation limit (`stream.rs:1696`; `probe:iccp-bad-zlib`, `iccp-bomb-100m`: image decodes the pixels; the profile result was not captured by the runner, UNVERIFIED). GDK exposes the profile base64-encoded as the `icc-profile` option (`io-png.c:356–366`). |
| `text` | **`NotExposed`**. | GDK exposes tEXt/zTXt/iTXt as `tEXt::key` options and converts iTXt as if it were Latin-1 (`io-png.c:211–241`); png decompresses lazily with a 2 MiB default (`text_metadata.rs:108`). |

## Named profiles

| Field | Strict | RecommendedBeta | GdkCompat-2.42.12 | ImageCrateCompat-0.25.10 |
|---|---|---|---|---|
| crc_critical | Reject | Reject | Reject | Reject |
| crc_ancillary | Reject | Skip | Skip | Skip |
| iend | Strict | Lenient | Lenient | Lenient |
| chunk_type | Reject | Reject | Reject | Ancillary |
| reserved_bit | Accept | Accept | RejectBeforeIdat | Accept |
| after_idat | Full | Full | Full | ImageRs |
| missing_iend | Reject | AtEof | AtEof | Reject |
| eof | Reject | Reject | GdkPartial | Reject |
| trailing | Reject | Ignore | Ignore | Ignore |
| ancillary_order | Reject | Drop | Drop | ImageRs |
| ancillary_duplicate | Reject | FirstWins | FirstWins | FirstWins |
| ancillary_values | Reject | Drop | Drop | ImageRs |
| trns_invalid | Reject | Drop | Drop | ImageRs |
| plte_unexpected | Reject | Ignore | Ignore | Ignore |
| palette_size | Reject | Allow | Allow | Allow |
| palette_index | Reject | Black | Black | Black |
| adler | Check | Check | Check | Ignore |
| stream_end | Require | Require | Require | AtRaster |
| zlib_tail | Reject | Ignore | StopAtRaster | Ignore |
| window | Enforce | Ignore | Ignore | Ignore |
| single_code | Accept | Accept | Accept | Reject |
| image_data_error | Reject | Reject | GdkGrey | Reject |
| depth16 | Round | Round | HighByte | Round |
| apng_in_still | Validate | Ignore | Ignore | ImageRs |
| apng_errors | Reject | Reject | Static | Prefix |
| blend | Exact | Exact | Exact | ImageF32 |
| dispose_previous | Spec | Spec | Spec | ImageRs |
| apng16 | Composite | Composite | Composite | Reject |
| apng | On | On | Off | On |

**Strict** is PNG-CORE-SPEC draft 0.1: `validate.py` finds `policy_oracle(Strict)` equal to `png_oracle` on all 737 distinct vector, probe and witness inputs (still and frames, including the first error).

**RecommendedBeta** changes only error sites. Each of its fourteen non-Strict fields replaces a branch on which Strict returns an error and is inert on every other branch: `Skip` never fires for a chunk whose CRC is right, `Drop` never sees a well-placed chunk, `Ignore` on the zlib tail stops exactly where Strict's checks succeed, `palette_index=Black` never sees an index below `E`, `apng_in_still=Ignore` changes no pixel of the default image. Therefore every image Strict accepts decodes to identical RGBA with `recovered=false` (obligation R; `validate.py`: 364 Strict-accepted still inputs and every Strict frame, 0 violations). Of the 289 vector, probe and witness inputs that Strict rejects and image's still path or the GDK loader accepts, RecommendedBeta accepts 231. These inputs are synthetic; no real-world PNG corpus has been measured (PNG-Q7). The name records that the field set is provisional (owner decision); it becomes `Recommended` only by a later owner decision.

**GdkCompat-2.42.12** and **ImageCrateCompat-0.25.10** are opt-in byte-semantics proposals. Agreement on the 721 inputs that are at most 4 KiB, not beyond a cap and not crashing the runner (accept/reject, dimensions, SHA-256 of the RGBA8 result): GdkCompat vs the GDK loader 721/721 (vs `gdk_pixbuf_new_from_file`, which it does not target, 616/721); ImageCrateCompat vs image's still path 720/721 and vs image's frame loop 718/721. For comparison, Strict agrees with image's still path on 495/721 and with the GDK loader on 378/721. **None of these numbers is a hold-out measurement**: the probe results and the witnesses were visible while the compatibility fields were fitted, and `image_data_error=GdkGrey`, `zlib_tail=StopAtRaster`, `reserved_bit=RejectBeforeIdat`, `apng_in_still=ImageRs`, `apng_errors=Prefix` and `dispose_previous=ImageRs` were each changed after a mismatch. A necessity check (reverting one field at a time, as `../../gif/policy-draft/necessity.py` does) was not run.

## Residuals (measured, not reproduced)

- **png 0.18.1, interlaced APNG subframes:** `probe:apng-interlaced` (a 3×3 interlaced frame on a 4×3 canvas) is garbled by image; source reading attributes it to `mod.rs:454` using the canvas width as the row stride (fixed in glycin's png revision). Not modelled.
- **image, fcTL replacing a pending fcTL:** `vec:apng-bad/fctl-no-fdat-then-fctl` (image draws the second fcTL's frame with the first's slot) and `vec:apng-bad/two-fctl-before-idat` (image's still path accepts it, its frame loop shows one frame then an error). Not modelled.
- **image panics** on a PLTE whose length is not a multiple of 3 (`vec:plte/length-8`, `probe:plte-len-13`, `-14`); the runner catches the panic, a real caller may not (DESIGN-NOTES N16). Every profile rejects these files.
- **GDK path dependence:** the whole-buffer loader loses the last row on a bad Adler-32 while one-byte writes show the full image (`probe:adler-bad`); one-byte writes also differ on `probe:zlib-window-too-small`. GdkCompat models the whole-buffer loader only.
- **Inputs not compared** (larger than 4 KiB, so `results.json` stores only their hash): the bombs (`probe:iccp-bomb-100m`, `ztxt-bomb-64m`, `itxt-bomb-64m`), `probe:text-many-20000`, `text-large-8m`, three vectors and six witnesses. Their runner outcomes are in `results.json`; every incumbent except libspng accepted the bombs and text inputs.

## Not reproducible, explicitly excluded

- Acceptance beyond the P1 caps (image and GDK accept 16,385-pixel and 100,000-pixel widths, image accepts `num_frames` above 65,536); allocation failures (image aborts on `probe:dims-65535-square-tiny-data` under a 768 MiB address-space limit; GDK reports "Insufficient memory").
- libpng's warnings, gdk-pixbuf's option strings, image's metadata accessors: not part of the pixel result.
- libspng, stb, lodepng, zune and minipng behaviours (CRC indifference, inventing palettes, rejecting tRNS variants): measured in the earlier leniency run and here for libspng and stb; no profile targets them.

## Recorded decisions and open boundary

- D1: the stable entry points are Strict; profiled entry points serve named profiles only; arbitrary records are `#[cfg(test)]`.
- D2: RecommendedBeta contains only error-site relaxations; every pixel-rule field (`depth16`, `blend`, `dispose_previous`) keeps Strict's value.
- D3: truncated image data, zlib damage and APNG structure errors remain errors in Strict and RecommendedBeta (PNG-Q10, PNG-Q11).
- D4: `recovered` is the closed five-event list R1–R5.
- D5: profile names carry the measured library version and are immutable once frozen; `RecommendedBeta` is the owner's name for the provisional recommended profile.
- Open: PNG-Q1–Q16 in `QUESTIONS.md`.
