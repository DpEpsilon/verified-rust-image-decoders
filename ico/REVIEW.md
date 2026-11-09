# Independent review of ICO-CORE-SPEC 1.0 draft (SPEC, VECTORS, SOURCES, DESIGN-NOTES, check_vectors.py, THEOREMS-DRAFT)

Reviewer: independent of the author. RECALLED = from memory, not read from a primary source in this review. Read in full: all five spec files, `THEOREMS-DRAFT.md`, both tools, `io-ico.c` (1,427 lines), DIB `spec-frozen/SPEC.md` and `DESIGN-NOTES.md`, the DIB crate's public `decode`/`decode_mp_eq`, the TGA and DIB reviews. No Microsoft format specification was opened (neither did the author).

Counts: **blocking 2, should-fix 10, nit 6.**

Verdict: **freeze after listed fixes.** The spec text is internally consistent and the error order is total; my independent oracle agrees with all 153 vectors, so no vector and no sentence of SPEC.md is wrong. The two blocking items are vector gaps: bugs of the exact kinds the spec and manifest emphasise survive all 153 vectors, and the vector hash is about to lock them in. The rest is documentation, one unprovable manifest clause, and two owner decisions the draft does not list (D9, D10).

## Checks run

- `python3 -I spec/check_vectors.py`: 153 passed. It loads the DIB oracle by absolute path `/home/ws/projects/dib/spec/check_vectors.py`; its sha256 (`01029baeccb7...`) equals the hash FREEZE.txt records for the frozen DIB oracle (S6).
- Own oracle from SPEC.md R1-R12 (`/tmp/claude-0/---modal-volumes-vo-Iyh3VLkOB7cEHrIp2RkTs7/07bf9765-af64-477d-a906-dbb14d47e2dd/scratchpad/myico.py`). I had read `check_vectors.py` before writing it, so independence is of code, not of reading. It delegates DIB bytes to the frozen DIB oracle on `b[start:end]`. **0 disagreements on 153/153**, including both 64 MiB vectors and `count-exact-1024` fully expanded.
- Expected DIB results come from the frozen oracle on exactly the entry slice: confirmed. `gen_vectors.py:52,124-126` pass the payload bytes (whose length equals the declared size) to `dib.decode(..., 'IcoEntryDib')`; the checker and my oracle recompute from `b[start:end]` and agree on all 32 decode vectors. I hand-decoded `dib-basic-decode` (70 bytes = 6+16+40+4+4, BGR 00 00 ff gives `ff0000ff`) and `dib-mismatch-decode` (2x1, stride 8, 22+52=74 bytes, `00ff00ff`). Circularity is real but harmless: the ICO suite tests the slice boundaries, not DIB pixels (SOURCES.md says so). Only 5 decode vectors succeed and all are 24-bit 1x1/2x1; that is enough for slice and mode mutants (all killed) but not for DIB itself.
- Mutation testing: 116 single-point mutants of my oracle (parse and decode; scripts `mut.py`, `mut2.py`, `mut3.py` in the same scratchpad). 90 killed; 3 equivalent (count-zero check after the cap check; `off>=n` in addition to the size check; my own `idx<=len` coding); **23 real survivors in 8 groups = B1, B2, S7 and two nits below**. Coverage otherwise good: every one of the 13 container error names plus six DIB error names appears; every directory field is exercised; 256 convention (0,0 / 0,1 / 1,0 / 255); CUR hotspot raw 65535/500; `offset+size == |b|`, `== |b|+1`, `offset == |b|`, `|b|+1`; u32 wrap (`22+0xffffffff`); 1024/1025/4096/65535 counts; identical and nested overlap; 7-byte/8-byte/9-byte signatures; precedence pairs reserved/type, type/empty, empty/table, count/table, reserved/size, size/overlap, overlap/range, earlier/later record.
- Cross-crate probe: I re-ran `cargo verus verify --manifest-path /tmp/ico-cross-crate/ico_probe/Cargo.toml` (PATH `/opt/cargo/bin:/opt/verus`): `1 verified, 0 errors`. The probe's `dib_verus` copy is byte-identical to `verified-decoders/dib/dib_verus` (`diff -rq`). I did not reproduce the 1,367 figure for the dependency.
- Baseline behaviour, empirically: the installed gdk-pixbuf 2.42.12 (`gdk-pixbuf-thumbnailer`; version may differ from the inspected `io-ico.c`) on the 13 corpus files and on seven files I built in scratch from `black_and_white.ico`'s DIB (not saved in the project). Results in S2/S3.

