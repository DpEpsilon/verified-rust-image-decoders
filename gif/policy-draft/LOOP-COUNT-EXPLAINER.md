# GIF loop count: what it is, how decoders read it, and what GIF-Q15 decides

Prepared 2026-11-14 for the decision on GIF-Q15 (`QUESTIONS.md:37`). Sources were read this session at the commits pinned in the key at the end; anything I could not check is marked UNVERIFIED.

## Conclusion

GIF89a has no repeat field. Animations repeat only if the file carries an optional block, the NETSCAPE2.0 application extension, a Netscape-era convention that every browser copied. The 16-bit number stored in it means "repeat this many times **after** the first play": no block means one play, stored 0 means forever, stored N >= 1 means N+1 plays. Of the twelve implementations in the table, seven convert the stored value to total plays this way (gdk-pixbuf, Wuffs/Chrome, WebKit, Firefox, libnsgif, ImageMagick, gif2webp). gdk-pixbuf's "+1" is that conversion, the same one the browsers make; it is not a gdk quirk. Pillow, the `gif` crate and giflib's tools pass the raw number through.

The implementations disagree on four things that change what a viewer sees:

- **No extension.** `image` 0.25.10 reports `LoopCount::Infinite` (forever). Every other reader plays once.
- **Stored N >= 1.** `image` forwards the raw N as `LoopCount::Finite(N)`. Its own trait default, `Finite(1)`, means "run once" (`io/decoder.rs:202-208`), so a consumer following that convention plays a stored-1 GIF once; browsers play it twice.
- **Position and identifier.** `gif`/`image` and Pillow read only blocks before the first image and only `NETSCAPE2.0`. Browsers, gdk-pixbuf, libnsgif and gif2webp accept the block anywhere and also accept `ANIMEXTS1.0`. I measured gdk-pixbuf: stored 2 after the first frame still gives 3 plays.
- **Consumption.** glycin never reads the file's loop count for GIF; its loader loops forever unless the caller passes `loop_animation=false`.

The draft's description of Q15 ("image-rs's rule") is accurate for position and identifier only. It also differs from the `gif` crate in how sub-blocks are scanned (see "Edge cases"), and `image` does not return the raw value (rows above). Recommendation (section 5): keep the raw `Option<u16>` before the first image as the normative field, as Q15 proposes, and add one documented pure function from that field to a plays enum. Put "anywhere" and `ANIMEXTS1.0` behind the `loop_source` switch that `OPTIONS.md:54` already has, for the GdkCompat profile only.

## 1. The extension

GIF89a (`spec-draft/sources/spec-gif89a.txt`, section 26) defines an Application Extension: label `0xFF`, block size 11, an 8-byte application identifier, a 3-byte authentication code, then data sub-blocks, then a zero-length terminator. It says nothing about repetition. The Graphic Control Extension carries only disposal, user-input flag, transparency and delay, and Appendix D says GIF "is not intended as a platform for animation" (`:2053`). Netscape therefore put its loop count in an application extension with identifier `NETSCAPE2.0`. A sub-block is a length byte followed by that many data bytes.

Worked example, the 19 bytes of `netscape-loop-5` in `spec-draft/VECTORS.json`:

```
21 ff 0b                                extension introducer, application label, block size = 11
4e 45 54 53 43 41 50 45 32 2e 30        "NETSCAPE" (8-byte identifier) + "2.0" (3-byte authentication code)
03 01 05 00                             sub-block of 3 bytes: id 1 = loop count, value 05 00 little-endian = 5
00                                      terminator (sub-block of length 0)
```

It sits after the header, logical screen descriptor and global colour table, and before the first Graphic Control (`21 f9 ...`) and image descriptor (`2c`). With two frames A and B, stored 5 plays A B six times, then holds B. Stored 0 repeats forever; no block plays A B once.

| stored value | meaning | total plays |
|---|---|---|
| no block | play once | 1 |
| 0 | forever | unbounded |
| 1 | 1 repeat after the first play | 2 |
| 5 | 5 repeats | 6 |
| 65535 | 65535 repeats | 65536 |

