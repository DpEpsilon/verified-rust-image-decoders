# verified-decoders

Image-decoder cores in Rust whose behaviour is **proved** (using Verus) to equal an independent reference model, for every input.

Pure functions, bytes in and pixels (or byte ranges) out: no I/O, no `unsafe` in the cores, no dependencies except `vstd` (ICO also depends on the DIB core). Each core is `#![no_std]` and uses `alloc`; its public entry points are the fallible `try_*` functions, which return `TryError { Format(Error), OutOfMemory(..) }` when a buffer reservation fails instead of aborting. Written largely by AI agents under a frozen spec; every claim below is backed by a checked artifact in this repo.

Starting with simpler file formats, the eventual goal is to implement everything that already exists in GtkPixbuf, and then get decoders merged, somehow make it usable from within Glycin, and/or get it into the `image` crate.

| Format | Dir | Public entry points | Verus result (all 0 errors) | Vectors | Mutations that break the proof (campaigns predate the allocation change) |
|---|---|---|---|---|---|
| ICNS (container + legacy RLE/mask) | `icns/` | `try_parse`, `try_decode_rgb`, `try_decode_mask`, `try_combine` | 412 verified | 111 | 10/10 |
| TGA | `tga/` | `try_decode`, `try_decode_with_policy`, `try_decode_with_steps_logged` | 974 verified | 169 | 15/15 (executable-side) |
| DIB / BMP / ICO-entry bitmap | `dib/` | `try_decode`, `try_decode_with_policy`, `try_decode_with_steps`, `try_decode_with_steps_logged` | 1764 verified (spec 1.1; 1.0 is `dib/spec-frozen`) | 302 | 43/44 verifier kills + 1 audit kill |
| ICO / CUR container (over the DIB core) | `ico/` | `try_parse`, `try_parse_with_steps`, `try_decode_entry`, `try_decode_entry_with_steps` | 346 verified | 217 | 32 verifier kills + 1 audit kill of 33, executable-side (m17 reworked as m34) |

The infallible names (`parse`, `decode`, `decode_with_steps`, `decode_entry`, ...) were removed in the allocation change (`alloc-delta/`, frozen spec `alloc-delta/SPEC.md`, errata `alloc-delta/ERRATA.md`); older notes and proof reports that name them describe the pre-change cores. The counts above are the latest `RUNLOG.tsv` runs (DIB 556, TGA 231, ICNS 51, ICO 175, all on 2026-11-13).

`leniency/`: differential harness comparing our cores with gdk-pixbuf, the `image` crate, glycin's loaders and other crates, and the BMP/DIB leniency matrix (`leniency/bmp/LENIENCY.md`); TGA, ICNS and ICO results follow.
`alt/icns-creusot/`: the same ICNS spec proved with Creusot/Why3 (comparison only; Verus won on effort, see `reviews/COMPARISON.md`). `tools/diff-harness/`: differential test of both ICNS cores against a Python oracle.

## What each decoder accepts

**ICNS** (`ICNS-CORE-SPEC 1.1`). Container parse plus legacy `is32/il32/ih32/it32` RLE colour and `s8mk/l8mk/h8mk/t8mk` masks (16/32/48/128 px), RGB + alpha combine. PNG and JPEG 2000 entries are *not decoded*: they are classified and returned as untouched byte ranges (`opaque_candidate`). ARGB (`ic04/ic05`) and 1/4/8-bit types are out of scope. Limits: 64 MiB input, 4096 entries.
- Strict, by owner decision: declared length must equal the slice (`TruncatedFile`/`TrailingFileBytes`); 1-7 stray bytes after the last entry are `TruncatedEntryHeader`; RLE must consume its payload exactly (`RleTrailingBytes`, so a padding byte is rejected and an uncompressed 4N-byte ARGB payload in an `is32` slot is an error); `it32` strips a 4-byte zero prefix only when present.
- Lenient: unknown types, duplicates and empty payloads are kept; selection of which entries to combine is the caller's.

**TGA** (`TGA-CORE-SPEC 1.0`). Image types 1/2/3/9/10/11 (colour-mapped, true-colour, grey; raw and RLE), depths 8/15/16/24/32. Output: top-row-first, non-premultiplied RGBA8. Limits: 64 MiB in/out, 8192 px per side.
- Lenient (differs from gdk-pixbuf): true-colour/grey images carrying a colour map are accepted (map skipped); 15-bit and 16-bit-indexed supported; 16-bit true-colour with attribute count 1 and bit 15 clear is opaque.
- Strict: attribute count is authoritative (32-bit with count 0 gets alpha 255, so unmarked alpha is dropped; 24-bit with count 8 and other off-policy counts are `InvalidAttributeBits`); RLE packet past the pixel count is an error (gdk-pixbuf clamps); zero-length colour map flag is `InvalidColorMap`; non-zero interleave rejected; trailing bytes rejected unless a TGA 2.0 footer follows; indexed maps use the absolute `first` index.

