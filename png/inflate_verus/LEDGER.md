# Inflate proof ledger

## M2 signed off, 2026-11-19

The coordinator signs off M2 on the evidence below. The Astra M2 sign-off review (`png/REVIEW-ASTRA-M2.md`) found one blocking item, the absent kill for mutation 39. Run 613 now kills that mutation: `stored_payload` fails two postconditions when it resets the stored prefix on truncation. The review confirmed every other M2 row and gate record. Fresh split checkpoints on the final source (tree sha256 `f56fa5f7…c5df`) verify 1,381 unique items with zero errors on both targets: host runs 617–622 and i686 runs 623–628. The audit tests pass (46 in run 629), as do the provenance controls (run 630). Coverage stands at 615 of 631 extracted core stream instances. The other 16 have no legal independent R and are excluded by council decision. L-WALK-FIRST, L-INFO-RANGE, L-SEG-PNG, per-reader L-SEG-FRAME and the integration and allocation gates remain open M3–M6 obligations.

Correction, 2026-11-21 (E7): this sign-off does not mention the runtime gate of `THEOREMS.md:264` ("from M1b on all 810 records"). When M2 was signed off, the latest `png_verus` conformance result was 808 of 810 records passing and 2 ImageCrateCompat-0.25.10 records skipped. E7, approved by the owner on 2026-11-21, makes those two records gates of P2. The M2 sign-off stands on the evidence it lists; it does not claim, and never claimed, that all 810 records pass.

## M2 review findings closed; coordinator sign-off pending, 2026-11-19

Mutation 39 is killed in run **613**: base pass, **371 verified, 1 error**, Verus 0.2026.11.02.8bf68ee, unchanged no-cheating/default limits. The disposable implementation resets `output.produced = 0` only on stored-payload EOF and retains `Err(Status::Err(e))`; the model, contracts and witness expectations are unchanged. Verus rejects `stored_payload` postconditions `final(output).wf()` and `old(output).produced <= final(output).produced` at the error return. The retained log is `evidence/run-0613.txt`; the exact mutated source is `backups/m2-review-stored-truncation-drop-prefix.tar.gz`.

`witnesses::truncated_stored_payload` supplies two present bytes `[0,65]` followed by missing payload. Both public entries must report TruncatedZlib and produced=2 at limits 4 and 2; the mutant reports produced=0. At limit 1 the unchanged branch returns Full with `[0]`. The frozen empty-prefix cut-mid-stored fixture is unchanged and is not the observability argument.

The requirement-indexed table is `evidence/mutation-index.json`: rows 1–40, exact selectors/revisions/failed functions/diagnostics/logs for the fourteen M2 requirements, and 52 additional mutations listed separately. Mutations 13–36, 38 and 40 are explicitly not M2; run 453 is excluded as type-check-only. The historical total is 68, including mutation 12 as **ghost representation narrowing**, not an executable u32 counter experiment. Historical `evidence_commit` values identify the commit that recorded a working-tree run, not a retroactive claim that the run used that clean commit; the affected implementation snapshots are retained and hashed. Mutation 39 additionally has an exact disposable source archive and source-tree hash.

The audit now checks numbered coverage, selector definitions, implementation snapshot hashes, semantic proof diagnostics and future-milestone scope without reading `runs/`. Nine new negative controls pass. `adler::lemma_adler_nmax_maximal` and its doubled-bound spec function are added; they assert safety at NMAX, failure at NMAX+1 and failure for every larger n. The E2 and header-coding-cost comments now describe the established proof state. Production runtime bodies are unchanged.

Handoff after the clean host milestone: **runs 617–622 verify 1,381 unique functions, 0 errors**, on tree `f56fa5f7b8d7ac91cd1ef93a7715c0fccff8fce128e65c930fc6b7f26ff1c5df`. Pass counts are 374, 813, 679, 519, 454 and 412; plain sum 3,251 minus five repeated base copies (1,870) gives 1,381. Total verifier time is 1,327.95 seconds (22 minutes 8 seconds). Each pass has a fresh JSON report and the checkpoint is retained as `evidence/checkpoint-0617.json`. The two added verified functions are the Adler bound specification and maximality lemma. Earlier checkpoint 592 and monolithic 582 no longer bind this source and are historical evidence only.

Run 614 caught a nat/int specification type mismatch; run 615 proved the corrected base (374 verified, 0 errors). I stopped pass 616 before checkpoint completion to add proof-only crate-root re-exports; neither 614 nor 616 is a final proof checkpoint or mutation kill. The requirement index and its nine negative controls are complete.

Handoff after the target milestone: **i686 runs 623–628 also verify 1,381 unique functions, 0 errors**, with exactly the host's six counts and source hash. Every pass has a fresh JSON report, names `--target i686-unknown-linux-gnu` and the pinned `libvstd-a8222d889f5a22f0.rmeta`, and reuses `Fresh vstd`; the cache was not rebuilt. Target verifier time is 1,426.08 seconds (23 minutes 46 seconds). `evidence/checkpoint-0623.json` retains the replay. The before/after snapshots `evidence/m2-review-host-cache-{before,after}-i686.json` agree on hashes, sizes and mtimes of all **2,204** files under `target/debug`; the target replay did not write the host cache. Both verifier jobs have exited. **All 46 audit tests pass in run 629**, including nine new mutation-index controls, complete audit main and the runs-free evidence checkout. **Run 630** passes the complete source/provenance/allocation/cap/erased-work controls, all twelve frozen hashes and structural split coverage. Logs 629/630 are retained with hashes in `evidence/manifest.json`.

The review's blocking mutation-39 item is closed, together with its four requested non-blocking repairs. This is **not M2 sign-off**; the coordinator must check the evidence and decide. The optional executable narrowing mutant was not added: mutation 12 remains explicitly qualified as ghost representation narrowing. Production runtime bodies did not change, so complete runtime evidence 566–572 remains applicable without a rerun. The original 615-of-631 count and council exclusion of the other 16 remain unchanged; L-WALK-FIRST, per-reader framing/call binding, integration and allocation gates remain M3–M6 obligations.

The mutation archive now contains every file in run 613's split source inventory plus examples/tests and the mutation selector, but omits unrelated large authoring data; the audit reconstructs the recorded tree hash from the archive. Initial audit development exposed the need to parse JSON-only mutation counts and to keep this archive below the existing one-million-byte bound; both are fixed. Verus 0.2026.11.02.8bf68ee, `--no-cheating`, default solver limits, two verifier threads and one Cargo job are unchanged. No trusted item, proof escape, runtime-body edit, frozen/review/errata edit, target cleaning, commit/push or edit outside this crate occurred. All started jobs have exited. **Next logged run: 631.**

## Council decision: the sixteen instances without a legal independent R, 2026-11-19

Decided by the council (option A, 4 of 4 in round 3; sealed report `/home/ws/projects/council/sealed/COUNCIL-REPORT-PNG-M2R.md`, sha256 `49827654a4a203a1931924d07e5c629bb6c6c25074caa22e73b986e8e80f1c48`). The recorded wording is the fable seat's round-3 text, verbatim, followed by the astra seat's scope sentence:

