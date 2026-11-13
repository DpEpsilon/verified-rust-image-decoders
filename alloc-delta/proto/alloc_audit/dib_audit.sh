#!/bin/bash
# Positive control: run the alloc_audit prototype on dib_verus's public decode
# entry (gate F1). The tool runs as `RUSTC_WORKSPACE_WRAPPER` on the dib_verus
# crate, exactly as SPEC §6 specifies.
#
# Usage: dib_audit.sh
#   ALLOC_AUDIT_BIN        path to the built tool (default: the proto target dir)
#   ALLOC_AUDIT_ROOTS      roots (default: DIB's exported decode entry points)
#   ALLOC_AUDIT_ALLOWLIST  allowlist file (default: none)
#   DIB_CRATE              dib_verus crate dir
set -u
here="$(cd "$(dirname "$0")" && pwd)"
export PATH=/opt/cargo/bin:/opt/verus:$PATH
export RUSTC_BOOTSTRAP=1
SR=$(rustc --print sysroot)
export LD_LIBRARY_PATH="$SR/lib/rustlib/x86_64-unknown-linux-gnu/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
bin="${ALLOC_AUDIT_BIN:-/tmp/claude-0/alloc-audit-target/debug/alloc_audit}"
target="${ALLOC_AUDIT_DIB_TARGET:-/tmp/claude-0/dib-audit-target}"
# Default to the *pinned* revision of dib_verus that the allowlist's
# `source_hash` covers. The live crate under ../dib/dib_verus is under active
# rewrite by another writer and does not always compile; the positive control
# must audit the exact text the allowlist pins. Point DIB_CRATE at the live
# crate to audit it directly once it builds.
crate="${DIB_CRATE:-$here/pinned-dib}"
roots="${ALLOC_AUDIT_ROOTS:-decode_with_steps,decode,decode_with_policy}"
allowlist="${ALLOC_AUDIT_ALLOWLIST:-}"
# The audit's configuration is carried by environment variables, because a
# cargo workspace wrapper cannot carry its own CLI. Export only the ones that
# are set, so an unset flag is *absent* rather than empty.
cfg=""
if [ -n "${ALLOC_AUDIT_PROTOTYPE:-}" ]; then cfg="$cfg export ALLOC_AUDIT_PROTOTYPE=1;"; fi
if [ -n "${ALLOC_AUDIT_EDGES:-}" ]; then cfg="$cfg export ALLOC_AUDIT_EDGES=1;"; fi
# The release profile is the spec's production configuration and the one the
# allowlist's `profile` records. Default to it, so a bare invocation matches.
profile="${ALLOC_AUDIT_DIB_PROFILE:-release}"
mkdir -p "$here/reports"

# Cargo invokes the wrapper on every crate (including vstd and its build deps)
# and probes it with `-vV`. Gate the audit on the crate name; pass every other
# crate straight through to the real rustc.
wrapper="$here/reports/wrapper.sh"
cat >"$wrapper" <<WRAP
#!/bin/bash
real_rustc="\$1"; shift
argv=("\$@")
crate_name=""
for ((i=0; i<\${#argv[@]}; i++)); do
  if [ "\${argv[i]}" = "--crate-name" ]; then
    crate_name="\${argv[i+1]}"
  fi
done
if [ "\$crate_name" != "dib_verus" ]; then
  exec "\$real_rustc" "\$@"
fi
export ALLOC_AUDIT_REPORT="$here/reports/dib_verus.txt"
export ALLOC_AUDIT_ROOTS="$roots"
export ALLOC_AUDIT_ALLOWLIST="$allowlist"
$cfg
exec "$bin" "\$real_rustc" "\$@"
WRAP
chmod +x "$wrapper"

cd "$crate" || exit 2
CARGO_TARGET_DIR="$target" \
  RUSTC_WORKSPACE_WRAPPER="$wrapper" \
  cargo build --offline -j2 --profile "$profile" \
  >"$here/reports/dib_verus.build" 2>&1
rc=$?

echo "dib_audit: cargo exit=$rc"
if [ -f "$here/reports/dib_verus.txt" ]; then
  head -50 "$here/reports/dib_verus.txt"
else
  echo "dib_audit: no audit report produced (dib_verus not audited?)" >&2
  tail -25 "$here/reports/dib_verus.build" >&2
fi
exit $rc
