#!/usr/bin/env bash
# One command: build all runners (shared with ../gif) under /tmp/claude-0, then generate, compare and render.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
"$here/../gif/build.sh"
PYTHONDONTWRITEBYTECODE=1 python3 -I "$here/png_adapter.py" "$@"
