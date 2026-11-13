#!/bin/bash
# alloc-delta source lint (SPEC.md §6 "Source lint"; THEOREMS-ALLOC-DELTA.md §2,
# A-TGA-1/A-TGA-3).
#
# THIS IS A FALSIFICATION GATE, NOT ENFORCEMENT. Verus cannot see allocations, so
# this check lists allocating constructs in production exec code and fails on the
# ones it recognises (it cannot prove the list is exhaustive; that is the
# production call-graph audit's job, SPEC.md §6, which is not built). It also
# pins the guard's two text files and requires exactly one `try_zeroed` call
# site and zero `vec!` outside the guard.
#
# The scan covers every production module under `tga_verus/src` except
# `alloc_guard/` (the guard IS the allocation; SPEC.md §3, decision D6) and
# `#[cfg(test)]` files. `.push(` is matched with ghost/proof/spec regions
# stripped by `tools/alloc_scan.py`, so `Seq::push` in a spec fn, an `ensures`
# clause or an `assert` is not flagged while an exec `.push(` outside the guard
# is. A hit on any allocating construct, or on `handle_alloc_error` /
# `process::abort`, is a finding.
#
# Usage: tools/audit_alloc.sh [--self-test]
#   --self-test  also run the scanner's own positive/negative controls
set -u
root="$(cd "$(dirname "$0")/.." && pwd)"
srcdir="$root/tga_verus/src"
scanner="$root/tools/alloc_scan.py"
status=0

# D6: the guard `mod.rs` is byte-identical across the four cores (the `use
# alloc::vec::Vec;` line is part of the common text, ERRATA E1; the fill is
# `resize(n, 0u8)` after `try_reserve_exact(n)`, ERRATA E6, trusted item A-STD-4).
# Both guard files are pinned: `mod.rs` to the reviewed cross-core hash,
# `sites.rs` to TGA's own.
EXPECTED_MOD_SHA=26e4c832d2df80b782d157e68ade31066cc82ca2774b4c714a6df0f4edaf86b7
EXPECTED_SITES_SHA=58e1fb285124557080dde6a6bc01c0d6e839cd5402b804b08b240de43750c885

# Production modules: every `src/**/*.rs` except `alloc_guard/` and cfg(test)
# files (`policy_tests.rs` is `#[cfg(test)]` and never in the shipped artifact).
mapfile -t files < <(find "$srcdir" -name '*.rs' -not -path "$srcdir/alloc_guard/*" -not -name 'policy_tests.rs' | sort)
if [ "${#files[@]}" -eq 0 ]; then echo "no production modules found under $srcdir"; exit 1; fi
names=()
for f in "${files[@]}"; do names+=("${f#"$srcdir"/}"); done
echo "scanned: ${names[*]} (alloc_guard/ and cfg(test) excluded)"

# The syntax-aware scan: allocating constructs and exec `.push(`.
if ! python3 "$scanner" "${files[@]}"; then status=1; fi

# Exactly one guard reservation call site on the production path, `try_zeroed`.
n_zeroed=$(sed -E 's#//.*$##' "$srcdir/tga.rs" | grep -cE 'alloc_guard::try_zeroed\(')
n_guard_calls=$(sed -E 's#//.*$##' "$srcdir/tga.rs" | grep -cE 'alloc_guard::try_(zeroed|copy|with_room)\(')
n_vec=$(python3 "$scanner" --count-vec "$srcdir/tga.rs" 2>/dev/null || echo "?")
echo "vec! in tga.rs: $n_vec (expected 0, the guard replaces the 4N output buffer)"
echo "guard reservation call sites: $n_guard_calls (try_zeroed: $n_zeroed; expected exactly 1)"
if [ "$n_zeroed" != 1 ] || [ "$n_guard_calls" != 1 ]; then status=1; fi

# The guard module is exempt from the scan, but its text is hash-pinned (D6).
for pair in "mod.rs:$EXPECTED_MOD_SHA" "sites.rs:$EXPECTED_SITES_SHA"; do
  name="${pair%%:*}"; want="${pair##*:}"
  f="$srcdir/alloc_guard/$name"
  if [ -f "$f" ]; then
    got=$(sha256sum "$f" | awk '{print $1}')
    if [ "$got" != "$want" ]; then
      echo "alloc_guard/$name hash mismatch: got $got expected $want"; status=1
    else
      echo "alloc_guard/$name hash OK"
    fi
  else
    echo "alloc_guard/$name missing"; status=1
  fi
done

if [ "${1:-}" = "--self-test" ]; then
  echo "--- scanner self-test ---"
  if ! python3 "$scanner" --self-test; then status=1; fi
  echo "--- negative gate self-test: a temporary unguarded exec v.push(x) must fail ---"
  tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT
  cat > "$tmp/unguarded.rs" <<'EOF'
verus! {
fn leaky(v: &mut Vec<u8>, n: usize)
    ensures final(v)@.len() == old(v)@.len() + n,
{
    let mut i: usize = 0;
    while i < n
        invariant 0 <= i <= n,
        decreases n - i,
    {
        v.push(0u8);
        i = i + 1;
    }
}
} // verus!
EOF
  if python3 "$scanner" "$tmp/unguarded.rs" > /dev/null 2>&1; then
    echo "negative gate self-test FAILED: unguarded exec v.push(x) was not flagged"; status=1
  else
    echo "negative gate self-test OK: unguarded exec v.push(x) flagged"
  fi
fi

# The production transitive call-graph audit (alloc-delta/audit, SPEC §6), when
# it is present and this script audits the real tree (no source-path argument;
# mutation runners pass one). The source lint above stays: it is a separate,
# falsification-only gate. The production audit's verdict is its own.
prod="$root/../alloc-delta/audit/run_audit.sh"
if [ -x "$prod" ] && { [ $# -eq 0 ] || [ "${1:-}" = "--self-test" ]; }; then
  echo "--- production call-graph audit: alloc-delta/audit/run_audit.sh tga ---"
  if ! "$prod" tga; then echo "production audit: FAIL (see the run directory printed above)"; status=1; fi
fi

if [ "$status" = 0 ]; then echo "OK"; exit 0; else echo "FAIL"; exit 1; fi
