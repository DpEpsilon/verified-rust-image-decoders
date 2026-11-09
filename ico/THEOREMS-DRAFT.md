# ICO/CUR theorem manifest (frozen)

Targets `/home/ws/projects/ico/spec/SPEC.md` revised ICO-CORE-SPEC 1.0 and proposed `/home/ws/projects/dib/spec-1.1/SPEC.md`, not the shipped DIB 1.0 crate. No ICO or DIB 1.1 Verus implementation exists. The earlier disposable two-crate probe proved the shipped DIB 1.0 `decode_mp_eq` postcondition on a checked slice, not the new mode or the new total API. Independent mathematical models may import DIB 1.1's *public proven* `MP_1.1` as a dependency; they must not import the ICO Python oracle or executable parser. No `unsafe`, `assume`, `admit`, `external_body`, false preconditions, trusted I/O or unproved source equivalences. Safety conditions exclude allocator exhaustion and operating-system stack failure, not malformed bytes.

## Total mathematical models

`H(b)` applies the R2/R3 file guard on **every** finite representable `b` with conditional guarded LE reads. It returns a directory error or `(type,count,dir_end)`. `ME(b,H,i)` is a total selected-record model for `i<count` after successful `H`: read the complete record, derive every metadata field, and choose the first per-record defect in R4 order or a validated range with R6 kind. It returns an internal `OutsideGuard` only for invalid model arguments; this marker is **never** a public error. Its valid range satisfies `dir_end<=start<end<=|b|`. `MC(b)` returns `Err` only if `H` errors, otherwise `Ok([ME(b,H,i) for i=0..count-1])` including **every invalid status**. No invalid record causes `MC` to fail.

`MD(b,E,i)` is total for **every** typed `E:Seq<Entry>` and `i:u32`: apply H; then `i>=count`; then `|E|!=count`; reconstruct `ME(b,H,i)` and compare the entire selected record with `E[i]`; then selected invalid status; PNG kind; otherwise call the **mathematical DIB 1.1** `MP_1.1(b.subrange(start,end),IcoEntryDibSlack)` and map the exact result to the public DIB `Decoded` or typed `Dib(e)`. `MD` has no provenance precondition and no `OutsideDomain` result. Since `ME` checks the range, no forged `E` can authorize an unchecked slice. The model rechecks selected *metadata/status*, not equality of all payload bytes with a previous call. The real decoder must use the current bytes. No model calls the executable parser.

## Quantified obligations

| ID | Obligation on **all** representable slices and typed arguments |
| --- | --- |
| T1 safety | `∀b: Safe(parse(b))`; `∀b,E,i:u32: Safe(decode_entry(b,E,i))`. Directory guards precede reads; selected-record comparison and range guards precede slice indexing on both 32- and 64-bit `usize` domains. In particular a forged range, kind, list length or index cannot panic. |
| T2 exact parse | `∀b: parse(b)=MC(b)` as the **complete Result**: directory-wide first error or every record in original order, with exact raw fields and valid/invalid status. A bad first record followed by a good last record returns both, without aborting. |
| T3 record validity | If `parse(b)=Ok(E)`, then `|E|=count∈[1,1024]` and all `E[j]` equal their wire-derived `ME` fields; **only valid** entries additionally satisfy `dir_end<=start<end<=|b|<=67,108,864`. Invalid entries carry raw offset/size but no validated payload. Derive from T2, not from an assumed provenance axiom. |
| T4 signature classification | For every valid `E[j]`, `kind=Png ⇔ raw_size>=8 ∧ b[start:start+8]=SIG`; no byte beyond `end` may participate, even when trailing file bytes complete a seven-byte prefix. Near-miss bytes at indices 0/3/4/7 and an embedded shifted signature remain Dib. |
| T5 exact total decode | `∀b,E,i:u32: decode_entry(b,E,i)=MD(b,E,i)` including header errors, `IndexOutOfRange`, forged-list/record `InvalidEntryRecord`, selected entry defect, `PngDelegation` and **all DIB bytes/errors**. For a selected valid matching DIB `[a,z)`, derive the checked slice and invoke the **same** exported `dib_verus::decode_with_steps` DIB 1.1 call whose result satisfies `dib::decode_mp_eq(r,b@.subrange(a,z),IcoEntryDibSlack)` and whose ghost steps satisfy DIB T12; map each success field and every RGBA byte or exact error into `MD`. No antecedent `parse(b)=Ok(E)` is allowed in the public theorem. |
| T6 exact precedence | Directory R2/R3 priority on both public calls; decode index before list mismatch; list/selected record mismatch before selected defect; selected defect before PNG/DIB; PNG before any DIB error; selected DIB R14 priority delegated unchanged. An unselected invalid wire record or forged unselected supplied record never changes a selected result; decode checks only `entries.len()` and `entries[index]`. For parse, record defects are statuses rather than parse errors. |
| T7 arithmetic and output bounds | Prove `dir_end<=16,390`, count<=1,024, valid range inside file and u32, no wrapping `offset+size` nor narrowing `u32 index` to an inadequate type. DIB output obeys the dependency's `4*w*h<=67,108,864` postcondition. Wrapper never allocates a pixel vector. |
| T8 termination | Parse variant `count-i` decreases for each complete record whether valid or invalid; bounded eight-byte signature comparison; decode revalidates only one record then calls terminating DIB 1.1 once. An input with count=65,535 returns before iterating. |
| T9 ICO-side linear work | Attach a ghost step to **every ICO-controlled real loop iteration**, including list initialization and library bulk fills: with `steps(parse)<=16*(|b|+1)` and `steps(wrapper)<=16*(|b|+1)` for every `b,E,i`. The wrapper's constant selected-record work counts at most eight signature-byte checks; it does not scan E or parse all records. No in-crate theorem currently claims an end-to-end numeric DIB+ICO bound: the shipped DIB `decode` exports no step count. The DIB 1.1 proposal exports its existing tracked witness body as public `decode_with_steps` with `steps_DIB<=16*(|slice|+N)` and `decode_mp_eq`; ICO calls **that same function**. Only after DIB proves this new public contract can ICO derive a combined bound from both counters. Until then cite the separately proved DIB 1.0 T12 as external evidence, not a discharged ICO obligation. Audit uncharged new loops. |
| T10 allocation-call bound and audit | Header-error paths request no entry-list allocation; after guards, request at most one bounded allocation for at most 1,024 records; decode wrapper makes no allocation and invokes DIB at most once. A ghost call counter charged at *documented* sites can express that shape, but a Verus-verified ghost count cannot detect a new uncharged `vec!`, clone, `collect` or formatting call. Source audit is mandatory; no physical heap peak or `Vec` capacity theorem is claimed. DIB's own allocation audit remains separate. |
| T11 witness bridge | Establish `MC` versus a small subset of independently authored expected records and prove one public `decode_entry` call on a file containing a valid selected record beside an invalid unselected record, plus one forged selected record producing `InvalidEntryRecord`. These are ICO-only model obligations; **do not** require concrete `MP_1.1` evaluation for `dib-basic-decode`/`dib-mismatch-decode` in Verus: the generic T5 slice-model lemma supplies the exact pixel bridge and both remain runtime vectors. |

