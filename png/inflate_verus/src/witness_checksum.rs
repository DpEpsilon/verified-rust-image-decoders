//! Balanced independent Adler sums for authored raw byte sequences.
use crate::model as m;
use vstd::prelude::*;
verus! {
pub(crate) open spec fn sums(raw: Seq<u8>, a: nat, b: nat) -> (nat,nat)
    decreases b-a,
{
    if a >= b {(0,0)} else if b == a+1 {(raw[a as int] as nat,1+raw[a as int] as nat)}
    else {
        let mid = (a+(b-a)/2) as nat;
        let l = sums(raw,a,mid); let r = sums(raw,mid,b);
        (l.0+r.0,(l.1+r.1+(b-mid)*l.0) as nat)
    }
}
proof fn concat_sums(a: Seq<u8>, b: Seq<u8>)
    ensures m::sum_bytes(a+b) == m::sum_bytes(a)+m::sum_bytes(b),
        m::sum_prefixes(a+b) == m::sum_prefixes(a)+m::sum_prefixes(b)+b.len()*m::sum_bytes(a),
    decreases b.len(),
{
    if b.len() == 0 {assert(a+b =~= a);}
    else {
        concat_sums(a,b.drop_last());
        assert((a+b).drop_last() =~= a+b.drop_last());
        assert((a+b).last() == b.last());
        assert(m::sum_prefixes(a+b) == m::sum_prefixes(a)+m::sum_prefixes(b)+b.len()*m::sum_bytes(a)) by (nonlinear_arith)
            requires m::sum_prefixes(a+b) == m::sum_prefixes(a+b.drop_last())+1+m::sum_bytes(a+b),
                m::sum_prefixes(a+b.drop_last()) == m::sum_prefixes(a)+m::sum_prefixes(b.drop_last())+b.drop_last().len()*m::sum_bytes(a),
                m::sum_prefixes(b) == m::sum_prefixes(b.drop_last())+1+m::sum_bytes(b),
                m::sum_bytes(a+b) == m::sum_bytes(a)+m::sum_bytes(b), b.drop_last().len()+1 == b.len();
    }
}
pub(crate) proof fn sums_correct(raw: Seq<u8>, a: nat, b: nat)
    requires a <= b <= raw.len(),
    ensures sums(raw,a,b).0 == m::sum_bytes(raw.subrange(a as int,b as int)),
        sums(raw,a,b).1 == m::sum_prefixes(raw.subrange(a as int,b as int)),
    decreases b-a,
{
    assert(m::sum_bytes(Seq::<u8>::empty()) == 0) by (compute);
    assert(m::sum_prefixes(Seq::<u8>::empty()) == 0) by (compute);
    if a == b {assert(raw.subrange(a as int,b as int) =~= Seq::<u8>::empty());}
    else if b == a+1 {
        m::lemma_adler_sums_push(Seq::<u8>::empty(),raw[a as int]);
        assert(Seq::<u8>::empty().push(raw[a as int]) =~= seq![raw[a as int]]);
        assert(raw.subrange(a as int,b as int) =~= seq![raw[a as int]]);
        assert(seq![raw[a as int]].drop_last() =~= Seq::<u8>::empty());
    } else {
        let mid = (a+(b-a)/2) as nat;
        sums_correct(raw,a,mid); sums_correct(raw,mid,b);
        let l = raw.subrange(a as int,mid as int); let r = raw.subrange(mid as int,b as int);
        assert(l+r =~= raw.subrange(a as int,b as int));
        concat_sums(l,r);
        assert(r.len() == b-mid);
        assert(sums(raw,a,b) == (sums(raw,a,mid).0+sums(raw,mid,b).0,
            (sums(raw,a,mid).1+sums(raw,mid,b).1+(b-mid)*sums(raw,a,mid).0) as nat));
        assert(sums(raw,a,b).1 == m::sum_prefixes(l+r));
    }
}
} // verus!
