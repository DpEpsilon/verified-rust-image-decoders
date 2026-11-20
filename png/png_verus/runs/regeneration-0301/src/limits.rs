//! PNG-CORE-SPEC P1 resource caps. The only Rust that writes the six cap
//! values (`../../limits-draft/LIMITS.md`). The values are DRAFT: they are held
//! out of the Stage-1 freeze until the corpus pass (M0-corpus, PNG-Q7) is re-run
//! under them, and come from `../../limits-draft/limits.py`. A Stage-2 change
//! edits this file only; every other module names the constants, never their
//! values (gate: `tools/lint_caps.sh`).
//!
//! Permitted values (SPEC P1, LIMITS.md "Permitted values"): any tuple for which
//! `caps_ok` holds and each cap is at least the Stage-1 fixture envelope.

use vstd::prelude::*;

verus! {

/// Format limit for chunk lengths and IHDR/fcTL dimensions, not a resource cap.
pub const PNG_WIRE_MAX: u32 = 0x7fff_ffff;

/// Input bytes (P2): `|b| > MAX_INPUT` is `FileTooLarge`. 2^30 (draft value).
pub const MAX_INPUT: u64 = 1 << 30;

/// IHDR width and height (P4). The PNG wire limit 2^31 − 1 (draft value).
pub const MAX_DIMENSION: u32 = 0x7fff_ffff;

/// Bytes of one RGBA8 image or canvas, `4·W·H` (P4). 256 MiB (draft value).
pub const MAX_OUTPUT: u64 = 256 * 1024 * 1024;

/// acTL `num_frames` (P14.1) (draft value).
pub const MAX_FRAMES: u32 = 1 << 16;

/// Sum of `w·h` over the fcTL frames of one stream (P14.1). 2^30 (draft value).
pub const MAX_TOTAL_PIXELS: u64 = 1 << 30;

/// Bytes of a decompressed iCCP profile (P17). 16 MiB (draft value).
pub const MAX_ICC: u64 = 16 * 1024 * 1024;

/// The arithmetic premises the decoder relies on, collected in one lemma
/// (`../../THEOREMS.md` Constants, PLAN.md "Every limit ..."). Changing a cap
/// outside this domain fails here rather than silently adding an assumption.
pub proof fn caps_ok()
    ensures
        1 <= MAX_DIMENSION <= 0x7fff_ffffu32,
        4 <= MAX_OUTPUT,
        5 * (MAX_OUTPUT as int) / 2 + 2 <= crate::alloc_guard::MAX_BYTES as int,
        2 * (MAX_OUTPUT as int) <= crate::alloc_guard::MAX_BYTES as int,
        1 <= MAX_ICC <= crate::alloc_guard::MAX_BYTES as int,
        MAX_INPUT <= 0x1000_0000_0000_0000u64,
        MAX_INPUT <= inflate_verus::limits::SEGMENT_BYTES_MAX,
        1 <= MAX_FRAMES <= 0xffff_ffffu32,
        MAX_TOTAL_PIXELS <= 0x4000_0000_0000_0000u64,
        crate::alloc_guard::MAX_BYTES <= 0x7fff_ffffusize,
        inflate_verus::limits::WINDOW_MAX <= crate::alloc_guard::MAX_BYTES,
{
    assert(MAX_INPUT <= 0x1000_0000_0000_0000u64) by (bit_vector);
    assert(inflate_verus::limits::SEGMENT_BYTES_MAX == 0x1000_0000_0000_0000u64) by (bit_vector);
    assert(1 <= MAX_FRAMES <= 0xffff_ffffu32) by (bit_vector);
    assert(MAX_TOTAL_PIXELS <= 0x4000_0000_0000_0000u64) by (bit_vector);
    assert(inflate_verus::limits::WINDOW_MAX <= crate::alloc_guard::MAX_BYTES) by (bit_vector);
}

} // verus!
