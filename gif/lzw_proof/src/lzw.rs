//! M2: the GIF LZW decoder core (T-LZW), inside `verus!`.
//!
//! Executable form of `crate::model`'s `ML` (SPEC G10). The tables are
//! caller-provided slices, so `lzw_decode` allocates nothing; it is total (no
//! panic, no out-of-bounds access, no unchecked arithmetic, no recursion) for
//! every representable combination of arguments and equals `ML` exactly on its
//! contract domain.
//!
//! Table layout (SPEC G18's one 20,480-byte `LzwTable`, split across the caller's
//! three slices here so the same proof serves a parser that holds one byte buffer):
//!   * `prefix[k]`, `suffix[k]` for `k < 4096`: entry `k`'s parent code and last
//!     index. Literal entries (`k < clear`) are never stored: a code below `clear`
//!     is a literal *by rule*, which is the coordinator's D20 ruling that no
//!     per-frame initialisation of the `2^m` literal entries happens.
//!   * `length[k]`: entry `k`'s string length. SPEC G18 packs it as a
//!     little-endian `u16` pair at byte `12288 + 2*k`. This standalone core uses
//!     a `u16` slice; connecting these typed views to the raw byte accessors belongs
//!     to parser/allocator integration.

use vstd::prelude::*;

#[cfg(verus_only)]
use crate::model::{ErrK, MRes};


verus! {

/// G16's names, restricted to the seven errors G10.2 can return (G12.2's
/// `ColorIndexOutOfRange` is the compositor's and the parser owns
/// `TruncatedImageData`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LzwError {
    InvalidLzwMinCodeSize,
    InvalidLzwCode,
    TooManyPixels,
    TooFewPixels,
    MissingEndOfInformation,
    DataAfterEndOfInformation,
    ColorIndexOutOfRange,
    /// Not a SPEC G16 error: the caller's slices cannot hold this `(m, d, a)`.
    /// `lzw_decode` returns it instead of writing out of bounds, so the function is
    /// panic-free for every caller. T-LZW's contract domain
    /// (`out.len() >= a`, three 4,096-entry views, valid policy bounds) makes it
    /// unreachable. Invalid caller buffers or policy bounds return this guard error.
    BufferTooSmall,
}

/// `ErrK` to `LzwError`, the bijection the equality proof travels along.
pub open spec fn err_of(k: ErrK) -> LzwError {
    match k {
        ErrK::InvalidLzwMinCodeSize => LzwError::InvalidLzwMinCodeSize,
        ErrK::InvalidLzwCode => LzwError::InvalidLzwCode,
        ErrK::TooManyPixels => LzwError::TooManyPixels,
        ErrK::TooFewPixels => LzwError::TooFewPixels,
        ErrK::MissingEndOfInformation => LzwError::MissingEndOfInformation,
        ErrK::DataAfterEndOfInformation => LzwError::DataAfterEndOfInformation,
        ErrK::ColorIndexOutOfRange => LzwError::ColorIndexOutOfRange,
    }
}

/// G9's minimum-code-size policy (`policy-draft/OPTIONS.md` field `lzw_min`) plus
/// the G10.2 switches this milestone's proofs depend on. Strict and RecommendedBeta
/// instantiate the record; nothing in the decoder tests a profile by name, so the
/// model covers every record in the option table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Params {
    /// Smallest accepted `m` (`lzw_min`: 2 for `Spec`, 1 for `Range1to11*`).
    pub m_lo: u8,
    /// Largest accepted `m` (8 for `Spec`, 11 otherwise).
    pub m_hi: u8,
    /// `lzw_min = Range1to11Checked`'s size-1 rule (Q6): `m = 1` only at `A = 1`.
    pub size1_only_one_pixel: bool,
    /// `Range1to11Checked`: a literal above 255 is `ColorIndexOutOfRange`.
    pub checked_literals: bool,
    /// Retained policy bit. Both values reject code 4096: a 12-bit reader cannot
    /// produce it. Entry insertion always stops at 4096, as the policy oracle does;
    /// this bit has no observable effect in the represented policy subset.
    pub deferred_clear: bool,
    /// `lzw_tail = StopAtRaster` (RecommendedBeta): the loop returns as soon as the
    /// raster is complete, so no tail error is reported. `false` is Strict's
    /// `lzw_tail = Strict`.
    pub stop_at_raster: bool,
}

