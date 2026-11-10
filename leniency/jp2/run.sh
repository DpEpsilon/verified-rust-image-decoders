#!/usr/bin/env bash
# One command: build native libs (once), runners, seeds; run the JPEG 2000 matrix. Usage: ./run.sh [--inputs 7000 --seed 731 --jobs 2]
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
"$here/../modern/build-native.sh"
export PATH=/opt/rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu/bin:$PATH CARGO_TARGET_DIR=/tmp/claude-0/target-modern
cargo build --release --manifest-path "$here/Cargo.toml"
B=/tmp/claude-0/leniency-jp2/bin; mkdir -p "$B"; O=/tmp/claude-0/src3/opj-inst; J=/tmp/claude-0/src3/jasper-inst
gcc -O2 -w -I$O/include/openjpeg-2.5 -o "$B/openjpeg" "$here/runners/openjpeg.c" $O/lib/libopenjp2.a -lm -lpthread
gcc -O2 -w -I$J/include -o "$B/jasper" "$here/runners/jasper.c" $J/lib/libjasper.a -ljpeg -lm -lpthread
gcc -O2 -w -I$O/include/openjpeg-2.5 -o "$B/mkseeds" "$here/runners/mkseeds.c" $O/lib/libopenjp2.a -lm -lpthread
"$B/mkseeds" "$here/seeds"
PYTHONDONTWRITEBYTECODE=1 python3 -I "$here/harness.py" "$@"
