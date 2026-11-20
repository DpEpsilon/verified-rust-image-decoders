# png_verus ledger

M3 has begun; it is not signed off. The final host checkpoint is **run 24: 78 verified, 0 errors**, with `--no-cheating`, default solver limits and one verifier thread. `inflate_verus` remains an unmodified path dependency. Its target directory was read only to copy the pinned vstd cache into this crate's target directory.

## M3 checkpoint (2026-11-20)

The root crate now completes a nonzero, zero-error Verus run. This is not a claim that all existing code is under `verus!`: the CRC, wire framing, complete header prefix, keyword helpers, error-name helpers, allocation guard and re-walk adapter are verified; `walk.rs`'s ordering/policy loop, `image.rs`, the public entry-point bodies and allocation sequencing remain ordinary Rust. `model.rs` defines the independent bitwise CRC, P3 framing, `mh` header prefix, keywords and a lengths-and-types reader stream. It does **not** yet define the complete `MW`/`MW_p`/`MWF_p` document model.

| THEOREMS row or M3 gate | Checked statement and named proof/call binding | State at run 24 |
|---|---|---|
| T2 | `header::prefix` equals `model::mh` for every input, including exact first error and complete IHDR fields; `header::parse_ihdr`, `header::area_cap_u64` prove field checks and unnarrowed area arithmetic; the real `walk` calls this prefix | Header theorem proved on host; required frozen-record witness coverage and 32-bit replay still open |
| L-CRC-TABLE, T6 checksum sub-obligation | `crc::make_table` proves all 256 entries equal `model::crc_bits(i, 8)`; the verified static `TABLE` initializer preserves that fact; `crc::lemma_crc_update` proves the symbolic table-update equation; `crc::crc32` equals preset-and-complemented `model::crc` on every slice | Construction/update/message equations proved; Q6 asks about T6's literal “Crc on single bytes” wording |
| T6 framing and keywords | `framing::frame_at` equals `model::frame` on lengths, letters and exact first framing error; `framing::chunk_at`/`crc_matches` bind the CRC comparison to the bitwise model; `keywords::{valid_keyword,find_nul,split_keyword}` equal P8.2/first-NUL models | Partial; duplicate/order automaton, per-chunk values, APNG counters and complete MW equality untouched |
| T-SEG | `reader::DataReader::read` satisfies every inherited M2 trait clause, including source-frame preservation, fresh-cursor construction, exact byte/EOF behaviour, segment/core counters and termination; `model::reader_shape` and `DataReader::at` prove `12 * seg_count <= input.len()` | Adapter proved against a lengths-and-types stream; partial row because the caller's input bound and equality with MW's selected ranges remain open |
| L-REWALK, Strict | `reader::{segment_at,DataReader::read}` implement the specified lengths-and-types scan; fdAT mode stops at the next fcTL/IEND, and skipped non-fdAT chunks are empty segments | Partial transport foundation; no theorem yet starts from `MW(b) = Ok(doc)` |
| L-REWALK, policy form | No proof yet connects successful animation selection under every Strict/RecommendedBeta-valued record to CRC-validated fcTL boundaries; Static's no-fdAT claim still rests on the M1b inspection/tests below | Untouched as a policy theorem |
| T10 walk prefix | T2 and local P3/keyword first-error equivalences above | Partial; whole-walk-before-reservation/inflate precedence unproved |
| A-PNG-2, A-PNG-7 | No new entry-point/log theorem; inherited plain callers still pass `Tracked::assume_new()` outside verified bodies | Untouched |
| L-WALK-FIRST | No new Verus theorem beyond the existing M1b inspection/runtime evidence below | Untouched |
| L-INFO-RANGE | Framing bounds are proved, but no proof covers a returned `Info` range or equality with `MW_p` metadata; callers must still supply the original input | Untouched |
| M3 witness gates (THEOREMS.md:194) | `witness_m3::{crc_known_answers,header_error_precedence}` bind actual executable calls to concrete CRC constants and two header-error results | Incomplete; not the required M3 frozen-record witness set |

All proof claims in the table refer to **run 24**. `evidence/claims-m3-start.json` indexes these claims by requirement; `evidence/checkpoint-0024.json` binds the count to the source hashes and pinned versions. `evidence/SHA256SUMS` retains hashes of the selected full run transcripts, checkpoint/control records and cache evidence. No new `assume`, `admit`, `external_body`, `assume_specification` or trusted item was added; the sealed guard files retain their original hashes.

