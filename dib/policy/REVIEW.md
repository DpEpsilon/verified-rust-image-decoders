# Independent review of the DIB/BMP policy draft (POLICY-SPEC.md, THEOREMS-POLICY-DELTA.md, REPORT.md)

Verdict: **freeze after listed fixes.** Counts: **3 blocking, 13 should-fix, 7 nit.** No finding breaks T1/T3/T11/T12; every blocking item is a place where the text leaves a named profile's behaviour underdetermined, or where a profile does something the owner decisions say it does not.

Everything below was produced with a reviewer oracle (`/tmp/claude-0/rev/myoracle.py`, written from POLICY-SPEC.md and frozen SPEC.md text before `policy_oracle.py` was opened). Scratch scripts are in `/tmp/claude-0/rev/`. Nothing outside this file was modified.

## 1. Reproducibility from the text

My oracle was built with the same 27 fields and the four profile tables read from the spec. Profile tables match `policy_oracle.py` field for field (4 x 27 values identical).

| Check | Result |
|---|---|
| STRICT-VECTORS.json (302) | 0 disagreements (`cmp` against the frozen VECTORS.json is identical) |
| POLICY-VECTORS.json (59 x 4 profiles) | 0 disagreements |
| CUSTOM-VECTORS.json (67) | 0 disagreements |
| Seed-731 matrix, 10,000 inputs x 4 profiles, BmpFile | Strict, GdkCompat, Recommended: 0 diffs; ImageCrateCompat: 11 diffs (finding B2) |

Zero vector disagreements is weak evidence of precision: the vectors do not discriminate most of the ambiguities below (section 4). The real test was differential fuzzing of my oracle against the author's:

- 8 fuzz runs, about 270,000 inputs (mutated vector inputs, a structured generator that builds well-formed BMP/DIB files of every depth, compression, header size, palette count, offset and RLE stream, plus an RLE-only run), random policies of 1 to 8 non-Strict fields, BmpFile and BareDib.
- Disagreements collapse to 8 distinct minimal causes. All 8 are text ambiguities or text-vs-oracle contradictions (B2, B3, S1 to S5). In each, the oracle's choice is defensible but not derivable from the text. In S1, S2 and S3 the oracle contradicts the text.

So: on every field combination my generators reached outside those 8 causes, a reader of the text gets the author's result; for the 8 causes they do not.

## 2. Strict equals frozen 1.1

My Strict (text-only) against `~/projects/dib/spec-1.1/check_vectors.py` (identical to the frozen copy): **0 differences** on 20,000 mutated inputs (all 4 modes, 1,961 accepted), plus all 10,000 seed-731 matrix inputs and the corpus directory in BmpFile, BareDib (header stripped), IcoEntryDib and IcoEntryDibSlack (50,405 comparisons). The author's `Strict` also matched the frozen oracle on 15,000 further mutated inputs. The `bfSize` rule (zero or `<=` length) and the first-error order agree. The strict bridge is a sound target for the proof.

## 3. The 27 fields

Hard invariant (T1/T3/T11/T12 for every policy value): holds under my analysis, with two wording fixes (S8, S9). Output size is `4N <= 64 MiB` before any allocation, including every Blank/ZeroFill/fill path. Worst-case step count by path, charging bulk fills by bytes touched as R18 requires: raw with ZeroFill about 4N (zeroed allocation) + N (alpha prefill) + N (pixels) = 6N; RLE about 4N + 4N (`rle_fill` prefill) + N (paint) + |b|/2 = 9N + |b|/2; Blank 4N. All are under `16*(|b|+N)`. A 54-byte BITFIELDS file with no masks decodes under GdkCompat to a 64 MiB image (`8192x2048`, probe `blank-bomb`): a 1.2-million-to-one amplification, bounded and within the contract, but worth a line in the API docs. Strict RLE already allows about 60,000 to 1 (EOB-only stream with a palette).

Per field (only items that need action; "ok" fields omitted):

| Field | Exact? | Interactions / redundancy / safety |
|---|---|---|
| file_size, image_size, planes, reserved, signature, alpha32, headers | yes | independent |
| offset | partly | Gdk: alpha of "initialized blank output" unstated; "indexed" does not say whether 2-bpp counts; with `bitfield_offset=DeclaredOffset` the "accepted offset" under Gdk is undefined (S5). Recover and AllowAny interact with `palette_gap`/`bitfield_offset` only by oracle code order (see S1). |
| palette_gap, bitfield_offset | no | text: palette_gap covers indexed/BI_RGB, bitfield_offset covers BITFIELDS. Oracle: palette_gap also moves BITFIELDS (S1). Two fields that together encode one decision (pixel start); no profile uses palette_gap=MetadataEnd. |
| incomplete_metadata, short_indexed_gap | partly | scope of "palette" and "mask" unclear (S4); blank alpha for RLE/32-bit is a convention restated three times (two fields plus offset=Gdk) and pinned by no vector. Both are admitted surrogates for GDK progressive-buffer state. |
| indexed_count=FullIgnoringUsed, important=RequireWithinCount | yes/no | no native evidence for either (spec admits for `important`); error name for `important` unspecified (S5) |
| high_count | no | BareDib/ICO behaviour for Ignore and GdkBound unspecified (B3) |
| short_pixels=ZeroFill | partly | `biSizeImage` check uses logical X (oracle) but text says "consumed" (S6). Logical cursor passes `|b|`: must be u64 on 32-bit targets (nit N3). |
| rle_eol / rle_eob / rle_delta | partly | rle_eob.AtHeight overlaps rle_eol.StopAtHeight on the EOL case. ClampStop + StopAtHeight, EOL arriving with y already at height: text says RleBounds ("successful EOL raising y"), oracle accepts (S2). |
| rle_overrun=Clip | yes | but T12 wording invites a violation (S8) |
| masks=DefaultRGB | partly | "BGRA 8888" does not say the 32-bit default includes alpha mask `0xff000000` (oracle exposes the stored alpha byte); scaling choice for non-contiguous masks under Replicate/Image unstated (S5) |
| masks=Independent | partly | alpha mask contiguity/range check unstated |
| scaling | yes | Gdk n<4 branch is a safe substitute, correctly flagged |
| header_bytes=ParsedFields | no | availability rule for the metadata end (B2); error and Blank behaviour when header-internal masks (H>=52) lie past EOF (S4) |
| unsupported=Rgb2 | contradicted | text: size >= 40; oracle: any non-CORE header including 16 (S3) |

Redundancy and dead surface: 10 non-default values are used by no profile and have no native counterpart: `file_size=Exact`, `offset=Recover`, `palette_gap=MetadataEnd`, `indexed_count=FullIgnoringUsed`, `important=RequireWithinCount`, `rle_eob=AtHeight|AtEOF`, `masks=ZeroChannel`, `scaling=Replicate`, `alpha32=Stored`, `signature=Ignore`. The option space is 1.6 x 10^10 records; only four are named and only single-field probes exist. Fuzzing found the S-class defects precisely in combinations (two or three fields). See S12.

Unsafe options: none unbounded. Caveats are S8, N3, and the amplification above. All fields are deterministic functions of bytes; the surrogates (`Blank`, `ZeroFill`, `short_indexed_gap`) are explicitly not GDK parity.

## 4. Findings

### Blocking

