//! Segmented public-call harness for the implemented window/tail option domain.
use crate::{model as m, InflateOptions, SegmentSlices, Status, Window, ZlibTail};
use vstd::prelude::*;
verus! {
pub(crate) fn check_policy_parts(parts: &[&[u8]], output: &mut [u8], enforce: bool, ignore_tail: bool, expected: Status, raw: Ghost<Seq<u8>>)
    requires crate::segments::parts_prefix(parts@,parts@.len()).len() <= crate::limits::SEGMENT_BYTES_MAX,
        raw@.len() <= old(output).len(),
        crate::status_eq(expected,m::Inflate(crate::segments::parts_prefix(parts@,parts@.len()),old(output).len() as nat,enforce,ignore_tail).status),
        raw@ == m::Inflate(crate::segments::parts_prefix(parts@,parts@.len()),old(output).len() as nat,enforce,ignore_tail).out,
{
    hide(m::Inflate);
    let options = InflateOptions { window: if enforce {Window::Enforce} else {Window::Ignore},
        zlib_tail: if ignore_tail {ZlibTail::IgnoreTail} else {ZlibTail::Reject}, ..InflateOptions::STRICT };
    let limit = output.len() as u64;
    let mut source = match SegmentSlices::new(parts) { Some(s) => s, None => { proof { assert(false); } return; } };
    let result = crate::inflate_zlib_into(&mut source,output,options);
    proof { assert(result.status == expected); assert(result.produced == raw@.len()); assert(output@.take(result.produced as int) == raw@); }
    let mut source = match SegmentSlices::new(parts) { Some(s) => s, None => { proof { assert(false); } return; } };
    let mut ring = [0xccu8;crate::limits::WINDOW_MAX];
    proof { crate::limits::caps_ok(); assert(crate::limits::WINDOW_MAX == 32768) by (compute); }
    let measured = crate::inflate_zlib_measure(&mut source,&mut ring,limit,options);
    proof { assert(measured.status == expected); assert(measured.produced == raw@.len()); assert(crate::ring_repr(ring@,raw@)); }
}
} // verus!
