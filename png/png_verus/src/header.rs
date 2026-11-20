//! P2–P4 exact header prefix (T2), with symbolic cap premises.
use vstd::prelude::*;
use crate::framing::{be32, chunk_at};
use crate::limits::{MAX_DIMENSION, MAX_INPUT, MAX_OUTPUT, PNG_WIRE_MAX};
use crate::png::Error;
#[cfg(verus_only)]
use crate::model;

verus! {

#[derive(Clone, Copy, Debug)]
pub(crate) struct Ihdr {
    pub w: u32,
    pub h: u32,
    pub depth: u8,
    pub ct: u8,
    pub interlace: u8,
}

impl Ihdr {
    pub(crate) open spec fn view(self) -> model::Ihdr {
        model::Ihdr { width: self.w, height: self.h, depth: self.depth, color_type: self.ct, interlace: self.interlace }
    }
    pub(crate) open spec fn wf(self) -> bool {
        1 <= self.w <= MAX_DIMENSION && 1 <= self.h <= MAX_DIMENSION
        && 4 * (self.w as int) * (self.h as int) <= MAX_OUTPUT
        && model::valid_depth(self.ct, self.depth) && self.interlace <= 1
    }
    pub(crate) fn channels(&self) -> (r: u32)
        ensures 1 <= r <= 4,r==if self.ct==0 || self.ct==3 {1int}else if self.ct==2 {3int}else if self.ct==4 {2int}else{4int},
    {
        match self.ct { 0 => 1, 2 => 3, 3 => 1, 4 => 2, _ => 4 }
    }
    pub(crate) fn bits_pp(&self) -> (r: u32)
        ensures r <= 4 * 255,r==crate::layout_model::bits(self.view()),
            self.wf() ==> r<=64,
    {
        let channels = self.channels();
        let depth = self.depth as u32;
        assert(channels * depth <= 4 * 255) by (nonlinear_arith)
            requires channels <= 4, depth <= 255;
        channels * depth
    }
    pub fn bpp(&self) -> (r: usize)
        ensures r >= 1,
    {
        let b = (self.bits_pp() / 8) as usize;
        if b == 0 { 1 } else { b }
    }
}

fn valid_depth(ct: u8, d: u8) -> (r: bool)
    ensures r == model::valid_depth(ct, d),
{
    match ct {
        0 => matches!(d, 1 | 2 | 4 | 8 | 16),
        2 | 4 | 6 => matches!(d, 8 | 16),
        3 => matches!(d, 1 | 2 | 4 | 8),
        _ => false,
    }
}

/// The wire bounds, not the configured cap numerals, justify the u64 product.
proof fn area_cap_u64(w: u32, h: u32)
    requires w <= PNG_WIRE_MAX, h <= PNG_WIRE_MAX,
    ensures 4 * (w as int) * (h as int) <= u64::MAX,
        4 * (w as int) <= u64::MAX,
{
    assert(4 * (w as int) * (h as int) <= u64::MAX) by (nonlinear_arith)
        requires w <= PNG_WIRE_MAX, h <= PNG_WIRE_MAX;
}

fn parse_ihdr(d: &[u8]) -> (r: Result<Ihdr, Error>)
    requires d.len() == 13,
    ensures match r {
        Err(e) => model::ihdr(d@) == Err(e),
        Ok(h) => h.wf() && model::ihdr(d@) == Ok(h.view()),
    },
{
    let w = be32(d, 0);
    let h = be32(d, 4);
    let (depth, ct, cm, fm, im) = (d[8], d[9], d[10], d[11], d[12]);
    if !(1 <= w && w <= MAX_DIMENSION && 1 <= h && h <= MAX_DIMENSION) {
        return Err(Error::InvalidDimensions);
    }
    proof { crate::limits::caps_ok(); area_cap_u64(w, h); }
    if 4 * (w as u64) * (h as u64) > MAX_OUTPUT { return Err(Error::OutputTooLarge); }
    if !valid_depth(ct, depth) { return Err(Error::InvalidColorTypeDepth); }
    if cm != 0 { return Err(Error::UnsupportedCompressionMethod); }
    if fm != 0 { return Err(Error::UnsupportedFilterMethod); }
    if im > 1 { return Err(Error::UnsupportedInterlaceMethod); }
    Ok(Ihdr { w, h, depth, ct, interlace: im })
}

fn signature(b: &[u8]) -> (r: bool)
    ensures r == model::signature(b@),
{
    b.len() >= 8 && b[0] == 0x89 && b[1] == 0x50 && b[2] == 0x4E && b[3] == 0x47
        && b[4] == 0x0D && b[5] == 0x0A && b[6] == 0x1A && b[7] == 0x0A
}

/// The actual walk calls this prefix. Its contract covers every slice, not
/// only valid PNGs; errors precede the IHDR-kind and IHDR-field tests exactly.
pub(crate) fn prefix(b: &[u8]) -> (r: Result<(Ihdr, usize), Error>)
    ensures match r {
        Err(e) => model::mh(b@) == Err(e),
        Ok((h, next)) => h.wf() && 8 < next <= b.len() && model::mh(b@) == Ok((h.view(), next as nat)),
    },
{
    if b.len() as u64 > MAX_INPUT { return Err(Error::FileTooLarge); }
    if !signature(b) { return Err(Error::BadSignature); }
    let c = match chunk_at(b, 8) { Ok(c) => c, Err(e) => return Err(e) };
    if !c.crc_ok { return Err(Error::CrcMismatch); }
    let is_ihdr = c.ty[0] == b'I' && c.ty[1] == b'H' && c.ty[2] == b'D' && c.ty[3] == b'R';
    proof { if is_ihdr { assert(c.ty@ =~= seq![b'I', b'H', b'D', b'R']); } }
    if !is_ihdr { return Err(Error::MissingIhdr); }
    if c.len != 13 { return Err(Error::InvalidChunkLength); }
    let h = match parse_ihdr(&b[c.data..c.data + 13]) { Ok(h) => h, Err(e) => return Err(e) };
    Ok((h, c.next))
}

} // verus!
