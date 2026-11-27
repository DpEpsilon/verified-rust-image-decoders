//! T10 composition of complete Strict public contracts with P18 locations.
use vstd::prelude::*;
use crate::{document_model as d,raster_model as r,precedence_model as loc,
    still_model as s,still_equivalence as se,frame_model as f,frame_equivalence as fe,
    iterator_data_model as it,icc_model as icc,png::Error,policy::STRICT,
    alloc_guard::{AllocSite,OutOfMemory,sites::TryError}};
verus! {
pub enum ImagePoint {Walk,Default(loc::DataPoint)}
pub open spec fn default_at(b: Seq<u8>,doc: d::Doc,point: loc::DataPoint,e: Error) -> bool {
    match d::default_stream(b,doc) {
        Some(z)=>loc::data_at(z,r::document_shape(doc,STRICT),STRICT,point,e),_=>false,
    }
}
pub open spec fn still_at(b: Seq<u8>,point: ImagePoint,e: Error) -> bool {
    match d::MW_p(b,d::strict()) {
        d::Outcome::Error(f,_)=>matches!(point,ImagePoint::Walk) && e==f,
        d::Outcome::Done(doc,_)=>match point {ImagePoint::Default(at)=>default_at(b,doc,at,e),_=>false},
        _=>false,
    }
}
pub open spec fn constructor_at(b: Seq<u8>,point: ImagePoint,e: Error) -> bool {
    match d::MWF_p(b,d::strict()) {
        d::Outcome::Error(f,_)=>matches!(point,ImagePoint::Walk) && e==f,
        d::Outcome::Done(doc,_)=>match point {ImagePoint::Default(at)=>default_at(b,it::constructor_doc(doc),at,e),_=>false},
        _=>false,
    }
}
pub proof fn default_first(b: Seq<u8>,doc: d::Doc,e: Error) -> (point: loc::DataPoint)
    requires r::document_data(b,doc,STRICT)==Some(Err(e)),
    ensures default_at(b,doc,point,e),e!=Error::IccProfileTooLarge,
{
    hide(inflate_verus::model::Inflate);
    let z=d::default_stream(b,doc).unwrap();
    loc::data_not_profile_cap(z,r::document_shape(doc,STRICT),STRICT,e);
    loc::data_first(z,r::document_shape(doc,STRICT),STRICT,e)
}
pub proof fn still_first(b: Seq<u8>,wide: bool,e: Error) -> (point: ImagePoint)
    requires s::MS_wide(b,wide)==s::Outcome::Error(e),
    ensures still_at(b,point,e),e!=Error::IccProfileTooLarge,
{
    hide(inflate_verus::model::Inflate);hide(r::document_data);
    d::precedence_walk(b);
    match d::MW_p(b,d::strict()) {
        d::Outcome::Done(doc,_)=>{let at=default_first(b,doc,e);ImagePoint::Default(at)},
        _=>ImagePoint::Walk,
    }
}
pub proof fn still_selected(b: Seq<u8>,wide: bool,point: ImagePoint,e: Error)
    requires still_at(b,point,e),
    ensures s::MS_wide(b,wide)==s::Outcome::Error(e),
{
    hide(inflate_verus::model::Inflate);
    if let d::Outcome::Done(doc,_)=d::MW_p(b,d::strict()) {
        if let ImagePoint::Default(at)=point {
            loc::data_selected(d::default_stream(b,doc).unwrap(),r::document_shape(doc,STRICT),STRICT,at,e);
        }
    }
}
pub proof fn constructor_first(b: Seq<u8>,e: Error) -> (point: ImagePoint)
    requires f::MFr(b)==f::Outcome::Error(e),
    ensures constructor_at(b,point,e),e!=Error::IccProfileTooLarge,
{
    hide(inflate_verus::model::Inflate);hide(r::document_data);
    d::precedence_walk(b);
    match d::MWF_p(b,d::strict()) {
        d::Outcome::Done(doc,_)=>{let at=default_first(b,it::constructor_doc(doc),e);ImagePoint::Default(at)},
        _=>ImagePoint::Walk,
    }
}
pub proof fn constructor_selected(b: Seq<u8>,point: ImagePoint,e: Error)
    requires constructor_at(b,point,e),
    ensures f::MFr(b)==f::Outcome::Error(e),
{
    hide(inflate_verus::model::Inflate);
    if let d::Outcome::Done(original,_)=d::MWF_p(b,d::strict()) {
        let doc=it::constructor_doc(original);
        if let ImagePoint::Default(at)=point {
            loc::data_selected(d::default_stream(b,doc).unwrap(),r::document_shape(doc,STRICT),STRICT,at,e);
        }
    }
}
/// The arbitrary Error parameter covers every member of P18's closed set.
pub proof fn still_error_iff(b: Seq<u8>,wide: bool,result: Result<s::Value,TryError>,e: Error)
    requires se::eq(b,wide,result),!matches!(result,Err(TryError::OutOfMemory(_))),
    ensures (result==Err(TryError::Format(e)))==(s::MS_wide(b,wide)==s::Outcome::Error(e)),
        (result==Err(TryError::Format(e)))==(exists|point: ImagePoint| still_at(b,point,e)),
        result!=Err(TryError::Format(Error::IccProfileTooLarge)),
{
    hide(d::MW_p);hide(inflate_verus::model::Inflate);
    if s::MS_wide(b,wide)==s::Outcome::Error(e) {let point=still_first(b,wide,e);}
    if exists|point: ImagePoint| still_at(b,point,e) {
        let point=choose|point: ImagePoint| still_at(b,point,e);still_selected(b,wide,point,e);
    }
    if let Err(TryError::Format(actual))=result {let point=still_first(b,wide,actual);}
}
pub proof fn constructor_error_iff<T>(b: Seq<u8>,result: Result<T,TryError>,e: Error)
    requires fe::constructor_eq(b,result),!matches!(result,Err(TryError::OutOfMemory(_))),
    ensures (result==Err(TryError::Format(e)))==(f::MFr(b)==f::Outcome::Error(e)),
        (result==Err(TryError::Format(e)))==(exists|point: ImagePoint| constructor_at(b,point,e)),
        result!=Err(TryError::Format(Error::IccProfileTooLarge)),
{
    hide(d::MWF_p);hide(inflate_verus::model::Inflate);fe::observe_constructor(b,result);
    if f::MFr(b)==f::Outcome::Error(e) {let point=constructor_first(b,e);}
    if exists|point: ImagePoint| constructor_at(b,point,e) {
        let point=choose|point: ImagePoint| constructor_at(b,point,e);constructor_selected(b,point,e);
    }
    if let Err(TryError::Format(actual))=result {let point=constructor_first(b,actual);}
}
/// A Raw/Output failure has no walk point to displace; both requests precede data.
pub proof fn still_oom(b: Seq<u8>,wide: bool,o: OutOfMemory,log: Seq<(AllocSite,nat,bool)>)
    requires se::G(b,wide,Err(TryError::OutOfMemory(o)),log),
    ensures matches!(d::MW_p(b,d::strict()),d::Outcome::Done(_,_)),
        o.site_spec()==AllocSite::Raw || o.site_spec()==AllocSite::Output,
        forall|e: Error| s::MS_wide(b,wide)==s::Outcome::Error(e) ==>
            exists|point: loc::DataPoint| still_at(b,ImagePoint::Default(point),e),
{
    hide(d::MW_p);hide(inflate_verus::model::Inflate);
    se::failure_member(se::requests(b,wide),Err::<s::Value,TryError>(TryError::OutOfMemory(o)),log);
    match d::MW_p(b,d::strict()) {
        d::Outcome::Done(doc,_)=>{
            assert(se::requests(b,wide)==crate::allocation_model::image_requests(doc,wide));
            assert(se::requests(b,wide).contains((o.site_spec(),o.elements_spec() as nat)));
            assert forall|e: Error| s::MS_wide(b,wide)==s::Outcome::Error(e) implies
                exists|point: loc::DataPoint| still_at(b,ImagePoint::Default(point),e) by {
                let at=still_first(b,wide,e);
            }
        },_=>{assert(se::requests(b,wide).len()==0);assert(log.len()==0);},
    }
}
pub proof fn constructor_oom<T>(b: Seq<u8>,o: OutOfMemory,log: Seq<(AllocSite,nat,bool)>)
    requires fe::G_new(b,Err::<T,TryError>(TryError::OutOfMemory(o)),log),
    ensures matches!(d::MWF_p(b,d::strict()),d::Outcome::Done(_,_)),
        o.site_spec()==AllocSite::Raw || o.site_spec()==AllocSite::Output,
        forall|e: Error| f::MFr(b)==f::Outcome::Error(e) ==>
            exists|point: loc::DataPoint| constructor_at(b,ImagePoint::Default(point),e),
{
    hide(d::MWF_p);hide(inflate_verus::model::Inflate);
    se::failure_member(fe::requests(b),Err::<T,TryError>(TryError::OutOfMemory(o)),log);
    match d::MWF_p(b,d::strict()) {
        d::Outcome::Done(doc,_)=>{
            assert(fe::requests(b).contains((o.site_spec(),o.elements_spec() as nat)));
            assert forall|e: Error| f::MFr(b)==f::Outcome::Error(e) implies
                exists|point: loc::DataPoint| constructor_at(b,ImagePoint::Default(point),e) by {
                let at=constructor_first(b,e);
            }
        },_=>{assert(fe::requests(b).len()==0);assert(log.len()==0);},
    }
}
pub open spec fn frame_at(b: Seq<u8>,doc: d::Doc,k: nat,point: loc::DataPoint,e: Error) -> bool {
    it::data_ok(r::document_data(b,doc,STRICT)) && k<it::count(doc) && it::prefix(b,doc,STRICT,k)
    && if doc.actl.is_none() {default_at(b,doc,point,e)} else {
        match d::frame_stream(b,doc,k) {Some(z)=>loc::data_at(z,it::frame_shape(doc,STRICT,k),STRICT,point,e),_=>false}
    }
}
pub proof fn frame_first(b: Seq<u8>,doc: d::Doc,k: nat,e: Error) -> (point: loc::DataPoint)
    requires it::data_ok(r::document_data(b,doc,STRICT)),it::prefix(b,doc,STRICT,k),
        it::frame_data(b,doc,STRICT,k)==Some(Err(e)),
    ensures frame_at(b,doc,k,point,e),e!=Error::IccProfileTooLarge,
{
    hide(inflate_verus::model::Inflate);it::frame_rule(b,doc,STRICT,k);
    if doc.actl.is_none() {default_first(b,doc,e)} else {
        let z=d::frame_stream(b,doc,k).unwrap();let a=it::frame_shape(doc,STRICT,k);
        loc::data_not_profile_cap(z,a,STRICT,e);loc::data_first(z,a,STRICT,e)
    }
}
/// Every earlier frame succeeded; the first terminal error fixes the frame index.
pub proof fn iterator_error(b: Seq<u8>,doc: d::Doc,h: it::History,call: fe::Call,e: Error)
    requires it::history_wf(b,doc,STRICT,h),fe::call_eq(b,doc,h,call),
        matches!(h.phase,it::Phase::Active),!matches!(call,fe::Call::Oom(_)),
        it::from_walk(b,doc,STRICT),
    ensures (call==fe::Call::Format(e))==(it::frame_data(b,doc,STRICT,h.returned)==Some(Err(e))),
        call==fe::Call::Format(e) ==> exists|point: loc::DataPoint| frame_at(b,doc,h.returned,point,e),
        call!=fe::Call::Format(Error::IccProfileTooLarge),
{
    hide(d::MWF_p);hide(inflate_verus::model::Inflate);
    it::history_context(b,doc,STRICT,h);fe::observe_call(b,doc,h,call);fe::document_projection(b,doc);
    it::frame_rule(b,doc,STRICT,h.returned);
    fe::suffix_after_prefix(b,doc,0,h.returned);it::suffix_step(b,doc,STRICT,h.returned);
    if h.returned<it::count(doc) {it::summary_bounds(b,doc,STRICT,h.returned+1);}
    if let fe::Call::Format(actual)=call {let at=frame_first(b,doc,h.returned,actual);}
}
/// Saved can suppress its own frame or a later frame, never an earlier failure.
pub proof fn saved_oom(b: Seq<u8>,doc: d::Doc,h: it::History,o: OutOfMemory,n: nat,e: Error)
    requires it::history_wf(b,doc,STRICT,h),it::from_walk(b,doc,STRICT),
        matches!(h.phase,it::Phase::Active),fe::call_eq(b,doc,h,fe::Call::Oom(o)),
    ensures o.site_spec()==AllocSite::Saved,it::SavedAt_p(b,STRICT)==Some(h.returned),
        it::is_saved_at(b,doc,STRICT,h.returned),it::data_ok(r::document_data(b,doc,STRICT)),
        forall|j: nat| j<h.returned ==> #[trigger] it::frame_ok(b,doc,STRICT,j),
        (it::frame_data(b,doc,STRICT,n)==Some(Err(e)) ==> h.returned<=n),
{
    hide(d::MWF_p);hide(r::document_data);hide(inflate_verus::model::Inflate);
    it::history_context(b,doc,STRICT,h);fe::observe_call(b,doc,h,fe::Call::Oom(o));
    it::default_intro(b,doc,STRICT);it::canonical_saved_at(b,doc,STRICT);
    assert(h.calls==h.returned);
    assert(it::SavedAt(b,doc,STRICT)==Some(h.returned));
    assert(it::is_saved_at(b,doc,STRICT,h.returned));
    if n<h.returned {
        assert(it::frame_ok(b,doc,STRICT,n));
        assert(it::data_ok(it::frame_data(b,doc,STRICT,n)));
    }
}
pub enum IccPoint {Walk,Measure}
pub open spec fn icc_at(b: Seq<u8>,point: IccPoint,e: Error) -> bool {
    match d::MW_p(b,d::strict()) {
        d::Outcome::Error(f,_)=>matches!(point,IccPoint::Walk) && e==f,
        d::Outcome::Done(doc,_)=>matches!(point,IccPoint::Measure) && doc.iccp.is_some()
            && icc::data(icc::stream(b,doc))==icc::Outcome::Error(e),
        _=>false,
    }
}
pub proof fn icc_error_iff(b: Seq<u8>,result: Result<Option<Seq<u8>>,TryError>,log: Seq<(AllocSite,nat,bool)>,e: Error)
    requires icc::G(b,result,log),!matches!(result,Err(TryError::OutOfMemory(_))),
    ensures (result==Err(TryError::Format(e)))==(icc::MI(b)==icc::Outcome::Error(e)),
        (result==Err(TryError::Format(e)))==(exists|point: IccPoint| icc_at(b,point,e)),
        result==Err(TryError::Format(Error::IccProfileTooLarge)) ==> icc_at(b,IccPoint::Measure,Error::IccProfileTooLarge),
{
    hide(inflate_verus::model::Inflate);d::precedence_walk(b);
    if icc::MI(b)==icc::Outcome::Error(e) {
        match d::MW_p(b,d::strict()) {
            d::Outcome::Error(_,_)=>{assert(icc_at(b,IccPoint::Walk,e));},
            _=>{assert(icc_at(b,IccPoint::Measure,e));},
        }
    }
}
/// IccWindow precedes measurement; Icc follows a successful measuring pass.
pub proof fn icc_oom(b: Seq<u8>,o: OutOfMemory,log: Seq<(AllocSite,nat,bool)>)
    requires icc::G(b,Err(TryError::OutOfMemory(o)),log),
    ensures o.site_spec()==AllocSite::IccWindow || o.site_spec()==AllocSite::Icc,
        match d::MW_p(b,d::strict()) {
            d::Outcome::Done(doc,_)=>doc.iccp.is_some() &&
                (o.site_spec()==AllocSite::Icc ==> {
                    let z=icc::stream(b,doc);let measured=icc::measured(z);
                    icc::MI(b)==icc::Outcome::Profile(measured.out)
                    && inflate_verus::model::Inflate(z,measured.out.len(),true,false).status==inflate_verus::model::InflateStatus::Done
                    && inflate_verus::model::Inflate(z,measured.out.len(),true,false).out==measured.out
                }),_=>false,
        },
{
    hide(d::MW_p);hide(inflate_verus::model::Inflate);
    se::failure_member(icc::requests(b),Err::<Option<Seq<u8>>,TryError>(TryError::OutOfMemory(o)),log);
    assert(icc::requests(b).contains((o.site_spec(),o.elements_spec() as nat)));
    if let d::Outcome::Done(doc,_)=d::MW_p(b,d::strict()) {
        if o.site_spec()==AllocSite::Icc {icc::exact_second_pass(icc::stream(b,doc));}
    }
}
/// The public terminal-step contracts imply no second error over any finite tail.
pub proof fn no_second_error(histories: Seq<it::History>,calls: Seq<fe::Call>,
    logs: Seq<Seq<(AllocSite,nat,bool)>>,work: Seq<crate::frame_activity::Work>,start: nat)
    requires histories.len()==calls.len()+1,logs.len()==histories.len(),work.len()==calls.len(),
        start<=calls.len(),!fe::active_history(histories[start as int]),
        forall|j: int| #![trigger histories[j]] 0<=j<calls.len() ==> !fe::active_history(histories[j]) ==>
            histories[j+1].phase==histories[j].phase && fe::call_is_end(calls[j])
                && logs[j+1]==logs[j] && crate::frame_activity::idle(work[j]),
    ensures forall|j: int| #![trigger calls[j]] start<=j<calls.len() ==>
        fe::call_is_end(calls[j]) && logs[j+1]==logs[start as int] && crate::frame_activity::idle(work[j]),
{
    fe::finite_fusion(histories,calls,logs,work,start);
    assert forall|j: int| #![trigger calls[j]] start<=j<calls.len() implies
        fe::call_is_end(calls[j]) && logs[j+1]==logs[start as int] && crate::frame_activity::idle(work[j]) by {
        assert(fe::call_is_end(calls[j]));
        assert(logs[j+1]==logs[start as int]);
        assert(crate::frame_activity::idle(work[j]));
    }
}
}
