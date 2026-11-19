Reviewer: GPT-6 Astra, 2026-11-19, commit b34b217. Read-only; saved verbatim by the coordinating session because the reviewer's instructions forbade writing report files.

# Adversarial M2 sign-off review

**Verdict: no, not yet.** At commit `b34b217ee033de2033fe616cd1e23de973591f12`, I found one definite missing M2 assurance gate: the required stored-truncation/prefix-loss mutation has no recorded verifier kill. The implementation refinement, named witness gates, amended reader-effect obligation and host/i686 split checkpoints withstand this review. The remaining work does not require redesigning inflate or replacing the new witnesses.

I did not run Verus, Cargo, builds or mutations. I ran the split coverage check and the complete witness audit with bytecode generation disabled and the inventory write suppressed; the suppressed output was identical to the existing inventory. Both passed. I inspected the retained verifier and runtime evidence, source contracts, mutation definitions and relevant historical diagnostics. I wrote no files and started no background jobs. The instruction prohibiting report files takes precedence, so this report is the deliverable rather than `/home/ws/projects/verified-decoders/png/REVIEW-ASTRA-M2.md`.

## Blocking item

1. **Run and retain the missing mutation for loss of the present stored-block prefix on truncation.** `/home/ws/projects/verified-decoders/png/THEOREMS.md:253` requires mutation 39: dropping present stored bytes before returning `TruncatedZlib`. The milestone-completion rule requires its mutations to be killed by verifier failures, not merely covered by positive proofs or runtime tests (`/home/ws/projects/verified-decoders/png/THEOREMS.md:260–266`). The mutation inventory in `/home/ws/projects/verified-decoders/png/inflate_verus/tools/proof_mutations.py:16–118` and the mutation runs in `/home/ws/projects/verified-decoders/png/inflate_verus/RUNLOG.tsv` contain no such case. The 67 historical kills do not close this particular requirement.

   The named frozen witness does not substitute for this mutation: `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_cases.rs:329–338` shows that `zlib/cut-mid-stored` cuts inside LEN/NLEN and has an **empty** output prefix. The useful nonempty-prefix witness already exists at `/home/ws/projects/verified-decoders/png/inflate_verus/src/witnesses.rs:426–445`: two present payload bytes, a missing third byte, and calls of both entries at limits 4, 2 and 1.

   **Concrete fix:** add an implementation-only mutant that discards the produced prefix/count on a stored-payload truncation while retaining `TruncatedZlib`; leave the independent model, contracts and witness expectations unchanged. Run the prescribed bounded verification in a disposable copy, retain a real failed proof diagnostic rather than a type-check failure, and record its association with mutation 39. Use the existing nonempty-prefix witness to demonstrate that the mutated behavior is observable. Then obtain the normal clean-source checkpoint. Do not change the frozen fixture to manufacture a nonempty prefix.

## Theorem and domain assessment

These conclusions concern the actual inflate APIs, not PNG integration. The 16 council-excluded instances are not blockers: `/home/ws/projects/verified-decoders/png/inflate_verus/LEDGER.md:3–9` explicitly limits that decision to supplementary accounting, preserves every named M2 gate, and leaves L-WALK-FIRST and the malformed-container expectations for M3/M6.

### T-HUFF, L-CANON and L-PREFIX-FREE — proved on the stated M2 domains

