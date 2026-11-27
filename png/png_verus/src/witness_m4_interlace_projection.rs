//! Adam7 M4 format/shape projections; complete pixel witnesses remain due at M6.
use vstd::prelude::*;
use crate::{raster_model as r,png::Error,policy::STRICT,m4_trace_witness_model as t,
    m4_checksum_witness_model as c,m4_stream_witness_model as stream,m4_crc_witness_model as crc_cert};
use inflate_verus::model as zm;
verus! {
// interlace/ct0-d1-1x1; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 8450adbbe6a5cbc5246022fa2f62f8077a9b97bc31ee8a2a4e2e60618851689d
spec fn bytes_interlace_ct0_d1_1x1() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,1u8,0u8,0u8,0u8,1u8,64u8,105u8,201u8,178u8,0u8,0u8,0u8,10u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,104u8,0u8,0u8,0u8,130u8,0u8,129u8,119u8,205u8,114u8,182u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_interlace_ct0_d1_1x1() -> Seq<u8> {seq![120u8,156u8,99u8,104u8,0u8,0u8,0u8,130u8,0u8,129u8]}
spec fn h_interlace_ct0_d1_1x1() -> crate::model::Ihdr {crate::model::Ihdr {width:1u32,height:1u32,depth:1u8,color_type:0u8,interlace:1u8}}
spec fn a_interlace_ct0_d1_1x1() -> r::Raster {r::shape(h_interlace_ct0_d1_1x1(),1nat,1nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_interlace_ct0_d1_1x1(b: Seq<u8>)
    requires b.len()==67,crate::model::mh(b)==Ok((crate::model::Ihdr {width:1u32,height:1u32,depth:1u8,color_type:0u8,interlace:1u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:10nat,next:55nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,55nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:63nat,len:0nat,next:67nat,crc_ok:true,synthetic:false})),
        b.subrange(41,51)==z_interlace_ct0_d1_1x1(),
    ensures r::default_case(b,crate::model::Ihdr {width:1u32,height:1u32,depth:1u8,color_type:0u8,interlace:1u8},z_interlace_ct0_d1_1x1(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    hide(z_interlace_ct0_d1_1x1);
        assert((1u32 << 1u8)==2u32) by (compute);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_interlace_ct0_d1_1x1() -> Seq<u8> {seq![0u8,128u8]}
spec fn ts_interlace_ct0_d1_1x1() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:35nat,before:1nat,after:2nat,symbol:128nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:35nat,next:42nat,before:2nat,after:2nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_interlace_ct0_d1_1x1()
    ensures t::trace(z_interlace_ct0_d1_1x1(),raw_interlace_ct0_d1_1x1(),ts_interlace_ct0_d1_1x1(),0,3nat),
{
    hide(z_interlace_ct0_d1_1x1);hide(raw_interlace_ct0_d1_1x1);hide(ts_interlace_ct0_d1_1x1);hide(t::valid_step);
    assert(t::trace(z_interlace_ct0_d1_1x1(),raw_interlace_ct0_d1_1x1(),ts_interlace_ct0_d1_1x1(),0,3nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_interlace_ct0_d1_1x1()
    ensures raw_interlace_ct0_d1_1x1().len()==2nat,
        z_interlace_ct0_d1_1x1().len()==10nat,
        z_interlace_ct0_d1_1x1()[0]==120u8,
        z_interlace_ct0_d1_1x1()[1]==156u8,
        zm::field_value(z_interlace_ct0_d1_1x1(),16,1)==1nat,
        zm::field_value(z_interlace_ct0_d1_1x1(),17,2)==1nat,
        zm::be_value(z_interlace_ct0_d1_1x1(),6nat,4)==8519809nat,
        ts_interlace_ct0_d1_1x1().len()==3nat,
        ts_interlace_ct0_d1_1x1()[0].pos==19nat,
        ts_interlace_ct0_d1_1x1()[0].before==0nat,
        ts_interlace_ct0_d1_1x1().last().next==42nat,
{
    hide(z_interlace_ct0_d1_1x1);hide(raw_interlace_ct0_d1_1x1);hide(ts_interlace_ct0_d1_1x1);
    assert(raw_interlace_ct0_d1_1x1().len()==2nat) by (compute);
    assert(z_interlace_ct0_d1_1x1().len()==10nat) by (compute);
    assert(z_interlace_ct0_d1_1x1()[0]==120u8) by (compute);
    assert(z_interlace_ct0_d1_1x1()[1]==156u8) by (compute);
    assert(zm::field_value(z_interlace_ct0_d1_1x1(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_interlace_ct0_d1_1x1(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_interlace_ct0_d1_1x1(),6nat,4)==8519809nat) by (compute);
    assert(ts_interlace_ct0_d1_1x1().len()==3nat) by (compute);
    assert(ts_interlace_ct0_d1_1x1()[0].pos==19nat) by (compute);
    assert(ts_interlace_ct0_d1_1x1()[0].before==0nat) by (compute);
    assert(ts_interlace_ct0_d1_1x1().last().next==42nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_interlace_ct0_d1_1x1()
    ensures zm::Adler(raw_interlace_ct0_d1_1x1())==8519809nat,
{
    hide(raw_interlace_ct0_d1_1x1);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_interlace_ct0_d1_1x1().len()==2nat) by (compute);
    assert(c::sums(raw_interlace_ct0_d1_1x1(),0,2nat)==(128nat,130nat)) by (compute);
    c::sums_correct(raw_interlace_ct0_d1_1x1(),0,2nat);
    assert(raw_interlace_ct0_d1_1x1().subrange(0,2) =~= raw_interlace_ct0_d1_1x1());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((130nat%65521)*(u16::MAX as nat+1)+(1+128nat)%65521==8519809nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_interlace_ct0_d1_1x1()
    ensures r::inflated(z_interlace_ct0_d1_1x1(),a_interlace_ct0_d1_1x1(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_interlace_ct0_d1_1x1(),r2:false}),
        raw_interlace_ct0_d1_1x1().len()==2nat,
{
    hide(z_interlace_ct0_d1_1x1);hide(raw_interlace_ct0_d1_1x1);hide(ts_interlace_ct0_d1_1x1);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_interlace_ct0_d1_1x1();checksum_interlace_ct0_d1_1x1();
    assert(r::raw(a_interlace_ct0_d1_1x1())==2nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_interlace_ct0_d1_1x1();
    stream::fixed_stream(z_interlace_ct0_d1_1x1(),raw_interlace_ct0_d1_1x1(),ts_interlace_ct0_d1_1x1(),2nat,156nat,8519809nat);
}
#[verifier::spinoff_prover]
proof fn scan_interlace_ct0_d1_1x1()
    ensures r::scan(raw_interlace_ct0_d1_1x1(),a_interlace_ct0_d1_1x1(),0)==Ok(()),
{
    hide(raw_interlace_ct0_d1_1x1);
    assert(r::count(a_interlace_ct0_d1_1x1())==7nat) by (compute);
    r::scan_step(raw_interlace_ct0_d1_1x1(),a_interlace_ct0_d1_1x1(),7);
    assert(r::pw(a_interlace_ct0_d1_1x1(),6nat)==1nat && r::ph(a_interlace_ct0_d1_1x1(),6nat)==0nat && r::rb(a_interlace_ct0_d1_1x1(),6nat)==1nat && r::base(a_interlace_ct0_d1_1x1(),6nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct0_d1_1x1(),a_interlace_ct0_d1_1x1(),6nat,0);
    r::scan_step(raw_interlace_ct0_d1_1x1(),a_interlace_ct0_d1_1x1(),6nat);
    assert(r::pw(a_interlace_ct0_d1_1x1(),5nat)==0nat && r::ph(a_interlace_ct0_d1_1x1(),5nat)==1nat && r::rb(a_interlace_ct0_d1_1x1(),5nat)==0nat && r::base(a_interlace_ct0_d1_1x1(),5nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct0_d1_1x1(),a_interlace_ct0_d1_1x1(),5nat,0);
    r::scan_step(raw_interlace_ct0_d1_1x1(),a_interlace_ct0_d1_1x1(),5nat);
    assert(r::pw(a_interlace_ct0_d1_1x1(),4nat)==1nat && r::ph(a_interlace_ct0_d1_1x1(),4nat)==0nat && r::rb(a_interlace_ct0_d1_1x1(),4nat)==1nat && r::base(a_interlace_ct0_d1_1x1(),4nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct0_d1_1x1(),a_interlace_ct0_d1_1x1(),4nat,0);
    r::scan_step(raw_interlace_ct0_d1_1x1(),a_interlace_ct0_d1_1x1(),4nat);
    assert(r::pw(a_interlace_ct0_d1_1x1(),3nat)==0nat && r::ph(a_interlace_ct0_d1_1x1(),3nat)==1nat && r::rb(a_interlace_ct0_d1_1x1(),3nat)==0nat && r::base(a_interlace_ct0_d1_1x1(),3nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct0_d1_1x1(),a_interlace_ct0_d1_1x1(),3nat,0);
    r::scan_step(raw_interlace_ct0_d1_1x1(),a_interlace_ct0_d1_1x1(),3nat);
    assert(r::pw(a_interlace_ct0_d1_1x1(),2nat)==1nat && r::ph(a_interlace_ct0_d1_1x1(),2nat)==0nat && r::rb(a_interlace_ct0_d1_1x1(),2nat)==1nat && r::base(a_interlace_ct0_d1_1x1(),2nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct0_d1_1x1(),a_interlace_ct0_d1_1x1(),2nat,0);
    r::scan_step(raw_interlace_ct0_d1_1x1(),a_interlace_ct0_d1_1x1(),2nat);
    assert(r::pw(a_interlace_ct0_d1_1x1(),1nat)==0nat && r::ph(a_interlace_ct0_d1_1x1(),1nat)==1nat && r::rb(a_interlace_ct0_d1_1x1(),1nat)==0nat && r::base(a_interlace_ct0_d1_1x1(),1nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct0_d1_1x1(),a_interlace_ct0_d1_1x1(),1nat,0);
    r::scan_step(raw_interlace_ct0_d1_1x1(),a_interlace_ct0_d1_1x1(),1nat);
    assert(r::pw(a_interlace_ct0_d1_1x1(),0nat)==1nat && r::ph(a_interlace_ct0_d1_1x1(),0nat)==1nat && r::rb(a_interlace_ct0_d1_1x1(),0nat)==1nat && r::base(a_interlace_ct0_d1_1x1(),0nat)==0nat) by (compute);
    assert(raw_interlace_ct0_d1_1x1()[0]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_1x1(),a_interlace_ct0_d1_1x1(),0nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_1x1(),a_interlace_ct0_d1_1x1(),0nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_1x1(),a_interlace_ct0_d1_1x1(),0nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_1x1(),a_interlace_ct0_d1_1x1(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_interlace_ct0_d1_1x1()
    ensures r::still(bytes_interlace_ct0_d1_1x1(),STRICT,false)==(r::Projection::Ready {width:1u32,height:1u32,bytes:4nat}),
{
    hide(bytes_interlace_ct0_d1_1x1);hide(z_interlace_ct0_d1_1x1);hide(raw_interlace_ct0_d1_1x1);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_interlace_ct0_d1_1x1().len()==67nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct0_d1_1x1())==Ok((crate::model::Ihdr {width:1u32,height:1u32,depth:1u8,color_type:0u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct0_d1_1x1())==Ok((crate::model::Ihdr {width:1u32,height:1u32,depth:1u8,color_type:0u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 1u8)==2u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct0_d1_1x1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:10nat,next:55nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct0_d1_1x1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:10nat,next:55nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct0_d1_1x1(),55nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:63nat,len:0nat,next:67nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct0_d1_1x1(),55nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:63nat,len:0nat,next:67nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct0_d1_1x1().subrange(41,51)==z_interlace_ct0_d1_1x1()) by (compute);
    walk_interlace_ct0_d1_1x1(bytes_interlace_ct0_d1_1x1());
    inflate_interlace_ct0_d1_1x1();scan_interlace_ct0_d1_1x1();
    assert(r::raw(a_interlace_ct0_d1_1x1())==2nat) by (compute);
    assert(r::data(z_interlace_ct0_d1_1x1(),a_interlace_ct0_d1_1x1(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_interlace_ct0_d1_1x1(),h_interlace_ct0_d1_1x1(),z_interlace_ct0_d1_1x1(),0nat,false);
}
fn public_interlace_ct0_d1_1x1() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,1u8,0u8,0u8,0u8,1u8,64u8,105u8,201u8,178u8,0u8,0u8,0u8,10u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,104u8,0u8,0u8,0u8,130u8,0u8,129u8,119u8,205u8,114u8,182u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_interlace_ct0_d1_1x1());model_interlace_ct0_d1_1x1();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==1u32 && s.height==1u32 && s.rgba@.len()==4nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// interlace/ct6-d16-1x1; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_interlace_ct6_d16_1x1() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,16u8,6u8,0u8,0u8,1u8,56u8,130u8,40u8,92u8,0u8,0u8,0u8,17u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,200u8,80u8,188u8,62u8,125u8,91u8,124u8,174u8,23u8,0u8,17u8,70u8,3u8,196u8,100u8,212u8,110u8,131u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_interlace_ct6_d16_1x1() -> Seq<u8> {seq![120u8,156u8,99u8,200u8,80u8,188u8,62u8,125u8,91u8,124u8,174u8,23u8,0u8,17u8,70u8,3u8,196u8]}
spec fn h_interlace_ct6_d16_1x1() -> crate::model::Ihdr {crate::model::Ihdr {width:1u32,height:1u32,depth:16u8,color_type:6u8,interlace:1u8}}
spec fn a_interlace_ct6_d16_1x1() -> r::Raster {r::shape(h_interlace_ct6_d16_1x1(),1nat,1nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_interlace_ct6_d16_1x1(b: Seq<u8>)
    requires b.len()==74,crate::model::mh(b)==Ok((crate::model::Ihdr {width:1u32,height:1u32,depth:16u8,color_type:6u8,interlace:1u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:17nat,next:62nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,62nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:70nat,len:0nat,next:74nat,crc_ok:true,synthetic:false})),
        b.subrange(41,58)==z_interlace_ct6_d16_1x1(),
    ensures r::default_case(b,crate::model::Ihdr {width:1u32,height:1u32,depth:16u8,color_type:6u8,interlace:1u8},z_interlace_ct6_d16_1x1(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    hide(z_interlace_ct6_d16_1x1);
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_interlace_ct6_d16_1x1() -> Seq<u8> {seq![0u8,104u8,33u8,215u8,151u8,182u8,95u8,109u8,74u8]}
spec fn ts_interlace_ct6_d16_1x1() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:35nat,before:1nat,after:2nat,symbol:104nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:35nat,next:43nat,before:2nat,after:3nat,symbol:33nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:43nat,next:52nat,before:3nat,after:4nat,symbol:215nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:52nat,next:61nat,before:4nat,after:5nat,symbol:151nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:61nat,next:70nat,before:5nat,after:6nat,symbol:182nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:70nat,next:78nat,before:6nat,after:7nat,symbol:95nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:78nat,next:86nat,before:7nat,after:8nat,symbol:109nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:86nat,next:94nat,before:8nat,after:9nat,symbol:74nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:94nat,next:101nat,before:9nat,after:9nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_interlace_ct6_d16_1x1()
    ensures t::trace(z_interlace_ct6_d16_1x1(),raw_interlace_ct6_d16_1x1(),ts_interlace_ct6_d16_1x1(),0,10nat),
{
    hide(z_interlace_ct6_d16_1x1);hide(raw_interlace_ct6_d16_1x1);hide(ts_interlace_ct6_d16_1x1);hide(t::valid_step);
    assert(t::trace(z_interlace_ct6_d16_1x1(),raw_interlace_ct6_d16_1x1(),ts_interlace_ct6_d16_1x1(),0,10nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_interlace_ct6_d16_1x1()
    ensures raw_interlace_ct6_d16_1x1().len()==9nat,
        z_interlace_ct6_d16_1x1().len()==17nat,
        z_interlace_ct6_d16_1x1()[0]==120u8,
        z_interlace_ct6_d16_1x1()[1]==156u8,
        zm::field_value(z_interlace_ct6_d16_1x1(),16,1)==1nat,
        zm::field_value(z_interlace_ct6_d16_1x1(),17,2)==1nat,
        zm::be_value(z_interlace_ct6_d16_1x1(),13nat,4)==289801156nat,
        ts_interlace_ct6_d16_1x1().len()==10nat,
        ts_interlace_ct6_d16_1x1()[0].pos==19nat,
        ts_interlace_ct6_d16_1x1()[0].before==0nat,
        ts_interlace_ct6_d16_1x1().last().next==101nat,
{
    hide(z_interlace_ct6_d16_1x1);hide(raw_interlace_ct6_d16_1x1);hide(ts_interlace_ct6_d16_1x1);
    assert(raw_interlace_ct6_d16_1x1().len()==9nat) by (compute);
    assert(z_interlace_ct6_d16_1x1().len()==17nat) by (compute);
    assert(z_interlace_ct6_d16_1x1()[0]==120u8) by (compute);
    assert(z_interlace_ct6_d16_1x1()[1]==156u8) by (compute);
    assert(zm::field_value(z_interlace_ct6_d16_1x1(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_interlace_ct6_d16_1x1(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_interlace_ct6_d16_1x1(),13nat,4)==289801156nat) by (compute);
    assert(ts_interlace_ct6_d16_1x1().len()==10nat) by (compute);
    assert(ts_interlace_ct6_d16_1x1()[0].pos==19nat) by (compute);
    assert(ts_interlace_ct6_d16_1x1()[0].before==0nat) by (compute);
    assert(ts_interlace_ct6_d16_1x1().last().next==101nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_interlace_ct6_d16_1x1()
    ensures zm::Adler(raw_interlace_ct6_d16_1x1())==289801156nat,
{
    hide(raw_interlace_ct6_d16_1x1);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_interlace_ct6_d16_1x1().len()==9nat) by (compute);
    assert(c::sums(raw_interlace_ct6_d16_1x1(),0,9nat)==(963nat,4422nat)) by (compute);
    c::sums_correct(raw_interlace_ct6_d16_1x1(),0,9nat);
    assert(raw_interlace_ct6_d16_1x1().subrange(0,9) =~= raw_interlace_ct6_d16_1x1());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((4422nat%65521)*(u16::MAX as nat+1)+(1+963nat)%65521==289801156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_interlace_ct6_d16_1x1()
    ensures r::inflated(z_interlace_ct6_d16_1x1(),a_interlace_ct6_d16_1x1(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_interlace_ct6_d16_1x1(),r2:false}),
        raw_interlace_ct6_d16_1x1().len()==9nat,
{
    hide(z_interlace_ct6_d16_1x1);hide(raw_interlace_ct6_d16_1x1);hide(ts_interlace_ct6_d16_1x1);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_interlace_ct6_d16_1x1();checksum_interlace_ct6_d16_1x1();
    assert(r::raw(a_interlace_ct6_d16_1x1())==9nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_interlace_ct6_d16_1x1();
    stream::fixed_stream(z_interlace_ct6_d16_1x1(),raw_interlace_ct6_d16_1x1(),ts_interlace_ct6_d16_1x1(),9nat,156nat,289801156nat);
}
#[verifier::spinoff_prover]
proof fn scan_interlace_ct6_d16_1x1()
    ensures r::scan(raw_interlace_ct6_d16_1x1(),a_interlace_ct6_d16_1x1(),0)==Ok(()),
{
    hide(raw_interlace_ct6_d16_1x1);
    assert(r::count(a_interlace_ct6_d16_1x1())==7nat) by (compute);
    r::scan_step(raw_interlace_ct6_d16_1x1(),a_interlace_ct6_d16_1x1(),7);
    assert(r::pw(a_interlace_ct6_d16_1x1(),6nat)==1nat && r::ph(a_interlace_ct6_d16_1x1(),6nat)==0nat && r::rb(a_interlace_ct6_d16_1x1(),6nat)==8nat && r::base(a_interlace_ct6_d16_1x1(),6nat)==9nat) by (compute);
    r::pass_step(raw_interlace_ct6_d16_1x1(),a_interlace_ct6_d16_1x1(),6nat,0);
    r::scan_step(raw_interlace_ct6_d16_1x1(),a_interlace_ct6_d16_1x1(),6nat);
    assert(r::pw(a_interlace_ct6_d16_1x1(),5nat)==0nat && r::ph(a_interlace_ct6_d16_1x1(),5nat)==1nat && r::rb(a_interlace_ct6_d16_1x1(),5nat)==0nat && r::base(a_interlace_ct6_d16_1x1(),5nat)==9nat) by (compute);
    r::pass_step(raw_interlace_ct6_d16_1x1(),a_interlace_ct6_d16_1x1(),5nat,0);
    r::scan_step(raw_interlace_ct6_d16_1x1(),a_interlace_ct6_d16_1x1(),5nat);
    assert(r::pw(a_interlace_ct6_d16_1x1(),4nat)==1nat && r::ph(a_interlace_ct6_d16_1x1(),4nat)==0nat && r::rb(a_interlace_ct6_d16_1x1(),4nat)==8nat && r::base(a_interlace_ct6_d16_1x1(),4nat)==9nat) by (compute);
    r::pass_step(raw_interlace_ct6_d16_1x1(),a_interlace_ct6_d16_1x1(),4nat,0);
    r::scan_step(raw_interlace_ct6_d16_1x1(),a_interlace_ct6_d16_1x1(),4nat);
    assert(r::pw(a_interlace_ct6_d16_1x1(),3nat)==0nat && r::ph(a_interlace_ct6_d16_1x1(),3nat)==1nat && r::rb(a_interlace_ct6_d16_1x1(),3nat)==0nat && r::base(a_interlace_ct6_d16_1x1(),3nat)==9nat) by (compute);
    r::pass_step(raw_interlace_ct6_d16_1x1(),a_interlace_ct6_d16_1x1(),3nat,0);
    r::scan_step(raw_interlace_ct6_d16_1x1(),a_interlace_ct6_d16_1x1(),3nat);
    assert(r::pw(a_interlace_ct6_d16_1x1(),2nat)==1nat && r::ph(a_interlace_ct6_d16_1x1(),2nat)==0nat && r::rb(a_interlace_ct6_d16_1x1(),2nat)==8nat && r::base(a_interlace_ct6_d16_1x1(),2nat)==9nat) by (compute);
    r::pass_step(raw_interlace_ct6_d16_1x1(),a_interlace_ct6_d16_1x1(),2nat,0);
    r::scan_step(raw_interlace_ct6_d16_1x1(),a_interlace_ct6_d16_1x1(),2nat);
    assert(r::pw(a_interlace_ct6_d16_1x1(),1nat)==0nat && r::ph(a_interlace_ct6_d16_1x1(),1nat)==1nat && r::rb(a_interlace_ct6_d16_1x1(),1nat)==0nat && r::base(a_interlace_ct6_d16_1x1(),1nat)==9nat) by (compute);
    r::pass_step(raw_interlace_ct6_d16_1x1(),a_interlace_ct6_d16_1x1(),1nat,0);
    r::scan_step(raw_interlace_ct6_d16_1x1(),a_interlace_ct6_d16_1x1(),1nat);
    assert(r::pw(a_interlace_ct6_d16_1x1(),0nat)==1nat && r::ph(a_interlace_ct6_d16_1x1(),0nat)==1nat && r::rb(a_interlace_ct6_d16_1x1(),0nat)==8nat && r::base(a_interlace_ct6_d16_1x1(),0nat)==0nat) by (compute);
    assert(raw_interlace_ct6_d16_1x1()[0]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_1x1(),a_interlace_ct6_d16_1x1(),0nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_1x1(),a_interlace_ct6_d16_1x1(),0nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_1x1(),a_interlace_ct6_d16_1x1(),0nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_1x1(),a_interlace_ct6_d16_1x1(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_interlace_ct6_d16_1x1()
    ensures r::still(bytes_interlace_ct6_d16_1x1(),STRICT,false)==(r::Projection::Ready {width:1u32,height:1u32,bytes:4nat}),
{
    hide(bytes_interlace_ct6_d16_1x1);hide(z_interlace_ct6_d16_1x1);hide(raw_interlace_ct6_d16_1x1);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_interlace_ct6_d16_1x1().len()==74nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct6_d16_1x1())==Ok((crate::model::Ihdr {width:1u32,height:1u32,depth:16u8,color_type:6u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct6_d16_1x1())==Ok((crate::model::Ihdr {width:1u32,height:1u32,depth:16u8,color_type:6u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct6_d16_1x1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:17nat,next:62nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct6_d16_1x1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:17nat,next:62nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct6_d16_1x1(),62nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:70nat,len:0nat,next:74nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct6_d16_1x1(),62nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:70nat,len:0nat,next:74nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct6_d16_1x1().subrange(41,58)==z_interlace_ct6_d16_1x1()) by (compute);
    walk_interlace_ct6_d16_1x1(bytes_interlace_ct6_d16_1x1());
    inflate_interlace_ct6_d16_1x1();scan_interlace_ct6_d16_1x1();
    assert(r::raw(a_interlace_ct6_d16_1x1())==9nat) by (compute);
    assert(r::data(z_interlace_ct6_d16_1x1(),a_interlace_ct6_d16_1x1(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_interlace_ct6_d16_1x1(),h_interlace_ct6_d16_1x1(),z_interlace_ct6_d16_1x1(),0nat,false);
}
fn public_interlace_ct6_d16_1x1() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,16u8,6u8,0u8,0u8,1u8,56u8,130u8,40u8,92u8,0u8,0u8,0u8,17u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,200u8,80u8,188u8,62u8,125u8,91u8,124u8,174u8,23u8,0u8,17u8,70u8,3u8,196u8,100u8,212u8,110u8,131u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_interlace_ct6_d16_1x1());model_interlace_ct6_d16_1x1();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==1u32 && s.height==1u32 && s.rgba@.len()==4nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// interlace/ct3-d4-1x1; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_interlace_ct3_d4_1x1() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,4u8,3u8,0u8,0u8,1u8,154u8,60u8,233u8,44u8,0u8,0u8,0u8,48u8,80u8,76u8,84u8,69u8,81u8,144u8,125u8,101u8,64u8,177u8,228u8,169u8,188u8,76u8,40u8,153u8,237u8,218u8,35u8,151u8,40u8,162u8,34u8,24u8,141u8,23u8,54u8,198u8,120u8,240u8,209u8,125u8,189u8,50u8,29u8,165u8,196u8,115u8,208u8,128u8,195u8,46u8,41u8,101u8,192u8,41u8,152u8,245u8,128u8,108u8,162u8,192u8,130u8,245u8,208u8,46u8,0u8,0u8,0u8,10u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,48u8,0u8,0u8,0u8,50u8,0u8,49u8,105u8,200u8,152u8,250u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_interlace_ct3_d4_1x1() -> Seq<u8> {seq![120u8,156u8,99u8,48u8,0u8,0u8,0u8,50u8,0u8,49u8]}
spec fn h_interlace_ct3_d4_1x1() -> crate::model::Ihdr {crate::model::Ihdr {width:1u32,height:1u32,depth:4u8,color_type:3u8,interlace:1u8}}
spec fn a_interlace_ct3_d4_1x1() -> r::Raster {r::shape(h_interlace_ct3_d4_1x1(),1nat,1nat,16nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_interlace_ct3_d4_1x1(b: Seq<u8>)
    requires b.len()==127,crate::model::mh(b)==Ok((crate::model::Ihdr {width:1u32,height:1u32,depth:4u8,color_type:3u8,interlace:1u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:10nat,next:115nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,115nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:123nat,len:0nat,next:127nat,crc_ok:true,synthetic:false})),
        b.subrange(101,111)==z_interlace_ct3_d4_1x1(),
    ensures r::default_case(b,crate::model::Ihdr {width:1u32,height:1u32,depth:4u8,color_type:3u8,interlace:1u8},z_interlace_ct3_d4_1x1(),16nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);
    hide(z_interlace_ct3_d4_1x1);
    r::palette_default_case(b,crate::model::Ihdr {width:1u32,height:1u32,depth:4u8,color_type:3u8,interlace:1u8},z_interlace_ct3_d4_1x1(),10nat);
}
spec fn raw_interlace_ct3_d4_1x1() -> Seq<u8> {seq![0u8,48u8]}
spec fn ts_interlace_ct3_d4_1x1() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:35nat,before:1nat,after:2nat,symbol:48nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:35nat,next:42nat,before:2nat,after:2nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_interlace_ct3_d4_1x1()
    ensures t::trace(z_interlace_ct3_d4_1x1(),raw_interlace_ct3_d4_1x1(),ts_interlace_ct3_d4_1x1(),0,3nat),
{
    hide(z_interlace_ct3_d4_1x1);hide(raw_interlace_ct3_d4_1x1);hide(ts_interlace_ct3_d4_1x1);hide(t::valid_step);
    assert(t::trace(z_interlace_ct3_d4_1x1(),raw_interlace_ct3_d4_1x1(),ts_interlace_ct3_d4_1x1(),0,3nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_interlace_ct3_d4_1x1()
    ensures raw_interlace_ct3_d4_1x1().len()==2nat,
        z_interlace_ct3_d4_1x1().len()==10nat,
        z_interlace_ct3_d4_1x1()[0]==120u8,
        z_interlace_ct3_d4_1x1()[1]==156u8,
        zm::field_value(z_interlace_ct3_d4_1x1(),16,1)==1nat,
        zm::field_value(z_interlace_ct3_d4_1x1(),17,2)==1nat,
        zm::be_value(z_interlace_ct3_d4_1x1(),6nat,4)==3276849nat,
        ts_interlace_ct3_d4_1x1().len()==3nat,
        ts_interlace_ct3_d4_1x1()[0].pos==19nat,
        ts_interlace_ct3_d4_1x1()[0].before==0nat,
        ts_interlace_ct3_d4_1x1().last().next==42nat,
{
    hide(z_interlace_ct3_d4_1x1);hide(raw_interlace_ct3_d4_1x1);hide(ts_interlace_ct3_d4_1x1);
    assert(raw_interlace_ct3_d4_1x1().len()==2nat) by (compute);
    assert(z_interlace_ct3_d4_1x1().len()==10nat) by (compute);
    assert(z_interlace_ct3_d4_1x1()[0]==120u8) by (compute);
    assert(z_interlace_ct3_d4_1x1()[1]==156u8) by (compute);
    assert(zm::field_value(z_interlace_ct3_d4_1x1(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_interlace_ct3_d4_1x1(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_interlace_ct3_d4_1x1(),6nat,4)==3276849nat) by (compute);
    assert(ts_interlace_ct3_d4_1x1().len()==3nat) by (compute);
    assert(ts_interlace_ct3_d4_1x1()[0].pos==19nat) by (compute);
    assert(ts_interlace_ct3_d4_1x1()[0].before==0nat) by (compute);
    assert(ts_interlace_ct3_d4_1x1().last().next==42nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_interlace_ct3_d4_1x1()
    ensures zm::Adler(raw_interlace_ct3_d4_1x1())==3276849nat,
{
    hide(raw_interlace_ct3_d4_1x1);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_interlace_ct3_d4_1x1().len()==2nat) by (compute);
    assert(c::sums(raw_interlace_ct3_d4_1x1(),0,2nat)==(48nat,50nat)) by (compute);
    c::sums_correct(raw_interlace_ct3_d4_1x1(),0,2nat);
    assert(raw_interlace_ct3_d4_1x1().subrange(0,2) =~= raw_interlace_ct3_d4_1x1());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((50nat%65521)*(u16::MAX as nat+1)+(1+48nat)%65521==3276849nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_interlace_ct3_d4_1x1()
    ensures r::inflated(z_interlace_ct3_d4_1x1(),a_interlace_ct3_d4_1x1(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_interlace_ct3_d4_1x1(),r2:false}),
        raw_interlace_ct3_d4_1x1().len()==2nat,
{
    hide(z_interlace_ct3_d4_1x1);hide(raw_interlace_ct3_d4_1x1);hide(ts_interlace_ct3_d4_1x1);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_interlace_ct3_d4_1x1();checksum_interlace_ct3_d4_1x1();
    assert(r::raw(a_interlace_ct3_d4_1x1())==2nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_interlace_ct3_d4_1x1();
    stream::fixed_stream(z_interlace_ct3_d4_1x1(),raw_interlace_ct3_d4_1x1(),ts_interlace_ct3_d4_1x1(),2nat,156nat,3276849nat);
}
#[verifier::spinoff_prover]
proof fn scan_interlace_ct3_d4_1x1()
    ensures r::scan(raw_interlace_ct3_d4_1x1(),a_interlace_ct3_d4_1x1(),0)==Ok(()),
{
    hide(raw_interlace_ct3_d4_1x1);
    assert(r::count(a_interlace_ct3_d4_1x1())==7nat) by (compute);
    r::scan_step(raw_interlace_ct3_d4_1x1(),a_interlace_ct3_d4_1x1(),7);
    assert(r::pw(a_interlace_ct3_d4_1x1(),6nat)==1nat && r::ph(a_interlace_ct3_d4_1x1(),6nat)==0nat && r::rb(a_interlace_ct3_d4_1x1(),6nat)==1nat && r::base(a_interlace_ct3_d4_1x1(),6nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct3_d4_1x1(),a_interlace_ct3_d4_1x1(),6nat,0);
    r::scan_step(raw_interlace_ct3_d4_1x1(),a_interlace_ct3_d4_1x1(),6nat);
    assert(r::pw(a_interlace_ct3_d4_1x1(),5nat)==0nat && r::ph(a_interlace_ct3_d4_1x1(),5nat)==1nat && r::rb(a_interlace_ct3_d4_1x1(),5nat)==0nat && r::base(a_interlace_ct3_d4_1x1(),5nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct3_d4_1x1(),a_interlace_ct3_d4_1x1(),5nat,0);
    r::scan_step(raw_interlace_ct3_d4_1x1(),a_interlace_ct3_d4_1x1(),5nat);
    assert(r::pw(a_interlace_ct3_d4_1x1(),4nat)==1nat && r::ph(a_interlace_ct3_d4_1x1(),4nat)==0nat && r::rb(a_interlace_ct3_d4_1x1(),4nat)==1nat && r::base(a_interlace_ct3_d4_1x1(),4nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct3_d4_1x1(),a_interlace_ct3_d4_1x1(),4nat,0);
    r::scan_step(raw_interlace_ct3_d4_1x1(),a_interlace_ct3_d4_1x1(),4nat);
    assert(r::pw(a_interlace_ct3_d4_1x1(),3nat)==0nat && r::ph(a_interlace_ct3_d4_1x1(),3nat)==1nat && r::rb(a_interlace_ct3_d4_1x1(),3nat)==0nat && r::base(a_interlace_ct3_d4_1x1(),3nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct3_d4_1x1(),a_interlace_ct3_d4_1x1(),3nat,0);
    r::scan_step(raw_interlace_ct3_d4_1x1(),a_interlace_ct3_d4_1x1(),3nat);
    assert(r::pw(a_interlace_ct3_d4_1x1(),2nat)==1nat && r::ph(a_interlace_ct3_d4_1x1(),2nat)==0nat && r::rb(a_interlace_ct3_d4_1x1(),2nat)==1nat && r::base(a_interlace_ct3_d4_1x1(),2nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct3_d4_1x1(),a_interlace_ct3_d4_1x1(),2nat,0);
    r::scan_step(raw_interlace_ct3_d4_1x1(),a_interlace_ct3_d4_1x1(),2nat);
    assert(r::pw(a_interlace_ct3_d4_1x1(),1nat)==0nat && r::ph(a_interlace_ct3_d4_1x1(),1nat)==1nat && r::rb(a_interlace_ct3_d4_1x1(),1nat)==0nat && r::base(a_interlace_ct3_d4_1x1(),1nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct3_d4_1x1(),a_interlace_ct3_d4_1x1(),1nat,0);
    r::scan_step(raw_interlace_ct3_d4_1x1(),a_interlace_ct3_d4_1x1(),1nat);
    assert(r::pw(a_interlace_ct3_d4_1x1(),0nat)==1nat && r::ph(a_interlace_ct3_d4_1x1(),0nat)==1nat && r::rb(a_interlace_ct3_d4_1x1(),0nat)==1nat && r::base(a_interlace_ct3_d4_1x1(),0nat)==0nat) by (compute);
    assert(raw_interlace_ct3_d4_1x1()[0]==0u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_1x1(),a_interlace_ct3_d4_1x1(),0nat,0nat),1nat);
    assert(r::row_result(raw_interlace_ct3_d4_1x1(),a_interlace_ct3_d4_1x1(),0nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_1x1(),a_interlace_ct3_d4_1x1(),0nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_1x1(),a_interlace_ct3_d4_1x1(),0nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_1x1(),a_interlace_ct3_d4_1x1(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_interlace_ct3_d4_1x1()
    ensures r::still(bytes_interlace_ct3_d4_1x1(),STRICT,false)==(r::Projection::Ready {width:1u32,height:1u32,bytes:4nat}),
{
    hide(bytes_interlace_ct3_d4_1x1);hide(z_interlace_ct3_d4_1x1);hide(raw_interlace_ct3_d4_1x1);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);
    assert(bytes_interlace_ct3_d4_1x1().len()==127nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct3_d4_1x1())==Ok((crate::model::Ihdr {width:1u32,height:1u32,depth:4u8,color_type:3u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct3_d4_1x1())==Ok((crate::model::Ihdr {width:1u32,height:1u32,depth:4u8,color_type:3u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 4u8)==16u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_1x1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_1x1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(48nat,3u8,4u8,false,false,false,false)==crate::metadata_model::Decision::Keep(16nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_1x1(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:10nat,next:115nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_1x1(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:10nat,next:115nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_1x1(),115nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:123nat,len:0nat,next:127nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_1x1(),115nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:123nat,len:0nat,next:127nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct3_d4_1x1().subrange(101,111)==z_interlace_ct3_d4_1x1()) by (compute);
    walk_interlace_ct3_d4_1x1(bytes_interlace_ct3_d4_1x1());
    inflate_interlace_ct3_d4_1x1();scan_interlace_ct3_d4_1x1();
    assert(r::raw(a_interlace_ct3_d4_1x1())==2nat) by (compute);
    assert(r::data(z_interlace_ct3_d4_1x1(),a_interlace_ct3_d4_1x1(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_interlace_ct3_d4_1x1(),h_interlace_ct3_d4_1x1(),z_interlace_ct3_d4_1x1(),16nat,false);
}
fn public_interlace_ct3_d4_1x1() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,4u8,3u8,0u8,0u8,1u8,154u8,60u8,233u8,44u8,0u8,0u8,0u8,48u8,80u8,76u8,84u8,69u8,81u8,144u8,125u8,101u8,64u8,177u8,228u8,169u8,188u8,76u8,40u8,153u8,237u8,218u8,35u8,151u8,40u8,162u8,34u8,24u8,141u8,23u8,54u8,198u8,120u8,240u8,209u8,125u8,189u8,50u8,29u8,165u8,196u8,115u8,208u8,128u8,195u8,46u8,41u8,101u8,192u8,41u8,152u8,245u8,128u8,108u8,162u8,192u8,130u8,245u8,208u8,46u8,0u8,0u8,0u8,10u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,48u8,0u8,0u8,0u8,50u8,0u8,49u8,105u8,200u8,152u8,250u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_interlace_ct3_d4_1x1());model_interlace_ct3_d4_1x1();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==1u32 && s.height==1u32 && s.rgba@.len()==4nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// interlace/ct0-d1-1x2; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_interlace_ct0_d1_1x2() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,2u8,1u8,0u8,0u8,0u8,1u8,198u8,253u8,187u8,28u8,0u8,0u8,0u8,12u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,104u8,96u8,108u8,0u8,0u8,2u8,6u8,1u8,2u8,163u8,240u8,140u8,39u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_interlace_ct0_d1_1x2() -> Seq<u8> {seq![120u8,156u8,99u8,104u8,96u8,108u8,0u8,0u8,2u8,6u8,1u8,2u8]}
spec fn h_interlace_ct0_d1_1x2() -> crate::model::Ihdr {crate::model::Ihdr {width:1u32,height:2u32,depth:1u8,color_type:0u8,interlace:1u8}}
spec fn a_interlace_ct0_d1_1x2() -> r::Raster {r::shape(h_interlace_ct0_d1_1x2(),1nat,2nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_interlace_ct0_d1_1x2(b: Seq<u8>)
    requires b.len()==69,crate::model::mh(b)==Ok((crate::model::Ihdr {width:1u32,height:2u32,depth:1u8,color_type:0u8,interlace:1u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:12nat,next:57nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,57nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:65nat,len:0nat,next:69nat,crc_ok:true,synthetic:false})),
        b.subrange(41,53)==z_interlace_ct0_d1_1x2(),
    ensures r::default_case(b,crate::model::Ihdr {width:1u32,height:2u32,depth:1u8,color_type:0u8,interlace:1u8},z_interlace_ct0_d1_1x2(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    hide(z_interlace_ct0_d1_1x2);
        assert((1u32 << 1u8)==2u32) by (compute);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_interlace_ct0_d1_1x2() -> Seq<u8> {seq![0u8,128u8,1u8,128u8]}
spec fn ts_interlace_ct0_d1_1x2() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:35nat,before:1nat,after:2nat,symbol:128nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:35nat,next:43nat,before:2nat,after:3nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:43nat,next:51nat,before:3nat,after:4nat,symbol:128nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:51nat,next:58nat,before:4nat,after:4nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_interlace_ct0_d1_1x2()
    ensures t::trace(z_interlace_ct0_d1_1x2(),raw_interlace_ct0_d1_1x2(),ts_interlace_ct0_d1_1x2(),0,5nat),
{
    hide(z_interlace_ct0_d1_1x2);hide(raw_interlace_ct0_d1_1x2);hide(ts_interlace_ct0_d1_1x2);hide(t::valid_step);
    assert(t::trace(z_interlace_ct0_d1_1x2(),raw_interlace_ct0_d1_1x2(),ts_interlace_ct0_d1_1x2(),0,5nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_interlace_ct0_d1_1x2()
    ensures raw_interlace_ct0_d1_1x2().len()==4nat,
        z_interlace_ct0_d1_1x2().len()==12nat,
        z_interlace_ct0_d1_1x2()[0]==120u8,
        z_interlace_ct0_d1_1x2()[1]==156u8,
        zm::field_value(z_interlace_ct0_d1_1x2(),16,1)==1nat,
        zm::field_value(z_interlace_ct0_d1_1x2(),17,2)==1nat,
        zm::be_value(z_interlace_ct0_d1_1x2(),8nat,4)==33947906nat,
        ts_interlace_ct0_d1_1x2().len()==5nat,
        ts_interlace_ct0_d1_1x2()[0].pos==19nat,
        ts_interlace_ct0_d1_1x2()[0].before==0nat,
        ts_interlace_ct0_d1_1x2().last().next==58nat,
{
    hide(z_interlace_ct0_d1_1x2);hide(raw_interlace_ct0_d1_1x2);hide(ts_interlace_ct0_d1_1x2);
    assert(raw_interlace_ct0_d1_1x2().len()==4nat) by (compute);
    assert(z_interlace_ct0_d1_1x2().len()==12nat) by (compute);
    assert(z_interlace_ct0_d1_1x2()[0]==120u8) by (compute);
    assert(z_interlace_ct0_d1_1x2()[1]==156u8) by (compute);
    assert(zm::field_value(z_interlace_ct0_d1_1x2(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_interlace_ct0_d1_1x2(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_interlace_ct0_d1_1x2(),8nat,4)==33947906nat) by (compute);
    assert(ts_interlace_ct0_d1_1x2().len()==5nat) by (compute);
    assert(ts_interlace_ct0_d1_1x2()[0].pos==19nat) by (compute);
    assert(ts_interlace_ct0_d1_1x2()[0].before==0nat) by (compute);
    assert(ts_interlace_ct0_d1_1x2().last().next==58nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_interlace_ct0_d1_1x2()
    ensures zm::Adler(raw_interlace_ct0_d1_1x2())==33947906nat,
{
    hide(raw_interlace_ct0_d1_1x2);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_interlace_ct0_d1_1x2().len()==4nat) by (compute);
    assert(c::sums(raw_interlace_ct0_d1_1x2(),0,4nat)==(257nat,518nat)) by (compute);
    c::sums_correct(raw_interlace_ct0_d1_1x2(),0,4nat);
    assert(raw_interlace_ct0_d1_1x2().subrange(0,4) =~= raw_interlace_ct0_d1_1x2());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((518nat%65521)*(u16::MAX as nat+1)+(1+257nat)%65521==33947906nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_interlace_ct0_d1_1x2()
    ensures r::inflated(z_interlace_ct0_d1_1x2(),a_interlace_ct0_d1_1x2(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_interlace_ct0_d1_1x2(),r2:false}),
        raw_interlace_ct0_d1_1x2().len()==4nat,
{
    hide(z_interlace_ct0_d1_1x2);hide(raw_interlace_ct0_d1_1x2);hide(ts_interlace_ct0_d1_1x2);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_interlace_ct0_d1_1x2();checksum_interlace_ct0_d1_1x2();
    assert(r::raw(a_interlace_ct0_d1_1x2())==4nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_interlace_ct0_d1_1x2();
    stream::fixed_stream(z_interlace_ct0_d1_1x2(),raw_interlace_ct0_d1_1x2(),ts_interlace_ct0_d1_1x2(),4nat,156nat,33947906nat);
}
#[verifier::spinoff_prover]
proof fn scan_interlace_ct0_d1_1x2()
    ensures r::scan(raw_interlace_ct0_d1_1x2(),a_interlace_ct0_d1_1x2(),0)==Ok(()),
{
    hide(raw_interlace_ct0_d1_1x2);
    assert(r::count(a_interlace_ct0_d1_1x2())==7nat) by (compute);
    r::scan_step(raw_interlace_ct0_d1_1x2(),a_interlace_ct0_d1_1x2(),7);
    assert(r::pw(a_interlace_ct0_d1_1x2(),6nat)==1nat && r::ph(a_interlace_ct0_d1_1x2(),6nat)==1nat && r::rb(a_interlace_ct0_d1_1x2(),6nat)==1nat && r::base(a_interlace_ct0_d1_1x2(),6nat)==2nat) by (compute);
    assert(raw_interlace_ct0_d1_1x2()[2]==1u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_1x2(),a_interlace_ct0_d1_1x2(),6nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_1x2(),a_interlace_ct0_d1_1x2(),6nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_1x2(),a_interlace_ct0_d1_1x2(),6nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_1x2(),a_interlace_ct0_d1_1x2(),6nat);
    assert(r::pw(a_interlace_ct0_d1_1x2(),5nat)==0nat && r::ph(a_interlace_ct0_d1_1x2(),5nat)==1nat && r::rb(a_interlace_ct0_d1_1x2(),5nat)==0nat && r::base(a_interlace_ct0_d1_1x2(),5nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct0_d1_1x2(),a_interlace_ct0_d1_1x2(),5nat,0);
    r::scan_step(raw_interlace_ct0_d1_1x2(),a_interlace_ct0_d1_1x2(),5nat);
    assert(r::pw(a_interlace_ct0_d1_1x2(),4nat)==1nat && r::ph(a_interlace_ct0_d1_1x2(),4nat)==0nat && r::rb(a_interlace_ct0_d1_1x2(),4nat)==1nat && r::base(a_interlace_ct0_d1_1x2(),4nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct0_d1_1x2(),a_interlace_ct0_d1_1x2(),4nat,0);
    r::scan_step(raw_interlace_ct0_d1_1x2(),a_interlace_ct0_d1_1x2(),4nat);
    assert(r::pw(a_interlace_ct0_d1_1x2(),3nat)==0nat && r::ph(a_interlace_ct0_d1_1x2(),3nat)==1nat && r::rb(a_interlace_ct0_d1_1x2(),3nat)==0nat && r::base(a_interlace_ct0_d1_1x2(),3nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct0_d1_1x2(),a_interlace_ct0_d1_1x2(),3nat,0);
    r::scan_step(raw_interlace_ct0_d1_1x2(),a_interlace_ct0_d1_1x2(),3nat);
    assert(r::pw(a_interlace_ct0_d1_1x2(),2nat)==1nat && r::ph(a_interlace_ct0_d1_1x2(),2nat)==0nat && r::rb(a_interlace_ct0_d1_1x2(),2nat)==1nat && r::base(a_interlace_ct0_d1_1x2(),2nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct0_d1_1x2(),a_interlace_ct0_d1_1x2(),2nat,0);
    r::scan_step(raw_interlace_ct0_d1_1x2(),a_interlace_ct0_d1_1x2(),2nat);
    assert(r::pw(a_interlace_ct0_d1_1x2(),1nat)==0nat && r::ph(a_interlace_ct0_d1_1x2(),1nat)==1nat && r::rb(a_interlace_ct0_d1_1x2(),1nat)==0nat && r::base(a_interlace_ct0_d1_1x2(),1nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct0_d1_1x2(),a_interlace_ct0_d1_1x2(),1nat,0);
    r::scan_step(raw_interlace_ct0_d1_1x2(),a_interlace_ct0_d1_1x2(),1nat);
    assert(r::pw(a_interlace_ct0_d1_1x2(),0nat)==1nat && r::ph(a_interlace_ct0_d1_1x2(),0nat)==1nat && r::rb(a_interlace_ct0_d1_1x2(),0nat)==1nat && r::base(a_interlace_ct0_d1_1x2(),0nat)==0nat) by (compute);
    assert(raw_interlace_ct0_d1_1x2()[0]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_1x2(),a_interlace_ct0_d1_1x2(),0nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_1x2(),a_interlace_ct0_d1_1x2(),0nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_1x2(),a_interlace_ct0_d1_1x2(),0nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_1x2(),a_interlace_ct0_d1_1x2(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_interlace_ct0_d1_1x2()
    ensures r::still(bytes_interlace_ct0_d1_1x2(),STRICT,false)==(r::Projection::Ready {width:1u32,height:2u32,bytes:8nat}),
{
    hide(bytes_interlace_ct0_d1_1x2);hide(z_interlace_ct0_d1_1x2);hide(raw_interlace_ct0_d1_1x2);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_interlace_ct0_d1_1x2().len()==69nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct0_d1_1x2())==Ok((crate::model::Ihdr {width:1u32,height:2u32,depth:1u8,color_type:0u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct0_d1_1x2())==Ok((crate::model::Ihdr {width:1u32,height:2u32,depth:1u8,color_type:0u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 1u8)==2u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct0_d1_1x2(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:12nat,next:57nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct0_d1_1x2(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:12nat,next:57nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct0_d1_1x2(),57nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:65nat,len:0nat,next:69nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct0_d1_1x2(),57nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:65nat,len:0nat,next:69nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct0_d1_1x2().subrange(41,53)==z_interlace_ct0_d1_1x2()) by (compute);
    walk_interlace_ct0_d1_1x2(bytes_interlace_ct0_d1_1x2());
    inflate_interlace_ct0_d1_1x2();scan_interlace_ct0_d1_1x2();
    assert(r::raw(a_interlace_ct0_d1_1x2())==4nat) by (compute);
    assert(r::data(z_interlace_ct0_d1_1x2(),a_interlace_ct0_d1_1x2(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_interlace_ct0_d1_1x2(),h_interlace_ct0_d1_1x2(),z_interlace_ct0_d1_1x2(),0nat,false);
}
fn public_interlace_ct0_d1_1x2() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,2u8,1u8,0u8,0u8,0u8,1u8,198u8,253u8,187u8,28u8,0u8,0u8,0u8,12u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,104u8,96u8,108u8,0u8,0u8,2u8,6u8,1u8,2u8,163u8,240u8,140u8,39u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_interlace_ct0_d1_1x2());model_interlace_ct0_d1_1x2();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==1u32 && s.height==2u32 && s.rgba@.len()==8nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// interlace/ct6-d16-1x2; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_interlace_ct6_d16_1x2() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,2u8,16u8,6u8,0u8,0u8,1u8,190u8,22u8,90u8,242u8,0u8,0u8,0u8,27u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,88u8,114u8,116u8,39u8,59u8,79u8,103u8,193u8,71u8,198u8,27u8,31u8,229u8,171u8,210u8,38u8,62u8,23u8,0u8,0u8,77u8,102u8,8u8,113u8,190u8,101u8,86u8,216u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_interlace_ct6_d16_1x2() -> Seq<u8> {seq![120u8,156u8,99u8,88u8,114u8,116u8,39u8,59u8,79u8,103u8,193u8,71u8,198u8,27u8,31u8,229u8,171u8,210u8,38u8,62u8,23u8,0u8,0u8,77u8,102u8,8u8,113u8]}
spec fn h_interlace_ct6_d16_1x2() -> crate::model::Ihdr {crate::model::Ihdr {width:1u32,height:2u32,depth:16u8,color_type:6u8,interlace:1u8}}
spec fn a_interlace_ct6_d16_1x2() -> r::Raster {r::shape(h_interlace_ct6_d16_1x2(),1nat,2nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_interlace_ct6_d16_1x2(b: Seq<u8>)
    requires b.len()==84,crate::model::mh(b)==Ok((crate::model::Ihdr {width:1u32,height:2u32,depth:16u8,color_type:6u8,interlace:1u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:27nat,next:72nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,72nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:80nat,len:0nat,next:84nat,crc_ok:true,synthetic:false})),
        b.subrange(41,68)==z_interlace_ct6_d16_1x2(),
    ensures r::default_case(b,crate::model::Ihdr {width:1u32,height:2u32,depth:16u8,color_type:6u8,interlace:1u8},z_interlace_ct6_d16_1x2(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    hide(z_interlace_ct6_d16_1x2);
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_interlace_ct6_d16_1x2() -> Seq<u8> {seq![0u8,164u8,197u8,185u8,7u8,12u8,137u8,112u8,241u8,1u8,216u8,241u8,31u8,122u8,102u8,145u8,231u8,16u8]}
spec fn ts_interlace_ct6_d16_1x2() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:36nat,before:1nat,after:2nat,symbol:164nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:36nat,next:45nat,before:2nat,after:3nat,symbol:197nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:45nat,next:54nat,before:3nat,after:4nat,symbol:185nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:54nat,next:62nat,before:4nat,after:5nat,symbol:7nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:62nat,next:70nat,before:5nat,after:6nat,symbol:12nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:70nat,next:78nat,before:6nat,after:7nat,symbol:137nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:78nat,next:86nat,before:7nat,after:8nat,symbol:112nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:86nat,next:95nat,before:8nat,after:9nat,symbol:241nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:95nat,next:103nat,before:9nat,after:10nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:103nat,next:112nat,before:10nat,after:11nat,symbol:216nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:112nat,next:121nat,before:11nat,after:12nat,symbol:241nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:121nat,next:129nat,before:12nat,after:13nat,symbol:31nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:129nat,next:137nat,before:13nat,after:14nat,symbol:122nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:137nat,next:145nat,before:14nat,after:15nat,symbol:102nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:145nat,next:154nat,before:15nat,after:16nat,symbol:145nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:154nat,next:163nat,before:16nat,after:17nat,symbol:231nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:163nat,next:171nat,before:17nat,after:18nat,symbol:16nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:171nat,next:178nat,before:18nat,after:18nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_interlace_ct6_d16_1x2()
    ensures t::trace(z_interlace_ct6_d16_1x2(),raw_interlace_ct6_d16_1x2(),ts_interlace_ct6_d16_1x2(),0,19nat),
{
    hide(z_interlace_ct6_d16_1x2);hide(raw_interlace_ct6_d16_1x2);hide(ts_interlace_ct6_d16_1x2);hide(t::valid_step);
    assert(t::trace(z_interlace_ct6_d16_1x2(),raw_interlace_ct6_d16_1x2(),ts_interlace_ct6_d16_1x2(),0,19nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_interlace_ct6_d16_1x2()
    ensures raw_interlace_ct6_d16_1x2().len()==18nat,
        z_interlace_ct6_d16_1x2().len()==27nat,
        z_interlace_ct6_d16_1x2()[0]==120u8,
        z_interlace_ct6_d16_1x2()[1]==156u8,
        zm::field_value(z_interlace_ct6_d16_1x2(),16,1)==1nat,
        zm::field_value(z_interlace_ct6_d16_1x2(),17,2)==1nat,
        zm::be_value(z_interlace_ct6_d16_1x2(),23nat,4)==1298532465nat,
        ts_interlace_ct6_d16_1x2().len()==19nat,
        ts_interlace_ct6_d16_1x2()[0].pos==19nat,
        ts_interlace_ct6_d16_1x2()[0].before==0nat,
        ts_interlace_ct6_d16_1x2().last().next==178nat,
{
    hide(z_interlace_ct6_d16_1x2);hide(raw_interlace_ct6_d16_1x2);hide(ts_interlace_ct6_d16_1x2);
    assert(raw_interlace_ct6_d16_1x2().len()==18nat) by (compute);
    assert(z_interlace_ct6_d16_1x2().len()==27nat) by (compute);
    assert(z_interlace_ct6_d16_1x2()[0]==120u8) by (compute);
    assert(z_interlace_ct6_d16_1x2()[1]==156u8) by (compute);
    assert(zm::field_value(z_interlace_ct6_d16_1x2(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_interlace_ct6_d16_1x2(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_interlace_ct6_d16_1x2(),23nat,4)==1298532465nat) by (compute);
    assert(ts_interlace_ct6_d16_1x2().len()==19nat) by (compute);
    assert(ts_interlace_ct6_d16_1x2()[0].pos==19nat) by (compute);
    assert(ts_interlace_ct6_d16_1x2()[0].before==0nat) by (compute);
    assert(ts_interlace_ct6_d16_1x2().last().next==178nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_interlace_ct6_d16_1x2()
    ensures zm::Adler(raw_interlace_ct6_d16_1x2())==1298532465nat,
{
    hide(raw_interlace_ct6_d16_1x2);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_interlace_ct6_d16_1x2().len()==18nat) by (compute);
    assert(c::sums(raw_interlace_ct6_d16_1x2(),0,18nat)==(2160nat,19814nat)) by (compute);
    c::sums_correct(raw_interlace_ct6_d16_1x2(),0,18nat);
    assert(raw_interlace_ct6_d16_1x2().subrange(0,18) =~= raw_interlace_ct6_d16_1x2());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((19814nat%65521)*(u16::MAX as nat+1)+(1+2160nat)%65521==1298532465nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_interlace_ct6_d16_1x2()
    ensures r::inflated(z_interlace_ct6_d16_1x2(),a_interlace_ct6_d16_1x2(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_interlace_ct6_d16_1x2(),r2:false}),
        raw_interlace_ct6_d16_1x2().len()==18nat,
{
    hide(z_interlace_ct6_d16_1x2);hide(raw_interlace_ct6_d16_1x2);hide(ts_interlace_ct6_d16_1x2);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_interlace_ct6_d16_1x2();checksum_interlace_ct6_d16_1x2();
    assert(r::raw(a_interlace_ct6_d16_1x2())==18nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_interlace_ct6_d16_1x2();
    stream::fixed_stream(z_interlace_ct6_d16_1x2(),raw_interlace_ct6_d16_1x2(),ts_interlace_ct6_d16_1x2(),18nat,156nat,1298532465nat);
}
#[verifier::spinoff_prover]
proof fn scan_interlace_ct6_d16_1x2()
    ensures r::scan(raw_interlace_ct6_d16_1x2(),a_interlace_ct6_d16_1x2(),0)==Ok(()),
{
    hide(raw_interlace_ct6_d16_1x2);
    assert(r::count(a_interlace_ct6_d16_1x2())==7nat) by (compute);
    r::scan_step(raw_interlace_ct6_d16_1x2(),a_interlace_ct6_d16_1x2(),7);
    assert(r::pw(a_interlace_ct6_d16_1x2(),6nat)==1nat && r::ph(a_interlace_ct6_d16_1x2(),6nat)==1nat && r::rb(a_interlace_ct6_d16_1x2(),6nat)==8nat && r::base(a_interlace_ct6_d16_1x2(),6nat)==9nat) by (compute);
    assert(raw_interlace_ct6_d16_1x2()[9]==1u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_1x2(),a_interlace_ct6_d16_1x2(),6nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_1x2(),a_interlace_ct6_d16_1x2(),6nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_1x2(),a_interlace_ct6_d16_1x2(),6nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_1x2(),a_interlace_ct6_d16_1x2(),6nat);
    assert(r::pw(a_interlace_ct6_d16_1x2(),5nat)==0nat && r::ph(a_interlace_ct6_d16_1x2(),5nat)==1nat && r::rb(a_interlace_ct6_d16_1x2(),5nat)==0nat && r::base(a_interlace_ct6_d16_1x2(),5nat)==9nat) by (compute);
    r::pass_step(raw_interlace_ct6_d16_1x2(),a_interlace_ct6_d16_1x2(),5nat,0);
    r::scan_step(raw_interlace_ct6_d16_1x2(),a_interlace_ct6_d16_1x2(),5nat);
    assert(r::pw(a_interlace_ct6_d16_1x2(),4nat)==1nat && r::ph(a_interlace_ct6_d16_1x2(),4nat)==0nat && r::rb(a_interlace_ct6_d16_1x2(),4nat)==8nat && r::base(a_interlace_ct6_d16_1x2(),4nat)==9nat) by (compute);
    r::pass_step(raw_interlace_ct6_d16_1x2(),a_interlace_ct6_d16_1x2(),4nat,0);
    r::scan_step(raw_interlace_ct6_d16_1x2(),a_interlace_ct6_d16_1x2(),4nat);
    assert(r::pw(a_interlace_ct6_d16_1x2(),3nat)==0nat && r::ph(a_interlace_ct6_d16_1x2(),3nat)==1nat && r::rb(a_interlace_ct6_d16_1x2(),3nat)==0nat && r::base(a_interlace_ct6_d16_1x2(),3nat)==9nat) by (compute);
    r::pass_step(raw_interlace_ct6_d16_1x2(),a_interlace_ct6_d16_1x2(),3nat,0);
    r::scan_step(raw_interlace_ct6_d16_1x2(),a_interlace_ct6_d16_1x2(),3nat);
    assert(r::pw(a_interlace_ct6_d16_1x2(),2nat)==1nat && r::ph(a_interlace_ct6_d16_1x2(),2nat)==0nat && r::rb(a_interlace_ct6_d16_1x2(),2nat)==8nat && r::base(a_interlace_ct6_d16_1x2(),2nat)==9nat) by (compute);
    r::pass_step(raw_interlace_ct6_d16_1x2(),a_interlace_ct6_d16_1x2(),2nat,0);
    r::scan_step(raw_interlace_ct6_d16_1x2(),a_interlace_ct6_d16_1x2(),2nat);
    assert(r::pw(a_interlace_ct6_d16_1x2(),1nat)==0nat && r::ph(a_interlace_ct6_d16_1x2(),1nat)==1nat && r::rb(a_interlace_ct6_d16_1x2(),1nat)==0nat && r::base(a_interlace_ct6_d16_1x2(),1nat)==9nat) by (compute);
    r::pass_step(raw_interlace_ct6_d16_1x2(),a_interlace_ct6_d16_1x2(),1nat,0);
    r::scan_step(raw_interlace_ct6_d16_1x2(),a_interlace_ct6_d16_1x2(),1nat);
    assert(r::pw(a_interlace_ct6_d16_1x2(),0nat)==1nat && r::ph(a_interlace_ct6_d16_1x2(),0nat)==1nat && r::rb(a_interlace_ct6_d16_1x2(),0nat)==8nat && r::base(a_interlace_ct6_d16_1x2(),0nat)==0nat) by (compute);
    assert(raw_interlace_ct6_d16_1x2()[0]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_1x2(),a_interlace_ct6_d16_1x2(),0nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_1x2(),a_interlace_ct6_d16_1x2(),0nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_1x2(),a_interlace_ct6_d16_1x2(),0nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_1x2(),a_interlace_ct6_d16_1x2(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_interlace_ct6_d16_1x2()
    ensures r::still(bytes_interlace_ct6_d16_1x2(),STRICT,false)==(r::Projection::Ready {width:1u32,height:2u32,bytes:8nat}),
{
    hide(bytes_interlace_ct6_d16_1x2);hide(z_interlace_ct6_d16_1x2);hide(raw_interlace_ct6_d16_1x2);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_interlace_ct6_d16_1x2().len()==84nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct6_d16_1x2())==Ok((crate::model::Ihdr {width:1u32,height:2u32,depth:16u8,color_type:6u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct6_d16_1x2())==Ok((crate::model::Ihdr {width:1u32,height:2u32,depth:16u8,color_type:6u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct6_d16_1x2(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:27nat,next:72nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct6_d16_1x2(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:27nat,next:72nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct6_d16_1x2(),72nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:80nat,len:0nat,next:84nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct6_d16_1x2(),72nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:80nat,len:0nat,next:84nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct6_d16_1x2().subrange(41,68)==z_interlace_ct6_d16_1x2()) by (compute);
    walk_interlace_ct6_d16_1x2(bytes_interlace_ct6_d16_1x2());
    inflate_interlace_ct6_d16_1x2();scan_interlace_ct6_d16_1x2();
    assert(r::raw(a_interlace_ct6_d16_1x2())==18nat) by (compute);
    assert(r::data(z_interlace_ct6_d16_1x2(),a_interlace_ct6_d16_1x2(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_interlace_ct6_d16_1x2(),h_interlace_ct6_d16_1x2(),z_interlace_ct6_d16_1x2(),0nat,false);
}
fn public_interlace_ct6_d16_1x2() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,2u8,16u8,6u8,0u8,0u8,1u8,190u8,22u8,90u8,242u8,0u8,0u8,0u8,27u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,88u8,114u8,116u8,39u8,59u8,79u8,103u8,193u8,71u8,198u8,27u8,31u8,229u8,171u8,210u8,38u8,62u8,23u8,0u8,0u8,77u8,102u8,8u8,113u8,190u8,101u8,86u8,216u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_interlace_ct6_d16_1x2());model_interlace_ct6_d16_1x2();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==1u32 && s.height==2u32 && s.rgba@.len()==8nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// interlace/ct3-d4-1x2; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_interlace_ct3_d4_1x2() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,2u8,4u8,3u8,0u8,0u8,1u8,28u8,168u8,155u8,130u8,0u8,0u8,0u8,48u8,80u8,76u8,84u8,69u8,48u8,133u8,152u8,119u8,73u8,36u8,137u8,169u8,47u8,31u8,222u8,37u8,218u8,42u8,11u8,195u8,93u8,100u8,205u8,159u8,179u8,3u8,251u8,34u8,252u8,216u8,107u8,91u8,226u8,12u8,195u8,205u8,79u8,172u8,16u8,169u8,169u8,254u8,66u8,31u8,92u8,97u8,148u8,36u8,145u8,249u8,13u8,185u8,153u8,213u8,47u8,0u8,0u8,0u8,0u8,12u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,80u8,96u8,44u8,0u8,0u8,0u8,214u8,0u8,146u8,188u8,134u8,196u8,42u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_interlace_ct3_d4_1x2() -> Seq<u8> {seq![120u8,156u8,99u8,80u8,96u8,44u8,0u8,0u8,0u8,214u8,0u8,146u8]}
spec fn h_interlace_ct3_d4_1x2() -> crate::model::Ihdr {crate::model::Ihdr {width:1u32,height:2u32,depth:4u8,color_type:3u8,interlace:1u8}}
spec fn a_interlace_ct3_d4_1x2() -> r::Raster {r::shape(h_interlace_ct3_d4_1x2(),1nat,2nat,16nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_interlace_ct3_d4_1x2(b: Seq<u8>)
    requires b.len()==129,crate::model::mh(b)==Ok((crate::model::Ihdr {width:1u32,height:2u32,depth:4u8,color_type:3u8,interlace:1u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:12nat,next:117nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,117nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:125nat,len:0nat,next:129nat,crc_ok:true,synthetic:false})),
        b.subrange(101,113)==z_interlace_ct3_d4_1x2(),
    ensures r::default_case(b,crate::model::Ihdr {width:1u32,height:2u32,depth:4u8,color_type:3u8,interlace:1u8},z_interlace_ct3_d4_1x2(),16nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);
    hide(z_interlace_ct3_d4_1x2);
    r::palette_default_case(b,crate::model::Ihdr {width:1u32,height:2u32,depth:4u8,color_type:3u8,interlace:1u8},z_interlace_ct3_d4_1x2(),12nat);
}
spec fn raw_interlace_ct3_d4_1x2() -> Seq<u8> {seq![0u8,32u8,1u8,112u8]}
spec fn ts_interlace_ct3_d4_1x2() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:35nat,before:1nat,after:2nat,symbol:32nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:35nat,next:43nat,before:2nat,after:3nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:43nat,next:51nat,before:3nat,after:4nat,symbol:112nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:51nat,next:58nat,before:4nat,after:4nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_interlace_ct3_d4_1x2()
    ensures t::trace(z_interlace_ct3_d4_1x2(),raw_interlace_ct3_d4_1x2(),ts_interlace_ct3_d4_1x2(),0,5nat),
{
    hide(z_interlace_ct3_d4_1x2);hide(raw_interlace_ct3_d4_1x2);hide(ts_interlace_ct3_d4_1x2);hide(t::valid_step);
    assert(t::trace(z_interlace_ct3_d4_1x2(),raw_interlace_ct3_d4_1x2(),ts_interlace_ct3_d4_1x2(),0,5nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_interlace_ct3_d4_1x2()
    ensures raw_interlace_ct3_d4_1x2().len()==4nat,
        z_interlace_ct3_d4_1x2().len()==12nat,
        z_interlace_ct3_d4_1x2()[0]==120u8,
        z_interlace_ct3_d4_1x2()[1]==156u8,
        zm::field_value(z_interlace_ct3_d4_1x2(),16,1)==1nat,
        zm::field_value(z_interlace_ct3_d4_1x2(),17,2)==1nat,
        zm::be_value(z_interlace_ct3_d4_1x2(),8nat,4)==14024850nat,
        ts_interlace_ct3_d4_1x2().len()==5nat,
        ts_interlace_ct3_d4_1x2()[0].pos==19nat,
        ts_interlace_ct3_d4_1x2()[0].before==0nat,
        ts_interlace_ct3_d4_1x2().last().next==58nat,
{
    hide(z_interlace_ct3_d4_1x2);hide(raw_interlace_ct3_d4_1x2);hide(ts_interlace_ct3_d4_1x2);
    assert(raw_interlace_ct3_d4_1x2().len()==4nat) by (compute);
    assert(z_interlace_ct3_d4_1x2().len()==12nat) by (compute);
    assert(z_interlace_ct3_d4_1x2()[0]==120u8) by (compute);
    assert(z_interlace_ct3_d4_1x2()[1]==156u8) by (compute);
    assert(zm::field_value(z_interlace_ct3_d4_1x2(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_interlace_ct3_d4_1x2(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_interlace_ct3_d4_1x2(),8nat,4)==14024850nat) by (compute);
    assert(ts_interlace_ct3_d4_1x2().len()==5nat) by (compute);
    assert(ts_interlace_ct3_d4_1x2()[0].pos==19nat) by (compute);
    assert(ts_interlace_ct3_d4_1x2()[0].before==0nat) by (compute);
    assert(ts_interlace_ct3_d4_1x2().last().next==58nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_interlace_ct3_d4_1x2()
    ensures zm::Adler(raw_interlace_ct3_d4_1x2())==14024850nat,
{
    hide(raw_interlace_ct3_d4_1x2);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_interlace_ct3_d4_1x2().len()==4nat) by (compute);
    assert(c::sums(raw_interlace_ct3_d4_1x2(),0,4nat)==(145nat,214nat)) by (compute);
    c::sums_correct(raw_interlace_ct3_d4_1x2(),0,4nat);
    assert(raw_interlace_ct3_d4_1x2().subrange(0,4) =~= raw_interlace_ct3_d4_1x2());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((214nat%65521)*(u16::MAX as nat+1)+(1+145nat)%65521==14024850nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_interlace_ct3_d4_1x2()
    ensures r::inflated(z_interlace_ct3_d4_1x2(),a_interlace_ct3_d4_1x2(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_interlace_ct3_d4_1x2(),r2:false}),
        raw_interlace_ct3_d4_1x2().len()==4nat,
{
    hide(z_interlace_ct3_d4_1x2);hide(raw_interlace_ct3_d4_1x2);hide(ts_interlace_ct3_d4_1x2);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_interlace_ct3_d4_1x2();checksum_interlace_ct3_d4_1x2();
    assert(r::raw(a_interlace_ct3_d4_1x2())==4nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_interlace_ct3_d4_1x2();
    stream::fixed_stream(z_interlace_ct3_d4_1x2(),raw_interlace_ct3_d4_1x2(),ts_interlace_ct3_d4_1x2(),4nat,156nat,14024850nat);
}
#[verifier::spinoff_prover]
proof fn scan_interlace_ct3_d4_1x2()
    ensures r::scan(raw_interlace_ct3_d4_1x2(),a_interlace_ct3_d4_1x2(),0)==Ok(()),
{
    hide(raw_interlace_ct3_d4_1x2);
    assert(r::count(a_interlace_ct3_d4_1x2())==7nat) by (compute);
    r::scan_step(raw_interlace_ct3_d4_1x2(),a_interlace_ct3_d4_1x2(),7);
    assert(r::pw(a_interlace_ct3_d4_1x2(),6nat)==1nat && r::ph(a_interlace_ct3_d4_1x2(),6nat)==1nat && r::rb(a_interlace_ct3_d4_1x2(),6nat)==1nat && r::base(a_interlace_ct3_d4_1x2(),6nat)==2nat) by (compute);
    assert(raw_interlace_ct3_d4_1x2()[2]==1u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_1x2(),a_interlace_ct3_d4_1x2(),6nat,0nat),1nat);
    assert(r::row_result(raw_interlace_ct3_d4_1x2(),a_interlace_ct3_d4_1x2(),6nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_1x2(),a_interlace_ct3_d4_1x2(),6nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_1x2(),a_interlace_ct3_d4_1x2(),6nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_1x2(),a_interlace_ct3_d4_1x2(),6nat);
    assert(r::pw(a_interlace_ct3_d4_1x2(),5nat)==0nat && r::ph(a_interlace_ct3_d4_1x2(),5nat)==1nat && r::rb(a_interlace_ct3_d4_1x2(),5nat)==0nat && r::base(a_interlace_ct3_d4_1x2(),5nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct3_d4_1x2(),a_interlace_ct3_d4_1x2(),5nat,0);
    r::scan_step(raw_interlace_ct3_d4_1x2(),a_interlace_ct3_d4_1x2(),5nat);
    assert(r::pw(a_interlace_ct3_d4_1x2(),4nat)==1nat && r::ph(a_interlace_ct3_d4_1x2(),4nat)==0nat && r::rb(a_interlace_ct3_d4_1x2(),4nat)==1nat && r::base(a_interlace_ct3_d4_1x2(),4nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct3_d4_1x2(),a_interlace_ct3_d4_1x2(),4nat,0);
    r::scan_step(raw_interlace_ct3_d4_1x2(),a_interlace_ct3_d4_1x2(),4nat);
    assert(r::pw(a_interlace_ct3_d4_1x2(),3nat)==0nat && r::ph(a_interlace_ct3_d4_1x2(),3nat)==1nat && r::rb(a_interlace_ct3_d4_1x2(),3nat)==0nat && r::base(a_interlace_ct3_d4_1x2(),3nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct3_d4_1x2(),a_interlace_ct3_d4_1x2(),3nat,0);
    r::scan_step(raw_interlace_ct3_d4_1x2(),a_interlace_ct3_d4_1x2(),3nat);
    assert(r::pw(a_interlace_ct3_d4_1x2(),2nat)==1nat && r::ph(a_interlace_ct3_d4_1x2(),2nat)==0nat && r::rb(a_interlace_ct3_d4_1x2(),2nat)==1nat && r::base(a_interlace_ct3_d4_1x2(),2nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct3_d4_1x2(),a_interlace_ct3_d4_1x2(),2nat,0);
    r::scan_step(raw_interlace_ct3_d4_1x2(),a_interlace_ct3_d4_1x2(),2nat);
    assert(r::pw(a_interlace_ct3_d4_1x2(),1nat)==0nat && r::ph(a_interlace_ct3_d4_1x2(),1nat)==1nat && r::rb(a_interlace_ct3_d4_1x2(),1nat)==0nat && r::base(a_interlace_ct3_d4_1x2(),1nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct3_d4_1x2(),a_interlace_ct3_d4_1x2(),1nat,0);
    r::scan_step(raw_interlace_ct3_d4_1x2(),a_interlace_ct3_d4_1x2(),1nat);
    assert(r::pw(a_interlace_ct3_d4_1x2(),0nat)==1nat && r::ph(a_interlace_ct3_d4_1x2(),0nat)==1nat && r::rb(a_interlace_ct3_d4_1x2(),0nat)==1nat && r::base(a_interlace_ct3_d4_1x2(),0nat)==0nat) by (compute);
    assert(raw_interlace_ct3_d4_1x2()[0]==0u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_1x2(),a_interlace_ct3_d4_1x2(),0nat,0nat),1nat);
    assert(r::row_result(raw_interlace_ct3_d4_1x2(),a_interlace_ct3_d4_1x2(),0nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_1x2(),a_interlace_ct3_d4_1x2(),0nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_1x2(),a_interlace_ct3_d4_1x2(),0nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_1x2(),a_interlace_ct3_d4_1x2(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_interlace_ct3_d4_1x2()
    ensures r::still(bytes_interlace_ct3_d4_1x2(),STRICT,false)==(r::Projection::Ready {width:1u32,height:2u32,bytes:8nat}),
{
    hide(bytes_interlace_ct3_d4_1x2);hide(z_interlace_ct3_d4_1x2);hide(raw_interlace_ct3_d4_1x2);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);
    assert(bytes_interlace_ct3_d4_1x2().len()==129nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct3_d4_1x2())==Ok((crate::model::Ihdr {width:1u32,height:2u32,depth:4u8,color_type:3u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct3_d4_1x2())==Ok((crate::model::Ihdr {width:1u32,height:2u32,depth:4u8,color_type:3u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 4u8)==16u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_1x2(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_1x2(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(48nat,3u8,4u8,false,false,false,false)==crate::metadata_model::Decision::Keep(16nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_1x2(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:12nat,next:117nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_1x2(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:12nat,next:117nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_1x2(),117nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:125nat,len:0nat,next:129nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_1x2(),117nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:125nat,len:0nat,next:129nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct3_d4_1x2().subrange(101,113)==z_interlace_ct3_d4_1x2()) by (compute);
    walk_interlace_ct3_d4_1x2(bytes_interlace_ct3_d4_1x2());
    inflate_interlace_ct3_d4_1x2();scan_interlace_ct3_d4_1x2();
    assert(r::raw(a_interlace_ct3_d4_1x2())==4nat) by (compute);
    assert(r::data(z_interlace_ct3_d4_1x2(),a_interlace_ct3_d4_1x2(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_interlace_ct3_d4_1x2(),h_interlace_ct3_d4_1x2(),z_interlace_ct3_d4_1x2(),16nat,false);
}
fn public_interlace_ct3_d4_1x2() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,2u8,4u8,3u8,0u8,0u8,1u8,28u8,168u8,155u8,130u8,0u8,0u8,0u8,48u8,80u8,76u8,84u8,69u8,48u8,133u8,152u8,119u8,73u8,36u8,137u8,169u8,47u8,31u8,222u8,37u8,218u8,42u8,11u8,195u8,93u8,100u8,205u8,159u8,179u8,3u8,251u8,34u8,252u8,216u8,107u8,91u8,226u8,12u8,195u8,205u8,79u8,172u8,16u8,169u8,169u8,254u8,66u8,31u8,92u8,97u8,148u8,36u8,145u8,249u8,13u8,185u8,153u8,213u8,47u8,0u8,0u8,0u8,0u8,12u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,80u8,96u8,44u8,0u8,0u8,0u8,214u8,0u8,146u8,188u8,134u8,196u8,42u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_interlace_ct3_d4_1x2());model_interlace_ct3_d4_1x2();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==1u32 && s.height==2u32 && s.rgba@.len()==8nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// interlace/ct0-d1-2x1; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_interlace_ct0_d1_2x1() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,1u8,1u8,0u8,0u8,0u8,1u8,171u8,94u8,114u8,177u8,0u8,0u8,0u8,12u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,96u8,108u8,0u8,0u8,0u8,134u8,0u8,130u8,167u8,183u8,79u8,85u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_interlace_ct0_d1_2x1() -> Seq<u8> {seq![120u8,156u8,99u8,96u8,96u8,108u8,0u8,0u8,0u8,134u8,0u8,130u8]}
spec fn h_interlace_ct0_d1_2x1() -> crate::model::Ihdr {crate::model::Ihdr {width:2u32,height:1u32,depth:1u8,color_type:0u8,interlace:1u8}}
spec fn a_interlace_ct0_d1_2x1() -> r::Raster {r::shape(h_interlace_ct0_d1_2x1(),2nat,1nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_interlace_ct0_d1_2x1(b: Seq<u8>)
    requires b.len()==69,crate::model::mh(b)==Ok((crate::model::Ihdr {width:2u32,height:1u32,depth:1u8,color_type:0u8,interlace:1u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:12nat,next:57nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,57nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:65nat,len:0nat,next:69nat,crc_ok:true,synthetic:false})),
        b.subrange(41,53)==z_interlace_ct0_d1_2x1(),
    ensures r::default_case(b,crate::model::Ihdr {width:2u32,height:1u32,depth:1u8,color_type:0u8,interlace:1u8},z_interlace_ct0_d1_2x1(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    hide(z_interlace_ct0_d1_2x1);
        assert((1u32 << 1u8)==2u32) by (compute);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_interlace_ct0_d1_2x1() -> Seq<u8> {seq![0u8,0u8,1u8,128u8]}
spec fn ts_interlace_ct0_d1_2x1() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:35nat,before:1nat,after:2nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:35nat,next:43nat,before:2nat,after:3nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:43nat,next:51nat,before:3nat,after:4nat,symbol:128nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:51nat,next:58nat,before:4nat,after:4nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_interlace_ct0_d1_2x1()
    ensures t::trace(z_interlace_ct0_d1_2x1(),raw_interlace_ct0_d1_2x1(),ts_interlace_ct0_d1_2x1(),0,5nat),
{
    hide(z_interlace_ct0_d1_2x1);hide(raw_interlace_ct0_d1_2x1);hide(ts_interlace_ct0_d1_2x1);hide(t::valid_step);
    assert(t::trace(z_interlace_ct0_d1_2x1(),raw_interlace_ct0_d1_2x1(),ts_interlace_ct0_d1_2x1(),0,5nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_interlace_ct0_d1_2x1()
    ensures raw_interlace_ct0_d1_2x1().len()==4nat,
        z_interlace_ct0_d1_2x1().len()==12nat,
        z_interlace_ct0_d1_2x1()[0]==120u8,
        z_interlace_ct0_d1_2x1()[1]==156u8,
        zm::field_value(z_interlace_ct0_d1_2x1(),16,1)==1nat,
        zm::field_value(z_interlace_ct0_d1_2x1(),17,2)==1nat,
        zm::be_value(z_interlace_ct0_d1_2x1(),8nat,4)==8781954nat,
        ts_interlace_ct0_d1_2x1().len()==5nat,
        ts_interlace_ct0_d1_2x1()[0].pos==19nat,
        ts_interlace_ct0_d1_2x1()[0].before==0nat,
        ts_interlace_ct0_d1_2x1().last().next==58nat,
{
    hide(z_interlace_ct0_d1_2x1);hide(raw_interlace_ct0_d1_2x1);hide(ts_interlace_ct0_d1_2x1);
    assert(raw_interlace_ct0_d1_2x1().len()==4nat) by (compute);
    assert(z_interlace_ct0_d1_2x1().len()==12nat) by (compute);
    assert(z_interlace_ct0_d1_2x1()[0]==120u8) by (compute);
    assert(z_interlace_ct0_d1_2x1()[1]==156u8) by (compute);
    assert(zm::field_value(z_interlace_ct0_d1_2x1(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_interlace_ct0_d1_2x1(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_interlace_ct0_d1_2x1(),8nat,4)==8781954nat) by (compute);
    assert(ts_interlace_ct0_d1_2x1().len()==5nat) by (compute);
    assert(ts_interlace_ct0_d1_2x1()[0].pos==19nat) by (compute);
    assert(ts_interlace_ct0_d1_2x1()[0].before==0nat) by (compute);
    assert(ts_interlace_ct0_d1_2x1().last().next==58nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_interlace_ct0_d1_2x1()
    ensures zm::Adler(raw_interlace_ct0_d1_2x1())==8781954nat,
{
    hide(raw_interlace_ct0_d1_2x1);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_interlace_ct0_d1_2x1().len()==4nat) by (compute);
    assert(c::sums(raw_interlace_ct0_d1_2x1(),0,4nat)==(129nat,134nat)) by (compute);
    c::sums_correct(raw_interlace_ct0_d1_2x1(),0,4nat);
    assert(raw_interlace_ct0_d1_2x1().subrange(0,4) =~= raw_interlace_ct0_d1_2x1());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((134nat%65521)*(u16::MAX as nat+1)+(1+129nat)%65521==8781954nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_interlace_ct0_d1_2x1()
    ensures r::inflated(z_interlace_ct0_d1_2x1(),a_interlace_ct0_d1_2x1(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_interlace_ct0_d1_2x1(),r2:false}),
        raw_interlace_ct0_d1_2x1().len()==4nat,
{
    hide(z_interlace_ct0_d1_2x1);hide(raw_interlace_ct0_d1_2x1);hide(ts_interlace_ct0_d1_2x1);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_interlace_ct0_d1_2x1();checksum_interlace_ct0_d1_2x1();
    assert(r::raw(a_interlace_ct0_d1_2x1())==4nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_interlace_ct0_d1_2x1();
    stream::fixed_stream(z_interlace_ct0_d1_2x1(),raw_interlace_ct0_d1_2x1(),ts_interlace_ct0_d1_2x1(),4nat,156nat,8781954nat);
}
#[verifier::spinoff_prover]
proof fn scan_interlace_ct0_d1_2x1()
    ensures r::scan(raw_interlace_ct0_d1_2x1(),a_interlace_ct0_d1_2x1(),0)==Ok(()),
{
    hide(raw_interlace_ct0_d1_2x1);
    assert(r::count(a_interlace_ct0_d1_2x1())==7nat) by (compute);
    r::scan_step(raw_interlace_ct0_d1_2x1(),a_interlace_ct0_d1_2x1(),7);
    assert(r::pw(a_interlace_ct0_d1_2x1(),6nat)==2nat && r::ph(a_interlace_ct0_d1_2x1(),6nat)==0nat && r::rb(a_interlace_ct0_d1_2x1(),6nat)==1nat && r::base(a_interlace_ct0_d1_2x1(),6nat)==4nat) by (compute);
    r::pass_step(raw_interlace_ct0_d1_2x1(),a_interlace_ct0_d1_2x1(),6nat,0);
    r::scan_step(raw_interlace_ct0_d1_2x1(),a_interlace_ct0_d1_2x1(),6nat);
    assert(r::pw(a_interlace_ct0_d1_2x1(),5nat)==1nat && r::ph(a_interlace_ct0_d1_2x1(),5nat)==1nat && r::rb(a_interlace_ct0_d1_2x1(),5nat)==1nat && r::base(a_interlace_ct0_d1_2x1(),5nat)==2nat) by (compute);
    assert(raw_interlace_ct0_d1_2x1()[2]==1u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_2x1(),a_interlace_ct0_d1_2x1(),5nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_2x1(),a_interlace_ct0_d1_2x1(),5nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_2x1(),a_interlace_ct0_d1_2x1(),5nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_2x1(),a_interlace_ct0_d1_2x1(),5nat);
    assert(r::pw(a_interlace_ct0_d1_2x1(),4nat)==1nat && r::ph(a_interlace_ct0_d1_2x1(),4nat)==0nat && r::rb(a_interlace_ct0_d1_2x1(),4nat)==1nat && r::base(a_interlace_ct0_d1_2x1(),4nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct0_d1_2x1(),a_interlace_ct0_d1_2x1(),4nat,0);
    r::scan_step(raw_interlace_ct0_d1_2x1(),a_interlace_ct0_d1_2x1(),4nat);
    assert(r::pw(a_interlace_ct0_d1_2x1(),3nat)==0nat && r::ph(a_interlace_ct0_d1_2x1(),3nat)==1nat && r::rb(a_interlace_ct0_d1_2x1(),3nat)==0nat && r::base(a_interlace_ct0_d1_2x1(),3nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct0_d1_2x1(),a_interlace_ct0_d1_2x1(),3nat,0);
    r::scan_step(raw_interlace_ct0_d1_2x1(),a_interlace_ct0_d1_2x1(),3nat);
    assert(r::pw(a_interlace_ct0_d1_2x1(),2nat)==1nat && r::ph(a_interlace_ct0_d1_2x1(),2nat)==0nat && r::rb(a_interlace_ct0_d1_2x1(),2nat)==1nat && r::base(a_interlace_ct0_d1_2x1(),2nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct0_d1_2x1(),a_interlace_ct0_d1_2x1(),2nat,0);
    r::scan_step(raw_interlace_ct0_d1_2x1(),a_interlace_ct0_d1_2x1(),2nat);
    assert(r::pw(a_interlace_ct0_d1_2x1(),1nat)==0nat && r::ph(a_interlace_ct0_d1_2x1(),1nat)==1nat && r::rb(a_interlace_ct0_d1_2x1(),1nat)==0nat && r::base(a_interlace_ct0_d1_2x1(),1nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct0_d1_2x1(),a_interlace_ct0_d1_2x1(),1nat,0);
    r::scan_step(raw_interlace_ct0_d1_2x1(),a_interlace_ct0_d1_2x1(),1nat);
    assert(r::pw(a_interlace_ct0_d1_2x1(),0nat)==1nat && r::ph(a_interlace_ct0_d1_2x1(),0nat)==1nat && r::rb(a_interlace_ct0_d1_2x1(),0nat)==1nat && r::base(a_interlace_ct0_d1_2x1(),0nat)==0nat) by (compute);
    assert(raw_interlace_ct0_d1_2x1()[0]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_2x1(),a_interlace_ct0_d1_2x1(),0nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_2x1(),a_interlace_ct0_d1_2x1(),0nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_2x1(),a_interlace_ct0_d1_2x1(),0nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_2x1(),a_interlace_ct0_d1_2x1(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_interlace_ct0_d1_2x1()
    ensures r::still(bytes_interlace_ct0_d1_2x1(),STRICT,false)==(r::Projection::Ready {width:2u32,height:1u32,bytes:8nat}),
{
    hide(bytes_interlace_ct0_d1_2x1);hide(z_interlace_ct0_d1_2x1);hide(raw_interlace_ct0_d1_2x1);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_interlace_ct0_d1_2x1().len()==69nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct0_d1_2x1())==Ok((crate::model::Ihdr {width:2u32,height:1u32,depth:1u8,color_type:0u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct0_d1_2x1())==Ok((crate::model::Ihdr {width:2u32,height:1u32,depth:1u8,color_type:0u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 1u8)==2u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct0_d1_2x1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:12nat,next:57nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct0_d1_2x1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:12nat,next:57nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct0_d1_2x1(),57nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:65nat,len:0nat,next:69nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct0_d1_2x1(),57nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:65nat,len:0nat,next:69nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct0_d1_2x1().subrange(41,53)==z_interlace_ct0_d1_2x1()) by (compute);
    walk_interlace_ct0_d1_2x1(bytes_interlace_ct0_d1_2x1());
    inflate_interlace_ct0_d1_2x1();scan_interlace_ct0_d1_2x1();
    assert(r::raw(a_interlace_ct0_d1_2x1())==4nat) by (compute);
    assert(r::data(z_interlace_ct0_d1_2x1(),a_interlace_ct0_d1_2x1(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_interlace_ct0_d1_2x1(),h_interlace_ct0_d1_2x1(),z_interlace_ct0_d1_2x1(),0nat,false);
}
fn public_interlace_ct0_d1_2x1() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,1u8,1u8,0u8,0u8,0u8,1u8,171u8,94u8,114u8,177u8,0u8,0u8,0u8,12u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,96u8,108u8,0u8,0u8,0u8,134u8,0u8,130u8,167u8,183u8,79u8,85u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_interlace_ct0_d1_2x1());model_interlace_ct0_d1_2x1();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==2u32 && s.height==1u32 && s.rgba@.len()==8nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// interlace/ct6-d16-2x1; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_interlace_ct6_d16_2x1() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,1u8,16u8,6u8,0u8,0u8,1u8,211u8,181u8,147u8,95u8,0u8,0u8,0u8,27u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,8u8,102u8,158u8,94u8,252u8,238u8,68u8,213u8,74u8,70u8,150u8,218u8,60u8,173u8,235u8,55u8,79u8,171u8,3u8,0u8,68u8,212u8,7u8,246u8,210u8,201u8,17u8,122u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_interlace_ct6_d16_2x1() -> Seq<u8> {seq![120u8,156u8,99u8,8u8,102u8,158u8,94u8,252u8,238u8,68u8,213u8,74u8,70u8,150u8,218u8,60u8,173u8,235u8,55u8,79u8,171u8,3u8,0u8,68u8,212u8,7u8,246u8]}
spec fn h_interlace_ct6_d16_2x1() -> crate::model::Ihdr {crate::model::Ihdr {width:2u32,height:1u32,depth:16u8,color_type:6u8,interlace:1u8}}
spec fn a_interlace_ct6_d16_2x1() -> r::Raster {r::shape(h_interlace_ct6_d16_2x1(),2nat,1nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_interlace_ct6_d16_2x1(b: Seq<u8>)
    requires b.len()==84,crate::model::mh(b)==Ok((crate::model::Ihdr {width:2u32,height:1u32,depth:16u8,color_type:6u8,interlace:1u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:27nat,next:72nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,72nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:80nat,len:0nat,next:84nat,crc_ok:true,synthetic:false})),
        b.subrange(41,68)==z_interlace_ct6_d16_2x1(),
    ensures r::default_case(b,crate::model::Ihdr {width:2u32,height:1u32,depth:16u8,color_type:6u8,interlace:1u8},z_interlace_ct6_d16_2x1(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    hide(z_interlace_ct6_d16_2x1);
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_interlace_ct6_d16_2x1() -> Seq<u8> {seq![0u8,83u8,3u8,151u8,115u8,238u8,200u8,122u8,169u8,1u8,4u8,125u8,110u8,42u8,215u8,217u8,203u8,39u8]}
spec fn ts_interlace_ct6_d16_2x1() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:35nat,before:1nat,after:2nat,symbol:83nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:35nat,next:43nat,before:2nat,after:3nat,symbol:3nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:43nat,next:52nat,before:3nat,after:4nat,symbol:151nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:52nat,next:60nat,before:4nat,after:5nat,symbol:115nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:60nat,next:69nat,before:5nat,after:6nat,symbol:238nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:69nat,next:78nat,before:6nat,after:7nat,symbol:200nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:78nat,next:86nat,before:7nat,after:8nat,symbol:122nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:86nat,next:95nat,before:8nat,after:9nat,symbol:169nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:95nat,next:103nat,before:9nat,after:10nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:103nat,next:111nat,before:10nat,after:11nat,symbol:4nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:111nat,next:119nat,before:11nat,after:12nat,symbol:125nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:119nat,next:127nat,before:12nat,after:13nat,symbol:110nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:127nat,next:135nat,before:13nat,after:14nat,symbol:42nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:135nat,next:144nat,before:14nat,after:15nat,symbol:215nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:144nat,next:153nat,before:15nat,after:16nat,symbol:217nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:153nat,next:162nat,before:16nat,after:17nat,symbol:203nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:162nat,next:170nat,before:17nat,after:18nat,symbol:39nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:170nat,next:177nat,before:18nat,after:18nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_interlace_ct6_d16_2x1()
    ensures t::trace(z_interlace_ct6_d16_2x1(),raw_interlace_ct6_d16_2x1(),ts_interlace_ct6_d16_2x1(),0,19nat),
{
    hide(z_interlace_ct6_d16_2x1);hide(raw_interlace_ct6_d16_2x1);hide(ts_interlace_ct6_d16_2x1);hide(t::valid_step);
    assert(t::trace(z_interlace_ct6_d16_2x1(),raw_interlace_ct6_d16_2x1(),ts_interlace_ct6_d16_2x1(),0,19nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_interlace_ct6_d16_2x1()
    ensures raw_interlace_ct6_d16_2x1().len()==18nat,
        z_interlace_ct6_d16_2x1().len()==27nat,
        z_interlace_ct6_d16_2x1()[0]==120u8,
        z_interlace_ct6_d16_2x1()[1]==156u8,
        zm::field_value(z_interlace_ct6_d16_2x1(),16,1)==1nat,
        zm::field_value(z_interlace_ct6_d16_2x1(),17,2)==1nat,
        zm::be_value(z_interlace_ct6_d16_2x1(),23nat,4)==1154746358nat,
        ts_interlace_ct6_d16_2x1().len()==19nat,
        ts_interlace_ct6_d16_2x1()[0].pos==19nat,
        ts_interlace_ct6_d16_2x1()[0].before==0nat,
        ts_interlace_ct6_d16_2x1().last().next==177nat,
{
    hide(z_interlace_ct6_d16_2x1);hide(raw_interlace_ct6_d16_2x1);hide(ts_interlace_ct6_d16_2x1);
    assert(raw_interlace_ct6_d16_2x1().len()==18nat) by (compute);
    assert(z_interlace_ct6_d16_2x1().len()==27nat) by (compute);
    assert(z_interlace_ct6_d16_2x1()[0]==120u8) by (compute);
    assert(z_interlace_ct6_d16_2x1()[1]==156u8) by (compute);
    assert(zm::field_value(z_interlace_ct6_d16_2x1(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_interlace_ct6_d16_2x1(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_interlace_ct6_d16_2x1(),23nat,4)==1154746358nat) by (compute);
    assert(ts_interlace_ct6_d16_2x1().len()==19nat) by (compute);
    assert(ts_interlace_ct6_d16_2x1()[0].pos==19nat) by (compute);
    assert(ts_interlace_ct6_d16_2x1()[0].before==0nat) by (compute);
    assert(ts_interlace_ct6_d16_2x1().last().next==177nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_interlace_ct6_d16_2x1()
    ensures zm::Adler(raw_interlace_ct6_d16_2x1())==1154746358nat,
{
    hide(raw_interlace_ct6_d16_2x1);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_interlace_ct6_d16_2x1().len()==18nat) by (compute);
    assert(c::sums(raw_interlace_ct6_d16_2x1(),0,18nat)==(2037nat,17620nat)) by (compute);
    c::sums_correct(raw_interlace_ct6_d16_2x1(),0,18nat);
    assert(raw_interlace_ct6_d16_2x1().subrange(0,18) =~= raw_interlace_ct6_d16_2x1());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((17620nat%65521)*(u16::MAX as nat+1)+(1+2037nat)%65521==1154746358nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_interlace_ct6_d16_2x1()
    ensures r::inflated(z_interlace_ct6_d16_2x1(),a_interlace_ct6_d16_2x1(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_interlace_ct6_d16_2x1(),r2:false}),
        raw_interlace_ct6_d16_2x1().len()==18nat,
{
    hide(z_interlace_ct6_d16_2x1);hide(raw_interlace_ct6_d16_2x1);hide(ts_interlace_ct6_d16_2x1);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_interlace_ct6_d16_2x1();checksum_interlace_ct6_d16_2x1();
    assert(r::raw(a_interlace_ct6_d16_2x1())==18nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_interlace_ct6_d16_2x1();
    stream::fixed_stream(z_interlace_ct6_d16_2x1(),raw_interlace_ct6_d16_2x1(),ts_interlace_ct6_d16_2x1(),18nat,156nat,1154746358nat);
}
#[verifier::spinoff_prover]
proof fn scan_interlace_ct6_d16_2x1()
    ensures r::scan(raw_interlace_ct6_d16_2x1(),a_interlace_ct6_d16_2x1(),0)==Ok(()),
{
    hide(raw_interlace_ct6_d16_2x1);
    assert(r::count(a_interlace_ct6_d16_2x1())==7nat) by (compute);
    r::scan_step(raw_interlace_ct6_d16_2x1(),a_interlace_ct6_d16_2x1(),7);
    assert(r::pw(a_interlace_ct6_d16_2x1(),6nat)==2nat && r::ph(a_interlace_ct6_d16_2x1(),6nat)==0nat && r::rb(a_interlace_ct6_d16_2x1(),6nat)==16nat && r::base(a_interlace_ct6_d16_2x1(),6nat)==18nat) by (compute);
    r::pass_step(raw_interlace_ct6_d16_2x1(),a_interlace_ct6_d16_2x1(),6nat,0);
    r::scan_step(raw_interlace_ct6_d16_2x1(),a_interlace_ct6_d16_2x1(),6nat);
    assert(r::pw(a_interlace_ct6_d16_2x1(),5nat)==1nat && r::ph(a_interlace_ct6_d16_2x1(),5nat)==1nat && r::rb(a_interlace_ct6_d16_2x1(),5nat)==8nat && r::base(a_interlace_ct6_d16_2x1(),5nat)==9nat) by (compute);
    assert(raw_interlace_ct6_d16_2x1()[9]==1u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_2x1(),a_interlace_ct6_d16_2x1(),5nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_2x1(),a_interlace_ct6_d16_2x1(),5nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_2x1(),a_interlace_ct6_d16_2x1(),5nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_2x1(),a_interlace_ct6_d16_2x1(),5nat);
    assert(r::pw(a_interlace_ct6_d16_2x1(),4nat)==1nat && r::ph(a_interlace_ct6_d16_2x1(),4nat)==0nat && r::rb(a_interlace_ct6_d16_2x1(),4nat)==8nat && r::base(a_interlace_ct6_d16_2x1(),4nat)==9nat) by (compute);
    r::pass_step(raw_interlace_ct6_d16_2x1(),a_interlace_ct6_d16_2x1(),4nat,0);
    r::scan_step(raw_interlace_ct6_d16_2x1(),a_interlace_ct6_d16_2x1(),4nat);
    assert(r::pw(a_interlace_ct6_d16_2x1(),3nat)==0nat && r::ph(a_interlace_ct6_d16_2x1(),3nat)==1nat && r::rb(a_interlace_ct6_d16_2x1(),3nat)==0nat && r::base(a_interlace_ct6_d16_2x1(),3nat)==9nat) by (compute);
    r::pass_step(raw_interlace_ct6_d16_2x1(),a_interlace_ct6_d16_2x1(),3nat,0);
    r::scan_step(raw_interlace_ct6_d16_2x1(),a_interlace_ct6_d16_2x1(),3nat);
    assert(r::pw(a_interlace_ct6_d16_2x1(),2nat)==1nat && r::ph(a_interlace_ct6_d16_2x1(),2nat)==0nat && r::rb(a_interlace_ct6_d16_2x1(),2nat)==8nat && r::base(a_interlace_ct6_d16_2x1(),2nat)==9nat) by (compute);
    r::pass_step(raw_interlace_ct6_d16_2x1(),a_interlace_ct6_d16_2x1(),2nat,0);
    r::scan_step(raw_interlace_ct6_d16_2x1(),a_interlace_ct6_d16_2x1(),2nat);
    assert(r::pw(a_interlace_ct6_d16_2x1(),1nat)==0nat && r::ph(a_interlace_ct6_d16_2x1(),1nat)==1nat && r::rb(a_interlace_ct6_d16_2x1(),1nat)==0nat && r::base(a_interlace_ct6_d16_2x1(),1nat)==9nat) by (compute);
    r::pass_step(raw_interlace_ct6_d16_2x1(),a_interlace_ct6_d16_2x1(),1nat,0);
    r::scan_step(raw_interlace_ct6_d16_2x1(),a_interlace_ct6_d16_2x1(),1nat);
    assert(r::pw(a_interlace_ct6_d16_2x1(),0nat)==1nat && r::ph(a_interlace_ct6_d16_2x1(),0nat)==1nat && r::rb(a_interlace_ct6_d16_2x1(),0nat)==8nat && r::base(a_interlace_ct6_d16_2x1(),0nat)==0nat) by (compute);
    assert(raw_interlace_ct6_d16_2x1()[0]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_2x1(),a_interlace_ct6_d16_2x1(),0nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_2x1(),a_interlace_ct6_d16_2x1(),0nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_2x1(),a_interlace_ct6_d16_2x1(),0nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_2x1(),a_interlace_ct6_d16_2x1(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_interlace_ct6_d16_2x1()
    ensures r::still(bytes_interlace_ct6_d16_2x1(),STRICT,false)==(r::Projection::Ready {width:2u32,height:1u32,bytes:8nat}),
{
    hide(bytes_interlace_ct6_d16_2x1);hide(z_interlace_ct6_d16_2x1);hide(raw_interlace_ct6_d16_2x1);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_interlace_ct6_d16_2x1().len()==84nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct6_d16_2x1())==Ok((crate::model::Ihdr {width:2u32,height:1u32,depth:16u8,color_type:6u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct6_d16_2x1())==Ok((crate::model::Ihdr {width:2u32,height:1u32,depth:16u8,color_type:6u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct6_d16_2x1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:27nat,next:72nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct6_d16_2x1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:27nat,next:72nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct6_d16_2x1(),72nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:80nat,len:0nat,next:84nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct6_d16_2x1(),72nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:80nat,len:0nat,next:84nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct6_d16_2x1().subrange(41,68)==z_interlace_ct6_d16_2x1()) by (compute);
    walk_interlace_ct6_d16_2x1(bytes_interlace_ct6_d16_2x1());
    inflate_interlace_ct6_d16_2x1();scan_interlace_ct6_d16_2x1();
    assert(r::raw(a_interlace_ct6_d16_2x1())==18nat) by (compute);
    assert(r::data(z_interlace_ct6_d16_2x1(),a_interlace_ct6_d16_2x1(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_interlace_ct6_d16_2x1(),h_interlace_ct6_d16_2x1(),z_interlace_ct6_d16_2x1(),0nat,false);
}
fn public_interlace_ct6_d16_2x1() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,1u8,16u8,6u8,0u8,0u8,1u8,211u8,181u8,147u8,95u8,0u8,0u8,0u8,27u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,8u8,102u8,158u8,94u8,252u8,238u8,68u8,213u8,74u8,70u8,150u8,218u8,60u8,173u8,235u8,55u8,79u8,171u8,3u8,0u8,68u8,212u8,7u8,246u8,210u8,201u8,17u8,122u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_interlace_ct6_d16_2x1());model_interlace_ct6_d16_2x1();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==2u32 && s.height==1u32 && s.rgba@.len()==8nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// interlace/ct3-d4-2x1; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_interlace_ct3_d4_2x1() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,1u8,4u8,3u8,0u8,0u8,1u8,113u8,11u8,82u8,47u8,0u8,0u8,0u8,48u8,80u8,76u8,84u8,69u8,89u8,196u8,227u8,155u8,87u8,162u8,121u8,246u8,210u8,86u8,127u8,185u8,92u8,199u8,181u8,14u8,145u8,216u8,190u8,196u8,167u8,144u8,238u8,26u8,246u8,17u8,167u8,145u8,35u8,138u8,66u8,27u8,249u8,24u8,58u8,27u8,53u8,70u8,220u8,235u8,135u8,120u8,6u8,211u8,79u8,87u8,184u8,23u8,22u8,106u8,116u8,123u8,0u8,0u8,0u8,12u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,120u8,192u8,152u8,0u8,0u8,3u8,6u8,1u8,66u8,251u8,44u8,17u8,229u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_interlace_ct3_d4_2x1() -> Seq<u8> {seq![120u8,156u8,99u8,120u8,192u8,152u8,0u8,0u8,3u8,6u8,1u8,66u8]}
spec fn h_interlace_ct3_d4_2x1() -> crate::model::Ihdr {crate::model::Ihdr {width:2u32,height:1u32,depth:4u8,color_type:3u8,interlace:1u8}}
spec fn a_interlace_ct3_d4_2x1() -> r::Raster {r::shape(h_interlace_ct3_d4_2x1(),2nat,1nat,16nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_interlace_ct3_d4_2x1(b: Seq<u8>)
    requires b.len()==129,crate::model::mh(b)==Ok((crate::model::Ihdr {width:2u32,height:1u32,depth:4u8,color_type:3u8,interlace:1u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:12nat,next:117nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,117nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:125nat,len:0nat,next:129nat,crc_ok:true,synthetic:false})),
        b.subrange(101,113)==z_interlace_ct3_d4_2x1(),
    ensures r::default_case(b,crate::model::Ihdr {width:2u32,height:1u32,depth:4u8,color_type:3u8,interlace:1u8},z_interlace_ct3_d4_2x1(),16nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);
    hide(z_interlace_ct3_d4_2x1);
    r::palette_default_case(b,crate::model::Ihdr {width:2u32,height:1u32,depth:4u8,color_type:3u8,interlace:1u8},z_interlace_ct3_d4_2x1(),12nat);
}
spec fn raw_interlace_ct3_d4_2x1() -> Seq<u8> {seq![0u8,224u8,1u8,96u8]}
spec fn ts_interlace_ct3_d4_2x1() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:36nat,before:1nat,after:2nat,symbol:224nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:36nat,next:44nat,before:2nat,after:3nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:44nat,next:52nat,before:3nat,after:4nat,symbol:96nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:52nat,next:59nat,before:4nat,after:4nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_interlace_ct3_d4_2x1()
    ensures t::trace(z_interlace_ct3_d4_2x1(),raw_interlace_ct3_d4_2x1(),ts_interlace_ct3_d4_2x1(),0,5nat),
{
    hide(z_interlace_ct3_d4_2x1);hide(raw_interlace_ct3_d4_2x1);hide(ts_interlace_ct3_d4_2x1);hide(t::valid_step);
    assert(t::trace(z_interlace_ct3_d4_2x1(),raw_interlace_ct3_d4_2x1(),ts_interlace_ct3_d4_2x1(),0,5nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_interlace_ct3_d4_2x1()
    ensures raw_interlace_ct3_d4_2x1().len()==4nat,
        z_interlace_ct3_d4_2x1().len()==12nat,
        z_interlace_ct3_d4_2x1()[0]==120u8,
        z_interlace_ct3_d4_2x1()[1]==156u8,
        zm::field_value(z_interlace_ct3_d4_2x1(),16,1)==1nat,
        zm::field_value(z_interlace_ct3_d4_2x1(),17,2)==1nat,
        zm::be_value(z_interlace_ct3_d4_2x1(),8nat,4)==50725186nat,
        ts_interlace_ct3_d4_2x1().len()==5nat,
        ts_interlace_ct3_d4_2x1()[0].pos==19nat,
        ts_interlace_ct3_d4_2x1()[0].before==0nat,
        ts_interlace_ct3_d4_2x1().last().next==59nat,
{
    hide(z_interlace_ct3_d4_2x1);hide(raw_interlace_ct3_d4_2x1);hide(ts_interlace_ct3_d4_2x1);
    assert(raw_interlace_ct3_d4_2x1().len()==4nat) by (compute);
    assert(z_interlace_ct3_d4_2x1().len()==12nat) by (compute);
    assert(z_interlace_ct3_d4_2x1()[0]==120u8) by (compute);
    assert(z_interlace_ct3_d4_2x1()[1]==156u8) by (compute);
    assert(zm::field_value(z_interlace_ct3_d4_2x1(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_interlace_ct3_d4_2x1(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_interlace_ct3_d4_2x1(),8nat,4)==50725186nat) by (compute);
    assert(ts_interlace_ct3_d4_2x1().len()==5nat) by (compute);
    assert(ts_interlace_ct3_d4_2x1()[0].pos==19nat) by (compute);
    assert(ts_interlace_ct3_d4_2x1()[0].before==0nat) by (compute);
    assert(ts_interlace_ct3_d4_2x1().last().next==59nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_interlace_ct3_d4_2x1()
    ensures zm::Adler(raw_interlace_ct3_d4_2x1())==50725186nat,
{
    hide(raw_interlace_ct3_d4_2x1);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_interlace_ct3_d4_2x1().len()==4nat) by (compute);
    assert(c::sums(raw_interlace_ct3_d4_2x1(),0,4nat)==(321nat,774nat)) by (compute);
    c::sums_correct(raw_interlace_ct3_d4_2x1(),0,4nat);
    assert(raw_interlace_ct3_d4_2x1().subrange(0,4) =~= raw_interlace_ct3_d4_2x1());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((774nat%65521)*(u16::MAX as nat+1)+(1+321nat)%65521==50725186nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_interlace_ct3_d4_2x1()
    ensures r::inflated(z_interlace_ct3_d4_2x1(),a_interlace_ct3_d4_2x1(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_interlace_ct3_d4_2x1(),r2:false}),
        raw_interlace_ct3_d4_2x1().len()==4nat,
{
    hide(z_interlace_ct3_d4_2x1);hide(raw_interlace_ct3_d4_2x1);hide(ts_interlace_ct3_d4_2x1);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_interlace_ct3_d4_2x1();checksum_interlace_ct3_d4_2x1();
    assert(r::raw(a_interlace_ct3_d4_2x1())==4nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_interlace_ct3_d4_2x1();
    stream::fixed_stream(z_interlace_ct3_d4_2x1(),raw_interlace_ct3_d4_2x1(),ts_interlace_ct3_d4_2x1(),4nat,156nat,50725186nat);
}
#[verifier::spinoff_prover]
proof fn scan_interlace_ct3_d4_2x1()
    ensures r::scan(raw_interlace_ct3_d4_2x1(),a_interlace_ct3_d4_2x1(),0)==Ok(()),
{
    hide(raw_interlace_ct3_d4_2x1);
    assert(r::count(a_interlace_ct3_d4_2x1())==7nat) by (compute);
    r::scan_step(raw_interlace_ct3_d4_2x1(),a_interlace_ct3_d4_2x1(),7);
    assert(r::pw(a_interlace_ct3_d4_2x1(),6nat)==2nat && r::ph(a_interlace_ct3_d4_2x1(),6nat)==0nat && r::rb(a_interlace_ct3_d4_2x1(),6nat)==1nat && r::base(a_interlace_ct3_d4_2x1(),6nat)==4nat) by (compute);
    r::pass_step(raw_interlace_ct3_d4_2x1(),a_interlace_ct3_d4_2x1(),6nat,0);
    r::scan_step(raw_interlace_ct3_d4_2x1(),a_interlace_ct3_d4_2x1(),6nat);
    assert(r::pw(a_interlace_ct3_d4_2x1(),5nat)==1nat && r::ph(a_interlace_ct3_d4_2x1(),5nat)==1nat && r::rb(a_interlace_ct3_d4_2x1(),5nat)==1nat && r::base(a_interlace_ct3_d4_2x1(),5nat)==2nat) by (compute);
    assert(raw_interlace_ct3_d4_2x1()[2]==1u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_2x1(),a_interlace_ct3_d4_2x1(),5nat,0nat),1nat);
    assert(r::row_result(raw_interlace_ct3_d4_2x1(),a_interlace_ct3_d4_2x1(),5nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_2x1(),a_interlace_ct3_d4_2x1(),5nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_2x1(),a_interlace_ct3_d4_2x1(),5nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_2x1(),a_interlace_ct3_d4_2x1(),5nat);
    assert(r::pw(a_interlace_ct3_d4_2x1(),4nat)==1nat && r::ph(a_interlace_ct3_d4_2x1(),4nat)==0nat && r::rb(a_interlace_ct3_d4_2x1(),4nat)==1nat && r::base(a_interlace_ct3_d4_2x1(),4nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct3_d4_2x1(),a_interlace_ct3_d4_2x1(),4nat,0);
    r::scan_step(raw_interlace_ct3_d4_2x1(),a_interlace_ct3_d4_2x1(),4nat);
    assert(r::pw(a_interlace_ct3_d4_2x1(),3nat)==0nat && r::ph(a_interlace_ct3_d4_2x1(),3nat)==1nat && r::rb(a_interlace_ct3_d4_2x1(),3nat)==0nat && r::base(a_interlace_ct3_d4_2x1(),3nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct3_d4_2x1(),a_interlace_ct3_d4_2x1(),3nat,0);
    r::scan_step(raw_interlace_ct3_d4_2x1(),a_interlace_ct3_d4_2x1(),3nat);
    assert(r::pw(a_interlace_ct3_d4_2x1(),2nat)==1nat && r::ph(a_interlace_ct3_d4_2x1(),2nat)==0nat && r::rb(a_interlace_ct3_d4_2x1(),2nat)==1nat && r::base(a_interlace_ct3_d4_2x1(),2nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct3_d4_2x1(),a_interlace_ct3_d4_2x1(),2nat,0);
    r::scan_step(raw_interlace_ct3_d4_2x1(),a_interlace_ct3_d4_2x1(),2nat);
    assert(r::pw(a_interlace_ct3_d4_2x1(),1nat)==0nat && r::ph(a_interlace_ct3_d4_2x1(),1nat)==1nat && r::rb(a_interlace_ct3_d4_2x1(),1nat)==0nat && r::base(a_interlace_ct3_d4_2x1(),1nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct3_d4_2x1(),a_interlace_ct3_d4_2x1(),1nat,0);
    r::scan_step(raw_interlace_ct3_d4_2x1(),a_interlace_ct3_d4_2x1(),1nat);
    assert(r::pw(a_interlace_ct3_d4_2x1(),0nat)==1nat && r::ph(a_interlace_ct3_d4_2x1(),0nat)==1nat && r::rb(a_interlace_ct3_d4_2x1(),0nat)==1nat && r::base(a_interlace_ct3_d4_2x1(),0nat)==0nat) by (compute);
    assert(raw_interlace_ct3_d4_2x1()[0]==0u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_2x1(),a_interlace_ct3_d4_2x1(),0nat,0nat),1nat);
    assert(r::row_result(raw_interlace_ct3_d4_2x1(),a_interlace_ct3_d4_2x1(),0nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_2x1(),a_interlace_ct3_d4_2x1(),0nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_2x1(),a_interlace_ct3_d4_2x1(),0nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_2x1(),a_interlace_ct3_d4_2x1(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_interlace_ct3_d4_2x1()
    ensures r::still(bytes_interlace_ct3_d4_2x1(),STRICT,false)==(r::Projection::Ready {width:2u32,height:1u32,bytes:8nat}),
{
    hide(bytes_interlace_ct3_d4_2x1);hide(z_interlace_ct3_d4_2x1);hide(raw_interlace_ct3_d4_2x1);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);
    assert(bytes_interlace_ct3_d4_2x1().len()==129nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct3_d4_2x1())==Ok((crate::model::Ihdr {width:2u32,height:1u32,depth:4u8,color_type:3u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct3_d4_2x1())==Ok((crate::model::Ihdr {width:2u32,height:1u32,depth:4u8,color_type:3u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 4u8)==16u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_2x1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_2x1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(48nat,3u8,4u8,false,false,false,false)==crate::metadata_model::Decision::Keep(16nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_2x1(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:12nat,next:117nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_2x1(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:12nat,next:117nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_2x1(),117nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:125nat,len:0nat,next:129nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_2x1(),117nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:125nat,len:0nat,next:129nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct3_d4_2x1().subrange(101,113)==z_interlace_ct3_d4_2x1()) by (compute);
    walk_interlace_ct3_d4_2x1(bytes_interlace_ct3_d4_2x1());
    inflate_interlace_ct3_d4_2x1();scan_interlace_ct3_d4_2x1();
    assert(r::raw(a_interlace_ct3_d4_2x1())==4nat) by (compute);
    assert(r::data(z_interlace_ct3_d4_2x1(),a_interlace_ct3_d4_2x1(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_interlace_ct3_d4_2x1(),h_interlace_ct3_d4_2x1(),z_interlace_ct3_d4_2x1(),16nat,false);
}
fn public_interlace_ct3_d4_2x1() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,1u8,4u8,3u8,0u8,0u8,1u8,113u8,11u8,82u8,47u8,0u8,0u8,0u8,48u8,80u8,76u8,84u8,69u8,89u8,196u8,227u8,155u8,87u8,162u8,121u8,246u8,210u8,86u8,127u8,185u8,92u8,199u8,181u8,14u8,145u8,216u8,190u8,196u8,167u8,144u8,238u8,26u8,246u8,17u8,167u8,145u8,35u8,138u8,66u8,27u8,249u8,24u8,58u8,27u8,53u8,70u8,220u8,235u8,135u8,120u8,6u8,211u8,79u8,87u8,184u8,23u8,22u8,106u8,116u8,123u8,0u8,0u8,0u8,12u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,120u8,192u8,152u8,0u8,0u8,3u8,6u8,1u8,66u8,251u8,44u8,17u8,229u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_interlace_ct3_d4_2x1());model_interlace_ct3_d4_2x1();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==2u32 && s.height==1u32 && s.rgba@.len()==8nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// interlace/ct0-d1-1x5; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_interlace_ct0_d1_1x5() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,5u8,1u8,0u8,0u8,0u8,1u8,219u8,248u8,139u8,164u8,0u8,0u8,0u8,18u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,96u8,108u8,96u8,98u8,96u8,102u8,96u8,105u8,0u8,0u8,4u8,50u8,1u8,11u8,87u8,131u8,16u8,229u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_interlace_ct0_d1_1x5() -> Seq<u8> {seq![120u8,156u8,99u8,96u8,96u8,108u8,96u8,98u8,96u8,102u8,96u8,105u8,0u8,0u8,4u8,50u8,1u8,11u8]}
spec fn h_interlace_ct0_d1_1x5() -> crate::model::Ihdr {crate::model::Ihdr {width:1u32,height:5u32,depth:1u8,color_type:0u8,interlace:1u8}}
spec fn a_interlace_ct0_d1_1x5() -> r::Raster {r::shape(h_interlace_ct0_d1_1x5(),1nat,5nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_interlace_ct0_d1_1x5(b: Seq<u8>)
    requires b.len()==75,crate::model::mh(b)==Ok((crate::model::Ihdr {width:1u32,height:5u32,depth:1u8,color_type:0u8,interlace:1u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:18nat,next:63nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,63nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:71nat,len:0nat,next:75nat,crc_ok:true,synthetic:false})),
        b.subrange(41,59)==z_interlace_ct0_d1_1x5(),
    ensures r::default_case(b,crate::model::Ihdr {width:1u32,height:5u32,depth:1u8,color_type:0u8,interlace:1u8},z_interlace_ct0_d1_1x5(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    hide(z_interlace_ct0_d1_1x5);
        assert((1u32 << 1u8)==2u32) by (compute);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_interlace_ct0_d1_1x5() -> Seq<u8> {seq![0u8,0u8,1u8,128u8,2u8,0u8,3u8,0u8,4u8,128u8]}
spec fn ts_interlace_ct0_d1_1x5() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:35nat,before:1nat,after:2nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:35nat,next:43nat,before:2nat,after:3nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:43nat,next:51nat,before:3nat,after:4nat,symbol:128nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:51nat,next:59nat,before:4nat,after:5nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:59nat,next:67nat,before:5nat,after:6nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:67nat,next:75nat,before:6nat,after:7nat,symbol:3nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:75nat,next:83nat,before:7nat,after:8nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:83nat,next:91nat,before:8nat,after:9nat,symbol:4nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:91nat,next:99nat,before:9nat,after:10nat,symbol:128nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:99nat,next:106nat,before:10nat,after:10nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_interlace_ct0_d1_1x5()
    ensures t::trace(z_interlace_ct0_d1_1x5(),raw_interlace_ct0_d1_1x5(),ts_interlace_ct0_d1_1x5(),0,11nat),
{
    hide(z_interlace_ct0_d1_1x5);hide(raw_interlace_ct0_d1_1x5);hide(ts_interlace_ct0_d1_1x5);hide(t::valid_step);
    assert(t::trace(z_interlace_ct0_d1_1x5(),raw_interlace_ct0_d1_1x5(),ts_interlace_ct0_d1_1x5(),0,11nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_interlace_ct0_d1_1x5()
    ensures raw_interlace_ct0_d1_1x5().len()==10nat,
        z_interlace_ct0_d1_1x5().len()==18nat,
        z_interlace_ct0_d1_1x5()[0]==120u8,
        z_interlace_ct0_d1_1x5()[1]==156u8,
        zm::field_value(z_interlace_ct0_d1_1x5(),16,1)==1nat,
        zm::field_value(z_interlace_ct0_d1_1x5(),17,2)==1nat,
        zm::be_value(z_interlace_ct0_d1_1x5(),14nat,4)==70385931nat,
        ts_interlace_ct0_d1_1x5().len()==11nat,
        ts_interlace_ct0_d1_1x5()[0].pos==19nat,
        ts_interlace_ct0_d1_1x5()[0].before==0nat,
        ts_interlace_ct0_d1_1x5().last().next==106nat,
{
    hide(z_interlace_ct0_d1_1x5);hide(raw_interlace_ct0_d1_1x5);hide(ts_interlace_ct0_d1_1x5);
    assert(raw_interlace_ct0_d1_1x5().len()==10nat) by (compute);
    assert(z_interlace_ct0_d1_1x5().len()==18nat) by (compute);
    assert(z_interlace_ct0_d1_1x5()[0]==120u8) by (compute);
    assert(z_interlace_ct0_d1_1x5()[1]==156u8) by (compute);
    assert(zm::field_value(z_interlace_ct0_d1_1x5(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_interlace_ct0_d1_1x5(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_interlace_ct0_d1_1x5(),14nat,4)==70385931nat) by (compute);
    assert(ts_interlace_ct0_d1_1x5().len()==11nat) by (compute);
    assert(ts_interlace_ct0_d1_1x5()[0].pos==19nat) by (compute);
    assert(ts_interlace_ct0_d1_1x5()[0].before==0nat) by (compute);
    assert(ts_interlace_ct0_d1_1x5().last().next==106nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_interlace_ct0_d1_1x5()
    ensures zm::Adler(raw_interlace_ct0_d1_1x5())==70385931nat,
{
    hide(raw_interlace_ct0_d1_1x5);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_interlace_ct0_d1_1x5().len()==10nat) by (compute);
    assert(c::sums(raw_interlace_ct0_d1_1x5(),0,10nat)==(266nat,1074nat)) by (compute);
    c::sums_correct(raw_interlace_ct0_d1_1x5(),0,10nat);
    assert(raw_interlace_ct0_d1_1x5().subrange(0,10) =~= raw_interlace_ct0_d1_1x5());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((1074nat%65521)*(u16::MAX as nat+1)+(1+266nat)%65521==70385931nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_interlace_ct0_d1_1x5()
    ensures r::inflated(z_interlace_ct0_d1_1x5(),a_interlace_ct0_d1_1x5(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_interlace_ct0_d1_1x5(),r2:false}),
        raw_interlace_ct0_d1_1x5().len()==10nat,
{
    hide(z_interlace_ct0_d1_1x5);hide(raw_interlace_ct0_d1_1x5);hide(ts_interlace_ct0_d1_1x5);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_interlace_ct0_d1_1x5();checksum_interlace_ct0_d1_1x5();
    assert(r::raw(a_interlace_ct0_d1_1x5())==10nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_interlace_ct0_d1_1x5();
    stream::fixed_stream(z_interlace_ct0_d1_1x5(),raw_interlace_ct0_d1_1x5(),ts_interlace_ct0_d1_1x5(),10nat,156nat,70385931nat);
}
#[verifier::spinoff_prover]
proof fn scan_interlace_ct0_d1_1x5()
    ensures r::scan(raw_interlace_ct0_d1_1x5(),a_interlace_ct0_d1_1x5(),0)==Ok(()),
{
    hide(raw_interlace_ct0_d1_1x5);
    assert(r::count(a_interlace_ct0_d1_1x5())==7nat) by (compute);
    r::scan_step(raw_interlace_ct0_d1_1x5(),a_interlace_ct0_d1_1x5(),7);
    assert(r::pw(a_interlace_ct0_d1_1x5(),6nat)==1nat && r::ph(a_interlace_ct0_d1_1x5(),6nat)==2nat && r::rb(a_interlace_ct0_d1_1x5(),6nat)==1nat && r::base(a_interlace_ct0_d1_1x5(),6nat)==6nat) by (compute);
    assert(raw_interlace_ct0_d1_1x5()[6]==3u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_1x5(),a_interlace_ct0_d1_1x5(),6nat,0nat)==Ok(()));
    assert(raw_interlace_ct0_d1_1x5()[8]==4u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_1x5(),a_interlace_ct0_d1_1x5(),6nat,1nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_1x5(),a_interlace_ct0_d1_1x5(),6nat,2nat);
    r::pass_step(raw_interlace_ct0_d1_1x5(),a_interlace_ct0_d1_1x5(),6nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_1x5(),a_interlace_ct0_d1_1x5(),6nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_1x5(),a_interlace_ct0_d1_1x5(),6nat);
    assert(r::pw(a_interlace_ct0_d1_1x5(),5nat)==0nat && r::ph(a_interlace_ct0_d1_1x5(),5nat)==3nat && r::rb(a_interlace_ct0_d1_1x5(),5nat)==0nat && r::base(a_interlace_ct0_d1_1x5(),5nat)==6nat) by (compute);
    r::pass_step(raw_interlace_ct0_d1_1x5(),a_interlace_ct0_d1_1x5(),5nat,0);
    r::scan_step(raw_interlace_ct0_d1_1x5(),a_interlace_ct0_d1_1x5(),5nat);
    assert(r::pw(a_interlace_ct0_d1_1x5(),4nat)==1nat && r::ph(a_interlace_ct0_d1_1x5(),4nat)==1nat && r::rb(a_interlace_ct0_d1_1x5(),4nat)==1nat && r::base(a_interlace_ct0_d1_1x5(),4nat)==4nat) by (compute);
    assert(raw_interlace_ct0_d1_1x5()[4]==2u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_1x5(),a_interlace_ct0_d1_1x5(),4nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_1x5(),a_interlace_ct0_d1_1x5(),4nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_1x5(),a_interlace_ct0_d1_1x5(),4nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_1x5(),a_interlace_ct0_d1_1x5(),4nat);
    assert(r::pw(a_interlace_ct0_d1_1x5(),3nat)==0nat && r::ph(a_interlace_ct0_d1_1x5(),3nat)==2nat && r::rb(a_interlace_ct0_d1_1x5(),3nat)==0nat && r::base(a_interlace_ct0_d1_1x5(),3nat)==4nat) by (compute);
    r::pass_step(raw_interlace_ct0_d1_1x5(),a_interlace_ct0_d1_1x5(),3nat,0);
    r::scan_step(raw_interlace_ct0_d1_1x5(),a_interlace_ct0_d1_1x5(),3nat);
    assert(r::pw(a_interlace_ct0_d1_1x5(),2nat)==1nat && r::ph(a_interlace_ct0_d1_1x5(),2nat)==1nat && r::rb(a_interlace_ct0_d1_1x5(),2nat)==1nat && r::base(a_interlace_ct0_d1_1x5(),2nat)==2nat) by (compute);
    assert(raw_interlace_ct0_d1_1x5()[2]==1u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_1x5(),a_interlace_ct0_d1_1x5(),2nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_1x5(),a_interlace_ct0_d1_1x5(),2nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_1x5(),a_interlace_ct0_d1_1x5(),2nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_1x5(),a_interlace_ct0_d1_1x5(),2nat);
    assert(r::pw(a_interlace_ct0_d1_1x5(),1nat)==0nat && r::ph(a_interlace_ct0_d1_1x5(),1nat)==1nat && r::rb(a_interlace_ct0_d1_1x5(),1nat)==0nat && r::base(a_interlace_ct0_d1_1x5(),1nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct0_d1_1x5(),a_interlace_ct0_d1_1x5(),1nat,0);
    r::scan_step(raw_interlace_ct0_d1_1x5(),a_interlace_ct0_d1_1x5(),1nat);
    assert(r::pw(a_interlace_ct0_d1_1x5(),0nat)==1nat && r::ph(a_interlace_ct0_d1_1x5(),0nat)==1nat && r::rb(a_interlace_ct0_d1_1x5(),0nat)==1nat && r::base(a_interlace_ct0_d1_1x5(),0nat)==0nat) by (compute);
    assert(raw_interlace_ct0_d1_1x5()[0]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_1x5(),a_interlace_ct0_d1_1x5(),0nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_1x5(),a_interlace_ct0_d1_1x5(),0nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_1x5(),a_interlace_ct0_d1_1x5(),0nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_1x5(),a_interlace_ct0_d1_1x5(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_interlace_ct0_d1_1x5()
    ensures r::still(bytes_interlace_ct0_d1_1x5(),STRICT,false)==(r::Projection::Ready {width:1u32,height:5u32,bytes:20nat}),
{
    hide(bytes_interlace_ct0_d1_1x5);hide(z_interlace_ct0_d1_1x5);hide(raw_interlace_ct0_d1_1x5);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_interlace_ct0_d1_1x5().len()==75nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct0_d1_1x5())==Ok((crate::model::Ihdr {width:1u32,height:5u32,depth:1u8,color_type:0u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct0_d1_1x5())==Ok((crate::model::Ihdr {width:1u32,height:5u32,depth:1u8,color_type:0u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 1u8)==2u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct0_d1_1x5(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:18nat,next:63nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct0_d1_1x5(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:18nat,next:63nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct0_d1_1x5(),63nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:71nat,len:0nat,next:75nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct0_d1_1x5(),63nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:71nat,len:0nat,next:75nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct0_d1_1x5().subrange(41,59)==z_interlace_ct0_d1_1x5()) by (compute);
    walk_interlace_ct0_d1_1x5(bytes_interlace_ct0_d1_1x5());
    inflate_interlace_ct0_d1_1x5();scan_interlace_ct0_d1_1x5();
    assert(r::raw(a_interlace_ct0_d1_1x5())==10nat) by (compute);
    assert(r::data(z_interlace_ct0_d1_1x5(),a_interlace_ct0_d1_1x5(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_interlace_ct0_d1_1x5(),h_interlace_ct0_d1_1x5(),z_interlace_ct0_d1_1x5(),0nat,false);
}
fn public_interlace_ct0_d1_1x5() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,5u8,1u8,0u8,0u8,0u8,1u8,219u8,248u8,139u8,164u8,0u8,0u8,0u8,18u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,96u8,108u8,96u8,98u8,96u8,102u8,96u8,105u8,0u8,0u8,4u8,50u8,1u8,11u8,87u8,131u8,16u8,229u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_interlace_ct0_d1_1x5());model_interlace_ct0_d1_1x5();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==1u32 && s.height==5u32 && s.rgba@.len()==20nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// interlace/ct6-d16-1x5; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_interlace_ct6_d16_1x5() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,5u8,16u8,6u8,0u8,0u8,1u8,163u8,19u8,106u8,74u8,0u8,0u8,0u8,56u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,45u8,0u8,210u8,255u8,0u8,158u8,163u8,251u8,139u8,234u8,109u8,72u8,74u8,1u8,222u8,255u8,55u8,114u8,225u8,214u8,106u8,77u8,2u8,139u8,98u8,221u8,148u8,1u8,110u8,82u8,237u8,3u8,186u8,20u8,40u8,253u8,174u8,161u8,199u8,249u8,4u8,94u8,33u8,232u8,25u8,67u8,210u8,157u8,76u8,9u8,226u8,22u8,59u8,4u8,201u8,254u8,17u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_interlace_ct6_d16_1x5() -> Seq<u8> {seq![120u8,156u8,1u8,45u8,0u8,210u8,255u8,0u8,158u8,163u8,251u8,139u8,234u8,109u8,72u8,74u8,1u8,222u8,255u8,55u8,114u8,225u8,214u8,106u8,77u8,2u8,139u8,98u8,221u8,148u8,1u8,110u8,82u8,237u8,3u8,186u8,20u8,40u8,253u8,174u8,161u8,199u8,249u8,4u8,94u8,33u8,232u8,25u8,67u8,210u8,157u8,76u8,9u8,226u8,22u8,59u8]}
spec fn h_interlace_ct6_d16_1x5() -> crate::model::Ihdr {crate::model::Ihdr {width:1u32,height:5u32,depth:16u8,color_type:6u8,interlace:1u8}}
spec fn a_interlace_ct6_d16_1x5() -> r::Raster {r::shape(h_interlace_ct6_d16_1x5(),1nat,5nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_interlace_ct6_d16_1x5(b: Seq<u8>)
    requires b.len()==113,crate::model::mh(b)==Ok((crate::model::Ihdr {width:1u32,height:5u32,depth:16u8,color_type:6u8,interlace:1u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:56nat,next:101nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,101nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:109nat,len:0nat,next:113nat,crc_ok:true,synthetic:false})),
        b.subrange(41,97)==z_interlace_ct6_d16_1x5(),
    ensures r::default_case(b,crate::model::Ihdr {width:1u32,height:5u32,depth:16u8,color_type:6u8,interlace:1u8},z_interlace_ct6_d16_1x5(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    hide(z_interlace_ct6_d16_1x5);
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_interlace_ct6_d16_1x5() -> Seq<u8> {seq![0u8,158u8,163u8,251u8,139u8,234u8,109u8,72u8,74u8,1u8,222u8,255u8,55u8,114u8,225u8,214u8,106u8,77u8,2u8,139u8,98u8,221u8,148u8,1u8,110u8,82u8,237u8,3u8,186u8,20u8,40u8,253u8,174u8,161u8,199u8,249u8,4u8,94u8,33u8,232u8,25u8,67u8,210u8,157u8,76u8]}
#[verifier::spinoff_prover]
proof fn shape_interlace_ct6_d16_1x5()
    ensures raw_interlace_ct6_d16_1x5().len()==45nat,
        z_interlace_ct6_d16_1x5().len()==56nat,
        z_interlace_ct6_d16_1x5()[0]==120u8,
        z_interlace_ct6_d16_1x5()[1]==156u8,
        zm::field_value(z_interlace_ct6_d16_1x5(),16,1)==1nat,
        zm::field_value(z_interlace_ct6_d16_1x5(),17,2)==0nat,
        zm::be_value(z_interlace_ct6_d16_1x5(),52nat,4)==165811771nat,
        z_interlace_ct6_d16_1x5().subrange(7,52)==raw_interlace_ct6_d16_1x5(),
        z_interlace_ct6_d16_1x5()[3] as nat+256*z_interlace_ct6_d16_1x5()[4] as nat==45nat,
        z_interlace_ct6_d16_1x5()[5] as nat+256*z_interlace_ct6_d16_1x5()[6] as nat==u16::MAX as nat-45nat,
        zm::be_value(z_interlace_ct6_d16_1x5(),0,1)==120nat,
        zm::be_value(z_interlace_ct6_d16_1x5(),1,1)==156nat,
{
    hide(z_interlace_ct6_d16_1x5);hide(raw_interlace_ct6_d16_1x5);
    assert(raw_interlace_ct6_d16_1x5().len()==45nat) by (compute);
    assert(z_interlace_ct6_d16_1x5().len()==56nat) by (compute);
    assert(z_interlace_ct6_d16_1x5()[0]==120u8) by (compute);
    assert(z_interlace_ct6_d16_1x5()[1]==156u8) by (compute);
    assert(zm::field_value(z_interlace_ct6_d16_1x5(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_interlace_ct6_d16_1x5(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_interlace_ct6_d16_1x5(),52nat,4)==165811771nat) by (compute);
    assert(z_interlace_ct6_d16_1x5().subrange(7,52)==raw_interlace_ct6_d16_1x5()) by (compute);
    assert(z_interlace_ct6_d16_1x5()[3] as nat+256*z_interlace_ct6_d16_1x5()[4] as nat==45nat) by (compute);
    assert(z_interlace_ct6_d16_1x5()[5] as nat+256*z_interlace_ct6_d16_1x5()[6] as nat==u16::MAX as nat-45nat) by (compute);
    assert(zm::be_value(z_interlace_ct6_d16_1x5(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_interlace_ct6_d16_1x5(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_interlace_ct6_d16_1x5()
    ensures zm::Adler(raw_interlace_ct6_d16_1x5())==165811771nat,
{
    hide(raw_interlace_ct6_d16_1x5);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_interlace_ct6_d16_1x5().len()==45nat) by (compute);
    assert(c::sums(raw_interlace_ct6_d16_1x5(),0,45nat)==(5690nat,133572nat)) by (compute);
    c::sums_correct(raw_interlace_ct6_d16_1x5(),0,45nat);
    assert(raw_interlace_ct6_d16_1x5().subrange(0,45) =~= raw_interlace_ct6_d16_1x5());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((133572nat%65521)*(u16::MAX as nat+1)+(1+5690nat)%65521==165811771nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_interlace_ct6_d16_1x5()
    ensures r::inflated(z_interlace_ct6_d16_1x5(),a_interlace_ct6_d16_1x5(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_interlace_ct6_d16_1x5(),r2:false}),
        raw_interlace_ct6_d16_1x5().len()==45nat,
{
    hide(z_interlace_ct6_d16_1x5);hide(raw_interlace_ct6_d16_1x5);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_interlace_ct6_d16_1x5();checksum_interlace_ct6_d16_1x5();
    assert(r::raw(a_interlace_ct6_d16_1x5())==45nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_interlace_ct6_d16_1x5(),raw_interlace_ct6_d16_1x5(),45nat,120nat,156nat,165811771nat);
}
#[verifier::spinoff_prover]
proof fn scan_interlace_ct6_d16_1x5()
    ensures r::scan(raw_interlace_ct6_d16_1x5(),a_interlace_ct6_d16_1x5(),0)==Ok(()),
{
    hide(raw_interlace_ct6_d16_1x5);
    assert(r::count(a_interlace_ct6_d16_1x5())==7nat) by (compute);
    r::scan_step(raw_interlace_ct6_d16_1x5(),a_interlace_ct6_d16_1x5(),7);
    assert(r::pw(a_interlace_ct6_d16_1x5(),6nat)==1nat && r::ph(a_interlace_ct6_d16_1x5(),6nat)==2nat && r::rb(a_interlace_ct6_d16_1x5(),6nat)==8nat && r::base(a_interlace_ct6_d16_1x5(),6nat)==27nat) by (compute);
    assert(raw_interlace_ct6_d16_1x5()[27]==3u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_1x5(),a_interlace_ct6_d16_1x5(),6nat,0nat)==Ok(()));
    assert(raw_interlace_ct6_d16_1x5()[36]==4u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_1x5(),a_interlace_ct6_d16_1x5(),6nat,1nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_1x5(),a_interlace_ct6_d16_1x5(),6nat,2nat);
    r::pass_step(raw_interlace_ct6_d16_1x5(),a_interlace_ct6_d16_1x5(),6nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_1x5(),a_interlace_ct6_d16_1x5(),6nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_1x5(),a_interlace_ct6_d16_1x5(),6nat);
    assert(r::pw(a_interlace_ct6_d16_1x5(),5nat)==0nat && r::ph(a_interlace_ct6_d16_1x5(),5nat)==3nat && r::rb(a_interlace_ct6_d16_1x5(),5nat)==0nat && r::base(a_interlace_ct6_d16_1x5(),5nat)==27nat) by (compute);
    r::pass_step(raw_interlace_ct6_d16_1x5(),a_interlace_ct6_d16_1x5(),5nat,0);
    r::scan_step(raw_interlace_ct6_d16_1x5(),a_interlace_ct6_d16_1x5(),5nat);
    assert(r::pw(a_interlace_ct6_d16_1x5(),4nat)==1nat && r::ph(a_interlace_ct6_d16_1x5(),4nat)==1nat && r::rb(a_interlace_ct6_d16_1x5(),4nat)==8nat && r::base(a_interlace_ct6_d16_1x5(),4nat)==18nat) by (compute);
    assert(raw_interlace_ct6_d16_1x5()[18]==2u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_1x5(),a_interlace_ct6_d16_1x5(),4nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_1x5(),a_interlace_ct6_d16_1x5(),4nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_1x5(),a_interlace_ct6_d16_1x5(),4nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_1x5(),a_interlace_ct6_d16_1x5(),4nat);
    assert(r::pw(a_interlace_ct6_d16_1x5(),3nat)==0nat && r::ph(a_interlace_ct6_d16_1x5(),3nat)==2nat && r::rb(a_interlace_ct6_d16_1x5(),3nat)==0nat && r::base(a_interlace_ct6_d16_1x5(),3nat)==18nat) by (compute);
    r::pass_step(raw_interlace_ct6_d16_1x5(),a_interlace_ct6_d16_1x5(),3nat,0);
    r::scan_step(raw_interlace_ct6_d16_1x5(),a_interlace_ct6_d16_1x5(),3nat);
    assert(r::pw(a_interlace_ct6_d16_1x5(),2nat)==1nat && r::ph(a_interlace_ct6_d16_1x5(),2nat)==1nat && r::rb(a_interlace_ct6_d16_1x5(),2nat)==8nat && r::base(a_interlace_ct6_d16_1x5(),2nat)==9nat) by (compute);
    assert(raw_interlace_ct6_d16_1x5()[9]==1u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_1x5(),a_interlace_ct6_d16_1x5(),2nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_1x5(),a_interlace_ct6_d16_1x5(),2nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_1x5(),a_interlace_ct6_d16_1x5(),2nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_1x5(),a_interlace_ct6_d16_1x5(),2nat);
    assert(r::pw(a_interlace_ct6_d16_1x5(),1nat)==0nat && r::ph(a_interlace_ct6_d16_1x5(),1nat)==1nat && r::rb(a_interlace_ct6_d16_1x5(),1nat)==0nat && r::base(a_interlace_ct6_d16_1x5(),1nat)==9nat) by (compute);
    r::pass_step(raw_interlace_ct6_d16_1x5(),a_interlace_ct6_d16_1x5(),1nat,0);
    r::scan_step(raw_interlace_ct6_d16_1x5(),a_interlace_ct6_d16_1x5(),1nat);
    assert(r::pw(a_interlace_ct6_d16_1x5(),0nat)==1nat && r::ph(a_interlace_ct6_d16_1x5(),0nat)==1nat && r::rb(a_interlace_ct6_d16_1x5(),0nat)==8nat && r::base(a_interlace_ct6_d16_1x5(),0nat)==0nat) by (compute);
    assert(raw_interlace_ct6_d16_1x5()[0]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_1x5(),a_interlace_ct6_d16_1x5(),0nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_1x5(),a_interlace_ct6_d16_1x5(),0nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_1x5(),a_interlace_ct6_d16_1x5(),0nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_1x5(),a_interlace_ct6_d16_1x5(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_interlace_ct6_d16_1x5()
    ensures r::still(bytes_interlace_ct6_d16_1x5(),STRICT,false)==(r::Projection::Ready {width:1u32,height:5u32,bytes:20nat}),
{
    hide(bytes_interlace_ct6_d16_1x5);hide(z_interlace_ct6_d16_1x5);hide(raw_interlace_ct6_d16_1x5);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_interlace_ct6_d16_1x5().len()==113nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct6_d16_1x5())==Ok((crate::model::Ihdr {width:1u32,height:5u32,depth:16u8,color_type:6u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct6_d16_1x5())==Ok((crate::model::Ihdr {width:1u32,height:5u32,depth:16u8,color_type:6u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct6_d16_1x5(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:56nat,next:101nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct6_d16_1x5(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:56nat,next:101nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct6_d16_1x5(),101nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:109nat,len:0nat,next:113nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct6_d16_1x5(),101nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:109nat,len:0nat,next:113nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct6_d16_1x5().subrange(41,97)==z_interlace_ct6_d16_1x5()) by (compute);
    walk_interlace_ct6_d16_1x5(bytes_interlace_ct6_d16_1x5());
    inflate_interlace_ct6_d16_1x5();scan_interlace_ct6_d16_1x5();
    assert(r::raw(a_interlace_ct6_d16_1x5())==45nat) by (compute);
    assert(r::data(z_interlace_ct6_d16_1x5(),a_interlace_ct6_d16_1x5(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_interlace_ct6_d16_1x5(),h_interlace_ct6_d16_1x5(),z_interlace_ct6_d16_1x5(),0nat,false);
}
fn public_interlace_ct6_d16_1x5() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,5u8,16u8,6u8,0u8,0u8,1u8,163u8,19u8,106u8,74u8,0u8,0u8,0u8,56u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,45u8,0u8,210u8,255u8,0u8,158u8,163u8,251u8,139u8,234u8,109u8,72u8,74u8,1u8,222u8,255u8,55u8,114u8,225u8,214u8,106u8,77u8,2u8,139u8,98u8,221u8,148u8,1u8,110u8,82u8,237u8,3u8,186u8,20u8,40u8,253u8,174u8,161u8,199u8,249u8,4u8,94u8,33u8,232u8,25u8,67u8,210u8,157u8,76u8,9u8,226u8,22u8,59u8,4u8,201u8,254u8,17u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_interlace_ct6_d16_1x5());model_interlace_ct6_d16_1x5();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==1u32 && s.height==5u32 && s.rgba@.len()==20nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// interlace/ct3-d4-1x5; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_interlace_ct3_d4_1x5() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,5u8,4u8,3u8,0u8,0u8,1u8,1u8,173u8,171u8,58u8,0u8,0u8,0u8,48u8,80u8,76u8,84u8,69u8,87u8,8u8,201u8,231u8,105u8,48u8,146u8,222u8,193u8,246u8,123u8,62u8,127u8,17u8,162u8,150u8,214u8,243u8,70u8,168u8,2u8,185u8,190u8,15u8,115u8,201u8,98u8,124u8,177u8,208u8,133u8,240u8,176u8,10u8,36u8,156u8,79u8,103u8,70u8,180u8,169u8,43u8,220u8,164u8,131u8,13u8,126u8,151u8,186u8,111u8,222u8,13u8,0u8,0u8,0u8,18u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,96u8,76u8,96u8,122u8,192u8,220u8,192u8,242u8,1u8,0u8,9u8,162u8,2u8,187u8,231u8,49u8,168u8,193u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_interlace_ct3_d4_1x5() -> Seq<u8> {seq![120u8,156u8,99u8,96u8,96u8,76u8,96u8,122u8,192u8,220u8,192u8,242u8,1u8,0u8,9u8,162u8,2u8,187u8]}
spec fn h_interlace_ct3_d4_1x5() -> crate::model::Ihdr {crate::model::Ihdr {width:1u32,height:5u32,depth:4u8,color_type:3u8,interlace:1u8}}
spec fn a_interlace_ct3_d4_1x5() -> r::Raster {r::shape(h_interlace_ct3_d4_1x5(),1nat,5nat,16nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_interlace_ct3_d4_1x5(b: Seq<u8>)
    requires b.len()==135,crate::model::mh(b)==Ok((crate::model::Ihdr {width:1u32,height:5u32,depth:4u8,color_type:3u8,interlace:1u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:18nat,next:123nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,123nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:131nat,len:0nat,next:135nat,crc_ok:true,synthetic:false})),
        b.subrange(101,119)==z_interlace_ct3_d4_1x5(),
    ensures r::default_case(b,crate::model::Ihdr {width:1u32,height:5u32,depth:4u8,color_type:3u8,interlace:1u8},z_interlace_ct3_d4_1x5(),16nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);
    hide(z_interlace_ct3_d4_1x5);
    r::palette_default_case(b,crate::model::Ihdr {width:1u32,height:5u32,depth:4u8,color_type:3u8,interlace:1u8},z_interlace_ct3_d4_1x5(),18nat);
}
spec fn raw_interlace_ct3_d4_1x5() -> Seq<u8> {seq![0u8,0u8,1u8,96u8,2u8,224u8,3u8,128u8,4u8,240u8]}
spec fn ts_interlace_ct3_d4_1x5() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:35nat,before:1nat,after:2nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:35nat,next:43nat,before:2nat,after:3nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:43nat,next:51nat,before:3nat,after:4nat,symbol:96nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:51nat,next:59nat,before:4nat,after:5nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:59nat,next:68nat,before:5nat,after:6nat,symbol:224nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:68nat,next:76nat,before:6nat,after:7nat,symbol:3nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:76nat,next:84nat,before:7nat,after:8nat,symbol:128nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:84nat,next:92nat,before:8nat,after:9nat,symbol:4nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:92nat,next:101nat,before:9nat,after:10nat,symbol:240nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:101nat,next:108nat,before:10nat,after:10nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_interlace_ct3_d4_1x5()
    ensures t::trace(z_interlace_ct3_d4_1x5(),raw_interlace_ct3_d4_1x5(),ts_interlace_ct3_d4_1x5(),0,11nat),
{
    hide(z_interlace_ct3_d4_1x5);hide(raw_interlace_ct3_d4_1x5);hide(ts_interlace_ct3_d4_1x5);hide(t::valid_step);
    assert(t::trace(z_interlace_ct3_d4_1x5(),raw_interlace_ct3_d4_1x5(),ts_interlace_ct3_d4_1x5(),0,11nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_interlace_ct3_d4_1x5()
    ensures raw_interlace_ct3_d4_1x5().len()==10nat,
        z_interlace_ct3_d4_1x5().len()==18nat,
        z_interlace_ct3_d4_1x5()[0]==120u8,
        z_interlace_ct3_d4_1x5()[1]==156u8,
        zm::field_value(z_interlace_ct3_d4_1x5(),16,1)==1nat,
        zm::field_value(z_interlace_ct3_d4_1x5(),17,2)==1nat,
        zm::be_value(z_interlace_ct3_d4_1x5(),14nat,4)==161612475nat,
        ts_interlace_ct3_d4_1x5().len()==11nat,
        ts_interlace_ct3_d4_1x5()[0].pos==19nat,
        ts_interlace_ct3_d4_1x5()[0].before==0nat,
        ts_interlace_ct3_d4_1x5().last().next==108nat,
{
    hide(z_interlace_ct3_d4_1x5);hide(raw_interlace_ct3_d4_1x5);hide(ts_interlace_ct3_d4_1x5);
    assert(raw_interlace_ct3_d4_1x5().len()==10nat) by (compute);
    assert(z_interlace_ct3_d4_1x5().len()==18nat) by (compute);
    assert(z_interlace_ct3_d4_1x5()[0]==120u8) by (compute);
    assert(z_interlace_ct3_d4_1x5()[1]==156u8) by (compute);
    assert(zm::field_value(z_interlace_ct3_d4_1x5(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_interlace_ct3_d4_1x5(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_interlace_ct3_d4_1x5(),14nat,4)==161612475nat) by (compute);
    assert(ts_interlace_ct3_d4_1x5().len()==11nat) by (compute);
    assert(ts_interlace_ct3_d4_1x5()[0].pos==19nat) by (compute);
    assert(ts_interlace_ct3_d4_1x5()[0].before==0nat) by (compute);
    assert(ts_interlace_ct3_d4_1x5().last().next==108nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_interlace_ct3_d4_1x5()
    ensures zm::Adler(raw_interlace_ct3_d4_1x5())==161612475nat,
{
    hide(raw_interlace_ct3_d4_1x5);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_interlace_ct3_d4_1x5().len()==10nat) by (compute);
    assert(c::sums(raw_interlace_ct3_d4_1x5(),0,10nat)==(698nat,2466nat)) by (compute);
    c::sums_correct(raw_interlace_ct3_d4_1x5(),0,10nat);
    assert(raw_interlace_ct3_d4_1x5().subrange(0,10) =~= raw_interlace_ct3_d4_1x5());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((2466nat%65521)*(u16::MAX as nat+1)+(1+698nat)%65521==161612475nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_interlace_ct3_d4_1x5()
    ensures r::inflated(z_interlace_ct3_d4_1x5(),a_interlace_ct3_d4_1x5(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_interlace_ct3_d4_1x5(),r2:false}),
        raw_interlace_ct3_d4_1x5().len()==10nat,
{
    hide(z_interlace_ct3_d4_1x5);hide(raw_interlace_ct3_d4_1x5);hide(ts_interlace_ct3_d4_1x5);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_interlace_ct3_d4_1x5();checksum_interlace_ct3_d4_1x5();
    assert(r::raw(a_interlace_ct3_d4_1x5())==10nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_interlace_ct3_d4_1x5();
    stream::fixed_stream(z_interlace_ct3_d4_1x5(),raw_interlace_ct3_d4_1x5(),ts_interlace_ct3_d4_1x5(),10nat,156nat,161612475nat);
}
#[verifier::spinoff_prover]
proof fn scan_interlace_ct3_d4_1x5()
    ensures r::scan(raw_interlace_ct3_d4_1x5(),a_interlace_ct3_d4_1x5(),0)==Ok(()),
{
    hide(raw_interlace_ct3_d4_1x5);
    assert(r::count(a_interlace_ct3_d4_1x5())==7nat) by (compute);
    r::scan_step(raw_interlace_ct3_d4_1x5(),a_interlace_ct3_d4_1x5(),7);
    assert(r::pw(a_interlace_ct3_d4_1x5(),6nat)==1nat && r::ph(a_interlace_ct3_d4_1x5(),6nat)==2nat && r::rb(a_interlace_ct3_d4_1x5(),6nat)==1nat && r::base(a_interlace_ct3_d4_1x5(),6nat)==6nat) by (compute);
    assert(raw_interlace_ct3_d4_1x5()[6]==3u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_1x5(),a_interlace_ct3_d4_1x5(),6nat,0nat),1nat);
    assert(r::row_result(raw_interlace_ct3_d4_1x5(),a_interlace_ct3_d4_1x5(),6nat,0nat)==Ok(()));
    assert(raw_interlace_ct3_d4_1x5()[8]==4u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_1x5(),a_interlace_ct3_d4_1x5(),6nat,1nat),1nat);
    assert(r::row_result(raw_interlace_ct3_d4_1x5(),a_interlace_ct3_d4_1x5(),6nat,1nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_1x5(),a_interlace_ct3_d4_1x5(),6nat,2nat);
    r::pass_step(raw_interlace_ct3_d4_1x5(),a_interlace_ct3_d4_1x5(),6nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_1x5(),a_interlace_ct3_d4_1x5(),6nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_1x5(),a_interlace_ct3_d4_1x5(),6nat);
    assert(r::pw(a_interlace_ct3_d4_1x5(),5nat)==0nat && r::ph(a_interlace_ct3_d4_1x5(),5nat)==3nat && r::rb(a_interlace_ct3_d4_1x5(),5nat)==0nat && r::base(a_interlace_ct3_d4_1x5(),5nat)==6nat) by (compute);
    r::pass_step(raw_interlace_ct3_d4_1x5(),a_interlace_ct3_d4_1x5(),5nat,0);
    r::scan_step(raw_interlace_ct3_d4_1x5(),a_interlace_ct3_d4_1x5(),5nat);
    assert(r::pw(a_interlace_ct3_d4_1x5(),4nat)==1nat && r::ph(a_interlace_ct3_d4_1x5(),4nat)==1nat && r::rb(a_interlace_ct3_d4_1x5(),4nat)==1nat && r::base(a_interlace_ct3_d4_1x5(),4nat)==4nat) by (compute);
    assert(raw_interlace_ct3_d4_1x5()[4]==2u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_1x5(),a_interlace_ct3_d4_1x5(),4nat,0nat),1nat);
    assert(r::row_result(raw_interlace_ct3_d4_1x5(),a_interlace_ct3_d4_1x5(),4nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_1x5(),a_interlace_ct3_d4_1x5(),4nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_1x5(),a_interlace_ct3_d4_1x5(),4nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_1x5(),a_interlace_ct3_d4_1x5(),4nat);
    assert(r::pw(a_interlace_ct3_d4_1x5(),3nat)==0nat && r::ph(a_interlace_ct3_d4_1x5(),3nat)==2nat && r::rb(a_interlace_ct3_d4_1x5(),3nat)==0nat && r::base(a_interlace_ct3_d4_1x5(),3nat)==4nat) by (compute);
    r::pass_step(raw_interlace_ct3_d4_1x5(),a_interlace_ct3_d4_1x5(),3nat,0);
    r::scan_step(raw_interlace_ct3_d4_1x5(),a_interlace_ct3_d4_1x5(),3nat);
    assert(r::pw(a_interlace_ct3_d4_1x5(),2nat)==1nat && r::ph(a_interlace_ct3_d4_1x5(),2nat)==1nat && r::rb(a_interlace_ct3_d4_1x5(),2nat)==1nat && r::base(a_interlace_ct3_d4_1x5(),2nat)==2nat) by (compute);
    assert(raw_interlace_ct3_d4_1x5()[2]==1u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_1x5(),a_interlace_ct3_d4_1x5(),2nat,0nat),1nat);
    assert(r::row_result(raw_interlace_ct3_d4_1x5(),a_interlace_ct3_d4_1x5(),2nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_1x5(),a_interlace_ct3_d4_1x5(),2nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_1x5(),a_interlace_ct3_d4_1x5(),2nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_1x5(),a_interlace_ct3_d4_1x5(),2nat);
    assert(r::pw(a_interlace_ct3_d4_1x5(),1nat)==0nat && r::ph(a_interlace_ct3_d4_1x5(),1nat)==1nat && r::rb(a_interlace_ct3_d4_1x5(),1nat)==0nat && r::base(a_interlace_ct3_d4_1x5(),1nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct3_d4_1x5(),a_interlace_ct3_d4_1x5(),1nat,0);
    r::scan_step(raw_interlace_ct3_d4_1x5(),a_interlace_ct3_d4_1x5(),1nat);
    assert(r::pw(a_interlace_ct3_d4_1x5(),0nat)==1nat && r::ph(a_interlace_ct3_d4_1x5(),0nat)==1nat && r::rb(a_interlace_ct3_d4_1x5(),0nat)==1nat && r::base(a_interlace_ct3_d4_1x5(),0nat)==0nat) by (compute);
    assert(raw_interlace_ct3_d4_1x5()[0]==0u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_1x5(),a_interlace_ct3_d4_1x5(),0nat,0nat),1nat);
    assert(r::row_result(raw_interlace_ct3_d4_1x5(),a_interlace_ct3_d4_1x5(),0nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_1x5(),a_interlace_ct3_d4_1x5(),0nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_1x5(),a_interlace_ct3_d4_1x5(),0nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_1x5(),a_interlace_ct3_d4_1x5(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_interlace_ct3_d4_1x5()
    ensures r::still(bytes_interlace_ct3_d4_1x5(),STRICT,false)==(r::Projection::Ready {width:1u32,height:5u32,bytes:20nat}),
{
    hide(bytes_interlace_ct3_d4_1x5);hide(z_interlace_ct3_d4_1x5);hide(raw_interlace_ct3_d4_1x5);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);
    assert(bytes_interlace_ct3_d4_1x5().len()==135nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct3_d4_1x5())==Ok((crate::model::Ihdr {width:1u32,height:5u32,depth:4u8,color_type:3u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct3_d4_1x5())==Ok((crate::model::Ihdr {width:1u32,height:5u32,depth:4u8,color_type:3u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 4u8)==16u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_1x5(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_1x5(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(48nat,3u8,4u8,false,false,false,false)==crate::metadata_model::Decision::Keep(16nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_1x5(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:18nat,next:123nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_1x5(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:18nat,next:123nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_1x5(),123nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:131nat,len:0nat,next:135nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_1x5(),123nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:131nat,len:0nat,next:135nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct3_d4_1x5().subrange(101,119)==z_interlace_ct3_d4_1x5()) by (compute);
    walk_interlace_ct3_d4_1x5(bytes_interlace_ct3_d4_1x5());
    inflate_interlace_ct3_d4_1x5();scan_interlace_ct3_d4_1x5();
    assert(r::raw(a_interlace_ct3_d4_1x5())==10nat) by (compute);
    assert(r::data(z_interlace_ct3_d4_1x5(),a_interlace_ct3_d4_1x5(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_interlace_ct3_d4_1x5(),h_interlace_ct3_d4_1x5(),z_interlace_ct3_d4_1x5(),16nat,false);
}
fn public_interlace_ct3_d4_1x5() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,5u8,4u8,3u8,0u8,0u8,1u8,1u8,173u8,171u8,58u8,0u8,0u8,0u8,48u8,80u8,76u8,84u8,69u8,87u8,8u8,201u8,231u8,105u8,48u8,146u8,222u8,193u8,246u8,123u8,62u8,127u8,17u8,162u8,150u8,214u8,243u8,70u8,168u8,2u8,185u8,190u8,15u8,115u8,201u8,98u8,124u8,177u8,208u8,133u8,240u8,176u8,10u8,36u8,156u8,79u8,103u8,70u8,180u8,169u8,43u8,220u8,164u8,131u8,13u8,126u8,151u8,186u8,111u8,222u8,13u8,0u8,0u8,0u8,18u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,96u8,76u8,96u8,122u8,192u8,220u8,192u8,242u8,1u8,0u8,9u8,162u8,2u8,187u8,231u8,49u8,168u8,193u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_interlace_ct3_d4_1x5());model_interlace_ct3_d4_1x5();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==1u32 && s.height==5u32 && s.rgba@.len()==20nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// interlace/ct0-d1-5x1; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_interlace_ct0_d1_5x1() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,5u8,0u8,0u8,0u8,1u8,1u8,0u8,0u8,0u8,1u8,73u8,130u8,105u8,200u8,0u8,0u8,0u8,16u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,104u8,96u8,108u8,96u8,98u8,96u8,102u8,0u8,0u8,6u8,28u8,1u8,7u8,19u8,83u8,222u8,14u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_interlace_ct0_d1_5x1() -> Seq<u8> {seq![120u8,156u8,99u8,104u8,96u8,108u8,96u8,98u8,96u8,102u8,0u8,0u8,6u8,28u8,1u8,7u8]}
spec fn h_interlace_ct0_d1_5x1() -> crate::model::Ihdr {crate::model::Ihdr {width:5u32,height:1u32,depth:1u8,color_type:0u8,interlace:1u8}}
spec fn a_interlace_ct0_d1_5x1() -> r::Raster {r::shape(h_interlace_ct0_d1_5x1(),5nat,1nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_interlace_ct0_d1_5x1(b: Seq<u8>)
    requires b.len()==73,crate::model::mh(b)==Ok((crate::model::Ihdr {width:5u32,height:1u32,depth:1u8,color_type:0u8,interlace:1u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:16nat,next:61nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,61nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:69nat,len:0nat,next:73nat,crc_ok:true,synthetic:false})),
        b.subrange(41,57)==z_interlace_ct0_d1_5x1(),
    ensures r::default_case(b,crate::model::Ihdr {width:5u32,height:1u32,depth:1u8,color_type:0u8,interlace:1u8},z_interlace_ct0_d1_5x1(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    hide(z_interlace_ct0_d1_5x1);
        assert((1u32 << 1u8)==2u32) by (compute);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_interlace_ct0_d1_5x1() -> Seq<u8> {seq![0u8,128u8,1u8,128u8,2u8,0u8,3u8,0u8]}
spec fn ts_interlace_ct0_d1_5x1() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:35nat,before:1nat,after:2nat,symbol:128nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:35nat,next:43nat,before:2nat,after:3nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:43nat,next:51nat,before:3nat,after:4nat,symbol:128nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:51nat,next:59nat,before:4nat,after:5nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:59nat,next:67nat,before:5nat,after:6nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:67nat,next:75nat,before:6nat,after:7nat,symbol:3nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:75nat,next:83nat,before:7nat,after:8nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:83nat,next:90nat,before:8nat,after:8nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_interlace_ct0_d1_5x1()
    ensures t::trace(z_interlace_ct0_d1_5x1(),raw_interlace_ct0_d1_5x1(),ts_interlace_ct0_d1_5x1(),0,9nat),
{
    hide(z_interlace_ct0_d1_5x1);hide(raw_interlace_ct0_d1_5x1);hide(ts_interlace_ct0_d1_5x1);hide(t::valid_step);
    assert(t::trace(z_interlace_ct0_d1_5x1(),raw_interlace_ct0_d1_5x1(),ts_interlace_ct0_d1_5x1(),0,9nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_interlace_ct0_d1_5x1()
    ensures raw_interlace_ct0_d1_5x1().len()==8nat,
        z_interlace_ct0_d1_5x1().len()==16nat,
        z_interlace_ct0_d1_5x1()[0]==120u8,
        z_interlace_ct0_d1_5x1()[1]==156u8,
        zm::field_value(z_interlace_ct0_d1_5x1(),16,1)==1nat,
        zm::field_value(z_interlace_ct0_d1_5x1(),17,2)==1nat,
        zm::be_value(z_interlace_ct0_d1_5x1(),12nat,4)==102498567nat,
        ts_interlace_ct0_d1_5x1().len()==9nat,
        ts_interlace_ct0_d1_5x1()[0].pos==19nat,
        ts_interlace_ct0_d1_5x1()[0].before==0nat,
        ts_interlace_ct0_d1_5x1().last().next==90nat,
{
    hide(z_interlace_ct0_d1_5x1);hide(raw_interlace_ct0_d1_5x1);hide(ts_interlace_ct0_d1_5x1);
    assert(raw_interlace_ct0_d1_5x1().len()==8nat) by (compute);
    assert(z_interlace_ct0_d1_5x1().len()==16nat) by (compute);
    assert(z_interlace_ct0_d1_5x1()[0]==120u8) by (compute);
    assert(z_interlace_ct0_d1_5x1()[1]==156u8) by (compute);
    assert(zm::field_value(z_interlace_ct0_d1_5x1(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_interlace_ct0_d1_5x1(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_interlace_ct0_d1_5x1(),12nat,4)==102498567nat) by (compute);
    assert(ts_interlace_ct0_d1_5x1().len()==9nat) by (compute);
    assert(ts_interlace_ct0_d1_5x1()[0].pos==19nat) by (compute);
    assert(ts_interlace_ct0_d1_5x1()[0].before==0nat) by (compute);
    assert(ts_interlace_ct0_d1_5x1().last().next==90nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_interlace_ct0_d1_5x1()
    ensures zm::Adler(raw_interlace_ct0_d1_5x1())==102498567nat,
{
    hide(raw_interlace_ct0_d1_5x1);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_interlace_ct0_d1_5x1().len()==8nat) by (compute);
    assert(c::sums(raw_interlace_ct0_d1_5x1(),0,8nat)==(262nat,1564nat)) by (compute);
    c::sums_correct(raw_interlace_ct0_d1_5x1(),0,8nat);
    assert(raw_interlace_ct0_d1_5x1().subrange(0,8) =~= raw_interlace_ct0_d1_5x1());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((1564nat%65521)*(u16::MAX as nat+1)+(1+262nat)%65521==102498567nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_interlace_ct0_d1_5x1()
    ensures r::inflated(z_interlace_ct0_d1_5x1(),a_interlace_ct0_d1_5x1(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_interlace_ct0_d1_5x1(),r2:false}),
        raw_interlace_ct0_d1_5x1().len()==8nat,
{
    hide(z_interlace_ct0_d1_5x1);hide(raw_interlace_ct0_d1_5x1);hide(ts_interlace_ct0_d1_5x1);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_interlace_ct0_d1_5x1();checksum_interlace_ct0_d1_5x1();
    assert(r::raw(a_interlace_ct0_d1_5x1())==8nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_interlace_ct0_d1_5x1();
    stream::fixed_stream(z_interlace_ct0_d1_5x1(),raw_interlace_ct0_d1_5x1(),ts_interlace_ct0_d1_5x1(),8nat,156nat,102498567nat);
}
#[verifier::spinoff_prover]
proof fn scan_interlace_ct0_d1_5x1()
    ensures r::scan(raw_interlace_ct0_d1_5x1(),a_interlace_ct0_d1_5x1(),0)==Ok(()),
{
    hide(raw_interlace_ct0_d1_5x1);
    assert(r::count(a_interlace_ct0_d1_5x1())==7nat) by (compute);
    r::scan_step(raw_interlace_ct0_d1_5x1(),a_interlace_ct0_d1_5x1(),7);
    assert(r::pw(a_interlace_ct0_d1_5x1(),6nat)==5nat && r::ph(a_interlace_ct0_d1_5x1(),6nat)==0nat && r::rb(a_interlace_ct0_d1_5x1(),6nat)==1nat && r::base(a_interlace_ct0_d1_5x1(),6nat)==8nat) by (compute);
    r::pass_step(raw_interlace_ct0_d1_5x1(),a_interlace_ct0_d1_5x1(),6nat,0);
    r::scan_step(raw_interlace_ct0_d1_5x1(),a_interlace_ct0_d1_5x1(),6nat);
    assert(r::pw(a_interlace_ct0_d1_5x1(),5nat)==2nat && r::ph(a_interlace_ct0_d1_5x1(),5nat)==1nat && r::rb(a_interlace_ct0_d1_5x1(),5nat)==1nat && r::base(a_interlace_ct0_d1_5x1(),5nat)==6nat) by (compute);
    assert(raw_interlace_ct0_d1_5x1()[6]==3u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_5x1(),a_interlace_ct0_d1_5x1(),5nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_5x1(),a_interlace_ct0_d1_5x1(),5nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_5x1(),a_interlace_ct0_d1_5x1(),5nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_5x1(),a_interlace_ct0_d1_5x1(),5nat);
    assert(r::pw(a_interlace_ct0_d1_5x1(),4nat)==3nat && r::ph(a_interlace_ct0_d1_5x1(),4nat)==0nat && r::rb(a_interlace_ct0_d1_5x1(),4nat)==1nat && r::base(a_interlace_ct0_d1_5x1(),4nat)==6nat) by (compute);
    r::pass_step(raw_interlace_ct0_d1_5x1(),a_interlace_ct0_d1_5x1(),4nat,0);
    r::scan_step(raw_interlace_ct0_d1_5x1(),a_interlace_ct0_d1_5x1(),4nat);
    assert(r::pw(a_interlace_ct0_d1_5x1(),3nat)==1nat && r::ph(a_interlace_ct0_d1_5x1(),3nat)==1nat && r::rb(a_interlace_ct0_d1_5x1(),3nat)==1nat && r::base(a_interlace_ct0_d1_5x1(),3nat)==4nat) by (compute);
    assert(raw_interlace_ct0_d1_5x1()[4]==2u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_5x1(),a_interlace_ct0_d1_5x1(),3nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_5x1(),a_interlace_ct0_d1_5x1(),3nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_5x1(),a_interlace_ct0_d1_5x1(),3nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_5x1(),a_interlace_ct0_d1_5x1(),3nat);
    assert(r::pw(a_interlace_ct0_d1_5x1(),2nat)==2nat && r::ph(a_interlace_ct0_d1_5x1(),2nat)==0nat && r::rb(a_interlace_ct0_d1_5x1(),2nat)==1nat && r::base(a_interlace_ct0_d1_5x1(),2nat)==4nat) by (compute);
    r::pass_step(raw_interlace_ct0_d1_5x1(),a_interlace_ct0_d1_5x1(),2nat,0);
    r::scan_step(raw_interlace_ct0_d1_5x1(),a_interlace_ct0_d1_5x1(),2nat);
    assert(r::pw(a_interlace_ct0_d1_5x1(),1nat)==1nat && r::ph(a_interlace_ct0_d1_5x1(),1nat)==1nat && r::rb(a_interlace_ct0_d1_5x1(),1nat)==1nat && r::base(a_interlace_ct0_d1_5x1(),1nat)==2nat) by (compute);
    assert(raw_interlace_ct0_d1_5x1()[2]==1u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_5x1(),a_interlace_ct0_d1_5x1(),1nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_5x1(),a_interlace_ct0_d1_5x1(),1nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_5x1(),a_interlace_ct0_d1_5x1(),1nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_5x1(),a_interlace_ct0_d1_5x1(),1nat);
    assert(r::pw(a_interlace_ct0_d1_5x1(),0nat)==1nat && r::ph(a_interlace_ct0_d1_5x1(),0nat)==1nat && r::rb(a_interlace_ct0_d1_5x1(),0nat)==1nat && r::base(a_interlace_ct0_d1_5x1(),0nat)==0nat) by (compute);
    assert(raw_interlace_ct0_d1_5x1()[0]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_5x1(),a_interlace_ct0_d1_5x1(),0nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_5x1(),a_interlace_ct0_d1_5x1(),0nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_5x1(),a_interlace_ct0_d1_5x1(),0nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_5x1(),a_interlace_ct0_d1_5x1(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_interlace_ct0_d1_5x1()
    ensures r::still(bytes_interlace_ct0_d1_5x1(),STRICT,false)==(r::Projection::Ready {width:5u32,height:1u32,bytes:20nat}),
{
    hide(bytes_interlace_ct0_d1_5x1);hide(z_interlace_ct0_d1_5x1);hide(raw_interlace_ct0_d1_5x1);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_interlace_ct0_d1_5x1().len()==73nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct0_d1_5x1())==Ok((crate::model::Ihdr {width:5u32,height:1u32,depth:1u8,color_type:0u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct0_d1_5x1())==Ok((crate::model::Ihdr {width:5u32,height:1u32,depth:1u8,color_type:0u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 1u8)==2u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct0_d1_5x1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:16nat,next:61nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct0_d1_5x1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:16nat,next:61nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct0_d1_5x1(),61nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:69nat,len:0nat,next:73nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct0_d1_5x1(),61nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:69nat,len:0nat,next:73nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct0_d1_5x1().subrange(41,57)==z_interlace_ct0_d1_5x1()) by (compute);
    walk_interlace_ct0_d1_5x1(bytes_interlace_ct0_d1_5x1());
    inflate_interlace_ct0_d1_5x1();scan_interlace_ct0_d1_5x1();
    assert(r::raw(a_interlace_ct0_d1_5x1())==8nat) by (compute);
    assert(r::data(z_interlace_ct0_d1_5x1(),a_interlace_ct0_d1_5x1(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_interlace_ct0_d1_5x1(),h_interlace_ct0_d1_5x1(),z_interlace_ct0_d1_5x1(),0nat,false);
}
fn public_interlace_ct0_d1_5x1() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,5u8,0u8,0u8,0u8,1u8,1u8,0u8,0u8,0u8,1u8,73u8,130u8,105u8,200u8,0u8,0u8,0u8,16u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,104u8,96u8,108u8,96u8,98u8,96u8,102u8,0u8,0u8,6u8,28u8,1u8,7u8,19u8,83u8,222u8,14u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_interlace_ct0_d1_5x1());model_interlace_ct0_d1_5x1();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==5u32 && s.height==1u32 && s.rgba@.len()==20nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// interlace/ct6-d16-5x1; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_interlace_ct6_d16_5x1() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,5u8,0u8,0u8,0u8,1u8,16u8,6u8,0u8,0u8,1u8,49u8,105u8,136u8,38u8,0u8,0u8,0u8,55u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,44u8,0u8,211u8,255u8,0u8,120u8,193u8,130u8,10u8,237u8,34u8,223u8,182u8,1u8,87u8,141u8,58u8,175u8,165u8,32u8,46u8,134u8,2u8,246u8,181u8,232u8,19u8,221u8,101u8,184u8,36u8,3u8,25u8,176u8,158u8,11u8,39u8,83u8,115u8,169u8,28u8,252u8,208u8,43u8,185u8,193u8,224u8,245u8,182u8,42u8,20u8,228u8,206u8,95u8,35u8,250u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_interlace_ct6_d16_5x1() -> Seq<u8> {seq![120u8,156u8,1u8,44u8,0u8,211u8,255u8,0u8,120u8,193u8,130u8,10u8,237u8,34u8,223u8,182u8,1u8,87u8,141u8,58u8,175u8,165u8,32u8,46u8,134u8,2u8,246u8,181u8,232u8,19u8,221u8,101u8,184u8,36u8,3u8,25u8,176u8,158u8,11u8,39u8,83u8,115u8,169u8,28u8,252u8,208u8,43u8,185u8,193u8,224u8,245u8,182u8,42u8,20u8,228u8]}
spec fn h_interlace_ct6_d16_5x1() -> crate::model::Ihdr {crate::model::Ihdr {width:5u32,height:1u32,depth:16u8,color_type:6u8,interlace:1u8}}
spec fn a_interlace_ct6_d16_5x1() -> r::Raster {r::shape(h_interlace_ct6_d16_5x1(),5nat,1nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_interlace_ct6_d16_5x1(b: Seq<u8>)
    requires b.len()==112,crate::model::mh(b)==Ok((crate::model::Ihdr {width:5u32,height:1u32,depth:16u8,color_type:6u8,interlace:1u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:55nat,next:100nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,100nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:108nat,len:0nat,next:112nat,crc_ok:true,synthetic:false})),
        b.subrange(41,96)==z_interlace_ct6_d16_5x1(),
    ensures r::default_case(b,crate::model::Ihdr {width:5u32,height:1u32,depth:16u8,color_type:6u8,interlace:1u8},z_interlace_ct6_d16_5x1(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    hide(z_interlace_ct6_d16_5x1);
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_interlace_ct6_d16_5x1() -> Seq<u8> {seq![0u8,120u8,193u8,130u8,10u8,237u8,34u8,223u8,182u8,1u8,87u8,141u8,58u8,175u8,165u8,32u8,46u8,134u8,2u8,246u8,181u8,232u8,19u8,221u8,101u8,184u8,36u8,3u8,25u8,176u8,158u8,11u8,39u8,83u8,115u8,169u8,28u8,252u8,208u8,43u8,185u8,193u8,224u8,245u8]}
#[verifier::spinoff_prover]
proof fn shape_interlace_ct6_d16_5x1()
    ensures raw_interlace_ct6_d16_5x1().len()==44nat,
        z_interlace_ct6_d16_5x1().len()==55nat,
        z_interlace_ct6_d16_5x1()[0]==120u8,
        z_interlace_ct6_d16_5x1()[1]==156u8,
        zm::field_value(z_interlace_ct6_d16_5x1(),16,1)==1nat,
        zm::field_value(z_interlace_ct6_d16_5x1(),17,2)==0nat,
        zm::be_value(z_interlace_ct6_d16_5x1(),51nat,4)==3056211172nat,
        z_interlace_ct6_d16_5x1().subrange(7,51)==raw_interlace_ct6_d16_5x1(),
        z_interlace_ct6_d16_5x1()[3] as nat+256*z_interlace_ct6_d16_5x1()[4] as nat==44nat,
        z_interlace_ct6_d16_5x1()[5] as nat+256*z_interlace_ct6_d16_5x1()[6] as nat==u16::MAX as nat-44nat,
        zm::be_value(z_interlace_ct6_d16_5x1(),0,1)==120nat,
        zm::be_value(z_interlace_ct6_d16_5x1(),1,1)==156nat,
{
    hide(z_interlace_ct6_d16_5x1);hide(raw_interlace_ct6_d16_5x1);
    assert(raw_interlace_ct6_d16_5x1().len()==44nat) by (compute);
    assert(z_interlace_ct6_d16_5x1().len()==55nat) by (compute);
    assert(z_interlace_ct6_d16_5x1()[0]==120u8) by (compute);
    assert(z_interlace_ct6_d16_5x1()[1]==156u8) by (compute);
    assert(zm::field_value(z_interlace_ct6_d16_5x1(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_interlace_ct6_d16_5x1(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_interlace_ct6_d16_5x1(),51nat,4)==3056211172nat) by (compute);
    assert(z_interlace_ct6_d16_5x1().subrange(7,51)==raw_interlace_ct6_d16_5x1()) by (compute);
    assert(z_interlace_ct6_d16_5x1()[3] as nat+256*z_interlace_ct6_d16_5x1()[4] as nat==44nat) by (compute);
    assert(z_interlace_ct6_d16_5x1()[5] as nat+256*z_interlace_ct6_d16_5x1()[6] as nat==u16::MAX as nat-44nat) by (compute);
    assert(zm::be_value(z_interlace_ct6_d16_5x1(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_interlace_ct6_d16_5x1(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_interlace_ct6_d16_5x1()
    ensures zm::Adler(raw_interlace_ct6_d16_5x1())==3056211172nat,
{
    hide(raw_interlace_ct6_d16_5x1);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_interlace_ct6_d16_5x1().len()==44nat) by (compute);
    assert(c::sums(raw_interlace_ct6_d16_5x1(),0,44nat)==(5347nat,112155nat)) by (compute);
    c::sums_correct(raw_interlace_ct6_d16_5x1(),0,44nat);
    assert(raw_interlace_ct6_d16_5x1().subrange(0,44) =~= raw_interlace_ct6_d16_5x1());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((112155nat%65521)*(u16::MAX as nat+1)+(1+5347nat)%65521==3056211172nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_interlace_ct6_d16_5x1()
    ensures r::inflated(z_interlace_ct6_d16_5x1(),a_interlace_ct6_d16_5x1(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_interlace_ct6_d16_5x1(),r2:false}),
        raw_interlace_ct6_d16_5x1().len()==44nat,
{
    hide(z_interlace_ct6_d16_5x1);hide(raw_interlace_ct6_d16_5x1);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_interlace_ct6_d16_5x1();checksum_interlace_ct6_d16_5x1();
    assert(r::raw(a_interlace_ct6_d16_5x1())==44nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_interlace_ct6_d16_5x1(),raw_interlace_ct6_d16_5x1(),44nat,120nat,156nat,3056211172nat);
}
#[verifier::spinoff_prover]
proof fn scan_interlace_ct6_d16_5x1()
    ensures r::scan(raw_interlace_ct6_d16_5x1(),a_interlace_ct6_d16_5x1(),0)==Ok(()),
{
    hide(raw_interlace_ct6_d16_5x1);
    assert(r::count(a_interlace_ct6_d16_5x1())==7nat) by (compute);
    r::scan_step(raw_interlace_ct6_d16_5x1(),a_interlace_ct6_d16_5x1(),7);
    assert(r::pw(a_interlace_ct6_d16_5x1(),6nat)==5nat && r::ph(a_interlace_ct6_d16_5x1(),6nat)==0nat && r::rb(a_interlace_ct6_d16_5x1(),6nat)==40nat && r::base(a_interlace_ct6_d16_5x1(),6nat)==44nat) by (compute);
    r::pass_step(raw_interlace_ct6_d16_5x1(),a_interlace_ct6_d16_5x1(),6nat,0);
    r::scan_step(raw_interlace_ct6_d16_5x1(),a_interlace_ct6_d16_5x1(),6nat);
    assert(r::pw(a_interlace_ct6_d16_5x1(),5nat)==2nat && r::ph(a_interlace_ct6_d16_5x1(),5nat)==1nat && r::rb(a_interlace_ct6_d16_5x1(),5nat)==16nat && r::base(a_interlace_ct6_d16_5x1(),5nat)==27nat) by (compute);
    assert(raw_interlace_ct6_d16_5x1()[27]==3u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_5x1(),a_interlace_ct6_d16_5x1(),5nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_5x1(),a_interlace_ct6_d16_5x1(),5nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_5x1(),a_interlace_ct6_d16_5x1(),5nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_5x1(),a_interlace_ct6_d16_5x1(),5nat);
    assert(r::pw(a_interlace_ct6_d16_5x1(),4nat)==3nat && r::ph(a_interlace_ct6_d16_5x1(),4nat)==0nat && r::rb(a_interlace_ct6_d16_5x1(),4nat)==24nat && r::base(a_interlace_ct6_d16_5x1(),4nat)==27nat) by (compute);
    r::pass_step(raw_interlace_ct6_d16_5x1(),a_interlace_ct6_d16_5x1(),4nat,0);
    r::scan_step(raw_interlace_ct6_d16_5x1(),a_interlace_ct6_d16_5x1(),4nat);
    assert(r::pw(a_interlace_ct6_d16_5x1(),3nat)==1nat && r::ph(a_interlace_ct6_d16_5x1(),3nat)==1nat && r::rb(a_interlace_ct6_d16_5x1(),3nat)==8nat && r::base(a_interlace_ct6_d16_5x1(),3nat)==18nat) by (compute);
    assert(raw_interlace_ct6_d16_5x1()[18]==2u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_5x1(),a_interlace_ct6_d16_5x1(),3nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_5x1(),a_interlace_ct6_d16_5x1(),3nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_5x1(),a_interlace_ct6_d16_5x1(),3nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_5x1(),a_interlace_ct6_d16_5x1(),3nat);
    assert(r::pw(a_interlace_ct6_d16_5x1(),2nat)==2nat && r::ph(a_interlace_ct6_d16_5x1(),2nat)==0nat && r::rb(a_interlace_ct6_d16_5x1(),2nat)==16nat && r::base(a_interlace_ct6_d16_5x1(),2nat)==18nat) by (compute);
    r::pass_step(raw_interlace_ct6_d16_5x1(),a_interlace_ct6_d16_5x1(),2nat,0);
    r::scan_step(raw_interlace_ct6_d16_5x1(),a_interlace_ct6_d16_5x1(),2nat);
    assert(r::pw(a_interlace_ct6_d16_5x1(),1nat)==1nat && r::ph(a_interlace_ct6_d16_5x1(),1nat)==1nat && r::rb(a_interlace_ct6_d16_5x1(),1nat)==8nat && r::base(a_interlace_ct6_d16_5x1(),1nat)==9nat) by (compute);
    assert(raw_interlace_ct6_d16_5x1()[9]==1u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_5x1(),a_interlace_ct6_d16_5x1(),1nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_5x1(),a_interlace_ct6_d16_5x1(),1nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_5x1(),a_interlace_ct6_d16_5x1(),1nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_5x1(),a_interlace_ct6_d16_5x1(),1nat);
    assert(r::pw(a_interlace_ct6_d16_5x1(),0nat)==1nat && r::ph(a_interlace_ct6_d16_5x1(),0nat)==1nat && r::rb(a_interlace_ct6_d16_5x1(),0nat)==8nat && r::base(a_interlace_ct6_d16_5x1(),0nat)==0nat) by (compute);
    assert(raw_interlace_ct6_d16_5x1()[0]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_5x1(),a_interlace_ct6_d16_5x1(),0nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_5x1(),a_interlace_ct6_d16_5x1(),0nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_5x1(),a_interlace_ct6_d16_5x1(),0nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_5x1(),a_interlace_ct6_d16_5x1(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_interlace_ct6_d16_5x1()
    ensures r::still(bytes_interlace_ct6_d16_5x1(),STRICT,false)==(r::Projection::Ready {width:5u32,height:1u32,bytes:20nat}),
{
    hide(bytes_interlace_ct6_d16_5x1);hide(z_interlace_ct6_d16_5x1);hide(raw_interlace_ct6_d16_5x1);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_interlace_ct6_d16_5x1().len()==112nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct6_d16_5x1())==Ok((crate::model::Ihdr {width:5u32,height:1u32,depth:16u8,color_type:6u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct6_d16_5x1())==Ok((crate::model::Ihdr {width:5u32,height:1u32,depth:16u8,color_type:6u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct6_d16_5x1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:55nat,next:100nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct6_d16_5x1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:55nat,next:100nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct6_d16_5x1(),100nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:108nat,len:0nat,next:112nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct6_d16_5x1(),100nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:108nat,len:0nat,next:112nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct6_d16_5x1().subrange(41,96)==z_interlace_ct6_d16_5x1()) by (compute);
    walk_interlace_ct6_d16_5x1(bytes_interlace_ct6_d16_5x1());
    inflate_interlace_ct6_d16_5x1();scan_interlace_ct6_d16_5x1();
    assert(r::raw(a_interlace_ct6_d16_5x1())==44nat) by (compute);
    assert(r::data(z_interlace_ct6_d16_5x1(),a_interlace_ct6_d16_5x1(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_interlace_ct6_d16_5x1(),h_interlace_ct6_d16_5x1(),z_interlace_ct6_d16_5x1(),0nat,false);
}
fn public_interlace_ct6_d16_5x1() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,5u8,0u8,0u8,0u8,1u8,16u8,6u8,0u8,0u8,1u8,49u8,105u8,136u8,38u8,0u8,0u8,0u8,55u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,44u8,0u8,211u8,255u8,0u8,120u8,193u8,130u8,10u8,237u8,34u8,223u8,182u8,1u8,87u8,141u8,58u8,175u8,165u8,32u8,46u8,134u8,2u8,246u8,181u8,232u8,19u8,221u8,101u8,184u8,36u8,3u8,25u8,176u8,158u8,11u8,39u8,83u8,115u8,169u8,28u8,252u8,208u8,43u8,185u8,193u8,224u8,245u8,182u8,42u8,20u8,228u8,206u8,95u8,35u8,250u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_interlace_ct6_d16_5x1());model_interlace_ct6_d16_5x1();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==5u32 && s.height==1u32 && s.rgba@.len()==20nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// interlace/ct3-d4-5x1; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_interlace_ct3_d4_5x1() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,5u8,0u8,0u8,0u8,1u8,4u8,3u8,0u8,0u8,1u8,147u8,215u8,73u8,86u8,0u8,0u8,0u8,48u8,80u8,76u8,84u8,69u8,46u8,39u8,214u8,171u8,90u8,127u8,169u8,19u8,13u8,145u8,106u8,212u8,19u8,229u8,18u8,255u8,102u8,165u8,147u8,116u8,179u8,14u8,105u8,220u8,149u8,215u8,123u8,11u8,255u8,65u8,144u8,74u8,73u8,227u8,67u8,94u8,118u8,224u8,157u8,159u8,235u8,74u8,99u8,222u8,157u8,20u8,121u8,123u8,68u8,20u8,215u8,220u8,0u8,0u8,0u8,16u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,216u8,192u8,88u8,192u8,244u8,128u8,185u8,24u8,0u8,10u8,47u8,2u8,122u8,28u8,110u8,199u8,221u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_interlace_ct3_d4_5x1() -> Seq<u8> {seq![120u8,156u8,99u8,216u8,192u8,88u8,192u8,244u8,128u8,185u8,24u8,0u8,10u8,47u8,2u8,122u8]}
spec fn h_interlace_ct3_d4_5x1() -> crate::model::Ihdr {crate::model::Ihdr {width:5u32,height:1u32,depth:4u8,color_type:3u8,interlace:1u8}}
spec fn a_interlace_ct3_d4_5x1() -> r::Raster {r::shape(h_interlace_ct3_d4_5x1(),5nat,1nat,16nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_interlace_ct3_d4_5x1(b: Seq<u8>)
    requires b.len()==133,crate::model::mh(b)==Ok((crate::model::Ihdr {width:5u32,height:1u32,depth:4u8,color_type:3u8,interlace:1u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:16nat,next:121nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,121nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:129nat,len:0nat,next:133nat,crc_ok:true,synthetic:false})),
        b.subrange(101,117)==z_interlace_ct3_d4_5x1(),
    ensures r::default_case(b,crate::model::Ihdr {width:5u32,height:1u32,depth:4u8,color_type:3u8,interlace:1u8},z_interlace_ct3_d4_5x1(),16nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);
    hide(z_interlace_ct3_d4_5x1);
    r::palette_default_case(b,crate::model::Ihdr {width:5u32,height:1u32,depth:4u8,color_type:3u8,interlace:1u8},z_interlace_ct3_d4_5x1(),16nat);
}
spec fn raw_interlace_ct3_d4_5x1() -> Seq<u8> {seq![0u8,176u8,1u8,112u8,2u8,224u8,3u8,115u8]}
spec fn ts_interlace_ct3_d4_5x1() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:36nat,before:1nat,after:2nat,symbol:176nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:36nat,next:44nat,before:2nat,after:3nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:44nat,next:52nat,before:3nat,after:4nat,symbol:112nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:52nat,next:60nat,before:4nat,after:5nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:60nat,next:69nat,before:5nat,after:6nat,symbol:224nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:69nat,next:77nat,before:6nat,after:7nat,symbol:3nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:77nat,next:85nat,before:7nat,after:8nat,symbol:115nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:85nat,next:92nat,before:8nat,after:8nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_interlace_ct3_d4_5x1()
    ensures t::trace(z_interlace_ct3_d4_5x1(),raw_interlace_ct3_d4_5x1(),ts_interlace_ct3_d4_5x1(),0,9nat),
{
    hide(z_interlace_ct3_d4_5x1);hide(raw_interlace_ct3_d4_5x1);hide(ts_interlace_ct3_d4_5x1);hide(t::valid_step);
    assert(t::trace(z_interlace_ct3_d4_5x1(),raw_interlace_ct3_d4_5x1(),ts_interlace_ct3_d4_5x1(),0,9nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_interlace_ct3_d4_5x1()
    ensures raw_interlace_ct3_d4_5x1().len()==8nat,
        z_interlace_ct3_d4_5x1().len()==16nat,
        z_interlace_ct3_d4_5x1()[0]==120u8,
        z_interlace_ct3_d4_5x1()[1]==156u8,
        zm::field_value(z_interlace_ct3_d4_5x1(),16,1)==1nat,
        zm::field_value(z_interlace_ct3_d4_5x1(),17,2)==1nat,
        zm::be_value(z_interlace_ct3_d4_5x1(),12nat,4)==170852986nat,
        ts_interlace_ct3_d4_5x1().len()==9nat,
        ts_interlace_ct3_d4_5x1()[0].pos==19nat,
        ts_interlace_ct3_d4_5x1()[0].before==0nat,
        ts_interlace_ct3_d4_5x1().last().next==92nat,
{
    hide(z_interlace_ct3_d4_5x1);hide(raw_interlace_ct3_d4_5x1);hide(ts_interlace_ct3_d4_5x1);
    assert(raw_interlace_ct3_d4_5x1().len()==8nat) by (compute);
    assert(z_interlace_ct3_d4_5x1().len()==16nat) by (compute);
    assert(z_interlace_ct3_d4_5x1()[0]==120u8) by (compute);
    assert(z_interlace_ct3_d4_5x1()[1]==156u8) by (compute);
    assert(zm::field_value(z_interlace_ct3_d4_5x1(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_interlace_ct3_d4_5x1(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_interlace_ct3_d4_5x1(),12nat,4)==170852986nat) by (compute);
    assert(ts_interlace_ct3_d4_5x1().len()==9nat) by (compute);
    assert(ts_interlace_ct3_d4_5x1()[0].pos==19nat) by (compute);
    assert(ts_interlace_ct3_d4_5x1()[0].before==0nat) by (compute);
    assert(ts_interlace_ct3_d4_5x1().last().next==92nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_interlace_ct3_d4_5x1()
    ensures zm::Adler(raw_interlace_ct3_d4_5x1())==170852986nat,
{
    hide(raw_interlace_ct3_d4_5x1);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_interlace_ct3_d4_5x1().len()==8nat) by (compute);
    assert(c::sums(raw_interlace_ct3_d4_5x1(),0,8nat)==(633nat,2607nat)) by (compute);
    c::sums_correct(raw_interlace_ct3_d4_5x1(),0,8nat);
    assert(raw_interlace_ct3_d4_5x1().subrange(0,8) =~= raw_interlace_ct3_d4_5x1());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((2607nat%65521)*(u16::MAX as nat+1)+(1+633nat)%65521==170852986nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_interlace_ct3_d4_5x1()
    ensures r::inflated(z_interlace_ct3_d4_5x1(),a_interlace_ct3_d4_5x1(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_interlace_ct3_d4_5x1(),r2:false}),
        raw_interlace_ct3_d4_5x1().len()==8nat,
{
    hide(z_interlace_ct3_d4_5x1);hide(raw_interlace_ct3_d4_5x1);hide(ts_interlace_ct3_d4_5x1);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_interlace_ct3_d4_5x1();checksum_interlace_ct3_d4_5x1();
    assert(r::raw(a_interlace_ct3_d4_5x1())==8nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_interlace_ct3_d4_5x1();
    stream::fixed_stream(z_interlace_ct3_d4_5x1(),raw_interlace_ct3_d4_5x1(),ts_interlace_ct3_d4_5x1(),8nat,156nat,170852986nat);
}
#[verifier::spinoff_prover]
proof fn scan_interlace_ct3_d4_5x1()
    ensures r::scan(raw_interlace_ct3_d4_5x1(),a_interlace_ct3_d4_5x1(),0)==Ok(()),
{
    hide(raw_interlace_ct3_d4_5x1);
    assert(r::count(a_interlace_ct3_d4_5x1())==7nat) by (compute);
    r::scan_step(raw_interlace_ct3_d4_5x1(),a_interlace_ct3_d4_5x1(),7);
    assert(r::pw(a_interlace_ct3_d4_5x1(),6nat)==5nat && r::ph(a_interlace_ct3_d4_5x1(),6nat)==0nat && r::rb(a_interlace_ct3_d4_5x1(),6nat)==3nat && r::base(a_interlace_ct3_d4_5x1(),6nat)==8nat) by (compute);
    r::pass_step(raw_interlace_ct3_d4_5x1(),a_interlace_ct3_d4_5x1(),6nat,0);
    r::scan_step(raw_interlace_ct3_d4_5x1(),a_interlace_ct3_d4_5x1(),6nat);
    assert(r::pw(a_interlace_ct3_d4_5x1(),5nat)==2nat && r::ph(a_interlace_ct3_d4_5x1(),5nat)==1nat && r::rb(a_interlace_ct3_d4_5x1(),5nat)==1nat && r::base(a_interlace_ct3_d4_5x1(),5nat)==6nat) by (compute);
    assert(raw_interlace_ct3_d4_5x1()[6]==3u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_5x1(),a_interlace_ct3_d4_5x1(),5nat,0nat),2nat);
    assert(r::row_result(raw_interlace_ct3_d4_5x1(),a_interlace_ct3_d4_5x1(),5nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_5x1(),a_interlace_ct3_d4_5x1(),5nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_5x1(),a_interlace_ct3_d4_5x1(),5nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_5x1(),a_interlace_ct3_d4_5x1(),5nat);
    assert(r::pw(a_interlace_ct3_d4_5x1(),4nat)==3nat && r::ph(a_interlace_ct3_d4_5x1(),4nat)==0nat && r::rb(a_interlace_ct3_d4_5x1(),4nat)==2nat && r::base(a_interlace_ct3_d4_5x1(),4nat)==6nat) by (compute);
    r::pass_step(raw_interlace_ct3_d4_5x1(),a_interlace_ct3_d4_5x1(),4nat,0);
    r::scan_step(raw_interlace_ct3_d4_5x1(),a_interlace_ct3_d4_5x1(),4nat);
    assert(r::pw(a_interlace_ct3_d4_5x1(),3nat)==1nat && r::ph(a_interlace_ct3_d4_5x1(),3nat)==1nat && r::rb(a_interlace_ct3_d4_5x1(),3nat)==1nat && r::base(a_interlace_ct3_d4_5x1(),3nat)==4nat) by (compute);
    assert(raw_interlace_ct3_d4_5x1()[4]==2u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_5x1(),a_interlace_ct3_d4_5x1(),3nat,0nat),1nat);
    assert(r::row_result(raw_interlace_ct3_d4_5x1(),a_interlace_ct3_d4_5x1(),3nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_5x1(),a_interlace_ct3_d4_5x1(),3nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_5x1(),a_interlace_ct3_d4_5x1(),3nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_5x1(),a_interlace_ct3_d4_5x1(),3nat);
    assert(r::pw(a_interlace_ct3_d4_5x1(),2nat)==2nat && r::ph(a_interlace_ct3_d4_5x1(),2nat)==0nat && r::rb(a_interlace_ct3_d4_5x1(),2nat)==1nat && r::base(a_interlace_ct3_d4_5x1(),2nat)==4nat) by (compute);
    r::pass_step(raw_interlace_ct3_d4_5x1(),a_interlace_ct3_d4_5x1(),2nat,0);
    r::scan_step(raw_interlace_ct3_d4_5x1(),a_interlace_ct3_d4_5x1(),2nat);
    assert(r::pw(a_interlace_ct3_d4_5x1(),1nat)==1nat && r::ph(a_interlace_ct3_d4_5x1(),1nat)==1nat && r::rb(a_interlace_ct3_d4_5x1(),1nat)==1nat && r::base(a_interlace_ct3_d4_5x1(),1nat)==2nat) by (compute);
    assert(raw_interlace_ct3_d4_5x1()[2]==1u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_5x1(),a_interlace_ct3_d4_5x1(),1nat,0nat),1nat);
    assert(r::row_result(raw_interlace_ct3_d4_5x1(),a_interlace_ct3_d4_5x1(),1nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_5x1(),a_interlace_ct3_d4_5x1(),1nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_5x1(),a_interlace_ct3_d4_5x1(),1nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_5x1(),a_interlace_ct3_d4_5x1(),1nat);
    assert(r::pw(a_interlace_ct3_d4_5x1(),0nat)==1nat && r::ph(a_interlace_ct3_d4_5x1(),0nat)==1nat && r::rb(a_interlace_ct3_d4_5x1(),0nat)==1nat && r::base(a_interlace_ct3_d4_5x1(),0nat)==0nat) by (compute);
    assert(raw_interlace_ct3_d4_5x1()[0]==0u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_5x1(),a_interlace_ct3_d4_5x1(),0nat,0nat),1nat);
    assert(r::row_result(raw_interlace_ct3_d4_5x1(),a_interlace_ct3_d4_5x1(),0nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_5x1(),a_interlace_ct3_d4_5x1(),0nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_5x1(),a_interlace_ct3_d4_5x1(),0nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_5x1(),a_interlace_ct3_d4_5x1(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_interlace_ct3_d4_5x1()
    ensures r::still(bytes_interlace_ct3_d4_5x1(),STRICT,false)==(r::Projection::Ready {width:5u32,height:1u32,bytes:20nat}),
{
    hide(bytes_interlace_ct3_d4_5x1);hide(z_interlace_ct3_d4_5x1);hide(raw_interlace_ct3_d4_5x1);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);
    assert(bytes_interlace_ct3_d4_5x1().len()==133nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct3_d4_5x1())==Ok((crate::model::Ihdr {width:5u32,height:1u32,depth:4u8,color_type:3u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct3_d4_5x1())==Ok((crate::model::Ihdr {width:5u32,height:1u32,depth:4u8,color_type:3u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 4u8)==16u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_5x1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_5x1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(48nat,3u8,4u8,false,false,false,false)==crate::metadata_model::Decision::Keep(16nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_5x1(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:16nat,next:121nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_5x1(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:16nat,next:121nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_5x1(),121nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:129nat,len:0nat,next:133nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_5x1(),121nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:129nat,len:0nat,next:133nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct3_d4_5x1().subrange(101,117)==z_interlace_ct3_d4_5x1()) by (compute);
    walk_interlace_ct3_d4_5x1(bytes_interlace_ct3_d4_5x1());
    inflate_interlace_ct3_d4_5x1();scan_interlace_ct3_d4_5x1();
    assert(r::raw(a_interlace_ct3_d4_5x1())==8nat) by (compute);
    assert(r::data(z_interlace_ct3_d4_5x1(),a_interlace_ct3_d4_5x1(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_interlace_ct3_d4_5x1(),h_interlace_ct3_d4_5x1(),z_interlace_ct3_d4_5x1(),16nat,false);
}
fn public_interlace_ct3_d4_5x1() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,5u8,0u8,0u8,0u8,1u8,4u8,3u8,0u8,0u8,1u8,147u8,215u8,73u8,86u8,0u8,0u8,0u8,48u8,80u8,76u8,84u8,69u8,46u8,39u8,214u8,171u8,90u8,127u8,169u8,19u8,13u8,145u8,106u8,212u8,19u8,229u8,18u8,255u8,102u8,165u8,147u8,116u8,179u8,14u8,105u8,220u8,149u8,215u8,123u8,11u8,255u8,65u8,144u8,74u8,73u8,227u8,67u8,94u8,118u8,224u8,157u8,159u8,235u8,74u8,99u8,222u8,157u8,20u8,121u8,123u8,68u8,20u8,215u8,220u8,0u8,0u8,0u8,16u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,216u8,192u8,88u8,192u8,244u8,128u8,185u8,24u8,0u8,10u8,47u8,2u8,122u8,28u8,110u8,199u8,221u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_interlace_ct3_d4_5x1());model_interlace_ct3_d4_5x1();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==5u32 && s.height==1u32 && s.rgba@.len()==20nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// interlace/ct0-d1-2x2; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_interlace_ct0_d1_2x2() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,1u8,0u8,0u8,0u8,1u8,45u8,202u8,0u8,31u8,0u8,0u8,0u8,14u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,104u8,96u8,100u8,96u8,114u8,0u8,0u8,2u8,206u8,0u8,196u8,187u8,87u8,157u8,8u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_interlace_ct0_d1_2x2() -> Seq<u8> {seq![120u8,156u8,99u8,104u8,96u8,100u8,96u8,114u8,0u8,0u8,2u8,206u8,0u8,196u8]}
spec fn h_interlace_ct0_d1_2x2() -> crate::model::Ihdr {crate::model::Ihdr {width:2u32,height:2u32,depth:1u8,color_type:0u8,interlace:1u8}}
spec fn a_interlace_ct0_d1_2x2() -> r::Raster {r::shape(h_interlace_ct0_d1_2x2(),2nat,2nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_interlace_ct0_d1_2x2(b: Seq<u8>)
    requires b.len()==71,crate::model::mh(b)==Ok((crate::model::Ihdr {width:2u32,height:2u32,depth:1u8,color_type:0u8,interlace:1u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:14nat,next:59nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,59nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:67nat,len:0nat,next:71nat,crc_ok:true,synthetic:false})),
        b.subrange(41,55)==z_interlace_ct0_d1_2x2(),
    ensures r::default_case(b,crate::model::Ihdr {width:2u32,height:2u32,depth:1u8,color_type:0u8,interlace:1u8},z_interlace_ct0_d1_2x2(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    hide(z_interlace_ct0_d1_2x2);
        assert((1u32 << 1u8)==2u32) by (compute);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_interlace_ct0_d1_2x2() -> Seq<u8> {seq![0u8,128u8,1u8,0u8,2u8,64u8]}
spec fn ts_interlace_ct0_d1_2x2() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:35nat,before:1nat,after:2nat,symbol:128nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:35nat,next:43nat,before:2nat,after:3nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:43nat,next:51nat,before:3nat,after:4nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:51nat,next:59nat,before:4nat,after:5nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:59nat,next:67nat,before:5nat,after:6nat,symbol:64nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:67nat,next:74nat,before:6nat,after:6nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_interlace_ct0_d1_2x2()
    ensures t::trace(z_interlace_ct0_d1_2x2(),raw_interlace_ct0_d1_2x2(),ts_interlace_ct0_d1_2x2(),0,7nat),
{
    hide(z_interlace_ct0_d1_2x2);hide(raw_interlace_ct0_d1_2x2);hide(ts_interlace_ct0_d1_2x2);hide(t::valid_step);
    assert(t::trace(z_interlace_ct0_d1_2x2(),raw_interlace_ct0_d1_2x2(),ts_interlace_ct0_d1_2x2(),0,7nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_interlace_ct0_d1_2x2()
    ensures raw_interlace_ct0_d1_2x2().len()==6nat,
        z_interlace_ct0_d1_2x2().len()==14nat,
        z_interlace_ct0_d1_2x2()[0]==120u8,
        z_interlace_ct0_d1_2x2()[1]==156u8,
        zm::field_value(z_interlace_ct0_d1_2x2(),16,1)==1nat,
        zm::field_value(z_interlace_ct0_d1_2x2(),17,2)==1nat,
        zm::be_value(z_interlace_ct0_d1_2x2(),10nat,4)==47055044nat,
        ts_interlace_ct0_d1_2x2().len()==7nat,
        ts_interlace_ct0_d1_2x2()[0].pos==19nat,
        ts_interlace_ct0_d1_2x2()[0].before==0nat,
        ts_interlace_ct0_d1_2x2().last().next==74nat,
{
    hide(z_interlace_ct0_d1_2x2);hide(raw_interlace_ct0_d1_2x2);hide(ts_interlace_ct0_d1_2x2);
    assert(raw_interlace_ct0_d1_2x2().len()==6nat) by (compute);
    assert(z_interlace_ct0_d1_2x2().len()==14nat) by (compute);
    assert(z_interlace_ct0_d1_2x2()[0]==120u8) by (compute);
    assert(z_interlace_ct0_d1_2x2()[1]==156u8) by (compute);
    assert(zm::field_value(z_interlace_ct0_d1_2x2(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_interlace_ct0_d1_2x2(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_interlace_ct0_d1_2x2(),10nat,4)==47055044nat) by (compute);
    assert(ts_interlace_ct0_d1_2x2().len()==7nat) by (compute);
    assert(ts_interlace_ct0_d1_2x2()[0].pos==19nat) by (compute);
    assert(ts_interlace_ct0_d1_2x2()[0].before==0nat) by (compute);
    assert(ts_interlace_ct0_d1_2x2().last().next==74nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_interlace_ct0_d1_2x2()
    ensures zm::Adler(raw_interlace_ct0_d1_2x2())==47055044nat,
{
    hide(raw_interlace_ct0_d1_2x2);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_interlace_ct0_d1_2x2().len()==6nat) by (compute);
    assert(c::sums(raw_interlace_ct0_d1_2x2(),0,6nat)==(195nat,718nat)) by (compute);
    c::sums_correct(raw_interlace_ct0_d1_2x2(),0,6nat);
    assert(raw_interlace_ct0_d1_2x2().subrange(0,6) =~= raw_interlace_ct0_d1_2x2());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((718nat%65521)*(u16::MAX as nat+1)+(1+195nat)%65521==47055044nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_interlace_ct0_d1_2x2()
    ensures r::inflated(z_interlace_ct0_d1_2x2(),a_interlace_ct0_d1_2x2(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_interlace_ct0_d1_2x2(),r2:false}),
        raw_interlace_ct0_d1_2x2().len()==6nat,
{
    hide(z_interlace_ct0_d1_2x2);hide(raw_interlace_ct0_d1_2x2);hide(ts_interlace_ct0_d1_2x2);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_interlace_ct0_d1_2x2();checksum_interlace_ct0_d1_2x2();
    assert(r::raw(a_interlace_ct0_d1_2x2())==6nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_interlace_ct0_d1_2x2();
    stream::fixed_stream(z_interlace_ct0_d1_2x2(),raw_interlace_ct0_d1_2x2(),ts_interlace_ct0_d1_2x2(),6nat,156nat,47055044nat);
}
#[verifier::spinoff_prover]
proof fn scan_interlace_ct0_d1_2x2()
    ensures r::scan(raw_interlace_ct0_d1_2x2(),a_interlace_ct0_d1_2x2(),0)==Ok(()),
{
    hide(raw_interlace_ct0_d1_2x2);
    assert(r::count(a_interlace_ct0_d1_2x2())==7nat) by (compute);
    r::scan_step(raw_interlace_ct0_d1_2x2(),a_interlace_ct0_d1_2x2(),7);
    assert(r::pw(a_interlace_ct0_d1_2x2(),6nat)==2nat && r::ph(a_interlace_ct0_d1_2x2(),6nat)==1nat && r::rb(a_interlace_ct0_d1_2x2(),6nat)==1nat && r::base(a_interlace_ct0_d1_2x2(),6nat)==4nat) by (compute);
    assert(raw_interlace_ct0_d1_2x2()[4]==2u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_2x2(),a_interlace_ct0_d1_2x2(),6nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_2x2(),a_interlace_ct0_d1_2x2(),6nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_2x2(),a_interlace_ct0_d1_2x2(),6nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_2x2(),a_interlace_ct0_d1_2x2(),6nat);
    assert(r::pw(a_interlace_ct0_d1_2x2(),5nat)==1nat && r::ph(a_interlace_ct0_d1_2x2(),5nat)==1nat && r::rb(a_interlace_ct0_d1_2x2(),5nat)==1nat && r::base(a_interlace_ct0_d1_2x2(),5nat)==2nat) by (compute);
    assert(raw_interlace_ct0_d1_2x2()[2]==1u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_2x2(),a_interlace_ct0_d1_2x2(),5nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_2x2(),a_interlace_ct0_d1_2x2(),5nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_2x2(),a_interlace_ct0_d1_2x2(),5nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_2x2(),a_interlace_ct0_d1_2x2(),5nat);
    assert(r::pw(a_interlace_ct0_d1_2x2(),4nat)==1nat && r::ph(a_interlace_ct0_d1_2x2(),4nat)==0nat && r::rb(a_interlace_ct0_d1_2x2(),4nat)==1nat && r::base(a_interlace_ct0_d1_2x2(),4nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct0_d1_2x2(),a_interlace_ct0_d1_2x2(),4nat,0);
    r::scan_step(raw_interlace_ct0_d1_2x2(),a_interlace_ct0_d1_2x2(),4nat);
    assert(r::pw(a_interlace_ct0_d1_2x2(),3nat)==0nat && r::ph(a_interlace_ct0_d1_2x2(),3nat)==1nat && r::rb(a_interlace_ct0_d1_2x2(),3nat)==0nat && r::base(a_interlace_ct0_d1_2x2(),3nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct0_d1_2x2(),a_interlace_ct0_d1_2x2(),3nat,0);
    r::scan_step(raw_interlace_ct0_d1_2x2(),a_interlace_ct0_d1_2x2(),3nat);
    assert(r::pw(a_interlace_ct0_d1_2x2(),2nat)==1nat && r::ph(a_interlace_ct0_d1_2x2(),2nat)==0nat && r::rb(a_interlace_ct0_d1_2x2(),2nat)==1nat && r::base(a_interlace_ct0_d1_2x2(),2nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct0_d1_2x2(),a_interlace_ct0_d1_2x2(),2nat,0);
    r::scan_step(raw_interlace_ct0_d1_2x2(),a_interlace_ct0_d1_2x2(),2nat);
    assert(r::pw(a_interlace_ct0_d1_2x2(),1nat)==0nat && r::ph(a_interlace_ct0_d1_2x2(),1nat)==1nat && r::rb(a_interlace_ct0_d1_2x2(),1nat)==0nat && r::base(a_interlace_ct0_d1_2x2(),1nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct0_d1_2x2(),a_interlace_ct0_d1_2x2(),1nat,0);
    r::scan_step(raw_interlace_ct0_d1_2x2(),a_interlace_ct0_d1_2x2(),1nat);
    assert(r::pw(a_interlace_ct0_d1_2x2(),0nat)==1nat && r::ph(a_interlace_ct0_d1_2x2(),0nat)==1nat && r::rb(a_interlace_ct0_d1_2x2(),0nat)==1nat && r::base(a_interlace_ct0_d1_2x2(),0nat)==0nat) by (compute);
    assert(raw_interlace_ct0_d1_2x2()[0]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_2x2(),a_interlace_ct0_d1_2x2(),0nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_2x2(),a_interlace_ct0_d1_2x2(),0nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_2x2(),a_interlace_ct0_d1_2x2(),0nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_2x2(),a_interlace_ct0_d1_2x2(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_interlace_ct0_d1_2x2()
    ensures r::still(bytes_interlace_ct0_d1_2x2(),STRICT,false)==(r::Projection::Ready {width:2u32,height:2u32,bytes:16nat}),
{
    hide(bytes_interlace_ct0_d1_2x2);hide(z_interlace_ct0_d1_2x2);hide(raw_interlace_ct0_d1_2x2);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_interlace_ct0_d1_2x2().len()==71nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct0_d1_2x2())==Ok((crate::model::Ihdr {width:2u32,height:2u32,depth:1u8,color_type:0u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct0_d1_2x2())==Ok((crate::model::Ihdr {width:2u32,height:2u32,depth:1u8,color_type:0u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 1u8)==2u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct0_d1_2x2(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:14nat,next:59nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct0_d1_2x2(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:14nat,next:59nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct0_d1_2x2(),59nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:67nat,len:0nat,next:71nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct0_d1_2x2(),59nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:67nat,len:0nat,next:71nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct0_d1_2x2().subrange(41,55)==z_interlace_ct0_d1_2x2()) by (compute);
    walk_interlace_ct0_d1_2x2(bytes_interlace_ct0_d1_2x2());
    inflate_interlace_ct0_d1_2x2();scan_interlace_ct0_d1_2x2();
    assert(r::raw(a_interlace_ct0_d1_2x2())==6nat) by (compute);
    assert(r::data(z_interlace_ct0_d1_2x2(),a_interlace_ct0_d1_2x2(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_interlace_ct0_d1_2x2(),h_interlace_ct0_d1_2x2(),z_interlace_ct0_d1_2x2(),0nat,false);
}
fn public_interlace_ct0_d1_2x2() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,1u8,0u8,0u8,0u8,1u8,45u8,202u8,0u8,31u8,0u8,0u8,0u8,14u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,104u8,96u8,100u8,96u8,114u8,0u8,0u8,2u8,206u8,0u8,196u8,187u8,87u8,157u8,8u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_interlace_ct0_d1_2x2());model_interlace_ct0_d1_2x2();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==2u32 && s.height==2u32 && s.rgba@.len()==16nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// interlace/ct6-d16-2x2; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_interlace_ct6_d16_2x2() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,16u8,6u8,0u8,0u8,1u8,85u8,33u8,225u8,241u8,0u8,0u8,0u8,44u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,240u8,121u8,214u8,239u8,92u8,31u8,35u8,93u8,203u8,104u8,187u8,76u8,42u8,220u8,172u8,147u8,95u8,149u8,233u8,138u8,70u8,197u8,50u8,205u8,19u8,18u8,179u8,126u8,60u8,201u8,208u8,231u8,43u8,57u8,191u8,16u8,0u8,226u8,87u8,13u8,228u8,180u8,246u8,129u8,27u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_interlace_ct6_d16_2x2() -> Seq<u8> {seq![120u8,156u8,99u8,240u8,121u8,214u8,239u8,92u8,31u8,35u8,93u8,203u8,104u8,187u8,76u8,42u8,220u8,172u8,147u8,95u8,149u8,233u8,138u8,70u8,197u8,50u8,205u8,19u8,18u8,179u8,126u8,60u8,201u8,208u8,231u8,43u8,57u8,191u8,16u8,0u8,226u8,87u8,13u8,228u8]}
spec fn h_interlace_ct6_d16_2x2() -> crate::model::Ihdr {crate::model::Ihdr {width:2u32,height:2u32,depth:16u8,color_type:6u8,interlace:1u8}}
spec fn a_interlace_ct6_d16_2x2() -> r::Raster {r::shape(h_interlace_ct6_d16_2x2(),2nat,2nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_interlace_ct6_d16_2x2(b: Seq<u8>)
    requires b.len()==101,crate::model::mh(b)==Ok((crate::model::Ihdr {width:2u32,height:2u32,depth:16u8,color_type:6u8,interlace:1u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:44nat,next:89nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,89nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:97nat,len:0nat,next:101nat,crc_ok:true,synthetic:false})),
        b.subrange(41,85)==z_interlace_ct6_d16_2x2(),
    ensures r::default_case(b,crate::model::Ihdr {width:2u32,height:2u32,depth:16u8,color_type:6u8,interlace:1u8},z_interlace_ct6_d16_2x2(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    hide(z_interlace_ct6_d16_2x2);
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_interlace_ct6_d16_2x2() -> Seq<u8> {seq![0u8,76u8,230u8,143u8,67u8,127u8,92u8,27u8,125u8,1u8,61u8,166u8,26u8,87u8,54u8,137u8,15u8,37u8,2u8,212u8,40u8,120u8,166u8,41u8,200u8,24u8,154u8,248u8,228u8,104u8,47u8,14u8,116u8,207u8,161u8]}
spec fn ts_interlace_ct6_d16_2x2() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:35nat,before:1nat,after:2nat,symbol:76nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:35nat,next:44nat,before:2nat,after:3nat,symbol:230nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:44nat,next:52nat,before:3nat,after:4nat,symbol:143nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:52nat,next:60nat,before:4nat,after:5nat,symbol:67nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:60nat,next:68nat,before:5nat,after:6nat,symbol:127nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:68nat,next:76nat,before:6nat,after:7nat,symbol:92nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:76nat,next:84nat,before:7nat,after:8nat,symbol:27nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:84nat,next:92nat,before:8nat,after:9nat,symbol:125nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:92nat,next:100nat,before:9nat,after:10nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:100nat,next:108nat,before:10nat,after:11nat,symbol:61nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:108nat,next:117nat,before:11nat,after:12nat,symbol:166nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:117nat,next:125nat,before:12nat,after:13nat,symbol:26nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:125nat,next:133nat,before:13nat,after:14nat,symbol:87nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:133nat,next:141nat,before:14nat,after:15nat,symbol:54nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:141nat,next:149nat,before:15nat,after:16nat,symbol:137nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:149nat,next:157nat,before:16nat,after:17nat,symbol:15nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:157nat,next:165nat,before:17nat,after:18nat,symbol:37nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:165nat,next:173nat,before:18nat,after:19nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:173nat,next:182nat,before:19nat,after:20nat,symbol:212nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:182nat,next:190nat,before:20nat,after:21nat,symbol:40nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:190nat,next:198nat,before:21nat,after:22nat,symbol:120nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:198nat,next:207nat,before:22nat,after:23nat,symbol:166nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:207nat,next:215nat,before:23nat,after:24nat,symbol:41nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:215nat,next:224nat,before:24nat,after:25nat,symbol:200nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:224nat,next:232nat,before:25nat,after:26nat,symbol:24nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:232nat,next:241nat,before:26nat,after:27nat,symbol:154nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:241nat,next:250nat,before:27nat,after:28nat,symbol:248nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:250nat,next:259nat,before:28nat,after:29nat,symbol:228nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:259nat,next:267nat,before:29nat,after:30nat,symbol:104nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:267nat,next:275nat,before:30nat,after:31nat,symbol:47nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:275nat,next:283nat,before:31nat,after:32nat,symbol:14nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:283nat,next:291nat,before:32nat,after:33nat,symbol:116nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:291nat,next:300nat,before:33nat,after:34nat,symbol:207nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:300nat,next:309nat,before:34nat,after:35nat,symbol:161nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:309nat,next:316nat,before:35nat,after:35nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_interlace_ct6_d16_2x2()
    ensures t::trace(z_interlace_ct6_d16_2x2(),raw_interlace_ct6_d16_2x2(),ts_interlace_ct6_d16_2x2(),0,36nat),
{
    hide(z_interlace_ct6_d16_2x2);hide(raw_interlace_ct6_d16_2x2);hide(ts_interlace_ct6_d16_2x2);hide(t::valid_step);
    assert(t::trace(z_interlace_ct6_d16_2x2(),raw_interlace_ct6_d16_2x2(),ts_interlace_ct6_d16_2x2(),0,36nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_interlace_ct6_d16_2x2()
    ensures raw_interlace_ct6_d16_2x2().len()==35nat,
        z_interlace_ct6_d16_2x2().len()==44nat,
        z_interlace_ct6_d16_2x2()[0]==120u8,
        z_interlace_ct6_d16_2x2()[1]==156u8,
        zm::field_value(z_interlace_ct6_d16_2x2(),16,1)==1nat,
        zm::field_value(z_interlace_ct6_d16_2x2(),17,2)==1nat,
        zm::be_value(z_interlace_ct6_d16_2x2(),40nat,4)==3797356004nat,
        ts_interlace_ct6_d16_2x2().len()==36nat,
        ts_interlace_ct6_d16_2x2()[0].pos==19nat,
        ts_interlace_ct6_d16_2x2()[0].before==0nat,
        ts_interlace_ct6_d16_2x2().last().next==316nat,
{
    hide(z_interlace_ct6_d16_2x2);hide(raw_interlace_ct6_d16_2x2);hide(ts_interlace_ct6_d16_2x2);
    assert(raw_interlace_ct6_d16_2x2().len()==35nat) by (compute);
    assert(z_interlace_ct6_d16_2x2().len()==44nat) by (compute);
    assert(z_interlace_ct6_d16_2x2()[0]==120u8) by (compute);
    assert(z_interlace_ct6_d16_2x2()[1]==156u8) by (compute);
    assert(zm::field_value(z_interlace_ct6_d16_2x2(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_interlace_ct6_d16_2x2(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_interlace_ct6_d16_2x2(),40nat,4)==3797356004nat) by (compute);
    assert(ts_interlace_ct6_d16_2x2().len()==36nat) by (compute);
    assert(ts_interlace_ct6_d16_2x2()[0].pos==19nat) by (compute);
    assert(ts_interlace_ct6_d16_2x2()[0].before==0nat) by (compute);
    assert(ts_interlace_ct6_d16_2x2().last().next==316nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_interlace_ct6_d16_2x2()
    ensures zm::Adler(raw_interlace_ct6_d16_2x2())==3797356004nat,
{
    hide(raw_interlace_ct6_d16_2x2);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_interlace_ct6_d16_2x2().len()==35nat) by (compute);
    assert(c::sums(raw_interlace_ct6_d16_2x2(),0,35nat)==(3555nat,57943nat)) by (compute);
    c::sums_correct(raw_interlace_ct6_d16_2x2(),0,35nat);
    assert(raw_interlace_ct6_d16_2x2().subrange(0,35) =~= raw_interlace_ct6_d16_2x2());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((57943nat%65521)*(u16::MAX as nat+1)+(1+3555nat)%65521==3797356004nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_interlace_ct6_d16_2x2()
    ensures r::inflated(z_interlace_ct6_d16_2x2(),a_interlace_ct6_d16_2x2(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_interlace_ct6_d16_2x2(),r2:false}),
        raw_interlace_ct6_d16_2x2().len()==35nat,
{
    hide(z_interlace_ct6_d16_2x2);hide(raw_interlace_ct6_d16_2x2);hide(ts_interlace_ct6_d16_2x2);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_interlace_ct6_d16_2x2();checksum_interlace_ct6_d16_2x2();
    assert(r::raw(a_interlace_ct6_d16_2x2())==35nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_interlace_ct6_d16_2x2();
    stream::fixed_stream(z_interlace_ct6_d16_2x2(),raw_interlace_ct6_d16_2x2(),ts_interlace_ct6_d16_2x2(),35nat,156nat,3797356004nat);
}
#[verifier::spinoff_prover]
proof fn scan_interlace_ct6_d16_2x2()
    ensures r::scan(raw_interlace_ct6_d16_2x2(),a_interlace_ct6_d16_2x2(),0)==Ok(()),
{
    hide(raw_interlace_ct6_d16_2x2);
    assert(r::count(a_interlace_ct6_d16_2x2())==7nat) by (compute);
    r::scan_step(raw_interlace_ct6_d16_2x2(),a_interlace_ct6_d16_2x2(),7);
    assert(r::pw(a_interlace_ct6_d16_2x2(),6nat)==2nat && r::ph(a_interlace_ct6_d16_2x2(),6nat)==1nat && r::rb(a_interlace_ct6_d16_2x2(),6nat)==16nat && r::base(a_interlace_ct6_d16_2x2(),6nat)==18nat) by (compute);
    assert(raw_interlace_ct6_d16_2x2()[18]==2u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_2x2(),a_interlace_ct6_d16_2x2(),6nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_2x2(),a_interlace_ct6_d16_2x2(),6nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_2x2(),a_interlace_ct6_d16_2x2(),6nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_2x2(),a_interlace_ct6_d16_2x2(),6nat);
    assert(r::pw(a_interlace_ct6_d16_2x2(),5nat)==1nat && r::ph(a_interlace_ct6_d16_2x2(),5nat)==1nat && r::rb(a_interlace_ct6_d16_2x2(),5nat)==8nat && r::base(a_interlace_ct6_d16_2x2(),5nat)==9nat) by (compute);
    assert(raw_interlace_ct6_d16_2x2()[9]==1u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_2x2(),a_interlace_ct6_d16_2x2(),5nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_2x2(),a_interlace_ct6_d16_2x2(),5nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_2x2(),a_interlace_ct6_d16_2x2(),5nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_2x2(),a_interlace_ct6_d16_2x2(),5nat);
    assert(r::pw(a_interlace_ct6_d16_2x2(),4nat)==1nat && r::ph(a_interlace_ct6_d16_2x2(),4nat)==0nat && r::rb(a_interlace_ct6_d16_2x2(),4nat)==8nat && r::base(a_interlace_ct6_d16_2x2(),4nat)==9nat) by (compute);
    r::pass_step(raw_interlace_ct6_d16_2x2(),a_interlace_ct6_d16_2x2(),4nat,0);
    r::scan_step(raw_interlace_ct6_d16_2x2(),a_interlace_ct6_d16_2x2(),4nat);
    assert(r::pw(a_interlace_ct6_d16_2x2(),3nat)==0nat && r::ph(a_interlace_ct6_d16_2x2(),3nat)==1nat && r::rb(a_interlace_ct6_d16_2x2(),3nat)==0nat && r::base(a_interlace_ct6_d16_2x2(),3nat)==9nat) by (compute);
    r::pass_step(raw_interlace_ct6_d16_2x2(),a_interlace_ct6_d16_2x2(),3nat,0);
    r::scan_step(raw_interlace_ct6_d16_2x2(),a_interlace_ct6_d16_2x2(),3nat);
    assert(r::pw(a_interlace_ct6_d16_2x2(),2nat)==1nat && r::ph(a_interlace_ct6_d16_2x2(),2nat)==0nat && r::rb(a_interlace_ct6_d16_2x2(),2nat)==8nat && r::base(a_interlace_ct6_d16_2x2(),2nat)==9nat) by (compute);
    r::pass_step(raw_interlace_ct6_d16_2x2(),a_interlace_ct6_d16_2x2(),2nat,0);
    r::scan_step(raw_interlace_ct6_d16_2x2(),a_interlace_ct6_d16_2x2(),2nat);
    assert(r::pw(a_interlace_ct6_d16_2x2(),1nat)==0nat && r::ph(a_interlace_ct6_d16_2x2(),1nat)==1nat && r::rb(a_interlace_ct6_d16_2x2(),1nat)==0nat && r::base(a_interlace_ct6_d16_2x2(),1nat)==9nat) by (compute);
    r::pass_step(raw_interlace_ct6_d16_2x2(),a_interlace_ct6_d16_2x2(),1nat,0);
    r::scan_step(raw_interlace_ct6_d16_2x2(),a_interlace_ct6_d16_2x2(),1nat);
    assert(r::pw(a_interlace_ct6_d16_2x2(),0nat)==1nat && r::ph(a_interlace_ct6_d16_2x2(),0nat)==1nat && r::rb(a_interlace_ct6_d16_2x2(),0nat)==8nat && r::base(a_interlace_ct6_d16_2x2(),0nat)==0nat) by (compute);
    assert(raw_interlace_ct6_d16_2x2()[0]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_2x2(),a_interlace_ct6_d16_2x2(),0nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_2x2(),a_interlace_ct6_d16_2x2(),0nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_2x2(),a_interlace_ct6_d16_2x2(),0nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_2x2(),a_interlace_ct6_d16_2x2(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_interlace_ct6_d16_2x2()
    ensures r::still(bytes_interlace_ct6_d16_2x2(),STRICT,false)==(r::Projection::Ready {width:2u32,height:2u32,bytes:16nat}),
{
    hide(bytes_interlace_ct6_d16_2x2);hide(z_interlace_ct6_d16_2x2);hide(raw_interlace_ct6_d16_2x2);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_interlace_ct6_d16_2x2().len()==101nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct6_d16_2x2())==Ok((crate::model::Ihdr {width:2u32,height:2u32,depth:16u8,color_type:6u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct6_d16_2x2())==Ok((crate::model::Ihdr {width:2u32,height:2u32,depth:16u8,color_type:6u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct6_d16_2x2(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:44nat,next:89nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct6_d16_2x2(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:44nat,next:89nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct6_d16_2x2(),89nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:97nat,len:0nat,next:101nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct6_d16_2x2(),89nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:97nat,len:0nat,next:101nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct6_d16_2x2().subrange(41,85)==z_interlace_ct6_d16_2x2()) by (compute);
    walk_interlace_ct6_d16_2x2(bytes_interlace_ct6_d16_2x2());
    inflate_interlace_ct6_d16_2x2();scan_interlace_ct6_d16_2x2();
    assert(r::raw(a_interlace_ct6_d16_2x2())==35nat) by (compute);
    assert(r::data(z_interlace_ct6_d16_2x2(),a_interlace_ct6_d16_2x2(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_interlace_ct6_d16_2x2(),h_interlace_ct6_d16_2x2(),z_interlace_ct6_d16_2x2(),0nat,false);
}
fn public_interlace_ct6_d16_2x2() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,16u8,6u8,0u8,0u8,1u8,85u8,33u8,225u8,241u8,0u8,0u8,0u8,44u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,240u8,121u8,214u8,239u8,92u8,31u8,35u8,93u8,203u8,104u8,187u8,76u8,42u8,220u8,172u8,147u8,95u8,149u8,233u8,138u8,70u8,197u8,50u8,205u8,19u8,18u8,179u8,126u8,60u8,201u8,208u8,231u8,43u8,57u8,191u8,16u8,0u8,226u8,87u8,13u8,228u8,180u8,246u8,129u8,27u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_interlace_ct6_d16_2x2());model_interlace_ct6_d16_2x2();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==2u32 && s.height==2u32 && s.rgba@.len()==16nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// interlace/ct3-d4-2x2; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_interlace_ct3_d4_2x2() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,4u8,3u8,0u8,0u8,1u8,247u8,159u8,32u8,129u8,0u8,0u8,0u8,48u8,80u8,76u8,84u8,69u8,194u8,155u8,159u8,253u8,215u8,50u8,11u8,140u8,194u8,168u8,145u8,56u8,151u8,195u8,123u8,17u8,249u8,103u8,143u8,55u8,212u8,184u8,212u8,105u8,207u8,41u8,49u8,81u8,122u8,2u8,25u8,134u8,96u8,239u8,125u8,219u8,110u8,210u8,237u8,228u8,162u8,17u8,50u8,150u8,193u8,202u8,116u8,25u8,70u8,49u8,9u8,56u8,0u8,0u8,0u8,14u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,80u8,96u8,60u8,192u8,212u8,2u8,0u8,3u8,114u8,1u8,104u8,145u8,9u8,202u8,127u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_interlace_ct3_d4_2x2() -> Seq<u8> {seq![120u8,156u8,99u8,80u8,96u8,60u8,192u8,212u8,2u8,0u8,3u8,114u8,1u8,104u8]}
spec fn h_interlace_ct3_d4_2x2() -> crate::model::Ihdr {crate::model::Ihdr {width:2u32,height:2u32,depth:4u8,color_type:3u8,interlace:1u8}}
spec fn a_interlace_ct3_d4_2x2() -> r::Raster {r::shape(h_interlace_ct3_d4_2x2(),2nat,2nat,16nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_interlace_ct3_d4_2x2(b: Seq<u8>)
    requires b.len()==131,crate::model::mh(b)==Ok((crate::model::Ihdr {width:2u32,height:2u32,depth:4u8,color_type:3u8,interlace:1u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:14nat,next:119nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,119nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:127nat,len:0nat,next:131nat,crc_ok:true,synthetic:false})),
        b.subrange(101,115)==z_interlace_ct3_d4_2x2(),
    ensures r::default_case(b,crate::model::Ihdr {width:2u32,height:2u32,depth:4u8,color_type:3u8,interlace:1u8},z_interlace_ct3_d4_2x2(),16nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);
    hide(z_interlace_ct3_d4_2x2);
    r::palette_default_case(b,crate::model::Ihdr {width:2u32,height:2u32,depth:4u8,color_type:3u8,interlace:1u8},z_interlace_ct3_d4_2x2(),14nat);
}
spec fn raw_interlace_ct3_d4_2x2() -> Seq<u8> {seq![0u8,32u8,1u8,192u8,2u8,132u8]}
spec fn ts_interlace_ct3_d4_2x2() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:35nat,before:1nat,after:2nat,symbol:32nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:35nat,next:43nat,before:2nat,after:3nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:43nat,next:52nat,before:3nat,after:4nat,symbol:192nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:52nat,next:60nat,before:4nat,after:5nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:60nat,next:68nat,before:5nat,after:6nat,symbol:132nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:68nat,next:75nat,before:6nat,after:6nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_interlace_ct3_d4_2x2()
    ensures t::trace(z_interlace_ct3_d4_2x2(),raw_interlace_ct3_d4_2x2(),ts_interlace_ct3_d4_2x2(),0,7nat),
{
    hide(z_interlace_ct3_d4_2x2);hide(raw_interlace_ct3_d4_2x2);hide(ts_interlace_ct3_d4_2x2);hide(t::valid_step);
    assert(t::trace(z_interlace_ct3_d4_2x2(),raw_interlace_ct3_d4_2x2(),ts_interlace_ct3_d4_2x2(),0,7nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_interlace_ct3_d4_2x2()
    ensures raw_interlace_ct3_d4_2x2().len()==6nat,
        z_interlace_ct3_d4_2x2().len()==14nat,
        z_interlace_ct3_d4_2x2()[0]==120u8,
        z_interlace_ct3_d4_2x2()[1]==156u8,
        zm::field_value(z_interlace_ct3_d4_2x2(),16,1)==1nat,
        zm::field_value(z_interlace_ct3_d4_2x2(),17,2)==1nat,
        zm::be_value(z_interlace_ct3_d4_2x2(),10nat,4)==57803112nat,
        ts_interlace_ct3_d4_2x2().len()==7nat,
        ts_interlace_ct3_d4_2x2()[0].pos==19nat,
        ts_interlace_ct3_d4_2x2()[0].before==0nat,
        ts_interlace_ct3_d4_2x2().last().next==75nat,
{
    hide(z_interlace_ct3_d4_2x2);hide(raw_interlace_ct3_d4_2x2);hide(ts_interlace_ct3_d4_2x2);
    assert(raw_interlace_ct3_d4_2x2().len()==6nat) by (compute);
    assert(z_interlace_ct3_d4_2x2().len()==14nat) by (compute);
    assert(z_interlace_ct3_d4_2x2()[0]==120u8) by (compute);
    assert(z_interlace_ct3_d4_2x2()[1]==156u8) by (compute);
    assert(zm::field_value(z_interlace_ct3_d4_2x2(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_interlace_ct3_d4_2x2(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_interlace_ct3_d4_2x2(),10nat,4)==57803112nat) by (compute);
    assert(ts_interlace_ct3_d4_2x2().len()==7nat) by (compute);
    assert(ts_interlace_ct3_d4_2x2()[0].pos==19nat) by (compute);
    assert(ts_interlace_ct3_d4_2x2()[0].before==0nat) by (compute);
    assert(ts_interlace_ct3_d4_2x2().last().next==75nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_interlace_ct3_d4_2x2()
    ensures zm::Adler(raw_interlace_ct3_d4_2x2())==57803112nat,
{
    hide(raw_interlace_ct3_d4_2x2);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_interlace_ct3_d4_2x2().len()==6nat) by (compute);
    assert(c::sums(raw_interlace_ct3_d4_2x2(),0,6nat)==(359nat,882nat)) by (compute);
    c::sums_correct(raw_interlace_ct3_d4_2x2(),0,6nat);
    assert(raw_interlace_ct3_d4_2x2().subrange(0,6) =~= raw_interlace_ct3_d4_2x2());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((882nat%65521)*(u16::MAX as nat+1)+(1+359nat)%65521==57803112nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_interlace_ct3_d4_2x2()
    ensures r::inflated(z_interlace_ct3_d4_2x2(),a_interlace_ct3_d4_2x2(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_interlace_ct3_d4_2x2(),r2:false}),
        raw_interlace_ct3_d4_2x2().len()==6nat,
{
    hide(z_interlace_ct3_d4_2x2);hide(raw_interlace_ct3_d4_2x2);hide(ts_interlace_ct3_d4_2x2);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_interlace_ct3_d4_2x2();checksum_interlace_ct3_d4_2x2();
    assert(r::raw(a_interlace_ct3_d4_2x2())==6nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_interlace_ct3_d4_2x2();
    stream::fixed_stream(z_interlace_ct3_d4_2x2(),raw_interlace_ct3_d4_2x2(),ts_interlace_ct3_d4_2x2(),6nat,156nat,57803112nat);
}
#[verifier::spinoff_prover]
proof fn scan_interlace_ct3_d4_2x2()
    ensures r::scan(raw_interlace_ct3_d4_2x2(),a_interlace_ct3_d4_2x2(),0)==Ok(()),
{
    hide(raw_interlace_ct3_d4_2x2);
    assert(r::count(a_interlace_ct3_d4_2x2())==7nat) by (compute);
    r::scan_step(raw_interlace_ct3_d4_2x2(),a_interlace_ct3_d4_2x2(),7);
    assert(r::pw(a_interlace_ct3_d4_2x2(),6nat)==2nat && r::ph(a_interlace_ct3_d4_2x2(),6nat)==1nat && r::rb(a_interlace_ct3_d4_2x2(),6nat)==1nat && r::base(a_interlace_ct3_d4_2x2(),6nat)==4nat) by (compute);
    assert(raw_interlace_ct3_d4_2x2()[4]==2u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_2x2(),a_interlace_ct3_d4_2x2(),6nat,0nat),2nat);
    assert(r::row_result(raw_interlace_ct3_d4_2x2(),a_interlace_ct3_d4_2x2(),6nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_2x2(),a_interlace_ct3_d4_2x2(),6nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_2x2(),a_interlace_ct3_d4_2x2(),6nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_2x2(),a_interlace_ct3_d4_2x2(),6nat);
    assert(r::pw(a_interlace_ct3_d4_2x2(),5nat)==1nat && r::ph(a_interlace_ct3_d4_2x2(),5nat)==1nat && r::rb(a_interlace_ct3_d4_2x2(),5nat)==1nat && r::base(a_interlace_ct3_d4_2x2(),5nat)==2nat) by (compute);
    assert(raw_interlace_ct3_d4_2x2()[2]==1u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_2x2(),a_interlace_ct3_d4_2x2(),5nat,0nat),1nat);
    assert(r::row_result(raw_interlace_ct3_d4_2x2(),a_interlace_ct3_d4_2x2(),5nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_2x2(),a_interlace_ct3_d4_2x2(),5nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_2x2(),a_interlace_ct3_d4_2x2(),5nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_2x2(),a_interlace_ct3_d4_2x2(),5nat);
    assert(r::pw(a_interlace_ct3_d4_2x2(),4nat)==1nat && r::ph(a_interlace_ct3_d4_2x2(),4nat)==0nat && r::rb(a_interlace_ct3_d4_2x2(),4nat)==1nat && r::base(a_interlace_ct3_d4_2x2(),4nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct3_d4_2x2(),a_interlace_ct3_d4_2x2(),4nat,0);
    r::scan_step(raw_interlace_ct3_d4_2x2(),a_interlace_ct3_d4_2x2(),4nat);
    assert(r::pw(a_interlace_ct3_d4_2x2(),3nat)==0nat && r::ph(a_interlace_ct3_d4_2x2(),3nat)==1nat && r::rb(a_interlace_ct3_d4_2x2(),3nat)==0nat && r::base(a_interlace_ct3_d4_2x2(),3nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct3_d4_2x2(),a_interlace_ct3_d4_2x2(),3nat,0);
    r::scan_step(raw_interlace_ct3_d4_2x2(),a_interlace_ct3_d4_2x2(),3nat);
    assert(r::pw(a_interlace_ct3_d4_2x2(),2nat)==1nat && r::ph(a_interlace_ct3_d4_2x2(),2nat)==0nat && r::rb(a_interlace_ct3_d4_2x2(),2nat)==1nat && r::base(a_interlace_ct3_d4_2x2(),2nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct3_d4_2x2(),a_interlace_ct3_d4_2x2(),2nat,0);
    r::scan_step(raw_interlace_ct3_d4_2x2(),a_interlace_ct3_d4_2x2(),2nat);
    assert(r::pw(a_interlace_ct3_d4_2x2(),1nat)==0nat && r::ph(a_interlace_ct3_d4_2x2(),1nat)==1nat && r::rb(a_interlace_ct3_d4_2x2(),1nat)==0nat && r::base(a_interlace_ct3_d4_2x2(),1nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct3_d4_2x2(),a_interlace_ct3_d4_2x2(),1nat,0);
    r::scan_step(raw_interlace_ct3_d4_2x2(),a_interlace_ct3_d4_2x2(),1nat);
    assert(r::pw(a_interlace_ct3_d4_2x2(),0nat)==1nat && r::ph(a_interlace_ct3_d4_2x2(),0nat)==1nat && r::rb(a_interlace_ct3_d4_2x2(),0nat)==1nat && r::base(a_interlace_ct3_d4_2x2(),0nat)==0nat) by (compute);
    assert(raw_interlace_ct3_d4_2x2()[0]==0u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_2x2(),a_interlace_ct3_d4_2x2(),0nat,0nat),1nat);
    assert(r::row_result(raw_interlace_ct3_d4_2x2(),a_interlace_ct3_d4_2x2(),0nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_2x2(),a_interlace_ct3_d4_2x2(),0nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_2x2(),a_interlace_ct3_d4_2x2(),0nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_2x2(),a_interlace_ct3_d4_2x2(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_interlace_ct3_d4_2x2()
    ensures r::still(bytes_interlace_ct3_d4_2x2(),STRICT,false)==(r::Projection::Ready {width:2u32,height:2u32,bytes:16nat}),
{
    hide(bytes_interlace_ct3_d4_2x2);hide(z_interlace_ct3_d4_2x2);hide(raw_interlace_ct3_d4_2x2);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);
    assert(bytes_interlace_ct3_d4_2x2().len()==131nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct3_d4_2x2())==Ok((crate::model::Ihdr {width:2u32,height:2u32,depth:4u8,color_type:3u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct3_d4_2x2())==Ok((crate::model::Ihdr {width:2u32,height:2u32,depth:4u8,color_type:3u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 4u8)==16u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_2x2(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_2x2(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(48nat,3u8,4u8,false,false,false,false)==crate::metadata_model::Decision::Keep(16nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_2x2(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:14nat,next:119nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_2x2(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:14nat,next:119nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_2x2(),119nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:127nat,len:0nat,next:131nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_2x2(),119nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:127nat,len:0nat,next:131nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct3_d4_2x2().subrange(101,115)==z_interlace_ct3_d4_2x2()) by (compute);
    walk_interlace_ct3_d4_2x2(bytes_interlace_ct3_d4_2x2());
    inflate_interlace_ct3_d4_2x2();scan_interlace_ct3_d4_2x2();
    assert(r::raw(a_interlace_ct3_d4_2x2())==6nat) by (compute);
    assert(r::data(z_interlace_ct3_d4_2x2(),a_interlace_ct3_d4_2x2(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_interlace_ct3_d4_2x2(),h_interlace_ct3_d4_2x2(),z_interlace_ct3_d4_2x2(),16nat,false);
}
fn public_interlace_ct3_d4_2x2() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,4u8,3u8,0u8,0u8,1u8,247u8,159u8,32u8,129u8,0u8,0u8,0u8,48u8,80u8,76u8,84u8,69u8,194u8,155u8,159u8,253u8,215u8,50u8,11u8,140u8,194u8,168u8,145u8,56u8,151u8,195u8,123u8,17u8,249u8,103u8,143u8,55u8,212u8,184u8,212u8,105u8,207u8,41u8,49u8,81u8,122u8,2u8,25u8,134u8,96u8,239u8,125u8,219u8,110u8,210u8,237u8,228u8,162u8,17u8,50u8,150u8,193u8,202u8,116u8,25u8,70u8,49u8,9u8,56u8,0u8,0u8,0u8,14u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,80u8,96u8,60u8,192u8,212u8,2u8,0u8,3u8,114u8,1u8,104u8,145u8,9u8,202u8,127u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_interlace_ct3_d4_2x2());model_interlace_ct3_d4_2x2();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==2u32 && s.height==2u32 && s.rgba@.len()==16nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// interlace/ct0-d1-3x3; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_interlace_ct0_d1_3x3() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,3u8,1u8,0u8,0u8,0u8,1u8,9u8,84u8,184u8,132u8,0u8,0u8,0u8,20u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,104u8,96u8,108u8,96u8,114u8,96u8,102u8,96u8,97u8,96u8,56u8,0u8,0u8,12u8,200u8,2u8,11u8,89u8,216u8,176u8,93u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_interlace_ct0_d1_3x3() -> Seq<u8> {seq![120u8,156u8,99u8,104u8,96u8,108u8,96u8,114u8,96u8,102u8,96u8,97u8,96u8,56u8,0u8,0u8,12u8,200u8,2u8,11u8]}
spec fn h_interlace_ct0_d1_3x3() -> crate::model::Ihdr {crate::model::Ihdr {width:3u32,height:3u32,depth:1u8,color_type:0u8,interlace:1u8}}
spec fn a_interlace_ct0_d1_3x3() -> r::Raster {r::shape(h_interlace_ct0_d1_3x3(),3nat,3nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_interlace_ct0_d1_3x3(b: Seq<u8>)
    requires b.len()==77,crate::model::mh(b)==Ok((crate::model::Ihdr {width:3u32,height:3u32,depth:1u8,color_type:0u8,interlace:1u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:20nat,next:65nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,65nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:73nat,len:0nat,next:77nat,crc_ok:true,synthetic:false})),
        b.subrange(41,61)==z_interlace_ct0_d1_3x3(),
    ensures r::default_case(b,crate::model::Ihdr {width:3u32,height:3u32,depth:1u8,color_type:0u8,interlace:1u8},z_interlace_ct0_d1_3x3(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    hide(z_interlace_ct0_d1_3x3);
        assert((1u32 << 1u8)==2u32) by (compute);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_interlace_ct0_d1_3x3() -> Seq<u8> {seq![0u8,128u8,1u8,128u8,2u8,64u8,3u8,0u8,4u8,0u8,0u8,192u8]}
spec fn ts_interlace_ct0_d1_3x3() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:35nat,before:1nat,after:2nat,symbol:128nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:35nat,next:43nat,before:2nat,after:3nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:43nat,next:51nat,before:3nat,after:4nat,symbol:128nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:51nat,next:59nat,before:4nat,after:5nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:59nat,next:67nat,before:5nat,after:6nat,symbol:64nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:67nat,next:75nat,before:6nat,after:7nat,symbol:3nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:75nat,next:83nat,before:7nat,after:8nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:83nat,next:91nat,before:8nat,after:9nat,symbol:4nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:91nat,next:99nat,before:9nat,after:10nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:99nat,next:107nat,before:10nat,after:11nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:107nat,next:116nat,before:11nat,after:12nat,symbol:192nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:116nat,next:123nat,before:12nat,after:12nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_interlace_ct0_d1_3x3()
    ensures t::trace(z_interlace_ct0_d1_3x3(),raw_interlace_ct0_d1_3x3(),ts_interlace_ct0_d1_3x3(),0,13nat),
{
    hide(z_interlace_ct0_d1_3x3);hide(raw_interlace_ct0_d1_3x3);hide(ts_interlace_ct0_d1_3x3);hide(t::valid_step);
    assert(t::trace(z_interlace_ct0_d1_3x3(),raw_interlace_ct0_d1_3x3(),ts_interlace_ct0_d1_3x3(),0,13nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_interlace_ct0_d1_3x3()
    ensures raw_interlace_ct0_d1_3x3().len()==12nat,
        z_interlace_ct0_d1_3x3().len()==20nat,
        z_interlace_ct0_d1_3x3()[0]==120u8,
        z_interlace_ct0_d1_3x3()[1]==156u8,
        zm::field_value(z_interlace_ct0_d1_3x3(),16,1)==1nat,
        zm::field_value(z_interlace_ct0_d1_3x3(),17,2)==1nat,
        zm::be_value(z_interlace_ct0_d1_3x3(),16nat,4)==214434315nat,
        ts_interlace_ct0_d1_3x3().len()==13nat,
        ts_interlace_ct0_d1_3x3()[0].pos==19nat,
        ts_interlace_ct0_d1_3x3()[0].before==0nat,
        ts_interlace_ct0_d1_3x3().last().next==123nat,
{
    hide(z_interlace_ct0_d1_3x3);hide(raw_interlace_ct0_d1_3x3);hide(ts_interlace_ct0_d1_3x3);
    assert(raw_interlace_ct0_d1_3x3().len()==12nat) by (compute);
    assert(z_interlace_ct0_d1_3x3().len()==20nat) by (compute);
    assert(z_interlace_ct0_d1_3x3()[0]==120u8) by (compute);
    assert(z_interlace_ct0_d1_3x3()[1]==156u8) by (compute);
    assert(zm::field_value(z_interlace_ct0_d1_3x3(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_interlace_ct0_d1_3x3(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_interlace_ct0_d1_3x3(),16nat,4)==214434315nat) by (compute);
    assert(ts_interlace_ct0_d1_3x3().len()==13nat) by (compute);
    assert(ts_interlace_ct0_d1_3x3()[0].pos==19nat) by (compute);
    assert(ts_interlace_ct0_d1_3x3()[0].before==0nat) by (compute);
    assert(ts_interlace_ct0_d1_3x3().last().next==123nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_interlace_ct0_d1_3x3()
    ensures zm::Adler(raw_interlace_ct0_d1_3x3())==214434315nat,
{
    hide(raw_interlace_ct0_d1_3x3);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_interlace_ct0_d1_3x3().len()==12nat) by (compute);
    assert(c::sums(raw_interlace_ct0_d1_3x3(),0,12nat)==(522nat,3272nat)) by (compute);
    c::sums_correct(raw_interlace_ct0_d1_3x3(),0,12nat);
    assert(raw_interlace_ct0_d1_3x3().subrange(0,12) =~= raw_interlace_ct0_d1_3x3());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((3272nat%65521)*(u16::MAX as nat+1)+(1+522nat)%65521==214434315nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_interlace_ct0_d1_3x3()
    ensures r::inflated(z_interlace_ct0_d1_3x3(),a_interlace_ct0_d1_3x3(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_interlace_ct0_d1_3x3(),r2:false}),
        raw_interlace_ct0_d1_3x3().len()==12nat,
{
    hide(z_interlace_ct0_d1_3x3);hide(raw_interlace_ct0_d1_3x3);hide(ts_interlace_ct0_d1_3x3);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_interlace_ct0_d1_3x3();checksum_interlace_ct0_d1_3x3();
    assert(r::raw(a_interlace_ct0_d1_3x3())==12nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_interlace_ct0_d1_3x3();
    stream::fixed_stream(z_interlace_ct0_d1_3x3(),raw_interlace_ct0_d1_3x3(),ts_interlace_ct0_d1_3x3(),12nat,156nat,214434315nat);
}
#[verifier::spinoff_prover]
proof fn scan_interlace_ct0_d1_3x3()
    ensures r::scan(raw_interlace_ct0_d1_3x3(),a_interlace_ct0_d1_3x3(),0)==Ok(()),
{
    hide(raw_interlace_ct0_d1_3x3);
    assert(r::count(a_interlace_ct0_d1_3x3())==7nat) by (compute);
    r::scan_step(raw_interlace_ct0_d1_3x3(),a_interlace_ct0_d1_3x3(),7);
    assert(r::pw(a_interlace_ct0_d1_3x3(),6nat)==3nat && r::ph(a_interlace_ct0_d1_3x3(),6nat)==1nat && r::rb(a_interlace_ct0_d1_3x3(),6nat)==1nat && r::base(a_interlace_ct0_d1_3x3(),6nat)==10nat) by (compute);
    assert(raw_interlace_ct0_d1_3x3()[10]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_3x3(),a_interlace_ct0_d1_3x3(),6nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_3x3(),a_interlace_ct0_d1_3x3(),6nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_3x3(),a_interlace_ct0_d1_3x3(),6nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_3x3(),a_interlace_ct0_d1_3x3(),6nat);
    assert(r::pw(a_interlace_ct0_d1_3x3(),5nat)==1nat && r::ph(a_interlace_ct0_d1_3x3(),5nat)==2nat && r::rb(a_interlace_ct0_d1_3x3(),5nat)==1nat && r::base(a_interlace_ct0_d1_3x3(),5nat)==6nat) by (compute);
    assert(raw_interlace_ct0_d1_3x3()[6]==3u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_3x3(),a_interlace_ct0_d1_3x3(),5nat,0nat)==Ok(()));
    assert(raw_interlace_ct0_d1_3x3()[8]==4u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_3x3(),a_interlace_ct0_d1_3x3(),5nat,1nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_3x3(),a_interlace_ct0_d1_3x3(),5nat,2nat);
    r::pass_step(raw_interlace_ct0_d1_3x3(),a_interlace_ct0_d1_3x3(),5nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_3x3(),a_interlace_ct0_d1_3x3(),5nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_3x3(),a_interlace_ct0_d1_3x3(),5nat);
    assert(r::pw(a_interlace_ct0_d1_3x3(),4nat)==2nat && r::ph(a_interlace_ct0_d1_3x3(),4nat)==1nat && r::rb(a_interlace_ct0_d1_3x3(),4nat)==1nat && r::base(a_interlace_ct0_d1_3x3(),4nat)==4nat) by (compute);
    assert(raw_interlace_ct0_d1_3x3()[4]==2u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_3x3(),a_interlace_ct0_d1_3x3(),4nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_3x3(),a_interlace_ct0_d1_3x3(),4nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_3x3(),a_interlace_ct0_d1_3x3(),4nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_3x3(),a_interlace_ct0_d1_3x3(),4nat);
    assert(r::pw(a_interlace_ct0_d1_3x3(),3nat)==1nat && r::ph(a_interlace_ct0_d1_3x3(),3nat)==1nat && r::rb(a_interlace_ct0_d1_3x3(),3nat)==1nat && r::base(a_interlace_ct0_d1_3x3(),3nat)==2nat) by (compute);
    assert(raw_interlace_ct0_d1_3x3()[2]==1u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_3x3(),a_interlace_ct0_d1_3x3(),3nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_3x3(),a_interlace_ct0_d1_3x3(),3nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_3x3(),a_interlace_ct0_d1_3x3(),3nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_3x3(),a_interlace_ct0_d1_3x3(),3nat);
    assert(r::pw(a_interlace_ct0_d1_3x3(),2nat)==1nat && r::ph(a_interlace_ct0_d1_3x3(),2nat)==0nat && r::rb(a_interlace_ct0_d1_3x3(),2nat)==1nat && r::base(a_interlace_ct0_d1_3x3(),2nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct0_d1_3x3(),a_interlace_ct0_d1_3x3(),2nat,0);
    r::scan_step(raw_interlace_ct0_d1_3x3(),a_interlace_ct0_d1_3x3(),2nat);
    assert(r::pw(a_interlace_ct0_d1_3x3(),1nat)==0nat && r::ph(a_interlace_ct0_d1_3x3(),1nat)==1nat && r::rb(a_interlace_ct0_d1_3x3(),1nat)==0nat && r::base(a_interlace_ct0_d1_3x3(),1nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct0_d1_3x3(),a_interlace_ct0_d1_3x3(),1nat,0);
    r::scan_step(raw_interlace_ct0_d1_3x3(),a_interlace_ct0_d1_3x3(),1nat);
    assert(r::pw(a_interlace_ct0_d1_3x3(),0nat)==1nat && r::ph(a_interlace_ct0_d1_3x3(),0nat)==1nat && r::rb(a_interlace_ct0_d1_3x3(),0nat)==1nat && r::base(a_interlace_ct0_d1_3x3(),0nat)==0nat) by (compute);
    assert(raw_interlace_ct0_d1_3x3()[0]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_3x3(),a_interlace_ct0_d1_3x3(),0nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_3x3(),a_interlace_ct0_d1_3x3(),0nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_3x3(),a_interlace_ct0_d1_3x3(),0nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_3x3(),a_interlace_ct0_d1_3x3(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_interlace_ct0_d1_3x3()
    ensures r::still(bytes_interlace_ct0_d1_3x3(),STRICT,false)==(r::Projection::Ready {width:3u32,height:3u32,bytes:36nat}),
{
    hide(bytes_interlace_ct0_d1_3x3);hide(z_interlace_ct0_d1_3x3);hide(raw_interlace_ct0_d1_3x3);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_interlace_ct0_d1_3x3().len()==77nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct0_d1_3x3())==Ok((crate::model::Ihdr {width:3u32,height:3u32,depth:1u8,color_type:0u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct0_d1_3x3())==Ok((crate::model::Ihdr {width:3u32,height:3u32,depth:1u8,color_type:0u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 1u8)==2u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct0_d1_3x3(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:20nat,next:65nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct0_d1_3x3(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:20nat,next:65nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct0_d1_3x3(),65nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:73nat,len:0nat,next:77nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct0_d1_3x3(),65nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:73nat,len:0nat,next:77nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct0_d1_3x3().subrange(41,61)==z_interlace_ct0_d1_3x3()) by (compute);
    walk_interlace_ct0_d1_3x3(bytes_interlace_ct0_d1_3x3());
    inflate_interlace_ct0_d1_3x3();scan_interlace_ct0_d1_3x3();
    assert(r::raw(a_interlace_ct0_d1_3x3())==12nat) by (compute);
    assert(r::data(z_interlace_ct0_d1_3x3(),a_interlace_ct0_d1_3x3(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_interlace_ct0_d1_3x3(),h_interlace_ct0_d1_3x3(),z_interlace_ct0_d1_3x3(),0nat,false);
}
fn public_interlace_ct0_d1_3x3() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,3u8,1u8,0u8,0u8,0u8,1u8,9u8,84u8,184u8,132u8,0u8,0u8,0u8,20u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,104u8,96u8,108u8,96u8,114u8,96u8,102u8,96u8,97u8,96u8,56u8,0u8,0u8,12u8,200u8,2u8,11u8,89u8,216u8,176u8,93u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_interlace_ct0_d1_3x3());model_interlace_ct0_d1_3x3();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==3u32 && s.height==3u32 && s.rgba@.len()==36nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// interlace/ct6-d16-3x3; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_interlace_ct6_d16_3x3() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,3u8,16u8,6u8,0u8,0u8,1u8,113u8,191u8,89u8,106u8,0u8,0u8,0u8,89u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,78u8,0u8,177u8,255u8,0u8,106u8,51u8,72u8,202u8,7u8,37u8,107u8,131u8,1u8,65u8,76u8,26u8,146u8,135u8,159u8,22u8,57u8,2u8,221u8,3u8,12u8,95u8,254u8,37u8,202u8,188u8,225u8,224u8,150u8,201u8,166u8,132u8,12u8,21u8,3u8,73u8,112u8,52u8,233u8,246u8,19u8,71u8,152u8,4u8,106u8,136u8,38u8,39u8,238u8,100u8,203u8,66u8,0u8,165u8,128u8,124u8,223u8,127u8,22u8,24u8,234u8,7u8,104u8,68u8,198u8,253u8,156u8,57u8,51u8,196u8,39u8,174u8,126u8,198u8,150u8,126u8,255u8,185u8,85u8,33u8,194u8,53u8,255u8,29u8,250u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_interlace_ct6_d16_3x3() -> Seq<u8> {seq![120u8,156u8,1u8,78u8,0u8,177u8,255u8,0u8,106u8,51u8,72u8,202u8,7u8,37u8,107u8,131u8,1u8,65u8,76u8,26u8,146u8,135u8,159u8,22u8,57u8,2u8,221u8,3u8,12u8,95u8,254u8,37u8,202u8,188u8,225u8,224u8,150u8,201u8,166u8,132u8,12u8,21u8,3u8,73u8,112u8,52u8,233u8,246u8,19u8,71u8,152u8,4u8,106u8,136u8,38u8,39u8,238u8,100u8,203u8,66u8,0u8,165u8,128u8,124u8,223u8,127u8,22u8,24u8,234u8,7u8,104u8,68u8,198u8,253u8,156u8,57u8,51u8,196u8,39u8,174u8,126u8,198u8,150u8,126u8,255u8,185u8,85u8,33u8,194u8]}
spec fn h_interlace_ct6_d16_3x3() -> crate::model::Ihdr {crate::model::Ihdr {width:3u32,height:3u32,depth:16u8,color_type:6u8,interlace:1u8}}
spec fn a_interlace_ct6_d16_3x3() -> r::Raster {r::shape(h_interlace_ct6_d16_3x3(),3nat,3nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_interlace_ct6_d16_3x3(b: Seq<u8>)
    requires b.len()==146,crate::model::mh(b)==Ok((crate::model::Ihdr {width:3u32,height:3u32,depth:16u8,color_type:6u8,interlace:1u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:89nat,next:134nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,134nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:142nat,len:0nat,next:146nat,crc_ok:true,synthetic:false})),
        b.subrange(41,130)==z_interlace_ct6_d16_3x3(),
    ensures r::default_case(b,crate::model::Ihdr {width:3u32,height:3u32,depth:16u8,color_type:6u8,interlace:1u8},z_interlace_ct6_d16_3x3(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    hide(z_interlace_ct6_d16_3x3);
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_interlace_ct6_d16_3x3() -> Seq<u8> {seq![0u8,106u8,51u8,72u8,202u8,7u8,37u8,107u8,131u8,1u8,65u8,76u8,26u8,146u8,135u8,159u8,22u8,57u8,2u8,221u8,3u8,12u8,95u8,254u8,37u8,202u8,188u8,225u8,224u8,150u8,201u8,166u8,132u8,12u8,21u8,3u8,73u8,112u8,52u8,233u8,246u8,19u8,71u8,152u8,4u8,106u8,136u8,38u8,39u8,238u8,100u8,203u8,66u8,0u8,165u8,128u8,124u8,223u8,127u8,22u8,24u8,234u8,7u8,104u8,68u8,198u8,253u8,156u8,57u8,51u8,196u8,39u8,174u8,126u8,198u8,150u8,126u8,255u8]}
#[verifier::spinoff_prover]
proof fn shape_interlace_ct6_d16_3x3()
    ensures raw_interlace_ct6_d16_3x3().len()==78nat,
        z_interlace_ct6_d16_3x3().len()==89nat,
        z_interlace_ct6_d16_3x3()[0]==120u8,
        z_interlace_ct6_d16_3x3()[1]==156u8,
        zm::field_value(z_interlace_ct6_d16_3x3(),16,1)==1nat,
        zm::field_value(z_interlace_ct6_d16_3x3(),17,2)==0nat,
        zm::be_value(z_interlace_ct6_d16_3x3(),85nat,4)==3109364162nat,
        z_interlace_ct6_d16_3x3().subrange(7,85)==raw_interlace_ct6_d16_3x3(),
        z_interlace_ct6_d16_3x3()[3] as nat+256*z_interlace_ct6_d16_3x3()[4] as nat==78nat,
        z_interlace_ct6_d16_3x3()[5] as nat+256*z_interlace_ct6_d16_3x3()[6] as nat==u16::MAX as nat-78nat,
        zm::be_value(z_interlace_ct6_d16_3x3(),0,1)==120nat,
        zm::be_value(z_interlace_ct6_d16_3x3(),1,1)==156nat,
{
    hide(z_interlace_ct6_d16_3x3);hide(raw_interlace_ct6_d16_3x3);
    assert(raw_interlace_ct6_d16_3x3().len()==78nat) by (compute);
    assert(z_interlace_ct6_d16_3x3().len()==89nat) by (compute);
    assert(z_interlace_ct6_d16_3x3()[0]==120u8) by (compute);
    assert(z_interlace_ct6_d16_3x3()[1]==156u8) by (compute);
    assert(zm::field_value(z_interlace_ct6_d16_3x3(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_interlace_ct6_d16_3x3(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_interlace_ct6_d16_3x3(),85nat,4)==3109364162nat) by (compute);
    assert(z_interlace_ct6_d16_3x3().subrange(7,85)==raw_interlace_ct6_d16_3x3()) by (compute);
    assert(z_interlace_ct6_d16_3x3()[3] as nat+256*z_interlace_ct6_d16_3x3()[4] as nat==78nat) by (compute);
    assert(z_interlace_ct6_d16_3x3()[5] as nat+256*z_interlace_ct6_d16_3x3()[6] as nat==u16::MAX as nat-78nat) by (compute);
    assert(zm::be_value(z_interlace_ct6_d16_3x3(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_interlace_ct6_d16_3x3(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_interlace_ct6_d16_3x3()
    ensures zm::Adler(raw_interlace_ct6_d16_3x3())==3109364162nat,
{
    hide(raw_interlace_ct6_d16_3x3);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_interlace_ct6_d16_3x3().len()==78nat) by (compute);
    assert(c::sums(raw_interlace_ct6_d16_3x3(),0,78nat)==(8641nat,309529nat)) by (compute);
    c::sums_correct(raw_interlace_ct6_d16_3x3(),0,78nat);
    assert(raw_interlace_ct6_d16_3x3().subrange(0,78) =~= raw_interlace_ct6_d16_3x3());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((309529nat%65521)*(u16::MAX as nat+1)+(1+8641nat)%65521==3109364162nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_interlace_ct6_d16_3x3()
    ensures r::inflated(z_interlace_ct6_d16_3x3(),a_interlace_ct6_d16_3x3(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_interlace_ct6_d16_3x3(),r2:false}),
        raw_interlace_ct6_d16_3x3().len()==78nat,
{
    hide(z_interlace_ct6_d16_3x3);hide(raw_interlace_ct6_d16_3x3);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_interlace_ct6_d16_3x3();checksum_interlace_ct6_d16_3x3();
    assert(r::raw(a_interlace_ct6_d16_3x3())==78nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_interlace_ct6_d16_3x3(),raw_interlace_ct6_d16_3x3(),78nat,120nat,156nat,3109364162nat);
}
#[verifier::spinoff_prover]
proof fn scan_interlace_ct6_d16_3x3()
    ensures r::scan(raw_interlace_ct6_d16_3x3(),a_interlace_ct6_d16_3x3(),0)==Ok(()),
{
    hide(raw_interlace_ct6_d16_3x3);
    assert(r::count(a_interlace_ct6_d16_3x3())==7nat) by (compute);
    r::scan_step(raw_interlace_ct6_d16_3x3(),a_interlace_ct6_d16_3x3(),7);
    assert(r::pw(a_interlace_ct6_d16_3x3(),6nat)==3nat && r::ph(a_interlace_ct6_d16_3x3(),6nat)==1nat && r::rb(a_interlace_ct6_d16_3x3(),6nat)==24nat && r::base(a_interlace_ct6_d16_3x3(),6nat)==53nat) by (compute);
    assert(raw_interlace_ct6_d16_3x3()[53]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_3x3(),a_interlace_ct6_d16_3x3(),6nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_3x3(),a_interlace_ct6_d16_3x3(),6nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_3x3(),a_interlace_ct6_d16_3x3(),6nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_3x3(),a_interlace_ct6_d16_3x3(),6nat);
    assert(r::pw(a_interlace_ct6_d16_3x3(),5nat)==1nat && r::ph(a_interlace_ct6_d16_3x3(),5nat)==2nat && r::rb(a_interlace_ct6_d16_3x3(),5nat)==8nat && r::base(a_interlace_ct6_d16_3x3(),5nat)==35nat) by (compute);
    assert(raw_interlace_ct6_d16_3x3()[35]==3u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_3x3(),a_interlace_ct6_d16_3x3(),5nat,0nat)==Ok(()));
    assert(raw_interlace_ct6_d16_3x3()[44]==4u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_3x3(),a_interlace_ct6_d16_3x3(),5nat,1nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_3x3(),a_interlace_ct6_d16_3x3(),5nat,2nat);
    r::pass_step(raw_interlace_ct6_d16_3x3(),a_interlace_ct6_d16_3x3(),5nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_3x3(),a_interlace_ct6_d16_3x3(),5nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_3x3(),a_interlace_ct6_d16_3x3(),5nat);
    assert(r::pw(a_interlace_ct6_d16_3x3(),4nat)==2nat && r::ph(a_interlace_ct6_d16_3x3(),4nat)==1nat && r::rb(a_interlace_ct6_d16_3x3(),4nat)==16nat && r::base(a_interlace_ct6_d16_3x3(),4nat)==18nat) by (compute);
    assert(raw_interlace_ct6_d16_3x3()[18]==2u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_3x3(),a_interlace_ct6_d16_3x3(),4nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_3x3(),a_interlace_ct6_d16_3x3(),4nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_3x3(),a_interlace_ct6_d16_3x3(),4nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_3x3(),a_interlace_ct6_d16_3x3(),4nat);
    assert(r::pw(a_interlace_ct6_d16_3x3(),3nat)==1nat && r::ph(a_interlace_ct6_d16_3x3(),3nat)==1nat && r::rb(a_interlace_ct6_d16_3x3(),3nat)==8nat && r::base(a_interlace_ct6_d16_3x3(),3nat)==9nat) by (compute);
    assert(raw_interlace_ct6_d16_3x3()[9]==1u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_3x3(),a_interlace_ct6_d16_3x3(),3nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_3x3(),a_interlace_ct6_d16_3x3(),3nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_3x3(),a_interlace_ct6_d16_3x3(),3nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_3x3(),a_interlace_ct6_d16_3x3(),3nat);
    assert(r::pw(a_interlace_ct6_d16_3x3(),2nat)==1nat && r::ph(a_interlace_ct6_d16_3x3(),2nat)==0nat && r::rb(a_interlace_ct6_d16_3x3(),2nat)==8nat && r::base(a_interlace_ct6_d16_3x3(),2nat)==9nat) by (compute);
    r::pass_step(raw_interlace_ct6_d16_3x3(),a_interlace_ct6_d16_3x3(),2nat,0);
    r::scan_step(raw_interlace_ct6_d16_3x3(),a_interlace_ct6_d16_3x3(),2nat);
    assert(r::pw(a_interlace_ct6_d16_3x3(),1nat)==0nat && r::ph(a_interlace_ct6_d16_3x3(),1nat)==1nat && r::rb(a_interlace_ct6_d16_3x3(),1nat)==0nat && r::base(a_interlace_ct6_d16_3x3(),1nat)==9nat) by (compute);
    r::pass_step(raw_interlace_ct6_d16_3x3(),a_interlace_ct6_d16_3x3(),1nat,0);
    r::scan_step(raw_interlace_ct6_d16_3x3(),a_interlace_ct6_d16_3x3(),1nat);
    assert(r::pw(a_interlace_ct6_d16_3x3(),0nat)==1nat && r::ph(a_interlace_ct6_d16_3x3(),0nat)==1nat && r::rb(a_interlace_ct6_d16_3x3(),0nat)==8nat && r::base(a_interlace_ct6_d16_3x3(),0nat)==0nat) by (compute);
    assert(raw_interlace_ct6_d16_3x3()[0]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_3x3(),a_interlace_ct6_d16_3x3(),0nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_3x3(),a_interlace_ct6_d16_3x3(),0nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_3x3(),a_interlace_ct6_d16_3x3(),0nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_3x3(),a_interlace_ct6_d16_3x3(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_interlace_ct6_d16_3x3()
    ensures r::still(bytes_interlace_ct6_d16_3x3(),STRICT,false)==(r::Projection::Ready {width:3u32,height:3u32,bytes:36nat}),
{
    hide(bytes_interlace_ct6_d16_3x3);hide(z_interlace_ct6_d16_3x3);hide(raw_interlace_ct6_d16_3x3);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_interlace_ct6_d16_3x3().len()==146nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct6_d16_3x3())==Ok((crate::model::Ihdr {width:3u32,height:3u32,depth:16u8,color_type:6u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct6_d16_3x3())==Ok((crate::model::Ihdr {width:3u32,height:3u32,depth:16u8,color_type:6u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct6_d16_3x3(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:89nat,next:134nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct6_d16_3x3(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:89nat,next:134nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct6_d16_3x3(),134nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:142nat,len:0nat,next:146nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct6_d16_3x3(),134nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:142nat,len:0nat,next:146nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct6_d16_3x3().subrange(41,130)==z_interlace_ct6_d16_3x3()) by (compute);
    walk_interlace_ct6_d16_3x3(bytes_interlace_ct6_d16_3x3());
    inflate_interlace_ct6_d16_3x3();scan_interlace_ct6_d16_3x3();
    assert(r::raw(a_interlace_ct6_d16_3x3())==78nat) by (compute);
    assert(r::data(z_interlace_ct6_d16_3x3(),a_interlace_ct6_d16_3x3(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_interlace_ct6_d16_3x3(),h_interlace_ct6_d16_3x3(),z_interlace_ct6_d16_3x3(),0nat,false);
}
fn public_interlace_ct6_d16_3x3() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,3u8,16u8,6u8,0u8,0u8,1u8,113u8,191u8,89u8,106u8,0u8,0u8,0u8,89u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,78u8,0u8,177u8,255u8,0u8,106u8,51u8,72u8,202u8,7u8,37u8,107u8,131u8,1u8,65u8,76u8,26u8,146u8,135u8,159u8,22u8,57u8,2u8,221u8,3u8,12u8,95u8,254u8,37u8,202u8,188u8,225u8,224u8,150u8,201u8,166u8,132u8,12u8,21u8,3u8,73u8,112u8,52u8,233u8,246u8,19u8,71u8,152u8,4u8,106u8,136u8,38u8,39u8,238u8,100u8,203u8,66u8,0u8,165u8,128u8,124u8,223u8,127u8,22u8,24u8,234u8,7u8,104u8,68u8,198u8,253u8,156u8,57u8,51u8,196u8,39u8,174u8,126u8,198u8,150u8,126u8,255u8,185u8,85u8,33u8,194u8,53u8,255u8,29u8,250u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_interlace_ct6_d16_3x3());model_interlace_ct6_d16_3x3();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==3u32 && s.height==3u32 && s.rgba@.len()==36nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// interlace/ct3-d4-3x3; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_interlace_ct3_d4_3x3() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,3u8,4u8,3u8,0u8,0u8,1u8,211u8,1u8,152u8,26u8,0u8,0u8,0u8,48u8,80u8,76u8,84u8,69u8,64u8,165u8,197u8,59u8,205u8,174u8,113u8,157u8,26u8,122u8,35u8,237u8,139u8,153u8,15u8,105u8,130u8,34u8,54u8,172u8,38u8,103u8,35u8,57u8,174u8,18u8,146u8,64u8,39u8,165u8,74u8,77u8,4u8,212u8,157u8,168u8,28u8,25u8,234u8,253u8,1u8,109u8,219u8,136u8,206u8,247u8,94u8,54u8,222u8,12u8,88u8,7u8,0u8,0u8,0u8,21u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,184u8,192u8,232u8,192u8,228u8,204u8,252u8,128u8,229u8,0u8,67u8,141u8,0u8,0u8,23u8,243u8,3u8,138u8,115u8,22u8,83u8,6u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_interlace_ct3_d4_3x3() -> Seq<u8> {seq![120u8,156u8,99u8,184u8,192u8,232u8,192u8,228u8,204u8,252u8,128u8,229u8,0u8,67u8,141u8,0u8,0u8,23u8,243u8,3u8,138u8]}
spec fn h_interlace_ct3_d4_3x3() -> crate::model::Ihdr {crate::model::Ihdr {width:3u32,height:3u32,depth:4u8,color_type:3u8,interlace:1u8}}
spec fn a_interlace_ct3_d4_3x3() -> r::Raster {r::shape(h_interlace_ct3_d4_3x3(),3nat,3nat,16nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_interlace_ct3_d4_3x3(b: Seq<u8>)
    requires b.len()==138,crate::model::mh(b)==Ok((crate::model::Ihdr {width:3u32,height:3u32,depth:4u8,color_type:3u8,interlace:1u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:21nat,next:126nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:134nat,len:0nat,next:138nat,crc_ok:true,synthetic:false})),
        b.subrange(101,122)==z_interlace_ct3_d4_3x3(),
    ensures r::default_case(b,crate::model::Ihdr {width:3u32,height:3u32,depth:4u8,color_type:3u8,interlace:1u8},z_interlace_ct3_d4_3x3(),16nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);
    hide(z_interlace_ct3_d4_3x3);
    r::palette_default_case(b,crate::model::Ihdr {width:3u32,height:3u32,depth:4u8,color_type:3u8,interlace:1u8},z_interlace_ct3_d4_3x3(),21nat);
}
spec fn raw_interlace_ct3_d4_3x3() -> Seq<u8> {seq![0u8,208u8,1u8,64u8,2u8,67u8,3u8,224u8,4u8,192u8,0u8,124u8,16u8]}
spec fn ts_interlace_ct3_d4_3x3() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:36nat,before:1nat,after:2nat,symbol:208nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:36nat,next:44nat,before:2nat,after:3nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:44nat,next:52nat,before:3nat,after:4nat,symbol:64nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:52nat,next:60nat,before:4nat,after:5nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:60nat,next:68nat,before:5nat,after:6nat,symbol:67nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:68nat,next:76nat,before:6nat,after:7nat,symbol:3nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:76nat,next:85nat,before:7nat,after:8nat,symbol:224nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:85nat,next:93nat,before:8nat,after:9nat,symbol:4nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:93nat,next:102nat,before:9nat,after:10nat,symbol:192nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:102nat,next:110nat,before:10nat,after:11nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:110nat,next:118nat,before:11nat,after:12nat,symbol:124nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:118nat,next:126nat,before:12nat,after:13nat,symbol:16nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:126nat,next:133nat,before:13nat,after:13nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_interlace_ct3_d4_3x3()
    ensures t::trace(z_interlace_ct3_d4_3x3(),raw_interlace_ct3_d4_3x3(),ts_interlace_ct3_d4_3x3(),0,14nat),
{
    hide(z_interlace_ct3_d4_3x3);hide(raw_interlace_ct3_d4_3x3);hide(ts_interlace_ct3_d4_3x3);hide(t::valid_step);
    assert(t::trace(z_interlace_ct3_d4_3x3(),raw_interlace_ct3_d4_3x3(),ts_interlace_ct3_d4_3x3(),0,14nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_interlace_ct3_d4_3x3()
    ensures raw_interlace_ct3_d4_3x3().len()==13nat,
        z_interlace_ct3_d4_3x3().len()==21nat,
        z_interlace_ct3_d4_3x3()[0]==120u8,
        z_interlace_ct3_d4_3x3()[1]==156u8,
        zm::field_value(z_interlace_ct3_d4_3x3(),16,1)==1nat,
        zm::field_value(z_interlace_ct3_d4_3x3(),17,2)==1nat,
        zm::be_value(z_interlace_ct3_d4_3x3(),17nat,4)==401802122nat,
        ts_interlace_ct3_d4_3x3().len()==14nat,
        ts_interlace_ct3_d4_3x3()[0].pos==19nat,
        ts_interlace_ct3_d4_3x3()[0].before==0nat,
        ts_interlace_ct3_d4_3x3().last().next==133nat,
{
    hide(z_interlace_ct3_d4_3x3);hide(raw_interlace_ct3_d4_3x3);hide(ts_interlace_ct3_d4_3x3);
    assert(raw_interlace_ct3_d4_3x3().len()==13nat) by (compute);
    assert(z_interlace_ct3_d4_3x3().len()==21nat) by (compute);
    assert(z_interlace_ct3_d4_3x3()[0]==120u8) by (compute);
    assert(z_interlace_ct3_d4_3x3()[1]==156u8) by (compute);
    assert(zm::field_value(z_interlace_ct3_d4_3x3(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_interlace_ct3_d4_3x3(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_interlace_ct3_d4_3x3(),17nat,4)==401802122nat) by (compute);
    assert(ts_interlace_ct3_d4_3x3().len()==14nat) by (compute);
    assert(ts_interlace_ct3_d4_3x3()[0].pos==19nat) by (compute);
    assert(ts_interlace_ct3_d4_3x3()[0].before==0nat) by (compute);
    assert(ts_interlace_ct3_d4_3x3().last().next==133nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_interlace_ct3_d4_3x3()
    ensures zm::Adler(raw_interlace_ct3_d4_3x3())==401802122nat,
{
    hide(raw_interlace_ct3_d4_3x3);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_interlace_ct3_d4_3x3().len()==13nat) by (compute);
    assert(c::sums(raw_interlace_ct3_d4_3x3(),0,13nat)==(905nat,6131nat)) by (compute);
    c::sums_correct(raw_interlace_ct3_d4_3x3(),0,13nat);
    assert(raw_interlace_ct3_d4_3x3().subrange(0,13) =~= raw_interlace_ct3_d4_3x3());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((6131nat%65521)*(u16::MAX as nat+1)+(1+905nat)%65521==401802122nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_interlace_ct3_d4_3x3()
    ensures r::inflated(z_interlace_ct3_d4_3x3(),a_interlace_ct3_d4_3x3(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_interlace_ct3_d4_3x3(),r2:false}),
        raw_interlace_ct3_d4_3x3().len()==13nat,
{
    hide(z_interlace_ct3_d4_3x3);hide(raw_interlace_ct3_d4_3x3);hide(ts_interlace_ct3_d4_3x3);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_interlace_ct3_d4_3x3();checksum_interlace_ct3_d4_3x3();
    assert(r::raw(a_interlace_ct3_d4_3x3())==13nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_interlace_ct3_d4_3x3();
    stream::fixed_stream(z_interlace_ct3_d4_3x3(),raw_interlace_ct3_d4_3x3(),ts_interlace_ct3_d4_3x3(),13nat,156nat,401802122nat);
}
#[verifier::spinoff_prover]
proof fn scan_interlace_ct3_d4_3x3()
    ensures r::scan(raw_interlace_ct3_d4_3x3(),a_interlace_ct3_d4_3x3(),0)==Ok(()),
{
    hide(raw_interlace_ct3_d4_3x3);
    assert(r::count(a_interlace_ct3_d4_3x3())==7nat) by (compute);
    r::scan_step(raw_interlace_ct3_d4_3x3(),a_interlace_ct3_d4_3x3(),7);
    assert(r::pw(a_interlace_ct3_d4_3x3(),6nat)==3nat && r::ph(a_interlace_ct3_d4_3x3(),6nat)==1nat && r::rb(a_interlace_ct3_d4_3x3(),6nat)==2nat && r::base(a_interlace_ct3_d4_3x3(),6nat)==10nat) by (compute);
    assert(raw_interlace_ct3_d4_3x3()[10]==0u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_3x3(),a_interlace_ct3_d4_3x3(),6nat,0nat),3nat);
    assert(r::row_result(raw_interlace_ct3_d4_3x3(),a_interlace_ct3_d4_3x3(),6nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_3x3(),a_interlace_ct3_d4_3x3(),6nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_3x3(),a_interlace_ct3_d4_3x3(),6nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_3x3(),a_interlace_ct3_d4_3x3(),6nat);
    assert(r::pw(a_interlace_ct3_d4_3x3(),5nat)==1nat && r::ph(a_interlace_ct3_d4_3x3(),5nat)==2nat && r::rb(a_interlace_ct3_d4_3x3(),5nat)==1nat && r::base(a_interlace_ct3_d4_3x3(),5nat)==6nat) by (compute);
    assert(raw_interlace_ct3_d4_3x3()[6]==3u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_3x3(),a_interlace_ct3_d4_3x3(),5nat,0nat),1nat);
    assert(r::row_result(raw_interlace_ct3_d4_3x3(),a_interlace_ct3_d4_3x3(),5nat,0nat)==Ok(()));
    assert(raw_interlace_ct3_d4_3x3()[8]==4u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_3x3(),a_interlace_ct3_d4_3x3(),5nat,1nat),1nat);
    assert(r::row_result(raw_interlace_ct3_d4_3x3(),a_interlace_ct3_d4_3x3(),5nat,1nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_3x3(),a_interlace_ct3_d4_3x3(),5nat,2nat);
    r::pass_step(raw_interlace_ct3_d4_3x3(),a_interlace_ct3_d4_3x3(),5nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_3x3(),a_interlace_ct3_d4_3x3(),5nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_3x3(),a_interlace_ct3_d4_3x3(),5nat);
    assert(r::pw(a_interlace_ct3_d4_3x3(),4nat)==2nat && r::ph(a_interlace_ct3_d4_3x3(),4nat)==1nat && r::rb(a_interlace_ct3_d4_3x3(),4nat)==1nat && r::base(a_interlace_ct3_d4_3x3(),4nat)==4nat) by (compute);
    assert(raw_interlace_ct3_d4_3x3()[4]==2u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_3x3(),a_interlace_ct3_d4_3x3(),4nat,0nat),2nat);
    assert(r::row_result(raw_interlace_ct3_d4_3x3(),a_interlace_ct3_d4_3x3(),4nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_3x3(),a_interlace_ct3_d4_3x3(),4nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_3x3(),a_interlace_ct3_d4_3x3(),4nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_3x3(),a_interlace_ct3_d4_3x3(),4nat);
    assert(r::pw(a_interlace_ct3_d4_3x3(),3nat)==1nat && r::ph(a_interlace_ct3_d4_3x3(),3nat)==1nat && r::rb(a_interlace_ct3_d4_3x3(),3nat)==1nat && r::base(a_interlace_ct3_d4_3x3(),3nat)==2nat) by (compute);
    assert(raw_interlace_ct3_d4_3x3()[2]==1u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_3x3(),a_interlace_ct3_d4_3x3(),3nat,0nat),1nat);
    assert(r::row_result(raw_interlace_ct3_d4_3x3(),a_interlace_ct3_d4_3x3(),3nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_3x3(),a_interlace_ct3_d4_3x3(),3nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_3x3(),a_interlace_ct3_d4_3x3(),3nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_3x3(),a_interlace_ct3_d4_3x3(),3nat);
    assert(r::pw(a_interlace_ct3_d4_3x3(),2nat)==1nat && r::ph(a_interlace_ct3_d4_3x3(),2nat)==0nat && r::rb(a_interlace_ct3_d4_3x3(),2nat)==1nat && r::base(a_interlace_ct3_d4_3x3(),2nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct3_d4_3x3(),a_interlace_ct3_d4_3x3(),2nat,0);
    r::scan_step(raw_interlace_ct3_d4_3x3(),a_interlace_ct3_d4_3x3(),2nat);
    assert(r::pw(a_interlace_ct3_d4_3x3(),1nat)==0nat && r::ph(a_interlace_ct3_d4_3x3(),1nat)==1nat && r::rb(a_interlace_ct3_d4_3x3(),1nat)==0nat && r::base(a_interlace_ct3_d4_3x3(),1nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct3_d4_3x3(),a_interlace_ct3_d4_3x3(),1nat,0);
    r::scan_step(raw_interlace_ct3_d4_3x3(),a_interlace_ct3_d4_3x3(),1nat);
    assert(r::pw(a_interlace_ct3_d4_3x3(),0nat)==1nat && r::ph(a_interlace_ct3_d4_3x3(),0nat)==1nat && r::rb(a_interlace_ct3_d4_3x3(),0nat)==1nat && r::base(a_interlace_ct3_d4_3x3(),0nat)==0nat) by (compute);
    assert(raw_interlace_ct3_d4_3x3()[0]==0u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_3x3(),a_interlace_ct3_d4_3x3(),0nat,0nat),1nat);
    assert(r::row_result(raw_interlace_ct3_d4_3x3(),a_interlace_ct3_d4_3x3(),0nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_3x3(),a_interlace_ct3_d4_3x3(),0nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_3x3(),a_interlace_ct3_d4_3x3(),0nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_3x3(),a_interlace_ct3_d4_3x3(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_interlace_ct3_d4_3x3()
    ensures r::still(bytes_interlace_ct3_d4_3x3(),STRICT,false)==(r::Projection::Ready {width:3u32,height:3u32,bytes:36nat}),
{
    hide(bytes_interlace_ct3_d4_3x3);hide(z_interlace_ct3_d4_3x3);hide(raw_interlace_ct3_d4_3x3);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);
    assert(bytes_interlace_ct3_d4_3x3().len()==138nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct3_d4_3x3())==Ok((crate::model::Ihdr {width:3u32,height:3u32,depth:4u8,color_type:3u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct3_d4_3x3())==Ok((crate::model::Ihdr {width:3u32,height:3u32,depth:4u8,color_type:3u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 4u8)==16u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_3x3(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_3x3(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(48nat,3u8,4u8,false,false,false,false)==crate::metadata_model::Decision::Keep(16nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_3x3(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:21nat,next:126nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_3x3(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:21nat,next:126nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_3x3(),126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:134nat,len:0nat,next:138nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_3x3(),126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:134nat,len:0nat,next:138nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct3_d4_3x3().subrange(101,122)==z_interlace_ct3_d4_3x3()) by (compute);
    walk_interlace_ct3_d4_3x3(bytes_interlace_ct3_d4_3x3());
    inflate_interlace_ct3_d4_3x3();scan_interlace_ct3_d4_3x3();
    assert(r::raw(a_interlace_ct3_d4_3x3())==13nat) by (compute);
    assert(r::data(z_interlace_ct3_d4_3x3(),a_interlace_ct3_d4_3x3(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_interlace_ct3_d4_3x3(),h_interlace_ct3_d4_3x3(),z_interlace_ct3_d4_3x3(),16nat,false);
}
fn public_interlace_ct3_d4_3x3() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,3u8,4u8,3u8,0u8,0u8,1u8,211u8,1u8,152u8,26u8,0u8,0u8,0u8,48u8,80u8,76u8,84u8,69u8,64u8,165u8,197u8,59u8,205u8,174u8,113u8,157u8,26u8,122u8,35u8,237u8,139u8,153u8,15u8,105u8,130u8,34u8,54u8,172u8,38u8,103u8,35u8,57u8,174u8,18u8,146u8,64u8,39u8,165u8,74u8,77u8,4u8,212u8,157u8,168u8,28u8,25u8,234u8,253u8,1u8,109u8,219u8,136u8,206u8,247u8,94u8,54u8,222u8,12u8,88u8,7u8,0u8,0u8,0u8,21u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,184u8,192u8,232u8,192u8,228u8,204u8,252u8,128u8,229u8,0u8,67u8,141u8,0u8,0u8,23u8,243u8,3u8,138u8,115u8,22u8,83u8,6u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_interlace_ct3_d4_3x3());model_interlace_ct3_d4_3x3();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==3u32 && s.height==3u32 && s.rgba@.len()==36nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// interlace/ct0-d1-4x4; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_interlace_ct0_d1_4x4() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,4u8,1u8,0u8,0u8,0u8,1u8,246u8,141u8,147u8,69u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,96u8,108u8,96u8,114u8,96u8,110u8,96u8,97u8,96u8,16u8,96u8,188u8,0u8,0u8,12u8,160u8,2u8,44u8,67u8,163u8,24u8,60u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_interlace_ct0_d1_4x4() -> Seq<u8> {seq![120u8,156u8,99u8,96u8,96u8,108u8,96u8,114u8,96u8,110u8,96u8,97u8,96u8,16u8,96u8,188u8,0u8,0u8,12u8,160u8,2u8,44u8]}
spec fn h_interlace_ct0_d1_4x4() -> crate::model::Ihdr {crate::model::Ihdr {width:4u32,height:4u32,depth:1u8,color_type:0u8,interlace:1u8}}
spec fn a_interlace_ct0_d1_4x4() -> r::Raster {r::shape(h_interlace_ct0_d1_4x4(),4nat,4nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_interlace_ct0_d1_4x4(b: Seq<u8>)
    requires b.len()==79,crate::model::mh(b)==Ok((crate::model::Ihdr {width:4u32,height:4u32,depth:1u8,color_type:0u8,interlace:1u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:22nat,next:67nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:75nat,len:0nat,next:79nat,crc_ok:true,synthetic:false})),
        b.subrange(41,63)==z_interlace_ct0_d1_4x4(),
    ensures r::default_case(b,crate::model::Ihdr {width:4u32,height:4u32,depth:1u8,color_type:0u8,interlace:1u8},z_interlace_ct0_d1_4x4(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    hide(z_interlace_ct0_d1_4x4);
        assert((1u32 << 1u8)==2u32) by (compute);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_interlace_ct0_d1_4x4() -> Seq<u8> {seq![0u8,0u8,1u8,128u8,2u8,64u8,3u8,128u8,4u8,0u8,0u8,16u8,1u8,208u8]}
spec fn ts_interlace_ct0_d1_4x4() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:35nat,before:1nat,after:2nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:35nat,next:43nat,before:2nat,after:3nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:43nat,next:51nat,before:3nat,after:4nat,symbol:128nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:51nat,next:59nat,before:4nat,after:5nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:59nat,next:67nat,before:5nat,after:6nat,symbol:64nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:67nat,next:75nat,before:6nat,after:7nat,symbol:3nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:75nat,next:83nat,before:7nat,after:8nat,symbol:128nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:83nat,next:91nat,before:8nat,after:9nat,symbol:4nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:91nat,next:99nat,before:9nat,after:10nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:99nat,next:107nat,before:10nat,after:11nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:107nat,next:115nat,before:11nat,after:12nat,symbol:16nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:115nat,next:123nat,before:12nat,after:13nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:123nat,next:132nat,before:13nat,after:14nat,symbol:208nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:132nat,next:139nat,before:14nat,after:14nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_interlace_ct0_d1_4x4()
    ensures t::trace(z_interlace_ct0_d1_4x4(),raw_interlace_ct0_d1_4x4(),ts_interlace_ct0_d1_4x4(),0,15nat),
{
    hide(z_interlace_ct0_d1_4x4);hide(raw_interlace_ct0_d1_4x4);hide(ts_interlace_ct0_d1_4x4);hide(t::valid_step);
    assert(t::trace(z_interlace_ct0_d1_4x4(),raw_interlace_ct0_d1_4x4(),ts_interlace_ct0_d1_4x4(),0,15nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_interlace_ct0_d1_4x4()
    ensures raw_interlace_ct0_d1_4x4().len()==14nat,
        z_interlace_ct0_d1_4x4().len()==22nat,
        z_interlace_ct0_d1_4x4()[0]==120u8,
        z_interlace_ct0_d1_4x4()[1]==156u8,
        zm::field_value(z_interlace_ct0_d1_4x4(),16,1)==1nat,
        zm::field_value(z_interlace_ct0_d1_4x4(),17,2)==1nat,
        zm::be_value(z_interlace_ct0_d1_4x4(),18nat,4)==211812908nat,
        ts_interlace_ct0_d1_4x4().len()==15nat,
        ts_interlace_ct0_d1_4x4()[0].pos==19nat,
        ts_interlace_ct0_d1_4x4()[0].before==0nat,
        ts_interlace_ct0_d1_4x4().last().next==139nat,
{
    hide(z_interlace_ct0_d1_4x4);hide(raw_interlace_ct0_d1_4x4);hide(ts_interlace_ct0_d1_4x4);
    assert(raw_interlace_ct0_d1_4x4().len()==14nat) by (compute);
    assert(z_interlace_ct0_d1_4x4().len()==22nat) by (compute);
    assert(z_interlace_ct0_d1_4x4()[0]==120u8) by (compute);
    assert(z_interlace_ct0_d1_4x4()[1]==156u8) by (compute);
    assert(zm::field_value(z_interlace_ct0_d1_4x4(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_interlace_ct0_d1_4x4(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_interlace_ct0_d1_4x4(),18nat,4)==211812908nat) by (compute);
    assert(ts_interlace_ct0_d1_4x4().len()==15nat) by (compute);
    assert(ts_interlace_ct0_d1_4x4()[0].pos==19nat) by (compute);
    assert(ts_interlace_ct0_d1_4x4()[0].before==0nat) by (compute);
    assert(ts_interlace_ct0_d1_4x4().last().next==139nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_interlace_ct0_d1_4x4()
    ensures zm::Adler(raw_interlace_ct0_d1_4x4())==211812908nat,
{
    hide(raw_interlace_ct0_d1_4x4);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_interlace_ct0_d1_4x4().len()==14nat) by (compute);
    assert(c::sums(raw_interlace_ct0_d1_4x4(),0,14nat)==(555nat,3232nat)) by (compute);
    c::sums_correct(raw_interlace_ct0_d1_4x4(),0,14nat);
    assert(raw_interlace_ct0_d1_4x4().subrange(0,14) =~= raw_interlace_ct0_d1_4x4());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((3232nat%65521)*(u16::MAX as nat+1)+(1+555nat)%65521==211812908nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_interlace_ct0_d1_4x4()
    ensures r::inflated(z_interlace_ct0_d1_4x4(),a_interlace_ct0_d1_4x4(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_interlace_ct0_d1_4x4(),r2:false}),
        raw_interlace_ct0_d1_4x4().len()==14nat,
{
    hide(z_interlace_ct0_d1_4x4);hide(raw_interlace_ct0_d1_4x4);hide(ts_interlace_ct0_d1_4x4);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_interlace_ct0_d1_4x4();checksum_interlace_ct0_d1_4x4();
    assert(r::raw(a_interlace_ct0_d1_4x4())==14nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_interlace_ct0_d1_4x4();
    stream::fixed_stream(z_interlace_ct0_d1_4x4(),raw_interlace_ct0_d1_4x4(),ts_interlace_ct0_d1_4x4(),14nat,156nat,211812908nat);
}
#[verifier::spinoff_prover]
proof fn scan_interlace_ct0_d1_4x4()
    ensures r::scan(raw_interlace_ct0_d1_4x4(),a_interlace_ct0_d1_4x4(),0)==Ok(()),
{
    hide(raw_interlace_ct0_d1_4x4);
    assert(r::count(a_interlace_ct0_d1_4x4())==7nat) by (compute);
    r::scan_step(raw_interlace_ct0_d1_4x4(),a_interlace_ct0_d1_4x4(),7);
    assert(r::pw(a_interlace_ct0_d1_4x4(),6nat)==4nat && r::ph(a_interlace_ct0_d1_4x4(),6nat)==2nat && r::rb(a_interlace_ct0_d1_4x4(),6nat)==1nat && r::base(a_interlace_ct0_d1_4x4(),6nat)==10nat) by (compute);
    assert(raw_interlace_ct0_d1_4x4()[10]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_4x4(),a_interlace_ct0_d1_4x4(),6nat,0nat)==Ok(()));
    assert(raw_interlace_ct0_d1_4x4()[12]==1u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_4x4(),a_interlace_ct0_d1_4x4(),6nat,1nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_4x4(),a_interlace_ct0_d1_4x4(),6nat,2nat);
    r::pass_step(raw_interlace_ct0_d1_4x4(),a_interlace_ct0_d1_4x4(),6nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_4x4(),a_interlace_ct0_d1_4x4(),6nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_4x4(),a_interlace_ct0_d1_4x4(),6nat);
    assert(r::pw(a_interlace_ct0_d1_4x4(),5nat)==2nat && r::ph(a_interlace_ct0_d1_4x4(),5nat)==2nat && r::rb(a_interlace_ct0_d1_4x4(),5nat)==1nat && r::base(a_interlace_ct0_d1_4x4(),5nat)==6nat) by (compute);
    assert(raw_interlace_ct0_d1_4x4()[6]==3u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_4x4(),a_interlace_ct0_d1_4x4(),5nat,0nat)==Ok(()));
    assert(raw_interlace_ct0_d1_4x4()[8]==4u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_4x4(),a_interlace_ct0_d1_4x4(),5nat,1nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_4x4(),a_interlace_ct0_d1_4x4(),5nat,2nat);
    r::pass_step(raw_interlace_ct0_d1_4x4(),a_interlace_ct0_d1_4x4(),5nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_4x4(),a_interlace_ct0_d1_4x4(),5nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_4x4(),a_interlace_ct0_d1_4x4(),5nat);
    assert(r::pw(a_interlace_ct0_d1_4x4(),4nat)==2nat && r::ph(a_interlace_ct0_d1_4x4(),4nat)==1nat && r::rb(a_interlace_ct0_d1_4x4(),4nat)==1nat && r::base(a_interlace_ct0_d1_4x4(),4nat)==4nat) by (compute);
    assert(raw_interlace_ct0_d1_4x4()[4]==2u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_4x4(),a_interlace_ct0_d1_4x4(),4nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_4x4(),a_interlace_ct0_d1_4x4(),4nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_4x4(),a_interlace_ct0_d1_4x4(),4nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_4x4(),a_interlace_ct0_d1_4x4(),4nat);
    assert(r::pw(a_interlace_ct0_d1_4x4(),3nat)==1nat && r::ph(a_interlace_ct0_d1_4x4(),3nat)==1nat && r::rb(a_interlace_ct0_d1_4x4(),3nat)==1nat && r::base(a_interlace_ct0_d1_4x4(),3nat)==2nat) by (compute);
    assert(raw_interlace_ct0_d1_4x4()[2]==1u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_4x4(),a_interlace_ct0_d1_4x4(),3nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_4x4(),a_interlace_ct0_d1_4x4(),3nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_4x4(),a_interlace_ct0_d1_4x4(),3nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_4x4(),a_interlace_ct0_d1_4x4(),3nat);
    assert(r::pw(a_interlace_ct0_d1_4x4(),2nat)==1nat && r::ph(a_interlace_ct0_d1_4x4(),2nat)==0nat && r::rb(a_interlace_ct0_d1_4x4(),2nat)==1nat && r::base(a_interlace_ct0_d1_4x4(),2nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct0_d1_4x4(),a_interlace_ct0_d1_4x4(),2nat,0);
    r::scan_step(raw_interlace_ct0_d1_4x4(),a_interlace_ct0_d1_4x4(),2nat);
    assert(r::pw(a_interlace_ct0_d1_4x4(),1nat)==0nat && r::ph(a_interlace_ct0_d1_4x4(),1nat)==1nat && r::rb(a_interlace_ct0_d1_4x4(),1nat)==0nat && r::base(a_interlace_ct0_d1_4x4(),1nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct0_d1_4x4(),a_interlace_ct0_d1_4x4(),1nat,0);
    r::scan_step(raw_interlace_ct0_d1_4x4(),a_interlace_ct0_d1_4x4(),1nat);
    assert(r::pw(a_interlace_ct0_d1_4x4(),0nat)==1nat && r::ph(a_interlace_ct0_d1_4x4(),0nat)==1nat && r::rb(a_interlace_ct0_d1_4x4(),0nat)==1nat && r::base(a_interlace_ct0_d1_4x4(),0nat)==0nat) by (compute);
    assert(raw_interlace_ct0_d1_4x4()[0]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_4x4(),a_interlace_ct0_d1_4x4(),0nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_4x4(),a_interlace_ct0_d1_4x4(),0nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_4x4(),a_interlace_ct0_d1_4x4(),0nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_4x4(),a_interlace_ct0_d1_4x4(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_interlace_ct0_d1_4x4()
    ensures r::still(bytes_interlace_ct0_d1_4x4(),STRICT,false)==(r::Projection::Ready {width:4u32,height:4u32,bytes:64nat}),
{
    hide(bytes_interlace_ct0_d1_4x4);hide(z_interlace_ct0_d1_4x4);hide(raw_interlace_ct0_d1_4x4);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_interlace_ct0_d1_4x4().len()==79nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct0_d1_4x4())==Ok((crate::model::Ihdr {width:4u32,height:4u32,depth:1u8,color_type:0u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct0_d1_4x4())==Ok((crate::model::Ihdr {width:4u32,height:4u32,depth:1u8,color_type:0u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 1u8)==2u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct0_d1_4x4(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:22nat,next:67nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct0_d1_4x4(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:22nat,next:67nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct0_d1_4x4(),67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:75nat,len:0nat,next:79nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct0_d1_4x4(),67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:75nat,len:0nat,next:79nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct0_d1_4x4().subrange(41,63)==z_interlace_ct0_d1_4x4()) by (compute);
    walk_interlace_ct0_d1_4x4(bytes_interlace_ct0_d1_4x4());
    inflate_interlace_ct0_d1_4x4();scan_interlace_ct0_d1_4x4();
    assert(r::raw(a_interlace_ct0_d1_4x4())==14nat) by (compute);
    assert(r::data(z_interlace_ct0_d1_4x4(),a_interlace_ct0_d1_4x4(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_interlace_ct0_d1_4x4(),h_interlace_ct0_d1_4x4(),z_interlace_ct0_d1_4x4(),0nat,false);
}
fn public_interlace_ct0_d1_4x4() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,4u8,1u8,0u8,0u8,0u8,1u8,246u8,141u8,147u8,69u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,96u8,108u8,96u8,114u8,96u8,110u8,96u8,97u8,96u8,16u8,96u8,188u8,0u8,0u8,12u8,160u8,2u8,44u8,67u8,163u8,24u8,60u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_interlace_ct0_d1_4x4());model_interlace_ct0_d1_4x4();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==4u32 && s.height==4u32 && s.rgba@.len()==64nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// interlace/ct6-d16-4x4; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_interlace_ct6_d16_4x4() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,4u8,16u8,6u8,0u8,0u8,1u8,142u8,102u8,114u8,171u8,0u8,0u8,0u8,146u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,135u8,0u8,120u8,255u8,0u8,122u8,246u8,68u8,152u8,139u8,160u8,154u8,125u8,1u8,128u8,248u8,33u8,65u8,225u8,44u8,127u8,209u8,2u8,34u8,1u8,106u8,171u8,17u8,142u8,109u8,159u8,210u8,235u8,90u8,69u8,167u8,88u8,251u8,146u8,3u8,89u8,99u8,28u8,228u8,114u8,190u8,66u8,183u8,82u8,208u8,203u8,70u8,9u8,201u8,128u8,60u8,4u8,227u8,188u8,98u8,136u8,174u8,239u8,181u8,178u8,247u8,96u8,115u8,132u8,13u8,132u8,182u8,7u8,0u8,2u8,221u8,193u8,227u8,42u8,12u8,247u8,149u8,157u8,104u8,81u8,118u8,29u8,89u8,88u8,80u8,224u8,224u8,175u8,1u8,6u8,183u8,33u8,111u8,214u8,164u8,82u8,104u8,209u8,190u8,133u8,50u8,1u8,199u8,205u8,114u8,175u8,189u8,95u8,142u8,142u8,88u8,253u8,143u8,52u8,127u8,76u8,91u8,163u8,179u8,244u8,113u8,34u8,186u8,167u8,73u8,208u8,4u8,0u8,133u8,63u8,197u8,90u8,116u8,217u8,41u8,230u8,65u8,135u8,16u8,12u8,143u8,198u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_interlace_ct6_d16_4x4() -> Seq<u8> {seq![120u8,156u8,1u8,135u8,0u8,120u8,255u8,0u8,122u8,246u8,68u8,152u8,139u8,160u8,154u8,125u8,1u8,128u8,248u8,33u8,65u8,225u8,44u8,127u8,209u8,2u8,34u8,1u8,106u8,171u8,17u8,142u8,109u8,159u8,210u8,235u8,90u8,69u8,167u8,88u8,251u8,146u8,3u8,89u8,99u8,28u8,228u8,114u8,190u8,66u8,183u8,82u8,208u8,203u8,70u8,9u8,201u8,128u8,60u8,4u8,227u8,188u8,98u8,136u8,174u8,239u8,181u8,178u8,247u8,96u8,115u8,132u8,13u8,132u8,182u8,7u8,0u8,2u8,221u8,193u8,227u8,42u8,12u8,247u8,149u8,157u8,104u8,81u8,118u8,29u8,89u8,88u8,80u8,224u8,224u8,175u8,1u8,6u8,183u8,33u8,111u8,214u8,164u8,82u8,104u8,209u8,190u8,133u8,50u8,1u8,199u8,205u8,114u8,175u8,189u8,95u8,142u8,142u8,88u8,253u8,143u8,52u8,127u8,76u8,91u8,163u8,179u8,244u8,113u8,34u8,186u8,167u8,73u8,208u8,4u8,0u8,133u8,63u8,197u8,90u8,116u8,217u8,41u8,230u8,65u8,135u8]}
spec fn h_interlace_ct6_d16_4x4() -> crate::model::Ihdr {crate::model::Ihdr {width:4u32,height:4u32,depth:16u8,color_type:6u8,interlace:1u8}}
spec fn a_interlace_ct6_d16_4x4() -> r::Raster {r::shape(h_interlace_ct6_d16_4x4(),4nat,4nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_interlace_ct6_d16_4x4(b: Seq<u8>)
    requires b.len()==203,crate::model::mh(b)==Ok((crate::model::Ihdr {width:4u32,height:4u32,depth:16u8,color_type:6u8,interlace:1u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:146nat,next:191nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,191nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:199nat,len:0nat,next:203nat,crc_ok:true,synthetic:false})),
        b.subrange(41,187)==z_interlace_ct6_d16_4x4(),
    ensures r::default_case(b,crate::model::Ihdr {width:4u32,height:4u32,depth:16u8,color_type:6u8,interlace:1u8},z_interlace_ct6_d16_4x4(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    hide(z_interlace_ct6_d16_4x4);
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_interlace_ct6_d16_4x4() -> Seq<u8> {seq![0u8,122u8,246u8,68u8,152u8,139u8,160u8,154u8,125u8,1u8,128u8,248u8,33u8,65u8,225u8,44u8,127u8,209u8,2u8,34u8,1u8,106u8,171u8,17u8,142u8,109u8,159u8,210u8,235u8,90u8,69u8,167u8,88u8,251u8,146u8,3u8,89u8,99u8,28u8,228u8,114u8,190u8,66u8,183u8,82u8,208u8,203u8,70u8,9u8,201u8,128u8,60u8,4u8,227u8,188u8,98u8,136u8,174u8,239u8,181u8,178u8,247u8,96u8,115u8,132u8,13u8,132u8,182u8,7u8,0u8,2u8,221u8,193u8,227u8,42u8,12u8,247u8,149u8,157u8,104u8,81u8,118u8,29u8,89u8,88u8,80u8,224u8,224u8,175u8,1u8,6u8,183u8,33u8,111u8,214u8,164u8,82u8,104u8,209u8,190u8,133u8,50u8,1u8,199u8,205u8,114u8,175u8,189u8,95u8,142u8,142u8,88u8,253u8,143u8,52u8,127u8,76u8,91u8,163u8,179u8,244u8,113u8,34u8,186u8,167u8,73u8,208u8,4u8,0u8,133u8,63u8,197u8,90u8,116u8,217u8]}
#[verifier::spinoff_prover]
proof fn shape_interlace_ct6_d16_4x4()
    ensures raw_interlace_ct6_d16_4x4().len()==135nat,
        z_interlace_ct6_d16_4x4().len()==146nat,
        z_interlace_ct6_d16_4x4()[0]==120u8,
        z_interlace_ct6_d16_4x4()[1]==156u8,
        zm::field_value(z_interlace_ct6_d16_4x4(),16,1)==1nat,
        zm::field_value(z_interlace_ct6_d16_4x4(),17,2)==0nat,
        zm::be_value(z_interlace_ct6_d16_4x4(),142nat,4)==702955911nat,
        z_interlace_ct6_d16_4x4().subrange(7,142)==raw_interlace_ct6_d16_4x4(),
        z_interlace_ct6_d16_4x4()[3] as nat+256*z_interlace_ct6_d16_4x4()[4] as nat==135nat,
        z_interlace_ct6_d16_4x4()[5] as nat+256*z_interlace_ct6_d16_4x4()[6] as nat==u16::MAX as nat-135nat,
        zm::be_value(z_interlace_ct6_d16_4x4(),0,1)==120nat,
        zm::be_value(z_interlace_ct6_d16_4x4(),1,1)==156nat,
{
    hide(z_interlace_ct6_d16_4x4);hide(raw_interlace_ct6_d16_4x4);
    assert(raw_interlace_ct6_d16_4x4().len()==135nat) by (compute);
    assert(z_interlace_ct6_d16_4x4().len()==146nat) by (compute);
    assert(z_interlace_ct6_d16_4x4()[0]==120u8) by (compute);
    assert(z_interlace_ct6_d16_4x4()[1]==156u8) by (compute);
    assert(zm::field_value(z_interlace_ct6_d16_4x4(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_interlace_ct6_d16_4x4(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_interlace_ct6_d16_4x4(),142nat,4)==702955911nat) by (compute);
    assert(z_interlace_ct6_d16_4x4().subrange(7,142)==raw_interlace_ct6_d16_4x4()) by (compute);
    assert(z_interlace_ct6_d16_4x4()[3] as nat+256*z_interlace_ct6_d16_4x4()[4] as nat==135nat) by (compute);
    assert(z_interlace_ct6_d16_4x4()[5] as nat+256*z_interlace_ct6_d16_4x4()[6] as nat==u16::MAX as nat-135nat) by (compute);
    assert(zm::be_value(z_interlace_ct6_d16_4x4(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_interlace_ct6_d16_4x4(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_interlace_ct6_d16_4x4()
    ensures zm::Adler(raw_interlace_ct6_d16_4x4())==702955911nat,
{
    hide(raw_interlace_ct6_d16_4x4);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_interlace_ct6_d16_4x4().len()==135nat) by (compute);
    assert(c::sums(raw_interlace_ct6_d16_4x4(),0,135nat)==(16774nat,1124583nat)) by (compute);
    c::sums_correct(raw_interlace_ct6_d16_4x4(),0,135nat);
    assert(raw_interlace_ct6_d16_4x4().subrange(0,135) =~= raw_interlace_ct6_d16_4x4());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((1124583nat%65521)*(u16::MAX as nat+1)+(1+16774nat)%65521==702955911nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_interlace_ct6_d16_4x4()
    ensures r::inflated(z_interlace_ct6_d16_4x4(),a_interlace_ct6_d16_4x4(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_interlace_ct6_d16_4x4(),r2:false}),
        raw_interlace_ct6_d16_4x4().len()==135nat,
{
    hide(z_interlace_ct6_d16_4x4);hide(raw_interlace_ct6_d16_4x4);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_interlace_ct6_d16_4x4();checksum_interlace_ct6_d16_4x4();
    assert(r::raw(a_interlace_ct6_d16_4x4())==135nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_interlace_ct6_d16_4x4(),raw_interlace_ct6_d16_4x4(),135nat,120nat,156nat,702955911nat);
}
#[verifier::spinoff_prover]
proof fn scan_interlace_ct6_d16_4x4()
    ensures r::scan(raw_interlace_ct6_d16_4x4(),a_interlace_ct6_d16_4x4(),0)==Ok(()),
{
    hide(raw_interlace_ct6_d16_4x4);
    assert(r::count(a_interlace_ct6_d16_4x4())==7nat) by (compute);
    r::scan_step(raw_interlace_ct6_d16_4x4(),a_interlace_ct6_d16_4x4(),7);
    assert(r::pw(a_interlace_ct6_d16_4x4(),6nat)==4nat && r::ph(a_interlace_ct6_d16_4x4(),6nat)==2nat && r::rb(a_interlace_ct6_d16_4x4(),6nat)==32nat && r::base(a_interlace_ct6_d16_4x4(),6nat)==69nat) by (compute);
    assert(raw_interlace_ct6_d16_4x4()[69]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_4x4(),a_interlace_ct6_d16_4x4(),6nat,0nat)==Ok(()));
    assert(raw_interlace_ct6_d16_4x4()[102]==1u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_4x4(),a_interlace_ct6_d16_4x4(),6nat,1nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_4x4(),a_interlace_ct6_d16_4x4(),6nat,2nat);
    r::pass_step(raw_interlace_ct6_d16_4x4(),a_interlace_ct6_d16_4x4(),6nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_4x4(),a_interlace_ct6_d16_4x4(),6nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_4x4(),a_interlace_ct6_d16_4x4(),6nat);
    assert(r::pw(a_interlace_ct6_d16_4x4(),5nat)==2nat && r::ph(a_interlace_ct6_d16_4x4(),5nat)==2nat && r::rb(a_interlace_ct6_d16_4x4(),5nat)==16nat && r::base(a_interlace_ct6_d16_4x4(),5nat)==35nat) by (compute);
    assert(raw_interlace_ct6_d16_4x4()[35]==3u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_4x4(),a_interlace_ct6_d16_4x4(),5nat,0nat)==Ok(()));
    assert(raw_interlace_ct6_d16_4x4()[52]==4u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_4x4(),a_interlace_ct6_d16_4x4(),5nat,1nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_4x4(),a_interlace_ct6_d16_4x4(),5nat,2nat);
    r::pass_step(raw_interlace_ct6_d16_4x4(),a_interlace_ct6_d16_4x4(),5nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_4x4(),a_interlace_ct6_d16_4x4(),5nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_4x4(),a_interlace_ct6_d16_4x4(),5nat);
    assert(r::pw(a_interlace_ct6_d16_4x4(),4nat)==2nat && r::ph(a_interlace_ct6_d16_4x4(),4nat)==1nat && r::rb(a_interlace_ct6_d16_4x4(),4nat)==16nat && r::base(a_interlace_ct6_d16_4x4(),4nat)==18nat) by (compute);
    assert(raw_interlace_ct6_d16_4x4()[18]==2u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_4x4(),a_interlace_ct6_d16_4x4(),4nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_4x4(),a_interlace_ct6_d16_4x4(),4nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_4x4(),a_interlace_ct6_d16_4x4(),4nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_4x4(),a_interlace_ct6_d16_4x4(),4nat);
    assert(r::pw(a_interlace_ct6_d16_4x4(),3nat)==1nat && r::ph(a_interlace_ct6_d16_4x4(),3nat)==1nat && r::rb(a_interlace_ct6_d16_4x4(),3nat)==8nat && r::base(a_interlace_ct6_d16_4x4(),3nat)==9nat) by (compute);
    assert(raw_interlace_ct6_d16_4x4()[9]==1u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_4x4(),a_interlace_ct6_d16_4x4(),3nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_4x4(),a_interlace_ct6_d16_4x4(),3nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_4x4(),a_interlace_ct6_d16_4x4(),3nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_4x4(),a_interlace_ct6_d16_4x4(),3nat);
    assert(r::pw(a_interlace_ct6_d16_4x4(),2nat)==1nat && r::ph(a_interlace_ct6_d16_4x4(),2nat)==0nat && r::rb(a_interlace_ct6_d16_4x4(),2nat)==8nat && r::base(a_interlace_ct6_d16_4x4(),2nat)==9nat) by (compute);
    r::pass_step(raw_interlace_ct6_d16_4x4(),a_interlace_ct6_d16_4x4(),2nat,0);
    r::scan_step(raw_interlace_ct6_d16_4x4(),a_interlace_ct6_d16_4x4(),2nat);
    assert(r::pw(a_interlace_ct6_d16_4x4(),1nat)==0nat && r::ph(a_interlace_ct6_d16_4x4(),1nat)==1nat && r::rb(a_interlace_ct6_d16_4x4(),1nat)==0nat && r::base(a_interlace_ct6_d16_4x4(),1nat)==9nat) by (compute);
    r::pass_step(raw_interlace_ct6_d16_4x4(),a_interlace_ct6_d16_4x4(),1nat,0);
    r::scan_step(raw_interlace_ct6_d16_4x4(),a_interlace_ct6_d16_4x4(),1nat);
    assert(r::pw(a_interlace_ct6_d16_4x4(),0nat)==1nat && r::ph(a_interlace_ct6_d16_4x4(),0nat)==1nat && r::rb(a_interlace_ct6_d16_4x4(),0nat)==8nat && r::base(a_interlace_ct6_d16_4x4(),0nat)==0nat) by (compute);
    assert(raw_interlace_ct6_d16_4x4()[0]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_4x4(),a_interlace_ct6_d16_4x4(),0nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_4x4(),a_interlace_ct6_d16_4x4(),0nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_4x4(),a_interlace_ct6_d16_4x4(),0nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_4x4(),a_interlace_ct6_d16_4x4(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_interlace_ct6_d16_4x4()
    ensures r::still(bytes_interlace_ct6_d16_4x4(),STRICT,false)==(r::Projection::Ready {width:4u32,height:4u32,bytes:64nat}),
{
    hide(bytes_interlace_ct6_d16_4x4);hide(z_interlace_ct6_d16_4x4);hide(raw_interlace_ct6_d16_4x4);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_interlace_ct6_d16_4x4().len()==203nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct6_d16_4x4())==Ok((crate::model::Ihdr {width:4u32,height:4u32,depth:16u8,color_type:6u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct6_d16_4x4())==Ok((crate::model::Ihdr {width:4u32,height:4u32,depth:16u8,color_type:6u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct6_d16_4x4(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:146nat,next:191nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct6_d16_4x4(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:146nat,next:191nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct6_d16_4x4(),191nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:199nat,len:0nat,next:203nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct6_d16_4x4(),191nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:199nat,len:0nat,next:203nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct6_d16_4x4().subrange(41,187)==z_interlace_ct6_d16_4x4()) by (compute);
    walk_interlace_ct6_d16_4x4(bytes_interlace_ct6_d16_4x4());
    inflate_interlace_ct6_d16_4x4();scan_interlace_ct6_d16_4x4();
    assert(r::raw(a_interlace_ct6_d16_4x4())==135nat) by (compute);
    assert(r::data(z_interlace_ct6_d16_4x4(),a_interlace_ct6_d16_4x4(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_interlace_ct6_d16_4x4(),h_interlace_ct6_d16_4x4(),z_interlace_ct6_d16_4x4(),0nat,false);
}
fn public_interlace_ct6_d16_4x4() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,4u8,16u8,6u8,0u8,0u8,1u8,142u8,102u8,114u8,171u8,0u8,0u8,0u8,146u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,135u8,0u8,120u8,255u8,0u8,122u8,246u8,68u8,152u8,139u8,160u8,154u8,125u8,1u8,128u8,248u8,33u8,65u8,225u8,44u8,127u8,209u8,2u8,34u8,1u8,106u8,171u8,17u8,142u8,109u8,159u8,210u8,235u8,90u8,69u8,167u8,88u8,251u8,146u8,3u8,89u8,99u8,28u8,228u8,114u8,190u8,66u8,183u8,82u8,208u8,203u8,70u8,9u8,201u8,128u8,60u8,4u8,227u8,188u8,98u8,136u8,174u8,239u8,181u8,178u8,247u8,96u8,115u8,132u8,13u8,132u8,182u8,7u8,0u8,2u8,221u8,193u8,227u8,42u8,12u8,247u8,149u8,157u8,104u8,81u8,118u8,29u8,89u8,88u8,80u8,224u8,224u8,175u8,1u8,6u8,183u8,33u8,111u8,214u8,164u8,82u8,104u8,209u8,190u8,133u8,50u8,1u8,199u8,205u8,114u8,175u8,189u8,95u8,142u8,142u8,88u8,253u8,143u8,52u8,127u8,76u8,91u8,163u8,179u8,244u8,113u8,34u8,186u8,167u8,73u8,208u8,4u8,0u8,133u8,63u8,197u8,90u8,116u8,217u8,41u8,230u8,65u8,135u8,16u8,12u8,143u8,198u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_interlace_ct6_d16_4x4());model_interlace_ct6_d16_4x4();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==4u32 && s.height==4u32 && s.rgba@.len()==64nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// interlace/ct3-d4-4x4; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_interlace_ct3_d4_4x4() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,4u8,4u8,3u8,0u8,0u8,1u8,44u8,216u8,179u8,219u8,0u8,0u8,0u8,48u8,80u8,76u8,84u8,69u8,154u8,62u8,40u8,122u8,148u8,201u8,94u8,104u8,241u8,34u8,148u8,250u8,101u8,127u8,11u8,235u8,20u8,153u8,70u8,219u8,142u8,44u8,135u8,68u8,215u8,177u8,205u8,225u8,240u8,138u8,16u8,249u8,75u8,193u8,13u8,84u8,197u8,59u8,212u8,181u8,168u8,32u8,235u8,247u8,221u8,159u8,115u8,37u8,93u8,242u8,195u8,173u8,0u8,0u8,0u8,24u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,112u8,96u8,12u8,96u8,202u8,99u8,222u8,199u8,226u8,202u8,144u8,28u8,198u8,168u8,238u8,10u8,0u8,25u8,108u8,3u8,50u8,13u8,59u8,193u8,49u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_interlace_ct3_d4_4x4() -> Seq<u8> {seq![120u8,156u8,99u8,112u8,96u8,12u8,96u8,202u8,99u8,222u8,199u8,226u8,202u8,144u8,28u8,198u8,168u8,238u8,10u8,0u8,25u8,108u8,3u8,50u8]}
spec fn h_interlace_ct3_d4_4x4() -> crate::model::Ihdr {crate::model::Ihdr {width:4u32,height:4u32,depth:4u8,color_type:3u8,interlace:1u8}}
spec fn a_interlace_ct3_d4_4x4() -> r::Raster {r::shape(h_interlace_ct3_d4_4x4(),4nat,4nat,16nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_interlace_ct3_d4_4x4(b: Seq<u8>)
    requires b.len()==141,crate::model::mh(b)==Ok((crate::model::Ihdr {width:4u32,height:4u32,depth:4u8,color_type:3u8,interlace:1u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:24nat,next:129nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,129nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:137nat,len:0nat,next:141nat,crc_ok:true,synthetic:false})),
        b.subrange(101,125)==z_interlace_ct3_d4_4x4(),
    ensures r::default_case(b,crate::model::Ihdr {width:4u32,height:4u32,depth:4u8,color_type:3u8,interlace:1u8},z_interlace_ct3_d4_4x4(),16nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);
    hide(z_interlace_ct3_d4_4x4);
    r::palette_default_case(b,crate::model::Ihdr {width:4u32,height:4u32,depth:4u8,color_type:3u8,interlace:1u8},z_interlace_ct3_d4_4x4(),24nat);
}
spec fn raw_interlace_ct3_d4_4x4() -> Seq<u8> {seq![0u8,64u8,1u8,80u8,2u8,110u8,3u8,190u8,4u8,69u8,0u8,99u8,86u8,1u8,39u8,69u8]}
spec fn ts_interlace_ct3_d4_4x4() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:35nat,before:1nat,after:2nat,symbol:64nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:35nat,next:43nat,before:2nat,after:3nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:43nat,next:51nat,before:3nat,after:4nat,symbol:80nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:51nat,next:59nat,before:4nat,after:5nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:59nat,next:67nat,before:5nat,after:6nat,symbol:110nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:67nat,next:75nat,before:6nat,after:7nat,symbol:3nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:75nat,next:84nat,before:7nat,after:8nat,symbol:190nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:84nat,next:92nat,before:8nat,after:9nat,symbol:4nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:92nat,next:100nat,before:9nat,after:10nat,symbol:69nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:100nat,next:108nat,before:10nat,after:11nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:108nat,next:116nat,before:11nat,after:12nat,symbol:99nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:116nat,next:124nat,before:12nat,after:13nat,symbol:86nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:124nat,next:132nat,before:13nat,after:14nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:132nat,next:140nat,before:14nat,after:15nat,symbol:39nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:140nat,next:148nat,before:15nat,after:16nat,symbol:69nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:148nat,next:155nat,before:16nat,after:16nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_interlace_ct3_d4_4x4()
    ensures t::trace(z_interlace_ct3_d4_4x4(),raw_interlace_ct3_d4_4x4(),ts_interlace_ct3_d4_4x4(),0,17nat),
{
    hide(z_interlace_ct3_d4_4x4);hide(raw_interlace_ct3_d4_4x4);hide(ts_interlace_ct3_d4_4x4);hide(t::valid_step);
    assert(t::trace(z_interlace_ct3_d4_4x4(),raw_interlace_ct3_d4_4x4(),ts_interlace_ct3_d4_4x4(),0,17nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_interlace_ct3_d4_4x4()
    ensures raw_interlace_ct3_d4_4x4().len()==16nat,
        z_interlace_ct3_d4_4x4().len()==24nat,
        z_interlace_ct3_d4_4x4()[0]==120u8,
        z_interlace_ct3_d4_4x4()[1]==156u8,
        zm::field_value(z_interlace_ct3_d4_4x4(),16,1)==1nat,
        zm::field_value(z_interlace_ct3_d4_4x4(),17,2)==1nat,
        zm::be_value(z_interlace_ct3_d4_4x4(),20nat,4)==426509106nat,
        ts_interlace_ct3_d4_4x4().len()==17nat,
        ts_interlace_ct3_d4_4x4()[0].pos==19nat,
        ts_interlace_ct3_d4_4x4()[0].before==0nat,
        ts_interlace_ct3_d4_4x4().last().next==155nat,
{
    hide(z_interlace_ct3_d4_4x4);hide(raw_interlace_ct3_d4_4x4);hide(ts_interlace_ct3_d4_4x4);
    assert(raw_interlace_ct3_d4_4x4().len()==16nat) by (compute);
    assert(z_interlace_ct3_d4_4x4().len()==24nat) by (compute);
    assert(z_interlace_ct3_d4_4x4()[0]==120u8) by (compute);
    assert(z_interlace_ct3_d4_4x4()[1]==156u8) by (compute);
    assert(zm::field_value(z_interlace_ct3_d4_4x4(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_interlace_ct3_d4_4x4(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_interlace_ct3_d4_4x4(),20nat,4)==426509106nat) by (compute);
    assert(ts_interlace_ct3_d4_4x4().len()==17nat) by (compute);
    assert(ts_interlace_ct3_d4_4x4()[0].pos==19nat) by (compute);
    assert(ts_interlace_ct3_d4_4x4()[0].before==0nat) by (compute);
    assert(ts_interlace_ct3_d4_4x4().last().next==155nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_interlace_ct3_d4_4x4()
    ensures zm::Adler(raw_interlace_ct3_d4_4x4())==426509106nat,
{
    hide(raw_interlace_ct3_d4_4x4);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_interlace_ct3_d4_4x4().len()==16nat) by (compute);
    assert(c::sums(raw_interlace_ct3_d4_4x4(),0,16nat)==(817nat,6508nat)) by (compute);
    c::sums_correct(raw_interlace_ct3_d4_4x4(),0,16nat);
    assert(raw_interlace_ct3_d4_4x4().subrange(0,16) =~= raw_interlace_ct3_d4_4x4());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((6508nat%65521)*(u16::MAX as nat+1)+(1+817nat)%65521==426509106nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_interlace_ct3_d4_4x4()
    ensures r::inflated(z_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_interlace_ct3_d4_4x4(),r2:false}),
        raw_interlace_ct3_d4_4x4().len()==16nat,
{
    hide(z_interlace_ct3_d4_4x4);hide(raw_interlace_ct3_d4_4x4);hide(ts_interlace_ct3_d4_4x4);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_interlace_ct3_d4_4x4();checksum_interlace_ct3_d4_4x4();
    assert(r::raw(a_interlace_ct3_d4_4x4())==16nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_interlace_ct3_d4_4x4();
    stream::fixed_stream(z_interlace_ct3_d4_4x4(),raw_interlace_ct3_d4_4x4(),ts_interlace_ct3_d4_4x4(),16nat,156nat,426509106nat);
}
#[verifier::spinoff_prover]
proof fn scan_interlace_ct3_d4_4x4()
    ensures r::scan(raw_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),0)==Ok(()),
{
    hide(raw_interlace_ct3_d4_4x4);
    assert(r::count(a_interlace_ct3_d4_4x4())==7nat) by (compute);
    r::scan_step(raw_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),7);
    assert(r::pw(a_interlace_ct3_d4_4x4(),6nat)==4nat && r::ph(a_interlace_ct3_d4_4x4(),6nat)==2nat && r::rb(a_interlace_ct3_d4_4x4(),6nat)==2nat && r::base(a_interlace_ct3_d4_4x4(),6nat)==10nat) by (compute);
    assert(raw_interlace_ct3_d4_4x4()[10]==0u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),6nat,0nat),4nat);
    assert(r::row_result(raw_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),6nat,0nat)==Ok(()));
    assert(raw_interlace_ct3_d4_4x4()[13]==1u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),6nat,1nat),4nat);
    assert(r::row_result(raw_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),6nat,1nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),6nat,2nat);
    r::pass_step(raw_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),6nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),6nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),6nat);
    assert(r::pw(a_interlace_ct3_d4_4x4(),5nat)==2nat && r::ph(a_interlace_ct3_d4_4x4(),5nat)==2nat && r::rb(a_interlace_ct3_d4_4x4(),5nat)==1nat && r::base(a_interlace_ct3_d4_4x4(),5nat)==6nat) by (compute);
    assert(raw_interlace_ct3_d4_4x4()[6]==3u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),5nat,0nat),2nat);
    assert(r::row_result(raw_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),5nat,0nat)==Ok(()));
    assert(raw_interlace_ct3_d4_4x4()[8]==4u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),5nat,1nat),2nat);
    assert(r::row_result(raw_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),5nat,1nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),5nat,2nat);
    r::pass_step(raw_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),5nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),5nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),5nat);
    assert(r::pw(a_interlace_ct3_d4_4x4(),4nat)==2nat && r::ph(a_interlace_ct3_d4_4x4(),4nat)==1nat && r::rb(a_interlace_ct3_d4_4x4(),4nat)==1nat && r::base(a_interlace_ct3_d4_4x4(),4nat)==4nat) by (compute);
    assert(raw_interlace_ct3_d4_4x4()[4]==2u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),4nat,0nat),2nat);
    assert(r::row_result(raw_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),4nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),4nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),4nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),4nat);
    assert(r::pw(a_interlace_ct3_d4_4x4(),3nat)==1nat && r::ph(a_interlace_ct3_d4_4x4(),3nat)==1nat && r::rb(a_interlace_ct3_d4_4x4(),3nat)==1nat && r::base(a_interlace_ct3_d4_4x4(),3nat)==2nat) by (compute);
    assert(raw_interlace_ct3_d4_4x4()[2]==1u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),3nat,0nat),1nat);
    assert(r::row_result(raw_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),3nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),3nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),3nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),3nat);
    assert(r::pw(a_interlace_ct3_d4_4x4(),2nat)==1nat && r::ph(a_interlace_ct3_d4_4x4(),2nat)==0nat && r::rb(a_interlace_ct3_d4_4x4(),2nat)==1nat && r::base(a_interlace_ct3_d4_4x4(),2nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),2nat,0);
    r::scan_step(raw_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),2nat);
    assert(r::pw(a_interlace_ct3_d4_4x4(),1nat)==0nat && r::ph(a_interlace_ct3_d4_4x4(),1nat)==1nat && r::rb(a_interlace_ct3_d4_4x4(),1nat)==0nat && r::base(a_interlace_ct3_d4_4x4(),1nat)==2nat) by (compute);
    r::pass_step(raw_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),1nat,0);
    r::scan_step(raw_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),1nat);
    assert(r::pw(a_interlace_ct3_d4_4x4(),0nat)==1nat && r::ph(a_interlace_ct3_d4_4x4(),0nat)==1nat && r::rb(a_interlace_ct3_d4_4x4(),0nat)==1nat && r::base(a_interlace_ct3_d4_4x4(),0nat)==0nat) by (compute);
    assert(raw_interlace_ct3_d4_4x4()[0]==0u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),0nat,0nat),1nat);
    assert(r::row_result(raw_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),0nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),0nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),0nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_interlace_ct3_d4_4x4()
    ensures r::still(bytes_interlace_ct3_d4_4x4(),STRICT,false)==(r::Projection::Ready {width:4u32,height:4u32,bytes:64nat}),
{
    hide(bytes_interlace_ct3_d4_4x4);hide(z_interlace_ct3_d4_4x4);hide(raw_interlace_ct3_d4_4x4);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);
    assert(bytes_interlace_ct3_d4_4x4().len()==141nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct3_d4_4x4())==Ok((crate::model::Ihdr {width:4u32,height:4u32,depth:4u8,color_type:3u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct3_d4_4x4())==Ok((crate::model::Ihdr {width:4u32,height:4u32,depth:4u8,color_type:3u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 4u8)==16u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_4x4(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_4x4(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(48nat,3u8,4u8,false,false,false,false)==crate::metadata_model::Decision::Keep(16nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_4x4(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:24nat,next:129nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_4x4(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:24nat,next:129nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_4x4(),129nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:137nat,len:0nat,next:141nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_4x4(),129nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:137nat,len:0nat,next:141nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct3_d4_4x4().subrange(101,125)==z_interlace_ct3_d4_4x4()) by (compute);
    walk_interlace_ct3_d4_4x4(bytes_interlace_ct3_d4_4x4());
    inflate_interlace_ct3_d4_4x4();scan_interlace_ct3_d4_4x4();
    assert(r::raw(a_interlace_ct3_d4_4x4())==16nat) by (compute);
    assert(r::data(z_interlace_ct3_d4_4x4(),a_interlace_ct3_d4_4x4(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_interlace_ct3_d4_4x4(),h_interlace_ct3_d4_4x4(),z_interlace_ct3_d4_4x4(),16nat,false);
}
fn public_interlace_ct3_d4_4x4() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,4u8,4u8,3u8,0u8,0u8,1u8,44u8,216u8,179u8,219u8,0u8,0u8,0u8,48u8,80u8,76u8,84u8,69u8,154u8,62u8,40u8,122u8,148u8,201u8,94u8,104u8,241u8,34u8,148u8,250u8,101u8,127u8,11u8,235u8,20u8,153u8,70u8,219u8,142u8,44u8,135u8,68u8,215u8,177u8,205u8,225u8,240u8,138u8,16u8,249u8,75u8,193u8,13u8,84u8,197u8,59u8,212u8,181u8,168u8,32u8,235u8,247u8,221u8,159u8,115u8,37u8,93u8,242u8,195u8,173u8,0u8,0u8,0u8,24u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,112u8,96u8,12u8,96u8,202u8,99u8,222u8,199u8,226u8,202u8,144u8,28u8,198u8,168u8,238u8,10u8,0u8,25u8,108u8,3u8,50u8,13u8,59u8,193u8,49u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_interlace_ct3_d4_4x4());model_interlace_ct3_d4_4x4();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==4u32 && s.height==4u32 && s.rgba@.len()==64nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// interlace/ct0-d1-5x5; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_interlace_ct0_d1_5x5() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,5u8,0u8,0u8,0u8,5u8,1u8,0u8,0u8,0u8,1u8,210u8,19u8,43u8,222u8,0u8,0u8,0u8,27u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,104u8,96u8,108u8,96u8,58u8,192u8,220u8,192u8,210u8,192u8,176u8,0u8,204u8,122u8,192u8,114u8,131u8,65u8,2u8,0u8,71u8,82u8,6u8,133u8,90u8,167u8,112u8,214u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_interlace_ct0_d1_5x5() -> Seq<u8> {seq![120u8,156u8,99u8,104u8,96u8,108u8,96u8,58u8,192u8,220u8,192u8,210u8,192u8,176u8,0u8,204u8,122u8,192u8,114u8,131u8,65u8,2u8,0u8,71u8,82u8,6u8,133u8]}
spec fn h_interlace_ct0_d1_5x5() -> crate::model::Ihdr {crate::model::Ihdr {width:5u32,height:5u32,depth:1u8,color_type:0u8,interlace:1u8}}
spec fn a_interlace_ct0_d1_5x5() -> r::Raster {r::shape(h_interlace_ct0_d1_5x5(),5nat,5nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_interlace_ct0_d1_5x5(b: Seq<u8>)
    requires b.len()==84,crate::model::mh(b)==Ok((crate::model::Ihdr {width:5u32,height:5u32,depth:1u8,color_type:0u8,interlace:1u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:27nat,next:72nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,72nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:80nat,len:0nat,next:84nat,crc_ok:true,synthetic:false})),
        b.subrange(41,68)==z_interlace_ct0_d1_5x5(),
    ensures r::default_case(b,crate::model::Ihdr {width:5u32,height:5u32,depth:1u8,color_type:0u8,interlace:1u8},z_interlace_ct0_d1_5x5(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    hide(z_interlace_ct0_d1_5x5);
        assert((1u32 << 1u8)==2u32) by (compute);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_interlace_ct0_d1_5x5() -> Seq<u8> {seq![0u8,128u8,1u8,128u8,2u8,192u8,3u8,128u8,4u8,128u8,0u8,160u8,1u8,128u8,2u8,192u8,3u8,224u8,4u8,216u8,0u8,24u8]}
spec fn ts_interlace_ct0_d1_5x5() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:35nat,before:1nat,after:2nat,symbol:128nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:35nat,next:43nat,before:2nat,after:3nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:43nat,next:51nat,before:3nat,after:4nat,symbol:128nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:51nat,next:59nat,before:4nat,after:5nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:59nat,next:68nat,before:5nat,after:6nat,symbol:192nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:68nat,next:76nat,before:6nat,after:7nat,symbol:3nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:76nat,next:84nat,before:7nat,after:8nat,symbol:128nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:84nat,next:92nat,before:8nat,after:9nat,symbol:4nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:92nat,next:100nat,before:9nat,after:10nat,symbol:128nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:100nat,next:108nat,before:10nat,after:11nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:108nat,next:117nat,before:11nat,after:12nat,symbol:160nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:117nat,next:131nat,before:12nat,after:17nat,symbol:259nat,le:0nat,ds:6nat,de:1nat,length:5nat,distance:10nat},t::Step {pos:131nat,next:140nat,before:17nat,after:18nat,symbol:224nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:140nat,next:148nat,before:18nat,after:19nat,symbol:4nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:148nat,next:157nat,before:19nat,after:20nat,symbol:216nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:157nat,next:165nat,before:20nat,after:21nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:165nat,next:173nat,before:21nat,after:22nat,symbol:24nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:173nat,next:180nat,before:22nat,after:22nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_interlace_ct0_d1_5x5()
    ensures t::trace(z_interlace_ct0_d1_5x5(),raw_interlace_ct0_d1_5x5(),ts_interlace_ct0_d1_5x5(),0,19nat),
{
    hide(z_interlace_ct0_d1_5x5);hide(raw_interlace_ct0_d1_5x5);hide(ts_interlace_ct0_d1_5x5);hide(t::valid_step);
    assert(t::trace(z_interlace_ct0_d1_5x5(),raw_interlace_ct0_d1_5x5(),ts_interlace_ct0_d1_5x5(),0,19nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_interlace_ct0_d1_5x5()
    ensures raw_interlace_ct0_d1_5x5().len()==22nat,
        z_interlace_ct0_d1_5x5().len()==27nat,
        z_interlace_ct0_d1_5x5()[0]==120u8,
        z_interlace_ct0_d1_5x5()[1]==156u8,
        zm::field_value(z_interlace_ct0_d1_5x5(),16,1)==1nat,
        zm::field_value(z_interlace_ct0_d1_5x5(),17,2)==1nat,
        zm::be_value(z_interlace_ct0_d1_5x5(),23nat,4)==1196557957nat,
        ts_interlace_ct0_d1_5x5().len()==19nat,
        ts_interlace_ct0_d1_5x5()[0].pos==19nat,
        ts_interlace_ct0_d1_5x5()[0].before==0nat,
        ts_interlace_ct0_d1_5x5().last().next==180nat,
{
    hide(z_interlace_ct0_d1_5x5);hide(raw_interlace_ct0_d1_5x5);hide(ts_interlace_ct0_d1_5x5);
    assert(raw_interlace_ct0_d1_5x5().len()==22nat) by (compute);
    assert(z_interlace_ct0_d1_5x5().len()==27nat) by (compute);
    assert(z_interlace_ct0_d1_5x5()[0]==120u8) by (compute);
    assert(z_interlace_ct0_d1_5x5()[1]==156u8) by (compute);
    assert(zm::field_value(z_interlace_ct0_d1_5x5(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_interlace_ct0_d1_5x5(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_interlace_ct0_d1_5x5(),23nat,4)==1196557957nat) by (compute);
    assert(ts_interlace_ct0_d1_5x5().len()==19nat) by (compute);
    assert(ts_interlace_ct0_d1_5x5()[0].pos==19nat) by (compute);
    assert(ts_interlace_ct0_d1_5x5()[0].before==0nat) by (compute);
    assert(ts_interlace_ct0_d1_5x5().last().next==180nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_interlace_ct0_d1_5x5()
    ensures zm::Adler(raw_interlace_ct0_d1_5x5())==1196557957nat,
{
    hide(raw_interlace_ct0_d1_5x5);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_interlace_ct0_d1_5x5().len()==22nat) by (compute);
    assert(c::sums(raw_interlace_ct0_d1_5x5(),0,22nat)==(1668nat,18258nat)) by (compute);
    c::sums_correct(raw_interlace_ct0_d1_5x5(),0,22nat);
    assert(raw_interlace_ct0_d1_5x5().subrange(0,22) =~= raw_interlace_ct0_d1_5x5());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((18258nat%65521)*(u16::MAX as nat+1)+(1+1668nat)%65521==1196557957nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_interlace_ct0_d1_5x5()
    ensures r::inflated(z_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_interlace_ct0_d1_5x5(),r2:false}),
        raw_interlace_ct0_d1_5x5().len()==22nat,
{
    hide(z_interlace_ct0_d1_5x5);hide(raw_interlace_ct0_d1_5x5);hide(ts_interlace_ct0_d1_5x5);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_interlace_ct0_d1_5x5();checksum_interlace_ct0_d1_5x5();
    assert(r::raw(a_interlace_ct0_d1_5x5())==22nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_interlace_ct0_d1_5x5();
    stream::fixed_stream(z_interlace_ct0_d1_5x5(),raw_interlace_ct0_d1_5x5(),ts_interlace_ct0_d1_5x5(),22nat,156nat,1196557957nat);
}
#[verifier::spinoff_prover]
proof fn scan_interlace_ct0_d1_5x5()
    ensures r::scan(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),0)==Ok(()),
{
    hide(raw_interlace_ct0_d1_5x5);
    assert(r::count(a_interlace_ct0_d1_5x5())==7nat) by (compute);
    r::scan_step(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),7);
    assert(r::pw(a_interlace_ct0_d1_5x5(),6nat)==5nat && r::ph(a_interlace_ct0_d1_5x5(),6nat)==2nat && r::rb(a_interlace_ct0_d1_5x5(),6nat)==1nat && r::base(a_interlace_ct0_d1_5x5(),6nat)==18nat) by (compute);
    assert(raw_interlace_ct0_d1_5x5()[18]==4u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),6nat,0nat)==Ok(()));
    assert(raw_interlace_ct0_d1_5x5()[20]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),6nat,1nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),6nat,2nat);
    r::pass_step(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),6nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),6nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),6nat);
    assert(r::pw(a_interlace_ct0_d1_5x5(),5nat)==2nat && r::ph(a_interlace_ct0_d1_5x5(),5nat)==3nat && r::rb(a_interlace_ct0_d1_5x5(),5nat)==1nat && r::base(a_interlace_ct0_d1_5x5(),5nat)==12nat) by (compute);
    assert(raw_interlace_ct0_d1_5x5()[12]==1u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),5nat,0nat)==Ok(()));
    assert(raw_interlace_ct0_d1_5x5()[14]==2u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),5nat,1nat)==Ok(()));
    assert(raw_interlace_ct0_d1_5x5()[16]==3u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),5nat,2nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),5nat,3nat);
    r::pass_step(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),5nat,2nat);
    r::pass_step(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),5nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),5nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),5nat);
    assert(r::pw(a_interlace_ct0_d1_5x5(),4nat)==3nat && r::ph(a_interlace_ct0_d1_5x5(),4nat)==1nat && r::rb(a_interlace_ct0_d1_5x5(),4nat)==1nat && r::base(a_interlace_ct0_d1_5x5(),4nat)==10nat) by (compute);
    assert(raw_interlace_ct0_d1_5x5()[10]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),4nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),4nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),4nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),4nat);
    assert(r::pw(a_interlace_ct0_d1_5x5(),3nat)==1nat && r::ph(a_interlace_ct0_d1_5x5(),3nat)==2nat && r::rb(a_interlace_ct0_d1_5x5(),3nat)==1nat && r::base(a_interlace_ct0_d1_5x5(),3nat)==6nat) by (compute);
    assert(raw_interlace_ct0_d1_5x5()[6]==3u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),3nat,0nat)==Ok(()));
    assert(raw_interlace_ct0_d1_5x5()[8]==4u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),3nat,1nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),3nat,2nat);
    r::pass_step(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),3nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),3nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),3nat);
    assert(r::pw(a_interlace_ct0_d1_5x5(),2nat)==2nat && r::ph(a_interlace_ct0_d1_5x5(),2nat)==1nat && r::rb(a_interlace_ct0_d1_5x5(),2nat)==1nat && r::base(a_interlace_ct0_d1_5x5(),2nat)==4nat) by (compute);
    assert(raw_interlace_ct0_d1_5x5()[4]==2u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),2nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),2nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),2nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),2nat);
    assert(r::pw(a_interlace_ct0_d1_5x5(),1nat)==1nat && r::ph(a_interlace_ct0_d1_5x5(),1nat)==1nat && r::rb(a_interlace_ct0_d1_5x5(),1nat)==1nat && r::base(a_interlace_ct0_d1_5x5(),1nat)==2nat) by (compute);
    assert(raw_interlace_ct0_d1_5x5()[2]==1u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),1nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),1nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),1nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),1nat);
    assert(r::pw(a_interlace_ct0_d1_5x5(),0nat)==1nat && r::ph(a_interlace_ct0_d1_5x5(),0nat)==1nat && r::rb(a_interlace_ct0_d1_5x5(),0nat)==1nat && r::base(a_interlace_ct0_d1_5x5(),0nat)==0nat) by (compute);
    assert(raw_interlace_ct0_d1_5x5()[0]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),0nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),0nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),0nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_interlace_ct0_d1_5x5()
    ensures r::still(bytes_interlace_ct0_d1_5x5(),STRICT,false)==(r::Projection::Ready {width:5u32,height:5u32,bytes:100nat}),
{
    hide(bytes_interlace_ct0_d1_5x5);hide(z_interlace_ct0_d1_5x5);hide(raw_interlace_ct0_d1_5x5);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_interlace_ct0_d1_5x5().len()==84nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct0_d1_5x5())==Ok((crate::model::Ihdr {width:5u32,height:5u32,depth:1u8,color_type:0u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct0_d1_5x5())==Ok((crate::model::Ihdr {width:5u32,height:5u32,depth:1u8,color_type:0u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 1u8)==2u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct0_d1_5x5(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:27nat,next:72nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct0_d1_5x5(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:27nat,next:72nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct0_d1_5x5(),72nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:80nat,len:0nat,next:84nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct0_d1_5x5(),72nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:80nat,len:0nat,next:84nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct0_d1_5x5().subrange(41,68)==z_interlace_ct0_d1_5x5()) by (compute);
    walk_interlace_ct0_d1_5x5(bytes_interlace_ct0_d1_5x5());
    inflate_interlace_ct0_d1_5x5();scan_interlace_ct0_d1_5x5();
    assert(r::raw(a_interlace_ct0_d1_5x5())==22nat) by (compute);
    assert(r::data(z_interlace_ct0_d1_5x5(),a_interlace_ct0_d1_5x5(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_interlace_ct0_d1_5x5(),h_interlace_ct0_d1_5x5(),z_interlace_ct0_d1_5x5(),0nat,false);
}
fn public_interlace_ct0_d1_5x5() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,5u8,0u8,0u8,0u8,5u8,1u8,0u8,0u8,0u8,1u8,210u8,19u8,43u8,222u8,0u8,0u8,0u8,27u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,104u8,96u8,108u8,96u8,58u8,192u8,220u8,192u8,210u8,192u8,176u8,0u8,204u8,122u8,192u8,114u8,131u8,65u8,2u8,0u8,71u8,82u8,6u8,133u8,90u8,167u8,112u8,214u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_interlace_ct0_d1_5x5());model_interlace_ct0_d1_5x5();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==5u32 && s.height==5u32 && s.rgba@.len()==100nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// interlace/ct6-d16-5x5; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_interlace_ct6_d16_5x5() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,5u8,0u8,0u8,0u8,5u8,16u8,6u8,0u8,0u8,1u8,170u8,248u8,202u8,48u8,0u8,0u8,0u8,222u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,211u8,0u8,44u8,255u8,0u8,116u8,245u8,191u8,242u8,137u8,77u8,242u8,116u8,1u8,20u8,101u8,201u8,254u8,62u8,177u8,123u8,87u8,2u8,62u8,113u8,22u8,183u8,4u8,38u8,172u8,180u8,237u8,233u8,100u8,37u8,226u8,167u8,17u8,49u8,3u8,246u8,157u8,132u8,68u8,63u8,213u8,166u8,222u8,4u8,174u8,211u8,214u8,34u8,110u8,53u8,78u8,212u8,0u8,112u8,2u8,100u8,30u8,68u8,103u8,23u8,244u8,213u8,222u8,169u8,96u8,243u8,187u8,83u8,105u8,49u8,91u8,139u8,115u8,20u8,55u8,191u8,115u8,1u8,135u8,93u8,142u8,243u8,100u8,198u8,176u8,54u8,224u8,198u8,179u8,90u8,109u8,81u8,214u8,188u8,2u8,196u8,56u8,63u8,96u8,255u8,79u8,210u8,47u8,180u8,210u8,212u8,31u8,209u8,122u8,225u8,98u8,3u8,241u8,54u8,24u8,174u8,173u8,90u8,119u8,72u8,46u8,112u8,182u8,46u8,77u8,162u8,45u8,153u8,4u8,114u8,84u8,98u8,137u8,187u8,7u8,154u8,120u8,187u8,238u8,237u8,39u8,25u8,8u8,37u8,233u8,66u8,251u8,95u8,77u8,38u8,45u8,12u8,81u8,87u8,206u8,124u8,54u8,84u8,11u8,21u8,185u8,230u8,67u8,118u8,24u8,34u8,132u8,78u8,51u8,0u8,56u8,23u8,123u8,131u8,36u8,66u8,236u8,132u8,163u8,97u8,51u8,97u8,48u8,103u8,110u8,77u8,170u8,182u8,73u8,182u8,248u8,176u8,222u8,135u8,253u8,201u8,220u8,116u8,101u8,227u8,40u8,202u8,32u8,195u8,79u8,182u8,106u8,139u8,158u8,41u8,7u8,219u8,98u8,80u8,130u8,233u8,199u8,110u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_interlace_ct6_d16_5x5() -> Seq<u8> {seq![120u8,156u8,1u8,211u8,0u8,44u8,255u8,0u8,116u8,245u8,191u8,242u8,137u8,77u8,242u8,116u8,1u8,20u8,101u8,201u8,254u8,62u8,177u8,123u8,87u8,2u8,62u8,113u8,22u8,183u8,4u8,38u8,172u8,180u8,237u8,233u8,100u8,37u8,226u8,167u8,17u8,49u8,3u8,246u8,157u8,132u8,68u8,63u8,213u8,166u8,222u8,4u8,174u8,211u8,214u8,34u8,110u8,53u8,78u8,212u8,0u8,112u8,2u8,100u8,30u8,68u8,103u8,23u8,244u8,213u8,222u8,169u8,96u8,243u8,187u8,83u8,105u8,49u8,91u8,139u8,115u8,20u8,55u8,191u8,115u8,1u8,135u8,93u8,142u8,243u8,100u8,198u8,176u8,54u8,224u8,198u8,179u8,90u8,109u8,81u8,214u8,188u8,2u8,196u8,56u8,63u8,96u8,255u8,79u8,210u8,47u8,180u8,210u8,212u8,31u8,209u8,122u8,225u8,98u8,3u8,241u8,54u8,24u8,174u8,173u8,90u8,119u8,72u8,46u8,112u8,182u8,46u8,77u8,162u8,45u8,153u8,4u8,114u8,84u8,98u8,137u8,187u8,7u8,154u8,120u8,187u8,238u8,237u8,39u8,25u8,8u8,37u8,233u8,66u8,251u8,95u8,77u8,38u8,45u8,12u8,81u8,87u8,206u8,124u8,54u8,84u8,11u8,21u8,185u8,230u8,67u8,118u8,24u8,34u8,132u8,78u8,51u8,0u8,56u8,23u8,123u8,131u8,36u8,66u8,236u8,132u8,163u8,97u8,51u8,97u8,48u8,103u8,110u8,77u8,170u8,182u8,73u8,182u8,248u8,176u8,222u8,135u8,253u8,201u8,220u8,116u8,101u8,227u8,40u8,202u8,32u8,195u8,79u8,182u8,106u8,139u8,158u8,41u8,7u8,219u8,98u8,80u8]}
spec fn h_interlace_ct6_d16_5x5() -> crate::model::Ihdr {crate::model::Ihdr {width:5u32,height:5u32,depth:16u8,color_type:6u8,interlace:1u8}}
spec fn a_interlace_ct6_d16_5x5() -> r::Raster {r::shape(h_interlace_ct6_d16_5x5(),5nat,5nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_interlace_ct6_d16_5x5(b: Seq<u8>)
    requires b.len()==279,crate::model::mh(b)==Ok((crate::model::Ihdr {width:5u32,height:5u32,depth:16u8,color_type:6u8,interlace:1u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:222nat,next:267nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,267nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:275nat,len:0nat,next:279nat,crc_ok:true,synthetic:false})),
        b.subrange(41,263)==z_interlace_ct6_d16_5x5(),
    ensures r::default_case(b,crate::model::Ihdr {width:5u32,height:5u32,depth:16u8,color_type:6u8,interlace:1u8},z_interlace_ct6_d16_5x5(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    hide(z_interlace_ct6_d16_5x5);
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_interlace_ct6_d16_5x5() -> Seq<u8> {seq![0u8,116u8,245u8,191u8,242u8,137u8,77u8,242u8,116u8,1u8,20u8,101u8,201u8,254u8,62u8,177u8,123u8,87u8,2u8,62u8,113u8,22u8,183u8,4u8,38u8,172u8,180u8,237u8,233u8,100u8,37u8,226u8,167u8,17u8,49u8,3u8,246u8,157u8,132u8,68u8,63u8,213u8,166u8,222u8,4u8,174u8,211u8,214u8,34u8,110u8,53u8,78u8,212u8,0u8,112u8,2u8,100u8,30u8,68u8,103u8,23u8,244u8,213u8,222u8,169u8,96u8,243u8,187u8,83u8,105u8,49u8,91u8,139u8,115u8,20u8,55u8,191u8,115u8,1u8,135u8,93u8,142u8,243u8,100u8,198u8,176u8,54u8,224u8,198u8,179u8,90u8,109u8,81u8,214u8,188u8,2u8,196u8,56u8,63u8,96u8,255u8,79u8,210u8,47u8,180u8,210u8,212u8,31u8,209u8,122u8,225u8,98u8,3u8,241u8,54u8,24u8,174u8,173u8,90u8,119u8,72u8,46u8,112u8,182u8,46u8,77u8,162u8,45u8,153u8,4u8,114u8,84u8,98u8,137u8,187u8,7u8,154u8,120u8,187u8,238u8,237u8,39u8,25u8,8u8,37u8,233u8,66u8,251u8,95u8,77u8,38u8,45u8,12u8,81u8,87u8,206u8,124u8,54u8,84u8,11u8,21u8,185u8,230u8,67u8,118u8,24u8,34u8,132u8,78u8,51u8,0u8,56u8,23u8,123u8,131u8,36u8,66u8,236u8,132u8,163u8,97u8,51u8,97u8,48u8,103u8,110u8,77u8,170u8,182u8,73u8,182u8,248u8,176u8,222u8,135u8,253u8,201u8,220u8,116u8,101u8,227u8,40u8,202u8,32u8,195u8,79u8,182u8,106u8,139u8,158u8,41u8]}
#[verifier::spinoff_prover]
proof fn shape_interlace_ct6_d16_5x5()
    ensures raw_interlace_ct6_d16_5x5().len()==211nat,
        z_interlace_ct6_d16_5x5().len()==222nat,
        z_interlace_ct6_d16_5x5()[0]==120u8,
        z_interlace_ct6_d16_5x5()[1]==156u8,
        zm::field_value(z_interlace_ct6_d16_5x5(),16,1)==1nat,
        zm::field_value(z_interlace_ct6_d16_5x5(),17,2)==0nat,
        zm::be_value(z_interlace_ct6_d16_5x5(),218nat,4)==131818064nat,
        z_interlace_ct6_d16_5x5().subrange(7,218)==raw_interlace_ct6_d16_5x5(),
        z_interlace_ct6_d16_5x5()[3] as nat+256*z_interlace_ct6_d16_5x5()[4] as nat==211nat,
        z_interlace_ct6_d16_5x5()[5] as nat+256*z_interlace_ct6_d16_5x5()[6] as nat==u16::MAX as nat-211nat,
        zm::be_value(z_interlace_ct6_d16_5x5(),0,1)==120nat,
        zm::be_value(z_interlace_ct6_d16_5x5(),1,1)==156nat,
{
    hide(z_interlace_ct6_d16_5x5);hide(raw_interlace_ct6_d16_5x5);
    assert(raw_interlace_ct6_d16_5x5().len()==211nat) by (compute);
    assert(z_interlace_ct6_d16_5x5().len()==222nat) by (compute);
    assert(z_interlace_ct6_d16_5x5()[0]==120u8) by (compute);
    assert(z_interlace_ct6_d16_5x5()[1]==156u8) by (compute);
    assert(zm::field_value(z_interlace_ct6_d16_5x5(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_interlace_ct6_d16_5x5(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_interlace_ct6_d16_5x5(),218nat,4)==131818064nat) by (compute);
    assert(z_interlace_ct6_d16_5x5().subrange(7,218)==raw_interlace_ct6_d16_5x5()) by (compute);
    assert(z_interlace_ct6_d16_5x5()[3] as nat+256*z_interlace_ct6_d16_5x5()[4] as nat==211nat) by (compute);
    assert(z_interlace_ct6_d16_5x5()[5] as nat+256*z_interlace_ct6_d16_5x5()[6] as nat==u16::MAX as nat-211nat) by (compute);
    assert(zm::be_value(z_interlace_ct6_d16_5x5(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_interlace_ct6_d16_5x5(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_interlace_ct6_d16_5x5()
    ensures zm::Adler(raw_interlace_ct6_d16_5x5())==131818064nat,
{
    hide(raw_interlace_ct6_d16_5x5);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_interlace_ct6_d16_5x5().len()==211nat) by (compute);
    assert(c::sums(raw_interlace_ct6_d16_5x5(),0,211nat)==(25167nat,2688372nat)) by (compute);
    c::sums_correct(raw_interlace_ct6_d16_5x5(),0,211nat);
    assert(raw_interlace_ct6_d16_5x5().subrange(0,211) =~= raw_interlace_ct6_d16_5x5());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((2688372nat%65521)*(u16::MAX as nat+1)+(1+25167nat)%65521==131818064nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_interlace_ct6_d16_5x5()
    ensures r::inflated(z_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_interlace_ct6_d16_5x5(),r2:false}),
        raw_interlace_ct6_d16_5x5().len()==211nat,
{
    hide(z_interlace_ct6_d16_5x5);hide(raw_interlace_ct6_d16_5x5);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_interlace_ct6_d16_5x5();checksum_interlace_ct6_d16_5x5();
    assert(r::raw(a_interlace_ct6_d16_5x5())==211nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_interlace_ct6_d16_5x5(),raw_interlace_ct6_d16_5x5(),211nat,120nat,156nat,131818064nat);
}
#[verifier::spinoff_prover]
proof fn scan_interlace_ct6_d16_5x5()
    ensures r::scan(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),0)==Ok(()),
{
    hide(raw_interlace_ct6_d16_5x5);
    assert(r::count(a_interlace_ct6_d16_5x5())==7nat) by (compute);
    r::scan_step(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),7);
    assert(r::pw(a_interlace_ct6_d16_5x5(),6nat)==5nat && r::ph(a_interlace_ct6_d16_5x5(),6nat)==2nat && r::rb(a_interlace_ct6_d16_5x5(),6nat)==40nat && r::base(a_interlace_ct6_d16_5x5(),6nat)==129nat) by (compute);
    assert(raw_interlace_ct6_d16_5x5()[129]==4u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),6nat,0nat)==Ok(()));
    assert(raw_interlace_ct6_d16_5x5()[170]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),6nat,1nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),6nat,2nat);
    r::pass_step(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),6nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),6nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),6nat);
    assert(r::pw(a_interlace_ct6_d16_5x5(),5nat)==2nat && r::ph(a_interlace_ct6_d16_5x5(),5nat)==3nat && r::rb(a_interlace_ct6_d16_5x5(),5nat)==16nat && r::base(a_interlace_ct6_d16_5x5(),5nat)==78nat) by (compute);
    assert(raw_interlace_ct6_d16_5x5()[78]==1u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),5nat,0nat)==Ok(()));
    assert(raw_interlace_ct6_d16_5x5()[95]==2u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),5nat,1nat)==Ok(()));
    assert(raw_interlace_ct6_d16_5x5()[112]==3u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),5nat,2nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),5nat,3nat);
    r::pass_step(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),5nat,2nat);
    r::pass_step(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),5nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),5nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),5nat);
    assert(r::pw(a_interlace_ct6_d16_5x5(),4nat)==3nat && r::ph(a_interlace_ct6_d16_5x5(),4nat)==1nat && r::rb(a_interlace_ct6_d16_5x5(),4nat)==24nat && r::base(a_interlace_ct6_d16_5x5(),4nat)==53nat) by (compute);
    assert(raw_interlace_ct6_d16_5x5()[53]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),4nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),4nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),4nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),4nat);
    assert(r::pw(a_interlace_ct6_d16_5x5(),3nat)==1nat && r::ph(a_interlace_ct6_d16_5x5(),3nat)==2nat && r::rb(a_interlace_ct6_d16_5x5(),3nat)==8nat && r::base(a_interlace_ct6_d16_5x5(),3nat)==35nat) by (compute);
    assert(raw_interlace_ct6_d16_5x5()[35]==3u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),3nat,0nat)==Ok(()));
    assert(raw_interlace_ct6_d16_5x5()[44]==4u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),3nat,1nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),3nat,2nat);
    r::pass_step(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),3nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),3nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),3nat);
    assert(r::pw(a_interlace_ct6_d16_5x5(),2nat)==2nat && r::ph(a_interlace_ct6_d16_5x5(),2nat)==1nat && r::rb(a_interlace_ct6_d16_5x5(),2nat)==16nat && r::base(a_interlace_ct6_d16_5x5(),2nat)==18nat) by (compute);
    assert(raw_interlace_ct6_d16_5x5()[18]==2u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),2nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),2nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),2nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),2nat);
    assert(r::pw(a_interlace_ct6_d16_5x5(),1nat)==1nat && r::ph(a_interlace_ct6_d16_5x5(),1nat)==1nat && r::rb(a_interlace_ct6_d16_5x5(),1nat)==8nat && r::base(a_interlace_ct6_d16_5x5(),1nat)==9nat) by (compute);
    assert(raw_interlace_ct6_d16_5x5()[9]==1u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),1nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),1nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),1nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),1nat);
    assert(r::pw(a_interlace_ct6_d16_5x5(),0nat)==1nat && r::ph(a_interlace_ct6_d16_5x5(),0nat)==1nat && r::rb(a_interlace_ct6_d16_5x5(),0nat)==8nat && r::base(a_interlace_ct6_d16_5x5(),0nat)==0nat) by (compute);
    assert(raw_interlace_ct6_d16_5x5()[0]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),0nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),0nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),0nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_interlace_ct6_d16_5x5()
    ensures r::still(bytes_interlace_ct6_d16_5x5(),STRICT,false)==(r::Projection::Ready {width:5u32,height:5u32,bytes:100nat}),
{
    hide(bytes_interlace_ct6_d16_5x5);hide(z_interlace_ct6_d16_5x5);hide(raw_interlace_ct6_d16_5x5);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_interlace_ct6_d16_5x5().len()==279nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct6_d16_5x5())==Ok((crate::model::Ihdr {width:5u32,height:5u32,depth:16u8,color_type:6u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct6_d16_5x5())==Ok((crate::model::Ihdr {width:5u32,height:5u32,depth:16u8,color_type:6u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct6_d16_5x5(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:222nat,next:267nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct6_d16_5x5(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:222nat,next:267nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct6_d16_5x5(),267nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:275nat,len:0nat,next:279nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct6_d16_5x5(),267nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:275nat,len:0nat,next:279nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct6_d16_5x5().subrange(41,263)==z_interlace_ct6_d16_5x5()) by (compute);
    walk_interlace_ct6_d16_5x5(bytes_interlace_ct6_d16_5x5());
    inflate_interlace_ct6_d16_5x5();scan_interlace_ct6_d16_5x5();
    assert(r::raw(a_interlace_ct6_d16_5x5())==211nat) by (compute);
    assert(r::data(z_interlace_ct6_d16_5x5(),a_interlace_ct6_d16_5x5(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_interlace_ct6_d16_5x5(),h_interlace_ct6_d16_5x5(),z_interlace_ct6_d16_5x5(),0nat,false);
}
fn public_interlace_ct6_d16_5x5() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,5u8,0u8,0u8,0u8,5u8,16u8,6u8,0u8,0u8,1u8,170u8,248u8,202u8,48u8,0u8,0u8,0u8,222u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,211u8,0u8,44u8,255u8,0u8,116u8,245u8,191u8,242u8,137u8,77u8,242u8,116u8,1u8,20u8,101u8,201u8,254u8,62u8,177u8,123u8,87u8,2u8,62u8,113u8,22u8,183u8,4u8,38u8,172u8,180u8,237u8,233u8,100u8,37u8,226u8,167u8,17u8,49u8,3u8,246u8,157u8,132u8,68u8,63u8,213u8,166u8,222u8,4u8,174u8,211u8,214u8,34u8,110u8,53u8,78u8,212u8,0u8,112u8,2u8,100u8,30u8,68u8,103u8,23u8,244u8,213u8,222u8,169u8,96u8,243u8,187u8,83u8,105u8,49u8,91u8,139u8,115u8,20u8,55u8,191u8,115u8,1u8,135u8,93u8,142u8,243u8,100u8,198u8,176u8,54u8,224u8,198u8,179u8,90u8,109u8,81u8,214u8,188u8,2u8,196u8,56u8,63u8,96u8,255u8,79u8,210u8,47u8,180u8,210u8,212u8,31u8,209u8,122u8,225u8,98u8,3u8,241u8,54u8,24u8,174u8,173u8,90u8,119u8,72u8,46u8,112u8,182u8,46u8,77u8,162u8,45u8,153u8,4u8,114u8,84u8,98u8,137u8,187u8,7u8,154u8,120u8,187u8,238u8,237u8,39u8,25u8,8u8,37u8,233u8,66u8,251u8,95u8,77u8,38u8,45u8,12u8,81u8,87u8,206u8,124u8,54u8,84u8,11u8,21u8,185u8,230u8,67u8,118u8,24u8,34u8,132u8,78u8,51u8,0u8,56u8,23u8,123u8,131u8,36u8,66u8,236u8,132u8,163u8,97u8,51u8,97u8,48u8,103u8,110u8,77u8,170u8,182u8,73u8,182u8,248u8,176u8,222u8,135u8,253u8,201u8,220u8,116u8,101u8,227u8,40u8,202u8,32u8,195u8,79u8,182u8,106u8,139u8,158u8,41u8,7u8,219u8,98u8,80u8,130u8,233u8,199u8,110u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_interlace_ct6_d16_5x5());model_interlace_ct6_d16_5x5();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==5u32 && s.height==5u32 && s.rgba@.len()==100nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// interlace/ct3-d4-5x5; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_interlace_ct3_d4_5x5() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,5u8,0u8,0u8,0u8,5u8,4u8,3u8,0u8,0u8,1u8,8u8,70u8,11u8,64u8,0u8,0u8,0u8,48u8,80u8,76u8,84u8,69u8,36u8,232u8,239u8,139u8,215u8,250u8,206u8,180u8,111u8,198u8,225u8,23u8,232u8,97u8,47u8,183u8,158u8,121u8,241u8,229u8,45u8,213u8,146u8,231u8,43u8,40u8,106u8,227u8,168u8,135u8,172u8,74u8,202u8,154u8,237u8,172u8,109u8,123u8,83u8,124u8,229u8,54u8,33u8,54u8,151u8,173u8,202u8,100u8,139u8,60u8,110u8,245u8,0u8,0u8,0u8,36u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,216u8,192u8,88u8,192u8,164u8,195u8,172u8,192u8,114u8,128u8,33u8,40u8,129u8,81u8,149u8,233u8,44u8,243u8,15u8,22u8,225u8,91u8,135u8,25u8,154u8,12u8,31u8,0u8,0u8,91u8,217u8,8u8,32u8,80u8,131u8,97u8,98u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_interlace_ct3_d4_5x5() -> Seq<u8> {seq![120u8,156u8,99u8,216u8,192u8,88u8,192u8,164u8,195u8,172u8,192u8,114u8,128u8,33u8,40u8,129u8,81u8,149u8,233u8,44u8,243u8,15u8,22u8,225u8,91u8,135u8,25u8,154u8,12u8,31u8,0u8,0u8,91u8,217u8,8u8,32u8]}
spec fn h_interlace_ct3_d4_5x5() -> crate::model::Ihdr {crate::model::Ihdr {width:5u32,height:5u32,depth:4u8,color_type:3u8,interlace:1u8}}
spec fn a_interlace_ct3_d4_5x5() -> r::Raster {r::shape(h_interlace_ct3_d4_5x5(),5nat,5nat,16nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_interlace_ct3_d4_5x5(b: Seq<u8>)
    requires b.len()==153,crate::model::mh(b)==Ok((crate::model::Ihdr {width:5u32,height:5u32,depth:4u8,color_type:3u8,interlace:1u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:36nat,next:141nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,141nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:149nat,len:0nat,next:153nat,crc_ok:true,synthetic:false})),
        b.subrange(101,137)==z_interlace_ct3_d4_5x5(),
    ensures r::default_case(b,crate::model::Ihdr {width:5u32,height:5u32,depth:4u8,color_type:3u8,interlace:1u8},z_interlace_ct3_d4_5x5(),16nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);
    hide(z_interlace_ct3_d4_5x5);
    r::palette_default_case(b,crate::model::Ihdr {width:5u32,height:5u32,depth:4u8,color_type:3u8,interlace:1u8},z_interlace_ct3_d4_5x5(),36nat);
}
spec fn raw_interlace_ct3_d4_5x5() -> Seq<u8> {seq![0u8,176u8,1u8,112u8,2u8,44u8,3u8,32u8,4u8,192u8,0u8,82u8,96u8,1u8,37u8,2u8,205u8,3u8,248u8,4u8,19u8,218u8,195u8,0u8,130u8,49u8,224u8]}
spec fn ts_interlace_ct3_d4_5x5() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:36nat,before:1nat,after:2nat,symbol:176nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:36nat,next:44nat,before:2nat,after:3nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:44nat,next:52nat,before:3nat,after:4nat,symbol:112nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:52nat,next:60nat,before:4nat,after:5nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:60nat,next:68nat,before:5nat,after:6nat,symbol:44nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:68nat,next:76nat,before:6nat,after:7nat,symbol:3nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:76nat,next:84nat,before:7nat,after:8nat,symbol:32nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:84nat,next:92nat,before:8nat,after:9nat,symbol:4nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:92nat,next:101nat,before:9nat,after:10nat,symbol:192nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:101nat,next:109nat,before:10nat,after:11nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:109nat,next:117nat,before:11nat,after:12nat,symbol:82nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:117nat,next:125nat,before:12nat,after:13nat,symbol:96nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:125nat,next:133nat,before:13nat,after:14nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:133nat,next:141nat,before:14nat,after:15nat,symbol:37nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:141nat,next:149nat,before:15nat,after:16nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:149nat,next:158nat,before:16nat,after:17nat,symbol:205nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:158nat,next:166nat,before:17nat,after:18nat,symbol:3nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:166nat,next:175nat,before:18nat,after:19nat,symbol:248nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:175nat,next:183nat,before:19nat,after:20nat,symbol:4nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:183nat,next:191nat,before:20nat,after:21nat,symbol:19nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:191nat,next:200nat,before:21nat,after:22nat,symbol:218nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:200nat,next:209nat,before:22nat,after:23nat,symbol:195nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:209nat,next:217nat,before:23nat,after:24nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:217nat,next:225nat,before:24nat,after:25nat,symbol:130nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:225nat,next:233nat,before:25nat,after:26nat,symbol:49nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:233nat,next:242nat,before:26nat,after:27nat,symbol:224nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:242nat,next:249nat,before:27nat,after:27nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_interlace_ct3_d4_5x5()
    ensures t::trace(z_interlace_ct3_d4_5x5(),raw_interlace_ct3_d4_5x5(),ts_interlace_ct3_d4_5x5(),0,28nat),
{
    hide(z_interlace_ct3_d4_5x5);hide(raw_interlace_ct3_d4_5x5);hide(ts_interlace_ct3_d4_5x5);hide(t::valid_step);
    assert(t::trace(z_interlace_ct3_d4_5x5(),raw_interlace_ct3_d4_5x5(),ts_interlace_ct3_d4_5x5(),0,28nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_interlace_ct3_d4_5x5()
    ensures raw_interlace_ct3_d4_5x5().len()==27nat,
        z_interlace_ct3_d4_5x5().len()==36nat,
        z_interlace_ct3_d4_5x5()[0]==120u8,
        z_interlace_ct3_d4_5x5()[1]==156u8,
        zm::field_value(z_interlace_ct3_d4_5x5(),16,1)==1nat,
        zm::field_value(z_interlace_ct3_d4_5x5(),17,2)==1nat,
        zm::be_value(z_interlace_ct3_d4_5x5(),32nat,4)==1540950048nat,
        ts_interlace_ct3_d4_5x5().len()==28nat,
        ts_interlace_ct3_d4_5x5()[0].pos==19nat,
        ts_interlace_ct3_d4_5x5()[0].before==0nat,
        ts_interlace_ct3_d4_5x5().last().next==249nat,
{
    hide(z_interlace_ct3_d4_5x5);hide(raw_interlace_ct3_d4_5x5);hide(ts_interlace_ct3_d4_5x5);
    assert(raw_interlace_ct3_d4_5x5().len()==27nat) by (compute);
    assert(z_interlace_ct3_d4_5x5().len()==36nat) by (compute);
    assert(z_interlace_ct3_d4_5x5()[0]==120u8) by (compute);
    assert(z_interlace_ct3_d4_5x5()[1]==156u8) by (compute);
    assert(zm::field_value(z_interlace_ct3_d4_5x5(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_interlace_ct3_d4_5x5(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_interlace_ct3_d4_5x5(),32nat,4)==1540950048nat) by (compute);
    assert(ts_interlace_ct3_d4_5x5().len()==28nat) by (compute);
    assert(ts_interlace_ct3_d4_5x5()[0].pos==19nat) by (compute);
    assert(ts_interlace_ct3_d4_5x5()[0].before==0nat) by (compute);
    assert(ts_interlace_ct3_d4_5x5().last().next==249nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_interlace_ct3_d4_5x5()
    ensures zm::Adler(raw_interlace_ct3_d4_5x5())==1540950048nat,
{
    hide(raw_interlace_ct3_d4_5x5);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_interlace_ct3_d4_5x5().len()==27nat) by (compute);
    assert(c::sums(raw_interlace_ct3_d4_5x5(),0,27nat)==(2079nat,23513nat)) by (compute);
    c::sums_correct(raw_interlace_ct3_d4_5x5(),0,27nat);
    assert(raw_interlace_ct3_d4_5x5().subrange(0,27) =~= raw_interlace_ct3_d4_5x5());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((23513nat%65521)*(u16::MAX as nat+1)+(1+2079nat)%65521==1540950048nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_interlace_ct3_d4_5x5()
    ensures r::inflated(z_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_interlace_ct3_d4_5x5(),r2:false}),
        raw_interlace_ct3_d4_5x5().len()==27nat,
{
    hide(z_interlace_ct3_d4_5x5);hide(raw_interlace_ct3_d4_5x5);hide(ts_interlace_ct3_d4_5x5);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_interlace_ct3_d4_5x5();checksum_interlace_ct3_d4_5x5();
    assert(r::raw(a_interlace_ct3_d4_5x5())==27nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_interlace_ct3_d4_5x5();
    stream::fixed_stream(z_interlace_ct3_d4_5x5(),raw_interlace_ct3_d4_5x5(),ts_interlace_ct3_d4_5x5(),27nat,156nat,1540950048nat);
}
#[verifier::spinoff_prover]
proof fn scan_interlace_ct3_d4_5x5()
    ensures r::scan(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),0)==Ok(()),
{
    hide(raw_interlace_ct3_d4_5x5);
    assert(r::count(a_interlace_ct3_d4_5x5())==7nat) by (compute);
    r::scan_step(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),7);
    assert(r::pw(a_interlace_ct3_d4_5x5(),6nat)==5nat && r::ph(a_interlace_ct3_d4_5x5(),6nat)==2nat && r::rb(a_interlace_ct3_d4_5x5(),6nat)==3nat && r::base(a_interlace_ct3_d4_5x5(),6nat)==19nat) by (compute);
    assert(raw_interlace_ct3_d4_5x5()[19]==4u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),6nat,0nat),5nat);
    assert(r::row_result(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),6nat,0nat)==Ok(()));
    assert(raw_interlace_ct3_d4_5x5()[23]==0u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),6nat,1nat),5nat);
    assert(r::row_result(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),6nat,1nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),6nat,2nat);
    r::pass_step(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),6nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),6nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),6nat);
    assert(r::pw(a_interlace_ct3_d4_5x5(),5nat)==2nat && r::ph(a_interlace_ct3_d4_5x5(),5nat)==3nat && r::rb(a_interlace_ct3_d4_5x5(),5nat)==1nat && r::base(a_interlace_ct3_d4_5x5(),5nat)==13nat) by (compute);
    assert(raw_interlace_ct3_d4_5x5()[13]==1u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),5nat,0nat),2nat);
    assert(r::row_result(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),5nat,0nat)==Ok(()));
    assert(raw_interlace_ct3_d4_5x5()[15]==2u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),5nat,1nat),2nat);
    assert(r::row_result(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),5nat,1nat)==Ok(()));
    assert(raw_interlace_ct3_d4_5x5()[17]==3u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),5nat,2nat),2nat);
    assert(r::row_result(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),5nat,2nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),5nat,3nat);
    r::pass_step(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),5nat,2nat);
    r::pass_step(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),5nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),5nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),5nat);
    assert(r::pw(a_interlace_ct3_d4_5x5(),4nat)==3nat && r::ph(a_interlace_ct3_d4_5x5(),4nat)==1nat && r::rb(a_interlace_ct3_d4_5x5(),4nat)==2nat && r::base(a_interlace_ct3_d4_5x5(),4nat)==10nat) by (compute);
    assert(raw_interlace_ct3_d4_5x5()[10]==0u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),4nat,0nat),3nat);
    assert(r::row_result(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),4nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),4nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),4nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),4nat);
    assert(r::pw(a_interlace_ct3_d4_5x5(),3nat)==1nat && r::ph(a_interlace_ct3_d4_5x5(),3nat)==2nat && r::rb(a_interlace_ct3_d4_5x5(),3nat)==1nat && r::base(a_interlace_ct3_d4_5x5(),3nat)==6nat) by (compute);
    assert(raw_interlace_ct3_d4_5x5()[6]==3u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),3nat,0nat),1nat);
    assert(r::row_result(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),3nat,0nat)==Ok(()));
    assert(raw_interlace_ct3_d4_5x5()[8]==4u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),3nat,1nat),1nat);
    assert(r::row_result(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),3nat,1nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),3nat,2nat);
    r::pass_step(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),3nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),3nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),3nat);
    assert(r::pw(a_interlace_ct3_d4_5x5(),2nat)==2nat && r::ph(a_interlace_ct3_d4_5x5(),2nat)==1nat && r::rb(a_interlace_ct3_d4_5x5(),2nat)==1nat && r::base(a_interlace_ct3_d4_5x5(),2nat)==4nat) by (compute);
    assert(raw_interlace_ct3_d4_5x5()[4]==2u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),2nat,0nat),2nat);
    assert(r::row_result(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),2nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),2nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),2nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),2nat);
    assert(r::pw(a_interlace_ct3_d4_5x5(),1nat)==1nat && r::ph(a_interlace_ct3_d4_5x5(),1nat)==1nat && r::rb(a_interlace_ct3_d4_5x5(),1nat)==1nat && r::base(a_interlace_ct3_d4_5x5(),1nat)==2nat) by (compute);
    assert(raw_interlace_ct3_d4_5x5()[2]==1u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),1nat,0nat),1nat);
    assert(r::row_result(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),1nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),1nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),1nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),1nat);
    assert(r::pw(a_interlace_ct3_d4_5x5(),0nat)==1nat && r::ph(a_interlace_ct3_d4_5x5(),0nat)==1nat && r::rb(a_interlace_ct3_d4_5x5(),0nat)==1nat && r::base(a_interlace_ct3_d4_5x5(),0nat)==0nat) by (compute);
    assert(raw_interlace_ct3_d4_5x5()[0]==0u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),0nat,0nat),1nat);
    assert(r::row_result(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),0nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),0nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),0nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_interlace_ct3_d4_5x5()
    ensures r::still(bytes_interlace_ct3_d4_5x5(),STRICT,false)==(r::Projection::Ready {width:5u32,height:5u32,bytes:100nat}),
{
    hide(bytes_interlace_ct3_d4_5x5);hide(z_interlace_ct3_d4_5x5);hide(raw_interlace_ct3_d4_5x5);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);
    assert(bytes_interlace_ct3_d4_5x5().len()==153nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct3_d4_5x5())==Ok((crate::model::Ihdr {width:5u32,height:5u32,depth:4u8,color_type:3u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct3_d4_5x5())==Ok((crate::model::Ihdr {width:5u32,height:5u32,depth:4u8,color_type:3u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 4u8)==16u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_5x5(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_5x5(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(48nat,3u8,4u8,false,false,false,false)==crate::metadata_model::Decision::Keep(16nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_5x5(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:36nat,next:141nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_5x5(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:36nat,next:141nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_5x5(),141nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:149nat,len:0nat,next:153nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_5x5(),141nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:149nat,len:0nat,next:153nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct3_d4_5x5().subrange(101,137)==z_interlace_ct3_d4_5x5()) by (compute);
    walk_interlace_ct3_d4_5x5(bytes_interlace_ct3_d4_5x5());
    inflate_interlace_ct3_d4_5x5();scan_interlace_ct3_d4_5x5();
    assert(r::raw(a_interlace_ct3_d4_5x5())==27nat) by (compute);
    assert(r::data(z_interlace_ct3_d4_5x5(),a_interlace_ct3_d4_5x5(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_interlace_ct3_d4_5x5(),h_interlace_ct3_d4_5x5(),z_interlace_ct3_d4_5x5(),16nat,false);
}
fn public_interlace_ct3_d4_5x5() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,5u8,0u8,0u8,0u8,5u8,4u8,3u8,0u8,0u8,1u8,8u8,70u8,11u8,64u8,0u8,0u8,0u8,48u8,80u8,76u8,84u8,69u8,36u8,232u8,239u8,139u8,215u8,250u8,206u8,180u8,111u8,198u8,225u8,23u8,232u8,97u8,47u8,183u8,158u8,121u8,241u8,229u8,45u8,213u8,146u8,231u8,43u8,40u8,106u8,227u8,168u8,135u8,172u8,74u8,202u8,154u8,237u8,172u8,109u8,123u8,83u8,124u8,229u8,54u8,33u8,54u8,151u8,173u8,202u8,100u8,139u8,60u8,110u8,245u8,0u8,0u8,0u8,36u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,216u8,192u8,88u8,192u8,164u8,195u8,172u8,192u8,114u8,128u8,33u8,40u8,129u8,81u8,149u8,233u8,44u8,243u8,15u8,22u8,225u8,91u8,135u8,25u8,154u8,12u8,31u8,0u8,0u8,91u8,217u8,8u8,32u8,80u8,131u8,97u8,98u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_interlace_ct3_d4_5x5());model_interlace_ct3_d4_5x5();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==5u32 && s.height==5u32 && s.rgba@.len()==100nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// interlace/ct0-d1-8x8; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_interlace_ct0_d1_8x8() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,8u8,1u8,0u8,0u8,0u8,1u8,155u8,115u8,179u8,176u8,0u8,0u8,0u8,38u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,104u8,96u8,108u8,96u8,114u8,96u8,102u8,96u8,105u8,96u8,88u8,192u8,200u8,192u8,212u8,192u8,236u8,192u8,114u8,128u8,161u8,128u8,81u8,128u8,41u8,141u8,121u8,59u8,75u8,14u8,0u8,86u8,81u8,6u8,8u8,126u8,229u8,218u8,239u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_interlace_ct0_d1_8x8() -> Seq<u8> {seq![120u8,156u8,99u8,104u8,96u8,108u8,96u8,114u8,96u8,102u8,96u8,105u8,96u8,88u8,192u8,200u8,192u8,212u8,192u8,236u8,192u8,114u8,128u8,161u8,128u8,81u8,128u8,41u8,141u8,121u8,59u8,75u8,14u8,0u8,86u8,81u8,6u8,8u8]}
spec fn h_interlace_ct0_d1_8x8() -> crate::model::Ihdr {crate::model::Ihdr {width:8u32,height:8u32,depth:1u8,color_type:0u8,interlace:1u8}}
spec fn a_interlace_ct0_d1_8x8() -> r::Raster {r::shape(h_interlace_ct0_d1_8x8(),8nat,8nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_interlace_ct0_d1_8x8(b: Seq<u8>)
    requires b.len()==95,crate::model::mh(b)==Ok((crate::model::Ihdr {width:8u32,height:8u32,depth:1u8,color_type:0u8,interlace:1u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:38nat,next:83nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,83nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:91nat,len:0nat,next:95nat,crc_ok:true,synthetic:false})),
        b.subrange(41,79)==z_interlace_ct0_d1_8x8(),
    ensures r::default_case(b,crate::model::Ihdr {width:8u32,height:8u32,depth:1u8,color_type:0u8,interlace:1u8},z_interlace_ct0_d1_8x8(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    hide(z_interlace_ct0_d1_8x8);
        assert((1u32 << 1u8)==2u32) by (compute);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_interlace_ct0_d1_8x8() -> Seq<u8> {seq![0u8,128u8,1u8,128u8,2u8,64u8,3u8,0u8,4u8,128u8,0u8,160u8,1u8,0u8,2u8,128u8,3u8,64u8,4u8,192u8,0u8,112u8,1u8,16u8,2u8,102u8,3u8,183u8,4u8,108u8]}
spec fn ts_interlace_ct0_d1_8x8() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:35nat,before:1nat,after:2nat,symbol:128nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:35nat,next:43nat,before:2nat,after:3nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:43nat,next:51nat,before:3nat,after:4nat,symbol:128nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:51nat,next:59nat,before:4nat,after:5nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:59nat,next:67nat,before:5nat,after:6nat,symbol:64nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:67nat,next:75nat,before:6nat,after:7nat,symbol:3nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:75nat,next:83nat,before:7nat,after:8nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:83nat,next:91nat,before:8nat,after:9nat,symbol:4nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:91nat,next:99nat,before:9nat,after:10nat,symbol:128nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:99nat,next:107nat,before:10nat,after:11nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:107nat,next:116nat,before:11nat,after:12nat,symbol:160nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:116nat,next:124nat,before:12nat,after:13nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:124nat,next:132nat,before:13nat,after:14nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:132nat,next:140nat,before:14nat,after:15nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:140nat,next:148nat,before:15nat,after:16nat,symbol:128nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:148nat,next:156nat,before:16nat,after:17nat,symbol:3nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:156nat,next:164nat,before:17nat,after:18nat,symbol:64nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:164nat,next:172nat,before:18nat,after:19nat,symbol:4nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:172nat,next:181nat,before:19nat,after:20nat,symbol:192nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:181nat,next:189nat,before:20nat,after:21nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:189nat,next:197nat,before:21nat,after:22nat,symbol:112nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:197nat,next:205nat,before:22nat,after:23nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:205nat,next:213nat,before:23nat,after:24nat,symbol:16nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:213nat,next:221nat,before:24nat,after:25nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:221nat,next:229nat,before:25nat,after:26nat,symbol:102nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:229nat,next:237nat,before:26nat,after:27nat,symbol:3nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:237nat,next:246nat,before:27nat,after:28nat,symbol:183nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:246nat,next:254nat,before:28nat,after:29nat,symbol:4nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:254nat,next:262nat,before:29nat,after:30nat,symbol:108nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:262nat,next:269nat,before:30nat,after:30nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_interlace_ct0_d1_8x8()
    ensures t::trace(z_interlace_ct0_d1_8x8(),raw_interlace_ct0_d1_8x8(),ts_interlace_ct0_d1_8x8(),0,31nat),
{
    hide(z_interlace_ct0_d1_8x8);hide(raw_interlace_ct0_d1_8x8);hide(ts_interlace_ct0_d1_8x8);hide(t::valid_step);
    assert(t::trace(z_interlace_ct0_d1_8x8(),raw_interlace_ct0_d1_8x8(),ts_interlace_ct0_d1_8x8(),0,31nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_interlace_ct0_d1_8x8()
    ensures raw_interlace_ct0_d1_8x8().len()==30nat,
        z_interlace_ct0_d1_8x8().len()==38nat,
        z_interlace_ct0_d1_8x8()[0]==120u8,
        z_interlace_ct0_d1_8x8()[1]==156u8,
        zm::field_value(z_interlace_ct0_d1_8x8(),16,1)==1nat,
        zm::field_value(z_interlace_ct0_d1_8x8(),17,2)==1nat,
        zm::be_value(z_interlace_ct0_d1_8x8(),34nat,4)==1448150536nat,
        ts_interlace_ct0_d1_8x8().len()==31nat,
        ts_interlace_ct0_d1_8x8()[0].pos==19nat,
        ts_interlace_ct0_d1_8x8()[0].before==0nat,
        ts_interlace_ct0_d1_8x8().last().next==269nat,
{
    hide(z_interlace_ct0_d1_8x8);hide(raw_interlace_ct0_d1_8x8);hide(ts_interlace_ct0_d1_8x8);
    assert(raw_interlace_ct0_d1_8x8().len()==30nat) by (compute);
    assert(z_interlace_ct0_d1_8x8().len()==38nat) by (compute);
    assert(z_interlace_ct0_d1_8x8()[0]==120u8) by (compute);
    assert(z_interlace_ct0_d1_8x8()[1]==156u8) by (compute);
    assert(zm::field_value(z_interlace_ct0_d1_8x8(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_interlace_ct0_d1_8x8(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_interlace_ct0_d1_8x8(),34nat,4)==1448150536nat) by (compute);
    assert(ts_interlace_ct0_d1_8x8().len()==31nat) by (compute);
    assert(ts_interlace_ct0_d1_8x8()[0].pos==19nat) by (compute);
    assert(ts_interlace_ct0_d1_8x8()[0].before==0nat) by (compute);
    assert(ts_interlace_ct0_d1_8x8().last().next==269nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_interlace_ct0_d1_8x8()
    ensures zm::Adler(raw_interlace_ct0_d1_8x8())==1448150536nat,
{
    hide(raw_interlace_ct0_d1_8x8);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_interlace_ct0_d1_8x8().len()==30nat) by (compute);
    assert(c::sums(raw_interlace_ct0_d1_8x8(),0,30nat)==(1543nat,22097nat)) by (compute);
    c::sums_correct(raw_interlace_ct0_d1_8x8(),0,30nat);
    assert(raw_interlace_ct0_d1_8x8().subrange(0,30) =~= raw_interlace_ct0_d1_8x8());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((22097nat%65521)*(u16::MAX as nat+1)+(1+1543nat)%65521==1448150536nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_interlace_ct0_d1_8x8()
    ensures r::inflated(z_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_interlace_ct0_d1_8x8(),r2:false}),
        raw_interlace_ct0_d1_8x8().len()==30nat,
{
    hide(z_interlace_ct0_d1_8x8);hide(raw_interlace_ct0_d1_8x8);hide(ts_interlace_ct0_d1_8x8);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_interlace_ct0_d1_8x8();checksum_interlace_ct0_d1_8x8();
    assert(r::raw(a_interlace_ct0_d1_8x8())==30nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_interlace_ct0_d1_8x8();
    stream::fixed_stream(z_interlace_ct0_d1_8x8(),raw_interlace_ct0_d1_8x8(),ts_interlace_ct0_d1_8x8(),30nat,156nat,1448150536nat);
}
#[verifier::spinoff_prover]
proof fn scan_interlace_ct0_d1_8x8()
    ensures r::scan(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),0)==Ok(()),
{
    hide(raw_interlace_ct0_d1_8x8);
    assert(r::count(a_interlace_ct0_d1_8x8())==7nat) by (compute);
    r::scan_step(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),7);
    assert(r::pw(a_interlace_ct0_d1_8x8(),6nat)==8nat && r::ph(a_interlace_ct0_d1_8x8(),6nat)==4nat && r::rb(a_interlace_ct0_d1_8x8(),6nat)==1nat && r::base(a_interlace_ct0_d1_8x8(),6nat)==22nat) by (compute);
    assert(raw_interlace_ct0_d1_8x8()[22]==1u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),6nat,0nat)==Ok(()));
    assert(raw_interlace_ct0_d1_8x8()[24]==2u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),6nat,1nat)==Ok(()));
    assert(raw_interlace_ct0_d1_8x8()[26]==3u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),6nat,2nat)==Ok(()));
    assert(raw_interlace_ct0_d1_8x8()[28]==4u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),6nat,3nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),6nat,4nat);
    r::pass_step(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),6nat,3nat);
    r::pass_step(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),6nat,2nat);
    r::pass_step(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),6nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),6nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),6nat);
    assert(r::pw(a_interlace_ct0_d1_8x8(),5nat)==4nat && r::ph(a_interlace_ct0_d1_8x8(),5nat)==4nat && r::rb(a_interlace_ct0_d1_8x8(),5nat)==1nat && r::base(a_interlace_ct0_d1_8x8(),5nat)==14nat) by (compute);
    assert(raw_interlace_ct0_d1_8x8()[14]==2u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),5nat,0nat)==Ok(()));
    assert(raw_interlace_ct0_d1_8x8()[16]==3u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),5nat,1nat)==Ok(()));
    assert(raw_interlace_ct0_d1_8x8()[18]==4u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),5nat,2nat)==Ok(()));
    assert(raw_interlace_ct0_d1_8x8()[20]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),5nat,3nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),5nat,4nat);
    r::pass_step(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),5nat,3nat);
    r::pass_step(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),5nat,2nat);
    r::pass_step(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),5nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),5nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),5nat);
    assert(r::pw(a_interlace_ct0_d1_8x8(),4nat)==4nat && r::ph(a_interlace_ct0_d1_8x8(),4nat)==2nat && r::rb(a_interlace_ct0_d1_8x8(),4nat)==1nat && r::base(a_interlace_ct0_d1_8x8(),4nat)==10nat) by (compute);
    assert(raw_interlace_ct0_d1_8x8()[10]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),4nat,0nat)==Ok(()));
    assert(raw_interlace_ct0_d1_8x8()[12]==1u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),4nat,1nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),4nat,2nat);
    r::pass_step(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),4nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),4nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),4nat);
    assert(r::pw(a_interlace_ct0_d1_8x8(),3nat)==2nat && r::ph(a_interlace_ct0_d1_8x8(),3nat)==2nat && r::rb(a_interlace_ct0_d1_8x8(),3nat)==1nat && r::base(a_interlace_ct0_d1_8x8(),3nat)==6nat) by (compute);
    assert(raw_interlace_ct0_d1_8x8()[6]==3u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),3nat,0nat)==Ok(()));
    assert(raw_interlace_ct0_d1_8x8()[8]==4u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),3nat,1nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),3nat,2nat);
    r::pass_step(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),3nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),3nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),3nat);
    assert(r::pw(a_interlace_ct0_d1_8x8(),2nat)==2nat && r::ph(a_interlace_ct0_d1_8x8(),2nat)==1nat && r::rb(a_interlace_ct0_d1_8x8(),2nat)==1nat && r::base(a_interlace_ct0_d1_8x8(),2nat)==4nat) by (compute);
    assert(raw_interlace_ct0_d1_8x8()[4]==2u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),2nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),2nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),2nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),2nat);
    assert(r::pw(a_interlace_ct0_d1_8x8(),1nat)==1nat && r::ph(a_interlace_ct0_d1_8x8(),1nat)==1nat && r::rb(a_interlace_ct0_d1_8x8(),1nat)==1nat && r::base(a_interlace_ct0_d1_8x8(),1nat)==2nat) by (compute);
    assert(raw_interlace_ct0_d1_8x8()[2]==1u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),1nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),1nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),1nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),1nat);
    assert(r::pw(a_interlace_ct0_d1_8x8(),0nat)==1nat && r::ph(a_interlace_ct0_d1_8x8(),0nat)==1nat && r::rb(a_interlace_ct0_d1_8x8(),0nat)==1nat && r::base(a_interlace_ct0_d1_8x8(),0nat)==0nat) by (compute);
    assert(raw_interlace_ct0_d1_8x8()[0]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),0nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),0nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),0nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_interlace_ct0_d1_8x8()
    ensures r::still(bytes_interlace_ct0_d1_8x8(),STRICT,false)==(r::Projection::Ready {width:8u32,height:8u32,bytes:256nat}),
{
    hide(bytes_interlace_ct0_d1_8x8);hide(z_interlace_ct0_d1_8x8);hide(raw_interlace_ct0_d1_8x8);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_interlace_ct0_d1_8x8().len()==95nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct0_d1_8x8())==Ok((crate::model::Ihdr {width:8u32,height:8u32,depth:1u8,color_type:0u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct0_d1_8x8())==Ok((crate::model::Ihdr {width:8u32,height:8u32,depth:1u8,color_type:0u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 1u8)==2u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct0_d1_8x8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:38nat,next:83nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct0_d1_8x8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:38nat,next:83nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct0_d1_8x8(),83nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:91nat,len:0nat,next:95nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct0_d1_8x8(),83nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:91nat,len:0nat,next:95nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct0_d1_8x8().subrange(41,79)==z_interlace_ct0_d1_8x8()) by (compute);
    walk_interlace_ct0_d1_8x8(bytes_interlace_ct0_d1_8x8());
    inflate_interlace_ct0_d1_8x8();scan_interlace_ct0_d1_8x8();
    assert(r::raw(a_interlace_ct0_d1_8x8())==30nat) by (compute);
    assert(r::data(z_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_interlace_ct0_d1_8x8(),h_interlace_ct0_d1_8x8(),z_interlace_ct0_d1_8x8(),0nat,false);
}
fn public_interlace_ct0_d1_8x8() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,8u8,1u8,0u8,0u8,0u8,1u8,155u8,115u8,179u8,176u8,0u8,0u8,0u8,38u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,104u8,96u8,108u8,96u8,114u8,96u8,102u8,96u8,105u8,96u8,88u8,192u8,200u8,192u8,212u8,192u8,236u8,192u8,114u8,128u8,161u8,128u8,81u8,128u8,41u8,141u8,121u8,59u8,75u8,14u8,0u8,86u8,81u8,6u8,8u8,126u8,229u8,218u8,239u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_interlace_ct0_d1_8x8());model_interlace_ct0_d1_8x8();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==8u32 && s.height==8u32 && s.rgba@.len()==256nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// interlace/ct6-d16-8x8; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_interlace_ct6_d16_8x8() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,8u8,16u8,6u8,0u8,0u8,1u8,227u8,152u8,82u8,94u8,0u8,0u8,2u8,26u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,15u8,2u8,240u8,253u8,0u8,74u8,135u8,183u8,180u8,49u8,71u8,131u8,229u8,1u8,90u8,155u8,171u8,67u8,219u8,123u8,213u8,78u8,2u8,57u8,70u8,159u8,194u8,106u8,221u8,6u8,19u8,62u8,67u8,91u8,207u8,143u8,40u8,31u8,209u8,3u8,253u8,150u8,22u8,157u8,0u8,42u8,136u8,97u8,60u8,66u8,42u8,1u8,105u8,251u8,24u8,141u8,4u8,14u8,21u8,230u8,114u8,152u8,203u8,54u8,71u8,47u8,29u8,205u8,183u8,111u8,22u8,129u8,43u8,0u8,131u8,226u8,223u8,202u8,105u8,248u8,33u8,83u8,254u8,128u8,210u8,6u8,45u8,230u8,163u8,240u8,4u8,199u8,240u8,17u8,47u8,233u8,78u8,151u8,38u8,152u8,252u8,18u8,153u8,198u8,27u8,138u8,1u8,195u8,71u8,133u8,85u8,164u8,46u8,11u8,63u8,163u8,239u8,178u8,116u8,239u8,51u8,203u8,120u8,63u8,47u8,2u8,108u8,179u8,61u8,208u8,228u8,110u8,111u8,93u8,232u8,194u8,39u8,153u8,69u8,2u8,235u8,236u8,29u8,178u8,82u8,3u8,208u8,141u8,104u8,62u8,97u8,19u8,230u8,59u8,7u8,89u8,216u8,226u8,48u8,150u8,62u8,198u8,123u8,4u8,229u8,34u8,190u8,201u8,92u8,115u8,154u8,150u8,3u8,145u8,47u8,71u8,173u8,87u8,213u8,157u8,249u8,254u8,222u8,165u8,89u8,60u8,126u8,201u8,76u8,164u8,104u8,66u8,78u8,220u8,112u8,151u8,175u8,93u8,64u8,30u8,212u8,115u8,154u8,8u8,167u8,4u8,23u8,192u8,165u8,8u8,49u8,234u8,126u8,91u8,218u8,137u8,149u8,124u8,52u8,142u8,111u8,72u8,208u8,102u8,196u8,73u8,58u8,87u8,204u8,169u8,45u8,190u8,142u8,207u8,116u8,85u8,195u8,194u8,0u8,75u8,122u8,104u8,183u8,226u8,85u8,98u8,215u8,13u8,130u8,129u8,202u8,18u8,26u8,116u8,236u8,53u8,152u8,202u8,240u8,84u8,251u8,110u8,221u8,65u8,47u8,227u8,193u8,186u8,180u8,233u8,209u8,1u8,148u8,62u8,246u8,131u8,187u8,154u8,171u8,181u8,163u8,83u8,104u8,9u8,250u8,9u8,45u8,150u8,182u8,33u8,146u8,30u8,174u8,3u8,96u8,61u8,216u8,69u8,71u8,146u8,52u8,240u8,6u8,250u8,86u8,94u8,60u8,191u8,58u8,89u8,71u8,203u8,63u8,216u8,101u8,212u8,255u8,188u8,248u8,19u8,79u8,137u8,224u8,160u8,185u8,194u8,159u8,117u8,101u8,94u8,23u8,245u8,52u8,22u8,61u8,239u8,2u8,55u8,200u8,217u8,115u8,111u8,220u8,36u8,117u8,120u8,160u8,138u8,56u8,50u8,225u8,41u8,97u8,87u8,55u8,101u8,210u8,118u8,34u8,30u8,252u8,130u8,219u8,25u8,31u8,164u8,85u8,202u8,184u8,246u8,196u8,162u8,97u8,202u8,72u8,59u8,79u8,1u8,249u8,48u8,209u8,99u8,220u8,77u8,84u8,42u8,185u8,236u8,39u8,198u8,224u8,101u8,194u8,116u8,255u8,1u8,156u8,145u8,158u8,116u8,172u8,3u8,68u8,102u8,167u8,4u8,233u8,84u8,165u8,120u8,35u8,9u8,47u8,123u8,112u8,61u8,34u8,81u8,51u8,141u8,157u8,91u8,80u8,215u8,14u8,188u8,55u8,108u8,212u8,188u8,34u8,5u8,42u8,191u8,203u8,169u8,240u8,165u8,177u8,28u8,251u8,157u8,144u8,9u8,64u8,94u8,200u8,187u8,180u8,253u8,172u8,60u8,128u8,108u8,76u8,58u8,9u8,151u8,99u8,65u8,221u8,96u8,188u8,53u8,169u8,29u8,4u8,15u8,222u8,0u8,198u8,132u8,34u8,52u8,194u8,245u8,100u8,148u8,120u8,236u8,0u8,63u8,138u8,231u8,28u8,65u8,161u8,17u8,55u8,28u8,132u8,244u8,41u8,110u8,170u8,30u8,207u8,233u8,1u8,190u8,70u8,173u8,226u8,54u8,127u8,34u8,147u8,218u8,96u8,165u8,87u8,246u8,177u8,157u8,248u8,113u8,146u8,104u8,184u8,5u8,53u8,156u8,88u8,100u8,247u8,13u8,171u8,34u8,119u8,50u8,114u8,130u8,52u8,250u8,135u8,224u8,177u8,204u8,96u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_interlace_ct6_d16_8x8() -> Seq<u8> {seq![120u8,156u8,1u8,15u8,2u8,240u8,253u8,0u8,74u8,135u8,183u8,180u8,49u8,71u8,131u8,229u8,1u8,90u8,155u8,171u8,67u8,219u8,123u8,213u8,78u8,2u8,57u8,70u8,159u8,194u8,106u8,221u8,6u8,19u8,62u8,67u8,91u8,207u8,143u8,40u8,31u8,209u8,3u8,253u8,150u8,22u8,157u8,0u8,42u8,136u8,97u8,60u8,66u8,42u8,1u8,105u8,251u8,24u8,141u8,4u8,14u8,21u8,230u8,114u8,152u8,203u8,54u8,71u8,47u8,29u8,205u8,183u8,111u8,22u8,129u8,43u8,0u8,131u8,226u8,223u8,202u8,105u8,248u8,33u8,83u8,254u8,128u8,210u8,6u8,45u8,230u8,163u8,240u8,4u8,199u8,240u8,17u8,47u8,233u8,78u8,151u8,38u8,152u8,252u8,18u8,153u8,198u8,27u8,138u8,1u8,195u8,71u8,133u8,85u8,164u8,46u8,11u8,63u8,163u8,239u8,178u8,116u8,239u8,51u8,203u8,120u8,63u8,47u8,2u8,108u8,179u8,61u8,208u8,228u8,110u8,111u8,93u8,232u8,194u8,39u8,153u8,69u8,2u8,235u8,236u8,29u8,178u8,82u8,3u8,208u8,141u8,104u8,62u8,97u8,19u8,230u8,59u8,7u8,89u8,216u8,226u8,48u8,150u8,62u8,198u8,123u8,4u8,229u8,34u8,190u8,201u8,92u8,115u8,154u8,150u8,3u8,145u8,47u8,71u8,173u8,87u8,213u8,157u8,249u8,254u8,222u8,165u8,89u8,60u8,126u8,201u8,76u8,164u8,104u8,66u8,78u8,220u8,112u8,151u8,175u8,93u8,64u8,30u8,212u8,115u8,154u8,8u8,167u8,4u8,23u8,192u8,165u8,8u8,49u8,234u8,126u8,91u8,218u8,137u8,149u8,124u8,52u8,142u8,111u8,72u8,208u8,102u8,196u8,73u8,58u8,87u8,204u8,169u8,45u8,190u8,142u8,207u8,116u8,85u8,195u8,194u8,0u8,75u8,122u8,104u8,183u8,226u8,85u8,98u8,215u8,13u8,130u8,129u8,202u8,18u8,26u8,116u8,236u8,53u8,152u8,202u8,240u8,84u8,251u8,110u8,221u8,65u8,47u8,227u8,193u8,186u8,180u8,233u8,209u8,1u8,148u8,62u8,246u8,131u8,187u8,154u8,171u8,181u8,163u8,83u8,104u8,9u8,250u8,9u8,45u8,150u8,182u8,33u8,146u8,30u8,174u8,3u8,96u8,61u8,216u8,69u8,71u8,146u8,52u8,240u8,6u8,250u8,86u8,94u8,60u8,191u8,58u8,89u8,71u8,203u8,63u8,216u8,101u8,212u8,255u8,188u8,248u8,19u8,79u8,137u8,224u8,160u8,185u8,194u8,159u8,117u8,101u8,94u8,23u8,245u8,52u8,22u8,61u8,239u8,2u8,55u8,200u8,217u8,115u8,111u8,220u8,36u8,117u8,120u8,160u8,138u8,56u8,50u8,225u8,41u8,97u8,87u8,55u8,101u8,210u8,118u8,34u8,30u8,252u8,130u8,219u8,25u8,31u8,164u8,85u8,202u8,184u8,246u8,196u8,162u8,97u8,202u8,72u8,59u8,79u8,1u8,249u8,48u8,209u8,99u8,220u8,77u8,84u8,42u8,185u8,236u8,39u8,198u8,224u8,101u8,194u8,116u8,255u8,1u8,156u8,145u8,158u8,116u8,172u8,3u8,68u8,102u8,167u8,4u8,233u8,84u8,165u8,120u8,35u8,9u8,47u8,123u8,112u8,61u8,34u8,81u8,51u8,141u8,157u8,91u8,80u8,215u8,14u8,188u8,55u8,108u8,212u8,188u8,34u8,5u8,42u8,191u8,203u8,169u8,240u8,165u8,177u8,28u8,251u8,157u8,144u8,9u8,64u8,94u8,200u8,187u8,180u8,253u8,172u8,60u8,128u8,108u8,76u8,58u8,9u8,151u8,99u8,65u8,221u8,96u8,188u8,53u8,169u8,29u8,4u8,15u8,222u8,0u8,198u8,132u8,34u8,52u8,194u8,245u8,100u8,148u8,120u8,236u8,0u8,63u8,138u8,231u8,28u8,65u8,161u8,17u8,55u8,28u8,132u8,244u8,41u8,110u8,170u8,30u8,207u8,233u8,1u8,190u8,70u8,173u8,226u8,54u8,127u8,34u8,147u8,218u8,96u8,165u8,87u8,246u8,177u8,157u8,248u8,113u8,146u8,104u8,184u8,5u8,53u8,156u8,88u8,100u8,247u8,13u8,171u8,34u8,119u8,50u8,114u8,130u8,52u8,250u8,135u8]}
spec fn h_interlace_ct6_d16_8x8() -> crate::model::Ihdr {crate::model::Ihdr {width:8u32,height:8u32,depth:16u8,color_type:6u8,interlace:1u8}}
spec fn a_interlace_ct6_d16_8x8() -> r::Raster {r::shape(h_interlace_ct6_d16_8x8(),8nat,8nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn crc_interlace_ct6_d16_8x8()
    ensures crate::model::crc(bytes_interlace_ct6_d16_8x8().subrange(37,579))==3769748576u32,
{
    hide(crate::model::crc_register);hide(crc_cert::fold);
    assert(crate::model::crc_register(bytes_interlace_ct6_d16_8x8().subrange(37,579).take(0))==0xffff_ffffu32) by {
        assert(bytes_interlace_ct6_d16_8x8().subrange(37,579).take(0) =~= Seq::<u8>::empty());
        reveal_with_fuel(crate::model::crc_register,1);
    }
    assert(crc_cert::fold(4294967295u32,bytes_interlace_ct6_d16_8x8().subrange(37,579).subrange(0,64))==1593252432u32) by (compute);
    crc_cert::suffix(bytes_interlace_ct6_d16_8x8().subrange(37,579).take(0),bytes_interlace_ct6_d16_8x8().subrange(37,579).subrange(0,64));
    assert(bytes_interlace_ct6_d16_8x8().subrange(37,579).take(0)+bytes_interlace_ct6_d16_8x8().subrange(37,579).subrange(0,64) =~= bytes_interlace_ct6_d16_8x8().subrange(37,579).take(64));
    assert(crate::model::crc_register(bytes_interlace_ct6_d16_8x8().subrange(37,579).take(64))==1593252432u32);
    assert(crc_cert::fold(1593252432u32,bytes_interlace_ct6_d16_8x8().subrange(37,579).subrange(64,128))==2558210290u32) by (compute);
    crc_cert::suffix(bytes_interlace_ct6_d16_8x8().subrange(37,579).take(64),bytes_interlace_ct6_d16_8x8().subrange(37,579).subrange(64,128));
    assert(bytes_interlace_ct6_d16_8x8().subrange(37,579).take(64)+bytes_interlace_ct6_d16_8x8().subrange(37,579).subrange(64,128) =~= bytes_interlace_ct6_d16_8x8().subrange(37,579).take(128));
    assert(crate::model::crc_register(bytes_interlace_ct6_d16_8x8().subrange(37,579).take(128))==2558210290u32);
    assert(crc_cert::fold(2558210290u32,bytes_interlace_ct6_d16_8x8().subrange(37,579).subrange(128,192))==4110561463u32) by (compute);
    crc_cert::suffix(bytes_interlace_ct6_d16_8x8().subrange(37,579).take(128),bytes_interlace_ct6_d16_8x8().subrange(37,579).subrange(128,192));
    assert(bytes_interlace_ct6_d16_8x8().subrange(37,579).take(128)+bytes_interlace_ct6_d16_8x8().subrange(37,579).subrange(128,192) =~= bytes_interlace_ct6_d16_8x8().subrange(37,579).take(192));
    assert(crate::model::crc_register(bytes_interlace_ct6_d16_8x8().subrange(37,579).take(192))==4110561463u32);
    assert(crc_cert::fold(4110561463u32,bytes_interlace_ct6_d16_8x8().subrange(37,579).subrange(192,256))==2681531128u32) by (compute);
    crc_cert::suffix(bytes_interlace_ct6_d16_8x8().subrange(37,579).take(192),bytes_interlace_ct6_d16_8x8().subrange(37,579).subrange(192,256));
    assert(bytes_interlace_ct6_d16_8x8().subrange(37,579).take(192)+bytes_interlace_ct6_d16_8x8().subrange(37,579).subrange(192,256) =~= bytes_interlace_ct6_d16_8x8().subrange(37,579).take(256));
    assert(crate::model::crc_register(bytes_interlace_ct6_d16_8x8().subrange(37,579).take(256))==2681531128u32);
    assert(crc_cert::fold(2681531128u32,bytes_interlace_ct6_d16_8x8().subrange(37,579).subrange(256,320))==3878707835u32) by (compute);
    crc_cert::suffix(bytes_interlace_ct6_d16_8x8().subrange(37,579).take(256),bytes_interlace_ct6_d16_8x8().subrange(37,579).subrange(256,320));
    assert(bytes_interlace_ct6_d16_8x8().subrange(37,579).take(256)+bytes_interlace_ct6_d16_8x8().subrange(37,579).subrange(256,320) =~= bytes_interlace_ct6_d16_8x8().subrange(37,579).take(320));
    assert(crate::model::crc_register(bytes_interlace_ct6_d16_8x8().subrange(37,579).take(320))==3878707835u32);
    assert(crc_cert::fold(3878707835u32,bytes_interlace_ct6_d16_8x8().subrange(37,579).subrange(320,384))==3961846850u32) by (compute);
    crc_cert::suffix(bytes_interlace_ct6_d16_8x8().subrange(37,579).take(320),bytes_interlace_ct6_d16_8x8().subrange(37,579).subrange(320,384));
    assert(bytes_interlace_ct6_d16_8x8().subrange(37,579).take(320)+bytes_interlace_ct6_d16_8x8().subrange(37,579).subrange(320,384) =~= bytes_interlace_ct6_d16_8x8().subrange(37,579).take(384));
    assert(crate::model::crc_register(bytes_interlace_ct6_d16_8x8().subrange(37,579).take(384))==3961846850u32);
    assert(crc_cert::fold(3961846850u32,bytes_interlace_ct6_d16_8x8().subrange(37,579).subrange(384,448))==4142445297u32) by (compute);
    crc_cert::suffix(bytes_interlace_ct6_d16_8x8().subrange(37,579).take(384),bytes_interlace_ct6_d16_8x8().subrange(37,579).subrange(384,448));
    assert(bytes_interlace_ct6_d16_8x8().subrange(37,579).take(384)+bytes_interlace_ct6_d16_8x8().subrange(37,579).subrange(384,448) =~= bytes_interlace_ct6_d16_8x8().subrange(37,579).take(448));
    assert(crate::model::crc_register(bytes_interlace_ct6_d16_8x8().subrange(37,579).take(448))==4142445297u32);
    assert(crc_cert::fold(4142445297u32,bytes_interlace_ct6_d16_8x8().subrange(37,579).subrange(448,512))==2906713706u32) by (compute);
    crc_cert::suffix(bytes_interlace_ct6_d16_8x8().subrange(37,579).take(448),bytes_interlace_ct6_d16_8x8().subrange(37,579).subrange(448,512));
    assert(bytes_interlace_ct6_d16_8x8().subrange(37,579).take(448)+bytes_interlace_ct6_d16_8x8().subrange(37,579).subrange(448,512) =~= bytes_interlace_ct6_d16_8x8().subrange(37,579).take(512));
    assert(crate::model::crc_register(bytes_interlace_ct6_d16_8x8().subrange(37,579).take(512))==2906713706u32);
    assert(crc_cert::fold(2906713706u32,bytes_interlace_ct6_d16_8x8().subrange(37,579).subrange(512,542))==525218719u32) by (compute);
    crc_cert::suffix(bytes_interlace_ct6_d16_8x8().subrange(37,579).take(512),bytes_interlace_ct6_d16_8x8().subrange(37,579).subrange(512,542));
    assert(bytes_interlace_ct6_d16_8x8().subrange(37,579).take(512)+bytes_interlace_ct6_d16_8x8().subrange(37,579).subrange(512,542) =~= bytes_interlace_ct6_d16_8x8().subrange(37,579).take(542));
    assert(crate::model::crc_register(bytes_interlace_ct6_d16_8x8().subrange(37,579).take(542))==525218719u32);
    assert(bytes_interlace_ct6_d16_8x8().subrange(37,579).take(542) =~= bytes_interlace_ct6_d16_8x8().subrange(37,579));
    assert((525218719u32 ^ 0xffff_ffffu32)==3769748576u32) by (compute);
}
#[verifier::spinoff_prover]
proof fn walk_interlace_ct6_d16_8x8(b: Seq<u8>)
    requires b.len()==595,crate::model::mh(b)==Ok((crate::model::Ihdr {width:8u32,height:8u32,depth:16u8,color_type:6u8,interlace:1u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:538nat,next:583nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,583nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:591nat,len:0nat,next:595nat,crc_ok:true,synthetic:false})),
        b.subrange(41,579)==z_interlace_ct6_d16_8x8(),
    ensures r::default_case(b,crate::model::Ihdr {width:8u32,height:8u32,depth:16u8,color_type:6u8,interlace:1u8},z_interlace_ct6_d16_8x8(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    hide(z_interlace_ct6_d16_8x8);
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_interlace_ct6_d16_8x8() -> Seq<u8> {seq![0u8,74u8,135u8,183u8,180u8,49u8,71u8,131u8,229u8,1u8,90u8,155u8,171u8,67u8,219u8,123u8,213u8,78u8,2u8,57u8,70u8,159u8,194u8,106u8,221u8,6u8,19u8,62u8,67u8,91u8,207u8,143u8,40u8,31u8,209u8,3u8,253u8,150u8,22u8,157u8,0u8,42u8,136u8,97u8,60u8,66u8,42u8,1u8,105u8,251u8,24u8,141u8,4u8,14u8,21u8,230u8,114u8,152u8,203u8,54u8,71u8,47u8,29u8,205u8,183u8,111u8,22u8,129u8,43u8,0u8,131u8,226u8,223u8,202u8,105u8,248u8,33u8,83u8,254u8,128u8,210u8,6u8,45u8,230u8,163u8,240u8,4u8,199u8,240u8,17u8,47u8,233u8,78u8,151u8,38u8,152u8,252u8,18u8,153u8,198u8,27u8,138u8,1u8,195u8,71u8,133u8,85u8,164u8,46u8,11u8,63u8,163u8,239u8,178u8,116u8,239u8,51u8,203u8,120u8,63u8,47u8,2u8,108u8,179u8,61u8,208u8,228u8,110u8,111u8,93u8,232u8,194u8,39u8,153u8,69u8,2u8,235u8,236u8,29u8,178u8,82u8,3u8,208u8,141u8,104u8,62u8,97u8,19u8,230u8,59u8,7u8,89u8,216u8,226u8,48u8,150u8,62u8,198u8,123u8,4u8,229u8,34u8,190u8,201u8,92u8,115u8,154u8,150u8,3u8,145u8,47u8,71u8,173u8,87u8,213u8,157u8,249u8,254u8,222u8,165u8,89u8,60u8,126u8,201u8,76u8,164u8,104u8,66u8,78u8,220u8,112u8,151u8,175u8,93u8,64u8,30u8,212u8,115u8,154u8,8u8,167u8,4u8,23u8,192u8,165u8,8u8,49u8,234u8,126u8,91u8,218u8,137u8,149u8,124u8,52u8,142u8,111u8,72u8,208u8,102u8,196u8,73u8,58u8,87u8,204u8,169u8,45u8,190u8,142u8,207u8,116u8,85u8,195u8,194u8,0u8,75u8,122u8,104u8,183u8,226u8,85u8,98u8,215u8,13u8,130u8,129u8,202u8,18u8,26u8,116u8,236u8,53u8,152u8,202u8,240u8,84u8,251u8,110u8,221u8,65u8,47u8,227u8,193u8,186u8,180u8,233u8,209u8,1u8,148u8,62u8,246u8,131u8,187u8,154u8,171u8,181u8,163u8,83u8,104u8,9u8,250u8,9u8,45u8,150u8,182u8,33u8,146u8,30u8,174u8,3u8,96u8,61u8,216u8,69u8,71u8,146u8,52u8,240u8,6u8,250u8,86u8,94u8,60u8,191u8,58u8,89u8,71u8,203u8,63u8,216u8,101u8,212u8,255u8,188u8,248u8,19u8,79u8,137u8,224u8,160u8,185u8,194u8,159u8,117u8,101u8,94u8,23u8,245u8,52u8,22u8,61u8,239u8,2u8,55u8,200u8,217u8,115u8,111u8,220u8,36u8,117u8,120u8,160u8,138u8,56u8,50u8,225u8,41u8,97u8,87u8,55u8,101u8,210u8,118u8,34u8,30u8,252u8,130u8,219u8,25u8,31u8,164u8,85u8,202u8,184u8,246u8,196u8,162u8,97u8,202u8,72u8,59u8,79u8,1u8,249u8,48u8,209u8,99u8,220u8,77u8,84u8,42u8,185u8,236u8,39u8,198u8,224u8,101u8,194u8,116u8,255u8,1u8,156u8,145u8,158u8,116u8,172u8,3u8,68u8,102u8,167u8,4u8,233u8,84u8,165u8,120u8,35u8,9u8,47u8,123u8,112u8,61u8,34u8,81u8,51u8,141u8,157u8,91u8,80u8,215u8,14u8,188u8,55u8,108u8,212u8,188u8,34u8,5u8,42u8,191u8,203u8,169u8,240u8,165u8,177u8,28u8,251u8,157u8,144u8,9u8,64u8,94u8,200u8,187u8,180u8,253u8,172u8,60u8,128u8,108u8,76u8,58u8,9u8,151u8,99u8,65u8,221u8,96u8,188u8,53u8,169u8,29u8,4u8,15u8,222u8,0u8,198u8,132u8,34u8,52u8,194u8,245u8,100u8,148u8,120u8,236u8,0u8,63u8,138u8,231u8,28u8,65u8,161u8,17u8,55u8,28u8,132u8,244u8,41u8,110u8,170u8,30u8,207u8,233u8,1u8,190u8,70u8,173u8,226u8,54u8,127u8,34u8,147u8,218u8,96u8,165u8,87u8,246u8,177u8,157u8,248u8,113u8,146u8,104u8,184u8,5u8,53u8,156u8,88u8,100u8,247u8,13u8,171u8,34u8,119u8,50u8,114u8]}
#[verifier::spinoff_prover]
proof fn shape_interlace_ct6_d16_8x8()
    ensures raw_interlace_ct6_d16_8x8().len()==527nat,
        z_interlace_ct6_d16_8x8().len()==538nat,
        z_interlace_ct6_d16_8x8()[0]==120u8,
        z_interlace_ct6_d16_8x8()[1]==156u8,
        zm::field_value(z_interlace_ct6_d16_8x8(),16,1)==1nat,
        zm::field_value(z_interlace_ct6_d16_8x8(),17,2)==0nat,
        zm::be_value(z_interlace_ct6_d16_8x8(),534nat,4)==2184510087nat,
        z_interlace_ct6_d16_8x8().subrange(7,534)==raw_interlace_ct6_d16_8x8(),
        z_interlace_ct6_d16_8x8()[3] as nat+256*z_interlace_ct6_d16_8x8()[4] as nat==527nat,
        z_interlace_ct6_d16_8x8()[5] as nat+256*z_interlace_ct6_d16_8x8()[6] as nat==u16::MAX as nat-527nat,
        zm::be_value(z_interlace_ct6_d16_8x8(),0,1)==120nat,
        zm::be_value(z_interlace_ct6_d16_8x8(),1,1)==156nat,
{
    hide(z_interlace_ct6_d16_8x8);hide(raw_interlace_ct6_d16_8x8);
    assert(raw_interlace_ct6_d16_8x8().len()==527nat) by (compute);
    assert(z_interlace_ct6_d16_8x8().len()==538nat) by (compute);
    assert(z_interlace_ct6_d16_8x8()[0]==120u8) by (compute);
    assert(z_interlace_ct6_d16_8x8()[1]==156u8) by (compute);
    assert(zm::field_value(z_interlace_ct6_d16_8x8(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_interlace_ct6_d16_8x8(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_interlace_ct6_d16_8x8(),534nat,4)==2184510087nat) by (compute);
    assert(z_interlace_ct6_d16_8x8().subrange(7,534)==raw_interlace_ct6_d16_8x8()) by (compute);
    assert(z_interlace_ct6_d16_8x8()[3] as nat+256*z_interlace_ct6_d16_8x8()[4] as nat==527nat) by (compute);
    assert(z_interlace_ct6_d16_8x8()[5] as nat+256*z_interlace_ct6_d16_8x8()[6] as nat==u16::MAX as nat-527nat) by (compute);
    assert(zm::be_value(z_interlace_ct6_d16_8x8(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_interlace_ct6_d16_8x8(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_interlace_ct6_d16_8x8()
    ensures zm::Adler(raw_interlace_ct6_d16_8x8())==2184510087nat,
{
    hide(raw_interlace_ct6_d16_8x8);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_interlace_ct6_d16_8x8().len()==527nat) by (compute);
    assert(c::sums(raw_interlace_ct6_d16_8x8(),0,527nat)==(64134nat,16741187nat)) by (compute);
    c::sums_correct(raw_interlace_ct6_d16_8x8(),0,527nat);
    assert(raw_interlace_ct6_d16_8x8().subrange(0,527) =~= raw_interlace_ct6_d16_8x8());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((16741187nat%65521)*(u16::MAX as nat+1)+(1+64134nat)%65521==2184510087nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_interlace_ct6_d16_8x8()
    ensures r::inflated(z_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_interlace_ct6_d16_8x8(),r2:false}),
        raw_interlace_ct6_d16_8x8().len()==527nat,
{
    hide(z_interlace_ct6_d16_8x8);hide(raw_interlace_ct6_d16_8x8);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_interlace_ct6_d16_8x8();checksum_interlace_ct6_d16_8x8();
    assert(r::raw(a_interlace_ct6_d16_8x8())==527nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_interlace_ct6_d16_8x8(),raw_interlace_ct6_d16_8x8(),527nat,120nat,156nat,2184510087nat);
}
#[verifier::spinoff_prover]
proof fn scan_interlace_ct6_d16_8x8()
    ensures r::scan(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),0)==Ok(()),
{
    hide(raw_interlace_ct6_d16_8x8);
    assert(r::count(a_interlace_ct6_d16_8x8())==7nat) by (compute);
    r::scan_step(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),7);
    assert(r::pw(a_interlace_ct6_d16_8x8(),6nat)==8nat && r::ph(a_interlace_ct6_d16_8x8(),6nat)==4nat && r::rb(a_interlace_ct6_d16_8x8(),6nat)==64nat && r::base(a_interlace_ct6_d16_8x8(),6nat)==267nat) by (compute);
    assert(raw_interlace_ct6_d16_8x8()[267]==1u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),6nat,0nat)==Ok(()));
    assert(raw_interlace_ct6_d16_8x8()[332]==2u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),6nat,1nat)==Ok(()));
    assert(raw_interlace_ct6_d16_8x8()[397]==3u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),6nat,2nat)==Ok(()));
    assert(raw_interlace_ct6_d16_8x8()[462]==4u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),6nat,3nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),6nat,4nat);
    r::pass_step(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),6nat,3nat);
    r::pass_step(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),6nat,2nat);
    r::pass_step(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),6nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),6nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),6nat);
    assert(r::pw(a_interlace_ct6_d16_8x8(),5nat)==4nat && r::ph(a_interlace_ct6_d16_8x8(),5nat)==4nat && r::rb(a_interlace_ct6_d16_8x8(),5nat)==32nat && r::base(a_interlace_ct6_d16_8x8(),5nat)==135nat) by (compute);
    assert(raw_interlace_ct6_d16_8x8()[135]==2u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),5nat,0nat)==Ok(()));
    assert(raw_interlace_ct6_d16_8x8()[168]==3u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),5nat,1nat)==Ok(()));
    assert(raw_interlace_ct6_d16_8x8()[201]==4u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),5nat,2nat)==Ok(()));
    assert(raw_interlace_ct6_d16_8x8()[234]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),5nat,3nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),5nat,4nat);
    r::pass_step(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),5nat,3nat);
    r::pass_step(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),5nat,2nat);
    r::pass_step(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),5nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),5nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),5nat);
    assert(r::pw(a_interlace_ct6_d16_8x8(),4nat)==4nat && r::ph(a_interlace_ct6_d16_8x8(),4nat)==2nat && r::rb(a_interlace_ct6_d16_8x8(),4nat)==32nat && r::base(a_interlace_ct6_d16_8x8(),4nat)==69nat) by (compute);
    assert(raw_interlace_ct6_d16_8x8()[69]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),4nat,0nat)==Ok(()));
    assert(raw_interlace_ct6_d16_8x8()[102]==1u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),4nat,1nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),4nat,2nat);
    r::pass_step(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),4nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),4nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),4nat);
    assert(r::pw(a_interlace_ct6_d16_8x8(),3nat)==2nat && r::ph(a_interlace_ct6_d16_8x8(),3nat)==2nat && r::rb(a_interlace_ct6_d16_8x8(),3nat)==16nat && r::base(a_interlace_ct6_d16_8x8(),3nat)==35nat) by (compute);
    assert(raw_interlace_ct6_d16_8x8()[35]==3u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),3nat,0nat)==Ok(()));
    assert(raw_interlace_ct6_d16_8x8()[52]==4u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),3nat,1nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),3nat,2nat);
    r::pass_step(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),3nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),3nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),3nat);
    assert(r::pw(a_interlace_ct6_d16_8x8(),2nat)==2nat && r::ph(a_interlace_ct6_d16_8x8(),2nat)==1nat && r::rb(a_interlace_ct6_d16_8x8(),2nat)==16nat && r::base(a_interlace_ct6_d16_8x8(),2nat)==18nat) by (compute);
    assert(raw_interlace_ct6_d16_8x8()[18]==2u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),2nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),2nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),2nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),2nat);
    assert(r::pw(a_interlace_ct6_d16_8x8(),1nat)==1nat && r::ph(a_interlace_ct6_d16_8x8(),1nat)==1nat && r::rb(a_interlace_ct6_d16_8x8(),1nat)==8nat && r::base(a_interlace_ct6_d16_8x8(),1nat)==9nat) by (compute);
    assert(raw_interlace_ct6_d16_8x8()[9]==1u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),1nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),1nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),1nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),1nat);
    assert(r::pw(a_interlace_ct6_d16_8x8(),0nat)==1nat && r::ph(a_interlace_ct6_d16_8x8(),0nat)==1nat && r::rb(a_interlace_ct6_d16_8x8(),0nat)==8nat && r::base(a_interlace_ct6_d16_8x8(),0nat)==0nat) by (compute);
    assert(raw_interlace_ct6_d16_8x8()[0]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),0nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),0nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),0nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_interlace_ct6_d16_8x8()
    ensures r::still(bytes_interlace_ct6_d16_8x8(),STRICT,false)==(r::Projection::Ready {width:8u32,height:8u32,bytes:256nat}),
{
    hide(bytes_interlace_ct6_d16_8x8);hide(z_interlace_ct6_d16_8x8);hide(raw_interlace_ct6_d16_8x8);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_interlace_ct6_d16_8x8().len()==595nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct6_d16_8x8())==Ok((crate::model::Ihdr {width:8u32,height:8u32,depth:16u8,color_type:6u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct6_d16_8x8())==Ok((crate::model::Ihdr {width:8u32,height:8u32,depth:16u8,color_type:6u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct6_d16_8x8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:538nat,next:583nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::model::frame(bytes_interlace_ct6_d16_8x8(),33nat)==Ok(crate::model::WireChunk {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:538nat,next:583nat})) by (compute);
            assert(crate::model::be32(bytes_interlace_ct6_d16_8x8(),579)==3769748576u32) by (compute);
            crc_interlace_ct6_d16_8x8();
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct6_d16_8x8(),583nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:591nat,len:0nat,next:595nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct6_d16_8x8(),583nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:591nat,len:0nat,next:595nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct6_d16_8x8().subrange(41,579)==z_interlace_ct6_d16_8x8()) by (compute);
    walk_interlace_ct6_d16_8x8(bytes_interlace_ct6_d16_8x8());
    inflate_interlace_ct6_d16_8x8();scan_interlace_ct6_d16_8x8();
    assert(r::raw(a_interlace_ct6_d16_8x8())==527nat) by (compute);
    assert(r::data(z_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_interlace_ct6_d16_8x8(),h_interlace_ct6_d16_8x8(),z_interlace_ct6_d16_8x8(),0nat,false);
}
fn public_interlace_ct6_d16_8x8() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,8u8,16u8,6u8,0u8,0u8,1u8,227u8,152u8,82u8,94u8,0u8,0u8,2u8,26u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,15u8,2u8,240u8,253u8,0u8,74u8,135u8,183u8,180u8,49u8,71u8,131u8,229u8,1u8,90u8,155u8,171u8,67u8,219u8,123u8,213u8,78u8,2u8,57u8,70u8,159u8,194u8,106u8,221u8,6u8,19u8,62u8,67u8,91u8,207u8,143u8,40u8,31u8,209u8,3u8,253u8,150u8,22u8,157u8,0u8,42u8,136u8,97u8,60u8,66u8,42u8,1u8,105u8,251u8,24u8,141u8,4u8,14u8,21u8,230u8,114u8,152u8,203u8,54u8,71u8,47u8,29u8,205u8,183u8,111u8,22u8,129u8,43u8,0u8,131u8,226u8,223u8,202u8,105u8,248u8,33u8,83u8,254u8,128u8,210u8,6u8,45u8,230u8,163u8,240u8,4u8,199u8,240u8,17u8,47u8,233u8,78u8,151u8,38u8,152u8,252u8,18u8,153u8,198u8,27u8,138u8,1u8,195u8,71u8,133u8,85u8,164u8,46u8,11u8,63u8,163u8,239u8,178u8,116u8,239u8,51u8,203u8,120u8,63u8,47u8,2u8,108u8,179u8,61u8,208u8,228u8,110u8,111u8,93u8,232u8,194u8,39u8,153u8,69u8,2u8,235u8,236u8,29u8,178u8,82u8,3u8,208u8,141u8,104u8,62u8,97u8,19u8,230u8,59u8,7u8,89u8,216u8,226u8,48u8,150u8,62u8,198u8,123u8,4u8,229u8,34u8,190u8,201u8,92u8,115u8,154u8,150u8,3u8,145u8,47u8,71u8,173u8,87u8,213u8,157u8,249u8,254u8,222u8,165u8,89u8,60u8,126u8,201u8,76u8,164u8,104u8,66u8,78u8,220u8,112u8,151u8,175u8,93u8,64u8,30u8,212u8,115u8,154u8,8u8,167u8,4u8,23u8,192u8,165u8,8u8,49u8,234u8,126u8,91u8,218u8,137u8,149u8,124u8,52u8,142u8,111u8,72u8,208u8,102u8,196u8,73u8,58u8,87u8,204u8,169u8,45u8,190u8,142u8,207u8,116u8,85u8,195u8,194u8,0u8,75u8,122u8,104u8,183u8,226u8,85u8,98u8,215u8,13u8,130u8,129u8,202u8,18u8,26u8,116u8,236u8,53u8,152u8,202u8,240u8,84u8,251u8,110u8,221u8,65u8,47u8,227u8,193u8,186u8,180u8,233u8,209u8,1u8,148u8,62u8,246u8,131u8,187u8,154u8,171u8,181u8,163u8,83u8,104u8,9u8,250u8,9u8,45u8,150u8,182u8,33u8,146u8,30u8,174u8,3u8,96u8,61u8,216u8,69u8,71u8,146u8,52u8,240u8,6u8,250u8,86u8,94u8,60u8,191u8,58u8,89u8,71u8,203u8,63u8,216u8,101u8,212u8,255u8,188u8,248u8,19u8,79u8,137u8,224u8,160u8,185u8,194u8,159u8,117u8,101u8,94u8,23u8,245u8,52u8,22u8,61u8,239u8,2u8,55u8,200u8,217u8,115u8,111u8,220u8,36u8,117u8,120u8,160u8,138u8,56u8,50u8,225u8,41u8,97u8,87u8,55u8,101u8,210u8,118u8,34u8,30u8,252u8,130u8,219u8,25u8,31u8,164u8,85u8,202u8,184u8,246u8,196u8,162u8,97u8,202u8,72u8,59u8,79u8,1u8,249u8,48u8,209u8,99u8,220u8,77u8,84u8,42u8,185u8,236u8,39u8,198u8,224u8,101u8,194u8,116u8,255u8,1u8,156u8,145u8,158u8,116u8,172u8,3u8,68u8,102u8,167u8,4u8,233u8,84u8,165u8,120u8,35u8,9u8,47u8,123u8,112u8,61u8,34u8,81u8,51u8,141u8,157u8,91u8,80u8,215u8,14u8,188u8,55u8,108u8,212u8,188u8,34u8,5u8,42u8,191u8,203u8,169u8,240u8,165u8,177u8,28u8,251u8,157u8,144u8,9u8,64u8,94u8,200u8,187u8,180u8,253u8,172u8,60u8,128u8,108u8,76u8,58u8,9u8,151u8,99u8,65u8,221u8,96u8,188u8,53u8,169u8,29u8,4u8,15u8,222u8,0u8,198u8,132u8,34u8,52u8,194u8,245u8,100u8,148u8,120u8,236u8,0u8,63u8,138u8,231u8,28u8,65u8,161u8,17u8,55u8,28u8,132u8,244u8,41u8,110u8,170u8,30u8,207u8,233u8,1u8,190u8,70u8,173u8,226u8,54u8,127u8,34u8,147u8,218u8,96u8,165u8,87u8,246u8,177u8,157u8,248u8,113u8,146u8,104u8,184u8,5u8,53u8,156u8,88u8,100u8,247u8,13u8,171u8,34u8,119u8,50u8,114u8,130u8,52u8,250u8,135u8,224u8,177u8,204u8,96u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_interlace_ct6_d16_8x8());model_interlace_ct6_d16_8x8();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==8u32 && s.height==8u32 && s.rgba@.len()==256nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// interlace/ct3-d4-8x8; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_interlace_ct3_d4_8x8() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,8u8,4u8,3u8,0u8,0u8,1u8,65u8,38u8,147u8,46u8,0u8,0u8,0u8,48u8,80u8,76u8,84u8,69u8,165u8,103u8,16u8,216u8,80u8,163u8,55u8,55u8,10u8,232u8,206u8,223u8,196u8,80u8,190u8,188u8,245u8,56u8,80u8,77u8,4u8,208u8,230u8,64u8,64u8,41u8,221u8,174u8,5u8,203u8,122u8,134u8,225u8,164u8,7u8,2u8,101u8,184u8,165u8,103u8,210u8,85u8,155u8,126u8,208u8,241u8,161u8,167u8,166u8,225u8,171u8,68u8,0u8,0u8,0u8,59u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,48u8,0u8,207u8,255u8,0u8,96u8,1u8,16u8,2u8,88u8,3u8,139u8,4u8,9u8,0u8,30u8,210u8,1u8,160u8,228u8,2u8,123u8,240u8,3u8,232u8,97u8,4u8,2u8,223u8,0u8,200u8,45u8,1u8,60u8,80u8,81u8,25u8,2u8,252u8,66u8,195u8,182u8,3u8,168u8,78u8,37u8,248u8,4u8,124u8,182u8,195u8,153u8,83u8,217u8,16u8,199u8,130u8,91u8,191u8,55u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_interlace_ct3_d4_8x8() -> Seq<u8> {seq![120u8,156u8,1u8,48u8,0u8,207u8,255u8,0u8,96u8,1u8,16u8,2u8,88u8,3u8,139u8,4u8,9u8,0u8,30u8,210u8,1u8,160u8,228u8,2u8,123u8,240u8,3u8,232u8,97u8,4u8,2u8,223u8,0u8,200u8,45u8,1u8,60u8,80u8,81u8,25u8,2u8,252u8,66u8,195u8,182u8,3u8,168u8,78u8,37u8,248u8,4u8,124u8,182u8,195u8,153u8,83u8,217u8,16u8,199u8]}
spec fn h_interlace_ct3_d4_8x8() -> crate::model::Ihdr {crate::model::Ihdr {width:8u32,height:8u32,depth:4u8,color_type:3u8,interlace:1u8}}
spec fn a_interlace_ct3_d4_8x8() -> r::Raster {r::shape(h_interlace_ct3_d4_8x8(),8nat,8nat,16nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_interlace_ct3_d4_8x8(b: Seq<u8>)
    requires b.len()==176,crate::model::mh(b)==Ok((crate::model::Ihdr {width:8u32,height:8u32,depth:4u8,color_type:3u8,interlace:1u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:59nat,next:164nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,164nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:172nat,len:0nat,next:176nat,crc_ok:true,synthetic:false})),
        b.subrange(101,160)==z_interlace_ct3_d4_8x8(),
    ensures r::default_case(b,crate::model::Ihdr {width:8u32,height:8u32,depth:4u8,color_type:3u8,interlace:1u8},z_interlace_ct3_d4_8x8(),16nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);
    hide(z_interlace_ct3_d4_8x8);
    r::palette_default_case(b,crate::model::Ihdr {width:8u32,height:8u32,depth:4u8,color_type:3u8,interlace:1u8},z_interlace_ct3_d4_8x8(),59nat);
}
spec fn raw_interlace_ct3_d4_8x8() -> Seq<u8> {seq![0u8,96u8,1u8,16u8,2u8,88u8,3u8,139u8,4u8,9u8,0u8,30u8,210u8,1u8,160u8,228u8,2u8,123u8,240u8,3u8,232u8,97u8,4u8,2u8,223u8,0u8,200u8,45u8,1u8,60u8,80u8,81u8,25u8,2u8,252u8,66u8,195u8,182u8,3u8,168u8,78u8,37u8,248u8,4u8,124u8,182u8,195u8,153u8]}
#[verifier::spinoff_prover]
proof fn shape_interlace_ct3_d4_8x8()
    ensures raw_interlace_ct3_d4_8x8().len()==48nat,
        z_interlace_ct3_d4_8x8().len()==59nat,
        z_interlace_ct3_d4_8x8()[0]==120u8,
        z_interlace_ct3_d4_8x8()[1]==156u8,
        zm::field_value(z_interlace_ct3_d4_8x8(),16,1)==1nat,
        zm::field_value(z_interlace_ct3_d4_8x8(),17,2)==0nat,
        zm::be_value(z_interlace_ct3_d4_8x8(),55nat,4)==1406734535nat,
        z_interlace_ct3_d4_8x8().subrange(7,55)==raw_interlace_ct3_d4_8x8(),
        z_interlace_ct3_d4_8x8()[3] as nat+256*z_interlace_ct3_d4_8x8()[4] as nat==48nat,
        z_interlace_ct3_d4_8x8()[5] as nat+256*z_interlace_ct3_d4_8x8()[6] as nat==u16::MAX as nat-48nat,
        zm::be_value(z_interlace_ct3_d4_8x8(),0,1)==120nat,
        zm::be_value(z_interlace_ct3_d4_8x8(),1,1)==156nat,
{
    hide(z_interlace_ct3_d4_8x8);hide(raw_interlace_ct3_d4_8x8);
    assert(raw_interlace_ct3_d4_8x8().len()==48nat) by (compute);
    assert(z_interlace_ct3_d4_8x8().len()==59nat) by (compute);
    assert(z_interlace_ct3_d4_8x8()[0]==120u8) by (compute);
    assert(z_interlace_ct3_d4_8x8()[1]==156u8) by (compute);
    assert(zm::field_value(z_interlace_ct3_d4_8x8(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_interlace_ct3_d4_8x8(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_interlace_ct3_d4_8x8(),55nat,4)==1406734535nat) by (compute);
    assert(z_interlace_ct3_d4_8x8().subrange(7,55)==raw_interlace_ct3_d4_8x8()) by (compute);
    assert(z_interlace_ct3_d4_8x8()[3] as nat+256*z_interlace_ct3_d4_8x8()[4] as nat==48nat) by (compute);
    assert(z_interlace_ct3_d4_8x8()[5] as nat+256*z_interlace_ct3_d4_8x8()[6] as nat==u16::MAX as nat-48nat) by (compute);
    assert(zm::be_value(z_interlace_ct3_d4_8x8(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_interlace_ct3_d4_8x8(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_interlace_ct3_d4_8x8()
    ensures zm::Adler(raw_interlace_ct3_d4_8x8())==1406734535nat,
{
    hide(raw_interlace_ct3_d4_8x8);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_interlace_ct3_d4_8x8().len()==48nat) by (compute);
    assert(c::sums(raw_interlace_ct3_d4_8x8(),0,48nat)==(4294nat,86986nat)) by (compute);
    c::sums_correct(raw_interlace_ct3_d4_8x8(),0,48nat);
    assert(raw_interlace_ct3_d4_8x8().subrange(0,48) =~= raw_interlace_ct3_d4_8x8());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((86986nat%65521)*(u16::MAX as nat+1)+(1+4294nat)%65521==1406734535nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_interlace_ct3_d4_8x8()
    ensures r::inflated(z_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_interlace_ct3_d4_8x8(),r2:false}),
        raw_interlace_ct3_d4_8x8().len()==48nat,
{
    hide(z_interlace_ct3_d4_8x8);hide(raw_interlace_ct3_d4_8x8);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_interlace_ct3_d4_8x8();checksum_interlace_ct3_d4_8x8();
    assert(r::raw(a_interlace_ct3_d4_8x8())==48nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_interlace_ct3_d4_8x8(),raw_interlace_ct3_d4_8x8(),48nat,120nat,156nat,1406734535nat);
}
#[verifier::spinoff_prover]
proof fn scan_interlace_ct3_d4_8x8()
    ensures r::scan(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),0)==Ok(()),
{
    hide(raw_interlace_ct3_d4_8x8);
    assert(r::count(a_interlace_ct3_d4_8x8())==7nat) by (compute);
    r::scan_step(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),7);
    assert(r::pw(a_interlace_ct3_d4_8x8(),6nat)==8nat && r::ph(a_interlace_ct3_d4_8x8(),6nat)==4nat && r::rb(a_interlace_ct3_d4_8x8(),6nat)==4nat && r::base(a_interlace_ct3_d4_8x8(),6nat)==28nat) by (compute);
    assert(raw_interlace_ct3_d4_8x8()[28]==1u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),6nat,0nat),8nat);
    assert(r::row_result(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),6nat,0nat)==Ok(()));
    assert(raw_interlace_ct3_d4_8x8()[33]==2u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),6nat,1nat),8nat);
    assert(r::row_result(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),6nat,1nat)==Ok(()));
    assert(raw_interlace_ct3_d4_8x8()[38]==3u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),6nat,2nat),8nat);
    assert(r::row_result(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),6nat,2nat)==Ok(()));
    assert(raw_interlace_ct3_d4_8x8()[43]==4u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),6nat,3nat),8nat);
    assert(r::row_result(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),6nat,3nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),6nat,4nat);
    r::pass_step(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),6nat,3nat);
    r::pass_step(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),6nat,2nat);
    r::pass_step(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),6nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),6nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),6nat);
    assert(r::pw(a_interlace_ct3_d4_8x8(),5nat)==4nat && r::ph(a_interlace_ct3_d4_8x8(),5nat)==4nat && r::rb(a_interlace_ct3_d4_8x8(),5nat)==2nat && r::base(a_interlace_ct3_d4_8x8(),5nat)==16nat) by (compute);
    assert(raw_interlace_ct3_d4_8x8()[16]==2u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),5nat,0nat),4nat);
    assert(r::row_result(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),5nat,0nat)==Ok(()));
    assert(raw_interlace_ct3_d4_8x8()[19]==3u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),5nat,1nat),4nat);
    assert(r::row_result(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),5nat,1nat)==Ok(()));
    assert(raw_interlace_ct3_d4_8x8()[22]==4u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),5nat,2nat),4nat);
    assert(r::row_result(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),5nat,2nat)==Ok(()));
    assert(raw_interlace_ct3_d4_8x8()[25]==0u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),5nat,3nat),4nat);
    assert(r::row_result(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),5nat,3nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),5nat,4nat);
    r::pass_step(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),5nat,3nat);
    r::pass_step(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),5nat,2nat);
    r::pass_step(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),5nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),5nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),5nat);
    assert(r::pw(a_interlace_ct3_d4_8x8(),4nat)==4nat && r::ph(a_interlace_ct3_d4_8x8(),4nat)==2nat && r::rb(a_interlace_ct3_d4_8x8(),4nat)==2nat && r::base(a_interlace_ct3_d4_8x8(),4nat)==10nat) by (compute);
    assert(raw_interlace_ct3_d4_8x8()[10]==0u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),4nat,0nat),4nat);
    assert(r::row_result(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),4nat,0nat)==Ok(()));
    assert(raw_interlace_ct3_d4_8x8()[13]==1u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),4nat,1nat),4nat);
    assert(r::row_result(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),4nat,1nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),4nat,2nat);
    r::pass_step(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),4nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),4nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),4nat);
    assert(r::pw(a_interlace_ct3_d4_8x8(),3nat)==2nat && r::ph(a_interlace_ct3_d4_8x8(),3nat)==2nat && r::rb(a_interlace_ct3_d4_8x8(),3nat)==1nat && r::base(a_interlace_ct3_d4_8x8(),3nat)==6nat) by (compute);
    assert(raw_interlace_ct3_d4_8x8()[6]==3u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),3nat,0nat),2nat);
    assert(r::row_result(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),3nat,0nat)==Ok(()));
    assert(raw_interlace_ct3_d4_8x8()[8]==4u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),3nat,1nat),2nat);
    assert(r::row_result(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),3nat,1nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),3nat,2nat);
    r::pass_step(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),3nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),3nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),3nat);
    assert(r::pw(a_interlace_ct3_d4_8x8(),2nat)==2nat && r::ph(a_interlace_ct3_d4_8x8(),2nat)==1nat && r::rb(a_interlace_ct3_d4_8x8(),2nat)==1nat && r::base(a_interlace_ct3_d4_8x8(),2nat)==4nat) by (compute);
    assert(raw_interlace_ct3_d4_8x8()[4]==2u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),2nat,0nat),2nat);
    assert(r::row_result(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),2nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),2nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),2nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),2nat);
    assert(r::pw(a_interlace_ct3_d4_8x8(),1nat)==1nat && r::ph(a_interlace_ct3_d4_8x8(),1nat)==1nat && r::rb(a_interlace_ct3_d4_8x8(),1nat)==1nat && r::base(a_interlace_ct3_d4_8x8(),1nat)==2nat) by (compute);
    assert(raw_interlace_ct3_d4_8x8()[2]==1u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),1nat,0nat),1nat);
    assert(r::row_result(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),1nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),1nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),1nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),1nat);
    assert(r::pw(a_interlace_ct3_d4_8x8(),0nat)==1nat && r::ph(a_interlace_ct3_d4_8x8(),0nat)==1nat && r::rb(a_interlace_ct3_d4_8x8(),0nat)==1nat && r::base(a_interlace_ct3_d4_8x8(),0nat)==0nat) by (compute);
    assert(raw_interlace_ct3_d4_8x8()[0]==0u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),0nat,0nat),1nat);
    assert(r::row_result(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),0nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),0nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),0nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_interlace_ct3_d4_8x8()
    ensures r::still(bytes_interlace_ct3_d4_8x8(),STRICT,false)==(r::Projection::Ready {width:8u32,height:8u32,bytes:256nat}),
{
    hide(bytes_interlace_ct3_d4_8x8);hide(z_interlace_ct3_d4_8x8);hide(raw_interlace_ct3_d4_8x8);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);
    assert(bytes_interlace_ct3_d4_8x8().len()==176nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct3_d4_8x8())==Ok((crate::model::Ihdr {width:8u32,height:8u32,depth:4u8,color_type:3u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct3_d4_8x8())==Ok((crate::model::Ihdr {width:8u32,height:8u32,depth:4u8,color_type:3u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 4u8)==16u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_8x8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_8x8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(48nat,3u8,4u8,false,false,false,false)==crate::metadata_model::Decision::Keep(16nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_8x8(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:59nat,next:164nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_8x8(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:59nat,next:164nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_8x8(),164nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:172nat,len:0nat,next:176nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_8x8(),164nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:172nat,len:0nat,next:176nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct3_d4_8x8().subrange(101,160)==z_interlace_ct3_d4_8x8()) by (compute);
    walk_interlace_ct3_d4_8x8(bytes_interlace_ct3_d4_8x8());
    inflate_interlace_ct3_d4_8x8();scan_interlace_ct3_d4_8x8();
    assert(r::raw(a_interlace_ct3_d4_8x8())==48nat) by (compute);
    assert(r::data(z_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_interlace_ct3_d4_8x8(),h_interlace_ct3_d4_8x8(),z_interlace_ct3_d4_8x8(),16nat,false);
}
fn public_interlace_ct3_d4_8x8() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,8u8,0u8,0u8,0u8,8u8,4u8,3u8,0u8,0u8,1u8,65u8,38u8,147u8,46u8,0u8,0u8,0u8,48u8,80u8,76u8,84u8,69u8,165u8,103u8,16u8,216u8,80u8,163u8,55u8,55u8,10u8,232u8,206u8,223u8,196u8,80u8,190u8,188u8,245u8,56u8,80u8,77u8,4u8,208u8,230u8,64u8,64u8,41u8,221u8,174u8,5u8,203u8,122u8,134u8,225u8,164u8,7u8,2u8,101u8,184u8,165u8,103u8,210u8,85u8,155u8,126u8,208u8,241u8,161u8,167u8,166u8,225u8,171u8,68u8,0u8,0u8,0u8,59u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,48u8,0u8,207u8,255u8,0u8,96u8,1u8,16u8,2u8,88u8,3u8,139u8,4u8,9u8,0u8,30u8,210u8,1u8,160u8,228u8,2u8,123u8,240u8,3u8,232u8,97u8,4u8,2u8,223u8,0u8,200u8,45u8,1u8,60u8,80u8,81u8,25u8,2u8,252u8,66u8,195u8,182u8,3u8,168u8,78u8,37u8,248u8,4u8,124u8,182u8,195u8,153u8,83u8,217u8,16u8,199u8,130u8,91u8,191u8,55u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_interlace_ct3_d4_8x8());model_interlace_ct3_d4_8x8();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==8u32 && s.height==8u32 && s.rgba@.len()==256nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// interlace/ct0-d1-9x1; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_interlace_ct0_d1_9x1() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,9u8,0u8,0u8,0u8,1u8,1u8,0u8,0u8,0u8,1u8,83u8,190u8,137u8,70u8,0u8,0u8,0u8,16u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,96u8,108u8,96u8,58u8,192u8,108u8,0u8,0u8,5u8,12u8,1u8,119u8,208u8,113u8,43u8,109u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_interlace_ct0_d1_9x1() -> Seq<u8> {seq![120u8,156u8,99u8,96u8,96u8,108u8,96u8,58u8,192u8,108u8,0u8,0u8,5u8,12u8,1u8,119u8]}
spec fn h_interlace_ct0_d1_9x1() -> crate::model::Ihdr {crate::model::Ihdr {width:9u32,height:1u32,depth:1u8,color_type:0u8,interlace:1u8}}
spec fn a_interlace_ct0_d1_9x1() -> r::Raster {r::shape(h_interlace_ct0_d1_9x1(),9nat,1nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_interlace_ct0_d1_9x1(b: Seq<u8>)
    requires b.len()==73,crate::model::mh(b)==Ok((crate::model::Ihdr {width:9u32,height:1u32,depth:1u8,color_type:0u8,interlace:1u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:16nat,next:61nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,61nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:69nat,len:0nat,next:73nat,crc_ok:true,synthetic:false})),
        b.subrange(41,57)==z_interlace_ct0_d1_9x1(),
    ensures r::default_case(b,crate::model::Ihdr {width:9u32,height:1u32,depth:1u8,color_type:0u8,interlace:1u8},z_interlace_ct0_d1_9x1(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    hide(z_interlace_ct0_d1_9x1);
        assert((1u32 << 1u8)==2u32) by (compute);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_interlace_ct0_d1_9x1() -> Seq<u8> {seq![0u8,0u8,1u8,128u8,2u8,192u8,3u8,48u8]}
spec fn ts_interlace_ct0_d1_9x1() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:35nat,before:1nat,after:2nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:35nat,next:43nat,before:2nat,after:3nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:43nat,next:51nat,before:3nat,after:4nat,symbol:128nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:51nat,next:59nat,before:4nat,after:5nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:59nat,next:68nat,before:5nat,after:6nat,symbol:192nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:68nat,next:76nat,before:6nat,after:7nat,symbol:3nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:76nat,next:84nat,before:7nat,after:8nat,symbol:48nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:84nat,next:91nat,before:8nat,after:8nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_interlace_ct0_d1_9x1()
    ensures t::trace(z_interlace_ct0_d1_9x1(),raw_interlace_ct0_d1_9x1(),ts_interlace_ct0_d1_9x1(),0,9nat),
{
    hide(z_interlace_ct0_d1_9x1);hide(raw_interlace_ct0_d1_9x1);hide(ts_interlace_ct0_d1_9x1);hide(t::valid_step);
    assert(t::trace(z_interlace_ct0_d1_9x1(),raw_interlace_ct0_d1_9x1(),ts_interlace_ct0_d1_9x1(),0,9nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_interlace_ct0_d1_9x1()
    ensures raw_interlace_ct0_d1_9x1().len()==8nat,
        z_interlace_ct0_d1_9x1().len()==16nat,
        z_interlace_ct0_d1_9x1()[0]==120u8,
        z_interlace_ct0_d1_9x1()[1]==156u8,
        zm::field_value(z_interlace_ct0_d1_9x1(),16,1)==1nat,
        zm::field_value(z_interlace_ct0_d1_9x1(),17,2)==1nat,
        zm::be_value(z_interlace_ct0_d1_9x1(),12nat,4)==84672887nat,
        ts_interlace_ct0_d1_9x1().len()==9nat,
        ts_interlace_ct0_d1_9x1()[0].pos==19nat,
        ts_interlace_ct0_d1_9x1()[0].before==0nat,
        ts_interlace_ct0_d1_9x1().last().next==91nat,
{
    hide(z_interlace_ct0_d1_9x1);hide(raw_interlace_ct0_d1_9x1);hide(ts_interlace_ct0_d1_9x1);
    assert(raw_interlace_ct0_d1_9x1().len()==8nat) by (compute);
    assert(z_interlace_ct0_d1_9x1().len()==16nat) by (compute);
    assert(z_interlace_ct0_d1_9x1()[0]==120u8) by (compute);
    assert(z_interlace_ct0_d1_9x1()[1]==156u8) by (compute);
    assert(zm::field_value(z_interlace_ct0_d1_9x1(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_interlace_ct0_d1_9x1(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_interlace_ct0_d1_9x1(),12nat,4)==84672887nat) by (compute);
    assert(ts_interlace_ct0_d1_9x1().len()==9nat) by (compute);
    assert(ts_interlace_ct0_d1_9x1()[0].pos==19nat) by (compute);
    assert(ts_interlace_ct0_d1_9x1()[0].before==0nat) by (compute);
    assert(ts_interlace_ct0_d1_9x1().last().next==91nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_interlace_ct0_d1_9x1()
    ensures zm::Adler(raw_interlace_ct0_d1_9x1())==84672887nat,
{
    hide(raw_interlace_ct0_d1_9x1);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_interlace_ct0_d1_9x1().len()==8nat) by (compute);
    assert(c::sums(raw_interlace_ct0_d1_9x1(),0,8nat)==(374nat,1292nat)) by (compute);
    c::sums_correct(raw_interlace_ct0_d1_9x1(),0,8nat);
    assert(raw_interlace_ct0_d1_9x1().subrange(0,8) =~= raw_interlace_ct0_d1_9x1());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((1292nat%65521)*(u16::MAX as nat+1)+(1+374nat)%65521==84672887nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_interlace_ct0_d1_9x1()
    ensures r::inflated(z_interlace_ct0_d1_9x1(),a_interlace_ct0_d1_9x1(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_interlace_ct0_d1_9x1(),r2:false}),
        raw_interlace_ct0_d1_9x1().len()==8nat,
{
    hide(z_interlace_ct0_d1_9x1);hide(raw_interlace_ct0_d1_9x1);hide(ts_interlace_ct0_d1_9x1);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_interlace_ct0_d1_9x1();checksum_interlace_ct0_d1_9x1();
    assert(r::raw(a_interlace_ct0_d1_9x1())==8nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_interlace_ct0_d1_9x1();
    stream::fixed_stream(z_interlace_ct0_d1_9x1(),raw_interlace_ct0_d1_9x1(),ts_interlace_ct0_d1_9x1(),8nat,156nat,84672887nat);
}
#[verifier::spinoff_prover]
proof fn scan_interlace_ct0_d1_9x1()
    ensures r::scan(raw_interlace_ct0_d1_9x1(),a_interlace_ct0_d1_9x1(),0)==Ok(()),
{
    hide(raw_interlace_ct0_d1_9x1);
    assert(r::count(a_interlace_ct0_d1_9x1())==7nat) by (compute);
    r::scan_step(raw_interlace_ct0_d1_9x1(),a_interlace_ct0_d1_9x1(),7);
    assert(r::pw(a_interlace_ct0_d1_9x1(),6nat)==9nat && r::ph(a_interlace_ct0_d1_9x1(),6nat)==0nat && r::rb(a_interlace_ct0_d1_9x1(),6nat)==2nat && r::base(a_interlace_ct0_d1_9x1(),6nat)==8nat) by (compute);
    r::pass_step(raw_interlace_ct0_d1_9x1(),a_interlace_ct0_d1_9x1(),6nat,0);
    r::scan_step(raw_interlace_ct0_d1_9x1(),a_interlace_ct0_d1_9x1(),6nat);
    assert(r::pw(a_interlace_ct0_d1_9x1(),5nat)==4nat && r::ph(a_interlace_ct0_d1_9x1(),5nat)==1nat && r::rb(a_interlace_ct0_d1_9x1(),5nat)==1nat && r::base(a_interlace_ct0_d1_9x1(),5nat)==6nat) by (compute);
    assert(raw_interlace_ct0_d1_9x1()[6]==3u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_9x1(),a_interlace_ct0_d1_9x1(),5nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_9x1(),a_interlace_ct0_d1_9x1(),5nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_9x1(),a_interlace_ct0_d1_9x1(),5nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_9x1(),a_interlace_ct0_d1_9x1(),5nat);
    assert(r::pw(a_interlace_ct0_d1_9x1(),4nat)==5nat && r::ph(a_interlace_ct0_d1_9x1(),4nat)==0nat && r::rb(a_interlace_ct0_d1_9x1(),4nat)==1nat && r::base(a_interlace_ct0_d1_9x1(),4nat)==6nat) by (compute);
    r::pass_step(raw_interlace_ct0_d1_9x1(),a_interlace_ct0_d1_9x1(),4nat,0);
    r::scan_step(raw_interlace_ct0_d1_9x1(),a_interlace_ct0_d1_9x1(),4nat);
    assert(r::pw(a_interlace_ct0_d1_9x1(),3nat)==2nat && r::ph(a_interlace_ct0_d1_9x1(),3nat)==1nat && r::rb(a_interlace_ct0_d1_9x1(),3nat)==1nat && r::base(a_interlace_ct0_d1_9x1(),3nat)==4nat) by (compute);
    assert(raw_interlace_ct0_d1_9x1()[4]==2u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_9x1(),a_interlace_ct0_d1_9x1(),3nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_9x1(),a_interlace_ct0_d1_9x1(),3nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_9x1(),a_interlace_ct0_d1_9x1(),3nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_9x1(),a_interlace_ct0_d1_9x1(),3nat);
    assert(r::pw(a_interlace_ct0_d1_9x1(),2nat)==3nat && r::ph(a_interlace_ct0_d1_9x1(),2nat)==0nat && r::rb(a_interlace_ct0_d1_9x1(),2nat)==1nat && r::base(a_interlace_ct0_d1_9x1(),2nat)==4nat) by (compute);
    r::pass_step(raw_interlace_ct0_d1_9x1(),a_interlace_ct0_d1_9x1(),2nat,0);
    r::scan_step(raw_interlace_ct0_d1_9x1(),a_interlace_ct0_d1_9x1(),2nat);
    assert(r::pw(a_interlace_ct0_d1_9x1(),1nat)==1nat && r::ph(a_interlace_ct0_d1_9x1(),1nat)==1nat && r::rb(a_interlace_ct0_d1_9x1(),1nat)==1nat && r::base(a_interlace_ct0_d1_9x1(),1nat)==2nat) by (compute);
    assert(raw_interlace_ct0_d1_9x1()[2]==1u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_9x1(),a_interlace_ct0_d1_9x1(),1nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_9x1(),a_interlace_ct0_d1_9x1(),1nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_9x1(),a_interlace_ct0_d1_9x1(),1nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_9x1(),a_interlace_ct0_d1_9x1(),1nat);
    assert(r::pw(a_interlace_ct0_d1_9x1(),0nat)==2nat && r::ph(a_interlace_ct0_d1_9x1(),0nat)==1nat && r::rb(a_interlace_ct0_d1_9x1(),0nat)==1nat && r::base(a_interlace_ct0_d1_9x1(),0nat)==0nat) by (compute);
    assert(raw_interlace_ct0_d1_9x1()[0]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_9x1(),a_interlace_ct0_d1_9x1(),0nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_9x1(),a_interlace_ct0_d1_9x1(),0nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_9x1(),a_interlace_ct0_d1_9x1(),0nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_9x1(),a_interlace_ct0_d1_9x1(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_interlace_ct0_d1_9x1()
    ensures r::still(bytes_interlace_ct0_d1_9x1(),STRICT,false)==(r::Projection::Ready {width:9u32,height:1u32,bytes:36nat}),
{
    hide(bytes_interlace_ct0_d1_9x1);hide(z_interlace_ct0_d1_9x1);hide(raw_interlace_ct0_d1_9x1);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_interlace_ct0_d1_9x1().len()==73nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct0_d1_9x1())==Ok((crate::model::Ihdr {width:9u32,height:1u32,depth:1u8,color_type:0u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct0_d1_9x1())==Ok((crate::model::Ihdr {width:9u32,height:1u32,depth:1u8,color_type:0u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 1u8)==2u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct0_d1_9x1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:16nat,next:61nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct0_d1_9x1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:16nat,next:61nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct0_d1_9x1(),61nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:69nat,len:0nat,next:73nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct0_d1_9x1(),61nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:69nat,len:0nat,next:73nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct0_d1_9x1().subrange(41,57)==z_interlace_ct0_d1_9x1()) by (compute);
    walk_interlace_ct0_d1_9x1(bytes_interlace_ct0_d1_9x1());
    inflate_interlace_ct0_d1_9x1();scan_interlace_ct0_d1_9x1();
    assert(r::raw(a_interlace_ct0_d1_9x1())==8nat) by (compute);
    assert(r::data(z_interlace_ct0_d1_9x1(),a_interlace_ct0_d1_9x1(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_interlace_ct0_d1_9x1(),h_interlace_ct0_d1_9x1(),z_interlace_ct0_d1_9x1(),0nat,false);
}
fn public_interlace_ct0_d1_9x1() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,9u8,0u8,0u8,0u8,1u8,1u8,0u8,0u8,0u8,1u8,83u8,190u8,137u8,70u8,0u8,0u8,0u8,16u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,96u8,108u8,96u8,58u8,192u8,108u8,0u8,0u8,5u8,12u8,1u8,119u8,208u8,113u8,43u8,109u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_interlace_ct0_d1_9x1());model_interlace_ct0_d1_9x1();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==9u32 && s.height==1u32 && s.rgba@.len()==36nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// interlace/ct6-d16-9x1; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_interlace_ct6_d16_9x1() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,9u8,0u8,0u8,0u8,1u8,16u8,6u8,0u8,0u8,1u8,43u8,85u8,104u8,168u8,0u8,0u8,0u8,87u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,76u8,0u8,179u8,255u8,0u8,42u8,81u8,252u8,161u8,212u8,199u8,150u8,2u8,108u8,45u8,247u8,237u8,77u8,199u8,103u8,74u8,1u8,72u8,199u8,68u8,202u8,47u8,121u8,202u8,221u8,2u8,151u8,117u8,81u8,133u8,113u8,95u8,231u8,227u8,39u8,125u8,205u8,115u8,122u8,137u8,59u8,139u8,3u8,75u8,150u8,24u8,50u8,24u8,113u8,105u8,128u8,57u8,31u8,5u8,1u8,250u8,237u8,168u8,88u8,135u8,202u8,11u8,197u8,59u8,69u8,218u8,163u8,102u8,249u8,204u8,152u8,48u8,169u8,223u8,150u8,88u8,143u8,36u8,58u8,135u8,85u8,108u8,126u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_interlace_ct6_d16_9x1() -> Seq<u8> {seq![120u8,156u8,1u8,76u8,0u8,179u8,255u8,0u8,42u8,81u8,252u8,161u8,212u8,199u8,150u8,2u8,108u8,45u8,247u8,237u8,77u8,199u8,103u8,74u8,1u8,72u8,199u8,68u8,202u8,47u8,121u8,202u8,221u8,2u8,151u8,117u8,81u8,133u8,113u8,95u8,231u8,227u8,39u8,125u8,205u8,115u8,122u8,137u8,59u8,139u8,3u8,75u8,150u8,24u8,50u8,24u8,113u8,105u8,128u8,57u8,31u8,5u8,1u8,250u8,237u8,168u8,88u8,135u8,202u8,11u8,197u8,59u8,69u8,218u8,163u8,102u8,249u8,204u8,152u8,48u8,169u8,223u8,150u8,88u8,143u8,36u8,58u8]}
spec fn h_interlace_ct6_d16_9x1() -> crate::model::Ihdr {crate::model::Ihdr {width:9u32,height:1u32,depth:16u8,color_type:6u8,interlace:1u8}}
spec fn a_interlace_ct6_d16_9x1() -> r::Raster {r::shape(h_interlace_ct6_d16_9x1(),9nat,1nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_interlace_ct6_d16_9x1(b: Seq<u8>)
    requires b.len()==144,crate::model::mh(b)==Ok((crate::model::Ihdr {width:9u32,height:1u32,depth:16u8,color_type:6u8,interlace:1u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:87nat,next:132nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,132nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:140nat,len:0nat,next:144nat,crc_ok:true,synthetic:false})),
        b.subrange(41,128)==z_interlace_ct6_d16_9x1(),
    ensures r::default_case(b,crate::model::Ihdr {width:9u32,height:1u32,depth:16u8,color_type:6u8,interlace:1u8},z_interlace_ct6_d16_9x1(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    hide(z_interlace_ct6_d16_9x1);
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_interlace_ct6_d16_9x1() -> Seq<u8> {seq![0u8,42u8,81u8,252u8,161u8,212u8,199u8,150u8,2u8,108u8,45u8,247u8,237u8,77u8,199u8,103u8,74u8,1u8,72u8,199u8,68u8,202u8,47u8,121u8,202u8,221u8,2u8,151u8,117u8,81u8,133u8,113u8,95u8,231u8,227u8,39u8,125u8,205u8,115u8,122u8,137u8,59u8,139u8,3u8,75u8,150u8,24u8,50u8,24u8,113u8,105u8,128u8,57u8,31u8,5u8,1u8,250u8,237u8,168u8,88u8,135u8,202u8,11u8,197u8,59u8,69u8,218u8,163u8,102u8,249u8,204u8,152u8,48u8,169u8,223u8,150u8]}
#[verifier::spinoff_prover]
proof fn shape_interlace_ct6_d16_9x1()
    ensures raw_interlace_ct6_d16_9x1().len()==76nat,
        z_interlace_ct6_d16_9x1().len()==87nat,
        z_interlace_ct6_d16_9x1()[0]==120u8,
        z_interlace_ct6_d16_9x1()[1]==156u8,
        zm::field_value(z_interlace_ct6_d16_9x1(),16,1)==1nat,
        zm::field_value(z_interlace_ct6_d16_9x1(),17,2)==0nat,
        zm::be_value(z_interlace_ct6_d16_9x1(),83nat,4)==1485775930nat,
        z_interlace_ct6_d16_9x1().subrange(7,83)==raw_interlace_ct6_d16_9x1(),
        z_interlace_ct6_d16_9x1()[3] as nat+256*z_interlace_ct6_d16_9x1()[4] as nat==76nat,
        z_interlace_ct6_d16_9x1()[5] as nat+256*z_interlace_ct6_d16_9x1()[6] as nat==u16::MAX as nat-76nat,
        zm::be_value(z_interlace_ct6_d16_9x1(),0,1)==120nat,
        zm::be_value(z_interlace_ct6_d16_9x1(),1,1)==156nat,
{
    hide(z_interlace_ct6_d16_9x1);hide(raw_interlace_ct6_d16_9x1);
    assert(raw_interlace_ct6_d16_9x1().len()==76nat) by (compute);
    assert(z_interlace_ct6_d16_9x1().len()==87nat) by (compute);
    assert(z_interlace_ct6_d16_9x1()[0]==120u8) by (compute);
    assert(z_interlace_ct6_d16_9x1()[1]==156u8) by (compute);
    assert(zm::field_value(z_interlace_ct6_d16_9x1(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_interlace_ct6_d16_9x1(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_interlace_ct6_d16_9x1(),83nat,4)==1485775930nat) by (compute);
    assert(z_interlace_ct6_d16_9x1().subrange(7,83)==raw_interlace_ct6_d16_9x1()) by (compute);
    assert(z_interlace_ct6_d16_9x1()[3] as nat+256*z_interlace_ct6_d16_9x1()[4] as nat==76nat) by (compute);
    assert(z_interlace_ct6_d16_9x1()[5] as nat+256*z_interlace_ct6_d16_9x1()[6] as nat==u16::MAX as nat-76nat) by (compute);
    assert(zm::be_value(z_interlace_ct6_d16_9x1(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_interlace_ct6_d16_9x1(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_interlace_ct6_d16_9x1()
    ensures zm::Adler(raw_interlace_ct6_d16_9x1())==1485775930nat,
{
    hide(raw_interlace_ct6_d16_9x1);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_interlace_ct6_d16_9x1().len()==76nat) by (compute);
    assert(c::sums(raw_interlace_ct6_d16_9x1(),0,76nat)==(9273nat,350276nat)) by (compute);
    c::sums_correct(raw_interlace_ct6_d16_9x1(),0,76nat);
    assert(raw_interlace_ct6_d16_9x1().subrange(0,76) =~= raw_interlace_ct6_d16_9x1());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((350276nat%65521)*(u16::MAX as nat+1)+(1+9273nat)%65521==1485775930nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_interlace_ct6_d16_9x1()
    ensures r::inflated(z_interlace_ct6_d16_9x1(),a_interlace_ct6_d16_9x1(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_interlace_ct6_d16_9x1(),r2:false}),
        raw_interlace_ct6_d16_9x1().len()==76nat,
{
    hide(z_interlace_ct6_d16_9x1);hide(raw_interlace_ct6_d16_9x1);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_interlace_ct6_d16_9x1();checksum_interlace_ct6_d16_9x1();
    assert(r::raw(a_interlace_ct6_d16_9x1())==76nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_interlace_ct6_d16_9x1(),raw_interlace_ct6_d16_9x1(),76nat,120nat,156nat,1485775930nat);
}
#[verifier::spinoff_prover]
proof fn scan_interlace_ct6_d16_9x1()
    ensures r::scan(raw_interlace_ct6_d16_9x1(),a_interlace_ct6_d16_9x1(),0)==Ok(()),
{
    hide(raw_interlace_ct6_d16_9x1);
    assert(r::count(a_interlace_ct6_d16_9x1())==7nat) by (compute);
    r::scan_step(raw_interlace_ct6_d16_9x1(),a_interlace_ct6_d16_9x1(),7);
    assert(r::pw(a_interlace_ct6_d16_9x1(),6nat)==9nat && r::ph(a_interlace_ct6_d16_9x1(),6nat)==0nat && r::rb(a_interlace_ct6_d16_9x1(),6nat)==72nat && r::base(a_interlace_ct6_d16_9x1(),6nat)==76nat) by (compute);
    r::pass_step(raw_interlace_ct6_d16_9x1(),a_interlace_ct6_d16_9x1(),6nat,0);
    r::scan_step(raw_interlace_ct6_d16_9x1(),a_interlace_ct6_d16_9x1(),6nat);
    assert(r::pw(a_interlace_ct6_d16_9x1(),5nat)==4nat && r::ph(a_interlace_ct6_d16_9x1(),5nat)==1nat && r::rb(a_interlace_ct6_d16_9x1(),5nat)==32nat && r::base(a_interlace_ct6_d16_9x1(),5nat)==43nat) by (compute);
    assert(raw_interlace_ct6_d16_9x1()[43]==3u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_9x1(),a_interlace_ct6_d16_9x1(),5nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_9x1(),a_interlace_ct6_d16_9x1(),5nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_9x1(),a_interlace_ct6_d16_9x1(),5nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_9x1(),a_interlace_ct6_d16_9x1(),5nat);
    assert(r::pw(a_interlace_ct6_d16_9x1(),4nat)==5nat && r::ph(a_interlace_ct6_d16_9x1(),4nat)==0nat && r::rb(a_interlace_ct6_d16_9x1(),4nat)==40nat && r::base(a_interlace_ct6_d16_9x1(),4nat)==43nat) by (compute);
    r::pass_step(raw_interlace_ct6_d16_9x1(),a_interlace_ct6_d16_9x1(),4nat,0);
    r::scan_step(raw_interlace_ct6_d16_9x1(),a_interlace_ct6_d16_9x1(),4nat);
    assert(r::pw(a_interlace_ct6_d16_9x1(),3nat)==2nat && r::ph(a_interlace_ct6_d16_9x1(),3nat)==1nat && r::rb(a_interlace_ct6_d16_9x1(),3nat)==16nat && r::base(a_interlace_ct6_d16_9x1(),3nat)==26nat) by (compute);
    assert(raw_interlace_ct6_d16_9x1()[26]==2u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_9x1(),a_interlace_ct6_d16_9x1(),3nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_9x1(),a_interlace_ct6_d16_9x1(),3nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_9x1(),a_interlace_ct6_d16_9x1(),3nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_9x1(),a_interlace_ct6_d16_9x1(),3nat);
    assert(r::pw(a_interlace_ct6_d16_9x1(),2nat)==3nat && r::ph(a_interlace_ct6_d16_9x1(),2nat)==0nat && r::rb(a_interlace_ct6_d16_9x1(),2nat)==24nat && r::base(a_interlace_ct6_d16_9x1(),2nat)==26nat) by (compute);
    r::pass_step(raw_interlace_ct6_d16_9x1(),a_interlace_ct6_d16_9x1(),2nat,0);
    r::scan_step(raw_interlace_ct6_d16_9x1(),a_interlace_ct6_d16_9x1(),2nat);
    assert(r::pw(a_interlace_ct6_d16_9x1(),1nat)==1nat && r::ph(a_interlace_ct6_d16_9x1(),1nat)==1nat && r::rb(a_interlace_ct6_d16_9x1(),1nat)==8nat && r::base(a_interlace_ct6_d16_9x1(),1nat)==17nat) by (compute);
    assert(raw_interlace_ct6_d16_9x1()[17]==1u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_9x1(),a_interlace_ct6_d16_9x1(),1nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_9x1(),a_interlace_ct6_d16_9x1(),1nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_9x1(),a_interlace_ct6_d16_9x1(),1nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_9x1(),a_interlace_ct6_d16_9x1(),1nat);
    assert(r::pw(a_interlace_ct6_d16_9x1(),0nat)==2nat && r::ph(a_interlace_ct6_d16_9x1(),0nat)==1nat && r::rb(a_interlace_ct6_d16_9x1(),0nat)==16nat && r::base(a_interlace_ct6_d16_9x1(),0nat)==0nat) by (compute);
    assert(raw_interlace_ct6_d16_9x1()[0]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_9x1(),a_interlace_ct6_d16_9x1(),0nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_9x1(),a_interlace_ct6_d16_9x1(),0nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_9x1(),a_interlace_ct6_d16_9x1(),0nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_9x1(),a_interlace_ct6_d16_9x1(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_interlace_ct6_d16_9x1()
    ensures r::still(bytes_interlace_ct6_d16_9x1(),STRICT,false)==(r::Projection::Ready {width:9u32,height:1u32,bytes:36nat}),
{
    hide(bytes_interlace_ct6_d16_9x1);hide(z_interlace_ct6_d16_9x1);hide(raw_interlace_ct6_d16_9x1);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_interlace_ct6_d16_9x1().len()==144nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct6_d16_9x1())==Ok((crate::model::Ihdr {width:9u32,height:1u32,depth:16u8,color_type:6u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct6_d16_9x1())==Ok((crate::model::Ihdr {width:9u32,height:1u32,depth:16u8,color_type:6u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct6_d16_9x1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:87nat,next:132nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct6_d16_9x1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:87nat,next:132nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct6_d16_9x1(),132nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:140nat,len:0nat,next:144nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct6_d16_9x1(),132nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:140nat,len:0nat,next:144nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct6_d16_9x1().subrange(41,128)==z_interlace_ct6_d16_9x1()) by (compute);
    walk_interlace_ct6_d16_9x1(bytes_interlace_ct6_d16_9x1());
    inflate_interlace_ct6_d16_9x1();scan_interlace_ct6_d16_9x1();
    assert(r::raw(a_interlace_ct6_d16_9x1())==76nat) by (compute);
    assert(r::data(z_interlace_ct6_d16_9x1(),a_interlace_ct6_d16_9x1(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_interlace_ct6_d16_9x1(),h_interlace_ct6_d16_9x1(),z_interlace_ct6_d16_9x1(),0nat,false);
}
fn public_interlace_ct6_d16_9x1() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,9u8,0u8,0u8,0u8,1u8,16u8,6u8,0u8,0u8,1u8,43u8,85u8,104u8,168u8,0u8,0u8,0u8,87u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,76u8,0u8,179u8,255u8,0u8,42u8,81u8,252u8,161u8,212u8,199u8,150u8,2u8,108u8,45u8,247u8,237u8,77u8,199u8,103u8,74u8,1u8,72u8,199u8,68u8,202u8,47u8,121u8,202u8,221u8,2u8,151u8,117u8,81u8,133u8,113u8,95u8,231u8,227u8,39u8,125u8,205u8,115u8,122u8,137u8,59u8,139u8,3u8,75u8,150u8,24u8,50u8,24u8,113u8,105u8,128u8,57u8,31u8,5u8,1u8,250u8,237u8,168u8,88u8,135u8,202u8,11u8,197u8,59u8,69u8,218u8,163u8,102u8,249u8,204u8,152u8,48u8,169u8,223u8,150u8,88u8,143u8,36u8,58u8,135u8,85u8,108u8,126u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_interlace_ct6_d16_9x1());model_interlace_ct6_d16_9x1();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==9u32 && s.height==1u32 && s.rgba@.len()==36nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// interlace/ct3-d4-9x1; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_interlace_ct3_d4_9x1() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,9u8,0u8,0u8,0u8,1u8,4u8,3u8,0u8,0u8,1u8,137u8,235u8,169u8,216u8,0u8,0u8,0u8,48u8,80u8,76u8,84u8,69u8,6u8,57u8,192u8,47u8,48u8,39u8,94u8,5u8,197u8,195u8,225u8,116u8,241u8,0u8,255u8,230u8,223u8,121u8,71u8,172u8,53u8,157u8,158u8,239u8,191u8,144u8,220u8,242u8,167u8,59u8,86u8,140u8,145u8,106u8,233u8,230u8,23u8,94u8,171u8,34u8,31u8,171u8,169u8,32u8,126u8,137u8,211u8,213u8,37u8,219u8,116u8,217u8,0u8,0u8,0u8,17u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,40u8,98u8,84u8,96u8,106u8,100u8,190u8,150u8,12u8,0u8,8u8,134u8,2u8,83u8,93u8,36u8,160u8,94u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_interlace_ct3_d4_9x1() -> Seq<u8> {seq![120u8,156u8,99u8,40u8,98u8,84u8,96u8,106u8,100u8,190u8,150u8,12u8,0u8,8u8,134u8,2u8,83u8]}
spec fn h_interlace_ct3_d4_9x1() -> crate::model::Ihdr {crate::model::Ihdr {width:9u32,height:1u32,depth:4u8,color_type:3u8,interlace:1u8}}
spec fn a_interlace_ct3_d4_9x1() -> r::Raster {r::shape(h_interlace_ct3_d4_9x1(),9nat,1nat,16nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_interlace_ct3_d4_9x1(b: Seq<u8>)
    requires b.len()==134,crate::model::mh(b)==Ok((crate::model::Ihdr {width:9u32,height:1u32,depth:4u8,color_type:3u8,interlace:1u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:17nat,next:122nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,122nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:130nat,len:0nat,next:134nat,crc_ok:true,synthetic:false})),
        b.subrange(101,118)==z_interlace_ct3_d4_9x1(),
    ensures r::default_case(b,crate::model::Ihdr {width:9u32,height:1u32,depth:4u8,color_type:3u8,interlace:1u8},z_interlace_ct3_d4_9x1(),16nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);
    hide(z_interlace_ct3_d4_9x1);
    r::palette_default_case(b,crate::model::Ihdr {width:9u32,height:1u32,depth:4u8,color_type:3u8,interlace:1u8},z_interlace_ct3_d4_9x1(),17nat);
}
spec fn raw_interlace_ct3_d4_9x1() -> Seq<u8> {seq![0u8,114u8,1u8,32u8,2u8,129u8,3u8,214u8,99u8]}
spec fn ts_interlace_ct3_d4_9x1() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:35nat,before:1nat,after:2nat,symbol:114nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:35nat,next:43nat,before:2nat,after:3nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:43nat,next:51nat,before:3nat,after:4nat,symbol:32nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:51nat,next:59nat,before:4nat,after:5nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:59nat,next:67nat,before:5nat,after:6nat,symbol:129nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:67nat,next:75nat,before:6nat,after:7nat,symbol:3nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:75nat,next:84nat,before:7nat,after:8nat,symbol:214nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:84nat,next:92nat,before:8nat,after:9nat,symbol:99nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:92nat,next:99nat,before:9nat,after:9nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_interlace_ct3_d4_9x1()
    ensures t::trace(z_interlace_ct3_d4_9x1(),raw_interlace_ct3_d4_9x1(),ts_interlace_ct3_d4_9x1(),0,10nat),
{
    hide(z_interlace_ct3_d4_9x1);hide(raw_interlace_ct3_d4_9x1);hide(ts_interlace_ct3_d4_9x1);hide(t::valid_step);
    assert(t::trace(z_interlace_ct3_d4_9x1(),raw_interlace_ct3_d4_9x1(),ts_interlace_ct3_d4_9x1(),0,10nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_interlace_ct3_d4_9x1()
    ensures raw_interlace_ct3_d4_9x1().len()==9nat,
        z_interlace_ct3_d4_9x1().len()==17nat,
        z_interlace_ct3_d4_9x1()[0]==120u8,
        z_interlace_ct3_d4_9x1()[1]==156u8,
        zm::field_value(z_interlace_ct3_d4_9x1(),16,1)==1nat,
        zm::field_value(z_interlace_ct3_d4_9x1(),17,2)==1nat,
        zm::be_value(z_interlace_ct3_d4_9x1(),13nat,4)==143000147nat,
        ts_interlace_ct3_d4_9x1().len()==10nat,
        ts_interlace_ct3_d4_9x1()[0].pos==19nat,
        ts_interlace_ct3_d4_9x1()[0].before==0nat,
        ts_interlace_ct3_d4_9x1().last().next==99nat,
{
    hide(z_interlace_ct3_d4_9x1);hide(raw_interlace_ct3_d4_9x1);hide(ts_interlace_ct3_d4_9x1);
    assert(raw_interlace_ct3_d4_9x1().len()==9nat) by (compute);
    assert(z_interlace_ct3_d4_9x1().len()==17nat) by (compute);
    assert(z_interlace_ct3_d4_9x1()[0]==120u8) by (compute);
    assert(z_interlace_ct3_d4_9x1()[1]==156u8) by (compute);
    assert(zm::field_value(z_interlace_ct3_d4_9x1(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_interlace_ct3_d4_9x1(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_interlace_ct3_d4_9x1(),13nat,4)==143000147nat) by (compute);
    assert(ts_interlace_ct3_d4_9x1().len()==10nat) by (compute);
    assert(ts_interlace_ct3_d4_9x1()[0].pos==19nat) by (compute);
    assert(ts_interlace_ct3_d4_9x1()[0].before==0nat) by (compute);
    assert(ts_interlace_ct3_d4_9x1().last().next==99nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_interlace_ct3_d4_9x1()
    ensures zm::Adler(raw_interlace_ct3_d4_9x1())==143000147nat,
{
    hide(raw_interlace_ct3_d4_9x1);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_interlace_ct3_d4_9x1().len()==9nat) by (compute);
    assert(c::sums(raw_interlace_ct3_d4_9x1(),0,9nat)==(594nat,2182nat)) by (compute);
    c::sums_correct(raw_interlace_ct3_d4_9x1(),0,9nat);
    assert(raw_interlace_ct3_d4_9x1().subrange(0,9) =~= raw_interlace_ct3_d4_9x1());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((2182nat%65521)*(u16::MAX as nat+1)+(1+594nat)%65521==143000147nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_interlace_ct3_d4_9x1()
    ensures r::inflated(z_interlace_ct3_d4_9x1(),a_interlace_ct3_d4_9x1(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_interlace_ct3_d4_9x1(),r2:false}),
        raw_interlace_ct3_d4_9x1().len()==9nat,
{
    hide(z_interlace_ct3_d4_9x1);hide(raw_interlace_ct3_d4_9x1);hide(ts_interlace_ct3_d4_9x1);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_interlace_ct3_d4_9x1();checksum_interlace_ct3_d4_9x1();
    assert(r::raw(a_interlace_ct3_d4_9x1())==9nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_interlace_ct3_d4_9x1();
    stream::fixed_stream(z_interlace_ct3_d4_9x1(),raw_interlace_ct3_d4_9x1(),ts_interlace_ct3_d4_9x1(),9nat,156nat,143000147nat);
}
#[verifier::spinoff_prover]
proof fn scan_interlace_ct3_d4_9x1()
    ensures r::scan(raw_interlace_ct3_d4_9x1(),a_interlace_ct3_d4_9x1(),0)==Ok(()),
{
    hide(raw_interlace_ct3_d4_9x1);
    assert(r::count(a_interlace_ct3_d4_9x1())==7nat) by (compute);
    r::scan_step(raw_interlace_ct3_d4_9x1(),a_interlace_ct3_d4_9x1(),7);
    assert(r::pw(a_interlace_ct3_d4_9x1(),6nat)==9nat && r::ph(a_interlace_ct3_d4_9x1(),6nat)==0nat && r::rb(a_interlace_ct3_d4_9x1(),6nat)==5nat && r::base(a_interlace_ct3_d4_9x1(),6nat)==9nat) by (compute);
    r::pass_step(raw_interlace_ct3_d4_9x1(),a_interlace_ct3_d4_9x1(),6nat,0);
    r::scan_step(raw_interlace_ct3_d4_9x1(),a_interlace_ct3_d4_9x1(),6nat);
    assert(r::pw(a_interlace_ct3_d4_9x1(),5nat)==4nat && r::ph(a_interlace_ct3_d4_9x1(),5nat)==1nat && r::rb(a_interlace_ct3_d4_9x1(),5nat)==2nat && r::base(a_interlace_ct3_d4_9x1(),5nat)==6nat) by (compute);
    assert(raw_interlace_ct3_d4_9x1()[6]==3u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_9x1(),a_interlace_ct3_d4_9x1(),5nat,0nat),4nat);
    assert(r::row_result(raw_interlace_ct3_d4_9x1(),a_interlace_ct3_d4_9x1(),5nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_9x1(),a_interlace_ct3_d4_9x1(),5nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_9x1(),a_interlace_ct3_d4_9x1(),5nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_9x1(),a_interlace_ct3_d4_9x1(),5nat);
    assert(r::pw(a_interlace_ct3_d4_9x1(),4nat)==5nat && r::ph(a_interlace_ct3_d4_9x1(),4nat)==0nat && r::rb(a_interlace_ct3_d4_9x1(),4nat)==3nat && r::base(a_interlace_ct3_d4_9x1(),4nat)==6nat) by (compute);
    r::pass_step(raw_interlace_ct3_d4_9x1(),a_interlace_ct3_d4_9x1(),4nat,0);
    r::scan_step(raw_interlace_ct3_d4_9x1(),a_interlace_ct3_d4_9x1(),4nat);
    assert(r::pw(a_interlace_ct3_d4_9x1(),3nat)==2nat && r::ph(a_interlace_ct3_d4_9x1(),3nat)==1nat && r::rb(a_interlace_ct3_d4_9x1(),3nat)==1nat && r::base(a_interlace_ct3_d4_9x1(),3nat)==4nat) by (compute);
    assert(raw_interlace_ct3_d4_9x1()[4]==2u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_9x1(),a_interlace_ct3_d4_9x1(),3nat,0nat),2nat);
    assert(r::row_result(raw_interlace_ct3_d4_9x1(),a_interlace_ct3_d4_9x1(),3nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_9x1(),a_interlace_ct3_d4_9x1(),3nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_9x1(),a_interlace_ct3_d4_9x1(),3nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_9x1(),a_interlace_ct3_d4_9x1(),3nat);
    assert(r::pw(a_interlace_ct3_d4_9x1(),2nat)==3nat && r::ph(a_interlace_ct3_d4_9x1(),2nat)==0nat && r::rb(a_interlace_ct3_d4_9x1(),2nat)==2nat && r::base(a_interlace_ct3_d4_9x1(),2nat)==4nat) by (compute);
    r::pass_step(raw_interlace_ct3_d4_9x1(),a_interlace_ct3_d4_9x1(),2nat,0);
    r::scan_step(raw_interlace_ct3_d4_9x1(),a_interlace_ct3_d4_9x1(),2nat);
    assert(r::pw(a_interlace_ct3_d4_9x1(),1nat)==1nat && r::ph(a_interlace_ct3_d4_9x1(),1nat)==1nat && r::rb(a_interlace_ct3_d4_9x1(),1nat)==1nat && r::base(a_interlace_ct3_d4_9x1(),1nat)==2nat) by (compute);
    assert(raw_interlace_ct3_d4_9x1()[2]==1u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_9x1(),a_interlace_ct3_d4_9x1(),1nat,0nat),1nat);
    assert(r::row_result(raw_interlace_ct3_d4_9x1(),a_interlace_ct3_d4_9x1(),1nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_9x1(),a_interlace_ct3_d4_9x1(),1nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_9x1(),a_interlace_ct3_d4_9x1(),1nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_9x1(),a_interlace_ct3_d4_9x1(),1nat);
    assert(r::pw(a_interlace_ct3_d4_9x1(),0nat)==2nat && r::ph(a_interlace_ct3_d4_9x1(),0nat)==1nat && r::rb(a_interlace_ct3_d4_9x1(),0nat)==1nat && r::base(a_interlace_ct3_d4_9x1(),0nat)==0nat) by (compute);
    assert(raw_interlace_ct3_d4_9x1()[0]==0u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_9x1(),a_interlace_ct3_d4_9x1(),0nat,0nat),2nat);
    assert(r::row_result(raw_interlace_ct3_d4_9x1(),a_interlace_ct3_d4_9x1(),0nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_9x1(),a_interlace_ct3_d4_9x1(),0nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_9x1(),a_interlace_ct3_d4_9x1(),0nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_9x1(),a_interlace_ct3_d4_9x1(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_interlace_ct3_d4_9x1()
    ensures r::still(bytes_interlace_ct3_d4_9x1(),STRICT,false)==(r::Projection::Ready {width:9u32,height:1u32,bytes:36nat}),
{
    hide(bytes_interlace_ct3_d4_9x1);hide(z_interlace_ct3_d4_9x1);hide(raw_interlace_ct3_d4_9x1);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);
    assert(bytes_interlace_ct3_d4_9x1().len()==134nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct3_d4_9x1())==Ok((crate::model::Ihdr {width:9u32,height:1u32,depth:4u8,color_type:3u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct3_d4_9x1())==Ok((crate::model::Ihdr {width:9u32,height:1u32,depth:4u8,color_type:3u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 4u8)==16u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_9x1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_9x1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(48nat,3u8,4u8,false,false,false,false)==crate::metadata_model::Decision::Keep(16nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_9x1(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:17nat,next:122nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_9x1(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:17nat,next:122nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_9x1(),122nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:130nat,len:0nat,next:134nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_9x1(),122nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:130nat,len:0nat,next:134nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct3_d4_9x1().subrange(101,118)==z_interlace_ct3_d4_9x1()) by (compute);
    walk_interlace_ct3_d4_9x1(bytes_interlace_ct3_d4_9x1());
    inflate_interlace_ct3_d4_9x1();scan_interlace_ct3_d4_9x1();
    assert(r::raw(a_interlace_ct3_d4_9x1())==9nat) by (compute);
    assert(r::data(z_interlace_ct3_d4_9x1(),a_interlace_ct3_d4_9x1(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_interlace_ct3_d4_9x1(),h_interlace_ct3_d4_9x1(),z_interlace_ct3_d4_9x1(),16nat,false);
}
fn public_interlace_ct3_d4_9x1() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,9u8,0u8,0u8,0u8,1u8,4u8,3u8,0u8,0u8,1u8,137u8,235u8,169u8,216u8,0u8,0u8,0u8,48u8,80u8,76u8,84u8,69u8,6u8,57u8,192u8,47u8,48u8,39u8,94u8,5u8,197u8,195u8,225u8,116u8,241u8,0u8,255u8,230u8,223u8,121u8,71u8,172u8,53u8,157u8,158u8,239u8,191u8,144u8,220u8,242u8,167u8,59u8,86u8,140u8,145u8,106u8,233u8,230u8,23u8,94u8,171u8,34u8,31u8,171u8,169u8,32u8,126u8,137u8,211u8,213u8,37u8,219u8,116u8,217u8,0u8,0u8,0u8,17u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,40u8,98u8,84u8,96u8,106u8,100u8,190u8,150u8,12u8,0u8,8u8,134u8,2u8,83u8,93u8,36u8,160u8,94u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_interlace_ct3_d4_9x1());model_interlace_ct3_d4_9x1();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==9u32 && s.height==1u32 && s.rgba@.len()==36nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// interlace/ct0-d1-1x9; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_interlace_ct0_d1_1x9() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,9u8,1u8,0u8,0u8,0u8,1u8,172u8,58u8,75u8,223u8,0u8,0u8,0u8,26u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,104u8,96u8,108u8,96u8,98u8,96u8,102u8,96u8,97u8,96u8,104u8,96u8,100u8,96u8,106u8,96u8,62u8,0u8,0u8,22u8,94u8,2u8,209u8,250u8,161u8,91u8,128u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_interlace_ct0_d1_1x9() -> Seq<u8> {seq![120u8,156u8,99u8,104u8,96u8,108u8,96u8,98u8,96u8,102u8,96u8,97u8,96u8,104u8,96u8,100u8,96u8,106u8,96u8,62u8,0u8,0u8,22u8,94u8,2u8,209u8]}
spec fn h_interlace_ct0_d1_1x9() -> crate::model::Ihdr {crate::model::Ihdr {width:1u32,height:9u32,depth:1u8,color_type:0u8,interlace:1u8}}
spec fn a_interlace_ct0_d1_1x9() -> r::Raster {r::shape(h_interlace_ct0_d1_1x9(),1nat,9nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_interlace_ct0_d1_1x9(b: Seq<u8>)
    requires b.len()==83,crate::model::mh(b)==Ok((crate::model::Ihdr {width:1u32,height:9u32,depth:1u8,color_type:0u8,interlace:1u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:26nat,next:71nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,71nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:79nat,len:0nat,next:83nat,crc_ok:true,synthetic:false})),
        b.subrange(41,67)==z_interlace_ct0_d1_1x9(),
    ensures r::default_case(b,crate::model::Ihdr {width:1u32,height:9u32,depth:1u8,color_type:0u8,interlace:1u8},z_interlace_ct0_d1_1x9(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    hide(z_interlace_ct0_d1_1x9);
        assert((1u32 << 1u8)==2u32) by (compute);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_interlace_ct0_d1_1x9() -> Seq<u8> {seq![0u8,128u8,1u8,128u8,2u8,0u8,3u8,0u8,4u8,0u8,0u8,128u8,1u8,0u8,2u8,128u8,3u8,192u8]}
spec fn ts_interlace_ct0_d1_1x9() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:35nat,before:1nat,after:2nat,symbol:128nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:35nat,next:43nat,before:2nat,after:3nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:43nat,next:51nat,before:3nat,after:4nat,symbol:128nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:51nat,next:59nat,before:4nat,after:5nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:59nat,next:67nat,before:5nat,after:6nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:67nat,next:75nat,before:6nat,after:7nat,symbol:3nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:75nat,next:83nat,before:7nat,after:8nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:83nat,next:91nat,before:8nat,after:9nat,symbol:4nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:91nat,next:99nat,before:9nat,after:10nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:99nat,next:107nat,before:10nat,after:11nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:107nat,next:115nat,before:11nat,after:12nat,symbol:128nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:115nat,next:123nat,before:12nat,after:13nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:123nat,next:131nat,before:13nat,after:14nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:131nat,next:139nat,before:14nat,after:15nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:139nat,next:147nat,before:15nat,after:16nat,symbol:128nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:147nat,next:155nat,before:16nat,after:17nat,symbol:3nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:155nat,next:164nat,before:17nat,after:18nat,symbol:192nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:164nat,next:171nat,before:18nat,after:18nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_interlace_ct0_d1_1x9()
    ensures t::trace(z_interlace_ct0_d1_1x9(),raw_interlace_ct0_d1_1x9(),ts_interlace_ct0_d1_1x9(),0,19nat),
{
    hide(z_interlace_ct0_d1_1x9);hide(raw_interlace_ct0_d1_1x9);hide(ts_interlace_ct0_d1_1x9);hide(t::valid_step);
    assert(t::trace(z_interlace_ct0_d1_1x9(),raw_interlace_ct0_d1_1x9(),ts_interlace_ct0_d1_1x9(),0,19nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_interlace_ct0_d1_1x9()
    ensures raw_interlace_ct0_d1_1x9().len()==18nat,
        z_interlace_ct0_d1_1x9().len()==26nat,
        z_interlace_ct0_d1_1x9()[0]==120u8,
        z_interlace_ct0_d1_1x9()[1]==156u8,
        zm::field_value(z_interlace_ct0_d1_1x9(),16,1)==1nat,
        zm::field_value(z_interlace_ct0_d1_1x9(),17,2)==1nat,
        zm::be_value(z_interlace_ct0_d1_1x9(),22nat,4)==375259857nat,
        ts_interlace_ct0_d1_1x9().len()==19nat,
        ts_interlace_ct0_d1_1x9()[0].pos==19nat,
        ts_interlace_ct0_d1_1x9()[0].before==0nat,
        ts_interlace_ct0_d1_1x9().last().next==171nat,
{
    hide(z_interlace_ct0_d1_1x9);hide(raw_interlace_ct0_d1_1x9);hide(ts_interlace_ct0_d1_1x9);
    assert(raw_interlace_ct0_d1_1x9().len()==18nat) by (compute);
    assert(z_interlace_ct0_d1_1x9().len()==26nat) by (compute);
    assert(z_interlace_ct0_d1_1x9()[0]==120u8) by (compute);
    assert(z_interlace_ct0_d1_1x9()[1]==156u8) by (compute);
    assert(zm::field_value(z_interlace_ct0_d1_1x9(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_interlace_ct0_d1_1x9(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_interlace_ct0_d1_1x9(),22nat,4)==375259857nat) by (compute);
    assert(ts_interlace_ct0_d1_1x9().len()==19nat) by (compute);
    assert(ts_interlace_ct0_d1_1x9()[0].pos==19nat) by (compute);
    assert(ts_interlace_ct0_d1_1x9()[0].before==0nat) by (compute);
    assert(ts_interlace_ct0_d1_1x9().last().next==171nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_interlace_ct0_d1_1x9()
    ensures zm::Adler(raw_interlace_ct0_d1_1x9())==375259857nat,
{
    hide(raw_interlace_ct0_d1_1x9);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_interlace_ct0_d1_1x9().len()==18nat) by (compute);
    assert(c::sums(raw_interlace_ct0_d1_1x9(),0,18nat)==(720nat,5726nat)) by (compute);
    c::sums_correct(raw_interlace_ct0_d1_1x9(),0,18nat);
    assert(raw_interlace_ct0_d1_1x9().subrange(0,18) =~= raw_interlace_ct0_d1_1x9());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((5726nat%65521)*(u16::MAX as nat+1)+(1+720nat)%65521==375259857nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_interlace_ct0_d1_1x9()
    ensures r::inflated(z_interlace_ct0_d1_1x9(),a_interlace_ct0_d1_1x9(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_interlace_ct0_d1_1x9(),r2:false}),
        raw_interlace_ct0_d1_1x9().len()==18nat,
{
    hide(z_interlace_ct0_d1_1x9);hide(raw_interlace_ct0_d1_1x9);hide(ts_interlace_ct0_d1_1x9);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_interlace_ct0_d1_1x9();checksum_interlace_ct0_d1_1x9();
    assert(r::raw(a_interlace_ct0_d1_1x9())==18nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_interlace_ct0_d1_1x9();
    stream::fixed_stream(z_interlace_ct0_d1_1x9(),raw_interlace_ct0_d1_1x9(),ts_interlace_ct0_d1_1x9(),18nat,156nat,375259857nat);
}
#[verifier::spinoff_prover]
proof fn scan_interlace_ct0_d1_1x9()
    ensures r::scan(raw_interlace_ct0_d1_1x9(),a_interlace_ct0_d1_1x9(),0)==Ok(()),
{
    hide(raw_interlace_ct0_d1_1x9);
    assert(r::count(a_interlace_ct0_d1_1x9())==7nat) by (compute);
    r::scan_step(raw_interlace_ct0_d1_1x9(),a_interlace_ct0_d1_1x9(),7);
    assert(r::pw(a_interlace_ct0_d1_1x9(),6nat)==1nat && r::ph(a_interlace_ct0_d1_1x9(),6nat)==4nat && r::rb(a_interlace_ct0_d1_1x9(),6nat)==1nat && r::base(a_interlace_ct0_d1_1x9(),6nat)==10nat) by (compute);
    assert(raw_interlace_ct0_d1_1x9()[10]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_1x9(),a_interlace_ct0_d1_1x9(),6nat,0nat)==Ok(()));
    assert(raw_interlace_ct0_d1_1x9()[12]==1u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_1x9(),a_interlace_ct0_d1_1x9(),6nat,1nat)==Ok(()));
    assert(raw_interlace_ct0_d1_1x9()[14]==2u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_1x9(),a_interlace_ct0_d1_1x9(),6nat,2nat)==Ok(()));
    assert(raw_interlace_ct0_d1_1x9()[16]==3u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_1x9(),a_interlace_ct0_d1_1x9(),6nat,3nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_1x9(),a_interlace_ct0_d1_1x9(),6nat,4nat);
    r::pass_step(raw_interlace_ct0_d1_1x9(),a_interlace_ct0_d1_1x9(),6nat,3nat);
    r::pass_step(raw_interlace_ct0_d1_1x9(),a_interlace_ct0_d1_1x9(),6nat,2nat);
    r::pass_step(raw_interlace_ct0_d1_1x9(),a_interlace_ct0_d1_1x9(),6nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_1x9(),a_interlace_ct0_d1_1x9(),6nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_1x9(),a_interlace_ct0_d1_1x9(),6nat);
    assert(r::pw(a_interlace_ct0_d1_1x9(),5nat)==0nat && r::ph(a_interlace_ct0_d1_1x9(),5nat)==5nat && r::rb(a_interlace_ct0_d1_1x9(),5nat)==0nat && r::base(a_interlace_ct0_d1_1x9(),5nat)==10nat) by (compute);
    r::pass_step(raw_interlace_ct0_d1_1x9(),a_interlace_ct0_d1_1x9(),5nat,0);
    r::scan_step(raw_interlace_ct0_d1_1x9(),a_interlace_ct0_d1_1x9(),5nat);
    assert(r::pw(a_interlace_ct0_d1_1x9(),4nat)==1nat && r::ph(a_interlace_ct0_d1_1x9(),4nat)==2nat && r::rb(a_interlace_ct0_d1_1x9(),4nat)==1nat && r::base(a_interlace_ct0_d1_1x9(),4nat)==6nat) by (compute);
    assert(raw_interlace_ct0_d1_1x9()[6]==3u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_1x9(),a_interlace_ct0_d1_1x9(),4nat,0nat)==Ok(()));
    assert(raw_interlace_ct0_d1_1x9()[8]==4u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_1x9(),a_interlace_ct0_d1_1x9(),4nat,1nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_1x9(),a_interlace_ct0_d1_1x9(),4nat,2nat);
    r::pass_step(raw_interlace_ct0_d1_1x9(),a_interlace_ct0_d1_1x9(),4nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_1x9(),a_interlace_ct0_d1_1x9(),4nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_1x9(),a_interlace_ct0_d1_1x9(),4nat);
    assert(r::pw(a_interlace_ct0_d1_1x9(),3nat)==0nat && r::ph(a_interlace_ct0_d1_1x9(),3nat)==3nat && r::rb(a_interlace_ct0_d1_1x9(),3nat)==0nat && r::base(a_interlace_ct0_d1_1x9(),3nat)==6nat) by (compute);
    r::pass_step(raw_interlace_ct0_d1_1x9(),a_interlace_ct0_d1_1x9(),3nat,0);
    r::scan_step(raw_interlace_ct0_d1_1x9(),a_interlace_ct0_d1_1x9(),3nat);
    assert(r::pw(a_interlace_ct0_d1_1x9(),2nat)==1nat && r::ph(a_interlace_ct0_d1_1x9(),2nat)==1nat && r::rb(a_interlace_ct0_d1_1x9(),2nat)==1nat && r::base(a_interlace_ct0_d1_1x9(),2nat)==4nat) by (compute);
    assert(raw_interlace_ct0_d1_1x9()[4]==2u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_1x9(),a_interlace_ct0_d1_1x9(),2nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_1x9(),a_interlace_ct0_d1_1x9(),2nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_1x9(),a_interlace_ct0_d1_1x9(),2nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_1x9(),a_interlace_ct0_d1_1x9(),2nat);
    assert(r::pw(a_interlace_ct0_d1_1x9(),1nat)==0nat && r::ph(a_interlace_ct0_d1_1x9(),1nat)==2nat && r::rb(a_interlace_ct0_d1_1x9(),1nat)==0nat && r::base(a_interlace_ct0_d1_1x9(),1nat)==4nat) by (compute);
    r::pass_step(raw_interlace_ct0_d1_1x9(),a_interlace_ct0_d1_1x9(),1nat,0);
    r::scan_step(raw_interlace_ct0_d1_1x9(),a_interlace_ct0_d1_1x9(),1nat);
    assert(r::pw(a_interlace_ct0_d1_1x9(),0nat)==1nat && r::ph(a_interlace_ct0_d1_1x9(),0nat)==2nat && r::rb(a_interlace_ct0_d1_1x9(),0nat)==1nat && r::base(a_interlace_ct0_d1_1x9(),0nat)==0nat) by (compute);
    assert(raw_interlace_ct0_d1_1x9()[0]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_1x9(),a_interlace_ct0_d1_1x9(),0nat,0nat)==Ok(()));
    assert(raw_interlace_ct0_d1_1x9()[2]==1u8) by (compute);
    assert(r::row_result(raw_interlace_ct0_d1_1x9(),a_interlace_ct0_d1_1x9(),0nat,1nat)==Ok(()));
    r::pass_step(raw_interlace_ct0_d1_1x9(),a_interlace_ct0_d1_1x9(),0nat,2nat);
    r::pass_step(raw_interlace_ct0_d1_1x9(),a_interlace_ct0_d1_1x9(),0nat,1nat);
    r::pass_step(raw_interlace_ct0_d1_1x9(),a_interlace_ct0_d1_1x9(),0nat,0nat);
    r::scan_step(raw_interlace_ct0_d1_1x9(),a_interlace_ct0_d1_1x9(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_interlace_ct0_d1_1x9()
    ensures r::still(bytes_interlace_ct0_d1_1x9(),STRICT,false)==(r::Projection::Ready {width:1u32,height:9u32,bytes:36nat}),
{
    hide(bytes_interlace_ct0_d1_1x9);hide(z_interlace_ct0_d1_1x9);hide(raw_interlace_ct0_d1_1x9);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_interlace_ct0_d1_1x9().len()==83nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct0_d1_1x9())==Ok((crate::model::Ihdr {width:1u32,height:9u32,depth:1u8,color_type:0u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct0_d1_1x9())==Ok((crate::model::Ihdr {width:1u32,height:9u32,depth:1u8,color_type:0u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 1u8)==2u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct0_d1_1x9(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:26nat,next:71nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct0_d1_1x9(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:26nat,next:71nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct0_d1_1x9(),71nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:79nat,len:0nat,next:83nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct0_d1_1x9(),71nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:79nat,len:0nat,next:83nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct0_d1_1x9().subrange(41,67)==z_interlace_ct0_d1_1x9()) by (compute);
    walk_interlace_ct0_d1_1x9(bytes_interlace_ct0_d1_1x9());
    inflate_interlace_ct0_d1_1x9();scan_interlace_ct0_d1_1x9();
    assert(r::raw(a_interlace_ct0_d1_1x9())==18nat) by (compute);
    assert(r::data(z_interlace_ct0_d1_1x9(),a_interlace_ct0_d1_1x9(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_interlace_ct0_d1_1x9(),h_interlace_ct0_d1_1x9(),z_interlace_ct0_d1_1x9(),0nat,false);
}
fn public_interlace_ct0_d1_1x9() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,9u8,1u8,0u8,0u8,0u8,1u8,172u8,58u8,75u8,223u8,0u8,0u8,0u8,26u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,104u8,96u8,108u8,96u8,98u8,96u8,102u8,96u8,97u8,96u8,104u8,96u8,100u8,96u8,106u8,96u8,62u8,0u8,0u8,22u8,94u8,2u8,209u8,250u8,161u8,91u8,128u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_interlace_ct0_d1_1x9());model_interlace_ct0_d1_1x9();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==1u32 && s.height==9u32 && s.rgba@.len()==36nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// interlace/ct6-d16-1x9; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_interlace_ct6_d16_1x9() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,9u8,16u8,6u8,0u8,0u8,1u8,212u8,209u8,170u8,49u8,0u8,0u8,0u8,92u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,81u8,0u8,174u8,255u8,0u8,253u8,103u8,180u8,101u8,210u8,97u8,178u8,156u8,1u8,56u8,19u8,63u8,80u8,49u8,204u8,67u8,203u8,2u8,172u8,136u8,45u8,185u8,89u8,170u8,76u8,252u8,3u8,13u8,28u8,217u8,34u8,235u8,59u8,236u8,173u8,4u8,161u8,126u8,0u8,157u8,176u8,11u8,215u8,175u8,0u8,129u8,23u8,35u8,113u8,99u8,127u8,29u8,85u8,1u8,187u8,186u8,184u8,95u8,244u8,102u8,52u8,30u8,2u8,94u8,134u8,7u8,203u8,106u8,30u8,199u8,43u8,3u8,148u8,226u8,137u8,179u8,28u8,174u8,168u8,63u8,145u8,30u8,34u8,132u8,53u8,7u8,249u8,90u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_interlace_ct6_d16_1x9() -> Seq<u8> {seq![120u8,156u8,1u8,81u8,0u8,174u8,255u8,0u8,253u8,103u8,180u8,101u8,210u8,97u8,178u8,156u8,1u8,56u8,19u8,63u8,80u8,49u8,204u8,67u8,203u8,2u8,172u8,136u8,45u8,185u8,89u8,170u8,76u8,252u8,3u8,13u8,28u8,217u8,34u8,235u8,59u8,236u8,173u8,4u8,161u8,126u8,0u8,157u8,176u8,11u8,215u8,175u8,0u8,129u8,23u8,35u8,113u8,99u8,127u8,29u8,85u8,1u8,187u8,186u8,184u8,95u8,244u8,102u8,52u8,30u8,2u8,94u8,134u8,7u8,203u8,106u8,30u8,199u8,43u8,3u8,148u8,226u8,137u8,179u8,28u8,174u8,168u8,63u8,145u8,30u8,34u8,132u8]}
spec fn h_interlace_ct6_d16_1x9() -> crate::model::Ihdr {crate::model::Ihdr {width:1u32,height:9u32,depth:16u8,color_type:6u8,interlace:1u8}}
spec fn a_interlace_ct6_d16_1x9() -> r::Raster {r::shape(h_interlace_ct6_d16_1x9(),1nat,9nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_interlace_ct6_d16_1x9(b: Seq<u8>)
    requires b.len()==149,crate::model::mh(b)==Ok((crate::model::Ihdr {width:1u32,height:9u32,depth:16u8,color_type:6u8,interlace:1u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:92nat,next:137nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,137nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:145nat,len:0nat,next:149nat,crc_ok:true,synthetic:false})),
        b.subrange(41,133)==z_interlace_ct6_d16_1x9(),
    ensures r::default_case(b,crate::model::Ihdr {width:1u32,height:9u32,depth:16u8,color_type:6u8,interlace:1u8},z_interlace_ct6_d16_1x9(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    hide(z_interlace_ct6_d16_1x9);
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_interlace_ct6_d16_1x9() -> Seq<u8> {seq![0u8,253u8,103u8,180u8,101u8,210u8,97u8,178u8,156u8,1u8,56u8,19u8,63u8,80u8,49u8,204u8,67u8,203u8,2u8,172u8,136u8,45u8,185u8,89u8,170u8,76u8,252u8,3u8,13u8,28u8,217u8,34u8,235u8,59u8,236u8,173u8,4u8,161u8,126u8,0u8,157u8,176u8,11u8,215u8,175u8,0u8,129u8,23u8,35u8,113u8,99u8,127u8,29u8,85u8,1u8,187u8,186u8,184u8,95u8,244u8,102u8,52u8,30u8,2u8,94u8,134u8,7u8,203u8,106u8,30u8,199u8,43u8,3u8,148u8,226u8,137u8,179u8,28u8,174u8,168u8,63u8]}
#[verifier::spinoff_prover]
proof fn shape_interlace_ct6_d16_1x9()
    ensures raw_interlace_ct6_d16_1x9().len()==81nat,
        z_interlace_ct6_d16_1x9().len()==92nat,
        z_interlace_ct6_d16_1x9()[0]==120u8,
        z_interlace_ct6_d16_1x9()[1]==156u8,
        zm::field_value(z_interlace_ct6_d16_1x9(),16,1)==1nat,
        zm::field_value(z_interlace_ct6_d16_1x9(),17,2)==0nat,
        zm::be_value(z_interlace_ct6_d16_1x9(),88nat,4)==2434671236nat,
        z_interlace_ct6_d16_1x9().subrange(7,88)==raw_interlace_ct6_d16_1x9(),
        z_interlace_ct6_d16_1x9()[3] as nat+256*z_interlace_ct6_d16_1x9()[4] as nat==81nat,
        z_interlace_ct6_d16_1x9()[5] as nat+256*z_interlace_ct6_d16_1x9()[6] as nat==u16::MAX as nat-81nat,
        zm::be_value(z_interlace_ct6_d16_1x9(),0,1)==120nat,
        zm::be_value(z_interlace_ct6_d16_1x9(),1,1)==156nat,
{
    hide(z_interlace_ct6_d16_1x9);hide(raw_interlace_ct6_d16_1x9);
    assert(raw_interlace_ct6_d16_1x9().len()==81nat) by (compute);
    assert(z_interlace_ct6_d16_1x9().len()==92nat) by (compute);
    assert(z_interlace_ct6_d16_1x9()[0]==120u8) by (compute);
    assert(z_interlace_ct6_d16_1x9()[1]==156u8) by (compute);
    assert(zm::field_value(z_interlace_ct6_d16_1x9(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_interlace_ct6_d16_1x9(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_interlace_ct6_d16_1x9(),88nat,4)==2434671236nat) by (compute);
    assert(z_interlace_ct6_d16_1x9().subrange(7,88)==raw_interlace_ct6_d16_1x9()) by (compute);
    assert(z_interlace_ct6_d16_1x9()[3] as nat+256*z_interlace_ct6_d16_1x9()[4] as nat==81nat) by (compute);
    assert(z_interlace_ct6_d16_1x9()[5] as nat+256*z_interlace_ct6_d16_1x9()[6] as nat==u16::MAX as nat-81nat) by (compute);
    assert(zm::be_value(z_interlace_ct6_d16_1x9(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_interlace_ct6_d16_1x9(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_interlace_ct6_d16_1x9()
    ensures zm::Adler(raw_interlace_ct6_d16_1x9())==2434671236nat,
{
    hide(raw_interlace_ct6_d16_1x9);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_interlace_ct6_d16_1x9().len()==81nat) by (compute);
    assert(c::sums(raw_interlace_ct6_d16_1x9(),0,81nat)==(8835nat,364755nat)) by (compute);
    c::sums_correct(raw_interlace_ct6_d16_1x9(),0,81nat);
    assert(raw_interlace_ct6_d16_1x9().subrange(0,81) =~= raw_interlace_ct6_d16_1x9());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((364755nat%65521)*(u16::MAX as nat+1)+(1+8835nat)%65521==2434671236nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_interlace_ct6_d16_1x9()
    ensures r::inflated(z_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_interlace_ct6_d16_1x9(),r2:false}),
        raw_interlace_ct6_d16_1x9().len()==81nat,
{
    hide(z_interlace_ct6_d16_1x9);hide(raw_interlace_ct6_d16_1x9);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_interlace_ct6_d16_1x9();checksum_interlace_ct6_d16_1x9();
    assert(r::raw(a_interlace_ct6_d16_1x9())==81nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_interlace_ct6_d16_1x9(),raw_interlace_ct6_d16_1x9(),81nat,120nat,156nat,2434671236nat);
}
#[verifier::spinoff_prover]
proof fn scan_interlace_ct6_d16_1x9()
    ensures r::scan(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),0)==Ok(()),
{
    hide(raw_interlace_ct6_d16_1x9);
    assert(r::count(a_interlace_ct6_d16_1x9())==7nat) by (compute);
    r::scan_step(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),7);
    assert(r::pw(a_interlace_ct6_d16_1x9(),6nat)==1nat && r::ph(a_interlace_ct6_d16_1x9(),6nat)==4nat && r::rb(a_interlace_ct6_d16_1x9(),6nat)==8nat && r::base(a_interlace_ct6_d16_1x9(),6nat)==45nat) by (compute);
    assert(raw_interlace_ct6_d16_1x9()[45]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),6nat,0nat)==Ok(()));
    assert(raw_interlace_ct6_d16_1x9()[54]==1u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),6nat,1nat)==Ok(()));
    assert(raw_interlace_ct6_d16_1x9()[63]==2u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),6nat,2nat)==Ok(()));
    assert(raw_interlace_ct6_d16_1x9()[72]==3u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),6nat,3nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),6nat,4nat);
    r::pass_step(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),6nat,3nat);
    r::pass_step(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),6nat,2nat);
    r::pass_step(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),6nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),6nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),6nat);
    assert(r::pw(a_interlace_ct6_d16_1x9(),5nat)==0nat && r::ph(a_interlace_ct6_d16_1x9(),5nat)==5nat && r::rb(a_interlace_ct6_d16_1x9(),5nat)==0nat && r::base(a_interlace_ct6_d16_1x9(),5nat)==45nat) by (compute);
    r::pass_step(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),5nat,0);
    r::scan_step(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),5nat);
    assert(r::pw(a_interlace_ct6_d16_1x9(),4nat)==1nat && r::ph(a_interlace_ct6_d16_1x9(),4nat)==2nat && r::rb(a_interlace_ct6_d16_1x9(),4nat)==8nat && r::base(a_interlace_ct6_d16_1x9(),4nat)==27nat) by (compute);
    assert(raw_interlace_ct6_d16_1x9()[27]==3u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),4nat,0nat)==Ok(()));
    assert(raw_interlace_ct6_d16_1x9()[36]==4u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),4nat,1nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),4nat,2nat);
    r::pass_step(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),4nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),4nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),4nat);
    assert(r::pw(a_interlace_ct6_d16_1x9(),3nat)==0nat && r::ph(a_interlace_ct6_d16_1x9(),3nat)==3nat && r::rb(a_interlace_ct6_d16_1x9(),3nat)==0nat && r::base(a_interlace_ct6_d16_1x9(),3nat)==27nat) by (compute);
    r::pass_step(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),3nat,0);
    r::scan_step(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),3nat);
    assert(r::pw(a_interlace_ct6_d16_1x9(),2nat)==1nat && r::ph(a_interlace_ct6_d16_1x9(),2nat)==1nat && r::rb(a_interlace_ct6_d16_1x9(),2nat)==8nat && r::base(a_interlace_ct6_d16_1x9(),2nat)==18nat) by (compute);
    assert(raw_interlace_ct6_d16_1x9()[18]==2u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),2nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),2nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),2nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),2nat);
    assert(r::pw(a_interlace_ct6_d16_1x9(),1nat)==0nat && r::ph(a_interlace_ct6_d16_1x9(),1nat)==2nat && r::rb(a_interlace_ct6_d16_1x9(),1nat)==0nat && r::base(a_interlace_ct6_d16_1x9(),1nat)==18nat) by (compute);
    r::pass_step(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),1nat,0);
    r::scan_step(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),1nat);
    assert(r::pw(a_interlace_ct6_d16_1x9(),0nat)==1nat && r::ph(a_interlace_ct6_d16_1x9(),0nat)==2nat && r::rb(a_interlace_ct6_d16_1x9(),0nat)==8nat && r::base(a_interlace_ct6_d16_1x9(),0nat)==0nat) by (compute);
    assert(raw_interlace_ct6_d16_1x9()[0]==0u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),0nat,0nat)==Ok(()));
    assert(raw_interlace_ct6_d16_1x9()[9]==1u8) by (compute);
    assert(r::row_result(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),0nat,1nat)==Ok(()));
    r::pass_step(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),0nat,2nat);
    r::pass_step(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),0nat,1nat);
    r::pass_step(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),0nat,0nat);
    r::scan_step(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_interlace_ct6_d16_1x9()
    ensures r::still(bytes_interlace_ct6_d16_1x9(),STRICT,false)==(r::Projection::Ready {width:1u32,height:9u32,bytes:36nat}),
{
    hide(bytes_interlace_ct6_d16_1x9);hide(z_interlace_ct6_d16_1x9);hide(raw_interlace_ct6_d16_1x9);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_interlace_ct6_d16_1x9().len()==149nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct6_d16_1x9())==Ok((crate::model::Ihdr {width:1u32,height:9u32,depth:16u8,color_type:6u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct6_d16_1x9())==Ok((crate::model::Ihdr {width:1u32,height:9u32,depth:16u8,color_type:6u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct6_d16_1x9(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:92nat,next:137nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct6_d16_1x9(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:92nat,next:137nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct6_d16_1x9(),137nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:145nat,len:0nat,next:149nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct6_d16_1x9(),137nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:145nat,len:0nat,next:149nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct6_d16_1x9().subrange(41,133)==z_interlace_ct6_d16_1x9()) by (compute);
    walk_interlace_ct6_d16_1x9(bytes_interlace_ct6_d16_1x9());
    inflate_interlace_ct6_d16_1x9();scan_interlace_ct6_d16_1x9();
    assert(r::raw(a_interlace_ct6_d16_1x9())==81nat) by (compute);
    assert(r::data(z_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_interlace_ct6_d16_1x9(),h_interlace_ct6_d16_1x9(),z_interlace_ct6_d16_1x9(),0nat,false);
}
fn public_interlace_ct6_d16_1x9() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,9u8,16u8,6u8,0u8,0u8,1u8,212u8,209u8,170u8,49u8,0u8,0u8,0u8,92u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,81u8,0u8,174u8,255u8,0u8,253u8,103u8,180u8,101u8,210u8,97u8,178u8,156u8,1u8,56u8,19u8,63u8,80u8,49u8,204u8,67u8,203u8,2u8,172u8,136u8,45u8,185u8,89u8,170u8,76u8,252u8,3u8,13u8,28u8,217u8,34u8,235u8,59u8,236u8,173u8,4u8,161u8,126u8,0u8,157u8,176u8,11u8,215u8,175u8,0u8,129u8,23u8,35u8,113u8,99u8,127u8,29u8,85u8,1u8,187u8,186u8,184u8,95u8,244u8,102u8,52u8,30u8,2u8,94u8,134u8,7u8,203u8,106u8,30u8,199u8,43u8,3u8,148u8,226u8,137u8,179u8,28u8,174u8,168u8,63u8,145u8,30u8,34u8,132u8,53u8,7u8,249u8,90u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_interlace_ct6_d16_1x9());model_interlace_ct6_d16_1x9();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==1u32 && s.height==9u32 && s.rgba@.len()==36nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// interlace/ct3-d4-1x9; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_interlace_ct3_d4_1x9() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,9u8,4u8,3u8,0u8,0u8,1u8,118u8,111u8,107u8,65u8,0u8,0u8,0u8,48u8,80u8,76u8,84u8,69u8,33u8,200u8,29u8,0u8,192u8,84u8,8u8,198u8,36u8,196u8,79u8,63u8,106u8,13u8,88u8,9u8,104u8,32u8,201u8,42u8,74u8,172u8,130u8,213u8,102u8,138u8,204u8,70u8,4u8,100u8,86u8,216u8,216u8,122u8,188u8,187u8,23u8,68u8,5u8,180u8,147u8,135u8,206u8,63u8,86u8,4u8,123u8,180u8,141u8,107u8,80u8,193u8,0u8,0u8,0u8,26u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,192u8,184u8,129u8,169u8,128u8,185u8,129u8,165u8,128u8,193u8,128u8,113u8,2u8,147u8,3u8,179u8,7u8,0u8,47u8,38u8,4u8,89u8,101u8,111u8,27u8,13u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_interlace_ct3_d4_1x9() -> Seq<u8> {seq![120u8,156u8,99u8,248u8,192u8,184u8,129u8,169u8,128u8,185u8,129u8,165u8,128u8,193u8,128u8,113u8,2u8,147u8,3u8,179u8,7u8,0u8,47u8,38u8,4u8,89u8]}
spec fn h_interlace_ct3_d4_1x9() -> crate::model::Ihdr {crate::model::Ihdr {width:1u32,height:9u32,depth:4u8,color_type:3u8,interlace:1u8}}
spec fn a_interlace_ct3_d4_1x9() -> r::Raster {r::shape(h_interlace_ct3_d4_1x9(),1nat,9nat,16nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_interlace_ct3_d4_1x9(b: Seq<u8>)
    requires b.len()==143,crate::model::mh(b)==Ok((crate::model::Ihdr {width:1u32,height:9u32,depth:4u8,color_type:3u8,interlace:1u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:26nat,next:131nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,131nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:139nat,len:0nat,next:143nat,crc_ok:true,synthetic:false})),
        b.subrange(101,127)==z_interlace_ct3_d4_1x9(),
    ensures r::default_case(b,crate::model::Ihdr {width:1u32,height:9u32,depth:4u8,color_type:3u8,interlace:1u8},z_interlace_ct3_d4_1x9(),16nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);
    hide(z_interlace_ct3_d4_1x9);
    r::palette_default_case(b,crate::model::Ihdr {width:1u32,height:9u32,depth:4u8,color_type:3u8,interlace:1u8},z_interlace_ct3_d4_1x9(),26nat);
}
spec fn raw_interlace_ct3_d4_1x9() -> Seq<u8> {seq![0u8,240u8,1u8,176u8,2u8,112u8,3u8,128u8,4u8,112u8,0u8,48u8,1u8,144u8,2u8,64u8,3u8,72u8]}
spec fn ts_interlace_ct3_d4_1x9() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:36nat,before:1nat,after:2nat,symbol:240nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:36nat,next:44nat,before:2nat,after:3nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:44nat,next:53nat,before:3nat,after:4nat,symbol:176nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:53nat,next:61nat,before:4nat,after:5nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:61nat,next:69nat,before:5nat,after:6nat,symbol:112nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:69nat,next:77nat,before:6nat,after:7nat,symbol:3nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:77nat,next:85nat,before:7nat,after:8nat,symbol:128nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:85nat,next:93nat,before:8nat,after:9nat,symbol:4nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:93nat,next:101nat,before:9nat,after:10nat,symbol:112nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:101nat,next:109nat,before:10nat,after:11nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:109nat,next:117nat,before:11nat,after:12nat,symbol:48nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:117nat,next:125nat,before:12nat,after:13nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:125nat,next:134nat,before:13nat,after:14nat,symbol:144nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:134nat,next:142nat,before:14nat,after:15nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:142nat,next:150nat,before:15nat,after:16nat,symbol:64nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:150nat,next:158nat,before:16nat,after:17nat,symbol:3nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:158nat,next:166nat,before:17nat,after:18nat,symbol:72nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:166nat,next:173nat,before:18nat,after:18nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_interlace_ct3_d4_1x9()
    ensures t::trace(z_interlace_ct3_d4_1x9(),raw_interlace_ct3_d4_1x9(),ts_interlace_ct3_d4_1x9(),0,19nat),
{
    hide(z_interlace_ct3_d4_1x9);hide(raw_interlace_ct3_d4_1x9);hide(ts_interlace_ct3_d4_1x9);hide(t::valid_step);
    assert(t::trace(z_interlace_ct3_d4_1x9(),raw_interlace_ct3_d4_1x9(),ts_interlace_ct3_d4_1x9(),0,19nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_interlace_ct3_d4_1x9()
    ensures raw_interlace_ct3_d4_1x9().len()==18nat,
        z_interlace_ct3_d4_1x9().len()==26nat,
        z_interlace_ct3_d4_1x9()[0]==120u8,
        z_interlace_ct3_d4_1x9()[1]==156u8,
        zm::field_value(z_interlace_ct3_d4_1x9(),16,1)==1nat,
        zm::field_value(z_interlace_ct3_d4_1x9(),17,2)==1nat,
        zm::be_value(z_interlace_ct3_d4_1x9(),22nat,4)==791020633nat,
        ts_interlace_ct3_d4_1x9().len()==19nat,
        ts_interlace_ct3_d4_1x9()[0].pos==19nat,
        ts_interlace_ct3_d4_1x9()[0].before==0nat,
        ts_interlace_ct3_d4_1x9().last().next==173nat,
{
    hide(z_interlace_ct3_d4_1x9);hide(raw_interlace_ct3_d4_1x9);hide(ts_interlace_ct3_d4_1x9);
    assert(raw_interlace_ct3_d4_1x9().len()==18nat) by (compute);
    assert(z_interlace_ct3_d4_1x9().len()==26nat) by (compute);
    assert(z_interlace_ct3_d4_1x9()[0]==120u8) by (compute);
    assert(z_interlace_ct3_d4_1x9()[1]==156u8) by (compute);
    assert(zm::field_value(z_interlace_ct3_d4_1x9(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_interlace_ct3_d4_1x9(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_interlace_ct3_d4_1x9(),22nat,4)==791020633nat) by (compute);
    assert(ts_interlace_ct3_d4_1x9().len()==19nat) by (compute);
    assert(ts_interlace_ct3_d4_1x9()[0].pos==19nat) by (compute);
    assert(ts_interlace_ct3_d4_1x9()[0].before==0nat) by (compute);
    assert(ts_interlace_ct3_d4_1x9().last().next==173nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_interlace_ct3_d4_1x9()
    ensures zm::Adler(raw_interlace_ct3_d4_1x9())==791020633nat,
{
    hide(raw_interlace_ct3_d4_1x9);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_interlace_ct3_d4_1x9().len()==18nat) by (compute);
    assert(c::sums(raw_interlace_ct3_d4_1x9(),0,18nat)==(1112nat,12070nat)) by (compute);
    c::sums_correct(raw_interlace_ct3_d4_1x9(),0,18nat);
    assert(raw_interlace_ct3_d4_1x9().subrange(0,18) =~= raw_interlace_ct3_d4_1x9());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((12070nat%65521)*(u16::MAX as nat+1)+(1+1112nat)%65521==791020633nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_interlace_ct3_d4_1x9()
    ensures r::inflated(z_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_interlace_ct3_d4_1x9(),r2:false}),
        raw_interlace_ct3_d4_1x9().len()==18nat,
{
    hide(z_interlace_ct3_d4_1x9);hide(raw_interlace_ct3_d4_1x9);hide(ts_interlace_ct3_d4_1x9);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_interlace_ct3_d4_1x9();checksum_interlace_ct3_d4_1x9();
    assert(r::raw(a_interlace_ct3_d4_1x9())==18nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_interlace_ct3_d4_1x9();
    stream::fixed_stream(z_interlace_ct3_d4_1x9(),raw_interlace_ct3_d4_1x9(),ts_interlace_ct3_d4_1x9(),18nat,156nat,791020633nat);
}
#[verifier::spinoff_prover]
proof fn scan_interlace_ct3_d4_1x9()
    ensures r::scan(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),0)==Ok(()),
{
    hide(raw_interlace_ct3_d4_1x9);
    assert(r::count(a_interlace_ct3_d4_1x9())==7nat) by (compute);
    r::scan_step(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),7);
    assert(r::pw(a_interlace_ct3_d4_1x9(),6nat)==1nat && r::ph(a_interlace_ct3_d4_1x9(),6nat)==4nat && r::rb(a_interlace_ct3_d4_1x9(),6nat)==1nat && r::base(a_interlace_ct3_d4_1x9(),6nat)==10nat) by (compute);
    assert(raw_interlace_ct3_d4_1x9()[10]==0u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),6nat,0nat),1nat);
    assert(r::row_result(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),6nat,0nat)==Ok(()));
    assert(raw_interlace_ct3_d4_1x9()[12]==1u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),6nat,1nat),1nat);
    assert(r::row_result(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),6nat,1nat)==Ok(()));
    assert(raw_interlace_ct3_d4_1x9()[14]==2u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),6nat,2nat),1nat);
    assert(r::row_result(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),6nat,2nat)==Ok(()));
    assert(raw_interlace_ct3_d4_1x9()[16]==3u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),6nat,3nat),1nat);
    assert(r::row_result(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),6nat,3nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),6nat,4nat);
    r::pass_step(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),6nat,3nat);
    r::pass_step(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),6nat,2nat);
    r::pass_step(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),6nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),6nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),6nat);
    assert(r::pw(a_interlace_ct3_d4_1x9(),5nat)==0nat && r::ph(a_interlace_ct3_d4_1x9(),5nat)==5nat && r::rb(a_interlace_ct3_d4_1x9(),5nat)==0nat && r::base(a_interlace_ct3_d4_1x9(),5nat)==10nat) by (compute);
    r::pass_step(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),5nat,0);
    r::scan_step(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),5nat);
    assert(r::pw(a_interlace_ct3_d4_1x9(),4nat)==1nat && r::ph(a_interlace_ct3_d4_1x9(),4nat)==2nat && r::rb(a_interlace_ct3_d4_1x9(),4nat)==1nat && r::base(a_interlace_ct3_d4_1x9(),4nat)==6nat) by (compute);
    assert(raw_interlace_ct3_d4_1x9()[6]==3u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),4nat,0nat),1nat);
    assert(r::row_result(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),4nat,0nat)==Ok(()));
    assert(raw_interlace_ct3_d4_1x9()[8]==4u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),4nat,1nat),1nat);
    assert(r::row_result(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),4nat,1nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),4nat,2nat);
    r::pass_step(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),4nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),4nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),4nat);
    assert(r::pw(a_interlace_ct3_d4_1x9(),3nat)==0nat && r::ph(a_interlace_ct3_d4_1x9(),3nat)==3nat && r::rb(a_interlace_ct3_d4_1x9(),3nat)==0nat && r::base(a_interlace_ct3_d4_1x9(),3nat)==6nat) by (compute);
    r::pass_step(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),3nat,0);
    r::scan_step(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),3nat);
    assert(r::pw(a_interlace_ct3_d4_1x9(),2nat)==1nat && r::ph(a_interlace_ct3_d4_1x9(),2nat)==1nat && r::rb(a_interlace_ct3_d4_1x9(),2nat)==1nat && r::base(a_interlace_ct3_d4_1x9(),2nat)==4nat) by (compute);
    assert(raw_interlace_ct3_d4_1x9()[4]==2u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),2nat,0nat),1nat);
    assert(r::row_result(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),2nat,0nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),2nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),2nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),2nat);
    assert(r::pw(a_interlace_ct3_d4_1x9(),1nat)==0nat && r::ph(a_interlace_ct3_d4_1x9(),1nat)==2nat && r::rb(a_interlace_ct3_d4_1x9(),1nat)==0nat && r::base(a_interlace_ct3_d4_1x9(),1nat)==4nat) by (compute);
    r::pass_step(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),1nat,0);
    r::scan_step(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),1nat);
    assert(r::pw(a_interlace_ct3_d4_1x9(),0nat)==1nat && r::ph(a_interlace_ct3_d4_1x9(),0nat)==2nat && r::rb(a_interlace_ct3_d4_1x9(),0nat)==1nat && r::base(a_interlace_ct3_d4_1x9(),0nat)==0nat) by (compute);
    assert(raw_interlace_ct3_d4_1x9()[0]==0u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),0nat,0nat),1nat);
    assert(r::row_result(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),0nat,0nat)==Ok(()));
    assert(raw_interlace_ct3_d4_1x9()[2]==1u8) by (compute);
    r::palette_full4(r::line(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),0nat,1nat),1nat);
    assert(r::row_result(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),0nat,1nat)==Ok(()));
    r::pass_step(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),0nat,2nat);
    r::pass_step(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),0nat,1nat);
    r::pass_step(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),0nat,0nat);
    r::scan_step(raw_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_interlace_ct3_d4_1x9()
    ensures r::still(bytes_interlace_ct3_d4_1x9(),STRICT,false)==(r::Projection::Ready {width:1u32,height:9u32,bytes:36nat}),
{
    hide(bytes_interlace_ct3_d4_1x9);hide(z_interlace_ct3_d4_1x9);hide(raw_interlace_ct3_d4_1x9);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);
    assert(bytes_interlace_ct3_d4_1x9().len()==143nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct3_d4_1x9())==Ok((crate::model::Ihdr {width:1u32,height:9u32,depth:4u8,color_type:3u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct3_d4_1x9())==Ok((crate::model::Ihdr {width:1u32,height:9u32,depth:4u8,color_type:3u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 4u8)==16u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_1x9(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_1x9(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(48nat,3u8,4u8,false,false,false,false)==crate::metadata_model::Decision::Keep(16nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_1x9(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:26nat,next:131nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_1x9(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:26nat,next:131nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_1x9(),131nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:139nat,len:0nat,next:143nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_1x9(),131nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:139nat,len:0nat,next:143nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct3_d4_1x9().subrange(101,127)==z_interlace_ct3_d4_1x9()) by (compute);
    walk_interlace_ct3_d4_1x9(bytes_interlace_ct3_d4_1x9());
    inflate_interlace_ct3_d4_1x9();scan_interlace_ct3_d4_1x9();
    assert(r::raw(a_interlace_ct3_d4_1x9())==18nat) by (compute);
    assert(r::data(z_interlace_ct3_d4_1x9(),a_interlace_ct3_d4_1x9(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_interlace_ct3_d4_1x9(),h_interlace_ct3_d4_1x9(),z_interlace_ct3_d4_1x9(),16nat,false);
}
fn public_interlace_ct3_d4_1x9() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,9u8,4u8,3u8,0u8,0u8,1u8,118u8,111u8,107u8,65u8,0u8,0u8,0u8,48u8,80u8,76u8,84u8,69u8,33u8,200u8,29u8,0u8,192u8,84u8,8u8,198u8,36u8,196u8,79u8,63u8,106u8,13u8,88u8,9u8,104u8,32u8,201u8,42u8,74u8,172u8,130u8,213u8,102u8,138u8,204u8,70u8,4u8,100u8,86u8,216u8,216u8,122u8,188u8,187u8,23u8,68u8,5u8,180u8,147u8,135u8,206u8,63u8,86u8,4u8,123u8,180u8,141u8,107u8,80u8,193u8,0u8,0u8,0u8,26u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,192u8,184u8,129u8,169u8,128u8,185u8,129u8,165u8,128u8,193u8,128u8,113u8,2u8,147u8,3u8,179u8,7u8,0u8,47u8,38u8,4u8,89u8,101u8,111u8,27u8,13u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_interlace_ct3_d4_1x9());model_interlace_ct3_d4_1x9();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==1u32 && s.height==9u32 && s.rgba@.len()==36nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// E6-M4-A7-C1 complete bridges; all original M4 certificates above are unchanged.

spec fn e6_a7_colour_interlace_ct0_d1_8x8() -> crate::colour_model::Context {crate::colour_model::Context {ct:0u8,depth:1u8,plte:seq![],trns:None}}
spec fn e6_a7_line_interlace_ct0_d1_8x8_0_0() -> Seq<u8> {seq![128u8]}
#[verifier::spinoff_prover]
proof fn e6_a7_pass_interlace_ct0_d1_8x8_0()
    ensures forall|y: nat| y<1 ==> #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),r::line(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),0nat,y),1nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct0_d1_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8(),0nat,y),
{
    hide(raw_interlace_ct0_d1_8x8);hide(crate::colour_model::pixel);hide(crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8);
    crate::colour_model::depth_values(1u8);crate::colour_model::depth_values(8u8);
    assert(r::pw(a_interlace_ct0_d1_8x8(),0nat)==1nat && r::ph(a_interlace_ct0_d1_8x8(),0nat)==1nat && r::base(a_interlace_ct0_d1_8x8(),0nat)==0nat && r::rb(a_interlace_ct0_d1_8x8(),0nat)==1nat && crate::filter_model::bpp(r::bits(a_interlace_ct0_d1_8x8()))==1nat) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct0_d1_8x8(),0nat,1nat,1nat,0nat);
    assert(raw_interlace_ct0_d1_8x8()[0]==0u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct0_d1_8x8(),0nat,1nat,0nat)==seq![128u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct0_d1_8x8(),0nat,1nat,1nat,0nat)==None);
    assert(crate::filter_model::encoded_row(0u8,seq![128u8],None,1nat,seq![128u8],0,1nat)) by (compute);
    crate::filter_model::checked_row_hint(0u8,seq![128u8],None,1nat,seq![128u8]);
    assert(r::line(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),0nat,0nat)==seq![128u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_0_0(),1nat)[0]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_0_0(),1nat)[1]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_0_0(),1nat)[2]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_0_0(),1nat)[3]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_0_0(),1nat),seq![255u8,255u8,255u8,255u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct0_d1_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8(),0nat,0nat)==seq![255u8,255u8,255u8,255u8]) by (compute);
    assert forall|y: nat| y<1 implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),r::line(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),0nat,y),1nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct0_d1_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8(),0nat,y) by {
        assert(y==0);
    }
}
spec fn e6_a7_line_interlace_ct0_d1_8x8_1_0() -> Seq<u8> {seq![128u8]}
#[verifier::spinoff_prover]
proof fn e6_a7_pass_interlace_ct0_d1_8x8_1()
    ensures forall|y: nat| y<1 ==> #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),r::line(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),1nat,y),1nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct0_d1_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8(),1nat,y),
{
    hide(raw_interlace_ct0_d1_8x8);hide(crate::colour_model::pixel);hide(crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8);
    crate::colour_model::depth_values(1u8);crate::colour_model::depth_values(8u8);
    assert(r::pw(a_interlace_ct0_d1_8x8(),1nat)==1nat && r::ph(a_interlace_ct0_d1_8x8(),1nat)==1nat && r::base(a_interlace_ct0_d1_8x8(),1nat)==2nat && r::rb(a_interlace_ct0_d1_8x8(),1nat)==1nat && crate::filter_model::bpp(r::bits(a_interlace_ct0_d1_8x8()))==1nat) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct0_d1_8x8(),2nat,1nat,1nat,0nat);
    assert(raw_interlace_ct0_d1_8x8()[2]==1u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct0_d1_8x8(),2nat,1nat,0nat)==seq![128u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct0_d1_8x8(),2nat,1nat,1nat,0nat)==None);
    assert(crate::filter_model::encoded_row(1u8,seq![128u8],None,1nat,seq![128u8],0,1nat)) by (compute);
    crate::filter_model::checked_row_hint(1u8,seq![128u8],None,1nat,seq![128u8]);
    assert(r::line(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),1nat,0nat)==seq![128u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_1_0(),1nat)[0]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_1_0(),1nat)[1]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_1_0(),1nat)[2]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_1_0(),1nat)[3]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_1_0(),1nat),seq![255u8,255u8,255u8,255u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct0_d1_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8(),1nat,0nat)==seq![255u8,255u8,255u8,255u8]) by (compute);
    assert forall|y: nat| y<1 implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),r::line(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),1nat,y),1nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct0_d1_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8(),1nat,y) by {
        assert(y==0);
    }
}
spec fn e6_a7_line_interlace_ct0_d1_8x8_2_0() -> Seq<u8> {seq![64u8]}
#[verifier::spinoff_prover]
proof fn e6_a7_pass_interlace_ct0_d1_8x8_2()
    ensures forall|y: nat| y<1 ==> #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),r::line(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),2nat,y),2nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct0_d1_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8(),2nat,y),
{
    hide(raw_interlace_ct0_d1_8x8);hide(crate::colour_model::pixel);hide(crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8);
    crate::colour_model::depth_values(1u8);crate::colour_model::depth_values(8u8);
    assert(r::pw(a_interlace_ct0_d1_8x8(),2nat)==2nat && r::ph(a_interlace_ct0_d1_8x8(),2nat)==1nat && r::base(a_interlace_ct0_d1_8x8(),2nat)==4nat && r::rb(a_interlace_ct0_d1_8x8(),2nat)==1nat && crate::filter_model::bpp(r::bits(a_interlace_ct0_d1_8x8()))==1nat) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct0_d1_8x8(),4nat,1nat,1nat,0nat);
    assert(raw_interlace_ct0_d1_8x8()[4]==2u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct0_d1_8x8(),4nat,1nat,0nat)==seq![64u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct0_d1_8x8(),4nat,1nat,1nat,0nat)==None);
    assert(crate::filter_model::encoded_row(2u8,seq![64u8],None,1nat,seq![64u8],0,1nat)) by (compute);
    crate::filter_model::checked_row_hint(2u8,seq![64u8],None,1nat,seq![64u8]);
    assert(r::line(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),2nat,0nat)==seq![64u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_2_0(),2nat)[0]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_2_0(),2nat)[1]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_2_0(),2nat)[2]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_2_0(),2nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_2_0(),2nat)[4]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_2_0(),2nat)[5]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_2_0(),2nat)[6]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_2_0(),2nat)[7]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_2_0(),2nat),seq![0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct0_d1_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8(),2nat,0nat)==seq![0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8]) by (compute);
    assert forall|y: nat| y<1 implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),r::line(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),2nat,y),2nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct0_d1_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8(),2nat,y) by {
        assert(y==0);
    }
}
spec fn e6_a7_line_interlace_ct0_d1_8x8_3_0() -> Seq<u8> {seq![0u8]}
spec fn e6_a7_line_interlace_ct0_d1_8x8_3_1() -> Seq<u8> {seq![128u8]}
#[verifier::spinoff_prover]
proof fn e6_a7_pass_interlace_ct0_d1_8x8_3()
    ensures forall|y: nat| y<2 ==> #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),r::line(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),3nat,y),2nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct0_d1_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8(),3nat,y),
{
    hide(raw_interlace_ct0_d1_8x8);hide(crate::colour_model::pixel);hide(crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8);
    crate::colour_model::depth_values(1u8);crate::colour_model::depth_values(8u8);
    assert(r::pw(a_interlace_ct0_d1_8x8(),3nat)==2nat && r::ph(a_interlace_ct0_d1_8x8(),3nat)==2nat && r::base(a_interlace_ct0_d1_8x8(),3nat)==6nat && r::rb(a_interlace_ct0_d1_8x8(),3nat)==1nat && crate::filter_model::bpp(r::bits(a_interlace_ct0_d1_8x8()))==1nat) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct0_d1_8x8(),6nat,1nat,1nat,0nat);
    assert(raw_interlace_ct0_d1_8x8()[6]==3u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct0_d1_8x8(),6nat,1nat,0nat)==seq![0u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct0_d1_8x8(),6nat,1nat,1nat,0nat)==None);
    assert(crate::filter_model::encoded_row(3u8,seq![0u8],None,1nat,seq![0u8],0,1nat)) by (compute);
    crate::filter_model::checked_row_hint(3u8,seq![0u8],None,1nat,seq![0u8]);
    assert(r::line(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),3nat,0nat)==seq![0u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_3_0(),2nat)[0]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_3_0(),2nat)[1]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_3_0(),2nat)[2]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_3_0(),2nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_3_0(),2nat)[4]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_3_0(),2nat)[5]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_3_0(),2nat)[6]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_3_0(),2nat)[7]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_3_0(),2nat),seq![0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct0_d1_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8(),3nat,0nat)==seq![0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct0_d1_8x8(),6nat,1nat,1nat,1nat);
    assert(raw_interlace_ct0_d1_8x8()[8]==4u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct0_d1_8x8(),6nat,1nat,1nat)==seq![128u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct0_d1_8x8(),6nat,1nat,1nat,1nat)==Some(seq![0u8]));
    assert(crate::filter_model::encoded_row(4u8,seq![128u8],Some(seq![0u8]),1nat,seq![128u8],0,1nat)) by (compute);
    crate::filter_model::checked_row_hint(4u8,seq![128u8],Some(seq![0u8]),1nat,seq![128u8]);
    assert(r::line(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),3nat,1nat)==seq![128u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_3_1(),2nat)[0]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_3_1(),2nat)[1]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_3_1(),2nat)[2]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_3_1(),2nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_3_1(),2nat)[4]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_3_1(),2nat)[5]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_3_1(),2nat)[6]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_3_1(),2nat)[7]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_3_1(),2nat),seq![255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct0_d1_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8(),3nat,1nat)==seq![255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8]) by (compute);
    assert forall|y: nat| y<2 implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),r::line(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),3nat,y),2nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct0_d1_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8(),3nat,y) by {
        if y==0 {} else {assert(y==1);}
    }
}
spec fn e6_a7_line_interlace_ct0_d1_8x8_4_0() -> Seq<u8> {seq![160u8]}
spec fn e6_a7_line_interlace_ct0_d1_8x8_4_1() -> Seq<u8> {seq![0u8]}
#[verifier::spinoff_prover]
proof fn e6_a7_pass_interlace_ct0_d1_8x8_4()
    ensures forall|y: nat| y<2 ==> #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),r::line(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),4nat,y),4nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct0_d1_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8(),4nat,y),
{
    hide(raw_interlace_ct0_d1_8x8);hide(crate::colour_model::pixel);hide(crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8);
    crate::colour_model::depth_values(1u8);crate::colour_model::depth_values(8u8);
    assert(r::pw(a_interlace_ct0_d1_8x8(),4nat)==4nat && r::ph(a_interlace_ct0_d1_8x8(),4nat)==2nat && r::base(a_interlace_ct0_d1_8x8(),4nat)==10nat && r::rb(a_interlace_ct0_d1_8x8(),4nat)==1nat && crate::filter_model::bpp(r::bits(a_interlace_ct0_d1_8x8()))==1nat) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct0_d1_8x8(),10nat,1nat,1nat,0nat);
    assert(raw_interlace_ct0_d1_8x8()[10]==0u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct0_d1_8x8(),10nat,1nat,0nat)==seq![160u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct0_d1_8x8(),10nat,1nat,1nat,0nat)==None);
    assert(crate::filter_model::encoded_row(0u8,seq![160u8],None,1nat,seq![160u8],0,1nat)) by (compute);
    crate::filter_model::checked_row_hint(0u8,seq![160u8],None,1nat,seq![160u8]);
    assert(r::line(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),4nat,0nat)==seq![160u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_4_0(),4nat)[0]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_4_0(),4nat)[1]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_4_0(),4nat)[2]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_4_0(),4nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_4_0(),4nat)[4]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_4_0(),4nat)[5]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_4_0(),4nat)[6]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_4_0(),4nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_4_0(),4nat)[8]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_4_0(),4nat)[9]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_4_0(),4nat)[10]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_4_0(),4nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_4_0(),4nat)[12]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_4_0(),4nat)[13]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_4_0(),4nat)[14]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_4_0(),4nat)[15]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_4_0(),4nat),seq![255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct0_d1_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8(),4nat,0nat)==seq![255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct0_d1_8x8(),10nat,1nat,1nat,1nat);
    assert(raw_interlace_ct0_d1_8x8()[12]==1u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct0_d1_8x8(),10nat,1nat,1nat)==seq![0u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct0_d1_8x8(),10nat,1nat,1nat,1nat)==Some(seq![160u8]));
    assert(crate::filter_model::encoded_row(1u8,seq![0u8],Some(seq![160u8]),1nat,seq![0u8],0,1nat)) by (compute);
    crate::filter_model::checked_row_hint(1u8,seq![0u8],Some(seq![160u8]),1nat,seq![0u8]);
    assert(r::line(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),4nat,1nat)==seq![0u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_4_1(),4nat)[0]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_4_1(),4nat)[1]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_4_1(),4nat)[2]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_4_1(),4nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_4_1(),4nat)[4]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_4_1(),4nat)[5]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_4_1(),4nat)[6]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_4_1(),4nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_4_1(),4nat)[8]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_4_1(),4nat)[9]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_4_1(),4nat)[10]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_4_1(),4nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_4_1(),4nat)[12]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_4_1(),4nat)[13]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_4_1(),4nat)[14]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_4_1(),4nat)[15]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_4_1(),4nat),seq![0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct0_d1_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8(),4nat,1nat)==seq![0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8]) by (compute);
    assert forall|y: nat| y<2 implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),r::line(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),4nat,y),4nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct0_d1_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8(),4nat,y) by {
        if y==0 {} else {assert(y==1);}
    }
}
spec fn e6_a7_line_interlace_ct0_d1_8x8_5_0() -> Seq<u8> {seq![128u8]}
spec fn e6_a7_line_interlace_ct0_d1_8x8_5_1() -> Seq<u8> {seq![128u8]}
spec fn e6_a7_line_interlace_ct0_d1_8x8_5_2() -> Seq<u8> {seq![64u8]}
spec fn e6_a7_line_interlace_ct0_d1_8x8_5_3() -> Seq<u8> {seq![112u8]}
#[verifier::spinoff_prover]
proof fn e6_a7_pass_interlace_ct0_d1_8x8_5()
    ensures forall|y: nat| y<4 ==> #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),r::line(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),5nat,y),4nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct0_d1_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8(),5nat,y),
{
    hide(raw_interlace_ct0_d1_8x8);hide(crate::colour_model::pixel);hide(crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8);
    crate::colour_model::depth_values(1u8);crate::colour_model::depth_values(8u8);
    assert(r::pw(a_interlace_ct0_d1_8x8(),5nat)==4nat && r::ph(a_interlace_ct0_d1_8x8(),5nat)==4nat && r::base(a_interlace_ct0_d1_8x8(),5nat)==14nat && r::rb(a_interlace_ct0_d1_8x8(),5nat)==1nat && crate::filter_model::bpp(r::bits(a_interlace_ct0_d1_8x8()))==1nat) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct0_d1_8x8(),14nat,1nat,1nat,0nat);
    assert(raw_interlace_ct0_d1_8x8()[14]==2u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct0_d1_8x8(),14nat,1nat,0nat)==seq![128u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct0_d1_8x8(),14nat,1nat,1nat,0nat)==None);
    assert(crate::filter_model::encoded_row(2u8,seq![128u8],None,1nat,seq![128u8],0,1nat)) by (compute);
    crate::filter_model::checked_row_hint(2u8,seq![128u8],None,1nat,seq![128u8]);
    assert(r::line(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),5nat,0nat)==seq![128u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_0(),4nat)[0]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_0(),4nat)[1]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_0(),4nat)[2]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_0(),4nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_0(),4nat)[4]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_0(),4nat)[5]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_0(),4nat)[6]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_0(),4nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_0(),4nat)[8]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_0(),4nat)[9]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_0(),4nat)[10]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_0(),4nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_0(),4nat)[12]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_0(),4nat)[13]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_0(),4nat)[14]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_0(),4nat)[15]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_0(),4nat),seq![255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct0_d1_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8(),5nat,0nat)==seq![255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct0_d1_8x8(),14nat,1nat,1nat,1nat);
    assert(raw_interlace_ct0_d1_8x8()[16]==3u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct0_d1_8x8(),14nat,1nat,1nat)==seq![64u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct0_d1_8x8(),14nat,1nat,1nat,1nat)==Some(seq![128u8]));
    assert(crate::filter_model::encoded_row(3u8,seq![64u8],Some(seq![128u8]),1nat,seq![128u8],0,1nat)) by (compute);
    crate::filter_model::checked_row_hint(3u8,seq![64u8],Some(seq![128u8]),1nat,seq![128u8]);
    assert(r::line(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),5nat,1nat)==seq![128u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_1(),4nat)[0]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_1(),4nat)[1]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_1(),4nat)[2]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_1(),4nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_1(),4nat)[4]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_1(),4nat)[5]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_1(),4nat)[6]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_1(),4nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_1(),4nat)[8]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_1(),4nat)[9]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_1(),4nat)[10]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_1(),4nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_1(),4nat)[12]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_1(),4nat)[13]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_1(),4nat)[14]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_1(),4nat)[15]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_1(),4nat),seq![255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct0_d1_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8(),5nat,1nat)==seq![255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct0_d1_8x8(),14nat,1nat,1nat,2nat);
    assert(raw_interlace_ct0_d1_8x8()[18]==4u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct0_d1_8x8(),14nat,1nat,2nat)==seq![192u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct0_d1_8x8(),14nat,1nat,1nat,2nat)==Some(seq![128u8]));
    assert(crate::filter_model::encoded_row(4u8,seq![192u8],Some(seq![128u8]),1nat,seq![64u8],0,1nat)) by (compute);
    crate::filter_model::checked_row_hint(4u8,seq![192u8],Some(seq![128u8]),1nat,seq![64u8]);
    assert(r::line(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),5nat,2nat)==seq![64u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_2(),4nat)[0]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_2(),4nat)[1]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_2(),4nat)[2]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_2(),4nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_2(),4nat)[4]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_2(),4nat)[5]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_2(),4nat)[6]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_2(),4nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_2(),4nat)[8]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_2(),4nat)[9]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_2(),4nat)[10]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_2(),4nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_2(),4nat)[12]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_2(),4nat)[13]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_2(),4nat)[14]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_2(),4nat)[15]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_2(),4nat),seq![0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct0_d1_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8(),5nat,2nat)==seq![0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct0_d1_8x8(),14nat,1nat,1nat,3nat);
    assert(raw_interlace_ct0_d1_8x8()[20]==0u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct0_d1_8x8(),14nat,1nat,3nat)==seq![112u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct0_d1_8x8(),14nat,1nat,1nat,3nat)==Some(seq![64u8]));
    assert(crate::filter_model::encoded_row(0u8,seq![112u8],Some(seq![64u8]),1nat,seq![112u8],0,1nat)) by (compute);
    crate::filter_model::checked_row_hint(0u8,seq![112u8],Some(seq![64u8]),1nat,seq![112u8]);
    assert(r::line(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),5nat,3nat)==seq![112u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_3(),4nat)[0]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_3(),4nat)[1]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_3(),4nat)[2]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_3(),4nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_3(),4nat)[4]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_3(),4nat)[5]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_3(),4nat)[6]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_3(),4nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_3(),4nat)[8]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_3(),4nat)[9]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_3(),4nat)[10]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_3(),4nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_3(),4nat)[12]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_3(),4nat)[13]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_3(),4nat)[14]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_3(),4nat)[15]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_5_3(),4nat),seq![0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct0_d1_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8(),5nat,3nat)==seq![0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8]) by (compute);
    assert forall|y: nat| y<4 implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),r::line(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),5nat,y),4nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct0_d1_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8(),5nat,y) by {
        if y==0 {} else if y==1 {} else if y==2 {} else {assert(y==3);}
    }
}
spec fn e6_a7_line_interlace_ct0_d1_8x8_6_0() -> Seq<u8> {seq![16u8]}
spec fn e6_a7_line_interlace_ct0_d1_8x8_6_1() -> Seq<u8> {seq![118u8]}
spec fn e6_a7_line_interlace_ct0_d1_8x8_6_2() -> Seq<u8> {seq![242u8]}
spec fn e6_a7_line_interlace_ct0_d1_8x8_6_3() -> Seq<u8> {seq![94u8]}
#[verifier::spinoff_prover]
proof fn e6_a7_pass_interlace_ct0_d1_8x8_6()
    ensures forall|y: nat| y<4 ==> #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),r::line(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),6nat,y),8nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct0_d1_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8(),6nat,y),
{
    hide(raw_interlace_ct0_d1_8x8);hide(crate::colour_model::pixel);hide(crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8);
    crate::colour_model::depth_values(1u8);crate::colour_model::depth_values(8u8);
    assert(r::pw(a_interlace_ct0_d1_8x8(),6nat)==8nat && r::ph(a_interlace_ct0_d1_8x8(),6nat)==4nat && r::base(a_interlace_ct0_d1_8x8(),6nat)==22nat && r::rb(a_interlace_ct0_d1_8x8(),6nat)==1nat && crate::filter_model::bpp(r::bits(a_interlace_ct0_d1_8x8()))==1nat) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct0_d1_8x8(),22nat,1nat,1nat,0nat);
    assert(raw_interlace_ct0_d1_8x8()[22]==1u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct0_d1_8x8(),22nat,1nat,0nat)==seq![16u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct0_d1_8x8(),22nat,1nat,1nat,0nat)==None);
    assert(crate::filter_model::encoded_row(1u8,seq![16u8],None,1nat,seq![16u8],0,1nat)) by (compute);
    crate::filter_model::checked_row_hint(1u8,seq![16u8],None,1nat,seq![16u8]);
    assert(r::line(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),6nat,0nat)==seq![16u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_0(),8nat)[0]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_0(),8nat)[1]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_0(),8nat)[2]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_0(),8nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_0(),8nat)[4]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_0(),8nat)[5]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_0(),8nat)[6]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_0(),8nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_0(),8nat)[8]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_0(),8nat)[9]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_0(),8nat)[10]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_0(),8nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_0(),8nat)[12]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_0(),8nat)[13]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_0(),8nat)[14]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_0(),8nat)[15]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_0(),8nat)[16]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_0(),8nat)[17]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_0(),8nat)[18]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_0(),8nat)[19]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_0(),8nat)[20]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_0(),8nat)[21]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_0(),8nat)[22]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_0(),8nat)[23]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_0(),8nat)[24]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_0(),8nat)[25]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_0(),8nat)[26]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_0(),8nat)[27]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_0(),8nat)[28]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_0(),8nat)[29]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_0(),8nat)[30]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_0(),8nat)[31]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_0(),8nat),seq![0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct0_d1_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8(),6nat,0nat)==seq![0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct0_d1_8x8(),22nat,1nat,1nat,1nat);
    assert(raw_interlace_ct0_d1_8x8()[24]==2u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct0_d1_8x8(),22nat,1nat,1nat)==seq![102u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct0_d1_8x8(),22nat,1nat,1nat,1nat)==Some(seq![16u8]));
    assert(crate::filter_model::encoded_row(2u8,seq![102u8],Some(seq![16u8]),1nat,seq![118u8],0,1nat)) by (compute);
    crate::filter_model::checked_row_hint(2u8,seq![102u8],Some(seq![16u8]),1nat,seq![118u8]);
    assert(r::line(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),6nat,1nat)==seq![118u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_1(),8nat)[0]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_1(),8nat)[1]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_1(),8nat)[2]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_1(),8nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_1(),8nat)[4]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_1(),8nat)[5]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_1(),8nat)[6]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_1(),8nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_1(),8nat)[8]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_1(),8nat)[9]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_1(),8nat)[10]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_1(),8nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_1(),8nat)[12]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_1(),8nat)[13]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_1(),8nat)[14]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_1(),8nat)[15]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_1(),8nat)[16]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_1(),8nat)[17]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_1(),8nat)[18]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_1(),8nat)[19]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_1(),8nat)[20]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_1(),8nat)[21]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_1(),8nat)[22]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_1(),8nat)[23]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_1(),8nat)[24]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_1(),8nat)[25]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_1(),8nat)[26]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_1(),8nat)[27]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_1(),8nat)[28]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_1(),8nat)[29]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_1(),8nat)[30]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_1(),8nat)[31]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_1(),8nat),seq![0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct0_d1_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8(),6nat,1nat)==seq![0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct0_d1_8x8(),22nat,1nat,1nat,2nat);
    assert(raw_interlace_ct0_d1_8x8()[26]==3u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct0_d1_8x8(),22nat,1nat,2nat)==seq![183u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct0_d1_8x8(),22nat,1nat,1nat,2nat)==Some(seq![118u8]));
    assert(crate::filter_model::encoded_row(3u8,seq![183u8],Some(seq![118u8]),1nat,seq![242u8],0,1nat)) by (compute);
    crate::filter_model::checked_row_hint(3u8,seq![183u8],Some(seq![118u8]),1nat,seq![242u8]);
    assert(r::line(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),6nat,2nat)==seq![242u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_2(),8nat)[0]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_2(),8nat)[1]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_2(),8nat)[2]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_2(),8nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_2(),8nat)[4]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_2(),8nat)[5]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_2(),8nat)[6]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_2(),8nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_2(),8nat)[8]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_2(),8nat)[9]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_2(),8nat)[10]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_2(),8nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_2(),8nat)[12]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_2(),8nat)[13]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_2(),8nat)[14]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_2(),8nat)[15]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_2(),8nat)[16]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_2(),8nat)[17]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_2(),8nat)[18]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_2(),8nat)[19]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_2(),8nat)[20]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_2(),8nat)[21]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_2(),8nat)[22]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_2(),8nat)[23]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_2(),8nat)[24]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_2(),8nat)[25]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_2(),8nat)[26]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_2(),8nat)[27]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_2(),8nat)[28]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_2(),8nat)[29]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_2(),8nat)[30]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_2(),8nat)[31]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_2(),8nat),seq![255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct0_d1_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8(),6nat,2nat)==seq![255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct0_d1_8x8(),22nat,1nat,1nat,3nat);
    assert(raw_interlace_ct0_d1_8x8()[28]==4u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct0_d1_8x8(),22nat,1nat,3nat)==seq![108u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct0_d1_8x8(),22nat,1nat,1nat,3nat)==Some(seq![242u8]));
    assert(crate::filter_model::encoded_row(4u8,seq![108u8],Some(seq![242u8]),1nat,seq![94u8],0,1nat)) by (compute);
    crate::filter_model::checked_row_hint(4u8,seq![108u8],Some(seq![242u8]),1nat,seq![94u8]);
    assert(r::line(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),6nat,3nat)==seq![94u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_3(),8nat)[0]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_3(),8nat)[1]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_3(),8nat)[2]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_3(),8nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_3(),8nat)[4]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_3(),8nat)[5]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_3(),8nat)[6]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_3(),8nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_3(),8nat)[8]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_3(),8nat)[9]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_3(),8nat)[10]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_3(),8nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_3(),8nat)[12]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_3(),8nat)[13]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_3(),8nat)[14]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_3(),8nat)[15]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_3(),8nat)[16]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_3(),8nat)[17]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_3(),8nat)[18]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_3(),8nat)[19]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_3(),8nat)[20]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_3(),8nat)[21]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_3(),8nat)[22]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_3(),8nat)[23]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_3(),8nat)[24]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_3(),8nat)[25]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_3(),8nat)[26]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_3(),8nat)[27]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_3(),8nat)[28]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_3(),8nat)[29]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_3(),8nat)[30]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_3(),8nat)[31]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),e6_a7_line_interlace_ct0_d1_8x8_6_3(),8nat),seq![0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct0_d1_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8(),6nat,3nat)==seq![0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8]) by (compute);
    assert forall|y: nat| y<4 implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),r::line(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),6nat,y),8nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct0_d1_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8(),6nat,y) by {
        if y==0 {} else if y==1 {} else if y==2 {} else {assert(y==3);}
    }
}
#[verifier::spinoff_prover]
pub(crate) proof fn e6_a7_complete_interlace_ct0_d1_8x8()
    ensures match crate::still_model::MS(crate::witness_m6_m4_a7::bytes_interlace_ct0_d1_8x8()) {crate::still_model::Outcome::Done(value)=>crate::witness_m6_m4_a7::expected_value_interlace_ct0_d1_8x8(value),_=>false},
{

    hide(bytes_interlace_ct0_d1_8x8);hide(z_interlace_ct0_d1_8x8);hide(raw_interlace_ct0_d1_8x8);hide(r::data);hide(crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_interlace_ct0_d1_8x8().len()==95nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct0_d1_8x8())==Ok((crate::model::Ihdr {width:8u32,height:8u32,depth:1u8,color_type:0u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct0_d1_8x8())==Ok((crate::model::Ihdr {width:8u32,height:8u32,depth:1u8,color_type:0u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 1u8)==2u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct0_d1_8x8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:38nat,next:83nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct0_d1_8x8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:38nat,next:83nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct0_d1_8x8(),83nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:91nat,len:0nat,next:95nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct0_d1_8x8(),83nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:91nat,len:0nat,next:95nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct0_d1_8x8().subrange(41,79)==z_interlace_ct0_d1_8x8()) by (compute);
    walk_interlace_ct0_d1_8x8(bytes_interlace_ct0_d1_8x8());

    model_interlace_ct0_d1_8x8();inflate_interlace_ct0_d1_8x8();scan_interlace_ct0_d1_8x8();
    assert(bytes_interlace_ct0_d1_8x8()==crate::witness_m6_m4_a7::bytes_interlace_ct0_d1_8x8()) by (compute);
    assert(r::raw(a_interlace_ct0_d1_8x8())==30nat) by (compute);
    assert(r::data(z_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    crate::witness_m6_rows_model::bare_colour(bytes_interlace_ct0_d1_8x8(),h_interlace_ct0_d1_8x8(),38nat);
    e6_a7_pass_interlace_ct0_d1_8x8_0();
    e6_a7_pass_interlace_ct0_d1_8x8_1();
    e6_a7_pass_interlace_ct0_d1_8x8_2();
    e6_a7_pass_interlace_ct0_d1_8x8_3();
    e6_a7_pass_interlace_ct0_d1_8x8_4();
    e6_a7_pass_interlace_ct0_d1_8x8_5();
    e6_a7_pass_interlace_ct0_d1_8x8_6();
    assert(r::count(a_interlace_ct0_d1_8x8())==7nat && crate::paint_model::domain(a_interlace_ct0_d1_8x8()) && crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8().len()==256nat) by (compute);
    assert(r::pw(a_interlace_ct0_d1_8x8(),0nat)==1nat && r::ph(a_interlace_ct0_d1_8x8(),0nat)==1nat) by (compute);
    assert(r::pw(a_interlace_ct0_d1_8x8(),1nat)==1nat && r::ph(a_interlace_ct0_d1_8x8(),1nat)==1nat) by (compute);
    assert(r::pw(a_interlace_ct0_d1_8x8(),2nat)==2nat && r::ph(a_interlace_ct0_d1_8x8(),2nat)==1nat) by (compute);
    assert(r::pw(a_interlace_ct0_d1_8x8(),3nat)==2nat && r::ph(a_interlace_ct0_d1_8x8(),3nat)==2nat) by (compute);
    assert(r::pw(a_interlace_ct0_d1_8x8(),4nat)==4nat && r::ph(a_interlace_ct0_d1_8x8(),4nat)==2nat) by (compute);
    assert(r::pw(a_interlace_ct0_d1_8x8(),5nat)==4nat && r::ph(a_interlace_ct0_d1_8x8(),5nat)==4nat) by (compute);
    assert(r::pw(a_interlace_ct0_d1_8x8(),6nat)==8nat && r::ph(a_interlace_ct0_d1_8x8(),6nat)==4nat) by (compute);
    assert forall|k: nat,y: nat| k<7 && r::pw(a_interlace_ct0_d1_8x8(),k)>0 && y<r::ph(a_interlace_ct0_d1_8x8(),k) implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct0_d1_8x8(),r::line(raw_interlace_ct0_d1_8x8(),a_interlace_ct0_d1_8x8(),k,y),r::pw(a_interlace_ct0_d1_8x8(),k))==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct0_d1_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8(),k,y) by {
        if k==0 {} else if k==1 {} else if k==2 {} else if k==3 {} else if k==4 {} else if k==5 {} else {assert(k==6);}
    }
    crate::witness_m6_a7_model::complete_still(bytes_interlace_ct0_d1_8x8(),h_interlace_ct0_d1_8x8(),z_interlace_ct0_d1_8x8(),raw_interlace_ct0_d1_8x8(),0nat,e6_a7_colour_interlace_ct0_d1_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct0_d1_8x8());
}
spec fn e6_a7_colour_interlace_ct6_d16_8x8() -> crate::colour_model::Context {crate::colour_model::Context {ct:6u8,depth:16u8,plte:seq![],trns:None}}
spec fn e6_a7_line_interlace_ct6_d16_8x8_0_0() -> Seq<u8> {seq![74u8,135u8,183u8,180u8,49u8,71u8,131u8,229u8]}
#[verifier::spinoff_prover]
proof fn e6_a7_pass_interlace_ct6_d16_8x8_0()
    ensures forall|y: nat| y<1 ==> #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),r::line(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),0nat,y),1nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8(),0nat,y),
{
    hide(raw_interlace_ct6_d16_8x8);hide(crate::colour_model::pixel);hide(crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8);
    crate::colour_model::depth_values(16u8);crate::colour_model::depth_values(8u8);
    assert(r::pw(a_interlace_ct6_d16_8x8(),0nat)==1nat && r::ph(a_interlace_ct6_d16_8x8(),0nat)==1nat && r::base(a_interlace_ct6_d16_8x8(),0nat)==0nat && r::rb(a_interlace_ct6_d16_8x8(),0nat)==8nat && crate::filter_model::bpp(r::bits(a_interlace_ct6_d16_8x8()))==8nat) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct6_d16_8x8(),0nat,8nat,8nat,0nat);
    assert(raw_interlace_ct6_d16_8x8()[0]==0u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct6_d16_8x8(),0nat,8nat,0nat)==seq![74u8,135u8,183u8,180u8,49u8,71u8,131u8,229u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct6_d16_8x8(),0nat,8nat,8nat,0nat)==None);
    assert(crate::filter_model::encoded_row(0u8,seq![74u8,135u8,183u8,180u8,49u8,71u8,131u8,229u8],None,8nat,seq![74u8,135u8,183u8,180u8,49u8,71u8,131u8,229u8],0,8nat)) by (compute);
    crate::filter_model::checked_row_hint(0u8,seq![74u8,135u8,183u8,180u8,49u8,71u8,131u8,229u8],None,8nat,seq![74u8,135u8,183u8,180u8,49u8,71u8,131u8,229u8]);
    assert(r::line(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),0nat,0nat)==seq![74u8,135u8,183u8,180u8,49u8,71u8,131u8,229u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_0_0(),1nat)[0]==74u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_0_0(),1nat)[1]==183u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_0_0(),1nat)[2]==49u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_0_0(),1nat)[3]==131u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_0_0(),1nat),seq![74u8,183u8,49u8,131u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8(),0nat,0nat)==seq![74u8,183u8,49u8,131u8]) by (compute);
    assert forall|y: nat| y<1 implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),r::line(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),0nat,y),1nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8(),0nat,y) by {
        assert(y==0);
    }
}
spec fn e6_a7_line_interlace_ct6_d16_8x8_1_0() -> Seq<u8> {seq![90u8,155u8,171u8,67u8,219u8,123u8,213u8,78u8]}
#[verifier::spinoff_prover]
proof fn e6_a7_pass_interlace_ct6_d16_8x8_1()
    ensures forall|y: nat| y<1 ==> #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),r::line(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),1nat,y),1nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8(),1nat,y),
{
    hide(raw_interlace_ct6_d16_8x8);hide(crate::colour_model::pixel);hide(crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8);
    crate::colour_model::depth_values(16u8);crate::colour_model::depth_values(8u8);
    assert(r::pw(a_interlace_ct6_d16_8x8(),1nat)==1nat && r::ph(a_interlace_ct6_d16_8x8(),1nat)==1nat && r::base(a_interlace_ct6_d16_8x8(),1nat)==9nat && r::rb(a_interlace_ct6_d16_8x8(),1nat)==8nat && crate::filter_model::bpp(r::bits(a_interlace_ct6_d16_8x8()))==8nat) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct6_d16_8x8(),9nat,8nat,8nat,0nat);
    assert(raw_interlace_ct6_d16_8x8()[9]==1u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct6_d16_8x8(),9nat,8nat,0nat)==seq![90u8,155u8,171u8,67u8,219u8,123u8,213u8,78u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct6_d16_8x8(),9nat,8nat,8nat,0nat)==None);
    assert(crate::filter_model::encoded_row(1u8,seq![90u8,155u8,171u8,67u8,219u8,123u8,213u8,78u8],None,8nat,seq![90u8,155u8,171u8,67u8,219u8,123u8,213u8,78u8],0,8nat)) by (compute);
    crate::filter_model::checked_row_hint(1u8,seq![90u8,155u8,171u8,67u8,219u8,123u8,213u8,78u8],None,8nat,seq![90u8,155u8,171u8,67u8,219u8,123u8,213u8,78u8]);
    assert(r::line(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),1nat,0nat)==seq![90u8,155u8,171u8,67u8,219u8,123u8,213u8,78u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_1_0(),1nat)[0]==90u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_1_0(),1nat)[1]==171u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_1_0(),1nat)[2]==219u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_1_0(),1nat)[3]==212u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_1_0(),1nat),seq![90u8,171u8,219u8,212u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8(),1nat,0nat)==seq![90u8,171u8,219u8,212u8]) by (compute);
    assert forall|y: nat| y<1 implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),r::line(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),1nat,y),1nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8(),1nat,y) by {
        assert(y==0);
    }
}
spec fn e6_a7_line_interlace_ct6_d16_8x8_2_0() -> Seq<u8> {seq![57u8,70u8,159u8,194u8,106u8,221u8,6u8,19u8,62u8,67u8,91u8,207u8,143u8,40u8,31u8,209u8]}
#[verifier::spinoff_prover]
proof fn e6_a7_pass_interlace_ct6_d16_8x8_2()
    ensures forall|y: nat| y<1 ==> #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),r::line(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),2nat,y),2nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8(),2nat,y),
{
    hide(raw_interlace_ct6_d16_8x8);hide(crate::colour_model::pixel);hide(crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8);
    crate::colour_model::depth_values(16u8);crate::colour_model::depth_values(8u8);
    assert(r::pw(a_interlace_ct6_d16_8x8(),2nat)==2nat && r::ph(a_interlace_ct6_d16_8x8(),2nat)==1nat && r::base(a_interlace_ct6_d16_8x8(),2nat)==18nat && r::rb(a_interlace_ct6_d16_8x8(),2nat)==16nat && crate::filter_model::bpp(r::bits(a_interlace_ct6_d16_8x8()))==8nat) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct6_d16_8x8(),18nat,16nat,8nat,0nat);
    assert(raw_interlace_ct6_d16_8x8()[18]==2u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct6_d16_8x8(),18nat,16nat,0nat)==seq![57u8,70u8,159u8,194u8,106u8,221u8,6u8,19u8,62u8,67u8,91u8,207u8,143u8,40u8,31u8,209u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct6_d16_8x8(),18nat,16nat,8nat,0nat)==None);
    assert(crate::filter_model::encoded_row(2u8,seq![57u8,70u8,159u8,194u8,106u8,221u8,6u8,19u8,62u8,67u8,91u8,207u8,143u8,40u8,31u8,209u8],None,8nat,seq![57u8,70u8,159u8,194u8,106u8,221u8,6u8,19u8,62u8,67u8,91u8,207u8,143u8,40u8,31u8,209u8],0,16nat)) by (compute);
    crate::filter_model::checked_row_hint(2u8,seq![57u8,70u8,159u8,194u8,106u8,221u8,6u8,19u8,62u8,67u8,91u8,207u8,143u8,40u8,31u8,209u8],None,8nat,seq![57u8,70u8,159u8,194u8,106u8,221u8,6u8,19u8,62u8,67u8,91u8,207u8,143u8,40u8,31u8,209u8]);
    assert(r::line(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),2nat,0nat)==seq![57u8,70u8,159u8,194u8,106u8,221u8,6u8,19u8,62u8,67u8,91u8,207u8,143u8,40u8,31u8,209u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_2_0(),2nat)[0]==57u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_2_0(),2nat)[1]==159u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_2_0(),2nat)[2]==106u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_2_0(),2nat)[3]==6u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_2_0(),2nat)[4]==62u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_2_0(),2nat)[5]==91u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_2_0(),2nat)[6]==143u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_2_0(),2nat)[7]==32u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_2_0(),2nat),seq![57u8,159u8,106u8,6u8,62u8,91u8,143u8,32u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8(),2nat,0nat)==seq![57u8,159u8,106u8,6u8,62u8,91u8,143u8,32u8]) by (compute);
    assert forall|y: nat| y<1 implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),r::line(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),2nat,y),2nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8(),2nat,y) by {
        assert(y==0);
    }
}
spec fn e6_a7_line_interlace_ct6_d16_8x8_3_0() -> Seq<u8> {seq![253u8,150u8,22u8,157u8,0u8,42u8,136u8,97u8,186u8,141u8,53u8,79u8,105u8,16u8,92u8,189u8]}
spec fn e6_a7_line_interlace_ct6_d16_8x8_3_1() -> Seq<u8> {seq![11u8,171u8,252u8,15u8,152u8,245u8,190u8,168u8,58u8,200u8,201u8,198u8,7u8,11u8,9u8,232u8]}
#[verifier::spinoff_prover]
proof fn e6_a7_pass_interlace_ct6_d16_8x8_3()
    ensures forall|y: nat| y<2 ==> #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),r::line(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),3nat,y),2nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8(),3nat,y),
{
    hide(raw_interlace_ct6_d16_8x8);hide(crate::colour_model::pixel);hide(crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8);
    crate::colour_model::depth_values(16u8);crate::colour_model::depth_values(8u8);
    assert(r::pw(a_interlace_ct6_d16_8x8(),3nat)==2nat && r::ph(a_interlace_ct6_d16_8x8(),3nat)==2nat && r::base(a_interlace_ct6_d16_8x8(),3nat)==35nat && r::rb(a_interlace_ct6_d16_8x8(),3nat)==16nat && crate::filter_model::bpp(r::bits(a_interlace_ct6_d16_8x8()))==8nat) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct6_d16_8x8(),35nat,16nat,8nat,0nat);
    assert(raw_interlace_ct6_d16_8x8()[35]==3u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct6_d16_8x8(),35nat,16nat,0nat)==seq![253u8,150u8,22u8,157u8,0u8,42u8,136u8,97u8,60u8,66u8,42u8,1u8,105u8,251u8,24u8,141u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct6_d16_8x8(),35nat,16nat,8nat,0nat)==None);
    assert(crate::filter_model::encoded_row(3u8,seq![253u8,150u8,22u8,157u8,0u8,42u8,136u8,97u8,60u8,66u8,42u8,1u8,105u8,251u8,24u8,141u8],None,8nat,seq![253u8,150u8,22u8,157u8,0u8,42u8,136u8,97u8,186u8,141u8,53u8,79u8,105u8,16u8,92u8,189u8],0,16nat)) by (compute);
    crate::filter_model::checked_row_hint(3u8,seq![253u8,150u8,22u8,157u8,0u8,42u8,136u8,97u8,60u8,66u8,42u8,1u8,105u8,251u8,24u8,141u8],None,8nat,seq![253u8,150u8,22u8,157u8,0u8,42u8,136u8,97u8,186u8,141u8,53u8,79u8,105u8,16u8,92u8,189u8]);
    assert(r::line(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),3nat,0nat)==seq![253u8,150u8,22u8,157u8,0u8,42u8,136u8,97u8,186u8,141u8,53u8,79u8,105u8,16u8,92u8,189u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_3_0(),2nat)[0]==253u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_3_0(),2nat)[1]==23u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_3_0(),2nat)[2]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_3_0(),2nat)[3]==136u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_3_0(),2nat)[4]==186u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_3_0(),2nat)[5]==53u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_3_0(),2nat)[6]==105u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_3_0(),2nat)[7]==92u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_3_0(),2nat),seq![253u8,23u8,0u8,136u8,186u8,53u8,105u8,92u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8(),3nat,0nat)==seq![253u8,23u8,0u8,136u8,186u8,53u8,105u8,92u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct6_d16_8x8(),35nat,16nat,8nat,1nat);
    assert(raw_interlace_ct6_d16_8x8()[52]==4u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct6_d16_8x8(),35nat,16nat,1nat)==seq![14u8,21u8,230u8,114u8,152u8,203u8,54u8,71u8,47u8,29u8,205u8,183u8,111u8,22u8,129u8,43u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct6_d16_8x8(),35nat,16nat,8nat,1nat)==Some(seq![253u8,150u8,22u8,157u8,0u8,42u8,136u8,97u8,186u8,141u8,53u8,79u8,105u8,16u8,92u8,189u8]));
    assert(crate::filter_model::encoded_row(4u8,seq![14u8,21u8,230u8,114u8,152u8,203u8,54u8,71u8,47u8,29u8,205u8,183u8,111u8,22u8,129u8,43u8],Some(seq![253u8,150u8,22u8,157u8,0u8,42u8,136u8,97u8,186u8,141u8,53u8,79u8,105u8,16u8,92u8,189u8]),8nat,seq![11u8,171u8,252u8,15u8,152u8,245u8,190u8,168u8,58u8,200u8,201u8,198u8,7u8,11u8,9u8,232u8],0,16nat)) by (compute);
    crate::filter_model::checked_row_hint(4u8,seq![14u8,21u8,230u8,114u8,152u8,203u8,54u8,71u8,47u8,29u8,205u8,183u8,111u8,22u8,129u8,43u8],Some(seq![253u8,150u8,22u8,157u8,0u8,42u8,136u8,97u8,186u8,141u8,53u8,79u8,105u8,16u8,92u8,189u8]),8nat,seq![11u8,171u8,252u8,15u8,152u8,245u8,190u8,168u8,58u8,200u8,201u8,198u8,7u8,11u8,9u8,232u8]);
    assert(r::line(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),3nat,1nat)==seq![11u8,171u8,252u8,15u8,152u8,245u8,190u8,168u8,58u8,200u8,201u8,198u8,7u8,11u8,9u8,232u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_3_1(),2nat)[0]==12u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_3_1(),2nat)[1]==251u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_3_1(),2nat)[2]==152u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_3_1(),2nat)[3]==190u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_3_1(),2nat)[4]==59u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_3_1(),2nat)[5]==201u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_3_1(),2nat)[6]==7u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_3_1(),2nat)[7]==10u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_3_1(),2nat),seq![12u8,251u8,152u8,190u8,59u8,201u8,7u8,10u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8(),3nat,1nat)==seq![12u8,251u8,152u8,190u8,59u8,201u8,7u8,10u8]) by (compute);
    assert forall|y: nat| y<2 implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),r::line(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),3nat,y),2nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8(),3nat,y) by {
        if y==0 {} else {assert(y==1);}
    }
}
spec fn e6_a7_line_interlace_ct6_d16_8x8_4_0() -> Seq<u8> {seq![131u8,226u8,223u8,202u8,105u8,248u8,33u8,83u8,254u8,128u8,210u8,6u8,45u8,230u8,163u8,240u8,4u8,199u8,240u8,17u8,47u8,233u8,78u8,151u8,38u8,152u8,252u8,18u8,153u8,198u8,27u8,138u8]}
spec fn e6_a7_line_interlace_ct6_d16_8x8_4_1() -> Seq<u8> {seq![195u8,71u8,133u8,85u8,164u8,46u8,11u8,63u8,102u8,54u8,55u8,201u8,147u8,97u8,214u8,183u8,165u8,101u8,57u8,53u8,70u8,158u8,166u8,155u8,19u8,212u8,150u8,29u8,8u8,197u8,63u8,224u8]}
#[verifier::spinoff_prover]
proof fn e6_a7_pass_interlace_ct6_d16_8x8_4()
    ensures forall|y: nat| y<2 ==> #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),r::line(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),4nat,y),4nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8(),4nat,y),
{
    hide(raw_interlace_ct6_d16_8x8);hide(crate::colour_model::pixel);hide(crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8);
    crate::colour_model::depth_values(16u8);crate::colour_model::depth_values(8u8);
    assert(r::pw(a_interlace_ct6_d16_8x8(),4nat)==4nat && r::ph(a_interlace_ct6_d16_8x8(),4nat)==2nat && r::base(a_interlace_ct6_d16_8x8(),4nat)==69nat && r::rb(a_interlace_ct6_d16_8x8(),4nat)==32nat && crate::filter_model::bpp(r::bits(a_interlace_ct6_d16_8x8()))==8nat) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct6_d16_8x8(),69nat,32nat,8nat,0nat);
    assert(raw_interlace_ct6_d16_8x8()[69]==0u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct6_d16_8x8(),69nat,32nat,0nat)==seq![131u8,226u8,223u8,202u8,105u8,248u8,33u8,83u8,254u8,128u8,210u8,6u8,45u8,230u8,163u8,240u8,4u8,199u8,240u8,17u8,47u8,233u8,78u8,151u8,38u8,152u8,252u8,18u8,153u8,198u8,27u8,138u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct6_d16_8x8(),69nat,32nat,8nat,0nat)==None);
    assert(crate::filter_model::encoded_row(0u8,seq![131u8,226u8,223u8,202u8,105u8,248u8,33u8,83u8,254u8,128u8,210u8,6u8,45u8,230u8,163u8,240u8,4u8,199u8,240u8,17u8,47u8,233u8,78u8,151u8,38u8,152u8,252u8,18u8,153u8,198u8,27u8,138u8],None,8nat,seq![131u8,226u8,223u8,202u8,105u8,248u8,33u8,83u8,254u8,128u8,210u8,6u8,45u8,230u8,163u8,240u8,4u8,199u8,240u8,17u8,47u8,233u8,78u8,151u8,38u8,152u8,252u8,18u8,153u8,198u8,27u8,138u8],0,32nat)) by (compute);
    crate::filter_model::checked_row_hint(0u8,seq![131u8,226u8,223u8,202u8,105u8,248u8,33u8,83u8,254u8,128u8,210u8,6u8,45u8,230u8,163u8,240u8,4u8,199u8,240u8,17u8,47u8,233u8,78u8,151u8,38u8,152u8,252u8,18u8,153u8,198u8,27u8,138u8],None,8nat,seq![131u8,226u8,223u8,202u8,105u8,248u8,33u8,83u8,254u8,128u8,210u8,6u8,45u8,230u8,163u8,240u8,4u8,199u8,240u8,17u8,47u8,233u8,78u8,151u8,38u8,152u8,252u8,18u8,153u8,198u8,27u8,138u8]);
    assert(r::line(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),4nat,0nat)==seq![131u8,226u8,223u8,202u8,105u8,248u8,33u8,83u8,254u8,128u8,210u8,6u8,45u8,230u8,163u8,240u8,4u8,199u8,240u8,17u8,47u8,233u8,78u8,151u8,38u8,152u8,252u8,18u8,153u8,198u8,27u8,138u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_4_0(),4nat)[0]==131u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_4_0(),4nat)[1]==223u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_4_0(),4nat)[2]==106u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_4_0(),4nat)[3]==33u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_4_0(),4nat)[4]==254u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_4_0(),4nat)[5]==209u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_4_0(),4nat)[6]==46u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_4_0(),4nat)[7]==163u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_4_0(),4nat)[8]==5u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_4_0(),4nat)[9]==239u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_4_0(),4nat)[10]==48u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_4_0(),4nat)[11]==78u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_4_0(),4nat)[12]==38u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_4_0(),4nat)[13]==251u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_4_0(),4nat)[14]==153u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_4_0(),4nat)[15]==27u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_4_0(),4nat),seq![131u8,223u8,106u8,33u8,254u8,209u8,46u8,163u8,5u8,239u8,48u8,78u8,38u8,251u8,153u8,27u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8(),4nat,0nat)==seq![131u8,223u8,106u8,33u8,254u8,209u8,46u8,163u8,5u8,239u8,48u8,78u8,38u8,251u8,153u8,27u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct6_d16_8x8(),69nat,32nat,8nat,1nat);
    assert(raw_interlace_ct6_d16_8x8()[102]==1u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct6_d16_8x8(),69nat,32nat,1nat)==seq![195u8,71u8,133u8,85u8,164u8,46u8,11u8,63u8,163u8,239u8,178u8,116u8,239u8,51u8,203u8,120u8,63u8,47u8,2u8,108u8,179u8,61u8,208u8,228u8,110u8,111u8,93u8,232u8,194u8,39u8,153u8,69u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct6_d16_8x8(),69nat,32nat,8nat,1nat)==Some(seq![131u8,226u8,223u8,202u8,105u8,248u8,33u8,83u8,254u8,128u8,210u8,6u8,45u8,230u8,163u8,240u8,4u8,199u8,240u8,17u8,47u8,233u8,78u8,151u8,38u8,152u8,252u8,18u8,153u8,198u8,27u8,138u8]));
    assert(crate::filter_model::encoded_row(1u8,seq![195u8,71u8,133u8,85u8,164u8,46u8,11u8,63u8,163u8,239u8,178u8,116u8,239u8,51u8,203u8,120u8,63u8,47u8,2u8,108u8,179u8,61u8,208u8,228u8,110u8,111u8,93u8,232u8,194u8,39u8,153u8,69u8],Some(seq![131u8,226u8,223u8,202u8,105u8,248u8,33u8,83u8,254u8,128u8,210u8,6u8,45u8,230u8,163u8,240u8,4u8,199u8,240u8,17u8,47u8,233u8,78u8,151u8,38u8,152u8,252u8,18u8,153u8,198u8,27u8,138u8]),8nat,seq![195u8,71u8,133u8,85u8,164u8,46u8,11u8,63u8,102u8,54u8,55u8,201u8,147u8,97u8,214u8,183u8,165u8,101u8,57u8,53u8,70u8,158u8,166u8,155u8,19u8,212u8,150u8,29u8,8u8,197u8,63u8,224u8],0,32nat)) by (compute);
    crate::filter_model::checked_row_hint(1u8,seq![195u8,71u8,133u8,85u8,164u8,46u8,11u8,63u8,163u8,239u8,178u8,116u8,239u8,51u8,203u8,120u8,63u8,47u8,2u8,108u8,179u8,61u8,208u8,228u8,110u8,111u8,93u8,232u8,194u8,39u8,153u8,69u8],Some(seq![131u8,226u8,223u8,202u8,105u8,248u8,33u8,83u8,254u8,128u8,210u8,6u8,45u8,230u8,163u8,240u8,4u8,199u8,240u8,17u8,47u8,233u8,78u8,151u8,38u8,152u8,252u8,18u8,153u8,198u8,27u8,138u8]),8nat,seq![195u8,71u8,133u8,85u8,164u8,46u8,11u8,63u8,102u8,54u8,55u8,201u8,147u8,97u8,214u8,183u8,165u8,101u8,57u8,53u8,70u8,158u8,166u8,155u8,19u8,212u8,150u8,29u8,8u8,197u8,63u8,224u8]);
    assert(r::line(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),4nat,1nat)==seq![195u8,71u8,133u8,85u8,164u8,46u8,11u8,63u8,102u8,54u8,55u8,201u8,147u8,97u8,214u8,183u8,165u8,101u8,57u8,53u8,70u8,158u8,166u8,155u8,19u8,212u8,150u8,29u8,8u8,197u8,63u8,224u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_4_1(),4nat)[0]==195u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_4_1(),4nat)[1]==133u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_4_1(),4nat)[2]==164u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_4_1(),4nat)[3]==11u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_4_1(),4nat)[4]==102u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_4_1(),4nat)[5]==56u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_4_1(),4nat)[6]==147u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_4_1(),4nat)[7]==214u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_4_1(),4nat)[8]==165u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_4_1(),4nat)[9]==57u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_4_1(),4nat)[10]==70u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_4_1(),4nat)[11]==166u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_4_1(),4nat)[12]==20u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_4_1(),4nat)[13]==150u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_4_1(),4nat)[14]==9u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_4_1(),4nat)[15]==64u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_4_1(),4nat),seq![195u8,133u8,164u8,11u8,102u8,56u8,147u8,214u8,165u8,57u8,70u8,166u8,20u8,150u8,9u8,64u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8(),4nat,1nat)==seq![195u8,133u8,164u8,11u8,102u8,56u8,147u8,214u8,165u8,57u8,70u8,166u8,20u8,150u8,9u8,64u8]) by (compute);
    assert forall|y: nat| y<2 implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),r::line(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),4nat,y),4nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8(),4nat,y) by {
        if y==0 {} else {assert(y==1);}
    }
}
spec fn e6_a7_line_interlace_ct6_d16_8x8_5_0() -> Seq<u8> {seq![235u8,236u8,29u8,178u8,82u8,3u8,208u8,141u8,104u8,62u8,97u8,19u8,230u8,59u8,7u8,89u8,216u8,226u8,48u8,150u8,62u8,198u8,123u8,4u8,229u8,34u8,190u8,201u8,92u8,115u8,154u8,150u8]}
spec fn e6_a7_line_interlace_ct6_d16_8x8_5_1() -> Seq<u8> {seq![6u8,165u8,85u8,6u8,128u8,214u8,5u8,63u8,53u8,79u8,0u8,101u8,239u8,6u8,207u8,152u8,42u8,0u8,90u8,203u8,114u8,214u8,60u8,253u8,228u8,81u8,170u8,158u8,218u8,62u8,115u8,112u8]}
spec fn e6_a7_line_interlace_ct6_d16_8x8_5_2() -> Seq<u8> {seq![29u8,101u8,250u8,14u8,177u8,192u8,131u8,154u8,15u8,216u8,234u8,225u8,35u8,148u8,62u8,226u8,223u8,181u8,174u8,42u8,93u8,45u8,8u8,166u8,17u8,115u8,60u8,249u8,78u8,130u8,255u8,50u8]}
spec fn e6_a7_line_interlace_ct6_d16_8x8_5_3() -> Seq<u8> {seq![75u8,122u8,104u8,183u8,226u8,85u8,98u8,215u8,13u8,130u8,129u8,202u8,18u8,26u8,116u8,236u8,53u8,152u8,202u8,240u8,84u8,251u8,110u8,221u8,65u8,47u8,227u8,193u8,186u8,180u8,233u8,209u8]}
#[verifier::spinoff_prover]
proof fn e6_a7_pass_interlace_ct6_d16_8x8_5()
    ensures forall|y: nat| y<4 ==> #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),r::line(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),5nat,y),4nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8(),5nat,y),
{
    hide(raw_interlace_ct6_d16_8x8);hide(crate::colour_model::pixel);hide(crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8);
    crate::colour_model::depth_values(16u8);crate::colour_model::depth_values(8u8);
    assert(r::pw(a_interlace_ct6_d16_8x8(),5nat)==4nat && r::ph(a_interlace_ct6_d16_8x8(),5nat)==4nat && r::base(a_interlace_ct6_d16_8x8(),5nat)==135nat && r::rb(a_interlace_ct6_d16_8x8(),5nat)==32nat && crate::filter_model::bpp(r::bits(a_interlace_ct6_d16_8x8()))==8nat) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct6_d16_8x8(),135nat,32nat,8nat,0nat);
    assert(raw_interlace_ct6_d16_8x8()[135]==2u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct6_d16_8x8(),135nat,32nat,0nat)==seq![235u8,236u8,29u8,178u8,82u8,3u8,208u8,141u8,104u8,62u8,97u8,19u8,230u8,59u8,7u8,89u8,216u8,226u8,48u8,150u8,62u8,198u8,123u8,4u8,229u8,34u8,190u8,201u8,92u8,115u8,154u8,150u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct6_d16_8x8(),135nat,32nat,8nat,0nat)==None);
    assert(crate::filter_model::encoded_row(2u8,seq![235u8,236u8,29u8,178u8,82u8,3u8,208u8,141u8,104u8,62u8,97u8,19u8,230u8,59u8,7u8,89u8,216u8,226u8,48u8,150u8,62u8,198u8,123u8,4u8,229u8,34u8,190u8,201u8,92u8,115u8,154u8,150u8],None,8nat,seq![235u8,236u8,29u8,178u8,82u8,3u8,208u8,141u8,104u8,62u8,97u8,19u8,230u8,59u8,7u8,89u8,216u8,226u8,48u8,150u8,62u8,198u8,123u8,4u8,229u8,34u8,190u8,201u8,92u8,115u8,154u8,150u8],0,32nat)) by (compute);
    crate::filter_model::checked_row_hint(2u8,seq![235u8,236u8,29u8,178u8,82u8,3u8,208u8,141u8,104u8,62u8,97u8,19u8,230u8,59u8,7u8,89u8,216u8,226u8,48u8,150u8,62u8,198u8,123u8,4u8,229u8,34u8,190u8,201u8,92u8,115u8,154u8,150u8],None,8nat,seq![235u8,236u8,29u8,178u8,82u8,3u8,208u8,141u8,104u8,62u8,97u8,19u8,230u8,59u8,7u8,89u8,216u8,226u8,48u8,150u8,62u8,198u8,123u8,4u8,229u8,34u8,190u8,201u8,92u8,115u8,154u8,150u8]);
    assert(r::line(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),5nat,0nat)==seq![235u8,236u8,29u8,178u8,82u8,3u8,208u8,141u8,104u8,62u8,97u8,19u8,230u8,59u8,7u8,89u8,216u8,226u8,48u8,150u8,62u8,198u8,123u8,4u8,229u8,34u8,190u8,201u8,92u8,115u8,154u8,150u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_0(),4nat)[0]==235u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_0(),4nat)[1]==30u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_0(),4nat)[2]==82u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_0(),4nat)[3]==208u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_0(),4nat)[4]==104u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_0(),4nat)[5]==97u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_0(),4nat)[6]==229u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_0(),4nat)[7]==7u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_0(),4nat)[8]==216u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_0(),4nat)[9]==48u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_0(),4nat)[10]==63u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_0(),4nat)[11]==123u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_0(),4nat)[12]==228u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_0(),4nat)[13]==190u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_0(),4nat)[14]==92u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_0(),4nat)[15]==154u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_0(),4nat),seq![235u8,30u8,82u8,208u8,104u8,97u8,229u8,7u8,216u8,48u8,63u8,123u8,228u8,190u8,92u8,154u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8(),5nat,0nat)==seq![235u8,30u8,82u8,208u8,104u8,97u8,229u8,7u8,216u8,48u8,63u8,123u8,228u8,190u8,92u8,154u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct6_d16_8x8(),135nat,32nat,8nat,1nat);
    assert(raw_interlace_ct6_d16_8x8()[168]==3u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct6_d16_8x8(),135nat,32nat,1nat)==seq![145u8,47u8,71u8,173u8,87u8,213u8,157u8,249u8,254u8,222u8,165u8,89u8,60u8,126u8,201u8,76u8,164u8,104u8,66u8,78u8,220u8,112u8,151u8,175u8,93u8,64u8,30u8,212u8,115u8,154u8,8u8,167u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct6_d16_8x8(),135nat,32nat,8nat,1nat)==Some(seq![235u8,236u8,29u8,178u8,82u8,3u8,208u8,141u8,104u8,62u8,97u8,19u8,230u8,59u8,7u8,89u8,216u8,226u8,48u8,150u8,62u8,198u8,123u8,4u8,229u8,34u8,190u8,201u8,92u8,115u8,154u8,150u8]));
    assert(crate::filter_model::encoded_row(3u8,seq![145u8,47u8,71u8,173u8,87u8,213u8,157u8,249u8,254u8,222u8,165u8,89u8,60u8,126u8,201u8,76u8,164u8,104u8,66u8,78u8,220u8,112u8,151u8,175u8,93u8,64u8,30u8,212u8,115u8,154u8,8u8,167u8],Some(seq![235u8,236u8,29u8,178u8,82u8,3u8,208u8,141u8,104u8,62u8,97u8,19u8,230u8,59u8,7u8,89u8,216u8,226u8,48u8,150u8,62u8,198u8,123u8,4u8,229u8,34u8,190u8,201u8,92u8,115u8,154u8,150u8]),8nat,seq![6u8,165u8,85u8,6u8,128u8,214u8,5u8,63u8,53u8,79u8,0u8,101u8,239u8,6u8,207u8,152u8,42u8,0u8,90u8,203u8,114u8,214u8,60u8,253u8,228u8,81u8,170u8,158u8,218u8,62u8,115u8,112u8],0,32nat)) by (compute);
    crate::filter_model::checked_row_hint(3u8,seq![145u8,47u8,71u8,173u8,87u8,213u8,157u8,249u8,254u8,222u8,165u8,89u8,60u8,126u8,201u8,76u8,164u8,104u8,66u8,78u8,220u8,112u8,151u8,175u8,93u8,64u8,30u8,212u8,115u8,154u8,8u8,167u8],Some(seq![235u8,236u8,29u8,178u8,82u8,3u8,208u8,141u8,104u8,62u8,97u8,19u8,230u8,59u8,7u8,89u8,216u8,226u8,48u8,150u8,62u8,198u8,123u8,4u8,229u8,34u8,190u8,201u8,92u8,115u8,154u8,150u8]),8nat,seq![6u8,165u8,85u8,6u8,128u8,214u8,5u8,63u8,53u8,79u8,0u8,101u8,239u8,6u8,207u8,152u8,42u8,0u8,90u8,203u8,114u8,214u8,60u8,253u8,228u8,81u8,170u8,158u8,218u8,62u8,115u8,112u8]);
    assert(r::line(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),5nat,1nat)==seq![6u8,165u8,85u8,6u8,128u8,214u8,5u8,63u8,53u8,79u8,0u8,101u8,239u8,6u8,207u8,152u8,42u8,0u8,90u8,203u8,114u8,214u8,60u8,253u8,228u8,81u8,170u8,158u8,218u8,62u8,115u8,112u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_1(),4nat)[0]==7u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_1(),4nat)[1]==85u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_1(),4nat)[2]==128u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_1(),4nat)[3]==5u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_1(),4nat)[4]==53u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_1(),4nat)[5]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_1(),4nat)[6]==238u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_1(),4nat)[7]==207u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_1(),4nat)[8]==42u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_1(),4nat)[9]==90u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_1(),4nat)[10]==114u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_1(),4nat)[11]==61u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_1(),4nat)[12]==227u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_1(),4nat)[13]==170u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_1(),4nat)[14]==217u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_1(),4nat)[15]==115u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_1(),4nat),seq![7u8,85u8,128u8,5u8,53u8,0u8,238u8,207u8,42u8,90u8,114u8,61u8,227u8,170u8,217u8,115u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8(),5nat,1nat)==seq![7u8,85u8,128u8,5u8,53u8,0u8,238u8,207u8,42u8,90u8,114u8,61u8,227u8,170u8,217u8,115u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct6_d16_8x8(),135nat,32nat,8nat,2nat);
    assert(raw_interlace_ct6_d16_8x8()[201]==4u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct6_d16_8x8(),135nat,32nat,2nat)==seq![23u8,192u8,165u8,8u8,49u8,234u8,126u8,91u8,218u8,137u8,149u8,124u8,52u8,142u8,111u8,72u8,208u8,102u8,196u8,73u8,58u8,87u8,204u8,169u8,45u8,190u8,142u8,207u8,116u8,85u8,195u8,194u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct6_d16_8x8(),135nat,32nat,8nat,2nat)==Some(seq![6u8,165u8,85u8,6u8,128u8,214u8,5u8,63u8,53u8,79u8,0u8,101u8,239u8,6u8,207u8,152u8,42u8,0u8,90u8,203u8,114u8,214u8,60u8,253u8,228u8,81u8,170u8,158u8,218u8,62u8,115u8,112u8]));
    assert(crate::filter_model::encoded_row(4u8,seq![23u8,192u8,165u8,8u8,49u8,234u8,126u8,91u8,218u8,137u8,149u8,124u8,52u8,142u8,111u8,72u8,208u8,102u8,196u8,73u8,58u8,87u8,204u8,169u8,45u8,190u8,142u8,207u8,116u8,85u8,195u8,194u8],Some(seq![6u8,165u8,85u8,6u8,128u8,214u8,5u8,63u8,53u8,79u8,0u8,101u8,239u8,6u8,207u8,152u8,42u8,0u8,90u8,203u8,114u8,214u8,60u8,253u8,228u8,81u8,170u8,158u8,218u8,62u8,115u8,112u8]),8nat,seq![29u8,101u8,250u8,14u8,177u8,192u8,131u8,154u8,15u8,216u8,234u8,225u8,35u8,148u8,62u8,226u8,223u8,181u8,174u8,42u8,93u8,45u8,8u8,166u8,17u8,115u8,60u8,249u8,78u8,130u8,255u8,50u8],0,32nat)) by (compute);
    crate::filter_model::checked_row_hint(4u8,seq![23u8,192u8,165u8,8u8,49u8,234u8,126u8,91u8,218u8,137u8,149u8,124u8,52u8,142u8,111u8,72u8,208u8,102u8,196u8,73u8,58u8,87u8,204u8,169u8,45u8,190u8,142u8,207u8,116u8,85u8,195u8,194u8],Some(seq![6u8,165u8,85u8,6u8,128u8,214u8,5u8,63u8,53u8,79u8,0u8,101u8,239u8,6u8,207u8,152u8,42u8,0u8,90u8,203u8,114u8,214u8,60u8,253u8,228u8,81u8,170u8,158u8,218u8,62u8,115u8,112u8]),8nat,seq![29u8,101u8,250u8,14u8,177u8,192u8,131u8,154u8,15u8,216u8,234u8,225u8,35u8,148u8,62u8,226u8,223u8,181u8,174u8,42u8,93u8,45u8,8u8,166u8,17u8,115u8,60u8,249u8,78u8,130u8,255u8,50u8]);
    assert(r::line(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),5nat,2nat)==seq![29u8,101u8,250u8,14u8,177u8,192u8,131u8,154u8,15u8,216u8,234u8,225u8,35u8,148u8,62u8,226u8,223u8,181u8,174u8,42u8,93u8,45u8,8u8,166u8,17u8,115u8,60u8,249u8,78u8,130u8,255u8,50u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_2(),4nat)[0]==29u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_2(),4nat)[1]==249u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_2(),4nat)[2]==177u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_2(),4nat)[3]==131u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_2(),4nat)[4]==16u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_2(),4nat)[5]==234u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_2(),4nat)[6]==35u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_2(),4nat)[7]==63u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_2(),4nat)[8]==223u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_2(),4nat)[9]==173u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_2(),4nat)[10]==93u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_2(),4nat)[11]==9u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_2(),4nat)[12]==17u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_2(),4nat)[13]==61u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_2(),4nat)[14]==78u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_2(),4nat)[15]==254u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_2(),4nat),seq![29u8,249u8,177u8,131u8,16u8,234u8,35u8,63u8,223u8,173u8,93u8,9u8,17u8,61u8,78u8,254u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8(),5nat,2nat)==seq![29u8,249u8,177u8,131u8,16u8,234u8,35u8,63u8,223u8,173u8,93u8,9u8,17u8,61u8,78u8,254u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct6_d16_8x8(),135nat,32nat,8nat,3nat);
    assert(raw_interlace_ct6_d16_8x8()[234]==0u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct6_d16_8x8(),135nat,32nat,3nat)==seq![75u8,122u8,104u8,183u8,226u8,85u8,98u8,215u8,13u8,130u8,129u8,202u8,18u8,26u8,116u8,236u8,53u8,152u8,202u8,240u8,84u8,251u8,110u8,221u8,65u8,47u8,227u8,193u8,186u8,180u8,233u8,209u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct6_d16_8x8(),135nat,32nat,8nat,3nat)==Some(seq![29u8,101u8,250u8,14u8,177u8,192u8,131u8,154u8,15u8,216u8,234u8,225u8,35u8,148u8,62u8,226u8,223u8,181u8,174u8,42u8,93u8,45u8,8u8,166u8,17u8,115u8,60u8,249u8,78u8,130u8,255u8,50u8]));
    assert(crate::filter_model::encoded_row(0u8,seq![75u8,122u8,104u8,183u8,226u8,85u8,98u8,215u8,13u8,130u8,129u8,202u8,18u8,26u8,116u8,236u8,53u8,152u8,202u8,240u8,84u8,251u8,110u8,221u8,65u8,47u8,227u8,193u8,186u8,180u8,233u8,209u8],Some(seq![29u8,101u8,250u8,14u8,177u8,192u8,131u8,154u8,15u8,216u8,234u8,225u8,35u8,148u8,62u8,226u8,223u8,181u8,174u8,42u8,93u8,45u8,8u8,166u8,17u8,115u8,60u8,249u8,78u8,130u8,255u8,50u8]),8nat,seq![75u8,122u8,104u8,183u8,226u8,85u8,98u8,215u8,13u8,130u8,129u8,202u8,18u8,26u8,116u8,236u8,53u8,152u8,202u8,240u8,84u8,251u8,110u8,221u8,65u8,47u8,227u8,193u8,186u8,180u8,233u8,209u8],0,32nat)) by (compute);
    crate::filter_model::checked_row_hint(0u8,seq![75u8,122u8,104u8,183u8,226u8,85u8,98u8,215u8,13u8,130u8,129u8,202u8,18u8,26u8,116u8,236u8,53u8,152u8,202u8,240u8,84u8,251u8,110u8,221u8,65u8,47u8,227u8,193u8,186u8,180u8,233u8,209u8],Some(seq![29u8,101u8,250u8,14u8,177u8,192u8,131u8,154u8,15u8,216u8,234u8,225u8,35u8,148u8,62u8,226u8,223u8,181u8,174u8,42u8,93u8,45u8,8u8,166u8,17u8,115u8,60u8,249u8,78u8,130u8,255u8,50u8]),8nat,seq![75u8,122u8,104u8,183u8,226u8,85u8,98u8,215u8,13u8,130u8,129u8,202u8,18u8,26u8,116u8,236u8,53u8,152u8,202u8,240u8,84u8,251u8,110u8,221u8,65u8,47u8,227u8,193u8,186u8,180u8,233u8,209u8]);
    assert(r::line(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),5nat,3nat)==seq![75u8,122u8,104u8,183u8,226u8,85u8,98u8,215u8,13u8,130u8,129u8,202u8,18u8,26u8,116u8,236u8,53u8,152u8,202u8,240u8,84u8,251u8,110u8,221u8,65u8,47u8,227u8,193u8,186u8,180u8,233u8,209u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_3(),4nat)[0]==75u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_3(),4nat)[1]==104u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_3(),4nat)[2]==225u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_3(),4nat)[3]==98u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_3(),4nat)[4]==13u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_3(),4nat)[5]==129u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_3(),4nat)[6]==18u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_3(),4nat)[7]==116u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_3(),4nat)[8]==53u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_3(),4nat)[9]==202u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_3(),4nat)[10]==85u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_3(),4nat)[11]==110u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_3(),4nat)[12]==65u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_3(),4nat)[13]==227u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_3(),4nat)[14]==186u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_3(),4nat)[15]==233u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_5_3(),4nat),seq![75u8,104u8,225u8,98u8,13u8,129u8,18u8,116u8,53u8,202u8,85u8,110u8,65u8,227u8,186u8,233u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8(),5nat,3nat)==seq![75u8,104u8,225u8,98u8,13u8,129u8,18u8,116u8,53u8,202u8,85u8,110u8,65u8,227u8,186u8,233u8]) by (compute);
    assert forall|y: nat| y<4 implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),r::line(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),5nat,y),4nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8(),5nat,y) by {
        if y==0 {} else if y==1 {} else if y==2 {} else {assert(y==3);}
    }
}
spec fn e6_a7_line_interlace_ct6_d16_8x8_6_0() -> Seq<u8> {seq![148u8,62u8,246u8,131u8,187u8,154u8,171u8,181u8,55u8,145u8,94u8,140u8,181u8,163u8,216u8,75u8,237u8,178u8,240u8,170u8,99u8,166u8,56u8,136u8,197u8,247u8,55u8,60u8,151u8,150u8,62u8,130u8,27u8,85u8,115u8,251u8,209u8,239u8,133u8,77u8,90u8,45u8,216u8,207u8,208u8,171u8,125u8,96u8,169u8,182u8,184u8,111u8,137u8,109u8,28u8,213u8,14u8,20u8,207u8,100u8,189u8,131u8,89u8,196u8]}
spec fn e6_a7_line_interlace_ct6_d16_8x8_6_1() -> Seq<u8> {seq![203u8,6u8,207u8,246u8,42u8,118u8,207u8,42u8,175u8,49u8,232u8,196u8,231u8,132u8,1u8,172u8,68u8,233u8,85u8,124u8,217u8,200u8,86u8,132u8,71u8,210u8,80u8,91u8,59u8,235u8,8u8,58u8,17u8,25u8,21u8,92u8,155u8,55u8,192u8,156u8,91u8,38u8,8u8,160u8,51u8,135u8,202u8,180u8,211u8,111u8,164u8,150u8,79u8,77u8,129u8,151u8,130u8,19u8,208u8,0u8,78u8,33u8,205u8,112u8]}
spec fn e6_a7_line_interlace_ct6_d16_8x8_6_2() -> Seq<u8> {seq![169u8,105u8,14u8,127u8,254u8,143u8,12u8,141u8,207u8,86u8,170u8,28u8,98u8,198u8,40u8,237u8,188u8,44u8,28u8,167u8,237u8,158u8,77u8,116u8,184u8,235u8,10u8,61u8,182u8,201u8,84u8,22u8,47u8,43u8,255u8,241u8,89u8,156u8,133u8,246u8,213u8,49u8,195u8,38u8,14u8,76u8,91u8,210u8,128u8,140u8,51u8,202u8,122u8,134u8,119u8,75u8,228u8,144u8,94u8,197u8,32u8,136u8,75u8,122u8]}
spec fn e6_a7_line_interlace_ct6_d16_8x8_6_3() -> Seq<u8> {seq![184u8,71u8,14u8,69u8,130u8,177u8,64u8,79u8,196u8,171u8,62u8,148u8,78u8,198u8,127u8,23u8,163u8,199u8,93u8,72u8,254u8,213u8,155u8,155u8,151u8,20u8,203u8,231u8,212u8,164u8,132u8,23u8,237u8,90u8,172u8,211u8,143u8,27u8,167u8,137u8,199u8,186u8,81u8,125u8,79u8,204u8,34u8,129u8,241u8,76u8,155u8,130u8,127u8,1u8,190u8,163u8,85u8,67u8,168u8,45u8,66u8,120u8,169u8,21u8]}
#[verifier::spinoff_prover]
proof fn e6_a7_pass_interlace_ct6_d16_8x8_6()
    ensures forall|y: nat| y<4 ==> #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),r::line(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),6nat,y),8nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8(),6nat,y),
{
    hide(raw_interlace_ct6_d16_8x8);hide(crate::colour_model::pixel);hide(crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8);
    crate::colour_model::depth_values(16u8);crate::colour_model::depth_values(8u8);
    assert(r::pw(a_interlace_ct6_d16_8x8(),6nat)==8nat && r::ph(a_interlace_ct6_d16_8x8(),6nat)==4nat && r::base(a_interlace_ct6_d16_8x8(),6nat)==267nat && r::rb(a_interlace_ct6_d16_8x8(),6nat)==64nat && crate::filter_model::bpp(r::bits(a_interlace_ct6_d16_8x8()))==8nat) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct6_d16_8x8(),267nat,64nat,8nat,0nat);
    assert(raw_interlace_ct6_d16_8x8()[267]==1u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct6_d16_8x8(),267nat,64nat,0nat)==seq![148u8,62u8,246u8,131u8,187u8,154u8,171u8,181u8,163u8,83u8,104u8,9u8,250u8,9u8,45u8,150u8,182u8,33u8,146u8,30u8,174u8,3u8,96u8,61u8,216u8,69u8,71u8,146u8,52u8,240u8,6u8,250u8,86u8,94u8,60u8,191u8,58u8,89u8,71u8,203u8,63u8,216u8,101u8,212u8,255u8,188u8,248u8,19u8,79u8,137u8,224u8,160u8,185u8,194u8,159u8,117u8,101u8,94u8,23u8,245u8,52u8,22u8,61u8,239u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct6_d16_8x8(),267nat,64nat,8nat,0nat)==None);
    assert(crate::filter_model::encoded_row(1u8,seq![148u8,62u8,246u8,131u8,187u8,154u8,171u8,181u8,163u8,83u8,104u8,9u8,250u8,9u8,45u8,150u8,182u8,33u8,146u8,30u8,174u8,3u8,96u8,61u8,216u8,69u8,71u8,146u8,52u8,240u8,6u8,250u8,86u8,94u8,60u8,191u8,58u8,89u8,71u8,203u8,63u8,216u8,101u8,212u8,255u8,188u8,248u8,19u8,79u8,137u8,224u8,160u8,185u8,194u8,159u8,117u8,101u8,94u8,23u8,245u8,52u8,22u8,61u8,239u8],None,8nat,seq![148u8,62u8,246u8,131u8,187u8,154u8,171u8,181u8,55u8,145u8,94u8,140u8,181u8,163u8,216u8,75u8,237u8,178u8,240u8,170u8,99u8,166u8,56u8,136u8,197u8,247u8,55u8,60u8,151u8,150u8,62u8,130u8,27u8,85u8,115u8,251u8,209u8,239u8,133u8,77u8,90u8,45u8,216u8,207u8,208u8,171u8,125u8,96u8,169u8,182u8,184u8,111u8,137u8,109u8,28u8,213u8,14u8,20u8,207u8,100u8,189u8,131u8,89u8,196u8],0,64nat)) by (compute);
    crate::filter_model::checked_row_hint(1u8,seq![148u8,62u8,246u8,131u8,187u8,154u8,171u8,181u8,163u8,83u8,104u8,9u8,250u8,9u8,45u8,150u8,182u8,33u8,146u8,30u8,174u8,3u8,96u8,61u8,216u8,69u8,71u8,146u8,52u8,240u8,6u8,250u8,86u8,94u8,60u8,191u8,58u8,89u8,71u8,203u8,63u8,216u8,101u8,212u8,255u8,188u8,248u8,19u8,79u8,137u8,224u8,160u8,185u8,194u8,159u8,117u8,101u8,94u8,23u8,245u8,52u8,22u8,61u8,239u8],None,8nat,seq![148u8,62u8,246u8,131u8,187u8,154u8,171u8,181u8,55u8,145u8,94u8,140u8,181u8,163u8,216u8,75u8,237u8,178u8,240u8,170u8,99u8,166u8,56u8,136u8,197u8,247u8,55u8,60u8,151u8,150u8,62u8,130u8,27u8,85u8,115u8,251u8,209u8,239u8,133u8,77u8,90u8,45u8,216u8,207u8,208u8,171u8,125u8,96u8,169u8,182u8,184u8,111u8,137u8,109u8,28u8,213u8,14u8,20u8,207u8,100u8,189u8,131u8,89u8,196u8]);
    assert(r::line(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),6nat,0nat)==seq![148u8,62u8,246u8,131u8,187u8,154u8,171u8,181u8,55u8,145u8,94u8,140u8,181u8,163u8,216u8,75u8,237u8,178u8,240u8,170u8,99u8,166u8,56u8,136u8,197u8,247u8,55u8,60u8,151u8,150u8,62u8,130u8,27u8,85u8,115u8,251u8,209u8,239u8,133u8,77u8,90u8,45u8,216u8,207u8,208u8,171u8,125u8,96u8,169u8,182u8,184u8,111u8,137u8,109u8,28u8,213u8,14u8,20u8,207u8,100u8,189u8,131u8,89u8,196u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_0(),8nat)[0]==148u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_0(),8nat)[1]==246u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_0(),8nat)[2]==187u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_0(),8nat)[3]==171u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_0(),8nat)[4]==55u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_0(),8nat)[5]==94u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_0(),8nat)[6]==181u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_0(),8nat)[7]==215u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_0(),8nat)[8]==237u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_0(),8nat)[9]==240u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_0(),8nat)[10]==99u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_0(),8nat)[11]==56u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_0(),8nat)[12]==197u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_0(),8nat)[13]==55u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_0(),8nat)[14]==151u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_0(),8nat)[15]==62u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_0(),8nat)[16]==27u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_0(),8nat)[17]==116u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_0(),8nat)[18]==209u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_0(),8nat)[19]==133u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_0(),8nat)[20]==90u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_0(),8nat)[21]==216u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_0(),8nat)[22]==208u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_0(),8nat)[23]==125u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_0(),8nat)[24]==169u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_0(),8nat)[25]==184u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_0(),8nat)[26]==137u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_0(),8nat)[27]==29u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_0(),8nat)[28]==14u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_0(),8nat)[29]==207u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_0(),8nat)[30]==189u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_0(),8nat)[31]==89u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_0(),8nat),seq![148u8,246u8,187u8,171u8,55u8,94u8,181u8,215u8,237u8,240u8,99u8,56u8,197u8,55u8,151u8,62u8,27u8,116u8,209u8,133u8,90u8,216u8,208u8,125u8,169u8,184u8,137u8,29u8,14u8,207u8,189u8,89u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8(),6nat,0nat)==seq![148u8,246u8,187u8,171u8,55u8,94u8,181u8,215u8,237u8,240u8,99u8,56u8,197u8,55u8,151u8,62u8,27u8,116u8,209u8,133u8,90u8,216u8,208u8,125u8,169u8,184u8,137u8,29u8,14u8,207u8,189u8,89u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct6_d16_8x8(),267nat,64nat,8nat,1nat);
    assert(raw_interlace_ct6_d16_8x8()[332]==2u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct6_d16_8x8(),267nat,64nat,1nat)==seq![55u8,200u8,217u8,115u8,111u8,220u8,36u8,117u8,120u8,160u8,138u8,56u8,50u8,225u8,41u8,97u8,87u8,55u8,101u8,210u8,118u8,34u8,30u8,252u8,130u8,219u8,25u8,31u8,164u8,85u8,202u8,184u8,246u8,196u8,162u8,97u8,202u8,72u8,59u8,79u8,1u8,249u8,48u8,209u8,99u8,220u8,77u8,84u8,42u8,185u8,236u8,39u8,198u8,224u8,101u8,194u8,116u8,255u8,1u8,156u8,145u8,158u8,116u8,172u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct6_d16_8x8(),267nat,64nat,8nat,1nat)==Some(seq![148u8,62u8,246u8,131u8,187u8,154u8,171u8,181u8,55u8,145u8,94u8,140u8,181u8,163u8,216u8,75u8,237u8,178u8,240u8,170u8,99u8,166u8,56u8,136u8,197u8,247u8,55u8,60u8,151u8,150u8,62u8,130u8,27u8,85u8,115u8,251u8,209u8,239u8,133u8,77u8,90u8,45u8,216u8,207u8,208u8,171u8,125u8,96u8,169u8,182u8,184u8,111u8,137u8,109u8,28u8,213u8,14u8,20u8,207u8,100u8,189u8,131u8,89u8,196u8]));
    assert(crate::filter_model::encoded_row(2u8,seq![55u8,200u8,217u8,115u8,111u8,220u8,36u8,117u8,120u8,160u8,138u8,56u8,50u8,225u8,41u8,97u8,87u8,55u8,101u8,210u8,118u8,34u8,30u8,252u8,130u8,219u8,25u8,31u8,164u8,85u8,202u8,184u8,246u8,196u8,162u8,97u8,202u8,72u8,59u8,79u8,1u8,249u8,48u8,209u8,99u8,220u8,77u8,84u8,42u8,185u8,236u8,39u8,198u8,224u8,101u8,194u8,116u8,255u8,1u8,156u8,145u8,158u8,116u8,172u8],Some(seq![148u8,62u8,246u8,131u8,187u8,154u8,171u8,181u8,55u8,145u8,94u8,140u8,181u8,163u8,216u8,75u8,237u8,178u8,240u8,170u8,99u8,166u8,56u8,136u8,197u8,247u8,55u8,60u8,151u8,150u8,62u8,130u8,27u8,85u8,115u8,251u8,209u8,239u8,133u8,77u8,90u8,45u8,216u8,207u8,208u8,171u8,125u8,96u8,169u8,182u8,184u8,111u8,137u8,109u8,28u8,213u8,14u8,20u8,207u8,100u8,189u8,131u8,89u8,196u8]),8nat,seq![203u8,6u8,207u8,246u8,42u8,118u8,207u8,42u8,175u8,49u8,232u8,196u8,231u8,132u8,1u8,172u8,68u8,233u8,85u8,124u8,217u8,200u8,86u8,132u8,71u8,210u8,80u8,91u8,59u8,235u8,8u8,58u8,17u8,25u8,21u8,92u8,155u8,55u8,192u8,156u8,91u8,38u8,8u8,160u8,51u8,135u8,202u8,180u8,211u8,111u8,164u8,150u8,79u8,77u8,129u8,151u8,130u8,19u8,208u8,0u8,78u8,33u8,205u8,112u8],0,64nat)) by (compute);
    crate::filter_model::checked_row_hint(2u8,seq![55u8,200u8,217u8,115u8,111u8,220u8,36u8,117u8,120u8,160u8,138u8,56u8,50u8,225u8,41u8,97u8,87u8,55u8,101u8,210u8,118u8,34u8,30u8,252u8,130u8,219u8,25u8,31u8,164u8,85u8,202u8,184u8,246u8,196u8,162u8,97u8,202u8,72u8,59u8,79u8,1u8,249u8,48u8,209u8,99u8,220u8,77u8,84u8,42u8,185u8,236u8,39u8,198u8,224u8,101u8,194u8,116u8,255u8,1u8,156u8,145u8,158u8,116u8,172u8],Some(seq![148u8,62u8,246u8,131u8,187u8,154u8,171u8,181u8,55u8,145u8,94u8,140u8,181u8,163u8,216u8,75u8,237u8,178u8,240u8,170u8,99u8,166u8,56u8,136u8,197u8,247u8,55u8,60u8,151u8,150u8,62u8,130u8,27u8,85u8,115u8,251u8,209u8,239u8,133u8,77u8,90u8,45u8,216u8,207u8,208u8,171u8,125u8,96u8,169u8,182u8,184u8,111u8,137u8,109u8,28u8,213u8,14u8,20u8,207u8,100u8,189u8,131u8,89u8,196u8]),8nat,seq![203u8,6u8,207u8,246u8,42u8,118u8,207u8,42u8,175u8,49u8,232u8,196u8,231u8,132u8,1u8,172u8,68u8,233u8,85u8,124u8,217u8,200u8,86u8,132u8,71u8,210u8,80u8,91u8,59u8,235u8,8u8,58u8,17u8,25u8,21u8,92u8,155u8,55u8,192u8,156u8,91u8,38u8,8u8,160u8,51u8,135u8,202u8,180u8,211u8,111u8,164u8,150u8,79u8,77u8,129u8,151u8,130u8,19u8,208u8,0u8,78u8,33u8,205u8,112u8]);
    assert(r::line(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),6nat,1nat)==seq![203u8,6u8,207u8,246u8,42u8,118u8,207u8,42u8,175u8,49u8,232u8,196u8,231u8,132u8,1u8,172u8,68u8,233u8,85u8,124u8,217u8,200u8,86u8,132u8,71u8,210u8,80u8,91u8,59u8,235u8,8u8,58u8,17u8,25u8,21u8,92u8,155u8,55u8,192u8,156u8,91u8,38u8,8u8,160u8,51u8,135u8,202u8,180u8,211u8,111u8,164u8,150u8,79u8,77u8,129u8,151u8,130u8,19u8,208u8,0u8,78u8,33u8,205u8,112u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_1(),8nat)[0]==202u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_1(),8nat)[1]==207u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_1(),8nat)[2]==42u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_1(),8nat)[3]==206u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_1(),8nat)[4]==175u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_1(),8nat)[5]==232u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_1(),8nat)[6]==231u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_1(),8nat)[7]==2u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_1(),8nat)[8]==69u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_1(),8nat)[9]==85u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_1(),8nat)[10]==217u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_1(),8nat)[11]==86u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_1(),8nat)[12]==72u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_1(),8nat)[13]==80u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_1(),8nat)[14]==60u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_1(),8nat)[15]==8u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_1(),8nat)[16]==17u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_1(),8nat)[17]==21u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_1(),8nat)[18]==155u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_1(),8nat)[19]==192u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_1(),8nat)[20]==91u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_1(),8nat)[21]==9u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_1(),8nat)[22]==51u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_1(),8nat)[23]==202u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_1(),8nat)[24]==211u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_1(),8nat)[25]==164u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_1(),8nat)[26]==79u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_1(),8nat)[27]==129u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_1(),8nat)[28]==130u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_1(),8nat)[29]==207u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_1(),8nat)[30]==78u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_1(),8nat)[31]==205u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_1(),8nat),seq![202u8,207u8,42u8,206u8,175u8,232u8,231u8,2u8,69u8,85u8,217u8,86u8,72u8,80u8,60u8,8u8,17u8,21u8,155u8,192u8,91u8,9u8,51u8,202u8,211u8,164u8,79u8,129u8,130u8,207u8,78u8,205u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8(),6nat,1nat)==seq![202u8,207u8,42u8,206u8,175u8,232u8,231u8,2u8,69u8,85u8,217u8,86u8,72u8,80u8,60u8,8u8,17u8,21u8,155u8,192u8,91u8,9u8,51u8,202u8,211u8,164u8,79u8,129u8,130u8,207u8,78u8,205u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct6_d16_8x8(),267nat,64nat,8nat,2nat);
    assert(raw_interlace_ct6_d16_8x8()[397]==3u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct6_d16_8x8(),267nat,64nat,2nat)==seq![68u8,102u8,167u8,4u8,233u8,84u8,165u8,120u8,35u8,9u8,47u8,123u8,112u8,61u8,34u8,81u8,51u8,141u8,157u8,91u8,80u8,215u8,14u8,188u8,55u8,108u8,212u8,188u8,34u8,5u8,42u8,191u8,203u8,169u8,240u8,165u8,177u8,28u8,251u8,157u8,144u8,9u8,64u8,94u8,200u8,187u8,180u8,253u8,172u8,60u8,128u8,108u8,76u8,58u8,9u8,151u8,99u8,65u8,221u8,96u8,188u8,53u8,169u8,29u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct6_d16_8x8(),267nat,64nat,8nat,2nat)==Some(seq![203u8,6u8,207u8,246u8,42u8,118u8,207u8,42u8,175u8,49u8,232u8,196u8,231u8,132u8,1u8,172u8,68u8,233u8,85u8,124u8,217u8,200u8,86u8,132u8,71u8,210u8,80u8,91u8,59u8,235u8,8u8,58u8,17u8,25u8,21u8,92u8,155u8,55u8,192u8,156u8,91u8,38u8,8u8,160u8,51u8,135u8,202u8,180u8,211u8,111u8,164u8,150u8,79u8,77u8,129u8,151u8,130u8,19u8,208u8,0u8,78u8,33u8,205u8,112u8]));
    assert(crate::filter_model::encoded_row(3u8,seq![68u8,102u8,167u8,4u8,233u8,84u8,165u8,120u8,35u8,9u8,47u8,123u8,112u8,61u8,34u8,81u8,51u8,141u8,157u8,91u8,80u8,215u8,14u8,188u8,55u8,108u8,212u8,188u8,34u8,5u8,42u8,191u8,203u8,169u8,240u8,165u8,177u8,28u8,251u8,157u8,144u8,9u8,64u8,94u8,200u8,187u8,180u8,253u8,172u8,60u8,128u8,108u8,76u8,58u8,9u8,151u8,99u8,65u8,221u8,96u8,188u8,53u8,169u8,29u8],Some(seq![203u8,6u8,207u8,246u8,42u8,118u8,207u8,42u8,175u8,49u8,232u8,196u8,231u8,132u8,1u8,172u8,68u8,233u8,85u8,124u8,217u8,200u8,86u8,132u8,71u8,210u8,80u8,91u8,59u8,235u8,8u8,58u8,17u8,25u8,21u8,92u8,155u8,55u8,192u8,156u8,91u8,38u8,8u8,160u8,51u8,135u8,202u8,180u8,211u8,111u8,164u8,150u8,79u8,77u8,129u8,151u8,130u8,19u8,208u8,0u8,78u8,33u8,205u8,112u8]),8nat,seq![169u8,105u8,14u8,127u8,254u8,143u8,12u8,141u8,207u8,86u8,170u8,28u8,98u8,198u8,40u8,237u8,188u8,44u8,28u8,167u8,237u8,158u8,77u8,116u8,184u8,235u8,10u8,61u8,182u8,201u8,84u8,22u8,47u8,43u8,255u8,241u8,89u8,156u8,133u8,246u8,213u8,49u8,195u8,38u8,14u8,76u8,91u8,210u8,128u8,140u8,51u8,202u8,122u8,134u8,119u8,75u8,228u8,144u8,94u8,197u8,32u8,136u8,75u8,122u8],0,64nat)) by (compute);
    crate::filter_model::checked_row_hint(3u8,seq![68u8,102u8,167u8,4u8,233u8,84u8,165u8,120u8,35u8,9u8,47u8,123u8,112u8,61u8,34u8,81u8,51u8,141u8,157u8,91u8,80u8,215u8,14u8,188u8,55u8,108u8,212u8,188u8,34u8,5u8,42u8,191u8,203u8,169u8,240u8,165u8,177u8,28u8,251u8,157u8,144u8,9u8,64u8,94u8,200u8,187u8,180u8,253u8,172u8,60u8,128u8,108u8,76u8,58u8,9u8,151u8,99u8,65u8,221u8,96u8,188u8,53u8,169u8,29u8],Some(seq![203u8,6u8,207u8,246u8,42u8,118u8,207u8,42u8,175u8,49u8,232u8,196u8,231u8,132u8,1u8,172u8,68u8,233u8,85u8,124u8,217u8,200u8,86u8,132u8,71u8,210u8,80u8,91u8,59u8,235u8,8u8,58u8,17u8,25u8,21u8,92u8,155u8,55u8,192u8,156u8,91u8,38u8,8u8,160u8,51u8,135u8,202u8,180u8,211u8,111u8,164u8,150u8,79u8,77u8,129u8,151u8,130u8,19u8,208u8,0u8,78u8,33u8,205u8,112u8]),8nat,seq![169u8,105u8,14u8,127u8,254u8,143u8,12u8,141u8,207u8,86u8,170u8,28u8,98u8,198u8,40u8,237u8,188u8,44u8,28u8,167u8,237u8,158u8,77u8,116u8,184u8,235u8,10u8,61u8,182u8,201u8,84u8,22u8,47u8,43u8,255u8,241u8,89u8,156u8,133u8,246u8,213u8,49u8,195u8,38u8,14u8,76u8,91u8,210u8,128u8,140u8,51u8,202u8,122u8,134u8,119u8,75u8,228u8,144u8,94u8,197u8,32u8,136u8,75u8,122u8]);
    assert(r::line(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),6nat,2nat)==seq![169u8,105u8,14u8,127u8,254u8,143u8,12u8,141u8,207u8,86u8,170u8,28u8,98u8,198u8,40u8,237u8,188u8,44u8,28u8,167u8,237u8,158u8,77u8,116u8,184u8,235u8,10u8,61u8,182u8,201u8,84u8,22u8,47u8,43u8,255u8,241u8,89u8,156u8,133u8,246u8,213u8,49u8,195u8,38u8,14u8,76u8,91u8,210u8,128u8,140u8,51u8,202u8,122u8,134u8,119u8,75u8,228u8,144u8,94u8,197u8,32u8,136u8,75u8,122u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_2(),8nat)[0]==169u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_2(),8nat)[1]==14u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_2(),8nat)[2]==254u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_2(),8nat)[3]==13u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_2(),8nat)[4]==207u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_2(),8nat)[5]==169u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_2(),8nat)[6]==98u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_2(),8nat)[7]==41u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_2(),8nat)[8]==187u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_2(),8nat)[9]==29u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_2(),8nat)[10]==237u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_2(),8nat)[11]==77u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_2(),8nat)[12]==184u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_2(),8nat)[13]==10u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_2(),8nat)[14]==182u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_2(),8nat)[15]==84u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_2(),8nat)[16]==47u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_2(),8nat)[17]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_2(),8nat)[18]==89u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_2(),8nat)[19]==133u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_2(),8nat)[20]==212u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_2(),8nat)[21]==194u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_2(),8nat)[22]==14u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_2(),8nat)[23]==91u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_2(),8nat)[24]==128u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_2(),8nat)[25]==52u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_2(),8nat)[26]==122u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_2(),8nat)[27]==119u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_2(),8nat)[28]==228u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_2(),8nat)[29]==94u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_2(),8nat)[30]==32u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_2(),8nat)[31]==75u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_2(),8nat),seq![169u8,14u8,254u8,13u8,207u8,169u8,98u8,41u8,187u8,29u8,237u8,77u8,184u8,10u8,182u8,84u8,47u8,255u8,89u8,133u8,212u8,194u8,14u8,91u8,128u8,52u8,122u8,119u8,228u8,94u8,32u8,75u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8(),6nat,2nat)==seq![169u8,14u8,254u8,13u8,207u8,169u8,98u8,41u8,187u8,29u8,237u8,77u8,184u8,10u8,182u8,84u8,47u8,255u8,89u8,133u8,212u8,194u8,14u8,91u8,128u8,52u8,122u8,119u8,228u8,94u8,32u8,75u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct6_d16_8x8(),267nat,64nat,8nat,3nat);
    assert(raw_interlace_ct6_d16_8x8()[462]==4u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct6_d16_8x8(),267nat,64nat,3nat)==seq![15u8,222u8,0u8,198u8,132u8,34u8,52u8,194u8,245u8,100u8,148u8,120u8,236u8,0u8,63u8,138u8,231u8,28u8,65u8,161u8,17u8,55u8,28u8,132u8,244u8,41u8,110u8,170u8,30u8,207u8,233u8,1u8,190u8,70u8,173u8,226u8,54u8,127u8,34u8,147u8,218u8,96u8,165u8,87u8,246u8,177u8,157u8,248u8,113u8,146u8,104u8,184u8,5u8,53u8,156u8,88u8,100u8,247u8,13u8,171u8,34u8,119u8,50u8,114u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct6_d16_8x8(),267nat,64nat,8nat,3nat)==Some(seq![169u8,105u8,14u8,127u8,254u8,143u8,12u8,141u8,207u8,86u8,170u8,28u8,98u8,198u8,40u8,237u8,188u8,44u8,28u8,167u8,237u8,158u8,77u8,116u8,184u8,235u8,10u8,61u8,182u8,201u8,84u8,22u8,47u8,43u8,255u8,241u8,89u8,156u8,133u8,246u8,213u8,49u8,195u8,38u8,14u8,76u8,91u8,210u8,128u8,140u8,51u8,202u8,122u8,134u8,119u8,75u8,228u8,144u8,94u8,197u8,32u8,136u8,75u8,122u8]));
    assert(crate::filter_model::encoded_row(4u8,seq![15u8,222u8,0u8,198u8,132u8,34u8,52u8,194u8,245u8,100u8,148u8,120u8,236u8,0u8,63u8,138u8,231u8,28u8,65u8,161u8,17u8,55u8,28u8,132u8,244u8,41u8,110u8,170u8,30u8,207u8,233u8,1u8,190u8,70u8,173u8,226u8,54u8,127u8,34u8,147u8,218u8,96u8,165u8,87u8,246u8,177u8,157u8,248u8,113u8,146u8,104u8,184u8,5u8,53u8,156u8,88u8,100u8,247u8,13u8,171u8,34u8,119u8,50u8,114u8],Some(seq![169u8,105u8,14u8,127u8,254u8,143u8,12u8,141u8,207u8,86u8,170u8,28u8,98u8,198u8,40u8,237u8,188u8,44u8,28u8,167u8,237u8,158u8,77u8,116u8,184u8,235u8,10u8,61u8,182u8,201u8,84u8,22u8,47u8,43u8,255u8,241u8,89u8,156u8,133u8,246u8,213u8,49u8,195u8,38u8,14u8,76u8,91u8,210u8,128u8,140u8,51u8,202u8,122u8,134u8,119u8,75u8,228u8,144u8,94u8,197u8,32u8,136u8,75u8,122u8]),8nat,seq![184u8,71u8,14u8,69u8,130u8,177u8,64u8,79u8,196u8,171u8,62u8,148u8,78u8,198u8,127u8,23u8,163u8,199u8,93u8,72u8,254u8,213u8,155u8,155u8,151u8,20u8,203u8,231u8,212u8,164u8,132u8,23u8,237u8,90u8,172u8,211u8,143u8,27u8,167u8,137u8,199u8,186u8,81u8,125u8,79u8,204u8,34u8,129u8,241u8,76u8,155u8,130u8,127u8,1u8,190u8,163u8,85u8,67u8,168u8,45u8,66u8,120u8,169u8,21u8],0,64nat)) by (compute);
    crate::filter_model::checked_row_hint(4u8,seq![15u8,222u8,0u8,198u8,132u8,34u8,52u8,194u8,245u8,100u8,148u8,120u8,236u8,0u8,63u8,138u8,231u8,28u8,65u8,161u8,17u8,55u8,28u8,132u8,244u8,41u8,110u8,170u8,30u8,207u8,233u8,1u8,190u8,70u8,173u8,226u8,54u8,127u8,34u8,147u8,218u8,96u8,165u8,87u8,246u8,177u8,157u8,248u8,113u8,146u8,104u8,184u8,5u8,53u8,156u8,88u8,100u8,247u8,13u8,171u8,34u8,119u8,50u8,114u8],Some(seq![169u8,105u8,14u8,127u8,254u8,143u8,12u8,141u8,207u8,86u8,170u8,28u8,98u8,198u8,40u8,237u8,188u8,44u8,28u8,167u8,237u8,158u8,77u8,116u8,184u8,235u8,10u8,61u8,182u8,201u8,84u8,22u8,47u8,43u8,255u8,241u8,89u8,156u8,133u8,246u8,213u8,49u8,195u8,38u8,14u8,76u8,91u8,210u8,128u8,140u8,51u8,202u8,122u8,134u8,119u8,75u8,228u8,144u8,94u8,197u8,32u8,136u8,75u8,122u8]),8nat,seq![184u8,71u8,14u8,69u8,130u8,177u8,64u8,79u8,196u8,171u8,62u8,148u8,78u8,198u8,127u8,23u8,163u8,199u8,93u8,72u8,254u8,213u8,155u8,155u8,151u8,20u8,203u8,231u8,212u8,164u8,132u8,23u8,237u8,90u8,172u8,211u8,143u8,27u8,167u8,137u8,199u8,186u8,81u8,125u8,79u8,204u8,34u8,129u8,241u8,76u8,155u8,130u8,127u8,1u8,190u8,163u8,85u8,67u8,168u8,45u8,66u8,120u8,169u8,21u8]);
    assert(r::line(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),6nat,3nat)==seq![184u8,71u8,14u8,69u8,130u8,177u8,64u8,79u8,196u8,171u8,62u8,148u8,78u8,198u8,127u8,23u8,163u8,199u8,93u8,72u8,254u8,213u8,155u8,155u8,151u8,20u8,203u8,231u8,212u8,164u8,132u8,23u8,237u8,90u8,172u8,211u8,143u8,27u8,167u8,137u8,199u8,186u8,81u8,125u8,79u8,204u8,34u8,129u8,241u8,76u8,155u8,130u8,127u8,1u8,190u8,163u8,85u8,67u8,168u8,45u8,66u8,120u8,169u8,21u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_3(),8nat)[0]==184u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_3(),8nat)[1]==14u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_3(),8nat)[2]==130u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_3(),8nat)[3]==64u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_3(),8nat)[4]==196u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_3(),8nat)[5]==62u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_3(),8nat)[6]==78u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_3(),8nat)[7]==127u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_3(),8nat)[8]==163u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_3(),8nat)[9]==93u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_3(),8nat)[10]==254u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_3(),8nat)[11]==155u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_3(),8nat)[12]==150u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_3(),8nat)[13]==203u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_3(),8nat)[14]==212u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_3(),8nat)[15]==132u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_3(),8nat)[16]==236u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_3(),8nat)[17]==172u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_3(),8nat)[18]==143u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_3(),8nat)[19]==167u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_3(),8nat)[20]==199u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_3(),8nat)[21]==81u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_3(),8nat)[22]==79u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_3(),8nat)[23]==34u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_3(),8nat)[24]==240u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_3(),8nat)[25]==155u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_3(),8nat)[26]==127u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_3(),8nat)[27]==190u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_3(),8nat)[28]==85u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_3(),8nat)[29]==168u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_3(),8nat)[30]==66u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_3(),8nat)[31]==168u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),e6_a7_line_interlace_ct6_d16_8x8_6_3(),8nat),seq![184u8,14u8,130u8,64u8,196u8,62u8,78u8,127u8,163u8,93u8,254u8,155u8,150u8,203u8,212u8,132u8,236u8,172u8,143u8,167u8,199u8,81u8,79u8,34u8,240u8,155u8,127u8,190u8,85u8,168u8,66u8,168u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8(),6nat,3nat)==seq![184u8,14u8,130u8,64u8,196u8,62u8,78u8,127u8,163u8,93u8,254u8,155u8,150u8,203u8,212u8,132u8,236u8,172u8,143u8,167u8,199u8,81u8,79u8,34u8,240u8,155u8,127u8,190u8,85u8,168u8,66u8,168u8]) by (compute);
    assert forall|y: nat| y<4 implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),r::line(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),6nat,y),8nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8(),6nat,y) by {
        if y==0 {} else if y==1 {} else if y==2 {} else {assert(y==3);}
    }
}
#[verifier::spinoff_prover]
pub(crate) proof fn e6_a7_complete_interlace_ct6_d16_8x8()
    ensures match crate::still_model::MS(crate::witness_m6_m4_a7::bytes_interlace_ct6_d16_8x8()) {crate::still_model::Outcome::Done(value)=>crate::witness_m6_m4_a7::expected_value_interlace_ct6_d16_8x8(value),_=>false},
{

    hide(bytes_interlace_ct6_d16_8x8);hide(z_interlace_ct6_d16_8x8);hide(raw_interlace_ct6_d16_8x8);hide(r::data);hide(crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_interlace_ct6_d16_8x8().len()==595nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct6_d16_8x8())==Ok((crate::model::Ihdr {width:8u32,height:8u32,depth:16u8,color_type:6u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct6_d16_8x8())==Ok((crate::model::Ihdr {width:8u32,height:8u32,depth:16u8,color_type:6u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct6_d16_8x8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:538nat,next:583nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::model::frame(bytes_interlace_ct6_d16_8x8(),33nat)==Ok(crate::model::WireChunk {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:538nat,next:583nat})) by (compute);
            assert(crate::model::be32(bytes_interlace_ct6_d16_8x8(),579)==3769748576u32) by (compute);
            crc_interlace_ct6_d16_8x8();
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct6_d16_8x8(),583nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:591nat,len:0nat,next:595nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct6_d16_8x8(),583nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:591nat,len:0nat,next:595nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct6_d16_8x8().subrange(41,579)==z_interlace_ct6_d16_8x8()) by (compute);
    walk_interlace_ct6_d16_8x8(bytes_interlace_ct6_d16_8x8());

    model_interlace_ct6_d16_8x8();inflate_interlace_ct6_d16_8x8();scan_interlace_ct6_d16_8x8();
    assert(bytes_interlace_ct6_d16_8x8()==crate::witness_m6_m4_a7::bytes_interlace_ct6_d16_8x8()) by (compute);
    assert(r::raw(a_interlace_ct6_d16_8x8())==527nat) by (compute);
    assert(r::data(z_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    crate::witness_m6_rows_model::bare_colour(bytes_interlace_ct6_d16_8x8(),h_interlace_ct6_d16_8x8(),538nat);
    e6_a7_pass_interlace_ct6_d16_8x8_0();
    e6_a7_pass_interlace_ct6_d16_8x8_1();
    e6_a7_pass_interlace_ct6_d16_8x8_2();
    e6_a7_pass_interlace_ct6_d16_8x8_3();
    e6_a7_pass_interlace_ct6_d16_8x8_4();
    e6_a7_pass_interlace_ct6_d16_8x8_5();
    e6_a7_pass_interlace_ct6_d16_8x8_6();
    assert(r::count(a_interlace_ct6_d16_8x8())==7nat && crate::paint_model::domain(a_interlace_ct6_d16_8x8()) && crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8().len()==256nat) by (compute);
    assert(r::pw(a_interlace_ct6_d16_8x8(),0nat)==1nat && r::ph(a_interlace_ct6_d16_8x8(),0nat)==1nat) by (compute);
    assert(r::pw(a_interlace_ct6_d16_8x8(),1nat)==1nat && r::ph(a_interlace_ct6_d16_8x8(),1nat)==1nat) by (compute);
    assert(r::pw(a_interlace_ct6_d16_8x8(),2nat)==2nat && r::ph(a_interlace_ct6_d16_8x8(),2nat)==1nat) by (compute);
    assert(r::pw(a_interlace_ct6_d16_8x8(),3nat)==2nat && r::ph(a_interlace_ct6_d16_8x8(),3nat)==2nat) by (compute);
    assert(r::pw(a_interlace_ct6_d16_8x8(),4nat)==4nat && r::ph(a_interlace_ct6_d16_8x8(),4nat)==2nat) by (compute);
    assert(r::pw(a_interlace_ct6_d16_8x8(),5nat)==4nat && r::ph(a_interlace_ct6_d16_8x8(),5nat)==4nat) by (compute);
    assert(r::pw(a_interlace_ct6_d16_8x8(),6nat)==8nat && r::ph(a_interlace_ct6_d16_8x8(),6nat)==4nat) by (compute);
    assert forall|k: nat,y: nat| k<7 && r::pw(a_interlace_ct6_d16_8x8(),k)>0 && y<r::ph(a_interlace_ct6_d16_8x8(),k) implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_8x8(),r::line(raw_interlace_ct6_d16_8x8(),a_interlace_ct6_d16_8x8(),k,y),r::pw(a_interlace_ct6_d16_8x8(),k))==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8(),k,y) by {
        if k==0 {} else if k==1 {} else if k==2 {} else if k==3 {} else if k==4 {} else if k==5 {} else {assert(k==6);}
    }
    crate::witness_m6_a7_model::complete_still(bytes_interlace_ct6_d16_8x8(),h_interlace_ct6_d16_8x8(),z_interlace_ct6_d16_8x8(),raw_interlace_ct6_d16_8x8(),0nat,e6_a7_colour_interlace_ct6_d16_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_8x8());
}
spec fn e6_a7_colour_interlace_ct3_d4_8x8() -> crate::colour_model::Context {crate::colour_model::Context {ct:3u8,depth:4u8,plte:seq![165u8,103u8,16u8,216u8,80u8,163u8,55u8,55u8,10u8,232u8,206u8,223u8,196u8,80u8,190u8,188u8,245u8,56u8,80u8,77u8,4u8,208u8,230u8,64u8,64u8,41u8,221u8,174u8,5u8,203u8,122u8,134u8,225u8,164u8,7u8,2u8,101u8,184u8,165u8,103u8,210u8,85u8,155u8,126u8,208u8,241u8,161u8,167u8],trns:None}}
spec fn e6_a7_line_interlace_ct3_d4_8x8_0_0() -> Seq<u8> {seq![96u8]}
#[verifier::spinoff_prover]
proof fn e6_a7_pass_interlace_ct3_d4_8x8_0()
    ensures forall|y: nat| y<1 ==> #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),0nat,y),1nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct3_d4_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8(),0nat,y),
{
    hide(raw_interlace_ct3_d4_8x8);hide(crate::colour_model::pixel);hide(crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8);
    crate::colour_model::depth_values(4u8);crate::colour_model::depth_values(8u8);
    assert(r::pw(a_interlace_ct3_d4_8x8(),0nat)==1nat && r::ph(a_interlace_ct3_d4_8x8(),0nat)==1nat && r::base(a_interlace_ct3_d4_8x8(),0nat)==0nat && r::rb(a_interlace_ct3_d4_8x8(),0nat)==1nat && crate::filter_model::bpp(r::bits(a_interlace_ct3_d4_8x8()))==1nat) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct3_d4_8x8(),0nat,1nat,1nat,0nat);
    assert(raw_interlace_ct3_d4_8x8()[0]==0u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct3_d4_8x8(),0nat,1nat,0nat)==seq![96u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct3_d4_8x8(),0nat,1nat,1nat,0nat)==None);
    assert(crate::filter_model::encoded_row(0u8,seq![96u8],None,1nat,seq![96u8],0,1nat)) by (compute);
    crate::filter_model::checked_row_hint(0u8,seq![96u8],None,1nat,seq![96u8]);
    assert(r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),0nat,0nat)==seq![96u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_0_0(),1nat)[0]==80u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_0_0(),1nat)[1]==77u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_0_0(),1nat)[2]==4u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_0_0(),1nat)[3]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_0_0(),1nat),seq![80u8,77u8,4u8,255u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct3_d4_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8(),0nat,0nat)==seq![80u8,77u8,4u8,255u8]) by (compute);
    assert forall|y: nat| y<1 implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),0nat,y),1nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct3_d4_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8(),0nat,y) by {
        assert(y==0);
    }
}
spec fn e6_a7_line_interlace_ct3_d4_8x8_1_0() -> Seq<u8> {seq![16u8]}
#[verifier::spinoff_prover]
proof fn e6_a7_pass_interlace_ct3_d4_8x8_1()
    ensures forall|y: nat| y<1 ==> #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),1nat,y),1nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct3_d4_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8(),1nat,y),
{
    hide(raw_interlace_ct3_d4_8x8);hide(crate::colour_model::pixel);hide(crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8);
    crate::colour_model::depth_values(4u8);crate::colour_model::depth_values(8u8);
    assert(r::pw(a_interlace_ct3_d4_8x8(),1nat)==1nat && r::ph(a_interlace_ct3_d4_8x8(),1nat)==1nat && r::base(a_interlace_ct3_d4_8x8(),1nat)==2nat && r::rb(a_interlace_ct3_d4_8x8(),1nat)==1nat && crate::filter_model::bpp(r::bits(a_interlace_ct3_d4_8x8()))==1nat) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct3_d4_8x8(),2nat,1nat,1nat,0nat);
    assert(raw_interlace_ct3_d4_8x8()[2]==1u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct3_d4_8x8(),2nat,1nat,0nat)==seq![16u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct3_d4_8x8(),2nat,1nat,1nat,0nat)==None);
    assert(crate::filter_model::encoded_row(1u8,seq![16u8],None,1nat,seq![16u8],0,1nat)) by (compute);
    crate::filter_model::checked_row_hint(1u8,seq![16u8],None,1nat,seq![16u8]);
    assert(r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),1nat,0nat)==seq![16u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_1_0(),1nat)[0]==216u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_1_0(),1nat)[1]==80u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_1_0(),1nat)[2]==163u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_1_0(),1nat)[3]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_1_0(),1nat),seq![216u8,80u8,163u8,255u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct3_d4_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8(),1nat,0nat)==seq![216u8,80u8,163u8,255u8]) by (compute);
    assert forall|y: nat| y<1 implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),1nat,y),1nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct3_d4_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8(),1nat,y) by {
        assert(y==0);
    }
}
spec fn e6_a7_line_interlace_ct3_d4_8x8_2_0() -> Seq<u8> {seq![88u8]}
#[verifier::spinoff_prover]
proof fn e6_a7_pass_interlace_ct3_d4_8x8_2()
    ensures forall|y: nat| y<1 ==> #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),2nat,y),2nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct3_d4_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8(),2nat,y),
{
    hide(raw_interlace_ct3_d4_8x8);hide(crate::colour_model::pixel);hide(crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8);
    crate::colour_model::depth_values(4u8);crate::colour_model::depth_values(8u8);
    assert(r::pw(a_interlace_ct3_d4_8x8(),2nat)==2nat && r::ph(a_interlace_ct3_d4_8x8(),2nat)==1nat && r::base(a_interlace_ct3_d4_8x8(),2nat)==4nat && r::rb(a_interlace_ct3_d4_8x8(),2nat)==1nat && crate::filter_model::bpp(r::bits(a_interlace_ct3_d4_8x8()))==1nat) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct3_d4_8x8(),4nat,1nat,1nat,0nat);
    assert(raw_interlace_ct3_d4_8x8()[4]==2u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct3_d4_8x8(),4nat,1nat,0nat)==seq![88u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct3_d4_8x8(),4nat,1nat,1nat,0nat)==None);
    assert(crate::filter_model::encoded_row(2u8,seq![88u8],None,1nat,seq![88u8],0,1nat)) by (compute);
    crate::filter_model::checked_row_hint(2u8,seq![88u8],None,1nat,seq![88u8]);
    assert(r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),2nat,0nat)==seq![88u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_2_0(),2nat)[0]==188u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_2_0(),2nat)[1]==245u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_2_0(),2nat)[2]==56u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_2_0(),2nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_2_0(),2nat)[4]==64u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_2_0(),2nat)[5]==41u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_2_0(),2nat)[6]==221u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_2_0(),2nat)[7]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_2_0(),2nat),seq![188u8,245u8,56u8,255u8,64u8,41u8,221u8,255u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct3_d4_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8(),2nat,0nat)==seq![188u8,245u8,56u8,255u8,64u8,41u8,221u8,255u8]) by (compute);
    assert forall|y: nat| y<1 implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),2nat,y),2nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct3_d4_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8(),2nat,y) by {
        assert(y==0);
    }
}
spec fn e6_a7_line_interlace_ct3_d4_8x8_3_0() -> Seq<u8> {seq![139u8]}
spec fn e6_a7_line_interlace_ct3_d4_8x8_3_1() -> Seq<u8> {seq![148u8]}
#[verifier::spinoff_prover]
proof fn e6_a7_pass_interlace_ct3_d4_8x8_3()
    ensures forall|y: nat| y<2 ==> #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),3nat,y),2nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct3_d4_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8(),3nat,y),
{
    hide(raw_interlace_ct3_d4_8x8);hide(crate::colour_model::pixel);hide(crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8);
    crate::colour_model::depth_values(4u8);crate::colour_model::depth_values(8u8);
    assert(r::pw(a_interlace_ct3_d4_8x8(),3nat)==2nat && r::ph(a_interlace_ct3_d4_8x8(),3nat)==2nat && r::base(a_interlace_ct3_d4_8x8(),3nat)==6nat && r::rb(a_interlace_ct3_d4_8x8(),3nat)==1nat && crate::filter_model::bpp(r::bits(a_interlace_ct3_d4_8x8()))==1nat) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct3_d4_8x8(),6nat,1nat,1nat,0nat);
    assert(raw_interlace_ct3_d4_8x8()[6]==3u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct3_d4_8x8(),6nat,1nat,0nat)==seq![139u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct3_d4_8x8(),6nat,1nat,1nat,0nat)==None);
    assert(crate::filter_model::encoded_row(3u8,seq![139u8],None,1nat,seq![139u8],0,1nat)) by (compute);
    crate::filter_model::checked_row_hint(3u8,seq![139u8],None,1nat,seq![139u8]);
    assert(r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),3nat,0nat)==seq![139u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_3_0(),2nat)[0]==64u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_3_0(),2nat)[1]==41u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_3_0(),2nat)[2]==221u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_3_0(),2nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_3_0(),2nat)[4]==164u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_3_0(),2nat)[5]==7u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_3_0(),2nat)[6]==2u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_3_0(),2nat)[7]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_3_0(),2nat),seq![64u8,41u8,221u8,255u8,164u8,7u8,2u8,255u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct3_d4_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8(),3nat,0nat)==seq![64u8,41u8,221u8,255u8,164u8,7u8,2u8,255u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct3_d4_8x8(),6nat,1nat,1nat,1nat);
    assert(raw_interlace_ct3_d4_8x8()[8]==4u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct3_d4_8x8(),6nat,1nat,1nat)==seq![9u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct3_d4_8x8(),6nat,1nat,1nat,1nat)==Some(seq![139u8]));
    assert(crate::filter_model::encoded_row(4u8,seq![9u8],Some(seq![139u8]),1nat,seq![148u8],0,1nat)) by (compute);
    crate::filter_model::checked_row_hint(4u8,seq![9u8],Some(seq![139u8]),1nat,seq![148u8]);
    assert(r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),3nat,1nat)==seq![148u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_3_1(),2nat)[0]==174u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_3_1(),2nat)[1]==5u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_3_1(),2nat)[2]==203u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_3_1(),2nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_3_1(),2nat)[4]==196u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_3_1(),2nat)[5]==80u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_3_1(),2nat)[6]==190u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_3_1(),2nat)[7]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_3_1(),2nat),seq![174u8,5u8,203u8,255u8,196u8,80u8,190u8,255u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct3_d4_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8(),3nat,1nat)==seq![174u8,5u8,203u8,255u8,196u8,80u8,190u8,255u8]) by (compute);
    assert forall|y: nat| y<2 implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),3nat,y),2nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct3_d4_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8(),3nat,y) by {
        if y==0 {} else {assert(y==1);}
    }
}
spec fn e6_a7_line_interlace_ct3_d4_8x8_4_0() -> Seq<u8> {seq![30u8,210u8]}
spec fn e6_a7_line_interlace_ct3_d4_8x8_4_1() -> Seq<u8> {seq![160u8,132u8]}
#[verifier::spinoff_prover]
proof fn e6_a7_pass_interlace_ct3_d4_8x8_4()
    ensures forall|y: nat| y<2 ==> #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),4nat,y),4nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct3_d4_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8(),4nat,y),
{
    hide(raw_interlace_ct3_d4_8x8);hide(crate::colour_model::pixel);hide(crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8);
    crate::colour_model::depth_values(4u8);crate::colour_model::depth_values(8u8);
    assert(r::pw(a_interlace_ct3_d4_8x8(),4nat)==4nat && r::ph(a_interlace_ct3_d4_8x8(),4nat)==2nat && r::base(a_interlace_ct3_d4_8x8(),4nat)==10nat && r::rb(a_interlace_ct3_d4_8x8(),4nat)==2nat && crate::filter_model::bpp(r::bits(a_interlace_ct3_d4_8x8()))==1nat) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct3_d4_8x8(),10nat,2nat,1nat,0nat);
    assert(raw_interlace_ct3_d4_8x8()[10]==0u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct3_d4_8x8(),10nat,2nat,0nat)==seq![30u8,210u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct3_d4_8x8(),10nat,2nat,1nat,0nat)==None);
    assert(crate::filter_model::encoded_row(0u8,seq![30u8,210u8],None,1nat,seq![30u8,210u8],0,2nat)) by (compute);
    crate::filter_model::checked_row_hint(0u8,seq![30u8,210u8],None,1nat,seq![30u8,210u8]);
    assert(r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),4nat,0nat)==seq![30u8,210u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_4_0(),4nat)[0]==216u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_4_0(),4nat)[1]==80u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_4_0(),4nat)[2]==163u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_4_0(),4nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_4_0(),4nat)[4]==155u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_4_0(),4nat)[5]==126u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_4_0(),4nat)[6]==208u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_4_0(),4nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_4_0(),4nat)[8]==103u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_4_0(),4nat)[9]==210u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_4_0(),4nat)[10]==85u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_4_0(),4nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_4_0(),4nat)[12]==55u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_4_0(),4nat)[13]==55u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_4_0(),4nat)[14]==10u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_4_0(),4nat)[15]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_4_0(),4nat),seq![216u8,80u8,163u8,255u8,155u8,126u8,208u8,255u8,103u8,210u8,85u8,255u8,55u8,55u8,10u8,255u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct3_d4_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8(),4nat,0nat)==seq![216u8,80u8,163u8,255u8,155u8,126u8,208u8,255u8,103u8,210u8,85u8,255u8,55u8,55u8,10u8,255u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct3_d4_8x8(),10nat,2nat,1nat,1nat);
    assert(raw_interlace_ct3_d4_8x8()[13]==1u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct3_d4_8x8(),10nat,2nat,1nat)==seq![160u8,228u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct3_d4_8x8(),10nat,2nat,1nat,1nat)==Some(seq![30u8,210u8]));
    assert(crate::filter_model::encoded_row(1u8,seq![160u8,228u8],Some(seq![30u8,210u8]),1nat,seq![160u8,132u8],0,2nat)) by (compute);
    crate::filter_model::checked_row_hint(1u8,seq![160u8,228u8],Some(seq![30u8,210u8]),1nat,seq![160u8,132u8]);
    assert(r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),4nat,1nat)==seq![160u8,132u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_4_1(),4nat)[0]==122u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_4_1(),4nat)[1]==134u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_4_1(),4nat)[2]==225u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_4_1(),4nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_4_1(),4nat)[4]==165u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_4_1(),4nat)[5]==103u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_4_1(),4nat)[6]==16u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_4_1(),4nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_4_1(),4nat)[8]==64u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_4_1(),4nat)[9]==41u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_4_1(),4nat)[10]==221u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_4_1(),4nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_4_1(),4nat)[12]==196u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_4_1(),4nat)[13]==80u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_4_1(),4nat)[14]==190u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_4_1(),4nat)[15]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_4_1(),4nat),seq![122u8,134u8,225u8,255u8,165u8,103u8,16u8,255u8,64u8,41u8,221u8,255u8,196u8,80u8,190u8,255u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct3_d4_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8(),4nat,1nat)==seq![122u8,134u8,225u8,255u8,165u8,103u8,16u8,255u8,64u8,41u8,221u8,255u8,196u8,80u8,190u8,255u8]) by (compute);
    assert forall|y: nat| y<2 implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),4nat,y),4nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct3_d4_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8(),4nat,y) by {
        if y==0 {} else {assert(y==1);}
    }
}
spec fn e6_a7_line_interlace_ct3_d4_8x8_5_0() -> Seq<u8> {seq![123u8,240u8]}
spec fn e6_a7_line_interlace_ct3_d4_8x8_5_1() -> Seq<u8> {seq![37u8,235u8]}
spec fn e6_a7_line_interlace_ct3_d4_8x8_5_2() -> Seq<u8> {seq![39u8,202u8]}
spec fn e6_a7_line_interlace_ct3_d4_8x8_5_3() -> Seq<u8> {seq![200u8,45u8]}
#[verifier::spinoff_prover]
proof fn e6_a7_pass_interlace_ct3_d4_8x8_5()
    ensures forall|y: nat| y<4 ==> #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),5nat,y),4nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct3_d4_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8(),5nat,y),
{
    hide(raw_interlace_ct3_d4_8x8);hide(crate::colour_model::pixel);hide(crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8);
    crate::colour_model::depth_values(4u8);crate::colour_model::depth_values(8u8);
    assert(r::pw(a_interlace_ct3_d4_8x8(),5nat)==4nat && r::ph(a_interlace_ct3_d4_8x8(),5nat)==4nat && r::base(a_interlace_ct3_d4_8x8(),5nat)==16nat && r::rb(a_interlace_ct3_d4_8x8(),5nat)==2nat && crate::filter_model::bpp(r::bits(a_interlace_ct3_d4_8x8()))==1nat) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct3_d4_8x8(),16nat,2nat,1nat,0nat);
    assert(raw_interlace_ct3_d4_8x8()[16]==2u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct3_d4_8x8(),16nat,2nat,0nat)==seq![123u8,240u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct3_d4_8x8(),16nat,2nat,1nat,0nat)==None);
    assert(crate::filter_model::encoded_row(2u8,seq![123u8,240u8],None,1nat,seq![123u8,240u8],0,2nat)) by (compute);
    crate::filter_model::checked_row_hint(2u8,seq![123u8,240u8],None,1nat,seq![123u8,240u8]);
    assert(r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),5nat,0nat)==seq![123u8,240u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_0(),4nat)[0]==208u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_0(),4nat)[1]==230u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_0(),4nat)[2]==64u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_0(),4nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_0(),4nat)[4]==164u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_0(),4nat)[5]==7u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_0(),4nat)[6]==2u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_0(),4nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_0(),4nat)[8]==241u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_0(),4nat)[9]==161u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_0(),4nat)[10]==167u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_0(),4nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_0(),4nat)[12]==165u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_0(),4nat)[13]==103u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_0(),4nat)[14]==16u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_0(),4nat)[15]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_0(),4nat),seq![208u8,230u8,64u8,255u8,164u8,7u8,2u8,255u8,241u8,161u8,167u8,255u8,165u8,103u8,16u8,255u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct3_d4_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8(),5nat,0nat)==seq![208u8,230u8,64u8,255u8,164u8,7u8,2u8,255u8,241u8,161u8,167u8,255u8,165u8,103u8,16u8,255u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct3_d4_8x8(),16nat,2nat,1nat,1nat);
    assert(raw_interlace_ct3_d4_8x8()[19]==3u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct3_d4_8x8(),16nat,2nat,1nat)==seq![232u8,97u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct3_d4_8x8(),16nat,2nat,1nat,1nat)==Some(seq![123u8,240u8]));
    assert(crate::filter_model::encoded_row(3u8,seq![232u8,97u8],Some(seq![123u8,240u8]),1nat,seq![37u8,235u8],0,2nat)) by (compute);
    crate::filter_model::checked_row_hint(3u8,seq![232u8,97u8],Some(seq![123u8,240u8]),1nat,seq![37u8,235u8]);
    assert(r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),5nat,1nat)==seq![37u8,235u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_1(),4nat)[0]==55u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_1(),4nat)[1]==55u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_1(),4nat)[2]==10u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_1(),4nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_1(),4nat)[4]==188u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_1(),4nat)[5]==245u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_1(),4nat)[6]==56u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_1(),4nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_1(),4nat)[8]==155u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_1(),4nat)[9]==126u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_1(),4nat)[10]==208u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_1(),4nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_1(),4nat)[12]==164u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_1(),4nat)[13]==7u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_1(),4nat)[14]==2u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_1(),4nat)[15]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_1(),4nat),seq![55u8,55u8,10u8,255u8,188u8,245u8,56u8,255u8,155u8,126u8,208u8,255u8,164u8,7u8,2u8,255u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct3_d4_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8(),5nat,1nat)==seq![55u8,55u8,10u8,255u8,188u8,245u8,56u8,255u8,155u8,126u8,208u8,255u8,164u8,7u8,2u8,255u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct3_d4_8x8(),16nat,2nat,1nat,2nat);
    assert(raw_interlace_ct3_d4_8x8()[22]==4u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct3_d4_8x8(),16nat,2nat,2nat)==seq![2u8,223u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct3_d4_8x8(),16nat,2nat,1nat,2nat)==Some(seq![37u8,235u8]));
    assert(crate::filter_model::encoded_row(4u8,seq![2u8,223u8],Some(seq![37u8,235u8]),1nat,seq![39u8,202u8],0,2nat)) by (compute);
    crate::filter_model::checked_row_hint(4u8,seq![2u8,223u8],Some(seq![37u8,235u8]),1nat,seq![39u8,202u8]);
    assert(r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),5nat,2nat)==seq![39u8,202u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_2(),4nat)[0]==55u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_2(),4nat)[1]==55u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_2(),4nat)[2]==10u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_2(),4nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_2(),4nat)[4]==208u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_2(),4nat)[5]==230u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_2(),4nat)[6]==64u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_2(),4nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_2(),4nat)[8]==101u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_2(),4nat)[9]==184u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_2(),4nat)[10]==165u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_2(),4nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_2(),4nat)[12]==122u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_2(),4nat)[13]==134u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_2(),4nat)[14]==225u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_2(),4nat)[15]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_2(),4nat),seq![55u8,55u8,10u8,255u8,208u8,230u8,64u8,255u8,101u8,184u8,165u8,255u8,122u8,134u8,225u8,255u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct3_d4_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8(),5nat,2nat)==seq![55u8,55u8,10u8,255u8,208u8,230u8,64u8,255u8,101u8,184u8,165u8,255u8,122u8,134u8,225u8,255u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct3_d4_8x8(),16nat,2nat,1nat,3nat);
    assert(raw_interlace_ct3_d4_8x8()[25]==0u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct3_d4_8x8(),16nat,2nat,3nat)==seq![200u8,45u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct3_d4_8x8(),16nat,2nat,1nat,3nat)==Some(seq![39u8,202u8]));
    assert(crate::filter_model::encoded_row(0u8,seq![200u8,45u8],Some(seq![39u8,202u8]),1nat,seq![200u8,45u8],0,2nat)) by (compute);
    crate::filter_model::checked_row_hint(0u8,seq![200u8,45u8],Some(seq![39u8,202u8]),1nat,seq![200u8,45u8]);
    assert(r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),5nat,3nat)==seq![200u8,45u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_3(),4nat)[0]==101u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_3(),4nat)[1]==184u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_3(),4nat)[2]==165u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_3(),4nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_3(),4nat)[4]==64u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_3(),4nat)[5]==41u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_3(),4nat)[6]==221u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_3(),4nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_3(),4nat)[8]==55u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_3(),4nat)[9]==55u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_3(),4nat)[10]==10u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_3(),4nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_3(),4nat)[12]==103u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_3(),4nat)[13]==210u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_3(),4nat)[14]==85u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_3(),4nat)[15]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_5_3(),4nat),seq![101u8,184u8,165u8,255u8,64u8,41u8,221u8,255u8,55u8,55u8,10u8,255u8,103u8,210u8,85u8,255u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct3_d4_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8(),5nat,3nat)==seq![101u8,184u8,165u8,255u8,64u8,41u8,221u8,255u8,55u8,55u8,10u8,255u8,103u8,210u8,85u8,255u8]) by (compute);
    assert forall|y: nat| y<4 implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),5nat,y),4nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct3_d4_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8(),5nat,y) by {
        if y==0 {} else if y==1 {} else if y==2 {} else {assert(y==3);}
    }
}
spec fn e6_a7_line_interlace_ct3_d4_8x8_6_0() -> Seq<u8> {seq![60u8,140u8,221u8,246u8]}
spec fn e6_a7_line_interlace_ct3_d4_8x8_6_1() -> Seq<u8> {seq![56u8,206u8,160u8,172u8]}
spec fn e6_a7_line_interlace_ct3_d4_8x8_6_2() -> Seq<u8> {seq![196u8,23u8,128u8,142u8]}
spec fn e6_a7_line_interlace_ct3_d4_8x8_6_3() -> Seq<u8> {seq![64u8,205u8,144u8,41u8]}
#[verifier::spinoff_prover]
proof fn e6_a7_pass_interlace_ct3_d4_8x8_6()
    ensures forall|y: nat| y<4 ==> #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),6nat,y),8nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct3_d4_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8(),6nat,y),
{
    hide(raw_interlace_ct3_d4_8x8);hide(crate::colour_model::pixel);hide(crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8);
    crate::colour_model::depth_values(4u8);crate::colour_model::depth_values(8u8);
    assert(r::pw(a_interlace_ct3_d4_8x8(),6nat)==8nat && r::ph(a_interlace_ct3_d4_8x8(),6nat)==4nat && r::base(a_interlace_ct3_d4_8x8(),6nat)==28nat && r::rb(a_interlace_ct3_d4_8x8(),6nat)==4nat && crate::filter_model::bpp(r::bits(a_interlace_ct3_d4_8x8()))==1nat) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct3_d4_8x8(),28nat,4nat,1nat,0nat);
    assert(raw_interlace_ct3_d4_8x8()[28]==1u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct3_d4_8x8(),28nat,4nat,0nat)==seq![60u8,80u8,81u8,25u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct3_d4_8x8(),28nat,4nat,1nat,0nat)==None);
    assert(crate::filter_model::encoded_row(1u8,seq![60u8,80u8,81u8,25u8],None,1nat,seq![60u8,140u8,221u8,246u8],0,4nat)) by (compute);
    crate::filter_model::checked_row_hint(1u8,seq![60u8,80u8,81u8,25u8],None,1nat,seq![60u8,140u8,221u8,246u8]);
    assert(r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),6nat,0nat)==seq![60u8,140u8,221u8,246u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_0(),8nat)[0]==232u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_0(),8nat)[1]==206u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_0(),8nat)[2]==223u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_0(),8nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_0(),8nat)[4]==101u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_0(),8nat)[5]==184u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_0(),8nat)[6]==165u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_0(),8nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_0(),8nat)[8]==64u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_0(),8nat)[9]==41u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_0(),8nat)[10]==221u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_0(),8nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_0(),8nat)[12]==101u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_0(),8nat)[13]==184u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_0(),8nat)[14]==165u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_0(),8nat)[15]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_0(),8nat)[16]==103u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_0(),8nat)[17]==210u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_0(),8nat)[18]==85u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_0(),8nat)[19]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_0(),8nat)[20]==103u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_0(),8nat)[21]==210u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_0(),8nat)[22]==85u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_0(),8nat)[23]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_0(),8nat)[24]==241u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_0(),8nat)[25]==161u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_0(),8nat)[26]==167u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_0(),8nat)[27]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_0(),8nat)[28]==80u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_0(),8nat)[29]==77u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_0(),8nat)[30]==4u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_0(),8nat)[31]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_0(),8nat),seq![232u8,206u8,223u8,255u8,101u8,184u8,165u8,255u8,64u8,41u8,221u8,255u8,101u8,184u8,165u8,255u8,103u8,210u8,85u8,255u8,103u8,210u8,85u8,255u8,241u8,161u8,167u8,255u8,80u8,77u8,4u8,255u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct3_d4_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8(),6nat,0nat)==seq![232u8,206u8,223u8,255u8,101u8,184u8,165u8,255u8,64u8,41u8,221u8,255u8,101u8,184u8,165u8,255u8,103u8,210u8,85u8,255u8,103u8,210u8,85u8,255u8,241u8,161u8,167u8,255u8,80u8,77u8,4u8,255u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct3_d4_8x8(),28nat,4nat,1nat,1nat);
    assert(raw_interlace_ct3_d4_8x8()[33]==2u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct3_d4_8x8(),28nat,4nat,1nat)==seq![252u8,66u8,195u8,182u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct3_d4_8x8(),28nat,4nat,1nat,1nat)==Some(seq![60u8,140u8,221u8,246u8]));
    assert(crate::filter_model::encoded_row(2u8,seq![252u8,66u8,195u8,182u8],Some(seq![60u8,140u8,221u8,246u8]),1nat,seq![56u8,206u8,160u8,172u8],0,4nat)) by (compute);
    crate::filter_model::checked_row_hint(2u8,seq![252u8,66u8,195u8,182u8],Some(seq![60u8,140u8,221u8,246u8]),1nat,seq![56u8,206u8,160u8,172u8]);
    assert(r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),6nat,1nat)==seq![56u8,206u8,160u8,172u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_1(),8nat)[0]==232u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_1(),8nat)[1]==206u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_1(),8nat)[2]==223u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_1(),8nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_1(),8nat)[4]==64u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_1(),8nat)[5]==41u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_1(),8nat)[6]==221u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_1(),8nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_1(),8nat)[8]==101u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_1(),8nat)[9]==184u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_1(),8nat)[10]==165u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_1(),8nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_1(),8nat)[12]==155u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_1(),8nat)[13]==126u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_1(),8nat)[14]==208u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_1(),8nat)[15]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_1(),8nat)[16]==122u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_1(),8nat)[17]==134u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_1(),8nat)[18]==225u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_1(),8nat)[19]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_1(),8nat)[20]==165u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_1(),8nat)[21]==103u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_1(),8nat)[22]==16u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_1(),8nat)[23]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_1(),8nat)[24]==122u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_1(),8nat)[25]==134u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_1(),8nat)[26]==225u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_1(),8nat)[27]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_1(),8nat)[28]==101u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_1(),8nat)[29]==184u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_1(),8nat)[30]==165u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_1(),8nat)[31]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_1(),8nat),seq![232u8,206u8,223u8,255u8,64u8,41u8,221u8,255u8,101u8,184u8,165u8,255u8,155u8,126u8,208u8,255u8,122u8,134u8,225u8,255u8,165u8,103u8,16u8,255u8,122u8,134u8,225u8,255u8,101u8,184u8,165u8,255u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct3_d4_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8(),6nat,1nat)==seq![232u8,206u8,223u8,255u8,64u8,41u8,221u8,255u8,101u8,184u8,165u8,255u8,155u8,126u8,208u8,255u8,122u8,134u8,225u8,255u8,165u8,103u8,16u8,255u8,122u8,134u8,225u8,255u8,101u8,184u8,165u8,255u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct3_d4_8x8(),28nat,4nat,1nat,2nat);
    assert(raw_interlace_ct3_d4_8x8()[38]==3u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct3_d4_8x8(),28nat,4nat,2nat)==seq![168u8,78u8,37u8,248u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct3_d4_8x8(),28nat,4nat,1nat,2nat)==Some(seq![56u8,206u8,160u8,172u8]));
    assert(crate::filter_model::encoded_row(3u8,seq![168u8,78u8,37u8,248u8],Some(seq![56u8,206u8,160u8,172u8]),1nat,seq![196u8,23u8,128u8,142u8],0,4nat)) by (compute);
    crate::filter_model::checked_row_hint(3u8,seq![168u8,78u8,37u8,248u8],Some(seq![56u8,206u8,160u8,172u8]),1nat,seq![196u8,23u8,128u8,142u8]);
    assert(r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),6nat,2nat)==seq![196u8,23u8,128u8,142u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_2(),8nat)[0]==101u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_2(),8nat)[1]==184u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_2(),8nat)[2]==165u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_2(),8nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_2(),8nat)[4]==196u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_2(),8nat)[5]==80u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_2(),8nat)[6]==190u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_2(),8nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_2(),8nat)[8]==216u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_2(),8nat)[9]==80u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_2(),8nat)[10]==163u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_2(),8nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_2(),8nat)[12]==208u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_2(),8nat)[13]==230u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_2(),8nat)[14]==64u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_2(),8nat)[15]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_2(),8nat)[16]==64u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_2(),8nat)[17]==41u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_2(),8nat)[18]==221u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_2(),8nat)[19]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_2(),8nat)[20]==165u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_2(),8nat)[21]==103u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_2(),8nat)[22]==16u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_2(),8nat)[23]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_2(),8nat)[24]==64u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_2(),8nat)[25]==41u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_2(),8nat)[26]==221u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_2(),8nat)[27]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_2(),8nat)[28]==155u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_2(),8nat)[29]==126u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_2(),8nat)[30]==208u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_2(),8nat)[31]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_2(),8nat),seq![101u8,184u8,165u8,255u8,196u8,80u8,190u8,255u8,216u8,80u8,163u8,255u8,208u8,230u8,64u8,255u8,64u8,41u8,221u8,255u8,165u8,103u8,16u8,255u8,64u8,41u8,221u8,255u8,155u8,126u8,208u8,255u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct3_d4_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8(),6nat,2nat)==seq![101u8,184u8,165u8,255u8,196u8,80u8,190u8,255u8,216u8,80u8,163u8,255u8,208u8,230u8,64u8,255u8,64u8,41u8,221u8,255u8,165u8,103u8,16u8,255u8,64u8,41u8,221u8,255u8,155u8,126u8,208u8,255u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct3_d4_8x8(),28nat,4nat,1nat,3nat);
    assert(raw_interlace_ct3_d4_8x8()[43]==4u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct3_d4_8x8(),28nat,4nat,3nat)==seq![124u8,182u8,195u8,153u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct3_d4_8x8(),28nat,4nat,1nat,3nat)==Some(seq![196u8,23u8,128u8,142u8]));
    assert(crate::filter_model::encoded_row(4u8,seq![124u8,182u8,195u8,153u8],Some(seq![196u8,23u8,128u8,142u8]),1nat,seq![64u8,205u8,144u8,41u8],0,4nat)) by (compute);
    crate::filter_model::checked_row_hint(4u8,seq![124u8,182u8,195u8,153u8],Some(seq![196u8,23u8,128u8,142u8]),1nat,seq![64u8,205u8,144u8,41u8]);
    assert(r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),6nat,3nat)==seq![64u8,205u8,144u8,41u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_3(),8nat)[0]==196u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_3(),8nat)[1]==80u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_3(),8nat)[2]==190u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_3(),8nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_3(),8nat)[4]==165u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_3(),8nat)[5]==103u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_3(),8nat)[6]==16u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_3(),8nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_3(),8nat)[8]==101u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_3(),8nat)[9]==184u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_3(),8nat)[10]==165u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_3(),8nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_3(),8nat)[12]==103u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_3(),8nat)[13]==210u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_3(),8nat)[14]==85u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_3(),8nat)[15]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_3(),8nat)[16]==174u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_3(),8nat)[17]==5u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_3(),8nat)[18]==203u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_3(),8nat)[19]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_3(),8nat)[20]==165u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_3(),8nat)[21]==103u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_3(),8nat)[22]==16u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_3(),8nat)[23]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_3(),8nat)[24]==55u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_3(),8nat)[25]==55u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_3(),8nat)[26]==10u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_3(),8nat)[27]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_3(),8nat)[28]==174u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_3(),8nat)[29]==5u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_3(),8nat)[30]==203u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_3(),8nat)[31]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),e6_a7_line_interlace_ct3_d4_8x8_6_3(),8nat),seq![196u8,80u8,190u8,255u8,165u8,103u8,16u8,255u8,101u8,184u8,165u8,255u8,103u8,210u8,85u8,255u8,174u8,5u8,203u8,255u8,165u8,103u8,16u8,255u8,55u8,55u8,10u8,255u8,174u8,5u8,203u8,255u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct3_d4_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8(),6nat,3nat)==seq![196u8,80u8,190u8,255u8,165u8,103u8,16u8,255u8,101u8,184u8,165u8,255u8,103u8,210u8,85u8,255u8,174u8,5u8,203u8,255u8,165u8,103u8,16u8,255u8,55u8,55u8,10u8,255u8,174u8,5u8,203u8,255u8]) by (compute);
    assert forall|y: nat| y<4 implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),6nat,y),8nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct3_d4_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8(),6nat,y) by {
        if y==0 {} else if y==1 {} else if y==2 {} else {assert(y==3);}
    }
}
#[verifier::spinoff_prover]
pub(crate) proof fn e6_a7_complete_interlace_ct3_d4_8x8()
    ensures match crate::still_model::MS(crate::witness_m6_m4_a7::bytes_interlace_ct3_d4_8x8()) {crate::still_model::Outcome::Done(value)=>crate::witness_m6_m4_a7::expected_value_interlace_ct3_d4_8x8(value),_=>false},
{

    hide(bytes_interlace_ct3_d4_8x8);hide(z_interlace_ct3_d4_8x8);hide(raw_interlace_ct3_d4_8x8);hide(r::data);hide(crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8);
    hide(crate::critical_model::read);hide(crate::model::mh);
    assert(bytes_interlace_ct3_d4_8x8().len()==176nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct3_d4_8x8())==Ok((crate::model::Ihdr {width:8u32,height:8u32,depth:4u8,color_type:3u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct3_d4_8x8())==Ok((crate::model::Ihdr {width:8u32,height:8u32,depth:4u8,color_type:3u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 4u8)==16u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_8x8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_8x8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(48nat,3u8,4u8,false,false,false,false)==crate::metadata_model::Decision::Keep(16nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_8x8(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:59nat,next:164nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_8x8(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:59nat,next:164nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_8x8(),164nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:172nat,len:0nat,next:176nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_8x8(),164nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:172nat,len:0nat,next:176nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct3_d4_8x8().subrange(101,160)==z_interlace_ct3_d4_8x8()) by (compute);
    walk_interlace_ct3_d4_8x8(bytes_interlace_ct3_d4_8x8());

    model_interlace_ct3_d4_8x8();inflate_interlace_ct3_d4_8x8();scan_interlace_ct3_d4_8x8();
    assert(bytes_interlace_ct3_d4_8x8()==crate::witness_m6_m4_a7::bytes_interlace_ct3_d4_8x8()) by (compute);
    assert(r::raw(a_interlace_ct3_d4_8x8())==48nat) by (compute);
    assert(r::data(z_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    assert(bytes_interlace_ct3_d4_8x8().subrange(41,89)==seq![165u8,103u8,16u8,216u8,80u8,163u8,55u8,55u8,10u8,232u8,206u8,223u8,196u8,80u8,190u8,188u8,245u8,56u8,80u8,77u8,4u8,208u8,230u8,64u8,64u8,41u8,221u8,174u8,5u8,203u8,122u8,134u8,225u8,164u8,7u8,2u8,101u8,184u8,165u8,103u8,210u8,85u8,155u8,126u8,208u8,241u8,161u8,167u8]) by (compute);
    crate::witness_m6_a7_model::palette_colour(bytes_interlace_ct3_d4_8x8(),h_interlace_ct3_d4_8x8(),59nat);
    e6_a7_pass_interlace_ct3_d4_8x8_0();
    e6_a7_pass_interlace_ct3_d4_8x8_1();
    e6_a7_pass_interlace_ct3_d4_8x8_2();
    e6_a7_pass_interlace_ct3_d4_8x8_3();
    e6_a7_pass_interlace_ct3_d4_8x8_4();
    e6_a7_pass_interlace_ct3_d4_8x8_5();
    e6_a7_pass_interlace_ct3_d4_8x8_6();
    assert(r::count(a_interlace_ct3_d4_8x8())==7nat && crate::paint_model::domain(a_interlace_ct3_d4_8x8()) && crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8().len()==256nat) by (compute);
    assert(r::pw(a_interlace_ct3_d4_8x8(),0nat)==1nat && r::ph(a_interlace_ct3_d4_8x8(),0nat)==1nat) by (compute);
    assert(r::pw(a_interlace_ct3_d4_8x8(),1nat)==1nat && r::ph(a_interlace_ct3_d4_8x8(),1nat)==1nat) by (compute);
    assert(r::pw(a_interlace_ct3_d4_8x8(),2nat)==2nat && r::ph(a_interlace_ct3_d4_8x8(),2nat)==1nat) by (compute);
    assert(r::pw(a_interlace_ct3_d4_8x8(),3nat)==2nat && r::ph(a_interlace_ct3_d4_8x8(),3nat)==2nat) by (compute);
    assert(r::pw(a_interlace_ct3_d4_8x8(),4nat)==4nat && r::ph(a_interlace_ct3_d4_8x8(),4nat)==2nat) by (compute);
    assert(r::pw(a_interlace_ct3_d4_8x8(),5nat)==4nat && r::ph(a_interlace_ct3_d4_8x8(),5nat)==4nat) by (compute);
    assert(r::pw(a_interlace_ct3_d4_8x8(),6nat)==8nat && r::ph(a_interlace_ct3_d4_8x8(),6nat)==4nat) by (compute);
    assert forall|k: nat,y: nat| k<7 && r::pw(a_interlace_ct3_d4_8x8(),k)>0 && y<r::ph(a_interlace_ct3_d4_8x8(),k) implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_8x8(),r::line(raw_interlace_ct3_d4_8x8(),a_interlace_ct3_d4_8x8(),k,y),r::pw(a_interlace_ct3_d4_8x8(),k))==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct3_d4_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8(),k,y) by {
        if k==0 {} else if k==1 {} else if k==2 {} else if k==3 {} else if k==4 {} else if k==5 {} else {assert(k==6);}
    }
    crate::witness_m6_a7_model::complete_still(bytes_interlace_ct3_d4_8x8(),h_interlace_ct3_d4_8x8(),z_interlace_ct3_d4_8x8(),raw_interlace_ct3_d4_8x8(),16nat,e6_a7_colour_interlace_ct3_d4_8x8(),crate::witness_m6_m4_a7::expected_interlace_ct3_d4_8x8());
}
spec fn e6_a7_colour_interlace_ct6_d16_1x9() -> crate::colour_model::Context {crate::colour_model::Context {ct:6u8,depth:16u8,plte:seq![],trns:None}}
spec fn e6_a7_line_interlace_ct6_d16_1x9_0_0() -> Seq<u8> {seq![253u8,103u8,180u8,101u8,210u8,97u8,178u8,156u8]}
spec fn e6_a7_line_interlace_ct6_d16_1x9_0_1() -> Seq<u8> {seq![56u8,19u8,63u8,80u8,49u8,204u8,67u8,203u8]}
#[verifier::spinoff_prover]
proof fn e6_a7_pass_interlace_ct6_d16_1x9_0()
    ensures forall|y: nat| y<2 ==> #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),r::line(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),0nat,y),1nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_1x9(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_1x9(),0nat,y),
{
    hide(raw_interlace_ct6_d16_1x9);hide(crate::colour_model::pixel);hide(crate::witness_m6_m4_a7::expected_interlace_ct6_d16_1x9);
    crate::colour_model::depth_values(16u8);crate::colour_model::depth_values(8u8);
    assert(r::pw(a_interlace_ct6_d16_1x9(),0nat)==1nat && r::ph(a_interlace_ct6_d16_1x9(),0nat)==2nat && r::base(a_interlace_ct6_d16_1x9(),0nat)==0nat && r::rb(a_interlace_ct6_d16_1x9(),0nat)==8nat && crate::filter_model::bpp(r::bits(a_interlace_ct6_d16_1x9()))==8nat) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct6_d16_1x9(),0nat,8nat,8nat,0nat);
    assert(raw_interlace_ct6_d16_1x9()[0]==0u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct6_d16_1x9(),0nat,8nat,0nat)==seq![253u8,103u8,180u8,101u8,210u8,97u8,178u8,156u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct6_d16_1x9(),0nat,8nat,8nat,0nat)==None);
    assert(crate::filter_model::encoded_row(0u8,seq![253u8,103u8,180u8,101u8,210u8,97u8,178u8,156u8],None,8nat,seq![253u8,103u8,180u8,101u8,210u8,97u8,178u8,156u8],0,8nat)) by (compute);
    crate::filter_model::checked_row_hint(0u8,seq![253u8,103u8,180u8,101u8,210u8,97u8,178u8,156u8],None,8nat,seq![253u8,103u8,180u8,101u8,210u8,97u8,178u8,156u8]);
    assert(r::line(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),0nat,0nat)==seq![253u8,103u8,180u8,101u8,210u8,97u8,178u8,156u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_0_0(),1nat)[0]==252u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_0_0(),1nat)[1]==180u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_0_0(),1nat)[2]==210u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_0_0(),1nat)[3]==178u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_0_0(),1nat),seq![252u8,180u8,210u8,178u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_1x9(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_1x9(),0nat,0nat)==seq![252u8,180u8,210u8,178u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct6_d16_1x9(),0nat,8nat,8nat,1nat);
    assert(raw_interlace_ct6_d16_1x9()[9]==1u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct6_d16_1x9(),0nat,8nat,1nat)==seq![56u8,19u8,63u8,80u8,49u8,204u8,67u8,203u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct6_d16_1x9(),0nat,8nat,8nat,1nat)==Some(seq![253u8,103u8,180u8,101u8,210u8,97u8,178u8,156u8]));
    assert(crate::filter_model::encoded_row(1u8,seq![56u8,19u8,63u8,80u8,49u8,204u8,67u8,203u8],Some(seq![253u8,103u8,180u8,101u8,210u8,97u8,178u8,156u8]),8nat,seq![56u8,19u8,63u8,80u8,49u8,204u8,67u8,203u8],0,8nat)) by (compute);
    crate::filter_model::checked_row_hint(1u8,seq![56u8,19u8,63u8,80u8,49u8,204u8,67u8,203u8],Some(seq![253u8,103u8,180u8,101u8,210u8,97u8,178u8,156u8]),8nat,seq![56u8,19u8,63u8,80u8,49u8,204u8,67u8,203u8]);
    assert(r::line(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),0nat,1nat)==seq![56u8,19u8,63u8,80u8,49u8,204u8,67u8,203u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_0_1(),1nat)[0]==56u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_0_1(),1nat)[1]==63u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_0_1(),1nat)[2]==50u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_0_1(),1nat)[3]==68u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_0_1(),1nat),seq![56u8,63u8,50u8,68u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_1x9(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_1x9(),0nat,1nat)==seq![56u8,63u8,50u8,68u8]) by (compute);
    assert forall|y: nat| y<2 implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),r::line(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),0nat,y),1nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_1x9(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_1x9(),0nat,y) by {
        if y==0 {} else {assert(y==1);}
    }
}
spec fn e6_a7_line_interlace_ct6_d16_1x9_2_0() -> Seq<u8> {seq![172u8,136u8,45u8,185u8,89u8,170u8,76u8,252u8]}
#[verifier::spinoff_prover]
proof fn e6_a7_pass_interlace_ct6_d16_1x9_2()
    ensures forall|y: nat| y<1 ==> #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),r::line(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),2nat,y),1nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_1x9(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_1x9(),2nat,y),
{
    hide(raw_interlace_ct6_d16_1x9);hide(crate::colour_model::pixel);hide(crate::witness_m6_m4_a7::expected_interlace_ct6_d16_1x9);
    crate::colour_model::depth_values(16u8);crate::colour_model::depth_values(8u8);
    assert(r::pw(a_interlace_ct6_d16_1x9(),2nat)==1nat && r::ph(a_interlace_ct6_d16_1x9(),2nat)==1nat && r::base(a_interlace_ct6_d16_1x9(),2nat)==18nat && r::rb(a_interlace_ct6_d16_1x9(),2nat)==8nat && crate::filter_model::bpp(r::bits(a_interlace_ct6_d16_1x9()))==8nat) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct6_d16_1x9(),18nat,8nat,8nat,0nat);
    assert(raw_interlace_ct6_d16_1x9()[18]==2u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct6_d16_1x9(),18nat,8nat,0nat)==seq![172u8,136u8,45u8,185u8,89u8,170u8,76u8,252u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct6_d16_1x9(),18nat,8nat,8nat,0nat)==None);
    assert(crate::filter_model::encoded_row(2u8,seq![172u8,136u8,45u8,185u8,89u8,170u8,76u8,252u8],None,8nat,seq![172u8,136u8,45u8,185u8,89u8,170u8,76u8,252u8],0,8nat)) by (compute);
    crate::filter_model::checked_row_hint(2u8,seq![172u8,136u8,45u8,185u8,89u8,170u8,76u8,252u8],None,8nat,seq![172u8,136u8,45u8,185u8,89u8,170u8,76u8,252u8]);
    assert(r::line(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),2nat,0nat)==seq![172u8,136u8,45u8,185u8,89u8,170u8,76u8,252u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_2_0(),1nat)[0]==172u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_2_0(),1nat)[1]==46u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_2_0(),1nat)[2]==89u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_2_0(),1nat)[3]==77u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_2_0(),1nat),seq![172u8,46u8,89u8,77u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_1x9(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_1x9(),2nat,0nat)==seq![172u8,46u8,89u8,77u8]) by (compute);
    assert forall|y: nat| y<1 implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),r::line(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),2nat,y),1nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_1x9(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_1x9(),2nat,y) by {
        assert(y==0);
    }
}
spec fn e6_a7_line_interlace_ct6_d16_1x9_4_0() -> Seq<u8> {seq![13u8,28u8,217u8,34u8,235u8,59u8,236u8,173u8]}
spec fn e6_a7_line_interlace_ct6_d16_1x9_4_1() -> Seq<u8> {seq![174u8,154u8,217u8,191u8,155u8,70u8,195u8,92u8]}
#[verifier::spinoff_prover]
proof fn e6_a7_pass_interlace_ct6_d16_1x9_4()
    ensures forall|y: nat| y<2 ==> #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),r::line(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),4nat,y),1nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_1x9(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_1x9(),4nat,y),
{
    hide(raw_interlace_ct6_d16_1x9);hide(crate::colour_model::pixel);hide(crate::witness_m6_m4_a7::expected_interlace_ct6_d16_1x9);
    crate::colour_model::depth_values(16u8);crate::colour_model::depth_values(8u8);
    assert(r::pw(a_interlace_ct6_d16_1x9(),4nat)==1nat && r::ph(a_interlace_ct6_d16_1x9(),4nat)==2nat && r::base(a_interlace_ct6_d16_1x9(),4nat)==27nat && r::rb(a_interlace_ct6_d16_1x9(),4nat)==8nat && crate::filter_model::bpp(r::bits(a_interlace_ct6_d16_1x9()))==8nat) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct6_d16_1x9(),27nat,8nat,8nat,0nat);
    assert(raw_interlace_ct6_d16_1x9()[27]==3u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct6_d16_1x9(),27nat,8nat,0nat)==seq![13u8,28u8,217u8,34u8,235u8,59u8,236u8,173u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct6_d16_1x9(),27nat,8nat,8nat,0nat)==None);
    assert(crate::filter_model::encoded_row(3u8,seq![13u8,28u8,217u8,34u8,235u8,59u8,236u8,173u8],None,8nat,seq![13u8,28u8,217u8,34u8,235u8,59u8,236u8,173u8],0,8nat)) by (compute);
    crate::filter_model::checked_row_hint(3u8,seq![13u8,28u8,217u8,34u8,235u8,59u8,236u8,173u8],None,8nat,seq![13u8,28u8,217u8,34u8,235u8,59u8,236u8,173u8]);
    assert(r::line(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),4nat,0nat)==seq![13u8,28u8,217u8,34u8,235u8,59u8,236u8,173u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_4_0(),1nat)[0]==13u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_4_0(),1nat)[1]==216u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_4_0(),1nat)[2]==234u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_4_0(),1nat)[3]==236u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_4_0(),1nat),seq![13u8,216u8,234u8,236u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_1x9(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_1x9(),4nat,0nat)==seq![13u8,216u8,234u8,236u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct6_d16_1x9(),27nat,8nat,8nat,1nat);
    assert(raw_interlace_ct6_d16_1x9()[36]==4u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct6_d16_1x9(),27nat,8nat,1nat)==seq![161u8,126u8,0u8,157u8,176u8,11u8,215u8,175u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct6_d16_1x9(),27nat,8nat,8nat,1nat)==Some(seq![13u8,28u8,217u8,34u8,235u8,59u8,236u8,173u8]));
    assert(crate::filter_model::encoded_row(4u8,seq![161u8,126u8,0u8,157u8,176u8,11u8,215u8,175u8],Some(seq![13u8,28u8,217u8,34u8,235u8,59u8,236u8,173u8]),8nat,seq![174u8,154u8,217u8,191u8,155u8,70u8,195u8,92u8],0,8nat)) by (compute);
    crate::filter_model::checked_row_hint(4u8,seq![161u8,126u8,0u8,157u8,176u8,11u8,215u8,175u8],Some(seq![13u8,28u8,217u8,34u8,235u8,59u8,236u8,173u8]),8nat,seq![174u8,154u8,217u8,191u8,155u8,70u8,195u8,92u8]);
    assert(r::line(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),4nat,1nat)==seq![174u8,154u8,217u8,191u8,155u8,70u8,195u8,92u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_4_1(),1nat)[0]==174u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_4_1(),1nat)[1]==217u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_4_1(),1nat)[2]==155u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_4_1(),1nat)[3]==195u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_4_1(),1nat),seq![174u8,217u8,155u8,195u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_1x9(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_1x9(),4nat,1nat)==seq![174u8,217u8,155u8,195u8]) by (compute);
    assert forall|y: nat| y<2 implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),r::line(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),4nat,y),1nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_1x9(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_1x9(),4nat,y) by {
        if y==0 {} else {assert(y==1);}
    }
}
spec fn e6_a7_line_interlace_ct6_d16_1x9_6_0() -> Seq<u8> {seq![129u8,23u8,35u8,113u8,99u8,127u8,29u8,85u8]}
spec fn e6_a7_line_interlace_ct6_d16_1x9_6_1() -> Seq<u8> {seq![187u8,186u8,184u8,95u8,244u8,102u8,52u8,30u8]}
spec fn e6_a7_line_interlace_ct6_d16_1x9_6_2() -> Seq<u8> {seq![25u8,64u8,191u8,42u8,94u8,132u8,251u8,73u8]}
spec fn e6_a7_line_interlace_ct6_d16_1x9_6_3() -> Seq<u8> {seq![160u8,2u8,232u8,200u8,75u8,240u8,37u8,99u8]}
#[verifier::spinoff_prover]
proof fn e6_a7_pass_interlace_ct6_d16_1x9_6()
    ensures forall|y: nat| y<4 ==> #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),r::line(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),6nat,y),1nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_1x9(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_1x9(),6nat,y),
{
    hide(raw_interlace_ct6_d16_1x9);hide(crate::colour_model::pixel);hide(crate::witness_m6_m4_a7::expected_interlace_ct6_d16_1x9);
    crate::colour_model::depth_values(16u8);crate::colour_model::depth_values(8u8);
    assert(r::pw(a_interlace_ct6_d16_1x9(),6nat)==1nat && r::ph(a_interlace_ct6_d16_1x9(),6nat)==4nat && r::base(a_interlace_ct6_d16_1x9(),6nat)==45nat && r::rb(a_interlace_ct6_d16_1x9(),6nat)==8nat && crate::filter_model::bpp(r::bits(a_interlace_ct6_d16_1x9()))==8nat) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct6_d16_1x9(),45nat,8nat,8nat,0nat);
    assert(raw_interlace_ct6_d16_1x9()[45]==0u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct6_d16_1x9(),45nat,8nat,0nat)==seq![129u8,23u8,35u8,113u8,99u8,127u8,29u8,85u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct6_d16_1x9(),45nat,8nat,8nat,0nat)==None);
    assert(crate::filter_model::encoded_row(0u8,seq![129u8,23u8,35u8,113u8,99u8,127u8,29u8,85u8],None,8nat,seq![129u8,23u8,35u8,113u8,99u8,127u8,29u8,85u8],0,8nat)) by (compute);
    crate::filter_model::checked_row_hint(0u8,seq![129u8,23u8,35u8,113u8,99u8,127u8,29u8,85u8],None,8nat,seq![129u8,23u8,35u8,113u8,99u8,127u8,29u8,85u8]);
    assert(r::line(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),6nat,0nat)==seq![129u8,23u8,35u8,113u8,99u8,127u8,29u8,85u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_6_0(),1nat)[0]==129u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_6_0(),1nat)[1]==35u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_6_0(),1nat)[2]==99u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_6_0(),1nat)[3]==29u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_6_0(),1nat),seq![129u8,35u8,99u8,29u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_1x9(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_1x9(),6nat,0nat)==seq![129u8,35u8,99u8,29u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct6_d16_1x9(),45nat,8nat,8nat,1nat);
    assert(raw_interlace_ct6_d16_1x9()[54]==1u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct6_d16_1x9(),45nat,8nat,1nat)==seq![187u8,186u8,184u8,95u8,244u8,102u8,52u8,30u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct6_d16_1x9(),45nat,8nat,8nat,1nat)==Some(seq![129u8,23u8,35u8,113u8,99u8,127u8,29u8,85u8]));
    assert(crate::filter_model::encoded_row(1u8,seq![187u8,186u8,184u8,95u8,244u8,102u8,52u8,30u8],Some(seq![129u8,23u8,35u8,113u8,99u8,127u8,29u8,85u8]),8nat,seq![187u8,186u8,184u8,95u8,244u8,102u8,52u8,30u8],0,8nat)) by (compute);
    crate::filter_model::checked_row_hint(1u8,seq![187u8,186u8,184u8,95u8,244u8,102u8,52u8,30u8],Some(seq![129u8,23u8,35u8,113u8,99u8,127u8,29u8,85u8]),8nat,seq![187u8,186u8,184u8,95u8,244u8,102u8,52u8,30u8]);
    assert(r::line(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),6nat,1nat)==seq![187u8,186u8,184u8,95u8,244u8,102u8,52u8,30u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_6_1(),1nat)[0]==187u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_6_1(),1nat)[1]==184u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_6_1(),1nat)[2]==243u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_6_1(),1nat)[3]==52u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_6_1(),1nat),seq![187u8,184u8,243u8,52u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_1x9(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_1x9(),6nat,1nat)==seq![187u8,184u8,243u8,52u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct6_d16_1x9(),45nat,8nat,8nat,2nat);
    assert(raw_interlace_ct6_d16_1x9()[63]==2u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct6_d16_1x9(),45nat,8nat,2nat)==seq![94u8,134u8,7u8,203u8,106u8,30u8,199u8,43u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct6_d16_1x9(),45nat,8nat,8nat,2nat)==Some(seq![187u8,186u8,184u8,95u8,244u8,102u8,52u8,30u8]));
    assert(crate::filter_model::encoded_row(2u8,seq![94u8,134u8,7u8,203u8,106u8,30u8,199u8,43u8],Some(seq![187u8,186u8,184u8,95u8,244u8,102u8,52u8,30u8]),8nat,seq![25u8,64u8,191u8,42u8,94u8,132u8,251u8,73u8],0,8nat)) by (compute);
    crate::filter_model::checked_row_hint(2u8,seq![94u8,134u8,7u8,203u8,106u8,30u8,199u8,43u8],Some(seq![187u8,186u8,184u8,95u8,244u8,102u8,52u8,30u8]),8nat,seq![25u8,64u8,191u8,42u8,94u8,132u8,251u8,73u8]);
    assert(r::line(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),6nat,2nat)==seq![25u8,64u8,191u8,42u8,94u8,132u8,251u8,73u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_6_2(),1nat)[0]==25u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_6_2(),1nat)[1]==190u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_6_2(),1nat)[2]==94u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_6_2(),1nat)[3]==250u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_6_2(),1nat),seq![25u8,190u8,94u8,250u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_1x9(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_1x9(),6nat,2nat)==seq![25u8,190u8,94u8,250u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct6_d16_1x9(),45nat,8nat,8nat,3nat);
    assert(raw_interlace_ct6_d16_1x9()[72]==3u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct6_d16_1x9(),45nat,8nat,3nat)==seq![148u8,226u8,137u8,179u8,28u8,174u8,168u8,63u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct6_d16_1x9(),45nat,8nat,8nat,3nat)==Some(seq![25u8,64u8,191u8,42u8,94u8,132u8,251u8,73u8]));
    assert(crate::filter_model::encoded_row(3u8,seq![148u8,226u8,137u8,179u8,28u8,174u8,168u8,63u8],Some(seq![25u8,64u8,191u8,42u8,94u8,132u8,251u8,73u8]),8nat,seq![160u8,2u8,232u8,200u8,75u8,240u8,37u8,99u8],0,8nat)) by (compute);
    crate::filter_model::checked_row_hint(3u8,seq![148u8,226u8,137u8,179u8,28u8,174u8,168u8,63u8],Some(seq![25u8,64u8,191u8,42u8,94u8,132u8,251u8,73u8]),8nat,seq![160u8,2u8,232u8,200u8,75u8,240u8,37u8,99u8]);
    assert(r::line(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),6nat,3nat)==seq![160u8,2u8,232u8,200u8,75u8,240u8,37u8,99u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_6_3(),1nat)[0]==159u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_6_3(),1nat)[1]==232u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_6_3(),1nat)[2]==76u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_6_3(),1nat)[3]==37u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),e6_a7_line_interlace_ct6_d16_1x9_6_3(),1nat),seq![159u8,232u8,76u8,37u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_1x9(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_1x9(),6nat,3nat)==seq![159u8,232u8,76u8,37u8]) by (compute);
    assert forall|y: nat| y<4 implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),r::line(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),6nat,y),1nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_1x9(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_1x9(),6nat,y) by {
        if y==0 {} else if y==1 {} else if y==2 {} else {assert(y==3);}
    }
}
#[verifier::spinoff_prover]
pub(crate) proof fn e6_a7_complete_interlace_ct6_d16_1x9()
    ensures match crate::still_model::MS(crate::witness_m6_m4_a7::bytes_interlace_ct6_d16_1x9()) {crate::still_model::Outcome::Done(value)=>crate::witness_m6_m4_a7::expected_value_interlace_ct6_d16_1x9(value),_=>false},
{

    hide(bytes_interlace_ct6_d16_1x9);hide(z_interlace_ct6_d16_1x9);hide(raw_interlace_ct6_d16_1x9);hide(r::data);hide(crate::witness_m6_m4_a7::expected_interlace_ct6_d16_1x9);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_interlace_ct6_d16_1x9().len()==149nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct6_d16_1x9())==Ok((crate::model::Ihdr {width:1u32,height:9u32,depth:16u8,color_type:6u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct6_d16_1x9())==Ok((crate::model::Ihdr {width:1u32,height:9u32,depth:16u8,color_type:6u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct6_d16_1x9(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:92nat,next:137nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct6_d16_1x9(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:92nat,next:137nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct6_d16_1x9(),137nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:145nat,len:0nat,next:149nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct6_d16_1x9(),137nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:145nat,len:0nat,next:149nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct6_d16_1x9().subrange(41,133)==z_interlace_ct6_d16_1x9()) by (compute);
    walk_interlace_ct6_d16_1x9(bytes_interlace_ct6_d16_1x9());

    model_interlace_ct6_d16_1x9();inflate_interlace_ct6_d16_1x9();scan_interlace_ct6_d16_1x9();
    assert(bytes_interlace_ct6_d16_1x9()==crate::witness_m6_m4_a7::bytes_interlace_ct6_d16_1x9()) by (compute);
    assert(r::raw(a_interlace_ct6_d16_1x9())==81nat) by (compute);
    assert(r::data(z_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    crate::witness_m6_rows_model::bare_colour(bytes_interlace_ct6_d16_1x9(),h_interlace_ct6_d16_1x9(),92nat);
    e6_a7_pass_interlace_ct6_d16_1x9_0();
    e6_a7_pass_interlace_ct6_d16_1x9_2();
    e6_a7_pass_interlace_ct6_d16_1x9_4();
    e6_a7_pass_interlace_ct6_d16_1x9_6();
    assert(r::count(a_interlace_ct6_d16_1x9())==7nat && crate::paint_model::domain(a_interlace_ct6_d16_1x9()) && crate::witness_m6_m4_a7::expected_interlace_ct6_d16_1x9().len()==36nat) by (compute);
    assert(r::pw(a_interlace_ct6_d16_1x9(),0nat)==1nat && r::ph(a_interlace_ct6_d16_1x9(),0nat)==2nat) by (compute);
    assert(r::pw(a_interlace_ct6_d16_1x9(),1nat)==0nat && r::ph(a_interlace_ct6_d16_1x9(),1nat)==2nat) by (compute);
    assert(r::pw(a_interlace_ct6_d16_1x9(),2nat)==1nat && r::ph(a_interlace_ct6_d16_1x9(),2nat)==1nat) by (compute);
    assert(r::pw(a_interlace_ct6_d16_1x9(),3nat)==0nat && r::ph(a_interlace_ct6_d16_1x9(),3nat)==3nat) by (compute);
    assert(r::pw(a_interlace_ct6_d16_1x9(),4nat)==1nat && r::ph(a_interlace_ct6_d16_1x9(),4nat)==2nat) by (compute);
    assert(r::pw(a_interlace_ct6_d16_1x9(),5nat)==0nat && r::ph(a_interlace_ct6_d16_1x9(),5nat)==5nat) by (compute);
    assert(r::pw(a_interlace_ct6_d16_1x9(),6nat)==1nat && r::ph(a_interlace_ct6_d16_1x9(),6nat)==4nat) by (compute);
    assert forall|k: nat,y: nat| k<7 && r::pw(a_interlace_ct6_d16_1x9(),k)>0 && y<r::ph(a_interlace_ct6_d16_1x9(),k) implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct6_d16_1x9(),r::line(raw_interlace_ct6_d16_1x9(),a_interlace_ct6_d16_1x9(),k,y),r::pw(a_interlace_ct6_d16_1x9(),k))==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct6_d16_1x9(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_1x9(),k,y) by {
        if k==0 {} else if k==1 {} else if k==2 {} else if k==3 {} else if k==4 {} else if k==5 {} else {assert(k==6);}
    }
    crate::witness_m6_a7_model::complete_still(bytes_interlace_ct6_d16_1x9(),h_interlace_ct6_d16_1x9(),z_interlace_ct6_d16_1x9(),raw_interlace_ct6_d16_1x9(),0nat,e6_a7_colour_interlace_ct6_d16_1x9(),crate::witness_m6_m4_a7::expected_interlace_ct6_d16_1x9());
}
spec fn e6_a7_colour_interlace_ct3_d4_1x1() -> crate::colour_model::Context {crate::colour_model::Context {ct:3u8,depth:4u8,plte:seq![81u8,144u8,125u8,101u8,64u8,177u8,228u8,169u8,188u8,76u8,40u8,153u8,237u8,218u8,35u8,151u8,40u8,162u8,34u8,24u8,141u8,23u8,54u8,198u8,120u8,240u8,209u8,125u8,189u8,50u8,29u8,165u8,196u8,115u8,208u8,128u8,195u8,46u8,41u8,101u8,192u8,41u8,152u8,245u8,128u8,108u8,162u8,192u8],trns:None}}
spec fn e6_a7_line_interlace_ct3_d4_1x1_0_0() -> Seq<u8> {seq![48u8]}
#[verifier::spinoff_prover]
proof fn e6_a7_pass_interlace_ct3_d4_1x1_0()
    ensures forall|y: nat| y<1 ==> #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_1x1(),r::line(raw_interlace_ct3_d4_1x1(),a_interlace_ct3_d4_1x1(),0nat,y),1nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct3_d4_1x1(),crate::witness_m6_m4_a7::expected_interlace_ct3_d4_1x1(),0nat,y),
{
    hide(raw_interlace_ct3_d4_1x1);hide(crate::colour_model::pixel);hide(crate::witness_m6_m4_a7::expected_interlace_ct3_d4_1x1);
    crate::colour_model::depth_values(4u8);crate::colour_model::depth_values(8u8);
    assert(r::pw(a_interlace_ct3_d4_1x1(),0nat)==1nat && r::ph(a_interlace_ct3_d4_1x1(),0nat)==1nat && r::base(a_interlace_ct3_d4_1x1(),0nat)==0nat && r::rb(a_interlace_ct3_d4_1x1(),0nat)==1nat && crate::filter_model::bpp(r::bits(a_interlace_ct3_d4_1x1()))==1nat) by (compute);
    crate::filter_model::pass_row_equation(raw_interlace_ct3_d4_1x1(),0nat,1nat,1nat,0nat);
    assert(raw_interlace_ct3_d4_1x1()[0]==0u8) by (compute);
    assert(crate::filter_model::row_data(raw_interlace_ct3_d4_1x1(),0nat,1nat,0nat)==seq![48u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_interlace_ct3_d4_1x1(),0nat,1nat,1nat,0nat)==None);
    assert(crate::filter_model::encoded_row(0u8,seq![48u8],None,1nat,seq![48u8],0,1nat)) by (compute);
    crate::filter_model::checked_row_hint(0u8,seq![48u8],None,1nat,seq![48u8]);
    assert(r::line(raw_interlace_ct3_d4_1x1(),a_interlace_ct3_d4_1x1(),0nat,0nat)==seq![48u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_1x1(),e6_a7_line_interlace_ct3_d4_1x1_0_0(),1nat)[0]==76u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_1x1(),e6_a7_line_interlace_ct3_d4_1x1_0_0(),1nat)[1]==40u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_1x1(),e6_a7_line_interlace_ct3_d4_1x1_0_0(),1nat)[2]==153u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_1x1(),e6_a7_line_interlace_ct3_d4_1x1_0_0(),1nat)[3]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_1x1(),e6_a7_line_interlace_ct3_d4_1x1_0_0(),1nat),seq![76u8,40u8,153u8,255u8]);
    assert(crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct3_d4_1x1(),crate::witness_m6_m4_a7::expected_interlace_ct3_d4_1x1(),0nat,0nat)==seq![76u8,40u8,153u8,255u8]) by (compute);
    assert forall|y: nat| y<1 implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_1x1(),r::line(raw_interlace_ct3_d4_1x1(),a_interlace_ct3_d4_1x1(),0nat,y),1nat)==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct3_d4_1x1(),crate::witness_m6_m4_a7::expected_interlace_ct3_d4_1x1(),0nat,y) by {
        assert(y==0);
    }
}
#[verifier::spinoff_prover]
pub(crate) proof fn e6_a7_complete_interlace_ct3_d4_1x1()
    ensures match crate::still_model::MS(crate::witness_m6_m4_a7::bytes_interlace_ct3_d4_1x1()) {crate::still_model::Outcome::Done(value)=>crate::witness_m6_m4_a7::expected_value_interlace_ct3_d4_1x1(value),_=>false},
{

    hide(bytes_interlace_ct3_d4_1x1);hide(z_interlace_ct3_d4_1x1);hide(raw_interlace_ct3_d4_1x1);hide(r::data);hide(crate::witness_m6_m4_a7::expected_interlace_ct3_d4_1x1);
    hide(crate::critical_model::read);hide(crate::model::mh);
    assert(bytes_interlace_ct3_d4_1x1().len()==127nat) by (compute);
    assert(crate::model::mh(bytes_interlace_ct3_d4_1x1())==Ok((crate::model::Ihdr {width:1u32,height:1u32,depth:4u8,color_type:3u8,interlace:1u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_interlace_ct3_d4_1x1())==Ok((crate::model::Ihdr {width:1u32,height:1u32,depth:4u8,color_type:3u8,interlace:1u8},33nat))) by (compute);
    }
        assert((1u32 << 4u8)==16u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_1x1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_1x1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(48nat,3u8,4u8,false,false,false,false)==crate::metadata_model::Decision::Keep(16nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_1x1(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:10nat,next:115nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_1x1(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:10nat,next:115nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_interlace_ct3_d4_1x1(),115nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:123nat,len:0nat,next:127nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_interlace_ct3_d4_1x1(),115nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:123nat,len:0nat,next:127nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_interlace_ct3_d4_1x1().subrange(101,111)==z_interlace_ct3_d4_1x1()) by (compute);
    walk_interlace_ct3_d4_1x1(bytes_interlace_ct3_d4_1x1());

    model_interlace_ct3_d4_1x1();inflate_interlace_ct3_d4_1x1();scan_interlace_ct3_d4_1x1();
    assert(bytes_interlace_ct3_d4_1x1()==crate::witness_m6_m4_a7::bytes_interlace_ct3_d4_1x1()) by (compute);
    assert(r::raw(a_interlace_ct3_d4_1x1())==2nat) by (compute);
    assert(r::data(z_interlace_ct3_d4_1x1(),a_interlace_ct3_d4_1x1(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    assert(bytes_interlace_ct3_d4_1x1().subrange(41,89)==seq![81u8,144u8,125u8,101u8,64u8,177u8,228u8,169u8,188u8,76u8,40u8,153u8,237u8,218u8,35u8,151u8,40u8,162u8,34u8,24u8,141u8,23u8,54u8,198u8,120u8,240u8,209u8,125u8,189u8,50u8,29u8,165u8,196u8,115u8,208u8,128u8,195u8,46u8,41u8,101u8,192u8,41u8,152u8,245u8,128u8,108u8,162u8,192u8]) by (compute);
    crate::witness_m6_a7_model::palette_colour(bytes_interlace_ct3_d4_1x1(),h_interlace_ct3_d4_1x1(),10nat);
    e6_a7_pass_interlace_ct3_d4_1x1_0();
    assert(r::count(a_interlace_ct3_d4_1x1())==7nat && crate::paint_model::domain(a_interlace_ct3_d4_1x1()) && crate::witness_m6_m4_a7::expected_interlace_ct3_d4_1x1().len()==4nat) by (compute);
    assert(r::pw(a_interlace_ct3_d4_1x1(),0nat)==1nat && r::ph(a_interlace_ct3_d4_1x1(),0nat)==1nat) by (compute);
    assert(r::pw(a_interlace_ct3_d4_1x1(),1nat)==0nat && r::ph(a_interlace_ct3_d4_1x1(),1nat)==1nat) by (compute);
    assert(r::pw(a_interlace_ct3_d4_1x1(),2nat)==1nat && r::ph(a_interlace_ct3_d4_1x1(),2nat)==0nat) by (compute);
    assert(r::pw(a_interlace_ct3_d4_1x1(),3nat)==0nat && r::ph(a_interlace_ct3_d4_1x1(),3nat)==1nat) by (compute);
    assert(r::pw(a_interlace_ct3_d4_1x1(),4nat)==1nat && r::ph(a_interlace_ct3_d4_1x1(),4nat)==0nat) by (compute);
    assert(r::pw(a_interlace_ct3_d4_1x1(),5nat)==0nat && r::ph(a_interlace_ct3_d4_1x1(),5nat)==1nat) by (compute);
    assert(r::pw(a_interlace_ct3_d4_1x1(),6nat)==1nat && r::ph(a_interlace_ct3_d4_1x1(),6nat)==0nat) by (compute);
    assert forall|k: nat,y: nat| k<7 && r::pw(a_interlace_ct3_d4_1x1(),k)>0 && y<r::ph(a_interlace_ct3_d4_1x1(),k) implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_a7_colour_interlace_ct3_d4_1x1(),r::line(raw_interlace_ct3_d4_1x1(),a_interlace_ct3_d4_1x1(),k,y),r::pw(a_interlace_ct3_d4_1x1(),k))==crate::witness_m6_a7_model::expected_pass_row(a_interlace_ct3_d4_1x1(),crate::witness_m6_m4_a7::expected_interlace_ct3_d4_1x1(),k,y) by {
        if k==0 {} else if k==1 {} else if k==2 {} else if k==3 {} else if k==4 {} else if k==5 {} else {assert(k==6);}
    }
    crate::witness_m6_a7_model::complete_still(bytes_interlace_ct3_d4_1x1(),h_interlace_ct3_d4_1x1(),z_interlace_ct3_d4_1x1(),raw_interlace_ct3_d4_1x1(),16nat,e6_a7_colour_interlace_ct3_d4_1x1(),crate::witness_m6_m4_a7::expected_interlace_ct3_d4_1x1());
}
} // verus!
