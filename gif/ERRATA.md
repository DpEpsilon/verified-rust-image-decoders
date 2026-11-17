# Errata against the frozen GIF document set

Append-only. This file is **not** covered by the GIF freeze: it records
discrepancies between the frozen texts (`THEOREMS.md`, `spec-frozen/`) and the
implementation. Nothing here edits a frozen file. New entries are appended at the
end with their date and the artifact they concern. An entry marked PROPOSED has
not been applied: it takes effect only after the owner approves it, and the entry
is then amended in place of a new one by appending an "Owner decision" line.

## Index and status (as of 2026-11-17)

| Erratum | Status |
|---|---|
| E1 | approved; condition discharged (see Evidence (2026-11-17) under E1 below; run 644) |
| E2 | approved (wording amended; gates in `../RELEASE-GATES.md` item 7); obligations (1)-(2) still open, (3) proved on E4's domain (see E2 Evidence) |
| E3 | approved; conditions (a) and (b) discharged (runs 644-646); finding 6 stays open (runs 638, 639) |
| E4 | approved; (b), (c), (d) discharged; (a) discharged for q < A, wording corrected for the full-raster case (see E4 Evidence) |
| E5 | informational |

Owner approval of E1-E4 was given on 2026-11-17 as amended by the council. The "PROPOSED" in the headings below is the original status and is kept (append-only); the Status block at the end of each entry supersedes it.

Source of E1 and E2: `REVIEW-ASTRA-INTEGRATED.md` (GPT-6 Astra, proof checkpoint
606, findings 4 and 5). Line numbers below were checked against the current
files on 2026-11-16.

## E1 — 2026-11-16: `Saved` storage stride (T8) — PROPOSED, awaiting owner approval (not applied)

**Frozen text.** `THEOREMS.md:59` (T8): "`Saved` holds the rectangle's pixels after
that disposal and before frame `k`'s first write, row stride `4·w`", with `w` the
frame rectangle width.

**Implementation.** `Saved` is stored with the full-canvas stride (`4·sw` bytes
per row), as a canvas-shaped buffer. Save is `copy_rect(saved, &self.canvas, sw, sh, rect)`
(`gif_verus/src/gif.rs:1716`), restore is `copy_rect(&mut self.canvas, saved, sw, sh, prev.rect)`
(`gif.rs:1672`); `copy_rect` requires both buffers to be `4·sw·sh` bytes
(`compose_new.rs:954–965`, body from `:1055`). The independent frame model does the
same: `saved_image` copies into a canvas-length buffer (`frame_model.rs:115–119`).

**Counterexample.** 3×2 screen, disposal-3 frame with rectangle `(left=1, top=0, width=1, height=2)`.
The two saved pixels sit at byte offsets 4 and 16 (rows are `4·3 = 12` bytes
apart). T8 as written requires offsets 0 and 4 (row stride `4·1`).

**Effect.** None on observable behaviour: save and restore use the same layout,
so the restored canvas equals the frozen G12.3 result. Only the stated storage
layout differs.

**Proposed erratum.** Replace the clause "row stride `4·w`" by: "`Saved` holds the
`4·N`-byte canvas-shaped copy of the rectangle region (same layout as the canvas;
bytes outside the rectangle are unspecified-but-unread)". `N` is the canvas pixel
count. A-GIF-3's "Saved at most once" and the `4·N ≤ MAX_OUTPUT` reservation
bound are unchanged.

**Alternative.** Keep T8 verbatim and change the implementation and the frame
model to a rectangle-shaped `Saved` (`4·w·h` bytes, stride `4·w`). This changes
the verified `copy_rect` contract, the reservation size and the allocation-delta
bound for `Saved`, and requires re-verification.

### E1 Status (2026-11-17)

**APPROVED WITH CONDITIONS; conditions open until evidence is recorded.** The owner approved E1 as amended by the council on 2026-11-17 (council report `~/projects/council/sealed/COUNCIL-REPORT-E1-E4.md`, sha256 `50dbd1ddbf4baf9c31b27d8089acef21fba5ea61bc97d75738950933e3f0effa`; four members, round 3, all "approve with amendment" on E1, no dissent). E1 takes effect only when the condition below is met. Until then the frozen T8 text stands.

