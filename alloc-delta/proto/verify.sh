#!/bin/bash
# Runs the Verus verifier on a prototype crate and logs the run. Use this instead of calling cargo
# directly, so every verifier run is counted. Usage: ./verify.sh "what I am trying to prove"
# VERIFY_CRATE selects the crate (guard_proto | guard_proto_dep), default guard_proto.
# Never delete or rename a run log; the run number is the count in runs/.
cd "$(dirname "$0")" || exit 2
export PATH=/opt/cargo/bin:/opt/verus:$PATH
mkdir -p runs; n=$(( $(ls runs | wc -l) + 1 )); log=runs/$(printf %03d "$n").log
start=$(date +%s)
( cd "${VERIFY_CRATE:-guard_proto}" && cargo verus verify --offline ) > "$log" 2>&1; rc=$?
dur=$(( $(date +%s) - start ))
sumline=$(grep -E "verification results" "$log" | tail -1)
printf '%s\t%s\texit=%s\t%ss\t%s\t%s\n' "$n" "$(date -Is)" "$rc" "$dur" "$1 [crate=${VERIFY_CRATE:-guard_proto}]" "$sumline" >> RUNLOG.tsv
tail -n 60 "$log"; echo "[verify.sh] run $n exit=$rc ${dur}s log=$log"; exit $rc
