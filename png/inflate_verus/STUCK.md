# Current checkpoint: split verifier checkpoints

2026-11-19. **Split checkpoint 592–597: 1,379 unique verified functions, 0 errors**, on the same proof sources as monolithic run 582. Core coverage is still 615/631. Run a checkpoint with `/home/ws/projects/verified-decoders/png/inflate_verus/verify.sh [label]`, which runs the six passes of `tools/verify_passes.json` serially under one lock. Each pass is bounded at 25 minutes with unchanged flags plus `--output-json`. `verify.sh --only PASS` runs one pass and is never a checkpoint.

Passes select modules through Cargo features and `#[cfg]` gates in `src/lib.rs`, because the pinned Verus rejects `--verify-module`/`--verify-root` under `--no-cheating` (585–587). Per-pass counts and durations: base 372 (29 s), config-cases 811 (12 min 39 s), extended 677 (5 min), classes-blocks 517 (2 min 14 s), dynamic 452 (1 min 27 s), fixed-policy 410 (50 s). The plain sum 3,239 counts base six times; the unique total 1,379 is what the audit compares against reference 582. All 33 tests pass in 598; controls and the coverage check pass in 599.

Open points: `config-cases` is a single module at 12 min 39 s, under 15 minutes but not well under, and it can only be divided by splitting `witness_config_cases.rs`. Cargo replays an unchanged pass without running Verus; the replay fails closed, so rerunning a checkpoint needs a changed Rust source. **Next logged run: 601.** All jobs have exited.

---

# Previous checkpoint: round 7 recovered at default limits

2026-11-19. **Fresh final-source run 582: 1,379 verified, 0 errors. Core coverage: 615/631, up by 23.** All 21 first-fixed-block gaps and both remaining dynamic profiles now verify. The sixteen remaining malformed/ambiguous IHDR/fcTL instances lack legal independent R and remain uncredited. No legal-R stream is resource-blocked; `tools/witness_blocked.json` is empty and all 23 round-7 failed formulations remain in retained historical diagnostics.

Both frozen declared-window records, all remaining level/filter/flush/truncation streams, image excess-output/excess-row families and the four new policy families are covered. Eighteen policy records now have inflate witnesses outside the core numerator; these do not establish integrated PNG policy acceptance. The exact class/stream list and failure reconstruction are at the top of LEDGER.md.

Final provenance/source controls pass in 583. All 27 negative/checkout/main-pipeline tests pass in 584. Source audit 576 found and then repaired a local trace-list name shadowing the PNG chunk iterator. Required logs and archives have retained hashes; run 582 binds all 42 source/Cargo/verify.sh files, and 63 retained evidence logs are hash-checked. Six unused backup files were pruned without dropping a referenced archive.

Runtime source is unchanged from 5f697fa. Fresh complete runtime evidence 566–572 passes: thirteen tests, host/thumb instantiated allocator probes and symbols, all vector layouts, 10,000 generated streams and 10,000 corruptions, zero unexpected differences. No signed allocation audit or M2 sign-off is claimed.

Runs 580 and 581 kill `length-extra-value-plus-one` and `window-boundary-equality-rejected`: each reports 1,378 verified, 1 error, at actual implementation/model refinement obligations. Historical total: **67**. Both exact sources are archived below 1 MB with retained log/archive hashes. Disposable failed source/Cargo files are removed; only target/log links remain. The unchanged-main fresh replay 582 is clean, and the pre-existing untracked `mutation-cases/` remains untouched. **Final count: run 582, 1,379 verified, 0 errors. Next logged run: 585. All jobs have exited; no failing or speculative proof remains active.**

Pinned i686 replay remains open because no matching pinned vstd cache exists (565). E1/E2 statuses, the SegmentSlices L-SEG-FRAME result and separate M3 caller obligations are unchanged. No new frozen-statement concern, proof escape, solver-limit increase, target cleaning, commit/push or edit outside the crate occurred.

---

# Previous checkpoint: default-limit decomposition, original layouts and parsed audit bindings

2026-11-18. **Fresh final-source run 529: 831 verified, 0 errors.** Core coverage is **592 / 631**, up by 29. The exact handoff is the top section of `/home/ws/projects/verified-decoders/png/inflate_verus/LEDGER.md`. All jobs have exited; no failing or speculative proof remains active. **Next logged run is 542.**

All twelve previously blocked streams now pass at default limits. Five stored classes use separate constructors, payload equality, fields, balanced checksum sums and symbolic model/public-call composition. Three further large BASE_RAW fixed/RLE classes cover nine instances. Six fdAT instances now have checksum-error or successful short-output proofs at their own fcTL R; two additional small IDAT instances close palette and trailing-stream cases. There are 250 regular stream classes covering 533 instances, plus 48 individual, three earlier small-error and eight finishing instances.

The three named original IDAT gates are closed: 1,005 one-byte parts, 67 irregular parts and five parts with lengths `[0,10,0,995,0]`. A verified borrowed-slice layout constructor calls both actual entries at R=1,023. These layout proofs add no duplicate coverage credit. Complete forward encoding means complete candidate DEFLATE-body encoding, not a zlib decoder or a source of expected output.

Both medium findings in `/home/ws/projects/verified-decoders/png/REVIEW-ASTRA-WITNESSES.md` are repaired. The audit parses exact input/limit/options/status/raw call descriptors for every credited witness, explicitly binds all 48 individual cases to independent R, checks exact policy tuple multisets, pins reviewed harness bodies and rejects unsupported runtime control flow. Eleven negative/checkout tests pass in runs 530/540; run 541 validates the final source-bound count, twenty retained log hashes, all source/frozen controls and a checkout replay without `runs/`. Required proof logs and final runtime logs live under non-ignored `evidence/`, with SHA-256 checks; historical blocked diagnostics remain retained and current `tools/witness_blocked.json` is empty.

**39 core instances remain:** 21 first-fixed-block trace gaps, dynamic `depth16/rounding-boundaries`, the dynamic fdAT `apng/interlaced-subframe`, and sixteen malformed/ambiguous IHDR/fcTL instances without a legal independent R. Do not invent fallback R or call these unattempted streams resource-blocked. The other frozen declared-window, level/filter/flush/truncation, image/policy excess-output/excess-row and beyond-declared-window gates remain open. Existing synthetic window/ring proofs do not replace them.

New implementation mutation kills are 524 (length-extra width plus one) and 528 (distance-extra width plus one), giving historical total **65**. Production runtime bodies did not change. Full runtime evidence passes in 531–537: thirteen tests, host/thumb instantiated allocator probes, every vector layout, 10,000 generated streams and 10,000 corruptions, zero unexpected differences; 33,544 entry-point pairs / 67,088 actual calls. Work is erased and proved, not measured. No signed allocation audit or M2 sign-off is claimed.

Pinned i686 proof replay remains open: run 539 finds no candidate in the inflate/GIF targets or `/home/ws/.cache`; no bootstrap or target substitution occurred. E1/E2 statuses, the SegmentSlices L-SEG-FRAME discharge and the separate M3 reader/call-site obligations remain unchanged. No new frozen-statement discrepancy, trust addition, solver-limit increase, target cleaning, commit/push or edit outside the crate occurred. Existing mutation/review evidence was left unchanged.

---

## Previous checkpoint (historical)

# Current checkpoint: fixed-match/effect bridge and extended authored classes

2026-11-18. **Fresh final-source run 510: 767 verified, 0 errors.** Core coverage is **563 / 631**, up by 220. The exact handoff is the top section of `/home/ws/projects/verified-decoders/png/inflate_verus/LEDGER.md`. All jobs have exited; no failing/speculative proof remains active. Next logged run is 521.

