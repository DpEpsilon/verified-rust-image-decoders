#!/usr/bin/env bash
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
"$here/../ext/build.sh" > /tmp/claude-0/build-ext.log 2>&1 || { tail -30 /tmp/claude-0/build-ext.log; exit 1; }
PYTHONDONTWRITEBYTECODE=1 python3 -I "$here/harness.py" "$@"
