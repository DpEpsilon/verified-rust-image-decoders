## Performance

The verified decoders are slower than the incumbents on most 24/32-bit BMP and uncompressed TGA rows, and faster than gdk-pixbuf, `tinytga` and the `ico` crate on TGA and ICO. Method, corpora and all tables are in `bench/README.md`; this is the summary.

**What is measured.** Decode only, in process, input already in memory, CPU time (`CLOCK_PROCESS_CPUTIME_ID`) in ns per output pixel, RGBA8 output, each benchmark process pinned to one core (`taskset -c 2`), 5 interleaved rounds. Incumbents: gdk-pixbuf 2.42.12 (C, `-O2`), `image` 0.25.10, and the `icns` 0.3, `ico` 0.3 and `tinytga` 0.5.0 crates. Outputs are compared by SHA-256 before timing; a pair that disagrees is not timed.

Large generated files, best of 5 rounds, CPU ns/px (lower is better). Run of 2026-11-12, **measured with the push-loop guard (before ERRATA E6)**:

| file (4096x4096 unless noted) | verified | `image` | gdk-pixbuf |
|---|---|---|---|
| BMP 24-bit | 6.6 | 3.2 | 1.6 |
| BMP 32-bit (4096x3840) | 6.6 | 3.1 | 1.5 |
| BMP 8-bit palette | 7.9 | 1.6 (rounds vary 74%) | 2.2 |
| TGA 24-bit raw | 9.2 | 1.7 | 17.9 |
| TGA 32-bit raw (4096x3840) | 9.5 | 1.2 | 18.7 |
| TGA 24-bit RLE | 6.2 | 4.9 | 18.0 |
| ICO 256x256 32-bit | 6.5 | 2.5 | 1.7 |

The verified BMP 24/32-bit decoders take 2.1x the time of `image` and 4.2-4.4x that of gdk-pixbuf; uncompressed TGA takes 5-8x that of `image` and about half that of gdk-pixbuf and `tinytga` (17 ns/px). TGA RLE is 1.3x `image` (inconclusive, see below) and 2.9x faster than gdk-pixbuf. The ICO core is 2.2x faster than the `ico` crate, 2.6x slower than `image` and 3.9x slower than gdk-pixbuf. BMP RLE8 and TGA32 RLE rounds differ by about 100%, so their ordering against `image` is not established. ICNS legacy (128x128) is slower than the `icns` crate and than gdk-pixbuf, with 18-42% round spreads; ICNS figures differ between the tables and the headline text of `bench/README.md`, so none are quoted here.

**Cost of fallible allocation.** Same-session A/B against the pre-migration cores (9 interleaved rounds, outputs byte-identical):
- With the `push` fill, 64 MiB outputs cost 1.3-1.6x the old `vec![0; n]` (BMP 24-bit 4.29 to 6.56 ns/px median, TGA 24-bit 7.03 to 9.18). The old code got zeroed pages from the allocator; the loop wrote 64 MiB byte by byte, 47.8 ms against 6.6 ms for `resize(n, 0)` in `bench/fillbench/RESULTS.md`.
- With the `resize` fill (ERRATA E6, trusted item A-STD-4), two runs give 1.00-1.04x on BMP 24-bit, 0.92-1.03x on BMP 8-bit, 0.98-1.24x on TGA 24-bit, and 0.98-1.11x on ICO. BMP 32-bit (1.82x, then 0.91x) is inconclusive because the old cores' rounds vary by up to 2x. The large-file table above has not been re-measured against the incumbents after this change; the A/B medians for BMP 24-bit are 4.2 ns/px, against 6.6 in the table.
- ICNS legacy stays 1.6-2.5x slower than the pre-migration core after the change (128x128: 2.50x and 1.94x; with the 32 MiB padding element: 1.88x and 1.60x). The output is 64 KiB, so the fill is not the cause; the cause was not investigated.

**Conditions.** One gVisor sandbox with 4 CPUs, no bare-metal run. The CPU clock ticks every 10 ms, so each reading is a total over several passes. Round-to-round spreads are 5-35% on large files and up to about 100% on some RLE rows; read ratios under about 1.5x as inconclusive. gdk-pixbuf output is not converted to RGBA inside the timed region, which favours it on 24-bit inputs (3 bytes per pixel written). The corpora were collected from several sources under mixed licences and are not a representative sample of real-world files; the large files are synthetic (gradient plus noise). The verified decoders reject inputs above 8192 px per side or 64 MiB, which `image` and gdk-pixbuf decode (8192x8192 BMP/TGA probes ran on the incumbents only). Absolute numbers will differ on bare metal.

