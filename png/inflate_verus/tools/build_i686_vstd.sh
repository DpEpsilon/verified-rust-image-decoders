#!/bin/bash
# Builds the pinned i686-unknown-linux-gnu vstd verification cache (.vir) once,
# in the dedicated target directory target/i686-verify. See I686-CACHE.md.
#
# The host cache (target/debug, vstd 01822e92e4f44378) was produced by a plain
# `cargo verus verify` (GIF run 1), which verifies and exports vstd with no
# forwarded Verus arguments. `--is-vstd` is refused under `--no-cheating`
# (run 19), so the same holds here: `--fwd-verus-args-to roots` gives vstd no
# forwarded arguments, exactly as on the host, and the root crate receives only
# `--no-verify`, so this run makes no proof claim about inflate_verus.
# CARGO_TARGET_DIR keeps cargo's host-side units (proc macros, build scripts)
# out of target/debug.
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd -P)"
export PATH="/opt/cargo/bin:/opt/verus:$PATH"
export CARGO_BUILD_JOBS=1
mkdir -p /home/ws/.cache/verified-decoders
exec 9>/home/ws/.cache/verified-decoders/verifier.lock
flock -n 9 || { printf '%s\n' 'Another verifier holds the shared lock.' >&2; exit 2; }
if pgrep -f '(cargo-verus verify|rust_verify .*--crate-name)' >/dev/null; then
  printf '%s\n' 'Another Verus invocation is already running.' >&2
  exit 2
fi
cd "$root"
exec python3 -I tools/run_logged.py --timeout 1500 \
  --label 'i686 cache: build pinned i686-unknown-linux-gnu vstd (.vir) in target/i686-verify; root --no-verify' -- \
  env CARGO_TARGET_DIR="$root/target/i686-verify" CARGO_BUILD_TARGET=i686-unknown-linux-gnu CARGO_TERM_VERBOSE=true \
  cargo verus verify --fwd-verus-args-to roots -- --no-verify
