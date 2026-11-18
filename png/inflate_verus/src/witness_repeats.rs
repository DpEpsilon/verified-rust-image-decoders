//! Authored frozen repeat/admission rejections; no decode oracle.
use crate::{
    model as m,
    witness_dynamic::*,
    witnesses::{check_at, follow_word, wire_word},
    InflateError as E, Status,
};
use vstd::prelude::*;
verus! {
/// deflate/repeat-with-no-previous: authored CL/repeat rejection, R=4.
#[verifier::spinoff_prover]
fn repeat_with_no_previous() {
    hide(m::MDynamicLengths); hide(m::MDec); hide(m::decode_prefix);
    let wire = [0x78u8, 0x1, 0xed, 0xe1, 0x49, 0x92, 0x24, 0x49, 0x92, 0x6d, 0xdb, 0x16, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1];
    proof {
        let z = seq![120u8, 1, 237, 225, 73, 146, 36, 73, 146, 109, 219, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        assert(crate::limits::NCL == 19) by (compute); assert(crate::limits::NLIT_MAX == 286) by (compute); assert(crate::limits::NDIST_MAX == 30) by (compute); assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute);
        let code = seq![4u8, 5, 5, 5, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 4, 4, 4];
        assert(m::Kraft(seq![4u8, 5, 5, 5, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 4, 4, 4]) == 32768) by (compute); assert(m::admissible(code,m::CodeKind::CodeLen,true));
        assert(cl_trace(seq![120u8, 1, 237, 225, 73, 146, 36, 73, 146, 109, 219, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],33,Seq::new(19,|i:int|0u8),0) == seq![4u8, 5, 5, 5, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 4, 4, 4]) by (compute);
        assert(wire_word(seq![120u8, 1, 237, 225, 73, 146, 36, 73, 146, 109, 219, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],90,4) == m::cw(seq![4u8, 5, 5, 5, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 4, 4, 4],16).unwrap()) by (compute);
        follow_word(code,z,90,16,0);
        assert(m::MDec(code,z,90) == (m::Decoded::Symbol{symbol:16,position:94})) by { reveal(m::MDec); }
        assert(m::MDynamicLengths(z,90,code,Seq::empty(),286,2,true) == (m::DynamicHeader::Error{error:m::InflateFault::InvalidCodeLengths,position:94})) by { reveal(m::MDynamicLengths); }
        cl_bridge(z,33,Seq::new(19,|i:int|0u8),0,286,2);
        assert(m::MDynamicCL(z,33,Seq::new(19,|i:int|0u8),0,19,286,2,true) == (m::DynamicHeader::Error{error:m::InflateFault::InvalidCodeLengths,position:94}));
        assert(m::field_value(seq![120u8, 1, 237, 225, 73, 146, 36, 73, 146, 109, 219, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],19,5) == 29) by (compute); assert(m::field_value(seq![120u8, 1, 237, 225, 73, 146, 36, 73, 146, 109, 219, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],24,5) == 1) by (compute); assert(m::field_value(seq![120u8, 1, 237, 225, 73, 146, 36, 73, 146, 109, 219, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],29,4) == 15) by (compute);
        assert(m::MDynamic(z,19,true) == (m::DynamicHeader::Error{error:m::InflateFault::InvalidCodeLengths,position:94}));
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 1, 237, 225, 73, 146, 36, 73, 146, 109, 219, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 1, 237, 225, 73, 146, 36, 73, 146, 109, 219, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],17,2) == 2) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),4,32768,true) == (m::StoredEffect::Error{error:m::InflateFault::InvalidCodeLengths,out:Seq::empty(),position:94}));
        assert(m::Inflate(z,4,true,false).status == m::InflateStatus::Error(m::InflateFault::InvalidCodeLengths)); assert(m::Inflate(z,4,true,false).out == Seq::<u8>::empty());
        assert(crate::limits::SEGMENT_BYTES_MAX >= 56) by (compute);
    }
    let mut output = [0xa5u8;4]; check_at(&wire,&mut output,Status::Err(E::InvalidCodeLengths),Ghost(Seq::<u8>::empty()));
}
/// deflate/repeat-overflows-count: authored CL/repeat rejection, R=4.
#[verifier::spinoff_prover]
fn repeat_overflows_count() {
    hide(m::MDynamicLengths); hide(m::MDec); hide(m::decode_prefix);
    let wire = [0x78u8, 0x1, 0xed, 0xe1, 0x49, 0x92, 0x24, 0x49, 0x92, 0x6d, 0xdb, 0xce, 0x7f, 0xfe, 0xf3, 0x7, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1];
    proof {
        let z = seq![120u8, 1, 237, 225, 73, 146, 36, 73, 146, 109, 219, 206, 127, 254, 243, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        assert(crate::limits::NCL == 19) by (compute); assert(crate::limits::NLIT_MAX == 286) by (compute); assert(crate::limits::NDIST_MAX == 30) by (compute); assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute);
        let code = seq![4u8, 5, 5, 5, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 4, 4, 4];
        assert(m::Kraft(seq![4u8, 5, 5, 5, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 4, 4, 4]) == 32768) by (compute); assert(m::admissible(code,m::CodeKind::CodeLen,true));
        assert(cl_trace(seq![120u8, 1, 237, 225, 73, 146, 36, 73, 146, 109, 219, 206, 127, 254, 243, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],33,Seq::new(19,|i:int|0u8),0) == seq![4u8, 5, 5, 5, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 4, 4, 4]) by (compute);
        let values = Seq::new(276,|i:int|0u8);
        assert(repeat_trace(seq![120u8, 1, 237, 225, 73, 146, 36, 73, 146, 109, 219, 206, 127, 254, 243, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],seq![4u8, 5, 5, 5, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 4, 4, 4],seq![18u8,18],seq![90nat,101,112],seq![0nat,138,276],Seq::new(276,|i:int|0u8),0,2)) by (compute);
        expand_repeats(z,code,seq![18u8,18],seq![90nat,101,112],seq![0nat,138,276],values,286,2,0);
        assert(values.take(0) =~= Seq::<u8>::empty());
        assert(wire_word(seq![120u8, 1, 237, 225, 73, 146, 36, 73, 146, 109, 219, 206, 127, 254, 243, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],112,4) == m::cw(seq![4u8, 5, 5, 5, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 4, 4, 4],18).unwrap()) by (compute);
        follow_word(code,z,112,18,0);
        assert(m::MDec(code,z,112) == (m::Decoded::Symbol{symbol:18,position:116})) by { reveal(m::MDec); }
        assert(m::field_value(seq![120u8, 1, 237, 225, 73, 146, 36, 73, 146, 109, 219, 206, 127, 254, 243, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],116,7) == 127) by (compute);
        assert(m::MDynamicLengths(z,112,code,values,286,2,true) == (m::DynamicHeader::Error{error:m::InflateFault::InvalidCodeLengths,position:123})) by { reveal(m::MDynamicLengths); }
        cl_bridge(z,33,Seq::new(19,|i:int|0u8),0,286,2);
        assert(m::MDynamicCL(z,33,Seq::new(19,|i:int|0u8),0,19,286,2,true) == (m::DynamicHeader::Error{error:m::InflateFault::InvalidCodeLengths,position:123}));
        assert(m::field_value(seq![120u8, 1, 237, 225, 73, 146, 36, 73, 146, 109, 219, 206, 127, 254, 243, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],19,5) == 29) by (compute); assert(m::field_value(seq![120u8, 1, 237, 225, 73, 146, 36, 73, 146, 109, 219, 206, 127, 254, 243, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],24,5) == 1) by (compute); assert(m::field_value(seq![120u8, 1, 237, 225, 73, 146, 36, 73, 146, 109, 219, 206, 127, 254, 243, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],29,4) == 15) by (compute);
        assert(m::MDynamic(z,19,true) == (m::DynamicHeader::Error{error:m::InflateFault::InvalidCodeLengths,position:123}));
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 1, 237, 225, 73, 146, 36, 73, 146, 109, 219, 206, 127, 254, 243, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 1, 237, 225, 73, 146, 36, 73, 146, 109, 219, 206, 127, 254, 243, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],17,2) == 2) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),4,32768,true) == (m::StoredEffect::Error{error:m::InflateFault::InvalidCodeLengths,out:Seq::empty(),position:123}));
        assert(m::Inflate(z,4,true,false).status == m::InflateStatus::Error(m::InflateFault::InvalidCodeLengths)); assert(m::Inflate(z,4,true,false).out == Seq::<u8>::empty());
        assert(crate::limits::SEGMENT_BYTES_MAX >= 60) by (compute);
    }
    let mut output = [0xa5u8;4]; check_at(&wire,&mut output,Status::Err(E::InvalidCodeLengths),Ghost(Seq::<u8>::empty()));
}
/// deflate/code-length-code-oversubscribed: authored CL/repeat rejection, R=4.
#[verifier::spinoff_prover]
fn code_length_code_oversubscribed() {
    hide(m::MDynamicLengths); hide(m::MDec); hide(m::decode_prefix);
    let wire = [0x78u8, 0x1, 0xed, 0xe1, 0xb7, 0x6d, 0xdb, 0xb6, 0x6d, 0xdb, 0xb6, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1];
    proof {
        let z = seq![120u8, 1, 237, 225, 183, 109, 219, 182, 109, 219, 182, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        assert(crate::limits::NCL == 19) by (compute); assert(crate::limits::NLIT_MAX == 286) by (compute); assert(crate::limits::NDIST_MAX == 30) by (compute); assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute);
        let code = Seq::new(19,|i:int|3u8);
        assert(cl_trace(seq![120u8, 1, 237, 225, 183, 109, 219, 182, 109, 219, 182, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],33,Seq::new(19,|i:int|0u8),0) == Seq::new(19,|i:int|3u8)) by (compute);
        assert(m::Kraft(Seq::new(19,|i:int|3u8)) == 77824) by (compute);
        assert(!m::admissible(code,m::CodeKind::CodeLen,true));
        cl_fields(z,33,Seq::new(19,|i:int|0u8),0,286,2);
        assert(m::MDynamicCL(z,90,code,19,19,286,2,true) == (m::DynamicHeader::Error{error:m::InflateFault::InvalidCodeLengths,position:90}));
        assert(m::field_value(seq![120u8, 1, 237, 225, 183, 109, 219, 182, 109, 219, 182, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],19,5) == 29) by (compute); assert(m::field_value(seq![120u8, 1, 237, 225, 183, 109, 219, 182, 109, 219, 182, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],24,5) == 1) by (compute); assert(m::field_value(seq![120u8, 1, 237, 225, 183, 109, 219, 182, 109, 219, 182, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],29,4) == 15) by (compute);
        assert(m::MDynamic(z,19,true) == (m::DynamicHeader::Error{error:m::InflateFault::InvalidCodeLengths,position:90}));
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 1, 237, 225, 183, 109, 219, 182, 109, 219, 182, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 1, 237, 225, 183, 109, 219, 182, 109, 219, 182, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],17,2) == 2) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),4,32768,true) == (m::StoredEffect::Error{error:m::InflateFault::InvalidCodeLengths,out:Seq::empty(),position:90}));
        assert(m::Inflate(z,4,true,false).status == m::InflateStatus::Error(m::InflateFault::InvalidCodeLengths)); assert(m::Inflate(z,4,true,false).out == Seq::<u8>::empty());
        assert(crate::limits::SEGMENT_BYTES_MAX >= 56) by (compute);
    }
    let mut output = [0xa5u8;4]; check_at(&wire,&mut output,Status::Err(E::InvalidCodeLengths),Ghost(Seq::<u8>::empty()));
}
/// deflate/code-length-code-incomplete: authored CL/repeat rejection, R=4.
#[verifier::spinoff_prover]
fn code_length_code_incomplete() {
    hide(m::MDynamicLengths); hide(m::MDec); hide(m::decode_prefix);
    let wire = [0x78u8, 0x1, 0xed, 0xe1, 0xdb, 0xb6, 0x6d, 0xdb, 0xb6, 0x6d, 0xdb, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1];
    proof {
        let z = seq![120u8, 1, 237, 225, 219, 182, 109, 219, 182, 109, 219, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        assert(crate::limits::NCL == 19) by (compute); assert(crate::limits::NLIT_MAX == 286) by (compute); assert(crate::limits::NDIST_MAX == 30) by (compute); assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute);
        let code = Seq::new(19,|i:int|5u8);
        assert(cl_trace(seq![120u8, 1, 237, 225, 219, 182, 109, 219, 182, 109, 219, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],33,Seq::new(19,|i:int|0u8),0) == Seq::new(19,|i:int|5u8)) by (compute);
        assert(m::Kraft(Seq::new(19,|i:int|5u8)) == 19456) by (compute);
        assert(!m::admissible(code,m::CodeKind::CodeLen,true));
        cl_fields(z,33,Seq::new(19,|i:int|0u8),0,286,2);
        assert(m::MDynamicCL(z,90,code,19,19,286,2,true) == (m::DynamicHeader::Error{error:m::InflateFault::InvalidCodeLengths,position:90}));
        assert(m::field_value(seq![120u8, 1, 237, 225, 219, 182, 109, 219, 182, 109, 219, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],19,5) == 29) by (compute); assert(m::field_value(seq![120u8, 1, 237, 225, 219, 182, 109, 219, 182, 109, 219, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],24,5) == 1) by (compute); assert(m::field_value(seq![120u8, 1, 237, 225, 219, 182, 109, 219, 182, 109, 219, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],29,4) == 15) by (compute);
        assert(m::MDynamic(z,19,true) == (m::DynamicHeader::Error{error:m::InflateFault::InvalidCodeLengths,position:90}));
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 1, 237, 225, 219, 182, 109, 219, 182, 109, 219, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 1, 237, 225, 219, 182, 109, 219, 182, 109, 219, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],17,2) == 2) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),4,32768,true) == (m::StoredEffect::Error{error:m::InflateFault::InvalidCodeLengths,out:Seq::empty(),position:90}));
        assert(m::Inflate(z,4,true,false).status == m::InflateStatus::Error(m::InflateFault::InvalidCodeLengths)); assert(m::Inflate(z,4,true,false).out == Seq::<u8>::empty());
        assert(crate::limits::SEGMENT_BYTES_MAX >= 56) by (compute);
    }
    let mut output = [0xa5u8;4]; check_at(&wire,&mut output,Status::Err(E::InvalidCodeLengths),Ghost(Seq::<u8>::empty()));
}
} // verus!
