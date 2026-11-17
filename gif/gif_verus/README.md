# gif_verus

GIF decoder core for GIF87a and GIF89a, specified by GIF-CORE-SPEC draft 0.2 (`../spec-draft/SPEC.md`), with the policy record of `../policy-draft/OPTIONS.md`. Milestones M1 (plain implementation) and M3 (in progress): the header and block parser are verified against the model (`src/model.rs`, `parse_header` = `MH` exactly, the structural sub-block scan, the G6.1 Graphic Control and the G6/G7 extension dispatch); the rest is M1 plain Rust. The remaining model, contracts and proofs come in M4–M5 and P1; `STUCK.md` tracks them. `no_std + alloc`, no `unsafe`, no recursion; every heap allocation goes through the sealed `alloc_guard`.

## API

```rust
pub fn try_decode_still(b: &[u8]) -> Result<Still, TryError>;
pub fn try_decode_still_with_policy(b: &[u8], p: Profile) -> Result<(Still, bool), TryError>;

impl<'a> Frames<'a> {
    pub fn try_new(b: &'a [u8]) -> Result<Frames<'a>, TryError>;
    pub fn try_new_with_policy(b: &'a [u8], p: Profile) -> Result<Frames<'a>, TryError>;
    pub fn info(&self) -> &Info;
    pub fn next_frame(&mut self) -> Option<Result<FrameView<'_>, TryError>>;
}

pub fn plays(loop_count: Option<u16>) -> Plays;
impl Plays { pub fn total_plays(self) -> Option<u32>; }
```

Types:

- `Still { width: u16, height: u16, rgba: Vec<u8> }`: the logical screen after the first image is drawn (G14), `4·width·height` bytes of RGBA8, top row first, non-premultiplied, alpha 0 or 255. Bytes after the first image's block terminator are not examined. The `bool` of the policy variant is `recovered`.
- `Info { width: u16, height: u16, version: Version, background_index: u8, aspect: u8, loop_count: Option<u16> }`: fixed by the constructor. `loop_count` is the raw NETSCAPE2.0 value found before the first image descriptor (every sub-block scanned, the last 3-byte sub-block with id 1 wins); background index and aspect are raw and unused.
- `FrameView<'f> { rgba: &'f [u8], rect: (u16, u16, u16, u16), delay_cs: u16, disposal: u8, user_input: bool, transparent: Option<u8>, interlaced: bool, recovered: bool }`: the whole canvas after the frame, borrowed from the decoder, plus the frame's fields. `rect` is `(left, top, width, height)`; `delay_cs` is raw hundredths of a second (mapping 0 or small delays is the caller's policy); `disposal` is raw (4–7 occur only under RecommendedBeta, which composites them as 1).
- `Plays = Once | Forever | Repeats(u16)`: `None ↦ Once`, `Some(0) ↦ Forever`, `Some(n) ↦ Repeats(n)`; `total_plays` is 1, none, or `n + 1` (G20).
- `Profile = Strict | RecommendedBeta`, `#[non_exhaustive]`. Strict is the supported GIF subset of SPEC.md. RecommendedBeta (provisional, unfrozen) clips frames to the screen, keeps disposal 4–7 as 1, lets a duplicate Graphic Control replace the pending one, applies a Graphic Control that precedes a Plain Text or unknown graphic-rendering block to the next image, ignores an orphan Graphic Control, Application/Plain Text block sizes and trailing bytes, accepts a missing Trailer at EOF, stops each frame at its full raster, and accepts minimum code sizes 2–11 (literals above 255 rejected) and 1 for a one-pixel frame. On every input Strict accepts, RecommendedBeta returns the identical result with `recovered = false`. The compatibility profiles (`GdkCompat-2.42.12`, `ImageCrateCompat-0.25.10`) arrive with milestone P2.
- `TryError = Format(Error) | OutOfMemory(OutOfMemory)`; `Error` is the closed list of 33 names of G16, `Error::name()` and `TryError::name()` give the exact spelling. `OutOfMemory::site()` is `AllocSite::{Canvas, LzwTable, Saved}`, `elements()` the request size.

`Frames` is not an `Iterator`, because each view borrows the single canvas:

