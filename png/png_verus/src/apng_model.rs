//! Independent P14.1 local transition model. Counters are natural numbers;
//! the complete MW document/range trace will compose these transitions later.
use vstd::prelude::*;
use crate::png::Error;
use crate::model::{be16, be32};
use crate::limits::{MAX_FRAMES, MAX_TOTAL_PIXELS};

verus! {

pub enum Current { None, Default, Frame(bool) }
pub struct Control {
    pub sequence: nat,
    pub current: Current,
    pub frame_count: nat,
    pub fctl_before_idat: bool,
    pub pixel_total: nat,
}
pub struct FrameControl {
    pub width: u32, pub height: u32, pub x: u32, pub y: u32,
    pub delay_num: u16, pub delay_den: u16, pub dispose: u8, pub blend: u8,
}
pub open spec fn initial() -> Control {
    Control { sequence: 0, current: Current::None, frame_count: 0, fctl_before_idat: false, pixel_total: 0 }
}
pub open spec fn parse_fctl(d: Seq<u8>) -> FrameControl {
    FrameControl { width: be32(d, 4), height: be32(d, 8), x: be32(d, 12), y: be32(d, 16),
        delay_num: be16(d, 20), delay_den: be16(d, 22), dispose: d[24], blend: d[25] }
}

/// The Boolean on a failure distinguishes an APNG structural recovery site
/// from a mandatory resource failure; error spelling alone does not decide it.
pub open spec fn actl(d: Seq<u8>) -> Result<(u32, u32), (Error, bool)> {
    if d.len() != 8 { Err((Error::InvalidChunkLength, true)) }
    else {
        let count = be32(d, 0);
        if count == 0 { Err((Error::InvalidAnimationControl, true)) }
        else if count > MAX_FRAMES { Err((Error::TooManyFrames, false)) }
        else { Ok((count, be32(d, 4))) }
    }
}

pub open spec fn fctl(d: Seq<u8>, actl_present: bool, before_idat: bool,
    width: u32, height: u32, state: Control) -> Result<(FrameControl, Control), (Error, bool)>
{
    if !actl_present { Err((Error::MissingAnimationControl, true)) }
    else if before_idat && state.fctl_before_idat { Err((Error::ChunkOutOfOrder, true)) }
    else if d.len() != 26 { Err((Error::InvalidChunkLength, true)) }
    else if be32(d, 0) as nat != state.sequence { Err((Error::SequenceError, true)) }
    else if matches!(state.current, Current::Frame(false)) { Err((Error::MissingFrameData, true)) }
    else {
        let f = parse_fctl(d);
        if f.width == 0 || f.height == 0 || (f.x as nat) + (f.width as nat) > width as nat
            || (f.y as nat) + (f.height as nat) > height as nat || f.dispose > 2 || f.blend > 1 {
            Err((Error::InvalidFrameControl, true))
        } else if before_idat && (f.x != 0 || f.y != 0 || f.width != width || f.height != height) {
            Err((Error::InvalidFrameControl, true))
        } else {
            let pixels = state.pixel_total + (f.width as nat) * (f.height as nat);
            if pixels > MAX_TOTAL_PIXELS { Err((Error::PixelBudgetExceeded, false)) }
            else { Ok((f, Control { sequence: state.sequence + 1, current: Current::Frame(false),
                frame_count: state.frame_count + 1, fctl_before_idat: state.fctl_before_idat || before_idat,
                pixel_total: pixels })) }
        }
    }
}

pub open spec fn fdat(d: Seq<u8>, actl_present: bool, before_idat: bool,
    state: Control) -> Result<Control, (Error, bool)>
{
    if !actl_present { Err((Error::MissingAnimationControl, true)) }
    else if before_idat { Err((Error::ChunkOutOfOrder, true)) }
    else if d.len() < 4 { Err((Error::InvalidChunkLength, true)) }
    else if be32(d, 0) as nat != state.sequence { Err((Error::SequenceError, true)) }
    else if !matches!(state.current, Current::Frame(_)) { Err((Error::UnexpectedFrameData, true)) }
    else { Ok(Control { sequence: state.sequence + 1, current: Current::Frame(true),
        frame_count: state.frame_count, fctl_before_idat: state.fctl_before_idat, pixel_total: state.pixel_total }) }
}

pub open spec fn begin_idat(state: Control) -> Control {
    Control { sequence: state.sequence,
        current: if matches!(state.current, Current::None) { Current::None } else { Current::Default },
        frame_count: state.frame_count, fctl_before_idat: state.fctl_before_idat, pixel_total: state.pixel_total }
}

pub open spec fn finish(state: Control, declared: Option<u32>) -> Result<(), (Error, bool)> {
    if matches!(state.current, Current::Frame(false)) { Err((Error::MissingFrameData, true)) }
    else {
        match declared {
            Some(count) => if state.frame_count != count as nat { Err((Error::FrameCountMismatch, true)) } else { Ok(()) },
            None => Ok(()),
        }
    }
}

} // verus!
