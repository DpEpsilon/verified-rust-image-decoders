# Errata against the frozen GIF document set

Append-only. This file is **not** covered by the GIF freeze: it records
discrepancies between the frozen texts (`THEOREMS.md`, `spec-frozen/`) and the
implementation. Nothing here edits a frozen file. New entries are appended at the
end with their date and the artifact they concern. An entry marked PROPOSED has
not been applied: it takes effect only after the owner approves it, and the entry
is then amended in place of a new one by appending an "Owner decision" line.

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

## E3 (PROPOSED, awaiting owner approval; not applied): the permitted range of `MAX_INPUT`

**Frozen text.** G1's derived bounds and `THEOREMS.md:9`: cap values may change by editing `limits.rs` alone, with every theorem statement and proof unchanged, provided the derived bounds stated in `SPEC.md` G1 hold. G1 states that `MAX_INPUT` plus one 65,536-byte block must fit a 32-bit `usize`, which permits `MAX_INPUT` up to about 4 GiB.

**Implementation.** `BitReader::new` (`gif_verus/src/lzw.rs:663`) computes the payload bit count `data_len * 8` in `u32`. With `MAX_INPUT = 2^29` the helper call `BitReader::new(&[], 0, 536870912)` satisfies its precondition and computes 4,294,967,296, one above `u32::MAX`. This is the smallest helper-domain counterexample; it is not a reachable failure of the public decoder at the current cap (`MAX_INPUT = 2^28`). Evidence: runs 609 (`MAX_INPUT = 2^27`: all 697 verify unchanged), 610 and 611 (`MAX_INPUT = u32::MAX` and `2^29`: the bit-reader multiplication fails), recorded in `RUNLOG.tsv`.

**Proposed erratum.** Add to G1's derived bounds `8 * MAX_INPUT <= u32::MAX`, that is `MAX_INPUT <= 536,870,911`, and to the `caps_ok` lemma the same condition. The alternative is to widen bit counts, positions, timestamps and reports to `u64` throughout the proofs while preserving the integer observations of ML; that restores the frozen "any G1-permitted cap" reading at the cost of a large proof change for an input cap above 512 MiB that no decoder in the corpus survey needs.

**Effect.** None at the current cap. Any later raise of `MAX_INPUT` above 512 MiB needs the `u64` widening first.

## E4 (PROPOSED, awaiting owner approval; not applied): domain of the T-LZW representation lemma's literal arm

**Frozen text.** `THEOREMS.md:71` (T-LZW representation lemma): the domain contains every `c < clear` (plus `clear + 2 <= c < next`) and requires `model_table[c][0] = code_first(t, m, c)`, with `model_table` a dictionary of byte strings.

**Contradiction (found by Sol, 2026-11-16).** Under the checked-literal policy (`m = 9, 10, 11`) the literal codes below `clear` include `c` in `[256, clear)`. For `m = 9`, `clear = 512`, `c = 256`, the executable's literal rule gives `code_first(t, 9, 256, 4096) = 256`, and no `u8` element of a byte string can equal 256. The contradiction is already in the initial dictionary, needs no input and no table bytes; `gif_verus/src/representation_domain.rs` proves it (run 614: 698 verified, 0 errors; the preconditions are satisfiable, so the lemma is not vacuous). RecommendedBeta rejects such a literal (`ColorIndexOutOfRange`) before walking its chain, so no public output is affected.

**Proposed erratum.** Restrict the literal arm of the representation lemma's domain to `0 <= c < min(clear, 256)`; keep the stored-entry arm `clear + 2 <= c < next` entirely; keep the checked-root invariant, and require the existing checked-literal guard to reject the excluded codes before resolution, emission or insertion. No observable G10 behaviour changes. The alternative is an integer-string dictionary (elements wider than a byte), which would need a separate conversion theorem to the byte-valued output.

**Effect.** Task 2 (independent string-table model, review finding 1) proceeds on the restricted domain; if the erratum is not approved, that task stays blocked.

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
