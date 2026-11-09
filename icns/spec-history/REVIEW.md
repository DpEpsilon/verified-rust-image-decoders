# Independent review of SPEC.md, VECTORS.json, SOURCES.md

Reviewer: independent of the spec author. Scope: fidelity and completeness as a format spec.
Marking: RECALLED = from memory or a search snippet, not read from a primary source here.
Primary sources actually read: `io-icns.c` (all 524 lines), rust-icns `icontype.rs`, `element.rs`, `family.rs` (fetched from raw.githubusercontent.com). The Wikipedia page itself timed out (ECONNRESET); the claims taken from it come via a web-search snippet and are RECALLED.

Counts: blocking 3, should-fix 10, nit 7.

Result of vector checks: `python3 -I check_vectors.py` passes 68/68. I wrote a second oracle from the spec text (`/tmp/claude-0/---modal-volumes-vo-Iyh3VLkOB7cEHrIp2RkTs7/07bf9765-af64-477d-a906-dbb14d47e2dd/scratchpad/myoracle.py`) and it agrees on all 68, so no vector is wrong against the spec as written. I hand-decoded 25 of the small vectors (listed at the end). The vector set has coverage gaps (S7), not wrong answers.

The C-code citations in SPEC.md and SOURCES.md are accurate. The line ranges for rust-icns are also correct, except one SOURCES.md remark about the C comment (N6).

---

## Blocking

### B1. The result types are never defined (intro, R9, R10, R14, R15)
The intro says `parse` "returns entries", but the spec never says what an entry contains. The vectors use `type_hex`, `payload [start,end]`, `opaque_candidate`, `nominal_side`; none of these names or semantics appears in SPEC.md. Also undefined:
- the integer types of `width`, `height`, the indices, and the payload offsets;
- how an absent `color_index`/`mask_index` is represented;
- `nominal_side` for an entry classified opaque by signature only, or for a legacy type;
- whether `nominal_side` is set for a type-listed entry that also matches a signature (the oracle says yes, from the type).

Suggested sentence after the first paragraph:
> An entry is the record `{type: [u8;4], start: u32, end: u32, opaque_candidate: bool, nominal_side: Option<u32>}` with `start <= end <= len(bytes)`. `nominal_side` is `Some(d)` exactly when the type code is one of the fourteen listed in R9 and `None` otherwise (including signature-only matches and R8 types). Indices are `u32`; an absent index is `None`. `width` and `height` are `u32`. `decode_*` do not re-validate `entries`; passing entries not produced by `parse` of the same slice is a precondition violation.

### B2. R5 does not say what "remaining container bytes" is measured from
Two implementers differ: remaining from the entry start (`L - offset`, includes the 8-byte header, what the oracle and the length semantics imply) or remaining after the header (`L - offset - 8`). No vector separates them: `entry-exceeds-file` has length 17 against 8 remaining, which fails under both readings. An off-by-eight bug passes the suite.
Rewrite R5: "If `length > declared_length - offset`, where `offset` is the offset of the entry's first header byte, return `EntryOutOfBounds`." Add vectors: an entry whose length equals `L-offset` (accepted) and one with `L-offset+1` (`EntryOutOfBounds`).

### B3. R6/R15 ordering of TooManyEntries versus later malformed entries is ambiguous
R15 says "R2–R6 in order for parse", which reads as if the count limit is applied after the whole container has been scanned. R6 says the 4,097th entry is rejected right after its own header validation, so garbage after entry 4,097 is never examined. The oracle follows R6. No vector has a malformed entry after the 4,097th.
Rewrite R15: "`parse` scans entries in file order and stops at the first error. For each entry the checks are, in order: `TruncatedEntryHeader`, `InvalidEntryLength`, `EntryOutOfBounds`, then `TooManyEntries` if 4,096 entries were already accepted. Bytes after the first rejected entry are never examined." Add vectors: 4,096 entries accepted; 4,097 valid entries then 3 stray bytes returns `TooManyEntries`; 4,097th entry with length 7 returns `InvalidEntryLength`.

---

## Should-fix

