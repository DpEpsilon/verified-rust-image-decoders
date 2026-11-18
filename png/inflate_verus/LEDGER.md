# Inflate proof ledger

## Actual block refinement complete, 2026-11-18

**Fresh final-source run 285: 236 verified, 0 errors.** Steps (2a) and (2b) are complete at the production stored/compressed helpers, dispatch and full DEFLATE block loop, under their explicit well-formed reader/output/table and admitted-window preconditions. This is a clean handoff before step (2c), not completion of layer 3 or a public inflate theorem. No current proof/tooling blocker exists. The zlib envelope and public callers still need to establish those preconditions and compose the local theorems.

| Runs | Evidence | Result |
|---|---|---|
| 259 | Cached baseline after restart | Exit 0, no fresh verifier count |
| 260–261 | Stored alignment, four LEN/NLEN bytes, mathematical complement and payload composition | Run 261: 219 verified, 0 errors |
| 262–269 | Independent match/symbol/block models, arithmetic RFC groups, actual copy and compressed loops | Run 269: 232 verified, 0 errors; finite power/table and window-constant proof fixes, no solver-limit change |
| 270–272 | All-outcome bit-sensitive construction charge, actual block dispatch and MBlk loop | Run 272: 235 verified, 0 errors |
| 273–274 | Actual declared-window versus Ignore selection helper | Run 274: 236 verified, 0 errors |
| 275–278 | Formatting, thirteen release tests, adapter and cached formatted replay | Runtime gates pass; run 278 has no fresh verifier count |
| 279–280 | Full vector layouts, 10,000 generated streams plus 10,000 single-bit corruptions, seeds 20 and 21 | 33,544 public-entry pairs; zero unexpected differences |
| 281–284 | Host/thumbv7m libraries and instantiated allocator probes, source/cap controls and frozen hashes | Passed; twelve frozen entries |
| 285 | Fresh final-source proof replay after an erased block-origin bound assertion | 236 verified, 0 errors |

### Exact proved surface

