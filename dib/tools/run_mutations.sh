#!/bin/bash
# Runs each implementation mutation through ./verify.sh, keeping result.txt and
# failure.log per mutation, then deletes the copy's regenerable caches.
cd "$(dirname "$0")/.." || exit 2
for n in "$@"; do
  d=mutation/m$n
  VERIFY_CRATE=$d/dib_verus ./verify.sh "mutation $n" > "$d/run.out" 2>&1
  rc=$?
  log=$(grep -oE 'runs/[0-9]+\.log' "$d/run.out" | tail -1)
  cp "$log" "$d/failure.log" 2>/dev/null
  crate=$(grep -E "verification results" "$d/failure.log" | tail -1)
  {
    echo "implementation mutation $n"
    echo "crate result: ${crate:-<no crate line; compile aborted>}"
    echo "exit=$rc"
    echo "error diagnostics:"
    grep -E "^error" "$d/failure.log" | sort | uniq -c | sed 's/^/  /'
    grep -E "at src/|--> src/" "$d/failure.log" | head -12 | sed 's/^/  /'
    echo "log: $d/failure.log"
  } > "$d/result.txt"
  rm -rf "$d/dib_verus/target" "$d/dib_verus/.verus" "$d/dib_verus/.gitignore"
  echo "== m$n rc=$rc ${crate:-no-crate-line} =="
done
