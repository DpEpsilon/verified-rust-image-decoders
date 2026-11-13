#!/bin/bash
# Source-level falsification check, not a typed/transitive allocation audit.
# Optional argument: production src directory (or its dib.rs, for compatibility).
# Without an argument it also runs the production transitive call-graph audit
# (alloc-delta/audit/run_audit.sh dib) when that is present; with an argument
# (a mutated copy) only the source lint runs.
set -uo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
python3 "$here/audit_alloc.py" "$@"
status=$?
prod="$here/../../alloc-delta/audit/run_audit.sh"
if [ $# -eq 0 ] && [ -x "$prod" ]; then
  echo "--- production call-graph audit: alloc-delta/audit/run_audit.sh dib ---"
  if ! "$prod" dib; then echo "production audit: FAIL (see the run directory printed above)"; status=1; fi
fi
exit $status