- `/home/ws/projects/verified-decoders/png/inflate_verus/src/model.rs:22–64` defines the bounded length-vector domain, mathematical Kraft sum and kind-specific admissibility independently of the executable arrays.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/classification.rs:64–81` returns the exact Kraft sum and counts even for oversubscribed vectors; `/home/ws/projects/verified-decoders/png/inflate_verus/src/limits.rs:43–56` supplies the machine-arithmetic bound.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/canonical.rs:152–175,264–307` gives the counts/first-code/symbol-order representation and proves construction succeeds exactly for valid, non-oversubscribed lengths.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/huffman.rs:36–51,69–95` binds lookup to independent codewords in both directions and proves admissibility for **both values of the `single_accept` Boolean**, even though the public option type currently exposes only `SingleCode::Accept`.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/model.rs:403–433` proves that a codeword prefix implies the symbols are identical; this establishes both distinctness and absence of proper prefixes.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/huffman.rs:162–180` binds actual decoding to `MDec`, including the exact position of successful, dead-prefix and live-prefix-EOF results. `ready` requires admissibility, so the decoding theorem does not silently assume that malformed dynamic tables are acceptable.

The model permits length vectors up to 288 entries for its generic classification machinery; that is not a weakening of the smaller distance/code-length domains in T-HUFF. The executable dynamic-header code separately imposes their format-specific sizes.

### T-INFL — proved for Strict and every currently representable tuple, with E2's amended effects

The public contracts are at `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:1335–1386`. They establish exact status and produced length on **all outcomes**, output-prefix equality and untouched suffix for `into`, and the ring representation for `measure`. Neither contract assumes valid compressed input or successful completion.

The contracts require only source `wf()` and a fresh byte cursor, plus exactly `WINDOW_MAX` ring bytes for measuring. The source invariant supplies the manifest's concatenation ceiling; it is not an extra PNG `MAX_INPUT` precondition. There is no hidden requirement to reset segment-work counters or start at segment index zero. Into limits range over actual target slice lengths; measuring limits range over `u64`, as the API requires.

The public `InflateOptions` type has **four**, not 64, possible values: two window choices times two tail choices, with singleton Adler, stream-end and single-code fields (`/home/ws/projects/verified-decoders/png/inflate_verus/src/lib.rs:127–168`). `/home/ws/projects/verified-decoders/png/inflate_verus/src/model.rs:1747–1755` expressly models that implemented domain. The four tuples are universally covered. The absent compatibility values are **not proved**, but `/home/ws/projects/verified-decoders/png/PLAN.md:23` explicitly schedules the remaining values for P2 and `/home/ws/projects/verified-decoders/png/THEOREMS.md:112` freezes Strict rather than all future combinations. This is not an M2 exclusion disguised as a theorem.

The public result does not expose a bit position. The internal observation relation does preserve error positions (`/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:285–291,1202–1216`); the public contract intentionally leaves the final source cursor unspecified within its bounds. That matches the frozen public observation and E2 rather than weakening it.

### T-SEG and L-BITS — proved for the trait contract and concrete SegmentSlices reader

`/home/ws/projects/verified-decoders/png/inflate_verus/src/segments.rs:35–92` requires preserved concatenation/frame/count, exact returned byte and cursor progression, monotone segment rank, bounded segment work and the core-work increment. The separate `lemma_wf` is important: generic consumers do not unsoundly rely on an overridable default `wf` definition.

`SegmentSlices` is verified at `/home/ws/projects/verified-decoders/png/inflate_verus/src/segments.rs:96–208`. It handles empty parts and repeated EOF with a decreasing traversal variant. The previously reported double charge of the first EOF iteration is fixed: first EOF contributes segment work only; subsequent EOF contributes one core iteration (`:186–206`). The zero/empty-reader witnesses at `:211–238` pin that distinction.

`/home/ws/projects/verified-decoders/png/inflate_verus/src/bits.rs:46–64,122–175` relates the real cache to the concatenation and proves `get(n)` equals mathematical field extraction for every `n ≤ 32`. Successful and truncated reads preserve the appropriate exact position. The implementation's smaller, one-byte cache is a stronger restriction than the manifest's at-most-63-buffered-bits allowance, not a weaker theorem.

The integrated IDAT, fdAT and iCCP readers are not proved here. T-SEG explicitly assigns L-SEG-PNG to M3.

### E2 / L-SEG-FRAME — satisfied for SegmentSlices; not a universal arbitrary-reader theorem

The amended obligation is `/home/ws/projects/verified-decoders/png/ERRATA.md:67`. The current evidence respects its distinction between verified value preservation and an inspected Rust effect argument.

The concrete type contains shared byte-slice borrows and scalar traversal fields, not mutable referents, interior mutability, raw pointers or I/O capabilities (`/home/ws/projects/verified-decoders/png/inflate_verus/src/segments.rs:96–104`). `source_frame` returns the complete noncursor `parts` field (`:154–155`), and `read` ensures preservation on Some and None returns (`:168–174`). The full four-clause discharge, enumerated cursor fields, checked/inspected distinction and actual entry instantiations are recorded at `/home/ws/projects/verified-decoders/png/inflate_verus/LEDGER.md:341–351`.

For a different `Segments` implementation, T-SEG alone still does not exclude external effects. Both public entries correctly preserve only the declared frame and concatenation generically. M3 must discharge L-SEG-FRAME separately for each real PNG reader and connect it to both call sites. E2 approves this narrower universal statement; it does not approve a claim of universal no-extra-effects.

### L-WINDOW, L-RING and L-LIMIT — proved

`/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:194–202` ties each reachable sink to the evolving model sequence and preserves the unwritten suffix. The ring predicate at `:130–134` covers every retained byte, not only the most recent one. `previous` at `:273–280` returns the exact model byte for admissible distances.

The real copy loop checks distance against both produced output and the selected window before writing (`/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:855–900`), and its invariant equates the output to successive independent `copy_distance` results. This covers overlap and distance equal to produced output.

`append_byte` tests the limit before writing (`:232–269`). Stored payload refinement at `:295–309` preserves the ordered availability/limit behavior and partial prefixes. Reaching the limit does not bypass finishing: `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:1181–1199` still checks the checksum and the applicable tail rule. The missing mutation above is a missing falsification gate, not a discovered failure of these contracts.

### L-LIMIT-MONO — proved

`/home/ws/projects/verified-decoders/png/inflate_verus/src/limit_mono.rs:234–251` has the intended antecedent, `Done` and `out.len() ≤ small ≤ large`, and concludes equal output, `Done` and equal R2 for both supported window/tail Booleans. It is a theorem of the independent model, usable with the universal public refinement for the second measuring/into pass. It is not a promise that every arbitrary error or `Full` outcome is invariant under a limit change.

### L-ADLER — operational equality and overflow safety proved

`/home/ws/projects/verified-decoders/png/inflate_verus/src/adler.rs:8–47,67–115` proves the deferred accumulator bounds, maintains congruence with the mathematical sums and returns exactly `Adler(data)`. Reduction occurs at `ADLER_NMAX`, not one byte later (`:99–108`). The independent sums are in `/home/ws/projects/verified-decoders/png/inflate_verus/src/model.rs:1279–1296`.

One small distinction from the full wording should be recorded: the proof explicitly establishes the safe bound at `ADLER_NMAX`; it does not export a separate maximality postcondition saying that `ADLER_NMAX + 1` violates that bound. The parenthetical arithmetic in `/home/ws/projects/verified-decoders/png/THEOREMS.md:134` is true: the expression is 4,294,690,200 at 5,552 and 4,296,171,735 at 5,553, on opposite sides of 2^32. This is an auxiliary certificate/documentation improvement, not a hole in the accumulator or public refinement proof.

### L-HDR and L-HDR-BITS — proved at the real construction boundary

`/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:738–775` refines the dynamic header to `MDynamic`; its helpers preserve the repeat and admission ordering. EOB is checked before literal admission, which precedes distance admission (`:722–735`).

The 16-bit aggregate code-length lower bound is connected to the real literal-table construction marker (`:600–616`), not merely an unused arithmetic lemma. The outer block-header contract gives at least 45 consumed bits whenever construction is reached and separately bounds construction below 2,048 (`:781–794`). The comment on the arithmetic lemma at `/home/ws/projects/verified-decoders/png/inflate_verus/src/model.rs:1522` saying it is “not yet tied” is stale; the current executable proof does tie it.

### T1, T11 and T12 — proved within the documented entry/reader contract

The public APIs have no valid-input assumption. The measuring assertion at `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:1385` is justified by its explicit ring-length precondition; it is not an unconditional panic-freedom promise for malformed caller buffers or reused nonfresh cursors.

Block and symbol loops decrease remaining input bits (`/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:995–1012,1095–1109`), a stronger sufficient variant than needing the second component of the manifest's lexicographic variant. Match, header, code and bit loops have their own bounded variants. The inspected production implementation loops rather than recursing; recursive mathematical helpers erase.

Work observations really are erased `Ghost<nat>` values (`/home/ws/projects/verified-decoders/png/inflate_verus/src/lib.rs:180–203`). The public inequalities at `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:1348–1352,1379–1383` use the required constants and include every outcome. The segment result is the actual reader-counter delta. I checked the production loops and bulk initializations against their charges, including five Huffman initializations, table-building helper counters, dynamic length fills, match-copy iterations, bit iterations and checksum reads. The disjoint source-reader accounting is consumed at `/home/ws/projects/verified-decoders/png/inflate_verus/src/bits.rs:148–150,181–195`.

As with the manifest's step model, the connection between source operations and the manually placed counters requires inspection of the instrumentation; Verus then proves their arithmetic bounds. Runtime differential tests do not measure these erased counters. A generic reader must actually meet T-SEG's behavioral work contract, not merely supply arbitrary numerical ghost fields.

## Every named M2 witness gate

**All 54 principal records, all three segmentation records, both window/flush records and all nine named RecommendedBeta records have the required proof/call binding.** These are 68 named records after expanding the seven-record tail wildcard. The supplementary 615/631 figure is not what establishes this conclusion.

The following index gives each gate's executable witness location. Record identifiers below are fixture names, not filesystem paths. A line number within a row refers to the absolute source file named in that row.

| Witness source | Named gate records and lines |
|---|---|
| `/home/ws/projects/verified-decoders/png/inflate_verus/src/witnesses.rs` | `deflate/stored-only` 179; `deflate/fixed-literals` 377; `deflate/stored-nlen` 364; `zlib/cm-7` 256; `zlib/cinfo-8` 268; `zlib/fdict` 280; `zlib/one-byte` 292; `zlib/empty-idat-only` 144; `zlib/bad-adler` 304; `zlib/missing-adler` 316; `zlib/partial-adler` 328; `zlib/trailing-byte` 340; `zlib/second-stream` 352 |
| `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_cases.rs` | `deflate/empty-stored-then-stored` 392; `deflate/distance-equals-output` 243; `deflate/match-overruns-image-is-excess` 290; `deflate/btype-3` 341; `deflate/fixed-lit-286` 8; `deflate/fixed-dist-30` 103; `deflate/fixed-dist-31` 138; `deflate/fixed-dist-30-then-eof` 173; `deflate/fixed-dist-31-then-eof` 208; `deflate/distance-before-output` 36; `deflate/distance-beyond-output` 67; `deflate/hlit-287` 352; `deflate/hdist-31` 363; `zlib/fcheck` 383; `zlib/cut-mid-stored` 330 |
| `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_dynamic_cases.rs` | `deflate/dynamic-literals-two-dist-codes` 12; `deflate/dynamic-single-dist-code-len1` 80; `deflate/dynamic-no-dist-codes` 148 |
| `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_overlap.rs` | `deflate/fixed-overlapping-matches` 47 |
| `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_single_literal.rs` | `deflate/single-literal-code-len1` 12 |
| `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_repeats.rs` | `deflate/code-length-code-oversubscribed` 71; `deflate/code-length-code-incomplete` 95; `deflate/repeat-with-no-previous` 12; `deflate/repeat-overflows-count` 39 |
| `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_invalid_dynamic.rs` | `deflate/no-end-of-block-code` 7; `deflate/incomplete-literal-code` 42; `deflate/incomplete-distance-code` 77; `deflate/oversubscribed-literal-code` 112 |
| `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_d4.rs` | `deflate/unassigned-code-in-incomplete-literal-code` 280; `deflate/length-with-no-distance-codes` 322; `deflate/single-literal-code-other-bit-then-eof` 12; `deflate/length-with-no-distance-codes-then-eof` 111; `deflate/single-distance-code-other-bit-then-eof` 158; `deflate/single-literal-code-live-prefix-eof` 54; `deflate/single-distance-code-live-prefix-eof` 207 |
| `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_config_cases.rs` | `deflate/distance-within-declared-window` 2699; `deflate/distance-beyond-declared-window` 2536; `image/excess-data` 2094; `image/excess-row` 2238; `image/truncated-data` 1461; `zlib/window-9` 972 |
| `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_blocks.rs` | `zlib/cut-mid-block` 302; `zlib/sync-and-full-flush-empty-stored-blocks` 168 |
| `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_extended.rs` | `idat/zero-length-idats` 4716; `idat/split-irregular` 4688 |
| `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_special.rs` | `zlib/trailing-in-later-idat` 37; `policy/window/distance-before-output-stays-an-error/recommended-beta` 54 |
| `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_config_cases.rs` | `policy/window/distance-beyond-declared-window/recommended-beta` 3849; `policy/zlib-tail/excess-output-stays-an-error/recommended-beta` 2972; `policy/zlib-tail/excess-row-stays-an-error/recommended-beta` 3229; `policy/zlib-tail/excess-output-and-trailing-byte/recommended-beta` 3516 |
| `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_policy_cases.rs` | `policy/zlib-tail/trailing-byte/recommended-beta` 7; `policy/zlib-tail/second-stream/recommended-beta` 59; `policy/zlib-tail/trailing-in-later-idat/recommended-beta` 111; `policy/zlib-tail/bad-adler-with-trailing-byte-stays-an-error/recommended-beta` 164 |

These are not merely names in JSON. `/home/ws/projects/verified-decoders/png/inflate_verus/tools/audit_witnesses.py:130–197` binds parsed, unconditional calls to exact compressed bytes, independently derived R, segmentation where required, options, status and authored raw output. The shared harnesses make **both** public calls and assert their outputs: `/home/ws/projects/verified-decoders/png/inflate_verus/src/witnesses.rs:221–250` and `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_policy.rs:5–22`. The layout bridge constructs the original segments before calling that harness (`/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_layout.rs:29–57`). Thus a flattened-stream proof alone is not being credited as the T-SEG boundary gate.

The read-only audit reported 615 of 631 extracted core instances, with the other 16 uncredited; 18 policy records are separate. All named M2 core records have their extracted streams covered. The council exclusions therefore do not hide a missing gate.

## Round 6–7 independence and binding

I found no oracle, decoder, executable decode-table import or frozen expected-result field supplying expected bytes in the spot-checked new authoring chain.

- Fixed configurations derive raw samples/filter bytes independently, choose forward match tokens from those raw bytes and compare a complete forward encoding with compressed input. They do not read compressed symbols to obtain the raw output (`/home/ws/projects/verified-decoders/png/inflate_verus/tools/author_config_witnesses.py:19–100,102–119`). Expected Full/window-error prefixes come from those authored tokens. The proof bridge checks token effects against the independent model (`/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_config.rs:12–36,99–119`).
- Dynamic profile witnesses start with explicit samples and forward filtering (`/home/ws/projects/verified-decoders/png/inflate_verus/tools/author_profile_witnesses.py:25–39`). `/home/ws/projects/verified-decoders/png/inflate_verus/tools/author_huffman_intent.py:7–83` builds frequencies, lengths, canonical words and an encoding from the authored tokens. Its tables are encoder-side candidates, not tables imported from the decoder. The Verus witnesses independently establish repeat fields, codewords and copy effects. The HCLEN bridge handles declared fields and omitted zeros (`/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_profile_bridge.rs:5–24`).
- The flush author constructs a five-block schedule and compares its encoding; the EOF author derives the 552-byte prefix from an authored complete token stream and proves live-prefix EOF (`/home/ws/projects/verified-decoders/png/inflate_verus/tools/author_block_witnesses.py:21–36`; `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_block_bridge.rs:5–41,43–79`).
- Finishing witnesses use explicit sample/filter intent and tokens before encoding comparison (`/home/ws/projects/verified-decoders/png/inflate_verus/tools/author_finish_witnesses.py:35–65`). The stored/fixed composition bridges require raw-payload or token-trace evidence plus independent checksum equality; they do not manufacture expected output by running inflate (`/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_stored_bridge.rs:5–44`).
- The shared authoring module uses authored/RNG-replayed samples and forward filtering, not frozen expected pixels (`/home/ws/projects/verified-decoders/png/inflate_verus/tools/author_extended_classes.py:1–15,36–81`). Matching a complete independent forward encoding to the frozen compressed bytes is an eligibility check, not an output oracle.

The independent core model itself only imports format constants and vstd (`/home/ws/projects/verified-decoders/png/inflate_verus/src/model.rs:1–4`). It defines canonical recurrence/prefix matching, sequence append/copy and mathematical checksum sums, not executable Huffman tables or output-ring state. Shared format constants do not defeat independence; importing the implementation's table recurrence or ring would have done so, and I found neither.

## Split verification and i686 replay

**No current obligation was found gated out of every pass. The unique total is sound for this source tree.**

`/home/ws/projects/verified-decoders/png/inflate_verus/src/lib.rs:18–96` selects leaf witness modules only; core modules and cross-witness helpers remain in base. `/home/ws/projects/verified-decoders/png/inflate_verus/tools/split_checkpoint.py:66–129` checks module declaration coverage, unique ownership, exact feature definitions and cross-pass witness imports. My coverage run found 39 Verus modules assigned to six passes.

The pass checker requires the exact source-bound command, one JSON report, successful whole-crate verification, positive counts and the expected module inventory (`/home/ws/projects/verified-decoders/png/inflate_verus/tools/split_checkpoint.py:132–181`). Across passes it compares base function identities (`:184–206`). I also checked the retained function inventories directly: every pair of leaf-pass crate-function sets intersects in exactly the base set. No extra shared leaf contribution needs subtracting.

Counts are 372, 811, 677, 517, 452 and 410. Their plain sum is 3,239. Base was counted six times, so subtract five redundant copies of 372: **3,239 − 1,860 = 1,379**. The source-equivalent monolithic run 582 independently reports the same verified count. JSON function inventories include spec functions and are not themselves the verifier's `verified` count; confusing those quantities would give a different, invalid total.

The retained source inventory is checked against current files by `/home/ws/projects/verified-decoders/png/inflate_verus/tools/audit_witnesses.py:347–386`. This audit passed, including the source-equivalent monolithic reference and both checkpoints.

A Cargo replay cannot count as a fresh split run: it lacks the mandatory JSON report, and `json_report` rejects it (`/home/ws/projects/verified-decoders/png/inflate_verus/tools/split_checkpoint.py:137–147`). The driver also checks exit status and source stability before writing a checkpoint (`/home/ws/projects/verified-decoders/png/inflate_verus/tools/verify_split.py:46–88`).

The i686 cache build is correctly distinguished from root verification. `/home/ws/projects/verified-decoders/png/inflate_verus/tools/build_i686_vstd.sh:5–10,25–28` sends `--no-verify` only to the root while building vstd without forwarded Verus arguments. `/home/ws/projects/verified-decoders/png/inflate_verus/I686-CACHE.md:15–17,32` explicitly makes no inflate proof claim for run 602 and warns about replaying its feature-free no-verify result. The six real target passes use distinct split features and normal no-cheating verification.

The target audit requires each pass's verbose rustc line to name i686 and the recorded vstd artifact, requires `Fresh vstd`, rejects rebuilding vstd, and requires the same source inventory and unique count as host (`/home/ws/projects/verified-decoders/png/inflate_verus/tools/audit_witnesses.py:325–343`). The retained logs satisfy those checks. Runs 603–608 are not host proofs relabeled as i686 or cached root no-verify runs.

Toolchain evidence names Verus **0.2026.11.02.8bf68ee**, vstd **0.0.0-2026-11-01-0251**, and rustc **1.99.1**. The target/cache provenance is at `/home/ws/projects/verified-decoders/png/inflate_verus/I686-CACHE.md:5–17`. No new verification was performed during this review.

## Mutation mapping

The following run numbers refer to `/home/ws/projects/verified-decoders/png/inflate_verus/RUNLOG.tsv`; its line number equals the run number for these rows. I inspected retained diagnostics for the relevant early kills, not only their nonzero process status.

| Required mutation | Evidence |
|---|---|
| 1 — swap fixed/dynamic BTYPE | Run 383 |
| 2 — LSB-first Huffman decoding | Run 371 |
| 3 — admit incomplete literal code | Run 372 |
| 4 — omit NLEN | Run 317 |
| 5 — wrong symbol-285 base | Run 320 |
| 6 — reserved distance / 30-entry fixed table | Runs 387 and 382 |
| 7 — distance equals output + 1 | Run 384 |
| 8 — omit declared-window guard | Run 321 |
| 9 — reduce Adler one byte late | Run 450 |
| 10 — omit Adler comparison | Run 318 |
| 11 — Full one byte early / remove limit guard | Runs 323 and 389 |
| 12 — u32 bit position | Run 451, with the qualification below |
| 37 — continue after a dead prefix | Run 373 |
| 39 — discard present stored prefix on truncation | **Missing** |

Mutation 12 is narrower evidence than its informal description can suggest: `/home/ws/projects/verified-decoders/png/inflate_verus/tools/proof_mutations.py:60` narrows the **implementation-side ghost bit-position observation**, not an executable u64 field. Run 451 genuinely fails five proofs, and the independent model is unchanged. It is valid sensitivity evidence for the representation relation, but should not be described as an executed narrowed-counter experiment. A concrete executable narrowing mutant would strengthen that coverage.

Mutation 36, making IgnoreTail discard surplus output, also has no matching recorded kill. `ignore-short-tail` in run 319 is different: it removes the exactly-full condition after checksum validation, not the output limit. Mutation 36 is explicitly assigned to P-T9/P1 at `/home/ws/projects/verified-decoders/png/THEOREMS.md:250`; I have not promoted that future policy mutation into a Strict M2 blocker. The nine requested RB witness gates are nevertheless proved now. Mutations 13–35, 38 and 40 concern later PNG/allocation milestones, not this inflate sign-off.

Run 453 was a type-check failure and is correctly excluded from the kill count (`/home/ws/projects/verified-decoders/png/inflate_verus/LEDGER.md:355–356`). Neither it nor a timeout supplies a missing kill.

## Trusted surface and runtime evidence

I found no local `assume`, `admit`, `external_body`, new `assume_specification`, trusted FFI or unsafe production implementation. The only `#[verifier::...]` attribute kinds in source are `spinoff_prover` and two `prophetic` annotations. The former selects solver queries; the latter describes final borrowed referents for framing (`/home/ws/projects/verified-decoders/png/inflate_verus/src/bits.rs:67–69` and `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:180–182`). Neither skips checking a body.

