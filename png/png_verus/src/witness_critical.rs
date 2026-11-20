//! Direct production-call witnesses for the remaining control components.
//! Constructed local states are not whole-input M3 frozen-record witnesses.
use vstd::prelude::*;
use crate::{critical::{self,CrcAction}, apng::Control, kind::Kind, png::Error, walk::WalkFail};

verus! {

fn idat_first_error() {
    let mut phase = 2u8;
    let nonconsecutive = critical::idat(&mut phase,3,false);
    proof { assert(matches!(nonconsecutive,Err(Error::NonConsecutiveIdat))); assert(phase == 2); }
    phase = 0;
    let missing = critical::idat(&mut phase,3,false);
    proof { assert(matches!(missing,Err(Error::MissingPalette))); assert(phase == 0); }
    let first = critical::idat(&mut phase,3,true);
    proof { assert(matches!(first,Ok(true))); assert(phase == 1); }
    let next = critical::idat(&mut phase,3,true);
    proof { assert(matches!(next,Ok(false))); assert(phase == 1); }
}

fn iend_error_precedence() {
    let empty = Control::new();
    let pending = Control {seq:1,cur:Some((false,false)),frames:1,fctl_before_idat:false,pixels:1};
    let complete = Control {seq:2,cur:Some((false,true)),frames:1,fctl_before_idat:false,pixels:1};
    let a = critical::iend(0,empty,Some(2),10,11,false);
    let b = critical::iend(2,pending,Some(2),10,11,false);
    let c = critical::iend(2,complete,Some(2),10,11,false);
    let d = critical::iend(2,complete,Some(1),10,11,false);
    let e = critical::iend(2,complete,Some(1),10,11,true);
    let unknown = critical::unknown(false);
    let ancillary = critical::unknown(true);
    proof {
        assert(matches!(a,Err((Error::MissingImageData,false))));
        assert(matches!(b,Err((Error::MissingFrameData,true))));
        assert(matches!(c,Err((Error::FrameCountMismatch,true))));
        assert(matches!(d,Err((Error::TrailingBytes,false))));
        assert(e.is_ok());
        assert(matches!(unknown,Err(Error::UnknownCriticalChunk)));
        assert(ancillary.is_ok());
    }
}

fn crc_structural_sites() {
    let tolerated_fctl = critical::crc_rule(Kind::Fctl,Kind::Fctl,true,false,true,false,true);
    let rejected_fctl = critical::crc_rule(Kind::Fctl,Kind::Fctl,true,false,false,false,true);
    let still_fctl = critical::crc_rule(Kind::Fctl,Kind::Fctl,true,false,true,false,false);
    let (off_kind,off_anc) = critical::effective(Kind::Fctl,true,false);
    let off_fctl = critical::crc_rule(Kind::Fctl,off_kind,off_anc,false,true,false,true);
    let (off_kind,off_anc) = critical::effective(Kind::Fdat,true,false);
    let off_fdat = critical::crc_rule(Kind::Fdat,off_kind,off_anc,false,true,false,true);
    let lenient_iend = critical::crc_rule(Kind::Iend,Kind::Iend,false,false,false,true,true);
    let fallback = critical::route(Error::CrcMismatch,true,true);
    let mandatory = critical::route(Error::CrcMismatch,false,true);
    proof {
        assert(matches!(tolerated_fctl,CrcAction::Fail(Error::CrcMismatch,true)));
        assert(matches!(rejected_fctl,CrcAction::Fail(Error::CrcMismatch,false)));
        assert(matches!(still_fctl,CrcAction::Drop));
        assert(matches!(off_fctl,CrcAction::Drop));
        assert(matches!(off_fdat,CrcAction::Fail(Error::CrcMismatch,false)));
        assert(matches!(lenient_iend,CrcAction::Keep));
        assert(matches!(fallback,WalkFail::Fallback));
        assert(matches!(mandatory,WalkFail::Format(Error::CrcMismatch)));
    }
}

fn eof_candidate_boundaries() {
    let b = [0u8];
    let before_data = critical::read(&b,1,0,true);
    let strict_eof = critical::read(&b,1,1,false);
    let implicit_iend = critical::read(&b,1,1,true);
    let partial = critical::read(&b,0,1,true);
    let beyond = critical::read(&b,2,1,true);
    proof {
        assert(matches!(before_data,Err(Error::MissingIend)));
        assert(matches!(strict_eof,Err(Error::MissingIend)));
        assert(implicit_iend.is_ok());
        assert(matches!(partial,Err(Error::TruncatedChunk)));
        assert(matches!(beyond,Err(Error::TruncatedChunk)));
    }
    match implicit_iend {
        Ok(c) => { proof {
            assert(c.synthetic@ && c.chunk.data == 1 && c.chunk.len == 0 && c.chunk.next == 1 && c.chunk.crc_ok);
        } },
        Err(_) => {},
    }
}

} // verus!
