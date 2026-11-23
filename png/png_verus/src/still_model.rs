//! Complete deterministic Strict still records; no public-entry equivalence.
use vstd::prelude::*;
use crate::{colour_model as c,document_model as d,raster_model as r,paint_model as p};
use crate::still_samples::pixel_pair;
verus! {
pub struct Value {
    pub width: u32,
    pub height: u32,
    pub info: d::Info,
    pub pixels: Seq<u8>,
}
pub enum Outcome {Error(crate::png::Error),Done(Value),OutsideDomain}
pub open spec fn checked_header(h: crate::model::Ihdr) -> bool {
    1<=h.width<=crate::limits::MAX_DIMENSION && 1<=h.height<=crate::limits::MAX_DIMENSION
    && 4*(h.width as int)*(h.height as int)<=crate::limits::MAX_OUTPUT
    && crate::model::valid_depth(h.color_type,h.depth) && h.interlace<=1
}
pub open spec fn walk_header(b: Seq<u8>,pol: d::Policy) -> bool {
    match d::MW_p(b,pol) {d::Outcome::Done(doc,_)=>checked_header(doc.ihdr),_=>true}
}
/// The canonical result uses the same mathematical sixteen-bit source sample
/// for each pair: P13.3 encodes it big-endian; P13.2 rounds it by 257.
pub open spec fn image(input: Seq<u8>,a: r::Raster,colour: c::Context,wide: bool) -> Seq<u8> {
    let bytes=if wide {8nat}else{4nat};
    Seq::new(bytes*a.width*a.height,|j: int| {
        let i=j as nat/bytes;
        let px=p::source_pixel(input,a,colour,i%a.width,i/a.width,true,false).get_Ok_0();
        if wide {c::physical(px,true)[j%8]}else{((px[j%4] as nat+128)/257) as u8}
    })
}
pub open spec fn MS_wide(b: Seq<u8>,wide: bool) -> Outcome {
    match d::MW_p(b,d::strict()) {
        d::Outcome::Error(e,_)=>Outcome::Error(e),
        d::Outcome::Done(doc,_)=>match r::document_data(b,doc,crate::policy::STRICT) {
            Some(Err(e))=>Outcome::Error(e),
            Some(Ok(_))=>match d::default_stream(b,doc) {
                Some(z)=>Outcome::Done(Value {width:doc.ihdr.width,height:doc.ihdr.height,info:doc.info,
                    pixels:image(r::inflated(z,p::default_raster(doc,crate::policy::STRICT),crate::policy::STRICT).out,
                        p::default_raster(doc,crate::policy::STRICT),c::from_doc(doc,b),wide)}),
                None=>Outcome::OutsideDomain,
            },
            None=>Outcome::OutsideDomain,
        },
        _=>Outcome::OutsideDomain,
    }
}
pub open spec fn MS(b: Seq<u8>) -> Outcome {MS_wide(b,false)}
pub open spec fn MS16(b: Seq<u8>) -> Outcome {MS_wide(b,true)}
pub open spec fn value_eq(a: Value,b: Value) -> bool {
    a.width==b.width && a.height==b.height && a.info==b.info && a.pixels==b.pixels
}
pub open spec fn resources(value: Value,wide: bool) -> bool {
    1<=value.width<=crate::limits::MAX_DIMENSION && 1<=value.height<=crate::limits::MAX_DIMENSION
    && value.pixels.len()==(if wide {8nat}else{4nat})*(value.width as nat)*(value.height as nat)
    && value.pixels.len()<=(if wide {2nat}else{1nat})*crate::limits::MAX_OUTPUT
}


pub proof fn successful_pass_row(input: Seq<u8>,a: r::Raster,k: nat,start: nat,row: nat)
    requires r::pw(a,k)>0,start<=row<r::ph(a,k),r::pass_result(input,a,k,start).is_ok(),
    ensures r::row_result(input,a,k,row).is_ok(),
    decreases row-start,
{
    r::pass_step(input,a,k,start);
    if start<row {successful_pass_row(input,a,k,start+1,row);}
}
pub proof fn successful_scan_pass(input: Seq<u8>,a: r::Raster,start: nat,k: nat)
    requires start<=k<r::count(a),r::scan(input,a,start).is_ok(),
    ensures r::pass_result(input,a,k,0).is_ok(),
    decreases k-start,
{
    r::scan_step(input,a,start);
    assert(start<r::count(a));
    let first=r::pass_result(input,a,start,0);let rest=r::scan(input,a,start+1);
    assert(r::chain(first,rest).is_ok());
    assert(first.is_ok());
    if start<k {successful_scan_pass(input,a,start+1,k);}
}
pub proof fn source_pixel_ok(input: Seq<u8>,a: r::Raster,colour: c::Context,x: nat,y: nat)
    requires p::domain(a),crate::model::valid_depth(colour.ct,colour.depth),
        a.ihdr.color_type==colour.ct,a.ihdr.depth==colour.depth,a.entries==colour.plte.len()/3,a.reject_index,
        r::scan(input,a,0).is_ok(),x<a.width,y<a.height,
    ensures p::source_pixel(input,a,colour,x,y,true,false).is_ok(),
        p::source_pixel(input,a,colour,x,y,false,false).is_ok(),
{
    p::source_coordinate(a,x,y);
    let at=p::source_cell(a,x,y);
    successful_scan_pass(input,a,0,at.0);
    successful_pass_row(input,a,at.0,0,at.2);
    let line=r::line(input,a,at.0,at.2);
    if colour.ct==3 {
        assert(colour.depth!=16);
        assert(c::sample(line,colour.depth,at.1)==r::palette_sample(line,colour.depth,at.1));
        assert((r::palette_sample(line,colour.depth,at.1) as nat)<a.entries);
    }
}

/// Observe a canonical pixel block without a native Sink or output buffer.
#[verifier::spinoff_prover]
pub proof fn canonical_observe(input: Seq<u8>,a: r::Raster,colour: c::Context,wide: bool,x: nat,y: nat)
    requires a.width>=1,a.height>=1,x<a.width,y<a.height,crate::model::valid_depth(colour.ct,colour.depth),
        p::source_pixel(input,a,colour,x,y,wide,false).is_ok(),
    ensures {
        let state=image(input,a,colour,wide);
        c::observe((if wide {1nat}else{0nat},state.len(),0nat,0nat,0nat,false),state,a.width,x,y)
            ==c::physical(p::source_pixel(input,a,colour,x,y,wide,false).get_Ok_0(),wide)
    },
{
    hide(c::pixel);
    let bytes=if wide {8nat}else{4nat};let state=image(input,a,colour,wide);
    let shape=(if wide {1nat}else{0nat},state.len(),0nat,0nat,0nat,false);
    let n=y*a.width+x;let offset=bytes*n;
    let at=p::source_cell(a,x,y);let line=r::line(input,a,at.0,at.2);
    pixel_pair(colour,line,at.1);
    let px=p::source_pixel(input,a,colour,x,y,wide,false).get_Ok_0();
    let large=p::source_pixel(input,a,colour,x,y,true,false).get_Ok_0();
    crate::layout_model::axis_forward(x,a.width,y);
    assert(n/a.width==y && n%a.width==x);
    assert(n<a.width*a.height) by (nonlinear_arith)
        requires n==y*a.width+x,x<a.width,y<a.height,a.width>=1;
    assert(offset+bytes<=state.len()) by (nonlinear_arith)
        requires offset==bytes*n,n<a.width*a.height,state.len()==bytes*a.width*a.height,bytes>=1;
    assert(c::destination_offset(shape,a.width,x,y)==offset);
    c::observed_slice(shape,state,a.width,x,y);
    let actual=c::observe(shape,state,a.width,x,y);let expected=c::physical(px,wide);
    assert(actual.len()==bytes && expected.len()==bytes);
    assert forall|j: int| 0<=j<bytes implies #[trigger] actual[j]==expected[j] by {
        let index=offset as int+j;
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(index,bytes as int,n as int,j);
        assert(index/(bytes as int)==n && index%(bytes as int)==j);
        if wide {assert(actual[j]==c::physical(large,true)[j]);}
        else {
            assert(px[j] as nat==(large[j] as nat+128)/257);
            assert(actual[j]==((large[j] as nat+128)/257) as u8);
        }
    }
    assert(actual =~= expected);
}

