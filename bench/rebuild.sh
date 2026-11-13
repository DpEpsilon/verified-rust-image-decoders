#!/usr/bin/env bash
# Rebuild runners and large files into /tmp (regenerable). Usage: bash rebuild.sh
set -euo pipefail
export PATH=/opt/rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu/bin:$PATH
H=$(cd "$(dirname "$0")" && pwd)
export CARGO_TARGET_DIR=/tmp/claude-0/bench-target
cd "$H" && cargo build --release --offline
mkdir -p /tmp/claude-0/bench-bin /tmp/claude-0/bench-old/src/bin /tmp/claude-0/bench-large
cc -O2 -Wall gdkbench.c -o /tmp/claude-0/bench-bin/gdkbench $(pkg-config --cflags --libs gdk-pixbuf-2.0 gio-2.0)
/tmp/claude-0/bench-target/release/gen /tmp/claude-0/bench-large 20260101 --probes
cp old-runner/Cargo.toml.example /tmp/claude-0/bench-old/Cargo.toml
cp Cargo.lock /tmp/claude-0/bench-old/Cargo.lock
cp src/bin/gen.rs /tmp/claude-0/bench-old/src/bin/gen.rs
cp old-runner/main.rs /tmp/claude-0/bench-old/src/main.rs
cd /tmp/claude-0/bench-old && CARGO_TARGET_DIR=/tmp/claude-0/bench-old-target cargo build --release --offline
echo REBUILD-DONE