/// Strict's record, as an executable value, so the release decoder and its tests can
/// build it outside `verus!`. The spec function below is the same record; the two are
/// pinned by `lemma_strict_params_eq`.
pub fn strict() -> (r: Params)
    ensures
        r.m_lo == 2u8,
        r.m_hi == 8u8,
        !r.size1_only_one_pixel,
        !r.checked_literals,
        r.deferred_clear,
        !r.stop_at_raster,
{
    Params {
        m_lo: 2,
        m_hi: 8,
        size1_only_one_pixel: false,
        checked_literals: false,
        deferred_clear: true,
        stop_at_raster: false,
    }
}

/// RecommendedBeta's record, as an executable value.
pub fn recommended_beta() -> (r: Params)
    ensures
        r.m_lo == 1u8,
        r.m_hi == 11u8,
        r.size1_only_one_pixel,
        r.checked_literals,
        r.deferred_clear,
        r.stop_at_raster,
{
    Params {
        m_lo: 1,
        m_hi: 11,
        size1_only_one_pixel: true,
        checked_literals: true,
        deferred_clear: true,
        stop_at_raster: true,
    }
}

/// Strict's record: G9 keeps `2 <= m <= 8`, `lzw_tail = Strict`, no checked
/// literals, the deferred clear.
pub open spec fn strict_params() -> Params {
    Params {
        m_lo: 2,
        m_hi: 8,
        size1_only_one_pixel: false,
        checked_literals: false,
        deferred_clear: true,
        stop_at_raster: false,
    }
}

/// RecommendedBeta's `lzw_min = Range1to11Checked` plus `lzw_tail = StopAtRaster`.
pub open spec fn recommended_beta_params() -> Params {
    Params {
        m_lo: 1,
        m_hi: 11,
        size1_only_one_pixel: true,
        checked_literals: true,
        deferred_clear: true,
        stop_at_raster: true,
    }
}

// G18's table contains prefix, suffix and length regions only. The decoder
// emits backwards directly into the caller's output and uses no scratch stack.

} // verus!

// ---------------------------------------------------------------------------
// The executable decoder. Its proof is the fold `L(st) = L(verdict-step(st))` of
// `crate::model`, one equation per loop iteration: the loop's ghost state is the
// model's `St`, and the loop invariant says the byte arrays and the ghost agree on
// every entry the model can read.
// ---------------------------------------------------------------------------

verus! {

#[cfg(verus_only)]
use crate::chain::{
    chain_ok, code_len, fuel_above, lemma_chain_step, lemma_fresh_entry, lemma_frame,
    lemma_len_bound, lemma_rev_is_reverse, lemma_root_literal, lemma_str_shape, rev_of, root_of,
    str_of, stored_len,
};
#[cfg(verus_only)]
use crate::model::{bit_at, clear_of, code_at, eoi_of, eoi_tail, init_st, ipow2, m_ok, nbits,
    semit, str_ok, verdict, widen, St, Vd};

/// One past the highest table index (SPEC G18's 4,096 entries).
pub const TABSIZE: usize = 4096;

/// The chain walk's budget for a stored entry: strictly more than any index the
/// model holds, so `fuel_above` holds at every entry and for every predecessor.
pub open spec fn tab_fuel() -> nat {
    4096
}

/// G9's minimum-code-size check on the executable's `m`, mirroring `model::m_ok`.
pub open spec fn params_ok(ps: Params, m: u8, a: int) -> bool {
    &&& ps.m_lo <= m <= ps.m_hi
    &&& (!(ps.size1_only_one_pixel && m <= 1) || a == 1)
}

/// `params_ok` implies the model's `m_ok`, so the model and the executable reject
/// the same `(m, a)` pairs.
pub proof fn lemma_m_ok_iff(ps: Params, m: u8, a: int)
    ensures
        params_ok(ps, m, a) <==> m_ok(ps, m as int, a),
{
    reveal(m_ok);
}

} // verus!

// G18 uses only the three table views; the decoder needs no scratch stack.
pub use crate::decode::{lzw_decode, Decoded};
#[cfg(verus_only)]
pub use crate::decode::result_eq;
