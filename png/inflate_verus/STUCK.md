# Huffman foundation checkpoint: 70 verified

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
