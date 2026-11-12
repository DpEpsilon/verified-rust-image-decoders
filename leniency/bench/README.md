# Decode benchmark: verified Rust decoders vs incumbents

Decode-only, in-process timing of the verified cores (`dib_verus`, `tga_verus`, `icns_verus`, `ico_verus`, all under `~/projects/verified-decoders`) against gdk-pixbuf 2.42.12 (C, as installed, `-O2` runner), the `image` crate 0.25.10, and the format crates the leniency harnesses use (`icns` 0.3, `ico` 0.3, `tinytga` 0.5.0). Nothing in the verified crates was modified; `cargo verus` and `verify.sh` were not run.

## Headline

Verified decoders are not uniformly fast or slow. Best-pass comparisons on the large synthetic files, other conditions equal:

- ICO: 3-4x faster than the `ico` crate; 1.2-1.5x slower than `image` and gdk-pixbuf.
- TGA RLE: 1.5-1.7x faster than `image`, about 4x faster than gdk-pixbuf and `tinytga`.
- TGA uncompressed: 2-3x faster than gdk-pixbuf/`tinytga`, but 2-4.5x slower than `image`.
- BMP: within noise of `image` on 4096x4096 24-bit, about 1.5x faster on the 8192x2048 and 32-bit files, 1.4x slower on 8-bit palette; 1.3-3x slower than gdk-pixbuf; 1.9x faster than `image` and 1.4x faster than gdk-pixbuf on RLE8.
- ICNS legacy (128x128 only): 1.7x faster than the `icns` crate, 4x slower than gdk-pixbuf.
- Caps: the verified decoders reject inputs above 64 MiB, outputs above 64 MiB (16,777,216 pixels) and dimensions above 8,192. The 8192x8192 and 16384x16384 files are rejected with `FileTooLarge`; the incumbents decode 8192x8192 (peak RSS 645 MB `image`, 394 MB gdk-pixbuf). Large-file comparisons therefore stop at 4096x4096 (24-bit, 8-bit, RLE8, TGA) and 4096x3840 (32-bit), plus 8192x2048 BMP.

Treat every large-file number as noisy (see "Noise"): other agents ran cargo/Verus jobs on the single CPU during the final run.

## Method

- One benchmark binary per language, same CLI and same output format: `src/main.rs` (Rust: verified, `image`, `icns`, `ico`, `tinytga`) and `gdkbench.c` (gdk-pixbuf, `GdkPixbufLoader` with the format forced, one `write` of the in-memory file, `close`, `unref`). Release builds: Rust `opt-level = 3` (all dependencies, including the verified crates), C `-O2`.
- All input files are read into memory before timing; the timed region is decode only. The output is RGBA8 for every decoder: verified decoders return it directly; the `image` crate result is `into_rgba8()` (copy-free for RGBA, one conversion pass for RGB); gdk-pixbuf returns RGB or RGBA natively and is **not** converted in the timed region (this favours gdk-pixbuf on 24-bit inputs: it writes 3 bytes per pixel, hence the 49 MB RSS delta against 64 MB). `tinytga` output is filled into an RGBA buffer inside the timed region (the harness does the same).
- Equality gate (`bench.py`, "check" mode): every decoder decodes every file once; width, height and the SHA-256 of the RGBA bytes must equal the verified decoder's. A decoder/file pair that disagrees, or where either side errors, is excluded from the timing set and counted in the "agreement" line. Corpus timing is per pair: verified and one incumbent are timed on exactly the files both decode identically. For BMP the two `decode_with_policy(BmpFile, Strict|Recommended)` variants must also match.
- A pass decodes the file list once (the runner repeats the list so one pass lasts at least about 100 ms). Warm-up: 2 passes (corpus) or 1 pass (large). Per process: 7 timed passes (corpus) or 5 (large). 5 processes per decoder, interleaved (round 1 runs every decoder, then round 2, and so on), each a fresh process.
- Primary number: **CPU time** (`CLOCK_PROCESS_CPUTIME_ID`) over all timed passes of one process divided by the pass count, then the median over the 5 processes ("median CPU ns/px") and the minimum ("min CPU ns/px"). The sandbox CPU clock ticks every 10 ms, so a single pass's CPU reading is quantised; the total over a process is not. Wall time is reported next to it: median over all passes and the single fastest pass ("best pass"). ns/px divides by decoded pixels. MB/s is decoded RGBA bytes (4 x pixels) per CPU second.
- Peak RSS: `getrusage(RUSAGE_SELF).ru_maxrss` at process end (`/proc/self/status` VmHWM reads 0 in this sandbox). It includes the input file held in memory; the parenthesised value is the increase over the RSS after loading the input, i.e. the decoder's own peak. Meaningful on the large files only; corpus rows show it for completeness.
- Noise bookkeeping: before every process `bench.py` checks `ps` for other processes at 25% CPU or more (lifetime-averaged), waits 5 s once, and records the hit in the JSON `flags`; it also records the 1-minute load average (always 0.0 in this sandbox, so useless) and the number of other runnable processes before and after each process (last column). A row is marked NOISY when the CPU time of the 5 processes differs by more than 5% (max/min - 1) or the wall time of a single pass varies by more than 30% (max-min over median).

