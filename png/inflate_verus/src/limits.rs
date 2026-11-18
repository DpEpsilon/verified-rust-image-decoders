//! Format and work constants named by the frozen manifest. The six PNG resource
//! caps belong to png_verus, not to this allocation-free crate. caps_ok proves
//! inflate's format/domain facts without assuming any PNG resource-cap value.
use vstd::prelude::*;

verus! {
pub const MAX_CODE_BITS: usize = 15;
pub const WINDOW_MAX: usize = 1 << 15;
pub const LEN_MAX: usize = 258;
pub const NLIT_MAX: usize = 286;
pub const NDIST_MAX: usize = 30;
pub const NCL: usize = 19;
pub const FIXED_LIT_COUNT: usize = 288;
pub const FIXED_DIST_COUNT: usize = 32;
pub const DYNAMIC_LENGTHS: usize = NLIT_MAX + NDIST_MAX;
pub const CANON_FIXED_STEPS: usize = 3 * (MAX_CODE_BITS + 1) - 1;
pub const HUFF_BUILD_STEPS: usize = CANON_FIXED_STEPS + 2 * FIXED_LIT_COUNT + MAX_CODE_BITS + 1;
pub const HUFF_INIT_STEPS: usize = 4 * (MAX_CODE_BITS + 1) + FIXED_LIT_COUNT;
pub const CODE_SPACE: u32 = 1 << MAX_CODE_BITS;
pub const KRAFT_TERM_MAX: u32 = CODE_SPACE / 2;
pub const ADLER_MOD: u32 = 65_521;
pub const ADLER_NMAX: usize = 5_552;
pub const C_IN: u128 = 512;
pub const C_OUT: u128 = 8;
pub const C_INFL_FIXED: u128 = 4_096;
pub const C_SEG: u128 = 1;
/// T-SEG's domain ceiling, independent of PNG's configurable MAX_INPUT cap.
pub const SEGMENT_BYTES_MAX: u64 = 1 << 60;

/// The inflate component's domain and budget facts. PNG's six configurable
/// resource caps remain the PNG crate's obligation; inflate assumes none of them.
pub proof fn caps_ok()
    ensures 0 < WINDOW_MAX <= usize::MAX, LEN_MAX <= u16::MAX,
        8 * SEGMENT_BYTES_MAX + 64 < u64::MAX,
        C_IN == 8 * 64, 2 <= C_OUT, C_SEG == 1,
        5 * HUFF_INIT_STEPS + 2 * FIXED_LIT_COUNT + 2 * HUFF_BUILD_STEPS + FIXED_DIST_COUNT <= C_INFL_FIXED,
{
    assert(WINDOW_MAX == 32768) by (compute);
    assert(8 * SEGMENT_BYTES_MAX + 64 < u64::MAX) by (compute);
}

/// Format/type facts only; the PNG resource-cap caps_ok belongs to png_verus.
pub proof fn huffman_format_ok()
    ensures
        0 < MAX_CODE_BITS < u32::BITS,
        FIXED_LIT_COUNT < u16::MAX,
        FIXED_LIT_COUNT + MAX_CODE_BITS + 1 < usize::MAX,
        FIXED_LIT_COUNT * KRAFT_TERM_MAX <= u32::MAX,
        KRAFT_TERM_MAX <= u32::MAX,
        CANON_FIXED_STEPS == 2 * (MAX_CODE_BITS + 1) + MAX_CODE_BITS,
        CODE_SPACE == crate::model::pow2(MAX_CODE_BITS as nat),
        KRAFT_TERM_MAX == crate::model::pow2((MAX_CODE_BITS - 1) as nat),
{
    assert(FIXED_LIT_COUNT * KRAFT_TERM_MAX <= u32::MAX) by (compute);
    assert(CODE_SPACE == crate::model::pow2(MAX_CODE_BITS as nat)) by (compute);
    assert(KRAFT_TERM_MAX == crate::model::pow2((MAX_CODE_BITS - 1) as nat)) by (compute);
}
} // verus!
