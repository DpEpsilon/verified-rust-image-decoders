#!/bin/bash
# alloc-delta source lint for the ICNS core (SPEC.md §6 "Source lint";
# THEOREMS-ALLOC-DELTA.md §2, A-ICNS-1..6).
#
# THIS IS A FALSIFICATION GATE, NOT ENFORCEMENT. Verus cannot see allocations, so
# this check lists allocating constructs in production exec code and fails on the
# ones it recognises (it cannot prove the list is exhaustive; that is the
# production call-graph audit's job, SPEC.md §6, which is not built). It also
# pins the guard's two text files and requires the guard's reservation call
# sites to be exactly the six A-ICNS sites.
#
# The scan covers every production module under `icns_verus/src` except
# `alloc_guard/` (the guard IS the allocation; SPEC.md §3, decision D6) and
# `#[cfg(test)]` files. `.push(` is matched with ghost/proof/spec regions and
# loop `invariant` clauses stripped by `tools/alloc_scan.py`, so `Seq::push` in a
# spec fn, an `ensures` clause, an `assert` or an `invariant` is not flagged
# while an exec `.push(` outside the guard is. A hit on any allocating construct,
# or on `handle_alloc_error` / `process::abort`, is a finding.
#
# Two guard-mediated calls are permitted, and only where they sit: `res.push(entry)`
# and `res.into_vec()` on the `Reserved<Entry>` that the same function body binds
# from `try_with_room(..)` (allocation-free by A-STD-2, the capacity invariant;
# `into_vec` moves the `Vec` out). The scanner's `--allow-reserved` mode blanks
# exactly those two call shapes inside such a function body and nowhere else, so
# the gate (1) requires the `--allow-reserved` run to be clean and (2) requires
# the run without the flag to show exactly one `.push(` and one `.into_vec(`
# (the guarded pair). Any other `.push(`/`.into_vec(`, including a `v.push(x)`
# that replaced the guarded one, or a `res` Vec outside a function that binds it
# from the guard, fails. The textual match cannot prove the receiver's type: this
# remains a falsification lint, not enforcement.
#
# Usage: tools/audit_alloc.sh [--self-test]
#   --self-test  also run the scanner's own positive/negative controls and a
#                negative gate control (an unguarded exec `v.push(x)` must fail)
set -u
root="$(cd "$(dirname "$0")/.." && pwd)"
srcdir="$root/icns_verus/src"
scanner="$root/tools/alloc_scan.py"
status=0

# D6: the guard `mod.rs` is byte-identical across the four cores (the `use
# alloc::vec::Vec;` line is part of the common text, ERRATA E1; the fill is
# `resize(n, 0u8)` after `try_reserve_exact(n)`, ERRATA E6, trusted item A-STD-4).
# `mod.rs` is pinned to the reviewed cross-core hash; `sites.rs` is ICNS's own.
#
# PIN TO UPDATE when `alloc_guard/sites.rs` changes: EXPECTED_SITES_SHA below is
# the only place the `sites.rs` hash appears (old/new values are recorded in
# NOTES.md). A deliberate edit of `sites.rs` fails the gate until this one line is
# updated to the new `sha256sum` of the file.
EXPECTED_MOD_SHA=26e4c832d2df80b782d157e68ade31066cc82ca2774b4c714a6df0f4edaf86b7
EXPECTED_SITES_SHA=adcc09bed170a230216248e1c3898dcc1b32af392b45032352ed717044c88c41

# Production modules: every `src/**/*.rs` except `alloc_guard/`.
mapfile -t files < <(find "$srcdir" -name '*.rs' -not -path "$srcdir/alloc_guard/*" | sort)
if [ "${#files[@]}" -eq 0 ]; then echo "no production modules found under $srcdir"; exit 1; fi
names=()
for f in "${files[@]}"; do names+=("${f#"$srcdir"/}"); done
echo "scanned: ${names[*]} (alloc_guard/ excluded)"

# The syntax-aware scan, scoped suppression: must be clean.
findings=$(python3 "$scanner" --allow-reserved "${files[@]}" 2>&1)
scan_rc=$?
if [ "$scan_rc" -ne 0 ]; then
  echo "unexpected allocating construct(s) in exec code (guard-scoped suppression applied):"
  printf '%s\n' "$findings"
  status=1
else
  echo "alloc_scan (--allow-reserved): clean"
fi

# The same scan without the suppression must show exactly the guarded pair.
raw=$(python3 "$scanner" "${files[@]}" 2>&1)
n_push=$(printf '%s\n' "$raw" | grep -cE '^  [0-9]+: \.push\($')
n_into=$(printf '%s\n' "$raw" | grep -cE '^  [0-9]+: \.into_vec\($')
n_other=$(printf '%s\n' "$raw" | grep -E '^  [0-9]+: ' | grep -vE '^  [0-9]+: \.push\($' | grep -cvE '^  [0-9]+: \.into_vec\($')
if [ "$n_push" != 1 ] || [ "$n_into" != 1 ] || [ "$n_other" != 0 ]; then
  echo "unsuppressed scan: push=$n_push into_vec=$n_into other=$n_other (expected 1, 1, 0)"
  status=1
else
  echo "unsuppressed scan: 1 push, 1 into_vec (the entry-list fill), nothing else"
fi

# The seven A-ICNS reservation call sites, by site name. `try_with_room` once
# (EntryList), `try_zeroed` five times (three channels, RgbOut, CombineOut) and
# `try_copy` once (MaskData).
n_room=$(sed -E 's#//.*$##' "$srcdir/icns.rs" | grep -cE 'alloc_guard::sites::try_with_room::<Entry>\(')
n_zeroed=$(sed -E 's#//.*$##' "$srcdir/icns.rs" | grep -cE 'alloc_guard::try_zeroed\(')
n_copy=$(sed -E 's#//.*$##' "$srcdir/icns.rs" | grep -cE 'alloc_guard::sites::try_copy\(')
n_guard_calls=$((n_room + n_zeroed + n_copy))
echo "guard reservation call sites: $n_guard_calls (try_with_room: $n_room, try_zeroed: $n_zeroed, try_copy: $n_copy; expected 1, 5, 1)"
if [ "$n_room" != 1 ] || [ "$n_zeroed" != 5 ] || [ "$n_copy" != 1 ]; then status=1; fi

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
  echo "--- negative gate self-test: an exec vec! must fail ---"
  cat > "$tmp/vec.rs" <<'EOF'
verus! {
fn alloc(n: usize) -> (v: Vec<u8>) {
    let v: Vec<u8> = vec![0u8; n];
    v
}
} // verus!
EOF
  if python3 "$scanner" "$tmp/vec.rs" > /dev/null 2>&1; then
    echo "negative gate self-test FAILED: exec vec! was not flagged"; status=1
  else
    echo "negative gate self-test OK: exec vec! flagged"
  fi
fi

# The production transitive call-graph audit (alloc-delta/audit, SPEC §6), when
# it is present and this script audits the real tree (no source-path argument;
# mutation runners pass one). The source lint above stays: it is a separate,
# falsification-only gate. The production audit's verdict is its own.
prod="$root/../alloc-delta/audit/run_audit.sh"
if [ -x "$prod" ] && { [ $# -eq 0 ] || [ "${1:-}" = "--self-test" ]; }; then
  echo "--- production call-graph audit: alloc-delta/audit/run_audit.sh icns ---"
  if ! "$prod" icns; then echo "production audit: FAIL (see the run directory printed above)"; status=1; fi
fi

if [ "$status" = 0 ]; then echo "OK"; exit 0; else echo "FAIL"; exit 1; fi
