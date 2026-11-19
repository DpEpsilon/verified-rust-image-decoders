# Allocation-free PNG inflate

Fresh host split checkpoint **617–622** and fresh `i686-unknown-linux-gnu` replay **623–628** each report **1,381 unique verified functions, 0 errors**, on source tree `f56fa5f7…1c5df`, with equal per-pass counts. The two added verified functions are the Adler deferral-bound specification and maximality lemma. The replay reused the existing pinned cache in `target/i686-verify`; `I686-CACHE.md` records the provenance. Earlier checkpoints 592 and 603 no longer bind the changed source tree.

## Running a verifier checkpoint

`/home/ws/projects/verified-decoders/png/inflate_verus/verify.sh [label]` runs one checkpoint: the six passes of `tools/verify_passes.json`, serially, under one held verifier lock. Each pass is its own `run_logged.py` run with the 25-minute bound, `CARGO_BUILD_JOBS=1`, `--num-threads 2`, `--no-cheating` and `--output-json`. The sequence stops at the first failing pass; a complete checkpoint writes `runs/checkpoint-NNNN.json`. `verify.sh --only PASS [label]` runs one pass and is never a checkpoint. `python3 -I tools/split_checkpoint.py --check` runs the coverage check without the verifier.

Passes select modules at compile time, because the pinned Verus rejects `--verify-module` and `--verify-root` under `--no-cheating`. Pass `base` (feature `split`) verifies the crate root, the core modules and the twelve witness modules that other witness modules import. Every other pass verifies base plus its own leaf witness modules, enabled by its `split-NAME` feature through a `#[cfg]` gate in `src/lib.rs`. A build without features compiles every module, as before. The coverage check fails if any module containing `verus!` code is in no pass or in two passes, or if the manifest, the gates and the Cargo features disagree. Each pass log must contain one JSON report with 0 errors and `is-verifying-entire-crate`, list functions of every module in base and in its own pass, and list none from other passes.

Every pass re-verifies base, so the plain sum of pass counts (3,251) counts base's 374 functions six times. Subtract five repeated base copies: 3,251 − 1,870 = **1,381 unique verified functions**. Cargo replays a cached result without running Verus when a pass's features and Rust sources are unchanged since an earlier successful pass; the replay has no JSON report and fails its check, so rerunning a checkpoint needs a changed Rust source.

Independent authored witnesses cover **615 of 631 extracted core stream instances; the other 16 have no legal independent R and are excluded by council decision**. All named M2 gates remain credited separately from this supplementary count. No fallback R is invented. Historical monolithic run 582 proved 1,379 functions before the Adler certificate was added; it is retained as historical evidence, not a source-equivalent reference for the current count.

Every witness/member/layout/policy claim has an audited exact public-call descriptor. Required evidence records and source archives have retained hashes, and checkpoint 617 binds all 46 source, build and checkpoint-tool files. The mutation audit additionally checks numbered requirements, selector definitions, retained diagnostics, implementation revisions and mutation 39's full split source hash. Current resource-blocked metadata is empty; resolved default-limit attempts remain historical evidence. **All 46 audit tests pass in run 629**, including nine new mutation-index controls and a checkout without `runs/`; complete source/provenance/cap controls and structural split coverage pass in **630**.

Production runtime bodies are unchanged from 5f697fa. Complete runtime evidence 566–572 remains applicable: thirteen tests, host/thumb instantiated allocator probes and symbols, all vector layouts, 10,000 generated streams and 10,000 corruptions, no unexpected differences. Eighteen policy records are proved as inflate outcomes outside the core numerator, not integrated PNG policy results.

Mutation **39** is killed in run **613**, 371 verified and 1 failed proof: the implementation-only stored-payload EOF branch resets the count while retaining TruncatedZlib, violating `stored_payload` postconditions. The unchanged D5 companion witness requires `[0,65]` and produced=2 at limits 4 and 2 through both public entries, so the reported count loss is observable. `evidence/mutation-index.json` maps all forty requirement numbers to their evidence, separates 52 additional mutations and marks 13–36, 38 and 40 as not M2. The historical total is **68**, including mutation 12 as **ghost representation narrowing**, not an executable narrowed-counter experiment. No executable narrowing mutant was added.

