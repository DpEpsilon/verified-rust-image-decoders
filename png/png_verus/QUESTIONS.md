# png_verus M1b: open questions

Items where the frozen spec, the draft policy texts and `inflate_verus` leave a choice, or where the implementation found something the texts did not anticipate. In each case M1b follows the most literal reading, named below; nothing in `spec-frozen/`, `THEOREMS.md`, `ERRATA.md` or `REVIEW-*.md` was edited.

## Q1. `zlib_tail = IgnoreTail` with fewer than `R` bytes: `TrailingZlibData` or `TruncatedImageData`?

`inflate_verus` ignores bytes after a matching Adler-32 under `IgnoreTail` only when the output is exactly full (`finish` in `inflate_verus/src/inflate.rs`; README "`IgnoreTail` ignores bytes after a matching Adler-32 only when `produced == limit`", following THEOREMS.md and OPTIONS.md's "acts only after the Adler-32 matched and the output is exactly `R` bytes"). With fewer than `R` bytes it returns `Err(TrailingZlibData)`. `policy_oracle.py` `zlib_decompress` instead ignores the tail whenever `zlib_tail != 'Reject'`, and its caller then reports `TruncatedImageData` because `len(raw) < R`.

The two error names differ for one input class: a RecommendedBeta image stream that is short and has bytes after a matching Adler-32. M1b maps `Err(TrailingZlibData)` to `TruncatedImageData` when the profile is IgnoreTail and `produced < R` (`image.rs` `image_status`). The mapping is exact: under IgnoreTail inflate reports `TrailingZlibData` only after the Adler-32 matched, which is the case in which the oracle reaches its length check. Example (scratch only): a 2 × 2 greyscale image whose IDAT holds a zlib stream of one scanline followed by two bytes: Strict `TrailingZlibData`, RecommendedBeta `TruncatedImageData` in both the oracle and png_verus; without the mapping png_verus would report `TrailingZlibData` under RecommendedBeta. The rule needs a line in RecommendedBeta's model at P1 (Stage 2), or a decision that the inflate-level name should surface instead; no vector of the 810 pins it.

## Q2. RecommendedBeta: a fcTL dropped for its CRC need not make the animation fail, and the fdAT after it is then inflated

OPTIONS.md (`crc_ancillary`) says "acTL and fcTL stay ancillary, so a bad-CRC one is dropped and the animation then fails structurally (R4)". That holds when the dropped fcTL's sequence number was the expected one. When the fdAT after the dropped fcTL carries the sequence number the walk expects next, the walk accepts the file, and the fdAT payload is appended to the preceding fdAT-based frame. Example (built in scratch only): `acTL(1 frame)`, IDAT, `fcTL(seq 0)`, `fdAT(seq 1)` holding the first half of a zlib stream, `fcTL(seq 2)` with a bad CRC, `fdAT(seq 2)` holding the second half, IEND. `policy_oracle.decode_frames(RecommendedBeta)` returns one frame decoded from both halves with `recovered = false`; Strict returns `CrcMismatch`. png_verus matches the oracle (its fdAT reader ends a frame only at a fcTL whose CRC matches, `reader.rs`).

This bears on the council's L-WALK-FIRST obligation ("never inflates the fdAT data of a frame whose fcTL failed its checks"). Under the literal reading of the oracle, the data after a CRC-dropped fcTL belongs to the previous frame, not to a frame of its own, so no rejected frame's stream is inflated; but a reader of L-WALK-FIRST could count the dropped fcTL as "a frame whose fcTL failed its checks". Also, no `recovered` event fires although the frame's data changed composition. Decision wanted at P1: keep (and state it in OPTIONS.md and L-WALK-FIRST), or reject a bad-CRC fcTL like fdAT (`crc_ancillary = SkipExceptFdat` extended to fcTL), which would make every such input `CrcMismatch` or an R4 fallback.

## Q3. The two `ImageCrateCompat-0.25.10` records of VECTORS-POLICY.json

`policy/crc-ancillary/fdat-frames/image-crate-compat` and `.../fdat-still/image-crate-compat` need `crc_ancillary = Skip`, `apng_errors = Prefix` and `after_idat = ImageRs`, which belong to the compatibility profiles (P2, out of scope for M1b). `tests/conformance.rs` counts them as skipped, never as passed: 808 of 810 records pass, 2 are skipped. GIF ran its two compatibility records under a test-only record; that is possible here at P2, once those three field values exist.

## Q4. API items of BRIEF.md without a model

`chunks(b, p)` with `ChunkStatus` and `try_icc_profile_with_policy` (PNG-Q4, Q13) are not implemented: SPEC P8.3 and P17 keep them outside the Stage-1 model, and PLAN.md P1 says they "need models first (D14–D16)". `Info` already carries the three-valued `Meta` (`Absent | Discarded | Present`) per item; under Strict nothing is ever `Discarded`.

## Q5. Representation choices (no spec text decides them)

- `Still16::rgba16` is `8·W·H` bytes with each 16-bit sample big-endian, because the guard reserves bytes (`try_zeroed`) and P20's `Output` site is byte-counted. A `Vec<u16>` would need a second element type in the guard.
- `Frames::try_new(b, p)` takes the profile, as BRIEF.md's API block writes it; there is no separate Strict-only constructor. `Frames` is not an `Iterator` (each view borrows the canvas), like GIF's.
- `Info` byte ranges (iCCP name, eXIf) are `(offset, length)` pairs into the caller's input, so `Still` carries no lifetime.
- The RecommendedBeta RGBA16 entry (`try_decode_still16_with_policy`, PNG-Q13 (i)) has no oracle; M1b draws a substituted palette index (R3) as `(0, 0, 0, 65535)` and otherwise applies P13.3 with RecommendedBeta's walk and inflate options.
