//! Authored fixed-step effects at independently selected limits and windows.
use crate::{model as m, witness_match as t};
use vstd::prelude::*;
verus! {
pub(crate) open spec fn step(raw: Seq<u8>, s: t::Step, limit: nat, window: nat) -> m::SymbolEffect {
    if s.symbol == 256 {m::SymbolEffect::End {out:raw,position:s.next}}
    else if s.symbol > 256 && s.distance > window {
        m::SymbolEffect::Error {error:m::InflateFault::DistanceTooFar,out:raw.take(s.before as int),position:s.next}
    } else if s.after > limit {m::SymbolEffect::Full {out:raw.take(limit as int)}}
    else {m::SymbolEffect::Next {out:raw.take(s.after as int),position:s.next}}
}
proof fn effect(z: Seq<u8>, raw: Seq<u8>, s: t::Step, last: bool, limit: nat, window: nat)
    requires t::valid_step(z,raw,s,last), s.before <= limit, 1 <= window <= 32768,
    ensures m::MCompressedStep(z,s.pos,raw.take(s.before as int),limit,window,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == step(raw,s,limit,window),
{
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::copy_distance);
    assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute);
    assert(crate::limits::FIXED_DIST_COUNT == 32) by (compute);
    assert(crate::limits::WINDOW_MAX == 32768) by (compute);
    assert(crate::limits::LEN_MAX == 258) by (compute);
    crate::witnesses::fixed_decode(z,s.pos,s.symbol as int);
    if s.symbol < 256 {
        assert(raw.take(s.before as int).push(raw[s.before as int]) =~= raw.take(s.after as int));
        if s.before == limit {assert(raw.take(s.before as int) == raw.take(limit as int));}
    } else if s.symbol == 256 {
        assert(raw.take(s.before as int) =~= raw);
    } else {
        crate::witnesses::fixed_dist_decode(z,s.pos+t::width(s.symbol)+m::length_extra(s.symbol),s.ds as int);
        if s.distance <= window {
            let fit = if s.after <= limit {s.length} else {(limit-s.before) as nat};
            t::copy_quant(raw,s.before,s.distance,0,s.length);
            t::copy_prefix(raw,s.before,s.distance,fit);
            assert(m::copy_distance(raw.take(s.before as int),s.distance,fit) == raw.take((s.before+fit) as int));
        }
    }
    reveal(m::MCompressedStep);
}
pub(crate) proof fn effects(z: Seq<u8>, raw: Seq<u8>, ts: Seq<t::Step>, a: nat, b: nat, limit: nat, window: nat)
    requires a <= b <= ts.len(), t::trace(z,raw,ts,a,b), 1 <= window <= 32768,
    ensures forall|j:int| a <= j < b ==> t::valid_step(z,raw,#[trigger] ts[j],j+1 == ts.len()),
        forall|j:int| a <= j < b && j+1 < ts.len() ==> ts[j+1].pos == ts[j].next && ts[j+1].before == ts[j].after,
        forall|j:int| a <= j < b && ts[j].before <= limit ==> #[trigger] m::MCompressedStep(z,ts[j].pos,raw.take(ts[j].before as int),limit,window,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == step(raw,ts[j],limit,window),
    decreases b-a,
{
    if a < b {
        if b == a+1 {if ts[a as int].before <= limit {effect(z,raw,ts[a as int],a+1 == ts.len(),limit,window);}}
        else {let mid = (a+(b-a)/2) as nat; effects(z,raw,ts,a,mid,limit,window); effects(z,raw,ts,mid,b,limit,window);}
    }
}
pub(crate) open spec fn result(raw: Seq<u8>, ts: Seq<t::Step>, k: nat, limit: nat, window: nat) -> m::StoredEffect
    decreases ts.len()-k,
{
    if k >= ts.len() {m::StoredEffect::OutsideDomain}
    else {match step(raw,ts[k as int],limit,window) {
        m::SymbolEffect::Next {..} => result(raw,ts,k+1,limit,window),
        m::SymbolEffect::End {out,position} => m::StoredEffect::End {out,position},
        m::SymbolEffect::Full {out} => m::StoredEffect::Full {out},
        m::SymbolEffect::Error {error,out,position} => m::StoredEffect::Error {error,out,position},
        _ => m::StoredEffect::OutsideDomain,
    }}
}
pub(crate) proof fn expand(z: Seq<u8>, raw: Seq<u8>, ts: Seq<t::Step>, k: nat, limit: nat, window: nat)
    requires k < ts.len(), ts[k as int].before <= limit, 1 <= window <= 32768,
        forall|j:int| k <= j < ts.len() ==> t::valid_step(z,raw,#[trigger] ts[j],j+1 == ts.len()),
        forall|j:int| k <= j < ts.len() && j+1 < ts.len() ==> ts[j+1].pos == ts[j].next && ts[j+1].before == ts[j].after,
        forall|j:int| k <= j < ts.len() && ts[j].before <= limit ==> #[trigger] m::MCompressedStep(z,ts[j].pos,raw.take(ts[j].before as int),limit,window,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == step(raw,ts[j],limit,window),
    ensures m::MCompressed(z,ts[k as int].pos,raw.take(ts[k as int].before as int),limit,window,m::fixed_lit_lengths(),m::fixed_dist_lengths()) == result(raw,ts,k,limit,window),
    decreases ts.len()-k,
{
    hide(m::MCompressedStep);
    if step(raw,ts[k as int],limit,window) is Next {expand(z,raw,ts,k+1,limit,window);}
    reveal_with_fuel(m::MCompressed,2);
}
pub(crate) open spec fn walk(ts: Seq<t::Step>, a: nat, b: nat, limit: nat, window: nat) -> bool
    decreases b-a,
{
    if a >= b {true} else if b == a+1 {
        ts[a as int].symbol != 256 && ts[a as int].after <= limit
        && (ts[a as int].symbol < 256 || ts[a as int].distance <= window)
    } else {let mid = (a+(b-a)/2) as nat; walk(ts,a,mid,limit,window) && walk(ts,mid,b,limit,window)}
}
pub(crate) proof fn walk_quant(ts: Seq<t::Step>, a: nat, b: nat, limit: nat, window: nat)
    requires a <= b <= ts.len(), walk(ts,a,b,limit,window),
    ensures forall|j:int| a <= j < b ==> ts[j].symbol != 256 && ts[j].after <= limit
        && (ts[j].symbol < 256 || ts[j].distance <= window),
    decreases b-a,
{
    if a < b && b != a+1 {let mid = (a+(b-a)/2) as nat; walk_quant(ts,a,mid,limit,window); walk_quant(ts,mid,b,limit,window);}
}
pub(crate) proof fn result_prefix(raw: Seq<u8>, ts: Seq<t::Step>, k: nat, b: nat, limit: nat, window: nat)
    requires k <= b < ts.len(),
        forall|j:int| k <= j < b ==> ts[j].symbol != 256 && ts[j].after <= limit
            && (ts[j].symbol < 256 || ts[j].distance <= window),
    ensures result(raw,ts,k,limit,window) == result(raw,ts,b,limit,window),
    decreases b-k,
{
    if k < b {result_prefix(raw,ts,k+1,b,limit,window);}
}
pub(crate) proof fn block(z: Seq<u8>, raw: Seq<u8>, ts: Seq<t::Step>, limit: nat, window: nat)
    requires z.len() >= 3, ts.len() > 0, ts[0].pos == 19, ts[0].before == 0,
        t::trace(z,raw,ts,0,ts.len()), 1 <= window <= 32768,
        m::field_value(z,16,1) == 1, m::field_value(z,17,2) == 1,
        match result(raw,ts,0,limit,window) {m::StoredEffect::End {position,..} => 16 < position <= 8*z.len(), _ => true},
    ensures m::MBlk(z,16,Seq::empty(),limit,window,true) == result(raw,ts,0,limit,window),
{
    hide(m::MCompressedStep); hide(m::MCompressed); hide(m::MDec); hide(m::decode_prefix); hide(m::cw);
    effects(z,raw,ts,0,ts.len(),limit,window); expand(z,raw,ts,0,limit,window);
    assert(raw.take(0) =~= Seq::<u8>::empty());
    assert(crate::limits::WINDOW_MAX == 32768) by (compute);
}
pub(crate) proof fn envelope(z: Seq<u8>, limit: nat, enforce: bool, tail: bool, cmf: nat, flg: nat, window: nat, block_effect: m::StoredEffect, final_effect: m::StoredEffect)
    requires z.len() >= 2, z[0] == cmf, z[1] == flg,
        cmf%16 == 8, cmf/16 <= 7, (cmf*256+flg)%31 == 0, (flg/32)%2 == 0,
        m::window_size(cmf/16,enforce) == window,
        m::MBlk(z,16,Seq::empty(),limit,window,true) == block_effect,
        match block_effect {m::StoredEffect::End {out,position} => m::MFinish(z,position,out,limit,tail) == final_effect, _ => final_effect == block_effect},
    ensures m::MEnvelope(z,limit,enforce,tail) == final_effect,
{
    hide(m::MBlk); hide(m::MFinish);
}
} // verus!
