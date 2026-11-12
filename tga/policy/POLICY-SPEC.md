# TGA policy specification (frozen candidate, not frozen)

This document defines a byte-only extension of frozen TGA-CORE-SPEC 1.0 (`../spec-frozen/SPEC.md`, R1–R17). The 1.0 rules, error names, R14 precedence, RGBA8 layout and limits remain the default. The stable `decode(bytes)` keeps its result type and behaviour. A separate `decode_with_policy(bytes, profile)` takes one of four named profiles and returns width, height, RGBA8 and `recovered: bool`. `Policy::default()` is Strict; the profile selector is `#[non_exhaustive]`. The record of individual field values is internal: Rust code may construct an arbitrary record only under `#[cfg(test)]`, and the Python oracle's `EXPERIMENTAL_OPTIONS` switch controls the equivalent reference-model probes. This candidate does not choose glycin's default profile (QUESTIONS.md Q6).

**Non-negotiable for every policy value**, including test-only records: `MAX_INPUT = MAX_OUTPUT = 67,108,864` bytes and `MAX_DIMENSION = 8192` with the 1.0 check order; exactly one output allocation of `4N` bytes on success, zero header-error allocations and no other input-dependent allocations (no input-sized scratch buffer, palette cache or row buffer; THEOREMS-POLICY-DELTA.md obligation A); no read beyond the slice, integer overflow, unchecked narrowing, panic or nontermination for any bytes; work linear in `|b| + N` (THEOREMS-POLICY-DELTA.md obligation W: `steps <= c*(|b|+N)`, proved for the implementation with a constant independent of the oracle), with clipped RLE positions costing zero loop iterations. A native decoder's acceptance beyond these caps (gdk-pixbuf accepts 65,535-pixel dimensions) is not part of any profile. The Python oracle allocates temporaries and caches conversions; its memory behaviour is not evidence about a Rust implementation.

## The `recovered` flag

For every retained field record, `recovered` is true **if and only if** at least one of these data-affecting events occurs during a successful profiled decode (THEOREMS-POLICY-DELTA.md obligation E):

- **RLE clip:** under `rle_overrun = ClipPacket | ClipSamples`, an RLE packet's count exceeds the pixels remaining, so at least one encoded position is discarded.
- **Index substitution:** under `index_range = Black`, an out-of-range palette index is replaced by black at a pixel write.

No other tolerance sets the flag: ignored trailing bytes, a repaired attribute count (`Lenient`), ignored or relaxed colour-map fields, zero-based palette lookup, alternative alpha or 5-bit expansion rules, or a narrower depth table. Strict never sets it. Consequently `recovered = false` does not imply that Strict accepts the file: Recommended accepts metadata-only defects with the flag false.

## Option fields

Values in **bold** are Strict. Evidence cites gdk-pixbuf 2.42.12 `io-tga.c` (`/home/ws/projects/spec/gdk-pixbuf/gdk-pixbuf/io-tga.c`), image 0.25.10 `src/codecs/tga/decoder.rs` and tinytga 0.5.0, all read for this draft; corpus counts refer to the 300 measured real files in `/home/ws/projects/leniency/corpora/NOTES.md`. Every value is a deterministic function of the bytes and is safe when implemented with guarded reads and checked arithmetic.