## Blocking

### B1. Classification blind spots: reading past the payload end, near-miss signatures, signature not at offset 0 (VECTORS.json; SPEC.md:15; THEOREMS T4)
T4 says "Classification must not read past a short range" and M13/M14 target the length rule, but every `signature-prefix-n` and `signature-decode-n` vector (n=1,3,7) puts the prefix at the very end of the file, so an implementation that reads eight bytes at `offset` regardless of `size` cannot be told apart. Every 8+-byte payload is either the exact signature or a DIB header beginning `28 00 00 00`. Mutants that survive all 153 vectors:
- `kind=Png` iff `b[off:off+8]==SIG` (ignores `size`; also `sig_wholefile`). Concrete input: header, one entry with size 7 and offset 22, payload `89 50 4e 47 0d 0a 1a`, then one trailing byte `0a`. Spec: `Dib`; mutant: `Png`. Same for sizes 1..6 with the rest of the signature following in the file.
- first byte not compared (`pay[1:8]==SIG[1:8]`): payload `88 50 4e 47 0d 0a 1a 0a`.
- last byte not compared (`pay[:7]==SIG[:7]` with `len>=8`): payload `89 50 4e 47 0d 0a 1a 0b`.
- middle bytes not compared (bytes 4-5 skipped).
- signature found anywhere in the payload: payload `00 89 50 4e 47 0d 0a 1a 0a`.
Add parse and decode vectors for: size-7 payload followed by the eighth byte; a one-bit corruption of each of bytes 0, 3, 4, 7 at length 8 (expect `Dib`, and `Dib(UnsupportedHeaderSize)` on decode); signature shifted by one byte; and a second entry whose range starts inside the first entry's signature. Map them to M13/M14 and a new M22 (below).

### B2. Range-field blind spots: offset checked against the wrong directory end, 16-bit and signed narrowing (VECTORS.json; SPEC.md:11)
- Mutant `offset < 6+16*(i+1)` (end of the current record, not of the whole table) survives. Every multi-entry vector has offsets at or beyond `dir_end`, and the only overlap vectors have count 1. Concrete input: count 2, entry 0 with offset 22 (the start of entry 1) and a positive size that fits; expect `EntryOverlapsDirectory`. Add also offset 37 and 38 with count 2 (boundary `dir_end-1`/`dir_end`), and an entry-1 offset inside the table with a valid entry 0.
- Mutants that read `size` or `offset` as u16, u24 or signed i32 survive (7 variants tried), because the only wide values used are `0xffffffff` (all narrowings of it are still out of bounds). Concrete inputs: size `0x00010001` at offset 22 in a 70-byte file (u16 gives size 1, accepted; spec: `EntryOutOfBounds`); size `0x00010000` (u16 gives 0: `EmptyEntry` instead of `EntryOutOfBounds`); size `0x80000000`; offset `0x00010016` (u16 gives 22); offset `0x80000016`. The baseline itself carries a signed-int offset check (`io-ico.c:316,338`), so a signed reading is a live implementation risk, and the manifest's M12 only mutates the add. Add one vector per field for 2^16+small, 2^31, and 2^32-22 style values, and pair each with `offset+size` exactly equal to `|b|` modulo 2^32 for the wrap case.

## Should-fix

