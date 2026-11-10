#!/usr/bin/env bash
# One command: build native libs (once), runners, seeds; run the JPEG XL matrix. Usage: ./run.sh [--inputs 7000 --seed 731 --jobs 2]
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
"$here/../modern/build-native.sh"
export PATH=/opt/rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu/bin:$PATH CARGO_TARGET_DIR=/tmp/claude-0/target-modern
export PKG_CONFIG_PATH=/tmp/claude-0/src3/jxl-inst/lib/pkgconfig
cargo build --release --manifest-path "$here/Cargo.toml"
B=/tmp/claude-0/leniency-jxl/bin; mkdir -p "$B"
for t in libjxl mkseeds; do gcc -O2 -w $(pkg-config --cflags libjxl) -o "$B/$t" "$here/runners/$t.c" $(pkg-config --static --libs libjxl) -lm; done
"$B/mkseeds" "$here/seeds"
PYTHONDONTWRITEBYTECODE=1 python3 -I "$here/harness.py" "$@"
