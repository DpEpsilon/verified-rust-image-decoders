//! Independent fixed-literal stream classes with balanced authored wire checks.
use crate::{
    model as m,
    witnesses::{fixed_decode, wire_word},
};
use vstd::prelude::*;
verus! {
pub(crate) open spec fn literal_width(s: nat) -> nat { if s < 144 { 8 } else if s < 256 { 9 } else { 7 } }
pub(crate) open spec fn literal_word(s: nat) -> nat { if s < 144 { 48+s } else if s < 256 { 256+s } else { 0 } }
pub(crate) open spec fn literal_words(z: Seq<u8>, raw: Seq<u8>, positions: Seq<nat>, a: nat, b: nat) -> bool
    decreases b-a,
{
    if a >= b { true } else if b == a+1 {
        let s = if a < raw.len() { raw[a as int] as nat } else { 256nat };
        positions[(a+1) as int] == positions[a as int]+literal_width(s)
        && positions[(a+1) as int] <= 8*z.len()
        && wire_word(z,positions[a as int],literal_width(s)) == m::msb_bits(literal_word(s),literal_width(s))
    } else {
        let mid = (a+(b-a)/2) as nat;
        literal_words(z,raw,positions,a,mid) && literal_words(z,raw,positions,mid,b)
    }
}
pub(crate) proof fn literal_decodes(z: Seq<u8>, raw: Seq<u8>, positions: Seq<nat>, a: nat, b: nat)
    requires a <= b <= raw.len()+1, positions.len() == raw.len()+2,
        literal_words(z,raw,positions,a,b),
    ensures forall|j:int| a <= j < b ==> #[trigger] m::MDec(m::fixed_lit_lengths(),z,positions[j]) ==
        (m::Decoded::Symbol { symbol: if j < raw.len() { raw[j] as nat } else { 256nat }, position: positions[j+1] }),
        forall|j:int| a <= j < b ==> #[trigger] positions[j] < positions[j+1] <= 8*z.len(),
    decreases b-a,
{
    if a < b {
        if b == a+1 {
            let s = if a < raw.len() { raw[a as int] as int } else { 256int };
            assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute);
            fixed_decode(z,positions[a as int],s);
        } else {
            let mid = (a+(b-a)/2) as nat;
            literal_decodes(z,raw,positions,a,mid); literal_decodes(z,raw,positions,mid,b);
        }
    }
}
pub(crate) proof fn literal_expand(z: Seq<u8>, raw: Seq<u8>, positions: Seq<nat>, k: nat)
    requires positions.len() == raw.len()+2, k <= raw.len(),
        forall|j:int| k <= j <= raw.len() ==> #[trigger] m::MDec(m::fixed_lit_lengths(),z,positions[j]) ==
            (m::Decoded::Symbol { symbol: if j < raw.len() { raw[j] as nat } else { 256nat }, position: positions[j+1] }),
        forall|j:int| k <= j <= raw.len() ==> #[trigger] positions[j] < positions[j+1] <= 8*z.len(),
    ensures m::MCompressed(z,positions[k as int],raw.take(k as int),raw.len(),32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) ==
        (m::StoredEffect::End { out: raw, position: positions[(raw.len()+1) as int] }),
    decreases raw.len()-k,
{
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw);
    assert(crate::limits::WINDOW_MAX == 32768) by (compute);
    m::lemma_fixed_codes(true);
    if k < raw.len() {
        literal_expand(z,raw,positions,k+1);
        assert(raw.take(k as int).push(raw[k as int]) =~= raw.take((k+1) as int));
    } else { assert(raw.take(k as int) =~= raw); }
    reveal(m::MCompressedStep); reveal_with_fuel(m::MCompressed,2);
}
// The model's D5 summary gives the complete stored payload without unfolding
// one verifier query per byte. Expected bytes remain a separate raw sequence.
pub(crate) proof fn stored_payload(z: Seq<u8>, raw: Seq<u8>)
    requires raw.len() <= u16::MAX, 7+raw.len() <= z.len(),
        z.subrange(7,(7+raw.len()) as int) == raw,
    ensures m::MStoredPayload(z,7,Seq::empty(),raw.len(),raw.len()) ==
        (m::StoredEffect::End { out: raw, position: 8*(7+raw.len()) }),
{
    hide(m::MStoredPayload);
    m::lemma_stored_payload(z,7,Seq::empty(),raw.len(),raw.len());
    assert(Seq::<u8>::empty()+raw =~= raw);
}
} // verus!