## Non-vacuity and symbolic boundaries

All **217** vectors run through the independent ICO Python oracle; retained draft-one fixtures include the restored full-word type/reserved discrimination cases, while deliberately redundant metadata permutations remain trimmed. The model proof gate consists of small ICO-only witnesses: `short-dir-0`, `reserved-1`, `type-0`, `empty-directory-1`, `count-cap-1025`, `short-table-15`, `dimensions-0-0-type1`, `words-65535-0-type2`, `entry-reserved-1`, `size-0`, `directory-overlap-21`, `range-4294967295-1`, `png-prefix-followed-by-rest-7`, `png-bit-0-parse`, `png-bit-7-parse`, `png-shifted`, `table-boundary-record0-off22`, `table-boundary-record0-off38`, `wide-size-10001`, `wide-offset-10016`, `straddling-ranges`, `directory-order-large-first`, `valid-then-bad-oob`, `bad-then-valid-reserved`, `forged-wrong-kind`, `wire-invalid-forged-valid`, `wire-invalid-forged-defect`, `forged-unselected-ignored`, `type-257`, `reserved-256`, `decode-index-priority`. No proof-gate witness needs a concrete 24-bit DIB pixel evaluation: `forged-unselected-ignored` can use a selected PNG signature, while its runtime DIB decode remains a fixture.

- L1. Every `|b|>67,108,864` produces `FileTooLarge` on parse **and decode_entry for every E/i**. Instantiate `file-size-67108865` without expanding its oversized input as a verifier `Seq`.
- L2. Exactly 67,108,864 bytes comprising a 30-byte one-PNG-entry prefix and zero suffix pass the cap and yield one validated PNG range `[22,30)`; `file-size-67108864` is a symbolic length lemma, not a 64 MiB verifier expansion.
- L3. For any `k<=1024` complete records, induct on parsed index `j` that the first `j` output records exactly equal the mathematical `ME` results, **including invalid statuses**. At `j=k` get `MC=Ok(E)` irrespective of defects. The runtime `count-exact-1024` fixture has **distinct** metadata for all 1,024 records and a different final payload kind; do not build a 1,024-record verifier term. `count-cap-1025` rejects before table availability.
- L4. For every valid record, `offset>=dir_end`, `size>0`, `offset<=|b|` and `size<=|b|-offset`, hence end fits u32/usize. For invalid records no such range premise is asserted. Use `wide-size-10000`, `wide-offset-10016`, `wide-sum-wrap` to distinguish 16-bit narrowing and u32 wrap.

## Planned executable-only mutations — not run

Keep `MC`, `MD`, DIB 1.1 model and expected vectors fixed; a valid kill is a **failed verifier obligation**, not a compile error or runtime mismatch alone. Mutating the model with the code can preserve equality; test the model independently. For an implementation in which a mutation is observationally equivalent, construct a new witness and state the actual failing obligation.

