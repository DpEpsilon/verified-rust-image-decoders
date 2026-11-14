# LZW minimum code size 1: the widening rule (decision input, draft 0.2; decided 2026-11-14, see the last section)

At `m = 1` gdk-pixbuf, weezl (image/gif) and giflib read the same stream with three different code widths. Neither gdk-pixbuf nor image decodes what an Appendix F encoder writes; giflib's rule is the Appendix F one (read from source, not run). At `m >= 2` the three agree on every stream tested. Recommendation: RecommendedBeta accepts `m = 1` only for 1-pixel frames (`w·h = 1`) and returns `InvalidLzwMinCodeSize` for any larger frame. On a 1-pixel frame the loop reads Clear codes and one 2-bit literal and stops, so the widening rule is unobservable; both size-1 corpus files are 1×1 and stay accepted. This note was written before the decision and still reads as the decision input; the outcome is in the last section.

## Width of the j-th code after a Clear at m = 1

| j | 1 | 2 | 3 | 4 | 5 | 6 to 13 | 14 on |
|---|---|---|---|---|---|---|---|
| gdk-pixbuf, oracle (G10.2) | 2 | 2 | 2 | 2 | 2 | 2 | 2 |
| weezl (image 0.25.10 / gif 0.14.2) | 2 | 2 | 3 | 3 | 3 | 4 | 5 |
| giflib; Appendix F applied literally | 2 | 3 | 3 | 3 | 3 | 4 | 5 |

At m = 2 all three read 3, 3, 3, 4, ... (j = 4 widens). The draft's statement that G10.2's test "never fires" is right, and stronger than "for a while": gdk-pixbuf and the oracle keep width 2 until the next Clear, so only codes 0 to 3 are ever readable and no table entry is ever referenced. weezl and giflib agree from j = 3 on; they differ only in the width of the second code.

Sources, read in this sandbox:

- gdk-pixbuf 2.42.12: `io-gif-animation.c:342` passes `m + 1`, so `lzw.c:131-139` gives clear 2, EOI 3, table size 4 = 2^2. The widening test `code_table_size == (1 << code_size)` (`lzw.c:204`) runs only after an entry is added (`lzw.c:197-206`), when the size is already 5, and it is an equality test, so it never matches 4 again. A Clear resets the width to 2 (`lzw.c:192-193`).
- weezl 0.1.12: `decode.rs:1273-1277` (`code_size = min_size + 1`), `decode.rs:1098-1101`: `next_code >= max_code` (4 >= 3) runs when an entry is derived, which the first code after a Clear does not do (`decode.rs:880-888`). The width grows after the second code.
- giflib (`lib/dgif_lib.c:1026-1030` at revision fa37672, fetched and read, not run): `RunningCode` is incremented after every code, including the first, and the width grows when it exceeds `MaxCode1`; the second code at `m = 1` is 3 bits.
- Appendix F (`spec-draft/sources/spec-gif89a.txt:2261`, `2293`, `2300`) says one-bit images must use size 2 and that codes are 3 to 12 bits. `m = 1` is outside it. The gif crate's encoder refuses `m < 2` (`encoder.rs:381`, comment at `encoder.rs:449`), as does weezl's (`lib.rs:104`), while the gif decoder accepts 1 (`reader/decoder.rs:293`). Strict's range 2 to 8 is the spec's.
- Arithmetic: the initial `next` is `2^m + 2` and `2^width` is `2^(m+1)`, so `next < 2^width` holds at the start iff `m >= 2`. At `m = 1` it is 4 against 4.

## Evidence