## Corpora and large files

- BMP: the 71 `*.bmp` files in `~/projects/verified-decoders/dib/corpus/` (Pillow test images; local test use only, not copied anywhere). ICO also uses its `*.ico` files.
- TGA (930 files), ICNS (835), ICO (1,268) from `/tmp/claude-0/corpora/<format>/files/`, collected by another agent; sources and licences are recorded per file in `/tmp/claude-0/corpora/<format>/MANIFEST.tsv` (mixed: MIT, LGPL-2.1+ gdk-pixbuf tests, Pillow tests with NOASSERTION). They were read as they stood during the run. Most of them are malformed or unsupported on purpose, so each pair's timing set is the subset that decodes identically (counts below).
- Large files are generated by `src/bin/gen.rs` (seeded xorshift, seed 20260101; gradient plus noise tiles; RLE packet mixes from the same generator; PNG with stored deflate blocks) into `/tmp/claude-0/bench-large/`. No third-party bytes are involved and the files are not committed. List: BMP 24-bit 4096x4096 and 8192x2048, 32-bit 4096x3840 (alpha 255), 8-bit palette raw and RLE8 4096x4096, TGA 24-bit raw/RLE 4096x4096 and 32-bit raw/RLE 4096x3840; ICNS legacy 128x128 (`it32`+`t8mk`), the same plus a 32 MiB unrelated element, and a PNG `ic10` 1024x1024; ICO one 256x256 32-bit DIB, six entries 16..256, and a 1024x1024 DIB under a 256x256 directory record. Probes above the verified caps: BMP 24-bit 8192x8192 and 16384x16384, TGA 24-bit 8192x8192 (incumbents only; 16384 is checked, not timed).
- ICNS selection: verified = the largest legacy colour+mask pair via `parse` and `combine` (no PNG support in the core); `icns` crate = its own largest icon; gdk-pixbuf = its own choice. ICO selection: verified and `ico` crate both pick the largest entry by area then bit depth; `image` and gdk-pixbuf pick their own. Files where the choices differ show up as disagreements.

## Reproduce

```sh
export PATH=/opt/rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu/bin:$PATH
export CARGO_TARGET_DIR=/tmp/claude-0/bench-target          # keep build output off the volume
cd ~/projects/leniency/bench
cargo build --release --offline                              # no --locked: Cargo.lock is copied from ../shared and stale
mkdir -p /tmp/claude-0/bench-bin
cc -O2 -Wall gdkbench.c -o /tmp/claude-0/bench-bin/gdkbench $(pkg-config --cflags --libs gdk-pixbuf-2.0 gio-2.0)
/tmp/claude-0/bench-target/release/gen /tmp/claude-0/bench-large 20260101 --probes
ps -eo pid,pcpu,args --sort=-pcpu | head                     # confirm nothing heavy is running; nproc is 1
./run_all.sh                                                 # serial: bench.py {bmp,tga,icns,ico} {corpus,large}
python3 -I report.py > results/tables.md                     # regenerate the tables below
```

`python3 -I bench.py FORMAT corpus|large [--rounds 5] [--passes N] [--warmup N] [--no-time]` runs one cell; `--no-time` does only the equality check. Raw results (aggregates only, no input bytes) are in `results/*.json`. `rm -rf /tmp/claude-0/bench-target` removes the build output; no `target/` directory exists under the repository.

## Noise

