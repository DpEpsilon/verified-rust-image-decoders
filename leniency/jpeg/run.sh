#!/usr/bin/env bash
# Build all runners under /tmp/claude-0, then generate, compare and summarise JPEG inputs.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
"$here/build.sh"
PYTHONDONTWRITEBYTECODE=1 python3 -I "$here/harness.py" "$@"
