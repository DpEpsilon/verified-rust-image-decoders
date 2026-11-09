#!/bin/bash
# Runs the Creusot prover on the crate and logs the run. Use this instead of calling cargo directly,
# so every prover run is counted. Usage: ./verify.sh "what I am trying to prove"
cd "$(dirname "$0")" || exit 2
export PATH=/opt/cargo/bin:$PATH
mkdir -p runs
last=$(find runs -maxdepth 1 -type f -name '[0-9]*.log' -printf '%f\n' | sort -n | tail -1)
n=$(( ${last%.log} + 1 )); log=runs/$(printf %03d "$n").log
if [ -e "$log" ]; then printf 'Refusing to overwrite existing verifier log: %s\n' "$log" >&2; exit 2; fi
start=$(date +%s)
( cd "${VERIFY_CRATE:-icns_creusot}" && cargo creusot ) > "$log" 2>&1; rc=$?
dur=$(( $(date +%s) - start ))
sumline=$(grep -E "Proved|Failed|Unproved|error" "$log" | tail -1)
printf '%s\t%s\texit=%s\t%ss\t%s\t%s\n' "$n" "$(date -Is)" "$rc" "$dur" "$1 [crate=${VERIFY_CRATE:-icns_creusot}]" "$sumline" >> RUNLOG.tsv
tail -n 60 "$log"; echo "[verify.sh] run $n exit=$rc ${dur}s log=$log"; exit $rc
