#!/bin/bash
# Regenerate the DRAFT allowlists (unsigned) for the four cores from audit
# runs, then rewrite allowlist/PINS.sha256. Drafting runs go to scratch, not
# to runs/. A human must review and sign each file afterwards; regenerating
# discards that review.
#
# Usage: tools/draft_all.sh [core ...]
set -u
here="$(cd "$(dirname "$0")/.." && pwd)"
scratch="${ALLOC_AUDIT_TARGET:-/tmp/claude-0/alloc-audit}/drafting"
aldir="$here/allowlist/x86_64-unknown-linux-gnu/release"
mkdir -p "$aldir" "$scratch"
cores=("$@"); [ ${#cores[@]} -eq 0 ] && cores=(dib tga icns ico)
declare -A crate_of=([dib]=dib_verus [tga]=tga_verus [icns]=icns_verus [ico]=ico_verus)
for i in 1 2 3; do
  run="$scratch/iter$i"; rm -rf "$run"
  "$here/run_audit.sh" --out "$run" "${cores[@]}" >/dev/null 2>&1
  changed=0
  for c in "${cores[@]}"; do
    cr=${crate_of[$c]}
    new="$scratch/$cr.toml.new"
    python3 "$here/tools/draft_allowlist.py" "$run/$cr.txt" "$here/allowlist/decisions/$c.toml" "$new" 2>"$scratch/$cr.unhandled.txt"
    if ! cmp -s <(grep -v '^#' "$new") <(grep -v '^#' "$aldir/$cr.toml" 2>/dev/null); then
      cp "$new" "$aldir/$cr.toml"; changed=1
    fi
  done
  echo "draft iteration $i: changed=$changed"
  [ $changed = 0 ] && break
done
(cd "$here/allowlist" && find x86_64-unknown-linux-gnu -name '*.toml' | sort | xargs sha256sum > PINS.sha256)
echo "PINS.sha256 rewritten:"; cat "$here/allowlist/PINS.sha256"
for c in "${cores[@]}"; do cr=${crate_of[$c]}; echo "-- $cr: not admitted by any drafting rule:"; cat "$scratch/$cr.unhandled.txt"; done
