# Re-review of ICNS-CORE-SPEC 1.0 against REVIEW.md

Inputs checked: SPEC.md and VECTORS.json hashes match FREEZE.txt (sha256 `aa9e0d9d...` and `7b6a723d...`). `python3 -I check_vectors.py` reports 107/107 passing.
Remaining: blocking 0, should-fix 1, nit 4.

## 1. Status of each earlier finding

| ID | Fixed? | Evidence in the revised SPEC.md | New ambiguity? |
| --- | --- | --- | --- |
| B1 entry record and scalar types | Yes | R1: "The returned entry record has exactly `{type_hex: string, payload: [u32,u32], opaque_candidate: bool, nominal_side: Option<u32>}`"; "Width and height are `u32`; an entry index is `u32`, and only `combine` accepts an absent index, represented by `None`". Preamble: forged entries "violate the API precondition, not an error result". `nominal_side` rule is stated twice (R1 and R9) and the two agree. | No. |
| B2 EntryOutOfBounds base | Yes | R5: "if `length > declared_length − offset`, measured from the **first byte of this entry's header**". Vectors `entry-length-equals-remaining` (accepted) and `entry-length-one-too-long` (`EntryOutOfBounds`) pin the off-by-eight. | No. |
| B3 TooManyEntries ordering | Yes | R15: "scans entries **in file order**, stopping at the first error ... then `TooManyEntries` when 4,096 entries have already been accepted; never inspect later bytes after an error, including a malformed entry following a valid 4,097th entry." Consistent with R6's "after validating that entry's header and length". | No. |
| S1 signature on legacy types | Yes | R9: "For any type **other than the eight R8 legacy colour/mask codes**, also classify ... Every R8 entry is never an `opaque_candidate`, even if its payload starts with those bytes." Vectors `png-signature-legacy-mask-stays-mask` and `...-colour-stays-colour`. | No. R1 gives R8 types `nominal_side=None` through "exactly when the type belongs to its fourteen-code table". |
| S2 OutputTooLarge | Yes (by removal) | R14 DECISION: "every permitted output is at most 65,536 bytes, so the ceiling is provably unreachable and this version does not define `OutputTooLarge` as an error." The name is absent from R17. See N-a for a wording nit. | No. |
| S3 closed error list | Yes | R17 lists 19 names (the earlier 20 minus `OutputTooLarge`) and states "missing RLE control or data bytes both return `RleTruncated`; no separate underrun error exists." Every error used in R2 to R15 appears in the list. | No. |
| S4 pixel layout | Yes | R10: "`out[3i..3i+3] = [R[i],G[i],B[i]]` where `i=y×width+x`, x increases left to right and y increases top to bottom". R13: "`alpha[i]=payload[i]`" with the same `i`. | No. |
| S5 R11 error | Yes | R11: "Four leading zeros could also represent **two** one-byte literal runs ... for `it32` this contract always interprets them as a prefix"; rust-icns's all-RLE24 stripping is recorded. Vector `is32-four-zero-literals` pins the non-`it32` case. | No. |
| S6 strict length and suffix rules | Documented, not changed | R4 and R5 now cite family.rs:138–151 and state "We found no evidence that macOS tools actually write such suffixes, and no evidence that they never do." R4 gives the caller a workaround (pass the declared-length slice). SOURCES.md hands the decision to the user. Adequate for a frozen spec. | No. |
| S7 vector gaps | Mostly (see S-1) | Added: FileTooLarge, precedence pair, 4,096 accepted, 4,097th variants, overrun-before-data (repeat and literal), green/blue overrun, `it32` four-zero-only, `is32` four-zero literals, index-equals-length, unsupported-mask-on-colour, same-index combine, error order in combine, legacy parse for all eight types, ic04/ic05/icsb with PNG. | The 67,108,864-byte acceptance boundary is still absent (S-1). |
| S8 vector schema | Yes | R18 defines the array shape, `input_repeat`, run encoding (maximal runs, fixed-width hex, counts sum to `side×side`), `{error: name}`, and "Every vector runs `parse` first". | No. |
| S9 icp4/icp5, ic04 nominal | Yes | R9: "`icp4` and `icp5` remain opaque even if their payloads could represent another encoding; `decode_rgb` rejects them with `UnsupportedType`"; "`nominal_side` for those three types is `None`". Vector `opaque-icp4-not-rgb`. | No. |
| S10 SOURCES overstatement | Yes | SOURCES.md: "Type-code mappings (R9) are **VERIFIED against one third-party implementation only**"; the RECALLED items are named. | No. |
| N1..N7 | Yes | N1 R1 narrowing text; N2 R8 list of older types; N3 R14 "equivalent to requiring the corresponding pair"; N4 R16 "allocates only its entry list"; N5 vector renamed `repeat-129-and-127`; N6 SOURCES now says the C comment and code agree; N7 R2 "checks input length first, without reading its bytes". | No. |

R12's pseudocode block matches its prose line by line (control split at 128, overrun test before the availability test, `needed` is `count` or 1, trailing test once after blue). I found no contradiction between the prose and the block.

## 2. Independent oracle over the 107 vectors