**Position.** The de-facto description says the block must come immediately after the global colour table (the giflib bundle: `doc/whatsinagif/bits_and_bytes.html:586`, "19 bytes long"; `doc/gifbuild.xml:140`). I could not fetch Netscape's original text. A Pillow maintainer quotes it as "indicate the number of iterations the loop should be executed" and "an iteration count of zero indicates infinite" (Pillow issue 4575). The first phrase is ambiguous between N plays and N repeats, which is the whole off-by-one problem. Where the block appears elsewhere, the readers differ (table, column 2).

**Buffering sub-block.** The same identifier carries a second sub-block type, id 2. The project's vector `netscape-buffering-subblock-ignored` uses `05 02 00 10 00 00`: size 5, id 2, a 4-byte value. Its meaning is UNVERIFIED (no primary source reachable). Firefox "allows, but ignores" it (`nsGIFDecoder2.cpp:781-784`); WebKit's comment says it "doesn't do anything" (`GIFImageReader.cpp:643-649`).

**`ANIMEXTS1.0`.** Same layout with a different identifier. Wuffs cites a page on it (`decode_gif.wuffs:687-690`, vurdalakov.net, unreachable from here). Its origin is UNVERIFIED. Gecko, WebKit, Wuffs, libnsgif, gdk-pixbuf and gif2webp accept it; `gif`, Pillow and ImageMagick do not.

## 2. Semantics and browser history

Browsers: stored N >= 1 gives N+1 plays; absent gives one play. Evidence:

- Firefox sets `mLoopCount = N` (0 becomes -1, forever), starts a remaining-repeats counter at N when the first play ends and decrements it per extra play (`FrameAnimator.cpp:204-232`). N repeats after the first play is N+1 plays. An absent block leaves `loop_count` at 0 from `memset` (`nsGIFDecoder2.cpp:95`), so the animation finishes after one play.
- Chrome decodes GIF with Skia's Wuffs codec (`gif_image_decoder.cc:36-44` calls `SkGifDecoder::Decode`). Wuffs turns stored N >= 1 into N+1 total loops (`decode_gif.wuffs:727-740`); Skia subtracts one again to report "repetitions after the first play" (`SkWuffsCodec.cpp:889-900`, `SkCodec.h:791-795`), and Blink maps 0 to play once for an animated image (`skia_image_decoder_base.cc:166-173`).
- WebKit returns `loopCount + 1` for positive values (`GIFImageDecoder.cpp:94-96`) and one play when no count was seen.

History: WebKit changed to N+1 in April 2018 (commit 729a5cee31, bug 183153, "Animated GIF imagery with finite looping are falling one loop short"). Its message states that "the extra loop seems to be the consensus", that Chrome 65 and Firefox 59 translate stored n-1 to n plays, and that generators disagree: Photoshop and ezgif write n-1 for n loops, gifmaker.me and gifmaker.org write n. I did not check the generators. I could not reach Mozilla's or Chromium's bug trackers, so their own histories for this are UNVERIFIED; the WebKit commit is the earliest primary statement I found. Pillow documents `loop` as "number of times the GIF should loop" and writes it unconverted, so `loop=1` plays twice; a user reported that as unexpected, the maintainer answered by quoting the Netscape text, and the reporter closed the issue (Pillow issue 4575). The `gif` crate's encoder agrees with the repeat reading: `Finite(0)` writes no block, "0 finite repetitions can only be achieved if the corresponding extension is not written" (`encoder.rs:318-323`).

## 3. How each implementation handles it

"Raw" means the stored number is passed through unchanged. All rows are from source; where I ran code it says so.