### S1. R12 does not define a `decode` vector whose `parse` fails (SPEC.md:27,19-23; check_vectors.py:79-87)
The oracle runs `parse` first and returns the parse error. SPEC R8/R11 define `decode_entry` only for provenance-valid entries, so `MD` calls such a triple `OutsideDomain` and no API call corresponds to it. No decode vector exercises it (decode errors in the suite: Dib(...), IndexOutOfRange, PngDelegation only). State in R12 that a decode vector means "run `parse`; it must succeed; then `decode_entry`" and have the checker assert it, or define the combined behaviour. Today two implementers of the harness can differ on a decode vector over a malformed file.

### S2. D5 (slack and stale sizes) costs three whole files, and the draft does not show the baseline loads them (DESIGN-NOTES.md:49,65)
Installed gdk-pixbuf loads `hopper_mask.ico`, `mask_1.cur`, `mask_L.cur` and `pillow.ico` (its 48x48 DIB; the core gives the 16x16 only). With the core, `hopper_mask.ico`, `mask_1.cur` and `mask_L.cur` yield no decodable image at all (their only entry is a DIB error). Details in the corpus table. This is the largest compatibility cost in the draft, and it is a DIB-layer limit (frozen `IcoEntryDib` requires `cursor==|b|`): the container cannot repair it without a new DIB mode that reports consumed length. See D5 in the table. At minimum DESIGN-NOTES must say "baseline loads these four files" and "3 of 12 accepted files have no decodable image".

### S3. Whole-file all-or-nothing directory validation is an unlisted policy; add D9 and a baseline table (DESIGN-NOTES.md:37,45)
`parse` fails the whole file if any one record has nonzero reserved, size 0, offset in the directory, or a range past EOF (SPEC R4, R10 "An invalid earlier record wins over all later records"). DESIGN-NOTES mentions it only as a side effect inside "Strictness costs" and lists it in no D item. Evidence from the installed baseline (scratch files, entry 0 a valid 16x16 1-bpp DIB): second entry offset 100,000 beyond EOF loads; entry reserved byte 7 loads; size 0 loads; size 500 larger than EOF loads; offset pointing into the directory fails (dimension mismatch on the garbage header); table cut mid-record fails ("truncated"). So the core is stricter in four of the six, which DESIGN-NOTES:37 states in general terms but not with the consequence "a file the baseline loads is rejected because an unselected entry is bad". Recommend keep (D9) but record it, because a truncated download with an intact first image is the realistic case (RECALLED: some decoders return the entries parsed so far).

### S4. `decode_entry` with forged entries has unspecified behaviour (SPEC.md:5,19; THEOREMS.md `MD`/`OutsideDomain`)
R1 says forged entries "violate a documented API precondition rather than producing a format error". In safe Rust `Entry` has public fields, so `start>end` or `end>|b|` reaches `&bytes[start..end]` (a panic, or undefined behaviour if an implementer uses `get_unchecked`). This is the same precedent as the frozen ICNS spec (checked: ICNS SPEC.md:5), so it is consistent, not a defect to block on. A cheap total alternative: `decode_entry` re-checks `start<=end<=|b|` and returns `EntryOutOfBounds`, which removes `OutsideDomain` and the provenance premise from T5 for the price of one comparison; the cost is that `EntryOutOfBounds` stops being parse-only (R9). Owner decision D10; I recommend the total form if the owner is willing to touch R9, otherwise keep and state "panics" in R1 so the behaviour is at least named.

### S5. T9's combined bound cannot be proved from the DIB crate's public contract (THEOREMS-DRAFT.md T9; SPEC.md:29)
The public `dib_verus::decode` (`dib.rs:5368-5385`) ensures `decode_mp_eq`, the `mh` error equality and the output-size bound; it exports no step count. The linear-work witness `t12_linear_work` (`dib.rs:5388`) is a private `#[allow(dead_code)]` function. So `steps(DIB)<=16*(size+N)` can be cited but not used by an ICO proof, and the same holds for the DIB allocation count in T10. The frozen DIB cannot change. Rewrite T9: the ICO theorem covers `steps(parse)` and `steps(wrapper)` only, and the end-to-end sentence is "plus the DIB crate's R18 bound, imported by citation (private witness at dib.rs:5388), not re-proved". R13's combined clause is conditional ("if reporting"), so SPEC.md needs only the same one-line note.