### S1. R9 signature classification flags legitimate legacy payloads
"Any entry whose payload starts with the PNG or JP2 signature, regardless of type" includes `is32`..`it32` and the `*8mk` masks. A mask is arbitrary alpha bytes, so eight alpha values `89 50 4e 47 0d 0a 1a 0a` are legal and the entry becomes `opaque_candidate`. The same bytes also parse as a valid RLE start (`89 50` is a 14-pixel run of 0x50). The spec's own last sentence ("still reject malformed legacy payloads") concedes the entry stays decodable, so the flag is wrong for a caller that routes on it. gdk-pixbuf only delegates for `ic08`/`ic09` (io-icns.c:259–278).
Rewrite: "Signature recognition applies only to entries whose type is not one of the eight R8 types. R8 types are never `opaque_candidate`." Update vector `png-in-legacy-rgb-is-not-opaque-decode` to also assert the `parse` result (`opaque_candidate:false`), and add a `s8mk` entry with a PNG-signature-prefixed 256-byte payload.

### S2. OutputTooLarge is unreachable and unplaced (R14, R15)
Largest output is 4·128·128 = 65,536 bytes against a 4,194,304-byte limit, so no input can produce `OutputTooLarge` and no vector can test it. R15 does not place it in the precedence list; the oracle puts it after `TypeMismatch`. Either delete it or state it as a defined unreachable branch.
Rewrite R14: "The RGBA length `4N` is at most 65,536 for every R8 type, so `OutputTooLarge` is never returned by this contract; implementations must not define it as a reachable result." (Or keep it and add "evaluated after `TypeMismatch`".) The R10 "at most 49,152 bytes" remark is the same kind of dead bound; state it as a derived fact, not a limit.

### S3. No consolidated error list
The 20 names occur inline: FileTooLarge, TruncatedHeader, BadMagic, InvalidFileLength, TruncatedFile, TrailingFileBytes, TruncatedEntryHeader, InvalidEntryLength, EntryOutOfBounds, TooManyEntries, UnsupportedType, IndexOutOfRange, RleTruncated, RleOverrun, RleTrailingBytes, InvalidMaskLength, MissingColor, MissingMask, TypeMismatch, OutputTooLarge. A verified enum must be closed.
Add after R16: "The complete error set is exactly the 20 names above; no other error value exists. A function returns the first error in the order given in R15."
Also note there is no "underrun" error: both a missing control byte and missing data return `RleTruncated` (the vector name `rle-second-channel-underrun` uses a word the spec lacks).

### S4. Pixel layout is stated only for combine
R10/R12/R13 never say that channel index `i` is `y*width+x`, top row first. Row order, and that the RGB output is R,G,B per pixel, appear only in R14.
Add to R10: "`decode_rgb` returns `3N` bytes, `out[3i..3i+3] = R[i],G[i],B[i]`, `i = y*width + x`, top row first." Add to R13: "`alpha[i] = payload[i]` with the same `i`."

### S5. R11 contains a factual error
"four leading zeros are ambiguous with four valid one-byte literal runs". Control 0x00 plus one data byte is one run of two bytes, so `00 00 00 00` is two one-byte literal runs of value 0 (two black red-channel pixels, one pixel each), not four. The ambiguity is real but narrower: only an `it32` red channel that starts with two isolated zero-valued pixels written as separate literals. Other facts:
- rust-icns strips the four zeros for every RLE24 type, not just `it32` (element.rs:557–562); gdk-pixbuf only for `it32`. The spec follows gdk. Keep that, but say so.
- Wikipedia (via search snippet, RECALLED) says the `it32` header value is ignored and conventionally zero. R11 strips only zeros; a nonzero 4-byte header would then be parsed as RLE and fail. Neither reference loader handles that either, so keep, and record it as a DECISION.
Rewrite R11's DECISION: "`00 00 00 00` could also be two one-byte literal runs of value 0; this contract always treats it as the prefix for `it32`. rust-icns also strips it for `is32`, `il32`, `ih32`; this core does not."

