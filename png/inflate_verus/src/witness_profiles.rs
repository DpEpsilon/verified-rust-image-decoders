//! Authored dynamic profiles, repeat fields and sequence-copy effects.
use crate::{model as m, witness_dynamic as d, witness_profile_bridge as b, witnesses::{follow_word,wire_word}, witness_match as t, witness_policy::check_policy_parts, Status};
use vstd::prelude::*;
verus! {
spec fn rounding_z() -> Seq<u8> { seq![120u8, 156, 13, 198, 177, 1, 0, 0, 8, 194, 176, 114, 57, 253, 28, 205, 20, 128, 98, 26, 59, 177, 186, 207, 118, 95, 223, 10, 249] }
spec fn rounding_raw() -> Seq<u8> { seq![0u8, 0, 0, 0, 127, 0, 128, 1, 127, 1, 128, 127, 255, 128, 0, 128, 127, 128, 128, 255, 127, 255, 128, 255, 255] }
spec fn rounding_lit() -> Seq<u8> { Seq::new(258,|i:int| if i == 0 {2u8} else if i == 1 {3u8} else if i == 127 {3u8} else if i == 128 {2u8} else if i == 255 {3u8} else if i == 256 {4u8} else if i == 257 {4u8} else {0u8}) }
spec fn rounding_dist() -> Seq<u8> { Seq::new(7,|i:int| if i == 0 {1u8} else if i == 6 {1u8} else {0u8}) }
spec fn rounding_cl() -> Seq<u8> { seq![0u8, 3, 3, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3] }
fn rounding_part_0() -> (wire: [u8;29])
    ensures wire@ == seq![120u8, 156, 13, 198, 177, 1, 0, 0, 8, 194, 176, 114, 57, 253, 28, 205, 20, 128, 98, 26, 59, 177, 186, 207, 118, 95, 223, 10, 249],
{
    [0x78u8, 0x9c, 0xd, 0xc6, 0xb1, 0x1, 0x0, 0x0, 0x8, 0xc2, 0xb0, 0x72, 0x39, 0xfd, 0x1c, 0xcd, 0x14, 0x80, 0x62, 0x1a, 0x3b, 0xb1, 0xba, 0xcf, 0x76, 0x5f, 0xdf, 0xa, 0xf9]
}
#[verifier::spinoff_prover]
proof fn rounding_header()
    ensures m::MDynamic(rounding_z(),19,true) == (m::DynamicHeader::Tables {lit:rounding_lit(),dist:rounding_dist(),position:135}),
{
    hide(m::MDec); hide(m::decode_prefix); hide(m::MDynamicLengths);
    let z = rounding_z(); let lit = rounding_lit(); let dist = rounding_dist(); let code = rounding_cl();
    assert(crate::limits::NCL == 19) by (compute); assert(crate::limits::NLIT_MAX == 286) by (compute); assert(crate::limits::NDIST_MAX == 30) by (compute); assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute);
    assert(m::Kraft(rounding_lit()) == 32768) by (compute);
    assert(m::Kraft(rounding_dist()) == 32768) by (compute);
    assert(m::Kraft(rounding_cl()) == 32768) by (compute);
    assert(m::admissible(lit,m::CodeKind::LitLen,true)); assert(m::admissible(dist,m::CodeKind::Dist,true)); assert(m::admissible(code,m::CodeKind::CodeLen,true));
    let values = lit+dist;
    assert(values =~= seq![2u8, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 4, 4, 1, 0, 0, 0, 0, 0, 1]);
    assert(d::repeat_trace(seq![120u8, 156, 13, 198, 177, 1, 0, 0, 8, 194, 176, 114, 57, 253, 28, 205, 20, 128, 98, 26, 59, 177, 186, 207, 118, 95, 223, 10, 249],seq![0u8, 3, 3, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3],seq![2u8, 3, 18, 3, 2, 18, 3, 4, 4, 1, 17, 1],seq![87nat, 90, 92, 102, 104, 107, 117, 119, 121, 123, 126, 132, 135],seq![0nat, 1, 2, 127, 128, 129, 255, 256, 257, 258, 259, 264, 265],seq![2u8, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 4, 4, 1, 0, 0, 0, 0, 0, 1],0,12)) by (compute);
    d::expand_repeats(z,code,seq![2u8, 3, 18, 3, 2, 18, 3, 4, 4, 1, 17, 1],seq![87nat, 90, 92, 102, 104, 107, 117, 119, 121, 123, 126, 132, 135],seq![0nat, 1, 2, 127, 128, 129, 255, 256, 257, 258, 259, 264, 265],values,258,7,0);
    assert(values.take(0) =~= Seq::<u8>::empty()); assert(values.take(258) =~= lit); assert(values.skip(258) =~= dist);
    assert(m::MDynamicLengths(z,135,code,values,258,7,true) == (m::DynamicHeader::Tables {lit,dist,position:135})) by { reveal(m::MDynamicLengths); }
    assert(b::fields(seq![120u8, 156, 13, 198, 177, 1, 0, 0, 8, 194, 176, 114, 57, 253, 28, 205, 20, 128, 98, 26, 59, 177, 186, 207, 118, 95, 223, 10, 249],33,Seq::new(19,|i:int|0u8),0,18) == seq![0u8, 3, 3, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3]) by (compute);
    b::bridge(z,33,Seq::new(19,|i:int|0u8),0,18,258,7);
    assert(m::field_value(seq![120u8, 156, 13, 198, 177, 1, 0, 0, 8, 194, 176, 114, 57, 253, 28, 205, 20, 128, 98, 26, 59, 177, 186, 207, 118, 95, 223, 10, 249],19,5) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 13, 198, 177, 1, 0, 0, 8, 194, 176, 114, 57, 253, 28, 205, 20, 128, 98, 26, 59, 177, 186, 207, 118, 95, 223, 10, 249],24,5) == 6) by (compute); assert(m::field_value(seq![120u8, 156, 13, 198, 177, 1, 0, 0, 8, 194, 176, 114, 57, 253, 28, 205, 20, 128, 98, 26, 59, 177, 186, 207, 118, 95, 223, 10, 249],29,4) == 14) by (compute);
}
#[verifier::spinoff_prover]
proof fn rounding_profiles()
    ensures m::lengths_ok(rounding_lit()), m::lengths_ok(rounding_dist()),
        m::Kraft(rounding_lit()) == 32768, m::Kraft(rounding_dist()) == 32768,
{

    let z = rounding_z(); let raw = rounding_raw(); let lit = rounding_lit(); let dist = rounding_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    assert(m::Kraft(rounding_lit()) == 32768) by (compute); assert(m::Kraft(rounding_dist()) == 32768) by (compute);
}
#[verifier::spinoff_prover]
proof fn rounding_step_0()
    ensures m::MCompressedStep(rounding_z(),135,rounding_raw().take(0),25,32768,rounding_lit(),rounding_dist()) == (m::SymbolEffect::Next {out:rounding_raw().take(1),position:137}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = rounding_z(); let raw = rounding_raw(); let lit = rounding_lit(); let dist = rounding_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    rounding_profiles();
    assert(rounding_z().len() == 29) by (compute); assert(rounding_raw().len() == 25) by (compute);
    assert(wire_word(seq![120u8, 156, 13, 198, 177, 1, 0, 0, 8, 194, 176, 114, 57, 253, 28, 205, 20, 128, 98, 26, 59, 177, 186, 207, 118, 95, 223, 10, 249],135,2) == m::cw(rounding_lit(),0).unwrap()) by (compute);
    follow_word(lit,z,135,0,0);
    assert(m::MDec(lit,z,135) == (m::Decoded::Symbol {symbol:0,position:137})) by {reveal(m::MDec);}
    assert(raw.take(0).push(0u8) =~= raw.take(1));
    assert(m::MCompressedStep(z,135,raw.take(0),25,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(1),position:137})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn rounding_step_1()
    ensures m::MCompressedStep(rounding_z(),137,rounding_raw().take(1),25,32768,rounding_lit(),rounding_dist()) == (m::SymbolEffect::Next {out:rounding_raw().take(2),position:139}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = rounding_z(); let raw = rounding_raw(); let lit = rounding_lit(); let dist = rounding_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    rounding_profiles();
    assert(rounding_z().len() == 29) by (compute); assert(rounding_raw().len() == 25) by (compute);
    assert(wire_word(seq![120u8, 156, 13, 198, 177, 1, 0, 0, 8, 194, 176, 114, 57, 253, 28, 205, 20, 128, 98, 26, 59, 177, 186, 207, 118, 95, 223, 10, 249],137,2) == m::cw(rounding_lit(),0).unwrap()) by (compute);
    follow_word(lit,z,137,0,0);
    assert(m::MDec(lit,z,137) == (m::Decoded::Symbol {symbol:0,position:139})) by {reveal(m::MDec);}
    assert(raw.take(1).push(0u8) =~= raw.take(2));
    assert(m::MCompressedStep(z,137,raw.take(1),25,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(2),position:139})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn rounding_step_2()
    ensures m::MCompressedStep(rounding_z(),139,rounding_raw().take(2),25,32768,rounding_lit(),rounding_dist()) == (m::SymbolEffect::Next {out:rounding_raw().take(3),position:141}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = rounding_z(); let raw = rounding_raw(); let lit = rounding_lit(); let dist = rounding_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    rounding_profiles();
    assert(rounding_z().len() == 29) by (compute); assert(rounding_raw().len() == 25) by (compute);
    assert(wire_word(seq![120u8, 156, 13, 198, 177, 1, 0, 0, 8, 194, 176, 114, 57, 253, 28, 205, 20, 128, 98, 26, 59, 177, 186, 207, 118, 95, 223, 10, 249],139,2) == m::cw(rounding_lit(),0).unwrap()) by (compute);
    follow_word(lit,z,139,0,0);
    assert(m::MDec(lit,z,139) == (m::Decoded::Symbol {symbol:0,position:141})) by {reveal(m::MDec);}
    assert(raw.take(2).push(0u8) =~= raw.take(3));
    assert(m::MCompressedStep(z,139,raw.take(2),25,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(3),position:141})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn rounding_step_3()
    ensures m::MCompressedStep(rounding_z(),141,rounding_raw().take(3),25,32768,rounding_lit(),rounding_dist()) == (m::SymbolEffect::Next {out:rounding_raw().take(4),position:143}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = rounding_z(); let raw = rounding_raw(); let lit = rounding_lit(); let dist = rounding_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    rounding_profiles();
    assert(rounding_z().len() == 29) by (compute); assert(rounding_raw().len() == 25) by (compute);
    assert(wire_word(seq![120u8, 156, 13, 198, 177, 1, 0, 0, 8, 194, 176, 114, 57, 253, 28, 205, 20, 128, 98, 26, 59, 177, 186, 207, 118, 95, 223, 10, 249],141,2) == m::cw(rounding_lit(),0).unwrap()) by (compute);
    follow_word(lit,z,141,0,0);
    assert(m::MDec(lit,z,141) == (m::Decoded::Symbol {symbol:0,position:143})) by {reveal(m::MDec);}
    assert(raw.take(3).push(0u8) =~= raw.take(4));
    assert(m::MCompressedStep(z,141,raw.take(3),25,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(4),position:143})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn rounding_step_4()
    ensures m::MCompressedStep(rounding_z(),143,rounding_raw().take(4),25,32768,rounding_lit(),rounding_dist()) == (m::SymbolEffect::Next {out:rounding_raw().take(5),position:146}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = rounding_z(); let raw = rounding_raw(); let lit = rounding_lit(); let dist = rounding_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    rounding_profiles();
    assert(rounding_z().len() == 29) by (compute); assert(rounding_raw().len() == 25) by (compute);
    assert(wire_word(seq![120u8, 156, 13, 198, 177, 1, 0, 0, 8, 194, 176, 114, 57, 253, 28, 205, 20, 128, 98, 26, 59, 177, 186, 207, 118, 95, 223, 10, 249],143,3) == m::cw(rounding_lit(),127).unwrap()) by (compute);
    follow_word(lit,z,143,127,0);
    assert(m::MDec(lit,z,143) == (m::Decoded::Symbol {symbol:127,position:146})) by {reveal(m::MDec);}
    assert(raw.take(4).push(127u8) =~= raw.take(5));
    assert(m::MCompressedStep(z,143,raw.take(4),25,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(5),position:146})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn rounding_step_5()
    ensures m::MCompressedStep(rounding_z(),146,rounding_raw().take(5),25,32768,rounding_lit(),rounding_dist()) == (m::SymbolEffect::Next {out:rounding_raw().take(6),position:148}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = rounding_z(); let raw = rounding_raw(); let lit = rounding_lit(); let dist = rounding_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    rounding_profiles();
    assert(rounding_z().len() == 29) by (compute); assert(rounding_raw().len() == 25) by (compute);
    assert(wire_word(seq![120u8, 156, 13, 198, 177, 1, 0, 0, 8, 194, 176, 114, 57, 253, 28, 205, 20, 128, 98, 26, 59, 177, 186, 207, 118, 95, 223, 10, 249],146,2) == m::cw(rounding_lit(),0).unwrap()) by (compute);
    follow_word(lit,z,146,0,0);
    assert(m::MDec(lit,z,146) == (m::Decoded::Symbol {symbol:0,position:148})) by {reveal(m::MDec);}
    assert(raw.take(5).push(0u8) =~= raw.take(6));
    assert(m::MCompressedStep(z,146,raw.take(5),25,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(6),position:148})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn rounding_step_6()
    ensures m::MCompressedStep(rounding_z(),148,rounding_raw().take(6),25,32768,rounding_lit(),rounding_dist()) == (m::SymbolEffect::Next {out:rounding_raw().take(7),position:150}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = rounding_z(); let raw = rounding_raw(); let lit = rounding_lit(); let dist = rounding_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    rounding_profiles();
    assert(rounding_z().len() == 29) by (compute); assert(rounding_raw().len() == 25) by (compute);
    assert(wire_word(seq![120u8, 156, 13, 198, 177, 1, 0, 0, 8, 194, 176, 114, 57, 253, 28, 205, 20, 128, 98, 26, 59, 177, 186, 207, 118, 95, 223, 10, 249],148,2) == m::cw(rounding_lit(),128).unwrap()) by (compute);
    follow_word(lit,z,148,128,0);
    assert(m::MDec(lit,z,148) == (m::Decoded::Symbol {symbol:128,position:150})) by {reveal(m::MDec);}
    assert(raw.take(6).push(128u8) =~= raw.take(7));
    assert(m::MCompressedStep(z,148,raw.take(6),25,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(7),position:150})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn rounding_step_7()
    ensures m::MCompressedStep(rounding_z(),150,rounding_raw().take(7),25,32768,rounding_lit(),rounding_dist()) == (m::SymbolEffect::Next {out:rounding_raw().take(8),position:153}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = rounding_z(); let raw = rounding_raw(); let lit = rounding_lit(); let dist = rounding_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    rounding_profiles();
    assert(rounding_z().len() == 29) by (compute); assert(rounding_raw().len() == 25) by (compute);
    assert(wire_word(seq![120u8, 156, 13, 198, 177, 1, 0, 0, 8, 194, 176, 114, 57, 253, 28, 205, 20, 128, 98, 26, 59, 177, 186, 207, 118, 95, 223, 10, 249],150,3) == m::cw(rounding_lit(),1).unwrap()) by (compute);
    follow_word(lit,z,150,1,0);
    assert(m::MDec(lit,z,150) == (m::Decoded::Symbol {symbol:1,position:153})) by {reveal(m::MDec);}
    assert(raw.take(7).push(1u8) =~= raw.take(8));
    assert(m::MCompressedStep(z,150,raw.take(7),25,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(8),position:153})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn rounding_step_8()
    ensures m::MCompressedStep(rounding_z(),153,rounding_raw().take(8),25,32768,rounding_lit(),rounding_dist()) == (m::SymbolEffect::Next {out:rounding_raw().take(9),position:156}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = rounding_z(); let raw = rounding_raw(); let lit = rounding_lit(); let dist = rounding_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    rounding_profiles();
    assert(rounding_z().len() == 29) by (compute); assert(rounding_raw().len() == 25) by (compute);
    assert(wire_word(seq![120u8, 156, 13, 198, 177, 1, 0, 0, 8, 194, 176, 114, 57, 253, 28, 205, 20, 128, 98, 26, 59, 177, 186, 207, 118, 95, 223, 10, 249],153,3) == m::cw(rounding_lit(),127).unwrap()) by (compute);
    follow_word(lit,z,153,127,0);
    assert(m::MDec(lit,z,153) == (m::Decoded::Symbol {symbol:127,position:156})) by {reveal(m::MDec);}
    assert(raw.take(8).push(127u8) =~= raw.take(9));
    assert(m::MCompressedStep(z,153,raw.take(8),25,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(9),position:156})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn rounding_step_9()
    ensures m::MCompressedStep(rounding_z(),156,rounding_raw().take(9),25,32768,rounding_lit(),rounding_dist()) == (m::SymbolEffect::Next {out:rounding_raw().take(10),position:159}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = rounding_z(); let raw = rounding_raw(); let lit = rounding_lit(); let dist = rounding_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    rounding_profiles();
    assert(rounding_z().len() == 29) by (compute); assert(rounding_raw().len() == 25) by (compute);
    assert(wire_word(seq![120u8, 156, 13, 198, 177, 1, 0, 0, 8, 194, 176, 114, 57, 253, 28, 205, 20, 128, 98, 26, 59, 177, 186, 207, 118, 95, 223, 10, 249],156,3) == m::cw(rounding_lit(),1).unwrap()) by (compute);
    follow_word(lit,z,156,1,0);
    assert(m::MDec(lit,z,156) == (m::Decoded::Symbol {symbol:1,position:159})) by {reveal(m::MDec);}
    assert(raw.take(9).push(1u8) =~= raw.take(10));
    assert(m::MCompressedStep(z,156,raw.take(9),25,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(10),position:159})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn rounding_step_10()
    ensures m::MCompressedStep(rounding_z(),159,rounding_raw().take(10),25,32768,rounding_lit(),rounding_dist()) == (m::SymbolEffect::Next {out:rounding_raw().take(11),position:161}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = rounding_z(); let raw = rounding_raw(); let lit = rounding_lit(); let dist = rounding_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    rounding_profiles();
    assert(rounding_z().len() == 29) by (compute); assert(rounding_raw().len() == 25) by (compute);
    assert(wire_word(seq![120u8, 156, 13, 198, 177, 1, 0, 0, 8, 194, 176, 114, 57, 253, 28, 205, 20, 128, 98, 26, 59, 177, 186, 207, 118, 95, 223, 10, 249],159,2) == m::cw(rounding_lit(),128).unwrap()) by (compute);
    follow_word(lit,z,159,128,0);
    assert(m::MDec(lit,z,159) == (m::Decoded::Symbol {symbol:128,position:161})) by {reveal(m::MDec);}
    assert(raw.take(10).push(128u8) =~= raw.take(11));
    assert(m::MCompressedStep(z,159,raw.take(10),25,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(11),position:161})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn rounding_step_11()
    ensures m::MCompressedStep(rounding_z(),161,rounding_raw().take(11),25,32768,rounding_lit(),rounding_dist()) == (m::SymbolEffect::Next {out:rounding_raw().take(12),position:164}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = rounding_z(); let raw = rounding_raw(); let lit = rounding_lit(); let dist = rounding_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    rounding_profiles();
    assert(rounding_z().len() == 29) by (compute); assert(rounding_raw().len() == 25) by (compute);
    assert(wire_word(seq![120u8, 156, 13, 198, 177, 1, 0, 0, 8, 194, 176, 114, 57, 253, 28, 205, 20, 128, 98, 26, 59, 177, 186, 207, 118, 95, 223, 10, 249],161,3) == m::cw(rounding_lit(),127).unwrap()) by (compute);
    follow_word(lit,z,161,127,0);
    assert(m::MDec(lit,z,161) == (m::Decoded::Symbol {symbol:127,position:164})) by {reveal(m::MDec);}
    assert(raw.take(11).push(127u8) =~= raw.take(12));
    assert(m::MCompressedStep(z,161,raw.take(11),25,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(12),position:164})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn rounding_step_12()
    ensures m::MCompressedStep(rounding_z(),164,rounding_raw().take(12),25,32768,rounding_lit(),rounding_dist()) == (m::SymbolEffect::Next {out:rounding_raw().take(13),position:167}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = rounding_z(); let raw = rounding_raw(); let lit = rounding_lit(); let dist = rounding_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    rounding_profiles();
    assert(rounding_z().len() == 29) by (compute); assert(rounding_raw().len() == 25) by (compute);
    assert(wire_word(seq![120u8, 156, 13, 198, 177, 1, 0, 0, 8, 194, 176, 114, 57, 253, 28, 205, 20, 128, 98, 26, 59, 177, 186, 207, 118, 95, 223, 10, 249],164,3) == m::cw(rounding_lit(),255).unwrap()) by (compute);
    follow_word(lit,z,164,255,0);
    assert(m::MDec(lit,z,164) == (m::Decoded::Symbol {symbol:255,position:167})) by {reveal(m::MDec);}
    assert(raw.take(12).push(255u8) =~= raw.take(13));
    assert(m::MCompressedStep(z,164,raw.take(12),25,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(13),position:167})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn rounding_step_13()
    ensures m::MCompressedStep(rounding_z(),167,rounding_raw().take(13),25,32768,rounding_lit(),rounding_dist()) == (m::SymbolEffect::Next {out:rounding_raw().take(14),position:169}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = rounding_z(); let raw = rounding_raw(); let lit = rounding_lit(); let dist = rounding_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    rounding_profiles();
    assert(rounding_z().len() == 29) by (compute); assert(rounding_raw().len() == 25) by (compute);
    assert(wire_word(seq![120u8, 156, 13, 198, 177, 1, 0, 0, 8, 194, 176, 114, 57, 253, 28, 205, 20, 128, 98, 26, 59, 177, 186, 207, 118, 95, 223, 10, 249],167,2) == m::cw(rounding_lit(),128).unwrap()) by (compute);
    follow_word(lit,z,167,128,0);
    assert(m::MDec(lit,z,167) == (m::Decoded::Symbol {symbol:128,position:169})) by {reveal(m::MDec);}
    assert(raw.take(13).push(128u8) =~= raw.take(14));
    assert(m::MCompressedStep(z,167,raw.take(13),25,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(14),position:169})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn rounding_step_14()
    ensures m::MCompressedStep(rounding_z(),169,rounding_raw().take(14),25,32768,rounding_lit(),rounding_dist()) == (m::SymbolEffect::Next {out:rounding_raw().take(15),position:171}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = rounding_z(); let raw = rounding_raw(); let lit = rounding_lit(); let dist = rounding_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    rounding_profiles();
    assert(rounding_z().len() == 29) by (compute); assert(rounding_raw().len() == 25) by (compute);
    assert(wire_word(seq![120u8, 156, 13, 198, 177, 1, 0, 0, 8, 194, 176, 114, 57, 253, 28, 205, 20, 128, 98, 26, 59, 177, 186, 207, 118, 95, 223, 10, 249],169,2) == m::cw(rounding_lit(),0).unwrap()) by (compute);
    follow_word(lit,z,169,0,0);
    assert(m::MDec(lit,z,169) == (m::Decoded::Symbol {symbol:0,position:171})) by {reveal(m::MDec);}
    assert(raw.take(14).push(0u8) =~= raw.take(15));
    assert(m::MCompressedStep(z,169,raw.take(14),25,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(15),position:171})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn rounding_step_15()
    ensures m::MCompressedStep(rounding_z(),171,rounding_raw().take(15),25,32768,rounding_lit(),rounding_dist()) == (m::SymbolEffect::Next {out:rounding_raw().take(16),position:173}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = rounding_z(); let raw = rounding_raw(); let lit = rounding_lit(); let dist = rounding_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    rounding_profiles();
    assert(rounding_z().len() == 29) by (compute); assert(rounding_raw().len() == 25) by (compute);
    assert(wire_word(seq![120u8, 156, 13, 198, 177, 1, 0, 0, 8, 194, 176, 114, 57, 253, 28, 205, 20, 128, 98, 26, 59, 177, 186, 207, 118, 95, 223, 10, 249],171,2) == m::cw(rounding_lit(),128).unwrap()) by (compute);
    follow_word(lit,z,171,128,0);
    assert(m::MDec(lit,z,171) == (m::Decoded::Symbol {symbol:128,position:173})) by {reveal(m::MDec);}
    assert(raw.take(15).push(128u8) =~= raw.take(16));
    assert(m::MCompressedStep(z,171,raw.take(15),25,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(16),position:173})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn rounding_step_16()
    ensures m::MCompressedStep(rounding_z(),173,rounding_raw().take(16),25,32768,rounding_lit(),rounding_dist()) == (m::SymbolEffect::Next {out:rounding_raw().take(17),position:176}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = rounding_z(); let raw = rounding_raw(); let lit = rounding_lit(); let dist = rounding_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    rounding_profiles();
    assert(rounding_z().len() == 29) by (compute); assert(rounding_raw().len() == 25) by (compute);
    assert(wire_word(seq![120u8, 156, 13, 198, 177, 1, 0, 0, 8, 194, 176, 114, 57, 253, 28, 205, 20, 128, 98, 26, 59, 177, 186, 207, 118, 95, 223, 10, 249],173,3) == m::cw(rounding_lit(),127).unwrap()) by (compute);
    follow_word(lit,z,173,127,0);
    assert(m::MDec(lit,z,173) == (m::Decoded::Symbol {symbol:127,position:176})) by {reveal(m::MDec);}
    assert(raw.take(16).push(127u8) =~= raw.take(17));
    assert(m::MCompressedStep(z,173,raw.take(16),25,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(17),position:176})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn rounding_step_17()
    ensures m::MCompressedStep(rounding_z(),176,rounding_raw().take(17),25,32768,rounding_lit(),rounding_dist()) == (m::SymbolEffect::Next {out:rounding_raw().take(18),position:178}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = rounding_z(); let raw = rounding_raw(); let lit = rounding_lit(); let dist = rounding_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    rounding_profiles();
    assert(rounding_z().len() == 29) by (compute); assert(rounding_raw().len() == 25) by (compute);
    assert(wire_word(seq![120u8, 156, 13, 198, 177, 1, 0, 0, 8, 194, 176, 114, 57, 253, 28, 205, 20, 128, 98, 26, 59, 177, 186, 207, 118, 95, 223, 10, 249],176,2) == m::cw(rounding_lit(),128).unwrap()) by (compute);
    follow_word(lit,z,176,128,0);
    assert(m::MDec(lit,z,176) == (m::Decoded::Symbol {symbol:128,position:178})) by {reveal(m::MDec);}
    assert(raw.take(17).push(128u8) =~= raw.take(18));
    assert(m::MCompressedStep(z,176,raw.take(17),25,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(18),position:178})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn rounding_step_18()
    ensures m::MCompressedStep(rounding_z(),178,rounding_raw().take(18),25,32768,rounding_lit(),rounding_dist()) == (m::SymbolEffect::Next {out:rounding_raw().take(19),position:180}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = rounding_z(); let raw = rounding_raw(); let lit = rounding_lit(); let dist = rounding_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    rounding_profiles();
    assert(rounding_z().len() == 29) by (compute); assert(rounding_raw().len() == 25) by (compute);
    assert(wire_word(seq![120u8, 156, 13, 198, 177, 1, 0, 0, 8, 194, 176, 114, 57, 253, 28, 205, 20, 128, 98, 26, 59, 177, 186, 207, 118, 95, 223, 10, 249],178,2) == m::cw(rounding_lit(),128).unwrap()) by (compute);
    follow_word(lit,z,178,128,0);
    assert(m::MDec(lit,z,178) == (m::Decoded::Symbol {symbol:128,position:180})) by {reveal(m::MDec);}
    assert(raw.take(18).push(128u8) =~= raw.take(19));
    assert(m::MCompressedStep(z,178,raw.take(18),25,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(19),position:180})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn rounding_step_19()
    ensures m::MCompressedStep(rounding_z(),180,rounding_raw().take(19),25,32768,rounding_lit(),rounding_dist()) == (m::SymbolEffect::Next {out:rounding_raw().take(20),position:183}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = rounding_z(); let raw = rounding_raw(); let lit = rounding_lit(); let dist = rounding_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    rounding_profiles();
    assert(rounding_z().len() == 29) by (compute); assert(rounding_raw().len() == 25) by (compute);
    assert(wire_word(seq![120u8, 156, 13, 198, 177, 1, 0, 0, 8, 194, 176, 114, 57, 253, 28, 205, 20, 128, 98, 26, 59, 177, 186, 207, 118, 95, 223, 10, 249],180,3) == m::cw(rounding_lit(),255).unwrap()) by (compute);
    follow_word(lit,z,180,255,0);
    assert(m::MDec(lit,z,180) == (m::Decoded::Symbol {symbol:255,position:183})) by {reveal(m::MDec);}
    assert(raw.take(19).push(255u8) =~= raw.take(20));
    assert(m::MCompressedStep(z,180,raw.take(19),25,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(20),position:183})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn rounding_step_20()
    ensures m::MCompressedStep(rounding_z(),183,rounding_raw().take(20),25,32768,rounding_lit(),rounding_dist()) == (m::SymbolEffect::Next {out:rounding_raw().take(23),position:190}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = rounding_z(); let raw = rounding_raw(); let lit = rounding_lit(); let dist = rounding_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    rounding_profiles();
    assert(rounding_z().len() == 29) by (compute); assert(rounding_raw().len() == 25) by (compute);
    assert(wire_word(seq![120u8, 156, 13, 198, 177, 1, 0, 0, 8, 194, 176, 114, 57, 253, 28, 205, 20, 128, 98, 26, 59, 177, 186, 207, 118, 95, 223, 10, 249],183,4) == m::cw(rounding_lit(),257).unwrap()) by (compute);
    follow_word(lit,z,183,257,0);
    assert(m::MDec(lit,z,183) == (m::Decoded::Symbol {symbol:257,position:187})) by {reveal(m::MDec);}
    assert(m::field_value(seq![120u8, 156, 13, 198, 177, 1, 0, 0, 8, 194, 176, 114, 57, 253, 28, 205, 20, 128, 98, 26, 59, 177, 186, 207, 118, 95, 223, 10, 249],187,0) == 0) by (compute);
    assert(wire_word(seq![120u8, 156, 13, 198, 177, 1, 0, 0, 8, 194, 176, 114, 57, 253, 28, 205, 20, 128, 98, 26, 59, 177, 186, 207, 118, 95, 223, 10, 249],187,1) == m::cw(rounding_dist(),6).unwrap()) by (compute);
    follow_word(dist,z,187,6,0);
    assert(m::MDec(dist,z,187) == (m::Decoded::Symbol {symbol:6,position:188})) by {reveal(m::MDec);}
    assert(m::field_value(seq![120u8, 156, 13, 198, 177, 1, 0, 0, 8, 194, 176, 114, 57, 253, 28, 205, 20, 128, 98, 26, 59, 177, 186, 207, 118, 95, 223, 10, 249],188,2) == 0) by (compute);
    assert(t::copy_bytes(seq![0u8, 0, 0, 0, 127, 0, 128, 1, 127, 1, 128, 127, 255, 128, 0, 128, 127, 128, 128, 255, 127, 255, 128, 255, 255],20,9,0,3)) by (compute);
    t::copy_quant(raw,20,9,0,3); t::copy_prefix(raw,20,9,3);
    assert(m::MCopy(raw.take(20),25,32768,9,3) == (m::OutputEffect::Next {out:raw.take(23)}));
    assert(m::length_extra(257) == 0) by (compute); assert(m::length_base(257) == 3) by (compute);
    assert(m::distance_extra(6) == 2) by (compute); assert(m::distance_base(6) == 9) by (compute);
    assert(crate::limits::NDIST_MAX == 30) by (compute);
    assert(m::MCompressedStep(z,183,raw.take(20),25,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(23),position:190})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn rounding_step_21()
    ensures m::MCompressedStep(rounding_z(),190,rounding_raw().take(23),25,32768,rounding_lit(),rounding_dist()) == (m::SymbolEffect::Next {out:rounding_raw().take(24),position:193}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = rounding_z(); let raw = rounding_raw(); let lit = rounding_lit(); let dist = rounding_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    rounding_profiles();
    assert(rounding_z().len() == 29) by (compute); assert(rounding_raw().len() == 25) by (compute);
    assert(wire_word(seq![120u8, 156, 13, 198, 177, 1, 0, 0, 8, 194, 176, 114, 57, 253, 28, 205, 20, 128, 98, 26, 59, 177, 186, 207, 118, 95, 223, 10, 249],190,3) == m::cw(rounding_lit(),255).unwrap()) by (compute);
    follow_word(lit,z,190,255,0);
    assert(m::MDec(lit,z,190) == (m::Decoded::Symbol {symbol:255,position:193})) by {reveal(m::MDec);}
    assert(raw.take(23).push(255u8) =~= raw.take(24));
    assert(m::MCompressedStep(z,190,raw.take(23),25,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(24),position:193})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn rounding_step_22()
    ensures m::MCompressedStep(rounding_z(),193,rounding_raw().take(24),25,32768,rounding_lit(),rounding_dist()) == (m::SymbolEffect::Next {out:rounding_raw().take(25),position:196}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = rounding_z(); let raw = rounding_raw(); let lit = rounding_lit(); let dist = rounding_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    rounding_profiles();
    assert(rounding_z().len() == 29) by (compute); assert(rounding_raw().len() == 25) by (compute);
    assert(wire_word(seq![120u8, 156, 13, 198, 177, 1, 0, 0, 8, 194, 176, 114, 57, 253, 28, 205, 20, 128, 98, 26, 59, 177, 186, 207, 118, 95, 223, 10, 249],193,3) == m::cw(rounding_lit(),255).unwrap()) by (compute);
    follow_word(lit,z,193,255,0);
    assert(m::MDec(lit,z,193) == (m::Decoded::Symbol {symbol:255,position:196})) by {reveal(m::MDec);}
    assert(raw.take(24).push(255u8) =~= raw.take(25));
    assert(m::MCompressedStep(z,193,raw.take(24),25,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(25),position:196})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn rounding_step_23()
    ensures m::MCompressedStep(rounding_z(),196,rounding_raw().take(25),25,32768,rounding_lit(),rounding_dist()) == (m::SymbolEffect::End {out:rounding_raw(),position:200}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = rounding_z(); let raw = rounding_raw(); let lit = rounding_lit(); let dist = rounding_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    rounding_profiles();
    assert(rounding_z().len() == 29) by (compute); assert(rounding_raw().len() == 25) by (compute);
    assert(wire_word(seq![120u8, 156, 13, 198, 177, 1, 0, 0, 8, 194, 176, 114, 57, 253, 28, 205, 20, 128, 98, 26, 59, 177, 186, 207, 118, 95, 223, 10, 249],196,4) == m::cw(rounding_lit(),256).unwrap()) by (compute);
    follow_word(lit,z,196,256,0);
    assert(m::MDec(lit,z,196) == (m::Decoded::Symbol {symbol:256,position:200})) by {reveal(m::MDec);}
    assert(raw.take(25) =~= raw);
    assert(m::MCompressedStep(z,196,raw.take(25),25,32768,lit,dist) == (m::SymbolEffect::End {out:raw,position:200})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn rounding_payload()
    ensures m::MCompressed(rounding_z(),135,Seq::empty(),25,32768,rounding_lit(),rounding_dist()) == (m::StoredEffect::End {out:rounding_raw(),position:200}),
{
    hide(m::MCompressedStep); hide(m::MCompressed);
    let z = rounding_z(); let raw = rounding_raw(); let lit = rounding_lit(); let dist = rounding_dist();
    rounding_step_0();
    rounding_step_1();
    rounding_step_2();
    rounding_step_3();
    rounding_step_4();
    rounding_step_5();
    rounding_step_6();
    rounding_step_7();
    rounding_step_8();
    rounding_step_9();
    rounding_step_10();
    rounding_step_11();
    rounding_step_12();
    rounding_step_13();
    rounding_step_14();
    rounding_step_15();
    rounding_step_16();
    rounding_step_17();
    rounding_step_18();
    rounding_step_19();
    rounding_step_20();
    rounding_step_21();
    rounding_step_22();
    rounding_step_23();
    assert(raw.take(0) =~= Seq::<u8>::empty());
    assert(m::MCompressed(z,135,Seq::empty(),25,32768,lit,dist) == (m::StoredEffect::End {out:raw,position:200})) by {reveal_with_fuel(m::MCompressed,25);}
}
#[verifier::spinoff_prover]
fn rounding() {
    hide(m::MDec); hide(m::MDynamic); hide(m::MCompressed); hide(m::sum_bytes); hide(m::sum_prefixes);
    let segment_0 = rounding_part_0();
    let parts: [&[u8];1] = [&segment_0];
    proof {
        rounding_header(); rounding_payload();
        let z = rounding_z(); let raw = rounding_raw();
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 156, 13, 198, 177, 1, 0, 0, 8, 194, 176, 114, 57, 253, 28, 205, 20, 128, 98, 26, 59, 177, 186, 207, 118, 95, 223, 10, 249],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 13, 198, 177, 1, 0, 0, 8, 194, 176, 114, 57, 253, 28, 205, 20, 128, 98, 26, 59, 177, 186, 207, 118, 95, 223, 10, 249],17,2) == 2) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),25,32768,true) == (m::StoredEffect::End {out:raw,position:200}));
        assert(m::MFinish(seq![120u8, 156, 13, 198, 177, 1, 0, 0, 8, 194, 176, 114, 57, 253, 28, 205, 20, 128, 98, 26, 59, 177, 186, 207, 118, 95, 223, 10, 249],200,seq![0u8, 0, 0, 0, 127, 0, 128, 1, 127, 1, 128, 127, 255, 128, 0, 128, 127, 128, 128, 255, 127, 255, 128, 255, 255],25,false) == (m::StoredEffect::End {out:seq![0u8, 0, 0, 0, 127, 0, 128, 1, 127, 1, 128, 127, 255, 128, 0, 128, 127, 128, 128, 255, 127, 255, 128, 255, 255],position:232})) by (compute);
        assert(m::Inflate(z,25,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,25,true,false).out == raw);
        reveal_with_fuel(crate::segments::parts_prefix,2); assert(crate::segments::parts_prefix(parts@,1) =~= z);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 29) by (compute);
    }
    let ghost raw = rounding_raw(); let mut output = [0xa5u8;25]; check_policy_parts(&parts,&mut output,true,false,Status::Done,Ghost(raw));
}
spec fn subframe_z() -> Seq<u8> { seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253] }
spec fn subframe_raw() -> Seq<u8> { seq![0u8, 255, 0, 0, 255, 1, 0, 0, 255, 255, 2, 0, 0, 255, 255, 255, 0, 0, 255, 3, 0, 255, 0, 255, 4, 0, 1, 255, 129, 0, 0, 0, 255, 128, 255, 0, 0, 255, 0, 255, 0, 255] }
spec fn subframe_lit() -> Seq<u8> { Seq::new(259,|i:int| if i == 0 {2u8} else if i == 1 {4u8} else if i == 2 {5u8} else if i == 3 {4u8} else if i == 4 {5u8} else if i == 128 {5u8} else if i == 129 {5u8} else if i == 255 {2u8} else if i == 256 {4u8} else if i == 257 {4u8} else if i == 258 {3u8} else {0u8}) }
spec fn subframe_dist() -> Seq<u8> { Seq::new(9,|i:int| if i == 1 {3u8} else if i == 3 {2u8} else if i == 4 {2u8} else if i == 7 {2u8} else if i == 8 {3u8} else {0u8}) }
spec fn subframe_cl() -> Seq<u8> { seq![3u8, 0, 2, 3, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3] }
fn subframe_part_0() -> (wire: [u8;36])
    ensures wire@ == seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],
{
    [0x78u8, 0x9c, 0x15, 0x88, 0x81, 0xd, 0x0, 0x30, 0x8, 0x83, 0x70, 0xee, 0xf0, 0xed, 0x72, 0xb4, 0x69, 0x20, 0xa1, 0x8, 0xd6, 0xe2, 0x89, 0x52, 0x8d, 0x78, 0x29, 0x3f, 0x5b, 0x2f, 0x57, 0x36, 0x5b, 0x79, 0x10, 0xfd]
}
#[verifier::spinoff_prover]
proof fn subframe_header()
    ensures m::MDynamic(subframe_z(),19,true) == (m::DynamicHeader::Tables {lit:subframe_lit(),dist:subframe_dist(),position:152}),
{
    hide(m::MDec); hide(m::decode_prefix); hide(m::MDynamicLengths);
    let z = subframe_z(); let lit = subframe_lit(); let dist = subframe_dist(); let code = subframe_cl();
    assert(crate::limits::NCL == 19) by (compute); assert(crate::limits::NLIT_MAX == 286) by (compute); assert(crate::limits::NDIST_MAX == 30) by (compute); assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute);
    assert(m::Kraft(subframe_lit()) == 32768) by (compute);
    assert(m::Kraft(subframe_dist()) == 32768) by (compute);
    assert(m::Kraft(subframe_cl()) == 32768) by (compute);
    assert(m::admissible(lit,m::CodeKind::LitLen,true)); assert(m::admissible(dist,m::CodeKind::Dist,true)); assert(m::admissible(code,m::CodeKind::CodeLen,true));
    let values = lit+dist;
    assert(values =~= seq![2u8, 4, 5, 4, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 4, 4, 3, 0, 3, 0, 2, 2, 0, 0, 2, 3]);
    assert(d::repeat_trace(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],seq![3u8, 0, 2, 3, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3],seq![2u8, 4, 5, 4, 5, 18, 5, 5, 18, 2, 4, 4, 3, 0, 3, 0, 2, 2, 0, 0, 2, 3],seq![81nat, 83, 85, 88, 90, 93, 103, 106, 109, 119, 121, 123, 125, 128, 131, 134, 137, 139, 141, 144, 147, 149, 152],seq![0nat, 1, 2, 3, 4, 5, 128, 129, 130, 255, 256, 257, 258, 259, 260, 261, 262, 263, 264, 265, 266, 267, 268],seq![2u8, 4, 5, 4, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 4, 4, 3, 0, 3, 0, 2, 2, 0, 0, 2, 3],0,22)) by (compute);
    d::expand_repeats(z,code,seq![2u8, 4, 5, 4, 5, 18, 5, 5, 18, 2, 4, 4, 3, 0, 3, 0, 2, 2, 0, 0, 2, 3],seq![81nat, 83, 85, 88, 90, 93, 103, 106, 109, 119, 121, 123, 125, 128, 131, 134, 137, 139, 141, 144, 147, 149, 152],seq![0nat, 1, 2, 3, 4, 5, 128, 129, 130, 255, 256, 257, 258, 259, 260, 261, 262, 263, 264, 265, 266, 267, 268],values,259,9,0);
    assert(values.take(0) =~= Seq::<u8>::empty()); assert(values.take(259) =~= lit); assert(values.skip(259) =~= dist);
    assert(m::MDynamicLengths(z,152,code,values,259,9,true) == (m::DynamicHeader::Tables {lit,dist,position:152})) by { reveal(m::MDynamicLengths); }
    assert(b::fields(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],33,Seq::new(19,|i:int|0u8),0,16) == seq![3u8, 0, 2, 3, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3]) by (compute);
    b::bridge(z,33,Seq::new(19,|i:int|0u8),0,16,259,9);
    assert(m::field_value(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],19,5) == 2) by (compute); assert(m::field_value(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],24,5) == 8) by (compute); assert(m::field_value(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],29,4) == 12) by (compute);
}
#[verifier::spinoff_prover]
proof fn subframe_profiles()
    ensures m::lengths_ok(subframe_lit()), m::lengths_ok(subframe_dist()),
        m::Kraft(subframe_lit()) == 32768, m::Kraft(subframe_dist()) == 32768,
{

    let z = subframe_z(); let raw = subframe_raw(); let lit = subframe_lit(); let dist = subframe_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    assert(m::Kraft(subframe_lit()) == 32768) by (compute); assert(m::Kraft(subframe_dist()) == 32768) by (compute);
}
#[verifier::spinoff_prover]
proof fn subframe_step_0()
    ensures m::MCompressedStep(subframe_z(),152,subframe_raw().take(0),42,32768,subframe_lit(),subframe_dist()) == (m::SymbolEffect::Next {out:subframe_raw().take(1),position:154}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = subframe_z(); let raw = subframe_raw(); let lit = subframe_lit(); let dist = subframe_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    subframe_profiles();
    assert(subframe_z().len() == 36) by (compute); assert(subframe_raw().len() == 42) by (compute);
    assert(wire_word(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],152,2) == m::cw(subframe_lit(),0).unwrap()) by (compute);
    follow_word(lit,z,152,0,0);
    assert(m::MDec(lit,z,152) == (m::Decoded::Symbol {symbol:0,position:154})) by {reveal(m::MDec);}
    assert(raw.take(0).push(0u8) =~= raw.take(1));
    assert(m::MCompressedStep(z,152,raw.take(0),42,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(1),position:154})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn subframe_step_1()
    ensures m::MCompressedStep(subframe_z(),154,subframe_raw().take(1),42,32768,subframe_lit(),subframe_dist()) == (m::SymbolEffect::Next {out:subframe_raw().take(2),position:156}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = subframe_z(); let raw = subframe_raw(); let lit = subframe_lit(); let dist = subframe_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    subframe_profiles();
    assert(subframe_z().len() == 36) by (compute); assert(subframe_raw().len() == 42) by (compute);
    assert(wire_word(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],154,2) == m::cw(subframe_lit(),255).unwrap()) by (compute);
    follow_word(lit,z,154,255,0);
    assert(m::MDec(lit,z,154) == (m::Decoded::Symbol {symbol:255,position:156})) by {reveal(m::MDec);}
    assert(raw.take(1).push(255u8) =~= raw.take(2));
    assert(m::MCompressedStep(z,154,raw.take(1),42,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(2),position:156})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn subframe_step_2()
    ensures m::MCompressedStep(subframe_z(),156,subframe_raw().take(2),42,32768,subframe_lit(),subframe_dist()) == (m::SymbolEffect::Next {out:subframe_raw().take(3),position:158}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = subframe_z(); let raw = subframe_raw(); let lit = subframe_lit(); let dist = subframe_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    subframe_profiles();
    assert(subframe_z().len() == 36) by (compute); assert(subframe_raw().len() == 42) by (compute);
    assert(wire_word(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],156,2) == m::cw(subframe_lit(),0).unwrap()) by (compute);
    follow_word(lit,z,156,0,0);
    assert(m::MDec(lit,z,156) == (m::Decoded::Symbol {symbol:0,position:158})) by {reveal(m::MDec);}
    assert(raw.take(2).push(0u8) =~= raw.take(3));
    assert(m::MCompressedStep(z,156,raw.take(2),42,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(3),position:158})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn subframe_step_3()
    ensures m::MCompressedStep(subframe_z(),158,subframe_raw().take(3),42,32768,subframe_lit(),subframe_dist()) == (m::SymbolEffect::Next {out:subframe_raw().take(4),position:160}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = subframe_z(); let raw = subframe_raw(); let lit = subframe_lit(); let dist = subframe_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    subframe_profiles();
    assert(subframe_z().len() == 36) by (compute); assert(subframe_raw().len() == 42) by (compute);
    assert(wire_word(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],158,2) == m::cw(subframe_lit(),0).unwrap()) by (compute);
    follow_word(lit,z,158,0,0);
    assert(m::MDec(lit,z,158) == (m::Decoded::Symbol {symbol:0,position:160})) by {reveal(m::MDec);}
    assert(raw.take(3).push(0u8) =~= raw.take(4));
    assert(m::MCompressedStep(z,158,raw.take(3),42,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(4),position:160})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn subframe_step_4()
    ensures m::MCompressedStep(subframe_z(),160,subframe_raw().take(4),42,32768,subframe_lit(),subframe_dist()) == (m::SymbolEffect::Next {out:subframe_raw().take(5),position:162}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = subframe_z(); let raw = subframe_raw(); let lit = subframe_lit(); let dist = subframe_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    subframe_profiles();
    assert(subframe_z().len() == 36) by (compute); assert(subframe_raw().len() == 42) by (compute);
    assert(wire_word(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],160,2) == m::cw(subframe_lit(),255).unwrap()) by (compute);
    follow_word(lit,z,160,255,0);
    assert(m::MDec(lit,z,160) == (m::Decoded::Symbol {symbol:255,position:162})) by {reveal(m::MDec);}
    assert(raw.take(4).push(255u8) =~= raw.take(5));
    assert(m::MCompressedStep(z,160,raw.take(4),42,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(5),position:162})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn subframe_step_5()
    ensures m::MCompressedStep(subframe_z(),162,subframe_raw().take(5),42,32768,subframe_lit(),subframe_dist()) == (m::SymbolEffect::Next {out:subframe_raw().take(6),position:166}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = subframe_z(); let raw = subframe_raw(); let lit = subframe_lit(); let dist = subframe_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    subframe_profiles();
    assert(subframe_z().len() == 36) by (compute); assert(subframe_raw().len() == 42) by (compute);
    assert(wire_word(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],162,4) == m::cw(subframe_lit(),1).unwrap()) by (compute);
    follow_word(lit,z,162,1,0);
    assert(m::MDec(lit,z,162) == (m::Decoded::Symbol {symbol:1,position:166})) by {reveal(m::MDec);}
    assert(raw.take(5).push(1u8) =~= raw.take(6));
    assert(m::MCompressedStep(z,162,raw.take(5),42,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(6),position:166})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn subframe_step_6()
    ensures m::MCompressedStep(subframe_z(),166,subframe_raw().take(6),42,32768,subframe_lit(),subframe_dist()) == (m::SymbolEffect::Next {out:subframe_raw().take(9),position:172}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = subframe_z(); let raw = subframe_raw(); let lit = subframe_lit(); let dist = subframe_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    subframe_profiles();
    assert(subframe_z().len() == 36) by (compute); assert(subframe_raw().len() == 42) by (compute);
    assert(wire_word(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],166,4) == m::cw(subframe_lit(),257).unwrap()) by (compute);
    follow_word(lit,z,166,257,0);
    assert(m::MDec(lit,z,166) == (m::Decoded::Symbol {symbol:257,position:170})) by {reveal(m::MDec);}
    assert(m::field_value(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],170,0) == 0) by (compute);
    assert(wire_word(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],170,2) == m::cw(subframe_dist(),3).unwrap()) by (compute);
    follow_word(dist,z,170,3,0);
    assert(m::MDec(dist,z,170) == (m::Decoded::Symbol {symbol:3,position:172})) by {reveal(m::MDec);}
    assert(m::field_value(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],172,0) == 0) by (compute);
    assert(t::copy_bytes(seq![0u8, 255, 0, 0, 255, 1, 0, 0, 255, 255, 2, 0, 0, 255, 255, 255, 0, 0, 255, 3, 0, 255, 0, 255, 4, 0, 1, 255, 129, 0, 0, 0, 255, 128, 255, 0, 0, 255, 0, 255, 0, 255],6,4,0,3)) by (compute);
    t::copy_quant(raw,6,4,0,3); t::copy_prefix(raw,6,4,3);
    assert(m::MCopy(raw.take(6),42,32768,4,3) == (m::OutputEffect::Next {out:raw.take(9)}));
    assert(m::length_extra(257) == 0) by (compute); assert(m::length_base(257) == 3) by (compute);
    assert(m::distance_extra(3) == 0) by (compute); assert(m::distance_base(3) == 4) by (compute);
    assert(crate::limits::NDIST_MAX == 30) by (compute);
    assert(m::MCompressedStep(z,166,raw.take(6),42,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(9),position:172})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn subframe_step_7()
    ensures m::MCompressedStep(subframe_z(),172,subframe_raw().take(9),42,32768,subframe_lit(),subframe_dist()) == (m::SymbolEffect::Next {out:subframe_raw().take(10),position:174}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = subframe_z(); let raw = subframe_raw(); let lit = subframe_lit(); let dist = subframe_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    subframe_profiles();
    assert(subframe_z().len() == 36) by (compute); assert(subframe_raw().len() == 42) by (compute);
    assert(wire_word(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],172,2) == m::cw(subframe_lit(),255).unwrap()) by (compute);
    follow_word(lit,z,172,255,0);
    assert(m::MDec(lit,z,172) == (m::Decoded::Symbol {symbol:255,position:174})) by {reveal(m::MDec);}
    assert(raw.take(9).push(255u8) =~= raw.take(10));
    assert(m::MCompressedStep(z,172,raw.take(9),42,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(10),position:174})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn subframe_step_8()
    ensures m::MCompressedStep(subframe_z(),174,subframe_raw().take(10),42,32768,subframe_lit(),subframe_dist()) == (m::SymbolEffect::Next {out:subframe_raw().take(11),position:179}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = subframe_z(); let raw = subframe_raw(); let lit = subframe_lit(); let dist = subframe_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    subframe_profiles();
    assert(subframe_z().len() == 36) by (compute); assert(subframe_raw().len() == 42) by (compute);
    assert(wire_word(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],174,5) == m::cw(subframe_lit(),2).unwrap()) by (compute);
    follow_word(lit,z,174,2,0);
    assert(m::MDec(lit,z,174) == (m::Decoded::Symbol {symbol:2,position:179})) by {reveal(m::MDec);}
    assert(raw.take(10).push(2u8) =~= raw.take(11));
    assert(m::MCompressedStep(z,174,raw.take(10),42,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(11),position:179})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn subframe_step_9()
    ensures m::MCompressedStep(subframe_z(),179,subframe_raw().take(11),42,32768,subframe_lit(),subframe_dist()) == (m::SymbolEffect::Next {out:subframe_raw().take(15),position:185}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = subframe_z(); let raw = subframe_raw(); let lit = subframe_lit(); let dist = subframe_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    subframe_profiles();
    assert(subframe_z().len() == 36) by (compute); assert(subframe_raw().len() == 42) by (compute);
    assert(wire_word(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],179,3) == m::cw(subframe_lit(),258).unwrap()) by (compute);
    follow_word(lit,z,179,258,0);
    assert(m::MDec(lit,z,179) == (m::Decoded::Symbol {symbol:258,position:182})) by {reveal(m::MDec);}
    assert(m::field_value(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],182,0) == 0) by (compute);
    assert(wire_word(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],182,2) == m::cw(subframe_dist(),4).unwrap()) by (compute);
    follow_word(dist,z,182,4,0);
    assert(m::MDec(dist,z,182) == (m::Decoded::Symbol {symbol:4,position:184})) by {reveal(m::MDec);}
    assert(m::field_value(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],184,1) == 0) by (compute);
    assert(t::copy_bytes(seq![0u8, 255, 0, 0, 255, 1, 0, 0, 255, 255, 2, 0, 0, 255, 255, 255, 0, 0, 255, 3, 0, 255, 0, 255, 4, 0, 1, 255, 129, 0, 0, 0, 255, 128, 255, 0, 0, 255, 0, 255, 0, 255],11,5,0,4)) by (compute);
    t::copy_quant(raw,11,5,0,4); t::copy_prefix(raw,11,5,4);
    assert(m::MCopy(raw.take(11),42,32768,5,4) == (m::OutputEffect::Next {out:raw.take(15)}));
    assert(m::length_extra(258) == 0) by (compute); assert(m::length_base(258) == 4) by (compute);
    assert(m::distance_extra(4) == 1) by (compute); assert(m::distance_base(4) == 5) by (compute);
    assert(crate::limits::NDIST_MAX == 30) by (compute);
    assert(m::MCompressedStep(z,179,raw.take(11),42,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(15),position:185})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn subframe_step_10()
    ensures m::MCompressedStep(subframe_z(),185,subframe_raw().take(15),42,32768,subframe_lit(),subframe_dist()) == (m::SymbolEffect::Next {out:subframe_raw().take(19),position:192}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = subframe_z(); let raw = subframe_raw(); let lit = subframe_lit(); let dist = subframe_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    subframe_profiles();
    assert(subframe_z().len() == 36) by (compute); assert(subframe_raw().len() == 42) by (compute);
    assert(wire_word(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],185,3) == m::cw(subframe_lit(),258).unwrap()) by (compute);
    follow_word(lit,z,185,258,0);
    assert(m::MDec(lit,z,185) == (m::Decoded::Symbol {symbol:258,position:188})) by {reveal(m::MDec);}
    assert(m::field_value(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],188,0) == 0) by (compute);
    assert(wire_word(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],188,2) == m::cw(subframe_dist(),7).unwrap()) by (compute);
    follow_word(dist,z,188,7,0);
    assert(m::MDec(dist,z,188) == (m::Decoded::Symbol {symbol:7,position:190})) by {reveal(m::MDec);}
    assert(m::field_value(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],190,2) == 1) by (compute);
    assert(t::copy_bytes(seq![0u8, 255, 0, 0, 255, 1, 0, 0, 255, 255, 2, 0, 0, 255, 255, 255, 0, 0, 255, 3, 0, 255, 0, 255, 4, 0, 1, 255, 129, 0, 0, 0, 255, 128, 255, 0, 0, 255, 0, 255, 0, 255],15,14,0,4)) by (compute);
    t::copy_quant(raw,15,14,0,4); t::copy_prefix(raw,15,14,4);
    assert(m::MCopy(raw.take(15),42,32768,14,4) == (m::OutputEffect::Next {out:raw.take(19)}));
    assert(m::length_extra(258) == 0) by (compute); assert(m::length_base(258) == 4) by (compute);
    assert(m::distance_extra(7) == 2) by (compute); assert(m::distance_base(7) == 13) by (compute);
    assert(crate::limits::NDIST_MAX == 30) by (compute);
    assert(m::MCompressedStep(z,185,raw.take(15),42,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(19),position:192})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn subframe_step_11()
    ensures m::MCompressedStep(subframe_z(),192,subframe_raw().take(19),42,32768,subframe_lit(),subframe_dist()) == (m::SymbolEffect::Next {out:subframe_raw().take(20),position:196}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = subframe_z(); let raw = subframe_raw(); let lit = subframe_lit(); let dist = subframe_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    subframe_profiles();
    assert(subframe_z().len() == 36) by (compute); assert(subframe_raw().len() == 42) by (compute);
    assert(wire_word(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],192,4) == m::cw(subframe_lit(),3).unwrap()) by (compute);
    follow_word(lit,z,192,3,0);
    assert(m::MDec(lit,z,192) == (m::Decoded::Symbol {symbol:3,position:196})) by {reveal(m::MDec);}
    assert(raw.take(19).push(3u8) =~= raw.take(20));
    assert(m::MCompressedStep(z,192,raw.take(19),42,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(20),position:196})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn subframe_step_12()
    ensures m::MCompressedStep(subframe_z(),196,subframe_raw().take(20),42,32768,subframe_lit(),subframe_dist()) == (m::SymbolEffect::Next {out:subframe_raw().take(21),position:198}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = subframe_z(); let raw = subframe_raw(); let lit = subframe_lit(); let dist = subframe_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    subframe_profiles();
    assert(subframe_z().len() == 36) by (compute); assert(subframe_raw().len() == 42) by (compute);
    assert(wire_word(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],196,2) == m::cw(subframe_lit(),0).unwrap()) by (compute);
    follow_word(lit,z,196,0,0);
    assert(m::MDec(lit,z,196) == (m::Decoded::Symbol {symbol:0,position:198})) by {reveal(m::MDec);}
    assert(raw.take(20).push(0u8) =~= raw.take(21));
    assert(m::MCompressedStep(z,196,raw.take(20),42,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(21),position:198})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn subframe_step_13()
    ensures m::MCompressedStep(subframe_z(),198,subframe_raw().take(21),42,32768,subframe_lit(),subframe_dist()) == (m::SymbolEffect::Next {out:subframe_raw().take(22),position:200}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = subframe_z(); let raw = subframe_raw(); let lit = subframe_lit(); let dist = subframe_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    subframe_profiles();
    assert(subframe_z().len() == 36) by (compute); assert(subframe_raw().len() == 42) by (compute);
    assert(wire_word(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],198,2) == m::cw(subframe_lit(),255).unwrap()) by (compute);
    follow_word(lit,z,198,255,0);
    assert(m::MDec(lit,z,198) == (m::Decoded::Symbol {symbol:255,position:200})) by {reveal(m::MDec);}
    assert(raw.take(21).push(255u8) =~= raw.take(22));
    assert(m::MCompressedStep(z,198,raw.take(21),42,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(22),position:200})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn subframe_step_14()
    ensures m::MCompressedStep(subframe_z(),200,subframe_raw().take(22),42,32768,subframe_lit(),subframe_dist()) == (m::SymbolEffect::Next {out:subframe_raw().take(23),position:202}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = subframe_z(); let raw = subframe_raw(); let lit = subframe_lit(); let dist = subframe_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    subframe_profiles();
    assert(subframe_z().len() == 36) by (compute); assert(subframe_raw().len() == 42) by (compute);
    assert(wire_word(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],200,2) == m::cw(subframe_lit(),0).unwrap()) by (compute);
    follow_word(lit,z,200,0,0);
    assert(m::MDec(lit,z,200) == (m::Decoded::Symbol {symbol:0,position:202})) by {reveal(m::MDec);}
    assert(raw.take(22).push(0u8) =~= raw.take(23));
    assert(m::MCompressedStep(z,200,raw.take(22),42,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(23),position:202})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn subframe_step_15()
    ensures m::MCompressedStep(subframe_z(),202,subframe_raw().take(23),42,32768,subframe_lit(),subframe_dist()) == (m::SymbolEffect::Next {out:subframe_raw().take(24),position:204}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = subframe_z(); let raw = subframe_raw(); let lit = subframe_lit(); let dist = subframe_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    subframe_profiles();
    assert(subframe_z().len() == 36) by (compute); assert(subframe_raw().len() == 42) by (compute);
    assert(wire_word(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],202,2) == m::cw(subframe_lit(),255).unwrap()) by (compute);
    follow_word(lit,z,202,255,0);
    assert(m::MDec(lit,z,202) == (m::Decoded::Symbol {symbol:255,position:204})) by {reveal(m::MDec);}
    assert(raw.take(23).push(255u8) =~= raw.take(24));
    assert(m::MCompressedStep(z,202,raw.take(23),42,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(24),position:204})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn subframe_step_16()
    ensures m::MCompressedStep(subframe_z(),204,subframe_raw().take(24),42,32768,subframe_lit(),subframe_dist()) == (m::SymbolEffect::Next {out:subframe_raw().take(25),position:209}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = subframe_z(); let raw = subframe_raw(); let lit = subframe_lit(); let dist = subframe_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    subframe_profiles();
    assert(subframe_z().len() == 36) by (compute); assert(subframe_raw().len() == 42) by (compute);
    assert(wire_word(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],204,5) == m::cw(subframe_lit(),4).unwrap()) by (compute);
    follow_word(lit,z,204,4,0);
    assert(m::MDec(lit,z,204) == (m::Decoded::Symbol {symbol:4,position:209})) by {reveal(m::MDec);}
    assert(raw.take(24).push(4u8) =~= raw.take(25));
    assert(m::MCompressedStep(z,204,raw.take(24),42,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(25),position:209})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn subframe_step_17()
    ensures m::MCompressedStep(subframe_z(),209,subframe_raw().take(25),42,32768,subframe_lit(),subframe_dist()) == (m::SymbolEffect::Next {out:subframe_raw().take(26),position:211}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = subframe_z(); let raw = subframe_raw(); let lit = subframe_lit(); let dist = subframe_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    subframe_profiles();
    assert(subframe_z().len() == 36) by (compute); assert(subframe_raw().len() == 42) by (compute);
    assert(wire_word(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],209,2) == m::cw(subframe_lit(),0).unwrap()) by (compute);
    follow_word(lit,z,209,0,0);
    assert(m::MDec(lit,z,209) == (m::Decoded::Symbol {symbol:0,position:211})) by {reveal(m::MDec);}
    assert(raw.take(25).push(0u8) =~= raw.take(26));
    assert(m::MCompressedStep(z,209,raw.take(25),42,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(26),position:211})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn subframe_step_18()
    ensures m::MCompressedStep(subframe_z(),211,subframe_raw().take(26),42,32768,subframe_lit(),subframe_dist()) == (m::SymbolEffect::Next {out:subframe_raw().take(27),position:215}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = subframe_z(); let raw = subframe_raw(); let lit = subframe_lit(); let dist = subframe_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    subframe_profiles();
    assert(subframe_z().len() == 36) by (compute); assert(subframe_raw().len() == 42) by (compute);
    assert(wire_word(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],211,4) == m::cw(subframe_lit(),1).unwrap()) by (compute);
    follow_word(lit,z,211,1,0);
    assert(m::MDec(lit,z,211) == (m::Decoded::Symbol {symbol:1,position:215})) by {reveal(m::MDec);}
    assert(raw.take(26).push(1u8) =~= raw.take(27));
    assert(m::MCompressedStep(z,211,raw.take(26),42,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(27),position:215})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn subframe_step_19()
    ensures m::MCompressedStep(subframe_z(),215,subframe_raw().take(27),42,32768,subframe_lit(),subframe_dist()) == (m::SymbolEffect::Next {out:subframe_raw().take(28),position:217}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = subframe_z(); let raw = subframe_raw(); let lit = subframe_lit(); let dist = subframe_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    subframe_profiles();
    assert(subframe_z().len() == 36) by (compute); assert(subframe_raw().len() == 42) by (compute);
    assert(wire_word(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],215,2) == m::cw(subframe_lit(),255).unwrap()) by (compute);
    follow_word(lit,z,215,255,0);
    assert(m::MDec(lit,z,215) == (m::Decoded::Symbol {symbol:255,position:217})) by {reveal(m::MDec);}
    assert(raw.take(27).push(255u8) =~= raw.take(28));
    assert(m::MCompressedStep(z,215,raw.take(27),42,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(28),position:217})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn subframe_step_20()
    ensures m::MCompressedStep(subframe_z(),217,subframe_raw().take(28),42,32768,subframe_lit(),subframe_dist()) == (m::SymbolEffect::Next {out:subframe_raw().take(29),position:222}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = subframe_z(); let raw = subframe_raw(); let lit = subframe_lit(); let dist = subframe_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    subframe_profiles();
    assert(subframe_z().len() == 36) by (compute); assert(subframe_raw().len() == 42) by (compute);
    assert(wire_word(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],217,5) == m::cw(subframe_lit(),129).unwrap()) by (compute);
    follow_word(lit,z,217,129,0);
    assert(m::MDec(lit,z,217) == (m::Decoded::Symbol {symbol:129,position:222})) by {reveal(m::MDec);}
    assert(raw.take(28).push(129u8) =~= raw.take(29));
    assert(m::MCompressedStep(z,217,raw.take(28),42,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(29),position:222})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn subframe_step_21()
    ensures m::MCompressedStep(subframe_z(),222,subframe_raw().take(29),42,32768,subframe_lit(),subframe_dist()) == (m::SymbolEffect::Next {out:subframe_raw().take(30),position:224}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = subframe_z(); let raw = subframe_raw(); let lit = subframe_lit(); let dist = subframe_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    subframe_profiles();
    assert(subframe_z().len() == 36) by (compute); assert(subframe_raw().len() == 42) by (compute);
    assert(wire_word(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],222,2) == m::cw(subframe_lit(),0).unwrap()) by (compute);
    follow_word(lit,z,222,0,0);
    assert(m::MDec(lit,z,222) == (m::Decoded::Symbol {symbol:0,position:224})) by {reveal(m::MDec);}
    assert(raw.take(29).push(0u8) =~= raw.take(30));
    assert(m::MCompressedStep(z,222,raw.take(29),42,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(30),position:224})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn subframe_step_22()
    ensures m::MCompressedStep(subframe_z(),224,subframe_raw().take(30),42,32768,subframe_lit(),subframe_dist()) == (m::SymbolEffect::Next {out:subframe_raw().take(33),position:232}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = subframe_z(); let raw = subframe_raw(); let lit = subframe_lit(); let dist = subframe_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    subframe_profiles();
    assert(subframe_z().len() == 36) by (compute); assert(subframe_raw().len() == 42) by (compute);
    assert(wire_word(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],224,4) == m::cw(subframe_lit(),257).unwrap()) by (compute);
    follow_word(lit,z,224,257,0);
    assert(m::MDec(lit,z,224) == (m::Decoded::Symbol {symbol:257,position:228})) by {reveal(m::MDec);}
    assert(m::field_value(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],228,0) == 0) by (compute);
    assert(wire_word(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],228,2) == m::cw(subframe_dist(),7).unwrap()) by (compute);
    follow_word(dist,z,228,7,0);
    assert(m::MDec(dist,z,228) == (m::Decoded::Symbol {symbol:7,position:230})) by {reveal(m::MDec);}
    assert(m::field_value(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],230,2) == 1) by (compute);
    assert(t::copy_bytes(seq![0u8, 255, 0, 0, 255, 1, 0, 0, 255, 255, 2, 0, 0, 255, 255, 255, 0, 0, 255, 3, 0, 255, 0, 255, 4, 0, 1, 255, 129, 0, 0, 0, 255, 128, 255, 0, 0, 255, 0, 255, 0, 255],30,14,0,3)) by (compute);
    t::copy_quant(raw,30,14,0,3); t::copy_prefix(raw,30,14,3);
    assert(m::MCopy(raw.take(30),42,32768,14,3) == (m::OutputEffect::Next {out:raw.take(33)}));
    assert(m::length_extra(257) == 0) by (compute); assert(m::length_base(257) == 3) by (compute);
    assert(m::distance_extra(7) == 2) by (compute); assert(m::distance_base(7) == 13) by (compute);
    assert(crate::limits::NDIST_MAX == 30) by (compute);
    assert(m::MCompressedStep(z,224,raw.take(30),42,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(33),position:232})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn subframe_step_23()
    ensures m::MCompressedStep(subframe_z(),232,subframe_raw().take(33),42,32768,subframe_lit(),subframe_dist()) == (m::SymbolEffect::Next {out:subframe_raw().take(34),position:237}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = subframe_z(); let raw = subframe_raw(); let lit = subframe_lit(); let dist = subframe_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    subframe_profiles();
    assert(subframe_z().len() == 36) by (compute); assert(subframe_raw().len() == 42) by (compute);
    assert(wire_word(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],232,5) == m::cw(subframe_lit(),128).unwrap()) by (compute);
    follow_word(lit,z,232,128,0);
    assert(m::MDec(lit,z,232) == (m::Decoded::Symbol {symbol:128,position:237})) by {reveal(m::MDec);}
    assert(raw.take(33).push(128u8) =~= raw.take(34));
    assert(m::MCompressedStep(z,232,raw.take(33),42,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(34),position:237})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn subframe_step_24()
    ensures m::MCompressedStep(subframe_z(),237,subframe_raw().take(34),42,32768,subframe_lit(),subframe_dist()) == (m::SymbolEffect::Next {out:subframe_raw().take(38),position:246}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = subframe_z(); let raw = subframe_raw(); let lit = subframe_lit(); let dist = subframe_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    subframe_profiles();
    assert(subframe_z().len() == 36) by (compute); assert(subframe_raw().len() == 42) by (compute);
    assert(wire_word(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],237,3) == m::cw(subframe_lit(),258).unwrap()) by (compute);
    follow_word(lit,z,237,258,0);
    assert(m::MDec(lit,z,237) == (m::Decoded::Symbol {symbol:258,position:240})) by {reveal(m::MDec);}
    assert(m::field_value(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],240,0) == 0) by (compute);
    assert(wire_word(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],240,3) == m::cw(subframe_dist(),8).unwrap()) by (compute);
    follow_word(dist,z,240,8,0);
    assert(m::MDec(dist,z,240) == (m::Decoded::Symbol {symbol:8,position:243})) by {reveal(m::MDec);}
    assert(m::field_value(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],243,3) == 2) by (compute);
    assert(t::copy_bytes(seq![0u8, 255, 0, 0, 255, 1, 0, 0, 255, 255, 2, 0, 0, 255, 255, 255, 0, 0, 255, 3, 0, 255, 0, 255, 4, 0, 1, 255, 129, 0, 0, 0, 255, 128, 255, 0, 0, 255, 0, 255, 0, 255],34,19,0,4)) by (compute);
    t::copy_quant(raw,34,19,0,4); t::copy_prefix(raw,34,19,4);
    assert(m::MCopy(raw.take(34),42,32768,19,4) == (m::OutputEffect::Next {out:raw.take(38)}));
    assert(m::length_extra(258) == 0) by (compute); assert(m::length_base(258) == 4) by (compute);
    assert(m::distance_extra(8) == 3) by (compute); assert(m::distance_base(8) == 17) by (compute);
    assert(crate::limits::NDIST_MAX == 30) by (compute);
    assert(m::MCompressedStep(z,237,raw.take(34),42,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(38),position:246})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn subframe_step_25()
    ensures m::MCompressedStep(subframe_z(),246,subframe_raw().take(38),42,32768,subframe_lit(),subframe_dist()) == (m::SymbolEffect::Next {out:subframe_raw().take(42),position:252}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = subframe_z(); let raw = subframe_raw(); let lit = subframe_lit(); let dist = subframe_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    subframe_profiles();
    assert(subframe_z().len() == 36) by (compute); assert(subframe_raw().len() == 42) by (compute);
    assert(wire_word(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],246,3) == m::cw(subframe_lit(),258).unwrap()) by (compute);
    follow_word(lit,z,246,258,0);
    assert(m::MDec(lit,z,246) == (m::Decoded::Symbol {symbol:258,position:249})) by {reveal(m::MDec);}
    assert(m::field_value(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],249,0) == 0) by (compute);
    assert(wire_word(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],249,3) == m::cw(subframe_dist(),1).unwrap()) by (compute);
    follow_word(dist,z,249,1,0);
    assert(m::MDec(dist,z,249) == (m::Decoded::Symbol {symbol:1,position:252})) by {reveal(m::MDec);}
    assert(m::field_value(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],252,0) == 0) by (compute);
    assert(t::copy_bytes(seq![0u8, 255, 0, 0, 255, 1, 0, 0, 255, 255, 2, 0, 0, 255, 255, 255, 0, 0, 255, 3, 0, 255, 0, 255, 4, 0, 1, 255, 129, 0, 0, 0, 255, 128, 255, 0, 0, 255, 0, 255, 0, 255],38,2,0,4)) by (compute);
    t::copy_quant(raw,38,2,0,4); t::copy_prefix(raw,38,2,4);
    assert(m::MCopy(raw.take(38),42,32768,2,4) == (m::OutputEffect::Next {out:raw.take(42)}));
    assert(m::length_extra(258) == 0) by (compute); assert(m::length_base(258) == 4) by (compute);
    assert(m::distance_extra(1) == 0) by (compute); assert(m::distance_base(1) == 2) by (compute);
    assert(crate::limits::NDIST_MAX == 30) by (compute);
    assert(m::MCompressedStep(z,246,raw.take(38),42,32768,lit,dist) == (m::SymbolEffect::Next {out:raw.take(42),position:252})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn subframe_step_26()
    ensures m::MCompressedStep(subframe_z(),252,subframe_raw().take(42),42,32768,subframe_lit(),subframe_dist()) == (m::SymbolEffect::End {out:subframe_raw(),position:256}),
{

    hide(m::MDec); hide(m::decode_prefix); hide(m::MCompressedStep); hide(m::copy_distance);
    let z = subframe_z(); let raw = subframe_raw(); let lit = subframe_lit(); let dist = subframe_dist();
    assert(crate::limits::CODE_SPACE == 32768) by (compute); assert(crate::limits::MAX_CODE_BITS == 15) by (compute); assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute); assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(crate::limits::LEN_MAX == 258) by (compute);
    subframe_profiles();
    assert(subframe_z().len() == 36) by (compute); assert(subframe_raw().len() == 42) by (compute);
    assert(wire_word(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],252,4) == m::cw(subframe_lit(),256).unwrap()) by (compute);
    follow_word(lit,z,252,256,0);
    assert(m::MDec(lit,z,252) == (m::Decoded::Symbol {symbol:256,position:256})) by {reveal(m::MDec);}
    assert(raw.take(42) =~= raw);
    assert(m::MCompressedStep(z,252,raw.take(42),42,32768,lit,dist) == (m::SymbolEffect::End {out:raw,position:256})) by {reveal(m::MCompressedStep);}
}
#[verifier::spinoff_prover]
proof fn subframe_payload()
    ensures m::MCompressed(subframe_z(),152,Seq::empty(),42,32768,subframe_lit(),subframe_dist()) == (m::StoredEffect::End {out:subframe_raw(),position:256}),
{
    hide(m::MCompressedStep); hide(m::MCompressed);
    let z = subframe_z(); let raw = subframe_raw(); let lit = subframe_lit(); let dist = subframe_dist();
    subframe_step_0();
    subframe_step_1();
    subframe_step_2();
    subframe_step_3();
    subframe_step_4();
    subframe_step_5();
    subframe_step_6();
    subframe_step_7();
    subframe_step_8();
    subframe_step_9();
    subframe_step_10();
    subframe_step_11();
    subframe_step_12();
    subframe_step_13();
    subframe_step_14();
    subframe_step_15();
    subframe_step_16();
    subframe_step_17();
    subframe_step_18();
    subframe_step_19();
    subframe_step_20();
    subframe_step_21();
    subframe_step_22();
    subframe_step_23();
    subframe_step_24();
    subframe_step_25();
    subframe_step_26();
    assert(raw.take(0) =~= Seq::<u8>::empty());
    assert(m::MCompressed(z,152,Seq::empty(),42,32768,lit,dist) == (m::StoredEffect::End {out:raw,position:256})) by {reveal_with_fuel(m::MCompressed,28);}
}
#[verifier::spinoff_prover]
fn subframe() {
    hide(m::MDec); hide(m::MDynamic); hide(m::MCompressed); hide(m::sum_bytes); hide(m::sum_prefixes);
    let segment_0 = subframe_part_0();
    let parts: [&[u8];1] = [&segment_0];
    proof {
        subframe_header(); subframe_payload();
        let z = subframe_z(); let raw = subframe_raw();
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(m::field_value(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],16,1) == 1) by (compute); assert(m::field_value(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],17,2) == 2) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),42,32768,true) == (m::StoredEffect::End {out:raw,position:256}));
        assert(m::MFinish(seq![120u8, 156, 21, 136, 129, 13, 0, 48, 8, 131, 112, 238, 240, 237, 114, 180, 105, 32, 161, 8, 214, 226, 137, 82, 141, 120, 41, 63, 91, 47, 87, 54, 91, 121, 16, 253],256,seq![0u8, 255, 0, 0, 255, 1, 0, 0, 255, 255, 2, 0, 0, 255, 255, 255, 0, 0, 255, 3, 0, 255, 0, 255, 4, 0, 1, 255, 129, 0, 0, 0, 255, 128, 255, 0, 0, 255, 0, 255, 0, 255],42,false) == (m::StoredEffect::End {out:seq![0u8, 255, 0, 0, 255, 1, 0, 0, 255, 255, 2, 0, 0, 255, 255, 255, 0, 0, 255, 3, 0, 255, 0, 255, 4, 0, 1, 255, 129, 0, 0, 0, 255, 128, 255, 0, 0, 255, 0, 255, 0, 255],position:288})) by (compute);
        assert(m::Inflate(z,42,true,false).status == m::InflateStatus::Done); assert(m::Inflate(z,42,true,false).out == raw);
        reveal_with_fuel(crate::segments::parts_prefix,2); assert(crate::segments::parts_prefix(parts@,1) =~= z);
        assert(crate::limits::SEGMENT_BYTES_MAX >= 36) by (compute);
    }
    let ghost raw = subframe_raw(); let mut output = [0xa5u8;42]; check_policy_parts(&parts,&mut output,true,false,Status::Done,Ghost(raw));
}
} // verus!
