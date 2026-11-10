#!/usr/bin/env bash
# Builds every runner for pnm/, qoi/ and farbfeld/ under /tmp/claude-0 only.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
export PATH=/opt/rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu/bin:$PATH
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/claude-0/target-ext}
out=/tmp/claude-0/leniency-ext-bin; mkdir -p "$out" /tmp/claude-0/srcs
cargo build --manifest-path "$here/Cargo.toml"
cp "$CARGO_TARGET_DIR/debug/leniency-ext-runner" "$out/"
cc -O2 -Wall "$here/gdk.c" -o "$out/gdk-formats" $(pkg-config --cflags --libs gdk-pixbuf-2.0 gio-2.0)
# Reference QOI decoder: phoboslab/qoi (header-only C), fetched, never copied into this repository.
if [ ! -f /tmp/claude-0/srcs/qoi/qoi.h ]; then git clone -q https://github.com/phoboslab/qoi.git /tmp/claude-0/srcs/qoi; fi
git -C /tmp/claude-0/srcs/qoi rev-parse HEAD > "$out/qoi-ref-commit.txt"
cc -O2 -Wall -I/tmp/claude-0/srcs/qoi "$here/qoi_ref.c" -o "$out/qoi-ref" $(pkg-config --cflags --libs glib-2.0)
