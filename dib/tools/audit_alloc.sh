#!/bin/bash
# T3 source-level review, automated (THEOREMS.md T3 says a source-level review excludes clones,
# growing vectors, formatting, recursion, I/O). Verus cannot see allocations, so this check is the
# enforcement for 'no allocation other than the single output buffer'. Usage: tools/audit_alloc.sh [src/dib.rs]
f="${1:-$(dirname "$0")/../dib_verus/src/dib.rs}"
# strip // comments, then count allocating constructs
body=$(sed -E 's#//.*$##' "$f")
bad=$(printf '%s\n' "$body" | grep -nE 'Vec::new|Vec::with_capacity|\.to_vec\(|\.clone\(|\.to_owned\(|String::|format!|Box::new|Vec::[a-z_]+|from_iter|iter::repeat|repeat_n\(|\.take\(|\.into_boxed|\.resize\(|\.reserve\(|\.extend\(|\.collect|Rc::|Arc::' | grep -v '^\s*$')
n=$(printf '%s\n' "$body" | grep -cE '\bvec!\[')
echo "vec! allocations: $n (expected exactly 1, in alloc_output)"
if [ -n "$bad" ]; then echo "other allocating constructs:"; echo "$bad"; fi
if [ "$n" = 1 ] && [ -z "$bad" ]; then echo "OK"; exit 0; else echo "FAIL"; exit 1; fi