**DIB** (`DIB-CORE-SPEC 1.1`; 1.0 = the same without the fourth mode). Four modes: `BmpFile`, `BareDib`, `IcoEntryDib` (height/2 + 1-bpp AND mask) and `IcoEntryDibSlack` (same, but bytes after the AND mask are ignored). Headers 12/16/40/52/56/64/108/124; 1/4/8/16/24/32 bpp; BI_RGB, BI_BITFIELDS, RLE4/RLE8; top-down and bottom-up. Output RGBA8. Limits: 64 MiB, 8192 px per side. The ICO directory/PNG choice is a separate (future) layer.
- Lenient (`BmpFile` only): `bfSize` 0 or <= file length, bytes after the pixel array ignored (covers V5 profile data); `biClrUsed` > 0 at 16/24/32 bpp skipped. 32-bpp ICO alpha uses the Windows rule (any non-zero XOR alpha -> AND ignored). ICO `biSizeImage` may be 0, XOR or XOR+AND.
- Strict: BITFIELDS masks must be non-zero, contiguous, disjoint; RLE must end with EOB, runs may not cross rows, deltas must land in-bounds; 2-bpp, BI_ALPHABITFIELDS, JPEG/PNG-in-BMP, OS/2 Huffman/RLE24 unsupported; slack bytes inside an ICO entry or BareDib are `TrailingBytes`; a short last-row pad is `TruncatedPixels`; indexed short palettes give `PaletteIndexOutOfRange`.
- Measured on 84 Pillow test files (deliberately odd; not redistributed, `dib/tools/fetch_corpus.sh`): 58 accepted, 24 rejected by design, 2 outside the core.

**ICO/CUR** (`ICO-CORE-SPEC 1.0`, `ico/spec-frozen/`). Standalone `.ico`/`.cur` byte slices: directory parse into entry records (invalid entries keep a defect status instead of failing the parse), then `try_decode_entry` for DIB entries by calling the DIB core in `IcoEntryDibSlack` mode. PNG entries are classified, not decoded. Limits: 64 MiB input, 1,024 directory entries. The ICO core lives in `ico/` (`ico/ico_verus`) and depends on `dib/dib_verus` by path.

Every deliberate difference from gdk-pixbuf is listed in `<fmt>/spec-frozen/DESIGN-NOTES.md` (headers there say "draft": the text is the frozen one).

## What is proved

