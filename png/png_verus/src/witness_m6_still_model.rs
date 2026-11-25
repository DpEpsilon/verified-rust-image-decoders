//! Compose accepted canonical-image observations with existing content witnesses.
use vstd::prelude::*;
use crate::{paint_model as p,still_model as sm,raster_model as r,colour_model as c,
    document_model as d,policy::STRICT};
verus! {
#[verifier::spinoff_prover]
pub(crate) proof fn canonical_image(input: Seq<u8>,a: r::Raster,colour: c::Context,wide: bool)
    requires p::domain(a),crate::model::valid_depth(colour.ct,colour.depth),
        a.ihdr.color_type==colour.ct,a.ihdr.depth==colour.depth,a.entries==colour.plte.len()/3,a.reject_index,
        r::scan(input,a,0).is_ok(),
    ensures p::image_pixels(input,a,colour,sm::image(input,a,colour,wide),wide,false),
{
    hide(c::pixel);
    let state=sm::image(input,a,colour,wide);
    assert forall|x: nat,y: nat| x<a.width && y<a.height implies
        (#[trigger] p::source_pixel(input,a,colour,x,y,wide,false)).is_ok()
        && c::observe((if wide {1nat}else{0nat},state.len(),0nat,0nat,0nat,false),state,a.width,x,y)
            ==c::physical(p::source_pixel(input,a,colour,x,y,wide,false).get_Ok_0(),wide) by {
        sm::source_pixel_ok(input,a,colour,x,y);
        sm::canonical_observe(input,a,colour,wide,x,y);
    }
    p::image_pixels_intro(input,a,colour,state,wide,false);
}
/// No executable or allocation is used to place the independently defined MS
/// pixels in the existing M5 content relation.
#[verifier::spinoff_prover]
pub(crate) proof fn canonical_still(b: Seq<u8>,h: crate::model::Ihdr,z: Seq<u8>,entries: nat,colour: c::Context,wide: bool)
    requires r::default_case(b,h,z,entries),
        r::data(z,r::shape(h,h.width as nat,h.height as nat,entries,STRICT),STRICT)==Some(Ok(())),
        p::domain(r::shape(h,h.width as nat,h.height as nat,entries,STRICT)),
        crate::model::valid_depth(colour.ct,colour.depth),h.color_type==colour.ct,h.depth==colour.depth,
        entries==colour.plte.len()/3,
        match d::MW_p(b,d::strict()) {d::Outcome::Done(doc,_)=>c::from_doc(doc,b)==colour,_=>false},
    ensures match sm::MS_wide(b,wide) {
        sm::Outcome::Done(value)=>value.width==h.width && value.height==h.height
            && p::still_pixels(b,STRICT,wide,value.pixels),
        _=>false,
    },
{
    hide(d::MW_p);hide(inflate_verus::model::Inflate);hide(c::pixel);
    crate::walk_policy::profiles_project_exactly();
    match d::MW_p(b,d::strict()) {
        d::Outcome::Done(doc,ev)=>{
            let a=r::shape(h,h.width as nat,h.height as nat,entries,STRICT);
            assert(r::document_shape(doc,STRICT)==a);
            assert(p::default_raster(doc,STRICT)==a);
            assert(d::default_stream(b,doc)==Some(z));
            assert(r::document_data(b,doc,STRICT)==Some(Ok(())));
            assert(r::document_data(b,doc,STRICT)==r::data(z,a,STRICT));
            assert(r::data(z,a,STRICT)==Some(Ok(())));
            let input=r::inflated(z,a,STRICT).out;
            assert(r::scan(input,a,0).is_ok());
            canonical_image(input,a,colour,wide);
            p::still_pixels_intro(b,doc,STRICT,wide,sm::image(input,a,colour,wide),z,ev);
        },_=>{},
    }
}
}
