//! Complete named-policy still result and allocation relations.
use vstd::prelude::*;
use crate::{still_model as s,policy_still_model as m,document_model as d,raster_model as r,
    paint_model as p,allocation_model as a,still_equivalence as se,
    policy::{Policy,PaletteIndex},alloc_guard::{AllocSite,sites::TryError}};
verus! {
pub open spec fn native8(result: Result<(crate::png::Still,bool),TryError>) -> Result<(s::Value,bool),TryError> {
    match result {Ok((v,recovered))=>Ok((se::value8(v),recovered)),Err(e)=>Err(e)}
}
pub open spec fn native16(result: Result<(crate::png::Still16,bool),TryError>) -> Result<(s::Value,bool),TryError> {
    match result {Ok((v,recovered))=>Ok((se::value16(v),recovered)),Err(e)=>Err(e)}
}
pub open spec fn eq(b: Seq<u8>,pol: Policy,wide: bool,result: Result<(s::Value,bool),TryError>) -> bool {
    match result {
        Err(TryError::OutOfMemory(_))=>true,
        Err(TryError::Format(e))=>m::MS_wide_p(b,pol,wide)==m::Outcome::Error(e),
        Ok((value,recovered))=>match m::MS_wide_p(b,pol,wide) {
            m::Outcome::Done(expected,expected_recovered)=>s::value_eq(value,expected) && recovered==expected_recovered,
            _=>false,
        },
    }
}
pub open spec fn recovery_matches(b: Seq<u8>,pol: Policy,recovered: bool) -> bool {
    match d::MW_p(b,m::walk(pol)) {d::Outcome::Done(doc,events)=>recovered==m::recovered(b,doc,events,pol),_=>false}
}
pub proof fn bind_recovery(b: Seq<u8>,pol: Policy,events: d::Events)
    requires m::events_match(b,pol,events),
    ensures recovery_matches(b,pol,events.r1 || events.r3 || events.r4),
{
    hide(d::MW_p);
    m::walk_exact(pol);
    match d::MW_p(b,m::walk(pol)) {d::Outcome::Done(doc,walked)=>{m::events_recovered(b,pol,events,doc,walked);},_=>{}}
}
pub open spec fn requests(b: Seq<u8>,pol: Policy,wide: bool) -> Seq<(AllocSite,nat)> {
    match d::MW_p(b,m::walk(pol)) {d::Outcome::Done(doc,_)=>a::image_requests(doc,wide),_=>Seq::empty()}
}
pub open spec fn request_bounds(b: Seq<u8>,pol: Policy,wide: bool) -> bool {
    match d::MW_p(b,m::walk(pol)) {
        d::Outcome::Done(doc,_)=>{
            let req=a::image_requests(doc,wide);
            2<=req[0].1<=2*(crate::limits::MAX_OUTPUT as nat)+crate::limits::MAX_OUTPUT as nat/2+2
            && req[0].1<=crate::alloc_guard::MAX_BYTES
            && 1<=req[1].1<=(if wide {2nat}else{1nat})*crate::limits::MAX_OUTPUT
            && req[1].1<=crate::alloc_guard::MAX_BYTES
        },_=>true,
    }
}
pub open spec fn header_facts(b: Seq<u8>,pol: Policy,wide: bool) -> bool {
    s::walk_header(b,m::walk(pol)) && request_bounds(b,pol,wide)
}
#[verifier::spinoff_prover]
pub proof fn complete(b: Seq<u8>,pol: Policy,wide: bool,result: Result<(s::Value,bool),TryError>)
    requires s::walk_header(b,m::walk(pol)),r::fmt_eq(b,pol,wide,result),
        match result {Ok((v,recovered))=>a::returned_info(b,m::walk(pol),false,v.info)
            && r::shape_matches(b,pol,wide,v.width,v.height,v.pixels.len())
            && p::still_pixels(b,pol,wide,v.pixels) && recovery_matches(b,pol,recovered),_=>true},
    ensures eq(b,pol,wide,result),match result {Ok((v,_))=>s::resources(v,wide),_=>true},
{
    hide(d::MW_p);hide(r::document_data);hide(inflate_verus::model::Inflate);
    m::walk_exact(pol);r::still_walk(b,pol,wide);
    if let Ok((value,recovered))=result {
        match d::MW_p(b,m::walk(pol)) {
            d::Outcome::Done(doc,events)=>match d::default_stream(b,doc) {
                Some(z)=>{
                    p::still_pixels_context(b,pol,wide,value.pixels,doc,events,z);
                    m::image_complete(r::inflated(z,p::default_raster(doc,pol),pol).out,
                        p::default_raster(doc,pol),crate::colour_model::from_doc(doc,b),value.pixels,wide,pol.palette_index==PaletteIndex::Black);
                    s::output_bound(doc.ihdr.width as nat,doc.ihdr.height as nat,wide);
                },None=>{reveal(r::document_data);},
            },_=>{},
        }
    }
}
pub open spec fn G(b: Seq<u8>,pol: Policy,wide: bool,result: Result<(s::Value,bool),TryError>,log: Seq<(AllocSite,nat,bool)>) -> bool {
    eq(b,pol,wide,result) && request_bounds(b,pol,wide)
    && crate::alloc_guard::sites::log_tied(result,log)
    && crate::alloc_guard::req_of_log(log).is_prefix_of(requests(b,pol,wide))
    && match result {Err(TryError::OutOfMemory(_))=>log.len()>=1,
        _=>crate::alloc_guard::req_of_log(log)==requests(b,pol,wide) && crate::alloc_guard::all_ok(log)}
}
pub open spec fn Prog(b: Seq<u8>,pol: Policy,wide: bool,result: Result<(s::Value,bool),TryError>,log: Seq<(AllocSite,nat,bool)>) -> bool {
    crate::alloc_guard::all_ok(log) ==> !matches!(result,Err(TryError::OutOfMemory(_)))
        && eq(b,pol,wide,result) && crate::alloc_guard::req_of_log(log)==requests(b,pol,wide)
}
#[verifier::spinoff_prover]
pub proof fn allocation_complete(b: Seq<u8>,pol: Policy,wide: bool,result: Result<(s::Value,bool),TryError>,log: Seq<(AllocSite,nat,bool)>)
    requires eq(b,pol,wide,result),request_bounds(b,pol,wide),a::image_call(b,m::walk(pol),false,wide,result,log),
    ensures G(b,pol,wide,result,log),Prog(b,pol,wide,result,log),se::only_still_sites(log),
{
    hide(d::MW_p);hide(eq);hide(request_bounds);
    m::walk_exact(pol);
    assert(log.len()<=2);crate::icc_model::short_log(log);
    match d::MW_p(b,m::walk(pol)) {
        d::Outcome::Done(doc,_)=>{
            let req=crate::alloc_guard::req_of_log(log);let expected=a::image_requests(doc,wide);
            assert(req.is_prefix_of(expected));
            if log.len()>0 {assert(req[0]==expected[0]);assert(req[0].0==log[0].0);assert(log[0].0==AllocSite::Raw);}
            if log.len()>1 {assert(req[1]==expected[1]);assert(req[1].0==log[1].0);assert(log[1].0==AllocSite::Output);}
        },d::Outcome::Error(_,_)=>{
            assert(log.len()==0);assert(crate::alloc_guard::all_ok(log));assert(crate::alloc_guard::sites::log_tied(result,log));
        },_=>{},
    }
}
}
