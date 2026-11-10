#!/usr/bin/env bash
# Builds the HDR runners under /tmp/claude-0/target-hdr (Rust, release) and /tmp/claude-0/target-hdr/stb-hdr (C).
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
export CARGO_TARGET_DIR=/tmp/claude-0/target-hdr
export PATH=/opt/rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu/bin:$PATH
cargo build --release --manifest-path "$here/Cargo.toml"
if [ ! -f "$here/runners/stb_image.h" ]; then curl -fsSL -o "$here/runners/stb_image.h" https://raw.githubusercontent.com/nothings/stb/master/stb_image.h; fi
cc -O2 -w "$here/runners/stb.c" -I"$here/runners" -o /tmp/claude-0/target-hdr/stb-hdr -lm