M2 witness accounting, the sixteen instances without a legal independent R. This entry settles how M2 sign-off counts these sixteen instances and nothing else; it does not establish that the named M2 gates are complete. The frozen M2 witness gate is the record list at THEOREMS.md:193, and M2 sign-off separately requires every stream of every record named there to carry a model-versus-expected proof and a call of both public inflate entries at L = R, together with the T-SEG, window-size and INFL(RB) gates listed there. The 631-instance figure is the proof team's supplementary target, not the frozen gate. Of the 631 extracted core stream instances, 615 carry such a witness, with R computed from the record's IHDR or fcTL by tools/audit_witnesses.py independently of any decoder or oracle. The audit derives no legal R for the other 16: inventory indices 364, 381, 384-396 and 616 (two container/*, thirteen ihdr/* and one apng-bad/* record; index 616 is stream 1, the fdAT stream, of apng-bad/fctl-width-0). None of these records is named at THEOREMS.md:193; all are M3 gate records under THEOREMS.md:194. 'No legal R' is the audit's classification (not exactly one 13-byte IHDR, an IHDR field that fails P4, or a frame size outside 1..2^31-1); it is not a claim that the raster formula is undefined on every such header. The reference oracle computes no R for these records, and the frozen specification rejects each record in the walk before it decodes the stream. These 16 instances are outside the supplementary target by council decision. They are not witnessed, not resource-blocked and not credited: no fallback limit was assigned (neither R_p = 0, nor the formula applied to malformed fields, nor the generator's 14 and 18), and no witness of a byte-identical stream in another record is counted for them. For information only: their compressed bytes equal streams witnessed in 136 other instances at R = 14 (indices 364-396) and in 23 other instances at R = 18 (index 616). Of the 615 witnessed instances, 164 are in records whose frozen Strict expectation is a walk error; those witnesses remain in the numerator as standalone inflate outcomes at the audit's R, not as evidence that PNG decoding reaches inflate on those records. Coverage is reported as '615 of 631 extracted core stream instances witnessed; the other 16 have no legal independent R and are excluded by council decision', always with all three numbers. The ledger must not report 631 of 631, 615 of 615 without the 16 and the 631 beside it, 100 per cent of core streams, a solver-resource blocker for the 16, or whole-PNG conformance on any record from M2 evidence. The universal inflate refinement keeps its stated domain of streams, limits, supported option tuples and verified target. Open obligation L-WALK-FIRST, an M3 sign-off gate: for every input, under Strict each public PNG entry makes no inflate call and reserves no Raw buffer unless the walk (MW, or MWF_p for the constructor) succeeds; under every other claimed profile, each entry inflates only the streams that the profile's specified outcome decodes, and in particular never inflates the fdAT data of a frame whose fcTL failed its checks (under RecommendedBeta's static fallback the valid default image may be allocated and decoded; the rejected frame's stream may not). The frozen expectations of the 16 records are M3 witness gates under THEOREMS.md:194. M6 sign-off must establish that the integrated public PNG paths preserve the walk, allocation, rejection and recovery behaviour proved at M3. The ledger must link L-WALK-FIRST and each of these gates to its evidence and say which part Verus checks and which rests on inspection or runtime tests. M2 sign-off discharges none of them; they stay open until evidenced and block M3 or M6 sign-off, not M2.

This decision permits M2 sign-off on this point only, subject to all remaining M2 obligations.

---

## Pinned i686 verifier replay, 2026-11-19

**i686 split checkpoint 603–608: 1,379 unique verified functions, 0 errors**, on the host checkpoint's source tree (tree hash `edcde0f4…6020`). Every pass count equals host checkpoint 592–597, so the unique total equals it too. The owner authorised building the missing i686 vstd cache once; `I686-CACHE.md` records how to rebuild it.

| Runs | Result |
|---|---|
| 601 | `target/debug` snapshot before: sha256, size and mtime of all 2,179 files |
| 602 | i686 vstd cache built in `target/i686-verify/`: vstd **1,950 verified, 0 errors** (same as the host build), `libvstd-a8222d889f5a22f0` |
| 603 | Pass `base` on i686: **372** verified, 0 errors, 27.86 s |
| 604 | Pass `config-cases`: **811** verified, 0 errors, 744.74 s |
| 605 | Pass `extended`: **677** verified, 0 errors, 294.07 s |
| 606 | Pass `classes-blocks`: **517** verified, 0 errors, 157.68 s |
| 607 | Pass `dynamic`: **452** verified, 0 errors, 103.47 s |
| 608 | Pass `fixed-policy`: **410** verified, 0 errors, 56.15 s |
| 609 | `target/debug` snapshot after: identical to 601 |
| 610 | Audit tests: one new test failed on its own edit string (expected `--` after the target triple; rustc's next token is `-C`); test fixed, no audit change |
| 611 | All **37** audit tests pass: the 33 existing tests and four target-replay negative tests |
| 612 | Source/provenance/cap controls and the split coverage check pass |

### How the cache was built

The host cache `libvstd-01822e92e4f44378` comes from GIF run 1, a plain `cargo verus verify`: cargo-verus verifies vstd with `--is-vstd`, exports its `.vir`, and forwards no Verus arguments to it. A `--no-cheating` pass cannot build vstd, because `--is-vstd` is refused under `--no-cheating` (run 19); the host passes only work because cargo reuses the cached vstd. Run 602 (`tools/build_i686_vstd.sh`, under the verifier lock and the 25-minute bound) repeats that build with `CARGO_BUILD_TARGET=i686-unknown-linux-gnu` and `--fwd-verus-args-to roots`, so vstd again receives no forwarded arguments. The root crate receives only `--no-verify` and yields no count. Verus, vstd, rustc, `Cargo.lock` and vstd's features (`alloc` only) are unchanged. `CARGO_TARGET_DIR=target/i686-verify` keeps cargo's host-side proc macros and build scripts out of `target/debug`. The cache directory holds 273 MB: 210 MB of host-side units and 78 MB of i686 units. The `.vir` has 45,171,447 bytes and sha256 `2f705972…343f`.

### How the replay ran

`verify.sh`, `verify_split.py`, `split_checkpoint.py` and `run_logged.py` are unchanged, so the source tree hash and each pass command equal the host's. The target and cache directory come from the environment (`CARGO_BUILD_TARGET`, `CARGO_TARGET_DIR`). `CARGO_TERM_VERBOSE=true` makes each log show the `inflate_verus` rustc line with `--target i686-unknown-linux-gnu` and `--extern vstd=…/libvstd-a8222d889f5a22f0.rmeta`, plus `Fresh vstd`. Flags, bounds, flock, legacy-invocation check and one-at-a-time rule are unchanged. No cargo replay occurred: target/i686-verify had no earlier result for any `split*` feature, and each pass log holds its `--output-json` report.

The counts match exactly, as expected: the crate does not fix `size_of usize`, so Verus already proves every obligation for a 32- or 64-bit `usize` on the host, and neither vstd nor the crate has a `target_pointer_width` or `target_arch` cfg.

### Evidence and checks

Logs 601–609, 611 and 612 and `evidence/checkpoint-0603.json` are retained in `evidence/` with hashes. The checkpoint record has the new role `target-replay-count`. For it, `tools/audit_witnesses.py` runs the existing split-checkpoint validation and also requires the source inventory and unique total of the current final count. It also requires, in every pass log, one verbose `inflate_verus` rustc line carrying the recorded target and vstd `.rmeta`, `Fresh vstd`, and no `Compiling vstd`. The four new tests reject a pass compiled without the target, a different vstd, a rebuilt vstd, and a different source tree.

`target/debug` was not written: listings 601 and 609 agree byte for byte below their command lines, and `libvstd-01822e92e4f44378.vir`/`.rmeta` still hash to `49934e17…a533`/`6cf84fc3…e618`, equal to the GIF copy. **Final counts: host checkpoint 592–597 and i686 checkpoint 603–608, each 1,379 unique verified, 0 errors. Tests: 611; controls: 612. Next logged run: 613.** No frozen, review or errata file, proof or source file, commit, push or edit outside the crate occurred.

---

## Split verifier checkpoints, 2026-11-19

**Split checkpoint 592–597: 1,379 unique verified functions, 0 errors, on one source tree** (tree hash `edcde0f4…6020`). This equals monolithic run 582. The six passes take 22 min 38 s in total; the longest takes 12 min 39 s, against 23 min 02 s for the single run 582 under the same 25-minute bound. No proof changed, and core coverage stays **615 / 631**.

| Runs | Result |
|---|---|
| 585 | `cargo verus verify` refuses `--verify-module`/`--verify-root`: "Partial verification must use `cargo verus focus`" |
| 586 | `cargo verus focus`: "`--verify-root` is not allowed with `--no-cheating`" (focus also builds vstd afresh under `target/verus-partial/`) |
| 587 | "`--verify-module` is not allowed with `--no-cheating`" |
| 588 | Base pass in a disposable copy: 372 verified, 0 errors, 29 s; `--time-expanded` names only the three slowest modules |
| 589, 590 | Cargo replays of unchanged sources: no verifier output (fails closed) |
| 591 | Base pass in a disposable copy with `--output-json`: per-function report, 372 verified |
| 592 | Pass `base`: **372** verified, 0 errors, 28.58 s |
| 593 | Pass `config-cases` (`witness_config_cases`): **811** verified, 0 errors, 759.19 s |
| 594 | Pass `extended` (`witness_extended`): **677** verified, 0 errors, 298.28 s |
| 595 | Pass `classes-blocks` (2 modules): **517** verified, 0 errors, 133.79 s |
| 596 | Pass `dynamic` (6 modules): **452** verified, 0 errors, 87.37 s |
| 597 | Pass `fixed-policy` (6 modules): **410** verified, 0 errors, 50.46 s |
| 598 | All **33** audit tests pass: the 27 existing tests and six split-evidence negative tests |
| 599 | Source/provenance/cap controls and the split coverage check pass |
| 600 | Plain `cargo test --all-targets` passes with the gates present |

### How passes select modules

The pinned Verus cannot select modules under `--no-cheating`, so passes select modules at compile time. `src/lib.rs` adds one `#[cfg(any(not(feature = "split"), feature = "split-PASS"))]` line under the existing `#[cfg(verus_only)]` of each of the 16 leaf witness modules (modules that no other module imports). `Cargo.toml` adds a `[features]` table with `split` and the five `split-PASS` features. These are the only source changes. With the gate lines removed, `src/lib.rs` hashes to run 582's `e745b40b…`; with the table removed, `Cargo.toml` hashes to 582's `0d588588…`. Every other `src/*.rs` file is byte-identical to run 582. A build without features compiles every module, so a feature-free build is the same program as run 582's.

Pass `base` (feature `split`) verifies 23 ungated modules: the crate root, the ten core modules and the twelve imported witness modules. Each leaf pass verifies base plus its own leaves. Every pass adds `--output-json`, which prints the verification results and lists every function of the crate. This is the only flag added; `--num-threads 2`, `--no-cheating`, `CARGO_BUILD_JOBS=1`, default solver limits, the 25-minute bound, the flock and the legacy-invocation checks are unchanged. `run_logged.py` turns the JSON counts into the usual `verification results::` RUNLOG summary.

### Count reconciliation

The plain sum of the six pass counts is 3,239, which counts base's 372 functions six times (5 × 372 = 1,860 extra). The unique total is 372 + (811 − 372) + (677 − 372) + (517 − 372) + (452 − 372) + (410 − 372) = 372 + 439 + 305 + 145 + 80 + 38 = **1,379**. The subtraction is valid because the audit checks that every pass lists the identical function set for each base module. The audit compares the unique total with monolithic reference 582 whenever the reference's sources are equivalent in the sense above, and rejects any difference.

### Evidence and checks

`tools/verify_passes.json` is the checked-in pass manifest, and `evidence/checkpoint-0592.json` records the per-pass durations that justify the grouping. `config-cases` holds the single module `witness_config_cases` and cannot be divided further without splitting that source file; at 12 min 39 s it is under 15 minutes but is not well under. `tools/split_checkpoint.py --check` fails if any module containing `verus!` code is in no pass or in two passes, if a manifest module has no `verus!` code, or if the manifest, the gates in `src/lib.rs` and the Cargo features disagree. Each pass command carries `VERIFY_SOURCE_TREE=<hash>` over all 46 bound files: `src/*.rs`, Cargo files, `verify.sh`, the manifest, `verify_split.py`, `split_checkpoint.py` and `run_logged.py`.

`tools/audit_witnesses.py` accepts a split checkpoint as the final count. Pass logs 592–597 and the checkpoint record are retained in `evidence/` with hashes, and selector diagnostics 585–587 are retained as well. Run 582 becomes `monolithic-reference-count`. The audit checks that every manifest pass is present in order, each pass log is bound to the checkpoint's tree hash, each report shows 0 errors and `is-verifying-entire-crate`, each pass lists its own modules and base and no other pass's leaves, base function sets are equal across passes, and the stated sum and unique total are correct. The six new tests reject a missing pass, a pass with errors, a pass on a different source tree hash, a cargo replay, a unique total that differs from the monolithic reference, and an uncovered module (a manifest module dropped, and a new `verus!` module in no pass).

`tools/proof_mutations.py` still calls the copied `verify.sh`. It now bounds the whole sequence at six × 25.5 minutes, and the driver stops at the first failing pass. A rerun on an unchanged tree replays cargo's cache and fails closed, as run 475 already did for monolithic runs, so a fresh checkpoint needs a changed Rust source. Disposable probe copies were removed. The focus probe left a new `target/verus-partial/` (172 MB). `target/debug` was not touched; its vstd `01822e92e4f44378` files still match the GIF copy. **Final count: split checkpoint 592–597, 1,379 unique verified, 0 errors. Tests: 598; controls: 599. Next logged run: 601.** No frozen, review or errata file, commit, push or edit outside the crate occurred.

---

## Round 7 recovered: shared default-limit queries, 2026-11-19

**Fresh final-source run 582: 1,379 verified, 0 errors.** Core coverage is **615 / 631**, up by **23** from 592. All 21 first-fixed-block gaps and both remaining dynamic profiles now pass at default solver limits. The only sixteen uncredited core instances lack a legal independent IHDR/fcTL R. No legal-R stream remains solver-resource blocked. This is a clean partial M2 checkpoint, not M2 sign-off.

| Runs | Result |
|---|---|
| 562, 573 | Whole-batch 25-minute wall-clock timeouts; no verification-results line or query-rlimit diagnostic; not success evidence |
| 574 | Shared-query exact public-call bindings and three cached-parser duplicate guards pass |
| 575 | **1,379 verified, 0 errors**, 1,306.68 seconds; source-bound evidence retained |
| 576 | Full audit exposed a local `chunks` list shadowing the PNG chunk iterator; repaired without changing proof source |
| 577 | Initial 26 negative/checkout tests pass |
| 578 | Full source/provenance/allocation/cap/erased-work controls and twelve frozen hashes pass |
| 579 | All **27** audit tests pass, including full audit main and runs-free evidence checkout |
| 580 | Length-value-plus-one mutation killed: 1,378 verified, 1 error |
| 581 | Legal window-equality rejection mutation killed: 1,378 verified, 1 error |
| 582 | **Fresh final replay: 1,379 verified, 0 errors**, 1,381.77 seconds |
| 583–584 | Final complete source controls and all **27** audit/negative/checkout tests pass |
| 566–572 | Complete fresh runtime replay passes |

### Exact added witness surface

The fixed-first-block additions cover `zlib/level1-default`, `level1-fixed`, `level1-filtered`, `level6-fixed`, `level9-fixed`, `level6-filtered`, `level9-filtered`, `window-9`, `window-10`, `window-12`, `sync-and-full-flush-empty-stored-blocks`, and `cut-mid-block`; `image/truncated-data`, `excess-data`, `excess-row`, `filter-type-5`, and `filter-type-255`; `precedence/adler-before-filter`; `interlace/1x1-filter-bytes-for-empty-passes`; and both `deflate/distance-*-declared-window` records. These are 21 core instances, not 21 new physical constructors: the three level-one records share one representative only after exact Z, independent R, raw intent and option/result equality checks.

The complete dynamic additions are `depth16/rounding-boundaries` at R=25 and fdAT stream 1 of `apng/interlaced-subframe` at its own fcTL R=42. Their Huffman tables, repeat fields, words, length/distance extras and copy effects come from forward sample/encoder intent, not a decoder or frozen expected fields. The flush witness composes five authored blocks; the EOF witness proves the independent 552-byte prefix and a live-code truncation.

Twenty-one fixed configuration classes now cover nineteen core instances and eight policy records, with two further dynamic and two block classes. The four new policy families are excess-output, excess-row, excess-output-and-tail, and distance-beyond-declared-window, each with strict/recommended-beta records and all eight `(R or R+1, enforce-window, ignore-tail)` tuples. Together with the ten earlier policy records, eighteen policy records are proved outside the core numerator. These are inflate outcomes, not integrated PNG policy results: an excess-output record can be Done at R+1 while its PNG image policy still rejects the extra byte.

Both declared-window core records use their actual headers: distance 300 is rejected after 400 bytes with the declared 256-byte window and accepted with the declared 512-byte window. The policy counterpart proves the 256-byte/32,768-byte selection at both limits and both tail choices. The empty-Adam7-pass fixture remains Full at R=2, not Done on its eight authored bytes.

### Repairs, evidence and current handoff

All six interrupted proof modules remain active and now verify. Run 553 failed two empty stored-range preconditions and then the pinned Verus undeclared-MDec-fuel panic; 555 repaired those issues but left six dynamic match/EOB assertion failures. Concrete z/raw length facts close those six failures. Direct fixture-function compute expressions eliminate run 562's 102 local-alias simplification warnings. Shared output-prefix, block-result and finishing subqueries, unused-checksum removal and the level-one representative let the complete crate finish inside the unchanged 25-minute bound. No solver limit increased.

The audit checks every exact-call descriptor and now indexes function declarations without weakening duplicate rejection. Full audit run 576 additionally found the interrupted main-path iterator shadowing bug; `trace_ranges` avoids that shadow, and the 27th test executes the complete audit pipeline. All required logs are retained under `evidence/`, with hashes and run 582's complete 42-file source/Cargo/verify.sh binding; 63 retained logs are checked without reading ignored runs/. Twenty-three round-7 rlimit records moved from `tools/witness_blocked.json` into `evidence/historical-blocked.json`, with `resolved_run: 575`; the current blocked list is empty. Required 543/549 archives remain retained. Six unreferenced backup files were pruned, reclaiming 2,061,648 bytes.

Production runtime bodies match 5f697fa. Fresh runtime evidence 566–572 includes thirteen release tests, the adapter, instantiated host/thumb allocator probes and symbol checks, all 1,181 extracted vector streams in four layouts per seed, 10,000 generated streams and 10,000 corruptions, with zero unexpected differences. The two seeds check 33,544 public-entry pairs / 67,088 actual calls. Work is erased and proved, not measured. No signed allocation audit is claimed.

Run 580 kills `length-extra-value-plus-one`: **1,378 verified, 1 error**, with compressed-step refinement and match-length-range diagnostics, not a type-check or resource failure. Run 581 additionally kills `window-boundary-equality-rejected`: **1,378 verified, 1 error**, at the match-copy/model postcondition when a legal `distance == window` is rejected. The historical mutation total is now **67**. Its 776,153-byte source snapshot is `backups/round7-length-extra-value-plus-one.tar.gz`; the retained log and archive both have manifest hashes. Disposable failed source files were removed, while their target link and the shared cache were left untouched. The final unchanged-main replay 582 reconfirms 1,379 verified, 0 errors after both implementation mutations. These totals include mutation 12 as ghost representation narrowing, not an executable narrowed-counter experiment.

The interrupted `distance-extra-value-plus-one` selector was backed out because it duplicated historical `distance-wrong-base`; aliases earn no new kill. Both new mutations used the optional `--case-root` to avoid the untouched pre-existing `mutation-cases/`. The second exact source archive is `backups/round7-window-boundary-equality-rejected.tar.gz` (776,154 bytes). Both new mutation sources are retained as sub-1-MB archives with log/archive hashes; disposable failed source and Cargo files are removed, retaining only target/log links. The pre-existing untracked mutation cases remain untouched. **Final accepted count: run 582, 1,379 verified, 0 errors. Final controls: 583; all 27 tests: 584. Next logged run: 585. All jobs have exited; no failing or speculative proof remains active.**

Pinned i686 replay remains blocked by the absent matching vstd cache (fresh inventory 565); no bootstrap or target substitution occurred. E1 remains APPROVED; E2 remains APPROVED WITH AMENDMENT. The existing SegmentSlices L-SEG-FRAME discharge, L-LIMIT-MONO and separate M3 IDAT/fdAT/iCCP reader/call-site obligations are unchanged. No new frozen-statement concern was found. Frozen/review/errata files and pre-existing mutation/review evidence are unchanged. No target cleaning, vstd cache repair, solver-flag change or trust addition occurred. No commit/push or edit outside this crate occurred.

---

## Round 7 recovery diagnostics (historical), 2026-11-19

At recovery start, run 529 remained the only accepted checkpoint: 831 verified, 0 errors and 592/631 core instances. Runs 562 and 573 later hit the whole-batch 25-minute wall-clock bound without a count or query error. Run 562 also printed 102 failed-to-simplify warnings from local-alias compute expressions; direct fixture calls eliminated them in 573. Shared subqueries then produced clean run 575, followed by fresh final-source replay 582. Neither wall-clock timeout is used to label an individual stream solver-rlimit blocked.

Run 542 failed on a missing generated module; 547 rejected `hide` declarations placed after ordinary statements. Runs 543 and 549 exhausted default limits in large concrete configuration/profile/flush queries. The later decomposition separates constructors, trace chunks, balanced trace joins, outputs, result walks, checksums, model composition and public callers. Run 552 hit the pinned Verus undeclared-fuel panic for `witness_blocks.cut_source`. Run 553 failed two empty-stored-payload subrange preconditions and then panicked on undeclared `model.MDec` fuel; it has no verification-results line. Run 555 supplied the empty subrange facts, removed irrelevant fuel declarations, avoided the panic and reported 1487 verified, six failed dynamic match/EOB assertions, no remaining resource failure. An unlogged run 561 was interrupted after warnings and has no count. Its log is not evidence of success.

Preparatory audit 563 failed closed because the interrupted manifest had no current final-source count. Descriptor checks passed in 564/574. Full main-path audit 576 later exposed the `chunks` iterator shadowing error; the repaired main pipeline and all negative/checkout tests pass in 583/584. Required failure diagnostics and their referenced source archives remain retained.

---

## Sol default-limit decomposition, original layouts and audited call bindings, 2026-11-18

**Fresh final-source run 529: 831 verified, 0 errors.** Core coverage is **592 / 631**, up by **29** from 563. All twelve previously blocked streams now verify at default solver limits; no stream remains resource-blocked. The three named original IDAT split/empty-segment gates are proved. Both medium Astra witness-audit findings are repaired. This remains a partial M2 checkpoint, not M2 sign-off.

| Runs | Evidence | Result |
|---|---|---|
| 521 | Unsuccessful preparatory Python edit, followed by unchanged-source Cargo cache hit | No verification-results line; not fresh proof evidence |
| 522 | First separated stored constructors/payload/fields/checksum queries | 787 verified, 6 errors; missing symbolic header and cap facts, no resource exhaustion |
| 523 | Repaired stored composition and per-frame finishing witnesses | 801 verified, 0 errors; all twelve old resource blocks resolved |
| 524 | Implementation length-extra width increased by one | 800 verified, 1 error; real mutation kill |
| 525 | Three decomposed large BASE_RAW fixed/RLE classes | 817 verified, 0 errors |
| 526–527 | Source/provenance/frozen controls and initial nine negative audit tests | Pass |
| 528 | Implementation distance-extra width increased by one | 816 verified, 1 error; real mutation kill |
| 529 | Original one-byte, irregular and empty-IDAT layout witnesses | **831 verified, 0 errors** |
| 530 | Eleven negative audit tests, including original-layout call/cut mutations | Pass |
| 531–535 | Release tests, adapter, host/thumb instantiated probes and allocator symbols | Pass; thirteen tests |
| 536–537 | Full differential seeds 34 and 35 | 10,000 generated streams and 10,000 corruptions; zero unexpected differences |
| 538–540 | Final source/provenance/cap/allocation/frozen controls, pinned i686 inventory and eleven source-bound checkout/audit tests | All pass; 539 finds no candidate in the documented cache roots |
| 541 | Final twenty retained log hashes, final-count source hashes, full source controls and runs-free evidence checkout replay | Pass |

### Exact proved surface

- `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_stored_bridge.rs` composes symbolic stored and fixed streams from separately proved concrete facts. Large constructors, payload equality, balanced checksum sums, wire fields and fixed traces no longer share one solver query with model/public-call refinement. All queries use unchanged default limits. The four RGB16/RGBA16 9x10 stored classes and BASE_RAW stored class close the twelve old blocks; raw lengths are 550, 560, 730, 740 and 1,023.
- `/home/ws/projects/verified-decoders/png/inflate_verus/tools/author_extended_classes.py` adds the BASE_RAW default-fixed, level-nine and distance-one RLE forward candidates. Their three classes cover nine more instances. Complete forward encoding means **complete candidate DEFLATE-body encoding**: comparison with `z[2:-4]` selects eligibility and an authored trace; it never supplies raw bytes, and the proof separately checks the header, Adler and finish.
- There are now **250 classes covering 533 instances**: 136 fixed-literal classes, 94 stored classes and 20 fixed-match classes. The 136 extended classes cover 238 instances. Together with 48 individual witnesses, three earlier small error instances and eight finishing instances, these give **592 / 631**. Ten policy records remain outside the numerator.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_match.rs` adds checked fixed-match expansion at any limit at least the authored raw length. `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_finish.rs` uses it for two checksum-error fdAT prefixes at R=18, a successful 17-byte frame stream at R=18, and three green 18-byte streams whose own enclosing fcTL gives R=51. The other two finishing instances are the packed five-entry-palette stream and IDAT-after-IEND's trailing-zlib error. Each case preserves original segment boundaries and calls both actual entries. These are inflate outcomes, not PNG acceptance or APNG-composition results.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_layout.rs` verifies a generic borrowed-slice layout constructor and replays both actual entries. `/home/ws/projects/verified-decoders/png/inflate_verus/tools/witness_layouts.json` binds the three named records to their original boundaries: 1,005 one-byte IDATs, 67 irregular IDATs, and five IDATs of lengths `[0,10,0,995,0]`. All three use their independently calculated R=1,023 and authored BASE_RAW. These layout witnesses close named original-boundary gates but add no inventory credit beyond the already proved stream class.

### Astra audit findings repaired

The read-only report is `/home/ws/projects/verified-decoders/png/REVIEW-ASTRA-WITNESSES.md`; its files were not edited. Astra found no wrong expected bytes, oracle leak or weakened public-entry proof, and confirmed the previous 563/631 count.

`/home/ws/projects/verified-decoders/png/inflate_verus/tools/witness_calls.py` parses a restricted straight-line Rust witness language. It skips proof blocks, resolves actual constructor bytes and ghost raw bindings, and rejects unsupported runtime statements or control flow around a harness. The audit now binds every credited caller to the exact multiset of input, limit, window/tail options, status and raw sequence. `/home/ws/projects/verified-decoders/png/inflate_verus/tools/witness_individual.json` explicitly describes all 48 individual witnesses at independently derived R; class, special, finishing, policy and original-layout callers receive the same parsed-call checks. Reviewed harness declarations/bodies have token hashes in `tools/witness_harnesses.json`; this also pins `check_public`'s implicit four-byte limit and the helpers that invoke both real entries.

`/home/ws/projects/verified-decoders/png/inflate_verus/tools/test_witness_audit.py` rejects removed calls, changed limits, duplicate option tuples, changed individual/special manifest statuses, calls hidden under control flow, a removed new finishing/layout call and changed layout cuts. Its checkout test copies required evidence and source files without a `runs/` directory, checks them, then confirms that a changed log fails its hash check. This is a fail-closed checked-descriptor gate, not a claim to parse arbitrary Rust.

All audit-required diagnostics now live under the non-ignored `/home/ws/projects/verified-decoders/png/inflate_verus/evidence/`. `evidence/manifest.json` checks log SHA-256 digests, exact count lines and the final run's source/Cargo/verify.sh hashes. Historical 507/508 failure logs, 510's count, new proof/mutation counts and final runtime logs are retained; originals remain in `runs/`. `evidence/historical-blocked.json` preserves all old block identities, diagnostics and source snapshots. `tools/witness_blocked.json` is empty because every old block was resolved, not because its evidence was discarded. No `.gitignore` edit occurred.

### Runtime, mutations and frozen boundary

Production runtime implementations did not change; only verifier-gated modules and module declarations, authoring/audit tooling, inventories, evidence and documentation changed. Full runtime evidence was nevertheless replayed in 531–537: thirteen release tests, adapter, host/thumb libraries and instantiated allocator probes, allocator-symbol checks, all 1,181 extracted streams in four layouts per seed, 10,000 generated streams and 10,000 single-bit corruptions. The differential checks **33,544 public-entry pairs / 67,088 actual calls**, with zero unexpected differences. Work remains erased and proved, not runtime-measured. No signed production allocation/call-graph audit is claimed.

New real mutation kills are 524 (`length-extra-width-plus-one`) and 528 (`distance-extra-width-plus-one`), raising the historical total from 63 to **65**. Each changes only the implementation in a new disposable copy and fails the compressed-step refinement postcondition; neither is a type-check failure or a resource failure. Existing mutation copies and `runs/sol-*.out` were left unchanged. These totals include mutation 12 as ghost representation narrowing, not an executable narrowed-counter experiment.

E1 remains APPROVED; E2 remains APPROVED WITH AMENDMENT. The earlier SegmentSlices L-SEG-FRAME discharge and the separate M3 IDAT/fdAT/iCCP reader/call-site obligations remain unchanged. No universal no-extra-effect guarantee for arbitrary T-SEG readers is claimed. No new frozen-statement discrepancy was found; ERRATA.md, THEOREMS.md and spec-frozen were not edited. Sixteen malformed/ambiguous IHDR/fcTL instances remain uncredited without an invented fallback R.

### Exact resume

1. **39 core instances remain:** 21 first-fixed-block trace gaps, `depth16/rounding-boundaries`, the dynamic first block of fdAT `apng/interlaced-subframe`, and sixteen instances without a legal independent image/frame R. There are **no current default-limit blocks**. The inventory gives every name; do not relabel unattempted records as resource failures.
2. The BASE_RAW decomposed constructors/trace/checksum/model pattern is ready for further large classes. The remaining level-six/level-nine fixed-header variants are natural next candidates; other window headers require the correct selected window, not a silent 32,768-byte substitution. The small empty-Adam7-pass case still needs Full at R=2, not Done on its eight authored raw bytes.
3. Named original IDAT split/empty gates are now closed. Both frozen declared-window records, the beyond-declared-window policy family, large remaining level/filter/flush/truncation streams, image excess-data/excess-row cases, policy excess-output/excess-row/excess-output-and-tail, the two dynamic profiles and further mutation variants remain open. Success at R or an existing synthetic exact/window+1/ring-wrap stream is not a substitute for those records.
4. Pinned i686 proof replay remains open. Run 539 finds no vstd `.vir` candidate under an i686 path in the inflate/GIF targets or `/home/ws/.cache`; no bootstrap, rebuild, copy, feature/flag/fingerprint change or target substitution occurred. Thumb probes remain runtime evidence, not an i686 proof replay.

All jobs started in this continuation have exited. **Next logged run is 542.** Runs 538/541 pass all source/provenance/allocation/cap/erased-counter controls and all twelve frozen hashes. Run 540 passes all eleven negative audit/checkout tests, and 541 rechecks the final twenty retained evidence-log hashes plus the final count's complete source/Cargo/verify.sh hashes with no `runs/` directory in its checkout replay.

Pre-edit snapshots are `backups/sol-before-decomposed-witnesses.tar.gz` (351,710 bytes), `backups/sol-before-large-fixed-decomposition.tar.gz` (346,203 bytes) and `backups/sol-before-original-layouts.tar.gz` (505,182 bytes), all below 1 MB. No target directory was cleaned, removed or repaired. No commit/push or edit outside this crate occurred. Verus 0.2026.11.02.8bf68ee, cached vstd hash 01822e92e4f44378, rustc 1.99.1, one Cargo job, two verifier threads, default solver limits and --no-cheating are unchanged. No trusted item or proof escape was added. Only tooling/evidence/documentation changed after final-source run 529.

---

## Sol fixed-match/effect bridge and extended authored classes, 2026-11-18

**Fresh final-source run 510: 767 verified, 0 errors.** The independently authored core-stream subset is now **563 of 631**, up from 343: **220 additional frozen stream instances**. The remaining 68 have explicit incomplete entries, including twelve streams blocked by five named default-limit queries. Ten policy records are additionally proved, outside the core numerator. This is a clean partial M2 checkpoint, not M2 sign-off.

| Runs | Evidence | Result |
|---|---|---|
| 502–503 | Independent fixed-match wire/extra/copy-effect bridge | Run 503: 503 verified, 0 errors |
| 504–506 | Balanced independent Adler-sum bridge development | Nat/base-case proof failures; no mutation kill |
| 507 | First 80 extended authored classes | 652 verified, 15 errors; three named default-resource failures retained |
| 508 | Extended interlace/APNG schedule and explicit scalar Adler arithmetic | 749 verified, 2 errors; two further named default-resource failures retained |
| 509 | Eligible classes and small original-segment error/policy gates | 767 verified, 0 errors |
| 510 | Final cap-clean Adler-radix representation | **767 verified, 0 errors** |
| 511–516 | Source/provenance/allocation/cap/hash controls, release tests/adapter, host/thumb probes and symbols | All pass |
| 517–518 | Full differential seeds 32 and 33 | 10,000 generated streams plus 10,000 corruptions; zero unexpected differences |
| 519 | Matching pinned i686 verifier-cache inventory | No candidate; no bootstrap or replay |
| 520 | Final source controls with precise R/blocked inventory reasons | All pass; all twelve frozen hashes unchanged |

### Exact proved surface

- `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_match.rs` adds a balanced authored fixed-code trace bridge. Each step specifies its wire position, next position, produced-prefix lengths, literal/length symbol, length extra, distance symbol, distance extra, copy length and distance. The bridge proves both canonical decodes, the extra fields and each output effect. Its independent sequence-copy predicate checks every copied byte against the evolving raw prefix, including overlap; no executable Huffman table or measuring-ring representation supplies expected bytes or effects.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_checksum.rs` proves balanced byte/prefix sums equal the independent mathematical Adler sums, using a separately proved concatenation identity. Concrete classes check balanced sums and scalar checksum arithmetic separately; they do not interpret the entire inflate/checksum recursively in one compute expression. The checksum radix uses `u16::MAX+1`, avoiding the cap lint's unrelated MAX_FRAMES numeral collision without weakening the lint.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_extended.rs` adds **128 classes covering 217 new streams**: 50 fixed-literal classes cover 59 instances, 61 stored classes cover 70, and 17 fixed-match classes cover 88. Every class has a separately verified exact literal input constructor, an independently authored raw sequence, a model/status proof at R and calls to both actual public entries. Successful new classes reach 527 raw bytes; that observation is not a general solver bound.
- Across old and new classes, **242 classes cover 512 instances**: 136 fixed-literal classes cover 318, 89 stored classes cover 106, and 17 fixed-match classes cover 88. The 48 earlier individual witnesses and three new small error instances give the total **563 / 631**.
- `/home/ws/projects/verified-decoders/png/inflate_verus/tools/author_extended_classes.py` independently replays the sample schedule through the initial images, individual filters, tRNS, BASE_PX/rawW RNG consumption and interlace-edge images. It authors APNG samples directly, before composition. Forward LZ77 candidates depend only on intended raw bytes and fixed greedy/lazy dictionary rules; complete forward-encoding equality against the input selects eligibility, never expected output bytes. The tool imports only json, random and pathlib; it imports/executes neither the frozen encoder nor a decoder, oracle or executable decode table. The 128-byte batch restriction is gone.
- `/home/ws/projects/verified-decoders/png/inflate_verus/tools/audit_witnesses.py` checks every new constructor's exact frozen bytes, both authored raw bindings, every trace transcription, every class member and its independently calculated R. fdAT membership uses the stream's own enclosing fcTL dimensions and original sequence-number-stripped payloads, not canvas dimensions. The new successful classes cover **33 fdAT instances**; two separately proved empty fdAT instances bring the newly covered fdAT total to 35. No malformed/ambiguous shape receives an invented R.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_special.rs` proves both frozen empty-fdAT streams at R=5 with empty output/TruncatedZlib, and core `zlib/trailing-in-later-idat` at R=4 with raw `[0,65,0,66]`/TrailingZlibData using its original two IDAT segments. It additionally closes both policy `window/distance-before-output-stays-an-error` records at R=4 and R+1=5 under all four window/tail combinations, retaining empty output/DistanceTooFar. The eight previous tail-policy records remain proved, for ten policy records in total; these are inflate outcomes, not integrated PNG policy theorems.

### Named default-limit blocks and other failures

The following queries actually exhausted the unchanged default solver limit. They were removed from active witness claims and were not retried with increased limits. Diagnostics and exact failing source snapshots remain in the crate; `/home/ws/projects/verified-decoders/png/inflate_verus/tools/witness_blocked.json` maps each failure to every affected frozen instance.

| Named query | Frozen streams | Raw bytes | Diagnostic run |
|---|---|---:|---:|
| extended_ct2_d16_il0_9x10 | ok/ct2-d16-il0-9x10 | 550 | 507 |
| extended_ct2_d16_il1_9x10 | ok/ct2-d16-il1-9x10 | 560 | 508 |
| extended_ct6_d16_il0_9x10 | ok/ct6-d16-il0-9x10 | 730 | 507 |
| extended_ct6_d16_il1_9x10 | ok/ct6-d16-il1-9x10 | 740 | 507 |
| base_stored | level0 default/fixed/huffman/rle/filtered, plus level1/6/9 huffman | 1,023 | 508 |

Run 502 rejected a boolean-containing trigger; selecting the trace-index term fixed it. Run 504 rejected a nat/int mismatch. Runs 505–506 needed explicit empty/singleton Adler-sum base facts. Run 507's twelve non-resource Adler assertions needed explicit scalar sums/modular arithmetic, not larger solver limits. Run 508 left only its two named resource failures. An unlogged source-control trial after 509 detected the checksum-radix/cap-lint collision; 510 and source-control runs 511/520 validate the representation-only fix. No failing/speculative proof remains active.

There are **no new mutation kills**; the historical total remains 63. Historical run 453 remains a type-check failure, not a kill. Existing mutation cases, selectors and `runs/sol-*.out` evidence were not changed. These totals include mutation 12 as ghost representation narrowing, not an executable narrowed-counter experiment.

### Runtime and frozen boundary

Only verifier-gated modules, cfg(verus_only) module declarations, authoring/audit tooling, inventories and documentation changed. Production implementations are unchanged. Full runtime evidence was nevertheless replayed: source/allocation/cap/erased-counter/provenance controls and twelve frozen hashes (511/520), thirteen release tests (512), adapter (513), host/thumb libraries and instantiated probes (514–515), allocator-symbol checks (516), all 1,181 extracted vector streams in four layouts per seed plus 10,000 generated streams and 10,000 single-bit corruptions (517–518). The differential checks **33,544 public-entry pairs / 67,088 actual calls**, with zero unexpected differences. Work observations remain erased and proved, not runtime measured. No signed production allocation/call-graph audit is claimed.

E1 remains APPROVED; E2 remains APPROVED WITH AMENDMENT. The earlier SegmentSlices L-SEG-FRAME discharge and separate per-reader/call-site M3 obligations are unchanged. No universal no-extra-effect guarantee for arbitrary T-SEG readers is claimed. No new frozen-statement discrepancy was found; ERRATA.md, THEOREMS.md and spec-frozen were not edited. Sixteen remaining extracted instances lack a legal independent image/frame R because of malformed or ambiguous IHDR/fcTL; they remain uncredited rather than silently receiving a fallback limit.

### Exact resume

1. Continue `/home/ws/projects/verified-decoders/png/inflate_verus/tools/witness_inventory.json`: **68 remain** — twelve default-limit-blocked streams, 32 first-fixed-block trace gaps, seven further fdAT gaps, one dynamic profile (`depth16/rounding-boundaries`), and sixteen malformed/ambiguous image/frame-R cases. Preserve the five named resource failures and their diagnostics; do not raise limits or describe unattempted streams as resource failures.
2. The reusable fixed-match/effect and balanced Adler bridges are ready. Extend the independent forward encoder/profile intent, not decode-oracle output. For the remaining fdAT records, distinguish checksum-error prefixes, a successful zlib stream with short image data, the interlaced subframe and streams whose enclosing frame R differs from the existing green-frame class. A byte-identical Z does not justify reusing a proof at a different R. Keep original parts and per-stream fcTL provenance.
3. Named gates still open: core idat/split-1-byte, idat/split-irregular and idat/zero-length-idats; both frozen declared-window records; large zlib/flush/truncation streams; policy excess-output/excess-row/excess-output-and-tail; policy window/distance-beyond-declared-window; and further required implementation mutations. Core trailing-in-later-IDAT and the distance-before-output window-policy family are now closed, not substitutes for those open gates. Existing synthetic exact/window+1, ring-wrap, D5 and L-LIMIT-MONO proofs remain checked.
4. i686 replay remains open. Run 519 finds no pinned matching i686 vstd .vir in inflate/GIF targets or /home/ws/.cache. No bootstrap, version/feature/flag/fingerprint change or target substitution occurred. Thumb probes are runtime evidence, not an i686 proof replay.

All jobs started in this continuation have exited. **Next logged run is 521.** The pre-edit snapshot is `/home/ws/projects/verified-decoders/png/inflate_verus/backups/sol-before-match-bridge.tar.gz` (211,523 bytes). Exact failing queries/manifests are in `backups/sol-run507-extended-queries.tar.gz` (102,465 bytes) and `backups/sol-run508-extended-queries.tar.gz` (127,306 bytes); each archive is below 1 MB. No target directory was cleaned, removed or repaired; no commit/push or edit outside the crate occurred. Verus 0.2026.11.02.8bf68ee, cached vstd hash 01822e92e4f44378, rustc 1.99.1, one Cargo job, two verifier threads, default solver limits and --no-cheating are unchanged. No trusted item or proof escape was added. Only tooling/inventory/documentation changed after fresh final-source run 510.

---

## Sol authored repeats and systematic stream classes, 2026-11-18

**Fresh final-source run 492: 496 verified, 0 errors.** The independently authored core-stream subset is now **343 of 631**, up from 43: 300 additional frozen stream instances. All 631 instances have an explicit coverage/incomplete entry. Four tail-policy families additionally cover eight policy records, outside the core numerator. This is a clean partial M2 checkpoint, not M2 sign-off.

| Runs | Evidence | Result |
|---|---|---|
| 477–481 | Independent partial repeat-16/17/18 trace bridge, CL-field bridge and four frozen repeat/admission errors | Run 481: 371 verified, 0 errors |
| 482–484 | Balanced fixed-literal trace bridge and first four byte-identical classes | Run 484: 378 verified, 0 errors |
| 485–486 | Systematic encoder-intent literal classes and an authored all-three-repeat-kind regression | Run 486: 461 verified, 0 errors |
| 487–489 | Generic D5 stored-payload class bridge, 28 stored classes, and original-segment tail-policy witnesses | Run 489: 495 verified, 0 errors |
| 490–492 | Frozen single-literal-code-len1 gate, scalar admission/window facts and formatting | Run 492: 496 verified, 0 errors |
| 493–498 | Final source/provenance/allocation/cap/erased-counter/hash controls, release tests/adapter and host/thumb probes/symbols | All pass |
| 499–500 | Full differential seeds 30 and 31 | 10,000 generated streams plus 10,000 corruptions; zero unexpected differences |
| 501 | Matching pinned i686 verifier-cache inventory | No candidate; no replay or bootstrap |

### Exact proved surface

- `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_dynamic.rs` now checks authored repeat symbols, consumed bit intervals, extra-field values and independently specified expanded length intervals. `expand_repeats` composes a partial trace with a subsequent trace, `finish_dynamic`, or an independently authored rejection. It supports literals plus 16/17/18, including the previous-value condition, widths, counts and remaining-entry bound. The expected length sequence is supplied independently; no decoder constructs it. `authored_three_repeat_kinds` checks literal 3, repeat-16 of three 3s, repeat-17 of three zeros and repeat-18 of eleven zeros against explicit wire words. This last item is a partial synthetic expansion regression, not a frozen stream or a complete public-call stream witness. `cl_fields` separates all nineteen authored CL fields from final admission, including rejected CL profiles.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_repeats.rs` closes frozen repeat-with-no-previous, repeat-overflows-count, code-length-code-oversubscribed and code-length-code-incomplete. The overflow trace authors two successful 138-zero emissions and rejection of the third, at 288 available slots. The frozen generator's nearby comment says 257+2; its actual literal profile has 286 entries, and the proof uses 286+2. Expected errors and empty output prefixes are authored, not copied from an oracle.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_single_literal.rs` closes frozen single-literal-code-len1: an accepted sole one-bit EOB, an empty non-final dynamic block, then an authored stored `[0,65,0,66]`, R=4 and Done. All five D4 gates and the earlier dynamic/dead-prefix witnesses remain checked.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_literal_class.rs` proves generic balanced fixed-literal word/decode/expansion bridges and a separate stored-payload bridge using the existing independent D5 summary. `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_classes.rs` instantiates 114 classes: 86 fixed-literal classes cover 259 frozen instances, and 28 stored classes cover 36 more. Every class proves its exact independently authored raw bytes and Done at image R, then invokes the common harness that calls both actual public entries.
- `/home/ws/projects/verified-decoders/png/inflate_verus/tools/author_literal_classes.py` reconstructs samples, packing, forward filtering and Adam7 traversal from the frozen encoder's stated intent and initial RNG schedule, not from decoding compressed bytes. The four small seed profiles use explicit sample/raw literals. The RGB TPX literals were recovered by a read-only replay of the frozen encoder prefix, with no decoder or frozen-file write. The systematic schedule uses only Python random, json and pathlib; it never imports or executes a reference inflate or an executable Huffman table. Forward fixed-literal/stored encoding is only an eligibility comparison against the transcribed input, never a source of expected raw bytes. This batch limits independently reconstructed rasters to 128 raw bytes and only these two forward encodings.
- `/home/ws/projects/verified-decoders/png/inflate_verus/tools/witness_classes.json` lists every class member by frozen record name and extracted stream index. `/home/ws/projects/verified-decoders/png/inflate_verus/tools/audit_witnesses.py` independently checks each membership's exact input bytes and legal IHDR/Adam7 image R against the verified representative's raw length, rejects duplicate coverage, checks every literal transcription, and restricts authoring imports/expected-field reads/eval/exec. Byte-identical Z with equal independently computed R reuses one model/public-entry proof; it is not one separately transcribed proof per record. Invalid IHDR shapes and fdAT streams receive no class credit. `/home/ws/projects/verified-decoders/png/inflate_verus/tools/witness_inventory.json` inventories all 631 instances, including the 288 not covered. Its incomplete categories inspect only the first block type/presence, not a decode oracle.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_policy.rs` calls both actual entries with configurable window/tail choices and the supplied SegmentSlices parts. `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_policy_cases.rs` proves trailing-byte, second-stream, trailing-in-later-idat and bad-adler-with-trailing-byte-stays-an-error for both frozen Strict/RB records in each family. The proof uses the original IDAT segment arrays, including the split before the later trailing byte. All four window/tail combinations are checked at R=4 and R+1=5: IgnoreTail succeeds only at exactly full output with a matching Adler; bad Adler always fails. The eight policy records have separate segment-provenance checks and are not counted among 343 core instances. These are inflate outcomes, not integrated PNG policy theorems.

### Failures, limits and runtime boundary

- Runs 477 and 482 failed before verification (nat/int mismatch and a ghost sequence-expression placement). Run 480 hit the known Verus compute/free-parameter panic when interpreting quantified CL admission; scalar Kraft facts plus `cl_fields` avoid it. Runs 478–479 needed an explicit MDec bridge, run 483 needed explicit decode triggers, runs 487–488 needed explicit stored header/window facts, and run 490 needed scalar zero-distance admission and an earlier window fact. These are proof-development failures, not mutation kills. No failed or speculative proof remains active.
- No stream attempted in this round remains blocked by a solver resource limit. The 128-byte authoring batch boundary is not a proved solver bound, and unattempted larger streams must not be described as verifier failures. The remaining 288 instances are inventoried as 161 first-fixed-block, 83 first-stored-block, 43 non-IDAT and one first-dynamic-block cases. Their authored traces/profiles, not runtime decoding, remain missing.
- Existing production implementations are byte-identical to the initial snapshot. Only verifier-gated modules, their specifications/proofs, authoring/audit tooling and documentation changed; `src/lib.rs` adds only cfg(verus_only) module declarations. Complete runtime evidence was nevertheless replayed: source/allocation/cap/erased-counter/provenance controls and all twelve frozen hashes (493), thirteen release tests (494), adapter (495), host/thumb libraries and instantiated probes (496–497), allocator symbols (498), all 1,181 vector streams in four layouts per seed plus 10,000 generated streams and 10,000 single-bit corruptions across seeds 30/31 (499–500). The differential makes 33,544 public-entry pairs / 67,088 actual calls, with zero unexpected differences. Its work observations remain erased and proved, not runtime-measured. No signed production allocation/call-graph audit is claimed.
- There are **no new mutation kills** in this continuation; the historical total remains 63. Run 453 remains a type-check failure, not a kill. Existing mutation selectors and evidence were not changed or weakened. These totals include mutation 12 as ghost representation narrowing, not an executable narrowed-counter experiment.

### Exact resume

1. Continue the 288 missing core instances from `/home/ws/projects/verified-decoders/png/inflate_verus/tools/witness_inventory.json`. Extend independent encoder intent beyond the initial valid-image schedule and add an authored fixed-match/effect trace bridge; the repeated APNG frame streams are useful shared classes. Extend stored classes past this batch only with default limits; a balanced independent Adler/checksum bridge and separately proved large literal constructors can avoid monolithic interpretation. If an individual query actually exhausts its default limit, retain its named diagnostics, mark that instance blocked, and move on. Do not infer a resource failure from this round's 128-byte eligibility cap.
2. Preserve per-stream provenance and independent R when adding fdAT class membership. Current successful class membership deliberately credits IDAT only. The generic segmented harness does not by itself credit the still-open core idat/split-1-byte, idat/split-irregular and idat/zero-length-idats records, whose large shared compressed stream still needs an authored model trace.
3. The named frozen declared-window records and the large zlib/flush/truncation streams remain open. Policy excess-output/excess-row/excess-output-and-tail and both window-policy families remain open. This round's eight policy witnesses do not replace those gates. The synthetic exact/window+1, wrapped ring, D5 and separate L-LIMIT-MONO proofs from the preceding handoff remain proved. Any further required implementation mutation variants remain work after the concrete gates.
4. Matching pinned i686 verification is still blocked. Run 501 again finds no i686 vstd .vir candidate in inflate/GIF targets or /home/ws/.cache. No cache bootstrap, version/feature/flag/fingerprint change or target substitution occurred. Thumb runtime probes are not an i686 verifier replay.
5. E1 remains APPROVED and E2 remains APPROVED WITH AMENDMENT. The preceding handoff's complete SegmentSlices L-SEG-FRAME discharge is unchanged. No universal no-extra-effect guarantee for arbitrary T-SEG readers is claimed; IDAT, fdAT and iCCP each still need the M3 reader and call-site discharge. No new frozen-statement discrepancy was introduced or resolved here; ERRATA.md, THEOREMS.md and spec-frozen were not edited.

All jobs started by this continuation have exited. **Next logged run is 502.** Only handoff documentation changed after the final-source verifier run and runtime evidence. The pre-edit snapshot is `/home/ws/projects/verified-decoders/png/inflate_verus/backups/sol-before-repeat-bridge.tar` (993,280 bytes, below 1 MB). No target directory was cleaned, removed or repaired; no commit/push or edit outside this crate occurred. Verus 0.2026.11.02.8bf68ee, vstd 0.0.0-2026-11-01-0251 / cached hash 01822e92e4f44378, rustc 1.99.1, one Cargo job, two verifier threads, default solver limits and --no-cheating are unchanged. No trusted item or proof escape was added.

---

## Sol dynamic/D4 witnesses, limit monotonicity and remaining mutations, 2026-11-18

**Fresh final-source run 476: 362 verified, 0 errors.** The selected successful dynamic, five D4, large fixed-overlap, exact/window+1 and measuring-ring-wrap gates are proved. L-LIMIT-MONO is now a separate independent-model theorem. The frozen subset is 43 of 631 core streams, so 588 remain. Nineteen additional verifier-killed mutations bring the historical total to 63; run 453 is a type-check failure and is not a kill. E2 is now APPROVED WITH AMENDMENT by the council, and the first L-SEG-FRAME discharge is recorded below. This is not M2 sign-off. These totals include mutation 12 as ghost representation narrowing, not an executable narrowed-counter experiment.

| Runs | Evidence | Result |
|---|---|---|
| 411–430 | Balanced literal-length trace bridge, sparse canonical profiles and three successful frozen dynamic blocks | Run 430: 317 verified, 0 errors |
| 431–432 | All five frozen D4 cases, including aligned live-prefix EOF | Run 432: 324 verified, 0 errors |
| 433–435 | Concrete non-cursor frame postcondition and large fixed-overlap record | Run 435: 329 verified, 0 errors |
| 436–437 | Synthetic exact declared window, window+1 and measuring ring wrap | Run 437: 336 verified, 0 errors |
| 438–452 | Fifteen implementation-side verifier mutation kills | 335 verified/1 error, except run 451: 331 verified/5 errors (ghost representation narrowing, not an executable counter) |
| 453 | Ill-typed exploratory construction/read reclassification | No verifier count; not a kill |
| 454–455 | Separate model L-LIMIT-MONO and six systematic dynamic companion witnesses | Run 455: 358 verified, 0 errors |
| 456–459 | Read/decode construction-budget contamination and dynamic EOB/admission ordering | Each: 357 verified, 1 error |
| 460–465 | Two more frozen gates, isolated large FCHECK literal constructor, formatting and default-limit proof fixes | Run 465: 362 verified, 0 errors |
| 466–471 | Final source/allocation/cap/erased-counter/provenance/hash controls, release tests/adapter, host/thumb probes and symbols | All gates pass |
| 472–473 | Final-source full differential seeds 28/29 | 33,544 public-entry pairs; zero unexpected differences |
| 474 | Matching pinned i686 verifier-cache inventory | No candidate; no bootstrap or replay |
| 475 | Cargo-cached verifier invocation after an mtime-only touch | No count; not fresh proof evidence |
| 476 | Fresh final-source replay after an erased limit-bound projection | 362 verified, 0 errors |

### Exact proved surface

- `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_dynamic.rs` proves a balanced conjunction of authored canonical wire words, their MDec results, literal code-length expansion and the nineteen CL fields against the existing independent MDynamic model. Its sparse profiles describe EOB alone, the four tiny-image symbols and the eight-symbol match code arithmetically; they do not import executable tables. Balanced checking avoids the interpreter's default recursion depth without raising it. `by (compute)` over quantified admissibility produced an internal free-parameter panic in runs 423–425; computing only scalar Kraft/count/word facts and proving admission separately resolved it. Scoped scalar/recursive hides resolved the later rejection-profile resource pressure without changing limits.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_dynamic_cases.rs` adds dynamic-literals-two-dist-codes, dynamic-single-dist-code-len1 and dynamic-no-dist-codes, each with authored raw `[0,65,0,66]`, R=4 and Done. `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_d4.rs` adds all five frozen short-input D4 records. The dead cases return InvalidSymbol at the empty distance code or first wrong one-bit prefix. The live cases return TruncatedZlib at actual EOF after the authored three/six fixed literals 144 that align the dynamic continuation; R=17 and the corresponding output prefixes are explicit. The same bridge covers the companion unassigned-code-in-incomplete-literal-code and length-with-no-distance-codes records.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_invalid_dynamic.rs` adds no-end-of-block-code, incomplete-literal-code, incomplete-distance-code and oversubscribed-literal-code from independently specified length profiles and first errors. `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_cases.rs` additionally proves empty-stored-then-stored and zlib/fcheck. FCHECK uses a general early-rejection lemma and a separately verified 1,005-byte literal constructor: only the constructor's first two bytes enter the caller contract, because the error precedes every block read. Keeping that large literal in the public-call query exceeded the default resource limit in runs 461/463; isolation fixed it in 465.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_overlap.rs` proves fixed-overlapping-matches at R=301: literals 0/A, then distance-one copies of 258, 11 and 30 bytes, then EOB. The authored result is 0 followed by 300 A bytes. Generic sequence-copy and closed Adler-sum lemmas avoid expanding the output recursively in the verifier; the two length-extra fields are explicitly pinned to 0 and 3.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_window.rs` and `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_window_cases.rs` add three synthetic, not frozen, streams. Stored prefixes of 256 and 257 zeros precede a length-three match at the same distance under CINFO=0. Distance 256 succeeds under Enforce and Ignore; distance 257 gives DistanceTooFar with 257 output bytes under Enforce and Done with 260 zeros under Ignore. A stored prefix of 32,768 zeros followed by a length-three match at distance 32,768 produces 32,771 zeros, wraps the actual measuring ring and proves its complete last-window representation. Both public entries are called under both window settings with Reject tail; these are not the named RB policy witnesses.
- `/home/ws/projects/verified-decoders/png/inflate_verus/tools/audit_witnesses.py` checks all 43 frozen IDAT transcriptions, including the isolated FCHECK constructor, and checks the authoring scripts' import/expected-field restrictions. The scripts transcribe input bytes only and author profiles, positions, effects and expected bytes/statuses without a decoder oracle or executable table. The audit is a provenance check, not a proof that all 631 extracted streams have witnesses. Seventeen frozen records and three synthetic streams were added in this continuation; the existing synthetic D5 witnesses remain separate.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/limit_mono.rs:234` proves `lemma_limit_mono` solely over the independent model: if Inflate at large is Done and its output length is at most small at most large, Inflate at small is Done with the identical output and r2. Copy/symbol/compressed/stored/block shape and cross-limit lemmas compose through checksum/tail handling. Both implemented window and tail choices are covered. No executable buffer, table, reader, oracle or public-entry assumption enters this theorem. The future ICC caller still has to instantiate it; no integrated ICC implementation is claimed.

### First L-SEG-FRAME discharge: SegmentSlices under approved E2

This discharges E2's per-reader obligation for the current concrete representation, not for arbitrary T-SEG implementations. Return values are outside the state-effect restriction. A representation or call-graph change reopens the inspection obligation.

1. **(i), inspected structure.** The type is `/home/ws/projects/verified-decoders/png/inflate_verus/src/segments.rs:96–104`. `parts` is a shared slice of shared byte slices; the reachable runtime types are shared references, slices, u8, usize, u64 and bool. There is no mutable reference, raw pointer, interior-mutability type, I/O capability or wrapper carrying one. `seg_work` and `read_work` are erased Ghost<nat> observations, not runtime capabilities. This is a type-definition inspection, not a quantified Verus heap theorem.
2. **(ii), checked safe implementation and inspected call closure.** The concrete read is `/home/ws/projects/verified-decoders/png/inflate_verus/src/segments.rs:173–208`; its only nontrivial helper is the erased `lemma_parts_byte` at :23, which recurses through the mathematical shared-slice concatenation. Runtime operations are safe field updates, slice length/index operations and return. `/home/ws/projects/verified-decoders/png/inflate_verus/src/lib.rs:6` forbids unsafe code. The concrete body and its proof helpers verify under the unchanged `--no-cheating` invocation; source inspection finds no external_body, assume_specification, static mutable state or I/O in this call closure. Existing Verus/vstd/Rust built-ins remain trusted.
3. **(iii), complete frame and enumerated cursor bookkeeping.** `/home/ws/projects/verified-decoders/png/inflate_verus/src/segments.rs:155` returns all runtime non-cursor state, the complete `parts` borrow. Runtime cursor fields are `index`, `offset`, `position` and `end_reported`. The two erased cursor/work bookkeeping fields are `seg_work` (segment/end advances) and `read_work` (byte-return/repeated-EOF iterations). Neither stores an external referent or creates an observable runtime effect. The frame omits no other field of the current concrete representation.
4. **(iv), checked value preservation plus inspected aliasing argument.** `/home/ws/projects/verified-decoders/png/inflate_verus/src/segments.rs:108–110` defines `preserves_noncursor` as exact equality of `parts`. The concrete read postcondition at :174 proves that predicate on every reachable return, Some and None, in addition to the inherited source-frame/concat/cursor laws. Since the type has no other runtime non-cursor field, Verus checks unchanged runtime non-cursor state. The conclusion that no other externally observable effect occurs also uses the inspected absence of another mutable capability or global/I/O operation and Rust's shared-reference/aliasing rules. Rust's aliasing rules are trusted; Verus does not prove that language-level inference or a universal effect theorem.
5. **Actual entries and instantiations.** `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:1335` is actual `inflate_zlib_into`, with the declared source-frame postcondition at :1338; actual `inflate_zlib_measure` is :1362, with that postcondition at :1366. `/home/ws/projects/verified-decoders/png/inflate_verus/src/witnesses.rs:227–246` constructs concrete SegmentSlices values and calls those actual entries at :234 and :246. All new frozen witnesses use this harness, and the synthetic window harness also instantiates both entries with SegmentSlices. Combined with the verified output/suffix contracts, the checked frame and the inspection/aliasing argument give only output/ring bytes and source cursor/bookkeeping effects for these instantiations. There is no integrated PNG IDAT, fdAT or iCCP reader/call-site discharge here; each remains an M3 obligation, separately.

E2 in `/home/ws/projects/verified-decoders/png/ERRATA.md` is APPROVED WITH AMENDMENT, not PROPOSED. Its approval resolves the original universal-effect statement mismatch, not the remaining M2 gates. The generic marker reproducer from run 410 remains consistent with the amended contract. No original universal no-extra-effect guarantee is claimed for an arbitrary T-SEG reader. ERRATA.md, THEOREMS.md and spec-frozen were not edited in this continuation.

### Mutation evidence and exact resume

- New real verifier kills: 438 bits partial-EOF invented buffer; 439 reader EOF byte advance; 440 rejection of an accepted single literal code; 441 admission of a one-symbol two-bit code; 442 empty distance rejection; 443 floored live count; 444 reversed live shift; 445 wrong reverse depth; 446 missing depth zero; 447 inclusive dead lower boundary; 448 exclusive dead upper boundary; 449 omitted length extras; 450 Adler reduction one byte late; 451 ghost representation narrowing of the implementation bit-position observation to u32 (not an executable counter); 452 byte work charged as segment work; 456 reader work included in construction budget; 457 Huffman decode work included in construction budget; 458 EOB check after literal admission; 459 distance admission before literal admission.
- Each mutation used `/home/ws/projects/verified-decoders/png/inflate_verus/tools/proof_mutations.py`, a disposable source copy, every then-current theorem and the same bounded serial verify.sh/default flags. Run 451 has five verifier errors; all other new kills have one. Run 453 failed Rust type checking before any verifier count and is not a kill; its directory/log is retained. An unrun speculative reader-ghost reclassification selector was removed rather than counted. The two actual construction-budget variants are runs 456/457. The ordering kills use proved boundary assertions at the actual build calls; they test frozen operational ordering even where the error label and cursor coincide, not an independent runtime error oracle. The u32 case mutates an implementation-side ghost position observation, not the independent model or an added runtime u32 counter.
- Continue the concrete gates for the other 588 core streams, especially single-literal-code-len1, the frozen declared-window records, CL admission/repeat-error records, segment-boundary records, remaining zlib records and the named RB/tail policy records. The literal-length bridge covers only literal length emissions; repeat-16/17/18 proofs need analogous authored repeat traces. The synthetic exact/window+1 and ring-wrap streams do not close the unproved frozen window/policy records. No complete witness or exhaustive mutation gate is claimed.
- L-LIMIT-MONO is proved, not an open handoff item. The matching pinned i686 verifier replay remains blocked on a real matching cache: run 474 again finds no i686 .vir candidate in inflate/GIF targets or /home/ws/.cache. No cache bootstrap, version/feature/flag/fingerprint change, target substitution or i686 verifier replay occurred.
- Production runtime bodies are unchanged from runs 400–407. Only verifier-only modules, specifications, erased assertions and tooling were added. Complete fresh runtime evidence nevertheless passes in runs 466–473: all source/allocation/cap/erased-counter controls, all 43 transcriptions and twelve frozen hashes (466); thirteen release tests (467); release adapter (468); host/thumbv7m instantiated libraries and probes (469–470); allocator symbols (471); all 1,181 vector streams in four layouts per seed, plus 10,000 generated streams and 10,000 single-bit corruptions across seeds 28/29 (472–473). The differential makes 33,544 public-entry pairs or 67,088 actual public calls, zero unexpected differences. Work remains erased and proved, not measured by the adapter. The only source change afterwards is the erased `small <= large` projection in the model theorem, freshly checked in 476. Run 475 was a Cargo cache hit despite an mtime-only touch and has no count; neither 475 nor the ill-typed run 453 is fresh verification evidence.
- All jobs have exited. Next logged run is 477. Only this handoff documentation changed after final-source run 476. No failing or speculative proof remains in the active source tree.
- Durable snapshots are `/home/ws/projects/verified-decoders/png/inflate_verus/backups/sol-before-dynamic-witnesses.tar` (378,880 bytes) and `/home/ws/projects/verified-decoders/png/inflate_verus/backups/sol-before-remaining-mutations.tar` (573,440 bytes), both below 1 MB. No target directory was deleted, cleaned or repaired. No commit/push or edit outside this crate occurred. Accepted Verus 0.2026.11.02.8bf68ee, vstd 0.0.0-2026-11-01-0251, rustc 1.99.1 (58cf8f9c3), one Cargo job, two verifier threads, default solver limits and --no-cheating are unchanged. No new trusted item or proof escape was added.

---

## Astra accounting/frame repairs and fixed-block witnesses, 2026-11-18

**Fresh final-source run 408: 300 verified, 0 errors.** Astra finding 2 is repaired: the first EOF iteration belongs only to `seg_steps`, and `Bits::byte` mirrors the actual reader core delta instead of adding one unconditionally. Astra finding 1 is resolved for `SegmentSlices` by an explicit source-frame postcondition at both actual public entries, backed by the production reader's shared-input-only representation and Rust's borrow rules. The generic effect clause is still a frozen-statement concern: E2 is PROPOSED in `/home/ws/projects/verified-decoders/png/ERRATA.md`, not approved or applied. This is a clean partial item-4 handoff, not M2 sign-off. The full witness/mutation gates, L-LIMIT-MONO and the matching i686 verifier replay remain open.

| Runs | Evidence | Result |
|---|---|---|
| 345–349 | Exact first/repeated-EOF partition, reader delta mirror and zero/one-empty-segment witnesses | Run 349: 274 verified, 0 errors |
| 350–351 | Typed stable source frame through reader, bit reader and actual public entries | Run 351: 274 verified, 0 errors |
| 352–361 | Independent canonical-word witness bridge, arithmetic fixed-code profiles and fixed literals | Run 361: 282 verified, 0 errors; resolved interpreter-depth/resource issues without increasing limits |
| 362–363 | Thirteen more frozen witnesses and synthetic D5 at three limits | Run 363: 300 verified, 0 errors |
| 365–389 | Twenty-five reader/bit/Huffman/dynamic/dispatch/distance/output mutations | Every run: 299 verified, 1 error |
| 390–391 | Runtime API erasure of the associated frame type and exact work-boundary pins | Thirteen release tests pass; run 391: 300 verified, 0 errors |
| 392–398 | Seven omitted work-charge mutations | Every run: 299 verified, 1 error |
| 399–405 | Final formatting, source/cap/counter/provenance/hash controls, release tests/adapter and host/thumb probes | All gates pass |
| 406–407 | Final-source full differential seeds 26/27 | 33,544 public-entry pairs; zero unexpected differences |
| 408 | Fresh final-source verifier replay | 300 verified, 0 errors |
| 409–410 | Matching pinned i686 cache inventory and generic marker-effect reproducer | No matching cache; marker still changes 7 to 99 with Done and unchanged output |

### Exact proved surface

- `/home/ws/projects/verified-decoders/png/inflate_verus/src/segments.rs:67` read now exports a core delta in 0..1, exactly one for a returned byte or a repeated EOF and zero for a first EOF. The source retains the same advance-rank/segment-delta contract and `seg_count()+1` bound. At :174 the first EOF increments only `seg_work`; the `else` branch charges a repeated EOF to `read_work`. The loop explicitly retains the initial wf/end-reported state. At :201/:215 executable verifier witnesses pin zero segments to first `(core,seg)=(0,1)` and second `(1,1)`, and one empty segment to first `(0,2)` and second `(1,2)`. Each concrete iteration now contributes once to one counter.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/bits.rs:122` byte permits a zero-or-one core delta, exports an exact equality to its reader observation and retains exactly one on success. At :148–150 it snapshots `src.core_steps()`, calls the actual read and adds only that delta to Work. This removes the mirrored EOF overlap without altering runtime input/output behaviour or weakening any manifest work bound. Explicit cache projections after loading a byte stabilise the proof under the changed contract.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/segments.rs:48` source_frame is implementation-selected and preserved by read. SegmentSlices' frame (:150) is the complete shared input borrow; its remaining runtime fields are cursor state, and its work fields are erased. `/home/ws/projects/verified-decoders/png/inflate_verus/src/bits.rs:42` origin_source, the prophetic frame tuple and complete carry that frame through the source borrow. Actual public into/measure at `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:1332/:1359` explicitly ensure source-frame preservation and document the complete effect restriction when S=SegmentSlices. The associated type, origin field and initialiser are cfg(verus_only), so ordinary custom-reader implementations do not acquire a runtime associated-type requirement.
- Generic source-frame preservation does not quantify over an implementation's omitted mutable referents or other external effects. No new trusted heap/effect specification was introduced. `/home/ws/projects/verified-decoders/png/ERRATA.md:49` contains E2, with its index row and PROPOSED status; E1 and every frozen text remain unchanged. Run 410 recompiles the unchanged original `/home/ws/projects/verified-decoders/png/inflate_verus/review-astra/caller_effect.rs` against the final release library and reproduces its marker write. E2 proposes preserving the full restriction for SegmentSlices and separately framed production PNG readers while making generic reader effects explicit. Rejection leaves the generic clause unproved and blocks M2.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/witnesses.rs:18` follow_word bridges an authored mathematical wire word to the original existential/prefix-match MDec using the existing prefix-free theorem. Arithmetic fixed-profile count/next-code lemmas and fixed_decode (:100)/fixed_dist_decode (:131) avoid importing executable tables or an oracle. The original independent model is unchanged. Direct by-compute expansion of the fixed profile exceeded the interpreter's default recursion depth (run 354); scoped model unfolding and separate symbol-step assertions resolved the later resource pressure without changing defaults.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/witnesses.rs:221` check_at makes both real public calls at the authored image R; check_public remains the R=4 wrapper. Fixed literals (:377) proves raw `[0,65,0,66]`. `/home/ws/projects/verified-decoders/png/inflate_verus/src/witness_cases.rs` adds fixed-lit-286, distance-before-output, distance-beyond-output, fixed-dist-30, fixed-dist-31, both reserved-distance-then-EOF cases, distance-equals-output (R=5, raw `[0,7,0,7,0]`, distance 2/length 3 so the copy overlaps), match-overruns-image-is-excess (R=4, Full with `[0,7,0,7]`), cut-mid-stored, BTYPE 3, HLIT 287 and HDIST 31. These fourteen additions bring the frozen subset to 26 of 631 extracted core streams, not complete gate coverage.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/witnesses.rs:427` adds a synthetic D5 companion with two present stored bytes `[0,65]` and a missing third byte. Both public entries return TruncatedZlib with that prefix at L=4 and L=2, and Full with `[0]` at L=1. This is not a frozen record and is not included in the 26-record provenance count; the frozen cut-mid-stored case cuts inside the stored header and does not itself exercise a partial payload.
- `/home/ws/projects/verified-decoders/png/inflate_verus/tools/author_more_witnesses.py` transcribes only named frozen input bytes and emits the thirteen new case functions from explicitly authored symbol traces/statuses/raw prefixes; it never calls an inflate oracle. `/home/ws/projects/verified-decoders/png/inflate_verus/tools/audit_witnesses.py` checks both witness files and all 26 IDAT transcriptions. The checker recognises spinoff attributes and scoped hides; its first attempted check after expansion counted only 16 because the old regex skipped those forms, and the final controlled check passes.
- Exact local work pins at actual loop/fill boundaries cover CL zero fill, dynamic expansion symbols, Huffman decode, compressed symbols and block dispatch. `/home/ws/projects/verified-decoders/png/inflate_verus/tools/proof_mutations.py` now supports per-file selectors, preflights them before creating a disposable copy and retains every diagnostic. Thirty-two new verifier kills plus the twelve historical kills give 44 recorded implementation-only kills; no runtime failure or selector failure is counted as a kill.

### Exact resume and remaining boundaries

1. Continue item 4 with successful dynamic-block witnesses, the five frozen D4 public-call witnesses, the large fixed-overlap record, declared-window edge cases and ring wrap through the measuring entry. The new distance-equals-output case proves a small overlapping copy, not a wrapped ring. Dynamic HLIT/HDIST rejection witnesses do not cover successful dynamic table construction or symbol decoding. The other 605 core streams and required RB/policy witnesses remain open; runtime comparisons do not discharge those gates.
2. Reuse follow_word and the arithmetic fixed-code helpers for additional authored traces. Do not run by-compute on the complete fixed Inflate expression: the default interpreter recursion limit is insufficient, and MDec contains existential choice. The accepted pattern proves concrete canonical wire words, isolates each MCompressedStep, then composes MCompressed, header fields, MBlk and MFinish. Full dynamic headers need analogous independently proved length/code-length traces; no speculative dynamic proof was left in the tree.
3. New kills by run are 365 reader-wrong-byte; 366 first-EOF double charge; 367 omitted segment charge; 368 wrong bit shift; 369 wrong alignment; 370 mirrored first-EOF core charge; 371 Huffman LSB-first; 372 incomplete-code admission; 373 skipped dead-prefix rejection; 374 empty code as TruncatedZlib; 375 omitted Huffman initialisation charge; 376 wrong CL permutation; 377 omitted repeat-overflow guard; 378 wrong repeat fill; 379 omitted EOB check; 380 count admission before the third field; 381 repeat-16 extras before previous-entry validation; 382 fixed distance table with only 30 codes; 383 swapped fixed/dynamic BTYPE; 384 output+1 distance admission; 385 wrong distance base; 386 omitted distance extras; 387 reserved distance admission; 388 wrong empty-segment offset; 389 omitted output-limit guard; 392 omitted symbol charge; 393 omitted block charge; 394 omitted CL construction charge; 395 omitted dynamic expansion charge; 396 omitted Huffman decode charge; 397 omitted bit-loop charge; 398 omitted byte-return charge. All use the complete theorem set and have 299 verified, 1 error. Several work mutations fail at an exact boundary assertion; these are instrumentation-sensitivity checks, not a second independent runtime work oracle.
4. Preserve the historical unrun mutation list: partial-EOF consumption/advance; canonical/single-code admission variants; live-count quotient flooring, reverse depth/shift, omitted depth zero and dead-interval boundaries; dynamic EOB/admission-order changes; additional wrong length extras, late Adler deferral, bit-position narrowing and construction/read/decode work misclassification. Omitted EOB and several work charges are now killed, but this does not close every order or misclassification variant. Historical runs 317–328 remain valid evidence for their twelve named cases; the old unmutated reserved-length selector-failure directory still does not count.
5. L-LIMIT-MONO remains unproved and was not attempted in this partial item-4 continuation. Prove it on the independent model before claiming the ICC measuring-to-exact-allocation second pass.
6. Item 5 remains blocked on an actually matching pinned i686 vstd verification cache. Run 409 rechecks inflate/GIF target directories and /home/ws/.cache and finds no i686 .vir candidate. No bootstrap, feature/version/flag/fingerprint change, target substitution or i686 verifier replay occurred.
7. Final runtime evidence is runs 400–407: source/allocation/cap/erased-counter controls, all 26 transcriptions and twelve frozen hashes (400); thirteen release tests (401); release adapter (402); host/thumbv7m libraries and instantiated probes (403–404); symbols (405); every 1,181 extracted stream in four layouts per seed plus 10,000 generated streams and 10,000 corruptions across seeds 26/27 (406–407). The differential makes 33,544 public-entry pairs or 67,088 real calls with zero unexpected differences. Work remains erased and proved, not inspected by the adapter.
8. All jobs have exited. No commit or push was made. No target directory was deleted, cleaned or repaired. Next logged run is 411. Final count is the fresh 300/0 in run 408; runs 409–410 are inventory/reproducer commands, not verifier counts. Only handoff documentation changed after run 408.

No frozen decoding counterexample, forbidden proof escape, trusted addition or solver-limit increase was introduced. E2 is the sole frozen-statement concern newly proposed here, and the sole file edited outside the crate is ERRATA.md; THEOREMS.md, spec-frozen and E1 are unchanged. Accepted Verus 0.2026.11.02.8bf68ee, vstd 0.0.0-2026-11-01-0251, rustc 1.99.1 (58cf8f9c3), CARGO_BUILD_JOBS=1, default limits, two verifier threads and --no-cheating are unchanged. Existing trust is vstd/Verus/rustc; no signed allocation/call-graph audit is claimed. New snapshots: `/home/ws/projects/verified-decoders/png/inflate_verus/backups/astra-before-accounting-framing.tar` (337,920 bytes) and `/home/ws/projects/verified-decoders/png/inflate_verus/backups/astra-before-work-mutations.tar` (378,880 bytes), both below 1 MB.

---

## Public zlib envelope and erased work complete on host, 2026-11-18

**Fresh final-source run 343: 272 verified, 0 errors.** Step (2c) is complete for the implemented Strict/RB field domain on the verified host target: both actual public entry points refine the one independent Inflate model, preserve the caller's original output suffix and source, establish public window/ring behaviour, and return erased per-call work with the manifest's separate and summed bounds. Public source freshness remains exactly `wf()` plus `pos()==0`; no index or counter reset was added. This is not M2 sign-off: the complete witness/mutation gates, L-LIMIT-MONO and the matching 32-bit verification replay remain open.

| Runs | Evidence | Result |
|---|---|---|
| 286–292 | Independent zlib envelope, actual header/BE32/checksum/tail and block composition | Run 292: 253 verified, 0 errors |
| 293–301 | Borrowed-sink/source framing, counter erasure and actual public entries | Run 301: 259 verified, 0 errors |
| 302–306 | Formatting, thirteen release tests, adapter and full differential seeds 22/23 | Passed; 10,000 generated streams plus 10,000 corruptions |
| 307–316 | Twelve authored model witnesses, real public calls and explicit public L-RING indexing | Run 316: 272 verified, 0 errors |
| 317–328 | Twelve implementation-only mutations in disposable copies, all theorems enabled | Each failed verification: 271 verified, 1 error |
| 329 | Matching pinned i686 verification-cache inventory | No candidate in inflate/GIF target directories or `/home/ws/.cache`; no bootstrap attempted |
| 330–332 | Source/cap/erased-counter/provenance controls and format-word-width proof fix | Run 331 gates pass; run 332: 272 verified, 0 errors |
| 333–340 | Final formatting, proof, thirteen release tests, adapter, host/thumb probes, symbols and source/hash controls | Run 334: 272 verified, 0 errors; all runtime/source gates pass |
| 341–342 | Final-source full differential seeds 24/25 | 33,544 public-entry pairs; zero unexpected differences |
| 343 | Fresh final-source replay, adding only the erased summed-work postconditions | 272 verified, 0 errors |

### Exact proved surface

- `/home/ws/projects/verified-decoders/png/inflate_verus/src/model.rs:1710` MFinish independently describes alignment, complete checksum availability, mathematical big-endian value, Adler comparison, exactly-full IgnoreTail and tail rejection. MEnvelope (:1727) performs two-byte availability, FCHECK, CM/CINFO and FDICT in production order, then independent MBlk and MFinish. Inflate (:1749) supplies independent status/out/r2 fields; r2 is false for the implemented domain. No executable options, tables, buffers, ring or ring_repr enter these model definitions. The two boolean parameters encode the only multi-valued implemented fields; the other three fields have one inhabitant, so this is four representable tuples, not a claim about unimplemented Stage-2 values.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:1124` checksum is the actual four-byte production loop and proves exact mathematical BE32, truncation position, preserved reader frame and at most eight core steps. finish (:1172) proves MFinish, including checksum enforcement at exactly L, and costs at most nine steps. run (:1192) proves MEnvelope on every outcome, establishes selected_window/table/blocks preconditions from the real guards, and proves `work_delta <= C_IN*|Z| + C_OUT*produced + C_INFL_FIXED`. Header validation, error precedence and checksum/tail composition are no longer unverified boundaries.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:181` sink_identity preserves the borrowed sink's variant and eventual view through every actual output-mutating helper. Output::new (:210) exports that relation and its original snapshot. The prophetic annotation permits reasoning about an existing mutable reference's future contents; it adds no axiom or trusted body. `/home/ws/projects/verified-decoders/png/inflate_verus/src/bits.rs:65` frame preserves the original source's eventual state and all counter/rank/count origins. Bits::complete (:104) releases the borrow and exports the exact per-reader segment delta. Neither helper resets the source.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/inflate.rs:1267` entry composes those observations. Actual public inflate_zlib_into (:1319) proves exact status/count/model prefix and the untouched suffix relative to `old(out)`, source wf/concat/count preservation and bounded final cursor. Actual public inflate_zlib_measure (:1342) proves exact status/count and the full last-window predicate, also stated explicitly with the manifest's `(k-1-j) mod WINDOW_MAX` indexing. Both source antecedents are exactly wf plus pos()==0; measure additionally retains its documented ring-length requirement. T-INFL(a,b) and its public window/ring/limit/checksum refinement are established on host for every represented tuple and every outcome; the 32-bit quantifier and the milestone assurance gates are not discharged.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/lib.rs:103` InflateResult now has `core_steps: Ghost<nat>` and `seg_steps: Ghost<nat>`. Work (:110) and SegmentSlices (:87 in segments.rs) contain no runtime-u128 work counters; the trait no longer demands an executable diagnostic accessor. Each public result proves `core_steps <= C_IN*|Z| + C_OUT*k + C_INFL_FIXED`, `seg_steps <= C_SEG*(old(src).seg_count()+1)`, their summed bound, and `seg_steps == final(src).seg_steps()-old(src).seg_steps()`. These are per-call observations even when a fresh-by-position source has nonzero prior indices/counters. T12's inflate inequalities, and the inflate portions of T1/T11, are composed at the actual public entries on host, not only helper-local.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/limits.rs:32` caps_ok proves inflate's format/domain/budget component without importing or assuming any of PNG's six configurable resource-cap values. PNG's full cap-tuple lemma and CAPS-SYM remain PNG-level obligations. Run 330 caught checksum proof numerals that collided lexically with two PNG caps; replacing those mathematical word-width facts with u16::MAX/u32::MAX expressions fixed the lint without changing runtime code or weakening it. Runs 331/340 pass its controls. A-PNG-6's inflate build/symbol portion passes under approved E1; no signed allocation/call-graph audit is claimed.
- `/home/ws/projects/verified-decoders/png/inflate_verus/src/witnesses.rs:11` begins twelve authored frozen model-versus-expected proofs with real into/measure calls, including Z=[]. The cases are empty-idat-only, stored-only, cm-7, cinfo-8, fdict, one-byte, bad-adler, missing-adler, partial-adler, trailing-byte, second-stream and stored-nlen. Expected raw bytes/statuses are written directly, not obtained from an oracle. `/home/ws/projects/verified-decoders/png/inflate_verus/tools/audit_witnesses.py` confirms the twelve literal inputs equal the frozen records' IDAT extractions; each has the actual R=4. This is twelve of the 631 core streams, not the complete model-witness gate.

### Exact resume and remaining boundaries

1. Continue item 4 with authored model/public-entry witnesses for the other 619 core streams, plus the required policy witnesses. Do not treat the runtime vector comparisons or their 44 authored error checks as these proofs. L-LIMIT-MONO, the separate cross-limit model theorem used by ICC, is still unproved.
2. Twelve mutations are killed in runs 317–328: omitted NLEN, omitted Adler comparison, IgnoreTail on short output, wrong length-285 base, omitted window guard, wrong match-copy offset, Full at L, omitted copy work charge, early BFINAL termination, reserved length 286, omitted FCHECK and little-endian checksum assembly. `/home/ws/projects/verified-decoders/png/inflate_verus/tools/proof_mutations.py` creates copies under `mutation-cases/`, shares but never removes the target cache, and appends verifier diagnostics through the same verify.sh/lock/default limits. Remaining mutations include the earlier reader/Huffman/dynamic lists, BTYPE swap, distance bases/extras/reserved symbols/output guard and additional construction/read/decode/block/symbol or misclassified work charges. The unmutated `mutation-cases/reserved-length-286/` was a harness selector failure before any verifier invocation; it is not a kill. The corrected admitted-symbol case is run 326.
3. Item 5 is blocked on a genuinely matching pinned i686 vstd verification cache. Run 329 found no i686 .vir candidate in the named directories. Do not substitute thumbv7m, change dependency features, weaken --no-cheating, forge fingerprints or clean any target directory. No i686 verifier replay was attempted or claimed in this continuation.
4. The runtime adapter now reports status/count/prefix/cursor, checks untouched suffix and last-window contents, and compares the two actual public entries. It cannot observe erased counters; the public proof carries their bounds. Runs 305/306 retained an obsolete final stdout phrase saying observed work, although those assertions had already been removed; corrected final runs 341/342 explicitly state that work is erased and proved. Reintroducing runtime counters to satisfy that old diagnostic protocol would violate the manifest.
5. All jobs have exited. No commit or push was made. Next logged run is 344. The fresh count is run 343, not a cached Cargo exit-0 invocation. The only source change after final runtime gates was the erased summed-work postcondition.

Final runtime evidence: thirteen release tests in run 335; release adapter in 336; host and thumbv7m instantiated-entry library probes in 337–338; allocator symbols in 339; allocation/source/cap/erased-counter controls, twelve witness transcriptions and all twelve frozen hashes in 340. Runs 341–342 check every 1,181 extracted stream in four layouts per shard, plus 10,000 generated streams and 10,000 single-bit corruptions across seeds 24/25: 33,544 public-entry pairs or 67,088 real public calls, zero unexpected differences. Earlier seeds 22/23 also pass after counter erasure.

No frozen counterexample/new erratum, forbidden proof escape, trusted addition, solver-limit increase, frozen/manifest/errata edit or target deletion/repair occurred. E1 is APPROVED, not an open dependency-wording blocker. Verus 0.2026.11.02.8bf68ee, vstd 0.0.0-2026-11-01-0251, rustc 1.99.1 (58cf8f9c3), CARGO_BUILD_JOBS=1, two verifier threads and --no-cheating are unchanged. Existing trusted boundaries remain vstd/Verus/rustc; the allocation allowlist/audit is not supplied by this proof. New backups, each below 1 MB: `/home/ws/projects/verified-decoders/png/inflate_verus/backups/envelope-before-layer3c.tar` (419,840 bytes) and `/home/ws/projects/verified-decoders/png/inflate_verus/backups/public-envelope-before-witnesses.tar` (307,200 bytes).

---

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
