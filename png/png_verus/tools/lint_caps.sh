#!/bin/bash
# P1 symbolic-cap source lint for png_verus (PLAN.md: every limit is a
# `pub const` in limits.rs and everything else names it). Adapted from
# ../../../gif/tools/lint_caps.sh. Scans executable code, comments and (later)
# models and proofs under png_verus/src. It recognises the configured cap
# values read from limits.rs, the derived values MAX_OUTPUT/4, 2·MAX_OUTPUT and
# 5·MAX_OUTPUT/2 + 2, their decimal/hex spellings with digit separators, the
# shifts and powers 1 << k / 2^k for the caps' exponents, and `N * 1024 * 1024`
# products. It is a grep-based falsification gate, not a Rust parser.
# Use i32::MAX/u32::MAX for wire and type bounds rather than cap numerals.
# The only exemptions are limits.rs and the hash-pinned alloc_guard/mod.rs,
# which owns MAX_BYTES (tools/audit_alloc.sh pins its hash).
# Usage: lint_caps.sh [--self-test] [--src /absolute/path/to/src]
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd -P)"
src="$root/src"
self_test=0
while [ "$#" -gt 0 ]; do
  case "$1" in
    --self-test) self_test=1; shift ;;
    --src) src="$2"; shift 2 ;;
    *) printf 'unknown argument: %s\n' "$1" >&2; exit 2 ;;
  esac
done
[[ "$src" = /* && -f "$src/limits.rs" ]] || { printf 'missing absolute source directory: %s\n' "$src" >&2; exit 2; }

digits() {
  local s="$1" out='' i
  for ((i=0; i<${#s}; i++)); do out+="${s:i:1}_*"; done
  printf '%s' "$out"
}
alternatives=''
exponents=''
add_number() {
  local n="$1" dec hex e
  # Small values collide with ordinary literals (MAX_FRAMES's lower bound
  # aside, every PNG cap is at least 2^16).
  if ((n < 32768)); then return; fi
  dec="$(digits "$n")"
  printf -v hex '%x' "$n"
  hex="$(digits "$hex")"
  alternatives+="${alternatives:+|}${dec}|0[xX]0*${hex}"
  # Record k when n is 2^k or 2^k - 1.
  for e in $(seq 15 62); do
    if (( n == (1 << e) || n == (1 << e) - 1 )); then exponents+="${exponents:+|}$e"; fi
  done
}
values=()
while IFS= read -r line; do
  expr="${line#*=}"; expr="${expr%%;*}"; expr="${expr//_/}"
  if [[ "$expr" =~ ^[[:space:]0-9xXa-fA-F\<\>\(\)\+\*/-]+$ ]]; then
    n=$((expr)); values+=("$n"); add_number "$n"
  else
    printf 'unrecognised cap expression: %s\n' "$line" >&2; exit 2
  fi
done < <(grep -E '^pub const MAX_(INPUT|DIMENSION|OUTPUT|FRAMES|TOTAL_PIXELS|ICC):' "$src/limits.rs")
[ "${#values[@]}" -eq 6 ] || { printf 'expected six caps in limits.rs, found %s\n' "${#values[@]}" >&2; exit 2; }
out=$(grep -E '^pub const MAX_OUTPUT:' "$src/limits.rs"); out="${out#*=}"; out="${out%%;*}"; out=$((${out//_/}))
for n in $((out / 4)) $((2 * out)) $((5 * out / 2 + 2)); do add_number "$n"; done
number="(^|[^[:alnum:]_])(${alternatives})([ui](8|16|32|64|128|size))?([^[:alnum:]_]|$)"
shift_re="(^|[^[:alnum:]_])1([ui](32|64|size))?[[:space:]]*<<[[:space:]]*(${exponents})([^[:alnum:]_]|$)"
power="2[[:space:]]*\^[[:space:]]*(${exponents})([^[:alnum:]_]|$)"
mib='(^|[^[:alnum:]_])[0-9]+[[:space:]]*\*[[:space:]]*1024[[:space:]]*\*[[:space:]]*1024'
pattern="$number|$shift_re|$power|$mib"
scan() {
  local dir="$1" f rc status=0 count=0
  while IFS= read -r -d '' f; do
    case "${f#"$dir/"}" in limits.rs|alloc_guard/mod.rs) continue ;; esac
    count=$((count+1))
    if grep -nEi "$pattern" "$f"; then
      printf 'copied cap numeral in %s\n' "$f" >&2; status=1
    else
      rc=$?; if [ "$rc" -ne 1 ]; then return "$rc"; fi
    fi
  done < <(find "$dir" -type f -name '*.rs' -print0)
  if [ "$count" -eq 0 ]; then printf 'no Rust sources scanned\n' >&2; return 2; fi
  return "$status"
}
scan "$src"
printf 'symbolic caps: clean (exponents %s)\n' "$exponents"

if [ "$self_test" -eq 1 ]; then
  # Scratch copies only, under the crate's own runs/ directory; never seed a
  # numeral in the working source tree.
  mkdir -p "$root/runs"
  tmp="$(mktemp -d "$root/runs/caps-lint.XXXXXX")"
  trap 'find "$tmp" -type f -delete; rmdir "$tmp"' EXIT
  cp "$src/limits.rs" "$tmp/limits.rs"
  cp "$src/walk.rs" "$tmp/walk.rs"
  scan "$tmp"
  for literal in 1073741824 1_073_741_824 0x4000_0000 '1u64 << 30' '2^30' 2147483647 0x7fff_ffff '2^31' \
                 268435456 268_435_456 '256 * 1024 * 1024' 65536 '1 << 16' 16777216 '16 * 1024 * 1024' 67108864 671088642; do
    cp "$src/walk.rs" "$tmp/walk.rs"
    printf '\nfn injected_cap() -> u64 { %s }\n' "$literal" >> "$tmp/walk.rs"
    if scan "$tmp" > "$tmp/finding" 2>&1; then
      printf 'negative control missed %s\n' "$literal" >&2; exit 1
    else
      rc=$?
      [ "$rc" -eq 1 ] && grep -q 'copied cap numeral' "$tmp/finding" || { printf 'negative control failed unexpectedly\n' >&2; exit 1; }
    fi
  done
  cp "$src/walk.rs" "$tmp/walk.rs"
  printf 'fn wire_and_types() -> bool { i32::MAX as u32 == u32::MAX / 2 && 65535u32 * 255 > 0 }\n' >> "$tmp/walk.rs"
  scan "$tmp"
  printf 'symbolic caps: scratch-copy positive/negative controls passed\n'
fi