I extended my own oracle (`/tmp/claude-0/---modal-volumes-vo-Iyh3VLkOB7cEHrIp2RkTs7/07bf9765-af64-477d-a906-dbb14d47e2dd/scratchpad/myoracle.py`) with `input_repeat` and the R9 exclusion of R8 types, both taken from the revised text, not from check_vectors.py. Result: 0 disagreements with VECTORS.json on 107/107, and therefore none with check_vectors.py. Before I applied the R9 change the oracle disagreed on exactly the two new `png-signature-legacy-*` vectors, which is the expected sign that those vectors test the S1 fix.

## 3. New vectors recomputed by hand (21)

| Vector | Recomputation | OK |
| --- | --- | --- |
| entry-length-equals-remaining | declared 0x18 = 24; entry at 8 with length 0x10 = 16 = 24 - 8; payload [16,24] | yes |
| entry-length-one-too-long | same file, length 0x11 = 17 > 16 | yes (`EntryOutOfBounds`) |
| entry-count-exactly-4096 | 8 + 4096·8 = 32,776 = 0x8008 matches the header; first payload [16,16], second [24,24]; 4,096 entries returned | yes |
| entry-count-limit | 8 + 4097·8 = 32,784 = 0x8010; 4,097th valid | yes (`TooManyEntries`) |
| count-limit-before-stray-suffix | 32,787 = 0x8013; 4,097 entries then the 3 bytes `424144`; 4,097th is rejected before the suffix is looked at | yes |
| invalid-4097th-entry-before-limit | 4,097th length 7 | yes (`InvalidEntryLength` takes precedence) |
| entry-count-followed-by-overflowing-4097th | 4,097th length `ffffffff` | yes (`EntryOutOfBounds`) |
| entry-count-before-later-malformed-entry | 4,098 8-byte slots; entry 4,097 valid, entry 4,098 length 7 | yes (`TooManyEntries`) |
| file-too-large | 67,108,865 bytes of 0x00 (repeat count times 1 byte) > 67,108,864 | yes |
| bad-magic-before-length | `78787878 00000000` | yes (`BadMagic`, not `InvalidFileLength`) |
| invalid-length-before-input-mismatch | `icns`, length 0, 20 bytes | yes (`InvalidFileLength`) |
| repeat-overrun-before-missing-value | `ff 11 ff`: red = 130, next `ff` is 130 > 126, no value byte present | yes (`RleOverrun`) |
| literal-overrun-before-missing-data | `ff 11 7f`: `7f` is a 128-literal, 128 > 126, data absent | yes (`RleOverrun`) |
| green-channel-overrun | red `ff11 fb11` = 256; green `ff22 ff22`: second 130 > 126 | yes |
| blue-channel-overrun | red and green complete; blue `ff33 ff33` | yes |
| run-crosses-channel-boundary | `ff11 ff11`: byte-identical to `rle-first-channel-overrun` | yes, but duplicate (N-b) |
| it32-four-zero-prefix-only | `it32` payload `00000000` is stripped to empty, then no control byte | yes (`RleTruncated`) |
| is32-four-zero-literals | `00 00 | 00 00 | ff 11 | f9 11 ...`: red = 0,0 then 130+124 of 0x11 = 256; green and blue as `22`,`33` | yes (2×`002233`, 254×`112233`) |
| png-signature-legacy-mask-stays-mask | `s8mk` with 256 bytes starting `89504e47...` | yes (`opaque_candidate:false`) |
| png-signature-ic05 | `ic05` + PNG signature | yes (`opaque_candidate:true`, `nominal_side:null`) |
| combine-same-index | index 0 for both on a valid `is32`: colour decodes, mask step rejects a colour type | yes (`UnsupportedType`) |

## 4. Can it be implemented and verified without asking the author?

Yes. Every public function now has defined inputs, outputs, integer widths, error names, check order and termination rules, and an independent oracle written from the text agrees with every fixture. The open choices (R4, R5, R10, R11 strictness) are documented as DECISIONs with the evidence limits stated; they are product decisions for the user, not spec gaps. No exact questions remain for the author.

## 5. Remaining items

### Should-fix
**S-1. No vector for the input-size acceptance boundary.** `file-too-large` is at 67,108,865 bytes. An implementation that uses `>=` instead of `>` still passes every vector. Add `{"name":"file-size-at-limit","input_hex":"00","input_repeat":67108864,"operation":"parse","expected":{"error":"BadMagic"}}`: 67,108,864 zero bytes pass R2 and fail R3. (This is a vector-only change; no SPEC.md text needs to change.)

### Nits
- N-a. R14's DECISION sets a "defensive output ceiling" and in the same sentence says it is unreachable and not an error. A verifier cannot use it; delete the ceiling, keep the sentence "every permitted output is at most 65,536 bytes".
- N-b. `run-crosses-channel-boundary` has the same input and result as `rle-first-channel-overrun`. A crossing run can only fail the same `count > N - produced` test, so one vector is enough; drop one or rename.
- N-c. R1 says an entry index is `u32` while entries are capped at 4,096; the `IndexOutOfRange` condition is `index >= len(entries)` and is only implicit. Add "`IndexOutOfRange` when `index >= len(entries)`" to R10 and R13.
- N-d. No vector shows that only the first four zeros are removed for `it32` when the payload begins with eight zeros (the remaining `00 00 00 00` must be decoded as RLE). R11 says "exactly those four bytes", so the behaviour is specified; a vector would pin it.
