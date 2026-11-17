//! Policy theorems over the independent models and real public-call contracts.
use vstd::prelude::*;
#[cfg(verus_only)]
use crate::{model as pm,frame_model as fm,lzw_model as ml};
#[cfg(verus_only)]
use crate::policy::*;
#[cfg(verus_only)]
use crate::gif::Still;
#[cfg(verus_only)]
use crate::alloc_guard::sites::TryError;

verus! {

pub proof fn lemma_strict_tail_no_e4(m:int,d:Seq<u8>,a:int,ps:crate::lzw::LzwParams,s:ml::St)
    requires !ps.stop_at_raster,
    ensures !ml::L(m,d,a,ps,s).e4,
    decreases ml::nbits(d)-s.pos,
{
    hide(ml::L);hide(ml::semit);hide(ml::first_of);
    ml::lemma_L_unfold(m,d,a,ps,s);
    match ml::verdict(m,d,a,ps,s) {
        ml::Vd::Step(ns)=>if ns.pos>s.pos {lemma_strict_tail_no_e4(m,d,a,ps,ns);},
        _=>{},
    }
}

pub proof fn lemma_no_e4(m:int,d:Seq<u8>,a:int,ps:crate::lzw::LzwParams)
    requires !ps.stop_at_raster,
    ensures !ml::ML(m,d,a,ps).e4,
{
    hide(ml::L);
    if ml::m_ok(ps,m,a) {lemma_strict_tail_no_e4(m,d,a,ps,ml::init_st(m));}
}

pub proof fn lemma_reject_inside(b:Seq<u8>,h:pm::HeaderM,p:Policy,s:pm::WalkM,fuel:nat,
    fh:pm::FrameHeaderM,g:Option<pm::GceM>,m:int,d:Seq<u8>,walk:pm::WalkM)
    requires p.frame_bounds==FrameBounds::Reject,
        pm::mb_fuel(b,h,p,s,fuel)==Ok(pm::MbOut::Image(fh,g,m,d,walk)),
    ensures fh.inside,
    decreases fuel,
{
    hide(pm::ext_step);hide(pm::data_step);
    reveal_with_fuel(pm::mb_fuel,1);
    if fuel>0 && s.p<b.len() && b[s.p]==0x21 {
        lemma_reject_inside(b,h,p,pm::ext_step(b,p,s).unwrap(),(fuel-1) as nat,fh,g,m,d,walk);
    }
}

pub proof fn lemma_call_no_recovery(b:Seq<u8>,h:pm::HeaderM,p:Policy,s:fm::State,still:bool)
    requires p.frame_bounds==FrameBounds::Reject,p.lzw_tail==LzwTail::Strict,
    ensures match fm::call_mode(b,h,p,s,still) {
        Some(c)=>c.frame is Some ==> !c.frame.unwrap().recovered,
        None=>true,
    },
{
    hide(pm::mb);hide(fm::image_mode);
    if s.status==fm::Status::Running {
        match pm::mb(b,h,p,s.walk) {
            Ok(pm::MbOut::Image(fh,g,m,d,walk))=>{
                assert(pm::mb_fuel(b,h,p,s.walk,(b.len()-s.walk.p) as nat)==Ok(pm::MbOut::Image(fh,g,m,d,walk))) by {reveal(pm::mb);}
                lemma_reject_inside(b,h,p,s.walk,(b.len()-s.walk.p) as nat,fh,g,m,d,walk);
                lemma_no_e4(m,d,fh.w*fh.h,fm::params(p));
                reveal(fm::image_mode);
            },
            _=>{},
        }
    }
}

pub proof fn lemma_still_no_recovery(b:Seq<u8>,p:Policy)
    requires p.frame_bounds==FrameBounds::Reject,p.lzw_tail==LzwTail::Strict,
    ensures match fm::ms(b,p) {Some(Ok(s))=>!s.recovered,_=>true},
{
    hide(pm::mp);hide(pm::mh);hide(fm::call_mode);
    if pm::mp(b,p) is Ok && pm::mh(b) is Ok {
        lemma_call_no_recovery(b,pm::mh(b).unwrap(),p,fm::initial(b,p).unwrap(),true);
    }
}

pub proof fn lemma_stream_no_recovery(b:Seq<u8>,h:pm::HeaderM,p:Policy,s:fm::State,fuel:nat)
    requires p.frame_bounds==FrameBounds::Reject,p.lzw_tail==LzwTail::Strict,
    ensures forall|i:int| 0<=i<fm::stream_fuel(b,h,p,s,fuel).frames.len()
        ==> !#[trigger] fm::stream_fuel(b,h,p,s,fuel).frames[i].recovered,
    decreases fuel,
{
    hide(fm::call);
    reveal_with_fuel(fm::stream_fuel,1);
    if fuel>0 {
        lemma_call_no_recovery(b,h,p,s,false);
        match fm::call(b,h,p,s) {
            Some(c)=>if c.error is None && c.frame is Some {
                assert(fm::call(b,h,p,s)==fm::call_mode(b,h,p,s,false)) by {reveal(fm::call);}
                assert(!c.frame.unwrap().recovered);
                lemma_stream_no_recovery(b,h,p,c.after,(fuel-1) as nat);
                let tail=fm::stream_fuel(b,h,p,c.after,(fuel-1) as nat);
                let fs=fm::stream_fuel(b,h,p,s,fuel).frames;
                assert(fs==seq![c.frame.unwrap()]+tail.frames);
                assert forall|i:int| 0<=i<fs.len() implies !#[trigger] fs[i].recovered by {
                    if i==0 {assert(fs[i]==c.frame.unwrap());}
                    else {assert(fs[i]==tail.frames[i-1]);}
                }
            },
            _=>{},
        }
    }
}

