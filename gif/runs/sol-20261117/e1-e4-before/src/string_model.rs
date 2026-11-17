//! Independent G10 dictionary semantics: dictionary entries and prev are byte
//! strings, never prefix/suffix arrays. The representation proofs below depend
//! on proposed E4's byte-readable literal domain; excluded checked literals
//! are rejected before any dictionary lookup or insertion.
use vstd::prelude::*;
#[cfg(verus_only)]
use crate::lzw_model as ml;
use crate::lzw::LzwParams;

verus! {

pub struct State {
    pub pos: int,
    pub width: int,
    pub next: int,
    pub prev: Option<Seq<u8>>,
    pub q: int,
    pub table: Seq<Seq<u8>>,
    pub out: Seq<u8>,
}

/// Unreadable entries are empty, including checked literals above the byte
/// domain. Neither Clear/EOI nor these excluded literals denote byte strings.
pub open spec fn initial_table(m: int) -> Seq<Seq<u8>> {
    Seq::new(4096, |c:int| if c < ml::lzw_clear_of(m) && c < 256 {seq![c as u8]} else {Seq::empty()})
}

pub open spec fn initial(m: int) -> State {
    State {pos:0,width:m+1,next:ml::lzw_clear_of(m)+2,prev:None,q:0,table:initial_table(m),out:Seq::empty()}
}

pub open spec fn clear(m:int,s:State,after:int)->State {
    State {pos:after,width:m+1,next:ml::lzw_clear_of(m)+2,prev:None,q:s.q,table:initial_table(m),out:s.out}
}

pub open spec fn readable(m:int,next:int,c:int)->bool {
    0<=c && (c<ml::lzw_clear_of(m) && c<256 || ml::lzw_clear_of(m)+2<=c<next)
}

pub open spec fn resolvable(m:int,s:State,c:int)->bool {
    0<=c && (c<ml::lzw_clear_of(m) && c<256
        || ml::lzw_clear_of(m)+2<=c<s.next
        || c==s.next && s.prev is Some && s.next<4096)
}

pub open spec fn resolve(s:State,c:int)->Seq<u8> {
    if c<s.next {s.table[c]} else {let p=s.prev.unwrap(); p.push(p[0])}
}

/// G10.2 steps 7-8. The only dictionary update is prev ++ [first(cur)].
pub open spec fn emit(s:State,after:int,c:int,cur:Seq<u8>)->State {
    let grow=s.prev is Some && s.next<4096;
    State {pos:after,
        width:if grow {if s.width<12 && s.next+1==ml::lzw_ipow2(s.width) {s.width+1} else {s.width}} else {s.width},
        next:if grow {s.next+1} else {s.next},
        prev:Some(cur),q:s.q+cur.len(),
        table:if grow {s.table.update(s.next,s.prev.unwrap().push(cur[0]))} else {s.table},
        out:s.out+cur}
}

pub enum Step {Halt(ml::MRes), Continue(State)}

/// G10.2 in the specified error order. Code extraction is the mathematical
/// LSB-first bit function, not an executable reader or a dictionary operation.
pub open spec fn step(m:int,d:Seq<u8>,a:int,ps:LzwParams,s:State)->Step {
    if ps.stop_at_raster && s.q==a {
        Step::Halt(ml::observation(s.out,None,s.pos,false))
    } else if s.pos+s.width>ml::nbits(d) {
        Step::Halt(ml::observation(s.out,Some(if s.q<a {ml::ErrK::TooFewPixels} else {ml::ErrK::MissingEndOfInformation}),s.pos,false))
    } else {
        let c=ml::code_at(d,s.pos,s.width,0);
        let after=s.pos+s.width;
        if c==ml::lzw_clear_of(m) {Step::Continue(clear(m,s,after))}
        else if s.q==a && c!=ml::lzw_eoi_of(m) {
            Step::Halt(ml::observation(s.out,Some(ml::ErrK::MissingEndOfInformation),after,false))
        } else if c==ml::lzw_eoi_of(m) {
            Step::Halt(ml::observation(s.out,if s.q<a {Some(ml::ErrK::TooFewPixels)}
                else if !ps.stop_at_raster && ml::eoi_tail(after,d) {Some(ml::ErrK::DataAfterEndOfInformation)} else {None},after,false))
        } else if ps.checked_literals && c<ml::lzw_clear_of(m) && c>=256 {
            Step::Halt(ml::observation(s.out,Some(ml::ErrK::ColorIndexOutOfRange),after,false))
        } else if resolvable(m,s,c) {
            let cur=resolve(s,c);
            let ns=emit(s,after,c,cur);
            if s.q+cur.len()>a {
                Step::Halt(ml::observation(s.out+cur.take(a-s.q),if ps.stop_at_raster {None} else {Some(ml::ErrK::TooManyPixels)},after,ps.stop_at_raster))
            } else {Step::Continue(ns)}
        } else {Step::Halt(ml::observation(s.out,Some(ml::ErrK::InvalidLzwCode),after,false))}
    }
}

pub open spec fn fold(m:int,d:Seq<u8>,a:int,ps:LzwParams,s:State)->ml::MRes
    decreases ml::nbits(d)-s.pos,
{
    match step(m,d,a,ps,s) {
        Step::Halt(r)=>r,
        Step::Continue(ns)=>if ns.pos>s.pos {fold(m,d,a,ps,ns)}
            else {ml::observation(s.out,Some(ml::ErrK::InvalidLzwCode),s.pos,false)},
    }
}

pub open spec fn ST(m:int,d:Seq<u8>,a:int,ps:LzwParams)->ml::MRes {
    if !ml::m_ok(ps,m,a) {ml::observation(Seq::empty(),Some(ml::ErrK::InvalidLzwMinCodeSize),0,false)}
    else {fold(m,d,a,ps,initial(m))}
}

// The independent definitions above never inspect a prefix link. Only the
// simulation relation and its proofs below mention the ML representation.
pub open spec fn relates(m:int,x:ml::St,s:State)->bool {
    &&& 2<=ml::lzw_clear_of(m)<=2048
    &&& ml::lzw_clear_of(m)+2<=x.next<=4096
    &&& x.pf.len()==4096 && x.sf.len()==4096 && s.table.len()==4096
    &&& 0<=x.pos && 2<=x.width<=12 && 0<=x.q && x.q==x.o.len()
    &&& s.pos==x.pos && s.width==x.width && s.next==x.next && s.q==x.q && s.out==x.o
    &&& x.prev>=-1 && (x.prev>=0 ==> readable(m,x.next,x.prev))
    &&& s.prev==if x.prev>=0 {Some(s.table[x.prev])} else {None}
    &&& forall|c:int| ml::lzw_clear_of(m)+2<=c<x.next ==> #[trigger] crate::chain::link_ok(x.pf,ml::lzw_clear_of(m),c)
        && ((x.pf[c] as int)<ml::lzw_clear_of(m) ==> x.pf[c]<256)
    &&& forall|c:int| readable(m,x.next,c) ==> #[trigger] s.table[c]==crate::chain::str_of(x.sf,x.pf,ml::lzw_clear_of(m),c,4096)
        && s.table[c].len()>=1
}

pub proof fn lemma_initial(m:int)
    requires 1<=m<=11,
    ensures relates(m,ml::init_st(m),initial(m)),
{
    crate::lzw::lemma_shift_power(m as u32);
    assert forall|c:int| readable(m,ml::init_st(m).next,c) implies
        #[trigger] initial(m).table[c]==crate::chain::str_of(ml::init_st(m).sf,ml::init_st(m).pf,ml::lzw_clear_of(m),c,4096)
            && initial(m).table[c].len()>=1 by {
        reveal(crate::chain::str_of);
    }
}

/// Root/first agreement is proved from the links, not used to define a string.
pub proof fn lemma_chain_first(sf:Seq<u8>,pf:Seq<u16>,clear:int,c:int,f:nat)
    requires crate::chain::chain_ok(pf,clear,c),crate::chain::fuel_above(c,f),
    ensures crate::chain::str_of(sf,pf,clear,c,f).len()>=1,
        crate::chain::str_of(sf,pf,clear,c,f)[0]==crate::chain::root_of(pf,clear,c,f) as u8,
    decreases c,
{
    if c<clear {reveal(crate::chain::str_of);reveal(crate::chain::root_of);}
    else {
        crate::chain::lemma_chain_step(sf,pf,clear,c,f);
        lemma_chain_first(sf,pf,clear,pf[c] as int,(f-1) as nat);
    }
}

pub proof fn lemma_read(m:int,x:ml::St,s:State,c:int)
    requires relates(m,x,s),readable(m,x.next,c),
    ensures crate::chain::chain_ok(x.pf,ml::lzw_clear_of(m),c),
        ml::semit(m,x,c)==s.table[c],ml::first_of(m,x,c)==s.table[c][0],
{
    assert(crate::chain::chain_ok(x.pf,ml::lzw_clear_of(m),c)) by {
        assert forall|i:int| ml::lzw_clear_of(m)+2<=i<=c implies
            #[trigger] crate::chain::link_ok(x.pf,ml::lzw_clear_of(m),i) by {}
    }
    lemma_chain_first(x.sf,x.pf,ml::lzw_clear_of(m),c,4096);
}

pub proof fn lemma_resolve(m:int,x:ml::St,s:State,c:int)
    requires relates(m,x,s),resolvable(m,s,c),
    ensures ml::semit(m,x,c)==resolve(s,c), ml::first_of(m,x,c)==resolve(s,c)[0],resolve(s,c).len()>=1,
{
    if c<x.next {lemma_read(m,x,s,c);}
    else {
        lemma_read(m,x,s,x.prev);
        assert(s.prev.unwrap().len()>=1);
    }
}

pub proof fn lemma_clear(m:int,x:ml::St,s:State,after:int)
    requires relates(m,x,s),1<=m<=11,0<=after,
    ensures relates(m,ml::clear_st(m,x,after),clear(m,s,after)),
{
    crate::lzw::lemma_shift_power(m as u32);
    assert forall|c:int| readable(m,ml::lzw_clear_of(m)+2,c) implies
        #[trigger] clear(m,s,after).table[c]==crate::chain::str_of(x.sf,x.pf,ml::lzw_clear_of(m),c,4096)
            && clear(m,s,after).table[c].len()>=1 by {
        reveal(crate::chain::str_of);
    }
}

pub proof fn lemma_emit(m:int,x:ml::St,s:State,c:int,after:int)
    requires relates(m,x,s),resolvable(m,s,c),0<=after,
    ensures relates(m,ml::emit_st(m,x,after,c,ml::semit(m,x,c),ml::first_of(m,x,c),x.prev>=0 && x.next<4096),
        emit(s,after,c,resolve(s,c))),
{
    lemma_resolve(m,x,s,c);
    let cur=resolve(s,c);
    let v=cur[0];
    let grow=x.prev>=0 && x.next<4096;
    let nx=ml::emit_st(m,x,after,c,ml::semit(m,x,c),ml::first_of(m,x,c),grow);
    let ns=emit(s,after,c,cur);
    if grow {lemma_read(m,x,s,x.prev);}
    assert forall|i:int| ml::lzw_clear_of(m)+2<=i<nx.next implies
        #[trigger] crate::chain::link_ok(nx.pf,ml::lzw_clear_of(m),i)
            && ((nx.pf[i] as int)<ml::lzw_clear_of(m) ==> nx.pf[i]<256) by {
        if grow && i==x.next {} else {assert(crate::chain::link_ok(x.pf,ml::lzw_clear_of(m),i));}
    }
    assert forall|i:int| readable(m,nx.next,i) implies
        #[trigger] ns.table[i]==crate::chain::str_of(nx.sf,nx.pf,ml::lzw_clear_of(m),i,4096)
            && ns.table[i].len()>=1 by {
        if grow && i==x.next {
            assert(crate::chain::chain_ok(nx.pf,ml::lzw_clear_of(m),i));
            crate::chain::lemma_chain_step(nx.sf,nx.pf,ml::lzw_clear_of(m),i,4096);
            crate::refinement::fresh_frame(x.sf,x.pf,ml::lzw_clear_of(m),i,x.prev,v,x.prev);
            crate::refinement::fuel_irrelevant(nx.sf,nx.pf,ml::lzw_clear_of(m),x.prev,4095,4096);
        } else {
            lemma_read(m,x,s,i);
            if grow {crate::refinement::fresh_frame(x.sf,x.pf,ml::lzw_clear_of(m),x.next,x.prev,v,i);}
        }
    }
    if c==x.next {
        assert(grow);
        assert(ns.table[c]==cur);
    } else {assert(ns.table[c]==cur);}
}

/// The policy guard excludes non-byte literals before any resolution or add.
pub proof fn lemma_accepted_code(m:int,x:ml::St,s:State,c:int,checked:bool)
    requires 1<=m<=11,m<=8 || checked,relates(m,x,s),0<=c,
        ml::str_ok(m,x.next,x.prev,c),checked && c<ml::lzw_clear_of(m) ==> c<256,
    ensures resolvable(m,s,c),
{
    if m<=8 {reveal_with_fuel(ml::lzw_ipow2,9);assert(ml::lzw_clear_of(m)<=256);}
}

pub open spec fn outcomes_match(m:int,v:ml::Vd,w:Step)->bool {
    match (v,w) {
        (ml::Vd::Halt(r),Step::Halt(t))=>r==t,
        (ml::Vd::Step(x),Step::Continue(s))=>relates(m,x,s),
        _=>false,
    }
}

/// Lockstep simulation, including error position, emitted prefix and E4.
pub proof fn lemma_step(m:int,d:Seq<u8>,a:int,ps:LzwParams,x:ml::St,s:State)
    requires 1<=m<=11,m<=8 || ps.checked_literals,relates(m,x,s),
    ensures outcomes_match(m,ml::verdict(m,d,a,ps,x),step(m,d,a,ps,s)),
{
    hide(ml::L);hide(fold);
    if !(ps.stop_at_raster && x.q==a) && x.pos+x.width<=ml::nbits(d) {
        crate::lzw::lemma_lzw_code_bound(d,x.pos,x.width,0);
        let c=ml::code_at(d,x.pos,x.width,0);
        let after=x.pos+x.width;
        if c==ml::lzw_clear_of(m) {lemma_clear(m,x,s,after);}
        else if !(x.q==a && c!=ml::lzw_eoi_of(m)) && c!=ml::lzw_eoi_of(m)
            && !(ps.checked_literals && c<ml::lzw_clear_of(m) && c>=256) {
            if m<=8 {reveal_with_fuel(ml::lzw_ipow2,9);assert(ml::lzw_clear_of(m)<=256);}
            assert(ml::str_ok(m,x.next,x.prev,c)==resolvable(m,s,c));
            if resolvable(m,s,c) {
                lemma_resolve(m,x,s,c);
                if x.q+resolve(s,c).len()<=a {lemma_emit(m,x,s,c,after);}
            }
        }
    }
}

pub proof fn lemma_fold(m:int,d:Seq<u8>,a:int,ps:LzwParams,x:ml::St,s:State)
    requires 1<=m<=11,m<=8 || ps.checked_literals,relates(m,x,s),
    ensures ml::L(m,d,a,ps,x)==fold(m,d,a,ps,s),
    decreases ml::nbits(d)-x.pos,
{
    lemma_step(m,d,a,ps,x,s);
    ml::lemma_L_unfold(m,d,a,ps,x);
    reveal_with_fuel(fold,1);
    match ml::verdict(m,d,a,ps,x) {
        ml::Vd::Halt(r)=>{},
        ml::Vd::Step(nx)=>{
            let ns=match step(m,d,a,ps,s) {Step::Continue(ns)=>ns, _=>s};
            if nx.pos>x.pos {lemma_fold(m,d,a,ps,nx,ns);}
        },
    }
}

/// Equality of complete observations on the supported policy domain. No input
/// validity, minimum-size admissibility or non-error antecedent is needed.
pub proof fn theorem_ml_st(m:int,d:Seq<u8>,a:int,ps:LzwParams)
    requires 1<=m<=11,m<=8 || ps.checked_literals,
    ensures ml::ML(m,d,a,ps)==ST(m,d,a,ps),
{
    if ml::m_ok(ps,m,a) {
        lemma_initial(m);
        lemma_fold(m,d,a,ps,ml::init_st(m),initial(m));
    }
}

pub proof fn lemma_powers(m:int)
    requires 0<=m<=12,
    ensures crate::chain::ipow2(m)==ml::lzw_clear_of(m),
    decreases m,
{
    reveal(crate::chain::ipow2);reveal(ml::lzw_ipow2);
    if m>0 {lemma_powers(m-1);}
}

/// The frozen byte-readable domain with E4 applied. This lemma reaches the
/// actual 20,480-byte reservation through the raw prefix/suffix/length views.
pub proof fn theorem_representation(t:Seq<u8>,m:int,x:ml::St,s:State,c:int)
    requires 1<=m<=11,relates(m,x,s),readable(m,x.next,c),
        crate::lzw::table_len_ok(t),
        crate::dictionary::table_ok(crate::lzw::raw_pf_seq(t),crate::lzw::raw_ls_seq(t),ml::lzw_clear_of(m),x.next),
        crate::lzw::table_agrees(t,x.pf,x.sf,ml::lzw_clear_of(m),x.next-1),
    ensures crate::lzw::chain(t,m,c,4096)==s.table[c],
        s.table[c].len()==crate::chain::code_len(t,m,c),
        (s.table[c][0] as int)==crate::lzw::code_first(t,m,c,4096),
        (s.table[c][s.table[c].len() as int-1] as int)==crate::lzw::code_suffix(t,m,c),
        1<=s.table[c].len()<=4096,
{
    lemma_powers(m);
    lemma_read(m,x,s,c);
    let clear=ml::lzw_clear_of(m);
    assert(crate::lzw::table_agrees(t,x.pf,x.sf,clear,c));
    crate::lzw::lemma_walk_agrees(t,x.pf,x.sf,clear,c,4096);
    crate::lzw::lemma_raw_views(t);
    assert forall|i:int| clear+2<=i<=c implies
        #[trigger] crate::lzw::raw_pf_seq(t)[i]==x.pf[i] && crate::lzw::raw_sf_seq(t)[i]==x.sf[i] by {
        assert(crate::lzw::raw_prefix(t,i)==x.pf[i] as int);
        assert(crate::lzw::raw_suffix(t,i)==x.sf[i] as int);
    }
    crate::refinement::table_agreement(x.sf,x.pf,crate::lzw::raw_sf_seq(t),crate::lzw::raw_pf_seq(t),clear,c,4096);
    crate::dictionary::chain_valid(crate::lzw::raw_pf_seq(t),crate::lzw::raw_ls_seq(t),clear,x.next,c);
    crate::refinement::cache_shape(crate::lzw::raw_sf_seq(t),crate::lzw::raw_pf_seq(t),crate::lzw::raw_ls_seq(t),clear,c,4096);
    assert forall|i:int| clear+2<=i<=c && (x.pf[i] as int)<clear implies #[trigger] x.pf[i]<256 by {
        assert(crate::chain::link_ok(x.pf,clear,i));
    }
    crate::refinement::checked_root(x.pf,clear,c,4096);
    crate::chain::lemma_root_literal(x.pf,clear,c,4096);
    if c<clear {
        reveal(crate::chain::str_of);
    } else {
        crate::chain::lemma_chain_step(x.sf,x.pf,clear,c,4096);
        assert(crate::lzw::raw_suffix(t,c)==x.sf[c] as int);
    }
}

