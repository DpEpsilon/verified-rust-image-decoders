//! Policy regressions pinned to the complete observable result.
use vstd::prelude::*;

verus! {
#[cfg(verus_only)]
use crate::model::{ML, observation, ErrK};

pub proof fn stop_at_raster_overrun()
    ensures ML(2, seq![140u8, 11u8], 2, crate::lzw::recommended_beta_params())
        == observation(seq![1u8, 1u8], None, 9, true),
{
    assert(ML(2, seq![140u8, 11u8], 2, crate::lzw::recommended_beta_params())
        == observation(seq![1u8, 1u8], None, 9, true)) by (compute);
}

pub proof fn strict_overrun_keeps_emitted_prefix()
    ensures ML(2, seq![140u8, 11u8], 2, crate::lzw::strict_params())
        == observation(seq![1u8, 1u8], Some(ErrK::TooManyPixels), 9, false),
{
    assert(ML(2, seq![140u8, 11u8], 2, crate::lzw::strict_params())
        == observation(seq![1u8, 1u8], Some(ErrK::TooManyPixels), 9, false)) by (compute);
}

pub proof fn strict_tail_precedes_checked_literal()
    ensures ML(9, seq![1u8, 176u8, 4u8], 1,
        crate::lzw::Params { stop_at_raster: false, ..crate::lzw::recommended_beta_params() })
        == observation(seq![1u8], Some(ErrK::MissingEndOfInformation), 20, false),
{
    assert(ML(9, seq![1u8, 176u8, 4u8], 1,
        crate::lzw::Params { stop_at_raster: false, ..crate::lzw::recommended_beta_params() })
        == observation(seq![1u8], Some(ErrK::MissingEndOfInformation), 20, false)) by (compute);
}
} // verus!