pub(crate) proof fn theorem_strict_no_recovery(b:Seq<u8>)
    ensures match fm::ms(b,STRICT) {Some(Ok(s))=>!s.recovered,_=>true},
        match fm::mf(b,STRICT) {Ok((_,st))=>forall|i:int| 0<=i<st.frames.len() ==> !#[trigger] st.frames[i].recovered,_=>true},
{
    hide(pm::mp);hide(pm::mh);hide(fm::stream_fuel);
    lemma_still_no_recovery(b,STRICT);
    if pm::mp(b,STRICT) is Ok && pm::mh(b) is Ok {
        lemma_stream_no_recovery(b,pm::mh(b).unwrap(),STRICT,fm::initial(b,STRICT).unwrap(),b.len()+1);
    }
}

/// The existing core models are already record-indexed. This bridge identifies
/// Profile::Strict with their core specialization; it does not replace the
/// independent OPTIONS.md transcription required for the next policy layer.
pub(crate) proof fn theorem_strict_model_specialization(b:Seq<u8>,m:int,d:Seq<u8>,a:int)
    ensures pm::mp(b,profile_record(Profile::Strict))==pm::mp(b,STRICT),
        fm::params(profile_record(Profile::Strict))==crate::lzw::strict_params(),
        ml::ML(m,d,a,fm::params(profile_record(Profile::Strict)))==ml::ML(m,d,a,crate::lzw::strict_params()),
        fm::ms(b,profile_record(Profile::Strict))==fm::ms(b,STRICT),
        fm::mf(b,profile_record(Profile::Strict))==fm::mf(b,STRICT),
        match fm::ms(b,profile_record(Profile::Strict)) {Some(Ok(s))=>!s.recovered,_=>true},
        match fm::mf(b,profile_record(Profile::Strict)) {Ok((_,st))=>forall|i:int| 0<=i<st.frames.len() ==> !#[trigger] st.frames[i].recovered,_=>true},
{
    hide(pm::mp);hide(fm::ms);hide(fm::mf);hide(ml::ML);
    lemma_profile_records();
    theorem_strict_no_recovery(b);
}

pub open spec fn same_still(a:Result<Still,TryError>,c:Result<(Still,bool),TryError>)->bool {
    match (a,c) {
        (Ok(s),Ok((t,r)))=>s.width==t.width && s.height==t.height && s.rgba@==t.rgba@ && !r,
        (Err(TryError::Format(e)),Err(TryError::Format(f)))=>e==f,
        _=>false,
    }
}

/// The premises are exactly the existing contracts of two independent calls.
/// Allocation failure is excluded independently for each result, not coupled.
pub proof fn theorem_strict_two_still_calls(b:Seq<u8>,a:Result<Still,TryError>,c:Result<(Still,bool),TryError>)
    requires crate::gif::still_core_result(b,a),
        crate::gif::still_result(b,profile_record(Profile::Strict),c),
        !(a is Err && a->Err_0 is OutOfMemory),!(c is Err && c->Err_0 is OutOfMemory),
    ensures same_still(a,c),
{
    crate::gif::lemma_still_core_observation(b,a);
    crate::gif::lemma_still_policy_observation(b,profile_record(Profile::Strict),c);
    lemma_profile_records();
    theorem_strict_no_recovery(b);
}

/// Executable L-PREFIX corollary: the first actual call succeeds, and the
/// independent replacement call does not fail its allocation requests.
pub proof fn theorem_two_still_calls_prefix(b:Seq<u8>,c:Seq<u8>,a:Result<Still,TryError>,r:Result<Still,TryError>)
    requires crate::gif::still_core_result(b,a),a is Ok,crate::gif::still_core_result(c,r),
        !(r is Err && r->Err_0 is OutOfMemory),
        crate::prefix_model::agrees(b,c,crate::prefix_model::first_terminator(b,profile_record(Profile::Strict))),
        c.len()<=pm::max_input(),
    ensures r is Ok,r.unwrap().width==a.unwrap().width,r.unwrap().height==a.unwrap().height,
        r.unwrap().rgba@==a.unwrap().rgba@,
{
    crate::gif::lemma_still_core_observation(b,a);
    crate::gif::lemma_still_core_observation(c,r);
    let s=fm::ms(b,profile_record(Profile::Strict)).unwrap().unwrap();
    crate::prefix_model::theorem_successful_still_prefix(b,c,profile_record(Profile::Strict),s);
}

} // verus!