/// Every physical block from M5 agrees with the deterministic construction.
#[verifier::spinoff_prover]
pub proof fn image_complete(input: Seq<u8>,a: r::Raster,colour: c::Context,state: Seq<u8>,wide: bool)
    requires a.width>=1,a.height>=1,crate::model::valid_depth(colour.ct,colour.depth),
        p::image_pixels(input,a,colour,state,wide,false),
    ensures state==image(input,a,colour,wide),
{
    hide(c::pixel);
    let bytes=if wide {8nat}else{4nat};let expected=image(input,a,colour,wide);
    p::image_observe(input,a,colour,state,wide,false,0,0);
    assert forall|i: nat| i<a.width*a.height implies
        #[trigger] state.subrange((bytes*i) as int,(bytes*(i+1)) as int)
        ==expected.subrange((bytes*i) as int,(bytes*(i+1)) as int) by {
        let x=i%a.width;let y=i/a.width;
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod(i as int,a.width as int);
        assert(y<a.height) by (nonlinear_arith)
            requires i<a.width*a.height,i==a.width*y+x,x>=0,a.width>=1;
        p::image_observe(input,a,colour,state,wide,false,x,y);
        canonical_observe(input,a,colour,wide,x,y);
        let shape=(if wide {1nat}else{0nat},state.len(),0nat,0nat,0nat,false);
        c::observed_slice(shape,state,a.width,x,y);c::observed_slice(shape,expected,a.width,x,y);
        assert(c::destination_offset(shape,a.width,x,y)==bytes*i);
    }
    assert(state.len()==bytes*(a.width*a.height)) by (nonlinear_arith)
        requires state.len()==bytes*a.width*a.height;
    assert(expected.len()==state.len());
    p::pixel_blocks_equal(state,expected,bytes,a.width*a.height);
}

