#!/bin/bash
# Fetches the real-world BMP/ICO/CUR test files used by dib/dib_verus/tests/corpus.rs (local testing only;
# Pillow's repository is MIT-CMU licensed, individual images may carry other rights: do not redistribute).
set -e
cd "$(dirname "$0")/.."
tmp=$(mktemp -d)
git clone --depth 1 --filter=blob:none --sparse https://github.com/python-pillow/Pillow "$tmp/p"
git -C "$tmp/p" fetch --depth 1 origin aa60851e1f0f00a82560a553cbad4429d9233b86 2>/dev/null || true
git -C "$tmp/p" sparse-checkout set Tests/images
mkdir -p corpus
find "$tmp/p/Tests/images" -maxdepth 1 \( -name '*.bmp' -o -name '*.ico' -o -name '*.cur' \) -size -1025k -exec cp {} corpus/ \;
rm -rf "$tmp"
echo "corpus/: $(ls corpus | wc -l) files. NOTE: the frozen results were measured on 84 files at Pillow commit aa60851e; a newer checkout may differ. Then: python3 -I tools/corpus_expected.py"
