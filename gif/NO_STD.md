# no_std + alloc status of gif_verus (2026-11-15, milestone M1)

`gif_verus` is `no_std + alloc` and `try_*`-only from its first line (`../alloc-delta/SPEC.md` §1–§9; PLAN.md: no retrofit). Nothing here is verified yet: M1 is the plain implementation, and no verifier run has been made (`RUNLOG.tsv` starts with the first `verify.sh` run).

- `Cargo.toml`: `vstd = { version = "=0.0.0-2026-11-01-0251", default-features = false, features = ["alloc"] }`. Dev-dependencies (tests only): `serde_json`, and the crate itself with the feature `test-records`, which exposes record-level constructors for the two `@ImageCrateCompat-0.25.10` loop-count vectors; a consumer never enables it.
- `src/lib.rs`: `#![no_std]`, `extern crate alloc;`. No `std::` path in any production module. `alloc::vec::Vec` is the only alloc type named outside the guard (`gif.rs`: the canvas, the table record and `Saved` it receives from the guard).
- `src/alloc_guard/mod.rs`: byte-identical to the other cores, SHA-256 `add634bc74ff7feed568c90d5cc8c504538b4d4b80420e7fc9228fceecc2a137` (trusted items A-STD-1/2/4 as there).
- `src/alloc_guard/sites.rs`: GIF's own; SHA-256 `ba8f9fe5719341171767e5b932e1b8db973c8db487efa668ddfd138c2784b133`, pinned in `tools/audit_alloc.sh` (the only place the pin appears). It declares `AllocSite = Canvas | LzwTable | Saved` and `log_tied`; all three GIF reservations are byte-counted `try_zeroed` calls of the shared guard (G18, D4): `LzwTable` is one `try_zeroed(LzwTable, 20_480)` region, not a `Vec<LzwTable>`.
- The decoder calls the guard outside `verus!` with `Tracked::assume_new()` for the log argument (the M1 code is not verified; M3/M5 replace those calls by `*_logged` functions that thread a real `AllocLog`).

## Evidence (2026-11-15, `PATH=/opt/cargo/bin`, rustc 1.99.1)

`cargo tree --offline -e normal,no-proc-macro -f '{p} [{f}]'`:

```
gif_verus v0.1.0 []
└── vstd v0.0.0-2026-11-01-0251 [alloc]
    └── verus_builtin v0.0.0-2026-11-01-0251 []
```

`cargo build --offline --lib --release`, then `RUSTC_BOOTSTRAP=1 rustc -Zls=root target/release/libgif_verus.rlib`: external dependencies `core`, `alloc`, `compiler_builtins` (private), `vstd`, `verus_builtin`, and the two `MacrosOnly` crates; no `std`.

`cargo build --offline --lib --target T` (dev profile): `x86_64-unknown-none`, `thumbv7m-none-eabi` and `i686-unknown-linux-gnu` all finish with rc 0. With a 32-bit `usize`, the canvas size `4·W·H` is formed only after the `MAX_OUTPUT` check (at most 2^26; the input index bound 2^28 + 2^16 also fits a 32-bit `usize`) and pixel offsets are below it, so no `usize` product overflows; this is compile evidence and reasoning, not a test on a 32-bit target.

`tools/audit_alloc.sh --self-test`: OK (scan clean with and without `--allow-reserved`, reservation call sites Canvas 1, LzwTable 1, Saved 1, all three `try_zeroed`, both hashes OK, scanner self-test and negative controls OK). The production call-graph audit (`../alloc-delta/audit/run_audit.sh`) has no GIF configuration yet.

## Not certified

The same limits as the other cores' NO_STD.md: no bare-metal link or run, no verification per target, the test crates use `std`, and the source lint is a falsification gate rather than enforcement.

## Commands

```
cd gif && ./verify.sh "<note>"            # Verus run, logged to RUNLOG.tsv and runs/NNN.log
cd gif && tools/audit_alloc.sh --self-test
cd gif/gif_verus
nice -n 10 cargo test --offline --release -j1
cargo tree --offline -e normal,no-proc-macro -f '{p} [{f}]'
cargo build --offline --lib --target x86_64-unknown-none
cargo build --offline --lib --target thumbv7m-none-eabi
cargo build --offline --lib --target i686-unknown-linux-gnu
```

## Shared guard limit — 2026-11-17 (alloc-delta E16)

The owner raised `MAX_BYTES` from 67,108,864 bytes (64 MiB) to 1,073,741,824
bytes (1 GiB). The production `alloc_guard/mod.rs` SHA-256 is now
`add634bc74ff7feed568c90d5cc8c504538b4d4b80420e7fc9228fceecc2a137`, byte-identical in
DIB, TGA, ICNS, ICO and GIF. This supersedes earlier statements of the current
guard hash or guard limit in this file; dated E6 results remain historical.
This core's own caps remain unchanged. The guard raise does not authorize a
core-cap raise: that needs the core's own erratum and re-verification.
Re-verification and audit evidence is appended to `alloc-delta/ERRATA.md` E16;
DIB verification is pending a sandbox RAM increase.
