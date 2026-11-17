//! G1 limits table (SPEC.md G1, Q8). Every resource cap is one named constant
//! here and nowhere else; the module and, later, the theorems refer to these
//! names only, so a cap change is one edit to G1 and one edit here.

use vstd::prelude::*;

verus! {

/// Input bytes (G2): `|b| > MAX_INPUT` is `FileTooLarge`. 256 MiB (2^28).
pub const MAX_INPUT: usize = 268_435_456;

/// Screen width and height (G3). Equal to the LE16 wire limit, so it adds no
/// separate per-side cap; only zero fails.
pub const MAX_DIMENSION: u32 = 65_535;

/// Bytes of one RGBA8 canvas, `4·W·H` (G3). 64 MiB (2^26), 16,777,216 (2^24)
/// pixels. This core cap is unchanged by alloc-delta ERRATA E16 (2026-11-17),
/// which raises the shared guard's `MAX_BYTES` to 1,073,741,824 bytes (1 GiB).
/// Raising this canvas cap needs a separate core erratum and re-verification.
pub const MAX_OUTPUT: u64 = 67_108_864;

/// Image descriptors per stream (G8).
pub const MAX_FRAMES: u32 = 65_536;

/// Sum of `w·h` over the image descriptors processed (G8). 2^30.
pub const MAX_TOTAL_PIXELS: u64 = 1 << 30;

} // verus!