- `/home/ws/projects/verified-decoders/png/inflate_verus/src/model.rs:1539` MStoredBlock independently models alignment, complete four-byte header availability, mathematical little-endian words, the complement as LEN+NLEN=u16::MAX, and ordered MStoredPayload continuation. The executable/model observation relation now maps every independent InflateFault label, not only stored truncation.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:348` stored_block is the actual production alignment/four-read/complement path extracted from run. It proves exact MStoredBlock error/output/position effects and calls the already-verified payload. stored_payload (:287) now also exports monotone bounded positions, monotone produced and work <=2*(newly_produced+1), while retaining its <=2*count bound and exact D5 refinement. This discharges the stored-header/payload part of T-INFL/P10.2 and local L-LIMIT, not a public-entry theorem.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/model.rs:1555–1572` length_extra/length_base/distance_extra/distance_base define RFC symbol groups arithmetically, without importing executable arrays. lemma_length_tables at `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:28` and lemma_distance_tables (:60) prove all executable entries equal these independent functions and bound decoded lengths by LEN_MAX and distances by WINDOW_MAX.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/model.rs:1584` MMatch, MCompressedStep (:1620), MCompressed (:1640), MOneBlock (:1658) and MBlk (:1679) independently define all symbol and block control flow. They use canonical-code MDec, mathematical fields, MEmit/MCopy and evolving Seq<u8> output only. No executable tables, buffers or ring_repr occur in these models. Their recursive decreases checks are verified; unreachable non-progressing continuations return OutsideDomain.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:838` match_copy is the real distance guards and overlapping-copy loop. Its exact MCopy refinement rejects distance beyond produced or the selected window before the first append and before a Full decision; it resolves each next byte against the evolving output through previous and append_byte, preserving output/Adler/slice-or-ring invariants. Its exact success work is length and every outcome costs at most newly_produced+1. This establishes the match/overlap/distance-check parts of L-WINDOW and preserves the local L-RING representation; public sink identity/original-slice framing remains open.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:901` compressed_step is the actual symbol iteration. It refines MCompressedStep on every outcome, rejects literal/length 286/287 before nonexistent extras, rejects distance 30/31 immediately after decoding, proves bases/extras and calls match_copy. compressed (:947) composes these actual iterations against MCompressed with exact output/status/error-position effects and a strictly decreasing remaining-bit variant. Every successful iteration consumes at least one bit; copy writes remain bounded by LEN_MAX. These are local T1/T11 and L-LIMIT obligations, not global public T1/T11.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:998` one_block composes the real read_block_header, stored path and fixed/dynamic compressed paths against MOneBlock, including BTYPE 3 and dynamic-header errors. blocks (:1038) is the actual full BFINAL loop against MBlk, now called by run. Both preserve the reader, output and table invariants on every outcome and prove success advances by at least three bits. This completes block dispatch and DEFLATE-loop refinement, including non-final empty stored blocks.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:1085` selected_window is the actual production policy helper called by run after its existing CINFO guard. For cinfo<=7 it equals independent model::window_size (:1695): pow2(cinfo+8) under Enforce and WINDOW_MAX under Ignore, always in 1..WINDOW_MAX. The still-unverified run must establish the helper's CINFO precondition; no complete zlib-policy/public theorem is claimed.
- Construction/read/decode charges remain disjoint. dynamic_lengths (:586) bounds construction by 3*DYNAMIC_LENGTHS if it never reaches literal construction; dynamic (:720) and read_block_header (:763) now prove construction <=48*consumed_bits on every outcome while retaining the separate <2,048 budget and L-HDR-BITS. one_block and blocks prove observed work delta <=64*(consumed_bits+1)+2*newly_produced on every outcome, without the +1 on success. The one final unsuccessful attempt gets the additive allowance only once across blocks. These local Ghost bounds do not yet discharge public T12.

### Exact resume and remaining boundaries

1. Start step (2c) at `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:1105`, run. The real zlib CMF/FLG reads/check order, caller-side window selection, table construction, blocks call, checksum alignment/four-byte BE32, Adler comparison and exactly-full IgnoreTail condition remain ordinary unverified Rust. Add an independent Inflate envelope around MBlk; preserve first-error order and exact output prefixes on every outcome. The complete block loop and its preconditions are now available, rather than needing another block-model foundation.
2. Compose entry (:1148), inflate_zlib_into (:1166) and inflate_zlib_measure (:1178) against that one Inflate model. Preserve public source freshness as exactly wf plus pos()==0, with no index/counter reset antecedent. Export sink identity and the constructor's original snapshot to prove the public untouched suffix relative to the caller's original slice. Block-level output/ring preservation is not yet complete public L-WINDOW/L-RING or T-INFL.
3. Replace the historical public runtime-u128 diagnostics with returned Ghost<nat> core_steps/seg_steps and prove per-call deltas, not absolute segment counters. Work and SegmentSlices still retain their existing runtime diagnostics; this continuation added no runtime counters. Use limits.rs caps_ok as required by the manifest when closing global arithmetic/work. Tables::new currently exports <=C_INFL_FIXED; its proved internal uniform expression is 5*HUFF_INIT_STEPS+2*FIXED_LIT_COUNT+2*HUFF_BUILD_STEPS+FIXED_DIST_COUNT. A tighter constructor bound may be useful to leave room for envelope/final-attempt charges. Global public T1/T11/T12 remain open.
4. Authored model-vector witnesses and proof mutations remain open. Preserve all previous reader/Huffman/dynamic mutation lists, and add stored NLEN omission, wrong length/distance bases/extras, reserved-symbol admission, distance/output/window guard removal, non-overlapping or wrong-offset copy, Full at L rather than L+1, wrong BTYPE dispatch/BFINAL continuation, and missing/misclassified block/symbol/copy work charges. No runtime expected-error checks count as model witness proofs.
5. The pinned matching i686 vstd-cache verifier replay remains open. Do not substitute the ordinary thumbv7m build or change features/--no-cheating. No target directory was deleted, cleaned or repaired. Next logged run is 286.

Runs 279–280 exercise all 1,181 extracted streams in four layouts per shard and together 10,000 generated streams plus 10,000 corruptions: 33,544 public-entry pairs or 67,088 actual public calls, including empty Z. Each pair checks status/count, written prefix, untouched suffix, ring and observed diagnostic work. The thirteen release tests and both instantiated allocator-symbol gates pass on the final runtime bodies. The final run 285 changed only an erased proof assertion after these runtime gates.

No frozen counterexample/new erratum, forbidden proof escape, trusted item, solver-limit increase, frozen/manifest/errata edit or commit was introduced. All jobs have exited; the combined differential command was automatically backgrounded by the tool but completed with exit 0, and no background process remains. E1 remains coordinator-owned. Verus 0.2026.11.02.8bf68ee, vstd 0.0.0-2026-11-01-0251, rustc 1.99.1 (58cf8f9c3), default solver limits, CARGO_BUILD_JOBS=1, two verifier threads and --no-cheating are unchanged. Backups, each below 1 MB: `/home/ws/projects/verified-decoders/png/inflate_verus/backups/stored-header-before-layer3.tar`, `/home/ws/projects/verified-decoders/png/inflate_verus/backups/compressed-before-layer3.tar` and `/home/ws/projects/verified-decoders/png/inflate_verus/backups/block-loop-before-window-helper.tar`.

---

## Actual dynamic-header refinement complete, 2026-11-17

**Fresh final-source run 252: 215 verified, 0 errors.** Assigned step (1) is complete at the actual production parser and block-header reader. This is not completion of layer 3 or a public inflate theorem. No current tooling/proof blocker exists. Resume with step (2), the stored LEN/NLEN check and compressed/window/copy/block composition.

| Runs | Evidence | Result |
|---|---|---|
| 239–245 | Production parser conversion, explicit continuation snapshots, consumed-bit-sensitive helper costs and repeat-loop extraction | Typing/invariant fixes; removed unused-hide AIR failures; no solver-limit change |
| 246 | Actual Tables::dynamic against MDynamic, separate construction budget | 214 verified, 0 errors |
| 247–248 | Literal-construction reach marker and actual three-bit block-header reads | Run 248: 215 verified, 0 errors |
| 249–252 | Formatting, thirteen release tests, adapter and fresh formatted-source proof replay | Run 252: 215 verified, 0 errors |
| 253–254 | All vector layouts, 10,000 generated streams plus 10,000 single-bit corruptions, seeds 18 and 19 | 33,544 public-entry pairs; zero unexpected differences |
| 255–258 | Host/thumbv7m libraries and instantiated allocator probes, symbol/source/cap controls and frozen hashes | Passed; twelve frozen entries |

### Exact proved surface

- `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:32` lemma_cl_order proves the executable CL_ORDER equals the independent model::code_length_order permutation at every index and stays below NCL.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:358` dynamic_code is the actual CL-read/permutation loop and code-length-table construction. Its loop maintains equality to MDynamicCL; success continues at MDynamicLengths with the exact admitted code and position, and failure has the exact model error/position. Its returned Ghost construction observation excludes Bits::get work.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:425` write_length_repeat is the actual repeat-fill loop, extracted from dynamic parsing. It proves exact mathematical repeated-prefix extension, preserved length_entries_ok, array safety, termination and exactly count observed loop iterations.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:455` Tables::dynamic_lengths is the actual expansion loop, EOB check and ordered literal/distance builds. It maintains equality to MDynamicLengths and proves exact output-table profiles or model error/position through dynamic_eq (:347). The combined sequence uses length_entries_ok, not the canonical 288-entry lengths_ok; total <= DYNAMIC_LENGTHS and every actual write is bounded. Repeat-16 checks the previous entry before reading extras, overflow checks precede repeat writes, and EOB precedes literal admission then distance admission.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:588` Tables::dynamic composes all three count reads before count admission, then the two verified production helpers, and proves exact MDynamic refinement on every outcome. It preserves both fixed tables and all Huffman well-formedness requirements. The actual run calls this parser through read_block_header, not a model-mirror implementation.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:630` read_block_header performs the actual BFINAL/BTYPE reads, rejects type 3 and invokes dynamic parsing for type 2. Its contract relates these fields to independent field_value and MDynamic, and proves L-HDR-BITS at the real literal-construction boundary: header_table_start is set immediately before self.lit.build (:577–579), and whenever set it equals the endpoint at least 45 bits after the block start, including when either final table build fails. The expansion loop tracks symbol-18 counts/entries/bits separately from other entries/bits and establishes lemma_header_coding_cost's premises at :572; counts + CL fields + expansion account for 14 + at least 12 + at least 16 bits, plus the actual three block bits.
- Construction work is separate from reader/decode work throughout. The CL helper bounds construction by 139 iterations and the expansion/finalization helper by 1,706, giving at most 1,845 construction iterations. Tables::dynamic and read_block_header export header_construction < 2,048 and actual observed work delta <= header_construction + 3*(consumed_bits+1). This is a local cost theorem, not global T12; the additive allowance covers a failed input attempt.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/bits.rs:120` get now exports monotonic bounded positions, value < pow2(n), consumed-bit-sensitive work <= 2*(consumed+1), and <= 2*consumed on success. `/home/ws/projects/verified-decoders/png/inflate_verus/src/huffman.rs:165` decode now exports endpoint bounds, successful symbol < profile length, a positive successful bit count, work <= 3*(consumed+1), and <= 3*consumed on success. Existing exact refinement contracts remain intact; these changes do not alter runtime bodies.

### Exact resume and remaining boundaries

1. Continue step (2) at `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:745–755`: alignment, four stored LEN/NLEN bytes, complement check and composition with the already-verified stored_payload. read_block_header already handles the actual three-bit reads/type-3 rejection/dynamic-table selection; it does not prove any block payload or full dispatch simulation.
2. Verify compressed at `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:674`, including reserved literal/distance symbols, decoded length/distance bases/extras, declared-window versus Ignore, DistanceTooFar before emission and the actual overlapping-copy loop against MCopy. Extend the independent model to full MBlk/control flow; keep ring_repr strictly executable/model representation, not part of that model.
3. Compose run at `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:717` and both public entries (:808, :820). Full L-WINDOW/L-RING, T-INFL, global T1/T11/T12, caller-original-slice framing and public Ghost result accounting remain open. Public freshness must remain exactly wf plus pos()==0, without index/counter resets.
4. No authored model-vector witnesses or proof mutations ran. Add the dynamic-specific mutations later: count admission before the third field; CL permutation swap; repeat-16 extra read before the previous-entry check; repeat overflow check removal; EOB/admission order changes; wrong repeat fill; and omission/misclassification of a construction/read/decode work charge. Existing earlier mutation lists still apply.
5. The 32-bit verifier replay remains open. Plan: supply an actually matching pinned i686 vstd verification cache without changing features, then run CARGO_BUILD_TARGET=i686-unknown-linux-gnu through the same bounded serial verify.sh and record a fresh count; do not substitute the thumbv7m ordinary build for that replay or weaken --no-cheating if dependency bootstrap fails.

Runs 253–254 compare all 1,181 extracted streams in four layouts and together check 10,000 generated streams/corruptions: 33,544 public-entry pairs or 67,088 actual public calls, including empty Z. Those are runtime observations, not authored model-versus-expected witness proofs. The thirteen release tests and both target allocator-symbol gates pass on final runtime source.

No frozen counterexample/new erratum, forbidden proof escape, trusted item, solver-limit increase, frozen/manifest/errata edit, commit or background job was introduced. E1 remains coordinator-owned. The accepted verifier/vstd/rustc versions and ghost-vstd/alloc dependency boundary are unchanged. Backup before conversion: `/home/ws/projects/verified-decoders/png/inflate_verus/backups/dynamic-parser-before-layer3.tar`. Scoped hides of Huffman::repr/wf caused the known unused-fuel AIR bug in runs 240/244 and were removed; production repeat-loop extraction plus spinoff_prover resolved the expansion query's resource pressure without changing limits.

---

## Dynamic-header model foundation within layer 3, 2026-11-17

**Fresh run 231: 205 verified, 0 errors.** Low-context handback, not completion of layer 3. The independent ordered dynamic-header model and three supporting lemmas are checked; the actual Tables::dynamic remains ordinary unverified Rust. No runtime body changed after the accepted 200-verified checkpoint. No current tooling/proof blocker exists.

| Runs | Evidence | Result |
|---|---|---|
| 231 | Independent dynamic-header model, supporting lemmas and length-dependent Huffman work clauses | Fresh 205 verified, 0 errors |
| 232–234 | Thirteen release tests, adapter, all vector layouts plus 1,000 generated streams and 1,000 corruptions | Passed; 8,772 public-entry pairs, zero unexpected differences |
| 235–238 | Host/thumbv7m libraries and instantiated allocator probes, source/cap controls and frozen hashes | Passed; twelve frozen entries |

### Exact new surface

- `/home/ws/projects/verified-decoders/png/inflate_verus/src/model.rs:1379` DynamicHeader, MDynamic at line 1479, MDynamicCL at line 1459 and MDynamicLengths at line 1414 independently describe ordered count fields, the RFC code-length permutation, code-length decoding and repeat expansion, final EOB presence, literal admission and distance admission. They contain only mathematical length sequences and bit positions, never executable tables or ring state. Their recursive decreases clauses are checked; no actual parser-refinement theorem is claimed.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/model.rs:1393` length_entries_ok checks entry values without canonical lengths_ok's 288-entry bound. The combined dynamic profile can contain 316 entries and must use this separate predicate.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/model.rs:1496` lemma_dynamic_count_fields proves the three wire-field ranges. lemma_finish_dynamic at line 1508 proves successful finalization's exact split, EOB presence, admissions, unchanged endpoint and DYNAMIC_LENGTHS bound. These are model-local facts, not yet bounds on the actual expansion loop.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/model.rs:1524` lemma_header_coding_cost proves at least sixteen length-code bits from explicit aggregate repeat/entry/bit inequalities. It is only an arithmetic ingredient: the actual loop does not yet establish those inequalities, and L-HDR-BITS remains open.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/huffman.rs:55` build, line 70 build_with_single and line 114 build_canonical now export the upper bound `work_delta <= CANON_FIXED_STEPS + 2*lengths.len() + MAX_CODE_BITS + 1` in addition to their uniform bound. This clause is an upper bound on every outcome, not an exported equality.

