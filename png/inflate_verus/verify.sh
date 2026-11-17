#!/bin/bash
# M2 verifier entry point. One foreground run, at most 25 minutes, one cargo job.
set -euo pipefail
root="$(cd "$(dirname "$0")" && pwd -P)"
export PATH="/opt/cargo/bin:/opt/verus:$PATH"
export CARGO_BUILD_JOBS=1
# M1a contains only plain Rust: refusing a run avoids a misleading zero-proof
# green result. Add the pinned no-alloc vstd dependency before starting M2.
if ! grep -Rq 'verus!' "$root/src"; then
  printf '%s\n' 'M1a plain Rust has no Verus obligations; start M2 before running verify.sh.' >&2
  exit 2
fi
mkdir -p /home/ws/.cache/verified-decoders
exec 9>/home/ws/.cache/verified-decoders/verifier.lock
flock -n 9 || { printf '%s\n' 'Another verifier holds the shared lock.' >&2; exit 2; }
# Existing cores predate the lock. Refuse an already running legacy invocation.
if pgrep -f '(cargo-verus verify|rust_verify .*--crate-name)' >/dev/null; then
  printf '%s\n' 'Another Verus invocation is already running.' >&2
  exit 2
fi
exec python3 -I "$root/tools/run_logged.py" --timeout 1500 --label "Verus: ${1:-checkpoint}" -- \
  cargo verus verify -- --num-threads 2 --no-cheating
