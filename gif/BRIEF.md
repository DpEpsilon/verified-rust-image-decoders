# Brief: verified GIF decoder core (Verus) — scope

Status: specification phase. The draft spec is `spec-draft/` (GIF-CORE-SPEC draft 0.2, oracle, 373 vectors), the leniency catalogue is `policy-draft/` (22-field record plus the metadata field `loop_scan`, four profiles), and the proof plan is `PLAN.md`. The owner decided all 18 questions of `policy-draft/QUESTIONS.md` on 2026-11-14 (changelog `DECISIONS.md`). Nothing is frozen and no Rust exists yet. `spec-draft/` and Strict are frozen after M0 (`THEOREMS.md` and its review, hashes in a `FREEZE.txt` each); RecommendedBeta keeps its `Beta` suffix and stays unfrozen until its real-corpus pass is recorded (Q9). At the freeze this brief gains the binding rules of `../dib/BRIEF.md`, `../dib/BRIEF-POLICY.md` and `../alloc-delta/SPEC.md` unchanged.

## Goal

A feature-complete GIF decoder core with the guarantees the DIB/BMP core has: safety (T1) for every input and every policy record; exact equality with the frozen model (T2 and the integrated T9/T10), including the first error; allocation bounds (T3) through the sealed `alloc_guard` with `try_*` entry points, `OutOfMemory` outside the format error set, and the G/Prog/PreAlloc theorem shape; termination (T11); a linear work bound (T12); and the policy machinery: a closed `recovered` event list (E1–E4), the Strict bridge (`policy(Strict) = core model` for every input), obligation E (`recovered` iff an event occurred) and obligation R, stated for both meanings of Strict-accepted (Q3): RecommendedBeta returns the identical still for every input Strict's still entry point accepts, and the identical info and frames, ending at the Trailer, for every stream Strict accepts, always with `recovered=false`. Strict is the supported GIF subset, not complete GIF89a (Q7). The crate is `no_std + alloc`; one crate, with parser, LZW, compositor and policy layer as separate modules and separately proved models (Q10).

## Features in scope

