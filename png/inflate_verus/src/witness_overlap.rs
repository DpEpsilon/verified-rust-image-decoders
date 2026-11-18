//! Authored large fixed-overlap record, using sequence rather than ring output.
use crate::{
    model as m,
    witnesses::{check_at, fixed_decode, fixed_dist_decode, wire_word},
    Status,
};
use vstd::prelude::*;
verus! {
pub(crate) open spec fn a_row(n: nat) -> Seq<u8> {
    Seq::new(n+1,|i:int| if i == 0 {0u8} else {65u8})
}
pub(crate) proof fn a_copy(n: nat, count: nat)
    requires n > 0,
    ensures m::copy_distance(a_row(n),1,count) == a_row(n+count),
    decreases count,
{
    if count > 0 {
        a_copy(n,(count-1) as nat);
        assert(a_row((n+count-1) as nat).push(65) =~= a_row(n+count));
    }
}
pub(crate) proof fn a_sums(n: nat)
    ensures m::sum_bytes(a_row(n)) == 65*n,
        2*m::sum_prefixes(a_row(n)) == 2*(n+1)+65*n*(n+1),
    decreases n,
{
    if n == 0 {
        assert(a_row(0) =~= seq![0u8]);
        assert(m::sum_bytes(seq![0u8]) == 0) by (compute);
        assert(m::sum_prefixes(seq![0u8]) == 1) by (compute);
    } else {
        a_sums((n-1) as nat);
        assert(a_row((n-1) as nat).push(65) =~= a_row(n));
        m::lemma_adler_sums_push(a_row((n-1) as nat),65);
        assert(m::sum_bytes(a_row(n)) == 65*n) by (nonlinear_arith)
            requires m::sum_bytes(a_row(n)) == m::sum_bytes(a_row((n-1) as nat))+65,
                m::sum_bytes(a_row((n-1) as nat)) == 65*(n-1);
        assert(2*m::sum_prefixes(a_row(n)) == 2*(n+1)+65*n*(n+1)) by (nonlinear_arith)
            requires m::sum_prefixes(a_row(n)) == m::sum_prefixes(a_row((n-1) as nat))+1+m::sum_bytes(a_row(n)),
                m::sum_bytes(a_row(n)) == 65*n,
                2*m::sum_prefixes(a_row((n-1) as nat)) == 2*n+65*(n-1)*n;
    }
}

/// deflate/fixed-overlapping-matches: 0 followed by 300 A bytes, R=301.
#[verifier::spinoff_prover]
fn fixed_overlapping_matches() {
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw);
    hide(m::MCompressedStep); hide(m::MCompressed); hide(m::copy_distance);
    let wire = [0x78u8,0x01,0x63,0x70,0x1c,0x05,0x48,0x00,0x3f,0x00,0x00,0xcb,0x9f,0x4c,0x2d];
    proof {
        let z=seq![0x78u8,0x01,0x63,0x70,0x1c,0x05,0x48,0x00,0x3f,0x00,0x00,0xcb,0x9f,0x4c,0x2d];
        assert(crate::limits::WINDOW_MAX == 32768) by (compute); assert(m::pow2(15) == 32768) by (compute);
        assert(crate::limits::LEN_MAX == 258) by (compute);
        assert(wire_word(seq![0x78u8,0x01,0x63,0x70,0x1c,0x05,0x48,0x00,0x3f,0x00,0x00,0xcb,0x9f,0x4c,0x2d],19,8) == m::msb_bits(48,8)) by (compute); fixed_decode(z,19,0);
        assert(wire_word(seq![0x78u8,0x01,0x63,0x70,0x1c,0x05,0x48,0x00,0x3f,0x00,0x00,0xcb,0x9f,0x4c,0x2d],27,8) == m::msb_bits(113,8)) by (compute); fixed_decode(z,27,65);
        assert(wire_word(seq![0x78u8,0x01,0x63,0x70,0x1c,0x05,0x48,0x00,0x3f,0x00,0x00,0xcb,0x9f,0x4c,0x2d],35,8) == m::msb_bits(197,8)) by (compute); fixed_decode(z,35,285);
        assert(wire_word(seq![0x78u8,0x01,0x63,0x70,0x1c,0x05,0x48,0x00,0x3f,0x00,0x00,0xcb,0x9f,0x4c,0x2d],43,5) == m::msb_bits(0,5)) by (compute); fixed_dist_decode(z,43,0);
        assert(wire_word(seq![0x78u8,0x01,0x63,0x70,0x1c,0x05,0x48,0x00,0x3f,0x00,0x00,0xcb,0x9f,0x4c,0x2d],48,7) == m::msb_bits(9,7)) by (compute); fixed_decode(z,48,265);
        assert(m::field_value(seq![0x78u8,0x01,0x63,0x70,0x1c,0x05,0x48,0x00,0x3f,0x00,0x00,0xcb,0x9f,0x4c,0x2d],55,1) == 0) by (compute);
        assert(wire_word(seq![0x78u8,0x01,0x63,0x70,0x1c,0x05,0x48,0x00,0x3f,0x00,0x00,0xcb,0x9f,0x4c,0x2d],56,5) == m::msb_bits(0,5)) by (compute); fixed_dist_decode(z,56,0);
        assert(wire_word(seq![0x78u8,0x01,0x63,0x70,0x1c,0x05,0x48,0x00,0x3f,0x00,0x00,0xcb,0x9f,0x4c,0x2d],61,7) == m::msb_bits(15,7)) by (compute); fixed_decode(z,61,271);
        assert(m::field_value(seq![0x78u8,0x01,0x63,0x70,0x1c,0x05,0x48,0x00,0x3f,0x00,0x00,0xcb,0x9f,0x4c,0x2d],68,2) == 3) by (compute);
        assert(wire_word(seq![0x78u8,0x01,0x63,0x70,0x1c,0x05,0x48,0x00,0x3f,0x00,0x00,0xcb,0x9f,0x4c,0x2d],70,5) == m::msb_bits(0,5)) by (compute); fixed_dist_decode(z,70,0);
        assert(wire_word(seq![0x78u8,0x01,0x63,0x70,0x1c,0x05,0x48,0x00,0x3f,0x00,0x00,0xcb,0x9f,0x4c,0x2d],75,7) == m::msb_bits(0,7)) by (compute); fixed_decode(z,75,256);
        assert(seq![0u8].push(65) =~= a_row(1));
        assert(m::length_base(265) == 11) by (compute); assert(m::length_extra(265) == 1) by (compute);
        assert(m::length_base(271) == 27) by (compute); assert(m::length_extra(271) == 2) by (compute);
        a_copy(1,258); a_copy(259,11); a_copy(270,30);
        assert(m::MCopy(a_row(1),301,32768,1,258) == (m::OutputEffect::Next {out:a_row(259)}));
        assert(m::MCopy(a_row(259),301,32768,1,11) == (m::OutputEffect::Next {out:a_row(270)}));
        assert(m::MCopy(a_row(270),301,32768,1,30) == (m::OutputEffect::Next {out:a_row(300)}));
        assert(m::MCompressedStep(z,19,Seq::empty(),301,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::SymbolEffect::Next {out:seq![0u8],position:27})) by {reveal(m::MCompressedStep);}
        assert(m::MCompressedStep(z,27,seq![0u8],301,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::SymbolEffect::Next {out:a_row(1),position:35})) by {reveal(m::MCompressedStep);}
        assert(m::MCompressedStep(z,35,a_row(1),301,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::SymbolEffect::Next {out:a_row(259),position:48})) by {reveal(m::MCompressedStep);}
        assert(m::MCompressedStep(z,48,a_row(259),301,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::SymbolEffect::Next {out:a_row(270),position:61})) by {reveal(m::MCompressedStep);}
        assert(m::MCompressedStep(z,61,a_row(270),301,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::SymbolEffect::Next {out:a_row(300),position:75})) by {reveal(m::MCompressedStep);}
        assert(m::MCompressedStep(z,75,a_row(300),301,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::SymbolEffect::End {out:a_row(300),position:82})) by {reveal(m::MCompressedStep);}
        assert(m::MCompressed(z,19,Seq::empty(),301,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == (m::StoredEffect::End {out:a_row(300),position:82})) by {reveal_with_fuel(m::MCompressed,7);}
        assert(m::field_value(seq![0x78u8,0x01,0x63,0x70,0x1c,0x05,0x48,0x00,0x3f,0x00,0x00,0xcb,0x9f,0x4c,0x2d],16,1) == 1) by (compute);
        assert(m::field_value(seq![0x78u8,0x01,0x63,0x70,0x1c,0x05,0x48,0x00,0x3f,0x00,0x00,0xcb,0x9f,0x4c,0x2d],17,2) == 1) by (compute);
        assert(m::MBlk(z,16,Seq::empty(),301,32768,true) == (m::StoredEffect::End {out:a_row(300),position:82}));
        a_sums(300);
        assert(m::sum_bytes(a_row(300)) == 19500); assert(m::sum_prefixes(a_row(300)) == 2935051);
        assert(crate::limits::ADLER_MOD == 65521) by (compute); assert(m::pow2(16) == u16::MAX + 1) by (compute);
        assert(m::be_value(seq![0x78u8,0x01,0x63,0x70,0x1c,0x05,0x48,0x00,0x3f,0x00,0x00,0xcb,0x9f,0x4c,0x2d],11,4) == 3416214573) by (compute);
        assert(m::Adler(a_row(300)) == 3416214573);
        assert(m::Inflate(z,301,true,false).status == m::InflateStatus::Done);
        assert(m::Inflate(z,301,true,false).out == a_row(300));
        assert(crate::limits::SEGMENT_BYTES_MAX >= 15) by (compute);
    }
    let mut output=[0xa5u8;301]; let ghost raw=a_row(300);
    check_at(&wire,&mut output,Status::Done,Ghost(raw));
}
} // verus!
