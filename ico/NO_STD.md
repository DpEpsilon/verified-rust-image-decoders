# ICO no_std evidence

The live `ico_verus` 0.2.0 library builds without a `std` dependency in the
configurations below. It still requires `alloc`. Host verification run **172**
completed with **347 verified, 0 errors** on 2026-11-12 (the alloc-delta
delivery's 343 plus four: the two strengthened PreAlloc corollaries and their two
public-contract bridges; the `name`/`name_spec` postconditions add no new
function). Cross-target compilation
is separate evidence; no target-side verifier or executable was run.

## Configuration

`ico_verus/src/lib.rs` declares `#![no_std]` and `extern crate alloc`. Production
modules import `alloc::vec::Vec`; the guard's `sites.rs` imports
`alloc::vec::Vec` for its `Reserved<T>`. The crate enables `std` only under
`cfg(test)`, for the host test harness, filesystem access, and the
fault-injecting allocator.

The dependency is pinned to:

```toml
vstd = { version = "=0.0.0-2026-11-01-0251", default-features = false, features = ["alloc"] }
dib_verus = { path = "../../dib/dib_verus" }
```

`cargo tree --offline -e normal,no-proc-macro -f '{p} [{f}]'` reported:

```text
ico_verus v0.2.0 (/…/ico/ico_verus) []
├── dib_verus v0.4.0 (/…/dib/dib_verus) []
│   └── vstd v0.0.0-2026-11-01-0251 [alloc]
│       └── verus_builtin v0.0.0-2026-11-01-0251 []
└── vstd v0.0.0-2026-11-01-0251 [alloc] (*)
```

The native release rlib's metadata, inspected with `RUSTC_BOOTSTRAP=1 rustc
-Zls=root`, listed `core`, `alloc`, `compiler_builtins`, `vstd`, `verus_builtin`,
`dib_verus`, and `verus_builtin_macros` and `verus_state_machines_macros` as
`MacrosOnly` dependencies. It did not list `std`. The rlib supplies no global
allocator, allocation-error handler, or panic handler. `RUSTC_BOOTSTRAP` was used
only for metadata inspection, not to compile or verify the decoder.

Evidence is retained in `alloc-a5-dependencies.log` (the `cargo tree` and
`-Zls=root` output). Cargo feature unification can enable `vstd/std` if another
dependency requests it; these observations apply to the dependency graph actually
built, not every downstream graph.

## Builds and verification

The host tools reported Rust 1.99.1 (`58cf8f9c3`, 2026-10-13) and Cargo 1.99.1
(`5f94df478`, 2026-08-27). From `ico_verus/`, with `/opt/cargo/bin` and
`/opt/verus` on `PATH`, all four library builds exited successfully:

```sh
cargo build --offline --lib --release
cargo build --offline --lib --target x86_64-unknown-none
cargo build --offline --lib --target thumbv7m-none-eabi
cargo build --offline --lib --target i686-unknown-linux-gnu
```

The two bare-metal builds establish that this configuration does not need a
target `std` library. The i686 build adds a 32-bit compilation check. These
commands produced libraries, not linked applications.

All verifier invocations used `ico/verify.sh`. Run 172 verified the
`try_*` implementation at 347/0 on the live tree (a clean reverify after removing
`target/`, the same tree run 168 verified); it ran while the ICNS agent's
verifier campaign was also active, because that campaign left no idle slot (see
`NOTES.md`). Earlier invocations were serial. The pre-conversion baseline was
303/0 (run 1 of the disposable copy at `~/projects/vd-baseline/`, against the
committed pre-migration DIB); the count rose because the alloc-delta step adds
the guard, `import_dib`, the `G`/`Prog` shapes and their lemmas, and the Sol
review fix adds four more obligations. The verifier ran
on the x86_64 Linux host, not on any of the three cross targets.

## Runtime checks

The complete host suite ran with:

```sh
cargo test --offline --release -- --nocapture
```

- All 217 frozen ICO vectors passed (`conformance: 217/217 vectors passed`).
- The serial fault-injection test checked all 217 vectors under eight allocation
  schedules each, at ICO's `EntryList` reservation (through `try_parse`) and at
  DIB's `Output` reservation inside an ICO decode (through `try_decode_entry`).
  It observed 217 cases, 117 allocating, largest reservation 45,056 bytes, and 44
  decode cases that returned `OutOfMemory` at `Dib(Output)`. It checks the exact
  OOM site and size, no partial output, no retry, no reallocation, equal
  uninjected results, and zero live measured bytes after drop. The 65,536 and
  67,108,864 byte thresholds are no-failure controls here: no vector reaches a
  DIB output that large.

## What this does not show

- The production call-graph allocation audit (`SPEC.md` §6) has not run on
  `ico_verus` on any target. `tools/audit_alloc.sh` is a source lint, a
  falsification gate, not that audit.
- No binary was linked or run on any bare-metal target; the builds are compile
  evidence only.
- The verifier, the layout check and the runtime tests ran on
  `x86_64-unknown-linux-gnu` only. 32-bit verification and behaviour are
  unchecked.
- A `no_std` consumer supplies a `#[global_allocator]` and a `#[panic_handler]`.
