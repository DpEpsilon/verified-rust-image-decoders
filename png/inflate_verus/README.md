# Allocation-free PNG inflate

Fresh final-source run 510 reports **767 verified, 0 errors**. Both actual public entries retain their independent-model, output/suffix, source-frame and erased-work contracts. The independently authored subset covers **563 of 631 frozen core stream instances**, up by 220 this round. A balanced fixed-match/effect bridge checks length/distance words, extras and overlap against authored raw prefixes; a balanced independent Adler bridge and separate input constructors support larger stored classes. The new 128 classes cover 217 instances across fixed literals, matches and stored payloads; three small original-segment error instances complete the increase. Independent encoder intent now extends through filters, tRNS, interlace edges and APNG sample operands. The audit checks every new transcription/member and derives each fdAT stream's own frame R. Ten additional policy records are proved outside the core numerator, including the distance-before-output window-policy family under all four window/tail choices at R and R+1. **68 core instances remain**, including twelve streams blocked by five named default-solver-limit queries; exact diagnostics and source snapshots are preserved, with no raised limits. Sixteen incomplete instances lack a legal independent image/frame R and receive no invented fallback limit. Large segment/window/zlib gates, excess-output policy families, the beyond-window policy family, further mutations and matching pinned i686 replay remain open. The historical mutation total remains 63. Full runtime evidence passes in 511–518 and final source/provenance controls in 520, including thirteen tests, host/thumb instantiated allocator probes, all vector layouts, 10,000 generated streams and 10,000 corruptions with zero unexpected differences. Run 519 finds no matching i686 verifier cache. E1 remains APPROVED and E2 APPROVED WITH AMENDMENT; the earlier SegmentSlices frame discharge and separate M3 obligations are unchanged. This is not M2 sign-off or a signed allocation audit. `/home/ws/projects/verified-decoders/png/inflate_verus/LEDGER.md` and `/home/ws/projects/verified-decoders/png/inflate_verus/STUCK.md` give the exact handoff; next logged run is 521.

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