Condition: this erratum must cite the `copy_rect` postcondition or lemma that proves "restore reads `Saved` only at byte offsets inside the rectangle that the matching save wrote". If no such postcondition or lemma exists, it must be added and verified first.

Evidence: see the E1 Evidence (2026-11-17) block below.

### E1 Approved text (2026-11-17)

"`Saved` holds a `4·N`-byte canvas-shaped copy of the rectangle region, with the same layout as the canvas. Bytes outside the rectangle are unspecified. Restore reads `Saved` only at byte offsets inside the rectangle that the matching save wrote. The saved pixels' values and save timing, A-GIF-3's 'Saved at most once', and the `4·N ≤ MAX_OUTPUT` reservation bound are unchanged." This replaces the proposed clause "unspecified-but-unread". The citation of the proving `copy_rect` postcondition or lemma is to be added at the Evidence line when it exists.

## E2 — 2026-11-16: scope of the integrated T-LZW theorem — PROPOSED, awaiting owner approval (not applied)

**Frozen text.** `THEOREMS.md:57` (T-LZW · M2): "For all `m ∈ [2, 8]`, all `D`, all
`A ≥ 1` (and under `RB` all `m ∈ [2, 11]` ...)" — a universal statement.

**Implementation.** The integrated executable `lzw::decode` has a narrower entry
contract (`gif_verus/src/lzw.rs:2355–2366`): `u32` data length and area,
`m_ok(lzw_ps(m, opts), m, area)` (the G9-admissible `m`), `area <= MAX_TOTAL_PIXELS`,
and a fresh reader satisfying `reader_ready` (`lzw.rs:2205–2216`, input length
`<= MAX_INPUT`) that has been structurally scanned (`sub_scan ... is Some`).
Example excluded: `m = 2`, empty payload, `A = MAX_TOTAL_PIXELS + 1`.

**Effect.** None on the public path: GIF calls establish these preconditions
through G8–G9, so the integrated theorem covers every reachable call. It is a
reachable-domain theorem, not the unrestricted T-LZW.

**Proposed erratum.** State the scope: the integrated T-LZW (`gif_verus`) holds on
the reachable domain (G9-admissible `m`, `A <= MAX_TOTAL_PIXELS`, fresh
structurally scanned reader, input at most `MAX_INPUT`); the unrestricted
universal statement is established by the standalone `lzw_proof/`. Reports must
not cite the integrated theorem as the unrestricted T-LZW.

**Open.** The bridge from the standalone `lzw_proof` statement to the integrated
entry contract (that the standalone theorem's model and the integrated model
agree on the reachable domain, and that the integrated precondition is exactly
what G8–G9 establish) is not proved. It is listed as open in `../RELEASE-GATES.md`.

### E2 Status (2026-11-17)

**APPROVED** as amended by the council on 2026-11-17 (same report and hash as E1; four members, round 3, all "approve with amendment", no dissent). The amendment is a wording change with no pre-condition, so it takes effect now. The first version of E2 overstated the standalone proof and the parser connection (`lzw_proof/` was credited with the unrestricted T-LZW, and the Effect paragraph claimed coverage of every reachable call while listing G8–G9 exactness as open); the approved text below replaces those claims (see `REVIEW-ASTRA-INTEGRATED.md`).

### E2 Approved text (2026-11-17)

(a) Replaces the Effect paragraph: Verus checks the integrated entry contract at every call site in `gif_verus`, so every verified call lies in the reachable domain; that the contract is exactly what G8–G9 establish remains open.

(b) Replaces "the unrestricted universal statement is established by the standalone `lzw_proof/`": `lzw_proof/` establishes the universal statement relative to its own prefix/suffix model, subject to review finding 1 and E4; it does not discharge T-LZW for the integrated executable.

