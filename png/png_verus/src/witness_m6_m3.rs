//! Complete E6 clause-2 Strict witnesses; no projection credit is claimed.
use vstd::prelude::*;
use crate::{document_model as m,raster_model as r,filter_model as f,png::Error,policy::STRICT,
    still_model as sm,still_equivalence as se,colour_model as colour,alloc_guard::TryError,
    m4_trace_witness_model as t,m4_checksum_witness_model as c,m4_stream_witness_model as stream,m5_stream_witness_model as ms,m4_crc_witness_model as crc_cert};
use inflate_verus::model as zm;
verus! {


#[verifier::spinoff_prover]
proof fn stored_structure(doc: m::Doc,value: crate::metadata_model::Value,base: nat)
    ensures m::store(doc,value,base).ihdr==doc.ihdr,m::store(doc,value,base).plte==doc.plte,
        m::store(doc,value,base).trns==doc.trns,m::store(doc,value,base).actl==doc.actl,
        m::store(doc,value,base).idat==doc.idat,m::store(doc,value,base).frames==doc.frames,
{}
#[verifier::spinoff_prover]
proof fn iccp_success(d: Seq<u8>,z: nat)
    requires crate::model::first_nul(d,0)==Some(z),crate::model::keyword(d.subrange(0,z as int)),
        z+1<d.len(),d[z as int+1]==0,
    ensures crate::metadata_model::parse(crate::walk_model::Name::Iccp,d,2,8,0)==crate::metadata_model::Outcome::Value(
        crate::metadata_model::Value::Iccp {name_len:z,profile_start:z+2,profile_len:(d.len()-z-2) as nat}),
{
    hide(crate::model::first_nul);hide(crate::model::keyword);
}
#[verifier::spinoff_prover]
proof fn text_success(d: Seq<u8>,z: nat)
    requires crate::model::first_nul(d,0)==Some(z),crate::model::keyword(d.subrange(0,z as int)),
    ensures crate::metadata_model::parse(crate::walk_model::Name::Text,d,2,8,0)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored),
{
    hide(crate::model::first_nul);hide(crate::model::keyword);
}
#[verifier::spinoff_prover]
proof fn ztxt_success(d: Seq<u8>,z: nat)
    requires crate::model::first_nul(d,0)==Some(z),crate::model::keyword(d.subrange(0,z as int)),
        z+1<d.len(),d[z as int+1]==0,
    ensures crate::metadata_model::parse(crate::walk_model::Name::Ztxt,d,2,8,0)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored),
{
    hide(crate::model::first_nul);hide(crate::model::keyword);
}
#[verifier::spinoff_prover]
proof fn rgb_background_success(d: Seq<u8>,entries: nat)
    requires d.len()==6,entries<=256,
        forall|i: int| 0<=i<3 ==> (#[trigger] crate::metadata_model::words16(d,3)[i] as nat)<(1u32<<8u8) as nat,
    ensures crate::metadata_model::parse(crate::walk_model::Name::Bkgd,d,2,8,entries)==crate::metadata_model::Outcome::Value(
        crate::metadata_model::Value::Bkgd(crate::metadata_model::words16(d,3))),
{}


// Only compose the accepted independent definitions; no executable is called.
#[verifier::spinoff_prover]
proof fn compose_complete(b: Seq<u8>,h: crate::model::Ihdr,z: Seq<u8>,entries: nat,col: colour::Context,pixels: Seq<u8>)
    requires r::default_case(b,h,z,entries),
        r::data(z,r::shape(h,h.width as nat,h.height as nat,entries,STRICT),STRICT)==Some(Ok(())),
        match m::MW_p(b,m::strict()) {m::Outcome::Done(doc,_)=>colour::from_doc(doc,b)==col,_=>false},
        sm::image(r::inflated(z,r::shape(h,h.width as nat,h.height as nat,entries,STRICT),STRICT).out,
            r::shape(h,h.width as nat,h.height as nat,entries,STRICT),col,false)==pixels,
    ensures match sm::MS(b) {sm::Outcome::Done(value)=>value.width==h.width && value.height==h.height && value.pixels==pixels,_=>false},
{
    hide(m::MW_p);hide(zm::Inflate);
    match m::MW_p(b,m::strict()) {m::Outcome::Done(doc,_)=>{
        assert(r::document_shape(doc,STRICT)==r::shape(h,h.width as nat,h.height as nat,entries,STRICT));
        assert(r::document_data(b,doc,STRICT)==Some(Ok(())));
        assert(crate::paint_model::default_raster(doc,STRICT)==r::shape(h,h.width as nat,h.height as nat,entries,STRICT));
    },_=>{}}
}

// container/minimal-ok; frozen fixture SHA256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
pub open spec fn bytes_container_minimal_ok() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn h_container_minimal_ok() -> crate::model::Ihdr {crate::model::Ihdr {width:2u32,height:2u32,depth:8u8,color_type:2u8,interlace:0u8}}
spec fn a_container_minimal_ok() -> r::Raster {r::shape(h_container_minimal_ok(),2nat,2nat,0nat,STRICT)}
spec fn colour_container_minimal_ok() -> crate::colour_model::Context {crate::colour_model::Context {ct:2u8,depth:8u8,plte:seq![],trns:None}}
pub open spec fn expected_container_minimal_ok() -> Seq<u8> {seq![201u8,73u8,63u8,255u8,202u8,75u8,151u8,255u8,225u8,26u8,23u8,255u8,252u8,246u8,129u8,255u8]}
spec fn z_container_minimal_ok() -> Seq<u8> {seq![120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8]}
spec fn raw_container_minimal_ok() -> Seq<u8> {seq![0u8,201u8,73u8,63u8,202u8,75u8,151u8,1u8,225u8,26u8,23u8,27u8,220u8,106u8]}
spec fn ts_container_minimal_ok() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:36nat,before:1nat,after:2nat,symbol:201nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:36nat,next:44nat,before:2nat,after:3nat,symbol:73nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:44nat,next:52nat,before:3nat,after:4nat,symbol:63nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:52nat,next:61nat,before:4nat,after:5nat,symbol:202nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:61nat,next:69nat,before:5nat,after:6nat,symbol:75nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:69nat,next:78nat,before:6nat,after:7nat,symbol:151nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:78nat,next:86nat,before:7nat,after:8nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:86nat,next:95nat,before:8nat,after:9nat,symbol:225nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:95nat,next:103nat,before:9nat,after:10nat,symbol:26nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:103nat,next:111nat,before:10nat,after:11nat,symbol:23nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:111nat,next:119nat,before:11nat,after:12nat,symbol:27nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:119nat,next:128nat,before:12nat,after:13nat,symbol:220nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:128nat,next:136nat,before:13nat,after:14nat,symbol:106nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:136nat,next:143nat,before:14nat,after:14nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_container_minimal_ok_0() ensures t::trace(z_container_minimal_ok(),raw_container_minimal_ok(),ts_container_minimal_ok(),0nat,15nat),
{
    hide(t::valid_step);assert(t::trace(z_container_minimal_ok(),raw_container_minimal_ok(),ts_container_minimal_ok(),0nat,15nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn trace_container_minimal_ok() ensures t::trace(z_container_minimal_ok(),raw_container_minimal_ok(),ts_container_minimal_ok(),0,15nat),
{
    hide(z_container_minimal_ok);hide(raw_container_minimal_ok);hide(ts_container_minimal_ok);hide(t::trace);
    trace_container_minimal_ok_0();
    assert(t::trace(z_container_minimal_ok(),raw_container_minimal_ok(),ts_container_minimal_ok(),0,15nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_container_minimal_ok() ensures z_container_minimal_ok().len()==22nat,
        raw_container_minimal_ok().len()==14nat,
        z_container_minimal_ok()[0]==120u8,
        z_container_minimal_ok()[1]==156u8,
        zm::field_value(z_container_minimal_ok(),16,1)==1nat,
        zm::field_value(z_container_minimal_ok(),17,2)==1nat,
        zm::be_value(z_container_minimal_ok(),18nat,4)==675349874nat,
        ts_container_minimal_ok().len()==15nat,
        ts_container_minimal_ok()[0].pos==19nat,
        ts_container_minimal_ok()[0].before==0nat,
        ts_container_minimal_ok().last().next==143nat,
{
    assert(z_container_minimal_ok().len()==22nat) by (compute);
    assert(raw_container_minimal_ok().len()==14nat) by (compute);
    assert(z_container_minimal_ok()[0]==120u8) by (compute);
    assert(z_container_minimal_ok()[1]==156u8) by (compute);
    assert(zm::field_value(z_container_minimal_ok(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_container_minimal_ok(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_container_minimal_ok(),18nat,4)==675349874nat) by (compute);
    assert(ts_container_minimal_ok().len()==15nat) by (compute);
    assert(ts_container_minimal_ok()[0].pos==19nat) by (compute);
    assert(ts_container_minimal_ok()[0].before==0nat) by (compute);
    assert(ts_container_minimal_ok().last().next==143nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_container_minimal_ok() ensures zm::Adler(raw_container_minimal_ok())==675349874nat,
{
    hide(raw_container_minimal_ok);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_container_minimal_ok().len()==14nat) by (compute);
    assert(c::sums(raw_container_minimal_ok(),0,14nat)==(1393nat,10305nat)) by (compute);
    c::sums_correct(raw_container_minimal_ok(),0,14nat);assert(raw_container_minimal_ok().subrange(0,14)=~=raw_container_minimal_ok());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((10305nat%65521)*(u16::MAX as nat+1)+(1+1393nat)%65521==675349874nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_container_minimal_ok() ensures zm::Inflate(z_container_minimal_ok(),14nat,true,false)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_container_minimal_ok(),r2:false}),raw_container_minimal_ok().len()==14nat,
{
    hide(z_container_minimal_ok);hide(raw_container_minimal_ok);hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);hide(ts_container_minimal_ok);
    shape_container_minimal_ok();checksum_container_minimal_ok();
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_container_minimal_ok();ms::fixed_stream(z_container_minimal_ok(),raw_container_minimal_ok(),ts_container_minimal_ok(),14nat,156nat,675349874nat,675349874nat);
}
#[verifier::spinoff_prover]
proof fn walk_container_minimal_ok(b: Seq<u8>)
    requires b.len()==79,
        crate::model::mh(b)==Ok((h_container_minimal_ok(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:22nat,next:67nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:75nat,len:0nat,next:79nat,crc_ok:true,synthetic:false})),
        b.subrange(41,63)==z_container_minimal_ok(),
    ensures r::default_case(b,h_container_minimal_ok(),z_container_minimal_ok(),0nat),match m::MW_p(b,m::strict()) {m::Outcome::Done(doc,_)=>doc.plte==None && doc.trns==None,_=>false},
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(m::store);hide(crate::metadata_model::parse);
    assert forall|doc: m::Doc,value: crate::metadata_model::Value,base: nat|
        (#[trigger] m::store(doc,value,base)).ihdr==doc.ihdr && m::store(doc,value,base).plte==doc.plte
        && m::store(doc,value,base).trns==doc.trns && m::store(doc,value,base).actl==doc.actl
        && m::store(doc,value,base).idat==doc.idat && m::store(doc,value,base).frames==doc.frames by {
        stored_structure(doc,value,base);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    reveal_with_fuel(m::drive,3);
    reveal_with_fuel(m::concat_ranges,3);
}
#[verifier::spinoff_prover]
proof fn scan_container_minimal_ok() ensures r::scan(raw_container_minimal_ok(),a_container_minimal_ok(),0)==Ok(()),
{
    hide(raw_container_minimal_ok);assert(raw_container_minimal_ok().len()==14nat) by (compute);assert(r::count(a_container_minimal_ok())==1nat) by (compute);
    assert(r::pw(a_container_minimal_ok(),0nat)==2nat && r::ph(a_container_minimal_ok(),0nat)==2nat && r::rb(a_container_minimal_ok(),0nat)==6nat && r::base(a_container_minimal_ok(),0nat)==0nat) by (compute);
    assert(raw_container_minimal_ok()[0]==0u8) by (compute);
    assert(r::row_result(raw_container_minimal_ok(),a_container_minimal_ok(),0nat,0nat)==Ok(()));
    assert(raw_container_minimal_ok()[7]==1u8) by (compute);
    assert(r::row_result(raw_container_minimal_ok(),a_container_minimal_ok(),0nat,1nat)==Ok(()));
    r::scan_step(raw_container_minimal_ok(),a_container_minimal_ok(),1nat);
    r::pass_step(raw_container_minimal_ok(),a_container_minimal_ok(),0nat,2nat);
    r::pass_step(raw_container_minimal_ok(),a_container_minimal_ok(),0nat,1nat);
    r::pass_step(raw_container_minimal_ok(),a_container_minimal_ok(),0nat,0nat);
    r::scan_step(raw_container_minimal_ok(),a_container_minimal_ok(),0nat);
}
#[verifier::spinoff_prover]
proof fn row_container_minimal_ok_0_0() ensures r::line(raw_container_minimal_ok(),a_container_minimal_ok(),0nat,0nat)==seq![201u8,73u8,63u8,202u8,75u8,151u8],
{
    hide(raw_container_minimal_ok);
    assert(raw_container_minimal_ok().len()==14nat) by (compute);
    assert(raw_container_minimal_ok()[0]==0u8) by (compute);
    assert(r::base(a_container_minimal_ok(),0nat)==0nat && r::rb(a_container_minimal_ok(),0nat)==6nat && f::bpp(r::bits(a_container_minimal_ok()))==3nat) by (compute);
    assert(f::row_data(raw_container_minimal_ok(),0nat,6nat,0nat)==seq![201u8,73u8,63u8,202u8,75u8,151u8]) by (compute);
    f::pass_row_equation(raw_container_minimal_ok(),0nat,6nat,3nat,0nat);
    assert(f::previous_row(raw_container_minimal_ok(),0nat,6nat,3nat,0nat)==None);
    assert(f::encoded_row(0u8,f::row_data(raw_container_minimal_ok(),0nat,6nat,0nat),None,3nat,seq![201u8,73u8,63u8,202u8,75u8,151u8],0,6nat)) by (compute);
    f::checked_row_hint(0u8,f::row_data(raw_container_minimal_ok(),0nat,6nat,0nat),None,3nat,seq![201u8,73u8,63u8,202u8,75u8,151u8]);
}
#[verifier::spinoff_prover]
proof fn row_container_minimal_ok_0_1() ensures r::line(raw_container_minimal_ok(),a_container_minimal_ok(),0nat,1nat)==seq![225u8,26u8,23u8,252u8,246u8,129u8],
{
    hide(raw_container_minimal_ok);
    assert(raw_container_minimal_ok().len()==14nat) by (compute);
    assert(raw_container_minimal_ok()[7]==1u8) by (compute);
    assert(r::base(a_container_minimal_ok(),0nat)==0nat && r::rb(a_container_minimal_ok(),0nat)==6nat && f::bpp(r::bits(a_container_minimal_ok()))==3nat) by (compute);
    assert(f::row_data(raw_container_minimal_ok(),0nat,6nat,1nat)==seq![225u8,26u8,23u8,27u8,220u8,106u8]) by (compute);
    row_container_minimal_ok_0_0();
    f::pass_row_equation(raw_container_minimal_ok(),0nat,6nat,3nat,1nat);
    assert(f::previous_row(raw_container_minimal_ok(),0nat,6nat,3nat,1nat)==Some(seq![201u8,73u8,63u8,202u8,75u8,151u8]));
    assert(f::encoded_row(1u8,f::row_data(raw_container_minimal_ok(),0nat,6nat,1nat),Some(seq![201u8,73u8,63u8,202u8,75u8,151u8]),3nat,seq![225u8,26u8,23u8,252u8,246u8,129u8],0,6nat)) by (compute);
    f::checked_row_hint(1u8,f::row_data(raw_container_minimal_ok(),0nat,6nat,1nat),Some(seq![201u8,73u8,63u8,202u8,75u8,151u8]),3nat,seq![225u8,26u8,23u8,252u8,246u8,129u8]);
}
#[verifier::spinoff_prover]
proof fn raster_container_minimal_ok()
    ensures r::default_case(bytes_container_minimal_ok(),h_container_minimal_ok(),z_container_minimal_ok(),0nat),match m::MW_p(bytes_container_minimal_ok(),m::strict()) {m::Outcome::Done(doc,_)=>doc.plte==None && doc.trns==None,_=>false},r::data(z_container_minimal_ok(),a_container_minimal_ok(),STRICT)==Some(Ok(())),
{
    hide(bytes_container_minimal_ok);hide(z_container_minimal_ok);hide(raw_container_minimal_ok);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    assert(bytes_container_minimal_ok().len()==79nat) by (compute);
    assert(crate::model::mh(bytes_container_minimal_ok())==Ok((h_container_minimal_ok(),33nat))) by {reveal(crate::model::mh);assert(crate::model::mh(bytes_container_minimal_ok())==Ok((h_container_minimal_ok(),33nat))) by (compute);}
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_minimal_ok(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:22nat,next:67nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_minimal_ok(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:22nat,next:67nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_minimal_ok(),67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:75nat,len:0nat,next:79nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_minimal_ok(),67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:75nat,len:0nat,next:79nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_container_minimal_ok().subrange(41,63)==z_container_minimal_ok()) by (compute);
    walk_container_minimal_ok(bytes_container_minimal_ok());
    inflate_container_minimal_ok();scan_container_minimal_ok();
    assert(r::raw(a_container_minimal_ok())==14nat) by (compute);
    assert(r::data(z_container_minimal_ok(),a_container_minimal_ok(),STRICT)==Some(Ok(()))) by {reveal(r::data);};
}
#[verifier::spinoff_prover]
proof fn pixel_container_minimal_ok_0() ensures sm::image(raw_container_minimal_ok(),a_container_minimal_ok(),colour_container_minimal_ok(),false)[0]==201u8,sm::image(raw_container_minimal_ok(),a_container_minimal_ok(),colour_container_minimal_ok(),false)[1]==73u8,sm::image(raw_container_minimal_ok(),a_container_minimal_ok(),colour_container_minimal_ok(),false)[2]==63u8,sm::image(raw_container_minimal_ok(),a_container_minimal_ok(),colour_container_minimal_ok(),false)[3]==255u8,
{
    colour::depth_values(8u8);
    row_container_minimal_ok_0_0();
    assert(crate::paint_model::source_cell(a_container_minimal_ok(),0nat,0nat)==(0nat,0nat,0nat)) by (compute);
    assert(colour::pixel(colour_container_minimal_ok(),seq![201u8,73u8,63u8,202u8,75u8,151u8],0nat,true,false)==Ok(seq![51657u16,18761u16,16191u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_container_minimal_ok(),a_container_minimal_ok(),colour_container_minimal_ok(),0nat,0nat,true,false)==Ok(seq![51657u16,18761u16,16191u16,65535u16]));
    assert(((seq![51657u16,18761u16,16191u16,65535u16][0] as nat+128)/257) as u8==201u8) by (compute);
    assert(((seq![51657u16,18761u16,16191u16,65535u16][1] as nat+128)/257) as u8==73u8) by (compute);
    assert(((seq![51657u16,18761u16,16191u16,65535u16][2] as nat+128)/257) as u8==63u8) by (compute);
    assert(((seq![51657u16,18761u16,16191u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixel_container_minimal_ok_1() ensures sm::image(raw_container_minimal_ok(),a_container_minimal_ok(),colour_container_minimal_ok(),false)[4]==202u8,sm::image(raw_container_minimal_ok(),a_container_minimal_ok(),colour_container_minimal_ok(),false)[5]==75u8,sm::image(raw_container_minimal_ok(),a_container_minimal_ok(),colour_container_minimal_ok(),false)[6]==151u8,sm::image(raw_container_minimal_ok(),a_container_minimal_ok(),colour_container_minimal_ok(),false)[7]==255u8,
{
    colour::depth_values(8u8);
    row_container_minimal_ok_0_0();
    assert(crate::paint_model::source_cell(a_container_minimal_ok(),1nat,0nat)==(0nat,1nat,0nat)) by (compute);
    assert(colour::pixel(colour_container_minimal_ok(),seq![201u8,73u8,63u8,202u8,75u8,151u8],1nat,true,false)==Ok(seq![51914u16,19275u16,38807u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_container_minimal_ok(),a_container_minimal_ok(),colour_container_minimal_ok(),1nat,0nat,true,false)==Ok(seq![51914u16,19275u16,38807u16,65535u16]));
    assert(((seq![51914u16,19275u16,38807u16,65535u16][0] as nat+128)/257) as u8==202u8) by (compute);
    assert(((seq![51914u16,19275u16,38807u16,65535u16][1] as nat+128)/257) as u8==75u8) by (compute);
    assert(((seq![51914u16,19275u16,38807u16,65535u16][2] as nat+128)/257) as u8==151u8) by (compute);
    assert(((seq![51914u16,19275u16,38807u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixel_container_minimal_ok_2() ensures sm::image(raw_container_minimal_ok(),a_container_minimal_ok(),colour_container_minimal_ok(),false)[8]==225u8,sm::image(raw_container_minimal_ok(),a_container_minimal_ok(),colour_container_minimal_ok(),false)[9]==26u8,sm::image(raw_container_minimal_ok(),a_container_minimal_ok(),colour_container_minimal_ok(),false)[10]==23u8,sm::image(raw_container_minimal_ok(),a_container_minimal_ok(),colour_container_minimal_ok(),false)[11]==255u8,
{
    colour::depth_values(8u8);
    row_container_minimal_ok_0_1();
    assert(crate::paint_model::source_cell(a_container_minimal_ok(),0nat,1nat)==(0nat,0nat,1nat)) by (compute);
    assert(colour::pixel(colour_container_minimal_ok(),seq![225u8,26u8,23u8,252u8,246u8,129u8],0nat,true,false)==Ok(seq![57825u16,6682u16,5911u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_container_minimal_ok(),a_container_minimal_ok(),colour_container_minimal_ok(),0nat,1nat,true,false)==Ok(seq![57825u16,6682u16,5911u16,65535u16]));
    assert(((seq![57825u16,6682u16,5911u16,65535u16][0] as nat+128)/257) as u8==225u8) by (compute);
    assert(((seq![57825u16,6682u16,5911u16,65535u16][1] as nat+128)/257) as u8==26u8) by (compute);
    assert(((seq![57825u16,6682u16,5911u16,65535u16][2] as nat+128)/257) as u8==23u8) by (compute);
    assert(((seq![57825u16,6682u16,5911u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixel_container_minimal_ok_3() ensures sm::image(raw_container_minimal_ok(),a_container_minimal_ok(),colour_container_minimal_ok(),false)[12]==252u8,sm::image(raw_container_minimal_ok(),a_container_minimal_ok(),colour_container_minimal_ok(),false)[13]==246u8,sm::image(raw_container_minimal_ok(),a_container_minimal_ok(),colour_container_minimal_ok(),false)[14]==129u8,sm::image(raw_container_minimal_ok(),a_container_minimal_ok(),colour_container_minimal_ok(),false)[15]==255u8,
{
    colour::depth_values(8u8);
    row_container_minimal_ok_0_1();
    assert(crate::paint_model::source_cell(a_container_minimal_ok(),1nat,1nat)==(0nat,1nat,1nat)) by (compute);
    assert(colour::pixel(colour_container_minimal_ok(),seq![225u8,26u8,23u8,252u8,246u8,129u8],1nat,true,false)==Ok(seq![64764u16,63222u16,33153u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_container_minimal_ok(),a_container_minimal_ok(),colour_container_minimal_ok(),1nat,1nat,true,false)==Ok(seq![64764u16,63222u16,33153u16,65535u16]));
    assert(((seq![64764u16,63222u16,33153u16,65535u16][0] as nat+128)/257) as u8==252u8) by (compute);
    assert(((seq![64764u16,63222u16,33153u16,65535u16][1] as nat+128)/257) as u8==246u8) by (compute);
    assert(((seq![64764u16,63222u16,33153u16,65535u16][2] as nat+128)/257) as u8==129u8) by (compute);
    assert(((seq![64764u16,63222u16,33153u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixels_container_minimal_ok() ensures sm::image(raw_container_minimal_ok(),a_container_minimal_ok(),colour_container_minimal_ok(),false)==expected_container_minimal_ok(),
{
    pixel_container_minimal_ok_0();
    pixel_container_minimal_ok_1();
    pixel_container_minimal_ok_2();
    pixel_container_minimal_ok_3();
    assert(a_container_minimal_ok().width==2nat && a_container_minimal_ok().height==2nat) by (compute);
    assert(sm::image(raw_container_minimal_ok(),a_container_minimal_ok(),colour_container_minimal_ok(),false).len()==16nat);
    assert(expected_container_minimal_ok().len()==16nat) by (compute);
    assert forall|j: int| 0<=j<16 implies (#[trigger] sm::image(raw_container_minimal_ok(),a_container_minimal_ok(),colour_container_minimal_ok(),false)[j])==expected_container_minimal_ok()[j] by {
        if j==0 {assert(expected_container_minimal_ok()[0]==201u8) by (compute);}
        else if j==1 {assert(expected_container_minimal_ok()[1]==73u8) by (compute);}
        else if j==2 {assert(expected_container_minimal_ok()[2]==63u8) by (compute);}
        else if j==3 {assert(expected_container_minimal_ok()[3]==255u8) by (compute);}
        else if j==4 {assert(expected_container_minimal_ok()[4]==202u8) by (compute);}
        else if j==5 {assert(expected_container_minimal_ok()[5]==75u8) by (compute);}
        else if j==6 {assert(expected_container_minimal_ok()[6]==151u8) by (compute);}
        else if j==7 {assert(expected_container_minimal_ok()[7]==255u8) by (compute);}
        else if j==8 {assert(expected_container_minimal_ok()[8]==225u8) by (compute);}
        else if j==9 {assert(expected_container_minimal_ok()[9]==26u8) by (compute);}
        else if j==10 {assert(expected_container_minimal_ok()[10]==23u8) by (compute);}
        else if j==11 {assert(expected_container_minimal_ok()[11]==255u8) by (compute);}
        else if j==12 {assert(expected_container_minimal_ok()[12]==252u8) by (compute);}
        else if j==13 {assert(expected_container_minimal_ok()[13]==246u8) by (compute);}
        else if j==14 {assert(expected_container_minimal_ok()[14]==129u8) by (compute);}
        else {assert(j==15);assert(expected_container_minimal_ok()[j]==255u8) by (compute);}
    }
    assert(sm::image(raw_container_minimal_ok(),a_container_minimal_ok(),colour_container_minimal_ok(),false)=~=expected_container_minimal_ok());
}
pub open spec fn expected_value_container_minimal_ok(value: sm::Value) -> bool {value.width==2u32 && value.height==2u32 && value.pixels==expected_container_minimal_ok()}
#[verifier::spinoff_prover]
pub proof fn model_complete_container_minimal_ok()
    ensures match sm::MS(bytes_container_minimal_ok()) {sm::Outcome::Done(value)=>expected_value_container_minimal_ok(value),_=>false},
{
    hide(m::MW_p);hide(zm::Inflate);hide(bytes_container_minimal_ok);hide(z_container_minimal_ok);hide(raw_container_minimal_ok);hide(expected_container_minimal_ok);
    assert(bytes_container_minimal_ok().len()==79nat) by (compute);
    raster_container_minimal_ok();inflate_container_minimal_ok();pixels_container_minimal_ok();
    match m::MW_p(bytes_container_minimal_ok(),m::strict()) {m::Outcome::Done(doc,_)=>{
        assert(colour::from_doc(doc,bytes_container_minimal_ok())==colour_container_minimal_ok());
        assert(crate::paint_model::default_raster(doc,STRICT)==a_container_minimal_ok());
    },_=>{}}
    assert(r::raw(a_container_minimal_ok())==14nat) by (compute);
    assert(r::inflated(z_container_minimal_ok(),a_container_minimal_ok(),STRICT).out==raw_container_minimal_ok());
    compose_complete(bytes_container_minimal_ok(),h_container_minimal_ok(),z_container_minimal_ok(),0nat,colour_container_minimal_ok(),expected_container_minimal_ok());
}
pub fn public_complete_container_minimal_ok() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_container_minimal_ok(),false),result),se::eq(bytes_container_minimal_ok(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_container_minimal_ok(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_container_minimal_ok());model_complete_container_minimal_ok();}
    crate::png::try_decode_still(&b)
}
// container/unknown-ancillary-everywhere; frozen fixture SHA256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
pub open spec fn bytes_container_unknown_ancillary_everywhere() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,3u8,97u8,98u8,67u8,100u8,120u8,121u8,122u8,74u8,73u8,23u8,198u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,97u8,98u8,99u8,100u8,237u8,130u8,205u8,17u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn h_container_unknown_ancillary_everywhere() -> crate::model::Ihdr {crate::model::Ihdr {width:2u32,height:2u32,depth:8u8,color_type:2u8,interlace:0u8}}
spec fn a_container_unknown_ancillary_everywhere() -> r::Raster {r::shape(h_container_unknown_ancillary_everywhere(),2nat,2nat,0nat,STRICT)}
spec fn colour_container_unknown_ancillary_everywhere() -> crate::colour_model::Context {crate::colour_model::Context {ct:2u8,depth:8u8,plte:seq![],trns:None}}
pub open spec fn expected_container_unknown_ancillary_everywhere() -> Seq<u8> {seq![201u8,73u8,63u8,255u8,202u8,75u8,151u8,255u8,225u8,26u8,23u8,255u8,252u8,246u8,129u8,255u8]}
spec fn z_container_unknown_ancillary_everywhere() -> Seq<u8> {seq![120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8]}
spec fn raw_container_unknown_ancillary_everywhere() -> Seq<u8> {seq![0u8,201u8,73u8,63u8,202u8,75u8,151u8,1u8,225u8,26u8,23u8,27u8,220u8,106u8]}
spec fn ts_container_unknown_ancillary_everywhere() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:36nat,before:1nat,after:2nat,symbol:201nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:36nat,next:44nat,before:2nat,after:3nat,symbol:73nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:44nat,next:52nat,before:3nat,after:4nat,symbol:63nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:52nat,next:61nat,before:4nat,after:5nat,symbol:202nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:61nat,next:69nat,before:5nat,after:6nat,symbol:75nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:69nat,next:78nat,before:6nat,after:7nat,symbol:151nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:78nat,next:86nat,before:7nat,after:8nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:86nat,next:95nat,before:8nat,after:9nat,symbol:225nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:95nat,next:103nat,before:9nat,after:10nat,symbol:26nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:103nat,next:111nat,before:10nat,after:11nat,symbol:23nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:111nat,next:119nat,before:11nat,after:12nat,symbol:27nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:119nat,next:128nat,before:12nat,after:13nat,symbol:220nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:128nat,next:136nat,before:13nat,after:14nat,symbol:106nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:136nat,next:143nat,before:14nat,after:14nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_container_unknown_ancillary_everywhere_0() ensures t::trace(z_container_unknown_ancillary_everywhere(),raw_container_unknown_ancillary_everywhere(),ts_container_unknown_ancillary_everywhere(),0nat,15nat),
{
    hide(t::valid_step);assert(t::trace(z_container_unknown_ancillary_everywhere(),raw_container_unknown_ancillary_everywhere(),ts_container_unknown_ancillary_everywhere(),0nat,15nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn trace_container_unknown_ancillary_everywhere() ensures t::trace(z_container_unknown_ancillary_everywhere(),raw_container_unknown_ancillary_everywhere(),ts_container_unknown_ancillary_everywhere(),0,15nat),
{
    hide(z_container_unknown_ancillary_everywhere);hide(raw_container_unknown_ancillary_everywhere);hide(ts_container_unknown_ancillary_everywhere);hide(t::trace);
    trace_container_unknown_ancillary_everywhere_0();
    assert(t::trace(z_container_unknown_ancillary_everywhere(),raw_container_unknown_ancillary_everywhere(),ts_container_unknown_ancillary_everywhere(),0,15nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_container_unknown_ancillary_everywhere() ensures z_container_unknown_ancillary_everywhere().len()==22nat,
        raw_container_unknown_ancillary_everywhere().len()==14nat,
        z_container_unknown_ancillary_everywhere()[0]==120u8,
        z_container_unknown_ancillary_everywhere()[1]==156u8,
        zm::field_value(z_container_unknown_ancillary_everywhere(),16,1)==1nat,
        zm::field_value(z_container_unknown_ancillary_everywhere(),17,2)==1nat,
        zm::be_value(z_container_unknown_ancillary_everywhere(),18nat,4)==675349874nat,
        ts_container_unknown_ancillary_everywhere().len()==15nat,
        ts_container_unknown_ancillary_everywhere()[0].pos==19nat,
        ts_container_unknown_ancillary_everywhere()[0].before==0nat,
        ts_container_unknown_ancillary_everywhere().last().next==143nat,
{
    assert(z_container_unknown_ancillary_everywhere().len()==22nat) by (compute);
    assert(raw_container_unknown_ancillary_everywhere().len()==14nat) by (compute);
    assert(z_container_unknown_ancillary_everywhere()[0]==120u8) by (compute);
    assert(z_container_unknown_ancillary_everywhere()[1]==156u8) by (compute);
    assert(zm::field_value(z_container_unknown_ancillary_everywhere(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_container_unknown_ancillary_everywhere(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_container_unknown_ancillary_everywhere(),18nat,4)==675349874nat) by (compute);
    assert(ts_container_unknown_ancillary_everywhere().len()==15nat) by (compute);
    assert(ts_container_unknown_ancillary_everywhere()[0].pos==19nat) by (compute);
    assert(ts_container_unknown_ancillary_everywhere()[0].before==0nat) by (compute);
    assert(ts_container_unknown_ancillary_everywhere().last().next==143nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_container_unknown_ancillary_everywhere() ensures zm::Adler(raw_container_unknown_ancillary_everywhere())==675349874nat,
{
    hide(raw_container_unknown_ancillary_everywhere);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_container_unknown_ancillary_everywhere().len()==14nat) by (compute);
    assert(c::sums(raw_container_unknown_ancillary_everywhere(),0,14nat)==(1393nat,10305nat)) by (compute);
    c::sums_correct(raw_container_unknown_ancillary_everywhere(),0,14nat);assert(raw_container_unknown_ancillary_everywhere().subrange(0,14)=~=raw_container_unknown_ancillary_everywhere());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((10305nat%65521)*(u16::MAX as nat+1)+(1+1393nat)%65521==675349874nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_container_unknown_ancillary_everywhere() ensures zm::Inflate(z_container_unknown_ancillary_everywhere(),14nat,true,false)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_container_unknown_ancillary_everywhere(),r2:false}),raw_container_unknown_ancillary_everywhere().len()==14nat,
{
    hide(z_container_unknown_ancillary_everywhere);hide(raw_container_unknown_ancillary_everywhere);hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);hide(ts_container_unknown_ancillary_everywhere);
    shape_container_unknown_ancillary_everywhere();checksum_container_unknown_ancillary_everywhere();
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_container_unknown_ancillary_everywhere();ms::fixed_stream(z_container_unknown_ancillary_everywhere(),raw_container_unknown_ancillary_everywhere(),ts_container_unknown_ancillary_everywhere(),14nat,156nat,675349874nat,675349874nat);
}
#[verifier::spinoff_prover]
proof fn walk_container_unknown_ancillary_everywhere(b: Seq<u8>)
    requires b.len()==106,
        crate::model::mh(b)==Ok((h_container_unknown_ancillary_everywhere(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,98u8,67u8,100u8],data:41nat,len:3nat,next:48nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,48nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:56nat,len:22nat,next:82nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,82nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,98u8,99u8,100u8],data:90nat,len:0nat,next:94nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,94nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:102nat,len:0nat,next:106nat,crc_ok:true,synthetic:false})),
        b.subrange(56,78)==z_container_unknown_ancillary_everywhere(),
    ensures r::default_case(b,h_container_unknown_ancillary_everywhere(),z_container_unknown_ancillary_everywhere(),0nat),match m::MW_p(b,m::strict()) {m::Outcome::Done(doc,_)=>doc.plte==None && doc.trns==None,_=>false},
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(m::store);hide(crate::metadata_model::parse);
    assert forall|doc: m::Doc,value: crate::metadata_model::Value,base: nat|
        (#[trigger] m::store(doc,value,base)).ihdr==doc.ihdr && m::store(doc,value,base).plte==doc.plte
        && m::store(doc,value,base).trns==doc.trns && m::store(doc,value,base).actl==doc.actl
        && m::store(doc,value,base).idat==doc.idat && m::store(doc,value,base).frames==doc.frames by {
        stored_structure(doc,value,base);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,98u8,67u8,100u8])==crate::walk_model::Name::Unknown(true)) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,98u8,67u8,100u8])==crate::walk_model::Name::Unknown(true)) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,98u8,99u8,100u8])==crate::walk_model::Name::Unknown(true)) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,98u8,99u8,100u8])==crate::walk_model::Name::Unknown(true)) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    reveal_with_fuel(m::drive,5);
    reveal_with_fuel(m::concat_ranges,3);
}
#[verifier::spinoff_prover]
proof fn scan_container_unknown_ancillary_everywhere() ensures r::scan(raw_container_unknown_ancillary_everywhere(),a_container_unknown_ancillary_everywhere(),0)==Ok(()),
{
    hide(raw_container_unknown_ancillary_everywhere);assert(raw_container_unknown_ancillary_everywhere().len()==14nat) by (compute);assert(r::count(a_container_unknown_ancillary_everywhere())==1nat) by (compute);
    assert(r::pw(a_container_unknown_ancillary_everywhere(),0nat)==2nat && r::ph(a_container_unknown_ancillary_everywhere(),0nat)==2nat && r::rb(a_container_unknown_ancillary_everywhere(),0nat)==6nat && r::base(a_container_unknown_ancillary_everywhere(),0nat)==0nat) by (compute);
    assert(raw_container_unknown_ancillary_everywhere()[0]==0u8) by (compute);
    assert(r::row_result(raw_container_unknown_ancillary_everywhere(),a_container_unknown_ancillary_everywhere(),0nat,0nat)==Ok(()));
    assert(raw_container_unknown_ancillary_everywhere()[7]==1u8) by (compute);
    assert(r::row_result(raw_container_unknown_ancillary_everywhere(),a_container_unknown_ancillary_everywhere(),0nat,1nat)==Ok(()));
    r::scan_step(raw_container_unknown_ancillary_everywhere(),a_container_unknown_ancillary_everywhere(),1nat);
    r::pass_step(raw_container_unknown_ancillary_everywhere(),a_container_unknown_ancillary_everywhere(),0nat,2nat);
    r::pass_step(raw_container_unknown_ancillary_everywhere(),a_container_unknown_ancillary_everywhere(),0nat,1nat);
    r::pass_step(raw_container_unknown_ancillary_everywhere(),a_container_unknown_ancillary_everywhere(),0nat,0nat);
    r::scan_step(raw_container_unknown_ancillary_everywhere(),a_container_unknown_ancillary_everywhere(),0nat);
}
#[verifier::spinoff_prover]
proof fn row_container_unknown_ancillary_everywhere_0_0() ensures r::line(raw_container_unknown_ancillary_everywhere(),a_container_unknown_ancillary_everywhere(),0nat,0nat)==seq![201u8,73u8,63u8,202u8,75u8,151u8],
{
    hide(raw_container_unknown_ancillary_everywhere);
    assert(raw_container_unknown_ancillary_everywhere().len()==14nat) by (compute);
    assert(raw_container_unknown_ancillary_everywhere()[0]==0u8) by (compute);
    assert(r::base(a_container_unknown_ancillary_everywhere(),0nat)==0nat && r::rb(a_container_unknown_ancillary_everywhere(),0nat)==6nat && f::bpp(r::bits(a_container_unknown_ancillary_everywhere()))==3nat) by (compute);
    assert(f::row_data(raw_container_unknown_ancillary_everywhere(),0nat,6nat,0nat)==seq![201u8,73u8,63u8,202u8,75u8,151u8]) by (compute);
    f::pass_row_equation(raw_container_unknown_ancillary_everywhere(),0nat,6nat,3nat,0nat);
    assert(f::previous_row(raw_container_unknown_ancillary_everywhere(),0nat,6nat,3nat,0nat)==None);
    assert(f::encoded_row(0u8,f::row_data(raw_container_unknown_ancillary_everywhere(),0nat,6nat,0nat),None,3nat,seq![201u8,73u8,63u8,202u8,75u8,151u8],0,6nat)) by (compute);
    f::checked_row_hint(0u8,f::row_data(raw_container_unknown_ancillary_everywhere(),0nat,6nat,0nat),None,3nat,seq![201u8,73u8,63u8,202u8,75u8,151u8]);
}
#[verifier::spinoff_prover]
proof fn row_container_unknown_ancillary_everywhere_0_1() ensures r::line(raw_container_unknown_ancillary_everywhere(),a_container_unknown_ancillary_everywhere(),0nat,1nat)==seq![225u8,26u8,23u8,252u8,246u8,129u8],
{
    hide(raw_container_unknown_ancillary_everywhere);
    assert(raw_container_unknown_ancillary_everywhere().len()==14nat) by (compute);
    assert(raw_container_unknown_ancillary_everywhere()[7]==1u8) by (compute);
    assert(r::base(a_container_unknown_ancillary_everywhere(),0nat)==0nat && r::rb(a_container_unknown_ancillary_everywhere(),0nat)==6nat && f::bpp(r::bits(a_container_unknown_ancillary_everywhere()))==3nat) by (compute);
    assert(f::row_data(raw_container_unknown_ancillary_everywhere(),0nat,6nat,1nat)==seq![225u8,26u8,23u8,27u8,220u8,106u8]) by (compute);
    row_container_unknown_ancillary_everywhere_0_0();
    f::pass_row_equation(raw_container_unknown_ancillary_everywhere(),0nat,6nat,3nat,1nat);
    assert(f::previous_row(raw_container_unknown_ancillary_everywhere(),0nat,6nat,3nat,1nat)==Some(seq![201u8,73u8,63u8,202u8,75u8,151u8]));
    assert(f::encoded_row(1u8,f::row_data(raw_container_unknown_ancillary_everywhere(),0nat,6nat,1nat),Some(seq![201u8,73u8,63u8,202u8,75u8,151u8]),3nat,seq![225u8,26u8,23u8,252u8,246u8,129u8],0,6nat)) by (compute);
    f::checked_row_hint(1u8,f::row_data(raw_container_unknown_ancillary_everywhere(),0nat,6nat,1nat),Some(seq![201u8,73u8,63u8,202u8,75u8,151u8]),3nat,seq![225u8,26u8,23u8,252u8,246u8,129u8]);
}
#[verifier::spinoff_prover]
proof fn raster_container_unknown_ancillary_everywhere()
    ensures r::default_case(bytes_container_unknown_ancillary_everywhere(),h_container_unknown_ancillary_everywhere(),z_container_unknown_ancillary_everywhere(),0nat),match m::MW_p(bytes_container_unknown_ancillary_everywhere(),m::strict()) {m::Outcome::Done(doc,_)=>doc.plte==None && doc.trns==None,_=>false},r::data(z_container_unknown_ancillary_everywhere(),a_container_unknown_ancillary_everywhere(),STRICT)==Some(Ok(())),
{
    hide(bytes_container_unknown_ancillary_everywhere);hide(z_container_unknown_ancillary_everywhere);hide(raw_container_unknown_ancillary_everywhere);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    assert(bytes_container_unknown_ancillary_everywhere().len()==106nat) by (compute);
    assert(crate::model::mh(bytes_container_unknown_ancillary_everywhere())==Ok((h_container_unknown_ancillary_everywhere(),33nat))) by {reveal(crate::model::mh);assert(crate::model::mh(bytes_container_unknown_ancillary_everywhere())==Ok((h_container_unknown_ancillary_everywhere(),33nat))) by (compute);}
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_unknown_ancillary_everywhere(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,98u8,67u8,100u8],data:41nat,len:3nat,next:48nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_unknown_ancillary_everywhere(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,98u8,67u8,100u8],data:41nat,len:3nat,next:48nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,98u8,67u8,100u8])==crate::walk_model::Name::Unknown(true)) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,98u8,67u8,100u8])==crate::walk_model::Name::Unknown(true)) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_unknown_ancillary_everywhere(),48nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:56nat,len:22nat,next:82nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_unknown_ancillary_everywhere(),48nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:56nat,len:22nat,next:82nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_unknown_ancillary_everywhere(),82nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,98u8,99u8,100u8],data:90nat,len:0nat,next:94nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_unknown_ancillary_everywhere(),82nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,98u8,99u8,100u8],data:90nat,len:0nat,next:94nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,98u8,99u8,100u8])==crate::walk_model::Name::Unknown(true)) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,98u8,99u8,100u8])==crate::walk_model::Name::Unknown(true)) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_unknown_ancillary_everywhere(),94nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:102nat,len:0nat,next:106nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_unknown_ancillary_everywhere(),94nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:102nat,len:0nat,next:106nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_container_unknown_ancillary_everywhere().subrange(56,78)==z_container_unknown_ancillary_everywhere()) by (compute);
    walk_container_unknown_ancillary_everywhere(bytes_container_unknown_ancillary_everywhere());
    inflate_container_unknown_ancillary_everywhere();scan_container_unknown_ancillary_everywhere();
    assert(r::raw(a_container_unknown_ancillary_everywhere())==14nat) by (compute);
    assert(r::data(z_container_unknown_ancillary_everywhere(),a_container_unknown_ancillary_everywhere(),STRICT)==Some(Ok(()))) by {reveal(r::data);};
}
#[verifier::spinoff_prover]
proof fn pixel_container_unknown_ancillary_everywhere_0() ensures sm::image(raw_container_unknown_ancillary_everywhere(),a_container_unknown_ancillary_everywhere(),colour_container_unknown_ancillary_everywhere(),false)[0]==201u8,sm::image(raw_container_unknown_ancillary_everywhere(),a_container_unknown_ancillary_everywhere(),colour_container_unknown_ancillary_everywhere(),false)[1]==73u8,sm::image(raw_container_unknown_ancillary_everywhere(),a_container_unknown_ancillary_everywhere(),colour_container_unknown_ancillary_everywhere(),false)[2]==63u8,sm::image(raw_container_unknown_ancillary_everywhere(),a_container_unknown_ancillary_everywhere(),colour_container_unknown_ancillary_everywhere(),false)[3]==255u8,
{
    colour::depth_values(8u8);
    row_container_unknown_ancillary_everywhere_0_0();
    assert(crate::paint_model::source_cell(a_container_unknown_ancillary_everywhere(),0nat,0nat)==(0nat,0nat,0nat)) by (compute);
    assert(colour::pixel(colour_container_unknown_ancillary_everywhere(),seq![201u8,73u8,63u8,202u8,75u8,151u8],0nat,true,false)==Ok(seq![51657u16,18761u16,16191u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_container_unknown_ancillary_everywhere(),a_container_unknown_ancillary_everywhere(),colour_container_unknown_ancillary_everywhere(),0nat,0nat,true,false)==Ok(seq![51657u16,18761u16,16191u16,65535u16]));
    assert(((seq![51657u16,18761u16,16191u16,65535u16][0] as nat+128)/257) as u8==201u8) by (compute);
    assert(((seq![51657u16,18761u16,16191u16,65535u16][1] as nat+128)/257) as u8==73u8) by (compute);
    assert(((seq![51657u16,18761u16,16191u16,65535u16][2] as nat+128)/257) as u8==63u8) by (compute);
    assert(((seq![51657u16,18761u16,16191u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixel_container_unknown_ancillary_everywhere_1() ensures sm::image(raw_container_unknown_ancillary_everywhere(),a_container_unknown_ancillary_everywhere(),colour_container_unknown_ancillary_everywhere(),false)[4]==202u8,sm::image(raw_container_unknown_ancillary_everywhere(),a_container_unknown_ancillary_everywhere(),colour_container_unknown_ancillary_everywhere(),false)[5]==75u8,sm::image(raw_container_unknown_ancillary_everywhere(),a_container_unknown_ancillary_everywhere(),colour_container_unknown_ancillary_everywhere(),false)[6]==151u8,sm::image(raw_container_unknown_ancillary_everywhere(),a_container_unknown_ancillary_everywhere(),colour_container_unknown_ancillary_everywhere(),false)[7]==255u8,
{
    colour::depth_values(8u8);
    row_container_unknown_ancillary_everywhere_0_0();
    assert(crate::paint_model::source_cell(a_container_unknown_ancillary_everywhere(),1nat,0nat)==(0nat,1nat,0nat)) by (compute);
    assert(colour::pixel(colour_container_unknown_ancillary_everywhere(),seq![201u8,73u8,63u8,202u8,75u8,151u8],1nat,true,false)==Ok(seq![51914u16,19275u16,38807u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_container_unknown_ancillary_everywhere(),a_container_unknown_ancillary_everywhere(),colour_container_unknown_ancillary_everywhere(),1nat,0nat,true,false)==Ok(seq![51914u16,19275u16,38807u16,65535u16]));
    assert(((seq![51914u16,19275u16,38807u16,65535u16][0] as nat+128)/257) as u8==202u8) by (compute);
    assert(((seq![51914u16,19275u16,38807u16,65535u16][1] as nat+128)/257) as u8==75u8) by (compute);
    assert(((seq![51914u16,19275u16,38807u16,65535u16][2] as nat+128)/257) as u8==151u8) by (compute);
    assert(((seq![51914u16,19275u16,38807u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixel_container_unknown_ancillary_everywhere_2() ensures sm::image(raw_container_unknown_ancillary_everywhere(),a_container_unknown_ancillary_everywhere(),colour_container_unknown_ancillary_everywhere(),false)[8]==225u8,sm::image(raw_container_unknown_ancillary_everywhere(),a_container_unknown_ancillary_everywhere(),colour_container_unknown_ancillary_everywhere(),false)[9]==26u8,sm::image(raw_container_unknown_ancillary_everywhere(),a_container_unknown_ancillary_everywhere(),colour_container_unknown_ancillary_everywhere(),false)[10]==23u8,sm::image(raw_container_unknown_ancillary_everywhere(),a_container_unknown_ancillary_everywhere(),colour_container_unknown_ancillary_everywhere(),false)[11]==255u8,
{
    colour::depth_values(8u8);
    row_container_unknown_ancillary_everywhere_0_1();
    assert(crate::paint_model::source_cell(a_container_unknown_ancillary_everywhere(),0nat,1nat)==(0nat,0nat,1nat)) by (compute);
    assert(colour::pixel(colour_container_unknown_ancillary_everywhere(),seq![225u8,26u8,23u8,252u8,246u8,129u8],0nat,true,false)==Ok(seq![57825u16,6682u16,5911u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_container_unknown_ancillary_everywhere(),a_container_unknown_ancillary_everywhere(),colour_container_unknown_ancillary_everywhere(),0nat,1nat,true,false)==Ok(seq![57825u16,6682u16,5911u16,65535u16]));
    assert(((seq![57825u16,6682u16,5911u16,65535u16][0] as nat+128)/257) as u8==225u8) by (compute);
    assert(((seq![57825u16,6682u16,5911u16,65535u16][1] as nat+128)/257) as u8==26u8) by (compute);
    assert(((seq![57825u16,6682u16,5911u16,65535u16][2] as nat+128)/257) as u8==23u8) by (compute);
    assert(((seq![57825u16,6682u16,5911u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixel_container_unknown_ancillary_everywhere_3() ensures sm::image(raw_container_unknown_ancillary_everywhere(),a_container_unknown_ancillary_everywhere(),colour_container_unknown_ancillary_everywhere(),false)[12]==252u8,sm::image(raw_container_unknown_ancillary_everywhere(),a_container_unknown_ancillary_everywhere(),colour_container_unknown_ancillary_everywhere(),false)[13]==246u8,sm::image(raw_container_unknown_ancillary_everywhere(),a_container_unknown_ancillary_everywhere(),colour_container_unknown_ancillary_everywhere(),false)[14]==129u8,sm::image(raw_container_unknown_ancillary_everywhere(),a_container_unknown_ancillary_everywhere(),colour_container_unknown_ancillary_everywhere(),false)[15]==255u8,
{
    colour::depth_values(8u8);
    row_container_unknown_ancillary_everywhere_0_1();
    assert(crate::paint_model::source_cell(a_container_unknown_ancillary_everywhere(),1nat,1nat)==(0nat,1nat,1nat)) by (compute);
    assert(colour::pixel(colour_container_unknown_ancillary_everywhere(),seq![225u8,26u8,23u8,252u8,246u8,129u8],1nat,true,false)==Ok(seq![64764u16,63222u16,33153u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_container_unknown_ancillary_everywhere(),a_container_unknown_ancillary_everywhere(),colour_container_unknown_ancillary_everywhere(),1nat,1nat,true,false)==Ok(seq![64764u16,63222u16,33153u16,65535u16]));
    assert(((seq![64764u16,63222u16,33153u16,65535u16][0] as nat+128)/257) as u8==252u8) by (compute);
    assert(((seq![64764u16,63222u16,33153u16,65535u16][1] as nat+128)/257) as u8==246u8) by (compute);
    assert(((seq![64764u16,63222u16,33153u16,65535u16][2] as nat+128)/257) as u8==129u8) by (compute);
    assert(((seq![64764u16,63222u16,33153u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixels_container_unknown_ancillary_everywhere() ensures sm::image(raw_container_unknown_ancillary_everywhere(),a_container_unknown_ancillary_everywhere(),colour_container_unknown_ancillary_everywhere(),false)==expected_container_unknown_ancillary_everywhere(),
{
    pixel_container_unknown_ancillary_everywhere_0();
    pixel_container_unknown_ancillary_everywhere_1();
    pixel_container_unknown_ancillary_everywhere_2();
    pixel_container_unknown_ancillary_everywhere_3();
    assert(a_container_unknown_ancillary_everywhere().width==2nat && a_container_unknown_ancillary_everywhere().height==2nat) by (compute);
    assert(sm::image(raw_container_unknown_ancillary_everywhere(),a_container_unknown_ancillary_everywhere(),colour_container_unknown_ancillary_everywhere(),false).len()==16nat);
    assert(expected_container_unknown_ancillary_everywhere().len()==16nat) by (compute);
    assert forall|j: int| 0<=j<16 implies (#[trigger] sm::image(raw_container_unknown_ancillary_everywhere(),a_container_unknown_ancillary_everywhere(),colour_container_unknown_ancillary_everywhere(),false)[j])==expected_container_unknown_ancillary_everywhere()[j] by {
        if j==0 {assert(expected_container_unknown_ancillary_everywhere()[0]==201u8) by (compute);}
        else if j==1 {assert(expected_container_unknown_ancillary_everywhere()[1]==73u8) by (compute);}
        else if j==2 {assert(expected_container_unknown_ancillary_everywhere()[2]==63u8) by (compute);}
        else if j==3 {assert(expected_container_unknown_ancillary_everywhere()[3]==255u8) by (compute);}
        else if j==4 {assert(expected_container_unknown_ancillary_everywhere()[4]==202u8) by (compute);}
        else if j==5 {assert(expected_container_unknown_ancillary_everywhere()[5]==75u8) by (compute);}
        else if j==6 {assert(expected_container_unknown_ancillary_everywhere()[6]==151u8) by (compute);}
        else if j==7 {assert(expected_container_unknown_ancillary_everywhere()[7]==255u8) by (compute);}
        else if j==8 {assert(expected_container_unknown_ancillary_everywhere()[8]==225u8) by (compute);}
        else if j==9 {assert(expected_container_unknown_ancillary_everywhere()[9]==26u8) by (compute);}
        else if j==10 {assert(expected_container_unknown_ancillary_everywhere()[10]==23u8) by (compute);}
        else if j==11 {assert(expected_container_unknown_ancillary_everywhere()[11]==255u8) by (compute);}
        else if j==12 {assert(expected_container_unknown_ancillary_everywhere()[12]==252u8) by (compute);}
        else if j==13 {assert(expected_container_unknown_ancillary_everywhere()[13]==246u8) by (compute);}
        else if j==14 {assert(expected_container_unknown_ancillary_everywhere()[14]==129u8) by (compute);}
        else {assert(j==15);assert(expected_container_unknown_ancillary_everywhere()[j]==255u8) by (compute);}
    }
    assert(sm::image(raw_container_unknown_ancillary_everywhere(),a_container_unknown_ancillary_everywhere(),colour_container_unknown_ancillary_everywhere(),false)=~=expected_container_unknown_ancillary_everywhere());
}
pub open spec fn expected_value_container_unknown_ancillary_everywhere(value: sm::Value) -> bool {value.width==2u32 && value.height==2u32 && value.pixels==expected_container_unknown_ancillary_everywhere()}
#[verifier::spinoff_prover]
pub proof fn model_complete_container_unknown_ancillary_everywhere()
    ensures match sm::MS(bytes_container_unknown_ancillary_everywhere()) {sm::Outcome::Done(value)=>expected_value_container_unknown_ancillary_everywhere(value),_=>false},
{
    hide(m::MW_p);hide(zm::Inflate);hide(bytes_container_unknown_ancillary_everywhere);hide(z_container_unknown_ancillary_everywhere);hide(raw_container_unknown_ancillary_everywhere);hide(expected_container_unknown_ancillary_everywhere);
    assert(bytes_container_unknown_ancillary_everywhere().len()==106nat) by (compute);
    raster_container_unknown_ancillary_everywhere();inflate_container_unknown_ancillary_everywhere();pixels_container_unknown_ancillary_everywhere();
    match m::MW_p(bytes_container_unknown_ancillary_everywhere(),m::strict()) {m::Outcome::Done(doc,_)=>{
        assert(colour::from_doc(doc,bytes_container_unknown_ancillary_everywhere())==colour_container_unknown_ancillary_everywhere());
        assert(crate::paint_model::default_raster(doc,STRICT)==a_container_unknown_ancillary_everywhere());
    },_=>{}}
    assert(r::raw(a_container_unknown_ancillary_everywhere())==14nat) by (compute);
    assert(r::inflated(z_container_unknown_ancillary_everywhere(),a_container_unknown_ancillary_everywhere(),STRICT).out==raw_container_unknown_ancillary_everywhere());
    compose_complete(bytes_container_unknown_ancillary_everywhere(),h_container_unknown_ancillary_everywhere(),z_container_unknown_ancillary_everywhere(),0nat,colour_container_unknown_ancillary_everywhere(),expected_container_unknown_ancillary_everywhere());
}
pub fn public_complete_container_unknown_ancillary_everywhere() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_container_unknown_ancillary_everywhere(),false),result),se::eq(bytes_container_unknown_ancillary_everywhere(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_container_unknown_ancillary_everywhere(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,3u8,97u8,98u8,67u8,100u8,120u8,121u8,122u8,74u8,73u8,23u8,198u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,97u8,98u8,99u8,100u8,237u8,130u8,205u8,17u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_container_unknown_ancillary_everywhere());model_complete_container_unknown_ancillary_everywhere();}
    crate::png::try_decode_still(&b)
}
// container/reserved-bit-ancillary; frozen fixture SHA256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
pub open spec fn bytes_container_reserved_bit_ancillary() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,1u8,122u8,122u8,122u8,122u8,49u8,172u8,170u8,51u8,75u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn h_container_reserved_bit_ancillary() -> crate::model::Ihdr {crate::model::Ihdr {width:2u32,height:2u32,depth:8u8,color_type:2u8,interlace:0u8}}
spec fn a_container_reserved_bit_ancillary() -> r::Raster {r::shape(h_container_reserved_bit_ancillary(),2nat,2nat,0nat,STRICT)}
spec fn colour_container_reserved_bit_ancillary() -> crate::colour_model::Context {crate::colour_model::Context {ct:2u8,depth:8u8,plte:seq![],trns:None}}
pub open spec fn expected_container_reserved_bit_ancillary() -> Seq<u8> {seq![201u8,73u8,63u8,255u8,202u8,75u8,151u8,255u8,225u8,26u8,23u8,255u8,252u8,246u8,129u8,255u8]}
spec fn z_container_reserved_bit_ancillary() -> Seq<u8> {seq![120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8]}
spec fn raw_container_reserved_bit_ancillary() -> Seq<u8> {seq![0u8,201u8,73u8,63u8,202u8,75u8,151u8,1u8,225u8,26u8,23u8,27u8,220u8,106u8]}
spec fn ts_container_reserved_bit_ancillary() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:36nat,before:1nat,after:2nat,symbol:201nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:36nat,next:44nat,before:2nat,after:3nat,symbol:73nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:44nat,next:52nat,before:3nat,after:4nat,symbol:63nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:52nat,next:61nat,before:4nat,after:5nat,symbol:202nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:61nat,next:69nat,before:5nat,after:6nat,symbol:75nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:69nat,next:78nat,before:6nat,after:7nat,symbol:151nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:78nat,next:86nat,before:7nat,after:8nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:86nat,next:95nat,before:8nat,after:9nat,symbol:225nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:95nat,next:103nat,before:9nat,after:10nat,symbol:26nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:103nat,next:111nat,before:10nat,after:11nat,symbol:23nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:111nat,next:119nat,before:11nat,after:12nat,symbol:27nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:119nat,next:128nat,before:12nat,after:13nat,symbol:220nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:128nat,next:136nat,before:13nat,after:14nat,symbol:106nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:136nat,next:143nat,before:14nat,after:14nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_container_reserved_bit_ancillary_0() ensures t::trace(z_container_reserved_bit_ancillary(),raw_container_reserved_bit_ancillary(),ts_container_reserved_bit_ancillary(),0nat,15nat),
{
    hide(t::valid_step);assert(t::trace(z_container_reserved_bit_ancillary(),raw_container_reserved_bit_ancillary(),ts_container_reserved_bit_ancillary(),0nat,15nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn trace_container_reserved_bit_ancillary() ensures t::trace(z_container_reserved_bit_ancillary(),raw_container_reserved_bit_ancillary(),ts_container_reserved_bit_ancillary(),0,15nat),
{
    hide(z_container_reserved_bit_ancillary);hide(raw_container_reserved_bit_ancillary);hide(ts_container_reserved_bit_ancillary);hide(t::trace);
    trace_container_reserved_bit_ancillary_0();
    assert(t::trace(z_container_reserved_bit_ancillary(),raw_container_reserved_bit_ancillary(),ts_container_reserved_bit_ancillary(),0,15nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_container_reserved_bit_ancillary() ensures z_container_reserved_bit_ancillary().len()==22nat,
        raw_container_reserved_bit_ancillary().len()==14nat,
        z_container_reserved_bit_ancillary()[0]==120u8,
        z_container_reserved_bit_ancillary()[1]==156u8,
        zm::field_value(z_container_reserved_bit_ancillary(),16,1)==1nat,
        zm::field_value(z_container_reserved_bit_ancillary(),17,2)==1nat,
        zm::be_value(z_container_reserved_bit_ancillary(),18nat,4)==675349874nat,
        ts_container_reserved_bit_ancillary().len()==15nat,
        ts_container_reserved_bit_ancillary()[0].pos==19nat,
        ts_container_reserved_bit_ancillary()[0].before==0nat,
        ts_container_reserved_bit_ancillary().last().next==143nat,
{
    assert(z_container_reserved_bit_ancillary().len()==22nat) by (compute);
    assert(raw_container_reserved_bit_ancillary().len()==14nat) by (compute);
    assert(z_container_reserved_bit_ancillary()[0]==120u8) by (compute);
    assert(z_container_reserved_bit_ancillary()[1]==156u8) by (compute);
    assert(zm::field_value(z_container_reserved_bit_ancillary(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_container_reserved_bit_ancillary(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_container_reserved_bit_ancillary(),18nat,4)==675349874nat) by (compute);
    assert(ts_container_reserved_bit_ancillary().len()==15nat) by (compute);
    assert(ts_container_reserved_bit_ancillary()[0].pos==19nat) by (compute);
    assert(ts_container_reserved_bit_ancillary()[0].before==0nat) by (compute);
    assert(ts_container_reserved_bit_ancillary().last().next==143nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_container_reserved_bit_ancillary() ensures zm::Adler(raw_container_reserved_bit_ancillary())==675349874nat,
{
    hide(raw_container_reserved_bit_ancillary);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_container_reserved_bit_ancillary().len()==14nat) by (compute);
    assert(c::sums(raw_container_reserved_bit_ancillary(),0,14nat)==(1393nat,10305nat)) by (compute);
    c::sums_correct(raw_container_reserved_bit_ancillary(),0,14nat);assert(raw_container_reserved_bit_ancillary().subrange(0,14)=~=raw_container_reserved_bit_ancillary());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((10305nat%65521)*(u16::MAX as nat+1)+(1+1393nat)%65521==675349874nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_container_reserved_bit_ancillary() ensures zm::Inflate(z_container_reserved_bit_ancillary(),14nat,true,false)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_container_reserved_bit_ancillary(),r2:false}),raw_container_reserved_bit_ancillary().len()==14nat,
{
    hide(z_container_reserved_bit_ancillary);hide(raw_container_reserved_bit_ancillary);hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);hide(ts_container_reserved_bit_ancillary);
    shape_container_reserved_bit_ancillary();checksum_container_reserved_bit_ancillary();
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_container_reserved_bit_ancillary();ms::fixed_stream(z_container_reserved_bit_ancillary(),raw_container_reserved_bit_ancillary(),ts_container_reserved_bit_ancillary(),14nat,156nat,675349874nat,675349874nat);
}
#[verifier::spinoff_prover]
proof fn walk_container_reserved_bit_ancillary(b: Seq<u8>)
    requires b.len()==92,
        crate::model::mh(b)==Ok((h_container_reserved_bit_ancillary(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![122u8,122u8,122u8,122u8],data:41nat,len:1nat,next:46nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,46nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:54nat,len:22nat,next:80nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,80nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:88nat,len:0nat,next:92nat,crc_ok:true,synthetic:false})),
        b.subrange(54,76)==z_container_reserved_bit_ancillary(),
    ensures r::default_case(b,h_container_reserved_bit_ancillary(),z_container_reserved_bit_ancillary(),0nat),match m::MW_p(b,m::strict()) {m::Outcome::Done(doc,_)=>doc.plte==None && doc.trns==None,_=>false},
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(m::store);hide(crate::metadata_model::parse);
    assert forall|doc: m::Doc,value: crate::metadata_model::Value,base: nat|
        (#[trigger] m::store(doc,value,base)).ihdr==doc.ihdr && m::store(doc,value,base).plte==doc.plte
        && m::store(doc,value,base).trns==doc.trns && m::store(doc,value,base).actl==doc.actl
        && m::store(doc,value,base).idat==doc.idat && m::store(doc,value,base).frames==doc.frames by {
        stored_structure(doc,value,base);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((122u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![122u8,122u8,122u8,122u8])==crate::walk_model::Name::Unknown(true)) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![122u8,122u8,122u8,122u8])==crate::walk_model::Name::Unknown(true)) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    reveal_with_fuel(m::drive,4);
    reveal_with_fuel(m::concat_ranges,3);
}
#[verifier::spinoff_prover]
proof fn scan_container_reserved_bit_ancillary() ensures r::scan(raw_container_reserved_bit_ancillary(),a_container_reserved_bit_ancillary(),0)==Ok(()),
{
    hide(raw_container_reserved_bit_ancillary);assert(raw_container_reserved_bit_ancillary().len()==14nat) by (compute);assert(r::count(a_container_reserved_bit_ancillary())==1nat) by (compute);
    assert(r::pw(a_container_reserved_bit_ancillary(),0nat)==2nat && r::ph(a_container_reserved_bit_ancillary(),0nat)==2nat && r::rb(a_container_reserved_bit_ancillary(),0nat)==6nat && r::base(a_container_reserved_bit_ancillary(),0nat)==0nat) by (compute);
    assert(raw_container_reserved_bit_ancillary()[0]==0u8) by (compute);
    assert(r::row_result(raw_container_reserved_bit_ancillary(),a_container_reserved_bit_ancillary(),0nat,0nat)==Ok(()));
    assert(raw_container_reserved_bit_ancillary()[7]==1u8) by (compute);
    assert(r::row_result(raw_container_reserved_bit_ancillary(),a_container_reserved_bit_ancillary(),0nat,1nat)==Ok(()));
    r::scan_step(raw_container_reserved_bit_ancillary(),a_container_reserved_bit_ancillary(),1nat);
    r::pass_step(raw_container_reserved_bit_ancillary(),a_container_reserved_bit_ancillary(),0nat,2nat);
    r::pass_step(raw_container_reserved_bit_ancillary(),a_container_reserved_bit_ancillary(),0nat,1nat);
    r::pass_step(raw_container_reserved_bit_ancillary(),a_container_reserved_bit_ancillary(),0nat,0nat);
    r::scan_step(raw_container_reserved_bit_ancillary(),a_container_reserved_bit_ancillary(),0nat);
}
#[verifier::spinoff_prover]
proof fn row_container_reserved_bit_ancillary_0_0() ensures r::line(raw_container_reserved_bit_ancillary(),a_container_reserved_bit_ancillary(),0nat,0nat)==seq![201u8,73u8,63u8,202u8,75u8,151u8],
{
    hide(raw_container_reserved_bit_ancillary);
    assert(raw_container_reserved_bit_ancillary().len()==14nat) by (compute);
    assert(raw_container_reserved_bit_ancillary()[0]==0u8) by (compute);
    assert(r::base(a_container_reserved_bit_ancillary(),0nat)==0nat && r::rb(a_container_reserved_bit_ancillary(),0nat)==6nat && f::bpp(r::bits(a_container_reserved_bit_ancillary()))==3nat) by (compute);
    assert(f::row_data(raw_container_reserved_bit_ancillary(),0nat,6nat,0nat)==seq![201u8,73u8,63u8,202u8,75u8,151u8]) by (compute);
    f::pass_row_equation(raw_container_reserved_bit_ancillary(),0nat,6nat,3nat,0nat);
    assert(f::previous_row(raw_container_reserved_bit_ancillary(),0nat,6nat,3nat,0nat)==None);
    assert(f::encoded_row(0u8,f::row_data(raw_container_reserved_bit_ancillary(),0nat,6nat,0nat),None,3nat,seq![201u8,73u8,63u8,202u8,75u8,151u8],0,6nat)) by (compute);
    f::checked_row_hint(0u8,f::row_data(raw_container_reserved_bit_ancillary(),0nat,6nat,0nat),None,3nat,seq![201u8,73u8,63u8,202u8,75u8,151u8]);
}
#[verifier::spinoff_prover]
proof fn row_container_reserved_bit_ancillary_0_1() ensures r::line(raw_container_reserved_bit_ancillary(),a_container_reserved_bit_ancillary(),0nat,1nat)==seq![225u8,26u8,23u8,252u8,246u8,129u8],
{
    hide(raw_container_reserved_bit_ancillary);
    assert(raw_container_reserved_bit_ancillary().len()==14nat) by (compute);
    assert(raw_container_reserved_bit_ancillary()[7]==1u8) by (compute);
    assert(r::base(a_container_reserved_bit_ancillary(),0nat)==0nat && r::rb(a_container_reserved_bit_ancillary(),0nat)==6nat && f::bpp(r::bits(a_container_reserved_bit_ancillary()))==3nat) by (compute);
    assert(f::row_data(raw_container_reserved_bit_ancillary(),0nat,6nat,1nat)==seq![225u8,26u8,23u8,27u8,220u8,106u8]) by (compute);
    row_container_reserved_bit_ancillary_0_0();
    f::pass_row_equation(raw_container_reserved_bit_ancillary(),0nat,6nat,3nat,1nat);
    assert(f::previous_row(raw_container_reserved_bit_ancillary(),0nat,6nat,3nat,1nat)==Some(seq![201u8,73u8,63u8,202u8,75u8,151u8]));
    assert(f::encoded_row(1u8,f::row_data(raw_container_reserved_bit_ancillary(),0nat,6nat,1nat),Some(seq![201u8,73u8,63u8,202u8,75u8,151u8]),3nat,seq![225u8,26u8,23u8,252u8,246u8,129u8],0,6nat)) by (compute);
    f::checked_row_hint(1u8,f::row_data(raw_container_reserved_bit_ancillary(),0nat,6nat,1nat),Some(seq![201u8,73u8,63u8,202u8,75u8,151u8]),3nat,seq![225u8,26u8,23u8,252u8,246u8,129u8]);
}
#[verifier::spinoff_prover]
proof fn raster_container_reserved_bit_ancillary()
    ensures r::default_case(bytes_container_reserved_bit_ancillary(),h_container_reserved_bit_ancillary(),z_container_reserved_bit_ancillary(),0nat),match m::MW_p(bytes_container_reserved_bit_ancillary(),m::strict()) {m::Outcome::Done(doc,_)=>doc.plte==None && doc.trns==None,_=>false},r::data(z_container_reserved_bit_ancillary(),a_container_reserved_bit_ancillary(),STRICT)==Some(Ok(())),
{
    hide(bytes_container_reserved_bit_ancillary);hide(z_container_reserved_bit_ancillary);hide(raw_container_reserved_bit_ancillary);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    assert(bytes_container_reserved_bit_ancillary().len()==92nat) by (compute);
    assert(crate::model::mh(bytes_container_reserved_bit_ancillary())==Ok((h_container_reserved_bit_ancillary(),33nat))) by {reveal(crate::model::mh);assert(crate::model::mh(bytes_container_reserved_bit_ancillary())==Ok((h_container_reserved_bit_ancillary(),33nat))) by (compute);}
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_reserved_bit_ancillary(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![122u8,122u8,122u8,122u8],data:41nat,len:1nat,next:46nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_reserved_bit_ancillary(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![122u8,122u8,122u8,122u8],data:41nat,len:1nat,next:46nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((122u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![122u8,122u8,122u8,122u8])==crate::walk_model::Name::Unknown(true)) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![122u8,122u8,122u8,122u8])==crate::walk_model::Name::Unknown(true)) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_reserved_bit_ancillary(),46nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:54nat,len:22nat,next:80nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_reserved_bit_ancillary(),46nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:54nat,len:22nat,next:80nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_container_reserved_bit_ancillary(),80nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:88nat,len:0nat,next:92nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_container_reserved_bit_ancillary(),80nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:88nat,len:0nat,next:92nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_container_reserved_bit_ancillary().subrange(54,76)==z_container_reserved_bit_ancillary()) by (compute);
    walk_container_reserved_bit_ancillary(bytes_container_reserved_bit_ancillary());
    inflate_container_reserved_bit_ancillary();scan_container_reserved_bit_ancillary();
    assert(r::raw(a_container_reserved_bit_ancillary())==14nat) by (compute);
    assert(r::data(z_container_reserved_bit_ancillary(),a_container_reserved_bit_ancillary(),STRICT)==Some(Ok(()))) by {reveal(r::data);};
}
#[verifier::spinoff_prover]
proof fn pixel_container_reserved_bit_ancillary_0() ensures sm::image(raw_container_reserved_bit_ancillary(),a_container_reserved_bit_ancillary(),colour_container_reserved_bit_ancillary(),false)[0]==201u8,sm::image(raw_container_reserved_bit_ancillary(),a_container_reserved_bit_ancillary(),colour_container_reserved_bit_ancillary(),false)[1]==73u8,sm::image(raw_container_reserved_bit_ancillary(),a_container_reserved_bit_ancillary(),colour_container_reserved_bit_ancillary(),false)[2]==63u8,sm::image(raw_container_reserved_bit_ancillary(),a_container_reserved_bit_ancillary(),colour_container_reserved_bit_ancillary(),false)[3]==255u8,
{
    colour::depth_values(8u8);
    row_container_reserved_bit_ancillary_0_0();
    assert(crate::paint_model::source_cell(a_container_reserved_bit_ancillary(),0nat,0nat)==(0nat,0nat,0nat)) by (compute);
    assert(colour::pixel(colour_container_reserved_bit_ancillary(),seq![201u8,73u8,63u8,202u8,75u8,151u8],0nat,true,false)==Ok(seq![51657u16,18761u16,16191u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_container_reserved_bit_ancillary(),a_container_reserved_bit_ancillary(),colour_container_reserved_bit_ancillary(),0nat,0nat,true,false)==Ok(seq![51657u16,18761u16,16191u16,65535u16]));
    assert(((seq![51657u16,18761u16,16191u16,65535u16][0] as nat+128)/257) as u8==201u8) by (compute);
    assert(((seq![51657u16,18761u16,16191u16,65535u16][1] as nat+128)/257) as u8==73u8) by (compute);
    assert(((seq![51657u16,18761u16,16191u16,65535u16][2] as nat+128)/257) as u8==63u8) by (compute);
    assert(((seq![51657u16,18761u16,16191u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixel_container_reserved_bit_ancillary_1() ensures sm::image(raw_container_reserved_bit_ancillary(),a_container_reserved_bit_ancillary(),colour_container_reserved_bit_ancillary(),false)[4]==202u8,sm::image(raw_container_reserved_bit_ancillary(),a_container_reserved_bit_ancillary(),colour_container_reserved_bit_ancillary(),false)[5]==75u8,sm::image(raw_container_reserved_bit_ancillary(),a_container_reserved_bit_ancillary(),colour_container_reserved_bit_ancillary(),false)[6]==151u8,sm::image(raw_container_reserved_bit_ancillary(),a_container_reserved_bit_ancillary(),colour_container_reserved_bit_ancillary(),false)[7]==255u8,
{
    colour::depth_values(8u8);
    row_container_reserved_bit_ancillary_0_0();
    assert(crate::paint_model::source_cell(a_container_reserved_bit_ancillary(),1nat,0nat)==(0nat,1nat,0nat)) by (compute);
    assert(colour::pixel(colour_container_reserved_bit_ancillary(),seq![201u8,73u8,63u8,202u8,75u8,151u8],1nat,true,false)==Ok(seq![51914u16,19275u16,38807u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_container_reserved_bit_ancillary(),a_container_reserved_bit_ancillary(),colour_container_reserved_bit_ancillary(),1nat,0nat,true,false)==Ok(seq![51914u16,19275u16,38807u16,65535u16]));
    assert(((seq![51914u16,19275u16,38807u16,65535u16][0] as nat+128)/257) as u8==202u8) by (compute);
    assert(((seq![51914u16,19275u16,38807u16,65535u16][1] as nat+128)/257) as u8==75u8) by (compute);
    assert(((seq![51914u16,19275u16,38807u16,65535u16][2] as nat+128)/257) as u8==151u8) by (compute);
    assert(((seq![51914u16,19275u16,38807u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixel_container_reserved_bit_ancillary_2() ensures sm::image(raw_container_reserved_bit_ancillary(),a_container_reserved_bit_ancillary(),colour_container_reserved_bit_ancillary(),false)[8]==225u8,sm::image(raw_container_reserved_bit_ancillary(),a_container_reserved_bit_ancillary(),colour_container_reserved_bit_ancillary(),false)[9]==26u8,sm::image(raw_container_reserved_bit_ancillary(),a_container_reserved_bit_ancillary(),colour_container_reserved_bit_ancillary(),false)[10]==23u8,sm::image(raw_container_reserved_bit_ancillary(),a_container_reserved_bit_ancillary(),colour_container_reserved_bit_ancillary(),false)[11]==255u8,
{
    colour::depth_values(8u8);
    row_container_reserved_bit_ancillary_0_1();
    assert(crate::paint_model::source_cell(a_container_reserved_bit_ancillary(),0nat,1nat)==(0nat,0nat,1nat)) by (compute);
    assert(colour::pixel(colour_container_reserved_bit_ancillary(),seq![225u8,26u8,23u8,252u8,246u8,129u8],0nat,true,false)==Ok(seq![57825u16,6682u16,5911u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_container_reserved_bit_ancillary(),a_container_reserved_bit_ancillary(),colour_container_reserved_bit_ancillary(),0nat,1nat,true,false)==Ok(seq![57825u16,6682u16,5911u16,65535u16]));
    assert(((seq![57825u16,6682u16,5911u16,65535u16][0] as nat+128)/257) as u8==225u8) by (compute);
    assert(((seq![57825u16,6682u16,5911u16,65535u16][1] as nat+128)/257) as u8==26u8) by (compute);
    assert(((seq![57825u16,6682u16,5911u16,65535u16][2] as nat+128)/257) as u8==23u8) by (compute);
    assert(((seq![57825u16,6682u16,5911u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixel_container_reserved_bit_ancillary_3() ensures sm::image(raw_container_reserved_bit_ancillary(),a_container_reserved_bit_ancillary(),colour_container_reserved_bit_ancillary(),false)[12]==252u8,sm::image(raw_container_reserved_bit_ancillary(),a_container_reserved_bit_ancillary(),colour_container_reserved_bit_ancillary(),false)[13]==246u8,sm::image(raw_container_reserved_bit_ancillary(),a_container_reserved_bit_ancillary(),colour_container_reserved_bit_ancillary(),false)[14]==129u8,sm::image(raw_container_reserved_bit_ancillary(),a_container_reserved_bit_ancillary(),colour_container_reserved_bit_ancillary(),false)[15]==255u8,
{
    colour::depth_values(8u8);
    row_container_reserved_bit_ancillary_0_1();
    assert(crate::paint_model::source_cell(a_container_reserved_bit_ancillary(),1nat,1nat)==(0nat,1nat,1nat)) by (compute);
    assert(colour::pixel(colour_container_reserved_bit_ancillary(),seq![225u8,26u8,23u8,252u8,246u8,129u8],1nat,true,false)==Ok(seq![64764u16,63222u16,33153u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_container_reserved_bit_ancillary(),a_container_reserved_bit_ancillary(),colour_container_reserved_bit_ancillary(),1nat,1nat,true,false)==Ok(seq![64764u16,63222u16,33153u16,65535u16]));
    assert(((seq![64764u16,63222u16,33153u16,65535u16][0] as nat+128)/257) as u8==252u8) by (compute);
    assert(((seq![64764u16,63222u16,33153u16,65535u16][1] as nat+128)/257) as u8==246u8) by (compute);
    assert(((seq![64764u16,63222u16,33153u16,65535u16][2] as nat+128)/257) as u8==129u8) by (compute);
    assert(((seq![64764u16,63222u16,33153u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixels_container_reserved_bit_ancillary() ensures sm::image(raw_container_reserved_bit_ancillary(),a_container_reserved_bit_ancillary(),colour_container_reserved_bit_ancillary(),false)==expected_container_reserved_bit_ancillary(),
{
    pixel_container_reserved_bit_ancillary_0();
    pixel_container_reserved_bit_ancillary_1();
    pixel_container_reserved_bit_ancillary_2();
    pixel_container_reserved_bit_ancillary_3();
    assert(a_container_reserved_bit_ancillary().width==2nat && a_container_reserved_bit_ancillary().height==2nat) by (compute);
    assert(sm::image(raw_container_reserved_bit_ancillary(),a_container_reserved_bit_ancillary(),colour_container_reserved_bit_ancillary(),false).len()==16nat);
    assert(expected_container_reserved_bit_ancillary().len()==16nat) by (compute);
    assert forall|j: int| 0<=j<16 implies (#[trigger] sm::image(raw_container_reserved_bit_ancillary(),a_container_reserved_bit_ancillary(),colour_container_reserved_bit_ancillary(),false)[j])==expected_container_reserved_bit_ancillary()[j] by {
        if j==0 {assert(expected_container_reserved_bit_ancillary()[0]==201u8) by (compute);}
        else if j==1 {assert(expected_container_reserved_bit_ancillary()[1]==73u8) by (compute);}
        else if j==2 {assert(expected_container_reserved_bit_ancillary()[2]==63u8) by (compute);}
        else if j==3 {assert(expected_container_reserved_bit_ancillary()[3]==255u8) by (compute);}
        else if j==4 {assert(expected_container_reserved_bit_ancillary()[4]==202u8) by (compute);}
        else if j==5 {assert(expected_container_reserved_bit_ancillary()[5]==75u8) by (compute);}
        else if j==6 {assert(expected_container_reserved_bit_ancillary()[6]==151u8) by (compute);}
        else if j==7 {assert(expected_container_reserved_bit_ancillary()[7]==255u8) by (compute);}
        else if j==8 {assert(expected_container_reserved_bit_ancillary()[8]==225u8) by (compute);}
        else if j==9 {assert(expected_container_reserved_bit_ancillary()[9]==26u8) by (compute);}
        else if j==10 {assert(expected_container_reserved_bit_ancillary()[10]==23u8) by (compute);}
        else if j==11 {assert(expected_container_reserved_bit_ancillary()[11]==255u8) by (compute);}
        else if j==12 {assert(expected_container_reserved_bit_ancillary()[12]==252u8) by (compute);}
        else if j==13 {assert(expected_container_reserved_bit_ancillary()[13]==246u8) by (compute);}
        else if j==14 {assert(expected_container_reserved_bit_ancillary()[14]==129u8) by (compute);}
        else {assert(j==15);assert(expected_container_reserved_bit_ancillary()[j]==255u8) by (compute);}
    }
    assert(sm::image(raw_container_reserved_bit_ancillary(),a_container_reserved_bit_ancillary(),colour_container_reserved_bit_ancillary(),false)=~=expected_container_reserved_bit_ancillary());
}
pub open spec fn expected_value_container_reserved_bit_ancillary(value: sm::Value) -> bool {value.width==2u32 && value.height==2u32 && value.pixels==expected_container_reserved_bit_ancillary()}
#[verifier::spinoff_prover]
pub proof fn model_complete_container_reserved_bit_ancillary()
    ensures match sm::MS(bytes_container_reserved_bit_ancillary()) {sm::Outcome::Done(value)=>expected_value_container_reserved_bit_ancillary(value),_=>false},
{
    hide(m::MW_p);hide(zm::Inflate);hide(bytes_container_reserved_bit_ancillary);hide(z_container_reserved_bit_ancillary);hide(raw_container_reserved_bit_ancillary);hide(expected_container_reserved_bit_ancillary);
    assert(bytes_container_reserved_bit_ancillary().len()==92nat) by (compute);
    raster_container_reserved_bit_ancillary();inflate_container_reserved_bit_ancillary();pixels_container_reserved_bit_ancillary();
    match m::MW_p(bytes_container_reserved_bit_ancillary(),m::strict()) {m::Outcome::Done(doc,_)=>{
        assert(colour::from_doc(doc,bytes_container_reserved_bit_ancillary())==colour_container_reserved_bit_ancillary());
        assert(crate::paint_model::default_raster(doc,STRICT)==a_container_reserved_bit_ancillary());
    },_=>{}}
    assert(r::raw(a_container_reserved_bit_ancillary())==14nat) by (compute);
    assert(r::inflated(z_container_reserved_bit_ancillary(),a_container_reserved_bit_ancillary(),STRICT).out==raw_container_reserved_bit_ancillary());
    compose_complete(bytes_container_reserved_bit_ancillary(),h_container_reserved_bit_ancillary(),z_container_reserved_bit_ancillary(),0nat,colour_container_reserved_bit_ancillary(),expected_container_reserved_bit_ancillary());
}
pub fn public_complete_container_reserved_bit_ancillary() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_container_reserved_bit_ancillary(),false),result),se::eq(bytes_container_reserved_bit_ancillary(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_container_reserved_bit_ancillary(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,1u8,122u8,122u8,122u8,122u8,49u8,172u8,170u8,51u8,75u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_container_reserved_bit_ancillary());model_complete_container_reserved_bit_ancillary();}
    crate::png::try_decode_still(&b)
}
// plte/ok; frozen fixture SHA256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
pub open spec fn bytes_plte_ok() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,3u8,0u8,0u8,0u8,69u8,104u8,253u8,22u8,0u8,0u8,0u8,9u8,80u8,76u8,84u8,69u8,47u8,194u8,40u8,139u8,44u8,88u8,228u8,36u8,202u8,64u8,183u8,32u8,36u8,0u8,0u8,0u8,14u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,96u8,100u8,100u8,250u8,15u8,0u8,1u8,16u8,1u8,4u8,216u8,172u8,32u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn h_plte_ok() -> crate::model::Ihdr {crate::model::Ihdr {width:2u32,height:2u32,depth:8u8,color_type:3u8,interlace:0u8}}
spec fn a_plte_ok() -> r::Raster {r::shape(h_plte_ok(),2nat,2nat,3nat,STRICT)}
spec fn colour_plte_ok() -> crate::colour_model::Context {crate::colour_model::Context {ct:3u8,depth:8u8,plte:seq![47u8,194u8,40u8,139u8,44u8,88u8,228u8,36u8,202u8],trns:None}}
pub open spec fn expected_plte_ok() -> Seq<u8> {seq![47u8,194u8,40u8,255u8,139u8,44u8,88u8,255u8,228u8,36u8,202u8,255u8,139u8,44u8,88u8,255u8]}
spec fn z_plte_ok() -> Seq<u8> {seq![120u8,156u8,99u8,96u8,96u8,100u8,100u8,250u8,15u8,0u8,1u8,16u8,1u8,4u8]}
spec fn raw_plte_ok() -> Seq<u8> {seq![0u8,0u8,1u8,1u8,2u8,255u8]}
spec fn ts_plte_ok() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:35nat,before:1nat,after:2nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:35nat,next:43nat,before:2nat,after:3nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:43nat,next:51nat,before:3nat,after:4nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:51nat,next:59nat,before:4nat,after:5nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:59nat,next:68nat,before:5nat,after:6nat,symbol:255nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:68nat,next:75nat,before:6nat,after:6nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_plte_ok_0() ensures t::trace(z_plte_ok(),raw_plte_ok(),ts_plte_ok(),0nat,7nat),
{
    hide(t::valid_step);assert(t::trace(z_plte_ok(),raw_plte_ok(),ts_plte_ok(),0nat,7nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn trace_plte_ok() ensures t::trace(z_plte_ok(),raw_plte_ok(),ts_plte_ok(),0,7nat),
{
    hide(z_plte_ok);hide(raw_plte_ok);hide(ts_plte_ok);hide(t::trace);
    trace_plte_ok_0();
    assert(t::trace(z_plte_ok(),raw_plte_ok(),ts_plte_ok(),0,7nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_plte_ok() ensures z_plte_ok().len()==14nat,
        raw_plte_ok().len()==6nat,
        z_plte_ok()[0]==120u8,
        z_plte_ok()[1]==156u8,
        zm::field_value(z_plte_ok(),16,1)==1nat,
        zm::field_value(z_plte_ok(),17,2)==1nat,
        zm::be_value(z_plte_ok(),10nat,4)==17826052nat,
        ts_plte_ok().len()==7nat,
        ts_plte_ok()[0].pos==19nat,
        ts_plte_ok()[0].before==0nat,
        ts_plte_ok().last().next==75nat,
{
    assert(z_plte_ok().len()==14nat) by (compute);
    assert(raw_plte_ok().len()==6nat) by (compute);
    assert(z_plte_ok()[0]==120u8) by (compute);
    assert(z_plte_ok()[1]==156u8) by (compute);
    assert(zm::field_value(z_plte_ok(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_plte_ok(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_plte_ok(),10nat,4)==17826052nat) by (compute);
    assert(ts_plte_ok().len()==7nat) by (compute);
    assert(ts_plte_ok()[0].pos==19nat) by (compute);
    assert(ts_plte_ok()[0].before==0nat) by (compute);
    assert(ts_plte_ok().last().next==75nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_plte_ok() ensures zm::Adler(raw_plte_ok())==17826052nat,
{
    hide(raw_plte_ok);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_plte_ok().len()==6nat) by (compute);
    assert(c::sums(raw_plte_ok(),0,6nat)==(259nat,272nat)) by (compute);
    c::sums_correct(raw_plte_ok(),0,6nat);assert(raw_plte_ok().subrange(0,6)=~=raw_plte_ok());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((272nat%65521)*(u16::MAX as nat+1)+(1+259nat)%65521==17826052nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_plte_ok() ensures zm::Inflate(z_plte_ok(),6nat,true,false)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_plte_ok(),r2:false}),raw_plte_ok().len()==6nat,
{
    hide(z_plte_ok);hide(raw_plte_ok);hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);hide(ts_plte_ok);
    shape_plte_ok();checksum_plte_ok();
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_plte_ok();ms::fixed_stream(z_plte_ok(),raw_plte_ok(),ts_plte_ok(),6nat,156nat,17826052nat,17826052nat);
}
#[verifier::spinoff_prover]
proof fn walk_plte_ok(b: Seq<u8>)
    requires b.len()==92,
        crate::model::mh(b)==Ok((h_plte_ok(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:9nat,next:54nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,54nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:62nat,len:14nat,next:80nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,80nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:88nat,len:0nat,next:92nat,crc_ok:true,synthetic:false})),
        b.subrange(62,76)==z_plte_ok(),
    ensures r::default_case(b,h_plte_ok(),z_plte_ok(),3nat),match m::MW_p(b,m::strict()) {m::Outcome::Done(doc,_)=>doc.plte==Some(crate::walk_model::Range {offset:41nat,len:9nat}) && doc.trns==None,_=>false},
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(m::store);hide(crate::metadata_model::parse);
    assert forall|doc: m::Doc,value: crate::metadata_model::Value,base: nat|
        (#[trigger] m::store(doc,value,base)).ihdr==doc.ihdr && m::store(doc,value,base).plte==doc.plte
        && m::store(doc,value,base).trns==doc.trns && m::store(doc,value,base).actl==doc.actl
        && m::store(doc,value,base).idat==doc.idat && m::store(doc,value,base).frames==doc.frames by {
        stored_structure(doc,value,base);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(9nat,3u8,8u8,false,false,false,false)==crate::metadata_model::Decision::Keep(3nat)) by (compute);
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    reveal_with_fuel(m::drive,4);
    reveal_with_fuel(m::concat_ranges,3);
}
#[verifier::spinoff_prover]
proof fn scan_plte_ok() ensures r::scan(raw_plte_ok(),a_plte_ok(),0)==Ok(()),
{
    hide(raw_plte_ok);assert(raw_plte_ok().len()==6nat) by (compute);assert(r::count(a_plte_ok())==1nat) by (compute);
    assert(r::pw(a_plte_ok(),0nat)==2nat && r::ph(a_plte_ok(),0nat)==2nat && r::rb(a_plte_ok(),0nat)==2nat && r::base(a_plte_ok(),0nat)==0nat) by (compute);
    assert(raw_plte_ok()[0]==0u8) by (compute);
    f::pass_row_equation(raw_plte_ok(),0nat,2nat,1nat,0nat);
    assert(f::previous_row(raw_plte_ok(),0nat,2nat,1nat,0nat)==None);
    assert(f::encoded_row(0u8,f::row_data(raw_plte_ok(),0nat,2nat,0nat),None,1nat,seq![0u8,1u8],0,2nat)) by (compute);
    f::checked_row_hint(0u8,f::row_data(raw_plte_ok(),0nat,2nat,0nat),None,1nat,seq![0u8,1u8]);
    assert(r::line(raw_plte_ok(),a_plte_ok(),0nat,0nat)==seq![0u8,1u8]);
    assert forall|x: nat| x<2 implies (#[trigger] r::palette_sample(r::line(raw_plte_ok(),a_plte_ok(),0nat,0nat),8u8,x) as nat)<3 by {
        if x==0 {assert((r::palette_sample(seq![0u8,1u8],8u8,0nat) as nat)<3nat) by (compute);assert((r::palette_sample(r::line(raw_plte_ok(),a_plte_ok(),0nat,0nat),8u8,0nat) as nat)<3nat);}
        else if x==1 {assert((r::palette_sample(seq![0u8,1u8],8u8,1nat) as nat)<3nat) by (compute);assert((r::palette_sample(r::line(raw_plte_ok(),a_plte_ok(),0nat,0nat),8u8,1nat) as nat)<3nat);}
    }
    assert(r::palette_valid(r::line(raw_plte_ok(),a_plte_ok(),0nat,0nat),8u8,2nat,3nat));
    assert(r::row_result(raw_plte_ok(),a_plte_ok(),0nat,0nat)==Ok(()));
    assert(raw_plte_ok()[3]==1u8) by (compute);
    f::pass_row_equation(raw_plte_ok(),0nat,2nat,1nat,1nat);
    assert(f::previous_row(raw_plte_ok(),0nat,2nat,1nat,1nat)==Some(seq![0u8,1u8]));
    assert(f::encoded_row(1u8,f::row_data(raw_plte_ok(),0nat,2nat,1nat),Some(seq![0u8,1u8]),1nat,seq![2u8,1u8],0,2nat)) by (compute);
    f::checked_row_hint(1u8,f::row_data(raw_plte_ok(),0nat,2nat,1nat),Some(seq![0u8,1u8]),1nat,seq![2u8,1u8]);
    assert(r::line(raw_plte_ok(),a_plte_ok(),0nat,1nat)==seq![2u8,1u8]);
    assert forall|x: nat| x<2 implies (#[trigger] r::palette_sample(r::line(raw_plte_ok(),a_plte_ok(),0nat,1nat),8u8,x) as nat)<3 by {
        if x==0 {assert((r::palette_sample(seq![2u8,1u8],8u8,0nat) as nat)<3nat) by (compute);assert((r::palette_sample(r::line(raw_plte_ok(),a_plte_ok(),0nat,1nat),8u8,0nat) as nat)<3nat);}
        else if x==1 {assert((r::palette_sample(seq![2u8,1u8],8u8,1nat) as nat)<3nat) by (compute);assert((r::palette_sample(r::line(raw_plte_ok(),a_plte_ok(),0nat,1nat),8u8,1nat) as nat)<3nat);}
    }
    assert(r::palette_valid(r::line(raw_plte_ok(),a_plte_ok(),0nat,1nat),8u8,2nat,3nat));
    assert(r::row_result(raw_plte_ok(),a_plte_ok(),0nat,1nat)==Ok(()));
    r::scan_step(raw_plte_ok(),a_plte_ok(),1nat);
    r::pass_step(raw_plte_ok(),a_plte_ok(),0nat,2nat);
    r::pass_step(raw_plte_ok(),a_plte_ok(),0nat,1nat);
    r::pass_step(raw_plte_ok(),a_plte_ok(),0nat,0nat);
    r::scan_step(raw_plte_ok(),a_plte_ok(),0nat);
}
#[verifier::spinoff_prover]
proof fn row_plte_ok_0_0() ensures r::line(raw_plte_ok(),a_plte_ok(),0nat,0nat)==seq![0u8,1u8],
{
    hide(raw_plte_ok);
    assert(raw_plte_ok().len()==6nat) by (compute);
    assert(raw_plte_ok()[0]==0u8) by (compute);
    assert(r::base(a_plte_ok(),0nat)==0nat && r::rb(a_plte_ok(),0nat)==2nat && f::bpp(r::bits(a_plte_ok()))==1nat) by (compute);
    assert(f::row_data(raw_plte_ok(),0nat,2nat,0nat)==seq![0u8,1u8]) by (compute);
    f::pass_row_equation(raw_plte_ok(),0nat,2nat,1nat,0nat);
    assert(f::previous_row(raw_plte_ok(),0nat,2nat,1nat,0nat)==None);
    assert(f::encoded_row(0u8,f::row_data(raw_plte_ok(),0nat,2nat,0nat),None,1nat,seq![0u8,1u8],0,2nat)) by (compute);
    f::checked_row_hint(0u8,f::row_data(raw_plte_ok(),0nat,2nat,0nat),None,1nat,seq![0u8,1u8]);
}
#[verifier::spinoff_prover]
proof fn row_plte_ok_0_1() ensures r::line(raw_plte_ok(),a_plte_ok(),0nat,1nat)==seq![2u8,1u8],
{
    hide(raw_plte_ok);
    assert(raw_plte_ok().len()==6nat) by (compute);
    assert(raw_plte_ok()[3]==1u8) by (compute);
    assert(r::base(a_plte_ok(),0nat)==0nat && r::rb(a_plte_ok(),0nat)==2nat && f::bpp(r::bits(a_plte_ok()))==1nat) by (compute);
    assert(f::row_data(raw_plte_ok(),0nat,2nat,1nat)==seq![2u8,255u8]) by (compute);
    row_plte_ok_0_0();
    f::pass_row_equation(raw_plte_ok(),0nat,2nat,1nat,1nat);
    assert(f::previous_row(raw_plte_ok(),0nat,2nat,1nat,1nat)==Some(seq![0u8,1u8]));
    assert(f::encoded_row(1u8,f::row_data(raw_plte_ok(),0nat,2nat,1nat),Some(seq![0u8,1u8]),1nat,seq![2u8,1u8],0,2nat)) by (compute);
    f::checked_row_hint(1u8,f::row_data(raw_plte_ok(),0nat,2nat,1nat),Some(seq![0u8,1u8]),1nat,seq![2u8,1u8]);
}
#[verifier::spinoff_prover]
proof fn raster_plte_ok()
    ensures r::default_case(bytes_plte_ok(),h_plte_ok(),z_plte_ok(),3nat),match m::MW_p(bytes_plte_ok(),m::strict()) {m::Outcome::Done(doc,_)=>doc.plte==Some(crate::walk_model::Range {offset:41nat,len:9nat}) && doc.trns==None,_=>false},r::data(z_plte_ok(),a_plte_ok(),STRICT)==Some(Ok(())),
{
    hide(bytes_plte_ok);hide(z_plte_ok);hide(raw_plte_ok);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    assert(bytes_plte_ok().len()==92nat) by (compute);
    assert(crate::model::mh(bytes_plte_ok())==Ok((h_plte_ok(),33nat))) by {reveal(crate::model::mh);assert(crate::model::mh(bytes_plte_ok())==Ok((h_plte_ok(),33nat))) by (compute);}
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_ok(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:9nat,next:54nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_plte_ok(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:9nat,next:54nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(9nat,3u8,8u8,false,false,false,false)==crate::metadata_model::Decision::Keep(3nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_ok(),54nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:62nat,len:14nat,next:80nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_plte_ok(),54nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:62nat,len:14nat,next:80nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_ok(),80nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:88nat,len:0nat,next:92nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_plte_ok(),80nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:88nat,len:0nat,next:92nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_plte_ok().subrange(62,76)==z_plte_ok()) by (compute);
    walk_plte_ok(bytes_plte_ok());
    inflate_plte_ok();scan_plte_ok();
    assert(r::raw(a_plte_ok())==6nat) by (compute);
    assert(r::data(z_plte_ok(),a_plte_ok(),STRICT)==Some(Ok(()))) by {reveal(r::data);};
}
#[verifier::spinoff_prover]
proof fn pixel_plte_ok_0() ensures sm::image(raw_plte_ok(),a_plte_ok(),colour_plte_ok(),false)[0]==47u8,sm::image(raw_plte_ok(),a_plte_ok(),colour_plte_ok(),false)[1]==194u8,sm::image(raw_plte_ok(),a_plte_ok(),colour_plte_ok(),false)[2]==40u8,sm::image(raw_plte_ok(),a_plte_ok(),colour_plte_ok(),false)[3]==255u8,
{
    colour::depth_values(8u8);
    row_plte_ok_0_0();
    assert(crate::paint_model::source_cell(a_plte_ok(),0nat,0nat)==(0nat,0nat,0nat)) by (compute);
    assert(colour::pixel(colour_plte_ok(),seq![0u8,1u8],0nat,true,false)==Ok(seq![12079u16,49858u16,10280u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_plte_ok(),a_plte_ok(),colour_plte_ok(),0nat,0nat,true,false)==Ok(seq![12079u16,49858u16,10280u16,65535u16]));
    assert(((seq![12079u16,49858u16,10280u16,65535u16][0] as nat+128)/257) as u8==47u8) by (compute);
    assert(((seq![12079u16,49858u16,10280u16,65535u16][1] as nat+128)/257) as u8==194u8) by (compute);
    assert(((seq![12079u16,49858u16,10280u16,65535u16][2] as nat+128)/257) as u8==40u8) by (compute);
    assert(((seq![12079u16,49858u16,10280u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixel_plte_ok_1() ensures sm::image(raw_plte_ok(),a_plte_ok(),colour_plte_ok(),false)[4]==139u8,sm::image(raw_plte_ok(),a_plte_ok(),colour_plte_ok(),false)[5]==44u8,sm::image(raw_plte_ok(),a_plte_ok(),colour_plte_ok(),false)[6]==88u8,sm::image(raw_plte_ok(),a_plte_ok(),colour_plte_ok(),false)[7]==255u8,
{
    colour::depth_values(8u8);
    row_plte_ok_0_0();
    assert(crate::paint_model::source_cell(a_plte_ok(),1nat,0nat)==(0nat,1nat,0nat)) by (compute);
    assert(colour::pixel(colour_plte_ok(),seq![0u8,1u8],1nat,true,false)==Ok(seq![35723u16,11308u16,22616u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_plte_ok(),a_plte_ok(),colour_plte_ok(),1nat,0nat,true,false)==Ok(seq![35723u16,11308u16,22616u16,65535u16]));
    assert(((seq![35723u16,11308u16,22616u16,65535u16][0] as nat+128)/257) as u8==139u8) by (compute);
    assert(((seq![35723u16,11308u16,22616u16,65535u16][1] as nat+128)/257) as u8==44u8) by (compute);
    assert(((seq![35723u16,11308u16,22616u16,65535u16][2] as nat+128)/257) as u8==88u8) by (compute);
    assert(((seq![35723u16,11308u16,22616u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixel_plte_ok_2() ensures sm::image(raw_plte_ok(),a_plte_ok(),colour_plte_ok(),false)[8]==228u8,sm::image(raw_plte_ok(),a_plte_ok(),colour_plte_ok(),false)[9]==36u8,sm::image(raw_plte_ok(),a_plte_ok(),colour_plte_ok(),false)[10]==202u8,sm::image(raw_plte_ok(),a_plte_ok(),colour_plte_ok(),false)[11]==255u8,
{
    colour::depth_values(8u8);
    row_plte_ok_0_1();
    assert(crate::paint_model::source_cell(a_plte_ok(),0nat,1nat)==(0nat,0nat,1nat)) by (compute);
    assert(colour::pixel(colour_plte_ok(),seq![2u8,1u8],0nat,true,false)==Ok(seq![58596u16,9252u16,51914u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_plte_ok(),a_plte_ok(),colour_plte_ok(),0nat,1nat,true,false)==Ok(seq![58596u16,9252u16,51914u16,65535u16]));
    assert(((seq![58596u16,9252u16,51914u16,65535u16][0] as nat+128)/257) as u8==228u8) by (compute);
    assert(((seq![58596u16,9252u16,51914u16,65535u16][1] as nat+128)/257) as u8==36u8) by (compute);
    assert(((seq![58596u16,9252u16,51914u16,65535u16][2] as nat+128)/257) as u8==202u8) by (compute);
    assert(((seq![58596u16,9252u16,51914u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixel_plte_ok_3() ensures sm::image(raw_plte_ok(),a_plte_ok(),colour_plte_ok(),false)[12]==139u8,sm::image(raw_plte_ok(),a_plte_ok(),colour_plte_ok(),false)[13]==44u8,sm::image(raw_plte_ok(),a_plte_ok(),colour_plte_ok(),false)[14]==88u8,sm::image(raw_plte_ok(),a_plte_ok(),colour_plte_ok(),false)[15]==255u8,
{
    colour::depth_values(8u8);
    row_plte_ok_0_1();
    assert(crate::paint_model::source_cell(a_plte_ok(),1nat,1nat)==(0nat,1nat,1nat)) by (compute);
    assert(colour::pixel(colour_plte_ok(),seq![2u8,1u8],1nat,true,false)==Ok(seq![35723u16,11308u16,22616u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_plte_ok(),a_plte_ok(),colour_plte_ok(),1nat,1nat,true,false)==Ok(seq![35723u16,11308u16,22616u16,65535u16]));
    assert(((seq![35723u16,11308u16,22616u16,65535u16][0] as nat+128)/257) as u8==139u8) by (compute);
    assert(((seq![35723u16,11308u16,22616u16,65535u16][1] as nat+128)/257) as u8==44u8) by (compute);
    assert(((seq![35723u16,11308u16,22616u16,65535u16][2] as nat+128)/257) as u8==88u8) by (compute);
    assert(((seq![35723u16,11308u16,22616u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixels_plte_ok() ensures sm::image(raw_plte_ok(),a_plte_ok(),colour_plte_ok(),false)==expected_plte_ok(),
{
    pixel_plte_ok_0();
    pixel_plte_ok_1();
    pixel_plte_ok_2();
    pixel_plte_ok_3();
    assert(a_plte_ok().width==2nat && a_plte_ok().height==2nat) by (compute);
    assert(sm::image(raw_plte_ok(),a_plte_ok(),colour_plte_ok(),false).len()==16nat);
    assert(expected_plte_ok().len()==16nat) by (compute);
    assert forall|j: int| 0<=j<16 implies (#[trigger] sm::image(raw_plte_ok(),a_plte_ok(),colour_plte_ok(),false)[j])==expected_plte_ok()[j] by {
        if j==0 {assert(expected_plte_ok()[0]==47u8) by (compute);}
        else if j==1 {assert(expected_plte_ok()[1]==194u8) by (compute);}
        else if j==2 {assert(expected_plte_ok()[2]==40u8) by (compute);}
        else if j==3 {assert(expected_plte_ok()[3]==255u8) by (compute);}
        else if j==4 {assert(expected_plte_ok()[4]==139u8) by (compute);}
        else if j==5 {assert(expected_plte_ok()[5]==44u8) by (compute);}
        else if j==6 {assert(expected_plte_ok()[6]==88u8) by (compute);}
        else if j==7 {assert(expected_plte_ok()[7]==255u8) by (compute);}
        else if j==8 {assert(expected_plte_ok()[8]==228u8) by (compute);}
        else if j==9 {assert(expected_plte_ok()[9]==36u8) by (compute);}
        else if j==10 {assert(expected_plte_ok()[10]==202u8) by (compute);}
        else if j==11 {assert(expected_plte_ok()[11]==255u8) by (compute);}
        else if j==12 {assert(expected_plte_ok()[12]==139u8) by (compute);}
        else if j==13 {assert(expected_plte_ok()[13]==44u8) by (compute);}
        else if j==14 {assert(expected_plte_ok()[14]==88u8) by (compute);}
        else {assert(j==15);assert(expected_plte_ok()[j]==255u8) by (compute);}
    }
    assert(sm::image(raw_plte_ok(),a_plte_ok(),colour_plte_ok(),false)=~=expected_plte_ok());
}
pub open spec fn expected_value_plte_ok(value: sm::Value) -> bool {value.width==2u32 && value.height==2u32 && value.pixels==expected_plte_ok()}
#[verifier::spinoff_prover]
pub proof fn model_complete_plte_ok()
    ensures match sm::MS(bytes_plte_ok()) {sm::Outcome::Done(value)=>expected_value_plte_ok(value),_=>false},
{
    hide(m::MW_p);hide(zm::Inflate);hide(bytes_plte_ok);hide(z_plte_ok);hide(raw_plte_ok);hide(expected_plte_ok);
    assert(bytes_plte_ok().len()==92nat) by (compute);
    raster_plte_ok();inflate_plte_ok();pixels_plte_ok();
    assert(bytes_plte_ok().subrange(41,50)==seq![47u8,194u8,40u8,139u8,44u8,88u8,228u8,36u8,202u8]) by (compute);
    match m::MW_p(bytes_plte_ok(),m::strict()) {m::Outcome::Done(doc,_)=>{
        assert(colour::from_doc(doc,bytes_plte_ok())==colour_plte_ok());
        assert(crate::paint_model::default_raster(doc,STRICT)==a_plte_ok());
    },_=>{}}
    assert(r::raw(a_plte_ok())==6nat) by (compute);
    assert(r::inflated(z_plte_ok(),a_plte_ok(),STRICT).out==raw_plte_ok());
    compose_complete(bytes_plte_ok(),h_plte_ok(),z_plte_ok(),3nat,colour_plte_ok(),expected_plte_ok());
}
pub fn public_complete_plte_ok() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_plte_ok(),false),result),se::eq(bytes_plte_ok(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_plte_ok(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,3u8,0u8,0u8,0u8,69u8,104u8,253u8,22u8,0u8,0u8,0u8,9u8,80u8,76u8,84u8,69u8,47u8,194u8,40u8,139u8,44u8,88u8,228u8,36u8,202u8,64u8,183u8,32u8,36u8,0u8,0u8,0u8,14u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,96u8,100u8,100u8,250u8,15u8,0u8,1u8,16u8,1u8,4u8,216u8,172u8,32u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_plte_ok());model_complete_plte_ok();}
    crate::png::try_decode_still(&b)
}
// plte/suggested-palette-in-rgb; frozen fixture SHA256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
pub open spec fn bytes_plte_suggested_palette_in_rgb() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,2u8,88u8,80u8,76u8,84u8,69u8,217u8,5u8,126u8,229u8,40u8,28u8,92u8,184u8,223u8,213u8,121u8,238u8,109u8,36u8,77u8,4u8,167u8,27u8,166u8,48u8,111u8,49u8,91u8,221u8,89u8,53u8,190u8,63u8,235u8,76u8,90u8,66u8,129u8,96u8,87u8,99u8,39u8,127u8,211u8,1u8,24u8,51u8,79u8,48u8,95u8,25u8,251u8,236u8,141u8,58u8,166u8,39u8,245u8,176u8,238u8,21u8,176u8,218u8,3u8,95u8,44u8,200u8,190u8,150u8,158u8,3u8,203u8,176u8,192u8,177u8,86u8,198u8,222u8,55u8,22u8,49u8,216u8,137u8,191u8,32u8,64u8,11u8,236u8,238u8,221u8,93u8,106u8,153u8,119u8,70u8,160u8,70u8,148u8,150u8,144u8,89u8,193u8,82u8,205u8,5u8,90u8,208u8,139u8,82u8,184u8,18u8,158u8,37u8,15u8,12u8,185u8,71u8,81u8,180u8,11u8,151u8,95u8,249u8,148u8,227u8,205u8,172u8,235u8,189u8,229u8,52u8,2u8,44u8,166u8,182u8,186u8,227u8,37u8,103u8,130u8,94u8,104u8,184u8,196u8,230u8,13u8,127u8,235u8,164u8,217u8,129u8,6u8,224u8,233u8,236u8,72u8,246u8,169u8,246u8,62u8,80u8,88u8,219u8,152u8,231u8,189u8,105u8,160u8,27u8,215u8,164u8,173u8,35u8,140u8,75u8,214u8,92u8,33u8,84u8,15u8,83u8,66u8,220u8,58u8,109u8,174u8,187u8,145u8,26u8,93u8,11u8,244u8,15u8,150u8,157u8,165u8,108u8,14u8,34u8,101u8,70u8,42u8,210u8,68u8,224u8,234u8,58u8,144u8,7u8,8u8,200u8,112u8,114u8,9u8,227u8,95u8,168u8,194u8,121u8,243u8,15u8,246u8,20u8,171u8,24u8,54u8,128u8,113u8,96u8,45u8,80u8,75u8,94u8,243u8,169u8,254u8,241u8,217u8,214u8,102u8,180u8,124u8,54u8,47u8,156u8,179u8,173u8,102u8,209u8,90u8,28u8,66u8,17u8,60u8,126u8,113u8,193u8,193u8,66u8,150u8,139u8,200u8,94u8,162u8,111u8,143u8,200u8,56u8,253u8,248u8,90u8,238u8,150u8,233u8,78u8,42u8,134u8,186u8,216u8,70u8,219u8,58u8,151u8,73u8,56u8,238u8,38u8,222u8,47u8,178u8,181u8,4u8,136u8,214u8,241u8,102u8,191u8,39u8,49u8,55u8,195u8,188u8,50u8,105u8,210u8,89u8,27u8,49u8,27u8,141u8,39u8,245u8,231u8,35u8,162u8,106u8,53u8,66u8,44u8,203u8,2u8,112u8,205u8,234u8,232u8,133u8,0u8,87u8,145u8,15u8,0u8,125u8,215u8,173u8,105u8,7u8,91u8,227u8,195u8,221u8,8u8,191u8,160u8,132u8,214u8,106u8,143u8,215u8,2u8,98u8,88u8,220u8,115u8,213u8,193u8,166u8,32u8,233u8,198u8,77u8,22u8,163u8,234u8,97u8,166u8,220u8,183u8,61u8,91u8,16u8,231u8,70u8,168u8,57u8,36u8,111u8,154u8,180u8,231u8,115u8,221u8,117u8,210u8,228u8,244u8,152u8,199u8,195u8,219u8,60u8,15u8,195u8,114u8,200u8,222u8,241u8,5u8,154u8,61u8,251u8,237u8,87u8,7u8,200u8,90u8,30u8,146u8,116u8,162u8,211u8,33u8,75u8,247u8,91u8,245u8,163u8,252u8,212u8,218u8,231u8,68u8,20u8,108u8,86u8,62u8,179u8,124u8,34u8,164u8,253u8,8u8,165u8,157u8,17u8,198u8,157u8,168u8,130u8,98u8,106u8,84u8,83u8,32u8,16u8,240u8,55u8,94u8,26u8,86u8,170u8,69u8,120u8,114u8,244u8,241u8,223u8,14u8,212u8,227u8,219u8,218u8,82u8,90u8,212u8,102u8,200u8,191u8,175u8,62u8,1u8,118u8,236u8,85u8,111u8,228u8,181u8,36u8,156u8,216u8,90u8,236u8,19u8,246u8,19u8,182u8,40u8,196u8,221u8,94u8,247u8,11u8,241u8,38u8,236u8,90u8,45u8,63u8,248u8,237u8,144u8,82u8,35u8,143u8,89u8,30u8,100u8,154u8,194u8,162u8,16u8,140u8,82u8,88u8,69u8,53u8,106u8,170u8,117u8,126u8,213u8,72u8,33u8,243u8,176u8,253u8,209u8,23u8,227u8,0u8,194u8,26u8,251u8,229u8,54u8,130u8,184u8,87u8,126u8,228u8,255u8,39u8,216u8,35u8,81u8,111u8,215u8,222u8,86u8,250u8,173u8,226u8,86u8,44u8,120u8,253u8,66u8,1u8,217u8,1u8,43u8,145u8,75u8,132u8,77u8,196u8,237u8,112u8,162u8,123u8,156u8,80u8,214u8,90u8,240u8,154u8,205u8,49u8,148u8,210u8,253u8,245u8,117u8,255u8,99u8,48u8,82u8,20u8,48u8,201u8,131u8,237u8,156u8,171u8,200u8,8u8,88u8,124u8,80u8,131u8,125u8,108u8,63u8,37u8,245u8,67u8,214u8,20u8,107u8,218u8,0u8,0u8,0u8,6u8,98u8,75u8,71u8,68u8,0u8,1u8,0u8,2u8,0u8,3u8,94u8,174u8,23u8,27u8,0u8,0u8,1u8,144u8,104u8,73u8,83u8,84u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,93u8,124u8,21u8,200u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn h_plte_suggested_palette_in_rgb() -> crate::model::Ihdr {crate::model::Ihdr {width:2u32,height:2u32,depth:8u8,color_type:2u8,interlace:0u8}}
spec fn a_plte_suggested_palette_in_rgb() -> r::Raster {r::shape(h_plte_suggested_palette_in_rgb(),2nat,2nat,200nat,STRICT)}
spec fn colour_plte_suggested_palette_in_rgb() -> crate::colour_model::Context {crate::colour_model::Context {ct:2u8,depth:8u8,plte:seq![217u8,5u8,126u8,229u8,40u8,28u8,92u8,184u8,223u8,213u8,121u8,238u8,109u8,36u8,77u8,4u8,167u8,27u8,166u8,48u8,111u8,49u8,91u8,221u8,89u8,53u8,190u8,63u8,235u8,76u8,90u8,66u8,129u8,96u8,87u8,99u8,39u8,127u8,211u8,1u8,24u8,51u8,79u8,48u8,95u8,25u8,251u8,236u8,141u8,58u8,166u8,39u8,245u8,176u8,238u8,21u8,176u8,218u8,3u8,95u8,44u8,200u8,190u8,150u8,158u8,3u8,203u8,176u8,192u8,177u8,86u8,198u8,222u8,55u8,22u8,49u8,216u8,137u8,191u8,32u8,64u8,11u8,236u8,238u8,221u8,93u8,106u8,153u8,119u8,70u8,160u8,70u8,148u8,150u8,144u8,89u8,193u8,82u8,205u8,5u8,90u8,208u8,139u8,82u8,184u8,18u8,158u8,37u8,15u8,12u8,185u8,71u8,81u8,180u8,11u8,151u8,95u8,249u8,148u8,227u8,205u8,172u8,235u8,189u8,229u8,52u8,2u8,44u8,166u8,182u8,186u8,227u8,37u8,103u8,130u8,94u8,104u8,184u8,196u8,230u8,13u8,127u8,235u8,164u8,217u8,129u8,6u8,224u8,233u8,236u8,72u8,246u8,169u8,246u8,62u8,80u8,88u8,219u8,152u8,231u8,189u8,105u8,160u8,27u8,215u8,164u8,173u8,35u8,140u8,75u8,214u8,92u8,33u8,84u8,15u8,83u8,66u8,220u8,58u8,109u8,174u8,187u8,145u8,26u8,93u8,11u8,244u8,15u8,150u8,157u8,165u8,108u8,14u8,34u8,101u8,70u8,42u8,210u8,68u8,224u8,234u8,58u8,144u8,7u8,8u8,200u8,112u8,114u8,9u8,227u8,95u8,168u8,194u8,121u8,243u8,15u8,246u8,20u8,171u8,24u8,54u8,128u8,113u8,96u8,45u8,80u8,75u8,94u8,243u8,169u8,254u8,241u8,217u8,214u8,102u8,180u8,124u8,54u8,47u8,156u8,179u8,173u8,102u8,209u8,90u8,28u8,66u8,17u8,60u8,126u8,113u8,193u8,193u8,66u8,150u8,139u8,200u8,94u8,162u8,111u8,143u8,200u8,56u8,253u8,248u8,90u8,238u8,150u8,233u8,78u8,42u8,134u8,186u8,216u8,70u8,219u8,58u8,151u8,73u8,56u8,238u8,38u8,222u8,47u8,178u8,181u8,4u8,136u8,214u8,241u8,102u8,191u8,39u8,49u8,55u8,195u8,188u8,50u8,105u8,210u8,89u8,27u8,49u8,27u8,141u8,39u8,245u8,231u8,35u8,162u8,106u8,53u8,66u8,44u8,203u8,2u8,112u8,205u8,234u8,232u8,133u8,0u8,87u8,145u8,15u8,0u8,125u8,215u8,173u8,105u8,7u8,91u8,227u8,195u8,221u8,8u8,191u8,160u8,132u8,214u8,106u8,143u8,215u8,2u8,98u8,88u8,220u8,115u8,213u8,193u8,166u8,32u8,233u8,198u8,77u8,22u8,163u8,234u8,97u8,166u8,220u8,183u8,61u8,91u8,16u8,231u8,70u8,168u8,57u8,36u8,111u8,154u8,180u8,231u8,115u8,221u8,117u8,210u8,228u8,244u8,152u8,199u8,195u8,219u8,60u8,15u8,195u8,114u8,200u8,222u8,241u8,5u8,154u8,61u8,251u8,237u8,87u8,7u8,200u8,90u8,30u8,146u8,116u8,162u8,211u8,33u8,75u8,247u8,91u8,245u8,163u8,252u8,212u8,218u8,231u8,68u8,20u8,108u8,86u8,62u8,179u8,124u8,34u8,164u8,253u8,8u8,165u8,157u8,17u8,198u8,157u8,168u8,130u8,98u8,106u8,84u8,83u8,32u8,16u8,240u8,55u8,94u8,26u8,86u8,170u8,69u8,120u8,114u8,244u8,241u8,223u8,14u8,212u8,227u8,219u8,218u8,82u8,90u8,212u8,102u8,200u8,191u8,175u8,62u8,1u8,118u8,236u8,85u8,111u8,228u8,181u8,36u8,156u8,216u8,90u8,236u8,19u8,246u8,19u8,182u8,40u8,196u8,221u8,94u8,247u8,11u8,241u8,38u8,236u8,90u8,45u8,63u8,248u8,237u8,144u8,82u8,35u8,143u8,89u8,30u8,100u8,154u8,194u8,162u8,16u8,140u8,82u8,88u8,69u8,53u8,106u8,170u8,117u8,126u8,213u8,72u8,33u8,243u8,176u8,253u8,209u8,23u8,227u8,0u8,194u8,26u8,251u8,229u8,54u8,130u8,184u8,87u8,126u8,228u8,255u8,39u8,216u8,35u8,81u8,111u8,215u8,222u8,86u8,250u8,173u8,226u8,86u8,44u8,120u8,253u8,66u8,1u8,217u8,1u8,43u8,145u8,75u8,132u8,77u8,196u8,237u8,112u8,162u8,123u8,156u8,80u8,214u8,90u8,240u8,154u8,205u8,49u8,148u8,210u8,253u8,245u8,117u8,255u8,99u8,48u8,82u8,20u8,48u8,201u8,131u8,237u8,156u8,171u8,200u8,8u8,88u8,124u8,80u8,131u8,125u8,108u8,63u8,37u8,245u8,67u8],trns:None}}
pub open spec fn expected_plte_suggested_palette_in_rgb() -> Seq<u8> {seq![201u8,73u8,63u8,255u8,202u8,75u8,151u8,255u8,225u8,26u8,23u8,255u8,252u8,246u8,129u8,255u8]}
spec fn z_plte_suggested_palette_in_rgb() -> Seq<u8> {seq![120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8]}
spec fn raw_plte_suggested_palette_in_rgb() -> Seq<u8> {seq![0u8,201u8,73u8,63u8,202u8,75u8,151u8,1u8,225u8,26u8,23u8,27u8,220u8,106u8]}
spec fn ts_plte_suggested_palette_in_rgb() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:36nat,before:1nat,after:2nat,symbol:201nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:36nat,next:44nat,before:2nat,after:3nat,symbol:73nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:44nat,next:52nat,before:3nat,after:4nat,symbol:63nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:52nat,next:61nat,before:4nat,after:5nat,symbol:202nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:61nat,next:69nat,before:5nat,after:6nat,symbol:75nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:69nat,next:78nat,before:6nat,after:7nat,symbol:151nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:78nat,next:86nat,before:7nat,after:8nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:86nat,next:95nat,before:8nat,after:9nat,symbol:225nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:95nat,next:103nat,before:9nat,after:10nat,symbol:26nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:103nat,next:111nat,before:10nat,after:11nat,symbol:23nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:111nat,next:119nat,before:11nat,after:12nat,symbol:27nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:119nat,next:128nat,before:12nat,after:13nat,symbol:220nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:128nat,next:136nat,before:13nat,after:14nat,symbol:106nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:136nat,next:143nat,before:14nat,after:14nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_plte_suggested_palette_in_rgb_0() ensures t::trace(z_plte_suggested_palette_in_rgb(),raw_plte_suggested_palette_in_rgb(),ts_plte_suggested_palette_in_rgb(),0nat,15nat),
{
    hide(t::valid_step);assert(t::trace(z_plte_suggested_palette_in_rgb(),raw_plte_suggested_palette_in_rgb(),ts_plte_suggested_palette_in_rgb(),0nat,15nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn trace_plte_suggested_palette_in_rgb() ensures t::trace(z_plte_suggested_palette_in_rgb(),raw_plte_suggested_palette_in_rgb(),ts_plte_suggested_palette_in_rgb(),0,15nat),
{
    hide(z_plte_suggested_palette_in_rgb);hide(raw_plte_suggested_palette_in_rgb);hide(ts_plte_suggested_palette_in_rgb);hide(t::trace);
    trace_plte_suggested_palette_in_rgb_0();
    assert(t::trace(z_plte_suggested_palette_in_rgb(),raw_plte_suggested_palette_in_rgb(),ts_plte_suggested_palette_in_rgb(),0,15nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_plte_suggested_palette_in_rgb() ensures z_plte_suggested_palette_in_rgb().len()==22nat,
        raw_plte_suggested_palette_in_rgb().len()==14nat,
        z_plte_suggested_palette_in_rgb()[0]==120u8,
        z_plte_suggested_palette_in_rgb()[1]==156u8,
        zm::field_value(z_plte_suggested_palette_in_rgb(),16,1)==1nat,
        zm::field_value(z_plte_suggested_palette_in_rgb(),17,2)==1nat,
        zm::be_value(z_plte_suggested_palette_in_rgb(),18nat,4)==675349874nat,
        ts_plte_suggested_palette_in_rgb().len()==15nat,
        ts_plte_suggested_palette_in_rgb()[0].pos==19nat,
        ts_plte_suggested_palette_in_rgb()[0].before==0nat,
        ts_plte_suggested_palette_in_rgb().last().next==143nat,
{
    assert(z_plte_suggested_palette_in_rgb().len()==22nat) by (compute);
    assert(raw_plte_suggested_palette_in_rgb().len()==14nat) by (compute);
    assert(z_plte_suggested_palette_in_rgb()[0]==120u8) by (compute);
    assert(z_plte_suggested_palette_in_rgb()[1]==156u8) by (compute);
    assert(zm::field_value(z_plte_suggested_palette_in_rgb(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_plte_suggested_palette_in_rgb(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_plte_suggested_palette_in_rgb(),18nat,4)==675349874nat) by (compute);
    assert(ts_plte_suggested_palette_in_rgb().len()==15nat) by (compute);
    assert(ts_plte_suggested_palette_in_rgb()[0].pos==19nat) by (compute);
    assert(ts_plte_suggested_palette_in_rgb()[0].before==0nat) by (compute);
    assert(ts_plte_suggested_palette_in_rgb().last().next==143nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_plte_suggested_palette_in_rgb() ensures zm::Adler(raw_plte_suggested_palette_in_rgb())==675349874nat,
{
    hide(raw_plte_suggested_palette_in_rgb);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_plte_suggested_palette_in_rgb().len()==14nat) by (compute);
    assert(c::sums(raw_plte_suggested_palette_in_rgb(),0,14nat)==(1393nat,10305nat)) by (compute);
    c::sums_correct(raw_plte_suggested_palette_in_rgb(),0,14nat);assert(raw_plte_suggested_palette_in_rgb().subrange(0,14)=~=raw_plte_suggested_palette_in_rgb());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((10305nat%65521)*(u16::MAX as nat+1)+(1+1393nat)%65521==675349874nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_plte_suggested_palette_in_rgb() ensures zm::Inflate(z_plte_suggested_palette_in_rgb(),14nat,true,false)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_plte_suggested_palette_in_rgb(),r2:false}),raw_plte_suggested_palette_in_rgb().len()==14nat,
{
    hide(z_plte_suggested_palette_in_rgb);hide(raw_plte_suggested_palette_in_rgb);hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);hide(ts_plte_suggested_palette_in_rgb);
    shape_plte_suggested_palette_in_rgb();checksum_plte_suggested_palette_in_rgb();
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_plte_suggested_palette_in_rgb();ms::fixed_stream(z_plte_suggested_palette_in_rgb(),raw_plte_suggested_palette_in_rgb(),ts_plte_suggested_palette_in_rgb(),14nat,156nat,675349874nat,675349874nat);
}
#[verifier::spinoff_prover]
proof fn crc_plte_suggested_palette_in_rgb_crc_33()
    ensures crate::model::crc(bytes_plte_suggested_palette_in_rgb().subrange(37,641))==3591662554u32,
{
    hide(crate::model::crc_register);hide(crc_cert::fold);
    assert(crate::model::crc_register(bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(0))==0xffff_ffffu32) by {
        assert(bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(0) =~= Seq::<u8>::empty());
        reveal_with_fuel(crate::model::crc_register,1);
    }
    assert(crc_cert::fold(4294967295u32,bytes_plte_suggested_palette_in_rgb().subrange(37,641).subrange(0,64))==3048298906u32) by (compute);
    crc_cert::suffix(bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(0),bytes_plte_suggested_palette_in_rgb().subrange(37,641).subrange(0,64));
    assert(bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(0)+bytes_plte_suggested_palette_in_rgb().subrange(37,641).subrange(0,64) =~= bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(64));
    assert(crate::model::crc_register(bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(64))==3048298906u32);
    assert(crc_cert::fold(3048298906u32,bytes_plte_suggested_palette_in_rgb().subrange(37,641).subrange(64,128))==4228138138u32) by (compute);
    crc_cert::suffix(bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(64),bytes_plte_suggested_palette_in_rgb().subrange(37,641).subrange(64,128));
    assert(bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(64)+bytes_plte_suggested_palette_in_rgb().subrange(37,641).subrange(64,128) =~= bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(128));
    assert(crate::model::crc_register(bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(128))==4228138138u32);
    assert(crc_cert::fold(4228138138u32,bytes_plte_suggested_palette_in_rgb().subrange(37,641).subrange(128,192))==2647548738u32) by (compute);
    crc_cert::suffix(bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(128),bytes_plte_suggested_palette_in_rgb().subrange(37,641).subrange(128,192));
    assert(bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(128)+bytes_plte_suggested_palette_in_rgb().subrange(37,641).subrange(128,192) =~= bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(192));
    assert(crate::model::crc_register(bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(192))==2647548738u32);
    assert(crc_cert::fold(2647548738u32,bytes_plte_suggested_palette_in_rgb().subrange(37,641).subrange(192,256))==51299165u32) by (compute);
    crc_cert::suffix(bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(192),bytes_plte_suggested_palette_in_rgb().subrange(37,641).subrange(192,256));
    assert(bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(192)+bytes_plte_suggested_palette_in_rgb().subrange(37,641).subrange(192,256) =~= bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(256));
    assert(crate::model::crc_register(bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(256))==51299165u32);
    assert(crc_cert::fold(51299165u32,bytes_plte_suggested_palette_in_rgb().subrange(37,641).subrange(256,320))==1506279533u32) by (compute);
    crc_cert::suffix(bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(256),bytes_plte_suggested_palette_in_rgb().subrange(37,641).subrange(256,320));
    assert(bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(256)+bytes_plte_suggested_palette_in_rgb().subrange(37,641).subrange(256,320) =~= bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(320));
    assert(crate::model::crc_register(bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(320))==1506279533u32);
    assert(crc_cert::fold(1506279533u32,bytes_plte_suggested_palette_in_rgb().subrange(37,641).subrange(320,384))==3502014779u32) by (compute);
    crc_cert::suffix(bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(320),bytes_plte_suggested_palette_in_rgb().subrange(37,641).subrange(320,384));
    assert(bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(320)+bytes_plte_suggested_palette_in_rgb().subrange(37,641).subrange(320,384) =~= bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(384));
    assert(crate::model::crc_register(bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(384))==3502014779u32);
    assert(crc_cert::fold(3502014779u32,bytes_plte_suggested_palette_in_rgb().subrange(37,641).subrange(384,448))==2742300815u32) by (compute);
    crc_cert::suffix(bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(384),bytes_plte_suggested_palette_in_rgb().subrange(37,641).subrange(384,448));
    assert(bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(384)+bytes_plte_suggested_palette_in_rgb().subrange(37,641).subrange(384,448) =~= bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(448));
    assert(crate::model::crc_register(bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(448))==2742300815u32);
    assert(crc_cert::fold(2742300815u32,bytes_plte_suggested_palette_in_rgb().subrange(37,641).subrange(448,512))==3094920238u32) by (compute);
    crc_cert::suffix(bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(448),bytes_plte_suggested_palette_in_rgb().subrange(37,641).subrange(448,512));
    assert(bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(448)+bytes_plte_suggested_palette_in_rgb().subrange(37,641).subrange(448,512) =~= bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(512));
    assert(crate::model::crc_register(bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(512))==3094920238u32);
    assert(crc_cert::fold(3094920238u32,bytes_plte_suggested_palette_in_rgb().subrange(37,641).subrange(512,576))==2901865761u32) by (compute);
    crc_cert::suffix(bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(512),bytes_plte_suggested_palette_in_rgb().subrange(37,641).subrange(512,576));
    assert(bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(512)+bytes_plte_suggested_palette_in_rgb().subrange(37,641).subrange(512,576) =~= bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(576));
    assert(crate::model::crc_register(bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(576))==2901865761u32);
    assert(crc_cert::fold(2901865761u32,bytes_plte_suggested_palette_in_rgb().subrange(37,641).subrange(576,604))==703304741u32) by (compute);
    crc_cert::suffix(bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(576),bytes_plte_suggested_palette_in_rgb().subrange(37,641).subrange(576,604));
    assert(bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(576)+bytes_plte_suggested_palette_in_rgb().subrange(37,641).subrange(576,604) =~= bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(604));
    assert(crate::model::crc_register(bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(604))==703304741u32);
    assert(bytes_plte_suggested_palette_in_rgb().subrange(37,641).take(604) =~= bytes_plte_suggested_palette_in_rgb().subrange(37,641));
    assert((703304741u32 ^ 0xffff_ffffu32)==3591662554u32) by (compute);
}
#[verifier::spinoff_prover]
proof fn crc_plte_suggested_palette_in_rgb_crc_663()
    ensures crate::model::crc(bytes_plte_suggested_palette_in_rgb().subrange(667,1071))==1568413128u32,
{
    hide(crate::model::crc_register);hide(crc_cert::fold);
    assert(crate::model::crc_register(bytes_plte_suggested_palette_in_rgb().subrange(667,1071).take(0))==0xffff_ffffu32) by {
        assert(bytes_plte_suggested_palette_in_rgb().subrange(667,1071).take(0) =~= Seq::<u8>::empty());
        reveal_with_fuel(crate::model::crc_register,1);
    }
    assert(crc_cert::fold(4294967295u32,bytes_plte_suggested_palette_in_rgb().subrange(667,1071).subrange(0,64))==1173182323u32) by (compute);
    crc_cert::suffix(bytes_plte_suggested_palette_in_rgb().subrange(667,1071).take(0),bytes_plte_suggested_palette_in_rgb().subrange(667,1071).subrange(0,64));
    assert(bytes_plte_suggested_palette_in_rgb().subrange(667,1071).take(0)+bytes_plte_suggested_palette_in_rgb().subrange(667,1071).subrange(0,64) =~= bytes_plte_suggested_palette_in_rgb().subrange(667,1071).take(64));
    assert(crate::model::crc_register(bytes_plte_suggested_palette_in_rgb().subrange(667,1071).take(64))==1173182323u32);
    assert(crc_cert::fold(1173182323u32,bytes_plte_suggested_palette_in_rgb().subrange(667,1071).subrange(64,128))==3665903195u32) by (compute);
    crc_cert::suffix(bytes_plte_suggested_palette_in_rgb().subrange(667,1071).take(64),bytes_plte_suggested_palette_in_rgb().subrange(667,1071).subrange(64,128));
    assert(bytes_plte_suggested_palette_in_rgb().subrange(667,1071).take(64)+bytes_plte_suggested_palette_in_rgb().subrange(667,1071).subrange(64,128) =~= bytes_plte_suggested_palette_in_rgb().subrange(667,1071).take(128));
    assert(crate::model::crc_register(bytes_plte_suggested_palette_in_rgb().subrange(667,1071).take(128))==3665903195u32);
    assert(crc_cert::fold(3665903195u32,bytes_plte_suggested_palette_in_rgb().subrange(667,1071).subrange(128,192))==4191442066u32) by (compute);
    crc_cert::suffix(bytes_plte_suggested_palette_in_rgb().subrange(667,1071).take(128),bytes_plte_suggested_palette_in_rgb().subrange(667,1071).subrange(128,192));
    assert(bytes_plte_suggested_palette_in_rgb().subrange(667,1071).take(128)+bytes_plte_suggested_palette_in_rgb().subrange(667,1071).subrange(128,192) =~= bytes_plte_suggested_palette_in_rgb().subrange(667,1071).take(192));
    assert(crate::model::crc_register(bytes_plte_suggested_palette_in_rgb().subrange(667,1071).take(192))==4191442066u32);
    assert(crc_cert::fold(4191442066u32,bytes_plte_suggested_palette_in_rgb().subrange(667,1071).subrange(192,256))==2561080018u32) by (compute);
    crc_cert::suffix(bytes_plte_suggested_palette_in_rgb().subrange(667,1071).take(192),bytes_plte_suggested_palette_in_rgb().subrange(667,1071).subrange(192,256));
    assert(bytes_plte_suggested_palette_in_rgb().subrange(667,1071).take(192)+bytes_plte_suggested_palette_in_rgb().subrange(667,1071).subrange(192,256) =~= bytes_plte_suggested_palette_in_rgb().subrange(667,1071).take(256));
    assert(crate::model::crc_register(bytes_plte_suggested_palette_in_rgb().subrange(667,1071).take(256))==2561080018u32);
    assert(crc_cert::fold(2561080018u32,bytes_plte_suggested_palette_in_rgb().subrange(667,1071).subrange(256,320))==1515736370u32) by (compute);
    crc_cert::suffix(bytes_plte_suggested_palette_in_rgb().subrange(667,1071).take(256),bytes_plte_suggested_palette_in_rgb().subrange(667,1071).subrange(256,320));
    assert(bytes_plte_suggested_palette_in_rgb().subrange(667,1071).take(256)+bytes_plte_suggested_palette_in_rgb().subrange(667,1071).subrange(256,320) =~= bytes_plte_suggested_palette_in_rgb().subrange(667,1071).take(320));
    assert(crate::model::crc_register(bytes_plte_suggested_palette_in_rgb().subrange(667,1071).take(320))==1515736370u32);
    assert(crc_cert::fold(1515736370u32,bytes_plte_suggested_palette_in_rgb().subrange(667,1071).subrange(320,384))==2239207243u32) by (compute);
    crc_cert::suffix(bytes_plte_suggested_palette_in_rgb().subrange(667,1071).take(320),bytes_plte_suggested_palette_in_rgb().subrange(667,1071).subrange(320,384));
    assert(bytes_plte_suggested_palette_in_rgb().subrange(667,1071).take(320)+bytes_plte_suggested_palette_in_rgb().subrange(667,1071).subrange(320,384) =~= bytes_plte_suggested_palette_in_rgb().subrange(667,1071).take(384));
    assert(crate::model::crc_register(bytes_plte_suggested_palette_in_rgb().subrange(667,1071).take(384))==2239207243u32);
    assert(crc_cert::fold(2239207243u32,bytes_plte_suggested_palette_in_rgb().subrange(667,1071).subrange(384,404))==2726554167u32) by (compute);
    crc_cert::suffix(bytes_plte_suggested_palette_in_rgb().subrange(667,1071).take(384),bytes_plte_suggested_palette_in_rgb().subrange(667,1071).subrange(384,404));
    assert(bytes_plte_suggested_palette_in_rgb().subrange(667,1071).take(384)+bytes_plte_suggested_palette_in_rgb().subrange(667,1071).subrange(384,404) =~= bytes_plte_suggested_palette_in_rgb().subrange(667,1071).take(404));
    assert(crate::model::crc_register(bytes_plte_suggested_palette_in_rgb().subrange(667,1071).take(404))==2726554167u32);
    assert(bytes_plte_suggested_palette_in_rgb().subrange(667,1071).take(404) =~= bytes_plte_suggested_palette_in_rgb().subrange(667,1071));
    assert((2726554167u32 ^ 0xffff_ffffu32)==1568413128u32) by (compute);
}
#[verifier::spinoff_prover]
proof fn bkgd_rgb_certificate(d: Seq<u8>)
    requires d.len()==6,crate::metadata_model::words16(d,3)==seq![1u16,2u16,3u16],
    ensures crate::metadata_model::parse(crate::walk_model::Name::Bkgd,d,2u8,8u8,200nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Bkgd(seq![1u16,2u16,3u16])),
{
    let samples=crate::metadata_model::words16(d,3);
    assert forall|i: int| 0<=i<samples.len() implies (#[trigger] samples[i] as nat)<256nat by {if i==0 {}else if i==1 {}else {assert(i==2);}}
    assert((1u32<<8u8)==256u32) by (compute);
}
spec fn plte_rgb_state_0() -> m::State {m::initial(h_plte_suggested_palette_in_rgb(),33,m::no_events())}
spec fn plte_rgb_state_1() -> m::State {let s=plte_rgb_state_0();m::State {pos:645nat,seen:s.seen.insert(crate::walk_model::Name::Plte),doc:m::Doc {plte:Some(crate::walk_model::Range {offset:41nat,len:600nat}),info:m::Info {palette_entries:200nat,..s.doc.info},..s.doc},..s}}
spec fn plte_rgb_state_2() -> m::State {let s=plte_rgb_state_1();m::State {pos:663nat,seen:s.seen.insert(crate::walk_model::Name::Bkgd),doc:m::store(s.doc,crate::metadata_model::Value::Bkgd(seq![1u16,2u16,3u16]),653nat),..s}}
spec fn plte_rgb_state_3() -> m::State {let s=plte_rgb_state_2();m::State {pos:1075nat,seen:s.seen.insert(crate::walk_model::Name::Hist),..s}}
spec fn plte_rgb_state_4() -> m::State {let s=plte_rgb_state_3();m::State {pos:1109nat,phase:crate::walk_model::Phase::During,doc:m::Doc {idat:seq![crate::walk_model::Range {offset:1083nat,len:22nat}],..s.doc},..s}}
#[verifier::spinoff_prover]
proof fn plte_rgb_step_0(b: Seq<u8>)
    requires b.len()==1121,
        crate::model::mh(b)==Ok((h_plte_suggested_palette_in_rgb(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:600nat,next:645nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,645nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![98u8,75u8,71u8,68u8],data:653nat,len:6nat,next:663nat,crc_ok:true,synthetic:false})),
        b.subrange(653,659)[0]==0u8,
        b.subrange(653,659)[1]==1u8,
        b.subrange(653,659)[2]==0u8,
        b.subrange(653,659)[3]==2u8,
        b.subrange(653,659)[4]==0u8,
        b.subrange(653,659)[5]==3u8,
        crate::metadata_model::words16(b.subrange(653,659),3nat).len()==3,
        crate::model::be16(b.subrange(653,659),0)==1u16,
        crate::metadata_model::words16(b.subrange(653,659),3nat)[0]==1u16,
        crate::model::be16(b.subrange(653,659),2)==2u16,
        crate::metadata_model::words16(b.subrange(653,659),3nat)[1]==2u16,
        crate::model::be16(b.subrange(653,659),4)==3u16,
        crate::metadata_model::words16(b.subrange(653,659),3nat)[2]==3u16,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,663nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![104u8,73u8,83u8,84u8],data:671nat,len:400nat,next:1075nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,1075nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:1083nat,len:22nat,next:1109nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,1109nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:1117nat,len:0nat,next:1121nat,crc_ok:true,synthetic:false})),
        (forall|i: int| 0<=i<3 ==> (#[trigger] crate::metadata_model::words16(b.subrange(653,659),3)[i] as nat)<(1u32<<8u8) as nat),
        crate::metadata_model::parse(crate::walk_model::Name::Bkgd,b.subrange(653,659),2u8,8u8,200nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Bkgd(crate::metadata_model::words16(b.subrange(653,659),3))),
        crate::metadata_model::parse(crate::walk_model::Name::Hist,b.subrange(671,1071),2u8,8u8,200nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored),
        crate::metadata_model::words16(b.subrange(653,659),3nat)=~=seq![1u16,2u16,3u16],
        crate::metadata_model::parse(crate::walk_model::Name::Bkgd,b.subrange(653,659),2u8,8u8,200nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Bkgd(seq![1u16,2u16,3u16])),
        crate::metadata_model::parse(crate::walk_model::Name::Hist,b.subrange(671,1071),2u8,8u8,200nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored),
        b.subrange(1083,1105)==z_plte_suggested_palette_in_rgb(),
    ensures m::step(b,m::strict(),false,false,plte_rgb_state_0())==m::Step::Next(plte_rgb_state_1()),
{
    hide(crate::metadata_model::parse);hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(600nat,2u8,8u8,false,false,false,false)==crate::metadata_model::Decision::Keep(200nat)) by (compute);
        assert((98u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![98u8,75u8,71u8,68u8])==crate::walk_model::Name::Bkgd) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![98u8,75u8,71u8,68u8])==crate::walk_model::Name::Bkgd) by (compute);
        }
        assert((104u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![104u8,73u8,83u8,84u8])==crate::walk_model::Name::Hist) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![104u8,73u8,83u8,84u8])==crate::walk_model::Name::Hist) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
}
#[verifier::spinoff_prover]
proof fn plte_rgb_step_1(b: Seq<u8>)
    requires b.len()==1121,
        crate::model::mh(b)==Ok((h_plte_suggested_palette_in_rgb(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:600nat,next:645nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,645nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![98u8,75u8,71u8,68u8],data:653nat,len:6nat,next:663nat,crc_ok:true,synthetic:false})),
        b.subrange(653,659)[0]==0u8,
        b.subrange(653,659)[1]==1u8,
        b.subrange(653,659)[2]==0u8,
        b.subrange(653,659)[3]==2u8,
        b.subrange(653,659)[4]==0u8,
        b.subrange(653,659)[5]==3u8,
        crate::metadata_model::words16(b.subrange(653,659),3nat).len()==3,
        crate::model::be16(b.subrange(653,659),0)==1u16,
        crate::metadata_model::words16(b.subrange(653,659),3nat)[0]==1u16,
        crate::model::be16(b.subrange(653,659),2)==2u16,
        crate::metadata_model::words16(b.subrange(653,659),3nat)[1]==2u16,
        crate::model::be16(b.subrange(653,659),4)==3u16,
        crate::metadata_model::words16(b.subrange(653,659),3nat)[2]==3u16,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,663nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![104u8,73u8,83u8,84u8],data:671nat,len:400nat,next:1075nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,1075nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:1083nat,len:22nat,next:1109nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,1109nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:1117nat,len:0nat,next:1121nat,crc_ok:true,synthetic:false})),
        (forall|i: int| 0<=i<3 ==> (#[trigger] crate::metadata_model::words16(b.subrange(653,659),3)[i] as nat)<(1u32<<8u8) as nat),
        crate::metadata_model::parse(crate::walk_model::Name::Bkgd,b.subrange(653,659),2u8,8u8,200nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Bkgd(crate::metadata_model::words16(b.subrange(653,659),3))),
        crate::metadata_model::parse(crate::walk_model::Name::Hist,b.subrange(671,1071),2u8,8u8,200nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored),
        crate::metadata_model::words16(b.subrange(653,659),3nat)=~=seq![1u16,2u16,3u16],
        crate::metadata_model::parse(crate::walk_model::Name::Bkgd,b.subrange(653,659),2u8,8u8,200nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Bkgd(seq![1u16,2u16,3u16])),
        crate::metadata_model::parse(crate::walk_model::Name::Hist,b.subrange(671,1071),2u8,8u8,200nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored),
        b.subrange(1083,1105)==z_plte_suggested_palette_in_rgb(),
    ensures m::step(b,m::strict(),false,false,plte_rgb_state_1())==m::Step::Next(plte_rgb_state_2()),
{
    hide(crate::metadata_model::parse);hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(600nat,2u8,8u8,false,false,false,false)==crate::metadata_model::Decision::Keep(200nat)) by (compute);
        assert((98u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![98u8,75u8,71u8,68u8])==crate::walk_model::Name::Bkgd) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![98u8,75u8,71u8,68u8])==crate::walk_model::Name::Bkgd) by (compute);
        }
        assert((104u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![104u8,73u8,83u8,84u8])==crate::walk_model::Name::Hist) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![104u8,73u8,83u8,84u8])==crate::walk_model::Name::Hist) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
}
#[verifier::spinoff_prover]
proof fn plte_rgb_step_2(b: Seq<u8>)
    requires b.len()==1121,
        crate::model::mh(b)==Ok((h_plte_suggested_palette_in_rgb(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:600nat,next:645nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,645nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![98u8,75u8,71u8,68u8],data:653nat,len:6nat,next:663nat,crc_ok:true,synthetic:false})),
        b.subrange(653,659)[0]==0u8,
        b.subrange(653,659)[1]==1u8,
        b.subrange(653,659)[2]==0u8,
        b.subrange(653,659)[3]==2u8,
        b.subrange(653,659)[4]==0u8,
        b.subrange(653,659)[5]==3u8,
        crate::metadata_model::words16(b.subrange(653,659),3nat).len()==3,
        crate::model::be16(b.subrange(653,659),0)==1u16,
        crate::metadata_model::words16(b.subrange(653,659),3nat)[0]==1u16,
        crate::model::be16(b.subrange(653,659),2)==2u16,
        crate::metadata_model::words16(b.subrange(653,659),3nat)[1]==2u16,
        crate::model::be16(b.subrange(653,659),4)==3u16,
        crate::metadata_model::words16(b.subrange(653,659),3nat)[2]==3u16,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,663nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![104u8,73u8,83u8,84u8],data:671nat,len:400nat,next:1075nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,1075nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:1083nat,len:22nat,next:1109nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,1109nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:1117nat,len:0nat,next:1121nat,crc_ok:true,synthetic:false})),
        (forall|i: int| 0<=i<3 ==> (#[trigger] crate::metadata_model::words16(b.subrange(653,659),3)[i] as nat)<(1u32<<8u8) as nat),
        crate::metadata_model::parse(crate::walk_model::Name::Bkgd,b.subrange(653,659),2u8,8u8,200nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Bkgd(crate::metadata_model::words16(b.subrange(653,659),3))),
        crate::metadata_model::parse(crate::walk_model::Name::Hist,b.subrange(671,1071),2u8,8u8,200nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored),
        crate::metadata_model::words16(b.subrange(653,659),3nat)=~=seq![1u16,2u16,3u16],
        crate::metadata_model::parse(crate::walk_model::Name::Bkgd,b.subrange(653,659),2u8,8u8,200nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Bkgd(seq![1u16,2u16,3u16])),
        crate::metadata_model::parse(crate::walk_model::Name::Hist,b.subrange(671,1071),2u8,8u8,200nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored),
        b.subrange(1083,1105)==z_plte_suggested_palette_in_rgb(),
    ensures m::step(b,m::strict(),false,false,plte_rgb_state_2())==m::Step::Next(plte_rgb_state_3()),
{
    hide(crate::metadata_model::parse);hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(600nat,2u8,8u8,false,false,false,false)==crate::metadata_model::Decision::Keep(200nat)) by (compute);
        assert((98u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![98u8,75u8,71u8,68u8])==crate::walk_model::Name::Bkgd) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![98u8,75u8,71u8,68u8])==crate::walk_model::Name::Bkgd) by (compute);
        }
        assert((104u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![104u8,73u8,83u8,84u8])==crate::walk_model::Name::Hist) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![104u8,73u8,83u8,84u8])==crate::walk_model::Name::Hist) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
}
#[verifier::spinoff_prover]
proof fn plte_rgb_step_3(b: Seq<u8>)
    requires b.len()==1121,
        crate::model::mh(b)==Ok((h_plte_suggested_palette_in_rgb(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:600nat,next:645nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,645nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![98u8,75u8,71u8,68u8],data:653nat,len:6nat,next:663nat,crc_ok:true,synthetic:false})),
        b.subrange(653,659)[0]==0u8,
        b.subrange(653,659)[1]==1u8,
        b.subrange(653,659)[2]==0u8,
        b.subrange(653,659)[3]==2u8,
        b.subrange(653,659)[4]==0u8,
        b.subrange(653,659)[5]==3u8,
        crate::metadata_model::words16(b.subrange(653,659),3nat).len()==3,
        crate::model::be16(b.subrange(653,659),0)==1u16,
        crate::metadata_model::words16(b.subrange(653,659),3nat)[0]==1u16,
        crate::model::be16(b.subrange(653,659),2)==2u16,
        crate::metadata_model::words16(b.subrange(653,659),3nat)[1]==2u16,
        crate::model::be16(b.subrange(653,659),4)==3u16,
        crate::metadata_model::words16(b.subrange(653,659),3nat)[2]==3u16,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,663nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![104u8,73u8,83u8,84u8],data:671nat,len:400nat,next:1075nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,1075nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:1083nat,len:22nat,next:1109nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,1109nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:1117nat,len:0nat,next:1121nat,crc_ok:true,synthetic:false})),
        (forall|i: int| 0<=i<3 ==> (#[trigger] crate::metadata_model::words16(b.subrange(653,659),3)[i] as nat)<(1u32<<8u8) as nat),
        crate::metadata_model::parse(crate::walk_model::Name::Bkgd,b.subrange(653,659),2u8,8u8,200nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Bkgd(crate::metadata_model::words16(b.subrange(653,659),3))),
        crate::metadata_model::parse(crate::walk_model::Name::Hist,b.subrange(671,1071),2u8,8u8,200nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored),
        crate::metadata_model::words16(b.subrange(653,659),3nat)=~=seq![1u16,2u16,3u16],
        crate::metadata_model::parse(crate::walk_model::Name::Bkgd,b.subrange(653,659),2u8,8u8,200nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Bkgd(seq![1u16,2u16,3u16])),
        crate::metadata_model::parse(crate::walk_model::Name::Hist,b.subrange(671,1071),2u8,8u8,200nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored),
        b.subrange(1083,1105)==z_plte_suggested_palette_in_rgb(),
    ensures m::step(b,m::strict(),false,false,plte_rgb_state_3())==m::Step::Next(plte_rgb_state_4()),
{
    hide(crate::metadata_model::parse);hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(600nat,2u8,8u8,false,false,false,false)==crate::metadata_model::Decision::Keep(200nat)) by (compute);
        assert((98u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![98u8,75u8,71u8,68u8])==crate::walk_model::Name::Bkgd) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![98u8,75u8,71u8,68u8])==crate::walk_model::Name::Bkgd) by (compute);
        }
        assert((104u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![104u8,73u8,83u8,84u8])==crate::walk_model::Name::Hist) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![104u8,73u8,83u8,84u8])==crate::walk_model::Name::Hist) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
}
#[verifier::spinoff_prover]
proof fn plte_rgb_step_4(b: Seq<u8>)
    requires b.len()==1121,
        crate::model::mh(b)==Ok((h_plte_suggested_palette_in_rgb(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:600nat,next:645nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,645nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![98u8,75u8,71u8,68u8],data:653nat,len:6nat,next:663nat,crc_ok:true,synthetic:false})),
        b.subrange(653,659)[0]==0u8,
        b.subrange(653,659)[1]==1u8,
        b.subrange(653,659)[2]==0u8,
        b.subrange(653,659)[3]==2u8,
        b.subrange(653,659)[4]==0u8,
        b.subrange(653,659)[5]==3u8,
        crate::metadata_model::words16(b.subrange(653,659),3nat).len()==3,
        crate::model::be16(b.subrange(653,659),0)==1u16,
        crate::metadata_model::words16(b.subrange(653,659),3nat)[0]==1u16,
        crate::model::be16(b.subrange(653,659),2)==2u16,
        crate::metadata_model::words16(b.subrange(653,659),3nat)[1]==2u16,
        crate::model::be16(b.subrange(653,659),4)==3u16,
        crate::metadata_model::words16(b.subrange(653,659),3nat)[2]==3u16,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,663nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![104u8,73u8,83u8,84u8],data:671nat,len:400nat,next:1075nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,1075nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:1083nat,len:22nat,next:1109nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,1109nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:1117nat,len:0nat,next:1121nat,crc_ok:true,synthetic:false})),
        (forall|i: int| 0<=i<3 ==> (#[trigger] crate::metadata_model::words16(b.subrange(653,659),3)[i] as nat)<(1u32<<8u8) as nat),
        crate::metadata_model::parse(crate::walk_model::Name::Bkgd,b.subrange(653,659),2u8,8u8,200nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Bkgd(crate::metadata_model::words16(b.subrange(653,659),3))),
        crate::metadata_model::parse(crate::walk_model::Name::Hist,b.subrange(671,1071),2u8,8u8,200nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored),
        crate::metadata_model::words16(b.subrange(653,659),3nat)=~=seq![1u16,2u16,3u16],
        crate::metadata_model::parse(crate::walk_model::Name::Bkgd,b.subrange(653,659),2u8,8u8,200nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Bkgd(seq![1u16,2u16,3u16])),
        crate::metadata_model::parse(crate::walk_model::Name::Hist,b.subrange(671,1071),2u8,8u8,200nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored),
        b.subrange(1083,1105)==z_plte_suggested_palette_in_rgb(),
    ensures m::step(b,m::strict(),false,false,plte_rgb_state_4())==m::Step::Done(m::Doc {info:m::Info {frame_count:0nat,..plte_rgb_state_4().doc.info},..plte_rgb_state_4().doc},m::no_events()),
{
    hide(crate::metadata_model::parse);hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(600nat,2u8,8u8,false,false,false,false)==crate::metadata_model::Decision::Keep(200nat)) by (compute);
        assert((98u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![98u8,75u8,71u8,68u8])==crate::walk_model::Name::Bkgd) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![98u8,75u8,71u8,68u8])==crate::walk_model::Name::Bkgd) by (compute);
        }
        assert((104u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![104u8,73u8,83u8,84u8])==crate::walk_model::Name::Hist) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![104u8,73u8,83u8,84u8])==crate::walk_model::Name::Hist) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
}
#[verifier::spinoff_prover]
proof fn walk_plte_suggested_palette_in_rgb(b: Seq<u8>)
    requires b.len()==1121,
        crate::model::mh(b)==Ok((h_plte_suggested_palette_in_rgb(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:600nat,next:645nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,645nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![98u8,75u8,71u8,68u8],data:653nat,len:6nat,next:663nat,crc_ok:true,synthetic:false})),
        b.subrange(653,659)[0]==0u8,
        b.subrange(653,659)[1]==1u8,
        b.subrange(653,659)[2]==0u8,
        b.subrange(653,659)[3]==2u8,
        b.subrange(653,659)[4]==0u8,
        b.subrange(653,659)[5]==3u8,
        crate::metadata_model::words16(b.subrange(653,659),3nat).len()==3,
        crate::model::be16(b.subrange(653,659),0)==1u16,
        crate::metadata_model::words16(b.subrange(653,659),3nat)[0]==1u16,
        crate::model::be16(b.subrange(653,659),2)==2u16,
        crate::metadata_model::words16(b.subrange(653,659),3nat)[1]==2u16,
        crate::model::be16(b.subrange(653,659),4)==3u16,
        crate::metadata_model::words16(b.subrange(653,659),3nat)[2]==3u16,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,663nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![104u8,73u8,83u8,84u8],data:671nat,len:400nat,next:1075nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,1075nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:1083nat,len:22nat,next:1109nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,1109nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:1117nat,len:0nat,next:1121nat,crc_ok:true,synthetic:false})),
        (forall|i: int| 0<=i<3 ==> (#[trigger] crate::metadata_model::words16(b.subrange(653,659),3)[i] as nat)<(1u32<<8u8) as nat),
        crate::metadata_model::parse(crate::walk_model::Name::Bkgd,b.subrange(653,659),2u8,8u8,200nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Bkgd(crate::metadata_model::words16(b.subrange(653,659),3))),
        crate::metadata_model::parse(crate::walk_model::Name::Hist,b.subrange(671,1071),2u8,8u8,200nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored),
        crate::metadata_model::words16(b.subrange(653,659),3nat)=~=seq![1u16,2u16,3u16],
        crate::metadata_model::parse(crate::walk_model::Name::Bkgd,b.subrange(653,659),2u8,8u8,200nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Bkgd(seq![1u16,2u16,3u16])),
        crate::metadata_model::parse(crate::walk_model::Name::Hist,b.subrange(671,1071),2u8,8u8,200nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored),
        b.subrange(1083,1105)==z_plte_suggested_palette_in_rgb(),
    ensures r::default_case(b,h_plte_suggested_palette_in_rgb(),z_plte_suggested_palette_in_rgb(),200nat),match m::MW_p(b,m::strict()) {m::Outcome::Done(doc,_)=>doc.plte==Some(crate::walk_model::Range {offset:41nat,len:600nat}) && doc.trns==None,_=>false},
{
    hide(m::drive);hide(crate::critical_model::read);hide(crate::metadata_model::parse);hide(crate::model::mh);
    hide(m::store);hide(crate::metadata_model::parse);
    assert forall|doc: m::Doc,value: crate::metadata_model::Value,base: nat|
        (#[trigger] m::store(doc,value,base)).ihdr==doc.ihdr && m::store(doc,value,base).plte==doc.plte
        && m::store(doc,value,base).trns==doc.trns && m::store(doc,value,base).actl==doc.actl
        && m::store(doc,value,base).idat==doc.idat && m::store(doc,value,base).frames==doc.frames by {
        stored_structure(doc,value,base);
    }
    plte_rgb_step_0(b);
    plte_rgb_step_1(b);
    plte_rgb_step_2(b);
    plte_rgb_step_3(b);
    plte_rgb_step_4(b);
    m::drive_step(b,m::strict(),false,false,plte_rgb_state_4());
    m::drive_step(b,m::strict(),false,false,plte_rgb_state_3());
    m::drive_step(b,m::strict(),false,false,plte_rgb_state_2());
    m::drive_step(b,m::strict(),false,false,plte_rgb_state_1());
    m::drive_step(b,m::strict(),false,false,plte_rgb_state_0());
    reveal_with_fuel(m::concat_ranges,3);
}
#[verifier::spinoff_prover]
proof fn scan_plte_suggested_palette_in_rgb() ensures r::scan(raw_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),0)==Ok(()),
{
    hide(raw_plte_suggested_palette_in_rgb);assert(raw_plte_suggested_palette_in_rgb().len()==14nat) by (compute);assert(r::count(a_plte_suggested_palette_in_rgb())==1nat) by (compute);
    assert(r::pw(a_plte_suggested_palette_in_rgb(),0nat)==2nat && r::ph(a_plte_suggested_palette_in_rgb(),0nat)==2nat && r::rb(a_plte_suggested_palette_in_rgb(),0nat)==6nat && r::base(a_plte_suggested_palette_in_rgb(),0nat)==0nat) by (compute);
    assert(raw_plte_suggested_palette_in_rgb()[0]==0u8) by (compute);
    assert(r::row_result(raw_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),0nat,0nat)==Ok(()));
    assert(raw_plte_suggested_palette_in_rgb()[7]==1u8) by (compute);
    assert(r::row_result(raw_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),0nat,1nat)==Ok(()));
    r::scan_step(raw_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),1nat);
    r::pass_step(raw_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),0nat,2nat);
    r::pass_step(raw_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),0nat,1nat);
    r::pass_step(raw_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),0nat,0nat);
    r::scan_step(raw_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),0nat);
}
#[verifier::spinoff_prover]
proof fn row_plte_suggested_palette_in_rgb_0_0() ensures r::line(raw_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),0nat,0nat)==seq![201u8,73u8,63u8,202u8,75u8,151u8],
{
    hide(raw_plte_suggested_palette_in_rgb);
    assert(raw_plte_suggested_palette_in_rgb().len()==14nat) by (compute);
    assert(raw_plte_suggested_palette_in_rgb()[0]==0u8) by (compute);
    assert(r::base(a_plte_suggested_palette_in_rgb(),0nat)==0nat && r::rb(a_plte_suggested_palette_in_rgb(),0nat)==6nat && f::bpp(r::bits(a_plte_suggested_palette_in_rgb()))==3nat) by (compute);
    assert(f::row_data(raw_plte_suggested_palette_in_rgb(),0nat,6nat,0nat)==seq![201u8,73u8,63u8,202u8,75u8,151u8]) by (compute);
    f::pass_row_equation(raw_plte_suggested_palette_in_rgb(),0nat,6nat,3nat,0nat);
    assert(f::previous_row(raw_plte_suggested_palette_in_rgb(),0nat,6nat,3nat,0nat)==None);
    assert(f::encoded_row(0u8,f::row_data(raw_plte_suggested_palette_in_rgb(),0nat,6nat,0nat),None,3nat,seq![201u8,73u8,63u8,202u8,75u8,151u8],0,6nat)) by (compute);
    f::checked_row_hint(0u8,f::row_data(raw_plte_suggested_palette_in_rgb(),0nat,6nat,0nat),None,3nat,seq![201u8,73u8,63u8,202u8,75u8,151u8]);
}
#[verifier::spinoff_prover]
proof fn row_plte_suggested_palette_in_rgb_0_1() ensures r::line(raw_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),0nat,1nat)==seq![225u8,26u8,23u8,252u8,246u8,129u8],
{
    hide(raw_plte_suggested_palette_in_rgb);
    assert(raw_plte_suggested_palette_in_rgb().len()==14nat) by (compute);
    assert(raw_plte_suggested_palette_in_rgb()[7]==1u8) by (compute);
    assert(r::base(a_plte_suggested_palette_in_rgb(),0nat)==0nat && r::rb(a_plte_suggested_palette_in_rgb(),0nat)==6nat && f::bpp(r::bits(a_plte_suggested_palette_in_rgb()))==3nat) by (compute);
    assert(f::row_data(raw_plte_suggested_palette_in_rgb(),0nat,6nat,1nat)==seq![225u8,26u8,23u8,27u8,220u8,106u8]) by (compute);
    row_plte_suggested_palette_in_rgb_0_0();
    f::pass_row_equation(raw_plte_suggested_palette_in_rgb(),0nat,6nat,3nat,1nat);
    assert(f::previous_row(raw_plte_suggested_palette_in_rgb(),0nat,6nat,3nat,1nat)==Some(seq![201u8,73u8,63u8,202u8,75u8,151u8]));
    assert(f::encoded_row(1u8,f::row_data(raw_plte_suggested_palette_in_rgb(),0nat,6nat,1nat),Some(seq![201u8,73u8,63u8,202u8,75u8,151u8]),3nat,seq![225u8,26u8,23u8,252u8,246u8,129u8],0,6nat)) by (compute);
    f::checked_row_hint(1u8,f::row_data(raw_plte_suggested_palette_in_rgb(),0nat,6nat,1nat),Some(seq![201u8,73u8,63u8,202u8,75u8,151u8]),3nat,seq![225u8,26u8,23u8,252u8,246u8,129u8]);
}
#[verifier::spinoff_prover]
proof fn raster_plte_suggested_palette_in_rgb()
    ensures r::default_case(bytes_plte_suggested_palette_in_rgb(),h_plte_suggested_palette_in_rgb(),z_plte_suggested_palette_in_rgb(),200nat),match m::MW_p(bytes_plte_suggested_palette_in_rgb(),m::strict()) {m::Outcome::Done(doc,_)=>doc.plte==Some(crate::walk_model::Range {offset:41nat,len:600nat}) && doc.trns==None,_=>false},r::data(z_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),STRICT)==Some(Ok(())),
{
    hide(bytes_plte_suggested_palette_in_rgb);hide(z_plte_suggested_palette_in_rgb);hide(raw_plte_suggested_palette_in_rgb);hide(r::data);
    hide(crate::metadata_model::parse);hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    assert(bytes_plte_suggested_palette_in_rgb().len()==1121nat) by (compute);
    assert(crate::model::mh(bytes_plte_suggested_palette_in_rgb())==Ok((h_plte_suggested_palette_in_rgb(),33nat))) by {reveal(crate::model::mh);assert(crate::model::mh(bytes_plte_suggested_palette_in_rgb())==Ok((h_plte_suggested_palette_in_rgb(),33nat))) by (compute);}
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_suggested_palette_in_rgb(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:600nat,next:645nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::model::frame(bytes_plte_suggested_palette_in_rgb(),33nat)==Ok(crate::model::WireChunk {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:600nat,next:645nat})) by (compute);
            assert(crate::model::be32(bytes_plte_suggested_palette_in_rgb(),641)==3591662554u32) by (compute);
            crc_plte_suggested_palette_in_rgb_crc_33();
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(600nat,2u8,8u8,false,false,false,false)==crate::metadata_model::Decision::Keep(200nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_suggested_palette_in_rgb(),645nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![98u8,75u8,71u8,68u8],data:653nat,len:6nat,next:663nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_plte_suggested_palette_in_rgb(),645nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![98u8,75u8,71u8,68u8],data:653nat,len:6nat,next:663nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((98u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![98u8,75u8,71u8,68u8])==crate::walk_model::Name::Bkgd) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![98u8,75u8,71u8,68u8])==crate::walk_model::Name::Bkgd) by (compute);
        }
        assert(bytes_plte_suggested_palette_in_rgb().subrange(653,659)[0]==0u8) by (compute);
        assert(bytes_plte_suggested_palette_in_rgb().subrange(653,659)[1]==1u8) by (compute);
        assert(bytes_plte_suggested_palette_in_rgb().subrange(653,659)[2]==0u8) by (compute);
        assert(bytes_plte_suggested_palette_in_rgb().subrange(653,659)[3]==2u8) by (compute);
        assert(bytes_plte_suggested_palette_in_rgb().subrange(653,659)[4]==0u8) by (compute);
        assert(bytes_plte_suggested_palette_in_rgb().subrange(653,659)[5]==3u8) by (compute);
        assert(crate::metadata_model::words16(bytes_plte_suggested_palette_in_rgb().subrange(653,659),3nat).len()==3);
        assert(crate::model::be16(bytes_plte_suggested_palette_in_rgb().subrange(653,659),0)==1u16) by (compute);
        assert(crate::metadata_model::words16(bytes_plte_suggested_palette_in_rgb().subrange(653,659),3nat)[0]==1u16);
        assert(crate::model::be16(bytes_plte_suggested_palette_in_rgb().subrange(653,659),2)==2u16) by (compute);
        assert(crate::metadata_model::words16(bytes_plte_suggested_palette_in_rgb().subrange(653,659),3nat)[1]==2u16);
        assert(crate::model::be16(bytes_plte_suggested_palette_in_rgb().subrange(653,659),4)==3u16) by (compute);
        assert(crate::metadata_model::words16(bytes_plte_suggested_palette_in_rgb().subrange(653,659),3nat)[2]==3u16);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_suggested_palette_in_rgb(),663nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![104u8,73u8,83u8,84u8],data:671nat,len:400nat,next:1075nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::model::frame(bytes_plte_suggested_palette_in_rgb(),663nat)==Ok(crate::model::WireChunk {ty:seq![104u8,73u8,83u8,84u8],data:671nat,len:400nat,next:1075nat})) by (compute);
            assert(crate::model::be32(bytes_plte_suggested_palette_in_rgb(),1071)==1568413128u32) by (compute);
            crc_plte_suggested_palette_in_rgb_crc_663();
        }
        assert((104u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![104u8,73u8,83u8,84u8])==crate::walk_model::Name::Hist) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![104u8,73u8,83u8,84u8])==crate::walk_model::Name::Hist) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_suggested_palette_in_rgb(),1075nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:1083nat,len:22nat,next:1109nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_plte_suggested_palette_in_rgb(),1075nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:1083nat,len:22nat,next:1109nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_suggested_palette_in_rgb(),1109nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:1117nat,len:0nat,next:1121nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_plte_suggested_palette_in_rgb(),1109nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:1117nat,len:0nat,next:1121nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert forall|i: int| 0<=i<3 implies (#[trigger] crate::metadata_model::words16(bytes_plte_suggested_palette_in_rgb().subrange(653,659),3)[i] as nat)<(1u32<<8u8) as nat by {
            if i==0 {assert(crate::metadata_model::words16(bytes_plte_suggested_palette_in_rgb().subrange(653,659),3)[i]==1u16);}
            else if i==1 {assert(crate::metadata_model::words16(bytes_plte_suggested_palette_in_rgb().subrange(653,659),3)[i]==2u16);}
            else if i==2 {assert(crate::metadata_model::words16(bytes_plte_suggested_palette_in_rgb().subrange(653,659),3)[i]==3u16);}
        }
        assert(crate::metadata_model::parse(crate::walk_model::Name::Bkgd,bytes_plte_suggested_palette_in_rgb().subrange(653,659),2u8,8u8,200nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Bkgd(crate::metadata_model::words16(bytes_plte_suggested_palette_in_rgb().subrange(653,659),3)))) by {
            reveal(crate::metadata_model::parse);
            rgb_background_success(bytes_plte_suggested_palette_in_rgb().subrange(653,659),200nat);
        }
        assert(crate::metadata_model::parse(crate::walk_model::Name::Hist,bytes_plte_suggested_palette_in_rgb().subrange(671,1071),2u8,8u8,200nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored)) by {
            reveal(crate::metadata_model::parse);
            assert(crate::metadata_model::parse(crate::walk_model::Name::Hist,bytes_plte_suggested_palette_in_rgb().subrange(671,1071),2u8,8u8,200nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored)) by (compute);
        }
        assert(crate::metadata_model::words16(bytes_plte_suggested_palette_in_rgb().subrange(653,659),3nat)=~=seq![1u16,2u16,3u16]);
        assert(crate::metadata_model::parse(crate::walk_model::Name::Bkgd,bytes_plte_suggested_palette_in_rgb().subrange(653,659),2u8,8u8,200nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Bkgd(seq![1u16,2u16,3u16]))) by {bkgd_rgb_certificate(bytes_plte_suggested_palette_in_rgb().subrange(653,659));};
        assert(crate::metadata_model::parse(crate::walk_model::Name::Hist,bytes_plte_suggested_palette_in_rgb().subrange(671,1071),2u8,8u8,200nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored)) by {reveal(crate::metadata_model::parse);};
    assert(bytes_plte_suggested_palette_in_rgb().subrange(1083,1105)==z_plte_suggested_palette_in_rgb()) by (compute);
    walk_plte_suggested_palette_in_rgb(bytes_plte_suggested_palette_in_rgb());
    inflate_plte_suggested_palette_in_rgb();scan_plte_suggested_palette_in_rgb();
    assert(r::raw(a_plte_suggested_palette_in_rgb())==14nat) by (compute);
    assert(r::data(z_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),STRICT)==Some(Ok(()))) by {reveal(r::data);};
}
#[verifier::spinoff_prover]
proof fn pixel_plte_suggested_palette_in_rgb_0() ensures sm::image(raw_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),colour_plte_suggested_palette_in_rgb(),false)[0]==201u8,sm::image(raw_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),colour_plte_suggested_palette_in_rgb(),false)[1]==73u8,sm::image(raw_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),colour_plte_suggested_palette_in_rgb(),false)[2]==63u8,sm::image(raw_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),colour_plte_suggested_palette_in_rgb(),false)[3]==255u8,
{
    colour::depth_values(8u8);
    row_plte_suggested_palette_in_rgb_0_0();
    assert(crate::paint_model::source_cell(a_plte_suggested_palette_in_rgb(),0nat,0nat)==(0nat,0nat,0nat)) by (compute);
    assert(colour::pixel(colour_plte_suggested_palette_in_rgb(),seq![201u8,73u8,63u8,202u8,75u8,151u8],0nat,true,false)==Ok(seq![51657u16,18761u16,16191u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),colour_plte_suggested_palette_in_rgb(),0nat,0nat,true,false)==Ok(seq![51657u16,18761u16,16191u16,65535u16]));
    assert(((seq![51657u16,18761u16,16191u16,65535u16][0] as nat+128)/257) as u8==201u8) by (compute);
    assert(((seq![51657u16,18761u16,16191u16,65535u16][1] as nat+128)/257) as u8==73u8) by (compute);
    assert(((seq![51657u16,18761u16,16191u16,65535u16][2] as nat+128)/257) as u8==63u8) by (compute);
    assert(((seq![51657u16,18761u16,16191u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixel_plte_suggested_palette_in_rgb_1() ensures sm::image(raw_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),colour_plte_suggested_palette_in_rgb(),false)[4]==202u8,sm::image(raw_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),colour_plte_suggested_palette_in_rgb(),false)[5]==75u8,sm::image(raw_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),colour_plte_suggested_palette_in_rgb(),false)[6]==151u8,sm::image(raw_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),colour_plte_suggested_palette_in_rgb(),false)[7]==255u8,
{
    colour::depth_values(8u8);
    row_plte_suggested_palette_in_rgb_0_0();
    assert(crate::paint_model::source_cell(a_plte_suggested_palette_in_rgb(),1nat,0nat)==(0nat,1nat,0nat)) by (compute);
    assert(colour::pixel(colour_plte_suggested_palette_in_rgb(),seq![201u8,73u8,63u8,202u8,75u8,151u8],1nat,true,false)==Ok(seq![51914u16,19275u16,38807u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),colour_plte_suggested_palette_in_rgb(),1nat,0nat,true,false)==Ok(seq![51914u16,19275u16,38807u16,65535u16]));
    assert(((seq![51914u16,19275u16,38807u16,65535u16][0] as nat+128)/257) as u8==202u8) by (compute);
    assert(((seq![51914u16,19275u16,38807u16,65535u16][1] as nat+128)/257) as u8==75u8) by (compute);
    assert(((seq![51914u16,19275u16,38807u16,65535u16][2] as nat+128)/257) as u8==151u8) by (compute);
    assert(((seq![51914u16,19275u16,38807u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixel_plte_suggested_palette_in_rgb_2() ensures sm::image(raw_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),colour_plte_suggested_palette_in_rgb(),false)[8]==225u8,sm::image(raw_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),colour_plte_suggested_palette_in_rgb(),false)[9]==26u8,sm::image(raw_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),colour_plte_suggested_palette_in_rgb(),false)[10]==23u8,sm::image(raw_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),colour_plte_suggested_palette_in_rgb(),false)[11]==255u8,
{
    colour::depth_values(8u8);
    row_plte_suggested_palette_in_rgb_0_1();
    assert(crate::paint_model::source_cell(a_plte_suggested_palette_in_rgb(),0nat,1nat)==(0nat,0nat,1nat)) by (compute);
    assert(colour::pixel(colour_plte_suggested_palette_in_rgb(),seq![225u8,26u8,23u8,252u8,246u8,129u8],0nat,true,false)==Ok(seq![57825u16,6682u16,5911u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),colour_plte_suggested_palette_in_rgb(),0nat,1nat,true,false)==Ok(seq![57825u16,6682u16,5911u16,65535u16]));
    assert(((seq![57825u16,6682u16,5911u16,65535u16][0] as nat+128)/257) as u8==225u8) by (compute);
    assert(((seq![57825u16,6682u16,5911u16,65535u16][1] as nat+128)/257) as u8==26u8) by (compute);
    assert(((seq![57825u16,6682u16,5911u16,65535u16][2] as nat+128)/257) as u8==23u8) by (compute);
    assert(((seq![57825u16,6682u16,5911u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixel_plte_suggested_palette_in_rgb_3() ensures sm::image(raw_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),colour_plte_suggested_palette_in_rgb(),false)[12]==252u8,sm::image(raw_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),colour_plte_suggested_palette_in_rgb(),false)[13]==246u8,sm::image(raw_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),colour_plte_suggested_palette_in_rgb(),false)[14]==129u8,sm::image(raw_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),colour_plte_suggested_palette_in_rgb(),false)[15]==255u8,
{
    colour::depth_values(8u8);
    row_plte_suggested_palette_in_rgb_0_1();
    assert(crate::paint_model::source_cell(a_plte_suggested_palette_in_rgb(),1nat,1nat)==(0nat,1nat,1nat)) by (compute);
    assert(colour::pixel(colour_plte_suggested_palette_in_rgb(),seq![225u8,26u8,23u8,252u8,246u8,129u8],1nat,true,false)==Ok(seq![64764u16,63222u16,33153u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),colour_plte_suggested_palette_in_rgb(),1nat,1nat,true,false)==Ok(seq![64764u16,63222u16,33153u16,65535u16]));
    assert(((seq![64764u16,63222u16,33153u16,65535u16][0] as nat+128)/257) as u8==252u8) by (compute);
    assert(((seq![64764u16,63222u16,33153u16,65535u16][1] as nat+128)/257) as u8==246u8) by (compute);
    assert(((seq![64764u16,63222u16,33153u16,65535u16][2] as nat+128)/257) as u8==129u8) by (compute);
    assert(((seq![64764u16,63222u16,33153u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixels_plte_suggested_palette_in_rgb() ensures sm::image(raw_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),colour_plte_suggested_palette_in_rgb(),false)==expected_plte_suggested_palette_in_rgb(),
{
    pixel_plte_suggested_palette_in_rgb_0();
    pixel_plte_suggested_palette_in_rgb_1();
    pixel_plte_suggested_palette_in_rgb_2();
    pixel_plte_suggested_palette_in_rgb_3();
    assert(a_plte_suggested_palette_in_rgb().width==2nat && a_plte_suggested_palette_in_rgb().height==2nat) by (compute);
    assert(sm::image(raw_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),colour_plte_suggested_palette_in_rgb(),false).len()==16nat);
    assert(expected_plte_suggested_palette_in_rgb().len()==16nat) by (compute);
    assert forall|j: int| 0<=j<16 implies (#[trigger] sm::image(raw_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),colour_plte_suggested_palette_in_rgb(),false)[j])==expected_plte_suggested_palette_in_rgb()[j] by {
        if j==0 {assert(expected_plte_suggested_palette_in_rgb()[0]==201u8) by (compute);}
        else if j==1 {assert(expected_plte_suggested_palette_in_rgb()[1]==73u8) by (compute);}
        else if j==2 {assert(expected_plte_suggested_palette_in_rgb()[2]==63u8) by (compute);}
        else if j==3 {assert(expected_plte_suggested_palette_in_rgb()[3]==255u8) by (compute);}
        else if j==4 {assert(expected_plte_suggested_palette_in_rgb()[4]==202u8) by (compute);}
        else if j==5 {assert(expected_plte_suggested_palette_in_rgb()[5]==75u8) by (compute);}
        else if j==6 {assert(expected_plte_suggested_palette_in_rgb()[6]==151u8) by (compute);}
        else if j==7 {assert(expected_plte_suggested_palette_in_rgb()[7]==255u8) by (compute);}
        else if j==8 {assert(expected_plte_suggested_palette_in_rgb()[8]==225u8) by (compute);}
        else if j==9 {assert(expected_plte_suggested_palette_in_rgb()[9]==26u8) by (compute);}
        else if j==10 {assert(expected_plte_suggested_palette_in_rgb()[10]==23u8) by (compute);}
        else if j==11 {assert(expected_plte_suggested_palette_in_rgb()[11]==255u8) by (compute);}
        else if j==12 {assert(expected_plte_suggested_palette_in_rgb()[12]==252u8) by (compute);}
        else if j==13 {assert(expected_plte_suggested_palette_in_rgb()[13]==246u8) by (compute);}
        else if j==14 {assert(expected_plte_suggested_palette_in_rgb()[14]==129u8) by (compute);}
        else {assert(j==15);assert(expected_plte_suggested_palette_in_rgb()[j]==255u8) by (compute);}
    }
    assert(sm::image(raw_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),colour_plte_suggested_palette_in_rgb(),false)=~=expected_plte_suggested_palette_in_rgb());
}
pub open spec fn expected_value_plte_suggested_palette_in_rgb(value: sm::Value) -> bool {value.width==2u32 && value.height==2u32 && value.pixels==expected_plte_suggested_palette_in_rgb()}
#[verifier::spinoff_prover]
pub proof fn model_complete_plte_suggested_palette_in_rgb()
    ensures match sm::MS(bytes_plte_suggested_palette_in_rgb()) {sm::Outcome::Done(value)=>expected_value_plte_suggested_palette_in_rgb(value),_=>false},
{
    hide(m::MW_p);hide(zm::Inflate);hide(bytes_plte_suggested_palette_in_rgb);hide(z_plte_suggested_palette_in_rgb);hide(raw_plte_suggested_palette_in_rgb);hide(expected_plte_suggested_palette_in_rgb);
    assert(bytes_plte_suggested_palette_in_rgb().len()==1121nat) by (compute);
    raster_plte_suggested_palette_in_rgb();inflate_plte_suggested_palette_in_rgb();pixels_plte_suggested_palette_in_rgb();
    assert(bytes_plte_suggested_palette_in_rgb().subrange(41,641)==seq![217u8,5u8,126u8,229u8,40u8,28u8,92u8,184u8,223u8,213u8,121u8,238u8,109u8,36u8,77u8,4u8,167u8,27u8,166u8,48u8,111u8,49u8,91u8,221u8,89u8,53u8,190u8,63u8,235u8,76u8,90u8,66u8,129u8,96u8,87u8,99u8,39u8,127u8,211u8,1u8,24u8,51u8,79u8,48u8,95u8,25u8,251u8,236u8,141u8,58u8,166u8,39u8,245u8,176u8,238u8,21u8,176u8,218u8,3u8,95u8,44u8,200u8,190u8,150u8,158u8,3u8,203u8,176u8,192u8,177u8,86u8,198u8,222u8,55u8,22u8,49u8,216u8,137u8,191u8,32u8,64u8,11u8,236u8,238u8,221u8,93u8,106u8,153u8,119u8,70u8,160u8,70u8,148u8,150u8,144u8,89u8,193u8,82u8,205u8,5u8,90u8,208u8,139u8,82u8,184u8,18u8,158u8,37u8,15u8,12u8,185u8,71u8,81u8,180u8,11u8,151u8,95u8,249u8,148u8,227u8,205u8,172u8,235u8,189u8,229u8,52u8,2u8,44u8,166u8,182u8,186u8,227u8,37u8,103u8,130u8,94u8,104u8,184u8,196u8,230u8,13u8,127u8,235u8,164u8,217u8,129u8,6u8,224u8,233u8,236u8,72u8,246u8,169u8,246u8,62u8,80u8,88u8,219u8,152u8,231u8,189u8,105u8,160u8,27u8,215u8,164u8,173u8,35u8,140u8,75u8,214u8,92u8,33u8,84u8,15u8,83u8,66u8,220u8,58u8,109u8,174u8,187u8,145u8,26u8,93u8,11u8,244u8,15u8,150u8,157u8,165u8,108u8,14u8,34u8,101u8,70u8,42u8,210u8,68u8,224u8,234u8,58u8,144u8,7u8,8u8,200u8,112u8,114u8,9u8,227u8,95u8,168u8,194u8,121u8,243u8,15u8,246u8,20u8,171u8,24u8,54u8,128u8,113u8,96u8,45u8,80u8,75u8,94u8,243u8,169u8,254u8,241u8,217u8,214u8,102u8,180u8,124u8,54u8,47u8,156u8,179u8,173u8,102u8,209u8,90u8,28u8,66u8,17u8,60u8,126u8,113u8,193u8,193u8,66u8,150u8,139u8,200u8,94u8,162u8,111u8,143u8,200u8,56u8,253u8,248u8,90u8,238u8,150u8,233u8,78u8,42u8,134u8,186u8,216u8,70u8,219u8,58u8,151u8,73u8,56u8,238u8,38u8,222u8,47u8,178u8,181u8,4u8,136u8,214u8,241u8,102u8,191u8,39u8,49u8,55u8,195u8,188u8,50u8,105u8,210u8,89u8,27u8,49u8,27u8,141u8,39u8,245u8,231u8,35u8,162u8,106u8,53u8,66u8,44u8,203u8,2u8,112u8,205u8,234u8,232u8,133u8,0u8,87u8,145u8,15u8,0u8,125u8,215u8,173u8,105u8,7u8,91u8,227u8,195u8,221u8,8u8,191u8,160u8,132u8,214u8,106u8,143u8,215u8,2u8,98u8,88u8,220u8,115u8,213u8,193u8,166u8,32u8,233u8,198u8,77u8,22u8,163u8,234u8,97u8,166u8,220u8,183u8,61u8,91u8,16u8,231u8,70u8,168u8,57u8,36u8,111u8,154u8,180u8,231u8,115u8,221u8,117u8,210u8,228u8,244u8,152u8,199u8,195u8,219u8,60u8,15u8,195u8,114u8,200u8,222u8,241u8,5u8,154u8,61u8,251u8,237u8,87u8,7u8,200u8,90u8,30u8,146u8,116u8,162u8,211u8,33u8,75u8,247u8,91u8,245u8,163u8,252u8,212u8,218u8,231u8,68u8,20u8,108u8,86u8,62u8,179u8,124u8,34u8,164u8,253u8,8u8,165u8,157u8,17u8,198u8,157u8,168u8,130u8,98u8,106u8,84u8,83u8,32u8,16u8,240u8,55u8,94u8,26u8,86u8,170u8,69u8,120u8,114u8,244u8,241u8,223u8,14u8,212u8,227u8,219u8,218u8,82u8,90u8,212u8,102u8,200u8,191u8,175u8,62u8,1u8,118u8,236u8,85u8,111u8,228u8,181u8,36u8,156u8,216u8,90u8,236u8,19u8,246u8,19u8,182u8,40u8,196u8,221u8,94u8,247u8,11u8,241u8,38u8,236u8,90u8,45u8,63u8,248u8,237u8,144u8,82u8,35u8,143u8,89u8,30u8,100u8,154u8,194u8,162u8,16u8,140u8,82u8,88u8,69u8,53u8,106u8,170u8,117u8,126u8,213u8,72u8,33u8,243u8,176u8,253u8,209u8,23u8,227u8,0u8,194u8,26u8,251u8,229u8,54u8,130u8,184u8,87u8,126u8,228u8,255u8,39u8,216u8,35u8,81u8,111u8,215u8,222u8,86u8,250u8,173u8,226u8,86u8,44u8,120u8,253u8,66u8,1u8,217u8,1u8,43u8,145u8,75u8,132u8,77u8,196u8,237u8,112u8,162u8,123u8,156u8,80u8,214u8,90u8,240u8,154u8,205u8,49u8,148u8,210u8,253u8,245u8,117u8,255u8,99u8,48u8,82u8,20u8,48u8,201u8,131u8,237u8,156u8,171u8,200u8,8u8,88u8,124u8,80u8,131u8,125u8,108u8,63u8,37u8,245u8,67u8]) by (compute);
    match m::MW_p(bytes_plte_suggested_palette_in_rgb(),m::strict()) {m::Outcome::Done(doc,_)=>{
        assert(colour::from_doc(doc,bytes_plte_suggested_palette_in_rgb())==colour_plte_suggested_palette_in_rgb());
        assert(crate::paint_model::default_raster(doc,STRICT)==a_plte_suggested_palette_in_rgb());
    },_=>{}}
    assert(r::raw(a_plte_suggested_palette_in_rgb())==14nat) by (compute);
    assert(r::inflated(z_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),STRICT).out==raw_plte_suggested_palette_in_rgb());
    compose_complete(bytes_plte_suggested_palette_in_rgb(),h_plte_suggested_palette_in_rgb(),z_plte_suggested_palette_in_rgb(),200nat,colour_plte_suggested_palette_in_rgb(),expected_plte_suggested_palette_in_rgb());
}
pub fn public_complete_plte_suggested_palette_in_rgb() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_plte_suggested_palette_in_rgb(),false),result),se::eq(bytes_plte_suggested_palette_in_rgb(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_plte_suggested_palette_in_rgb(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,2u8,88u8,80u8,76u8,84u8,69u8,217u8,5u8,126u8,229u8,40u8,28u8,92u8,184u8,223u8,213u8,121u8,238u8,109u8,36u8,77u8,4u8,167u8,27u8,166u8,48u8,111u8,49u8,91u8,221u8,89u8,53u8,190u8,63u8,235u8,76u8,90u8,66u8,129u8,96u8,87u8,99u8,39u8,127u8,211u8,1u8,24u8,51u8,79u8,48u8,95u8,25u8,251u8,236u8,141u8,58u8,166u8,39u8,245u8,176u8,238u8,21u8,176u8,218u8,3u8,95u8,44u8,200u8,190u8,150u8,158u8,3u8,203u8,176u8,192u8,177u8,86u8,198u8,222u8,55u8,22u8,49u8,216u8,137u8,191u8,32u8,64u8,11u8,236u8,238u8,221u8,93u8,106u8,153u8,119u8,70u8,160u8,70u8,148u8,150u8,144u8,89u8,193u8,82u8,205u8,5u8,90u8,208u8,139u8,82u8,184u8,18u8,158u8,37u8,15u8,12u8,185u8,71u8,81u8,180u8,11u8,151u8,95u8,249u8,148u8,227u8,205u8,172u8,235u8,189u8,229u8,52u8,2u8,44u8,166u8,182u8,186u8,227u8,37u8,103u8,130u8,94u8,104u8,184u8,196u8,230u8,13u8,127u8,235u8,164u8,217u8,129u8,6u8,224u8,233u8,236u8,72u8,246u8,169u8,246u8,62u8,80u8,88u8,219u8,152u8,231u8,189u8,105u8,160u8,27u8,215u8,164u8,173u8,35u8,140u8,75u8,214u8,92u8,33u8,84u8,15u8,83u8,66u8,220u8,58u8,109u8,174u8,187u8,145u8,26u8,93u8,11u8,244u8,15u8,150u8,157u8,165u8,108u8,14u8,34u8,101u8,70u8,42u8,210u8,68u8,224u8,234u8,58u8,144u8,7u8,8u8,200u8,112u8,114u8,9u8,227u8,95u8,168u8,194u8,121u8,243u8,15u8,246u8,20u8,171u8,24u8,54u8,128u8,113u8,96u8,45u8,80u8,75u8,94u8,243u8,169u8,254u8,241u8,217u8,214u8,102u8,180u8,124u8,54u8,47u8,156u8,179u8,173u8,102u8,209u8,90u8,28u8,66u8,17u8,60u8,126u8,113u8,193u8,193u8,66u8,150u8,139u8,200u8,94u8,162u8,111u8,143u8,200u8,56u8,253u8,248u8,90u8,238u8,150u8,233u8,78u8,42u8,134u8,186u8,216u8,70u8,219u8,58u8,151u8,73u8,56u8,238u8,38u8,222u8,47u8,178u8,181u8,4u8,136u8,214u8,241u8,102u8,191u8,39u8,49u8,55u8,195u8,188u8,50u8,105u8,210u8,89u8,27u8,49u8,27u8,141u8,39u8,245u8,231u8,35u8,162u8,106u8,53u8,66u8,44u8,203u8,2u8,112u8,205u8,234u8,232u8,133u8,0u8,87u8,145u8,15u8,0u8,125u8,215u8,173u8,105u8,7u8,91u8,227u8,195u8,221u8,8u8,191u8,160u8,132u8,214u8,106u8,143u8,215u8,2u8,98u8,88u8,220u8,115u8,213u8,193u8,166u8,32u8,233u8,198u8,77u8,22u8,163u8,234u8,97u8,166u8,220u8,183u8,61u8,91u8,16u8,231u8,70u8,168u8,57u8,36u8,111u8,154u8,180u8,231u8,115u8,221u8,117u8,210u8,228u8,244u8,152u8,199u8,195u8,219u8,60u8,15u8,195u8,114u8,200u8,222u8,241u8,5u8,154u8,61u8,251u8,237u8,87u8,7u8,200u8,90u8,30u8,146u8,116u8,162u8,211u8,33u8,75u8,247u8,91u8,245u8,163u8,252u8,212u8,218u8,231u8,68u8,20u8,108u8,86u8,62u8,179u8,124u8,34u8,164u8,253u8,8u8,165u8,157u8,17u8,198u8,157u8,168u8,130u8,98u8,106u8,84u8,83u8,32u8,16u8,240u8,55u8,94u8,26u8,86u8,170u8,69u8,120u8,114u8,244u8,241u8,223u8,14u8,212u8,227u8,219u8,218u8,82u8,90u8,212u8,102u8,200u8,191u8,175u8,62u8,1u8,118u8,236u8,85u8,111u8,228u8,181u8,36u8,156u8,216u8,90u8,236u8,19u8,246u8,19u8,182u8,40u8,196u8,221u8,94u8,247u8,11u8,241u8,38u8,236u8,90u8,45u8,63u8,248u8,237u8,144u8,82u8,35u8,143u8,89u8,30u8,100u8,154u8,194u8,162u8,16u8,140u8,82u8,88u8,69u8,53u8,106u8,170u8,117u8,126u8,213u8,72u8,33u8,243u8,176u8,253u8,209u8,23u8,227u8,0u8,194u8,26u8,251u8,229u8,54u8,130u8,184u8,87u8,126u8,228u8,255u8,39u8,216u8,35u8,81u8,111u8,215u8,222u8,86u8,250u8,173u8,226u8,86u8,44u8,120u8,253u8,66u8,1u8,217u8,1u8,43u8,145u8,75u8,132u8,77u8,196u8,237u8,112u8,162u8,123u8,156u8,80u8,214u8,90u8,240u8,154u8,205u8,49u8,148u8,210u8,253u8,245u8,117u8,255u8,99u8,48u8,82u8,20u8,48u8,201u8,131u8,237u8,156u8,171u8,200u8,8u8,88u8,124u8,80u8,131u8,125u8,108u8,63u8,37u8,245u8,67u8,214u8,20u8,107u8,218u8,0u8,0u8,0u8,6u8,98u8,75u8,71u8,68u8,0u8,1u8,0u8,2u8,0u8,3u8,94u8,174u8,23u8,27u8,0u8,0u8,1u8,144u8,104u8,73u8,83u8,84u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,93u8,124u8,21u8,200u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_plte_suggested_palette_in_rgb());model_complete_plte_suggested_palette_in_rgb();}
    crate::png::try_decode_still(&b)
}
// anc-ok/iCCP-303; frozen fixture SHA256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
pub open spec fn bytes_anc_ok_iccp_303() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,1u8,47u8,105u8,67u8,67u8,80u8,115u8,82u8,71u8,66u8,32u8,73u8,69u8,67u8,54u8,49u8,57u8,54u8,54u8,45u8,50u8,46u8,49u8,0u8,0u8,120u8,156u8,99u8,96u8,100u8,98u8,102u8,97u8,101u8,99u8,231u8,224u8,228u8,226u8,230u8,225u8,229u8,227u8,23u8,16u8,20u8,18u8,22u8,17u8,21u8,19u8,151u8,144u8,148u8,146u8,150u8,145u8,149u8,147u8,87u8,80u8,84u8,82u8,86u8,81u8,85u8,83u8,215u8,208u8,212u8,210u8,214u8,209u8,213u8,211u8,55u8,48u8,52u8,50u8,54u8,49u8,53u8,51u8,183u8,176u8,180u8,178u8,182u8,177u8,181u8,179u8,119u8,112u8,116u8,114u8,118u8,113u8,117u8,115u8,247u8,240u8,244u8,242u8,246u8,241u8,245u8,243u8,15u8,8u8,12u8,10u8,14u8,9u8,13u8,11u8,143u8,136u8,140u8,138u8,142u8,137u8,141u8,139u8,79u8,72u8,76u8,74u8,78u8,73u8,77u8,75u8,207u8,200u8,204u8,202u8,206u8,201u8,205u8,203u8,47u8,40u8,44u8,42u8,46u8,41u8,45u8,43u8,175u8,168u8,172u8,170u8,174u8,169u8,173u8,171u8,111u8,104u8,108u8,106u8,110u8,105u8,109u8,107u8,239u8,232u8,236u8,234u8,238u8,233u8,237u8,235u8,159u8,48u8,113u8,210u8,228u8,41u8,83u8,167u8,77u8,159u8,49u8,115u8,214u8,236u8,57u8,115u8,231u8,205u8,95u8,176u8,112u8,209u8,226u8,37u8,75u8,151u8,45u8,95u8,177u8,114u8,213u8,234u8,53u8,107u8,215u8,173u8,223u8,176u8,113u8,211u8,230u8,45u8,91u8,183u8,109u8,223u8,177u8,115u8,215u8,238u8,61u8,123u8,247u8,237u8,63u8,112u8,240u8,208u8,225u8,35u8,71u8,143u8,29u8,63u8,113u8,242u8,212u8,233u8,51u8,103u8,207u8,157u8,191u8,112u8,241u8,210u8,229u8,43u8,87u8,175u8,93u8,191u8,113u8,243u8,214u8,237u8,59u8,119u8,239u8,221u8,127u8,240u8,240u8,209u8,227u8,39u8,79u8,159u8,61u8,127u8,241u8,242u8,213u8,235u8,55u8,111u8,223u8,189u8,255u8,240u8,241u8,211u8,231u8,47u8,95u8,191u8,125u8,255u8,241u8,243u8,215u8,239u8,63u8,127u8,255u8,253u8,103u8,24u8,245u8,255u8,136u8,246u8,63u8,0u8,160u8,98u8,126u8,144u8,16u8,123u8,222u8,207u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn h_anc_ok_iccp_303() -> crate::model::Ihdr {crate::model::Ihdr {width:2u32,height:2u32,depth:8u8,color_type:2u8,interlace:0u8}}
spec fn a_anc_ok_iccp_303() -> r::Raster {r::shape(h_anc_ok_iccp_303(),2nat,2nat,0nat,STRICT)}
spec fn colour_anc_ok_iccp_303() -> crate::colour_model::Context {crate::colour_model::Context {ct:2u8,depth:8u8,plte:seq![],trns:None}}
pub open spec fn expected_anc_ok_iccp_303() -> Seq<u8> {seq![201u8,73u8,63u8,255u8,202u8,75u8,151u8,255u8,225u8,26u8,23u8,255u8,252u8,246u8,129u8,255u8]}
spec fn z_anc_ok_iccp_303() -> Seq<u8> {seq![120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8]}
spec fn raw_anc_ok_iccp_303() -> Seq<u8> {seq![0u8,201u8,73u8,63u8,202u8,75u8,151u8,1u8,225u8,26u8,23u8,27u8,220u8,106u8]}
spec fn ts_anc_ok_iccp_303() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:36nat,before:1nat,after:2nat,symbol:201nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:36nat,next:44nat,before:2nat,after:3nat,symbol:73nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:44nat,next:52nat,before:3nat,after:4nat,symbol:63nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:52nat,next:61nat,before:4nat,after:5nat,symbol:202nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:61nat,next:69nat,before:5nat,after:6nat,symbol:75nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:69nat,next:78nat,before:6nat,after:7nat,symbol:151nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:78nat,next:86nat,before:7nat,after:8nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:86nat,next:95nat,before:8nat,after:9nat,symbol:225nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:95nat,next:103nat,before:9nat,after:10nat,symbol:26nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:103nat,next:111nat,before:10nat,after:11nat,symbol:23nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:111nat,next:119nat,before:11nat,after:12nat,symbol:27nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:119nat,next:128nat,before:12nat,after:13nat,symbol:220nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:128nat,next:136nat,before:13nat,after:14nat,symbol:106nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:136nat,next:143nat,before:14nat,after:14nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_anc_ok_iccp_303_0() ensures t::trace(z_anc_ok_iccp_303(),raw_anc_ok_iccp_303(),ts_anc_ok_iccp_303(),0nat,15nat),
{
    hide(t::valid_step);assert(t::trace(z_anc_ok_iccp_303(),raw_anc_ok_iccp_303(),ts_anc_ok_iccp_303(),0nat,15nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn trace_anc_ok_iccp_303() ensures t::trace(z_anc_ok_iccp_303(),raw_anc_ok_iccp_303(),ts_anc_ok_iccp_303(),0,15nat),
{
    hide(z_anc_ok_iccp_303);hide(raw_anc_ok_iccp_303);hide(ts_anc_ok_iccp_303);hide(t::trace);
    trace_anc_ok_iccp_303_0();
    assert(t::trace(z_anc_ok_iccp_303(),raw_anc_ok_iccp_303(),ts_anc_ok_iccp_303(),0,15nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_anc_ok_iccp_303() ensures z_anc_ok_iccp_303().len()==22nat,
        raw_anc_ok_iccp_303().len()==14nat,
        z_anc_ok_iccp_303()[0]==120u8,
        z_anc_ok_iccp_303()[1]==156u8,
        zm::field_value(z_anc_ok_iccp_303(),16,1)==1nat,
        zm::field_value(z_anc_ok_iccp_303(),17,2)==1nat,
        zm::be_value(z_anc_ok_iccp_303(),18nat,4)==675349874nat,
        ts_anc_ok_iccp_303().len()==15nat,
        ts_anc_ok_iccp_303()[0].pos==19nat,
        ts_anc_ok_iccp_303()[0].before==0nat,
        ts_anc_ok_iccp_303().last().next==143nat,
{
    assert(z_anc_ok_iccp_303().len()==22nat) by (compute);
    assert(raw_anc_ok_iccp_303().len()==14nat) by (compute);
    assert(z_anc_ok_iccp_303()[0]==120u8) by (compute);
    assert(z_anc_ok_iccp_303()[1]==156u8) by (compute);
    assert(zm::field_value(z_anc_ok_iccp_303(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_anc_ok_iccp_303(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_anc_ok_iccp_303(),18nat,4)==675349874nat) by (compute);
    assert(ts_anc_ok_iccp_303().len()==15nat) by (compute);
    assert(ts_anc_ok_iccp_303()[0].pos==19nat) by (compute);
    assert(ts_anc_ok_iccp_303()[0].before==0nat) by (compute);
    assert(ts_anc_ok_iccp_303().last().next==143nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_anc_ok_iccp_303() ensures zm::Adler(raw_anc_ok_iccp_303())==675349874nat,
{
    hide(raw_anc_ok_iccp_303);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_anc_ok_iccp_303().len()==14nat) by (compute);
    assert(c::sums(raw_anc_ok_iccp_303(),0,14nat)==(1393nat,10305nat)) by (compute);
    c::sums_correct(raw_anc_ok_iccp_303(),0,14nat);assert(raw_anc_ok_iccp_303().subrange(0,14)=~=raw_anc_ok_iccp_303());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((10305nat%65521)*(u16::MAX as nat+1)+(1+1393nat)%65521==675349874nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_anc_ok_iccp_303() ensures zm::Inflate(z_anc_ok_iccp_303(),14nat,true,false)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_anc_ok_iccp_303(),r2:false}),raw_anc_ok_iccp_303().len()==14nat,
{
    hide(z_anc_ok_iccp_303);hide(raw_anc_ok_iccp_303);hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);hide(ts_anc_ok_iccp_303);
    shape_anc_ok_iccp_303();checksum_anc_ok_iccp_303();
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_anc_ok_iccp_303();ms::fixed_stream(z_anc_ok_iccp_303(),raw_anc_ok_iccp_303(),ts_anc_ok_iccp_303(),14nat,156nat,675349874nat,675349874nat);
}
#[verifier::spinoff_prover]
proof fn crc_anc_ok_iccp_303_crc_33()
    ensures crate::model::crc(bytes_anc_ok_iccp_303().subrange(37,344))==276553423u32,
{
    hide(crate::model::crc_register);hide(crc_cert::fold);
    assert(crate::model::crc_register(bytes_anc_ok_iccp_303().subrange(37,344).take(0))==0xffff_ffffu32) by {
        assert(bytes_anc_ok_iccp_303().subrange(37,344).take(0) =~= Seq::<u8>::empty());
        reveal_with_fuel(crate::model::crc_register,1);
    }
    assert(crc_cert::fold(4294967295u32,bytes_anc_ok_iccp_303().subrange(37,344).subrange(0,64))==542836632u32) by (compute);
    crc_cert::suffix(bytes_anc_ok_iccp_303().subrange(37,344).take(0),bytes_anc_ok_iccp_303().subrange(37,344).subrange(0,64));
    assert(bytes_anc_ok_iccp_303().subrange(37,344).take(0)+bytes_anc_ok_iccp_303().subrange(37,344).subrange(0,64) =~= bytes_anc_ok_iccp_303().subrange(37,344).take(64));
    assert(crate::model::crc_register(bytes_anc_ok_iccp_303().subrange(37,344).take(64))==542836632u32);
    assert(crc_cert::fold(542836632u32,bytes_anc_ok_iccp_303().subrange(37,344).subrange(64,128))==4139143574u32) by (compute);
    crc_cert::suffix(bytes_anc_ok_iccp_303().subrange(37,344).take(64),bytes_anc_ok_iccp_303().subrange(37,344).subrange(64,128));
    assert(bytes_anc_ok_iccp_303().subrange(37,344).take(64)+bytes_anc_ok_iccp_303().subrange(37,344).subrange(64,128) =~= bytes_anc_ok_iccp_303().subrange(37,344).take(128));
    assert(crate::model::crc_register(bytes_anc_ok_iccp_303().subrange(37,344).take(128))==4139143574u32);
    assert(crc_cert::fold(4139143574u32,bytes_anc_ok_iccp_303().subrange(37,344).subrange(128,192))==4256178532u32) by (compute);
    crc_cert::suffix(bytes_anc_ok_iccp_303().subrange(37,344).take(128),bytes_anc_ok_iccp_303().subrange(37,344).subrange(128,192));
    assert(bytes_anc_ok_iccp_303().subrange(37,344).take(128)+bytes_anc_ok_iccp_303().subrange(37,344).subrange(128,192) =~= bytes_anc_ok_iccp_303().subrange(37,344).take(192));
    assert(crate::model::crc_register(bytes_anc_ok_iccp_303().subrange(37,344).take(192))==4256178532u32);
    assert(crc_cert::fold(4256178532u32,bytes_anc_ok_iccp_303().subrange(37,344).subrange(192,256))==2484499056u32) by (compute);
    crc_cert::suffix(bytes_anc_ok_iccp_303().subrange(37,344).take(192),bytes_anc_ok_iccp_303().subrange(37,344).subrange(192,256));
    assert(bytes_anc_ok_iccp_303().subrange(37,344).take(192)+bytes_anc_ok_iccp_303().subrange(37,344).subrange(192,256) =~= bytes_anc_ok_iccp_303().subrange(37,344).take(256));
    assert(crate::model::crc_register(bytes_anc_ok_iccp_303().subrange(37,344).take(256))==2484499056u32);
    assert(crc_cert::fold(2484499056u32,bytes_anc_ok_iccp_303().subrange(37,344).subrange(256,307))==4018413872u32) by (compute);
    crc_cert::suffix(bytes_anc_ok_iccp_303().subrange(37,344).take(256),bytes_anc_ok_iccp_303().subrange(37,344).subrange(256,307));
    assert(bytes_anc_ok_iccp_303().subrange(37,344).take(256)+bytes_anc_ok_iccp_303().subrange(37,344).subrange(256,307) =~= bytes_anc_ok_iccp_303().subrange(37,344).take(307));
    assert(crate::model::crc_register(bytes_anc_ok_iccp_303().subrange(37,344).take(307))==4018413872u32);
    assert(bytes_anc_ok_iccp_303().subrange(37,344).take(307) =~= bytes_anc_ok_iccp_303().subrange(37,344));
    assert((4018413872u32 ^ 0xffff_ffffu32)==276553423u32) by (compute);
}
#[verifier::spinoff_prover]
proof fn walk_anc_ok_iccp_303(b: Seq<u8>)
    requires b.len()==394,
        crate::model::mh(b)==Ok((h_anc_ok_iccp_303(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![105u8,67u8,67u8,80u8],data:41nat,len:303nat,next:348nat,crc_ok:true,synthetic:false})),
        crate::model::first_nul(b.subrange(41,344),0)==Some(17nat),
        b.subrange(41,344).subrange(0,17)==seq![115u8,82u8,71u8,66u8,32u8,73u8,69u8,67u8,54u8,49u8,57u8,54u8,54u8,45u8,50u8,46u8,49u8],
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,348nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:356nat,len:22nat,next:382nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,382nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:390nat,len:0nat,next:394nat,crc_ok:true,synthetic:false})),
        b.subrange(41,344)[18]==0u8,
        crate::metadata_model::parse(crate::walk_model::Name::Iccp,b.subrange(41,344),2u8,8u8,0nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Iccp {name_len:17,profile_start:19,profile_len:284}),
        b.subrange(356,378)==z_anc_ok_iccp_303(),
    ensures r::default_case(b,h_anc_ok_iccp_303(),z_anc_ok_iccp_303(),0nat),match m::MW_p(b,m::strict()) {m::Outcome::Done(doc,_)=>doc.plte==None && doc.trns==None,_=>false},
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(m::store);hide(crate::metadata_model::parse);
    assert forall|doc: m::Doc,value: crate::metadata_model::Value,base: nat|
        (#[trigger] m::store(doc,value,base)).ihdr==doc.ihdr && m::store(doc,value,base).plte==doc.plte
        && m::store(doc,value,base).trns==doc.trns && m::store(doc,value,base).actl==doc.actl
        && m::store(doc,value,base).idat==doc.idat && m::store(doc,value,base).frames==doc.frames by {
        stored_structure(doc,value,base);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((105u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![105u8,67u8,67u8,80u8])==crate::walk_model::Name::Iccp) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![105u8,67u8,67u8,80u8])==crate::walk_model::Name::Iccp) by (compute);
        }
        assert(crate::model::keyword(seq![115u8,82u8,71u8,66u8,32u8,73u8,69u8,67u8,54u8,49u8,57u8,54u8,54u8,45u8,50u8,46u8,49u8]));
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    reveal_with_fuel(m::drive,4);
    reveal_with_fuel(m::concat_ranges,3);
}
#[verifier::spinoff_prover]
proof fn scan_anc_ok_iccp_303() ensures r::scan(raw_anc_ok_iccp_303(),a_anc_ok_iccp_303(),0)==Ok(()),
{
    hide(raw_anc_ok_iccp_303);assert(raw_anc_ok_iccp_303().len()==14nat) by (compute);assert(r::count(a_anc_ok_iccp_303())==1nat) by (compute);
    assert(r::pw(a_anc_ok_iccp_303(),0nat)==2nat && r::ph(a_anc_ok_iccp_303(),0nat)==2nat && r::rb(a_anc_ok_iccp_303(),0nat)==6nat && r::base(a_anc_ok_iccp_303(),0nat)==0nat) by (compute);
    assert(raw_anc_ok_iccp_303()[0]==0u8) by (compute);
    assert(r::row_result(raw_anc_ok_iccp_303(),a_anc_ok_iccp_303(),0nat,0nat)==Ok(()));
    assert(raw_anc_ok_iccp_303()[7]==1u8) by (compute);
    assert(r::row_result(raw_anc_ok_iccp_303(),a_anc_ok_iccp_303(),0nat,1nat)==Ok(()));
    r::scan_step(raw_anc_ok_iccp_303(),a_anc_ok_iccp_303(),1nat);
    r::pass_step(raw_anc_ok_iccp_303(),a_anc_ok_iccp_303(),0nat,2nat);
    r::pass_step(raw_anc_ok_iccp_303(),a_anc_ok_iccp_303(),0nat,1nat);
    r::pass_step(raw_anc_ok_iccp_303(),a_anc_ok_iccp_303(),0nat,0nat);
    r::scan_step(raw_anc_ok_iccp_303(),a_anc_ok_iccp_303(),0nat);
}
#[verifier::spinoff_prover]
proof fn row_anc_ok_iccp_303_0_0() ensures r::line(raw_anc_ok_iccp_303(),a_anc_ok_iccp_303(),0nat,0nat)==seq![201u8,73u8,63u8,202u8,75u8,151u8],
{
    hide(raw_anc_ok_iccp_303);
    assert(raw_anc_ok_iccp_303().len()==14nat) by (compute);
    assert(raw_anc_ok_iccp_303()[0]==0u8) by (compute);
    assert(r::base(a_anc_ok_iccp_303(),0nat)==0nat && r::rb(a_anc_ok_iccp_303(),0nat)==6nat && f::bpp(r::bits(a_anc_ok_iccp_303()))==3nat) by (compute);
    assert(f::row_data(raw_anc_ok_iccp_303(),0nat,6nat,0nat)==seq![201u8,73u8,63u8,202u8,75u8,151u8]) by (compute);
    f::pass_row_equation(raw_anc_ok_iccp_303(),0nat,6nat,3nat,0nat);
    assert(f::previous_row(raw_anc_ok_iccp_303(),0nat,6nat,3nat,0nat)==None);
    assert(f::encoded_row(0u8,f::row_data(raw_anc_ok_iccp_303(),0nat,6nat,0nat),None,3nat,seq![201u8,73u8,63u8,202u8,75u8,151u8],0,6nat)) by (compute);
    f::checked_row_hint(0u8,f::row_data(raw_anc_ok_iccp_303(),0nat,6nat,0nat),None,3nat,seq![201u8,73u8,63u8,202u8,75u8,151u8]);
}
#[verifier::spinoff_prover]
proof fn row_anc_ok_iccp_303_0_1() ensures r::line(raw_anc_ok_iccp_303(),a_anc_ok_iccp_303(),0nat,1nat)==seq![225u8,26u8,23u8,252u8,246u8,129u8],
{
    hide(raw_anc_ok_iccp_303);
    assert(raw_anc_ok_iccp_303().len()==14nat) by (compute);
    assert(raw_anc_ok_iccp_303()[7]==1u8) by (compute);
    assert(r::base(a_anc_ok_iccp_303(),0nat)==0nat && r::rb(a_anc_ok_iccp_303(),0nat)==6nat && f::bpp(r::bits(a_anc_ok_iccp_303()))==3nat) by (compute);
    assert(f::row_data(raw_anc_ok_iccp_303(),0nat,6nat,1nat)==seq![225u8,26u8,23u8,27u8,220u8,106u8]) by (compute);
    row_anc_ok_iccp_303_0_0();
    f::pass_row_equation(raw_anc_ok_iccp_303(),0nat,6nat,3nat,1nat);
    assert(f::previous_row(raw_anc_ok_iccp_303(),0nat,6nat,3nat,1nat)==Some(seq![201u8,73u8,63u8,202u8,75u8,151u8]));
    assert(f::encoded_row(1u8,f::row_data(raw_anc_ok_iccp_303(),0nat,6nat,1nat),Some(seq![201u8,73u8,63u8,202u8,75u8,151u8]),3nat,seq![225u8,26u8,23u8,252u8,246u8,129u8],0,6nat)) by (compute);
    f::checked_row_hint(1u8,f::row_data(raw_anc_ok_iccp_303(),0nat,6nat,1nat),Some(seq![201u8,73u8,63u8,202u8,75u8,151u8]),3nat,seq![225u8,26u8,23u8,252u8,246u8,129u8]);
}
#[verifier::spinoff_prover]
proof fn raster_anc_ok_iccp_303()
    ensures r::default_case(bytes_anc_ok_iccp_303(),h_anc_ok_iccp_303(),z_anc_ok_iccp_303(),0nat),match m::MW_p(bytes_anc_ok_iccp_303(),m::strict()) {m::Outcome::Done(doc,_)=>doc.plte==None && doc.trns==None,_=>false},r::data(z_anc_ok_iccp_303(),a_anc_ok_iccp_303(),STRICT)==Some(Ok(())),
{
    hide(bytes_anc_ok_iccp_303);hide(z_anc_ok_iccp_303);hide(raw_anc_ok_iccp_303);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    assert(bytes_anc_ok_iccp_303().len()==394nat) by (compute);
    assert(crate::model::mh(bytes_anc_ok_iccp_303())==Ok((h_anc_ok_iccp_303(),33nat))) by {reveal(crate::model::mh);assert(crate::model::mh(bytes_anc_ok_iccp_303())==Ok((h_anc_ok_iccp_303(),33nat))) by (compute);}
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_ok_iccp_303(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![105u8,67u8,67u8,80u8],data:41nat,len:303nat,next:348nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::model::frame(bytes_anc_ok_iccp_303(),33nat)==Ok(crate::model::WireChunk {ty:seq![105u8,67u8,67u8,80u8],data:41nat,len:303nat,next:348nat})) by (compute);
            assert(crate::model::be32(bytes_anc_ok_iccp_303(),344)==276553423u32) by (compute);
            crc_anc_ok_iccp_303_crc_33();
        }
        assert((105u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![105u8,67u8,67u8,80u8])==crate::walk_model::Name::Iccp) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![105u8,67u8,67u8,80u8])==crate::walk_model::Name::Iccp) by (compute);
        }
        assert(crate::model::first_nul(bytes_anc_ok_iccp_303().subrange(41,344),0)==Some(17nat)) by (compute);
        assert(bytes_anc_ok_iccp_303().subrange(41,344).subrange(0,17)==seq![115u8,82u8,71u8,66u8,32u8,73u8,69u8,67u8,54u8,49u8,57u8,54u8,54u8,45u8,50u8,46u8,49u8]) by (compute);
        assert(crate::model::keyword(seq![115u8,82u8,71u8,66u8,32u8,73u8,69u8,67u8,54u8,49u8,57u8,54u8,54u8,45u8,50u8,46u8,49u8]));
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_ok_iccp_303(),348nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:356nat,len:22nat,next:382nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_ok_iccp_303(),348nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:356nat,len:22nat,next:382nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_ok_iccp_303(),382nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:390nat,len:0nat,next:394nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_ok_iccp_303(),382nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:390nat,len:0nat,next:394nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(bytes_anc_ok_iccp_303().subrange(41,344)[18]==0u8) by (compute);
        assert(crate::metadata_model::parse(crate::walk_model::Name::Iccp,bytes_anc_ok_iccp_303().subrange(41,344),2u8,8u8,0nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Iccp {name_len:17,profile_start:19,profile_len:284})) by {
            reveal(crate::metadata_model::parse);
            iccp_success(bytes_anc_ok_iccp_303().subrange(41,344),17nat);
        }
    assert(bytes_anc_ok_iccp_303().subrange(356,378)==z_anc_ok_iccp_303()) by (compute);
    walk_anc_ok_iccp_303(bytes_anc_ok_iccp_303());
    inflate_anc_ok_iccp_303();scan_anc_ok_iccp_303();
    assert(r::raw(a_anc_ok_iccp_303())==14nat) by (compute);
    assert(r::data(z_anc_ok_iccp_303(),a_anc_ok_iccp_303(),STRICT)==Some(Ok(()))) by {reveal(r::data);};
}
#[verifier::spinoff_prover]
proof fn pixel_anc_ok_iccp_303_0() ensures sm::image(raw_anc_ok_iccp_303(),a_anc_ok_iccp_303(),colour_anc_ok_iccp_303(),false)[0]==201u8,sm::image(raw_anc_ok_iccp_303(),a_anc_ok_iccp_303(),colour_anc_ok_iccp_303(),false)[1]==73u8,sm::image(raw_anc_ok_iccp_303(),a_anc_ok_iccp_303(),colour_anc_ok_iccp_303(),false)[2]==63u8,sm::image(raw_anc_ok_iccp_303(),a_anc_ok_iccp_303(),colour_anc_ok_iccp_303(),false)[3]==255u8,
{
    colour::depth_values(8u8);
    row_anc_ok_iccp_303_0_0();
    assert(crate::paint_model::source_cell(a_anc_ok_iccp_303(),0nat,0nat)==(0nat,0nat,0nat)) by (compute);
    assert(colour::pixel(colour_anc_ok_iccp_303(),seq![201u8,73u8,63u8,202u8,75u8,151u8],0nat,true,false)==Ok(seq![51657u16,18761u16,16191u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_anc_ok_iccp_303(),a_anc_ok_iccp_303(),colour_anc_ok_iccp_303(),0nat,0nat,true,false)==Ok(seq![51657u16,18761u16,16191u16,65535u16]));
    assert(((seq![51657u16,18761u16,16191u16,65535u16][0] as nat+128)/257) as u8==201u8) by (compute);
    assert(((seq![51657u16,18761u16,16191u16,65535u16][1] as nat+128)/257) as u8==73u8) by (compute);
    assert(((seq![51657u16,18761u16,16191u16,65535u16][2] as nat+128)/257) as u8==63u8) by (compute);
    assert(((seq![51657u16,18761u16,16191u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixel_anc_ok_iccp_303_1() ensures sm::image(raw_anc_ok_iccp_303(),a_anc_ok_iccp_303(),colour_anc_ok_iccp_303(),false)[4]==202u8,sm::image(raw_anc_ok_iccp_303(),a_anc_ok_iccp_303(),colour_anc_ok_iccp_303(),false)[5]==75u8,sm::image(raw_anc_ok_iccp_303(),a_anc_ok_iccp_303(),colour_anc_ok_iccp_303(),false)[6]==151u8,sm::image(raw_anc_ok_iccp_303(),a_anc_ok_iccp_303(),colour_anc_ok_iccp_303(),false)[7]==255u8,
{
    colour::depth_values(8u8);
    row_anc_ok_iccp_303_0_0();
    assert(crate::paint_model::source_cell(a_anc_ok_iccp_303(),1nat,0nat)==(0nat,1nat,0nat)) by (compute);
    assert(colour::pixel(colour_anc_ok_iccp_303(),seq![201u8,73u8,63u8,202u8,75u8,151u8],1nat,true,false)==Ok(seq![51914u16,19275u16,38807u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_anc_ok_iccp_303(),a_anc_ok_iccp_303(),colour_anc_ok_iccp_303(),1nat,0nat,true,false)==Ok(seq![51914u16,19275u16,38807u16,65535u16]));
    assert(((seq![51914u16,19275u16,38807u16,65535u16][0] as nat+128)/257) as u8==202u8) by (compute);
    assert(((seq![51914u16,19275u16,38807u16,65535u16][1] as nat+128)/257) as u8==75u8) by (compute);
    assert(((seq![51914u16,19275u16,38807u16,65535u16][2] as nat+128)/257) as u8==151u8) by (compute);
    assert(((seq![51914u16,19275u16,38807u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixel_anc_ok_iccp_303_2() ensures sm::image(raw_anc_ok_iccp_303(),a_anc_ok_iccp_303(),colour_anc_ok_iccp_303(),false)[8]==225u8,sm::image(raw_anc_ok_iccp_303(),a_anc_ok_iccp_303(),colour_anc_ok_iccp_303(),false)[9]==26u8,sm::image(raw_anc_ok_iccp_303(),a_anc_ok_iccp_303(),colour_anc_ok_iccp_303(),false)[10]==23u8,sm::image(raw_anc_ok_iccp_303(),a_anc_ok_iccp_303(),colour_anc_ok_iccp_303(),false)[11]==255u8,
{
    colour::depth_values(8u8);
    row_anc_ok_iccp_303_0_1();
    assert(crate::paint_model::source_cell(a_anc_ok_iccp_303(),0nat,1nat)==(0nat,0nat,1nat)) by (compute);
    assert(colour::pixel(colour_anc_ok_iccp_303(),seq![225u8,26u8,23u8,252u8,246u8,129u8],0nat,true,false)==Ok(seq![57825u16,6682u16,5911u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_anc_ok_iccp_303(),a_anc_ok_iccp_303(),colour_anc_ok_iccp_303(),0nat,1nat,true,false)==Ok(seq![57825u16,6682u16,5911u16,65535u16]));
    assert(((seq![57825u16,6682u16,5911u16,65535u16][0] as nat+128)/257) as u8==225u8) by (compute);
    assert(((seq![57825u16,6682u16,5911u16,65535u16][1] as nat+128)/257) as u8==26u8) by (compute);
    assert(((seq![57825u16,6682u16,5911u16,65535u16][2] as nat+128)/257) as u8==23u8) by (compute);
    assert(((seq![57825u16,6682u16,5911u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixel_anc_ok_iccp_303_3() ensures sm::image(raw_anc_ok_iccp_303(),a_anc_ok_iccp_303(),colour_anc_ok_iccp_303(),false)[12]==252u8,sm::image(raw_anc_ok_iccp_303(),a_anc_ok_iccp_303(),colour_anc_ok_iccp_303(),false)[13]==246u8,sm::image(raw_anc_ok_iccp_303(),a_anc_ok_iccp_303(),colour_anc_ok_iccp_303(),false)[14]==129u8,sm::image(raw_anc_ok_iccp_303(),a_anc_ok_iccp_303(),colour_anc_ok_iccp_303(),false)[15]==255u8,
{
    colour::depth_values(8u8);
    row_anc_ok_iccp_303_0_1();
    assert(crate::paint_model::source_cell(a_anc_ok_iccp_303(),1nat,1nat)==(0nat,1nat,1nat)) by (compute);
    assert(colour::pixel(colour_anc_ok_iccp_303(),seq![225u8,26u8,23u8,252u8,246u8,129u8],1nat,true,false)==Ok(seq![64764u16,63222u16,33153u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_anc_ok_iccp_303(),a_anc_ok_iccp_303(),colour_anc_ok_iccp_303(),1nat,1nat,true,false)==Ok(seq![64764u16,63222u16,33153u16,65535u16]));
    assert(((seq![64764u16,63222u16,33153u16,65535u16][0] as nat+128)/257) as u8==252u8) by (compute);
    assert(((seq![64764u16,63222u16,33153u16,65535u16][1] as nat+128)/257) as u8==246u8) by (compute);
    assert(((seq![64764u16,63222u16,33153u16,65535u16][2] as nat+128)/257) as u8==129u8) by (compute);
    assert(((seq![64764u16,63222u16,33153u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixels_anc_ok_iccp_303() ensures sm::image(raw_anc_ok_iccp_303(),a_anc_ok_iccp_303(),colour_anc_ok_iccp_303(),false)==expected_anc_ok_iccp_303(),
{
    pixel_anc_ok_iccp_303_0();
    pixel_anc_ok_iccp_303_1();
    pixel_anc_ok_iccp_303_2();
    pixel_anc_ok_iccp_303_3();
    assert(a_anc_ok_iccp_303().width==2nat && a_anc_ok_iccp_303().height==2nat) by (compute);
    assert(sm::image(raw_anc_ok_iccp_303(),a_anc_ok_iccp_303(),colour_anc_ok_iccp_303(),false).len()==16nat);
    assert(expected_anc_ok_iccp_303().len()==16nat) by (compute);
    assert forall|j: int| 0<=j<16 implies (#[trigger] sm::image(raw_anc_ok_iccp_303(),a_anc_ok_iccp_303(),colour_anc_ok_iccp_303(),false)[j])==expected_anc_ok_iccp_303()[j] by {
        if j==0 {assert(expected_anc_ok_iccp_303()[0]==201u8) by (compute);}
        else if j==1 {assert(expected_anc_ok_iccp_303()[1]==73u8) by (compute);}
        else if j==2 {assert(expected_anc_ok_iccp_303()[2]==63u8) by (compute);}
        else if j==3 {assert(expected_anc_ok_iccp_303()[3]==255u8) by (compute);}
        else if j==4 {assert(expected_anc_ok_iccp_303()[4]==202u8) by (compute);}
        else if j==5 {assert(expected_anc_ok_iccp_303()[5]==75u8) by (compute);}
        else if j==6 {assert(expected_anc_ok_iccp_303()[6]==151u8) by (compute);}
        else if j==7 {assert(expected_anc_ok_iccp_303()[7]==255u8) by (compute);}
        else if j==8 {assert(expected_anc_ok_iccp_303()[8]==225u8) by (compute);}
        else if j==9 {assert(expected_anc_ok_iccp_303()[9]==26u8) by (compute);}
        else if j==10 {assert(expected_anc_ok_iccp_303()[10]==23u8) by (compute);}
        else if j==11 {assert(expected_anc_ok_iccp_303()[11]==255u8) by (compute);}
        else if j==12 {assert(expected_anc_ok_iccp_303()[12]==252u8) by (compute);}
        else if j==13 {assert(expected_anc_ok_iccp_303()[13]==246u8) by (compute);}
        else if j==14 {assert(expected_anc_ok_iccp_303()[14]==129u8) by (compute);}
        else {assert(j==15);assert(expected_anc_ok_iccp_303()[j]==255u8) by (compute);}
    }
    assert(sm::image(raw_anc_ok_iccp_303(),a_anc_ok_iccp_303(),colour_anc_ok_iccp_303(),false)=~=expected_anc_ok_iccp_303());
}
pub open spec fn expected_value_anc_ok_iccp_303(value: sm::Value) -> bool {value.width==2u32 && value.height==2u32 && value.pixels==expected_anc_ok_iccp_303()}
#[verifier::spinoff_prover]
pub proof fn model_complete_anc_ok_iccp_303()
    ensures match sm::MS(bytes_anc_ok_iccp_303()) {sm::Outcome::Done(value)=>expected_value_anc_ok_iccp_303(value),_=>false},
{
    hide(m::MW_p);hide(zm::Inflate);hide(bytes_anc_ok_iccp_303);hide(z_anc_ok_iccp_303);hide(raw_anc_ok_iccp_303);hide(expected_anc_ok_iccp_303);
    assert(bytes_anc_ok_iccp_303().len()==394nat) by (compute);
    raster_anc_ok_iccp_303();inflate_anc_ok_iccp_303();pixels_anc_ok_iccp_303();
    match m::MW_p(bytes_anc_ok_iccp_303(),m::strict()) {m::Outcome::Done(doc,_)=>{
        assert(colour::from_doc(doc,bytes_anc_ok_iccp_303())==colour_anc_ok_iccp_303());
        assert(crate::paint_model::default_raster(doc,STRICT)==a_anc_ok_iccp_303());
    },_=>{}}
    assert(r::raw(a_anc_ok_iccp_303())==14nat) by (compute);
    assert(r::inflated(z_anc_ok_iccp_303(),a_anc_ok_iccp_303(),STRICT).out==raw_anc_ok_iccp_303());
    compose_complete(bytes_anc_ok_iccp_303(),h_anc_ok_iccp_303(),z_anc_ok_iccp_303(),0nat,colour_anc_ok_iccp_303(),expected_anc_ok_iccp_303());
}
pub fn public_complete_anc_ok_iccp_303() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_anc_ok_iccp_303(),false),result),se::eq(bytes_anc_ok_iccp_303(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_anc_ok_iccp_303(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,1u8,47u8,105u8,67u8,67u8,80u8,115u8,82u8,71u8,66u8,32u8,73u8,69u8,67u8,54u8,49u8,57u8,54u8,54u8,45u8,50u8,46u8,49u8,0u8,0u8,120u8,156u8,99u8,96u8,100u8,98u8,102u8,97u8,101u8,99u8,231u8,224u8,228u8,226u8,230u8,225u8,229u8,227u8,23u8,16u8,20u8,18u8,22u8,17u8,21u8,19u8,151u8,144u8,148u8,146u8,150u8,145u8,149u8,147u8,87u8,80u8,84u8,82u8,86u8,81u8,85u8,83u8,215u8,208u8,212u8,210u8,214u8,209u8,213u8,211u8,55u8,48u8,52u8,50u8,54u8,49u8,53u8,51u8,183u8,176u8,180u8,178u8,182u8,177u8,181u8,179u8,119u8,112u8,116u8,114u8,118u8,113u8,117u8,115u8,247u8,240u8,244u8,242u8,246u8,241u8,245u8,243u8,15u8,8u8,12u8,10u8,14u8,9u8,13u8,11u8,143u8,136u8,140u8,138u8,142u8,137u8,141u8,139u8,79u8,72u8,76u8,74u8,78u8,73u8,77u8,75u8,207u8,200u8,204u8,202u8,206u8,201u8,205u8,203u8,47u8,40u8,44u8,42u8,46u8,41u8,45u8,43u8,175u8,168u8,172u8,170u8,174u8,169u8,173u8,171u8,111u8,104u8,108u8,106u8,110u8,105u8,109u8,107u8,239u8,232u8,236u8,234u8,238u8,233u8,237u8,235u8,159u8,48u8,113u8,210u8,228u8,41u8,83u8,167u8,77u8,159u8,49u8,115u8,214u8,236u8,57u8,115u8,231u8,205u8,95u8,176u8,112u8,209u8,226u8,37u8,75u8,151u8,45u8,95u8,177u8,114u8,213u8,234u8,53u8,107u8,215u8,173u8,223u8,176u8,113u8,211u8,230u8,45u8,91u8,183u8,109u8,223u8,177u8,115u8,215u8,238u8,61u8,123u8,247u8,237u8,63u8,112u8,240u8,208u8,225u8,35u8,71u8,143u8,29u8,63u8,113u8,242u8,212u8,233u8,51u8,103u8,207u8,157u8,191u8,112u8,241u8,210u8,229u8,43u8,87u8,175u8,93u8,191u8,113u8,243u8,214u8,237u8,59u8,119u8,239u8,221u8,127u8,240u8,240u8,209u8,227u8,39u8,79u8,159u8,61u8,127u8,241u8,242u8,213u8,235u8,55u8,111u8,223u8,189u8,255u8,240u8,241u8,211u8,231u8,47u8,95u8,191u8,125u8,255u8,241u8,243u8,215u8,239u8,63u8,127u8,255u8,253u8,103u8,24u8,245u8,255u8,136u8,246u8,63u8,0u8,160u8,98u8,126u8,144u8,16u8,123u8,222u8,207u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_anc_ok_iccp_303());model_complete_anc_ok_iccp_303();}
    crate::png::try_decode_still(&b)
}
// anc-ok/tIME-7-after-idat; frozen fixture SHA256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
pub open spec fn bytes_anc_ok_time_7_after_idat() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,7u8,116u8,73u8,77u8,69u8,7u8,234u8,11u8,14u8,23u8,59u8,60u8,230u8,176u8,242u8,44u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn h_anc_ok_time_7_after_idat() -> crate::model::Ihdr {crate::model::Ihdr {width:2u32,height:2u32,depth:8u8,color_type:2u8,interlace:0u8}}
spec fn a_anc_ok_time_7_after_idat() -> r::Raster {r::shape(h_anc_ok_time_7_after_idat(),2nat,2nat,0nat,STRICT)}
spec fn colour_anc_ok_time_7_after_idat() -> crate::colour_model::Context {crate::colour_model::Context {ct:2u8,depth:8u8,plte:seq![],trns:None}}
pub open spec fn expected_anc_ok_time_7_after_idat() -> Seq<u8> {seq![201u8,73u8,63u8,255u8,202u8,75u8,151u8,255u8,225u8,26u8,23u8,255u8,252u8,246u8,129u8,255u8]}
spec fn z_anc_ok_time_7_after_idat() -> Seq<u8> {seq![120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8]}
spec fn raw_anc_ok_time_7_after_idat() -> Seq<u8> {seq![0u8,201u8,73u8,63u8,202u8,75u8,151u8,1u8,225u8,26u8,23u8,27u8,220u8,106u8]}
spec fn ts_anc_ok_time_7_after_idat() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:36nat,before:1nat,after:2nat,symbol:201nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:36nat,next:44nat,before:2nat,after:3nat,symbol:73nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:44nat,next:52nat,before:3nat,after:4nat,symbol:63nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:52nat,next:61nat,before:4nat,after:5nat,symbol:202nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:61nat,next:69nat,before:5nat,after:6nat,symbol:75nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:69nat,next:78nat,before:6nat,after:7nat,symbol:151nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:78nat,next:86nat,before:7nat,after:8nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:86nat,next:95nat,before:8nat,after:9nat,symbol:225nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:95nat,next:103nat,before:9nat,after:10nat,symbol:26nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:103nat,next:111nat,before:10nat,after:11nat,symbol:23nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:111nat,next:119nat,before:11nat,after:12nat,symbol:27nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:119nat,next:128nat,before:12nat,after:13nat,symbol:220nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:128nat,next:136nat,before:13nat,after:14nat,symbol:106nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:136nat,next:143nat,before:14nat,after:14nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_anc_ok_time_7_after_idat_0() ensures t::trace(z_anc_ok_time_7_after_idat(),raw_anc_ok_time_7_after_idat(),ts_anc_ok_time_7_after_idat(),0nat,15nat),
{
    hide(t::valid_step);assert(t::trace(z_anc_ok_time_7_after_idat(),raw_anc_ok_time_7_after_idat(),ts_anc_ok_time_7_after_idat(),0nat,15nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn trace_anc_ok_time_7_after_idat() ensures t::trace(z_anc_ok_time_7_after_idat(),raw_anc_ok_time_7_after_idat(),ts_anc_ok_time_7_after_idat(),0,15nat),
{
    hide(z_anc_ok_time_7_after_idat);hide(raw_anc_ok_time_7_after_idat);hide(ts_anc_ok_time_7_after_idat);hide(t::trace);
    trace_anc_ok_time_7_after_idat_0();
    assert(t::trace(z_anc_ok_time_7_after_idat(),raw_anc_ok_time_7_after_idat(),ts_anc_ok_time_7_after_idat(),0,15nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_anc_ok_time_7_after_idat() ensures z_anc_ok_time_7_after_idat().len()==22nat,
        raw_anc_ok_time_7_after_idat().len()==14nat,
        z_anc_ok_time_7_after_idat()[0]==120u8,
        z_anc_ok_time_7_after_idat()[1]==156u8,
        zm::field_value(z_anc_ok_time_7_after_idat(),16,1)==1nat,
        zm::field_value(z_anc_ok_time_7_after_idat(),17,2)==1nat,
        zm::be_value(z_anc_ok_time_7_after_idat(),18nat,4)==675349874nat,
        ts_anc_ok_time_7_after_idat().len()==15nat,
        ts_anc_ok_time_7_after_idat()[0].pos==19nat,
        ts_anc_ok_time_7_after_idat()[0].before==0nat,
        ts_anc_ok_time_7_after_idat().last().next==143nat,
{
    assert(z_anc_ok_time_7_after_idat().len()==22nat) by (compute);
    assert(raw_anc_ok_time_7_after_idat().len()==14nat) by (compute);
    assert(z_anc_ok_time_7_after_idat()[0]==120u8) by (compute);
    assert(z_anc_ok_time_7_after_idat()[1]==156u8) by (compute);
    assert(zm::field_value(z_anc_ok_time_7_after_idat(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_anc_ok_time_7_after_idat(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_anc_ok_time_7_after_idat(),18nat,4)==675349874nat) by (compute);
    assert(ts_anc_ok_time_7_after_idat().len()==15nat) by (compute);
    assert(ts_anc_ok_time_7_after_idat()[0].pos==19nat) by (compute);
    assert(ts_anc_ok_time_7_after_idat()[0].before==0nat) by (compute);
    assert(ts_anc_ok_time_7_after_idat().last().next==143nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_anc_ok_time_7_after_idat() ensures zm::Adler(raw_anc_ok_time_7_after_idat())==675349874nat,
{
    hide(raw_anc_ok_time_7_after_idat);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_anc_ok_time_7_after_idat().len()==14nat) by (compute);
    assert(c::sums(raw_anc_ok_time_7_after_idat(),0,14nat)==(1393nat,10305nat)) by (compute);
    c::sums_correct(raw_anc_ok_time_7_after_idat(),0,14nat);assert(raw_anc_ok_time_7_after_idat().subrange(0,14)=~=raw_anc_ok_time_7_after_idat());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((10305nat%65521)*(u16::MAX as nat+1)+(1+1393nat)%65521==675349874nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_anc_ok_time_7_after_idat() ensures zm::Inflate(z_anc_ok_time_7_after_idat(),14nat,true,false)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_anc_ok_time_7_after_idat(),r2:false}),raw_anc_ok_time_7_after_idat().len()==14nat,
{
    hide(z_anc_ok_time_7_after_idat);hide(raw_anc_ok_time_7_after_idat);hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);hide(ts_anc_ok_time_7_after_idat);
    shape_anc_ok_time_7_after_idat();checksum_anc_ok_time_7_after_idat();
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_anc_ok_time_7_after_idat();ms::fixed_stream(z_anc_ok_time_7_after_idat(),raw_anc_ok_time_7_after_idat(),ts_anc_ok_time_7_after_idat(),14nat,156nat,675349874nat,675349874nat);
}
#[verifier::spinoff_prover]
proof fn walk_anc_ok_time_7_after_idat(b: Seq<u8>)
    requires b.len()==98,
        crate::model::mh(b)==Ok((h_anc_ok_time_7_after_idat(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:22nat,next:67nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,73u8,77u8,69u8],data:75nat,len:7nat,next:86nat,crc_ok:true,synthetic:false})),
        b.subrange(75,82)[0]==7u8,
        b.subrange(75,82)[1]==234u8,
        b.subrange(75,82)[2]==11u8,
        b.subrange(75,82)[3]==14u8,
        b.subrange(75,82)[4]==23u8,
        b.subrange(75,82)[5]==59u8,
        b.subrange(75,82)[6]==60u8,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,86nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:94nat,len:0nat,next:98nat,crc_ok:true,synthetic:false})),
        crate::metadata_model::parse(crate::walk_model::Name::Time,b.subrange(75,82),2u8,8u8,0nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Time(2026u16,11u8,14u8,23u8,59u8,60u8)),
        b.subrange(41,63)==z_anc_ok_time_7_after_idat(),
    ensures r::default_case(b,h_anc_ok_time_7_after_idat(),z_anc_ok_time_7_after_idat(),0nat),match m::MW_p(b,m::strict()) {m::Outcome::Done(doc,_)=>doc.plte==None && doc.trns==None,_=>false},
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(m::store);hide(crate::metadata_model::parse);
    assert forall|doc: m::Doc,value: crate::metadata_model::Value,base: nat|
        (#[trigger] m::store(doc,value,base)).ihdr==doc.ihdr && m::store(doc,value,base).plte==doc.plte
        && m::store(doc,value,base).trns==doc.trns && m::store(doc,value,base).actl==doc.actl
        && m::store(doc,value,base).idat==doc.idat && m::store(doc,value,base).frames==doc.frames by {
        stored_structure(doc,value,base);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,73u8,77u8,69u8])==crate::walk_model::Name::Time) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,73u8,77u8,69u8])==crate::walk_model::Name::Time) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    reveal_with_fuel(m::drive,4);
    reveal_with_fuel(m::concat_ranges,3);
}
#[verifier::spinoff_prover]
proof fn scan_anc_ok_time_7_after_idat() ensures r::scan(raw_anc_ok_time_7_after_idat(),a_anc_ok_time_7_after_idat(),0)==Ok(()),
{
    hide(raw_anc_ok_time_7_after_idat);assert(raw_anc_ok_time_7_after_idat().len()==14nat) by (compute);assert(r::count(a_anc_ok_time_7_after_idat())==1nat) by (compute);
    assert(r::pw(a_anc_ok_time_7_after_idat(),0nat)==2nat && r::ph(a_anc_ok_time_7_after_idat(),0nat)==2nat && r::rb(a_anc_ok_time_7_after_idat(),0nat)==6nat && r::base(a_anc_ok_time_7_after_idat(),0nat)==0nat) by (compute);
    assert(raw_anc_ok_time_7_after_idat()[0]==0u8) by (compute);
    assert(r::row_result(raw_anc_ok_time_7_after_idat(),a_anc_ok_time_7_after_idat(),0nat,0nat)==Ok(()));
    assert(raw_anc_ok_time_7_after_idat()[7]==1u8) by (compute);
    assert(r::row_result(raw_anc_ok_time_7_after_idat(),a_anc_ok_time_7_after_idat(),0nat,1nat)==Ok(()));
    r::scan_step(raw_anc_ok_time_7_after_idat(),a_anc_ok_time_7_after_idat(),1nat);
    r::pass_step(raw_anc_ok_time_7_after_idat(),a_anc_ok_time_7_after_idat(),0nat,2nat);
    r::pass_step(raw_anc_ok_time_7_after_idat(),a_anc_ok_time_7_after_idat(),0nat,1nat);
    r::pass_step(raw_anc_ok_time_7_after_idat(),a_anc_ok_time_7_after_idat(),0nat,0nat);
    r::scan_step(raw_anc_ok_time_7_after_idat(),a_anc_ok_time_7_after_idat(),0nat);
}
#[verifier::spinoff_prover]
proof fn row_anc_ok_time_7_after_idat_0_0() ensures r::line(raw_anc_ok_time_7_after_idat(),a_anc_ok_time_7_after_idat(),0nat,0nat)==seq![201u8,73u8,63u8,202u8,75u8,151u8],
{
    hide(raw_anc_ok_time_7_after_idat);
    assert(raw_anc_ok_time_7_after_idat().len()==14nat) by (compute);
    assert(raw_anc_ok_time_7_after_idat()[0]==0u8) by (compute);
    assert(r::base(a_anc_ok_time_7_after_idat(),0nat)==0nat && r::rb(a_anc_ok_time_7_after_idat(),0nat)==6nat && f::bpp(r::bits(a_anc_ok_time_7_after_idat()))==3nat) by (compute);
    assert(f::row_data(raw_anc_ok_time_7_after_idat(),0nat,6nat,0nat)==seq![201u8,73u8,63u8,202u8,75u8,151u8]) by (compute);
    f::pass_row_equation(raw_anc_ok_time_7_after_idat(),0nat,6nat,3nat,0nat);
    assert(f::previous_row(raw_anc_ok_time_7_after_idat(),0nat,6nat,3nat,0nat)==None);
    assert(f::encoded_row(0u8,f::row_data(raw_anc_ok_time_7_after_idat(),0nat,6nat,0nat),None,3nat,seq![201u8,73u8,63u8,202u8,75u8,151u8],0,6nat)) by (compute);
    f::checked_row_hint(0u8,f::row_data(raw_anc_ok_time_7_after_idat(),0nat,6nat,0nat),None,3nat,seq![201u8,73u8,63u8,202u8,75u8,151u8]);
}
#[verifier::spinoff_prover]
proof fn row_anc_ok_time_7_after_idat_0_1() ensures r::line(raw_anc_ok_time_7_after_idat(),a_anc_ok_time_7_after_idat(),0nat,1nat)==seq![225u8,26u8,23u8,252u8,246u8,129u8],
{
    hide(raw_anc_ok_time_7_after_idat);
    assert(raw_anc_ok_time_7_after_idat().len()==14nat) by (compute);
    assert(raw_anc_ok_time_7_after_idat()[7]==1u8) by (compute);
    assert(r::base(a_anc_ok_time_7_after_idat(),0nat)==0nat && r::rb(a_anc_ok_time_7_after_idat(),0nat)==6nat && f::bpp(r::bits(a_anc_ok_time_7_after_idat()))==3nat) by (compute);
    assert(f::row_data(raw_anc_ok_time_7_after_idat(),0nat,6nat,1nat)==seq![225u8,26u8,23u8,27u8,220u8,106u8]) by (compute);
    row_anc_ok_time_7_after_idat_0_0();
    f::pass_row_equation(raw_anc_ok_time_7_after_idat(),0nat,6nat,3nat,1nat);
    assert(f::previous_row(raw_anc_ok_time_7_after_idat(),0nat,6nat,3nat,1nat)==Some(seq![201u8,73u8,63u8,202u8,75u8,151u8]));
    assert(f::encoded_row(1u8,f::row_data(raw_anc_ok_time_7_after_idat(),0nat,6nat,1nat),Some(seq![201u8,73u8,63u8,202u8,75u8,151u8]),3nat,seq![225u8,26u8,23u8,252u8,246u8,129u8],0,6nat)) by (compute);
    f::checked_row_hint(1u8,f::row_data(raw_anc_ok_time_7_after_idat(),0nat,6nat,1nat),Some(seq![201u8,73u8,63u8,202u8,75u8,151u8]),3nat,seq![225u8,26u8,23u8,252u8,246u8,129u8]);
}
#[verifier::spinoff_prover]
proof fn raster_anc_ok_time_7_after_idat()
    ensures r::default_case(bytes_anc_ok_time_7_after_idat(),h_anc_ok_time_7_after_idat(),z_anc_ok_time_7_after_idat(),0nat),match m::MW_p(bytes_anc_ok_time_7_after_idat(),m::strict()) {m::Outcome::Done(doc,_)=>doc.plte==None && doc.trns==None,_=>false},r::data(z_anc_ok_time_7_after_idat(),a_anc_ok_time_7_after_idat(),STRICT)==Some(Ok(())),
{
    hide(bytes_anc_ok_time_7_after_idat);hide(z_anc_ok_time_7_after_idat);hide(raw_anc_ok_time_7_after_idat);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    assert(bytes_anc_ok_time_7_after_idat().len()==98nat) by (compute);
    assert(crate::model::mh(bytes_anc_ok_time_7_after_idat())==Ok((h_anc_ok_time_7_after_idat(),33nat))) by {reveal(crate::model::mh);assert(crate::model::mh(bytes_anc_ok_time_7_after_idat())==Ok((h_anc_ok_time_7_after_idat(),33nat))) by (compute);}
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_ok_time_7_after_idat(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:22nat,next:67nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_ok_time_7_after_idat(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:22nat,next:67nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_ok_time_7_after_idat(),67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,73u8,77u8,69u8],data:75nat,len:7nat,next:86nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_ok_time_7_after_idat(),67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,73u8,77u8,69u8],data:75nat,len:7nat,next:86nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,73u8,77u8,69u8])==crate::walk_model::Name::Time) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,73u8,77u8,69u8])==crate::walk_model::Name::Time) by (compute);
        }
        assert(bytes_anc_ok_time_7_after_idat().subrange(75,82)[0]==7u8) by (compute);
        assert(bytes_anc_ok_time_7_after_idat().subrange(75,82)[1]==234u8) by (compute);
        assert(bytes_anc_ok_time_7_after_idat().subrange(75,82)[2]==11u8) by (compute);
        assert(bytes_anc_ok_time_7_after_idat().subrange(75,82)[3]==14u8) by (compute);
        assert(bytes_anc_ok_time_7_after_idat().subrange(75,82)[4]==23u8) by (compute);
        assert(bytes_anc_ok_time_7_after_idat().subrange(75,82)[5]==59u8) by (compute);
        assert(bytes_anc_ok_time_7_after_idat().subrange(75,82)[6]==60u8) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_ok_time_7_after_idat(),86nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:94nat,len:0nat,next:98nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_ok_time_7_after_idat(),86nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:94nat,len:0nat,next:98nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::metadata_model::parse(crate::walk_model::Name::Time,bytes_anc_ok_time_7_after_idat().subrange(75,82),2u8,8u8,0nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Time(2026u16,11u8,14u8,23u8,59u8,60u8))) by {
            reveal(crate::metadata_model::parse);
            assert(crate::metadata_model::parse(crate::walk_model::Name::Time,bytes_anc_ok_time_7_after_idat().subrange(75,82),2u8,8u8,0nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Time(2026u16,11u8,14u8,23u8,59u8,60u8))) by (compute);
        }
    assert(bytes_anc_ok_time_7_after_idat().subrange(41,63)==z_anc_ok_time_7_after_idat()) by (compute);
    walk_anc_ok_time_7_after_idat(bytes_anc_ok_time_7_after_idat());
    inflate_anc_ok_time_7_after_idat();scan_anc_ok_time_7_after_idat();
    assert(r::raw(a_anc_ok_time_7_after_idat())==14nat) by (compute);
    assert(r::data(z_anc_ok_time_7_after_idat(),a_anc_ok_time_7_after_idat(),STRICT)==Some(Ok(()))) by {reveal(r::data);};
}
#[verifier::spinoff_prover]
proof fn pixel_anc_ok_time_7_after_idat_0() ensures sm::image(raw_anc_ok_time_7_after_idat(),a_anc_ok_time_7_after_idat(),colour_anc_ok_time_7_after_idat(),false)[0]==201u8,sm::image(raw_anc_ok_time_7_after_idat(),a_anc_ok_time_7_after_idat(),colour_anc_ok_time_7_after_idat(),false)[1]==73u8,sm::image(raw_anc_ok_time_7_after_idat(),a_anc_ok_time_7_after_idat(),colour_anc_ok_time_7_after_idat(),false)[2]==63u8,sm::image(raw_anc_ok_time_7_after_idat(),a_anc_ok_time_7_after_idat(),colour_anc_ok_time_7_after_idat(),false)[3]==255u8,
{
    colour::depth_values(8u8);
    row_anc_ok_time_7_after_idat_0_0();
    assert(crate::paint_model::source_cell(a_anc_ok_time_7_after_idat(),0nat,0nat)==(0nat,0nat,0nat)) by (compute);
    assert(colour::pixel(colour_anc_ok_time_7_after_idat(),seq![201u8,73u8,63u8,202u8,75u8,151u8],0nat,true,false)==Ok(seq![51657u16,18761u16,16191u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_anc_ok_time_7_after_idat(),a_anc_ok_time_7_after_idat(),colour_anc_ok_time_7_after_idat(),0nat,0nat,true,false)==Ok(seq![51657u16,18761u16,16191u16,65535u16]));
    assert(((seq![51657u16,18761u16,16191u16,65535u16][0] as nat+128)/257) as u8==201u8) by (compute);
    assert(((seq![51657u16,18761u16,16191u16,65535u16][1] as nat+128)/257) as u8==73u8) by (compute);
    assert(((seq![51657u16,18761u16,16191u16,65535u16][2] as nat+128)/257) as u8==63u8) by (compute);
    assert(((seq![51657u16,18761u16,16191u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixel_anc_ok_time_7_after_idat_1() ensures sm::image(raw_anc_ok_time_7_after_idat(),a_anc_ok_time_7_after_idat(),colour_anc_ok_time_7_after_idat(),false)[4]==202u8,sm::image(raw_anc_ok_time_7_after_idat(),a_anc_ok_time_7_after_idat(),colour_anc_ok_time_7_after_idat(),false)[5]==75u8,sm::image(raw_anc_ok_time_7_after_idat(),a_anc_ok_time_7_after_idat(),colour_anc_ok_time_7_after_idat(),false)[6]==151u8,sm::image(raw_anc_ok_time_7_after_idat(),a_anc_ok_time_7_after_idat(),colour_anc_ok_time_7_after_idat(),false)[7]==255u8,
{
    colour::depth_values(8u8);
    row_anc_ok_time_7_after_idat_0_0();
    assert(crate::paint_model::source_cell(a_anc_ok_time_7_after_idat(),1nat,0nat)==(0nat,1nat,0nat)) by (compute);
    assert(colour::pixel(colour_anc_ok_time_7_after_idat(),seq![201u8,73u8,63u8,202u8,75u8,151u8],1nat,true,false)==Ok(seq![51914u16,19275u16,38807u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_anc_ok_time_7_after_idat(),a_anc_ok_time_7_after_idat(),colour_anc_ok_time_7_after_idat(),1nat,0nat,true,false)==Ok(seq![51914u16,19275u16,38807u16,65535u16]));
    assert(((seq![51914u16,19275u16,38807u16,65535u16][0] as nat+128)/257) as u8==202u8) by (compute);
    assert(((seq![51914u16,19275u16,38807u16,65535u16][1] as nat+128)/257) as u8==75u8) by (compute);
    assert(((seq![51914u16,19275u16,38807u16,65535u16][2] as nat+128)/257) as u8==151u8) by (compute);
    assert(((seq![51914u16,19275u16,38807u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixel_anc_ok_time_7_after_idat_2() ensures sm::image(raw_anc_ok_time_7_after_idat(),a_anc_ok_time_7_after_idat(),colour_anc_ok_time_7_after_idat(),false)[8]==225u8,sm::image(raw_anc_ok_time_7_after_idat(),a_anc_ok_time_7_after_idat(),colour_anc_ok_time_7_after_idat(),false)[9]==26u8,sm::image(raw_anc_ok_time_7_after_idat(),a_anc_ok_time_7_after_idat(),colour_anc_ok_time_7_after_idat(),false)[10]==23u8,sm::image(raw_anc_ok_time_7_after_idat(),a_anc_ok_time_7_after_idat(),colour_anc_ok_time_7_after_idat(),false)[11]==255u8,
{
    colour::depth_values(8u8);
    row_anc_ok_time_7_after_idat_0_1();
    assert(crate::paint_model::source_cell(a_anc_ok_time_7_after_idat(),0nat,1nat)==(0nat,0nat,1nat)) by (compute);
    assert(colour::pixel(colour_anc_ok_time_7_after_idat(),seq![225u8,26u8,23u8,252u8,246u8,129u8],0nat,true,false)==Ok(seq![57825u16,6682u16,5911u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_anc_ok_time_7_after_idat(),a_anc_ok_time_7_after_idat(),colour_anc_ok_time_7_after_idat(),0nat,1nat,true,false)==Ok(seq![57825u16,6682u16,5911u16,65535u16]));
    assert(((seq![57825u16,6682u16,5911u16,65535u16][0] as nat+128)/257) as u8==225u8) by (compute);
    assert(((seq![57825u16,6682u16,5911u16,65535u16][1] as nat+128)/257) as u8==26u8) by (compute);
    assert(((seq![57825u16,6682u16,5911u16,65535u16][2] as nat+128)/257) as u8==23u8) by (compute);
    assert(((seq![57825u16,6682u16,5911u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixel_anc_ok_time_7_after_idat_3() ensures sm::image(raw_anc_ok_time_7_after_idat(),a_anc_ok_time_7_after_idat(),colour_anc_ok_time_7_after_idat(),false)[12]==252u8,sm::image(raw_anc_ok_time_7_after_idat(),a_anc_ok_time_7_after_idat(),colour_anc_ok_time_7_after_idat(),false)[13]==246u8,sm::image(raw_anc_ok_time_7_after_idat(),a_anc_ok_time_7_after_idat(),colour_anc_ok_time_7_after_idat(),false)[14]==129u8,sm::image(raw_anc_ok_time_7_after_idat(),a_anc_ok_time_7_after_idat(),colour_anc_ok_time_7_after_idat(),false)[15]==255u8,
{
    colour::depth_values(8u8);
    row_anc_ok_time_7_after_idat_0_1();
    assert(crate::paint_model::source_cell(a_anc_ok_time_7_after_idat(),1nat,1nat)==(0nat,1nat,1nat)) by (compute);
    assert(colour::pixel(colour_anc_ok_time_7_after_idat(),seq![225u8,26u8,23u8,252u8,246u8,129u8],1nat,true,false)==Ok(seq![64764u16,63222u16,33153u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_anc_ok_time_7_after_idat(),a_anc_ok_time_7_after_idat(),colour_anc_ok_time_7_after_idat(),1nat,1nat,true,false)==Ok(seq![64764u16,63222u16,33153u16,65535u16]));
    assert(((seq![64764u16,63222u16,33153u16,65535u16][0] as nat+128)/257) as u8==252u8) by (compute);
    assert(((seq![64764u16,63222u16,33153u16,65535u16][1] as nat+128)/257) as u8==246u8) by (compute);
    assert(((seq![64764u16,63222u16,33153u16,65535u16][2] as nat+128)/257) as u8==129u8) by (compute);
    assert(((seq![64764u16,63222u16,33153u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixels_anc_ok_time_7_after_idat() ensures sm::image(raw_anc_ok_time_7_after_idat(),a_anc_ok_time_7_after_idat(),colour_anc_ok_time_7_after_idat(),false)==expected_anc_ok_time_7_after_idat(),
{
    pixel_anc_ok_time_7_after_idat_0();
    pixel_anc_ok_time_7_after_idat_1();
    pixel_anc_ok_time_7_after_idat_2();
    pixel_anc_ok_time_7_after_idat_3();
    assert(a_anc_ok_time_7_after_idat().width==2nat && a_anc_ok_time_7_after_idat().height==2nat) by (compute);
    assert(sm::image(raw_anc_ok_time_7_after_idat(),a_anc_ok_time_7_after_idat(),colour_anc_ok_time_7_after_idat(),false).len()==16nat);
    assert(expected_anc_ok_time_7_after_idat().len()==16nat) by (compute);
    assert forall|j: int| 0<=j<16 implies (#[trigger] sm::image(raw_anc_ok_time_7_after_idat(),a_anc_ok_time_7_after_idat(),colour_anc_ok_time_7_after_idat(),false)[j])==expected_anc_ok_time_7_after_idat()[j] by {
        if j==0 {assert(expected_anc_ok_time_7_after_idat()[0]==201u8) by (compute);}
        else if j==1 {assert(expected_anc_ok_time_7_after_idat()[1]==73u8) by (compute);}
        else if j==2 {assert(expected_anc_ok_time_7_after_idat()[2]==63u8) by (compute);}
        else if j==3 {assert(expected_anc_ok_time_7_after_idat()[3]==255u8) by (compute);}
        else if j==4 {assert(expected_anc_ok_time_7_after_idat()[4]==202u8) by (compute);}
        else if j==5 {assert(expected_anc_ok_time_7_after_idat()[5]==75u8) by (compute);}
        else if j==6 {assert(expected_anc_ok_time_7_after_idat()[6]==151u8) by (compute);}
        else if j==7 {assert(expected_anc_ok_time_7_after_idat()[7]==255u8) by (compute);}
        else if j==8 {assert(expected_anc_ok_time_7_after_idat()[8]==225u8) by (compute);}
        else if j==9 {assert(expected_anc_ok_time_7_after_idat()[9]==26u8) by (compute);}
        else if j==10 {assert(expected_anc_ok_time_7_after_idat()[10]==23u8) by (compute);}
        else if j==11 {assert(expected_anc_ok_time_7_after_idat()[11]==255u8) by (compute);}
        else if j==12 {assert(expected_anc_ok_time_7_after_idat()[12]==252u8) by (compute);}
        else if j==13 {assert(expected_anc_ok_time_7_after_idat()[13]==246u8) by (compute);}
        else if j==14 {assert(expected_anc_ok_time_7_after_idat()[14]==129u8) by (compute);}
        else {assert(j==15);assert(expected_anc_ok_time_7_after_idat()[j]==255u8) by (compute);}
    }
    assert(sm::image(raw_anc_ok_time_7_after_idat(),a_anc_ok_time_7_after_idat(),colour_anc_ok_time_7_after_idat(),false)=~=expected_anc_ok_time_7_after_idat());
}
pub open spec fn expected_value_anc_ok_time_7_after_idat(value: sm::Value) -> bool {value.width==2u32 && value.height==2u32 && value.pixels==expected_anc_ok_time_7_after_idat()}
#[verifier::spinoff_prover]
pub proof fn model_complete_anc_ok_time_7_after_idat()
    ensures match sm::MS(bytes_anc_ok_time_7_after_idat()) {sm::Outcome::Done(value)=>expected_value_anc_ok_time_7_after_idat(value),_=>false},
{
    hide(m::MW_p);hide(zm::Inflate);hide(bytes_anc_ok_time_7_after_idat);hide(z_anc_ok_time_7_after_idat);hide(raw_anc_ok_time_7_after_idat);hide(expected_anc_ok_time_7_after_idat);
    assert(bytes_anc_ok_time_7_after_idat().len()==98nat) by (compute);
    raster_anc_ok_time_7_after_idat();inflate_anc_ok_time_7_after_idat();pixels_anc_ok_time_7_after_idat();
    match m::MW_p(bytes_anc_ok_time_7_after_idat(),m::strict()) {m::Outcome::Done(doc,_)=>{
        assert(colour::from_doc(doc,bytes_anc_ok_time_7_after_idat())==colour_anc_ok_time_7_after_idat());
        assert(crate::paint_model::default_raster(doc,STRICT)==a_anc_ok_time_7_after_idat());
    },_=>{}}
    assert(r::raw(a_anc_ok_time_7_after_idat())==14nat) by (compute);
    assert(r::inflated(z_anc_ok_time_7_after_idat(),a_anc_ok_time_7_after_idat(),STRICT).out==raw_anc_ok_time_7_after_idat());
    compose_complete(bytes_anc_ok_time_7_after_idat(),h_anc_ok_time_7_after_idat(),z_anc_ok_time_7_after_idat(),0nat,colour_anc_ok_time_7_after_idat(),expected_anc_ok_time_7_after_idat());
}
pub fn public_complete_anc_ok_time_7_after_idat() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_anc_ok_time_7_after_idat(),false),result),se::eq(bytes_anc_ok_time_7_after_idat(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_anc_ok_time_7_after_idat(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,7u8,116u8,73u8,77u8,69u8,7u8,234u8,11u8,14u8,23u8,59u8,60u8,230u8,176u8,242u8,44u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_anc_ok_time_7_after_idat());model_complete_anc_ok_time_7_after_idat();}
    crate::png::try_decode_still(&b)
}
// anc-ok/srgb-and-iccp-both; frozen fixture SHA256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
pub open spec fn bytes_anc_ok_srgb_and_iccp_both() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,1u8,115u8,82u8,71u8,66u8,1u8,217u8,201u8,44u8,127u8,0u8,0u8,1u8,31u8,105u8,67u8,67u8,80u8,120u8,0u8,0u8,120u8,156u8,99u8,96u8,100u8,98u8,102u8,97u8,101u8,99u8,231u8,224u8,228u8,226u8,230u8,225u8,229u8,227u8,23u8,16u8,20u8,18u8,22u8,17u8,21u8,19u8,151u8,144u8,148u8,146u8,150u8,145u8,149u8,147u8,87u8,80u8,84u8,82u8,86u8,81u8,85u8,83u8,215u8,208u8,212u8,210u8,214u8,209u8,213u8,211u8,55u8,48u8,52u8,50u8,54u8,49u8,53u8,51u8,183u8,176u8,180u8,178u8,182u8,177u8,181u8,179u8,119u8,112u8,116u8,114u8,118u8,113u8,117u8,115u8,247u8,240u8,244u8,242u8,246u8,241u8,245u8,243u8,15u8,8u8,12u8,10u8,14u8,9u8,13u8,11u8,143u8,136u8,140u8,138u8,142u8,137u8,141u8,139u8,79u8,72u8,76u8,74u8,78u8,73u8,77u8,75u8,207u8,200u8,204u8,202u8,206u8,201u8,205u8,203u8,47u8,40u8,44u8,42u8,46u8,41u8,45u8,43u8,175u8,168u8,172u8,170u8,174u8,169u8,173u8,171u8,111u8,104u8,108u8,106u8,110u8,105u8,109u8,107u8,239u8,232u8,236u8,234u8,238u8,233u8,237u8,235u8,159u8,48u8,113u8,210u8,228u8,41u8,83u8,167u8,77u8,159u8,49u8,115u8,214u8,236u8,57u8,115u8,231u8,205u8,95u8,176u8,112u8,209u8,226u8,37u8,75u8,151u8,45u8,95u8,177u8,114u8,213u8,234u8,53u8,107u8,215u8,173u8,223u8,176u8,113u8,211u8,230u8,45u8,91u8,183u8,109u8,223u8,177u8,115u8,215u8,238u8,61u8,123u8,247u8,237u8,63u8,112u8,240u8,208u8,225u8,35u8,71u8,143u8,29u8,63u8,113u8,242u8,212u8,233u8,51u8,103u8,207u8,157u8,191u8,112u8,241u8,210u8,229u8,43u8,87u8,175u8,93u8,191u8,113u8,243u8,214u8,237u8,59u8,119u8,239u8,221u8,127u8,240u8,240u8,209u8,227u8,39u8,79u8,159u8,61u8,127u8,241u8,242u8,213u8,235u8,55u8,111u8,223u8,189u8,255u8,240u8,241u8,211u8,231u8,47u8,95u8,191u8,125u8,255u8,241u8,243u8,215u8,239u8,63u8,127u8,255u8,253u8,103u8,24u8,245u8,255u8,136u8,246u8,63u8,0u8,160u8,98u8,126u8,144u8,17u8,127u8,182u8,55u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn h_anc_ok_srgb_and_iccp_both() -> crate::model::Ihdr {crate::model::Ihdr {width:2u32,height:2u32,depth:8u8,color_type:2u8,interlace:0u8}}
spec fn a_anc_ok_srgb_and_iccp_both() -> r::Raster {r::shape(h_anc_ok_srgb_and_iccp_both(),2nat,2nat,0nat,STRICT)}
spec fn colour_anc_ok_srgb_and_iccp_both() -> crate::colour_model::Context {crate::colour_model::Context {ct:2u8,depth:8u8,plte:seq![],trns:None}}
pub open spec fn expected_anc_ok_srgb_and_iccp_both() -> Seq<u8> {seq![201u8,73u8,63u8,255u8,202u8,75u8,151u8,255u8,225u8,26u8,23u8,255u8,252u8,246u8,129u8,255u8]}
spec fn z_anc_ok_srgb_and_iccp_both() -> Seq<u8> {seq![120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8]}
spec fn raw_anc_ok_srgb_and_iccp_both() -> Seq<u8> {seq![0u8,201u8,73u8,63u8,202u8,75u8,151u8,1u8,225u8,26u8,23u8,27u8,220u8,106u8]}
spec fn ts_anc_ok_srgb_and_iccp_both() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:36nat,before:1nat,after:2nat,symbol:201nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:36nat,next:44nat,before:2nat,after:3nat,symbol:73nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:44nat,next:52nat,before:3nat,after:4nat,symbol:63nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:52nat,next:61nat,before:4nat,after:5nat,symbol:202nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:61nat,next:69nat,before:5nat,after:6nat,symbol:75nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:69nat,next:78nat,before:6nat,after:7nat,symbol:151nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:78nat,next:86nat,before:7nat,after:8nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:86nat,next:95nat,before:8nat,after:9nat,symbol:225nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:95nat,next:103nat,before:9nat,after:10nat,symbol:26nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:103nat,next:111nat,before:10nat,after:11nat,symbol:23nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:111nat,next:119nat,before:11nat,after:12nat,symbol:27nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:119nat,next:128nat,before:12nat,after:13nat,symbol:220nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:128nat,next:136nat,before:13nat,after:14nat,symbol:106nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:136nat,next:143nat,before:14nat,after:14nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_anc_ok_srgb_and_iccp_both_0() ensures t::trace(z_anc_ok_srgb_and_iccp_both(),raw_anc_ok_srgb_and_iccp_both(),ts_anc_ok_srgb_and_iccp_both(),0nat,15nat),
{
    hide(t::valid_step);assert(t::trace(z_anc_ok_srgb_and_iccp_both(),raw_anc_ok_srgb_and_iccp_both(),ts_anc_ok_srgb_and_iccp_both(),0nat,15nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn trace_anc_ok_srgb_and_iccp_both() ensures t::trace(z_anc_ok_srgb_and_iccp_both(),raw_anc_ok_srgb_and_iccp_both(),ts_anc_ok_srgb_and_iccp_both(),0,15nat),
{
    hide(z_anc_ok_srgb_and_iccp_both);hide(raw_anc_ok_srgb_and_iccp_both);hide(ts_anc_ok_srgb_and_iccp_both);hide(t::trace);
    trace_anc_ok_srgb_and_iccp_both_0();
    assert(t::trace(z_anc_ok_srgb_and_iccp_both(),raw_anc_ok_srgb_and_iccp_both(),ts_anc_ok_srgb_and_iccp_both(),0,15nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_anc_ok_srgb_and_iccp_both() ensures z_anc_ok_srgb_and_iccp_both().len()==22nat,
        raw_anc_ok_srgb_and_iccp_both().len()==14nat,
        z_anc_ok_srgb_and_iccp_both()[0]==120u8,
        z_anc_ok_srgb_and_iccp_both()[1]==156u8,
        zm::field_value(z_anc_ok_srgb_and_iccp_both(),16,1)==1nat,
        zm::field_value(z_anc_ok_srgb_and_iccp_both(),17,2)==1nat,
        zm::be_value(z_anc_ok_srgb_and_iccp_both(),18nat,4)==675349874nat,
        ts_anc_ok_srgb_and_iccp_both().len()==15nat,
        ts_anc_ok_srgb_and_iccp_both()[0].pos==19nat,
        ts_anc_ok_srgb_and_iccp_both()[0].before==0nat,
        ts_anc_ok_srgb_and_iccp_both().last().next==143nat,
{
    assert(z_anc_ok_srgb_and_iccp_both().len()==22nat) by (compute);
    assert(raw_anc_ok_srgb_and_iccp_both().len()==14nat) by (compute);
    assert(z_anc_ok_srgb_and_iccp_both()[0]==120u8) by (compute);
    assert(z_anc_ok_srgb_and_iccp_both()[1]==156u8) by (compute);
    assert(zm::field_value(z_anc_ok_srgb_and_iccp_both(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_anc_ok_srgb_and_iccp_both(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_anc_ok_srgb_and_iccp_both(),18nat,4)==675349874nat) by (compute);
    assert(ts_anc_ok_srgb_and_iccp_both().len()==15nat) by (compute);
    assert(ts_anc_ok_srgb_and_iccp_both()[0].pos==19nat) by (compute);
    assert(ts_anc_ok_srgb_and_iccp_both()[0].before==0nat) by (compute);
    assert(ts_anc_ok_srgb_and_iccp_both().last().next==143nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_anc_ok_srgb_and_iccp_both() ensures zm::Adler(raw_anc_ok_srgb_and_iccp_both())==675349874nat,
{
    hide(raw_anc_ok_srgb_and_iccp_both);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_anc_ok_srgb_and_iccp_both().len()==14nat) by (compute);
    assert(c::sums(raw_anc_ok_srgb_and_iccp_both(),0,14nat)==(1393nat,10305nat)) by (compute);
    c::sums_correct(raw_anc_ok_srgb_and_iccp_both(),0,14nat);assert(raw_anc_ok_srgb_and_iccp_both().subrange(0,14)=~=raw_anc_ok_srgb_and_iccp_both());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((10305nat%65521)*(u16::MAX as nat+1)+(1+1393nat)%65521==675349874nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_anc_ok_srgb_and_iccp_both() ensures zm::Inflate(z_anc_ok_srgb_and_iccp_both(),14nat,true,false)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_anc_ok_srgb_and_iccp_both(),r2:false}),raw_anc_ok_srgb_and_iccp_both().len()==14nat,
{
    hide(z_anc_ok_srgb_and_iccp_both);hide(raw_anc_ok_srgb_and_iccp_both);hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);hide(ts_anc_ok_srgb_and_iccp_both);
    shape_anc_ok_srgb_and_iccp_both();checksum_anc_ok_srgb_and_iccp_both();
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_anc_ok_srgb_and_iccp_both();ms::fixed_stream(z_anc_ok_srgb_and_iccp_both(),raw_anc_ok_srgb_and_iccp_both(),ts_anc_ok_srgb_and_iccp_both(),14nat,156nat,675349874nat,675349874nat);
}
#[verifier::spinoff_prover]
proof fn crc_anc_ok_srgb_and_iccp_both_crc_46()
    ensures crate::model::crc(bytes_anc_ok_srgb_and_iccp_both().subrange(50,341))==293582391u32,
{
    hide(crate::model::crc_register);hide(crc_cert::fold);
    assert(crate::model::crc_register(bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).take(0))==0xffff_ffffu32) by {
        assert(bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).take(0) =~= Seq::<u8>::empty());
        reveal_with_fuel(crate::model::crc_register,1);
    }
    assert(crc_cert::fold(4294967295u32,bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).subrange(0,64))==3464954982u32) by (compute);
    crc_cert::suffix(bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).take(0),bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).subrange(0,64));
    assert(bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).take(0)+bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).subrange(0,64) =~= bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).take(64));
    assert(crate::model::crc_register(bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).take(64))==3464954982u32);
    assert(crc_cert::fold(3464954982u32,bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).subrange(64,128))==2467918098u32) by (compute);
    crc_cert::suffix(bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).take(64),bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).subrange(64,128));
    assert(bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).take(64)+bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).subrange(64,128) =~= bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).take(128));
    assert(crate::model::crc_register(bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).take(128))==2467918098u32);
    assert(crc_cert::fold(2467918098u32,bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).subrange(128,192))==2031984057u32) by (compute);
    crc_cert::suffix(bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).take(128),bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).subrange(128,192));
    assert(bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).take(128)+bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).subrange(128,192) =~= bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).take(192));
    assert(crate::model::crc_register(bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).take(192))==2031984057u32);
    assert(crc_cert::fold(2031984057u32,bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).subrange(192,256))==2801487409u32) by (compute);
    crc_cert::suffix(bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).take(192),bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).subrange(192,256));
    assert(bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).take(192)+bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).subrange(192,256) =~= bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).take(256));
    assert(crate::model::crc_register(bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).take(256))==2801487409u32);
    assert(crc_cert::fold(2801487409u32,bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).subrange(256,291))==4001384904u32) by (compute);
    crc_cert::suffix(bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).take(256),bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).subrange(256,291));
    assert(bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).take(256)+bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).subrange(256,291) =~= bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).take(291));
    assert(crate::model::crc_register(bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).take(291))==4001384904u32);
    assert(bytes_anc_ok_srgb_and_iccp_both().subrange(50,341).take(291) =~= bytes_anc_ok_srgb_and_iccp_both().subrange(50,341));
    assert((4001384904u32 ^ 0xffff_ffffu32)==293582391u32) by (compute);
}
#[verifier::spinoff_prover]
proof fn walk_anc_ok_srgb_and_iccp_both(b: Seq<u8>)
    requires b.len()==391,
        crate::model::mh(b)==Ok((h_anc_ok_srgb_and_iccp_both(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![115u8,82u8,71u8,66u8],data:41nat,len:1nat,next:46nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,46nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![105u8,67u8,67u8,80u8],data:54nat,len:287nat,next:345nat,crc_ok:true,synthetic:false})),
        crate::model::first_nul(b.subrange(54,341),0)==Some(1nat),
        b.subrange(54,341).subrange(0,1)==seq![120u8],
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,345nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:353nat,len:22nat,next:379nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,379nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:387nat,len:0nat,next:391nat,crc_ok:true,synthetic:false})),
        crate::metadata_model::parse(crate::walk_model::Name::Srgb,b.subrange(41,42),2u8,8u8,0nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Srgb(1u8)),
        b.subrange(54,341)[2]==0u8,
        crate::metadata_model::parse(crate::walk_model::Name::Iccp,b.subrange(54,341),2u8,8u8,0nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Iccp {name_len:1,profile_start:3,profile_len:284}),
        b.subrange(353,375)==z_anc_ok_srgb_and_iccp_both(),
    ensures r::default_case(b,h_anc_ok_srgb_and_iccp_both(),z_anc_ok_srgb_and_iccp_both(),0nat),match m::MW_p(b,m::strict()) {m::Outcome::Done(doc,_)=>doc.plte==None && doc.trns==None,_=>false},
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(m::store);hide(crate::metadata_model::parse);
    assert forall|doc: m::Doc,value: crate::metadata_model::Value,base: nat|
        (#[trigger] m::store(doc,value,base)).ihdr==doc.ihdr && m::store(doc,value,base).plte==doc.plte
        && m::store(doc,value,base).trns==doc.trns && m::store(doc,value,base).actl==doc.actl
        && m::store(doc,value,base).idat==doc.idat && m::store(doc,value,base).frames==doc.frames by {
        stored_structure(doc,value,base);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((115u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![115u8,82u8,71u8,66u8])==crate::walk_model::Name::Srgb) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![115u8,82u8,71u8,66u8])==crate::walk_model::Name::Srgb) by (compute);
        }
        assert((105u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![105u8,67u8,67u8,80u8])==crate::walk_model::Name::Iccp) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![105u8,67u8,67u8,80u8])==crate::walk_model::Name::Iccp) by (compute);
        }
        assert(crate::model::keyword(seq![120u8]));
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    reveal_with_fuel(m::drive,5);
    reveal_with_fuel(m::concat_ranges,3);
}
#[verifier::spinoff_prover]
proof fn scan_anc_ok_srgb_and_iccp_both() ensures r::scan(raw_anc_ok_srgb_and_iccp_both(),a_anc_ok_srgb_and_iccp_both(),0)==Ok(()),
{
    hide(raw_anc_ok_srgb_and_iccp_both);assert(raw_anc_ok_srgb_and_iccp_both().len()==14nat) by (compute);assert(r::count(a_anc_ok_srgb_and_iccp_both())==1nat) by (compute);
    assert(r::pw(a_anc_ok_srgb_and_iccp_both(),0nat)==2nat && r::ph(a_anc_ok_srgb_and_iccp_both(),0nat)==2nat && r::rb(a_anc_ok_srgb_and_iccp_both(),0nat)==6nat && r::base(a_anc_ok_srgb_and_iccp_both(),0nat)==0nat) by (compute);
    assert(raw_anc_ok_srgb_and_iccp_both()[0]==0u8) by (compute);
    assert(r::row_result(raw_anc_ok_srgb_and_iccp_both(),a_anc_ok_srgb_and_iccp_both(),0nat,0nat)==Ok(()));
    assert(raw_anc_ok_srgb_and_iccp_both()[7]==1u8) by (compute);
    assert(r::row_result(raw_anc_ok_srgb_and_iccp_both(),a_anc_ok_srgb_and_iccp_both(),0nat,1nat)==Ok(()));
    r::scan_step(raw_anc_ok_srgb_and_iccp_both(),a_anc_ok_srgb_and_iccp_both(),1nat);
    r::pass_step(raw_anc_ok_srgb_and_iccp_both(),a_anc_ok_srgb_and_iccp_both(),0nat,2nat);
    r::pass_step(raw_anc_ok_srgb_and_iccp_both(),a_anc_ok_srgb_and_iccp_both(),0nat,1nat);
    r::pass_step(raw_anc_ok_srgb_and_iccp_both(),a_anc_ok_srgb_and_iccp_both(),0nat,0nat);
    r::scan_step(raw_anc_ok_srgb_and_iccp_both(),a_anc_ok_srgb_and_iccp_both(),0nat);
}
#[verifier::spinoff_prover]
proof fn row_anc_ok_srgb_and_iccp_both_0_0() ensures r::line(raw_anc_ok_srgb_and_iccp_both(),a_anc_ok_srgb_and_iccp_both(),0nat,0nat)==seq![201u8,73u8,63u8,202u8,75u8,151u8],
{
    hide(raw_anc_ok_srgb_and_iccp_both);
    assert(raw_anc_ok_srgb_and_iccp_both().len()==14nat) by (compute);
    assert(raw_anc_ok_srgb_and_iccp_both()[0]==0u8) by (compute);
    assert(r::base(a_anc_ok_srgb_and_iccp_both(),0nat)==0nat && r::rb(a_anc_ok_srgb_and_iccp_both(),0nat)==6nat && f::bpp(r::bits(a_anc_ok_srgb_and_iccp_both()))==3nat) by (compute);
    assert(f::row_data(raw_anc_ok_srgb_and_iccp_both(),0nat,6nat,0nat)==seq![201u8,73u8,63u8,202u8,75u8,151u8]) by (compute);
    f::pass_row_equation(raw_anc_ok_srgb_and_iccp_both(),0nat,6nat,3nat,0nat);
    assert(f::previous_row(raw_anc_ok_srgb_and_iccp_both(),0nat,6nat,3nat,0nat)==None);
    assert(f::encoded_row(0u8,f::row_data(raw_anc_ok_srgb_and_iccp_both(),0nat,6nat,0nat),None,3nat,seq![201u8,73u8,63u8,202u8,75u8,151u8],0,6nat)) by (compute);
    f::checked_row_hint(0u8,f::row_data(raw_anc_ok_srgb_and_iccp_both(),0nat,6nat,0nat),None,3nat,seq![201u8,73u8,63u8,202u8,75u8,151u8]);
}
#[verifier::spinoff_prover]
proof fn row_anc_ok_srgb_and_iccp_both_0_1() ensures r::line(raw_anc_ok_srgb_and_iccp_both(),a_anc_ok_srgb_and_iccp_both(),0nat,1nat)==seq![225u8,26u8,23u8,252u8,246u8,129u8],
{
    hide(raw_anc_ok_srgb_and_iccp_both);
    assert(raw_anc_ok_srgb_and_iccp_both().len()==14nat) by (compute);
    assert(raw_anc_ok_srgb_and_iccp_both()[7]==1u8) by (compute);
    assert(r::base(a_anc_ok_srgb_and_iccp_both(),0nat)==0nat && r::rb(a_anc_ok_srgb_and_iccp_both(),0nat)==6nat && f::bpp(r::bits(a_anc_ok_srgb_and_iccp_both()))==3nat) by (compute);
    assert(f::row_data(raw_anc_ok_srgb_and_iccp_both(),0nat,6nat,1nat)==seq![225u8,26u8,23u8,27u8,220u8,106u8]) by (compute);
    row_anc_ok_srgb_and_iccp_both_0_0();
    f::pass_row_equation(raw_anc_ok_srgb_and_iccp_both(),0nat,6nat,3nat,1nat);
    assert(f::previous_row(raw_anc_ok_srgb_and_iccp_both(),0nat,6nat,3nat,1nat)==Some(seq![201u8,73u8,63u8,202u8,75u8,151u8]));
    assert(f::encoded_row(1u8,f::row_data(raw_anc_ok_srgb_and_iccp_both(),0nat,6nat,1nat),Some(seq![201u8,73u8,63u8,202u8,75u8,151u8]),3nat,seq![225u8,26u8,23u8,252u8,246u8,129u8],0,6nat)) by (compute);
    f::checked_row_hint(1u8,f::row_data(raw_anc_ok_srgb_and_iccp_both(),0nat,6nat,1nat),Some(seq![201u8,73u8,63u8,202u8,75u8,151u8]),3nat,seq![225u8,26u8,23u8,252u8,246u8,129u8]);
}
#[verifier::spinoff_prover]
proof fn raster_anc_ok_srgb_and_iccp_both()
    ensures r::default_case(bytes_anc_ok_srgb_and_iccp_both(),h_anc_ok_srgb_and_iccp_both(),z_anc_ok_srgb_and_iccp_both(),0nat),match m::MW_p(bytes_anc_ok_srgb_and_iccp_both(),m::strict()) {m::Outcome::Done(doc,_)=>doc.plte==None && doc.trns==None,_=>false},r::data(z_anc_ok_srgb_and_iccp_both(),a_anc_ok_srgb_and_iccp_both(),STRICT)==Some(Ok(())),
{
    hide(bytes_anc_ok_srgb_and_iccp_both);hide(z_anc_ok_srgb_and_iccp_both);hide(raw_anc_ok_srgb_and_iccp_both);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    assert(bytes_anc_ok_srgb_and_iccp_both().len()==391nat) by (compute);
    assert(crate::model::mh(bytes_anc_ok_srgb_and_iccp_both())==Ok((h_anc_ok_srgb_and_iccp_both(),33nat))) by {reveal(crate::model::mh);assert(crate::model::mh(bytes_anc_ok_srgb_and_iccp_both())==Ok((h_anc_ok_srgb_and_iccp_both(),33nat))) by (compute);}
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_ok_srgb_and_iccp_both(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![115u8,82u8,71u8,66u8],data:41nat,len:1nat,next:46nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_ok_srgb_and_iccp_both(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![115u8,82u8,71u8,66u8],data:41nat,len:1nat,next:46nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((115u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![115u8,82u8,71u8,66u8])==crate::walk_model::Name::Srgb) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![115u8,82u8,71u8,66u8])==crate::walk_model::Name::Srgb) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_ok_srgb_and_iccp_both(),46nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![105u8,67u8,67u8,80u8],data:54nat,len:287nat,next:345nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::model::frame(bytes_anc_ok_srgb_and_iccp_both(),46nat)==Ok(crate::model::WireChunk {ty:seq![105u8,67u8,67u8,80u8],data:54nat,len:287nat,next:345nat})) by (compute);
            assert(crate::model::be32(bytes_anc_ok_srgb_and_iccp_both(),341)==293582391u32) by (compute);
            crc_anc_ok_srgb_and_iccp_both_crc_46();
        }
        assert((105u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![105u8,67u8,67u8,80u8])==crate::walk_model::Name::Iccp) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![105u8,67u8,67u8,80u8])==crate::walk_model::Name::Iccp) by (compute);
        }
        assert(crate::model::first_nul(bytes_anc_ok_srgb_and_iccp_both().subrange(54,341),0)==Some(1nat)) by (compute);
        assert(bytes_anc_ok_srgb_and_iccp_both().subrange(54,341).subrange(0,1)==seq![120u8]) by (compute);
        assert(crate::model::keyword(seq![120u8]));
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_ok_srgb_and_iccp_both(),345nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:353nat,len:22nat,next:379nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_ok_srgb_and_iccp_both(),345nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:353nat,len:22nat,next:379nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_ok_srgb_and_iccp_both(),379nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:387nat,len:0nat,next:391nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_ok_srgb_and_iccp_both(),379nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:387nat,len:0nat,next:391nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::metadata_model::parse(crate::walk_model::Name::Srgb,bytes_anc_ok_srgb_and_iccp_both().subrange(41,42),2u8,8u8,0nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Srgb(1u8))) by {
            reveal(crate::metadata_model::parse);
            assert(crate::metadata_model::parse(crate::walk_model::Name::Srgb,bytes_anc_ok_srgb_and_iccp_both().subrange(41,42),2u8,8u8,0nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Srgb(1u8))) by (compute);
        }
        assert(bytes_anc_ok_srgb_and_iccp_both().subrange(54,341)[2]==0u8) by (compute);
        assert(crate::metadata_model::parse(crate::walk_model::Name::Iccp,bytes_anc_ok_srgb_and_iccp_both().subrange(54,341),2u8,8u8,0nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Iccp {name_len:1,profile_start:3,profile_len:284})) by {
            reveal(crate::metadata_model::parse);
            iccp_success(bytes_anc_ok_srgb_and_iccp_both().subrange(54,341),1nat);
        }
    assert(bytes_anc_ok_srgb_and_iccp_both().subrange(353,375)==z_anc_ok_srgb_and_iccp_both()) by (compute);
    walk_anc_ok_srgb_and_iccp_both(bytes_anc_ok_srgb_and_iccp_both());
    inflate_anc_ok_srgb_and_iccp_both();scan_anc_ok_srgb_and_iccp_both();
    assert(r::raw(a_anc_ok_srgb_and_iccp_both())==14nat) by (compute);
    assert(r::data(z_anc_ok_srgb_and_iccp_both(),a_anc_ok_srgb_and_iccp_both(),STRICT)==Some(Ok(()))) by {reveal(r::data);};
}
#[verifier::spinoff_prover]
proof fn pixel_anc_ok_srgb_and_iccp_both_0() ensures sm::image(raw_anc_ok_srgb_and_iccp_both(),a_anc_ok_srgb_and_iccp_both(),colour_anc_ok_srgb_and_iccp_both(),false)[0]==201u8,sm::image(raw_anc_ok_srgb_and_iccp_both(),a_anc_ok_srgb_and_iccp_both(),colour_anc_ok_srgb_and_iccp_both(),false)[1]==73u8,sm::image(raw_anc_ok_srgb_and_iccp_both(),a_anc_ok_srgb_and_iccp_both(),colour_anc_ok_srgb_and_iccp_both(),false)[2]==63u8,sm::image(raw_anc_ok_srgb_and_iccp_both(),a_anc_ok_srgb_and_iccp_both(),colour_anc_ok_srgb_and_iccp_both(),false)[3]==255u8,
{
    colour::depth_values(8u8);
    row_anc_ok_srgb_and_iccp_both_0_0();
    assert(crate::paint_model::source_cell(a_anc_ok_srgb_and_iccp_both(),0nat,0nat)==(0nat,0nat,0nat)) by (compute);
    assert(colour::pixel(colour_anc_ok_srgb_and_iccp_both(),seq![201u8,73u8,63u8,202u8,75u8,151u8],0nat,true,false)==Ok(seq![51657u16,18761u16,16191u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_anc_ok_srgb_and_iccp_both(),a_anc_ok_srgb_and_iccp_both(),colour_anc_ok_srgb_and_iccp_both(),0nat,0nat,true,false)==Ok(seq![51657u16,18761u16,16191u16,65535u16]));
    assert(((seq![51657u16,18761u16,16191u16,65535u16][0] as nat+128)/257) as u8==201u8) by (compute);
    assert(((seq![51657u16,18761u16,16191u16,65535u16][1] as nat+128)/257) as u8==73u8) by (compute);
    assert(((seq![51657u16,18761u16,16191u16,65535u16][2] as nat+128)/257) as u8==63u8) by (compute);
    assert(((seq![51657u16,18761u16,16191u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixel_anc_ok_srgb_and_iccp_both_1() ensures sm::image(raw_anc_ok_srgb_and_iccp_both(),a_anc_ok_srgb_and_iccp_both(),colour_anc_ok_srgb_and_iccp_both(),false)[4]==202u8,sm::image(raw_anc_ok_srgb_and_iccp_both(),a_anc_ok_srgb_and_iccp_both(),colour_anc_ok_srgb_and_iccp_both(),false)[5]==75u8,sm::image(raw_anc_ok_srgb_and_iccp_both(),a_anc_ok_srgb_and_iccp_both(),colour_anc_ok_srgb_and_iccp_both(),false)[6]==151u8,sm::image(raw_anc_ok_srgb_and_iccp_both(),a_anc_ok_srgb_and_iccp_both(),colour_anc_ok_srgb_and_iccp_both(),false)[7]==255u8,
{
    colour::depth_values(8u8);
    row_anc_ok_srgb_and_iccp_both_0_0();
    assert(crate::paint_model::source_cell(a_anc_ok_srgb_and_iccp_both(),1nat,0nat)==(0nat,1nat,0nat)) by (compute);
    assert(colour::pixel(colour_anc_ok_srgb_and_iccp_both(),seq![201u8,73u8,63u8,202u8,75u8,151u8],1nat,true,false)==Ok(seq![51914u16,19275u16,38807u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_anc_ok_srgb_and_iccp_both(),a_anc_ok_srgb_and_iccp_both(),colour_anc_ok_srgb_and_iccp_both(),1nat,0nat,true,false)==Ok(seq![51914u16,19275u16,38807u16,65535u16]));
    assert(((seq![51914u16,19275u16,38807u16,65535u16][0] as nat+128)/257) as u8==202u8) by (compute);
    assert(((seq![51914u16,19275u16,38807u16,65535u16][1] as nat+128)/257) as u8==75u8) by (compute);
    assert(((seq![51914u16,19275u16,38807u16,65535u16][2] as nat+128)/257) as u8==151u8) by (compute);
    assert(((seq![51914u16,19275u16,38807u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixel_anc_ok_srgb_and_iccp_both_2() ensures sm::image(raw_anc_ok_srgb_and_iccp_both(),a_anc_ok_srgb_and_iccp_both(),colour_anc_ok_srgb_and_iccp_both(),false)[8]==225u8,sm::image(raw_anc_ok_srgb_and_iccp_both(),a_anc_ok_srgb_and_iccp_both(),colour_anc_ok_srgb_and_iccp_both(),false)[9]==26u8,sm::image(raw_anc_ok_srgb_and_iccp_both(),a_anc_ok_srgb_and_iccp_both(),colour_anc_ok_srgb_and_iccp_both(),false)[10]==23u8,sm::image(raw_anc_ok_srgb_and_iccp_both(),a_anc_ok_srgb_and_iccp_both(),colour_anc_ok_srgb_and_iccp_both(),false)[11]==255u8,
{
    colour::depth_values(8u8);
    row_anc_ok_srgb_and_iccp_both_0_1();
    assert(crate::paint_model::source_cell(a_anc_ok_srgb_and_iccp_both(),0nat,1nat)==(0nat,0nat,1nat)) by (compute);
    assert(colour::pixel(colour_anc_ok_srgb_and_iccp_both(),seq![225u8,26u8,23u8,252u8,246u8,129u8],0nat,true,false)==Ok(seq![57825u16,6682u16,5911u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_anc_ok_srgb_and_iccp_both(),a_anc_ok_srgb_and_iccp_both(),colour_anc_ok_srgb_and_iccp_both(),0nat,1nat,true,false)==Ok(seq![57825u16,6682u16,5911u16,65535u16]));
    assert(((seq![57825u16,6682u16,5911u16,65535u16][0] as nat+128)/257) as u8==225u8) by (compute);
    assert(((seq![57825u16,6682u16,5911u16,65535u16][1] as nat+128)/257) as u8==26u8) by (compute);
    assert(((seq![57825u16,6682u16,5911u16,65535u16][2] as nat+128)/257) as u8==23u8) by (compute);
    assert(((seq![57825u16,6682u16,5911u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixel_anc_ok_srgb_and_iccp_both_3() ensures sm::image(raw_anc_ok_srgb_and_iccp_both(),a_anc_ok_srgb_and_iccp_both(),colour_anc_ok_srgb_and_iccp_both(),false)[12]==252u8,sm::image(raw_anc_ok_srgb_and_iccp_both(),a_anc_ok_srgb_and_iccp_both(),colour_anc_ok_srgb_and_iccp_both(),false)[13]==246u8,sm::image(raw_anc_ok_srgb_and_iccp_both(),a_anc_ok_srgb_and_iccp_both(),colour_anc_ok_srgb_and_iccp_both(),false)[14]==129u8,sm::image(raw_anc_ok_srgb_and_iccp_both(),a_anc_ok_srgb_and_iccp_both(),colour_anc_ok_srgb_and_iccp_both(),false)[15]==255u8,
{
    colour::depth_values(8u8);
    row_anc_ok_srgb_and_iccp_both_0_1();
    assert(crate::paint_model::source_cell(a_anc_ok_srgb_and_iccp_both(),1nat,1nat)==(0nat,1nat,1nat)) by (compute);
    assert(colour::pixel(colour_anc_ok_srgb_and_iccp_both(),seq![225u8,26u8,23u8,252u8,246u8,129u8],1nat,true,false)==Ok(seq![64764u16,63222u16,33153u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_anc_ok_srgb_and_iccp_both(),a_anc_ok_srgb_and_iccp_both(),colour_anc_ok_srgb_and_iccp_both(),1nat,1nat,true,false)==Ok(seq![64764u16,63222u16,33153u16,65535u16]));
    assert(((seq![64764u16,63222u16,33153u16,65535u16][0] as nat+128)/257) as u8==252u8) by (compute);
    assert(((seq![64764u16,63222u16,33153u16,65535u16][1] as nat+128)/257) as u8==246u8) by (compute);
    assert(((seq![64764u16,63222u16,33153u16,65535u16][2] as nat+128)/257) as u8==129u8) by (compute);
    assert(((seq![64764u16,63222u16,33153u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixels_anc_ok_srgb_and_iccp_both() ensures sm::image(raw_anc_ok_srgb_and_iccp_both(),a_anc_ok_srgb_and_iccp_both(),colour_anc_ok_srgb_and_iccp_both(),false)==expected_anc_ok_srgb_and_iccp_both(),
{
    pixel_anc_ok_srgb_and_iccp_both_0();
    pixel_anc_ok_srgb_and_iccp_both_1();
    pixel_anc_ok_srgb_and_iccp_both_2();
    pixel_anc_ok_srgb_and_iccp_both_3();
    assert(a_anc_ok_srgb_and_iccp_both().width==2nat && a_anc_ok_srgb_and_iccp_both().height==2nat) by (compute);
    assert(sm::image(raw_anc_ok_srgb_and_iccp_both(),a_anc_ok_srgb_and_iccp_both(),colour_anc_ok_srgb_and_iccp_both(),false).len()==16nat);
    assert(expected_anc_ok_srgb_and_iccp_both().len()==16nat) by (compute);
    assert forall|j: int| 0<=j<16 implies (#[trigger] sm::image(raw_anc_ok_srgb_and_iccp_both(),a_anc_ok_srgb_and_iccp_both(),colour_anc_ok_srgb_and_iccp_both(),false)[j])==expected_anc_ok_srgb_and_iccp_both()[j] by {
        if j==0 {assert(expected_anc_ok_srgb_and_iccp_both()[0]==201u8) by (compute);}
        else if j==1 {assert(expected_anc_ok_srgb_and_iccp_both()[1]==73u8) by (compute);}
        else if j==2 {assert(expected_anc_ok_srgb_and_iccp_both()[2]==63u8) by (compute);}
        else if j==3 {assert(expected_anc_ok_srgb_and_iccp_both()[3]==255u8) by (compute);}
        else if j==4 {assert(expected_anc_ok_srgb_and_iccp_both()[4]==202u8) by (compute);}
        else if j==5 {assert(expected_anc_ok_srgb_and_iccp_both()[5]==75u8) by (compute);}
        else if j==6 {assert(expected_anc_ok_srgb_and_iccp_both()[6]==151u8) by (compute);}
        else if j==7 {assert(expected_anc_ok_srgb_and_iccp_both()[7]==255u8) by (compute);}
        else if j==8 {assert(expected_anc_ok_srgb_and_iccp_both()[8]==225u8) by (compute);}
        else if j==9 {assert(expected_anc_ok_srgb_and_iccp_both()[9]==26u8) by (compute);}
        else if j==10 {assert(expected_anc_ok_srgb_and_iccp_both()[10]==23u8) by (compute);}
        else if j==11 {assert(expected_anc_ok_srgb_and_iccp_both()[11]==255u8) by (compute);}
        else if j==12 {assert(expected_anc_ok_srgb_and_iccp_both()[12]==252u8) by (compute);}
        else if j==13 {assert(expected_anc_ok_srgb_and_iccp_both()[13]==246u8) by (compute);}
        else if j==14 {assert(expected_anc_ok_srgb_and_iccp_both()[14]==129u8) by (compute);}
        else {assert(j==15);assert(expected_anc_ok_srgb_and_iccp_both()[j]==255u8) by (compute);}
    }
    assert(sm::image(raw_anc_ok_srgb_and_iccp_both(),a_anc_ok_srgb_and_iccp_both(),colour_anc_ok_srgb_and_iccp_both(),false)=~=expected_anc_ok_srgb_and_iccp_both());
}
pub open spec fn expected_value_anc_ok_srgb_and_iccp_both(value: sm::Value) -> bool {value.width==2u32 && value.height==2u32 && value.pixels==expected_anc_ok_srgb_and_iccp_both()}
#[verifier::spinoff_prover]
pub proof fn model_complete_anc_ok_srgb_and_iccp_both()
    ensures match sm::MS(bytes_anc_ok_srgb_and_iccp_both()) {sm::Outcome::Done(value)=>expected_value_anc_ok_srgb_and_iccp_both(value),_=>false},
{
    hide(m::MW_p);hide(zm::Inflate);hide(bytes_anc_ok_srgb_and_iccp_both);hide(z_anc_ok_srgb_and_iccp_both);hide(raw_anc_ok_srgb_and_iccp_both);hide(expected_anc_ok_srgb_and_iccp_both);
    assert(bytes_anc_ok_srgb_and_iccp_both().len()==391nat) by (compute);
    raster_anc_ok_srgb_and_iccp_both();inflate_anc_ok_srgb_and_iccp_both();pixels_anc_ok_srgb_and_iccp_both();
    match m::MW_p(bytes_anc_ok_srgb_and_iccp_both(),m::strict()) {m::Outcome::Done(doc,_)=>{
        assert(colour::from_doc(doc,bytes_anc_ok_srgb_and_iccp_both())==colour_anc_ok_srgb_and_iccp_both());
        assert(crate::paint_model::default_raster(doc,STRICT)==a_anc_ok_srgb_and_iccp_both());
    },_=>{}}
    assert(r::raw(a_anc_ok_srgb_and_iccp_both())==14nat) by (compute);
    assert(r::inflated(z_anc_ok_srgb_and_iccp_both(),a_anc_ok_srgb_and_iccp_both(),STRICT).out==raw_anc_ok_srgb_and_iccp_both());
    compose_complete(bytes_anc_ok_srgb_and_iccp_both(),h_anc_ok_srgb_and_iccp_both(),z_anc_ok_srgb_and_iccp_both(),0nat,colour_anc_ok_srgb_and_iccp_both(),expected_anc_ok_srgb_and_iccp_both());
}
pub fn public_complete_anc_ok_srgb_and_iccp_both() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_anc_ok_srgb_and_iccp_both(),false),result),se::eq(bytes_anc_ok_srgb_and_iccp_both(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_anc_ok_srgb_and_iccp_both(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,1u8,115u8,82u8,71u8,66u8,1u8,217u8,201u8,44u8,127u8,0u8,0u8,1u8,31u8,105u8,67u8,67u8,80u8,120u8,0u8,0u8,120u8,156u8,99u8,96u8,100u8,98u8,102u8,97u8,101u8,99u8,231u8,224u8,228u8,226u8,230u8,225u8,229u8,227u8,23u8,16u8,20u8,18u8,22u8,17u8,21u8,19u8,151u8,144u8,148u8,146u8,150u8,145u8,149u8,147u8,87u8,80u8,84u8,82u8,86u8,81u8,85u8,83u8,215u8,208u8,212u8,210u8,214u8,209u8,213u8,211u8,55u8,48u8,52u8,50u8,54u8,49u8,53u8,51u8,183u8,176u8,180u8,178u8,182u8,177u8,181u8,179u8,119u8,112u8,116u8,114u8,118u8,113u8,117u8,115u8,247u8,240u8,244u8,242u8,246u8,241u8,245u8,243u8,15u8,8u8,12u8,10u8,14u8,9u8,13u8,11u8,143u8,136u8,140u8,138u8,142u8,137u8,141u8,139u8,79u8,72u8,76u8,74u8,78u8,73u8,77u8,75u8,207u8,200u8,204u8,202u8,206u8,201u8,205u8,203u8,47u8,40u8,44u8,42u8,46u8,41u8,45u8,43u8,175u8,168u8,172u8,170u8,174u8,169u8,173u8,171u8,111u8,104u8,108u8,106u8,110u8,105u8,109u8,107u8,239u8,232u8,236u8,234u8,238u8,233u8,237u8,235u8,159u8,48u8,113u8,210u8,228u8,41u8,83u8,167u8,77u8,159u8,49u8,115u8,214u8,236u8,57u8,115u8,231u8,205u8,95u8,176u8,112u8,209u8,226u8,37u8,75u8,151u8,45u8,95u8,177u8,114u8,213u8,234u8,53u8,107u8,215u8,173u8,223u8,176u8,113u8,211u8,230u8,45u8,91u8,183u8,109u8,223u8,177u8,115u8,215u8,238u8,61u8,123u8,247u8,237u8,63u8,112u8,240u8,208u8,225u8,35u8,71u8,143u8,29u8,63u8,113u8,242u8,212u8,233u8,51u8,103u8,207u8,157u8,191u8,112u8,241u8,210u8,229u8,43u8,87u8,175u8,93u8,191u8,113u8,243u8,214u8,237u8,59u8,119u8,239u8,221u8,127u8,240u8,240u8,209u8,227u8,39u8,79u8,159u8,61u8,127u8,241u8,242u8,213u8,235u8,55u8,111u8,223u8,189u8,255u8,240u8,241u8,211u8,231u8,47u8,95u8,191u8,125u8,255u8,241u8,243u8,215u8,239u8,63u8,127u8,255u8,253u8,103u8,24u8,245u8,255u8,136u8,246u8,63u8,0u8,160u8,98u8,126u8,144u8,17u8,127u8,182u8,55u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_anc_ok_srgb_and_iccp_both());model_complete_anc_ok_srgb_and_iccp_both();}
    crate::png::try_decode_still(&b)
}
// anc-ok/many-text; frozen fixture SHA256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
pub open spec fn bytes_anc_ok_many_text() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,4u8,116u8,69u8,88u8,116u8,107u8,48u8,0u8,118u8,198u8,174u8,151u8,211u8,0u8,0u8,0u8,4u8,116u8,69u8,88u8,116u8,107u8,49u8,0u8,118u8,199u8,108u8,253u8,228u8,0u8,0u8,0u8,4u8,116u8,69u8,88u8,116u8,107u8,50u8,0u8,118u8,197u8,42u8,67u8,189u8,0u8,0u8,0u8,4u8,116u8,69u8,88u8,116u8,107u8,51u8,0u8,118u8,196u8,232u8,41u8,138u8,0u8,0u8,0u8,4u8,116u8,69u8,88u8,116u8,107u8,52u8,0u8,118u8,193u8,167u8,63u8,15u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn h_anc_ok_many_text() -> crate::model::Ihdr {crate::model::Ihdr {width:2u32,height:2u32,depth:8u8,color_type:2u8,interlace:0u8}}
spec fn a_anc_ok_many_text() -> r::Raster {r::shape(h_anc_ok_many_text(),2nat,2nat,0nat,STRICT)}
spec fn colour_anc_ok_many_text() -> crate::colour_model::Context {crate::colour_model::Context {ct:2u8,depth:8u8,plte:seq![],trns:None}}
pub open spec fn expected_anc_ok_many_text() -> Seq<u8> {seq![201u8,73u8,63u8,255u8,202u8,75u8,151u8,255u8,225u8,26u8,23u8,255u8,252u8,246u8,129u8,255u8]}
spec fn z_anc_ok_many_text() -> Seq<u8> {seq![120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8]}
spec fn raw_anc_ok_many_text() -> Seq<u8> {seq![0u8,201u8,73u8,63u8,202u8,75u8,151u8,1u8,225u8,26u8,23u8,27u8,220u8,106u8]}
spec fn ts_anc_ok_many_text() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:36nat,before:1nat,after:2nat,symbol:201nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:36nat,next:44nat,before:2nat,after:3nat,symbol:73nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:44nat,next:52nat,before:3nat,after:4nat,symbol:63nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:52nat,next:61nat,before:4nat,after:5nat,symbol:202nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:61nat,next:69nat,before:5nat,after:6nat,symbol:75nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:69nat,next:78nat,before:6nat,after:7nat,symbol:151nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:78nat,next:86nat,before:7nat,after:8nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:86nat,next:95nat,before:8nat,after:9nat,symbol:225nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:95nat,next:103nat,before:9nat,after:10nat,symbol:26nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:103nat,next:111nat,before:10nat,after:11nat,symbol:23nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:111nat,next:119nat,before:11nat,after:12nat,symbol:27nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:119nat,next:128nat,before:12nat,after:13nat,symbol:220nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:128nat,next:136nat,before:13nat,after:14nat,symbol:106nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:136nat,next:143nat,before:14nat,after:14nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_anc_ok_many_text_0() ensures t::trace(z_anc_ok_many_text(),raw_anc_ok_many_text(),ts_anc_ok_many_text(),0nat,15nat),
{
    hide(t::valid_step);assert(t::trace(z_anc_ok_many_text(),raw_anc_ok_many_text(),ts_anc_ok_many_text(),0nat,15nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn trace_anc_ok_many_text() ensures t::trace(z_anc_ok_many_text(),raw_anc_ok_many_text(),ts_anc_ok_many_text(),0,15nat),
{
    hide(z_anc_ok_many_text);hide(raw_anc_ok_many_text);hide(ts_anc_ok_many_text);hide(t::trace);
    trace_anc_ok_many_text_0();
    assert(t::trace(z_anc_ok_many_text(),raw_anc_ok_many_text(),ts_anc_ok_many_text(),0,15nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_anc_ok_many_text() ensures z_anc_ok_many_text().len()==22nat,
        raw_anc_ok_many_text().len()==14nat,
        z_anc_ok_many_text()[0]==120u8,
        z_anc_ok_many_text()[1]==156u8,
        zm::field_value(z_anc_ok_many_text(),16,1)==1nat,
        zm::field_value(z_anc_ok_many_text(),17,2)==1nat,
        zm::be_value(z_anc_ok_many_text(),18nat,4)==675349874nat,
        ts_anc_ok_many_text().len()==15nat,
        ts_anc_ok_many_text()[0].pos==19nat,
        ts_anc_ok_many_text()[0].before==0nat,
        ts_anc_ok_many_text().last().next==143nat,
{
    assert(z_anc_ok_many_text().len()==22nat) by (compute);
    assert(raw_anc_ok_many_text().len()==14nat) by (compute);
    assert(z_anc_ok_many_text()[0]==120u8) by (compute);
    assert(z_anc_ok_many_text()[1]==156u8) by (compute);
    assert(zm::field_value(z_anc_ok_many_text(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_anc_ok_many_text(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_anc_ok_many_text(),18nat,4)==675349874nat) by (compute);
    assert(ts_anc_ok_many_text().len()==15nat) by (compute);
    assert(ts_anc_ok_many_text()[0].pos==19nat) by (compute);
    assert(ts_anc_ok_many_text()[0].before==0nat) by (compute);
    assert(ts_anc_ok_many_text().last().next==143nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_anc_ok_many_text() ensures zm::Adler(raw_anc_ok_many_text())==675349874nat,
{
    hide(raw_anc_ok_many_text);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_anc_ok_many_text().len()==14nat) by (compute);
    assert(c::sums(raw_anc_ok_many_text(),0,14nat)==(1393nat,10305nat)) by (compute);
    c::sums_correct(raw_anc_ok_many_text(),0,14nat);assert(raw_anc_ok_many_text().subrange(0,14)=~=raw_anc_ok_many_text());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((10305nat%65521)*(u16::MAX as nat+1)+(1+1393nat)%65521==675349874nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_anc_ok_many_text() ensures zm::Inflate(z_anc_ok_many_text(),14nat,true,false)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_anc_ok_many_text(),r2:false}),raw_anc_ok_many_text().len()==14nat,
{
    hide(z_anc_ok_many_text);hide(raw_anc_ok_many_text);hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);hide(ts_anc_ok_many_text);
    shape_anc_ok_many_text();checksum_anc_ok_many_text();
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_anc_ok_many_text();ms::fixed_stream(z_anc_ok_many_text(),raw_anc_ok_many_text(),ts_anc_ok_many_text(),14nat,156nat,675349874nat,675349874nat);
}
#[verifier::spinoff_prover]
proof fn walk_anc_ok_many_text(b: Seq<u8>)
    requires b.len()==159,
        crate::model::mh(b)==Ok((h_anc_ok_many_text(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,69u8,88u8,116u8],data:41nat,len:4nat,next:49nat,crc_ok:true,synthetic:false})),
        crate::model::first_nul(b.subrange(41,45),0)==Some(2nat),
        b.subrange(41,45).subrange(0,2)==seq![107u8,48u8],
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,49nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,69u8,88u8,116u8],data:57nat,len:4nat,next:65nat,crc_ok:true,synthetic:false})),
        crate::model::first_nul(b.subrange(57,61),0)==Some(2nat),
        b.subrange(57,61).subrange(0,2)==seq![107u8,49u8],
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,65nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,69u8,88u8,116u8],data:73nat,len:4nat,next:81nat,crc_ok:true,synthetic:false})),
        crate::model::first_nul(b.subrange(73,77),0)==Some(2nat),
        b.subrange(73,77).subrange(0,2)==seq![107u8,50u8],
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,81nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,69u8,88u8,116u8],data:89nat,len:4nat,next:97nat,crc_ok:true,synthetic:false})),
        crate::model::first_nul(b.subrange(89,93),0)==Some(2nat),
        b.subrange(89,93).subrange(0,2)==seq![107u8,51u8],
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,97nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,69u8,88u8,116u8],data:105nat,len:4nat,next:113nat,crc_ok:true,synthetic:false})),
        crate::model::first_nul(b.subrange(105,109),0)==Some(2nat),
        b.subrange(105,109).subrange(0,2)==seq![107u8,52u8],
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,113nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:121nat,len:22nat,next:147nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,147nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:155nat,len:0nat,next:159nat,crc_ok:true,synthetic:false})),
        crate::metadata_model::parse(crate::walk_model::Name::Text,b.subrange(41,45),2u8,8u8,0nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored),
        crate::metadata_model::parse(crate::walk_model::Name::Text,b.subrange(57,61),2u8,8u8,0nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored),
        crate::metadata_model::parse(crate::walk_model::Name::Text,b.subrange(73,77),2u8,8u8,0nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored),
        crate::metadata_model::parse(crate::walk_model::Name::Text,b.subrange(89,93),2u8,8u8,0nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored),
        crate::metadata_model::parse(crate::walk_model::Name::Text,b.subrange(105,109),2u8,8u8,0nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored),
        b.subrange(121,143)==z_anc_ok_many_text(),
    ensures r::default_case(b,h_anc_ok_many_text(),z_anc_ok_many_text(),0nat),match m::MW_p(b,m::strict()) {m::Outcome::Done(doc,_)=>doc.plte==None && doc.trns==None,_=>false},
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(m::store);hide(crate::metadata_model::parse);
    assert forall|doc: m::Doc,value: crate::metadata_model::Value,base: nat|
        (#[trigger] m::store(doc,value,base)).ihdr==doc.ihdr && m::store(doc,value,base).plte==doc.plte
        && m::store(doc,value,base).trns==doc.trns && m::store(doc,value,base).actl==doc.actl
        && m::store(doc,value,base).idat==doc.idat && m::store(doc,value,base).frames==doc.frames by {
        stored_structure(doc,value,base);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,69u8,88u8,116u8])==crate::walk_model::Name::Text) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,69u8,88u8,116u8])==crate::walk_model::Name::Text) by (compute);
        }
        assert(crate::model::keyword(seq![107u8,48u8]));
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,69u8,88u8,116u8])==crate::walk_model::Name::Text) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,69u8,88u8,116u8])==crate::walk_model::Name::Text) by (compute);
        }
        assert(crate::model::keyword(seq![107u8,49u8]));
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,69u8,88u8,116u8])==crate::walk_model::Name::Text) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,69u8,88u8,116u8])==crate::walk_model::Name::Text) by (compute);
        }
        assert(crate::model::keyword(seq![107u8,50u8]));
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,69u8,88u8,116u8])==crate::walk_model::Name::Text) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,69u8,88u8,116u8])==crate::walk_model::Name::Text) by (compute);
        }
        assert(crate::model::keyword(seq![107u8,51u8]));
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,69u8,88u8,116u8])==crate::walk_model::Name::Text) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,69u8,88u8,116u8])==crate::walk_model::Name::Text) by (compute);
        }
        assert(crate::model::keyword(seq![107u8,52u8]));
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    reveal_with_fuel(m::drive,8);
    reveal_with_fuel(m::concat_ranges,3);
}
#[verifier::spinoff_prover]
proof fn scan_anc_ok_many_text() ensures r::scan(raw_anc_ok_many_text(),a_anc_ok_many_text(),0)==Ok(()),
{
    hide(raw_anc_ok_many_text);assert(raw_anc_ok_many_text().len()==14nat) by (compute);assert(r::count(a_anc_ok_many_text())==1nat) by (compute);
    assert(r::pw(a_anc_ok_many_text(),0nat)==2nat && r::ph(a_anc_ok_many_text(),0nat)==2nat && r::rb(a_anc_ok_many_text(),0nat)==6nat && r::base(a_anc_ok_many_text(),0nat)==0nat) by (compute);
    assert(raw_anc_ok_many_text()[0]==0u8) by (compute);
    assert(r::row_result(raw_anc_ok_many_text(),a_anc_ok_many_text(),0nat,0nat)==Ok(()));
    assert(raw_anc_ok_many_text()[7]==1u8) by (compute);
    assert(r::row_result(raw_anc_ok_many_text(),a_anc_ok_many_text(),0nat,1nat)==Ok(()));
    r::scan_step(raw_anc_ok_many_text(),a_anc_ok_many_text(),1nat);
    r::pass_step(raw_anc_ok_many_text(),a_anc_ok_many_text(),0nat,2nat);
    r::pass_step(raw_anc_ok_many_text(),a_anc_ok_many_text(),0nat,1nat);
    r::pass_step(raw_anc_ok_many_text(),a_anc_ok_many_text(),0nat,0nat);
    r::scan_step(raw_anc_ok_many_text(),a_anc_ok_many_text(),0nat);
}
#[verifier::spinoff_prover]
proof fn row_anc_ok_many_text_0_0() ensures r::line(raw_anc_ok_many_text(),a_anc_ok_many_text(),0nat,0nat)==seq![201u8,73u8,63u8,202u8,75u8,151u8],
{
    hide(raw_anc_ok_many_text);
    assert(raw_anc_ok_many_text().len()==14nat) by (compute);
    assert(raw_anc_ok_many_text()[0]==0u8) by (compute);
    assert(r::base(a_anc_ok_many_text(),0nat)==0nat && r::rb(a_anc_ok_many_text(),0nat)==6nat && f::bpp(r::bits(a_anc_ok_many_text()))==3nat) by (compute);
    assert(f::row_data(raw_anc_ok_many_text(),0nat,6nat,0nat)==seq![201u8,73u8,63u8,202u8,75u8,151u8]) by (compute);
    f::pass_row_equation(raw_anc_ok_many_text(),0nat,6nat,3nat,0nat);
    assert(f::previous_row(raw_anc_ok_many_text(),0nat,6nat,3nat,0nat)==None);
    assert(f::encoded_row(0u8,f::row_data(raw_anc_ok_many_text(),0nat,6nat,0nat),None,3nat,seq![201u8,73u8,63u8,202u8,75u8,151u8],0,6nat)) by (compute);
    f::checked_row_hint(0u8,f::row_data(raw_anc_ok_many_text(),0nat,6nat,0nat),None,3nat,seq![201u8,73u8,63u8,202u8,75u8,151u8]);
}
#[verifier::spinoff_prover]
proof fn row_anc_ok_many_text_0_1() ensures r::line(raw_anc_ok_many_text(),a_anc_ok_many_text(),0nat,1nat)==seq![225u8,26u8,23u8,252u8,246u8,129u8],
{
    hide(raw_anc_ok_many_text);
    assert(raw_anc_ok_many_text().len()==14nat) by (compute);
    assert(raw_anc_ok_many_text()[7]==1u8) by (compute);
    assert(r::base(a_anc_ok_many_text(),0nat)==0nat && r::rb(a_anc_ok_many_text(),0nat)==6nat && f::bpp(r::bits(a_anc_ok_many_text()))==3nat) by (compute);
    assert(f::row_data(raw_anc_ok_many_text(),0nat,6nat,1nat)==seq![225u8,26u8,23u8,27u8,220u8,106u8]) by (compute);
    row_anc_ok_many_text_0_0();
    f::pass_row_equation(raw_anc_ok_many_text(),0nat,6nat,3nat,1nat);
    assert(f::previous_row(raw_anc_ok_many_text(),0nat,6nat,3nat,1nat)==Some(seq![201u8,73u8,63u8,202u8,75u8,151u8]));
    assert(f::encoded_row(1u8,f::row_data(raw_anc_ok_many_text(),0nat,6nat,1nat),Some(seq![201u8,73u8,63u8,202u8,75u8,151u8]),3nat,seq![225u8,26u8,23u8,252u8,246u8,129u8],0,6nat)) by (compute);
    f::checked_row_hint(1u8,f::row_data(raw_anc_ok_many_text(),0nat,6nat,1nat),Some(seq![201u8,73u8,63u8,202u8,75u8,151u8]),3nat,seq![225u8,26u8,23u8,252u8,246u8,129u8]);
}
#[verifier::spinoff_prover]
proof fn raster_anc_ok_many_text()
    ensures r::default_case(bytes_anc_ok_many_text(),h_anc_ok_many_text(),z_anc_ok_many_text(),0nat),match m::MW_p(bytes_anc_ok_many_text(),m::strict()) {m::Outcome::Done(doc,_)=>doc.plte==None && doc.trns==None,_=>false},r::data(z_anc_ok_many_text(),a_anc_ok_many_text(),STRICT)==Some(Ok(())),
{
    hide(bytes_anc_ok_many_text);hide(z_anc_ok_many_text);hide(raw_anc_ok_many_text);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    assert(bytes_anc_ok_many_text().len()==159nat) by (compute);
    assert(crate::model::mh(bytes_anc_ok_many_text())==Ok((h_anc_ok_many_text(),33nat))) by {reveal(crate::model::mh);assert(crate::model::mh(bytes_anc_ok_many_text())==Ok((h_anc_ok_many_text(),33nat))) by (compute);}
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_ok_many_text(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,69u8,88u8,116u8],data:41nat,len:4nat,next:49nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_ok_many_text(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,69u8,88u8,116u8],data:41nat,len:4nat,next:49nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,69u8,88u8,116u8])==crate::walk_model::Name::Text) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,69u8,88u8,116u8])==crate::walk_model::Name::Text) by (compute);
        }
        assert(crate::model::first_nul(bytes_anc_ok_many_text().subrange(41,45),0)==Some(2nat)) by (compute);
        assert(bytes_anc_ok_many_text().subrange(41,45).subrange(0,2)==seq![107u8,48u8]) by (compute);
        assert(crate::model::keyword(seq![107u8,48u8]));
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_ok_many_text(),49nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,69u8,88u8,116u8],data:57nat,len:4nat,next:65nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_ok_many_text(),49nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,69u8,88u8,116u8],data:57nat,len:4nat,next:65nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,69u8,88u8,116u8])==crate::walk_model::Name::Text) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,69u8,88u8,116u8])==crate::walk_model::Name::Text) by (compute);
        }
        assert(crate::model::first_nul(bytes_anc_ok_many_text().subrange(57,61),0)==Some(2nat)) by (compute);
        assert(bytes_anc_ok_many_text().subrange(57,61).subrange(0,2)==seq![107u8,49u8]) by (compute);
        assert(crate::model::keyword(seq![107u8,49u8]));
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_ok_many_text(),65nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,69u8,88u8,116u8],data:73nat,len:4nat,next:81nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_ok_many_text(),65nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,69u8,88u8,116u8],data:73nat,len:4nat,next:81nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,69u8,88u8,116u8])==crate::walk_model::Name::Text) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,69u8,88u8,116u8])==crate::walk_model::Name::Text) by (compute);
        }
        assert(crate::model::first_nul(bytes_anc_ok_many_text().subrange(73,77),0)==Some(2nat)) by (compute);
        assert(bytes_anc_ok_many_text().subrange(73,77).subrange(0,2)==seq![107u8,50u8]) by (compute);
        assert(crate::model::keyword(seq![107u8,50u8]));
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_ok_many_text(),81nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,69u8,88u8,116u8],data:89nat,len:4nat,next:97nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_ok_many_text(),81nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,69u8,88u8,116u8],data:89nat,len:4nat,next:97nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,69u8,88u8,116u8])==crate::walk_model::Name::Text) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,69u8,88u8,116u8])==crate::walk_model::Name::Text) by (compute);
        }
        assert(crate::model::first_nul(bytes_anc_ok_many_text().subrange(89,93),0)==Some(2nat)) by (compute);
        assert(bytes_anc_ok_many_text().subrange(89,93).subrange(0,2)==seq![107u8,51u8]) by (compute);
        assert(crate::model::keyword(seq![107u8,51u8]));
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_ok_many_text(),97nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,69u8,88u8,116u8],data:105nat,len:4nat,next:113nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_ok_many_text(),97nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,69u8,88u8,116u8],data:105nat,len:4nat,next:113nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,69u8,88u8,116u8])==crate::walk_model::Name::Text) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,69u8,88u8,116u8])==crate::walk_model::Name::Text) by (compute);
        }
        assert(crate::model::first_nul(bytes_anc_ok_many_text().subrange(105,109),0)==Some(2nat)) by (compute);
        assert(bytes_anc_ok_many_text().subrange(105,109).subrange(0,2)==seq![107u8,52u8]) by (compute);
        assert(crate::model::keyword(seq![107u8,52u8]));
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_ok_many_text(),113nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:121nat,len:22nat,next:147nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_ok_many_text(),113nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:121nat,len:22nat,next:147nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_ok_many_text(),147nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:155nat,len:0nat,next:159nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_ok_many_text(),147nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:155nat,len:0nat,next:159nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::metadata_model::parse(crate::walk_model::Name::Text,bytes_anc_ok_many_text().subrange(41,45),2u8,8u8,0nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored)) by {
            reveal(crate::metadata_model::parse);
            text_success(bytes_anc_ok_many_text().subrange(41,45),2nat);
        }
        assert(crate::metadata_model::parse(crate::walk_model::Name::Text,bytes_anc_ok_many_text().subrange(57,61),2u8,8u8,0nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored)) by {
            reveal(crate::metadata_model::parse);
            text_success(bytes_anc_ok_many_text().subrange(57,61),2nat);
        }
        assert(crate::metadata_model::parse(crate::walk_model::Name::Text,bytes_anc_ok_many_text().subrange(73,77),2u8,8u8,0nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored)) by {
            reveal(crate::metadata_model::parse);
            text_success(bytes_anc_ok_many_text().subrange(73,77),2nat);
        }
        assert(crate::metadata_model::parse(crate::walk_model::Name::Text,bytes_anc_ok_many_text().subrange(89,93),2u8,8u8,0nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored)) by {
            reveal(crate::metadata_model::parse);
            text_success(bytes_anc_ok_many_text().subrange(89,93),2nat);
        }
        assert(crate::metadata_model::parse(crate::walk_model::Name::Text,bytes_anc_ok_many_text().subrange(105,109),2u8,8u8,0nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored)) by {
            reveal(crate::metadata_model::parse);
            text_success(bytes_anc_ok_many_text().subrange(105,109),2nat);
        }
    assert(bytes_anc_ok_many_text().subrange(121,143)==z_anc_ok_many_text()) by (compute);
    walk_anc_ok_many_text(bytes_anc_ok_many_text());
    inflate_anc_ok_many_text();scan_anc_ok_many_text();
    assert(r::raw(a_anc_ok_many_text())==14nat) by (compute);
    assert(r::data(z_anc_ok_many_text(),a_anc_ok_many_text(),STRICT)==Some(Ok(()))) by {reveal(r::data);};
}
#[verifier::spinoff_prover]
proof fn pixel_anc_ok_many_text_0() ensures sm::image(raw_anc_ok_many_text(),a_anc_ok_many_text(),colour_anc_ok_many_text(),false)[0]==201u8,sm::image(raw_anc_ok_many_text(),a_anc_ok_many_text(),colour_anc_ok_many_text(),false)[1]==73u8,sm::image(raw_anc_ok_many_text(),a_anc_ok_many_text(),colour_anc_ok_many_text(),false)[2]==63u8,sm::image(raw_anc_ok_many_text(),a_anc_ok_many_text(),colour_anc_ok_many_text(),false)[3]==255u8,
{
    colour::depth_values(8u8);
    row_anc_ok_many_text_0_0();
    assert(crate::paint_model::source_cell(a_anc_ok_many_text(),0nat,0nat)==(0nat,0nat,0nat)) by (compute);
    assert(colour::pixel(colour_anc_ok_many_text(),seq![201u8,73u8,63u8,202u8,75u8,151u8],0nat,true,false)==Ok(seq![51657u16,18761u16,16191u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_anc_ok_many_text(),a_anc_ok_many_text(),colour_anc_ok_many_text(),0nat,0nat,true,false)==Ok(seq![51657u16,18761u16,16191u16,65535u16]));
    assert(((seq![51657u16,18761u16,16191u16,65535u16][0] as nat+128)/257) as u8==201u8) by (compute);
    assert(((seq![51657u16,18761u16,16191u16,65535u16][1] as nat+128)/257) as u8==73u8) by (compute);
    assert(((seq![51657u16,18761u16,16191u16,65535u16][2] as nat+128)/257) as u8==63u8) by (compute);
    assert(((seq![51657u16,18761u16,16191u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixel_anc_ok_many_text_1() ensures sm::image(raw_anc_ok_many_text(),a_anc_ok_many_text(),colour_anc_ok_many_text(),false)[4]==202u8,sm::image(raw_anc_ok_many_text(),a_anc_ok_many_text(),colour_anc_ok_many_text(),false)[5]==75u8,sm::image(raw_anc_ok_many_text(),a_anc_ok_many_text(),colour_anc_ok_many_text(),false)[6]==151u8,sm::image(raw_anc_ok_many_text(),a_anc_ok_many_text(),colour_anc_ok_many_text(),false)[7]==255u8,
{
    colour::depth_values(8u8);
    row_anc_ok_many_text_0_0();
    assert(crate::paint_model::source_cell(a_anc_ok_many_text(),1nat,0nat)==(0nat,1nat,0nat)) by (compute);
    assert(colour::pixel(colour_anc_ok_many_text(),seq![201u8,73u8,63u8,202u8,75u8,151u8],1nat,true,false)==Ok(seq![51914u16,19275u16,38807u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_anc_ok_many_text(),a_anc_ok_many_text(),colour_anc_ok_many_text(),1nat,0nat,true,false)==Ok(seq![51914u16,19275u16,38807u16,65535u16]));
    assert(((seq![51914u16,19275u16,38807u16,65535u16][0] as nat+128)/257) as u8==202u8) by (compute);
    assert(((seq![51914u16,19275u16,38807u16,65535u16][1] as nat+128)/257) as u8==75u8) by (compute);
    assert(((seq![51914u16,19275u16,38807u16,65535u16][2] as nat+128)/257) as u8==151u8) by (compute);
    assert(((seq![51914u16,19275u16,38807u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixel_anc_ok_many_text_2() ensures sm::image(raw_anc_ok_many_text(),a_anc_ok_many_text(),colour_anc_ok_many_text(),false)[8]==225u8,sm::image(raw_anc_ok_many_text(),a_anc_ok_many_text(),colour_anc_ok_many_text(),false)[9]==26u8,sm::image(raw_anc_ok_many_text(),a_anc_ok_many_text(),colour_anc_ok_many_text(),false)[10]==23u8,sm::image(raw_anc_ok_many_text(),a_anc_ok_many_text(),colour_anc_ok_many_text(),false)[11]==255u8,
{
    colour::depth_values(8u8);
    row_anc_ok_many_text_0_1();
    assert(crate::paint_model::source_cell(a_anc_ok_many_text(),0nat,1nat)==(0nat,0nat,1nat)) by (compute);
    assert(colour::pixel(colour_anc_ok_many_text(),seq![225u8,26u8,23u8,252u8,246u8,129u8],0nat,true,false)==Ok(seq![57825u16,6682u16,5911u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_anc_ok_many_text(),a_anc_ok_many_text(),colour_anc_ok_many_text(),0nat,1nat,true,false)==Ok(seq![57825u16,6682u16,5911u16,65535u16]));
    assert(((seq![57825u16,6682u16,5911u16,65535u16][0] as nat+128)/257) as u8==225u8) by (compute);
    assert(((seq![57825u16,6682u16,5911u16,65535u16][1] as nat+128)/257) as u8==26u8) by (compute);
    assert(((seq![57825u16,6682u16,5911u16,65535u16][2] as nat+128)/257) as u8==23u8) by (compute);
    assert(((seq![57825u16,6682u16,5911u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixel_anc_ok_many_text_3() ensures sm::image(raw_anc_ok_many_text(),a_anc_ok_many_text(),colour_anc_ok_many_text(),false)[12]==252u8,sm::image(raw_anc_ok_many_text(),a_anc_ok_many_text(),colour_anc_ok_many_text(),false)[13]==246u8,sm::image(raw_anc_ok_many_text(),a_anc_ok_many_text(),colour_anc_ok_many_text(),false)[14]==129u8,sm::image(raw_anc_ok_many_text(),a_anc_ok_many_text(),colour_anc_ok_many_text(),false)[15]==255u8,
{
    colour::depth_values(8u8);
    row_anc_ok_many_text_0_1();
    assert(crate::paint_model::source_cell(a_anc_ok_many_text(),1nat,1nat)==(0nat,1nat,1nat)) by (compute);
    assert(colour::pixel(colour_anc_ok_many_text(),seq![225u8,26u8,23u8,252u8,246u8,129u8],1nat,true,false)==Ok(seq![64764u16,63222u16,33153u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_anc_ok_many_text(),a_anc_ok_many_text(),colour_anc_ok_many_text(),1nat,1nat,true,false)==Ok(seq![64764u16,63222u16,33153u16,65535u16]));
    assert(((seq![64764u16,63222u16,33153u16,65535u16][0] as nat+128)/257) as u8==252u8) by (compute);
    assert(((seq![64764u16,63222u16,33153u16,65535u16][1] as nat+128)/257) as u8==246u8) by (compute);
    assert(((seq![64764u16,63222u16,33153u16,65535u16][2] as nat+128)/257) as u8==129u8) by (compute);
    assert(((seq![64764u16,63222u16,33153u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixels_anc_ok_many_text() ensures sm::image(raw_anc_ok_many_text(),a_anc_ok_many_text(),colour_anc_ok_many_text(),false)==expected_anc_ok_many_text(),
{
    pixel_anc_ok_many_text_0();
    pixel_anc_ok_many_text_1();
    pixel_anc_ok_many_text_2();
    pixel_anc_ok_many_text_3();
    assert(a_anc_ok_many_text().width==2nat && a_anc_ok_many_text().height==2nat) by (compute);
    assert(sm::image(raw_anc_ok_many_text(),a_anc_ok_many_text(),colour_anc_ok_many_text(),false).len()==16nat);
    assert(expected_anc_ok_many_text().len()==16nat) by (compute);
    assert forall|j: int| 0<=j<16 implies (#[trigger] sm::image(raw_anc_ok_many_text(),a_anc_ok_many_text(),colour_anc_ok_many_text(),false)[j])==expected_anc_ok_many_text()[j] by {
        if j==0 {assert(expected_anc_ok_many_text()[0]==201u8) by (compute);}
        else if j==1 {assert(expected_anc_ok_many_text()[1]==73u8) by (compute);}
        else if j==2 {assert(expected_anc_ok_many_text()[2]==63u8) by (compute);}
        else if j==3 {assert(expected_anc_ok_many_text()[3]==255u8) by (compute);}
        else if j==4 {assert(expected_anc_ok_many_text()[4]==202u8) by (compute);}
        else if j==5 {assert(expected_anc_ok_many_text()[5]==75u8) by (compute);}
        else if j==6 {assert(expected_anc_ok_many_text()[6]==151u8) by (compute);}
        else if j==7 {assert(expected_anc_ok_many_text()[7]==255u8) by (compute);}
        else if j==8 {assert(expected_anc_ok_many_text()[8]==225u8) by (compute);}
        else if j==9 {assert(expected_anc_ok_many_text()[9]==26u8) by (compute);}
        else if j==10 {assert(expected_anc_ok_many_text()[10]==23u8) by (compute);}
        else if j==11 {assert(expected_anc_ok_many_text()[11]==255u8) by (compute);}
        else if j==12 {assert(expected_anc_ok_many_text()[12]==252u8) by (compute);}
        else if j==13 {assert(expected_anc_ok_many_text()[13]==246u8) by (compute);}
        else if j==14 {assert(expected_anc_ok_many_text()[14]==129u8) by (compute);}
        else {assert(j==15);assert(expected_anc_ok_many_text()[j]==255u8) by (compute);}
    }
    assert(sm::image(raw_anc_ok_many_text(),a_anc_ok_many_text(),colour_anc_ok_many_text(),false)=~=expected_anc_ok_many_text());
}
pub open spec fn expected_value_anc_ok_many_text(value: sm::Value) -> bool {value.width==2u32 && value.height==2u32 && value.pixels==expected_anc_ok_many_text()}
#[verifier::spinoff_prover]
pub proof fn model_complete_anc_ok_many_text()
    ensures match sm::MS(bytes_anc_ok_many_text()) {sm::Outcome::Done(value)=>expected_value_anc_ok_many_text(value),_=>false},
{
    hide(m::MW_p);hide(zm::Inflate);hide(bytes_anc_ok_many_text);hide(z_anc_ok_many_text);hide(raw_anc_ok_many_text);hide(expected_anc_ok_many_text);
    assert(bytes_anc_ok_many_text().len()==159nat) by (compute);
    raster_anc_ok_many_text();inflate_anc_ok_many_text();pixels_anc_ok_many_text();
    match m::MW_p(bytes_anc_ok_many_text(),m::strict()) {m::Outcome::Done(doc,_)=>{
        assert(colour::from_doc(doc,bytes_anc_ok_many_text())==colour_anc_ok_many_text());
        assert(crate::paint_model::default_raster(doc,STRICT)==a_anc_ok_many_text());
    },_=>{}}
    assert(r::raw(a_anc_ok_many_text())==14nat) by (compute);
    assert(r::inflated(z_anc_ok_many_text(),a_anc_ok_many_text(),STRICT).out==raw_anc_ok_many_text());
    compose_complete(bytes_anc_ok_many_text(),h_anc_ok_many_text(),z_anc_ok_many_text(),0nat,colour_anc_ok_many_text(),expected_anc_ok_many_text());
}
pub fn public_complete_anc_ok_many_text() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_anc_ok_many_text(),false),result),se::eq(bytes_anc_ok_many_text(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_anc_ok_many_text(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,4u8,116u8,69u8,88u8,116u8,107u8,48u8,0u8,118u8,198u8,174u8,151u8,211u8,0u8,0u8,0u8,4u8,116u8,69u8,88u8,116u8,107u8,49u8,0u8,118u8,199u8,108u8,253u8,228u8,0u8,0u8,0u8,4u8,116u8,69u8,88u8,116u8,107u8,50u8,0u8,118u8,197u8,42u8,67u8,189u8,0u8,0u8,0u8,4u8,116u8,69u8,88u8,116u8,107u8,51u8,0u8,118u8,196u8,232u8,41u8,138u8,0u8,0u8,0u8,4u8,116u8,69u8,88u8,116u8,107u8,52u8,0u8,118u8,193u8,167u8,63u8,15u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_anc_ok_many_text());model_complete_anc_ok_many_text();}
    crate::png::try_decode_still(&b)
}
// anc-ok/ztxt-with-corrupt-zlib-not-inflated; frozen fixture SHA256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
pub open spec fn bytes_anc_ok_ztxt_with_corrupt_zlib_not_inflated() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,10u8,122u8,84u8,88u8,116u8,107u8,0u8,0u8,103u8,97u8,114u8,98u8,97u8,103u8,101u8,188u8,174u8,84u8,233u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn h_anc_ok_ztxt_with_corrupt_zlib_not_inflated() -> crate::model::Ihdr {crate::model::Ihdr {width:2u32,height:2u32,depth:8u8,color_type:2u8,interlace:0u8}}
spec fn a_anc_ok_ztxt_with_corrupt_zlib_not_inflated() -> r::Raster {r::shape(h_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),2nat,2nat,0nat,STRICT)}
spec fn colour_anc_ok_ztxt_with_corrupt_zlib_not_inflated() -> crate::colour_model::Context {crate::colour_model::Context {ct:2u8,depth:8u8,plte:seq![],trns:None}}
pub open spec fn expected_anc_ok_ztxt_with_corrupt_zlib_not_inflated() -> Seq<u8> {seq![201u8,73u8,63u8,255u8,202u8,75u8,151u8,255u8,225u8,26u8,23u8,255u8,252u8,246u8,129u8,255u8]}
spec fn z_anc_ok_ztxt_with_corrupt_zlib_not_inflated() -> Seq<u8> {seq![120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8]}
spec fn raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated() -> Seq<u8> {seq![0u8,201u8,73u8,63u8,202u8,75u8,151u8,1u8,225u8,26u8,23u8,27u8,220u8,106u8]}
spec fn ts_anc_ok_ztxt_with_corrupt_zlib_not_inflated() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:36nat,before:1nat,after:2nat,symbol:201nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:36nat,next:44nat,before:2nat,after:3nat,symbol:73nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:44nat,next:52nat,before:3nat,after:4nat,symbol:63nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:52nat,next:61nat,before:4nat,after:5nat,symbol:202nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:61nat,next:69nat,before:5nat,after:6nat,symbol:75nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:69nat,next:78nat,before:6nat,after:7nat,symbol:151nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:78nat,next:86nat,before:7nat,after:8nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:86nat,next:95nat,before:8nat,after:9nat,symbol:225nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:95nat,next:103nat,before:9nat,after:10nat,symbol:26nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:103nat,next:111nat,before:10nat,after:11nat,symbol:23nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:111nat,next:119nat,before:11nat,after:12nat,symbol:27nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:119nat,next:128nat,before:12nat,after:13nat,symbol:220nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:128nat,next:136nat,before:13nat,after:14nat,symbol:106nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:136nat,next:143nat,before:14nat,after:14nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_anc_ok_ztxt_with_corrupt_zlib_not_inflated_0() ensures t::trace(z_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),ts_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0nat,15nat),
{
    hide(t::valid_step);assert(t::trace(z_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),ts_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0nat,15nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn trace_anc_ok_ztxt_with_corrupt_zlib_not_inflated() ensures t::trace(z_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),ts_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0,15nat),
{
    hide(z_anc_ok_ztxt_with_corrupt_zlib_not_inflated);hide(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated);hide(ts_anc_ok_ztxt_with_corrupt_zlib_not_inflated);hide(t::trace);
    trace_anc_ok_ztxt_with_corrupt_zlib_not_inflated_0();
    assert(t::trace(z_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),ts_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0,15nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_anc_ok_ztxt_with_corrupt_zlib_not_inflated() ensures z_anc_ok_ztxt_with_corrupt_zlib_not_inflated().len()==22nat,
        raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated().len()==14nat,
        z_anc_ok_ztxt_with_corrupt_zlib_not_inflated()[0]==120u8,
        z_anc_ok_ztxt_with_corrupt_zlib_not_inflated()[1]==156u8,
        zm::field_value(z_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),16,1)==1nat,
        zm::field_value(z_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),17,2)==1nat,
        zm::be_value(z_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),18nat,4)==675349874nat,
        ts_anc_ok_ztxt_with_corrupt_zlib_not_inflated().len()==15nat,
        ts_anc_ok_ztxt_with_corrupt_zlib_not_inflated()[0].pos==19nat,
        ts_anc_ok_ztxt_with_corrupt_zlib_not_inflated()[0].before==0nat,
        ts_anc_ok_ztxt_with_corrupt_zlib_not_inflated().last().next==143nat,
{
    assert(z_anc_ok_ztxt_with_corrupt_zlib_not_inflated().len()==22nat) by (compute);
    assert(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated().len()==14nat) by (compute);
    assert(z_anc_ok_ztxt_with_corrupt_zlib_not_inflated()[0]==120u8) by (compute);
    assert(z_anc_ok_ztxt_with_corrupt_zlib_not_inflated()[1]==156u8) by (compute);
    assert(zm::field_value(z_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),18nat,4)==675349874nat) by (compute);
    assert(ts_anc_ok_ztxt_with_corrupt_zlib_not_inflated().len()==15nat) by (compute);
    assert(ts_anc_ok_ztxt_with_corrupt_zlib_not_inflated()[0].pos==19nat) by (compute);
    assert(ts_anc_ok_ztxt_with_corrupt_zlib_not_inflated()[0].before==0nat) by (compute);
    assert(ts_anc_ok_ztxt_with_corrupt_zlib_not_inflated().last().next==143nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_anc_ok_ztxt_with_corrupt_zlib_not_inflated() ensures zm::Adler(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated())==675349874nat,
{
    hide(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated().len()==14nat) by (compute);
    assert(c::sums(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0,14nat)==(1393nat,10305nat)) by (compute);
    c::sums_correct(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0,14nat);assert(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated().subrange(0,14)=~=raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((10305nat%65521)*(u16::MAX as nat+1)+(1+1393nat)%65521==675349874nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_anc_ok_ztxt_with_corrupt_zlib_not_inflated() ensures zm::Inflate(z_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),14nat,true,false)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),r2:false}),raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated().len()==14nat,
{
    hide(z_anc_ok_ztxt_with_corrupt_zlib_not_inflated);hide(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated);hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);hide(ts_anc_ok_ztxt_with_corrupt_zlib_not_inflated);
    shape_anc_ok_ztxt_with_corrupt_zlib_not_inflated();checksum_anc_ok_ztxt_with_corrupt_zlib_not_inflated();
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_anc_ok_ztxt_with_corrupt_zlib_not_inflated();ms::fixed_stream(z_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),ts_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),14nat,156nat,675349874nat,675349874nat);
}
#[verifier::spinoff_prover]
proof fn walk_anc_ok_ztxt_with_corrupt_zlib_not_inflated(b: Seq<u8>)
    requires b.len()==101,
        crate::model::mh(b)==Ok((h_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![122u8,84u8,88u8,116u8],data:41nat,len:10nat,next:55nat,crc_ok:true,synthetic:false})),
        crate::model::first_nul(b.subrange(41,51),0)==Some(1nat),
        b.subrange(41,51).subrange(0,1)==seq![107u8],
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,55nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:63nat,len:22nat,next:89nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,89nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:97nat,len:0nat,next:101nat,crc_ok:true,synthetic:false})),
        b.subrange(41,51)[2]==0u8,
        crate::metadata_model::parse(crate::walk_model::Name::Ztxt,b.subrange(41,51),2u8,8u8,0nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored),
        b.subrange(63,85)==z_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),
    ensures r::default_case(b,h_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),z_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0nat),match m::MW_p(b,m::strict()) {m::Outcome::Done(doc,_)=>doc.plte==None && doc.trns==None,_=>false},
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(m::store);hide(crate::metadata_model::parse);
    assert forall|doc: m::Doc,value: crate::metadata_model::Value,base: nat|
        (#[trigger] m::store(doc,value,base)).ihdr==doc.ihdr && m::store(doc,value,base).plte==doc.plte
        && m::store(doc,value,base).trns==doc.trns && m::store(doc,value,base).actl==doc.actl
        && m::store(doc,value,base).idat==doc.idat && m::store(doc,value,base).frames==doc.frames by {
        stored_structure(doc,value,base);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((122u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![122u8,84u8,88u8,116u8])==crate::walk_model::Name::Ztxt) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![122u8,84u8,88u8,116u8])==crate::walk_model::Name::Ztxt) by (compute);
        }
        assert(crate::model::keyword(seq![107u8]));
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    reveal_with_fuel(m::drive,4);
    reveal_with_fuel(m::concat_ranges,3);
}
#[verifier::spinoff_prover]
proof fn scan_anc_ok_ztxt_with_corrupt_zlib_not_inflated() ensures r::scan(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0)==Ok(()),
{
    hide(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated);assert(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated().len()==14nat) by (compute);assert(r::count(a_anc_ok_ztxt_with_corrupt_zlib_not_inflated())==1nat) by (compute);
    assert(r::pw(a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0nat)==2nat && r::ph(a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0nat)==2nat && r::rb(a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0nat)==6nat && r::base(a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0nat)==0nat) by (compute);
    assert(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated()[0]==0u8) by (compute);
    assert(r::row_result(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0nat,0nat)==Ok(()));
    assert(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated()[7]==1u8) by (compute);
    assert(r::row_result(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0nat,1nat)==Ok(()));
    r::scan_step(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),1nat);
    r::pass_step(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0nat,2nat);
    r::pass_step(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0nat,1nat);
    r::pass_step(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0nat,0nat);
    r::scan_step(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0nat);
}
#[verifier::spinoff_prover]
proof fn row_anc_ok_ztxt_with_corrupt_zlib_not_inflated_0_0() ensures r::line(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0nat,0nat)==seq![201u8,73u8,63u8,202u8,75u8,151u8],
{
    hide(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated);
    assert(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated().len()==14nat) by (compute);
    assert(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated()[0]==0u8) by (compute);
    assert(r::base(a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0nat)==0nat && r::rb(a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0nat)==6nat && f::bpp(r::bits(a_anc_ok_ztxt_with_corrupt_zlib_not_inflated()))==3nat) by (compute);
    assert(f::row_data(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0nat,6nat,0nat)==seq![201u8,73u8,63u8,202u8,75u8,151u8]) by (compute);
    f::pass_row_equation(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0nat,6nat,3nat,0nat);
    assert(f::previous_row(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0nat,6nat,3nat,0nat)==None);
    assert(f::encoded_row(0u8,f::row_data(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0nat,6nat,0nat),None,3nat,seq![201u8,73u8,63u8,202u8,75u8,151u8],0,6nat)) by (compute);
    f::checked_row_hint(0u8,f::row_data(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0nat,6nat,0nat),None,3nat,seq![201u8,73u8,63u8,202u8,75u8,151u8]);
}
#[verifier::spinoff_prover]
proof fn row_anc_ok_ztxt_with_corrupt_zlib_not_inflated_0_1() ensures r::line(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0nat,1nat)==seq![225u8,26u8,23u8,252u8,246u8,129u8],
{
    hide(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated);
    assert(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated().len()==14nat) by (compute);
    assert(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated()[7]==1u8) by (compute);
    assert(r::base(a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0nat)==0nat && r::rb(a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0nat)==6nat && f::bpp(r::bits(a_anc_ok_ztxt_with_corrupt_zlib_not_inflated()))==3nat) by (compute);
    assert(f::row_data(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0nat,6nat,1nat)==seq![225u8,26u8,23u8,27u8,220u8,106u8]) by (compute);
    row_anc_ok_ztxt_with_corrupt_zlib_not_inflated_0_0();
    f::pass_row_equation(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0nat,6nat,3nat,1nat);
    assert(f::previous_row(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0nat,6nat,3nat,1nat)==Some(seq![201u8,73u8,63u8,202u8,75u8,151u8]));
    assert(f::encoded_row(1u8,f::row_data(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0nat,6nat,1nat),Some(seq![201u8,73u8,63u8,202u8,75u8,151u8]),3nat,seq![225u8,26u8,23u8,252u8,246u8,129u8],0,6nat)) by (compute);
    f::checked_row_hint(1u8,f::row_data(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0nat,6nat,1nat),Some(seq![201u8,73u8,63u8,202u8,75u8,151u8]),3nat,seq![225u8,26u8,23u8,252u8,246u8,129u8]);
}
#[verifier::spinoff_prover]
proof fn raster_anc_ok_ztxt_with_corrupt_zlib_not_inflated()
    ensures r::default_case(bytes_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),h_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),z_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0nat),match m::MW_p(bytes_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),m::strict()) {m::Outcome::Done(doc,_)=>doc.plte==None && doc.trns==None,_=>false},r::data(z_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),STRICT)==Some(Ok(())),
{
    hide(bytes_anc_ok_ztxt_with_corrupt_zlib_not_inflated);hide(z_anc_ok_ztxt_with_corrupt_zlib_not_inflated);hide(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    assert(bytes_anc_ok_ztxt_with_corrupt_zlib_not_inflated().len()==101nat) by (compute);
    assert(crate::model::mh(bytes_anc_ok_ztxt_with_corrupt_zlib_not_inflated())==Ok((h_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),33nat))) by {reveal(crate::model::mh);assert(crate::model::mh(bytes_anc_ok_ztxt_with_corrupt_zlib_not_inflated())==Ok((h_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),33nat))) by (compute);}
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![122u8,84u8,88u8,116u8],data:41nat,len:10nat,next:55nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![122u8,84u8,88u8,116u8],data:41nat,len:10nat,next:55nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((122u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![122u8,84u8,88u8,116u8])==crate::walk_model::Name::Ztxt) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![122u8,84u8,88u8,116u8])==crate::walk_model::Name::Ztxt) by (compute);
        }
        assert(crate::model::first_nul(bytes_anc_ok_ztxt_with_corrupt_zlib_not_inflated().subrange(41,51),0)==Some(1nat)) by (compute);
        assert(bytes_anc_ok_ztxt_with_corrupt_zlib_not_inflated().subrange(41,51).subrange(0,1)==seq![107u8]) by (compute);
        assert(crate::model::keyword(seq![107u8]));
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),55nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:63nat,len:22nat,next:89nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),55nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:63nat,len:22nat,next:89nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),89nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:97nat,len:0nat,next:101nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),89nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:97nat,len:0nat,next:101nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(bytes_anc_ok_ztxt_with_corrupt_zlib_not_inflated().subrange(41,51)[2]==0u8) by (compute);
        assert(crate::metadata_model::parse(crate::walk_model::Name::Ztxt,bytes_anc_ok_ztxt_with_corrupt_zlib_not_inflated().subrange(41,51),2u8,8u8,0nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored)) by {
            reveal(crate::metadata_model::parse);
            ztxt_success(bytes_anc_ok_ztxt_with_corrupt_zlib_not_inflated().subrange(41,51),1nat);
        }
    assert(bytes_anc_ok_ztxt_with_corrupt_zlib_not_inflated().subrange(63,85)==z_anc_ok_ztxt_with_corrupt_zlib_not_inflated()) by (compute);
    walk_anc_ok_ztxt_with_corrupt_zlib_not_inflated(bytes_anc_ok_ztxt_with_corrupt_zlib_not_inflated());
    inflate_anc_ok_ztxt_with_corrupt_zlib_not_inflated();scan_anc_ok_ztxt_with_corrupt_zlib_not_inflated();
    assert(r::raw(a_anc_ok_ztxt_with_corrupt_zlib_not_inflated())==14nat) by (compute);
    assert(r::data(z_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),STRICT)==Some(Ok(()))) by {reveal(r::data);};
}
#[verifier::spinoff_prover]
proof fn pixel_anc_ok_ztxt_with_corrupt_zlib_not_inflated_0() ensures sm::image(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),colour_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),false)[0]==201u8,sm::image(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),colour_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),false)[1]==73u8,sm::image(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),colour_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),false)[2]==63u8,sm::image(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),colour_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),false)[3]==255u8,
{
    colour::depth_values(8u8);
    row_anc_ok_ztxt_with_corrupt_zlib_not_inflated_0_0();
    assert(crate::paint_model::source_cell(a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0nat,0nat)==(0nat,0nat,0nat)) by (compute);
    assert(colour::pixel(colour_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),seq![201u8,73u8,63u8,202u8,75u8,151u8],0nat,true,false)==Ok(seq![51657u16,18761u16,16191u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),colour_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0nat,0nat,true,false)==Ok(seq![51657u16,18761u16,16191u16,65535u16]));
    assert(((seq![51657u16,18761u16,16191u16,65535u16][0] as nat+128)/257) as u8==201u8) by (compute);
    assert(((seq![51657u16,18761u16,16191u16,65535u16][1] as nat+128)/257) as u8==73u8) by (compute);
    assert(((seq![51657u16,18761u16,16191u16,65535u16][2] as nat+128)/257) as u8==63u8) by (compute);
    assert(((seq![51657u16,18761u16,16191u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixel_anc_ok_ztxt_with_corrupt_zlib_not_inflated_1() ensures sm::image(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),colour_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),false)[4]==202u8,sm::image(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),colour_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),false)[5]==75u8,sm::image(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),colour_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),false)[6]==151u8,sm::image(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),colour_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),false)[7]==255u8,
{
    colour::depth_values(8u8);
    row_anc_ok_ztxt_with_corrupt_zlib_not_inflated_0_0();
    assert(crate::paint_model::source_cell(a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),1nat,0nat)==(0nat,1nat,0nat)) by (compute);
    assert(colour::pixel(colour_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),seq![201u8,73u8,63u8,202u8,75u8,151u8],1nat,true,false)==Ok(seq![51914u16,19275u16,38807u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),colour_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),1nat,0nat,true,false)==Ok(seq![51914u16,19275u16,38807u16,65535u16]));
    assert(((seq![51914u16,19275u16,38807u16,65535u16][0] as nat+128)/257) as u8==202u8) by (compute);
    assert(((seq![51914u16,19275u16,38807u16,65535u16][1] as nat+128)/257) as u8==75u8) by (compute);
    assert(((seq![51914u16,19275u16,38807u16,65535u16][2] as nat+128)/257) as u8==151u8) by (compute);
    assert(((seq![51914u16,19275u16,38807u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixel_anc_ok_ztxt_with_corrupt_zlib_not_inflated_2() ensures sm::image(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),colour_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),false)[8]==225u8,sm::image(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),colour_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),false)[9]==26u8,sm::image(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),colour_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),false)[10]==23u8,sm::image(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),colour_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),false)[11]==255u8,
{
    colour::depth_values(8u8);
    row_anc_ok_ztxt_with_corrupt_zlib_not_inflated_0_1();
    assert(crate::paint_model::source_cell(a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0nat,1nat)==(0nat,0nat,1nat)) by (compute);
    assert(colour::pixel(colour_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),seq![225u8,26u8,23u8,252u8,246u8,129u8],0nat,true,false)==Ok(seq![57825u16,6682u16,5911u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),colour_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0nat,1nat,true,false)==Ok(seq![57825u16,6682u16,5911u16,65535u16]));
    assert(((seq![57825u16,6682u16,5911u16,65535u16][0] as nat+128)/257) as u8==225u8) by (compute);
    assert(((seq![57825u16,6682u16,5911u16,65535u16][1] as nat+128)/257) as u8==26u8) by (compute);
    assert(((seq![57825u16,6682u16,5911u16,65535u16][2] as nat+128)/257) as u8==23u8) by (compute);
    assert(((seq![57825u16,6682u16,5911u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixel_anc_ok_ztxt_with_corrupt_zlib_not_inflated_3() ensures sm::image(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),colour_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),false)[12]==252u8,sm::image(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),colour_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),false)[13]==246u8,sm::image(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),colour_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),false)[14]==129u8,sm::image(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),colour_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),false)[15]==255u8,
{
    colour::depth_values(8u8);
    row_anc_ok_ztxt_with_corrupt_zlib_not_inflated_0_1();
    assert(crate::paint_model::source_cell(a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),1nat,1nat)==(0nat,1nat,1nat)) by (compute);
    assert(colour::pixel(colour_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),seq![225u8,26u8,23u8,252u8,246u8,129u8],1nat,true,false)==Ok(seq![64764u16,63222u16,33153u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),colour_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),1nat,1nat,true,false)==Ok(seq![64764u16,63222u16,33153u16,65535u16]));
    assert(((seq![64764u16,63222u16,33153u16,65535u16][0] as nat+128)/257) as u8==252u8) by (compute);
    assert(((seq![64764u16,63222u16,33153u16,65535u16][1] as nat+128)/257) as u8==246u8) by (compute);
    assert(((seq![64764u16,63222u16,33153u16,65535u16][2] as nat+128)/257) as u8==129u8) by (compute);
    assert(((seq![64764u16,63222u16,33153u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixels_anc_ok_ztxt_with_corrupt_zlib_not_inflated() ensures sm::image(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),colour_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),false)==expected_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),
{
    pixel_anc_ok_ztxt_with_corrupt_zlib_not_inflated_0();
    pixel_anc_ok_ztxt_with_corrupt_zlib_not_inflated_1();
    pixel_anc_ok_ztxt_with_corrupt_zlib_not_inflated_2();
    pixel_anc_ok_ztxt_with_corrupt_zlib_not_inflated_3();
    assert(a_anc_ok_ztxt_with_corrupt_zlib_not_inflated().width==2nat && a_anc_ok_ztxt_with_corrupt_zlib_not_inflated().height==2nat) by (compute);
    assert(sm::image(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),colour_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),false).len()==16nat);
    assert(expected_anc_ok_ztxt_with_corrupt_zlib_not_inflated().len()==16nat) by (compute);
    assert forall|j: int| 0<=j<16 implies (#[trigger] sm::image(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),colour_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),false)[j])==expected_anc_ok_ztxt_with_corrupt_zlib_not_inflated()[j] by {
        if j==0 {assert(expected_anc_ok_ztxt_with_corrupt_zlib_not_inflated()[0]==201u8) by (compute);}
        else if j==1 {assert(expected_anc_ok_ztxt_with_corrupt_zlib_not_inflated()[1]==73u8) by (compute);}
        else if j==2 {assert(expected_anc_ok_ztxt_with_corrupt_zlib_not_inflated()[2]==63u8) by (compute);}
        else if j==3 {assert(expected_anc_ok_ztxt_with_corrupt_zlib_not_inflated()[3]==255u8) by (compute);}
        else if j==4 {assert(expected_anc_ok_ztxt_with_corrupt_zlib_not_inflated()[4]==202u8) by (compute);}
        else if j==5 {assert(expected_anc_ok_ztxt_with_corrupt_zlib_not_inflated()[5]==75u8) by (compute);}
        else if j==6 {assert(expected_anc_ok_ztxt_with_corrupt_zlib_not_inflated()[6]==151u8) by (compute);}
        else if j==7 {assert(expected_anc_ok_ztxt_with_corrupt_zlib_not_inflated()[7]==255u8) by (compute);}
        else if j==8 {assert(expected_anc_ok_ztxt_with_corrupt_zlib_not_inflated()[8]==225u8) by (compute);}
        else if j==9 {assert(expected_anc_ok_ztxt_with_corrupt_zlib_not_inflated()[9]==26u8) by (compute);}
        else if j==10 {assert(expected_anc_ok_ztxt_with_corrupt_zlib_not_inflated()[10]==23u8) by (compute);}
        else if j==11 {assert(expected_anc_ok_ztxt_with_corrupt_zlib_not_inflated()[11]==255u8) by (compute);}
        else if j==12 {assert(expected_anc_ok_ztxt_with_corrupt_zlib_not_inflated()[12]==252u8) by (compute);}
        else if j==13 {assert(expected_anc_ok_ztxt_with_corrupt_zlib_not_inflated()[13]==246u8) by (compute);}
        else if j==14 {assert(expected_anc_ok_ztxt_with_corrupt_zlib_not_inflated()[14]==129u8) by (compute);}
        else {assert(j==15);assert(expected_anc_ok_ztxt_with_corrupt_zlib_not_inflated()[j]==255u8) by (compute);}
    }
    assert(sm::image(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),colour_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),false)=~=expected_anc_ok_ztxt_with_corrupt_zlib_not_inflated());
}
pub open spec fn expected_value_anc_ok_ztxt_with_corrupt_zlib_not_inflated(value: sm::Value) -> bool {value.width==2u32 && value.height==2u32 && value.pixels==expected_anc_ok_ztxt_with_corrupt_zlib_not_inflated()}
#[verifier::spinoff_prover]
pub proof fn model_complete_anc_ok_ztxt_with_corrupt_zlib_not_inflated()
    ensures match sm::MS(bytes_anc_ok_ztxt_with_corrupt_zlib_not_inflated()) {sm::Outcome::Done(value)=>expected_value_anc_ok_ztxt_with_corrupt_zlib_not_inflated(value),_=>false},
{
    hide(m::MW_p);hide(zm::Inflate);hide(bytes_anc_ok_ztxt_with_corrupt_zlib_not_inflated);hide(z_anc_ok_ztxt_with_corrupt_zlib_not_inflated);hide(raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated);hide(expected_anc_ok_ztxt_with_corrupt_zlib_not_inflated);
    assert(bytes_anc_ok_ztxt_with_corrupt_zlib_not_inflated().len()==101nat) by (compute);
    raster_anc_ok_ztxt_with_corrupt_zlib_not_inflated();inflate_anc_ok_ztxt_with_corrupt_zlib_not_inflated();pixels_anc_ok_ztxt_with_corrupt_zlib_not_inflated();
    match m::MW_p(bytes_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),m::strict()) {m::Outcome::Done(doc,_)=>{
        assert(colour::from_doc(doc,bytes_anc_ok_ztxt_with_corrupt_zlib_not_inflated())==colour_anc_ok_ztxt_with_corrupt_zlib_not_inflated());
        assert(crate::paint_model::default_raster(doc,STRICT)==a_anc_ok_ztxt_with_corrupt_zlib_not_inflated());
    },_=>{}}
    assert(r::raw(a_anc_ok_ztxt_with_corrupt_zlib_not_inflated())==14nat) by (compute);
    assert(r::inflated(z_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),a_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),STRICT).out==raw_anc_ok_ztxt_with_corrupt_zlib_not_inflated());
    compose_complete(bytes_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),h_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),z_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),0nat,colour_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),expected_anc_ok_ztxt_with_corrupt_zlib_not_inflated());
}
pub fn public_complete_anc_ok_ztxt_with_corrupt_zlib_not_inflated() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),false),result),se::eq(bytes_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_anc_ok_ztxt_with_corrupt_zlib_not_inflated(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,10u8,122u8,84u8,88u8,116u8,107u8,0u8,0u8,103u8,97u8,114u8,98u8,97u8,103u8,101u8,188u8,174u8,84u8,233u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_anc_ok_ztxt_with_corrupt_zlib_not_inflated());model_complete_anc_ok_ztxt_with_corrupt_zlib_not_inflated();}
    crate::png::try_decode_still(&b)
}
// order/splt-twice; frozen fixture SHA256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
pub open spec fn bytes_order_splt_twice() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,3u8,115u8,80u8,76u8,84u8,97u8,0u8,8u8,210u8,192u8,25u8,167u8,0u8,0u8,0u8,3u8,115u8,80u8,76u8,84u8,98u8,0u8,8u8,208u8,134u8,167u8,254u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn h_order_splt_twice() -> crate::model::Ihdr {crate::model::Ihdr {width:2u32,height:2u32,depth:8u8,color_type:2u8,interlace:0u8}}
spec fn a_order_splt_twice() -> r::Raster {r::shape(h_order_splt_twice(),2nat,2nat,0nat,STRICT)}
spec fn colour_order_splt_twice() -> crate::colour_model::Context {crate::colour_model::Context {ct:2u8,depth:8u8,plte:seq![],trns:None}}
pub open spec fn expected_order_splt_twice() -> Seq<u8> {seq![201u8,73u8,63u8,255u8,202u8,75u8,151u8,255u8,225u8,26u8,23u8,255u8,252u8,246u8,129u8,255u8]}
spec fn z_order_splt_twice() -> Seq<u8> {seq![120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8]}
spec fn raw_order_splt_twice() -> Seq<u8> {seq![0u8,201u8,73u8,63u8,202u8,75u8,151u8,1u8,225u8,26u8,23u8,27u8,220u8,106u8]}
spec fn ts_order_splt_twice() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:36nat,before:1nat,after:2nat,symbol:201nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:36nat,next:44nat,before:2nat,after:3nat,symbol:73nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:44nat,next:52nat,before:3nat,after:4nat,symbol:63nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:52nat,next:61nat,before:4nat,after:5nat,symbol:202nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:61nat,next:69nat,before:5nat,after:6nat,symbol:75nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:69nat,next:78nat,before:6nat,after:7nat,symbol:151nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:78nat,next:86nat,before:7nat,after:8nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:86nat,next:95nat,before:8nat,after:9nat,symbol:225nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:95nat,next:103nat,before:9nat,after:10nat,symbol:26nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:103nat,next:111nat,before:10nat,after:11nat,symbol:23nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:111nat,next:119nat,before:11nat,after:12nat,symbol:27nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:119nat,next:128nat,before:12nat,after:13nat,symbol:220nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:128nat,next:136nat,before:13nat,after:14nat,symbol:106nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:136nat,next:143nat,before:14nat,after:14nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_order_splt_twice_0() ensures t::trace(z_order_splt_twice(),raw_order_splt_twice(),ts_order_splt_twice(),0nat,15nat),
{
    hide(t::valid_step);assert(t::trace(z_order_splt_twice(),raw_order_splt_twice(),ts_order_splt_twice(),0nat,15nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn trace_order_splt_twice() ensures t::trace(z_order_splt_twice(),raw_order_splt_twice(),ts_order_splt_twice(),0,15nat),
{
    hide(z_order_splt_twice);hide(raw_order_splt_twice);hide(ts_order_splt_twice);hide(t::trace);
    trace_order_splt_twice_0();
    assert(t::trace(z_order_splt_twice(),raw_order_splt_twice(),ts_order_splt_twice(),0,15nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_order_splt_twice() ensures z_order_splt_twice().len()==22nat,
        raw_order_splt_twice().len()==14nat,
        z_order_splt_twice()[0]==120u8,
        z_order_splt_twice()[1]==156u8,
        zm::field_value(z_order_splt_twice(),16,1)==1nat,
        zm::field_value(z_order_splt_twice(),17,2)==1nat,
        zm::be_value(z_order_splt_twice(),18nat,4)==675349874nat,
        ts_order_splt_twice().len()==15nat,
        ts_order_splt_twice()[0].pos==19nat,
        ts_order_splt_twice()[0].before==0nat,
        ts_order_splt_twice().last().next==143nat,
{
    assert(z_order_splt_twice().len()==22nat) by (compute);
    assert(raw_order_splt_twice().len()==14nat) by (compute);
    assert(z_order_splt_twice()[0]==120u8) by (compute);
    assert(z_order_splt_twice()[1]==156u8) by (compute);
    assert(zm::field_value(z_order_splt_twice(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_order_splt_twice(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_order_splt_twice(),18nat,4)==675349874nat) by (compute);
    assert(ts_order_splt_twice().len()==15nat) by (compute);
    assert(ts_order_splt_twice()[0].pos==19nat) by (compute);
    assert(ts_order_splt_twice()[0].before==0nat) by (compute);
    assert(ts_order_splt_twice().last().next==143nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_order_splt_twice() ensures zm::Adler(raw_order_splt_twice())==675349874nat,
{
    hide(raw_order_splt_twice);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_order_splt_twice().len()==14nat) by (compute);
    assert(c::sums(raw_order_splt_twice(),0,14nat)==(1393nat,10305nat)) by (compute);
    c::sums_correct(raw_order_splt_twice(),0,14nat);assert(raw_order_splt_twice().subrange(0,14)=~=raw_order_splt_twice());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((10305nat%65521)*(u16::MAX as nat+1)+(1+1393nat)%65521==675349874nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_order_splt_twice() ensures zm::Inflate(z_order_splt_twice(),14nat,true,false)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_order_splt_twice(),r2:false}),raw_order_splt_twice().len()==14nat,
{
    hide(z_order_splt_twice);hide(raw_order_splt_twice);hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);hide(ts_order_splt_twice);
    shape_order_splt_twice();checksum_order_splt_twice();
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_order_splt_twice();ms::fixed_stream(z_order_splt_twice(),raw_order_splt_twice(),ts_order_splt_twice(),14nat,156nat,675349874nat,675349874nat);
}
#[verifier::spinoff_prover]
proof fn walk_order_splt_twice(b: Seq<u8>)
    requires b.len()==109,
        crate::model::mh(b)==Ok((h_order_splt_twice(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![115u8,80u8,76u8,84u8],data:41nat,len:3nat,next:48nat,crc_ok:true,synthetic:false})),
        crate::model::first_nul(b.subrange(41,44),0)==Some(1nat),
        b.subrange(41,44).subrange(0,1)==seq![97u8],
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,48nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![115u8,80u8,76u8,84u8],data:56nat,len:3nat,next:63nat,crc_ok:true,synthetic:false})),
        crate::model::first_nul(b.subrange(56,59),0)==Some(1nat),
        b.subrange(56,59).subrange(0,1)==seq![98u8],
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,63nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:71nat,len:22nat,next:97nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,97nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:105nat,len:0nat,next:109nat,crc_ok:true,synthetic:false})),
        b.subrange(41,44)[2]==8u8,
        crate::metadata_model::parse(crate::walk_model::Name::Splt,b.subrange(41,44),2u8,8u8,0nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored),
        b.subrange(56,59)[2]==8u8,
        crate::metadata_model::parse(crate::walk_model::Name::Splt,b.subrange(56,59),2u8,8u8,0nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored),
        b.subrange(71,93)==z_order_splt_twice(),
    ensures r::default_case(b,h_order_splt_twice(),z_order_splt_twice(),0nat),match m::MW_p(b,m::strict()) {m::Outcome::Done(doc,_)=>doc.plte==None && doc.trns==None,_=>false},
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(m::store);hide(crate::metadata_model::parse);
    assert forall|doc: m::Doc,value: crate::metadata_model::Value,base: nat|
        (#[trigger] m::store(doc,value,base)).ihdr==doc.ihdr && m::store(doc,value,base).plte==doc.plte
        && m::store(doc,value,base).trns==doc.trns && m::store(doc,value,base).actl==doc.actl
        && m::store(doc,value,base).idat==doc.idat && m::store(doc,value,base).frames==doc.frames by {
        stored_structure(doc,value,base);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((115u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![115u8,80u8,76u8,84u8])==crate::walk_model::Name::Splt) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![115u8,80u8,76u8,84u8])==crate::walk_model::Name::Splt) by (compute);
        }
        assert(crate::model::keyword(seq![97u8]));
        assert((115u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![115u8,80u8,76u8,84u8])==crate::walk_model::Name::Splt) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![115u8,80u8,76u8,84u8])==crate::walk_model::Name::Splt) by (compute);
        }
        assert(crate::model::keyword(seq![98u8]));
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    reveal_with_fuel(m::drive,5);
    reveal_with_fuel(m::concat_ranges,3);
}
#[verifier::spinoff_prover]
proof fn scan_order_splt_twice() ensures r::scan(raw_order_splt_twice(),a_order_splt_twice(),0)==Ok(()),
{
    hide(raw_order_splt_twice);assert(raw_order_splt_twice().len()==14nat) by (compute);assert(r::count(a_order_splt_twice())==1nat) by (compute);
    assert(r::pw(a_order_splt_twice(),0nat)==2nat && r::ph(a_order_splt_twice(),0nat)==2nat && r::rb(a_order_splt_twice(),0nat)==6nat && r::base(a_order_splt_twice(),0nat)==0nat) by (compute);
    assert(raw_order_splt_twice()[0]==0u8) by (compute);
    assert(r::row_result(raw_order_splt_twice(),a_order_splt_twice(),0nat,0nat)==Ok(()));
    assert(raw_order_splt_twice()[7]==1u8) by (compute);
    assert(r::row_result(raw_order_splt_twice(),a_order_splt_twice(),0nat,1nat)==Ok(()));
    r::scan_step(raw_order_splt_twice(),a_order_splt_twice(),1nat);
    r::pass_step(raw_order_splt_twice(),a_order_splt_twice(),0nat,2nat);
    r::pass_step(raw_order_splt_twice(),a_order_splt_twice(),0nat,1nat);
    r::pass_step(raw_order_splt_twice(),a_order_splt_twice(),0nat,0nat);
    r::scan_step(raw_order_splt_twice(),a_order_splt_twice(),0nat);
}
#[verifier::spinoff_prover]
proof fn row_order_splt_twice_0_0() ensures r::line(raw_order_splt_twice(),a_order_splt_twice(),0nat,0nat)==seq![201u8,73u8,63u8,202u8,75u8,151u8],
{
    hide(raw_order_splt_twice);
    assert(raw_order_splt_twice().len()==14nat) by (compute);
    assert(raw_order_splt_twice()[0]==0u8) by (compute);
    assert(r::base(a_order_splt_twice(),0nat)==0nat && r::rb(a_order_splt_twice(),0nat)==6nat && f::bpp(r::bits(a_order_splt_twice()))==3nat) by (compute);
    assert(f::row_data(raw_order_splt_twice(),0nat,6nat,0nat)==seq![201u8,73u8,63u8,202u8,75u8,151u8]) by (compute);
    f::pass_row_equation(raw_order_splt_twice(),0nat,6nat,3nat,0nat);
    assert(f::previous_row(raw_order_splt_twice(),0nat,6nat,3nat,0nat)==None);
    assert(f::encoded_row(0u8,f::row_data(raw_order_splt_twice(),0nat,6nat,0nat),None,3nat,seq![201u8,73u8,63u8,202u8,75u8,151u8],0,6nat)) by (compute);
    f::checked_row_hint(0u8,f::row_data(raw_order_splt_twice(),0nat,6nat,0nat),None,3nat,seq![201u8,73u8,63u8,202u8,75u8,151u8]);
}
#[verifier::spinoff_prover]
proof fn row_order_splt_twice_0_1() ensures r::line(raw_order_splt_twice(),a_order_splt_twice(),0nat,1nat)==seq![225u8,26u8,23u8,252u8,246u8,129u8],
{
    hide(raw_order_splt_twice);
    assert(raw_order_splt_twice().len()==14nat) by (compute);
    assert(raw_order_splt_twice()[7]==1u8) by (compute);
    assert(r::base(a_order_splt_twice(),0nat)==0nat && r::rb(a_order_splt_twice(),0nat)==6nat && f::bpp(r::bits(a_order_splt_twice()))==3nat) by (compute);
    assert(f::row_data(raw_order_splt_twice(),0nat,6nat,1nat)==seq![225u8,26u8,23u8,27u8,220u8,106u8]) by (compute);
    row_order_splt_twice_0_0();
    f::pass_row_equation(raw_order_splt_twice(),0nat,6nat,3nat,1nat);
    assert(f::previous_row(raw_order_splt_twice(),0nat,6nat,3nat,1nat)==Some(seq![201u8,73u8,63u8,202u8,75u8,151u8]));
    assert(f::encoded_row(1u8,f::row_data(raw_order_splt_twice(),0nat,6nat,1nat),Some(seq![201u8,73u8,63u8,202u8,75u8,151u8]),3nat,seq![225u8,26u8,23u8,252u8,246u8,129u8],0,6nat)) by (compute);
    f::checked_row_hint(1u8,f::row_data(raw_order_splt_twice(),0nat,6nat,1nat),Some(seq![201u8,73u8,63u8,202u8,75u8,151u8]),3nat,seq![225u8,26u8,23u8,252u8,246u8,129u8]);
}
#[verifier::spinoff_prover]
proof fn raster_order_splt_twice()
    ensures r::default_case(bytes_order_splt_twice(),h_order_splt_twice(),z_order_splt_twice(),0nat),match m::MW_p(bytes_order_splt_twice(),m::strict()) {m::Outcome::Done(doc,_)=>doc.plte==None && doc.trns==None,_=>false},r::data(z_order_splt_twice(),a_order_splt_twice(),STRICT)==Some(Ok(())),
{
    hide(bytes_order_splt_twice);hide(z_order_splt_twice);hide(raw_order_splt_twice);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    assert(bytes_order_splt_twice().len()==109nat) by (compute);
    assert(crate::model::mh(bytes_order_splt_twice())==Ok((h_order_splt_twice(),33nat))) by {reveal(crate::model::mh);assert(crate::model::mh(bytes_order_splt_twice())==Ok((h_order_splt_twice(),33nat))) by (compute);}
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_order_splt_twice(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![115u8,80u8,76u8,84u8],data:41nat,len:3nat,next:48nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_order_splt_twice(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![115u8,80u8,76u8,84u8],data:41nat,len:3nat,next:48nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((115u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![115u8,80u8,76u8,84u8])==crate::walk_model::Name::Splt) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![115u8,80u8,76u8,84u8])==crate::walk_model::Name::Splt) by (compute);
        }
        assert(crate::model::first_nul(bytes_order_splt_twice().subrange(41,44),0)==Some(1nat)) by (compute);
        assert(bytes_order_splt_twice().subrange(41,44).subrange(0,1)==seq![97u8]) by (compute);
        assert(crate::model::keyword(seq![97u8]));
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_order_splt_twice(),48nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![115u8,80u8,76u8,84u8],data:56nat,len:3nat,next:63nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_order_splt_twice(),48nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![115u8,80u8,76u8,84u8],data:56nat,len:3nat,next:63nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((115u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![115u8,80u8,76u8,84u8])==crate::walk_model::Name::Splt) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![115u8,80u8,76u8,84u8])==crate::walk_model::Name::Splt) by (compute);
        }
        assert(crate::model::first_nul(bytes_order_splt_twice().subrange(56,59),0)==Some(1nat)) by (compute);
        assert(bytes_order_splt_twice().subrange(56,59).subrange(0,1)==seq![98u8]) by (compute);
        assert(crate::model::keyword(seq![98u8]));
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_order_splt_twice(),63nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:71nat,len:22nat,next:97nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_order_splt_twice(),63nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:71nat,len:22nat,next:97nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_order_splt_twice(),97nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:105nat,len:0nat,next:109nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_order_splt_twice(),97nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:105nat,len:0nat,next:109nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(bytes_order_splt_twice().subrange(41,44)[2]==8u8) by (compute);
        assert(crate::metadata_model::parse(crate::walk_model::Name::Splt,bytes_order_splt_twice().subrange(41,44),2u8,8u8,0nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored)) by {
            reveal(crate::metadata_model::parse);
        }
        assert(bytes_order_splt_twice().subrange(56,59)[2]==8u8) by (compute);
        assert(crate::metadata_model::parse(crate::walk_model::Name::Splt,bytes_order_splt_twice().subrange(56,59),2u8,8u8,0nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored)) by {
            reveal(crate::metadata_model::parse);
        }
    assert(bytes_order_splt_twice().subrange(71,93)==z_order_splt_twice()) by (compute);
    walk_order_splt_twice(bytes_order_splt_twice());
    inflate_order_splt_twice();scan_order_splt_twice();
    assert(r::raw(a_order_splt_twice())==14nat) by (compute);
    assert(r::data(z_order_splt_twice(),a_order_splt_twice(),STRICT)==Some(Ok(()))) by {reveal(r::data);};
}
#[verifier::spinoff_prover]
proof fn pixel_order_splt_twice_0() ensures sm::image(raw_order_splt_twice(),a_order_splt_twice(),colour_order_splt_twice(),false)[0]==201u8,sm::image(raw_order_splt_twice(),a_order_splt_twice(),colour_order_splt_twice(),false)[1]==73u8,sm::image(raw_order_splt_twice(),a_order_splt_twice(),colour_order_splt_twice(),false)[2]==63u8,sm::image(raw_order_splt_twice(),a_order_splt_twice(),colour_order_splt_twice(),false)[3]==255u8,
{
    colour::depth_values(8u8);
    row_order_splt_twice_0_0();
    assert(crate::paint_model::source_cell(a_order_splt_twice(),0nat,0nat)==(0nat,0nat,0nat)) by (compute);
    assert(colour::pixel(colour_order_splt_twice(),seq![201u8,73u8,63u8,202u8,75u8,151u8],0nat,true,false)==Ok(seq![51657u16,18761u16,16191u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_order_splt_twice(),a_order_splt_twice(),colour_order_splt_twice(),0nat,0nat,true,false)==Ok(seq![51657u16,18761u16,16191u16,65535u16]));
    assert(((seq![51657u16,18761u16,16191u16,65535u16][0] as nat+128)/257) as u8==201u8) by (compute);
    assert(((seq![51657u16,18761u16,16191u16,65535u16][1] as nat+128)/257) as u8==73u8) by (compute);
    assert(((seq![51657u16,18761u16,16191u16,65535u16][2] as nat+128)/257) as u8==63u8) by (compute);
    assert(((seq![51657u16,18761u16,16191u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixel_order_splt_twice_1() ensures sm::image(raw_order_splt_twice(),a_order_splt_twice(),colour_order_splt_twice(),false)[4]==202u8,sm::image(raw_order_splt_twice(),a_order_splt_twice(),colour_order_splt_twice(),false)[5]==75u8,sm::image(raw_order_splt_twice(),a_order_splt_twice(),colour_order_splt_twice(),false)[6]==151u8,sm::image(raw_order_splt_twice(),a_order_splt_twice(),colour_order_splt_twice(),false)[7]==255u8,
{
    colour::depth_values(8u8);
    row_order_splt_twice_0_0();
    assert(crate::paint_model::source_cell(a_order_splt_twice(),1nat,0nat)==(0nat,1nat,0nat)) by (compute);
    assert(colour::pixel(colour_order_splt_twice(),seq![201u8,73u8,63u8,202u8,75u8,151u8],1nat,true,false)==Ok(seq![51914u16,19275u16,38807u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_order_splt_twice(),a_order_splt_twice(),colour_order_splt_twice(),1nat,0nat,true,false)==Ok(seq![51914u16,19275u16,38807u16,65535u16]));
    assert(((seq![51914u16,19275u16,38807u16,65535u16][0] as nat+128)/257) as u8==202u8) by (compute);
    assert(((seq![51914u16,19275u16,38807u16,65535u16][1] as nat+128)/257) as u8==75u8) by (compute);
    assert(((seq![51914u16,19275u16,38807u16,65535u16][2] as nat+128)/257) as u8==151u8) by (compute);
    assert(((seq![51914u16,19275u16,38807u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixel_order_splt_twice_2() ensures sm::image(raw_order_splt_twice(),a_order_splt_twice(),colour_order_splt_twice(),false)[8]==225u8,sm::image(raw_order_splt_twice(),a_order_splt_twice(),colour_order_splt_twice(),false)[9]==26u8,sm::image(raw_order_splt_twice(),a_order_splt_twice(),colour_order_splt_twice(),false)[10]==23u8,sm::image(raw_order_splt_twice(),a_order_splt_twice(),colour_order_splt_twice(),false)[11]==255u8,
{
    colour::depth_values(8u8);
    row_order_splt_twice_0_1();
    assert(crate::paint_model::source_cell(a_order_splt_twice(),0nat,1nat)==(0nat,0nat,1nat)) by (compute);
    assert(colour::pixel(colour_order_splt_twice(),seq![225u8,26u8,23u8,252u8,246u8,129u8],0nat,true,false)==Ok(seq![57825u16,6682u16,5911u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_order_splt_twice(),a_order_splt_twice(),colour_order_splt_twice(),0nat,1nat,true,false)==Ok(seq![57825u16,6682u16,5911u16,65535u16]));
    assert(((seq![57825u16,6682u16,5911u16,65535u16][0] as nat+128)/257) as u8==225u8) by (compute);
    assert(((seq![57825u16,6682u16,5911u16,65535u16][1] as nat+128)/257) as u8==26u8) by (compute);
    assert(((seq![57825u16,6682u16,5911u16,65535u16][2] as nat+128)/257) as u8==23u8) by (compute);
    assert(((seq![57825u16,6682u16,5911u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixel_order_splt_twice_3() ensures sm::image(raw_order_splt_twice(),a_order_splt_twice(),colour_order_splt_twice(),false)[12]==252u8,sm::image(raw_order_splt_twice(),a_order_splt_twice(),colour_order_splt_twice(),false)[13]==246u8,sm::image(raw_order_splt_twice(),a_order_splt_twice(),colour_order_splt_twice(),false)[14]==129u8,sm::image(raw_order_splt_twice(),a_order_splt_twice(),colour_order_splt_twice(),false)[15]==255u8,
{
    colour::depth_values(8u8);
    row_order_splt_twice_0_1();
    assert(crate::paint_model::source_cell(a_order_splt_twice(),1nat,1nat)==(0nat,1nat,1nat)) by (compute);
    assert(colour::pixel(colour_order_splt_twice(),seq![225u8,26u8,23u8,252u8,246u8,129u8],1nat,true,false)==Ok(seq![64764u16,63222u16,33153u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_order_splt_twice(),a_order_splt_twice(),colour_order_splt_twice(),1nat,1nat,true,false)==Ok(seq![64764u16,63222u16,33153u16,65535u16]));
    assert(((seq![64764u16,63222u16,33153u16,65535u16][0] as nat+128)/257) as u8==252u8) by (compute);
    assert(((seq![64764u16,63222u16,33153u16,65535u16][1] as nat+128)/257) as u8==246u8) by (compute);
    assert(((seq![64764u16,63222u16,33153u16,65535u16][2] as nat+128)/257) as u8==129u8) by (compute);
    assert(((seq![64764u16,63222u16,33153u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixels_order_splt_twice() ensures sm::image(raw_order_splt_twice(),a_order_splt_twice(),colour_order_splt_twice(),false)==expected_order_splt_twice(),
{
    pixel_order_splt_twice_0();
    pixel_order_splt_twice_1();
    pixel_order_splt_twice_2();
    pixel_order_splt_twice_3();
    assert(a_order_splt_twice().width==2nat && a_order_splt_twice().height==2nat) by (compute);
    assert(sm::image(raw_order_splt_twice(),a_order_splt_twice(),colour_order_splt_twice(),false).len()==16nat);
    assert(expected_order_splt_twice().len()==16nat) by (compute);
    assert forall|j: int| 0<=j<16 implies (#[trigger] sm::image(raw_order_splt_twice(),a_order_splt_twice(),colour_order_splt_twice(),false)[j])==expected_order_splt_twice()[j] by {
        if j==0 {assert(expected_order_splt_twice()[0]==201u8) by (compute);}
        else if j==1 {assert(expected_order_splt_twice()[1]==73u8) by (compute);}
        else if j==2 {assert(expected_order_splt_twice()[2]==63u8) by (compute);}
        else if j==3 {assert(expected_order_splt_twice()[3]==255u8) by (compute);}
        else if j==4 {assert(expected_order_splt_twice()[4]==202u8) by (compute);}
        else if j==5 {assert(expected_order_splt_twice()[5]==75u8) by (compute);}
        else if j==6 {assert(expected_order_splt_twice()[6]==151u8) by (compute);}
        else if j==7 {assert(expected_order_splt_twice()[7]==255u8) by (compute);}
        else if j==8 {assert(expected_order_splt_twice()[8]==225u8) by (compute);}
        else if j==9 {assert(expected_order_splt_twice()[9]==26u8) by (compute);}
        else if j==10 {assert(expected_order_splt_twice()[10]==23u8) by (compute);}
        else if j==11 {assert(expected_order_splt_twice()[11]==255u8) by (compute);}
        else if j==12 {assert(expected_order_splt_twice()[12]==252u8) by (compute);}
        else if j==13 {assert(expected_order_splt_twice()[13]==246u8) by (compute);}
        else if j==14 {assert(expected_order_splt_twice()[14]==129u8) by (compute);}
        else {assert(j==15);assert(expected_order_splt_twice()[j]==255u8) by (compute);}
    }
    assert(sm::image(raw_order_splt_twice(),a_order_splt_twice(),colour_order_splt_twice(),false)=~=expected_order_splt_twice());
}
pub open spec fn expected_value_order_splt_twice(value: sm::Value) -> bool {value.width==2u32 && value.height==2u32 && value.pixels==expected_order_splt_twice()}
#[verifier::spinoff_prover]
pub proof fn model_complete_order_splt_twice()
    ensures match sm::MS(bytes_order_splt_twice()) {sm::Outcome::Done(value)=>expected_value_order_splt_twice(value),_=>false},
{
    hide(m::MW_p);hide(zm::Inflate);hide(bytes_order_splt_twice);hide(z_order_splt_twice);hide(raw_order_splt_twice);hide(expected_order_splt_twice);
    assert(bytes_order_splt_twice().len()==109nat) by (compute);
    raster_order_splt_twice();inflate_order_splt_twice();pixels_order_splt_twice();
    match m::MW_p(bytes_order_splt_twice(),m::strict()) {m::Outcome::Done(doc,_)=>{
        assert(colour::from_doc(doc,bytes_order_splt_twice())==colour_order_splt_twice());
        assert(crate::paint_model::default_raster(doc,STRICT)==a_order_splt_twice());
    },_=>{}}
    assert(r::raw(a_order_splt_twice())==14nat) by (compute);
    assert(r::inflated(z_order_splt_twice(),a_order_splt_twice(),STRICT).out==raw_order_splt_twice());
    compose_complete(bytes_order_splt_twice(),h_order_splt_twice(),z_order_splt_twice(),0nat,colour_order_splt_twice(),expected_order_splt_twice());
}
pub fn public_complete_order_splt_twice() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_order_splt_twice(),false),result),se::eq(bytes_order_splt_twice(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_order_splt_twice(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,3u8,115u8,80u8,76u8,84u8,97u8,0u8,8u8,210u8,192u8,25u8,167u8,0u8,0u8,0u8,3u8,115u8,80u8,76u8,84u8,98u8,0u8,8u8,208u8,134u8,167u8,254u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_order_splt_twice());model_complete_order_splt_twice();}
    crate::png::try_decode_still(&b)
}
// order/bkgd-hist-after-plte-before-idat-rgb; frozen fixture SHA256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
pub open spec fn bytes_order_bkgd_hist_after_plte_before_idat_rgb() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,9u8,80u8,76u8,84u8,69u8,47u8,194u8,40u8,139u8,44u8,88u8,228u8,36u8,202u8,64u8,183u8,32u8,36u8,0u8,0u8,0u8,6u8,104u8,73u8,83u8,84u8,0u8,0u8,0u8,0u8,0u8,0u8,95u8,184u8,160u8,47u8,0u8,0u8,0u8,6u8,98u8,75u8,71u8,68u8,0u8,0u8,0u8,0u8,0u8,0u8,249u8,67u8,187u8,127u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn h_order_bkgd_hist_after_plte_before_idat_rgb() -> crate::model::Ihdr {crate::model::Ihdr {width:2u32,height:2u32,depth:8u8,color_type:2u8,interlace:0u8}}
spec fn a_order_bkgd_hist_after_plte_before_idat_rgb() -> r::Raster {r::shape(h_order_bkgd_hist_after_plte_before_idat_rgb(),2nat,2nat,3nat,STRICT)}
spec fn colour_order_bkgd_hist_after_plte_before_idat_rgb() -> crate::colour_model::Context {crate::colour_model::Context {ct:2u8,depth:8u8,plte:seq![47u8,194u8,40u8,139u8,44u8,88u8,228u8,36u8,202u8],trns:None}}
pub open spec fn expected_order_bkgd_hist_after_plte_before_idat_rgb() -> Seq<u8> {seq![201u8,73u8,63u8,255u8,202u8,75u8,151u8,255u8,225u8,26u8,23u8,255u8,252u8,246u8,129u8,255u8]}
spec fn z_order_bkgd_hist_after_plte_before_idat_rgb() -> Seq<u8> {seq![120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8]}
spec fn raw_order_bkgd_hist_after_plte_before_idat_rgb() -> Seq<u8> {seq![0u8,201u8,73u8,63u8,202u8,75u8,151u8,1u8,225u8,26u8,23u8,27u8,220u8,106u8]}
spec fn ts_order_bkgd_hist_after_plte_before_idat_rgb() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:36nat,before:1nat,after:2nat,symbol:201nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:36nat,next:44nat,before:2nat,after:3nat,symbol:73nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:44nat,next:52nat,before:3nat,after:4nat,symbol:63nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:52nat,next:61nat,before:4nat,after:5nat,symbol:202nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:61nat,next:69nat,before:5nat,after:6nat,symbol:75nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:69nat,next:78nat,before:6nat,after:7nat,symbol:151nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:78nat,next:86nat,before:7nat,after:8nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:86nat,next:95nat,before:8nat,after:9nat,symbol:225nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:95nat,next:103nat,before:9nat,after:10nat,symbol:26nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:103nat,next:111nat,before:10nat,after:11nat,symbol:23nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:111nat,next:119nat,before:11nat,after:12nat,symbol:27nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:119nat,next:128nat,before:12nat,after:13nat,symbol:220nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:128nat,next:136nat,before:13nat,after:14nat,symbol:106nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:136nat,next:143nat,before:14nat,after:14nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_order_bkgd_hist_after_plte_before_idat_rgb_0() ensures t::trace(z_order_bkgd_hist_after_plte_before_idat_rgb(),raw_order_bkgd_hist_after_plte_before_idat_rgb(),ts_order_bkgd_hist_after_plte_before_idat_rgb(),0nat,15nat),
{
    hide(t::valid_step);assert(t::trace(z_order_bkgd_hist_after_plte_before_idat_rgb(),raw_order_bkgd_hist_after_plte_before_idat_rgb(),ts_order_bkgd_hist_after_plte_before_idat_rgb(),0nat,15nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn trace_order_bkgd_hist_after_plte_before_idat_rgb() ensures t::trace(z_order_bkgd_hist_after_plte_before_idat_rgb(),raw_order_bkgd_hist_after_plte_before_idat_rgb(),ts_order_bkgd_hist_after_plte_before_idat_rgb(),0,15nat),
{
    hide(z_order_bkgd_hist_after_plte_before_idat_rgb);hide(raw_order_bkgd_hist_after_plte_before_idat_rgb);hide(ts_order_bkgd_hist_after_plte_before_idat_rgb);hide(t::trace);
    trace_order_bkgd_hist_after_plte_before_idat_rgb_0();
    assert(t::trace(z_order_bkgd_hist_after_plte_before_idat_rgb(),raw_order_bkgd_hist_after_plte_before_idat_rgb(),ts_order_bkgd_hist_after_plte_before_idat_rgb(),0,15nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_order_bkgd_hist_after_plte_before_idat_rgb() ensures z_order_bkgd_hist_after_plte_before_idat_rgb().len()==22nat,
        raw_order_bkgd_hist_after_plte_before_idat_rgb().len()==14nat,
        z_order_bkgd_hist_after_plte_before_idat_rgb()[0]==120u8,
        z_order_bkgd_hist_after_plte_before_idat_rgb()[1]==156u8,
        zm::field_value(z_order_bkgd_hist_after_plte_before_idat_rgb(),16,1)==1nat,
        zm::field_value(z_order_bkgd_hist_after_plte_before_idat_rgb(),17,2)==1nat,
        zm::be_value(z_order_bkgd_hist_after_plte_before_idat_rgb(),18nat,4)==675349874nat,
        ts_order_bkgd_hist_after_plte_before_idat_rgb().len()==15nat,
        ts_order_bkgd_hist_after_plte_before_idat_rgb()[0].pos==19nat,
        ts_order_bkgd_hist_after_plte_before_idat_rgb()[0].before==0nat,
        ts_order_bkgd_hist_after_plte_before_idat_rgb().last().next==143nat,
{
    assert(z_order_bkgd_hist_after_plte_before_idat_rgb().len()==22nat) by (compute);
    assert(raw_order_bkgd_hist_after_plte_before_idat_rgb().len()==14nat) by (compute);
    assert(z_order_bkgd_hist_after_plte_before_idat_rgb()[0]==120u8) by (compute);
    assert(z_order_bkgd_hist_after_plte_before_idat_rgb()[1]==156u8) by (compute);
    assert(zm::field_value(z_order_bkgd_hist_after_plte_before_idat_rgb(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_order_bkgd_hist_after_plte_before_idat_rgb(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_order_bkgd_hist_after_plte_before_idat_rgb(),18nat,4)==675349874nat) by (compute);
    assert(ts_order_bkgd_hist_after_plte_before_idat_rgb().len()==15nat) by (compute);
    assert(ts_order_bkgd_hist_after_plte_before_idat_rgb()[0].pos==19nat) by (compute);
    assert(ts_order_bkgd_hist_after_plte_before_idat_rgb()[0].before==0nat) by (compute);
    assert(ts_order_bkgd_hist_after_plte_before_idat_rgb().last().next==143nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_order_bkgd_hist_after_plte_before_idat_rgb() ensures zm::Adler(raw_order_bkgd_hist_after_plte_before_idat_rgb())==675349874nat,
{
    hide(raw_order_bkgd_hist_after_plte_before_idat_rgb);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_order_bkgd_hist_after_plte_before_idat_rgb().len()==14nat) by (compute);
    assert(c::sums(raw_order_bkgd_hist_after_plte_before_idat_rgb(),0,14nat)==(1393nat,10305nat)) by (compute);
    c::sums_correct(raw_order_bkgd_hist_after_plte_before_idat_rgb(),0,14nat);assert(raw_order_bkgd_hist_after_plte_before_idat_rgb().subrange(0,14)=~=raw_order_bkgd_hist_after_plte_before_idat_rgb());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((10305nat%65521)*(u16::MAX as nat+1)+(1+1393nat)%65521==675349874nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_order_bkgd_hist_after_plte_before_idat_rgb() ensures zm::Inflate(z_order_bkgd_hist_after_plte_before_idat_rgb(),14nat,true,false)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_order_bkgd_hist_after_plte_before_idat_rgb(),r2:false}),raw_order_bkgd_hist_after_plte_before_idat_rgb().len()==14nat,
{
    hide(z_order_bkgd_hist_after_plte_before_idat_rgb);hide(raw_order_bkgd_hist_after_plte_before_idat_rgb);hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);hide(ts_order_bkgd_hist_after_plte_before_idat_rgb);
    shape_order_bkgd_hist_after_plte_before_idat_rgb();checksum_order_bkgd_hist_after_plte_before_idat_rgb();
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_order_bkgd_hist_after_plte_before_idat_rgb();ms::fixed_stream(z_order_bkgd_hist_after_plte_before_idat_rgb(),raw_order_bkgd_hist_after_plte_before_idat_rgb(),ts_order_bkgd_hist_after_plte_before_idat_rgb(),14nat,156nat,675349874nat,675349874nat);
}
#[verifier::spinoff_prover]
spec fn e6_bkgd_hist_state_0(b: Seq<u8>) -> m::State {m::initial(h_order_bkgd_hist_after_plte_before_idat_rgb(),33,m::no_events())}
spec fn e6_bkgd_hist_state_1(b: Seq<u8>) -> m::State {let s=e6_bkgd_hist_state_0(b);m::State {pos:54nat,seen:s.seen.insert(crate::walk_model::Name::Plte),doc:m::Doc {plte:Some(crate::walk_model::Range {offset:41nat,len:9nat}),info:m::Info {palette_entries:3nat,..s.doc.info},..s.doc},..s}}
spec fn e6_bkgd_hist_state_2(b: Seq<u8>) -> m::State {let s=e6_bkgd_hist_state_1(b);m::State {pos:72nat,seen:s.seen.insert(crate::walk_model::Name::Hist),doc:m::store(s.doc,crate::metadata_model::Value::Ignored,62nat),..s}}
spec fn e6_bkgd_hist_state_3(b: Seq<u8>) -> m::State {let s=e6_bkgd_hist_state_2(b);m::State {pos:90nat,seen:s.seen.insert(crate::walk_model::Name::Bkgd),doc:m::store(s.doc,crate::metadata_model::Value::Bkgd(crate::metadata_model::words16(b.subrange(80,86),3)),80nat),..s}}
spec fn e6_bkgd_hist_state_4(b: Seq<u8>) -> m::State {let s=e6_bkgd_hist_state_3(b);m::State {pos:124nat,phase:crate::walk_model::Phase::During,doc:m::Doc {idat:seq![crate::walk_model::Range {offset:98nat,len:22nat}],..s.doc},..s}}
#[verifier::spinoff_prover]
proof fn e6_bkgd_hist_step_0(b: Seq<u8>)
    requires b.len()==136,
        crate::model::mh(b)==Ok((h_order_bkgd_hist_after_plte_before_idat_rgb(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:9nat,next:54nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,54nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![104u8,73u8,83u8,84u8],data:62nat,len:6nat,next:72nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,72nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![98u8,75u8,71u8,68u8],data:80nat,len:6nat,next:90nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,90nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:98nat,len:22nat,next:124nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,124nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:132nat,len:0nat,next:136nat,crc_ok:true,synthetic:false})),
        crate::metadata_model::parse(crate::walk_model::Name::Hist,b.subrange(62,68),2u8,8u8,3nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored),
        crate::metadata_model::parse(crate::walk_model::Name::Bkgd,b.subrange(80,86),2u8,8u8,3nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Bkgd(crate::metadata_model::words16(b.subrange(80,86),3))),
        b.subrange(98,120)==z_order_bkgd_hist_after_plte_before_idat_rgb(),
    ensures m::step(b,m::strict(),false,false,e6_bkgd_hist_state_0(b))==m::Step::Next(e6_bkgd_hist_state_1(b)),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(m::store);hide(crate::metadata_model::parse);
    assert forall|doc: m::Doc,value: crate::metadata_model::Value,base: nat|
        (#[trigger] m::store(doc,value,base)).ihdr==doc.ihdr && m::store(doc,value,base).plte==doc.plte
        && m::store(doc,value,base).trns==doc.trns && m::store(doc,value,base).actl==doc.actl
        && m::store(doc,value,base).idat==doc.idat && m::store(doc,value,base).frames==doc.frames by {
        stored_structure(doc,value,base);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(9nat,2u8,8u8,false,false,false,false)==crate::metadata_model::Decision::Keep(3nat)) by (compute);
        assert((104u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![104u8,73u8,83u8,84u8])==crate::walk_model::Name::Hist) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![104u8,73u8,83u8,84u8])==crate::walk_model::Name::Hist) by (compute);
        }
        assert((98u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![98u8,75u8,71u8,68u8])==crate::walk_model::Name::Bkgd) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![98u8,75u8,71u8,68u8])==crate::walk_model::Name::Bkgd) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
}
#[verifier::spinoff_prover]
proof fn e6_bkgd_hist_step_1(b: Seq<u8>)
    requires b.len()==136,
        crate::model::mh(b)==Ok((h_order_bkgd_hist_after_plte_before_idat_rgb(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:9nat,next:54nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,54nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![104u8,73u8,83u8,84u8],data:62nat,len:6nat,next:72nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,72nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![98u8,75u8,71u8,68u8],data:80nat,len:6nat,next:90nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,90nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:98nat,len:22nat,next:124nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,124nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:132nat,len:0nat,next:136nat,crc_ok:true,synthetic:false})),
        crate::metadata_model::parse(crate::walk_model::Name::Hist,b.subrange(62,68),2u8,8u8,3nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored),
        crate::metadata_model::parse(crate::walk_model::Name::Bkgd,b.subrange(80,86),2u8,8u8,3nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Bkgd(crate::metadata_model::words16(b.subrange(80,86),3))),
        b.subrange(98,120)==z_order_bkgd_hist_after_plte_before_idat_rgb(),
    ensures m::step(b,m::strict(),false,false,e6_bkgd_hist_state_1(b))==m::Step::Next(e6_bkgd_hist_state_2(b)),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(m::store);hide(crate::metadata_model::parse);
    assert forall|doc: m::Doc,value: crate::metadata_model::Value,base: nat|
        (#[trigger] m::store(doc,value,base)).ihdr==doc.ihdr && m::store(doc,value,base).plte==doc.plte
        && m::store(doc,value,base).trns==doc.trns && m::store(doc,value,base).actl==doc.actl
        && m::store(doc,value,base).idat==doc.idat && m::store(doc,value,base).frames==doc.frames by {
        stored_structure(doc,value,base);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(9nat,2u8,8u8,false,false,false,false)==crate::metadata_model::Decision::Keep(3nat)) by (compute);
        assert((104u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![104u8,73u8,83u8,84u8])==crate::walk_model::Name::Hist) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![104u8,73u8,83u8,84u8])==crate::walk_model::Name::Hist) by (compute);
        }
        assert((98u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![98u8,75u8,71u8,68u8])==crate::walk_model::Name::Bkgd) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![98u8,75u8,71u8,68u8])==crate::walk_model::Name::Bkgd) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
}
#[verifier::spinoff_prover]
proof fn e6_bkgd_hist_step_2(b: Seq<u8>)
    requires b.len()==136,
        crate::model::mh(b)==Ok((h_order_bkgd_hist_after_plte_before_idat_rgb(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:9nat,next:54nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,54nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![104u8,73u8,83u8,84u8],data:62nat,len:6nat,next:72nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,72nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![98u8,75u8,71u8,68u8],data:80nat,len:6nat,next:90nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,90nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:98nat,len:22nat,next:124nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,124nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:132nat,len:0nat,next:136nat,crc_ok:true,synthetic:false})),
        crate::metadata_model::parse(crate::walk_model::Name::Hist,b.subrange(62,68),2u8,8u8,3nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored),
        crate::metadata_model::parse(crate::walk_model::Name::Bkgd,b.subrange(80,86),2u8,8u8,3nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Bkgd(crate::metadata_model::words16(b.subrange(80,86),3))),
        b.subrange(98,120)==z_order_bkgd_hist_after_plte_before_idat_rgb(),
    ensures m::step(b,m::strict(),false,false,e6_bkgd_hist_state_2(b))==m::Step::Next(e6_bkgd_hist_state_3(b)),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(m::store);hide(crate::metadata_model::parse);
    assert forall|doc: m::Doc,value: crate::metadata_model::Value,base: nat|
        (#[trigger] m::store(doc,value,base)).ihdr==doc.ihdr && m::store(doc,value,base).plte==doc.plte
        && m::store(doc,value,base).trns==doc.trns && m::store(doc,value,base).actl==doc.actl
        && m::store(doc,value,base).idat==doc.idat && m::store(doc,value,base).frames==doc.frames by {
        stored_structure(doc,value,base);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(9nat,2u8,8u8,false,false,false,false)==crate::metadata_model::Decision::Keep(3nat)) by (compute);
        assert((104u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![104u8,73u8,83u8,84u8])==crate::walk_model::Name::Hist) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![104u8,73u8,83u8,84u8])==crate::walk_model::Name::Hist) by (compute);
        }
        assert((98u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![98u8,75u8,71u8,68u8])==crate::walk_model::Name::Bkgd) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![98u8,75u8,71u8,68u8])==crate::walk_model::Name::Bkgd) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
}
#[verifier::spinoff_prover]
proof fn e6_bkgd_hist_step_3(b: Seq<u8>)
    requires b.len()==136,
        crate::model::mh(b)==Ok((h_order_bkgd_hist_after_plte_before_idat_rgb(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:9nat,next:54nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,54nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![104u8,73u8,83u8,84u8],data:62nat,len:6nat,next:72nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,72nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![98u8,75u8,71u8,68u8],data:80nat,len:6nat,next:90nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,90nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:98nat,len:22nat,next:124nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,124nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:132nat,len:0nat,next:136nat,crc_ok:true,synthetic:false})),
        crate::metadata_model::parse(crate::walk_model::Name::Hist,b.subrange(62,68),2u8,8u8,3nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored),
        crate::metadata_model::parse(crate::walk_model::Name::Bkgd,b.subrange(80,86),2u8,8u8,3nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Bkgd(crate::metadata_model::words16(b.subrange(80,86),3))),
        b.subrange(98,120)==z_order_bkgd_hist_after_plte_before_idat_rgb(),
    ensures m::step(b,m::strict(),false,false,e6_bkgd_hist_state_3(b))==m::Step::Next(e6_bkgd_hist_state_4(b)),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(m::store);hide(crate::metadata_model::parse);
    assert forall|doc: m::Doc,value: crate::metadata_model::Value,base: nat|
        (#[trigger] m::store(doc,value,base)).ihdr==doc.ihdr && m::store(doc,value,base).plte==doc.plte
        && m::store(doc,value,base).trns==doc.trns && m::store(doc,value,base).actl==doc.actl
        && m::store(doc,value,base).idat==doc.idat && m::store(doc,value,base).frames==doc.frames by {
        stored_structure(doc,value,base);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(9nat,2u8,8u8,false,false,false,false)==crate::metadata_model::Decision::Keep(3nat)) by (compute);
        assert((104u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![104u8,73u8,83u8,84u8])==crate::walk_model::Name::Hist) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![104u8,73u8,83u8,84u8])==crate::walk_model::Name::Hist) by (compute);
        }
        assert((98u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![98u8,75u8,71u8,68u8])==crate::walk_model::Name::Bkgd) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![98u8,75u8,71u8,68u8])==crate::walk_model::Name::Bkgd) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
}
#[verifier::spinoff_prover]
proof fn e6_bkgd_hist_step_4(b: Seq<u8>)
    requires b.len()==136,
        crate::model::mh(b)==Ok((h_order_bkgd_hist_after_plte_before_idat_rgb(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:9nat,next:54nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,54nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![104u8,73u8,83u8,84u8],data:62nat,len:6nat,next:72nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,72nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![98u8,75u8,71u8,68u8],data:80nat,len:6nat,next:90nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,90nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:98nat,len:22nat,next:124nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,124nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:132nat,len:0nat,next:136nat,crc_ok:true,synthetic:false})),
        crate::metadata_model::parse(crate::walk_model::Name::Hist,b.subrange(62,68),2u8,8u8,3nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored),
        crate::metadata_model::parse(crate::walk_model::Name::Bkgd,b.subrange(80,86),2u8,8u8,3nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Bkgd(crate::metadata_model::words16(b.subrange(80,86),3))),
        b.subrange(98,120)==z_order_bkgd_hist_after_plte_before_idat_rgb(),
    ensures m::step(b,m::strict(),false,false,e6_bkgd_hist_state_4(b))==m::Step::Done(m::Doc {info:m::Info {frame_count:0nat,..e6_bkgd_hist_state_4(b).doc.info},..e6_bkgd_hist_state_4(b).doc},m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(m::store);hide(crate::metadata_model::parse);
    assert forall|doc: m::Doc,value: crate::metadata_model::Value,base: nat|
        (#[trigger] m::store(doc,value,base)).ihdr==doc.ihdr && m::store(doc,value,base).plte==doc.plte
        && m::store(doc,value,base).trns==doc.trns && m::store(doc,value,base).actl==doc.actl
        && m::store(doc,value,base).idat==doc.idat && m::store(doc,value,base).frames==doc.frames by {
        stored_structure(doc,value,base);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(9nat,2u8,8u8,false,false,false,false)==crate::metadata_model::Decision::Keep(3nat)) by (compute);
        assert((104u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![104u8,73u8,83u8,84u8])==crate::walk_model::Name::Hist) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![104u8,73u8,83u8,84u8])==crate::walk_model::Name::Hist) by (compute);
        }
        assert((98u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![98u8,75u8,71u8,68u8])==crate::walk_model::Name::Bkgd) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![98u8,75u8,71u8,68u8])==crate::walk_model::Name::Bkgd) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
}
#[verifier::spinoff_prover]
proof fn walk_shape_order_bkgd_hist_after_plte_before_idat_rgb(b: Seq<u8>)
    requires b.len()==136,
        crate::model::mh(b)==Ok((h_order_bkgd_hist_after_plte_before_idat_rgb(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:9nat,next:54nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,54nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![104u8,73u8,83u8,84u8],data:62nat,len:6nat,next:72nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,72nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![98u8,75u8,71u8,68u8],data:80nat,len:6nat,next:90nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,90nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:98nat,len:22nat,next:124nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,124nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:132nat,len:0nat,next:136nat,crc_ok:true,synthetic:false})),
        crate::metadata_model::parse(crate::walk_model::Name::Hist,b.subrange(62,68),2u8,8u8,3nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored),
        crate::metadata_model::parse(crate::walk_model::Name::Bkgd,b.subrange(80,86),2u8,8u8,3nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Bkgd(crate::metadata_model::words16(b.subrange(80,86),3))),
        b.subrange(98,120)==z_order_bkgd_hist_after_plte_before_idat_rgb(),
    ensures r::default_case(b,h_order_bkgd_hist_after_plte_before_idat_rgb(),z_order_bkgd_hist_after_plte_before_idat_rgb(),3nat),
{
    hide(m::drive);hide(crate::critical_model::read);hide(crate::metadata_model::parse);hide(crate::model::mh);
    e6_bkgd_hist_step_0(b);
    e6_bkgd_hist_step_1(b);
    e6_bkgd_hist_step_2(b);
    e6_bkgd_hist_step_3(b);
    e6_bkgd_hist_step_4(b);
    m::drive_step(b,m::strict(),false,false,e6_bkgd_hist_state_4(b));
    m::drive_step(b,m::strict(),false,false,e6_bkgd_hist_state_3(b));
    m::drive_step(b,m::strict(),false,false,e6_bkgd_hist_state_2(b));
    m::drive_step(b,m::strict(),false,false,e6_bkgd_hist_state_1(b));
    m::drive_step(b,m::strict(),false,false,e6_bkgd_hist_state_0(b));
    reveal_with_fuel(m::concat_ranges,3);
}
#[verifier::spinoff_prover]
proof fn walk_ranges_order_bkgd_hist_after_plte_before_idat_rgb(b: Seq<u8>)
    requires b.len()==136,
        crate::model::mh(b)==Ok((h_order_bkgd_hist_after_plte_before_idat_rgb(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:9nat,next:54nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,54nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![104u8,73u8,83u8,84u8],data:62nat,len:6nat,next:72nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,72nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![98u8,75u8,71u8,68u8],data:80nat,len:6nat,next:90nat,crc_ok:true,synthetic:false})),
        b.subrange(80,86)[0]==0u8,
        b.subrange(80,86)[1]==0u8,
        b.subrange(80,86)[2]==0u8,
        b.subrange(80,86)[3]==0u8,
        b.subrange(80,86)[4]==0u8,
        b.subrange(80,86)[5]==0u8,
        crate::metadata_model::words16(b.subrange(80,86),3nat).len()==3,
        crate::model::be16(b.subrange(80,86),0)==0u16,
        crate::metadata_model::words16(b.subrange(80,86),3nat)[0]==0u16,
        crate::model::be16(b.subrange(80,86),2)==0u16,
        crate::metadata_model::words16(b.subrange(80,86),3nat)[1]==0u16,
        crate::model::be16(b.subrange(80,86),4)==0u16,
        crate::metadata_model::words16(b.subrange(80,86),3nat)[2]==0u16,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,90nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:98nat,len:22nat,next:124nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,124nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:132nat,len:0nat,next:136nat,crc_ok:true,synthetic:false})),
        crate::metadata_model::parse(crate::walk_model::Name::Hist,b.subrange(62,68),2u8,8u8,3nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored),
        (forall|i: int| 0<=i<3 ==> (#[trigger] crate::metadata_model::words16(b.subrange(80,86),3)[i] as nat)<(1u32<<8u8) as nat),
        crate::metadata_model::parse(crate::walk_model::Name::Bkgd,b.subrange(80,86),2u8,8u8,3nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Bkgd(crate::metadata_model::words16(b.subrange(80,86),3))),
        b.subrange(98,120)==z_order_bkgd_hist_after_plte_before_idat_rgb(),
    ensures match m::MW_p(b,m::strict()) {m::Outcome::Done(doc,_)=>doc.plte==Some(crate::walk_model::Range {offset:41nat,len:9nat}) && doc.trns==None,_=>false},
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(m::store);hide(crate::metadata_model::parse);
    assert forall|doc: m::Doc,value: crate::metadata_model::Value,base: nat|
        (#[trigger] m::store(doc,value,base)).ihdr==doc.ihdr && m::store(doc,value,base).plte==doc.plte
        && m::store(doc,value,base).trns==doc.trns && m::store(doc,value,base).actl==doc.actl
        && m::store(doc,value,base).idat==doc.idat && m::store(doc,value,base).frames==doc.frames by {
        stored_structure(doc,value,base);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(9nat,2u8,8u8,false,false,false,false)==crate::metadata_model::Decision::Keep(3nat)) by (compute);
        assert((104u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![104u8,73u8,83u8,84u8])==crate::walk_model::Name::Hist) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![104u8,73u8,83u8,84u8])==crate::walk_model::Name::Hist) by (compute);
        }
        assert((98u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![98u8,75u8,71u8,68u8])==crate::walk_model::Name::Bkgd) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![98u8,75u8,71u8,68u8])==crate::walk_model::Name::Bkgd) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    reveal_with_fuel(m::drive,6);
    reveal_with_fuel(m::concat_ranges,3);
}
#[verifier::spinoff_prover]
proof fn walk_order_bkgd_hist_after_plte_before_idat_rgb(b: Seq<u8>)
    requires b.len()==136,
        crate::model::mh(b)==Ok((h_order_bkgd_hist_after_plte_before_idat_rgb(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:9nat,next:54nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,54nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![104u8,73u8,83u8,84u8],data:62nat,len:6nat,next:72nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,72nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![98u8,75u8,71u8,68u8],data:80nat,len:6nat,next:90nat,crc_ok:true,synthetic:false})),
        b.subrange(80,86)[0]==0u8,
        b.subrange(80,86)[1]==0u8,
        b.subrange(80,86)[2]==0u8,
        b.subrange(80,86)[3]==0u8,
        b.subrange(80,86)[4]==0u8,
        b.subrange(80,86)[5]==0u8,
        crate::metadata_model::words16(b.subrange(80,86),3nat).len()==3,
        crate::model::be16(b.subrange(80,86),0)==0u16,
        crate::metadata_model::words16(b.subrange(80,86),3nat)[0]==0u16,
        crate::model::be16(b.subrange(80,86),2)==0u16,
        crate::metadata_model::words16(b.subrange(80,86),3nat)[1]==0u16,
        crate::model::be16(b.subrange(80,86),4)==0u16,
        crate::metadata_model::words16(b.subrange(80,86),3nat)[2]==0u16,
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,90nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:98nat,len:22nat,next:124nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,124nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:132nat,len:0nat,next:136nat,crc_ok:true,synthetic:false})),
        crate::metadata_model::parse(crate::walk_model::Name::Hist,b.subrange(62,68),2u8,8u8,3nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored),
        (forall|i: int| 0<=i<3 ==> (#[trigger] crate::metadata_model::words16(b.subrange(80,86),3)[i] as nat)<(1u32<<8u8) as nat),
        crate::metadata_model::parse(crate::walk_model::Name::Bkgd,b.subrange(80,86),2u8,8u8,3nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Bkgd(crate::metadata_model::words16(b.subrange(80,86),3))),
        b.subrange(98,120)==z_order_bkgd_hist_after_plte_before_idat_rgb(),
    ensures r::default_case(b,h_order_bkgd_hist_after_plte_before_idat_rgb(),z_order_bkgd_hist_after_plte_before_idat_rgb(),3nat),match m::MW_p(b,m::strict()) {m::Outcome::Done(doc,_)=>doc.plte==Some(crate::walk_model::Range {offset:41nat,len:9nat}) && doc.trns==None,_=>false},
{
    walk_shape_order_bkgd_hist_after_plte_before_idat_rgb(b);walk_ranges_order_bkgd_hist_after_plte_before_idat_rgb(b);
}
#[verifier::spinoff_prover]
proof fn scan_order_bkgd_hist_after_plte_before_idat_rgb() ensures r::scan(raw_order_bkgd_hist_after_plte_before_idat_rgb(),a_order_bkgd_hist_after_plte_before_idat_rgb(),0)==Ok(()),
{
    hide(raw_order_bkgd_hist_after_plte_before_idat_rgb);assert(raw_order_bkgd_hist_after_plte_before_idat_rgb().len()==14nat) by (compute);assert(r::count(a_order_bkgd_hist_after_plte_before_idat_rgb())==1nat) by (compute);
    assert(r::pw(a_order_bkgd_hist_after_plte_before_idat_rgb(),0nat)==2nat && r::ph(a_order_bkgd_hist_after_plte_before_idat_rgb(),0nat)==2nat && r::rb(a_order_bkgd_hist_after_plte_before_idat_rgb(),0nat)==6nat && r::base(a_order_bkgd_hist_after_plte_before_idat_rgb(),0nat)==0nat) by (compute);
    assert(raw_order_bkgd_hist_after_plte_before_idat_rgb()[0]==0u8) by (compute);
    assert(r::row_result(raw_order_bkgd_hist_after_plte_before_idat_rgb(),a_order_bkgd_hist_after_plte_before_idat_rgb(),0nat,0nat)==Ok(()));
    assert(raw_order_bkgd_hist_after_plte_before_idat_rgb()[7]==1u8) by (compute);
    assert(r::row_result(raw_order_bkgd_hist_after_plte_before_idat_rgb(),a_order_bkgd_hist_after_plte_before_idat_rgb(),0nat,1nat)==Ok(()));
    r::scan_step(raw_order_bkgd_hist_after_plte_before_idat_rgb(),a_order_bkgd_hist_after_plte_before_idat_rgb(),1nat);
    r::pass_step(raw_order_bkgd_hist_after_plte_before_idat_rgb(),a_order_bkgd_hist_after_plte_before_idat_rgb(),0nat,2nat);
    r::pass_step(raw_order_bkgd_hist_after_plte_before_idat_rgb(),a_order_bkgd_hist_after_plte_before_idat_rgb(),0nat,1nat);
    r::pass_step(raw_order_bkgd_hist_after_plte_before_idat_rgb(),a_order_bkgd_hist_after_plte_before_idat_rgb(),0nat,0nat);
    r::scan_step(raw_order_bkgd_hist_after_plte_before_idat_rgb(),a_order_bkgd_hist_after_plte_before_idat_rgb(),0nat);
}
#[verifier::spinoff_prover]
proof fn row_order_bkgd_hist_after_plte_before_idat_rgb_0_0() ensures r::line(raw_order_bkgd_hist_after_plte_before_idat_rgb(),a_order_bkgd_hist_after_plte_before_idat_rgb(),0nat,0nat)==seq![201u8,73u8,63u8,202u8,75u8,151u8],
{
    hide(raw_order_bkgd_hist_after_plte_before_idat_rgb);
    assert(raw_order_bkgd_hist_after_plte_before_idat_rgb().len()==14nat) by (compute);
    assert(raw_order_bkgd_hist_after_plte_before_idat_rgb()[0]==0u8) by (compute);
    assert(r::base(a_order_bkgd_hist_after_plte_before_idat_rgb(),0nat)==0nat && r::rb(a_order_bkgd_hist_after_plte_before_idat_rgb(),0nat)==6nat && f::bpp(r::bits(a_order_bkgd_hist_after_plte_before_idat_rgb()))==3nat) by (compute);
    assert(f::row_data(raw_order_bkgd_hist_after_plte_before_idat_rgb(),0nat,6nat,0nat)==seq![201u8,73u8,63u8,202u8,75u8,151u8]) by (compute);
    f::pass_row_equation(raw_order_bkgd_hist_after_plte_before_idat_rgb(),0nat,6nat,3nat,0nat);
    assert(f::previous_row(raw_order_bkgd_hist_after_plte_before_idat_rgb(),0nat,6nat,3nat,0nat)==None);
    assert(f::encoded_row(0u8,f::row_data(raw_order_bkgd_hist_after_plte_before_idat_rgb(),0nat,6nat,0nat),None,3nat,seq![201u8,73u8,63u8,202u8,75u8,151u8],0,6nat)) by (compute);
    f::checked_row_hint(0u8,f::row_data(raw_order_bkgd_hist_after_plte_before_idat_rgb(),0nat,6nat,0nat),None,3nat,seq![201u8,73u8,63u8,202u8,75u8,151u8]);
}
#[verifier::spinoff_prover]
proof fn row_order_bkgd_hist_after_plte_before_idat_rgb_0_1() ensures r::line(raw_order_bkgd_hist_after_plte_before_idat_rgb(),a_order_bkgd_hist_after_plte_before_idat_rgb(),0nat,1nat)==seq![225u8,26u8,23u8,252u8,246u8,129u8],
{
    hide(raw_order_bkgd_hist_after_plte_before_idat_rgb);
    assert(raw_order_bkgd_hist_after_plte_before_idat_rgb().len()==14nat) by (compute);
    assert(raw_order_bkgd_hist_after_plte_before_idat_rgb()[7]==1u8) by (compute);
    assert(r::base(a_order_bkgd_hist_after_plte_before_idat_rgb(),0nat)==0nat && r::rb(a_order_bkgd_hist_after_plte_before_idat_rgb(),0nat)==6nat && f::bpp(r::bits(a_order_bkgd_hist_after_plte_before_idat_rgb()))==3nat) by (compute);
    assert(f::row_data(raw_order_bkgd_hist_after_plte_before_idat_rgb(),0nat,6nat,1nat)==seq![225u8,26u8,23u8,27u8,220u8,106u8]) by (compute);
    row_order_bkgd_hist_after_plte_before_idat_rgb_0_0();
    f::pass_row_equation(raw_order_bkgd_hist_after_plte_before_idat_rgb(),0nat,6nat,3nat,1nat);
    assert(f::previous_row(raw_order_bkgd_hist_after_plte_before_idat_rgb(),0nat,6nat,3nat,1nat)==Some(seq![201u8,73u8,63u8,202u8,75u8,151u8]));
    assert(f::encoded_row(1u8,f::row_data(raw_order_bkgd_hist_after_plte_before_idat_rgb(),0nat,6nat,1nat),Some(seq![201u8,73u8,63u8,202u8,75u8,151u8]),3nat,seq![225u8,26u8,23u8,252u8,246u8,129u8],0,6nat)) by (compute);
    f::checked_row_hint(1u8,f::row_data(raw_order_bkgd_hist_after_plte_before_idat_rgb(),0nat,6nat,1nat),Some(seq![201u8,73u8,63u8,202u8,75u8,151u8]),3nat,seq![225u8,26u8,23u8,252u8,246u8,129u8]);
}
#[verifier::spinoff_prover]
proof fn raster_order_bkgd_hist_after_plte_before_idat_rgb()
    ensures r::default_case(bytes_order_bkgd_hist_after_plte_before_idat_rgb(),h_order_bkgd_hist_after_plte_before_idat_rgb(),z_order_bkgd_hist_after_plte_before_idat_rgb(),3nat),match m::MW_p(bytes_order_bkgd_hist_after_plte_before_idat_rgb(),m::strict()) {m::Outcome::Done(doc,_)=>doc.plte==Some(crate::walk_model::Range {offset:41nat,len:9nat}) && doc.trns==None,_=>false},r::data(z_order_bkgd_hist_after_plte_before_idat_rgb(),a_order_bkgd_hist_after_plte_before_idat_rgb(),STRICT)==Some(Ok(())),
{
    hide(bytes_order_bkgd_hist_after_plte_before_idat_rgb);hide(z_order_bkgd_hist_after_plte_before_idat_rgb);hide(raw_order_bkgd_hist_after_plte_before_idat_rgb);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    assert(bytes_order_bkgd_hist_after_plte_before_idat_rgb().len()==136nat) by (compute);
    assert(crate::model::mh(bytes_order_bkgd_hist_after_plte_before_idat_rgb())==Ok((h_order_bkgd_hist_after_plte_before_idat_rgb(),33nat))) by {reveal(crate::model::mh);assert(crate::model::mh(bytes_order_bkgd_hist_after_plte_before_idat_rgb())==Ok((h_order_bkgd_hist_after_plte_before_idat_rgb(),33nat))) by (compute);}
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_order_bkgd_hist_after_plte_before_idat_rgb(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:9nat,next:54nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_order_bkgd_hist_after_plte_before_idat_rgb(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:9nat,next:54nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(9nat,2u8,8u8,false,false,false,false)==crate::metadata_model::Decision::Keep(3nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_order_bkgd_hist_after_plte_before_idat_rgb(),54nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![104u8,73u8,83u8,84u8],data:62nat,len:6nat,next:72nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_order_bkgd_hist_after_plte_before_idat_rgb(),54nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![104u8,73u8,83u8,84u8],data:62nat,len:6nat,next:72nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((104u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![104u8,73u8,83u8,84u8])==crate::walk_model::Name::Hist) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![104u8,73u8,83u8,84u8])==crate::walk_model::Name::Hist) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_order_bkgd_hist_after_plte_before_idat_rgb(),72nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![98u8,75u8,71u8,68u8],data:80nat,len:6nat,next:90nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_order_bkgd_hist_after_plte_before_idat_rgb(),72nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![98u8,75u8,71u8,68u8],data:80nat,len:6nat,next:90nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((98u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![98u8,75u8,71u8,68u8])==crate::walk_model::Name::Bkgd) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![98u8,75u8,71u8,68u8])==crate::walk_model::Name::Bkgd) by (compute);
        }
        assert(bytes_order_bkgd_hist_after_plte_before_idat_rgb().subrange(80,86)[0]==0u8) by (compute);
        assert(bytes_order_bkgd_hist_after_plte_before_idat_rgb().subrange(80,86)[1]==0u8) by (compute);
        assert(bytes_order_bkgd_hist_after_plte_before_idat_rgb().subrange(80,86)[2]==0u8) by (compute);
        assert(bytes_order_bkgd_hist_after_plte_before_idat_rgb().subrange(80,86)[3]==0u8) by (compute);
        assert(bytes_order_bkgd_hist_after_plte_before_idat_rgb().subrange(80,86)[4]==0u8) by (compute);
        assert(bytes_order_bkgd_hist_after_plte_before_idat_rgb().subrange(80,86)[5]==0u8) by (compute);
        assert(crate::metadata_model::words16(bytes_order_bkgd_hist_after_plte_before_idat_rgb().subrange(80,86),3nat).len()==3);
        assert(crate::model::be16(bytes_order_bkgd_hist_after_plte_before_idat_rgb().subrange(80,86),0)==0u16) by (compute);
        assert(crate::metadata_model::words16(bytes_order_bkgd_hist_after_plte_before_idat_rgb().subrange(80,86),3nat)[0]==0u16);
        assert(crate::model::be16(bytes_order_bkgd_hist_after_plte_before_idat_rgb().subrange(80,86),2)==0u16) by (compute);
        assert(crate::metadata_model::words16(bytes_order_bkgd_hist_after_plte_before_idat_rgb().subrange(80,86),3nat)[1]==0u16);
        assert(crate::model::be16(bytes_order_bkgd_hist_after_plte_before_idat_rgb().subrange(80,86),4)==0u16) by (compute);
        assert(crate::metadata_model::words16(bytes_order_bkgd_hist_after_plte_before_idat_rgb().subrange(80,86),3nat)[2]==0u16);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_order_bkgd_hist_after_plte_before_idat_rgb(),90nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:98nat,len:22nat,next:124nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_order_bkgd_hist_after_plte_before_idat_rgb(),90nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:98nat,len:22nat,next:124nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_order_bkgd_hist_after_plte_before_idat_rgb(),124nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:132nat,len:0nat,next:136nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_order_bkgd_hist_after_plte_before_idat_rgb(),124nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:132nat,len:0nat,next:136nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::metadata_model::parse(crate::walk_model::Name::Hist,bytes_order_bkgd_hist_after_plte_before_idat_rgb().subrange(62,68),2u8,8u8,3nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored)) by {
            reveal(crate::metadata_model::parse);
            assert(crate::metadata_model::parse(crate::walk_model::Name::Hist,bytes_order_bkgd_hist_after_plte_before_idat_rgb().subrange(62,68),2u8,8u8,3nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored)) by (compute);
        }
        assert forall|i: int| 0<=i<3 implies (#[trigger] crate::metadata_model::words16(bytes_order_bkgd_hist_after_plte_before_idat_rgb().subrange(80,86),3)[i] as nat)<(1u32<<8u8) as nat by {
            if i==0 {assert(crate::metadata_model::words16(bytes_order_bkgd_hist_after_plte_before_idat_rgb().subrange(80,86),3)[i]==0u16);}
            else if i==1 {assert(crate::metadata_model::words16(bytes_order_bkgd_hist_after_plte_before_idat_rgb().subrange(80,86),3)[i]==0u16);}
            else if i==2 {assert(crate::metadata_model::words16(bytes_order_bkgd_hist_after_plte_before_idat_rgb().subrange(80,86),3)[i]==0u16);}
        }
        assert(crate::metadata_model::parse(crate::walk_model::Name::Bkgd,bytes_order_bkgd_hist_after_plte_before_idat_rgb().subrange(80,86),2u8,8u8,3nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Bkgd(crate::metadata_model::words16(bytes_order_bkgd_hist_after_plte_before_idat_rgb().subrange(80,86),3)))) by {
            reveal(crate::metadata_model::parse);
            rgb_background_success(bytes_order_bkgd_hist_after_plte_before_idat_rgb().subrange(80,86),3nat);
        }
    assert(bytes_order_bkgd_hist_after_plte_before_idat_rgb().subrange(98,120)==z_order_bkgd_hist_after_plte_before_idat_rgb()) by (compute);
    walk_order_bkgd_hist_after_plte_before_idat_rgb(bytes_order_bkgd_hist_after_plte_before_idat_rgb());
    inflate_order_bkgd_hist_after_plte_before_idat_rgb();scan_order_bkgd_hist_after_plte_before_idat_rgb();
    assert(r::raw(a_order_bkgd_hist_after_plte_before_idat_rgb())==14nat) by (compute);
    assert(r::data(z_order_bkgd_hist_after_plte_before_idat_rgb(),a_order_bkgd_hist_after_plte_before_idat_rgb(),STRICT)==Some(Ok(()))) by {reveal(r::data);};
}
#[verifier::spinoff_prover]
proof fn pixel_order_bkgd_hist_after_plte_before_idat_rgb_0() ensures sm::image(raw_order_bkgd_hist_after_plte_before_idat_rgb(),a_order_bkgd_hist_after_plte_before_idat_rgb(),colour_order_bkgd_hist_after_plte_before_idat_rgb(),false)[0]==201u8,sm::image(raw_order_bkgd_hist_after_plte_before_idat_rgb(),a_order_bkgd_hist_after_plte_before_idat_rgb(),colour_order_bkgd_hist_after_plte_before_idat_rgb(),false)[1]==73u8,sm::image(raw_order_bkgd_hist_after_plte_before_idat_rgb(),a_order_bkgd_hist_after_plte_before_idat_rgb(),colour_order_bkgd_hist_after_plte_before_idat_rgb(),false)[2]==63u8,sm::image(raw_order_bkgd_hist_after_plte_before_idat_rgb(),a_order_bkgd_hist_after_plte_before_idat_rgb(),colour_order_bkgd_hist_after_plte_before_idat_rgb(),false)[3]==255u8,
{
    colour::depth_values(8u8);
    row_order_bkgd_hist_after_plte_before_idat_rgb_0_0();
    assert(crate::paint_model::source_cell(a_order_bkgd_hist_after_plte_before_idat_rgb(),0nat,0nat)==(0nat,0nat,0nat)) by (compute);
    assert(colour::pixel(colour_order_bkgd_hist_after_plte_before_idat_rgb(),seq![201u8,73u8,63u8,202u8,75u8,151u8],0nat,true,false)==Ok(seq![51657u16,18761u16,16191u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_order_bkgd_hist_after_plte_before_idat_rgb(),a_order_bkgd_hist_after_plte_before_idat_rgb(),colour_order_bkgd_hist_after_plte_before_idat_rgb(),0nat,0nat,true,false)==Ok(seq![51657u16,18761u16,16191u16,65535u16]));
    assert(((seq![51657u16,18761u16,16191u16,65535u16][0] as nat+128)/257) as u8==201u8) by (compute);
    assert(((seq![51657u16,18761u16,16191u16,65535u16][1] as nat+128)/257) as u8==73u8) by (compute);
    assert(((seq![51657u16,18761u16,16191u16,65535u16][2] as nat+128)/257) as u8==63u8) by (compute);
    assert(((seq![51657u16,18761u16,16191u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixel_order_bkgd_hist_after_plte_before_idat_rgb_1() ensures sm::image(raw_order_bkgd_hist_after_plte_before_idat_rgb(),a_order_bkgd_hist_after_plte_before_idat_rgb(),colour_order_bkgd_hist_after_plte_before_idat_rgb(),false)[4]==202u8,sm::image(raw_order_bkgd_hist_after_plte_before_idat_rgb(),a_order_bkgd_hist_after_plte_before_idat_rgb(),colour_order_bkgd_hist_after_plte_before_idat_rgb(),false)[5]==75u8,sm::image(raw_order_bkgd_hist_after_plte_before_idat_rgb(),a_order_bkgd_hist_after_plte_before_idat_rgb(),colour_order_bkgd_hist_after_plte_before_idat_rgb(),false)[6]==151u8,sm::image(raw_order_bkgd_hist_after_plte_before_idat_rgb(),a_order_bkgd_hist_after_plte_before_idat_rgb(),colour_order_bkgd_hist_after_plte_before_idat_rgb(),false)[7]==255u8,
{
    colour::depth_values(8u8);
    row_order_bkgd_hist_after_plte_before_idat_rgb_0_0();
    assert(crate::paint_model::source_cell(a_order_bkgd_hist_after_plte_before_idat_rgb(),1nat,0nat)==(0nat,1nat,0nat)) by (compute);
    assert(colour::pixel(colour_order_bkgd_hist_after_plte_before_idat_rgb(),seq![201u8,73u8,63u8,202u8,75u8,151u8],1nat,true,false)==Ok(seq![51914u16,19275u16,38807u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_order_bkgd_hist_after_plte_before_idat_rgb(),a_order_bkgd_hist_after_plte_before_idat_rgb(),colour_order_bkgd_hist_after_plte_before_idat_rgb(),1nat,0nat,true,false)==Ok(seq![51914u16,19275u16,38807u16,65535u16]));
    assert(((seq![51914u16,19275u16,38807u16,65535u16][0] as nat+128)/257) as u8==202u8) by (compute);
    assert(((seq![51914u16,19275u16,38807u16,65535u16][1] as nat+128)/257) as u8==75u8) by (compute);
    assert(((seq![51914u16,19275u16,38807u16,65535u16][2] as nat+128)/257) as u8==151u8) by (compute);
    assert(((seq![51914u16,19275u16,38807u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixel_order_bkgd_hist_after_plte_before_idat_rgb_2() ensures sm::image(raw_order_bkgd_hist_after_plte_before_idat_rgb(),a_order_bkgd_hist_after_plte_before_idat_rgb(),colour_order_bkgd_hist_after_plte_before_idat_rgb(),false)[8]==225u8,sm::image(raw_order_bkgd_hist_after_plte_before_idat_rgb(),a_order_bkgd_hist_after_plte_before_idat_rgb(),colour_order_bkgd_hist_after_plte_before_idat_rgb(),false)[9]==26u8,sm::image(raw_order_bkgd_hist_after_plte_before_idat_rgb(),a_order_bkgd_hist_after_plte_before_idat_rgb(),colour_order_bkgd_hist_after_plte_before_idat_rgb(),false)[10]==23u8,sm::image(raw_order_bkgd_hist_after_plte_before_idat_rgb(),a_order_bkgd_hist_after_plte_before_idat_rgb(),colour_order_bkgd_hist_after_plte_before_idat_rgb(),false)[11]==255u8,
{
    colour::depth_values(8u8);
    row_order_bkgd_hist_after_plte_before_idat_rgb_0_1();
    assert(crate::paint_model::source_cell(a_order_bkgd_hist_after_plte_before_idat_rgb(),0nat,1nat)==(0nat,0nat,1nat)) by (compute);
    assert(colour::pixel(colour_order_bkgd_hist_after_plte_before_idat_rgb(),seq![225u8,26u8,23u8,252u8,246u8,129u8],0nat,true,false)==Ok(seq![57825u16,6682u16,5911u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_order_bkgd_hist_after_plte_before_idat_rgb(),a_order_bkgd_hist_after_plte_before_idat_rgb(),colour_order_bkgd_hist_after_plte_before_idat_rgb(),0nat,1nat,true,false)==Ok(seq![57825u16,6682u16,5911u16,65535u16]));
    assert(((seq![57825u16,6682u16,5911u16,65535u16][0] as nat+128)/257) as u8==225u8) by (compute);
    assert(((seq![57825u16,6682u16,5911u16,65535u16][1] as nat+128)/257) as u8==26u8) by (compute);
    assert(((seq![57825u16,6682u16,5911u16,65535u16][2] as nat+128)/257) as u8==23u8) by (compute);
    assert(((seq![57825u16,6682u16,5911u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixel_order_bkgd_hist_after_plte_before_idat_rgb_3() ensures sm::image(raw_order_bkgd_hist_after_plte_before_idat_rgb(),a_order_bkgd_hist_after_plte_before_idat_rgb(),colour_order_bkgd_hist_after_plte_before_idat_rgb(),false)[12]==252u8,sm::image(raw_order_bkgd_hist_after_plte_before_idat_rgb(),a_order_bkgd_hist_after_plte_before_idat_rgb(),colour_order_bkgd_hist_after_plte_before_idat_rgb(),false)[13]==246u8,sm::image(raw_order_bkgd_hist_after_plte_before_idat_rgb(),a_order_bkgd_hist_after_plte_before_idat_rgb(),colour_order_bkgd_hist_after_plte_before_idat_rgb(),false)[14]==129u8,sm::image(raw_order_bkgd_hist_after_plte_before_idat_rgb(),a_order_bkgd_hist_after_plte_before_idat_rgb(),colour_order_bkgd_hist_after_plte_before_idat_rgb(),false)[15]==255u8,
{
    colour::depth_values(8u8);
    row_order_bkgd_hist_after_plte_before_idat_rgb_0_1();
    assert(crate::paint_model::source_cell(a_order_bkgd_hist_after_plte_before_idat_rgb(),1nat,1nat)==(0nat,1nat,1nat)) by (compute);
    assert(colour::pixel(colour_order_bkgd_hist_after_plte_before_idat_rgb(),seq![225u8,26u8,23u8,252u8,246u8,129u8],1nat,true,false)==Ok(seq![64764u16,63222u16,33153u16,65535u16])) by (compute);
    assert(crate::paint_model::source_pixel(raw_order_bkgd_hist_after_plte_before_idat_rgb(),a_order_bkgd_hist_after_plte_before_idat_rgb(),colour_order_bkgd_hist_after_plte_before_idat_rgb(),1nat,1nat,true,false)==Ok(seq![64764u16,63222u16,33153u16,65535u16]));
    assert(((seq![64764u16,63222u16,33153u16,65535u16][0] as nat+128)/257) as u8==252u8) by (compute);
    assert(((seq![64764u16,63222u16,33153u16,65535u16][1] as nat+128)/257) as u8==246u8) by (compute);
    assert(((seq![64764u16,63222u16,33153u16,65535u16][2] as nat+128)/257) as u8==129u8) by (compute);
    assert(((seq![64764u16,63222u16,33153u16,65535u16][3] as nat+128)/257) as u8==255u8) by (compute);
}
#[verifier::spinoff_prover]
proof fn pixels_order_bkgd_hist_after_plte_before_idat_rgb() ensures sm::image(raw_order_bkgd_hist_after_plte_before_idat_rgb(),a_order_bkgd_hist_after_plte_before_idat_rgb(),colour_order_bkgd_hist_after_plte_before_idat_rgb(),false)==expected_order_bkgd_hist_after_plte_before_idat_rgb(),
{
    pixel_order_bkgd_hist_after_plte_before_idat_rgb_0();
    pixel_order_bkgd_hist_after_plte_before_idat_rgb_1();
    pixel_order_bkgd_hist_after_plte_before_idat_rgb_2();
    pixel_order_bkgd_hist_after_plte_before_idat_rgb_3();
    assert(a_order_bkgd_hist_after_plte_before_idat_rgb().width==2nat && a_order_bkgd_hist_after_plte_before_idat_rgb().height==2nat) by (compute);
    assert(sm::image(raw_order_bkgd_hist_after_plte_before_idat_rgb(),a_order_bkgd_hist_after_plte_before_idat_rgb(),colour_order_bkgd_hist_after_plte_before_idat_rgb(),false).len()==16nat);
    assert(expected_order_bkgd_hist_after_plte_before_idat_rgb().len()==16nat) by (compute);
    assert forall|j: int| 0<=j<16 implies (#[trigger] sm::image(raw_order_bkgd_hist_after_plte_before_idat_rgb(),a_order_bkgd_hist_after_plte_before_idat_rgb(),colour_order_bkgd_hist_after_plte_before_idat_rgb(),false)[j])==expected_order_bkgd_hist_after_plte_before_idat_rgb()[j] by {
        if j==0 {assert(expected_order_bkgd_hist_after_plte_before_idat_rgb()[0]==201u8) by (compute);}
        else if j==1 {assert(expected_order_bkgd_hist_after_plte_before_idat_rgb()[1]==73u8) by (compute);}
        else if j==2 {assert(expected_order_bkgd_hist_after_plte_before_idat_rgb()[2]==63u8) by (compute);}
        else if j==3 {assert(expected_order_bkgd_hist_after_plte_before_idat_rgb()[3]==255u8) by (compute);}
        else if j==4 {assert(expected_order_bkgd_hist_after_plte_before_idat_rgb()[4]==202u8) by (compute);}
        else if j==5 {assert(expected_order_bkgd_hist_after_plte_before_idat_rgb()[5]==75u8) by (compute);}
        else if j==6 {assert(expected_order_bkgd_hist_after_plte_before_idat_rgb()[6]==151u8) by (compute);}
        else if j==7 {assert(expected_order_bkgd_hist_after_plte_before_idat_rgb()[7]==255u8) by (compute);}
        else if j==8 {assert(expected_order_bkgd_hist_after_plte_before_idat_rgb()[8]==225u8) by (compute);}
        else if j==9 {assert(expected_order_bkgd_hist_after_plte_before_idat_rgb()[9]==26u8) by (compute);}
        else if j==10 {assert(expected_order_bkgd_hist_after_plte_before_idat_rgb()[10]==23u8) by (compute);}
        else if j==11 {assert(expected_order_bkgd_hist_after_plte_before_idat_rgb()[11]==255u8) by (compute);}
        else if j==12 {assert(expected_order_bkgd_hist_after_plte_before_idat_rgb()[12]==252u8) by (compute);}
        else if j==13 {assert(expected_order_bkgd_hist_after_plte_before_idat_rgb()[13]==246u8) by (compute);}
        else if j==14 {assert(expected_order_bkgd_hist_after_plte_before_idat_rgb()[14]==129u8) by (compute);}
        else {assert(j==15);assert(expected_order_bkgd_hist_after_plte_before_idat_rgb()[j]==255u8) by (compute);}
    }
    assert(sm::image(raw_order_bkgd_hist_after_plte_before_idat_rgb(),a_order_bkgd_hist_after_plte_before_idat_rgb(),colour_order_bkgd_hist_after_plte_before_idat_rgb(),false)=~=expected_order_bkgd_hist_after_plte_before_idat_rgb());
}
pub open spec fn expected_value_order_bkgd_hist_after_plte_before_idat_rgb(value: sm::Value) -> bool {value.width==2u32 && value.height==2u32 && value.pixels==expected_order_bkgd_hist_after_plte_before_idat_rgb()}
#[verifier::spinoff_prover]
pub proof fn model_complete_order_bkgd_hist_after_plte_before_idat_rgb()
    ensures match sm::MS(bytes_order_bkgd_hist_after_plte_before_idat_rgb()) {sm::Outcome::Done(value)=>expected_value_order_bkgd_hist_after_plte_before_idat_rgb(value),_=>false},
{
    hide(m::MW_p);hide(zm::Inflate);hide(bytes_order_bkgd_hist_after_plte_before_idat_rgb);hide(z_order_bkgd_hist_after_plte_before_idat_rgb);hide(raw_order_bkgd_hist_after_plte_before_idat_rgb);hide(expected_order_bkgd_hist_after_plte_before_idat_rgb);
    assert(bytes_order_bkgd_hist_after_plte_before_idat_rgb().len()==136nat) by (compute);
    raster_order_bkgd_hist_after_plte_before_idat_rgb();inflate_order_bkgd_hist_after_plte_before_idat_rgb();pixels_order_bkgd_hist_after_plte_before_idat_rgb();
    assert(bytes_order_bkgd_hist_after_plte_before_idat_rgb().subrange(41,50)==seq![47u8,194u8,40u8,139u8,44u8,88u8,228u8,36u8,202u8]) by (compute);
    match m::MW_p(bytes_order_bkgd_hist_after_plte_before_idat_rgb(),m::strict()) {m::Outcome::Done(doc,_)=>{
        assert(colour::from_doc(doc,bytes_order_bkgd_hist_after_plte_before_idat_rgb())==colour_order_bkgd_hist_after_plte_before_idat_rgb());
        assert(crate::paint_model::default_raster(doc,STRICT)==a_order_bkgd_hist_after_plte_before_idat_rgb());
    },_=>{}}
    assert(r::raw(a_order_bkgd_hist_after_plte_before_idat_rgb())==14nat) by (compute);
    assert(r::inflated(z_order_bkgd_hist_after_plte_before_idat_rgb(),a_order_bkgd_hist_after_plte_before_idat_rgb(),STRICT).out==raw_order_bkgd_hist_after_plte_before_idat_rgb());
    compose_complete(bytes_order_bkgd_hist_after_plte_before_idat_rgb(),h_order_bkgd_hist_after_plte_before_idat_rgb(),z_order_bkgd_hist_after_plte_before_idat_rgb(),3nat,colour_order_bkgd_hist_after_plte_before_idat_rgb(),expected_order_bkgd_hist_after_plte_before_idat_rgb());
}
pub fn public_complete_order_bkgd_hist_after_plte_before_idat_rgb() -> (result: Result<crate::png::Still,TryError>)
    ensures se::oom_member(se::requests(bytes_order_bkgd_hist_after_plte_before_idat_rgb(),false),result),se::eq(bytes_order_bkgd_hist_after_plte_before_idat_rgb(),false,se::native8(result)),
        match se::native8(result) {Ok(value)=>expected_value_order_bkgd_hist_after_plte_before_idat_rgb(value),Err(TryError::OutOfMemory(_))=>true,_=>false},
{
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,9u8,80u8,76u8,84u8,69u8,47u8,194u8,40u8,139u8,44u8,88u8,228u8,36u8,202u8,64u8,183u8,32u8,36u8,0u8,0u8,0u8,6u8,104u8,73u8,83u8,84u8,0u8,0u8,0u8,0u8,0u8,0u8,95u8,184u8,160u8,47u8,0u8,0u8,0u8,6u8,98u8,75u8,71u8,68u8,0u8,0u8,0u8,0u8,0u8,0u8,249u8,67u8,187u8,127u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_order_bkgd_hist_after_plte_before_idat_rgb());model_complete_order_bkgd_hist_after_plte_before_idat_rgb();}
    crate::png::try_decode_still(&b)
}
} // verus!