### Exact resume and remaining boundaries

1. Verify Tables::dynamic at `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:324` against MDynamic. Expose the actual CL_ORDER to Verus and prove its correspondence to the independent permutation; maintain CL-loop and expansion-loop equalities to the respective model continuations. Keep all count fields before count admission, repeat-16's previous-entry check before extras, and EOB presence before literal then distance admission.
2. Prove actual 316-entry safety, consumed-bit accounting for L-HDR-BITS and a separate construction-only work bound. Track symbol-18 counts/entries/bits separately from other entries/bits to discharge lemma_header_coding_cost. Reader and Huffman-decode work must not be silently included in the 2,048-iteration construction budget.
3. Complete stored header/LEN-NLEN/dispatch, reserved-symbol rejection, distance/window checks and the real overlapping-copy loop against MCopy/MBlk; then compose the outer run. Full MBlk/Inflate models, complete L-WINDOW/L-RING and public-entry refinement remain open.
4. Preserve public source freshness as wf plus pos()==0; public sink identity/initial-snapshot framing, Ghost result accounting, global safety/work, 32-bit verifier replay, authored model-vector proofs and proof mutations remain open.

Runs 225–226 remain the latest full 10,000-stream/10,000-corruption differential on the unchanged runtime bodies. Runs 232–238 check the latest spec-only source changes. No forbidden proof escape, trusted item, solver-limit change, frozen/manifest/errata edit, commit or background job was introduced. E1 remains coordinator-owned. Backup: `/home/ws/projects/verified-decoders/png/inflate_verus/backups/dynamic-model-before-layer3.tar`.

---

## Historical stored/output/Adler/fixed checkpoint within layer 3, 2026-11-17

**Fresh run 222: 200 verified, 0 errors.** Low-context handback within layer 3, with no current tooling/proof blocker. The actual Adler methods, output append/previous methods, stored-payload loop and fixed-table constructor are verified. Dynamic-header parsing, full block/compressed-match simulation, L-HDR-BITS/construction budget and complete L-WINDOW/L-RING remain open. There is still no complete MBlk/Inflate model or public T-INFL theorem.

| Runs | Evidence | Result |
|---|---|---|
| 196–198 | Actual deferred Adler update/finish against independent mathematical sums | Natural modulo casts and explicit nonlinear local definitions; 184 verified |
| 199–205 | Adler release/vector/differential/build/symbol/source/hash checks | Passed |
| 206–211 | Actual output constructor/append/previous, ring-append representation and suffix preservation | Mutable-sink borrowing, suffix composition and explicit sink-size contract fixes; 191 verified |
| 212 | Actual production stored-payload loop against MStoredPayload | 195 verified |
| 213–219 | Stored/output release/vector/differential/build/symbol/source/hash checks | Passed |
| 220 | Independent fixed profiles, actual Tables::new and fixed initialization budget | 200 verified |
| 221–224 | Formatting, fresh proof replay, thirteen release tests and adapter | Run 222: 200 verified, 0 errors |
| 225–226 | Final-source full differential, seeds 15 and 16 | 10,000 generated streams plus 10,000 corruptions; zero unexpected differences |
| 227–230 | Final host/thumbv7m libraries/instantiated probes, allocator/source/cap controls and frozen hashes | Passed; twelve frozen entries |

### Exact proved surface

- `/home/ws/projects/verified-decoders/png/inflate_verus/src/model.rs:1279` sum_bytes and line 1286 sum_prefixes independently define mathematical single and prefix sums, without deferred accumulators. Adler at line 1293 encodes their modular checksum; lemma_adler_sums_push at line 1298 proves the append recurrence.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/adler.rs:76` Adler::new, line 83 update_byte and line 113 finish are the actual production methods. They prove safety, mathematical checksum equivalence and exact observed byte-sequence extension, including deferred reduction at ADLER_NMAX. adler_format_ok at line 8, lemma_bound_step at line 19 and lemma_mod_add_equal at line 50 provide the bounds/congruence proof. No call to the old unverified update is trusted.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:131` Output::append_byte refines MEmit while preserving produced/output/Adler consistency, the caller-slice prefix and the untouched suffix relative to its constructor snapshot, or the last-window ring representation. Previous at line 170 returns exactly the byte at model-output index produced-distance under the explicit valid-distance precondition. New at line 110 initializes the observation with arbitrary caller-owned contents; its sink-size contract matches the existing into/ring requirements.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:37` ring_repr is a representation predicate, never part of an independent model definition. lemma_ring_append at line 61 preserves every readable last-window byte; lemma_recent_distinct at line 43 proves the update slot cannot collide with a retained byte. lemma_skip_later at line 51 proves the suffix projection needed for untouched output.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:193` stored_payload is the actual loop called by run at line 462. It proves exact MStoredPayload output/status/bit-position effects after alignment and validated LEN/NLEN, D5 availability-before-limit, preserved output invariants and a measured Ghost work delta <=2*count. stored_eq at line 182 states the executable/model observation relation. Bits::align now explicitly exports remaining==0.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/model.rs:1322` fixed_lit_lengths and line 1327 fixed_dist_lengths are independent RFC profiles. lemma_fixed_lit_mass at line 1341, lemma_fixed_dist_mass at line 1352 and lemma_fixed_codes at line 1361 prove exact complete Kraft mass/admissibility, under either single-code choice. The distance profile has FIXED_DIST_COUNT entries, including symbols 30 and 31.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:265` Tables::new is the actual constructor. It builds those exact profiles through the accepted verified Huffman constructor, establishes fixed_wf at line 260, always succeeds and charges at most C_INFL_FIXED exact Ghost work, including the three initially empty dynamic Huffman states. The general Huffman build contracts now export their already-proved Ghost work bounds.

### Exact resume and remaining boundaries

1. Continue with Tables::dynamic at `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:324`: independently model ordered count/length/repeat parsing; prove admission/error positions, the 316-entry bound, L-HDR-BITS and the 2,048-iteration construction budget. The current Huffman constructor exports a uniform bound; strengthen its success work clause to the existing exact length-dependent observation when needed for the tighter header budget.
2. Complete stored header/dispatch and the compressed symbol/match path. run at line 424 still contains unverified zlib/block headers and the LEN/NLEN check; compressed at line 381 is unverified. In particular the post-decode errors for fixed reserved symbols and the actual distance/window checks are not yet proved, despite the fixed tables now being verified.
3. Compose the existing output/copy/ring primitives against MCopy through the real overlapping-copy loop; do not cite primitive ring preservation as full L-WINDOW/L-RING or an inflate theorem. Extend the independent model with full MBlk/control flow and eventually Inflate, never by importing executable transitions/tables/rings.
4. When proving public entries, preserve wf plus pos()==0 and the existing sink-length contracts, with no reset/index assumptions. Export the constructor's sink identity/initial snapshot as needed to tie the local untouched-suffix invariant to the caller's original slice; the current new contract establishes local wf, not that complete public borrowing/framing theorem.

