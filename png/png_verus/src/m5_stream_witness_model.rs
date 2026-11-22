//! Checked fixed-token witnesses at limits at least the authored raw length.
use inflate_verus::model as m;
use vstd::prelude::*;
use crate::m4_fixed_witness_model::{fixed_decode,fixed_dist_decode};
use crate::m4_trace_witness_model::{Step,valid_step,copy_quant,copy_prefix,width,trace};
verus! {
pub(crate) proof fn step_effect_at(z: Seq<u8>, raw: Seq<u8>, t: Step, last: bool,limit: nat)
    requires raw.len()<=limit,valid_step(z,raw,t,last),
    ensures m::MCompressedStep(z,t.pos,raw.take(t.before as int),limit,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) ==
        if last {m::SymbolEffect::End {out:raw,position:t.next}}
        else {m::SymbolEffect::Next {out:raw.take(t.after as int),position:t.next}},
{
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::copy_distance);
    assert(inflate_verus::limits::FIXED_LIT_COUNT == 288) by (compute);
    assert(inflate_verus::limits::FIXED_DIST_COUNT == 32) by (compute);
    assert(inflate_verus::limits::WINDOW_MAX == 32768) by (compute);
    assert(inflate_verus::limits::LEN_MAX == 258) by (compute);
    fixed_decode(z,t.pos,t.symbol as int);
    if t.symbol < 256 {
        assert(raw.take(t.before as int).push(raw[t.before as int]) =~= raw.take(t.after as int));
    } else if t.symbol == 256 {
        assert(raw.take(t.before as int) =~= raw);
    } else {
        let dp = t.pos+width(t.symbol)+m::length_extra(t.symbol);
        fixed_dist_decode(z,dp,t.ds as int);
        copy_quant(raw,t.before,t.distance,0,t.length);
        copy_prefix(raw,t.before,t.distance,t.length);
        assert(m::MCopy(raw.take(t.before as int),limit,32768,t.distance,t.length) ==
            (m::OutputEffect::Next {out:raw.take(t.after as int)}));
    }
    reveal(m::MCompressedStep);
}
pub(crate) proof fn effects_at(z: Seq<u8>, raw: Seq<u8>, ts: Seq<Step>, a: nat, b: nat,limit: nat)
    requires raw.len()<=limit,a <= b <= ts.len(), trace(z,raw,ts,a,b),
    ensures forall|j:int| a <= j < b ==> valid_step(z,raw,#[trigger] ts[j],j+1 == ts.len()),
        forall|j:int| a <= j < b && j+1 < ts.len() ==> ts[j+1].pos == ts[j].next && ts[j+1].before == ts[j].after,
        forall|j:int| a <= j < b ==> #[trigger] m::MCompressedStep(z,ts[j].pos,raw.take(ts[j].before as int),limit,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) ==
            if j+1 == ts.len() {m::SymbolEffect::End {out:raw,position:ts[j].next}}
            else {m::SymbolEffect::Next {out:raw.take(ts[j].after as int),position:ts[j].next}},
    decreases b-a,
{
    if a < b {
        if b == a+1 {step_effect_at(z,raw,ts[a as int],a+1 == ts.len(),limit);}
        else {
            let mid = (a+(b-a)/2) as nat;
            effects_at(z,raw,ts,a,mid,limit); effects_at(z,raw,ts,mid,b,limit);
        }
    }
}
pub(crate) proof fn expand_at(z: Seq<u8>, raw: Seq<u8>, ts: Seq<Step>, k: nat,limit: nat)
    requires raw.len()<=limit,k < ts.len(),
        forall|j:int| k <= j < ts.len() ==> valid_step(z,raw,#[trigger] ts[j],j+1 == ts.len()),
        forall|j:int| k <= j < ts.len() && j+1 < ts.len() ==> ts[j+1].pos == ts[j].next && ts[j+1].before == ts[j].after,
        forall|j:int| k <= j < ts.len() ==> #[trigger] m::MCompressedStep(z,ts[j].pos,raw.take(ts[j].before as int),limit,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) ==
            if j+1 == ts.len() {m::SymbolEffect::End {out:raw,position:ts[j].next}}
            else {m::SymbolEffect::Next {out:raw.take(ts[j].after as int),position:ts[j].next}},
    ensures m::MCompressed(z,ts[k as int].pos,raw.take(ts[k as int].before as int),limit,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) ==
        (m::StoredEffect::End {out:raw,position:ts.last().next}),
    decreases ts.len()-k,
{
    hide(m::MCompressedStep);
    if k+1 < ts.len() {expand_at(z,raw,ts,k+1,limit);}
    reveal_with_fuel(m::MCompressed,2);
}

pub(crate) proof fn fixed_stream(z: Seq<u8>,raw: Seq<u8>,ts: Seq<Step>,limit: nat,flg: nat,checksum: nat,wire: nat)
    requires raw.len()<=limit,z.len()>=3,z[0]==120,z[1]==flg,flg<256,(120*256+flg)%31==0,(flg/32)%2==0,
        ts.len()>0,ts[0].pos==19,ts[0].before==0,
        crate::m4_trace_witness_model::trace(z,raw,ts,0,ts.len()),
        m::field_value(z,16,1)==1,m::field_value(z,17,2)==1,
        (ts.last().next+7)/8+4==z.len(),m::be_value(z,((ts.last().next+7)/8) as nat,4)==wire,m::Adler(raw)==checksum,
    ensures m::Inflate(z,limit,true,false)==(m::Inflated {status:if wire==checksum {m::InflateStatus::Done}else{m::InflateStatus::Error(m::InflateFault::AdlerMismatch)},out:raw,r2:false}),
{
    hide(m::MDec);hide(m::decode_prefix);hide(m::cw);hide(m::MCompressed);hide(m::MCompressedStep);hide(m::MFinish);hide(m::MBlk);hide(m::MOneBlock);
    hide(m::sum_bytes);hide(m::sum_prefixes);
    assert(inflate_verus::limits::WINDOW_MAX==32768) by (compute);assert(m::pow2(15)==32768) by (compute);
    effects_at(z,raw,ts,0,ts.len(),limit);expand_at(z,raw,ts,0,limit);
    assert(raw.take(0)=~=Seq::<u8>::empty());
    let block=m::StoredEffect::End {out:raw,position:ts.last().next};
    let finished=if wire==checksum {m::StoredEffect::End {out:raw,position:8*z.len()}}else{m::StoredEffect::Error {error:m::InflateFault::AdlerMismatch,out:raw,position:8*z.len()}};
    assert(m::MCompressed(z,19,Seq::empty(),limit,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths())==block);
    assert(m::MFinish(z,ts.last().next,raw,limit,false)==finished) by {reveal(m::MFinish);}
    assert(m::MOneBlock(z,16,Seq::empty(),limit,32768,true)==(true,block)) by {reveal(m::MOneBlock);}
    assert(m::MBlk(z,16,Seq::empty(),limit,32768,true)==block) by {reveal_with_fuel(m::MBlk,2);}
    assert(m::MEnvelope(z,limit,true,false)==finished) by {reveal(m::MEnvelope);}
    reveal(m::Inflate);
}
} // verus!