### S6. Strict file-length and suffix rules (R4, R5 DECISIONs) are stricter than both reference readers
Evidence read: gdk-pixbuf accepts bytes past a shorter declaration (io-icns.c:91–96). rust-icns `IconFamily::read` loops `while file_position < file_length` and never looks past the declared length (family.rs:138–150), so a trailing suffix is silently ignored there too. No source here shows macOS tools writing trailing bytes; that claim is RECALLED-free but unverified, so I cannot show these decisions reject real files. The risk is files modified by tools that append padding or a signature after the icns body (RECALLED, plausible, not confirmed).
Recommendation: keep `TrailingFileBytes` and `TruncatedEntryHeader` but say in the DECISION text that both major readers tolerate them, and require callers who need leniency to truncate the slice to the declared length before calling `parse`.
Note: the DECISION on RLE trailing bytes (R12) is supported: rust-icns also errors on leftover input (element.rs:589–593) and on a short or overlong run (element.rs:596–632).

### S7. Vector coverage gaps
Requirements without a vector:
- `FileTooLarge` (needs a generated >67,108,864-byte input; add a vector form such as `{"input_len": 67108865, "fill_hex": "..."}` or drop the clause from conformance and test it separately).
- R2 vs R3 precedence: 8-byte input with bad magic and `InvalidFileLength` returns `BadMagic`; `icns` with length 0 and 20 bytes returns `InvalidFileLength` before `TruncatedFile`.
- 4,096 entries accepted (B3).
- RleOverrun checked before data is read: payload `ff 11 ff` (second control overruns with no data byte) must return `RleOverrun`, not `RleTruncated`.
- RleOverrun from a literal control (e.g. `7f` with 100 remaining) and in the green and blue channels.
- A run that crosses a channel boundary (must be `RleOverrun`).
- `it32` payload of exactly `00 00 00 00` (prefix stripped, then `RleTruncated`); `is32` payload beginning `00 00 00 00` decoded as RLE, not stripped.
- `IndexOutOfRange` for index equal to length and for the mask index; `UnsupportedType` for `decode_mask` on a colour type; `combine` with colour error before mask error.
- `combine` with the same index for colour and mask.
- Empty-payload `rgb` for each legacy type.
- Opaque ic04/ic05/icsb with a PNG signature (should be `opaque_candidate:true`, `nominal_side:null`).
- Legacy entries in `parse` (is32/s8mk, ...) asserted `opaque_candidate:false` for all eight types.

### S8. The vector format is undocumented
VECTORS.json uses `operation`, `color_index`/`mask_index` (absent means None), and `*_runs` with `{count, hex}` per group of 3, 1 or 4 bytes. None of that is in SPEC.md or SOURCES.md. Add a short README or a schema paragraph. Also say that `parse` is run first in every vector (an erroring parse makes `rgb`/`mask`/`combine` vectors report the parse error).

### S9. R9 is silent on formats the real world uses for the same type codes
Wikipedia (search snippet, RECALLED) says `icp4` and `icp5` may hold 24-bit RGB (RLE) data, not only PNG/JP2. The spec treats them as opaque by type, which is safe, but it should say the core never decodes `icp4`/`icp5` as RLE (`decode_rgb` returns `UnsupportedType`). rust-icns also assigns nominal sizes to `ic04` (16), `ic05` (32 via 16@2x) and `icsb` (18) (icontype.rs:132–134); R9 gives none. `nominal_side` for those is `None` in the vectors; write that down.

### S10. SOURCES.md self-description overstates
"No format fact below relies solely on memory; no claim is marked RECALLED." The R9 mappings rest on one third-party Rust crate that the file itself says is not normative, and the Apple documentation was not checked. That is a source-limited claim, not a verified one. Replace with: "Type-code mappings (R9) are VERIFIED against one third-party implementation only; the Apple format is unverified."

---

## Nits

