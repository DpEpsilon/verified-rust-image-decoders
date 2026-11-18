//! Independent literal code-length traces for the frozen dynamic witnesses.
use crate::{
    model as m,
    witnesses::{follow_word, wire_word},
};
use vstd::prelude::*;
verus! {

// A balanced conjunction keeps concrete checking below the interpreter's
// default recursion depth without changing the independent decoder model.
pub(crate) open spec fn trace_words(z: Seq<u8>, code: Seq<u8>, values: Seq<u8>, positions: Seq<nat>, a: nat, b: nat) -> bool
    decreases b - a,
{
    if a >= b { true }
    else if b == a + 1 {
        values[a as int] < code.len() && code[values[a as int] as int] > 0
        && positions[(a + 1) as int] == positions[a as int] + code[values[a as int] as int]
        && positions[(a + 1) as int] <= 8 * z.len()
        && wire_word(z, positions[a as int], code[values[a as int] as int] as nat) == m::cw(code, values[a as int] as int).unwrap()
    } else {
        let mid = (a + (b - a) / 2) as nat;
        trace_words(z, code, values, positions, a, mid) && trace_words(z, code, values, positions, mid, b)
    }
}

pub(crate) proof fn trace_decodes(z: Seq<u8>, code: Seq<u8>, values: Seq<u8>, positions: Seq<nat>, a: nat, b: nat)
    requires m::lengths_ok(code), m::Kraft(code) <= crate::limits::CODE_SPACE,
        a <= b <= values.len(), positions.len() == values.len() + 1,
        trace_words(z, code, values, positions, a, b),
    ensures forall|j: int| a <= j < b ==> m::MDec(code, z, positions[j]) ==
        (m::Decoded::Symbol { symbol: values[j] as nat, position: positions[j+1] }),
        forall|j: int| a <= j < b ==> #[trigger] positions[j] < positions[j+1] <= 8 * z.len(),
    decreases b - a,
{
    if a < b {
        if b == a + 1 {
            follow_word(code, z, positions[a as int], values[a as int] as int, 0);
        } else {
            let mid = (a + (b - a) / 2) as nat;
            trace_decodes(z, code, values, positions, a, mid);
            trace_decodes(z, code, values, positions, mid, b);
        }
    }
}

pub(crate) proof fn expand_literals(z: Seq<u8>, code: Seq<u8>, values: Seq<u8>, positions: Seq<nat>, nlen: nat, ndist: nat, k: nat)
    requires m::dynamic_sizes_ok(nlen, ndist), values.len() == nlen + ndist,
        m::length_entries_ok(values), code.len() == crate::limits::NCL,
        m::admissible(code, m::CodeKind::CodeLen, true),
        positions.len() == values.len() + 1, k <= values.len(), positions[k as int] <= 8 * z.len(),
        forall|j: int| 0 <= j < values.len() ==> values[j] < 16,
        forall|j: int| k <= j < values.len() ==> m::MDec(code, z, positions[j]) ==
            (m::Decoded::Symbol { symbol: values[j] as nat, position: positions[j+1] }),
        forall|j: int| k <= j < values.len() ==> #[trigger] positions[j] < positions[j+1] <= 8 * z.len(),
    ensures m::MDynamicLengths(z, positions[k as int], code, values.take(k as int), nlen, ndist, true)
        == m::finish_dynamic(values, nlen, ndist, positions[values.len() as int], true),
    decreases values.len() - k,
{
    assert(m::length_entries_ok(values.take(k as int)));
    if k < values.len() {
        expand_literals(z, code, values, positions, nlen, ndist, k+1);
        assert(values.take(k as int).push(values[k as int]) =~= values.take((k+1) as int));
    } else {
        assert(values.take(k as int) =~= values);
    }
}

pub(crate) open spec fn cl_trace(z: Seq<u8>, p: nat, code: Seq<u8>, i: nat) -> Seq<u8>
    decreases 19-i,
{
    if i >= 19 { code } else {
        cl_trace(z,p+3,code.update(m::code_length_order()[i as int],m::field_value(z,p,3) as u8),i+1)
    }
}
pub(crate) proof fn cl_bridge(z: Seq<u8>, p: nat, code: Seq<u8>, i: nat, nlen: nat, ndist: nat)
    requires i <= 19, code.len() == 19, forall|s: int| 0 <= s < code.len() ==> code[s] <= 7,
        p+3*(19-i) <= 8*z.len(), m::dynamic_sizes_ok(nlen,ndist),
        m::admissible(cl_trace(z,p,code,i),m::CodeKind::CodeLen,true),
    ensures m::MDynamicCL(z,p,code,i,19,nlen,ndist,true) ==
        m::MDynamicLengths(z,(p+3*(19-i)) as nat,cl_trace(z,p,code,i),Seq::empty(),nlen,ndist,true),
    decreases 19-i,
{
    assert(crate::limits::NCL == 19) by (compute);
    if i < 19 {
        m::lemma_field_bound(z,p,3);
        assert(m::pow2(3) == 8) by (compute);
        cl_bridge(z,p+3,code.update(m::code_length_order()[i as int],m::field_value(z,p,3) as u8),i+1,nlen,ndist);
    }
}

// Authored sparse RFC profiles: EOB alone; four tiny-image symbols; eight
// symbols including the length-3 match and three unused fillers.
pub(crate) open spec fn profile_active(mode: nat, k: nat) -> nat {
    (if k > 256 { 1nat } else { 0nat })
    + if mode == 0 { 0nat } else {
        (if k > 0 { 1nat } else { 0nat }) + (if k > 65 { 1nat } else { 0nat }) + (if k > 66 { 1nat } else { 0nat })
        + if mode == 1 { 0nat } else {
            (if k > 1 { 1nat } else { 0nat }) + (if k > 2 { 1nat } else { 0nat })
            + (if k > 3 { 1nat } else { 0nat }) + (if k > 257 { 1nat } else { 0nat })
        }
    }
}
pub(crate) open spec fn profile(mode: nat) -> Seq<u8> {
    Seq::new(if mode == 0 { 257nat } else { 286nat }, |i: int|
        if i == 256 || (mode != 0 && (i == 0 || i == 65 || i == 66))
            || (mode == 2 && (i == 1 || i == 2 || i == 3 || i == 257)) { (mode+1) as u8 } else { 0u8 })
}
proof fn profile_counts(mode: nat, n: nat, k: nat)
    requires mode <= 2, k <= profile(mode).len(),
    ensures m::count_prefix(profile(mode), n, k) == if n == 0 { (k - profile_active(mode,k)) as nat }
        else if n == mode+1 { profile_active(mode,k) } else { 0nat },
    decreases k,
{
    if k > 0 { profile_counts(mode, n, (k-1) as nat); }
}
proof fn profile_kraft(mode: nat, k: nat)
    requires mode <= 2, k <= profile(mode).len(),
    ensures m::kraft_prefix(profile(mode),k) == profile_active(mode,k) * m::pow2((14-mode) as nat),
    decreases k,
{
    assert(crate::limits::MAX_CODE_BITS == 15) by (compute);
    assert(m::pow2(12) == 4096) by (compute);
    assert(m::pow2(13) == 8192) by (compute);
    assert(m::pow2(14) == 16384) by (compute);
    reveal_with_fuel(m::kraft_prefix, 2);
    if k > 0 {
        profile_kraft(mode, (k-1) as nat);
        assert(profile_active(mode,k) == profile_active(mode,(k-1) as nat) + if profile(mode)[(k-1) as int] > 0 { 1nat } else { 0nat });
        if profile(mode)[(k-1) as int] > 0 {
            assert(m::pow2((crate::limits::MAX_CODE_BITS-profile(mode)[(k-1) as int]) as nat) == m::pow2((14-mode) as nat));
        }
    }
    assert(m::kraft_prefix(profile(mode),k) == profile_active(mode,k) * m::pow2((14-mode) as nat)) by (nonlinear_arith)
        requires k == 0 ==> m::kraft_prefix(profile(mode),k) == 0 && profile_active(mode,k) == 0,
            k > 0 ==> m::kraft_prefix(profile(mode),k) == m::kraft_prefix(profile(mode),(k-1) as nat) + if profile(mode)[(k-1) as int] > 0 { m::pow2((14-mode) as nat) } else { 0nat },
            k > 0 ==> m::kraft_prefix(profile(mode),(k-1) as nat) == profile_active(mode,(k-1) as nat) * m::pow2((14-mode) as nat),
            k > 0 ==> profile_active(mode,k) == profile_active(mode,(k-1) as nat) + if profile(mode)[(k-1) as int] > 0 { 1nat } else { 0nat };
}
pub(crate) proof fn profile_facts(mode: nat)
    requires mode <= 2,
    ensures m::lengths_ok(profile(mode)), m::admissible(profile(mode),m::CodeKind::LitLen,true),
        m::Kraft(profile(mode)) <= crate::limits::CODE_SPACE,
{
    profile_counts(mode,0,profile(mode).len());
    profile_counts(mode,1,profile(mode).len());
    profile_kraft(mode,profile(mode).len());
    assert(crate::limits::MAX_CODE_BITS == 15) by (compute);
    assert(m::pow2(12) == 4096) by (compute);
    assert(m::pow2(13) == 8192) by (compute);
    assert(m::pow2(14) == 16384) by (compute);
    assert(crate::limits::CODE_SPACE == 32768) by (compute);
    assert(m::Kraft(profile(mode)) == profile_active(mode,profile(mode).len()) * m::pow2((14-mode) as nat));
    if mode == 0 {
        assert(profile_active(mode,profile(mode).len()) == 1);
        assert((14-mode) as nat == 14);
        assert(m::Kraft(profile(mode)) == 16384) by (nonlinear_arith)
            requires m::Kraft(profile(mode)) == 1 * 16384;
    } else if mode == 1 {
        assert(profile_active(mode,profile(mode).len()) == 4);
        assert((14-mode) as nat == 13);
        assert(m::Kraft(profile(mode)) == 32768) by (nonlinear_arith)
            requires m::Kraft(profile(mode)) == 4 * 8192;
    }
    else {
        assert(profile_active(mode,profile(mode).len()) == 8);
        assert((14-mode) as nat == 12);
        assert(m::Kraft(profile(mode)) == 32768) by (nonlinear_arith)
            requires m::Kraft(profile(mode)) == 8 * 4096;
    }
}
proof fn profile_next(mode: nat, n: nat)
    requires mode <= 2, n <= mode+1,
    ensures m::next_code(profile(mode),n) == 0,
    decreases n,
{
    if n > 0 { profile_next(mode,(n-1) as nat); profile_counts(mode,(n-1) as nat,profile(mode).len()); }
}
pub(crate) proof fn profile_decode(z: Seq<u8>, start: nat, mode: nat, s: int)
    requires mode <= 2, 0 <= s < profile(mode).len(), profile(mode)[s] > 0,
        start+mode+1 <= 8*z.len(),
        wire_word(z,start,mode+1) == m::msb_bits(profile_active(mode,s as nat),mode+1),
    ensures m::MDec(profile(mode),z,start) == (m::Decoded::Symbol { symbol: s as nat, position: start+mode+1 }),
{
    profile_facts(mode); profile_next(mode,mode+1); profile_counts(mode,mode+1,s as nat);
    follow_word(profile(mode),z,start,s,0);
}

pub(crate) proof fn single_error(lengths: Seq<u8>, z: Seq<u8>, start: nat, s: int)
    requires m::lengths_ok(lengths), m::Kraft(lengths) <= crate::limits::CODE_SPACE,
        0 <= s < lengths.len(), start <= 8*z.len(),
        m::cw(lengths,s) == Some(seq![false]),
        forall|t: int| t != s ==> m::cw(lengths,t).is_none(),
        start == 8*z.len() || m::bit(z,start),
    ensures m::MDec(lengths,z,start) == if start == 8*z.len() {
        m::Decoded::TruncatedZlib { position: start }
    } else { m::Decoded::InvalidSymbol { position: start+1 } },
{
    assert(crate::limits::MAX_CODE_BITS == 15) by (compute);
    assert forall|t: int| !m::exact(lengths,Seq::empty(),t) by {}
    assert(m::prefix(Seq::empty(),seq![false]));
    assert(m::live(lengths,Seq::empty()));
    if start == 8*z.len() {
        m::lemma_decode_prefix_errors(lengths,z,start,Seq::empty());
    } else {
        m::lemma_decode_prefix_step(lengths,z,start,Seq::empty());
        assert(Seq::<bool>::empty().push(m::bit(z,start)) =~= seq![true]);
        assert(seq![true] != seq![false]) by { assert(seq![true][0] != seq![false][0]); }
        assert(!m::prefix(seq![true],seq![false])) by {
            assert(seq![false].take(1) =~= seq![false]);
        }
        assert forall|t: int| !m::exact(lengths,seq![true],t) by {
            if t != s { assert(m::cw(lengths,t).is_none()); }
        }
        assert(!m::live(lengths,seq![true]));
        m::lemma_decode_prefix_errors(lengths,z,start,seq![true]);
    }
}

pub(crate) proof fn eob_single_word()
    ensures m::cw(profile(0),256) == Some(seq![false]),
        forall|t: int| t != 256 ==> m::cw(profile(0),t).is_none(),
{
    profile_next(0,1); profile_counts(0,1,256);
    assert(m::msb_bits(0,1) == seq![false]) by (compute);
}

} // verus!
