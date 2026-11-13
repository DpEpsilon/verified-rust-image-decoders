# DIB no_std evidence

The live `dib_verus` 0.4.0 library builds without a `std` dependency in the configurations below. It still requires `alloc`. Host verification run **552** completed with **1,765 verified, 0 errors** on 2026-11-12. Cross-target compilation is separate evidence; no target-side verifier or executable was run.

## ERRATA E6 update (2026-11-13): `try_zeroed` fills with `resize`

`alloc-delta/ERRATA.md` E6 changed the guard body in all four cores: `try_zeroed` now calls `try_reserve_exact(n)` and, on `Ok`, `v.resize(n, 0u8)`, instead of a `push` loop. `alloc_guard/mod.rs` is byte-identical in the four cores with SHA-256 `26e4c832d2df80b782d157e68ade31066cc82ca2774b4c714a6df0f4edaf86b7`; the pin in `tools/audit_alloc.*` was updated. The trusted list gains A-STD-4: `Vec::<u8>::resize(n, 0)` on a `Vec` with `len == 0` and `capacity() >= n` does not allocate or reallocate. It is observed behaviour of std 1.99.1 (pinned source, counting-allocator and fault-injection tests: one allocation, zero reallocations), not a documented guarantee; `Vec`'s docs say bulk insertion methods "may reallocate, even when not necessary". A reallocation inside `resize` would be infallible and would abort, so it cannot be detected after the fact. The reason is cost: for a 64 MiB buffer the `push` loop takes 47.8 ms and `resize(n, 0)` 6.6 ms (`bench/fillbench/RESULTS.md`).

Latest verifier result: run 556, **1,764 verified, 0 errors** (before E6: 1,765 (run 552)). The live source now carries two `#[verifier::rlimit(20)]` attributes, on `parse_header` and `dib_finish` (RUNLOG runs 555-556); statements in this file that no solver-budget attribute was added describe run 552 only. `alloc_guard/sites.rs` is unchanged (SHA-256 `33465c740e10162d0d6634afd0a6033e26f3d0f523b40e375f4b1d46bcdf000a`). DIB has no other use of A-STD-2: its guard makes one byte reservation and has no element list.

Statements elsewhere in this file that describe the guard as a `push` loop, that use A-STD-2 for `try_zeroed`, that quote the old `mod.rs` hash `018ed17a...` or that give the pre-E6 verified count describe the cores before E6 and are superseded by this section. Test and lint results recorded in this file predate E6 unless dated 2026-11-13; a post-E6 rerun of them is not recorded here.

## Configuration

`dib_verus/src/lib.rs` declares `#![no_std]` and `extern crate alloc`. Production vector users import `alloc::vec::Vec`; `BmpProfile` implements `core::default::Default`. The crate enables `std` only under `cfg(test)`, for the host test harness, filesystem access, Python oracle subprocess, and fault-injecting allocator.

The dependency is pinned to:

```toml
vstd = { version = "=0.0.0-2026-11-01-0251", default-features = false, features = ["alloc"] }
```

`cargo tree --offline -e normal,no-proc-macro -f '{p} [{f}]'` reported:

```text
dib_verus v0.4.0 []
└── vstd v0.0.0-2026-11-01-0251 [alloc]
    └── verus_builtin v0.0.0-2026-11-01-0251 []
```

The native release rlib's metadata, inspected with `RUSTC_BOOTSTRAP=1 rustc -Zls=root`, listed `core`, `alloc`, `compiler_builtins`, `vstd`, and `verus_builtin`. It also listed `verus_builtin_macros` and `verus_state_machines_macros` as `MacrosOnly` dependencies. It did not list `std`. The rlib supplies no global allocator, allocation-error handler, or panic handler. `RUSTC_BOOTSTRAP` was used only for metadata inspection, not to compile or verify the decoder.

Evidence is retained in `alloc-a5-dependencies.log`. Cargo feature unification can enable `vstd/std` if another dependency requests it; these observations apply to the dependency graph actually built, not every downstream graph.

## Builds and verification

The host tools reported Rust 1.99.1 (`58cf8f9c3`, 2026-10-13) and Cargo 1.99.1 (`5f94df478`, 2026-08-27). From `dib_verus/`, with `/opt/cargo/bin` and `/opt/verus` on `PATH`, all four library builds exited successfully:

```sh
cargo build --offline --lib --release
cargo build --offline --lib --target x86_64-unknown-none
cargo build --offline --lib --target thumbv7m-none-eabi
cargo build --offline --lib --target i686-unknown-linux-gnu
```