The new balanced fixed-match trace bridge proves literal/length and distance words, both extra fields and sequence-copy effects including overlap, independently of executable decode tables and the ring. A balanced Adler-sum bridge and separately verified literal input constructors support larger stored classes. The 128-byte authoring batch boundary is removed; successful new classes reach 527 raw bytes without implying a general solver bound.

128 new classes cover 217 frozen streams: 50 fixed-literal classes cover 59, 61 stored classes cover 70, and 17 fixed-match classes cover 88. The independent sample/filter/Adam7 replay now extends beyond initial images through filters, tRNS and interlace edges; APNG samples are authored directly. Complete forward encoding is an eligibility comparison, never a source of expected raw bytes. The audit checks all constructor/trace transcriptions and members at independently derived image/frame R, including 33 successful fdAT instances. Two empty fdAT errors and core trailing-in-later-IDAT add three more core instances, giving the increase of 220.

Both distance-before-output window-policy records additionally prove empty output/DistanceTooFar for all four window/tail choices at R and R+1, with original segments. Together with the earlier eight tail-policy records, ten policy records are proved outside the core numerator. These remain inflate outcomes, not integrated PNG policy results.

All 631 core instances are inventoried. **68 remain:** twelve default-limit-blocked streams, 32 first-fixed-block trace gaps, seven further fdAT gaps, one dynamic profile, and sixteen malformed/ambiguous IHDR/fcTL cases without a legal independent R. Do not invent fallback R or reuse a byte-identical class at a different R. Remaining named gates include the three large IDAT split/empty-segment records, declared windows, large zlib/flush/truncation, excess-output policy families, the distance-beyond-window policy family and further mutations.

Five queries exhausted default solver limits and were excluded without raising limits: RGB16 9x10 noninterlaced (550 raw bytes, run 507) and interlaced (560, 508), RGBA16 9x10 noninterlaced (730, 507) and interlaced (740, 507), and BASE_RAW stored (1,023, 508), the last covering eight frozen records. `/home/ws/projects/verified-decoders/png/inflate_verus/tools/witness_blocked.json` records exact class/member identities and diagnostics. The source/manifests from both failed runs are preserved in sub-1-MB backups. No unattempted stream is labelled resource-blocked.

Production implementations did not change. Full runtime evidence passes in runs 511–518 and final source/inventory controls in 520: thirteen tests, host/thumb instantiated probes and allocator symbols, all vector layouts, 10,000 generated streams and 10,000 corruptions, zero unexpected differences. No new mutation kills were added; the historical total remains 63. No signed allocation audit or M2 sign-off is claimed.

Pinned i686 replay remains blocked by the absent matching cache (519), not replaced by thumb. L-LIMIT-MONO and the existing synthetic window/ring/D5 proofs remain checked. E1/E2 statuses and the amended SegmentSlices L-SEG-FRAME discharge are unchanged; IDAT/fdAT/iCCP reader/call-site M3 duties remain. No frozen document, theorem manifest or errata file was edited; no new frozen-statement discrepancy arose.

---

## Previous checkpoint (historical)

# Current checkpoint: authored repeats and systematic stream classes

2026-11-18. **Fresh final-source run 492: 496 verified, 0 errors.** Core coverage is **343 / 631 frozen stream instances**, up by 300. The exact handoff is the top section of `/home/ws/projects/verified-decoders/png/inflate_verus/LEDGER.md`. All jobs have exited; no failed or speculative proof remains active. Next logged run is 502.

The new independent partial trace bridge handles literal code lengths plus repeats 16/17/18. Four frozen repeat/admission-error records and single-literal-code-len1 are closed. An explicit synthetic partial trace checks all three successful repeat kinds; it is not counted as a frozen stream or a complete public-entry stream witness.

A generic balanced fixed-literal bridge and independent D5 stored-payload bridge support 114 stream classes: 86 fixed-literal classes cover 259 frozen instances, and 28 stored classes cover 36. The other 48 core instances have individual authored witnesses. All class representatives prove independent raw bytes/status at R and call both actual entries; the provenance audit checks every member's exact Z and independently calculated legal image R. The authoring pipeline reconstructs encoder-intent samples, forward filters and Adam7 order without a decode oracle. Its forward encoding comparison only selects eligible traces. This batch uses at most 128 raw bytes; that is an authoring boundary, not a solver-bound claim.

Eight tail-policy records additionally verify under all four implemented window/tail combinations at R and R+1, including the original later-IDAT segmentation. IgnoreTail succeeds only for exactly full output after matching Adler; bad Adler is never ignored. These policy records do not enter the 343/631 core numerator and are not integrated PNG policy results.

All 631 core instances are listed in `/home/ws/projects/verified-decoders/png/inflate_verus/tools/witness_inventory.json`. The remaining 288 await authored traces: 161 first-fixed-block, 83 first-stored-block, 43 non-IDAT and one first-dynamic-block. No attempted instance remains resource-blocked. Extend independent fixed-match/effect traces and APNG frame classes next; retain per-stream provenance and frame R. The large frozen window/zlib/flush/truncation/segment-boundary records and excess-output/window-policy families remain open. Any new default-limit resource failure must be recorded against its named instance without raising limits.

Runtime implementations did not change. Full fresh evidence nevertheless passes in runs 493–500: source/cap/allocation/erased-counter/provenance controls and twelve hashes, thirteen release tests, release adapter, host/thumb instantiated allocator probes/symbols, all vector layouts, 10,000 generated streams and 10,000 corruptions with zero unexpected differences. No new mutation kill was added; the historical total stays 63. No signed allocation audit or M2 sign-off is claimed.

Pinned i686 replay remains blocked by the absent matching cache (run 501), not replaced by thumb. L-LIMIT-MONO, the prior synthetic window/ring/D5 proofs and the amended E2 SegmentSlices frame discharge remain proved. E1 and E2 statuses are unchanged; per-reader IDAT/fdAT/iCCP M3 obligations remain. No frozen file, theorem manifest or errata file was edited, and no new frozen-statement concern arose.

---

## Previous checkpoint (historical)

# Current checkpoint: selected dynamic/D4/window/ring gates and L-LIMIT-MONO

2026-11-18. **Fresh final-source run 476: 362 verified, 0 errors.** The exact handoff is the top section of `/home/ws/projects/verified-decoders/png/inflate_verus/LEDGER.md`. No failing or speculative proof remains in the active source tree. Next logged run is 477.

The frozen subset increased from 26 to 43 streams. New witnesses cover three successful dynamic blocks, all five frozen D4 short-input cases, the large fixed-overlap record, four rejected dynamic profiles, two companion dead-prefix records, empty-stored-then-stored and FCHECK. Every witness specifies its expected bytes/status independently and calls both real public entries. The balanced CL/literal-length trace bridge and sparse canonical-profile lemmas support further systematic witnesses without executable tables or an oracle. The provenance audit checks all 43 transcriptions and authoring-script restrictions. The large FCHECK literal constructor is isolated from its public-call query to stay within default solver limits.

Three additional synthetic streams prove distance 256 under a declared 256-byte window, distance 257 under Enforce versus Ignore, and ring wrap after 32,768 stored zeros followed by a length-three match at distance 32,768. They make both real public calls under both window settings with Reject tail and prove the actual measuring ring representation. They are not substitutes for the remaining frozen window or named RB-policy records. The existing synthetic D5 calls remain proved separately.

`/home/ws/projects/verified-decoders/png/inflate_verus/src/limit_mono.rs:234` now proves the separate independent-model L-LIMIT-MONO theorem for both implemented window/tail choices: a Done stream retains its exact output when the limit is lowered no further than that output's length. Future ICC callers must instantiate this theorem; no integrated ICC code was added.