| Field | Values and their exact delta from 1.0 | Evidence; safety |
|---|---|---|
| `trailing` | **`FooterOnly`**: R12. `Ignore`: after pixel N, accept any suffix, including a partial footer. | 4 corpus files carry an unmarked suffix (R12); image-rs and GDK accept all 4, GDK skips the rest (`io-tga.c:184–190`). No read of the suffix. |
| `rle_overrun` | **`Reject`**: R11 returns `RleOverrun` before reading packet data. `ClipPacket`: require the whole packet (`1+sample_bytes` for repeat, `1+count*sample_bytes` for literal) or return `RleTruncated`; paint `min(count, N-produced)` samples; advance the cursor past the whole packet. `ClipSamples`: as ClipPacket, but a literal packet requires and consumes only the `min(count, N-produced)` samples it paints; a repeat packet still requires its one sample. Indices of unpainted samples are never looked up. | 2 corpus files end with an over-long packet; all four libraries accept both. GDK requires the whole literal packet (`io-tga.c:449–453`); image-rs reads only the painted bytes (`decoder.rs:271–277`). The two values differ only on a truncated final literal packet, or in where the cursor stops when `trailing=FooterOnly`. |
| `attribute_bits` | **`Exact`**: R5. `Lenient`: a count R5 rejects is replaced, for every later rule, by an effective count of 8 if R5 allows 8 for that element (32-bit true-colour or palette entry, 16-bit grey) and by 0 otherwise. `ImageTable`: allowed counts as R5 except 8-bit grey allows {0, 8} (8 is treated like 0) and 16-bit grey allows only {8}. | 5 corpus files fail R5 (32-bit with count 4 ×3, 16-bit with count 8, 24-bit palette with count 8); GDK accepts all 5 because it never reads the count. image-rs derives its colour type from `(count, depth-count)` (`decoder.rs:143–183`). |
| `map_header` | **`Core`**: R2 flag ∈ {0,1}, R4 coherence and range. `IgnoreAbsent`: as Core, but flag 0 on a non-indexed image skips the check that `first`, `length` and entry depth are zero; flag 0 still consumes no map bytes, and an indexed image still requires flag 1. `Gdk`: any nonzero flag means a map is present; a present map may have `length=0`, needs entry depth in {15,16,24,32} only when `length>0`, and has no `ColorMapRange` check; an indexed image without a map returns `InvalidColorMap`. `Image`: only flag 1 means present (other values mean absent and consume nothing); a present map needs entry depth in {15,16,24,32} and may have `length=0`; no `ColorMapRange` check. | 7 corpus files fail R4; three (a stray entry depth or garbage map fields with flag 0) are accepted by some library. GDK tests `has_cmap` as a boolean (`io-tga.c:486`); image-rs reads a map only for flag 1 (`decoder.rs:193–207`). Map bytes stay at most 262,140. |
| `depths` | **`Core`**: R3. `Gdk`: indexed 8; true-colour 16, 24, 32; grey 8, 16 (`io-tga.c:258–281`). `Image`: Core, plus an indexed image whose index depth exceeds its entry depth returns `UnsupportedPixelDepth` after the map checks (`decoder.rs:125`). | Narrows acceptance only. |
| `alpha` | **`AttributeBits`**: R8, alpha from the A byte only when the effective count is 8. `Stored`: 32-bit true-colour, 32-bit palette entries and 16-bit grey always take the stored byte as alpha. | Explains all 11 corpus "alpha-only" differences for both GDK (`io-tga.c:314,324`) and image-rs (`(0,32)` maps to RGBA8, `decoder.rs:158`). |
| `map16_alpha` | **`Bit15`**: R8, a 16-bit palette entry with effective count 1 has alpha 255 if bit 15 is set, else 0. `Opaque`: always 255. | GDK, image-rs and tinytga all show these entries opaque (2 corpus files). |
| `rgb5` | **`Replicate`**: R8 `(c<<3)\|(c>>2)` for 15/16-bit true-colour. `Round`: `(255c+15) div 31`. | image-rs's lookup table (`decoder.rs:81–94`) equals Round exactly; GDK replicates (`io-tga.c:299–305`). The rules differ for c ∈ {3,7,24,28}, by one level. 7 image-rs corpus differences. |
| `map5` | **`Replicate`**, `Round`: as `rgb5`, for 15/16-bit palette entries. `GdkSwapShift`: red = `(v & 31) << 3`, green = `((v>>5) & 31) << 3`, blue = `((v>>10) & 31) << 3`, with no low-bit replication. | GDK assigns the low five bits to red (`io-tga.c:499–506`), which swaps red and blue; this explains all 8 GDK palette differences. image-rs rounds (3 differences, plus 2 combined with `map16_alpha`). |
| `map_index` | **`Absolute`**: R4/R8, entry `j = index - first`. `ZeroBased`: entry `j = index`; `first` is ignored. | GDK and tinytga ignore `first` (`io-tga.c:154–164`; tinytga `color_map.rs:54` carries `//TODO: use start_index`). One corpus file (`first=39`) decodes differently under GDK. |
| `interleave` | **`Reject`**: R2 `UnsupportedInterleave` when descriptor bits 6–7 are nonzero. `Ignore`: decode as if they were zero. | image-rs never reads these bits (`decoder.rs:185`: `TODO: validate the rest of the fields in the header`); GDK rejects (`io-tga.c:612`). Found by the mutation sets, not the corpus. |
| `image_type` | **`Core`**: R1 types {1,2,3,9,10,11}. `GreyFallback`: every other type byte, including 0 ("no image data"), is decoded as raw greyscale. | image-rs maps unknown types to `ImageType::Unknown`/`NoImageData`, which are neither colour, mapped nor encoded, so they take the raw grey path (`header.rs:24–61`). Reproduces a defect; only ImageCrateCompat uses it. |
| `index_range` | **`Reject`**: `ColorIndexOutOfRange` unless `0 <= j < length`. `Black`: substitute `(0,0,0,0)` when the palette entry depth is 32 and `(0,0,0,255)` otherwise, and set `recovered`. | GDK returns transparent black, but its pixbuf has alpha only for 32-bit palettes (`io-tga.c:350–352`), so callers see opaque black otherwise. |

Operational order is R14's wherever a check still exists: input cap, header length, type, flag, dimensions, interleave, depth, map checks, attribute count, output cap, ID, map bytes, then pixels/packets in stream order, then suffix. `Image` places its index-depth check immediately after the map checks. Within an RLE packet: control presence, overrun (Reject only), whole-packet or painted-sample availability, then index checks in output order. A relaxed check never moves another check earlier.

## Named profiles

