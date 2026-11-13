#!/bin/bash
# Independent build-input inventory (SPEC §6 *Build-input inventory*; §8
# prerequisite 2), captured WITHOUT the audit tool's detection code:
#   * the build runs from an empty target directory (every build unit, build
#     script and proc macro actually executes) under `strace -f`, recording
#     every exec (each build unit's command line) and every file open;
#   * the environment is reduced (`env -i`) to the declared variables, and an
#     LD_PRELOAD shim logs every getenv() by every process, including names
#     that are absent (denied);
#   * cargo's --message-format=json artifact and build-script messages, and
#     rustc's dep-info files, are kept.
# compare.py then classifies every read, fails ("BLOCK") on any input it
# cannot account for, hashes every linked artifact, and compares its list
# mechanically with the audit report produced by the same build (the tool runs
# as RUSTC_WRAPPER in this build, so report and inventory describe one build).
#
# Usage: capture.sh <crate-dir> <crate-name> <allowlist|-> <out-dir> [declared-file]
# declared-file: lines `env NAME=VALUE` (passed into the reduced environment)
# and `file PATH` (a reviewed extra input); see README.md.
set -u
here="$(cd "$(dirname "$0")" && pwd)"
audit="$(cd "$here/.." && pwd)"
dir="$(cd "$1" && pwd)"; crate="$2"; al="$3"; out="$4"; declared="${5:-}"
mkdir -p "$out"; out="$(cd "$out" && pwd)"
scratch="${ALLOC_AUDIT_TARGET:-/tmp/claude-0/alloc-audit}"
T="$scratch/inventory/$(basename "$out")-target"
rm -rf "$T"; mkdir -p "$T"
tc=/opt/rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu
SR="$("$tc/bin/rustc" --print sysroot)"
shim="$scratch/inventory/getenv_log.so"
gcc -O2 -shared -fPIC -o "$shim" "$here/getenv_log.c" -ldl || exit 2
tool="$scratch/tool-target/release/alloc_audit"
[ -x "$tool" ] || { echo "capture: build the tool first (run_audit.sh or controls)"; exit 2; }
echo "$crate $al $out/report.txt" > "$out/plan"
[ "$crate" = ico_verus ] && echo "dib_verus $audit/allowlist/x86_64-unknown-linux-gnu/release/dib_verus.toml $out/dib_verus-in-ico.txt" >> "$out/plan"
envs=(PATH=/opt/cargo/bin:/usr/bin:/bin HOME="$HOME" CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"
      RUSTUP_HOME="${RUSTUP_HOME:-/opt/rustup}" RUSTUP_TOOLCHAIN=1.99.1-x86_64-unknown-linux-gnu
      LD_LIBRARY_PATH="$SR/lib/rustlib/x86_64-unknown-linux-gnu/lib"
      CARGO_TARGET_DIR="$T" RUSTC_WRAPPER="$tool" ALLOC_AUDIT_PLAN="$out/plan" ALLOC_AUDIT_CONTINUE=1
      ALLOC_AUDIT_EDGES=1 ALLOC_AUDIT_RUN_DIR="$out"
      LD_PRELOAD="$shim" ALLOC_INV_GETENV_LOG="$out/getenv.log")
if [ -n "$declared" ]; then
  while read -r kind rest; do
    [ "$kind" = env ] && envs+=("$rest")
  done < "$declared"
  cp "$declared" "$out/declared.txt"
else
  : > "$out/declared.txt"
fi
printf '%s\n' "${envs[@]}" > "$out/environment.txt"
rm -f "$out/getenv.log"
(cd "$dir" && timeout 3600 env -i "${envs[@]}" strace -f -qq -y -s 4096 -v -e trace=execve,openat,open,chdir,clone,clone3,fork,vfork -o "$out/strace.log" \
   cargo build --offline -j1 --release --lib --target x86_64-unknown-linux-gnu --message-format=json \
   >"$out/cargo.json" 2>"$out/cargo.stderr")
echo "cargo exit $?" > "$out/cargo.exit"
find "$T" -name '*.d' -path '*/deps/*' -exec cp {} "$out/" \; 2>/dev/null
python3 "$here/compare.py" "$out" "$dir" "$crate" "$T" "$SR"