Nineteen new verifier kills are runs 438–452 and 456–459, bringing the historical total to 63. Run 451 has 331 verified/5 errors; the other first-batch kills have 335 verified/1 error; the four later kills have 357 verified/1 error. They cover partial EOF/advance, single-code admission variants, live-count rounding/reverse shift/reverse depth/depth zero/dead boundaries, length extras, late Adler deferral, implementation-side bit-position narrowing, core/segment charge classification, reader/decode contamination of the construction budget and dynamic EOB/literal/distance admission order. Run 453 failed type checking before verification and is not a kill; its diagnostics are preserved. The budget/order kills include exact boundary/instrumentation checks, not an independent runtime work oracle. The ledger maps every run to its selector.

E2 in `/home/ws/projects/verified-decoders/png/ERRATA.md` is now APPROVED WITH AMENDMENT by the council. The ledger records SegmentSlices as the first L-SEG-FRAME discharge in the required (i)–(iv) form, with type/read/frame/public-entry/call-site file and line references. Its concrete read postcondition proves complete runtime non-cursor equality for both Some and None returns. The ledger distinguishes Verus value preservation from inspected type/call-graph structure and trusted Rust aliasing. No universal no-extra-effect claim is made for arbitrary T-SEG readers. IDAT, fdAT and iCCP readers still need separate M3 obligations and actual integrated call-site connections. No ERRATA.md, THEOREMS.md or frozen file was edited here.

Fresh full runtime evidence passes in 466–473: source/allocation/cap/erased-counter controls, 43 transcriptions and twelve frozen hashes; thirteen release tests; release adapter; host/thumb instantiated probes and allocator symbols; every 1,181 vector stream in four layouts per seed plus 10,000 generated streams and 10,000 corruptions across seeds 28/29. The differential makes 33,544 public-entry pairs, zero unexpected differences. Work is erased and proved, not measured. Production runtime bodies were unchanged; the only source change after those gates was an erased limit-bound projection, freshly verified in 476. Run 475 was a Cargo cache hit after an mtime-only touch and printed no count.

Resume with the remaining 588 core streams and named policy/segment gates, especially single-literal-code-len1, frozen declared-window records, CL/repeat-error records and RB/tail cases. The literal-length bridge does not yet handle repeat-16/17/18 traces. Additional mutation variants may still be useful; no exhaustive gate is claimed. Run 474 again finds no matching pinned i686 .vir cache in inflate/GIF targets or /home/ws/.cache; no bootstrap, feature/version/flag/fingerprint change, target substitution or i686 verifier replay occurred.

All jobs have exited. No commit/push, target deletion/cleaning/repair, new trust, proof escape, solver-limit increase or edit outside this crate. Accepted Verus/vstd/rustc, one Cargo job, two verifier threads and --no-cheating are unchanged. This is not M2 sign-off or a signed allocation audit. The ledger names both new sub-megabyte snapshots.

---

# Historical checkpoint: Astra repairs and partial witness/mutation expansion

2026-11-18. **Fresh final-source run 408: 300 verified, 0 errors.** Astra finding 2 is fixed: first EOF is a segment-only iteration; repeated EOF is a core-only iteration. Exact verifier witnesses cover zero segments and one empty segment. Bits mirrors the reader's actual core delta. Separate and summed manifest bounds remain proved on the host.

Astra finding 1 is resolved for production SegmentSlices: both actual public entries preserve its complete shared input frame, and its other runtime fields are cursor state. The frame type and origin observation are erased from the ordinary API. The generic restriction remains unsupported: the final release library still reproduces the unrelated marker write in run 410. `/home/ws/projects/verified-decoders/png/ERRATA.md` E2 is PROPOSED, not approved or applied; it narrows only the generic effect promise and adds explicit framing obligations for other production PNG readers. THEOREMS.md, spec-frozen and E1 are unchanged.

The exact handoff is the new top section of `/home/ws/projects/verified-decoders/png/inflate_verus/LEDGER.md`. Frozen input witnesses increased from twelve to 26: fixed literals, reserved literal/distance symbols including both reserved-distance EOF cases, small overlapping distance-equals-output, Full across the limit, both output-distance errors, stored-header truncation, BTYPE 3 and invalid dynamic counts. A separate synthetic D5 witness makes both public calls at L=4, 2 and 1 and pins the present payload prefixes and availability-before-limit order. Independent canonical-word bridge lemmas avoid the default interpreter recursion limit without changing limits or the model. Thirty-two new implementation-only verifier kills, all 299 verified/1 error, bring the recorded total to 44 with the historical twelve.

Resume item 4 with successful dynamic witnesses, all five frozen D4 public-call witnesses, declared-window edge cases, the large fixed-overlap record and ring wrap through measure. The other 605 core streams and required policy witnesses remain open. Preserve the historical remaining mutations, particularly partial-EOF consumption, live-count rounding/reverse-depth/depth-zero/boundaries, dynamic EOB/admission-order variants, length extras, late Adler deferral, bit-position narrowing and remaining work misclassification. Then prove L-LIMIT-MONO; it was not attempted in this partial continuation. No speculative or failing proof remains in the tree.

Run 409 again found no matching pinned i686 vstd verification cache in inflate/GIF targets or /home/ws/.cache. No bootstrap, target substitution, flag/feature/version/fingerprint change or i686 verifier replay occurred.

Final runtime evidence passes in runs 400–407: source/allocation/cap/erased-counter controls, 26 transcriptions and twelve frozen hashes; thirteen release tests; release adapter; host/thumb instantiated allocator probes and symbols; every 1,181 vector stream in four layouts per seed plus 10,000 generated streams and 10,000 corruptions across seeds 26/27. The differential made 33,544 entry pairs, zero unexpected differences. Work is erased and proved, not measured by that adapter. Run 408 is the fresh final count; runs 409–410 are not verifier counts. Only handoff documentation changed afterwards.

No new trust, forbidden proof escape, solver-limit increase, target deletion/cleaning/repair, commit, push or surviving background job. Accepted Verus/vstd/rustc and --no-cheating settings are unchanged. No M2 sign-off or signed allocation audit is claimed. Next logged run is 411; the ledger names both sub-megabyte snapshots and every new mutation/run mapping.

---

# Historical checkpoint: public envelope and erased work complete on host

2026-11-18. **Fresh final-source run 343: 272 verified, 0 errors.** Step (2c) is complete for the implemented four Strict/RB field combinations on the verified host target. Both actual public entries refine one independent Inflate model, preserve the original caller slice/source and public window/ring invariants, and return erased per-call counters with separate and summed manifest bounds. Public source freshness is exactly wf plus pos()==0, with no index/counter-reset antecedent. No host proof blocker remains. This is not complete M2 sign-off or a 32-bit verification result.

The exact contracts and evidence are at the top of `/home/ws/projects/verified-decoders/png/inflate_verus/LEDGER.md`. The new model is `/home/ws/projects/verified-decoders/png/inflate_verus/src/model.rs:1710–1759`; production checksum/finish/run are `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:1124/1172/1192`; actual entry/into/measure are :1267/:1319/:1342. Sink/source future-view framing uses ordinary verified prophetic specs, with no new trust. `InflateResult` at `/home/ws/projects/verified-decoders/png/inflate_verus/src/lib.rs:103` has Ghost<nat> counters; Work and SegmentSlices have no runtime-u128 work state. The measuring slice retains its exact documented ring-length precondition.

Resume item 4 with the other 619 of 631 authored core model/public-call witnesses and the required policy witnesses. Twelve named envelope/stored witnesses, including Z=[], are proved in `/home/ws/projects/verified-decoders/png/inflate_verus/src/witnesses.rs`, with both real public entries. Twelve mutations fail verification in runs 317–328; the ledger enumerates those kills and the unrun reader/Huffman/dynamic/block/work mutations. The selector failure for the unmutated reserved-length-286 copy is not a verifier kill. The separate L-LIMIT-MONO model theorem remains open.