Public T-INFL, global T1/T11/T12, 32-bit verifier replay, public Ghost result counters, authored model-vector proofs and proof mutations remain open. L-ADLER is proved at the actual accumulator interface; checksum enforcement in the unverified outer run is not yet a public-entry theorem. The latest full differential checks 33,544 public-entry pairs (67,088 actual calls), all vector layouts and 10,000 generated streams/corruptions; thirteen release tests and both allocator-symbol probes pass.

No new trusted item, forbidden proof escape, solver-limit change, frozen/manifest/errata edit, commit or background job was introduced. E1 remains coordinator-owned. Backups for these moves are `/home/ws/projects/verified-decoders/png/inflate_verus/backups/adler-before-layer3.tar`, `/home/ws/projects/verified-decoders/png/inflate_verus/backups/output-before-stored-layer3.tar` and `/home/ws/projects/verified-decoders/png/inflate_verus/backups/tables-before-fixed-layer3.tar`. Accepted verifier/dependency versions and bounded serial settings remain unchanged.

---

## Independent output-model foundation within layer 3, 2026-11-17

**Fresh run 193: 170 verified, 0 errors.** This is a low-context checkpoint inside layer 3. Only ghost/model code changed after the accepted layer-2 commit 4510229. The independent evolving output and stored-payload semantics are defined and their foundational lemmas are proved. This is not a full MBlk/Inflate model, executable stored-block refinement, L-WINDOW/L-RING or completion of layer 3. No runtime path was replaced or changed.

| Runs | Evidence | Result |
|---|---|---|
| 184 | Initial independent sequence-output/copy/stored definitions | Required explicit triggers for overlap indices |
| 185 | Evolving sequence copy and exact D5 stored-payload theorem | 170 verified, 0 errors |
| 186–192 | Thirteen release tests, all vector layouts plus 1,000 generated streams/corruptions, host/thumb probes, source/cap controls and hashes | Passed; 8,772 public-entry pairs, twelve frozen hashes |
| 193 | Separate mathematical InflateFault labels, no executable error datatype in model | Fresh 170 verified, 0 errors |
| 194–195 | Final source/hash controls and thirteen release tests | Passed |

New definitions and proofs in `/home/ws/projects/verified-decoders/png/inflate_verus/src/model.rs`:

- `InflateFault` at line 1078 supplies the model's own error labels; `OutputEffect` at line 1092 distinguishes Next, Full, Error and OutsideDomain.
- `MEmit` at line 1108 appends to Seq<u8> only below L; reaching L does not itself terminate a stream.
- `copy_distance` at line 1116 computes every next match byte from the sequence after earlier match bytes were appended; it never reads the executable ring/tables. `lemma_copy_distance` at line 1167 proves exact length, old-prefix preservation and extended[out.len()+j] == extended[out.len()+j-distance], including overlap. `lemma_copy_prefix` at line 1195 proves every intermediate copy is the corresponding prefix of a longer copy.
- `MCopy` at line 1126 checks distance against output/window before emission, appends what fits and returns Full only when another byte exists. `lemma_MCopy_shape` at line 1207 proves its exact Next/Full/DistanceTooFar shape under the explicit decoded-match domain.
- `MStoredPayload` at line 1151 models an aligned payload after LEN/NLEN validation, taking bytes in stream order and testing availability before the output limit. `lemma_stored_payload` at line 1240 proves the exact output sequence and End/Full/TruncatedZlib outcome: Full requires a present byte beyond the limit; truncation retains all present bytes that fit; an exactly filled payload can return End. Positions on End/Error are nat bit positions. `lemma_push_subrange` at line 1230 supplies the sequence identity used by the induction.

The copy domain explicitly requires a positive distance, a decoded length in 3..LEN_MAX, a valid positive window no larger than WINDOW_MAX, and an existing output of at most L bytes. Outside-domain arguments receive the internal marker, not a new format error. Invalid admissible-domain distances still return DistanceTooFar before the limit outcome. Stored-payload domain requires cursor<=|Z|, existing output<=L and the wire LEN bound. The full block parser and its guards are not yet transcribed.

Exact resume:

1. Extend this foundation to the independent fixed/dynamic-header and block-loop models; do not define them by executable transitions, tables or buffers.
2. Convert the actual stored path in `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:229` against MStoredPayload and the actual output append operation at line 42 against MEmit. The sink/Adler helper contracts remain unverified; do not treat Adler.update_byte as trusted while composing append.
3. Prove fixed table construction at line 70 with all 32 distance codes, dynamic header at line 102 with ordered admission/checks, 316 entries and L-HDR-BITS/construction budget, then match/window/ring/overlap representation through the actual compressed loop at line 159.
4. Public entry contracts must retain source wf plus pos()==0, without index/counter-reset assumptions. Public T-INFL, global safety/work, 32-bit replay, authored model-vector witnesses and proof mutations remain open.

The previous layer-2 source, full 10,000-stream differential and proof inventory below remain accepted. No executable code changed in this continuation. No new trusted item, forbidden proof escape, solver-limit increase, frozen/manifest/errata edit, commit or background job was introduced. E1 remains coordinator-owned. Backup: `/home/ws/projects/verified-decoders/png/inflate_verus/backups/output-model-before-layer3.tar`. The verifier/vstd/rustc versions and bounded serial settings remain as recorded below.

---

## Completed reader/Huffman layer, 2026-11-17

**Fresh run 176: 163 verified, 0 errors.** Assigned layer 2 is complete at the actual Segments/Bits/Huffman methods: L-BITS, outer canonical construction/admission and decode/MDec refinement are proved. T-HUFF(a–c) and L-CANON are established on this verified host target through those real methods, not a separate decoder. A 32-bit verifier replay and the manifest's authored-witness/mutation gates remain open. This is not T-INFL, global T1/T11/T12, full container T-SEG, allocation sign-off or a release declaration. Resume with layer 3, blocks/window and the independent append-only inflate model.

| Runs | Evidence | Result |
|---|---|---|
| 132–135 | Verified Work interface and erased observation | Natural cast/visibility fixes, then 133 verified |
| 136–140 | Actual Bits cache/read/align contracts and independent field_value | Position cast, mutable-entry snapshots, cache visibility/refill and generic-wf-law fixes |
| 141 | Bits and generic Segments wf law | 147 verified |
| 142–148 | Reader release/differential/allocator/source/hash checks | Passed |
| 149 | Actual outer Huffman construction/admission/error mapping; Ghost live-count lengths | 152 verified |
| 150–156 | Constructor release/differential/allocator/source/hash checks | Passed |
| 157–159 | Actual decoder refinement | Default resource-limit failure, then scoped abstraction/projection helpers and exact EOF-position fix; no limit increase |
| 160 | Actual decode equals MDec, symbol/error and position | 160 verified |
| 161–162 | Both single-code choices, outer L-CANON bridges and construction work composition | 163 verified |
| 163–164 | Exact Ghost work, total legacy diagnostic update, no work-slack preconditions | 163 verified |
| 165–168 | Formatting, fresh proof replay, thirteen release tests and adapter | Run 166: 163 verified, 0 errors |
| 169–170 | Final-source full differential, seeds 9 and 10 | 10,000 generated streams plus 10,000 corruptions, zero unexpected differences |
| 171–174 | Host/thumbv7m libraries and instantiated probes, allocator/source/cap controls and hashes | Passed; twelve frozen entries |
| 175–176 | Proof-only stable-cache invariant and exact next-byte alignment | Exposed pre-refill cache facts, then 163 verified, 0 errors |
| 177–183 | Final-source thirteen release tests, all vectors plus 1,000 streams/corruptions, both target gates and frozen hashes | Passed; no runtime-body change after the full differential |

### Exact proved surface