The public decoder call graph is in verified code. Ordinary Rust test code and `Work`'s convenience `Default` implementation do not create an unverified decoding path reachable from these entries. Built-in slice/arithmetic specifications, vstd, Verus, compilation and Rust aliasing remain the declared trusted basis. Absence of allocator symbols and executable recursion is an inspection/build fact, not an extra Verus theorem.

The pinned dependency has `default-features = false` and only `features = ["alloc"]` (`/home/ws/projects/verified-decoders/png/inflate_verus/Cargo.toml:8–9`). This is exactly the E1 exception, not a new std/allocator exemption. Retained runs 568–570 build instantiated host and freestanding entries and report no allocator references; `/home/ws/projects/verified-decoders/png/inflate_verus/evidence/run-0570.txt:1–6` distinguishes the instantiated probes from an uninstantiated generic library.

Retained differential runs 571–572 cover 10,000 generated streams and 10,000 single-bit corruptions, with zero unexpected differences, plus all 1,181 extracted vector streams across the tested layouts. Their output expressly says work is erased and proved. This is runtime evidence for inflate observations, not proof of all 810 PNG outcomes or integrated policy conformance.

## Non-blocking items and scope controls

1. **Add a requirement-indexed mutation table.** The current total of 67 makes the absent mutation 39 easy to miss. Record required mutation numbers, exact mutant, source revision, failed proof and retained log; keep additional useful mutations separate from required coverage.
2. **State the mutation-12 qualification wherever its coverage is summarized.** It currently tests a ghost representation observation. Add an executable narrowing mutant if the project wants the stronger experimental claim.
3. **Export Adler maximality if “every word formally checked” is the intended documentation standard.** Add the inequality at `ADLER_NMAX + 1` and its monotonic extension; the current operational theorem needs only the safe bound and is already proved.
4. **Remove stale status comments.** `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:1334` still calls E2 proposed, and `/home/ws/projects/verified-decoders/png/inflate_verus/src/model.rs:1522` still calls the coding-cost lemma untied. Neither reflects the current proof.
5. **Keep split-coverage checks structural as the crate evolves.** The current check is sufficient for the inspected flat module tree. If nested modules, new cfg expressions, nonstandard Verus wrappers or build-script-controlled source selection are introduced, extend the checker before relying on its existing claim.
6. **Preserve the milestone boundary.** L-SEG-PNG, per-reader L-SEG-FRAME and call-site binding, L-WALK-FIRST, allocation obligations and integrated PNG outcomes remain M3–M6 work. The council exclusion and the 18 standalone policy witnesses do not discharge them.

**Bottom line:** the proof and witness implementation is substantially complete for M2's implemented domain, and I found no decoder-correctness or split-verification defect. M2 as an assurance milestone is not complete until the required stored-prefix-loss mutation has a retained verifier kill. The 615/631 exclusion, ghost-only vstd allocation feature and i686 cache bootstrap are not reasons to withhold sign-off.