`alloc-a5-builds.log` retains their output. The two bare-metal builds establish that this configuration does not need a target `std` library. The i686 build adds a 32-bit compilation check. These commands produced libraries, not linked applications.

All verifier invocations used `dib/verify.sh`, serially. Run 549 verified the std try_* implementation at 1,765/0. Run 550 exposed a resource-limit failure in `dib_finish` after the alloc-only dependency change. Replacing broad model unfolding with `lemma_r2r5_policy_ok` closed it structurally: run 551 reached 1,765/0. Final run 552 also reached 1,765/0 after the policy-contract cleanup and explicit all-outcome work assertion. No solver-budget attribute or proof escape was added. The verifier ran on the x86_64 Linux host, not on any of the three cross targets.

## Runtime checks

The complete host suite ran with:

```sh
cargo test --offline -- --nocapture --test-threads=1
```

`alloc-a5-tests.log` records eight passing tests across four test binaries, plus zero doc tests:

- All 302 frozen DIB 1.1 vectors passed.
- All four named profiles passed their 102 vectors each.
- BMP corpus acceptance remained 50/71 Strict, 52/71 Recommended, 58/71 GdkCompat, and 55/71 ImageCrateCompat.
- The original mixed corpus checked 84 files and accepted 58.
- All 66 frozen custom-policy vectors matched.
- `custom_policies_match_python_oracle` passed, including 26,238 public-profile versus custom-record comparisons.
- The serial allocation test checked 1,131 cases under seven measured schedules each, for 7,917 measured decoder calls.

The allocation cases comprise 302 core vectors, 408 named-profile vectors, 66 custom vectors, and all 71 BMP corpus files through the core and four named profiles. The harness materializes even the frozen over-limit input fixtures before arming injection. Each case runs an unmeasured baseline, a measured no-failure control, failure at allocation 1, non-triggering failure at allocation 2, and rejection thresholds of 1, 4,096, 65,536, and 67,108,864 bytes.

The test allocator lives in `dib_verus/tests/support/fault_alloc.rs`; `src/policy_tests.rs` includes it only in the test build. One serial test owns every injection window, and thread-local state excludes other harness threads. The allocator increments live bytes only after `System` returns a non-null pointer. Pointer records let it count deallocation without subtracting unrelated allocations. The tests check the returned OOM site and size, false recovery on OOM, no partial output or retained bytes, no retry, no reallocation, and byte-for-byte results including alpha and recovery whenever injection does not fire. They also check that dropping each result releases every measured allocation. These are host runtime observations, not a proof of the physical allocation trace.

## Allocation assurance boundary

The verified implementation exposes the logical-log contracts for A-DIB-1 through A-DIB-8: full `G` and `Prog`, exact non-OOM format/alpha/recovery relations for every policy record, Blank revalidation reachability, one bounded request, termination and all-outcome work bounds, the conditional Strict bridge, and the public ICO logged export. A-DIB-4's vector-length invariants are part of that verification; its physical no-other-allocation requirement also needs the production call-graph audit specified by the frozen allocation delta. That audit has not been performed here.

`tools/audit_alloc.sh` is a source lint. Its Python helper strips comments, literals, spec/proof functions, contracts, proof blocks and ghost declarations before checking executable allocation syntax, including `.push(`, `handle_alloc_error`, and `process::abort`. It checks the production-module inventory, exactly one executable `try_zeroed` call site, regression controls, and both guard hashes:

```text
018ed17a077916108c5d2aead35f4d493207a633d0c69cc806a8627e08cf22e6  alloc_guard/mod.rs
33465c740e10162d0d6634afd0a6033e26f3d0f523b40e375f4b1d46bcdf000a  alloc_guard/sites.rs
```

> Superseded by ERRATA E6 (2026-11-13): `alloc_guard/mod.rs` now has SHA-256 `26e4c832d2df80b782d157e68ade31066cc82ca2774b4c714a6df0f4edaf86b7` (fill by `resize`; see the E6 section of this file).

The lint passed, but it is not a typed or transitive call-graph analysis. It recognizes this source tree's Verus syntax and formatting, not every possible Rust program. The sealed ghost log is verified; its correspondence to physical reservations is tested and depends on the hash-pinned guard text. The frozen A-STD assumptions about reservation capacity and no-growth vector writes remain assumptions about the Rust allocation implementation. Neither the source lint nor the fault tests discharge that trust boundary.

No bare-metal binary was linked or executed. No target allocator or panic handler was supplied. No 32-bit or embedded target verification, target fault injection, end-to-end loader integration, or production call-graph audit was performed. A downstream executable must provide its allocator and panic behavior and review its unified dependency features. Build targets were removed after recording the evidence; source backups and all verifier logs remain under the durable project directory.