**B1. Recommended contradicts D2/D3 and REPORT.** D2 and REPORT describe Recommended as "Strict plus bfSize/biSizeImage ignored". The table also sets `high_count=Ignore` and `rle_eol=StopAtHeight`. `StopAtHeight` accepts an RLE stream that lost its final `00 01`. Input (BareDib, 1x1 RLE8, one palette entry): `280000000100000001000000010008000100000000000000000000000000000001000000000000000000ff0001000000` (palette `00 00 ff 00`, then the commands `01 00` and `00 00`). Strict: `RleTruncated`. Recommended: Ok. That is acceptance of a file truncated by two bytes, against D3's "preserve rejection of malformed RLE". Either drop both fields from Recommended or reword D2/D3 and REPORT, and state that Recommended is not "metadata only". Also decide whether `file_size=Ignore` (accepts `bfSize > |b|`, the classic truncated-download signal) belongs in a profile whose rationale is "strict on content".

**B2. ParsedFields: availability of the metadata end is unspecified.** With `header_bytes=ParsedFields` a declared header can end beyond EOF (the text says so). Palette end is then `s+H+table > |b|` even when the table is empty. The text's R7 literally says to return `TruncatedPalette` when bytes are insufficient. The oracle skips the check when `table_count=0`; my text-only reading rejected. ImageCrateCompat is affected: my oracle disagreed with the author's, and with image-rs, on matrix #4371, #4497, #5878, #6004, #6137, #8433, #8517, #8769, #8895, #9972 (the text cites #4371/#8433 itself), and in 10 of 10,000 seed-4242 inputs (the `biSize` cluster). Add the rule: "a zero-length palette or table needs no availability check; `palette_end` for offset purposes is `s+H+...` even if past EOF".

**B3. `high_count=Ignore` and `GdkBound` in BareDib are unspecified, and the oracle contradicts the plausible reading.** Text: "BmpTable ... BareDib still rejects nonzero count. Ignore: BMP skips no optional table." The oracle rejects any nonzero high-depth `biClrUsed` in BareDib for every value of the field, including `Ignore` and `GdkBound`. Input (BareDib): `280000000100000001000000010018000000000000000000000000000000000001000000000000000000ff00` (1x1 24-bpp, `biClrUsed=1`, one pixel row). With `high_count=Ignore` or `GdkBound`: oracle `InvalidColorCount`; my reading of the text: Ok (red pixel). Both profiles select Ignore, and the harness never exercises BareDib against a library (validate.py feeds BMP files only), so nothing pins it. State the BareDib result for each value.

### Should-fix

**S1. palette_gap vs BITFIELDS: text and oracle disagree.** 16-bpp BITFIELDS 1x1, masks `f800 07e0 001f`, 2 gap bytes, `bfOffBits` after the gap, `palette_gap=MetadataEnd`, `bitfield_offset=DeclaredOffset`. Text: pixels start at the declared offset, Ok. Oracle: starts at metadata end, reads the gap as pixels, different result (my oracle returns `TruncatedPixels`; the pixel bytes differ). Hex: `424d4600000000000000440000002800000001000000010000000100100003000000000000000000000000000000000000000000000000f80000e00700001f000000112200f8` (`bfOffBits=0x44`; with `bitfield_offset=MetadataEnd` as well both oracles agree, so the difference is purely palette_gap). Probe is in `/tmp/claude-0/rev/probes.py`. Profiles are unaffected (no profile sets palette_gap=MetadataEnd) but the field is. Fix the oracle or the text, then add a vector.

**S2. ClampStop + StopAtHeight, EOL with y already at height.** Text: only an EOL that raises y to height ends decoding; an EOL at y=height is RleBounds. Oracle: accepts and stops. GdkCompat is unaffected (AtInputEnd breaks at the loop top first); custom records are. Fix text or oracle.

**S3. Rgb2 at H=16.** Text: "only for size >= 40". Oracle: any non-CORE header, so 16-byte headers get 2-bpp. Unreachable in ImageCrateCompat (No16No64) but reachable with `headers=All1_1|` and `unsupported=Rgb2`. Probe: `rgb2-h16` accepted by oracle, rejected by text.

**S4. Blank scope.** `incomplete_metadata=Blank` says "short mask or palette". The oracle also blanks a short high-depth optional table (`BmpTable`) but not header-internal masks for H>=52 (`TruncatedMasks`). Probe: 24-bpp BMP, `biClrUsed=1000`, `offset=Recover`, `incomplete_metadata=Blank`: oracle Ok (black 1x1), text-only reading `TruncatedPalette`. Say which sites Blank covers.

**S5. Underspecified details with no vector (each changes output or error name).**
- Blank alpha for `offset=Gdk` ("initialized blank output"); oracle uses 0 for RLE, 255 otherwise.
- `important=RequireWithinCount` error name (oracle: `InvalidColorCount`); `offset=Gdk` BI_RGB past-EOF error name (`InvalidPixelOffset`); depth-2 with a non-BI_RGB compression (which of `UnsupportedPixelDepth`/`InvalidCompressionDepth`). R15 is a closed set and T10 is exact-first-error, so names matter.
- `DefaultRGB`, 32-bit: does "BGRA 8888" include the alpha mask?
- `Independent`: are the alpha mask's contiguity and range checked?
- Under `ParsedFields` and H>=52 with masks past EOF.

**S6. ZeroFill and image_size.** Text: R12 compares `biSizeImage` with consumed X. Oracle: X is the logical full-raster size, so a 2x2 24-bpp file with 8 bytes of pixel data and `biSizeImage=16` is accepted, and one with `biSizeImage=8` (the actual consumed size) is rejected. Probes: `zerofill-imagesize`. Specify which; I would use the logical size, as the oracle does, and say so.

**S7. Vector blind spots (mutation testing).** 111 single-point mutants of my oracle, 60 killed, 51 survived. Of the survivors, 33 are not equivalent mutants: they change output on random inputs from the structured generator (counts in the table). 7 non-default CUSTOM probes give the same result as Strict and so test nothing (`short_pixels=ZeroFill`, `rle_fill=OpaqueBlack`, `masks=Independent`, `scaling=Image`, `bitfield_offset=MetadataEnd`, `incomplete_metadata=Blank`, `header_bytes=ParsedFields`). 14 of the 59 per-profile vectors have the same outcome in all four profiles. Table in section 5.

**S8. T12 under Clip.** THEOREMS-POLICY-DELTA says "clipping counts each examined encoded pixel". Take width 1, RLE8, a stream of `ff 01` repeated k times. Under `rle_overrun=Clip`, after the first run every later run is clipped; each 2-byte command would be charged 255 steps (127.5 steps per input byte), over the `16*(|b|+N)` bound. The spec must require the implementation not to iterate clipped positions (O(1) per clipped command), and the theorem must say clipped positions cost zero. The oracle's own loop breaks at width, which is the right behaviour.