(c) Added: no report or release may claim the frozen T-LZW for the shipped decoder until three obligations are proved: (1) agreement of the standalone and integrated models on the reachable domain, (2) G8–G9 coverage of the integrated precondition, (3) the independent string-table representation proof. Note on (3): it is proved on E4's restricted domain in `gif_verus/src/string_model.rs` (run 625, 718 verified), and therefore depends on E4.

The gate list is item 7 of `../RELEASE-GATES.md`.

## E3 (PROPOSED, awaiting owner approval; not applied): the permitted range of `MAX_INPUT`

**Frozen text.** G1's derived bounds and `THEOREMS.md:9`: cap values may change by editing `limits.rs` alone, with every theorem statement and proof unchanged, provided the derived bounds stated in `SPEC.md` G1 hold. G1 states that `MAX_INPUT` plus one 65,536-byte block must fit a 32-bit `usize`, which permits `MAX_INPUT` up to about 4 GiB.

**Implementation.** `BitReader::new` (`gif_verus/src/lzw.rs:663`) computes the payload bit count `data_len * 8` in `u32`. With `MAX_INPUT = 2^29` the helper call `BitReader::new(&[], 0, 536870912)` satisfies its precondition and computes 4,294,967,296, one above `u32::MAX`. This is the smallest helper-domain counterexample; it is not a reachable failure of the public decoder at the current cap (`MAX_INPUT = 2^28`). Evidence: runs 609 (`MAX_INPUT = 2^27`: all 697 verify unchanged), 610 and 611 (`MAX_INPUT = u32::MAX` and `2^29`: the bit-reader multiplication fails), recorded in `RUNLOG.tsv`.

**Proposed erratum.** Add to G1's derived bounds `8 * MAX_INPUT <= u32::MAX`, that is `MAX_INPUT <= 536,870,911`, and to the `caps_ok` lemma the same condition. The alternative is to widen bit counts, positions, timestamps and reports to `u64` throughout the proofs while preserving the integer observations of ML; that restores the frozen "any G1-permitted cap" reading at the cost of a large proof change for an input cap above 512 MiB that no decoder in the corpus survey needs.

**Effect.** None at the current cap. Any later raise of `MAX_INPUT` above 512 MiB needs the `u64` widening first.

### E3 Status (2026-11-17)

**APPROVED WITH CONDITIONS; conditions open until evidence is recorded.** The owner approved E3 as amended by the council on 2026-11-17 (same report and hash as E1; four members, round 3, all "approve with amendment", no dissent). E3 takes effect only when condition (a) is met.

### E3 Approved text (2026-11-17)

Conditions:

- (a) Before the erratum takes effect, record in `RUNLOG.tsv` a verification run at `MAX_INPUT = 536,870,911` in which all proofs verify with no proof edits. Evidence: see the E3 Evidence (2026-11-17) block below (runs 644-646).
- (b) Retain every existing G1 bound, and add `8 * MAX_INPUT <= u32::MAX` (equivalently `MAX_INPUT <= 536,870,911`) to G1's derived bounds and to `caps_ok`. Exactly 512 MiB (536,870,912) exceeds this bound and requires wider arithmetic.
- (c) E3 does not close review finding 6. The claim "unchanged proofs for every cap within the amended G1 bounds" stays unestablished until the duplicated cap numerals (such as `MAX_INPUT == 268435456` in `model.rs`) are removed and the proofs depend on the cap only through `caps_ok`. Approval does not waive the symbolic-cap requirement or the lint against copied cap constants.

## E4 (PROPOSED, awaiting owner approval; not applied): domain of the T-LZW representation lemma's literal arm

**Frozen text.** `THEOREMS.md:71` (T-LZW representation lemma): the domain contains every `c < clear` (plus `clear + 2 <= c < next`) and requires `model_table[c][0] = code_first(t, m, c)`, with `model_table` a dictionary of byte strings.