pub closed spec fn represented(t:Seq<u8>,m:int,s:State)->bool {
    forall|c:int| readable(m,s.next,c) ==> #[trigger] crate::lzw::chain(t,m,c,4096)==s.table[c]
        && s.table[c].len()==crate::chain::code_len(t,m,c)
        && (s.table[c][0] as int)==crate::lzw::code_first(t,m,c,4096)
        && (s.table[c][s.table[c].len() as int-1] as int)==crate::lzw::code_suffix(t,m,c)
        && 1<=s.table[c].len()<=4096
}

pub proof fn theorem_all_represented(t:Seq<u8>,m:int,x:ml::St,s:State)
    requires 1<=m<=11,relates(m,x,s),crate::lzw::table_len_ok(t),
        crate::dictionary::table_ok(crate::lzw::raw_pf_seq(t),crate::lzw::raw_ls_seq(t),ml::lzw_clear_of(m),x.next),
        crate::lzw::table_agrees(t,x.pf,x.sf,ml::lzw_clear_of(m),x.next-1),
    ensures represented(t,m,s),
{
    reveal(represented);
    assert forall|c:int| readable(m,s.next,c) implies #[trigger] crate::lzw::chain(t,m,c,4096)==s.table[c]
        && s.table[c].len()==crate::chain::code_len(t,m,c)
        && (s.table[c][0] as int)==crate::lzw::code_first(t,m,c,4096)
        && (s.table[c][s.table[c].len() as int-1] as int)==crate::lzw::code_suffix(t,m,c)
        && 1<=s.table[c].len()<=4096 by {
        theorem_representation(t,m,x,s,c);
    }
}