Item 5 needs a matching pinned i686 vstd verification cache. Run 329 found no i686 .vir candidate in inflate/GIF target directories or `/home/ws/.cache`. No i686 replay or bootstrap was attempted, no thumbv7m build was substituted, and no flag, feature, fingerprint or target cache was weakened, deleted or repaired.

Final runtime gates pass: thirteen release tests (335), adapter (336), host/thumbv7m libraries and instantiated probes (337–338), allocator symbols (339), source/cap/erased-counter controls, witness provenance and twelve frozen hashes (340). Runs 341–342 pass all 1,181 vector streams in four layouts plus 10,000 generated streams and 10,000 corruptions, seeds 24/25: 33,544 public-entry pairs, zero unexpected differences. Runtime checks no longer inspect erased work; public proof contracts establish those bounds. Runs 305/306 had a stale stdout label saying observed work, corrected in the final runs. Run 343 changes only erased summed-work postconditions after these gates.

No frozen-statement concern, new erratum, trusted addition, solver-limit change, frozen/manifest/errata edit, commit or surviving background job was introduced. E1 is APPROVED. Accepted verifier/vstd/rustc settings are unchanged. Next logged run is 344. The ledger names the two sub-megabyte pre-edit snapshots.

---

# Historical checkpoint: actual stored/compressed/block refinement complete

2026-11-18. **Fresh final-source run 285: 236 verified, 0 errors.** Steps (2a) and (2b) are complete at the actual production helpers and full DEFLATE block loop under their explicit well-formedness/window preconditions. This is a clean handoff before step (2c), not a public inflate theorem. No current proof/tooling blocker exists. `/home/ws/projects/verified-decoders/png/inflate_verus/LEDGER.md` has the exact contracts, locations and evidence.

The production chain in `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs` now includes stored_block (:348) against MStoredBlock, match_copy (:838) against MCopy, compressed_step (:901) against MCompressedStep, compressed (:947) against MCompressed, one_block (:998) against MOneBlock and blocks (:1038) against MBlk. The actual run calls these helpers. The independent models in `/home/ws/projects/verified-decoders/png/inflate_verus/src/model.rs:1539–1700` use mathematical sequences, canonical MDec and arithmetic RFC bases/extras; they never import executable tables, buffers or ring_repr.

The stored proof includes alignment, four ordered LEN/NLEN reads, complement and D5 payload composition. The compressed proof includes reserved literal/length 286/287 and distance 30/31, bases/extras, distance/output/window rejection before any emission or Full decision, the actual overlapping copy and maintained slice/ring/Adler representation. selected_window (:1085) proves declared-window versus Ignore for validated CINFO; the outer guard/caller composition remains open. blocks proves exact full BFINAL-loop effects and termination, and local work <=64*(consumed_bits+1)+2*newly_produced, without the +1 on success. Header construction is now <=48*consumed_bits on every outcome, separate from reader/decode work and retaining its <2,048 budget.

Resume step (2c) at run (:1105): independent Inflate envelope, zlib header/check order, table/blocks caller preconditions, checksum alignment/BE32/Adler and exactly-full IgnoreTail; then entry (:1148), inflate_zlib_into (:1166) and inflate_zlib_measure (:1178). Public freshness must remain exactly wf plus pos()==0, with no index/counter resets. Public sink identity/caller-original-slice framing, returned Ghost per-call work counters and limits.rs caps_ok/global T1/T11/T12 remain open. Existing public runtime-u128 diagnostics were not changed or expanded. Do not call block-local L-WINDOW/L-RING preservation a complete public theorem.

Runs 276–277 pass thirteen release tests and the adapter. Runs 279–280 pass all 1,181 vector streams in four layouts plus 10,000 generated streams and 10,000 corruptions, seeds 20/21: 33,544 public-entry pairs, zero unexpected differences. Runs 281–284 pass host/thumbv7m libraries/instantiated allocator probes, symbols, source/cap controls and twelve frozen hashes. Run 285 changes only an erased origin-bound assertion after those runtime gates. Runs 259 and 278 were cached green invocations without fresh verifier counts; neither is evidence for a new count. Next logged run is 286.

Authored model-vector witnesses, all proof mutations and the matching pinned i686 verification replay remain open. No frozen counterexample/new erratum, trusted addition, solver-limit change, frozen edit, commit or surviving background job was introduced. No target directory was cleaned, deleted or repaired. Accepted verifier/vstd/rustc settings and E1 are unchanged. The ledger names the three sub-megabyte pre-edit backups and the precise remaining boundaries.

---

# Historical checkpoint: actual dynamic-header parser complete

2026-11-17. **Fresh final-source run 252: 215 verified, 0 errors.** Assigned step (1) is complete; no current tooling/proof blocker exists. The actual Tables::dynamic now refines independent MDynamic on every outcome. This is not completion of layer 3 or a public inflate theorem. `/home/ws/projects/verified-decoders/png/inflate_verus/LEDGER.md` has the exact contracts, locations and runs.

The verified production chain in `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs` is lemma_cl_order (:32), dynamic_code (:358), write_length_repeat (:425), Tables::dynamic_lengths (:455), Tables::dynamic (:588), and read_block_header (:630). CL and expansion invariants maintain equality to MDynamicCL/MDynamicLengths. The combined 316-entry vector uses length_entries_ok. Actual read/check/admission order is preserved, including all count reads before admission and repeat-16's previous-entry check before extras.

L-HDR-BITS now reaches the actual literal-construction boundary: the expansion loop establishes lemma_header_coding_cost from separate symbol-18 and other counts/entries/bits; header_table_start is set immediately before the real literal build, including a build that rejects. Tables::dynamic proves at least 42 bits from its entry, and read_block_header includes the actual three block bits to prove at least 45. Ghost header_construction excludes reader/decode work: helper bounds sum to at most 1,845 construction iterations, exported as <2,048; actual observed work delta <= construction + 3*(consumed_bits+1). This is local accounting, not global T12. Bits::get (:120) and Huffman::decode (:165) now export consumed-bit-sensitive cost and endpoint/symbol bounds without changing their runtime bodies.

Resume step (2) at `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:745–755`, the still-unverified stored alignment/LEN/NLEN/complement check, then compressed (:674): reserved symbols, bases/extras, window/distance checks and actual overlapping-copy refinement against MCopy. Add independent full MBlk/control flow and compose run (:717). read_block_header already handles the real bit fields/type-3 rejection/dynamic selection but does not prove block payloads or full dispatch simulation. Full L-WINDOW/L-RING and public T-INFL/global T1/T11/T12 remain open.

Public entries (:808, :820) still need Ghost returned counters, wf plus pos()==0 freshness with no reset antecedent, and suffix framing relative to the caller's original slice. Authored model-vector witnesses, proof mutations and the 32-bit verifier replay remain open. The ledger records the dynamic mutation list and i686 replay plan; thumbv7m ordinary builds are not 32-bit verification evidence.

Runs 250–258 pass thirteen release tests, the fresh 215-proof replay, all vector layouts plus 10,000 generated streams and 10,000 corruptions (seeds 18/19), host/thumbv7m libraries/instantiated probes, allocator/source/cap controls and twelve frozen hashes. No frozen counterexample/new erratum, commit or background job was introduced. E1 and accepted verifier/dependency settings remain unchanged. Backup: `/home/ws/projects/verified-decoders/png/inflate_verus/backups/dynamic-parser-before-layer3.tar`.

