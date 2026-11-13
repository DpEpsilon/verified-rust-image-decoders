# no_std feasibility probe for the verified decoder crates

Scope: `~/projects/verified-decoders/alloc-delta/proto/nostd_probe/` only. No core crate was edited; `icns_nostd/` is a *copy* of ICNS made for the trial. Toolchain rustc 1.99.1, Verus 0.2026.11.02.8bf68ee, pinned vstd `=0.0.0-2026-11-01-0251`. Verifier runs are in `runs/NNN.log` and `RUNLOG.tsv` (via `verify.sh`).

## Conclusion

The four cores can be made `#![no_std]` + `extern crate alloc` with vstd `default-features = false, features = ["alloc"]`. The trial on a copy of ICNS (`icns_nostd/`) needed 4 added lines in `lib.rs`, 3 `use` lines in `icns.rs`, 1 `use alloc::vec::Vec;` in `model.rs`, and a Cargo.toml edit. **Verification result of the converted ICNS copy is NOT yet known**: run 8 stopped at name resolution (17 errors, all `cannot find type Vec` in `model.rs` spec/proof signatures; plain `cargo build` had passed because ghost-only uses are erased, so the verifier, not cargo build, is the check that finds missing imports); run 9 (fix applied) was still running when this probe was handed back. Read `runs/009.log` and the last line of `RUNLOG.tsv`; baseline to compare with is 315 verified, 0 errors (ICNS RUNLOG run 208). Until that line shows 315/0, the claim 'no change to verification results' is unproven. `target/` directories under `nostd_core`, `nostd_probe`, `nostd_probe_a`, `icns_nostd` were not deleted yet (run 9 in progress); delete them after run 9 ends.

What can be certified here: the `#![no_std]` attribute (enforced under `cargo verus verify` too), a std-free dependency closure of the built rlibs, and a freestanding link (no libc, no std, no loader) on the host target. What cannot: any bare-metal target build (rust-std for `*-unknown-none`/`thumbv7m` and for i686, and rust-src, cannot be downloaded: `static.rust-lang.org` resets the connection).

## Evidence for question 1: vstd with `alloc` only

Crate `nostd_probe/` (`#![no_std]`, `extern crate alloc`, vstd alloc-only; toy fn does `Vec::new`, `try_reserve_exact`, `push`, and asserts on `out@`).

| run | result |
|---|---|
| 1 | vstd verified (1950 functions), toy: 1 error, my wrong assertion `out@.len() == n + 1` (try_reserve_exact does not change the view; the spec is right, the toy was wrong) |
| 2 | `let before = out@` in exec context: spec-mode error, toy problem; fixed with `let ghost` |
| 3 | `2 verified, 0 errors`: `Vec::try_reserve_exact` and `Vec::push` resolve against vstd's `std_specs/vec.rs` in a no_std crate |

Does cargo verus rebuild vstd? Yes, from the registry source, with the requested features:
- `runs/001.log` shows `Compiling vstd v0.0.0-2026-11-01-0251`, then `verification results:: 1950 verified` (vstd itself is verified on every fresh target dir, about 55 s; std-default vstd verifies 2159 functions in the existing TGA/ICNS/DIB logs, the difference being the std-gated modules `hash_map`, `hash_set`, `thread`, `future`, `contrib::exec_spec`).
- `target/debug/.fingerprint/vstd-*/lib-vstd.json` records `"features":"[\"alloc\"]"`; `target/debug/deps/` has `libvstd-<hash>.rmeta` + `.vir` built there, no use of `/opt/verus/libvstd.rlib`.
- `cargo tree -e normal,no-proc-macro` of the no_std crate: `vstd [alloc]` -> `verus_builtin []`. The same command in `icns/icns_verus` (default features) prints `vstd [alloc,default,std]`.

The prebuilt `/opt/verus/libvstd.rlib` is std-enabled (`RUSTC_BOOTSTRAP=1 rustc -Zls=root` lists `std`, `libc`, `miniz_oxide`, `adler2` among its dependencies), so it must not be what a certified build uses; cargo verus does not use it. Direct `verus` invocations (not cargo) would.

