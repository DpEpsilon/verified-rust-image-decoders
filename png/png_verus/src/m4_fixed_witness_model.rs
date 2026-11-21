//! Fixed-code wire arithmetic adapted from M2 witness helpers and reverified here.
//! No private imported theorem is assumed or exposed; only public model lemmas are used.
use vstd::prelude::*;
use inflate_verus::model as m;
verus! {
pub(crate) open spec fn wire_word(z: Seq<u8>, start: nat, n: nat) -> Seq<bool>
    decreases n,
{
    if n == 0 { Seq::empty() }
    else { wire_word(z, start, (n - 1) as nat).push(m::bit(z, (start + n - 1) as nat)) }
}

pub(crate) proof fn follow_word(lengths: Seq<u8>, z: Seq<u8>, start: nat, s: int, n: nat)
    requires m::lengths_ok(lengths), m::Kraft(lengths) <= inflate_verus::limits::CODE_SPACE,
        0 <= s < lengths.len(), 0 < lengths[s], n <= lengths[s],
        start + lengths[s] <= 8 * z.len(),
        wire_word(z, start, lengths[s] as nat) == m::cw(lengths, s).unwrap(),
    ensures m::decode_prefix(lengths, z, start, wire_word(z, start, n))
        == (m::Decoded::Symbol { symbol: s as nat, position: (start + lengths[s]) as nat }),
    decreases lengths[s] - n,
{
    m::lemma_msb_bits_len(m::cw_value(lengths, s), lengths[s] as nat);
    // Every shorter wire prefix is a prefix of the authored canonical word.
    lemma_wire_prefix(z, start, n, lengths[s] as nat);
    if n == lengths[s] {
        m::lemma_decode_prefix_hit(lengths, z, start, wire_word(z, start, n), s);
    } else {
        let bits = wire_word(z, start, n);
        assert(m::prefix(bits, m::cw(lengths, s).unwrap()));
        assert(m::live(lengths, bits));
        assert forall|t: int| !m::exact(lengths, bits, t) by {
            if m::exact(lengths, bits, t) {
                m::lemma_msb_bits_len(m::cw_value(lengths, t), lengths[t] as nat);
                m::lemma_prefix_free(lengths, t, s);
                assert(false);
            }
        }
        m::lemma_decode_prefix_step(lengths, z, start, bits);
        follow_word(lengths, z, start, s, n + 1);
    }
}

pub(crate) proof fn lemma_wire_prefix(z: Seq<u8>, start: nat, a: nat, b: nat)
    requires a <= b,
    ensures wire_word(z, start, a) == wire_word(z, start, b).take(a as int),
        wire_word(z, start, b).len() == b,
    decreases b,
{
    if b > 0 {
        if a < b { lemma_wire_prefix(z, start, a, (b - 1) as nat); }
        else { lemma_wire_prefix(z, start, (b - 1) as nat, (b - 1) as nat); }
        assert(wire_word(z, start, b).len() == b);
        assert(wire_word(z, start, a) =~= wire_word(z, start, b).take(a as int));
    }
}

spec fn fixed_count(n: nat, k: nat) -> nat {
    if n == 7 { if k <= 256 { 0 } else if k <= 280 { (k - 256) as nat } else { 24 } }
    else if n == 8 { if k <= 144 { k } else if k <= 280 { 144 } else { (k - 136) as nat } }
    else if n == 9 { if k <= 144 { 0 } else if k <= 256 { (k - 144) as nat } else { 112 } }
    else { 0 }
}

proof fn fixed_counts(n: nat, k: nat)
    requires k <= inflate_verus::limits::FIXED_LIT_COUNT,
    ensures m::count_prefix(m::fixed_lit_lengths(), n, k) == fixed_count(n, k),
    decreases k,
{
    if k > 0 { fixed_counts(n, (k - 1) as nat); }
}

proof fn fixed_next(n: nat)
    requires n <= 9,
    ensures m::next_code(m::fixed_lit_lengths(), n)
        == if n <= 7 { 0nat } else if n == 8 { 48nat } else { 400nat },
    decreases n,
{
    if n > 0 {
        fixed_next((n - 1) as nat);
        fixed_counts((n - 1) as nat, inflate_verus::limits::FIXED_LIT_COUNT as nat);
    }
}

pub(crate) proof fn fixed_word(s: int)
    requires 0 <= s < inflate_verus::limits::FIXED_LIT_COUNT,
    ensures m::cw(m::fixed_lit_lengths(), s) == Some(m::msb_bits(
        if s < 144 { (48 + s) as nat } else if s < 256 { (400 + s - 144) as nat }
        else if s < 280 { (s - 256) as nat } else { (192 + s - 280) as nat },
        m::fixed_lit_lengths()[s] as nat)),
{
    fixed_next(m::fixed_lit_lengths()[s] as nat);
    fixed_counts(m::fixed_lit_lengths()[s] as nat, s as nat);
}

pub(crate) proof fn fixed_decode(z: Seq<u8>, start: nat, s: int)
    requires 0 <= s < inflate_verus::limits::FIXED_LIT_COUNT,
        start + m::fixed_lit_lengths()[s] <= 8 * z.len(),
        wire_word(z, start, m::fixed_lit_lengths()[s] as nat) == m::msb_bits(
            if s < 144 { (48 + s) as nat } else if s < 256 { (400 + s - 144) as nat }
            else if s < 280 { (s - 256) as nat } else { (192 + s - 280) as nat },
            m::fixed_lit_lengths()[s] as nat),
    ensures m::MDec(m::fixed_lit_lengths(), z, start)
        == (m::Decoded::Symbol { symbol: s as nat, position: (start + m::fixed_lit_lengths()[s]) as nat }),
{
    m::lemma_fixed_codes(true);
    fixed_word(s);
    follow_word(m::fixed_lit_lengths(), z, start, s, 0);
}


pub(crate) proof fn end(z: Seq<u8>,p: nat,out: Seq<u8>,limit: nat,after: nat)
    requires p<=8*z.len(),out.len()<=limit,
        m::MDec(m::fixed_lit_lengths(),z,p)==(m::Decoded::Symbol {symbol:256,position:after}),
    ensures m::MCompressed(z,p,out,limit,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths())
        ==(m::StoredEffect::End {out,position:after}),
{
    hide(m::MDec);hide(m::decode_prefix);hide(m::cw);
    assert(inflate_verus::limits::WINDOW_MAX==32768) by (compute);
    reveal(m::MCompressedStep);reveal_with_fuel(m::MCompressed,2);
}
pub(crate) proof fn full(z: Seq<u8>,p: nat,out: Seq<u8>,limit: nat,byte: u8,after: nat)
    requires p<=8*z.len(),out.len()==limit,
        m::MDec(m::fixed_lit_lengths(),z,p)==(m::Decoded::Symbol {symbol:byte as nat,position:after}),
    ensures m::MCompressed(z,p,out,limit,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths())
        ==(m::StoredEffect::Full {out}),
{
    hide(m::MDec);hide(m::decode_prefix);hide(m::cw);
    assert(inflate_verus::limits::WINDOW_MAX==32768) by (compute);
    reveal(m::MCompressedStep);reveal_with_fuel(m::MCompressed,2);
}
pub(crate) proof fn literal_step(z: Seq<u8>,p: nat,out: Seq<u8>,byte: u8,limit: nat,after: nat,terminal: m::StoredEffect)
    requires out.len()<limit,p<after<=8*z.len(),
        m::MDec(m::fixed_lit_lengths(),z,p)==(m::Decoded::Symbol {symbol:byte as nat,position:after}),
        m::MCompressed(z,after,out.push(byte),limit,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths())==terminal,
    ensures m::MCompressed(z,p,out,limit,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths())==terminal,
{
    hide(m::MDec);hide(m::decode_prefix);hide(m::cw);
    assert(inflate_verus::limits::WINDOW_MAX==32768) by (compute);
    reveal(m::MCompressedStep);reveal_with_fuel(m::MCompressed,2);
}
pub(crate) proof fn envelope(z: Seq<u8>,limit: nat,effect: m::StoredEffect,finished: m::StoredEffect)
    requires z.len()>=3,z[0]==120u8,z[1]==156u8,
        m::field_value(z,16,1)==1,m::field_value(z,17,2)==1,
        m::MCompressed(z,19,Seq::empty(),limit,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths())==effect,
        match effect {
            m::StoredEffect::End {out,position}=>16<position<=8*z.len() && m::MFinish(z,position,out,limit,false)==finished,
            m::StoredEffect::Full {..}=>finished==effect,
            _=>false,
        },
    ensures m::MEnvelope(z,limit,true,false)==finished,
{
    hide(m::MCompressed);hide(m::MDec);hide(m::decode_prefix);hide(m::cw);hide(m::MFinish);hide(m::MBlk);hide(m::MOneBlock);
    assert(m::pow2(15)==32768) by (compute);
    assert(inflate_verus::limits::WINDOW_MAX==32768) by (compute);
    assert(m::MOneBlock(z,16,Seq::empty(),limit,32768,true)==(true,effect)) by {reveal(m::MOneBlock);}
    assert(m::MBlk(z,16,Seq::empty(),limit,32768,true)==effect) by {reveal_with_fuel(m::MBlk,2);}
    reveal(m::MEnvelope);
}

proof fn dist_counts(n: nat, k: nat)
    requires k <= inflate_verus::limits::FIXED_DIST_COUNT,
    ensures m::count_prefix(m::fixed_dist_lengths(), n, k) == if n == 5 { k } else { 0nat },
    decreases k,
{
    if k > 0 { dist_counts(n, (k - 1) as nat); }
}

proof fn dist_next(n: nat)
    requires n <= 5,
    ensures m::next_code(m::fixed_dist_lengths(), n) == 0,
    decreases n,
{
    if n > 0 { dist_next((n - 1) as nat); dist_counts((n - 1) as nat, inflate_verus::limits::FIXED_DIST_COUNT as nat); }
}

pub(crate) proof fn fixed_dist_decode(z: Seq<u8>, start: nat, s: int)
    requires 0 <= s < inflate_verus::limits::FIXED_DIST_COUNT, start + 5 <= 8 * z.len(),
        wire_word(z, start, 5) == m::msb_bits(s as nat, 5),
    ensures m::MDec(m::fixed_dist_lengths(), z, start)
        == (m::Decoded::Symbol { symbol: s as nat, position: start + 5 }),
{
    m::lemma_fixed_codes(true);
    dist_next(5); dist_counts(5, s as nat);
    assert(m::cw(m::fixed_dist_lengths(), s) == Some(m::msb_bits(s as nat, 5)));
    follow_word(m::fixed_dist_lengths(), z, start, s, 0);
}


} // verus!
