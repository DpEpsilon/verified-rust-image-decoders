# PNG cap values (draft, not frozen)

The six caps of PNG-CORE-SPEC P1 (`../spec-draft/SPEC.md`), their current values and the decisions behind them. The specification and the theorems name the caps and never their values; `limits.py` next to this file is the only Python that writes them, and `png_verus/src/limits.rs` will be the only Rust. These values freeze at Stage 2, after the corpus pass is re-run under them (`../REVIEW-BRIEF.md`, PNG-Q7); until then they may change inside the permitted domain below without touching the Stage-1 freeze.

| Constant | Value | Bounds |
|---|---|---|
| `MAX_INPUT` | 1,073,741,824 (2^30, 1 GiB; 256 MiB until 2026-11-17) | input bytes (borrowed, never copied or reserved) |
| `MAX_DIMENSION` | 2^31 − 1 (2,147,483,647, the PNG wire limit) | IHDR width and height; `MAX_OUTPUT` bounds the area, so the longest admissible side is `MAX_OUTPUT/4` = 2^26 (a one-pixel-wide image) |
| `MAX_OUTPUT` | 268,435,456 (256 MiB) | `4·W·H`, one RGBA8 image or canvas (2^26 = 67,108,864 pixels, 8192 × 8192 equivalent); the RGBA16 output is `2·MAX_OUTPUT` (512 MiB) |
| `MAX_FRAMES` | 65,536 | acTL `num_frames` |
| `MAX_TOTAL_PIXELS` | 2^30 | sum of `w·h` over all fcTL frames of one stream |
| `MAX_ICC` | 16,777,216 (16 MiB) | decompressed iCCP profile (P17 only) |

Decisions. The owner decided on 2026-11-17 (PNG-Q5, `../policy-draft/DECISIONS.md`) to raise the per-side cap from 16,384 to the PNG wire limit and the RGBA8 output cap from 64 MiB to 256 MiB; the coordinator raised the input cap from 256 MiB to 2^30 the same day, pending owner objection ("Input cap" below). The caps depend on the shared allocation guard's `MAX_BYTES` (per-reservation cap, copied byte-identically into every core) being raised from its frozen 67,108,864 to 2^30 (1 GiB), which alloc-delta erratum E16 did.

Derived bounds at these values: `R ≤ 2·MAX_OUTPUT + MAX_OUTPUT/2 + 2` = 671,088,642 (640 MiB), at most `MAX_BYTES` = 2^30; the looser form `2·MAX_OUTPUT + 2·MAX_DIMENSION + 2` would be 4,831,838,208 and does not fit one reservation. `8·MAX_INPUT` = 2^33 does not fit `u32`, so 64-bit bit positions are required.

**Guard dependence (PNG-Q5, decided 2026-11-17):** each reservation must be at most the guard's `MAX_BYTES`. The values above assume `MAX_BYTES` = 2^30 (alloc-delta erratum E16, applied separately); the largest single reservations are `Raw` for 16-bit RGBA (`R ≤ 640 MiB`, 576 MiB at the worst shape tried) and the RGBA16 `Output` (512 MiB), each below 2^30. Under the previous guard value (67,108,864) `Raw` for a 16-bit RGBA image, the RGBA16 `Output`, and even 8-bit RGBA `Raw` at `4N = MAX_OUTPUT` exceeded one reservation, so the effective caps were lower; that was open until this decision, and these caps are not implementable until E16 is in. **Input cap (decided 2026-11-17 by the coordinator, pending owner objection; `../policy-draft/DECISIONS.md`):** `MAX_INPUT` is 2^30, raised from 256 MiB. At 256 MiB a PNG whose image data is stored (uncompressed deflate blocks) was rejected `FileTooLarge` below the output cap: a stored 8192 × 8192 RGBA8 image is about 268 MB, already over 256 MiB, and a 16-bit RGBA one of 2^26 pixels about 537 MB. The input is borrowed, not reserved, so the raise changes no reservation and no live-heap figure; it needs the 64-bit bit positions above. At 2^30 every still image within the output caps fits a stored encoding: the largest `R` bound (671,088,642 bytes) stored in one zlib stream is 671,139,853 bytes, leaving about 402 MB for chunk overhead, enough for IDAT chunks that average at least 21 payload bytes. **Remaining open edge:** an APNG whose frames' stored data together exceed the input cap (frame data is bounded by `MAX_TOTAL_PIXELS`, not by `MAX_OUTPUT`) is still `FileTooLarge`; any further raise is again a strict widening.

Live heap at these values (SPEC P20 gives the bounds by name): the aggregate bound `R + 8N ≤ 4.5·MAX_OUTPUT + 2` is 1.125 GiB, an upper bound that no input attains (`R` is largest where `N` is smaller); the exact `1 × 2^26` RGBA16 shape gives `R` = 576 MiB and `R + 8N` = 1,088 MiB.

## Permitted values (the Stage-1 fixture envelope)

A cap tuple is permitted when it satisfies `caps_ok` (`../THEOREMS.md`, Constants) and every cap is at least the largest quantity a frozen Stage-1 vector compares with it:

| Constant | Lower bound | Upper bound (`caps_ok`, with `MAX_BYTES` = 2^30) | Record that sets the lower bound |
|---|---|---|---|
| `MAX_INPUT` | 13,110 | 2^60 | `idat/split-1-byte` |
| `MAX_DIMENSION` | 402 | 2^31 − 1 (the PNG wire limit) | `deflate/distance-beyond-declared-window` |
| `MAX_OUTPUT` | 1,608 | 429,496,728 (`5·MAX_OUTPUT/2 + 2 ≤ MAX_BYTES`) | `deflate/distance-beyond-declared-window` |
| `MAX_FRAMES` | 3 | 2^32 − 1 | `apng/default-is-frame0-dispose-bg-blend-over` |
| `MAX_TOTAL_PIXELS` | 24 | 2^62 | `apng/blend-over-transparent-source` |
| `MAX_ICC` | 768 | `MAX_BYTES` = 2^30 | `icc/present` |

Why this domain suffices. Each cap enters the Strict model only through one test of the form `quantity > cap` (`|b|`, `max(W, H)`, `4·W·H`, `num_frames`, the running sum of fcTL `w·h`, the inflated profile length), and the quantity does not depend on any cap. So a record whose quantities are at most the caps keeps its outcome when any cap rises, and a record whose quantity exceeds every permitted value (`ihdr/width-2^31`: `2^31 > 2^31 − 1`) fails the same test everywhere. `python3 -I ../spec-draft/check_vectors.py ../spec-draft/VECTORS.json --usage` recomputes the lower bounds; `--caps` evaluates the frozen records at the lower corner (all six caps at their lower bounds) and at the upper corner, and both pass 599/599 (2026-11-17). Each lower bound is tight: lowering any one cap by 1 alone makes one to five records fail.

A Stage-2 choice inside this domain needs no Stage-1 change. A value below a lower bound or above an upper bound changes a frozen record or breaks a proof premise, so it is a Stage-1 erratum. The cap-dependent records of `VECTORS-CAPS.json` are re-authored by `gen_vectors_caps.py` for the chosen values (its assertions name the values its record names assume).