What the final artifact links, measured with `rustc -Zls=root` on the rlibs from `cargo build`:
- `libvstd` (alloc-only): core, alloc, compiler_builtins (+ the proc-macro/builtin crates, `MacrosOnly`/`verus_builtin`)
- `libnostd_core`: core, alloc, compiler_builtins, vstd, verus_builtin
- no `std` in either list.

A truly std-free closure requires: every crate in the graph sets `default-features = false` on vstd (cargo unifies features, so one dependent with default features re-enables `std` for everyone); no `std`-feature dependencies elsewhere. The host-side crates (`syn`, `proc-macro2`, `indexmap[std]`, `vstd`'s `build.rs`, which uses `std::env`) do use std, but they run in the compiler process and never reach the artifact; `cargo tree -e normal,no-proc-macro` excludes them, plain `-e normal` lists 31 lines including them.

Note: `cargo verus verify` emits `.rmeta`/`.vir` only, no code. The linked-artifact evidence therefore needs a separate plain `cargo build` with the same features.

## Evidence for question 2: what a no_std core looks like

Crate `nostd_core/` copies the item shapes of `tga/tga_verus/src/policy.rs` (option enums with derives and `#[non_exhaustive]`, `Default` impl, struct) plus `Vec`, `vec!`, `String`, `format!`, and a `#[cfg(test)]` module. Run 6: `9 verified, 0 errors`. `cargo build` and `cargo test` pass (4 tests).

Census of std-isms in the live sources (backup `*.rs.<tag>` files ignored):

| construct | where | no_std form |
|---|---|---|
| `impl std::default::Default for BmpProfile` | dib.rs:480 | `core::default::Default` |
| `impl std::default::Default for Profile` | tga policy.rs:200 | `core::default::Default` |
| `vec![0u8; n]` | tga.rs:8210; icns.rs (3 sites); DIB uses `Vec::new` in alloc_guard | `use alloc::vec;` (imports module and macro) |
| `Vec`, `String`, `Vec::new`, `String::new` | icns.rs, dib alloc_guard/mod.rs, ... | `use alloc::vec::Vec; use alloc::string::String;` |
| `#[cfg(test)] mod policy_tests;` using `std::{collections,io,path,process,fs}`, `format!`, `println!` | tga lib.rs:9, dib lib.rs:17 | first line of the test file: `extern crate std;` |
| `format!` in `verus!` | none in live sources | outside `verus!` only: run 4 showed vstd's `fmt_req` precondition fails; this is a Verus property, not a no_std one |
| `impl std::error::Error` | none in the four cores (only in `alt/icns-creusot`) | `core::error::Error` (stable since Rust 1.81) |
| `HashMap`/`HashSet`/`thread` | none | would need `hashbrown`/`BTreeMap`, and vstd's `hash_map` is std-gated |
| derives (`Debug, Clone, Copy, PartialEq, Eq`) | everywhere | unchanged |
| `tests/*.rs`, `examples/*.rs` | all four | separate crates with their own std; unchanged |

Not exercised: `.to_string()`/`.to_owned()`/`ToString`/`ToOwned`/`Box`/`collect` need `use alloc::{string::ToString, borrow::ToOwned, boxed::Box}`; the grep found no live use in `src/`. The ICO crate has no live `.rs` file (only `ico.rs.m*` backups, no Cargo.toml), so nothing there was checked.

Negative control (run 7, crate `nostd_probe_a`): under `cargo verus verify`, `std::vec::Vec` in a type, `impl std::default::Default`, an unimported `vec!`, and an unimported `String` are all rejected (`E0433 cannot find module or crate std`, `cannot find macro vec`, `E0425 cannot find type String`). So `#![no_std]` is enforced in verify mode, not only in plain builds.

## Evidence for question 3: certification mechanisms

(a) `#![no_std]` attribute: enforced as above. Weakness: it does not stop a dependency from linking std (`extern crate std;` can reappear anywhere, including `cfg(test)`-guarded code, which is fine, but also unguarded). Check with a grep for `extern crate std` and `use std::` outside `#[cfg(test)]` files.

(b) Dependency closure: `cargo tree -e normal,no-proc-macro -f '{p} [{f}]'` must print exactly `crate -> vstd [alloc] -> verus_builtin []`, with no `std` or `default` feature on vstd. Stronger: `rustc -Zls=root target/debug/deps/lib<crate>-*.rlib` (needs `RUSTC_BOOTSTRAP=1`) must list only core, alloc, compiler_builtins, vstd, verus_builtin and `MacrosOnly` crates. Both were run on `nostd_core`: pass; the prebuilt `/opt/verus/libvstd.rlib` fails the same check, as expected.

(c) Bare-metal build. Installed: only `x86_64-unknown-linux-gnu` (`rustc --print sysroot` = `/opt/rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu`; `rustup target list --installed` agrees; no `rust-src`; `rustc-src` is rustc-dev's compiler source, not the library source). `rustup target add x86_64-unknown-none` fails: `static.rust-lang.org` answers `Connection reset by peer` (curl and rustup; `index.crates.io` and `github.com` return 200). Exact components, from the installed channel manifest (date 2026-10-15), all host `static.rust-lang.org`:
- `https://static.rust-lang.org/dist/2026-10-15/rust-std-1.99.1-x86_64-unknown-none.tar.xz`
- `https://static.rust-lang.org/dist/2026-10-15/rust-std-1.99.1-thumbv7m-none-eabi.tar.xz`
- `https://static.rust-lang.org/dist/2026-10-15/rust-std-1.99.1-i686-unknown-linux-gnu.tar.xz`
- `https://static.rust-lang.org/dist/2026-10-15/rust-src-1.99.1.tar.xz`
The sandbox allowlist needs `static.rust-lang.org` (the coordinator proposes that via `ws.sandbox.propose`; I changed nothing). Once reachable, `rustup target add x86_64-unknown-none thumbv7m-none-eabi i686-unknown-linux-gnu` and `rustup component add rust-src` suffice. Note that `*-unknown-none` and thumbv7m ship only `core`+`alloc` (no `std`), so a successful `cargo build --target x86_64-unknown-none` on a crate with vstd alloc-only is a hard certificate that no std item is referenced; vstd itself must also build there (untested; it uses atomics and `core::intrinsics`, which thumbv7m lacks 64-bit atomics for, so expect that target to need vstd checks).

Substitute that works offline: **freestanding link test** (`freestanding/main.rs`, binary `freestanding/fs_test`), host target, `-C panic=abort -C link-arg=-nostartfiles -nostdlib -static`. It is `#![no_std] #![no_main]` with its own `_start` (raw `exit` syscall), bump `#[global_allocator]`, panic handler, `memcpy/memmove/memset/memcmp/bcmp`, and stubs for `rust_eh_personality` and `_Unwind_Resume` (landing pads in the unwind-built prebuilt core/alloc and in the debug rlibs reference them; never executed with `panic=abort` and a diverging handler). It links `nostd_core` (so vstd alloc-only) and calls `build_vec`, `hex_bytes`, `describe` (which uses `alloc::format!`). Result: links with no other undefined symbol (first attempt failed only on `_Unwind_Resume`, none on libc or std), runs, exits 36 = 16 + 3 + 17 as computed from the three return lengths; `readelf -d` says no dynamic section; `strings` finds no `libstd`/`library/std`/`libc.so`/`GLIBC`, only `library/core/` and `library/alloc/` paths. What it certifies: the closure of symbols needed by the crate and vstd is satisfiable by core+alloc+compiler_builtins plus the user's allocator and mem functions. What it does not: a non-x86_64 or non-Linux ABI, 32-bit pointers, or the target's own atomics support.

(d) alloc_audit (`../alloc_audit/`, read, not edited). It walks monomorphised MIR from roots (`drain_queue` in `src/walk.rs`, ~line 436) and keys everything by `Instance`/DefId; it already marks audited-crate nodes with `DefId::is_local()` (line 456) and fails closed on unresolved edges, so every reachable instance passes through one place. It can be extended to assert "no instance outside core/alloc/vstd (and the audited crate) is reachable":
1. In `drain_queue`, after `visited.insert`, compute `let k = self.tcx.crate_name(instance.def.def_id().krate).to_string();`.
2. If the instance is not local and `k` is not in a configured set (default `core`, `alloc`, `vstd`, `verus_builtin`, `compiler_builtins`), push `Finding::new("foreign-crate-instance", format!("{label} defined in crate {k}"))` and keep a per-crate count for the report header ("crates reached: core 83, alloc 21, vstd 7").
3. Allow an `[[entry]] kind = "crate" item = "<name>"` in the allowlist for any deliberate exception, so the check goes through the same provenance machinery.
4. Also fail on `k == "std"` explicitly, so the message distinguishes "std reachable" from "unknown crate reachable".
Limits: it covers only code *reachable from the roots* (a linked-but-unused std would not show; the `-Zls` check covers that), and it needs a compiler built for the target: the 32-bit scope is blocked by the same missing i686 rust-std/rust-src (NOTES.md there says so). A seeded negative control would be a crate calling `std::fs::read` that must yield `foreign-crate-instance`; it requires a std-linked build, so it would be run on the std variant of an existing control. Not implemented or run here.

## Recommendation (question 4)

Per core crate (DIB, TGA, ICNS, ICO), exact changes:

1. `Cargo.toml`: `vstd = { version = "=0.0.0-2026-11-01-0251", default-features = false, features = ["alloc"] }`. Keep dev-dependencies (`serde_json`) as is; integration tests and examples compile as separate std crates.
2. `src/lib.rs`: add `#![no_std]` and `extern crate alloc;` at the top.
3. Each file naming `Vec`, `String` or `vec!`: add `use alloc::vec::Vec; use alloc::string::String; use alloc::vec;` (ICNS needed these only in `icns.rs`; the compiler lists any others).
4. `impl std::default::Default` -> `impl core::default::Default` (DIB dib.rs:480, TGA policy.rs:200).
5. Each `#[cfg(test)] mod policy_tests;` file: add `extern crate std;` as its first item. Not exercised against the real files (they were not copied); exercised on the pattern in `nostd_core`.
6. Keep `format!`/`println!` out of `verus!` (already so).
7. Certification script: `cargo tree` check, `-Zls` check on the built rlib, plain `cargo build`, `cargo test`, freestanding link test; then the bare-metal build once the components exist.

Effort: about one hour per crate plus a full reverify (ICNS full reverify is 4-5 minutes; DIB and TGA longer). Risk: no change to proof obligations is expected, since all edits are imports, paths and a Cargo feature; vstd verifies a smaller set (1950 vs 2159 functions) but the crates only use items present in both. The ICNS trial result above is the evidence for that; DIB (alloc_guard uses `try_reserve_exact`, which is the toy shape verified in run 3) and TGA were not trial-converted.

Risks to watch: feature unification (any std-default vstd dependent re-enables std; the check in (b) catches it); `cargo verus` first run on a fresh target dir re-verifies vstd (~1 min); a new vstd release may move specs behind `std`.

Cannot be certified without component installs: bare-metal build of any crate; the 32-bit (i686) audit scope and the 32-bit view of the no_std claim; `-Zbuild-std` experiments (rust-src).

## Draft README paragraph

> **no_std.** `<crate>` is `#![no_std]` and depends on `core` and `alloc` only. Its sole dependency is `vstd` built with `default-features = false, features = ["alloc"]`; the build-time proc-macro crates behind it (`syn` and friends) run on the host and are not linked. Checked on every release: `cargo tree -e normal,no-proc-macro` lists only `vstd [alloc]` and `verus_builtin`; the built rlib's crate-dependency list (`rustc -Zls=root`) contains no `std`; and a freestanding x86_64 Linux binary with no libc, no C runtime and no dynamic loader links and runs. Unit tests and the integration tests use `std` in separate test crates and are not part of the claim. Not certified: a build for a bare-metal or 32-bit target (`x86_64-unknown-none`, `thumbv7m-none-eabi`, `i686`), because those standard-library components are not installed in the verification environment; and the audited property "no allocation outside the guard" applies to x86_64 only. The crates allocate through `alloc`, so a `#[global_allocator]` is required of the embedding program, and out-of-memory is reported as an error only on the guarded paths.

## Not tested / honest limits

- No real core crate was edited or built in place; only ICNS was converted, as a copy.
- DIB, TGA, ICO were not trial-converted. ICO has no live source.
- No target other than x86_64-unknown-linux-gnu was compiled; vstd on `thumbv7m` is untested.
- The alloc_audit extension is a design, not code.
- `core::error::Error`/`Display` impls: not compiled in the probe because none of the four cores has such an impl.
- `_Unwind_Resume` was stubbed in the freestanding test; a build with `-C panic=abort` for all crates would not need it (needs rebuilding vstd and the crate with that flag; not done).
