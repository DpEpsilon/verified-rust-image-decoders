# Decode benchmark: verified Rust decoders vs incumbents

Decode-only, in-process timing of the verified cores (`dib_verus`, `tga_verus`, `icns_verus`, `ico_verus`, all under `~/projects/verified-decoders`) against gdk-pixbuf 2.42.12 (C, as installed, `-O2` runner), the `image` crate 0.25.10, and the format crates the leniency harnesses use (`icns` 0.3, `ico` 0.3, `tinytga` 0.5.0). Nothing in the verified crates was modified; `cargo verus` and `verify.sh` were not run.

## Headline

> Update 2026-11-13 (ERRATA E6): every verified-core number in the Headline and the Results tables was measured with the `push`-loop guard. After the guard changed to `resize(n, 0)` the zero-fill regression disappeared on BMP, TGA and ICO in the A/B at the end of this file; the tables were not re-measured against the incumbents.

Post-migration run, 2026-11-12: the cores are the `try_*`/`no_std` versions (fallible allocation guard). 4 CPUs, each benchmark process pinned to core 2 with `taskset -c 2`, no other job running (process check below), 5 interleaved rounds. Numbers are CPU ns/px, best of 5 rounds (the least contended estimate; spreads are in the tables) unless stated. "x slower" divides the incumbent's CPU time into the verified decoder's.

- **The verified decoders are now slower than the incumbents on most rows that they were comparable on before the migration.** Large files, best CPU ns/px: BMP 24/32-bit 6.6 (verified) vs 3.1 (`image`, 2.1x slower) vs 1.5 (gdk-pixbuf, 4.3x slower); BMP 8-bit 7.9 vs 1.6-2.4 (`image`; its rounds vary 74%) vs 2.2 (gdk-pixbuf); TGA uncompressed 9.2-9.5 vs 1.2-1.7 (`image`, 5-8x slower, rounds vary 36-42%) vs 18 (gdk-pixbuf) and 17 (`tinytga`) (verified about 1.9x faster than both); TGA RLE 6.1-6.2 vs 4.0-4.9 (`image`, 1.3-1.5x slower) vs 18-19 (gdk-pixbuf, `tinytga`; verified 3x faster).
- BMP RLE8 and TGA32 RLE: verified rounds differ 98-104% (best 2.9-3.1, median 5.5-6.4), so the ordering against `image` (2.3, 4.0) is inconclusive.
- ICO: 2.2x faster than the `ico` crate on the large files (6.4-7.3 vs 14-15), 1.2x faster on the corpus (8.5 vs 10.0, spreads 19% and 31%); 2.5-2.8x slower than `image` and 3.5x slower than gdk-pixbuf.
- ICNS legacy: 1.3x slower than the `icns` crate on the 128x128 file (19.5 vs 14.7, spreads 18% and 42%, so inconclusive), 1.9x slower on the 43-file corpus subset (21.7 vs 11.6), 3.4x slower than gdk-pixbuf on the 101-file subset (17.1 vs 5.1); the padded file is still decoded without touching the 32 MiB element (19.8 vs 373 `icns`, 287 gdk-pixbuf).
- BMP corpus (45 files shared with `image`): 8.6 vs 4.1, 2.1x slower; vs gdk-pixbuf (40 files): 7.8 vs 2.9, 2.7x slower.
- **Migration cost, same-session A/B** (old committed cores in `~/projects/vd-commit` vs migrated cores, outputs byte-identical on all 13 files, `results/ab-large.json`): the migrated cores are 1.3-1.6x slower on every large file with a quiet baseline (BMP 24-bit 4.29 -> 6.56 ns/px median, TGA raw 7.03 -> 9.18), peak RSS is unchanged. See "Before/after the migration". Part of that is the zero-fill of the 4N output buffer (section there); this run did not isolate it from other changes in the migrated cores.
- Policy profiles (new rows): `bmp-verified-policy-gdk` and `-imagecompat` cost 1.3-1.45x the plain decode on 24/32-bit and 8-bit BMP (9.7 and 8.9 vs 6.8 on bmp24_4096x4096); TGA profiles are within noise of plain decode on the large files. Strict and RecommendedBeta are within noise of plain decode for both formats.
- Caps unchanged: inputs above 64 MiB, outputs above 64 MiB (16,777,216 pixels) and dimensions above 8,192 are rejected, so the 8192x8192 probes are decoded only by the incumbents (peak RSS 644 MB `image`, 394 MB gdk-pixbuf).
- No `OutOfMemory` result occurred in any equality gate or timing run (the runner would have exited with status 86 and the driver aborts loudly).

Noise: the machine was idle, but the sandbox still gives CPU spreads of 5-35% on large files and above 90% on some RLE rows (see each row). Treat ratios under about 1.5x as inconclusive unless the row is marked otherwise.

Machine state before timing (`ps -eo pid,pcpu,etime,args --sort=-pcpu | head -15`, 2026-11-12 17:29 UTC, `nproc` = 4): only the harness's own `ps`/`cut`, `claude`, `cli-proxy-api`, `model-router`, `htop`, `tmux` and idle `sleep` poll loops were present, all under 7% CPU; no `rust_verify`, `z3`, `cargo`, `rustc`, `cc` or test job. The driver re-checks `ps` before every process and records hits in each JSON's `flags`; the final run has 0 flags in all four `*-large.json` files.

## Method

