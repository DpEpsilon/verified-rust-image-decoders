#!/bin/bash
# Run mutations FIRST..LAST of THEOREMS.md's protocol, one at a time.
#
# Usage: tools/run_mutations.sh [FIRST] [LAST]   (default 1 20)
#
# For each N: build mutation/mN as a disposable copy of mutation/base with
# mutation N applied, run the verifier on it through ./verify.sh (the only allowed
# way to verify), and keep the verifier output as mutation/mN/failure.log.  The
# copy is discarded and rebuilt from the pristine base before the next mutation,
# so mutations never compose.  All of T1-T11, L1-L4 and the 165 model-vs-vector
# obligations are present in every copy: a mutation copy is the whole crate.
#
# After each run the copy's regenerable `target/` cache is deleted, because the
# volume has a 500,000-inode cap.
set -u
cd "$(dirname "$0")/.." || exit 2

first=${1:-1}
last=${2:-20}

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
  python3 tools/mutations.py apply "$n" || exit 3
  out=$(VERIFY_CRATE="mutation/m$n/tga_verus" ./verify.sh "mutation $n: $what" 2>&1)
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
  # reclaim inodes: the target/ cache is regenerable
  rm -rf "mutation/m$n/tga_verus/target"
  echo
done
echo "mutations $first..$last done"
