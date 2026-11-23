//! Allocation-free public-call walk gates. Fresh events are initialized here,
//! inside checked code, rather than required of an ordinary Rust caller.
use vstd::prelude::*;
use crate::{walk::Doc,png::Events,policy::Policy,alloc_guard::sites::TryError};
#[cfg(verus_only)]
use crate::{document_model as m,model};

verus! {

pub(crate) open spec fn outcome(r: Result<Doc,TryError>, ev: Events) -> m::Outcome {
    match r {
        Ok(doc)=>m::Outcome::Done(doc.view(),ev.view()),
        Err(TryError::Format(e))=>m::Outcome::Error(e,ev.view()),
        Err(TryError::OutOfMemory(_))=>m::Outcome::OutsideDomain,
    }
}

pub(crate) fn still(b: &[u8], pol: &Policy, ev: &mut Events) -> (r: Result<Doc,TryError>)
    ensures *pol==crate::policy::STRICT ==> final(ev).view()==crate::document_model::no_events(),
        outcome(r,*final(ev))==model::MW_p(b@,pol.walk_view()),
        !matches!(r,Err(TryError::OutOfMemory(_))),
        match r {Ok(doc)=>doc.wf(b@) && doc.ihdr.wf() && doc.source_partition()
            && crate::transport_model::idat_result(b@,doc.view())
            && b.len()<=crate::limits::MAX_INPUT && model::mh(b@).is_ok()
            && doc.info.frame_count==doc.frame_ranges@.len(),Err(_)=>true},
{
    proof {crate::walk_policy::profiles_project_exactly();}
    *ev=Events {r1:false,r3:false,r4:false};
    match crate::walk::still(b,pol,ev) {
        Err(e)=>Err(TryError::Format(e)),
        Ok(doc)=>{assert(model::mh(b@).is_ok());Ok(doc)},
    }
}

pub(crate) fn frames(b: &[u8], pol: &Policy, ev: &mut Events) -> (r: Result<Doc,TryError>)
    ensures *pol==crate::policy::STRICT ==> final(ev).view()==crate::document_model::no_events(),
        outcome(r,*final(ev))==model::MWF_p(b@,pol.walk_view()),
        !matches!(r,Err(TryError::OutOfMemory(_))),
        match r {Ok(doc)=>doc.wf(b@) && doc.ihdr.wf() && b.len()<=crate::limits::MAX_INPUT && crate::frame_state::source_document(b@,doc)
            && crate::transport_model::idat_result(b@,doc.view())
            && (final(ev).r4 ==> crate::frame_transport_model::default_only(doc.view())),Err(_)=>true},
{
    proof {crate::walk_policy::profiles_project_exactly();}
    *ev=Events {r1:false,r3:false,r4:false};
    match crate::walk::frames(b,pol,ev) {
        Err(e)=>Err(TryError::Format(e)),
        Ok(doc)=>{assert(model::mh(b@).is_ok());reveal(crate::frame_state::source_document);Ok(doc)},
    }
}

} // verus!