- **Versions.** GIF87a and GIF89a; other version strings rejected in every profile (Q12). The version does not gate 89a blocks.
- **Logical Screen Descriptor.** Screen size (zero rejected; any side up to 65,535 within the area cap), global table flag and size, colour resolution and sort flag (ignored), background colour index and pixel aspect ratio (exposed raw, unused).
- **Colour tables.** Global and local, `2^(n+1)` entries; local overrides global for one image; a frame with neither is an error in every profile (Q11); sort flags ignored.
- **Image Descriptor.** Position and size (zero size rejected; frames must lie inside the screen in Strict, clipped with checked arithmetic in RecommendedBeta, Q18), interlace flag, reserved bits ignored.
- **Interlacing.** The four-pass row order for every height, including heights below 8 where passes are empty.
- **LZW.** Minimum code size 2–8 in Strict; 2–11 with literals above 255 rejected, and 1 only for a one-pixel frame, in RecommendedBeta (Q6); codes LSB-first from 3 to 12 bits, Clear and End of Information codes, optional leading Clear, the KwKwK case, widening when the table reaches `2^width`, the full 4096-entry table with deferred clear, and exact end conditions (too few pixels, missing End of Information, surplus pixels, bytes after End of Information) with the incumbents' leniencies as policy fields (`lzw_tail`, `short_frame`, `lzw_invalid`, `lzw_min`, `empty_first_subblock`, `image_data`).
- **Graphic Control Extension.** Disposal 0–3 (4–7 rejected in Strict, treated as 1 in RecommendedBeta with the raw value reported, Q13), transparency index (an index outside the table is allowed when transparent), delay in hundredths (raw, Q16), user input flag (exposed), exact block size, at most one per image, applies to the next image only; a pending Graphic Control that meets a Plain Text or unknown graphic-rendering block is `UnsupportedGraphicControlTarget` in Strict and is carried to the next image in every other profile (Q7).
- **Application Extension.** NETSCAPE2.0 loop count: the raw value before the first image, every sub-block scanned, last id-1 sub-block wins (Q15); `plays()` gives its meaning. The buffering sub-block, XMP, ICC and other applications parsed and skipped; no payload exposed (Q14).
- **Comment and Plain Text Extensions.** Parsed and skipped; Plain Text is not rendered and nothing is exposed (Q7, Q14). Unknown extension labels are skipped unless a Graphic Control is pending at a label 0x00–0x7F (Q7).
- **Multiple frames and compositing.** A transparent-black canvas the size of the logical screen; each frame drawn with its transparency; disposal 2 clears to transparent (the background index is unused, as in every incumbent); disposal 3 restores the pre-frame pixels; frames outside the screen (RecommendedBeta: clipped, with GDK's restore quirk as a compatibility field); zero-size screens rejected.
- **Damaged streams.** Truncation at every block boundary and inside every block, short frames and invalid LZW codes (named errors in Strict and RecommendedBeta; partial frames only in GdkCompat, Q4); out-of-range colour indices (rejected in RecommendedBeta, Q5); trailing garbage (`TrailingBytes` in Strict, ignored in RecommendedBeta); missing trailer (`MissingTrailer`, accepted in RecommendedBeta by design: gdk-pixbuf rejects it); stray bytes between blocks (GDK compatibility only); block sizes and sub-block termination exactly as §15–§16.

## API

Matching how gdk-pixbuf (`GdkPixbufAnimation` + iterator), image-rs (`GifDecoder::into_frames`) and glycin (`specific_frame`) hand out one composited, screen-sized RGBA8 frame at a time (Q2 (a)):

```rust
pub fn try_decode_still(b: &[u8]) -> Result<Still, TryError>;                      // first frame on the canvas (G14)
pub fn try_decode_still_with_policy(b: &[u8], p: Profile) -> Result<(Still, bool), TryError>;
pub struct Frames<'a> { /* borrowed input, canvas, LZW table, saved rectangle */ }
impl<'a> Frames<'a> {
    pub fn try_new(b: &'a [u8], p: Profile) -> Result<Frames<'a>, TryError>;      // G2–G4, reserves Canvas + LzwTable,
                                                                                 // then walks to the first image descriptor (G15)
    pub fn info(&self) -> &Info;                                                 // W, H, version, background, aspect, loop_count
    pub fn next_frame(&mut self) -> Option<Result<FrameView<'_>, TryError>>;     // Some(Ok) per frame, Some(Err) once, then None
}
pub struct FrameView<'f> { pub rgba: &'f [u8], pub rect: (u16, u16, u16, u16), pub delay_cs: u16,
                           pub disposal: u8, pub user_input: bool, pub transparent: Option<u8>,
                           pub interlaced: bool, pub recovered: bool }
pub enum Plays { Once, Forever, Repeats(u16) }
pub fn plays(loop_count: Option<u16>) -> Plays;                                  // G20
impl Plays { pub fn total_plays(self) -> Option<u32>; }                          // Once 1, Repeats(n) n+1, Forever None
```

`Frames` is not an `Iterator`: each view borrows the decoder's single canvas, so the caller writes `while let Some(f) = dec.next_frame() { let f = f?; … }`. The iterator is fused after its first error. `try_new` parses up to the first image descriptor, because `info()` promises the loop count found before the first image; an error in that prefix is the constructor's error. `Profile` is `Strict | RecommendedBeta | GdkCompat_2_42_12 | ImageCrateCompat_0_25_10`, `#[non_exhaustive]`; the compatibility profiles are opt-in and versioned, never glycin's default (Q17). `TryError` is `Format(Error) | OutOfMemory(OutOfMemory)` with the 33 error names of G16 (`UnsupportedGraphicControlTarget` added by Q7). The output is untransformed RGBA (no colour management or ICC; Q14). `loop_count` is the raw `Option<u16>` (normative); `plays` is its documented meaning in the browser convention (Q15). Delay mapping (gdk-pixbuf's 0→100 ms and <20 ms→20 ms, glycin's 0→100 ms) belongs to the glue (Q16). A gdk-pixbuf module wraps `Frames` in a `GdkPixbufAnimation`; glycin wraps it in its frame request loop; both copy each `FrameView` into their own buffers.

## Resource caps and allocation

Caps (G1's limits table, Q8; theorems are stated over the constant names): `MAX_INPUT` 64 MiB; `MAX_DIMENSION` 65,535, the GIF wire limit, so no separate per-side cap; `MAX_OUTPUT`: `4·W·H ≤ 64 MiB` (16.7 megapixels); `MAX_FRAMES` 65,536; `MAX_TOTAL_PIXELS` 2^30 decoded frame pixels per decoder. Coordinate arithmetic is specified over the full 0–65,535 range (sums up to 131,070). Clipped pixels and disposal/restore work count against the work bound. Allocation (G18), all through the guard, none before G2–G4 succeed:

| Site | Bytes | Reserved |
|---|---|---|
| `Canvas` | `4·W·H` (≤ 64 MiB) | once, by `try_decode_still` / `Frames::try_new`, after G2–G4 and before the prefix walk |
| `LzwTable` | 20,480 (prefix `u16`, suffix, first byte, string stack; 4096 each) | once, same call |
| `Saved` | `4·W·H` | once per decoder, at the first frame with disposal 3 (`Frames` only) |

Peak live heap per decoder is therefore at most `8·W·H + 20,480` bytes (128 MiB + 20 KiB at the cap), independent of frame count. No index buffer: indices go straight to the canvas through the interlace map, with clipping. An error leaves the failing frame unexposed and the decoder fused, so no per-frame copy is needed. Per call, `Req` is the ordered list above restricted to the reservations that call makes; `PreAlloc` holds exactly when G2–G4 fail (constructor) or when no disposal-3 frame is reached (iterator). `OutOfMemory` from `Saved` can displace any later format error of that frame, as accepted for ICNS (alloc-delta D5). The work bound `steps ≤ c·(|b| + N + P)` (G18) holds for every outcome, including `OutOfMemory`.

## Evidence base

- Spec text: GIF89a (CompuServe 1990), copy and provenance in `spec-draft/sources/`, `spec-draft/SOURCES.md`.
- Incumbents: gdk-pixbuf 2.42.12 source (`sources/gdk-pixbuf-2.42.12/`) and the installed Debian build; image 0.25.10 with gif 0.14.2 and weezl 0.1.12; glycin's in-tree image-rs loader; giflib 5.1.4 and libnsgif c26c7aa as references. Runners from `~/projects/leniency/gif` (built under `/tmp/claude-0`, rebuild with its `build.sh`), probes in `policy-draft/probes/`.
- Real corpus: 1,407 files pinned by `../leniency/corpora/manifests/gif.MANIFEST.tsv` (statistics `gif-STATS-interpretation.md`, minimum code size analysis `gif-LZWMIN-ANALYSIS.md`).
- Results: `policy-draft/OPTIONS.md` (option table with evidence and measured profile agreement), `policy-draft/QUESTIONS.md` (with decisions), `DECISIONS.md` (changelog).
