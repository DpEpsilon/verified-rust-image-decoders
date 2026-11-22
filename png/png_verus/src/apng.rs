//! Verified P14.1 control transitions. The sequence counter is widened to
//! u64: after wire sequence u32::MAX it becomes u32::MAX + 1, never zero.
use vstd::prelude::*;
use crate::framing::{be16, be32};
use crate::limits::{MAX_FRAMES, MAX_TOTAL_PIXELS, PNG_WIRE_MAX};
use crate::png::Error;
#[cfg(verus_only)]
use crate::apng_model as m;

verus! {

#[derive(Clone, Copy, Debug)]
pub(crate) struct Fctl {
    pub w: u32, pub h: u32, pub x: u32, pub y: u32,
    pub delay_num: u16, pub delay_den: u16, pub dispose: u8, pub blend: u8,
}
impl Fctl {
    pub(crate) open spec fn view(self) -> m::FrameControl {
        m::FrameControl { width: self.w, height: self.h, x: self.x, y: self.y,
            delay_num: self.delay_num, delay_den: self.delay_den, dispose: self.dispose, blend: self.blend }
    }
    pub(crate) fn parse(d: &[u8]) -> (r: Self)
        requires d.len() == 26,
        ensures r.view() == m::parse_fctl(d@),
    {
        Self { w: be32(d, 4), h: be32(d, 8), x: be32(d, 12), y: be32(d, 16),
            delay_num: be16(d, 20), delay_den: be16(d, 22), dispose: d[24], blend: d[25] }
    }
}

/// Fixed-size runtime control record, with no chunk or frame list.
#[derive(Clone, Copy)]
pub(crate) struct Control {
    pub seq: u64,
    pub cur: Option<(bool, bool)>,
    pub frames: u64,
    pub fctl_before_idat: bool,
    pub pixels: u64,
}
impl Control {
    pub(crate) open spec fn view(self) -> m::Control {
        m::Control { sequence: self.seq as nat,
            current: match self.cur { None => m::Current::None,
                Some((true, _)) => m::Current::Default, Some((false, seen)) => m::Current::Frame(seen) },
            frame_count: self.frames as nat, fctl_before_idat: self.fctl_before_idat, pixel_total: self.pixels as nat }
    }
    pub(crate) open spec fn wf(self) -> bool {
        self.seq <= u32::MAX as int + 1 && self.frames <= self.pixels <= MAX_TOTAL_PIXELS
    }
    pub(crate) fn new() -> (r: Self)
        ensures r.wf(), r.view() == m::initial(),
    { Self { seq: 0, cur: None, frames: 0, fctl_before_idat: false, pixels: 0 } }

    pub(crate) fn begin_idat(&mut self) -> (default_is_frame: bool)
        ensures final(self).view() == m::begin_idat(old(self).view()),
            old(self).wf() ==> final(self).wf(),
            default_is_frame == old(self).cur.is_some(),
    {
        match self.cur {
            Some((_, seen)) => { self.cur = Some((true, seen)); true },
            None => false,
        }
    }
}

pub(crate) fn actl(d: &[u8]) -> (r: Result<(u32, u32), (Error, bool)>)
    ensures r == m::actl(d@),
{
    if d.len() != 8 { return Err((Error::InvalidChunkLength, true)); }
    let count = be32(d, 0);
    if count == 0 { return Err((Error::InvalidAnimationControl, true)); }
    if count > MAX_FRAMES { return Err((Error::TooManyFrames, false)); }
    Ok((count, be32(d, 4)))
}

proof fn pixel_add_safe(w: u32, h: u32, previous: u64)
    requires 1 <= w <= PNG_WIRE_MAX, 1 <= h <= PNG_WIRE_MAX, previous <= MAX_TOTAL_PIXELS,
    ensures 1 <= (w as int) * (h as int) <= u64::MAX,
        previous as int + (w as int) * (h as int) <= u64::MAX,
{
    crate::limits::caps_ok();
    assert(1 <= (w as int) * (h as int) <= u64::MAX) by (nonlinear_arith)
        requires 1 <= w <= PNG_WIRE_MAX, 1 <= h <= PNG_WIRE_MAX;
    assert(previous as int + (w as int) * (h as int) <= u64::MAX) by (nonlinear_arith)
        requires 1 <= w <= PNG_WIRE_MAX, 1 <= h <= PNG_WIRE_MAX,
            previous <= MAX_TOTAL_PIXELS, MAX_TOTAL_PIXELS as int <= (u64::MAX as int + 1) / 4;
}

pub(crate) fn fctl(d: &[u8], actl_present: bool, before_idat: bool,
    width: u32, height: u32, state: Control) -> (r: Result<(Fctl, Control), (Error, bool)>)
    requires state.wf(), width <= PNG_WIRE_MAX, height <= PNG_WIRE_MAX,
    ensures match r {
        Err(e) => m::fctl(d@, actl_present, before_idat, width, height, state.view()) == Err(e),
        Ok((f, next)) => next.wf() && 1<=f.w && 1<=f.h
            && f.x as int+f.w as int<=width && f.y as int+f.h as int<=height && f.dispose<=2 && f.blend<=1
            && m::fctl(d@, actl_present, before_idat, width, height, state.view()) == Ok((f.view(), next.view())),
    },
{
    if !actl_present { return Err((Error::MissingAnimationControl, true)); }
    if before_idat && state.fctl_before_idat { return Err((Error::ChunkOutOfOrder, true)); }
    if d.len() != 26 { return Err((Error::InvalidChunkLength, true)); }
    if be32(d, 0) as u64 != state.seq { return Err((Error::SequenceError, true)); }
    let seq = state.seq + 1;
    if let Some((false, false)) = state.cur { return Err((Error::MissingFrameData, true)); }
    let f = Fctl::parse(d);
    proof {
        crate::layout_model::rect_sum_unnarrowed(f.x,f.w);
        crate::layout_model::rect_sum_unnarrowed(f.y,f.h);
    }
    if f.w == 0 || f.h == 0 || f.x as u64 + f.w as u64 > width as u64
        || f.y as u64 + f.h as u64 > height as u64 || f.dispose > 2 || f.blend > 1 {
        return Err((Error::InvalidFrameControl, true));
    }
    proof {
        crate::layout_model::rect_sum_wire_bound(f.x,f.w);
        crate::layout_model::rect_sum_wire_bound(f.y,f.h);
    }
    if before_idat && (f.x != 0 || f.y != 0 || f.w != width || f.h != height) {
        return Err((Error::InvalidFrameControl, true));
    }
    proof { pixel_add_safe(f.w, f.h, state.pixels); }
    let pixels = state.pixels + f.w as u64 * f.h as u64;
    if pixels > MAX_TOTAL_PIXELS { return Err((Error::PixelBudgetExceeded, false)); }
    let frames = state.frames + 1;
    Ok((f, Control { seq, cur: Some((false, false)), frames,
        fctl_before_idat: state.fctl_before_idat || before_idat, pixels }))
}

pub(crate) fn fdat(d: &[u8], actl_present: bool, before_idat: bool,
    state: Control) -> (r: Result<Control, (Error, bool)>)
    requires state.wf(),
    ensures match r {
        Err(e) => m::fdat(d@, actl_present, before_idat, state.view()) == Err(e),
        Ok(next) => next.wf() && m::fdat(d@, actl_present, before_idat, state.view()) == Ok(next.view()),
    },
{
    if !actl_present { return Err((Error::MissingAnimationControl, true)); }
    if before_idat { return Err((Error::ChunkOutOfOrder, true)); }
    if d.len() < 4 { return Err((Error::InvalidChunkLength, true)); }
    if be32(d, 0) as u64 != state.seq { return Err((Error::SequenceError, true)); }
    let seq = state.seq + 1;
    match state.cur {
        Some((false, _)) => Ok(Control { seq, cur: Some((false, true)), frames: state.frames,
            fctl_before_idat: state.fctl_before_idat, pixels: state.pixels }),
        _ => Err((Error::UnexpectedFrameData, true)),
    }
}

pub(crate) fn finish(state: Control, declared: Option<u32>) -> (r: Result<(), (Error, bool)>)
    ensures r == m::finish(state.view(), declared),
{
    if let Some((false, false)) = state.cur { return Err((Error::MissingFrameData, true)); }
    if let Some(count) = declared {
        if state.frames != count as u64 { return Err((Error::FrameCountMismatch, true)); }
    }
    Ok(())
}

} // verus!
