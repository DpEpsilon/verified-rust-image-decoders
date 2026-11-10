#!/usr/bin/env bash
# One command: build all runners under /tmp/claude-0, then generate, compare and render.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
"$here/build.sh"
PYTHONDONTWRITEBYTECODE=1 python3 -I "$here/gif_adapter.py" "$@"
