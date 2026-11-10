#!/bin/bash
# T10 source-level review, automated.  Verus cannot see allocations, so this
# check is the enforcement for the manifest's allocation-call bound: the
# header-error paths request no entry-list allocation, and after the
# directory/count/table guards the code requests exactly one bounded reservation
# for at most 1,024 records.  The decode wrapper requests no allocation.  A ghost
# call counter cannot detect an added, uncharged `vec!`/clone/`collect`/format,
# so this source audit is the enforcement, not the ghost counter.
#
# Exactly one allowed site: `Vec::with_capacity(header.count)` in `parse_tracked`
# (bounded by `count <= 1,024`).  Everything else that allocates is a finding.
#
# Usage: tools/audit_alloc.sh [path/to/ico.rs]
#   The argument lets the check run on a mutated copy and confirm it fails.
f="${1:-$(dirname "$0")/../ico_verus/src/ico.rs}"
if [ ! -f "$f" ]; then echo "no such file: $f"; exit 2; fi
# Strip line comments: the doc comments name the allowed reservation, which must
# not be counted as a real occurrence.
body=$(sed -E 's#//.*$##' "$f")
nres=$(printf '%s\n' "$body" | grep -cE 'Vec::with_capacity')
nv=$(printf '%s\n' "$body" | grep -cE '\bvec!\[')
# Any `Vec::` other than the single allowed `Vec::with_capacity`, plus every other
# allocating construct (clones, growing vectors, formatting, boxed/refcounted).
bad=$(printf '%s\n' "$body" | grep -nE 'Vec::|\.to_vec\(|\.clone\(|\.to_owned\(|String::|format!|Box::new|from_iter|iter::repeat|repeat_n\(|\.into_boxed|\.resize\(|\.reserve\(|\.extend\(|\.collect|Rc::|Arc::|to_string\(' | grep -vE 'Vec::with_capacity' | grep -v '^\s*$')
echo "Vec::with_capacity reservations: $nres (expected exactly 1, in parse_tracked)"
echo "vec! allocations: $nv (expected 0)"
if [ -n "$bad" ]; then echo "other allocating constructs:"; echo "$bad"; fi
if [ "$nres" = 1 ] && [ "$nv" = 0 ] && [ -z "$bad" ]; then echo "OK"; exit 0; else echo "FAIL"; exit 1; fi
