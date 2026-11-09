#!/bin/bash
# Refresh mutation/base/{src,tests,Cargo.toml,Cargo.lock} from the live crate, keeping
# only the src/dib.rs.model.* backups out (they are not part of the live source).
set -e
cd "$(dirname "$0")/.." || exit 2
rm -rf mutation/base/dib_verus
mkdir -p mutation/base/dib_verus
cp -a dib_verus/Cargo.toml dib_verus/Cargo.lock dib_verus/tests mutation/base/dib_verus/
mkdir -p mutation/base/dib_verus/src
for f in dib.rs model.rs model_vectors.rs lib.rs; do
  cp -a "dib_verus/src/$f" mutation/base/dib_verus/src/$f
done
echo "base refreshed"
