#!/usr/bin/env bash
# Builds only the `gif:image` selector (image crate 0.25.10) and the gdk-pixbuf runner, for machines where the
# full build.sh (glycin path dependency, ~400 crates) is too heavy. Writes under /tmp/claude-0; nothing under the source tree.
# The Rust source is src/main.rs with the glycin/png/gift/tinygif/lodepng/minipng/zune code cut out by line range,
# so Acc and image_frames are the same code the full runner uses.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
out=/tmp/claude-0/leniency-gifpng-bin; w=/tmp/claude-0/gif-image-only
mkdir -p "$out" "$w/src"
export CARGO_HOME=${CARGO_HOME_GIFPNG:-/tmp/claude-0/cargo-home-gifpng}
export CARGO_TARGET_DIR=/tmp/claude-0/target-gif-image-only
export PATH=/opt/rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu/bin:$PATH
python3 -I - "$here/src/main.rs" "$w/src/main.rs" <<'PY'
import sys
L = open(sys.argv[1]).read().split('\n')
def seg(a, b): return L[a - 1:b]          # 1-based inclusive
head = ['use std::io::Cursor;', 'use image::{AnimationDecoder, ImageDecoder};', 'use serde_json::{json, Value};', 'use sha2::{Digest, Sha256};']
body = seg(15, 58) + seg(67, 69) + seg(124, 145)
tail = '''fn run(which: &str, bytes: &[u8], dump: Option<&str>) -> Value {
    match which {
        "gif:image" => {
            let d = match image::codecs::gif::GifDecoder::new(Cursor::new(bytes)) { Ok(d) => d, Err(e) => return err(e) };
            let orig = format!("{:?}", d.color_type()); image_frames(d.into_frames(), &orig, "frame-sequence", dump)
        }
        _ => err("unknown selector"),
    }
}
'''.split('\n') + seg(370, len(L))
open(sys.argv[2], 'w').write('\n'.join(head + body + tail))
PY
cat > "$w/Cargo.toml" <<'TOML'
[package]
name = "gif-image-only"
version = "0.1.0"
edition = "2021"
[dependencies]
image = { version = "=0.25.10", default-features = false, features = ["gif"] }
serde_json = "1.0"
sha2 = "0.10"
[profile.release]
opt-level = 1
debug = false
panic = "unwind"
codegen-units = 16
TOML
cargo build -j1 --release --manifest-path "$w/Cargo.toml"
install -m755 "$CARGO_TARGET_DIR/release/gif-image-only" "$out/rust-image"
cc -O2 -w -I"$here/runners" "$here/runners/gdk_anim.c" -o "$out/gdk-anim" $(pkg-config --cflags --libs gdk-pixbuf-2.0 gio-2.0)
grep -A1 'name = "gif"' "$w/Cargo.lock" | tr '\n' ' '; echo
echo "built $out/rust-image and $out/gdk-anim"