Proof lessons: keep the real repeat-fill loop in a separate verified production helper, retain explicit model-origin equalities in loop invariants, and export bit-position bounds rather than asking callers to unfold reader wf. The expansion helper uses spinoff_prover. Scoped hide(Huffman::repr/wf) triggered unused-fuel AIR failures and was removed; neither solver limits nor trust changed.

---

# Historical checkpoint: independent dynamic-header model

2026-11-17. **Fresh run 231: 205 verified, 0 errors.** Low-context handback within layer 3, not completion of it. No current tooling/proof blocker exists. Only model definitions/proofs and Huffman work postconditions changed since the 200-verified checkpoint; runtime bodies are unchanged. `/home/ws/projects/verified-decoders/png/inflate_verus/LEDGER.md` records the exact surface and run history.

The independent ordered MDynamic/MDynamicCL/MDynamicLengths model is in `/home/ws/projects/verified-decoders/png/inflate_verus/src/model.rs:1379–1494`. It uses mathematical length sequences and bit positions, never executable tables. lemma_dynamic_count_fields (:1496), lemma_finish_dynamic (:1508) and lemma_header_coding_cost (:1524) prove count ranges, final split/admission/316-entry facts and an aggregate sixteen-bit arithmetic ingredient. The actual header loop does not yet establish the coding-cost premises; L-HDR-BITS and its construction budget remain open. Huffman build/build_with_single/build_canonical now export a length-dependent Ghost-work upper bound alongside the uniform bound.

Resume at `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:324`, the unchanged, unverified Tables::dynamic. Move the real method into Verus; connect the actual CL_ORDER to the independent permutation; maintain concrete CL/length-prefix relations to MDynamicCL/MDynamicLengths. Use model::length_entries_ok for the combined 316-entry sequence, not lengths_ok's 288-entry canonical-vector bound. Preserve count-field admission order, repeat-16's previous-entry check before extra bits, EOB presence, then literal admission, then distance admission.

Track construction work separately from reader/decode work. For L-HDR-BITS, track symbol-18 counts, emitted entries and consumed bits separately from other entries/bits, establish lemma_header_coding_cost's premises and include count/CL/block-header bits. Export successful-decode symbol-domain and consumed-bit facts if repeatedly unfolding MDec becomes expensive. The new constructor bound is an upper bound, not an equality.

After dynamic parsing, complete stored header/dispatch, reserved-symbol and distance/window checks, overlapping-copy refinement against MCopy/MBlk, then the outer run. Full MBlk/Inflate, complete L-WINDOW/L-RING and public T-INFL remain open. Preserve wf plus pos()==0 for public freshness and finish caller-buffer identity/snapshot framing, Ghost result/global-work accounting, 32-bit replay, authored witnesses and proof mutations later.

Runs 232–238 pass thirteen release tests, all vectors plus 1,000 generated streams and 1,000 corruptions, host/thumbv7m builds/instantiated allocator probes, source/cap controls and twelve frozen hashes. Runs 225–226 remain the full 10,000-stream/10,000-corruption differential on unchanged runtime bodies. No frozen counterexample/new erratum, commit or background job was introduced; E1 remains coordinator-owned. Backup: `/home/ws/projects/verified-decoders/png/inflate_verus/backups/dynamic-model-before-layer3.tar`.

---

# Historical checkpoint: stored payload, output, Adler and fixed tables

2026-11-17. **Fresh run 222: 200 verified, 0 errors.** Low-context handback within layer 3, not completion of it. There is no current tooling/proof blocker. `/home/ws/projects/verified-decoders/png/inflate_verus/LEDGER.md` has the exact surface and run history.

Proved actual production paths now include Adler new/update_byte/finish against independent mathematical sums, Output new/append_byte/previous against the sequence-output observation and ring/slice representations, stored_payload against MStoredPayload, and Tables::new against independent complete fixed profiles including all 32 distance codes. The real run calls the verified stored-payload helper. No Adler update is treated as trusted. Stored payload work and fixed initialization work have exact Ghost observations/bounds.

Resume with `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:324`, Tables::dynamic, and its independent header model. Prove count/length/repeat ordering and exact errors/admission, 316 entries, L-HDR-BITS and the 2,048-iteration construction budget. Then finish the actual stored header/dispatch and compressed symbol/match loop (:381), using the independent MCopy and verified output primitives. The whole run (:424), including LEN/NLEN and zlib header checks, remains unverified. Full MBlk/Inflate control-flow models and complete L-WINDOW/L-RING are not yet proved.

Useful interfaces: `/home/ws/projects/verified-decoders/png/inflate_verus/src/adler.rs:83` update_byte preserves wf and appends exactly one observed byte; finish (:113) equals model::Adler. `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:131` append_byte refines MEmit and retains output/Adler/ring/suffix invariants; previous (:170) reads model output by distance. stored_payload (:193) proves exact D5 effects after header validation/alignment, with Ghost cost<=2*count. Tables::new (:265) establishes fixed_wf and Ghost cost<=C_INFL_FIXED.

Do not overclaim the primitives: public output borrowing/initial-suffix framing through entry still needs constructor identity/snapshot facts; fixed reserved-symbol error checks and distance/window enforcement are still in unverified compressed. Public freshness remains wf plus pos()==0, without reset/index assumptions. Public T-INFL/global safety/work, 32-bit verifier replay, public Ghost result counters and authored witness/mutation gates remain open.

Runs 223–230 pass thirteen release tests, all vector layouts, 10,000 generated streams plus 10,000 corruptions, host/thumbv7m libraries and actual-entry allocator-symbol probes, source/cap controls and all twelve frozen hashes. No frozen counterexample/new erratum, commit or background job was introduced; E1 remains unchanged.

Proof lessons: carry local polynomial definitions explicitly into nonlinear_arith requires; use the pinned vstd skip-composition lemma to project an untouched suffix; borrow enum-held mutable sinks in proof matches rather than partially moving them. Output::new explicitly states the existing ring-view length requirement because a nested borrowed array view did not automatically yield its length. Do not replace that issue with a stronger public cursor/freshness precondition.

---

# Historical checkpoint: independent output-model foundation

2026-11-17. **Fresh run 193: 170 verified, 0 errors.** Low-context checkpoint within layer 3, not completion of that layer. There is no tooling/proof blocker. Only `/home/ws/projects/verified-decoders/png/inflate_verus/src/model.rs` changed beyond the accepted layer-2 source; no runtime code changed. `/home/ws/projects/verified-decoders/png/inflate_verus/LEDGER.md` has the exact new definitions, proofs and run evidence.

The model now has independent MEmit, copy_distance, MCopy and MStoredPayload operations. It appends to its own Seq<u8>, resolves each match byte by distance into the evolving sequence, and uses its own InflateFault labels. `lemma_copy_distance` at line 1167 proves length, prefix preservation and overlapping-copy recurrence; `lemma_copy_prefix` at line 1195 proves intermediate prefixes. `lemma_MCopy_shape` at line 1207 proves distance-before-limit and exact match effects. `lemma_stored_payload` at line 1240 proves the exact D5 stream-order prefix and End/Full/TruncatedZlib conditions. This is not yet MBlk/Inflate or an executable/model representation theorem.

Resume with the actual stored path at `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:229` and Output::append_byte at line 42. The output sink and Adler calls remain outside Verus, so their preconditions/updates must be proved or factored through the same actual production path; do not introduce a trusted call to Adler.update_byte. Extend the independent model with fixed/dynamic parsing and block control, prove fixed construction with 32 distance codes at line 70, dynamic ordering/admission/316-entry/L-HDR-BITS/budget at line 102, then the actual compressed/window/ring/overlap paths at line 159. The current pure output lemmas do not close L-WINDOW or L-RING.