The reader migration was required for a green baseline: a plain `Segments` implementation cannot compile under Verus against M2's trait because its ghost members are missing. The two old readers are now aliases of one verified `DataReader`, with IDAT/fdAT constructors selecting the mode. This adds no runtime allocation and recomputes no CRC. The constructor requires `input.len() <= inflate_verus::limits::SEGMENT_BYTES_MAX`; the future verified entry-point callers must derive that precondition from the walk's input guard and `caps_ok`, not assume it. This dispatch does not claim that connection. The fdAT reader charges each intervening non-fdAT chunk as an empty segment, which preserves its concatenated byte stream and accounts for the actual scan work.

Tooling: `verify.sh` takes the shared verifier lock and runs serially in the foreground with a 25-minute bound, `CARGO_BUILD_JOBS=1`, `--no-cheating` and no solver-limit overrides. It selects one verifier thread while `pgrep` finds `corpus_pass.py`, otherwise two, and forces output into this crate's `target`. `tools/verify_m3.py` accepts only this root crate's fresh positive count, unchanged source hashes and `Fresh vstd`; a dependency's count cannot satisfy the gate. Run 20 deliberately repeats an unchanged cargo invocation: cargo exits zero, but the checkpoint rejects it for no fresh root count (`accepted=false`). `RUNLOG.tsv` records cargo's exit status; the per-run JSON records checkpoint acceptance separately. A rerun of unchanged sources can therefore be refused without invalidating run 24.

Pinned environment: Verus **0.2026.11.02.8bf68ee**, rustc **1.99.1 (58cf8f9c3)**, vstd **0.0.0-2026-11-01-0251**, `default-features=false, features=[alloc]`. The cache copy is hash- and mtime-recorded in `evidence/vstd-cache-copy.json`; the final comparison is in `evidence/vstd-cache-after.json`. Cargo reports `Fresh vstd` in every verbose verification run; no vstd verification rebuild occurred. The dependency enables inflate's existing `split` base feature: it includes all production code and public-entry proofs, while omitting only the optional M2 witness leaves. Run 1 verified that dependency base locally (374 verified, 0 errors) before the PNG compilation failed; those obligations are not added to the PNG count. All outputs went into PNG's own target directory.

Validation on the final source tree: run **25**, `cargo test --all-targets -- --nocapture --test-threads=1`, passes. Its conformance output is **808 passed, 0 failed, 2 skipped** of the 810 records: the inherited ImageCrateCompat/P2 exception is still real, not 810 passes. Fault injection passes; the council tests report 6 passed and the inherited phase-2 vector-presence test ignored. Run **23** passes both source lints and their negative controls with both sealed guard hashes unchanged. No test file, policy vector, frozen specification or protected document was edited. This dispatch did not repeat the M1b differential corpus/probes, add a supported-policy claim, or replay proofs on i686.

Resume at the fixed-size ordering automaton and independent `MW` document model: model singleton sets independently of the executable bit mask, connect their representation, then add per-chunk values and APNG state with exact first errors and ghost payload ranges. Connect the existing verified reader stream to those ranges separately for Strict and the policy form. Only then migrate the logged public callers for A-PNG-2/A-PNG-7, L-WALK-FIRST and L-INFO-RANGE. Q6 is a question, not an authorised manifest correction.

## Historical M1b state (2026-11-19)

At the M1b checkpoint no Verus run had been made; the following records retain their original runtime/inspection scope.

M1b done, with one scoped exception (the two `ImageCrateCompat-0.25.10` records, P2, `QUESTIONS.md` Q3).

| Gate (PLAN.md M1b) | Result |
|---|---|
| `spec-frozen/VECTORS.json` (599 Strict) | 599 pass |
| `limits-draft/VECTORS-CAPS.json` (13, both `pad_to` records included) | 13 pass |
| `policy-draft/VECTORS-POLICY.json` (198) | 196 pass, 2 skipped (ImageCrateCompat, P2), 0 fail |
| Probe inputs vs `png_oracle.py` (Strict) and `policy_oracle.py` (RecommendedBeta) | 1,607 distinct inputs, 0 differences on each of six entries |
| Source lint: cap numerals (`tools/lint_caps.sh --self-test`) | clean; controls pass |
| Source lint: allocation outside the guard (`tools/audit_alloc.sh --self-test`) | clean; call sites 1/1/1/1/1 + 1 empty; guard hashes OK |
| Leniency runner `verified` (`~/projects/leniency/png`) vs `png_oracle.py` | 10,000 harness inputs, 0 differences, both selectors |
| Allocation shape (`tests/fault_injection.rs`) | `Req` exact, each reservation failed in turn gives its `OutOfMemory` |
| `no_std` builds | x86_64, i686, `thumbv7m-none-eabi` library builds pass |