For **all** input slices (the `try_*` entry points have no precondition on the bytes), against the frozen spec's total reference model written independently of the code (`*/model.rs`):
- no panic, no out-of-bounds access, no integer overflow, no `unsafe`; termination (explicit decreasing measures);
- **exact functional correctness**: result (every output byte or the exact error, in the spec's error precedence) equals the model;
- bounded allocation, as a verified ghost log of reservations: one output buffer of exactly the size the header implies (ICNS and ICO also reserve an entry list), within the 64 MiB cap, reserved only after header checks; a failed reservation returns `OutOfMemory` with no partial output;
- RLE/raw loops: the exact first error, ghost final cursor and partial output also match the model on failure paths (TGA T6/T7, DIB T6-T8);
- **DIB only**: linear work bound, `steps <= 16*(|b|+N)`, via a ghost counter charged on every real loop iteration.
Theorem statements: `*/THEOREMS.md`; per-milestone proof reports: `*/PROOF-REPORT-*.md`; effort: `*/LEDGER.md`. Each model is also checked against every small spec vector (`model_vectors.rs`); the few very large vectors (up to 64 MiB) are covered by symbolic lemmas or runtime tests. Specs, vectors and an independent Python oracle (which also checks every vector): `*/spec-frozen/`, `*/spec-history/`.

## What is not proved (read this)

- **The spec.** Proofs show code = model; the model is derived from the spec text. Fidelity to real files rests on independent review, the vectors (all synthetic) and oracle fuzzing (17.7M differential cases for ICNS, 7.6k for TGA, 10.3k for DIB; zero disagreements), not proof. Only DIB was measured on real files.
- Stack use and physical memory. The reservation log is *ghost*: the verifier does not see allocations, so an extra `vec!` outside the guard is caught only by the source lint `*/tools/audit_alloc.sh` (DIB mutation 20 is an audit kill, not a proof kill). The lint matches method names in text; the production call-graph audit that `alloc-delta/SPEC.md` section 6 requires does not exist.
- The guard `alloc_guard/mod.rs` (byte-identical in all four cores, SHA-256 `26e4c832d2df80b782d157e68ade31066cc82ca2774b4c714a6df0f4edaf86b7`) is hash-pinned text, not a verified body. It rests on trusted std behaviour, A-STD-1 to A-STD-4: after `try_reserve_exact(n)` returns `Ok` the capacity is at least `n` (A-STD-1), `push` does not reallocate below capacity (A-STD-2, used by ICNS and ICO for entry lists), `Err` means the allocator failed (A-STD-3), and `resize(n, 0)` on an empty `Vec<u8>` with capacity at least `n` does not reallocate (A-STD-4). A-STD-4 is observed behaviour of std 1.99.1, not a documented guarantee (ERRATA E6); it replaced a documented-only `push` loop because the loop cost 47.8 ms against 6.6 ms for a 64 MiB buffer. A reallocation inside `resize` would be infallible and abort, and cannot be detected after the fact.
- TGA allocates the output before the ID/map availability check (bounded, allowed by the spec).
- Work bounds for ICNS/TGA (only termination). No i686 verifier run: Verus treats `usize` as 32-or-64-bit unless a crate fixes it, and none does, so proofs are width-generic; this is not a 32-bit build.
- Soundness of Verus, vstd, Z3 and rustc. Solver budgets: the live DIB source has two `#[verifier::rlimit(20)]` attributes (`parse_header`, `dib_finish`, added at run 556); the live TGA, ICNS and ICO sources have none.
- The decoders are cores. Selection logic, stream handling, PNG/JPEG 2000 payload decoding and any glib/glycin glue are outside the proofs.
- Models and mutations: model-side and implementation-side mutations were both run; review caught real gaps in early TGA proofs (documented in `reviews/`), later fixed and re-verified.

## Performance

The verified decoders are slower than `image` on every row of the table below (TGA RLE by 1.3x, inconclusive) and slower than gdk-pixbuf on BMP and ICO; they are faster than gdk-pixbuf and `tinytga` on TGA and faster than the `ico` crate. Method, corpora and all tables are in `bench/README.md`; this is the summary.

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
- With the `resize` fill (ERRATA E6, trusted item A-STD-4), two runs give 1.00-1.04x on BMP 24-bit, 0.92-1.03x on BMP 8-bit, 0.90-1.24x on the five TGA rows (24-bit: 1.22x, then 0.98x), and 0.98-1.11x on ICO. BMP 32-bit (1.82x, then 0.91x) is inconclusive because the old cores' rounds vary by up to 2x. The large-file table above has not been re-measured against the incumbents after this change; the A/B medians for BMP 24-bit are 4.2 ns/px, against 6.6 in the table.
- ICNS legacy stays 1.6-2.5x slower than the pre-migration core after the change (128x128: 2.50x and 1.94x; with the 32 MiB padding element: 1.88x and 1.60x). The output is 64 KiB, so the fill is not the cause; the cause was not investigated.

**Conditions.** One gVisor sandbox with 4 CPUs, no bare-metal run. The CPU clock ticks every 10 ms, so each reading is a total over several passes. Round-to-round spreads are 5-35% on large files and up to about 100% on some RLE rows; read ratios under about 1.5x as inconclusive. gdk-pixbuf output is not converted to RGBA inside the timed region, which favours it on 24-bit inputs (3 bytes per pixel written). The corpora were collected from several sources under mixed licences and are not a representative sample of real-world files; the large files are synthetic (gradient plus noise). The verified decoders reject inputs above 8192 px per side or 64 MiB, which `image` and gdk-pixbuf decode (8192x8192 BMP/TGA probes ran on the incumbents only). Absolute numbers will differ on bare metal.

## Build and check

Needs Rust 1.99.1 and [Verus](https://github.com/verus-lang/verus) 0.2026.11.02 (`vstd` pinned in each `Cargo.toml`).
```
cd icns/icns_verus && cargo test && cargo verus verify     # same for tga/tga_verus, dib/dib_verus, ico/ico_verus
```
`./verify.sh "<note>"` in each project root (`icns/`, `tga/`, `dib/`, `ico/`) wraps the verifier and logs runs to `RUNLOG.tsv` (run logs omitted from this repo; mutation `failure.log`/`result.txt` kept under `*/mutation/`). The Creusot variant needs Creusot 0.14.0 (`cargo creusot`).
`reviews/`: independent reviews (Sol, Sonnet, Fable) and the Verus-vs-Creusot comparison. `*/BRIEF.md`, `LEDGER.md`: how the agents were run and what it cost.

MIT licensed (`LICENSE`). Spec fidelity notes cite gdk-pixbuf (LGPL) behaviour; no code was copied from it.
