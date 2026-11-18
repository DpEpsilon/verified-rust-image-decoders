//! Explicit small error gates, with original IDAT/fdAT payload boundaries.
use crate::{model as m, witness_policy::check_policy_parts, witnesses::{fixed_decode,fixed_dist_decode,wire_word}, Status, InflateError as E};
use vstd::prelude::*;
verus! {
proof fn before_output(z: Seq<u8>, limit: nat, enforce: bool, tail: bool)
    requires z == seq![120u8,1,3,2,0,0,0,1],
    ensures m::Inflate(z,limit,enforce,tail).status == m::InflateStatus::Error(m::InflateFault::DistanceTooFar),
        m::Inflate(z,limit,enforce,tail).out == Seq::<u8>::empty(),
{
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::MCompressedStep); hide(m::MCompressed);
    assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
    assert(wire_word(seq![120u8,1,3,2,0,0,0,1],19,7) == m::msb_bits(1,7)) by (compute); fixed_decode(z,19,257);
    assert(wire_word(seq![120u8,1,3,2,0,0,0,1],26,5) == m::msb_bits(0,5)) by (compute); fixed_dist_decode(z,26,0);
    assert(m::MCopy(Seq::<u8>::empty(),limit,32768,1,3) == (m::OutputEffect::Error {error:m::InflateFault::DistanceTooFar,out:Seq::<u8>::empty()}));
    assert(m::MCompressedStep(z,19,Seq::empty(),limit,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::SymbolEffect::Error {error:m::InflateFault::DistanceTooFar,out:Seq::<u8>::empty(),position:31})) by {reveal(m::MCompressedStep);}
    assert(m::MCompressed(z,19,Seq::empty(),limit,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::Error {error:m::InflateFault::DistanceTooFar,out:Seq::<u8>::empty(),position:31})) by {reveal_with_fuel(m::MCompressed,2);}
    assert(m::field_value(seq![120u8,1,3,2,0,0,0,1],16,1) == 1) by (compute); assert(m::field_value(seq![120u8,1,3,2,0,0,0,1],17,2) == 1) by (compute);
    assert(m::MBlk(z,16,Seq::empty(),limit,32768,true) == (m::StoredEffect::Error {error:m::InflateFault::DistanceTooFar,out:Seq::<u8>::empty(),position:31}));
}
#[verifier::spinoff_prover]
fn empty_fdat() {
    let segment_0: [u8;0] = [];
    let parts: [&[u8];1] = [&segment_0];
    proof {
        reveal_with_fuel(crate::segments::parts_prefix,2);
        assert(crate::segments::parts_prefix(parts@,1) =~= Seq::<u8>::empty());
        assert(crate::limits::SEGMENT_BYTES_MAX >= 0) by (compute);
    }
    let ghost raw = Seq::<u8>::empty();
    proof {
        assert(m::Inflate(Seq::<u8>::empty(),5,true,false).status == m::InflateStatus::Error(m::InflateFault::TruncatedZlib)) by (compute);
        assert(m::Inflate(Seq::<u8>::empty(),5,true,false).out == Seq::<u8>::empty()) by (compute);
    }
    let mut output = [0xa5u8;5]; check_policy_parts(&parts,&mut output,true,false,Status::Err(E::TruncatedZlib),Ghost(raw));
}
#[verifier::spinoff_prover]
fn core_tail_later_idat() {
    let segment_0: [u8;15] = [0x78u8, 0x1, 0x1, 0x4, 0x0, 0xfb, 0xff, 0x0, 0x41, 0x0, 0x42, 0x1, 0x9, 0x0, 0x84];
    let segment_1: [u8;1] = [0x0u8];
    let parts: [&[u8];2] = [&segment_0, &segment_1];
    proof {
        reveal_with_fuel(crate::segments::parts_prefix,3);
        assert(crate::segments::parts_prefix(parts@,2) =~= seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 0]);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 16) by (compute);
    }
    let ghost raw = seq![0u8, 65, 0, 66];
    proof {
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 0],4,true,false).status == m::InflateStatus::Error(m::InflateFault::TrailingZlibData)) by (compute);
        assert(m::Inflate(seq![120u8, 1, 1, 4, 0, 251, 255, 0, 65, 0, 66, 1, 9, 0, 132, 0],4,true,false).out == seq![0u8, 65, 0, 66]) by (compute);
    }
    let mut output = [0xa5u8;4]; check_policy_parts(&parts,&mut output,true,false,Status::Err(E::TrailingZlibData),Ghost(raw));
}
#[verifier::spinoff_prover]
fn policy_distance_before() {
    let segment_0: [u8;8] = [0x78u8, 0x1, 0x3, 0x2, 0x0, 0x0, 0x0, 0x1];
    let parts: [&[u8];1] = [&segment_0];
    proof {
        reveal_with_fuel(crate::segments::parts_prefix,2);
        assert(crate::segments::parts_prefix(parts@,1) =~= seq![120u8, 1, 3, 2, 0, 0, 0, 1]);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 8) by (compute);
    }
    let ghost raw = Seq::<u8>::empty();
    proof {before_output(seq![120u8, 1, 3, 2, 0, 0, 0, 1],4,true,false);}
    let mut output = [0xa5u8;4]; check_policy_parts(&parts,&mut output,true,false,Status::Err(E::DistanceTooFar),Ghost(raw));
    proof {before_output(seq![120u8, 1, 3, 2, 0, 0, 0, 1],4,true,true);}
    let mut output = [0xa5u8;4]; check_policy_parts(&parts,&mut output,true,true,Status::Err(E::DistanceTooFar),Ghost(raw));
    proof {before_output(seq![120u8, 1, 3, 2, 0, 0, 0, 1],4,false,false);}
    let mut output = [0xa5u8;4]; check_policy_parts(&parts,&mut output,false,false,Status::Err(E::DistanceTooFar),Ghost(raw));
    proof {before_output(seq![120u8, 1, 3, 2, 0, 0, 0, 1],4,false,true);}
    let mut output = [0xa5u8;4]; check_policy_parts(&parts,&mut output,false,true,Status::Err(E::DistanceTooFar),Ghost(raw));
    proof {before_output(seq![120u8, 1, 3, 2, 0, 0, 0, 1],5,true,false);}
    let mut output = [0xa5u8;5]; check_policy_parts(&parts,&mut output,true,false,Status::Err(E::DistanceTooFar),Ghost(raw));
    proof {before_output(seq![120u8, 1, 3, 2, 0, 0, 0, 1],5,true,true);}
    let mut output = [0xa5u8;5]; check_policy_parts(&parts,&mut output,true,true,Status::Err(E::DistanceTooFar),Ghost(raw));
    proof {before_output(seq![120u8, 1, 3, 2, 0, 0, 0, 1],5,false,false);}
    let mut output = [0xa5u8;5]; check_policy_parts(&parts,&mut output,false,false,Status::Err(E::DistanceTooFar),Ghost(raw));
    proof {before_output(seq![120u8, 1, 3, 2, 0, 0, 0, 1],5,false,true);}
    let mut output = [0xa5u8;5]; check_policy_parts(&parts,&mut output,false,true,Status::Err(E::DistanceTooFar),Ghost(raw));
}
} // verus!
