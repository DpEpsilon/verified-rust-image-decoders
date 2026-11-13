#!/bin/bash
# Regenerate the control fixtures (allowlists) in fixtures/. Each base fixture
# is drafted from a diagnostic run of its testcase with draft_allowlist.py and
# controls/decisions/<base>.toml; the variants are single-field edits of a base,
# made by fixture_variants.py. Run after a testcase or the tool's naming changes.
source "$(dirname "$0")/lib.sh"
build_tool && build_deps || exit 2
mkdir -p "$scratch/control-out" "$scratch/fixture-diag"
d="$scratch/fixture-diag"
base() { # base <name> <crate> <src> <opt> <roots|-> <extra args...>
  local name="$1" crate="$2" src="$3" opt="$4" roots="$5"; shift 5
  run_direct "$d/$name.txt" "$crate" "$src" - "$opt" 1 "$roots" "$@" >/dev/null 2>&1
  python3 "$audit/tools/draft_allowlist.py" "$d/$name.txt" "$ctl/decisions/$name.toml" "$ctl/fixtures/$name.toml" 2>"$d/$name.unhandled"
  # A second pass picks up root exclusions and drops leaves only they reached.
  run_direct "$d/$name.txt" "$crate" "$src" "$ctl/fixtures/$name.toml" "$opt" 1 "$roots" "$@" >/dev/null 2>&1
  python3 "$audit/tools/draft_allowlist.py" "$d/$name.txt" "$ctl/decisions/$name.toml" "$ctl/fixtures/$name.toml" 2>"$d/$name.unhandled"
  echo "fixture $name: $(grep -c '^\[\[entry' "$ctl/fixtures/$name.toml") entries; unhandled: $(grep -c '^    ' "$d/$name.unhandled")"
}
base c1_clean c1 "$TC/legacy/c1_clean.rs" 2 -
base c2_two_guards c2 "$TC/legacy/c2_two_guards.rs" 2 -
base n11_shared_instance n11 "$TC/legacy/n11_shared_instance.rs" 2 -
base n12_suffix_lookalike n12 "$TC/legacy/n12_suffix_lookalike.rs" 2 -
base n13_root_lookalike n13 "$TC/legacy/n13_root_lookalike.rs" 2 -
base n14_two_guards_plus_third n14 "$TC/legacy/n14_two_guards_plus_third.rs" 2 -
ENVS=(ALLOC_AUDIT_VSTD_VERSION=0.0.0-2026-11-01-0251); base c3_vstd_ok c3 "$TC/legacy/c3_vstd_linked.rs" 2 - -L /opt/verus --extern vstd=/opt/verus/libvstd.rlib; ENVS=()
base x1_extern_namesake x1 "$TC/x/x1_extern_namesake.rs" 2 - --extern guard="$deps/libguard.rlib"
base x14_inlined_helper x14 "$TC/x/x14_inlined_helper.rs" 2 -
base x23_star_alloc x23 "$TC/x/x23_star_alloc.rs" 2 -
ENVS=(ALLOC_AUDIT_X24=x24value); base x24_env x24 "$TC/x/x24_env.rs" 2 -; ENVS=()
base x12_helperdep helperdep "$TC/x/x12_helperdep.rs" 2 - -C metadata=one
base x12_foreign_crate x12 "$TC/x/x12_foreign_crate.rs" 2 - --extern helperdep="$scratch/control-out/libhelperdep.rlib"
base x12c_two_helpers x12c "$TC/x/x12c_two_helpers.rs" 2 - --extern helperdep="$scratch/control-out/libhelperdep.rlib" --extern hd2="$deps/libhelperdep2.rlib"
base x2_include_after_url x2 "$TC/x/x2_include_after_url.rs" 2 -
python3 "$ctl/fixture_variants.py" "$ctl/fixtures"
# x8: a real cargo package with a build script.
cargo_audit "$d/x8_declared.txt" "$ctl/cargo/x8" x8_real_build_script - "$scratch/control-x8"
python3 "$audit/tools/draft_allowlist.py" "$d/x8_declared.txt" "$ctl/decisions/x8_declared.toml" "$ctl/fixtures/x8_declared.toml" 2>"$d/x8_declared.unhandled"
python3 - "$ctl/fixtures/x8_declared.toml" <<'PY'
import re,sys
p=sys.argv[1]; t=open(p).read()
t=re.sub(r'^reviewer = .*$', 'reviewer = "A. Fixture-Reviewer (control fixture, not a review)"', t, count=1, flags=re.M)
open(p,'w').write(t)
PY
echo "fixture x8_declared: $(grep -c '^\[\[entry' "$ctl/fixtures/x8_declared.toml") entries"