pub open spec fn total(b: Seq<u8>) -> bool {
    !matches!(MS(b),Outcome::OutsideDomain) && !matches!(MS16(b),Outcome::OutsideDomain)
}
pub proof fn output_bound(w: nat,h: nat,wide: bool)
    requires 4*w*h<=crate::limits::MAX_OUTPUT,
    ensures (if wide {8nat}else{4nat})*w*h<=(if wide {2nat}else{1nat})*crate::limits::MAX_OUTPUT,
{
    if wide {
        assert(8*w*h==2*(4*w*h)) by (nonlinear_arith);
    }
}
/// A checked theorem harness, not a decoder replacement. The independent
/// models above never call this function. Its fixed stack window instantiates
/// M2's total measuring contract without a fallible allocation or pixel buffer.
#[verifier::spinoff_prover]
pub fn model_totality(b: &[u8])
    ensures total(b@),walk_header(b@,d::strict()),walk_context(b@),
        match MS(b@) {Outcome::Done(v)=>resources(v,false),_=>true},
        match MS16(b@) {Outcome::Done(v)=>resources(v,true),_=>true},
{
    hide(d::MW_p);hide(r::document_data);hide(inflate_verus::model::Inflate);
    proof {crate::walk_policy::profiles_project_exactly();}
    let mut events=crate::png::Events {r1:false,r3:false,r4:false};
    let result=crate::callers::still(b,&crate::policy::STRICT,&mut events);
    match result {
        Err(_)=>{},
        Ok(doc)=>{
            proof {doc.colour_context(b@);}
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
            let measured=inflate_verus::inflate_zlib_measure(&mut source,ring,limit,inflate_verus::InflateOptions::STRICT);
            proof {
                use inflate_verus::Segments;
                let h=doc.ihdr.view();let a=p::default_raster(doc.view(),crate::policy::STRICT);
                let z=source.concat();
                assert(inflate_verus::status_eq(measured.status, r::inflated(z,a,crate::policy::STRICT).status));
                assert(!matches!(r::inflated(z,a,crate::policy::STRICT).status,inflate_verus::model::InflateStatus::OutsideDomain));
                reveal(r::document_data);
                assert(checked_header(h));
                output_bound(h.width as nat,h.height as nat,false);
                output_bound(h.width as nat,h.height as nat,true);

            }
        },
    }
}

