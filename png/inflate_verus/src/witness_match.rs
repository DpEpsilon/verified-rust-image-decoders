//! Authored fixed-code words, extra fields and sequence-copy effects.
//! No executable Huffman table or measuring ring enters this bridge.
use crate::{model as m, witnesses::{fixed_decode, fixed_dist_decode, wire_word}};
use vstd::prelude::*;
verus! {
pub(crate) struct Step {
    pub pos: nat, pub next: nat, pub before: nat, pub after: nat,
    pub symbol: nat, pub le: nat, pub ds: nat, pub de: nat,
    pub length: nat, pub distance: nat,
}
pub(crate) open spec fn width(s: nat) -> nat {
    if s < 144 {8} else if s < 256 {9} else if s < 280 {7} else {8}
}
pub(crate) open spec fn word(s: nat) -> nat {
    if s < 144 {48+s} else if s < 256 {256+s} else if s < 280 {(s-256) as nat} else {(s-88) as nat}
}
pub(crate) open spec fn copy_bytes(raw: Seq<u8>, start: nat, distance: nat, a: nat, b: nat) -> bool
    decreases b-a,
{
    if a >= b {true} else if b == a+1 {
        raw[(start+a) as int] == raw[(start+a-distance) as int]
    } else {
        let mid = (a+(b-a)/2) as nat;
        copy_bytes(raw,start,distance,a,mid) && copy_bytes(raw,start,distance,mid,b)
    }
}
proof fn copy_quant(raw: Seq<u8>, start: nat, distance: nat, a: nat, b: nat)
    requires a <= b, distance <= start, start+b <= raw.len(), copy_bytes(raw,start,distance,a,b),
    ensures forall|j:int| a <= j < b ==> #[trigger] raw[start+j] == raw[start+j-distance],
    decreases b-a,
{
    if a < b && b != a+1 {
        let mid = (a+(b-a)/2) as nat;
        copy_quant(raw,start,distance,a,mid); copy_quant(raw,start,distance,mid,b);
    }
}
proof fn copy_prefix(raw: Seq<u8>, start: nat, distance: nat, n: nat)
    requires 0 < distance <= start, start+n <= raw.len(),
        forall|j:int| 0 <= j < n ==> #[trigger] raw[start+j] == raw[start+j-distance],
    ensures m::copy_distance(raw.take(start as int),distance,n) == raw.take((start+n) as int),
    decreases n,
{
    if n > 0 {
        copy_prefix(raw,start,distance,(n-1) as nat);
        assert(raw.take((start+n-1) as int).push(raw[(start+n-1) as int]) =~= raw.take((start+n) as int));
    }
}
pub(crate) open spec fn valid_step(z: Seq<u8>, raw: Seq<u8>, t: Step, last: bool) -> bool {
    t.before <= t.after <= raw.len() && t.pos < t.next <= 8*z.len()
    && t.symbol <= 285 && (last <==> t.symbol == 256)
    && wire_word(z,t.pos,width(t.symbol)) == m::msb_bits(word(t.symbol),width(t.symbol))
    && if t.symbol < 256 {
        t.next == t.pos+width(t.symbol) && t.after == t.before+1 && raw[t.before as int] == t.symbol
    } else if t.symbol == 256 {
        t.next == t.pos+7 && t.before == t.after == raw.len()
    } else {
        let lp = t.pos+width(t.symbol);
        let dp = lp+m::length_extra(t.symbol);
        let ep = dp+5;
        t.ds < 30 && t.length == m::length_base(t.symbol)+t.le
        && t.distance == m::distance_base(t.ds)+t.de
        && t.le < m::pow2(m::length_extra(t.symbol)) && t.de < m::pow2(m::distance_extra(t.ds))
        && m::field_value(z,lp,m::length_extra(t.symbol)) == t.le
        && wire_word(z,dp,5) == m::msb_bits(t.ds,5)
        && m::field_value(z,ep,m::distance_extra(t.ds)) == t.de
        && t.next == ep+m::distance_extra(t.ds)
        && 3 <= t.length <= 258 && 0 < t.distance <= t.before && t.distance <= 32768
        && t.after == t.before+t.length && copy_bytes(raw,t.before,t.distance,0,t.length)
    }
}
pub(crate) open spec fn trace(z: Seq<u8>, raw: Seq<u8>, ts: Seq<Step>, a: nat, b: nat) -> bool
    decreases b-a,
{
    if a >= b {true} else if b == a+1 {
        valid_step(z,raw,ts[a as int],a+1 == ts.len())
        && (a+1 < ts.len() ==> ts[(a+1) as int].pos == ts[a as int].next
            && ts[(a+1) as int].before == ts[a as int].after)
    } else {
        let mid = (a+(b-a)/2) as nat;
        trace(z,raw,ts,a,mid) && trace(z,raw,ts,mid,b)
    }
}
proof fn step_effect(z: Seq<u8>, raw: Seq<u8>, t: Step, last: bool)
    requires valid_step(z,raw,t,last),
    ensures m::MCompressedStep(z,t.pos,raw.take(t.before as int),raw.len(),32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) ==
        if last {m::SymbolEffect::End {out:raw,position:t.next}}
        else {m::SymbolEffect::Next {out:raw.take(t.after as int),position:t.next}},
{
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::copy_distance);
    assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute);
    assert(crate::limits::FIXED_DIST_COUNT == 32) by (compute);
    assert(crate::limits::WINDOW_MAX == 32768) by (compute);
    assert(crate::limits::LEN_MAX == 258) by (compute);
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
        assert(m::MCopy(raw.take(t.before as int),raw.len(),32768,t.distance,t.length) ==
            (m::OutputEffect::Next {out:raw.take(t.after as int)}));
    }
    reveal(m::MCompressedStep);
}
pub(crate) proof fn effects(z: Seq<u8>, raw: Seq<u8>, ts: Seq<Step>, a: nat, b: nat)
    requires a <= b <= ts.len(), trace(z,raw,ts,a,b),
    ensures forall|j:int| a <= j < b ==> valid_step(z,raw,#[trigger] ts[j],j+1 == ts.len()),
        forall|j:int| a <= j < b && j+1 < ts.len() ==> ts[j+1].pos == ts[j].next && ts[j+1].before == ts[j].after,
        forall|j:int| a <= j < b ==> #[trigger] m::MCompressedStep(z,ts[j].pos,raw.take(ts[j].before as int),raw.len(),32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) ==
            if j+1 == ts.len() {m::SymbolEffect::End {out:raw,position:ts[j].next}}
            else {m::SymbolEffect::Next {out:raw.take(ts[j].after as int),position:ts[j].next}},
    decreases b-a,
{
    if a < b {
        if b == a+1 {step_effect(z,raw,ts[a as int],a+1 == ts.len());}
        else {
            let mid = (a+(b-a)/2) as nat;
            effects(z,raw,ts,a,mid); effects(z,raw,ts,mid,b);
        }
    }
}
pub(crate) proof fn expand(z: Seq<u8>, raw: Seq<u8>, ts: Seq<Step>, k: nat)
    requires k < ts.len(),
        forall|j:int| k <= j < ts.len() ==> valid_step(z,raw,#[trigger] ts[j],j+1 == ts.len()),
        forall|j:int| k <= j < ts.len() && j+1 < ts.len() ==> ts[j+1].pos == ts[j].next && ts[j+1].before == ts[j].after,
        forall|j:int| k <= j < ts.len() ==> #[trigger] m::MCompressedStep(z,ts[j].pos,raw.take(ts[j].before as int),raw.len(),32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) ==
            if j+1 == ts.len() {m::SymbolEffect::End {out:raw,position:ts[j].next}}
            else {m::SymbolEffect::Next {out:raw.take(ts[j].after as int),position:ts[j].next}},
    ensures m::MCompressed(z,ts[k as int].pos,raw.take(ts[k as int].before as int),raw.len(),32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) ==
        (m::StoredEffect::End {out:raw,position:ts.last().next}),
    decreases ts.len()-k,
{
    hide(m::MCompressedStep);
    if k+1 < ts.len() {expand(z,raw,ts,k+1);}
    reveal_with_fuel(m::MCompressed,2);
}
proof fn step_effect_at(z: Seq<u8>, raw: Seq<u8>, t: Step, last: bool, limit: nat)
    requires raw.len() <= limit, valid_step(z,raw,t,last),
    ensures m::MCompressedStep(z,t.pos,raw.take(t.before as int),limit,32768,m::fixed_lit_lengths(),m::fixed_dist_lengths()) ==
        if last {m::SymbolEffect::End {out:raw,position:t.next}}
        else {m::SymbolEffect::Next {out:raw.take(t.after as int),position:t.next}},
{
    hide(m::MDec); hide(m::decode_prefix); hide(m::cw); hide(m::copy_distance);
    assert(crate::limits::FIXED_LIT_COUNT == 288) by (compute);
    assert(crate::limits::FIXED_DIST_COUNT == 32) by (compute);
    assert(crate::limits::WINDOW_MAX == 32768) by (compute);
    assert(crate::limits::LEN_MAX == 258) by (compute);
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
pub(crate) proof fn effects_at(z: Seq<u8>, raw: Seq<u8>, ts: Seq<Step>, a: nat, b: nat, limit: nat)
    requires raw.len() <= limit, a <= b <= ts.len(), trace(z,raw,ts,a,b),
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
pub(crate) proof fn expand_at(z: Seq<u8>, raw: Seq<u8>, ts: Seq<Step>, k: nat, limit: nat)
    requires raw.len() <= limit, k < ts.len(),
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
} // verus!
