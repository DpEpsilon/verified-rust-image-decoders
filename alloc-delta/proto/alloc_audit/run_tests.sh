#!/bin/bash
# Run the alloc_audit prototype over every testcase and print a compact verdict.
# Usage: run_tests.sh
# Assumes the tool is built; pass the binary path in ALLOC_AUDIT_BIN.
set -u
here="$(cd "$(dirname "$0")" && pwd)"
export PATH=/opt/cargo/bin:/opt/verus:$PATH
SR=$(rustc --print sysroot)
export LD_LIBRARY_PATH="$SR/lib/rustlib/x86_64-unknown-linux-gnu/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
bin="${ALLOC_AUDIT_BIN:-/tmp/claude-0/alloc-audit-target/debug/alloc_audit}"
outdir="${here}/reports"
mkdir -p "$outdir"

# Build the toy proc-macro crate for control n16_proc_macro. It depends only on
# `proc_macro` (from the sysroot), so this needs no crates.io download and works
# under --offline. The `.so` is written to the scratch target dir (not the
# project tree): it is a build artifact, regenerated on every run. If the build
# fails, the n16 control is not run and reports that, rather than pretending the
# control ran.
pm_dir="/tmp/claude-0/alloc-audit-pm"
mkdir -p "$pm_dir"
pm_so="${pm_dir}/libpmtoy.so"
if rustc --crate-name pmtoy --edition 2021 --crate-type proc-macro -Copt-level=2 \
    "$here/testcases/pmtoy.rs" -o "$pm_so" 2>"$outdir/n16_pmtoy_build.stderr"; then
  pm_ok=1
else
  pm_ok=0
fi

# The fixtures carry reviewer = "prototype-fixture", so the run needs the
# explicit prototype flag; it is printed in the report header. Provenance runs
# that must *reject* the placeholder unset it (see `run_no_proto`).
run_one() {
  local tag="$1" crate="$2" file="$3" roots="$4" allowlist="${5:-}" optlevel="${6:-2}" proto="${7:-1}" extra="${8:-}" extra_env="${9:-}" unset_env="${10:-}"
  local envs=(ALLOC_AUDIT_ROOTS="$roots" ALLOC_AUDIT_REPORT="$outdir/$tag.txt")
  if [ -n "$allowlist" ]; then
    envs+=(ALLOC_AUDIT_ALLOWLIST="$allowlist")
  fi
  if [ "$proto" = "1" ]; then
    envs+=(ALLOC_AUDIT_PROTOTYPE=1)
  fi
  # Extra environment (space-separated KEY=VALUE) for controls that need one,
  # e.g. ALLOC_AUDIT_VSTD_VERSION or OUT_DIR (the build-script probe).
  if [ -n "$extra_env" ]; then
    for kv in $extra_env; do envs+=("$kv"); done
  fi
  # Environment keys to *remove* (space-separated) for a control that must run
  # with the variable absent, e.g. c3_vstd_undet needs ALLOC_AUDIT_VSTD_VERSION
  # unset even if the caller exported it.
  local unset_args=()
  if [ -n "$unset_env" ]; then
    for k in $unset_env; do unset_args+=(-u "$k"); done
  fi
  # -Z flags need the nightly gate open; set it only for the case that passes one.
  if [ -n "$extra" ]; then envs+=(RUSTC_BOOTSTRAP=1); fi
  env "${unset_args[@]}" "${envs[@]}" timeout 300 "$bin" \
      --crate-name "$crate" --edition 2021 --crate-type lib -Copt-level="$optlevel" \
      ${extra:+$extra} "$file" >"$outdir/$tag.stdout" 2>"$outdir/$tag.stderr"
  local rc=$?
  printf '%-24s exit=%s  ' "$tag" "$rc"
  grep -E '^(verdict|allocator entry points reachable|fail-closed findings|unexplained leaves):' \
      "$outdir/$tag.stderr" | tr '\n' ' '
  echo
}

echo "== negative controls (expected: exit 1, an allocation reported) =="
run_one n1_push_unreserved n1 "$here/testcases/n1_push_unreserved.rs" "n1_push_unreserved"
run_one n2_clone          n2 "$here/testcases/n2_clone.rs"            "n2_clone"
run_one n3_format         n3 "$here/testcases/n3_format.rs"           "n3_format"
run_one n4_box            n4 "$here/testcases/n4_box.rs"              "n4_box"
run_one n5_collect        n5 "$here/testcases/n5_collect.rs"          "n5_collect"
run_one n6_fnptr          n6 "$here/testcases/n6_fnptr.rs"            "n6_fnptr"
run_one n7_dyn            n7 "$here/testcases/n7_dyn.rs"              "n7_dyn"
run_one n7b_dyn_local     n7b "$here/testcases/n7b_dyn_local.rs"      "n7b_dyn_local"
run_one n8_helper         n8 "$here/testcases/n8_helper.rs"           "n8_caller"
run_one n9_drop           n9 "$here/testcases/n9_drop.rs"             "n9_drop"
run_one n10_ffi           n10 "$here/testcases/n10_ffi.rs"            "n10_ffi"

