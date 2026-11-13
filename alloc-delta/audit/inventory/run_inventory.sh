#!/bin/bash
# Capture the independent build-input inventory of each core (and run the
# x9/x10 inventory controls). Exit 0 only if every core's inventory PASSes and
# every control has its expected verdict.
# Usage: inventory/run_inventory.sh [--out DIR] [core ...]
set -u
here="$(cd "$(dirname "$0")" && pwd)"; audit="$(cd "$here/.." && pwd)"; repo="$(cd "$audit/../.." && pwd)"
out=""; cores=()
while [ $# -gt 0 ]; do case "$1" in --out) out="$2"; shift ;; *) cores+=("$1") ;; esac; shift; done
[ ${#cores[@]} -eq 0 ] && cores=(dib tga icns ico)
[ -z "$out" ] && out="$audit/runs/$(date -u +%Y%m%dT%H%M%SZ)-inventory"
mkdir -p "$out"; out="$(cd "$out" && pwd)"
status=0
AL="$audit/allowlist/x86_64-unknown-linux-gnu/release"
for c in "${cores[@]}"; do
  timeout 3600 "$here/capture.sh" "$repo/$c/${c}_verus" "${c}_verus" "$AL/${c}_verus.toml" "$out/$c" | tee -a "$out/SUMMARY.txt" || status=1
done
ctl() { # ctl <name> <expect PASS|BLOCK> <needles |-sep> <capture args...>
  local name="$1" expect="$2" needles="$3"; shift 3
  timeout 1800 "$here/capture.sh" "$@" >/dev/null 2>&1
  local inv="${4}/INVENTORY.txt" v why=""
  v=$(sed -n 's/^inventory-verdict: //p' "$inv" 2>/dev/null)
  [ "$v" = "$expect" ] || why="verdict ${v:-none}, expected $expect"
  IFS='|' read -ra arr <<< "$needles"
  for t in "${arr[@]}"; do [ -z "$t" ] || grep -qF -- "$t" "$inv" || why="$why; lacks: $t"; done
  if [ -z "$why" ]; then echo "control $name: OK ($v)"; else echo "control $name: FAILED ($why)"; status=1; fi
}
X="$audit/controls/cargo"
{
ctl x9_build_script_untracked_read BLOCK "x9-undeclared-input.txt was opened by ['build-script-build']|variable X9_SECRET" "$X/x9" x9_build_script_untracked_read - "$out/x9"
ctl x9_declared PASS "" "$X/x9" x9_build_script_untracked_read - "$out/x9-declared" "$X/x9/INVENTORY-DECLARED.txt"
ctl x10_proc_macro_untracked_read BLOCK "x10-undeclared-input.txt was opened by|variable X10_SECRET" "$X/x10/x10_lib" x10_lib - "$out/x10"
ctl x10_declared PASS "" "$X/x10/x10_lib" x10_lib - "$out/x10-declared" "$X/x10/x10_lib/INVENTORY-DECLARED.txt"
} | tee -a "$out/SUMMARY.txt"
grep -q FAILED "$out/SUMMARY.txt" && status=1
grep -q ': BLOCK' "$out/SUMMARY.txt" && status=1
# Large raw traces are regenerable; keep them compressed.
find "$out" -name strace.log -exec gzip -f {} \;
echo "inventory exit status: $status" | tee -a "$out/SUMMARY.txt"
exit $status
