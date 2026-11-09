#!/bin/bash
# Run mutations N..M of THEOREMS.md's protocol, one at a time.
#
# Usage: tools/run_mutations.sh [FIRST] [LAST]   (default 1 10)
#
# For each N: build mutation/mN as a disposable copy of mutation/base with
# mutation N applied, run the verifier on it through ./verify.sh (the only allowed
# way to verify), and keep the verifier output as mutation/mN/failure.log.  The
# copy is discarded and rebuilt from the pristine base before the next mutation,
# so mutations never compose.  All of T1-T13 and L1-L6 are present in every copy:
# a mutation copy is the whole crate.
#
# An optional third argument `fallback` applies mutation 5's mandated fallback
# instead (see THEOREMS.md item 5).
set -u
cd "$(dirname "$0")/.." || exit 2

first=${1:-1}
last=${2:-10}
mode=${3:-}

for n in $(seq "$first" "$last"); do
  what=$(python3 - "$n" <<'PY'
import importlib.util, sys
spec = importlib.util.spec_from_file_location("m", "tools/mutations.py")
m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
n = int(sys.argv[1])
print(m.MUTATIONS[n]["what"].split("(")[0].strip())
PY
)
  echo "=== mutation $n: $what ==="
  rm -rf "mutation/m$n"
  if [ "$mode" = "fallback" ]; then
    python3 tools/mutations.py apply "$n" fallback || exit 3
  else
    python3 tools/mutations.py apply "$n" || exit 3
  fi
  out=$(VERIFY_CRATE="mutation/m$n/icns_verus" ./verify.sh "mutation $n: $what" 2>&1)
  rc=$?
  log=$(printf '%s\n' "$out" | sed -n 's/.*log=\(runs\/[0-9]*\.log\).*/\1/p' | tail -1)
  if [ -n "$log" ] && [ -f "$log" ]; then
    cp "$log" "mutation/m$n/failure.log"
  else
    printf '%s\n' "$out" > "mutation/m$n/failure.log"
  fi
  {
    echo "mutation $n: exit=$rc"
    echo "verifier log: ${log:-none}"
    grep -E "verification results" "mutation/m$n/failure.log" | tail -1
  } > "mutation/m$n/result.txt"
  cat "mutation/m$n/result.txt"
  echo
done
echo "mutations $first..$last done"