### S6. Test harness depends on an unpinned absolute path (check_vectors.py:12; gen_vectors.py:10; run_corpus.py)
The frozen DIB directory holds no `check_vectors.py` although FREEZE.txt lists its hash; the ICO oracle loads `/home/ws/projects/dib/spec/check_vectors.py`. The hash happens to match today. Add `hashlib.sha256` verification against the FREEZE.txt value at load, so that a later edit of the DIB working copy cannot silently change ICO expectations.

### S7. The 1,024-entry vector has 1,024 identical records (VECTORS.json `count-exact-1024`, `-decode-last`; gen_vectors.py:141-145)
The repeat language can only repeat identical records, so a stride/index bug that appears past a few records (mutant: record `i&255` read for record `i`) survives, and so does any implementation that reads the directory in chunks and mishandles the chunk boundary. The file is 16,398 bytes; write it as a literal vector with per-record distinct width, height, colour count, hotspot/planes words, size and offset (all ranges inside one blob so `offset+size<=|b|`), and decode the last index to a different payload kind than the first.

### S8. Manifest mutation table misses the classes in B1/B2 and the index narrowing (THEOREMS-DRAFT.md, "Implementation-side falsification plan")
M1-M19 each map to a vector that fails under my corresponding mutant, and each named theorem would fail (M2 by an out-of-bounds precondition, M16 likewise, M17 through the `decode_mp_eq` slice argument). The honest AUDIT labels on M20 and M21 are correct. Add: M22 classify by reading `b[off..off+8]` without the `size>=8` guard (T4, not T1); M23 compare `offset` against the end of the current record (T2); M24 narrow `size`/`offset` to u16 or i32 (T2/T7); M25 narrow the decode `index` (u32 to u16/u8/usize-32): vectors currently use 1, 2, 65535, 4294967295, none of which narrows to a valid index, so add 256 and 65536 against a one-entry file (T5); M26 mix up record stride after a multiple of 256 (T2, with S7); M27 sort or select entries by nominal size: survives all vectors because every multi-entry vector has equal 1x1 nominal dimensions, so add two entries 16x16/32x32 and the reverse (T2 ordering). Also split M12 into "wrap in u32" and "skip `offset>|b|`" (the first is killed by `size-4294967295` or `range-22-4294967295`, the second by `range-4294967295-1`).

### S9. T11 witnesses: proportionality for DIB pixels (THEOREMS-DRAFT.md T11 and witness list)
The ICO-side witnesses (parse errors, ranges, classification, `PngDelegation`, `IndexOutOfRange`, DIB header errors) are small unfoldings of one recursive spec function and are proportionate. The two witnesses that need `MP` evaluated on a concrete DIB (`dib-basic-decode`, `dib-mismatch-decode`; the DIB crate does this in `model_vectors.rs`) are expensive and do not test any ICO rule beyond "slice = subrange", which the generic T5 lemma already gives. Drop them from the proof gate and keep them as runtime vectors.

### S10. "Append without reallocation" and "reserve once for exactly `count`" are unprovable claims in normative text (SPEC.md:7; T10)
R2 states them as requirements; T10 admits allocation is AUDIT-only, but R2's wording reads as a checked contract. As far as I recall, vstd does not expose `Vec` capacity in its specs (RECALLED), so even the ghost counter cannot assert the capacity. Mark R2's sentence "source-audit requirement" in the same words T10 uses, and keep M21 labelled audit-only.

## Nits