**S9. "Held-out" claim is overstated.** `harness.generate(10000, seed)` builds a fixed, seed-independent ladder of mutations (field-value lists, truncations, escape bytes, trailing bytes); the seed only shuffles the order and chooses the 20 random bit flips per source file. Measured with the harness: seeds 731 and 81573 share 4,773 of 10,000 distinct-byte inputs; 20261109 and 81573 share 4,867; **2,920 of the 10,000 "held-out" inputs are not in the union of the two tuning seeds, so about 71% of seed 81573 had already been seen**. The residual cluster counts (`bitfield-mask=14`, `rle-escape=13`, `frozen:BareDib=11`) are identical across seeds for the same reason. The author's timestamps are consistent with no tuning after the 81573 run (oracle edited 14:23, held-out seed directory created 14:23:18), so there is no evidence of cheating, but the sample is not independent: the 840 random bit-flip inputs per seed are the only genuinely new bytes. Say "agreement on a second sample from the same generator, 71% overlapping", not "held-out".

**S10. API and ICO.** Spec first paragraph says `decode(bytes, mode, &Policy)`; THEOREMS says `decode_with_policy` plus wrappers and a possible later facade. Choose one. Passing a non-Strict policy with an ICO mode is silently Strict ("an explicit API contract rather than silent fallback" is not an error, it is a fallback). Prefer a type that cannot express it (a `BmpMode` enum for the policy entry point) or a typed `PolicyNotApplicable` error. Also: say what `Policy::default()` is (Strict), mark `Policy` and its enums `#[non_exhaustive]` (adding a field or variant otherwise breaks semver), and give the policy record a JSON schema like R16 (`POLICY-VECTORS.json` has no policy field, `CUSTOM-VECTORS.json` does; neither is described by the spec).

**S11. No signal that output was recovered.** `ZeroFill`, `Blank` and `short_indexed_gap=Blank` return Ok with the same type as a clean decode. D4 says opt-in only, but a caller cannot tell a 54-byte BITFIELDS stub that decoded to a black 8192x2048 image from a real one. Add a `recovered` flag (or a distinct result type) before freezing; this is cheap now and a breaking change later.

**S12. Surface area.** 10 non-default values have no profile and no native counterpart (section 3); 5 fields (palette_gap, indexed_count, important, alpha32, signature) have no non-Strict value in any profile. Each costs proof effort and spec risk. Either drop them from the first freeze, or mark them experimental and out of the T-quantified contract, or add vectors for each. Custom records: 1.6 x 10^10 combinations, 4 tested; fuzzing found S1, S2, S3, S4 only in combinations. Restrict the public constructor to the four profiles plus named single deltas, or state that custom combinations are unsupported.

**S13. Validation covers BmpFile only.** `validate.py` and the harness give libraries BMP files. GdkCompat and ImageCrateCompat in BareDib mode (and B3) are checked against nothing. Add a wrapped-DIB measurement (the harness already has `wrap_dib`) or state that BareDib profile behaviour is unvalidated.

### Nit

- N1. validate.py's `reason()` labels every residual "missing option: ..." from the provenance label; it is a heuristic, not a diagnosis. REPORT's "roughly 50-70 bounded, reproducible but not yet modelled" is not backed by a per-residual proof.
- N2. Spec text "GdkBound ... the modeled 32-bpp threshold is zero": call this out in the profile table row, since it makes GdkCompat reject every 32-bpp `biClrUsed>0` while the native module accepts values 1 and `0xffffffff`.
- N3. ZeroFill/AllowAny logical cursor can reach about `2^32 + 8192*32768`: needs u64 on 32-bit targets, say so in R17's arithmetic note.
- N4. `rle_eob=AtEOF` and `AtInputEnd` for a delta with 1 to 3 bytes left: text says only incomplete absolute packets reject. Add "an incomplete delta rejects".
- N5. Spec says the oracle "allocates Python temporaries"; fine, but the 302-vector `cmp` claim is evidence of the file, not of the implementation. Reword.
- N6. REPORT lists 59 vectors per profile and 67 custom probes: true (verified). It should also say 7 of those probes do not discriminate.
- N7. `scaling=Image` text "same as Round through 8 bits" reads as "n <= 8"; say so.

## 5. Mutation survivors (my oracle, single-point mutants)

Column "diff" is the number of 7,000 random structured inputs (random profile plus 0 to 4 option edits) on which the mutant differs from my oracle; a non-zero count shows a non-equivalent mutant that no vector kills. "0" means equivalent or beyond my generator, not proven equivalent.

