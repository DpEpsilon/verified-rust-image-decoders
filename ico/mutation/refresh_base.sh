#!/bin/bash
# Refresh mutation/base/ico_verus from the live crate, keeping only the live
# source: src/{ico,model,model_vectors,witnesses,lib}.rs, tests/, Cargo.toml,
# Cargo.lock.  The *.m1/*.m2/*.m3 milestone backups are not part of the live
# source tree for the campaign and are left out.
set -e
cd "$(dirname "$0")/.." || exit 2
rm -rf mutation/base/ico_verus
mkdir -p mutation/base/ico_verus
cp -a ico_verus/Cargo.toml ico_verus/Cargo.lock ico_verus/tests mutation/base/ico_verus/
mkdir -p mutation/base/ico_verus/src
for f in ico.rs model.rs model_vectors.rs witnesses.rs lib.rs; do
  cp -a "ico_verus/src/$f" mutation/base/ico_verus/src/$f
done
echo "base refreshed"
