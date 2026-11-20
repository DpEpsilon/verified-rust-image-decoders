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

} // verus!
