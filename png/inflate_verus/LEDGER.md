# Inflate proof ledger

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
