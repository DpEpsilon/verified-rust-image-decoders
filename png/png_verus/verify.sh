#!/bin/bash
# M3 verifier entry point: serial foreground run, at most 25 minutes.
set -euo pipefail
root="$(cd "$(dirname "$0")" && pwd -P)"
export PATH="/opt/cargo/bin:/opt/verus:$PATH"
export CARGO_BUILD_JOBS=1
# A path dependency must never redirect build outputs into inflate_verus.
export CARGO_TARGET_DIR="$root/target"
export CARGO_TERM_VERBOSE=true
if ! grep -Rq 'verus!' "$root/src"; then
  printf '%s\n' 'No Verus obligations: refusing a zero-proof run.' >&2
  exit 2
fi
mkdir -p /home/ws/.cache/verified-decoders
exec 9>/home/ws/.cache/verified-decoders/verifier.lock
flock -n 9 || { printf '%s\n' 'Another verifier holds the shared lock.' >&2; exit 2; }
if pgrep -f '(cargo-verus verify|rust_verify .*--crate-name)' >/dev/null; then
  printf '%s\n' 'Another Verus invocation is already running.' >&2
  exit 2
fi
exec timeout --kill-after=10s 1530 python3 -I "$root/tools/verify_m3.py" "${1:-M3 checkpoint}"