pub proof fn doubled_coordinate(i: nat)
    ensures (2*i)/8==i/4,(2*i+1)/8==i/4,(2*i)%8==2*(i%4),(2*i+1)%8==2*(i%4)+1,
{
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod(i as int,4);
    let q=i/4;let rem=i%4;
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse((2*i) as int,8,q as int,(2*rem) as int);
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse((2*i+1) as int,8,q as int,(2*rem+1) as int);
}
/// Sample i of the big-endian byte model and its RGBA8 rounded value.
pub open spec fn sample_relation(input: Seq<u8>,a: r::Raster,colour: c::Context,i: int) -> bool {
    let small=image(input,a,colour,false);let big=image(input,a,colour,true);
    let n=i as nat/4;let px=p::source_pixel(input,a,colour,n%a.width,n/a.width,true,false).get_Ok_0();
    px[i%4] as nat==256*(big[2*i] as nat)+big[2*i+1] as nat
    && small[i] as nat==(px[i%4] as nat+128)/257
}
pub proof fn image_pair_at(input: Seq<u8>,a: r::Raster,colour: c::Context,i: int)
    requires 0<=i<image(input,a,colour,false).len(),
    ensures sample_relation(input,a,colour,i),
{
    hide(c::pixel);
    doubled_coordinate(i as nat);
    let n=i as nat/4;let px=p::source_pixel(input,a,colour,n%a.width,n/a.width,true,false).get_Ok_0();
    c::physical_word(px[i%4]);
    let small=image(input,a,colour,false);let big=image(input,a,colour,true);
    assert(small.len()==4*a.width*a.height);assert(big.len()==8*a.width*a.height);
    assert(big.len()==2*small.len()) by (nonlinear_arith)
        requires small.len()==4*a.width*a.height,big.len()==8*a.width*a.height;
    assert(0<=2*i<big.len() && 0<=2*i+1<big.len());
    assert(small[i]==((px[i%4] as nat+128)/257) as u8);
    assert(big[2*i]==(px[i%4]>>8) as u8);
    assert(big[2*i+1]==(px[i%4]&255) as u8);
}
/// The adjacent-byte E3 relation and half-up rounding hold at every sample.
pub proof fn image_pair(input: Seq<u8>,a: r::Raster,colour: c::Context)
    ensures image(input,a,colour,true).len()==2*image(input,a,colour,false).len(),
        forall|i: int| 0<=i<image(input,a,colour,false).len() ==> #[trigger] sample_relation(input,a,colour,i),
{
    hide(c::pixel);
    let small=image(input,a,colour,false);let big=image(input,a,colour,true);
    assert(small.len()==4*a.width*a.height);assert(big.len()==8*a.width*a.height);
    assert(big.len()==2*small.len()) by (nonlinear_arith)
        requires small.len()==4*a.width*a.height,big.len()==8*a.width*a.height;
    assert forall|i: int| 0<=i<image(input,a,colour,false).len() implies #[trigger] sample_relation(input,a,colour,i) by {
        image_pair_at(input,a,colour,i);
    }
}
pub open spec fn coupled(b: Seq<u8>) -> bool {
    match (MS(b),MS16(b)) {
        (Outcome::Error(a),Outcome::Error(z))=>a==z,
        (Outcome::OutsideDomain,Outcome::OutsideDomain)=>false,
        (Outcome::Done(a),Outcome::Done(z))=>a.width==z.width && a.height==z.height && a.info==z.info
            && z.pixels.len()==2*a.pixels.len()
            && forall|i: int| 0<=i<a.pixels.len() ==> #[trigger] a.pixels[i] as nat==
                (256*(z.pixels[2*i] as nat)+z.pixels[2*i+1] as nat+128)/257,
        _=>false,
    }
}
pub proof fn pair_when_total(b: Seq<u8>)
    requires total(b),
    ensures coupled(b),
{
    hide(d::MW_p);hide(r::document_data);hide(c::pixel);
    match d::MW_p(b,d::strict()) {
        d::Outcome::Done(doc,_)=>match d::default_stream(b,doc) {
            Some(z)=>{
                let a=p::default_raster(doc,crate::policy::STRICT);let colour=c::from_doc(doc,b);
                let input=r::inflated(z,a,crate::policy::STRICT).out;
                image_pair(input,a,colour);
                let small=image(input,a,colour,false);let big=image(input,a,colour,true);
                assert forall|i: int| 0<=i<small.len() implies #[trigger] small[i] as nat==
                    (256*(big[2*i] as nat)+big[2*i+1] as nat+128)/257 by {
                    image_pair_at(input,a,colour,i);
                }
            },None=>{},
        },_=>{},
    }
}

