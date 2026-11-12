#!/bin/bash
# Obligation A (THEOREMS-POLICY-DELTA.md): exactly one input-dependent
# allocation, the 4N-byte output buffer, on success; none on a header error; no
# other input-dependent scratch allocation. Verus cannot see allocations, so
# this source-level audit is the enforcement, as in dib/tools/audit_alloc.sh.
# Usage: tools/audit_alloc.sh [src/tga.rs]
f="${1:-$(dirname "$0")/../tga_verus/src/tga.rs}"
body=$(sed -E 's#//.*$##' "$f")
bad=$(printf '%s\n' "$body" | grep -nE 'Vec::new|Vec::with_capacity|\.to_vec\(|\.clone\(|\.to_owned\(|String::|format!|Box::new|Vec::[a-z_]+|from_iter|iter::repeat|repeat_n\(|\.take\(|\.into_boxed|\.resize\(|\.reserve\(|\.extend\(|\.collect|Rc::|Arc::' | grep -v '^\s*$')
n=$(printf '%s\n' "$body" | grep -cE '\bvec!\[')
echo "vec! allocations: $n (expected exactly 1, the 4N output buffer)"
if [ -n "$bad" ]; then echo "other allocating constructs:"; echo "$bad"; fi
if [ "$n" = 1 ] && [ -z "$bad" ]; then echo "OK"; exit 0; else echo "FAIL"; exit 1; fi
