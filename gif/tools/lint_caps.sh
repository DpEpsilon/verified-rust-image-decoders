#!/bin/bash
# G1 symbolic-cap source lint. Scan models, proofs, executable code and comments.
# This grep-based falsification gate recognises the frozen cap values, their
# common decimal/hex/shift spellings, the old input-lookahead sum and output/4,
# and configured decimal cap values. It is not a Rust constant-expression parser.
# Use u16::MAX/u32::MAX for wire/type bounds rather than ambiguous cap numerals.
# The only source exemptions are limits.rs and the frozen alloc_guard/mod.rs:
# the latter independently owns MAX_BYTES, whose hash audit_alloc.sh pins.
# Usage: lint_caps.sh [--self-test] [--src /absolute/path/to/src]
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd -P)"
src="$root/gif_verus/src"
self_test=0
while [ "$#" -gt 0 ]; do
  case "$1" in
    --self-test) self_test=1; shift ;;
    --src) src="$2"; shift 2 ;;
    *) printf 'unknown argument: %s\n' "$1" >&2; exit 2 ;;
  esac
done
[[ "$src" = /* && -f "$src/limits.rs" ]] || { printf 'missing absolute source directory: %s\n' "$src" >&2; exit 2; }

# Digit separators may occur at any digit boundary. Identifier boundaries avoid
# matching a longer integer or a hash. Integer suffixes remain in scope.
digits() {
  local s="$1" out='' i
  for ((i=0; i<${#s}; i++)); do out+="${s:i:1}_*"; done
  printf '%s' "$out"
}
alternatives=''
add_number() {
  local n="$1" dec hex
  # Small caps collide with ordinary non-cap literals; the frozen-value gate
  # below still applies, but this lexical gate cannot classify those uses.
  if ((n < 32768)); then return; fi
  dec="$(digits "$n")"
  printf -v hex '%x' "$n"
  hex="$(digits "$hex")"
  alternatives+="${alternatives:+|}${dec}|0[xX]0*${hex}"
}
for n in 268435456 65535 67108864 65536 1073741824 16777216 268500992; do add_number "$n"; done
# Extract only literal arithmetic, never shell-evaluate source text.
while IFS= read -r line; do
  expr="${line#*=}"; expr="${expr%%;*}"; expr="${expr//_/}"
  if [[ "$expr" =~ ^[[:space:]0-9xXa-fA-F\<\>\(\)\+\*/-]+$ ]]; then
    n=$((expr)); add_number "$n"
  else
    printf 'unrecognised cap expression: %s\n' "$line" >&2; exit 2
  fi
done < <(grep -E '^pub const MAX_(INPUT|DIMENSION|OUTPUT|FRAMES|TOTAL_PIXELS):' "$src/limits.rs")
number="(^|[^[:alnum:]_])(${alternatives})([ui](8|16|32|64|128|size))?([^[:alnum:]_]|$)"
shift='(^|[^[:alnum:]_])1([ui](32|64|size))?[[:space:]]*<<[[:space:]]*(16|24|26|28|30)([^[:alnum:]_]|$)'
# 2^16 is a legitimate LE16-domain block allowance, not MAX_FRAMES.
power='2\^(24|26|28|30)([^[:alnum:]_]|$)'
pattern="$number|$shift|$power"
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
printf 'symbolic caps: clean\n'

if [ "$self_test" -eq 1 ]; then
  # Scratch copies only; never seed a numeral in the working source tree.
  tmp="$(mktemp -d "${CAPS_LINT_SCRATCH:-$root/runs}/caps-lint.XXXXXX")"
  trap 'find "$tmp" -type f -delete; rmdir "$tmp"' EXIT
  cp "$src/limits.rs" "$tmp/limits.rs"
  cp "$src/compose_new.rs" "$tmp/proof.rs"
  scan "$tmp"
  for literal in 268435456 268_435_456 65_535 67_108_864 65_536 1_073_741_824 16_777_216 268_500_992 0x1000_0000 '1u64 << 30' '2^30'; do
    cp "$src/compose_new.rs" "$tmp/proof.rs"
    printf '\nproof fn injected_cap() { assert(MAX_INPUT == %s); }\n' "$literal" >> "$tmp/proof.rs"
    if scan "$tmp" > "$tmp/finding" 2>&1; then
      printf 'negative control missed %s\n' "$literal" >&2; exit 1
    else
      rc=$?
      [ "$rc" -eq 1 ] && grep -q 'copied cap numeral' "$tmp/finding" || { printf 'negative control failed unexpectedly\n' >&2; exit 1; }
    fi
  done
  printf 'proof fn wire_and_types() { assert(u16::MAX < u32::MAX); assert(u32::MAX == 4294967295); }\n' > "$tmp/proof.rs"
  scan "$tmp"
  printf 'symbolic caps: scratch-copy positive/negative controls passed\n'
fi
