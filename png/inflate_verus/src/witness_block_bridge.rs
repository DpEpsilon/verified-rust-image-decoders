//! Authored live-prefix EOF and a verified prefix-to-terminal composition.
use crate::{model as m, witness_match as t, witness_config as g, witnesses::wire_word};
use vstd::prelude::*;
verus! {
pub(crate) proof fn live_eof(lengths: Seq<u8>, z: Seq<u8>, start: nat, s: int, n: nat, available: nat)
    requires m::lengths_ok(lengths), m::Kraft(lengths) <= crate::limits::CODE_SPACE,
        0 <= s < lengths.len(), n <= available < lengths[s], start+available == 8*z.len(),
        wire_word(z,start,available) == m::cw(lengths,s).unwrap().take(available as int),
    ensures m::decode_prefix(lengths,z,start,wire_word(z,start,n)) ==
        (m::Decoded::TruncatedZlib {position:8*z.len()}),
    decreases available-n,
{
    m::lemma_msb_bits_len(m::cw_value(lengths,s),lengths[s] as nat);
    crate::witnesses::lemma_wire_prefix(z,start,n,available);
    let bits = wire_word(z,start,n);
    assert(m::prefix(bits,m::cw(lengths,s).unwrap()));
    assert(m::live(lengths,bits));
    assert forall|v:int| !m::exact(lengths,bits,v) by {
        if m::exact(lengths,bits,v) {
            m::lemma_msb_bits_len(m::cw_value(lengths,v),lengths[v] as nat);
            m::lemma_prefix_free(lengths,v,s);
            assert(false);
        }
    }
    if n == available {m::lemma_decode_prefix_errors(lengths,z,start,bits);}
    else {
        m::lemma_decode_prefix_step(lengths,z,start,bits);
        live_eof(lengths,z,start,s,n+1,available);
    }
}
pub(crate) proof fn prefix(z: Seq<u8>, raw: Seq<u8>, ts: Seq<t::Step>, k: nat, b: nat, limit: nat, terminal: m::StoredEffect)
    requires raw.len() <= limit, k <= b < ts.len(),
        forall|j:int| k <= j < b ==> t::valid_step(z,raw,#[trigger] ts[j],false),
        forall|j:int| k <= j < b ==> ts[j+1].pos == ts[j].next && ts[j+1].before == ts[j].after,
        forall|j:int| k <= j < b ==> #[trigger] m::MCompressedStep(z,ts[j].pos,raw.take(ts[j].before as int),limit,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == g::step(raw,ts[j],limit,32768),
        m::MCompressed(z,ts[b as int].pos,raw.take(ts[b as int].before as int),limit,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == terminal,
    ensures m::MCompressed(z,ts[k as int].pos,raw.take(ts[k as int].before as int),limit,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == terminal,
    decreases b-k,
{
    hide(m::MCompressedStep);
    if k < b {prefix(z,raw,ts,k+1,b,limit,terminal); reveal_with_fuel(m::MCompressed,2);}
}
pub(crate) proof fn stored(z: Seq<u8>, p: nat, out: Seq<u8>, payload: Seq<u8>, limit: nat)
    requires out.len()+payload.len() <= limit, payload.len() <= u16::MAX,
        (p+3+7)/8+4+payload.len() <= z.len(),
        m::field_value(z,p+1,2) == 0,
        z[((p+3+7)/8) as int] as nat+256*z[((p+3+7)/8+1) as int] as nat == payload.len(),
        z[((p+3+7)/8+2) as int] as nat+256*z[((p+3+7)/8+3) as int] as nat == u16::MAX-payload.len(),
        z.subrange(((p+3+7)/8+4) as int,((p+3+7)/8+4+payload.len()) as int) == payload,
    ensures m::MStoredBlock(z,p+3,out,limit) == (m::StoredEffect::End {out:out+payload,position:8*((p+3+7)/8+4+payload.len())}),
{
    hide(m::MStoredPayload);
    let h = (p+3+7)/8;
    m::lemma_stored_payload(z,h+4,out,limit,payload.len());
    m::lemma_field_bound(z,8*h,16); m::lemma_field_bound(z,8*h+16,16);
    assert(m::pow2(16) == u16::MAX+1) by (compute);
}
pub(crate) proof fn fixed_first(z: Seq<u8>, raw: Seq<u8>, ts: Seq<t::Step>, limit: nat)
    requires raw.len() <= limit, z.len() >= 3, ts.len() > 0,
        ts[0].pos == 19, ts[0].before == 0, t::trace(z,raw,ts,0,ts.len()),
        m::field_value(z,16,1) == 0, m::field_value(z,17,2) == 1,
    ensures m::MOneBlock(z,16,Seq::empty(),limit,32768,true) == (false,m::StoredEffect::End {out:raw,position:ts.last().next}),
{
    hide(m::MCompressedStep); hide(m::MCompressed); hide(m::MDec); hide(m::decode_prefix); hide(m::cw);
    t::effects_at(z,raw,ts,0,ts.len(),limit); t::expand_at(z,raw,ts,0,limit);
    assert(raw.take(0) =~= Seq::<u8>::empty());
    assert(crate::limits::WINDOW_MAX == 32768) by (compute);
}
pub(crate) proof fn truncated_block(z: Seq<u8>, raw: Seq<u8>, ts: Seq<t::Step>, b: nat, limit: nat, terminal: m::StoredEffect)
    requires raw.len() <= limit, z.len() >= 3, b < ts.len(), ts[0].pos == 19, ts[0].before == 0,
        t::trace(z,raw,ts,0,b), terminal is Error,
        m::field_value(z,16,1) == 1, m::field_value(z,17,2) == 1,
        m::MCompressed(z,ts[b as int].pos,raw.take(ts[b as int].before as int),limit,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == terminal,
    ensures m::MBlk(z,16,Seq::empty(),limit,32768,true) == terminal,
{
    hide(m::MCompressedStep); hide(m::MCompressed); hide(m::MDec); hide(m::decode_prefix); hide(m::cw);
    g::effects(z,raw,ts,0,b,limit,32768); prefix(z,raw,ts,0,b,limit,terminal);
    assert(raw.take(0) =~= Seq::<u8>::empty());
    assert(crate::limits::WINDOW_MAX == 32768) by (compute);
}
} // verus!