- `/home/ws/projects/verified-decoders/png/inflate_verus/src/segments.rs:54` adds the proved trait law lemma_wf. An overridable default spec body alone does not expose its facts to generic consumers; SegmentSlices proves the law at line 155. The actual constructor/read methods remain verified at lines 105 and 157. No initial index/counter reset is required by read.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/bits.rs:77` Bits::new requires only source wf and snapshots ghost work/rank origins. Its byte at line 90, get at line 120 and align at line 210 prove exact bytes/bits, preserved concat, bit-position range, exact rounding to the next byte boundary, partial consumption on truncation and termination. Stable wf excludes the transient full-byte refill state; that state uses cache_wf only inside get. get covers n<=32 and returns model::field_value; bit position is a nat, 8*src.pos()-remaining. Measured segment work is a delta bounded by C_SEG*(seg_count+1), not an absolute counter requiring reset. get's exact Ghost work delta is at most 2*n.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/lib.rs:116` Work::new and line 121 Work::add maintain an exact erased Ghost<nat> observation. The legacy u128 diagnostic clamps at its type maximum; no helper now requires spare diagnostic capacity. The diagnostic clamp does not clamp the mathematical counter.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/huffman.rs:110` build_canonical composes the actual canonical constructor and live loop, maps every construction error to InvalidCodeLengths, and establishes the full representation on every non-oversubscribed input, including otherwise inadmissible vectors. It asserts the work observation equals the actual helper observations. `/home/ws/projects/verified-decoders/png/inflate_verus/src/canonical.rs:312` live_counts now takes Ghost lengths, with no executable observation borrow.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/huffman.rs:68` build_with_single proves exact admission under both single-code choices; the actual build at line 55 delegates to it with true. Successful construction establishes ready, representation and the matching independent admissibility predicate; every failure clears ready. new at line 95 is verified.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/huffman.rs:36` lemma_canon_forward and line 45 lemma_canon_reverse expose the outer L-CANON word/lookup equivalence without constraining unused symbol slots.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/huffman.rs:159` decode proves exact MDec symbol/error and bit position on the real Bits reader. Its invariant tracks the reachable independent prefix, no exact hit at the pending read, numerical msb_bits correspondence and model recursion equality. Empty codes fail without reading, dead prefixes fail before another read, and truncation occurs only on a live prefix. The actual decoder terminates and adds at most 3*MAX_CODE_BITS exact Ghost work.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/model.rs:1004` field_value independently sums wire bits; lemma_field_one at line 1019, lemma_msb_push at line 1025, lemma_initial_prefix at line 1033, lemma_decode_prefix_step at line 1056 and lemma_decode_prefix_hit at line 1065 supply the decoder bridge. The model never reads the concrete cache or tables.

The final decoder uses code*2+bit and the bit reader uses addition of a disjoint bit weight; the independent specifications and runtime differential cover those plain-code changes. Existing public option types remain the Strict/RecommendedBeta domain; the both-choice primitive does not claim implementation of all Stage-2 inflate tuples.

### Resume and remaining boundaries

1. Add an independent evolving inflate model in `/home/ws/projects/verified-decoders/png/inflate_verus/src/model.rs`: append decoded bytes to Seq<u8>, resolving each match distance within that evolving output, never by reading executable ring/table state.
2. Verify the actual stored/fixed/dynamic block paths in `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs`: D5 stream-order stored prefixes and availability-before-limit, all 32 fixed distance codes, dynamic header ordering/admission, the 316-entry bound and L-HDR-BITS/2,048-iteration budget.
3. Prove window/ring representation and overlapping copies, then both actual public entries against the independent Inflate model, including status/count/written prefix, limit/Adler/window rules. Discharge the actual inflate callers' helper preconditions; do not equate the proved Huffman layer with public T-INFL.
4. Public results remain runtime u128 diagnostics; move returned observations to Ghost counters and prove global T12 later. The historical public seg_steps field still reports an absolute diagnostic, so public accounting must return a per-call ghost delta under wf plus pos()==0, without reset/index assumptions.

No proof mutations or authored frozen-model witness computations were run. Pending mutations include cache bit order/advance, partial-EOF consumption, canonical/single admission, failed-lookup/dead-prefix boundaries, reading before dead-prefix rejection, plus the later block/window/limit/Adler/work mutations. Runtime authored expectations remain runtime evidence only.

Runs 169–170 each check all 1,181 extracted streams in four layouts and deterministic stress cases, and together check 10,000 generated streams and 10,000 single-bit corruptions: 33,544 public-entry pairs, 67,088 actual entry calls, zero unexpected differences. The final release suite has thirteen tests. All final build/symbol/source/cap/frozen gates pass.

Verus 0.2026.11.02.8bf68ee, vstd 0.0.0-2026-11-01-0251 and rustc 1.99.1 (58cf8f9c3), default solver limits, two verifier threads, CARGO_BUILD_JOBS=1, --no-cheating and bounded serial verify.sh remain unchanged. No new trust item, forbidden proof escape, frozen/manifest/errata edit, commit or background job was introduced. E1 remains coordinator-owned. Backups before these moves are `/home/ws/projects/verified-decoders/png/inflate_verus/backups/bits-before-layer2.tar` and `/home/ws/projects/verified-decoders/png/inflate_verus/backups/huffman-outer-before-layer2.tar`.

---

## Segments checkpoint within layer 2, 2026-11-17

**Run 124: 130 verified, 0 errors.** The actual Segments trait and SegmentSlices constructor/read loop now have verified contracts. This is a low-context checkpoint inside layer 2, not completion of L-BITS, T-HUFF or T-INFL. Resume with the unchanged Bits reader.

| Runs | Evidence | Result |
|---|---|---|
| 110–112 | Initial trait/read conversion | Ghost natural field initialization errors; replaced raw ghost fields with Ghost<nat> wrappers |
| 113 | Generic trait reveal attempt | Verus internal AIR error, no completed count; removed generic reveals without changing verifier settings |
| 114–115 | Read-loop entry snapshot and progress | Explicit initial == *old(self) invariant, then 126 verified |
| 116 | Universal wf bounds and stable repeated EOF | 126 verified |
| 117–123 | Release tests, vector/differential regression, host/thumb probes, source controls and hashes | Passed |
| 124 | Concrete diagnostic getters tied to spec views | 130 verified, 0 errors |
| 125–131 | Final release/differential/build/symbol/source/hash gates | Passed; 8,772 public-entry pairs in run 127, twelve frozen hashes in run 131 |

`/home/ws/projects/verified-decoders/png/inflate_verus/src/segments.rs:35` is the actual trait. Its spec views are concat, pos, seg_index, seg_count, seg_steps and core_steps; wf at line 44 includes the universal cursor/length/progress bounds plus an implementation-specific state_wf. The read contract at line 52 requires only wf, preserves concat and segment count, returns the exact next concatenated byte or EOF, keeps positions/ranks monotonic, and charges exactly one further core iteration. The segment counter delta equals the advance-rank delta; rank is bounded by seg_count + 1, so costs telescope across reads without resetting counters or assuming initial segment index zero. A None return exhausts the rank, making subsequent EOF reads preserve both position and segment work.

`SegmentSlices::new` at `/home/ws/projects/verified-decoders/png/inflate_verus/src/segments.rs:95` proves wf, pos()==0, exact concatenation, segment count and zero initial ghost counters, and rejects a concatenation beyond SEGMENT_BYTES_MAX. `Segments::read` for SegmentSlices at line 145 is the actual loop: every segment advance increments the erased seg_work counter, each returning iteration increments read_work, and the remaining segment count is its termination variant. The runtime advances diagnostic remains independently tied to seg_work. The four concrete diagnostics at lines 187–199 are also verified.

`lemma_parts_monotone` at line 15 and `lemma_parts_byte` at line 23 connect the concrete borrowed segment cursor to concatenation. These are observation/representation lemmas, not a replacement byte-reader runtime path. The public concrete diagnostic names remain available; generic diagnostic reporting now calls the trait's diagnostic_seg_steps at `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:299`, because seg_steps is the ghost spec view.

Exact resume:

1. Convert `/home/ws/projects/verified-decoders/png/inflate_verus/src/bits.rs` into Verus, using the now-real trait contracts. Bits::new should require src.wf, not reset indices/counters; capture ghost counter/rank origins so work is a delta. Preserve the public inflate entry antecedent wf plus pos()==0. Bit position is 8*src.pos()-remaining in nat; prove cached-byte correctness, k<=32 reads, partial consumption on truncation, alignment, concat preservation and loop termination.
2. Verify `/home/ws/projects/verified-decoders/png/inflate_verus/src/huffman.rs:73` decode/MDec using reachable prefixes and the accepted D4 bridge, then compose constructor/admission/error mapping and convert live_counts' lengths observation to Ghost.
3. Do not claim full T-HUFF/L-CANON until those actual outer functions are verified. Public result counters remain runtime u128; the current public seg_steps result is still the historical absolute diagnostic, not a proved per-call delta for arbitrary wf plus pos()==0 states. Global work accounting, blocks/window, T-INFL, 32-bit replay and authored witness/mutation gates remain open.

The current trait contract works from arbitrary well-formed cursor states; no index/counter-reset precondition was added to read or to a public inflate theorem. No Bits source was changed in this continuation. No new trust item, solver-limit increase, frozen/manifest/errata edit, commit or background job was introduced. Backup before this layer: `/home/ws/projects/verified-decoders/png/inflate_verus/backups/segments-before-layer2.tar`. Accepted verifier/dependency versions and E1 boundary remain as recorded below.

---

## Live-prefix and D4 checkpoint, 2026-11-17

**Run 108: 118 verified, 0 errors.** Assigned layer 1 is complete: the actual live-count reverse loop and its independent remaining-mass invariant are verified, and the post-lookup dead-prefix expression is proved equivalent to independent canonical-word liveness for every admissible code. This is not a verified Huffman decoder or full T-HUFF. Resume with layer 2, Bits/Segments contracts.

| Runs | Evidence | Result |
|---|---|---|
| 80–84 | Independent remaining-mass recurrence and executable reverse loop | Integer quotient cast, explicit zero-product base case and existing code-growth unfolding fixes |
| 85 | Reverse loop, rounded division, safety, termination and erased work | 92 verified |
| 86–88 | Ten release tests, adapter, all vector layouts and 1,000 generated streams/corruptions | Passed |
| 89–90 | Complete-code prefix witnesses and exact remaining-mass partition | Pinned division lemma name/argument fixes and explicit zero-product base case |
| 91 | Complete-code proper-prefix interval | 104 verified |
| 92–93 | Single/empty code semantics | Explicit one-bit string and RFC first-code base unfolding |
| 94 | Complete/single/empty admissible live interval | 113 verified |
| 95–96 | Failed-lookup dead-prefix semantics and local MDec error precedence | Syntax fix, then 117 verified |
| 97 | Concrete table/live-array representation bridge for D4 | 118 verified |
| 98–102 | Formatting, ten release tests, adapter, all vector layouts and differential seed 4 | Passed; 8,772 public-entry pairs in run 102 |
| 103–106, 109 | Host/thumbv7m builds, instantiated allocator probes, source/cap controls and frozen hashes; final source-control replay | Passed; twelve frozen entries |
| 99, 107 | Unchanged-source verifier replays | Cargo cache hits; no fresh proof count |
| 108 | Final source after D4 precedence comment; fresh verifier replay | 118 verified, 0 errors |

### Exact proof surface

- `/home/ws/projects/verified-decoders/png/inflate_verus/src/model.rs:550` `remaining_mass_prefix` independently sums the leaf mass of symbols longer than depth n; `lemma_remaining_bounds` at line 570, `lemma_remaining_step` at line 580 and `lemma_remaining_partition` at line 628 prove bounds, reverse recurrence and the partition against the independent occupied mass.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/canonical.rs:312` `live_counts` is the actual executable reverse loop, called by `Huffman::build_canonical` at `/home/ws/projects/verified-decoders/png/inflate_verus/src/huffman.rs:69`; it proves every live entry equals the rounded independent mass, terminates safely and returns exactly MAX_CODE_BITS + 1 erased Ghost steps.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/model.rs:658` `lemma_complete_live_count` and line 750 `lemma_complete_proper_live` prove the complete-code interval; line 681 `lemma_rank_exists` and line 716 `lemma_complete_extension` construct independent symbol witnesses without executable table slots.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/model.rs:831` `lemma_single_words`, line 853 `lemma_single_live` and line 904 `lemma_empty_live` prove the other two admissible shapes; line 919 `lemma_admissible_live_interval` combines all three.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/model.rs:951` `lemma_numeric_exact`, line 974 `lemma_admissible_dead_prefix` and line 990 `lemma_decode_prefix_errors` establish failed-lookup subtraction safety, dead-prefix equivalence and local MDec InvalidSymbol-before-EOF precedence.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/canonical.rs:353` `lemma_dead_prefix_representation` connects the actual count/first/live arrays and failed table lookup to the independent liveness predicate.

