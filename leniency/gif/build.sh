#!/usr/bin/env bash
# Builds every GIF/PNG runner under /tmp/claude-0 (nothing is written to the source trees).
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
out=/tmp/claude-0/leniency-gifpng-bin; src=/tmp/claude-0/gifpng-src
mkdir -p "$out" "$src"
export CARGO_HOME=${CARGO_HOME_GIFPNG:-/tmp/claude-0/cargo-home-gifpng}
export CARGO_TARGET_DIR=/tmp/claude-0/target-gifpng
export PATH=/opt/rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu/bin:$PATH
cargo build --release --manifest-path "$here/Cargo.toml" --locked
install -m755 "$CARGO_TARGET_DIR/release/leniency-gifpng-runner" "$out/rust.new.$$" && mv -f "$out/rust.new.$$" "$out/rust"   # atomic: safe during a running harness
fetch() { # name url-or-repo pinned-rev
  [ -d "$src/$1" ] || git clone -q https://github.com/$2 "$src/$1"
  git -C "$src/$1" checkout -q "$3"
}
fetch libnsgif netsurf-browser/libnsgif c26c7aa
fetch libspng randy408/libspng 46c81dc
fetch giflib mirrorer/giflib fa37672
fetch stb nothings/stb 4e9b368
G="$(pkg-config --cflags --libs glib-2.0)"
cc -O2 -Wall -I"$here/runners" "$here/runners/gdk_anim.c" -o "$out/gdk-anim.new.$$" $(pkg-config --cflags --libs gdk-pixbuf-2.0 gio-2.0)
cc -O2 -w -I"$here/runners" -I"$src/libnsgif/include" -I"$src/libnsgif/src" -I"$src/giflib/lib" -I"$src/stb" \
   "$here/runners/gif_native.c" "$src/libnsgif/src/gif.c" "$src/libnsgif/src/lzw.c" \
   "$src/giflib/lib/dgif_lib.c" "$src/giflib/lib/gif_err.c" "$src/giflib/lib/gifalloc.c" "$src/giflib/lib/gif_hash.c" \
   "$src/giflib/lib/openbsd-reallocarray.c" -o "$out/gif-native.new.$$" $G -lm
cc -O2 -w -I"$here/runners" -I"$src/libspng/spng" -I"$src/stb" \
   "$here/runners/png_native.c" "$src/libspng/spng/spng.c" -o "$out/png-native.new.$$" $G $(pkg-config --cflags --libs libpng) -lz -lm
for n in gdk-anim gif-native png-native; do mv -f "$out/$n.new.$$" "$out/$n"; done
echo "built $out"
