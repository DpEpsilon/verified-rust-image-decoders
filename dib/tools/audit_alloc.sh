#!/bin/bash
# Source-level falsification check, not a typed/transitive allocation audit.
# Optional argument: production src directory (or its dib.rs, for compatibility).
set -euo pipefail
exec python3 "$(dirname "$0")/audit_alloc.py" "$@"