| implementation | where it looks | identifiers | stored value to loop count | no extension | evidence |
|---|---|---|---|---|---|
| project draft | before the first image descriptor (`k = 0`); every 3-byte id-1 sub-block, last wins | `NETSCAPE2.0` | raw `Option<u16>` | `None` | `SPEC.md:16,50`; `policy_oracle.py:121-126` |
| gdk-pixbuf 2.42.12 | anywhere, during block parsing; every sub-block, id 1 taken, size unchecked; last wins | both | 0 forever; N >= 1 stored as N+1 total plays | 1 play (`loop = 1`); then last frame held, delay -1 | `io-gif.c:312-337,796`; `io-gif-animation.c:268-274,317`; measured below |
| `gif` 0.14.2 | only in the header loop, which ends at the first non-extension block; later blocks ignored | `NETSCAPE2.0` | `Repeat::Infinite` if 0, else `Repeat::Finite(N)` raw; only the first sub-block after the identifier is examined | `Repeat::Finite(0)` (the default), same value as an absent block | `reader/mod.rs:259,317,326-352,363-393,625`; `encoder.rs:102-113` |
| `image` 0.25.10 | as `gif` | as `gif` | `Finite(N >= 1)` becomes `LoopCount::Finite(N)` raw; `Finite(0)` and `Infinite` both become `LoopCount::Infinite` | `LoopCount::Infinite` | `codecs/gif.rs:427-434`; `metadata.rs:164-171` |
| glycin (GIF via `image`) | does not read it | none | never calls `loop_count()`; replays forever unless `loop_animation=false`, then stops after one pass | same for every file | glycin 7344b95 `glycin-image-rs/src/animated.rs:16-84`, `lib.rs:260-272`; `glycin-utils/src/api/loader.rs:84-95` |
| Pillow | only while seeking frame 0 (extensions before the first image) | `NETSCAPE2.0` | `info["loop"]` raw; first sub-block after identifier only | no `loop` key | `GifImagePlugin.py:262-270`; `docs/handbook/image-file-formats.rst:311-313,401-403` |
| Wuffs (Chrome via Skia) | any position; value is whatever has been decoded when asked | both | 0 forever; N >= 1 gives N+1 loops; first sub-block only | animated 1, still 0 | `decode_gif.wuffs:47-50,297-312,672-760`; `test/c/std/gif.c:657-662,1501-1560` |
| WebKit | any position (state machine) | both | 0 infinite; N >= 1 gives N+1 | once | `GIFImageReader.cpp:610-650`; `GIFImageDecoder.cpp:67-97` |
| Firefox (Gecko) | any position; last wins | both | 0 becomes -1 forever; N means N repeats after the first play | one play | `nsGIFDecoder2.cpp:95,110,124,736-785`; `FrameAnimator.cpp:204-232`; `FrameAnimator.h:30-31` |
| libnsgif | each frame's extensions while scanning; later overrides | both | `loop_max` = N+1 if N > 0, 0 forever; first sub-block only | `loop_max = 1` | `src/gif.c:808-875,1601,1885-1930`; `include/nsgif.h:388` |
| ImageMagick | any position (extension loop); no sub-block id check | `NETSCAPE2.0` | `iterations` = N+1 if N != 0; writer stores `iterations-1` | `iterations = 1` | `coders/gif.c:1056,1149-1160,1743-1757` |
| gif2webp (libwebp) | any position | both | N+1 for 0 < N < 65535, else raw; `-loop_compatibility` keeps raw | multi-frame: 1 | `examples/gif2webp.c:427-436,492-510`; `examples/gifdec.c:172-187` |
| giflib (AOSP mirror) | the decoder library never interprets it; `gifbuild` prints and writes the raw value | `NETSCAPE2.0` (tool only) | raw ("repeat count 0 means loop forever") | n/a | `gifbuild.c:481-500,765-768`; `doc/gifbuild.xml:138-142`; no match in `dgif_lib.c` |

**Measured, gdk-pixbuf 2.42.12** (the installed Debian library). A 25-line C program calls `gdk_pixbuf_animation_iter_advance` with synthetic time in 10 ms steps and records when `get_delay_time` first returns -1. Nine two-frame GIFs, 100 ms per frame, so one play is 200 ms:

| file | result |
|---|---|
| no block | stops at 200 ms: 1 play |
| `NETSCAPE2.0` stored 0 | still animating at 6000 ms: forever |
| stored 1 | 400 ms: 2 plays |
| stored 2 | 600 ms: 3 plays |
| `ANIMEXTS1.0` stored 2 | 3 plays |
| stored 2 between Graphic Control and first image | 3 plays |
| stored 2 after first frame | 3 plays |
| stored 2 after last frame | 3 plays |
| buffering sub-block, then loop sub-block, stored 2 | 3 plays |

The harness lives in the session scratchpad and is not kept. The public gdk-pixbuf API has no loop-count getter (no match in `gdk-pixbuf-animation.h/.c`); the count acts only inside `iter_advance` (`io-gif-animation.c:274`), which reads `anim->loop` live, so a late extension can change playback mid-animation under progressive loading.

**Not measured.** The sandbox has no Rust toolchain, so `gif`, `image` and glycin are source reading only. Browsers, Pillow, libnsgif, ImageMagick and gif2webp were not run. Firefox, WebKit and Chrome behaviour for an extension after the first frame is inferred from their state machines; for Wuffs it is pinned by its test (`gif.c:1501-1560`: a file with several loop blocks reports 0, 1, 51, 31 after successive frame configs and 41 at the end), which also shows the answer depends on when you ask.

**Edge cases.** The sub-block rules differ. `gif`, Pillow, libnsgif and Wuffs examine only the sub-block directly after the identifier: a buffering sub-block first hides the loop sub-block, and the count is lost (for `image`, that means `Infinite`). gdk-pixbuf and the draft scan every sub-block. ImageMagick reads bytes 1-2 of every sub-block without checking the id, so a buffering sub-block after the loop sub-block overwrites the count. Gecko and WebKit abort decoding on a NETSCAPE sub-block with an id other than 1 or 2 (`nsGIFDecoder2.cpp:786-787`; `GIFImageReader.cpp:650-652`). The draft's vector `netscape-buffering-subblock-ignored` (buffering first, then stored 7, expecting 7) is a case where the draft and the `gif` crate differ; no runner exposes the loop count, so the 326-327/329 agreement in Q17 could not have caught it.

## 4. The design question in practice

Under the draft, `Info.loop_count` is `Option<u16>`, set while the parser walks the blocks before the first image descriptor, so the header is complete when it is returned. The frame iterator needs nothing for looping: it yields each frame once, and the glue replays (gdk-pixbuf does it with modular arithmetic on elapsed time, `io-gif-animation.c:268-274`; glycin restarts its decoder, `animated.rs:16-84`). `SPEC.md:142` already excludes loop playback from the core.

What each rule returns for two-frame files, and what a viewer sees if the glue converts the way browsers do (None = once, 0 = forever, N = N+1 plays):

| file | draft: raw before first image | gdk-pixbuf rule: anywhere, `ANIMEXTS1.0`, +1 | enum on the draft's position rule | viewer, browsers / gdk-pixbuf | `image` 0.25.10 reports |
|---|---|---|---|---|---|
| no block | `None` | 1 | `Absent` | 1 play | `Infinite` |
| stored 0 | `Some(0)` | 0 | `Forever` | forever | `Infinite` |
| stored 2 before first image | `Some(2)` | 3 | `Repeats(2)` | 3 plays | `Finite(2)` |
| stored 2 after first frame | `None` | 3 | `Absent` | Firefox, WebKit, gdk-pixbuf: 3 plays (Firefox and WebKit by source); Chrome depends on when it asks | `Infinite` |
| `ANIMEXTS1.0` stored 2 | `None` | 3 | `Absent` | 3 plays | `Infinite` |

The two bottom rows are where the draft's position and identifier rules cost something: those files play once under the draft and three times in browsers and gdk-pixbuf. Under `image`, they and the no-block file report "forever".

Interaction with the frame iterator: a count after the first frame cannot be in a header returned before the frames. The core takes the whole stream as a slice (`SPEC.md:142`), so it could pre-scan, but that adds a second block walker whose block boundaries, termination and error behaviour must be proved equal to the main loop's. Alternatives are a value available only after iteration ends, or a header that changes. gdk-pixbuf shows the hazard: its value changes under a running iterator.

