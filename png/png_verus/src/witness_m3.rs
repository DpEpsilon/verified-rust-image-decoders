//! Initial M3 concrete call bindings. These are not the complete M3 witness
//! gates of THEOREMS.md:194; those remain explicitly open in LEDGER.md.
use vstd::prelude::*;
use crate::{crc::crc32, header::prefix, model, png::Error};

verus! {

fn crc_known_answers() {
    let empty: [u8; 0] = [];
    let zero = [0u8];
    let digits = [b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'];
    let a = crc32(&empty);
    let b = crc32(&zero);
    let c = crc32(&digits);
    proof {
        assert(empty@ == Seq::<u8>::empty());
        assert(zero@ == seq![0u8]);
        assert(digits@ == seq![b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9']);
        assert(model::crc(seq![]) == 0u32) by (compute);
        assert(model::crc(seq![0u8]) == 0xD202_EF8Du32) by (compute);
        assert(model::crc(seq![b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'])
            == 0xCBF4_3926u32) by (compute);
        assert(a == 0 && b == 0xD202_EF8D && c == 0xCBF4_3926);
    }
}

fn header_error_precedence() {
    let short = [0x89u8, b'P', b'N', b'G'];
    // Wire length exceeds PNG_WIRE_MAX; the type also contains an invalid
    // letter and the payload is absent. ChunkTooLong must win.
    let long = [0x89u8, b'P', b'N', b'G', 13, 10, 26, 10,
        0x80, 0, 0, 0, 0, b'H', b'D', b'R'];
    let a = prefix(&short);
    let b = prefix(&long);
    proof {
        assert(short@ == seq![0x89u8, b'P', b'N', b'G']);
        assert(long@ == seq![0x89u8, b'P', b'N', b'G', 13u8, 10u8, 26u8, 10u8,
            0x80u8, 0u8, 0u8, 0u8, 0u8, b'H', b'D', b'R']);
        assert(model::mh(seq![0x89u8, b'P', b'N', b'G']) == Err(Error::BadSignature)) by (compute);
        assert(model::mh(seq![0x89u8, b'P', b'N', b'G', 13u8, 10u8, 26u8, 10u8,
            0x80u8, 0u8, 0u8, 0u8, 0u8, b'H', b'D', b'R']) == Err(Error::ChunkTooLong)) by (compute);
        assert(a.is_err() && a.err().unwrap() == Error::BadSignature);
        assert(b.is_err() && b.err().unwrap() == Error::ChunkTooLong);
    }
}

/// Concrete production calls pin duplicate > ordering > fixed-length order.
fn ordering_precedence() {
    use crate::kind::{Kind, initial_seen, mark_seen};
    use crate::ordering::{preamble, OrderAction, Rules};
    let strict = Rules { first_duplicate_wins: false, drop_misordered: false,
        drop_invalid: false, ignore_unexpected_plte: false, lenient_iend: false };
    let mut seen = initial_seen();
    mark_seen(&mut seen, Kind::Gama);
    let duplicate = preamble(seen, 2, true, 2, Kind::Gama, true, 3, strict);
    let order = preamble(seen, 2, true, 2, Kind::Phys, true, 8, strict);
    let length = preamble(seen, 0, false, 2, Kind::Phys, true, 8, strict);
    proof {
        assert(matches!(duplicate, OrderAction::Fail(Error::DuplicateChunk, false)));
        assert(matches!(order, OrderAction::Fail(Error::ChunkOutOfOrder, false)));
        assert(matches!(length, OrderAction::Fail(Error::InvalidChunkLength, false)));
    }
}

/// Recovery selection is indexed by the chunk/stage, not by the error name.
fn ordering_recovery_sites() {
    use crate::kind::{Kind, initial_seen, mark_seen};
    use crate::ordering::{advance_phase, preamble, OrderAction, Rules};
    let drop = Rules { first_duplicate_wins: true, drop_misordered: true,
        drop_invalid: true, ignore_unexpected_plte: true, lenient_iend: true };
    let mut seen = initial_seen();
    mark_seen(&mut seen, Kind::Gama);
    let duplicate = preamble(seen, 2, true, 2, Kind::Gama, true, 3, drop);
    let actl = preamble(seen, 2, true, 2, Kind::Actl, true, 7, drop);
    let iend = preamble(seen, 1, false, 2, Kind::Iend, false, 9, drop);
    let mut idat = 1u8;
    advance_phase(&mut idat, Kind::Gama);
    let skipped = preamble(seen, idat, false, 2, Kind::Phys, true, 9, drop);
    proof {
        assert(matches!(duplicate, OrderAction::Drop));
        assert(matches!(actl, OrderAction::Fail(Error::ChunkOutOfOrder, true)));
        assert(matches!(iend, OrderAction::Keep(0)));
        assert(idat == 2 && matches!(skipped, OrderAction::Drop));
    }
}

/// Direct production-transition calls, not a claim about a whole PNG file.
fn apng_error_precedence() {
    use crate::apng::{self, Control};
    let empty: [u8; 0] = [];
    let zero = [0u8; 4];
    let state = Control::new();
    let a = apng::fctl(&empty, false, true, 1, 1, state);
    let b = apng::fdat(&empty, true, true, state);
    let c = apng::fdat(&empty, true, false, state);
    let d = apng::fdat(&zero, true, false, state);
    proof {
        assert(zero@ == seq![0u8, 0u8, 0u8, 0u8]);
        assert(model::be32(seq![0u8, 0u8, 0u8, 0u8], 0) == 0u32) by (compute);
        assert(matches!(a, Err((Error::MissingAnimationControl, true))));
        assert(matches!(b, Err((Error::ChunkOutOfOrder, true))));
        assert(matches!(c, Err((Error::InvalidChunkLength, true))));
        assert(matches!(d, Err((Error::UnexpectedFrameData, true))));
    }
}

/// A four-byte fdAT contributes frame-data presence even with no payload.
fn apng_empty_payload_presence() {
    use crate::apng::{self, Control};
    use crate::apng_model as m;
    if crate::limits::MAX_TOTAL_PIXELS == 0 { return; }
    let d = [0u8,0,0,0, 0,0,0,1, 0,0,0,1, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0];
    let seq_one = [0u8,0,0,1];
    let r = apng::fctl(&d, true, false, 1, 1, Control::new());
    proof {
        assert(d@ == seq![0u8,0u8,0u8,0u8, 0u8,0u8,0u8,1u8, 0u8,0u8,0u8,1u8,
            0u8,0u8,0u8,0u8, 0u8,0u8,0u8,0u8, 0u8,0u8,0u8,0u8, 0u8,0u8]);
        assert(m::parse_fctl(seq![0u8,0u8,0u8,0u8, 0u8,0u8,0u8,1u8, 0u8,0u8,0u8,1u8,
            0u8,0u8,0u8,0u8, 0u8,0u8,0u8,0u8, 0u8,0u8,0u8,0u8, 0u8,0u8])
            == m::FrameControl { width: 1, height: 1, x: 0, y: 0,
                delay_num: 0, delay_den: 0, dispose: 0, blend: 0 }) by (compute);
        assert(model::be32(seq![0u8,0u8,0u8,0u8],0) == 0u32) by (compute);
        assert(model::be32(d@, 0) == 0u32);
        let ghost f = m::parse_fctl(d@);
        assert(f.width == 1 && f.height == 1 && f.x == 0 && f.y == 0 && f.dispose == 0 && f.blend == 0);
        assert(m::initial().sequence == 0 && m::initial().pixel_total == 0);
        assert(matches!(m::initial().current, m::Current::None));
        assert(1 <= crate::limits::MAX_TOTAL_PIXELS);
        assert(d@.len() == 26);
        assert(model::be32(d@, 0) as nat == m::initial().sequence);
        assert(!matches!(m::initial().current, m::Current::Frame(false)));
        assert(!(f.width == 0 || f.height == 0 || (f.x as nat) + (f.width as nat) > 1nat
            || (f.y as nat) + (f.height as nat) > 1nat || f.dispose > 2 || f.blend > 1));
        assert((f.width as nat) * (f.height as nat) == 1) by (nonlinear_arith)
            requires f.width == 1, f.height == 1;
        assert(m::initial().pixel_total + (f.width as nat) * (f.height as nat) <= crate::limits::MAX_TOTAL_PIXELS);
        reveal(m::fctl);
        assert(m::fctl(d@, true, false, 1, 1, m::initial()).is_ok());
        assert(r.is_ok());
    }
    let (_, state) = match r { Ok(s) => s, Err(_) => return };
    let before = apng::finish(state, Some(1));
    let next = apng::fdat(&seq_one, true, false, state);
    proof {
        assert(seq_one@ == seq![0u8,0u8,0u8,1u8]);
        assert(model::be32(seq![0u8,0u8,0u8,1u8],0) == 1u32) by (compute);
        assert(matches!(before, Err((Error::MissingFrameData, true))));
        assert(next.is_ok());
    }
    let next = match next { Ok(s) => s, Err(_) => return };
    let after = apng::finish(next, Some(1));
    proof { assert(after.is_ok()); assert(matches!(next.view().current, m::Current::Frame(true))); }
}

/// Accepting the largest wire sequence advances to a non-wire value. A
/// following zero sequence is an error, not a wrapped continuation.
fn apng_sequence_does_not_wrap() {
    use crate::apng::{self, Control};
    if crate::limits::MAX_TOTAL_PIXELS == 0 { return; }
    let state = Control { seq: u32::MAX as u64, cur: Some((false, true)), frames: 1,
        fctl_before_idat: false, pixels: 1 };
    let last = [255u8; 4];
    let zero = [0u8; 4];
    let r = apng::fdat(&last, true, false, state);
    proof {
        assert(last@ == seq![255u8,255u8,255u8,255u8]);
        assert(model::be32(seq![255u8,255u8,255u8,255u8],0) == u32::MAX) by (compute);
        assert(r.is_ok());
    }
    let next = match r { Ok(s) => s, Err(_) => return };
    proof { assert(next.seq == u32::MAX as int + 1); }
    let r = apng::fdat(&zero, true, false, next);
    proof {
        assert(zero@ == seq![0u8,0u8,0u8,0u8]);
        assert(model::be32(seq![0u8,0u8,0u8,0u8],0) == 0u32) by (compute);
        assert(matches!(r, Err((Error::SequenceError, true))));
    }
}

} // verus!
