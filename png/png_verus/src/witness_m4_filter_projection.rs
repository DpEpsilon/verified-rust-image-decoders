//! M4 filter format/shape projections; complete pixel witnesses remain due at M6.
use vstd::prelude::*;
use crate::{raster_model as r, png::Error, policy::STRICT, m4_trace_witness_model as t,
    m4_checksum_witness_model as c, m4_stream_witness_model as stream, m4_crc_witness_model as crc_cert};
use inflate_verus::model as zm;
verus! {
// filter/0-ct0-d1; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_filter_0_ct0_d1() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,1u8,0u8,0u8,0u8,0u8,133u8,127u8,115u8,238u8,0u8,0u8,0u8,16u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,224u8,97u8,112u8,96u8,152u8,193u8,48u8,7u8,0u8,4u8,0u8,1u8,129u8,85u8,162u8,196u8,190u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_filter_0_ct0_d1() -> Seq<u8> {seq![120u8,156u8,99u8,224u8,97u8,112u8,96u8,152u8,193u8,48u8,7u8,0u8,4u8,0u8,1u8,129u8]}
spec fn h_filter_0_ct0_d1() -> crate::model::Ihdr {crate::model::Ihdr {width:6u32,height:4u32,depth:1u8,color_type:0u8,interlace:0u8}}
spec fn a_filter_0_ct0_d1() -> r::Raster {r::shape(h_filter_0_ct0_d1(),6nat,4nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_filter_0_ct0_d1(b: Seq<u8>)
    requires b.len()==73,crate::model::mh(b)==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:1u8,color_type:0u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:16nat,next:61nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,61nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:69nat,len:0nat,next:73nat,crc_ok:true,synthetic:false})),
        b.subrange(41,57)==z_filter_0_ct0_d1(),
    ensures r::default_case(b,crate::model::Ihdr {width:6u32,height:4u32,depth:1u8,color_type:0u8,interlace:0u8},z_filter_0_ct0_d1(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
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
spec fn raw_filter_0_ct0_d1() -> Seq<u8> {seq![0u8,12u8,0u8,64u8,0u8,152u8,0u8,156u8]}
spec fn ts_filter_0_ct0_d1() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:35nat,before:1nat,after:2nat,symbol:12nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:35nat,next:43nat,before:2nat,after:3nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:43nat,next:51nat,before:3nat,after:4nat,symbol:64nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:51nat,next:59nat,before:4nat,after:5nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:59nat,next:68nat,before:5nat,after:6nat,symbol:152nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:68nat,next:76nat,before:6nat,after:7nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:76nat,next:85nat,before:7nat,after:8nat,symbol:156nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:85nat,next:92nat,before:8nat,after:8nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_filter_0_ct0_d1()
    ensures t::trace(z_filter_0_ct0_d1(),raw_filter_0_ct0_d1(),ts_filter_0_ct0_d1(),0,9nat),
{
    hide(z_filter_0_ct0_d1);hide(raw_filter_0_ct0_d1);hide(ts_filter_0_ct0_d1);hide(t::valid_step);
    assert(t::trace(z_filter_0_ct0_d1(),raw_filter_0_ct0_d1(),ts_filter_0_ct0_d1(),0,9nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_filter_0_ct0_d1()
    ensures raw_filter_0_ct0_d1().len()==8nat,
        z_filter_0_ct0_d1().len()==16nat,
        z_filter_0_ct0_d1()[0]==120u8,
        z_filter_0_ct0_d1()[1]==156u8,
        zm::field_value(z_filter_0_ct0_d1(),16,1)==1nat,
        zm::field_value(z_filter_0_ct0_d1(),17,2)==1nat,
        zm::be_value(z_filter_0_ct0_d1(),12nat,4)==67109249nat,
        ts_filter_0_ct0_d1().len()==9nat,
        ts_filter_0_ct0_d1()[0].pos==19nat,
        ts_filter_0_ct0_d1()[0].before==0nat,
        ts_filter_0_ct0_d1().last().next==92nat,
{
    hide(z_filter_0_ct0_d1);hide(raw_filter_0_ct0_d1);hide(ts_filter_0_ct0_d1);
    assert(raw_filter_0_ct0_d1().len()==8nat) by (compute);
    assert(z_filter_0_ct0_d1().len()==16nat) by (compute);
    assert(z_filter_0_ct0_d1()[0]==120u8) by (compute);
    assert(z_filter_0_ct0_d1()[1]==156u8) by (compute);
    assert(zm::field_value(z_filter_0_ct0_d1(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_filter_0_ct0_d1(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_filter_0_ct0_d1(),12nat,4)==67109249nat) by (compute);
    assert(ts_filter_0_ct0_d1().len()==9nat) by (compute);
    assert(ts_filter_0_ct0_d1()[0].pos==19nat) by (compute);
    assert(ts_filter_0_ct0_d1()[0].before==0nat) by (compute);
    assert(ts_filter_0_ct0_d1().last().next==92nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_filter_0_ct0_d1()
    ensures zm::Adler(raw_filter_0_ct0_d1())==67109249nat,
{
    hide(raw_filter_0_ct0_d1);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_filter_0_ct0_d1().len()==8nat) by (compute);
    assert(c::sums(raw_filter_0_ct0_d1(),0,8nat)==(384nat,1024nat)) by (compute);
    c::sums_correct(raw_filter_0_ct0_d1(),0,8nat);
    assert(raw_filter_0_ct0_d1().subrange(0,8) =~= raw_filter_0_ct0_d1());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((1024nat%65521)*(u16::MAX as nat+1)+(1+384nat)%65521==67109249nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_filter_0_ct0_d1()
    ensures r::inflated(z_filter_0_ct0_d1(),a_filter_0_ct0_d1(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_filter_0_ct0_d1(),r2:false}),
        raw_filter_0_ct0_d1().len()==8nat,
{
    hide(z_filter_0_ct0_d1);hide(raw_filter_0_ct0_d1);hide(ts_filter_0_ct0_d1);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_filter_0_ct0_d1();checksum_filter_0_ct0_d1();
    assert(r::raw(a_filter_0_ct0_d1())==8nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_filter_0_ct0_d1();
    stream::fixed_stream(z_filter_0_ct0_d1(),raw_filter_0_ct0_d1(),ts_filter_0_ct0_d1(),8nat,156nat,67109249nat);
}
#[verifier::spinoff_prover]
proof fn scan_filter_0_ct0_d1()
    ensures r::scan(raw_filter_0_ct0_d1(),a_filter_0_ct0_d1(),0)==Ok(()),
{
    hide(raw_filter_0_ct0_d1);
    assert(r::base(a_filter_0_ct0_d1(),0)==0nat && r::ph(a_filter_0_ct0_d1(),0)==4nat && r::count(a_filter_0_ct0_d1())==1nat) by (compute);
    assert(r::rb(a_filter_0_ct0_d1(),0)==1nat) by (compute);
    assert(raw_filter_0_ct0_d1()[0]==0u8) by (compute);
    assert(r::row_result(raw_filter_0_ct0_d1(),a_filter_0_ct0_d1(),0,0nat)==Ok(()));
    assert(raw_filter_0_ct0_d1()[2]==0u8) by (compute);
    assert(r::row_result(raw_filter_0_ct0_d1(),a_filter_0_ct0_d1(),0,1nat)==Ok(()));
    assert(raw_filter_0_ct0_d1()[4]==0u8) by (compute);
    assert(r::row_result(raw_filter_0_ct0_d1(),a_filter_0_ct0_d1(),0,2nat)==Ok(()));
    assert(raw_filter_0_ct0_d1()[6]==0u8) by (compute);
    assert(r::row_result(raw_filter_0_ct0_d1(),a_filter_0_ct0_d1(),0,3nat)==Ok(()));
    r::pass_step(raw_filter_0_ct0_d1(),a_filter_0_ct0_d1(),0,4nat);
    r::pass_step(raw_filter_0_ct0_d1(),a_filter_0_ct0_d1(),0,3nat);
    r::pass_step(raw_filter_0_ct0_d1(),a_filter_0_ct0_d1(),0,2nat);
    r::pass_step(raw_filter_0_ct0_d1(),a_filter_0_ct0_d1(),0,1nat);
    r::pass_step(raw_filter_0_ct0_d1(),a_filter_0_ct0_d1(),0,0nat);
    r::scan_step(raw_filter_0_ct0_d1(),a_filter_0_ct0_d1(),1);
    r::scan_step(raw_filter_0_ct0_d1(),a_filter_0_ct0_d1(),0);
}
#[verifier::spinoff_prover]
proof fn model_filter_0_ct0_d1()
    ensures r::still(bytes_filter_0_ct0_d1(),STRICT,false)==(r::Projection::Ready {width:6u32,height:4u32,bytes:96nat}),
{
    hide(bytes_filter_0_ct0_d1);hide(z_filter_0_ct0_d1);hide(raw_filter_0_ct0_d1);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_0_ct0_d1().len()==73nat) by (compute);
    assert(crate::model::mh(bytes_filter_0_ct0_d1())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:1u8,color_type:0u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_0_ct0_d1())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:1u8,color_type:0u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 1u8)==2u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_0_ct0_d1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:16nat,next:61nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_0_ct0_d1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:16nat,next:61nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_0_ct0_d1(),61nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:69nat,len:0nat,next:73nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_0_ct0_d1(),61nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:69nat,len:0nat,next:73nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_0_ct0_d1().subrange(41,57)==z_filter_0_ct0_d1()) by (compute);
    walk_filter_0_ct0_d1(bytes_filter_0_ct0_d1());
    inflate_filter_0_ct0_d1();scan_filter_0_ct0_d1();
    assert(r::raw(a_filter_0_ct0_d1())==8nat) by (compute);
    assert(r::data(z_filter_0_ct0_d1(),a_filter_0_ct0_d1(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_filter_0_ct0_d1(),h_filter_0_ct0_d1(),z_filter_0_ct0_d1(),0nat,false);
}
fn public_filter_0_ct0_d1() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,1u8,0u8,0u8,0u8,0u8,133u8,127u8,115u8,238u8,0u8,0u8,0u8,16u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,224u8,97u8,112u8,96u8,152u8,193u8,48u8,7u8,0u8,4u8,0u8,1u8,129u8,85u8,162u8,196u8,190u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_0_ct0_d1());model_filter_0_ct0_d1();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==6u32 && s.height==4u32 && s.rgba@.len()==96nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// filter/0-ct0-d8; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_filter_0_ct0_d8() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,0u8,0u8,0u8,0u8,136u8,111u8,17u8,159u8,0u8,0u8,0u8,37u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,159u8,200u8,190u8,165u8,202u8,144u8,65u8,238u8,230u8,219u8,223u8,245u8,121u8,12u8,118u8,255u8,197u8,121u8,111u8,177u8,50u8,148u8,92u8,250u8,93u8,190u8,220u8,25u8,0u8,173u8,223u8,12u8,117u8,24u8,26u8,151u8,33u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_filter_0_ct0_d8() -> Seq<u8> {seq![120u8,156u8,99u8,248u8,159u8,200u8,190u8,165u8,202u8,144u8,65u8,238u8,230u8,219u8,223u8,245u8,121u8,12u8,118u8,255u8,197u8,121u8,111u8,177u8,50u8,148u8,92u8,250u8,93u8,190u8,220u8,25u8,0u8,173u8,223u8,12u8,117u8]}
spec fn h_filter_0_ct0_d8() -> crate::model::Ihdr {crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:0u8,interlace:0u8}}
spec fn a_filter_0_ct0_d8() -> r::Raster {r::shape(h_filter_0_ct0_d8(),6nat,4nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_filter_0_ct0_d8(b: Seq<u8>)
    requires b.len()==94,crate::model::mh(b)==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:0u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:37nat,next:82nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,82nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:90nat,len:0nat,next:94nat,crc_ok:true,synthetic:false})),
        b.subrange(41,78)==z_filter_0_ct0_d8(),
    ensures r::default_case(b,crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:0u8,interlace:0u8},z_filter_0_ct0_d8(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_filter_0_ct0_d8() -> Seq<u8> {seq![0u8,255u8,97u8,7u8,180u8,122u8,49u8,0u8,30u8,217u8,237u8,251u8,127u8,110u8,0u8,62u8,255u8,23u8,13u8,218u8,5u8,0u8,116u8,210u8,251u8,119u8,167u8,67u8]}
spec fn ts_filter_0_ct0_d8() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:36nat,before:1nat,after:2nat,symbol:255nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:36nat,next:44nat,before:2nat,after:3nat,symbol:97nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:44nat,next:52nat,before:3nat,after:4nat,symbol:7nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:52nat,next:61nat,before:4nat,after:5nat,symbol:180nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:61nat,next:69nat,before:5nat,after:6nat,symbol:122nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:69nat,next:77nat,before:6nat,after:7nat,symbol:49nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:77nat,next:85nat,before:7nat,after:8nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:85nat,next:93nat,before:8nat,after:9nat,symbol:30nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:93nat,next:102nat,before:9nat,after:10nat,symbol:217nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:102nat,next:111nat,before:10nat,after:11nat,symbol:237nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:111nat,next:120nat,before:11nat,after:12nat,symbol:251nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:120nat,next:128nat,before:12nat,after:13nat,symbol:127nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:128nat,next:136nat,before:13nat,after:14nat,symbol:110nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:136nat,next:144nat,before:14nat,after:15nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:144nat,next:152nat,before:15nat,after:16nat,symbol:62nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:152nat,next:161nat,before:16nat,after:17nat,symbol:255nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:161nat,next:169nat,before:17nat,after:18nat,symbol:23nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:169nat,next:177nat,before:18nat,after:19nat,symbol:13nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:177nat,next:186nat,before:19nat,after:20nat,symbol:218nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:186nat,next:194nat,before:20nat,after:21nat,symbol:5nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:194nat,next:202nat,before:21nat,after:22nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:202nat,next:210nat,before:22nat,after:23nat,symbol:116nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:210nat,next:219nat,before:23nat,after:24nat,symbol:210nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:219nat,next:228nat,before:24nat,after:25nat,symbol:251nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:228nat,next:236nat,before:25nat,after:26nat,symbol:119nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:236nat,next:245nat,before:26nat,after:27nat,symbol:167nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:245nat,next:253nat,before:27nat,after:28nat,symbol:67nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:253nat,next:260nat,before:28nat,after:28nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_filter_0_ct0_d8()
    ensures t::trace(z_filter_0_ct0_d8(),raw_filter_0_ct0_d8(),ts_filter_0_ct0_d8(),0,29nat),
{
    hide(z_filter_0_ct0_d8);hide(raw_filter_0_ct0_d8);hide(ts_filter_0_ct0_d8);hide(t::valid_step);
    assert(t::trace(z_filter_0_ct0_d8(),raw_filter_0_ct0_d8(),ts_filter_0_ct0_d8(),0,29nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_filter_0_ct0_d8()
    ensures raw_filter_0_ct0_d8().len()==28nat,
        z_filter_0_ct0_d8().len()==37nat,
        z_filter_0_ct0_d8()[0]==120u8,
        z_filter_0_ct0_d8()[1]==156u8,
        zm::field_value(z_filter_0_ct0_d8(),16,1)==1nat,
        zm::field_value(z_filter_0_ct0_d8(),17,2)==1nat,
        zm::be_value(z_filter_0_ct0_d8(),33nat,4)==2917076085nat,
        ts_filter_0_ct0_d8().len()==29nat,
        ts_filter_0_ct0_d8()[0].pos==19nat,
        ts_filter_0_ct0_d8()[0].before==0nat,
        ts_filter_0_ct0_d8().last().next==260nat,
{
    hide(z_filter_0_ct0_d8);hide(raw_filter_0_ct0_d8);hide(ts_filter_0_ct0_d8);
    assert(raw_filter_0_ct0_d8().len()==28nat) by (compute);
    assert(z_filter_0_ct0_d8().len()==37nat) by (compute);
    assert(z_filter_0_ct0_d8()[0]==120u8) by (compute);
    assert(z_filter_0_ct0_d8()[1]==156u8) by (compute);
    assert(zm::field_value(z_filter_0_ct0_d8(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_filter_0_ct0_d8(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_filter_0_ct0_d8(),33nat,4)==2917076085nat) by (compute);
    assert(ts_filter_0_ct0_d8().len()==29nat) by (compute);
    assert(ts_filter_0_ct0_d8()[0].pos==19nat) by (compute);
    assert(ts_filter_0_ct0_d8()[0].before==0nat) by (compute);
    assert(ts_filter_0_ct0_d8().last().next==260nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_filter_0_ct0_d8()
    ensures zm::Adler(raw_filter_0_ct0_d8())==2917076085nat,
{
    hide(raw_filter_0_ct0_d8);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_filter_0_ct0_d8().len()==28nat) by (compute);
    assert(c::sums(raw_filter_0_ct0_d8(),0,28nat)==(3188nat,44511nat)) by (compute);
    c::sums_correct(raw_filter_0_ct0_d8(),0,28nat);
    assert(raw_filter_0_ct0_d8().subrange(0,28) =~= raw_filter_0_ct0_d8());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((44511nat%65521)*(u16::MAX as nat+1)+(1+3188nat)%65521==2917076085nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_filter_0_ct0_d8()
    ensures r::inflated(z_filter_0_ct0_d8(),a_filter_0_ct0_d8(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_filter_0_ct0_d8(),r2:false}),
        raw_filter_0_ct0_d8().len()==28nat,
{
    hide(z_filter_0_ct0_d8);hide(raw_filter_0_ct0_d8);hide(ts_filter_0_ct0_d8);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_filter_0_ct0_d8();checksum_filter_0_ct0_d8();
    assert(r::raw(a_filter_0_ct0_d8())==28nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_filter_0_ct0_d8();
    stream::fixed_stream(z_filter_0_ct0_d8(),raw_filter_0_ct0_d8(),ts_filter_0_ct0_d8(),28nat,156nat,2917076085nat);
}
#[verifier::spinoff_prover]
proof fn scan_filter_0_ct0_d8()
    ensures r::scan(raw_filter_0_ct0_d8(),a_filter_0_ct0_d8(),0)==Ok(()),
{
    hide(raw_filter_0_ct0_d8);
    assert(r::base(a_filter_0_ct0_d8(),0)==0nat && r::ph(a_filter_0_ct0_d8(),0)==4nat && r::count(a_filter_0_ct0_d8())==1nat) by (compute);
    assert(r::rb(a_filter_0_ct0_d8(),0)==6nat) by (compute);
    assert(raw_filter_0_ct0_d8()[0]==0u8) by (compute);
    assert(r::row_result(raw_filter_0_ct0_d8(),a_filter_0_ct0_d8(),0,0nat)==Ok(()));
    assert(raw_filter_0_ct0_d8()[7]==0u8) by (compute);
    assert(r::row_result(raw_filter_0_ct0_d8(),a_filter_0_ct0_d8(),0,1nat)==Ok(()));
    assert(raw_filter_0_ct0_d8()[14]==0u8) by (compute);
    assert(r::row_result(raw_filter_0_ct0_d8(),a_filter_0_ct0_d8(),0,2nat)==Ok(()));
    assert(raw_filter_0_ct0_d8()[21]==0u8) by (compute);
    assert(r::row_result(raw_filter_0_ct0_d8(),a_filter_0_ct0_d8(),0,3nat)==Ok(()));
    r::pass_step(raw_filter_0_ct0_d8(),a_filter_0_ct0_d8(),0,4nat);
    r::pass_step(raw_filter_0_ct0_d8(),a_filter_0_ct0_d8(),0,3nat);
    r::pass_step(raw_filter_0_ct0_d8(),a_filter_0_ct0_d8(),0,2nat);
    r::pass_step(raw_filter_0_ct0_d8(),a_filter_0_ct0_d8(),0,1nat);
    r::pass_step(raw_filter_0_ct0_d8(),a_filter_0_ct0_d8(),0,0nat);
    r::scan_step(raw_filter_0_ct0_d8(),a_filter_0_ct0_d8(),1);
    r::scan_step(raw_filter_0_ct0_d8(),a_filter_0_ct0_d8(),0);
}
#[verifier::spinoff_prover]
proof fn model_filter_0_ct0_d8()
    ensures r::still(bytes_filter_0_ct0_d8(),STRICT,false)==(r::Projection::Ready {width:6u32,height:4u32,bytes:96nat}),
{
    hide(bytes_filter_0_ct0_d8);hide(z_filter_0_ct0_d8);hide(raw_filter_0_ct0_d8);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_0_ct0_d8().len()==94nat) by (compute);
    assert(crate::model::mh(bytes_filter_0_ct0_d8())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:0u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_0_ct0_d8())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:0u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_0_ct0_d8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:37nat,next:82nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_0_ct0_d8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:37nat,next:82nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_0_ct0_d8(),82nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:90nat,len:0nat,next:94nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_0_ct0_d8(),82nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:90nat,len:0nat,next:94nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_0_ct0_d8().subrange(41,78)==z_filter_0_ct0_d8()) by (compute);
    walk_filter_0_ct0_d8(bytes_filter_0_ct0_d8());
    inflate_filter_0_ct0_d8();scan_filter_0_ct0_d8();
    assert(r::raw(a_filter_0_ct0_d8())==28nat) by (compute);
    assert(r::data(z_filter_0_ct0_d8(),a_filter_0_ct0_d8(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_filter_0_ct0_d8(),h_filter_0_ct0_d8(),z_filter_0_ct0_d8(),0nat,false);
}
fn public_filter_0_ct0_d8() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,0u8,0u8,0u8,0u8,136u8,111u8,17u8,159u8,0u8,0u8,0u8,37u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,159u8,200u8,190u8,165u8,202u8,144u8,65u8,238u8,230u8,219u8,223u8,245u8,121u8,12u8,118u8,255u8,197u8,121u8,111u8,177u8,50u8,148u8,92u8,250u8,93u8,190u8,220u8,25u8,0u8,173u8,223u8,12u8,117u8,24u8,26u8,151u8,33u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_0_ct0_d8());model_filter_0_ct0_d8();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==6u32 && s.height==4u32 && s.rgba@.len()==96nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// filter/0-ct0-d16; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_filter_0_ct0_d16() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,0u8,0u8,0u8,0u8,216u8,255u8,205u8,220u8,0u8,0u8,0u8,61u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,241u8,238u8,82u8,203u8,203u8,144u8,222u8,16u8,103u8,35u8,139u8,70u8,6u8,11u8,159u8,93u8,188u8,201u8,137u8,13u8,95u8,52u8,31u8,133u8,165u8,48u8,176u8,114u8,112u8,104u8,92u8,99u8,75u8,14u8,204u8,83u8,158u8,153u8,201u8,112u8,79u8,223u8,76u8,116u8,219u8,205u8,182u8,252u8,70u8,137u8,166u8,60u8,0u8,60u8,58u8,20u8,150u8,248u8,127u8,179u8,191u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_filter_0_ct0_d16() -> Seq<u8> {seq![120u8,156u8,99u8,248u8,241u8,238u8,82u8,203u8,203u8,144u8,222u8,16u8,103u8,35u8,139u8,70u8,6u8,11u8,159u8,93u8,188u8,201u8,137u8,13u8,95u8,52u8,31u8,133u8,165u8,48u8,176u8,114u8,112u8,104u8,92u8,99u8,75u8,14u8,204u8,83u8,158u8,153u8,201u8,112u8,79u8,223u8,76u8,116u8,219u8,205u8,182u8,252u8,70u8,137u8,166u8,60u8,0u8,60u8,58u8,20u8,150u8]}
spec fn h_filter_0_ct0_d16() -> crate::model::Ihdr {crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:0u8,interlace:0u8}}
spec fn a_filter_0_ct0_d16() -> r::Raster {r::shape(h_filter_0_ct0_d16(),6nat,4nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_filter_0_ct0_d16(b: Seq<u8>)
    requires b.len()==118,crate::model::mh(b)==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:0u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:61nat,next:106nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,106nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:114nat,len:0nat,next:118nat,crc_ok:true,synthetic:false})),
        b.subrange(41,102)==z_filter_0_ct0_d16(),
    ensures r::default_case(b,crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:0u8,interlace:0u8},z_filter_0_ct0_d16(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
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
spec fn raw_filter_0_ct0_d16() -> Seq<u8> {seq![0u8,248u8,238u8,210u8,132u8,233u8,84u8,141u8,84u8,67u8,50u8,56u8,129u8,0u8,56u8,76u8,186u8,13u8,99u8,97u8,128u8,244u8,41u8,226u8,86u8,100u8,0u8,5u8,8u8,8u8,40u8,214u8,6u8,99u8,81u8,110u8,35u8,153u8,105u8,0u8,222u8,47u8,54u8,21u8,182u8,217u8,134u8,111u8,129u8,24u8,130u8,110u8]}
spec fn ts_filter_0_ct0_d16() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:36nat,before:1nat,after:2nat,symbol:248nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:36nat,next:45nat,before:2nat,after:3nat,symbol:238nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:45nat,next:54nat,before:3nat,after:4nat,symbol:210nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:54nat,next:62nat,before:4nat,after:5nat,symbol:132nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:62nat,next:71nat,before:5nat,after:6nat,symbol:233nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:71nat,next:79nat,before:6nat,after:7nat,symbol:84nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:79nat,next:87nat,before:7nat,after:8nat,symbol:141nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:87nat,next:95nat,before:8nat,after:9nat,symbol:84nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:95nat,next:103nat,before:9nat,after:10nat,symbol:67nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:103nat,next:111nat,before:10nat,after:11nat,symbol:50nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:111nat,next:119nat,before:11nat,after:12nat,symbol:56nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:119nat,next:127nat,before:12nat,after:13nat,symbol:129nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:127nat,next:135nat,before:13nat,after:14nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:135nat,next:143nat,before:14nat,after:15nat,symbol:56nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:143nat,next:151nat,before:15nat,after:16nat,symbol:76nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:151nat,next:160nat,before:16nat,after:17nat,symbol:186nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:160nat,next:168nat,before:17nat,after:18nat,symbol:13nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:168nat,next:176nat,before:18nat,after:19nat,symbol:99nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:176nat,next:184nat,before:19nat,after:20nat,symbol:97nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:184nat,next:192nat,before:20nat,after:21nat,symbol:128nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:192nat,next:201nat,before:21nat,after:22nat,symbol:244nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:201nat,next:209nat,before:22nat,after:23nat,symbol:41nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:209nat,next:218nat,before:23nat,after:24nat,symbol:226nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:218nat,next:226nat,before:24nat,after:25nat,symbol:86nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:226nat,next:234nat,before:25nat,after:26nat,symbol:100nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:234nat,next:242nat,before:26nat,after:27nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:242nat,next:250nat,before:27nat,after:28nat,symbol:5nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:250nat,next:258nat,before:28nat,after:29nat,symbol:8nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:258nat,next:266nat,before:29nat,after:30nat,symbol:8nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:266nat,next:274nat,before:30nat,after:31nat,symbol:40nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:274nat,next:283nat,before:31nat,after:32nat,symbol:214nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:283nat,next:291nat,before:32nat,after:33nat,symbol:6nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:291nat,next:299nat,before:33nat,after:34nat,symbol:99nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:299nat,next:307nat,before:34nat,after:35nat,symbol:81nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:307nat,next:315nat,before:35nat,after:36nat,symbol:110nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:315nat,next:323nat,before:36nat,after:37nat,symbol:35nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:323nat,next:332nat,before:37nat,after:38nat,symbol:153nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:332nat,next:340nat,before:38nat,after:39nat,symbol:105nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:340nat,next:348nat,before:39nat,after:40nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:348nat,next:357nat,before:40nat,after:41nat,symbol:222nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:357nat,next:365nat,before:41nat,after:42nat,symbol:47nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:365nat,next:373nat,before:42nat,after:43nat,symbol:54nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:373nat,next:381nat,before:43nat,after:44nat,symbol:21nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:381nat,next:390nat,before:44nat,after:45nat,symbol:182nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:390nat,next:399nat,before:45nat,after:46nat,symbol:217nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:399nat,next:407nat,before:46nat,after:47nat,symbol:134nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:407nat,next:415nat,before:47nat,after:48nat,symbol:111nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:415nat,next:423nat,before:48nat,after:49nat,symbol:129nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:423nat,next:431nat,before:49nat,after:50nat,symbol:24nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:431nat,next:439nat,before:50nat,after:51nat,symbol:130nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:439nat,next:447nat,before:51nat,after:52nat,symbol:110nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:447nat,next:454nat,before:52nat,after:52nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_filter_0_ct0_d16()
    ensures t::trace(z_filter_0_ct0_d16(),raw_filter_0_ct0_d16(),ts_filter_0_ct0_d16(),0,53nat),
{
    hide(z_filter_0_ct0_d16);hide(raw_filter_0_ct0_d16);hide(ts_filter_0_ct0_d16);hide(t::valid_step);
    assert(t::trace(z_filter_0_ct0_d16(),raw_filter_0_ct0_d16(),ts_filter_0_ct0_d16(),0,53nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_filter_0_ct0_d16()
    ensures raw_filter_0_ct0_d16().len()==52nat,
        z_filter_0_ct0_d16().len()==61nat,
        z_filter_0_ct0_d16()[0]==120u8,
        z_filter_0_ct0_d16()[1]==156u8,
        zm::field_value(z_filter_0_ct0_d16(),16,1)==1nat,
        zm::field_value(z_filter_0_ct0_d16(),17,2)==1nat,
        zm::be_value(z_filter_0_ct0_d16(),57nat,4)==1010439318nat,
        ts_filter_0_ct0_d16().len()==53nat,
        ts_filter_0_ct0_d16()[0].pos==19nat,
        ts_filter_0_ct0_d16()[0].before==0nat,
        ts_filter_0_ct0_d16().last().next==454nat,
{
    hide(z_filter_0_ct0_d16);hide(raw_filter_0_ct0_d16);hide(ts_filter_0_ct0_d16);
    assert(raw_filter_0_ct0_d16().len()==52nat) by (compute);
    assert(z_filter_0_ct0_d16().len()==61nat) by (compute);
    assert(z_filter_0_ct0_d16()[0]==120u8) by (compute);
    assert(z_filter_0_ct0_d16()[1]==156u8) by (compute);
    assert(zm::field_value(z_filter_0_ct0_d16(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_filter_0_ct0_d16(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_filter_0_ct0_d16(),57nat,4)==1010439318nat) by (compute);
    assert(ts_filter_0_ct0_d16().len()==53nat) by (compute);
    assert(ts_filter_0_ct0_d16()[0].pos==19nat) by (compute);
    assert(ts_filter_0_ct0_d16()[0].before==0nat) by (compute);
    assert(ts_filter_0_ct0_d16().last().next==454nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_filter_0_ct0_d16()
    ensures zm::Adler(raw_filter_0_ct0_d16())==1010439318nat,
{
    hide(raw_filter_0_ct0_d16);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_filter_0_ct0_d16().len()==52nat) by (compute);
    assert(c::sums(raw_filter_0_ct0_d16(),0,52nat)==(5269nat,146460nat)) by (compute);
    c::sums_correct(raw_filter_0_ct0_d16(),0,52nat);
    assert(raw_filter_0_ct0_d16().subrange(0,52) =~= raw_filter_0_ct0_d16());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((146460nat%65521)*(u16::MAX as nat+1)+(1+5269nat)%65521==1010439318nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_filter_0_ct0_d16()
    ensures r::inflated(z_filter_0_ct0_d16(),a_filter_0_ct0_d16(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_filter_0_ct0_d16(),r2:false}),
        raw_filter_0_ct0_d16().len()==52nat,
{
    hide(z_filter_0_ct0_d16);hide(raw_filter_0_ct0_d16);hide(ts_filter_0_ct0_d16);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_filter_0_ct0_d16();checksum_filter_0_ct0_d16();
    assert(r::raw(a_filter_0_ct0_d16())==52nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_filter_0_ct0_d16();
    stream::fixed_stream(z_filter_0_ct0_d16(),raw_filter_0_ct0_d16(),ts_filter_0_ct0_d16(),52nat,156nat,1010439318nat);
}
#[verifier::spinoff_prover]
proof fn scan_filter_0_ct0_d16()
    ensures r::scan(raw_filter_0_ct0_d16(),a_filter_0_ct0_d16(),0)==Ok(()),
{
    hide(raw_filter_0_ct0_d16);
    assert(r::base(a_filter_0_ct0_d16(),0)==0nat && r::ph(a_filter_0_ct0_d16(),0)==4nat && r::count(a_filter_0_ct0_d16())==1nat) by (compute);
    assert(r::rb(a_filter_0_ct0_d16(),0)==12nat) by (compute);
    assert(raw_filter_0_ct0_d16()[0]==0u8) by (compute);
    assert(r::row_result(raw_filter_0_ct0_d16(),a_filter_0_ct0_d16(),0,0nat)==Ok(()));
    assert(raw_filter_0_ct0_d16()[13]==0u8) by (compute);
    assert(r::row_result(raw_filter_0_ct0_d16(),a_filter_0_ct0_d16(),0,1nat)==Ok(()));
    assert(raw_filter_0_ct0_d16()[26]==0u8) by (compute);
    assert(r::row_result(raw_filter_0_ct0_d16(),a_filter_0_ct0_d16(),0,2nat)==Ok(()));
    assert(raw_filter_0_ct0_d16()[39]==0u8) by (compute);
    assert(r::row_result(raw_filter_0_ct0_d16(),a_filter_0_ct0_d16(),0,3nat)==Ok(()));
    r::pass_step(raw_filter_0_ct0_d16(),a_filter_0_ct0_d16(),0,4nat);
    r::pass_step(raw_filter_0_ct0_d16(),a_filter_0_ct0_d16(),0,3nat);
    r::pass_step(raw_filter_0_ct0_d16(),a_filter_0_ct0_d16(),0,2nat);
    r::pass_step(raw_filter_0_ct0_d16(),a_filter_0_ct0_d16(),0,1nat);
    r::pass_step(raw_filter_0_ct0_d16(),a_filter_0_ct0_d16(),0,0nat);
    r::scan_step(raw_filter_0_ct0_d16(),a_filter_0_ct0_d16(),1);
    r::scan_step(raw_filter_0_ct0_d16(),a_filter_0_ct0_d16(),0);
}
#[verifier::spinoff_prover]
proof fn model_filter_0_ct0_d16()
    ensures r::still(bytes_filter_0_ct0_d16(),STRICT,false)==(r::Projection::Ready {width:6u32,height:4u32,bytes:96nat}),
{
    hide(bytes_filter_0_ct0_d16);hide(z_filter_0_ct0_d16);hide(raw_filter_0_ct0_d16);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_0_ct0_d16().len()==118nat) by (compute);
    assert(crate::model::mh(bytes_filter_0_ct0_d16())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:0u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_0_ct0_d16())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:0u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_0_ct0_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:61nat,next:106nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_0_ct0_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:61nat,next:106nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_0_ct0_d16(),106nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:114nat,len:0nat,next:118nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_0_ct0_d16(),106nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:114nat,len:0nat,next:118nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_0_ct0_d16().subrange(41,102)==z_filter_0_ct0_d16()) by (compute);
    walk_filter_0_ct0_d16(bytes_filter_0_ct0_d16());
    inflate_filter_0_ct0_d16();scan_filter_0_ct0_d16();
    assert(r::raw(a_filter_0_ct0_d16())==52nat) by (compute);
    assert(r::data(z_filter_0_ct0_d16(),a_filter_0_ct0_d16(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_filter_0_ct0_d16(),h_filter_0_ct0_d16(),z_filter_0_ct0_d16(),0nat,false);
}
fn public_filter_0_ct0_d16() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,0u8,0u8,0u8,0u8,216u8,255u8,205u8,220u8,0u8,0u8,0u8,61u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,241u8,238u8,82u8,203u8,203u8,144u8,222u8,16u8,103u8,35u8,139u8,70u8,6u8,11u8,159u8,93u8,188u8,201u8,137u8,13u8,95u8,52u8,31u8,133u8,165u8,48u8,176u8,114u8,112u8,104u8,92u8,99u8,75u8,14u8,204u8,83u8,158u8,153u8,201u8,112u8,79u8,223u8,76u8,116u8,219u8,205u8,182u8,252u8,70u8,137u8,166u8,60u8,0u8,60u8,58u8,20u8,150u8,248u8,127u8,179u8,191u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_0_ct0_d16());model_filter_0_ct0_d16();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==6u32 && s.height==4u32 && s.rgba@.len()==96nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// filter/0-ct2-d8; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_filter_0_ct2_d8() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,2u8,0u8,0u8,0u8,34u8,102u8,217u8,20u8,0u8,0u8,0u8,87u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,76u8,0u8,179u8,255u8,0u8,130u8,201u8,136u8,75u8,0u8,41u8,139u8,10u8,24u8,81u8,11u8,150u8,76u8,112u8,11u8,192u8,240u8,148u8,0u8,38u8,252u8,186u8,71u8,79u8,32u8,71u8,72u8,111u8,215u8,127u8,61u8,24u8,48u8,223u8,149u8,46u8,117u8,0u8,66u8,198u8,224u8,254u8,7u8,188u8,239u8,45u8,234u8,214u8,142u8,226u8,70u8,154u8,227u8,194u8,43u8,114u8,0u8,246u8,114u8,49u8,144u8,235u8,140u8,92u8,53u8,61u8,218u8,219u8,44u8,250u8,209u8,220u8,81u8,135u8,178u8,207u8,173u8,36u8,11u8,101u8,174u8,80u8,132u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_filter_0_ct2_d8() -> Seq<u8> {seq![120u8,156u8,1u8,76u8,0u8,179u8,255u8,0u8,130u8,201u8,136u8,75u8,0u8,41u8,139u8,10u8,24u8,81u8,11u8,150u8,76u8,112u8,11u8,192u8,240u8,148u8,0u8,38u8,252u8,186u8,71u8,79u8,32u8,71u8,72u8,111u8,215u8,127u8,61u8,24u8,48u8,223u8,149u8,46u8,117u8,0u8,66u8,198u8,224u8,254u8,7u8,188u8,239u8,45u8,234u8,214u8,142u8,226u8,70u8,154u8,227u8,194u8,43u8,114u8,0u8,246u8,114u8,49u8,144u8,235u8,140u8,92u8,53u8,61u8,218u8,219u8,44u8,250u8,209u8,220u8,81u8,135u8,178u8,207u8,173u8,36u8,11u8]}
spec fn h_filter_0_ct2_d8() -> crate::model::Ihdr {crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:2u8,interlace:0u8}}
spec fn a_filter_0_ct2_d8() -> r::Raster {r::shape(h_filter_0_ct2_d8(),6nat,4nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_filter_0_ct2_d8(b: Seq<u8>)
    requires b.len()==144,crate::model::mh(b)==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:2u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:87nat,next:132nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,132nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:140nat,len:0nat,next:144nat,crc_ok:true,synthetic:false})),
        b.subrange(41,128)==z_filter_0_ct2_d8(),
    ensures r::default_case(b,crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:2u8,interlace:0u8},z_filter_0_ct2_d8(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_filter_0_ct2_d8() -> Seq<u8> {seq![0u8,130u8,201u8,136u8,75u8,0u8,41u8,139u8,10u8,24u8,81u8,11u8,150u8,76u8,112u8,11u8,192u8,240u8,148u8,0u8,38u8,252u8,186u8,71u8,79u8,32u8,71u8,72u8,111u8,215u8,127u8,61u8,24u8,48u8,223u8,149u8,46u8,117u8,0u8,66u8,198u8,224u8,254u8,7u8,188u8,239u8,45u8,234u8,214u8,142u8,226u8,70u8,154u8,227u8,194u8,43u8,114u8,0u8,246u8,114u8,49u8,144u8,235u8,140u8,92u8,53u8,61u8,218u8,219u8,44u8,250u8,209u8,220u8,81u8,135u8,178u8]}
#[verifier::spinoff_prover]
proof fn shape_filter_0_ct2_d8()
    ensures raw_filter_0_ct2_d8().len()==76nat,
        z_filter_0_ct2_d8().len()==87nat,
        z_filter_0_ct2_d8()[0]==120u8,
        z_filter_0_ct2_d8()[1]==156u8,
        zm::field_value(z_filter_0_ct2_d8(),16,1)==1nat,
        zm::field_value(z_filter_0_ct2_d8(),17,2)==0nat,
        zm::be_value(z_filter_0_ct2_d8(),83nat,4)==3484230667nat,
        z_filter_0_ct2_d8().subrange(7,83)==raw_filter_0_ct2_d8(),
        z_filter_0_ct2_d8()[3] as nat+256*z_filter_0_ct2_d8()[4] as nat==76nat,
        z_filter_0_ct2_d8()[5] as nat+256*z_filter_0_ct2_d8()[6] as nat==u16::MAX as nat-76nat,
        zm::be_value(z_filter_0_ct2_d8(),0,1)==120nat,
        zm::be_value(z_filter_0_ct2_d8(),1,1)==156nat,
{
    hide(z_filter_0_ct2_d8);hide(raw_filter_0_ct2_d8);
    assert(raw_filter_0_ct2_d8().len()==76nat) by (compute);
    assert(z_filter_0_ct2_d8().len()==87nat) by (compute);
    assert(z_filter_0_ct2_d8()[0]==120u8) by (compute);
    assert(z_filter_0_ct2_d8()[1]==156u8) by (compute);
    assert(zm::field_value(z_filter_0_ct2_d8(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_filter_0_ct2_d8(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_filter_0_ct2_d8(),83nat,4)==3484230667nat) by (compute);
    assert(z_filter_0_ct2_d8().subrange(7,83)==raw_filter_0_ct2_d8()) by (compute);
    assert(z_filter_0_ct2_d8()[3] as nat+256*z_filter_0_ct2_d8()[4] as nat==76nat) by (compute);
    assert(z_filter_0_ct2_d8()[5] as nat+256*z_filter_0_ct2_d8()[6] as nat==u16::MAX as nat-76nat) by (compute);
    assert(zm::be_value(z_filter_0_ct2_d8(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_filter_0_ct2_d8(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_filter_0_ct2_d8()
    ensures zm::Adler(raw_filter_0_ct2_d8())==3484230667nat,
{
    hide(raw_filter_0_ct2_d8);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_filter_0_ct2_d8().len()==76nat) by (compute);
    assert(c::sums(raw_filter_0_ct2_d8(),0,76nat)==(9226nat,315249nat)) by (compute);
    c::sums_correct(raw_filter_0_ct2_d8(),0,76nat);
    assert(raw_filter_0_ct2_d8().subrange(0,76) =~= raw_filter_0_ct2_d8());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((315249nat%65521)*(u16::MAX as nat+1)+(1+9226nat)%65521==3484230667nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_filter_0_ct2_d8()
    ensures r::inflated(z_filter_0_ct2_d8(),a_filter_0_ct2_d8(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_filter_0_ct2_d8(),r2:false}),
        raw_filter_0_ct2_d8().len()==76nat,
{
    hide(z_filter_0_ct2_d8);hide(raw_filter_0_ct2_d8);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_filter_0_ct2_d8();checksum_filter_0_ct2_d8();
    assert(r::raw(a_filter_0_ct2_d8())==76nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_filter_0_ct2_d8(),raw_filter_0_ct2_d8(),76nat,120nat,156nat,3484230667nat);
}
#[verifier::spinoff_prover]
proof fn scan_filter_0_ct2_d8()
    ensures r::scan(raw_filter_0_ct2_d8(),a_filter_0_ct2_d8(),0)==Ok(()),
{
    hide(raw_filter_0_ct2_d8);
    assert(r::base(a_filter_0_ct2_d8(),0)==0nat && r::ph(a_filter_0_ct2_d8(),0)==4nat && r::count(a_filter_0_ct2_d8())==1nat) by (compute);
    assert(r::rb(a_filter_0_ct2_d8(),0)==18nat) by (compute);
    assert(raw_filter_0_ct2_d8()[0]==0u8) by (compute);
    assert(r::row_result(raw_filter_0_ct2_d8(),a_filter_0_ct2_d8(),0,0nat)==Ok(()));
    assert(raw_filter_0_ct2_d8()[19]==0u8) by (compute);
    assert(r::row_result(raw_filter_0_ct2_d8(),a_filter_0_ct2_d8(),0,1nat)==Ok(()));
    assert(raw_filter_0_ct2_d8()[38]==0u8) by (compute);
    assert(r::row_result(raw_filter_0_ct2_d8(),a_filter_0_ct2_d8(),0,2nat)==Ok(()));
    assert(raw_filter_0_ct2_d8()[57]==0u8) by (compute);
    assert(r::row_result(raw_filter_0_ct2_d8(),a_filter_0_ct2_d8(),0,3nat)==Ok(()));
    r::pass_step(raw_filter_0_ct2_d8(),a_filter_0_ct2_d8(),0,4nat);
    r::pass_step(raw_filter_0_ct2_d8(),a_filter_0_ct2_d8(),0,3nat);
    r::pass_step(raw_filter_0_ct2_d8(),a_filter_0_ct2_d8(),0,2nat);
    r::pass_step(raw_filter_0_ct2_d8(),a_filter_0_ct2_d8(),0,1nat);
    r::pass_step(raw_filter_0_ct2_d8(),a_filter_0_ct2_d8(),0,0nat);
    r::scan_step(raw_filter_0_ct2_d8(),a_filter_0_ct2_d8(),1);
    r::scan_step(raw_filter_0_ct2_d8(),a_filter_0_ct2_d8(),0);
}
#[verifier::spinoff_prover]
proof fn model_filter_0_ct2_d8()
    ensures r::still(bytes_filter_0_ct2_d8(),STRICT,false)==(r::Projection::Ready {width:6u32,height:4u32,bytes:96nat}),
{
    hide(bytes_filter_0_ct2_d8);hide(z_filter_0_ct2_d8);hide(raw_filter_0_ct2_d8);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_0_ct2_d8().len()==144nat) by (compute);
    assert(crate::model::mh(bytes_filter_0_ct2_d8())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:2u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_0_ct2_d8())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:2u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_0_ct2_d8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:87nat,next:132nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_0_ct2_d8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:87nat,next:132nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_0_ct2_d8(),132nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:140nat,len:0nat,next:144nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_0_ct2_d8(),132nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:140nat,len:0nat,next:144nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_0_ct2_d8().subrange(41,128)==z_filter_0_ct2_d8()) by (compute);
    walk_filter_0_ct2_d8(bytes_filter_0_ct2_d8());
    inflate_filter_0_ct2_d8();scan_filter_0_ct2_d8();
    assert(r::raw(a_filter_0_ct2_d8())==76nat) by (compute);
    assert(r::data(z_filter_0_ct2_d8(),a_filter_0_ct2_d8(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_filter_0_ct2_d8(),h_filter_0_ct2_d8(),z_filter_0_ct2_d8(),0nat,false);
}
fn public_filter_0_ct2_d8() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,2u8,0u8,0u8,0u8,34u8,102u8,217u8,20u8,0u8,0u8,0u8,87u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,76u8,0u8,179u8,255u8,0u8,130u8,201u8,136u8,75u8,0u8,41u8,139u8,10u8,24u8,81u8,11u8,150u8,76u8,112u8,11u8,192u8,240u8,148u8,0u8,38u8,252u8,186u8,71u8,79u8,32u8,71u8,72u8,111u8,215u8,127u8,61u8,24u8,48u8,223u8,149u8,46u8,117u8,0u8,66u8,198u8,224u8,254u8,7u8,188u8,239u8,45u8,234u8,214u8,142u8,226u8,70u8,154u8,227u8,194u8,43u8,114u8,0u8,246u8,114u8,49u8,144u8,235u8,140u8,92u8,53u8,61u8,218u8,219u8,44u8,250u8,209u8,220u8,81u8,135u8,178u8,207u8,173u8,36u8,11u8,101u8,174u8,80u8,132u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_0_ct2_d8());model_filter_0_ct2_d8();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==6u32 && s.height==4u32 && s.rgba@.len()==96nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// filter/0-ct2-d16; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_filter_0_ct2_d16() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,2u8,0u8,0u8,0u8,114u8,246u8,5u8,87u8,0u8,0u8,0u8,159u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,148u8,0u8,107u8,255u8,0u8,195u8,200u8,43u8,128u8,82u8,171u8,253u8,198u8,222u8,220u8,213u8,254u8,30u8,89u8,122u8,36u8,185u8,70u8,137u8,127u8,178u8,143u8,11u8,143u8,184u8,132u8,10u8,224u8,167u8,159u8,72u8,144u8,103u8,38u8,186u8,30u8,0u8,112u8,198u8,219u8,42u8,125u8,237u8,249u8,3u8,106u8,124u8,13u8,73u8,219u8,25u8,103u8,143u8,66u8,48u8,210u8,215u8,16u8,82u8,61u8,39u8,26u8,48u8,114u8,127u8,246u8,168u8,107u8,236u8,36u8,79u8,88u8,52u8,0u8,203u8,224u8,225u8,146u8,37u8,241u8,158u8,129u8,174u8,69u8,57u8,243u8,200u8,101u8,144u8,153u8,110u8,93u8,201u8,18u8,190u8,63u8,251u8,123u8,60u8,112u8,165u8,189u8,69u8,23u8,71u8,190u8,121u8,37u8,157u8,107u8,0u8,6u8,17u8,113u8,54u8,93u8,236u8,197u8,80u8,214u8,78u8,228u8,41u8,89u8,27u8,110u8,190u8,193u8,227u8,134u8,251u8,86u8,28u8,48u8,161u8,18u8,240u8,227u8,154u8,159u8,152u8,226u8,65u8,11u8,177u8,245u8,115u8,13u8,247u8,72u8,67u8,125u8,149u8,250u8,88u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_filter_0_ct2_d16() -> Seq<u8> {seq![120u8,156u8,1u8,148u8,0u8,107u8,255u8,0u8,195u8,200u8,43u8,128u8,82u8,171u8,253u8,198u8,222u8,220u8,213u8,254u8,30u8,89u8,122u8,36u8,185u8,70u8,137u8,127u8,178u8,143u8,11u8,143u8,184u8,132u8,10u8,224u8,167u8,159u8,72u8,144u8,103u8,38u8,186u8,30u8,0u8,112u8,198u8,219u8,42u8,125u8,237u8,249u8,3u8,106u8,124u8,13u8,73u8,219u8,25u8,103u8,143u8,66u8,48u8,210u8,215u8,16u8,82u8,61u8,39u8,26u8,48u8,114u8,127u8,246u8,168u8,107u8,236u8,36u8,79u8,88u8,52u8,0u8,203u8,224u8,225u8,146u8,37u8,241u8,158u8,129u8,174u8,69u8,57u8,243u8,200u8,101u8,144u8,153u8,110u8,93u8,201u8,18u8,190u8,63u8,251u8,123u8,60u8,112u8,165u8,189u8,69u8,23u8,71u8,190u8,121u8,37u8,157u8,107u8,0u8,6u8,17u8,113u8,54u8,93u8,236u8,197u8,80u8,214u8,78u8,228u8,41u8,89u8,27u8,110u8,190u8,193u8,227u8,134u8,251u8,86u8,28u8,48u8,161u8,18u8,240u8,227u8,154u8,159u8,152u8,226u8,65u8,11u8,177u8,245u8,115u8,13u8,247u8,72u8,67u8]}
spec fn h_filter_0_ct2_d16() -> crate::model::Ihdr {crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:2u8,interlace:0u8}}
spec fn a_filter_0_ct2_d16() -> r::Raster {r::shape(h_filter_0_ct2_d16(),6nat,4nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_filter_0_ct2_d16(b: Seq<u8>)
    requires b.len()==216,crate::model::mh(b)==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:2u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:159nat,next:204nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,204nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:212nat,len:0nat,next:216nat,crc_ok:true,synthetic:false})),
        b.subrange(41,200)==z_filter_0_ct2_d16(),
    ensures r::default_case(b,crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:2u8,interlace:0u8},z_filter_0_ct2_d16(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
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
spec fn raw_filter_0_ct2_d16() -> Seq<u8> {seq![0u8,195u8,200u8,43u8,128u8,82u8,171u8,253u8,198u8,222u8,220u8,213u8,254u8,30u8,89u8,122u8,36u8,185u8,70u8,137u8,127u8,178u8,143u8,11u8,143u8,184u8,132u8,10u8,224u8,167u8,159u8,72u8,144u8,103u8,38u8,186u8,30u8,0u8,112u8,198u8,219u8,42u8,125u8,237u8,249u8,3u8,106u8,124u8,13u8,73u8,219u8,25u8,103u8,143u8,66u8,48u8,210u8,215u8,16u8,82u8,61u8,39u8,26u8,48u8,114u8,127u8,246u8,168u8,107u8,236u8,36u8,79u8,88u8,52u8,0u8,203u8,224u8,225u8,146u8,37u8,241u8,158u8,129u8,174u8,69u8,57u8,243u8,200u8,101u8,144u8,153u8,110u8,93u8,201u8,18u8,190u8,63u8,251u8,123u8,60u8,112u8,165u8,189u8,69u8,23u8,71u8,190u8,121u8,37u8,157u8,107u8,0u8,6u8,17u8,113u8,54u8,93u8,236u8,197u8,80u8,214u8,78u8,228u8,41u8,89u8,27u8,110u8,190u8,193u8,227u8,134u8,251u8,86u8,28u8,48u8,161u8,18u8,240u8,227u8,154u8,159u8,152u8,226u8,65u8,11u8,177u8,245u8,115u8]}
#[verifier::spinoff_prover]
proof fn shape_filter_0_ct2_d16()
    ensures raw_filter_0_ct2_d16().len()==148nat,
        z_filter_0_ct2_d16().len()==159nat,
        z_filter_0_ct2_d16()[0]==120u8,
        z_filter_0_ct2_d16()[1]==156u8,
        zm::field_value(z_filter_0_ct2_d16(),16,1)==1nat,
        zm::field_value(z_filter_0_ct2_d16(),17,2)==0nat,
        zm::be_value(z_filter_0_ct2_d16(),155nat,4)==234309699nat,
        z_filter_0_ct2_d16().subrange(7,155)==raw_filter_0_ct2_d16(),
        z_filter_0_ct2_d16()[3] as nat+256*z_filter_0_ct2_d16()[4] as nat==148nat,
        z_filter_0_ct2_d16()[5] as nat+256*z_filter_0_ct2_d16()[6] as nat==u16::MAX as nat-148nat,
        zm::be_value(z_filter_0_ct2_d16(),0,1)==120nat,
        zm::be_value(z_filter_0_ct2_d16(),1,1)==156nat,
{
    hide(z_filter_0_ct2_d16);hide(raw_filter_0_ct2_d16);
    assert(raw_filter_0_ct2_d16().len()==148nat) by (compute);
    assert(z_filter_0_ct2_d16().len()==159nat) by (compute);
    assert(z_filter_0_ct2_d16()[0]==120u8) by (compute);
    assert(z_filter_0_ct2_d16()[1]==156u8) by (compute);
    assert(zm::field_value(z_filter_0_ct2_d16(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_filter_0_ct2_d16(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_filter_0_ct2_d16(),155nat,4)==234309699nat) by (compute);
    assert(z_filter_0_ct2_d16().subrange(7,155)==raw_filter_0_ct2_d16()) by (compute);
    assert(z_filter_0_ct2_d16()[3] as nat+256*z_filter_0_ct2_d16()[4] as nat==148nat) by (compute);
    assert(z_filter_0_ct2_d16()[5] as nat+256*z_filter_0_ct2_d16()[6] as nat==u16::MAX as nat-148nat) by (compute);
    assert(zm::be_value(z_filter_0_ct2_d16(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_filter_0_ct2_d16(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_filter_0_ct2_d16()
    ensures zm::Adler(raw_filter_0_ct2_d16())==234309699nat,
{
    hide(raw_filter_0_ct2_d16);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_filter_0_ct2_d16().len()==148nat) by (compute);
    assert(c::sums(raw_filter_0_ct2_d16(),0,148nat)==(18498nat,1379516nat)) by (compute);
    c::sums_correct(raw_filter_0_ct2_d16(),0,148nat);
    assert(raw_filter_0_ct2_d16().subrange(0,148) =~= raw_filter_0_ct2_d16());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((1379516nat%65521)*(u16::MAX as nat+1)+(1+18498nat)%65521==234309699nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_filter_0_ct2_d16()
    ensures r::inflated(z_filter_0_ct2_d16(),a_filter_0_ct2_d16(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_filter_0_ct2_d16(),r2:false}),
        raw_filter_0_ct2_d16().len()==148nat,
{
    hide(z_filter_0_ct2_d16);hide(raw_filter_0_ct2_d16);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_filter_0_ct2_d16();checksum_filter_0_ct2_d16();
    assert(r::raw(a_filter_0_ct2_d16())==148nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_filter_0_ct2_d16(),raw_filter_0_ct2_d16(),148nat,120nat,156nat,234309699nat);
}
#[verifier::spinoff_prover]
proof fn scan_filter_0_ct2_d16()
    ensures r::scan(raw_filter_0_ct2_d16(),a_filter_0_ct2_d16(),0)==Ok(()),
{
    hide(raw_filter_0_ct2_d16);
    assert(r::base(a_filter_0_ct2_d16(),0)==0nat && r::ph(a_filter_0_ct2_d16(),0)==4nat && r::count(a_filter_0_ct2_d16())==1nat) by (compute);
    assert(r::rb(a_filter_0_ct2_d16(),0)==36nat) by (compute);
    assert(raw_filter_0_ct2_d16()[0]==0u8) by (compute);
    assert(r::row_result(raw_filter_0_ct2_d16(),a_filter_0_ct2_d16(),0,0nat)==Ok(()));
    assert(raw_filter_0_ct2_d16()[37]==0u8) by (compute);
    assert(r::row_result(raw_filter_0_ct2_d16(),a_filter_0_ct2_d16(),0,1nat)==Ok(()));
    assert(raw_filter_0_ct2_d16()[74]==0u8) by (compute);
    assert(r::row_result(raw_filter_0_ct2_d16(),a_filter_0_ct2_d16(),0,2nat)==Ok(()));
    assert(raw_filter_0_ct2_d16()[111]==0u8) by (compute);
    assert(r::row_result(raw_filter_0_ct2_d16(),a_filter_0_ct2_d16(),0,3nat)==Ok(()));
    r::pass_step(raw_filter_0_ct2_d16(),a_filter_0_ct2_d16(),0,4nat);
    r::pass_step(raw_filter_0_ct2_d16(),a_filter_0_ct2_d16(),0,3nat);
    r::pass_step(raw_filter_0_ct2_d16(),a_filter_0_ct2_d16(),0,2nat);
    r::pass_step(raw_filter_0_ct2_d16(),a_filter_0_ct2_d16(),0,1nat);
    r::pass_step(raw_filter_0_ct2_d16(),a_filter_0_ct2_d16(),0,0nat);
    r::scan_step(raw_filter_0_ct2_d16(),a_filter_0_ct2_d16(),1);
    r::scan_step(raw_filter_0_ct2_d16(),a_filter_0_ct2_d16(),0);
}
#[verifier::spinoff_prover]
proof fn model_filter_0_ct2_d16()
    ensures r::still(bytes_filter_0_ct2_d16(),STRICT,false)==(r::Projection::Ready {width:6u32,height:4u32,bytes:96nat}),
{
    hide(bytes_filter_0_ct2_d16);hide(z_filter_0_ct2_d16);hide(raw_filter_0_ct2_d16);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_0_ct2_d16().len()==216nat) by (compute);
    assert(crate::model::mh(bytes_filter_0_ct2_d16())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:2u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_0_ct2_d16())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:2u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_0_ct2_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:159nat,next:204nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_0_ct2_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:159nat,next:204nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_0_ct2_d16(),204nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:212nat,len:0nat,next:216nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_0_ct2_d16(),204nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:212nat,len:0nat,next:216nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_0_ct2_d16().subrange(41,200)==z_filter_0_ct2_d16()) by (compute);
    walk_filter_0_ct2_d16(bytes_filter_0_ct2_d16());
    inflate_filter_0_ct2_d16();scan_filter_0_ct2_d16();
    assert(r::raw(a_filter_0_ct2_d16())==148nat) by (compute);
    assert(r::data(z_filter_0_ct2_d16(),a_filter_0_ct2_d16(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_filter_0_ct2_d16(),h_filter_0_ct2_d16(),z_filter_0_ct2_d16(),0nat,false);
}
fn public_filter_0_ct2_d16() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,2u8,0u8,0u8,0u8,114u8,246u8,5u8,87u8,0u8,0u8,0u8,159u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,148u8,0u8,107u8,255u8,0u8,195u8,200u8,43u8,128u8,82u8,171u8,253u8,198u8,222u8,220u8,213u8,254u8,30u8,89u8,122u8,36u8,185u8,70u8,137u8,127u8,178u8,143u8,11u8,143u8,184u8,132u8,10u8,224u8,167u8,159u8,72u8,144u8,103u8,38u8,186u8,30u8,0u8,112u8,198u8,219u8,42u8,125u8,237u8,249u8,3u8,106u8,124u8,13u8,73u8,219u8,25u8,103u8,143u8,66u8,48u8,210u8,215u8,16u8,82u8,61u8,39u8,26u8,48u8,114u8,127u8,246u8,168u8,107u8,236u8,36u8,79u8,88u8,52u8,0u8,203u8,224u8,225u8,146u8,37u8,241u8,158u8,129u8,174u8,69u8,57u8,243u8,200u8,101u8,144u8,153u8,110u8,93u8,201u8,18u8,190u8,63u8,251u8,123u8,60u8,112u8,165u8,189u8,69u8,23u8,71u8,190u8,121u8,37u8,157u8,107u8,0u8,6u8,17u8,113u8,54u8,93u8,236u8,197u8,80u8,214u8,78u8,228u8,41u8,89u8,27u8,110u8,190u8,193u8,227u8,134u8,251u8,86u8,28u8,48u8,161u8,18u8,240u8,227u8,154u8,159u8,152u8,226u8,65u8,11u8,177u8,245u8,115u8,13u8,247u8,72u8,67u8,125u8,149u8,250u8,88u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_0_ct2_d16());model_filter_0_ct2_d16();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==6u32 && s.height==4u32 && s.rgba@.len()==96nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// filter/0-ct4-d8; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_filter_0_ct4_d8() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,4u8,0u8,0u8,0u8,7u8,13u8,134u8,200u8,0u8,0u8,0u8,63u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,52u8,0u8,203u8,255u8,0u8,132u8,182u8,213u8,13u8,179u8,117u8,92u8,240u8,34u8,58u8,244u8,195u8,0u8,10u8,154u8,2u8,192u8,114u8,58u8,68u8,205u8,171u8,170u8,21u8,14u8,0u8,212u8,0u8,204u8,250u8,203u8,205u8,187u8,163u8,76u8,202u8,15u8,20u8,0u8,162u8,207u8,96u8,191u8,81u8,159u8,128u8,227u8,38u8,150u8,62u8,33u8,121u8,35u8,24u8,6u8,250u8,7u8,239u8,81u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_filter_0_ct4_d8() -> Seq<u8> {seq![120u8,156u8,1u8,52u8,0u8,203u8,255u8,0u8,132u8,182u8,213u8,13u8,179u8,117u8,92u8,240u8,34u8,58u8,244u8,195u8,0u8,10u8,154u8,2u8,192u8,114u8,58u8,68u8,205u8,171u8,170u8,21u8,14u8,0u8,212u8,0u8,204u8,250u8,203u8,205u8,187u8,163u8,76u8,202u8,15u8,20u8,0u8,162u8,207u8,96u8,191u8,81u8,159u8,128u8,227u8,38u8,150u8,62u8,33u8,121u8,35u8,24u8,6u8]}
spec fn h_filter_0_ct4_d8() -> crate::model::Ihdr {crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:4u8,interlace:0u8}}
spec fn a_filter_0_ct4_d8() -> r::Raster {r::shape(h_filter_0_ct4_d8(),6nat,4nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_filter_0_ct4_d8(b: Seq<u8>)
    requires b.len()==120,crate::model::mh(b)==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:4u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:63nat,next:108nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,108nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:116nat,len:0nat,next:120nat,crc_ok:true,synthetic:false})),
        b.subrange(41,104)==z_filter_0_ct4_d8(),
    ensures r::default_case(b,crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:4u8,interlace:0u8},z_filter_0_ct4_d8(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_filter_0_ct4_d8() -> Seq<u8> {seq![0u8,132u8,182u8,213u8,13u8,179u8,117u8,92u8,240u8,34u8,58u8,244u8,195u8,0u8,10u8,154u8,2u8,192u8,114u8,58u8,68u8,205u8,171u8,170u8,21u8,14u8,0u8,212u8,0u8,204u8,250u8,203u8,205u8,187u8,163u8,76u8,202u8,15u8,20u8,0u8,162u8,207u8,96u8,191u8,81u8,159u8,128u8,227u8,38u8,150u8,62u8,33u8]}
#[verifier::spinoff_prover]
proof fn shape_filter_0_ct4_d8()
    ensures raw_filter_0_ct4_d8().len()==52nat,
        z_filter_0_ct4_d8().len()==63nat,
        z_filter_0_ct4_d8()[0]==120u8,
        z_filter_0_ct4_d8()[1]==156u8,
        zm::field_value(z_filter_0_ct4_d8(),16,1)==1nat,
        zm::field_value(z_filter_0_ct4_d8(),17,2)==0nat,
        zm::be_value(z_filter_0_ct4_d8(),59nat,4)==2032343046nat,
        z_filter_0_ct4_d8().subrange(7,59)==raw_filter_0_ct4_d8(),
        z_filter_0_ct4_d8()[3] as nat+256*z_filter_0_ct4_d8()[4] as nat==52nat,
        z_filter_0_ct4_d8()[5] as nat+256*z_filter_0_ct4_d8()[6] as nat==u16::MAX as nat-52nat,
        zm::be_value(z_filter_0_ct4_d8(),0,1)==120nat,
        zm::be_value(z_filter_0_ct4_d8(),1,1)==156nat,
{
    hide(z_filter_0_ct4_d8);hide(raw_filter_0_ct4_d8);
    assert(raw_filter_0_ct4_d8().len()==52nat) by (compute);
    assert(z_filter_0_ct4_d8().len()==63nat) by (compute);
    assert(z_filter_0_ct4_d8()[0]==120u8) by (compute);
    assert(z_filter_0_ct4_d8()[1]==156u8) by (compute);
    assert(zm::field_value(z_filter_0_ct4_d8(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_filter_0_ct4_d8(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_filter_0_ct4_d8(),59nat,4)==2032343046nat) by (compute);
    assert(z_filter_0_ct4_d8().subrange(7,59)==raw_filter_0_ct4_d8()) by (compute);
    assert(z_filter_0_ct4_d8()[3] as nat+256*z_filter_0_ct4_d8()[4] as nat==52nat) by (compute);
    assert(z_filter_0_ct4_d8()[5] as nat+256*z_filter_0_ct4_d8()[6] as nat==u16::MAX as nat-52nat) by (compute);
    assert(zm::be_value(z_filter_0_ct4_d8(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_filter_0_ct4_d8(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_filter_0_ct4_d8()
    ensures zm::Adler(raw_filter_0_ct4_d8())==2032343046nat,
{
    hide(raw_filter_0_ct4_d8);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_filter_0_ct4_d8().len()==52nat) by (compute);
    assert(c::sums(raw_filter_0_ct4_d8(),0,52nat)==(6149nat,162053nat)) by (compute);
    c::sums_correct(raw_filter_0_ct4_d8(),0,52nat);
    assert(raw_filter_0_ct4_d8().subrange(0,52) =~= raw_filter_0_ct4_d8());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((162053nat%65521)*(u16::MAX as nat+1)+(1+6149nat)%65521==2032343046nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_filter_0_ct4_d8()
    ensures r::inflated(z_filter_0_ct4_d8(),a_filter_0_ct4_d8(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_filter_0_ct4_d8(),r2:false}),
        raw_filter_0_ct4_d8().len()==52nat,
{
    hide(z_filter_0_ct4_d8);hide(raw_filter_0_ct4_d8);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_filter_0_ct4_d8();checksum_filter_0_ct4_d8();
    assert(r::raw(a_filter_0_ct4_d8())==52nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_filter_0_ct4_d8(),raw_filter_0_ct4_d8(),52nat,120nat,156nat,2032343046nat);
}
#[verifier::spinoff_prover]
proof fn scan_filter_0_ct4_d8()
    ensures r::scan(raw_filter_0_ct4_d8(),a_filter_0_ct4_d8(),0)==Ok(()),
{
    hide(raw_filter_0_ct4_d8);
    assert(r::base(a_filter_0_ct4_d8(),0)==0nat && r::ph(a_filter_0_ct4_d8(),0)==4nat && r::count(a_filter_0_ct4_d8())==1nat) by (compute);
    assert(r::rb(a_filter_0_ct4_d8(),0)==12nat) by (compute);
    assert(raw_filter_0_ct4_d8()[0]==0u8) by (compute);
    assert(r::row_result(raw_filter_0_ct4_d8(),a_filter_0_ct4_d8(),0,0nat)==Ok(()));
    assert(raw_filter_0_ct4_d8()[13]==0u8) by (compute);
    assert(r::row_result(raw_filter_0_ct4_d8(),a_filter_0_ct4_d8(),0,1nat)==Ok(()));
    assert(raw_filter_0_ct4_d8()[26]==0u8) by (compute);
    assert(r::row_result(raw_filter_0_ct4_d8(),a_filter_0_ct4_d8(),0,2nat)==Ok(()));
    assert(raw_filter_0_ct4_d8()[39]==0u8) by (compute);
    assert(r::row_result(raw_filter_0_ct4_d8(),a_filter_0_ct4_d8(),0,3nat)==Ok(()));
    r::pass_step(raw_filter_0_ct4_d8(),a_filter_0_ct4_d8(),0,4nat);
    r::pass_step(raw_filter_0_ct4_d8(),a_filter_0_ct4_d8(),0,3nat);
    r::pass_step(raw_filter_0_ct4_d8(),a_filter_0_ct4_d8(),0,2nat);
    r::pass_step(raw_filter_0_ct4_d8(),a_filter_0_ct4_d8(),0,1nat);
    r::pass_step(raw_filter_0_ct4_d8(),a_filter_0_ct4_d8(),0,0nat);
    r::scan_step(raw_filter_0_ct4_d8(),a_filter_0_ct4_d8(),1);
    r::scan_step(raw_filter_0_ct4_d8(),a_filter_0_ct4_d8(),0);
}
#[verifier::spinoff_prover]
proof fn model_filter_0_ct4_d8()
    ensures r::still(bytes_filter_0_ct4_d8(),STRICT,false)==(r::Projection::Ready {width:6u32,height:4u32,bytes:96nat}),
{
    hide(bytes_filter_0_ct4_d8);hide(z_filter_0_ct4_d8);hide(raw_filter_0_ct4_d8);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_0_ct4_d8().len()==120nat) by (compute);
    assert(crate::model::mh(bytes_filter_0_ct4_d8())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:4u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_0_ct4_d8())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:4u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_0_ct4_d8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:63nat,next:108nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_0_ct4_d8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:63nat,next:108nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_0_ct4_d8(),108nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:116nat,len:0nat,next:120nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_0_ct4_d8(),108nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:116nat,len:0nat,next:120nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_0_ct4_d8().subrange(41,104)==z_filter_0_ct4_d8()) by (compute);
    walk_filter_0_ct4_d8(bytes_filter_0_ct4_d8());
    inflate_filter_0_ct4_d8();scan_filter_0_ct4_d8();
    assert(r::raw(a_filter_0_ct4_d8())==52nat) by (compute);
    assert(r::data(z_filter_0_ct4_d8(),a_filter_0_ct4_d8(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_filter_0_ct4_d8(),h_filter_0_ct4_d8(),z_filter_0_ct4_d8(),0nat,false);
}
fn public_filter_0_ct4_d8() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,4u8,0u8,0u8,0u8,7u8,13u8,134u8,200u8,0u8,0u8,0u8,63u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,52u8,0u8,203u8,255u8,0u8,132u8,182u8,213u8,13u8,179u8,117u8,92u8,240u8,34u8,58u8,244u8,195u8,0u8,10u8,154u8,2u8,192u8,114u8,58u8,68u8,205u8,171u8,170u8,21u8,14u8,0u8,212u8,0u8,204u8,250u8,203u8,205u8,187u8,163u8,76u8,202u8,15u8,20u8,0u8,162u8,207u8,96u8,191u8,81u8,159u8,128u8,227u8,38u8,150u8,62u8,33u8,121u8,35u8,24u8,6u8,250u8,7u8,239u8,81u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_0_ct4_d8());model_filter_0_ct4_d8();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==6u32 && s.height==4u32 && s.rgba@.len()==96nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// filter/0-ct6-d16; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_filter_0_ct6_d16() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,6u8,0u8,0u8,0u8,253u8,148u8,146u8,0u8,0u8,0u8,0u8,207u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,196u8,0u8,59u8,255u8,0u8,242u8,221u8,15u8,221u8,175u8,215u8,138u8,191u8,142u8,170u8,244u8,218u8,95u8,104u8,64u8,63u8,135u8,219u8,42u8,226u8,169u8,94u8,173u8,66u8,150u8,85u8,165u8,35u8,146u8,141u8,200u8,77u8,182u8,189u8,185u8,114u8,105u8,188u8,106u8,103u8,150u8,217u8,158u8,196u8,184u8,15u8,186u8,220u8,0u8,169u8,65u8,87u8,50u8,208u8,208u8,121u8,104u8,127u8,150u8,253u8,9u8,212u8,63u8,7u8,31u8,187u8,70u8,106u8,126u8,51u8,46u8,233u8,77u8,3u8,135u8,234u8,88u8,46u8,33u8,204u8,197u8,185u8,120u8,196u8,3u8,92u8,66u8,252u8,101u8,231u8,21u8,201u8,170u8,106u8,23u8,230u8,7u8,0u8,210u8,139u8,142u8,3u8,164u8,150u8,52u8,8u8,221u8,195u8,147u8,138u8,122u8,133u8,99u8,181u8,229u8,63u8,222u8,137u8,33u8,209u8,111u8,251u8,60u8,176u8,109u8,128u8,171u8,30u8,50u8,194u8,133u8,150u8,25u8,219u8,148u8,49u8,149u8,40u8,58u8,159u8,238u8,6u8,190u8,143u8,31u8,25u8,0u8,84u8,222u8,127u8,184u8,176u8,146u8,217u8,144u8,51u8,106u8,11u8,121u8,206u8,40u8,43u8,243u8,200u8,67u8,222u8,110u8,40u8,182u8,85u8,87u8,164u8,134u8,88u8,47u8,234u8,163u8,255u8,37u8,113u8,3u8,99u8,182u8,10u8,124u8,94u8,29u8,0u8,36u8,209u8,180u8,102u8,75u8,238u8,214u8,148u8,154u8,96u8,252u8,75u8,11u8,232u8,231u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_filter_0_ct6_d16() -> Seq<u8> {seq![120u8,156u8,1u8,196u8,0u8,59u8,255u8,0u8,242u8,221u8,15u8,221u8,175u8,215u8,138u8,191u8,142u8,170u8,244u8,218u8,95u8,104u8,64u8,63u8,135u8,219u8,42u8,226u8,169u8,94u8,173u8,66u8,150u8,85u8,165u8,35u8,146u8,141u8,200u8,77u8,182u8,189u8,185u8,114u8,105u8,188u8,106u8,103u8,150u8,217u8,158u8,196u8,184u8,15u8,186u8,220u8,0u8,169u8,65u8,87u8,50u8,208u8,208u8,121u8,104u8,127u8,150u8,253u8,9u8,212u8,63u8,7u8,31u8,187u8,70u8,106u8,126u8,51u8,46u8,233u8,77u8,3u8,135u8,234u8,88u8,46u8,33u8,204u8,197u8,185u8,120u8,196u8,3u8,92u8,66u8,252u8,101u8,231u8,21u8,201u8,170u8,106u8,23u8,230u8,7u8,0u8,210u8,139u8,142u8,3u8,164u8,150u8,52u8,8u8,221u8,195u8,147u8,138u8,122u8,133u8,99u8,181u8,229u8,63u8,222u8,137u8,33u8,209u8,111u8,251u8,60u8,176u8,109u8,128u8,171u8,30u8,50u8,194u8,133u8,150u8,25u8,219u8,148u8,49u8,149u8,40u8,58u8,159u8,238u8,6u8,190u8,143u8,31u8,25u8,0u8,84u8,222u8,127u8,184u8,176u8,146u8,217u8,144u8,51u8,106u8,11u8,121u8,206u8,40u8,43u8,243u8,200u8,67u8,222u8,110u8,40u8,182u8,85u8,87u8,164u8,134u8,88u8,47u8,234u8,163u8,255u8,37u8,113u8,3u8,99u8,182u8,10u8,124u8,94u8,29u8,0u8,36u8,209u8,180u8,102u8,75u8,238u8,214u8,148u8,154u8,96u8,252u8]}
spec fn h_filter_0_ct6_d16() -> crate::model::Ihdr {crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:6u8,interlace:0u8}}
spec fn a_filter_0_ct6_d16() -> r::Raster {r::shape(h_filter_0_ct6_d16(),6nat,4nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_filter_0_ct6_d16(b: Seq<u8>)
    requires b.len()==264,crate::model::mh(b)==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:6u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:207nat,next:252nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,252nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:260nat,len:0nat,next:264nat,crc_ok:true,synthetic:false})),
        b.subrange(41,248)==z_filter_0_ct6_d16(),
    ensures r::default_case(b,crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:6u8,interlace:0u8},z_filter_0_ct6_d16(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
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
spec fn raw_filter_0_ct6_d16() -> Seq<u8> {seq![0u8,242u8,221u8,15u8,221u8,175u8,215u8,138u8,191u8,142u8,170u8,244u8,218u8,95u8,104u8,64u8,63u8,135u8,219u8,42u8,226u8,169u8,94u8,173u8,66u8,150u8,85u8,165u8,35u8,146u8,141u8,200u8,77u8,182u8,189u8,185u8,114u8,105u8,188u8,106u8,103u8,150u8,217u8,158u8,196u8,184u8,15u8,186u8,220u8,0u8,169u8,65u8,87u8,50u8,208u8,208u8,121u8,104u8,127u8,150u8,253u8,9u8,212u8,63u8,7u8,31u8,187u8,70u8,106u8,126u8,51u8,46u8,233u8,77u8,3u8,135u8,234u8,88u8,46u8,33u8,204u8,197u8,185u8,120u8,196u8,3u8,92u8,66u8,252u8,101u8,231u8,21u8,201u8,170u8,106u8,23u8,230u8,7u8,0u8,210u8,139u8,142u8,3u8,164u8,150u8,52u8,8u8,221u8,195u8,147u8,138u8,122u8,133u8,99u8,181u8,229u8,63u8,222u8,137u8,33u8,209u8,111u8,251u8,60u8,176u8,109u8,128u8,171u8,30u8,50u8,194u8,133u8,150u8,25u8,219u8,148u8,49u8,149u8,40u8,58u8,159u8,238u8,6u8,190u8,143u8,31u8,25u8,0u8,84u8,222u8,127u8,184u8,176u8,146u8,217u8,144u8,51u8,106u8,11u8,121u8,206u8,40u8,43u8,243u8,200u8,67u8,222u8,110u8,40u8,182u8,85u8,87u8,164u8,134u8,88u8,47u8,234u8,163u8,255u8,37u8,113u8,3u8,99u8,182u8,10u8,124u8,94u8,29u8,0u8,36u8,209u8,180u8,102u8,75u8,238u8,214u8]}
#[verifier::spinoff_prover]
proof fn shape_filter_0_ct6_d16()
    ensures raw_filter_0_ct6_d16().len()==196nat,
        z_filter_0_ct6_d16().len()==207nat,
        z_filter_0_ct6_d16()[0]==120u8,
        z_filter_0_ct6_d16()[1]==156u8,
        zm::field_value(z_filter_0_ct6_d16(),16,1)==1nat,
        zm::field_value(z_filter_0_ct6_d16(),17,2)==0nat,
        zm::be_value(z_filter_0_ct6_d16(),203nat,4)==2493145340nat,
        z_filter_0_ct6_d16().subrange(7,203)==raw_filter_0_ct6_d16(),
        z_filter_0_ct6_d16()[3] as nat+256*z_filter_0_ct6_d16()[4] as nat==196nat,
        z_filter_0_ct6_d16()[5] as nat+256*z_filter_0_ct6_d16()[6] as nat==u16::MAX as nat-196nat,
        zm::be_value(z_filter_0_ct6_d16(),0,1)==120nat,
        zm::be_value(z_filter_0_ct6_d16(),1,1)==156nat,
{
    hide(z_filter_0_ct6_d16);hide(raw_filter_0_ct6_d16);
    assert(raw_filter_0_ct6_d16().len()==196nat) by (compute);
    assert(z_filter_0_ct6_d16().len()==207nat) by (compute);
    assert(z_filter_0_ct6_d16()[0]==120u8) by (compute);
    assert(z_filter_0_ct6_d16()[1]==156u8) by (compute);
    assert(zm::field_value(z_filter_0_ct6_d16(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_filter_0_ct6_d16(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_filter_0_ct6_d16(),203nat,4)==2493145340nat) by (compute);
    assert(z_filter_0_ct6_d16().subrange(7,203)==raw_filter_0_ct6_d16()) by (compute);
    assert(z_filter_0_ct6_d16()[3] as nat+256*z_filter_0_ct6_d16()[4] as nat==196nat) by (compute);
    assert(z_filter_0_ct6_d16()[5] as nat+256*z_filter_0_ct6_d16()[6] as nat==u16::MAX as nat-196nat) by (compute);
    assert(zm::be_value(z_filter_0_ct6_d16(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_filter_0_ct6_d16(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_filter_0_ct6_d16()
    ensures zm::Adler(raw_filter_0_ct6_d16())==2493145340nat,
{
    hide(raw_filter_0_ct6_d16);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_filter_0_ct6_d16().len()==196nat) by (compute);
    assert(c::sums(raw_filter_0_ct6_d16(),0,196nat)==(24827nat,2527840nat)) by (compute);
    c::sums_correct(raw_filter_0_ct6_d16(),0,196nat);
    assert(raw_filter_0_ct6_d16().subrange(0,196) =~= raw_filter_0_ct6_d16());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((2527840nat%65521)*(u16::MAX as nat+1)+(1+24827nat)%65521==2493145340nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_filter_0_ct6_d16()
    ensures r::inflated(z_filter_0_ct6_d16(),a_filter_0_ct6_d16(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_filter_0_ct6_d16(),r2:false}),
        raw_filter_0_ct6_d16().len()==196nat,
{
    hide(z_filter_0_ct6_d16);hide(raw_filter_0_ct6_d16);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_filter_0_ct6_d16();checksum_filter_0_ct6_d16();
    assert(r::raw(a_filter_0_ct6_d16())==196nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_filter_0_ct6_d16(),raw_filter_0_ct6_d16(),196nat,120nat,156nat,2493145340nat);
}
#[verifier::spinoff_prover]
proof fn scan_filter_0_ct6_d16()
    ensures r::scan(raw_filter_0_ct6_d16(),a_filter_0_ct6_d16(),0)==Ok(()),
{
    hide(raw_filter_0_ct6_d16);
    assert(r::base(a_filter_0_ct6_d16(),0)==0nat && r::ph(a_filter_0_ct6_d16(),0)==4nat && r::count(a_filter_0_ct6_d16())==1nat) by (compute);
    assert(r::rb(a_filter_0_ct6_d16(),0)==48nat) by (compute);
    assert(raw_filter_0_ct6_d16()[0]==0u8) by (compute);
    assert(r::row_result(raw_filter_0_ct6_d16(),a_filter_0_ct6_d16(),0,0nat)==Ok(()));
    assert(raw_filter_0_ct6_d16()[49]==0u8) by (compute);
    assert(r::row_result(raw_filter_0_ct6_d16(),a_filter_0_ct6_d16(),0,1nat)==Ok(()));
    assert(raw_filter_0_ct6_d16()[98]==0u8) by (compute);
    assert(r::row_result(raw_filter_0_ct6_d16(),a_filter_0_ct6_d16(),0,2nat)==Ok(()));
    assert(raw_filter_0_ct6_d16()[147]==0u8) by (compute);
    assert(r::row_result(raw_filter_0_ct6_d16(),a_filter_0_ct6_d16(),0,3nat)==Ok(()));
    r::pass_step(raw_filter_0_ct6_d16(),a_filter_0_ct6_d16(),0,4nat);
    r::pass_step(raw_filter_0_ct6_d16(),a_filter_0_ct6_d16(),0,3nat);
    r::pass_step(raw_filter_0_ct6_d16(),a_filter_0_ct6_d16(),0,2nat);
    r::pass_step(raw_filter_0_ct6_d16(),a_filter_0_ct6_d16(),0,1nat);
    r::pass_step(raw_filter_0_ct6_d16(),a_filter_0_ct6_d16(),0,0nat);
    r::scan_step(raw_filter_0_ct6_d16(),a_filter_0_ct6_d16(),1);
    r::scan_step(raw_filter_0_ct6_d16(),a_filter_0_ct6_d16(),0);
}
#[verifier::spinoff_prover]
proof fn model_filter_0_ct6_d16()
    ensures r::still(bytes_filter_0_ct6_d16(),STRICT,false)==(r::Projection::Ready {width:6u32,height:4u32,bytes:96nat}),
{
    hide(bytes_filter_0_ct6_d16);hide(z_filter_0_ct6_d16);hide(raw_filter_0_ct6_d16);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_0_ct6_d16().len()==264nat) by (compute);
    assert(crate::model::mh(bytes_filter_0_ct6_d16())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:6u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_0_ct6_d16())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:6u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_0_ct6_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:207nat,next:252nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_0_ct6_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:207nat,next:252nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_0_ct6_d16(),252nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:260nat,len:0nat,next:264nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_0_ct6_d16(),252nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:260nat,len:0nat,next:264nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_0_ct6_d16().subrange(41,248)==z_filter_0_ct6_d16()) by (compute);
    walk_filter_0_ct6_d16(bytes_filter_0_ct6_d16());
    inflate_filter_0_ct6_d16();scan_filter_0_ct6_d16();
    assert(r::raw(a_filter_0_ct6_d16())==196nat) by (compute);
    assert(r::data(z_filter_0_ct6_d16(),a_filter_0_ct6_d16(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_filter_0_ct6_d16(),h_filter_0_ct6_d16(),z_filter_0_ct6_d16(),0nat,false);
}
fn public_filter_0_ct6_d16() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,6u8,0u8,0u8,0u8,253u8,148u8,146u8,0u8,0u8,0u8,0u8,207u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,196u8,0u8,59u8,255u8,0u8,242u8,221u8,15u8,221u8,175u8,215u8,138u8,191u8,142u8,170u8,244u8,218u8,95u8,104u8,64u8,63u8,135u8,219u8,42u8,226u8,169u8,94u8,173u8,66u8,150u8,85u8,165u8,35u8,146u8,141u8,200u8,77u8,182u8,189u8,185u8,114u8,105u8,188u8,106u8,103u8,150u8,217u8,158u8,196u8,184u8,15u8,186u8,220u8,0u8,169u8,65u8,87u8,50u8,208u8,208u8,121u8,104u8,127u8,150u8,253u8,9u8,212u8,63u8,7u8,31u8,187u8,70u8,106u8,126u8,51u8,46u8,233u8,77u8,3u8,135u8,234u8,88u8,46u8,33u8,204u8,197u8,185u8,120u8,196u8,3u8,92u8,66u8,252u8,101u8,231u8,21u8,201u8,170u8,106u8,23u8,230u8,7u8,0u8,210u8,139u8,142u8,3u8,164u8,150u8,52u8,8u8,221u8,195u8,147u8,138u8,122u8,133u8,99u8,181u8,229u8,63u8,222u8,137u8,33u8,209u8,111u8,251u8,60u8,176u8,109u8,128u8,171u8,30u8,50u8,194u8,133u8,150u8,25u8,219u8,148u8,49u8,149u8,40u8,58u8,159u8,238u8,6u8,190u8,143u8,31u8,25u8,0u8,84u8,222u8,127u8,184u8,176u8,146u8,217u8,144u8,51u8,106u8,11u8,121u8,206u8,40u8,43u8,243u8,200u8,67u8,222u8,110u8,40u8,182u8,85u8,87u8,164u8,134u8,88u8,47u8,234u8,163u8,255u8,37u8,113u8,3u8,99u8,182u8,10u8,124u8,94u8,29u8,0u8,36u8,209u8,180u8,102u8,75u8,238u8,214u8,148u8,154u8,96u8,252u8,75u8,11u8,232u8,231u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_0_ct6_d16());model_filter_0_ct6_d16();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==6u32 && s.height==4u32 && s.rgba@.len()==96nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// filter/1-ct0-d1; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_filter_1_ct0_d1() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,1u8,0u8,0u8,0u8,0u8,133u8,127u8,115u8,238u8,0u8,0u8,0u8,16u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,156u8,195u8,120u8,134u8,241u8,8u8,35u8,15u8,0u8,10u8,180u8,2u8,61u8,133u8,32u8,85u8,115u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_filter_1_ct0_d1() -> Seq<u8> {seq![120u8,156u8,99u8,156u8,195u8,120u8,134u8,241u8,8u8,35u8,15u8,0u8,10u8,180u8,2u8,61u8]}
spec fn h_filter_1_ct0_d1() -> crate::model::Ihdr {crate::model::Ihdr {width:6u32,height:4u32,depth:1u8,color_type:0u8,interlace:0u8}}
spec fn a_filter_1_ct0_d1() -> r::Raster {r::shape(h_filter_1_ct0_d1(),6nat,4nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_filter_1_ct0_d1(b: Seq<u8>)
    requires b.len()==73,crate::model::mh(b)==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:1u8,color_type:0u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:16nat,next:61nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,61nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:69nat,len:0nat,next:73nat,crc_ok:true,synthetic:false})),
        b.subrange(41,57)==z_filter_1_ct0_d1(),
    ensures r::default_case(b,crate::model::Ihdr {width:6u32,height:4u32,depth:1u8,color_type:0u8,interlace:0u8},z_filter_1_ct0_d1(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
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
spec fn raw_filter_1_ct0_d1() -> Seq<u8> {seq![1u8,156u8,1u8,204u8,1u8,196u8,1u8,12u8]}
spec fn ts_filter_1_ct0_d1() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:36nat,before:1nat,after:2nat,symbol:156nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:36nat,next:44nat,before:2nat,after:3nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:44nat,next:53nat,before:3nat,after:4nat,symbol:204nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:53nat,next:61nat,before:4nat,after:5nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:61nat,next:70nat,before:5nat,after:6nat,symbol:196nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:70nat,next:78nat,before:6nat,after:7nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:78nat,next:86nat,before:7nat,after:8nat,symbol:12nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:86nat,next:93nat,before:8nat,after:8nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_filter_1_ct0_d1()
    ensures t::trace(z_filter_1_ct0_d1(),raw_filter_1_ct0_d1(),ts_filter_1_ct0_d1(),0,9nat),
{
    hide(z_filter_1_ct0_d1);hide(raw_filter_1_ct0_d1);hide(ts_filter_1_ct0_d1);hide(t::valid_step);
    assert(t::trace(z_filter_1_ct0_d1(),raw_filter_1_ct0_d1(),ts_filter_1_ct0_d1(),0,9nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_filter_1_ct0_d1()
    ensures raw_filter_1_ct0_d1().len()==8nat,
        z_filter_1_ct0_d1().len()==16nat,
        z_filter_1_ct0_d1()[0]==120u8,
        z_filter_1_ct0_d1()[1]==156u8,
        zm::field_value(z_filter_1_ct0_d1(),16,1)==1nat,
        zm::field_value(z_filter_1_ct0_d1(),17,2)==1nat,
        zm::be_value(z_filter_1_ct0_d1(),12nat,4)==179569213nat,
        ts_filter_1_ct0_d1().len()==9nat,
        ts_filter_1_ct0_d1()[0].pos==19nat,
        ts_filter_1_ct0_d1()[0].before==0nat,
        ts_filter_1_ct0_d1().last().next==93nat,
{
    hide(z_filter_1_ct0_d1);hide(raw_filter_1_ct0_d1);hide(ts_filter_1_ct0_d1);
    assert(raw_filter_1_ct0_d1().len()==8nat) by (compute);
    assert(z_filter_1_ct0_d1().len()==16nat) by (compute);
    assert(z_filter_1_ct0_d1()[0]==120u8) by (compute);
    assert(z_filter_1_ct0_d1()[1]==156u8) by (compute);
    assert(zm::field_value(z_filter_1_ct0_d1(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_filter_1_ct0_d1(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_filter_1_ct0_d1(),12nat,4)==179569213nat) by (compute);
    assert(ts_filter_1_ct0_d1().len()==9nat) by (compute);
    assert(ts_filter_1_ct0_d1()[0].pos==19nat) by (compute);
    assert(ts_filter_1_ct0_d1()[0].before==0nat) by (compute);
    assert(ts_filter_1_ct0_d1().last().next==93nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_filter_1_ct0_d1()
    ensures zm::Adler(raw_filter_1_ct0_d1())==179569213nat,
{
    hide(raw_filter_1_ct0_d1);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_filter_1_ct0_d1().len()==8nat) by (compute);
    assert(c::sums(raw_filter_1_ct0_d1(),0,8nat)==(572nat,2740nat)) by (compute);
    c::sums_correct(raw_filter_1_ct0_d1(),0,8nat);
    assert(raw_filter_1_ct0_d1().subrange(0,8) =~= raw_filter_1_ct0_d1());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((2740nat%65521)*(u16::MAX as nat+1)+(1+572nat)%65521==179569213nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_filter_1_ct0_d1()
    ensures r::inflated(z_filter_1_ct0_d1(),a_filter_1_ct0_d1(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_filter_1_ct0_d1(),r2:false}),
        raw_filter_1_ct0_d1().len()==8nat,
{
    hide(z_filter_1_ct0_d1);hide(raw_filter_1_ct0_d1);hide(ts_filter_1_ct0_d1);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_filter_1_ct0_d1();checksum_filter_1_ct0_d1();
    assert(r::raw(a_filter_1_ct0_d1())==8nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_filter_1_ct0_d1();
    stream::fixed_stream(z_filter_1_ct0_d1(),raw_filter_1_ct0_d1(),ts_filter_1_ct0_d1(),8nat,156nat,179569213nat);
}
#[verifier::spinoff_prover]
proof fn scan_filter_1_ct0_d1()
    ensures r::scan(raw_filter_1_ct0_d1(),a_filter_1_ct0_d1(),0)==Ok(()),
{
    hide(raw_filter_1_ct0_d1);
    assert(r::base(a_filter_1_ct0_d1(),0)==0nat && r::ph(a_filter_1_ct0_d1(),0)==4nat && r::count(a_filter_1_ct0_d1())==1nat) by (compute);
    assert(r::rb(a_filter_1_ct0_d1(),0)==1nat) by (compute);
    assert(raw_filter_1_ct0_d1()[0]==1u8) by (compute);
    assert(r::row_result(raw_filter_1_ct0_d1(),a_filter_1_ct0_d1(),0,0nat)==Ok(()));
    assert(raw_filter_1_ct0_d1()[2]==1u8) by (compute);
    assert(r::row_result(raw_filter_1_ct0_d1(),a_filter_1_ct0_d1(),0,1nat)==Ok(()));
    assert(raw_filter_1_ct0_d1()[4]==1u8) by (compute);
    assert(r::row_result(raw_filter_1_ct0_d1(),a_filter_1_ct0_d1(),0,2nat)==Ok(()));
    assert(raw_filter_1_ct0_d1()[6]==1u8) by (compute);
    assert(r::row_result(raw_filter_1_ct0_d1(),a_filter_1_ct0_d1(),0,3nat)==Ok(()));
    r::pass_step(raw_filter_1_ct0_d1(),a_filter_1_ct0_d1(),0,4nat);
    r::pass_step(raw_filter_1_ct0_d1(),a_filter_1_ct0_d1(),0,3nat);
    r::pass_step(raw_filter_1_ct0_d1(),a_filter_1_ct0_d1(),0,2nat);
    r::pass_step(raw_filter_1_ct0_d1(),a_filter_1_ct0_d1(),0,1nat);
    r::pass_step(raw_filter_1_ct0_d1(),a_filter_1_ct0_d1(),0,0nat);
    r::scan_step(raw_filter_1_ct0_d1(),a_filter_1_ct0_d1(),1);
    r::scan_step(raw_filter_1_ct0_d1(),a_filter_1_ct0_d1(),0);
}
#[verifier::spinoff_prover]
proof fn model_filter_1_ct0_d1()
    ensures r::still(bytes_filter_1_ct0_d1(),STRICT,false)==(r::Projection::Ready {width:6u32,height:4u32,bytes:96nat}),
{
    hide(bytes_filter_1_ct0_d1);hide(z_filter_1_ct0_d1);hide(raw_filter_1_ct0_d1);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_1_ct0_d1().len()==73nat) by (compute);
    assert(crate::model::mh(bytes_filter_1_ct0_d1())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:1u8,color_type:0u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_1_ct0_d1())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:1u8,color_type:0u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 1u8)==2u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_1_ct0_d1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:16nat,next:61nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_1_ct0_d1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:16nat,next:61nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_1_ct0_d1(),61nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:69nat,len:0nat,next:73nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_1_ct0_d1(),61nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:69nat,len:0nat,next:73nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_1_ct0_d1().subrange(41,57)==z_filter_1_ct0_d1()) by (compute);
    walk_filter_1_ct0_d1(bytes_filter_1_ct0_d1());
    inflate_filter_1_ct0_d1();scan_filter_1_ct0_d1();
    assert(r::raw(a_filter_1_ct0_d1())==8nat) by (compute);
    assert(r::data(z_filter_1_ct0_d1(),a_filter_1_ct0_d1(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_filter_1_ct0_d1(),h_filter_1_ct0_d1(),z_filter_1_ct0_d1(),0nat,false);
}
fn public_filter_1_ct0_d1() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,1u8,0u8,0u8,0u8,0u8,133u8,127u8,115u8,238u8,0u8,0u8,0u8,16u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,156u8,195u8,120u8,134u8,241u8,8u8,35u8,15u8,0u8,10u8,180u8,2u8,61u8,133u8,32u8,85u8,115u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_1_ct0_d1());model_filter_1_ct0_d1();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==6u32 && s.height==4u32 && s.rgba@.len()==96nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// filter/1-ct0-d8; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_filter_1_ct0_d8() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,0u8,0u8,0u8,0u8,136u8,111u8,17u8,159u8,0u8,0u8,0u8,37u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,140u8,121u8,90u8,122u8,147u8,37u8,146u8,49u8,89u8,74u8,225u8,248u8,37u8,99u8,198u8,148u8,181u8,239u8,174u8,182u8,156u8,98u8,156u8,181u8,86u8,66u8,246u8,162u8,46u8,0u8,166u8,144u8,11u8,246u8,81u8,75u8,103u8,69u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_filter_1_ct0_d8() -> Seq<u8> {seq![120u8,156u8,99u8,140u8,121u8,90u8,122u8,147u8,37u8,146u8,49u8,89u8,74u8,225u8,248u8,37u8,99u8,198u8,148u8,181u8,239u8,174u8,182u8,156u8,98u8,156u8,181u8,86u8,66u8,246u8,162u8,46u8,0u8,166u8,144u8,11u8,246u8]}
spec fn h_filter_1_ct0_d8() -> crate::model::Ihdr {crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:0u8,interlace:0u8}}
spec fn a_filter_1_ct0_d8() -> r::Raster {r::shape(h_filter_1_ct0_d8(),6nat,4nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_filter_1_ct0_d8(b: Seq<u8>)
    requires b.len()==94,crate::model::mh(b)==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:0u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:37nat,next:82nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,82nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:90nat,len:0nat,next:94nat,crc_ok:true,synthetic:false})),
        b.subrange(41,78)==z_filter_1_ct0_d8(),
    ensures r::default_case(b,crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:0u8,interlace:0u8},z_filter_1_ct0_d8(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_filter_1_ct0_d8() -> Seq<u8> {seq![1u8,92u8,229u8,117u8,217u8,4u8,89u8,1u8,99u8,26u8,32u8,199u8,210u8,51u8,1u8,100u8,173u8,238u8,213u8,132u8,202u8,1u8,154u8,173u8,24u8,29u8,209u8,45u8]}
spec fn ts_filter_1_ct0_d8() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:35nat,before:1nat,after:2nat,symbol:92nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:35nat,next:44nat,before:2nat,after:3nat,symbol:229nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:44nat,next:52nat,before:3nat,after:4nat,symbol:117nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:52nat,next:61nat,before:4nat,after:5nat,symbol:217nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:61nat,next:69nat,before:5nat,after:6nat,symbol:4nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:69nat,next:77nat,before:6nat,after:7nat,symbol:89nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:77nat,next:85nat,before:7nat,after:8nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:85nat,next:93nat,before:8nat,after:9nat,symbol:99nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:93nat,next:101nat,before:9nat,after:10nat,symbol:26nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:101nat,next:109nat,before:10nat,after:11nat,symbol:32nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:109nat,next:118nat,before:11nat,after:12nat,symbol:199nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:118nat,next:127nat,before:12nat,after:13nat,symbol:210nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:127nat,next:135nat,before:13nat,after:14nat,symbol:51nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:135nat,next:143nat,before:14nat,after:15nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:143nat,next:151nat,before:15nat,after:16nat,symbol:100nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:151nat,next:160nat,before:16nat,after:17nat,symbol:173nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:160nat,next:169nat,before:17nat,after:18nat,symbol:238nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:169nat,next:178nat,before:18nat,after:19nat,symbol:213nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:178nat,next:186nat,before:19nat,after:20nat,symbol:132nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:186nat,next:195nat,before:20nat,after:21nat,symbol:202nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:195nat,next:203nat,before:21nat,after:22nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:203nat,next:212nat,before:22nat,after:23nat,symbol:154nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:212nat,next:221nat,before:23nat,after:24nat,symbol:173nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:221nat,next:229nat,before:24nat,after:25nat,symbol:24nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:229nat,next:237nat,before:25nat,after:26nat,symbol:29nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:237nat,next:246nat,before:26nat,after:27nat,symbol:209nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:246nat,next:254nat,before:27nat,after:28nat,symbol:45nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:254nat,next:261nat,before:28nat,after:28nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_filter_1_ct0_d8()
    ensures t::trace(z_filter_1_ct0_d8(),raw_filter_1_ct0_d8(),ts_filter_1_ct0_d8(),0,29nat),
{
    hide(z_filter_1_ct0_d8);hide(raw_filter_1_ct0_d8);hide(ts_filter_1_ct0_d8);hide(t::valid_step);
    assert(t::trace(z_filter_1_ct0_d8(),raw_filter_1_ct0_d8(),ts_filter_1_ct0_d8(),0,29nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_filter_1_ct0_d8()
    ensures raw_filter_1_ct0_d8().len()==28nat,
        z_filter_1_ct0_d8().len()==37nat,
        z_filter_1_ct0_d8()[0]==120u8,
        z_filter_1_ct0_d8()[1]==156u8,
        zm::field_value(z_filter_1_ct0_d8(),16,1)==1nat,
        zm::field_value(z_filter_1_ct0_d8(),17,2)==1nat,
        zm::be_value(z_filter_1_ct0_d8(),33nat,4)==2794458102nat,
        ts_filter_1_ct0_d8().len()==29nat,
        ts_filter_1_ct0_d8()[0].pos==19nat,
        ts_filter_1_ct0_d8()[0].before==0nat,
        ts_filter_1_ct0_d8().last().next==261nat,
{
    hide(z_filter_1_ct0_d8);hide(raw_filter_1_ct0_d8);hide(ts_filter_1_ct0_d8);
    assert(raw_filter_1_ct0_d8().len()==28nat) by (compute);
    assert(z_filter_1_ct0_d8().len()==37nat) by (compute);
    assert(z_filter_1_ct0_d8()[0]==120u8) by (compute);
    assert(z_filter_1_ct0_d8()[1]==156u8) by (compute);
    assert(zm::field_value(z_filter_1_ct0_d8(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_filter_1_ct0_d8(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_filter_1_ct0_d8(),33nat,4)==2794458102nat) by (compute);
    assert(ts_filter_1_ct0_d8().len()==29nat) by (compute);
    assert(ts_filter_1_ct0_d8()[0].pos==19nat) by (compute);
    assert(ts_filter_1_ct0_d8()[0].before==0nat) by (compute);
    assert(ts_filter_1_ct0_d8().last().next==261nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_filter_1_ct0_d8()
    ensures zm::Adler(raw_filter_1_ct0_d8())==2794458102nat,
{
    hide(raw_filter_1_ct0_d8);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_filter_1_ct0_d8().len()==28nat) by (compute);
    assert(c::sums(raw_filter_1_ct0_d8(),0,28nat)==(3061nat,42640nat)) by (compute);
    c::sums_correct(raw_filter_1_ct0_d8(),0,28nat);
    assert(raw_filter_1_ct0_d8().subrange(0,28) =~= raw_filter_1_ct0_d8());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((42640nat%65521)*(u16::MAX as nat+1)+(1+3061nat)%65521==2794458102nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_filter_1_ct0_d8()
    ensures r::inflated(z_filter_1_ct0_d8(),a_filter_1_ct0_d8(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_filter_1_ct0_d8(),r2:false}),
        raw_filter_1_ct0_d8().len()==28nat,
{
    hide(z_filter_1_ct0_d8);hide(raw_filter_1_ct0_d8);hide(ts_filter_1_ct0_d8);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_filter_1_ct0_d8();checksum_filter_1_ct0_d8();
    assert(r::raw(a_filter_1_ct0_d8())==28nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_filter_1_ct0_d8();
    stream::fixed_stream(z_filter_1_ct0_d8(),raw_filter_1_ct0_d8(),ts_filter_1_ct0_d8(),28nat,156nat,2794458102nat);
}
#[verifier::spinoff_prover]
proof fn scan_filter_1_ct0_d8()
    ensures r::scan(raw_filter_1_ct0_d8(),a_filter_1_ct0_d8(),0)==Ok(()),
{
    hide(raw_filter_1_ct0_d8);
    assert(r::base(a_filter_1_ct0_d8(),0)==0nat && r::ph(a_filter_1_ct0_d8(),0)==4nat && r::count(a_filter_1_ct0_d8())==1nat) by (compute);
    assert(r::rb(a_filter_1_ct0_d8(),0)==6nat) by (compute);
    assert(raw_filter_1_ct0_d8()[0]==1u8) by (compute);
    assert(r::row_result(raw_filter_1_ct0_d8(),a_filter_1_ct0_d8(),0,0nat)==Ok(()));
    assert(raw_filter_1_ct0_d8()[7]==1u8) by (compute);
    assert(r::row_result(raw_filter_1_ct0_d8(),a_filter_1_ct0_d8(),0,1nat)==Ok(()));
    assert(raw_filter_1_ct0_d8()[14]==1u8) by (compute);
    assert(r::row_result(raw_filter_1_ct0_d8(),a_filter_1_ct0_d8(),0,2nat)==Ok(()));
    assert(raw_filter_1_ct0_d8()[21]==1u8) by (compute);
    assert(r::row_result(raw_filter_1_ct0_d8(),a_filter_1_ct0_d8(),0,3nat)==Ok(()));
    r::pass_step(raw_filter_1_ct0_d8(),a_filter_1_ct0_d8(),0,4nat);
    r::pass_step(raw_filter_1_ct0_d8(),a_filter_1_ct0_d8(),0,3nat);
    r::pass_step(raw_filter_1_ct0_d8(),a_filter_1_ct0_d8(),0,2nat);
    r::pass_step(raw_filter_1_ct0_d8(),a_filter_1_ct0_d8(),0,1nat);
    r::pass_step(raw_filter_1_ct0_d8(),a_filter_1_ct0_d8(),0,0nat);
    r::scan_step(raw_filter_1_ct0_d8(),a_filter_1_ct0_d8(),1);
    r::scan_step(raw_filter_1_ct0_d8(),a_filter_1_ct0_d8(),0);
}
#[verifier::spinoff_prover]
proof fn model_filter_1_ct0_d8()
    ensures r::still(bytes_filter_1_ct0_d8(),STRICT,false)==(r::Projection::Ready {width:6u32,height:4u32,bytes:96nat}),
{
    hide(bytes_filter_1_ct0_d8);hide(z_filter_1_ct0_d8);hide(raw_filter_1_ct0_d8);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_1_ct0_d8().len()==94nat) by (compute);
    assert(crate::model::mh(bytes_filter_1_ct0_d8())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:0u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_1_ct0_d8())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:0u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_1_ct0_d8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:37nat,next:82nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_1_ct0_d8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:37nat,next:82nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_1_ct0_d8(),82nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:90nat,len:0nat,next:94nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_1_ct0_d8(),82nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:90nat,len:0nat,next:94nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_1_ct0_d8().subrange(41,78)==z_filter_1_ct0_d8()) by (compute);
    walk_filter_1_ct0_d8(bytes_filter_1_ct0_d8());
    inflate_filter_1_ct0_d8();scan_filter_1_ct0_d8();
    assert(r::raw(a_filter_1_ct0_d8())==28nat) by (compute);
    assert(r::data(z_filter_1_ct0_d8(),a_filter_1_ct0_d8(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_filter_1_ct0_d8(),h_filter_1_ct0_d8(),z_filter_1_ct0_d8(),0nat,false);
}
fn public_filter_1_ct0_d8() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,0u8,0u8,0u8,0u8,136u8,111u8,17u8,159u8,0u8,0u8,0u8,37u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,140u8,121u8,90u8,122u8,147u8,37u8,146u8,49u8,89u8,74u8,225u8,248u8,37u8,99u8,198u8,148u8,181u8,239u8,174u8,182u8,156u8,98u8,156u8,181u8,86u8,66u8,246u8,162u8,46u8,0u8,166u8,144u8,11u8,246u8,81u8,75u8,103u8,69u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_1_ct0_d8());model_filter_1_ct0_d8();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==6u32 && s.height==4u32 && s.rgba@.len()==96nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// filter/1-ct0-d16; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_filter_1_ct0_d16() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,0u8,0u8,0u8,0u8,216u8,255u8,205u8,220u8,0u8,0u8,0u8,63u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,52u8,0u8,203u8,255u8,1u8,194u8,130u8,61u8,111u8,133u8,138u8,14u8,53u8,85u8,85u8,31u8,67u8,1u8,105u8,187u8,5u8,126u8,32u8,179u8,77u8,234u8,8u8,164u8,166u8,117u8,1u8,13u8,188u8,51u8,77u8,175u8,219u8,7u8,112u8,192u8,166u8,246u8,209u8,1u8,190u8,182u8,76u8,222u8,24u8,244u8,150u8,148u8,0u8,116u8,45u8,30u8,27u8,183u8,21u8,213u8,163u8,179u8,160u8,217u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_filter_1_ct0_d16() -> Seq<u8> {seq![120u8,156u8,1u8,52u8,0u8,203u8,255u8,1u8,194u8,130u8,61u8,111u8,133u8,138u8,14u8,53u8,85u8,85u8,31u8,67u8,1u8,105u8,187u8,5u8,126u8,32u8,179u8,77u8,234u8,8u8,164u8,166u8,117u8,1u8,13u8,188u8,51u8,77u8,175u8,219u8,7u8,112u8,192u8,166u8,246u8,209u8,1u8,190u8,182u8,76u8,222u8,24u8,244u8,150u8,148u8,0u8,116u8,45u8,30u8,27u8,183u8,21u8,213u8]}
spec fn h_filter_1_ct0_d16() -> crate::model::Ihdr {crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:0u8,interlace:0u8}}
spec fn a_filter_1_ct0_d16() -> r::Raster {r::shape(h_filter_1_ct0_d16(),6nat,4nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_filter_1_ct0_d16(b: Seq<u8>)
    requires b.len()==120,crate::model::mh(b)==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:0u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:63nat,next:108nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,108nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:116nat,len:0nat,next:120nat,crc_ok:true,synthetic:false})),
        b.subrange(41,104)==z_filter_1_ct0_d16(),
    ensures r::default_case(b,crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:0u8,interlace:0u8},z_filter_1_ct0_d16(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
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
spec fn raw_filter_1_ct0_d16() -> Seq<u8> {seq![1u8,194u8,130u8,61u8,111u8,133u8,138u8,14u8,53u8,85u8,85u8,31u8,67u8,1u8,105u8,187u8,5u8,126u8,32u8,179u8,77u8,234u8,8u8,164u8,166u8,117u8,1u8,13u8,188u8,51u8,77u8,175u8,219u8,7u8,112u8,192u8,166u8,246u8,209u8,1u8,190u8,182u8,76u8,222u8,24u8,244u8,150u8,148u8,0u8,116u8,45u8,30u8]}
#[verifier::spinoff_prover]
proof fn shape_filter_1_ct0_d16()
    ensures raw_filter_1_ct0_d16().len()==52nat,
        z_filter_1_ct0_d16().len()==63nat,
        z_filter_1_ct0_d16()[0]==120u8,
        z_filter_1_ct0_d16()[1]==156u8,
        zm::field_value(z_filter_1_ct0_d16(),16,1)==1nat,
        zm::field_value(z_filter_1_ct0_d16(),17,2)==0nat,
        zm::be_value(z_filter_1_ct0_d16(),59nat,4)==464983509nat,
        z_filter_1_ct0_d16().subrange(7,59)==raw_filter_1_ct0_d16(),
        z_filter_1_ct0_d16()[3] as nat+256*z_filter_1_ct0_d16()[4] as nat==52nat,
        z_filter_1_ct0_d16()[5] as nat+256*z_filter_1_ct0_d16()[6] as nat==u16::MAX as nat-52nat,
        zm::be_value(z_filter_1_ct0_d16(),0,1)==120nat,
        zm::be_value(z_filter_1_ct0_d16(),1,1)==156nat,
{
    hide(z_filter_1_ct0_d16);hide(raw_filter_1_ct0_d16);
    assert(raw_filter_1_ct0_d16().len()==52nat) by (compute);
    assert(z_filter_1_ct0_d16().len()==63nat) by (compute);
    assert(z_filter_1_ct0_d16()[0]==120u8) by (compute);
    assert(z_filter_1_ct0_d16()[1]==156u8) by (compute);
    assert(zm::field_value(z_filter_1_ct0_d16(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_filter_1_ct0_d16(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_filter_1_ct0_d16(),59nat,4)==464983509nat) by (compute);
    assert(z_filter_1_ct0_d16().subrange(7,59)==raw_filter_1_ct0_d16()) by (compute);
    assert(z_filter_1_ct0_d16()[3] as nat+256*z_filter_1_ct0_d16()[4] as nat==52nat) by (compute);
    assert(z_filter_1_ct0_d16()[5] as nat+256*z_filter_1_ct0_d16()[6] as nat==u16::MAX as nat-52nat) by (compute);
    assert(zm::be_value(z_filter_1_ct0_d16(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_filter_1_ct0_d16(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_filter_1_ct0_d16()
    ensures zm::Adler(raw_filter_1_ct0_d16())==464983509nat,
{
    hide(raw_filter_1_ct0_d16);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_filter_1_ct0_d16().len()==52nat) by (compute);
    assert(c::sums(raw_filter_1_ct0_d16(),0,52nat)==(5588nat,138137nat)) by (compute);
    c::sums_correct(raw_filter_1_ct0_d16(),0,52nat);
    assert(raw_filter_1_ct0_d16().subrange(0,52) =~= raw_filter_1_ct0_d16());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((138137nat%65521)*(u16::MAX as nat+1)+(1+5588nat)%65521==464983509nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_filter_1_ct0_d16()
    ensures r::inflated(z_filter_1_ct0_d16(),a_filter_1_ct0_d16(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_filter_1_ct0_d16(),r2:false}),
        raw_filter_1_ct0_d16().len()==52nat,
{
    hide(z_filter_1_ct0_d16);hide(raw_filter_1_ct0_d16);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_filter_1_ct0_d16();checksum_filter_1_ct0_d16();
    assert(r::raw(a_filter_1_ct0_d16())==52nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_filter_1_ct0_d16(),raw_filter_1_ct0_d16(),52nat,120nat,156nat,464983509nat);
}
#[verifier::spinoff_prover]
proof fn scan_filter_1_ct0_d16()
    ensures r::scan(raw_filter_1_ct0_d16(),a_filter_1_ct0_d16(),0)==Ok(()),
{
    hide(raw_filter_1_ct0_d16);
    assert(r::base(a_filter_1_ct0_d16(),0)==0nat && r::ph(a_filter_1_ct0_d16(),0)==4nat && r::count(a_filter_1_ct0_d16())==1nat) by (compute);
    assert(r::rb(a_filter_1_ct0_d16(),0)==12nat) by (compute);
    assert(raw_filter_1_ct0_d16()[0]==1u8) by (compute);
    assert(r::row_result(raw_filter_1_ct0_d16(),a_filter_1_ct0_d16(),0,0nat)==Ok(()));
    assert(raw_filter_1_ct0_d16()[13]==1u8) by (compute);
    assert(r::row_result(raw_filter_1_ct0_d16(),a_filter_1_ct0_d16(),0,1nat)==Ok(()));
    assert(raw_filter_1_ct0_d16()[26]==1u8) by (compute);
    assert(r::row_result(raw_filter_1_ct0_d16(),a_filter_1_ct0_d16(),0,2nat)==Ok(()));
    assert(raw_filter_1_ct0_d16()[39]==1u8) by (compute);
    assert(r::row_result(raw_filter_1_ct0_d16(),a_filter_1_ct0_d16(),0,3nat)==Ok(()));
    r::pass_step(raw_filter_1_ct0_d16(),a_filter_1_ct0_d16(),0,4nat);
    r::pass_step(raw_filter_1_ct0_d16(),a_filter_1_ct0_d16(),0,3nat);
    r::pass_step(raw_filter_1_ct0_d16(),a_filter_1_ct0_d16(),0,2nat);
    r::pass_step(raw_filter_1_ct0_d16(),a_filter_1_ct0_d16(),0,1nat);
    r::pass_step(raw_filter_1_ct0_d16(),a_filter_1_ct0_d16(),0,0nat);
    r::scan_step(raw_filter_1_ct0_d16(),a_filter_1_ct0_d16(),1);
    r::scan_step(raw_filter_1_ct0_d16(),a_filter_1_ct0_d16(),0);
}
#[verifier::spinoff_prover]
proof fn model_filter_1_ct0_d16()
    ensures r::still(bytes_filter_1_ct0_d16(),STRICT,false)==(r::Projection::Ready {width:6u32,height:4u32,bytes:96nat}),
{
    hide(bytes_filter_1_ct0_d16);hide(z_filter_1_ct0_d16);hide(raw_filter_1_ct0_d16);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_1_ct0_d16().len()==120nat) by (compute);
    assert(crate::model::mh(bytes_filter_1_ct0_d16())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:0u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_1_ct0_d16())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:0u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_1_ct0_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:63nat,next:108nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_1_ct0_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:63nat,next:108nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_1_ct0_d16(),108nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:116nat,len:0nat,next:120nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_1_ct0_d16(),108nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:116nat,len:0nat,next:120nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_1_ct0_d16().subrange(41,104)==z_filter_1_ct0_d16()) by (compute);
    walk_filter_1_ct0_d16(bytes_filter_1_ct0_d16());
    inflate_filter_1_ct0_d16();scan_filter_1_ct0_d16();
    assert(r::raw(a_filter_1_ct0_d16())==52nat) by (compute);
    assert(r::data(z_filter_1_ct0_d16(),a_filter_1_ct0_d16(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_filter_1_ct0_d16(),h_filter_1_ct0_d16(),z_filter_1_ct0_d16(),0nat,false);
}
fn public_filter_1_ct0_d16() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,0u8,0u8,0u8,0u8,216u8,255u8,205u8,220u8,0u8,0u8,0u8,63u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,52u8,0u8,203u8,255u8,1u8,194u8,130u8,61u8,111u8,133u8,138u8,14u8,53u8,85u8,85u8,31u8,67u8,1u8,105u8,187u8,5u8,126u8,32u8,179u8,77u8,234u8,8u8,164u8,166u8,117u8,1u8,13u8,188u8,51u8,77u8,175u8,219u8,7u8,112u8,192u8,166u8,246u8,209u8,1u8,190u8,182u8,76u8,222u8,24u8,244u8,150u8,148u8,0u8,116u8,45u8,30u8,27u8,183u8,21u8,213u8,163u8,179u8,160u8,217u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_1_ct0_d16());model_filter_1_ct0_d16();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==6u32 && s.height==4u32 && s.rgba@.len()==96nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// filter/1-ct2-d8; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_filter_1_ct2_d8() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,2u8,0u8,0u8,0u8,34u8,102u8,217u8,20u8,0u8,0u8,0u8,87u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,76u8,0u8,179u8,255u8,1u8,127u8,211u8,204u8,60u8,110u8,171u8,88u8,133u8,21u8,208u8,113u8,32u8,41u8,70u8,245u8,96u8,167u8,87u8,1u8,206u8,63u8,161u8,59u8,177u8,55u8,178u8,36u8,42u8,95u8,243u8,107u8,138u8,254u8,96u8,242u8,214u8,240u8,1u8,33u8,15u8,61u8,83u8,128u8,176u8,199u8,211u8,54u8,91u8,202u8,116u8,143u8,69u8,66u8,18u8,37u8,10u8,1u8,208u8,98u8,169u8,157u8,169u8,105u8,69u8,130u8,203u8,29u8,30u8,203u8,173u8,169u8,33u8,228u8,27u8,154u8,50u8,96u8,34u8,157u8,121u8,4u8,237u8,249u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_filter_1_ct2_d8() -> Seq<u8> {seq![120u8,156u8,1u8,76u8,0u8,179u8,255u8,1u8,127u8,211u8,204u8,60u8,110u8,171u8,88u8,133u8,21u8,208u8,113u8,32u8,41u8,70u8,245u8,96u8,167u8,87u8,1u8,206u8,63u8,161u8,59u8,177u8,55u8,178u8,36u8,42u8,95u8,243u8,107u8,138u8,254u8,96u8,242u8,214u8,240u8,1u8,33u8,15u8,61u8,83u8,128u8,176u8,199u8,211u8,54u8,91u8,202u8,116u8,143u8,69u8,66u8,18u8,37u8,10u8,1u8,208u8,98u8,169u8,157u8,169u8,105u8,69u8,130u8,203u8,29u8,30u8,203u8,173u8,169u8,33u8,228u8,27u8,154u8,50u8,96u8,34u8,157u8]}
spec fn h_filter_1_ct2_d8() -> crate::model::Ihdr {crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:2u8,interlace:0u8}}
spec fn a_filter_1_ct2_d8() -> r::Raster {r::shape(h_filter_1_ct2_d8(),6nat,4nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_filter_1_ct2_d8(b: Seq<u8>)
    requires b.len()==144,crate::model::mh(b)==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:2u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:87nat,next:132nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,132nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:140nat,len:0nat,next:144nat,crc_ok:true,synthetic:false})),
        b.subrange(41,128)==z_filter_1_ct2_d8(),
    ensures r::default_case(b,crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:2u8,interlace:0u8},z_filter_1_ct2_d8(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_filter_1_ct2_d8() -> Seq<u8> {seq![1u8,127u8,211u8,204u8,60u8,110u8,171u8,88u8,133u8,21u8,208u8,113u8,32u8,41u8,70u8,245u8,96u8,167u8,87u8,1u8,206u8,63u8,161u8,59u8,177u8,55u8,178u8,36u8,42u8,95u8,243u8,107u8,138u8,254u8,96u8,242u8,214u8,240u8,1u8,33u8,15u8,61u8,83u8,128u8,176u8,199u8,211u8,54u8,91u8,202u8,116u8,143u8,69u8,66u8,18u8,37u8,10u8,1u8,208u8,98u8,169u8,157u8,169u8,105u8,69u8,130u8,203u8,29u8,30u8,203u8,173u8,169u8,33u8,228u8,27u8,154u8]}
#[verifier::spinoff_prover]
proof fn shape_filter_1_ct2_d8()
    ensures raw_filter_1_ct2_d8().len()==76nat,
        z_filter_1_ct2_d8().len()==87nat,
        z_filter_1_ct2_d8()[0]==120u8,
        z_filter_1_ct2_d8()[1]==156u8,
        zm::field_value(z_filter_1_ct2_d8(),16,1)==1nat,
        zm::field_value(z_filter_1_ct2_d8(),17,2)==0nat,
        zm::be_value(z_filter_1_ct2_d8(),83nat,4)==845161117nat,
        z_filter_1_ct2_d8().subrange(7,83)==raw_filter_1_ct2_d8(),
        z_filter_1_ct2_d8()[3] as nat+256*z_filter_1_ct2_d8()[4] as nat==76nat,
        z_filter_1_ct2_d8()[5] as nat+256*z_filter_1_ct2_d8()[6] as nat==u16::MAX as nat-76nat,
        zm::be_value(z_filter_1_ct2_d8(),0,1)==120nat,
        zm::be_value(z_filter_1_ct2_d8(),1,1)==156nat,
{
    hide(z_filter_1_ct2_d8);hide(raw_filter_1_ct2_d8);
    assert(raw_filter_1_ct2_d8().len()==76nat) by (compute);
    assert(z_filter_1_ct2_d8().len()==87nat) by (compute);
    assert(z_filter_1_ct2_d8()[0]==120u8) by (compute);
    assert(z_filter_1_ct2_d8()[1]==156u8) by (compute);
    assert(zm::field_value(z_filter_1_ct2_d8(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_filter_1_ct2_d8(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_filter_1_ct2_d8(),83nat,4)==845161117nat) by (compute);
    assert(z_filter_1_ct2_d8().subrange(7,83)==raw_filter_1_ct2_d8()) by (compute);
    assert(z_filter_1_ct2_d8()[3] as nat+256*z_filter_1_ct2_d8()[4] as nat==76nat) by (compute);
    assert(z_filter_1_ct2_d8()[5] as nat+256*z_filter_1_ct2_d8()[6] as nat==u16::MAX as nat-76nat) by (compute);
    assert(zm::be_value(z_filter_1_ct2_d8(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_filter_1_ct2_d8(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_filter_1_ct2_d8()
    ensures zm::Adler(raw_filter_1_ct2_d8())==845161117nat,
{
    hide(raw_filter_1_ct2_d8);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_filter_1_ct2_d8().len()==76nat) by (compute);
    assert(c::sums(raw_filter_1_ct2_d8(),0,76nat)==(8860nat,340501nat)) by (compute);
    c::sums_correct(raw_filter_1_ct2_d8(),0,76nat);
    assert(raw_filter_1_ct2_d8().subrange(0,76) =~= raw_filter_1_ct2_d8());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((340501nat%65521)*(u16::MAX as nat+1)+(1+8860nat)%65521==845161117nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_filter_1_ct2_d8()
    ensures r::inflated(z_filter_1_ct2_d8(),a_filter_1_ct2_d8(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_filter_1_ct2_d8(),r2:false}),
        raw_filter_1_ct2_d8().len()==76nat,
{
    hide(z_filter_1_ct2_d8);hide(raw_filter_1_ct2_d8);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_filter_1_ct2_d8();checksum_filter_1_ct2_d8();
    assert(r::raw(a_filter_1_ct2_d8())==76nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_filter_1_ct2_d8(),raw_filter_1_ct2_d8(),76nat,120nat,156nat,845161117nat);
}
#[verifier::spinoff_prover]
proof fn scan_filter_1_ct2_d8()
    ensures r::scan(raw_filter_1_ct2_d8(),a_filter_1_ct2_d8(),0)==Ok(()),
{
    hide(raw_filter_1_ct2_d8);
    assert(r::base(a_filter_1_ct2_d8(),0)==0nat && r::ph(a_filter_1_ct2_d8(),0)==4nat && r::count(a_filter_1_ct2_d8())==1nat) by (compute);
    assert(r::rb(a_filter_1_ct2_d8(),0)==18nat) by (compute);
    assert(raw_filter_1_ct2_d8()[0]==1u8) by (compute);
    assert(r::row_result(raw_filter_1_ct2_d8(),a_filter_1_ct2_d8(),0,0nat)==Ok(()));
    assert(raw_filter_1_ct2_d8()[19]==1u8) by (compute);
    assert(r::row_result(raw_filter_1_ct2_d8(),a_filter_1_ct2_d8(),0,1nat)==Ok(()));
    assert(raw_filter_1_ct2_d8()[38]==1u8) by (compute);
    assert(r::row_result(raw_filter_1_ct2_d8(),a_filter_1_ct2_d8(),0,2nat)==Ok(()));
    assert(raw_filter_1_ct2_d8()[57]==1u8) by (compute);
    assert(r::row_result(raw_filter_1_ct2_d8(),a_filter_1_ct2_d8(),0,3nat)==Ok(()));
    r::pass_step(raw_filter_1_ct2_d8(),a_filter_1_ct2_d8(),0,4nat);
    r::pass_step(raw_filter_1_ct2_d8(),a_filter_1_ct2_d8(),0,3nat);
    r::pass_step(raw_filter_1_ct2_d8(),a_filter_1_ct2_d8(),0,2nat);
    r::pass_step(raw_filter_1_ct2_d8(),a_filter_1_ct2_d8(),0,1nat);
    r::pass_step(raw_filter_1_ct2_d8(),a_filter_1_ct2_d8(),0,0nat);
    r::scan_step(raw_filter_1_ct2_d8(),a_filter_1_ct2_d8(),1);
    r::scan_step(raw_filter_1_ct2_d8(),a_filter_1_ct2_d8(),0);
}
#[verifier::spinoff_prover]
proof fn model_filter_1_ct2_d8()
    ensures r::still(bytes_filter_1_ct2_d8(),STRICT,false)==(r::Projection::Ready {width:6u32,height:4u32,bytes:96nat}),
{
    hide(bytes_filter_1_ct2_d8);hide(z_filter_1_ct2_d8);hide(raw_filter_1_ct2_d8);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_1_ct2_d8().len()==144nat) by (compute);
    assert(crate::model::mh(bytes_filter_1_ct2_d8())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:2u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_1_ct2_d8())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:2u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_1_ct2_d8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:87nat,next:132nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_1_ct2_d8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:87nat,next:132nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_1_ct2_d8(),132nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:140nat,len:0nat,next:144nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_1_ct2_d8(),132nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:140nat,len:0nat,next:144nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_1_ct2_d8().subrange(41,128)==z_filter_1_ct2_d8()) by (compute);
    walk_filter_1_ct2_d8(bytes_filter_1_ct2_d8());
    inflate_filter_1_ct2_d8();scan_filter_1_ct2_d8();
    assert(r::raw(a_filter_1_ct2_d8())==76nat) by (compute);
    assert(r::data(z_filter_1_ct2_d8(),a_filter_1_ct2_d8(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_filter_1_ct2_d8(),h_filter_1_ct2_d8(),z_filter_1_ct2_d8(),0nat,false);
}
fn public_filter_1_ct2_d8() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,2u8,0u8,0u8,0u8,34u8,102u8,217u8,20u8,0u8,0u8,0u8,87u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,76u8,0u8,179u8,255u8,1u8,127u8,211u8,204u8,60u8,110u8,171u8,88u8,133u8,21u8,208u8,113u8,32u8,41u8,70u8,245u8,96u8,167u8,87u8,1u8,206u8,63u8,161u8,59u8,177u8,55u8,178u8,36u8,42u8,95u8,243u8,107u8,138u8,254u8,96u8,242u8,214u8,240u8,1u8,33u8,15u8,61u8,83u8,128u8,176u8,199u8,211u8,54u8,91u8,202u8,116u8,143u8,69u8,66u8,18u8,37u8,10u8,1u8,208u8,98u8,169u8,157u8,169u8,105u8,69u8,130u8,203u8,29u8,30u8,203u8,173u8,169u8,33u8,228u8,27u8,154u8,50u8,96u8,34u8,157u8,121u8,4u8,237u8,249u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_1_ct2_d8());model_filter_1_ct2_d8();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==6u32 && s.height==4u32 && s.rgba@.len()==96nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// filter/1-ct2-d16; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_filter_1_ct2_d16() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,2u8,0u8,0u8,0u8,114u8,246u8,5u8,87u8,0u8,0u8,0u8,159u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,148u8,0u8,107u8,255u8,1u8,187u8,124u8,42u8,162u8,35u8,240u8,233u8,237u8,242u8,101u8,244u8,195u8,239u8,135u8,138u8,250u8,252u8,199u8,66u8,131u8,24u8,137u8,4u8,1u8,186u8,91u8,22u8,119u8,228u8,170u8,252u8,237u8,42u8,17u8,11u8,83u8,1u8,229u8,248u8,175u8,246u8,38u8,69u8,68u8,29u8,202u8,234u8,36u8,215u8,17u8,29u8,59u8,134u8,16u8,198u8,190u8,189u8,188u8,91u8,1u8,247u8,88u8,129u8,166u8,82u8,214u8,222u8,67u8,59u8,234u8,1u8,93u8,51u8,1u8,169u8,193u8,72u8,67u8,17u8,38u8,226u8,28u8,12u8,245u8,65u8,153u8,62u8,60u8,10u8,145u8,128u8,71u8,127u8,142u8,255u8,56u8,241u8,199u8,179u8,128u8,118u8,154u8,110u8,254u8,7u8,241u8,249u8,220u8,97u8,225u8,1u8,203u8,119u8,143u8,123u8,197u8,202u8,70u8,227u8,220u8,63u8,216u8,52u8,163u8,93u8,195u8,116u8,215u8,234u8,39u8,102u8,68u8,27u8,125u8,105u8,19u8,16u8,216u8,241u8,210u8,191u8,184u8,192u8,138u8,23u8,91u8,13u8,3u8,17u8,75u8,49u8,171u8,174u8,64u8,54u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_filter_1_ct2_d16() -> Seq<u8> {seq![120u8,156u8,1u8,148u8,0u8,107u8,255u8,1u8,187u8,124u8,42u8,162u8,35u8,240u8,233u8,237u8,242u8,101u8,244u8,195u8,239u8,135u8,138u8,250u8,252u8,199u8,66u8,131u8,24u8,137u8,4u8,1u8,186u8,91u8,22u8,119u8,228u8,170u8,252u8,237u8,42u8,17u8,11u8,83u8,1u8,229u8,248u8,175u8,246u8,38u8,69u8,68u8,29u8,202u8,234u8,36u8,215u8,17u8,29u8,59u8,134u8,16u8,198u8,190u8,189u8,188u8,91u8,1u8,247u8,88u8,129u8,166u8,82u8,214u8,222u8,67u8,59u8,234u8,1u8,93u8,51u8,1u8,169u8,193u8,72u8,67u8,17u8,38u8,226u8,28u8,12u8,245u8,65u8,153u8,62u8,60u8,10u8,145u8,128u8,71u8,127u8,142u8,255u8,56u8,241u8,199u8,179u8,128u8,118u8,154u8,110u8,254u8,7u8,241u8,249u8,220u8,97u8,225u8,1u8,203u8,119u8,143u8,123u8,197u8,202u8,70u8,227u8,220u8,63u8,216u8,52u8,163u8,93u8,195u8,116u8,215u8,234u8,39u8,102u8,68u8,27u8,125u8,105u8,19u8,16u8,216u8,241u8,210u8,191u8,184u8,192u8,138u8,23u8,91u8,13u8,3u8,17u8,75u8,49u8]}
spec fn h_filter_1_ct2_d16() -> crate::model::Ihdr {crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:2u8,interlace:0u8}}
spec fn a_filter_1_ct2_d16() -> r::Raster {r::shape(h_filter_1_ct2_d16(),6nat,4nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_filter_1_ct2_d16(b: Seq<u8>)
    requires b.len()==216,crate::model::mh(b)==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:2u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:159nat,next:204nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,204nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:212nat,len:0nat,next:216nat,crc_ok:true,synthetic:false})),
        b.subrange(41,200)==z_filter_1_ct2_d16(),
    ensures r::default_case(b,crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:2u8,interlace:0u8},z_filter_1_ct2_d16(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
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
spec fn raw_filter_1_ct2_d16() -> Seq<u8> {seq![1u8,187u8,124u8,42u8,162u8,35u8,240u8,233u8,237u8,242u8,101u8,244u8,195u8,239u8,135u8,138u8,250u8,252u8,199u8,66u8,131u8,24u8,137u8,4u8,1u8,186u8,91u8,22u8,119u8,228u8,170u8,252u8,237u8,42u8,17u8,11u8,83u8,1u8,229u8,248u8,175u8,246u8,38u8,69u8,68u8,29u8,202u8,234u8,36u8,215u8,17u8,29u8,59u8,134u8,16u8,198u8,190u8,189u8,188u8,91u8,1u8,247u8,88u8,129u8,166u8,82u8,214u8,222u8,67u8,59u8,234u8,1u8,93u8,51u8,1u8,169u8,193u8,72u8,67u8,17u8,38u8,226u8,28u8,12u8,245u8,65u8,153u8,62u8,60u8,10u8,145u8,128u8,71u8,127u8,142u8,255u8,56u8,241u8,199u8,179u8,128u8,118u8,154u8,110u8,254u8,7u8,241u8,249u8,220u8,97u8,225u8,1u8,203u8,119u8,143u8,123u8,197u8,202u8,70u8,227u8,220u8,63u8,216u8,52u8,163u8,93u8,195u8,116u8,215u8,234u8,39u8,102u8,68u8,27u8,125u8,105u8,19u8,16u8,216u8,241u8,210u8,191u8,184u8,192u8,138u8,23u8,91u8,13u8]}
#[verifier::spinoff_prover]
proof fn shape_filter_1_ct2_d16()
    ensures raw_filter_1_ct2_d16().len()==148nat,
        z_filter_1_ct2_d16().len()==159nat,
        z_filter_1_ct2_d16()[0]==120u8,
        z_filter_1_ct2_d16()[1]==156u8,
        zm::field_value(z_filter_1_ct2_d16(),16,1)==1nat,
        zm::field_value(z_filter_1_ct2_d16(),17,2)==0nat,
        zm::be_value(z_filter_1_ct2_d16(),155nat,4)==51465009nat,
        z_filter_1_ct2_d16().subrange(7,155)==raw_filter_1_ct2_d16(),
        z_filter_1_ct2_d16()[3] as nat+256*z_filter_1_ct2_d16()[4] as nat==148nat,
        z_filter_1_ct2_d16()[5] as nat+256*z_filter_1_ct2_d16()[6] as nat==u16::MAX as nat-148nat,
        zm::be_value(z_filter_1_ct2_d16(),0,1)==120nat,
        zm::be_value(z_filter_1_ct2_d16(),1,1)==156nat,
{
    hide(z_filter_1_ct2_d16);hide(raw_filter_1_ct2_d16);
    assert(raw_filter_1_ct2_d16().len()==148nat) by (compute);
    assert(z_filter_1_ct2_d16().len()==159nat) by (compute);
    assert(z_filter_1_ct2_d16()[0]==120u8) by (compute);
    assert(z_filter_1_ct2_d16()[1]==156u8) by (compute);
    assert(zm::field_value(z_filter_1_ct2_d16(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_filter_1_ct2_d16(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_filter_1_ct2_d16(),155nat,4)==51465009nat) by (compute);
    assert(z_filter_1_ct2_d16().subrange(7,155)==raw_filter_1_ct2_d16()) by (compute);
    assert(z_filter_1_ct2_d16()[3] as nat+256*z_filter_1_ct2_d16()[4] as nat==148nat) by (compute);
    assert(z_filter_1_ct2_d16()[5] as nat+256*z_filter_1_ct2_d16()[6] as nat==u16::MAX as nat-148nat) by (compute);
    assert(zm::be_value(z_filter_1_ct2_d16(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_filter_1_ct2_d16(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_filter_1_ct2_d16()
    ensures zm::Adler(raw_filter_1_ct2_d16())==51465009nat,
{
    hide(raw_filter_1_ct2_d16);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_filter_1_ct2_d16().len()==148nat) by (compute);
    assert(c::sums(raw_filter_1_ct2_d16(),0,148nat)==(19248nat,1442247nat)) by (compute);
    c::sums_correct(raw_filter_1_ct2_d16(),0,148nat);
    assert(raw_filter_1_ct2_d16().subrange(0,148) =~= raw_filter_1_ct2_d16());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((1442247nat%65521)*(u16::MAX as nat+1)+(1+19248nat)%65521==51465009nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_filter_1_ct2_d16()
    ensures r::inflated(z_filter_1_ct2_d16(),a_filter_1_ct2_d16(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_filter_1_ct2_d16(),r2:false}),
        raw_filter_1_ct2_d16().len()==148nat,
{
    hide(z_filter_1_ct2_d16);hide(raw_filter_1_ct2_d16);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_filter_1_ct2_d16();checksum_filter_1_ct2_d16();
    assert(r::raw(a_filter_1_ct2_d16())==148nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_filter_1_ct2_d16(),raw_filter_1_ct2_d16(),148nat,120nat,156nat,51465009nat);
}
#[verifier::spinoff_prover]
proof fn scan_filter_1_ct2_d16()
    ensures r::scan(raw_filter_1_ct2_d16(),a_filter_1_ct2_d16(),0)==Ok(()),
{
    hide(raw_filter_1_ct2_d16);
    assert(r::base(a_filter_1_ct2_d16(),0)==0nat && r::ph(a_filter_1_ct2_d16(),0)==4nat && r::count(a_filter_1_ct2_d16())==1nat) by (compute);
    assert(r::rb(a_filter_1_ct2_d16(),0)==36nat) by (compute);
    assert(raw_filter_1_ct2_d16()[0]==1u8) by (compute);
    assert(r::row_result(raw_filter_1_ct2_d16(),a_filter_1_ct2_d16(),0,0nat)==Ok(()));
    assert(raw_filter_1_ct2_d16()[37]==1u8) by (compute);
    assert(r::row_result(raw_filter_1_ct2_d16(),a_filter_1_ct2_d16(),0,1nat)==Ok(()));
    assert(raw_filter_1_ct2_d16()[74]==1u8) by (compute);
    assert(r::row_result(raw_filter_1_ct2_d16(),a_filter_1_ct2_d16(),0,2nat)==Ok(()));
    assert(raw_filter_1_ct2_d16()[111]==1u8) by (compute);
    assert(r::row_result(raw_filter_1_ct2_d16(),a_filter_1_ct2_d16(),0,3nat)==Ok(()));
    r::pass_step(raw_filter_1_ct2_d16(),a_filter_1_ct2_d16(),0,4nat);
    r::pass_step(raw_filter_1_ct2_d16(),a_filter_1_ct2_d16(),0,3nat);
    r::pass_step(raw_filter_1_ct2_d16(),a_filter_1_ct2_d16(),0,2nat);
    r::pass_step(raw_filter_1_ct2_d16(),a_filter_1_ct2_d16(),0,1nat);
    r::pass_step(raw_filter_1_ct2_d16(),a_filter_1_ct2_d16(),0,0nat);
    r::scan_step(raw_filter_1_ct2_d16(),a_filter_1_ct2_d16(),1);
    r::scan_step(raw_filter_1_ct2_d16(),a_filter_1_ct2_d16(),0);
}
#[verifier::spinoff_prover]
proof fn model_filter_1_ct2_d16()
    ensures r::still(bytes_filter_1_ct2_d16(),STRICT,false)==(r::Projection::Ready {width:6u32,height:4u32,bytes:96nat}),
{
    hide(bytes_filter_1_ct2_d16);hide(z_filter_1_ct2_d16);hide(raw_filter_1_ct2_d16);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_1_ct2_d16().len()==216nat) by (compute);
    assert(crate::model::mh(bytes_filter_1_ct2_d16())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:2u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_1_ct2_d16())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:2u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_1_ct2_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:159nat,next:204nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_1_ct2_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:159nat,next:204nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_1_ct2_d16(),204nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:212nat,len:0nat,next:216nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_1_ct2_d16(),204nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:212nat,len:0nat,next:216nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_1_ct2_d16().subrange(41,200)==z_filter_1_ct2_d16()) by (compute);
    walk_filter_1_ct2_d16(bytes_filter_1_ct2_d16());
    inflate_filter_1_ct2_d16();scan_filter_1_ct2_d16();
    assert(r::raw(a_filter_1_ct2_d16())==148nat) by (compute);
    assert(r::data(z_filter_1_ct2_d16(),a_filter_1_ct2_d16(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_filter_1_ct2_d16(),h_filter_1_ct2_d16(),z_filter_1_ct2_d16(),0nat,false);
}
fn public_filter_1_ct2_d16() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,2u8,0u8,0u8,0u8,114u8,246u8,5u8,87u8,0u8,0u8,0u8,159u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,148u8,0u8,107u8,255u8,1u8,187u8,124u8,42u8,162u8,35u8,240u8,233u8,237u8,242u8,101u8,244u8,195u8,239u8,135u8,138u8,250u8,252u8,199u8,66u8,131u8,24u8,137u8,4u8,1u8,186u8,91u8,22u8,119u8,228u8,170u8,252u8,237u8,42u8,17u8,11u8,83u8,1u8,229u8,248u8,175u8,246u8,38u8,69u8,68u8,29u8,202u8,234u8,36u8,215u8,17u8,29u8,59u8,134u8,16u8,198u8,190u8,189u8,188u8,91u8,1u8,247u8,88u8,129u8,166u8,82u8,214u8,222u8,67u8,59u8,234u8,1u8,93u8,51u8,1u8,169u8,193u8,72u8,67u8,17u8,38u8,226u8,28u8,12u8,245u8,65u8,153u8,62u8,60u8,10u8,145u8,128u8,71u8,127u8,142u8,255u8,56u8,241u8,199u8,179u8,128u8,118u8,154u8,110u8,254u8,7u8,241u8,249u8,220u8,97u8,225u8,1u8,203u8,119u8,143u8,123u8,197u8,202u8,70u8,227u8,220u8,63u8,216u8,52u8,163u8,93u8,195u8,116u8,215u8,234u8,39u8,102u8,68u8,27u8,125u8,105u8,19u8,16u8,216u8,241u8,210u8,191u8,184u8,192u8,138u8,23u8,91u8,13u8,3u8,17u8,75u8,49u8,171u8,174u8,64u8,54u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_1_ct2_d16());model_filter_1_ct2_d16();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==6u32 && s.height==4u32 && s.rgba@.len()==96nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// filter/1-ct4-d8; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_filter_1_ct4_d8() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,4u8,0u8,0u8,0u8,7u8,13u8,134u8,200u8,0u8,0u8,0u8,63u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,52u8,0u8,203u8,255u8,1u8,79u8,65u8,25u8,93u8,162u8,249u8,95u8,71u8,222u8,116u8,228u8,111u8,1u8,164u8,112u8,62u8,13u8,215u8,182u8,166u8,21u8,168u8,103u8,84u8,201u8,1u8,80u8,71u8,193u8,61u8,98u8,189u8,105u8,186u8,3u8,166u8,164u8,247u8,1u8,37u8,69u8,2u8,129u8,97u8,36u8,133u8,21u8,64u8,219u8,53u8,241u8,77u8,83u8,22u8,44u8,56u8,113u8,12u8,30u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_filter_1_ct4_d8() -> Seq<u8> {seq![120u8,156u8,1u8,52u8,0u8,203u8,255u8,1u8,79u8,65u8,25u8,93u8,162u8,249u8,95u8,71u8,222u8,116u8,228u8,111u8,1u8,164u8,112u8,62u8,13u8,215u8,182u8,166u8,21u8,168u8,103u8,84u8,201u8,1u8,80u8,71u8,193u8,61u8,98u8,189u8,105u8,186u8,3u8,166u8,164u8,247u8,1u8,37u8,69u8,2u8,129u8,97u8,36u8,133u8,21u8,64u8,219u8,53u8,241u8,77u8,83u8,22u8,44u8]}
spec fn h_filter_1_ct4_d8() -> crate::model::Ihdr {crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:4u8,interlace:0u8}}
spec fn a_filter_1_ct4_d8() -> r::Raster {r::shape(h_filter_1_ct4_d8(),6nat,4nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_filter_1_ct4_d8(b: Seq<u8>)
    requires b.len()==120,crate::model::mh(b)==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:4u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:63nat,next:108nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,108nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:116nat,len:0nat,next:120nat,crc_ok:true,synthetic:false})),
        b.subrange(41,104)==z_filter_1_ct4_d8(),
    ensures r::default_case(b,crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:4u8,interlace:0u8},z_filter_1_ct4_d8(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_filter_1_ct4_d8() -> Seq<u8> {seq![1u8,79u8,65u8,25u8,93u8,162u8,249u8,95u8,71u8,222u8,116u8,228u8,111u8,1u8,164u8,112u8,62u8,13u8,215u8,182u8,166u8,21u8,168u8,103u8,84u8,201u8,1u8,80u8,71u8,193u8,61u8,98u8,189u8,105u8,186u8,3u8,166u8,164u8,247u8,1u8,37u8,69u8,2u8,129u8,97u8,36u8,133u8,21u8,64u8,219u8,53u8,241u8]}
#[verifier::spinoff_prover]
proof fn shape_filter_1_ct4_d8()
    ensures raw_filter_1_ct4_d8().len()==52nat,
        z_filter_1_ct4_d8().len()==63nat,
        z_filter_1_ct4_d8()[0]==120u8,
        z_filter_1_ct4_d8()[1]==156u8,
        zm::field_value(z_filter_1_ct4_d8(),16,1)==1nat,
        zm::field_value(z_filter_1_ct4_d8(),17,2)==0nat,
        zm::be_value(z_filter_1_ct4_d8(),59nat,4)==1297290796nat,
        z_filter_1_ct4_d8().subrange(7,59)==raw_filter_1_ct4_d8(),
        z_filter_1_ct4_d8()[3] as nat+256*z_filter_1_ct4_d8()[4] as nat==52nat,
        z_filter_1_ct4_d8()[5] as nat+256*z_filter_1_ct4_d8()[6] as nat==u16::MAX as nat-52nat,
        zm::be_value(z_filter_1_ct4_d8(),0,1)==120nat,
        zm::be_value(z_filter_1_ct4_d8(),1,1)==156nat,
{
    hide(z_filter_1_ct4_d8);hide(raw_filter_1_ct4_d8);
    assert(raw_filter_1_ct4_d8().len()==52nat) by (compute);
    assert(z_filter_1_ct4_d8().len()==63nat) by (compute);
    assert(z_filter_1_ct4_d8()[0]==120u8) by (compute);
    assert(z_filter_1_ct4_d8()[1]==156u8) by (compute);
    assert(zm::field_value(z_filter_1_ct4_d8(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_filter_1_ct4_d8(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_filter_1_ct4_d8(),59nat,4)==1297290796nat) by (compute);
    assert(z_filter_1_ct4_d8().subrange(7,59)==raw_filter_1_ct4_d8()) by (compute);
    assert(z_filter_1_ct4_d8()[3] as nat+256*z_filter_1_ct4_d8()[4] as nat==52nat) by (compute);
    assert(z_filter_1_ct4_d8()[5] as nat+256*z_filter_1_ct4_d8()[6] as nat==u16::MAX as nat-52nat) by (compute);
    assert(zm::be_value(z_filter_1_ct4_d8(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_filter_1_ct4_d8(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_filter_1_ct4_d8()
    ensures zm::Adler(raw_filter_1_ct4_d8())==1297290796nat,
{
    hide(raw_filter_1_ct4_d8);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_filter_1_ct4_d8().len()==52nat) by (compute);
    assert(c::sums(raw_filter_1_ct4_d8(),0,52nat)==(5675nat,150837nat)) by (compute);
    c::sums_correct(raw_filter_1_ct4_d8(),0,52nat);
    assert(raw_filter_1_ct4_d8().subrange(0,52) =~= raw_filter_1_ct4_d8());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((150837nat%65521)*(u16::MAX as nat+1)+(1+5675nat)%65521==1297290796nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_filter_1_ct4_d8()
    ensures r::inflated(z_filter_1_ct4_d8(),a_filter_1_ct4_d8(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_filter_1_ct4_d8(),r2:false}),
        raw_filter_1_ct4_d8().len()==52nat,
{
    hide(z_filter_1_ct4_d8);hide(raw_filter_1_ct4_d8);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_filter_1_ct4_d8();checksum_filter_1_ct4_d8();
    assert(r::raw(a_filter_1_ct4_d8())==52nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_filter_1_ct4_d8(),raw_filter_1_ct4_d8(),52nat,120nat,156nat,1297290796nat);
}
#[verifier::spinoff_prover]
proof fn scan_filter_1_ct4_d8()
    ensures r::scan(raw_filter_1_ct4_d8(),a_filter_1_ct4_d8(),0)==Ok(()),
{
    hide(raw_filter_1_ct4_d8);
    assert(r::base(a_filter_1_ct4_d8(),0)==0nat && r::ph(a_filter_1_ct4_d8(),0)==4nat && r::count(a_filter_1_ct4_d8())==1nat) by (compute);
    assert(r::rb(a_filter_1_ct4_d8(),0)==12nat) by (compute);
    assert(raw_filter_1_ct4_d8()[0]==1u8) by (compute);
    assert(r::row_result(raw_filter_1_ct4_d8(),a_filter_1_ct4_d8(),0,0nat)==Ok(()));
    assert(raw_filter_1_ct4_d8()[13]==1u8) by (compute);
    assert(r::row_result(raw_filter_1_ct4_d8(),a_filter_1_ct4_d8(),0,1nat)==Ok(()));
    assert(raw_filter_1_ct4_d8()[26]==1u8) by (compute);
    assert(r::row_result(raw_filter_1_ct4_d8(),a_filter_1_ct4_d8(),0,2nat)==Ok(()));
    assert(raw_filter_1_ct4_d8()[39]==1u8) by (compute);
    assert(r::row_result(raw_filter_1_ct4_d8(),a_filter_1_ct4_d8(),0,3nat)==Ok(()));
    r::pass_step(raw_filter_1_ct4_d8(),a_filter_1_ct4_d8(),0,4nat);
    r::pass_step(raw_filter_1_ct4_d8(),a_filter_1_ct4_d8(),0,3nat);
    r::pass_step(raw_filter_1_ct4_d8(),a_filter_1_ct4_d8(),0,2nat);
    r::pass_step(raw_filter_1_ct4_d8(),a_filter_1_ct4_d8(),0,1nat);
    r::pass_step(raw_filter_1_ct4_d8(),a_filter_1_ct4_d8(),0,0nat);
    r::scan_step(raw_filter_1_ct4_d8(),a_filter_1_ct4_d8(),1);
    r::scan_step(raw_filter_1_ct4_d8(),a_filter_1_ct4_d8(),0);
}
#[verifier::spinoff_prover]
proof fn model_filter_1_ct4_d8()
    ensures r::still(bytes_filter_1_ct4_d8(),STRICT,false)==(r::Projection::Ready {width:6u32,height:4u32,bytes:96nat}),
{
    hide(bytes_filter_1_ct4_d8);hide(z_filter_1_ct4_d8);hide(raw_filter_1_ct4_d8);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_1_ct4_d8().len()==120nat) by (compute);
    assert(crate::model::mh(bytes_filter_1_ct4_d8())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:4u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_1_ct4_d8())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:4u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_1_ct4_d8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:63nat,next:108nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_1_ct4_d8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:63nat,next:108nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_1_ct4_d8(),108nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:116nat,len:0nat,next:120nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_1_ct4_d8(),108nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:116nat,len:0nat,next:120nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_1_ct4_d8().subrange(41,104)==z_filter_1_ct4_d8()) by (compute);
    walk_filter_1_ct4_d8(bytes_filter_1_ct4_d8());
    inflate_filter_1_ct4_d8();scan_filter_1_ct4_d8();
    assert(r::raw(a_filter_1_ct4_d8())==52nat) by (compute);
    assert(r::data(z_filter_1_ct4_d8(),a_filter_1_ct4_d8(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_filter_1_ct4_d8(),h_filter_1_ct4_d8(),z_filter_1_ct4_d8(),0nat,false);
}
fn public_filter_1_ct4_d8() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,4u8,0u8,0u8,0u8,7u8,13u8,134u8,200u8,0u8,0u8,0u8,63u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,52u8,0u8,203u8,255u8,1u8,79u8,65u8,25u8,93u8,162u8,249u8,95u8,71u8,222u8,116u8,228u8,111u8,1u8,164u8,112u8,62u8,13u8,215u8,182u8,166u8,21u8,168u8,103u8,84u8,201u8,1u8,80u8,71u8,193u8,61u8,98u8,189u8,105u8,186u8,3u8,166u8,164u8,247u8,1u8,37u8,69u8,2u8,129u8,97u8,36u8,133u8,21u8,64u8,219u8,53u8,241u8,77u8,83u8,22u8,44u8,56u8,113u8,12u8,30u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_1_ct4_d8());model_filter_1_ct4_d8();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==6u32 && s.height==4u32 && s.rgba@.len()==96nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// filter/1-ct6-d16; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_filter_1_ct6_d16() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,6u8,0u8,0u8,0u8,253u8,148u8,146u8,0u8,0u8,0u8,0u8,207u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,196u8,0u8,59u8,255u8,1u8,108u8,221u8,120u8,234u8,211u8,39u8,158u8,179u8,195u8,109u8,67u8,222u8,173u8,129u8,12u8,41u8,72u8,219u8,199u8,10u8,130u8,159u8,181u8,255u8,109u8,121u8,253u8,155u8,145u8,183u8,98u8,175u8,7u8,216u8,44u8,24u8,178u8,239u8,202u8,146u8,177u8,41u8,215u8,55u8,120u8,110u8,168u8,174u8,1u8,130u8,199u8,109u8,104u8,195u8,41u8,89u8,164u8,255u8,123u8,135u8,214u8,102u8,253u8,206u8,11u8,145u8,63u8,192u8,132u8,201u8,1u8,241u8,15u8,98u8,128u8,6u8,170u8,172u8,12u8,95u8,27u8,42u8,94u8,14u8,192u8,172u8,139u8,162u8,166u8,254u8,119u8,177u8,169u8,35u8,35u8,3u8,63u8,1u8,155u8,240u8,237u8,47u8,239u8,234u8,130u8,210u8,247u8,34u8,159u8,96u8,189u8,111u8,95u8,87u8,5u8,204u8,129u8,79u8,26u8,5u8,205u8,29u8,58u8,219u8,249u8,80u8,76u8,206u8,208u8,227u8,82u8,221u8,141u8,254u8,46u8,127u8,215u8,129u8,238u8,51u8,174u8,234u8,75u8,168u8,240u8,116u8,1u8,150u8,239u8,168u8,73u8,253u8,97u8,20u8,23u8,33u8,57u8,131u8,33u8,74u8,39u8,9u8,235u8,22u8,159u8,18u8,145u8,74u8,90u8,146u8,135u8,5u8,214u8,189u8,92u8,207u8,166u8,203u8,45u8,118u8,224u8,29u8,159u8,205u8,54u8,119u8,175u8,171u8,60u8,239u8,220u8,31u8,26u8,172u8,162u8,213u8,94u8,99u8,57u8,73u8,100u8,0u8,73u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_filter_1_ct6_d16() -> Seq<u8> {seq![120u8,156u8,1u8,196u8,0u8,59u8,255u8,1u8,108u8,221u8,120u8,234u8,211u8,39u8,158u8,179u8,195u8,109u8,67u8,222u8,173u8,129u8,12u8,41u8,72u8,219u8,199u8,10u8,130u8,159u8,181u8,255u8,109u8,121u8,253u8,155u8,145u8,183u8,98u8,175u8,7u8,216u8,44u8,24u8,178u8,239u8,202u8,146u8,177u8,41u8,215u8,55u8,120u8,110u8,168u8,174u8,1u8,130u8,199u8,109u8,104u8,195u8,41u8,89u8,164u8,255u8,123u8,135u8,214u8,102u8,253u8,206u8,11u8,145u8,63u8,192u8,132u8,201u8,1u8,241u8,15u8,98u8,128u8,6u8,170u8,172u8,12u8,95u8,27u8,42u8,94u8,14u8,192u8,172u8,139u8,162u8,166u8,254u8,119u8,177u8,169u8,35u8,35u8,3u8,63u8,1u8,155u8,240u8,237u8,47u8,239u8,234u8,130u8,210u8,247u8,34u8,159u8,96u8,189u8,111u8,95u8,87u8,5u8,204u8,129u8,79u8,26u8,5u8,205u8,29u8,58u8,219u8,249u8,80u8,76u8,206u8,208u8,227u8,82u8,221u8,141u8,254u8,46u8,127u8,215u8,129u8,238u8,51u8,174u8,234u8,75u8,168u8,240u8,116u8,1u8,150u8,239u8,168u8,73u8,253u8,97u8,20u8,23u8,33u8,57u8,131u8,33u8,74u8,39u8,9u8,235u8,22u8,159u8,18u8,145u8,74u8,90u8,146u8,135u8,5u8,214u8,189u8,92u8,207u8,166u8,203u8,45u8,118u8,224u8,29u8,159u8,205u8,54u8,119u8,175u8,171u8,60u8,239u8,220u8,31u8,26u8,172u8,162u8,213u8,94u8,99u8,57u8]}
spec fn h_filter_1_ct6_d16() -> crate::model::Ihdr {crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:6u8,interlace:0u8}}
spec fn a_filter_1_ct6_d16() -> r::Raster {r::shape(h_filter_1_ct6_d16(),6nat,4nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_filter_1_ct6_d16(b: Seq<u8>)
    requires b.len()==264,crate::model::mh(b)==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:6u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:207nat,next:252nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,252nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:260nat,len:0nat,next:264nat,crc_ok:true,synthetic:false})),
        b.subrange(41,248)==z_filter_1_ct6_d16(),
    ensures r::default_case(b,crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:6u8,interlace:0u8},z_filter_1_ct6_d16(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
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
spec fn raw_filter_1_ct6_d16() -> Seq<u8> {seq![1u8,108u8,221u8,120u8,234u8,211u8,39u8,158u8,179u8,195u8,109u8,67u8,222u8,173u8,129u8,12u8,41u8,72u8,219u8,199u8,10u8,130u8,159u8,181u8,255u8,109u8,121u8,253u8,155u8,145u8,183u8,98u8,175u8,7u8,216u8,44u8,24u8,178u8,239u8,202u8,146u8,177u8,41u8,215u8,55u8,120u8,110u8,168u8,174u8,1u8,130u8,199u8,109u8,104u8,195u8,41u8,89u8,164u8,255u8,123u8,135u8,214u8,102u8,253u8,206u8,11u8,145u8,63u8,192u8,132u8,201u8,1u8,241u8,15u8,98u8,128u8,6u8,170u8,172u8,12u8,95u8,27u8,42u8,94u8,14u8,192u8,172u8,139u8,162u8,166u8,254u8,119u8,177u8,169u8,35u8,35u8,3u8,63u8,1u8,155u8,240u8,237u8,47u8,239u8,234u8,130u8,210u8,247u8,34u8,159u8,96u8,189u8,111u8,95u8,87u8,5u8,204u8,129u8,79u8,26u8,5u8,205u8,29u8,58u8,219u8,249u8,80u8,76u8,206u8,208u8,227u8,82u8,221u8,141u8,254u8,46u8,127u8,215u8,129u8,238u8,51u8,174u8,234u8,75u8,168u8,240u8,116u8,1u8,150u8,239u8,168u8,73u8,253u8,97u8,20u8,23u8,33u8,57u8,131u8,33u8,74u8,39u8,9u8,235u8,22u8,159u8,18u8,145u8,74u8,90u8,146u8,135u8,5u8,214u8,189u8,92u8,207u8,166u8,203u8,45u8,118u8,224u8,29u8,159u8,205u8,54u8,119u8,175u8,171u8,60u8,239u8,220u8,31u8,26u8,172u8,162u8]}
#[verifier::spinoff_prover]
proof fn shape_filter_1_ct6_d16()
    ensures raw_filter_1_ct6_d16().len()==196nat,
        z_filter_1_ct6_d16().len()==207nat,
        z_filter_1_ct6_d16()[0]==120u8,
        z_filter_1_ct6_d16()[1]==156u8,
        zm::field_value(z_filter_1_ct6_d16(),16,1)==1nat,
        zm::field_value(z_filter_1_ct6_d16(),17,2)==0nat,
        zm::be_value(z_filter_1_ct6_d16(),203nat,4)==3579732793nat,
        z_filter_1_ct6_d16().subrange(7,203)==raw_filter_1_ct6_d16(),
        z_filter_1_ct6_d16()[3] as nat+256*z_filter_1_ct6_d16()[4] as nat==196nat,
        z_filter_1_ct6_d16()[5] as nat+256*z_filter_1_ct6_d16()[6] as nat==u16::MAX as nat-196nat,
        zm::be_value(z_filter_1_ct6_d16(),0,1)==120nat,
        zm::be_value(z_filter_1_ct6_d16(),1,1)==156nat,
{
    hide(z_filter_1_ct6_d16);hide(raw_filter_1_ct6_d16);
    assert(raw_filter_1_ct6_d16().len()==196nat) by (compute);
    assert(z_filter_1_ct6_d16().len()==207nat) by (compute);
    assert(z_filter_1_ct6_d16()[0]==120u8) by (compute);
    assert(z_filter_1_ct6_d16()[1]==156u8) by (compute);
    assert(zm::field_value(z_filter_1_ct6_d16(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_filter_1_ct6_d16(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_filter_1_ct6_d16(),203nat,4)==3579732793nat) by (compute);
    assert(z_filter_1_ct6_d16().subrange(7,203)==raw_filter_1_ct6_d16()) by (compute);
    assert(z_filter_1_ct6_d16()[3] as nat+256*z_filter_1_ct6_d16()[4] as nat==196nat) by (compute);
    assert(z_filter_1_ct6_d16()[5] as nat+256*z_filter_1_ct6_d16()[6] as nat==u16::MAX as nat-196nat) by (compute);
    assert(zm::be_value(z_filter_1_ct6_d16(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_filter_1_ct6_d16(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_filter_1_ct6_d16()
    ensures zm::Adler(raw_filter_1_ct6_d16())==3579732793nat,
{
    hide(raw_filter_1_ct6_d16);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_filter_1_ct6_d16().len()==196nat) by (compute);
    assert(c::sums(raw_filter_1_ct6_d16(),0,196nat)==(25400nat,2544420nat)) by (compute);
    c::sums_correct(raw_filter_1_ct6_d16(),0,196nat);
    assert(raw_filter_1_ct6_d16().subrange(0,196) =~= raw_filter_1_ct6_d16());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((2544420nat%65521)*(u16::MAX as nat+1)+(1+25400nat)%65521==3579732793nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_filter_1_ct6_d16()
    ensures r::inflated(z_filter_1_ct6_d16(),a_filter_1_ct6_d16(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_filter_1_ct6_d16(),r2:false}),
        raw_filter_1_ct6_d16().len()==196nat,
{
    hide(z_filter_1_ct6_d16);hide(raw_filter_1_ct6_d16);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_filter_1_ct6_d16();checksum_filter_1_ct6_d16();
    assert(r::raw(a_filter_1_ct6_d16())==196nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_filter_1_ct6_d16(),raw_filter_1_ct6_d16(),196nat,120nat,156nat,3579732793nat);
}
#[verifier::spinoff_prover]
proof fn scan_filter_1_ct6_d16()
    ensures r::scan(raw_filter_1_ct6_d16(),a_filter_1_ct6_d16(),0)==Ok(()),
{
    hide(raw_filter_1_ct6_d16);
    assert(r::base(a_filter_1_ct6_d16(),0)==0nat && r::ph(a_filter_1_ct6_d16(),0)==4nat && r::count(a_filter_1_ct6_d16())==1nat) by (compute);
    assert(r::rb(a_filter_1_ct6_d16(),0)==48nat) by (compute);
    assert(raw_filter_1_ct6_d16()[0]==1u8) by (compute);
    assert(r::row_result(raw_filter_1_ct6_d16(),a_filter_1_ct6_d16(),0,0nat)==Ok(()));
    assert(raw_filter_1_ct6_d16()[49]==1u8) by (compute);
    assert(r::row_result(raw_filter_1_ct6_d16(),a_filter_1_ct6_d16(),0,1nat)==Ok(()));
    assert(raw_filter_1_ct6_d16()[98]==1u8) by (compute);
    assert(r::row_result(raw_filter_1_ct6_d16(),a_filter_1_ct6_d16(),0,2nat)==Ok(()));
    assert(raw_filter_1_ct6_d16()[147]==1u8) by (compute);
    assert(r::row_result(raw_filter_1_ct6_d16(),a_filter_1_ct6_d16(),0,3nat)==Ok(()));
    r::pass_step(raw_filter_1_ct6_d16(),a_filter_1_ct6_d16(),0,4nat);
    r::pass_step(raw_filter_1_ct6_d16(),a_filter_1_ct6_d16(),0,3nat);
    r::pass_step(raw_filter_1_ct6_d16(),a_filter_1_ct6_d16(),0,2nat);
    r::pass_step(raw_filter_1_ct6_d16(),a_filter_1_ct6_d16(),0,1nat);
    r::pass_step(raw_filter_1_ct6_d16(),a_filter_1_ct6_d16(),0,0nat);
    r::scan_step(raw_filter_1_ct6_d16(),a_filter_1_ct6_d16(),1);
    r::scan_step(raw_filter_1_ct6_d16(),a_filter_1_ct6_d16(),0);
}
#[verifier::spinoff_prover]
proof fn model_filter_1_ct6_d16()
    ensures r::still(bytes_filter_1_ct6_d16(),STRICT,false)==(r::Projection::Ready {width:6u32,height:4u32,bytes:96nat}),
{
    hide(bytes_filter_1_ct6_d16);hide(z_filter_1_ct6_d16);hide(raw_filter_1_ct6_d16);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_1_ct6_d16().len()==264nat) by (compute);
    assert(crate::model::mh(bytes_filter_1_ct6_d16())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:6u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_1_ct6_d16())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:6u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_1_ct6_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:207nat,next:252nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_1_ct6_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:207nat,next:252nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_1_ct6_d16(),252nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:260nat,len:0nat,next:264nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_1_ct6_d16(),252nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:260nat,len:0nat,next:264nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_1_ct6_d16().subrange(41,248)==z_filter_1_ct6_d16()) by (compute);
    walk_filter_1_ct6_d16(bytes_filter_1_ct6_d16());
    inflate_filter_1_ct6_d16();scan_filter_1_ct6_d16();
    assert(r::raw(a_filter_1_ct6_d16())==196nat) by (compute);
    assert(r::data(z_filter_1_ct6_d16(),a_filter_1_ct6_d16(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_filter_1_ct6_d16(),h_filter_1_ct6_d16(),z_filter_1_ct6_d16(),0nat,false);
}
fn public_filter_1_ct6_d16() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,6u8,0u8,0u8,0u8,253u8,148u8,146u8,0u8,0u8,0u8,0u8,207u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,196u8,0u8,59u8,255u8,1u8,108u8,221u8,120u8,234u8,211u8,39u8,158u8,179u8,195u8,109u8,67u8,222u8,173u8,129u8,12u8,41u8,72u8,219u8,199u8,10u8,130u8,159u8,181u8,255u8,109u8,121u8,253u8,155u8,145u8,183u8,98u8,175u8,7u8,216u8,44u8,24u8,178u8,239u8,202u8,146u8,177u8,41u8,215u8,55u8,120u8,110u8,168u8,174u8,1u8,130u8,199u8,109u8,104u8,195u8,41u8,89u8,164u8,255u8,123u8,135u8,214u8,102u8,253u8,206u8,11u8,145u8,63u8,192u8,132u8,201u8,1u8,241u8,15u8,98u8,128u8,6u8,170u8,172u8,12u8,95u8,27u8,42u8,94u8,14u8,192u8,172u8,139u8,162u8,166u8,254u8,119u8,177u8,169u8,35u8,35u8,3u8,63u8,1u8,155u8,240u8,237u8,47u8,239u8,234u8,130u8,210u8,247u8,34u8,159u8,96u8,189u8,111u8,95u8,87u8,5u8,204u8,129u8,79u8,26u8,5u8,205u8,29u8,58u8,219u8,249u8,80u8,76u8,206u8,208u8,227u8,82u8,221u8,141u8,254u8,46u8,127u8,215u8,129u8,238u8,51u8,174u8,234u8,75u8,168u8,240u8,116u8,1u8,150u8,239u8,168u8,73u8,253u8,97u8,20u8,23u8,33u8,57u8,131u8,33u8,74u8,39u8,9u8,235u8,22u8,159u8,18u8,145u8,74u8,90u8,146u8,135u8,5u8,214u8,189u8,92u8,207u8,166u8,203u8,45u8,118u8,224u8,29u8,159u8,205u8,54u8,119u8,175u8,171u8,60u8,239u8,220u8,31u8,26u8,172u8,162u8,213u8,94u8,99u8,57u8,73u8,100u8,0u8,73u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_1_ct6_d16());model_filter_1_ct6_d16();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==6u32 && s.height==4u32 && s.rgba@.len()==96nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// filter/2-ct0-d1; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_filter_2_ct0_d1() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,1u8,0u8,0u8,0u8,0u8,133u8,127u8,115u8,238u8,0u8,0u8,0u8,16u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,90u8,193u8,180u8,134u8,105u8,14u8,211u8,3u8,0u8,10u8,216u8,2u8,217u8,61u8,158u8,25u8,28u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_filter_2_ct0_d1() -> Seq<u8> {seq![120u8,156u8,99u8,90u8,193u8,180u8,134u8,105u8,14u8,211u8,3u8,0u8,10u8,216u8,2u8,217u8]}
spec fn h_filter_2_ct0_d1() -> crate::model::Ihdr {crate::model::Ihdr {width:6u32,height:4u32,depth:1u8,color_type:0u8,interlace:0u8}}
spec fn a_filter_2_ct0_d1() -> r::Raster {r::shape(h_filter_2_ct0_d1(),6nat,4nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_filter_2_ct0_d1(b: Seq<u8>)
    requires b.len()==73,crate::model::mh(b)==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:1u8,color_type:0u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:16nat,next:61nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,61nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:69nat,len:0nat,next:73nat,crc_ok:true,synthetic:false})),
        b.subrange(41,57)==z_filter_2_ct0_d1(),
    ensures r::default_case(b,crate::model::Ihdr {width:6u32,height:4u32,depth:1u8,color_type:0u8,interlace:0u8},z_filter_2_ct0_d1(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
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
spec fn raw_filter_2_ct0_d1() -> Seq<u8> {seq![2u8,168u8,2u8,172u8,2u8,156u8,2u8,224u8]}
spec fn ts_filter_2_ct0_d1() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:36nat,before:1nat,after:2nat,symbol:168nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:36nat,next:44nat,before:2nat,after:3nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:44nat,next:53nat,before:3nat,after:4nat,symbol:172nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:53nat,next:61nat,before:4nat,after:5nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:61nat,next:70nat,before:5nat,after:6nat,symbol:156nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:70nat,next:78nat,before:6nat,after:7nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:78nat,next:87nat,before:7nat,after:8nat,symbol:224nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:87nat,next:94nat,before:8nat,after:8nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_filter_2_ct0_d1()
    ensures t::trace(z_filter_2_ct0_d1(),raw_filter_2_ct0_d1(),ts_filter_2_ct0_d1(),0,9nat),
{
    hide(z_filter_2_ct0_d1);hide(raw_filter_2_ct0_d1);hide(ts_filter_2_ct0_d1);hide(t::valid_step);
    assert(t::trace(z_filter_2_ct0_d1(),raw_filter_2_ct0_d1(),ts_filter_2_ct0_d1(),0,9nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_filter_2_ct0_d1()
    ensures raw_filter_2_ct0_d1().len()==8nat,
        z_filter_2_ct0_d1().len()==16nat,
        z_filter_2_ct0_d1()[0]==120u8,
        z_filter_2_ct0_d1()[1]==156u8,
        zm::field_value(z_filter_2_ct0_d1(),16,1)==1nat,
        zm::field_value(z_filter_2_ct0_d1(),17,2)==1nat,
        zm::be_value(z_filter_2_ct0_d1(),12nat,4)==181928665nat,
        ts_filter_2_ct0_d1().len()==9nat,
        ts_filter_2_ct0_d1()[0].pos==19nat,
        ts_filter_2_ct0_d1()[0].before==0nat,
        ts_filter_2_ct0_d1().last().next==94nat,
{
    hide(z_filter_2_ct0_d1);hide(raw_filter_2_ct0_d1);hide(ts_filter_2_ct0_d1);
    assert(raw_filter_2_ct0_d1().len()==8nat) by (compute);
    assert(z_filter_2_ct0_d1().len()==16nat) by (compute);
    assert(z_filter_2_ct0_d1()[0]==120u8) by (compute);
    assert(z_filter_2_ct0_d1()[1]==156u8) by (compute);
    assert(zm::field_value(z_filter_2_ct0_d1(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_filter_2_ct0_d1(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_filter_2_ct0_d1(),12nat,4)==181928665nat) by (compute);
    assert(ts_filter_2_ct0_d1().len()==9nat) by (compute);
    assert(ts_filter_2_ct0_d1()[0].pos==19nat) by (compute);
    assert(ts_filter_2_ct0_d1()[0].before==0nat) by (compute);
    assert(ts_filter_2_ct0_d1().last().next==94nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_filter_2_ct0_d1()
    ensures zm::Adler(raw_filter_2_ct0_d1())==181928665nat,
{
    hide(raw_filter_2_ct0_d1);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_filter_2_ct0_d1().len()==8nat) by (compute);
    assert(c::sums(raw_filter_2_ct0_d1(),0,8nat)==(728nat,2776nat)) by (compute);
    c::sums_correct(raw_filter_2_ct0_d1(),0,8nat);
    assert(raw_filter_2_ct0_d1().subrange(0,8) =~= raw_filter_2_ct0_d1());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((2776nat%65521)*(u16::MAX as nat+1)+(1+728nat)%65521==181928665nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_filter_2_ct0_d1()
    ensures r::inflated(z_filter_2_ct0_d1(),a_filter_2_ct0_d1(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_filter_2_ct0_d1(),r2:false}),
        raw_filter_2_ct0_d1().len()==8nat,
{
    hide(z_filter_2_ct0_d1);hide(raw_filter_2_ct0_d1);hide(ts_filter_2_ct0_d1);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_filter_2_ct0_d1();checksum_filter_2_ct0_d1();
    assert(r::raw(a_filter_2_ct0_d1())==8nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_filter_2_ct0_d1();
    stream::fixed_stream(z_filter_2_ct0_d1(),raw_filter_2_ct0_d1(),ts_filter_2_ct0_d1(),8nat,156nat,181928665nat);
}
#[verifier::spinoff_prover]
proof fn scan_filter_2_ct0_d1()
    ensures r::scan(raw_filter_2_ct0_d1(),a_filter_2_ct0_d1(),0)==Ok(()),
{
    hide(raw_filter_2_ct0_d1);
    assert(r::base(a_filter_2_ct0_d1(),0)==0nat && r::ph(a_filter_2_ct0_d1(),0)==4nat && r::count(a_filter_2_ct0_d1())==1nat) by (compute);
    assert(r::rb(a_filter_2_ct0_d1(),0)==1nat) by (compute);
    assert(raw_filter_2_ct0_d1()[0]==2u8) by (compute);
    assert(r::row_result(raw_filter_2_ct0_d1(),a_filter_2_ct0_d1(),0,0nat)==Ok(()));
    assert(raw_filter_2_ct0_d1()[2]==2u8) by (compute);
    assert(r::row_result(raw_filter_2_ct0_d1(),a_filter_2_ct0_d1(),0,1nat)==Ok(()));
    assert(raw_filter_2_ct0_d1()[4]==2u8) by (compute);
    assert(r::row_result(raw_filter_2_ct0_d1(),a_filter_2_ct0_d1(),0,2nat)==Ok(()));
    assert(raw_filter_2_ct0_d1()[6]==2u8) by (compute);
    assert(r::row_result(raw_filter_2_ct0_d1(),a_filter_2_ct0_d1(),0,3nat)==Ok(()));
    r::pass_step(raw_filter_2_ct0_d1(),a_filter_2_ct0_d1(),0,4nat);
    r::pass_step(raw_filter_2_ct0_d1(),a_filter_2_ct0_d1(),0,3nat);
    r::pass_step(raw_filter_2_ct0_d1(),a_filter_2_ct0_d1(),0,2nat);
    r::pass_step(raw_filter_2_ct0_d1(),a_filter_2_ct0_d1(),0,1nat);
    r::pass_step(raw_filter_2_ct0_d1(),a_filter_2_ct0_d1(),0,0nat);
    r::scan_step(raw_filter_2_ct0_d1(),a_filter_2_ct0_d1(),1);
    r::scan_step(raw_filter_2_ct0_d1(),a_filter_2_ct0_d1(),0);
}
#[verifier::spinoff_prover]
proof fn model_filter_2_ct0_d1()
    ensures r::still(bytes_filter_2_ct0_d1(),STRICT,false)==(r::Projection::Ready {width:6u32,height:4u32,bytes:96nat}),
{
    hide(bytes_filter_2_ct0_d1);hide(z_filter_2_ct0_d1);hide(raw_filter_2_ct0_d1);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_2_ct0_d1().len()==73nat) by (compute);
    assert(crate::model::mh(bytes_filter_2_ct0_d1())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:1u8,color_type:0u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_2_ct0_d1())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:1u8,color_type:0u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 1u8)==2u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_2_ct0_d1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:16nat,next:61nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_2_ct0_d1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:16nat,next:61nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_2_ct0_d1(),61nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:69nat,len:0nat,next:73nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_2_ct0_d1(),61nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:69nat,len:0nat,next:73nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_2_ct0_d1().subrange(41,57)==z_filter_2_ct0_d1()) by (compute);
    walk_filter_2_ct0_d1(bytes_filter_2_ct0_d1());
    inflate_filter_2_ct0_d1();scan_filter_2_ct0_d1();
    assert(r::raw(a_filter_2_ct0_d1())==8nat) by (compute);
    assert(r::data(z_filter_2_ct0_d1(),a_filter_2_ct0_d1(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_filter_2_ct0_d1(),h_filter_2_ct0_d1(),z_filter_2_ct0_d1(),0nat,false);
}
fn public_filter_2_ct0_d1() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,1u8,0u8,0u8,0u8,0u8,133u8,127u8,115u8,238u8,0u8,0u8,0u8,16u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,90u8,193u8,180u8,134u8,105u8,14u8,211u8,3u8,0u8,10u8,216u8,2u8,217u8,61u8,158u8,25u8,28u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_2_ct0_d1());model_filter_2_ct0_d1();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==6u32 && s.height==4u32 && s.rgba@.len()==96nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// filter/2-ct0-d8; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_filter_2_ct0_d8() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,0u8,0u8,0u8,0u8,136u8,111u8,17u8,159u8,0u8,0u8,0u8,37u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,178u8,123u8,234u8,35u8,246u8,91u8,131u8,233u8,28u8,135u8,197u8,242u8,144u8,88u8,166u8,157u8,238u8,215u8,195u8,23u8,109u8,101u8,250u8,185u8,224u8,205u8,141u8,55u8,11u8,1u8,162u8,180u8,13u8,134u8,226u8,196u8,163u8,215u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_filter_2_ct0_d8() -> Seq<u8> {seq![120u8,156u8,99u8,178u8,123u8,234u8,35u8,246u8,91u8,131u8,233u8,28u8,135u8,197u8,242u8,144u8,88u8,166u8,157u8,238u8,215u8,195u8,23u8,109u8,101u8,250u8,185u8,224u8,205u8,141u8,55u8,11u8,1u8,162u8,180u8,13u8,134u8]}
spec fn h_filter_2_ct0_d8() -> crate::model::Ihdr {crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:0u8,interlace:0u8}}
spec fn a_filter_2_ct0_d8() -> r::Raster {r::shape(h_filter_2_ct0_d8(),6nat,4nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_filter_2_ct0_d8(b: Seq<u8>)
    requires b.len()==94,crate::model::mh(b)==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:0u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:37nat,next:82nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,82nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:90nat,len:0nat,next:94nat,crc_ok:true,synthetic:false})),
        b.subrange(41,78)==z_filter_2_ct0_d8(),
    ensures r::default_case(b,crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:0u8,interlace:0u8},z_filter_2_ct0_d8(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_filter_2_ct0_d8() -> Seq<u8> {seq![2u8,62u8,229u8,76u8,22u8,251u8,40u8,2u8,206u8,8u8,56u8,167u8,84u8,93u8,2u8,185u8,71u8,215u8,87u8,162u8,181u8,2u8,249u8,160u8,236u8,216u8,236u8,161u8]}
spec fn ts_filter_2_ct0_d8() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:35nat,before:1nat,after:2nat,symbol:62nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:35nat,next:44nat,before:2nat,after:3nat,symbol:229nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:44nat,next:52nat,before:3nat,after:4nat,symbol:76nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:52nat,next:60nat,before:4nat,after:5nat,symbol:22nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:60nat,next:69nat,before:5nat,after:6nat,symbol:251nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:69nat,next:77nat,before:6nat,after:7nat,symbol:40nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:77nat,next:85nat,before:7nat,after:8nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:85nat,next:94nat,before:8nat,after:9nat,symbol:206nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:94nat,next:102nat,before:9nat,after:10nat,symbol:8nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:102nat,next:110nat,before:10nat,after:11nat,symbol:56nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:110nat,next:119nat,before:11nat,after:12nat,symbol:167nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:119nat,next:127nat,before:12nat,after:13nat,symbol:84nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:127nat,next:135nat,before:13nat,after:14nat,symbol:93nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:135nat,next:143nat,before:14nat,after:15nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:143nat,next:152nat,before:15nat,after:16nat,symbol:185nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:152nat,next:160nat,before:16nat,after:17nat,symbol:71nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:160nat,next:169nat,before:17nat,after:18nat,symbol:215nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:169nat,next:177nat,before:18nat,after:19nat,symbol:87nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:177nat,next:186nat,before:19nat,after:20nat,symbol:162nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:186nat,next:195nat,before:20nat,after:21nat,symbol:181nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:195nat,next:203nat,before:21nat,after:22nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:203nat,next:212nat,before:22nat,after:23nat,symbol:249nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:212nat,next:221nat,before:23nat,after:24nat,symbol:160nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:221nat,next:230nat,before:24nat,after:25nat,symbol:236nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:230nat,next:239nat,before:25nat,after:26nat,symbol:216nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:239nat,next:248nat,before:26nat,after:27nat,symbol:236nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:248nat,next:257nat,before:27nat,after:28nat,symbol:161nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:257nat,next:264nat,before:28nat,after:28nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_filter_2_ct0_d8()
    ensures t::trace(z_filter_2_ct0_d8(),raw_filter_2_ct0_d8(),ts_filter_2_ct0_d8(),0,29nat),
{
    hide(z_filter_2_ct0_d8);hide(raw_filter_2_ct0_d8);hide(ts_filter_2_ct0_d8);hide(t::valid_step);
    assert(t::trace(z_filter_2_ct0_d8(),raw_filter_2_ct0_d8(),ts_filter_2_ct0_d8(),0,29nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_filter_2_ct0_d8()
    ensures raw_filter_2_ct0_d8().len()==28nat,
        z_filter_2_ct0_d8().len()==37nat,
        z_filter_2_ct0_d8()[0]==120u8,
        z_filter_2_ct0_d8()[1]==156u8,
        zm::field_value(z_filter_2_ct0_d8(),16,1)==1nat,
        zm::field_value(z_filter_2_ct0_d8(),17,2)==1nat,
        zm::be_value(z_filter_2_ct0_d8(),33nat,4)==2729708934nat,
        ts_filter_2_ct0_d8().len()==29nat,
        ts_filter_2_ct0_d8()[0].pos==19nat,
        ts_filter_2_ct0_d8()[0].before==0nat,
        ts_filter_2_ct0_d8().last().next==264nat,
{
    hide(z_filter_2_ct0_d8);hide(raw_filter_2_ct0_d8);hide(ts_filter_2_ct0_d8);
    assert(raw_filter_2_ct0_d8().len()==28nat) by (compute);
    assert(z_filter_2_ct0_d8().len()==37nat) by (compute);
    assert(z_filter_2_ct0_d8()[0]==120u8) by (compute);
    assert(z_filter_2_ct0_d8()[1]==156u8) by (compute);
    assert(zm::field_value(z_filter_2_ct0_d8(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_filter_2_ct0_d8(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_filter_2_ct0_d8(),33nat,4)==2729708934nat) by (compute);
    assert(ts_filter_2_ct0_d8().len()==29nat) by (compute);
    assert(ts_filter_2_ct0_d8()[0].pos==19nat) by (compute);
    assert(ts_filter_2_ct0_d8()[0].before==0nat) by (compute);
    assert(ts_filter_2_ct0_d8().last().next==264nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_filter_2_ct0_d8()
    ensures zm::Adler(raw_filter_2_ct0_d8())==2729708934nat,
{
    hide(raw_filter_2_ct0_d8);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_filter_2_ct0_d8().len()==28nat) by (compute);
    assert(c::sums(raw_filter_2_ct0_d8(),0,28nat)==(3461nat,41652nat)) by (compute);
    c::sums_correct(raw_filter_2_ct0_d8(),0,28nat);
    assert(raw_filter_2_ct0_d8().subrange(0,28) =~= raw_filter_2_ct0_d8());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((41652nat%65521)*(u16::MAX as nat+1)+(1+3461nat)%65521==2729708934nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_filter_2_ct0_d8()
    ensures r::inflated(z_filter_2_ct0_d8(),a_filter_2_ct0_d8(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_filter_2_ct0_d8(),r2:false}),
        raw_filter_2_ct0_d8().len()==28nat,
{
    hide(z_filter_2_ct0_d8);hide(raw_filter_2_ct0_d8);hide(ts_filter_2_ct0_d8);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_filter_2_ct0_d8();checksum_filter_2_ct0_d8();
    assert(r::raw(a_filter_2_ct0_d8())==28nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_filter_2_ct0_d8();
    stream::fixed_stream(z_filter_2_ct0_d8(),raw_filter_2_ct0_d8(),ts_filter_2_ct0_d8(),28nat,156nat,2729708934nat);
}
#[verifier::spinoff_prover]
proof fn scan_filter_2_ct0_d8()
    ensures r::scan(raw_filter_2_ct0_d8(),a_filter_2_ct0_d8(),0)==Ok(()),
{
    hide(raw_filter_2_ct0_d8);
    assert(r::base(a_filter_2_ct0_d8(),0)==0nat && r::ph(a_filter_2_ct0_d8(),0)==4nat && r::count(a_filter_2_ct0_d8())==1nat) by (compute);
    assert(r::rb(a_filter_2_ct0_d8(),0)==6nat) by (compute);
    assert(raw_filter_2_ct0_d8()[0]==2u8) by (compute);
    assert(r::row_result(raw_filter_2_ct0_d8(),a_filter_2_ct0_d8(),0,0nat)==Ok(()));
    assert(raw_filter_2_ct0_d8()[7]==2u8) by (compute);
    assert(r::row_result(raw_filter_2_ct0_d8(),a_filter_2_ct0_d8(),0,1nat)==Ok(()));
    assert(raw_filter_2_ct0_d8()[14]==2u8) by (compute);
    assert(r::row_result(raw_filter_2_ct0_d8(),a_filter_2_ct0_d8(),0,2nat)==Ok(()));
    assert(raw_filter_2_ct0_d8()[21]==2u8) by (compute);
    assert(r::row_result(raw_filter_2_ct0_d8(),a_filter_2_ct0_d8(),0,3nat)==Ok(()));
    r::pass_step(raw_filter_2_ct0_d8(),a_filter_2_ct0_d8(),0,4nat);
    r::pass_step(raw_filter_2_ct0_d8(),a_filter_2_ct0_d8(),0,3nat);
    r::pass_step(raw_filter_2_ct0_d8(),a_filter_2_ct0_d8(),0,2nat);
    r::pass_step(raw_filter_2_ct0_d8(),a_filter_2_ct0_d8(),0,1nat);
    r::pass_step(raw_filter_2_ct0_d8(),a_filter_2_ct0_d8(),0,0nat);
    r::scan_step(raw_filter_2_ct0_d8(),a_filter_2_ct0_d8(),1);
    r::scan_step(raw_filter_2_ct0_d8(),a_filter_2_ct0_d8(),0);
}
#[verifier::spinoff_prover]
proof fn model_filter_2_ct0_d8()
    ensures r::still(bytes_filter_2_ct0_d8(),STRICT,false)==(r::Projection::Ready {width:6u32,height:4u32,bytes:96nat}),
{
    hide(bytes_filter_2_ct0_d8);hide(z_filter_2_ct0_d8);hide(raw_filter_2_ct0_d8);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_2_ct0_d8().len()==94nat) by (compute);
    assert(crate::model::mh(bytes_filter_2_ct0_d8())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:0u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_2_ct0_d8())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:0u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_2_ct0_d8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:37nat,next:82nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_2_ct0_d8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:37nat,next:82nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_2_ct0_d8(),82nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:90nat,len:0nat,next:94nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_2_ct0_d8(),82nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:90nat,len:0nat,next:94nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_2_ct0_d8().subrange(41,78)==z_filter_2_ct0_d8()) by (compute);
    walk_filter_2_ct0_d8(bytes_filter_2_ct0_d8());
    inflate_filter_2_ct0_d8();scan_filter_2_ct0_d8();
    assert(r::raw(a_filter_2_ct0_d8())==28nat) by (compute);
    assert(r::data(z_filter_2_ct0_d8(),a_filter_2_ct0_d8(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_filter_2_ct0_d8(),h_filter_2_ct0_d8(),z_filter_2_ct0_d8(),0nat,false);
}
fn public_filter_2_ct0_d8() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,0u8,0u8,0u8,0u8,136u8,111u8,17u8,159u8,0u8,0u8,0u8,37u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,178u8,123u8,234u8,35u8,246u8,91u8,131u8,233u8,28u8,135u8,197u8,242u8,144u8,88u8,166u8,157u8,238u8,215u8,195u8,23u8,109u8,101u8,250u8,185u8,224u8,205u8,141u8,55u8,11u8,1u8,162u8,180u8,13u8,134u8,226u8,196u8,163u8,215u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_2_ct0_d8());model_filter_2_ct0_d8();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==6u32 && s.height==4u32 && s.rgba@.len()==96nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// filter/2-ct0-d16; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_filter_2_ct0_d16() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,0u8,0u8,0u8,0u8,216u8,255u8,205u8,220u8,0u8,0u8,0u8,63u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,52u8,0u8,203u8,255u8,2u8,172u8,60u8,145u8,40u8,212u8,8u8,109u8,77u8,105u8,246u8,141u8,65u8,2u8,249u8,165u8,99u8,67u8,85u8,149u8,18u8,71u8,159u8,208u8,189u8,238u8,2u8,159u8,83u8,178u8,108u8,59u8,76u8,6u8,55u8,90u8,48u8,89u8,119u8,2u8,85u8,80u8,139u8,25u8,90u8,3u8,191u8,101u8,70u8,46u8,59u8,206u8,57u8,156u8,20u8,131u8,167u8,250u8,95u8,158u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_filter_2_ct0_d16() -> Seq<u8> {seq![120u8,156u8,1u8,52u8,0u8,203u8,255u8,2u8,172u8,60u8,145u8,40u8,212u8,8u8,109u8,77u8,105u8,246u8,141u8,65u8,2u8,249u8,165u8,99u8,67u8,85u8,149u8,18u8,71u8,159u8,208u8,189u8,238u8,2u8,159u8,83u8,178u8,108u8,59u8,76u8,6u8,55u8,90u8,48u8,89u8,119u8,2u8,85u8,80u8,139u8,25u8,90u8,3u8,191u8,101u8,70u8,46u8,59u8,206u8,57u8,156u8,20u8,131u8]}
spec fn h_filter_2_ct0_d16() -> crate::model::Ihdr {crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:0u8,interlace:0u8}}
spec fn a_filter_2_ct0_d16() -> r::Raster {r::shape(h_filter_2_ct0_d16(),6nat,4nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_filter_2_ct0_d16(b: Seq<u8>)
    requires b.len()==120,crate::model::mh(b)==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:0u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:63nat,next:108nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,108nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:116nat,len:0nat,next:120nat,crc_ok:true,synthetic:false})),
        b.subrange(41,104)==z_filter_2_ct0_d16(),
    ensures r::default_case(b,crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:0u8,interlace:0u8},z_filter_2_ct0_d16(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
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
spec fn raw_filter_2_ct0_d16() -> Seq<u8> {seq![2u8,172u8,60u8,145u8,40u8,212u8,8u8,109u8,77u8,105u8,246u8,141u8,65u8,2u8,249u8,165u8,99u8,67u8,85u8,149u8,18u8,71u8,159u8,208u8,189u8,238u8,2u8,159u8,83u8,178u8,108u8,59u8,76u8,6u8,55u8,90u8,48u8,89u8,119u8,2u8,85u8,80u8,139u8,25u8,90u8,3u8,191u8,101u8,70u8,46u8,59u8,206u8]}
#[verifier::spinoff_prover]
proof fn shape_filter_2_ct0_d16()
    ensures raw_filter_2_ct0_d16().len()==52nat,
        z_filter_2_ct0_d16().len()==63nat,
        z_filter_2_ct0_d16()[0]==120u8,
        z_filter_2_ct0_d16()[1]==156u8,
        zm::field_value(z_filter_2_ct0_d16(),16,1)==1nat,
        zm::field_value(z_filter_2_ct0_d16(),17,2)==0nat,
        zm::be_value(z_filter_2_ct0_d16(),59nat,4)==966530179nat,
        z_filter_2_ct0_d16().subrange(7,59)==raw_filter_2_ct0_d16(),
        z_filter_2_ct0_d16()[3] as nat+256*z_filter_2_ct0_d16()[4] as nat==52nat,
        z_filter_2_ct0_d16()[5] as nat+256*z_filter_2_ct0_d16()[6] as nat==u16::MAX as nat-52nat,
        zm::be_value(z_filter_2_ct0_d16(),0,1)==120nat,
        zm::be_value(z_filter_2_ct0_d16(),1,1)==156nat,
{
    hide(z_filter_2_ct0_d16);hide(raw_filter_2_ct0_d16);
    assert(raw_filter_2_ct0_d16().len()==52nat) by (compute);
    assert(z_filter_2_ct0_d16().len()==63nat) by (compute);
    assert(z_filter_2_ct0_d16()[0]==120u8) by (compute);
    assert(z_filter_2_ct0_d16()[1]==156u8) by (compute);
    assert(zm::field_value(z_filter_2_ct0_d16(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_filter_2_ct0_d16(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_filter_2_ct0_d16(),59nat,4)==966530179nat) by (compute);
    assert(z_filter_2_ct0_d16().subrange(7,59)==raw_filter_2_ct0_d16()) by (compute);
    assert(z_filter_2_ct0_d16()[3] as nat+256*z_filter_2_ct0_d16()[4] as nat==52nat) by (compute);
    assert(z_filter_2_ct0_d16()[5] as nat+256*z_filter_2_ct0_d16()[6] as nat==u16::MAX as nat-52nat) by (compute);
    assert(zm::be_value(z_filter_2_ct0_d16(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_filter_2_ct0_d16(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_filter_2_ct0_d16()
    ensures zm::Adler(raw_filter_2_ct0_d16())==966530179nat,
{
    hide(raw_filter_2_ct0_d16);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_filter_2_ct0_d16().len()==52nat) by (compute);
    assert(c::sums(raw_filter_2_ct0_d16(),0,52nat)==(5250nat,145790nat)) by (compute);
    c::sums_correct(raw_filter_2_ct0_d16(),0,52nat);
    assert(raw_filter_2_ct0_d16().subrange(0,52) =~= raw_filter_2_ct0_d16());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((145790nat%65521)*(u16::MAX as nat+1)+(1+5250nat)%65521==966530179nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_filter_2_ct0_d16()
    ensures r::inflated(z_filter_2_ct0_d16(),a_filter_2_ct0_d16(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_filter_2_ct0_d16(),r2:false}),
        raw_filter_2_ct0_d16().len()==52nat,
{
    hide(z_filter_2_ct0_d16);hide(raw_filter_2_ct0_d16);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_filter_2_ct0_d16();checksum_filter_2_ct0_d16();
    assert(r::raw(a_filter_2_ct0_d16())==52nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_filter_2_ct0_d16(),raw_filter_2_ct0_d16(),52nat,120nat,156nat,966530179nat);
}
#[verifier::spinoff_prover]
proof fn scan_filter_2_ct0_d16()
    ensures r::scan(raw_filter_2_ct0_d16(),a_filter_2_ct0_d16(),0)==Ok(()),
{
    hide(raw_filter_2_ct0_d16);
    assert(r::base(a_filter_2_ct0_d16(),0)==0nat && r::ph(a_filter_2_ct0_d16(),0)==4nat && r::count(a_filter_2_ct0_d16())==1nat) by (compute);
    assert(r::rb(a_filter_2_ct0_d16(),0)==12nat) by (compute);
    assert(raw_filter_2_ct0_d16()[0]==2u8) by (compute);
    assert(r::row_result(raw_filter_2_ct0_d16(),a_filter_2_ct0_d16(),0,0nat)==Ok(()));
    assert(raw_filter_2_ct0_d16()[13]==2u8) by (compute);
    assert(r::row_result(raw_filter_2_ct0_d16(),a_filter_2_ct0_d16(),0,1nat)==Ok(()));
    assert(raw_filter_2_ct0_d16()[26]==2u8) by (compute);
    assert(r::row_result(raw_filter_2_ct0_d16(),a_filter_2_ct0_d16(),0,2nat)==Ok(()));
    assert(raw_filter_2_ct0_d16()[39]==2u8) by (compute);
    assert(r::row_result(raw_filter_2_ct0_d16(),a_filter_2_ct0_d16(),0,3nat)==Ok(()));
    r::pass_step(raw_filter_2_ct0_d16(),a_filter_2_ct0_d16(),0,4nat);
    r::pass_step(raw_filter_2_ct0_d16(),a_filter_2_ct0_d16(),0,3nat);
    r::pass_step(raw_filter_2_ct0_d16(),a_filter_2_ct0_d16(),0,2nat);
    r::pass_step(raw_filter_2_ct0_d16(),a_filter_2_ct0_d16(),0,1nat);
    r::pass_step(raw_filter_2_ct0_d16(),a_filter_2_ct0_d16(),0,0nat);
    r::scan_step(raw_filter_2_ct0_d16(),a_filter_2_ct0_d16(),1);
    r::scan_step(raw_filter_2_ct0_d16(),a_filter_2_ct0_d16(),0);
}
#[verifier::spinoff_prover]
proof fn model_filter_2_ct0_d16()
    ensures r::still(bytes_filter_2_ct0_d16(),STRICT,false)==(r::Projection::Ready {width:6u32,height:4u32,bytes:96nat}),
{
    hide(bytes_filter_2_ct0_d16);hide(z_filter_2_ct0_d16);hide(raw_filter_2_ct0_d16);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_2_ct0_d16().len()==120nat) by (compute);
    assert(crate::model::mh(bytes_filter_2_ct0_d16())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:0u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_2_ct0_d16())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:0u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_2_ct0_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:63nat,next:108nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_2_ct0_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:63nat,next:108nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_2_ct0_d16(),108nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:116nat,len:0nat,next:120nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_2_ct0_d16(),108nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:116nat,len:0nat,next:120nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_2_ct0_d16().subrange(41,104)==z_filter_2_ct0_d16()) by (compute);
    walk_filter_2_ct0_d16(bytes_filter_2_ct0_d16());
    inflate_filter_2_ct0_d16();scan_filter_2_ct0_d16();
    assert(r::raw(a_filter_2_ct0_d16())==52nat) by (compute);
    assert(r::data(z_filter_2_ct0_d16(),a_filter_2_ct0_d16(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_filter_2_ct0_d16(),h_filter_2_ct0_d16(),z_filter_2_ct0_d16(),0nat,false);
}
fn public_filter_2_ct0_d16() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,0u8,0u8,0u8,0u8,216u8,255u8,205u8,220u8,0u8,0u8,0u8,63u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,52u8,0u8,203u8,255u8,2u8,172u8,60u8,145u8,40u8,212u8,8u8,109u8,77u8,105u8,246u8,141u8,65u8,2u8,249u8,165u8,99u8,67u8,85u8,149u8,18u8,71u8,159u8,208u8,189u8,238u8,2u8,159u8,83u8,178u8,108u8,59u8,76u8,6u8,55u8,90u8,48u8,89u8,119u8,2u8,85u8,80u8,139u8,25u8,90u8,3u8,191u8,101u8,70u8,46u8,59u8,206u8,57u8,156u8,20u8,131u8,167u8,250u8,95u8,158u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_2_ct0_d16());model_filter_2_ct0_d16();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==6u32 && s.height==4u32 && s.rgba@.len()==96nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// filter/2-ct2-d8; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_filter_2_ct2_d8() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,2u8,0u8,0u8,0u8,34u8,102u8,217u8,20u8,0u8,0u8,0u8,87u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,76u8,0u8,179u8,255u8,2u8,225u8,86u8,108u8,113u8,142u8,93u8,40u8,51u8,108u8,3u8,173u8,54u8,38u8,227u8,123u8,0u8,188u8,178u8,2u8,49u8,120u8,114u8,6u8,123u8,84u8,173u8,126u8,247u8,170u8,52u8,129u8,0u8,136u8,37u8,230u8,222u8,94u8,2u8,97u8,73u8,29u8,149u8,23u8,39u8,25u8,3u8,232u8,1u8,247u8,126u8,39u8,110u8,121u8,156u8,59u8,116u8,2u8,165u8,153u8,173u8,243u8,109u8,135u8,134u8,57u8,65u8,241u8,17u8,138u8,82u8,245u8,195u8,147u8,193u8,111u8,143u8,224u8,32u8,127u8,97u8,105u8,89u8,130u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_filter_2_ct2_d8() -> Seq<u8> {seq![120u8,156u8,1u8,76u8,0u8,179u8,255u8,2u8,225u8,86u8,108u8,113u8,142u8,93u8,40u8,51u8,108u8,3u8,173u8,54u8,38u8,227u8,123u8,0u8,188u8,178u8,2u8,49u8,120u8,114u8,6u8,123u8,84u8,173u8,126u8,247u8,170u8,52u8,129u8,0u8,136u8,37u8,230u8,222u8,94u8,2u8,97u8,73u8,29u8,149u8,23u8,39u8,25u8,3u8,232u8,1u8,247u8,126u8,39u8,110u8,121u8,156u8,59u8,116u8,2u8,165u8,153u8,173u8,243u8,109u8,135u8,134u8,57u8,65u8,241u8,17u8,138u8,82u8,245u8,195u8,147u8,193u8,111u8,143u8,224u8,32u8,127u8]}
spec fn h_filter_2_ct2_d8() -> crate::model::Ihdr {crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:2u8,interlace:0u8}}
spec fn a_filter_2_ct2_d8() -> r::Raster {r::shape(h_filter_2_ct2_d8(),6nat,4nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_filter_2_ct2_d8(b: Seq<u8>)
    requires b.len()==144,crate::model::mh(b)==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:2u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:87nat,next:132nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,132nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:140nat,len:0nat,next:144nat,crc_ok:true,synthetic:false})),
        b.subrange(41,128)==z_filter_2_ct2_d8(),
    ensures r::default_case(b,crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:2u8,interlace:0u8},z_filter_2_ct2_d8(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_filter_2_ct2_d8() -> Seq<u8> {seq![2u8,225u8,86u8,108u8,113u8,142u8,93u8,40u8,51u8,108u8,3u8,173u8,54u8,38u8,227u8,123u8,0u8,188u8,178u8,2u8,49u8,120u8,114u8,6u8,123u8,84u8,173u8,126u8,247u8,170u8,52u8,129u8,0u8,136u8,37u8,230u8,222u8,94u8,2u8,97u8,73u8,29u8,149u8,23u8,39u8,25u8,3u8,232u8,1u8,247u8,126u8,39u8,110u8,121u8,156u8,59u8,116u8,2u8,165u8,153u8,173u8,243u8,109u8,135u8,134u8,57u8,65u8,241u8,17u8,138u8,82u8,245u8,195u8,147u8,193u8,111u8]}
#[verifier::spinoff_prover]
proof fn shape_filter_2_ct2_d8()
    ensures raw_filter_2_ct2_d8().len()==76nat,
        z_filter_2_ct2_d8().len()==87nat,
        z_filter_2_ct2_d8()[0]==120u8,
        z_filter_2_ct2_d8()[1]==156u8,
        zm::field_value(z_filter_2_ct2_d8(),16,1)==1nat,
        zm::field_value(z_filter_2_ct2_d8(),17,2)==0nat,
        zm::be_value(z_filter_2_ct2_d8(),83nat,4)==2413830271nat,
        z_filter_2_ct2_d8().subrange(7,83)==raw_filter_2_ct2_d8(),
        z_filter_2_ct2_d8()[3] as nat+256*z_filter_2_ct2_d8()[4] as nat==76nat,
        z_filter_2_ct2_d8()[5] as nat+256*z_filter_2_ct2_d8()[6] as nat==u16::MAX as nat-76nat,
        zm::be_value(z_filter_2_ct2_d8(),0,1)==120nat,
        zm::be_value(z_filter_2_ct2_d8(),1,1)==156nat,
{
    hide(z_filter_2_ct2_d8);hide(raw_filter_2_ct2_d8);
    assert(raw_filter_2_ct2_d8().len()==76nat) by (compute);
    assert(z_filter_2_ct2_d8().len()==87nat) by (compute);
    assert(z_filter_2_ct2_d8()[0]==120u8) by (compute);
    assert(z_filter_2_ct2_d8()[1]==156u8) by (compute);
    assert(zm::field_value(z_filter_2_ct2_d8(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_filter_2_ct2_d8(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_filter_2_ct2_d8(),83nat,4)==2413830271nat) by (compute);
    assert(z_filter_2_ct2_d8().subrange(7,83)==raw_filter_2_ct2_d8()) by (compute);
    assert(z_filter_2_ct2_d8()[3] as nat+256*z_filter_2_ct2_d8()[4] as nat==76nat) by (compute);
    assert(z_filter_2_ct2_d8()[5] as nat+256*z_filter_2_ct2_d8()[6] as nat==u16::MAX as nat-76nat) by (compute);
    assert(zm::be_value(z_filter_2_ct2_d8(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_filter_2_ct2_d8(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_filter_2_ct2_d8()
    ensures zm::Adler(raw_filter_2_ct2_d8())==2413830271nat,
{
    hide(raw_filter_2_ct2_d8);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_filter_2_ct2_d8().len()==76nat) by (compute);
    assert(c::sums(raw_filter_2_ct2_d8(),0,76nat)==(8318nat,298916nat)) by (compute);
    c::sums_correct(raw_filter_2_ct2_d8(),0,76nat);
    assert(raw_filter_2_ct2_d8().subrange(0,76) =~= raw_filter_2_ct2_d8());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((298916nat%65521)*(u16::MAX as nat+1)+(1+8318nat)%65521==2413830271nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_filter_2_ct2_d8()
    ensures r::inflated(z_filter_2_ct2_d8(),a_filter_2_ct2_d8(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_filter_2_ct2_d8(),r2:false}),
        raw_filter_2_ct2_d8().len()==76nat,
{
    hide(z_filter_2_ct2_d8);hide(raw_filter_2_ct2_d8);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_filter_2_ct2_d8();checksum_filter_2_ct2_d8();
    assert(r::raw(a_filter_2_ct2_d8())==76nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_filter_2_ct2_d8(),raw_filter_2_ct2_d8(),76nat,120nat,156nat,2413830271nat);
}
#[verifier::spinoff_prover]
proof fn scan_filter_2_ct2_d8()
    ensures r::scan(raw_filter_2_ct2_d8(),a_filter_2_ct2_d8(),0)==Ok(()),
{
    hide(raw_filter_2_ct2_d8);
    assert(r::base(a_filter_2_ct2_d8(),0)==0nat && r::ph(a_filter_2_ct2_d8(),0)==4nat && r::count(a_filter_2_ct2_d8())==1nat) by (compute);
    assert(r::rb(a_filter_2_ct2_d8(),0)==18nat) by (compute);
    assert(raw_filter_2_ct2_d8()[0]==2u8) by (compute);
    assert(r::row_result(raw_filter_2_ct2_d8(),a_filter_2_ct2_d8(),0,0nat)==Ok(()));
    assert(raw_filter_2_ct2_d8()[19]==2u8) by (compute);
    assert(r::row_result(raw_filter_2_ct2_d8(),a_filter_2_ct2_d8(),0,1nat)==Ok(()));
    assert(raw_filter_2_ct2_d8()[38]==2u8) by (compute);
    assert(r::row_result(raw_filter_2_ct2_d8(),a_filter_2_ct2_d8(),0,2nat)==Ok(()));
    assert(raw_filter_2_ct2_d8()[57]==2u8) by (compute);
    assert(r::row_result(raw_filter_2_ct2_d8(),a_filter_2_ct2_d8(),0,3nat)==Ok(()));
    r::pass_step(raw_filter_2_ct2_d8(),a_filter_2_ct2_d8(),0,4nat);
    r::pass_step(raw_filter_2_ct2_d8(),a_filter_2_ct2_d8(),0,3nat);
    r::pass_step(raw_filter_2_ct2_d8(),a_filter_2_ct2_d8(),0,2nat);
    r::pass_step(raw_filter_2_ct2_d8(),a_filter_2_ct2_d8(),0,1nat);
    r::pass_step(raw_filter_2_ct2_d8(),a_filter_2_ct2_d8(),0,0nat);
    r::scan_step(raw_filter_2_ct2_d8(),a_filter_2_ct2_d8(),1);
    r::scan_step(raw_filter_2_ct2_d8(),a_filter_2_ct2_d8(),0);
}
#[verifier::spinoff_prover]
proof fn model_filter_2_ct2_d8()
    ensures r::still(bytes_filter_2_ct2_d8(),STRICT,false)==(r::Projection::Ready {width:6u32,height:4u32,bytes:96nat}),
{
    hide(bytes_filter_2_ct2_d8);hide(z_filter_2_ct2_d8);hide(raw_filter_2_ct2_d8);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_2_ct2_d8().len()==144nat) by (compute);
    assert(crate::model::mh(bytes_filter_2_ct2_d8())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:2u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_2_ct2_d8())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:2u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_2_ct2_d8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:87nat,next:132nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_2_ct2_d8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:87nat,next:132nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_2_ct2_d8(),132nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:140nat,len:0nat,next:144nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_2_ct2_d8(),132nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:140nat,len:0nat,next:144nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_2_ct2_d8().subrange(41,128)==z_filter_2_ct2_d8()) by (compute);
    walk_filter_2_ct2_d8(bytes_filter_2_ct2_d8());
    inflate_filter_2_ct2_d8();scan_filter_2_ct2_d8();
    assert(r::raw(a_filter_2_ct2_d8())==76nat) by (compute);
    assert(r::data(z_filter_2_ct2_d8(),a_filter_2_ct2_d8(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_filter_2_ct2_d8(),h_filter_2_ct2_d8(),z_filter_2_ct2_d8(),0nat,false);
}
fn public_filter_2_ct2_d8() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,2u8,0u8,0u8,0u8,34u8,102u8,217u8,20u8,0u8,0u8,0u8,87u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,76u8,0u8,179u8,255u8,2u8,225u8,86u8,108u8,113u8,142u8,93u8,40u8,51u8,108u8,3u8,173u8,54u8,38u8,227u8,123u8,0u8,188u8,178u8,2u8,49u8,120u8,114u8,6u8,123u8,84u8,173u8,126u8,247u8,170u8,52u8,129u8,0u8,136u8,37u8,230u8,222u8,94u8,2u8,97u8,73u8,29u8,149u8,23u8,39u8,25u8,3u8,232u8,1u8,247u8,126u8,39u8,110u8,121u8,156u8,59u8,116u8,2u8,165u8,153u8,173u8,243u8,109u8,135u8,134u8,57u8,65u8,241u8,17u8,138u8,82u8,245u8,195u8,147u8,193u8,111u8,143u8,224u8,32u8,127u8,97u8,105u8,89u8,130u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_2_ct2_d8());model_filter_2_ct2_d8();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==6u32 && s.height==4u32 && s.rgba@.len()==96nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// filter/2-ct2-d16; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_filter_2_ct2_d16() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,2u8,0u8,0u8,0u8,114u8,246u8,5u8,87u8,0u8,0u8,0u8,159u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,148u8,0u8,107u8,255u8,2u8,123u8,232u8,14u8,117u8,227u8,29u8,189u8,148u8,84u8,34u8,38u8,235u8,241u8,77u8,173u8,11u8,245u8,64u8,232u8,216u8,117u8,196u8,255u8,82u8,141u8,53u8,24u8,157u8,138u8,134u8,191u8,219u8,55u8,75u8,159u8,224u8,2u8,87u8,119u8,158u8,80u8,26u8,145u8,19u8,169u8,83u8,26u8,184u8,171u8,216u8,90u8,228u8,134u8,183u8,10u8,246u8,222u8,10u8,94u8,5u8,197u8,147u8,12u8,199u8,127u8,83u8,61u8,221u8,24u8,125u8,217u8,57u8,90u8,2u8,222u8,160u8,246u8,157u8,63u8,101u8,147u8,38u8,100u8,171u8,97u8,126u8,42u8,154u8,87u8,47u8,203u8,196u8,182u8,74u8,89u8,62u8,25u8,248u8,113u8,243u8,243u8,189u8,70u8,184u8,181u8,159u8,127u8,149u8,34u8,166u8,2u8,207u8,202u8,237u8,155u8,126u8,74u8,18u8,35u8,155u8,29u8,34u8,119u8,207u8,79u8,27u8,29u8,23u8,56u8,131u8,165u8,127u8,60u8,206u8,225u8,108u8,205u8,74u8,140u8,94u8,39u8,189u8,228u8,118u8,138u8,21u8,17u8,24u8,91u8,71u8,39u8,214u8,217u8,147u8,126u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_filter_2_ct2_d16() -> Seq<u8> {seq![120u8,156u8,1u8,148u8,0u8,107u8,255u8,2u8,123u8,232u8,14u8,117u8,227u8,29u8,189u8,148u8,84u8,34u8,38u8,235u8,241u8,77u8,173u8,11u8,245u8,64u8,232u8,216u8,117u8,196u8,255u8,82u8,141u8,53u8,24u8,157u8,138u8,134u8,191u8,219u8,55u8,75u8,159u8,224u8,2u8,87u8,119u8,158u8,80u8,26u8,145u8,19u8,169u8,83u8,26u8,184u8,171u8,216u8,90u8,228u8,134u8,183u8,10u8,246u8,222u8,10u8,94u8,5u8,197u8,147u8,12u8,199u8,127u8,83u8,61u8,221u8,24u8,125u8,217u8,57u8,90u8,2u8,222u8,160u8,246u8,157u8,63u8,101u8,147u8,38u8,100u8,171u8,97u8,126u8,42u8,154u8,87u8,47u8,203u8,196u8,182u8,74u8,89u8,62u8,25u8,248u8,113u8,243u8,243u8,189u8,70u8,184u8,181u8,159u8,127u8,149u8,34u8,166u8,2u8,207u8,202u8,237u8,155u8,126u8,74u8,18u8,35u8,155u8,29u8,34u8,119u8,207u8,79u8,27u8,29u8,23u8,56u8,131u8,165u8,127u8,60u8,206u8,225u8,108u8,205u8,74u8,140u8,94u8,39u8,189u8,228u8,118u8,138u8,21u8,17u8,24u8,91u8,71u8,39u8]}
spec fn h_filter_2_ct2_d16() -> crate::model::Ihdr {crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:2u8,interlace:0u8}}
spec fn a_filter_2_ct2_d16() -> r::Raster {r::shape(h_filter_2_ct2_d16(),6nat,4nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_filter_2_ct2_d16(b: Seq<u8>)
    requires b.len()==216,crate::model::mh(b)==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:2u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:159nat,next:204nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,204nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:212nat,len:0nat,next:216nat,crc_ok:true,synthetic:false})),
        b.subrange(41,200)==z_filter_2_ct2_d16(),
    ensures r::default_case(b,crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:2u8,interlace:0u8},z_filter_2_ct2_d16(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
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
spec fn raw_filter_2_ct2_d16() -> Seq<u8> {seq![2u8,123u8,232u8,14u8,117u8,227u8,29u8,189u8,148u8,84u8,34u8,38u8,235u8,241u8,77u8,173u8,11u8,245u8,64u8,232u8,216u8,117u8,196u8,255u8,82u8,141u8,53u8,24u8,157u8,138u8,134u8,191u8,219u8,55u8,75u8,159u8,224u8,2u8,87u8,119u8,158u8,80u8,26u8,145u8,19u8,169u8,83u8,26u8,184u8,171u8,216u8,90u8,228u8,134u8,183u8,10u8,246u8,222u8,10u8,94u8,5u8,197u8,147u8,12u8,199u8,127u8,83u8,61u8,221u8,24u8,125u8,217u8,57u8,90u8,2u8,222u8,160u8,246u8,157u8,63u8,101u8,147u8,38u8,100u8,171u8,97u8,126u8,42u8,154u8,87u8,47u8,203u8,196u8,182u8,74u8,89u8,62u8,25u8,248u8,113u8,243u8,243u8,189u8,70u8,184u8,181u8,159u8,127u8,149u8,34u8,166u8,2u8,207u8,202u8,237u8,155u8,126u8,74u8,18u8,35u8,155u8,29u8,34u8,119u8,207u8,79u8,27u8,29u8,23u8,56u8,131u8,165u8,127u8,60u8,206u8,225u8,108u8,205u8,74u8,140u8,94u8,39u8,189u8,228u8,118u8,138u8,21u8,17u8]}
#[verifier::spinoff_prover]
proof fn shape_filter_2_ct2_d16()
    ensures raw_filter_2_ct2_d16().len()==148nat,
        z_filter_2_ct2_d16().len()==159nat,
        z_filter_2_ct2_d16()[0]==120u8,
        z_filter_2_ct2_d16()[1]==156u8,
        zm::field_value(z_filter_2_ct2_d16(),16,1)==1nat,
        zm::field_value(z_filter_2_ct2_d16(),17,2)==0nat,
        zm::be_value(z_filter_2_ct2_d16(),155nat,4)==408635175nat,
        z_filter_2_ct2_d16().subrange(7,155)==raw_filter_2_ct2_d16(),
        z_filter_2_ct2_d16()[3] as nat+256*z_filter_2_ct2_d16()[4] as nat==148nat,
        z_filter_2_ct2_d16()[5] as nat+256*z_filter_2_ct2_d16()[6] as nat==u16::MAX as nat-148nat,
        zm::be_value(z_filter_2_ct2_d16(),0,1)==120nat,
        zm::be_value(z_filter_2_ct2_d16(),1,1)==156nat,
{
    hide(z_filter_2_ct2_d16);hide(raw_filter_2_ct2_d16);
    assert(raw_filter_2_ct2_d16().len()==148nat) by (compute);
    assert(z_filter_2_ct2_d16().len()==159nat) by (compute);
    assert(z_filter_2_ct2_d16()[0]==120u8) by (compute);
    assert(z_filter_2_ct2_d16()[1]==156u8) by (compute);
    assert(zm::field_value(z_filter_2_ct2_d16(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_filter_2_ct2_d16(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_filter_2_ct2_d16(),155nat,4)==408635175nat) by (compute);
    assert(z_filter_2_ct2_d16().subrange(7,155)==raw_filter_2_ct2_d16()) by (compute);
    assert(z_filter_2_ct2_d16()[3] as nat+256*z_filter_2_ct2_d16()[4] as nat==148nat) by (compute);
    assert(z_filter_2_ct2_d16()[5] as nat+256*z_filter_2_ct2_d16()[6] as nat==u16::MAX as nat-148nat) by (compute);
    assert(zm::be_value(z_filter_2_ct2_d16(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_filter_2_ct2_d16(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_filter_2_ct2_d16()
    ensures zm::Adler(raw_filter_2_ct2_d16())==408635175nat,
{
    hide(raw_filter_2_ct2_d16);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_filter_2_ct2_d16().len()==148nat) by (compute);
    assert(c::sums(raw_filter_2_ct2_d16(),0,148nat)==(18214nat,1382176nat)) by (compute);
    c::sums_correct(raw_filter_2_ct2_d16(),0,148nat);
    assert(raw_filter_2_ct2_d16().subrange(0,148) =~= raw_filter_2_ct2_d16());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((1382176nat%65521)*(u16::MAX as nat+1)+(1+18214nat)%65521==408635175nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_filter_2_ct2_d16()
    ensures r::inflated(z_filter_2_ct2_d16(),a_filter_2_ct2_d16(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_filter_2_ct2_d16(),r2:false}),
        raw_filter_2_ct2_d16().len()==148nat,
{
    hide(z_filter_2_ct2_d16);hide(raw_filter_2_ct2_d16);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_filter_2_ct2_d16();checksum_filter_2_ct2_d16();
    assert(r::raw(a_filter_2_ct2_d16())==148nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_filter_2_ct2_d16(),raw_filter_2_ct2_d16(),148nat,120nat,156nat,408635175nat);
}
#[verifier::spinoff_prover]
proof fn scan_filter_2_ct2_d16()
    ensures r::scan(raw_filter_2_ct2_d16(),a_filter_2_ct2_d16(),0)==Ok(()),
{
    hide(raw_filter_2_ct2_d16);
    assert(r::base(a_filter_2_ct2_d16(),0)==0nat && r::ph(a_filter_2_ct2_d16(),0)==4nat && r::count(a_filter_2_ct2_d16())==1nat) by (compute);
    assert(r::rb(a_filter_2_ct2_d16(),0)==36nat) by (compute);
    assert(raw_filter_2_ct2_d16()[0]==2u8) by (compute);
    assert(r::row_result(raw_filter_2_ct2_d16(),a_filter_2_ct2_d16(),0,0nat)==Ok(()));
    assert(raw_filter_2_ct2_d16()[37]==2u8) by (compute);
    assert(r::row_result(raw_filter_2_ct2_d16(),a_filter_2_ct2_d16(),0,1nat)==Ok(()));
    assert(raw_filter_2_ct2_d16()[74]==2u8) by (compute);
    assert(r::row_result(raw_filter_2_ct2_d16(),a_filter_2_ct2_d16(),0,2nat)==Ok(()));
    assert(raw_filter_2_ct2_d16()[111]==2u8) by (compute);
    assert(r::row_result(raw_filter_2_ct2_d16(),a_filter_2_ct2_d16(),0,3nat)==Ok(()));
    r::pass_step(raw_filter_2_ct2_d16(),a_filter_2_ct2_d16(),0,4nat);
    r::pass_step(raw_filter_2_ct2_d16(),a_filter_2_ct2_d16(),0,3nat);
    r::pass_step(raw_filter_2_ct2_d16(),a_filter_2_ct2_d16(),0,2nat);
    r::pass_step(raw_filter_2_ct2_d16(),a_filter_2_ct2_d16(),0,1nat);
    r::pass_step(raw_filter_2_ct2_d16(),a_filter_2_ct2_d16(),0,0nat);
    r::scan_step(raw_filter_2_ct2_d16(),a_filter_2_ct2_d16(),1);
    r::scan_step(raw_filter_2_ct2_d16(),a_filter_2_ct2_d16(),0);
}
#[verifier::spinoff_prover]
proof fn model_filter_2_ct2_d16()
    ensures r::still(bytes_filter_2_ct2_d16(),STRICT,false)==(r::Projection::Ready {width:6u32,height:4u32,bytes:96nat}),
{
    hide(bytes_filter_2_ct2_d16);hide(z_filter_2_ct2_d16);hide(raw_filter_2_ct2_d16);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_2_ct2_d16().len()==216nat) by (compute);
    assert(crate::model::mh(bytes_filter_2_ct2_d16())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:2u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_2_ct2_d16())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:2u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_2_ct2_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:159nat,next:204nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_2_ct2_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:159nat,next:204nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_2_ct2_d16(),204nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:212nat,len:0nat,next:216nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_2_ct2_d16(),204nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:212nat,len:0nat,next:216nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_2_ct2_d16().subrange(41,200)==z_filter_2_ct2_d16()) by (compute);
    walk_filter_2_ct2_d16(bytes_filter_2_ct2_d16());
    inflate_filter_2_ct2_d16();scan_filter_2_ct2_d16();
    assert(r::raw(a_filter_2_ct2_d16())==148nat) by (compute);
    assert(r::data(z_filter_2_ct2_d16(),a_filter_2_ct2_d16(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_filter_2_ct2_d16(),h_filter_2_ct2_d16(),z_filter_2_ct2_d16(),0nat,false);
}
fn public_filter_2_ct2_d16() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,2u8,0u8,0u8,0u8,114u8,246u8,5u8,87u8,0u8,0u8,0u8,159u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,148u8,0u8,107u8,255u8,2u8,123u8,232u8,14u8,117u8,227u8,29u8,189u8,148u8,84u8,34u8,38u8,235u8,241u8,77u8,173u8,11u8,245u8,64u8,232u8,216u8,117u8,196u8,255u8,82u8,141u8,53u8,24u8,157u8,138u8,134u8,191u8,219u8,55u8,75u8,159u8,224u8,2u8,87u8,119u8,158u8,80u8,26u8,145u8,19u8,169u8,83u8,26u8,184u8,171u8,216u8,90u8,228u8,134u8,183u8,10u8,246u8,222u8,10u8,94u8,5u8,197u8,147u8,12u8,199u8,127u8,83u8,61u8,221u8,24u8,125u8,217u8,57u8,90u8,2u8,222u8,160u8,246u8,157u8,63u8,101u8,147u8,38u8,100u8,171u8,97u8,126u8,42u8,154u8,87u8,47u8,203u8,196u8,182u8,74u8,89u8,62u8,25u8,248u8,113u8,243u8,243u8,189u8,70u8,184u8,181u8,159u8,127u8,149u8,34u8,166u8,2u8,207u8,202u8,237u8,155u8,126u8,74u8,18u8,35u8,155u8,29u8,34u8,119u8,207u8,79u8,27u8,29u8,23u8,56u8,131u8,165u8,127u8,60u8,206u8,225u8,108u8,205u8,74u8,140u8,94u8,39u8,189u8,228u8,118u8,138u8,21u8,17u8,24u8,91u8,71u8,39u8,214u8,217u8,147u8,126u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_2_ct2_d16());model_filter_2_ct2_d16();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==6u32 && s.height==4u32 && s.rgba@.len()==96nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// filter/2-ct4-d8; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_filter_2_ct4_d8() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,4u8,0u8,0u8,0u8,7u8,13u8,134u8,200u8,0u8,0u8,0u8,63u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,52u8,0u8,203u8,255u8,2u8,26u8,146u8,68u8,149u8,181u8,33u8,132u8,45u8,142u8,77u8,18u8,143u8,2u8,174u8,239u8,6u8,24u8,129u8,147u8,141u8,132u8,165u8,34u8,221u8,120u8,2u8,234u8,210u8,96u8,217u8,224u8,231u8,54u8,210u8,16u8,73u8,8u8,18u8,2u8,114u8,113u8,157u8,122u8,14u8,196u8,229u8,77u8,188u8,215u8,127u8,147u8,62u8,101u8,23u8,103u8,90u8,111u8,248u8,112u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_filter_2_ct4_d8() -> Seq<u8> {seq![120u8,156u8,1u8,52u8,0u8,203u8,255u8,2u8,26u8,146u8,68u8,149u8,181u8,33u8,132u8,45u8,142u8,77u8,18u8,143u8,2u8,174u8,239u8,6u8,24u8,129u8,147u8,141u8,132u8,165u8,34u8,221u8,120u8,2u8,234u8,210u8,96u8,217u8,224u8,231u8,54u8,210u8,16u8,73u8,8u8,18u8,2u8,114u8,113u8,157u8,122u8,14u8,196u8,229u8,77u8,188u8,215u8,127u8,147u8,62u8,101u8,23u8,103u8]}
spec fn h_filter_2_ct4_d8() -> crate::model::Ihdr {crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:4u8,interlace:0u8}}
spec fn a_filter_2_ct4_d8() -> r::Raster {r::shape(h_filter_2_ct4_d8(),6nat,4nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_filter_2_ct4_d8(b: Seq<u8>)
    requires b.len()==120,crate::model::mh(b)==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:4u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:63nat,next:108nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,108nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:116nat,len:0nat,next:120nat,crc_ok:true,synthetic:false})),
        b.subrange(41,104)==z_filter_2_ct4_d8(),
    ensures r::default_case(b,crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:4u8,interlace:0u8},z_filter_2_ct4_d8(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_filter_2_ct4_d8() -> Seq<u8> {seq![2u8,26u8,146u8,68u8,149u8,181u8,33u8,132u8,45u8,142u8,77u8,18u8,143u8,2u8,174u8,239u8,6u8,24u8,129u8,147u8,141u8,132u8,165u8,34u8,221u8,120u8,2u8,234u8,210u8,96u8,217u8,224u8,231u8,54u8,210u8,16u8,73u8,8u8,18u8,2u8,114u8,113u8,157u8,122u8,14u8,196u8,229u8,77u8,188u8,215u8,127u8,147u8]}
#[verifier::spinoff_prover]
proof fn shape_filter_2_ct4_d8()
    ensures raw_filter_2_ct4_d8().len()==52nat,
        z_filter_2_ct4_d8().len()==63nat,
        z_filter_2_ct4_d8()[0]==120u8,
        z_filter_2_ct4_d8()[1]==156u8,
        zm::field_value(z_filter_2_ct4_d8(),16,1)==1nat,
        zm::field_value(z_filter_2_ct4_d8(),17,2)==0nat,
        zm::be_value(z_filter_2_ct4_d8(),59nat,4)==1046812519nat,
        z_filter_2_ct4_d8().subrange(7,59)==raw_filter_2_ct4_d8(),
        z_filter_2_ct4_d8()[3] as nat+256*z_filter_2_ct4_d8()[4] as nat==52nat,
        z_filter_2_ct4_d8()[5] as nat+256*z_filter_2_ct4_d8()[6] as nat==u16::MAX as nat-52nat,
        zm::be_value(z_filter_2_ct4_d8(),0,1)==120nat,
        zm::be_value(z_filter_2_ct4_d8(),1,1)==156nat,
{
    hide(z_filter_2_ct4_d8);hide(raw_filter_2_ct4_d8);
    assert(raw_filter_2_ct4_d8().len()==52nat) by (compute);
    assert(z_filter_2_ct4_d8().len()==63nat) by (compute);
    assert(z_filter_2_ct4_d8()[0]==120u8) by (compute);
    assert(z_filter_2_ct4_d8()[1]==156u8) by (compute);
    assert(zm::field_value(z_filter_2_ct4_d8(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_filter_2_ct4_d8(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_filter_2_ct4_d8(),59nat,4)==1046812519nat) by (compute);
    assert(z_filter_2_ct4_d8().subrange(7,59)==raw_filter_2_ct4_d8()) by (compute);
    assert(z_filter_2_ct4_d8()[3] as nat+256*z_filter_2_ct4_d8()[4] as nat==52nat) by (compute);
    assert(z_filter_2_ct4_d8()[5] as nat+256*z_filter_2_ct4_d8()[6] as nat==u16::MAX as nat-52nat) by (compute);
    assert(zm::be_value(z_filter_2_ct4_d8(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_filter_2_ct4_d8(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_filter_2_ct4_d8()
    ensures zm::Adler(raw_filter_2_ct4_d8())==1046812519nat,
{
    hide(raw_filter_2_ct4_d8);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_filter_2_ct4_d8().len()==52nat) by (compute);
    assert(c::sums(raw_filter_2_ct4_d8(),0,52nat)==(5990nat,147015nat)) by (compute);
    c::sums_correct(raw_filter_2_ct4_d8(),0,52nat);
    assert(raw_filter_2_ct4_d8().subrange(0,52) =~= raw_filter_2_ct4_d8());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((147015nat%65521)*(u16::MAX as nat+1)+(1+5990nat)%65521==1046812519nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_filter_2_ct4_d8()
    ensures r::inflated(z_filter_2_ct4_d8(),a_filter_2_ct4_d8(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_filter_2_ct4_d8(),r2:false}),
        raw_filter_2_ct4_d8().len()==52nat,
{
    hide(z_filter_2_ct4_d8);hide(raw_filter_2_ct4_d8);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_filter_2_ct4_d8();checksum_filter_2_ct4_d8();
    assert(r::raw(a_filter_2_ct4_d8())==52nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_filter_2_ct4_d8(),raw_filter_2_ct4_d8(),52nat,120nat,156nat,1046812519nat);
}
#[verifier::spinoff_prover]
proof fn scan_filter_2_ct4_d8()
    ensures r::scan(raw_filter_2_ct4_d8(),a_filter_2_ct4_d8(),0)==Ok(()),
{
    hide(raw_filter_2_ct4_d8);
    assert(r::base(a_filter_2_ct4_d8(),0)==0nat && r::ph(a_filter_2_ct4_d8(),0)==4nat && r::count(a_filter_2_ct4_d8())==1nat) by (compute);
    assert(r::rb(a_filter_2_ct4_d8(),0)==12nat) by (compute);
    assert(raw_filter_2_ct4_d8()[0]==2u8) by (compute);
    assert(r::row_result(raw_filter_2_ct4_d8(),a_filter_2_ct4_d8(),0,0nat)==Ok(()));
    assert(raw_filter_2_ct4_d8()[13]==2u8) by (compute);
    assert(r::row_result(raw_filter_2_ct4_d8(),a_filter_2_ct4_d8(),0,1nat)==Ok(()));
    assert(raw_filter_2_ct4_d8()[26]==2u8) by (compute);
    assert(r::row_result(raw_filter_2_ct4_d8(),a_filter_2_ct4_d8(),0,2nat)==Ok(()));
    assert(raw_filter_2_ct4_d8()[39]==2u8) by (compute);
    assert(r::row_result(raw_filter_2_ct4_d8(),a_filter_2_ct4_d8(),0,3nat)==Ok(()));
    r::pass_step(raw_filter_2_ct4_d8(),a_filter_2_ct4_d8(),0,4nat);
    r::pass_step(raw_filter_2_ct4_d8(),a_filter_2_ct4_d8(),0,3nat);
    r::pass_step(raw_filter_2_ct4_d8(),a_filter_2_ct4_d8(),0,2nat);
    r::pass_step(raw_filter_2_ct4_d8(),a_filter_2_ct4_d8(),0,1nat);
    r::pass_step(raw_filter_2_ct4_d8(),a_filter_2_ct4_d8(),0,0nat);
    r::scan_step(raw_filter_2_ct4_d8(),a_filter_2_ct4_d8(),1);
    r::scan_step(raw_filter_2_ct4_d8(),a_filter_2_ct4_d8(),0);
}
#[verifier::spinoff_prover]
proof fn model_filter_2_ct4_d8()
    ensures r::still(bytes_filter_2_ct4_d8(),STRICT,false)==(r::Projection::Ready {width:6u32,height:4u32,bytes:96nat}),
{
    hide(bytes_filter_2_ct4_d8);hide(z_filter_2_ct4_d8);hide(raw_filter_2_ct4_d8);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_2_ct4_d8().len()==120nat) by (compute);
    assert(crate::model::mh(bytes_filter_2_ct4_d8())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:4u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_2_ct4_d8())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:4u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_2_ct4_d8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:63nat,next:108nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_2_ct4_d8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:63nat,next:108nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_2_ct4_d8(),108nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:116nat,len:0nat,next:120nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_2_ct4_d8(),108nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:116nat,len:0nat,next:120nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_2_ct4_d8().subrange(41,104)==z_filter_2_ct4_d8()) by (compute);
    walk_filter_2_ct4_d8(bytes_filter_2_ct4_d8());
    inflate_filter_2_ct4_d8();scan_filter_2_ct4_d8();
    assert(r::raw(a_filter_2_ct4_d8())==52nat) by (compute);
    assert(r::data(z_filter_2_ct4_d8(),a_filter_2_ct4_d8(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_filter_2_ct4_d8(),h_filter_2_ct4_d8(),z_filter_2_ct4_d8(),0nat,false);
}
fn public_filter_2_ct4_d8() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,4u8,0u8,0u8,0u8,7u8,13u8,134u8,200u8,0u8,0u8,0u8,63u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,52u8,0u8,203u8,255u8,2u8,26u8,146u8,68u8,149u8,181u8,33u8,132u8,45u8,142u8,77u8,18u8,143u8,2u8,174u8,239u8,6u8,24u8,129u8,147u8,141u8,132u8,165u8,34u8,221u8,120u8,2u8,234u8,210u8,96u8,217u8,224u8,231u8,54u8,210u8,16u8,73u8,8u8,18u8,2u8,114u8,113u8,157u8,122u8,14u8,196u8,229u8,77u8,188u8,215u8,127u8,147u8,62u8,101u8,23u8,103u8,90u8,111u8,248u8,112u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_2_ct4_d8());model_filter_2_ct4_d8();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==6u32 && s.height==4u32 && s.rgba@.len()==96nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// filter/2-ct6-d16; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_filter_2_ct6_d16() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,6u8,0u8,0u8,0u8,253u8,148u8,146u8,0u8,0u8,0u8,0u8,207u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,196u8,0u8,59u8,255u8,2u8,204u8,62u8,150u8,240u8,56u8,153u8,215u8,220u8,215u8,138u8,106u8,61u8,193u8,115u8,2u8,178u8,132u8,199u8,101u8,58u8,168u8,177u8,174u8,85u8,101u8,84u8,109u8,107u8,55u8,80u8,152u8,147u8,138u8,118u8,111u8,226u8,12u8,163u8,31u8,170u8,18u8,157u8,7u8,113u8,154u8,118u8,183u8,2u8,2u8,80u8,63u8,89u8,184u8,145u8,228u8,76u8,27u8,94u8,195u8,195u8,17u8,202u8,196u8,67u8,72u8,8u8,92u8,200u8,49u8,150u8,214u8,208u8,246u8,211u8,190u8,37u8,237u8,146u8,50u8,249u8,22u8,106u8,138u8,145u8,180u8,146u8,92u8,182u8,168u8,78u8,56u8,56u8,41u8,158u8,219u8,3u8,90u8,2u8,101u8,0u8,20u8,114u8,46u8,1u8,12u8,84u8,153u8,226u8,110u8,132u8,21u8,169u8,76u8,239u8,120u8,37u8,22u8,1u8,120u8,230u8,185u8,183u8,135u8,143u8,174u8,128u8,145u8,207u8,237u8,133u8,253u8,53u8,83u8,5u8,176u8,245u8,163u8,4u8,241u8,87u8,147u8,191u8,211u8,165u8,128u8,188u8,2u8,112u8,104u8,235u8,149u8,242u8,46u8,186u8,71u8,108u8,244u8,113u8,151u8,70u8,253u8,21u8,204u8,71u8,233u8,195u8,58u8,187u8,168u8,155u8,102u8,218u8,90u8,236u8,233u8,131u8,251u8,187u8,177u8,43u8,150u8,197u8,168u8,174u8,189u8,130u8,188u8,199u8,231u8,92u8,204u8,11u8,115u8,210u8,160u8,109u8,25u8,99u8,226u8,254u8,245u8,154u8,188u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_filter_2_ct6_d16() -> Seq<u8> {seq![120u8,156u8,1u8,196u8,0u8,59u8,255u8,2u8,204u8,62u8,150u8,240u8,56u8,153u8,215u8,220u8,215u8,138u8,106u8,61u8,193u8,115u8,2u8,178u8,132u8,199u8,101u8,58u8,168u8,177u8,174u8,85u8,101u8,84u8,109u8,107u8,55u8,80u8,152u8,147u8,138u8,118u8,111u8,226u8,12u8,163u8,31u8,170u8,18u8,157u8,7u8,113u8,154u8,118u8,183u8,2u8,2u8,80u8,63u8,89u8,184u8,145u8,228u8,76u8,27u8,94u8,195u8,195u8,17u8,202u8,196u8,67u8,72u8,8u8,92u8,200u8,49u8,150u8,214u8,208u8,246u8,211u8,190u8,37u8,237u8,146u8,50u8,249u8,22u8,106u8,138u8,145u8,180u8,146u8,92u8,182u8,168u8,78u8,56u8,56u8,41u8,158u8,219u8,3u8,90u8,2u8,101u8,0u8,20u8,114u8,46u8,1u8,12u8,84u8,153u8,226u8,110u8,132u8,21u8,169u8,76u8,239u8,120u8,37u8,22u8,1u8,120u8,230u8,185u8,183u8,135u8,143u8,174u8,128u8,145u8,207u8,237u8,133u8,253u8,53u8,83u8,5u8,176u8,245u8,163u8,4u8,241u8,87u8,147u8,191u8,211u8,165u8,128u8,188u8,2u8,112u8,104u8,235u8,149u8,242u8,46u8,186u8,71u8,108u8,244u8,113u8,151u8,70u8,253u8,21u8,204u8,71u8,233u8,195u8,58u8,187u8,168u8,155u8,102u8,218u8,90u8,236u8,233u8,131u8,251u8,187u8,177u8,43u8,150u8,197u8,168u8,174u8,189u8,130u8,188u8,199u8,231u8,92u8,204u8,11u8,115u8,210u8,160u8,109u8,25u8,99u8,226u8]}
spec fn h_filter_2_ct6_d16() -> crate::model::Ihdr {crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:6u8,interlace:0u8}}
spec fn a_filter_2_ct6_d16() -> r::Raster {r::shape(h_filter_2_ct6_d16(),6nat,4nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_filter_2_ct6_d16(b: Seq<u8>)
    requires b.len()==264,crate::model::mh(b)==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:6u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:207nat,next:252nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,252nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:260nat,len:0nat,next:264nat,crc_ok:true,synthetic:false})),
        b.subrange(41,248)==z_filter_2_ct6_d16(),
    ensures r::default_case(b,crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:6u8,interlace:0u8},z_filter_2_ct6_d16(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
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
spec fn raw_filter_2_ct6_d16() -> Seq<u8> {seq![2u8,204u8,62u8,150u8,240u8,56u8,153u8,215u8,220u8,215u8,138u8,106u8,61u8,193u8,115u8,2u8,178u8,132u8,199u8,101u8,58u8,168u8,177u8,174u8,85u8,101u8,84u8,109u8,107u8,55u8,80u8,152u8,147u8,138u8,118u8,111u8,226u8,12u8,163u8,31u8,170u8,18u8,157u8,7u8,113u8,154u8,118u8,183u8,2u8,2u8,80u8,63u8,89u8,184u8,145u8,228u8,76u8,27u8,94u8,195u8,195u8,17u8,202u8,196u8,67u8,72u8,8u8,92u8,200u8,49u8,150u8,214u8,208u8,246u8,211u8,190u8,37u8,237u8,146u8,50u8,249u8,22u8,106u8,138u8,145u8,180u8,146u8,92u8,182u8,168u8,78u8,56u8,56u8,41u8,158u8,219u8,3u8,90u8,2u8,101u8,0u8,20u8,114u8,46u8,1u8,12u8,84u8,153u8,226u8,110u8,132u8,21u8,169u8,76u8,239u8,120u8,37u8,22u8,1u8,120u8,230u8,185u8,183u8,135u8,143u8,174u8,128u8,145u8,207u8,237u8,133u8,253u8,53u8,83u8,5u8,176u8,245u8,163u8,4u8,241u8,87u8,147u8,191u8,211u8,165u8,128u8,188u8,2u8,112u8,104u8,235u8,149u8,242u8,46u8,186u8,71u8,108u8,244u8,113u8,151u8,70u8,253u8,21u8,204u8,71u8,233u8,195u8,58u8,187u8,168u8,155u8,102u8,218u8,90u8,236u8,233u8,131u8,251u8,187u8,177u8,43u8,150u8,197u8,168u8,174u8,189u8,130u8,188u8,199u8,231u8,92u8,204u8,11u8,115u8,210u8,160u8]}
#[verifier::spinoff_prover]
proof fn shape_filter_2_ct6_d16()
    ensures raw_filter_2_ct6_d16().len()==196nat,
        z_filter_2_ct6_d16().len()==207nat,
        z_filter_2_ct6_d16()[0]==120u8,
        z_filter_2_ct6_d16()[1]==156u8,
        zm::field_value(z_filter_2_ct6_d16(),16,1)==1nat,
        zm::field_value(z_filter_2_ct6_d16(),17,2)==0nat,
        zm::be_value(z_filter_2_ct6_d16(),203nat,4)==1830380514nat,
        z_filter_2_ct6_d16().subrange(7,203)==raw_filter_2_ct6_d16(),
        z_filter_2_ct6_d16()[3] as nat+256*z_filter_2_ct6_d16()[4] as nat==196nat,
        z_filter_2_ct6_d16()[5] as nat+256*z_filter_2_ct6_d16()[6] as nat==u16::MAX as nat-196nat,
        zm::be_value(z_filter_2_ct6_d16(),0,1)==120nat,
        zm::be_value(z_filter_2_ct6_d16(),1,1)==156nat,
{
    hide(z_filter_2_ct6_d16);hide(raw_filter_2_ct6_d16);
    assert(raw_filter_2_ct6_d16().len()==196nat) by (compute);
    assert(z_filter_2_ct6_d16().len()==207nat) by (compute);
    assert(z_filter_2_ct6_d16()[0]==120u8) by (compute);
    assert(z_filter_2_ct6_d16()[1]==156u8) by (compute);
    assert(zm::field_value(z_filter_2_ct6_d16(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_filter_2_ct6_d16(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_filter_2_ct6_d16(),203nat,4)==1830380514nat) by (compute);
    assert(z_filter_2_ct6_d16().subrange(7,203)==raw_filter_2_ct6_d16()) by (compute);
    assert(z_filter_2_ct6_d16()[3] as nat+256*z_filter_2_ct6_d16()[4] as nat==196nat) by (compute);
    assert(z_filter_2_ct6_d16()[5] as nat+256*z_filter_2_ct6_d16()[6] as nat==u16::MAX as nat-196nat) by (compute);
    assert(zm::be_value(z_filter_2_ct6_d16(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_filter_2_ct6_d16(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_filter_2_ct6_d16()
    ensures zm::Adler(raw_filter_2_ct6_d16())==1830380514nat,
{
    hide(raw_filter_2_ct6_d16);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_filter_2_ct6_d16().len()==196nat) by (compute);
    assert(c::sums(raw_filter_2_ct6_d16(),0,196nat)==(25569nat,2386685nat)) by (compute);
    c::sums_correct(raw_filter_2_ct6_d16(),0,196nat);
    assert(raw_filter_2_ct6_d16().subrange(0,196) =~= raw_filter_2_ct6_d16());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((2386685nat%65521)*(u16::MAX as nat+1)+(1+25569nat)%65521==1830380514nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_filter_2_ct6_d16()
    ensures r::inflated(z_filter_2_ct6_d16(),a_filter_2_ct6_d16(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_filter_2_ct6_d16(),r2:false}),
        raw_filter_2_ct6_d16().len()==196nat,
{
    hide(z_filter_2_ct6_d16);hide(raw_filter_2_ct6_d16);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_filter_2_ct6_d16();checksum_filter_2_ct6_d16();
    assert(r::raw(a_filter_2_ct6_d16())==196nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_filter_2_ct6_d16(),raw_filter_2_ct6_d16(),196nat,120nat,156nat,1830380514nat);
}
#[verifier::spinoff_prover]
proof fn scan_filter_2_ct6_d16()
    ensures r::scan(raw_filter_2_ct6_d16(),a_filter_2_ct6_d16(),0)==Ok(()),
{
    hide(raw_filter_2_ct6_d16);
    assert(r::base(a_filter_2_ct6_d16(),0)==0nat && r::ph(a_filter_2_ct6_d16(),0)==4nat && r::count(a_filter_2_ct6_d16())==1nat) by (compute);
    assert(r::rb(a_filter_2_ct6_d16(),0)==48nat) by (compute);
    assert(raw_filter_2_ct6_d16()[0]==2u8) by (compute);
    assert(r::row_result(raw_filter_2_ct6_d16(),a_filter_2_ct6_d16(),0,0nat)==Ok(()));
    assert(raw_filter_2_ct6_d16()[49]==2u8) by (compute);
    assert(r::row_result(raw_filter_2_ct6_d16(),a_filter_2_ct6_d16(),0,1nat)==Ok(()));
    assert(raw_filter_2_ct6_d16()[98]==2u8) by (compute);
    assert(r::row_result(raw_filter_2_ct6_d16(),a_filter_2_ct6_d16(),0,2nat)==Ok(()));
    assert(raw_filter_2_ct6_d16()[147]==2u8) by (compute);
    assert(r::row_result(raw_filter_2_ct6_d16(),a_filter_2_ct6_d16(),0,3nat)==Ok(()));
    r::pass_step(raw_filter_2_ct6_d16(),a_filter_2_ct6_d16(),0,4nat);
    r::pass_step(raw_filter_2_ct6_d16(),a_filter_2_ct6_d16(),0,3nat);
    r::pass_step(raw_filter_2_ct6_d16(),a_filter_2_ct6_d16(),0,2nat);
    r::pass_step(raw_filter_2_ct6_d16(),a_filter_2_ct6_d16(),0,1nat);
    r::pass_step(raw_filter_2_ct6_d16(),a_filter_2_ct6_d16(),0,0nat);
    r::scan_step(raw_filter_2_ct6_d16(),a_filter_2_ct6_d16(),1);
    r::scan_step(raw_filter_2_ct6_d16(),a_filter_2_ct6_d16(),0);
}
#[verifier::spinoff_prover]
proof fn model_filter_2_ct6_d16()
    ensures r::still(bytes_filter_2_ct6_d16(),STRICT,false)==(r::Projection::Ready {width:6u32,height:4u32,bytes:96nat}),
{
    hide(bytes_filter_2_ct6_d16);hide(z_filter_2_ct6_d16);hide(raw_filter_2_ct6_d16);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_2_ct6_d16().len()==264nat) by (compute);
    assert(crate::model::mh(bytes_filter_2_ct6_d16())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:6u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_2_ct6_d16())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:6u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_2_ct6_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:207nat,next:252nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_2_ct6_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:207nat,next:252nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_2_ct6_d16(),252nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:260nat,len:0nat,next:264nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_2_ct6_d16(),252nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:260nat,len:0nat,next:264nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_2_ct6_d16().subrange(41,248)==z_filter_2_ct6_d16()) by (compute);
    walk_filter_2_ct6_d16(bytes_filter_2_ct6_d16());
    inflate_filter_2_ct6_d16();scan_filter_2_ct6_d16();
    assert(r::raw(a_filter_2_ct6_d16())==196nat) by (compute);
    assert(r::data(z_filter_2_ct6_d16(),a_filter_2_ct6_d16(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_filter_2_ct6_d16(),h_filter_2_ct6_d16(),z_filter_2_ct6_d16(),0nat,false);
}
fn public_filter_2_ct6_d16() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,6u8,0u8,0u8,0u8,253u8,148u8,146u8,0u8,0u8,0u8,0u8,207u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,196u8,0u8,59u8,255u8,2u8,204u8,62u8,150u8,240u8,56u8,153u8,215u8,220u8,215u8,138u8,106u8,61u8,193u8,115u8,2u8,178u8,132u8,199u8,101u8,58u8,168u8,177u8,174u8,85u8,101u8,84u8,109u8,107u8,55u8,80u8,152u8,147u8,138u8,118u8,111u8,226u8,12u8,163u8,31u8,170u8,18u8,157u8,7u8,113u8,154u8,118u8,183u8,2u8,2u8,80u8,63u8,89u8,184u8,145u8,228u8,76u8,27u8,94u8,195u8,195u8,17u8,202u8,196u8,67u8,72u8,8u8,92u8,200u8,49u8,150u8,214u8,208u8,246u8,211u8,190u8,37u8,237u8,146u8,50u8,249u8,22u8,106u8,138u8,145u8,180u8,146u8,92u8,182u8,168u8,78u8,56u8,56u8,41u8,158u8,219u8,3u8,90u8,2u8,101u8,0u8,20u8,114u8,46u8,1u8,12u8,84u8,153u8,226u8,110u8,132u8,21u8,169u8,76u8,239u8,120u8,37u8,22u8,1u8,120u8,230u8,185u8,183u8,135u8,143u8,174u8,128u8,145u8,207u8,237u8,133u8,253u8,53u8,83u8,5u8,176u8,245u8,163u8,4u8,241u8,87u8,147u8,191u8,211u8,165u8,128u8,188u8,2u8,112u8,104u8,235u8,149u8,242u8,46u8,186u8,71u8,108u8,244u8,113u8,151u8,70u8,253u8,21u8,204u8,71u8,233u8,195u8,58u8,187u8,168u8,155u8,102u8,218u8,90u8,236u8,233u8,131u8,251u8,187u8,177u8,43u8,150u8,197u8,168u8,174u8,189u8,130u8,188u8,199u8,231u8,92u8,204u8,11u8,115u8,210u8,160u8,109u8,25u8,99u8,226u8,254u8,245u8,154u8,188u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_2_ct6_d16());model_filter_2_ct6_d16();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==6u32 && s.height==4u32 && s.rgba@.len()==96nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// filter/3-ct0-d1; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_filter_3_ct0_d1() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,1u8,0u8,0u8,0u8,0u8,133u8,127u8,115u8,238u8,0u8,0u8,0u8,16u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,246u8,97u8,102u8,98u8,174u8,97u8,14u8,1u8,0u8,4u8,42u8,1u8,43u8,217u8,76u8,137u8,10u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_filter_3_ct0_d1() -> Seq<u8> {seq![120u8,156u8,99u8,246u8,97u8,102u8,98u8,174u8,97u8,14u8,1u8,0u8,4u8,42u8,1u8,43u8]}
spec fn h_filter_3_ct0_d1() -> crate::model::Ihdr {crate::model::Ihdr {width:6u32,height:4u32,depth:1u8,color_type:0u8,interlace:0u8}}
spec fn a_filter_3_ct0_d1() -> r::Raster {r::shape(h_filter_3_ct0_d1(),6nat,4nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_filter_3_ct0_d1(b: Seq<u8>)
    requires b.len()==73,crate::model::mh(b)==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:1u8,color_type:0u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:16nat,next:61nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,61nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:69nat,len:0nat,next:73nat,crc_ok:true,synthetic:false})),
        b.subrange(41,57)==z_filter_3_ct0_d1(),
    ensures r::default_case(b,crate::model::Ihdr {width:6u32,height:4u32,depth:1u8,color_type:0u8,interlace:0u8},z_filter_3_ct0_d1(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
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
spec fn raw_filter_3_ct0_d1() -> Seq<u8> {seq![3u8,76u8,3u8,2u8,3u8,124u8,3u8,84u8]}
spec fn ts_filter_3_ct0_d1() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:3nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:35nat,before:1nat,after:2nat,symbol:76nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:35nat,next:43nat,before:2nat,after:3nat,symbol:3nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:43nat,next:51nat,before:3nat,after:4nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:51nat,next:59nat,before:4nat,after:5nat,symbol:3nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:59nat,next:67nat,before:5nat,after:6nat,symbol:124nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:67nat,next:75nat,before:6nat,after:7nat,symbol:3nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:75nat,next:83nat,before:7nat,after:8nat,symbol:84nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:83nat,next:90nat,before:8nat,after:8nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_filter_3_ct0_d1()
    ensures t::trace(z_filter_3_ct0_d1(),raw_filter_3_ct0_d1(),ts_filter_3_ct0_d1(),0,9nat),
{
    hide(z_filter_3_ct0_d1);hide(raw_filter_3_ct0_d1);hide(ts_filter_3_ct0_d1);hide(t::valid_step);
    assert(t::trace(z_filter_3_ct0_d1(),raw_filter_3_ct0_d1(),ts_filter_3_ct0_d1(),0,9nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_filter_3_ct0_d1()
    ensures raw_filter_3_ct0_d1().len()==8nat,
        z_filter_3_ct0_d1().len()==16nat,
        z_filter_3_ct0_d1()[0]==120u8,
        z_filter_3_ct0_d1()[1]==156u8,
        zm::field_value(z_filter_3_ct0_d1(),16,1)==1nat,
        zm::field_value(z_filter_3_ct0_d1(),17,2)==1nat,
        zm::be_value(z_filter_3_ct0_d1(),12nat,4)==69861675nat,
        ts_filter_3_ct0_d1().len()==9nat,
        ts_filter_3_ct0_d1()[0].pos==19nat,
        ts_filter_3_ct0_d1()[0].before==0nat,
        ts_filter_3_ct0_d1().last().next==90nat,
{
    hide(z_filter_3_ct0_d1);hide(raw_filter_3_ct0_d1);hide(ts_filter_3_ct0_d1);
    assert(raw_filter_3_ct0_d1().len()==8nat) by (compute);
    assert(z_filter_3_ct0_d1().len()==16nat) by (compute);
    assert(z_filter_3_ct0_d1()[0]==120u8) by (compute);
    assert(z_filter_3_ct0_d1()[1]==156u8) by (compute);
    assert(zm::field_value(z_filter_3_ct0_d1(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_filter_3_ct0_d1(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_filter_3_ct0_d1(),12nat,4)==69861675nat) by (compute);
    assert(ts_filter_3_ct0_d1().len()==9nat) by (compute);
    assert(ts_filter_3_ct0_d1()[0].pos==19nat) by (compute);
    assert(ts_filter_3_ct0_d1()[0].before==0nat) by (compute);
    assert(ts_filter_3_ct0_d1().last().next==90nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_filter_3_ct0_d1()
    ensures zm::Adler(raw_filter_3_ct0_d1())==69861675nat,
{
    hide(raw_filter_3_ct0_d1);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_filter_3_ct0_d1().len()==8nat) by (compute);
    assert(c::sums(raw_filter_3_ct0_d1(),0,8nat)==(298nat,1066nat)) by (compute);
    c::sums_correct(raw_filter_3_ct0_d1(),0,8nat);
    assert(raw_filter_3_ct0_d1().subrange(0,8) =~= raw_filter_3_ct0_d1());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((1066nat%65521)*(u16::MAX as nat+1)+(1+298nat)%65521==69861675nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_filter_3_ct0_d1()
    ensures r::inflated(z_filter_3_ct0_d1(),a_filter_3_ct0_d1(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_filter_3_ct0_d1(),r2:false}),
        raw_filter_3_ct0_d1().len()==8nat,
{
    hide(z_filter_3_ct0_d1);hide(raw_filter_3_ct0_d1);hide(ts_filter_3_ct0_d1);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_filter_3_ct0_d1();checksum_filter_3_ct0_d1();
    assert(r::raw(a_filter_3_ct0_d1())==8nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_filter_3_ct0_d1();
    stream::fixed_stream(z_filter_3_ct0_d1(),raw_filter_3_ct0_d1(),ts_filter_3_ct0_d1(),8nat,156nat,69861675nat);
}
#[verifier::spinoff_prover]
proof fn scan_filter_3_ct0_d1()
    ensures r::scan(raw_filter_3_ct0_d1(),a_filter_3_ct0_d1(),0)==Ok(()),
{
    hide(raw_filter_3_ct0_d1);
    assert(r::base(a_filter_3_ct0_d1(),0)==0nat && r::ph(a_filter_3_ct0_d1(),0)==4nat && r::count(a_filter_3_ct0_d1())==1nat) by (compute);
    assert(r::rb(a_filter_3_ct0_d1(),0)==1nat) by (compute);
    assert(raw_filter_3_ct0_d1()[0]==3u8) by (compute);
    assert(r::row_result(raw_filter_3_ct0_d1(),a_filter_3_ct0_d1(),0,0nat)==Ok(()));
    assert(raw_filter_3_ct0_d1()[2]==3u8) by (compute);
    assert(r::row_result(raw_filter_3_ct0_d1(),a_filter_3_ct0_d1(),0,1nat)==Ok(()));
    assert(raw_filter_3_ct0_d1()[4]==3u8) by (compute);
    assert(r::row_result(raw_filter_3_ct0_d1(),a_filter_3_ct0_d1(),0,2nat)==Ok(()));
    assert(raw_filter_3_ct0_d1()[6]==3u8) by (compute);
    assert(r::row_result(raw_filter_3_ct0_d1(),a_filter_3_ct0_d1(),0,3nat)==Ok(()));
    r::pass_step(raw_filter_3_ct0_d1(),a_filter_3_ct0_d1(),0,4nat);
    r::pass_step(raw_filter_3_ct0_d1(),a_filter_3_ct0_d1(),0,3nat);
    r::pass_step(raw_filter_3_ct0_d1(),a_filter_3_ct0_d1(),0,2nat);
    r::pass_step(raw_filter_3_ct0_d1(),a_filter_3_ct0_d1(),0,1nat);
    r::pass_step(raw_filter_3_ct0_d1(),a_filter_3_ct0_d1(),0,0nat);
    r::scan_step(raw_filter_3_ct0_d1(),a_filter_3_ct0_d1(),1);
    r::scan_step(raw_filter_3_ct0_d1(),a_filter_3_ct0_d1(),0);
}
#[verifier::spinoff_prover]
proof fn model_filter_3_ct0_d1()
    ensures r::still(bytes_filter_3_ct0_d1(),STRICT,false)==(r::Projection::Ready {width:6u32,height:4u32,bytes:96nat}),
{
    hide(bytes_filter_3_ct0_d1);hide(z_filter_3_ct0_d1);hide(raw_filter_3_ct0_d1);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_3_ct0_d1().len()==73nat) by (compute);
    assert(crate::model::mh(bytes_filter_3_ct0_d1())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:1u8,color_type:0u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_3_ct0_d1())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:1u8,color_type:0u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 1u8)==2u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_3_ct0_d1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:16nat,next:61nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_3_ct0_d1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:16nat,next:61nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_3_ct0_d1(),61nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:69nat,len:0nat,next:73nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_3_ct0_d1(),61nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:69nat,len:0nat,next:73nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_3_ct0_d1().subrange(41,57)==z_filter_3_ct0_d1()) by (compute);
    walk_filter_3_ct0_d1(bytes_filter_3_ct0_d1());
    inflate_filter_3_ct0_d1();scan_filter_3_ct0_d1();
    assert(r::raw(a_filter_3_ct0_d1())==8nat) by (compute);
    assert(r::data(z_filter_3_ct0_d1(),a_filter_3_ct0_d1(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_filter_3_ct0_d1(),h_filter_3_ct0_d1(),z_filter_3_ct0_d1(),0nat,false);
}
fn public_filter_3_ct0_d1() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,1u8,0u8,0u8,0u8,0u8,133u8,127u8,115u8,238u8,0u8,0u8,0u8,16u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,246u8,97u8,102u8,98u8,174u8,97u8,14u8,1u8,0u8,4u8,42u8,1u8,43u8,217u8,76u8,137u8,10u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_3_ct0_d1());model_filter_3_ct0_d1();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==6u32 && s.height==4u32 && s.rgba@.len()==96nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// filter/3-ct0-d8; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_filter_3_ct0_d8() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,0u8,0u8,0u8,0u8,136u8,111u8,17u8,159u8,0u8,0u8,0u8,37u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,142u8,252u8,213u8,157u8,183u8,200u8,138u8,249u8,238u8,245u8,226u8,73u8,19u8,115u8,152u8,89u8,5u8,142u8,151u8,174u8,248u8,205u8,188u8,190u8,209u8,110u8,206u8,252u8,16u8,0u8,184u8,100u8,12u8,220u8,35u8,200u8,177u8,93u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_filter_3_ct0_d8() -> Seq<u8> {seq![120u8,156u8,99u8,142u8,252u8,213u8,157u8,183u8,200u8,138u8,249u8,238u8,245u8,226u8,73u8,19u8,115u8,152u8,89u8,5u8,142u8,151u8,174u8,248u8,205u8,188u8,190u8,209u8,110u8,206u8,252u8,16u8,0u8,184u8,100u8,12u8,220u8]}
spec fn h_filter_3_ct0_d8() -> crate::model::Ihdr {crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:0u8,interlace:0u8}}
spec fn a_filter_3_ct0_d8() -> r::Raster {r::shape(h_filter_3_ct0_d8(),6nat,4nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_filter_3_ct0_d8(b: Seq<u8>)
    requires b.len()==94,crate::model::mh(b)==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:0u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:37nat,next:82nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,82nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:90nat,len:0nat,next:94nat,crc_ok:true,synthetic:false})),
        b.subrange(41,78)==z_filter_3_ct0_d8(),
    ensures r::default_case(b,crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:0u8,interlace:0u8},z_filter_3_ct0_d8(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_filter_3_ct0_d8() -> Seq<u8> {seq![3u8,89u8,250u8,139u8,110u8,162u8,58u8,3u8,221u8,215u8,115u8,146u8,145u8,108u8,3u8,5u8,16u8,199u8,117u8,168u8,251u8,3u8,175u8,129u8,62u8,156u8,159u8,84u8]}
spec fn ts_filter_3_ct0_d8() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:3nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:35nat,before:1nat,after:2nat,symbol:89nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:35nat,next:44nat,before:2nat,after:3nat,symbol:250nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:44nat,next:52nat,before:3nat,after:4nat,symbol:139nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:52nat,next:60nat,before:4nat,after:5nat,symbol:110nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:60nat,next:69nat,before:5nat,after:6nat,symbol:162nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:69nat,next:77nat,before:6nat,after:7nat,symbol:58nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:77nat,next:85nat,before:7nat,after:8nat,symbol:3nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:85nat,next:94nat,before:8nat,after:9nat,symbol:221nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:94nat,next:103nat,before:9nat,after:10nat,symbol:215nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:103nat,next:111nat,before:10nat,after:11nat,symbol:115nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:111nat,next:120nat,before:11nat,after:12nat,symbol:146nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:120nat,next:129nat,before:12nat,after:13nat,symbol:145nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:129nat,next:137nat,before:13nat,after:14nat,symbol:108nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:137nat,next:145nat,before:14nat,after:15nat,symbol:3nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:145nat,next:153nat,before:15nat,after:16nat,symbol:5nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:153nat,next:161nat,before:16nat,after:17nat,symbol:16nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:161nat,next:170nat,before:17nat,after:18nat,symbol:199nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:170nat,next:178nat,before:18nat,after:19nat,symbol:117nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:178nat,next:187nat,before:19nat,after:20nat,symbol:168nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:187nat,next:196nat,before:20nat,after:21nat,symbol:251nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:196nat,next:204nat,before:21nat,after:22nat,symbol:3nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:204nat,next:213nat,before:22nat,after:23nat,symbol:175nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:213nat,next:221nat,before:23nat,after:24nat,symbol:129nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:221nat,next:229nat,before:24nat,after:25nat,symbol:62nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:229nat,next:238nat,before:25nat,after:26nat,symbol:156nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:238nat,next:247nat,before:26nat,after:27nat,symbol:159nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:247nat,next:255nat,before:27nat,after:28nat,symbol:84nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:255nat,next:262nat,before:28nat,after:28nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_filter_3_ct0_d8()
    ensures t::trace(z_filter_3_ct0_d8(),raw_filter_3_ct0_d8(),ts_filter_3_ct0_d8(),0,29nat),
{
    hide(z_filter_3_ct0_d8);hide(raw_filter_3_ct0_d8);hide(ts_filter_3_ct0_d8);hide(t::valid_step);
    assert(t::trace(z_filter_3_ct0_d8(),raw_filter_3_ct0_d8(),ts_filter_3_ct0_d8(),0,29nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_filter_3_ct0_d8()
    ensures raw_filter_3_ct0_d8().len()==28nat,
        z_filter_3_ct0_d8().len()==37nat,
        z_filter_3_ct0_d8()[0]==120u8,
        z_filter_3_ct0_d8()[1]==156u8,
        zm::field_value(z_filter_3_ct0_d8(),16,1)==1nat,
        zm::field_value(z_filter_3_ct0_d8(),17,2)==1nat,
        zm::be_value(z_filter_3_ct0_d8(),33nat,4)==3093564636nat,
        ts_filter_3_ct0_d8().len()==29nat,
        ts_filter_3_ct0_d8()[0].pos==19nat,
        ts_filter_3_ct0_d8()[0].before==0nat,
        ts_filter_3_ct0_d8().last().next==262nat,
{
    hide(z_filter_3_ct0_d8);hide(raw_filter_3_ct0_d8);hide(ts_filter_3_ct0_d8);
    assert(raw_filter_3_ct0_d8().len()==28nat) by (compute);
    assert(z_filter_3_ct0_d8().len()==37nat) by (compute);
    assert(z_filter_3_ct0_d8()[0]==120u8) by (compute);
    assert(z_filter_3_ct0_d8()[1]==156u8) by (compute);
    assert(zm::field_value(z_filter_3_ct0_d8(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_filter_3_ct0_d8(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_filter_3_ct0_d8(),33nat,4)==3093564636nat) by (compute);
    assert(ts_filter_3_ct0_d8().len()==29nat) by (compute);
    assert(ts_filter_3_ct0_d8()[0].pos==19nat) by (compute);
    assert(ts_filter_3_ct0_d8()[0].before==0nat) by (compute);
    assert(ts_filter_3_ct0_d8().last().next==262nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_filter_3_ct0_d8()
    ensures zm::Adler(raw_filter_3_ct0_d8())==3093564636nat,
{
    hide(raw_filter_3_ct0_d8);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_filter_3_ct0_d8().len()==28nat) by (compute);
    assert(c::sums(raw_filter_3_ct0_d8(),0,28nat)==(3291nat,47204nat)) by (compute);
    c::sums_correct(raw_filter_3_ct0_d8(),0,28nat);
    assert(raw_filter_3_ct0_d8().subrange(0,28) =~= raw_filter_3_ct0_d8());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((47204nat%65521)*(u16::MAX as nat+1)+(1+3291nat)%65521==3093564636nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_filter_3_ct0_d8()
    ensures r::inflated(z_filter_3_ct0_d8(),a_filter_3_ct0_d8(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_filter_3_ct0_d8(),r2:false}),
        raw_filter_3_ct0_d8().len()==28nat,
{
    hide(z_filter_3_ct0_d8);hide(raw_filter_3_ct0_d8);hide(ts_filter_3_ct0_d8);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_filter_3_ct0_d8();checksum_filter_3_ct0_d8();
    assert(r::raw(a_filter_3_ct0_d8())==28nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_filter_3_ct0_d8();
    stream::fixed_stream(z_filter_3_ct0_d8(),raw_filter_3_ct0_d8(),ts_filter_3_ct0_d8(),28nat,156nat,3093564636nat);
}
#[verifier::spinoff_prover]
proof fn scan_filter_3_ct0_d8()
    ensures r::scan(raw_filter_3_ct0_d8(),a_filter_3_ct0_d8(),0)==Ok(()),
{
    hide(raw_filter_3_ct0_d8);
    assert(r::base(a_filter_3_ct0_d8(),0)==0nat && r::ph(a_filter_3_ct0_d8(),0)==4nat && r::count(a_filter_3_ct0_d8())==1nat) by (compute);
    assert(r::rb(a_filter_3_ct0_d8(),0)==6nat) by (compute);
    assert(raw_filter_3_ct0_d8()[0]==3u8) by (compute);
    assert(r::row_result(raw_filter_3_ct0_d8(),a_filter_3_ct0_d8(),0,0nat)==Ok(()));
    assert(raw_filter_3_ct0_d8()[7]==3u8) by (compute);
    assert(r::row_result(raw_filter_3_ct0_d8(),a_filter_3_ct0_d8(),0,1nat)==Ok(()));
    assert(raw_filter_3_ct0_d8()[14]==3u8) by (compute);
    assert(r::row_result(raw_filter_3_ct0_d8(),a_filter_3_ct0_d8(),0,2nat)==Ok(()));
    assert(raw_filter_3_ct0_d8()[21]==3u8) by (compute);
    assert(r::row_result(raw_filter_3_ct0_d8(),a_filter_3_ct0_d8(),0,3nat)==Ok(()));
    r::pass_step(raw_filter_3_ct0_d8(),a_filter_3_ct0_d8(),0,4nat);
    r::pass_step(raw_filter_3_ct0_d8(),a_filter_3_ct0_d8(),0,3nat);
    r::pass_step(raw_filter_3_ct0_d8(),a_filter_3_ct0_d8(),0,2nat);
    r::pass_step(raw_filter_3_ct0_d8(),a_filter_3_ct0_d8(),0,1nat);
    r::pass_step(raw_filter_3_ct0_d8(),a_filter_3_ct0_d8(),0,0nat);
    r::scan_step(raw_filter_3_ct0_d8(),a_filter_3_ct0_d8(),1);
    r::scan_step(raw_filter_3_ct0_d8(),a_filter_3_ct0_d8(),0);
}
#[verifier::spinoff_prover]
proof fn model_filter_3_ct0_d8()
    ensures r::still(bytes_filter_3_ct0_d8(),STRICT,false)==(r::Projection::Ready {width:6u32,height:4u32,bytes:96nat}),
{
    hide(bytes_filter_3_ct0_d8);hide(z_filter_3_ct0_d8);hide(raw_filter_3_ct0_d8);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_3_ct0_d8().len()==94nat) by (compute);
    assert(crate::model::mh(bytes_filter_3_ct0_d8())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:0u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_3_ct0_d8())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:0u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_3_ct0_d8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:37nat,next:82nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_3_ct0_d8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:37nat,next:82nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_3_ct0_d8(),82nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:90nat,len:0nat,next:94nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_3_ct0_d8(),82nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:90nat,len:0nat,next:94nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_3_ct0_d8().subrange(41,78)==z_filter_3_ct0_d8()) by (compute);
    walk_filter_3_ct0_d8(bytes_filter_3_ct0_d8());
    inflate_filter_3_ct0_d8();scan_filter_3_ct0_d8();
    assert(r::raw(a_filter_3_ct0_d8())==28nat) by (compute);
    assert(r::data(z_filter_3_ct0_d8(),a_filter_3_ct0_d8(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_filter_3_ct0_d8(),h_filter_3_ct0_d8(),z_filter_3_ct0_d8(),0nat,false);
}
fn public_filter_3_ct0_d8() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,0u8,0u8,0u8,0u8,136u8,111u8,17u8,159u8,0u8,0u8,0u8,37u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,142u8,252u8,213u8,157u8,183u8,200u8,138u8,249u8,238u8,245u8,226u8,73u8,19u8,115u8,152u8,89u8,5u8,142u8,151u8,174u8,248u8,205u8,188u8,190u8,209u8,110u8,206u8,252u8,16u8,0u8,184u8,100u8,12u8,220u8,35u8,200u8,177u8,93u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_3_ct0_d8());model_filter_3_ct0_d8();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==6u32 && s.height==4u32 && s.rgba@.len()==96nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// filter/3-ct0-d16; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_filter_3_ct0_d16() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,0u8,0u8,0u8,0u8,216u8,255u8,205u8,220u8,0u8,0u8,0u8,63u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,52u8,0u8,203u8,255u8,3u8,5u8,146u8,114u8,209u8,58u8,32u8,237u8,211u8,26u8,46u8,167u8,128u8,3u8,37u8,199u8,234u8,211u8,63u8,98u8,206u8,61u8,247u8,249u8,147u8,159u8,3u8,151u8,191u8,10u8,150u8,109u8,130u8,115u8,227u8,150u8,97u8,107u8,135u8,3u8,135u8,174u8,45u8,60u8,119u8,26u8,21u8,152u8,96u8,37u8,175u8,151u8,122u8,168u8,23u8,178u8,165u8,47u8,232u8,141u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_filter_3_ct0_d16() -> Seq<u8> {seq![120u8,156u8,1u8,52u8,0u8,203u8,255u8,3u8,5u8,146u8,114u8,209u8,58u8,32u8,237u8,211u8,26u8,46u8,167u8,128u8,3u8,37u8,199u8,234u8,211u8,63u8,98u8,206u8,61u8,247u8,249u8,147u8,159u8,3u8,151u8,191u8,10u8,150u8,109u8,130u8,115u8,227u8,150u8,97u8,107u8,135u8,3u8,135u8,174u8,45u8,60u8,119u8,26u8,21u8,152u8,96u8,37u8,175u8,151u8,122u8,168u8,23u8,178u8]}
spec fn h_filter_3_ct0_d16() -> crate::model::Ihdr {crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:0u8,interlace:0u8}}
spec fn a_filter_3_ct0_d16() -> r::Raster {r::shape(h_filter_3_ct0_d16(),6nat,4nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_filter_3_ct0_d16(b: Seq<u8>)
    requires b.len()==120,crate::model::mh(b)==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:0u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:63nat,next:108nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,108nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:116nat,len:0nat,next:120nat,crc_ok:true,synthetic:false})),
        b.subrange(41,104)==z_filter_3_ct0_d16(),
    ensures r::default_case(b,crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:0u8,interlace:0u8},z_filter_3_ct0_d16(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
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
spec fn raw_filter_3_ct0_d16() -> Seq<u8> {seq![3u8,5u8,146u8,114u8,209u8,58u8,32u8,237u8,211u8,26u8,46u8,167u8,128u8,3u8,37u8,199u8,234u8,211u8,63u8,98u8,206u8,61u8,247u8,249u8,147u8,159u8,3u8,151u8,191u8,10u8,150u8,109u8,130u8,115u8,227u8,150u8,97u8,107u8,135u8,3u8,135u8,174u8,45u8,60u8,119u8,26u8,21u8,152u8,96u8,37u8,175u8,151u8]}
#[verifier::spinoff_prover]
proof fn shape_filter_3_ct0_d16()
    ensures raw_filter_3_ct0_d16().len()==52nat,
        z_filter_3_ct0_d16().len()==63nat,
        z_filter_3_ct0_d16()[0]==120u8,
        z_filter_3_ct0_d16()[1]==156u8,
        zm::field_value(z_filter_3_ct0_d16(),16,1)==1nat,
        zm::field_value(z_filter_3_ct0_d16(),17,2)==0nat,
        zm::be_value(z_filter_3_ct0_d16(),59nat,4)==2057836466nat,
        z_filter_3_ct0_d16().subrange(7,59)==raw_filter_3_ct0_d16(),
        z_filter_3_ct0_d16()[3] as nat+256*z_filter_3_ct0_d16()[4] as nat==52nat,
        z_filter_3_ct0_d16()[5] as nat+256*z_filter_3_ct0_d16()[6] as nat==u16::MAX as nat-52nat,
        zm::be_value(z_filter_3_ct0_d16(),0,1)==120nat,
        zm::be_value(z_filter_3_ct0_d16(),1,1)==156nat,
{
    hide(z_filter_3_ct0_d16);hide(raw_filter_3_ct0_d16);
    assert(raw_filter_3_ct0_d16().len()==52nat) by (compute);
    assert(z_filter_3_ct0_d16().len()==63nat) by (compute);
    assert(z_filter_3_ct0_d16()[0]==120u8) by (compute);
    assert(z_filter_3_ct0_d16()[1]==156u8) by (compute);
    assert(zm::field_value(z_filter_3_ct0_d16(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_filter_3_ct0_d16(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_filter_3_ct0_d16(),59nat,4)==2057836466nat) by (compute);
    assert(z_filter_3_ct0_d16().subrange(7,59)==raw_filter_3_ct0_d16()) by (compute);
    assert(z_filter_3_ct0_d16()[3] as nat+256*z_filter_3_ct0_d16()[4] as nat==52nat) by (compute);
    assert(z_filter_3_ct0_d16()[5] as nat+256*z_filter_3_ct0_d16()[6] as nat==u16::MAX as nat-52nat) by (compute);
    assert(zm::be_value(z_filter_3_ct0_d16(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_filter_3_ct0_d16(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_filter_3_ct0_d16()
    ensures zm::Adler(raw_filter_3_ct0_d16())==2057836466nat,
{
    hide(raw_filter_3_ct0_d16);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_filter_3_ct0_d16().len()==52nat) by (compute);
    assert(c::sums(raw_filter_3_ct0_d16(),0,52nat)==(6065nat,162442nat)) by (compute);
    c::sums_correct(raw_filter_3_ct0_d16(),0,52nat);
    assert(raw_filter_3_ct0_d16().subrange(0,52) =~= raw_filter_3_ct0_d16());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((162442nat%65521)*(u16::MAX as nat+1)+(1+6065nat)%65521==2057836466nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_filter_3_ct0_d16()
    ensures r::inflated(z_filter_3_ct0_d16(),a_filter_3_ct0_d16(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_filter_3_ct0_d16(),r2:false}),
        raw_filter_3_ct0_d16().len()==52nat,
{
    hide(z_filter_3_ct0_d16);hide(raw_filter_3_ct0_d16);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_filter_3_ct0_d16();checksum_filter_3_ct0_d16();
    assert(r::raw(a_filter_3_ct0_d16())==52nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_filter_3_ct0_d16(),raw_filter_3_ct0_d16(),52nat,120nat,156nat,2057836466nat);
}
#[verifier::spinoff_prover]
proof fn scan_filter_3_ct0_d16()
    ensures r::scan(raw_filter_3_ct0_d16(),a_filter_3_ct0_d16(),0)==Ok(()),
{
    hide(raw_filter_3_ct0_d16);
    assert(r::base(a_filter_3_ct0_d16(),0)==0nat && r::ph(a_filter_3_ct0_d16(),0)==4nat && r::count(a_filter_3_ct0_d16())==1nat) by (compute);
    assert(r::rb(a_filter_3_ct0_d16(),0)==12nat) by (compute);
    assert(raw_filter_3_ct0_d16()[0]==3u8) by (compute);
    assert(r::row_result(raw_filter_3_ct0_d16(),a_filter_3_ct0_d16(),0,0nat)==Ok(()));
    assert(raw_filter_3_ct0_d16()[13]==3u8) by (compute);
    assert(r::row_result(raw_filter_3_ct0_d16(),a_filter_3_ct0_d16(),0,1nat)==Ok(()));
    assert(raw_filter_3_ct0_d16()[26]==3u8) by (compute);
    assert(r::row_result(raw_filter_3_ct0_d16(),a_filter_3_ct0_d16(),0,2nat)==Ok(()));
    assert(raw_filter_3_ct0_d16()[39]==3u8) by (compute);
    assert(r::row_result(raw_filter_3_ct0_d16(),a_filter_3_ct0_d16(),0,3nat)==Ok(()));
    r::pass_step(raw_filter_3_ct0_d16(),a_filter_3_ct0_d16(),0,4nat);
    r::pass_step(raw_filter_3_ct0_d16(),a_filter_3_ct0_d16(),0,3nat);
    r::pass_step(raw_filter_3_ct0_d16(),a_filter_3_ct0_d16(),0,2nat);
    r::pass_step(raw_filter_3_ct0_d16(),a_filter_3_ct0_d16(),0,1nat);
    r::pass_step(raw_filter_3_ct0_d16(),a_filter_3_ct0_d16(),0,0nat);
    r::scan_step(raw_filter_3_ct0_d16(),a_filter_3_ct0_d16(),1);
    r::scan_step(raw_filter_3_ct0_d16(),a_filter_3_ct0_d16(),0);
}
#[verifier::spinoff_prover]
proof fn model_filter_3_ct0_d16()
    ensures r::still(bytes_filter_3_ct0_d16(),STRICT,false)==(r::Projection::Ready {width:6u32,height:4u32,bytes:96nat}),
{
    hide(bytes_filter_3_ct0_d16);hide(z_filter_3_ct0_d16);hide(raw_filter_3_ct0_d16);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_3_ct0_d16().len()==120nat) by (compute);
    assert(crate::model::mh(bytes_filter_3_ct0_d16())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:0u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_3_ct0_d16())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:0u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_3_ct0_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:63nat,next:108nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_3_ct0_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:63nat,next:108nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_3_ct0_d16(),108nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:116nat,len:0nat,next:120nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_3_ct0_d16(),108nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:116nat,len:0nat,next:120nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_3_ct0_d16().subrange(41,104)==z_filter_3_ct0_d16()) by (compute);
    walk_filter_3_ct0_d16(bytes_filter_3_ct0_d16());
    inflate_filter_3_ct0_d16();scan_filter_3_ct0_d16();
    assert(r::raw(a_filter_3_ct0_d16())==52nat) by (compute);
    assert(r::data(z_filter_3_ct0_d16(),a_filter_3_ct0_d16(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_filter_3_ct0_d16(),h_filter_3_ct0_d16(),z_filter_3_ct0_d16(),0nat,false);
}
fn public_filter_3_ct0_d16() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,0u8,0u8,0u8,0u8,216u8,255u8,205u8,220u8,0u8,0u8,0u8,63u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,52u8,0u8,203u8,255u8,3u8,5u8,146u8,114u8,209u8,58u8,32u8,237u8,211u8,26u8,46u8,167u8,128u8,3u8,37u8,199u8,234u8,211u8,63u8,98u8,206u8,61u8,247u8,249u8,147u8,159u8,3u8,151u8,191u8,10u8,150u8,109u8,130u8,115u8,227u8,150u8,97u8,107u8,135u8,3u8,135u8,174u8,45u8,60u8,119u8,26u8,21u8,152u8,96u8,37u8,175u8,151u8,122u8,168u8,23u8,178u8,165u8,47u8,232u8,141u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_3_ct0_d16());model_filter_3_ct0_d16();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==6u32 && s.height==4u32 && s.rgba@.len()==96nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// filter/3-ct2-d8; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_filter_3_ct2_d8() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,2u8,0u8,0u8,0u8,34u8,102u8,217u8,20u8,0u8,0u8,0u8,87u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,76u8,0u8,179u8,255u8,3u8,50u8,39u8,159u8,166u8,203u8,252u8,101u8,202u8,88u8,184u8,82u8,211u8,186u8,174u8,250u8,85u8,24u8,102u8,3u8,194u8,102u8,126u8,217u8,168u8,176u8,85u8,229u8,181u8,67u8,107u8,78u8,124u8,228u8,87u8,180u8,175u8,215u8,3u8,227u8,227u8,0u8,225u8,49u8,77u8,247u8,185u8,247u8,132u8,117u8,192u8,227u8,66u8,13u8,46u8,97u8,56u8,3u8,134u8,67u8,87u8,167u8,161u8,142u8,226u8,87u8,173u8,201u8,248u8,19u8,15u8,112u8,51u8,92u8,65u8,2u8,7u8,34u8,38u8,61u8,42u8,102u8,146u8,196u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_filter_3_ct2_d8() -> Seq<u8> {seq![120u8,156u8,1u8,76u8,0u8,179u8,255u8,3u8,50u8,39u8,159u8,166u8,203u8,252u8,101u8,202u8,88u8,184u8,82u8,211u8,186u8,174u8,250u8,85u8,24u8,102u8,3u8,194u8,102u8,126u8,217u8,168u8,176u8,85u8,229u8,181u8,67u8,107u8,78u8,124u8,228u8,87u8,180u8,175u8,215u8,3u8,227u8,227u8,0u8,225u8,49u8,77u8,247u8,185u8,247u8,132u8,117u8,192u8,227u8,66u8,13u8,46u8,97u8,56u8,3u8,134u8,67u8,87u8,167u8,161u8,142u8,226u8,87u8,173u8,201u8,248u8,19u8,15u8,112u8,51u8,92u8,65u8,2u8,7u8,34u8,38u8,61u8]}
spec fn h_filter_3_ct2_d8() -> crate::model::Ihdr {crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:2u8,interlace:0u8}}
spec fn a_filter_3_ct2_d8() -> r::Raster {r::shape(h_filter_3_ct2_d8(),6nat,4nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_filter_3_ct2_d8(b: Seq<u8>)
    requires b.len()==144,crate::model::mh(b)==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:2u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:87nat,next:132nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,132nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:140nat,len:0nat,next:144nat,crc_ok:true,synthetic:false})),
        b.subrange(41,128)==z_filter_3_ct2_d8(),
    ensures r::default_case(b,crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:2u8,interlace:0u8},z_filter_3_ct2_d8(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_filter_3_ct2_d8() -> Seq<u8> {seq![3u8,50u8,39u8,159u8,166u8,203u8,252u8,101u8,202u8,88u8,184u8,82u8,211u8,186u8,174u8,250u8,85u8,24u8,102u8,3u8,194u8,102u8,126u8,217u8,168u8,176u8,85u8,229u8,181u8,67u8,107u8,78u8,124u8,228u8,87u8,180u8,175u8,215u8,3u8,227u8,227u8,0u8,225u8,49u8,77u8,247u8,185u8,247u8,132u8,117u8,192u8,227u8,66u8,13u8,46u8,97u8,56u8,3u8,134u8,67u8,87u8,167u8,161u8,142u8,226u8,87u8,173u8,201u8,248u8,19u8,15u8,112u8,51u8,92u8,65u8,2u8]}
#[verifier::spinoff_prover]
proof fn shape_filter_3_ct2_d8()
    ensures raw_filter_3_ct2_d8().len()==76nat,
        z_filter_3_ct2_d8().len()==87nat,
        z_filter_3_ct2_d8()[0]==120u8,
        z_filter_3_ct2_d8()[1]==156u8,
        zm::field_value(z_filter_3_ct2_d8(),16,1)==1nat,
        zm::field_value(z_filter_3_ct2_d8(),17,2)==0nat,
        zm::be_value(z_filter_3_ct2_d8(),83nat,4)==119678525nat,
        z_filter_3_ct2_d8().subrange(7,83)==raw_filter_3_ct2_d8(),
        z_filter_3_ct2_d8()[3] as nat+256*z_filter_3_ct2_d8()[4] as nat==76nat,
        z_filter_3_ct2_d8()[5] as nat+256*z_filter_3_ct2_d8()[6] as nat==u16::MAX as nat-76nat,
        zm::be_value(z_filter_3_ct2_d8(),0,1)==120nat,
        zm::be_value(z_filter_3_ct2_d8(),1,1)==156nat,
{
    hide(z_filter_3_ct2_d8);hide(raw_filter_3_ct2_d8);
    assert(raw_filter_3_ct2_d8().len()==76nat) by (compute);
    assert(z_filter_3_ct2_d8().len()==87nat) by (compute);
    assert(z_filter_3_ct2_d8()[0]==120u8) by (compute);
    assert(z_filter_3_ct2_d8()[1]==156u8) by (compute);
    assert(zm::field_value(z_filter_3_ct2_d8(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_filter_3_ct2_d8(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_filter_3_ct2_d8(),83nat,4)==119678525nat) by (compute);
    assert(z_filter_3_ct2_d8().subrange(7,83)==raw_filter_3_ct2_d8()) by (compute);
    assert(z_filter_3_ct2_d8()[3] as nat+256*z_filter_3_ct2_d8()[4] as nat==76nat) by (compute);
    assert(z_filter_3_ct2_d8()[5] as nat+256*z_filter_3_ct2_d8()[6] as nat==u16::MAX as nat-76nat) by (compute);
    assert(zm::be_value(z_filter_3_ct2_d8(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_filter_3_ct2_d8(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_filter_3_ct2_d8()
    ensures zm::Adler(raw_filter_3_ct2_d8())==119678525nat,
{
    hide(raw_filter_3_ct2_d8);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_filter_3_ct2_d8().len()==76nat) by (compute);
    assert(c::sums(raw_filter_3_ct2_d8(),0,76nat)==(9788nat,394952nat)) by (compute);
    c::sums_correct(raw_filter_3_ct2_d8(),0,76nat);
    assert(raw_filter_3_ct2_d8().subrange(0,76) =~= raw_filter_3_ct2_d8());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((394952nat%65521)*(u16::MAX as nat+1)+(1+9788nat)%65521==119678525nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_filter_3_ct2_d8()
    ensures r::inflated(z_filter_3_ct2_d8(),a_filter_3_ct2_d8(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_filter_3_ct2_d8(),r2:false}),
        raw_filter_3_ct2_d8().len()==76nat,
{
    hide(z_filter_3_ct2_d8);hide(raw_filter_3_ct2_d8);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_filter_3_ct2_d8();checksum_filter_3_ct2_d8();
    assert(r::raw(a_filter_3_ct2_d8())==76nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_filter_3_ct2_d8(),raw_filter_3_ct2_d8(),76nat,120nat,156nat,119678525nat);
}
#[verifier::spinoff_prover]
proof fn scan_filter_3_ct2_d8()
    ensures r::scan(raw_filter_3_ct2_d8(),a_filter_3_ct2_d8(),0)==Ok(()),
{
    hide(raw_filter_3_ct2_d8);
    assert(r::base(a_filter_3_ct2_d8(),0)==0nat && r::ph(a_filter_3_ct2_d8(),0)==4nat && r::count(a_filter_3_ct2_d8())==1nat) by (compute);
    assert(r::rb(a_filter_3_ct2_d8(),0)==18nat) by (compute);
    assert(raw_filter_3_ct2_d8()[0]==3u8) by (compute);
    assert(r::row_result(raw_filter_3_ct2_d8(),a_filter_3_ct2_d8(),0,0nat)==Ok(()));
    assert(raw_filter_3_ct2_d8()[19]==3u8) by (compute);
    assert(r::row_result(raw_filter_3_ct2_d8(),a_filter_3_ct2_d8(),0,1nat)==Ok(()));
    assert(raw_filter_3_ct2_d8()[38]==3u8) by (compute);
    assert(r::row_result(raw_filter_3_ct2_d8(),a_filter_3_ct2_d8(),0,2nat)==Ok(()));
    assert(raw_filter_3_ct2_d8()[57]==3u8) by (compute);
    assert(r::row_result(raw_filter_3_ct2_d8(),a_filter_3_ct2_d8(),0,3nat)==Ok(()));
    r::pass_step(raw_filter_3_ct2_d8(),a_filter_3_ct2_d8(),0,4nat);
    r::pass_step(raw_filter_3_ct2_d8(),a_filter_3_ct2_d8(),0,3nat);
    r::pass_step(raw_filter_3_ct2_d8(),a_filter_3_ct2_d8(),0,2nat);
    r::pass_step(raw_filter_3_ct2_d8(),a_filter_3_ct2_d8(),0,1nat);
    r::pass_step(raw_filter_3_ct2_d8(),a_filter_3_ct2_d8(),0,0nat);
    r::scan_step(raw_filter_3_ct2_d8(),a_filter_3_ct2_d8(),1);
    r::scan_step(raw_filter_3_ct2_d8(),a_filter_3_ct2_d8(),0);
}
#[verifier::spinoff_prover]
proof fn model_filter_3_ct2_d8()
    ensures r::still(bytes_filter_3_ct2_d8(),STRICT,false)==(r::Projection::Ready {width:6u32,height:4u32,bytes:96nat}),
{
    hide(bytes_filter_3_ct2_d8);hide(z_filter_3_ct2_d8);hide(raw_filter_3_ct2_d8);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_3_ct2_d8().len()==144nat) by (compute);
    assert(crate::model::mh(bytes_filter_3_ct2_d8())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:2u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_3_ct2_d8())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:2u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_3_ct2_d8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:87nat,next:132nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_3_ct2_d8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:87nat,next:132nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_3_ct2_d8(),132nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:140nat,len:0nat,next:144nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_3_ct2_d8(),132nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:140nat,len:0nat,next:144nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_3_ct2_d8().subrange(41,128)==z_filter_3_ct2_d8()) by (compute);
    walk_filter_3_ct2_d8(bytes_filter_3_ct2_d8());
    inflate_filter_3_ct2_d8();scan_filter_3_ct2_d8();
    assert(r::raw(a_filter_3_ct2_d8())==76nat) by (compute);
    assert(r::data(z_filter_3_ct2_d8(),a_filter_3_ct2_d8(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_filter_3_ct2_d8(),h_filter_3_ct2_d8(),z_filter_3_ct2_d8(),0nat,false);
}
fn public_filter_3_ct2_d8() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,2u8,0u8,0u8,0u8,34u8,102u8,217u8,20u8,0u8,0u8,0u8,87u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,76u8,0u8,179u8,255u8,3u8,50u8,39u8,159u8,166u8,203u8,252u8,101u8,202u8,88u8,184u8,82u8,211u8,186u8,174u8,250u8,85u8,24u8,102u8,3u8,194u8,102u8,126u8,217u8,168u8,176u8,85u8,229u8,181u8,67u8,107u8,78u8,124u8,228u8,87u8,180u8,175u8,215u8,3u8,227u8,227u8,0u8,225u8,49u8,77u8,247u8,185u8,247u8,132u8,117u8,192u8,227u8,66u8,13u8,46u8,97u8,56u8,3u8,134u8,67u8,87u8,167u8,161u8,142u8,226u8,87u8,173u8,201u8,248u8,19u8,15u8,112u8,51u8,92u8,65u8,2u8,7u8,34u8,38u8,61u8,42u8,102u8,146u8,196u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_3_ct2_d8());model_filter_3_ct2_d8();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==6u32 && s.height==4u32 && s.rgba@.len()==96nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// filter/3-ct2-d16; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_filter_3_ct2_d16() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,2u8,0u8,0u8,0u8,114u8,246u8,5u8,87u8,0u8,0u8,0u8,159u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,148u8,0u8,107u8,255u8,3u8,88u8,198u8,168u8,128u8,137u8,73u8,61u8,149u8,151u8,61u8,0u8,85u8,11u8,120u8,15u8,139u8,66u8,208u8,3u8,30u8,249u8,215u8,232u8,50u8,49u8,18u8,0u8,63u8,71u8,246u8,55u8,134u8,219u8,9u8,29u8,213u8,3u8,19u8,234u8,188u8,93u8,133u8,4u8,53u8,86u8,195u8,189u8,136u8,167u8,131u8,20u8,22u8,120u8,110u8,52u8,112u8,138u8,27u8,15u8,237u8,51u8,11u8,157u8,114u8,191u8,132u8,52u8,127u8,75u8,30u8,147u8,145u8,8u8,3u8,54u8,135u8,147u8,240u8,241u8,104u8,210u8,94u8,198u8,184u8,22u8,91u8,236u8,7u8,22u8,2u8,141u8,148u8,85u8,139u8,150u8,83u8,24u8,225u8,19u8,141u8,199u8,135u8,30u8,97u8,37u8,51u8,221u8,50u8,221u8,142u8,3u8,157u8,97u8,65u8,205u8,154u8,154u8,176u8,168u8,165u8,200u8,189u8,213u8,208u8,86u8,90u8,224u8,120u8,107u8,222u8,104u8,52u8,152u8,158u8,49u8,3u8,108u8,35u8,142u8,248u8,42u8,225u8,68u8,193u8,201u8,98u8,94u8,235u8,81u8,65u8,134u8,230u8,230u8,77u8,137u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_filter_3_ct2_d16() -> Seq<u8> {seq![120u8,156u8,1u8,148u8,0u8,107u8,255u8,3u8,88u8,198u8,168u8,128u8,137u8,73u8,61u8,149u8,151u8,61u8,0u8,85u8,11u8,120u8,15u8,139u8,66u8,208u8,3u8,30u8,249u8,215u8,232u8,50u8,49u8,18u8,0u8,63u8,71u8,246u8,55u8,134u8,219u8,9u8,29u8,213u8,3u8,19u8,234u8,188u8,93u8,133u8,4u8,53u8,86u8,195u8,189u8,136u8,167u8,131u8,20u8,22u8,120u8,110u8,52u8,112u8,138u8,27u8,15u8,237u8,51u8,11u8,157u8,114u8,191u8,132u8,52u8,127u8,75u8,30u8,147u8,145u8,8u8,3u8,54u8,135u8,147u8,240u8,241u8,104u8,210u8,94u8,198u8,184u8,22u8,91u8,236u8,7u8,22u8,2u8,141u8,148u8,85u8,139u8,150u8,83u8,24u8,225u8,19u8,141u8,199u8,135u8,30u8,97u8,37u8,51u8,221u8,50u8,221u8,142u8,3u8,157u8,97u8,65u8,205u8,154u8,154u8,176u8,168u8,165u8,200u8,189u8,213u8,208u8,86u8,90u8,224u8,120u8,107u8,222u8,104u8,52u8,152u8,158u8,49u8,3u8,108u8,35u8,142u8,248u8,42u8,225u8,68u8,193u8,201u8,98u8,94u8,235u8,81u8,65u8,134u8]}
spec fn h_filter_3_ct2_d16() -> crate::model::Ihdr {crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:2u8,interlace:0u8}}
spec fn a_filter_3_ct2_d16() -> r::Raster {r::shape(h_filter_3_ct2_d16(),6nat,4nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_filter_3_ct2_d16(b: Seq<u8>)
    requires b.len()==216,crate::model::mh(b)==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:2u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:159nat,next:204nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,204nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:212nat,len:0nat,next:216nat,crc_ok:true,synthetic:false})),
        b.subrange(41,200)==z_filter_3_ct2_d16(),
    ensures r::default_case(b,crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:2u8,interlace:0u8},z_filter_3_ct2_d16(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
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
spec fn raw_filter_3_ct2_d16() -> Seq<u8> {seq![3u8,88u8,198u8,168u8,128u8,137u8,73u8,61u8,149u8,151u8,61u8,0u8,85u8,11u8,120u8,15u8,139u8,66u8,208u8,3u8,30u8,249u8,215u8,232u8,50u8,49u8,18u8,0u8,63u8,71u8,246u8,55u8,134u8,219u8,9u8,29u8,213u8,3u8,19u8,234u8,188u8,93u8,133u8,4u8,53u8,86u8,195u8,189u8,136u8,167u8,131u8,20u8,22u8,120u8,110u8,52u8,112u8,138u8,27u8,15u8,237u8,51u8,11u8,157u8,114u8,191u8,132u8,52u8,127u8,75u8,30u8,147u8,145u8,8u8,3u8,54u8,135u8,147u8,240u8,241u8,104u8,210u8,94u8,198u8,184u8,22u8,91u8,236u8,7u8,22u8,2u8,141u8,148u8,85u8,139u8,150u8,83u8,24u8,225u8,19u8,141u8,199u8,135u8,30u8,97u8,37u8,51u8,221u8,50u8,221u8,142u8,3u8,157u8,97u8,65u8,205u8,154u8,154u8,176u8,168u8,165u8,200u8,189u8,213u8,208u8,86u8,90u8,224u8,120u8,107u8,222u8,104u8,52u8,152u8,158u8,49u8,3u8,108u8,35u8,142u8,248u8,42u8,225u8,68u8,193u8,201u8,98u8,94u8]}
#[verifier::spinoff_prover]
proof fn shape_filter_3_ct2_d16()
    ensures raw_filter_3_ct2_d16().len()==148nat,
        z_filter_3_ct2_d16().len()==159nat,
        z_filter_3_ct2_d16()[0]==120u8,
        z_filter_3_ct2_d16()[1]==156u8,
        zm::field_value(z_filter_3_ct2_d16(),16,1)==1nat,
        zm::field_value(z_filter_3_ct2_d16(),17,2)==0nat,
        zm::be_value(z_filter_3_ct2_d16(),155nat,4)==3947970950nat,
        z_filter_3_ct2_d16().subrange(7,155)==raw_filter_3_ct2_d16(),
        z_filter_3_ct2_d16()[3] as nat+256*z_filter_3_ct2_d16()[4] as nat==148nat,
        z_filter_3_ct2_d16()[5] as nat+256*z_filter_3_ct2_d16()[6] as nat==u16::MAX as nat-148nat,
        zm::be_value(z_filter_3_ct2_d16(),0,1)==120nat,
        zm::be_value(z_filter_3_ct2_d16(),1,1)==156nat,
{
    hide(z_filter_3_ct2_d16);hide(raw_filter_3_ct2_d16);
    assert(raw_filter_3_ct2_d16().len()==148nat) by (compute);
    assert(z_filter_3_ct2_d16().len()==159nat) by (compute);
    assert(z_filter_3_ct2_d16()[0]==120u8) by (compute);
    assert(z_filter_3_ct2_d16()[1]==156u8) by (compute);
    assert(zm::field_value(z_filter_3_ct2_d16(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_filter_3_ct2_d16(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_filter_3_ct2_d16(),155nat,4)==3947970950nat) by (compute);
    assert(z_filter_3_ct2_d16().subrange(7,155)==raw_filter_3_ct2_d16()) by (compute);
    assert(z_filter_3_ct2_d16()[3] as nat+256*z_filter_3_ct2_d16()[4] as nat==148nat) by (compute);
    assert(z_filter_3_ct2_d16()[5] as nat+256*z_filter_3_ct2_d16()[6] as nat==u16::MAX as nat-148nat) by (compute);
    assert(zm::be_value(z_filter_3_ct2_d16(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_filter_3_ct2_d16(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_filter_3_ct2_d16()
    ensures zm::Adler(raw_filter_3_ct2_d16())==3947970950nat,
{
    hide(raw_filter_3_ct2_d16);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_filter_3_ct2_d16().len()==148nat) by (compute);
    assert(c::sums(raw_filter_3_ct2_d16(),0,148nat)==(16773nat,1174098nat)) by (compute);
    c::sums_correct(raw_filter_3_ct2_d16(),0,148nat);
    assert(raw_filter_3_ct2_d16().subrange(0,148) =~= raw_filter_3_ct2_d16());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((1174098nat%65521)*(u16::MAX as nat+1)+(1+16773nat)%65521==3947970950nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_filter_3_ct2_d16()
    ensures r::inflated(z_filter_3_ct2_d16(),a_filter_3_ct2_d16(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_filter_3_ct2_d16(),r2:false}),
        raw_filter_3_ct2_d16().len()==148nat,
{
    hide(z_filter_3_ct2_d16);hide(raw_filter_3_ct2_d16);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_filter_3_ct2_d16();checksum_filter_3_ct2_d16();
    assert(r::raw(a_filter_3_ct2_d16())==148nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_filter_3_ct2_d16(),raw_filter_3_ct2_d16(),148nat,120nat,156nat,3947970950nat);
}
#[verifier::spinoff_prover]
proof fn scan_filter_3_ct2_d16()
    ensures r::scan(raw_filter_3_ct2_d16(),a_filter_3_ct2_d16(),0)==Ok(()),
{
    hide(raw_filter_3_ct2_d16);
    assert(r::base(a_filter_3_ct2_d16(),0)==0nat && r::ph(a_filter_3_ct2_d16(),0)==4nat && r::count(a_filter_3_ct2_d16())==1nat) by (compute);
    assert(r::rb(a_filter_3_ct2_d16(),0)==36nat) by (compute);
    assert(raw_filter_3_ct2_d16()[0]==3u8) by (compute);
    assert(r::row_result(raw_filter_3_ct2_d16(),a_filter_3_ct2_d16(),0,0nat)==Ok(()));
    assert(raw_filter_3_ct2_d16()[37]==3u8) by (compute);
    assert(r::row_result(raw_filter_3_ct2_d16(),a_filter_3_ct2_d16(),0,1nat)==Ok(()));
    assert(raw_filter_3_ct2_d16()[74]==3u8) by (compute);
    assert(r::row_result(raw_filter_3_ct2_d16(),a_filter_3_ct2_d16(),0,2nat)==Ok(()));
    assert(raw_filter_3_ct2_d16()[111]==3u8) by (compute);
    assert(r::row_result(raw_filter_3_ct2_d16(),a_filter_3_ct2_d16(),0,3nat)==Ok(()));
    r::pass_step(raw_filter_3_ct2_d16(),a_filter_3_ct2_d16(),0,4nat);
    r::pass_step(raw_filter_3_ct2_d16(),a_filter_3_ct2_d16(),0,3nat);
    r::pass_step(raw_filter_3_ct2_d16(),a_filter_3_ct2_d16(),0,2nat);
    r::pass_step(raw_filter_3_ct2_d16(),a_filter_3_ct2_d16(),0,1nat);
    r::pass_step(raw_filter_3_ct2_d16(),a_filter_3_ct2_d16(),0,0nat);
    r::scan_step(raw_filter_3_ct2_d16(),a_filter_3_ct2_d16(),1);
    r::scan_step(raw_filter_3_ct2_d16(),a_filter_3_ct2_d16(),0);
}
#[verifier::spinoff_prover]
proof fn model_filter_3_ct2_d16()
    ensures r::still(bytes_filter_3_ct2_d16(),STRICT,false)==(r::Projection::Ready {width:6u32,height:4u32,bytes:96nat}),
{
    hide(bytes_filter_3_ct2_d16);hide(z_filter_3_ct2_d16);hide(raw_filter_3_ct2_d16);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_3_ct2_d16().len()==216nat) by (compute);
    assert(crate::model::mh(bytes_filter_3_ct2_d16())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:2u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_3_ct2_d16())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:2u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_3_ct2_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:159nat,next:204nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_3_ct2_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:159nat,next:204nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_3_ct2_d16(),204nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:212nat,len:0nat,next:216nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_3_ct2_d16(),204nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:212nat,len:0nat,next:216nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_3_ct2_d16().subrange(41,200)==z_filter_3_ct2_d16()) by (compute);
    walk_filter_3_ct2_d16(bytes_filter_3_ct2_d16());
    inflate_filter_3_ct2_d16();scan_filter_3_ct2_d16();
    assert(r::raw(a_filter_3_ct2_d16())==148nat) by (compute);
    assert(r::data(z_filter_3_ct2_d16(),a_filter_3_ct2_d16(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_filter_3_ct2_d16(),h_filter_3_ct2_d16(),z_filter_3_ct2_d16(),0nat,false);
}
fn public_filter_3_ct2_d16() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,2u8,0u8,0u8,0u8,114u8,246u8,5u8,87u8,0u8,0u8,0u8,159u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,148u8,0u8,107u8,255u8,3u8,88u8,198u8,168u8,128u8,137u8,73u8,61u8,149u8,151u8,61u8,0u8,85u8,11u8,120u8,15u8,139u8,66u8,208u8,3u8,30u8,249u8,215u8,232u8,50u8,49u8,18u8,0u8,63u8,71u8,246u8,55u8,134u8,219u8,9u8,29u8,213u8,3u8,19u8,234u8,188u8,93u8,133u8,4u8,53u8,86u8,195u8,189u8,136u8,167u8,131u8,20u8,22u8,120u8,110u8,52u8,112u8,138u8,27u8,15u8,237u8,51u8,11u8,157u8,114u8,191u8,132u8,52u8,127u8,75u8,30u8,147u8,145u8,8u8,3u8,54u8,135u8,147u8,240u8,241u8,104u8,210u8,94u8,198u8,184u8,22u8,91u8,236u8,7u8,22u8,2u8,141u8,148u8,85u8,139u8,150u8,83u8,24u8,225u8,19u8,141u8,199u8,135u8,30u8,97u8,37u8,51u8,221u8,50u8,221u8,142u8,3u8,157u8,97u8,65u8,205u8,154u8,154u8,176u8,168u8,165u8,200u8,189u8,213u8,208u8,86u8,90u8,224u8,120u8,107u8,222u8,104u8,52u8,152u8,158u8,49u8,3u8,108u8,35u8,142u8,248u8,42u8,225u8,68u8,193u8,201u8,98u8,94u8,235u8,81u8,65u8,134u8,230u8,230u8,77u8,137u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_3_ct2_d16());model_filter_3_ct2_d16();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==6u32 && s.height==4u32 && s.rgba@.len()==96nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// filter/3-ct4-d8; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_filter_3_ct4_d8() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,4u8,0u8,0u8,0u8,7u8,13u8,134u8,200u8,0u8,0u8,0u8,63u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,52u8,0u8,203u8,255u8,3u8,54u8,146u8,8u8,1u8,39u8,145u8,242u8,250u8,196u8,37u8,65u8,59u8,3u8,37u8,152u8,183u8,89u8,6u8,23u8,52u8,135u8,145u8,216u8,113u8,251u8,3u8,138u8,223u8,104u8,31u8,105u8,234u8,84u8,208u8,187u8,127u8,20u8,137u8,3u8,127u8,238u8,103u8,51u8,181u8,243u8,91u8,49u8,211u8,122u8,254u8,78u8,48u8,3u8,23u8,115u8,82u8,182u8,110u8,40u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_filter_3_ct4_d8() -> Seq<u8> {seq![120u8,156u8,1u8,52u8,0u8,203u8,255u8,3u8,54u8,146u8,8u8,1u8,39u8,145u8,242u8,250u8,196u8,37u8,65u8,59u8,3u8,37u8,152u8,183u8,89u8,6u8,23u8,52u8,135u8,145u8,216u8,113u8,251u8,3u8,138u8,223u8,104u8,31u8,105u8,234u8,84u8,208u8,187u8,127u8,20u8,137u8,3u8,127u8,238u8,103u8,51u8,181u8,243u8,91u8,49u8,211u8,122u8,254u8,78u8,48u8,3u8,23u8,115u8]}
spec fn h_filter_3_ct4_d8() -> crate::model::Ihdr {crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:4u8,interlace:0u8}}
spec fn a_filter_3_ct4_d8() -> r::Raster {r::shape(h_filter_3_ct4_d8(),6nat,4nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_filter_3_ct4_d8(b: Seq<u8>)
    requires b.len()==120,crate::model::mh(b)==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:4u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:63nat,next:108nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,108nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:116nat,len:0nat,next:120nat,crc_ok:true,synthetic:false})),
        b.subrange(41,104)==z_filter_3_ct4_d8(),
    ensures r::default_case(b,crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:4u8,interlace:0u8},z_filter_3_ct4_d8(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_filter_3_ct4_d8() -> Seq<u8> {seq![3u8,54u8,146u8,8u8,1u8,39u8,145u8,242u8,250u8,196u8,37u8,65u8,59u8,3u8,37u8,152u8,183u8,89u8,6u8,23u8,52u8,135u8,145u8,216u8,113u8,251u8,3u8,138u8,223u8,104u8,31u8,105u8,234u8,84u8,208u8,187u8,127u8,20u8,137u8,3u8,127u8,238u8,103u8,51u8,181u8,243u8,91u8,49u8,211u8,122u8,254u8,78u8]}
#[verifier::spinoff_prover]
proof fn shape_filter_3_ct4_d8()
    ensures raw_filter_3_ct4_d8().len()==52nat,
        z_filter_3_ct4_d8().len()==63nat,
        z_filter_3_ct4_d8()[0]==120u8,
        z_filter_3_ct4_d8()[1]==156u8,
        zm::field_value(z_filter_3_ct4_d8(),16,1)==1nat,
        zm::field_value(z_filter_3_ct4_d8(),17,2)==0nat,
        zm::be_value(z_filter_3_ct4_d8(),59nat,4)==805508979nat,
        z_filter_3_ct4_d8().subrange(7,59)==raw_filter_3_ct4_d8(),
        z_filter_3_ct4_d8()[3] as nat+256*z_filter_3_ct4_d8()[4] as nat==52nat,
        z_filter_3_ct4_d8()[5] as nat+256*z_filter_3_ct4_d8()[6] as nat==u16::MAX as nat-52nat,
        zm::be_value(z_filter_3_ct4_d8(),0,1)==120nat,
        zm::be_value(z_filter_3_ct4_d8(),1,1)==156nat,
{
    hide(z_filter_3_ct4_d8);hide(raw_filter_3_ct4_d8);
    assert(raw_filter_3_ct4_d8().len()==52nat) by (compute);
    assert(z_filter_3_ct4_d8().len()==63nat) by (compute);
    assert(z_filter_3_ct4_d8()[0]==120u8) by (compute);
    assert(z_filter_3_ct4_d8()[1]==156u8) by (compute);
    assert(zm::field_value(z_filter_3_ct4_d8(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_filter_3_ct4_d8(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_filter_3_ct4_d8(),59nat,4)==805508979nat) by (compute);
    assert(z_filter_3_ct4_d8().subrange(7,59)==raw_filter_3_ct4_d8()) by (compute);
    assert(z_filter_3_ct4_d8()[3] as nat+256*z_filter_3_ct4_d8()[4] as nat==52nat) by (compute);
    assert(z_filter_3_ct4_d8()[5] as nat+256*z_filter_3_ct4_d8()[6] as nat==u16::MAX as nat-52nat) by (compute);
    assert(zm::be_value(z_filter_3_ct4_d8(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_filter_3_ct4_d8(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_filter_3_ct4_d8()
    ensures zm::Adler(raw_filter_3_ct4_d8())==805508979nat,
{
    hide(raw_filter_3_ct4_d8);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_filter_3_ct4_d8().len()==52nat) by (compute);
    assert(c::sums(raw_filter_3_ct4_d8(),0,52nat)==(6002nat,143333nat)) by (compute);
    c::sums_correct(raw_filter_3_ct4_d8(),0,52nat);
    assert(raw_filter_3_ct4_d8().subrange(0,52) =~= raw_filter_3_ct4_d8());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((143333nat%65521)*(u16::MAX as nat+1)+(1+6002nat)%65521==805508979nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_filter_3_ct4_d8()
    ensures r::inflated(z_filter_3_ct4_d8(),a_filter_3_ct4_d8(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_filter_3_ct4_d8(),r2:false}),
        raw_filter_3_ct4_d8().len()==52nat,
{
    hide(z_filter_3_ct4_d8);hide(raw_filter_3_ct4_d8);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_filter_3_ct4_d8();checksum_filter_3_ct4_d8();
    assert(r::raw(a_filter_3_ct4_d8())==52nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_filter_3_ct4_d8(),raw_filter_3_ct4_d8(),52nat,120nat,156nat,805508979nat);
}
#[verifier::spinoff_prover]
proof fn scan_filter_3_ct4_d8()
    ensures r::scan(raw_filter_3_ct4_d8(),a_filter_3_ct4_d8(),0)==Ok(()),
{
    hide(raw_filter_3_ct4_d8);
    assert(r::base(a_filter_3_ct4_d8(),0)==0nat && r::ph(a_filter_3_ct4_d8(),0)==4nat && r::count(a_filter_3_ct4_d8())==1nat) by (compute);
    assert(r::rb(a_filter_3_ct4_d8(),0)==12nat) by (compute);
    assert(raw_filter_3_ct4_d8()[0]==3u8) by (compute);
    assert(r::row_result(raw_filter_3_ct4_d8(),a_filter_3_ct4_d8(),0,0nat)==Ok(()));
    assert(raw_filter_3_ct4_d8()[13]==3u8) by (compute);
    assert(r::row_result(raw_filter_3_ct4_d8(),a_filter_3_ct4_d8(),0,1nat)==Ok(()));
    assert(raw_filter_3_ct4_d8()[26]==3u8) by (compute);
    assert(r::row_result(raw_filter_3_ct4_d8(),a_filter_3_ct4_d8(),0,2nat)==Ok(()));
    assert(raw_filter_3_ct4_d8()[39]==3u8) by (compute);
    assert(r::row_result(raw_filter_3_ct4_d8(),a_filter_3_ct4_d8(),0,3nat)==Ok(()));
    r::pass_step(raw_filter_3_ct4_d8(),a_filter_3_ct4_d8(),0,4nat);
    r::pass_step(raw_filter_3_ct4_d8(),a_filter_3_ct4_d8(),0,3nat);
    r::pass_step(raw_filter_3_ct4_d8(),a_filter_3_ct4_d8(),0,2nat);
    r::pass_step(raw_filter_3_ct4_d8(),a_filter_3_ct4_d8(),0,1nat);
    r::pass_step(raw_filter_3_ct4_d8(),a_filter_3_ct4_d8(),0,0nat);
    r::scan_step(raw_filter_3_ct4_d8(),a_filter_3_ct4_d8(),1);
    r::scan_step(raw_filter_3_ct4_d8(),a_filter_3_ct4_d8(),0);
}
#[verifier::spinoff_prover]
proof fn model_filter_3_ct4_d8()
    ensures r::still(bytes_filter_3_ct4_d8(),STRICT,false)==(r::Projection::Ready {width:6u32,height:4u32,bytes:96nat}),
{
    hide(bytes_filter_3_ct4_d8);hide(z_filter_3_ct4_d8);hide(raw_filter_3_ct4_d8);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_3_ct4_d8().len()==120nat) by (compute);
    assert(crate::model::mh(bytes_filter_3_ct4_d8())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:4u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_3_ct4_d8())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:4u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_3_ct4_d8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:63nat,next:108nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_3_ct4_d8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:63nat,next:108nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_3_ct4_d8(),108nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:116nat,len:0nat,next:120nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_3_ct4_d8(),108nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:116nat,len:0nat,next:120nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_3_ct4_d8().subrange(41,104)==z_filter_3_ct4_d8()) by (compute);
    walk_filter_3_ct4_d8(bytes_filter_3_ct4_d8());
    inflate_filter_3_ct4_d8();scan_filter_3_ct4_d8();
    assert(r::raw(a_filter_3_ct4_d8())==52nat) by (compute);
    assert(r::data(z_filter_3_ct4_d8(),a_filter_3_ct4_d8(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_filter_3_ct4_d8(),h_filter_3_ct4_d8(),z_filter_3_ct4_d8(),0nat,false);
}
fn public_filter_3_ct4_d8() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,4u8,0u8,0u8,0u8,7u8,13u8,134u8,200u8,0u8,0u8,0u8,63u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,52u8,0u8,203u8,255u8,3u8,54u8,146u8,8u8,1u8,39u8,145u8,242u8,250u8,196u8,37u8,65u8,59u8,3u8,37u8,152u8,183u8,89u8,6u8,23u8,52u8,135u8,145u8,216u8,113u8,251u8,3u8,138u8,223u8,104u8,31u8,105u8,234u8,84u8,208u8,187u8,127u8,20u8,137u8,3u8,127u8,238u8,103u8,51u8,181u8,243u8,91u8,49u8,211u8,122u8,254u8,78u8,48u8,3u8,23u8,115u8,82u8,182u8,110u8,40u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_3_ct4_d8());model_filter_3_ct4_d8();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==6u32 && s.height==4u32 && s.rgba@.len()==96nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// filter/3-ct6-d16; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_filter_3_ct6_d16() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,6u8,0u8,0u8,0u8,253u8,148u8,146u8,0u8,0u8,0u8,0u8,207u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,196u8,0u8,59u8,255u8,3u8,204u8,96u8,89u8,23u8,104u8,37u8,152u8,142u8,65u8,69u8,202u8,242u8,165u8,230u8,137u8,115u8,58u8,244u8,25u8,184u8,104u8,197u8,200u8,230u8,215u8,59u8,155u8,128u8,96u8,164u8,211u8,189u8,110u8,116u8,28u8,124u8,147u8,212u8,8u8,210u8,223u8,158u8,140u8,182u8,120u8,255u8,189u8,241u8,3u8,21u8,11u8,10u8,221u8,109u8,192u8,202u8,32u8,226u8,248u8,73u8,53u8,141u8,69u8,97u8,73u8,61u8,119u8,131u8,59u8,189u8,23u8,61u8,5u8,135u8,86u8,119u8,249u8,157u8,8u8,108u8,88u8,102u8,216u8,204u8,111u8,191u8,138u8,193u8,240u8,131u8,174u8,245u8,51u8,119u8,57u8,219u8,126u8,3u8,150u8,36u8,214u8,21u8,111u8,239u8,51u8,80u8,200u8,103u8,45u8,17u8,44u8,29u8,105u8,12u8,31u8,243u8,195u8,65u8,14u8,182u8,192u8,176u8,103u8,227u8,118u8,149u8,230u8,65u8,116u8,255u8,11u8,105u8,214u8,56u8,190u8,195u8,158u8,162u8,127u8,169u8,11u8,134u8,223u8,96u8,36u8,118u8,3u8,153u8,185u8,105u8,165u8,165u8,219u8,242u8,110u8,47u8,219u8,19u8,216u8,224u8,175u8,169u8,116u8,70u8,14u8,201u8,136u8,230u8,73u8,67u8,45u8,70u8,217u8,7u8,167u8,125u8,85u8,195u8,77u8,106u8,103u8,208u8,65u8,32u8,157u8,213u8,233u8,163u8,247u8,246u8,218u8,13u8,55u8,142u8,107u8,237u8,8u8,98u8,251u8,81u8,136u8,38u8,157u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_filter_3_ct6_d16() -> Seq<u8> {seq![120u8,156u8,1u8,196u8,0u8,59u8,255u8,3u8,204u8,96u8,89u8,23u8,104u8,37u8,152u8,142u8,65u8,69u8,202u8,242u8,165u8,230u8,137u8,115u8,58u8,244u8,25u8,184u8,104u8,197u8,200u8,230u8,215u8,59u8,155u8,128u8,96u8,164u8,211u8,189u8,110u8,116u8,28u8,124u8,147u8,212u8,8u8,210u8,223u8,158u8,140u8,182u8,120u8,255u8,189u8,241u8,3u8,21u8,11u8,10u8,221u8,109u8,192u8,202u8,32u8,226u8,248u8,73u8,53u8,141u8,69u8,97u8,73u8,61u8,119u8,131u8,59u8,189u8,23u8,61u8,5u8,135u8,86u8,119u8,249u8,157u8,8u8,108u8,88u8,102u8,216u8,204u8,111u8,191u8,138u8,193u8,240u8,131u8,174u8,245u8,51u8,119u8,57u8,219u8,126u8,3u8,150u8,36u8,214u8,21u8,111u8,239u8,51u8,80u8,200u8,103u8,45u8,17u8,44u8,29u8,105u8,12u8,31u8,243u8,195u8,65u8,14u8,182u8,192u8,176u8,103u8,227u8,118u8,149u8,230u8,65u8,116u8,255u8,11u8,105u8,214u8,56u8,190u8,195u8,158u8,162u8,127u8,169u8,11u8,134u8,223u8,96u8,36u8,118u8,3u8,153u8,185u8,105u8,165u8,165u8,219u8,242u8,110u8,47u8,219u8,19u8,216u8,224u8,175u8,169u8,116u8,70u8,14u8,201u8,136u8,230u8,73u8,67u8,45u8,70u8,217u8,7u8,167u8,125u8,85u8,195u8,77u8,106u8,103u8,208u8,65u8,32u8,157u8,213u8,233u8,163u8,247u8,246u8,218u8,13u8,55u8,142u8,107u8,237u8,8u8,98u8,251u8]}
spec fn h_filter_3_ct6_d16() -> crate::model::Ihdr {crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:6u8,interlace:0u8}}
spec fn a_filter_3_ct6_d16() -> r::Raster {r::shape(h_filter_3_ct6_d16(),6nat,4nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_filter_3_ct6_d16(b: Seq<u8>)
    requires b.len()==264,crate::model::mh(b)==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:6u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:207nat,next:252nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,252nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:260nat,len:0nat,next:264nat,crc_ok:true,synthetic:false})),
        b.subrange(41,248)==z_filter_3_ct6_d16(),
    ensures r::default_case(b,crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:6u8,interlace:0u8},z_filter_3_ct6_d16(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
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
spec fn raw_filter_3_ct6_d16() -> Seq<u8> {seq![3u8,204u8,96u8,89u8,23u8,104u8,37u8,152u8,142u8,65u8,69u8,202u8,242u8,165u8,230u8,137u8,115u8,58u8,244u8,25u8,184u8,104u8,197u8,200u8,230u8,215u8,59u8,155u8,128u8,96u8,164u8,211u8,189u8,110u8,116u8,28u8,124u8,147u8,212u8,8u8,210u8,223u8,158u8,140u8,182u8,120u8,255u8,189u8,241u8,3u8,21u8,11u8,10u8,221u8,109u8,192u8,202u8,32u8,226u8,248u8,73u8,53u8,141u8,69u8,97u8,73u8,61u8,119u8,131u8,59u8,189u8,23u8,61u8,5u8,135u8,86u8,119u8,249u8,157u8,8u8,108u8,88u8,102u8,216u8,204u8,111u8,191u8,138u8,193u8,240u8,131u8,174u8,245u8,51u8,119u8,57u8,219u8,126u8,3u8,150u8,36u8,214u8,21u8,111u8,239u8,51u8,80u8,200u8,103u8,45u8,17u8,44u8,29u8,105u8,12u8,31u8,243u8,195u8,65u8,14u8,182u8,192u8,176u8,103u8,227u8,118u8,149u8,230u8,65u8,116u8,255u8,11u8,105u8,214u8,56u8,190u8,195u8,158u8,162u8,127u8,169u8,11u8,134u8,223u8,96u8,36u8,118u8,3u8,153u8,185u8,105u8,165u8,165u8,219u8,242u8,110u8,47u8,219u8,19u8,216u8,224u8,175u8,169u8,116u8,70u8,14u8,201u8,136u8,230u8,73u8,67u8,45u8,70u8,217u8,7u8,167u8,125u8,85u8,195u8,77u8,106u8,103u8,208u8,65u8,32u8,157u8,213u8,233u8,163u8,247u8,246u8,218u8,13u8,55u8,142u8,107u8]}
#[verifier::spinoff_prover]
proof fn shape_filter_3_ct6_d16()
    ensures raw_filter_3_ct6_d16().len()==196nat,
        z_filter_3_ct6_d16().len()==207nat,
        z_filter_3_ct6_d16()[0]==120u8,
        z_filter_3_ct6_d16()[1]==156u8,
        zm::field_value(z_filter_3_ct6_d16(),16,1)==1nat,
        zm::field_value(z_filter_3_ct6_d16(),17,2)==0nat,
        zm::be_value(z_filter_3_ct6_d16(),203nat,4)==3976749819nat,
        z_filter_3_ct6_d16().subrange(7,203)==raw_filter_3_ct6_d16(),
        z_filter_3_ct6_d16()[3] as nat+256*z_filter_3_ct6_d16()[4] as nat==196nat,
        z_filter_3_ct6_d16()[5] as nat+256*z_filter_3_ct6_d16()[6] as nat==u16::MAX as nat-196nat,
        zm::be_value(z_filter_3_ct6_d16(),0,1)==120nat,
        zm::be_value(z_filter_3_ct6_d16(),1,1)==156nat,
{
    hide(z_filter_3_ct6_d16);hide(raw_filter_3_ct6_d16);
    assert(raw_filter_3_ct6_d16().len()==196nat) by (compute);
    assert(z_filter_3_ct6_d16().len()==207nat) by (compute);
    assert(z_filter_3_ct6_d16()[0]==120u8) by (compute);
    assert(z_filter_3_ct6_d16()[1]==156u8) by (compute);
    assert(zm::field_value(z_filter_3_ct6_d16(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_filter_3_ct6_d16(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_filter_3_ct6_d16(),203nat,4)==3976749819nat) by (compute);
    assert(z_filter_3_ct6_d16().subrange(7,203)==raw_filter_3_ct6_d16()) by (compute);
    assert(z_filter_3_ct6_d16()[3] as nat+256*z_filter_3_ct6_d16()[4] as nat==196nat) by (compute);
    assert(z_filter_3_ct6_d16()[5] as nat+256*z_filter_3_ct6_d16()[6] as nat==u16::MAX as nat-196nat) by (compute);
    assert(zm::be_value(z_filter_3_ct6_d16(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_filter_3_ct6_d16(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_filter_3_ct6_d16()
    ensures zm::Adler(raw_filter_3_ct6_d16())==3976749819nat,
{
    hide(raw_filter_3_ct6_d16);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_filter_3_ct6_d16().len()==196nat) by (compute);
    assert(c::sums(raw_filter_3_ct6_d16(),0,196nat)==(25338nat,2484957nat)) by (compute);
    c::sums_correct(raw_filter_3_ct6_d16(),0,196nat);
    assert(raw_filter_3_ct6_d16().subrange(0,196) =~= raw_filter_3_ct6_d16());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((2484957nat%65521)*(u16::MAX as nat+1)+(1+25338nat)%65521==3976749819nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_filter_3_ct6_d16()
    ensures r::inflated(z_filter_3_ct6_d16(),a_filter_3_ct6_d16(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_filter_3_ct6_d16(),r2:false}),
        raw_filter_3_ct6_d16().len()==196nat,
{
    hide(z_filter_3_ct6_d16);hide(raw_filter_3_ct6_d16);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_filter_3_ct6_d16();checksum_filter_3_ct6_d16();
    assert(r::raw(a_filter_3_ct6_d16())==196nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_filter_3_ct6_d16(),raw_filter_3_ct6_d16(),196nat,120nat,156nat,3976749819nat);
}
#[verifier::spinoff_prover]
proof fn scan_filter_3_ct6_d16()
    ensures r::scan(raw_filter_3_ct6_d16(),a_filter_3_ct6_d16(),0)==Ok(()),
{
    hide(raw_filter_3_ct6_d16);
    assert(r::base(a_filter_3_ct6_d16(),0)==0nat && r::ph(a_filter_3_ct6_d16(),0)==4nat && r::count(a_filter_3_ct6_d16())==1nat) by (compute);
    assert(r::rb(a_filter_3_ct6_d16(),0)==48nat) by (compute);
    assert(raw_filter_3_ct6_d16()[0]==3u8) by (compute);
    assert(r::row_result(raw_filter_3_ct6_d16(),a_filter_3_ct6_d16(),0,0nat)==Ok(()));
    assert(raw_filter_3_ct6_d16()[49]==3u8) by (compute);
    assert(r::row_result(raw_filter_3_ct6_d16(),a_filter_3_ct6_d16(),0,1nat)==Ok(()));
    assert(raw_filter_3_ct6_d16()[98]==3u8) by (compute);
    assert(r::row_result(raw_filter_3_ct6_d16(),a_filter_3_ct6_d16(),0,2nat)==Ok(()));
    assert(raw_filter_3_ct6_d16()[147]==3u8) by (compute);
    assert(r::row_result(raw_filter_3_ct6_d16(),a_filter_3_ct6_d16(),0,3nat)==Ok(()));
    r::pass_step(raw_filter_3_ct6_d16(),a_filter_3_ct6_d16(),0,4nat);
    r::pass_step(raw_filter_3_ct6_d16(),a_filter_3_ct6_d16(),0,3nat);
    r::pass_step(raw_filter_3_ct6_d16(),a_filter_3_ct6_d16(),0,2nat);
    r::pass_step(raw_filter_3_ct6_d16(),a_filter_3_ct6_d16(),0,1nat);
    r::pass_step(raw_filter_3_ct6_d16(),a_filter_3_ct6_d16(),0,0nat);
    r::scan_step(raw_filter_3_ct6_d16(),a_filter_3_ct6_d16(),1);
    r::scan_step(raw_filter_3_ct6_d16(),a_filter_3_ct6_d16(),0);
}
#[verifier::spinoff_prover]
proof fn model_filter_3_ct6_d16()
    ensures r::still(bytes_filter_3_ct6_d16(),STRICT,false)==(r::Projection::Ready {width:6u32,height:4u32,bytes:96nat}),
{
    hide(bytes_filter_3_ct6_d16);hide(z_filter_3_ct6_d16);hide(raw_filter_3_ct6_d16);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_3_ct6_d16().len()==264nat) by (compute);
    assert(crate::model::mh(bytes_filter_3_ct6_d16())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:6u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_3_ct6_d16())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:6u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_3_ct6_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:207nat,next:252nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_3_ct6_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:207nat,next:252nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_3_ct6_d16(),252nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:260nat,len:0nat,next:264nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_3_ct6_d16(),252nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:260nat,len:0nat,next:264nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_3_ct6_d16().subrange(41,248)==z_filter_3_ct6_d16()) by (compute);
    walk_filter_3_ct6_d16(bytes_filter_3_ct6_d16());
    inflate_filter_3_ct6_d16();scan_filter_3_ct6_d16();
    assert(r::raw(a_filter_3_ct6_d16())==196nat) by (compute);
    assert(r::data(z_filter_3_ct6_d16(),a_filter_3_ct6_d16(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_filter_3_ct6_d16(),h_filter_3_ct6_d16(),z_filter_3_ct6_d16(),0nat,false);
}
fn public_filter_3_ct6_d16() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,6u8,0u8,0u8,0u8,253u8,148u8,146u8,0u8,0u8,0u8,0u8,207u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,196u8,0u8,59u8,255u8,3u8,204u8,96u8,89u8,23u8,104u8,37u8,152u8,142u8,65u8,69u8,202u8,242u8,165u8,230u8,137u8,115u8,58u8,244u8,25u8,184u8,104u8,197u8,200u8,230u8,215u8,59u8,155u8,128u8,96u8,164u8,211u8,189u8,110u8,116u8,28u8,124u8,147u8,212u8,8u8,210u8,223u8,158u8,140u8,182u8,120u8,255u8,189u8,241u8,3u8,21u8,11u8,10u8,221u8,109u8,192u8,202u8,32u8,226u8,248u8,73u8,53u8,141u8,69u8,97u8,73u8,61u8,119u8,131u8,59u8,189u8,23u8,61u8,5u8,135u8,86u8,119u8,249u8,157u8,8u8,108u8,88u8,102u8,216u8,204u8,111u8,191u8,138u8,193u8,240u8,131u8,174u8,245u8,51u8,119u8,57u8,219u8,126u8,3u8,150u8,36u8,214u8,21u8,111u8,239u8,51u8,80u8,200u8,103u8,45u8,17u8,44u8,29u8,105u8,12u8,31u8,243u8,195u8,65u8,14u8,182u8,192u8,176u8,103u8,227u8,118u8,149u8,230u8,65u8,116u8,255u8,11u8,105u8,214u8,56u8,190u8,195u8,158u8,162u8,127u8,169u8,11u8,134u8,223u8,96u8,36u8,118u8,3u8,153u8,185u8,105u8,165u8,165u8,219u8,242u8,110u8,47u8,219u8,19u8,216u8,224u8,175u8,169u8,116u8,70u8,14u8,201u8,136u8,230u8,73u8,67u8,45u8,70u8,217u8,7u8,167u8,125u8,85u8,195u8,77u8,106u8,103u8,208u8,65u8,32u8,157u8,213u8,233u8,163u8,247u8,246u8,218u8,13u8,55u8,142u8,107u8,237u8,8u8,98u8,251u8,81u8,136u8,38u8,157u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_3_ct6_d16());model_filter_3_ct6_d16();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==6u32 && s.height==4u32 && s.rgba@.len()==96nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// filter/4-ct0-d1; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_filter_4_ct0_d1() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,1u8,0u8,0u8,0u8,0u8,133u8,127u8,115u8,238u8,0u8,0u8,0u8,16u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,49u8,97u8,153u8,192u8,178u8,129u8,197u8,0u8,0u8,6u8,212u8,1u8,181u8,192u8,200u8,214u8,106u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_filter_4_ct0_d1() -> Seq<u8> {seq![120u8,156u8,99u8,49u8,97u8,153u8,192u8,178u8,129u8,197u8,0u8,0u8,6u8,212u8,1u8,181u8]}
spec fn h_filter_4_ct0_d1() -> crate::model::Ihdr {crate::model::Ihdr {width:6u32,height:4u32,depth:1u8,color_type:0u8,interlace:0u8}}
spec fn a_filter_4_ct0_d1() -> r::Raster {r::shape(h_filter_4_ct0_d1(),6nat,4nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_filter_4_ct0_d1(b: Seq<u8>)
    requires b.len()==73,crate::model::mh(b)==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:1u8,color_type:0u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:16nat,next:61nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,61nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:69nat,len:0nat,next:73nat,crc_ok:true,synthetic:false})),
        b.subrange(41,57)==z_filter_4_ct0_d1(),
    ensures r::default_case(b,crate::model::Ihdr {width:6u32,height:4u32,depth:1u8,color_type:0u8,interlace:0u8},z_filter_4_ct0_d1(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
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
spec fn raw_filter_4_ct0_d1() -> Seq<u8> {seq![4u8,52u8,4u8,144u8,4u8,176u8,4u8,48u8]}
spec fn ts_filter_4_ct0_d1() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:4nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:35nat,before:1nat,after:2nat,symbol:52nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:35nat,next:43nat,before:2nat,after:3nat,symbol:4nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:43nat,next:52nat,before:3nat,after:4nat,symbol:144nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:52nat,next:60nat,before:4nat,after:5nat,symbol:4nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:60nat,next:69nat,before:5nat,after:6nat,symbol:176nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:69nat,next:77nat,before:6nat,after:7nat,symbol:4nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:77nat,next:85nat,before:7nat,after:8nat,symbol:48nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:85nat,next:92nat,before:8nat,after:8nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_filter_4_ct0_d1()
    ensures t::trace(z_filter_4_ct0_d1(),raw_filter_4_ct0_d1(),ts_filter_4_ct0_d1(),0,9nat),
{
    hide(z_filter_4_ct0_d1);hide(raw_filter_4_ct0_d1);hide(ts_filter_4_ct0_d1);hide(t::valid_step);
    assert(t::trace(z_filter_4_ct0_d1(),raw_filter_4_ct0_d1(),ts_filter_4_ct0_d1(),0,9nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_filter_4_ct0_d1()
    ensures raw_filter_4_ct0_d1().len()==8nat,
        z_filter_4_ct0_d1().len()==16nat,
        z_filter_4_ct0_d1()[0]==120u8,
        z_filter_4_ct0_d1()[1]==156u8,
        zm::field_value(z_filter_4_ct0_d1(),16,1)==1nat,
        zm::field_value(z_filter_4_ct0_d1(),17,2)==1nat,
        zm::be_value(z_filter_4_ct0_d1(),12nat,4)==114557365nat,
        ts_filter_4_ct0_d1().len()==9nat,
        ts_filter_4_ct0_d1()[0].pos==19nat,
        ts_filter_4_ct0_d1()[0].before==0nat,
        ts_filter_4_ct0_d1().last().next==92nat,
{
    hide(z_filter_4_ct0_d1);hide(raw_filter_4_ct0_d1);hide(ts_filter_4_ct0_d1);
    assert(raw_filter_4_ct0_d1().len()==8nat) by (compute);
    assert(z_filter_4_ct0_d1().len()==16nat) by (compute);
    assert(z_filter_4_ct0_d1()[0]==120u8) by (compute);
    assert(z_filter_4_ct0_d1()[1]==156u8) by (compute);
    assert(zm::field_value(z_filter_4_ct0_d1(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_filter_4_ct0_d1(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_filter_4_ct0_d1(),12nat,4)==114557365nat) by (compute);
    assert(ts_filter_4_ct0_d1().len()==9nat) by (compute);
    assert(ts_filter_4_ct0_d1()[0].pos==19nat) by (compute);
    assert(ts_filter_4_ct0_d1()[0].before==0nat) by (compute);
    assert(ts_filter_4_ct0_d1().last().next==92nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_filter_4_ct0_d1()
    ensures zm::Adler(raw_filter_4_ct0_d1())==114557365nat,
{
    hide(raw_filter_4_ct0_d1);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_filter_4_ct0_d1().len()==8nat) by (compute);
    assert(c::sums(raw_filter_4_ct0_d1(),0,8nat)==(436nat,1748nat)) by (compute);
    c::sums_correct(raw_filter_4_ct0_d1(),0,8nat);
    assert(raw_filter_4_ct0_d1().subrange(0,8) =~= raw_filter_4_ct0_d1());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((1748nat%65521)*(u16::MAX as nat+1)+(1+436nat)%65521==114557365nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_filter_4_ct0_d1()
    ensures r::inflated(z_filter_4_ct0_d1(),a_filter_4_ct0_d1(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_filter_4_ct0_d1(),r2:false}),
        raw_filter_4_ct0_d1().len()==8nat,
{
    hide(z_filter_4_ct0_d1);hide(raw_filter_4_ct0_d1);hide(ts_filter_4_ct0_d1);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_filter_4_ct0_d1();checksum_filter_4_ct0_d1();
    assert(r::raw(a_filter_4_ct0_d1())==8nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_filter_4_ct0_d1();
    stream::fixed_stream(z_filter_4_ct0_d1(),raw_filter_4_ct0_d1(),ts_filter_4_ct0_d1(),8nat,156nat,114557365nat);
}
#[verifier::spinoff_prover]
proof fn scan_filter_4_ct0_d1()
    ensures r::scan(raw_filter_4_ct0_d1(),a_filter_4_ct0_d1(),0)==Ok(()),
{
    hide(raw_filter_4_ct0_d1);
    assert(r::base(a_filter_4_ct0_d1(),0)==0nat && r::ph(a_filter_4_ct0_d1(),0)==4nat && r::count(a_filter_4_ct0_d1())==1nat) by (compute);
    assert(r::rb(a_filter_4_ct0_d1(),0)==1nat) by (compute);
    assert(raw_filter_4_ct0_d1()[0]==4u8) by (compute);
    assert(r::row_result(raw_filter_4_ct0_d1(),a_filter_4_ct0_d1(),0,0nat)==Ok(()));
    assert(raw_filter_4_ct0_d1()[2]==4u8) by (compute);
    assert(r::row_result(raw_filter_4_ct0_d1(),a_filter_4_ct0_d1(),0,1nat)==Ok(()));
    assert(raw_filter_4_ct0_d1()[4]==4u8) by (compute);
    assert(r::row_result(raw_filter_4_ct0_d1(),a_filter_4_ct0_d1(),0,2nat)==Ok(()));
    assert(raw_filter_4_ct0_d1()[6]==4u8) by (compute);
    assert(r::row_result(raw_filter_4_ct0_d1(),a_filter_4_ct0_d1(),0,3nat)==Ok(()));
    r::pass_step(raw_filter_4_ct0_d1(),a_filter_4_ct0_d1(),0,4nat);
    r::pass_step(raw_filter_4_ct0_d1(),a_filter_4_ct0_d1(),0,3nat);
    r::pass_step(raw_filter_4_ct0_d1(),a_filter_4_ct0_d1(),0,2nat);
    r::pass_step(raw_filter_4_ct0_d1(),a_filter_4_ct0_d1(),0,1nat);
    r::pass_step(raw_filter_4_ct0_d1(),a_filter_4_ct0_d1(),0,0nat);
    r::scan_step(raw_filter_4_ct0_d1(),a_filter_4_ct0_d1(),1);
    r::scan_step(raw_filter_4_ct0_d1(),a_filter_4_ct0_d1(),0);
}
#[verifier::spinoff_prover]
proof fn model_filter_4_ct0_d1()
    ensures r::still(bytes_filter_4_ct0_d1(),STRICT,false)==(r::Projection::Ready {width:6u32,height:4u32,bytes:96nat}),
{
    hide(bytes_filter_4_ct0_d1);hide(z_filter_4_ct0_d1);hide(raw_filter_4_ct0_d1);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_4_ct0_d1().len()==73nat) by (compute);
    assert(crate::model::mh(bytes_filter_4_ct0_d1())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:1u8,color_type:0u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_4_ct0_d1())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:1u8,color_type:0u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 1u8)==2u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_4_ct0_d1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:16nat,next:61nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_4_ct0_d1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:16nat,next:61nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_4_ct0_d1(),61nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:69nat,len:0nat,next:73nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_4_ct0_d1(),61nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:69nat,len:0nat,next:73nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_4_ct0_d1().subrange(41,57)==z_filter_4_ct0_d1()) by (compute);
    walk_filter_4_ct0_d1(bytes_filter_4_ct0_d1());
    inflate_filter_4_ct0_d1();scan_filter_4_ct0_d1();
    assert(r::raw(a_filter_4_ct0_d1())==8nat) by (compute);
    assert(r::data(z_filter_4_ct0_d1(),a_filter_4_ct0_d1(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_filter_4_ct0_d1(),h_filter_4_ct0_d1(),z_filter_4_ct0_d1(),0nat,false);
}
fn public_filter_4_ct0_d1() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,1u8,0u8,0u8,0u8,0u8,133u8,127u8,115u8,238u8,0u8,0u8,0u8,16u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,49u8,97u8,153u8,192u8,178u8,129u8,197u8,0u8,0u8,6u8,212u8,1u8,181u8,192u8,200u8,214u8,106u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_4_ct0_d1());model_filter_4_ct0_d1();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==6u32 && s.height==4u32 && s.rgba@.len()==96nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// filter/4-ct0-d8; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_filter_4_ct0_d8() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,0u8,0u8,0u8,0u8,136u8,111u8,17u8,159u8,0u8,0u8,0u8,37u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,241u8,139u8,94u8,48u8,189u8,117u8,21u8,75u8,221u8,169u8,183u8,28u8,230u8,234u8,44u8,110u8,90u8,219u8,174u8,20u8,213u8,177u8,20u8,71u8,111u8,108u8,102u8,218u8,11u8,0u8,161u8,221u8,11u8,102u8,173u8,20u8,183u8,50u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_filter_4_ct0_d8() -> Seq<u8> {seq![120u8,156u8,99u8,241u8,139u8,94u8,48u8,189u8,117u8,21u8,75u8,221u8,169u8,183u8,28u8,230u8,234u8,44u8,110u8,90u8,219u8,174u8,20u8,213u8,177u8,20u8,71u8,111u8,108u8,102u8,218u8,11u8,0u8,161u8,221u8,11u8,102u8]}
spec fn h_filter_4_ct0_d8() -> crate::model::Ihdr {crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:0u8,interlace:0u8}}
spec fn a_filter_4_ct0_d8() -> r::Raster {r::shape(h_filter_4_ct0_d8(),6nat,4nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_filter_4_ct0_d8(b: Seq<u8>)
    requires b.len()==94,crate::model::mh(b)==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:0u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:37nat,next:82nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,82nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:90nat,len:0nat,next:94nat,crc_ok:true,synthetic:false})),
        b.subrange(41,78)==z_filter_4_ct0_d8(),
    ensures r::default_case(b,crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:0u8,interlace:0u8},z_filter_4_ct0_d8(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_filter_4_ct0_d8() -> Seq<u8> {seq![4u8,78u8,91u8,160u8,151u8,133u8,170u8,4u8,126u8,202u8,237u8,8u8,55u8,39u8,4u8,70u8,42u8,182u8,212u8,114u8,126u8,4u8,115u8,91u8,177u8,131u8,2u8,189u8]}
spec fn ts_filter_4_ct0_d8() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:4nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:35nat,before:1nat,after:2nat,symbol:78nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:35nat,next:43nat,before:2nat,after:3nat,symbol:91nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:43nat,next:52nat,before:3nat,after:4nat,symbol:160nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:52nat,next:61nat,before:4nat,after:5nat,symbol:151nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:61nat,next:69nat,before:5nat,after:6nat,symbol:133nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:69nat,next:78nat,before:6nat,after:7nat,symbol:170nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:78nat,next:86nat,before:7nat,after:8nat,symbol:4nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:86nat,next:94nat,before:8nat,after:9nat,symbol:126nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:94nat,next:103nat,before:9nat,after:10nat,symbol:202nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:103nat,next:112nat,before:10nat,after:11nat,symbol:237nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:112nat,next:120nat,before:11nat,after:12nat,symbol:8nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:120nat,next:128nat,before:12nat,after:13nat,symbol:55nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:128nat,next:136nat,before:13nat,after:14nat,symbol:39nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:136nat,next:144nat,before:14nat,after:15nat,symbol:4nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:144nat,next:152nat,before:15nat,after:16nat,symbol:70nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:152nat,next:160nat,before:16nat,after:17nat,symbol:42nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:160nat,next:169nat,before:17nat,after:18nat,symbol:182nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:169nat,next:178nat,before:18nat,after:19nat,symbol:212nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:178nat,next:186nat,before:19nat,after:20nat,symbol:114nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:186nat,next:194nat,before:20nat,after:21nat,symbol:126nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:194nat,next:202nat,before:21nat,after:22nat,symbol:4nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:202nat,next:210nat,before:22nat,after:23nat,symbol:115nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:210nat,next:218nat,before:23nat,after:24nat,symbol:91nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:218nat,next:227nat,before:24nat,after:25nat,symbol:177nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:227nat,next:235nat,before:25nat,after:26nat,symbol:131nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:235nat,next:243nat,before:26nat,after:27nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:243nat,next:252nat,before:27nat,after:28nat,symbol:189nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:252nat,next:259nat,before:28nat,after:28nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_filter_4_ct0_d8()
    ensures t::trace(z_filter_4_ct0_d8(),raw_filter_4_ct0_d8(),ts_filter_4_ct0_d8(),0,29nat),
{
    hide(z_filter_4_ct0_d8);hide(raw_filter_4_ct0_d8);hide(ts_filter_4_ct0_d8);hide(t::valid_step);
    assert(t::trace(z_filter_4_ct0_d8(),raw_filter_4_ct0_d8(),ts_filter_4_ct0_d8(),0,29nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_filter_4_ct0_d8()
    ensures raw_filter_4_ct0_d8().len()==28nat,
        z_filter_4_ct0_d8().len()==37nat,
        z_filter_4_ct0_d8()[0]==120u8,
        z_filter_4_ct0_d8()[1]==156u8,
        zm::field_value(z_filter_4_ct0_d8(),16,1)==1nat,
        zm::field_value(z_filter_4_ct0_d8(),17,2)==1nat,
        zm::be_value(z_filter_4_ct0_d8(),33nat,4)==2715618150nat,
        ts_filter_4_ct0_d8().len()==29nat,
        ts_filter_4_ct0_d8()[0].pos==19nat,
        ts_filter_4_ct0_d8()[0].before==0nat,
        ts_filter_4_ct0_d8().last().next==259nat,
{
    hide(z_filter_4_ct0_d8);hide(raw_filter_4_ct0_d8);hide(ts_filter_4_ct0_d8);
    assert(raw_filter_4_ct0_d8().len()==28nat) by (compute);
    assert(z_filter_4_ct0_d8().len()==37nat) by (compute);
    assert(z_filter_4_ct0_d8()[0]==120u8) by (compute);
    assert(z_filter_4_ct0_d8()[1]==156u8) by (compute);
    assert(zm::field_value(z_filter_4_ct0_d8(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_filter_4_ct0_d8(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_filter_4_ct0_d8(),33nat,4)==2715618150nat) by (compute);
    assert(ts_filter_4_ct0_d8().len()==29nat) by (compute);
    assert(ts_filter_4_ct0_d8()[0].pos==19nat) by (compute);
    assert(ts_filter_4_ct0_d8()[0].before==0nat) by (compute);
    assert(ts_filter_4_ct0_d8().last().next==259nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_filter_4_ct0_d8()
    ensures zm::Adler(raw_filter_4_ct0_d8())==2715618150nat,
{
    hide(raw_filter_4_ct0_d8);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_filter_4_ct0_d8().len()==28nat) by (compute);
    assert(c::sums(raw_filter_4_ct0_d8(),0,28nat)==(2917nat,41437nat)) by (compute);
    c::sums_correct(raw_filter_4_ct0_d8(),0,28nat);
    assert(raw_filter_4_ct0_d8().subrange(0,28) =~= raw_filter_4_ct0_d8());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((41437nat%65521)*(u16::MAX as nat+1)+(1+2917nat)%65521==2715618150nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_filter_4_ct0_d8()
    ensures r::inflated(z_filter_4_ct0_d8(),a_filter_4_ct0_d8(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_filter_4_ct0_d8(),r2:false}),
        raw_filter_4_ct0_d8().len()==28nat,
{
    hide(z_filter_4_ct0_d8);hide(raw_filter_4_ct0_d8);hide(ts_filter_4_ct0_d8);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_filter_4_ct0_d8();checksum_filter_4_ct0_d8();
    assert(r::raw(a_filter_4_ct0_d8())==28nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_filter_4_ct0_d8();
    stream::fixed_stream(z_filter_4_ct0_d8(),raw_filter_4_ct0_d8(),ts_filter_4_ct0_d8(),28nat,156nat,2715618150nat);
}
#[verifier::spinoff_prover]
proof fn scan_filter_4_ct0_d8()
    ensures r::scan(raw_filter_4_ct0_d8(),a_filter_4_ct0_d8(),0)==Ok(()),
{
    hide(raw_filter_4_ct0_d8);
    assert(r::base(a_filter_4_ct0_d8(),0)==0nat && r::ph(a_filter_4_ct0_d8(),0)==4nat && r::count(a_filter_4_ct0_d8())==1nat) by (compute);
    assert(r::rb(a_filter_4_ct0_d8(),0)==6nat) by (compute);
    assert(raw_filter_4_ct0_d8()[0]==4u8) by (compute);
    assert(r::row_result(raw_filter_4_ct0_d8(),a_filter_4_ct0_d8(),0,0nat)==Ok(()));
    assert(raw_filter_4_ct0_d8()[7]==4u8) by (compute);
    assert(r::row_result(raw_filter_4_ct0_d8(),a_filter_4_ct0_d8(),0,1nat)==Ok(()));
    assert(raw_filter_4_ct0_d8()[14]==4u8) by (compute);
    assert(r::row_result(raw_filter_4_ct0_d8(),a_filter_4_ct0_d8(),0,2nat)==Ok(()));
    assert(raw_filter_4_ct0_d8()[21]==4u8) by (compute);
    assert(r::row_result(raw_filter_4_ct0_d8(),a_filter_4_ct0_d8(),0,3nat)==Ok(()));
    r::pass_step(raw_filter_4_ct0_d8(),a_filter_4_ct0_d8(),0,4nat);
    r::pass_step(raw_filter_4_ct0_d8(),a_filter_4_ct0_d8(),0,3nat);
    r::pass_step(raw_filter_4_ct0_d8(),a_filter_4_ct0_d8(),0,2nat);
    r::pass_step(raw_filter_4_ct0_d8(),a_filter_4_ct0_d8(),0,1nat);
    r::pass_step(raw_filter_4_ct0_d8(),a_filter_4_ct0_d8(),0,0nat);
    r::scan_step(raw_filter_4_ct0_d8(),a_filter_4_ct0_d8(),1);
    r::scan_step(raw_filter_4_ct0_d8(),a_filter_4_ct0_d8(),0);
}
#[verifier::spinoff_prover]
proof fn model_filter_4_ct0_d8()
    ensures r::still(bytes_filter_4_ct0_d8(),STRICT,false)==(r::Projection::Ready {width:6u32,height:4u32,bytes:96nat}),
{
    hide(bytes_filter_4_ct0_d8);hide(z_filter_4_ct0_d8);hide(raw_filter_4_ct0_d8);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_4_ct0_d8().len()==94nat) by (compute);
    assert(crate::model::mh(bytes_filter_4_ct0_d8())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:0u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_4_ct0_d8())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:0u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_4_ct0_d8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:37nat,next:82nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_4_ct0_d8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:37nat,next:82nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_4_ct0_d8(),82nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:90nat,len:0nat,next:94nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_4_ct0_d8(),82nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:90nat,len:0nat,next:94nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_4_ct0_d8().subrange(41,78)==z_filter_4_ct0_d8()) by (compute);
    walk_filter_4_ct0_d8(bytes_filter_4_ct0_d8());
    inflate_filter_4_ct0_d8();scan_filter_4_ct0_d8();
    assert(r::raw(a_filter_4_ct0_d8())==28nat) by (compute);
    assert(r::data(z_filter_4_ct0_d8(),a_filter_4_ct0_d8(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_filter_4_ct0_d8(),h_filter_4_ct0_d8(),z_filter_4_ct0_d8(),0nat,false);
}
fn public_filter_4_ct0_d8() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,0u8,0u8,0u8,0u8,136u8,111u8,17u8,159u8,0u8,0u8,0u8,37u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,241u8,139u8,94u8,48u8,189u8,117u8,21u8,75u8,221u8,169u8,183u8,28u8,230u8,234u8,44u8,110u8,90u8,219u8,174u8,20u8,213u8,177u8,20u8,71u8,111u8,108u8,102u8,218u8,11u8,0u8,161u8,221u8,11u8,102u8,173u8,20u8,183u8,50u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_4_ct0_d8());model_filter_4_ct0_d8();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==6u32 && s.height==4u32 && s.rgba@.len()==96nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// filter/4-ct0-d16; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_filter_4_ct0_d16() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,0u8,0u8,0u8,0u8,216u8,255u8,205u8,220u8,0u8,0u8,0u8,63u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,52u8,0u8,203u8,255u8,4u8,229u8,131u8,34u8,77u8,0u8,58u8,22u8,25u8,70u8,122u8,112u8,105u8,4u8,179u8,42u8,82u8,221u8,132u8,71u8,24u8,148u8,51u8,131u8,52u8,60u8,4u8,193u8,243u8,100u8,92u8,190u8,159u8,149u8,37u8,225u8,185u8,245u8,244u8,4u8,174u8,164u8,115u8,48u8,164u8,224u8,131u8,231u8,82u8,131u8,33u8,172u8,20u8,98u8,23u8,38u8,193u8,105u8,106u8,83u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_filter_4_ct0_d16() -> Seq<u8> {seq![120u8,156u8,1u8,52u8,0u8,203u8,255u8,4u8,229u8,131u8,34u8,77u8,0u8,58u8,22u8,25u8,70u8,122u8,112u8,105u8,4u8,179u8,42u8,82u8,221u8,132u8,71u8,24u8,148u8,51u8,131u8,52u8,60u8,4u8,193u8,243u8,100u8,92u8,190u8,159u8,149u8,37u8,225u8,185u8,245u8,244u8,4u8,174u8,164u8,115u8,48u8,164u8,224u8,131u8,231u8,82u8,131u8,33u8,172u8,20u8,98u8,23u8,38u8]}
spec fn h_filter_4_ct0_d16() -> crate::model::Ihdr {crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:0u8,interlace:0u8}}
spec fn a_filter_4_ct0_d16() -> r::Raster {r::shape(h_filter_4_ct0_d16(),6nat,4nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_filter_4_ct0_d16(b: Seq<u8>)
    requires b.len()==120,crate::model::mh(b)==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:0u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:63nat,next:108nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,108nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:116nat,len:0nat,next:120nat,crc_ok:true,synthetic:false})),
        b.subrange(41,104)==z_filter_4_ct0_d16(),
    ensures r::default_case(b,crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:0u8,interlace:0u8},z_filter_4_ct0_d16(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
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
spec fn raw_filter_4_ct0_d16() -> Seq<u8> {seq![4u8,229u8,131u8,34u8,77u8,0u8,58u8,22u8,25u8,70u8,122u8,112u8,105u8,4u8,179u8,42u8,82u8,221u8,132u8,71u8,24u8,148u8,51u8,131u8,52u8,60u8,4u8,193u8,243u8,100u8,92u8,190u8,159u8,149u8,37u8,225u8,185u8,245u8,244u8,4u8,174u8,164u8,115u8,48u8,164u8,224u8,131u8,231u8,82u8,131u8,33u8,172u8]}
#[verifier::spinoff_prover]
proof fn shape_filter_4_ct0_d16()
    ensures raw_filter_4_ct0_d16().len()==52nat,
        z_filter_4_ct0_d16().len()==63nat,
        z_filter_4_ct0_d16()[0]==120u8,
        z_filter_4_ct0_d16()[1]==156u8,
        zm::field_value(z_filter_4_ct0_d16(),16,1)==1nat,
        zm::field_value(z_filter_4_ct0_d16(),17,2)==0nat,
        zm::be_value(z_filter_4_ct0_d16(),59nat,4)==341972774nat,
        z_filter_4_ct0_d16().subrange(7,59)==raw_filter_4_ct0_d16(),
        z_filter_4_ct0_d16()[3] as nat+256*z_filter_4_ct0_d16()[4] as nat==52nat,
        z_filter_4_ct0_d16()[5] as nat+256*z_filter_4_ct0_d16()[6] as nat==u16::MAX as nat-52nat,
        zm::be_value(z_filter_4_ct0_d16(),0,1)==120nat,
        zm::be_value(z_filter_4_ct0_d16(),1,1)==156nat,
{
    hide(z_filter_4_ct0_d16);hide(raw_filter_4_ct0_d16);
    assert(raw_filter_4_ct0_d16().len()==52nat) by (compute);
    assert(z_filter_4_ct0_d16().len()==63nat) by (compute);
    assert(z_filter_4_ct0_d16()[0]==120u8) by (compute);
    assert(z_filter_4_ct0_d16()[1]==156u8) by (compute);
    assert(zm::field_value(z_filter_4_ct0_d16(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_filter_4_ct0_d16(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_filter_4_ct0_d16(),59nat,4)==341972774nat) by (compute);
    assert(z_filter_4_ct0_d16().subrange(7,59)==raw_filter_4_ct0_d16()) by (compute);
    assert(z_filter_4_ct0_d16()[3] as nat+256*z_filter_4_ct0_d16()[4] as nat==52nat) by (compute);
    assert(z_filter_4_ct0_d16()[5] as nat+256*z_filter_4_ct0_d16()[6] as nat==u16::MAX as nat-52nat) by (compute);
    assert(zm::be_value(z_filter_4_ct0_d16(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_filter_4_ct0_d16(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_filter_4_ct0_d16()
    ensures zm::Adler(raw_filter_4_ct0_d16())==341972774nat,
{
    hide(raw_filter_4_ct0_d16);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_filter_4_ct0_d16().len()==52nat) by (compute);
    assert(c::sums(raw_filter_4_ct0_d16(),0,52nat)==(5925nat,136260nat)) by (compute);
    c::sums_correct(raw_filter_4_ct0_d16(),0,52nat);
    assert(raw_filter_4_ct0_d16().subrange(0,52) =~= raw_filter_4_ct0_d16());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((136260nat%65521)*(u16::MAX as nat+1)+(1+5925nat)%65521==341972774nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_filter_4_ct0_d16()
    ensures r::inflated(z_filter_4_ct0_d16(),a_filter_4_ct0_d16(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_filter_4_ct0_d16(),r2:false}),
        raw_filter_4_ct0_d16().len()==52nat,
{
    hide(z_filter_4_ct0_d16);hide(raw_filter_4_ct0_d16);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_filter_4_ct0_d16();checksum_filter_4_ct0_d16();
    assert(r::raw(a_filter_4_ct0_d16())==52nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_filter_4_ct0_d16(),raw_filter_4_ct0_d16(),52nat,120nat,156nat,341972774nat);
}
#[verifier::spinoff_prover]
proof fn scan_filter_4_ct0_d16()
    ensures r::scan(raw_filter_4_ct0_d16(),a_filter_4_ct0_d16(),0)==Ok(()),
{
    hide(raw_filter_4_ct0_d16);
    assert(r::base(a_filter_4_ct0_d16(),0)==0nat && r::ph(a_filter_4_ct0_d16(),0)==4nat && r::count(a_filter_4_ct0_d16())==1nat) by (compute);
    assert(r::rb(a_filter_4_ct0_d16(),0)==12nat) by (compute);
    assert(raw_filter_4_ct0_d16()[0]==4u8) by (compute);
    assert(r::row_result(raw_filter_4_ct0_d16(),a_filter_4_ct0_d16(),0,0nat)==Ok(()));
    assert(raw_filter_4_ct0_d16()[13]==4u8) by (compute);
    assert(r::row_result(raw_filter_4_ct0_d16(),a_filter_4_ct0_d16(),0,1nat)==Ok(()));
    assert(raw_filter_4_ct0_d16()[26]==4u8) by (compute);
    assert(r::row_result(raw_filter_4_ct0_d16(),a_filter_4_ct0_d16(),0,2nat)==Ok(()));
    assert(raw_filter_4_ct0_d16()[39]==4u8) by (compute);
    assert(r::row_result(raw_filter_4_ct0_d16(),a_filter_4_ct0_d16(),0,3nat)==Ok(()));
    r::pass_step(raw_filter_4_ct0_d16(),a_filter_4_ct0_d16(),0,4nat);
    r::pass_step(raw_filter_4_ct0_d16(),a_filter_4_ct0_d16(),0,3nat);
    r::pass_step(raw_filter_4_ct0_d16(),a_filter_4_ct0_d16(),0,2nat);
    r::pass_step(raw_filter_4_ct0_d16(),a_filter_4_ct0_d16(),0,1nat);
    r::pass_step(raw_filter_4_ct0_d16(),a_filter_4_ct0_d16(),0,0nat);
    r::scan_step(raw_filter_4_ct0_d16(),a_filter_4_ct0_d16(),1);
    r::scan_step(raw_filter_4_ct0_d16(),a_filter_4_ct0_d16(),0);
}
#[verifier::spinoff_prover]
proof fn model_filter_4_ct0_d16()
    ensures r::still(bytes_filter_4_ct0_d16(),STRICT,false)==(r::Projection::Ready {width:6u32,height:4u32,bytes:96nat}),
{
    hide(bytes_filter_4_ct0_d16);hide(z_filter_4_ct0_d16);hide(raw_filter_4_ct0_d16);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_4_ct0_d16().len()==120nat) by (compute);
    assert(crate::model::mh(bytes_filter_4_ct0_d16())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:0u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_4_ct0_d16())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:0u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_4_ct0_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:63nat,next:108nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_4_ct0_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:63nat,next:108nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_4_ct0_d16(),108nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:116nat,len:0nat,next:120nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_4_ct0_d16(),108nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:116nat,len:0nat,next:120nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_4_ct0_d16().subrange(41,104)==z_filter_4_ct0_d16()) by (compute);
    walk_filter_4_ct0_d16(bytes_filter_4_ct0_d16());
    inflate_filter_4_ct0_d16();scan_filter_4_ct0_d16();
    assert(r::raw(a_filter_4_ct0_d16())==52nat) by (compute);
    assert(r::data(z_filter_4_ct0_d16(),a_filter_4_ct0_d16(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_filter_4_ct0_d16(),h_filter_4_ct0_d16(),z_filter_4_ct0_d16(),0nat,false);
}
fn public_filter_4_ct0_d16() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,0u8,0u8,0u8,0u8,216u8,255u8,205u8,220u8,0u8,0u8,0u8,63u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,52u8,0u8,203u8,255u8,4u8,229u8,131u8,34u8,77u8,0u8,58u8,22u8,25u8,70u8,122u8,112u8,105u8,4u8,179u8,42u8,82u8,221u8,132u8,71u8,24u8,148u8,51u8,131u8,52u8,60u8,4u8,193u8,243u8,100u8,92u8,190u8,159u8,149u8,37u8,225u8,185u8,245u8,244u8,4u8,174u8,164u8,115u8,48u8,164u8,224u8,131u8,231u8,82u8,131u8,33u8,172u8,20u8,98u8,23u8,38u8,193u8,105u8,106u8,83u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_4_ct0_d16());model_filter_4_ct0_d16();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==6u32 && s.height==4u32 && s.rgba@.len()==96nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// filter/4-ct2-d8; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_filter_4_ct2_d8() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,2u8,0u8,0u8,0u8,34u8,102u8,217u8,20u8,0u8,0u8,0u8,87u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,76u8,0u8,179u8,255u8,4u8,101u8,154u8,20u8,240u8,105u8,56u8,197u8,69u8,172u8,125u8,212u8,177u8,19u8,122u8,142u8,34u8,192u8,192u8,4u8,118u8,122u8,212u8,210u8,154u8,11u8,67u8,245u8,245u8,109u8,113u8,4u8,46u8,185u8,47u8,214u8,140u8,164u8,4u8,35u8,225u8,161u8,88u8,48u8,158u8,250u8,201u8,35u8,134u8,165u8,151u8,120u8,229u8,56u8,156u8,22u8,184u8,4u8,103u8,166u8,150u8,102u8,43u8,215u8,205u8,25u8,81u8,75u8,252u8,113u8,112u8,154u8,253u8,0u8,164u8,104u8,129u8,55u8,37u8,15u8,188u8,226u8,23u8,84u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_filter_4_ct2_d8() -> Seq<u8> {seq![120u8,156u8,1u8,76u8,0u8,179u8,255u8,4u8,101u8,154u8,20u8,240u8,105u8,56u8,197u8,69u8,172u8,125u8,212u8,177u8,19u8,122u8,142u8,34u8,192u8,192u8,4u8,118u8,122u8,212u8,210u8,154u8,11u8,67u8,245u8,245u8,109u8,113u8,4u8,46u8,185u8,47u8,214u8,140u8,164u8,4u8,35u8,225u8,161u8,88u8,48u8,158u8,250u8,201u8,35u8,134u8,165u8,151u8,120u8,229u8,56u8,156u8,22u8,184u8,4u8,103u8,166u8,150u8,102u8,43u8,215u8,205u8,25u8,81u8,75u8,252u8,113u8,112u8,154u8,253u8,0u8,164u8,104u8,129u8,55u8,37u8,15u8]}
spec fn h_filter_4_ct2_d8() -> crate::model::Ihdr {crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:2u8,interlace:0u8}}
spec fn a_filter_4_ct2_d8() -> r::Raster {r::shape(h_filter_4_ct2_d8(),6nat,4nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_filter_4_ct2_d8(b: Seq<u8>)
    requires b.len()==144,crate::model::mh(b)==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:2u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:87nat,next:132nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,132nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:140nat,len:0nat,next:144nat,crc_ok:true,synthetic:false})),
        b.subrange(41,128)==z_filter_4_ct2_d8(),
    ensures r::default_case(b,crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:2u8,interlace:0u8},z_filter_4_ct2_d8(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_filter_4_ct2_d8() -> Seq<u8> {seq![4u8,101u8,154u8,20u8,240u8,105u8,56u8,197u8,69u8,172u8,125u8,212u8,177u8,19u8,122u8,142u8,34u8,192u8,192u8,4u8,118u8,122u8,212u8,210u8,154u8,11u8,67u8,245u8,245u8,109u8,113u8,4u8,46u8,185u8,47u8,214u8,140u8,164u8,4u8,35u8,225u8,161u8,88u8,48u8,158u8,250u8,201u8,35u8,134u8,165u8,151u8,120u8,229u8,56u8,156u8,22u8,184u8,4u8,103u8,166u8,150u8,102u8,43u8,215u8,205u8,25u8,81u8,75u8,252u8,113u8,112u8,154u8,253u8,0u8,164u8,104u8]}
#[verifier::spinoff_prover]
proof fn shape_filter_4_ct2_d8()
    ensures raw_filter_4_ct2_d8().len()==76nat,
        z_filter_4_ct2_d8().len()==87nat,
        z_filter_4_ct2_d8()[0]==120u8,
        z_filter_4_ct2_d8()[1]==156u8,
        zm::field_value(z_filter_4_ct2_d8(),16,1)==1nat,
        zm::field_value(z_filter_4_ct2_d8(),17,2)==0nat,
        zm::be_value(z_filter_4_ct2_d8(),83nat,4)==2167874831nat,
        z_filter_4_ct2_d8().subrange(7,83)==raw_filter_4_ct2_d8(),
        z_filter_4_ct2_d8()[3] as nat+256*z_filter_4_ct2_d8()[4] as nat==76nat,
        z_filter_4_ct2_d8()[5] as nat+256*z_filter_4_ct2_d8()[6] as nat==u16::MAX as nat-76nat,
        zm::be_value(z_filter_4_ct2_d8(),0,1)==120nat,
        zm::be_value(z_filter_4_ct2_d8(),1,1)==156nat,
{
    hide(z_filter_4_ct2_d8);hide(raw_filter_4_ct2_d8);
    assert(raw_filter_4_ct2_d8().len()==76nat) by (compute);
    assert(z_filter_4_ct2_d8().len()==87nat) by (compute);
    assert(z_filter_4_ct2_d8()[0]==120u8) by (compute);
    assert(z_filter_4_ct2_d8()[1]==156u8) by (compute);
    assert(zm::field_value(z_filter_4_ct2_d8(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_filter_4_ct2_d8(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_filter_4_ct2_d8(),83nat,4)==2167874831nat) by (compute);
    assert(z_filter_4_ct2_d8().subrange(7,83)==raw_filter_4_ct2_d8()) by (compute);
    assert(z_filter_4_ct2_d8()[3] as nat+256*z_filter_4_ct2_d8()[4] as nat==76nat) by (compute);
    assert(z_filter_4_ct2_d8()[5] as nat+256*z_filter_4_ct2_d8()[6] as nat==u16::MAX as nat-76nat) by (compute);
    assert(zm::be_value(z_filter_4_ct2_d8(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_filter_4_ct2_d8(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_filter_4_ct2_d8()
    ensures zm::Adler(raw_filter_4_ct2_d8())==2167874831nat,
{
    hide(raw_filter_4_ct2_d8);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_filter_4_ct2_d8().len()==76nat) by (compute);
    assert(c::sums(raw_filter_4_ct2_d8(),0,76nat)==(9486nat,360684nat)) by (compute);
    c::sums_correct(raw_filter_4_ct2_d8(),0,76nat);
    assert(raw_filter_4_ct2_d8().subrange(0,76) =~= raw_filter_4_ct2_d8());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((360684nat%65521)*(u16::MAX as nat+1)+(1+9486nat)%65521==2167874831nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_filter_4_ct2_d8()
    ensures r::inflated(z_filter_4_ct2_d8(),a_filter_4_ct2_d8(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_filter_4_ct2_d8(),r2:false}),
        raw_filter_4_ct2_d8().len()==76nat,
{
    hide(z_filter_4_ct2_d8);hide(raw_filter_4_ct2_d8);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_filter_4_ct2_d8();checksum_filter_4_ct2_d8();
    assert(r::raw(a_filter_4_ct2_d8())==76nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_filter_4_ct2_d8(),raw_filter_4_ct2_d8(),76nat,120nat,156nat,2167874831nat);
}
#[verifier::spinoff_prover]
proof fn scan_filter_4_ct2_d8()
    ensures r::scan(raw_filter_4_ct2_d8(),a_filter_4_ct2_d8(),0)==Ok(()),
{
    hide(raw_filter_4_ct2_d8);
    assert(r::base(a_filter_4_ct2_d8(),0)==0nat && r::ph(a_filter_4_ct2_d8(),0)==4nat && r::count(a_filter_4_ct2_d8())==1nat) by (compute);
    assert(r::rb(a_filter_4_ct2_d8(),0)==18nat) by (compute);
    assert(raw_filter_4_ct2_d8()[0]==4u8) by (compute);
    assert(r::row_result(raw_filter_4_ct2_d8(),a_filter_4_ct2_d8(),0,0nat)==Ok(()));
    assert(raw_filter_4_ct2_d8()[19]==4u8) by (compute);
    assert(r::row_result(raw_filter_4_ct2_d8(),a_filter_4_ct2_d8(),0,1nat)==Ok(()));
    assert(raw_filter_4_ct2_d8()[38]==4u8) by (compute);
    assert(r::row_result(raw_filter_4_ct2_d8(),a_filter_4_ct2_d8(),0,2nat)==Ok(()));
    assert(raw_filter_4_ct2_d8()[57]==4u8) by (compute);
    assert(r::row_result(raw_filter_4_ct2_d8(),a_filter_4_ct2_d8(),0,3nat)==Ok(()));
    r::pass_step(raw_filter_4_ct2_d8(),a_filter_4_ct2_d8(),0,4nat);
    r::pass_step(raw_filter_4_ct2_d8(),a_filter_4_ct2_d8(),0,3nat);
    r::pass_step(raw_filter_4_ct2_d8(),a_filter_4_ct2_d8(),0,2nat);
    r::pass_step(raw_filter_4_ct2_d8(),a_filter_4_ct2_d8(),0,1nat);
    r::pass_step(raw_filter_4_ct2_d8(),a_filter_4_ct2_d8(),0,0nat);
    r::scan_step(raw_filter_4_ct2_d8(),a_filter_4_ct2_d8(),1);
    r::scan_step(raw_filter_4_ct2_d8(),a_filter_4_ct2_d8(),0);
}
#[verifier::spinoff_prover]
proof fn model_filter_4_ct2_d8()
    ensures r::still(bytes_filter_4_ct2_d8(),STRICT,false)==(r::Projection::Ready {width:6u32,height:4u32,bytes:96nat}),
{
    hide(bytes_filter_4_ct2_d8);hide(z_filter_4_ct2_d8);hide(raw_filter_4_ct2_d8);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_4_ct2_d8().len()==144nat) by (compute);
    assert(crate::model::mh(bytes_filter_4_ct2_d8())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:2u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_4_ct2_d8())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:2u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_4_ct2_d8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:87nat,next:132nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_4_ct2_d8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:87nat,next:132nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_4_ct2_d8(),132nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:140nat,len:0nat,next:144nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_4_ct2_d8(),132nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:140nat,len:0nat,next:144nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_4_ct2_d8().subrange(41,128)==z_filter_4_ct2_d8()) by (compute);
    walk_filter_4_ct2_d8(bytes_filter_4_ct2_d8());
    inflate_filter_4_ct2_d8();scan_filter_4_ct2_d8();
    assert(r::raw(a_filter_4_ct2_d8())==76nat) by (compute);
    assert(r::data(z_filter_4_ct2_d8(),a_filter_4_ct2_d8(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_filter_4_ct2_d8(),h_filter_4_ct2_d8(),z_filter_4_ct2_d8(),0nat,false);
}
fn public_filter_4_ct2_d8() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,2u8,0u8,0u8,0u8,34u8,102u8,217u8,20u8,0u8,0u8,0u8,87u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,76u8,0u8,179u8,255u8,4u8,101u8,154u8,20u8,240u8,105u8,56u8,197u8,69u8,172u8,125u8,212u8,177u8,19u8,122u8,142u8,34u8,192u8,192u8,4u8,118u8,122u8,212u8,210u8,154u8,11u8,67u8,245u8,245u8,109u8,113u8,4u8,46u8,185u8,47u8,214u8,140u8,164u8,4u8,35u8,225u8,161u8,88u8,48u8,158u8,250u8,201u8,35u8,134u8,165u8,151u8,120u8,229u8,56u8,156u8,22u8,184u8,4u8,103u8,166u8,150u8,102u8,43u8,215u8,205u8,25u8,81u8,75u8,252u8,113u8,112u8,154u8,253u8,0u8,164u8,104u8,129u8,55u8,37u8,15u8,188u8,226u8,23u8,84u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_4_ct2_d8());model_filter_4_ct2_d8();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==6u32 && s.height==4u32 && s.rgba@.len()==96nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// filter/4-ct2-d16; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_filter_4_ct2_d16() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,2u8,0u8,0u8,0u8,114u8,246u8,5u8,87u8,0u8,0u8,0u8,159u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,148u8,0u8,107u8,255u8,4u8,129u8,85u8,103u8,130u8,120u8,193u8,134u8,43u8,66u8,108u8,110u8,29u8,24u8,59u8,41u8,100u8,7u8,165u8,201u8,16u8,216u8,190u8,1u8,119u8,207u8,143u8,221u8,221u8,87u8,220u8,231u8,80u8,176u8,98u8,201u8,236u8,4u8,144u8,246u8,104u8,176u8,80u8,119u8,37u8,195u8,142u8,44u8,210u8,18u8,180u8,11u8,36u8,172u8,108u8,74u8,107u8,150u8,221u8,69u8,110u8,184u8,176u8,167u8,97u8,79u8,46u8,88u8,163u8,123u8,148u8,193u8,3u8,217u8,4u8,238u8,128u8,0u8,219u8,58u8,64u8,168u8,219u8,50u8,87u8,254u8,194u8,2u8,82u8,189u8,110u8,234u8,253u8,189u8,171u8,255u8,64u8,189u8,12u8,117u8,177u8,187u8,164u8,79u8,26u8,143u8,233u8,65u8,26u8,218u8,138u8,4u8,81u8,237u8,174u8,89u8,11u8,109u8,234u8,175u8,181u8,31u8,81u8,176u8,185u8,210u8,113u8,59u8,233u8,213u8,65u8,38u8,245u8,144u8,203u8,170u8,62u8,76u8,64u8,91u8,150u8,66u8,212u8,23u8,124u8,106u8,45u8,92u8,170u8,20u8,72u8,102u8,112u8,106u8,99u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_filter_4_ct2_d16() -> Seq<u8> {seq![120u8,156u8,1u8,148u8,0u8,107u8,255u8,4u8,129u8,85u8,103u8,130u8,120u8,193u8,134u8,43u8,66u8,108u8,110u8,29u8,24u8,59u8,41u8,100u8,7u8,165u8,201u8,16u8,216u8,190u8,1u8,119u8,207u8,143u8,221u8,221u8,87u8,220u8,231u8,80u8,176u8,98u8,201u8,236u8,4u8,144u8,246u8,104u8,176u8,80u8,119u8,37u8,195u8,142u8,44u8,210u8,18u8,180u8,11u8,36u8,172u8,108u8,74u8,107u8,150u8,221u8,69u8,110u8,184u8,176u8,167u8,97u8,79u8,46u8,88u8,163u8,123u8,148u8,193u8,3u8,217u8,4u8,238u8,128u8,0u8,219u8,58u8,64u8,168u8,219u8,50u8,87u8,254u8,194u8,2u8,82u8,189u8,110u8,234u8,253u8,189u8,171u8,255u8,64u8,189u8,12u8,117u8,177u8,187u8,164u8,79u8,26u8,143u8,233u8,65u8,26u8,218u8,138u8,4u8,81u8,237u8,174u8,89u8,11u8,109u8,234u8,175u8,181u8,31u8,81u8,176u8,185u8,210u8,113u8,59u8,233u8,213u8,65u8,38u8,245u8,144u8,203u8,170u8,62u8,76u8,64u8,91u8,150u8,66u8,212u8,23u8,124u8,106u8,45u8,92u8,170u8,20u8,72u8,102u8]}
spec fn h_filter_4_ct2_d16() -> crate::model::Ihdr {crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:2u8,interlace:0u8}}
spec fn a_filter_4_ct2_d16() -> r::Raster {r::shape(h_filter_4_ct2_d16(),6nat,4nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_filter_4_ct2_d16(b: Seq<u8>)
    requires b.len()==216,crate::model::mh(b)==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:2u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:159nat,next:204nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,204nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:212nat,len:0nat,next:216nat,crc_ok:true,synthetic:false})),
        b.subrange(41,200)==z_filter_4_ct2_d16(),
    ensures r::default_case(b,crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:2u8,interlace:0u8},z_filter_4_ct2_d16(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
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
spec fn raw_filter_4_ct2_d16() -> Seq<u8> {seq![4u8,129u8,85u8,103u8,130u8,120u8,193u8,134u8,43u8,66u8,108u8,110u8,29u8,24u8,59u8,41u8,100u8,7u8,165u8,201u8,16u8,216u8,190u8,1u8,119u8,207u8,143u8,221u8,221u8,87u8,220u8,231u8,80u8,176u8,98u8,201u8,236u8,4u8,144u8,246u8,104u8,176u8,80u8,119u8,37u8,195u8,142u8,44u8,210u8,18u8,180u8,11u8,36u8,172u8,108u8,74u8,107u8,150u8,221u8,69u8,110u8,184u8,176u8,167u8,97u8,79u8,46u8,88u8,163u8,123u8,148u8,193u8,3u8,217u8,4u8,238u8,128u8,0u8,219u8,58u8,64u8,168u8,219u8,50u8,87u8,254u8,194u8,2u8,82u8,189u8,110u8,234u8,253u8,189u8,171u8,255u8,64u8,189u8,12u8,117u8,177u8,187u8,164u8,79u8,26u8,143u8,233u8,65u8,26u8,218u8,138u8,4u8,81u8,237u8,174u8,89u8,11u8,109u8,234u8,175u8,181u8,31u8,81u8,176u8,185u8,210u8,113u8,59u8,233u8,213u8,65u8,38u8,245u8,144u8,203u8,170u8,62u8,76u8,64u8,91u8,150u8,66u8,212u8,23u8,124u8,106u8,45u8,92u8]}
#[verifier::spinoff_prover]
proof fn shape_filter_4_ct2_d16()
    ensures raw_filter_4_ct2_d16().len()==148nat,
        z_filter_4_ct2_d16().len()==159nat,
        z_filter_4_ct2_d16()[0]==120u8,
        z_filter_4_ct2_d16()[1]==156u8,
        zm::field_value(z_filter_4_ct2_d16(),16,1)==1nat,
        zm::field_value(z_filter_4_ct2_d16(),17,2)==0nat,
        zm::be_value(z_filter_4_ct2_d16(),155nat,4)==2853455974nat,
        z_filter_4_ct2_d16().subrange(7,155)==raw_filter_4_ct2_d16(),
        z_filter_4_ct2_d16()[3] as nat+256*z_filter_4_ct2_d16()[4] as nat==148nat,
        z_filter_4_ct2_d16()[5] as nat+256*z_filter_4_ct2_d16()[6] as nat==u16::MAX as nat-148nat,
        zm::be_value(z_filter_4_ct2_d16(),0,1)==120nat,
        zm::be_value(z_filter_4_ct2_d16(),1,1)==156nat,
{
    hide(z_filter_4_ct2_d16);hide(raw_filter_4_ct2_d16);
    assert(raw_filter_4_ct2_d16().len()==148nat) by (compute);
    assert(z_filter_4_ct2_d16().len()==159nat) by (compute);
    assert(z_filter_4_ct2_d16()[0]==120u8) by (compute);
    assert(z_filter_4_ct2_d16()[1]==156u8) by (compute);
    assert(zm::field_value(z_filter_4_ct2_d16(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_filter_4_ct2_d16(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_filter_4_ct2_d16(),155nat,4)==2853455974nat) by (compute);
    assert(z_filter_4_ct2_d16().subrange(7,155)==raw_filter_4_ct2_d16()) by (compute);
    assert(z_filter_4_ct2_d16()[3] as nat+256*z_filter_4_ct2_d16()[4] as nat==148nat) by (compute);
    assert(z_filter_4_ct2_d16()[5] as nat+256*z_filter_4_ct2_d16()[6] as nat==u16::MAX as nat-148nat) by (compute);
    assert(zm::be_value(z_filter_4_ct2_d16(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_filter_4_ct2_d16(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_filter_4_ct2_d16()
    ensures zm::Adler(raw_filter_4_ct2_d16())==2853455974nat,
{
    hide(raw_filter_4_ct2_d16);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_filter_4_ct2_d16().len()==148nat) by (compute);
    assert(c::sums(raw_filter_4_ct2_d16(),0,148nat)==(18533nat,1353960nat)) by (compute);
    c::sums_correct(raw_filter_4_ct2_d16(),0,148nat);
    assert(raw_filter_4_ct2_d16().subrange(0,148) =~= raw_filter_4_ct2_d16());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((1353960nat%65521)*(u16::MAX as nat+1)+(1+18533nat)%65521==2853455974nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_filter_4_ct2_d16()
    ensures r::inflated(z_filter_4_ct2_d16(),a_filter_4_ct2_d16(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_filter_4_ct2_d16(),r2:false}),
        raw_filter_4_ct2_d16().len()==148nat,
{
    hide(z_filter_4_ct2_d16);hide(raw_filter_4_ct2_d16);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_filter_4_ct2_d16();checksum_filter_4_ct2_d16();
    assert(r::raw(a_filter_4_ct2_d16())==148nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_filter_4_ct2_d16(),raw_filter_4_ct2_d16(),148nat,120nat,156nat,2853455974nat);
}
#[verifier::spinoff_prover]
proof fn scan_filter_4_ct2_d16()
    ensures r::scan(raw_filter_4_ct2_d16(),a_filter_4_ct2_d16(),0)==Ok(()),
{
    hide(raw_filter_4_ct2_d16);
    assert(r::base(a_filter_4_ct2_d16(),0)==0nat && r::ph(a_filter_4_ct2_d16(),0)==4nat && r::count(a_filter_4_ct2_d16())==1nat) by (compute);
    assert(r::rb(a_filter_4_ct2_d16(),0)==36nat) by (compute);
    assert(raw_filter_4_ct2_d16()[0]==4u8) by (compute);
    assert(r::row_result(raw_filter_4_ct2_d16(),a_filter_4_ct2_d16(),0,0nat)==Ok(()));
    assert(raw_filter_4_ct2_d16()[37]==4u8) by (compute);
    assert(r::row_result(raw_filter_4_ct2_d16(),a_filter_4_ct2_d16(),0,1nat)==Ok(()));
    assert(raw_filter_4_ct2_d16()[74]==4u8) by (compute);
    assert(r::row_result(raw_filter_4_ct2_d16(),a_filter_4_ct2_d16(),0,2nat)==Ok(()));
    assert(raw_filter_4_ct2_d16()[111]==4u8) by (compute);
    assert(r::row_result(raw_filter_4_ct2_d16(),a_filter_4_ct2_d16(),0,3nat)==Ok(()));
    r::pass_step(raw_filter_4_ct2_d16(),a_filter_4_ct2_d16(),0,4nat);
    r::pass_step(raw_filter_4_ct2_d16(),a_filter_4_ct2_d16(),0,3nat);
    r::pass_step(raw_filter_4_ct2_d16(),a_filter_4_ct2_d16(),0,2nat);
    r::pass_step(raw_filter_4_ct2_d16(),a_filter_4_ct2_d16(),0,1nat);
    r::pass_step(raw_filter_4_ct2_d16(),a_filter_4_ct2_d16(),0,0nat);
    r::scan_step(raw_filter_4_ct2_d16(),a_filter_4_ct2_d16(),1);
    r::scan_step(raw_filter_4_ct2_d16(),a_filter_4_ct2_d16(),0);
}
#[verifier::spinoff_prover]
proof fn model_filter_4_ct2_d16()
    ensures r::still(bytes_filter_4_ct2_d16(),STRICT,false)==(r::Projection::Ready {width:6u32,height:4u32,bytes:96nat}),
{
    hide(bytes_filter_4_ct2_d16);hide(z_filter_4_ct2_d16);hide(raw_filter_4_ct2_d16);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_4_ct2_d16().len()==216nat) by (compute);
    assert(crate::model::mh(bytes_filter_4_ct2_d16())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:2u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_4_ct2_d16())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:2u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_4_ct2_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:159nat,next:204nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_4_ct2_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:159nat,next:204nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_4_ct2_d16(),204nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:212nat,len:0nat,next:216nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_4_ct2_d16(),204nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:212nat,len:0nat,next:216nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_4_ct2_d16().subrange(41,200)==z_filter_4_ct2_d16()) by (compute);
    walk_filter_4_ct2_d16(bytes_filter_4_ct2_d16());
    inflate_filter_4_ct2_d16();scan_filter_4_ct2_d16();
    assert(r::raw(a_filter_4_ct2_d16())==148nat) by (compute);
    assert(r::data(z_filter_4_ct2_d16(),a_filter_4_ct2_d16(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_filter_4_ct2_d16(),h_filter_4_ct2_d16(),z_filter_4_ct2_d16(),0nat,false);
}
fn public_filter_4_ct2_d16() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,2u8,0u8,0u8,0u8,114u8,246u8,5u8,87u8,0u8,0u8,0u8,159u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,148u8,0u8,107u8,255u8,4u8,129u8,85u8,103u8,130u8,120u8,193u8,134u8,43u8,66u8,108u8,110u8,29u8,24u8,59u8,41u8,100u8,7u8,165u8,201u8,16u8,216u8,190u8,1u8,119u8,207u8,143u8,221u8,221u8,87u8,220u8,231u8,80u8,176u8,98u8,201u8,236u8,4u8,144u8,246u8,104u8,176u8,80u8,119u8,37u8,195u8,142u8,44u8,210u8,18u8,180u8,11u8,36u8,172u8,108u8,74u8,107u8,150u8,221u8,69u8,110u8,184u8,176u8,167u8,97u8,79u8,46u8,88u8,163u8,123u8,148u8,193u8,3u8,217u8,4u8,238u8,128u8,0u8,219u8,58u8,64u8,168u8,219u8,50u8,87u8,254u8,194u8,2u8,82u8,189u8,110u8,234u8,253u8,189u8,171u8,255u8,64u8,189u8,12u8,117u8,177u8,187u8,164u8,79u8,26u8,143u8,233u8,65u8,26u8,218u8,138u8,4u8,81u8,237u8,174u8,89u8,11u8,109u8,234u8,175u8,181u8,31u8,81u8,176u8,185u8,210u8,113u8,59u8,233u8,213u8,65u8,38u8,245u8,144u8,203u8,170u8,62u8,76u8,64u8,91u8,150u8,66u8,212u8,23u8,124u8,106u8,45u8,92u8,170u8,20u8,72u8,102u8,112u8,106u8,99u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_4_ct2_d16());model_filter_4_ct2_d16();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==6u32 && s.height==4u32 && s.rgba@.len()==96nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// filter/4-ct4-d8; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_filter_4_ct4_d8() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,4u8,0u8,0u8,0u8,7u8,13u8,134u8,200u8,0u8,0u8,0u8,63u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,52u8,0u8,203u8,255u8,4u8,33u8,90u8,24u8,108u8,114u8,92u8,221u8,253u8,204u8,174u8,68u8,169u8,4u8,127u8,196u8,88u8,212u8,17u8,73u8,102u8,81u8,212u8,76u8,249u8,161u8,4u8,197u8,122u8,108u8,143u8,91u8,194u8,189u8,128u8,149u8,34u8,113u8,136u8,4u8,116u8,187u8,52u8,198u8,183u8,106u8,179u8,180u8,183u8,108u8,19u8,0u8,127u8,37u8,24u8,132u8,210u8,72u8,235u8,233u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_filter_4_ct4_d8() -> Seq<u8> {seq![120u8,156u8,1u8,52u8,0u8,203u8,255u8,4u8,33u8,90u8,24u8,108u8,114u8,92u8,221u8,253u8,204u8,174u8,68u8,169u8,4u8,127u8,196u8,88u8,212u8,17u8,73u8,102u8,81u8,212u8,76u8,249u8,161u8,4u8,197u8,122u8,108u8,143u8,91u8,194u8,189u8,128u8,149u8,34u8,113u8,136u8,4u8,116u8,187u8,52u8,198u8,183u8,106u8,179u8,180u8,183u8,108u8,19u8,0u8,127u8,37u8,24u8,132u8]}
spec fn h_filter_4_ct4_d8() -> crate::model::Ihdr {crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:4u8,interlace:0u8}}
spec fn a_filter_4_ct4_d8() -> r::Raster {r::shape(h_filter_4_ct4_d8(),6nat,4nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_filter_4_ct4_d8(b: Seq<u8>)
    requires b.len()==120,crate::model::mh(b)==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:4u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:63nat,next:108nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,108nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:116nat,len:0nat,next:120nat,crc_ok:true,synthetic:false})),
        b.subrange(41,104)==z_filter_4_ct4_d8(),
    ensures r::default_case(b,crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:4u8,interlace:0u8},z_filter_4_ct4_d8(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
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
    reveal(crate::document_model::step);reveal(crate::document_model::ordered);reveal(crate::document_model::own);
    reveal_with_fuel(crate::document_model::drive,3);
    reveal_with_fuel(crate::document_model::concat_ranges,3);
}
spec fn raw_filter_4_ct4_d8() -> Seq<u8> {seq![4u8,33u8,90u8,24u8,108u8,114u8,92u8,221u8,253u8,204u8,174u8,68u8,169u8,4u8,127u8,196u8,88u8,212u8,17u8,73u8,102u8,81u8,212u8,76u8,249u8,161u8,4u8,197u8,122u8,108u8,143u8,91u8,194u8,189u8,128u8,149u8,34u8,113u8,136u8,4u8,116u8,187u8,52u8,198u8,183u8,106u8,179u8,180u8,183u8,108u8,19u8,0u8]}
#[verifier::spinoff_prover]
proof fn shape_filter_4_ct4_d8()
    ensures raw_filter_4_ct4_d8().len()==52nat,
        z_filter_4_ct4_d8().len()==63nat,
        z_filter_4_ct4_d8()[0]==120u8,
        z_filter_4_ct4_d8()[1]==156u8,
        zm::field_value(z_filter_4_ct4_d8(),16,1)==1nat,
        zm::field_value(z_filter_4_ct4_d8(),17,2)==0nat,
        zm::be_value(z_filter_4_ct4_d8(),59nat,4)==2133137540nat,
        z_filter_4_ct4_d8().subrange(7,59)==raw_filter_4_ct4_d8(),
        z_filter_4_ct4_d8()[3] as nat+256*z_filter_4_ct4_d8()[4] as nat==52nat,
        z_filter_4_ct4_d8()[5] as nat+256*z_filter_4_ct4_d8()[6] as nat==u16::MAX as nat-52nat,
        zm::be_value(z_filter_4_ct4_d8(),0,1)==120nat,
        zm::be_value(z_filter_4_ct4_d8(),1,1)==156nat,
{
    hide(z_filter_4_ct4_d8);hide(raw_filter_4_ct4_d8);
    assert(raw_filter_4_ct4_d8().len()==52nat) by (compute);
    assert(z_filter_4_ct4_d8().len()==63nat) by (compute);
    assert(z_filter_4_ct4_d8()[0]==120u8) by (compute);
    assert(z_filter_4_ct4_d8()[1]==156u8) by (compute);
    assert(zm::field_value(z_filter_4_ct4_d8(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_filter_4_ct4_d8(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_filter_4_ct4_d8(),59nat,4)==2133137540nat) by (compute);
    assert(z_filter_4_ct4_d8().subrange(7,59)==raw_filter_4_ct4_d8()) by (compute);
    assert(z_filter_4_ct4_d8()[3] as nat+256*z_filter_4_ct4_d8()[4] as nat==52nat) by (compute);
    assert(z_filter_4_ct4_d8()[5] as nat+256*z_filter_4_ct4_d8()[6] as nat==u16::MAX as nat-52nat) by (compute);
    assert(zm::be_value(z_filter_4_ct4_d8(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_filter_4_ct4_d8(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_filter_4_ct4_d8()
    ensures zm::Adler(raw_filter_4_ct4_d8())==2133137540nat,
{
    hide(raw_filter_4_ct4_d8);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_filter_4_ct4_d8().len()==52nat) by (compute);
    assert(c::sums(raw_filter_4_ct4_d8(),0,52nat)==(6275nat,163591nat)) by (compute);
    c::sums_correct(raw_filter_4_ct4_d8(),0,52nat);
    assert(raw_filter_4_ct4_d8().subrange(0,52) =~= raw_filter_4_ct4_d8());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((163591nat%65521)*(u16::MAX as nat+1)+(1+6275nat)%65521==2133137540nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_filter_4_ct4_d8()
    ensures r::inflated(z_filter_4_ct4_d8(),a_filter_4_ct4_d8(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_filter_4_ct4_d8(),r2:false}),
        raw_filter_4_ct4_d8().len()==52nat,
{
    hide(z_filter_4_ct4_d8);hide(raw_filter_4_ct4_d8);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_filter_4_ct4_d8();checksum_filter_4_ct4_d8();
    assert(r::raw(a_filter_4_ct4_d8())==52nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_filter_4_ct4_d8(),raw_filter_4_ct4_d8(),52nat,120nat,156nat,2133137540nat);
}
#[verifier::spinoff_prover]
proof fn scan_filter_4_ct4_d8()
    ensures r::scan(raw_filter_4_ct4_d8(),a_filter_4_ct4_d8(),0)==Ok(()),
{
    hide(raw_filter_4_ct4_d8);
    assert(r::base(a_filter_4_ct4_d8(),0)==0nat && r::ph(a_filter_4_ct4_d8(),0)==4nat && r::count(a_filter_4_ct4_d8())==1nat) by (compute);
    assert(r::rb(a_filter_4_ct4_d8(),0)==12nat) by (compute);
    assert(raw_filter_4_ct4_d8()[0]==4u8) by (compute);
    assert(r::row_result(raw_filter_4_ct4_d8(),a_filter_4_ct4_d8(),0,0nat)==Ok(()));
    assert(raw_filter_4_ct4_d8()[13]==4u8) by (compute);
    assert(r::row_result(raw_filter_4_ct4_d8(),a_filter_4_ct4_d8(),0,1nat)==Ok(()));
    assert(raw_filter_4_ct4_d8()[26]==4u8) by (compute);
    assert(r::row_result(raw_filter_4_ct4_d8(),a_filter_4_ct4_d8(),0,2nat)==Ok(()));
    assert(raw_filter_4_ct4_d8()[39]==4u8) by (compute);
    assert(r::row_result(raw_filter_4_ct4_d8(),a_filter_4_ct4_d8(),0,3nat)==Ok(()));
    r::pass_step(raw_filter_4_ct4_d8(),a_filter_4_ct4_d8(),0,4nat);
    r::pass_step(raw_filter_4_ct4_d8(),a_filter_4_ct4_d8(),0,3nat);
    r::pass_step(raw_filter_4_ct4_d8(),a_filter_4_ct4_d8(),0,2nat);
    r::pass_step(raw_filter_4_ct4_d8(),a_filter_4_ct4_d8(),0,1nat);
    r::pass_step(raw_filter_4_ct4_d8(),a_filter_4_ct4_d8(),0,0nat);
    r::scan_step(raw_filter_4_ct4_d8(),a_filter_4_ct4_d8(),1);
    r::scan_step(raw_filter_4_ct4_d8(),a_filter_4_ct4_d8(),0);
}
#[verifier::spinoff_prover]
proof fn model_filter_4_ct4_d8()
    ensures r::still(bytes_filter_4_ct4_d8(),STRICT,false)==(r::Projection::Ready {width:6u32,height:4u32,bytes:96nat}),
{
    hide(bytes_filter_4_ct4_d8);hide(z_filter_4_ct4_d8);hide(raw_filter_4_ct4_d8);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_4_ct4_d8().len()==120nat) by (compute);
    assert(crate::model::mh(bytes_filter_4_ct4_d8())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:4u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_4_ct4_d8())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:4u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_4_ct4_d8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:63nat,next:108nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_4_ct4_d8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:63nat,next:108nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_4_ct4_d8(),108nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:116nat,len:0nat,next:120nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_4_ct4_d8(),108nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:116nat,len:0nat,next:120nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_4_ct4_d8().subrange(41,104)==z_filter_4_ct4_d8()) by (compute);
    walk_filter_4_ct4_d8(bytes_filter_4_ct4_d8());
    inflate_filter_4_ct4_d8();scan_filter_4_ct4_d8();
    assert(r::raw(a_filter_4_ct4_d8())==52nat) by (compute);
    assert(r::data(z_filter_4_ct4_d8(),a_filter_4_ct4_d8(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_filter_4_ct4_d8(),h_filter_4_ct4_d8(),z_filter_4_ct4_d8(),0nat,false);
}
fn public_filter_4_ct4_d8() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,8u8,4u8,0u8,0u8,0u8,7u8,13u8,134u8,200u8,0u8,0u8,0u8,63u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,52u8,0u8,203u8,255u8,4u8,33u8,90u8,24u8,108u8,114u8,92u8,221u8,253u8,204u8,174u8,68u8,169u8,4u8,127u8,196u8,88u8,212u8,17u8,73u8,102u8,81u8,212u8,76u8,249u8,161u8,4u8,197u8,122u8,108u8,143u8,91u8,194u8,189u8,128u8,149u8,34u8,113u8,136u8,4u8,116u8,187u8,52u8,198u8,183u8,106u8,179u8,180u8,183u8,108u8,19u8,0u8,127u8,37u8,24u8,132u8,210u8,72u8,235u8,233u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_4_ct4_d8());model_filter_4_ct4_d8();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==6u32 && s.height==4u32 && s.rgba@.len()==96nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// filter/4-ct6-d16; frozen sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d; authored M2 intent sha256 62b590a2b88f4f80c38dac6c003e21028c73679e93fa2ee53909593b778ed8fb
spec fn bytes_filter_4_ct6_d16() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,6u8,0u8,0u8,0u8,253u8,148u8,146u8,0u8,0u8,0u8,0u8,207u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,196u8,0u8,59u8,255u8,4u8,52u8,83u8,111u8,111u8,23u8,132u8,165u8,198u8,184u8,250u8,230u8,127u8,175u8,6u8,115u8,58u8,105u8,114u8,174u8,54u8,226u8,223u8,35u8,222u8,173u8,252u8,215u8,42u8,164u8,80u8,65u8,25u8,116u8,175u8,210u8,88u8,202u8,173u8,207u8,115u8,47u8,179u8,159u8,14u8,87u8,1u8,26u8,82u8,4u8,41u8,192u8,208u8,93u8,97u8,8u8,14u8,192u8,138u8,131u8,206u8,93u8,19u8,38u8,211u8,200u8,17u8,26u8,80u8,71u8,249u8,11u8,211u8,180u8,95u8,226u8,31u8,11u8,243u8,226u8,173u8,234u8,5u8,79u8,1u8,76u8,51u8,93u8,184u8,60u8,59u8,238u8,53u8,213u8,44u8,134u8,21u8,104u8,4u8,202u8,183u8,105u8,236u8,240u8,122u8,75u8,123u8,30u8,3u8,145u8,159u8,173u8,1u8,67u8,157u8,15u8,170u8,56u8,123u8,143u8,151u8,246u8,91u8,13u8,52u8,118u8,10u8,40u8,137u8,234u8,245u8,46u8,59u8,221u8,98u8,211u8,152u8,7u8,96u8,87u8,206u8,106u8,244u8,20u8,177u8,192u8,57u8,4u8,147u8,155u8,28u8,13u8,194u8,168u8,134u8,143u8,28u8,233u8,154u8,12u8,2u8,73u8,231u8,145u8,190u8,240u8,58u8,106u8,0u8,24u8,9u8,231u8,143u8,170u8,113u8,56u8,19u8,165u8,232u8,159u8,149u8,159u8,92u8,121u8,161u8,101u8,12u8,251u8,29u8,15u8,66u8,172u8,31u8,5u8,8u8,47u8,193u8,11u8,87u8,233u8,29u8,229u8,5u8,110u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn z_filter_4_ct6_d16() -> Seq<u8> {seq![120u8,156u8,1u8,196u8,0u8,59u8,255u8,4u8,52u8,83u8,111u8,111u8,23u8,132u8,165u8,198u8,184u8,250u8,230u8,127u8,175u8,6u8,115u8,58u8,105u8,114u8,174u8,54u8,226u8,223u8,35u8,222u8,173u8,252u8,215u8,42u8,164u8,80u8,65u8,25u8,116u8,175u8,210u8,88u8,202u8,173u8,207u8,115u8,47u8,179u8,159u8,14u8,87u8,1u8,26u8,82u8,4u8,41u8,192u8,208u8,93u8,97u8,8u8,14u8,192u8,138u8,131u8,206u8,93u8,19u8,38u8,211u8,200u8,17u8,26u8,80u8,71u8,249u8,11u8,211u8,180u8,95u8,226u8,31u8,11u8,243u8,226u8,173u8,234u8,5u8,79u8,1u8,76u8,51u8,93u8,184u8,60u8,59u8,238u8,53u8,213u8,44u8,134u8,21u8,104u8,4u8,202u8,183u8,105u8,236u8,240u8,122u8,75u8,123u8,30u8,3u8,145u8,159u8,173u8,1u8,67u8,157u8,15u8,170u8,56u8,123u8,143u8,151u8,246u8,91u8,13u8,52u8,118u8,10u8,40u8,137u8,234u8,245u8,46u8,59u8,221u8,98u8,211u8,152u8,7u8,96u8,87u8,206u8,106u8,244u8,20u8,177u8,192u8,57u8,4u8,147u8,155u8,28u8,13u8,194u8,168u8,134u8,143u8,28u8,233u8,154u8,12u8,2u8,73u8,231u8,145u8,190u8,240u8,58u8,106u8,0u8,24u8,9u8,231u8,143u8,170u8,113u8,56u8,19u8,165u8,232u8,159u8,149u8,159u8,92u8,121u8,161u8,101u8,12u8,251u8,29u8,15u8,66u8,172u8,31u8,5u8,8u8,47u8,193u8,11u8,87u8,233u8]}
spec fn h_filter_4_ct6_d16() -> crate::model::Ihdr {crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:6u8,interlace:0u8}}
spec fn a_filter_4_ct6_d16() -> r::Raster {r::shape(h_filter_4_ct6_d16(),6nat,4nat,0nat,STRICT)}
#[verifier::spinoff_prover]
proof fn walk_filter_4_ct6_d16(b: Seq<u8>)
    requires b.len()==264,crate::model::mh(b)==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:6u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:207nat,next:252nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,252nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:260nat,len:0nat,next:264nat,crc_ok:true,synthetic:false})),
        b.subrange(41,248)==z_filter_4_ct6_d16(),
    ensures r::default_case(b,crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:6u8,interlace:0u8},z_filter_4_ct6_d16(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
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
spec fn raw_filter_4_ct6_d16() -> Seq<u8> {seq![4u8,52u8,83u8,111u8,111u8,23u8,132u8,165u8,198u8,184u8,250u8,230u8,127u8,175u8,6u8,115u8,58u8,105u8,114u8,174u8,54u8,226u8,223u8,35u8,222u8,173u8,252u8,215u8,42u8,164u8,80u8,65u8,25u8,116u8,175u8,210u8,88u8,202u8,173u8,207u8,115u8,47u8,179u8,159u8,14u8,87u8,1u8,26u8,82u8,4u8,41u8,192u8,208u8,93u8,97u8,8u8,14u8,192u8,138u8,131u8,206u8,93u8,19u8,38u8,211u8,200u8,17u8,26u8,80u8,71u8,249u8,11u8,211u8,180u8,95u8,226u8,31u8,11u8,243u8,226u8,173u8,234u8,5u8,79u8,1u8,76u8,51u8,93u8,184u8,60u8,59u8,238u8,53u8,213u8,44u8,134u8,21u8,104u8,4u8,202u8,183u8,105u8,236u8,240u8,122u8,75u8,123u8,30u8,3u8,145u8,159u8,173u8,1u8,67u8,157u8,15u8,170u8,56u8,123u8,143u8,151u8,246u8,91u8,13u8,52u8,118u8,10u8,40u8,137u8,234u8,245u8,46u8,59u8,221u8,98u8,211u8,152u8,7u8,96u8,87u8,206u8,106u8,244u8,20u8,177u8,192u8,57u8,4u8,147u8,155u8,28u8,13u8,194u8,168u8,134u8,143u8,28u8,233u8,154u8,12u8,2u8,73u8,231u8,145u8,190u8,240u8,58u8,106u8,0u8,24u8,9u8,231u8,143u8,170u8,113u8,56u8,19u8,165u8,232u8,159u8,149u8,159u8,92u8,121u8,161u8,101u8,12u8,251u8,29u8,15u8,66u8,172u8,31u8,5u8,8u8,47u8]}
#[verifier::spinoff_prover]
proof fn shape_filter_4_ct6_d16()
    ensures raw_filter_4_ct6_d16().len()==196nat,
        z_filter_4_ct6_d16().len()==207nat,
        z_filter_4_ct6_d16()[0]==120u8,
        z_filter_4_ct6_d16()[1]==156u8,
        zm::field_value(z_filter_4_ct6_d16(),16,1)==1nat,
        zm::field_value(z_filter_4_ct6_d16(),17,2)==0nat,
        zm::be_value(z_filter_4_ct6_d16(),203nat,4)==3238746089nat,
        z_filter_4_ct6_d16().subrange(7,203)==raw_filter_4_ct6_d16(),
        z_filter_4_ct6_d16()[3] as nat+256*z_filter_4_ct6_d16()[4] as nat==196nat,
        z_filter_4_ct6_d16()[5] as nat+256*z_filter_4_ct6_d16()[6] as nat==u16::MAX as nat-196nat,
        zm::be_value(z_filter_4_ct6_d16(),0,1)==120nat,
        zm::be_value(z_filter_4_ct6_d16(),1,1)==156nat,
{
    hide(z_filter_4_ct6_d16);hide(raw_filter_4_ct6_d16);
    assert(raw_filter_4_ct6_d16().len()==196nat) by (compute);
    assert(z_filter_4_ct6_d16().len()==207nat) by (compute);
    assert(z_filter_4_ct6_d16()[0]==120u8) by (compute);
    assert(z_filter_4_ct6_d16()[1]==156u8) by (compute);
    assert(zm::field_value(z_filter_4_ct6_d16(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_filter_4_ct6_d16(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_filter_4_ct6_d16(),203nat,4)==3238746089nat) by (compute);
    assert(z_filter_4_ct6_d16().subrange(7,203)==raw_filter_4_ct6_d16()) by (compute);
    assert(z_filter_4_ct6_d16()[3] as nat+256*z_filter_4_ct6_d16()[4] as nat==196nat) by (compute);
    assert(z_filter_4_ct6_d16()[5] as nat+256*z_filter_4_ct6_d16()[6] as nat==u16::MAX as nat-196nat) by (compute);
    assert(zm::be_value(z_filter_4_ct6_d16(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_filter_4_ct6_d16(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_filter_4_ct6_d16()
    ensures zm::Adler(raw_filter_4_ct6_d16())==3238746089nat,
{
    hide(raw_filter_4_ct6_d16);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_filter_4_ct6_d16().len()==196nat) by (compute);
    assert(c::sums(raw_filter_4_ct6_d16(),0,196nat)==(22504nat,2277133nat)) by (compute);
    c::sums_correct(raw_filter_4_ct6_d16(),0,196nat);
    assert(raw_filter_4_ct6_d16().subrange(0,196) =~= raw_filter_4_ct6_d16());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);
    assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((2277133nat%65521)*(u16::MAX as nat+1)+(1+22504nat)%65521==3238746089nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_filter_4_ct6_d16()
    ensures r::inflated(z_filter_4_ct6_d16(),a_filter_4_ct6_d16(),STRICT)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_filter_4_ct6_d16(),r2:false}),
        raw_filter_4_ct6_d16().len()==196nat,
{
    hide(z_filter_4_ct6_d16);hide(raw_filter_4_ct6_d16);
    hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_filter_4_ct6_d16();checksum_filter_4_ct6_d16();
    assert(r::raw(a_filter_4_ct6_d16())==196nat) by (compute);
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_filter_4_ct6_d16(),raw_filter_4_ct6_d16(),196nat,120nat,156nat,3238746089nat);
}
#[verifier::spinoff_prover]
proof fn scan_filter_4_ct6_d16()
    ensures r::scan(raw_filter_4_ct6_d16(),a_filter_4_ct6_d16(),0)==Ok(()),
{
    hide(raw_filter_4_ct6_d16);
    assert(r::base(a_filter_4_ct6_d16(),0)==0nat && r::ph(a_filter_4_ct6_d16(),0)==4nat && r::count(a_filter_4_ct6_d16())==1nat) by (compute);
    assert(r::rb(a_filter_4_ct6_d16(),0)==48nat) by (compute);
    assert(raw_filter_4_ct6_d16()[0]==4u8) by (compute);
    assert(r::row_result(raw_filter_4_ct6_d16(),a_filter_4_ct6_d16(),0,0nat)==Ok(()));
    assert(raw_filter_4_ct6_d16()[49]==4u8) by (compute);
    assert(r::row_result(raw_filter_4_ct6_d16(),a_filter_4_ct6_d16(),0,1nat)==Ok(()));
    assert(raw_filter_4_ct6_d16()[98]==4u8) by (compute);
    assert(r::row_result(raw_filter_4_ct6_d16(),a_filter_4_ct6_d16(),0,2nat)==Ok(()));
    assert(raw_filter_4_ct6_d16()[147]==4u8) by (compute);
    assert(r::row_result(raw_filter_4_ct6_d16(),a_filter_4_ct6_d16(),0,3nat)==Ok(()));
    r::pass_step(raw_filter_4_ct6_d16(),a_filter_4_ct6_d16(),0,4nat);
    r::pass_step(raw_filter_4_ct6_d16(),a_filter_4_ct6_d16(),0,3nat);
    r::pass_step(raw_filter_4_ct6_d16(),a_filter_4_ct6_d16(),0,2nat);
    r::pass_step(raw_filter_4_ct6_d16(),a_filter_4_ct6_d16(),0,1nat);
    r::pass_step(raw_filter_4_ct6_d16(),a_filter_4_ct6_d16(),0,0nat);
    r::scan_step(raw_filter_4_ct6_d16(),a_filter_4_ct6_d16(),1);
    r::scan_step(raw_filter_4_ct6_d16(),a_filter_4_ct6_d16(),0);
}
#[verifier::spinoff_prover]
proof fn model_filter_4_ct6_d16()
    ensures r::still(bytes_filter_4_ct6_d16(),STRICT,false)==(r::Projection::Ready {width:6u32,height:4u32,bytes:96nat}),
{
    hide(bytes_filter_4_ct6_d16);hide(z_filter_4_ct6_d16);hide(raw_filter_4_ct6_d16);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_4_ct6_d16().len()==264nat) by (compute);
    assert(crate::model::mh(bytes_filter_4_ct6_d16())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:6u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_4_ct6_d16())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:6u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_4_ct6_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:207nat,next:252nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_4_ct6_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:207nat,next:252nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_4_ct6_d16(),252nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:260nat,len:0nat,next:264nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_4_ct6_d16(),252nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:260nat,len:0nat,next:264nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_4_ct6_d16().subrange(41,248)==z_filter_4_ct6_d16()) by (compute);
    walk_filter_4_ct6_d16(bytes_filter_4_ct6_d16());
    inflate_filter_4_ct6_d16();scan_filter_4_ct6_d16();
    assert(r::raw(a_filter_4_ct6_d16())==196nat) by (compute);
    assert(r::data(z_filter_4_ct6_d16(),a_filter_4_ct6_d16(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    r::default_ready(bytes_filter_4_ct6_d16(),h_filter_4_ct6_d16(),z_filter_4_ct6_d16(),0nat,false);
}
fn public_filter_4_ct6_d16() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,6u8,0u8,0u8,0u8,4u8,16u8,6u8,0u8,0u8,0u8,253u8,148u8,146u8,0u8,0u8,0u8,0u8,207u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,196u8,0u8,59u8,255u8,4u8,52u8,83u8,111u8,111u8,23u8,132u8,165u8,198u8,184u8,250u8,230u8,127u8,175u8,6u8,115u8,58u8,105u8,114u8,174u8,54u8,226u8,223u8,35u8,222u8,173u8,252u8,215u8,42u8,164u8,80u8,65u8,25u8,116u8,175u8,210u8,88u8,202u8,173u8,207u8,115u8,47u8,179u8,159u8,14u8,87u8,1u8,26u8,82u8,4u8,41u8,192u8,208u8,93u8,97u8,8u8,14u8,192u8,138u8,131u8,206u8,93u8,19u8,38u8,211u8,200u8,17u8,26u8,80u8,71u8,249u8,11u8,211u8,180u8,95u8,226u8,31u8,11u8,243u8,226u8,173u8,234u8,5u8,79u8,1u8,76u8,51u8,93u8,184u8,60u8,59u8,238u8,53u8,213u8,44u8,134u8,21u8,104u8,4u8,202u8,183u8,105u8,236u8,240u8,122u8,75u8,123u8,30u8,3u8,145u8,159u8,173u8,1u8,67u8,157u8,15u8,170u8,56u8,123u8,143u8,151u8,246u8,91u8,13u8,52u8,118u8,10u8,40u8,137u8,234u8,245u8,46u8,59u8,221u8,98u8,211u8,152u8,7u8,96u8,87u8,206u8,106u8,244u8,20u8,177u8,192u8,57u8,4u8,147u8,155u8,28u8,13u8,194u8,168u8,134u8,143u8,28u8,233u8,154u8,12u8,2u8,73u8,231u8,145u8,190u8,240u8,58u8,106u8,0u8,24u8,9u8,231u8,143u8,170u8,113u8,56u8,19u8,165u8,232u8,159u8,149u8,159u8,92u8,121u8,161u8,101u8,12u8,251u8,29u8,15u8,66u8,172u8,31u8,5u8,8u8,47u8,193u8,11u8,87u8,233u8,29u8,229u8,5u8,110u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_filter_4_ct6_d16());model_filter_4_ct6_d16();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==6u32 && s.height==4u32 && s.rgba@.len()==96nat,
        Err(crate::alloc_guard::sites::TryError::OutOfMemory(_))=>true,_=>false});
}
// E6-M4-FILT-C1 proof bridges; original M4 certificates above are unchanged.

spec fn e6_colour_filter_0_ct0_d1() -> crate::colour_model::Context {(crate::colour_model::Context {ct:0u8,depth:1u8,plte:Seq::empty(),trns:None})}
spec fn e6_line_filter_0_ct0_d1_0() -> Seq<u8> {seq![12u8]}
spec fn e6_line_filter_0_ct0_d1_1() -> Seq<u8> {seq![64u8]}
spec fn e6_line_filter_0_ct0_d1_2() -> Seq<u8> {seq![152u8]}
spec fn e6_line_filter_0_ct0_d1_3() -> Seq<u8> {seq![156u8]}
#[verifier::spinoff_prover]
proof fn e6_rows_filter_0_ct0_d1()
    ensures forall|y: nat| y<4 ==> #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),r::line(raw_filter_0_ct0_d1(),a_filter_0_ct0_d1(),0,y),6nat)==crate::witness_m6_m4_filter::expected_filter_0_ct0_d1().subrange((24*y) as int,(24*(y+1)) as int),
{
    hide(raw_filter_0_ct0_d1);hide(crate::colour_model::pixel);hide(crate::witness_m6_m4_filter::expected_filter_0_ct0_d1);
    crate::colour_model::depth_values(1u8);
    assert(r::base(a_filter_0_ct0_d1(),0)==0nat && r::rb(a_filter_0_ct0_d1(),0)==1nat && crate::filter_model::bpp(r::bits(a_filter_0_ct0_d1()))==1nat) by (compute);
    crate::filter_model::pass_row_equation(raw_filter_0_ct0_d1(),0nat,1nat,1nat,0nat);
    assert(raw_filter_0_ct0_d1()[0]==0u8) by (compute);
    assert(crate::filter_model::row_data(raw_filter_0_ct0_d1(),0,1nat,0nat)==seq![12u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_filter_0_ct0_d1(),0,1nat,1nat,0nat)==None);
    assert(crate::filter_model::encoded_row(0u8,seq![12u8],None,1nat,seq![12u8],0,1nat)) by (compute);
    crate::filter_model::checked_row_hint(0u8,seq![12u8],None,1nat,seq![12u8]);
    assert(r::line(raw_filter_0_ct0_d1(),a_filter_0_ct0_d1(),0,0nat)==seq![12u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_0(),6nat)[0]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_0(),6nat)[1]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_0(),6nat)[2]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_0(),6nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_0(),6nat)[4]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_0(),6nat)[5]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_0(),6nat)[6]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_0(),6nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_0(),6nat)[8]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_0(),6nat)[9]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_0(),6nat)[10]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_0(),6nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_0(),6nat)[12]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_0(),6nat)[13]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_0(),6nat)[14]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_0(),6nat)[15]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_0(),6nat)[16]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_0(),6nat)[17]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_0(),6nat)[18]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_0(),6nat)[19]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_0(),6nat)[20]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_0(),6nat)[21]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_0(),6nat)[22]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_0(),6nat)[23]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_0(),6nat),seq![0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8]);
    assert(crate::witness_m6_m4_filter::expected_filter_0_ct0_d1().subrange(0,24)==seq![0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_filter_0_ct0_d1(),0nat,1nat,1nat,1nat);
    assert(raw_filter_0_ct0_d1()[2]==0u8) by (compute);
    assert(crate::filter_model::row_data(raw_filter_0_ct0_d1(),0,1nat,1nat)==seq![64u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_filter_0_ct0_d1(),0,1nat,1nat,1nat)==Some(seq![12u8]));
    assert(crate::filter_model::encoded_row(0u8,seq![64u8],Some(seq![12u8]),1nat,seq![64u8],0,1nat)) by (compute);
    crate::filter_model::checked_row_hint(0u8,seq![64u8],Some(seq![12u8]),1nat,seq![64u8]);
    assert(r::line(raw_filter_0_ct0_d1(),a_filter_0_ct0_d1(),0,1nat)==seq![64u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_1(),6nat)[0]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_1(),6nat)[1]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_1(),6nat)[2]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_1(),6nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_1(),6nat)[4]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_1(),6nat)[5]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_1(),6nat)[6]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_1(),6nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_1(),6nat)[8]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_1(),6nat)[9]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_1(),6nat)[10]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_1(),6nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_1(),6nat)[12]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_1(),6nat)[13]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_1(),6nat)[14]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_1(),6nat)[15]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_1(),6nat)[16]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_1(),6nat)[17]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_1(),6nat)[18]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_1(),6nat)[19]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_1(),6nat)[20]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_1(),6nat)[21]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_1(),6nat)[22]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_1(),6nat)[23]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_1(),6nat),seq![0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8]);
    assert(crate::witness_m6_m4_filter::expected_filter_0_ct0_d1().subrange(24,48)==seq![0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_filter_0_ct0_d1(),0nat,1nat,1nat,2nat);
    assert(raw_filter_0_ct0_d1()[4]==0u8) by (compute);
    assert(crate::filter_model::row_data(raw_filter_0_ct0_d1(),0,1nat,2nat)==seq![152u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_filter_0_ct0_d1(),0,1nat,1nat,2nat)==Some(seq![64u8]));
    assert(crate::filter_model::encoded_row(0u8,seq![152u8],Some(seq![64u8]),1nat,seq![152u8],0,1nat)) by (compute);
    crate::filter_model::checked_row_hint(0u8,seq![152u8],Some(seq![64u8]),1nat,seq![152u8]);
    assert(r::line(raw_filter_0_ct0_d1(),a_filter_0_ct0_d1(),0,2nat)==seq![152u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_2(),6nat)[0]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_2(),6nat)[1]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_2(),6nat)[2]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_2(),6nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_2(),6nat)[4]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_2(),6nat)[5]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_2(),6nat)[6]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_2(),6nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_2(),6nat)[8]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_2(),6nat)[9]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_2(),6nat)[10]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_2(),6nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_2(),6nat)[12]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_2(),6nat)[13]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_2(),6nat)[14]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_2(),6nat)[15]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_2(),6nat)[16]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_2(),6nat)[17]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_2(),6nat)[18]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_2(),6nat)[19]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_2(),6nat)[20]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_2(),6nat)[21]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_2(),6nat)[22]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_2(),6nat)[23]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_2(),6nat),seq![255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8]);
    assert(crate::witness_m6_m4_filter::expected_filter_0_ct0_d1().subrange(48,72)==seq![255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_filter_0_ct0_d1(),0nat,1nat,1nat,3nat);
    assert(raw_filter_0_ct0_d1()[6]==0u8) by (compute);
    assert(crate::filter_model::row_data(raw_filter_0_ct0_d1(),0,1nat,3nat)==seq![156u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_filter_0_ct0_d1(),0,1nat,1nat,3nat)==Some(seq![152u8]));
    assert(crate::filter_model::encoded_row(0u8,seq![156u8],Some(seq![152u8]),1nat,seq![156u8],0,1nat)) by (compute);
    crate::filter_model::checked_row_hint(0u8,seq![156u8],Some(seq![152u8]),1nat,seq![156u8]);
    assert(r::line(raw_filter_0_ct0_d1(),a_filter_0_ct0_d1(),0,3nat)==seq![156u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_3(),6nat)[0]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_3(),6nat)[1]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_3(),6nat)[2]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_3(),6nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_3(),6nat)[4]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_3(),6nat)[5]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_3(),6nat)[6]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_3(),6nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_3(),6nat)[8]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_3(),6nat)[9]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_3(),6nat)[10]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_3(),6nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_3(),6nat)[12]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_3(),6nat)[13]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_3(),6nat)[14]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_3(),6nat)[15]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_3(),6nat)[16]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_3(),6nat)[17]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_3(),6nat)[18]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_3(),6nat)[19]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_3(),6nat)[20]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_3(),6nat)[21]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_3(),6nat)[22]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_3(),6nat)[23]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),e6_line_filter_0_ct0_d1_3(),6nat),seq![255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8]);
    assert(crate::witness_m6_m4_filter::expected_filter_0_ct0_d1().subrange(72,96)==seq![255u8,255u8,255u8,255u8,0u8,0u8,0u8,255u8,0u8,0u8,0u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8,255u8]) by (compute);
    assert forall|y: nat| y<4 implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_0_ct0_d1(),r::line(raw_filter_0_ct0_d1(),a_filter_0_ct0_d1(),0,y),6nat)==crate::witness_m6_m4_filter::expected_filter_0_ct0_d1().subrange((24*y) as int,(24*(y+1)) as int) by {
        if y==0 {} else if y==1 {} else if y==2 {} else {assert(y==3);}
    }
}
#[verifier::spinoff_prover]
pub(crate) proof fn e6_complete_filter_0_ct0_d1()
    ensures match crate::still_model::MS(crate::witness_m6_m4_filter::bytes_filter_0_ct0_d1()) {crate::still_model::Outcome::Done(value)=>crate::witness_m6_m4_filter::expected_value_filter_0_ct0_d1(value),_=>false},
{

    hide(bytes_filter_0_ct0_d1);hide(z_filter_0_ct0_d1);hide(raw_filter_0_ct0_d1);hide(r::data);hide(crate::witness_m6_m4_filter::expected_filter_0_ct0_d1);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_0_ct0_d1().len()==73nat) by (compute);
    assert(crate::model::mh(bytes_filter_0_ct0_d1())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:1u8,color_type:0u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_0_ct0_d1())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:1u8,color_type:0u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 1u8)==2u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_0_ct0_d1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:16nat,next:61nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_0_ct0_d1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:16nat,next:61nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_0_ct0_d1(),61nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:69nat,len:0nat,next:73nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_0_ct0_d1(),61nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:69nat,len:0nat,next:73nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_0_ct0_d1().subrange(41,57)==z_filter_0_ct0_d1()) by (compute);
    walk_filter_0_ct0_d1(bytes_filter_0_ct0_d1());

    model_filter_0_ct0_d1();
    inflate_filter_0_ct0_d1();scan_filter_0_ct0_d1();
    assert(bytes_filter_0_ct0_d1()==crate::witness_m6_m4_filter::bytes_filter_0_ct0_d1()) by (compute);
    assert(r::raw(a_filter_0_ct0_d1())==8nat) by (compute);
    assert(r::data(z_filter_0_ct0_d1(),a_filter_0_ct0_d1(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    assert(crate::model::valid_depth(0u8,1u8)) by (compute);
    crate::witness_m6_rows_model::bare_colour(bytes_filter_0_ct0_d1(),h_filter_0_ct0_d1(),16nat);
    e6_rows_filter_0_ct0_d1();
    assert(crate::witness_m6_m4_filter::expected_filter_0_ct0_d1().len()==96nat) by (compute);
    crate::witness_m6_rows_model::complete_still(bytes_filter_0_ct0_d1(),h_filter_0_ct0_d1(),z_filter_0_ct0_d1(),raw_filter_0_ct0_d1(),e6_colour_filter_0_ct0_d1(),crate::witness_m6_m4_filter::expected_filter_0_ct0_d1());
}
spec fn e6_colour_filter_1_ct0_d8() -> crate::colour_model::Context {(crate::colour_model::Context {ct:0u8,depth:8u8,plte:Seq::empty(),trns:None})}
spec fn e6_line_filter_1_ct0_d8_0() -> Seq<u8> {seq![92u8,65u8,182u8,143u8,147u8,236u8]}
spec fn e6_line_filter_1_ct0_d8_1() -> Seq<u8> {seq![99u8,125u8,157u8,100u8,54u8,105u8]}
spec fn e6_line_filter_1_ct0_d8_2() -> Seq<u8> {seq![100u8,17u8,255u8,212u8,88u8,34u8]}
spec fn e6_line_filter_1_ct0_d8_3() -> Seq<u8> {seq![154u8,71u8,95u8,124u8,77u8,122u8]}
#[verifier::spinoff_prover]
proof fn e6_rows_filter_1_ct0_d8()
    ensures forall|y: nat| y<4 ==> #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),r::line(raw_filter_1_ct0_d8(),a_filter_1_ct0_d8(),0,y),6nat)==crate::witness_m6_m4_filter::expected_filter_1_ct0_d8().subrange((24*y) as int,(24*(y+1)) as int),
{
    hide(raw_filter_1_ct0_d8);hide(crate::colour_model::pixel);hide(crate::witness_m6_m4_filter::expected_filter_1_ct0_d8);
    crate::colour_model::depth_values(8u8);
    assert(r::base(a_filter_1_ct0_d8(),0)==0nat && r::rb(a_filter_1_ct0_d8(),0)==6nat && crate::filter_model::bpp(r::bits(a_filter_1_ct0_d8()))==1nat) by (compute);
    crate::filter_model::pass_row_equation(raw_filter_1_ct0_d8(),0nat,6nat,1nat,0nat);
    assert(raw_filter_1_ct0_d8()[0]==1u8) by (compute);
    assert(crate::filter_model::row_data(raw_filter_1_ct0_d8(),0,6nat,0nat)==seq![92u8,229u8,117u8,217u8,4u8,89u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_filter_1_ct0_d8(),0,6nat,1nat,0nat)==None);
    assert(crate::filter_model::encoded_row(1u8,seq![92u8,229u8,117u8,217u8,4u8,89u8],None,1nat,seq![92u8,65u8,182u8,143u8,147u8,236u8],0,6nat)) by (compute);
    crate::filter_model::checked_row_hint(1u8,seq![92u8,229u8,117u8,217u8,4u8,89u8],None,1nat,seq![92u8,65u8,182u8,143u8,147u8,236u8]);
    assert(r::line(raw_filter_1_ct0_d8(),a_filter_1_ct0_d8(),0,0nat)==seq![92u8,65u8,182u8,143u8,147u8,236u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_0(),6nat)[0]==92u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_0(),6nat)[1]==92u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_0(),6nat)[2]==92u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_0(),6nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_0(),6nat)[4]==65u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_0(),6nat)[5]==65u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_0(),6nat)[6]==65u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_0(),6nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_0(),6nat)[8]==182u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_0(),6nat)[9]==182u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_0(),6nat)[10]==182u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_0(),6nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_0(),6nat)[12]==143u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_0(),6nat)[13]==143u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_0(),6nat)[14]==143u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_0(),6nat)[15]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_0(),6nat)[16]==147u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_0(),6nat)[17]==147u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_0(),6nat)[18]==147u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_0(),6nat)[19]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_0(),6nat)[20]==236u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_0(),6nat)[21]==236u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_0(),6nat)[22]==236u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_0(),6nat)[23]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_0(),6nat),seq![92u8,92u8,92u8,255u8,65u8,65u8,65u8,255u8,182u8,182u8,182u8,255u8,143u8,143u8,143u8,255u8,147u8,147u8,147u8,255u8,236u8,236u8,236u8,255u8]);
    assert(crate::witness_m6_m4_filter::expected_filter_1_ct0_d8().subrange(0,24)==seq![92u8,92u8,92u8,255u8,65u8,65u8,65u8,255u8,182u8,182u8,182u8,255u8,143u8,143u8,143u8,255u8,147u8,147u8,147u8,255u8,236u8,236u8,236u8,255u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_filter_1_ct0_d8(),0nat,6nat,1nat,1nat);
    assert(raw_filter_1_ct0_d8()[7]==1u8) by (compute);
    assert(crate::filter_model::row_data(raw_filter_1_ct0_d8(),0,6nat,1nat)==seq![99u8,26u8,32u8,199u8,210u8,51u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_filter_1_ct0_d8(),0,6nat,1nat,1nat)==Some(seq![92u8,65u8,182u8,143u8,147u8,236u8]));
    assert(crate::filter_model::encoded_row(1u8,seq![99u8,26u8,32u8,199u8,210u8,51u8],Some(seq![92u8,65u8,182u8,143u8,147u8,236u8]),1nat,seq![99u8,125u8,157u8,100u8,54u8,105u8],0,6nat)) by (compute);
    crate::filter_model::checked_row_hint(1u8,seq![99u8,26u8,32u8,199u8,210u8,51u8],Some(seq![92u8,65u8,182u8,143u8,147u8,236u8]),1nat,seq![99u8,125u8,157u8,100u8,54u8,105u8]);
    assert(r::line(raw_filter_1_ct0_d8(),a_filter_1_ct0_d8(),0,1nat)==seq![99u8,125u8,157u8,100u8,54u8,105u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_1(),6nat)[0]==99u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_1(),6nat)[1]==99u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_1(),6nat)[2]==99u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_1(),6nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_1(),6nat)[4]==125u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_1(),6nat)[5]==125u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_1(),6nat)[6]==125u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_1(),6nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_1(),6nat)[8]==157u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_1(),6nat)[9]==157u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_1(),6nat)[10]==157u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_1(),6nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_1(),6nat)[12]==100u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_1(),6nat)[13]==100u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_1(),6nat)[14]==100u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_1(),6nat)[15]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_1(),6nat)[16]==54u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_1(),6nat)[17]==54u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_1(),6nat)[18]==54u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_1(),6nat)[19]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_1(),6nat)[20]==105u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_1(),6nat)[21]==105u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_1(),6nat)[22]==105u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_1(),6nat)[23]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_1(),6nat),seq![99u8,99u8,99u8,255u8,125u8,125u8,125u8,255u8,157u8,157u8,157u8,255u8,100u8,100u8,100u8,255u8,54u8,54u8,54u8,255u8,105u8,105u8,105u8,255u8]);
    assert(crate::witness_m6_m4_filter::expected_filter_1_ct0_d8().subrange(24,48)==seq![99u8,99u8,99u8,255u8,125u8,125u8,125u8,255u8,157u8,157u8,157u8,255u8,100u8,100u8,100u8,255u8,54u8,54u8,54u8,255u8,105u8,105u8,105u8,255u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_filter_1_ct0_d8(),0nat,6nat,1nat,2nat);
    assert(raw_filter_1_ct0_d8()[14]==1u8) by (compute);
    assert(crate::filter_model::row_data(raw_filter_1_ct0_d8(),0,6nat,2nat)==seq![100u8,173u8,238u8,213u8,132u8,202u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_filter_1_ct0_d8(),0,6nat,1nat,2nat)==Some(seq![99u8,125u8,157u8,100u8,54u8,105u8]));
    assert(crate::filter_model::encoded_row(1u8,seq![100u8,173u8,238u8,213u8,132u8,202u8],Some(seq![99u8,125u8,157u8,100u8,54u8,105u8]),1nat,seq![100u8,17u8,255u8,212u8,88u8,34u8],0,6nat)) by (compute);
    crate::filter_model::checked_row_hint(1u8,seq![100u8,173u8,238u8,213u8,132u8,202u8],Some(seq![99u8,125u8,157u8,100u8,54u8,105u8]),1nat,seq![100u8,17u8,255u8,212u8,88u8,34u8]);
    assert(r::line(raw_filter_1_ct0_d8(),a_filter_1_ct0_d8(),0,2nat)==seq![100u8,17u8,255u8,212u8,88u8,34u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_2(),6nat)[0]==100u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_2(),6nat)[1]==100u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_2(),6nat)[2]==100u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_2(),6nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_2(),6nat)[4]==17u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_2(),6nat)[5]==17u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_2(),6nat)[6]==17u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_2(),6nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_2(),6nat)[8]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_2(),6nat)[9]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_2(),6nat)[10]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_2(),6nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_2(),6nat)[12]==212u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_2(),6nat)[13]==212u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_2(),6nat)[14]==212u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_2(),6nat)[15]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_2(),6nat)[16]==88u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_2(),6nat)[17]==88u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_2(),6nat)[18]==88u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_2(),6nat)[19]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_2(),6nat)[20]==34u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_2(),6nat)[21]==34u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_2(),6nat)[22]==34u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_2(),6nat)[23]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_2(),6nat),seq![100u8,100u8,100u8,255u8,17u8,17u8,17u8,255u8,255u8,255u8,255u8,255u8,212u8,212u8,212u8,255u8,88u8,88u8,88u8,255u8,34u8,34u8,34u8,255u8]);
    assert(crate::witness_m6_m4_filter::expected_filter_1_ct0_d8().subrange(48,72)==seq![100u8,100u8,100u8,255u8,17u8,17u8,17u8,255u8,255u8,255u8,255u8,255u8,212u8,212u8,212u8,255u8,88u8,88u8,88u8,255u8,34u8,34u8,34u8,255u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_filter_1_ct0_d8(),0nat,6nat,1nat,3nat);
    assert(raw_filter_1_ct0_d8()[21]==1u8) by (compute);
    assert(crate::filter_model::row_data(raw_filter_1_ct0_d8(),0,6nat,3nat)==seq![154u8,173u8,24u8,29u8,209u8,45u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_filter_1_ct0_d8(),0,6nat,1nat,3nat)==Some(seq![100u8,17u8,255u8,212u8,88u8,34u8]));
    assert(crate::filter_model::encoded_row(1u8,seq![154u8,173u8,24u8,29u8,209u8,45u8],Some(seq![100u8,17u8,255u8,212u8,88u8,34u8]),1nat,seq![154u8,71u8,95u8,124u8,77u8,122u8],0,6nat)) by (compute);
    crate::filter_model::checked_row_hint(1u8,seq![154u8,173u8,24u8,29u8,209u8,45u8],Some(seq![100u8,17u8,255u8,212u8,88u8,34u8]),1nat,seq![154u8,71u8,95u8,124u8,77u8,122u8]);
    assert(r::line(raw_filter_1_ct0_d8(),a_filter_1_ct0_d8(),0,3nat)==seq![154u8,71u8,95u8,124u8,77u8,122u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_3(),6nat)[0]==154u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_3(),6nat)[1]==154u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_3(),6nat)[2]==154u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_3(),6nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_3(),6nat)[4]==71u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_3(),6nat)[5]==71u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_3(),6nat)[6]==71u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_3(),6nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_3(),6nat)[8]==95u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_3(),6nat)[9]==95u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_3(),6nat)[10]==95u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_3(),6nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_3(),6nat)[12]==124u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_3(),6nat)[13]==124u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_3(),6nat)[14]==124u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_3(),6nat)[15]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_3(),6nat)[16]==77u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_3(),6nat)[17]==77u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_3(),6nat)[18]==77u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_3(),6nat)[19]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_3(),6nat)[20]==122u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_3(),6nat)[21]==122u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_3(),6nat)[22]==122u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_3(),6nat)[23]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),e6_line_filter_1_ct0_d8_3(),6nat),seq![154u8,154u8,154u8,255u8,71u8,71u8,71u8,255u8,95u8,95u8,95u8,255u8,124u8,124u8,124u8,255u8,77u8,77u8,77u8,255u8,122u8,122u8,122u8,255u8]);
    assert(crate::witness_m6_m4_filter::expected_filter_1_ct0_d8().subrange(72,96)==seq![154u8,154u8,154u8,255u8,71u8,71u8,71u8,255u8,95u8,95u8,95u8,255u8,124u8,124u8,124u8,255u8,77u8,77u8,77u8,255u8,122u8,122u8,122u8,255u8]) by (compute);
    assert forall|y: nat| y<4 implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct0_d8(),r::line(raw_filter_1_ct0_d8(),a_filter_1_ct0_d8(),0,y),6nat)==crate::witness_m6_m4_filter::expected_filter_1_ct0_d8().subrange((24*y) as int,(24*(y+1)) as int) by {
        if y==0 {} else if y==1 {} else if y==2 {} else {assert(y==3);}
    }
}
#[verifier::spinoff_prover]
pub(crate) proof fn e6_complete_filter_1_ct0_d8()
    ensures match crate::still_model::MS(crate::witness_m6_m4_filter::bytes_filter_1_ct0_d8()) {crate::still_model::Outcome::Done(value)=>crate::witness_m6_m4_filter::expected_value_filter_1_ct0_d8(value),_=>false},
{

    hide(bytes_filter_1_ct0_d8);hide(z_filter_1_ct0_d8);hide(raw_filter_1_ct0_d8);hide(r::data);hide(crate::witness_m6_m4_filter::expected_filter_1_ct0_d8);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_1_ct0_d8().len()==94nat) by (compute);
    assert(crate::model::mh(bytes_filter_1_ct0_d8())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:0u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_1_ct0_d8())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:0u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_1_ct0_d8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:37nat,next:82nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_1_ct0_d8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:37nat,next:82nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_1_ct0_d8(),82nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:90nat,len:0nat,next:94nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_1_ct0_d8(),82nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:90nat,len:0nat,next:94nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_1_ct0_d8().subrange(41,78)==z_filter_1_ct0_d8()) by (compute);
    walk_filter_1_ct0_d8(bytes_filter_1_ct0_d8());

    model_filter_1_ct0_d8();
    inflate_filter_1_ct0_d8();scan_filter_1_ct0_d8();
    assert(bytes_filter_1_ct0_d8()==crate::witness_m6_m4_filter::bytes_filter_1_ct0_d8()) by (compute);
    assert(r::raw(a_filter_1_ct0_d8())==28nat) by (compute);
    assert(r::data(z_filter_1_ct0_d8(),a_filter_1_ct0_d8(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    assert(crate::model::valid_depth(0u8,8u8)) by (compute);
    crate::witness_m6_rows_model::bare_colour(bytes_filter_1_ct0_d8(),h_filter_1_ct0_d8(),37nat);
    e6_rows_filter_1_ct0_d8();
    assert(crate::witness_m6_m4_filter::expected_filter_1_ct0_d8().len()==96nat) by (compute);
    crate::witness_m6_rows_model::complete_still(bytes_filter_1_ct0_d8(),h_filter_1_ct0_d8(),z_filter_1_ct0_d8(),raw_filter_1_ct0_d8(),e6_colour_filter_1_ct0_d8(),crate::witness_m6_m4_filter::expected_filter_1_ct0_d8());
}
spec fn e6_colour_filter_2_ct0_d16() -> crate::colour_model::Context {(crate::colour_model::Context {ct:0u8,depth:16u8,plte:Seq::empty(),trns:None})}
spec fn e6_line_filter_2_ct0_d16_0() -> Seq<u8> {seq![172u8,60u8,145u8,40u8,212u8,8u8,109u8,77u8,105u8,246u8,141u8,65u8]}
spec fn e6_line_filter_2_ct0_d16_1() -> Seq<u8> {seq![165u8,225u8,244u8,107u8,41u8,157u8,127u8,148u8,8u8,198u8,74u8,47u8]}
spec fn e6_line_filter_2_ct0_d16_2() -> Seq<u8> {seq![68u8,52u8,166u8,215u8,100u8,233u8,133u8,203u8,98u8,246u8,163u8,166u8]}
spec fn e6_line_filter_2_ct0_d16_3() -> Seq<u8> {seq![153u8,132u8,49u8,240u8,190u8,236u8,68u8,48u8,168u8,36u8,222u8,116u8]}
#[verifier::spinoff_prover]
proof fn e6_rows_filter_2_ct0_d16()
    ensures forall|y: nat| y<4 ==> #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),r::line(raw_filter_2_ct0_d16(),a_filter_2_ct0_d16(),0,y),6nat)==crate::witness_m6_m4_filter::expected_filter_2_ct0_d16().subrange((24*y) as int,(24*(y+1)) as int),
{
    hide(raw_filter_2_ct0_d16);hide(crate::colour_model::pixel);hide(crate::witness_m6_m4_filter::expected_filter_2_ct0_d16);
    crate::colour_model::depth_values(16u8);
    assert(r::base(a_filter_2_ct0_d16(),0)==0nat && r::rb(a_filter_2_ct0_d16(),0)==12nat && crate::filter_model::bpp(r::bits(a_filter_2_ct0_d16()))==2nat) by (compute);
    crate::filter_model::pass_row_equation(raw_filter_2_ct0_d16(),0nat,12nat,2nat,0nat);
    assert(raw_filter_2_ct0_d16()[0]==2u8) by (compute);
    assert(crate::filter_model::row_data(raw_filter_2_ct0_d16(),0,12nat,0nat)==seq![172u8,60u8,145u8,40u8,212u8,8u8,109u8,77u8,105u8,246u8,141u8,65u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_filter_2_ct0_d16(),0,12nat,2nat,0nat)==None);
    assert(crate::filter_model::encoded_row(2u8,seq![172u8,60u8,145u8,40u8,212u8,8u8,109u8,77u8,105u8,246u8,141u8,65u8],None,2nat,seq![172u8,60u8,145u8,40u8,212u8,8u8,109u8,77u8,105u8,246u8,141u8,65u8],0,12nat)) by (compute);
    crate::filter_model::checked_row_hint(2u8,seq![172u8,60u8,145u8,40u8,212u8,8u8,109u8,77u8,105u8,246u8,141u8,65u8],None,2nat,seq![172u8,60u8,145u8,40u8,212u8,8u8,109u8,77u8,105u8,246u8,141u8,65u8]);
    assert(r::line(raw_filter_2_ct0_d16(),a_filter_2_ct0_d16(),0,0nat)==seq![172u8,60u8,145u8,40u8,212u8,8u8,109u8,77u8,105u8,246u8,141u8,65u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_0(),6nat)[0]==172u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_0(),6nat)[1]==172u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_0(),6nat)[2]==172u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_0(),6nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_0(),6nat)[4]==145u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_0(),6nat)[5]==145u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_0(),6nat)[6]==145u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_0(),6nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_0(),6nat)[8]==211u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_0(),6nat)[9]==211u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_0(),6nat)[10]==211u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_0(),6nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_0(),6nat)[12]==109u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_0(),6nat)[13]==109u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_0(),6nat)[14]==109u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_0(),6nat)[15]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_0(),6nat)[16]==106u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_0(),6nat)[17]==106u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_0(),6nat)[18]==106u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_0(),6nat)[19]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_0(),6nat)[20]==141u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_0(),6nat)[21]==141u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_0(),6nat)[22]==141u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_0(),6nat)[23]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_0(),6nat),seq![172u8,172u8,172u8,255u8,145u8,145u8,145u8,255u8,211u8,211u8,211u8,255u8,109u8,109u8,109u8,255u8,106u8,106u8,106u8,255u8,141u8,141u8,141u8,255u8]);
    assert(crate::witness_m6_m4_filter::expected_filter_2_ct0_d16().subrange(0,24)==seq![172u8,172u8,172u8,255u8,145u8,145u8,145u8,255u8,211u8,211u8,211u8,255u8,109u8,109u8,109u8,255u8,106u8,106u8,106u8,255u8,141u8,141u8,141u8,255u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_filter_2_ct0_d16(),0nat,12nat,2nat,1nat);
    assert(raw_filter_2_ct0_d16()[13]==2u8) by (compute);
    assert(crate::filter_model::row_data(raw_filter_2_ct0_d16(),0,12nat,1nat)==seq![249u8,165u8,99u8,67u8,85u8,149u8,18u8,71u8,159u8,208u8,189u8,238u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_filter_2_ct0_d16(),0,12nat,2nat,1nat)==Some(seq![172u8,60u8,145u8,40u8,212u8,8u8,109u8,77u8,105u8,246u8,141u8,65u8]));
    assert(crate::filter_model::encoded_row(2u8,seq![249u8,165u8,99u8,67u8,85u8,149u8,18u8,71u8,159u8,208u8,189u8,238u8],Some(seq![172u8,60u8,145u8,40u8,212u8,8u8,109u8,77u8,105u8,246u8,141u8,65u8]),2nat,seq![165u8,225u8,244u8,107u8,41u8,157u8,127u8,148u8,8u8,198u8,74u8,47u8],0,12nat)) by (compute);
    crate::filter_model::checked_row_hint(2u8,seq![249u8,165u8,99u8,67u8,85u8,149u8,18u8,71u8,159u8,208u8,189u8,238u8],Some(seq![172u8,60u8,145u8,40u8,212u8,8u8,109u8,77u8,105u8,246u8,141u8,65u8]),2nat,seq![165u8,225u8,244u8,107u8,41u8,157u8,127u8,148u8,8u8,198u8,74u8,47u8]);
    assert(r::line(raw_filter_2_ct0_d16(),a_filter_2_ct0_d16(),0,1nat)==seq![165u8,225u8,244u8,107u8,41u8,157u8,127u8,148u8,8u8,198u8,74u8,47u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_1(),6nat)[0]==165u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_1(),6nat)[1]==165u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_1(),6nat)[2]==165u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_1(),6nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_1(),6nat)[4]==243u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_1(),6nat)[5]==243u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_1(),6nat)[6]==243u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_1(),6nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_1(),6nat)[8]==41u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_1(),6nat)[9]==41u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_1(),6nat)[10]==41u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_1(),6nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_1(),6nat)[12]==127u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_1(),6nat)[13]==127u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_1(),6nat)[14]==127u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_1(),6nat)[15]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_1(),6nat)[16]==9u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_1(),6nat)[17]==9u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_1(),6nat)[18]==9u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_1(),6nat)[19]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_1(),6nat)[20]==74u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_1(),6nat)[21]==74u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_1(),6nat)[22]==74u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_1(),6nat)[23]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_1(),6nat),seq![165u8,165u8,165u8,255u8,243u8,243u8,243u8,255u8,41u8,41u8,41u8,255u8,127u8,127u8,127u8,255u8,9u8,9u8,9u8,255u8,74u8,74u8,74u8,255u8]);
    assert(crate::witness_m6_m4_filter::expected_filter_2_ct0_d16().subrange(24,48)==seq![165u8,165u8,165u8,255u8,243u8,243u8,243u8,255u8,41u8,41u8,41u8,255u8,127u8,127u8,127u8,255u8,9u8,9u8,9u8,255u8,74u8,74u8,74u8,255u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_filter_2_ct0_d16(),0nat,12nat,2nat,2nat);
    assert(raw_filter_2_ct0_d16()[26]==2u8) by (compute);
    assert(crate::filter_model::row_data(raw_filter_2_ct0_d16(),0,12nat,2nat)==seq![159u8,83u8,178u8,108u8,59u8,76u8,6u8,55u8,90u8,48u8,89u8,119u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_filter_2_ct0_d16(),0,12nat,2nat,2nat)==Some(seq![165u8,225u8,244u8,107u8,41u8,157u8,127u8,148u8,8u8,198u8,74u8,47u8]));
    assert(crate::filter_model::encoded_row(2u8,seq![159u8,83u8,178u8,108u8,59u8,76u8,6u8,55u8,90u8,48u8,89u8,119u8],Some(seq![165u8,225u8,244u8,107u8,41u8,157u8,127u8,148u8,8u8,198u8,74u8,47u8]),2nat,seq![68u8,52u8,166u8,215u8,100u8,233u8,133u8,203u8,98u8,246u8,163u8,166u8],0,12nat)) by (compute);
    crate::filter_model::checked_row_hint(2u8,seq![159u8,83u8,178u8,108u8,59u8,76u8,6u8,55u8,90u8,48u8,89u8,119u8],Some(seq![165u8,225u8,244u8,107u8,41u8,157u8,127u8,148u8,8u8,198u8,74u8,47u8]),2nat,seq![68u8,52u8,166u8,215u8,100u8,233u8,133u8,203u8,98u8,246u8,163u8,166u8]);
    assert(r::line(raw_filter_2_ct0_d16(),a_filter_2_ct0_d16(),0,2nat)==seq![68u8,52u8,166u8,215u8,100u8,233u8,133u8,203u8,98u8,246u8,163u8,166u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_2(),6nat)[0]==68u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_2(),6nat)[1]==68u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_2(),6nat)[2]==68u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_2(),6nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_2(),6nat)[4]==166u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_2(),6nat)[5]==166u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_2(),6nat)[6]==166u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_2(),6nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_2(),6nat)[8]==101u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_2(),6nat)[9]==101u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_2(),6nat)[10]==101u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_2(),6nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_2(),6nat)[12]==133u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_2(),6nat)[13]==133u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_2(),6nat)[14]==133u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_2(),6nat)[15]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_2(),6nat)[16]==99u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_2(),6nat)[17]==99u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_2(),6nat)[18]==99u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_2(),6nat)[19]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_2(),6nat)[20]==163u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_2(),6nat)[21]==163u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_2(),6nat)[22]==163u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_2(),6nat)[23]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_2(),6nat),seq![68u8,68u8,68u8,255u8,166u8,166u8,166u8,255u8,101u8,101u8,101u8,255u8,133u8,133u8,133u8,255u8,99u8,99u8,99u8,255u8,163u8,163u8,163u8,255u8]);
    assert(crate::witness_m6_m4_filter::expected_filter_2_ct0_d16().subrange(48,72)==seq![68u8,68u8,68u8,255u8,166u8,166u8,166u8,255u8,101u8,101u8,101u8,255u8,133u8,133u8,133u8,255u8,99u8,99u8,99u8,255u8,163u8,163u8,163u8,255u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_filter_2_ct0_d16(),0nat,12nat,2nat,3nat);
    assert(raw_filter_2_ct0_d16()[39]==2u8) by (compute);
    assert(crate::filter_model::row_data(raw_filter_2_ct0_d16(),0,12nat,3nat)==seq![85u8,80u8,139u8,25u8,90u8,3u8,191u8,101u8,70u8,46u8,59u8,206u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_filter_2_ct0_d16(),0,12nat,2nat,3nat)==Some(seq![68u8,52u8,166u8,215u8,100u8,233u8,133u8,203u8,98u8,246u8,163u8,166u8]));
    assert(crate::filter_model::encoded_row(2u8,seq![85u8,80u8,139u8,25u8,90u8,3u8,191u8,101u8,70u8,46u8,59u8,206u8],Some(seq![68u8,52u8,166u8,215u8,100u8,233u8,133u8,203u8,98u8,246u8,163u8,166u8]),2nat,seq![153u8,132u8,49u8,240u8,190u8,236u8,68u8,48u8,168u8,36u8,222u8,116u8],0,12nat)) by (compute);
    crate::filter_model::checked_row_hint(2u8,seq![85u8,80u8,139u8,25u8,90u8,3u8,191u8,101u8,70u8,46u8,59u8,206u8],Some(seq![68u8,52u8,166u8,215u8,100u8,233u8,133u8,203u8,98u8,246u8,163u8,166u8]),2nat,seq![153u8,132u8,49u8,240u8,190u8,236u8,68u8,48u8,168u8,36u8,222u8,116u8]);
    assert(r::line(raw_filter_2_ct0_d16(),a_filter_2_ct0_d16(),0,3nat)==seq![153u8,132u8,49u8,240u8,190u8,236u8,68u8,48u8,168u8,36u8,222u8,116u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_3(),6nat)[0]==153u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_3(),6nat)[1]==153u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_3(),6nat)[2]==153u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_3(),6nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_3(),6nat)[4]==50u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_3(),6nat)[5]==50u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_3(),6nat)[6]==50u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_3(),6nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_3(),6nat)[8]==190u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_3(),6nat)[9]==190u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_3(),6nat)[10]==190u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_3(),6nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_3(),6nat)[12]==68u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_3(),6nat)[13]==68u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_3(),6nat)[14]==68u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_3(),6nat)[15]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_3(),6nat)[16]==167u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_3(),6nat)[17]==167u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_3(),6nat)[18]==167u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_3(),6nat)[19]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_3(),6nat)[20]==222u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_3(),6nat)[21]==222u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_3(),6nat)[22]==222u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_3(),6nat)[23]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),e6_line_filter_2_ct0_d16_3(),6nat),seq![153u8,153u8,153u8,255u8,50u8,50u8,50u8,255u8,190u8,190u8,190u8,255u8,68u8,68u8,68u8,255u8,167u8,167u8,167u8,255u8,222u8,222u8,222u8,255u8]);
    assert(crate::witness_m6_m4_filter::expected_filter_2_ct0_d16().subrange(72,96)==seq![153u8,153u8,153u8,255u8,50u8,50u8,50u8,255u8,190u8,190u8,190u8,255u8,68u8,68u8,68u8,255u8,167u8,167u8,167u8,255u8,222u8,222u8,222u8,255u8]) by (compute);
    assert forall|y: nat| y<4 implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_2_ct0_d16(),r::line(raw_filter_2_ct0_d16(),a_filter_2_ct0_d16(),0,y),6nat)==crate::witness_m6_m4_filter::expected_filter_2_ct0_d16().subrange((24*y) as int,(24*(y+1)) as int) by {
        if y==0 {} else if y==1 {} else if y==2 {} else {assert(y==3);}
    }
}
#[verifier::spinoff_prover]
pub(crate) proof fn e6_complete_filter_2_ct0_d16()
    ensures match crate::still_model::MS(crate::witness_m6_m4_filter::bytes_filter_2_ct0_d16()) {crate::still_model::Outcome::Done(value)=>crate::witness_m6_m4_filter::expected_value_filter_2_ct0_d16(value),_=>false},
{

    hide(bytes_filter_2_ct0_d16);hide(z_filter_2_ct0_d16);hide(raw_filter_2_ct0_d16);hide(r::data);hide(crate::witness_m6_m4_filter::expected_filter_2_ct0_d16);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_2_ct0_d16().len()==120nat) by (compute);
    assert(crate::model::mh(bytes_filter_2_ct0_d16())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:0u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_2_ct0_d16())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:0u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_2_ct0_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:63nat,next:108nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_2_ct0_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:63nat,next:108nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_2_ct0_d16(),108nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:116nat,len:0nat,next:120nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_2_ct0_d16(),108nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:116nat,len:0nat,next:120nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_2_ct0_d16().subrange(41,104)==z_filter_2_ct0_d16()) by (compute);
    walk_filter_2_ct0_d16(bytes_filter_2_ct0_d16());

    model_filter_2_ct0_d16();
    inflate_filter_2_ct0_d16();scan_filter_2_ct0_d16();
    assert(bytes_filter_2_ct0_d16()==crate::witness_m6_m4_filter::bytes_filter_2_ct0_d16()) by (compute);
    assert(r::raw(a_filter_2_ct0_d16())==52nat) by (compute);
    assert(r::data(z_filter_2_ct0_d16(),a_filter_2_ct0_d16(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    assert(crate::model::valid_depth(0u8,16u8)) by (compute);
    crate::witness_m6_rows_model::bare_colour(bytes_filter_2_ct0_d16(),h_filter_2_ct0_d16(),63nat);
    e6_rows_filter_2_ct0_d16();
    assert(crate::witness_m6_m4_filter::expected_filter_2_ct0_d16().len()==96nat) by (compute);
    crate::witness_m6_rows_model::complete_still(bytes_filter_2_ct0_d16(),h_filter_2_ct0_d16(),z_filter_2_ct0_d16(),raw_filter_2_ct0_d16(),e6_colour_filter_2_ct0_d16(),crate::witness_m6_m4_filter::expected_filter_2_ct0_d16());
}
spec fn e6_colour_filter_3_ct2_d8() -> crate::colour_model::Context {(crate::colour_model::Context {ct:2u8,depth:8u8,plte:Seq::empty(),trns:None})}
spec fn e6_line_filter_3_ct2_d8_0() -> Seq<u8> {seq![50u8,39u8,159u8,191u8,222u8,75u8,196u8,57u8,125u8,26u8,110u8,17u8,199u8,229u8,2u8,184u8,138u8,103u8]}
spec fn e6_line_filter_3_ct2_d8_1() -> Seq<u8> {seq![219u8,121u8,205u8,166u8,83u8,60u8,10u8,43u8,17u8,85u8,183u8,95u8,10u8,178u8,135u8,21u8,77u8,78u8]}
spec fn e6_line_filter_3_ct2_d8_2() -> Seq<u8> {seq![80u8,31u8,102u8,92u8,106u8,158u8,42u8,3u8,78u8,195u8,210u8,22u8,73u8,4u8,91u8,93u8,137u8,140u8]}
spec fn e6_line_filter_3_ct2_d8_3() -> Seq<u8> {seq![174u8,82u8,138u8,44u8,255u8,34u8,13u8,216u8,229u8,49u8,205u8,144u8,76u8,216u8,168u8,176u8,241u8,156u8]}
#[verifier::spinoff_prover]
proof fn e6_rows_filter_3_ct2_d8()
    ensures forall|y: nat| y<4 ==> #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),r::line(raw_filter_3_ct2_d8(),a_filter_3_ct2_d8(),0,y),6nat)==crate::witness_m6_m4_filter::expected_filter_3_ct2_d8().subrange((24*y) as int,(24*(y+1)) as int),
{
    hide(raw_filter_3_ct2_d8);hide(crate::colour_model::pixel);hide(crate::witness_m6_m4_filter::expected_filter_3_ct2_d8);
    crate::colour_model::depth_values(8u8);
    assert(r::base(a_filter_3_ct2_d8(),0)==0nat && r::rb(a_filter_3_ct2_d8(),0)==18nat && crate::filter_model::bpp(r::bits(a_filter_3_ct2_d8()))==3nat) by (compute);
    crate::filter_model::pass_row_equation(raw_filter_3_ct2_d8(),0nat,18nat,3nat,0nat);
    assert(raw_filter_3_ct2_d8()[0]==3u8) by (compute);
    assert(crate::filter_model::row_data(raw_filter_3_ct2_d8(),0,18nat,0nat)==seq![50u8,39u8,159u8,166u8,203u8,252u8,101u8,202u8,88u8,184u8,82u8,211u8,186u8,174u8,250u8,85u8,24u8,102u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_filter_3_ct2_d8(),0,18nat,3nat,0nat)==None);
    assert(crate::filter_model::encoded_row(3u8,seq![50u8,39u8,159u8,166u8,203u8,252u8,101u8,202u8,88u8,184u8,82u8,211u8,186u8,174u8,250u8,85u8,24u8,102u8],None,3nat,seq![50u8,39u8,159u8,191u8,222u8,75u8,196u8,57u8,125u8,26u8,110u8,17u8,199u8,229u8,2u8,184u8,138u8,103u8],0,18nat)) by (compute);
    crate::filter_model::checked_row_hint(3u8,seq![50u8,39u8,159u8,166u8,203u8,252u8,101u8,202u8,88u8,184u8,82u8,211u8,186u8,174u8,250u8,85u8,24u8,102u8],None,3nat,seq![50u8,39u8,159u8,191u8,222u8,75u8,196u8,57u8,125u8,26u8,110u8,17u8,199u8,229u8,2u8,184u8,138u8,103u8]);
    assert(r::line(raw_filter_3_ct2_d8(),a_filter_3_ct2_d8(),0,0nat)==seq![50u8,39u8,159u8,191u8,222u8,75u8,196u8,57u8,125u8,26u8,110u8,17u8,199u8,229u8,2u8,184u8,138u8,103u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_0(),6nat)[0]==50u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_0(),6nat)[1]==39u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_0(),6nat)[2]==159u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_0(),6nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_0(),6nat)[4]==191u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_0(),6nat)[5]==222u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_0(),6nat)[6]==75u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_0(),6nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_0(),6nat)[8]==196u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_0(),6nat)[9]==57u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_0(),6nat)[10]==125u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_0(),6nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_0(),6nat)[12]==26u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_0(),6nat)[13]==110u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_0(),6nat)[14]==17u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_0(),6nat)[15]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_0(),6nat)[16]==199u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_0(),6nat)[17]==229u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_0(),6nat)[18]==2u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_0(),6nat)[19]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_0(),6nat)[20]==184u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_0(),6nat)[21]==138u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_0(),6nat)[22]==103u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_0(),6nat)[23]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_0(),6nat),seq![50u8,39u8,159u8,255u8,191u8,222u8,75u8,255u8,196u8,57u8,125u8,255u8,26u8,110u8,17u8,255u8,199u8,229u8,2u8,255u8,184u8,138u8,103u8,255u8]);
    assert(crate::witness_m6_m4_filter::expected_filter_3_ct2_d8().subrange(0,24)==seq![50u8,39u8,159u8,255u8,191u8,222u8,75u8,255u8,196u8,57u8,125u8,255u8,26u8,110u8,17u8,255u8,199u8,229u8,2u8,255u8,184u8,138u8,103u8,255u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_filter_3_ct2_d8(),0nat,18nat,3nat,1nat);
    assert(raw_filter_3_ct2_d8()[19]==3u8) by (compute);
    assert(crate::filter_model::row_data(raw_filter_3_ct2_d8(),0,18nat,1nat)==seq![194u8,102u8,126u8,217u8,168u8,176u8,85u8,229u8,181u8,67u8,107u8,78u8,124u8,228u8,87u8,180u8,175u8,215u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_filter_3_ct2_d8(),0,18nat,3nat,1nat)==Some(seq![50u8,39u8,159u8,191u8,222u8,75u8,196u8,57u8,125u8,26u8,110u8,17u8,199u8,229u8,2u8,184u8,138u8,103u8]));
    assert(crate::filter_model::encoded_row(3u8,seq![194u8,102u8,126u8,217u8,168u8,176u8,85u8,229u8,181u8,67u8,107u8,78u8,124u8,228u8,87u8,180u8,175u8,215u8],Some(seq![50u8,39u8,159u8,191u8,222u8,75u8,196u8,57u8,125u8,26u8,110u8,17u8,199u8,229u8,2u8,184u8,138u8,103u8]),3nat,seq![219u8,121u8,205u8,166u8,83u8,60u8,10u8,43u8,17u8,85u8,183u8,95u8,10u8,178u8,135u8,21u8,77u8,78u8],0,18nat)) by (compute);
    crate::filter_model::checked_row_hint(3u8,seq![194u8,102u8,126u8,217u8,168u8,176u8,85u8,229u8,181u8,67u8,107u8,78u8,124u8,228u8,87u8,180u8,175u8,215u8],Some(seq![50u8,39u8,159u8,191u8,222u8,75u8,196u8,57u8,125u8,26u8,110u8,17u8,199u8,229u8,2u8,184u8,138u8,103u8]),3nat,seq![219u8,121u8,205u8,166u8,83u8,60u8,10u8,43u8,17u8,85u8,183u8,95u8,10u8,178u8,135u8,21u8,77u8,78u8]);
    assert(r::line(raw_filter_3_ct2_d8(),a_filter_3_ct2_d8(),0,1nat)==seq![219u8,121u8,205u8,166u8,83u8,60u8,10u8,43u8,17u8,85u8,183u8,95u8,10u8,178u8,135u8,21u8,77u8,78u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_1(),6nat)[0]==219u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_1(),6nat)[1]==121u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_1(),6nat)[2]==205u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_1(),6nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_1(),6nat)[4]==166u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_1(),6nat)[5]==83u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_1(),6nat)[6]==60u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_1(),6nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_1(),6nat)[8]==10u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_1(),6nat)[9]==43u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_1(),6nat)[10]==17u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_1(),6nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_1(),6nat)[12]==85u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_1(),6nat)[13]==183u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_1(),6nat)[14]==95u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_1(),6nat)[15]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_1(),6nat)[16]==10u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_1(),6nat)[17]==178u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_1(),6nat)[18]==135u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_1(),6nat)[19]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_1(),6nat)[20]==21u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_1(),6nat)[21]==77u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_1(),6nat)[22]==78u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_1(),6nat)[23]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_1(),6nat),seq![219u8,121u8,205u8,255u8,166u8,83u8,60u8,255u8,10u8,43u8,17u8,255u8,85u8,183u8,95u8,255u8,10u8,178u8,135u8,255u8,21u8,77u8,78u8,255u8]);
    assert(crate::witness_m6_m4_filter::expected_filter_3_ct2_d8().subrange(24,48)==seq![219u8,121u8,205u8,255u8,166u8,83u8,60u8,255u8,10u8,43u8,17u8,255u8,85u8,183u8,95u8,255u8,10u8,178u8,135u8,255u8,21u8,77u8,78u8,255u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_filter_3_ct2_d8(),0nat,18nat,3nat,2nat);
    assert(raw_filter_3_ct2_d8()[38]==3u8) by (compute);
    assert(crate::filter_model::row_data(raw_filter_3_ct2_d8(),0,18nat,2nat)==seq![227u8,227u8,0u8,225u8,49u8,77u8,247u8,185u8,247u8,132u8,117u8,192u8,227u8,66u8,13u8,46u8,97u8,56u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_filter_3_ct2_d8(),0,18nat,3nat,2nat)==Some(seq![219u8,121u8,205u8,166u8,83u8,60u8,10u8,43u8,17u8,85u8,183u8,95u8,10u8,178u8,135u8,21u8,77u8,78u8]));
    assert(crate::filter_model::encoded_row(3u8,seq![227u8,227u8,0u8,225u8,49u8,77u8,247u8,185u8,247u8,132u8,117u8,192u8,227u8,66u8,13u8,46u8,97u8,56u8],Some(seq![219u8,121u8,205u8,166u8,83u8,60u8,10u8,43u8,17u8,85u8,183u8,95u8,10u8,178u8,135u8,21u8,77u8,78u8]),3nat,seq![80u8,31u8,102u8,92u8,106u8,158u8,42u8,3u8,78u8,195u8,210u8,22u8,73u8,4u8,91u8,93u8,137u8,140u8],0,18nat)) by (compute);
    crate::filter_model::checked_row_hint(3u8,seq![227u8,227u8,0u8,225u8,49u8,77u8,247u8,185u8,247u8,132u8,117u8,192u8,227u8,66u8,13u8,46u8,97u8,56u8],Some(seq![219u8,121u8,205u8,166u8,83u8,60u8,10u8,43u8,17u8,85u8,183u8,95u8,10u8,178u8,135u8,21u8,77u8,78u8]),3nat,seq![80u8,31u8,102u8,92u8,106u8,158u8,42u8,3u8,78u8,195u8,210u8,22u8,73u8,4u8,91u8,93u8,137u8,140u8]);
    assert(r::line(raw_filter_3_ct2_d8(),a_filter_3_ct2_d8(),0,2nat)==seq![80u8,31u8,102u8,92u8,106u8,158u8,42u8,3u8,78u8,195u8,210u8,22u8,73u8,4u8,91u8,93u8,137u8,140u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_2(),6nat)[0]==80u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_2(),6nat)[1]==31u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_2(),6nat)[2]==102u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_2(),6nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_2(),6nat)[4]==92u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_2(),6nat)[5]==106u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_2(),6nat)[6]==158u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_2(),6nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_2(),6nat)[8]==42u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_2(),6nat)[9]==3u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_2(),6nat)[10]==78u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_2(),6nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_2(),6nat)[12]==195u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_2(),6nat)[13]==210u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_2(),6nat)[14]==22u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_2(),6nat)[15]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_2(),6nat)[16]==73u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_2(),6nat)[17]==4u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_2(),6nat)[18]==91u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_2(),6nat)[19]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_2(),6nat)[20]==93u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_2(),6nat)[21]==137u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_2(),6nat)[22]==140u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_2(),6nat)[23]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_2(),6nat),seq![80u8,31u8,102u8,255u8,92u8,106u8,158u8,255u8,42u8,3u8,78u8,255u8,195u8,210u8,22u8,255u8,73u8,4u8,91u8,255u8,93u8,137u8,140u8,255u8]);
    assert(crate::witness_m6_m4_filter::expected_filter_3_ct2_d8().subrange(48,72)==seq![80u8,31u8,102u8,255u8,92u8,106u8,158u8,255u8,42u8,3u8,78u8,255u8,195u8,210u8,22u8,255u8,73u8,4u8,91u8,255u8,93u8,137u8,140u8,255u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_filter_3_ct2_d8(),0nat,18nat,3nat,3nat);
    assert(raw_filter_3_ct2_d8()[57]==3u8) by (compute);
    assert(crate::filter_model::row_data(raw_filter_3_ct2_d8(),0,18nat,3nat)==seq![134u8,67u8,87u8,167u8,161u8,142u8,226u8,87u8,173u8,201u8,248u8,19u8,15u8,112u8,51u8,92u8,65u8,2u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_filter_3_ct2_d8(),0,18nat,3nat,3nat)==Some(seq![80u8,31u8,102u8,92u8,106u8,158u8,42u8,3u8,78u8,195u8,210u8,22u8,73u8,4u8,91u8,93u8,137u8,140u8]));
    assert(crate::filter_model::encoded_row(3u8,seq![134u8,67u8,87u8,167u8,161u8,142u8,226u8,87u8,173u8,201u8,248u8,19u8,15u8,112u8,51u8,92u8,65u8,2u8],Some(seq![80u8,31u8,102u8,92u8,106u8,158u8,42u8,3u8,78u8,195u8,210u8,22u8,73u8,4u8,91u8,93u8,137u8,140u8]),3nat,seq![174u8,82u8,138u8,44u8,255u8,34u8,13u8,216u8,229u8,49u8,205u8,144u8,76u8,216u8,168u8,176u8,241u8,156u8],0,18nat)) by (compute);
    crate::filter_model::checked_row_hint(3u8,seq![134u8,67u8,87u8,167u8,161u8,142u8,226u8,87u8,173u8,201u8,248u8,19u8,15u8,112u8,51u8,92u8,65u8,2u8],Some(seq![80u8,31u8,102u8,92u8,106u8,158u8,42u8,3u8,78u8,195u8,210u8,22u8,73u8,4u8,91u8,93u8,137u8,140u8]),3nat,seq![174u8,82u8,138u8,44u8,255u8,34u8,13u8,216u8,229u8,49u8,205u8,144u8,76u8,216u8,168u8,176u8,241u8,156u8]);
    assert(r::line(raw_filter_3_ct2_d8(),a_filter_3_ct2_d8(),0,3nat)==seq![174u8,82u8,138u8,44u8,255u8,34u8,13u8,216u8,229u8,49u8,205u8,144u8,76u8,216u8,168u8,176u8,241u8,156u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_3(),6nat)[0]==174u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_3(),6nat)[1]==82u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_3(),6nat)[2]==138u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_3(),6nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_3(),6nat)[4]==44u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_3(),6nat)[5]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_3(),6nat)[6]==34u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_3(),6nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_3(),6nat)[8]==13u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_3(),6nat)[9]==216u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_3(),6nat)[10]==229u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_3(),6nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_3(),6nat)[12]==49u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_3(),6nat)[13]==205u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_3(),6nat)[14]==144u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_3(),6nat)[15]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_3(),6nat)[16]==76u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_3(),6nat)[17]==216u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_3(),6nat)[18]==168u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_3(),6nat)[19]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_3(),6nat)[20]==176u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_3(),6nat)[21]==241u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_3(),6nat)[22]==156u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_3(),6nat)[23]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),e6_line_filter_3_ct2_d8_3(),6nat),seq![174u8,82u8,138u8,255u8,44u8,255u8,34u8,255u8,13u8,216u8,229u8,255u8,49u8,205u8,144u8,255u8,76u8,216u8,168u8,255u8,176u8,241u8,156u8,255u8]);
    assert(crate::witness_m6_m4_filter::expected_filter_3_ct2_d8().subrange(72,96)==seq![174u8,82u8,138u8,255u8,44u8,255u8,34u8,255u8,13u8,216u8,229u8,255u8,49u8,205u8,144u8,255u8,76u8,216u8,168u8,255u8,176u8,241u8,156u8,255u8]) by (compute);
    assert forall|y: nat| y<4 implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_3_ct2_d8(),r::line(raw_filter_3_ct2_d8(),a_filter_3_ct2_d8(),0,y),6nat)==crate::witness_m6_m4_filter::expected_filter_3_ct2_d8().subrange((24*y) as int,(24*(y+1)) as int) by {
        if y==0 {} else if y==1 {} else if y==2 {} else {assert(y==3);}
    }
}
#[verifier::spinoff_prover]
pub(crate) proof fn e6_complete_filter_3_ct2_d8()
    ensures match crate::still_model::MS(crate::witness_m6_m4_filter::bytes_filter_3_ct2_d8()) {crate::still_model::Outcome::Done(value)=>crate::witness_m6_m4_filter::expected_value_filter_3_ct2_d8(value),_=>false},
{

    hide(bytes_filter_3_ct2_d8);hide(z_filter_3_ct2_d8);hide(raw_filter_3_ct2_d8);hide(r::data);hide(crate::witness_m6_m4_filter::expected_filter_3_ct2_d8);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_3_ct2_d8().len()==144nat) by (compute);
    assert(crate::model::mh(bytes_filter_3_ct2_d8())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:2u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_3_ct2_d8())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:2u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_3_ct2_d8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:87nat,next:132nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_3_ct2_d8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:87nat,next:132nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_3_ct2_d8(),132nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:140nat,len:0nat,next:144nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_3_ct2_d8(),132nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:140nat,len:0nat,next:144nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_3_ct2_d8().subrange(41,128)==z_filter_3_ct2_d8()) by (compute);
    walk_filter_3_ct2_d8(bytes_filter_3_ct2_d8());

    model_filter_3_ct2_d8();
    inflate_filter_3_ct2_d8();scan_filter_3_ct2_d8();
    assert(bytes_filter_3_ct2_d8()==crate::witness_m6_m4_filter::bytes_filter_3_ct2_d8()) by (compute);
    assert(r::raw(a_filter_3_ct2_d8())==76nat) by (compute);
    assert(r::data(z_filter_3_ct2_d8(),a_filter_3_ct2_d8(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    assert(crate::model::valid_depth(2u8,8u8)) by (compute);
    crate::witness_m6_rows_model::bare_colour(bytes_filter_3_ct2_d8(),h_filter_3_ct2_d8(),87nat);
    e6_rows_filter_3_ct2_d8();
    assert(crate::witness_m6_m4_filter::expected_filter_3_ct2_d8().len()==96nat) by (compute);
    crate::witness_m6_rows_model::complete_still(bytes_filter_3_ct2_d8(),h_filter_3_ct2_d8(),z_filter_3_ct2_d8(),raw_filter_3_ct2_d8(),e6_colour_filter_3_ct2_d8(),crate::witness_m6_m4_filter::expected_filter_3_ct2_d8());
}
spec fn e6_colour_filter_4_ct2_d16() -> crate::colour_model::Context {(crate::colour_model::Context {ct:2u8,depth:16u8,plte:Seq::empty(),trns:None})}
spec fn e6_line_filter_4_ct2_d16_0() -> Seq<u8> {seq![129u8,85u8,103u8,130u8,120u8,193u8,7u8,128u8,169u8,238u8,230u8,222u8,31u8,187u8,210u8,82u8,237u8,131u8,232u8,203u8,170u8,16u8,238u8,250u8,183u8,90u8,135u8,237u8,69u8,214u8,158u8,170u8,55u8,79u8,14u8,194u8]}
spec fn e6_line_filter_4_ct2_d16_1() -> Seq<u8> {seq![17u8,75u8,207u8,50u8,200u8,56u8,44u8,67u8,93u8,174u8,184u8,74u8,224u8,139u8,205u8,254u8,36u8,148u8,83u8,33u8,135u8,67u8,146u8,178u8,3u8,200u8,232u8,60u8,115u8,10u8,166u8,67u8,27u8,253u8,72u8,227u8]}
spec fn e6_line_filter_4_ct2_d16_2() -> Seq<u8> {seq![255u8,203u8,207u8,13u8,2u8,120u8,167u8,166u8,143u8,5u8,0u8,58u8,226u8,248u8,138u8,115u8,234u8,145u8,16u8,54u8,134u8,131u8,167u8,190u8,120u8,121u8,163u8,39u8,225u8,36u8,53u8,44u8,92u8,23u8,187u8,109u8]}
spec fn e6_line_filter_4_ct2_d16_3() -> Seq<u8> {seq![80u8,184u8,125u8,102u8,13u8,229u8,58u8,85u8,50u8,133u8,94u8,40u8,96u8,120u8,163u8,192u8,211u8,102u8,81u8,92u8,152u8,80u8,114u8,59u8,182u8,197u8,227u8,130u8,61u8,102u8,76u8,144u8,31u8,236u8,106u8,201u8]}
#[verifier::spinoff_prover]
proof fn e6_rows_filter_4_ct2_d16()
    ensures forall|y: nat| y<4 ==> #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),r::line(raw_filter_4_ct2_d16(),a_filter_4_ct2_d16(),0,y),6nat)==crate::witness_m6_m4_filter::expected_filter_4_ct2_d16().subrange((24*y) as int,(24*(y+1)) as int),
{
    hide(raw_filter_4_ct2_d16);hide(crate::colour_model::pixel);hide(crate::witness_m6_m4_filter::expected_filter_4_ct2_d16);
    crate::colour_model::depth_values(16u8);
    assert(r::base(a_filter_4_ct2_d16(),0)==0nat && r::rb(a_filter_4_ct2_d16(),0)==36nat && crate::filter_model::bpp(r::bits(a_filter_4_ct2_d16()))==6nat) by (compute);
    crate::filter_model::pass_row_equation(raw_filter_4_ct2_d16(),0nat,36nat,6nat,0nat);
    assert(raw_filter_4_ct2_d16()[0]==4u8) by (compute);
    assert(crate::filter_model::row_data(raw_filter_4_ct2_d16(),0,36nat,0nat)==seq![129u8,85u8,103u8,130u8,120u8,193u8,134u8,43u8,66u8,108u8,110u8,29u8,24u8,59u8,41u8,100u8,7u8,165u8,201u8,16u8,216u8,190u8,1u8,119u8,207u8,143u8,221u8,221u8,87u8,220u8,231u8,80u8,176u8,98u8,201u8,236u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_filter_4_ct2_d16(),0,36nat,6nat,0nat)==None);
    assert(crate::filter_model::encoded_row(4u8,seq![129u8,85u8,103u8,130u8,120u8,193u8,134u8,43u8,66u8,108u8,110u8,29u8,24u8,59u8,41u8,100u8,7u8,165u8,201u8,16u8,216u8,190u8,1u8,119u8,207u8,143u8,221u8,221u8,87u8,220u8,231u8,80u8,176u8,98u8,201u8,236u8],None,6nat,seq![129u8,85u8,103u8,130u8,120u8,193u8,7u8,128u8,169u8,238u8,230u8,222u8,31u8,187u8,210u8,82u8,237u8,131u8,232u8,203u8,170u8,16u8,238u8,250u8,183u8,90u8,135u8,237u8,69u8,214u8,158u8,170u8,55u8,79u8,14u8,194u8],0,36nat)) by (compute);
    crate::filter_model::checked_row_hint(4u8,seq![129u8,85u8,103u8,130u8,120u8,193u8,134u8,43u8,66u8,108u8,110u8,29u8,24u8,59u8,41u8,100u8,7u8,165u8,201u8,16u8,216u8,190u8,1u8,119u8,207u8,143u8,221u8,221u8,87u8,220u8,231u8,80u8,176u8,98u8,201u8,236u8],None,6nat,seq![129u8,85u8,103u8,130u8,120u8,193u8,7u8,128u8,169u8,238u8,230u8,222u8,31u8,187u8,210u8,82u8,237u8,131u8,232u8,203u8,170u8,16u8,238u8,250u8,183u8,90u8,135u8,237u8,69u8,214u8,158u8,170u8,55u8,79u8,14u8,194u8]);
    assert(r::line(raw_filter_4_ct2_d16(),a_filter_4_ct2_d16(),0,0nat)==seq![129u8,85u8,103u8,130u8,120u8,193u8,7u8,128u8,169u8,238u8,230u8,222u8,31u8,187u8,210u8,82u8,237u8,131u8,232u8,203u8,170u8,16u8,238u8,250u8,183u8,90u8,135u8,237u8,69u8,214u8,158u8,170u8,55u8,79u8,14u8,194u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_0(),6nat)[0]==129u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_0(),6nat)[1]==103u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_0(),6nat)[2]==120u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_0(),6nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_0(),6nat)[4]==7u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_0(),6nat)[5]==169u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_0(),6nat)[6]==230u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_0(),6nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_0(),6nat)[8]==32u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_0(),6nat)[9]==210u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_0(),6nat)[10]==237u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_0(),6nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_0(),6nat)[12]==232u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_0(),6nat)[13]==169u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_0(),6nat)[14]==238u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_0(),6nat)[15]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_0(),6nat)[16]==183u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_0(),6nat)[17]==135u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_0(),6nat)[18]==70u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_0(),6nat)[19]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_0(),6nat)[20]==158u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_0(),6nat)[21]==55u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_0(),6nat)[22]==15u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_0(),6nat)[23]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_0(),6nat),seq![129u8,103u8,120u8,255u8,7u8,169u8,230u8,255u8,32u8,210u8,237u8,255u8,232u8,169u8,238u8,255u8,183u8,135u8,70u8,255u8,158u8,55u8,15u8,255u8]);
    assert(crate::witness_m6_m4_filter::expected_filter_4_ct2_d16().subrange(0,24)==seq![129u8,103u8,120u8,255u8,7u8,169u8,230u8,255u8,32u8,210u8,237u8,255u8,232u8,169u8,238u8,255u8,183u8,135u8,70u8,255u8,158u8,55u8,15u8,255u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_filter_4_ct2_d16(),0nat,36nat,6nat,1nat);
    assert(raw_filter_4_ct2_d16()[37]==4u8) by (compute);
    assert(crate::filter_model::row_data(raw_filter_4_ct2_d16(),0,36nat,1nat)==seq![144u8,246u8,104u8,176u8,80u8,119u8,37u8,195u8,142u8,44u8,210u8,18u8,180u8,11u8,36u8,172u8,108u8,74u8,107u8,150u8,221u8,69u8,110u8,184u8,176u8,167u8,97u8,79u8,46u8,88u8,163u8,123u8,148u8,193u8,3u8,217u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_filter_4_ct2_d16(),0,36nat,6nat,1nat)==Some(seq![129u8,85u8,103u8,130u8,120u8,193u8,7u8,128u8,169u8,238u8,230u8,222u8,31u8,187u8,210u8,82u8,237u8,131u8,232u8,203u8,170u8,16u8,238u8,250u8,183u8,90u8,135u8,237u8,69u8,214u8,158u8,170u8,55u8,79u8,14u8,194u8]));
    assert(crate::filter_model::encoded_row(4u8,seq![144u8,246u8,104u8,176u8,80u8,119u8,37u8,195u8,142u8,44u8,210u8,18u8,180u8,11u8,36u8,172u8,108u8,74u8,107u8,150u8,221u8,69u8,110u8,184u8,176u8,167u8,97u8,79u8,46u8,88u8,163u8,123u8,148u8,193u8,3u8,217u8],Some(seq![129u8,85u8,103u8,130u8,120u8,193u8,7u8,128u8,169u8,238u8,230u8,222u8,31u8,187u8,210u8,82u8,237u8,131u8,232u8,203u8,170u8,16u8,238u8,250u8,183u8,90u8,135u8,237u8,69u8,214u8,158u8,170u8,55u8,79u8,14u8,194u8]),6nat,seq![17u8,75u8,207u8,50u8,200u8,56u8,44u8,67u8,93u8,174u8,184u8,74u8,224u8,139u8,205u8,254u8,36u8,148u8,83u8,33u8,135u8,67u8,146u8,178u8,3u8,200u8,232u8,60u8,115u8,10u8,166u8,67u8,27u8,253u8,72u8,227u8],0,36nat)) by (compute);
    crate::filter_model::checked_row_hint(4u8,seq![144u8,246u8,104u8,176u8,80u8,119u8,37u8,195u8,142u8,44u8,210u8,18u8,180u8,11u8,36u8,172u8,108u8,74u8,107u8,150u8,221u8,69u8,110u8,184u8,176u8,167u8,97u8,79u8,46u8,88u8,163u8,123u8,148u8,193u8,3u8,217u8],Some(seq![129u8,85u8,103u8,130u8,120u8,193u8,7u8,128u8,169u8,238u8,230u8,222u8,31u8,187u8,210u8,82u8,237u8,131u8,232u8,203u8,170u8,16u8,238u8,250u8,183u8,90u8,135u8,237u8,69u8,214u8,158u8,170u8,55u8,79u8,14u8,194u8]),6nat,seq![17u8,75u8,207u8,50u8,200u8,56u8,44u8,67u8,93u8,174u8,184u8,74u8,224u8,139u8,205u8,254u8,36u8,148u8,83u8,33u8,135u8,67u8,146u8,178u8,3u8,200u8,232u8,60u8,115u8,10u8,166u8,67u8,27u8,253u8,72u8,227u8]);
    assert(r::line(raw_filter_4_ct2_d16(),a_filter_4_ct2_d16(),0,1nat)==seq![17u8,75u8,207u8,50u8,200u8,56u8,44u8,67u8,93u8,174u8,184u8,74u8,224u8,139u8,205u8,254u8,36u8,148u8,83u8,33u8,135u8,67u8,146u8,178u8,3u8,200u8,232u8,60u8,115u8,10u8,166u8,67u8,27u8,253u8,72u8,227u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_1(),6nat)[0]==17u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_1(),6nat)[1]==206u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_1(),6nat)[2]==199u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_1(),6nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_1(),6nat)[4]==44u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_1(),6nat)[5]==93u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_1(),6nat)[6]==184u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_1(),6nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_1(),6nat)[8]==224u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_1(),6nat)[9]==205u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_1(),6nat)[10]==36u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_1(),6nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_1(),6nat)[12]==83u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_1(),6nat)[13]==135u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_1(),6nat)[14]==146u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_1(),6nat)[15]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_1(),6nat)[16]==4u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_1(),6nat)[17]==231u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_1(),6nat)[18]==115u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_1(),6nat)[19]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_1(),6nat)[20]==166u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_1(),6nat)[21]==28u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_1(),6nat)[22]==73u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_1(),6nat)[23]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_1(),6nat),seq![17u8,206u8,199u8,255u8,44u8,93u8,184u8,255u8,224u8,205u8,36u8,255u8,83u8,135u8,146u8,255u8,4u8,231u8,115u8,255u8,166u8,28u8,73u8,255u8]);
    assert(crate::witness_m6_m4_filter::expected_filter_4_ct2_d16().subrange(24,48)==seq![17u8,206u8,199u8,255u8,44u8,93u8,184u8,255u8,224u8,205u8,36u8,255u8,83u8,135u8,146u8,255u8,4u8,231u8,115u8,255u8,166u8,28u8,73u8,255u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_filter_4_ct2_d16(),0nat,36nat,6nat,2nat);
    assert(raw_filter_4_ct2_d16()[74]==4u8) by (compute);
    assert(crate::filter_model::row_data(raw_filter_4_ct2_d16(),0,36nat,2nat)==seq![238u8,128u8,0u8,219u8,58u8,64u8,168u8,219u8,50u8,87u8,254u8,194u8,2u8,82u8,189u8,110u8,234u8,253u8,189u8,171u8,255u8,64u8,189u8,12u8,117u8,177u8,187u8,164u8,79u8,26u8,143u8,233u8,65u8,26u8,218u8,138u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_filter_4_ct2_d16(),0,36nat,6nat,2nat)==Some(seq![17u8,75u8,207u8,50u8,200u8,56u8,44u8,67u8,93u8,174u8,184u8,74u8,224u8,139u8,205u8,254u8,36u8,148u8,83u8,33u8,135u8,67u8,146u8,178u8,3u8,200u8,232u8,60u8,115u8,10u8,166u8,67u8,27u8,253u8,72u8,227u8]));
    assert(crate::filter_model::encoded_row(4u8,seq![238u8,128u8,0u8,219u8,58u8,64u8,168u8,219u8,50u8,87u8,254u8,194u8,2u8,82u8,189u8,110u8,234u8,253u8,189u8,171u8,255u8,64u8,189u8,12u8,117u8,177u8,187u8,164u8,79u8,26u8,143u8,233u8,65u8,26u8,218u8,138u8],Some(seq![17u8,75u8,207u8,50u8,200u8,56u8,44u8,67u8,93u8,174u8,184u8,74u8,224u8,139u8,205u8,254u8,36u8,148u8,83u8,33u8,135u8,67u8,146u8,178u8,3u8,200u8,232u8,60u8,115u8,10u8,166u8,67u8,27u8,253u8,72u8,227u8]),6nat,seq![255u8,203u8,207u8,13u8,2u8,120u8,167u8,166u8,143u8,5u8,0u8,58u8,226u8,248u8,138u8,115u8,234u8,145u8,16u8,54u8,134u8,131u8,167u8,190u8,120u8,121u8,163u8,39u8,225u8,36u8,53u8,44u8,92u8,23u8,187u8,109u8],0,36nat)) by (compute);
    crate::filter_model::checked_row_hint(4u8,seq![238u8,128u8,0u8,219u8,58u8,64u8,168u8,219u8,50u8,87u8,254u8,194u8,2u8,82u8,189u8,110u8,234u8,253u8,189u8,171u8,255u8,64u8,189u8,12u8,117u8,177u8,187u8,164u8,79u8,26u8,143u8,233u8,65u8,26u8,218u8,138u8],Some(seq![17u8,75u8,207u8,50u8,200u8,56u8,44u8,67u8,93u8,174u8,184u8,74u8,224u8,139u8,205u8,254u8,36u8,148u8,83u8,33u8,135u8,67u8,146u8,178u8,3u8,200u8,232u8,60u8,115u8,10u8,166u8,67u8,27u8,253u8,72u8,227u8]),6nat,seq![255u8,203u8,207u8,13u8,2u8,120u8,167u8,166u8,143u8,5u8,0u8,58u8,226u8,248u8,138u8,115u8,234u8,145u8,16u8,54u8,134u8,131u8,167u8,190u8,120u8,121u8,163u8,39u8,225u8,36u8,53u8,44u8,92u8,23u8,187u8,109u8]);
    assert(r::line(raw_filter_4_ct2_d16(),a_filter_4_ct2_d16(),0,2nat)==seq![255u8,203u8,207u8,13u8,2u8,120u8,167u8,166u8,143u8,5u8,0u8,58u8,226u8,248u8,138u8,115u8,234u8,145u8,16u8,54u8,134u8,131u8,167u8,190u8,120u8,121u8,163u8,39u8,225u8,36u8,53u8,44u8,92u8,23u8,187u8,109u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_2(),6nat)[0]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_2(),6nat)[1]==206u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_2(),6nat)[2]==2u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_2(),6nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_2(),6nat)[4]==167u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_2(),6nat)[5]==142u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_2(),6nat)[6]==0u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_2(),6nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_2(),6nat)[8]==226u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_2(),6nat)[9]==138u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_2(),6nat)[10]==234u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_2(),6nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_2(),6nat)[12]==16u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_2(),6nat)[13]==134u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_2(),6nat)[14]==167u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_2(),6nat)[15]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_2(),6nat)[16]==120u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_2(),6nat)[17]==163u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_2(),6nat)[18]==224u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_2(),6nat)[19]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_2(),6nat)[20]==53u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_2(),6nat)[21]==92u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_2(),6nat)[22]==187u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_2(),6nat)[23]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_2(),6nat),seq![255u8,206u8,2u8,255u8,167u8,142u8,0u8,255u8,226u8,138u8,234u8,255u8,16u8,134u8,167u8,255u8,120u8,163u8,224u8,255u8,53u8,92u8,187u8,255u8]);
    assert(crate::witness_m6_m4_filter::expected_filter_4_ct2_d16().subrange(48,72)==seq![255u8,206u8,2u8,255u8,167u8,142u8,0u8,255u8,226u8,138u8,234u8,255u8,16u8,134u8,167u8,255u8,120u8,163u8,224u8,255u8,53u8,92u8,187u8,255u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_filter_4_ct2_d16(),0nat,36nat,6nat,3nat);
    assert(raw_filter_4_ct2_d16()[111]==4u8) by (compute);
    assert(crate::filter_model::row_data(raw_filter_4_ct2_d16(),0,36nat,3nat)==seq![81u8,237u8,174u8,89u8,11u8,109u8,234u8,175u8,181u8,31u8,81u8,176u8,185u8,210u8,113u8,59u8,233u8,213u8,65u8,38u8,245u8,144u8,203u8,170u8,62u8,76u8,64u8,91u8,150u8,66u8,212u8,23u8,124u8,106u8,45u8,92u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_filter_4_ct2_d16(),0,36nat,6nat,3nat)==Some(seq![255u8,203u8,207u8,13u8,2u8,120u8,167u8,166u8,143u8,5u8,0u8,58u8,226u8,248u8,138u8,115u8,234u8,145u8,16u8,54u8,134u8,131u8,167u8,190u8,120u8,121u8,163u8,39u8,225u8,36u8,53u8,44u8,92u8,23u8,187u8,109u8]));
    assert(crate::filter_model::encoded_row(4u8,seq![81u8,237u8,174u8,89u8,11u8,109u8,234u8,175u8,181u8,31u8,81u8,176u8,185u8,210u8,113u8,59u8,233u8,213u8,65u8,38u8,245u8,144u8,203u8,170u8,62u8,76u8,64u8,91u8,150u8,66u8,212u8,23u8,124u8,106u8,45u8,92u8],Some(seq![255u8,203u8,207u8,13u8,2u8,120u8,167u8,166u8,143u8,5u8,0u8,58u8,226u8,248u8,138u8,115u8,234u8,145u8,16u8,54u8,134u8,131u8,167u8,190u8,120u8,121u8,163u8,39u8,225u8,36u8,53u8,44u8,92u8,23u8,187u8,109u8]),6nat,seq![80u8,184u8,125u8,102u8,13u8,229u8,58u8,85u8,50u8,133u8,94u8,40u8,96u8,120u8,163u8,192u8,211u8,102u8,81u8,92u8,152u8,80u8,114u8,59u8,182u8,197u8,227u8,130u8,61u8,102u8,76u8,144u8,31u8,236u8,106u8,201u8],0,36nat)) by (compute);
    crate::filter_model::checked_row_hint(4u8,seq![81u8,237u8,174u8,89u8,11u8,109u8,234u8,175u8,181u8,31u8,81u8,176u8,185u8,210u8,113u8,59u8,233u8,213u8,65u8,38u8,245u8,144u8,203u8,170u8,62u8,76u8,64u8,91u8,150u8,66u8,212u8,23u8,124u8,106u8,45u8,92u8],Some(seq![255u8,203u8,207u8,13u8,2u8,120u8,167u8,166u8,143u8,5u8,0u8,58u8,226u8,248u8,138u8,115u8,234u8,145u8,16u8,54u8,134u8,131u8,167u8,190u8,120u8,121u8,163u8,39u8,225u8,36u8,53u8,44u8,92u8,23u8,187u8,109u8]),6nat,seq![80u8,184u8,125u8,102u8,13u8,229u8,58u8,85u8,50u8,133u8,94u8,40u8,96u8,120u8,163u8,192u8,211u8,102u8,81u8,92u8,152u8,80u8,114u8,59u8,182u8,197u8,227u8,130u8,61u8,102u8,76u8,144u8,31u8,236u8,106u8,201u8]);
    assert(r::line(raw_filter_4_ct2_d16(),a_filter_4_ct2_d16(),0,3nat)==seq![80u8,184u8,125u8,102u8,13u8,229u8,58u8,85u8,50u8,133u8,94u8,40u8,96u8,120u8,163u8,192u8,211u8,102u8,81u8,92u8,152u8,80u8,114u8,59u8,182u8,197u8,227u8,130u8,61u8,102u8,76u8,144u8,31u8,236u8,106u8,201u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_3(),6nat)[0]==80u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_3(),6nat)[1]==125u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_3(),6nat)[2]==14u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_3(),6nat)[3]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_3(),6nat)[4]==58u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_3(),6nat)[5]==50u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_3(),6nat)[6]==94u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_3(),6nat)[7]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_3(),6nat)[8]==96u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_3(),6nat)[9]==163u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_3(),6nat)[10]==211u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_3(),6nat)[11]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_3(),6nat)[12]==81u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_3(),6nat)[13]==152u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_3(),6nat)[14]==114u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_3(),6nat)[15]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_3(),6nat)[16]==182u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_3(),6nat)[17]==227u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_3(),6nat)[18]==61u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_3(),6nat)[19]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_3(),6nat)[20]==76u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_3(),6nat)[21]==32u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_3(),6nat)[22]==106u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_3(),6nat)[23]==255u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),e6_line_filter_4_ct2_d16_3(),6nat),seq![80u8,125u8,14u8,255u8,58u8,50u8,94u8,255u8,96u8,163u8,211u8,255u8,81u8,152u8,114u8,255u8,182u8,227u8,61u8,255u8,76u8,32u8,106u8,255u8]);
    assert(crate::witness_m6_m4_filter::expected_filter_4_ct2_d16().subrange(72,96)==seq![80u8,125u8,14u8,255u8,58u8,50u8,94u8,255u8,96u8,163u8,211u8,255u8,81u8,152u8,114u8,255u8,182u8,227u8,61u8,255u8,76u8,32u8,106u8,255u8]) by (compute);
    assert forall|y: nat| y<4 implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct2_d16(),r::line(raw_filter_4_ct2_d16(),a_filter_4_ct2_d16(),0,y),6nat)==crate::witness_m6_m4_filter::expected_filter_4_ct2_d16().subrange((24*y) as int,(24*(y+1)) as int) by {
        if y==0 {} else if y==1 {} else if y==2 {} else {assert(y==3);}
    }
}
#[verifier::spinoff_prover]
pub(crate) proof fn e6_complete_filter_4_ct2_d16()
    ensures match crate::still_model::MS(crate::witness_m6_m4_filter::bytes_filter_4_ct2_d16()) {crate::still_model::Outcome::Done(value)=>crate::witness_m6_m4_filter::expected_value_filter_4_ct2_d16(value),_=>false},
{

    hide(bytes_filter_4_ct2_d16);hide(z_filter_4_ct2_d16);hide(raw_filter_4_ct2_d16);hide(r::data);hide(crate::witness_m6_m4_filter::expected_filter_4_ct2_d16);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_4_ct2_d16().len()==216nat) by (compute);
    assert(crate::model::mh(bytes_filter_4_ct2_d16())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:2u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_4_ct2_d16())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:2u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_4_ct2_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:159nat,next:204nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_4_ct2_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:159nat,next:204nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_4_ct2_d16(),204nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:212nat,len:0nat,next:216nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_4_ct2_d16(),204nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:212nat,len:0nat,next:216nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_4_ct2_d16().subrange(41,200)==z_filter_4_ct2_d16()) by (compute);
    walk_filter_4_ct2_d16(bytes_filter_4_ct2_d16());

    model_filter_4_ct2_d16();
    inflate_filter_4_ct2_d16();scan_filter_4_ct2_d16();
    assert(bytes_filter_4_ct2_d16()==crate::witness_m6_m4_filter::bytes_filter_4_ct2_d16()) by (compute);
    assert(r::raw(a_filter_4_ct2_d16())==148nat) by (compute);
    assert(r::data(z_filter_4_ct2_d16(),a_filter_4_ct2_d16(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    assert(crate::model::valid_depth(2u8,16u8)) by (compute);
    crate::witness_m6_rows_model::bare_colour(bytes_filter_4_ct2_d16(),h_filter_4_ct2_d16(),159nat);
    e6_rows_filter_4_ct2_d16();
    assert(crate::witness_m6_m4_filter::expected_filter_4_ct2_d16().len()==96nat) by (compute);
    crate::witness_m6_rows_model::complete_still(bytes_filter_4_ct2_d16(),h_filter_4_ct2_d16(),z_filter_4_ct2_d16(),raw_filter_4_ct2_d16(),e6_colour_filter_4_ct2_d16(),crate::witness_m6_m4_filter::expected_filter_4_ct2_d16());
}
spec fn e6_colour_filter_1_ct4_d8() -> crate::colour_model::Context {(crate::colour_model::Context {ct:4u8,depth:8u8,plte:Seq::empty(),trns:None})}
spec fn e6_line_filter_1_ct4_d8_0() -> Seq<u8> {seq![79u8,65u8,104u8,158u8,10u8,151u8,105u8,222u8,71u8,82u8,43u8,193u8]}
spec fn e6_line_filter_1_ct4_d8_1() -> Seq<u8> {seq![164u8,112u8,226u8,125u8,185u8,51u8,95u8,72u8,7u8,175u8,91u8,120u8]}
spec fn e6_line_filter_1_ct4_d8_2() -> Seq<u8> {seq![80u8,71u8,17u8,132u8,115u8,65u8,220u8,251u8,223u8,161u8,131u8,152u8]}
spec fn e6_line_filter_1_ct4_d8_3() -> Seq<u8> {seq![37u8,69u8,39u8,198u8,136u8,234u8,13u8,255u8,77u8,218u8,130u8,203u8]}
#[verifier::spinoff_prover]
proof fn e6_rows_filter_1_ct4_d8()
    ensures forall|y: nat| y<4 ==> #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),r::line(raw_filter_1_ct4_d8(),a_filter_1_ct4_d8(),0,y),6nat)==crate::witness_m6_m4_filter::expected_filter_1_ct4_d8().subrange((24*y) as int,(24*(y+1)) as int),
{
    hide(raw_filter_1_ct4_d8);hide(crate::colour_model::pixel);hide(crate::witness_m6_m4_filter::expected_filter_1_ct4_d8);
    crate::colour_model::depth_values(8u8);
    assert(r::base(a_filter_1_ct4_d8(),0)==0nat && r::rb(a_filter_1_ct4_d8(),0)==12nat && crate::filter_model::bpp(r::bits(a_filter_1_ct4_d8()))==2nat) by (compute);
    crate::filter_model::pass_row_equation(raw_filter_1_ct4_d8(),0nat,12nat,2nat,0nat);
    assert(raw_filter_1_ct4_d8()[0]==1u8) by (compute);
    assert(crate::filter_model::row_data(raw_filter_1_ct4_d8(),0,12nat,0nat)==seq![79u8,65u8,25u8,93u8,162u8,249u8,95u8,71u8,222u8,116u8,228u8,111u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_filter_1_ct4_d8(),0,12nat,2nat,0nat)==None);
    assert(crate::filter_model::encoded_row(1u8,seq![79u8,65u8,25u8,93u8,162u8,249u8,95u8,71u8,222u8,116u8,228u8,111u8],None,2nat,seq![79u8,65u8,104u8,158u8,10u8,151u8,105u8,222u8,71u8,82u8,43u8,193u8],0,12nat)) by (compute);
    crate::filter_model::checked_row_hint(1u8,seq![79u8,65u8,25u8,93u8,162u8,249u8,95u8,71u8,222u8,116u8,228u8,111u8],None,2nat,seq![79u8,65u8,104u8,158u8,10u8,151u8,105u8,222u8,71u8,82u8,43u8,193u8]);
    assert(r::line(raw_filter_1_ct4_d8(),a_filter_1_ct4_d8(),0,0nat)==seq![79u8,65u8,104u8,158u8,10u8,151u8,105u8,222u8,71u8,82u8,43u8,193u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_0(),6nat)[0]==79u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_0(),6nat)[1]==79u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_0(),6nat)[2]==79u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_0(),6nat)[3]==65u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_0(),6nat)[4]==104u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_0(),6nat)[5]==104u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_0(),6nat)[6]==104u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_0(),6nat)[7]==158u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_0(),6nat)[8]==10u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_0(),6nat)[9]==10u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_0(),6nat)[10]==10u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_0(),6nat)[11]==151u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_0(),6nat)[12]==105u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_0(),6nat)[13]==105u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_0(),6nat)[14]==105u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_0(),6nat)[15]==222u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_0(),6nat)[16]==71u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_0(),6nat)[17]==71u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_0(),6nat)[18]==71u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_0(),6nat)[19]==82u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_0(),6nat)[20]==43u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_0(),6nat)[21]==43u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_0(),6nat)[22]==43u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_0(),6nat)[23]==193u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_0(),6nat),seq![79u8,79u8,79u8,65u8,104u8,104u8,104u8,158u8,10u8,10u8,10u8,151u8,105u8,105u8,105u8,222u8,71u8,71u8,71u8,82u8,43u8,43u8,43u8,193u8]);
    assert(crate::witness_m6_m4_filter::expected_filter_1_ct4_d8().subrange(0,24)==seq![79u8,79u8,79u8,65u8,104u8,104u8,104u8,158u8,10u8,10u8,10u8,151u8,105u8,105u8,105u8,222u8,71u8,71u8,71u8,82u8,43u8,43u8,43u8,193u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_filter_1_ct4_d8(),0nat,12nat,2nat,1nat);
    assert(raw_filter_1_ct4_d8()[13]==1u8) by (compute);
    assert(crate::filter_model::row_data(raw_filter_1_ct4_d8(),0,12nat,1nat)==seq![164u8,112u8,62u8,13u8,215u8,182u8,166u8,21u8,168u8,103u8,84u8,201u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_filter_1_ct4_d8(),0,12nat,2nat,1nat)==Some(seq![79u8,65u8,104u8,158u8,10u8,151u8,105u8,222u8,71u8,82u8,43u8,193u8]));
    assert(crate::filter_model::encoded_row(1u8,seq![164u8,112u8,62u8,13u8,215u8,182u8,166u8,21u8,168u8,103u8,84u8,201u8],Some(seq![79u8,65u8,104u8,158u8,10u8,151u8,105u8,222u8,71u8,82u8,43u8,193u8]),2nat,seq![164u8,112u8,226u8,125u8,185u8,51u8,95u8,72u8,7u8,175u8,91u8,120u8],0,12nat)) by (compute);
    crate::filter_model::checked_row_hint(1u8,seq![164u8,112u8,62u8,13u8,215u8,182u8,166u8,21u8,168u8,103u8,84u8,201u8],Some(seq![79u8,65u8,104u8,158u8,10u8,151u8,105u8,222u8,71u8,82u8,43u8,193u8]),2nat,seq![164u8,112u8,226u8,125u8,185u8,51u8,95u8,72u8,7u8,175u8,91u8,120u8]);
    assert(r::line(raw_filter_1_ct4_d8(),a_filter_1_ct4_d8(),0,1nat)==seq![164u8,112u8,226u8,125u8,185u8,51u8,95u8,72u8,7u8,175u8,91u8,120u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_1(),6nat)[0]==164u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_1(),6nat)[1]==164u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_1(),6nat)[2]==164u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_1(),6nat)[3]==112u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_1(),6nat)[4]==226u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_1(),6nat)[5]==226u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_1(),6nat)[6]==226u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_1(),6nat)[7]==125u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_1(),6nat)[8]==185u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_1(),6nat)[9]==185u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_1(),6nat)[10]==185u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_1(),6nat)[11]==51u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_1(),6nat)[12]==95u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_1(),6nat)[13]==95u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_1(),6nat)[14]==95u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_1(),6nat)[15]==72u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_1(),6nat)[16]==7u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_1(),6nat)[17]==7u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_1(),6nat)[18]==7u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_1(),6nat)[19]==175u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_1(),6nat)[20]==91u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_1(),6nat)[21]==91u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_1(),6nat)[22]==91u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_1(),6nat)[23]==120u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_1(),6nat),seq![164u8,164u8,164u8,112u8,226u8,226u8,226u8,125u8,185u8,185u8,185u8,51u8,95u8,95u8,95u8,72u8,7u8,7u8,7u8,175u8,91u8,91u8,91u8,120u8]);
    assert(crate::witness_m6_m4_filter::expected_filter_1_ct4_d8().subrange(24,48)==seq![164u8,164u8,164u8,112u8,226u8,226u8,226u8,125u8,185u8,185u8,185u8,51u8,95u8,95u8,95u8,72u8,7u8,7u8,7u8,175u8,91u8,91u8,91u8,120u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_filter_1_ct4_d8(),0nat,12nat,2nat,2nat);
    assert(raw_filter_1_ct4_d8()[26]==1u8) by (compute);
    assert(crate::filter_model::row_data(raw_filter_1_ct4_d8(),0,12nat,2nat)==seq![80u8,71u8,193u8,61u8,98u8,189u8,105u8,186u8,3u8,166u8,164u8,247u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_filter_1_ct4_d8(),0,12nat,2nat,2nat)==Some(seq![164u8,112u8,226u8,125u8,185u8,51u8,95u8,72u8,7u8,175u8,91u8,120u8]));
    assert(crate::filter_model::encoded_row(1u8,seq![80u8,71u8,193u8,61u8,98u8,189u8,105u8,186u8,3u8,166u8,164u8,247u8],Some(seq![164u8,112u8,226u8,125u8,185u8,51u8,95u8,72u8,7u8,175u8,91u8,120u8]),2nat,seq![80u8,71u8,17u8,132u8,115u8,65u8,220u8,251u8,223u8,161u8,131u8,152u8],0,12nat)) by (compute);
    crate::filter_model::checked_row_hint(1u8,seq![80u8,71u8,193u8,61u8,98u8,189u8,105u8,186u8,3u8,166u8,164u8,247u8],Some(seq![164u8,112u8,226u8,125u8,185u8,51u8,95u8,72u8,7u8,175u8,91u8,120u8]),2nat,seq![80u8,71u8,17u8,132u8,115u8,65u8,220u8,251u8,223u8,161u8,131u8,152u8]);
    assert(r::line(raw_filter_1_ct4_d8(),a_filter_1_ct4_d8(),0,2nat)==seq![80u8,71u8,17u8,132u8,115u8,65u8,220u8,251u8,223u8,161u8,131u8,152u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_2(),6nat)[0]==80u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_2(),6nat)[1]==80u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_2(),6nat)[2]==80u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_2(),6nat)[3]==71u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_2(),6nat)[4]==17u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_2(),6nat)[5]==17u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_2(),6nat)[6]==17u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_2(),6nat)[7]==132u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_2(),6nat)[8]==115u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_2(),6nat)[9]==115u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_2(),6nat)[10]==115u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_2(),6nat)[11]==65u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_2(),6nat)[12]==220u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_2(),6nat)[13]==220u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_2(),6nat)[14]==220u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_2(),6nat)[15]==251u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_2(),6nat)[16]==223u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_2(),6nat)[17]==223u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_2(),6nat)[18]==223u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_2(),6nat)[19]==161u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_2(),6nat)[20]==131u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_2(),6nat)[21]==131u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_2(),6nat)[22]==131u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_2(),6nat)[23]==152u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_2(),6nat),seq![80u8,80u8,80u8,71u8,17u8,17u8,17u8,132u8,115u8,115u8,115u8,65u8,220u8,220u8,220u8,251u8,223u8,223u8,223u8,161u8,131u8,131u8,131u8,152u8]);
    assert(crate::witness_m6_m4_filter::expected_filter_1_ct4_d8().subrange(48,72)==seq![80u8,80u8,80u8,71u8,17u8,17u8,17u8,132u8,115u8,115u8,115u8,65u8,220u8,220u8,220u8,251u8,223u8,223u8,223u8,161u8,131u8,131u8,131u8,152u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_filter_1_ct4_d8(),0nat,12nat,2nat,3nat);
    assert(raw_filter_1_ct4_d8()[39]==1u8) by (compute);
    assert(crate::filter_model::row_data(raw_filter_1_ct4_d8(),0,12nat,3nat)==seq![37u8,69u8,2u8,129u8,97u8,36u8,133u8,21u8,64u8,219u8,53u8,241u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_filter_1_ct4_d8(),0,12nat,2nat,3nat)==Some(seq![80u8,71u8,17u8,132u8,115u8,65u8,220u8,251u8,223u8,161u8,131u8,152u8]));
    assert(crate::filter_model::encoded_row(1u8,seq![37u8,69u8,2u8,129u8,97u8,36u8,133u8,21u8,64u8,219u8,53u8,241u8],Some(seq![80u8,71u8,17u8,132u8,115u8,65u8,220u8,251u8,223u8,161u8,131u8,152u8]),2nat,seq![37u8,69u8,39u8,198u8,136u8,234u8,13u8,255u8,77u8,218u8,130u8,203u8],0,12nat)) by (compute);
    crate::filter_model::checked_row_hint(1u8,seq![37u8,69u8,2u8,129u8,97u8,36u8,133u8,21u8,64u8,219u8,53u8,241u8],Some(seq![80u8,71u8,17u8,132u8,115u8,65u8,220u8,251u8,223u8,161u8,131u8,152u8]),2nat,seq![37u8,69u8,39u8,198u8,136u8,234u8,13u8,255u8,77u8,218u8,130u8,203u8]);
    assert(r::line(raw_filter_1_ct4_d8(),a_filter_1_ct4_d8(),0,3nat)==seq![37u8,69u8,39u8,198u8,136u8,234u8,13u8,255u8,77u8,218u8,130u8,203u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_3(),6nat)[0]==37u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_3(),6nat)[1]==37u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_3(),6nat)[2]==37u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_3(),6nat)[3]==69u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_3(),6nat)[4]==39u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_3(),6nat)[5]==39u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_3(),6nat)[6]==39u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_3(),6nat)[7]==198u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_3(),6nat)[8]==136u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_3(),6nat)[9]==136u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_3(),6nat)[10]==136u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_3(),6nat)[11]==234u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_3(),6nat)[12]==13u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_3(),6nat)[13]==13u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_3(),6nat)[14]==13u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_3(),6nat)[15]==255u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_3(),6nat)[16]==77u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_3(),6nat)[17]==77u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_3(),6nat)[18]==77u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_3(),6nat)[19]==218u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_3(),6nat)[20]==130u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_3(),6nat)[21]==130u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_3(),6nat)[22]==130u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_3(),6nat)[23]==203u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),e6_line_filter_1_ct4_d8_3(),6nat),seq![37u8,37u8,37u8,69u8,39u8,39u8,39u8,198u8,136u8,136u8,136u8,234u8,13u8,13u8,13u8,255u8,77u8,77u8,77u8,218u8,130u8,130u8,130u8,203u8]);
    assert(crate::witness_m6_m4_filter::expected_filter_1_ct4_d8().subrange(72,96)==seq![37u8,37u8,37u8,69u8,39u8,39u8,39u8,198u8,136u8,136u8,136u8,234u8,13u8,13u8,13u8,255u8,77u8,77u8,77u8,218u8,130u8,130u8,130u8,203u8]) by (compute);
    assert forall|y: nat| y<4 implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_1_ct4_d8(),r::line(raw_filter_1_ct4_d8(),a_filter_1_ct4_d8(),0,y),6nat)==crate::witness_m6_m4_filter::expected_filter_1_ct4_d8().subrange((24*y) as int,(24*(y+1)) as int) by {
        if y==0 {} else if y==1 {} else if y==2 {} else {assert(y==3);}
    }
}
#[verifier::spinoff_prover]
pub(crate) proof fn e6_complete_filter_1_ct4_d8()
    ensures match crate::still_model::MS(crate::witness_m6_m4_filter::bytes_filter_1_ct4_d8()) {crate::still_model::Outcome::Done(value)=>crate::witness_m6_m4_filter::expected_value_filter_1_ct4_d8(value),_=>false},
{

    hide(bytes_filter_1_ct4_d8);hide(z_filter_1_ct4_d8);hide(raw_filter_1_ct4_d8);hide(r::data);hide(crate::witness_m6_m4_filter::expected_filter_1_ct4_d8);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_1_ct4_d8().len()==120nat) by (compute);
    assert(crate::model::mh(bytes_filter_1_ct4_d8())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:4u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_1_ct4_d8())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:8u8,color_type:4u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_1_ct4_d8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:63nat,next:108nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_1_ct4_d8(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:63nat,next:108nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_1_ct4_d8(),108nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:116nat,len:0nat,next:120nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_1_ct4_d8(),108nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:116nat,len:0nat,next:120nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_1_ct4_d8().subrange(41,104)==z_filter_1_ct4_d8()) by (compute);
    walk_filter_1_ct4_d8(bytes_filter_1_ct4_d8());

    model_filter_1_ct4_d8();
    inflate_filter_1_ct4_d8();scan_filter_1_ct4_d8();
    assert(bytes_filter_1_ct4_d8()==crate::witness_m6_m4_filter::bytes_filter_1_ct4_d8()) by (compute);
    assert(r::raw(a_filter_1_ct4_d8())==52nat) by (compute);
    assert(r::data(z_filter_1_ct4_d8(),a_filter_1_ct4_d8(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    assert(crate::model::valid_depth(4u8,8u8)) by (compute);
    crate::witness_m6_rows_model::bare_colour(bytes_filter_1_ct4_d8(),h_filter_1_ct4_d8(),63nat);
    e6_rows_filter_1_ct4_d8();
    assert(crate::witness_m6_m4_filter::expected_filter_1_ct4_d8().len()==96nat) by (compute);
    crate::witness_m6_rows_model::complete_still(bytes_filter_1_ct4_d8(),h_filter_1_ct4_d8(),z_filter_1_ct4_d8(),raw_filter_1_ct4_d8(),e6_colour_filter_1_ct4_d8(),crate::witness_m6_m4_filter::expected_filter_1_ct4_d8());
}
spec fn e6_colour_filter_4_ct6_d16() -> crate::colour_model::Context {(crate::colour_model::Context {ct:6u8,depth:16u8,plte:Seq::empty(),trns:None})}
spec fn e6_line_filter_4_ct6_d16_0() -> Seq<u8> {seq![52u8,83u8,111u8,111u8,23u8,132u8,165u8,198u8,236u8,77u8,85u8,238u8,198u8,138u8,24u8,0u8,85u8,191u8,3u8,36u8,168u8,105u8,59u8,222u8,2u8,187u8,218u8,78u8,76u8,185u8,124u8,247u8,118u8,106u8,172u8,166u8,22u8,102u8,75u8,106u8,165u8,29u8,75u8,180u8,109u8,103u8,101u8,188u8]}
spec fn e6_line_filter_4_ct6_d16_1() -> Seq<u8> {seq![93u8,19u8,63u8,204u8,120u8,140u8,179u8,134u8,118u8,150u8,13u8,75u8,217u8,178u8,235u8,200u8,102u8,217u8,83u8,107u8,191u8,149u8,190u8,146u8,97u8,187u8,249u8,118u8,63u8,155u8,107u8,124u8,123u8,185u8,219u8,242u8,73u8,195u8,3u8,166u8,224u8,88u8,128u8,199u8,153u8,73u8,24u8,36u8]}
spec fn e6_line_filter_4_ct6_d16_2() -> Seq<u8> {seq![39u8,202u8,168u8,184u8,104u8,6u8,254u8,1u8,69u8,205u8,57u8,234u8,134u8,7u8,65u8,158u8,84u8,131u8,139u8,101u8,21u8,158u8,55u8,237u8,97u8,183u8,111u8,117u8,61u8,39u8,33u8,226u8,169u8,242u8,76u8,84u8,28u8,191u8,10u8,66u8,55u8,135u8,182u8,72u8,93u8,250u8,216u8,93u8]}
spec fn e6_line_filter_4_ct6_d16_3() -> Seq<u8> {seq![186u8,101u8,196u8,197u8,42u8,174u8,132u8,144u8,214u8,78u8,211u8,246u8,44u8,247u8,40u8,47u8,148u8,62u8,13u8,207u8,21u8,15u8,49u8,133u8,35u8,45u8,126u8,7u8,80u8,180u8,9u8,36u8,246u8,204u8,168u8,128u8,222u8,36u8,21u8,31u8,198u8,150u8,248u8,44u8,253u8,41u8,224u8,113u8]}
#[verifier::spinoff_prover]
proof fn e6_rows_filter_4_ct6_d16()
    ensures forall|y: nat| y<4 ==> #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),r::line(raw_filter_4_ct6_d16(),a_filter_4_ct6_d16(),0,y),6nat)==crate::witness_m6_m4_filter::expected_filter_4_ct6_d16().subrange((24*y) as int,(24*(y+1)) as int),
{
    hide(raw_filter_4_ct6_d16);hide(crate::colour_model::pixel);hide(crate::witness_m6_m4_filter::expected_filter_4_ct6_d16);
    crate::colour_model::depth_values(16u8);
    assert(r::base(a_filter_4_ct6_d16(),0)==0nat && r::rb(a_filter_4_ct6_d16(),0)==48nat && crate::filter_model::bpp(r::bits(a_filter_4_ct6_d16()))==8nat) by (compute);
    crate::filter_model::pass_row_equation(raw_filter_4_ct6_d16(),0nat,48nat,8nat,0nat);
    assert(raw_filter_4_ct6_d16()[0]==4u8) by (compute);
    assert(crate::filter_model::row_data(raw_filter_4_ct6_d16(),0,48nat,0nat)==seq![52u8,83u8,111u8,111u8,23u8,132u8,165u8,198u8,184u8,250u8,230u8,127u8,175u8,6u8,115u8,58u8,105u8,114u8,174u8,54u8,226u8,223u8,35u8,222u8,173u8,252u8,215u8,42u8,164u8,80u8,65u8,25u8,116u8,175u8,210u8,88u8,202u8,173u8,207u8,115u8,47u8,179u8,159u8,14u8,87u8,1u8,26u8,82u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_filter_4_ct6_d16(),0,48nat,8nat,0nat)==None);
    assert(crate::filter_model::encoded_row(4u8,seq![52u8,83u8,111u8,111u8,23u8,132u8,165u8,198u8,184u8,250u8,230u8,127u8,175u8,6u8,115u8,58u8,105u8,114u8,174u8,54u8,226u8,223u8,35u8,222u8,173u8,252u8,215u8,42u8,164u8,80u8,65u8,25u8,116u8,175u8,210u8,88u8,202u8,173u8,207u8,115u8,47u8,179u8,159u8,14u8,87u8,1u8,26u8,82u8],None,8nat,seq![52u8,83u8,111u8,111u8,23u8,132u8,165u8,198u8,236u8,77u8,85u8,238u8,198u8,138u8,24u8,0u8,85u8,191u8,3u8,36u8,168u8,105u8,59u8,222u8,2u8,187u8,218u8,78u8,76u8,185u8,124u8,247u8,118u8,106u8,172u8,166u8,22u8,102u8,75u8,106u8,165u8,29u8,75u8,180u8,109u8,103u8,101u8,188u8],0,48nat)) by (compute);
    crate::filter_model::checked_row_hint(4u8,seq![52u8,83u8,111u8,111u8,23u8,132u8,165u8,198u8,184u8,250u8,230u8,127u8,175u8,6u8,115u8,58u8,105u8,114u8,174u8,54u8,226u8,223u8,35u8,222u8,173u8,252u8,215u8,42u8,164u8,80u8,65u8,25u8,116u8,175u8,210u8,88u8,202u8,173u8,207u8,115u8,47u8,179u8,159u8,14u8,87u8,1u8,26u8,82u8],None,8nat,seq![52u8,83u8,111u8,111u8,23u8,132u8,165u8,198u8,236u8,77u8,85u8,238u8,198u8,138u8,24u8,0u8,85u8,191u8,3u8,36u8,168u8,105u8,59u8,222u8,2u8,187u8,218u8,78u8,76u8,185u8,124u8,247u8,118u8,106u8,172u8,166u8,22u8,102u8,75u8,106u8,165u8,29u8,75u8,180u8,109u8,103u8,101u8,188u8]);
    assert(r::line(raw_filter_4_ct6_d16(),a_filter_4_ct6_d16(),0,0nat)==seq![52u8,83u8,111u8,111u8,23u8,132u8,165u8,198u8,236u8,77u8,85u8,238u8,198u8,138u8,24u8,0u8,85u8,191u8,3u8,36u8,168u8,105u8,59u8,222u8,2u8,187u8,218u8,78u8,76u8,185u8,124u8,247u8,118u8,106u8,172u8,166u8,22u8,102u8,75u8,106u8,165u8,29u8,75u8,180u8,109u8,103u8,101u8,188u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_0(),6nat)[0]==52u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_0(),6nat)[1]==111u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_0(),6nat)[2]==23u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_0(),6nat)[3]==165u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_0(),6nat)[4]==235u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_0(),6nat)[5]==86u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_0(),6nat)[6]==198u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_0(),6nat)[7]==24u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_0(),6nat)[8]==85u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_0(),6nat)[9]==3u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_0(),6nat)[10]==168u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_0(),6nat)[11]==60u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_0(),6nat)[12]==3u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_0(),6nat)[13]==217u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_0(),6nat)[14]==76u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_0(),6nat)[15]==124u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_0(),6nat)[16]==118u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_0(),6nat)[17]==172u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_0(),6nat)[18]==22u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_0(),6nat)[19]==75u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_0(),6nat)[20]==164u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_0(),6nat)[21]==75u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_0(),6nat)[22]==109u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_0(),6nat)[23]==101u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_0(),6nat),seq![52u8,111u8,23u8,165u8,235u8,86u8,198u8,24u8,85u8,3u8,168u8,60u8,3u8,217u8,76u8,124u8,118u8,172u8,22u8,75u8,164u8,75u8,109u8,101u8]);
    assert(crate::witness_m6_m4_filter::expected_filter_4_ct6_d16().subrange(0,24)==seq![52u8,111u8,23u8,165u8,235u8,86u8,198u8,24u8,85u8,3u8,168u8,60u8,3u8,217u8,76u8,124u8,118u8,172u8,22u8,75u8,164u8,75u8,109u8,101u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_filter_4_ct6_d16(),0nat,48nat,8nat,1nat);
    assert(raw_filter_4_ct6_d16()[49]==4u8) by (compute);
    assert(crate::filter_model::row_data(raw_filter_4_ct6_d16(),0,48nat,1nat)==seq![41u8,192u8,208u8,93u8,97u8,8u8,14u8,192u8,138u8,131u8,206u8,93u8,19u8,38u8,211u8,200u8,17u8,26u8,80u8,71u8,249u8,11u8,211u8,180u8,95u8,226u8,31u8,11u8,243u8,226u8,173u8,234u8,5u8,79u8,1u8,76u8,51u8,93u8,184u8,60u8,59u8,238u8,53u8,213u8,44u8,134u8,21u8,104u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_filter_4_ct6_d16(),0,48nat,8nat,1nat)==Some(seq![52u8,83u8,111u8,111u8,23u8,132u8,165u8,198u8,236u8,77u8,85u8,238u8,198u8,138u8,24u8,0u8,85u8,191u8,3u8,36u8,168u8,105u8,59u8,222u8,2u8,187u8,218u8,78u8,76u8,185u8,124u8,247u8,118u8,106u8,172u8,166u8,22u8,102u8,75u8,106u8,165u8,29u8,75u8,180u8,109u8,103u8,101u8,188u8]));
    assert(crate::filter_model::encoded_row(4u8,seq![41u8,192u8,208u8,93u8,97u8,8u8,14u8,192u8,138u8,131u8,206u8,93u8,19u8,38u8,211u8,200u8,17u8,26u8,80u8,71u8,249u8,11u8,211u8,180u8,95u8,226u8,31u8,11u8,243u8,226u8,173u8,234u8,5u8,79u8,1u8,76u8,51u8,93u8,184u8,60u8,59u8,238u8,53u8,213u8,44u8,134u8,21u8,104u8],Some(seq![52u8,83u8,111u8,111u8,23u8,132u8,165u8,198u8,236u8,77u8,85u8,238u8,198u8,138u8,24u8,0u8,85u8,191u8,3u8,36u8,168u8,105u8,59u8,222u8,2u8,187u8,218u8,78u8,76u8,185u8,124u8,247u8,118u8,106u8,172u8,166u8,22u8,102u8,75u8,106u8,165u8,29u8,75u8,180u8,109u8,103u8,101u8,188u8]),8nat,seq![93u8,19u8,63u8,204u8,120u8,140u8,179u8,134u8,118u8,150u8,13u8,75u8,217u8,178u8,235u8,200u8,102u8,217u8,83u8,107u8,191u8,149u8,190u8,146u8,97u8,187u8,249u8,118u8,63u8,155u8,107u8,124u8,123u8,185u8,219u8,242u8,73u8,195u8,3u8,166u8,224u8,88u8,128u8,199u8,153u8,73u8,24u8,36u8],0,48nat)) by (compute);
    crate::filter_model::checked_row_hint(4u8,seq![41u8,192u8,208u8,93u8,97u8,8u8,14u8,192u8,138u8,131u8,206u8,93u8,19u8,38u8,211u8,200u8,17u8,26u8,80u8,71u8,249u8,11u8,211u8,180u8,95u8,226u8,31u8,11u8,243u8,226u8,173u8,234u8,5u8,79u8,1u8,76u8,51u8,93u8,184u8,60u8,59u8,238u8,53u8,213u8,44u8,134u8,21u8,104u8],Some(seq![52u8,83u8,111u8,111u8,23u8,132u8,165u8,198u8,236u8,77u8,85u8,238u8,198u8,138u8,24u8,0u8,85u8,191u8,3u8,36u8,168u8,105u8,59u8,222u8,2u8,187u8,218u8,78u8,76u8,185u8,124u8,247u8,118u8,106u8,172u8,166u8,22u8,102u8,75u8,106u8,165u8,29u8,75u8,180u8,109u8,103u8,101u8,188u8]),8nat,seq![93u8,19u8,63u8,204u8,120u8,140u8,179u8,134u8,118u8,150u8,13u8,75u8,217u8,178u8,235u8,200u8,102u8,217u8,83u8,107u8,191u8,149u8,190u8,146u8,97u8,187u8,249u8,118u8,63u8,155u8,107u8,124u8,123u8,185u8,219u8,242u8,73u8,195u8,3u8,166u8,224u8,88u8,128u8,199u8,153u8,73u8,24u8,36u8]);
    assert(r::line(raw_filter_4_ct6_d16(),a_filter_4_ct6_d16(),0,1nat)==seq![93u8,19u8,63u8,204u8,120u8,140u8,179u8,134u8,118u8,150u8,13u8,75u8,217u8,178u8,235u8,200u8,102u8,217u8,83u8,107u8,191u8,149u8,190u8,146u8,97u8,187u8,249u8,118u8,63u8,155u8,107u8,124u8,123u8,185u8,219u8,242u8,73u8,195u8,3u8,166u8,224u8,88u8,128u8,199u8,153u8,73u8,24u8,36u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_1(),6nat)[0]==93u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_1(),6nat)[1]==64u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_1(),6nat)[2]==120u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_1(),6nat)[3]==179u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_1(),6nat)[4]==118u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_1(),6nat)[5]==13u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_1(),6nat)[6]==217u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_1(),6nat)[7]==235u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_1(),6nat)[8]==102u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_1(),6nat)[9]==83u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_1(),6nat)[10]==191u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_1(),6nat)[11]==190u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_1(),6nat)[12]==97u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_1(),6nat)[13]==248u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_1(),6nat)[14]==63u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_1(),6nat)[15]==107u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_1(),6nat)[16]==123u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_1(),6nat)[17]==219u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_1(),6nat)[18]==73u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_1(),6nat)[19]==4u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_1(),6nat)[20]==223u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_1(),6nat)[21]==128u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_1(),6nat)[22]==153u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_1(),6nat)[23]==24u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_1(),6nat),seq![93u8,64u8,120u8,179u8,118u8,13u8,217u8,235u8,102u8,83u8,191u8,190u8,97u8,248u8,63u8,107u8,123u8,219u8,73u8,4u8,223u8,128u8,153u8,24u8]);
    assert(crate::witness_m6_m4_filter::expected_filter_4_ct6_d16().subrange(24,48)==seq![93u8,64u8,120u8,179u8,118u8,13u8,217u8,235u8,102u8,83u8,191u8,190u8,97u8,248u8,63u8,107u8,123u8,219u8,73u8,4u8,223u8,128u8,153u8,24u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_filter_4_ct6_d16(),0nat,48nat,8nat,2nat);
    assert(raw_filter_4_ct6_d16()[98]==4u8) by (compute);
    assert(crate::filter_model::row_data(raw_filter_4_ct6_d16(),0,48nat,2nat)==seq![202u8,183u8,105u8,236u8,240u8,122u8,75u8,123u8,30u8,3u8,145u8,159u8,173u8,1u8,67u8,157u8,15u8,170u8,56u8,123u8,143u8,151u8,246u8,91u8,13u8,52u8,118u8,10u8,40u8,137u8,234u8,245u8,46u8,59u8,221u8,98u8,211u8,152u8,7u8,96u8,87u8,206u8,106u8,244u8,20u8,177u8,192u8,57u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_filter_4_ct6_d16(),0,48nat,8nat,2nat)==Some(seq![93u8,19u8,63u8,204u8,120u8,140u8,179u8,134u8,118u8,150u8,13u8,75u8,217u8,178u8,235u8,200u8,102u8,217u8,83u8,107u8,191u8,149u8,190u8,146u8,97u8,187u8,249u8,118u8,63u8,155u8,107u8,124u8,123u8,185u8,219u8,242u8,73u8,195u8,3u8,166u8,224u8,88u8,128u8,199u8,153u8,73u8,24u8,36u8]));
    assert(crate::filter_model::encoded_row(4u8,seq![202u8,183u8,105u8,236u8,240u8,122u8,75u8,123u8,30u8,3u8,145u8,159u8,173u8,1u8,67u8,157u8,15u8,170u8,56u8,123u8,143u8,151u8,246u8,91u8,13u8,52u8,118u8,10u8,40u8,137u8,234u8,245u8,46u8,59u8,221u8,98u8,211u8,152u8,7u8,96u8,87u8,206u8,106u8,244u8,20u8,177u8,192u8,57u8],Some(seq![93u8,19u8,63u8,204u8,120u8,140u8,179u8,134u8,118u8,150u8,13u8,75u8,217u8,178u8,235u8,200u8,102u8,217u8,83u8,107u8,191u8,149u8,190u8,146u8,97u8,187u8,249u8,118u8,63u8,155u8,107u8,124u8,123u8,185u8,219u8,242u8,73u8,195u8,3u8,166u8,224u8,88u8,128u8,199u8,153u8,73u8,24u8,36u8]),8nat,seq![39u8,202u8,168u8,184u8,104u8,6u8,254u8,1u8,69u8,205u8,57u8,234u8,134u8,7u8,65u8,158u8,84u8,131u8,139u8,101u8,21u8,158u8,55u8,237u8,97u8,183u8,111u8,117u8,61u8,39u8,33u8,226u8,169u8,242u8,76u8,84u8,28u8,191u8,10u8,66u8,55u8,135u8,182u8,72u8,93u8,250u8,216u8,93u8],0,48nat)) by (compute);
    crate::filter_model::checked_row_hint(4u8,seq![202u8,183u8,105u8,236u8,240u8,122u8,75u8,123u8,30u8,3u8,145u8,159u8,173u8,1u8,67u8,157u8,15u8,170u8,56u8,123u8,143u8,151u8,246u8,91u8,13u8,52u8,118u8,10u8,40u8,137u8,234u8,245u8,46u8,59u8,221u8,98u8,211u8,152u8,7u8,96u8,87u8,206u8,106u8,244u8,20u8,177u8,192u8,57u8],Some(seq![93u8,19u8,63u8,204u8,120u8,140u8,179u8,134u8,118u8,150u8,13u8,75u8,217u8,178u8,235u8,200u8,102u8,217u8,83u8,107u8,191u8,149u8,190u8,146u8,97u8,187u8,249u8,118u8,63u8,155u8,107u8,124u8,123u8,185u8,219u8,242u8,73u8,195u8,3u8,166u8,224u8,88u8,128u8,199u8,153u8,73u8,24u8,36u8]),8nat,seq![39u8,202u8,168u8,184u8,104u8,6u8,254u8,1u8,69u8,205u8,57u8,234u8,134u8,7u8,65u8,158u8,84u8,131u8,139u8,101u8,21u8,158u8,55u8,237u8,97u8,183u8,111u8,117u8,61u8,39u8,33u8,226u8,169u8,242u8,76u8,84u8,28u8,191u8,10u8,66u8,55u8,135u8,182u8,72u8,93u8,250u8,216u8,93u8]);
    assert(r::line(raw_filter_4_ct6_d16(),a_filter_4_ct6_d16(),0,2nat)==seq![39u8,202u8,168u8,184u8,104u8,6u8,254u8,1u8,69u8,205u8,57u8,234u8,134u8,7u8,65u8,158u8,84u8,131u8,139u8,101u8,21u8,158u8,55u8,237u8,97u8,183u8,111u8,117u8,61u8,39u8,33u8,226u8,169u8,242u8,76u8,84u8,28u8,191u8,10u8,66u8,55u8,135u8,182u8,72u8,93u8,250u8,216u8,93u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_2(),6nat)[0]==40u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_2(),6nat)[1]==168u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_2(),6nat)[2]==104u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_2(),6nat)[3]==253u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_2(),6nat)[4]==70u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_2(),6nat)[5]==58u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_2(),6nat)[6]==134u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_2(),6nat)[7]==65u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_2(),6nat)[8]==84u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_2(),6nat)[9]==139u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_2(),6nat)[10]==22u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_2(),6nat)[11]==56u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_2(),6nat)[12]==97u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_2(),6nat)[13]==111u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_2(),6nat)[14]==61u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_2(),6nat)[15]==34u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_2(),6nat)[16]==169u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_2(),6nat)[17]==76u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_2(),6nat)[18]==29u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_2(),6nat)[19]==10u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_2(),6nat)[20]==55u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_2(),6nat)[21]==182u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_2(),6nat)[22]==94u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_2(),6nat)[23]==216u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_2(),6nat),seq![40u8,168u8,104u8,253u8,70u8,58u8,134u8,65u8,84u8,139u8,22u8,56u8,97u8,111u8,61u8,34u8,169u8,76u8,29u8,10u8,55u8,182u8,94u8,216u8]);
    assert(crate::witness_m6_m4_filter::expected_filter_4_ct6_d16().subrange(48,72)==seq![40u8,168u8,104u8,253u8,70u8,58u8,134u8,65u8,84u8,139u8,22u8,56u8,97u8,111u8,61u8,34u8,169u8,76u8,29u8,10u8,55u8,182u8,94u8,216u8]) by (compute);
    crate::filter_model::pass_row_equation(raw_filter_4_ct6_d16(),0nat,48nat,8nat,3nat);
    assert(raw_filter_4_ct6_d16()[147]==4u8) by (compute);
    assert(crate::filter_model::row_data(raw_filter_4_ct6_d16(),0,48nat,3nat)==seq![147u8,155u8,28u8,13u8,194u8,168u8,134u8,143u8,28u8,233u8,154u8,12u8,2u8,73u8,231u8,145u8,190u8,240u8,58u8,106u8,0u8,24u8,9u8,231u8,143u8,170u8,113u8,56u8,19u8,165u8,232u8,159u8,149u8,159u8,92u8,121u8,161u8,101u8,12u8,251u8,29u8,15u8,66u8,172u8,31u8,5u8,8u8,47u8]) by (compute);
    assert(crate::filter_model::previous_row(raw_filter_4_ct6_d16(),0,48nat,8nat,3nat)==Some(seq![39u8,202u8,168u8,184u8,104u8,6u8,254u8,1u8,69u8,205u8,57u8,234u8,134u8,7u8,65u8,158u8,84u8,131u8,139u8,101u8,21u8,158u8,55u8,237u8,97u8,183u8,111u8,117u8,61u8,39u8,33u8,226u8,169u8,242u8,76u8,84u8,28u8,191u8,10u8,66u8,55u8,135u8,182u8,72u8,93u8,250u8,216u8,93u8]));
    assert(crate::filter_model::encoded_row(4u8,seq![147u8,155u8,28u8,13u8,194u8,168u8,134u8,143u8,28u8,233u8,154u8,12u8,2u8,73u8,231u8,145u8,190u8,240u8,58u8,106u8,0u8,24u8,9u8,231u8,143u8,170u8,113u8,56u8,19u8,165u8,232u8,159u8,149u8,159u8,92u8,121u8,161u8,101u8,12u8,251u8,29u8,15u8,66u8,172u8,31u8,5u8,8u8,47u8],Some(seq![39u8,202u8,168u8,184u8,104u8,6u8,254u8,1u8,69u8,205u8,57u8,234u8,134u8,7u8,65u8,158u8,84u8,131u8,139u8,101u8,21u8,158u8,55u8,237u8,97u8,183u8,111u8,117u8,61u8,39u8,33u8,226u8,169u8,242u8,76u8,84u8,28u8,191u8,10u8,66u8,55u8,135u8,182u8,72u8,93u8,250u8,216u8,93u8]),8nat,seq![186u8,101u8,196u8,197u8,42u8,174u8,132u8,144u8,214u8,78u8,211u8,246u8,44u8,247u8,40u8,47u8,148u8,62u8,13u8,207u8,21u8,15u8,49u8,133u8,35u8,45u8,126u8,7u8,80u8,180u8,9u8,36u8,246u8,204u8,168u8,128u8,222u8,36u8,21u8,31u8,198u8,150u8,248u8,44u8,253u8,41u8,224u8,113u8],0,48nat)) by (compute);
    crate::filter_model::checked_row_hint(4u8,seq![147u8,155u8,28u8,13u8,194u8,168u8,134u8,143u8,28u8,233u8,154u8,12u8,2u8,73u8,231u8,145u8,190u8,240u8,58u8,106u8,0u8,24u8,9u8,231u8,143u8,170u8,113u8,56u8,19u8,165u8,232u8,159u8,149u8,159u8,92u8,121u8,161u8,101u8,12u8,251u8,29u8,15u8,66u8,172u8,31u8,5u8,8u8,47u8],Some(seq![39u8,202u8,168u8,184u8,104u8,6u8,254u8,1u8,69u8,205u8,57u8,234u8,134u8,7u8,65u8,158u8,84u8,131u8,139u8,101u8,21u8,158u8,55u8,237u8,97u8,183u8,111u8,117u8,61u8,39u8,33u8,226u8,169u8,242u8,76u8,84u8,28u8,191u8,10u8,66u8,55u8,135u8,182u8,72u8,93u8,250u8,216u8,93u8]),8nat,seq![186u8,101u8,196u8,197u8,42u8,174u8,132u8,144u8,214u8,78u8,211u8,246u8,44u8,247u8,40u8,47u8,148u8,62u8,13u8,207u8,21u8,15u8,49u8,133u8,35u8,45u8,126u8,7u8,80u8,180u8,9u8,36u8,246u8,204u8,168u8,128u8,222u8,36u8,21u8,31u8,198u8,150u8,248u8,44u8,253u8,41u8,224u8,113u8]);
    assert(r::line(raw_filter_4_ct6_d16(),a_filter_4_ct6_d16(),0,3nat)==seq![186u8,101u8,196u8,197u8,42u8,174u8,132u8,144u8,214u8,78u8,211u8,246u8,44u8,247u8,40u8,47u8,148u8,62u8,13u8,207u8,21u8,15u8,49u8,133u8,35u8,45u8,126u8,7u8,80u8,180u8,9u8,36u8,246u8,204u8,168u8,128u8,222u8,36u8,21u8,31u8,198u8,150u8,248u8,44u8,253u8,41u8,224u8,113u8]);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_3(),6nat)[0]==186u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_3(),6nat)[1]==196u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_3(),6nat)[2]==43u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_3(),6nat)[3]==132u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_3(),6nat)[4]==213u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_3(),6nat)[5]==211u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_3(),6nat)[6]==45u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_3(),6nat)[7]==40u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_3(),6nat)[8]==148u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_3(),6nat)[9]==14u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_3(),6nat)[10]==21u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_3(),6nat)[11]==49u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_3(),6nat)[12]==35u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_3(),6nat)[13]==126u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_3(),6nat)[14]==80u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_3(),6nat)[15]==9u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_3(),6nat)[16]==246u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_3(),6nat)[17]==168u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_3(),6nat)[18]==221u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_3(),6nat)[19]==21u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_3(),6nat)[20]==198u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_3(),6nat)[21]==247u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_3(),6nat)[22]==252u8) by (compute);
    assert(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_3(),6nat)[23]==224u8) by (compute);
    crate::witness_m6_rows_model::bounded_row_equal(crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),e6_line_filter_4_ct6_d16_3(),6nat),seq![186u8,196u8,43u8,132u8,213u8,211u8,45u8,40u8,148u8,14u8,21u8,49u8,35u8,126u8,80u8,9u8,246u8,168u8,221u8,21u8,198u8,247u8,252u8,224u8]);
    assert(crate::witness_m6_m4_filter::expected_filter_4_ct6_d16().subrange(72,96)==seq![186u8,196u8,43u8,132u8,213u8,211u8,45u8,40u8,148u8,14u8,21u8,49u8,35u8,126u8,80u8,9u8,246u8,168u8,221u8,21u8,198u8,247u8,252u8,224u8]) by (compute);
    assert forall|y: nat| y<4 implies #[trigger] crate::witness_m6_rows_model::row_bytes8(e6_colour_filter_4_ct6_d16(),r::line(raw_filter_4_ct6_d16(),a_filter_4_ct6_d16(),0,y),6nat)==crate::witness_m6_m4_filter::expected_filter_4_ct6_d16().subrange((24*y) as int,(24*(y+1)) as int) by {
        if y==0 {} else if y==1 {} else if y==2 {} else {assert(y==3);}
    }
}
#[verifier::spinoff_prover]
pub(crate) proof fn e6_complete_filter_4_ct6_d16()
    ensures match crate::still_model::MS(crate::witness_m6_m4_filter::bytes_filter_4_ct6_d16()) {crate::still_model::Outcome::Done(value)=>crate::witness_m6_m4_filter::expected_value_filter_4_ct6_d16(value),_=>false},
{

    hide(bytes_filter_4_ct6_d16);hide(z_filter_4_ct6_d16);hide(raw_filter_4_ct6_d16);hide(r::data);hide(crate::witness_m6_m4_filter::expected_filter_4_ct6_d16);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(crate::document_model::drive);hide(crate::document_model::initial_info);hide(crate::walk_model::classify);
    assert(bytes_filter_4_ct6_d16().len()==264nat) by (compute);
    assert(crate::model::mh(bytes_filter_4_ct6_d16())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:6u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_filter_4_ct6_d16())==Ok((crate::model::Ihdr {width:6u32,height:4u32,depth:16u8,color_type:6u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_4_ct6_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:207nat,next:252nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_4_ct6_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:207nat,next:252nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_filter_4_ct6_d16(),252nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:260nat,len:0nat,next:264nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_filter_4_ct6_d16(),252nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:260nat,len:0nat,next:264nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_filter_4_ct6_d16().subrange(41,248)==z_filter_4_ct6_d16()) by (compute);
    walk_filter_4_ct6_d16(bytes_filter_4_ct6_d16());

    model_filter_4_ct6_d16();
    inflate_filter_4_ct6_d16();scan_filter_4_ct6_d16();
    assert(bytes_filter_4_ct6_d16()==crate::witness_m6_m4_filter::bytes_filter_4_ct6_d16()) by (compute);
    assert(r::raw(a_filter_4_ct6_d16())==196nat) by (compute);
    assert(r::data(z_filter_4_ct6_d16(),a_filter_4_ct6_d16(),STRICT)==Some(Ok(()))) by {reveal(r::data);}
    assert(crate::model::valid_depth(6u8,16u8)) by (compute);
    crate::witness_m6_rows_model::bare_colour(bytes_filter_4_ct6_d16(),h_filter_4_ct6_d16(),207nat);
    e6_rows_filter_4_ct6_d16();
    assert(crate::witness_m6_m4_filter::expected_filter_4_ct6_d16().len()==96nat) by (compute);
    crate::witness_m6_rows_model::complete_still(bytes_filter_4_ct6_d16(),h_filter_4_ct6_d16(),z_filter_4_ct6_d16(),raw_filter_4_ct6_d16(),e6_colour_filter_4_ct6_d16(),crate::witness_m6_m4_filter::expected_filter_4_ct6_d16());
}
} // verus!
