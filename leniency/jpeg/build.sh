#!/usr/bin/env bash
# Builds every JPEG and TIFF column under /tmp/claude-0 (never in a source tree).
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
bin=/tmp/claude-0/leniency-jt-bin
mkdir -p "$bin"
export PATH=/opt/rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu/bin:$PATH
export CARGO_TARGET_DIR=/tmp/claude-0/target-jpegtiff
cargo build --release --manifest-path "$here/runner/Cargo.toml" --locked -q
cp "$CARGO_TARGET_DIR/release/leniency-jt-runner" "$bin/"
glib="$(pkg-config --cflags --libs glib-2.0)"
cc -O2 -Wall "$here/runners/jpeggen.c" -o "$bin/jpeggen" $(pkg-config --cflags --libs libjpeg)
cc -O2 -Wall "$here/runners/libjpeg.c" -o "$bin/libjpeg-runner" $(pkg-config --cflags --libs libjpeg) $glib
cc -O2 -Wall "$here/runners/gdk.c" -o "$bin/gdk-jt" $(pkg-config --cflags --libs gdk-pixbuf-2.0 gio-2.0)
cc -O2 -Wall "$here/../tiff/runners/libtiff.c" -o "$bin/libtiff-runner" $(pkg-config --cflags --libs libtiff-4) $glib