pub open spec fn sample_at(bytes: Seq<u8>,wide: bool,i: nat) -> nat {
    if wide {256*(bytes[(2*i) as int] as nat)+bytes[(2*i+1) as int] as nat}
    else {bytes[i as int] as nat}
}
#[verifier::spinoff_prover]
pub proof fn block_samples(input: Seq<u8>,a: r::Raster,colour: c::Context,wide: bool,x: nat,y: nat)
    requires a.width>=1,a.height>=1,x<a.width,y<a.height,crate::model::valid_depth(colour.ct,colour.depth),
        p::source_pixel(input,a,colour,x,y,wide,false).is_ok(),
    ensures forall|j: nat| j<4 ==> #[trigger] sample_at(image(input,a,colour,wide),wide,4*(y*a.width+x)+j)
        ==p::source_pixel(input,a,colour,x,y,wide,false).get_Ok_0()[j as int] as nat,
{
    hide(c::pixel);
    canonical_observe(input,a,colour,wide,x,y);
    let at=p::source_cell(a,x,y);let line=r::line(input,a,at.0,at.2);pixel_pair(colour,line,at.1);
    let px=p::source_pixel(input,a,colour,x,y,wide,false).get_Ok_0();let state=image(input,a,colour,wide);
    let shape=(if wide {1nat}else{0nat},state.len(),0nat,0nat,0nat,false);
    let n=y*a.width+x;let count=if wide {8nat}else{4nat};
    c::observed_slice(shape,state,a.width,x,y);
    assert(c::destination_offset(shape,a.width,x,y)==count*n);
    let offset=count*n;let encoded=c::physical(px,wide);
    assert(n<a.width*a.height) by (nonlinear_arith)
        requires n==y*a.width+x,x<a.width,y<a.height,a.width>=1;
    assert(offset+count<=state.len()) by (nonlinear_arith)
        requires offset==count*n,n<a.width*a.height,state.len()==count*a.width*a.height,count>=1;
    assert(state.subrange(offset as int,(offset+count) as int)==encoded);
    assert forall|j: nat| j<4 implies #[trigger] sample_at(state,wide,4*n+j)==px[j as int] as nat by {
        if wide {
            c::physical_word(px[j as int]);
            assert(state[(8*n+2*j) as int]==encoded[(2*j) as int]);
            assert(state[(8*n+2*j+1) as int]==encoded[(2*j+1) as int]);
            assert(encoded[(2*j) as int]==(px[j as int]>>8) as u8);
            assert(encoded[(2*j+1) as int]==(px[j as int]&255) as u8);
        }else {
            assert(px[j as int]<=255);
            assert(state[(4*n+j) as int]==encoded[j as int]);
        }
    }
}
pub open spec fn pixel_package(input: Seq<u8>,a: r::Raster,colour: c::Context,x: nat,y: nat) -> bool {
    let at=p::source_cell(a,x,y);let line=r::line(input,a,at.0,at.2);
    let small=p::source_pixel(input,a,colour,x,y,false,false);let big=p::source_pixel(input,a,colour,x,y,true,false);
    small.is_ok() && big.is_ok()
    && crate::still_samples::colour_properties(colour,line,at.1,false,small.get_Ok_0())
    && crate::still_samples::colour_properties(colour,line,at.1,true,big.get_Ok_0())
    && forall|j: nat| j<4 ==> #[trigger] sample_at(image(input,a,colour,false),false,4*(y*a.width+x)+j)==small.get_Ok_0()[j as int] as nat
        && sample_at(image(input,a,colour,true),true,4*(y*a.width+x)+j)==big.get_Ok_0()[j as int] as nat
        && (colour.depth<=8 ==> sample_at(image(input,a,colour,true),true,4*(y*a.width+x)+j)
            ==257*sample_at(image(input,a,colour,false),false,4*(y*a.width+x)+j))
}
#[verifier::spinoff_prover]
pub proof fn data_pixel_properties(input: Seq<u8>,a: r::Raster,colour: c::Context,x: nat,y: nat)
    requires p::domain(a),crate::model::valid_depth(colour.ct,colour.depth),
        a.ihdr.color_type==colour.ct,a.ihdr.depth==colour.depth,a.entries==colour.plte.len()/3,a.reject_index,
        r::scan(input,a,0).is_ok(),x<a.width,y<a.height,
    ensures pixel_package(input,a,colour,x,y),
{
    hide(c::pixel);
    source_pixel_ok(input,a,colour,x,y);
    let at=p::source_cell(a,x,y);let line=r::line(input,a,at.0,at.2);
    crate::still_samples::pixel_properties(colour,line,at.1,false);
    crate::still_samples::pixel_properties(colour,line,at.1,true);
    pixel_pair(colour,line,at.1);
    block_samples(input,a,colour,false,x,y);block_samples(input,a,colour,true,x,y);
}