The corpora tables were measured with little competing load (the "other runnable" column is mostly 1-2 on the machine's own tooling) and their CPU spreads are mostly 1-10%. The large-file tables were measured while other agents ran `rust_verify`/`rustc` and Python validation jobs on the same CPU (the flags in `results/*-large.json` name them, for example `/opt/verus/rust_verify` at 109-134% CPU). CPU time does not count time spent descheduled, but memory-bound decodes of 50-64 MB inputs still slow down when another job evicts the cache and shares memory bandwidth, so median CPU ns/px on the large files varies up to 2x between rounds (verified BMP 24-bit: median 3.10, min 1.99). Use the min columns and the "best pass" wall time as the least contended estimate, and read ratios under about 1.5x on large files as inconclusive. Ratios on the corpus tables are stable (CPU spread under 5% in most rows). Sandbox quirks: the CPU clock has 10 ms granularity, `/proc/loadavg` and `VmHWM` read zero, so the harness uses totals over many passes, `getrusage`, and `ps`.

## Results

CPU ns/px lower is better. The "(Nx)" is incumbent median CPU divided by verified median CPU, so below 1 means the incumbent is faster. Rows for the verified policy variants (`decode_with_policy` Strict and Recommended) are within noise of plain `decode` everywhere.


### BMP corpus

files: 71, verified decodes 50; rounds=5 passes=7 warmup=2

agreement with verified (per incumbent): bmp-image: both-ok-equal 45, both-error 14, incumbent-only-ok 7, both-ok-DIFFERENT 3, verified-only-ok 2; gdk:bmp: both-ok-equal 40, both-error 11, incumbent-only-ok 10, both-ok-DIFFERENT 9, verified-only-ok 1

| decoder | subset | median CPU ns/px (vs verified) | min CPU ns/px | wall ns/px median / best pass | MB/s RGBA (CPU median) | CPU spread across rounds / wall spread across passes | peak RSS MB (decode delta) | max other-runnable procs |
|---|---|---|---|---|---|---|---|---|
| bmp-verified | bmp-image (45 files) | 4.26 | 4.20 | 4.39 / 4.31 | 939 | 3% / 4% | 19 (+0) | 1 |
| bmp-verified-policy-strict | bmp-image (45 files) | 4.25 | 4.13 | 4.32 / 4.31 | 941 | 3% / 3% | 19 (+0) | 1 |
| bmp-verified-policy-recommended | bmp-image (45 files) | 4.19 | 4.13 | 4.32 / 4.31 | 955 | 3% / 2% | 19 (+0) | 1 |
| bmp-image | bmp-image (45 files) | 3.26 (0.76x) | 3.01 | 3.34 / 3.06 | 1228 | 18% / 19% NOISY | 19 (+0) | 0 |
| bmp-verified | gdk:bmp (40 files) | 4.13 | 3.94 | 4.29 / 4.05 | 969 | 11% / 27% NOISY | 19 (+0) | 1 |
| bmp-verified-policy-strict | gdk:bmp (40 files) | 4.07 | 4.07 | 4.18 / 4.17 | 983 | 1% / 4% | 19 (+0) | 2 |
| bmp-verified-policy-recommended | gdk:bmp (40 files) | 4.07 | 3.59 | 4.18 / 3.68 | 983 | 15% / 14% NOISY | 19 (+0) | 1 |
| gdk:bmp | gdk:bmp (40 files) | 2.59 (0.63x) | 2.59 | 2.68 / 2.66 | 1547 | 3% / 3% | 19 (+0) | 1 |

### BMP large

files: 7, verified decodes 5; rounds=5 passes=5 warmup=1

agreement with verified (per incumbent): bmp-image: both-ok-equal 5, both-error 1, incumbent-only-ok 1; gdk:bmp: both-ok-equal 5, incumbent-only-ok 2

| decoder | subset | median CPU ns/px (vs verified) | min CPU ns/px | wall ns/px median / best pass | MB/s RGBA (CPU median) | CPU spread across rounds / wall spread across passes | peak RSS MB (decode delta) | max other-runnable procs |
|---|---|---|---|---|---|---|---|---|
| bmp-verified | bmp24_4096x4096.bmp | 3.10 | 1.99 | 2.97 / 1.89 | 1291 | 72% / 72% NOISY | 116 (+64) | 1 |
| bmp-verified-policy-strict | bmp24_4096x4096.bmp | 2.70 | 2.15 | 2.84 / 1.88 | 1480 | 44% / 52% NOISY | 117 (+64) | 1 |
| bmp-verified-policy-recommended | bmp24_4096x4096.bmp | 2.74 | 2.62 | 2.88 / 2.22 | 1459 | 8% / 41% NOISY | 117 (+64) | 0 |
| bmp-image | bmp24_4096x4096.bmp | 3.16 (1.02x) | 2.86 | 3.30 / 2.45 | 1266 | 40% / 69% NOISY | 164 (+112) | 1 |
| gdk:bmp | bmp24_4096x4096.bmp | 1.88 (0.61x) | 1.73 | 1.92 / 1.36 | 2130 | 64% / 125% NOISY | 106 (+49) | 0 |
| bmp-verified | bmp24_8192x2048.bmp | 4.65 | 3.58 | 4.69 / 2.90 | 860 | 50% / 62% NOISY | 117 (+64) | 1 |
| bmp-verified-policy-strict | bmp24_8192x2048.bmp | 2.82 | 2.50 | 2.93 / 2.08 | 1418 | 54% / 79% NOISY | 116 (+64) | 0 |
| bmp-verified-policy-recommended | bmp24_8192x2048.bmp | 3.42 | 2.19 | 3.35 / 2.13 | 1171 | 58% / 70% NOISY | 116 (+64) | 1 |
| bmp-image | bmp24_8192x2048.bmp | 7.63 (1.64x) | 7.27 | 8.26 / 5.42 | 524 | 26% / 61% NOISY | 163 (+112) | 1 |
| gdk:bmp | bmp24_8192x2048.bmp | 1.99 (0.43x) | 1.75 | 2.06 / 1.46 | 2013 | 59% / 97% NOISY | 106 (+49) | 0 |
| bmp-verified | bmp32_4096x3840.bmp | 4.96 | 3.88 | 5.10 / 3.29 | 807 | 49% / 57% NOISY | 124 (+60) | 1 |
| bmp-verified-policy-strict | bmp32_4096x3840.bmp | 4.90 | 4.13 | 5.12 / 3.27 | 817 | 42% / 55% NOISY | 124 (+60) | 0 |
| bmp-verified-policy-recommended | bmp32_4096x3840.bmp | 4.45 | 3.56 | 4.68 / 3.22 | 899 | 112% / 127% NOISY | 124 (+60) | 1 |
| bmp-image | bmp32_4096x3840.bmp | 7.63 (1.54x) | 6.99 | 8.19 / 6.26 | 524 | 33% / 49% NOISY | 169 (+105) | 0 |
| gdk:bmp | bmp32_4096x3840.bmp | 3.94 (0.79x) | 2.42 | 3.80 / 1.51 | 1015 | 74% / 103% NOISY | 131 (+61) | 1 |
| bmp-verified | bmp8_4096x4096.bmp | 9.18 | 6.91 | 9.38 / 6.62 | 436 | 41% / 53% NOISY | 84 (+64) | 1 |
| bmp-verified-policy-strict | bmp8_4096x4096.bmp | 7.27 | 4.89 | 7.69 / 4.63 | 550 | 120% / 119% NOISY | 85 (+64) | 1 |
| bmp-verified-policy-recommended | bmp8_4096x4096.bmp | 6.56 | 5.13 | 6.50 / 4.77 | 610 | 95% / 101% NOISY | 85 (+64) | 1 |
| bmp-image | bmp8_4096x4096.bmp | 6.56 (0.71x) | 6.20 | 6.85 / 4.36 | 610 | 37% / 113% NOISY | 132 (+112) | 0 |
| gdk:bmp | bmp8_4096x4096.bmp | 3.04 (0.33x) | 2.32 | 3.16 / 2.09 | 1316 | 77% / 112% NOISY | 74 (+49) | 1 |
| bmp-verified | bmp8rle_4096x4096.bmp | 2.86 | 1.31 | 3.06 / 0.98 | 1398 | 215% / 153% NOISY | 74 (+55) | 0 |
| bmp-verified-policy-strict | bmp8rle_4096x4096.bmp | 3.76 | 2.74 | 4.07 / 2.56 | 1065 | 154% / 166% NOISY | 73 (+54) | 1 |
| bmp-verified-policy-recommended | bmp8rle_4096x4096.bmp | 3.87 | 2.50 | 4.12 / 2.52 | 1032 | 107% / 113% NOISY | 74 (+55) | 0 |
| bmp-image | bmp8rle_4096x4096.bmp | 5.36 (1.88x) | 3.34 | 5.41 / 2.25 | 746 | 143% / 139% NOISY | 122 (+103) | 1 |
| gdk:bmp | bmp8rle_4096x4096.bmp | 3.87 (1.35x) | 2.56 | 3.82 / 2.38 | 1032 | 79% / 84% NOISY | 79 (+60) | 1 |
| probe_bmp24_16384x16384.bmp | - | probe, not timed (FileTooLarge) | | | | |
| bmp-image | probe_bmp24_8192x8192.bmp | 6.32 | 5.30 | 6.66 / 3.94 | 633 | 31% / 56% NOISY | 645 (+448) | 0 |
| gdk:bmp | probe_bmp24_8192x8192.bmp | 2.77 | 1.61 | 2.79 / 1.45 | 1443 | 81% / 68% NOISY | 394 (+193) | 1 |

### TGA corpus

files: 930, verified decodes 262; rounds=5 passes=7 warmup=2

agreement with verified (per incumbent): tga-image: both-ok-equal 239, both-error 661, incumbent-only-ok 7, both-ok-DIFFERENT 23; tga-tinytga: verified-only-ok 102, both-ok-equal 138, both-error 656, incumbent-only-ok 12, both-ok-DIFFERENT 22; gdk:tga: both-ok-equal 239, both-error 653, incumbent-only-ok 15, verified-only-ok 3, both-ok-DIFFERENT 20

| decoder | subset | median CPU ns/px (vs verified) | min CPU ns/px | wall ns/px median / best pass | MB/s RGBA (CPU median) | CPU spread across rounds / wall spread across passes | peak RSS MB (decode delta) | max other-runnable procs |
|---|---|---|---|---|---|---|---|---|
| tga-verified | tga-image (239 files) | 5.82 | 5.75 | 6.00 / 5.96 | 688 | 2% / 2% | 72 (+4) | 1 |
| tga-image | tga-image (239 files) | 1.87 (0.32x) | 1.85 | 1.93 / 1.91 | 2138 | 2% / 3% | 75 (+7) | 1 |
| tga-verified | tga-tinytga (138 files) | 5.55 | 5.48 | 5.72 / 5.69 | 721 | 2% / 5% | 36 (+4) | 0 |
| tga-tinytga | tga-tinytga (138 files) | 15.81 (2.85x) | 15.69 | 16.35 / 16.17 | 253 | 10% / 35% NOISY | 37 (+4) | 0 |
<!-- round 3 tga-tinytga: other CPU load: 56154  160 /opt/rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu/bin/rustc --crate-name verus_state_machines_ -->
| tga-verified | gdk:tga (239 files) | 5.93 | 5.86 | 6.08 / 6.03 | 675 | 3% / 9% | 75 (+4) | 1 |
| gdk:tga | gdk:tga (239 files) | 18.94 (3.20x) | 18.81 | 19.57 / 19.01 | 211 | 2% / 6% | 84 (+9) | 1 |

### TGA large

files: 5, verified decodes 4; rounds=5 passes=5 warmup=1

agreement with verified (per incumbent): tga-image: incumbent-only-ok 1, both-ok-equal 4; tga-tinytga: both-error 1, both-ok-equal 2, verified-only-ok 2; gdk:tga: incumbent-only-ok 1, both-ok-equal 4

| decoder | subset | median CPU ns/px (vs verified) | min CPU ns/px | wall ns/px median / best pass | MB/s RGBA (CPU median) | CPU spread across rounds / wall spread across passes | peak RSS MB (decode delta) | max other-runnable procs |
|---|---|---|---|---|---|---|---|---|
| tga-image | probe_tga24_8192x8192.tga | 3.43 | 2.09 | 2.65 / 2.13 | 1167 | 256% / 232% NOISY | 644 (+448) | 1 |
| gdk:tga | probe_tga24_8192x8192.tga | 16.96 | 16.84 | 17.76 / 16.93 | 236 | 13% / 22% NOISY | 587 (+385) | 1 |
<!-- round 1 tga-image: other CPU load: 57322 96.9 python3 -I -B investigate_pixdiff.py -->
<!-- round 1 gdk:tga: other CPU load: 57322 96.8 python3 -I -B investigate_pixdiff.py -->
| tga-verified | tga24_4096x4096.tga | 8.23 | 6.32 | 8.32 / 6.50 | 486 | 40% / 39% NOISY | 117 (+64) | 1 |
| tga-image | tga24_4096x4096.tga | 4.41 (0.54x) | 2.38 | 4.02 / 2.11 | 907 | 178% / 138% NOISY | 164 (+112) | 0 |
| tga-tinytga | tga24_4096x4096.tga | 16.69 (2.03x) | 15.62 | 17.56 / 15.95 | 240 | 15% / 17% NOISY | 116 (+64) | 0 |
| gdk:tga | tga24_4096x4096.tga | 18.12 (2.20x) | 16.81 | 18.72 / 17.08 | 221 | 11% / 17% NOISY | 153 (+97) | 1 |
| tga-verified | tga24rle_4096x4096.tga | 4.77 | 2.86 | 4.96 / 2.64 | 839 | 94% / 90% NOISY | 84 (+64) | 5 |
| tga-image | tga24rle_4096x4096.tga | 8.23 (1.73x) | 7.27 | 8.30 / 4.73 | 486 | 33% / 77% NOISY | 131 (+112) | 2 |
| tga-tinytga | tga24rle_4096x4096.tga | 18.12 (3.80x) | 17.05 | 18.59 / 17.18 | 221 | 10% / 16% NOISY | 83 (+64) | 2 |
| gdk:tga | tga24rle_4096x4096.tga | 18.00 (3.77x) | 16.93 | 18.16 / 17.14 | 222 | 9% / 15% NOISY | 89 (+64) | 1 |
<!-- round 1 tga-tinytga: other CPU load: 60582 96.3 python3 -I -B policy_oracle.py -->
<!-- round 3 tga-tinytga: other CPU load: 61539 33.3 sleep 5 -->
| tga-verified | tga32_4096x3840.tga | 8.65 | 6.74 | 8.45 / 6.78 | 463 | 34% / 40% NOISY | 124 (+60) | 5 |
| tga-image | tga32_4096x3840.tga | 1.91 (0.22x) | 1.34 | 1.84 / 0.89 | 2097 | 69% / 157% NOISY | 124 (+60) | 5 |
| gdk:tga | tga32_4096x3840.tga | 19.71 (2.28x) | 19.20 | 21.11 / 17.97 | 203 | 14% / 28% NOISY | 190 (+121) | 5 |
<!-- round 1 tga-verified: other CPU load: 65132  134 /opt/verus/rust_verify /opt/rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu/bin/rustc --crate-nam -->
<!-- round 1 tga-image: other CPU load: 65132  109 /opt/verus/rust_verify /opt/rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu/bin/rustc --crate-nam -->
| tga-verified | tga32rle_4096x3840.tga | 4.39 | 2.92 | 3.93 / 2.77 | 912 | 115% / 105% NOISY | 83 (+60) | 2 |
| tga-image | tga32rle_4096x3840.tga | 6.36 (1.45x) | 5.98 | 7.04 / 5.26 | 629 | 91% / 110% NOISY | 83 (+60) | 2 |
| gdk:tga | tga32rle_4096x3840.tga | 19.20 (4.38x) | 18.44 | 19.83 / 17.60 | 208 | 8% / 26% NOISY | 108 (+80) | 2 |
<!-- round 1 tga-verified: other CPU load: 16091  100 [leniency-format] -->
<!-- round 1 tga-image: other CPU load: 65161 40.2 python3 -I -B validate.py -->

### ICNS corpus

files: 835, verified decodes 114; rounds=5 passes=7 warmup=2

agreement with verified (per incumbent): icns-crate: both-error 631, incumbent-only-ok 90, both-ok-DIFFERENT 44, verified-only-ok 31, both-ok-equal 39; gdk:icns: both-error 657, both-ok-equal 78, incumbent-only-ok 64, both-ok-DIFFERENT 36

| decoder | subset | median CPU ns/px (vs verified) | min CPU ns/px | wall ns/px median / best pass | MB/s RGBA (CPU median) | CPU spread across rounds / wall spread across passes | peak RSS MB (decode delta) | max other-runnable procs |
|---|---|---|---|---|---|---|---|---|
| icns-verified | icns-crate (39 files) | 8.25 | 8.01 | 8.50 / 7.86 | 485 | 10% / 15% NOISY | 20 (+0) | 1 |
| icns-crate | icns-crate (39 files) | 10.98 (1.33x) | 9.90 | 11.03 / 9.77 | 364 | 19% / 21% NOISY | 20 (+0) | 1 |
<!-- round 4 icns-crate: other CPU load: 26399 40.0 bash /home/ws/.claude/plugins/cache/agent-sanitizer/agent-sanitizer/2.70.4/scripts/safe-launch.sh P -->
| icns-verified | gdk:icns (78 files) | 8.44 | 8.32 | 8.41 / 8.07 | 474 | 3% / 15% | 20 (+0) | 1 |
| gdk:icns | gdk:icns (78 files) | 4.38 (0.52x) | 4.38 | 4.55 / 4.44 | 912 | 7% / 13% NOISY | 20 (+0) | 0 |

### ICNS large

files: 3, verified decodes 2; rounds=5 passes=5 warmup=1

agreement with verified (per incumbent): icns-crate: both-ok-equal 2, incumbent-only-ok 1; gdk:icns: both-ok-equal 2, both-error 1

| decoder | subset | median CPU ns/px (vs verified) | min CPU ns/px | wall ns/px median / best pass | MB/s RGBA (CPU median) | CPU spread across rounds / wall spread across passes | peak RSS MB (decode delta) | max other-runnable procs |
|---|---|---|---|---|---|---|---|---|
| icns-verified | icns_legacy128.icns | 6.35 | 5.91 | 6.48 / 5.77 | 629 | 20% / 32% NOISY | 20 (+0) | 0 |
| icns-crate | icns_legacy128.icns | 10.68 (1.68x) | 9.21 | 10.73 / 8.94 | 375 | 20% / 33% NOISY | 20 (+0) | 1 |
| gdk:icns | icns_legacy128.icns | 1.46 (0.23x) | 0.58 | 1.48 / 0.59 | 2743 | 175% / 81% NOISY | 20 (+0) | 2 |
<!-- round 4 gdk:icns: other CPU load: 26665 96.0 python3 -I -B policy_oracle.py -->
| icns-verified | icns_legacy128_pad32M.icns | 6.27 | 5.43 | 6.20 / 5.29 | 638 | 18% / 22% NOISY | 37 (+0) | 0 |
| icns-crate | icns_legacy128_pad32M.icns | 1220.70 (194.67x) | 579.83 | 1175.14 / 418.67 | 3 | 158% / 174% NOISY | 68 (+32) | 1 |
| gdk:icns | icns_legacy128_pad32M.icns | 714.98 (114.02x) | 575.47 | 768.20 / 296.87 | 6 | 221% / 254% NOISY | 75 (+34) | 1 |
<!-- round 5 gdk:icns: other CPU load: 26883 97.2 python3 -I -B investigate_pixdiff.py -->
| icns-crate | icns_png1024.icns | 13.03 | 11.13 | 12.89 / 10.33 | 307 | 34% / 51% NOISY | 25 (+5) | 2 |
<!-- round 1 icns-crate: other CPU load: 26883 97.2 python3 -I -B investigate_pixdiff.py -->
<!-- round 2 icns-crate: other CPU load: 26883 97.1 python3 -I -B investigate_pixdiff.py -->

### ICO corpus

files: 1277, verified decodes 241; rounds=5 passes=7 warmup=2

agreement with verified (per incumbent): ico-crate: both-ok-equal 238, both-error 939, incumbent-only-ok 97, verified-only-ok 2, both-ok-DIFFERENT 1; ico-image: both-ok-equal 209, both-error 941, incumbent-only-ok 95, verified-only-ok 5, both-ok-DIFFERENT 27; gdk:ico: both-ok-equal 231, both-error 988, verified-only-ok 4, incumbent-only-ok 48, both-ok-DIFFERENT 6

| decoder | subset | median CPU ns/px (vs verified) | min CPU ns/px | wall ns/px median / best pass | MB/s RGBA (CPU median) | CPU spread across rounds / wall spread across passes | peak RSS MB (decode delta) | max other-runnable procs |
|---|---|---|---|---|---|---|---|---|
| ico-verified | ico-crate (238 files) | 3.25 | 3.20 | 3.38 / 3.37 | 1230 | 3% / 1% | 22 (+0) | 1 |
| ico-crate | ico-crate (238 files) | 10.55 (3.25x) | 8.84 | 10.58 / 8.89 | 379 | 24% / 26% NOISY | 22 (+0) | 1 |
| ico-verified | ico-image (209 files) | 3.31 | 3.31 | 3.43 / 3.42 | 1207 | 3% / 7% | 22 (+0) | 2 |
| ico-image | ico-image (209 files) | 2.49 (0.75x) | 2.45 | 2.56 / 2.54 | 1608 | 1% / 1% | 22 (+0) | 0 |
| ico-verified | gdk:ico (231 files) | 3.31 | 3.31 | 3.39 / 3.37 | 1209 | 0% / 1% | 22 (+0) | 2 |
| gdk:ico | gdk:ico (231 files) | 2.72 (0.82x) | 2.57 | 2.81 / 2.64 | 1468 | 8% / 7% NOISY | 23 (+1) | 1 |
<!-- round 3 ico-verified: other CPU load: 28048 97.5 python3 -I -B validate.py -->
<!-- round 3 gdk:ico: other CPU load: 28048 97.3 python3 -I -B validate.py -->

### ICO large

files: 3, verified decodes 3; rounds=5 passes=5 warmup=1

agreement with verified (per incumbent): ico-crate: both-ok-equal 3; ico-image: both-ok-equal 3; gdk:ico: both-ok-equal 2, verified-only-ok 1

| decoder | subset | median CPU ns/px (vs verified) | min CPU ns/px | wall ns/px median / best pass | MB/s RGBA (CPU median) | CPU spread across rounds / wall spread across passes | peak RSS MB (decode delta) | max other-runnable procs |
|---|---|---|---|---|---|---|---|---|
| ico-verified | ico_dib256.ico | 3.18 | 2.90 | 3.20 / 2.92 | 1259 | 10% / 12% NOISY | 18 (+0) | 1 |
| ico-crate | ico_dib256.ico | 13.44 (4.23x) | 12.32 | 13.46 / 12.15 | 298 | 14% / 23% NOISY | 18 (+0) | 1 |
| ico-image | ico_dib256.ico | 2.27 (0.72x) | 2.27 | 2.43 / 2.34 | 1759 | 11% / 12% NOISY | 18 (+0) | 1 |
| gdk:ico | ico_dib256.ico | 1.81 (0.57x) | 1.81 | 1.88 / 1.85 | 2212 | 6% / 9% NOISY | 18 (+0) | 1 |
<!-- round 1 ico-verified: other CPU load: 28048 96.8 python3 -I -B validate.py -->
<!-- round 1 ico-crate: other CPU load: 28048 87.9 python3 -I -B validate.py -->
| ico-verified | ico_multi6.ico | 3.07 | 2.93 | 3.16 / 3.01 | 1305 | 19% / 19% NOISY | 18 (+0) | 1 |
| ico-crate | ico_multi6.ico | 13.50 (4.40x) | 10.56 | 13.86 / 10.45 | 296 | 33% / 33% NOISY | 18 (+0) | 1 |
| ico-image | ico_multi6.ico | 2.60 (0.85x) | 2.34 | 2.62 / 2.40 | 1540 | 17% / 17% NOISY | 18 (+0) | 1 |
| gdk:ico | ico_multi6.ico | 1.72 (0.56x) | 1.46 | 1.82 / 1.43 | 2319 | 36% / 31% NOISY | 18 (+0) | 1 |
| ico-verified | probe_ico_dib1024_dir256.ico | 2.98 | 2.86 | 3.10 / 2.90 | 1342 | 8% / 7% NOISY | 18 (+0) | 0 |
| ico-crate | probe_ico_dib1024_dir256.ico | 9.35 (3.14x) | 7.63 | 9.97 / 7.38 | 428 | 50% / 54% NOISY | 20 (+2) | 2 |
| ico-image | probe_ico_dib1024_dir256.ico | 2.24 (0.75x) | 2.13 | 2.26 / 2.11 | 1783 | 11% / 15% NOISY | 18 (+0) | 0 |

## Surprises

- **TGA uncompressed, verified vs `image` crate: 3x slower** on the corpus (5.8 vs 1.9 CPU ns/px) and 2-4.5x on the 24/32-bit large files (best CPU about 6.3-6.7 vs 1.3-2.4 ns/px; medians 8.2 vs 4.4 and 8.7 vs 1.9 under load). Guess from reading `tga_verus/src/tga.rs`: `decode_raw` converts and stores one pixel per loop iteration, through `write_pixel`, which is four separate `Vec::set` byte stores with bounds checks, plus an orientation-dependent destination offset per pixel. The `image` crate copies whole rows for 32-bit and byte-swaps rows for 24-bit, which the compiler vectorises. The verified code also zero-fills the 4N-byte output first (`vec![0u8; 4 * n]`).
- **BMP 8-bit palette: verified 1.4x slower than `image`, about 3x slower than gdk-pixbuf** (median CPU 9.2 vs 6.6 vs 3.0 ns/px; best 6.9, 6.2, 2.3). Likely the same per-pixel `out.set(o..o+3, ...)` pattern (`dib.rs` around lines 786-789) with a palette lookup per pixel. On the 24/32-bit files verified and `image` are within noise of each other; gdk-pixbuf is faster by roughly 1.5-2x partly because it keeps 3 bytes per pixel for 24-bit input.
- **BMP corpus: verified 1.6x slower than gdk-pixbuf, 1.3x slower than `image`.** The corpus files are small, so fixed per-file costs (header validation, one 4N allocation and zero fill) weigh more.
- **ICNS legacy: gdk-pixbuf about 4x faster than verified** on the 128x128 entry (1.5 vs 6.4 ns/px; the corpus subset shows 1.9x). `icns_verus::rle_decode` builds three separate channel `Vec`s, then `decode_rgb` interleaves into a 3N buffer, and `combine` copies again into 4N: three allocations and three passes against one in gdk-pixbuf.
- **ICNS padded file (`icns_legacy128_pad32M`): `icns` crate 195x and gdk-pixbuf 114x slower per pixel than verified.** Not a throughput result. Both incumbents read or copy the 32 MiB unrelated element; the verified `parse` records the payload range and never touches it. The ns/px figure is normalised by only 16,384 decoded pixels.
- **Verified is faster than three incumbents**: `ico` crate 3.3-4.4x slower than verified (cause not checked in its source), `tinytga` 2.9x and gdk-pixbuf 3.2x slower on TGA (about 16-19 ns/px; probably gdk-pixbuf's incremental loader and per-pixel conversion; not checked), `image` 1.5-1.9x slower on RLE TGA and RLE8 BMP. The corresponding verified RLE paths copy runs without a per-pixel orientation computation.
- **Resource caps**: 64 MiB input and output and 8,192 per side mean the verified BMP/TGA decoders cannot read the 8192x8192 and 16384x16384 files that `image` and gdk-pixbuf decode (peak RSS 645 and 394 MB at 8192x8192). Peak decode RSS delta on 4096x4096 is the exact 64 MiB output for verified, +112 MB for `image` (RGB intermediate plus RGBA), +49 MB for gdk-pixbuf (RGB only).
- **Disagreements**: Of 71 BMP files, 3 differ from `image` and 9 from gdk-pixbuf; of 930 TGA, 23 differ from `image`; of 835 ICNS, 44 differ from the `icns` crate (possibly different icon selection rather than pixel differences; not checked) and of 1,277 ICO, 27 differ from `image`. They are excluded from timing; the per-pair examples are in `results/*-corpus.json` (`agreement`). I did not investigate which side is right.
- **Not covered**: glycin loaders, PNG-bearing ICNS/ICO entries (the verified cores delegate PNG), 16384x16384 timing, multi-threaded gdk-pixbuf use, `gdk_pixbuf_new_from_file`/stream paths (the forced-type loader is the same decoder without sniffing and file I/O).