Preserve wf plus pos()==0 for eventual public entry contracts; no reset/index assumption is permitted. Public T-INFL, global work/safety, 32-bit proof replay and authored witness/mutation gates remain open. No concrete model-vector witness or proof mutation was run in this continuation.

Runs 186–192 pass thirteen release tests, all vector layouts plus 1,000 generated streams and corruptions, both target builds/instantiated allocator-symbol probes, source/cap controls and twelve frozen hashes. Run 193 freshly checks the model's separate error datatype; runs 194–195 repeat final source/hash controls and thirteen tests. Accepted runtime bodies are unchanged. No frozen counterexample/new erratum, commit or background job was introduced; E1 remains unchanged.

---

# Historical checkpoint: reader/Huffman layer complete

2026-11-17. **Fresh run 176: 163 verified, 0 errors.** Assigned layer 2 is complete at the actual Segments, Bits and Huffman methods. There is no current blocker. T-HUFF(a–c)/L-CANON are proved on the verified host target; 32-bit replay and authored witness/mutation gates remain open. No public T-INFL, global safety/termination/work theorem, full PNG-container T-SEG or allocation sign-off is claimed. `/home/ws/projects/verified-decoders/png/inflate_verus/LEDGER.md` gives exact locations and the run history.

Resume with layer 3 in `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs` and an independent evolving output-sequence model in `/home/ws/projects/verified-decoders/png/inflate_verus/src/model.rs`. The model must append to its own Seq<u8> and resolve overlap by distance into that output, never share the executable ring/tables. Start with stored D5 prefixes and availability-before-limit; then fixed codes including all 32 distance symbols; dynamic header ordering, 316 entries, L-HDR-BITS and the 2,048-iteration budget; then window/ring/overlap representation. After that prove both real public entries, not a wrapper or a separate decoder.

The actual Huffman constructor/admission methods now establish repr/wf/ready and both single-code choices; decode at `/home/ws/projects/verified-decoders/png/inflate_verus/src/huffman.rs:159` returns exactly MDec's symbol/error and bit position. It uses Bits::get's proved field_value contract and a reachable-prefix invariant with the D4 representation bridge. The live-count lengths observation is Ghost-only. The block-level callers remain unverified and still need to establish those helper preconditions.

Bits::new at `/home/ws/projects/verified-decoders/png/inflate_verus/src/bits.rs:77` requires only source wf and captures ghost counter/rank origins, without index/counter resets. Preserve wf plus pos()==0 as the public fresh-entry antecedent. Public results still contain runtime u128 diagnostics; the core diagnostic now clamps while exact Ghost work does not. The public segment diagnostic remains absolute, not a proved per-call ghost delta. Global returned Ghost accounting/T12 is still open.

Runs 167–174 pass thirteen release tests, all vector layouts, 10,000 generated streams plus 10,000 corruptions, host/thumbv7m instantiated allocator probes, source/cap controls and all twelve frozen hashes. Runs 175–176 strengthen only the proof invariant/alignment contract; run 176 rechecks all 163 obligations. Runs 177–183 repeat the final-source release/vector checks with another 1,000 streams/corruptions and both target/source/hash gates. No runtime body changed after the full differential. No authored model-vector proof or mutation campaign ran. No frozen counterexample/new erratum, commit or background job was introduced; E1 remains unchanged.

Proof lessons: generic consumers cannot infer laws from an overridable default trait spec body, so Segments now requires a proved lemma_wf, implemented by SegmentSlices. The decoder initially exceeded the default solver resource limit; scoped hide headers for recursive decode_prefix and table_repr, plus small projection lemmas, resolved it without changing limits. hide headers must sit directly at function entry, not inside a proof block. An explicit reachable bit-position bound was needed to show that a failed one-bit read ended exactly at EOF.

---

# Historical checkpoint: Segments proved, Bits next

2026-11-17. **Run 124: 130 verified, 0 errors.** This is a low-context checkpoint within assigned layer 2. There is no remaining Segments proof/tooling blocker; full layer 2 is not complete. The actual trait and SegmentSlices constructor/read loop are verified in `/home/ws/projects/verified-decoders/png/inflate_verus/src/segments.rs`. `/home/ws/projects/verified-decoders/png/inflate_verus/LEDGER.md` has the exact contracts, locations and run inventory.

Resume with the unchanged `/home/ws/projects/verified-decoders/png/inflate_verus/src/bits.rs`. The trait now supplies spec concat/pos/index/count, ghost core/segment work and an advance rank. wf exposes the universal bounds; read requires only wf and preserves concat and segment count. Its work delta equals the bounded rank delta, so do not assume reset counters or initial index zero. Bits::new should snapshot work/rank origins and require only source wf; the public inflate entry condition remains wf plus pos()==0. The current public runtime seg_steps result is still an absolute M1a diagnostic, not a proved per-call delta on arbitrary fresh-by-position states.

Prove the actual cached-byte reader's bit relation and k<=32 operations, alignment, truncation consumption, range and termination. Its calls to the current unverified Work::add also need a verified work interface. Then verify `/home/ws/projects/verified-decoders/png/inflate_verus/src/huffman.rs:73` decode/MDec with the reachable-prefix invariant and D4 bridge, and compose the outer constructor/admission/error mapping, converting live_counts' lengths observation to Ghost. No full L-BITS/T-HUFF/T-INFL or public work theorem is claimed.

Runs 125–131 pass ten release tests, all vector layouts, 1,000 generated streams plus corruptions, both target builds/instantiated allocator-symbol probes, source/cap controls and all twelve frozen hashes. No proof mutations were run. No frozen counterexample or new erratum was found; E1 and the accepted cache boundary remain unchanged. No commit or background job was introduced.

Verifier lesson from run 113: revealing generic trait functions with reveal(Segments::wf) caused an internal undeclared-fuel AIR error. Module-local closed implementation specs unfolded without those reveals. The read-loop proof also needed initial == *old(self) explicitly retained in its invariant. Natural ghost fields in executable constructors were replaced with erased Ghost<nat> wrapper fields.

---

# Historical checkpoint: live-prefix and D4 layer complete

2026-11-17. **Run 108: 118 verified, 0 errors.** There is no current tooling or proof blocker. This handback ends assigned layer 1, not full T-HUFF. `/home/ws/projects/verified-decoders/png/inflate_verus/LEDGER.md` has the exact proof inventory and run history. The previous sections below are historical and retain their original claims.

`canonical::live_counts` at `/home/ws/projects/verified-decoders/png/inflate_verus/src/canonical.rs:312` replaces the real reverse loop and proves every rounded live count against `model::remaining_mass`, an independent per-symbol sum. `model::lemma_admissible_live_interval` at `/home/ws/projects/verified-decoders/png/inflate_verus/src/model.rs:919` proves the complete/single-one-bit/empty split. `canonical::lemma_dead_prefix_representation` at line 353 ties failed concrete lookup and the tested arithmetic interval to independent canonical-word liveness. `model::lemma_decode_prefix_errors` at line 990 proves local MDec error precedence, not the concrete decode loop.

Resume with the actual `Segments` trait and `SegmentSlices` in `/home/ws/projects/verified-decoders/png/inflate_verus/src/segments.rs`, then `Bits` in `/home/ws/projects/verified-decoders/png/inflate_verus/src/bits.rs`. Public entry freshness is wf plus pos()==0; do not promote the old prose's zero index/counter state into a stronger theorem precondition. Require concat preservation, cursor range, stable repeated EOF, monotonic segment progress, separate erased segment work and termination. Keep bit positions nat/u64.

Then verify `/home/ws/projects/verified-decoders/png/inflate_verus/src/huffman.rs:73` `decode` against MDec using a reachable-prefix invariant and the new D4 bridge, and compose the outer constructor/admission/error mapping. Its helper calls are still outside Verus. `live_counts` currently takes a proof-only executable lengths borrow; replace that observation with Ghost lengths when the outer constructor can supply them. Both public result counters and segment observations remain runtime u128; no public work, T-SEG, full T-HUFF, T-INFL, window/ring, allocation or witness/mutation theorem is claimed.

