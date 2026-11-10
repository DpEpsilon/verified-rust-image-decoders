#!/usr/bin/env bash
# Builds Rust runner and C runners under /tmp/claude-0 only.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"; fmt="$(basename "$here")"
out=/tmp/claude-0/leniency-$fmt-bin; mkdir -p "$out"
export CARGO_TARGET_DIR=/tmp/claude-0/target-xbm   # shared by xbm/ and xpm/ (same dependency set, one compile)
export PATH=/opt/rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu/bin:$PATH
cargo build --manifest-path "$here/Cargo.toml" --release
cp "$CARGO_TARGET_DIR/release/leniency-$fmt-runner" "$out/rust-runner"
cc -O2 -Wall "$( [ -f "$here/runners/gdk.c" ] && echo "$here/runners/gdk.c" || echo "$here/../shared/gdk.c" )" -o "$out/gdk" $(pkg-config --cflags --libs gdk-pixbuf-2.0 gio-2.0)
[ -f "$here/runners/x11.c" ] && cc -O2 -Wall "$here/runners/x11.c" -o "$out/x11" -lX11 || true
echo built "$out"