- N1. SPEC R6: add one sentence that no valid DIB begins with the PNG signature (its first four bytes read as header size `0x474e5089`, rejected by DIB R2), so PNG classification never hides a decodable DIB. True from the frozen text, currently left for the reader to derive.
- N2. R13/T9: `n` is redundant; when the table guard passes `n<=(|b|-6)/16`, so `16*(|b|+n+1)` is no looser than `16*(|b|+1)+16*n`. Fine as written; the `n` term only costs a proof step.
- N3. SOURCES.md row "Maximum file size, count...": "1,024 is chosen rather than 4,096 because the ICO count is a u16" does not follow (u16 permits 65,535). State it as a pure policy (DESIGN-NOTES D7 does).
- N4. DESIGN-NOTES D1: also say the baseline rejects count 0 (`io-ico.c:333-389`: the loop is skipped and it reports "Compressed icons are not supported", a misleading message). `no_cursors.cur` is therefore not a regression against the baseline; the installed gdk-pixbuf fails on it too.
- N5. SPEC scope: say that a resource-file group directory (RT_GROUP_ICON, 14-byte entries with an id instead of an offset; RECALLED) is out of scope, and name the frozen DIB path/version the crate depends on.
- N6. Overlap: `two-partial-overlap` is a nested range (39..47 inside 38..86). Add a straddling pair (second starts inside the first and ends past it) so a "reject only straddling" mutant dies.

## Fidelity of SPEC.md to the format and to `io-ico.c` (task item 1)

Format facts below are RECALLED (Microsoft icon/cursor resource pages) unless a line is cited.

| Topic | Spec | Verdict |
| --- | --- | --- |
| ICONDIR: reserved 0, type 1/2, count u16 | R3 | matches `io-ico.c:234-256`. Baseline checks reserved, then type, then count: same order. |
| ICONDIRENTRY 16 bytes: w, h, colours, reserved, planes/hotX, bitcount/hotY, size, offset | R4 | matches `:292-298`. Baseline ignores byte 3 and `data_size` (`:289`, G_GNUC_UNUSED). |
| 0 means 256 | R4,R5 | matches `:322-323` and the writer `:1205-1211`. Stored as u16 so 256 fits. |
| CUR reuses planes/bitcount words as hotspot | R5 | matches `:295-296,535-540`. Baseline keeps raw values; its writer rejects out-of-range at save (`:1074-1084`) only. |
| PNG-compressed entries | R6 | baseline has no PNG support and skips any entry whose BIH bytes 16-19 are nonzero (`:366-371`), which includes PNG, BITFIELDS and RLE. Signature-only classification is a DECISION (D6), reasonable: all 8 bytes, no IHDR. |
| Size and offset semantics | R4,R7 | The core treats `[offset,offset+size)` as authoritative. The baseline reads from `offset` and consumes pixel rows sequentially, ignoring size, so it never sees "stale size" at all. Documented in DESIGN-NOTES:37. |
| Real producers: stale/too-small size (`mask_*.cur`: 8 vs 80/1,096 true), too-large size with zero slack (Pillow DIBs: 48, 128, 192 zero bytes; `hopper_mask.ico`: 64 extra bytes), offset inside the directory (only in my built file), overlapping/shared resources (RECALLED: icon tools that share one image across entries), size larger than the DIB needs (TrailingBytes) | R4,R7,D5,D8 | All are handled by explicit rules; the first two are the cost in S2. |
| Planes/bitcount retained, never validated against the DIB | R5 | agrees with the baseline, which never reads them for ICO. |

Deliberate differences and documentation: sort/selection (documented :36), PNG (:36), reserved/size/offset strictness (:37), progressive vs up-front table (:37), directory/DIB dimension equality (:38; baseline `:421-428`), zero DIB width (:38), hotspot clipping (:38), colour-count depth score (:39), compression/BITFIELDS skip (:40), monochrome palette and 32-bpp AND skip (:40), streaming/saver (:41). All line citations I checked are correct. Not documented: the baseline's `data_offset>INT_MAX-40` skip (`:316-319`), which makes it ignore such an entry silently where the core errors (covered by the general "out-of-bounds" bullet but not named), and the all-or-nothing consequence (S3). No other undocumented difference found.

## Error names and precedence (task item 2)