Runs 100–106 pass ten release tests, all vector layouts, 1,000 generated streams/corruptions, both target builds/instantiated allocator-symbol probes, source/cap controls and all twelve frozen hashes. Runs 99 and 107 were Cargo cache hits, not fresh proof evidence; run 108 did recheck the crate. The accepted ghost vstd/alloc boundary and coordinator-owned E1 wording proposal are unchanged. No frozen counterexample or new erratum was found. No commit or background job exists from this session.

Proof lesson: new definitions exposed implicit multiplication/unfolding dependencies in the old `lemma_code_growth`; explicit recurrence and base-case multiplication stabilized it. Per-symbol mass induction also needed an explicit zero-times-unit base fact. Use the pinned vstd `lemma_div_multiples_vanish_fancy(x, remainder, divisor)` argument order for rounded quotients.

---

# Historical Huffman foundation checkpoint: 70 verified

2026-11-17. Run 56 reports **70 verified, 0 errors**, default solver limits, two threads, one cargo job and `--no-cheating`. The cache blocker is resolved by the coordinator-approved reuse of GIF's alloc-enabled vstd artifact. This is a low-context handback within the Huffman layer, not completion of T-HUFF or L-CANON. There is no current verifier/tooling blocker.

## What is proved

- `src/classification.rs:64` `count_lengths` returns exact per-length counts, nonzero count and the independent per-symbol Kraft sum for every valid length vector of at most FIXED_LIT_COUNT entries, including oversubscribed vectors; invalid lengths return an error and both loops terminate safely.
- `src/classification.rs:29` `is_admissible` implements the complete/oversubscribed/single/empty conditions for all three kinds and both single-code values; `:39` `lemma_admissible_from_counts` relates those numeric tests to independent model admissibility.
- `src/canonical.rs:10` `first_codes` proves the actual per-length first codes equal the RFC next_code recurrence and offsets equal the number of shorter nonzero lengths, for any non-oversubscribed vector and matching counts; its loop terminates safely.
- `src/model.rs:397` `lemma_prefix_free` proves that one independent canonical word prefixes another only when the symbols are identical, under lengths_ok and Kraft <= CODE_SPACE; `:474` `lemma_exact_unique` gives unique exact decoding.
- `src/model.rs:255` `lemma_kraft_canonical`, `:276` `lemma_canonical_capacity` and `:285` `lemma_cw_value_bound` connect the per-symbol Kraft sum to canonical capacity and bounded codewords.

The actual plain-Rust Huffman constructor calls those executable helpers at `src/huffman.rs:22`, `:45` and `:58`. Their preconditions are not yet composed into a verified constructor or public-entry theorem. `build_canonical` still fills the symbol array and live-prefix counts outside Verus; `decode` is also unverified. Full L-CANON and T-HUFF remain open.

## Exact resume point

1. Continue in `/home/ws/projects/verified-decoders/png/inflate_verus/src/canonical.rs` with verified symbol-table filling. Define a model symbol slot as shorter_prefix(lengths, len, |lengths|) plus the count of same-length symbols before s. Prove slot bounds and distinctness using the existing rank and shorter-prefix lemmas. Maintain both forward symbol-to-slot and inverse occupied-slot invariants; leave unused slots unspecified.
2. Connect counts, first codes, offsets and symbols in the full L-CANON table-lookup theorem on canonical codewords, then prove live-prefix counts and dead-prefix rejection.
3. Prove decoding against MDec over the real bit reader. This needs verified bit/Segments contracts rather than treating an unverified reader as trusted. The source is still the M1a byte-cache reader, and the public Segments contract is still prose.
4. Compose helper contracts through the actual Huffman constructor and, eventually, the actual public inflate entry points. Do not cite the standalone helper proofs as a public-entry T-HUFF/T-INFL theorem.

The current core_steps and seg_steps are still runtime u128 diagnostics. Count/first-code helpers return runtime usize loop tallies to the M1a caller; they are not erased Ghost<nat> observations. Conversion and work proofs remain open. The first_codes lengths borrow is used only in specifications; remove that executable observation argument in favour of a ghost view when the verified constructor can supply it. huffman_format_ok proves only format/type facts, not PNG resource-cap caps_ok. A 32-bit verifier replay has not been performed.

## Cache and allocation boundary

The dependency now exactly matches GIF: vstd `=0.0.0-2026-11-01-0251`, default features off, alloc enabled for the ghost/spec library. GIF target/debug was copied intact into this crate without deleting the destination. Verified artifact hash `01822e92e4f44378` has .vir SHA-256 `49934e1761f92df9d380921b59b8cae9c6ecbe92768ffbafc3f57820629ca533` and .rmeta SHA-256 `6cf84fc37d8dac1e1cd2fa9b0308eca6f4ae0e9ff96a3cb329f8b776547ce618`. No verifier flag or solver limit changed.

Run 60 checks host and freestanding thumbv7m release libraries plus separate probes that instantiate both real public entry points with SegmentSlices; none contains alloc::, mangled alloc or allocator symbols. The probes prevent a generic-only library's small symbol table from making the check vacuous. This is a build/symbol falsification gate, not a signed call-graph audit or evidence about arbitrary downstream Segments implementations.

The literal frozen phrase 'no alloc dependency' remains a wording boundary: vstd/alloc is enabled as approved, although shipped inflate code has no allocator references. If that phrase forbids even an erased ghost-library feature, the dependency-feature build clause is not met. No frozen file was edited, and no full A-PNG-6 claim is made; the coordinator must reconcile that wording before sign-off.

Runs 58 and 50 pass ten release tests, all 1,181 extracted streams in four layouts and 1,000 generated streams plus 1,000 single-bit corruptions, with zero unexpected differences. Runs 57–61 recheck the host/thumb builds, instantiated allocator-symbol gate, source/cap controls and all twelve frozen hashes after replacing vstd array setters by direct core array writes. The accepted M1a 10,000-stream evidence remains historical; it is not claimed as a new 10,000-stream replay after proof refactoring.

Keep the Stage 2 IgnoreTail discrepancy below. No commit, forbidden proof escape, frozen edit or background process was introduced.

---

# Historical no-alloc vstd bootstrap blocker

2026-11-17. Verified count remains **zero**. Verifier run 19 exits 101 before checking this crate: `--is-vstd` is not allowed with `--no-cheating`. The newly pinned vstd dependency disables default features and enables neither alloc nor std, as the frozen manifest requires. Every verified GIF vstd cache found has alloc enabled; copying one under a different feature fingerprint would not supply a matching no-alloc artifact.

The required missing verification artifact has hash `03bc18a84b05fe12`, metadata `b88a4c9d341b7830`, for vstd `0.0.0-2026-11-01-0251`. Run 19's complete command and failure are in `/home/ws/projects/verified-decoders/png/inflate_verus/runs/0019.log`. A compatible prebuilt no-alloc vstd verification cache must be supplied, or the coordinator must establish an approved bootstrap procedure. I did not weaken `--no-cheating`, enable alloc, change the verifier or trusted library, forge a Cargo fingerprint, or delete any target/debug directory.

M1a is backed up at `/home/ws/projects/verified-decoders/png/inflate_verus/backups/m1a-before-m2.tar`, SHA-256 `71b4717543b6b88c8451ad7518e115f02a6d59697bb124cabedb2bcfa6e97466`. Its source hash manifest remains historical; it no longer describes the current M2 working tree.

Current source progress is not a proof:

