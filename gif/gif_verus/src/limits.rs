//! G1 limits table (SPEC.md G1, Q8). Every resource cap is one named constant
//! here and nowhere else; the decoder and, later, the theorems refer to these
//! names only, so a cap changes by editing one line.

use vstd::prelude::*;

verus! {

/// Input bytes (G2): `|b| > MAX_INPUT` is `FileTooLarge`. 256 MiB (2^28).
pub const MAX_INPUT: usize = 268_435_456;

/// Screen width and height (G3). Equal to the LE16 wire limit, so it adds no
/// separate per-side cap; only zero fails.
pub const MAX_DIMENSION: u32 = 65_535;

/// Bytes of one RGBA8 canvas, `4·W·H` (G3). 64 MiB (2^26), 16,777,216 (2^24)
/// pixels. Held at 64 MiB because it must not exceed the shared alloc guard's
/// `MAX_BYTES` (67,108,864, copied byte-identically into every core); the owner
/// expects to raise both later, as a strict widening through an alloc-delta
/// erratum with re-verification of every core.
pub const MAX_OUTPUT: u64 = 67_108_864;

/// Image descriptors per stream (G8).
pub const MAX_FRAMES: u32 = 65_536;

/// Sum of `w·h` over the image descriptors processed (G8). 2^30.
pub const MAX_TOTAL_PIXELS: u64 = 1 << 30;

/// G1's derived bounds, checked once against the configured caps. This lemma
/// has no precondition: changing a cap outside this domain fails verification
/// here rather than silently adding an assumption to any decoder theorem.
/// The LE16 wire domain stays fixed even when MAX_DIMENSION is lowered.
pub proof fn caps_ok()
    ensures
        MAX_DIMENSION <= u16::MAX,
        (MAX_INPUT as int) + (u16::MAX as int) + 1 <= u32::MAX,
        (MAX_INPUT as int) + (u16::MAX as int) + 1 <= usize::MAX,
        MAX_OUTPUT <= crate::alloc_guard::MAX_BYTES,
        MAX_OUTPUT <= u32::MAX,
        MAX_OUTPUT <= usize::MAX,
        4 * (MAX_DIMENSION as int) * (MAX_DIMENSION as int) <= u64::MAX,
        (MAX_TOTAL_PIXELS as int) + (MAX_DIMENSION as int) * (MAX_DIMENSION as int) <= u64::MAX,
        (MAX_DIMENSION as int) * (MAX_DIMENSION as int) <= u32::MAX,
        3 * (MAX_DIMENSION as int) <= u32::MAX,
{
    assert((MAX_TOTAL_PIXELS as int) + (MAX_DIMENSION as int) * (MAX_DIMENSION as int) <= u64::MAX) by(compute);
}

} // verus!
