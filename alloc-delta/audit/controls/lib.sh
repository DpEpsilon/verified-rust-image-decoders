# Shared setup for the controls (sourced by run_controls.sh and make_fixtures.sh).
set -u
ctl="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
audit="$(cd "$ctl/.." && pwd)"
export PATH=/opt/cargo/bin:$PATH
tc=/opt/rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu
RUSTC="$tc/bin/rustc"
SR="$("$RUSTC" --print sysroot)"
export LD_LIBRARY_PATH="$SR/lib/rustlib/x86_64-unknown-linux-gnu/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
scratch="${ALLOC_AUDIT_TARGET:-/tmp/claude-0/alloc-audit}"
deps="$scratch/control-deps"
mkdir -p "$deps"
TC="$ctl/testcases"

build_tool() {
  timeout 1800 env RUSTC_BOOTSTRAP=1 CARGO_TARGET_DIR="$scratch/tool-target" \
    cargo build --offline -j1 --release --manifest-path "$audit/tool/Cargo.toml" >"$scratch/tool-build.log" 2>&1 || {
      echo "tool build failed; see $scratch/tool-build.log" >&2; return 1; }
  TOOL="$scratch/tool-target/release/alloc_audit"
}

# Dependency crates the controls link. Built with the plain pinned rustc.
build_deps() {
  local r=0
  "$RUSTC" --crate-name pmtoy --edition 2021 --crate-type proc-macro -Copt-level=2 "$TC/x/pmtoy.rs" -o "$deps/libpmtoy.so" || r=1
  "$RUSTC" --crate-name verus_evil --edition 2021 --crate-type proc-macro -Copt-level=2 "$TC/x/pmtoy.rs" -o "$deps/libverus_evil.so" || r=1
  "$RUSTC" --crate-name guard --edition 2021 --crate-type rlib -Copt-level=2 "$TC/x/x1_guard_dep.rs" -o "$deps/libguard.rlib" || r=1
  "$RUSTC" --crate-name x2_incmac --edition 2021 --crate-type rlib -Copt-level=2 "$TC/x/x2_incmac.rs" -o "$deps/libx2_incmac.rlib" || r=1
  "$RUSTC" --crate-name helperdep --edition 2021 --crate-type rlib -Copt-level=2 -C metadata=one "$TC/x/x12_helperdep.rs" -o "$deps/libhelperdep.rlib" || r=1
  "$RUSTC" --crate-name helperdep --edition 2021 --crate-type rlib -Copt-level=2 -C metadata=two "$TC/x/x12_helperdep.rs" -o "$deps/libhelperdep2.rlib" || r=1
  "$RUSTC" --crate-name dyldep --edition 2021 --crate-type dylib -Copt-level=2 -C prefer-dynamic "$TC/x/x13_dylib_dep.rs" -o "$deps/libdyldep.so" || r=1
  return $r
}

# run_direct <out-report> <crate> <src> <allowlist|-> <opt> <proto 0|1> <roots|-> <extra rustc args...>
# Environment for the run comes from the caller (ENVS array, UNSETS array).
run_direct() {
  local rep="$1" crate="$2" src="$3" al="$4" opt="$5" proto="$6" roots="$7"; shift 7
  local envs=(ALLOC_AUDIT_REPORT="$rep" ALLOC_AUDIT_EDGES=1)
  [[ " ${UNSETS[*]} " == *" CARGO_MANIFEST_DIR "* ]] || envs+=(CARGO_MANIFEST_DIR="$TC")
  [ "$al" != - ] && envs+=(ALLOC_AUDIT_ALLOWLIST="$al")
  [ "$proto" = 1 ] && envs+=(ALLOC_AUDIT_PROTOTYPE=1)
  [ "$roots" != - ] && envs+=(ALLOC_AUDIT_ROOTS="$roots")
  envs+=("${ENVS[@]}")
  local un=()
  for k in "${UNSETS[@]}"; do un+=(-u "$k"); done
  (cd "$(dirname "$src")" && env -u ALLOC_AUDIT_PLAN -u ALLOC_AUDIT_VSTD_VERSION -u OUT_DIR "${un[@]}" "${envs[@]}" \
     timeout 600 "$TOOL" --crate-name "$crate" --edition 2021 --crate-type lib -Copt-level="$opt" \
     --out-dir "$scratch/control-out" -L "$deps" "$@" "$src")
}
ENVS=(); UNSETS=()

# cargo_audit <report> <crate-dir> <crate-name> <allowlist|-> <target-dir>:
# build the crate under cargo with the tool as RUSTC_WRAPPER; prints the
# tool-equivalent exit status (0 PASS, 1 FAIL, 2 no verdict).
cargo_audit() {
  local rep="$1" dir="$2" crate="$3" al="$4" T="$5"
  local plan="$rep.plan"
  echo "$crate $al $rep" > "$plan"
  rm -f "$rep"
  (cd "$dir" && cargo clean --offline -p "$crate" --release --target x86_64-unknown-linux-gnu --target-dir "$T" >/dev/null 2>&1
   timeout 1800 env ALLOC_AUDIT_PLAN="$plan" ALLOC_AUDIT_EDGES=1 ALLOC_AUDIT_CONTINUE=1 \
     CARGO_TARGET_DIR="$T" RUSTC_WRAPPER="$TOOL" \
     cargo build --offline -j1 --release --lib --target x86_64-unknown-linux-gnu) >"$rep.stdout" 2>"$rep.stderr"
  local rc=$?
  if [ $rc != 0 ] || [ ! -f "$rep" ]; then return 2; fi
  grep -q '^verdict: PASS' "$rep" && return 0
  return 1
}
