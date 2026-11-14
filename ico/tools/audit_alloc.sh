#!/bin/bash
# alloc-delta source lint (SPEC.md §6 "Source lint"; THEOREMS-ALLOC-DELTA.md §4,
# A-ICO-1/A-ICO-2).
#
# THIS IS A FALSIFICATION GATE, NOT ENFORCEMENT. Verus cannot see allocations, so
# this check lists allocating constructs in production exec code and fails on the
# ones it recognises (it cannot prove the list is exhaustive; that is the
# production call-graph audit's job, SPEC.md §6, which is not built). It also
# pins the guard's two text files and requires exactly one `try_with_room` call
# site, one `import_dib` call site, and zero `vec!` outside the guard.
#
# The scan covers every production module under `ico_verus/src` except
# `alloc_guard/` (the guard IS the allocation; SPEC.md §3, decision D6). `.push(`
# is matched with ghost/proof/spec regions stripped by `tools/alloc_scan.py`, so
# `Seq::push` in a spec fn, an `ensures` clause or an `assert` is not flagged
# while an exec `.push(` outside the guard is. A hit on any allocating construct,
# or on `handle_alloc_error` / `process::abort`, is a finding.
#
# Usage: tools/audit_alloc.sh [--self-test]
#   --self-test  also run the scanner's own positive/negative controls
set -u
root="$(cd "$(dirname "$0")/.." && pwd)"
srcdir="$root/ico_verus/src"
scanner="$root/tools/alloc_scan.py"
status=0

# D6: the guard text is byte-identical across the four cores. `mod.rs` is pinned
# to the reviewed cross-core hash; `sites.rs` to ICO's own (it carries
# `import_dib`, `try_with_room` and the `global layout` declaration).
EXPECTED_MOD_SHA=26e4c832d2df80b782d157e68ade31066cc82ca2774b4c714a6df0f4edaf86b7
EXPECTED_SITES_SHA=5918505c88282574c693481a557f3b0abd0c1022d4a3e11a2137891efd59bdcf

# Production modules: every `src/**/*.rs` except `alloc_guard/`.
mapfile -t files < <(find "$srcdir" -name '*.rs' -not -path "$srcdir/alloc_guard/*" | sort)
if [ "${#files[@]}" -eq 0 ]; then echo "no production modules found under $srcdir"; exit 1; fi
names=()
for f in "${files[@]}"; do names+=("${f#"$srcdir"/}"); done
echo "scanned: ${names[*]} (alloc_guard/ excluded)"

# The syntax-aware scan: allocating constructs and exec `.push(`.
if ! python3 "$scanner" --allow-reserved "${files[@]}"; then status=1; fi

# Exactly one guard reservation call site on the production path,
# `try_with_room`, and exactly one `import_dib` call site, both in `ico.rs`.
n_room=$(sed -E 's#//.*$##' "$srcdir/ico.rs" | grep -cE 'alloc_guard::sites::try_with_room(::<[^>]*>)?\(')
n_guard_calls=$(sed -E 's#//.*$##' "$srcdir/ico.rs" | grep -cE 'alloc_guard::sites::try_(zeroed|copy|with_room)(::<[^>]*>)?\(')
n_import=$(sed -E 's#//.*$##' "$srcdir/ico.rs" | grep -cE 'alloc_guard::sites::import_dib\(')
n_dib=$(sed -E 's#//.*$##' "$srcdir/ico.rs" | grep -cE 'dib_verus::try_decode_with_steps_logged\(')
n_vec=$(python3 "$scanner" --count-vec "$srcdir/ico.rs" 2>/dev/null || echo "?")
echo "vec! in ico.rs: $n_vec (expected 0, the guard replaces the entry-list reservation)"
echo "guard reservation call sites: $n_guard_calls (try_with_room: $n_room; expected exactly 1)"
echo "import_dib call sites: $n_import (expected exactly 1)"
echo "dib_verus::try_decode_with_steps_logged call sites: $n_dib (expected exactly 1)"
if [ "$n_room" != 1 ] || [ "$n_guard_calls" != 1 ] || [ "$n_import" != 1 ] || [ "$n_dib" != 1 ]; then status=1; fi

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
  echo "--- production call-graph audit: alloc-delta/audit/run_audit.sh ico ---"
  if ! "$prod" ico; then echo "production audit: FAIL (see the run directory printed above)"; status=1; fi
fi

if [ "$status" = 0 ]; then echo "OK"; exit 0; else echo "FAIL"; exit 1; fi
