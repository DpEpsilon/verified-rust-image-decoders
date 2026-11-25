//! Shared row decomposition of the unchanged canonical MS byte equation.
use vstd::prelude::*;
use crate::{colour_model as c,document_model as d,raster_model as r,
    still_model as sm,policy::STRICT};
verus! {
/// The same sixteen-bit sample and P13.2 rounding used by canonical MS.
/// This is a mathematical row observation, not an executable converter.
pub open spec fn row_bytes8(colour: c::Context,line: Seq<u8>,width: nat) -> Seq<u8> {
    Seq::new(4*width,|j: int| {
        let px=c::pixel(colour,line,j as nat/4,true,false).get_Ok_0();
        ((px[j%4] as nat+128)/257) as u8
    })
}
#[verifier::spinoff_prover]
pub(crate) proof fn image_from_rows(input: Seq<u8>,a: r::Raster,colour: c::Context,expected: Seq<u8>)
    requires a.ihdr.interlace==0,a.width>=1,a.height>=1,
        expected.len()==4*a.width*a.height,
        forall|y: nat| y<a.height ==> #[trigger] row_bytes8(colour,r::line(input,a,0,y),a.width)
            ==expected.subrange((4*a.width*y) as int,(4*a.width*(y+1)) as int),
    ensures sm::image(input,a,colour,false)==expected,
{
    hide(c::pixel);
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
        let start=4*a.width*y;let offset=4*x+channel;
        assert(start+offset==j) by (nonlinear_arith)
            requires j==4*i+channel,i==a.width*y+x,start==4*a.width*y,offset==4*x+channel;
        assert(0<=offset<4*a.width) by (nonlinear_arith)
            requires x<a.width,0<=channel<4,offset==4*x+channel;
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(offset,4,x as int,channel);
        let line=r::line(input,a,0,y);let row=row_bytes8(colour,line,a.width);
        assert(row==expected.subrange(start as int,(4*a.width*(y+1)) as int));
        assert(crate::paint_model::source_cell(a,x,y)==(0nat,x,y));
        assert(actual[j]==row[offset]);
        assert(4*a.width*(y+1)==start+4*a.width) by (nonlinear_arith)
            requires start==4*a.width*y;
        assert(start+4*a.width<=expected.len()) by (nonlinear_arith)
            requires start==4*a.width*y,y<a.height,expected.len()==4*a.width*a.height,a.width>=1;
        assert(row[offset]==expected[j]);
    }
    assert(actual=~=expected);
}
/// Finite byte extensionality shared by all small fixture rows.
#[verifier::spinoff_prover]
pub(crate) proof fn bounded_row_equal(actual: Seq<u8>,expected: Seq<u8>)
    requires actual.len()==expected.len(),actual.len()<=32,
        (0<actual.len() ==> actual[0]==expected[0]),
        (1<actual.len() ==> actual[1]==expected[1]),
        (2<actual.len() ==> actual[2]==expected[2]),
        (3<actual.len() ==> actual[3]==expected[3]),
        (4<actual.len() ==> actual[4]==expected[4]),
        (5<actual.len() ==> actual[5]==expected[5]),
        (6<actual.len() ==> actual[6]==expected[6]),
        (7<actual.len() ==> actual[7]==expected[7]),
        (8<actual.len() ==> actual[8]==expected[8]),
        (9<actual.len() ==> actual[9]==expected[9]),
        (10<actual.len() ==> actual[10]==expected[10]),
        (11<actual.len() ==> actual[11]==expected[11]),
        (12<actual.len() ==> actual[12]==expected[12]),
        (13<actual.len() ==> actual[13]==expected[13]),
        (14<actual.len() ==> actual[14]==expected[14]),
        (15<actual.len() ==> actual[15]==expected[15]),
        (16<actual.len() ==> actual[16]==expected[16]),
        (17<actual.len() ==> actual[17]==expected[17]),
        (18<actual.len() ==> actual[18]==expected[18]),
        (19<actual.len() ==> actual[19]==expected[19]),
        (20<actual.len() ==> actual[20]==expected[20]),
        (21<actual.len() ==> actual[21]==expected[21]),
        (22<actual.len() ==> actual[22]==expected[22]),
        (23<actual.len() ==> actual[23]==expected[23]),
        (24<actual.len() ==> actual[24]==expected[24]),
        (25<actual.len() ==> actual[25]==expected[25]),
        (26<actual.len() ==> actual[26]==expected[26]),
        (27<actual.len() ==> actual[27]==expected[27]),
        (28<actual.len() ==> actual[28]==expected[28]),
        (29<actual.len() ==> actual[29]==expected[29]),
        (30<actual.len() ==> actual[30]==expected[30]),
        (31<actual.len() ==> actual[31]==expected[31]),
    ensures actual==expected,
{
    assert forall|j: int| 0<=j<actual.len() implies #[trigger] actual[j]==expected[j] by {
        if j==0 {}
        else if j==1 {}
        else if j==2 {}
        else if j==3 {}
        else if j==4 {}
        else if j==5 {}
        else if j==6 {}
        else if j==7 {}
        else if j==8 {}
        else if j==9 {}
        else if j==10 {}
        else if j==11 {}
        else if j==12 {}
        else if j==13 {}
        else if j==14 {}
        else if j==15 {}
        else if j==16 {}
        else if j==17 {}
        else if j==18 {}
        else if j==19 {}
        else if j==20 {}
        else if j==21 {}
        else if j==22 {}
        else if j==23 {}
        else if j==24 {}
        else if j==25 {}
        else if j==26 {}
        else if j==27 {}
        else if j==28 {}
        else if j==29 {}
        else if j==30 {}
        else {assert(j==31);}
    }
    assert(actual=~=expected);
}
/// Compose existing walk/inflate/data contracts with the shared row equation.
#[verifier::spinoff_prover]
pub(crate) proof fn complete_still(b: Seq<u8>,h: crate::model::Ihdr,z: Seq<u8>,raw: Seq<u8>,
    colour: c::Context,expected: Seq<u8>)
    requires r::default_case(b,h,z,0),h.interlace==0,h.width>=1,h.height>=1,
        r::data(z,r::shape(h,h.width as nat,h.height as nat,0,STRICT),STRICT)==Some(Ok(())),
        r::inflated(z,r::shape(h,h.width as nat,h.height as nat,0,STRICT),STRICT).out==raw,
        match d::MW_p(b,d::strict()) {d::Outcome::Done(doc,_)=>c::from_doc(doc,b)==colour,_=>false},
        expected.len()==4*(h.width as nat)*(h.height as nat),
        forall|y: nat| y<h.height ==> #[trigger] row_bytes8(colour,
            r::line(raw,r::shape(h,h.width as nat,h.height as nat,0,STRICT),0,y),h.width as nat)
            ==expected.subrange((4*(h.width as nat)*y) as int,(4*(h.width as nat)*(y+1)) as int),
    ensures match sm::MS(b) {sm::Outcome::Done(value)=>value.width==h.width
        && value.height==h.height && value.pixels==expected,_=>false},
{
    hide(d::MW_p);hide(inflate_verus::model::Inflate);hide(c::pixel);
    crate::walk_policy::profiles_project_exactly();
    let a=r::shape(h,h.width as nat,h.height as nat,0,STRICT);
    image_from_rows(raw,a,colour,expected);
    match d::MW_p(b,d::strict()) {
        d::Outcome::Done(doc,_)=>{
            assert(r::document_shape(doc,STRICT)==a);
            assert(r::document_data(b,doc,STRICT)==Some(Ok(())));
        },_=>{},
    }
}
/// Compose two accepted transition observations without unfolding their
/// ownership/value machinery in the fixture proof. Success remains a separate
/// consequence of the original M4 walk certificate.
#[verifier::spinoff_prover]
pub(crate) proof fn bare_colour(b: Seq<u8>,h: crate::model::Ihdr,n: nat)
    requires b.len()==57+n,n>=1,
        crate::model::mh(b)==Ok((h,33nat)),
        forall|phase: crate::walk_model::Phase| #[trigger] crate::critical_model::read(b,33,phase,false)==Ok(
            crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41,len:n,next:45+n,crc_ok:true,synthetic:false}),
        forall|phase: crate::walk_model::Phase| #[trigger] crate::critical_model::read(b,45+n,phase,false)==Ok(
            crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:53+n,len:0,next:57+n,crc_ok:true,synthetic:false}),
    ensures match d::MW_p(b,d::strict()) {d::Outcome::Done(doc,_)=>c::from_doc(doc,b)
        ==(c::Context {ct:h.color_type,depth:h.depth,plte:Seq::empty(),trns:None}),_=>true},
{
    hide(crate::critical_model::read);hide(d::initial_info);
    let initial=d::initial(h,33,d::no_events());
    let idat=crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41,len:n,next:45+n,crc_ok:true,synthetic:false};
    let iend=crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:53+n,len:0,next:57+n,crc_ok:true,synthetic:false};
    assert(d::MW_p(b,d::strict())==d::drive(b,d::strict(),false,false,initial));
    d::drive_step(b,d::strict(),false,false,initial);
    d::witness_step_colour(b,initial,idat);
    match d::step(b,d::strict(),false,false,initial) {
        d::Step::Next(next)=>{
            assert(next.pos==45+n);
            d::drive_step(b,d::strict(),false,false,next);
            d::witness_step_colour(b,next,iend);
            assert(!matches!(d::step(b,d::strict(),false,false,next),d::Step::Next(_)));
        },_=>{},
    }
}
}
