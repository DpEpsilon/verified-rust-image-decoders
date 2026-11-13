#!/bin/bash
# Runs each ICO implementation mutation through ./verify.sh on its disposable
# copy (VERIFY_CRATE=mutation/mN/ico_verus), keeping result.txt and failure.log
# per mutation, running tools/audit_alloc.sh on the mutated source, then
# deleting the copy's regenerable caches.  Copy of ../dib/tools/run_mutations.sh.
cd "$(dirname "$0")/.." || exit 2
for n in "$@"; do
  d=mutation/m$n
  VERIFY_CRATE=$d/ico_verus ./verify.sh "mutation $n" > "$d/run.out" 2>&1
  rc=$?
  log=$(grep -oE 'runs/[0-9]+\.log' "$d/run.out" | tail -1)
  cp "$log" "$d/failure.log" 2>/dev/null
  crate=$(grep -E "verification results" "$d/failure.log" | tail -1)
  audit=$(tools/audit_alloc.sh "$d/ico_verus/src/ico.rs" 2>&1 | tr '\n' '|')
  {
    echo "implementation mutation $n"
    echo "crate result: ${crate:-<no crate line; compile aborted>}"
    echo "exit=$rc"
    echo "audit_alloc: $audit"
    echo "error diagnostics:"
    grep -E "^error" "$d/failure.log" | sort | uniq -c | sed 's/^/  /'
    echo "first diagnostics with location:"
    grep -nE "error|assertion|postcondition|precondition|invariant|decreases|--> src/" "$d/failure.log" \
      | head -14 | sed 's/^/  /'
    echo "log: $d/failure.log"
  } > "$d/result.txt"
  rm -rf "$d/ico_verus/target" "$d/ico_verus/.verus" "$d/ico_verus/.gitignore"
  echo "== m$n rc=$rc ${crate:-no-crate-line} =="
done
