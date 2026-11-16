#!/bin/bash
# alloc-delta source lint for the GIF core (alloc-delta SPEC.md §6 "Source lint";
# GIF SPEC.md G18). Adapted from icns/tools/audit_alloc.sh.
#
# THIS IS A FALSIFICATION GATE, NOT ENFORCEMENT. Verus cannot see allocations, so
# this check lists allocating constructs in production exec code and fails on the
# ones it recognises (it cannot prove the list is exhaustive; that is the
# production call-graph audit's job, alloc-delta SPEC.md §6). It also pins the
# guard's two text files and requires the guard's reservation call sites to be
# exactly GIF's three sites.
#
# The scan covers every production module under `gif_verus/src` except
# `alloc_guard/` (the guard IS the allocation; decision D6). GIF has no
# `Reserved<T>` site, so the scan must be clean both with and without
# `--allow-reserved`: no `.push(`/`.into_vec(` at all outside the guard.
#
# Reservation call sites (G18), all in `gif.rs`, all byte-counted `try_zeroed`:
#   try_zeroed(AllocSite::Canvas, ..)    once (Frames::open: decode_still and the constructor)
#   try_zeroed(AllocSite::LzwTable, ..)  once (same function, after Canvas)
#   try_zeroed(AllocSite::Saved, ..)     once (the first disposal-3 frame)
#
# Usage: tools/audit_alloc.sh [--self-test]
#   --self-test  also run the scanner's own positive/negative controls and two
#                negative gate controls (an exec `v.push(x)` and an exec `vec!`)
#
# The production call-graph audit (alloc-delta/audit/run_audit.sh) has no GIF
# configuration yet; its integration comes in a later milestone.
set -u
root="$(cd "$(dirname "$0")/.." && pwd)"
srcdir="$root/gif_verus/src"
scanner="$root/tools/alloc_scan.py"
status=0

# D6: `mod.rs` is byte-identical across the cores and pinned to the reviewed
# cross-core hash; `sites.rs` is GIF's own.
#
# PIN TO UPDATE when `alloc_guard/sites.rs` changes: EXPECTED_SITES_SHA below is
# the only place the `sites.rs` hash appears. A deliberate edit of `sites.rs`
# fails the gate until this one line is updated to the new `sha256sum`.
EXPECTED_MOD_SHA=26e4c832d2df80b782d157e68ade31066cc82ca2774b4c714a6df0f4edaf86b7
EXPECTED_SITES_SHA=ba8f9fe5719341171767e5b932e1b8db973c8db487efa668ddfd138c2784b133

mapfile -t files < <(find "$srcdir" -name '*.rs' -not -path "$srcdir/alloc_guard/*" | sort)
if [ "${#files[@]}" -eq 0 ]; then echo "no production modules found under $srcdir"; exit 1; fi
names=()
for f in "${files[@]}"; do names+=("${f#"$srcdir"/}"); done
echo "scanned: ${names[*]} (alloc_guard/ excluded)"

for mode in --allow-reserved ""; do
  findings=$(python3 "$scanner" $mode "${files[@]}" 2>&1)
  if [ $? -ne 0 ]; then
    echo "allocating construct(s) in exec code (scan ${mode:-without suppression}):"
    printf '%s\n' "$findings"
    status=1
  else
    echo "alloc_scan ${mode:-(no suppression)}: clean"
  fi
done

# The reservation call sites, by site name, comments stripped. A site's
# argument may sit on the next line (the logged `open` wraps its calls), so the
# pattern tolerates whitespace up to the site name.
flat() { sed -E 's#//.*$##' "$srcdir/gif.rs" | tr '\n' ' '; }
n_canvas=$(flat | grep -oE 'try_zeroed\( *AllocSite::Canvas' | wc -l)
n_saved=$(flat | grep -oE 'try_zeroed\( *AllocSite::Saved' | wc -l)
n_table=$(flat | grep -oE 'try_zeroed\( *AllocSite::LzwTable' | wc -l)
n_zeroed_all=$(cat "${files[@]}" | sed -E 's#//.*$##' | grep -cE '\btry_zeroed\(')
# M3: `reserve` (the verified G18 prefix) and the still-unverified `open` each
# name Canvas and LzwTable, and `image` names Saved once: 5 call expressions.
# Each site name appears at most twice (once per function), and `Saved` once.
echo "guard reservation call sites: Canvas $n_canvas, LzwTable $n_table, Saved $n_saved (expected 2, 2, 1); try_zeroed calls in gif.rs: $n_zeroed_all (expected 5)"
if [ "$n_canvas" != 2 ] || [ "$n_table" != 2 ] || [ "$n_saved" != 1 ] || [ "$n_zeroed_all" != 5 ]; then status=1; fi

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
  tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT
  printf 'fn leaky(v: &mut Vec<u8>, n: usize) {\n    let mut i = 0;\n    while i < n {\n        v.push(0u8);\n        i += 1;\n    }\n}\n' > "$tmp/unguarded.rs"
  if python3 "$scanner" "$tmp/unguarded.rs" > /dev/null 2>&1; then
    echo "negative gate self-test FAILED: unguarded exec v.push(x) was not flagged"; status=1
  else
    echo "negative gate self-test OK: unguarded exec v.push(x) flagged"
  fi
  printf 'fn alloc(n: usize) -> Vec<u8> {\n    vec![0u8; n]\n}\n' > "$tmp/vec.rs"
  if python3 "$scanner" "$tmp/vec.rs" > /dev/null 2>&1; then
    echo "negative gate self-test FAILED: exec vec! was not flagged"; status=1
  else
    echo "negative gate self-test OK: exec vec! flagged"
  fi
fi

if [ "$status" = 0 ]; then echo "OK"; exit 0; else echo "FAIL"; exit 1; fi
