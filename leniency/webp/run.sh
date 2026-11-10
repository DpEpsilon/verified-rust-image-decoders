#!/usr/bin/env bash
# One command: build runners + seed tool, regenerate seeds, run the WebP matrix. Usage: ./run.sh [--inputs 7000 --seed 731 --jobs 2]
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
export PATH=/opt/rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu/bin:$PATH CARGO_TARGET_DIR=/tmp/claude-0/target-modern
cargo build --release --manifest-path "$here/Cargo.toml"
B=/tmp/claude-0/leniency-webp/bin; mkdir -p "$B"
gcc -O2 -w -o "$B/libwebp" "$here/runners/libwebp.c" -lwebpdemux -lwebp -lm
gcc -O2 -w -o "$B/mkseeds" "$here/runners/mkseeds.c" -lwebpmux -lwebp -lsharpyuv -lm
"$B/mkseeds" "$here/seeds"
PYTHONDONTWRITEBYTECODE=1 python3 -I "$here/harness.py" "$@"