/// A directly computed independent-dictionary witness, before consulting any
/// prefix chain: the second literal creates entry 6 = [1,2].
pub proof fn lemma_literal_pair()
    ensures emit(emit(initial(2),3,1,seq![1u8]),6,2,seq![2u8]).table[6]==seq![1u8,2u8],
        emit(emit(initial(2),3,1,seq![1u8]),6,2,seq![2u8]).next==7,
{
    assert(emit(emit(initial(2),3,1,seq![1u8]),6,2,seq![2u8]).table[6]==seq![1u8,2u8]) by(compute);
}

pub proof fn lemma_literal_pair_steps()
    ensures step(2,seq![17u8],4,crate::lzw::strict_params(),initial(2))
            ==Step::Continue(emit(initial(2),3,1,seq![1u8])),
        step(2,seq![17u8],4,crate::lzw::strict_params(),emit(initial(2),3,1,seq![1u8]))
            ==Step::Continue(emit(emit(initial(2),3,1,seq![1u8]),6,2,seq![2u8])),
        emit(emit(initial(2),3,1,seq![1u8]),6,2,seq![2u8]).table[6]==seq![1u8,2u8],
{
    assert(ml::code_at(seq![17u8],0,3,0)==1) by(compute);
    assert(ml::code_at(seq![17u8],3,3,0)==2) by(compute);
    assert(ml::lzw_clear_of(2)==4) by(compute);
    assert(initial(2).table[1]==seq![1u8]) by(compute);
    assert(initial(2).table[2]==seq![2u8]) by(compute);
    lemma_literal_pair();
}