`lzw_size1_widening.py` builds 496 one-image GIFs for `m` in {1, 2, 3, 8}: random, all-zero, all-one, stripes and sparse pixels at 64×64, random and sparse at 256×256, and 1 to 16 pixels. Encoders, with E the index of the entry just assigned and w the current width: `std` widens when E >= 2^w and applies the same test before the End of Information code (what giflib's encoder does); `early` and `late` widen one entry sooner or later; `lit-gdk`, `lit-weezl` and `lit-std` emit literal codes only, with the widths of each decoder in the table above. Each stream goes through gdk-pixbuf (animation path and forced loader), the image runner, and the oracle (RecommendedBeta at `m = 1`, Strict at `m >= 2`), and the output is compared with the intended pixels.

`m = 1`, long streams (`std`: 7 streams of 4,096 or 65,536 pixels; `lit-*`: 2 streams of 3,600 pixels each):

| stream written with | gdk-pixbuf (both paths) | image | oracle |
|---|---|---|---|
| `std` (Appendix F) | wrong pixels, no error, 7 of 7 | error, 7 of 7 | `TooFewPixels`, 7 of 7 |
| `lit-gdk` | intended 2 of 2 | error 2 of 2 | intended 2 of 2 |
| `lit-weezl` | wrong 2 of 2 | intended 2 of 2 | wrong 2 of 2 |
| `lit-std` | wrong 2 of 2 | wrong 2 of 2 | wrong 2 of 2 |

gdk-pixbuf draws the first few pixels (4 and 2 in the two streams inspected, not all of them right), stops at the first code it cannot resolve, leaves the rest transparent and reports success (4,094 of 4,096 pixels differ on the 64×64 random stream). On the 72 short `std` streams (1 to 16 pixels) gdk-pixbuf is right on 18, image on 23, the oracle on 18 and silently wrong on 13. All three give the intended pixels on 8 of 8 streams of one pixel, 8 of 8 of two, 1 of 8 of three, 1 of 8 of four and 0 of 40 from five up. Each incumbent decodes 72 of 72 short streams written with its own widths.

Which rule each incumbent implements: gdk-pixbuf's output equals the gdk reference decoder's on 253 of 253 streams where the reference completes, the oracle's on 253 of 253, and image's equals the weezl reference decoder's on 216 of 216. The reference decoders are in the script.

Control, `m` = 2, 3, 8: the `std` streams decode to the intended pixels in all four decoders (7 of 7 long streams at each size, 72 of 72 short ones at `m = 2`), and `early` and `late` streams break all four alike (at `m = 2`, 7 of 7 `early` streams are wrong in gdk-pixbuf and rejected by image and the oracle). At `m >= 2` there is one rule; the split belongs to size 1.

Corpus: 2 of 1,407 files have `m = 1` and decode (`set_hsts.gif`, and chromium `0.gif`, the same 42 bytes plus 12 trailing bytes); both are 1×1, so they cannot tell the rules apart. The Q6 analysis lists two more size-1 files that Strict reaches (libnsgif `ns-afl-gif` id:000007 and id:000092, 32×32 mutants); all three incumbents reject them.

## Options

- **Follow gdk-pixbuf (what the oracle does now).** The accepted `m = 1` language is "every code is a 2-bit literal", which no encoder writes. Appendix F streams decode to wrong pixels without error in 13 of 72 short cases. The width lemma `next < 2^width ∨ width = 12` (PLAN M2) fails at the start and for ever after, so T-LZW needs an `m = 1` exception for a rule that nothing needs. Not recommended.
- **Appendix F literally (giflib).** Correct for what an encoder following the spec would write, but disagrees with both incumbents from the second code, and the proof needs the first code to count toward widening. Not recommended unless the owner wants spec fidelity over agreement.
- **Reject `m = 1`.** The simplest proof (no `m = 1` case at all) at the cost of the two 1×1 corpus files that every incumbent decodes identically, against Q6's rule that RecommendedBeta accepts what the incumbents decode identically.
- **Accept `m = 1` only when `w·h = 1` (recommended).** Corpus unchanged (the `lzw_min` field still flips the same five files). For a 1-pixel frame under `StopAtRaster` the loop reads Clear codes and at most one literal, all at width 2 in all three rules, then returns; `prev` is `none` when the literal is read, so no entry is added. T-LZW stays as drafted for `m` in [2, 11] and `m = 1` needs one lemma (no entry is added, width stays 2), inside the 20 to 40 obligations PLAN budgets for sizes 1 to 11 (the `m = 1` code width is part of that figure). The widest bound on which gdk-pixbuf, image and the oracle still agree is two pixels (all four encoders, 32 of 32 streams); giflib's rule breaks at two, so the area test is 1.

Obligation R is unaffected by every option: Strict accepts `m` in [2, 8] only and the rule at `m >= 2` does not change.

Consequences of the recommendation, for the owner to apply: the vector `lzw-min-code-1-1x1@RecommendedBeta` stays; add `lzw-min-code-1-2x1@RecommendedBeta` expecting `InvalidLzwMinCodeSize`. GdkCompat-2.42.12 keeps the gdk rule as it is. ImageCrateCompat-0.25.10 inherits G10.2 and so differs from image on `m = 1` frames of three or more codes; record it as a residual (no corpus file is affected) or give the profile weezl's schedule (the second code is 2 bits wide, then the Appendix F widths).

## Decision

The owner decided on 2026-11-14 to accept the recommendation: in RecommendedBeta, `lzw_min=Range1to11Checked` accepts `m = 1` only when the frame is exactly one pixel (the descriptor's `w·h = 1`) and returns `InvalidLzwMinCodeSize` for any larger frame with `m = 1`. Sizes 10 and 11 stay accepted with checked literals (a literal above 255 is `ColorIndexOutOfRange`); Strict stays at 2 to 8; compatibility profiles keep their library's rule.

Applied the same day:

- `policy_oracle.py`: `read_image_data` takes the frame's declared area and, for `Range1to11Checked` with `m = 1` and area not 1, raises `InvalidLzwMinCodeSize` at the minimum code size byte, before the sub-blocks are read. The size is the descriptor's, not the clipped visible part and not the screen's.
- `gen_vectors.py` (373 vectors, 360 before; 13 added, all `@RecommendedBeta`): `lzw-min-code-1-1x1` (accepted, unchanged, now with a `/frames` sibling), `-2x1` and `-1x2` (rejected), `-1x1-interlaced` (accepted), `-1x1-frame-in-4x3-screen` (accepted: the frame's size counts, not the screen's), `-2x1-frame-clipped-to-1x1-screen` (rejected: the declared size counts, not the visible part), `-second-frame-2x1` (the first frame is returned, the second fails; a still stops after the first image). Each has a still and a `/frames` record.
- Checks after the change: all 373 vectors agree; Strict bridge 360/360 inputs; obligation R unchanged, 0 violations; compatibility agreement unchanged for the three non-recommended profiles; RecommendedBeta against the incumbents 311 → 310 (`image`) and 269 → 268 (GDK loader), the one input being the 2×1 size-1 vector `vec:lzw-min-code-1`; the corpus accept/reject outcomes are unchanged (below).
- Corpus: the seven files whose walk reaches an `m = 1` image-data call were re-analysed with the pass script's own `analyse`. The two accepted files (`set_hsts.gif`, chromium `0.gif`, both 1×1) are accepted as before and still flip through `lzw_min`; the five rejected ones are fuzzer-derived libnsgif mutants with frames of 1,024 or 8,224 pixels, and their first error changes to `InvalidLzwMinCodeSize` (four were `TruncatedImageData`, one `TooFewPixels`). All three incumbents reject all five.
- Proof: T-LZW stays as drafted for `m` in [2, 11]; `m = 1` needs the one small lemma in `PLAN.md` M2 (a one-pixel frame adds no table entry, the width stays 2).

Still open: `ImageCrateCompat-0.25.10` keeps `Range1to11` and G10.2's width 2, so on an `m = 1` frame of three or more codes it differs from `image` (last paragraph above); no corpus file is affected.
