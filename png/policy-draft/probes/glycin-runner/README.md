# glycin PNG runner

`png-glycin-probe` runs the PNG path of glycin's image-rs loader in-process and prints one JSON line per input (see the header of `src/main.rs` for the fields). It produced `../results-glycin.json` and the glycin columns of the held-out runs (`../run_probes.py`, runner name `glycin`).

## What is not in this repository

`src/glycin_png.rs` is not tracked (it is listed in `png/.gitignore`). It is a PNG-only copy of code from glycin, which is licensed MPL-2.0 OR LGPL-2.1-or-later; this repository is MIT. Rather than redistribute the copy without glycin's licence texts, the repository holds the files written for this project (`Cargo.toml`, `Cargo.lock`, `src/main.rs`) and the recipe below.

`Cargo.lock` started as a copy of glycin's own `Cargo.lock` (so the crate versions match glycin's) and lists crate names, versions and checksums only.

## Recreating `src/glycin_png.rs`

1. Check out glycin at commit `7344b95` in `~/projects/glycin-plan/glycin`. `Cargo.toml` depends on `glycin-utils` by the relative path `../../../../../glycin-plan/glycin/glycin-utils`, so the checkout must be there (or edit that path).
2. Concatenate the PNG-relevant code of `glycin-loaders/glycin-image-rs/src/lib.rs` and `src/animated.rs` into one module:
   - from `lib.rs`: `ImgLoader`, `Decoder` (and its `Drop` impl), `LoaderImplementation::load` and `specific_frame`, `ImageRsDecoder`, `ImageRsFormat` with its methods, and the `Reader`, `FrameReceiver` and `FrameSender` type aliases;
   - from `animated.rs`: `worker` and `animated_get_frame`.
3. Delete what the PNG path never reaches, keeping every remaining statement in its original order:
   - the EXR, JPEG, TIFF and JXL branches of `load`;
   - the non-PNG variants of `ImageRsDecoder`;
   - the WebP background call in `worker`;
   - the `image_extras::register()` call (it registers only XBM and XPM hooks);
   - the `Builtin` config.
4. Merge the imports of the two files and drop the `use crate::{...}` line of `animated.rs` (all items now live in one module). Add `#![allow(dead_code)]` and a header stating the file's origin and its licence (MPL-2.0 OR LGPL-2.1-or-later, as glycin).

What this runner does not cover: glycin's sandbox and D-Bus process boundary, and its MIME sniffing (`main.rs` passes `image/png` or `image/apng` itself).

## Build

```sh
CARGO_TARGET_DIR=/tmp/claude-0/png-glycin-target cargo build --release --offline -j1
```

`png` is patched to `image-png` git `e917d6a`, as in glycin's workspace. `run_probes.py` looks for the binary at `/tmp/claude-0/png-glycin-target/release/png-glycin-probe`.
