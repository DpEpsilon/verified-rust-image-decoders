# no_std + alloc status of tga_verus (2026-11-12)

## What changed

## ERRATA E6 update (2026-11-13): `try_zeroed` fills with `resize`

`alloc-delta/ERRATA.md` E6 changed the guard body in all four cores: `try_zeroed` now calls `try_reserve_exact(n)` and, on `Ok`, `v.resize(n, 0u8)`, instead of a `push` loop. `alloc_guard/mod.rs` is byte-identical in the four cores with SHA-256 `26e4c832d2df80b782d157e68ade31066cc82ca2774b4c714a6df0f4edaf86b7`; the pin in `tools/audit_alloc.*` was updated. The trusted list gains A-STD-4: `Vec::<u8>::resize(n, 0)` on a `Vec` with `len == 0` and `capacity() >= n` does not allocate or reallocate. It is observed behaviour of std 1.99.1 (pinned source, counting-allocator and fault-injection tests: one allocation, zero reallocations), not a documented guarantee; `Vec`'s docs say bulk insertion methods "may reallocate, even when not necessary". A reallocation inside `resize` would be infallible and would abort, so it cannot be detected after the fact. The reason is cost: for a 64 MiB buffer the `push` loop takes 47.8 ms and `resize(n, 0)` 6.6 ms (`bench/fillbench/RESULTS.md`).

Latest verifier result: run 231, **974 verified, 0 errors** (before E6: 975 (run 230)). The live TGA source has no `rlimit` attribute. `alloc_guard/sites.rs` is unchanged (SHA-256 `58e1fb285124557080dde6a6bc01c0d6e839cd5402b804b08b240de43750c885`). TGA has no other use of A-STD-2: its guard makes one byte reservation and has no element list. `tests/fault_injection.rs` still labels its no-`realloc` assertions A-STD-2; the property they check is the one A-STD-4 states.

Statements elsewhere in this file that describe the guard as a `push` loop, that use A-STD-2 for `try_zeroed`, that quote the old `mod.rs` hash `018ed17a...` or that give the pre-E6 verified count describe the cores before E6 and are superseded by this section. Test and lint results recorded in this file predate E6 unless dated 2026-11-13; a post-E6 rerun of them is not recorded here.

`tga_verus` is `#![no_std]` with `extern crate alloc;`. Edit set (backups `*.nostd-1` are the green sources, `*.pre-nostd` the sources before the change):

