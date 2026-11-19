# Pinned i686 vstd verification cache

Built once on 2026-11-19 (run 602) with the owner's authorisation. Location: `target/i686-verify/` (273 MB), a dedicated `CARGO_TARGET_DIR`; `target/debug` is not read or written by it.

| Artefact | Bytes | sha256 |
|---|---|---|
| `target/i686-verify/i686-unknown-linux-gnu/debug/deps/libvstd-a8222d889f5a22f0.vir` | 45,171,447 | `2f705972fa0d742172662e678e70287dd7b326107c663e904d7905f517b9343f` |
| `…/deps/libvstd-a8222d889f5a22f0.rmeta` | 6,578,330 | `797e7a20193d953b3e208458a6db50f3a0d2ef36d1b8e6b8fac28dd35566ad1e` |
| `…/deps/vstd-a8222d889f5a22f0.d` | 62,760 | `760593acdf55856a304d71a9d0c736d88b8695e59da8ed49c975713514cb0fe1` |

The `.d` and `.rmeta` embed absolute paths, so their hashes are specific to this checkout; the cargo hash `a8222d889f5a22f0` is not.

## How the host cache was made, and what is the same here

The host cache `target/debug/deps/libvstd-01822e92e4f44378.*` is a copy of GIF's (inflate run 29). GIF produced it in its run 1 (2026-11-15) with a plain `cargo verus verify`: cargo-verus compiles the registry crate `vstd 0.0.0-2026-11-01-0251` (features `alloc`, from this crate's `default-features = false, features = ["alloc"]`) through `/opt/verus/verus` with `--is-vstd`, verifies it (1,950 verified, 0 errors) and exports the `.vir`. vstd receives no forwarded Verus arguments. `--is-vstd` is refused under `--no-cheating` (run 19), so no vstd cache can be built by a `--no-cheating` pass; later `--no-cheating` passes reuse the cache because cargo does not fingerprint forwarded Verus arguments.

Run 602 repeats that build for `--target i686-unknown-linux-gnu`: same Verus 0.2026.11.02.8bf68ee, rustc 1.99.1, vstd version, feature set (`--cfg feature="alloc"` only) and `Cargo.lock`. `--fwd-verus-args-to roots` keeps vstd's forwarded arguments empty, as on the host. The root crate gets only `--no-verify`, so run 602 makes no claim about inflate_verus. vstd again reports **1,950 verified, 0 errors**. No Verus, vstd, rustc, feature, `--no-cheating`, solver-limit, proof or source change was made.

## Rebuild

```
tools/build_i686_vstd.sh
```

It takes the shared verifier lock, refuses a running Verus invocation, and runs under `tools/run_logged.py --timeout 1500`:

```
env CARGO_TARGET_DIR=$PWD/target/i686-verify CARGO_BUILD_TARGET=i686-unknown-linux-gnu CARGO_TERM_VERBOSE=true \
  cargo verus verify --fwd-verus-args-to roots -- --no-verify
```

It leaves a `--no-verify` cargo result for the feature-free i686 build. A later feature-free i686 `cargo verus verify` on unchanged sources would replay it and print no count; the split passes use `split*` features and are unaffected.

## Replaying the checkpoint on i686

`verify.sh` and the split tools are unchanged; the target comes from the environment:

```
env CARGO_BUILD_TARGET=i686-unknown-linux-gnu CARGO_TARGET_DIR=$PWD/target/i686-verify CARGO_TERM_VERBOSE=true \
  ./verify.sh 'i686-unknown-linux-gnu replay …'
```

Each pass command therefore equals the host pass command, bound to the same source tree hash. `CARGO_TERM_VERBOSE` makes each pass log show the rustc line for `inflate_verus` with `--target i686-unknown-linux-gnu` and `--extern vstd=…/i686-unknown-linux-gnu/debug/deps/libvstd-a8222d889f5a22f0.rmeta`, and `Fresh vstd`. `tools/audit_witnesses.py` checks all three for the `target-replay-count` record.

First replay, runs 603–608 (`evidence/checkpoint-0603.json`): base 372, config-cases 811, extended 677, classes-blocks 517, dynamic 452, fixed-policy 410, all 0 errors, on host tree hash `edcde0f4…6020`; unique total **1,379**, equal to host checkpoint 592.

The replay is fresh only while cargo has no cached result for a pass's features and sources in `target/i686-verify`. A second replay on an unchanged tree is a cargo replay and fails closed (no JSON report).

## Host cache check

Runs 601 (before) and 609 (after) record the sha256, size and mtime of all 2,179 files under `target/debug`; the two listings are identical. `libvstd-01822e92e4f44378.vir` stays `49934e17…a533` and `.rmeta` `6cf84fc3…e618`, equal to the GIF copy.