/// Complete independent observation for codes 1,2,6,EOI, with the final
/// width change made explicit by the payload (3+3+3+4 code bits).
pub proof fn lemma_literal_pair_complete()
    ensures ST(2,seq![145u8,11u8],4,crate::lzw::strict_params())
        ==ml::observation(seq![1u8,2u8,1u8,2u8],None,13,false),
{
    assert(ST(2,seq![145u8,11u8],4,crate::lzw::strict_params())
        ==ml::observation(seq![1u8,2u8,1u8,2u8],None,13,false)) by(compute);
}

/// KwKwK resolves from the previous string and then stores that same string
/// at next; neither operation consults a prefix representation.
pub proof fn lemma_kwkwk(s:State,after:int)
    requires s.prev is Some,s.prev.unwrap().len()>=1,0<=s.next<4096,s.table.len()==4096,
    ensures resolve(s,s.next)==s.prev.unwrap().push(s.prev.unwrap()[0]),
        emit(s,after,s.next,resolve(s,s.next)).table[s.next]==resolve(s,s.next),
{}

/// Full-table non-Clear transitions leave the independent dictionary frozen;
/// Clear alone reinitializes it. This is G10's deferred-clear rule.
pub proof fn lemma_full(m:int,d:Seq<u8>,a:int,ps:LzwParams,s:State,ns:State)
    requires s.next==4096,s.width==12,
        ml::code_at(d,s.pos,s.width,0)!=ml::lzw_clear_of(m),
        step(m,d,a,ps,s)==Step::Continue(ns),
    ensures ns.next==4096,ns.width==12,ns.table==s.table,
{}

} // verus!