pub open spec fn walk_context(b: Seq<u8>) -> bool {
    match d::MW_p(b,d::strict()) {
        d::Outcome::Done(doc,_)=>checked_header(doc.ihdr) && doc.info.header==doc.ihdr
            && c::context_domain(c::from_doc(doc,b)) && d::entries(doc)==c::from_doc(doc,b).plte.len()/3
            && d::default_stream(b,doc).is_some() && r::document_data(b,doc,crate::policy::STRICT).is_some(),
        d::Outcome::Error(_,_)=>true,_=>false,
    }
}
pub open spec fn ready(b: Seq<u8>,doc: d::Doc) -> bool {
    match r::document_data(b,doc,crate::policy::STRICT) {Some(result)=>result.is_ok(),None=>false}
}
pub open spec fn P13(b: Seq<u8>) -> bool {
    match d::MW_p(b,d::strict()) {
        d::Outcome::Done(doc,_)=>match d::default_stream(b,doc) {
            Some(z)=>ready(b,doc) ==> {
                let a=p::default_raster(doc,crate::policy::STRICT);let colour=c::from_doc(doc,b);
                let input=r::inflated(z,a,crate::policy::STRICT).out;
                forall|x: nat,y: nat| x<a.width && y<a.height ==> #[trigger] pixel_package(input,a,colour,x,y)
            },None=>false,
        },d::Outcome::Error(_,_)=>true,_=>false,
    }
}
pub proof fn model_colour_properties(b: Seq<u8>)
    requires walk_context(b),
    ensures P13(b),
{
    hide(d::MW_p);hide(c::pixel);hide(inflate_verus::model::Inflate);
    crate::limits::caps_ok();
    match d::MW_p(b,d::strict()) {
        d::Outcome::Done(doc,_)=>match d::default_stream(b,doc) {
            Some(z)=>{
                if ready(b,doc) {
                    let a=p::default_raster(doc,crate::policy::STRICT);let colour=c::from_doc(doc,b);
                    let input=r::inflated(z,a,crate::policy::STRICT).out;
                    assert(p::domain(a));assert(r::scan(input,a,0).is_ok());
                    assert forall|x: nat,y: nat| x<a.width && y<a.height implies #[trigger] pixel_package(input,a,colour,x,y) by {
                        data_pixel_properties(input,a,colour,x,y);
                    }
                }
            },None=>{},
        },_=>{},
    }
}

