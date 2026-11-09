# Public repository copy manifest

Copy the following paths from `/home/ws/projects/leniency/`, preserving relative paths. These are the **exact** files for the current seed-731 run. The coordinator performs the copy and commit; do not copy a directory wholesale.

## Harness and documentation

- `README.md`
- `engine.py`
- `format_harness.py`
- `GLYCIN-FORMATS.md`
- `SYNC-TO-REPO.md`
- `shared/Cargo.toml`
- `shared/Cargo.lock`
- `shared/build.sh`
- `shared/gdk.c`
- `shared/src/main.rs`
- `bmp/Cargo.toml`
- `bmp/Cargo.lock`
- `bmp/build.sh`
- `bmp/harness.py`
- `bmp/reproduce.py`
- `bmp/runners/gdk.c`
- `bmp/src/main.rs`
- `bmp/tests/test_engine.py`
- `tga/run.sh`
- `tga/reproduce.py`
- `tga/tests/test_tga.py`
- `tga/summary-public.json`
- `tga/LENIENCY-PUBLIC.md`
- `icns/run.sh`
- `icns/reproduce.py`
- `icns/tests/test_icns.py`
- `icns/summary-public.json`
- `icns/LENIENCY-PUBLIC.md`
- `ico/run.sh`
- `ico/reproduce.py`
- `ico/oracle_runner.py`
- `ico/tests/test_ico.py`
- `ico/summary-public.json`
- `ico/LENIENCY-PUBLIC.md`

## Frozen-derived BMP witnesses

- `bmp/witnesses/01-planes.bmp`
- `bmp/witnesses/02-biSizeImage.bmp`
- `bmp/witnesses/05-width.bmp`
- `bmp/witnesses/06-biClrUsed.bmp`
- `bmp/witnesses/07-random-bitflip.bmp`
- `bmp/witnesses/08-bitfield-mask.bmp`
- `bmp/witnesses/09-bfSize.bmp`
- `bmp/witnesses/10-truncation.bmp`
- `bmp/witnesses/11-biSize.bmp`
- `bmp/witnesses/12-frozen-BareDib.bmp`
- `bmp/witnesses/13-inconsistent-pair.bmp`
- `bmp/witnesses/15-frozen-BmpFile.bmp`
- `bmp/witnesses/17-bitCount.bmp`
- `bmp/witnesses/gdk-path-01.bmp`
- `bmp/witnesses/gdk-path-02.bmp`
- `bmp/witnesses/pixel-01-frozen-BareDib.bmp`
- `bmp/witnesses/pixel-02-frozen-BmpFile.bmp`
- `bmp/witnesses/pixel-03-rle-escape.bmp`
- `bmp/witnesses/pixel-04-bitfield-mask.bmp`
- `bmp/witnesses/pixel-05-width.bmp`
- `bmp/witnesses/verified-only-header16.bmp`

## Frozen-derived TGA witnesses

- `tga/witnesses/01-origin-alpha-bits.tga`
- `tga/witnesses/03-map_flag.tga`
- `tga/witnesses/04-byte-flip.tga`
- `tga/witnesses/05-descriptor.tga`
- `tga/witnesses/06-trailing-bytes.tga`
- `tga/witnesses/08-image_type.tga`
- `tga/witnesses/10-height.tga`
- `tga/witnesses/11-width.tga`
- `tga/witnesses/12-rle-packet.tga`
- `tga/witnesses/13-frozen-decode.tga`
- `tga/witnesses/14-id_length.tga`
- `tga/witnesses/15-truncation.tga`
- `tga/witnesses/16-pixel_depth.tga`
- `tga/witnesses/pixel-01.tga`
- `tga/witnesses/pixel-02.tga`
- `tga/witnesses/pixel-03.tga`
- `tga/witnesses/pixel-04.tga`
- `tga/witnesses/pixel-06.tga`
- `tga/witnesses/pixel-09.tga`

## Frozen-derived ICNS witnesses

- `icns/witnesses/01-entry_length.icns`
- `icns/witnesses/02-entry_type.icns`
- `icns/witnesses/03-declared_length.icns`
- `icns/witnesses/04-trailing-bytes.icns`
- `icns/witnesses/05-byte-flip.icns`
- `icns/witnesses/06-entry-type.icns`
- `icns/witnesses/07-synth-mixed.icns`
- `icns/witnesses/08-synth-png.icns`
- `icns/witnesses/09-synth-legacy.icns`
- `icns/witnesses/10-frozen-parse.icns`
- `icns/witnesses/11-frozen-combine.icns`
- `icns/witnesses/12-synth-multi.icns`
- `icns/witnesses/selection-01.icns`
- `icns/witnesses/selection-02.icns`
- `icns/witnesses/selection-03.icns`
- `icns/witnesses/selection-04.icns`
- `icns/witnesses/selection-05.icns`
- `icns/witnesses/selection-06.icns`
- `icns/witnesses/selection-07.icns`
- `icns/witnesses/selection-08.icns`
- `icns/witnesses/pixel-01.icns`
- `icns/witnesses/pixel-02.icns`
- `icns/witnesses/pixel-03.icns`
- `icns/witnesses/pixel-04.icns`

## Frozen-derived ICO witnesses

- `ico/witnesses/01-payload_length.ico`
- `ico/witnesses/02-kind.ico`
- `ico/witnesses/03-entry_reserved.ico`
- `ico/witnesses/04-reserved.ico`
- `ico/witnesses/05-entry_height.ico`
- `ico/witnesses/06-entry_width.ico`
- `ico/witnesses/07-byte-flip.ico`
- `ico/witnesses/08-depth_or_hotspot_y.ico`
- `ico/witnesses/09-entry-byte.ico`
- `ico/witnesses/11-payload_offset.ico`
- `ico/witnesses/13-planes_or_hotspot_x.ico`
- `ico/witnesses/14-frozen-parse.ico`
- `ico/witnesses/16-frozen-decode.ico`
- `ico/witnesses/17-truncation.ico`
- `ico/witnesses/18-entry_count.ico`

Do **not** copy `bmp/summary.json`, `bmp/LENIENCY.md`, any new adapter's `summary.json` or `LENIENCY.md`: those full local matrices include findings/witness links derived from GDK/glycin/local corpora. Do not copy any file under `/tmp/claude-0/`, including input manifests, per-input results, corpus-derived witnesses, anomaly inputs, RGBA dumps, or build outputs. Do not copy unlisted stale witnesses, `__pycache__`, `.pyc`, third-party corpora, source decoder repositories, or generated artifacts from earlier smoke runs. The three `summary-public.json` files independently recount **only** frozen vectors and their mutations; their linked witnesses are listed above. No copy operation or commit was performed here.
