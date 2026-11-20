//! Independent remaining P3/P5 control rules, including EOF and CRC policy.
//! These components will be composed into MW; they are not the full walk.
use vstd::prelude::*;
use crate::{model, apng_model, png::Error, walk_model::{Name,Phase,apng}};

verus! {

pub struct Candidate {
    pub ty: Seq<u8>, pub data: nat, pub len: nat, pub next: nat,
    pub crc_ok: bool, pub synthetic: bool,
}

pub open spec fn read(b: Seq<u8>, p: nat, phase: Phase, allow_eof: bool) -> Result<Candidate,Error> {
    if p == b.len() {
        if !matches!(phase,Phase::Before) && allow_eof {
            Ok(Candidate { ty: seq![b'I',b'E',b'N',b'D'], data: p, len: 0, next: p, crc_ok: true, synthetic: true })
        } else { Err(Error::MissingIend) }
    } else {
        match model::frame(b,p) {
            Err(e) => Err(e),
            Ok(c) => Ok(Candidate { ty: c.ty, data: c.data, len: c.len, next: c.next,
                crc_ok: model::crc(b.subrange(c.data as int - 4,(c.data + c.len) as int))
                    == model::be32(b,(c.data + c.len) as int), synthetic: false }),
        }
    }
}

pub open spec fn effective(name: Name, ancillary: bool, apng_on: bool) -> (Name,bool) {
    if !apng_on && apng(name) { (Name::Unknown(true),true) } else { (name,ancillary) }
}

pub enum CrcAction { Keep, Drop, Fail(Error,bool) }

/// A bad fcTL is a structural failure only at the enabled frames-walk site
/// that would otherwise skip its ancillary CRC. fdAT remains mandatory even
/// when APNG is off; lenient IEND is a separate rule before ancillary handling.
pub open spec fn crc_rule(wire: Name, effective: Name, ancillary: bool, crc_ok: bool,
    skip_ancillary: bool, lenient_iend: bool, frames_entry: bool) -> CrcAction
{
    if crc_ok { CrcAction::Keep }
    else if effective == Name::Iend {
        if lenient_iend { CrcAction::Keep } else { CrcAction::Fail(Error::CrcMismatch,false) }
    } else if ancillary && !(wire == Name::Fdat && skip_ancillary) {
        if !skip_ancillary { CrcAction::Fail(Error::CrcMismatch,false) }
        else if effective == Name::Fctl && frames_entry { CrcAction::Fail(Error::CrcMismatch,true) }
        else { CrcAction::Drop }
    } else { CrcAction::Fail(Error::CrcMismatch,false) }
}

pub open spec fn idat(phase: Phase, ct: u8, palette_present: bool) -> Result<bool,Error> {
    if matches!(phase,Phase::After) { Err(Error::NonConsecutiveIdat) }
    else if ct == 3 && !palette_present { Err(Error::MissingPalette) }
    else { Ok(matches!(phase,Phase::Before)) }
}

pub open spec fn iend(phase: Phase, control: apng_model::Control, declared: Option<u32>,
    next: nat, input_len: nat, ignore_trailing: bool) -> Result<(),(Error,bool)>
{
    if matches!(phase,Phase::Before) { Err((Error::MissingImageData,false)) }
    else {
        match apng_model::finish(control,declared) {
            Err(e) => Err(e),
            Ok(()) => if next != input_len && !ignore_trailing { Err((Error::TrailingBytes,false)) }
                else { Ok(()) },
        }
    }
}

pub open spec fn unknown(ancillary: bool) -> Result<(),Error> {
    if ancillary { Ok(()) } else { Err(Error::UnknownCriticalChunk) }
}

pub enum Failure { Format(Error), Fallback }
pub open spec fn route(error: Error, structural: bool, static_frames: bool) -> Failure {
    if structural && static_frames { Failure::Fallback } else { Failure::Format(error) }
}

} // verus!
