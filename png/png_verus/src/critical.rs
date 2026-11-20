//! Remaining checked walk control: candidate/EOF framing, CRC-policy
//! selection, IDAT/IEND/unknown-critical checks and structural error routing.
use vstd::prelude::*;
use crate::{apng::{self,Control}, framing::{self,Chunk}, kind::{kind_of,Kind}, png::Error, walk::WalkFail};
#[cfg(verus_only)]
use crate::{critical_model as m, ordering::phase_view, walk_model as mw};

verus! {

pub(crate) struct Candidate {
    pub chunk: Chunk,
    pub kind: Kind,
    pub(crate) synthetic: Ghost<bool>,
}
impl Candidate {
    pub(crate) open spec fn view(self) -> m::Candidate {
        m::Candidate { ty: self.chunk.ty@, data: self.chunk.data as nat, len: self.chunk.len as nat,
            next: self.chunk.next as nat, crc_ok: self.chunk.crc_ok, synthetic: self.synthetic@ }
    }
    pub(crate) open spec fn wf(self, b: Seq<u8>, p: nat) -> bool {
        self.kind.view() == mw::classify(self.chunk.ty@)
        && self.chunk.data <= b.len() && self.chunk.len <= b.len() - self.chunk.data
        && self.chunk.next <= b.len()
        && if self.synthetic@ {
            p == b.len() && self.kind.view() == mw::Name::Iend
            && self.chunk.data == p && self.chunk.len == 0 && self.chunk.next == p && self.chunk.crc_ok
        } else { self.chunk.wf(b) && p + 12 <= self.chunk.next }
    }
}

/// This is the actual read site used by the walk, including the AtEof
/// synthetic IEND. A synthetic candidate has a bounded empty span, not the
/// ordinary framed-chunk invariant (there is no on-wire header or CRC).
pub(crate) fn read(b: &[u8], p: usize, idat: u8, allow_eof: bool) -> (r: Result<Candidate,Error>)
    ensures match r {
        Err(e) => m::read(b@,p as nat,phase_view(idat),allow_eof) == Err(e),
        Ok(c) => c.wf(b@,p as nat) && m::read(b@,p as nat,phase_view(idat),allow_eof) == Ok(c.view())
            && (c.synthetic@ ==> idat > 0 && allow_eof),
    },
{
    if p == b.len() {
        if idat > 0 && allow_eof {
            let ty = [b'I',b'E',b'N',b'D'];
            proof { assert(ty@ =~= seq![b'I',b'E',b'N',b'D']); }
            Ok(Candidate { chunk: Chunk { ty, data:p, len:0, next:p, crc_ok:true },
                kind:Kind::Iend, synthetic:Ghost(true) })
        } else { Err(Error::MissingIend) }
    } else {
        let c = match framing::chunk_at(b,p) { Ok(c) => c, Err(e) => return Err(e) };
        let kind = kind_of(&c.ty);
        Ok(Candidate { chunk:c, kind, synthetic:Ghost(false) })
    }
}

pub(crate) fn effective(kind: Kind, ancillary: bool, apng_on: bool) -> (r: (Kind,bool))
    ensures (r.0.view(),r.1) == m::effective(kind.view(),ancillary,apng_on),
{
    if !apng_on && kind.is_apng() { (Kind::Unknown(true),true) } else { (kind,ancillary) }
}

pub(crate) enum CrcAction { Keep, Drop, Fail(Error,bool) }
impl CrcAction {
    pub(crate) open spec fn view(self) -> m::CrcAction {
        match self { Self::Keep => m::CrcAction::Keep, Self::Drop => m::CrcAction::Drop,
            Self::Fail(e,s) => m::CrcAction::Fail(e,s) }
    }
}

pub(crate) fn crc_rule(wire: Kind, kind: Kind, ancillary: bool, crc_ok: bool,
    skip_ancillary: bool, lenient_iend: bool, frames_entry: bool) -> (r: CrcAction)
    ensures r.view() == m::crc_rule(wire.view(),kind.view(),ancillary,crc_ok,skip_ancillary,lenient_iend,frames_entry),
        matches!(wire,Kind::Fctl) && matches!(kind,Kind::Fctl) && frames_entry && !crc_ok
            ==> matches!(r,CrcAction::Fail(Error::CrcMismatch,_)),
        matches!(wire,Kind::Fdat) && !matches!(kind,Kind::Iend) && !crc_ok
            ==> matches!(r,CrcAction::Fail(Error::CrcMismatch,false)),
{
    if crc_ok { return CrcAction::Keep; }
    if matches!(kind,Kind::Iend) {
        return if lenient_iend { CrcAction::Keep } else { CrcAction::Fail(Error::CrcMismatch,false) };
    }
    if ancillary && !(matches!(wire,Kind::Fdat) && skip_ancillary) {
        if !skip_ancillary { return CrcAction::Fail(Error::CrcMismatch,false); }
        if matches!(kind,Kind::Fctl) && frames_entry { return CrcAction::Fail(Error::CrcMismatch,true); }
        return CrcAction::Drop;
    }
    CrcAction::Fail(Error::CrcMismatch,false)
}

pub(crate) fn idat(phase: &mut u8, ct: u8, palette_present: bool) -> (r: Result<bool,Error>)
    requires *old(phase) <= 2,
    ensures match r {
        Err(e) => *final(phase) == *old(phase)
            && m::idat(phase_view(*old(phase)),ct,palette_present) == Err(e),
        Ok(first) => *final(phase) == 1 && first == (*old(phase) == 0)
            && m::idat(phase_view(*old(phase)),ct,palette_present) == Ok(first),
    },
{
    if *phase == 2 { return Err(Error::NonConsecutiveIdat); }
    if ct == 3 && !palette_present { return Err(Error::MissingPalette); }
    let first = *phase == 0;
    *phase = 1;
    Ok(first)
}

pub(crate) fn iend(phase: u8, control: Control, declared: Option<u32>, next: usize,
    input_len: usize, ignore_trailing: bool) -> (r: Result<(),(Error,bool)>)
    ensures r == m::iend(phase_view(phase),control.view(),declared,next as nat,input_len as nat,ignore_trailing),
{
    if phase == 0 { return Err((Error::MissingImageData,false)); }
    match apng::finish(control,declared) { Ok(()) => {}, Err(e) => return Err(e) }
    if next != input_len && !ignore_trailing { return Err((Error::TrailingBytes,false)); }
    Ok(())
}

pub(crate) fn unknown(ancillary: bool) -> (r: Result<(),Error>)
    ensures r == m::unknown(ancillary),
{
    if ancillary { Ok(()) } else { Err(Error::UnknownCriticalChunk) }
}

pub(crate) fn route(error: Error, structural: bool, static_frames: bool) -> (r: WalkFail)
    ensures r.view() == m::route(error,structural,static_frames),
{
    if structural && static_frames { WalkFail::Fallback } else { WalkFail::Format(error) }
}

} // verus!