The loop now uses `(higher + unit - 1) / unit` instead of `div_ceil`; Verus proves the addition, subtraction and division safe under the exact non-oversubscription precondition. Runtime diagnostics charge the same number of iterations outside the erased helper. The helper's lengths slice is an executable observation borrow used only by its specification; convert it to a Ghost sequence when the outer constructor enters Verus. No dynamic allocation was introduced.

### Resume and remaining boundaries

1. Encode T-SEG on the actual trait and SegmentSlices implementation in `/home/ws/projects/verified-decoders/png/inflate_verus/src/segments.rs`, then L-BITS on the actual reader in `/home/ws/projects/verified-decoders/png/inflate_verus/src/bits.rs`; retain only wf plus pos()==0 as the public fresh-entry antecedent, not zero segment-index/counter assumptions.
2. Verify `/home/ws/projects/verified-decoders/png/inflate_verus/src/huffman.rs:73` `decode` against MDec with reachable-prefix/no-shorter-hit and numeric-bit-prefix invariants; call the new D4 representation lemma before another read.
3. Move and compose the outer Huffman constructor, its model lengths/ready state, admission and error mapping into Verus; current helper preconditions are not yet proved at the unverified outer calls.
4. Hand back full T-HUFF/L-CANON only after that composition; blocks/window, independent append-only inflate model, public T-INFL, ghost public work accounting, caps_ok/CAPS-SYM, 32-bit proof replay and authored witness/mutation gates remain open.

No proof mutations were run. Pending live-prefix mutations include flooring the quotient, changing the reverse depth/shift, omitting depth zero, changing the dead-interval boundary, and reading input before rejecting a dead prefix. The complete/single/empty split is intentionally not claimed for arbitrary inadmissible incomplete codes.

Run 102 checks 631 frozen, 140 cap and 410 policy extractor streams in four layouts plus 1,000 generated streams and 1,000 single-bit corruptions, with zero unexpected differences. Public counters remain runtime u128. The approved ghost vstd/alloc cache and coordinator-owned E1 clarification are unchanged; no full A-PNG allocation claim is made. Verus 0.2026.11.02.8bf68ee, vstd 0.0.0-2026-11-01-0251 and rustc 1.99.1 (58cf8f9c3), default solver limits, two verifier threads, CARGO_BUILD_JOBS=1, --no-cheating and the 25-minute wrapper bound remain in force. No new trusted item, forbidden proof escape, frozen/manifest/errata edit, commit or background job was introduced. Backup: `/home/ws/projects/verified-decoders/png/inflate_verus/backups/live-prefix-before-run80.tar`.

---

Canonical-table checkpoint, 2026-11-17: **run 74, 84 verified, 0 errors**. This is a low-context handback within the Huffman layer. The actual canonical-table constructor and lookup are verified; the outer Huffman constructor, live-prefix logic and decode/MDec are not yet verified. No full T-HUFF or public inflate theorem is claimed. STUCK.md remains byte-identical to the accepted run-56 checkpoint, as the coordinator requested; this section contains the current resume point.

| Runs | Evidence | Result |
|---|---|---|
| 62–64 | Symbol filling with forward/inverse occupied-slot invariants | Ghost nat assignment and explicit quantifier-trigger fixes |
| 65 | Symbol filling, slot bounds and slot distinctness | 77 verified |
| 66 | Concrete table lookup, canonical-word forward/reverse lemmas | 81 verified |
| 67 | Erase classification/first-code/fill loop counters | 81 verified |
| 68 | Compose canonical construction and exact errors | Ghost sum needed an explicit nat cast |
| 69 | Complete canonical-table constructor with exact table relation | 84 verified |
| 70–73 | Formatting, ten release tests, all vector layouts and 1,000 streams/corruptions | Passed; 8,772 public-entry pairs in run 73 |
| 74 | Erase proof-only length/offset observations | 84 verified |
| 75–79 | Final ten release tests, host/thumbv7m builds, allocator-symbol checks, source controls and frozen hashes | Passed |

