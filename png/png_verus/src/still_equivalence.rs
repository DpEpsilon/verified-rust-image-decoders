//! Complete Strict still result and sealed-log relations; policy recovery is
//! deliberately outside checkpoint 1a-E.
use vstd::prelude::*;
use crate::{still_model as m,document_model as d,raster_model as r,paint_model as p,
    allocation_model as a,alloc_guard::{AllocSite,sites::TryError}};
verus! {
pub open spec fn value8(s: crate::png::Still) -> m::Value {
    m::Value {width:s.width,height:s.height,info:s.info.model_value(),pixels:s.rgba@}
}
pub open spec fn value16(s: crate::png::Still16) -> m::Value {
    m::Value {width:s.width,height:s.height,info:s.info.model_value(),pixels:s.rgba16@}
}
pub open spec fn native8(result: Result<crate::png::Still,TryError>) -> Result<m::Value,TryError> {
    match result {Ok(s)=>Ok(value8(s)),Err(e)=>Err(e)}
}
pub open spec fn native16(result: Result<crate::png::Still16,TryError>) -> Result<m::Value,TryError> {
    match result {Ok(s)=>Ok(value16(s)),Err(e)=>Err(e)}
}
/// OOM is outside the format relation. Every other result has full equality.
pub open spec fn eq(b: Seq<u8>,wide: bool,result: Result<m::Value,TryError>) -> bool {
    match result {
        Err(TryError::OutOfMemory(_))=>true,
        Err(TryError::Format(e))=>m::MS_wide(b,wide)==m::Outcome::Error(e),
        Ok(value)=>match m::MS_wide(b,wide) {m::Outcome::Done(expected)=>m::value_eq(value,expected),_=>false},
    }
}
pub open spec fn success_package(b: Seq<u8>,wide: bool,value: m::Value) -> bool {
    eq(b,wide,Ok(value)) && m::resources(value,wide) && m::T13(b,wide)
}
pub open spec fn requests(b: Seq<u8>,wide: bool) -> Seq<(AllocSite,nat)> {
    match d::MW_p(b,d::strict()) {d::Outcome::Done(doc,_)=>a::image_requests(doc,wide),_=>Seq::empty()}
}
pub open spec fn request_bounds(b: Seq<u8>,wide: bool) -> bool {
    match d::MW_p(b,d::strict()) {
        d::Outcome::Done(doc,_)=>{
            let req=a::image_requests(doc,wide);
            1<=req[0].1<=2*(crate::limits::MAX_OUTPUT as nat)+crate::limits::MAX_OUTPUT as nat/2+2
            && req[0].1<=crate::alloc_guard::MAX_BYTES
            && 1<=req[1].1<=(if wide {2nat}else{1nat})*crate::limits::MAX_OUTPUT
            && req[1].1<=crate::alloc_guard::MAX_BYTES
        },_=>true,
    }
}
pub open spec fn header_facts(b: Seq<u8>,wide: bool) -> bool {
    m::walk_header(b,d::strict()) && request_bounds(b,wide)
}
pub(crate) proof fn document_context(b: Seq<u8>,doc: crate::walk::Doc)
    requires doc.wf(b),doc.ihdr.ct==3 ==> doc.plte.is_some(),
    ensures m::checked_header(doc.ihdr.view()),doc.info.view().header==doc.ihdr.view(),
        crate::colour_model::context_domain(crate::colour_model::from_doc(doc.view(),b)),
        d::entries(doc.view())==crate::colour_model::from_doc(doc.view(),b).plte.len()/3,
{doc.colour_context(b);}
/// The caller instantiates this before entering reserve_image, hence before
/// either guard call; the bounds do not depend on allocation success.
pub proof fn header_request_bounds(h: crate::model::Ihdr,wide: bool)
    requires m::checked_header(h),
    ensures 1<=crate::layout_model::raw(h.interlace,crate::layout_model::bits(h),h.width as nat,h.height as nat)
        <=2*(crate::limits::MAX_OUTPUT as nat)+crate::limits::MAX_OUTPUT as nat/2+2,
        crate::layout_model::raw(h.interlace,crate::layout_model::bits(h),h.width as nat,h.height as nat)<=crate::alloc_guard::MAX_BYTES,
        1<=crate::layout_model::output_count(h,wide)<=(if wide {2nat}else{1nat})*crate::limits::MAX_OUTPUT,
        crate::layout_model::output_count(h,wide)<=crate::alloc_guard::MAX_BYTES,
{
    crate::limits::caps_ok();m::output_bound(h.width as nat,h.height as nat,wide);
    let count=crate::layout_model::output_count(h,wide);
    let factor=if wide {8nat}else{4nat};
    assert(count==factor*(h.width as nat)*(h.height as nat));
    assert(count>=1) by (nonlinear_arith)
        requires h.width>=1,h.height>=1,factor>=1,count==factor*(h.width as nat)*(h.height as nat);
    assert(crate::layout_model::bits(h)<=64);
    crate::layout_model::raw_bounds(h.interlace,crate::layout_model::bits(h),h.width as nat,h.height as nat);
    assert(10*(h.width as int)*(h.height as int)+2<=2*(crate::limits::MAX_OUTPUT as int)+crate::limits::MAX_OUTPUT as int/2+2) by (nonlinear_arith)
        requires 4*(h.width as int)*(h.height as int)<=crate::limits::MAX_OUTPUT;
}

/// Compose the actual M4 error/shape and M5 content observations into the
/// independently defined complete MS/MS16 result. No public decoder is called.
#[verifier::spinoff_prover]
pub proof fn complete(b: Seq<u8>,wide: bool,result: Result<m::Value,TryError>)
    requires m::walk_header(b,d::strict()),r::fmt_eq(b,crate::policy::STRICT,wide,result),
        match result {Ok(v)=>a::returned_info(b,d::strict(),false,v.info)
            && r::shape_matches(b,crate::policy::STRICT,wide,v.width,v.height,v.pixels.len())
            && p::still_pixels(b,crate::policy::STRICT,wide,v.pixels),_=>true},
    ensures eq(b,wide,result),match result {Ok(v)=>m::resources(v,wide),_=>true},
{
    hide(d::MW_p);hide(r::document_data);hide(inflate_verus::model::Inflate);
    crate::walk_policy::profiles_project_exactly();r::still_walk(b,crate::policy::STRICT,wide);
    if let Ok(value)=result {
        match d::MW_p(b,d::strict()) {
            d::Outcome::Done(doc,ev)=>{
                match d::default_stream(b,doc) {
                    Some(z)=>{
                        p::still_pixels_context(b,crate::policy::STRICT,wide,value.pixels,doc,ev,z);
                        m::image_complete(r::inflated(z,p::default_raster(doc,crate::policy::STRICT),crate::policy::STRICT).out,
                            p::default_raster(doc,crate::policy::STRICT),crate::colour_model::from_doc(doc,b),value.pixels,wide);
                        m::output_bound(doc.ihdr.width as nat,doc.ihdr.height as nat,wide);
                    },None=>{reveal(r::document_data);},
                }
            },_=>{},
        }
    }
}
#[verifier::spinoff_prover]
pub proof fn success(b: Seq<u8>,wide: bool,value: m::Value)
    requires eq(b,wide,Ok(value)),m::resources(value,wide),m::walk_context(b),
    ensures success_package(b,wide,value),
{
    hide(d::MW_p);hide(inflate_verus::model::Inflate);
    m::model_colour_properties(b);
    m::bind_model_properties(b,wide);
}
/// Full alloc-delta clauses 1–3, with OOM tied to the actual sealed log.
pub open spec fn G(b: Seq<u8>,wide: bool,result: Result<m::Value,TryError>,log: Seq<(AllocSite,nat,bool)>) -> bool {
    eq(b,wide,result) && request_bounds(b,wide)
    && crate::alloc_guard::sites::log_tied(result,log)
    && crate::alloc_guard::req_of_log(log).is_prefix_of(requests(b,wide))
    && match result {
        Err(TryError::OutOfMemory(_))=>log.len()>=1,
        _=>crate::alloc_guard::req_of_log(log)==requests(b,wide) && crate::alloc_guard::all_ok(log),
    }
}
pub open spec fn Prog(b: Seq<u8>,wide: bool,result: Result<m::Value,TryError>,log: Seq<(AllocSite,nat,bool)>) -> bool {
    crate::alloc_guard::all_ok(log) ==> !matches!(result,Err(TryError::OutOfMemory(_)))
        && eq(b,wide,result) && crate::alloc_guard::req_of_log(log)==requests(b,wide)
}
pub open spec fn only_still_sites(log: Seq<(AllocSite,nat,bool)>) -> bool {
    log.len()<=2 && forall|j: int| 0<=j<log.len() ==> (#[trigger] log[j]).0==AllocSite::Raw || log[j].0==AllocSite::Output
}
#[verifier::spinoff_prover]
pub proof fn allocation_complete(b: Seq<u8>,wide: bool,result: Result<m::Value,TryError>,log: Seq<(AllocSite,nat,bool)>)
    requires eq(b,wide,result),request_bounds(b,wide),a::image_call(b,d::strict(),false,wide,result,log),
    ensures G(b,wide,result,log),Prog(b,wide,result,log),only_still_sites(log),
{
    hide(d::MW_p);hide(eq);hide(request_bounds);
    assert(log.len()<=2);
    crate::icc_model::short_log(log);
    match d::MW_p(b,d::strict()) {
        d::Outcome::Done(doc,_)=>{
            let req=crate::alloc_guard::req_of_log(log);let expected=a::image_requests(doc,wide);
            assert(req.is_prefix_of(expected));
            if log.len()>0 {
                assert(req[0]==expected[0]);assert(req[0].0==log[0].0);
                assert(expected[0].0==AllocSite::Raw);assert(log[0].0==AllocSite::Raw);
            }
            if log.len()>1 {
                assert(req[1]==expected[1]);assert(req[1].0==log[1].0);
                assert(expected[1].0==AllocSite::Output);assert(log[1].0==AllocSite::Output);
            }
        },d::Outcome::Error(_,_)=>{
            assert(log.len()==0);
            assert(crate::alloc_guard::all_ok(log));
            assert(crate::alloc_guard::sites::log_tied(result,log));
        },_=>{},
    }
}
}
