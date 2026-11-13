#!/usr/bin/env bash
# Serial full run (one shared CPU: never run two benchmarks at once).
set -euo pipefail
cd "$(dirname "$0")"
for f in bmp tga icns ico; do
  for p in corpus large; do
    python3 -I bench.py "$f" "$p"
  done
done
