//! Independent complete named-policy still values, including exact recovery.
use vstd::prelude::*;
use crate::{still_model as s,document_model as d,raster_model as r,paint_model as p,
    colour_model as c,policy::{Policy,Profile,PaletteIndex}};
verus! {
/// Public mathematical projection of the existing policy record.
pub closed spec fn walk(pol: Policy) -> d::Policy {pol.walk_view()}
pub(crate) proof fn walk_exact(pol: Policy)
    ensures walk(pol)==pol.walk_view(),
{reveal(walk);}

pub enum Outcome {Error(crate::png::Error),Done(s::Value,bool),OutsideDomain}
/// Reuse the accepted Reject construction; Black uses the same P13 equations
/// with the specified out-of-range opaque-black sample.
pub open spec fn image(input: Seq<u8>,a: r::Raster,colour: c::Context,wide: bool,black: bool) -> Seq<u8> {
    if !black {s::image(input,a,colour,wide)}else{
        let bytes=if wide {8nat}else{4nat};
        Seq::new(bytes*a.width*a.height,|j: int| {
            let i=j as nat/bytes;
            c::physical(p::source_pixel(input,a,colour,i%a.width,i/a.width,wide,true).get_Ok_0(),wide)[j%(bytes as int)]
        })
    }
}
pub open spec fn recovered(b: Seq<u8>,doc: d::Doc,events: d::Events,pol: Policy) -> bool {
    events.r1 || events.r3 || events.r4 || match d::default_stream(b,doc) {
        Some(z)=>crate::still_recovery::R3(r::inflated(z,p::default_raster(doc,pol),pol).out,p::default_raster(doc,pol)),
        None=>false,
    }
}
pub open spec fn MS_wide_p(b: Seq<u8>,pol: Policy,wide: bool) -> Outcome {
    match d::MW_p(b,walk(pol)) {
        d::Outcome::Error(e,_)=>Outcome::Error(e),
        d::Outcome::Done(doc,events)=>match r::document_data(b,doc,pol) {
            Some(Err(e))=>Outcome::Error(e),
            Some(Ok(_))=>match d::default_stream(b,doc) {
                Some(z)=>Outcome::Done(s::Value {width:doc.ihdr.width,height:doc.ihdr.height,info:doc.info,
                    pixels:image(r::inflated(z,p::default_raster(doc,pol),pol).out,p::default_raster(doc,pol),
                        c::from_doc(doc,b),wide,pol.palette_index==PaletteIndex::Black)},recovered(b,doc,events,pol)),
                None=>Outcome::OutsideDomain,
            },None=>Outcome::OutsideDomain,
        },_=>Outcome::OutsideDomain,
    }
}
pub open spec fn MS_p(b: Seq<u8>,profile: Profile) -> Outcome {MS_wide_p(b,r::profile(profile),false)}
pub open spec fn MS16_p(b: Seq<u8>,profile: Profile) -> Outcome {MS_wide_p(b,r::profile(profile),true)}
#[verifier::spinoff_prover]
pub proof fn image_complete(input: Seq<u8>,a: r::Raster,colour: c::Context,state: Seq<u8>,wide: bool,black: bool)
    requires a.width>=1,a.height>=1,crate::model::valid_depth(colour.ct,colour.depth),
        p::image_pixels(input,a,colour,state,wide,black),
    ensures state==image(input,a,colour,wide,black),
{
    hide(c::pixel);
    if !black {s::image_complete(input,a,colour,state,wide);}else{
        p::image_observe(input,a,colour,state,wide,true,0,0);
        let bytes=if wide {8nat}else{4nat};let expected=image(input,a,colour,wide,true);
        assert forall|j: int| 0<=j<state.len() implies #[trigger] state[j]==expected[j] by {
            vstd::arithmetic::div_mod::lemma_fundamental_div_mod(j,bytes as int);
            vstd::arithmetic::div_mod::lemma_mod_pos_bound(j,bytes as int);
            vstd::arithmetic::div_mod::lemma_div_pos_is_pos(j,bytes as int);
            let i=(j/(bytes as int)) as nat;
            assert(i<a.width*a.height) by (nonlinear_arith)
                requires j<bytes*a.width*a.height,j==bytes*i+j%(bytes as int),j%(bytes as int)>=0,bytes>=1;
            let x=i%a.width;let y=i/a.width;
            vstd::arithmetic::div_mod::lemma_fundamental_div_mod(i as int,a.width as int);
            assert(y<a.height) by (nonlinear_arith)
                requires i<a.width*a.height,i==a.width*y+x,x>=0,a.width>=1;
            p::image_observe(input,a,colour,state,wide,true,x,y);
            let shape=(if wide {1nat}else{0nat},state.len(),0nat,0nat,0nat,false);
            c::observed_slice(shape,state,a.width,x,y);
            assert(y*a.width+x==i) by (nonlinear_arith)
                requires i==a.width*y+x;
            assert(c::destination_offset(shape,a.width,x,y)==bytes*(y*a.width+x));
            assert(c::destination_offset(shape,a.width,x,y)==bytes*i);
            assert(0<=bytes*i<=j<bytes*i+bytes<=state.len()) by (nonlinear_arith)
                requires j==bytes*i+j%(bytes as int),0<=j%(bytes as int)<bytes,i<a.width*a.height,
                    state.len()==bytes*a.width*a.height,bytes>=1;
            assert(state[j]==c::physical(p::source_pixel(input,a,colour,x,y,wide,true).get_Ok_0(),wide)[j%(bytes as int)]);
        }
        assert(state=~=expected);
    }
}
/// This observation records every mathematical walk event, plus exact R3
/// accumulated over the successfully decoded raster; no native result oracle.
pub open spec fn events_match(b: Seq<u8>,pol: Policy,events: d::Events) -> bool {
    match d::MW_p(b,walk(pol)) {
        d::Outcome::Done(doc,walked)=>match d::default_stream(b,doc) {
            Some(z)=>events.r1==walked.r1 && events.r4==walked.r4
                && events.r3==(walked.r3 || crate::still_recovery::R3(
                    r::inflated(z,p::default_raster(doc,pol),pol).out,p::default_raster(doc,pol))),
            None=>false,
        },_=>false,
    }
}
pub proof fn events_recovered(b: Seq<u8>,pol: Policy,events: d::Events,doc: d::Doc,walked: d::Events)
    requires events_match(b,pol,events),d::MW_p(b,walk(pol))==d::Outcome::Done(doc,walked),
    ensures (events.r1 || events.r3 || events.r4)==recovered(b,doc,walked,pol),
{}
pub open spec fn total(b: Seq<u8>,profile: Profile) -> bool {
    !matches!(MS_p(b,profile),Outcome::OutsideDomain) && !matches!(MS16_p(b,profile),Outcome::OutsideDomain)
}
/// Reuse the reviewed allocation-free certificate construction with the
/// named profile's checked inflater options; the model definitions do not call it.
#[verifier::spinoff_prover]
pub fn model_totality(b: &[u8],profile: Profile)
    ensures total(b@,profile),s::walk_header(b@,walk(r::profile(profile))),
        match MS_p(b@,profile) {Outcome::Done(value,_)=>s::resources(value,false),_=>true},
        match MS16_p(b@,profile) {Outcome::Done(value,_)=>s::resources(value,true),_=>true},
{
    hide(d::MW_p);hide(r::document_data);hide(inflate_verus::model::Inflate);
    let pol=profile.checked_policy();
    proof {walk_exact(pol);}
    let mut events=crate::png::Events {r1:false,r3:false,r4:false};
    let result=crate::callers::still(b,&pol,&mut events);
    match result {
        Err(_)=>{},
        Ok(doc)=>{
            let limit=crate::image::raw_size(&doc.ihdr,doc.ihdr.w,doc.ihdr.h);
            let mut source=crate::reader::IdatReader::for_doc(b,&doc);
            let mut window=[0u8;inflate_verus::limits::WINDOW_MAX];
            proof {
                vstd::array::array_len_matches_n(&window);
                assert(inflate_verus::limits::WINDOW_MAX==32768usize) by (compute);
            }
            assert(window@.len()==inflate_verus::limits::WINDOW_MAX);
            let ring=vstd::array::ref_mut_array_unsizing_coercion(&mut window);
            assert(ring.len()==inflate_verus::limits::WINDOW_MAX);
            let options=pol.inflate_options();
            let measured=inflate_verus::inflate_zlib_measure(&mut source,ring,limit,options);
            proof {
                use inflate_verus::Segments;
                let a=p::default_raster(doc.view(),pol);let z=source.concat();
                assert(inflate_verus::status_eq(measured.status,r::inflated(z,a,pol).status));
                assert(!matches!(r::inflated(z,a,pol).status,inflate_verus::model::InflateStatus::OutsideDomain));
                reveal(r::document_data);
                s::output_bound(a.width,a.height,false);s::output_bound(a.width,a.height,true);
            }
        },
    }
}

}
