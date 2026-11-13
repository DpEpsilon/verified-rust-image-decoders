# Brief: verified GIF decoder core (Verus) — scope

Status: specification phase. The draft spec is `spec-draft/` (GIF-CORE-SPEC 0.1, oracle, 312 vectors), the leniency catalogue is `policy-draft/` (22-field record, four profiles, 18 owner questions), and the proof plan is `PLAN.md`. Nothing is frozen and no Rust exists yet. Once the owner answers `policy-draft/QUESTIONS.md` Q1–Q10, `spec-draft/` and `policy-draft/` are frozen (hashes in a `FREEZE.txt` each) and this brief gains the binding rules of `../dib/BRIEF.md`, `../dib/BRIEF-POLICY.md` and `../alloc-delta/SPEC.md` unchanged.

## Goal

A feature-complete GIF decoder core with the guarantees the DIB/BMP core has: safety (T1) for every input and every policy record; exact equality with the frozen model (T2 and the integrated T9/T10), including the first error; allocation bounds (T3) through the sealed `alloc_guard` with `try_*` entry points, `OutOfMemory` outside the format error set, and the G/Prog/PreAlloc theorem shape; termination (T11); a linear work bound (T12); and the policy machinery: a closed `recovered` event list (E1–E4), the Strict bridge (`policy(Strict) = core model` for every input), obligation E (`recovered` iff an event occurred) and obligation R (Recommended returns identical RGBA with `recovered=false` on every Strict-accepted input). The crate is `no_std + alloc`.

## Features in scope

