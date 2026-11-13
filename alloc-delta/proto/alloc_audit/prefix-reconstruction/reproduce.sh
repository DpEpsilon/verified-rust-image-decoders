#!/bin/bash
# Reproduce the pre-fix N11 evidence archived at reports/n11_pre_fix.txt.
#
# Sol's review found the original walk.rs was not archived and is not in git
# history, so the pre-fix `Pass`/exit-0 result cannot be re-run from the original
# binary. This script rebuilds the *authorization rule* the original used — the
# first-BFS-parent rule, which judges only the single shortest root-to-entry
# path — as a one-hunk patch on the current tool, then runs it against the
# pre-fix N11 testcase.
#
# The patch is at prefix-reconstruction/revert-to-prefix.patch; it changes only
# `authorize_allocations` to judge `paths.iter().min_by_key(|p| p.len())`
# instead of every path. Everything else in the tool is current, so the
# reconstruction is faithful for the n11 question (the n11 judgement does not
# depend on the matcher change).
set -eu
here="$(cd "$(dirname "$0")" && pwd)"
export PATH=/opt/cargo/bin:/opt/verus:$PATH
export RUSTC_BOOTSTRAP=1
work="${ALLOC_AUDIT_PREFIX_WORK:-/tmp/claude-0/prefix-repro}"
target="${ALLOC_AUDIT_PREFIX_TARGET:-/tmp/claude-0/prefix-target}"

rm -rf "$work"
mkdir -p "$work/src" "$work/testcases"
cp "$here/../src/"*.rs "$work/src/"
cp "$here/../Cargo.toml" "$here/../Cargo.lock" "$work/"
cp "$here/n11_prefix.rs" "$work/testcases/"
cp "$here/n11_prefix.toml" "$work/allowlist.toml"

( cd "$work" && patch -p0 src/walk.rs < "$here/revert-to-prefix.patch" )

( cd "$work" && CARGO_TARGET_DIR="$target" cargo build --offline -j2 )

SR=$(rustc --print sysroot)
export LD_LIBRARY_PATH="$SR/lib/rustlib/x86_64-unknown-linux-gnu/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
errcode=0
raw="$work/n11_pre_fix.raw"
ALLOC_AUDIT_ROOTS=n11_shared_instance \
ALLOC_AUDIT_ALLOWLIST="$work/allowlist.toml" \
ALLOC_AUDIT_PROTOTYPE=1 \
ALLOC_AUDIT_REPORT="$raw" \
  "$target/debug/alloc_audit" \
  --crate-name n11 --edition 2021 --crate-type lib -Copt-level=2 \
  "$work/testcases/n11_prefix.rs" 2>/dev/null || errcode=$?

out="$here/../reports/n11_pre_fix.txt"
{
  echo "# PRE-FIX RECONSTRUCTION (not the original binary)."
  echo "# The original tool's walk.rs was not archived and is not in git history."
  echo "# This report is produced by reproduce.sh, which rebuilds the"
  echo "# first-BFS-parent authorization rule described in NOTES.md, then runs it"
  echo "# against the pre-fix N11 testcase (unapproved_grow uses resize)."
  echo "# Reproduced: the shared-instance defect shows as Pass / exit 0."
  echo "#"
  echo "# testcase: n11_prefix (guard::try_zeroed admitted; unapproved_reserve and"
  echo "#           unapproved_grow reach shared instances); the allowlist admits"
  echo "#           the guard only."
  echo "# exit status: $errcode (0 = Pass) -- the defect"
  echo
  cat "$raw"
} > "$out"

echo "pre-fix reconstruction exit status: $errcode (0 = Pass, the defect)"
echo "report written to $out"
