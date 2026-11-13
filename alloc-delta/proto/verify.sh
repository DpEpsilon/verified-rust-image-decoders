#!/bin/bash
# Runs the Verus verifier on a prototype crate and logs the run. Use this instead of calling cargo
# directly, so every verifier run is counted. Usage: ./verify.sh "what I am trying to prove"
# VERIFY_CRATE selects the crate (guard_proto | guard_proto_dep), default guard_proto.
# Never delete or rename a run log; the run number is the count in runs/.
cd "$(dirname "$0")" || exit 2
export PATH=/opt/cargo/bin:/opt/verus:$PATH
mkdir -p runs; n=$(( $(ls runs | wc -l) + 1 )); log=runs/$(printf %03d "$n").log
start=$(date +%s)
peakf=$(mktemp); peakrss="$PWD/../../tools/peakrss.py"
( cd "${VERIFY_CRATE:-guard_proto}" && python3 -I "$peakrss" --out "$peakf" -- cargo verus verify --offline ) > "$log" 2>&1; rc=$?
peak_sum=$(sed -n 's/^peak_rss_sum_mb=\([0-9]*\).*/\1/p' "$peakf"); peak_max=$(sed -n 's/^[^\t]*\tpeak_rss_max_proc_mb=\([^\t]*\).*/\1/p' "$peakf"); rm -f "$peakf"
dur=$(( $(date +%s) - start ))
sumline=$(grep -E "verification results" "$log" | tail -1)
printf '%s\t%s\texit=%s\t%ss\t%s\t%s\tpeak_rss_sum_mb=%s\tpeak_rss_max_proc_mb=%s\n' "$n" "$(date -Is)" "$rc" "$dur" "$1 [crate=${VERIFY_CRATE:-guard_proto}]" "$sumline" "${peak_sum:-n/a}" "${peak_max:-n/a}" >> RUNLOG.tsv
tail -n 60 "$log"; echo "[verify.sh] run $n exit=$rc ${dur}s log=$log"; exit $rc