## 5. Options for the owner

- **(a) Raw stored value only, glue translates (current Q15).** Proof cost: none beyond G6.3, which already sets the field under `k = 0`. Risk: each glue re-derives the meaning. Pillow issue 4575, WebKit bug 183153 and `image`'s mapping show that is where the mistakes happen.
- **(b) Raw plus a documented meaning.** Keep `loop_count: Option<u16>` as the normative field and add a pure function `plays(Option<u16>)`: `None` gives Once, `Some(0)` Forever, `Some(n)` `Repeats(n)`, with `total_plays = n + 1` computed in `u32` (65535 + 1 cannot overflow). It is total and checkable exhaustively over its 65,537 inputs, with the raw value still available. Cost: a dozen lines and a spec paragraph. The convention it documents is the one seven of twelve implementations use.
- **(c) Mimic gdk-pixbuf in the core.** Position: anywhere, so pre-scan or post-iteration value (section 4). `ANIMEXTS1.0`: one more 11-byte comparison, trivial. Storing N+1 discards the raw value and merges "absent" with "stored 0 repeats". Highest proof cost for a rule one library has; better as a GdkCompat option.
- **(d) Mimic `image`.** Cheapest, but it makes a no-block GIF loop forever, against every browser and gdk-pixbuf, and it cannot distinguish absent from 0.

**Recommendation: (b), with the position and identifier rules unchanged in Strict.** Reasons: the byte-level facts (position, id, sub-block) are what a verified decoder can state exactly, so the field stays raw and before the first image; the conversion is the part consumers get wrong, so it should be written once, with test vectors; `loop_source` (`OPTIONS.md:54`) is the right place for `GdkAnywhere` plus `ANIMEXTS1.0`. Two additions to the draft: decide whether the Strict sub-block scan stays "every 3-byte id-1 sub-block" (current oracle) or becomes "first sub-block only" like `gif`, Pillow and libnsgif, and add a vector for it either way; and record that today only gdk-pixbuf's behaviour is measured, so the other rows in section 3 stay UNVERIFIED by measurement until a runner exposes the count (`QUESTIONS.md:37`). For glycin, no core change is needed to keep today's behaviour; honouring the file's count would be new glue, as its JXL loader already does for JXL (`jxl.rs:139-143,186-192`).

## Key

GitHub URL form: `https://github.com/<repo>/blob/<commit>/<path>`.

- gdk-pixbuf: local copies `sources/gdk-pixbuf-2.42.12/`; public API checked in `~/projects/spec/gdk-pixbuf` (ff4acb5)
- `gif` 0.14.2, `image` 0.25.10: `~/.cargo/registry/src/*/`; glycin: `~/projects/glycin-plan/glycin` 7344b95, which locks `image` 0.25.10 and `gif` 0.14.2
- Pillow `python-pillow/Pillow` 7533ff2; issue 4575 `/issues/4575`
- Wuffs `google/wuffs` b36a81d; Skia `google/skia` 7412787; Chromium `chromium/chromium` 644da39
- WebKit `WebKit/WebKit` 5f4e77b; commit `/commit/729a5cee31411522b069201b6b15c1347c9c08f6`
- Gecko `mozilla/gecko-dev` 5836a06; libnsgif `netsurf-browser/libnsgif` c26c7aa
- ImageMagick `ImageMagick/ImageMagick` cf973b4; libwebp `webmproject/libwebp` 7c382bd
- giflib: `aosp-mirror/platform_external_giflib` 425be06 (upstream on SourceForge was unreachable; this is Android's copy of 5.2)

Unreachable from this sandbox: sourceforge.net, git.netsurf-browser.org, searchfox.org, chromium.googlesource.com, web.archive.org, bugzilla.mozilla.org, bugs.webkit.org, crbug.com, w3.org, wiki.whatwg.org, vurdalakov.net. Netscape's original text and the browsers' bug trackers were therefore not read.