- **Versions.** GIF87a and GIF89a; other version strings rejected (Q12). The version does not gate 89a blocks.
- **Logical Screen Descriptor.** Screen size (zero rejected), global table flag and size, colour resolution and sort flag (ignored), background colour index and pixel aspect ratio (exposed raw, unused).
- **Colour tables.** Global and local, `2^(n+1)` entries; local overrides global for one image; a frame with neither is an error (Q11); sort flags ignored.
- **Image Descriptor.** Position and size (zero size rejected; frames must lie inside the screen in Strict, clipped in Recommended), interlace flag, reserved bits ignored.
- **Interlacing.** The four-pass row order for every height, including heights below 8 where passes are empty.
- **LZW.** Minimum code size 2–8 (Q6), codes LSB-first from 3 to 12 bits, Clear and End of Information codes, optional leading Clear, the KwKwK case, widening when the table reaches `2^width`, the full 4096-entry table with deferred clear, and exact end conditions (too few pixels, missing End of Information, surplus pixels, bytes after End of Information) with the incumbents' leniencies as policy fields (`lzw_tail`, `short_frame`, `lzw_invalid`, `lzw_min`, `empty_first_subblock`, `image_data`).
- **Graphic Control Extension.** Disposal 0–3 (4–7 rejected in Strict, treated as 1 in Recommended), transparency index (an index outside the table is allowed when transparent), delay in hundredths (raw), user input flag (exposed), exact block size, at most one per graphic, scope of exactly one graphic-rendering block (Q7).
- **Application Extension.** NETSCAPE2.0 loop count (raw value before the first image, Q15); the buffering sub-block, XMP, ICC and other applications parsed and skipped (Q14).
- **Comment and Plain Text Extensions.** Parsed and skipped; Plain Text is not rendered and nothing is exposed (Q7, Q14). Unknown extension labels are skipped.
- **Multiple frames and compositing.** A transparent-black canvas the size of the logical screen; each frame drawn with its transparency; disposal 2 clears to transparent (the background index is unused, as in every incumbent); disposal 3 restores the pre-frame pixels; frames outside the screen (Recommended: clipped, with GDK's restore quirk as a compatibility field); zero-size screens rejected.
- **Damaged streams.** Truncation at every block boundary and inside every block (named errors; partial frames only in compatibility profiles, Q4); trailing garbage (`TrailingBytes` in Strict, ignored in Recommended); missing trailer (`MissingTrailer`, accepted in Recommended); stray bytes between blocks (GDK compatibility only); block sizes and sub-block termination exactly as §15–§16.

## API

Matching how gdk-pixbuf (`GdkPixbufAnimation` + iterator), image-rs (`GifDecoder::into_frames`) and glycin (`specific_frame`) hand out one composited, screen-sized RGBA8 frame at a time (Q2):

```rust
pub fn try_decode_still(b: &[u8]) -> Result<Still, TryError>;                      // first frame on the canvas (G14)
pub fn try_decode_still_with_policy(b: &[u8], p: Profile) -> Result<(Still, bool), TryError>;
pub struct Frames<'a> { /* borrowed input, canvas, LZW table, saved rectangle */ }
impl<'a> Frames<'a> {
    pub fn try_new(b: &'a [u8], p: Profile) -> Result<Frames<'a>, TryError>;      // G2–G4, reserves Canvas + LzwTable
    pub fn info(&self) -> &Info;                                                 // W, H, version, background, aspect, loop
    pub fn try_next(&mut self) -> Result<Option<FrameView<'_>>, TryError>;        // fused after Err
}
pub struct FrameView<'f> { pub rgba: &'f [u8], pub rect: (u16, u16, u16, u16), pub delay_cs: u16,
                           pub disposal: u8, pub user_input: bool, pub transparent: Option<u8>,
                           pub interlaced: bool, pub recovered: bool }
```

`TryError` is `Format(Error) | OutOfMemory(OutOfMemory)` with the 32 error names of G16. Delay mapping (gdk-pixbuf's 0→100 ms and <20 ms→20 ms, glycin's 0→100 ms) and loop-count translation belong to the glue (Q15, Q16). A gdk-pixbuf module wraps `Frames` in a `GdkPixbufAnimation`; glycin wraps it in its frame request loop; both copy each `FrameView` into their own buffers.

## Resource caps and allocation

Caps (G1, Q8): input 64 MiB; screen ≤ 8,192 per side and `4·W·H ≤ 64 MiB`; at most 65,536 frames; at most 2^30 decoded frame pixels per decoder. Allocation (G18), all through the guard, none before G2–G4 succeed:

| Site | Bytes | Reserved |
|---|---|---|
| `Canvas` | `4·W·H` (≤ 64 MiB) | once, by `try_decode_still` / `Frames::try_new` |
| `LzwTable` | 20,480 (prefix `u16`, suffix, first byte, string stack; 4096 each) | once, same call |
| `Saved` | `4·W·H` | once per decoder, at the first frame with disposal 3 (`Frames` only) |

Peak live heap per decoder is therefore at most `8·W·H + 20,480` bytes (128 MiB + 20 KiB at the cap), independent of frame count. No index buffer: indices go straight to the canvas through the interlace map, with clipping. An error leaves the failing frame unexposed and the decoder fused, so no per-frame copy is needed. Per call, `Req` is the ordered list above restricted to the reservations that call makes; `PreAlloc` holds exactly when G2–G4 fail (constructor) or when no disposal-3 frame is reached (iterator). `OutOfMemory` from `Saved` can displace any later format error of that frame, as accepted for ICNS (alloc-delta D5). The work bound `steps ≤ c·(|b| + N + P)` (G18) holds for every outcome, including `OutOfMemory`.

## Evidence base

- Spec text: GIF89a (CompuServe 1990), copy and provenance in `spec-draft/sources/`, `spec-draft/SOURCES.md`.
- Incumbents: gdk-pixbuf 2.42.12 source (`sources/gdk-pixbuf-2.42.12/`) and the installed Debian build; image 0.25.10 with gif 0.14.2 and weezl 0.1.12; glycin's in-tree image-rs loader; giflib 5.1.4 and libnsgif c26c7aa as references. Runners from `~/projects/leniency/gif` (built under `/tmp/claude-0`, rebuild with its `build.sh`), probes in `policy-draft/probes/`.
- Results: `policy-draft/OPTIONS.md` (option table with evidence and measured profile agreement), `policy-draft/QUESTIONS.md`.