- `Cargo.toml`: `vstd = { version = "=0.0.0-2026-11-01-0251", default-features = false, features = ["alloc"] }`
- `src/lib.rs`: `#![no_std]`, `extern crate alloc;`, and `#[cfg(test)] #[macro_use] extern crate std;` (the macro_use must be at the crate root; it supplies `format!`/`println!` to the unit tests)
- `src/tga.rs`: `use alloc::vec; use alloc::vec::Vec;` (the only file naming `Vec`/`vec!`; `model.rs`, `policy.rs`, `policy_model.rs`, `model_vectors.rs` need no alloc import, so no ghost-only-use surprise like ICNS's `model.rs`)
- `src/policy.rs`: `impl std::default::Default` to `impl core::default::Default`
- `src/policy_tests.rs` (cfg(test) only): `extern crate std;` and `use std::prelude::v1::*;` (the std prelude, for `Vec`, `String`, `to_owned`, which a no_std crate's own prelude lacks)
- `tests/conformance.rs` and `examples/policy_corpus.rs` are separate std crates; unchanged.

No change to any verified item, spec or proof. Allocation code untouched (`tools/audit_alloc.sh`: 1 `vec!`, OK).

## alloc-delta re-check (2026-11-12)

The fallible-allocation delta (`src/alloc_guard/`, the `try_*` API) re-ran every
check in this file (`NOTES.md`, "alloc-delta: guard + `try_*` API landed"):

- **Verification:** run 223, **974 verified, 0 errors** (up from 947; the guard,
  the `try_*` surface and the retargeted witnesses add 27 obligations). The
  verifier still runs on x86_64 Linux only.
- **The guard is no_std.** `src/alloc_guard/mod.rs` is DIB's shipped cross-core
  text with one added line, `use alloc::vec::Vec;`: a `no_std` crate's prelude
  does not re-export `Vec` (the plain `cargo build` errors first, at the guard's
  `Result<Vec<u8>, OutOfMemory>` return type), and DIB's std crate masked that
  because std's prelude supplied it. New SHA-256
  `018ed17a077916108c5d2aead35f4d493207a633d0c69cc806a8627e08cf22e6`. This is the
  erratum SPEC.md §9 must record: the byte-identical cross-core text includes
  that import, and a std core adopts it when it converts.
- **Dependency closure and symbols unchanged.** `cargo tree -e
  normal,no-proc-macro` is still `tga_verus → vstd [alloc] → verus_builtin`; the
  release rlib's `-Zls=root` still lists core, alloc, compiler_builtins (private),
  vstd, verus_builtin and the two `MacrosOnly` macro crates, no `std`.
- **Bare-metal and 32-bit builds** re-run after the delta: `x86_64-unknown-none`,
  `thumbv7m-none-eabi` and `i686-unknown-linux-gnu` all finish with 0 errors in
  the dev profile.
- The new `tests/fault_injection.rs` and the migrated `tests/conformance.rs` are
  separate std test crates, outside the lib's `no_std` boundary; its
  `unsafe impl GlobalAlloc` is a test-only allocator wrapper, not a proof escape
  in the core.

> Superseded by ERRATA E6 (2026-11-13): `alloc_guard/mod.rs` now has SHA-256 `26e4c832d2df80b782d157e68ade31066cc82ca2774b4c714a6df0f4edaf86b7` (fill by `resize`; see the E6 section of this file).

## Verification

Baseline run 205: 947 verified, 0 errors. Converted: run 206 (fresh target dir, vstd rebuilt alloc-only) and run 207 (final sources): **947 verified, 0 errors**, identical crate obligation count. Tests after the change: `cargo test --offline`: unit tests (policy_tests) 2 passed, `tests/conformance.rs` 1 passed (169 vectors), the example builds.

## Certification evidence (all run in `tga_verus/`, PATH=/opt/cargo/bin:/opt/verus)

(a) Dependency closure, `cargo tree --offline -e normal,no-proc-macro -f '{p} [{f}]'` (identical with `--no-default-features`):

```
tga_verus v0.1.0 []
└── vstd v0.0.0-2026-11-01-0251 [alloc]
    └── verus_builtin v0.0.0-2026-11-01-0251 []
```

No `std` feature anywhere in the linked closure. Plain `-e normal` additionally lists the host-side proc-macro closure (`verus_builtin_macros`, `verus_state_machines_macros`, `syn`, `proc-macro2`, `quote`, `verus_syn`, `verus_prettyplease`, `synstructure`, `convert_case`, `unicode-ident`, `indexmap [default,std]`, `equivalent`, `hashbrown`). These run inside the compiler and are not linked (`-Zls` below shows them as `MacrosOnly`).

(b) `cargo build --offline --lib --release`, then `RUSTC_BOOTSTRAP=1 rustc -Zls=root target/release/libtga_verus.rlib`: external dependencies are core, alloc, compiler_builtins (private), vstd, verus_builtin, plus two `MacrosOnly` proc-macro crates. No `std`. Same list for `libvstd-*.rlib` (core, alloc, compiler_builtins, verus_builtin, the two macro crates).

(c) Bare-metal and 32-bit builds, `cargo build --offline --lib --target T` (dev profile), all rc=0, vstd compiled alloc-only for each target; `-Zls=root` on each target's rlib shows core, alloc, compiler_builtins, vstd, verus_builtin, MacrosOnly crates, no `std`:

| target | result |
|---|---|
| `x86_64-unknown-none` | Finished, 0 errors (this target has no `std` to link at all) |
| `thumbv7m-none-eabi` | Finished, 0 errors; vstd and the crate compile there (no 64-bit atomics or other failure hit) |
| `i686-unknown-linux-gnu` | Finished, 0 errors (32-bit pointer width) |

## What is and is not certified

Certified: the crate and its linked dependency closure use only `core` and `alloc` (build-time proof from the target list: `-Zls` on the rlibs, success on `x86_64-unknown-none`/`thumbv7m-none-eabi` where `std` does not exist), and the lib compiles for those targets and for i686. Verification (947/0) is unchanged by the conversion.

Not certified:
- Linking or running on bare metal. Only the lib rlib was built; no binary, linker script, `#[global_allocator]` or panic handler was provided. A consumer must provide a global allocator (the crate allocates the 4N-byte output buffer through `alloc`) and a panic handler.
- Verification per target. The verifier ran once, on x86_64 Linux; the 32-bit and 16/32-bit-microcontroller builds are compile evidence only. The `tools/audit_alloc.sh` source audit and the output-size cap (`4*N <= 67,108,864`) were reasoned about for 64-bit `usize`; their behaviour with a 32-bit `usize` is not re-proved here.
- Release-profile bare-metal builds and `-C panic=abort` were not exercised (dev profile only for the bare-metal targets).
- Unit tests, `tests/conformance.rs` and the examples use `std` in separate test crates and are not part of the claim.
- The vstd the crate is verified against is the registry source with `features=["alloc"]` (cargo verus rebuilds it), not the std-enabled prebuilt `/opt/verus/libvstd.rlib`; a direct `verus` invocation would use the prebuilt one.

Freestanding host-link technique (nostd_probe): not rerun; the bare-metal builds above supersede it for the "no std/libc" question.

## Commands

```
cd tga && ./verify.sh "<note>"                      # runs 206, 207
cd tga/tga_verus
cargo test --offline
cargo tree --offline -e normal,no-proc-macro -f '{p} [{f}]'
cargo build --offline --lib --release
RUSTC_BOOTSTRAP=1 rustc -Zls=root target/release/libtga_verus.rlib
cargo build --offline --lib --target x86_64-unknown-none
cargo build --offline --lib --target thumbv7m-none-eabi
cargo build --offline --lib --target i686-unknown-linux-gnu
```
