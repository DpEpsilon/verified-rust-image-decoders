#!/bin/bash
# Run every control of the production allocation audit and check each one's
# expected verdict, required finding tags and required report text. A control
# whose expectation does not hold is a FAILED control, and the runner exits
# nonzero: if the tool stops flagging a seeded violation, this script says so.
#
# Usage: controls/run_controls.sh [--out DIR] [--only REGEX]
# Default DIR: alloc-delta/audit/runs/<UTC timestamp>-controls
source "$(dirname "$0")/lib.sh"
out=""; only="."
while [ $# -gt 0 ]; do
  case "$1" in
    --out) out="$2"; shift ;;
    --only) only="$2"; shift ;;
    *) echo "unknown argument $1" >&2; exit 2 ;;
  esac
  shift
done
stamp="$(date -u +%Y%m%dT%H%M%SZ)"
[ -z "$out" ] && out="$audit/runs/$stamp-controls"
mkdir -p "$out" "$scratch/control-out"
out="$(cd "$out" && pwd)"
export ALLOC_AUDIT_RUN_DIR="$out"
build_tool || exit 2
build_deps 2>"$out/deps-build.stderr" || { echo "control dependency build failed (deps-build.stderr)"; exit 2; }
FX="$ctl/fixtures"
summary="$out/CONTROLS.tsv"
printf 'control\texpected\tverdict\texit\tresult\tfinding-tags\tfailed-checks\n' > "$summary"
n_ok=0; n_bad=0; n_sel=0
# Every control below is counted when selected; the full suite must produce
# exactly this many result rows, so a control that silently disappears (for
# example a seed generator that fails before running anything) fails the run.
EXPECTED_TOTAL=98