- `src/model.rs` independently defines Kraft, admissibility, the RFC next_code recurrence, canonical codeword bit sequences, prefix predicates and D4 MDec with error positions; it never reads executable tables or Python code.
- `src/huffman.rs` separates `build_canonical` from `build`'s dynamic admissibility decision, so non-oversubscribed but inadmissible vectors receive their canonical representation before rejection.
- Three new Rust tests check incomplete-vector representation, oversubscription before representation, and all 15,625 six-entry length vectors with entries 0..4 against independent word assignment.
- Run 20 passes those three tests and the seven existing contract tests; run 21 only macro-parses the ghost module with ordinary Rust, not Verus.
- Runs 22–24 pass source/cap controls, all twelve frozen hashes, the release adapter, all 1,181 extractor streams across four layouts and 100 generated-stream/corruption regression pairs.
- Runs 25–28 format the source and pass ten release tests, the no-alloc freestanding thumbv7m library build, source controls and all twelve frozen hashes.

The proposed `lemma_pow2_positive` and `lemma_count_bound` have not been checked by Verus. T-HUFF, L-CANON, L-PREFIX-FREE, MDec refinement and every later layer remain unproved. Diagnostic counters and Segments contracts have not yet been converted; do not cite them as Ghost counters or verified contracts.

Resume by supplying the matching no-alloc vstd cache, then rerun `/home/ws/projects/verified-decoders/png/inflate_verus/verify.sh`. Resolve model type/termination obligations first, then convert and prove canonical construction and admissibility, prefix-freeness, representation and decoding. Do not start the block/window layer before the Huffman handback.

## Stage 2 policy-oracle note

The implementation follows the manifest's IgnoreTail condition: only exactly-L output with a matching Adler-32 may ignore trailing compressed bytes. The direct non-frozen policy-oracle wrapper ignores those bytes even when output is short. For example, a valid empty zlib stream followed by one zero byte at L=1 is `TrailingZlibData` here but succeeds with empty output in that policy wrapper. No frozen Strict statement is contradicted; this note is for Stage 2 reconciliation, not a proposed frozen erratum. No errata or release-gate file changed.

---

# Historical M1a checkpoint

2026-11-17. Plain Rust runtime checks pass. **Verified count: zero. No lemma name or file location can be cited as a completed proof.** No frozen statement has been shown unsatisfiable; no erratum was written or applied.

The new crate lives at `/home/ws/projects/verified-decoders/png/inflate_verus`. Only this directory was created or changed by this task. There is no `model.rs`, Verus dependency, `verus!` block, proof stub, solver-limit override or new trusted item. No existing verification cache was removed.

## Evidence available

- Runs 12–13 test the final-source decoder against Python zlib on 5,000 generated streams and 5,000 single-bit corruptions per seed, with seeds 1 and 2 and zero unexpected differences.
- Each differential run compares all 631 frozen extractor streams and the 550 cap/policy extractor streams across four segment layouts.
- Each runner request calls both public entry points, compares their observations, checks the complete written prefix and untouched suffix, checks last-window contents, and asserts the two observed work bounds.
- The five D4 cases, empty `Z`, eight extractor-presence cases, D5 stored-prefix cases, Adler deferral, wrapping matches, repeated EOF and one million empty segments execute successfully.
- Run 17 passes both GIF allocation-scanner modes, its 44-construct controls, the cap-lint controls and all twelve frozen hashes.
- Runs 15–16 build the final dependency-free no_std library for i686 and freestanding thumbv7m; runs 10 and 14 pass seven Rust contract tests in debug and release builds.

The runner bounds the output buffer for 278 extractor streams with invalid or enormous raw shapes, including seven frozen malformed-header/container records. Those are non-R runtime comparisons, not M2 gates. Every inflate-specific M2 record uses its actual image R. Policy records labelled ImageCrateCompat currently exercise Strict inflate, not compatibility semantics. Forty-four authored frozen inflate errors are checked directly; other raw-byte observations come from the frozen reference, not authored expectations.

## Precise resume point

1. Preserve the M1a source snapshot before converting files. Add the pinned vstd version `=0.0.0-2026-11-01-0251` without its alloc feature. Do not copy GIF's alloc-enabled dependency configuration into this crate. Inspect the prebuilt vstd hash and feature compatibility before copying `/home/ws/projects/verified-decoders/gif/gif_verus/target/debug`; never delete any `target/debug` directory.
2. Start the Huffman layer. Add an independent mathematical `Kraft`, `admissible`, `cw` and prefix-match `MDec` in `model.rs`. The model must not read executable counts, symbol arrays, ring state, transitions or the Python oracle. Define canonical words by their own RFC recurrence over the length sequence.
3. Prove exact Kraft accumulation and admissibility, then L-PREFIX-FREE and L-CANON, then dead-prefix/truncation decoding. The current `Huffman::build` populates first codes and symbols only after admissibility succeeds. T-HUFF(a) also concerns non-oversubscribed but inadmissible vectors; separate canonical representation from the dynamic admissibility decision, or populate the non-oversubscribed representation before returning that rejection. This is implementation work, not a frozen-spec contradiction.
4. Convert `Bits` and `Segments` to verified contracts without narrowing their input domain. T-SEG needs immutable ghost `concat`, ghost cursor/segment views, `wf`, fresh-cursor requirements and both per-read work/termination clauses. `Bits` currently caches only one byte; its ghost bit position is `8*src.pos()-remaining` in `nat`, never a u32 position.
5. Continue through block/window, T-INFL/limit/Adler, segments/work, then allocation facts and independent witnesses, with a handback after each assigned layer.

Use `/home/ws/projects/verified-decoders/png/inflate_verus/verify.sh` for every verifier run. It currently refuses this plain-Rust tree rather than reporting a misleading green zero-obligation result. It bounds verification at 25 minutes, sets one cargo job, uses two verifier threads and `--no-cheating`, and checks a shared lock plus already-running legacy verifier commands. No verifier run has occurred.

## Gaps that must not be hidden

The current `core_steps` and `seg_steps` fields are runtime `u128`, not erased `Ghost<nat>`. Move observations into ghost state at the actual loops; do not verify a wrapper that changes production behaviour. Observed inequalities are not T12. `caps_ok`, symbolic proof use of caps and CAPS-SYM remain open; the six PNG caps do not belong in this crate. The cap lint is a lexical falsification gate, not a constant-expression proof.

Public `inflate_zlib_measure` requires `ring.len() == WINDOW_MAX`; it panics for a caller that violates that documented precondition. M2 must prove panic freedom under the exact permitted preconditions, not silently strengthen them. `Segments` is currently a prose contract, so arbitrary unverified trait implementations do not carry a universal correctness guarantee.

The implemented policy domain contains only Strict/RecommendedBeta field values, not all Stage-2 tuples. `IgnoreTail` requires exactly-L output after a matching checksum, per the manifest. The current policy oracle's direct wrapper ignores a trailing stream even for a short output; the test wrapper deliberately follows the manifest's restriction. No frozen Strict rule depends on this non-frozen policy discrepancy.

There is no independent evolving sequence-output inflate model, T-INFL public-entry refinement, L-WINDOW/L-RING/L-LIMIT/L-ADLER proof, safety/termination proof, symbolic work proof, model-versus-authored witness proof, or verifier mutation kill. A-PNG's no-alloc property has only source/dependency/build evidence here; the PNG crate's reservations and sealed log belong to later work. No signed production call-graph audit or physical/logical allocation audit is claimed.

The original long differential run completed, but the tool harness automatically detached it after three minutes despite a foreground request. It exited successfully and left no process running. Final-source runs were split into bounded foreground shards below that threshold. No background job remains; no commit, durable /tmp content, erratum edit or release-gate edit was made.
