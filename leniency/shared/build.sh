#!/usr/bin/env bash
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
mkdir -p /tmp/claude-0/leniency-target
export CARGO_TARGET_DIR=/tmp/claude-0/glycin-target
export PATH=/opt/rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu/bin:$PATH
cargo build --manifest-path "$here/Cargo.toml" --locked
cc -O2 -Wall "$here/gdk.c" -o /tmp/claude-0/leniency-target/gdk-formats $(pkg-config --cflags --libs gdk-pixbuf-2.0 gio-2.0)