- One benchmark binary per language, same CLI and same output format: `src/main.rs` (Rust: verified, `image`, `icns`, `ico`, `tinytga`) and `gdkbench.c` (gdk-pixbuf, `GdkPixbufLoader` with the format forced, one `write` of the in-memory file, `close`, `unref`). Every benchmark process is pinned to one core with `taskset -c 2` (set `BENCH_CPU` to change; the 4-CPU machine has no other job running), so scheduler migration does not add noise. Release builds: Rust `opt-level = 3` (all dependencies, including the verified crates), C `-O2`.
- All input files are read into memory before timing; the timed region is decode only. The output is RGBA8 for every decoder: verified decoders return it directly (`try_*` entry points; `TryError::Format(e)` goes through the runner's error path as `e.name()`, `TryError::OutOfMemory` makes the runner print `HARD FAILURE` and exit 86, which `bench.py` turns into an abort); the `image` crate result is `into_rgba8()` (copy-free for RGBA, one conversion pass for RGB); gdk-pixbuf returns RGB or RGBA natively and is **not** converted in the timed region (this favours gdk-pixbuf on 24-bit inputs: it writes 3 bytes per pixel, hence the 49 MB RSS delta against 64 MB). `tinytga` output is filled into an RGBA buffer inside the timed region (the harness does the same).
- Equality gate (`bench.py`, "check" mode): every decoder decodes every file once; width, height and the SHA-256 of the RGBA bytes must equal the verified decoder's. A decoder/file pair that disagrees, or where either side errors, is excluded from the timing set and counted in the "agreement" line. Corpus timing is per pair: verified and one incumbent are timed on exactly the files both decode identically. For BMP and TGA the `try_decode_with_policy` Strict and RecommendedBeta variants must also match (gate). The GdkCompat and ImageCrateCompat variants (`*-policy-gdk`, `*-policy-imagecompat`) are timed too; they decode some files differently, so they are timed on the files where every profile matches: in the same call when that is the whole gated set, otherwise in a second call on the smaller set (rows labelled "subset where gdk+imagecompat profiles also match"). Their agreement with plain decode is in each JSON's `agreement_profiles`.
- A pass decodes the file list once (the runner repeats the list so one pass lasts at least about 100 ms). Warm-up: 2 passes (corpus) or 1 pass (large). Per process: 7 timed passes (corpus) or 5 (large). 5 processes per decoder, interleaved (round 1 runs every decoder, then round 2, and so on), each a fresh process.
- Primary number: **CPU time** (`CLOCK_PROCESS_CPUTIME_ID`) over all timed passes of one process divided by the pass count, then the median over the 5 processes ("median CPU ns/px") and the minimum ("min CPU ns/px"). The sandbox CPU clock ticks every 10 ms, so a single pass's CPU reading is quantised; the total over a process is not. Wall time is reported next to it: median over all passes and the single fastest pass ("best pass"). ns/px divides by decoded pixels. MB/s is decoded RGBA bytes (4 x pixels) per CPU second.
- Peak RSS: `getrusage(RUSAGE_SELF).ru_maxrss` at process end (`/proc/self/status` VmHWM reads 0 in this sandbox). It includes the input file held in memory; the parenthesised value is the increase over the RSS after loading the input, i.e. the decoder's own peak. Meaningful on the large files only; corpus rows show it for completeness.
- Noise bookkeeping: before every process `bench.py` checks `ps` for other processes at 25% CPU or more (lifetime-averaged), waits 5 s once, and records the hit in the JSON `flags`; it also records the 1-minute load average (always 0.0 in this sandbox, so useless) and the number of other runnable processes before and after each process (last column). A row is marked NOISY when the CPU time of the 5 processes differs by more than 5% (max/min - 1) or the wall time of a single pass varies by more than 30% (max-min over median).

## Corpora and large files

- BMP: the 84 files in `$CORPUS_DIR/bmp/files/` plus the `*.bmp` files in `~/projects/verified-decoders/dib/corpus/` (Pillow test images; local test use only, not copied anywhere), de-duplicated by SHA-256: the 71 dib-corpus BMPs are all contained in the 84, so 84 files. ICO likewise adds the dib corpus's `*.ico`, de-duplicated (433 files).
- TGA (294 files), ICNS (15,701), ICO (433) from `$CORPUS_DIR/<format>/files/` (default `~/corpora`), collected by another agent; sources and licences are recorded per file in `$CORPUS_DIR/<format>/MANIFEST.tsv` (mixed: MIT, LGPL-2.1+ gdk-pixbuf tests, Pillow tests with NOASSERTION). They were read as they stood during the run. The corpora are not the ones of the 2026-11-11 run (BMP 71, TGA 930, ICNS 835, ICO 1,268), so corpus rows are not comparable with the old results. The ICNS corpus is dominated by icon repositories: the equality gate runs on all 15,701 files (verified decodes 267), and the timed subset per incumbent is capped deterministically at the first 835 files by SHA-256 of the content among the files both decoders decode identically. Only 43 (`icns` crate) and 101 (gdk-pixbuf) files agree, so the cap does not bind. (Capping before the gate would have left 2-6 timed files.) Most of them are malformed or unsupported on purpose, so each pair's timing set is the subset that decodes identically (counts below).
- Large files are generated by `src/bin/gen.rs` (seeded xorshift, seed 20260101; gradient plus noise tiles; RLE packet mixes from the same generator; PNG with stored deflate blocks) into `/tmp/claude-0/bench-large/`. No third-party bytes are involved and the files are not committed. List: BMP 24-bit 4096x4096 and 8192x2048, 32-bit 4096x3840 (alpha 255), 8-bit palette raw and RLE8 4096x4096, TGA 24-bit raw/RLE 4096x4096 and 32-bit raw/RLE 4096x3840; ICNS legacy 128x128 (`it32`+`t8mk`), the same plus a 32 MiB unrelated element, and a PNG `ic10` 1024x1024; ICO one 256x256 32-bit DIB, six entries 16..256, and a 1024x1024 DIB under a 256x256 directory record. Probes above the verified caps: BMP 24-bit 8192x8192 and 16384x16384, TGA 24-bit 8192x8192 (incumbents only; 16384 is checked, not timed).
- ICNS selection: verified = the largest legacy colour+mask pair via `parse` and `combine` (no PNG support in the core); `icns` crate = its own largest icon; gdk-pixbuf = its own choice. ICO selection: verified and `ico` crate both pick the largest entry by area then bit depth; `image` and gdk-pixbuf pick their own. Files where the choices differ show up as disagreements.

## Reproduce

```sh
export PATH=/opt/rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu/bin:$PATH
export CARGO_TARGET_DIR=/tmp/claude-0/bench-target          # keep build output off the volume
cd ~/projects/verified-decoders/bench
cargo build --release --offline                              # no --locked: Cargo.lock is copied from ../shared and stale
mkdir -p /tmp/claude-0/bench-bin
cc -O2 -Wall gdkbench.c -o /tmp/claude-0/bench-bin/gdkbench $(pkg-config --cflags --libs gdk-pixbuf-2.0 gio-2.0)
/tmp/claude-0/bench-target/release/gen /tmp/claude-0/bench-large 20260101 --probes
ps -eo pid,pcpu,etime,args --sort=-pcpu | head -15          # confirm no rust_verify/z3/cargo/rustc/cc/test job runs; nproc is 4, every process is pinned with taskset -c 2
export BENCH_CPU=2                                           # default
./run_all.sh                                                 # serial: bench.py {bmp,tga,icns,ico} {corpus,large}
# A/B against the pre-migration cores (copy of the old runner, path dependencies on ~/projects/vd-commit):
#   mkdir -p /tmp/claude-0/bench-old/src/bin; copy old-runner/Cargo.toml.example (-> Cargo.toml; paths point at ~/projects/vd-commit/<fmt>/<fmt>_verus), Cargo.lock, src/bin/gen.rs, and old-runner/main.rs (-> src/main.rs, the pre-migration runner) there;
#   CARGO_TARGET_DIR=/tmp/claude-0/bench-old-target cargo build --release --offline
python3 -I ab.py                                             # interleaved old/new on the large files -> results/ab-large.json
python3 -I report.py > results/tables.md                     # regenerate the tables below
```

`python3 -I bench.py FORMAT corpus|large [--rounds 5] [--passes N] [--warmup N] [--no-time]` runs one cell; `--no-time` does only the equality check. Raw results (aggregates only, no input bytes) are in `results/*.json`. `rm -rf /tmp/claude-0/bench-target /tmp/claude-0/bench-old-target` removes the build output; no `target/` directory exists under the repository.

## Noise

2026-11-12 run: 4 CPUs, benchmark processes pinned to core 2, no other job running (ps check in the Headline; 0 `flags` in every `*-large.json`). Spreads are still 5-35% on most large-file rows and 90-115% on some RLE rows, mostly rounds that differ by a factor of about 2 (for example BMP RLE8 verified: median 5.48, min 2.86); pinning removed scheduler migration but not that effect, whose cause was not identified (the machine is a gVisor sandbox; page placement or memory-bandwidth sharing with the host are guesses). Use the min columns and "best pass" wall time as the least contended estimate and read ratios under about 1.5x as inconclusive.

The 2026-11-11 results (`results/pre-migration-2026-11-11/`) were measured on one CPU while other agents ran `rust_verify`/`rustc`/Python jobs (flags in those JSONs), so they are noisier (verified BMP 24-bit: median 3.10, min 1.99, spread 72%).

Sandbox quirks: the CPU clock has 10 ms granularity, `/proc/loadavg` and `VmHWM` read zero, so the harness uses totals over many passes, `getrusage`, and `ps`.

## How to read the results

CPU ns/px lower is better. The "(Nx)" is incumbent median CPU divided by verified median CPU, so below 1 means the incumbent is faster. Rows for the verified policy variants (`try_decode_with_policy`) are within noise of plain `try_decode` for Strict and RecommendedBeta; GdkCompat and ImageCrateCompat BMP are slower (see Headline).


## Before/after the migration

Large-file BMP and TGA, verified decoder only. "OLD json" is `results/pre-migration-2026-11-11/*-large.json`: **a noisy run on one CPU with other agents' verifier and build jobs running, on the old (committed) cores, and with a different corpus** (irrelevant for large files, which are regenerated from the same seed, but relevant for everything else); do not read its medians as precise (spreads 34-215%). "NEW run_all" is the migrated cores in the pinned, idle run of this README. The "A/B" columns are the like-for-like comparison: both runner binaries (old cores from `~/projects/vd-commit`, built from a copy of the old runner; migrated cores from `src/main.rs`) alternate in one session, 9 interleaved rounds, same pinning; decoded output hashes are identical for every file. Columns are median / min CPU ns/px with the CPU spread across rounds in parentheses; RSS is the peak MB (decode delta).

| file | OLD json (2026-11-11, 1 CPU, other jobs running): median / min CPU ns/px (spread) | RSS MB | NEW run_all (pinned): median / min (spread) | RSS MB | A/B old cores, same session: median / min (spread) | A/B new cores: median / min (spread) | A/B new/old (median / min) | A/B peak RSS MB (decode delta) old vs new |
|---|---|---|---|---|---|---|---|---|
| bmp24_4096x4096.bmp | 3.10 / 1.99 (72%) | 116 (+64) | 6.79 / 6.56 (13%) | 116 (+64) | 4.29 / 4.23 (4%) | 6.56 / 6.32 (13%) | 1.53x / 1.49x | 116 (+64) vs 116 (+64) |
| bmp24_8192x2048.bmp | 4.65 / 3.58 (50%) | 117 (+64) | 6.79 / 6.56 (20%) | 116 (+64) | 4.29 / 4.23 (7%) | 6.79 / 6.56 (11%) | 1.58x / 1.55x | 117 (+64) vs 116 (+64) |
| bmp32_4096x3840.bmp | 4.96 / 3.88 (49%) | 124 (+60) | 6.74 / 6.61 (17%) | 124 (+60) | 5.15 / 5.09 (6%) | 7.25 / 6.87 (15%) | 1.41x / 1.35x | 125 (+60) vs 125 (+60) |
| bmp8_4096x4096.bmp | 9.18 / 6.91 (41%) | 84 (+64) | 7.99 / 7.87 (11%) | 84 (+57) | 5.78 / 2.80 (115%) | 8.23 / 7.63 (14%) | 1.42x / 2.72x | 85 (+59) vs 85 (+59) |
| bmp8rle_4096x4096.bmp | 2.86 / 1.31 (215%) | 74 (+55) | 5.48 / 2.86 (98%) | 74 (+47) | 2.74 / 1.83 (57%) | 5.30 / 2.74 (102%) | 1.93x / 1.50x | 74 (+48) vs 73 (+47) |
| tga24_4096x4096.tga | 8.23 / 6.32 (40%) | 117 (+64) | 9.66 / 9.18 (26%) | 116 (+64) | 7.03 / 6.79 (12%) | 9.18 / 8.94 (16%) | 1.31x / 1.32x | 116 (+64) vs 116 (+64) |
| tga24rle_4096x4096.tga | 4.77 / 2.86 (94%) | 84 (+64) | 6.44 / 6.20 (33%) | 84 (+57) | 3.81 / 3.58 (7%) | 6.20 / 5.84 (20%) | 1.62x / 1.63x | 84 (+58) vs 84 (+58) |
| tga32_4096x3840.tga | 8.65 / 6.74 (34%) | 124 (+60) | 9.79 / 9.54 (24%) | 124 (+60) | 7.12 / 6.87 (6%) | 9.54 / 9.16 (10%) | 1.34x / 1.33x | 124 (+60) vs 124 (+60) |
| tga32rle_4096x3840.tga | 4.39 / 2.92 (115%) | 83 (+60) | 6.36 / 6.10 (104%) | 84 (+57) | 3.81 / 3.69 (12%) | 6.04 / 3.05 (102%) | 1.58x / 0.83x | 83 (+57) vs 83 (+58) |

Reading it:

- Against the old JSON the new numbers look 1.3-2x worse, but the old medians are contaminated; in the same-session A/B the migrated cores are 1.3-1.6x slower on 24/32-bit BMP, raw and RLE TGA (min-to-min 1.32-1.63x), where both builds have spreads under 20%. 8-bit BMP (old spread 115%), BMP RLE8 and TGA32 RLE (new spread 102%) are too noisy for a ratio.
- Peak RSS is identical (same 64 MiB output delta on 4096x4096), as expected: the guard reserves exactly 4N bytes, as the old `vec![0u8; 4*n]` did.

### Does the guard's push-fill cost time on 64 MiB outputs?

> Superseded by ERRATA E6 (2026-11-13): the guard now fills with `resize(n, 0)`; see "A/B after A-STD-4" at the end. The text below describes the `push` loop.

Yes, measurably. `try_zeroed` reserves with `try_reserve_exact` and then writes the zeros with a `push` loop; the old code asked the allocator for zeroed memory (`vec![0u8; 4*n]`, a calloc), which for an allocation this large is fresh pages the kernel zeroes on first touch. In the same-session A/B (same decode, output bytes identical) the new core is 1.53x slower on bmp24_4096x4096 (4.29 -> 6.56 ns/px median, 4.23 -> 6.32 min; spreads 4% old, 13% new), 1.58x on bmp24_8192x2048, 1.41x on bmp32_4096x3840, 1.31x on tga24_4096x4096 (7.03 -> 9.18) and 1.34x on tga32_4096x3840. The added time is about 2.2-2.3 ns per pixel on the 24-bit files, which is 37 ms per 4096x4096 decode (16.8 million pixels) or about 0.55 ns per output byte, i.e. a fill rate of roughly 1.8 GB/s. That is the order of magnitude of one byte-at-a-time store loop with a capacity check per push, but this run did not profile it or replace the fill with a bulk write (`resize`/`memset`) to measure the difference, and the migrated cores differ from the old ones in other ways (policy plumbing and step/log accounting around every allocation), so the measured slowdown is an upper bound on the fill's cost. Peak RSS: no difference.

## Results (post-migration run, 2026-11-12)
### BMP corpus

files: 84, verified decodes 50; rounds=5 passes=7 warmup=2

agreement with verified (per incumbent): bmp-image: both-ok-equal 45, both-error 27, both-ok-DIFFERENT 3, incumbent-only-ok 7, verified-only-ok 2; gdk:bmp: both-ok-equal 40, both-error 24, both-ok-DIFFERENT 9, incumbent-only-ok 10, verified-only-ok 1

| decoder | subset | median CPU ns/px (vs verified) | min CPU ns/px | wall ns/px median / best pass | MB/s RGBA (CPU median) | CPU spread across rounds / wall spread across passes | peak RSS MB (decode delta) | max other-runnable procs |
|---|---|---|---|---|---|---|---|---|
| bmp-verified | bmp-image (45 files) | 8.62 | 8.38 | 8.85 / 8.31 | 464 | 9% / 13% NOISY | 26 (+0) | 1 |
| bmp-verified-policy-strict | bmp-image (45 files) | 8.46 | 8.11 | 8.57 / 8.14 | 473 | 10% / 15% NOISY | 26 (+0) | 0 |
| bmp-verified-policy-recommended-beta | bmp-image (45 files) | 8.46 | 7.75 | 8.63 / 7.64 | 473 | 14% / 19% NOISY | 26 (+0) | 1 |
| bmp-image | bmp-image (45 files) | 4.09 (0.47x) | 3.81 | 4.08 / 3.67 | 979 | 10% / 19% NOISY | 26 (+0) | 1 |
| bmp-verified | bmp-image [subset where gdk+imagecompat profiles also match] (39 files) | 10.57 | 8.46 | 10.72 / 8.30 | 378 | 29% / 29% NOISY | 26 (+0) | 1 |
| bmp-verified-policy-gdk | bmp-image [subset where gdk+imagecompat profiles also match] (39 files) | 13.65 | 12.69 | 13.84 / 12.75 | 293 | 12% / 13% NOISY | 26 (+0) | 1 |
| bmp-verified-policy-imagecompat | bmp-image [subset where gdk+imagecompat profiles also match] (39 files) | 10.96 | 10.81 | 11.32 / 10.78 | 365 | 12% / 21% NOISY | 26 (+0) | 0 |
| bmp-verified | gdk:bmp (40 files) | 7.80 | 7.57 | 8.02 / 7.65 | 513 | 14% / 18% NOISY | 26 (+0) | 1 |
| bmp-verified-policy-strict | gdk:bmp (40 files) | 7.91 | 7.34 | 8.03 / 7.28 | 505 | 12% / 17% NOISY | 26 (+0) | 0 |
| bmp-verified-policy-recommended-beta | gdk:bmp (40 files) | 8.03 | 8.03 | 8.24 / 7.96 | 498 | 4% / 20% | 26 (+0) | 1 |
| gdk:bmp | gdk:bmp (40 files) | 2.89 (0.37x) | 2.77 | 2.89 / 2.75 | 1384 | 7% / 13% NOISY | 26 (+0) | 1 |
| bmp-verified | gdk:bmp [subset where gdk+imagecompat profiles also match] (39 files) | 8.22 | 7.85 | 8.47 / 7.51 | 487 | 9% / 33% NOISY | 26 (+0) | 1 |
| bmp-verified-policy-gdk | gdk:bmp [subset where gdk+imagecompat profiles also match] (39 files) | 15.01 | 14.38 | 15.27 / 13.96 | 266 | 6% / 12% NOISY | 26 (+0) | 0 |
| bmp-verified-policy-imagecompat | gdk:bmp [subset where gdk+imagecompat profiles also match] (39 files) | 11.28 | 10.49 | 11.27 / 10.54 | 355 | 9% / 15% NOISY | 26 (+0) | 0 |

### BMP large

files: 7, verified decodes 5; rounds=5 passes=5 warmup=1

agreement with verified (per incumbent): bmp-image: both-ok-equal 5, both-error 1, incumbent-only-ok 1; gdk:bmp: both-ok-equal 5, incumbent-only-ok 2

| decoder | subset | median CPU ns/px (vs verified) | min CPU ns/px | wall ns/px median / best pass | MB/s RGBA (CPU median) | CPU spread across rounds / wall spread across passes | peak RSS MB (decode delta) | max other-runnable procs |
|---|---|---|---|---|---|---|---|---|
| bmp-verified | bmp24_4096x4096.bmp | 6.79 | 6.56 | 6.76 / 6.53 | 589 | 13% / 30% NOISY | 116 (+64) | 1 |
| bmp-verified-policy-strict | bmp24_4096x4096.bmp | 7.03 | 6.56 | 6.99 / 6.48 | 569 | 13% / 31% NOISY | 116 (+64) | 1 |
| bmp-verified-policy-recommended-beta | bmp24_4096x4096.bmp | 6.68 | 6.32 | 6.65 / 6.34 | 599 | 11% / 28% NOISY | 116 (+64) | 0 |
| bmp-verified-policy-gdk | bmp24_4096x4096.bmp | 9.78 | 9.66 | 9.81 / 9.49 | 409 | 2% / 14% | 116 (+64) | 1 |
| bmp-verified-policy-imagecompat | bmp24_4096x4096.bmp | 8.94 | 8.70 | 9.07 / 8.66 | 447 | 8% / 18% NOISY | 116 (+64) | 1 |
| bmp-image | bmp24_4096x4096.bmp | 3.22 (0.47x) | 3.16 | 3.25 / 3.06 | 1243 | 8% / 38% NOISY | 164 (+112) | 0 |
| gdk:bmp | bmp24_4096x4096.bmp | 1.61 (0.24x) | 1.58 | 1.63 / 1.52 | 2486 | 8% / 16% NOISY | 106 (+49) | 0 |
| bmp-verified | bmp24_8192x2048.bmp | 6.79 | 6.56 | 6.90 / 6.50 | 589 | 20% / 31% NOISY | 116 (+64) | 0 |
| bmp-verified-policy-strict | bmp24_8192x2048.bmp | 6.91 | 6.68 | 6.81 / 6.47 | 579 | 9% / 31% NOISY | 116 (+64) | 0 |
| bmp-verified-policy-recommended-beta | bmp24_8192x2048.bmp | 6.91 | 6.91 | 6.96 / 6.51 | 579 | 3% / 28% | 116 (+64) | 0 |
| bmp-verified-policy-gdk | bmp24_8192x2048.bmp | 9.66 | 9.42 | 9.78 / 9.24 | 414 | 8% / 27% NOISY | 116 (+64) | 0 |
| bmp-verified-policy-imagecompat | bmp24_8192x2048.bmp | 8.94 | 8.82 | 9.02 / 8.72 | 447 | 1% / 11% | 117 (+64) | 1 |
| bmp-image | bmp24_8192x2048.bmp | 3.22 (0.47x) | 3.10 | 3.21 / 3.04 | 1243 | 8% / 19% NOISY | 164 (+112) | 1 |
| gdk:bmp | bmp24_8192x2048.bmp | 1.61 (0.24x) | 1.49 | 1.62 / 1.49 | 2486 | 22% / 30% NOISY | 105 (+49) | 0 |
| bmp-verified | bmp32_4096x3840.bmp | 6.74 | 6.61 | 6.82 / 6.55 | 594 | 17% / 46% NOISY | 124 (+60) | 0 |
| bmp-verified-policy-strict | bmp32_4096x3840.bmp | 6.74 | 6.61 | 6.82 / 6.55 | 594 | 6% / 21% NOISY | 124 (+60) | 0 |
| bmp-verified-policy-recommended-beta | bmp32_4096x3840.bmp | 6.99 | 6.87 | 6.95 / 6.65 | 572 | 7% / 28% NOISY | 124 (+60) | 0 |
| bmp-verified-policy-gdk | bmp32_4096x3840.bmp | 9.03 | 8.90 | 9.19 / 8.78 | 443 | 10% / 22% NOISY | 125 (+60) | 0 |
| bmp-verified-policy-imagecompat | bmp32_4096x3840.bmp | 9.28 | 9.16 | 9.39 / 8.96 | 431 | 3% / 14% | 124 (+60) | 0 |
| bmp-image | bmp32_4096x3840.bmp | 3.43 (0.51x) | 3.12 | 3.41 / 3.10 | 1165 | 57% / 65% NOISY | 169 (+105) | 1 |
| gdk:bmp | bmp32_4096x3840.bmp | 1.53 (0.23x) | 1.49 | 1.62 / 1.48 | 2621 | 32% / 41% NOISY | 130 (+61) | 0 |
| bmp-verified | bmp8_4096x4096.bmp | 7.99 | 7.87 | 8.05 / 7.50 | 501 | 11% / 35% NOISY | 84 (+57) | 1 |
| bmp-verified-policy-strict | bmp8_4096x4096.bmp | 7.87 | 7.75 | 7.90 / 7.59 | 508 | 6% / 27% NOISY | 84 (+57) | 1 |
| bmp-verified-policy-recommended-beta | bmp8_4096x4096.bmp | 7.99 | 7.75 | 8.06 / 7.62 | 501 | 15% / 42% NOISY | 84 (+57) | 0 |
| bmp-verified-policy-gdk | bmp8_4096x4096.bmp | 11.09 | 10.85 | 11.21 / 10.78 | 361 | 3% / 9% | 84 (+57) | 1 |
| bmp-verified-policy-imagecompat | bmp8_4096x4096.bmp | 10.25 | 10.01 | 10.25 / 9.75 | 390 | 6% / 18% NOISY | 84 (+57) | 0 |
| bmp-image | bmp8_4096x4096.bmp | 2.38 (0.30x) | 1.55 | 2.37 / 1.49 | 1678 | 74% / 70% NOISY | 132 (+105) | 0 |
| gdk:bmp | bmp8_4096x4096.bmp | 2.19 (0.27x) | 2.15 | 2.23 / 2.11 | 1830 | 7% / 19% NOISY | 74 (+47) | 1 |
| bmp-verified | bmp8rle_4096x4096.bmp | 5.48 | 2.86 | 5.54 / 2.73 | 729 | 98% / 59% NOISY | 74 (+47) | 0 |
| bmp-verified-policy-strict | bmp8rle_4096x4096.bmp | 5.42 | 5.25 | 5.42 / 5.15 | 737 | 11% / 25% NOISY | 74 (+47) | 1 |
| bmp-verified-policy-recommended-beta | bmp8rle_4096x4096.bmp | 5.36 | 3.04 | 5.42 / 2.81 | 746 | 86% / 62% NOISY | 74 (+47) | 1 |
| bmp-verified-policy-gdk | bmp8rle_4096x4096.bmp | 5.66 | 2.98 | 5.72 / 2.88 | 706 | 96% / 56% NOISY | 74 (+47) | 0 |
| bmp-verified-policy-imagecompat | bmp8rle_4096x4096.bmp | 6.68 | 6.56 | 6.67 / 6.31 | 599 | 2% / 15% | 74 (+47) | 1 |
| bmp-image | bmp8rle_4096x4096.bmp | 2.30 (0.42x) | 2.26 | 2.40 / 2.20 | 1736 | 11% / 22% NOISY | 122 (+95) | 0 |
| gdk:bmp | bmp8rle_4096x4096.bmp | 4.17 (0.76x) | 4.05 | 4.27 / 4.02 | 959 | 6% / 12% NOISY | 79 (+52) | 0 |
| probe_bmp24_16384x16384.bmp | - | probe, not timed (FileTooLarge) | | | | |
| bmp-image | probe_bmp24_8192x8192.bmp | 3.34 | 3.22 | 3.42 / 3.05 | 1198 | 8% / 26% NOISY | 644 (+448) | 0 |
| gdk:bmp | probe_bmp24_8192x8192.bmp | 1.67 | 1.64 | 1.71 / 1.59 | 2397 | 5% / 19% NOISY | 394 (+193) | 1 |

### TGA corpus

files: 294, verified decodes 252; rounds=5 passes=7 warmup=2

agreement with verified (per incumbent): tga-image: both-ok-equal 230, both-error 35, incumbent-only-ok 7, both-ok-DIFFERENT 22; tga-tinytga: verified-only-ok 101, both-error 31, incumbent-only-ok 11, both-ok-DIFFERENT 22, both-ok-equal 129; gdk:tga: both-ok-equal 230, both-error 27, incumbent-only-ok 15, verified-only-ok 3, both-ok-DIFFERENT 19

| decoder | subset | median CPU ns/px (vs verified) | min CPU ns/px | wall ns/px median / best pass | MB/s RGBA (CPU median) | CPU spread across rounds / wall spread across passes | peak RSS MB (decode delta) | max other-runnable procs |
|---|---|---|---|---|---|---|---|---|
| tga-verified | tga-image (230 files) | 9.23 | 9.16 | 9.47 / 8.97 | 433 | 2% / 11% | 69 (+4) | 1 |
| tga-verified-policy-strict | tga-image (230 files) | 10.02 | 9.66 | 10.14 / 9.44 | 399 | 7% / 16% NOISY | 70 (+4) | 1 |
| tga-verified-policy-recommended-beta | tga-image (230 files) | 9.87 | 9.80 | 10.07 / 9.45 | 405 | 4% / 20% | 69 (+4) | 1 |
| tga-image | tga-image (230 files) | 3.27 (0.35x) | 2.20 | 3.25 / 2.13 | 1224 | 50% / 53% NOISY | 73 (+7) | 0 |
| tga-verified | tga-image [subset where gdk+imagecompat profiles also match] (223 files) | 9.33 | 8.90 | 9.43 / 8.86 | 429 | 14% / 27% NOISY | 69 (+4) | 0 |
| tga-verified-policy-gdk | tga-image [subset where gdk+imagecompat profiles also match] (223 files) | 9.97 | 9.54 | 10.15 / 9.45 | 401 | 13% / 18% NOISY | 70 (+4) | 1 |
| tga-verified-policy-imagecompat | tga-image [subset where gdk+imagecompat profiles also match] (223 files) | 9.97 | 9.54 | 10.11 / 9.57 | 401 | 6% / 12% NOISY | 69 (+4) | 1 |
| tga-verified | tga-tinytga (129 files) | 8.27 | 4.35 | 8.41 / 4.22 | 483 | 94% / 57% NOISY | 33 (+4) | 0 |
| tga-verified-policy-strict | tga-tinytga (129 files) | 9.33 | 4.63 | 9.44 / 4.67 | 429 | 112% / 67% NOISY | 33 (+4) | 0 |
| tga-verified-policy-recommended-beta | tga-tinytga (129 files) | 9.96 | 9.54 | 10.08 / 9.54 | 402 | 96% / 98% NOISY | 33 (+4) | 0 |
| tga-tinytga | tga-tinytga (129 files) | 18.09 (2.19x) | 17.53 | 18.52 / 17.58 | 221 | 9% / 15% NOISY | 34 (+4) | 0 |
| tga-verified | tga-tinytga [subset where gdk+imagecompat profiles also match] (126 files) | 9.02 | 8.67 | 9.08 / 8.56 | 444 | 12% / 17% NOISY | 34 (+4) | 0 |
| tga-verified-policy-gdk | tga-tinytga [subset where gdk+imagecompat profiles also match] (126 files) | 9.72 | 9.58 | 10.06 / 9.53 | 411 | 97% / 102% NOISY | 33 (+4) | 0 |
| tga-verified-policy-imagecompat | tga-tinytga [subset where gdk+imagecompat profiles also match] (126 files) | 18.60 | 9.72 | 18.60 / 9.63 | 215 | 104% / 60% NOISY | 33 (+4) | 0 |
| tga-verified | gdk:tga (230 files) | 9.38 | 9.18 | 9.52 / 8.91 | 426 | 15% / 29% NOISY | 72 (+4) | 1 |
| tga-verified-policy-strict | gdk:tga (230 files) | 9.99 | 9.65 | 10.08 / 9.24 | 400 | 8% / 18% NOISY | 72 (+4) | 0 |
| tga-verified-policy-recommended-beta | gdk:tga (230 files) | 9.79 | 9.52 | 9.91 / 9.46 | 409 | 4% / 10% | 71 (+4) | 0 |
| gdk:tga | gdk:tga (230 files) | 21.20 (2.26x) | 20.39 | 21.07 / 20.33 | 189 | 7% / 26% NOISY | 81 (+9) | 1 |
| tga-verified | gdk:tga [subset where gdk+imagecompat profiles also match] (223 files) | 8.69 | 8.55 | 8.81 / 8.50 | 460 | 3% / 8% | 70 (+4) | 0 |
| tga-verified-policy-gdk | gdk:tga [subset where gdk+imagecompat profiles also match] (223 files) | 9.47 | 9.47 | 9.65 / 9.28 | 422 | 5% / 29% NOISY | 69 (+4) | 1 |
| tga-verified-policy-imagecompat | gdk:tga [subset where gdk+imagecompat profiles also match] (223 files) | 9.61 | 9.26 | 9.73 / 9.34 | 416 | 14% / 22% NOISY | 69 (+4) | 0 |

### TGA large

files: 5, verified decodes 4; rounds=5 passes=5 warmup=1

agreement with verified (per incumbent): tga-image: incumbent-only-ok 1, both-ok-equal 4; tga-tinytga: both-error 1, both-ok-equal 2, verified-only-ok 2; gdk:tga: incumbent-only-ok 1, both-ok-equal 4

| decoder | subset | median CPU ns/px (vs verified) | min CPU ns/px | wall ns/px median / best pass | MB/s RGBA (CPU median) | CPU spread across rounds / wall spread across passes | peak RSS MB (decode delta) | max other-runnable procs |
|---|---|---|---|---|---|---|---|---|
| tga-image | probe_tga24_8192x8192.tga | 2.47 | 2.32 | 2.47 / 2.30 | 1617 | 19% / 41% NOISY | 644 (+448) | 0 |
| gdk:tga | probe_tga24_8192x8192.tga | 18.54 | 18.30 | 18.67 / 18.23 | 216 | 6% / 16% NOISY | 585 (+385) | 0 |
| tga-verified | tga24_4096x4096.tga | 9.66 | 9.18 | 9.77 / 9.02 | 414 | 26% / 33% NOISY | 116 (+64) | 0 |
| tga-verified-policy-strict | tga24_4096x4096.tga | 10.01 | 9.89 | 10.26 / 9.96 | 399 | 6% / 11% NOISY | 117 (+64) | 0 |
| tga-verified-policy-recommended-beta | tga24_4096x4096.tga | 10.13 | 10.01 | 10.41 / 9.87 | 395 | 5% / 11% | 117 (+64) | 0 |
| tga-verified-policy-gdk | tga24_4096x4096.tga | 10.25 | 9.66 | 10.44 / 9.91 | 390 | 12% / 17% NOISY | 116 (+64) | 0 |
| tga-verified-policy-imagecompat | tga24_4096x4096.tga | 10.37 | 10.13 | 10.51 / 10.09 | 386 | 15% / 36% NOISY | 116 (+64) | 0 |
| tga-image | tga24_4096x4096.tga | 2.34 (0.24x) | 1.71 | 2.36 / 1.70 | 1706 | 42% / 42% NOISY | 164 (+112) | 0 |
| tga-tinytga | tga24_4096x4096.tga | 16.93 (1.75x) | 16.45 | 17.12 / 16.32 | 236 | 13% / 40% NOISY | 117 (+64) | 1 |
| gdk:tga | tga24_4096x4096.tga | 18.36 (1.90x) | 17.88 | 18.49 / 17.72 | 218 | 5% / 12% | 154 (+97) | 0 |
| tga-verified | tga24rle_4096x4096.tga | 6.44 | 6.20 | 6.55 / 6.12 | 621 | 33% / 62% NOISY | 84 (+57) | 0 |
| tga-verified-policy-strict | tga24rle_4096x4096.tga | 6.79 | 6.44 | 6.86 / 6.46 | 589 | 7% / 16% NOISY | 83 (+57) | 0 |
| tga-verified-policy-recommended-beta | tga24rle_4096x4096.tga | 6.44 | 6.44 | 6.65 / 6.43 | 621 | 9% / 31% NOISY | 84 (+57) | 1 |
| tga-verified-policy-gdk | tga24rle_4096x4096.tga | 6.79 | 6.32 | 6.87 / 6.32 | 589 | 28% / 42% NOISY | 83 (+57) | 1 |
| tga-verified-policy-imagecompat | tga24rle_4096x4096.tga | 6.56 | 6.44 | 6.65 / 6.39 | 610 | 6% / 14% NOISY | 84 (+58) | 0 |
| tga-image | tga24rle_4096x4096.tga | 4.95 (0.77x) | 4.89 | 5.02 / 4.80 | 809 | 16% / 34% NOISY | 132 (+105) | 0 |
| tga-tinytga | tga24rle_4096x4096.tga | 18.95 (2.94x) | 18.00 | 18.64 / 17.99 | 211 | 7% / 28% NOISY | 84 (+57) | 0 |
| gdk:tga | tga24rle_4096x4096.tga | 18.48 (2.87x) | 18.00 | 18.72 / 18.19 | 216 | 14% / 27% NOISY | 88 (+62) | 1 |
| tga-verified | tga32_4096x3840.tga | 9.79 | 9.54 | 9.72 / 9.53 | 409 | 24% / 30% NOISY | 124 (+60) | 0 |
| tga-verified-policy-strict | tga32_4096x3840.tga | 10.43 | 10.30 | 10.52 / 10.14 | 384 | 2% / 13% | 124 (+60) | 0 |
| tga-verified-policy-recommended-beta | tga32_4096x3840.tga | 10.94 | 10.68 | 11.27 / 10.32 | 366 | 18% / 28% NOISY | 125 (+60) | 0 |
| tga-verified-policy-gdk | tga32_4096x3840.tga | 10.55 | 10.05 | 10.61 / 10.18 | 379 | 32% / 40% NOISY | 124 (+60) | 0 |
| tga-verified-policy-imagecompat | tga32_4096x3840.tga | 10.30 | 10.17 | 10.39 / 10.19 | 388 | 10% / 15% NOISY | 125 (+60) | 0 |
| tga-image | tga32_4096x3840.tga | 1.53 (0.16x) | 1.20 | 1.51 / 1.21 | 2621 | 36% / 39% NOISY | 125 (+60) | 1 |
| gdk:tga | tga32_4096x3840.tga | 18.69 (1.91x) | 18.69 | 19.10 / 18.31 | 214 | 7% / 24% NOISY | 190 (+121) | 1 |
| tga-verified | tga32rle_4096x3840.tga | 6.36 | 6.10 | 6.40 / 6.08 | 629 | 104% / 111% NOISY | 84 (+57) | 1 |
| tga-verified-policy-strict | tga32rle_4096x3840.tga | 6.74 | 6.48 | 6.87 / 6.52 | 594 | 35% / 43% NOISY | 83 (+57) | 1 |
| tga-verified-policy-recommended-beta | tga32rle_4096x3840.tga | 6.61 | 6.48 | 6.78 / 6.43 | 605 | 18% / 24% NOISY | 83 (+57) | 1 |
| tga-verified-policy-gdk | tga32rle_4096x3840.tga | 6.61 | 6.48 | 6.73 / 6.49 | 605 | 6% / 12% NOISY | 83 (+57) | 1 |
| tga-verified-policy-imagecompat | tga32rle_4096x3840.tga | 6.74 | 6.61 | 6.86 / 6.52 | 594 | 15% / 32% NOISY | 83 (+57) | 1 |
| tga-image | tga32rle_4096x3840.tga | 4.20 (0.66x) | 4.01 | 4.17 / 3.94 | 953 | 33% / 45% NOISY | 84 (+57) | 1 |
| gdk:tga | tga32rle_4096x3840.tga | 18.95 (2.98x) | 18.82 | 19.18 / 18.73 | 211 | 3% / 11% | 108 (+80) | 0 |

### ICNS corpus

files: 15701, verified decodes 267; rounds=5 passes=7 warmup=2

agreement with verified (per incumbent): icns-crate: incumbent-only-ok 11519, both-error 3915, both-ok-DIFFERENT 174, verified-only-ok 50, both-ok-equal 43; gdk:icns: both-error 15247, incumbent-only-ok 187, both-ok-DIFFERENT 166, both-ok-equal 101

| decoder | subset | median CPU ns/px (vs verified) | min CPU ns/px | wall ns/px median / best pass | MB/s RGBA (CPU median) | CPU spread across rounds / wall spread across passes | peak RSS MB (decode delta) | max other-runnable procs |
|---|---|---|---|---|---|---|---|---|
| icns-verified | icns-crate (43 files) | 21.67 | 20.00 | 21.68 / 18.69 | 185 | 15% / 35% NOISY | 61 (+0) | 0 |
| icns-crate | icns-crate (43 files) | 11.62 (0.54x) | 9.57 | 11.92 / 9.32 | 344 | 30% / 35% NOISY | 61 (+0) | 1 |
| icns-verified | gdk:icns (101 files) | 17.08 | 16.36 | 17.21 / 16.11 | 234 | 22% / 27% NOISY | 61 (+0) | 0 |
| gdk:icns | gdk:icns (101 files) | 5.09 (0.30x) | 4.47 | 5.16 / 4.38 | 787 | 18% / 40% NOISY | 61 (+0) | 0 |

### ICNS large

files: 3, verified decodes 2; rounds=5 passes=5 warmup=1

agreement with verified (per incumbent): icns-crate: both-ok-equal 2, incumbent-only-ok 1; gdk:icns: both-ok-equal 2, both-error 1

| decoder | subset | median CPU ns/px (vs verified) | min CPU ns/px | wall ns/px median / best pass | MB/s RGBA (CPU median) | CPU spread across rounds / wall spread across passes | peak RSS MB (decode delta) | max other-runnable procs |
|---|---|---|---|---|---|---|---|---|
| icns-verified | icns_legacy128.icns | 23.30 | 22.29 | 23.54 / 22.06 | 172 | 18% / 23% NOISY | 27 (+0) | 0 |
| icns-crate | icns_legacy128.icns | 14.69 (0.63x) | 13.29 | 15.06 / 12.76 | 272 | 42% / 48% NOISY | 27 (+0) | 0 |
| gdk:icns | icns_legacy128.icns | 1.05 (0.04x) | 0.93 | 1.09 / 0.86 | 3827 | 62% / 70% NOISY | 27 (+0) | 0 |
| icns-verified | icns_legacy128_pad32M.icns | 19.82 | 17.44 | 19.58 / 16.37 | 202 | 18% / 33% NOISY | 37 (+0) | 0 |
| icns-crate | icns_legacy128_pad32M.icns | 373.39 (18.84x) | 315.95 | 347.45 / 304.33 | 11 | 43% / 57% NOISY | 69 (+32) | 0 |
| gdk:icns | icns_legacy128_pad32M.icns | 286.87 (14.48x) | 195.31 | 291.31 / 164.23 | 14 | 66% / 70% NOISY | 75 (+34) | 1 |
| icns-crate | icns_png1024.icns | 12.40 | 11.92 | 12.61 / 11.51 | 323 | 24% / 32% NOISY | 27 (+0) | 0 |

### ICO corpus

files: 433, verified decodes 286; rounds=5 passes=7 warmup=2

agreement with verified (per incumbent): ico-crate: both-ok-equal 283, incumbent-only-ok 105, both-error 42, verified-only-ok 2, both-ok-DIFFERENT 1; ico-image: both-ok-equal 250, both-ok-DIFFERENT 32, incumbent-only-ok 105, both-error 42, verified-only-ok 4; gdk:ico: both-ok-equal 275, incumbent-only-ok 51, both-error 96, verified-only-ok 3, both-ok-DIFFERENT 8

| decoder | subset | median CPU ns/px (vs verified) | min CPU ns/px | wall ns/px median / best pass | MB/s RGBA (CPU median) | CPU spread across rounds / wall spread across passes | peak RSS MB (decode delta) | max other-runnable procs |
|---|---|---|---|---|---|---|---|---|
| ico-verified | ico-crate (283 files) | 8.52 | 7.32 | 8.62 / 7.21 | 469 | 19% / 27% NOISY | 28 (+0) | 0 |
| ico-crate | ico-crate (283 files) | 9.97 (1.17x) | 8.88 | 10.53 / 8.89 | 401 | 31% / 33% NOISY | 28 (+0) | 1 |
| ico-verified | ico-image (250 files) | 7.14 | 6.27 | 7.28 / 6.22 | 560 | 17% / 24% NOISY | 28 (+0) | 0 |
| ico-image | ico-image (250 files) | 2.74 (0.38x) | 2.59 | 2.78 / 2.53 | 1461 | 10% / 20% NOISY | 28 (+0) | 1 |
| ico-verified | gdk:ico (275 files) | 7.40 | 7.13 | 7.38 / 7.13 | 541 | 5% / 18% NOISY | 28 (+0) | 0 |
| gdk:ico | gdk:ico (275 files) | 3.06 (0.41x) | 2.97 | 3.15 / 2.88 | 1307 | 18% / 26% NOISY | 28 (+0) | 1 |

### ICO large

files: 3, verified decodes 3; rounds=5 passes=5 warmup=1

agreement with verified (per incumbent): ico-crate: both-ok-equal 3; ico-image: both-ok-equal 3; gdk:ico: both-ok-equal 2, verified-only-ok 1

| decoder | subset | median CPU ns/px (vs verified) | min CPU ns/px | wall ns/px median / best pass | MB/s RGBA (CPU median) | CPU spread across rounds / wall spread across passes | peak RSS MB (decode delta) | max other-runnable procs |
|---|---|---|---|---|---|---|---|---|
| ico-verified | ico_dib256.ico | 6.68 | 6.48 | 6.80 / 6.40 | 599 | 6% / 12% NOISY | 27 (+0) | 0 |
| ico-crate | ico_dib256.ico | 15.26 (2.29x) | 13.22 | 15.55 / 12.66 | 262 | 21% / 28% NOISY | 27 (+0) | 1 |
| ico-image | ico_dib256.ico | 2.54 (0.38x) | 2.54 | 2.64 / 2.50 | 1573 | 24% / 37% NOISY | 27 (+0) | 1 |
| gdk:ico | ico_dib256.ico | 1.85 (0.28x) | 1.66 | 1.95 / 1.70 | 2159 | 18% / 20% NOISY | 27 (+0) | 0 |
| ico-verified | ico_multi6.ico | 6.63 | 6.26 | 6.84 / 5.85 | 603 | 18% / 28% NOISY | 27 (+0) | 0 |
| ico-crate | ico_multi6.ico | 14.13 (2.13x) | 14.13 | 14.61 / 13.77 | 283 | 14% / 28% NOISY | 27 (+0) | 0 |
| ico-image | ico_multi6.ico | 2.74 (0.41x) | 1.57 | 2.79 / 1.50 | 1461 | 100% / 66% NOISY | 27 (+0) | 1 |
| gdk:ico | ico_multi6.ico | 1.82 (0.27x) | 1.59 | 1.81 / 1.58 | 2204 | 29% / 28% NOISY | 27 (+0) | 1 |
| ico-verified | probe_ico_dib1024_dir256.ico | 6.83 | 6.68 | 6.94 / 6.72 | 585 | 5% / 10% | 27 (+0) | 0 |
| ico-crate | probe_ico_dib1024_dir256.ico | 8.11 (1.19x) | 7.63 | 8.26 / 7.27 | 493 | 10% / 19% NOISY | 27 (+0) | 0 |
| ico-image | probe_ico_dib1024_dir256.ico | 2.54 (0.37x) | 2.46 | 2.57 / 2.51 | 1573 | 10% / 12% NOISY | 27 (+0) | 0 |

## Surprises (written for the 2026-11-11 run; the numbers in it are superseded by the Headline and the tables above, the mechanisms named were not re-checked against the migrated cores)

- **TGA uncompressed, verified vs `image` crate: 3x slower** on the corpus (5.8 vs 1.9 CPU ns/px) and 2-4.5x on the 24/32-bit large files (best CPU about 6.3-6.7 vs 1.3-2.4 ns/px; medians 8.2 vs 4.4 and 8.7 vs 1.9 under load). Guess from reading `tga_verus/src/tga.rs`: `decode_raw` converts and stores one pixel per loop iteration, through `write_pixel`, which is four separate `Vec::set` byte stores with bounds checks, plus an orientation-dependent destination offset per pixel. The `image` crate copies whole rows for 32-bit and byte-swaps rows for 24-bit, which the compiler vectorises. The verified code also zero-fills the 4N-byte output first (`vec![0u8; 4 * n]`).
- **BMP 8-bit palette: verified 1.4x slower than `image`, about 3x slower than gdk-pixbuf** (median CPU 9.2 vs 6.6 vs 3.0 ns/px; best 6.9, 6.2, 2.3). Likely the same per-pixel `out.set(o..o+3, ...)` pattern (`dib.rs` around lines 786-789) with a palette lookup per pixel. On the 24/32-bit files verified and `image` are within noise of each other; gdk-pixbuf is faster by roughly 1.5-2x partly because it keeps 3 bytes per pixel for 24-bit input.
- **BMP corpus: verified 1.6x slower than gdk-pixbuf, 1.3x slower than `image`.** The corpus files are small, so fixed per-file costs (header validation, one 4N allocation and zero fill) weigh more.
- **ICNS legacy: gdk-pixbuf about 4x faster than verified** on the 128x128 entry (1.5 vs 6.4 ns/px; the corpus subset shows 1.9x). `icns_verus::rle_decode` builds three separate channel `Vec`s, then `decode_rgb` interleaves into a 3N buffer, and `combine` copies again into 4N: three allocations and three passes against one in gdk-pixbuf.
- **ICNS padded file (`icns_legacy128_pad32M`): `icns` crate 195x and gdk-pixbuf 114x slower per pixel than verified.** Not a throughput result. Both incumbents read or copy the 32 MiB unrelated element; the verified `parse` records the payload range and never touches it. The ns/px figure is normalised by only 16,384 decoded pixels.
- **Verified is faster than three incumbents**: `ico` crate 3.3-4.4x slower than verified (cause not checked in its source), `tinytga` 2.9x and gdk-pixbuf 3.2x slower on TGA (about 16-19 ns/px; probably gdk-pixbuf's incremental loader and per-pixel conversion; not checked), `image` 1.5-1.9x slower on RLE TGA and RLE8 BMP. The corresponding verified RLE paths copy runs without a per-pixel orientation computation.
- **Resource caps**: 64 MiB input and output and 8,192 per side mean the verified BMP/TGA decoders cannot read the 8192x8192 and 16384x16384 files that `image` and gdk-pixbuf decode (peak RSS 645 and 394 MB at 8192x8192). Peak decode RSS delta on 4096x4096 is the exact 64 MiB output for verified, +112 MB for `image` (RGB intermediate plus RGBA), +49 MB for gdk-pixbuf (RGB only).
- **Disagreements**: Of 71 BMP files, 3 differ from `image` and 9 from gdk-pixbuf; of 930 TGA, 23 differ from `image`; of 835 ICNS, 44 differ from the `icns` crate (possibly different icon selection rather than pixel differences; not checked) and of 1,277 ICO, 27 differ from `image`. They are excluded from timing; the per-pair examples are in `results/*-corpus.json` (`agreement`). I did not investigate which side is right.
- **Not covered**: glycin loaders, PNG-bearing ICNS/ICO entries (the verified cores delegate PNG), 16384x16384 timing, multi-threaded gdk-pixbuf use, `gdk_pixbuf_new_from_file`/stream paths (the forced-type loader is the same decoder without sniffing and file I/O).

## A/B after A-STD-4 (`resize` fill), 2026-11-13

Cores with `try_zeroed` filling by `resize(n, 0)` (ERRATA E6) against the pre-migration cores, same method as above (9 interleaved rounds, pinned to core 2, machine idle), run twice: `results/ab-large-resize-run1.json` and `results/ab-large.json`; the push-loop run it replaces is `results/ab-large-2026-11-12-push-loop.json`. Ratios are new/old median CPU ns/px, run 1 then run 2.

- BMP 24-bit 1.01x and 1.00x (push loop: 1.53x), 1.03x and 1.04x on 8192x2048; BMP 8-bit 0.92x and 1.03x; BMP 32-bit 1.82x and 0.91x (inconclusive; the old rounds vary by up to 2x); TGA 24-bit 1.22x and 0.98x; TGA RLE 1.24x and 1.03x; TGA32 1.05x and 0.90x; ICO 0.98-1.11x. The zero-fill regression is gone for BMP, TGA and ICO within noise.
- ICNS legacy stays slower in both runs: 128x128 2.50x and 1.94x, with the 32 MiB padding element 1.88x and 1.60x. The output is 64 KiB, so the fill is not the cause; the cause was not investigated.
- Output hashes are identical to the old cores on every file.