I could not construct a pair of independent readings that disagree on the first error of `parse`: R10 lists the order once, R3 and R4 repeat it consistently, and the three potentially ambiguous points are pinned in the text (count cap before table length; incomplete table before any record; subtraction form for the range test). The three vectors `short-table-two-entries` (all-zero first record would be `EmptyEntry`), `count-before-short-table` and `two-errors-0-1`/`first-error-before-later-reserved` pin them. Residual ambiguity: the harness-level `decode` semantics (S1) and, for decode, which of `IndexOutOfRange` and a DIB error wins for an out-of-range index (R8: index first; pinned by `index-1`). `Dib(e)` ordering is delegated to DIB R14 and the delegation is exact.

## Corpus (task item 5)

I ran `tools/run_corpus.py`: 12 accepted, 1 rejected (`EmptyDirectory`); 18 entries (2 PNG, 16 DIB); 9 DIB successes and 7 DIB errors; every DIB outcome agrees with the frozen oracle on the exact slice (the runner asserts it). Matches DESIGN-NOTES. Baseline column is the installed gdk-pixbuf 2.42.12.

| File / entry | Core outcome | Baseline | Verdict |
| --- | --- | --- | --- |
| `no_cursors.cur` | `EmptyDirectory` | fails | justified (D1); the file has nothing to decode |
| `mask_1.cur`, `mask_L.cur` | `Dib(TruncatedHeader)`: directory size 8, true DIB 80 / 1,096 bytes = EOF-22 | load | **over-strict in outcome**, correct by the stated size contract; a stale directory size. Needs owner decision D5 |
| `hopper_mask.ico` | `Dib(TrailingBytes)`: 240 declared, 176 needed, 64 slack (48 zero) | loads | over-strict in outcome; DIB-layer rule. D5 |
| `pillow.ico` entries 1-3 (48x48, 32x32, 24x24) | `Dib(TrailingBytes)`: slack 192, 128, 48 zero bytes (AND-mask rows of width/4 bytes instead of 4*ceil(w/32); entry 4 at 16x16 has 0 slack and decodes) | loads (48x48) | over-strict in outcome; D5 |
| `decompression_bomb.ico` | `Dib(InvalidDimensions)`: BIH 65,535x65,535 in a 16x16 entry | fails (dimension mismatch) | justified (DIB cap 8,192; baseline rejects too) |
| `hopper_unexpected.ico` | decodes 16x16 (directory says 16x32) | fails (size mismatch) | a gain from D3; the baseline rejects |
| `hopper_draw.ico`, `pillow.ico` entry 0 | PNG range returned untouched | fail / skip | by design |
| other 8 DIBs | decode | load | agree |

Corpus-bias caveat: these are the Pillow test fixtures, which include deliberately malformed files (SOURCES says local testing only); the 6 failures are not a population rate. Still, 3 of the 12 accepted files return no image.

## Owner decisions (task item 5)

