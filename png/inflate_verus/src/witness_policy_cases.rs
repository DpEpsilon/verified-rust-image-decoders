//! Independently authored tail outcomes, including original later-IDAT boundaries.
use crate::{model as m, witness_policy::check_policy_parts, InflateError as E, Status};
use vstd::prelude::*;
verus! {
// Policy policy/zlib-tail/trailing-byte: authored raw [0,65,0,66], exact R and non-exact R+1.
#[verifier::spinoff_prover]
fn trailing_byte() {
    let segment_0 = [0x78u8, 0x1, 0x1, 0x4, 0x0, 0xfb, 0xff, 0x0, 0x41, 0x0, 0x42, 0x1, 0x9, 0x0, 0x84, 0x0];
    let parts: [&[u8];1] = [&segment_0];
    proof {
        reveal_with_fuel(crate::segments::parts_prefix,2);
        assert(crate::segments::parts_prefix(parts@,1) =~= seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 0]);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 30) by (compute);
    }
    let ghost raw = seq![0u8,65,0,66];
    proof {
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 0],4,true,false).status == m::InflateStatus::Error(m::InflateFault::TrailingZlibData)) by (compute);
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 0],4,true,false).out == seq![0u8,65,0,66]) by (compute);
    }
    let mut output = [0xa5u8;4]; check_policy_parts(&parts,&mut output,true,false,Status::Err(E::TrailingZlibData),Ghost(raw));
    proof {
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 0],4,true,true).status == m::InflateStatus::Done) by (compute);
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 0],4,true,true).out == seq![0u8,65,0,66]) by (compute);
    }
    let mut output = [0xa5u8;4]; check_policy_parts(&parts,&mut output,true,true,Status::Done,Ghost(raw));
    proof {
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 0],4,false,false).status == m::InflateStatus::Error(m::InflateFault::TrailingZlibData)) by (compute);
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 0],4,false,false).out == seq![0u8,65,0,66]) by (compute);
    }
    let mut output = [0xa5u8;4]; check_policy_parts(&parts,&mut output,false,false,Status::Err(E::TrailingZlibData),Ghost(raw));
    proof {
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 0],4,false,true).status == m::InflateStatus::Done) by (compute);
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 0],4,false,true).out == seq![0u8,65,0,66]) by (compute);
    }
    let mut output = [0xa5u8;4]; check_policy_parts(&parts,&mut output,false,true,Status::Done,Ghost(raw));
    proof {
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 0],5,true,false).status == m::InflateStatus::Error(m::InflateFault::TrailingZlibData)) by (compute);
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 0],5,true,false).out == seq![0u8,65,0,66]) by (compute);
    }
    let mut output = [0xa5u8;5]; check_policy_parts(&parts,&mut output,true,false,Status::Err(E::TrailingZlibData),Ghost(raw));
    proof {
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 0],5,true,true).status == m::InflateStatus::Error(m::InflateFault::TrailingZlibData)) by (compute);
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 0],5,true,true).out == seq![0u8,65,0,66]) by (compute);
    }
    let mut output = [0xa5u8;5]; check_policy_parts(&parts,&mut output,true,true,Status::Err(E::TrailingZlibData),Ghost(raw));
    proof {
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 0],5,false,false).status == m::InflateStatus::Error(m::InflateFault::TrailingZlibData)) by (compute);
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 0],5,false,false).out == seq![0u8,65,0,66]) by (compute);
    }
    let mut output = [0xa5u8;5]; check_policy_parts(&parts,&mut output,false,false,Status::Err(E::TrailingZlibData),Ghost(raw));
    proof {
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 0],5,false,true).status == m::InflateStatus::Error(m::InflateFault::TrailingZlibData)) by (compute);
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 0],5,false,true).out == seq![0u8,65,0,66]) by (compute);
    }
    let mut output = [0xa5u8;5]; check_policy_parts(&parts,&mut output,false,true,Status::Err(E::TrailingZlibData),Ghost(raw));
}
// Policy policy/zlib-tail/second-stream: authored raw [0,65,0,66], exact R and non-exact R+1.
#[verifier::spinoff_prover]
fn second_stream() {
    let segment_0 = [0x78u8, 0x1, 0x1, 0x4, 0x0, 0xfb, 0xff, 0x0, 0x41, 0x0, 0x42, 0x1, 0x9, 0x0, 0x84, 0x78, 0x1, 0x1, 0x4, 0x0, 0xfb, 0xff, 0x0, 0x41, 0x0, 0x42, 0x1, 0x9, 0x0, 0x84];
    let parts: [&[u8];1] = [&segment_0];
    proof {
        reveal_with_fuel(crate::segments::parts_prefix,2);
        assert(crate::segments::parts_prefix(parts@,1) =~= seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 120, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132]);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 30) by (compute);
    }
    let ghost raw = seq![0u8,65,0,66];
    proof {
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 120, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132],4,true,false).status == m::InflateStatus::Error(m::InflateFault::TrailingZlibData)) by (compute);
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 120, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132],4,true,false).out == seq![0u8,65,0,66]) by (compute);
    }
    let mut output = [0xa5u8;4]; check_policy_parts(&parts,&mut output,true,false,Status::Err(E::TrailingZlibData),Ghost(raw));
    proof {
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 120, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132],4,true,true).status == m::InflateStatus::Done) by (compute);
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 120, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132],4,true,true).out == seq![0u8,65,0,66]) by (compute);
    }
    let mut output = [0xa5u8;4]; check_policy_parts(&parts,&mut output,true,true,Status::Done,Ghost(raw));
    proof {
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 120, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132],4,false,false).status == m::InflateStatus::Error(m::InflateFault::TrailingZlibData)) by (compute);
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 120, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132],4,false,false).out == seq![0u8,65,0,66]) by (compute);
    }
    let mut output = [0xa5u8;4]; check_policy_parts(&parts,&mut output,false,false,Status::Err(E::TrailingZlibData),Ghost(raw));
    proof {
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 120, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132],4,false,true).status == m::InflateStatus::Done) by (compute);
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 120, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132],4,false,true).out == seq![0u8,65,0,66]) by (compute);
    }
    let mut output = [0xa5u8;4]; check_policy_parts(&parts,&mut output,false,true,Status::Done,Ghost(raw));
    proof {
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 120, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132],5,true,false).status == m::InflateStatus::Error(m::InflateFault::TrailingZlibData)) by (compute);
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 120, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132],5,true,false).out == seq![0u8,65,0,66]) by (compute);
    }
    let mut output = [0xa5u8;5]; check_policy_parts(&parts,&mut output,true,false,Status::Err(E::TrailingZlibData),Ghost(raw));
    proof {
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 120, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132],5,true,true).status == m::InflateStatus::Error(m::InflateFault::TrailingZlibData)) by (compute);
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 120, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132],5,true,true).out == seq![0u8,65,0,66]) by (compute);
    }
    let mut output = [0xa5u8;5]; check_policy_parts(&parts,&mut output,true,true,Status::Err(E::TrailingZlibData),Ghost(raw));
    proof {
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 120, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132],5,false,false).status == m::InflateStatus::Error(m::InflateFault::TrailingZlibData)) by (compute);
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 120, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132],5,false,false).out == seq![0u8,65,0,66]) by (compute);
    }
    let mut output = [0xa5u8;5]; check_policy_parts(&parts,&mut output,false,false,Status::Err(E::TrailingZlibData),Ghost(raw));
    proof {
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 120, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132],5,false,true).status == m::InflateStatus::Error(m::InflateFault::TrailingZlibData)) by (compute);
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 120, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132],5,false,true).out == seq![0u8,65,0,66]) by (compute);
    }
    let mut output = [0xa5u8;5]; check_policy_parts(&parts,&mut output,false,true,Status::Err(E::TrailingZlibData),Ghost(raw));
}
// Policy policy/zlib-tail/trailing-in-later-idat: authored raw [0,65,0,66], exact R and non-exact R+1.
#[verifier::spinoff_prover]
fn trailing_in_later_idat() {
    let segment_0 = [0x78u8, 0x1, 0x1, 0x4, 0x0, 0xfb, 0xff, 0x0, 0x41, 0x0, 0x42, 0x1, 0x9, 0x0, 0x84];
    let segment_1 = [0x0u8];
    let parts: [&[u8];2] = [&segment_0, &segment_1];
    proof {
        reveal_with_fuel(crate::segments::parts_prefix,3);
        assert(crate::segments::parts_prefix(parts@,2) =~= seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 0]);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 30) by (compute);
    }
    let ghost raw = seq![0u8,65,0,66];
    proof {
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 0],4,true,false).status == m::InflateStatus::Error(m::InflateFault::TrailingZlibData)) by (compute);
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 0],4,true,false).out == seq![0u8,65,0,66]) by (compute);
    }
    let mut output = [0xa5u8;4]; check_policy_parts(&parts,&mut output,true,false,Status::Err(E::TrailingZlibData),Ghost(raw));
    proof {
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 0],4,true,true).status == m::InflateStatus::Done) by (compute);
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 0],4,true,true).out == seq![0u8,65,0,66]) by (compute);
    }
    let mut output = [0xa5u8;4]; check_policy_parts(&parts,&mut output,true,true,Status::Done,Ghost(raw));
    proof {
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 0],4,false,false).status == m::InflateStatus::Error(m::InflateFault::TrailingZlibData)) by (compute);
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 0],4,false,false).out == seq![0u8,65,0,66]) by (compute);
    }
    let mut output = [0xa5u8;4]; check_policy_parts(&parts,&mut output,false,false,Status::Err(E::TrailingZlibData),Ghost(raw));
    proof {
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 0],4,false,true).status == m::InflateStatus::Done) by (compute);
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 0],4,false,true).out == seq![0u8,65,0,66]) by (compute);
    }
    let mut output = [0xa5u8;4]; check_policy_parts(&parts,&mut output,false,true,Status::Done,Ghost(raw));
    proof {
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 0],5,true,false).status == m::InflateStatus::Error(m::InflateFault::TrailingZlibData)) by (compute);
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 0],5,true,false).out == seq![0u8,65,0,66]) by (compute);
    }
    let mut output = [0xa5u8;5]; check_policy_parts(&parts,&mut output,true,false,Status::Err(E::TrailingZlibData),Ghost(raw));
    proof {
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 0],5,true,true).status == m::InflateStatus::Error(m::InflateFault::TrailingZlibData)) by (compute);
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 0],5,true,true).out == seq![0u8,65,0,66]) by (compute);
    }
    let mut output = [0xa5u8;5]; check_policy_parts(&parts,&mut output,true,true,Status::Err(E::TrailingZlibData),Ghost(raw));
    proof {
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 0],5,false,false).status == m::InflateStatus::Error(m::InflateFault::TrailingZlibData)) by (compute);
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 0],5,false,false).out == seq![0u8,65,0,66]) by (compute);
    }
    let mut output = [0xa5u8;5]; check_policy_parts(&parts,&mut output,false,false,Status::Err(E::TrailingZlibData),Ghost(raw));
    proof {
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 0],5,false,true).status == m::InflateStatus::Error(m::InflateFault::TrailingZlibData)) by (compute);
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 0],5,false,true).out == seq![0u8,65,0,66]) by (compute);
    }
    let mut output = [0xa5u8;5]; check_policy_parts(&parts,&mut output,false,true,Status::Err(E::TrailingZlibData),Ghost(raw));
}
// Policy policy/zlib-tail/bad-adler-with-trailing-byte-stays-an-error: authored raw [0,65,0,66], exact R and non-exact R+1.
#[verifier::spinoff_prover]
fn bad_adler_with_trailing_byte_stays_an_error() {
    let segment_0 = [0x78u8, 0x1, 0x1, 0x4, 0x0, 0xfb, 0xff, 0x0, 0x41, 0x0, 0x42, 0x0, 0x0, 0x0, 0x1, 0x0];
    let parts: [&[u8];1] = [&segment_0];
    proof {
        reveal_with_fuel(crate::segments::parts_prefix,2);
        assert(crate::segments::parts_prefix(parts@,1) =~= seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 0, 0, 0, 1, 0]);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 30) by (compute);
    }
    let ghost raw = seq![0u8,65,0,66];
    proof {
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 0, 0, 0, 1, 0],4,true,false).status == m::InflateStatus::Error(m::InflateFault::AdlerMismatch)) by (compute);
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 0, 0, 0, 1, 0],4,true,false).out == seq![0u8,65,0,66]) by (compute);
    }
    let mut output = [0xa5u8;4]; check_policy_parts(&parts,&mut output,true,false,Status::Err(E::AdlerMismatch),Ghost(raw));
    proof {
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 0, 0, 0, 1, 0],4,true,true).status == m::InflateStatus::Error(m::InflateFault::AdlerMismatch)) by (compute);
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 0, 0, 0, 1, 0],4,true,true).out == seq![0u8,65,0,66]) by (compute);
    }
    let mut output = [0xa5u8;4]; check_policy_parts(&parts,&mut output,true,true,Status::Err(E::AdlerMismatch),Ghost(raw));
    proof {
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 0, 0, 0, 1, 0],4,false,false).status == m::InflateStatus::Error(m::InflateFault::AdlerMismatch)) by (compute);
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 0, 0, 0, 1, 0],4,false,false).out == seq![0u8,65,0,66]) by (compute);
    }
    let mut output = [0xa5u8;4]; check_policy_parts(&parts,&mut output,false,false,Status::Err(E::AdlerMismatch),Ghost(raw));
    proof {
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 0, 0, 0, 1, 0],4,false,true).status == m::InflateStatus::Error(m::InflateFault::AdlerMismatch)) by (compute);
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 0, 0, 0, 1, 0],4,false,true).out == seq![0u8,65,0,66]) by (compute);
    }
    let mut output = [0xa5u8;4]; check_policy_parts(&parts,&mut output,false,true,Status::Err(E::AdlerMismatch),Ghost(raw));
    proof {
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 0, 0, 0, 1, 0],5,true,false).status == m::InflateStatus::Error(m::InflateFault::AdlerMismatch)) by (compute);
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 0, 0, 0, 1, 0],5,true,false).out == seq![0u8,65,0,66]) by (compute);
    }
    let mut output = [0xa5u8;5]; check_policy_parts(&parts,&mut output,true,false,Status::Err(E::AdlerMismatch),Ghost(raw));
    proof {
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 0, 0, 0, 1, 0],5,true,true).status == m::InflateStatus::Error(m::InflateFault::AdlerMismatch)) by (compute);
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 0, 0, 0, 1, 0],5,true,true).out == seq![0u8,65,0,66]) by (compute);
    }
    let mut output = [0xa5u8;5]; check_policy_parts(&parts,&mut output,true,true,Status::Err(E::AdlerMismatch),Ghost(raw));
    proof {
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 0, 0, 0, 1, 0],5,false,false).status == m::InflateStatus::Error(m::InflateFault::AdlerMismatch)) by (compute);
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 0, 0, 0, 1, 0],5,false,false).out == seq![0u8,65,0,66]) by (compute);
    }
    let mut output = [0xa5u8;5]; check_policy_parts(&parts,&mut output,false,false,Status::Err(E::AdlerMismatch),Ghost(raw));
    proof {
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 0, 0, 0, 1, 0],5,false,true).status == m::InflateStatus::Error(m::InflateFault::AdlerMismatch)) by (compute);
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 0, 0, 0, 1, 0],5,false,true).out == seq![0u8,65,0,66]) by (compute);
    }
    let mut output = [0xa5u8;5]; check_policy_parts(&parts,&mut output,false,true,Status::Err(E::AdlerMismatch),Ghost(raw));
}
} // verus!