# check <tag> <exit> <expect PASS|FAIL> <required tags, comma> <needles, |-sep> <forbidden tags, comma>
check() {
  local tag="$1" rc="$2" expect="$3" req="$4" needles="$5" forbid="$6"
  # A corrupt or unparsable fixture must never let a control fail "for free".
  forbid="${forbid:+$forbid,}allowlist-load,allowlist-format"
  local rep="$out/$tag.txt" why=()
  local verdict tags
  if [ ! -f "$rep" ]; then
    why+=("no report (exit $rc)")
    verdict="-"; tags="-"
  else
    verdict=$(sed -n 's/^verdict: //p' "$rep" | head -1)
    tags=$(sed -n 's/^finding-tags: //p' "$rep" | head -1)
    [ "$verdict" = "$expect" ] || why+=("verdict $verdict, expected $expect")
    if [ "$expect" = PASS ]; then [ "$rc" = 0 ] || why+=("exit $rc for a PASS"); else [ "$rc" = 1 ] || why+=("exit $rc for a FAIL"); fi
    local t
    IFS=, read -ra arr <<< "$req"
    for t in "${arr[@]}"; do [ -z "$t" ] || [[ ",$tags," == *",$t,"* ]] || why+=("missing tag $t"); done
    IFS=, read -ra arr <<< "$forbid"
    for t in "${arr[@]}"; do [ -z "$t" ] || [[ ",$tags," != *",$t,"* ]] || why+=("forbidden tag $t present"); done
    IFS='|' read -ra arr <<< "$needles"
    for t in "${arr[@]}"; do [ -z "$t" ] || grep -qF -- "$t" "$rep" || why+=("report lacks text: $t"); done
  fi
  local res=OK
  if [ ${#why[@]} -gt 0 ]; then res=FAILED; n_bad=$((n_bad+1)); else n_ok=$((n_ok+1)); fi
  printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\n' "$tag" "$expect" "$verdict" "$rc" "$res" "$tags" "$(IFS=';'; echo "${why[*]:-}")" >> "$summary"
  printf '%-32s %-6s %-4s exit=%s  %s\n' "$tag" "$res" "$verdict" "$rc" "$([ $res = OK ] || (IFS=';'; echo "${why[*]}"))"
}

# d <tag> <expect> <req> <needles> <forbid> <crate> <src> <allowlist|-> <opt> <proto> [extra rustc args...]
d() {
  local tag="$1" expect="$2" req="$3" needles="$4" forbid="$5" crate="$6" src="$7" al="$8" opt="$9" proto="${10}"; shift 10
  [[ "$tag" =~ $only ]] || return 0
  n_sel=$((n_sel+1))
  rm -f "$out/$tag.txt"
  run_direct "$out/$tag.txt" "$crate" "$src" "$al" "$opt" "$proto" - "$@" >"$out/$tag.stdout" 2>"$out/$tag.stderr"
  check "$tag" $? "$expect" "$req" "$needles" "$forbid"
  ENVS=(); UNSETS=()
}

# cargo_ctl <tag> <expect> <req> <needles> <forbid> <crate-dir> <crate-name> <allowlist|-> <target-dir>
cargo_ctl() {
  local tag="$1" expect="$2" req="$3" needles="$4" forbid="$5" dir="$6" crate="$7" al="$8" T="$9"
  [[ "$tag" =~ $only ]] || return 0
  n_sel=$((n_sel+1))
  cargo_audit "$out/$tag.txt" "$dir" "$crate" "$al" "$T"
  check "$tag" $? "$expect" "$req" "$needles" "$forbid"
}

L="$TC/legacy"; X="$TC/x"
VSTD=(-L /opt/verus --extern vstd=/opt/verus/libvstd.rlib)
echo "== the 42 F1 prototype controls, ported (expected verdicts unchanged) =="
d n1_push_unreserved FAIL unauthorized-alloc-path "crate::n1_push_unreserved -> " "" n1 "$L/n1_push_unreserved.rs" - 2 1
d n2_clone FAIL unauthorized-alloc-path "crate::n2_clone -> " "" n2 "$L/n2_clone.rs" - 2 1
d n3_format FAIL unauthorized-alloc-path "crate::n3_format -> " "" n3 "$L/n3_format.rs" - 2 1
d n4_box FAIL unauthorized-alloc-path "crate::n4_box -> " "" n4 "$L/n4_box.rs" - 2 1
d n5_collect FAIL unauthorized-alloc-path "crate::n5_collect -> " "" n5 "$L/n5_collect.rs" - 2 1
d n6_fnptr FAIL fn-ptr-call,unauthorized-alloc-path "crate::n6_fnptr: indirect call" "" n6 "$L/n6_fnptr.rs" - 2 1
d n7_dyn FAIL virtual-dispatch "virtual" "" n7 "$L/n7_dyn.rs" - 2 1
d n7b_dyn_local FAIL vtable-unsize,virtual-dispatch "crate::n7b_dyn_local: unsizes" "" n7b "$L/n7b_dyn_local.rs" - 2 1
d n8_helper FAIL unauthorized-alloc-path "crate::helper_grow -> " "" n8 "$L/n8_helper.rs" - 2 1
d n9_drop FAIL unauthorized-alloc-path "<crate::Dropper as core::ops::drop::Drop>::drop -> " "" n9 "$L/n9_drop.rs" - 2 1
d n10_ffi FAIL unexplained-leaf "[ffi] crate::getpid" "" n10 "$L/n10_ffi.rs" - 2 1
d n11_shared_instance FAIL unauthorized-alloc-path "crate::unapproved_reserve -> |crate::unapproved_grow -> " "" n11 "$L/n11_shared_instance.rs" "$FX/n11_shared_instance.toml" 2 1
d c1_clean PASS "" "" "" c1 "$L/c1_clean.rs" "$FX/c1_clean.toml" 2 1
d c1_mutated FAIL unauthorized-alloc-path,allowlist-source-hash "crate::scratch_mutated -> " "" c1 "$L/c1_mutated.rs" "$FX/c1_clean.toml" 2 1
d c1_prof_mismatch FAIL share-generics,allowlist-profile "" "" c1 "$L/c1_clean.rs" "$FX/c1_clean.toml" 0 1
d c1_prof_bad FAIL allowlist-profile "" "" c1 "$L/c1_clean.rs" "$FX/c1_profile_mismatch.toml" 2 1
d c1_hash_bad FAIL allowlist-source-hash "" "" c1 "$L/c1_clean.rs" "$FX/c1_bad_hash.toml" 2 1
d c1_rev_empty FAIL allowlist-reviewer "" "" c1 "$L/c1_clean.rs" "$FX/c1_bad_reviewer.toml" 2 1
d c1_rev_fixture FAIL allowlist-reviewer "prototype placeholder" "" c1 "$L/c1_clean.rs" "$FX/c1_clean.toml" 2 0
d c1_rev_named PASS "" "mode: PRODUCTION" "" c1 "$L/c1_clean.rs" "$FX/c1_named_reviewer.toml" 2 0
d c1_commit_mismatch FAIL allowlist-commit "" "" c1 "$L/c1_clean.rs" "$FX/c1_commit_mismatch.toml" 2 1
d c1_target_mismatch FAIL allowlist-target "" "" c1 "$L/c1_clean.rs" "$FX/c1_target_mismatch.toml" 2 1
d c1_source_hash_missing FAIL allowlist-source-hash-missing "" "" c1 "$L/c1_clean.rs" "$FX/c1_source_hash_missing.toml" 2 1
d c1_reviewer_missing FAIL allowlist-reviewer "" "" c1 "$L/c1_clean.rs" "$FX/c1_reviewer_missing.toml" 2 1
d n12_suffix_lookalike FAIL unauthorized-alloc-path "crate::unapproved_guard::try_zeroed -> " "" n12 "$L/n12_suffix_lookalike.rs" "$FX/n12_suffix_lookalike.toml" 2 1
d n13_root_lookalike FAIL unauthorized-alloc-path "crate::try_zeroed -> " "" n13 "$L/n13_root_lookalike.rs" "$FX/n13_root_lookalike.toml" 2 1
d c2_two_guards PASS "" "" "" c2 "$L/c2_two_guards.rs" "$FX/c2_two_guards.toml" 2 1
d n14_two_guards_plus_third FAIL unauthorized-alloc-path "crate::unapproved_third -> " "" n14 "$L/n14_two_guards_plus_third.rs" "$FX/n14_two_guards_plus_third.toml" 2 1
d c1_commit_missing FAIL allowlist-commit-missing "" "" c1 "$L/c1_clean.rs" "$FX/c1_commit_missing.toml" 2 1
d c1_target_missing FAIL allowlist-target-missing "" "" c1 "$L/c1_clean.rs" "$FX/c1_target_missing.toml" 2 1
d c1_profile_missing FAIL allowlist-profile-missing "" "" c1 "$L/c1_clean.rs" "$FX/c1_profile_missing.toml" 2 1
d c1_commit_toolong FAIL allowlist-commit "" "" c1 "$L/c1_clean.rs" "$FX/c1_commit_toolong.toml" 2 1
d c1_commit_short FAIL allowlist-commit "" "" c1 "$L/c1_clean.rs" "$FX/c1_commit_short.toml" 2 1
ENVS=(ALLOC_AUDIT_VSTD_VERSION=0.0.0-2026-11-01-0251)
d c3_vstd_ok PASS "" "mode: PROTOTYPE" "" c3 "$L/c3_vstd_linked.rs" "$FX/c3_vstd_ok.toml" 2 1 "${VSTD[@]}"
ENVS=(ALLOC_AUDIT_VSTD_VERSION=0.0.0-2026-11-01-0251)
d c3_vstd_change FAIL allowlist-vstd "" "" c3 "$L/c3_vstd_linked.rs" "$FX/c3_vstd_change.toml" 2 1 "${VSTD[@]}"
ENVS=(ALLOC_AUDIT_VSTD_VERSION=0.0.0-2026-11-01-0251)
d c3_vstd_missing FAIL allowlist-vstd-missing "" "" c3 "$L/c3_vstd_linked.rs" "$FX/c3_vstd_missing.toml" 2 1 "${VSTD[@]}"
d c1_vstd_undeclared FAIL allowlist-vstd-not-declared-none "" "" c1 "$L/c1_clean.rs" "$FX/c1_vstd_undeclared.toml" 2 1
UNSETS=(ALLOC_AUDIT_VSTD_VERSION CARGO_MANIFEST_DIR)
d c3_vstd_undet FAIL allowlist-vstd-undetermined "" "" c3 "$L/c3_vstd_linked.rs" "$FX/c3_vstd_undet.toml" 2 1 "${VSTD[@]}"
d n15_include FAIL undeclared-include "legacy/n15_payload.txt" "" n15 "$L/n15_include.rs" - 2 1
ENVS=(OUT_DIR="$scratch/fake-out-dir")
d bs_build_script FAIL undeclared-build-script "OUT_DIR is set" "" c1 "$L/c1_clean.rs" - 2 1
d n16_proc_macro FAIL undeclared-proc-macro "proc-macro crate pmtoy" "" n16 "$L/n16_proc_macro.rs" - 2 1 --extern pmtoy="$deps/libpmtoy.so"
ENVS=(RUSTC_BOOTSTRAP=1)
d c1_share_generics FAIL share-generics "" "" c1 "$L/c1_clean.rs" "$FX/c1_share_generics.toml" 2 1 -Zshare-generics=y

echo "== required production controls (SPEC §6, §9) and one per fixed prototype defect =="
d x1a_ambiguous FAIL allowlist-ambiguous "\"guard::try_zeroed\" resolves to 2 distinct items" "" x1 "$X/x1_extern_namesake.rs" "$FX/x1a_ambiguous.toml" 2 0 --extern guard="$deps/libguard.rlib"
d x1b_extern_unauthorized FAIL unauthorized-alloc-path "crate::x1_root -> guard::try_zeroed -> |e.g. crate::guard::try_zeroed -> " "" x1 "$X/x1_extern_namesake.rs" "$FX/x1_extern_namesake.toml" 2 0 --extern guard="$deps/libguard.rlib"
d x2_include_after_url FAIL undeclared-include "x/p.bin" "" x2 "$X/x2_include_after_url.rs" "$FX/x2_include_after_url.toml" 2 0
d x2_declared PASS "" "include x/p.bin" "" x2 "$X/x2_include_after_url.rs" "$FX/x2_declared.toml" 2 0
d x2b_renamed_import FAIL undeclared-include "x/p.bin" "" x2b "$X/x2b_renamed_import.rs" - 2 0
d x2c_foreign_macro FAIL undeclared-include "x/p.bin" "" x2c "$X/x2c_foreign_macro.rs" - 2 0 --extern x2_incmac="$deps/libx2_incmac.rlib"
d x3_wildcard_build_input FAIL allowlist-wildcard-build-input "" "" c1 "$L/c1_clean.rs" "$FX/x3_wildcard_build_input.toml" 2 0
d x4_verus_prefix FAIL undeclared-proc-macro "proc-macro crate verus_evil" "" x4 "$X/x4_verus_prefix.rs" - 2 0 --extern verus_evil="$deps/libverus_evil.so"
d x5_no_allowlist FAIL allowlist-absent "diagnostic: no allowlist" "" c1 "$L/c1_clean.rs" - 2 0
ENVS=(ALLOC_AUDIT_VSTD_VERSION=0.0.0-2026-11-01-0251)
d x6_vstd_override FAIL vstd-override-forbidden "" "" c1 "$L/c1_clean.rs" "$FX/c1_named_reviewer.toml" 2 0
UNSETS=(CARGO_MANIFEST_DIR); ENVS=(CARGO_MANIFEST_DIR="$ctl/vstd-manifest")
d x7_vstd_artifact FAIL allowlist-vstd-artifact "the linked vstd rlib has crate hash" "" c3 "$L/c3_vstd_linked.rs" "$FX/x7_vstd_artifact.toml" 2 0 "${VSTD[@]}"
cargo_ctl x8_real_build_script FAIL undeclared-build-script "build.rs exists at the package root" "" "$ctl/cargo/x8" x8_real_build_script - "$scratch/control-x8"
cargo_ctl x8b_build_script_declared PASS "" "build-script" "" "$ctl/cargo/x8" x8_real_build_script "$FX/x8_declared.toml" "$scratch/control-x8"
d x11_std_reachable FAIL std-instance "an instance from std is reachable" "" x11 "$X/x11_std_reachable.rs" - 2 0
# helperdep is audited first; its audited rlib (control-out) is what x12/x12b/x12c link.
d x12_helperdep PASS "" "" "" helperdep "$X/x12_helperdep.rs" "$FX/x12_helperdep.toml" 2 0 -C metadata=one
d x12_foreign_crate FAIL foreign-crate-instance "from crate helperdep" "" x12 "$X/x12_foreign_crate.rs" "$FX/x12_foreign_crate.toml" 2 0 --extern helperdep="$scratch/control-out/libhelperdep.rlib"
d x12b_admitted PASS "" "dependency contracts" "" x12 "$X/x12_foreign_crate.rs" "$FX/x12b_admitted.toml" 2 0 --extern helperdep="$scratch/control-out/libhelperdep.rlib"
d x12c_ambiguous_crate FAIL allowlist-ambiguous-crate,foreign-crate-instance "matches 2 crates" "" x12c "$X/x12c_two_helpers.rs" "$FX/x12c_two_helpers.toml" 2 0 --extern helperdep="$scratch/control-out/libhelperdep.rlib" --extern hd2="$deps/libhelperdep2.rlib"
d x13_dylib_not_proc_macro FAIL dylib-dependency "crate dyldep is linked as a dylib" undeclared-proc-macro x13 "$X/x13_dylib_not_proc_macro.rs" - 2 0 --extern dyldep="$deps/libdyldep.so"
d x14_inlined_helper FAIL unauthorized-alloc-path "crate::guard::try_zeroed -> crate::guard::helper_push -> " "" x14 "$X/x14_inlined_helper.rs" "$FX/x14_inlined_helper.toml" 2 0
d x15_generic_export FAIL generic-pub-fn "crate::x15_generic" "" x15 "$X/x15_generic_export.rs" - 2 0
d x16_guard_inline_never FAIL guard-inline-never "crate::alloc_guard::try_zeroed" "" x16 "$X/x16_guard_inline.rs" - 2 0
d x17_forbidden FAIL forbidden-item "core::mem::forget|alloc::rc::" "" x17 "$X/x17_forget.rs" - 2 0
d x19_const_fnptr FAIL fn-ptr-call "crate::x19_root: indirect call" "" x19 "$X/x19_const_fnptr.rs" - 2 0
d x20_const_dyn FAIL virtual-dispatch "" "" x20 "$X/x20_const_dyn.rs" - 2 0
d x21_inline_asm FAIL inline-asm "" "" x21 "$X/x21_asm.rs" - 2 0
d x22_unused_entry FAIL allowlist-unused-entry "crate::Holder::len" "" c1 "$L/c1_clean.rs" "$FX/x22_unused_entry.toml" 2 0
d x23_star_alloc FAIL unauthorized-alloc-path "crate::x23_root -> " "" x23 "$X/x23_star_alloc.rs" "$FX/x23_star_alloc.toml" 2 0
d x25_extern_malloc FAIL unauthorized-alloc-path "crate::malloc (Alloc)" "unexplained-leaf" x25 "$X/x25_extern_malloc.rs" - 2 0
ENVS=(ALLOC_AUDIT_X24=x24value)
d x24_env_declared PASS "" "env ALLOC_AUDIT_X24" "" x24 "$X/x24_env.rs" "$FX/x24_env.toml" 2 0
ENVS=(ALLOC_AUDIT_X24=x24value)
d x24_env_undeclared FAIL undeclared-env "ALLOC_AUDIT_X24" "" x24 "$X/x24_env.rs" "$FX/x24_env_undeclared.toml" 2 0

echo "== controls added for Sol's review (2026-11-13) =="
d x27_callback_cycle FAIL unauthorized-alloc-path "crate caller <crate::X<K> as core::clone::Clone>::clone<false>" "" cycle "$X/x27_callback_cycle.rs" "$FX/x27_callback_cycle.toml" 3 0
d x27b_cycle_admitted PASS "" "" "" cycle "$X/x27_callback_cycle.rs" "$FX/x27b_cycle_admitted.toml" 3 0
d x28_generic_caller_args FAIL unauthorized-alloc-path "crate::root -> crate::guard<u16> -> " "" generic "$X/x28_generic_caller.rs" "$FX/x28_generic_caller.toml" 3 0
d x28b_both_instantiations PASS "" "" "" generic "$X/x28_generic_caller.rs" "$FX/x28b_both_admitted.toml" 3 0
d x12_helperdep_proto PASS "" "mode: PROTOTYPE" "" helperdep "$X/x12_helperdep.rs" "$FX/x12_helperdep.toml" 2 1 -C metadata=one
d x31_dependency_not_production FAIL dependency-contract "was not produced in production mode" "" x12 "$X/x12_foreign_crate.rs" "$FX/x31_dependency_not_production.toml" 2 0 --extern helperdep="$scratch/control-out/libhelperdep.rlib"
echo "== controls added for Sol's re-check (2026-11-13) =="
d x33_callback_direct_alloc FAIL unauthorized-alloc-path "since entering crate function <crate::X<K> as core::clone::Clone>::clone<false>" "" callback "$X/x33_callback_direct_alloc.rs" "$FX/x33_callback_direct_alloc.toml" 3 0
d x33b_callback_no_alloc PASS "" "" "" callback "$X/x33b_callback_no_alloc.rs" "$FX/x33b_callback_no_alloc.toml" 3 0
ENVS=(RUSTC_BOOTSTRAP=1)
d x34_leafpanic_caller_args FAIL unexplained-leaf "findings: 2|[intrinsic] core::intrinsics::black_box<u8> [intrinsic] reached from crate::guard<u16>|[panic] PanicBoundsCheck in crate::guard<u16>" "" leafpanic "$X/x34_leafpanic_caller_args.rs" "$FX/x34_leafpanic_caller_args.toml" 3 0
# meta <tag> <expected exit> <needle> -- <command...>: a control on a command's
# exit status and output rather than on an audit report.
meta() {
  local tag="$1" want="$2" needle="$3"; shift 4
  [[ "$tag" =~ $only ]] || return 0
  n_sel=$((n_sel+1))
  "$@" >"$out/$tag.stdout" 2>"$out/$tag.stderr"
  local rc=$? why=()
  [ "$rc" = "$want" ] || why+=("exit $rc, expected $want")
  grep -qF -- "$needle" "$out/$tag.stdout" "$out/$tag.stderr" || why+=("output lacks: $needle")
  local res=OK; if [ ${#why[@]} -gt 0 ]; then res=FAILED; n_bad=$((n_bad+1)); else n_ok=$((n_ok+1)); fi
  printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\n' "$tag" "exit=$want" "-" "$rc" "$res" "-" "$(IFS=';'; echo "${why[*]:-}")" >> "$summary"
  printf '%-32s %-6s      exit=%s  %s\n' "$tag" "$res" "$rc" "$([ $res = OK ] || (IFS=';'; echo "${why[*]}"))"
}
meta x29_prototype_flag_value 2 "the only accepted value is 1" -- env ALLOC_AUDIT_PROTOTYPE=0 ALLOC_AUDIT_REPORT="$out/x29.txt" CARGO_MANIFEST_DIR="$TC" "$TOOL" --crate-name c1 --edition 2021 --crate-type lib -Copt-level=2 --out-dir "$scratch/control-out" "$L/c1_clean.rs"
meta x30_runner_refuses_prototype 2 "refusing to run with ALLOC_AUDIT_PROTOTYPE set" -- env ALLOC_AUDIT_PROTOTYPE=1 "$audit/run_audit.sh" --out "$out/x30-run" tga
meta x30b_runner_refuses_roots 2 "refusing to run with ALLOC_AUDIT_ROOTS set" -- env ALLOC_AUDIT_ROOTS=crate::tga::try_decode "$audit/run_audit.sh" --out "$out/x30b-run" tga
if [ "${ALLOC_AUDIT_CONTROLS_SELFTEST:-}" != 1 ]; then
  # Self-test: a targeted run that selects nothing (a renamed or mistyped
  # control) must fail, not report "0 FAILED".
  meta x35_empty_selection_fails 1 "no control selected" -- \
    "$ctl/run_controls.sh" --out "$out/x35-selftest" --only '^no_such_control$'
fi
if [ "${ALLOC_AUDIT_CONTROLS_SELFTEST:-}" != 1 ]; then
  # Self-test of this runner: a nested run whose only control has a seed
  # generator that fails must exit nonzero and record that control as FAILED.
  meta x32_failing_seed_generator_counted 1 "s_selftest_bad_anchor	FAIL	-	2	FAILED" -- \
    bash -c "ALLOC_AUDIT_CONTROLS_SELFTEST=1 '$ctl/run_controls.sh' --out '$out/x32-selftest' --only '^s_selftest_bad_anchor\$' >/dev/null 2>&1; rc=\$?; cat '$out/x32-selftest/CONTROLS.tsv'; exit \$rc"
fi

echo "== seeded violations in copies of the real cores (production allowlists) =="
AL="$audit/allowlist/x86_64-unknown-linux-gnu/release"
seed() { # seed <tag> <core> <seed> <expect> <req> <needles>
  local tag="$1" core="$2" s="$3" expect="$4" req="$5" needles="$6"
  [[ "$tag" =~ $only ]] || return 0
  local dst="$scratch/seeded/$tag"
  if ! python3 "$ctl/seed_core.py" "$core" "$s" "$dst" >"$out/$tag.seed.log" 2>&1; then
    # A seed that cannot be generated is a FAILED control, never a skipped one.
    n_sel=$((n_sel+1)); rm -f "$out/$tag.txt"
    echo "seed generation failed: $(tail -1 "$out/$tag.seed.log")" > "$out/$tag.stderr"
    check "$tag" 2 "$expect" "$req" "$needles" ""
    return
  fi
  cargo_ctl "$tag" "$expect" "$req" "$needles" "" "$dst/${core}_verus" "${core}_verus" "$AL/${core}_verus.toml" "$scratch/control-seeded-$core"
}
seed s0_tga_unseeded_copy tga none FAIL allowlist-reviewer "findings-other-than-unsigned-reviewer: 0"
for s in vec_zeroed with_capacity box_new push string_from format to_vec collect; do
  seed "s_tga_$s" tga "$s" FAIL unauthorized-alloc-path,allowlist-source-hash "crate::tga::alloc_output_guarded -> crate::tga::seeded_alloc -> "
done
seed s_dib_vec_zeroed dib vec_zeroed FAIL unauthorized-alloc-path,allowlist-source-hash "crate::dib::alloc_output_n -> crate::dib::seeded_alloc -> "
seed s_icns_push icns push FAIL unauthorized-alloc-path,allowlist-source-hash "crate::icns::seeded_alloc -> "
if [ "${ALLOC_AUDIT_CONTROLS_SELFTEST:-}" = 1 ]; then
  ALLOC_AUDIT_SEED_SELFTEST_BAD_ANCHOR=1 seed s_selftest_bad_anchor tga push FAIL unauthorized-alloc-path "crate::tga::seeded_alloc -> "
fi

rows=$(( $(wc -l < "$summary") - 1 ))
echo
echo "controls: $n_ok as expected, $n_bad FAILED, $rows result rows for $n_sel selected  (table: $summary)"
count_ok=1
[ "$n_sel" -ge 1 ] || { echo "no control selected (check --only); a run that tests nothing is a failure"; count_ok=0; }
[ "$rows" = "$n_sel" ] || { echo "COUNT MISMATCH: $n_sel controls selected but $rows results recorded"; count_ok=0; }
if [ "$only" = "." ] && [ "${ALLOC_AUDIT_CONTROLS_SELFTEST:-}" != 1 ] && [ "$rows" != "$EXPECTED_TOTAL" ]; then
  echo "COUNT MISMATCH: the full suite must record exactly $EXPECTED_TOTAL controls, recorded $rows"; count_ok=0
fi
[ $n_bad = 0 ] && [ $count_ok = 1 ]