`lemma_adler_nmax_maximal` and `deferred_b_bound_twice` are exported for proof clients: the deferred bound is safe at NMAX, fails at NMAX+1 and fails for every larger deferral. The E2 and header-loop comments now reflect the established obligations. The coordinator decides M2 sign-off after checking the fresh evidence; this crate does not declare it. Caller integration, L-WALK-FIRST, per-reader effects and allocation obligations remain separate M3–M6 work.

## Historical M1a implementation

The sections below describe the accepted M1a checkpoint, whose dependency-free source is preserved in its backup.

M1a plain Rust checkpoint, 2026-11-17. Runtime tests pass; **zero Verus obligations have been proved**. M2 has not started. The crate implements PNG-CORE-SPEC 1.0 P9–P10 without changing the frozen documents.

The frozen manifest requires `#![no_std]` **without `alloc`**, overriding the task's initial `no_std+alloc` wording. This library has no dependencies and no allocation guard. Callers own the output and the measuring ring. The PNG crate will own the hash-pinned guard and the `IccWindow` reservation; copying that guard into this crate would violate the manifest's allocation shape. Huffman tables use fixed-size arrays.

## Historical M1a API and behaviour

- `inflate_zlib_into(src, out, options)` uses `out.len()` as the limit and preserves the unwritten suffix on every outcome.
- `inflate_zlib_measure(src, ring, limit, options)` requires a ring of exactly `limits::WINDOW_MAX` bytes, with arbitrary initial contents.
- Both entry points require a fresh cursor satisfying the documented `Segments` contract and return `InflateResult { status, produced, core_steps, seg_steps }` on every outcome.
- `Status` distinguishes `Done`, `Full` and the eleven inflate errors; `Full` means another output byte exists, not that the output merely reached the limit.
- Stored blocks retain each present byte that fits before truncation; availability precedes the limit check for the next stored byte.
- Matches check the produced count and selected window before copying and resolve overlap against the evolving output or ring.
- Canonical construction rejects oversubscription and inadmissibility before lookup; decoding rejects dead prefixes before attempting another bit read.
- Fixed distance codes contain all 32 five-bit codewords; decoded symbols 30 and 31 fail as `InvalidSymbol`.
- Adler-32 reduces its running sums every `ADLER_NMAX` bytes and checks the final checksum even when output exactly fills the limit.

The five option fields expose only Strict/RecommendedBeta values. `InflateOptions::STRICT` and `RECOMMENDED_BETA` are provided; the two independent window/tail fields also permit their mixed combinations. Stage-2 compatibility values are absent. `IgnoreTail` ignores bytes after a matching Adler-32 only when `produced == limit`, following the manifest and OPTIONS' exactly-full restriction.

`SegmentSlices::new(parts)` borrows the segment list, checks T-SEG's concatenation-length ceiling, and starts a fresh cursor. Construction does not allocate; its traversal precedes the inflate call and does not enter that call's work counters. Empty segments participate in `seg_steps`; repeated EOF reads do not repeatedly charge the final advance.

## Historical M1a evidence and replay

The logged runner fixes `CARGO_BUILD_JOBS=1`, bounds the foreground command, selects the crate's working directory, and records success or failure in `RUNLOG.tsv` and `runs/NNNN.log`.