## Proved canonical-table surface

`/home/ws/projects/verified-decoders/png/inflate_verus/src/canonical.rs:61` `fill_symbols` proves forward symbol-to-slot and inverse occupied-slot mappings, with no specification of unused slots. The independent slot is `/home/ws/projects/verified-decoders/png/inflate_verus/src/model.rs:484` `symbol_slot`, with bounds at line 516 and distinctness at line 524.

`canonical.rs:264` `build_tables` is the composed executable constructor, not a helper-only proof. It is total for every input slice: success iff lengths_ok and Kraft <= CODE_SPACE; exact counts/Kraft/nonzero count and the full table_repr on success; TooManySymbols, InvalidLength(index) or Oversubscribed on the corresponding errors. Construction remains independent of kind-specific admissibility, so non-oversubscribed but inadmissible vectors receive their canonical representation.

`canonical.rs:232` `lookup` is the concrete function called by the decoder and carries the table relation in its ensures. `lemma_lookup_canonical` at line 208 proves every present independent canonical word selects its symbol. `lemma_lookup_word` at line 221 proves that every selected symbol has exactly the supplied codeword bit sequence; `lemma_lookup_representation` at line 186 provides the numeric relation. These establish the kernel-level L-CANON representation. They do not prove the outer Huffman constructor or its bit-reading decode loop.

Classification, first-code construction and symbol filling now update and return erased Ghost<nat> counters at their actual loops. build_tables composes those observations and charges its next-array fill. Success work is CANON_FIXED_STEPS + 2*lengths.len(); error work is tied to its actual classified outcome. The first-code length sequence and symbol-fill offset sequence are Ghost arguments, not executable observation borrows. The outer M1a wrapper retains separate runtime test diagnostics; public inflate result counters are still u128 until that outer surface is converted.

The actual production sites are `/home/ws/projects/verified-decoders/png/inflate_verus/src/huffman.rs`: build_canonical calls build_tables, and decode calls canonical::lookup. No separate model-mirror decoder or replacement runtime path was introduced.

## Exact next work

1. Verify the live-prefix-count constructor, replacing the remaining reverse loop in Huffman::build_canonical. Model the unassigned mass as a per-symbol sum over lengths greater than n; establish the reverse-loop invariant and its rounded division. For semantic D4 equivalence, admissible codes split into complete, one one-bit code and empty distance code; inadmissible non-oversubscribed vectors need canonical representation, not a stronger decode claim.
2. Prove the complete-code proper-prefix interval and the single/empty cases against model::cw/live/exact. Existing code-growth, msb-prefix-interval and prefix-free lemmas supply the numeric bounds. A count-prefix rank-existence lemma can produce the required independent model witnesses without importing executable tables.
3. Convert Segments' prose contract and the real Bits reader into Verus contracts, including concat preservation, positions, EOF, empty-segment advances, loop variants and separate ghost work. Preserve the manifest's wf plus pos()==0 entry antecedent rather than silently strengthening it with index/counter-reset assumptions.
4. Prove decode against MDec with a reachable-prefix invariant: all shorter prefixes have failed lookup, the current numeric code denotes the same bit prefix, and a dead prefix returns InvalidSymbol before another input read. Reader bit positions remain nat/u64, never u32.
5. Convert and compose the outer Huffman constructor itself, including kind admission, live counts, ghost length/ready state and error mapping; do not stop at the canonical kernel. Then finish T-HUFF(c), witnesses and the requested Huffman handback before moving to blocks/window.

Run 75 passes ten release tests. Run 73 compares all 631 frozen, 140 cap and 410 policy extractor streams across four layouts, plus 1,000 generated streams and 1,000 single-bit corruptions, with zero unexpected differences. Run 78 finds no alloc:: or allocator symbols in host/thumbv7m release libraries and in probes that instantiate both public inflate entries. Run 79 passes source/cap controls and all twelve frozen hashes.

The accepted cache configuration and coordinator-owned proposed no-alloc wording clarification are unchanged. Verifier 0.2026.11.02.8bf68ee; vstd 0.0.0-2026-11-01-0251; rustc 1.99.1 (58cf8f9c3); default solver limits, two threads, one cargo job and --no-cheating. No new trust item, forbidden proof escape, frozen/errata/release-gate edit, commit or background job was introduced.

---

# Historical Huffman foundation checkpoint

Huffman foundation checkpoint, 2026-11-17: **run 56, 70 verified, 0 errors**. The coordinator-approved GIF vstd cache resolves the bootstrap blocker. This is a partial Huffman-layer checkpoint, not T-HUFF or L-CANON completion.

| Runs | Evidence | Result |
|---|---|---|
| 29 | Copy intact GIF target/debug and reuse vstd/alloc hash 01822e92e4f44378 | Cache works; crate errors because constants were outside verus! |
| 30 | Independent model and verifier-visible constants | 24 verified |
| 31–35 | Host/thumbv7m builds and actual-entry allocator probes | Both libraries and both probes have no allocator symbols |
| 36–39 | Exact count/Kraft and admissibility proof iterations | Mutable-final-state, constant-bound and explicit nonlinear-premise fixes |
| 40 | Exact count/Kraft and numeric admissibility helpers | 37 verified |
| 41 | Independent rank, Kraft/canonical-capacity theory | 49 verified |
| 42–43 | Canonical bit-sequence and prefix-free proof iterations | Explicit take/index projections and equal-length injectivity fixes |
| 44 | Independent L-PREFIX-FREE | 62 verified |
| 45 | Executable first-code/offset representation | 69 verified |
| 46 | Unique exact codeword decoding | 70 verified |
| 47–50 | Formatting, ten release tests, all extractor layouts and 1,000 generated streams/corruptions | Passed; 8,772 public-entry pairs in run 50 |
| 51–54 | Final host/thumbv7m builds, instantiated allocator-symbol checks, source/cap controls and frozen hashes | Passed |
| 55 | Final formatted-source verifier replay | 70 verified, 0 errors |
| 56 | Replace vstd array setters by direct core array writes; verifier replay | 70 verified, 0 errors |
| 57–61 | Final host/thumb builds, ten release tests, allocator-symbol gate, source controls and frozen hashes | Passed |

Proved entry points at this checkpoint are helpers, not inflate's public API. `/home/ws/projects/verified-decoders/png/inflate_verus/src/classification.rs:64` computes exact counts, nonzero count and per-symbol Kraft including oversubscribed vectors. Its `is_admissible` at line 29 and `lemma_admissible_from_counts` at line 39 cover all kinds and both single-code choices. `/home/ws/projects/verified-decoders/png/inflate_verus/src/canonical.rs:10` represents first codes and offsets under the non-oversubscription antecedent. `/home/ws/projects/verified-decoders/png/inflate_verus/src/model.rs:397` proves L-PREFIX-FREE for independent canonical bit strings, and line 474 proves unique exact decoding.

The real constructor now calls those executable helpers. Its symbol filling, live-prefix counts and decode loop remain unverified; there is no complete L-CANON or T-HUFF, public-entry T-INFL, verified Segments contract, ghost work accounting or witness/mutation matrix. Runtime u128 diagnostics and helper usize loop tallies remain. The next step is verified symbol sorting with both forward and inverse occupied-slot invariants; STUCK.md gives the exact resume point. No block/window-layer proof was started.

The approved dependency has vstd/alloc enabled solely for ghost/spec support and cache compatibility. Model code is cfg(verus_only); verus macros erase proof code from the executable helpers/constants. The production source has no extern crate alloc, and nm checks find no allocator references in host/thumbv7m release libraries or actual public-entry probes. This does not settle a literal reading of the frozen 'no alloc dependency' phrase that forbids even ghost-library features; no A-PNG-6 sign-off is claimed and the frozen text remains unchanged.

Verifier: Verus 0.2026.11.02.8bf68ee, vstd 0.0.0-2026-11-01-0251, rustc 1.99.1 (58cf8f9c3), default solver limits, two threads, one cargo job, --no-cheating. The reused vstd .vir hash is `49934e1761f92df9d380921b59b8cae9c6ecbe92768ffbafc3f57820629ca533`. Trust remains the declared vstd specifications, verifier/compiler soundness and later allocation audit boundary; no new assumption, axiom, external body, unsafe block, solver-limit raise or weakened verifier flag was introduced. No frozen, errata or release-gate file changed. No commits or background jobs remain.

