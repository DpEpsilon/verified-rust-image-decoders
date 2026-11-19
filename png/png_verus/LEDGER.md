# png_verus ledger

Milestone M1b of `../PLAN.md`: the plain Rust implementation of `png_verus`, with the guard in place, calling `inflate_verus`. Zero Verus obligations are proved here; Verus was never run (another agent owns the `inflate_verus` verifier passes; this crate never touches `../inflate_verus/target`). `inflate_verus` is a path dependency and unmodified.

## State (2026-11-19)

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

## Layout

- `src/limits.rs`: the six P1 caps, the only Rust that writes them (draft values from `../limits-draft/limits.py`, held out of Stage 1), and `caps_ok`.
- `src/policy.rs`: the 30-field record, `Profile` (`#[non_exhaustive]`: Strict, RecommendedBeta). The fifteen held-out RecommendedBeta rows are written once, in `RECOMMENDED_BETA`.
- `src/walk.rs`: P2–P8, P14.1 under a policy record; a fixed-size `State`, no chunk or frame list; returns `Doc` (IHDR, byte ranges of PLTE/tRNS/iCCP, offset of the first IDAT, acTL, `Info`) or `WalkFail::{Format, Fallback}` (R4).
- `src/reader.rs`: `IdatReader`, `FdatReader` (`inflate_verus::Segments` over re-walked chunks, framing only, no CRC re-check except to recognise a frame-ending fcTL), `next_fctl`.
- `src/image.rs`: `R(w,h)`, Adam7 passes, inflate into `Raw`, in-place unfiltering, P13 conversion, `Sink` (RGBA8, RGBA16 BE, APNG canvas with P14.4 OVER).
- `src/png.rs`: `Error` (62 names, inside `verus!`), `Info`/`Meta`, entry points, `Frames`, ICC, the runtime request `Trace`.
- `src/alloc_guard/mod.rs`: byte-identical to every core (`add634bc…`); `sites.rs`: PNG's sites `Raw`, `Output`, `Saved`, `IccWindow`, `Icc`, `TryError`, `log_tied`, `try_empty` (D8's zero-element request). The plain code passes `Tracked::assume_new()` logs, as GIF's M1 did; M3–M6 replace them with the sealed log.
- `tests/conformance.rs` (810 records), `tests/fault_injection.rs`, `examples/png_probe.rs` (differential adapter), `tools/probe_diff.py`, `tools/lint_caps.sh`, `tools/audit_alloc.sh` (reuses `../../gif/tools/alloc_scan.py` in place).

## Walk first (L-WALK-FIRST, council obligation for M3; where M1b enforces it)

Every entry point completes the walk before its first reservation or inflate call, and no inflate call exists in `walk.rs`:

- Stills: `png.rs` `still_impl` calls `still_walk` (line 376) before `reserve_image` (378) and `IdatReader::new` (379); `decode_image` (`image.rs:340`) is the only `inflate_zlib_into` call for image data.
- Frames: `Frames::try_new` walks (499; on `Fallback` re-walks with APNG off and sets R4) before `reserve_image` (514) and the default image's decode (515). fdAT data is inflated only in `try_next` (`FdatReader::new`, 622), only for a fcTL found by `next_fctl` (563), only after `try_new` succeeded, and never in the R4 fallback (`FState::Single`, no fcTL is visited). Under Strict every fcTL passed its checks in the walk. Under RecommendedBeta a fcTL with a bad CRC is not a frame boundary, so the fdAT after it is read as part of the preceding frame, exactly as `policy_oracle.py` decodes it; whether L-WALK-FIRST's wording counts that as "a frame whose fcTL failed its checks" is `QUESTIONS.md` Q2.
- ICC: `icc_impl` calls `still_walk` (657) before `IccWindow` (666).

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

- ImageCrateCompat and GdkCompat (P2); `chunks()` and `try_icc_profile_with_policy` (no model yet, `QUESTIONS.md` Q4).
- The leniency harness's other runners were not rebuilt (`../gif/build.sh` clones and builds glycin/image under `/tmp/claude-0`, which a restart cleared); the `verified` runner was checked against the Strict model, not re-voted against the incumbents. `~/projects/leniency` is not a git repository; the public copy `verified-decoders/leniency/` has no PNG adapter and was not edited (syncing is the coordinator's call, `SYNC-TO-REPO.md`).

## Resume point

M1b is complete; the next milestone is M3 (chunk walk proofs, `../PLAN.md`). To resume or re-check: run the command block above from a fresh shell (all scratch inputs regenerate under `/tmp/claude-0`). Open decisions for the coordinator: `QUESTIONS.md` Q1 (IgnoreTail short-output name), Q2 (CRC-dropped fcTL under RecommendedBeta vs L-WALK-FIRST), Q3 (two compatibility records). Nothing is committed.