/// The result-facing T13 package has no separate data-success antecedent.
pub open spec fn T13(b: Seq<u8>,wide: bool) -> bool {
    match MS_wide(b,wide) {
        Outcome::Done(value)=>resources(value,wide) && match d::MW_p(b,d::strict()) {
            d::Outcome::Done(doc,_)=>match d::default_stream(b,doc) {
                Some(z)=>{
                    let a=p::default_raster(doc,crate::policy::STRICT);let colour=c::from_doc(doc,b);
                    let input=r::inflated(z,a,crate::policy::STRICT).out;
                    forall|x: nat,y: nat| #![trigger p::source_pixel(input,a,colour,x,y,wide,false)] x<a.width && y<a.height ==> {
                        let at=p::source_cell(a,x,y);let line=r::line(input,a,at.0,at.2);
                        let px=p::source_pixel(input,a,colour,x,y,wide,false);
                        px.is_ok() && crate::still_samples::colour_properties(colour,line,at.1,wide,px.get_Ok_0())
                        && forall|j: nat| j<4 ==> #[trigger] sample_at(value.pixels,wide,4*(y*a.width+x)+j)==px.get_Ok_0()[j as int] as nat
                    }
                },None=>false,
            },_=>false,
        },_=>true,
    }
}
#[verifier::spinoff_prover]
pub proof fn bind_model_properties(b: Seq<u8>,wide: bool)
    requires walk_context(b),P13(b),match MS_wide(b,wide) {Outcome::Done(v)=>resources(v,wide),_=>true},
    ensures T13(b,wide),
{
    hide(d::MW_p);hide(c::pixel);hide(inflate_verus::model::Inflate);
    match d::MW_p(b,d::strict()) {
        d::Outcome::Done(doc,_)=>match d::default_stream(b,doc) {
            Some(z)=>if let Outcome::Done(value)=MS_wide(b,wide) {
                let a=p::default_raster(doc,crate::policy::STRICT);let colour=c::from_doc(doc,b);
                let input=r::inflated(z,a,crate::policy::STRICT).out;
                assert(ready(b,doc));
                assert(value.pixels==image(input,a,colour,wide));
                assert forall|x: nat,y: nat| #![trigger p::source_pixel(input,a,colour,x,y,wide,false)] x<a.width && y<a.height implies {
                    let at=p::source_cell(a,x,y);let line=r::line(input,a,at.0,at.2);
                    let px=p::source_pixel(input,a,colour,x,y,wide,false);
                    px.is_ok() && crate::still_samples::colour_properties(colour,line,at.1,wide,px.get_Ok_0())
                    && forall|j: nat| j<4 ==> #[trigger] sample_at(value.pixels,wide,4*(y*a.width+x)+j)==px.get_Ok_0()[j as int] as nat
                } by {
                    assert(pixel_package(input,a,colour,x,y));
                    let px=p::source_pixel(input,a,colour,x,y,wide,false);
                    assert(px.is_ok());
                    block_samples(input,a,colour,wide,x,y);
                    assert forall|j: nat| j<4 implies #[trigger] sample_at(value.pixels,wide,4*(y*a.width+x)+j)==px.get_Ok_0()[j as int] as nat by {
                        assert(sample_at(image(input,a,colour,wide),wide,4*(y*a.width+x)+j)==px.get_Ok_0()[j as int] as nat);
                    }
                }
            },None=>{},
        },_=>{},
    }
}

/// Unconditional model-side theorem for every input slice. It instantiates
/// earlier checked contracts, never a public still decoder or allocator.
pub fn model_theorems(b: &[u8])
    ensures total(b@),coupled(b@),P13(b@),T13(b@,false),T13(b@,true),
        match MS(b@) {Outcome::Done(v)=>resources(v,false),_=>true},
        match MS16(b@) {Outcome::Done(v)=>resources(v,true),_=>true},
{
    model_totality(b);
    proof {pair_when_total(b@);model_colour_properties(b@);bind_model_properties(b@,false);bind_model_properties(b@,true);}
}
}
