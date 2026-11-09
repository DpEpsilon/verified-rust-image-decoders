#!/bin/bash
# Runs each executable mutation through ./verify.sh, keeping result.txt and
# failure.log per mutation, then deletes the copy's regenerable caches.
cd "$(dirname "$0")/.." || exit 2
for n in "$@"; do
  d=mutation/exec-m$n
  VERIFY_CRATE=$d/tga_verus ./verify.sh "exec mutation $n" > "$d/run.out" 2>&1
  rc=$?
  log=$(grep -oE 'runs/[0-9]+\.log' "$d/run.out" | tail -1)
  cp "$log" "$d/failure.log" 2>/dev/null
  crate=$(grep -E "verification results" "$d/failure.log" | tail -1)
  {
    echo "executable mutation $n"
    echo "crate result: ${crate:-<no crate line; compile aborted>}"
    echo "exit=$rc"
    echo "error diagnostics:"
    grep -E "^error" "$d/failure.log" | sort | uniq -c | sed 's/^/  /'
    grep -E "at src/|--> src/" "$d/failure.log" | head -12 | sed 's/^/  /'
    echo "log: $d/failure.log"
  } > "$d/result.txt"
  rm -rf "$d/tga_verus/target" "$d/tga_verus/.verus" "$d/tga_verus/.gitignore"
  echo "== exec-m$n rc=$rc ${crate:-no-crate-line} =="
done
