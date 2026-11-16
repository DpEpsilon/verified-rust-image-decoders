//! Representation lemmas for the executable decoder's readable dictionary.
use vstd::prelude::*;
verus! {
#[cfg(verus_only)]
use crate::chain::*;

pub proof fn fuel_irrelevant(sf: Seq<u8>, pf: Seq<u16>, clear: int, c: int, f: nat, g: nat)
    requires chain_ok(pf, clear, c), fuel_above(c, f), fuel_above(c, g),
    ensures str_of(sf,pf,clear,c,f) == str_of(sf,pf,clear,c,g),
        root_of(pf,clear,c,f) == root_of(pf,clear,c,g),
    decreases c,
{
    if c < clear { reveal(str_of); reveal(root_of); }
    else {
        lemma_chain_step(sf,pf,clear,c,f);
        lemma_chain_step(sf,pf,clear,c,g);
        fuel_irrelevant(sf,pf,clear,pf[c] as int,(f-1) as nat,(g-1) as nat);
    }
}

/// Agreement only on the readable stored range is sufficient; literals never
/// read table bytes, and every parent is smaller than its child.
pub proof fn table_agreement(sf: Seq<u8>, pf: Seq<u16>, gs: Seq<u8>, gp: Seq<u16>, clear: int, c: int, f: nat)
    requires chain_ok(pf,clear,c), fuel_above(c,f), gp.len()==4096,
        forall|x:int| clear+2<=x<=c ==> #[trigger] gp[x]==pf[x] && gs[x]==sf[x],
    ensures chain_ok(gp,clear,c),
        str_of(sf,pf,clear,c,f)==str_of(gs,gp,clear,c,f),
        root_of(pf,clear,c,f)==root_of(gp,clear,c,f),
    decreases c,
{
    assert(chain_ok(gp,clear,c)) by {
        assert forall|x:int| clear+2<=x<=c implies #[trigger] link_ok(gp,clear,x) by {
            assert(link_ok(pf,clear,x));
        }
    }
    if c<clear { reveal(str_of); reveal(root_of); }
    else {
        lemma_chain_step(sf,pf,clear,c,f);
        lemma_chain_step(gs,gp,clear,c,f);
        table_agreement(sf,pf,gs,gp,clear,pf[c] as int,(f-1) as nat);
    }
}

pub proof fn fresh_frame(sf:Seq<u8>,pf:Seq<u16>,clear:int,c:int,p:int,v:u8,cc:int)
    requires sf.len()==4096, chain_ok(pf,clear,cc), cc<c<4096, 0<=p<c,
        p<clear || clear+2<=p,
    ensures
        chain_ok(pf.update(c,p as u16),clear,cc),
        str_of(sf.update(c,v),pf.update(c,p as u16),clear,cc,4096)
            ==str_of(sf,pf,clear,cc,4096),
        root_of(pf.update(c,p as u16),clear,cc,4096)==root_of(pf,clear,cc,4096),
{
    table_agreement(sf,pf,sf.update(c,v),pf.update(c,p as u16),clear,cc,4096);
}

pub open spec fn clen(ls:Seq<u16>, clear:int, c:int)->int {
    if c<clear {1} else {ls[c] as int}
}

pub proof fn cache_shape(sf:Seq<u8>,pf:Seq<u16>,ls:Seq<u16>,clear:int,c:int,f:nat)
    requires chain_ok(pf,clear,c), c<4096, fuel_above(c,f), ls.len()==4096,
        forall|x:int| clear+2<=x<=c ==> #[trigger] clen(ls,clear,x)==clen(ls,clear,pf[x] as int)+1,
    ensures str_of(sf,pf,clear,c,f).len()==clen(ls,clear,c),
        1<=clen(ls,clear,c)<=4096,
        str_of(sf,pf,clear,c,f)[0]==root_of(pf,clear,c,f) as u8,
    decreases c,
{
    lemma_str_len_bound(sf,pf,clear,c,f);
    if c<clear { reveal(str_of); reveal(root_of); }
    else {
        lemma_chain_step(sf,pf,clear,c,f);
        cache_shape(sf,pf,ls,clear,pf[c] as int,(f-1) as nat);
    }
}
/// Checked literals never enter a prefix link, so every reachable chain root
/// remains a byte even when the literal domain extends above 255.
pub proof fn checked_root(pf:Seq<u16>,clear:int,c:int,f:nat)
    requires chain_ok(pf,clear,c), fuel_above(c,f),
        c<clear ==> c<256,
        forall|x:int| clear+2<=x<=c && (pf[x] as int)<clear ==> #[trigger] pf[x]<256,
    ensures root_of(pf,clear,c,f)<256,
    decreases c,
{
    if c<clear {reveal(root_of);} else {
        lemma_chain_step(sf_zero(),pf,clear,c,f);
        if (pf[c] as int)<clear {assert(pf[c]<256);}
        checked_root(pf,clear,pf[c] as int,(f-1) as nat);
    }
}

/// L-DISCARD: a code read before raster completion can discard at most A-1
/// indices because its string has at most q+1 indices.
pub proof fn discard_bound(q:int,a:int,n:int)
    requires 0<=q<a, 1<=n<=q+1, n>a-q,
    ensures 1<=n-(a-q)<=a-1,
{}

} // verus!
