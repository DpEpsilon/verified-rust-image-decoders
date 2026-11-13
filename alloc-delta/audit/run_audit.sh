#!/bin/bash
# Production allocation audit of the four verified decoder cores
# (alloc-delta SPEC.md §6, §8, §9; ERRATA.md E7; README.md in this directory).
#
# Builds alloc-delta/audit/tool (a rustc_driver program; RUSTC_BOOTSTRAP=1 on
# the pinned stable 1.99.1, as the F1 prototype did), checks the allowlist pins,
# then builds each core in the release profile on x86_64-unknown-linux-gnu with
# the tool as RUSTC_WRAPPER. The tool audits only the crates named in the run's
# plan (the core, and for ICO also the dib_verus it links) and passes every
# other compiler invocation through to the real rustc.
#
# Usage: run_audit.sh [--diagnostic] [--out DIR] [core ...]
#   core          dib | tga | icns | ico (default: all four)
#   --diagnostic  audit without allowlists (prints the reachable set for
#                 drafting an allowlist); always exits nonzero
#   --out DIR     run directory (default: runs/<UTC timestamp>[-diagnostic])
# Environment:
#   ALLOC_AUDIT_TARGET  scratch dir for build output (default
#                       /tmp/claude-0/alloc-audit; regenerable)
#   ALLOC_AUDIT_TIMEOUT per-core build timeout in seconds (default 1800)
#
# Exit status: 0 only if every requested core's report says `verdict: PASS`.
# A missing report, a failed build, a pin mismatch or any finding is nonzero.
set -u
here="$(cd "$(dirname "$0")" && pwd)"
repo="$(cd "$here/../.." && pwd)"
export PATH=/opt/cargo/bin:$PATH
tc=/opt/rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu
export RUSTUP_TOOLCHAIN=1.99.1-x86_64-unknown-linux-gnu
SR="$("$tc/bin/rustc" --print sysroot)"
export LD_LIBRARY_PATH="$SR/lib/rustlib/x86_64-unknown-linux-gnu/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
scratch="${ALLOC_AUDIT_TARGET:-/tmp/claude-0/alloc-audit}"
tmo="${ALLOC_AUDIT_TIMEOUT:-1800}"
target_triple=x86_64-unknown-linux-gnu
aldir="$here/allowlist/$target_triple/release"

# Variables that change what the tool audits or how it judges are refused, not
# inherited: a production run sets its own and accepts none from the caller.
refused=(ALLOC_AUDIT_PROTOTYPE ALLOC_AUDIT_ROOTS ALLOC_AUDIT_VSTD_VERSION ALLOC_AUDIT_ALLOWLIST
         ALLOC_AUDIT_REPORT ALLOC_AUDIT_PLAN ALLOC_AUDIT_CONTINUE ALLOC_AUDIT_EDGES ALLOC_AUDIT_RUN_DIR
         RUSTC_WRAPPER RUSTC_WORKSPACE_WRAPPER RUSTFLAGS CARGO_ENCODED_RUSTFLAGS CARGO_BUILD_RUSTFLAGS
         CARGO_BUILD_RUSTC_WRAPPER CARGO_BUILD_RUSTC RUSTC)
for v in "${refused[@]}"; do
  if [ -n "${!v+x}" ]; then
    echo "run_audit: refusing to run with $v set in the environment (value ${!v@Q}); unset it" >&2
    exit 2
  fi
done
unset_args=(); for v in "${refused[@]}"; do unset_args+=(-u "$v"); done

diagnostic=0
out=""
cores=()
while [ $# -gt 0 ]; do
  case "$1" in
    --diagnostic) diagnostic=1 ;;
    --out) out="$2"; shift ;;
    dib|tga|icns|ico) cores+=("$1") ;;
    *) echo "run_audit: unknown argument $1" >&2; exit 2 ;;
  esac
  shift