| ID | Decision | Reasonable? | What other software does | Recommendation | Owner needed? |
| --- | --- | --- | --- | --- | --- |
| D1 | `EmptyDirectory`, not `Ok([])` | yes | baseline also rejects count 0 (N4) | keep | no |
| D2 | allow aliases and overlaps | yes; bounded slices make aliasing safe | baseline has no per-entry slices | keep. State the cost: a 16 KiB file with 1,024 entries aliasing one 64 MiB DIB makes a "decode every entry" caller do 1,024 full decodes (up to 1,024 x 64 MiB of output over time, not at once). T9's closing paragraph says the bound excludes it; say so in DESIGN-NOTES too | no |
| D3 | directory dimensions nominal; decoded dimensions govern | yes; the file `hopper_unexpected.ico` is a real case the baseline rejects | baseline rejects mismatch; RECALLED: browsers decode from the DIB header | keep | no |
| D4 | hotspot raw u16, never validated | yes, matches baseline loader (`:391-394`) | baseline loader stores raw; writer validates | keep; the renderer must validate against decoded dimensions | no |
| D5 | exact `size` range, DIB `TrailingBytes` for slack | defensible for a verified core, costs 3 files and one candidate | baseline ignores size and loads all four; RECALLED: browsers and Pillow read from the offset and ignore the size | owner decision. Options: (a) keep and document; (b) ask for a new DIB mode (re-freeze) that decodes a prefix and reports the consumed length, so the container could then offer an explicit lenient helper (`decode_entry_lenient` through EOF for stale sizes, trim for slack); (c) ICO-side only: not possible, because only the DIB knows where a DIB ends. I recommend (a) for this freeze, with S2's documentation, and (b) as a recorded follow-up | **yes** |
| D6 | PNG on the 8-byte signature alone | yes | baseline has no PNG; RECALLED: Windows/browsers sniff the signature | keep (N1) | no |
| D7 | 64 MiB, 1,024 entries | yes (a policy, not a format fact) | RECALLED: real icon files rarely exceed a few dozen entries | keep | no |
| D8 | arbitrary gaps and suffixes between ranges, `TrailingBytes` inside a range | yes | baseline ignores both | keep | no |
| D9 (new) | all-or-nothing directory validation (S3) | defensible; simple and total | baseline loads files with an unselected bad entry | keep, add to DESIGN-NOTES | **yes** (it locks the API shape: per-entry `Result` would change `Entry`) |
| D10 (new) | forged `entries` are a precondition vs a total `decode_entry` (S4) | consistent with ICNS | n/a | total form if R9 may change; else document "panics" | **yes** |

## THEOREMS-DRAFT.md (task item 6)

- **T2/T3/T4/T6/T7/T8**: exact and statable against a total `MC`. T2 is one loop over records with the invariant "Vec == `mc_rec(b,i,acc)`", early returns matching the first-error branch of `mc_rec`; no obstacle beyond ordinary Verus work. T3's equivalence is definitional once T2 holds. T4 needs the length guard before the 8 comparisons (B1 tests it). T7's `dir_end<=16,390` and the list bound are immediate.
- **T1 on 32-bit and 64-bit `usize`**: sound; the largest value in play is 64 MiB.
- **T5 composition across crates**: statable and provable. The probe (re-run: 1 verified) shows that `decode_mp_eq(r, bytes@.subrange(start,end), IcoEntryDib)` is derivable from a checked slice, and `dib_verus::model::mp` is `pub open spec`, so `MD` can name it without a second DIB model. Caveats: the ICO `Error::Dib(dib_verus::Error)` mapping needs a one-line `match` lemma; the probe depended on a path copy, so the real crate must depend on `verified-decoders/dib/dib_verus` with the same pinned `vstd`; and S4 decides whether T5 carries a provenance premise.
- **T9**: parse and wrapper bounds are provable as stated (one loop, constant eight comparisons; the wrapper has no loop, so its bound is vacuous but harmless). The combined clause is not provable across crates (S5).
- **T10**: honest about audit status; see S10.
- **T11 and L1-L4**: L1, L4 are one-line arithmetic; L2 needs a symbolic-length sequence and `subrange` lemmas (moderate); L3 is an induction over identical records (moderate, riskier than the others for a DeepSeek-class implementer). Neither L2 nor L3 needs a 64 MiB or 1,024-record term. Witness list names all exist in VECTORS.json (checked mechanically; `count-i` in the text is an expression, not a name).
- **Mutations**: M1-M19 are each real kills of a named theorem except where marked audit (M20 conditional, M21 audit). Missing classes: S8.
- **Proportionality**: much lighter than DIB (one loop, no pixel arithmetic); feasible for a DeepSeek-class implementer in a few milestones, the main risks being the Entry/`Result` spec mapping, the loop invariant with `Vec::push`, and L3. Not feasible as stated: the T9 combined bound (S5) and any capacity claim (S10).

## Ready to freeze?

Freeze after: B1, B2 (new vectors; regenerate and re-check with both oracles), S1, S2, S3, S5, S6, S7, S10 text changes, and decisions on D5, D9, D10. S8, S9 and nits can land with the manifest. I would re-run my mutation set after the new vectors are added; I expect every survivor except the 3 equivalent ones to die.
