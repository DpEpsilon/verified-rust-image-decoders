#!/bin/bash
real_rustc="$1"; shift
argv=("$@")
crate_name=""
for ((i=0; i<${#argv[@]}; i++)); do
  if [ "${argv[i]}" = "--crate-name" ]; then
    crate_name="${argv[i+1]}"
  fi
done
if [ "$crate_name" != "dib_verus" ]; then
  exec "$real_rustc" "$@"
fi
export ALLOC_AUDIT_REPORT="/__modal/volumes/vo-Iyh3VLkOB7cEHrIp2RkTs7/projects/verified-decoders/alloc-delta/proto/alloc_audit/reports/dib_verus.txt"
export ALLOC_AUDIT_ROOTS="decode_with_steps,decode,decode_with_policy"
export ALLOC_AUDIT_ALLOWLIST="/__modal/volumes/vo-Iyh3VLkOB7cEHrIp2RkTs7/projects/verified-decoders/alloc-delta/proto/alloc_audit/allowlist/dib_verus.toml"
 export ALLOC_AUDIT_PROTOTYPE=1;
exec "/tmp/claude-0/alloc-audit-target/debug/alloc_audit" "$real_rustc" "$@"
