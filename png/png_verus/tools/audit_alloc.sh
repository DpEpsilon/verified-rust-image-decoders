#!/bin/bash
# alloc-delta source lint for the PNG core (alloc-delta SPEC.md §6 "Source
# lint"; PNG SPEC P20). Adapted from ../../../gif/tools/audit_alloc.sh, and it
# reuses GIF's syntax-aware scanner `../../../gif/tools/alloc_scan.py` in place
# (not a copy).
#
# THIS IS A FALSIFICATION GATE, NOT ENFORCEMENT. Verus cannot see allocations, so
# this check lists allocating constructs in production exec code and fails on the
# ones it recognises. It also pins the guard's two text files and requires the
# reservation call sites to be exactly PNG's:
#
#   try_zeroed(AllocSite::Raw, ..)        once (png.rs reserve_image: stills and Frames::try_new)
#   try_zeroed(AllocSite::Output, ..)     once (same function, after Raw)
#   try_zeroed(AllocSite::Saved, ..)      once (Frames::try_next, step S)
#   try_zeroed(AllocSite::IccWindow, ..)  once (icc_impl, the measuring ring)
#   try_zeroed(AllocSite::Icc, ..)        once (icc_impl, m >= 1)
#   try_empty(AllocSite::Icc, ..)         once (icc_impl, m = 0; D8)
#
# PNG has no `Reserved<T>` site, so both scanner modes must be clean: no exec
# `.push(`/`.into_vec(` at all outside the guard.
#
# Usage: tools/audit_alloc.sh [--self-test]
set -u
root="$(cd "$(dirname "$0")/.." && pwd -P)"
srcdir="$root/src"
scanner="$(cd "$root/../../gif/tools" && pwd -P)/alloc_scan.py"
status=0

# D6: `mod.rs` is byte-identical across the cores; `sites.rs` is PNG's own.
# PIN TO UPDATE when `alloc_guard/sites.rs` changes deliberately.
EXPECTED_MOD_SHA=add634bc74ff7feed568c90d5cc8c504538b4d4b80420e7fc9228fceecc2a137
EXPECTED_SITES_SHA=dcd747cf7721c33ff00fbffd83ca8838d12803f99c2fb896b8077f25f5d8a281

if ! inventory=$(python3 -I "$scanner" --production-files "$srcdir"); then
  echo "production module inventory failed"; exit 1
fi
if [ -z "$inventory" ]; then echo "no production modules found under $srcdir"; exit 1; fi
mapfile -t files <<< "$inventory"
names=()
for f in "${files[@]}"; do names+=("${f#"$srcdir"/}"); done
echo "scanned: ${names[*]} (alloc_guard/ and cfg(test)-only module subtrees excluded)"

for mode in --allow-reserved ""; do
  findings=$(python3 -I "$scanner" $mode "${files[@]}" 2>&1)
  if [ $? -ne 0 ]; then
    echo "allocating construct(s) in exec code (scan ${mode:-without suppression}):"
    printf '%s\n' "$findings"
    status=1
  else
    echo "alloc_scan ${mode:-(no suppression)}: clean"
  fi
done

flat() { sed -E 's#//.*$##' "$srcdir/png.rs" | tr '\n' ' '; }
count() { flat | grep -oE "$1" | wc -l; }
n_raw=$(count 'try_zeroed\( *AllocSite::Raw\b')
n_out=$(count 'try_zeroed\( *AllocSite::Output\b')
n_saved=$(count 'try_zeroed\( *AllocSite::Saved\b')
n_win=$(count 'try_zeroed\( *AllocSite::IccWindow\b')
n_icc=$(count 'try_zeroed\( *AllocSite::Icc\b')
n_empty=$(count 'try_empty\( *AllocSite::Icc\b')
n_zeroed_all=$(cat "${files[@]}" | sed -E 's#//.*$##' | grep -oE '\btry_zeroed\(' | wc -l)
n_empty_all=$(cat "${files[@]}" | sed -E 's#//.*$##' | grep -oE '\btry_empty\(' | wc -l)
echo "guard reservation call sites: Raw $n_raw, Output $n_out, Saved $n_saved, IccWindow $n_win, Icc $n_icc, empty Icc $n_empty (expected 1 each); try_zeroed calls in production: $n_zeroed_all (expected 5), try_empty: $n_empty_all (expected 1)"
if [ "$n_raw" != 1 ] || [ "$n_out" != 1 ] || [ "$n_saved" != 1 ] || [ "$n_win" != 1 ] || [ "$n_icc" != 1 ] \
   || [ "$n_empty" != 1 ] || [ "$n_zeroed_all" != 5 ] || [ "$n_empty_all" != 1 ]; then status=1; fi

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
  if ! python3 -I "$scanner" --self-test; then status=1; fi
  if ! "$root/tools/lint_caps.sh" --self-test; then status=1; fi
  mkdir -p "$root/runs"
  tmp="$(mktemp -d "$root/runs/alloc-lint.XXXXXX")"; trap 'rm -rf "$tmp"' EXIT
  printf 'fn leaky(v: &mut Vec<u8>) {\n    v.push(0u8);\n}\n' > "$tmp/unguarded.rs"
  python3 -I "$scanner" "$tmp/unguarded.rs" > "$tmp/push.out" 2>&1
  rc=$?
  if [ "$rc" = 1 ] && grep -qE '^  2: \.push\($' "$tmp/push.out"; then
    echo "negative gate self-test OK: exec v.push(x) flagged"
  else
    echo "negative gate self-test FAILED: expected an exec v.push(x) finding (exit $rc)"; status=1
  fi
  printf 'fn alloc(n: usize) -> Vec<u8> {\n    vec![0u8; n]\n}\n' > "$tmp/vec.rs"
  python3 -I "$scanner" "$tmp/vec.rs" > "$tmp/vec.out" 2>&1
  rc=$?
  if [ "$rc" = 1 ] && grep -qE '^  2: vec!\[$' "$tmp/vec.out"; then
    echo "negative gate self-test OK: exec vec! flagged"
  else
    echo "negative gate self-test FAILED: expected an exec vec! finding (exit $rc)"; status=1
  fi
fi

if [ "$status" = 0 ]; then echo "OK"; exit 0; else echo "FAIL"; exit 1; fi
