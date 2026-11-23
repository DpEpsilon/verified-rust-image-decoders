//! Complete Strict constructor and arbitrary-call iterator relations.
use vstd::prelude::*;
use crate::{document_model as d,raster_model as r,iterator_data_model as data,
    iterator_canvas_model as old,frame_canvas_model as canvas,frame_model as m,
    allocation_model as alloc,policy::STRICT,alloc_guard::{AllocSite,OutOfMemory,sites::TryError}};
verus! {
pub open spec fn requests(b: Seq<u8>) -> Seq<(AllocSite,nat)> {
    match d::MWF_p(b,d::strict()) {d::Outcome::Done(doc,_)=>alloc::image_requests(doc,false),_=>Seq::empty()}
}
pub open spec fn request_bounds(b: Seq<u8>) -> bool {
    match d::MWF_p(b,d::strict()) {
        d::Outcome::Done(doc,_)=>{
            let req=alloc::image_requests(doc,false);
            2<=req[0].1<=2*(crate::limits::MAX_OUTPUT as nat)+crate::limits::MAX_OUTPUT as nat/2+2
            && req[0].1<=crate::alloc_guard::MAX_BYTES
            && 1<=req[1].1<=crate::limits::MAX_OUTPUT && req[1].1<=crate::alloc_guard::MAX_BYTES
        },_=>true,
    }
}
#[verifier::opaque]
pub open spec fn constructor_eq<T>(b: Seq<u8>,result: Result<T,TryError>) -> bool {
    match result {
        Err(TryError::OutOfMemory(_))=>true,
        Err(TryError::Format(e))=>m::MFr(b)==m::Outcome::Error(e),
        Ok(_)=>matches!(m::MFr(b),m::Outcome::Done(_)),
    }
}
pub proof fn constructor_complete<T>(b: Seq<u8>,result: Result<T,TryError>)
    requires data::constructor_eq(b,STRICT,result),
    ensures constructor_eq(b,result),
{
    hide(d::MWF_p);hide(r::document_data);
    crate::walk_policy::profiles_project_exactly();data::constructor_projection(b,STRICT);reveal(constructor_eq);
}
pub open spec fn G_new<T>(b: Seq<u8>,result: Result<T,TryError>,log: Seq<(AllocSite,nat,bool)>) -> bool {
    constructor_eq(b,result) && request_bounds(b) && crate::alloc_guard::sites::log_tied(result,log)
    && crate::alloc_guard::req_of_log(log).is_prefix_of(requests(b))
    && match result {Err(TryError::OutOfMemory(_))=>log.len()>=1,
        _=>crate::alloc_guard::all_ok(log) && crate::alloc_guard::req_of_log(log)==requests(b)}
}
pub open spec fn Prog_new<T>(b: Seq<u8>,result: Result<T,TryError>,log: Seq<(AllocSite,nat,bool)>) -> bool {
    crate::alloc_guard::all_ok(log) ==> !matches!(result,Err(TryError::OutOfMemory(_)))
        && constructor_eq(b,result) && crate::alloc_guard::req_of_log(log)==requests(b)
}
pub proof fn allocation_complete<T>(b: Seq<u8>,result: Result<T,TryError>,log: Seq<(AllocSite,nat,bool)>)
    requires constructor_eq(b,result),request_bounds(b),alloc::image_call(b,d::strict(),true,false,result,log),
    ensures G_new(b,result,log),Prog_new(b,result,log),crate::still_equivalence::only_still_sites(log),
{
    hide(d::MWF_p);hide(constructor_eq);hide(request_bounds);
    assert(log.len()<=2);crate::icc_model::short_log(log);
    match d::MWF_p(b,d::strict()) {
        d::Outcome::Done(doc,_)=>{
            let actual=crate::alloc_guard::req_of_log(log);let expected=alloc::image_requests(doc,false);
            if log.len()>0 {assert(actual[0]==expected[0]);assert(actual[0].0==log[0].0);assert(log[0].0==AllocSite::Raw);}
            if log.len()>1 {assert(actual[1]==expected[1]);assert(actual[1].0==log[1].0);assert(log[1].0==AllocSite::Output);}
        },d::Outcome::Error(_,_)=>{assert(log.len()==0);assert(crate::alloc_guard::all_ok(log));assert(crate::alloc_guard::sites::log_tied(result,log));},_=>{},
    }
}
/// Only the successfully traversed prefix is needed; no future frame is measured.
pub proof fn suffix_after_prefix(b: Seq<u8>,doc: d::Doc,start: nat,n: nat)
    requires data::prefix_from(b,doc,STRICT,start,n),
    ensures data::suffix(b,doc,STRICT,start)==data::suffix(b,doc,STRICT,n),
    decreases n-start,
{
    if start<n {
        assert(data::frame_ok(b,doc,STRICT,start));data::suffix_step(b,doc,STRICT,start);
        assert(data::prefix_from(b,doc,STRICT,start+1,n));suffix_after_prefix(b,doc,start+1,n);
    }
}
#[verifier::opaque]
pub open spec fn wf(b: Seq<u8>,doc: d::Doc,info: d::Info,t: old::Trace,h: data::History) -> bool {
    m::structure(b,doc) && data::from_walk(b,doc,STRICT) && match m::MFr(b) {
        m::Outcome::Done(value)=>info==value.info && h.returned<=value.frames.len()
            && t.after.len()==h.returned
            && (forall|j: int| 0<=j<t.after.len() ==> (#[trigger] t.after[j])==value.frames[j].rgba)
            && match h.phase {
                data::Phase::Active=>h.calls==h.returned,
                data::Phase::Iend=>h.returned==value.frames.len() && value.end==data::End::Iend && h.calls>=h.returned,
                data::Phase::Format(e)=>h.returned==value.frames.len() && value.end==data::End::Format(e) && h.calls>h.returned,
                data::Phase::Oom=>data::SavedAt_p(b,STRICT)==Some(h.returned) && h.calls>h.returned,
            },_=>false,
    }
}
#[verifier::spinoff_prover]
pub proof fn establish(b: Seq<u8>,doc: d::Doc,info: d::Info,t: old::Trace,h: data::History)
    requires m::structure(b,doc),data::from_walk(b,doc,STRICT),data::history_wf(b,doc,STRICT,h),
        old::wf(b,doc,STRICT,t),t.after.len()==h.returned,info==doc.info,
    ensures wf(b,doc,info,t,h),
{
    hide(d::MWF_p);hide(r::document_data);hide(inflate_verus::model::Inflate);
    crate::walk_policy::profiles_project_exactly();data::from_walk_rule(b,doc,STRICT);
    data::history_context(b,doc,STRICT,h);data::default_intro(b,doc,STRICT);
    suffix_after_prefix(b,doc,0,h.returned);data::summary_bounds(b,doc,STRICT,h.returned);
    data::suffix_step(b,doc,STRICT,h.returned);canvas::history_complete(b,doc,t);
    if matches!(h.phase,data::Phase::Oom) {
        data::canonical_saved_at(b,doc,STRICT);data::saved_at_unique(b,doc,STRICT,h.returned,h.returned);
    }
    reveal(wf);
}
pub proof fn context(b: Seq<u8>,doc: d::Doc,info: d::Info,t: old::Trace,h: data::History)
    requires wf(b,doc,info,t,h),
    ensures m::structure(b,doc),data::from_walk(b,doc,STRICT),match m::MFr(b) {
        m::Outcome::Done(value)=>info==value.info && h.returned<=value.frames.len()
            && t.after.len()==h.returned && (forall|j: int| 0<=j<t.after.len() ==> (#[trigger] t.after[j])==value.frames[j].rgba)
            && match h.phase {
                data::Phase::Active=>h.calls==h.returned,
                data::Phase::Iend=>h.returned==value.frames.len() && value.end==data::End::Iend && h.calls>=h.returned,
                data::Phase::Format(e)=>h.returned==value.frames.len() && value.end==data::End::Format(e) && h.calls>h.returned,
                data::Phase::Oom=>data::SavedAt_p(b,STRICT)==Some(h.returned) && h.calls>h.returned,
            },_=>false,
    },
{reveal(wf);}
pub open spec fn info_eq(b: Seq<u8>,info: d::Info) -> bool {
    match m::MFr(b) {m::Outcome::Done(value)=>info==value.info,_=>false}
}
pub proof fn info_complete(b: Seq<u8>,doc: d::Doc,info: d::Info,t: old::Trace,h: data::History)
    requires wf(b,doc,info,t,h),
    ensures info_eq(b,info),
{context(b,doc,info,t,h);}
pub enum Call {Frame(m::Frame),Format(crate::png::Error),End,Oom(OutOfMemory)}
pub open spec fn call_is_oom(call: Call) -> bool {matches!(call,Call::Oom(_))}
pub open spec fn call_is_end(call: Call) -> bool {matches!(call,Call::End)}
pub open spec fn active_history(h: data::History) -> bool {matches!(h.phase,data::Phase::Active)}
pub open spec fn kind(call: Call) -> data::Call {
    match call {Call::Frame(_)=>data::Call::Frame,Call::Format(e)=>data::Call::Format(e),Call::End=>data::Call::End,Call::Oom(_)=>data::Call::Oom}
}
pub open spec fn native_frame(view: crate::png::FrameView<'_>) -> m::Frame {
    view.model_value()
}
pub open spec fn native_call(result: Result<Option<crate::png::FrameView<'_>>,TryError>) -> Call {
    match result {Ok(Some(view))=>Call::Frame(native_frame(view)),Ok(None)=>Call::End,
        Err(TryError::Format(e))=>Call::Format(e),Err(TryError::OutOfMemory(o))=>Call::Oom(o)}
}
#[verifier::opaque]
pub open spec fn call_eq(b: Seq<u8>,doc: d::Doc,h: data::History,call: Call) -> bool {
    if !matches!(h.phase,data::Phase::Active) {matches!(call,Call::End)}else{
        match call {
            Call::Oom(o)=>data::SavedAt_p(b,STRICT)==Some(h.calls) && o.site_spec()==AllocSite::Saved
                && o.elements_spec() as nat==canvas::bytes(doc),
            _=>match m::MFr(b) {
                m::Outcome::Done(value)=>if h.returned<value.frames.len() {
                    match call {Call::Frame(frame)=>frame==value.frames[h.returned as int],_=>false}
                }else if h.returned==value.frames.len() {
                    match value.end {data::End::Iend=>matches!(call,Call::End),data::End::Format(e)=>match call {Call::Format(actual)=>actual==e,_=>false},_=>false}
                }else{false},_=>false,
            },
        }
    }
}
/// A checked walk identifies the complete model's document without measuring
/// any future frame or assuming that future frame data succeeds.
pub proof fn document_projection(b: Seq<u8>,doc: d::Doc)
    requires data::from_walk(b,doc,STRICT),
    ensures m::MFr(b)==m::from_document(b,doc),
{
    hide(d::MWF_p);hide(r::document_data);
    crate::walk_policy::profiles_project_exactly();data::from_walk_rule(b,doc,STRICT);
}
pub open spec fn controls_eq(actual: m::Frame,expected: m::Frame) -> bool {
    actual.rect==expected.rect && actual.delay==expected.delay
    && actual.dispose==expected.dispose && actual.blend==expected.blend
}
#[verifier::spinoff_prover]
pub proof fn call_complete(b: Seq<u8>,doc: d::Doc,info: d::Info,
    before: old::Trace,h: data::History,after: old::Trace,next: data::History,call: Call)
    requires wf(b,doc,info,before,h),wf(b,doc,info,after,next),
        match call {
            Call::Frame(frame)=>matches!(h.phase,data::Phase::Active)
                && next.returned==h.returned+1 && after.after[h.returned as int]==frame.rgba
                && controls_eq(frame,m::record(b,doc,h.returned)),
            Call::Format(e)=>matches!(h.phase,data::Phase::Active)
                && next.returned==h.returned && next.phase==data::Phase::Format(e),
            Call::End=>matches!(h.phase,data::Phase::Active) ==>
                next.returned==h.returned && next.phase==data::Phase::Iend,
            Call::Oom(o)=>matches!(h.phase,data::Phase::Active)
                && data::SavedAt_p(b,STRICT)==Some(h.calls) && o.site_spec()==AllocSite::Saved
                && o.elements_spec() as nat==canvas::bytes(doc),
        },
    ensures call_eq(b,doc,h,call),
{
    hide(d::MWF_p);hide(r::document_data);
    context(b,doc,info,before,h);context(b,doc,info,after,next);
    document_projection(b,doc);reveal(call_eq);
    if let Call::Frame(frame)=call {
        if let m::Outcome::Done(value)=m::MFr(b) {
            assert(after.after[h.returned as int]==value.frames[h.returned as int].rgba);
            assert(value.frames[h.returned as int]==m::record(b,doc,h.returned));
        }
    }
}
pub proof fn canonical_requests(b: Seq<u8>,doc: d::Doc,calls: nat)
    requires data::from_walk(b,doc,STRICT),
    ensures data::requests_p(b,STRICT,calls)==requests(b)+if match data::SavedAt_p(b,STRICT) {Some(k)=>k<calls,None=>false}
        {seq![(AllocSite::Saved,canvas::bytes(doc))]}else{Seq::empty()},
{
    hide(d::MWF_p);
    crate::walk_policy::profiles_project_exactly();data::from_walk_rule(b,doc,STRICT);
    data::requests_canonical(b,doc,STRICT,calls);
}
pub open spec fn G_iter(b: Seq<u8>,doc: d::Doc,info: d::Info,t: old::Trace,h: data::History,log: Seq<(AllocSite,nat,bool)>) -> bool {
    wf(b,doc,info,t,h)
    && crate::alloc_guard::req_of_log(log)==requests(b)+if match data::SavedAt_p(b,STRICT) {Some(k)=>k<h.calls,None=>false}
        {seq![(AllocSite::Saved,canvas::bytes(doc))]}else{Seq::empty()}
    && crate::alloc_guard::all_ok(log)==!data::is_oom(h)
    && (data::is_oom(h) ==> data::SavedAt_p(b,STRICT)==Some(h.returned) && h.calls>h.returned
        && log==seq![(AllocSite::Raw,requests(b)[0].1,true),(AllocSite::Output,canvas::bytes(doc),true),(AllocSite::Saved,canvas::bytes(doc),false)])
}
pub open spec fn Prog_ITER(b: Seq<u8>,doc: d::Doc,h: data::History,call: Call,log: Seq<(AllocSite,nat,bool)>) -> bool {
    crate::alloc_guard::all_ok(log) ==> !matches!(call,Call::Oom(_)) && call_eq(b,doc,h,call)
}
pub proof fn resources_complete(b: Seq<u8>,doc: d::Doc,info: d::Info,t: old::Trace,h: data::History)
    requires wf(b,doc,info,t,h),
    ensures match m::MFr(b) {m::Outcome::Done(value)=>m::resources(value),_=>false},
{
    context(b,doc,info,t,h);document_projection(b,doc);m::complete_resources(b,doc);
}
pub proof fn oom_stays(histories: Seq<data::History>,start: nat,end: nat)
    requires start<=end<histories.len(),data::is_oom(histories[start as int]),
        forall|j: int| 0<=j<histories.len()-1 ==> data::is_oom(#[trigger] histories[j]) ==> data::is_oom(histories[j+1]),
    ensures data::is_oom(histories[end as int]),
    decreases end-start,
{
    if start<end {oom_stays(histories,start,(end-1) as nat);}
}
/// Arbitrary finite call sequences instantiate the public step contracts.
/// A successful cumulative log excludes every earlier OOM, not merely the
/// current result; no fixed bound on the sequence length appears here.
pub proof fn finite_progress(b: Seq<u8>,doc: d::Doc,histories: Seq<data::History>,calls: Seq<Call>,last_log: Seq<(AllocSite,nat,bool)>)
    requires histories.len()==calls.len()+1,
        forall|j: int| 0<=j<calls.len() ==> #[trigger] call_eq(b,doc,histories[j],calls[j]),
        forall|j: int| #![trigger calls[j]] 0<=j<calls.len() ==> call_is_oom(calls[j]) ==> data::is_oom(histories[j+1]),
        forall|j: int| 0<=j<calls.len() ==> data::is_oom(#[trigger] histories[j]) ==> data::is_oom(histories[j+1]),
        crate::alloc_guard::all_ok(last_log)==!data::is_oom(histories.last()),
    ensures crate::alloc_guard::all_ok(last_log) ==> forall|j: int| #![trigger calls[j]] 0<=j<calls.len() ==>
        !call_is_oom(calls[j]) && call_eq(b,doc,histories[j],calls[j]),
{
    if crate::alloc_guard::all_ok(last_log) {
        assert forall|j: int| #![trigger calls[j]] 0<=j<calls.len() implies !call_is_oom(calls[j])
            && call_eq(b,doc,histories[j],calls[j]) by {
            if call_is_oom(calls[j]) {oom_stays(histories,(j+1) as nat,calls.len());}
        }
    }
}
pub proof fn returned_record(b: Seq<u8>,doc: d::Doc,h: data::History,frame: m::Frame)
    requires data::from_walk(b,doc,STRICT),call_eq(b,doc,h,Call::Frame(frame)),
    ensures frame==m::record(b,doc,h.returned),
{
    document_projection(b,doc);reveal(call_eq);
}
pub proof fn returned_resources(b: Seq<u8>,doc: d::Doc,h: data::History,frame: m::Frame)
    requires m::structure(b,doc),data::from_walk(b,doc,STRICT),call_eq(b,doc,h,Call::Frame(frame)),
    ensures frame.rgba.len()==canvas::bytes(doc)<=crate::limits::MAX_OUTPUT,
{
    returned_record(b,doc,h,frame);canvas::lengths(b,doc,h.returned+1);
}
pub proof fn returned_untouched(b: Seq<u8>,doc: d::Doc,h: data::History,frame: m::Frame,x: nat,y: nat)
    requires data::from_walk(b,doc,STRICT),call_eq(b,doc,h,Call::Frame(frame)),
        x<doc.ihdr.width,y<doc.ihdr.height,!m::covered_pixel(doc,h.returned+1,x,y),
    ensures forall|channel: int| 0<=channel<4 ==>
        (#[trigger] frame.rgba[4*(y*(doc.ihdr.width as nat)+x)+channel])==0u8,
{
    returned_record(b,doc,h,frame);m::untouched_pixel(b,doc,h.returned,x,y);
}
/// Once a checked terminal transition occurs, every later call preserves its
/// exact log and performs none of the observed native work-entry operations.
pub proof fn finite_fusion(histories: Seq<data::History>,calls: Seq<Call>,logs: Seq<Seq<(AllocSite,nat,bool)>>,
    work: Seq<crate::frame_activity::Work>,start: nat)
    requires histories.len()==calls.len()+1,logs.len()==histories.len(),work.len()==calls.len(),
        start<=calls.len(),!active_history(histories[start as int]),
        forall|j: int| #![trigger histories[j]] 0<=j<calls.len() ==> !active_history(histories[j]) ==>
            histories[j+1].phase==histories[j].phase && call_is_end(calls[j])
                && logs[j+1]==logs[j] && crate::frame_activity::idle(work[j]),
    ensures forall|j: int| #![trigger calls[j]] start<=j<calls.len() ==> call_is_end(calls[j])
        && logs[j+1]==logs[start as int] && crate::frame_activity::idle(work[j]),
    decreases calls.len()-start,
{
    if start<calls.len() {
        assert(histories[start as int+1].phase==histories[start as int].phase);
        finite_fusion(histories,calls,logs,work,start+1);
        assert forall|j: int| #![trigger calls[j]] start<=j<calls.len() implies call_is_end(calls[j])
            && logs[j+1]==logs[start as int] && crate::frame_activity::idle(work[j]) by {
            if j==start {}else{assert(start+1<=j);}
        }
    }
}
/// Full public Strict result: the five-field unprofiled record and the
/// additional recovery bit exposed by the native FrameView.
pub open spec fn native_call_eq(b: Seq<u8>,doc: d::Doc,h: data::History,
    result: Result<Option<crate::png::FrameView<'_>>,TryError>) -> bool {
    call_eq(b,doc,h,native_call(result))
    && match result {Ok(Some(view))=>!view.is_recovered(),_=>true}
}
pub open spec fn Prog_ITER_native(b: Seq<u8>,doc: d::Doc,h: data::History,
    result: Result<Option<crate::png::FrameView<'_>>,TryError>,log: Seq<(AllocSite,nat,bool)>) -> bool {
    crate::alloc_guard::all_ok(log) ==> !matches!(result,Err(TryError::OutOfMemory(_)))
        && native_call_eq(b,doc,h,result)
}
}
