# Second independent review of the ICO-CORE-SPEC 1.0 redraft (per-entry validity, total decode_entry, DIB 1.1 slack mode)

Reviewer: independent of the author. Marking: RECALLED = not read from a primary source here. Read in full: redrafted SPEC.md, DESIGN-NOTES.md, SOURCES.md, check_vectors.py, tools/gen_vectors.py and run_corpus.py (headers and main logic), THEOREMS-DRAFT.md, the 200 vectors (by mechanical diff against the 153-vector draft), plus the DIB 1.1 amendment (reviewed separately in `/home/ws/projects/dib/spec-1.1/REVIEW.md`).

Counts: **blocking 2, should-fix 5, nit 3.**

Verdict: **freeze after listed fixes**, and only after the DIB 1.1 amendment is frozen: the ICO checker pins the SHA-256 of the DIB 1.1 oracle file, and the DIB review's blocking fix changes that file. The spec text is consistent and the decode precedence is total. Both blocking items are vector gaps in the new feature (forged-record comparison, per-field) and in vectors that the trimming from 153 to 200 removed.

## Checks run

- `python3 -I spec/check_vectors.py`: 200 passed. `sha256sum` of `/home/ws/projects/dib/spec-1.1/check_vectors.py` equals the pinned `214dbf12...`.
- Own oracle v2 from SPEC.md R1-R12 (`/tmp/claude-0/---modal-volumes-vo-Iyh3VLkOB7cEHrIp2RkTs7/07bf9765-af64-477d-a906-dbb14d47e2dd/scratchpad/myico2.py`), with DIB bytes from my own derived slack oracle (frozen 1.0 oracle plus the R19 semantics; not the author's 1.1 file): **0 disagreements on 200/200**, including `entries_override` vectors and the 64 MiB vectors.
- Corpus with my oracle: 13 files, 12 parse, `no_cursors.cur` is `EmptyDirectory`; 18 entries (2 PNG, 16 DIB); **13 DIB decode, 3 error**: `mask_1.cur` and `mask_L.cur` `Dib(TruncatedHeader)` (declared size 8), `decompression_bomb.ico` `Dib(InvalidDimensions)`. Matches the redraft's claims. The 13 include `hopper_mask.ico` and `pillow.ico` entries 1-3.
- Mutation sweep, adapted and extended to the new API: 121 single-point mutants of my oracle (including one harness artefact for the input cap, which dies once the oversized input is materialised). Survivors: 3 equivalent mutants and 13 real ones: B1 (9: seven field-subset comparisons, two precedence variants), B2 (3) and S1 (1, revalidate all supplied records).
- Cross-check of slack-mode pixels against the installed gdk-pixbuf 2.42.12 (own run; version may differ from the inspected source): `hopper_mask.ico` and `pillow.ico` entry 1 (48x48) are pixel-identical to the baseline output, and the baseline output for `mask_1.cur` and `mask_L.cur` is pixel-identical to decoding from `offset` to EOF with slack mode (used in D11 below).

## Status of my earlier findings (REVIEW.md)

| Finding | Status |
| --- | --- |
| B1 classification (read past payload, near-miss signature, shifted) | **Fixed.** `sig_beyond`, `sig_nofirst`, `sig_nolast`, `sig_nomid`, bytes 3/4/7, `sig_anyoff` mutants all killed (vectors `png-prefix-followed-by-rest-7`, `png-bit-*`, `png-shifted`). |
| B2 range fields (offset vs table end, narrowing, signed) | **Fixed.** `ov_thisrec`, u16/u24/i31 size and offset, wrap mutants all killed. |
| S1 decode vector semantics on parse failure | **Fixed** (R12, vectors `decode-parse-header-error`, `decode-count-cap-before-index`, `decode-short-table-before-index`). |
| S2 D5 slack | **Fixed** by DIB 1.1 slack mode; 13/16 DIB candidates decode. |
| S3 D9 per-entry validity | **Fixed.** `parse_abort_invalid`, `parse_drop_invalid`, `dec_any_invalid_blocks` killed. |
| S4 D10 total decode | **Fixed in text and model**; the comparison coverage is the new blocking item B1. |
| S5 T9 combined bound | **Fixed** (ICO-side only). See manifest section for a cheaper DIB export than the amendment proposes. |
| S6 pin of the DIB oracle | **Fixed** (SHA-256 in `check_vectors.py` and `gen_vectors.py`). Dependency-order issue: new S5 below. |
| S7 1,024 distinct records | **Fixed** (`rec_i255`, `rec_i1023` killed). |
| S8 mutation list | **Fixed** (M22-M30). Gaps: new S3. |
| S9 T11 proportionality | **Fixed** (pixel witnesses dropped). |
| S10 allocation wording | **Fixed** (R2 says source audit). |
| N1 PNG vs DIB sentence | **Fixed** (R6). N2 `n` term: not changed (still harmless). N3 rationale: **fixed** (R2 "policy"). N4 baseline rejects count 0: **fixed**. N5 RT_GROUP scope: **fixed** (SPEC.md:3). N6 straddling vector: **fixed** (`straddling-ranges`). |

## Blocking

### B1. Forged-record comparison is only tested on three of the ten compared fields (width, raw_offset, status), and its precedence against a wire defect is untested (VECTORS.json `forged-*`; SPEC.md:19, R10)
`decode_entry` returns `InvalidEntryRecord` unless the supplied record equals the reconstruction field for field. The forged vectors change only width, kind, payload end, status (`forged-oob`), list length. Mutants that compare a subset survive all 200 vectors: skip `color_count`, `planes`, `bit_count`, `hotspot_x`, `hotspot_y`, `height`, or `raw_size` (the last on invalid records, where the status carries no payload). Two further survivors concern precedence: (a) "selected defect before record mismatch" (R10 says mismatch first) and (b) "a supplied record that claims `Valid` for a wire-invalid record is accepted" both pass, because no vector forges a record whose wire status is invalid. THEOREMS M29 admits this ("add a dual-defect forged fixture if required"): it is required. Add `entries_override` vectors, one per field: a valid 1-entry file, supplied record identical except one field changed (height, color_count, planes, bit_count; for a CUR file hotspot_x, hotspot_y; raw_offset; raw_size on an invalid record), each expecting `InvalidEntryRecord`; and for a wire-invalid selected record (e.g. reserved byte 1): (i) the correct supplied record expects `InvalidEntryReserved`, (ii) a supplied record with `status` changed to `Valid{Dib,...}` expects `InvalidEntryRecord`, (iii) a supplied record with a different defect name expects `InvalidEntryRecord`.

### B2. The trim from 153 to 200 removed vectors that were the only killers of three obvious mutants
Survivors in the new suite: accept `type=3` (only type 0 and the precedence vectors remain; dropped `type-3`, `type-4`, `type-256`), compare only the low byte of the reserved word (dropped `reserved-2/255/256`; `reserved-1` remains), and compare only the low byte of the type word. These were killed by the 153-vector suite. Restore `type-3`, `type-256` (and add `type-257`, `type-513`, whose low byte is 1 or 2), `reserved-256` and `reserved-255`. The same trim also dropped `short-dir-1..4`, `short-table-1/2/7/8`, `png-9/16/24/32`, `dimensions-*` permutations and `payload-gap-23/1024`; none of those caused a surviving mutant in my set, so I do not ask for them back.

## Should-fix

### S1. R8 does not say whether unselected supplied records are consulted (SPEC.md:19 "No other entry needs revalidation")
"Needs" is not "must not". An implementation that revalidates every supplied record returns `InvalidEntryRecord` when an unselected record is forged; one that follows the model (`MD` reads only `|E|` and `E[i]`) decodes. Both pass all 200 vectors (mutant `dec_cmp_all_entries` survives). The revalidating variant also costs `O(count)` per call, which still fits `steps(wrapper)<=16*(|b|+1)` (count <= |b|/16), so T9 does not catch it either. Say "only `entries.len()` and `entries[index]` are read; no other element is inspected", add a vector (right length, `entries[0]` correct, `entries[1]` garbage, decode 0 succeeds), and add the mutant to the manifest.

### S2. Stale text in DESIGN-NOTES.md and SOURCES.md
SOURCES.md:48 says "The new 198-vector draft"; the suite has 200. DESIGN-NOTES.md:19 says "Two of the 12 parsed files still cannot yield DIB pixels"; it is three (`mask_1.cur`, `mask_L.cur`, `decompression_bomb.ico`), and a fourth, `hopper_draw.ico`, is PNG-only. DESIGN-NOTES.md:23 repeats the 1,367 probe figure from the shipped crate; I re-ran only the dependent probe (1 verified, 0 errors).

### S3. Manifest gaps (THEOREMS-DRAFT.md)
- Add: "compare only a subset of the record fields" (B1), "revalidate unselected records" (S1) under T5/T6 (the latter also T9 audit: it is a real loop the ghost counter must charge).
- M29 should drop "if required".
- T5: "invoke the same exported DIB call": name it. See the cross-crate section.

### S4. R8 states `decode_entry` calls `dib_verus::decode(s, IcoEntryDibSlack)`; the manifest wants a trace-connected call (SPEC.md:19, 29; THEOREMS T9)
Choose one name. See "What the DIB 1.1 proof must export".

### S5. Freeze order and hash churn
The ICO checker pins the DIB 1.1 oracle at `214dbf12...`. The DIB review's blocking fix (new slack vectors, possibly oracle edits) changes that file and its VECTORS.json, so the ICO vectors' expected pixel runs stay valid but the pin and `gen_vectors.py` hash must be re-taken after DIB 1.1 freezes. Write that ordering in SPEC.md:3 ("depends on the proposed...") and in the freeze checklist. Also record that the frozen 1.0 oracle hash (`01029baeccb7...`) is no longer the dependency identity.

## Nits

- N1. R13/T9: the `n` term is redundant (see REVIEW.md); harmless.
- N2. R8's last sentence ("A changed byte slice can produce ... another legitimate result if the selected reconstructed record is byte-for-byte identical") is correct but belongs in DESIGN-NOTES; the spec should state the rule, not the consequence.
- N3. `entries_override` is a harness field (R12) and `check_vectors.py` accepts it only on decode vectors by convention; assert that.

## `decode_entry`: total, and exactly what it returns

The model is total: every `(bytes, entries, index)` reaches one of the rows below, in this order, and each row's condition is decidable from `|b|`, the 6-byte header, `entries.len()`, `entries[index]` and the selected 16-byte wire record only. I could not construct two readings that disagree except S1.

| # | Condition (tested in this order) | Result |
| --- | --- | --- |
| 1 | `|b|>67,108,864` | `FileTooLarge` (before any read; `entries` and `index` irrelevant) |
| 2 | `|b|<6` | `TruncatedDirectory` |
| 3 | header reserved word `!=0` / type not in {1,2} / count 0 / count>1,024 / table past EOF | `InvalidReserved` / `UnsupportedType` / `EmptyDirectory` / `TooManyEntries` / `TruncatedEntries`, in that order; supplied list irrelevant |
| 4 | `index>=count` (wire count, not `entries.len()`) | `IndexOutOfRange` |
| 5 | `entries.len()!=count` | `InvalidEntryRecord` |
| 6 | `entries[index]` differs in any field or status from the reconstruction of wire record `index` | `InvalidEntryRecord` (including a forged record for a wire-invalid entry, B1) |
| 7 | reconstruction has `Invalid(e)` | `e` (`InvalidEntryReserved`, `EmptyEntry`, `EntryOverlapsDirectory`, `EntryOutOfBounds`, in that order of first match) |
| 8 | reconstruction is a valid `Png` range | `PngDelegation` (DIB not called) |
| 9 | reconstruction is a valid `Dib` range `[a,z)` | `dib.decode(b[a..z], IcoEntryDibSlack)`: `Ok(Decoded)` or `Dib(e)` with the DIB 1.1 first error |

Rows 5-6 are the only places the supplied list matters; unselected invalid wire records and unselected supplied records do not change any row (S1 pins the second half). `parse` is the same rows 1-3, then `Ok` of the full list of `ME` records, each with its row-7 status (no record can fail the list). Verified by oracle: decode vectors cover rows 1-9 except row 3 with `FileTooLarge`/`InvalidReserved`/`UnsupportedType`/`EmptyDirectory` (the first-error order within row 3 is covered by the parse vectors; the decode wrapper reuses the same guard, so I rate this a gap only for a wrapper that reimplements the guard separately, which I recommend adding to the manifest as T6 text, not as vectors).

## Manifest (THEOREMS-DRAFT.md)

- T2/T3/T4/T6/T7/T8: statable and exact against a total `MC`; the per-entry design makes T2 simpler than before (no early error return inside the loop, only a status per record).
- T5 total decode: feasible. Needs a spec `me(b,i)`, an exec field-by-field equality function (Verus has no derived `PartialEq`; write it and prove it equals spec equality, 10 fields), and the DIB call. Index arithmetic: `index<count<=1024` bounds `6+16*index`, so no overflow on 32-bit `usize`. Moderate.
- T9: ICO-side bound only is provable; the numerics are loose (`n` redundant) but correct. Do not let the wrapper loop over entries (S1).
- T10 audit-only: honest.
- T11: proportionate now.
- Mutations: M9-M30 each map to a vector that fails under my corresponding mutant, with the exceptions in B1/S1/S3. The audit-only labels (M20, M21) remain honest.

### What the DIB 1.1 proof must export, and the cross-crate composition statement
1. A public mode `IcoEntryDibSlack` and public `model::mp` / `dib::decode_mp_eq` that cover it (the existing `decode_mp_eq` quantifies over `Mode`, so no new statement is needed, only re-proof).
2. A public function whose body is the one `decode` runs and whose ensures clause includes `steps<=16*(|b|+N)` and `decode_mp_eq`: the existing `t12_linear_work` (`dib.rs:5388`, private, `#[allow(dead_code)]`) made public under a new name. The ICO crate then calls this function **instead of `decode`** and gets a connected bound with no change to the executable (the ghost return is erased). This is cheaper than the amendment's THEOREMS-DELTA suggests (see DIB REVIEW S4).
3. Composition statement for T5: for `b,E,i`, `decode_entry(b,E,i)=MD(b,E,i)`, where for a selected valid Dib `[a,z)` the DIB call on `&b[a..z]` returns `r` with `decode_mp_eq(r, b@.subrange(a,z), IcoEntryDibSlack)`, and `MD` is defined with `dib_verus::model::mp(b.subrange(a,z), IcoEntryDibSlack)`; mapping `Ok(d)` to the same `Decoded` and `Err(e)` to `Dib(e)`. The probe I re-ran shows this shape verifies across crates for the 1.0 crate (1 verified, 0 errors).

## D11: under-declared entry sizes (open owner decision)

Evidence I can establish:
- Of the 18 declared ranges in the 12 parsed corpus files, 16 end exactly at the next entry's offset or at EOF. The only two that leave bytes unclaimed are `mask_1.cur` (72 bytes) and `mask_L.cur` (1,088 bytes); both are one-entry files whose DIB, from `offset` to EOF, is exactly complete (80 and 1,096 bytes; strict `IcoEntryDib` already accepts it; no slack). Their declared size 8 is not a stale padding value but a wrong constant; I cannot establish which tool wrote it, and both are Pillow test fixtures, so they may be hand-built. Real multi-entry producers in the corpus (`python.ico`, `pillow.ico`) have exact ranges.
- The installed gdk-pixbuf loads both files and its output is pixel-identical to decoding `[offset, EOF)`; it ignores the size field altogether (`io-ico.c:289,297`). RECALLED, not verified here: Pillow's own ICO/CUR reader and browser decoders also read from the offset and do not use the size field to bound the DIB.
- The caller already has what it needs: `Entry.raw_offset` is retained for every record, valid or not, and slack mode never reports `TrailingBytes`. A caller that wants recovery writes `dib_verus::decode(&bytes[e.raw_offset as usize..], IcoEntryDibSlack)` after `decode_entry` returned a truncation error. No core change is needed to enable it.

Recommendation: **keep the exact range in the core** (agree with the author). Reasons: the recovery policy cannot be made safe in general (a later entry's bytes become part of this image) and each variant needs its own vectors and proof; the evidence supports a narrow rule ("an entry whose offset is the largest of all entries, with a truncation error, may be extended to EOF"), which is safe by construction because no entry starts later, but it adds an `O(count)` scan to the wrapper and an interplay with aliases, and it covers exactly the two files in a 13-file sample. If the owner wants it, specify it as a separate documented caller-side helper (not in `decode_entry`), and say in DESIGN-NOTES that the one-line caller workaround exists. Owner decision still required, but I see no reason to delay freeze for it.

## Ready to freeze?

**Freeze after:** B1, B2 (new and restored vectors; re-run the sweep, I expect every real survivor to die), S1-S5 text, and the DIB 1.1 freeze (hash re-pin). Nits do not block.

# REVIEW addendum (final recheck of the 217-vector draft)

Verdict: **ready to freeze, but only after DIB 1.1 is frozen**, because `spec/dib_oracle_pin.py` pins the draft DIB 1.1 oracle hash (`214dbf12...`, unchanged since my first review and still matching): freeze DIB 1.1, re-take the pin, regenerate and rerun both ICO tools, then freeze ICO.

- **Oracle on all 217 vectors:** my oracle v2 (DIB bytes from my own slack oracle, not the author's 1.1 file) agrees on **217/217, 0 disagreements**. `check_vectors.py`: 217 passed; `run_corpus.py`: 12 of 13 parse, 13 DIB decode, 3 DIB errors (`mask_1.cur`, `mask_L.cur` `Dib(TruncatedHeader)`, `decompression_bomb.ico` `Dib(InvalidDimensions)`), unchanged and independently confirmed with my oracle.
- **Mutation sweep** (121 mutants rerun on the 217 vectors; input-cap mutants checked with the oversized input expanded): of the **13 previous real survivors all are killed**: the seven field-subset comparisons (`height`, `color_count`, `planes`, `bit_count`, `hotspot_x`, `hotspot_y`, `raw_size`) by the new `forged-wrong-*` and `forged-hotspot-*` vectors; the two precedence survivors ("defect before mismatch", "forged Valid accepted for a wire-invalid record") by `wire-invalid-correct-record`, `wire-invalid-forged-valid`, `wire-invalid-forged-defect`; the three B2 survivors (type 3, low-byte reserved, low-byte type) by `type-3`, `type-256/257/513`, `reserved-255/256`; revalidate-all-supplied-records by `forged-unselected-ignored`. Remaining survivors: **0 real.** Three equivalent mutants remain (use the supplied status after equality is checked; test PNG before defect after the comparison; a no-op length helper), and the oversized-input cap mutant dies once expanded (`file-size-67108865`, `too-large-before-bad-header`, `file-size-67108864`).
- **Earlier findings:** B1 fixed (above). B2 fixed (above). S1 fixed (R8: "Only `entries.len()` and `entries[index]` may be inspected"; vector `forged-unselected-ignored`). S2 fixed (no "198", DESIGN-NOTES says three of 12 files, the 1,367 figure is now attributed to the shipped dependency run). S3 fixed (THEOREMS M29 re-pointed at the three `wire-invalid-*` vectors, new M31 field-subset, M32 validate-every-record, M33 low-byte type/reserved/type 3; every vector name in the manifest exists). S4 fixed (R8 names `dib_verus::decode_with_steps`, R13 states the bound is a theorem only after DIB exports it and ICO calls the same function). S5 fixed (SPEC.md:3 and DESIGN-NOTES state the order: freeze DIB 1.1, reprove, re-take the pin, rerun both tools; the pin lives in one file, `dib_oracle_pin.py`, shared by checker and generator). N1 fixed (R13 bound is `16*(|b|+1)`, `n` removed). N2 fixed (the consequence sentence moved to DESIGN-NOTES). N3 fixed (`check_vectors.py:119` asserts `entries_override` only on decode). D11 recorded as settled (keep exact range; caller-side retry).
- **Hand derivation (4 fixtures):** `forged-wrong-color-count`: the file is `00 00 01 00 01 00` (ICO, 1 entry) then `01 01 00 00 | 01 00 | 18 00 | 30 00 00 00 | 16 00 00 00` = 1x1, colour count 0, reserved 0, planes 1, bitcount 24, size 48, offset 22; the supplied record is identical except `color_count 7`, so row 6 of the precedence table gives `InvalidEntryRecord`. `forged-hotspot-y`: type 2 file, wire words `0b 00` and `0c 00` give hotspot (11,12); the supplied record says (11,13): mismatch, `InvalidEntryRecord`. `wire-invalid-forged-valid`: the fourth entry byte is `01`, so the wire status is `Invalid(InvalidEntryReserved)`; a supplied record claiming `Valid{Dib,[22,70]}` differs: `InvalidEntryRecord` (not the wire defect), while the same supplied record with the correct invalid status returns `InvalidEntryReserved`. `forged-unselected-ignored`: count 2, both records offset 38 size 48; the supplied list has the right length, `entries[0]` equal to wire record 0, and `entries[1]` claiming width 2 where the wire says 1; decode of index 0 reads only `entries.len()` and `entries[0]`, so it decodes the DIB successfully.
- **Manifest (THEOREMS-DRAFT.md, DESIGN-NOTES):** ICO-side-only work bound (`steps(parse)` and `steps(wrapper)<=16*(|b|+1)`), the cross-crate claim via a public `decode_with_steps` called by ICO instead of `decode`, no pixel evaluation witnesses, M22-M33 mapped to vectors that kill my corresponding mutants. I confirmed the DIB side (its THEOREMS-DELTA) now covers `model_vectors.rs`, the `Mode` enum break, `is_ico`/`exact_tail` helpers and `decode_with_steps`. No remaining manifest issue.
- **New ambiguities:** none found. The nine-row decode precedence table in this file still describes the behaviour exactly.