| Field | Surviving mutant | diff |
|---|---|---:|
| file_size | Exact rejects bfSize=0 | 12 |
| offset | Recover upper bound `off<L` | 4 |
| offset | Gdk indexed: blank only when off<pe | 5 |
| offset | Gdk indexed: off>=L | 2 |
| offset | Gdk BI_RGB: off>L accepted | 14 |
| offset | Gdk BITFIELDS default start = pe | 1 |
| palette_gap | MetadataEnd also for BITFIELDS (the oracle's behaviour) | 8 |
| bitfield_offset | MetadataEnd ignored | 8 |
| incomplete_metadata | BareDib palette Blank removed | 17 |
| incomplete_metadata | blank alpha 255 for all / for 32-bit only | 72 |
| short_indexed_gap | RLE alpha 255 | 26 |
| indexed_count | FullIgnoringUsed reads biClrUsed entries | 24 |
| high_count | GdkBound `>=` | 8 |
| high_count | Ignore in BareDib rejects | 168 |
| important | `>=` instead of `>` | 1 |
| important | also applied to high depth | 4 |
| short_pixels | BareDib cursor exemption removed | 87 |
| rle_overrun | Image: RLE4 encoded also clips | 14 |
| rle_overrun | Image: absolute overrun clips | 21 |
| rle_delta | ClampStop: x clamp width-1 / y clamp height-1 / stop without AtHeight | 1 / 1 / 1 |
| rle_eob | AtHeight ignored for delta | 9 |
| rle_eob | AtInputEnd one-byte accept removed | 11 |
| rle_fill | OpaqueBlack becomes Palette0 | 27 |
| masks | ZeroChannel skips disjointness | 3 |
| masks | DefaultRGB 16-bit default 565 | 7 |
| masks | DefaultRGB nonzero accepts bits above depth | 4 |
| scaling | Image n>8 rounds instead of top-8 | 5 |
| scaling | Gdk popcount>8 shift removed | 13 |
| headers | No16No64 keeps 64 | 190 |
| unsupported | Rgb2 also at H=16 | 12 |
| (18 equivalent or unreached by generator) | Recover lower bound, Gdk bitfield off>L, mask Blank (3-mask), blank alpha RLE-only, important ignores biClrUsed, ZeroFill continue-after-short, Clip no x clamp, ClampStop no x clamp, AtEOF accepts one byte, Black `index<=count`, DefaultRGB denominator, Replicate n>8, Gdk n cap, Gdk n<4, ParsedFields (3 mutants), Rgb2 any compression | 0 |

Most important blind spots: no vector for `headers=No16No64` with a 64-byte header (190 differences); BareDib under any non-Strict profile; Gdk offset boundaries; the blank-alpha conventions; every `rle_eob` accept boundary; `ZeroChannel`, `Independent`, `Replicate` non-trivial cases. Killed: 60 of 111, including all boundary mutations for file_size, reserved, signature, Declared offset, planes, short_indexed_gap, rle_eol, rle_overrun Reject, palette_index, scaling Round/Gdk core, alpha32.

## 6. Agreement with the real libraries, fresh seed 4242

Method: `harness.generate(10000, 4242)`, each input written to a file and run through `harness.invoke` for `gdk-file`, `gdk-stream`, `gdk-whole`, `gdk-byte`, `gdk-chunk` and `image-explicit` (same prlimit and timeout as validate.py), compared as accept/reject plus dimensions plus SHA-256 of the RGBA bytes. Profile tables are identical in my oracle and the author's. Native rows are in `/tmp/claude-0/rev/s4242/native_0_10000.jsonl`.

| Profile vs library | My oracle | Author's oracle | Claimed (seeds 731 / 20261109 / 81573) |
|---|---:|---:|---|
| ImageCrateCompat vs image 0.25.10 | 9,990 | **10,000** | 9,998 / 10,000 / 10,000 |
| GdkCompat vs gdk-file | 9,897 | 9,897 | 9,917 / 9,900 / 9,911 |
| GdkCompat vs gdk-stream/whole/byte/chunk | 9,893 | 9,893 | 9,914 / 9,893 / 9,909 |
| Strict vs gdk-stream (baseline) | 6,046 | 6,046 | not reported |
| Strict vs image-explicit (baseline) | 8,100 | 8,100 | not reported |

The 10 inputs where mine differs from the author's ImageCrateCompat are the ParsedFields cluster (B2). The claim "about 99% / 100%" **reproduces** on a fresh seed (98.9% to 99.0%; 100.0%).

Qualifications:
- On the 2,302 seed-4242 inputs not present in seeds 731, 20261109 or 81573: GdkCompat 2,254 of 2,302 (97.9%) on gdk-stream; ImageCrateCompat 2,302 of 2,302 (author's oracle). Smaller sample, still strong.
- Only 5,374 (GDK) and 3,240 (image-rs) of the 10,000 inputs are accepted by the library; agreement counts both-reject as agreement. On GDK-accepted inputs, same-pixels agreement is 5,272 of 5,374 (98.1%); on image-accepted, 3,240 of 3,240 for the author's tables.
- The 840 random-bit-flip inputs (the only fresh bytes): GdkCompat 806 of 840 (96.0%), ImageCrateCompat 840 of 840.
- GdkCompat residuals (107 on gdk-stream): 94 where the library accepts and the oracle rejects, 8 accept/accept with different pixels, 5 where the oracle accepts and the library rejects. By cluster: random bit flips 34, bitfield-mask 14, rle-escape 13, frozen:BareDib 11, width 9, height 8, bfOffBits 7, biClrUsed 5, bitCount 4, other 2. Several are heights/widths above 8,192 (excluded by the cap by design) and the 11 `frozen:BareDib` plus 14 `bitfield-mask` plus 13 `rle-escape` are identical to the tuned seeds because the input ladder is the same (S9).

## 7. THEOREMS-POLICY-DELTA.md

The plan is realistic as staged (typed `Policy`, strict bridge first, metadata fields, then palette/mask, then RLE) and the cost estimate ("at least one more substantial verification campaign") is honest. Specific advice:

- **Policy as a struct parameter.** Verus case-splitting on 27 enum fields inside every function risks SMT blow-up. Compute once, in `MH`, a small derived record (the effective choices: pixel start, offsets, count, mask set, fill pixel, accept-at-end predicate, clip flag) and pass that record, not the raw `Policy`, to `MP`/`MR`. Then T7/T12 proofs quantify over a few booleans.
- **Strict bridge.** Cheapest as a spec-level lemma: define `MH_1_1` and `MP_1_1` as the Strict instance, and make the old `decode` the Strict instance of the new function. An extensional equality between two separately written decoders would be far more expensive. My 70,000-input check says the Strict instance equals the frozen oracle, so the lemma is true.
- **What becomes policy-quantified.** T2, T4, T5, T6, T7, T9/T10, T13 as listed. T8 stays Strict (ICO). T1/T3/T11/T12 universal: fine, but see S8 for the Clip charge, and compute the explicit constant per path (section 3; margins of 16 against about 9 are comfortable).
- **Tagged partial-metadata record.** `MH` must return a third variant (complete, partial-metadata, error); every consumer of `MH` (including ICO's `MA`) must handle it. Prove ICO never receives it (`effective_policy` = Strict). Prefer making the ICO entry point take no policy at all (S10); the `effective_policy` function then disappears.
- **Logical cursor beyond `|b|`.** The plan's remedy (explicit remaining-row count) is right. Also check the Blank path: it returns before R6/R7 complete and before R12; state its T3 allocation site explicitly.
- **Witness explosion.** Plan is sensible (one strict witness file, small policy gates, runtime fixtures). Add: the vectors that matter most are the S7 blind spots, not duplicates of strict vectors.
- **Missing in the plan:** no per-field "off by default" kill-switch if a field's proof stalls (this is why S12 matters: drop speculative fields from the first campaign); no statement that error names for new sites are fixed (S5).

## 8. Owner decisions D1 to D7

| Decision | Assessment |
|---|---|
| D1 keep Strict and add policy entry point | Agree. Add: `Policy::default()` = Strict; non_exhaustive; one name (S10). |
| D2 Recommended ignores bfSize/biSizeImage | Rationale holds for reads. But the table does more (B1). Also consider keeping `file_size=AtMostFileLength` and ignoring only `biSizeImage`; `bfSize > |b|` is a free truncation detector. |
| D3 keep short-pixel and RLE bounds rejection | Agree, but Recommended's `rle_eol=StopAtHeight` weakens it (B1). |
| D4 ZeroFill opt-in | Agree, and add the recovered flag (S11); otherwise "opt-in" does not protect downstream consumers. |
| D5 count and offset authoritative | Agree. `offset=Recover` and `indexed_count=FullIgnoringUsed` then have no consumer; consider dropping (S12). |
| D6 unknown codecs and damaged magic rejected | Agree. Rgb2 is a non-default feature in ImageCrateCompat only; say so. |
| D7 ICO policy separate | Agree. Missing: how the policy argument behaves with ICO modes (S10), and what the glycin integration does with ICO-embedded BMPs. |

Decisions missing: **D8** which profile glycin's loader uses by default (ImageCrateCompat matches image-rs 100% on this corpus but accepts Rgb2 and wrong-size metadata; Recommended is the safer name but B1 applies); **D9** profile versioning, since GdkCompat and ImageCrateCompat name library versions that will change (freeze each profile table as immutable, name it with the library version, add new profiles rather than editing); **D10** whether custom policy records are public API (S12); **D11** whether recovered output carries a flag (S11); **D12** the error name for each new check site (S5).

## 9. Fixes required for the freeze

Blocking: B1 (choose Recommended's contents and reword D2/D3), B2 (ParsedFields availability rule), B3 (BareDib high_count). Should-fix: S1 to S6 (text/oracle reconciliation and a vector for each), S7 (vectors for each survivor in section 5, at minimum the 33 non-equivalent ones), S8, S9 (reword the claim), S10 to S13 (API decisions can ship with the freeze if recorded as decisions). After B1 to B3 and S1 to S6 are fixed, re-run vectors and the differential fuzz (`/tmp/claude-0/rev/fuzz_pol.py`, `gen_struct.py`, `fuzz_rle.py`, all seed-parameterised) and the seed-4242 agreement (`agree.py`); I expect 0 oracle disagreements.

---

# REVIEW addendum (second review, after the revision)

Verdict: **not ready to freeze; freeze after the fixes below.** Remaining: **1 blocking, 3 should-fix, 3 nit.** The first review's three blockers and all but two of its other findings are fixed. The new blocker is the public `recovered` flag, whose meaning cannot be derived from the text and which is a false positive on valid files.

Scratch: `/tmp/claude-0/rev/` (`myoracle2.py` is my oracle rewritten to the new text; `fuzz2.py`, `fuzz_rle2.py`, `fuzz_strict2.py`, `mutate2.py`, `equiv2.py`, `agree3.py`, `native2.py`).

## A. Status of each earlier finding

| # | Status | Evidence |
|---|---|---|
| B1 Recommended contents | **Fixed** | Table: Strict + `file_size=Ignore` + `image_size=Ignore` only; D2/D3 and REPORT agree. 40,000-input property check (author's oracle): Strict never sets `recovered`; whenever Strict accepts, Recommended returns identical pixels with `recovered=false`; whenever Recommended accepts unflagged, Strict accepts (0 violations). |
| B2 ParsedFields availability | **Fixed** | Text: zero-length table needs no check, nonempty checks its span, H>=52 internal masks give `TruncatedMasks`. Vectors `parsed-fields-past-eof-empty-table`, `-nonempty-table`, `-table-required`; my oracle agrees on all; the 10 seed-4242 ImageCrateCompat residuals are gone. |
| B3 BareDib `high_count` | **Fixed in text** | "All three values reject nonzero high-depth `biClrUsed` in BareDib with `InvalidColorCount`". Mutant "BareDib Ignore accepts" is killed; "BareDib GdkBound accepts" survives (no vector; see S15). |
| S1 palette_gap vs BITFIELDS | **Fixed** | Field `palette_gap` deleted; only `bitfield_offset` moves BITFIELDS pixels. |
| S2 ClampStop+StopAtHeight, EOL at height | **Text fixed, vector does not test it** | Text now matches the oracle (RleBounds, except AtInputEnd stops at loop entry). Vector `coupled-clamped-delta-eol-at-height` has the raster stream `00 ff 00 00`, which is an absolute packet of count 255 (RleBounds from the overrun), not a delta followed by an EOL. A mutant that re-accepts EOL at height still passes every vector. New finding S14. |
| S3 Rgb2 at H=16 | **Fixed** | Both oracles reject; vectors `rgb2-header16-*`; mutant "Rgb2 also at H=16" killed. New nit N8 (error name). |
| S4 Blank scope | **Fixed in text** | Blank = short external mask or nonempty palette/table; internal masks never Blank. Vector `incomplete-high-depth-optional-table`; mutant "Blank only for indexed" killed. "Blank also for H>=52 masks" survives (no ParsedFields+Blank+H>=52 vector; unreachable from my generator). |
| S5 underspecified details | **Mostly fixed** | Gdk blank alpha (stated, but unpinned: mutant "always 255" survives, 102 differences on random inputs); `important` removed; `InvalidPixelOffset` and `InvalidColorCount` named; 32-bit DefaultRGB alpha mask stated (mutant killed); Independent alpha check stated (mutant "alpha unchecked" survives, 9 differences). Still unspecified: depth-2 with a non-BI_RGB compression (N8). |
| S6 ZeroFill and image_size | **Fixed** | Text: logical X; vectors `short-raster-logical/physical-image-size`; mutant "X = consumed bytes" killed. |
| S7 vector blind spots | **Partly fixed** | 29 of 29 non-default values now have a custom probe (the earlier 7 non-discriminating probes are gone; 2 of 34 remaining non-default probes equal Strict, both deliberate negative probes). 78 vectors per profile (15 of 78 identical across profiles, was 14 of 59). Mutation audit in section C: 12 earlier non-equivalent survivors remain. |
| S8 T12 under Clip | **Fixed** | Spec and theorem text: `Clip` iterates only over `min(quantity, width-x)` painted positions; T12 states per-path constants. I re-derived them: raw 4N (zeroed allocation) + N (ZeroFill alpha prefill) + N (pixels) = 6N; RLE 4N + 4N (prefill) + at most N painted (x never decreases within a row, y never decreases, so each cell is written at most once) = 9N, plus constant work per command and at most one command per two input bytes; Blank 5N. All under `16*(|b|+N)`. |
| S9 "held-out" claim | **Fixed** | REPORT withdraws the claim and reports a novel slice. Caveat: seed 4242 was my review's seed and its residual clusters were reported to the author, so it is no longer untouched; section E uses a new seed. |
| S10 API and ICO | **Fixed** | `decode` unchanged for all four modes; `decode_with_policy(bytes, BmpMode, Profile)` BMP/BareDib only; oracle raises on ICO modes and unknown profiles; `Policy::default()` is Strict; `#[non_exhaustive]`; vector record schema described. |
| S11 recovered signal | **Fixed in design, new blocker B4** | Flag added; its definition is the problem. |
| S12 surface area | **Fixed** | 22 fields (from 27); `Exact`, `Recover`, `Replicate`, `ZeroChannel`, `AtHeight`, `AtEOF` and five whole fields removed; every remaining non-default value is used by some named profile; custom records internal (`EXPERIMENTAL_OPTIONS`). Combination count is now 7.2e7 (was 1.6e10). |
| S13 BareDib unvalidated | **Fixed (disclosed)** | Stated in spec and REPORT. |
| N1 validate.py `reason()` heuristic | **Not fixed** | Unchanged; still labels residuals from provenance. |
| N2 GdkBound threshold zero | Fixed | In the field row. |
| N3 u64 cursors | Fixed | In the R14 paragraph. |
| N4 incomplete delta | Fixed | `rle_eob` row. |
| N5 `cmp` wording | Fixed | "checks the copied vector file, not the decoder". |
| N6 vector counts | Fixed | 78 per profile and 56 custom, verified. |
| N7 `Image` scaling wording | Fixed | "same as Round for `n<=8`". |

## B. My oracle against the new text; differential fuzz

I rewrote `myoracle2.py` from the new POLICY-SPEC.md (22 fields, profile names, `recovered`). Results:

- STRICT-VECTORS (302): 0 disagreements. POLICY-VECTORS (78 x 4) and CUSTOM-VECTORS (56): 0 disagreements on width, height, pixels and error names.
- `recovered` could not be derived from the text. I had to make four vector-guided adjustments before all vectors passed: the flag is not set for AllowAny offsets that point into metadata but are inside `[18, length]`; not set for `high_count=Ignore`; set for `ParsedFields` whose declared header ends past EOF; set for a non-Palette0 fill. After that, 0 disagreements including the flag.
- Differential fuzz against `policy_oracle.py`, random policies (named profiles plus 0 to 3 field edits, or 1 to 7 random fields), BmpFile and BareDib, mutated vector inputs and the structured generator. This time the comparison includes **error names** (the first review's fuzz compared accept/reject and pixels only; vectors compared names). 120,000 inputs in 3 runs, plus 30,000 RLE-focused inputs:
  - Pixels and accept/reject: **0 disagreements.** All 8 earlier causes (B2, B3, S1 to S4 and the Blank scope) are gone.
  - Error names: 1 cause. Depth 2 with a non-BI_RGB compression under `unsupported=Rgb2`: mine `UnsupportedPixelDepth`, author `InvalidCompressionDepth` (about 14 per 40,000). Text does not say. Nit N8.
  - `recovered`: 3 causes (65 to 72 per 40,000), all in B4.

## C. Mutation sweep and kill audit

I re-ran single-point mutants on my new oracle against the new vectors: 58 mutants (25 carried over, 8 new for the review fixes, 7 new Blank/ParsedFields/ZeroFill/Gdk-alpha/mask mutants, 18 for the `recovered` flag), 37 killed, 21 survived. For each survivor I measured differences on 12,000 random structured inputs to separate real blind spots from equivalent mutants.

Of the earlier 33 non-equivalent survivors:

| Earlier survivor | Now |
|---|---|
| 7 on removed values or fields (`Exact` rejects zero, `Recover` upper bound, `palette_gap` BITFIELDS, `indexed_count`, both `important`, `ZeroChannel` disjointness) | not applicable |
| BareDib `high_count=Ignore` rejects | the behaviour is now specified (all reject); new mutant "Ignore accepts" **killed** |
| **Killed (12):** bitfield_offset MetadataEnd ignored; BareDib palette Blank removed; blank alpha always 255; short_indexed_gap RLE alpha; `Image` RLE4 clips; `Image` absolute clips; AtInputEnd one-byte accept removed; OpaqueBlack becomes Palette0; `Image` n>8 rounds; Gdk popcount shift; No16No64 keeps 64; Rgb2 at H=16 | |
| **Remain (12 non-equivalent + 1 unreached):** Gdk indexed blank only below pe (10 differences); Gdk indexed off>=L (6); Gdk BI_RGB past EOF accepted (26); Gdk BITFIELDS default start = pe (11); blank alpha for 32-bit only (33); GdkBound `>=` (37); BareDib ZeroFill cursor exemption removed (97); ClampStop x clamp width-1 (1); ClampStop y clamp height-1 (8); AtInputEnd ignores height after delta (9); DefaultRGB 16-bit default 565 (16); DefaultRGB nonzero accepts bits above depth (10); ClampStop stop-at-height without AtInputEnd (0 differences on my generator) | |

New survivors: BareDib `GdkBound` accepts (71 differences), Gdk blank alpha always 255 (102), Independent alpha mask unchecked (9), clip not flagged `recovered` for encoded runs (2); equivalent/unreached: Blank for H>=52 internal masks, EOL-at-height accepted (the vector bug in S14), absolute-packet clip flag, ClampStop clamp flag. Killed new mutants include every ParsedFields/zero-length-table, Blank-scope, ZeroFill-X and 15 of 18 `recovered` mutants.

## D. Strict equals frozen 1.1

My Strict (text-only, `myoracle2.py`) and the author's `decode` against `/home/ws/projects/dib/spec-1.1/check_vectors.py`: **0 differences each** on 39,996 comparisons (20,000 mutated inputs across all four modes, 6,655 accepted; the 10,000 seed-731 inputs as BmpFile and as BareDib with the 14-byte header stripped). `cmp STRICT-VECTORS.json` against the frozen VECTORS.json is identical. Strict never sets `recovered` (0 of 40,000).

## E. Agreement on a new seed, exact pixels

Seed 9091, restricted to distinct inputs absent from seeds 731, 20261109, 4242 and 81573: 1,779 of 10,000 (41% of them random bit flips). Native runs: `harness.generate` plus `harness.invoke`, same prlimit and timeout as `validate.py`. Oracle: **my `myoracle2.py` and the author's gave identical counts**; profile tables identical. GdkCompat target `gdk-stream`.

| Profile | inputs | both reject | native-accepted, exact pixels | native-accepted, ours rejects | native-accepted, different pixels | native rejects, ours accepts |
|---|---:|---:|---:|---:|---:|---:|
| GdkCompat-2.42.12 vs gdk-stream | 1,779 | 810 of 814 | **945 of 965 (97.9%)** | 20 | 0 | 4 |
| ImageCrateCompat-0.25.10 vs image-explicit | 1,779 | 1,166 of 1,166 | **613 of 613 (100%)** | 0 | 0 | 0 |

Overall agreement (both-reject counted): Gdk 1,755 of 1,779 (98.7%), Image 1,779 of 1,779. All 24 Gdk residuals are random bit flips (18 library-accepts, 4 library-rejects) and biClrUsed (2).

Replay of seed 4242 on the 3,180 inputs absent from the two tuning seeds, with both oracles: GdkCompat 3,125 of 3,180 (native-accepted exact pixels 1,710 of 1,761 = **97.1%**, 44 ours-rejects, 7 different pixels, 4 native-rejects ours-accepts), ImageCrateCompat 3,180 of 3,180 (1,112 of 1,112 = 100%). **The claimed 97.1% and 100% reproduce exactly**, and the 9091 numbers (97.9% and 100%) are consistent, on inputs the author has never seen. Limits: both samples come from the same mutation generator, so ImageCrateCompat's 100% means "no residual among 1,779 + 3,180 novel inputs", not equivalence; BareDib is unmeasured; GDK accepts about 54% of these inputs and image-rs about 34%, so the both-reject column carries most of the "all inputs" figure.

## F. Remaining findings

**B4 (blocking). `recovered` is not derivable from the text and is wrong on valid files.** The text defines it as "true whenever an ignored invalid declaration, initialized fill, clipped run, substituted palette entry or other recovery path fires". Consequences:

1. **False positive on ordinary files.** A complete RLE8 BMP ending `... 00 00 00 01` (EOL then EOB) decodes under GdkCompat and ImageCrateCompat with `recovered=true` in both oracles, because `rle_eol=StopAtHeight` stops at the EOL (before reading the EOB) and the stop is counted as a recovery. Hex: `424d44000000000000003e000000280000000200000001000000010008000100000000000000000000000000000002000000000000000000ff00ff000000020000000001`. Nearly every Windows-written RLE file has this shape, so every RLE image under a compat profile is flagged. The flag should be set only when something was actually skipped; peeking at most two bytes for `00 01` would cost nothing, but would contradict the text "ends decoding before reading another command", so the text must change.
2. **The two oracles disagree.** The author's oracle sets the flag for every RLE image under any `rle_fill != Palette0`, whether or not a fill pixel remains visible; mine sets it only when a fill pixel remains. Hex (stream `02 00 00 01`, all pixels painted, no EOL): `424d42000000000000003e000000280000000200000001000000010008000100000000000000000000000000000002000000000000000000ff00ff00000002000001`; author true, mine false under both compat profiles (the 64 of 40,000 differences).
3. **Other readings differ.** For Gdk with BITFIELDS and `bfOffBits` inside the metadata, the author does not flag the offset (only the BI_RGB high-depth case is flagged) while the text says "ignored invalid declaration"; AllowAny with an offset inside metadata is not flagged but an offset outside `[18, length]` is.
4. T9/T10 now claim equality of the flag; the proof target is undefined until this is fixed.

Fix: define `recovered` as a closed list of events with one line each (which field, which condition), state that a flag-false result must equal the result of the same bytes under a stricter rule set (for Recommended this holds: 0 violations in 40,000), and add a vector per event and one negative vector per profile with a valid file that must report `recovered=false`.

**S14 (should-fix). The S2 vector tests something else.** `coupled-clamped-delta-eol-at-height` contains `00 ff 00 00` (an absolute-packet overrun), so it errors with RleBounds regardless. Replace by a delta that clamps to height followed by `00 00` (BareDib probe in my scratch: `rle_eol=StopAtHeight, rle_delta=ClampStop` on `00 02 00 05 00 00 00 01`: RleBounds).

**S15 (should-fix). Vector blind spots that remain.** The 12 non-equivalent earlier survivors and the 4 new ones (section C): Gdk offset boundaries for indexed (off below pe, off equal to length), BI_RGB past EOF and BITFIELDS default; Gdk blank alpha; blank alpha for 32-bit; GdkBound threshold; BareDib GdkBound and BareDib ZeroFill cursor exemption; ClampStop clamp values and AtInputEnd after a delta that reaches height; DefaultRGB 16-bit default masks and bits above depth; Independent alpha mask; clip flag for encoded runs; ParsedFields with H>=52 internal masks and Blank. Each needs one input.

**S16 (should-fix). Derived header record in THEOREMS-POLICY-DELTA.** The record (pixel start, metadata end, count, masks, fill pixel, terminal predicate, clip mode) omits what `MP`/`MR`/`Conv` still need: `palette_index` mode, `short_pixels`, `scaling`, `image_size` mode, `bitfield16_alpha`, and separately the EOL, delta and EOB modes (the "terminal predicate" conflates three fields whose interactions were the source of S2). It also does not carry the header-stage recovery events (bfSize, reserved, planes, offset, ParsedFields) into `MP`, which must produce the final flag; add a `recovered_so_far` component. The other plan points hold: spec-level Strict bridge, no `effective_policy`, T12 accounting, Clip as zero iterations, and the gating of speculative fields. Verus will accept a record of about 15 scalars; 7.2e7 combinations is not an obstacle once the proofs quantify over the record.

**Nits.** N8: depth 2 with a non-BI_RGB compression under `Rgb2` has no stated error name (two readings; R14 order alone does not decide because the depth check can be read as "2 is allowed for BI_RGB only"). N9: `bitfield_offset=DeclaredOffset` under `offset=Gdk` leaves "the accepted offset" undefined for BITFIELDS (both oracles use the raw offset and fail on short data; custom records only). N10: the first review's fuzz did not compare error names, so its "0 vector-level surprises" covered accept/reject and pixels only; this addendum's does.

## G. Verdict and remaining owner decisions

**Not ready to freeze.** Required before freezing: B4 (define `recovered` precisely and remove the false positive), S14 (fix the vector), S15 (add the listed vectors), S16 (record contents). Nits N8 to N10 can ship as decisions. After those, re-run: `fuzz2.py` (expect 0 differences including flags and names), `mutate2.py`/`equiv2.py` (expect no non-equivalent survivors), `agree3.py` on seed 9091.

Remaining owner decisions:
- **D8** which named profile glycin's loader uses: ImageCrateCompat-0.25.10 matches image-rs on 100% of 4,959 novel inputs but accepts 2-bpp and `bfOffBits` inside metadata; Recommended is Strict plus metadata leniency and flags every ignored declaration.
- **D13** the exact `recovered` contract (events list; whether a stop at a terminal EOL counts).
- **D14** whether `EXPERIMENTAL_OPTIONS` exists in a released build at all (7.2e7 internal combinations stay in the universal proof either way).
- **D15** error name for depth 2 with non-BI_RGB under Rgb2 (N8) and for Gdk BITFIELDS offsets (N9).

---

# REVIEW addendum 2 (third pass)

Verdict: **not ready to freeze; one small blocking item, then freeze.** Remaining: **1 blocking, 2 should-fix, 2 nit.** The previous blocker (B4) is closed except for one event that the oracle and vectors set but the closed list does not name (B5). All earlier survivors except two are killed.

Scratch: `/tmp/claude-0/rev/` (`myoracle3.py`, `runvec3_lib.py`, `fuzz2.py`, `fuzz_rle2.py`, `mutate3.py`, `mutate3b.py`, `recov.py`, `native3.py`, `agree4.py`).

## 1. My oracle against the new text; vectors; differential fuzz

`myoracle3.py` implements the closed `recovered` list literally, the N8 rule (H>=40 depth 2 with any non-BI_RGB recognized compression gives `InvalidCompressionDepth`; unknown compression codes give `UnsupportedCompression` first; H=16 depth 2 gives `UnsupportedPixelDepth`) and the N9 rule (raw `bfOffBits` for Gdk + BITFIELDS + DeclaredOffset, `TruncatedPixels` on short data).

- Vectors: STRICT 302, POLICY 99 per profile, CUSTOM 63. **0 disagreements** on width, height, pixels, error names and `recovered`, after one change that the list does not authorize (B5).
- Differential fuzz against `policy_oracle.py`, random policies, error names and the flag compared: 120,000 inputs (3 seeds) plus 30,000 RLE-focused.
  - Pixels and accept/reject: 0 disagreements.
  - Error names: 1 cause, about 4 to 8 per 40,000. Depth 2, `unsupported=Rgb2`, **unknown** compression code (for example 5): text says `UnsupportedCompression` first; the oracle returns `InvalidCompressionDepth` because `if bits==2 and compression!=0: fail(InvalidCompressionDepth)` precedes the compression-code check. The oracle contradicts the text (S18).
  - Flag: 1 cause, about 3 per 40,000, 5 per 30,000 RLE-focused. `rle_delta=ClampStop` with a delta whose destination equals `width` or `height` exactly: the oracle flags it (`>=`); the text says "changes an out-of-bounds destination coordinate", and clamping an exact `width` changes nothing. A text ambiguity (N11); no vector pins it (mutant survives, 0 differences on my generator).

## 2. Mutation kill audit

I re-ran the carried-over mutants plus 25 new ones (negative and positive `recovered` mutants) against the new vectors: 72 mutants, 68 killed. The 4 survivors: BareDib `GdkBound` accepts, Gdk BITFIELDS out-of-range offset not flagged, ClampStop flagged at equality, and one ill-formed mutant that is equivalent.

Earlier survivors:

| Earlier survivor | Now |
|---|---|
| Gdk indexed blank only below pe; Gdk indexed off>=L; Gdk BITFIELDS default start; blank alpha 32-bit only; GdkBound `>=`; BareDib ZeroFill cursor exemption; ClampStop x clamp, y clamp; DefaultRGB 16-bit default; DefaultRGB bits above depth; Gdk BI_RGB past EOF accepted; AtInputEnd ignores height after delta | **killed** |
| ClampStop stop-at-height without AtInputEnd (unreached before) | **killed** |
| Gdk blank alpha always 255; Independent alpha unchecked; clip flag for encoded runs | **killed** |
| **BareDib `GdkBound` accepts** (71 differences on 12,000 random inputs) | **still survives**: no vector puts `high_count=GdkBound` and a nonzero high-depth count in BareDib |

New survivor: Gdk BITFIELDS with `bfOffBits` outside `[18, length]` not flagged `recovered` (3 differences on 12,000); no vector is a positive witness for that bullet (S17).

## 3. Strict vs frozen 1.1

0 differences for my oracle and the author's on 39,996 comparisons (20,000 mutated inputs across all four modes, 6,751 accepted; 10,000 seed-731 inputs as BmpFile and as BareDib). `STRICT-VECTORS.json` is identical to the frozen VECTORS.json.

## 4. Fresh seed 31337

1,538 distinct inputs absent from seeds 731, 20261109, 4242, 81573 and 9091. My oracle and the author's gave identical counts.

| Profile | both reject | native-accepted, exact pixels | native-accepted, ours rejects | native rejects, ours accepts |
|---|---:|---:|---:|---:|
| GdkCompat-2.42.12 vs gdk-stream | 561 of 569 | **947 of 969 (97.7%)** | 22 | 8 |
| ImageCrateCompat-0.25.10 vs image-explicit | 817 of 817 | **721 of 721 (100%)** | 0 | 0 |

Overall (both-reject counted): Gdk 1,508 of 1,538 (98.0%); Image 1,538 of 1,538. This is consistent with the claimed 97.1% (seed 4242 novel) and 97.9% (seed 9091); the three samples together give Gdk 3,602 of 3,695 native-accepted exact (97.5%) and Image 100% on 2,446 native-accepted inputs. All three samples come from the same mutation generator.

## 5. The `recovered` flag

**Closed list.** The wording ("only the following data-affecting events; no unlisted tolerance sets it") is the right contract, and the negative statements (bfSize/biSizeImage, reserved, planes, trailing bytes, EOL+EOB under StopAtHeight, fully overwritten prefill) are all enforced: I tested each as a mutant and every one is killed by a vector. Recommended's flag is exact: on 6,000 inputs Strict accepts, Recommended returns the same pixels with `recovered=false` every time (and earlier on 40,000 Strict-rejected/accepted inputs, never an unflagged accept that Strict rejects).

**Positive witnesses (one vector kills the "event not flagged" mutant).** Present for: Gdk indexed invalid offset, Gdk BI_RGB clamp below metadata end, AllowAny offset outside `[18,|b|]`, ParsedFields past EOF, incomplete-metadata Blank, short_indexed_gap Blank, DefaultRGB substitution, ZeroFill, Black substitution, encoded-run clip, absolute-run clip, ClampStop, early-end fill. **Missing:** the Gdk BITFIELDS outside-`[18,|b|]` clause of the first bullet (S17).

**Negatives under each profile.** Complete valid files give `recovered=false` under all four profiles: on 6,000 Strict-valid inputs, 0 of Recommended, and for the compat profiles the flag fires only on three events that change the data relative to the declared layout: early-end fill (valid RLE with an early EOB under a non-Palette0 prefill: 135 Gdk, 148 Image), short_indexed_gap Blank (42 Gdk), and **bitfield_offset=MetadataEnd with a gap (150 Gdk)**. The first two are in the list. The third is not (B5).

**B5 (blocking, small). The closed list omits an event that the oracle and the vectors set.** With `bitfield_offset=MetadataEnd` (GdkCompat), a valid BITFIELDS file whose `bfOffBits` is not the metadata end (a gap, an optimal colour table, V4/V5 colour-profile bytes) is decoded from the metadata end and `policy_oracle.py` sets `recovered`. Vectors `base-bmp-fields-40-masks-then-optimal-table-gap`, `gdk-bitfield-declared-offset-with-gap` and `bitfield_offset-MetadataEnd-...` expect `true`. No bullet covers it ("outside `[18,|b|]`" does not apply), and the text says no unlisted tolerance sets the flag. Both readings are defensible (the pixels change, so it is data-affecting), but text, oracle and vectors must agree. Fix: add the event to the list (`bitfield_offset=MetadataEnd` selects a start different from a valid declared offset), or drop the flag from the oracle and three vectors. I implemented the oracle's behaviour to make my vectors pass; 150 of 4,122 Strict-valid inputs accepted by GdkCompat in my sample hit it.

## 6. Other items

**S17 (should-fix).** Add vectors for the two survivors: BareDib + `high_count=GdkBound` + nonzero count (expect `InvalidColorCount`), and Gdk + BITFIELDS + `bfOffBits` below 18 or above the length with `bitfield_offset=DeclaredOffset` (positive witness, expect `recovered=true`).

**S18 (should-fix).** The oracle's depth-2 check order contradicts the text (section 1). Either reorder the oracle (compression-code check first) or reword the text ("any non-BI_RGB compression, including unknown codes, gives `InvalidCompressionDepth`"), and add a vector with an unknown compression code at depth 2.

**N11 (nit).** ClampStop flag at exact equality (section 1): say "when clamping alters the coordinate" or "when the destination is not inside".

**N12 (nit).** REPORT.md still opens "second independent review pending" and says the hard invariant is "reviewer-confirmed"; I confirmed it with the per-path bounds in addendum 1 only for the text as written, and the theorem text still omits `bitfield_offset=MetadataEnd` from the header-recovery events carried in `recovered_so_far` (follows B5).

## 7. Closed items

The theorem record (S16) now carries the three RLE modes separately, palette-index, short-pixel, scaling, image-size and alpha modes, and `recovered_so_far`; the plan is sound. N8 and N9 choices are specified and tested apart from S18. `#[cfg(test)]` custom records answer D14. D8 (glycin default profile) remains an integration decision.

## 8. Verdict

**Not ready to freeze.** Required: B5 (one list bullet or an oracle and vector change), S17 (two vectors), S18 (one ordering or wording fix). All are a few lines. After them, re-run `fuzz2.py` (expect 0 differences in pixels, names and flags, apart from N11 if it is not clarified), `mutate3.py`/`mutate3b.py` (expect only equivalent survivors) and the vector check. The nits can ship as decisions. If B5, S17 and S18 are fixed as proposed, my verdict becomes: ready to freeze after nits.

---

# REVIEW addendum 3 (final recheck)

Verdict: **ready to freeze** (no remaining blocking or should-fix items; no new nits).

- B5: the BITFIELDS `bitfield_offset=MetadataEnd` event is now the second bullet of the closed `recovered` list. My oracle already implemented it; the three vectors that expected it agree with the text.
- S17: BareDib + `GdkBound` + nonzero count and the Gdk BITFIELDS invalid-offset positive witness are present. Mutants "BareDib GdkBound accepts" and "Gdk BITFIELDS out-of-range offset not flagged" are **killed**.
- S18: unknown compression at depth 2 gives `UnsupportedCompression`; my oracle already did, and the author's oracle now agrees. Two depth-2 ordering mutants are **killed**.
- N11: the text now matches my oracle (flag only when the unclamped destination exceeds width or height). Mutants flagging at equality (`>=`) are **killed** by the new exact-boundary vector.
- Vectors: STRICT 302, POLICY 102 per profile, CUSTOM 66: **0 disagreements** (pixels, error names, `recovered`).
- Differential fuzz against `policy_oracle.py` with random policies, error names and flag compared: 30,000 mixed inputs plus 30,000 RLE-focused (delta dx/dy generated up to width+1 and height+1, so exact-boundary deltas occur): **0 disagreements** in pixels, accept/reject, error names and flags.
- Strict vs frozen 1.1: **0 differences** for my oracle and the author's on 39,996 comparisons (20,000 mutated inputs in all four modes plus the 10,000 seed-731 inputs as BmpFile and BareDib); `STRICT-VECTORS.json` is identical to the frozen file.
- Unchanged caveats, not blockers: BareDib has no native validation; GdkCompat agrees with GDK on 97.1% to 97.9% of native-accepted novel inputs (three samples: 3,602 of 3,695), ImageCrateCompat on 100% of 2,446; D8 (glycin's default profile) is an integration decision.
