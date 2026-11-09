#!/usr/bin/env bash
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
"$here/../shared/build.sh"
PYTHONDONTWRITEBYTECODE=1 python3 -I "$here/../format_harness.py" ico "$@"