| # | Mutation | Discriminating witness | Expected obligation |
| --- | --- | --- | --- |
| M1 | Input cap `>` to `>=`. | `file-size-67108864` | T2/T5 |
| M2 | Read reserved before length-six guard. | `short-dir-0` | T1/T2 |
| M3 | Swap LE type bytes. | `dib-basic-parse` | T2 |
| M4 | Accept count zero. | `empty-directory-1` | T2 |
| M5 | Check table before count cap. | `count-cap-1025` | T2/T6 |
| M6 | Step record cursor by 15 rather than 16. | `two-dib-png`, `count-exact-1024` | T2 |
| M7 | Keep zero nominal width/height as zero. | `dimensions-0-0-type1` | T2/T3 |
| M8 | Interpret CUR words as ICO planes/depth. | `words-65535-0-type2` | T2 |
| M9 | Discard an invalid unselected record or abort parse on it. | `valid-then-bad-reserved`, `bad-then-valid-oob` | T2/T5 |
| M10 | Check size zero after range bounds, or turn it into a valid empty slice. | `zero-size-before-overlap` | T2/T6 |
| M11 | Compare offset with six instead of full directory end. | `directory-overlap-21` | T2/T3 |
| M12 | Add offset and size with wrapping u32 instead of subtraction/wide arithmetic. | `wide-sum-wrap`, `size-4294967295` | T1/T2/T7 |
| M13 | Classify a seven-byte prefix as PNG. | `signature-prefix-7` | T4 |
| M14 | Require IHDR after the eight-byte signature. | `signature-prefix-8` | T4/T5 |
| M15 | Reject identical, nested or straddling bounded ranges. | `two-identical-range`, `straddling-ranges` | T2 |
| M16 | Index E before index/count guard or narrow u32 index to u8/u16. | `index-narrow-256`, `index-narrow-65536` | T1/T5/T7 |
| M17 | Pass offset-to-EOF to DIB rather than the declared slice. | `entry-size-truncates-dib` | T5 |
| M18 | Enforce directory/DIB dimension equality. | `dib-mismatch-decode` | T5 |
| M19 | Decode a PNG signature as DIB. | `two-dib-png-decode1` | T5/T6 |
| M20 | Rescan all previous records per record. | `count-exact-1024` | T9 **if charged**, otherwise AUDIT |
| M21 | Allocate a second scratch vector without ghost charge. | `dib-basic-parse` | T10 **AUDIT only** |
| M22 | Read eight signature bytes beyond the declared seven-byte payload. | `png-prefix-followed-by-rest-7` | T4 (may be within file: not necessarily T1) |
| M23 | Compare offset against only the current record end. | `table-boundary-record0-off22` | T2/T3 |
| M24 | Read size/offset as u16, u24 or signed i32. | `wide-size-10001`, `wide-offset-10016`, `wide-size-80000000` | T2/T7 |
| M25 | Read index as u16/u8 or compare after narrowing. | `index-narrow-256`, `index-narrow-65536` | T5/T7 |
| M26 | Read record `i&255` or reset stride after record 256. | `count-exact-1024`, `count-exact-1024-decode-last` | T2/T5 |
| M27 | Sort entries by nominal size or select the largest. | `directory-order-large-first`, `directory-order-small-first` | T2 |
| M28 | Trust forged kind/range or skip length/status comparison. | `forged-wrong-kind`, `forged-wrong-end`, `forged-empty` | T1/T5 |
| M29 | Return selected entry defect before selected-record mismatch. | `wire-invalid-forged-valid`, `wire-invalid-forged-defect`, `wire-invalid-correct-record` | T5/T6 |
| M30 | Use exact-slice `IcoEntryDib` instead of 1.1 slack mode. | `dib-slack-64-decode` | T5 |
| M31 | Compare only width, kind, range or another subset of selected fields. | `forged-wrong-height`, `forged-wrong-color-count`, `forged-wrong-planes`, `forged-wrong-bit-count`, `forged-hotspot-x`, `forged-hotspot-y`, `forged-wrong-raw-size` | T5/T6 |
| M32 | Validate every supplied record instead of only the selected one. | `forged-unselected-ignored` | T5/T6; T9 if the scan is charged, otherwise source audit |
| M33 | Compare only the low byte of the header type or reserved word, or accept type 3. | `type-257`, `type-513`, `type-3`, `type-256`, `reserved-255`, `reserved-256` | T2/T5/T6 |

No falsification run or verifier diagnostic is claimed. An audit-only kill cannot be reported as a verifier kill. The TGA/DIB reviews found unproved helper-state clauses and uncharged allocations; the public T5 proof must bind the actual DIB call's exact returned bytes/error to the same slice, not posit a model-only equality. The 1.1 work-contract export is a **separate DIB proof task**: the ICO crate cannot manufacture it by naming a private witness. Freeze and reprove DIB 1.1 first, retake the draft oracle pin in `spec/dib_oracle_pin.py`, regenerate and check ICO vectors, then obtain independent review before freezing ICO 1.0. The frozen DIB 1.0 oracle hash is not the new dependency identity.