```rust
let mut dec = Frames::try_new(bytes)?;          // G2–G4, Canvas + LzwTable, walk to the first image
let info = *dec.info();
while let Some(f) = dec.next_frame() {
    let f = f?;                                  // Some(Err) once, then None: fused
    show(f.rgba, f.delay_cs);
}
```

An error in G2–G4 or in the blocks before the first image descriptor is the constructor's error; an error inside the first image or later is returned by `next_frame` after the frames before it. A stream with no image and a valid Trailer yields zero frames; `try_decode_still` returns `NoImage` for it.

## Limits and allocation

The caps are the named constants of `src/limits.rs` (G1): `MAX_INPUT` 256 MiB (2^28), `MAX_DIMENSION` 65,535 (the wire limit), `MAX_OUTPUT` `4·W·H ≤ 64 MiB`, `MAX_FRAMES` 65,536, `MAX_TOTAL_PIXELS` 2^30. `MAX_OUTPUT` stays at 64 MiB (2^26; `W·H ≤ 16,777,216` pixels). The shared guard's `MAX_BYTES` is now 1,073,741,824 bytes (1 GiB), under owner-approved alloc-delta erratum E16 (2026-11-17); raising GIF's canvas cap needs its own erratum and re-verification.

| Site | Size | When |
|---|---|---|
| `Canvas` | `4·W·H` bytes, zero-filled | `try_decode_still*` and `Frames::try_new*`, after G2–G4 |
| `LzwTable` | one 20,480-byte `try_zeroed` region: prefix codes (LE16 pairs, bytes `[0, 8192)`), suffix bytes (`[8192, 12288)`), string lengths (LE16 pairs, `[12288, 20480)`), 4,096 each | same call, after `Canvas` |
| `Saved` | `4·W·H` bytes | the first frame with disposal 3, once per `Frames` (never for the still) |

Peak heap per decoder is `8·W·H + 20,480` bytes, independent of the frame count. There is no index buffer (indices go straight to the canvas through the interlace map) and no per-frame copy (an error leaves the decoder fused and the canvas unexposed). The guard's `mod.rs` is byte-identical to the other cores' (SHA-256 `add634bc…`); `sites.rs` is GIF's own (`../tools/audit_alloc.sh` pins both).

## Modules

- `gif.rs`: entry points, `Frames`, `Error` (inside `verus!`), `plays`.
- `parse.rs`: G2–G9 (header, colour tables by offset, extensions with the `UnsupportedGraphicControlTarget` check, descriptors, the structural scan of the image data, NETSCAPE2.0 rule A).
- `lzw.rs`: G10 over the caller-provided 20,480-byte `LzwTable` region and a bit reader over the sub-blocks; indices go to a `Sink` with their pixel position. Literal codes are decided by rule (no per-frame table initialisation); strings are emitted from the prefix chain backwards using the stored lengths.
- `compose.rs`: G11 `interlace_row`, G12 drawing (`Drawer`, the `Sink`), disposal 2/3, `Saved`, clipping.
- `policy.rs`: `Profile` and the 22-field record plus `loop_scan`.
- `limits.rs`: the G1 constants.
- `alloc_guard/`: the sealed guard (`mod.rs`) and GIF's sites (`sites.rs`: `AllocSite`, `TryError`, `log_tied`).

## Tests

```
cd gif/gif_verus
nice -n 10 cargo test --offline --release -j1
```

- `tests/conformance.rs`: all 373 records of `../spec-draft/VECTORS.json` (regenerated by `gen_vectors.py` if missing), including the 256 MiB padding and 65,536-frame records, and `tests/data/PROBES.json` (1,316 records over the 329 inputs `../policy-draft/validate.py` checks, Strict and RecommendedBeta, still and frames; regenerated by `tools/gen_probe_expect.py` if missing). `GIF_RECORDS=file` runs any further record file, for example the differential mutants of `tools/gen_mutants.py`. The two `@ImageCrateCompat-0.25.10` records run under a test-only record (feature `test-records`, enabled only through the crate's dev-dependency on itself).
- `tests/fault_injection.rs`: a thread-local failing allocator checks the exact physical request list against `Req`, zero reallocations, the peak bound, and `OutOfMemory` with the right site and size when each reservation fails in turn.
- `tests/stress.rs`: a generated 60×60 stream with 37,239 frames; checks every frame's pixels, the frame count and a working-memory peak of 49,280 bytes, and prints the decode time.