```sh
python3 -I /home/ws/projects/verified-decoders/png/inflate_verus/tools/run_logged.py --label 'Rust tests' --timeout 120 -- cargo test --all-targets
python3 -I /home/ws/projects/verified-decoders/png/inflate_verus/tools/run_logged.py --label 'Release adapter' --timeout 120 -- cargo build --release --example inflate_runner
python3 -I /home/ws/projects/verified-decoders/png/inflate_verus/tools/run_logged.py --label 'Differential seed 1' --timeout 175 -- python3 -I /home/ws/projects/verified-decoders/png/inflate_verus/tools/differential.py --fuzz 5000 --seed 1
python3 -I /home/ws/projects/verified-decoders/png/inflate_verus/tools/run_logged.py --label 'Differential seed 2' --timeout 175 -- python3 -I /home/ws/projects/verified-decoders/png/inflate_verus/tools/differential.py --fuzz 5000 --seed 2
python3 -I /home/ws/projects/verified-decoders/png/inflate_verus/tools/run_logged.py --label 'Source controls' --timeout 120 -- python3 -I /home/ws/projects/verified-decoders/png/inflate_verus/tools/audit_source.py --self-test
```

Final-source differential runs 12–13 pass 10,000 generated streams and 10,000 single-bit corruptions against Python zlib, with zero unexpected differences and no observed allowed differences. They also compare every corrupted stream's exact first error and written prefix against a test wrapper around the frozen inflate. Each run checks 1,181 extracted vector streams across four layouts: 631 from the 599 frozen records, 140 from the 13 cap records, and 410 from the 198 policy records. Layouts are contiguous, bytewise with empties, irregular with empties, and original PNG payload boundaries. These are inflate observations, not 810 whole-PNG conformance results. ImageCrateCompat-labelled records run through Strict inflate, not through unimplemented compatibility options.

Every adapter request calls both actual public entry points and compares status, produced count and both diagnostic counters. The adapter checks the exact written prefix against Python, the untouched suffix, the ring's last-window contents, cursor bounds and both observed work inequalities. Additional tests cover all five D4 short-input witnesses, D5 stored-prefix/limit precedence, an actual call on empty `Z`, eight extractor-presence cases, ring-wrap/overlap cases, all-255 Adler deferral, repeated EOF, a measuring limit of `u64::MAX`, and one million empty segments.

The harness uses a bounded output buffer for 278 extracted streams whose raw shape is invalid or exceeds its buffer ceiling; it explicitly reports them as non-R calls. Seven are frozen malformed-container/IHDR records. All inflate-specific M2 gate records use their image's actual R. No non-R call is a model witness proof. Forty-four frozen inflate-error expectations are checked directly against authored vector errors; success bytes and general error prefixes are compared with the frozen reference, not independently authored raw-byte expectations.

Library builds pass on x86_64, i686 and freestanding `thumbv7m-none-eabi`. Source checks reuse GIF's allocation scanner in both modes, run its positive/negative controls, reject copied cap numerals with a separate controlled lint, and confirm all twelve Stage-1 hashes. These are falsification gates, not the signed production call-graph audit.

## Historical M1a proof boundary and next step

The current counters are runtime `u128` diagnostics, not the manifest's erased `Ghost<nat>` observations. T-SEG's `concat`, `wf` and cost contracts currently exist only as prose. There is no independent mathematical model, `caps_ok`, verified loop variant, public-entry refinement theorem, model-versus-authored witness proof or verifier mutation kill. No T-HUFF, L-CANON, T-INFL, T-SEG, T1, T11, T12 or allocation theorem is claimed.

Start M2 with the independent canonical-code model and Huffman layer, then continue in the assigned layer order. Use only `verify.sh` for Verus: it enforces a shared verifier lock, checks for legacy verifier invocations, sets one cargo job, and applies a 25-minute timeout with `--no-cheating` and two verifier threads. It refuses the M1a tree, which contains no `verus!` obligations. Before the first proof run, add the pinned no-alloc vstd dependency and seed compatible prebuilt verification artifacts from GIF if necessary; never delete `target/debug` to force a rebuild. `STUCK.md` records the exact resume points and coverage gaps.

M1a used rustc 1.99.1 (`58cf8f9c3`, 2026-10-13), cargo 1.99.1 (`5f94df478`, 2026-08-27). No Verus or vstd artifact participated in this dependency-free checkpoint. Future proof reports must identify their actual verifier/vstd versions and trust boundary.
