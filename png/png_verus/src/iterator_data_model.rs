//! MFr's returned-frame count and terminal-data projection, independent of pixels.
//! SavedAt uses the mathematical data prefix, never a native iterator index.
use vstd::prelude::*;
use crate::{document_model as d,raster_model as r,policy::Policy,png::Error};
verus! {
/// A successful data result is the Ok variant; its unit payload is unobservable.
pub open spec fn data_ok(value: Option<Result<(),Error>>) -> bool {match value {Some(Ok(_))=>true,_=>false}}
pub open spec fn frame_ok(b: Seq<u8>,doc: d::Doc,p: Policy,k: nat) -> bool {data_ok(frame_data(b,doc,p,k))}
pub open spec fn count(doc: d::Doc) -> nat {if doc.actl.is_some() {doc.frames.len()}else{1}}
pub open spec fn frame_shape(doc: d::Doc,p: Policy,k: nat) -> r::Raster {
    r::shape(doc.ihdr,doc.frames[k as int].width as nat,doc.frames[k as int].height as nat,d::entries(doc),p)
}
#[verifier::opaque]
pub open spec fn frame_data(b: Seq<u8>,doc: d::Doc,p: Policy,k: nat) -> Option<Result<(),Error>> {
    if k>=count(doc) {None}else if doc.actl.is_none() {r::document_data(b,doc,p)}else{
        match d::frame_stream(b,doc,k) {Some(z)=>r::data(z,frame_shape(doc,p,k),p),None=>None}
    }
}
pub open spec fn prefix_from(b: Seq<u8>,doc: d::Doc,p: Policy,start: nat,n: nat) -> bool {
    start<=n<=count(doc) && forall|i: nat| start<=i<n ==> #[trigger] frame_ok(b,doc,p,i)
}
pub open spec fn prefix(b: Seq<u8>,doc: d::Doc,p: Policy,n: nat) -> bool {prefix_from(b,doc,p,0,n)}
pub enum End {Iend,Format(Error),OutsideDomain}
pub struct Projection {pub frames: nat,pub end: End}
#[verifier::opaque]
pub open spec fn suffix(b: Seq<u8>,doc: d::Doc,p: Policy,start: nat) -> Projection
    decreases (count(doc)-start),
{
    if start>=count(doc) {Projection {frames:count(doc),end:End::Iend}}
    else {match frame_data(b,doc,p,start) {
        Some(Ok(_))=>suffix(b,doc,p,start+1),
        Some(Err(e))=>Projection {frames:start,end:End::Format(e)},
        None=>Projection {frames:start,end:End::OutsideDomain},
    }}
}
pub open spec fn MFr_data(b: Seq<u8>,doc: d::Doc,p: Policy) -> Projection {suffix(b,doc,p,0)}
pub open spec fn reached(b: Seq<u8>,doc: d::Doc,p: Policy,k: nat) -> bool {
    data_ok(r::document_data(b,doc,p)) && k<count(doc) && prefix(b,doc,p,k)
}
pub open spec fn qualifies(doc: d::Doc,k: nat) -> bool {
    doc.actl.is_some() && 1<=k<doc.frames.len() && doc.frames[k as int].dispose==2
}
pub open spec fn is_saved_at(b: Seq<u8>,doc: d::Doc,p: Policy,k: nat) -> bool {
    reached(b,doc,p,k) && qualifies(doc,k) && forall|j: nat| 1<=j<k ==> !#[trigger] qualifies(doc,j)
}
pub open spec fn SavedAt(b: Seq<u8>,doc: d::Doc,p: Policy) -> Option<nat> {
    if exists|k: nat| is_saved_at(b,doc,p,k) {Some(choose|k: nat| is_saved_at(b,doc,p,k))}else{None}
}
pub proof fn prefix_step(b: Seq<u8>,doc: d::Doc,p: Policy,n: nat)
    requires n<count(doc),prefix(b,doc,p,n),frame_ok(b,doc,p,n),
    ensures prefix(b,doc,p,n+1),
{
    assert forall|i: nat| i<n+1 implies #[trigger] frame_ok(b,doc,p,i) by {
        if i<n {assert(frame_ok(b,doc,p,i));}else{assert(i==n);}
    }
}
pub proof fn prefix_down(b: Seq<u8>,doc: d::Doc,p: Policy,n: nat,k: nat)
    requires prefix(b,doc,p,n),k<=n,
    ensures prefix(b,doc,p,k),
{}
pub proof fn summary_bounds(b: Seq<u8>,doc: d::Doc,p: Policy,start: nat)
    requires start<=count(doc),
    ensures start<=suffix(b,doc,p,start).frames<=count(doc),
    decreases count(doc)-start,
{
    reveal_with_fuel(suffix,2);
    if start<count(doc) {
        match frame_data(b,doc,p,start) {
            Some(Ok(_))=>{summary_bounds(b,doc,p,start+1);assert(suffix(b,doc,p,start)==suffix(b,doc,p,start+1));},
            _=>{assert(suffix(b,doc,p,start).frames==start);},
        }
    }else{assert(suffix(b,doc,p,start).frames==count(doc));}
}
pub proof fn summary_reached(b: Seq<u8>,doc: d::Doc,p: Policy,start: nat,k: nat)
    requires start<=k<count(doc),
    ensures prefix_from(b,doc,p,start,k)==(k<=suffix(b,doc,p,start).frames),
    decreases count(doc)-start,
{
    reveal_with_fuel(suffix,2);summary_bounds(b,doc,p,start);
    if start<k {
        match frame_data(b,doc,p,start) {
            Some(Ok(_))=>{
                assert(frame_ok(b,doc,p,start));
                assert(suffix(b,doc,p,start)==suffix(b,doc,p,start+1));
                summary_reached(b,doc,p,start+1,k);
                if prefix_from(b,doc,p,start+1,k) {
                    assert forall|i: nat| start<=i<k implies #[trigger] frame_ok(b,doc,p,i) by {
                        if i==start {assert(frame_ok(b,doc,p,i));}else{assert(start+1<=i);assert(frame_ok(b,doc,p,i));}
                    }
                }
                if prefix_from(b,doc,p,start,k) {
                    assert forall|i: nat| start+1<=i<k implies #[trigger] frame_ok(b,doc,p,i) by {};
                }
                assert(prefix_from(b,doc,p,start,k)==prefix_from(b,doc,p,start+1,k));
            },
            _=>{assert(suffix(b,doc,p,start).frames==start);assert(!frame_ok(b,doc,p,start));
                if prefix_from(b,doc,p,start,k) {assert(frame_ok(b,doc,p,start));}
                assert(!prefix_from(b,doc,p,start,k));},
        }
    }else{assert(prefix_from(b,doc,p,start,k));}
}
pub proof fn saved_at_projection(b: Seq<u8>,doc: d::Doc,p: Policy,k: nat)
    requires data_ok(r::document_data(b,doc,p)),k<count(doc),
    ensures is_saved_at(b,doc,p,k)==(qualifies(doc,k) && k<=MFr_data(b,doc,p).frames
        && forall|j: nat| 1<=j<k ==> !#[trigger] qualifies(doc,j)),
{summary_reached(b,doc,p,0,k);}
pub proof fn saved_at_unique(b: Seq<u8>,doc: d::Doc,p: Policy,k: nat,j: nat)
    requires is_saved_at(b,doc,p,k),is_saved_at(b,doc,p,j),
    ensures k==j,SavedAt(b,doc,p)==Some(k),
{
    hide(r::document_data);
    if j<k {assert(!qualifies(doc,j));}if k<j {assert(!qualifies(doc,k));}
    let a=choose|a: nat| is_saved_at(b,doc,p,a);
    if a<k {assert(!qualifies(doc,a));}if k<a {assert(!qualifies(doc,k));}
}
/// First-error fusion excludes every later reached mathematical call.
pub proof fn terminal_blocks(b: Seq<u8>,doc: d::Doc,p: Policy,n: nat,k: nat)
    requires n<=k,n<=count(doc),n==count(doc) || !frame_ok(b,doc,p,n),
    ensures n<k || n==count(doc) ==> !reached(b,doc,p,k),
{
    if n<k && k<count(doc) && prefix(b,doc,p,k) {assert(frame_ok(b,doc,p,n));}
}
/// Public-call history records results, not working-state fusion during painting.
pub enum Phase {Active,Iend,Format(Error),Oom}
pub struct History {pub calls: nat,pub returned: nat,pub phase: Phase}
pub open spec fn initial() -> History {History {calls:0,returned:0,phase:Phase::Active}}
pub open spec fn is_oom(h: History) -> bool {matches!(h.phase,Phase::Oom)}
#[verifier::opaque]
pub open spec fn history_wf(b: Seq<u8>,doc: d::Doc,p: Policy,h: History) -> bool {
    data_ok(r::document_data(b,doc,p)) && prefix(b,doc,p,h.returned)
    && match h.phase {
        Phase::Active=>h.calls==h.returned,
        Phase::Iend=>h.returned==count(doc) && h.calls>=h.returned,
        Phase::Format(e)=>h.returned<count(doc) && frame_data(b,doc,p,h.returned)==Some(Err(e)) && h.calls>h.returned,
        Phase::Oom=>is_saved_at(b,doc,p,h.returned) && h.calls>h.returned,
    }
}
pub open spec fn attempted(b: Seq<u8>,doc: d::Doc,p: Policy,calls: nat) -> bool {
    exists|k: nat| k<calls && #[trigger] is_saved_at(b,doc,p,k)
}
pub open spec fn log_for(b: Seq<u8>,doc: d::Doc,p: Policy,h: History,raw: nat,out: nat)
    -> Seq<(crate::alloc_guard::AllocSite,nat,bool)>
{
    crate::iterator_log_model::initial(raw,out)+if attempted(b,doc,p,h.calls) {
        seq![(crate::alloc_guard::AllocSite::Saved,out,!is_oom(h))]
    }else{Seq::empty()}
}
pub proof fn history_context(b: Seq<u8>,doc: d::Doc,p: Policy,h: History)
    requires history_wf(b,doc,p,h),
    ensures data_ok(r::document_data(b,doc,p)),prefix(b,doc,p,h.returned),h.returned<=count(doc),
        match h.phase {
            Phase::Active=>h.calls==h.returned,
            Phase::Iend=>h.returned==count(doc) && h.calls>=h.returned,
            Phase::Format(e)=>h.returned<count(doc) && frame_data(b,doc,p,h.returned)==Some(Err(e)) && h.calls>h.returned,
            Phase::Oom=>is_saved_at(b,doc,p,h.returned) && h.calls>h.returned,
        },
{hide(r::document_data);reveal(history_wf);}
pub proof fn history_initial(b: Seq<u8>,doc: d::Doc,p: Policy)
    requires data_ok(r::document_data(b,doc,p)),
    ensures history_wf(b,doc,p,initial()),
{hide(r::document_data);reveal(history_wf);}
pub proof fn history_success(b: Seq<u8>,doc: d::Doc,p: Policy,h: History,single: bool)
    requires history_wf(b,doc,p,h),matches!(h.phase,Phase::Active),h.returned<count(doc),
        frame_ok(b,doc,p,h.returned),single ==> h.returned+1==count(doc),
    ensures history_wf(b,doc,p,History {calls:h.calls+1,returned:h.returned+1,phase:if single {Phase::Iend}else{Phase::Active}}),
{
    hide(r::document_data);reveal(history_wf);prefix_step(b,doc,p,h.returned);
}
pub proof fn history_format(b: Seq<u8>,doc: d::Doc,p: Policy,h: History,e: Error)
    requires history_wf(b,doc,p,h),matches!(h.phase,Phase::Active),h.returned<count(doc),frame_data(b,doc,p,h.returned)==Some(Err(e)),
    ensures history_wf(b,doc,p,History {calls:h.calls+1,returned:h.returned,phase:Phase::Format(e)}),
{hide(r::document_data);reveal(history_wf);}
pub proof fn history_oom(b: Seq<u8>,doc: d::Doc,p: Policy,h: History)
    requires history_wf(b,doc,p,h),matches!(h.phase,Phase::Active),is_saved_at(b,doc,p,h.returned),
    ensures history_wf(b,doc,p,History {calls:h.calls+1,returned:h.returned,phase:Phase::Oom}),
{hide(r::document_data);reveal(history_wf);}
pub proof fn history_end(b: Seq<u8>,doc: d::Doc,p: Policy,h: History)
    requires history_wf(b,doc,p,h),matches!(h.phase,Phase::Active),h.returned==count(doc),
    ensures history_wf(b,doc,p,History {calls:h.calls+1,returned:h.returned,phase:Phase::Iend}),
{hide(r::document_data);reveal(history_wf);}
pub proof fn history_fused(b: Seq<u8>,doc: d::Doc,p: Policy,h: History)
    requires history_wf(b,doc,p,h),!matches!(h.phase,Phase::Active),
    ensures history_wf(b,doc,p,History {calls:h.calls+1,..h}),!is_saved_at(b,doc,p,h.calls),
{
    hide(r::document_data);reveal(history_wf);
    match h.phase {
        Phase::Format(_)=>terminal_blocks(b,doc,p,h.returned,h.calls),
        Phase::Iend=>terminal_blocks(b,doc,p,h.returned,h.calls),
        Phase::Oom=>{
            if is_saved_at(b,doc,p,h.calls) {saved_at_unique(b,doc,p,h.returned,h.calls);}
        },
        _=>{},
    }
}
pub proof fn attempted_step(b: Seq<u8>,doc: d::Doc,p: Policy,calls: nat)
    ensures attempted(b,doc,p,calls+1)==(attempted(b,doc,p,calls) || is_saved_at(b,doc,p,calls)),
{
    if attempted(b,doc,p,calls+1) {
        let k=choose|k: nat| k<calls+1 && #[trigger] is_saved_at(b,doc,p,k);
        if k<calls {assert(attempted(b,doc,p,calls));}else{assert(k==calls);}
    }
    if attempted(b,doc,p,calls) {
        let k=choose|k: nat| k<calls && #[trigger] is_saved_at(b,doc,p,k);
        assert(attempted(b,doc,p,calls+1));
    }
    if is_saved_at(b,doc,p,calls) {assert(attempted(b,doc,p,calls+1));}
}
pub proof fn current_is_fresh(b: Seq<u8>,doc: d::Doc,p: Policy,calls: nat)
    requires is_saved_at(b,doc,p,calls),
    ensures !attempted(b,doc,p,calls),SavedAt(b,doc,p)==Some(calls),
{
    hide(r::document_data);
    if attempted(b,doc,p,calls) {
        let k=choose|k: nat| k<calls && #[trigger] is_saved_at(b,doc,p,k);
        saved_at_unique(b,doc,p,calls,k);
    }
    saved_at_unique(b,doc,p,calls,calls);
}
/// Composition of the exact held log with the one mathematical call request.
pub proof fn log_step(b: Seq<u8>,doc: d::Doc,p: Policy,h: History,after: History,raw: nat,out: nat)
    requires after.calls==h.calls+1,
        is_saved_at(b,doc,p,h.calls) ==> !is_oom(h),
        !is_saved_at(b,doc,p,h.calls) ==> is_oom(after)==is_oom(h),
    ensures log_for(b,doc,p,after,raw,out)==log_for(b,doc,p,h,raw,out)
        +if is_saved_at(b,doc,p,h.calls) {seq![(crate::alloc_guard::AllocSite::Saved,out,!is_oom(after))]}else{Seq::empty()},
{
    hide(r::document_data);attempted_step(b,doc,p,h.calls);
    if is_saved_at(b,doc,p,h.calls) {current_is_fresh(b,doc,p,h.calls);}
    assert(crate::iterator_log_model::initial(raw,out)+Seq::empty()=~=crate::iterator_log_model::initial(raw,out));
    assert(log_for(b,doc,p,after,raw,out)=~=log_for(b,doc,p,h,raw,out)
        +if is_saved_at(b,doc,p,h.calls) {seq![(crate::alloc_guard::AllocSite::Saved,out,!is_oom(after))]}else{Seq::empty()});
}
pub proof fn frame_rule(b: Seq<u8>,doc: d::Doc,p: Policy,k: nat)
    ensures frame_data(b,doc,p,k)==if k>=count(doc) {None}else if doc.actl.is_none() {r::document_data(b,doc,p)}else{
        match d::frame_stream(b,doc,k) {Some(z)=>r::data(z,frame_shape(doc,p,k),p),None=>None}},
{reveal(frame_data);}

#[verifier::opaque]
pub open spec fn default_success(b: Seq<u8>,doc: d::Doc,p: Policy) -> bool {data_ok(r::document_data(b,doc,p))}
pub open spec fn constructor_doc(doc: d::Doc) -> d::Doc {d::Doc {info:d::constructor_info(doc),..doc}}
pub closed spec fn from_walk(b: Seq<u8>,doc: d::Doc,p: Policy) -> bool {
    match d::MWF_p(b,p.walk_view()) {d::Outcome::Done(original,_)=>doc==constructor_doc(original),_=>false}
}
pub proof fn constructor_default(b: Seq<u8>,doc: d::Doc,p: Policy)
    ensures r::document_data(b,constructor_doc(doc),p)==r::document_data(b,doc,p),
{hide(r::data);}
pub open spec fn result_eq<T>(b: Seq<u8>,doc: d::Doc,p: Policy,k: nat,result: Result<T,crate::alloc_guard::TryError>) -> bool {
    match result {
        Ok(_)=>frame_ok(b,doc,p,k),
        Err(crate::alloc_guard::TryError::Format(e))=>frame_data(b,doc,p,k)==Some(Err(e)),
        _=>false,
    }
}

pub(crate) proof fn from_walk_rule(b: Seq<u8>,doc: d::Doc,p: Policy)
    ensures from_walk(b,doc,p)==match d::MWF_p(b,p.walk_view()) {d::Outcome::Done(original,_)=>doc==constructor_doc(original),_=>false},
{hide(d::MWF_p);reveal(from_walk);}

pub proof fn default_intro(b: Seq<u8>,doc: d::Doc,p: Policy)
    requires data_ok(r::document_data(b,doc,p)),
    ensures default_success(b,doc,p),
{hide(r::document_data);reveal(default_success);}
pub proof fn default_context(b: Seq<u8>,doc: d::Doc,p: Policy)
    requires default_success(b,doc,p),
    ensures data_ok(r::document_data(b,doc,p)),
{hide(r::document_data);reveal(default_success);}

pub proof fn cache_request(b: Seq<u8>,doc: d::Doc,p: Policy,k: nat)
    requires reached(b,doc,p,k),doc.actl.is_some(),
    ensures is_saved_at(b,doc,p,k)==(qualifies(doc,k) && !crate::frame_transport_model::saved_before(doc.frames,k)),
{
    hide(r::document_data);crate::frame_transport_model::saved_before_exact(doc.frames,k);
    if crate::frame_transport_model::saved_before(doc.frames,k) {
        let j=choose|j: nat| 1<=j<k && j<doc.frames.len() && (#[trigger] doc.frames[j as int]).dispose==2;
        assert(qualifies(doc,j));
        if is_saved_at(b,doc,p,k) {assert(!qualifies(doc,j));}
    }else{
        assert forall|j: nat| 1<=j<k implies !#[trigger] qualifies(doc,j) by {
            if qualifies(doc,j) {
                assert(exists|i: nat| 1<=i<k && i<doc.frames.len() && (#[trigger] doc.frames[i as int]).dispose==2);
            }
        }
    }
}

pub proof fn result_context<T>(b: Seq<u8>,doc: d::Doc,p: Policy,k: nat,result: Result<T,crate::alloc_guard::TryError>)
    requires result_eq(b,doc,p,k,result),
    ensures match result {Ok(_)=>frame_ok(b,doc,p,k),Err(crate::alloc_guard::TryError::Format(e))=>frame_data(b,doc,p,k)==Some(Err(e)),_=>false},
{}

/// The P16 data/control projection retains every returned-frame index and the
/// first terminal data error; complete canvas values remain a separate model.
pub enum Outcome {ConstructorError(Error),Frames(Projection),OutsideDomain}
pub closed spec fn MFr_data_p(b: Seq<u8>,p: Policy) -> Outcome {
    match d::MWF_p(b,p.walk_view()) {
        d::Outcome::Error(e,_)=>Outcome::ConstructorError(e),
        d::Outcome::Done(original,_)=>{
            let doc=constructor_doc(original);
            match r::document_data(b,doc,p) {
                Some(Ok(_))=>Outcome::Frames(MFr_data(b,doc,p)),
                Some(Err(e))=>Outcome::ConstructorError(e),
                None=>Outcome::OutsideDomain,
            }
        },
        _=>Outcome::OutsideDomain,
    }
}
pub closed spec fn SavedAt_p(b: Seq<u8>,p: Policy) -> Option<nat> {
    match d::MWF_p(b,p.walk_view()) {
        d::Outcome::Done(original,_)=>{
            let doc=constructor_doc(original);
            if default_success(b,doc,p) {SavedAt(b,doc,p)}else{None}
        },
        _=>None,
    }
}
pub proof fn canonical_saved_at(b: Seq<u8>,doc: d::Doc,p: Policy)
    requires from_walk(b,doc,p),default_success(b,doc,p),
    ensures SavedAt_p(b,p)==SavedAt(b,doc,p),
        match MFr_data_p(b,p) {Outcome::Frames(projection)=>projection==MFr_data(b,doc,p),_=>false},
{
    hide(d::MWF_p);hide(r::document_data);
    reveal(from_walk);reveal(SavedAt_p);reveal(MFr_data_p);
    default_context(b,doc,p);
}
pub proof fn saved_at_count(b: Seq<u8>,doc: d::Doc,p: Policy,k: nat)
    requires from_walk(b,doc,p),default_success(b,doc,p),SavedAt_p(b,p)==Some(k),
    ensures qualifies(doc,k),k<=MFr_data(b,doc,p).frames,
        forall|j: nat| 1<=j<k ==> !#[trigger] qualifies(doc,j),
{
    hide(r::document_data);canonical_saved_at(b,doc,p);default_context(b,doc,p);
    assert(is_saved_at(b,doc,p,k));saved_at_projection(b,doc,p,k);
}
pub proof fn attempted_saved_at(b: Seq<u8>,doc: d::Doc,p: Policy,calls: nat)
    ensures attempted(b,doc,p,calls)==match SavedAt(b,doc,p) {Some(k)=>k<calls,None=>false},
{
    hide(r::document_data);
    if attempted(b,doc,p,calls) {
        let k=choose|k: nat| k<calls && #[trigger] is_saved_at(b,doc,p,k);
        saved_at_unique(b,doc,p,k,k);
    }
    if let Some(k)=SavedAt(b,doc,p) {
        assert(is_saved_at(b,doc,p,k));
        if k<calls {assert(attempted(b,doc,p,calls));}
    }
}
pub open spec fn requests(doc: d::Doc,calls: nat,saved_at: Option<nat>) -> Seq<(crate::alloc_guard::AllocSite,nat)> {
    crate::allocation_model::image_requests(doc,false)+if match saved_at {Some(k)=>k<calls,None=>false} {
        seq![(crate::alloc_guard::AllocSite::Saved,crate::layout_model::output_count(doc.ihdr,false))]
    }else{Seq::empty()}
}
pub proof fn log_properties(b: Seq<u8>,doc: d::Doc,p: Policy,h: History,raw: nat,out: nat)
    requires history_wf(b,doc,p,h),
    ensures crate::alloc_guard::all_ok(log_for(b,doc,p,h,raw,out))==!is_oom(h),
        crate::alloc_guard::req_of_log(log_for(b,doc,p,h,raw,out))
            ==seq![(crate::alloc_guard::AllocSite::Raw,raw),(crate::alloc_guard::AllocSite::Output,out)]
                +if match SavedAt(b,doc,p) {Some(k)=>k<h.calls,None=>false} {seq![(crate::alloc_guard::AllocSite::Saved,out)]}else{Seq::empty()},
{
    hide(r::document_data);history_context(b,doc,p,h);attempted_saved_at(b,doc,p,h.calls);
    if is_oom(h) {assert(attempted(b,doc,p,h.calls));}
    let log=log_for(b,doc,p,h,raw,out);
    assert(log.take(2)=~=crate::iterator_log_model::initial(raw,out));
    crate::iterator_log_model::all_ok_cases(log,raw,out,attempted(b,doc,p,h.calls) && !is_oom(h),is_oom(h));
    let req=seq![(crate::alloc_guard::AllocSite::Raw,raw),(crate::alloc_guard::AllocSite::Output,out)]
        +if attempted(b,doc,p,h.calls) {seq![(crate::alloc_guard::AllocSite::Saved,out)]}else{Seq::empty()};
    assert(crate::alloc_guard::req_of_log(log)=~=req);
}

pub proof fn static_ready(b: Seq<u8>,doc: d::Doc,p: Policy)
    requires doc.actl.is_none(),default_success(b,doc,p),
    ensures frame_ok(b,doc,p,0),count(doc)==1,
{hide(r::document_data);default_context(b,doc,p);frame_rule(b,doc,p,0);}

pub closed spec fn requests_p(b: Seq<u8>,p: Policy,calls: nat) -> Seq<(crate::alloc_guard::AllocSite,nat)> {
    match d::MWF_p(b,p.walk_view()) {
        d::Outcome::Done(original,_)=>requests(constructor_doc(original),calls,SavedAt_p(b,p)),
        _=>Seq::empty(),
    }
}
pub proof fn requests_canonical(b: Seq<u8>,doc: d::Doc,p: Policy,calls: nat)
    requires from_walk(b,doc,p),
    ensures requests_p(b,p,calls)==requests(doc,calls,SavedAt_p(b,p)),
{hide(d::MWF_p);reveal(from_walk);reveal(requests_p);}
pub proof fn request_index(b: Seq<u8>,doc: d::Doc,p: Policy,k: nat)
    ensures is_saved_at(b,doc,p,k)==(SavedAt(b,doc,p)==Some(k)),
{
    hide(r::document_data);
    if is_saved_at(b,doc,p,k) {saved_at_unique(b,doc,p,k,k);}
    if SavedAt(b,doc,p)==Some(k) {assert(is_saved_at(b,doc,p,k));}
}
pub proof fn oom_log(b: Seq<u8>,doc: d::Doc,p: Policy,h: History,raw: nat,out: nat)
    requires history_wf(b,doc,p,h),is_oom(h),
    ensures SavedAt(b,doc,p)==Some(h.returned),
        log_for(b,doc,p,h,raw,out)==seq![(crate::alloc_guard::AllocSite::Raw,raw,true),
            (crate::alloc_guard::AllocSite::Output,out,true),(crate::alloc_guard::AllocSite::Saved,out,false)],
{
    hide(r::document_data);history_context(b,doc,p,h);
    saved_at_unique(b,doc,p,h.returned,h.returned);assert(attempted(b,doc,p,h.calls));
    assert(log_for(b,doc,p,h,raw,out)=~=seq![(crate::alloc_guard::AllocSite::Raw,raw,true),
        (crate::alloc_guard::AllocSite::Output,out,true),(crate::alloc_guard::AllocSite::Saved,out,false)]);
}

pub closed spec fn request_p(b: Seq<u8>,p: Policy,n: nat) -> Seq<(crate::alloc_guard::AllocSite,nat)> {
    match d::MWF_p(b,p.walk_view()) {
        d::Outcome::Done(original,_)=>if SavedAt_p(b,p)==Some(n) {
            seq![(crate::alloc_guard::AllocSite::Saved,crate::layout_model::output_count(original.ihdr,false))]
        }else{Seq::empty()},
        _=>Seq::empty(),
    }
}
pub proof fn requests_step(b: Seq<u8>,p: Policy,n: nat)
    ensures requests_p(b,p,n+1)==requests_p(b,p,n)+request_p(b,p,n),
{
    hide(d::MWF_p);reveal(requests_p);reveal(request_p);
    if let d::Outcome::Done(original,_)=d::MWF_p(b,p.walk_view()) {
        if let Some(k)=SavedAt_p(b,p) {
            if k<n {}else if k==n {}else{assert(k>=n+1);}
        }
    }
    assert(requests_p(b,p,n+1)=~=requests_p(b,p,n)+request_p(b,p,n));
}

}