- N1 R1: say that on 32-bit targets indices are narrowed from `u64` only after the check against `len(bytes)`. The u32 length fields cannot overflow `u64` anyway, so the "checked" wording is vacuous but harmless.
- N2 R8: add that every other legacy type (`ICON`, `ICN#`, `ics#`, `ics4/8`, `icl4/8`, `ich#/4/8`, `icm*`, `TOC `, `icnV`, `name`, `info`) returns `UnsupportedType` from the decode functions, and that pre-OS X icons lacking a 32-bit variant therefore cannot be decoded.
- N3 R14: `TypeMismatch` is reachable only when both entries decode and have different sizes, because the R8 pairing is one-to-one by size. State that, so implementers do not add a separate pair table check that disagrees.
- N4 R16: "consume no decoded-pixel allocation during parsing" is not testable by vectors; either drop it or restate as "`parse` allocates only the entry list".
- N5 Vector name `repeat-129-boundary`: control 0xfe is a 129-pixel run; the maximum is 130 (covered in `repeat-3-and-130-boundaries`). Rename `repeat-129-and-127`.
- N6 SOURCES.md says the C comment at io-icns.c:187–190 is "imprecise". It states "identical bytes + 125" and "non-repeating bytes − 1", which matches the code at :212 and :227. Remove the remark.
- N7 R2: "rejects `len(bytes) > 67,108,864`" followed by "otherwise rejects fewer than eight" is clear, but state the order once as R15 does for the others ("`FileTooLarge` is checked first, on length alone").

---

## DECISIONs against real files (task step 2)

| Req | Decision | Rejects real macOS output? |
| --- | --- | --- |
| R4 | exact file length equality | Not shown. Both reference readers tolerate trailing bytes (S6); no evidence macOS writes them. |
| R5 | 1–7 byte suffix rejected | Same as R4. |
| R7 | no duplicate winner | No rejection; moves choice to caller. |
| R9 | exact type or full-signature match | No rejection (classification only). S1 is a false-positive risk. |
| R10 | no 4N ARGB path | Not shown. Real ARGB (ic04/ic05/icsb) starts with ASCII `ARGB` and is RLE (element.rs:220–236, RECALLED for Apple's side), a different type from is32; a raw 4N `is32` has no cited producer. |
| R11 | zero prefix only for it32, and only zeros | Real `it32` always has four zero bytes (matches rust encoder, element.rs:481). Nonzero header values are said to be ignored by Apple (RECALLED); not handled. |
| R12 | reject trailing and truncated RLE | Supported by rust-icns, which rejects both. |
| R13 | exact N-byte mask | Supported by both references (io-icns.c:283; element.rs:255). |
| R14 | no default mask | Behaviour, not a rejection of valid files; callers wanting opaque output must build a mask themselves. |

## Completeness for formal verification (task step 4)

Missing sentences, in order of priority:
1. Entry record and scalar types (B1).
2. The base for the EntryOutOfBounds comparison (B2).
3. Stop-at-first-error scan semantics (B3).
4. The closed error set (S3).
5. Row-major output layout for `decode_rgb`/`decode_mask` (S4).
6. `OutputTooLarge` reachability and position (S2).
7. `nominal_side` for non-listed types (B1, S9).

Determined and consistent with the oracle: u32 big-endian length fields; counts 1..128 literal and 3..130 repeat; per-channel `produced` reset with a shared cursor; overrun check before run data is read; truncation on a missing control byte or data byte; trailing bytes only checked after the blue channel; the `it32` prefix is removed before RLE only when it is present; the input limits 67,108,864 bytes and 4,096 entries.

## Vectors recomputed by hand (25)
minimal-empty, short-header, bad-magic, invalid-file-length, declared-longer, declared-shorter, truncated-entry-header, entry-size-seven, entry-exceeds-file, entry-u32-max, trailing-after-entry, unknown-type (payload [16,18]), duplicate-order (payload [16,17],[25,26]), rgb-is32 (130+126 per channel), repeat-3-and-130-boundaries (3+130+123), literal-1-boundary (1+130+125), repeat-129-boundary (129+127), literal-nonuniform-rgb (2+130+124, first pixel 102233), rle-empty, rle-control-without-value, rle-literal-short (128-byte literal with 1 byte left), rle-first-channel-overrun (130 then 130>126), rle-second-channel-underrun (red completes, payload ends), rle-trailing-data, mismatched-mask-dimensions (8+20+1032 = 1060), entry-count-limit (8+4097·8 = 32,784). All match the spec. `argb-not-opaque-*` payloads are the ASCII bytes `ARGB` plus filler, as intended.
