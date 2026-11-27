//! Canonical Adam7 source-cell/strided-row observations for complete MS witnesses.
use vstd::prelude::*;
use crate::{colour_model as c,document_model as d,raster_model as r,
    paint_model as p,still_model as sm,policy::STRICT,witness_m6_rows_model as rows};
verus! {
pub open spec fn expected_pass_row(a: r::Raster,expected: Seq<u8>,k: nat,row: nat) -> Seq<u8> {
    let g=p::pass_geometry(a,k);
    Seq::new(4*r::pw(a,k),|j: int| expected[
        (4*((g.1+row*g.3)*a.width+g.0+(j as nat/4)*g.2)) as int+j%4])
}
/// Every output byte uses the accepted Adam7 inverse/forward coordinate theorem.
#[verifier::spinoff_prover]
pub(crate) proof fn image_from_pass_rows(input: Seq<u8>,a: r::Raster,colour: c::Context,expected: Seq<u8>)
    requires p::domain(a),a.ihdr.interlace==1,
        expected.len()==4*a.width*a.height,
        forall|k: nat,y: nat| k<r::count(a) && r::pw(a,k)>0 && y<r::ph(a,k)
            ==> #[trigger] rows::row_bytes8(colour,r::line(input,a,k,y),r::pw(a,k))
                ==expected_pass_row(a,expected,k,y),
    ensures sm::image(input,a,colour,false)==expected,
{
    hide(c::pixel);hide(p::source_cell);hide(p::pass_geometry);hide(r::line);
    let actual=sm::image(input,a,colour,false);
    assert forall|j: int| 0<=j<actual.len() implies #[trigger] actual[j]==expected[j] by {
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod(j,4);
        vstd::arithmetic::div_mod::lemma_mod_pos_bound(j,4);
        let i=(j/4) as nat;let channel=j%4;
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod(i as int,a.width as int);
        vstd::arithmetic::div_mod::lemma_mod_pos_bound(i as int,a.width as int);
        let x=i%a.width;let y=i/a.width;
        assert(y<a.height) by (nonlinear_arith)
            requires j<4*a.width*a.height,j==4*i+channel,i==a.width*y+x,channel>=0,x>=0,a.width>=1;
        p::source_coordinate(a,x,y);
        let at=p::source_cell(a,x,y);let g=p::pass_geometry(a,at.0);
        let q=4*at.1+channel;
        let converted=rows::row_bytes8(colour,r::line(input,a,at.0,at.2),r::pw(a,at.0));
        assert(0<=q<converted.len()) by (nonlinear_arith)
            requires at.1<r::pw(a,at.0),0<=channel<4,q==4*at.1+channel,converted.len()==4*r::pw(a,at.0);
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(q,4,at.1 as int,channel);
        assert(converted==expected_pass_row(a,expected,at.0,at.2));
        assert(actual[j]==converted[q]);
        assert(4*((g.1+at.2*g.3)*a.width+g.0+at.1*g.2)+channel==j) by (nonlinear_arith)
            requires g.1+at.2*g.3==y,g.0+at.1*g.2==x,i==a.width*y+x,j==4*i+channel;
        assert(converted[q]==expected[j]);
    }
    assert(actual=~=expected);
}
#[verifier::spinoff_prover]
pub(crate) proof fn complete_still(b: Seq<u8>,h: crate::model::Ihdr,z: Seq<u8>,raw: Seq<u8>,entries: nat,
    colour: c::Context,expected: Seq<u8>)
    requires r::default_case(b,h,z,entries),h.interlace==1,
        p::domain(r::shape(h,h.width as nat,h.height as nat,entries,STRICT)),
        r::data(z,r::shape(h,h.width as nat,h.height as nat,entries,STRICT),STRICT)==Some(Ok(())),
        r::inflated(z,r::shape(h,h.width as nat,h.height as nat,entries,STRICT),STRICT).out==raw,
        match d::MW_p(b,d::strict()) {d::Outcome::Done(doc,_)=>c::from_doc(doc,b)==colour,_=>false},
        expected.len()==4*(h.width as nat)*(h.height as nat),
        forall|k: nat,y: nat| k<7 && r::pw(r::shape(h,h.width as nat,h.height as nat,entries,STRICT),k)>0
            && y<r::ph(r::shape(h,h.width as nat,h.height as nat,entries,STRICT),k)
            ==> #[trigger] rows::row_bytes8(colour,r::line(raw,r::shape(h,h.width as nat,h.height as nat,entries,STRICT),k,y),
                    r::pw(r::shape(h,h.width as nat,h.height as nat,entries,STRICT),k))
                ==expected_pass_row(r::shape(h,h.width as nat,h.height as nat,entries,STRICT),expected,k,y),
    ensures match sm::MS(b) {sm::Outcome::Done(value)=>value.width==h.width
        && value.height==h.height && value.pixels==expected,_=>false},
{
    hide(d::MW_p);hide(inflate_verus::model::Inflate);hide(c::pixel);
    crate::walk_policy::profiles_project_exactly();
    let a=r::shape(h,h.width as nat,h.height as nat,entries,STRICT);
    image_from_pass_rows(raw,a,colour,expected);
    match d::MW_p(b,d::strict()) {
        d::Outcome::Done(doc,_)=>{
            assert(r::document_shape(doc,STRICT)==a);
            assert(r::document_data(b,doc,STRICT)==Some(Ok(())));
        },_=>{},
    }
}
/// Observe the two actual trailing transitions without assuming walk success.
#[verifier::spinoff_prover]
pub(crate) proof fn tail_colour(b: Seq<u8>,s: d::State,n: nat)
    requires b.len()==s.pos+24+n,n>=1,
        forall|phase: crate::walk_model::Phase| #[trigger] crate::critical_model::read(b,s.pos,phase,false)==Ok(
            crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:s.pos+8,len:n,next:s.pos+12+n,crc_ok:true,synthetic:false}),
        forall|phase: crate::walk_model::Phase| #[trigger] crate::critical_model::read(b,s.pos+12+n,phase,false)==Ok(
            crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:s.pos+20+n,len:0,next:s.pos+24+n,crc_ok:true,synthetic:false}),
    ensures match d::drive(b,d::strict(),false,false,s) {
        d::Outcome::Done(doc,_)=>d::witness_colour_eq(doc,s.doc),_=>true},
{
    hide(crate::critical_model::read);
    let idat=crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:s.pos+8,len:n,next:s.pos+12+n,crc_ok:true,synthetic:false};
    let iend=crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:s.pos+20+n,len:0,next:s.pos+24+n,crc_ok:true,synthetic:false};
    d::drive_step(b,d::strict(),false,false,s);
    d::witness_step_colour(b,s,idat);
    match d::step(b,d::strict(),false,false,s) {
        d::Step::Next(next)=>{
            assert(next.pos==s.pos+12+n);
            d::drive_step(b,d::strict(),false,false,next);
            d::witness_step_colour(b,next,iend);
            assert(!matches!(d::step(b,d::strict(),false,false,next),d::Step::Next(_)));
        },_=>{},
    }
}
#[verifier::spinoff_prover]
pub(crate) proof fn palette_colour(b: Seq<u8>,h: crate::model::Ihdr,n: nat)
    requires b.len()==117+n,n>=1,h.color_type==3,h.depth==4,
        crate::model::mh(b)==Ok((h,33nat)),
        forall|phase: crate::walk_model::Phase| #[trigger] crate::critical_model::read(b,33,phase,false)==Ok(
            crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41,len:48,next:93,crc_ok:true,synthetic:false}),
        forall|phase: crate::walk_model::Phase| #[trigger] crate::critical_model::read(b,93,phase,false)==Ok(
            crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101,len:n,next:105+n,crc_ok:true,synthetic:false}),
        forall|phase: crate::walk_model::Phase| #[trigger] crate::critical_model::read(b,105+n,phase,false)==Ok(
            crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:113+n,len:0,next:117+n,crc_ok:true,synthetic:false}),
    ensures match d::MW_p(b,d::strict()) {d::Outcome::Done(doc,_)=>c::from_doc(doc,b)
        ==(c::Context {ct:3u8,depth:4u8,plte:b.subrange(41,89),trns:None}),_=>true},
{
    hide(crate::critical_model::read);hide(d::initial_info);
    let initial=d::initial(h,33,d::no_events());
    assert(d::MW_p(b,d::strict())==d::drive(b,d::strict(),false,false,initial));
    d::drive_step(b,d::strict(),false,false,initial);
    d::witness_initial_palette_colour(b,h);
    match d::step(b,d::strict(),false,false,initial) {
        d::Step::Next(next)=>{tail_colour(b,next,n);},_=>{},
    }
}
} // verus!
