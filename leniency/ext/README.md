# PNM, QOI and farbfeld adapters (`pnm/`, `qoi/`, `farbfeld/`, shared code in `ext/`)

Run any one with `./run.sh --inputs 10000 --seed 731 --jobs 4` from `pnm/`, `qoi/` or `farbfeld/`. `run.sh` calls `ext/build.sh`
(Cargo target `/tmp/claude-0/target-ext`, binaries in `/tmp/claude-0/leniency-ext-bin`, the reference QOI decoder fetched from
github.com/phoboslab/qoi into `/tmp/claude-0/srcs`), then runs `harness.py`. Per-format inputs, manifest, per-input results and anomaly inputs go to
`/tmp/claude-0/leniency-{pnm,qoi,farbfeld}/`. Tests: `PYTHONDONTWRITEBYTECODE=1 python3 -I tests/test_<format>.py` after a build.
`python3 -I reproduce.py FILE` runs one file through every decoder.

Layout: `ext/src/main.rs` (one Rust binary, selector `format:decoder`), `ext/gdk.c` (the shared GDK runner plus an RGBA16 hash),
`ext/qoi_ref.c` (reference C QOI decoder wrapper), `ext/ff_oracle.py` (farbfeld spec oracle), `ext/common.py` (generator, runner, clustering,
minimiser, renderers; it imports the unmodified `engine.py`). Each format directory holds only its seeds, mutators and decoder table.

## Normalisation (all three formats)

Each decoder's output is expanded to RGBA16 (8-bit v -> v*257; grey -> r=g=b; no alpha -> 65535) and RGBA8 = (v+128)/257, which is
the identity on 8-bit sources. `rgba_sha256` hashes RGBA8 (the shared contract), `rgba16_sha256` hashes RGBA16 little-endian. The harness does not rescale low-maxval
PNM samples: whatever a decoder returns is hashed, so a decoder that returns unscaled samples for maxval 15 differs from one that scales to 0..255. The extra
class `accept-identical-rgba8-different-rgba16` marks inputs where all decoders agree after 8-bit rounding but not at 16 bits (precision policy). GDK is 8-bit only.
Images over 16,000,000 pixels are reported as `harness-limit` (an error class, not a decoder verdict) after the decoder's own header parse succeeded.

## Votes

`image` and `glycin` (glycin-image-rs `ImgLoader`) run the same `image::codecs::*` code and are one vote; the `qoi` crate is image's QOI backend (image streams the
body and ignores trailing padding; `qoi::decode_to_vec` validates it, so they can differ on one file but still share the group). The four GDK entry points are one vote.
"Lone accept / lone reject" counts compare a decoder against all other vote groups.

## Decoders

| Format | Counted | Unavailable |
| --- | --- | --- |
| PNM | image 0.25.10, glycin-image-rs, zune-ppm 0.5.1, oxideav-pbm 0.0.5, GDK pnm (file / auto / forced / forced one-byte writes) | netpbm CLI (not installed, SourceForge), `pnm` crate 0.1.1 (needs nightly), ImageMagick |
| QOI | image, glycin-image-rs, qoi 0.4.1, rapid-qoi 0.6.3, zune-qoi 0.5.2, phoboslab/qoi reference C | GDK (no module) |
| farbfeld | image, glycin-image-rs, farbfeld 0.3.1, oracle-spec (written here) | zune-farbfeld 0.5.2 (decode() always fails; kept as selector `farbfeld:zune`), GDK (no module), suckless tools |

The glycin MIME used: `image/x-portable-anymap` (all four PNM MIME types construct the same `PnmDecoder`), `image/qoi`, `image/x-ff`. glycin's `.conf` files
register only the PNM types; QOI and farbfeld are reachable only by calling `ImgLoader` directly, as the runner does.

Crashes, aborts and timeouts are recorded and counted per decoder (local view only) and the inputs are saved under `anomalies/`; they are not investigated here.