Council rulings, phase 1 (2026-11-20; section below). After the code changes: `cargo test --release` 808 pass and 2 skipped of the 810 records (unchanged: no recorded expectation depends on the rulings), `fault_injection` passes, `tests/m1bq_rulings.rs` 6 pass and 1 ignored (`m1bq_policy_vectors_present`, pending phase 2); `tools/lint_caps.sh --self-test` and `tools/audit_alloc.sh --self-test` clean, guard hashes OK; `thumbv7m-none-eabi` library build passes. Negative controls: with the old `image_status` arm the two Q1 tests fail, with the old fcTL handling the two Q2 tests fail. The probe row of the table predates the rulings and was not re-run: `policy_oracle.py` still reports `TruncatedImageData` for the Q1 class and splices the Q2 examples, so `png_verus` now differs from it on those inputs until phase 2 changes the oracle.

## Layout

- `src/limits.rs`: the six P1 caps, the only Rust that writes them (draft values from `../limits-draft/limits.py`, held out of Stage 1), and `caps_ok`.
- `src/policy.rs`: the 30-field record, `Profile` (`#[non_exhaustive]`: Strict, RecommendedBeta). The fifteen held-out RecommendedBeta rows are written once, in `RECOMMENDED_BETA`.
- `src/walk.rs`: P2–P8, P14.1 under a policy record; a fixed-size `State`, no chunk or frame list; returns `Doc` (IHDR, byte ranges of PLTE/tRNS/iCCP, offset of the first IDAT, acTL, `Info`) or `WalkFail::{Format, Fallback}` (R4).
- `src/reader.rs`: `IdatReader`, `FdatReader` (`inflate_verus::Segments` over re-walked chunks, framing only, no CRC re-check: since the council's M1b-Q2 ruling every fcTL a successful frames walk passed has a verified CRC), `next_fctl`.
- `src/image.rs`: `R(w,h)`, Adam7 passes, inflate into `Raw`, in-place unfiltering, P13 conversion, `Sink` (RGBA8, RGBA16 BE, APNG canvas with P14.4 OVER).
- `src/png.rs`: `Error` (62 names, inside `verus!`), `Info`/`Meta`, entry points, `Frames` (`try_new(b)` Strict, `try_new_with_policy(b, p)`, and the test-only `try_new_with_record` behind the `test-records` feature), ICC, the runtime request `Trace`.
- `src/alloc_guard/mod.rs`: byte-identical to every core (`add634bc…`); `sites.rs`: PNG's sites `Raw`, `Output`, `Saved`, `IccWindow`, `Icc`, `TryError`, `log_tied`, `try_empty` (D8's zero-element request). The plain code passes `Tracked::assume_new()` logs, as GIF's M1 did; M3–M6 replace them with the sealed log.
- `tests/conformance.rs` (810 records), `tests/fault_injection.rs`, `tests/m1bq_rulings.rs` (the council's M1b-Q rulings on inputs built in the test, expectations from the ruling text), `examples/png_probe.rs` (differential adapter), `tools/probe_diff.py`, `tools/lint_caps.sh`, `tools/audit_alloc.sh` (reuses `../../gif/tools/alloc_scan.py` in place).

## Walk first (L-WALK-FIRST, council obligation for M3; where M1b enforces it)

Every entry point completes the walk before its first reservation or inflate call, and no inflate call exists in `walk.rs`:

- Stills: `png.rs` `still_impl` calls `still_walk` (line 376) before `reserve_image` (378) and `IdatReader::new` (379); `decode_image` (`image.rs:337`) is the only `inflate_zlib_into` call for image data.
- Frames: `Frames::with_record` (behind `try_new` and `try_new_with_policy`) walks (521; on `Fallback` re-walks with APNG off and sets R4, 526) before `reserve_image` (536) and the default image's decode (537). fdAT data is inflated only in `try_next` (`FdatReader::new`, 644), only for a fcTL found by `next_fctl` (585), only after the constructor succeeded, and never in the R4 fallback (`FState::Single`, no fcTL is visited).
- A fcTL whose CRC does not verify is a fcTL that failed its checks (council ruling M1b-Q2, the clarification of L-WALK-FIRST in `../PLAN.md` M3). Under Strict the walk returns `CrcMismatch` at it. Under RecommendedBeta the frames walk raises the structural `CrcMismatch` at it (`walk.rs:434`), which `Static` turns into the R4 fallback, before any fdAT inflate call or frame `Raw` reservation; the fallback re-walk validates the rest of the file and decodes only the default image. So no entry inflates fdAT data of a file containing such a fcTL, and every fcTL at which `FdatReader` or `next_fctl` starts or stops a frame passed its CRC check in the walk; the readers no longer recompute it (policy form of L-REWALK, below).
- ICC: `icc_impl` calls `still_walk` (679) before `IccWindow` (688).

## Council rulings on the M1b questions (2026-11-20)

Unanimous, report M1BQ; full text in `../policy-draft/DECISIONS.md` "Council M1b-Q rulings"; resolutions in `QUESTIONS.md`.

- **Q1 (b).** `image_status` no longer renames `TrailingZlibData` (`src/image.rs`): a short image or frame stream followed by bytes after a matching Adler-32 is `TrailingZlibData` under every record with `IgnoreTail`, as under Strict. `inflate_verus` unchanged.
- **Q2 (c).** A bad-CRC fcTL in the frames constructor's walk is the APNG structural error `CrcMismatch` (R4 under RecommendedBeta); still decoding and the fallback's re-walk drop it (`src/walk.rs`, `src/reader.rs`).
- **Q5b (b).** `Frames::try_new(b)` (Strict) and `Frames::try_new_with_policy(b, p)`; `try_new(b)` is `try_new_with_policy(b, Profile::Strict)`, so errors and guard requests are identical. Callers updated: `tests/conformance.rs`, `tests/fault_injection.rs`, `examples/png_probe.rs`, `~/projects/leniency/png/verified-runner/src/main.rs`.
- **Policy L-REWALK obligation (M3).** Establish separately, for successful fdAT animation selection under every Strict/RecommendedBeta-valued record, that every fcTL in each frame's re-walk span passed CRC validation. The re-walk stops at the next fcTL or IEND and visits exactly the selected frame's fdAT payloads in order, without CRC recomputation to identify boundaries. The `Static` fallback selects no fdAT animation stream. The Strict lemma alone does not discharge this policy obligation.
- **`Still16::rgba16` (Q5a).** Big-endian bytes so that the shared, hash-pinned `alloc_guard` stays byte-identical across cores; no `u16` allocation site exists. Obligation: prove that the output byte length is `8·W·H` and that decoding adjacent byte pairs as big-endian yields exactly P13.3's sample sequence (T13 through `sample i = 256·bytes[2i] + bytes[2i+1]`, the notation entry proposed in `../ERRATA-PROPOSED-M1BQ.md`). The `Output` allocation stays byte-counted and the shared guard unchanged.
- **L-INFO-RANGE (Q5c), an M3 sign-off gate.** For every input `b`, claimed profile and returned `Meta::Present((o, n))` in an `Info`: `o ≤ |b|` and `n ≤ |b| − o` (so `o + n ≤ |b|` without overflow), and `b[o .. o+n]` equals, without copying, the byte range that `MW_p(b)` assigns to that item. This ledger will say which part Verus checks and which rests on inspection or runtime evidence, and keeps both apart from the caller's precondition that it supplies the original input. Checked accessors are not required by this ruling; any accessor added later must be memory-safe for any supplied slice, and its metadata correctness additionally requires the original input.
- **RGBA16 policy evidence (Q5d).** `try_decode_still16_with_policy` is a provisional implementation of D16, not an oracle-validated or proved policy result: it is covered by no conformance record, no differential probe (the six-entry probe supplies no RecommendedBeta RGBA16 evidence) and no M1b claim. P1 credit or a supported-policy claim requires an independent `MS16_p` model and a `decode_still16` policy oracle in `policy_oracle.py` written from the D16 text, not transcribed from `image.rs`; the entry added as a seventh probe entry, comparing returned byte pairs as big-endian samples; R-still for the 16-bit pair; and RecommendedBeta 16-bit records covering R3 opaque-black substitution, palette and tRNS expansion, a 16-bit source whose low bytes are non-zero, and equality with Strict on Strict-accepted inputs.

## Commands and results

Environment: `PATH=/opt/cargo/bin:$PATH` (rustc 1.99.1, 58cf8f9c3), crate target dir `png_verus/target`, `-j2`, `--offline`.

```sh
cd /home/ws/projects/verified-decoders/png/png_verus
timeout 900 nice -n 10 cargo test --offline -j2 -- --nocapture        # conformance (808 pass, 2 skipped) + fault_injection
timeout 600 nice -n 10 cargo build --offline -j2 --release --example png_probe
timeout 3000 python3 -I tools/probe_diff.py --runner target/release/examples/png_probe --out runs/probe-diff-2.json
timeout 120 tools/lint_caps.sh --self-test
timeout 300 tools/audit_alloc.sh --self-test
timeout 600 cargo build --offline -j2 --lib --release --target thumbv7m-none-eabi    # also i686-unknown-linux-gnu
cd /home/ws/projects/leniency/png && ./build-verified.sh && timeout 7200 python3 -I check_verified.py --inputs 10000
PYTHONDONTWRITEBYTECODE=1 python3 -I tests/test_png.py PngTests.test_verified_agrees_with_strict
```

Run log (2026-11-19):

1. First build: compiles; `Segments` is implementable from plain Rust with `read` alone (the trait's spec items are erased outside Verus).
2. `cargo test --test conformance`: 599/599, 13/13, 196/198 + 2 skipped. Negative control (16 records with six expectations altered: an error name, an ICC length, one pixel, a frame's blend, an events list, a cap-record error): exactly those six fail.
3. `probe_diff.py` run 1 (`runs/probe-diff-1.log`): 1,607 inputs (vectors, `results.json` 748 records with the 14 large ones regenerated, held-out A and B, ICC probes), 0 differences on Strict still, still16, frames, icc and RecommendedBeta still, frames; oracle accepts 446 Strict stills, 919 RecommendedBeta stills.
4. Cap lint: first run flagged three comments spelling 2^31; reworded to `i32::MAX`; clean, 18 negative controls caught.
5. Allocation lint: test-only helpers returning `Vec` and a method named `push` flagged by design; replaced with fixed arrays and `record`; clean in both scanner modes.
6. `check_verified.py`: 300 inputs, then 10,000 (`runs/leniency-check-10000.log`): 0 differences for `png:verified` (frames) and `png:verified-still`; Strict accepts 939.
7. `tests/fault_injection.rs`: passes (still RGBA8/RGBA16 `[(Raw, R), (Output, 4N|8N)]`; walk error reserves nothing; frames `Saved` once at the first PREVIOUS frame `k > 0`, its failure returns `OutOfMemory` once and then `Ok(None)`; ICC `[(IccWindow, 32768), (Icc, m)]`, `m = 0` without an allocator call).
8. `probe_diff.py` run 2 after the refactor (`runs/probe-diff-2.log`): identical to run 1.

## Not done in M1b (by scope)

- ImageCrateCompat and GdkCompat (P2).
- `chunks(b, p)` with `ChunkStatus`, and `try_icc_profile_with_policy`, are not M1b gates and are absent from the crate (no stub, no `unimplemented!`); neither is cancelled (council ruling M1b-Q4). Each is implemented at P1 only after its model exists in `policy_oracle.py`: `chunks` after `MChunks_p` and T-CHUNKS under D15 (reportable fatal walk errors, in-bounds borrowed ranges, no guard allocation, no `Accepted` status for an unverified CRC), `try_icc_profile_with_policy` after `MI_p` under D14 (RecommendedBeta's `icc_errors`, ICC tail handling, `Absent | Discarded | Failed | Present`, structural and allocation errors, projection to the Strict ICC call). Both remain P1 deliverables whose policy contracts freeze at Stage 2. M1b is complete without them. Until then the recorded PNG-Q4 and PNG-Q13 capabilities are delivered only in part: `Info`'s three-valued `Meta` and the Strict `try_icc_profile` exist; text chunks, per-chunk status and `Icc::Discarded | Failed` do not, and integration must claim none of them.
- The leniency harness's other runners were not rebuilt (`../gif/build.sh` clones and builds glycin/image under `/tmp/claude-0`, which a restart cleared); the `verified` runner was checked against the Strict model, not re-voted against the incumbents. `~/projects/leniency` is not a git repository; the public copy `verified-decoders/leniency/` has no PNG adapter and was not edited (syncing is the coordinator's call, `SYNC-TO-REPO.md`).

## Resume point

M1b is complete; the next milestone is M3 (chunk walk proofs, `../PLAN.md`). To resume or re-check: run the command block above from a fresh shell (all scratch inputs regenerate under `/tmp/claude-0`). Q1, Q2, Q4 and Q5 are decided (council, 2026-11-20); phase 1 of applying them is done (see "State"). Phase 2 is pending: `policy_oracle.py`, the new RecommendedBeta records in `VECTORS-POLICY.json`, `validate.py`, then a fresh `probe_diff.py` run (run 2's zero differences predate the rulings) and the ignored test `m1bq_policy_vectors_present`. Open for the coordinator: Q3 (two compatibility records). Nothing is committed.
