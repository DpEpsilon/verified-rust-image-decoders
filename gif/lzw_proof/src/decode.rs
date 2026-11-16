//! Concrete, stack-free G10 decoder and its complete-observation refinement.
use vstd::prelude::*;
use crate::lzw::{Params,LzwError};
use crate::bits::{power,read_code};
use crate::dictionary::insert;
verus! {
#[cfg(verus_only)]
use crate::model::*;
#[cfg(verus_only)]
use crate::model::ipow2;
#[cfg(verus_only)]
use crate::chain::*;
#[cfg(verus_only)]
use crate::refinement::*;
#[cfg(verus_only)]
use crate::dictionary::{table_ok,valid,chain_valid,history,history_empty,history_read,history_advance};

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct Decoded {
    pub written:usize,
    pub error:Option<LzwError>,
    pub pos:u128,
    pub e4:bool,
}
pub open spec fn policy_domain(ps:Params)->bool { 1<=ps.m_lo<=ps.m_hi<=11 }
pub open spec fn result_eq(r:Decoded,mr:MRes,out:Seq<u8>)->bool {
    &&& r.written<=out.len()
    &&& out.take(r.written as int)==mr.idx
    &&& r.pos as int==mr.pos
    &&& r.e4==mr.e4
    &&& match mr.error {
        None=>r.error.is_none(),
        Some(e)=>r.error==Some(crate::lzw::err_of(e)),
    }
}

#[verifier::spinoff_prover]
pub fn lzw_decode(m:u8,d:&[u8],a:usize,prefix:&mut [u16],suffix:&mut [u8],length:&mut [u16],
    out:&mut [u8],ps:Params)->(r:Decoded)
    ensures
        policy_domain(ps) && a<=old(out)@.len()
            && old(prefix)@.len()==4096 && old(suffix)@.len()==4096 && old(length)@.len()==4096
            ==> result_eq(r,ML(m as int,d@,a as int,ps),final(out)@),
        final(out)@.len()==old(out)@.len(),
        r.written<=a,
        r.e4 ==> r.error.is_none() && r.written==a && ps.stop_at_raster,
{
    hide(ipow2);
    hide(L);
    if prefix.len()!=4096 || suffix.len()!=4096 || length.len()!=4096 || a>out.len()
        || ps.m_lo<1 || ps.m_lo>ps.m_hi || ps.m_hi>11 {
        return Decoded{written:0,error:Some(LzwError::BufferTooSmall),pos:0,e4:false};
    }
    if m<ps.m_lo || m>ps.m_hi || (ps.size1_only_one_pixel && m<=1 && a!=1) {
        proof {
            crate::lzw::lemma_m_ok_iff(ps,m,a as int);
            assert(out@.take(0)==Seq::<u8>::empty());
        }
        return Decoded{written:0,error:Some(LzwError::InvalidLzwMinCodeSize),pos:0,e4:false};
    }
    let clear=power(m);
    proof { crate::bits::small_power(m as int); }
    let eoi=clear+1;
    let mut width=m+1;
    proof {
        crate::bits::small_power(width as int);
        crate::bits::power_recurrence(width as int);
        if m==1 {assert(clear==2);}
    }
    let mut next=clear+2;
    let mut have_prev=false;
    let mut prev:usize=0;
    let mut q:usize=0;
    let mut pos:u128=0;
    let total=(d.len() as u128)*8;
    let _cap=out.len();
    let ghost mut st=init_st(m as int);
    proof {
        assert(table_ok(prefix@,length@,clear as int,next as int));
        history_empty(prefix@,length@,clear as int,0,ps.checked_literals);
    }
    loop
        invariant
            1<=m<=11, policy_domain(ps), m_ok(ps,m as int,a as int),
            clear as int==clear_of(m as int), eoi as int==eoi_of(m as int), clear<=2048,
            ipow2(m as int+1)==2*clear,
            m>=2 ==> clear>=4,
            m==1 ==> clear==2,
            2<=width<=12,
            m>=2 ==> (next as int)<ipow2(width as int) || width==12,
            m==1 && a==1 && ps.stop_at_raster ==> next==4 && width==2,
            prefix@.len()==4096, suffix@.len()==4096, length@.len()==4096,
            out@.len()==_cap, _cap==old(out)@.len(), a<=_cap, _cap<=usize::MAX, q<=a,
            d@.len()<=usize::MAX, total as int==nbits(d@), pos<=total,
            table_ok(prefix@,length@,clear as int,next as int),
            have_prev ==> valid(clear as int,next as int,prev as int) && prev<next,
            history(prefix@,length@,clear as int,next as int,q as int,have_prev,prev as int,ps.checked_literals),
            !have_prev ==> next==clear+2,
            st.pos==pos as int, st.width==width as int, st.next==next as int,
            st.prev==if have_prev {prev as int} else {-1},
            st.q==q as int, st.o==out@.take(q as int),
            st.pf.len()==4096, st.sf.len()==4096,
            forall|x:int| clear as int+2<=x && x<next as int ==> #[trigger] prefix@[x]==st.pf[x],
            forall|x:int| clear as int+2<=x && x<next as int ==> #[trigger] suffix@[x]==st.sf[x],
            ML(m as int,d@,a as int,ps)==L(m as int,d@,a as int,ps,st),
        decreases nbits(d@)-pos as int,
    {
        let ghost start_pf=prefix@;
        let ghost start_sf=suffix@;
        proof {
            lemma_L_unfold(m as int,d@,a as int,ps,st);
            assert forall|x:int| clear+2<=x<next implies
                #[trigger] st.pf[x]==start_pf[x] && st.sf[x]==start_sf[x] by {}
        }
        if ps.stop_at_raster && q==a {
            return Decoded{written:q,error:None,pos,e4:false};
        }
        if total-pos<width as u128 {
            let error=if q<a {LzwError::TooFewPixels} else {LzwError::MissingEndOfInformation};
            return Decoded{written:q,error:Some(error),pos,e4:false};
        }
        let code=read_code(d,pos,width);
        let after=pos+width as u128;
        if code==clear {
            proof {
                let ns=clear_st(m as int,st,after as int);
                assert(verdict(m as int,d@,a as int,ps,st)==Vd::Step(ns));
                assert(ns.pos>st.pos);
                st=ns;
            }
            next=clear+2; width=m+1; have_prev=false; prev=0; pos=after;
            proof {history_empty(prefix@,length@,clear as int,q as int,ps.checked_literals);}
            continue;
        }
        if q==a && code!=eoi {
            return Decoded{written:q,error:Some(LzwError::MissingEndOfInformation),pos:after,e4:false};
        }
        if code==eoi {
            if q<a {
                return Decoded{written:q,error:Some(LzwError::TooFewPixels),pos:after,e4:false};
            }
            if !ps.stop_at_raster && (after+7)/8<d.len() as u128 {
                return Decoded{written:q,error:Some(LzwError::DataAfterEndOfInformation),pos:after,e4:false};
            }
            return Decoded{written:q,error:None,pos:after,e4:false};
        }
        if ps.checked_literals && code<clear && code>=256 {
            return Decoded{written:q,error:Some(LzwError::ColorIndexOutOfRange),pos:after,e4:false};
        }
        let kw=code==next;
        if !(code<next || (kw && have_prev && next<4096)) {
            return Decoded{written:q,error:Some(LzwError::InvalidLzwCode),pos:after,e4:false};
        }
        let ghost base=if kw {prev} else {code};
        proof {
            chain_valid(prefix@,length@,clear as int,next as int,base as int);
            table_agreement(suffix@,prefix@,st.sf,st.pf,clear as int,base as int,4096);
            history_read(prefix@,length@,clear as int,next as int,q as int,have_prev,prev as int,
                ps.checked_literals,base as int);
            if m==1 && a==1 && ps.stop_at_raster {
                assert(!have_prev);
                assert(code<clear);
                assert(next==4 && width==2);
            }
        }
        let ghost s=semit(m as int,st,code as int);
        let (n,first)=crate::emission::emit_code(code,clear,next,prev,q,a,suffix,prefix,length,out);
        proof {
            assert(s==crate::emission::resolved(suffix@,prefix@,clear as int,next as int,prev as int,code as int));
            assert(s.len()==n);
        }
        if n>a-q {
            proof {discard_bound(q as int,a as int,n as int);}
            return Decoded{written:a,error:if ps.stop_at_raster {None} else {Some(LzwError::TooManyPixels)},
                pos:after,e4:ps.stop_at_raster};
        }
        let grow=have_prev && next<4096;
        let ghost ns=emit_st(m as int,st,after as int,code as int,s,first,grow);
        proof {
            assert(first==first_of(m as int,st,code as int));
            assert(verdict(m as int,d@,a as int,ps,st)==Vd::Step(ns));
            assert(ns.pos>st.pos);
            if next==4096 {
                assert(!grow);
                if m>=2 {
                    crate::bits::small_power(width as int);
                    assert(width==12);
                    lemma_full_step(m as int,d@,a as int,ps,st,ns);
                }
            }
        }
        let ghost op=prefix@;
        let ghost ol=length@;
        let ghost os=suffix@;
        proof {
            assert forall|x:int| clear+2<=x<st.next implies
                #[trigger] st.pf[x]==op[x] && st.sf[x]==os[x] by {
                assert(st.pf[x]==prefix@[x] && st.sf[x]==suffix@[x]);
            }
        }
        if grow {
            insert(next,prev,first,clear,prefix,suffix,length);
            proof {
                assert(prefix@==op.update(st.next,prev as u16));
                assert(suffix@==os.update(st.next,first));
            }
            next=next+1;
            let ghost old_width=width;
            let threshold=power(width);
            if width<12 && next==threshold {width=width+1;}
            proof {
                crate::bits::small_power(width as int);
                if width!=old_width {
                    crate::bits::power_recurrence(width as int);
                    assert(ipow2(width as int)==2*ipow2(old_width as int));
                }
            }
        }
        proof {
            assert(next as int==st.next+if grow {1int} else {0int});
            assert forall|x:int| clear+2<=x<next implies
                #[trigger] ns.pf[x]==prefix@[x] && ns.sf[x]==suffix@[x] by {
                if grow && x==st.next {
                    assert(ns.pf[x]==prev as u16);
                    assert(ns.sf[x]==first);
                } else {
                    assert(st.pf[x]==op[x] && st.sf[x]==os[x]);
                    assert(prefix@[x]==op[x] && suffix@[x]==os[x]);
                }
            }
            history_advance(op,ol,prefix@,length@,clear as int,st.next,q as int,have_prev,
                prev as int,ps.checked_literals,code as int,n as int);
            st=ns;
        }
        q=q+n; pos=after; have_prev=true; prev=code;
    }
}
/// T-LZW as a concrete-call theorem: unlike the total entry point's guard
/// contract, this entry point has no BufferTooSmall alternative in its claim.
/// Strict and RecommendedBeta are instances of the same Params theorem.
pub fn theorem_t_lzw(m:u8,d:&[u8],a:usize,prefix:&mut [u16],suffix:&mut [u8],length:&mut [u16],
    out:&mut [u8],ps:Params)->(r:Decoded)
    requires policy_domain(ps), a>=1, a<=old(out)@.len(),
        old(prefix)@.len()==4096, old(suffix)@.len()==4096, old(length)@.len()==4096,
    ensures result_eq(r,ML(m as int,d@,a as int,ps),final(out)@),
        r.error!=Some(LzwError::BufferTooSmall), r.written<=a,
{
    let r=lzw_decode(m,d,a,prefix,suffix,length,out,ps);
    proof {
        let mr=ML(m as int,d@,a as int,ps);
        if let Some(e)=mr.error {
            assert(crate::lzw::err_of(e)!=LzwError::BufferTooSmall);
        }
    }
    r
}

} // verus!