| Field | Strict | Recommended | GdkCompat-2.42.12 | ImageCrateCompat-0.25.10 |
|---|---|---|---|---|
| trailing | FooterOnly | Ignore | Ignore | Ignore |
| rle_overrun | Reject | ClipPacket | ClipPacket | ClipSamples |
| attribute_bits | Exact | Lenient | Lenient | ImageTable |
| map_header | Core | IgnoreAbsent | Gdk | Image |
| depths | Core | Core | Gdk | Image |
| alpha | AttributeBits | AttributeBits | Stored | Stored |
| map16_alpha | Bit15 | Bit15 | Opaque | Opaque |
| rgb5 | Replicate | Replicate | Replicate | Round |
| map5 | Replicate | Replicate | GdkSwapShift | Round |
| map_index | Absolute | Absolute | ZeroBased | Absolute |
| index_range | Reject | Reject | Black | Reject |
| interleave | Reject | Reject | Reject | Ignore |
| image_type | Core | Core | Core | GreyFallback |

**Strict** is the shipped 1.0 behaviour; the oracle reproduces all 169 frozen vectors, and validate.py compares it with the shipped Verus core.

**Recommended** changes only error sites. Each of its four fields alters a branch on which Strict returns an error (`TrailingBytes`, `RleOverrun`, `InvalidAttributeBits`, `InvalidColorMap`), and on every other branch computes exactly what Strict computes: `Lenient` sets the effective count to the descriptor count whenever R5 allows it; `IgnoreAbsent` is inert when the flag-0 fields are zero; `ClipPacket` never fires when every packet fits; `Ignore` is inert on an empty suffix or a footer. Therefore every file Strict accepts decodes to identical RGBA with `recovered=false` (THEOREMS-POLICY-DELTA.md obligation R; checked on every corpus and synthetic input in REPORT.md). On the corpus it accepts 14 more real files (of the 19 Strict-rejected files that some library decodes) and leaves rejected the truncated files (R10) and the files whose defect is not one of these four.

**GdkCompat-2.42.12** and **ImageCrateCompat-0.25.10** are opt-in byte-semantics proposals measured against gdk-pixbuf 2.42.12 (forced TGA loader, whole buffer) and image 0.25.10. They reproduce every measured corpus pixel difference exactly; REPORT.md gives their agreement on all inputs, of which only the seed-4242 mutation set was held out from tuning (the corpus, vector and tuned-seed figures are regressions). They are not claims of complete native parity; known residuals are listed below. glycin's in-tree image-rs path is the same TGA code as image 0.25.10 and is not a second vote.

## Measured cause of every corpus pixel difference

All 85 decoder/file pairs in the corpus's `pixdiff.tsv` (41 distinct files; 20 gdk-file, 20 gdk-forced-whole, 23 image, 22 tinytga) are reproduced byte-exactly by the oracle under each decoder's chosen record (`investigate_pixdiff.py`). Reverting one field at a time shows that each listed field is necessary within the chosen record; sufficiency/uniqueness not shown: another record could also reproduce a pair. The two tinytga post-transforms (no bit-4 mirror, the runner's forced alpha 255) are harness-side adjustments that no independent check validates.

- **image-rs** (23): stored alpha with count 0 on 32-bit pixels or palettes (11); rounding 5-bit expansion of true-colour (7) and palette (3) samples; rounding plus opaque 16-bit palette entries with count 1 (2).
- **gdk-pixbuf** (20, identical for file and forced loaders): stored alpha (11); red/blue-swapped, unreplicated 15/16-bit palettes (6, plus 2 that also ignore bit 15); zero-based lookup ignoring `first` with black for indices past the map (1).
- **tinytga** (22): `pixels()` ignores descriptor bit 4 and never mirrors right-origin images (14, the "horizontal flips"); rounding 5-bit expansion (6); rounding plus the runner's forced alpha 255 on 16-bit palettes with count 1 (2). tinytga supports neither 32-bit pixels nor 32-bit palettes, so it shows no alpha-only difference.

No difference involves gamma, premultiplication, vertical orientation or palette entry size.

## Not reproducible, explicitly excluded

- Dimensions above 8,192 or output above 64 MiB, which GDK and image-rs accept.
- GDK's filename-based loader selection: one corpus file decodes with the forced TGA loader but not through `gdk_pixbuf_new_from_file`.
- Partial images that GDK's progressive loader exposes before `stop_load` reports truncation (`io-tga.c:707`).
- tinytga's zero-filled truncated raw data and its panic on an out-of-range palette index.
- Any behaviour depending on allocator contents, I/O, callbacks or the TGA 2.0 extension area (still ignored, as in R12).

## Recorded decisions and open boundary

- D1: `decode` is unchanged; `decode_with_policy` serves named profiles only; arbitrary records are `#[cfg(test)]`.
- D2: Recommended contains only error-site relaxations (four fields). Every pixel-rule field stays at its Strict value in Recommended.
- D3: Truncated raw pixels (R10) and the 1.0 caps remain errors in every profile.
- D4: `recovered` is the closed two-event list above.
- D5: Profile names carry the library version they were measured against and are immutable once frozen; a changed table needs a new name.
- Open, awaiting owner decisions: Q1–Q6 (reviewer recommendations recorded in QUESTIONS.md) and Q9 (16-bit palette alpha; proposed default: defer, with Strict and Recommended unchanged and any bit-15-opaque behaviour in a future, separately versioned opt-in profile).
