//! Force both public generic entry points into a no_std object for nm checks.
//! This is a build probe, not a replacement decoder or a proof entry point.
#![no_std]
#![forbid(unsafe_code)]
use inflate_verus::{
    inflate_zlib_into, inflate_zlib_measure, InflateOptions, InflateResult, SegmentSlices,
};

pub fn probe_into(data: &[u8], out: &mut [u8]) -> Option<InflateResult> {
    let parts = [data];
    let mut src = SegmentSlices::new(&parts)?;
    Some(inflate_zlib_into(&mut src, out, InflateOptions::STRICT))
}

pub fn probe_measure(data: &[u8], ring: &mut [u8], limit: u64) -> Option<InflateResult> {
    let parts = [data];
    let mut src = SegmentSlices::new(&parts)?;
    Some(inflate_zlib_measure(
        &mut src,
        ring,
        limit,
        InflateOptions::STRICT,
    ))
}