**Contradiction (found by Sol, 2026-11-16).** Under the checked-literal policy (`m = 9, 10, 11`) the literal codes below `clear` include `c` in `[256, clear)`. For `m = 9`, `clear = 512`, `c = 256`, the executable's literal rule gives `code_first(t, 9, 256, 4096) = 256`, and no `u8` element of a byte string can equal 256. The contradiction is already in the initial dictionary, needs no input and no table bytes; `gif_verus/src/representation_domain.rs` proves it (run 614: 698 verified, 0 errors; the preconditions are satisfiable, so the lemma is not vacuous). RecommendedBeta rejects such a literal (`ColorIndexOutOfRange`) before walking its chain, so no public output is affected.

**Proposed erratum.** Restrict the literal arm of the representation lemma's domain to `0 <= c < min(clear, 256)`; keep the stored-entry arm `clear + 2 <= c < next` entirely; keep the checked-root invariant, and require the existing checked-literal guard to reject the excluded codes before resolution, emission or insertion. No observable G10 behaviour changes. The alternative is an integer-string dictionary (elements wider than a byte), which would need a separate conversion theorem to the byte-valued output.

**Effect.** Task 2 (independent string-table model, review finding 1) proceeds on the restricted domain; if the erratum is not approved, that task stays blocked.

### E4 Status (2026-11-17)

**APPROVED WITH CONDITIONS; conditions open until evidence is recorded.** The owner approved E4 as amended by the council on 2026-11-17 (same report and hash as E1; four members, round 3, all "approve with amendment"; astra recorded a dissent on the narrower coverage wording, and the broader wording below follows astra and fable). E4 takes effect only when obligations (a)–(c) are proved.

### E4 Approved text (2026-11-17)

Restrict the representation lemma's literal arm to `0 <= c < min(clear, 256)`; keep the stored-entry arm `clear + 2 <= c < next`.

Proof obligations:

- (a) For `m` in [9, 11], a literal `c` in [256, clear) returns `ColorIndexOutOfRange` before resolution, emission or insertion. Evidence: see the E4 Evidence (2026-11-17) block below; the wording is refined there for the full-raster case.
- (b) Every stored entry in [clear + 2, next) has a prefix chain ending at a literal below `min(clear, 256)`, and each insertion and each dictionary reset preserves this. Evidence: see the E4 Evidence (2026-11-17) block below.
- (c) Coverage: every code the decoder processes is a domain member, `clear`, end-of-information, the separately justified code `c = next`, or rejected with an error before resolution, emission or insertion. Evidence: see the E4 Evidence (2026-11-17) block below.

Also: (d) for `m <= 8` the domain is unchanged, since `min(clear, 256) = clear`. (e) Approval does not resolve review finding 1 (the independent string-table construction and representation proof); that proof now exists on this domain in `gif_verus/src/string_model.rs` (run 625), which depends on this erratum.

## E5 — 2026-11-17: shared guard ceiling (PROPOSED / informational; no GIF cap change)

Frozen `THEOREMS.md:9` says the guard's `MAX_BYTES` is 67,108,864 today and
therefore holds GIF's `MAX_OUTPUT` at 64 MiB; `spec-frozen/FREEZE.txt` records
the same context. Those files remain untouched. Owner-approved alloc-delta
erratum E16 raises only the shared guard ceiling to 1,073,741,824 bytes
(1 GiB), with production guard SHA-256
`add634bc74ff7feed568c90d5cc8c504538b4d4b80420e7fc9228fceecc2a137`. The reference to the old
ceiling should be read as historical; `MAX_OUTPUT <= MAX_BYTES` still holds.
GIF's `MAX_OUTPUT` remains 67,108,864 bytes (64 MiB), and its other caps and
theorem obligations do not change. A later GIF output-cap raise needs its own
owner decision/erratum and re-verification; this informational proposal does
not apply that raise or amend the frozen text. Evidence for the guard raise
is appended to `../alloc-delta/ERRATA.md` E16.

## Evidence (2026-11-17), recorded by the proof agent (Sol)

