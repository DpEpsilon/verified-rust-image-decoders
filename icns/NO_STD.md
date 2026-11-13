# no_std + alloc status of icns_verus (2026-11-12)

## What changed

## ERRATA E6 update (2026-11-13): `try_zeroed` fills with `resize`

`alloc-delta/ERRATA.md` E6 changed the guard body in all four cores: `try_zeroed` now calls `try_reserve_exact(n)` and, on `Ok`, `v.resize(n, 0u8)`, instead of a `push` loop. `alloc_guard/mod.rs` is byte-identical in the four cores with SHA-256 `26e4c832d2df80b782d157e68ade31066cc82ca2774b4c714a6df0f4edaf86b7`; the pin in `tools/audit_alloc.*` was updated. The trusted list gains A-STD-4: `Vec::<u8>::resize(n, 0)` on a `Vec` with `len == 0` and `capacity() >= n` does not allocate or reallocate. It is observed behaviour of std 1.99.1 (pinned source, counting-allocator and fault-injection tests: one allocation, zero reallocations), not a documented guarantee; `Vec`'s docs say bulk insertion methods "may reallocate, even when not necessary". A reallocation inside `resize` would be infallible and would abort, so it cannot be detected after the fact. The reason is cost: for a 64 MiB buffer the `push` loop takes 47.8 ms and `resize(n, 0)` 6.6 ms (`bench/fillbench/RESULTS.md`).

Latest verifier result: run 51, **412 verified, 0 errors** (before E6: 413 (run 50)). `alloc_guard/sites.rs` is unchanged (SHA-256 `adcc09bed170a230216248e1c3898dcc1b32af392b45032352ed717044c88c41`). A-STD-2 is still used, by `try_with_room`/`Reserved` in `sites.rs` for the entry list (the `push` of each entry below `room`); it is no longer used by `try_zeroed`, which serves the three channel buffers, the RGB output and the combined output. `tests/fault_injection.rs` labels its no-`realloc` assertions A-STD-2; for the `try_zeroed` buffers the property they check is the one A-STD-4 states.

Statements elsewhere in this file that describe the guard as a `push` loop, that use A-STD-2 for `try_zeroed`, that quote the old `mod.rs` hash `018ed17a...` or that give the pre-E6 verified count describe the cores before E6 and are superseded by this section. Test and lint results recorded in this file predate E6 unless dated 2026-11-13; a post-E6 rerun of them is not recorded here.

`icns_verus` is `#![no_std]` with `extern crate alloc;`. Backups `*.nostd-1` are the green sources, `*.pre-nostd` the sources before the change.

- `Cargo.toml`: `vstd = { version = "=0.0.0-2026-11-01-0251", default-features = false, features = ["alloc"] }`
- `src/lib.rs`: `#![no_std]`, `extern crate alloc;`
- `src/icns.rs`: `use alloc::string::String; use alloc::vec; use alloc::vec::Vec;`
- `src/model.rs`: `use alloc::vec::Vec;`. The cargo build warns this import is unused because `Vec` appears there only in spec/proof signatures (erased in a normal build), but the verifier needs it (probe run 8: 17 errors without it). It is therefore a required import that a plain `cargo build` cannot validate.
- No `std::` paths existed in the ICNS sources, no cfg(test) modules, and `tests/conformance.rs` is a separate std crate (unchanged). The dev-dependency `serde_json` stays (test-only).

The alloc-delta migration (2026-11-12, `dib-policy`, WIP snapshots `icns/alloc-wip-1/` and `icns/alloc-notes/`) adds `src/alloc_guard/{mod.rs,sites.rs}` and `tests/fault_injection.rs`. Nothing changed about the `no_std` boundary: the guard uses `alloc::vec::Vec` with the explicit `use alloc::vec::Vec;`, `tests/fault_injection.rs` is a std test crate like `tests/conformance.rs`, and no production source gained a `std::` path. `tools/audit_alloc.sh` and `tools/alloc_scan.py` now exist.

No change to any verified item, spec or proof; nothing beyond imports and the Cargo feature.

## Verification

Baseline (RUNLOG run 208): 315 verified, 0 errors. Converted live crate: **315 verified, 0 errors** (`runs/001.log`, RUNLOG line 1 of 2026-11-12, 327 s, fresh target dir so vstd was rebuilt alloc-only). The probe copy `alloc-delta/proto/nostd_probe/icns_nostd` gave the same 315/0 (probe run 9). Note on numbering: `icns/runs/` did not exist, so `verify.sh` numbered the log 001 while RUNLOG.tsv continues from 208; the RUNLOG row carries the 2026-11-12 timestamp.

