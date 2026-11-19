#!/bin/bash
# M2 verifier entry point. Serial foreground passes, each at most 25 minutes, one cargo job.
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
# A checkpoint is the serial sequence of passes in tools/verify_passes.json.
# Each pass compiles base plus its feature-gated witness modules and is its own
# logged run with the unchanged 25-minute bound, two verifier threads and
# --no-cheating. The lock above stays held on descriptor 9 across all passes.
# `verify.sh --only PASS [label]` runs one pass, which is never a checkpoint.
only=()
if [ "${1:-}" = --only ]; then only=(--only "${2:?pass name}"); shift 2; fi
exec python3 -I "$root/tools/verify_split.py" "${only[@]}" --label "${1:-checkpoint}"