Source: `gif_verus/STUCK.md`, section "Owner-approved E1-E4 condition checks". Final tree: 729 verified, 0 errors (`runs/644.log`). Release tests, the allocation scanner self-test (exit 0) and the 14 frozen hashes pass (`runs/sol-20261117/e1-e4-release-tests.log`, `e1-e4-audit-self-test.log`, `e1-e4-frozen-hashes.log`). No frozen file changed. Line numbers below are in `gif_verus/src/` and were checked against the files on 2026-11-17.

### E1 Evidence (2026-11-17): DISCHARGED

- `compose_new.rs:953` `lemma_copy_rect_access`: each byte of a copied pixel is in `mc::in_clipped`. It is called at the four actual loads and stores (`compose_new.rs:1130-1138`).
- `compose_new.rs:985` `copy_rect`, strengthened postcondition at `:994-998`: every rectangle byte comes from the same source offset; every other destination byte is unchanged.
- `compose_new.rs:966` `lemma_copy_rect_restore_reads_saved`: `copy_rect(drawn, copy_rect(base, before, r), r) == copy_rect(drawn, before, r)` for arbitrary old `Saved` bytes.
- `frame_model.rs:124` `lemma_saved_image_restore`. Actual save `gif.rs:1721`, restore `gif.rs:1677`, `prev` stored at `gif.rs:819`.

The approved wording "restore reads `Saved` only at byte offsets inside the rectangle that the matching save wrote" is established. Layout, allocation bound, save timing and at-most-once reservation are unchanged. The citation `compose_new.rs:1055-1097` proposed earlier is not the proving postcondition; the lemmas above are.

### E2 Evidence (2026-11-17): inventory, obligations still open

- (1) Agreement of the standalone (`lzw_proof::model::ML`) and integrated (`gif_verus::lzw_model::ML`) models: NOT proved. No cross-crate theorem relates them; a textual diff is not a proof. `string_model.rs:304` `theorem_ml_st` compares the integrated ML with the independent string-table model ST, not the two ML models.
- (2) Full G8-G9 coverage and exactness of the integrated precondition: NOT separately proved. `lzw.rs:3082` `lemma_min_admissibility`, `:3089` `lemma_image_data_ready` and `:3099` `decode_image_data` supply the connection; the exactness theorem is absent.
- (3) Independent string-table representation: PROVED on E4's readable domain (`string_model.rs:325`, `:373`, `:304`).

The frozen T-LZW must not be claimed for the shipped decoder.

### E3 Evidence (2026-11-17): DISCHARGED; finding 6 stays open

- (b) `limits.rs:32` `caps_ok` now also ensures `8 * MAX_INPUT <= u32::MAX` (`limits.rs:35`). Live `MAX_INPUT` stays 268,435,456. All earlier ensures clauses remain.
- (a) Boundary run 645 (scratch copy, `MAX_INPUT = 536,870,911`, no proof differences from run 644; source hashes in `runs/sol-20261117/e3-boundary-run645-source-hashes.tsv`): 729 verified, 0 errors.
- Negative control run 646 (`MAX_INPUT = 536,870,912`): 727 verified, 2 errors (`caps_ok` postcondition `limits.rs:35`; `BitReader` `data_len * 8` overflow `lzw.rs:663`). Runs 628, 629, 630 did the same on the tree before the E1 and E4 additions.

Finding 6 stays open, as the council said (condition (c)). The claim "unchanged proofs for every cap within the amended G1 bounds" is established for `MAX_INPUT` only (up to 536,870,911) and NOT for `MAX_DIMENSION` or `MAX_TOTAL_PIXELS`:

- Run 638 (`MAX_DIMENSION = 32,767` only) fails downstream descriptor and screen-cap contracts (`parse.rs:1802`, `:1817`; `compose_new.rs:112`, `:191-195`; `gif.rs:1686`, `:1790`; `pixel_model.rs:76-79`). A Clip descriptor with left = 32,768 on a 1x1 screen is legal at that cap but violates them.
- Run 639 (`MAX_TOTAL_PIXELS = u32::MAX` only) fails the extra assumption `MAX_TOTAL_PIXELS + 4096 <= u32::MAX` at `lzw.rs:2163`, `:2336`, `:2960`, although `caps_ok` permits that value.
- Mechanical cleanup done: `frame_model.rs:107` `image_domain` uses LE16 wire bounds for descriptor fields; `frame_model.rs:260` no longer asserts `MAX_DIMENSION == u16::MAX`; `gif.rs:679` narrows the area from the wire product. The numerals 131070 encode twice the LE16 wire maximum and are not configurable caps.