After the alloc-delta migration: **406 verified, 0 errors** (`runs/041.log`). `cargo test --offline`: `tests/conformance.rs` 111/111 frozen vectors, `tests/fault_injection.rs` 8 passed. The source lint `tools/audit_alloc.sh` (a falsification gate, not enforcement) exits 0. The obligations added over the 315 baseline are the A-ICNS-1..6 shape lemmas, the guard call sites and the `try_*` entry points; none of the 315 was removed.

## Certification evidence (in `icns_verus/`, PATH=/opt/cargo/bin:/opt/verus)

(a) `cargo tree --offline -e normal,no-proc-macro -f '{p} [{f}]'` (identical with `--no-default-features`):

```
icns_verus v0.1.0 []
└── vstd v0.0.0-2026-11-01-0251 [alloc]
    └── verus_builtin v0.0.0-2026-11-01-0251 []
```

No `std` feature in the linked closure. The host-side proc-macro closure (`verus_builtin_macros`, `verus_state_machines_macros`, `syn`, `proc-macro2`, `quote`, `verus_syn`, `verus_prettyplease`, `synstructure`, `convert_case`, `unicode-ident`, `indexmap [default,std]`, `equivalent`, `hashbrown`) appears only under plain `-e normal`; it runs in the compiler and is `MacrosOnly` in the rlib.

(b) `cargo build --offline --lib --release` then `RUSTC_BOOTSTRAP=1 rustc -Zls=root target/release/libicns_verus.rlib`: dependencies core, alloc, compiler_builtins (private), vstd, verus_builtin, plus the two `MacrosOnly` crates. No `std`.

(c) `cargo build --offline --lib --target T` (dev profile), vstd compiled alloc-only each time; `-Zls=root` on each rlib lists the same seven dependencies, no `std`:

| target | result |
|---|---|
| `x86_64-unknown-none` | Finished, rc=0 (no `std` exists for this target) |
| `thumbv7m-none-eabi` | Finished, rc=0; no 64-bit-atomics or other failure in vstd or the crate |
| `i686-unknown-linux-gnu` | Finished, rc=0 (32-bit pointer width) |

## What is and is not certified

Certified: the crate and its linked dependency closure use only `core` and `alloc`; the lib compiles for `x86_64-unknown-none`, `thumbv7m-none-eabi` and `i686-unknown-linux-gnu`; verification (315/0) is unchanged.

Not certified:
- Linking or running on bare metal: only the lib rlib was built; no binary, global allocator or panic handler. A consumer must provide a `#[global_allocator]` and a panic handler; the crate allocates through `alloc`, and out-of-memory is reported as an error only where the code uses fallible reservation. The source lint is a falsification gate; it lists allocating constructs in exec code and fails on the ones it recognises, so it cannot establish that no allocation occurs outside the guard. The production call-graph audit (SPEC.md §6) does not exist for ICNS.
- Verification per target: the verifier ran once, on x86_64 Linux. The i686 and thumbv7m builds are compile evidence only; behaviour with a 32-bit `usize` (for example the 64 MiB input cap and `u64`/`usize` conversions) is not re-proved.
- Release-profile bare-metal builds and `-C panic=abort`.
- The integration test crate (`tests/conformance.rs`) uses `std` and is not part of the claim.
- The vstd used is the registry source with `features=["alloc"]`, rebuilt by `cargo verus`; the prebuilt `/opt/verus/libvstd.rlib` is std-enabled and would be used by a direct `verus` invocation.

The freestanding host-link technique from nostd_probe was not rerun; the bare-metal builds supersede it.

## Commands

```
cd icns && ./verify.sh "<note>"
cd icns/icns_verus
cargo test --offline
cargo tree --offline -e normal,no-proc-macro -f '{p} [{f}]'
cargo build --offline --lib --release
RUSTC_BOOTSTRAP=1 rustc -Zls=root target/release/libicns_verus.rlib
cargo build --offline --lib --target x86_64-unknown-none
cargo build --offline --lib --target thumbv7m-none-eabi
cargo build --offline --lib --target i686-unknown-linux-gnu
```