echo
echo "== shared-instance control (expected: exit 1; the allowlist admits the guard only) =="
run_one n11_shared_instance n11 "$here/testcases/n11_shared_instance.rs" "n11_shared_instance" \
    "$here/allowlist/n11_shared_instance.toml"

echo
echo "== clean control (expected with allowlist: exit 0) =="
run_one c1_clean c1 "$here/testcases/c1_clean.rs" "c1_clean_root" \
    "$here/allowlist/c1_clean.toml"

echo
echo "== mutation of the clean control (expected: exit 1 despite the allowlist) =="
run_one c1_mutated c1m "$here/testcases/c1_mutated.rs" "c1_mutated_root" \
    "$here/allowlist/c1_clean.toml"

echo
echo "== provenance controls (task 4/5) =="
# Profile: a release allowlist against a debug build must fail on the profile.
run_one c1_prof_mismatch c1 "$here/testcases/c1_clean.rs" "c1_clean_root" \
    "$here/allowlist/c1_clean.toml" 0
# Profile: an allowlist recording a different opt-level must fail.
run_one c1_prof_bad c1 "$here/testcases/c1_clean.rs" "c1_clean_root" \
    "$here/allowlist/c1_profile_mismatch.toml" 2
# Source hash: a wrong recorded hash must fail.
run_one c1_hash_bad c1 "$here/testcases/c1_clean.rs" "c1_clean_root" \
    "$here/allowlist/c1_bad_hash.toml" 2
# Reviewer: an empty reviewer must fail.
run_one c1_rev_empty c1 "$here/testcases/c1_clean.rs" "c1_clean_root" \
    "$here/allowlist/c1_bad_reviewer.toml" 2
# Reviewer: the prototype placeholder *without* --prototype must fail.
run_one c1_rev_fixture c1 "$here/testcases/c1_clean.rs" "c1_clean_root" \
    "$here/allowlist/c1_clean.toml" 2 0
# Reviewer: a named reviewer is accepted without --prototype.
run_one c1_rev_named c1 "$here/testcases/c1_clean.rs" "c1_clean_root" \
    "$here/allowlist/c1_named_reviewer.toml" 2 0
# Commit mismatch fixture.
run_one c1_commit_mismatch c1 "$here/testcases/c1_clean.rs" "c1_clean_root" \
    "$here/allowlist/c1_commit_mismatch.toml" 2
# Target mismatch fixture.
run_one c1_target_mismatch c1 "$here/testcases/c1_clean.rs" "c1_clean_root" \
    "$here/allowlist/c1_target_mismatch.toml" 2
# Source hash: the key is *absent* (not merely wrong), so the finding is the
# distinct *-missing one, and the tool fails closed.
run_one c1_source_hash_missing c1 "$here/testcases/c1_clean.rs" "c1_clean_root" \
    "$here/allowlist/c1_source_hash_missing.toml" 2
# Reviewer: the key is *absent* from [meta] (not merely empty), so the tool
# fails closed rather than accepting a file no human signed.
run_one c1_reviewer_missing c1 "$here/testcases/c1_clean.rs" "c1_clean_root" \
    "$here/allowlist/c1_reviewer_missing.toml" 2

echo
echo "== allowlist collision controls (expected: exit 1; a lookalike caller is refused) =="
# Suffix lookalike: unapproved_guard::try_zeroed must not be admitted by an
# entry that names guard::try_zeroed (the old ends_with/ends_with hole).
run_one n12_suffix_lookalike n12 "$here/testcases/n12_suffix_lookalike.rs" "n12_suffix_lookalike" \
    "$here/allowlist/n12_suffix_lookalike.toml"
# Root-level lookalike: a root-level try_zeroed must not be admitted by an entry
# that names guard::try_zeroed (the old reverse ends_with hole).
run_one n13_root_lookalike n13 "$here/testcases/n13_root_lookalike.rs" "n13_root_lookalike" \
    "$here/allowlist/n13_root_lookalike.toml"

echo
echo "== two-guard controls (expected: c2 passes; n14 fails on the third caller) =="
# Two admitted guard callers of the SAME instance, each with its own entry: the
# matcher must consult every matching entry, so this passes.
run_one c2_two_guards c2 "$here/testcases/c2_two_guards.rs" "c2_two_guards" \
    "$here/allowlist/c2_two_guards.toml"
# The same two guards plus a third unapproved caller: fails, naming the third.
run_one n14_two_guards_plus_third n14 "$here/testcases/n14_two_guards_plus_third.rs" \
    "n14_two_guards_plus_third" "$here/allowlist/n14_two_guards_plus_third.toml"

echo
echo "== missing-field controls (expected: exit 1, a distinct *-missing finding) =="
run_one c1_commit_missing c1 "$here/testcases/c1_clean.rs" "c1_clean_root" \
    "$here/allowlist/c1_commit_missing.toml" 2
