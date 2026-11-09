#!/usr/bin/env bash
set -euo pipefail
mkdir -p /tmp/claude-0/leniency-target /tmp/claude-0/glycin-target
export PATH=/opt/rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu/bin:$PATH
export CARGO_TARGET_DIR=/tmp/claude-0/glycin-target
cargo build --manifest-path "$(dirname "$0")/Cargo.toml" --locked
cc -O2 -Wall "$(dirname "$0")/runners/gdk.c" -o /tmp/claude-0/leniency-target/gdk-runner $(pkg-config --cflags --libs gdk-pixbuf-2.0 gio-2.0)