done
[ ${#cores[@]} -eq 0 ] && cores=(dib tga icns ico)
stamp="$(date -u +%Y%m%dT%H%M%SZ)"
[ -z "$out" ] && out="$here/runs/$stamp$([ $diagnostic = 1 ] && echo -diagnostic)"
mkdir -p "$out" "$scratch"
out="$(cd "$out" && pwd)"
summary="$out/SUMMARY.txt"
: > "$summary"
log() { echo "$*" | tee -a "$summary"; }

log "alloc-delta production allocation audit, run $stamp"
log "mode: $([ $diagnostic = 1 ] && echo 'DIAGNOSTIC (no allowlists; never a pass)' || echo PRODUCTION)"
log "repo: $repo"
log "rustc: $("$tc/bin/rustc" -vV | tr '\n' ' ')"
log "cargo: $(cargo -V)"

# 1. Build the tool.
if ! timeout 1800 env RUSTC_BOOTSTRAP=1 CARGO_TARGET_DIR="$scratch/tool-target" \
     cargo build --offline -j1 --release --manifest-path "$here/tool/Cargo.toml" \
     >"$out/tool-build.log" 2>&1; then
  log "FAIL: the audit tool did not build (see tool-build.log)"; exit 2
fi
tool="$scratch/tool-target/release/alloc_audit"
log "tool: $tool sha256=$(sha256sum "$tool" | cut -d' ' -f1)"
log "tool sources: $(cat "$here"/tool/src/*.rs "$here"/tool/Cargo.toml "$here"/tool/Cargo.lock | sha256sum | cut -d' ' -f1) (sha256 of src/*.rs + Cargo.toml + Cargo.lock, concatenated)"

# 2. Allowlist pins (the allowlists are themselves hash-pinned).
status=0
if [ $diagnostic = 0 ]; then
  if (cd "$here/allowlist" && sha256sum -c --strict PINS.sha256) >"$out/pins.txt" 2>&1; then
    log "allowlist pins: OK ($(wc -l < "$here/allowlist/PINS.sha256") files)"
  else
    log "FAIL: allowlist pins do not match (see pins.txt); an allowlist changed without re-review"
    status=1
  fi
fi

declare -A crate_of=([dib]=dib_verus [tga]=tga_verus [icns]=icns_verus [ico]=ico_verus)
al() { if [ $diagnostic = 1 ]; then echo -; else echo "$aldir/$1.toml"; fi; }

for core in "${cores[@]}"; do
  crate="${crate_of[$core]}"
  dir="$repo/$core/$crate"
  T="$scratch/cores/$core"
  plan="$out/$core.plan"
  echo "$crate $(al "$crate") $out/$crate.txt" > "$plan"
  if [ "$core" = ico ]; then
    echo "dib_verus $(al dib_verus) $out/dib_verus-in-ico.txt" >> "$plan"
  fi
  rm -f "$out/$crate.txt" "$out/dib_verus-in-ico.txt.$core"
  # Force the audited crates to be recompiled under the wrapper.
  (cd "$dir" && cargo clean --offline -p "$crate" --release --target "$target_triple" --target-dir "$T" >/dev/null 2>&1)
  [ "$core" = ico ] && (cd "$dir" && cargo clean --offline -p dib_verus --release --target "$target_triple" --target-dir "$T" >/dev/null 2>&1)
  (cd "$dir" && timeout "$tmo" env "${unset_args[@]}" \
      ALLOC_AUDIT_PLAN="$plan" ALLOC_AUDIT_CONTINUE=1 ALLOC_AUDIT_RUN_DIR="$out" ALLOC_AUDIT_EDGES=1 \
      CARGO_TARGET_DIR="$T" RUSTC_WRAPPER="$tool" \
      cargo build --offline -j1 --release --lib --target "$target_triple" --message-format=json \
      >"$out/$core.cargo.json" 2>"$out/$core.cargo.stderr")
  rc=$?
  if [ $rc -ne 0 ]; then
    log "$core: FAIL (cargo exit $rc; see $core.cargo.stderr)"; status=1; continue
  fi
  for rep in "$out/$crate.txt" $([ "$core" = ico ] && echo "$out/dib_verus-in-ico.txt"); do
    if [ ! -f "$rep" ]; then
      log "$core: FAIL (no report $rep: the crate was not audited)"; status=1; continue
    fi
    v=$(sed -n 's/^verdict: //p' "$rep" | head -1)
    n=$(sed -n 's/^findings: //p' "$rep" | head -1)
    nr=$(sed -n 's/^findings-other-than-unsigned-reviewer: //p' "$rep" | head -1)
    tags=$(sed -n 's/^finding-tags: //p' "$rep" | head -1)
    log "$core: $(basename "$rep"): verdict ${v:-<none>}, findings ${n:-?} (other than the unsigned reviewer: ${nr:-?}), tags ${tags:-?}"
    md=$(sed -n 's/^mode: //p' "$rep" | head -1)
    if [ "$md" != PRODUCTION ] && [ $diagnostic = 0 ]; then
      log "$core: $(basename "$rep"): REFUSED: report mode is '${md:-<none>}', not PRODUCTION"; status=1
    fi
    [ "$v" = PASS ] && [ "$md" = PRODUCTION ] || status=1
  done
done
log "exit status: $status"
exit $status
