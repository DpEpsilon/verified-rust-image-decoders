#!/usr/bin/env bash
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
"$here/build.sh"
PYTHONDONTWRITEBYTECODE=1 python3 -I "$here/harness.py" "$@"
