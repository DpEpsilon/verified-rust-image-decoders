# Astra inflate review

Reviewer: GPT-6 Astra (adversarial, read-only), 2026-11-18. Scope: `png/inflate_verus` at commit 682f926 plus the uncommitted step-2c layer (fresh run 343: 272 verified, 0 errors). The reviewer's text is saved here by the coordinator, verbatim in substance, because the reviewer's own rules forbid it writing report files.

No source, proof, frozen document, configuration or target cache was edited. One permitted verifier invocation appended run 344, which reused Cargo's cache and printed no count, so the reviewer did not independently reproduce a fresh 272-obligation run; `runs/0343.log` contains `verification results:: 272 verified, 0 errors`. Scratch files: `png/inflate_verus/review-astra/`.

## Verdict

Both actual public entries have model-refinement, output-prefix, untouched-suffix, source-view and work-bound postconditions. No circular use of executable Huffman tables or output buffers in the independent models, and no input contradicting the frozen decoding decisions, was found.

Two qualifications before describing the result as the complete frozen statement: T-INFL's restriction on other observable effects does not follow from the generic `Segments` contract, and the returned counters do not implement the manifest's disjoint exact accounting at the first EOF. The latter overcounts; the numerical upper bounds still hold.

Acknowledged limitations remain material: host verification only; four representable option tuples rather than the eventual 64; incomplete witness and mutation gates; L-LIMIT-MONO unproved; no signed allocation/call-graph audit. This is not M2 sign-off.

## Findings

### 1. Medium, confirmed: T-INFL's effect restriction is not established for generic readers

Locations: `png/THEOREMS.md:112`; `png/inflate_verus/src/segments.rs:35–83`; `src/inflate.rs:1319–1335`, `:1342–1362`.

T-INFL(a) says output bytes are the only observable effect besides the source cursor. The public contracts preserve source well-formedness, concatenated input, segment count and cursor bounds, but neither they nor `Segments::read` prohibit a reader from writing through another mutable reference it owns.

A wrapper around `SegmentSlices` whose `read` first sets an unrelated caller-owned byte to 99 and then delegates satisfies every listed source view (concat, position, segment index/count, counters, rank, well-formedness), and adds no loop work. Run against the release library with input `78 01 03 00 00 00 00 01` and a one-byte output initialised to `a5`, the public `into` call returned Done, produced=0, output unchanged, and changed the caller's marker from 7 to 99. Reproducer: `review-astra/caller_effect.rs`. The wrapper itself was not submitted to Verus.

This is not a decoding or memory-safety exploit in `SegmentSlices`; it is a statement-level mismatch. Either add a reader framing restriction and discharge it, or obtain an approved clarification of T-INFL's effect clause.

### 2. Low, confirmed: the first EOF loop iteration contributes to both counters

Locations: `png/THEOREMS.md:68,113,123`; `png/REVIEW-SOL-THEOREMS.md:66`; `src/segments.rs:150–180` (161–167); `src/bits.rs:139–140`; `src/lib.rs:101`.

The frozen definition partitions loop iterations into two disjoint counters. On the first EOF, the same `SegmentSlices::read` iteration increments `seg_work` (line 164) and `read_work` (line 166), and `Bits::byte` mirrors the read charge into `Work` (line 139). With one empty segment and Z=[], two concrete iterations yield core_steps=1, seg_steps=2 (sum 3); with zero segments, one iteration yields a sum of 2.

Conservative overcount by one at first EOF, not an uncharged loop; the bounds including `seg_steps <= seg_count+1` hold. It contradicts the exact/disjoint definition and the comment that public fields are exact per-call counts. Count that iteration once and update rank/counter bookkeeping, or obtain an approved accounting-definition change.

## Questions that produced no defect

- Model independence: `model.rs` builds canonical codewords from mathematical length counts, decodes by exact/prefix membership and evolves `Seq<u8>`; no executable tables, buffers, ring state or `ring_repr`. Shared format constants are not circular.
- D4: `model.rs:111–130` tests exact codewords, then dead prefixes, then exhaustion; the executable decoder checks an empty code before reading and liveness after a failed lookup.
- D5/D12: `model.rs:1151–1164` checks stored-byte availability before the limit; filling L exactly is not Full; matches check distance/output/window before emission; final blocks reach checksum handling at exactly L.
- Ring precondition: `inflate.rs:1344` requires exactly WINDOW_MAX bytes and `:1364` asserts it at runtime; a wrongly sized slice panics, but that caller violates T-INFL(b)'s explicit condition.
- Representable options: `lib.rs:49–80` gives one inhabitant for adler, stream_end and single_code, two for window and zlib_tail; all representable tuples are covered.
- Prophetic specifications (`bits.rs:64–67`, `inflate.rs:179–183`) name eventual contents of existing borrows; no future-value assumption in public preconditions.
- Loop charges and constants: every inspected production loop is charged; C_IN=512, C_OUT=8, C_INFL_FIXED=4096, C_SEG=1 match the manifest; no omitted charge beyond finding 2.
- Trusted surface: no assume/admit/external_body/assume_specification/unsafe; attributes seen are prophetic and spinoff_prover; `--no-cheating`; cfg gates exclude only ghost material.

## Witness and mutation evidence

The twelve witnesses in `src/witnesses.rs` compare model results with literal expectations and make real calls to both public entries; `tools/audit_witnesses.py` confirms the literal inputs match frozen IDAT extractions (provenance only). None exercises the dynamic/compressed model or ring wrap. Runs 317–328 are real verifier failures (271 verified, 1 error); some kills land at local assertions or invariants rather than the public postcondition. Remaining witnesses, mutations, L-LIMIT-MONO and the i686 replay are open, as `LEDGER.md:30–38` states.