Tracked as `../RELEASE-GATES.md` item 8.

### E4 Evidence (2026-11-17): (b), (c), (d) DISCHARGED; (a) corrected

- (b) `string_model.rs:221` `lemma_stored_root`: every stored prefix chain ends at a literal below both `clear` and 256. `theorem_representation` (`:325`) and `theorem_all_represented` (`:373`) export it for actual raw table roots. Preservation: `lemma_initial` (`:122`), `lemma_clear` (`:171`), `lemma_emit` (`:183`). The actual decode calls the raw-root theorem initially, after Clear and after every insertion (`lzw.rs:2453`, `:2540`, `:2581`).
- (c) `lzw.rs:2800` `lemma_dispatch_coverage` with `dispatch_covered` (`:2768`): every consumed code is a readable literal or stored entry, Clear, EOI, a justified KwKwK (c = next, previous exists, next < 4096) or an error-only Halt. Called after every `read_step` decision (`lzw.rs:2518`).
- (d) `string_model.rs:236` `lemma_small_literal_domain`.
- (a) Established only for q < A (with enough bits for a code). `lzw.rs:2785` `lemma_checked_literal_rejected` proves exact `ColorIndexOutOfRange` for 9 <= m <= 11, checked literals, a complete code c in [256, clear) and q < A, called after `read_step` at `lzw.rs:2516-2524`, before any resolution, emission or insertion. Runtime test `lzw_tests.rs:135` covers all 2,816 excluded literals for m = 9..11 (no table change, no sink calls) and both tail behaviours.
- The council's unconditional wording of (a) is false. Counterexample, proved at `string_model.rs:434` `lemma_checked_literal_full_raster_counterexample`: m = 9, A = 1, payload [0, 0, 4] (codes 0 then 256, ten bits each), checked literals, Strict tail. The decoder emits 0, then G10 step 4 returns `MissingEndOfInformation` at bit position 20 before reaching the checked-literal guard. With `StopAtRaster` it stops successfully at bit position 10 without consuming 256.

**Operative wording of E4 (a), replacing the approved text above:** "for m in [9, 11] and a complete code c in [256, clear) read while q < A, the decoder returns `ColorIndexOutOfRange` before resolution, emission or insertion; when q = A (raster full) the frozen G10 precedence applies first (Strict: `MissingEndOfInformation`; `StopAtRaster`: the code is not read); in every case such a code is rejected with an error before resolution, emission or insertion."

This refines the council's wording without changing its intent: the coverage clause (c) already said "rejected with an error". Frozen G10 error precedence is unchanged. The owner's approval covers it as a precision fix, since the owner delegated these details to the council. Flag for the next council round if one is held.

### E4 Correction (2026-11-17), from the second independent review (`REVIEW-ASTRA-INTEGRATED-2.md`, Low finding)

The sentence above ("Operative wording of E4 (a)...") ends by concluding that "in every case such a code is rejected with an error". That is false for the excluded case m = 9, area 1, payload [0, 0, 4] with checked literals: under Strict the tail emits index 0 and then reports `MissingEndOfInformation`; under StopAtRaster the decoder returns after index 0 without consuming literal 256 (`string_model.rs:65-80`, `:434-443`). The existing text is kept; the operative clause is corrected to:

"an excluded literal in [256, clear) is never resolved, emitted or inserted; if it is consumed (read while q < A), the decoder returns `ColorIndexOutOfRange` before those operations; when the raster is full the frozen G10 precedence applies first (Strict: `MissingEndOfInformation`; StopAtRaster: the code is not read)."

No code or proof change is needed: the proofs already establish exactly this.
