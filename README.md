# verified-decoders

Image-decoder cores in Rust whose behaviour is **proved** (using Verus) to equal an independent reference model, for every input.

Pure functions, bytes in and pixels (or byte ranges) out: no I/O, no `unsafe`, no dependencies except `vstd`. Written largely by AI agents under a frozen spec; every claim below is backed by a checked artifact in this repo.

Starting with simpler file formats, the eventual goal is to implement everything that already exists in GtkPixbuf, and then get decoders merged, somehow make it usable from within Glycin, and/or get it into the `image` crate.

| Format | Dir | API | Verus result | Vectors | Mutations that break the proof |
|---|---|---|---|---|---|
| ICNS (container + legacy RLE/mask) | `icns/` | `parse`, `decode_rgb`, `decode_mask`, `combine` | 315 verified | 111 | 10/10 |
| TGA | `tga/` | `decode` | 642 verified | 169 | 15/15 (executable-side) |
| DIB / BMP / ICO-entry bitmap | `dib/` | `decode(bytes, mode)`, `decode_with_steps` | 1557 verified (spec 1.1; 1.0 is `dib/spec-frozen`) | 302 | 43/44 verifier kills + 1 audit kill |
| ICO / CUR container | `ico/` | `parse`, `decode_entry` (spec + oracle only, proofs pending) | - | 217 | - |

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

Every deliberate difference from gdk-pixbuf is listed in `<fmt>/spec-frozen/DESIGN-NOTES.md` (headers there say "draft": the text is the frozen one).

## What is proved

For **all** input slices (`decode` has no precondition), against the frozen spec's total reference model written independently of the code (`*/model.rs`):
- no panic, no out-of-bounds access, no integer overflow, no `unsafe`; termination (explicit decreasing measures);
- **exact functional correctness**: result (every output byte or the exact error, in the spec's error precedence) equals the model;
- bounded allocation: one output buffer of exactly the size the header implies, within the 64 MiB cap, allocated only after header checks;
- RLE/raw loops: the exact first error, ghost final cursor and partial output also match the model on failure paths (TGA T6/T7, DIB T6-T8);
- **DIB only**: linear work bound, `steps <= 16*(|b|+N)`, via a ghost counter charged on every real loop iteration.
Theorem statements: `*/THEOREMS.md`; per-milestone proof reports: `*/PROOF-REPORT-*.md`; effort: `*/LEDGER.md`. Each model is also checked against every small spec vector (`model_vectors.rs`); the few very large vectors (up to 64 MiB) are covered by symbolic lemmas or runtime tests. Specs, vectors and an independent Python oracle (which also checks every vector): `*/spec-frozen/`, `*/spec-history/`.

## What is not proved (read this)

- **The spec.** Proofs show code = model; the model is derived from the spec text. Fidelity to real files rests on independent review, the vectors (all synthetic) and oracle fuzzing (17.7M differential cases for ICNS, 7.6k for TGA, 10.3k for DIB; zero disagreements), not proof. Only DIB was measured on real files.
- Allocation failure/OOM, stack use, physical memory; the single-allocation rule is a *ghost* count, so an extra `vec!` is not caught by the verifier: `dib/tools/audit_alloc.sh` is a source audit (mutation 20 is an audit kill, not a proof kill). TGA allocates the output before the ID/map availability check (bounded, allowed by the spec).
- Work bounds for ICNS/TGA (only termination). No i686 verifier run: Verus treats `usize` as 32-or-64-bit unless a crate fixes it, and none does, so proofs are width-generic; this is not a 32-bit build.
- Soundness of Verus, vstd, Z3 and rustc; the SMT solver budgets were not raised (one `rlimit(80)` on one TGA lemma).
- The decoders are cores. Selection logic, stream handling, PNG/JPEG 2000 payload decoding and any glib/glycin glue are outside the proofs.
- Models and mutations: model-side and implementation-side mutations were both run; review caught real gaps in early TGA proofs (documented in `reviews/`), later fixed and re-verified.

## Build and check

Needs Rust 1.99.1 and [Verus](https://github.com/verus-lang/verus) 0.2026.11.02 (`vstd` pinned in each `Cargo.toml`).
```
cd icns/icns_verus && cargo test && cargo verus verify     # same for tga/tga_verus, dib/dib_verus
```
`./verify.sh "<note>"` in each project root wraps the verifier and logs runs to `RUNLOG.tsv` (run logs omitted from this repo; mutation `failure.log`/`result.txt` kept under `*/mutation/`). The Creusot variant needs Creusot 0.14.0 (`cargo creusot`).
`reviews/`: independent reviews (Sol, Sonnet, Fable) and the Verus-vs-Creusot comparison. `*/BRIEF.md`, `LEDGER.md`: how the agents were run and what it cost.

MIT licensed (`LICENSE`). Spec fidelity notes cite gdk-pixbuf (LGPL) behaviour; no code was copied from it.