---

# Historical bootstrap attempt

M2 Huffman bootstrap, 2026-11-17. **Verified count: 0. One attempted verifier run failed before checking the crate.** No lemma, T-HUFF clause or later theorem is proved.

| Runs | Evidence | Result |
|---|---|---|
| 19 | Pinned no-alloc vstd; first independent-model verifier run through verify.sh | Exit 101: `--is-vstd` is not allowed with `--no-cheating`; no crate proof count |
| 20 | Canonical/admissibility split and existing runtime contracts | Three Huffman tests plus seven contract tests passed |
| 21 | Ordinary Rust macro parsing with the ghost model module enabled | Passed; not a verifier run or proof |
| 22 | Source/cap controls and frozen hashes with the pinned no-alloc dependency | Passed |
| 23 | Canonical-split release adapter build | Passed |
| 24 | Vector/layout regression plus 100 generated streams and corruptions | 5,172 entry-point pairs passed; no unexpected difference |
| 25–26 | Formatting and final-source release tests | Ten tests passed |
| 27 | Final-source no-alloc freestanding thumbv7m library build | Passed |
| 28 | Final-source allocation/cap controls and frozen hashes | Passed |

The new `/home/ws/projects/verified-decoders/png/inflate_verus/src/model.rs` defines independent Kraft/admissibility, the RFC next_code recurrence, canonical words as bit sequences, prefix predicates and D4 MDec. Its two elementary proof bodies are unverified. `/home/ws/projects/verified-decoders/png/inflate_verus/src/huffman.rs` now constructs canonical representation before the separate admissibility decision for every non-oversubscribed vector. A bounded Rust test covers all 15,625 length vectors of six entries in 0..4; this is runtime evidence, not T-HUFF.

The missing no-alloc vstd verification cache is `03bc18a84b05fe12` (metadata `b88a4c9d341b7830`) for vstd `0.0.0-2026-11-01-0251`. Located GIF verification caches enable alloc and do not match. No verifier flags were weakened, allocator feature enabled, fingerprints forged or cache directory removed. Exact diagnostics and required next action are in `/home/ws/projects/verified-decoders/png/inflate_verus/STUCK.md`.

M1a was backed up before edits at `/home/ws/projects/verified-decoders/png/inflate_verus/backups/m1a-before-m2.tar` (SHA-256 `71b4717543b6b88c8451ad7518e115f02a6d59697bb124cabedb2bcfa6e97466`). Its old source-hash file is historical. The Stage 2 short-output IgnoreTail discrepancy is explicitly recorded in STUCK.md. No frozen, errata or release-gate file changed. Runtime counters and prose Segments contracts remain unconverted.

---

# Historical M1a ledger

2026-11-17 checkpoint. Plain Rust implementation complete for the implemented Strict/RecommendedBeta option domain. **Verus verified count: 0; verifier runs: 0.** No theorem row or named lemma is proved. This is the requested handback after M1a, not an M2 or release declaration.

All run output is under `/home/ws/projects/verified-decoders/png/inflate_verus/runs/`; `/home/ws/projects/verified-decoders/png/inflate_verus/RUNLOG.tsv` records the commands, exits and durations. The filesystem resolves `/home/ws` to the mounted volume, so the TSV contains equivalent absolute volume paths.

| Runs | Evidence | Result |
|---|---|---|
| 1 | Initial debug contract suite | 7 passed |
| 2–3 | Release adapter; all extractor streams and ring smoke | Passed |
| 4 | Original full differential: 10,000 streams and 10,000 single-bit corruptions | Passed, 223.55 seconds, no unexpected differences |
| 5 | First source scan | Failed: the scanner conservatively flagged the nonallocating Adler/output methods named `push` |
| 6–7 | Initial i686 and freestanding thumbv7m library builds | Passed |
| 8 | Source scan after renaming those methods, scanner controls, cap controls, frozen hashes | Passed |
| 9 | Rust formatting | Passed |
| 10 | Final formatted-source debug suite | 7 passed; the label mentions release too, but this command was debug only |
| 11 | Final release differential adapter build | Passed |
| 12 | Final source, seed 1: 5,000 streams and 5,000 corruptions | Passed, 117.48 seconds, no unexpected differences |
| 13 | Final source, seed 2: 5,000 streams and 5,000 corruptions | Passed, 124.07 seconds, no unexpected differences |
| 14 | Final-source release suite | 7 passed |
| 15–16 | Final-source i686 and freestanding thumbv7m no_std builds | Passed |
| 17 | Final source scan, both scanner modes and controls, caps lint controls, twelve frozen hashes | Passed |

Each final differential shard runs 16,772 public-entry pairs: 4,724 vector-stream/layout requests, 48 ring/overlap requests, 10,000 valid/corrupt random requests and 2,000 limit/truncation requests. Every pair calls both `inflate_zlib_into` and `inflate_zlib_measure`. Across the two final shards, that is 33,544 pairs or 67,088 actual public calls. The final shards together contain the required 10,000 generated zlib streams and 10,000 single-bit corruptions. Each shard repeats all vectors and deterministic stress cases.

The vector-stream inventory is 631 frozen, 140 cap-dependent and 410 policy-labelled streams, for 1,181 streams. Four layouts include original PNG payload boundaries and empty segments. The harness explicitly marks 278 non-R cases with invalid or excessively large raw shapes; seven are frozen malformed-header/container records. It does not claim whole-PNG conformance on all 810 records. All inflate-specific M2 gate records use actual image R. The five D4 witnesses and `zlib/empty-idat-only` execute against authored errors; D5 has eight direct stored-prefix/limit cases. Forty-four frozen inflate-error expectations are checked directly against vector intent. General prefixes and success raw bytes use the frozen reference and are not independently authored model witnesses.

## Scope

The library is dependency-free `no_std`, without `alloc`, matching the frozen manifest rather than the task's initial `no_std+alloc` wording. Huffman state uses fixed arrays; both output and the measuring ring belong to the caller. `png_verus`, not this crate, will reserve `IccWindow` through the pinned guard. There is no guard copy or allocation log in this crate.

Runtime coverage includes stored/fixed/dynamic blocks, canonical admissibility, dead-prefix rejection, 32 fixed distance codes, window and overlap cases, exactly-full versus surplus output, Adler deferral, original and artificial segment boundaries, repeated EOF, one million empty segments, and a u64 measuring limit. The adapter checks status, count, exact written prefix, untouched suffix, last-window ring contents and observed core/segment work bounds on every request.

The current option type contains only the field values needed for Strict and RecommendedBeta. ImageCrateCompat-labelled vectors are extracted and exercised under Strict, not credited as compatibility semantics. `IgnoreTail` follows the manifest's exactly-L condition; the direct non-frozen policy-oracle wrapper is less restrictive for short output. No frozen Strict erratum is needed for this discrepancy.

## Unproved

There is no independent mathematical model, Verus public-entry contract or lemma. T-HUFF/L-CANON, L-PREFIX-FREE, T-INFL, T-SEG/L-BITS, L-WINDOW/L-RING, L-LIMIT/L-LIMIT-MONO, L-ADLER/L-HDR-BITS, T1/T11/T12, caps_ok, model witnesses and proof mutations all remain open. The runtime counters are u128 diagnostics and must become erased ghost observations. The allocation source scan and no_std builds are not a signed production call-graph audit or a proof of A-PNG rows.

The public-entry definitions are in `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs`; the exact next proof-layer steps are in `/home/ws/projects/verified-decoders/png/inflate_verus/STUCK.md`. Start with independent canonical words and Kraft/admissibility, and separate representation construction from dynamic admissibility rejection so T-HUFF(a) covers all non-oversubscribed vectors.

Rustc is 1.99.1 (`58cf8f9c3`, 2026-10-13); cargo is 1.99.1 (`5f94df478`, 2026-08-27). No Verus/vstd artifact was used. `verify.sh` is prepared for serial, bounded M2 verification and refuses a plain-Rust zero-obligation run. No solver-limit change, assumption, admission, external body, unsafe block or new axiom was introduced. No frozen, errata or release-gate file changed; no commit was made; no background process remains.
