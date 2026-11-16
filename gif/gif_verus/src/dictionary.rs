//! Readable-range invariants and insertion into the caller's three table views.
use vstd::prelude::*;
verus! {
#[cfg(verus_only)]
use crate::chain::*;
#[cfg(verus_only)]
use crate::refinement::*;

pub open spec fn valid(clear:int,next:int,c:int)->bool {
    0<=c && (c<clear || clear+2<=c<next)
}
pub open spec fn table_ok(pf:Seq<u16>,ls:Seq<u16>,clear:int,next:int)->bool {
    &&& pf.len()==4096 && ls.len()==4096
    &&& 0<=clear && clear+2<=next<=4096
    &&& forall|c:int| clear+2<=c<next ==> #[trigger] link_ok(pf,clear,c)
        && clen(ls,clear,c)==clen(ls,clear,pf[c] as int)+1
        && 1<=clen(ls,clear,c)<=4096
}
pub proof fn chain_valid(pf:Seq<u16>,ls:Seq<u16>,clear:int,next:int,c:int)
    requires table_ok(pf,ls,clear,next), valid(clear,next,c), c<4096,
    ensures chain_ok(pf,clear,c),
        forall|x:int| clear+2<=x<=c ==> #[trigger] clen(ls,clear,x)==clen(ls,clear,pf[x] as int)+1,
{
    assert forall|x:int| clear+2<=x<=c implies #[trigger] link_ok(pf,clear,x) by {}
    assert forall|x:int| clear+2<=x<=c implies
        #[trigger] clen(ls,clear,x)==clen(ls,clear,pf[x] as int)+1 by {
        assert(link_ok(pf,clear,x));
    }
}

pub fn cached(c:usize,clear:usize,ls:&[u16])->(n:usize)
    requires c<4096, ls@.len()==4096,
    ensures n as int==clen(ls@,clear as int,c as int),
{
    if c<clear {1} else {ls[c] as usize}
}

pub fn insert(e:usize,p:usize,v:u8,clear:usize,
    pf:&mut [u16],sf:&mut [u8],ls:&mut [u16])
    requires table_ok(old(pf)@,old(ls)@,clear as int,e as int),
        old(sf)@.len()==4096, e<4096,
        valid(clear as int,e as int,p as int), p<e,
    ensures clen(old(ls)@,clear as int,p as int)+1<=4096,
        table_ok(final(pf)@,final(ls)@,clear as int,e as int+1),
        final(pf)@==old(pf)@.update(e as int,p as u16),
        final(sf)@==old(sf)@.update(e as int,v),
        final(ls)@==old(ls)@.update(e as int,(clen(old(ls)@,clear as int,p as int)+1) as u16),
        str_of(final(sf)@,final(pf)@,clear as int,e as int,4096)
            ==str_of(old(sf)@,old(pf)@,clear as int,p as int,4096).push(v),
{
    let ghost op=pf@;
    let ghost os=sf@;
    let ghost ol=ls@;
    proof {
        chain_valid(pf@,ls@,clear as int,e as int,p as int);
        cache_shape(sf@,pf@,ls@,clear as int,p as int,4096);
        lemma_str_len_bound(sf@,pf@,clear as int,p as int,4096);
    }
    let len=cached(p,clear,ls);
    pf[e]=p as u16;
    sf[e]=v;
    ls[e]=(len+1) as u16;
    proof {
        assert(table_ok(pf@,ls@,clear as int,e as int+1)) by {
            assert forall|x:int| clear+2<=x<e+1 implies #[trigger] link_ok(pf@,clear as int,x)
                && clen(ls@,clear as int,x)==clen(ls@,clear as int,pf@[x] as int)+1
                && 1<=clen(ls@,clear as int,x)<=4096 by {
                if x==e {} else { assert(link_ok(op,clear as int,x)); }
            }
        }
        chain_valid(pf@,ls@,clear as int,e as int+1,e as int);
        lemma_chain_step(sf@,pf@,clear as int,e as int,4096);
        fresh_frame(os,op,clear as int,e as int,p as int,v,p as int);
        fuel_irrelevant(sf@,pf@,clear as int,p as int,4095,4096);
    }
}
/// Historical bounds are opaque to the decoder's large loop query. The
/// following lemmas expose only the facts needed by each concrete operation.
pub closed spec fn history(pf:Seq<u16>,ls:Seq<u16>,clear:int,next:int,q:int,
    have:bool,prev:int,checked:bool)->bool {
    &&& q>=0
    &&& have ==> q>=1 && clen(ls,clear,prev)<=q && (checked && prev<clear ==> prev<256)
    &&& forall|x:int| clear+2<=x<next ==> #[trigger] ls[x]<=q
    &&& forall|x:int| checked && clear+2<=x<next && (pf[x] as int)<clear ==> #[trigger] pf[x]<256
}
pub proof fn history_empty(pf:Seq<u16>,ls:Seq<u16>,clear:int,q:int,checked:bool)
    requires q>=0,
    ensures history(pf,ls,clear,clear+2,q,false,0,checked),
{reveal(history);}

pub proof fn history_read(pf:Seq<u16>,ls:Seq<u16>,clear:int,next:int,q:int,have:bool,prev:int,
    checked:bool,c:int)
    requires history(pf,ls,clear,next,q,have,prev,checked), valid(clear,next,c),
        chain_ok(pf,clear,c), fuel_above(c,4096),
        checked && c<clear && (!have || c!=prev) ==> c<256,
    ensures clen(ls,clear,c)<=q+1,
        c>=clear ==> clen(ls,clear,c)<=q,
        have ==> q>=1 && clen(ls,clear,prev)<=q && (checked && prev<clear ==> prev<256),
        checked ==> root_of(pf,clear,c,4096)<256,
{
    reveal(history);
    if checked {crate::refinement::checked_root(pf,clear,c,4096);}
}

pub proof fn history_advance(pf:Seq<u16>,ls:Seq<u16>,np:Seq<u16>,nl:Seq<u16>,clear:int,next:int,
    q:int,have:bool,prev:int,checked:bool,code:int,n:int)
    requires history(pf,ls,clear,next,q,have,prev,checked),
        pf.len()==4096, ls.len()==4096,
        have ==> 0<=prev<4096,
        valid(clear,next,code) || code==next && have && next<4096,
        checked && code<clear ==> code<256,
        n==if code<next {clen(ls,clear,code)} else {clen(ls,clear,prev)+1}, n>=1,
        np==if have && next<4096 {pf.update(next,prev as u16)} else {pf},
        nl==if have && next<4096 {ls.update(next,(clen(ls,clear,prev)+1) as u16)} else {ls},
        have ==> 0<=prev<next && (prev<clear || clear+2<=prev),
        have && next<4096 ==> clen(ls,clear,prev)+1<=4096,
        0<=clear && clear+2<=next<=4096,
    ensures history(np,nl,clear,if have && next<4096 {next+1} else {next},q+n,true,code,checked),
{
    reveal(history);
    let grow=have && next<4096;
    let nn=if grow {next+1} else {next};
    assert forall|x:int| clear+2<=x<nn implies #[trigger] nl[x]<=q+n by {
        if grow && x==next {} else {assert(ls[x]<=q);}
    }
    assert forall|x:int| checked && clear+2<=x<nn && (np[x] as int)<clear implies
        #[trigger] np[x]<256 by {
        if grow && x==next {} else {assert(pf[x]<256);}
    }
    assert(clen(nl,clear,code)<=q+n);
}

} // verus!