run_one c1_target_missing c1 "$here/testcases/c1_clean.rs" "c1_clean_root" \
    "$here/allowlist/c1_target_missing.toml" 2
run_one c1_profile_missing c1 "$here/testcases/c1_clean.rs" "c1_clean_root" \
    "$here/allowlist/c1_profile_missing.toml" 2

echo
echo "== commit-length controls (expected: exit 1, [allowlist-commit]) =="
# A recorded commit longer than the actual hash must be refused (the old
# rec.starts_with(run) accepted it).
run_one c1_commit_toolong c1 "$here/testcases/c1_clean.rs" "c1_clean_root" \
    "$here/allowlist/c1_commit_toolong.toml" 2
# A recorded commit shorter than 12 hex characters must be refused.
run_one c1_commit_short c1 "$here/testcases/c1_clean.rs" "c1_clean_root" \
    "$here/allowlist/c1_commit_short.toml" 2

echo
echo "== vstd controls (expected: ok passes; change/missing/undeclared fail) =="
# A vstd-linked crate with the right recorded version passes.
run_one c3_vstd_ok c3 "$here/testcases/c3_vstd_linked.rs" "c3_vstd_root" \
    "$here/allowlist/c3_vstd_ok.toml" 2 1 "-L /opt/verus --extern vstd=/opt/verus/libvstd.rlib" \
    "ALLOC_AUDIT_VSTD_VERSION=0.0.0-2026-11-01-0251"
# A recorded vstd version that differs from the linked one fails.
run_one c3_vstd_change c3 "$here/testcases/c3_vstd_linked.rs" "c3_vstd_root" \
    "$here/allowlist/c3_vstd_change.toml" 2 1 "-L /opt/verus --extern vstd=/opt/verus/libvstd.rlib" \
    "ALLOC_AUDIT_VSTD_VERSION=0.0.0-2026-11-01-0251"
# No recorded vstd version on a vstd-linked crate fails.
run_one c3_vstd_missing c3 "$here/testcases/c3_vstd_linked.rs" "c3_vstd_root" \
    "$here/allowlist/c3_vstd_missing.toml" 2 1 "-L /opt/verus --extern vstd=/opt/verus/libvstd.rlib" \
    "ALLOC_AUDIT_VSTD_VERSION=0.0.0-2026-11-01-0251"
# A toy crate whose allowlist never declares vstd="none" fails.
run_one c1_vstd_undeclared c1 "$here/testcases/c1_clean.rs" "c1_clean_root" \
    "$here/allowlist/c1_vstd_undeclared.toml" 2
# A vstd-linked crate whose version cannot be resolved: no ALLOC_AUDIT_VSTD_VERSION
# (unset here even if the caller exported it) and no readable Cargo.lock, so the
# tool must fail closed with allowlist-vstd-undetermined rather than trusting the
# recorded version.
run_one c3_vstd_undet c3 "$here/testcases/c3_vstd_linked.rs" "c3_vstd_root" \
    "$here/allowlist/c3_vstd_undet.toml" 2 1 "-L /opt/verus --extern vstd=/opt/verus/libvstd.rlib" \
    "" "ALLOC_AUDIT_VSTD_VERSION CARGO_MANIFEST_DIR"

echo
echo "== build-input controls (expected: exit 1, an undeclared-* finding) =="
# include_str! of a non-source file is not covered by the source hash.
run_one n15_include n15 "$here/testcases/n15_include.rs" "n15_include"
# A build script (probed via OUT_DIR) is not covered by the source hash.
run_one bs_build_script c1 "$here/testcases/c1_clean.rs" "c1_clean_root" \
    "" 2 1 "" "OUT_DIR=/tmp/claude-0/alloc-audit-fake-out"
# A foreign proc-macro crate whose expansion is not in the source map is not
# covered by the source hash; the tool must fail closed unless the allowlist
# declares it. The proc-macro crate is the toy `pmtoy` built above; a run with
# no declaration must report `undeclared-proc-macro` naming `pmtoy`.
if [ "$pm_ok" = "1" ]; then
  run_one n16_proc_macro n16 "$here/testcases/n16_proc_macro.rs" "n16_proc_macro" \
      "" 2 1 "-L $pm_dir --extern pmtoy=$pm_so"
else
  echo "n16_proc_macro            NOT RUN (toy proc-macro crate failed to build; see reports/n16_pmtoy_build.stderr)"
fi

echo
echo "== share-generics control (expected: exit 1, finding [share-generics]) =="
# The tool must fail closed when -Zshare-generics is on. c1_clean at opt-level=2
# (share-generics off by default) with the flag forced on. The allowlist is
# c1_clean.toml with its recorded profile changed to share-generics=on, so the
# profile matches and [share-generics] is the only fail-closed finding.
run_one c1_share_generics c1 "$here/testcases/c1_clean.rs" "c1_clean_root" \
    "$here/allowlist/c1_share_generics.toml" 2 1 "-Zshare-generics=y"
