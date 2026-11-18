//! Synthetic independently authored window boundaries and measuring ring wrap.
use crate::{
    model as m,
    witness_window::*,
    witnesses::{fixed_decode, fixed_dist_decode, wire_word},
    InflateError as E, Status,
};
use vstd::prelude::*;
verus! {
// Synthetic window_exact: stored 256 zeros, then length 3 at distance 256.
#[verifier::spinoff_prover]
fn window_exact() {
    let mut wire=[0u8;271];
    wire[0]=8;
    wire[1]=29;
    wire[4]=1;
    wire[5]=255;
    wire[6]=254;
    wire[263]=3;
    wire[264]=250;
    wire[265]=31;
    wire[267]=1;
    wire[268]=3;
    wire[270]=1;
    proof {
        let z=Seq::new(271,|i:int| if i < 7 { seq![8u8, 29, 0, 0, 1, 255, 254][i] } else if i < 263 {0u8} else { seq![3u8, 250, 31, 0, 1, 3, 0, 1][i-263] });
        assert(wire@ =~= z);
        assert(z.subrange(7,263) =~= zeros(256));
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute); assert(m::pow2(8) == 256) by (compute); assert(crate::limits::NDIST_MAX == 30) by (compute);
        assert(m::distance_extra(15) == 6) by (compute); assert(m::distance_base(15) == 193) by (compute);
        assert(wire_word(Seq::new(271,|i:int| if i < 7 { seq![8u8, 29, 0, 0, 1, 255, 254][i] } else if i < 263 {0u8} else { seq![3u8, 250, 31, 0, 1, 3, 0, 1][i-263] }),2107,7) == m::msb_bits(1,7)) by (compute); fixed_decode(z,2107,257);
        assert(wire_word(Seq::new(271,|i:int| if i < 7 { seq![8u8, 29, 0, 0, 1, 255, 254][i] } else if i < 263 {0u8} else { seq![3u8, 250, 31, 0, 1, 3, 0, 1][i-263] }),2114,5) == m::msb_bits(15,5)) by (compute); fixed_dist_decode(z,2114,15);
        assert(m::field_value(Seq::new(271,|i:int| if i < 7 { seq![8u8, 29, 0, 0, 1, 255, 254][i] } else if i < 263 {0u8} else { seq![3u8, 250, 31, 0, 1, 3, 0, 1][i-263] }),2119,6) == 63) by (compute);
        assert(wire_word(Seq::new(271,|i:int| if i < 7 { seq![8u8, 29, 0, 0, 1, 255, 254][i] } else if i < 263 {0u8} else { seq![3u8, 250, 31, 0, 1, 3, 0, 1][i-263] }),2125,7) == m::msb_bits(0,7)) by (compute); fixed_decode(z,2125,256);
        assert(m::field_value(Seq::new(271,|i:int| if i < 7 { seq![8u8, 29, 0, 0, 1, 255, 254][i] } else if i < 263 {0u8} else { seq![3u8, 250, 31, 0, 1, 3, 0, 1][i-263] }),16,1) == 0) by (compute); assert(m::field_value(Seq::new(271,|i:int| if i < 7 { seq![8u8, 29, 0, 0, 1, 255, 254][i] } else if i < 263 {0u8} else { seq![3u8, 250, 31, 0, 1, 3, 0, 1][i-263] }),17,2) == 0) by (compute);
        assert(m::field_value(Seq::new(271,|i:int| if i < 7 { seq![8u8, 29, 0, 0, 1, 255, 254][i] } else if i < 263 {0u8} else { seq![3u8, 250, 31, 0, 1, 3, 0, 1][i-263] }),2104,1) == 1) by (compute); assert(m::field_value(Seq::new(271,|i:int| if i < 7 { seq![8u8, 29, 0, 0, 1, 255, 254][i] } else if i < 263 {0u8} else { seq![3u8, 250, 31, 0, 1, 3, 0, 1][i-263] }),2105,2) == 1) by (compute);
        zero_sums(259);
        assert(crate::limits::ADLER_MOD == 65521) by (compute); assert(m::pow2(16) == u16::MAX + 1) by (compute);
        assert(m::be_value(Seq::new(271,|i:int| if i < 7 { seq![8u8, 29, 0, 0, 1, 255, 254][i] } else if i < 263 {0u8} else { seq![3u8, 250, 31, 0, 1, 3, 0, 1][i-263] }),267,4) == 16973825) by (compute);
        zero_blocks(z,256,256,15,63,6);
        assert(m::Inflate(z,259,true,false).status == m::InflateStatus::Done);
        assert(m::Inflate(z,259,true,false).out == zeros(259));
        zero_blocks(z,256,32768,15,63,6);
        assert(m::Inflate(z,259,false,false).status == m::InflateStatus::Done);
        assert(m::Inflate(z,259,false,false).out == zeros(259));
        assert(crate::limits::SEGMENT_BYTES_MAX >= 271) by (compute);
    }
    let mut output=[0xa5u8;259];
    let ghost raw=zeros(259); check_options(&wire,&mut output,true,Status::Done,Ghost(raw));
    let ghost raw=zeros(259); check_options(&wire,&mut output,false,Status::Done,Ghost(raw));
}
// Synthetic window_plus_one: stored 257 zeros, then length 3 at distance 257.
#[verifier::spinoff_prover]
fn window_plus_one() {
    let mut wire=[0u8;272];
    wire[0]=8;
    wire[1]=29;
    wire[3]=1;
    wire[4]=1;
    wire[5]=254;
    wire[6]=254;
    wire[264]=3;
    wire[265]=6;
    wire[268]=1;
    wire[269]=4;
    wire[271]=1;
    proof {
        let z=Seq::new(272,|i:int| if i < 7 { seq![8u8, 29, 0, 1, 1, 254, 254][i] } else if i < 264 {0u8} else { seq![3u8, 6, 0, 0, 1, 4, 0, 1][i-264] });
        assert(wire@ =~= z);
        assert(z.subrange(7,264) =~= zeros(257));
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute); assert(m::pow2(8) == 256) by (compute); assert(crate::limits::NDIST_MAX == 30) by (compute);
        assert(m::distance_extra(16) == 7) by (compute); assert(m::distance_base(16) == 257) by (compute);
        assert(wire_word(Seq::new(272,|i:int| if i < 7 { seq![8u8, 29, 0, 1, 1, 254, 254][i] } else if i < 264 {0u8} else { seq![3u8, 6, 0, 0, 1, 4, 0, 1][i-264] }),2115,7) == m::msb_bits(1,7)) by (compute); fixed_decode(z,2115,257);
        assert(wire_word(Seq::new(272,|i:int| if i < 7 { seq![8u8, 29, 0, 1, 1, 254, 254][i] } else if i < 264 {0u8} else { seq![3u8, 6, 0, 0, 1, 4, 0, 1][i-264] }),2122,5) == m::msb_bits(16,5)) by (compute); fixed_dist_decode(z,2122,16);
        assert(m::field_value(Seq::new(272,|i:int| if i < 7 { seq![8u8, 29, 0, 1, 1, 254, 254][i] } else if i < 264 {0u8} else { seq![3u8, 6, 0, 0, 1, 4, 0, 1][i-264] }),2127,7) == 0) by (compute);
        assert(wire_word(Seq::new(272,|i:int| if i < 7 { seq![8u8, 29, 0, 1, 1, 254, 254][i] } else if i < 264 {0u8} else { seq![3u8, 6, 0, 0, 1, 4, 0, 1][i-264] }),2134,7) == m::msb_bits(0,7)) by (compute); fixed_decode(z,2134,256);
        assert(m::field_value(Seq::new(272,|i:int| if i < 7 { seq![8u8, 29, 0, 1, 1, 254, 254][i] } else if i < 264 {0u8} else { seq![3u8, 6, 0, 0, 1, 4, 0, 1][i-264] }),16,1) == 0) by (compute); assert(m::field_value(Seq::new(272,|i:int| if i < 7 { seq![8u8, 29, 0, 1, 1, 254, 254][i] } else if i < 264 {0u8} else { seq![3u8, 6, 0, 0, 1, 4, 0, 1][i-264] }),17,2) == 0) by (compute);
        assert(m::field_value(Seq::new(272,|i:int| if i < 7 { seq![8u8, 29, 0, 1, 1, 254, 254][i] } else if i < 264 {0u8} else { seq![3u8, 6, 0, 0, 1, 4, 0, 1][i-264] }),2112,1) == 1) by (compute); assert(m::field_value(Seq::new(272,|i:int| if i < 7 { seq![8u8, 29, 0, 1, 1, 254, 254][i] } else if i < 264 {0u8} else { seq![3u8, 6, 0, 0, 1, 4, 0, 1][i-264] }),2113,2) == 1) by (compute);
        zero_sums(260);
        assert(crate::limits::ADLER_MOD == 65521) by (compute); assert(m::pow2(16) == u16::MAX + 1) by (compute);
        assert(m::be_value(Seq::new(272,|i:int| if i < 7 { seq![8u8, 29, 0, 1, 1, 254, 254][i] } else if i < 264 {0u8} else { seq![3u8, 6, 0, 0, 1, 4, 0, 1][i-264] }),268,4) == 17039361) by (compute);
        zero_blocks(z,257,256,16,0,7);
        assert(m::Inflate(z,260,true,false).status == m::InflateStatus::Error(m::InflateFault::DistanceTooFar));
        assert(m::Inflate(z,260,true,false).out == zeros(257));
        zero_blocks(z,257,32768,16,0,7);
        assert(m::Inflate(z,260,false,false).status == m::InflateStatus::Done);
        assert(m::Inflate(z,260,false,false).out == zeros(260));
        assert(crate::limits::SEGMENT_BYTES_MAX >= 272) by (compute);
    }
    let mut output=[0xa5u8;260];
    let ghost raw=zeros(257); check_options(&wire,&mut output,true,Status::Err(E::DistanceTooFar),Ghost(raw));
    let ghost raw=zeros(260); check_options(&wire,&mut output,false,Status::Done,Ghost(raw));
}
// Synthetic ring_wrap_exact_distance: stored 32768 zeros, then length 3 at distance 32768.
#[verifier::spinoff_prover]
fn ring_wrap_exact_distance() {
    let mut wire=[0u8;32784];
    wire[0]=120;
    wire[1]=1;
    wire[4]=128;
    wire[5]=255;
    wire[6]=127;
    wire[32775]=3;
    wire[32776]=222;
    wire[32777]=255;
    wire[32778]=15;
    wire[32780]=128;
    wire[32781]=3;
    wire[32783]=1;
    proof {
        let z=Seq::new(32784,|i:int| if i < 7 { seq![120u8, 1, 0, 0, 128, 255, 127][i] } else if i < 32775 {0u8} else { seq![3u8, 222, 255, 15, 0, 128, 3, 0, 1][i-32775] });
        assert(wire@ =~= z);
        assert(z.subrange(7,32775) =~= zeros(32768));
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute); assert(m::pow2(8) == 256) by (compute); assert(crate::limits::NDIST_MAX == 30) by (compute);
        assert(m::distance_extra(29) == 13) by (compute); assert(m::distance_base(29) == 24577) by (compute);
        assert(wire_word(Seq::new(32784,|i:int| if i < 7 { seq![120u8, 1, 0, 0, 128, 255, 127][i] } else if i < 32775 {0u8} else { seq![3u8, 222, 255, 15, 0, 128, 3, 0, 1][i-32775] }),262203,7) == m::msb_bits(1,7)) by (compute); fixed_decode(z,262203,257);
        assert(wire_word(Seq::new(32784,|i:int| if i < 7 { seq![120u8, 1, 0, 0, 128, 255, 127][i] } else if i < 32775 {0u8} else { seq![3u8, 222, 255, 15, 0, 128, 3, 0, 1][i-32775] }),262210,5) == m::msb_bits(29,5)) by (compute); fixed_dist_decode(z,262210,29);
        assert(m::field_value(Seq::new(32784,|i:int| if i < 7 { seq![120u8, 1, 0, 0, 128, 255, 127][i] } else if i < 32775 {0u8} else { seq![3u8, 222, 255, 15, 0, 128, 3, 0, 1][i-32775] }),262215,13) == 8191) by (compute);
        assert(wire_word(Seq::new(32784,|i:int| if i < 7 { seq![120u8, 1, 0, 0, 128, 255, 127][i] } else if i < 32775 {0u8} else { seq![3u8, 222, 255, 15, 0, 128, 3, 0, 1][i-32775] }),262228,7) == m::msb_bits(0,7)) by (compute); fixed_decode(z,262228,256);
        assert(m::field_value(Seq::new(32784,|i:int| if i < 7 { seq![120u8, 1, 0, 0, 128, 255, 127][i] } else if i < 32775 {0u8} else { seq![3u8, 222, 255, 15, 0, 128, 3, 0, 1][i-32775] }),16,1) == 0) by (compute); assert(m::field_value(Seq::new(32784,|i:int| if i < 7 { seq![120u8, 1, 0, 0, 128, 255, 127][i] } else if i < 32775 {0u8} else { seq![3u8, 222, 255, 15, 0, 128, 3, 0, 1][i-32775] }),17,2) == 0) by (compute);
        assert(m::field_value(Seq::new(32784,|i:int| if i < 7 { seq![120u8, 1, 0, 0, 128, 255, 127][i] } else if i < 32775 {0u8} else { seq![3u8, 222, 255, 15, 0, 128, 3, 0, 1][i-32775] }),262200,1) == 1) by (compute); assert(m::field_value(Seq::new(32784,|i:int| if i < 7 { seq![120u8, 1, 0, 0, 128, 255, 127][i] } else if i < 32775 {0u8} else { seq![3u8, 222, 255, 15, 0, 128, 3, 0, 1][i-32775] }),262201,2) == 1) by (compute);
        zero_sums(32771);
        assert(crate::limits::ADLER_MOD == 65521) by (compute); assert(m::pow2(16) == u16::MAX + 1) by (compute);
        assert(m::be_value(Seq::new(32784,|i:int| if i < 7 { seq![120u8, 1, 0, 0, 128, 255, 127][i] } else if i < 32775 {0u8} else { seq![3u8, 222, 255, 15, 0, 128, 3, 0, 1][i-32775] }),32780,4) == 2147680257) by (compute);
        zero_blocks(z,32768,32768,29,8191,13);
        assert(m::Inflate(z,32771,true,false).status == m::InflateStatus::Done);
        assert(m::Inflate(z,32771,true,false).out == zeros(32771));
        zero_blocks(z,32768,32768,29,8191,13);
        assert(m::Inflate(z,32771,false,false).status == m::InflateStatus::Done);
        assert(m::Inflate(z,32771,false,false).out == zeros(32771));
        assert(crate::limits::SEGMENT_BYTES_MAX >= 32784) by (compute);
    }
    let mut output=[0xa5u8;32771];
    let ghost raw=zeros(32771); check_options(&wire,&mut output,true,Status::Done,Ghost(raw));
    let ghost raw=zeros(32771); check_options(&wire,&mut output,false,Status::Done,Ghost(raw));
}
} // verus!
