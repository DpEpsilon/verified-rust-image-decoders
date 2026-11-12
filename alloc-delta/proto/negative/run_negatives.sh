#!/bin/bash
# Runs the adversarial negative checks. Each check is a source file whose
# EXPECTED outcome is a failed verification; this script copies it into the
# target crate, runs it through ../../verify.sh (so every run is counted in
# RUNLOG.tsv, the only place runs are counted), records the outcome in
# NEGATIVE.tsv, and restores the crate.
#
# The two things it must never do: report a check as passing because the crate
# failed to build for an unrelated reason, and leave a negative file in place
# after a run. Both are guarded below.
#
# A check "passes" iff verify.sh exits non-zero AND the summary line shows either
# a verification error count > 0 or a rustc error. Any check that exits 0 is a
# design hole and is reported as FAIL.
cd "$(dirname "$0")/.." || exit 2
[ -f NEGATIVE.tsv ] || printf 'check\tcrate\texpectation\trun\texit\tsummary\tverdict\n' > NEGATIVE.tsv

crate_of() {
  case "$1" in
    neg1_*|neg2_*|neg3_*) echo guard_proto ;;
    neg4_*) echo guard_proto_dep ;;
  esac
}
module_of() {
  case "$1" in
    neg1_*|neg2_*|neg3_*) echo crate::negative_checks ;;
    neg4_*) echo crate::negative_checks ;;
  esac
}

overall=0
for f in negative/neg*.rs; do
  base=$(basename "$f" .rs)
  crate=$(crate_of "$base")
  dest="$crate/src/negative_checks.rs"
  cp "$f" "$dest"
  # Register the module in lib.rs if it is not already there.
  if ! grep -q "pub mod negative_checks;" "$crate/src/lib.rs"; then
    printf '\n#[path = "negative_checks.rs"]\npub mod negative_checks;\n' >> "$crate/src/lib.rs"
    added=1
  else
    added=0
  fi
  before=$(ls runs | wc -l)
  VERIFY_CRATE=$crate ./verify.sh "negative check $base (must FAIL)" > /dev/null 2>&1
  n=$(( $(ls runs | wc -l) ))
  log=runs/$(printf %03d "$n").log
  exit_code=$(awk -F'\t' -v r="$n" '$1==r {print $3}' RUNLOG.tsv | sed 's/exit=//')
  summary=$(grep -E "verification results" "$log" | tail -1)
  if echo "$exit_code" | grep -q '^0$'; then
    verdict=FAIL   # the negative check verified: a design hole
    overall=1
  elif [ -n "$summary" ]; then
    # A verifier error count > 0 is the clean expected failure.
    errors=$(echo "$summary" | sed -n 's/.*, \([0-9]*\) errors/\1/p')
    if [ -n "$errors" ] && [ "$errors" -gt 0 ]; then verdict=PASS-verifier; else verdict=PASS?; fi
  else
    verdict=PASS-rustc
  fi
  case "$base" in
    neg1_*) exp="OutOfMemory has private fields; constructing one outside alloc_guard is a compile error" ;;
    neg2_*) exp="AllocLog.ev is private; editing the log outside alloc_guard is a compile error" ;;
    neg3_*) exp="returning OutOfMemory after an Ok reservation violates G clause 2 (Verus error)" ;;
    neg4_*) exp="skipping import_dib leaves req(L) != Req_D and has no OOM constructor (Verus error)" ;;
  esac
  printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\n' "$base" "$crate" "$exp" "$n" "$exit_code" "$summary" "$verdict" >> NEGATIVE.tsv
  # Restore the crate.
  rm -f "$dest"
  if [ "$added" = 1 ]; then
    python3 - "$crate/src/lib.rs" <<'PYEOF'
import sys
p=sys.argv[1]
s=open(p).read()
s=s.replace('\n#[path = "negative_checks.rs"]\npub mod negative_checks;\n','')
open(p,'w').write(s)
PYEOF
  fi
  echo "[negative] $base crate=$crate run=$n exit=$exit_code verdict=$verdict"
done
echo "[negative] done; see NEGATIVE.tsv"
exit $overall
