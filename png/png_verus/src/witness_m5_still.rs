//! M5 data/shape projections and complete bad-index errors; pixel results deferred by E6.
use vstd::prelude::*;
use crate::{document_model as m,raster_model as r,filter_model as f,png::Error,policy::STRICT,
    m4_trace_witness_model as t,m4_checksum_witness_model as c,m4_stream_witness_model as stream,m5_stream_witness_model as ms,m4_crc_witness_model as crc_cert};
use inflate_verus::model as zm;
verus! {
// ok16/ct0-d1-il0-7x5; frozen SHA256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_ok16_ct0_d1_il0_7x5() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,7u8,0u8,0u8,0u8,5u8,1u8,0u8,0u8,0u8,0u8,161u8,225u8,203u8,117u8,0u8,0u8,0u8,18u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,240u8,99u8,156u8,196u8,244u8,128u8,89u8,157u8,229u8,27u8,0u8,12u8,185u8,2u8,232u8,207u8,146u8,250u8,145u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn h_ok16_ct0_d1_il0_7x5() -> crate::model::Ihdr {crate::model::Ihdr {width:7u32,height:5u32,depth:1u8,color_type:0u8,interlace:0u8}}
spec fn a_ok16_ct0_d1_il0_7x5() -> r::Raster {r::shape(h_ok16_ct0_d1_il0_7x5(),7nat,5nat,0nat,STRICT)}
spec fn z_ok16_ct0_d1_il0_7x5() -> Seq<u8> {seq![120u8,156u8,99u8,240u8,99u8,156u8,196u8,244u8,128u8,89u8,157u8,229u8,27u8,0u8,12u8,185u8,2u8,232u8]}
spec fn raw_ok16_ct0_d1_il0_7x5() -> Seq<u8> {seq![0u8,78u8,1u8,146u8,2u8,224u8,3u8,39u8,4u8,246u8]}
spec fn ts_ok16_ct0_d1_il0_7x5() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:35nat,before:1nat,after:2nat,symbol:78nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:35nat,next:43nat,before:2nat,after:3nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:43nat,next:52nat,before:3nat,after:4nat,symbol:146nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:52nat,next:60nat,before:4nat,after:5nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:60nat,next:69nat,before:5nat,after:6nat,symbol:224nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:69nat,next:77nat,before:6nat,after:7nat,symbol:3nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:77nat,next:85nat,before:7nat,after:8nat,symbol:39nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:85nat,next:93nat,before:8nat,after:9nat,symbol:4nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:93nat,next:102nat,before:9nat,after:10nat,symbol:246nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:102nat,next:109nat,before:10nat,after:10nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_ok16_ct0_d1_il0_7x5_0() ensures t::trace(z_ok16_ct0_d1_il0_7x5(),raw_ok16_ct0_d1_il0_7x5(),ts_ok16_ct0_d1_il0_7x5(),0nat,11nat),
{
    hide(t::valid_step);assert(t::trace(z_ok16_ct0_d1_il0_7x5(),raw_ok16_ct0_d1_il0_7x5(),ts_ok16_ct0_d1_il0_7x5(),0nat,11nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn trace_ok16_ct0_d1_il0_7x5() ensures t::trace(z_ok16_ct0_d1_il0_7x5(),raw_ok16_ct0_d1_il0_7x5(),ts_ok16_ct0_d1_il0_7x5(),0,11nat),
{
    hide(z_ok16_ct0_d1_il0_7x5);hide(raw_ok16_ct0_d1_il0_7x5);hide(ts_ok16_ct0_d1_il0_7x5);hide(t::trace);
    trace_ok16_ct0_d1_il0_7x5_0();
    assert(t::trace(z_ok16_ct0_d1_il0_7x5(),raw_ok16_ct0_d1_il0_7x5(),ts_ok16_ct0_d1_il0_7x5(),0,11nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_ok16_ct0_d1_il0_7x5() ensures z_ok16_ct0_d1_il0_7x5().len()==18nat,
        raw_ok16_ct0_d1_il0_7x5().len()==10nat,
        z_ok16_ct0_d1_il0_7x5()[0]==120u8,
        z_ok16_ct0_d1_il0_7x5()[1]==156u8,
        zm::field_value(z_ok16_ct0_d1_il0_7x5(),16,1)==1nat,
        zm::field_value(z_ok16_ct0_d1_il0_7x5(),17,2)==1nat,
        zm::be_value(z_ok16_ct0_d1_il0_7x5(),14nat,4)==213451496nat,
        ts_ok16_ct0_d1_il0_7x5().len()==11nat,
        ts_ok16_ct0_d1_il0_7x5()[0].pos==19nat,
        ts_ok16_ct0_d1_il0_7x5()[0].before==0nat,
        ts_ok16_ct0_d1_il0_7x5().last().next==109nat,
{
    assert(z_ok16_ct0_d1_il0_7x5().len()==18nat) by (compute);
    assert(raw_ok16_ct0_d1_il0_7x5().len()==10nat) by (compute);
    assert(z_ok16_ct0_d1_il0_7x5()[0]==120u8) by (compute);
    assert(z_ok16_ct0_d1_il0_7x5()[1]==156u8) by (compute);
    assert(zm::field_value(z_ok16_ct0_d1_il0_7x5(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_ok16_ct0_d1_il0_7x5(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_ok16_ct0_d1_il0_7x5(),14nat,4)==213451496nat) by (compute);
    assert(ts_ok16_ct0_d1_il0_7x5().len()==11nat) by (compute);
    assert(ts_ok16_ct0_d1_il0_7x5()[0].pos==19nat) by (compute);
    assert(ts_ok16_ct0_d1_il0_7x5()[0].before==0nat) by (compute);
    assert(ts_ok16_ct0_d1_il0_7x5().last().next==109nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_ok16_ct0_d1_il0_7x5() ensures zm::Adler(raw_ok16_ct0_d1_il0_7x5())==213451496nat,
{
    hide(raw_ok16_ct0_d1_il0_7x5);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_ok16_ct0_d1_il0_7x5().len()==10nat) by (compute);
    assert(c::sums(raw_ok16_ct0_d1_il0_7x5(),0,10nat)==(743nat,3257nat)) by (compute);
    c::sums_correct(raw_ok16_ct0_d1_il0_7x5(),0,10nat);assert(raw_ok16_ct0_d1_il0_7x5().subrange(0,10)=~=raw_ok16_ct0_d1_il0_7x5());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((3257nat%65521)*(u16::MAX as nat+1)+(1+743nat)%65521==213451496nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_ok16_ct0_d1_il0_7x5() ensures zm::Inflate(z_ok16_ct0_d1_il0_7x5(),10nat,true,false)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_ok16_ct0_d1_il0_7x5(),r2:false}),raw_ok16_ct0_d1_il0_7x5().len()==10nat,
{
    hide(z_ok16_ct0_d1_il0_7x5);hide(raw_ok16_ct0_d1_il0_7x5);hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);hide(ts_ok16_ct0_d1_il0_7x5);
    shape_ok16_ct0_d1_il0_7x5();checksum_ok16_ct0_d1_il0_7x5();
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_ok16_ct0_d1_il0_7x5();ms::fixed_stream(z_ok16_ct0_d1_il0_7x5(),raw_ok16_ct0_d1_il0_7x5(),ts_ok16_ct0_d1_il0_7x5(),10nat,156nat,213451496nat,213451496nat);
}
#[verifier::spinoff_prover]
proof fn walk_ok16_ct0_d1_il0_7x5(b: Seq<u8>)
    requires b.len()==75,
        crate::model::mh(b)==Ok((h_ok16_ct0_d1_il0_7x5(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:18nat,next:63nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,63nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:71nat,len:0nat,next:75nat,crc_ok:true,synthetic:false})),
        b.subrange(41,59)==z_ok16_ct0_d1_il0_7x5(),
    ensures r::default_case(b,h_ok16_ct0_d1_il0_7x5(),z_ok16_ct0_d1_il0_7x5(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
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
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    reveal_with_fuel(m::drive,3);
    reveal_with_fuel(m::concat_ranges,3);
}
#[verifier::spinoff_prover]
proof fn scan_ok16_ct0_d1_il0_7x5() ensures r::scan(raw_ok16_ct0_d1_il0_7x5(),a_ok16_ct0_d1_il0_7x5(),0)==Ok(()),
{
    hide(raw_ok16_ct0_d1_il0_7x5);assert(raw_ok16_ct0_d1_il0_7x5().len()==10nat) by (compute);assert(r::count(a_ok16_ct0_d1_il0_7x5())==1nat) by (compute);
    assert(r::pw(a_ok16_ct0_d1_il0_7x5(),0nat)==7nat && r::ph(a_ok16_ct0_d1_il0_7x5(),0nat)==5nat && r::rb(a_ok16_ct0_d1_il0_7x5(),0nat)==1nat && r::base(a_ok16_ct0_d1_il0_7x5(),0nat)==0nat) by (compute);
    assert(raw_ok16_ct0_d1_il0_7x5()[0]==0u8) by (compute);
    assert(r::row_result(raw_ok16_ct0_d1_il0_7x5(),a_ok16_ct0_d1_il0_7x5(),0nat,0nat)==Ok(()));
    assert(raw_ok16_ct0_d1_il0_7x5()[2]==1u8) by (compute);
    assert(r::row_result(raw_ok16_ct0_d1_il0_7x5(),a_ok16_ct0_d1_il0_7x5(),0nat,1nat)==Ok(()));
    assert(raw_ok16_ct0_d1_il0_7x5()[4]==2u8) by (compute);
    assert(r::row_result(raw_ok16_ct0_d1_il0_7x5(),a_ok16_ct0_d1_il0_7x5(),0nat,2nat)==Ok(()));
    assert(raw_ok16_ct0_d1_il0_7x5()[6]==3u8) by (compute);
    assert(r::row_result(raw_ok16_ct0_d1_il0_7x5(),a_ok16_ct0_d1_il0_7x5(),0nat,3nat)==Ok(()));
    assert(raw_ok16_ct0_d1_il0_7x5()[8]==4u8) by (compute);
    assert(r::row_result(raw_ok16_ct0_d1_il0_7x5(),a_ok16_ct0_d1_il0_7x5(),0nat,4nat)==Ok(()));
    r::scan_step(raw_ok16_ct0_d1_il0_7x5(),a_ok16_ct0_d1_il0_7x5(),1nat);
    r::pass_step(raw_ok16_ct0_d1_il0_7x5(),a_ok16_ct0_d1_il0_7x5(),0nat,5nat);
    r::pass_step(raw_ok16_ct0_d1_il0_7x5(),a_ok16_ct0_d1_il0_7x5(),0nat,4nat);
    r::pass_step(raw_ok16_ct0_d1_il0_7x5(),a_ok16_ct0_d1_il0_7x5(),0nat,3nat);
    r::pass_step(raw_ok16_ct0_d1_il0_7x5(),a_ok16_ct0_d1_il0_7x5(),0nat,2nat);
    r::pass_step(raw_ok16_ct0_d1_il0_7x5(),a_ok16_ct0_d1_il0_7x5(),0nat,1nat);
    r::pass_step(raw_ok16_ct0_d1_il0_7x5(),a_ok16_ct0_d1_il0_7x5(),0nat,0nat);
    r::scan_step(raw_ok16_ct0_d1_il0_7x5(),a_ok16_ct0_d1_il0_7x5(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_ok16_ct0_d1_il0_7x5() ensures r::still(bytes_ok16_ct0_d1_il0_7x5(),STRICT,true)==(r::Projection::Ready {width:7u32,height:5u32,bytes:280nat}),
{
    hide(bytes_ok16_ct0_d1_il0_7x5);hide(z_ok16_ct0_d1_il0_7x5);hide(raw_ok16_ct0_d1_il0_7x5);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    assert(bytes_ok16_ct0_d1_il0_7x5().len()==75nat) by (compute);
    assert(crate::model::mh(bytes_ok16_ct0_d1_il0_7x5())==Ok((h_ok16_ct0_d1_il0_7x5(),33nat))) by {reveal(crate::model::mh);assert(crate::model::mh(bytes_ok16_ct0_d1_il0_7x5())==Ok((h_ok16_ct0_d1_il0_7x5(),33nat))) by (compute);}
        assert((1u32 << 1u8)==2u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_ok16_ct0_d1_il0_7x5(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:18nat,next:63nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_ok16_ct0_d1_il0_7x5(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:18nat,next:63nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_ok16_ct0_d1_il0_7x5(),63nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:71nat,len:0nat,next:75nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_ok16_ct0_d1_il0_7x5(),63nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:71nat,len:0nat,next:75nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_ok16_ct0_d1_il0_7x5().subrange(41,59)==z_ok16_ct0_d1_il0_7x5()) by (compute);
    walk_ok16_ct0_d1_il0_7x5(bytes_ok16_ct0_d1_il0_7x5());
    inflate_ok16_ct0_d1_il0_7x5();scan_ok16_ct0_d1_il0_7x5();
    assert(r::raw(a_ok16_ct0_d1_il0_7x5())==10nat) by (compute);
    assert(r::data(z_ok16_ct0_d1_il0_7x5(),a_ok16_ct0_d1_il0_7x5(),STRICT)==Some(Ok(()))) by {reveal(r::data);};
    r::default_ready(bytes_ok16_ct0_d1_il0_7x5(),h_ok16_ct0_d1_il0_7x5(),z_ok16_ct0_d1_il0_7x5(),0nat,true);
}
fn public_ok16_ct0_d1_il0_7x5() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,7u8,0u8,0u8,0u8,5u8,1u8,0u8,0u8,0u8,0u8,161u8,225u8,203u8,117u8,0u8,0u8,0u8,18u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,240u8,99u8,156u8,196u8,244u8,128u8,89u8,157u8,229u8,27u8,0u8,12u8,185u8,2u8,232u8,207u8,146u8,250u8,145u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_ok16_ct0_d1_il0_7x5());model_ok16_ct0_d1_il0_7x5();}
    let result=crate::png::try_decode_still16(&b);
    assert(match result {Ok(s)=>s.width==7u32 && s.height==5u32 && s.rgba16@.len()==280nat,Err(crate::alloc_guard::TryError::OutOfMemory(_))=>true,_=>false});
}
// ok/ct0-d2-il1-7x5; frozen SHA256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_ok_ct0_d2_il1_7x5() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,7u8,0u8,0u8,0u8,5u8,2u8,0u8,0u8,0u8,1u8,145u8,70u8,129u8,51u8,0u8,0u8,0u8,32u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,192u8,200u8,192u8,180u8,129u8,217u8,129u8,37u8,128u8,97u8,55u8,99u8,10u8,211u8,21u8,230u8,19u8,44u8,243u8,132u8,24u8,74u8,66u8,0u8,71u8,173u8,6u8,72u8,235u8,130u8,210u8,19u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn h_ok_ct0_d2_il1_7x5() -> crate::model::Ihdr {crate::model::Ihdr {width:7u32,height:5u32,depth:2u8,color_type:0u8,interlace:1u8}}
spec fn a_ok_ct0_d2_il1_7x5() -> r::Raster {r::shape(h_ok_ct0_d2_il1_7x5(),7nat,5nat,0nat,STRICT)}
spec fn z_ok_ct0_d2_il1_7x5() -> Seq<u8> {seq![120u8,156u8,99u8,56u8,192u8,200u8,192u8,180u8,129u8,217u8,129u8,37u8,128u8,97u8,55u8,99u8,10u8,211u8,21u8,230u8,19u8,44u8,243u8,132u8,24u8,74u8,66u8,0u8,71u8,173u8,6u8,72u8]}
spec fn raw_ok_ct0_d2_il1_7x5() -> Seq<u8> {seq![0u8,192u8,1u8,0u8,2u8,176u8,3u8,64u8,4u8,80u8,0u8,187u8,1u8,100u8,2u8,212u8,3u8,200u8,4u8,158u8,18u8,0u8,116u8,84u8]}
spec fn ts_ok_ct0_d2_il1_7x5() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:36nat,before:1nat,after:2nat,symbol:192nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:36nat,next:44nat,before:2nat,after:3nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:44nat,next:52nat,before:3nat,after:4nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:52nat,next:60nat,before:4nat,after:5nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:60nat,next:69nat,before:5nat,after:6nat,symbol:176nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:69nat,next:77nat,before:6nat,after:7nat,symbol:3nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:77nat,next:85nat,before:7nat,after:8nat,symbol:64nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:85nat,next:93nat,before:8nat,after:9nat,symbol:4nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:93nat,next:101nat,before:9nat,after:10nat,symbol:80nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:101nat,next:109nat,before:10nat,after:11nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:109nat,next:118nat,before:11nat,after:12nat,symbol:187nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:118nat,next:126nat,before:12nat,after:13nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:126nat,next:134nat,before:13nat,after:14nat,symbol:100nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:134nat,next:142nat,before:14nat,after:15nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:142nat,next:151nat,before:15nat,after:16nat,symbol:212nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:151nat,next:159nat,before:16nat,after:17nat,symbol:3nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:159nat,next:168nat,before:17nat,after:18nat,symbol:200nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:168nat,next:176nat,before:18nat,after:19nat,symbol:4nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:176nat,next:185nat,before:19nat,after:20nat,symbol:158nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:185nat,next:193nat,before:20nat,after:21nat,symbol:18nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:193nat,next:201nat,before:21nat,after:22nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:201nat,next:209nat,before:22nat,after:23nat,symbol:116nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:209nat,next:217nat,before:23nat,after:24nat,symbol:84nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:217nat,next:224nat,before:24nat,after:24nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_ok_ct0_d2_il1_7x5_0() ensures t::trace(z_ok_ct0_d2_il1_7x5(),raw_ok_ct0_d2_il1_7x5(),ts_ok_ct0_d2_il1_7x5(),0nat,16nat),
{
    hide(t::valid_step);assert(t::trace(z_ok_ct0_d2_il1_7x5(),raw_ok_ct0_d2_il1_7x5(),ts_ok_ct0_d2_il1_7x5(),0nat,16nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn trace_ok_ct0_d2_il1_7x5_1() ensures t::trace(z_ok_ct0_d2_il1_7x5(),raw_ok_ct0_d2_il1_7x5(),ts_ok_ct0_d2_il1_7x5(),16nat,25nat),
{
    hide(t::valid_step);assert(t::trace(z_ok_ct0_d2_il1_7x5(),raw_ok_ct0_d2_il1_7x5(),ts_ok_ct0_d2_il1_7x5(),16nat,25nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn trace_ok_ct0_d2_il1_7x5() ensures t::trace(z_ok_ct0_d2_il1_7x5(),raw_ok_ct0_d2_il1_7x5(),ts_ok_ct0_d2_il1_7x5(),0,25nat),
{
    hide(z_ok_ct0_d2_il1_7x5);hide(raw_ok_ct0_d2_il1_7x5);hide(ts_ok_ct0_d2_il1_7x5);hide(t::trace);
    trace_ok_ct0_d2_il1_7x5_0();
    trace_ok_ct0_d2_il1_7x5_1();
    assert(t::trace(z_ok_ct0_d2_il1_7x5(),raw_ok_ct0_d2_il1_7x5(),ts_ok_ct0_d2_il1_7x5(),0,25nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_ok_ct0_d2_il1_7x5() ensures z_ok_ct0_d2_il1_7x5().len()==32nat,
        raw_ok_ct0_d2_il1_7x5().len()==24nat,
        z_ok_ct0_d2_il1_7x5()[0]==120u8,
        z_ok_ct0_d2_il1_7x5()[1]==156u8,
        zm::field_value(z_ok_ct0_d2_il1_7x5(),16,1)==1nat,
        zm::field_value(z_ok_ct0_d2_il1_7x5(),17,2)==1nat,
        zm::be_value(z_ok_ct0_d2_il1_7x5(),28nat,4)==1202521672nat,
        ts_ok_ct0_d2_il1_7x5().len()==25nat,
        ts_ok_ct0_d2_il1_7x5()[0].pos==19nat,
        ts_ok_ct0_d2_il1_7x5()[0].before==0nat,
        ts_ok_ct0_d2_il1_7x5().last().next==224nat,
{
    assert(z_ok_ct0_d2_il1_7x5().len()==32nat) by (compute);
    assert(raw_ok_ct0_d2_il1_7x5().len()==24nat) by (compute);
    assert(z_ok_ct0_d2_il1_7x5()[0]==120u8) by (compute);
    assert(z_ok_ct0_d2_il1_7x5()[1]==156u8) by (compute);
    assert(zm::field_value(z_ok_ct0_d2_il1_7x5(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_ok_ct0_d2_il1_7x5(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_ok_ct0_d2_il1_7x5(),28nat,4)==1202521672nat) by (compute);
    assert(ts_ok_ct0_d2_il1_7x5().len()==25nat) by (compute);
    assert(ts_ok_ct0_d2_il1_7x5()[0].pos==19nat) by (compute);
    assert(ts_ok_ct0_d2_il1_7x5()[0].before==0nat) by (compute);
    assert(ts_ok_ct0_d2_il1_7x5().last().next==224nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_ok_ct0_d2_il1_7x5() ensures zm::Adler(raw_ok_ct0_d2_il1_7x5())==1202521672nat,
{
    hide(raw_ok_ct0_d2_il1_7x5);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_ok_ct0_d2_il1_7x5().len()==24nat) by (compute);
    assert(c::sums(raw_ok_ct0_d2_il1_7x5(),0,24nat)==(1607nat,18349nat)) by (compute);
    c::sums_correct(raw_ok_ct0_d2_il1_7x5(),0,24nat);assert(raw_ok_ct0_d2_il1_7x5().subrange(0,24)=~=raw_ok_ct0_d2_il1_7x5());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((18349nat%65521)*(u16::MAX as nat+1)+(1+1607nat)%65521==1202521672nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_ok_ct0_d2_il1_7x5() ensures zm::Inflate(z_ok_ct0_d2_il1_7x5(),24nat,true,false)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_ok_ct0_d2_il1_7x5(),r2:false}),raw_ok_ct0_d2_il1_7x5().len()==24nat,
{
    hide(z_ok_ct0_d2_il1_7x5);hide(raw_ok_ct0_d2_il1_7x5);hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);hide(ts_ok_ct0_d2_il1_7x5);
    shape_ok_ct0_d2_il1_7x5();checksum_ok_ct0_d2_il1_7x5();
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_ok_ct0_d2_il1_7x5();ms::fixed_stream(z_ok_ct0_d2_il1_7x5(),raw_ok_ct0_d2_il1_7x5(),ts_ok_ct0_d2_il1_7x5(),24nat,156nat,1202521672nat,1202521672nat);
}
#[verifier::spinoff_prover]
proof fn walk_ok_ct0_d2_il1_7x5(b: Seq<u8>)
    requires b.len()==89,
        crate::model::mh(b)==Ok((h_ok_ct0_d2_il1_7x5(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:32nat,next:77nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,77nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:85nat,len:0nat,next:89nat,crc_ok:true,synthetic:false})),
        b.subrange(41,73)==z_ok_ct0_d2_il1_7x5(),
    ensures r::default_case(b,h_ok_ct0_d2_il1_7x5(),z_ok_ct0_d2_il1_7x5(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert((1u32 << 2u8)==4u32) by (compute);
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
proof fn scan_ok_ct0_d2_il1_7x5() ensures r::scan(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),0)==Ok(()),
{
    hide(raw_ok_ct0_d2_il1_7x5);assert(raw_ok_ct0_d2_il1_7x5().len()==24nat) by (compute);assert(r::count(a_ok_ct0_d2_il1_7x5())==7nat) by (compute);
    assert(r::pw(a_ok_ct0_d2_il1_7x5(),0nat)==1nat && r::ph(a_ok_ct0_d2_il1_7x5(),0nat)==1nat && r::rb(a_ok_ct0_d2_il1_7x5(),0nat)==1nat && r::base(a_ok_ct0_d2_il1_7x5(),0nat)==0nat) by (compute);
    assert(raw_ok_ct0_d2_il1_7x5()[0]==0u8) by (compute);
    assert(r::row_result(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),0nat,0nat)==Ok(()));
    assert(r::pw(a_ok_ct0_d2_il1_7x5(),1nat)==1nat && r::ph(a_ok_ct0_d2_il1_7x5(),1nat)==1nat && r::rb(a_ok_ct0_d2_il1_7x5(),1nat)==1nat && r::base(a_ok_ct0_d2_il1_7x5(),1nat)==2nat) by (compute);
    assert(raw_ok_ct0_d2_il1_7x5()[2]==1u8) by (compute);
    assert(r::row_result(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),1nat,0nat)==Ok(()));
    assert(r::pw(a_ok_ct0_d2_il1_7x5(),2nat)==2nat && r::ph(a_ok_ct0_d2_il1_7x5(),2nat)==1nat && r::rb(a_ok_ct0_d2_il1_7x5(),2nat)==1nat && r::base(a_ok_ct0_d2_il1_7x5(),2nat)==4nat) by (compute);
    assert(raw_ok_ct0_d2_il1_7x5()[4]==2u8) by (compute);
    assert(r::row_result(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),2nat,0nat)==Ok(()));
    assert(r::pw(a_ok_ct0_d2_il1_7x5(),3nat)==2nat && r::ph(a_ok_ct0_d2_il1_7x5(),3nat)==2nat && r::rb(a_ok_ct0_d2_il1_7x5(),3nat)==1nat && r::base(a_ok_ct0_d2_il1_7x5(),3nat)==6nat) by (compute);
    assert(raw_ok_ct0_d2_il1_7x5()[6]==3u8) by (compute);
    assert(r::row_result(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),3nat,0nat)==Ok(()));
    assert(raw_ok_ct0_d2_il1_7x5()[8]==4u8) by (compute);
    assert(r::row_result(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),3nat,1nat)==Ok(()));
    assert(r::pw(a_ok_ct0_d2_il1_7x5(),4nat)==4nat && r::ph(a_ok_ct0_d2_il1_7x5(),4nat)==1nat && r::rb(a_ok_ct0_d2_il1_7x5(),4nat)==1nat && r::base(a_ok_ct0_d2_il1_7x5(),4nat)==10nat) by (compute);
    assert(raw_ok_ct0_d2_il1_7x5()[10]==0u8) by (compute);
    assert(r::row_result(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),4nat,0nat)==Ok(()));
    assert(r::pw(a_ok_ct0_d2_il1_7x5(),5nat)==3nat && r::ph(a_ok_ct0_d2_il1_7x5(),5nat)==3nat && r::rb(a_ok_ct0_d2_il1_7x5(),5nat)==1nat && r::base(a_ok_ct0_d2_il1_7x5(),5nat)==12nat) by (compute);
    assert(raw_ok_ct0_d2_il1_7x5()[12]==1u8) by (compute);
    assert(r::row_result(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),5nat,0nat)==Ok(()));
    assert(raw_ok_ct0_d2_il1_7x5()[14]==2u8) by (compute);
    assert(r::row_result(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),5nat,1nat)==Ok(()));
    assert(raw_ok_ct0_d2_il1_7x5()[16]==3u8) by (compute);
    assert(r::row_result(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),5nat,2nat)==Ok(()));
    assert(r::pw(a_ok_ct0_d2_il1_7x5(),6nat)==7nat && r::ph(a_ok_ct0_d2_il1_7x5(),6nat)==2nat && r::rb(a_ok_ct0_d2_il1_7x5(),6nat)==2nat && r::base(a_ok_ct0_d2_il1_7x5(),6nat)==18nat) by (compute);
    assert(raw_ok_ct0_d2_il1_7x5()[18]==4u8) by (compute);
    assert(r::row_result(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),6nat,0nat)==Ok(()));
    assert(raw_ok_ct0_d2_il1_7x5()[21]==0u8) by (compute);
    assert(r::row_result(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),6nat,1nat)==Ok(()));
    r::scan_step(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),7nat);
    r::pass_step(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),6nat,2nat);
    r::pass_step(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),6nat,1nat);
    r::pass_step(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),6nat,0nat);
    r::scan_step(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),6nat);
    r::pass_step(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),5nat,3nat);
    r::pass_step(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),5nat,2nat);
    r::pass_step(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),5nat,1nat);
    r::pass_step(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),5nat,0nat);
    r::scan_step(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),5nat);
    r::pass_step(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),4nat,1nat);
    r::pass_step(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),4nat,0nat);
    r::scan_step(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),4nat);
    r::pass_step(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),3nat,2nat);
    r::pass_step(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),3nat,1nat);
    r::pass_step(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),3nat,0nat);
    r::scan_step(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),3nat);
    r::pass_step(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),2nat,1nat);
    r::pass_step(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),2nat,0nat);
    r::scan_step(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),2nat);
    r::pass_step(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),1nat,1nat);
    r::pass_step(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),1nat,0nat);
    r::scan_step(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),1nat);
    r::pass_step(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),0nat,1nat);
    r::pass_step(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),0nat,0nat);
    r::scan_step(raw_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_ok_ct0_d2_il1_7x5() ensures r::still(bytes_ok_ct0_d2_il1_7x5(),STRICT,false)==(r::Projection::Ready {width:7u32,height:5u32,bytes:140nat}),
{
    hide(bytes_ok_ct0_d2_il1_7x5);hide(z_ok_ct0_d2_il1_7x5);hide(raw_ok_ct0_d2_il1_7x5);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    assert(bytes_ok_ct0_d2_il1_7x5().len()==89nat) by (compute);
    assert(crate::model::mh(bytes_ok_ct0_d2_il1_7x5())==Ok((h_ok_ct0_d2_il1_7x5(),33nat))) by {reveal(crate::model::mh);assert(crate::model::mh(bytes_ok_ct0_d2_il1_7x5())==Ok((h_ok_ct0_d2_il1_7x5(),33nat))) by (compute);}
        assert((1u32 << 2u8)==4u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_ok_ct0_d2_il1_7x5(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:32nat,next:77nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_ok_ct0_d2_il1_7x5(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:32nat,next:77nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_ok_ct0_d2_il1_7x5(),77nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:85nat,len:0nat,next:89nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_ok_ct0_d2_il1_7x5(),77nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:85nat,len:0nat,next:89nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_ok_ct0_d2_il1_7x5().subrange(41,73)==z_ok_ct0_d2_il1_7x5()) by (compute);
    walk_ok_ct0_d2_il1_7x5(bytes_ok_ct0_d2_il1_7x5());
    inflate_ok_ct0_d2_il1_7x5();scan_ok_ct0_d2_il1_7x5();
    assert(r::raw(a_ok_ct0_d2_il1_7x5())==24nat) by (compute);
    assert(r::data(z_ok_ct0_d2_il1_7x5(),a_ok_ct0_d2_il1_7x5(),STRICT)==Some(Ok(()))) by {reveal(r::data);};
    r::default_ready(bytes_ok_ct0_d2_il1_7x5(),h_ok_ct0_d2_il1_7x5(),z_ok_ct0_d2_il1_7x5(),0nat,false);
}
fn public_ok_ct0_d2_il1_7x5() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,7u8,0u8,0u8,0u8,5u8,2u8,0u8,0u8,0u8,1u8,145u8,70u8,129u8,51u8,0u8,0u8,0u8,32u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,192u8,200u8,192u8,180u8,129u8,217u8,129u8,37u8,128u8,97u8,55u8,99u8,10u8,211u8,21u8,230u8,19u8,44u8,243u8,132u8,24u8,74u8,66u8,0u8,71u8,173u8,6u8,72u8,235u8,130u8,210u8,19u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_ok_ct0_d2_il1_7x5());model_ok_ct0_d2_il1_7x5();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==7u32 && s.height==5u32 && s.rgba@.len()==140nat,Err(crate::alloc_guard::TryError::OutOfMemory(_))=>true,_=>false});
}
// ok/ct3-d4-il0-9x10; frozen SHA256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_ok_ct3_d4_il0_9x10() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,9u8,0u8,0u8,0u8,10u8,4u8,3u8,0u8,0u8,0u8,148u8,43u8,105u8,141u8,0u8,0u8,0u8,48u8,80u8,76u8,84u8,69u8,202u8,150u8,248u8,13u8,189u8,118u8,4u8,194u8,222u8,220u8,59u8,247u8,50u8,80u8,152u8,170u8,177u8,51u8,229u8,86u8,57u8,245u8,189u8,200u8,134u8,123u8,104u8,1u8,245u8,91u8,51u8,240u8,201u8,161u8,9u8,28u8,39u8,68u8,94u8,185u8,21u8,142u8,217u8,246u8,59u8,128u8,196u8,211u8,231u8,78u8,42u8,174u8,0u8,0u8,0u8,71u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,60u8,0u8,195u8,255u8,0u8,239u8,181u8,68u8,141u8,208u8,1u8,134u8,18u8,238u8,65u8,249u8,2u8,142u8,2u8,98u8,21u8,112u8,3u8,227u8,54u8,224u8,153u8,161u8,4u8,150u8,36u8,99u8,172u8,16u8,0u8,51u8,161u8,146u8,252u8,64u8,1u8,188u8,172u8,97u8,71u8,144u8,2u8,87u8,122u8,67u8,231u8,240u8,3u8,141u8,167u8,228u8,226u8,243u8,4u8,229u8,161u8,163u8,201u8,48u8,26u8,132u8,27u8,217u8,199u8,153u8,6u8,91u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn h_ok_ct3_d4_il0_9x10() -> crate::model::Ihdr {crate::model::Ihdr {width:9u32,height:10u32,depth:4u8,color_type:3u8,interlace:0u8}}
spec fn a_ok_ct3_d4_il0_9x10() -> r::Raster {r::shape(h_ok_ct3_d4_il0_9x10(),9nat,10nat,16nat,STRICT)}
spec fn z_ok_ct3_d4_il0_9x10() -> Seq<u8> {seq![120u8,156u8,1u8,60u8,0u8,195u8,255u8,0u8,239u8,181u8,68u8,141u8,208u8,1u8,134u8,18u8,238u8,65u8,249u8,2u8,142u8,2u8,98u8,21u8,112u8,3u8,227u8,54u8,224u8,153u8,161u8,4u8,150u8,36u8,99u8,172u8,16u8,0u8,51u8,161u8,146u8,252u8,64u8,1u8,188u8,172u8,97u8,71u8,144u8,2u8,87u8,122u8,67u8,231u8,240u8,3u8,141u8,167u8,228u8,226u8,243u8,4u8,229u8,161u8,163u8,201u8,48u8,26u8,132u8,27u8,217u8]}
spec fn raw_ok_ct3_d4_il0_9x10() -> Seq<u8> {seq![0u8,239u8,181u8,68u8,141u8,208u8,1u8,134u8,18u8,238u8,65u8,249u8,2u8,142u8,2u8,98u8,21u8,112u8,3u8,227u8,54u8,224u8,153u8,161u8,4u8,150u8,36u8,99u8,172u8,16u8,0u8,51u8,161u8,146u8,252u8,64u8,1u8,188u8,172u8,97u8,71u8,144u8,2u8,87u8,122u8,67u8,231u8,240u8,3u8,141u8,167u8,228u8,226u8,243u8,4u8,229u8,161u8,163u8,201u8,48u8]}
#[verifier::spinoff_prover]
proof fn shape_ok_ct3_d4_il0_9x10() ensures z_ok_ct3_d4_il0_9x10().len()==71nat,
        raw_ok_ct3_d4_il0_9x10().len()==60nat,
        z_ok_ct3_d4_il0_9x10()[0]==120u8,
        z_ok_ct3_d4_il0_9x10()[1]==156u8,
        zm::field_value(z_ok_ct3_d4_il0_9x10(),16,1)==1nat,
        zm::field_value(z_ok_ct3_d4_il0_9x10(),17,2)==0nat,
        zm::be_value(z_ok_ct3_d4_il0_9x10(),67nat,4)==444865497nat,
        z_ok_ct3_d4_il0_9x10().subrange(7,67)==raw_ok_ct3_d4_il0_9x10(),
        z_ok_ct3_d4_il0_9x10()[3] as nat+256*z_ok_ct3_d4_il0_9x10()[4] as nat==60nat,
        z_ok_ct3_d4_il0_9x10()[5] as nat+256*z_ok_ct3_d4_il0_9x10()[6] as nat==u16::MAX as nat-60nat,
        zm::be_value(z_ok_ct3_d4_il0_9x10(),0,1)==120nat,
        zm::be_value(z_ok_ct3_d4_il0_9x10(),1,1)==156nat,
{
    assert(z_ok_ct3_d4_il0_9x10().len()==71nat) by (compute);
    assert(raw_ok_ct3_d4_il0_9x10().len()==60nat) by (compute);
    assert(z_ok_ct3_d4_il0_9x10()[0]==120u8) by (compute);
    assert(z_ok_ct3_d4_il0_9x10()[1]==156u8) by (compute);
    assert(zm::field_value(z_ok_ct3_d4_il0_9x10(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_ok_ct3_d4_il0_9x10(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_ok_ct3_d4_il0_9x10(),67nat,4)==444865497nat) by (compute);
    assert(z_ok_ct3_d4_il0_9x10().subrange(7,67)==raw_ok_ct3_d4_il0_9x10()) by (compute);
    assert(z_ok_ct3_d4_il0_9x10()[3] as nat+256*z_ok_ct3_d4_il0_9x10()[4] as nat==60nat) by (compute);
    assert(z_ok_ct3_d4_il0_9x10()[5] as nat+256*z_ok_ct3_d4_il0_9x10()[6] as nat==u16::MAX as nat-60nat) by (compute);
    assert(zm::be_value(z_ok_ct3_d4_il0_9x10(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_ok_ct3_d4_il0_9x10(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_ok_ct3_d4_il0_9x10() ensures zm::Adler(raw_ok_ct3_d4_il0_9x10())==444865497nat,
{
    hide(raw_ok_ct3_d4_il0_9x10);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_ok_ct3_d4_il0_9x10().len()==60nat) by (compute);
    assert(c::sums(raw_ok_ct3_d4_il0_9x10(),0,60nat)==(7128nat,203351nat)) by (compute);
    c::sums_correct(raw_ok_ct3_d4_il0_9x10(),0,60nat);assert(raw_ok_ct3_d4_il0_9x10().subrange(0,60)=~=raw_ok_ct3_d4_il0_9x10());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((203351nat%65521)*(u16::MAX as nat+1)+(1+7128nat)%65521==444865497nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_ok_ct3_d4_il0_9x10() ensures zm::Inflate(z_ok_ct3_d4_il0_9x10(),60nat,true,false)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_ok_ct3_d4_il0_9x10(),r2:false}),raw_ok_ct3_d4_il0_9x10().len()==60nat,
{
    hide(z_ok_ct3_d4_il0_9x10);hide(raw_ok_ct3_d4_il0_9x10);hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_ok_ct3_d4_il0_9x10();checksum_ok_ct3_d4_il0_9x10();
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_ok_ct3_d4_il0_9x10(),raw_ok_ct3_d4_il0_9x10(),60nat,120nat,156nat,444865497nat);
}
#[verifier::spinoff_prover]
proof fn crc_ok_ct3_d4_il0_9x10_crc_93()
    ensures crate::model::crc(bytes_ok_ct3_d4_il0_9x10().subrange(97,172))==3348694619u32,
{
    hide(crate::model::crc_register);hide(crc_cert::fold);
    assert(crate::model::crc_register(bytes_ok_ct3_d4_il0_9x10().subrange(97,172).take(0))==0xffff_ffffu32) by {
        assert(bytes_ok_ct3_d4_il0_9x10().subrange(97,172).take(0) =~= Seq::<u8>::empty());
        reveal_with_fuel(crate::model::crc_register,1);
    }
    assert(crc_cert::fold(4294967295u32,bytes_ok_ct3_d4_il0_9x10().subrange(97,172).subrange(0,64))==4126203307u32) by (compute);
    crc_cert::suffix(bytes_ok_ct3_d4_il0_9x10().subrange(97,172).take(0),bytes_ok_ct3_d4_il0_9x10().subrange(97,172).subrange(0,64));
    assert(bytes_ok_ct3_d4_il0_9x10().subrange(97,172).take(0)+bytes_ok_ct3_d4_il0_9x10().subrange(97,172).subrange(0,64) =~= bytes_ok_ct3_d4_il0_9x10().subrange(97,172).take(64));
    assert(crate::model::crc_register(bytes_ok_ct3_d4_il0_9x10().subrange(97,172).take(64))==4126203307u32);
    assert(crc_cert::fold(4126203307u32,bytes_ok_ct3_d4_il0_9x10().subrange(97,172).subrange(64,75))==946272676u32) by (compute);
    crc_cert::suffix(bytes_ok_ct3_d4_il0_9x10().subrange(97,172).take(64),bytes_ok_ct3_d4_il0_9x10().subrange(97,172).subrange(64,75));
    assert(bytes_ok_ct3_d4_il0_9x10().subrange(97,172).take(64)+bytes_ok_ct3_d4_il0_9x10().subrange(97,172).subrange(64,75) =~= bytes_ok_ct3_d4_il0_9x10().subrange(97,172).take(75));
    assert(crate::model::crc_register(bytes_ok_ct3_d4_il0_9x10().subrange(97,172).take(75))==946272676u32);
    assert(bytes_ok_ct3_d4_il0_9x10().subrange(97,172).take(75) =~= bytes_ok_ct3_d4_il0_9x10().subrange(97,172));
    assert((946272676u32 ^ 0xffff_ffffu32)==3348694619u32) by (compute);
}
#[verifier::spinoff_prover]
proof fn walk_ok_ct3_d4_il0_9x10(b: Seq<u8>)
    requires b.len()==188,
        crate::model::mh(b)==Ok((h_ok_ct3_d4_il0_9x10(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:71nat,next:176nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,176nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:184nat,len:0nat,next:188nat,crc_ok:true,synthetic:false})),
        b.subrange(101,172)==z_ok_ct3_d4_il0_9x10(),
    ensures r::default_case(b,h_ok_ct3_d4_il0_9x10(),z_ok_ct3_d4_il0_9x10(),16nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert((1u32 << 4u8)==16u32) by (compute);
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(48nat,3u8,4u8,false,false,false,false)==crate::metadata_model::Decision::Keep(16nat)) by (compute);
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
proof fn scan_ok_ct3_d4_il0_9x10() ensures r::scan(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0)==Ok(()),
{
    hide(raw_ok_ct3_d4_il0_9x10);assert(raw_ok_ct3_d4_il0_9x10().len()==60nat) by (compute);assert(r::count(a_ok_ct3_d4_il0_9x10())==1nat) by (compute);
    assert(r::pw(a_ok_ct3_d4_il0_9x10(),0nat)==9nat && r::ph(a_ok_ct3_d4_il0_9x10(),0nat)==10nat && r::rb(a_ok_ct3_d4_il0_9x10(),0nat)==5nat && r::base(a_ok_ct3_d4_il0_9x10(),0nat)==0nat) by (compute);
    assert(raw_ok_ct3_d4_il0_9x10()[0]==0u8) by (compute);
    f::pass_row_equation(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,1nat,0nat);
    assert(f::previous_row(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,1nat,0nat)==None);
    assert(f::encoded_row(0u8,f::row_data(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,0nat),None,1nat,seq![239u8,181u8,68u8,141u8,208u8],0,5nat)) by (compute);
    f::checked_row_hint(0u8,f::row_data(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,0nat),None,1nat,seq![239u8,181u8,68u8,141u8,208u8]);
    assert(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,0nat)==seq![239u8,181u8,68u8,141u8,208u8]);
    assert forall|x: nat| x<9 implies (#[trigger] r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,0nat),4u8,x) as nat)<16 by {
        if x==0 {assert((r::palette_sample(seq![239u8,181u8,68u8,141u8,208u8],4u8,0nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,0nat),4u8,0nat) as nat)<16nat);}
        else if x==1 {assert((r::palette_sample(seq![239u8,181u8,68u8,141u8,208u8],4u8,1nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,0nat),4u8,1nat) as nat)<16nat);}
        else if x==2 {assert((r::palette_sample(seq![239u8,181u8,68u8,141u8,208u8],4u8,2nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,0nat),4u8,2nat) as nat)<16nat);}
        else if x==3 {assert((r::palette_sample(seq![239u8,181u8,68u8,141u8,208u8],4u8,3nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,0nat),4u8,3nat) as nat)<16nat);}
        else if x==4 {assert((r::palette_sample(seq![239u8,181u8,68u8,141u8,208u8],4u8,4nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,0nat),4u8,4nat) as nat)<16nat);}
        else if x==5 {assert((r::palette_sample(seq![239u8,181u8,68u8,141u8,208u8],4u8,5nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,0nat),4u8,5nat) as nat)<16nat);}
        else if x==6 {assert((r::palette_sample(seq![239u8,181u8,68u8,141u8,208u8],4u8,6nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,0nat),4u8,6nat) as nat)<16nat);}
        else if x==7 {assert((r::palette_sample(seq![239u8,181u8,68u8,141u8,208u8],4u8,7nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,0nat),4u8,7nat) as nat)<16nat);}
        else if x==8 {assert((r::palette_sample(seq![239u8,181u8,68u8,141u8,208u8],4u8,8nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,0nat),4u8,8nat) as nat)<16nat);}
    }
    assert(r::palette_valid(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,0nat),4u8,9nat,16nat));
    assert(r::row_result(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,0nat)==Ok(()));
    assert(raw_ok_ct3_d4_il0_9x10()[6]==1u8) by (compute);
    f::pass_row_equation(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,1nat,1nat);
    assert(f::previous_row(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,1nat,1nat)==Some(seq![239u8,181u8,68u8,141u8,208u8]));
    assert(f::encoded_row(1u8,f::row_data(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,1nat),Some(seq![239u8,181u8,68u8,141u8,208u8]),1nat,seq![134u8,152u8,134u8,199u8,192u8],0,5nat)) by (compute);
    f::checked_row_hint(1u8,f::row_data(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,1nat),Some(seq![239u8,181u8,68u8,141u8,208u8]),1nat,seq![134u8,152u8,134u8,199u8,192u8]);
    assert(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,1nat)==seq![134u8,152u8,134u8,199u8,192u8]);
    assert forall|x: nat| x<9 implies (#[trigger] r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,1nat),4u8,x) as nat)<16 by {
        if x==0 {assert((r::palette_sample(seq![134u8,152u8,134u8,199u8,192u8],4u8,0nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,1nat),4u8,0nat) as nat)<16nat);}
        else if x==1 {assert((r::palette_sample(seq![134u8,152u8,134u8,199u8,192u8],4u8,1nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,1nat),4u8,1nat) as nat)<16nat);}
        else if x==2 {assert((r::palette_sample(seq![134u8,152u8,134u8,199u8,192u8],4u8,2nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,1nat),4u8,2nat) as nat)<16nat);}
        else if x==3 {assert((r::palette_sample(seq![134u8,152u8,134u8,199u8,192u8],4u8,3nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,1nat),4u8,3nat) as nat)<16nat);}
        else if x==4 {assert((r::palette_sample(seq![134u8,152u8,134u8,199u8,192u8],4u8,4nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,1nat),4u8,4nat) as nat)<16nat);}
        else if x==5 {assert((r::palette_sample(seq![134u8,152u8,134u8,199u8,192u8],4u8,5nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,1nat),4u8,5nat) as nat)<16nat);}
        else if x==6 {assert((r::palette_sample(seq![134u8,152u8,134u8,199u8,192u8],4u8,6nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,1nat),4u8,6nat) as nat)<16nat);}
        else if x==7 {assert((r::palette_sample(seq![134u8,152u8,134u8,199u8,192u8],4u8,7nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,1nat),4u8,7nat) as nat)<16nat);}
        else if x==8 {assert((r::palette_sample(seq![134u8,152u8,134u8,199u8,192u8],4u8,8nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,1nat),4u8,8nat) as nat)<16nat);}
    }
    assert(r::palette_valid(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,1nat),4u8,9nat,16nat));
    assert(r::row_result(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,1nat)==Ok(()));
    assert(raw_ok_ct3_d4_il0_9x10()[12]==2u8) by (compute);
    f::pass_row_equation(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,1nat,2nat);
    assert(f::previous_row(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,1nat,2nat)==Some(seq![134u8,152u8,134u8,199u8,192u8]));
    assert(f::encoded_row(2u8,f::row_data(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,2nat),Some(seq![134u8,152u8,134u8,199u8,192u8]),1nat,seq![20u8,154u8,232u8,220u8,48u8],0,5nat)) by (compute);
    f::checked_row_hint(2u8,f::row_data(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,2nat),Some(seq![134u8,152u8,134u8,199u8,192u8]),1nat,seq![20u8,154u8,232u8,220u8,48u8]);
    assert(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,2nat)==seq![20u8,154u8,232u8,220u8,48u8]);
    assert forall|x: nat| x<9 implies (#[trigger] r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,2nat),4u8,x) as nat)<16 by {
        if x==0 {assert((r::palette_sample(seq![20u8,154u8,232u8,220u8,48u8],4u8,0nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,2nat),4u8,0nat) as nat)<16nat);}
        else if x==1 {assert((r::palette_sample(seq![20u8,154u8,232u8,220u8,48u8],4u8,1nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,2nat),4u8,1nat) as nat)<16nat);}
        else if x==2 {assert((r::palette_sample(seq![20u8,154u8,232u8,220u8,48u8],4u8,2nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,2nat),4u8,2nat) as nat)<16nat);}
        else if x==3 {assert((r::palette_sample(seq![20u8,154u8,232u8,220u8,48u8],4u8,3nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,2nat),4u8,3nat) as nat)<16nat);}
        else if x==4 {assert((r::palette_sample(seq![20u8,154u8,232u8,220u8,48u8],4u8,4nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,2nat),4u8,4nat) as nat)<16nat);}
        else if x==5 {assert((r::palette_sample(seq![20u8,154u8,232u8,220u8,48u8],4u8,5nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,2nat),4u8,5nat) as nat)<16nat);}
        else if x==6 {assert((r::palette_sample(seq![20u8,154u8,232u8,220u8,48u8],4u8,6nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,2nat),4u8,6nat) as nat)<16nat);}
        else if x==7 {assert((r::palette_sample(seq![20u8,154u8,232u8,220u8,48u8],4u8,7nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,2nat),4u8,7nat) as nat)<16nat);}
        else if x==8 {assert((r::palette_sample(seq![20u8,154u8,232u8,220u8,48u8],4u8,8nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,2nat),4u8,8nat) as nat)<16nat);}
    }
    assert(r::palette_valid(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,2nat),4u8,9nat,16nat));
    assert(r::row_result(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,2nat)==Ok(()));
    assert(raw_ok_ct3_d4_il0_9x10()[18]==3u8) by (compute);
    f::pass_row_equation(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,1nat,3nat);
    assert(f::previous_row(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,1nat,3nat)==Some(seq![20u8,154u8,232u8,220u8,48u8]));
    assert(f::encoded_row(3u8,f::row_data(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,3nat),Some(seq![20u8,154u8,232u8,220u8,48u8]),1nat,seq![237u8,249u8,208u8,111u8,240u8],0,5nat)) by (compute);
    f::checked_row_hint(3u8,f::row_data(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,3nat),Some(seq![20u8,154u8,232u8,220u8,48u8]),1nat,seq![237u8,249u8,208u8,111u8,240u8]);
    assert(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,3nat)==seq![237u8,249u8,208u8,111u8,240u8]);
    assert forall|x: nat| x<9 implies (#[trigger] r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,3nat),4u8,x) as nat)<16 by {
        if x==0 {assert((r::palette_sample(seq![237u8,249u8,208u8,111u8,240u8],4u8,0nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,3nat),4u8,0nat) as nat)<16nat);}
        else if x==1 {assert((r::palette_sample(seq![237u8,249u8,208u8,111u8,240u8],4u8,1nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,3nat),4u8,1nat) as nat)<16nat);}
        else if x==2 {assert((r::palette_sample(seq![237u8,249u8,208u8,111u8,240u8],4u8,2nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,3nat),4u8,2nat) as nat)<16nat);}
        else if x==3 {assert((r::palette_sample(seq![237u8,249u8,208u8,111u8,240u8],4u8,3nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,3nat),4u8,3nat) as nat)<16nat);}
        else if x==4 {assert((r::palette_sample(seq![237u8,249u8,208u8,111u8,240u8],4u8,4nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,3nat),4u8,4nat) as nat)<16nat);}
        else if x==5 {assert((r::palette_sample(seq![237u8,249u8,208u8,111u8,240u8],4u8,5nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,3nat),4u8,5nat) as nat)<16nat);}
        else if x==6 {assert((r::palette_sample(seq![237u8,249u8,208u8,111u8,240u8],4u8,6nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,3nat),4u8,6nat) as nat)<16nat);}
        else if x==7 {assert((r::palette_sample(seq![237u8,249u8,208u8,111u8,240u8],4u8,7nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,3nat),4u8,7nat) as nat)<16nat);}
        else if x==8 {assert((r::palette_sample(seq![237u8,249u8,208u8,111u8,240u8],4u8,8nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,3nat),4u8,8nat) as nat)<16nat);}
    }
    assert(r::palette_valid(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,3nat),4u8,9nat,16nat));
    assert(r::row_result(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,3nat)==Ok(()));
    assert(raw_ok_ct3_d4_il0_9x10()[24]==4u8) by (compute);
    f::pass_row_equation(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,1nat,4nat);
    assert(f::previous_row(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,1nat,4nat)==Some(seq![237u8,249u8,208u8,111u8,240u8]));
    assert(f::encoded_row(4u8,f::row_data(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,4nat),Some(seq![237u8,249u8,208u8,111u8,240u8]),1nat,seq![131u8,167u8,10u8,182u8,0u8],0,5nat)) by (compute);
    f::checked_row_hint(4u8,f::row_data(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,4nat),Some(seq![237u8,249u8,208u8,111u8,240u8]),1nat,seq![131u8,167u8,10u8,182u8,0u8]);
    assert(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,4nat)==seq![131u8,167u8,10u8,182u8,0u8]);
    assert forall|x: nat| x<9 implies (#[trigger] r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,4nat),4u8,x) as nat)<16 by {
        if x==0 {assert((r::palette_sample(seq![131u8,167u8,10u8,182u8,0u8],4u8,0nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,4nat),4u8,0nat) as nat)<16nat);}
        else if x==1 {assert((r::palette_sample(seq![131u8,167u8,10u8,182u8,0u8],4u8,1nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,4nat),4u8,1nat) as nat)<16nat);}
        else if x==2 {assert((r::palette_sample(seq![131u8,167u8,10u8,182u8,0u8],4u8,2nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,4nat),4u8,2nat) as nat)<16nat);}
        else if x==3 {assert((r::palette_sample(seq![131u8,167u8,10u8,182u8,0u8],4u8,3nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,4nat),4u8,3nat) as nat)<16nat);}
        else if x==4 {assert((r::palette_sample(seq![131u8,167u8,10u8,182u8,0u8],4u8,4nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,4nat),4u8,4nat) as nat)<16nat);}
        else if x==5 {assert((r::palette_sample(seq![131u8,167u8,10u8,182u8,0u8],4u8,5nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,4nat),4u8,5nat) as nat)<16nat);}
        else if x==6 {assert((r::palette_sample(seq![131u8,167u8,10u8,182u8,0u8],4u8,6nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,4nat),4u8,6nat) as nat)<16nat);}
        else if x==7 {assert((r::palette_sample(seq![131u8,167u8,10u8,182u8,0u8],4u8,7nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,4nat),4u8,7nat) as nat)<16nat);}
        else if x==8 {assert((r::palette_sample(seq![131u8,167u8,10u8,182u8,0u8],4u8,8nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,4nat),4u8,8nat) as nat)<16nat);}
    }
    assert(r::palette_valid(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,4nat),4u8,9nat,16nat));
    assert(r::row_result(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,4nat)==Ok(()));
    assert(raw_ok_ct3_d4_il0_9x10()[30]==0u8) by (compute);
    f::pass_row_equation(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,1nat,5nat);
    assert(f::previous_row(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,1nat,5nat)==Some(seq![131u8,167u8,10u8,182u8,0u8]));
    assert(f::encoded_row(0u8,f::row_data(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,5nat),Some(seq![131u8,167u8,10u8,182u8,0u8]),1nat,seq![51u8,161u8,146u8,252u8,64u8],0,5nat)) by (compute);
    f::checked_row_hint(0u8,f::row_data(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,5nat),Some(seq![131u8,167u8,10u8,182u8,0u8]),1nat,seq![51u8,161u8,146u8,252u8,64u8]);
    assert(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,5nat)==seq![51u8,161u8,146u8,252u8,64u8]);
    assert forall|x: nat| x<9 implies (#[trigger] r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,5nat),4u8,x) as nat)<16 by {
        if x==0 {assert((r::palette_sample(seq![51u8,161u8,146u8,252u8,64u8],4u8,0nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,5nat),4u8,0nat) as nat)<16nat);}
        else if x==1 {assert((r::palette_sample(seq![51u8,161u8,146u8,252u8,64u8],4u8,1nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,5nat),4u8,1nat) as nat)<16nat);}
        else if x==2 {assert((r::palette_sample(seq![51u8,161u8,146u8,252u8,64u8],4u8,2nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,5nat),4u8,2nat) as nat)<16nat);}
        else if x==3 {assert((r::palette_sample(seq![51u8,161u8,146u8,252u8,64u8],4u8,3nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,5nat),4u8,3nat) as nat)<16nat);}
        else if x==4 {assert((r::palette_sample(seq![51u8,161u8,146u8,252u8,64u8],4u8,4nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,5nat),4u8,4nat) as nat)<16nat);}
        else if x==5 {assert((r::palette_sample(seq![51u8,161u8,146u8,252u8,64u8],4u8,5nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,5nat),4u8,5nat) as nat)<16nat);}
        else if x==6 {assert((r::palette_sample(seq![51u8,161u8,146u8,252u8,64u8],4u8,6nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,5nat),4u8,6nat) as nat)<16nat);}
        else if x==7 {assert((r::palette_sample(seq![51u8,161u8,146u8,252u8,64u8],4u8,7nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,5nat),4u8,7nat) as nat)<16nat);}
        else if x==8 {assert((r::palette_sample(seq![51u8,161u8,146u8,252u8,64u8],4u8,8nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,5nat),4u8,8nat) as nat)<16nat);}
    }
    assert(r::palette_valid(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,5nat),4u8,9nat,16nat));
    assert(r::row_result(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,5nat)==Ok(()));
    assert(raw_ok_ct3_d4_il0_9x10()[36]==1u8) by (compute);
    f::pass_row_equation(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,1nat,6nat);
    assert(f::previous_row(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,1nat,6nat)==Some(seq![51u8,161u8,146u8,252u8,64u8]));
    assert(f::encoded_row(1u8,f::row_data(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,6nat),Some(seq![51u8,161u8,146u8,252u8,64u8]),1nat,seq![188u8,104u8,201u8,16u8,160u8],0,5nat)) by (compute);
    f::checked_row_hint(1u8,f::row_data(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,6nat),Some(seq![51u8,161u8,146u8,252u8,64u8]),1nat,seq![188u8,104u8,201u8,16u8,160u8]);
    assert(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,6nat)==seq![188u8,104u8,201u8,16u8,160u8]);
    assert forall|x: nat| x<9 implies (#[trigger] r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,6nat),4u8,x) as nat)<16 by {
        if x==0 {assert((r::palette_sample(seq![188u8,104u8,201u8,16u8,160u8],4u8,0nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,6nat),4u8,0nat) as nat)<16nat);}
        else if x==1 {assert((r::palette_sample(seq![188u8,104u8,201u8,16u8,160u8],4u8,1nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,6nat),4u8,1nat) as nat)<16nat);}
        else if x==2 {assert((r::palette_sample(seq![188u8,104u8,201u8,16u8,160u8],4u8,2nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,6nat),4u8,2nat) as nat)<16nat);}
        else if x==3 {assert((r::palette_sample(seq![188u8,104u8,201u8,16u8,160u8],4u8,3nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,6nat),4u8,3nat) as nat)<16nat);}
        else if x==4 {assert((r::palette_sample(seq![188u8,104u8,201u8,16u8,160u8],4u8,4nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,6nat),4u8,4nat) as nat)<16nat);}
        else if x==5 {assert((r::palette_sample(seq![188u8,104u8,201u8,16u8,160u8],4u8,5nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,6nat),4u8,5nat) as nat)<16nat);}
        else if x==6 {assert((r::palette_sample(seq![188u8,104u8,201u8,16u8,160u8],4u8,6nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,6nat),4u8,6nat) as nat)<16nat);}
        else if x==7 {assert((r::palette_sample(seq![188u8,104u8,201u8,16u8,160u8],4u8,7nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,6nat),4u8,7nat) as nat)<16nat);}
        else if x==8 {assert((r::palette_sample(seq![188u8,104u8,201u8,16u8,160u8],4u8,8nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,6nat),4u8,8nat) as nat)<16nat);}
    }
    assert(r::palette_valid(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,6nat),4u8,9nat,16nat));
    assert(r::row_result(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,6nat)==Ok(()));
    assert(raw_ok_ct3_d4_il0_9x10()[42]==2u8) by (compute);
    f::pass_row_equation(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,1nat,7nat);
    assert(f::previous_row(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,1nat,7nat)==Some(seq![188u8,104u8,201u8,16u8,160u8]));
    assert(f::encoded_row(2u8,f::row_data(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,7nat),Some(seq![188u8,104u8,201u8,16u8,160u8]),1nat,seq![19u8,226u8,12u8,247u8,144u8],0,5nat)) by (compute);
    f::checked_row_hint(2u8,f::row_data(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,7nat),Some(seq![188u8,104u8,201u8,16u8,160u8]),1nat,seq![19u8,226u8,12u8,247u8,144u8]);
    assert(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,7nat)==seq![19u8,226u8,12u8,247u8,144u8]);
    assert forall|x: nat| x<9 implies (#[trigger] r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,7nat),4u8,x) as nat)<16 by {
        if x==0 {assert((r::palette_sample(seq![19u8,226u8,12u8,247u8,144u8],4u8,0nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,7nat),4u8,0nat) as nat)<16nat);}
        else if x==1 {assert((r::palette_sample(seq![19u8,226u8,12u8,247u8,144u8],4u8,1nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,7nat),4u8,1nat) as nat)<16nat);}
        else if x==2 {assert((r::palette_sample(seq![19u8,226u8,12u8,247u8,144u8],4u8,2nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,7nat),4u8,2nat) as nat)<16nat);}
        else if x==3 {assert((r::palette_sample(seq![19u8,226u8,12u8,247u8,144u8],4u8,3nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,7nat),4u8,3nat) as nat)<16nat);}
        else if x==4 {assert((r::palette_sample(seq![19u8,226u8,12u8,247u8,144u8],4u8,4nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,7nat),4u8,4nat) as nat)<16nat);}
        else if x==5 {assert((r::palette_sample(seq![19u8,226u8,12u8,247u8,144u8],4u8,5nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,7nat),4u8,5nat) as nat)<16nat);}
        else if x==6 {assert((r::palette_sample(seq![19u8,226u8,12u8,247u8,144u8],4u8,6nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,7nat),4u8,6nat) as nat)<16nat);}
        else if x==7 {assert((r::palette_sample(seq![19u8,226u8,12u8,247u8,144u8],4u8,7nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,7nat),4u8,7nat) as nat)<16nat);}
        else if x==8 {assert((r::palette_sample(seq![19u8,226u8,12u8,247u8,144u8],4u8,8nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,7nat),4u8,8nat) as nat)<16nat);}
    }
    assert(r::palette_valid(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,7nat),4u8,9nat,16nat));
    assert(r::row_result(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,7nat)==Ok(()));
    assert(raw_ok_ct3_d4_il0_9x10()[48]==3u8) by (compute);
    f::pass_row_equation(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,1nat,8nat);
    assert(f::previous_row(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,1nat,8nat)==Some(seq![19u8,226u8,12u8,247u8,144u8]));
    assert(f::encoded_row(3u8,f::row_data(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,8nat),Some(seq![19u8,226u8,12u8,247u8,144u8]),1nat,seq![150u8,99u8,27u8,107u8,112u8],0,5nat)) by (compute);
    f::checked_row_hint(3u8,f::row_data(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,8nat),Some(seq![19u8,226u8,12u8,247u8,144u8]),1nat,seq![150u8,99u8,27u8,107u8,112u8]);
    assert(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,8nat)==seq![150u8,99u8,27u8,107u8,112u8]);
    assert forall|x: nat| x<9 implies (#[trigger] r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,8nat),4u8,x) as nat)<16 by {
        if x==0 {assert((r::palette_sample(seq![150u8,99u8,27u8,107u8,112u8],4u8,0nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,8nat),4u8,0nat) as nat)<16nat);}
        else if x==1 {assert((r::palette_sample(seq![150u8,99u8,27u8,107u8,112u8],4u8,1nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,8nat),4u8,1nat) as nat)<16nat);}
        else if x==2 {assert((r::palette_sample(seq![150u8,99u8,27u8,107u8,112u8],4u8,2nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,8nat),4u8,2nat) as nat)<16nat);}
        else if x==3 {assert((r::palette_sample(seq![150u8,99u8,27u8,107u8,112u8],4u8,3nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,8nat),4u8,3nat) as nat)<16nat);}
        else if x==4 {assert((r::palette_sample(seq![150u8,99u8,27u8,107u8,112u8],4u8,4nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,8nat),4u8,4nat) as nat)<16nat);}
        else if x==5 {assert((r::palette_sample(seq![150u8,99u8,27u8,107u8,112u8],4u8,5nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,8nat),4u8,5nat) as nat)<16nat);}
        else if x==6 {assert((r::palette_sample(seq![150u8,99u8,27u8,107u8,112u8],4u8,6nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,8nat),4u8,6nat) as nat)<16nat);}
        else if x==7 {assert((r::palette_sample(seq![150u8,99u8,27u8,107u8,112u8],4u8,7nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,8nat),4u8,7nat) as nat)<16nat);}
        else if x==8 {assert((r::palette_sample(seq![150u8,99u8,27u8,107u8,112u8],4u8,8nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,8nat),4u8,8nat) as nat)<16nat);}
    }
    assert(r::palette_valid(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,8nat),4u8,9nat,16nat));
    assert(r::row_result(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,8nat)==Ok(()));
    assert(raw_ok_ct3_d4_il0_9x10()[54]==4u8) by (compute);
    f::pass_row_equation(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,1nat,9nat);
    assert(f::previous_row(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,1nat,9nat)==Some(seq![150u8,99u8,27u8,107u8,112u8]));
    assert(f::encoded_row(4u8,f::row_data(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,9nat),Some(seq![150u8,99u8,27u8,107u8,112u8]),1nat,seq![123u8,4u8,167u8,112u8,160u8],0,5nat)) by (compute);
    f::checked_row_hint(4u8,f::row_data(raw_ok_ct3_d4_il0_9x10(),0nat,5nat,9nat),Some(seq![150u8,99u8,27u8,107u8,112u8]),1nat,seq![123u8,4u8,167u8,112u8,160u8]);
    assert(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,9nat)==seq![123u8,4u8,167u8,112u8,160u8]);
    assert forall|x: nat| x<9 implies (#[trigger] r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,9nat),4u8,x) as nat)<16 by {
        if x==0 {assert((r::palette_sample(seq![123u8,4u8,167u8,112u8,160u8],4u8,0nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,9nat),4u8,0nat) as nat)<16nat);}
        else if x==1 {assert((r::palette_sample(seq![123u8,4u8,167u8,112u8,160u8],4u8,1nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,9nat),4u8,1nat) as nat)<16nat);}
        else if x==2 {assert((r::palette_sample(seq![123u8,4u8,167u8,112u8,160u8],4u8,2nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,9nat),4u8,2nat) as nat)<16nat);}
        else if x==3 {assert((r::palette_sample(seq![123u8,4u8,167u8,112u8,160u8],4u8,3nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,9nat),4u8,3nat) as nat)<16nat);}
        else if x==4 {assert((r::palette_sample(seq![123u8,4u8,167u8,112u8,160u8],4u8,4nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,9nat),4u8,4nat) as nat)<16nat);}
        else if x==5 {assert((r::palette_sample(seq![123u8,4u8,167u8,112u8,160u8],4u8,5nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,9nat),4u8,5nat) as nat)<16nat);}
        else if x==6 {assert((r::palette_sample(seq![123u8,4u8,167u8,112u8,160u8],4u8,6nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,9nat),4u8,6nat) as nat)<16nat);}
        else if x==7 {assert((r::palette_sample(seq![123u8,4u8,167u8,112u8,160u8],4u8,7nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,9nat),4u8,7nat) as nat)<16nat);}
        else if x==8 {assert((r::palette_sample(seq![123u8,4u8,167u8,112u8,160u8],4u8,8nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,9nat),4u8,8nat) as nat)<16nat);}
    }
    assert(r::palette_valid(r::line(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,9nat),4u8,9nat,16nat));
    assert(r::row_result(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,9nat)==Ok(()));
    r::scan_step(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),1nat);
    r::pass_step(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,10nat);
    r::pass_step(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,9nat);
    r::pass_step(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,8nat);
    r::pass_step(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,7nat);
    r::pass_step(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,6nat);
    r::pass_step(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,5nat);
    r::pass_step(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,4nat);
    r::pass_step(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,3nat);
    r::pass_step(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,2nat);
    r::pass_step(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,1nat);
    r::pass_step(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat,0nat);
    r::scan_step(raw_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_ok_ct3_d4_il0_9x10() ensures r::still(bytes_ok_ct3_d4_il0_9x10(),STRICT,false)==(r::Projection::Ready {width:9u32,height:10u32,bytes:360nat}),
{
    hide(bytes_ok_ct3_d4_il0_9x10);hide(z_ok_ct3_d4_il0_9x10);hide(raw_ok_ct3_d4_il0_9x10);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    assert(bytes_ok_ct3_d4_il0_9x10().len()==188nat) by (compute);
    assert(crate::model::mh(bytes_ok_ct3_d4_il0_9x10())==Ok((h_ok_ct3_d4_il0_9x10(),33nat))) by {reveal(crate::model::mh);assert(crate::model::mh(bytes_ok_ct3_d4_il0_9x10())==Ok((h_ok_ct3_d4_il0_9x10(),33nat))) by (compute);}
        assert((1u32 << 4u8)==16u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_ok_ct3_d4_il0_9x10(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_ok_ct3_d4_il0_9x10(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(48nat,3u8,4u8,false,false,false,false)==crate::metadata_model::Decision::Keep(16nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_ok_ct3_d4_il0_9x10(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:71nat,next:176nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::model::frame(bytes_ok_ct3_d4_il0_9x10(),93nat)==Ok(crate::model::WireChunk {ty:seq![73u8,68u8,65u8,84u8],data:101nat,len:71nat,next:176nat})) by (compute);
            assert(crate::model::be32(bytes_ok_ct3_d4_il0_9x10(),172)==3348694619u32) by (compute);
            crc_ok_ct3_d4_il0_9x10_crc_93();
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_ok_ct3_d4_il0_9x10(),176nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:184nat,len:0nat,next:188nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_ok_ct3_d4_il0_9x10(),176nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:184nat,len:0nat,next:188nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_ok_ct3_d4_il0_9x10().subrange(101,172)==z_ok_ct3_d4_il0_9x10()) by (compute);
    walk_ok_ct3_d4_il0_9x10(bytes_ok_ct3_d4_il0_9x10());
    inflate_ok_ct3_d4_il0_9x10();scan_ok_ct3_d4_il0_9x10();
    assert(r::raw(a_ok_ct3_d4_il0_9x10())==60nat) by (compute);
    assert(r::data(z_ok_ct3_d4_il0_9x10(),a_ok_ct3_d4_il0_9x10(),STRICT)==Some(Ok(()))) by {reveal(r::data);};
    r::default_ready(bytes_ok_ct3_d4_il0_9x10(),h_ok_ct3_d4_il0_9x10(),z_ok_ct3_d4_il0_9x10(),16nat,false);
}
fn public_ok_ct3_d4_il0_9x10() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,9u8,0u8,0u8,0u8,10u8,4u8,3u8,0u8,0u8,0u8,148u8,43u8,105u8,141u8,0u8,0u8,0u8,48u8,80u8,76u8,84u8,69u8,202u8,150u8,248u8,13u8,189u8,118u8,4u8,194u8,222u8,220u8,59u8,247u8,50u8,80u8,152u8,170u8,177u8,51u8,229u8,86u8,57u8,245u8,189u8,200u8,134u8,123u8,104u8,1u8,245u8,91u8,51u8,240u8,201u8,161u8,9u8,28u8,39u8,68u8,94u8,185u8,21u8,142u8,217u8,246u8,59u8,128u8,196u8,211u8,231u8,78u8,42u8,174u8,0u8,0u8,0u8,71u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,60u8,0u8,195u8,255u8,0u8,239u8,181u8,68u8,141u8,208u8,1u8,134u8,18u8,238u8,65u8,249u8,2u8,142u8,2u8,98u8,21u8,112u8,3u8,227u8,54u8,224u8,153u8,161u8,4u8,150u8,36u8,99u8,172u8,16u8,0u8,51u8,161u8,146u8,252u8,64u8,1u8,188u8,172u8,97u8,71u8,144u8,2u8,87u8,122u8,67u8,231u8,240u8,3u8,141u8,167u8,228u8,226u8,243u8,4u8,229u8,161u8,163u8,201u8,48u8,26u8,132u8,27u8,217u8,199u8,153u8,6u8,91u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_ok_ct3_d4_il0_9x10());model_ok_ct3_d4_il0_9x10();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==9u32 && s.height==10u32 && s.rgba@.len()==360nat,Err(crate::alloc_guard::TryError::OutOfMemory(_))=>true,_=>false});
}
// ok16/ct3-d8-il1-7x5; frozen SHA256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_ok16_ct3_d8_il1_7x5() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,7u8,0u8,0u8,0u8,5u8,8u8,3u8,0u8,0u8,1u8,201u8,67u8,54u8,124u8,0u8,0u8,2u8,85u8,80u8,76u8,84u8,69u8,200u8,194u8,15u8,93u8,249u8,99u8,72u8,242u8,195u8,209u8,69u8,114u8,44u8,240u8,56u8,43u8,232u8,140u8,35u8,230u8,228u8,130u8,60u8,200u8,187u8,168u8,40u8,154u8,115u8,86u8,56u8,50u8,42u8,0u8,58u8,251u8,250u8,13u8,231u8,27u8,59u8,102u8,230u8,199u8,245u8,48u8,15u8,138u8,234u8,35u8,251u8,127u8,48u8,154u8,131u8,0u8,242u8,214u8,179u8,246u8,32u8,55u8,75u8,163u8,80u8,95u8,116u8,74u8,180u8,33u8,240u8,74u8,171u8,98u8,26u8,110u8,20u8,33u8,139u8,6u8,85u8,39u8,236u8,7u8,180u8,171u8,65u8,32u8,220u8,126u8,252u8,5u8,90u8,56u8,24u8,11u8,231u8,34u8,32u8,174u8,91u8,140u8,91u8,81u8,59u8,36u8,63u8,8u8,237u8,97u8,78u8,36u8,178u8,173u8,101u8,52u8,162u8,15u8,150u8,137u8,33u8,137u8,216u8,130u8,221u8,218u8,85u8,164u8,237u8,15u8,187u8,119u8,7u8,251u8,29u8,245u8,96u8,145u8,140u8,161u8,133u8,83u8,120u8,105u8,159u8,44u8,225u8,10u8,194u8,88u8,132u8,121u8,253u8,41u8,221u8,146u8,147u8,161u8,240u8,112u8,188u8,177u8,197u8,227u8,86u8,151u8,66u8,118u8,202u8,89u8,158u8,142u8,103u8,89u8,192u8,239u8,199u8,69u8,131u8,207u8,14u8,168u8,73u8,30u8,165u8,36u8,26u8,81u8,167u8,93u8,185u8,154u8,112u8,6u8,169u8,105u8,85u8,238u8,228u8,102u8,92u8,236u8,208u8,161u8,57u8,236u8,217u8,53u8,135u8,208u8,132u8,80u8,121u8,122u8,27u8,51u8,107u8,38u8,126u8,175u8,85u8,129u8,197u8,37u8,25u8,212u8,208u8,200u8,108u8,131u8,93u8,21u8,5u8,68u8,88u8,246u8,172u8,45u8,196u8,142u8,187u8,95u8,141u8,30u8,7u8,166u8,220u8,86u8,226u8,16u8,58u8,126u8,226u8,127u8,170u8,188u8,160u8,82u8,200u8,254u8,185u8,171u8,40u8,78u8,173u8,185u8,165u8,171u8,98u8,14u8,186u8,141u8,134u8,158u8,42u8,114u8,208u8,96u8,2u8,161u8,56u8,34u8,129u8,252u8,229u8,91u8,62u8,123u8,174u8,254u8,130u8,88u8,152u8,134u8,249u8,84u8,180u8,208u8,111u8,155u8,4u8,10u8,127u8,20u8,163u8,147u8,170u8,65u8,206u8,59u8,184u8,224u8,204u8,87u8,98u8,246u8,122u8,241u8,65u8,5u8,188u8,52u8,102u8,246u8,38u8,197u8,190u8,156u8,49u8,193u8,22u8,36u8,247u8,10u8,69u8,187u8,11u8,112u8,199u8,18u8,16u8,146u8,75u8,99u8,117u8,49u8,143u8,146u8,155u8,31u8,89u8,251u8,239u8,159u8,76u8,171u8,108u8,252u8,82u8,187u8,44u8,182u8,7u8,209u8,60u8,35u8,60u8,57u8,165u8,218u8,238u8,54u8,2u8,83u8,45u8,127u8,44u8,248u8,174u8,179u8,72u8,89u8,41u8,173u8,130u8,178u8,41u8,146u8,1u8,246u8,87u8,78u8,243u8,121u8,2u8,212u8,53u8,54u8,125u8,57u8,67u8,114u8,157u8,199u8,109u8,152u8,135u8,27u8,29u8,245u8,6u8,130u8,9u8,101u8,60u8,86u8,8u8,109u8,200u8,61u8,221u8,219u8,157u8,112u8,170u8,157u8,22u8,246u8,106u8,166u8,210u8,150u8,207u8,218u8,42u8,242u8,154u8,189u8,226u8,84u8,43u8,26u8,93u8,115u8,209u8,121u8,172u8,83u8,103u8,185u8,83u8,34u8,57u8,85u8,141u8,55u8,95u8,173u8,153u8,234u8,58u8,99u8,63u8,79u8,99u8,192u8,211u8,139u8,74u8,99u8,134u8,214u8,234u8,151u8,23u8,56u8,221u8,159u8,15u8,38u8,69u8,242u8,161u8,130u8,173u8,33u8,8u8,62u8,24u8,55u8,5u8,107u8,154u8,133u8,242u8,205u8,125u8,17u8,170u8,251u8,187u8,92u8,241u8,201u8,144u8,68u8,191u8,151u8,187u8,143u8,215u8,10u8,165u8,254u8,174u8,42u8,129u8,240u8,18u8,13u8,64u8,207u8,52u8,139u8,202u8,32u8,234u8,223u8,19u8,30u8,173u8,192u8,255u8,160u8,139u8,54u8,220u8,122u8,251u8,102u8,140u8,5u8,39u8,207u8,136u8,242u8,254u8,1u8,133u8,83u8,216u8,236u8,67u8,154u8,237u8,153u8,199u8,51u8,230u8,147u8,120u8,153u8,207u8,68u8,199u8,160u8,145u8,222u8,37u8,178u8,33u8,166u8,132u8,181u8,139u8,126u8,153u8,107u8,232u8,105u8,219u8,160u8,179u8,66u8,99u8,132u8,243u8,11u8,32u8,198u8,43u8,199u8,142u8,44u8,168u8,16u8,202u8,178u8,56u8,159u8,49u8,0u8,0u8,0u8,55u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,152u8,207u8,232u8,195u8,100u8,166u8,204u8,236u8,109u8,204u8,18u8,56u8,141u8,33u8,126u8,117u8,141u8,38u8,99u8,207u8,204u8,151u8,76u8,77u8,79u8,19u8,152u8,213u8,202u8,47u8,178u8,232u8,124u8,149u8,248u8,114u8,59u8,212u8,156u8,193u8,159u8,205u8,96u8,145u8,85u8,170u8,0u8,0u8,70u8,150u8,15u8,26u8,77u8,158u8,109u8,110u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn h_ok16_ct3_d8_il1_7x5() -> crate::model::Ihdr {crate::model::Ihdr {width:7u32,height:5u32,depth:8u8,color_type:3u8,interlace:1u8}}
spec fn a_ok16_ct3_d8_il1_7x5() -> r::Raster {r::shape(h_ok16_ct3_d8_il1_7x5(),7nat,5nat,199nat,STRICT)}
spec fn z_ok16_ct3_d8_il1_7x5() -> Seq<u8> {seq![120u8,156u8,99u8,152u8,207u8,232u8,195u8,100u8,166u8,204u8,236u8,109u8,204u8,18u8,56u8,141u8,33u8,126u8,117u8,141u8,38u8,99u8,207u8,204u8,151u8,76u8,77u8,79u8,19u8,152u8,213u8,202u8,47u8,178u8,232u8,124u8,149u8,248u8,114u8,59u8,212u8,156u8,193u8,159u8,205u8,96u8,145u8,85u8,170u8,0u8,0u8,70u8,150u8,15u8,26u8]}
spec fn raw_ok16_ct3_d8_il1_7x5() -> Seq<u8> {seq![0u8,159u8,1u8,76u8,2u8,54u8,35u8,3u8,75u8,51u8,4u8,81u8,150u8,0u8,95u8,171u8,124u8,41u8,1u8,140u8,153u8,233u8,2u8,130u8,229u8,96u8,3u8,38u8,119u8,209u8,4u8,44u8,245u8,24u8,244u8,219u8,85u8,55u8,0u8,79u8,6u8,48u8,162u8,58u8,101u8,16u8]}
spec fn ts_ok16_ct3_d8_il1_7x5() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:36nat,before:1nat,after:2nat,symbol:159nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:36nat,next:44nat,before:2nat,after:3nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:44nat,next:52nat,before:3nat,after:4nat,symbol:76nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:52nat,next:60nat,before:4nat,after:5nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:60nat,next:68nat,before:5nat,after:6nat,symbol:54nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:68nat,next:76nat,before:6nat,after:7nat,symbol:35nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:76nat,next:84nat,before:7nat,after:8nat,symbol:3nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:84nat,next:92nat,before:8nat,after:9nat,symbol:75nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:92nat,next:100nat,before:9nat,after:10nat,symbol:51nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:100nat,next:108nat,before:10nat,after:11nat,symbol:4nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:108nat,next:116nat,before:11nat,after:12nat,symbol:81nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:116nat,next:125nat,before:12nat,after:13nat,symbol:150nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:125nat,next:133nat,before:13nat,after:14nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:133nat,next:141nat,before:14nat,after:15nat,symbol:95nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:141nat,next:150nat,before:15nat,after:16nat,symbol:171nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:150nat,next:158nat,before:16nat,after:17nat,symbol:124nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:158nat,next:166nat,before:17nat,after:18nat,symbol:41nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:166nat,next:174nat,before:18nat,after:19nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:174nat,next:182nat,before:19nat,after:20nat,symbol:140nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:182nat,next:191nat,before:20nat,after:21nat,symbol:153nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:191nat,next:200nat,before:21nat,after:22nat,symbol:233nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:200nat,next:208nat,before:22nat,after:23nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:208nat,next:216nat,before:23nat,after:24nat,symbol:130nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:216nat,next:225nat,before:24nat,after:25nat,symbol:229nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:225nat,next:233nat,before:25nat,after:26nat,symbol:96nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:233nat,next:241nat,before:26nat,after:27nat,symbol:3nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:241nat,next:249nat,before:27nat,after:28nat,symbol:38nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:249nat,next:257nat,before:28nat,after:29nat,symbol:119nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:257nat,next:266nat,before:29nat,after:30nat,symbol:209nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:266nat,next:274nat,before:30nat,after:31nat,symbol:4nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:274nat,next:282nat,before:31nat,after:32nat,symbol:44nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:282nat,next:291nat,before:32nat,after:33nat,symbol:245nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:291nat,next:299nat,before:33nat,after:34nat,symbol:24nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:299nat,next:308nat,before:34nat,after:35nat,symbol:244nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:308nat,next:317nat,before:35nat,after:36nat,symbol:219nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:317nat,next:325nat,before:36nat,after:37nat,symbol:85nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:325nat,next:333nat,before:37nat,after:38nat,symbol:55nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:333nat,next:341nat,before:38nat,after:39nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:341nat,next:349nat,before:39nat,after:40nat,symbol:79nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:349nat,next:357nat,before:40nat,after:41nat,symbol:6nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:357nat,next:365nat,before:41nat,after:42nat,symbol:48nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:365nat,next:374nat,before:42nat,after:43nat,symbol:162nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:374nat,next:382nat,before:43nat,after:44nat,symbol:58nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:382nat,next:390nat,before:44nat,after:45nat,symbol:101nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:390nat,next:398nat,before:45nat,after:46nat,symbol:16nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:398nat,next:405nat,before:46nat,after:46nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_ok16_ct3_d8_il1_7x5_0() ensures t::trace(z_ok16_ct3_d8_il1_7x5(),raw_ok16_ct3_d8_il1_7x5(),ts_ok16_ct3_d8_il1_7x5(),0nat,16nat),
{
    hide(t::valid_step);assert(t::trace(z_ok16_ct3_d8_il1_7x5(),raw_ok16_ct3_d8_il1_7x5(),ts_ok16_ct3_d8_il1_7x5(),0nat,16nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn trace_ok16_ct3_d8_il1_7x5_1() ensures t::trace(z_ok16_ct3_d8_il1_7x5(),raw_ok16_ct3_d8_il1_7x5(),ts_ok16_ct3_d8_il1_7x5(),16nat,32nat),
{
    hide(t::valid_step);assert(t::trace(z_ok16_ct3_d8_il1_7x5(),raw_ok16_ct3_d8_il1_7x5(),ts_ok16_ct3_d8_il1_7x5(),16nat,32nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn trace_ok16_ct3_d8_il1_7x5_2() ensures t::trace(z_ok16_ct3_d8_il1_7x5(),raw_ok16_ct3_d8_il1_7x5(),ts_ok16_ct3_d8_il1_7x5(),32nat,47nat),
{
    hide(t::valid_step);assert(t::trace(z_ok16_ct3_d8_il1_7x5(),raw_ok16_ct3_d8_il1_7x5(),ts_ok16_ct3_d8_il1_7x5(),32nat,47nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn trace_ok16_ct3_d8_il1_7x5() ensures t::trace(z_ok16_ct3_d8_il1_7x5(),raw_ok16_ct3_d8_il1_7x5(),ts_ok16_ct3_d8_il1_7x5(),0,47nat),
{
    hide(z_ok16_ct3_d8_il1_7x5);hide(raw_ok16_ct3_d8_il1_7x5);hide(ts_ok16_ct3_d8_il1_7x5);hide(t::trace);
    trace_ok16_ct3_d8_il1_7x5_0();
    trace_ok16_ct3_d8_il1_7x5_1();
    trace_ok16_ct3_d8_il1_7x5_2();
    assert(t::trace(z_ok16_ct3_d8_il1_7x5(),raw_ok16_ct3_d8_il1_7x5(),ts_ok16_ct3_d8_il1_7x5(),0,47nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_ok16_ct3_d8_il1_7x5() ensures z_ok16_ct3_d8_il1_7x5().len()==55nat,
        raw_ok16_ct3_d8_il1_7x5().len()==46nat,
        z_ok16_ct3_d8_il1_7x5()[0]==120u8,
        z_ok16_ct3_d8_il1_7x5()[1]==156u8,
        zm::field_value(z_ok16_ct3_d8_il1_7x5(),16,1)==1nat,
        zm::field_value(z_ok16_ct3_d8_il1_7x5(),17,2)==1nat,
        zm::be_value(z_ok16_ct3_d8_il1_7x5(),51nat,4)==1184239386nat,
        ts_ok16_ct3_d8_il1_7x5().len()==47nat,
        ts_ok16_ct3_d8_il1_7x5()[0].pos==19nat,
        ts_ok16_ct3_d8_il1_7x5()[0].before==0nat,
        ts_ok16_ct3_d8_il1_7x5().last().next==405nat,
{
    assert(z_ok16_ct3_d8_il1_7x5().len()==55nat) by (compute);
    assert(raw_ok16_ct3_d8_il1_7x5().len()==46nat) by (compute);
    assert(z_ok16_ct3_d8_il1_7x5()[0]==120u8) by (compute);
    assert(z_ok16_ct3_d8_il1_7x5()[1]==156u8) by (compute);
    assert(zm::field_value(z_ok16_ct3_d8_il1_7x5(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_ok16_ct3_d8_il1_7x5(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_ok16_ct3_d8_il1_7x5(),51nat,4)==1184239386nat) by (compute);
    assert(ts_ok16_ct3_d8_il1_7x5().len()==47nat) by (compute);
    assert(ts_ok16_ct3_d8_il1_7x5()[0].pos==19nat) by (compute);
    assert(ts_ok16_ct3_d8_il1_7x5()[0].before==0nat) by (compute);
    assert(ts_ok16_ct3_d8_il1_7x5().last().next==405nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_ok16_ct3_d8_il1_7x5() ensures zm::Adler(raw_ok16_ct3_d8_il1_7x5())==1184239386nat,
{
    hide(raw_ok16_ct3_d8_il1_7x5);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_ok16_ct3_d8_il1_7x5().len()==46nat) by (compute);
    assert(c::sums(raw_ok16_ct3_d8_il1_7x5(),0,46nat)==(3865nat,83591nat)) by (compute);
    c::sums_correct(raw_ok16_ct3_d8_il1_7x5(),0,46nat);assert(raw_ok16_ct3_d8_il1_7x5().subrange(0,46)=~=raw_ok16_ct3_d8_il1_7x5());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((83591nat%65521)*(u16::MAX as nat+1)+(1+3865nat)%65521==1184239386nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_ok16_ct3_d8_il1_7x5() ensures zm::Inflate(z_ok16_ct3_d8_il1_7x5(),46nat,true,false)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_ok16_ct3_d8_il1_7x5(),r2:false}),raw_ok16_ct3_d8_il1_7x5().len()==46nat,
{
    hide(z_ok16_ct3_d8_il1_7x5);hide(raw_ok16_ct3_d8_il1_7x5);hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);hide(ts_ok16_ct3_d8_il1_7x5);
    shape_ok16_ct3_d8_il1_7x5();checksum_ok16_ct3_d8_il1_7x5();
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_ok16_ct3_d8_il1_7x5();ms::fixed_stream(z_ok16_ct3_d8_il1_7x5(),raw_ok16_ct3_d8_il1_7x5(),ts_ok16_ct3_d8_il1_7x5(),46nat,156nat,1184239386nat,1184239386nat);
}
#[verifier::spinoff_prover]
proof fn crc_ok16_ct3_d8_il1_7x5_crc_33()
    ensures crate::model::crc(bytes_ok16_ct3_d8_il1_7x5().subrange(37,638))==2990055217u32,
{
    hide(crate::model::crc_register);hide(crc_cert::fold);
    assert(crate::model::crc_register(bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(0))==0xffff_ffffu32) by {
        assert(bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(0) =~= Seq::<u8>::empty());
        reveal_with_fuel(crate::model::crc_register,1);
    }
    assert(crc_cert::fold(4294967295u32,bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).subrange(0,64))==3973125428u32) by (compute);
    crc_cert::suffix(bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(0),bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).subrange(0,64));
    assert(bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(0)+bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).subrange(0,64) =~= bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(64));
    assert(crate::model::crc_register(bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(64))==3973125428u32);
    assert(crc_cert::fold(3973125428u32,bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).subrange(64,128))==1200274943u32) by (compute);
    crc_cert::suffix(bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(64),bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).subrange(64,128));
    assert(bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(64)+bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).subrange(64,128) =~= bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(128));
    assert(crate::model::crc_register(bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(128))==1200274943u32);
    assert(crc_cert::fold(1200274943u32,bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).subrange(128,192))==4145548682u32) by (compute);
    crc_cert::suffix(bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(128),bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).subrange(128,192));
    assert(bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(128)+bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).subrange(128,192) =~= bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(192));
    assert(crate::model::crc_register(bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(192))==4145548682u32);
    assert(crc_cert::fold(4145548682u32,bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).subrange(192,256))==1631215492u32) by (compute);
    crc_cert::suffix(bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(192),bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).subrange(192,256));
    assert(bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(192)+bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).subrange(192,256) =~= bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(256));
    assert(crate::model::crc_register(bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(256))==1631215492u32);
    assert(crc_cert::fold(1631215492u32,bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).subrange(256,320))==4145161596u32) by (compute);
    crc_cert::suffix(bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(256),bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).subrange(256,320));
    assert(bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(256)+bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).subrange(256,320) =~= bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(320));
    assert(crate::model::crc_register(bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(320))==4145161596u32);
    assert(crc_cert::fold(4145161596u32,bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).subrange(320,384))==1332434372u32) by (compute);
    crc_cert::suffix(bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(320),bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).subrange(320,384));
    assert(bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(320)+bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).subrange(320,384) =~= bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(384));
    assert(crate::model::crc_register(bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(384))==1332434372u32);
    assert(crc_cert::fold(1332434372u32,bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).subrange(384,448))==2267397047u32) by (compute);
    crc_cert::suffix(bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(384),bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).subrange(384,448));
    assert(bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(384)+bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).subrange(384,448) =~= bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(448));
    assert(crate::model::crc_register(bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(448))==2267397047u32);
    assert(crc_cert::fold(2267397047u32,bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).subrange(448,512))==634564636u32) by (compute);
    crc_cert::suffix(bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(448),bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).subrange(448,512));
    assert(bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(448)+bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).subrange(448,512) =~= bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(512));
    assert(crate::model::crc_register(bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(512))==634564636u32);
    assert(crc_cert::fold(634564636u32,bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).subrange(512,576))==2539273757u32) by (compute);
    crc_cert::suffix(bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(512),bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).subrange(512,576));
    assert(bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(512)+bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).subrange(512,576) =~= bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(576));
    assert(crate::model::crc_register(bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(576))==2539273757u32);
    assert(crc_cert::fold(2539273757u32,bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).subrange(576,601))==1304912078u32) by (compute);
    crc_cert::suffix(bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(576),bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).subrange(576,601));
    assert(bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(576)+bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).subrange(576,601) =~= bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(601));
    assert(crate::model::crc_register(bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(601))==1304912078u32);
    assert(bytes_ok16_ct3_d8_il1_7x5().subrange(37,638).take(601) =~= bytes_ok16_ct3_d8_il1_7x5().subrange(37,638));
    assert((1304912078u32 ^ 0xffff_ffffu32)==2990055217u32) by (compute);
}
#[verifier::spinoff_prover]
proof fn walk_ok16_ct3_d8_il1_7x5(b: Seq<u8>)
    requires b.len()==721,
        crate::model::mh(b)==Ok((h_ok16_ct3_d8_il1_7x5(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:597nat,next:642nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,642nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:650nat,len:55nat,next:709nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,709nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:717nat,len:0nat,next:721nat,crc_ok:true,synthetic:false})),
        b.subrange(650,705)==z_ok16_ct3_d8_il1_7x5(),
    ensures r::default_case(b,h_ok16_ct3_d8_il1_7x5(),z_ok16_ct3_d8_il1_7x5(),199nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(597nat,3u8,8u8,false,false,false,false)==crate::metadata_model::Decision::Keep(199nat)) by (compute);
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
proof fn scan_ok16_ct3_d8_il1_7x5() ensures r::scan(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),0)==Ok(()),
{
    hide(raw_ok16_ct3_d8_il1_7x5);assert(raw_ok16_ct3_d8_il1_7x5().len()==46nat) by (compute);assert(r::count(a_ok16_ct3_d8_il1_7x5())==7nat) by (compute);
    assert(r::pw(a_ok16_ct3_d8_il1_7x5(),0nat)==1nat && r::ph(a_ok16_ct3_d8_il1_7x5(),0nat)==1nat && r::rb(a_ok16_ct3_d8_il1_7x5(),0nat)==1nat && r::base(a_ok16_ct3_d8_il1_7x5(),0nat)==0nat) by (compute);
    assert(raw_ok16_ct3_d8_il1_7x5()[0]==0u8) by (compute);
    f::pass_row_equation(raw_ok16_ct3_d8_il1_7x5(),0nat,1nat,1nat,0nat);
    assert(f::previous_row(raw_ok16_ct3_d8_il1_7x5(),0nat,1nat,1nat,0nat)==None);
    assert(f::encoded_row(0u8,f::row_data(raw_ok16_ct3_d8_il1_7x5(),0nat,1nat,0nat),None,1nat,seq![159u8],0,1nat)) by (compute);
    f::checked_row_hint(0u8,f::row_data(raw_ok16_ct3_d8_il1_7x5(),0nat,1nat,0nat),None,1nat,seq![159u8]);
    assert(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),0nat,0nat)==seq![159u8]);
    assert forall|x: nat| x<1 implies (#[trigger] r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),0nat,0nat),8u8,x) as nat)<199 by {
        if x==0 {assert((r::palette_sample(seq![159u8],8u8,0nat) as nat)<199nat) by (compute);assert((r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),0nat,0nat),8u8,0nat) as nat)<199nat);}
    }
    assert(r::palette_valid(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),0nat,0nat),8u8,1nat,199nat));
    assert(r::row_result(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),0nat,0nat)==Ok(()));
    assert(r::pw(a_ok16_ct3_d8_il1_7x5(),1nat)==1nat && r::ph(a_ok16_ct3_d8_il1_7x5(),1nat)==1nat && r::rb(a_ok16_ct3_d8_il1_7x5(),1nat)==1nat && r::base(a_ok16_ct3_d8_il1_7x5(),1nat)==2nat) by (compute);
    assert(raw_ok16_ct3_d8_il1_7x5()[2]==1u8) by (compute);
    f::pass_row_equation(raw_ok16_ct3_d8_il1_7x5(),2nat,1nat,1nat,0nat);
    assert(f::previous_row(raw_ok16_ct3_d8_il1_7x5(),2nat,1nat,1nat,0nat)==None);
    assert(f::encoded_row(1u8,f::row_data(raw_ok16_ct3_d8_il1_7x5(),2nat,1nat,0nat),None,1nat,seq![76u8],0,1nat)) by (compute);
    f::checked_row_hint(1u8,f::row_data(raw_ok16_ct3_d8_il1_7x5(),2nat,1nat,0nat),None,1nat,seq![76u8]);
    assert(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),1nat,0nat)==seq![76u8]);
    assert forall|x: nat| x<1 implies (#[trigger] r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),1nat,0nat),8u8,x) as nat)<199 by {
        if x==0 {assert((r::palette_sample(seq![76u8],8u8,0nat) as nat)<199nat) by (compute);assert((r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),1nat,0nat),8u8,0nat) as nat)<199nat);}
    }
    assert(r::palette_valid(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),1nat,0nat),8u8,1nat,199nat));
    assert(r::row_result(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),1nat,0nat)==Ok(()));
    assert(r::pw(a_ok16_ct3_d8_il1_7x5(),2nat)==2nat && r::ph(a_ok16_ct3_d8_il1_7x5(),2nat)==1nat && r::rb(a_ok16_ct3_d8_il1_7x5(),2nat)==2nat && r::base(a_ok16_ct3_d8_il1_7x5(),2nat)==4nat) by (compute);
    assert(raw_ok16_ct3_d8_il1_7x5()[4]==2u8) by (compute);
    f::pass_row_equation(raw_ok16_ct3_d8_il1_7x5(),4nat,2nat,1nat,0nat);
    assert(f::previous_row(raw_ok16_ct3_d8_il1_7x5(),4nat,2nat,1nat,0nat)==None);
    assert(f::encoded_row(2u8,f::row_data(raw_ok16_ct3_d8_il1_7x5(),4nat,2nat,0nat),None,1nat,seq![54u8,35u8],0,2nat)) by (compute);
    f::checked_row_hint(2u8,f::row_data(raw_ok16_ct3_d8_il1_7x5(),4nat,2nat,0nat),None,1nat,seq![54u8,35u8]);
    assert(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),2nat,0nat)==seq![54u8,35u8]);
    assert forall|x: nat| x<2 implies (#[trigger] r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),2nat,0nat),8u8,x) as nat)<199 by {
        if x==0 {assert((r::palette_sample(seq![54u8,35u8],8u8,0nat) as nat)<199nat) by (compute);assert((r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),2nat,0nat),8u8,0nat) as nat)<199nat);}
        else if x==1 {assert((r::palette_sample(seq![54u8,35u8],8u8,1nat) as nat)<199nat) by (compute);assert((r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),2nat,0nat),8u8,1nat) as nat)<199nat);}
    }
    assert(r::palette_valid(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),2nat,0nat),8u8,2nat,199nat));
    assert(r::row_result(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),2nat,0nat)==Ok(()));
    assert(r::pw(a_ok16_ct3_d8_il1_7x5(),3nat)==2nat && r::ph(a_ok16_ct3_d8_il1_7x5(),3nat)==2nat && r::rb(a_ok16_ct3_d8_il1_7x5(),3nat)==2nat && r::base(a_ok16_ct3_d8_il1_7x5(),3nat)==7nat) by (compute);
    assert(raw_ok16_ct3_d8_il1_7x5()[7]==3u8) by (compute);
    f::pass_row_equation(raw_ok16_ct3_d8_il1_7x5(),7nat,2nat,1nat,0nat);
    assert(f::previous_row(raw_ok16_ct3_d8_il1_7x5(),7nat,2nat,1nat,0nat)==None);
    assert(f::encoded_row(3u8,f::row_data(raw_ok16_ct3_d8_il1_7x5(),7nat,2nat,0nat),None,1nat,seq![75u8,88u8],0,2nat)) by (compute);
    f::checked_row_hint(3u8,f::row_data(raw_ok16_ct3_d8_il1_7x5(),7nat,2nat,0nat),None,1nat,seq![75u8,88u8]);
    assert(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),3nat,0nat)==seq![75u8,88u8]);
    assert forall|x: nat| x<2 implies (#[trigger] r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),3nat,0nat),8u8,x) as nat)<199 by {
        if x==0 {assert((r::palette_sample(seq![75u8,88u8],8u8,0nat) as nat)<199nat) by (compute);assert((r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),3nat,0nat),8u8,0nat) as nat)<199nat);}
        else if x==1 {assert((r::palette_sample(seq![75u8,88u8],8u8,1nat) as nat)<199nat) by (compute);assert((r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),3nat,0nat),8u8,1nat) as nat)<199nat);}
    }
    assert(r::palette_valid(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),3nat,0nat),8u8,2nat,199nat));
    assert(r::row_result(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),3nat,0nat)==Ok(()));
    assert(raw_ok16_ct3_d8_il1_7x5()[10]==4u8) by (compute);
    f::pass_row_equation(raw_ok16_ct3_d8_il1_7x5(),7nat,2nat,1nat,1nat);
    assert(f::previous_row(raw_ok16_ct3_d8_il1_7x5(),7nat,2nat,1nat,1nat)==Some(seq![75u8,88u8]));
    assert(f::encoded_row(4u8,f::row_data(raw_ok16_ct3_d8_il1_7x5(),7nat,2nat,1nat),Some(seq![75u8,88u8]),1nat,seq![156u8,50u8],0,2nat)) by (compute);
    f::checked_row_hint(4u8,f::row_data(raw_ok16_ct3_d8_il1_7x5(),7nat,2nat,1nat),Some(seq![75u8,88u8]),1nat,seq![156u8,50u8]);
    assert(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),3nat,1nat)==seq![156u8,50u8]);
    assert forall|x: nat| x<2 implies (#[trigger] r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),3nat,1nat),8u8,x) as nat)<199 by {
        if x==0 {assert((r::palette_sample(seq![156u8,50u8],8u8,0nat) as nat)<199nat) by (compute);assert((r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),3nat,1nat),8u8,0nat) as nat)<199nat);}
        else if x==1 {assert((r::palette_sample(seq![156u8,50u8],8u8,1nat) as nat)<199nat) by (compute);assert((r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),3nat,1nat),8u8,1nat) as nat)<199nat);}
    }
    assert(r::palette_valid(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),3nat,1nat),8u8,2nat,199nat));
    assert(r::row_result(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),3nat,1nat)==Ok(()));
    assert(r::pw(a_ok16_ct3_d8_il1_7x5(),4nat)==4nat && r::ph(a_ok16_ct3_d8_il1_7x5(),4nat)==1nat && r::rb(a_ok16_ct3_d8_il1_7x5(),4nat)==4nat && r::base(a_ok16_ct3_d8_il1_7x5(),4nat)==13nat) by (compute);
    assert(raw_ok16_ct3_d8_il1_7x5()[13]==0u8) by (compute);
    f::pass_row_equation(raw_ok16_ct3_d8_il1_7x5(),13nat,4nat,1nat,0nat);
    assert(f::previous_row(raw_ok16_ct3_d8_il1_7x5(),13nat,4nat,1nat,0nat)==None);
    assert(f::encoded_row(0u8,f::row_data(raw_ok16_ct3_d8_il1_7x5(),13nat,4nat,0nat),None,1nat,seq![95u8,171u8,124u8,41u8],0,4nat)) by (compute);
    f::checked_row_hint(0u8,f::row_data(raw_ok16_ct3_d8_il1_7x5(),13nat,4nat,0nat),None,1nat,seq![95u8,171u8,124u8,41u8]);
    assert(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),4nat,0nat)==seq![95u8,171u8,124u8,41u8]);
    assert forall|x: nat| x<4 implies (#[trigger] r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),4nat,0nat),8u8,x) as nat)<199 by {
        if x==0 {assert((r::palette_sample(seq![95u8,171u8,124u8,41u8],8u8,0nat) as nat)<199nat) by (compute);assert((r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),4nat,0nat),8u8,0nat) as nat)<199nat);}
        else if x==1 {assert((r::palette_sample(seq![95u8,171u8,124u8,41u8],8u8,1nat) as nat)<199nat) by (compute);assert((r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),4nat,0nat),8u8,1nat) as nat)<199nat);}
        else if x==2 {assert((r::palette_sample(seq![95u8,171u8,124u8,41u8],8u8,2nat) as nat)<199nat) by (compute);assert((r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),4nat,0nat),8u8,2nat) as nat)<199nat);}
        else if x==3 {assert((r::palette_sample(seq![95u8,171u8,124u8,41u8],8u8,3nat) as nat)<199nat) by (compute);assert((r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),4nat,0nat),8u8,3nat) as nat)<199nat);}
    }
    assert(r::palette_valid(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),4nat,0nat),8u8,4nat,199nat));
    assert(r::row_result(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),4nat,0nat)==Ok(()));
    assert(r::pw(a_ok16_ct3_d8_il1_7x5(),5nat)==3nat && r::ph(a_ok16_ct3_d8_il1_7x5(),5nat)==3nat && r::rb(a_ok16_ct3_d8_il1_7x5(),5nat)==3nat && r::base(a_ok16_ct3_d8_il1_7x5(),5nat)==18nat) by (compute);
    assert(raw_ok16_ct3_d8_il1_7x5()[18]==1u8) by (compute);
    f::pass_row_equation(raw_ok16_ct3_d8_il1_7x5(),18nat,3nat,1nat,0nat);
    assert(f::previous_row(raw_ok16_ct3_d8_il1_7x5(),18nat,3nat,1nat,0nat)==None);
    assert(f::encoded_row(1u8,f::row_data(raw_ok16_ct3_d8_il1_7x5(),18nat,3nat,0nat),None,1nat,seq![140u8,37u8,14u8],0,3nat)) by (compute);
    f::checked_row_hint(1u8,f::row_data(raw_ok16_ct3_d8_il1_7x5(),18nat,3nat,0nat),None,1nat,seq![140u8,37u8,14u8]);
    assert(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),5nat,0nat)==seq![140u8,37u8,14u8]);
    assert forall|x: nat| x<3 implies (#[trigger] r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),5nat,0nat),8u8,x) as nat)<199 by {
        if x==0 {assert((r::palette_sample(seq![140u8,37u8,14u8],8u8,0nat) as nat)<199nat) by (compute);assert((r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),5nat,0nat),8u8,0nat) as nat)<199nat);}
        else if x==1 {assert((r::palette_sample(seq![140u8,37u8,14u8],8u8,1nat) as nat)<199nat) by (compute);assert((r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),5nat,0nat),8u8,1nat) as nat)<199nat);}
        else if x==2 {assert((r::palette_sample(seq![140u8,37u8,14u8],8u8,2nat) as nat)<199nat) by (compute);assert((r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),5nat,0nat),8u8,2nat) as nat)<199nat);}
    }
    assert(r::palette_valid(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),5nat,0nat),8u8,3nat,199nat));
    assert(r::row_result(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),5nat,0nat)==Ok(()));
    assert(raw_ok16_ct3_d8_il1_7x5()[22]==2u8) by (compute);
    f::pass_row_equation(raw_ok16_ct3_d8_il1_7x5(),18nat,3nat,1nat,1nat);
    assert(f::previous_row(raw_ok16_ct3_d8_il1_7x5(),18nat,3nat,1nat,1nat)==Some(seq![140u8,37u8,14u8]));
    assert(f::encoded_row(2u8,f::row_data(raw_ok16_ct3_d8_il1_7x5(),18nat,3nat,1nat),Some(seq![140u8,37u8,14u8]),1nat,seq![14u8,10u8,110u8],0,3nat)) by (compute);
    f::checked_row_hint(2u8,f::row_data(raw_ok16_ct3_d8_il1_7x5(),18nat,3nat,1nat),Some(seq![140u8,37u8,14u8]),1nat,seq![14u8,10u8,110u8]);
    assert(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),5nat,1nat)==seq![14u8,10u8,110u8]);
    assert forall|x: nat| x<3 implies (#[trigger] r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),5nat,1nat),8u8,x) as nat)<199 by {
        if x==0 {assert((r::palette_sample(seq![14u8,10u8,110u8],8u8,0nat) as nat)<199nat) by (compute);assert((r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),5nat,1nat),8u8,0nat) as nat)<199nat);}
        else if x==1 {assert((r::palette_sample(seq![14u8,10u8,110u8],8u8,1nat) as nat)<199nat) by (compute);assert((r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),5nat,1nat),8u8,1nat) as nat)<199nat);}
        else if x==2 {assert((r::palette_sample(seq![14u8,10u8,110u8],8u8,2nat) as nat)<199nat) by (compute);assert((r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),5nat,1nat),8u8,2nat) as nat)<199nat);}
    }
    assert(r::palette_valid(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),5nat,1nat),8u8,3nat,199nat));
    assert(r::row_result(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),5nat,1nat)==Ok(()));
    assert(raw_ok16_ct3_d8_il1_7x5()[26]==3u8) by (compute);
    f::pass_row_equation(raw_ok16_ct3_d8_il1_7x5(),18nat,3nat,1nat,2nat);
    assert(f::previous_row(raw_ok16_ct3_d8_il1_7x5(),18nat,3nat,1nat,2nat)==Some(seq![14u8,10u8,110u8]));
    assert(f::encoded_row(3u8,f::row_data(raw_ok16_ct3_d8_il1_7x5(),18nat,3nat,2nat),Some(seq![14u8,10u8,110u8]),1nat,seq![45u8,146u8,81u8],0,3nat)) by (compute);
    f::checked_row_hint(3u8,f::row_data(raw_ok16_ct3_d8_il1_7x5(),18nat,3nat,2nat),Some(seq![14u8,10u8,110u8]),1nat,seq![45u8,146u8,81u8]);
    assert(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),5nat,2nat)==seq![45u8,146u8,81u8]);
    assert forall|x: nat| x<3 implies (#[trigger] r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),5nat,2nat),8u8,x) as nat)<199 by {
        if x==0 {assert((r::palette_sample(seq![45u8,146u8,81u8],8u8,0nat) as nat)<199nat) by (compute);assert((r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),5nat,2nat),8u8,0nat) as nat)<199nat);}
        else if x==1 {assert((r::palette_sample(seq![45u8,146u8,81u8],8u8,1nat) as nat)<199nat) by (compute);assert((r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),5nat,2nat),8u8,1nat) as nat)<199nat);}
        else if x==2 {assert((r::palette_sample(seq![45u8,146u8,81u8],8u8,2nat) as nat)<199nat) by (compute);assert((r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),5nat,2nat),8u8,2nat) as nat)<199nat);}
    }
    assert(r::palette_valid(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),5nat,2nat),8u8,3nat,199nat));
    assert(r::row_result(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),5nat,2nat)==Ok(()));
    assert(r::pw(a_ok16_ct3_d8_il1_7x5(),6nat)==7nat && r::ph(a_ok16_ct3_d8_il1_7x5(),6nat)==2nat && r::rb(a_ok16_ct3_d8_il1_7x5(),6nat)==7nat && r::base(a_ok16_ct3_d8_il1_7x5(),6nat)==30nat) by (compute);
    assert(raw_ok16_ct3_d8_il1_7x5()[30]==4u8) by (compute);
    f::pass_row_equation(raw_ok16_ct3_d8_il1_7x5(),30nat,7nat,1nat,0nat);
    assert(f::previous_row(raw_ok16_ct3_d8_il1_7x5(),30nat,7nat,1nat,0nat)==None);
    assert(f::encoded_row(4u8,f::row_data(raw_ok16_ct3_d8_il1_7x5(),30nat,7nat,0nat),None,1nat,seq![44u8,33u8,57u8,45u8,8u8,93u8,148u8],0,7nat)) by (compute);
    f::checked_row_hint(4u8,f::row_data(raw_ok16_ct3_d8_il1_7x5(),30nat,7nat,0nat),None,1nat,seq![44u8,33u8,57u8,45u8,8u8,93u8,148u8]);
    assert(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),6nat,0nat)==seq![44u8,33u8,57u8,45u8,8u8,93u8,148u8]);
    assert forall|x: nat| x<7 implies (#[trigger] r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),6nat,0nat),8u8,x) as nat)<199 by {
        if x==0 {assert((r::palette_sample(seq![44u8,33u8,57u8,45u8,8u8,93u8,148u8],8u8,0nat) as nat)<199nat) by (compute);assert((r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),6nat,0nat),8u8,0nat) as nat)<199nat);}
        else if x==1 {assert((r::palette_sample(seq![44u8,33u8,57u8,45u8,8u8,93u8,148u8],8u8,1nat) as nat)<199nat) by (compute);assert((r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),6nat,0nat),8u8,1nat) as nat)<199nat);}
        else if x==2 {assert((r::palette_sample(seq![44u8,33u8,57u8,45u8,8u8,93u8,148u8],8u8,2nat) as nat)<199nat) by (compute);assert((r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),6nat,0nat),8u8,2nat) as nat)<199nat);}
        else if x==3 {assert((r::palette_sample(seq![44u8,33u8,57u8,45u8,8u8,93u8,148u8],8u8,3nat) as nat)<199nat) by (compute);assert((r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),6nat,0nat),8u8,3nat) as nat)<199nat);}
        else if x==4 {assert((r::palette_sample(seq![44u8,33u8,57u8,45u8,8u8,93u8,148u8],8u8,4nat) as nat)<199nat) by (compute);assert((r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),6nat,0nat),8u8,4nat) as nat)<199nat);}
        else if x==5 {assert((r::palette_sample(seq![44u8,33u8,57u8,45u8,8u8,93u8,148u8],8u8,5nat) as nat)<199nat) by (compute);assert((r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),6nat,0nat),8u8,5nat) as nat)<199nat);}
        else if x==6 {assert((r::palette_sample(seq![44u8,33u8,57u8,45u8,8u8,93u8,148u8],8u8,6nat) as nat)<199nat) by (compute);assert((r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),6nat,0nat),8u8,6nat) as nat)<199nat);}
    }
    assert(r::palette_valid(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),6nat,0nat),8u8,7nat,199nat));
    assert(r::row_result(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),6nat,0nat)==Ok(()));
    assert(raw_ok16_ct3_d8_il1_7x5()[38]==0u8) by (compute);
    f::pass_row_equation(raw_ok16_ct3_d8_il1_7x5(),30nat,7nat,1nat,1nat);
    assert(f::previous_row(raw_ok16_ct3_d8_il1_7x5(),30nat,7nat,1nat,1nat)==Some(seq![44u8,33u8,57u8,45u8,8u8,93u8,148u8]));
    assert(f::encoded_row(0u8,f::row_data(raw_ok16_ct3_d8_il1_7x5(),30nat,7nat,1nat),Some(seq![44u8,33u8,57u8,45u8,8u8,93u8,148u8]),1nat,seq![79u8,6u8,48u8,162u8,58u8,101u8,16u8],0,7nat)) by (compute);
    f::checked_row_hint(0u8,f::row_data(raw_ok16_ct3_d8_il1_7x5(),30nat,7nat,1nat),Some(seq![44u8,33u8,57u8,45u8,8u8,93u8,148u8]),1nat,seq![79u8,6u8,48u8,162u8,58u8,101u8,16u8]);
    assert(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),6nat,1nat)==seq![79u8,6u8,48u8,162u8,58u8,101u8,16u8]);
    assert forall|x: nat| x<7 implies (#[trigger] r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),6nat,1nat),8u8,x) as nat)<199 by {
        if x==0 {assert((r::palette_sample(seq![79u8,6u8,48u8,162u8,58u8,101u8,16u8],8u8,0nat) as nat)<199nat) by (compute);assert((r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),6nat,1nat),8u8,0nat) as nat)<199nat);}
        else if x==1 {assert((r::palette_sample(seq![79u8,6u8,48u8,162u8,58u8,101u8,16u8],8u8,1nat) as nat)<199nat) by (compute);assert((r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),6nat,1nat),8u8,1nat) as nat)<199nat);}
        else if x==2 {assert((r::palette_sample(seq![79u8,6u8,48u8,162u8,58u8,101u8,16u8],8u8,2nat) as nat)<199nat) by (compute);assert((r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),6nat,1nat),8u8,2nat) as nat)<199nat);}
        else if x==3 {assert((r::palette_sample(seq![79u8,6u8,48u8,162u8,58u8,101u8,16u8],8u8,3nat) as nat)<199nat) by (compute);assert((r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),6nat,1nat),8u8,3nat) as nat)<199nat);}
        else if x==4 {assert((r::palette_sample(seq![79u8,6u8,48u8,162u8,58u8,101u8,16u8],8u8,4nat) as nat)<199nat) by (compute);assert((r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),6nat,1nat),8u8,4nat) as nat)<199nat);}
        else if x==5 {assert((r::palette_sample(seq![79u8,6u8,48u8,162u8,58u8,101u8,16u8],8u8,5nat) as nat)<199nat) by (compute);assert((r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),6nat,1nat),8u8,5nat) as nat)<199nat);}
        else if x==6 {assert((r::palette_sample(seq![79u8,6u8,48u8,162u8,58u8,101u8,16u8],8u8,6nat) as nat)<199nat) by (compute);assert((r::palette_sample(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),6nat,1nat),8u8,6nat) as nat)<199nat);}
    }
    assert(r::palette_valid(r::line(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),6nat,1nat),8u8,7nat,199nat));
    assert(r::row_result(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),6nat,1nat)==Ok(()));
    r::scan_step(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),7nat);
    r::pass_step(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),6nat,2nat);
    r::pass_step(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),6nat,1nat);
    r::pass_step(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),6nat,0nat);
    r::scan_step(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),6nat);
    r::pass_step(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),5nat,3nat);
    r::pass_step(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),5nat,2nat);
    r::pass_step(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),5nat,1nat);
    r::pass_step(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),5nat,0nat);
    r::scan_step(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),5nat);
    r::pass_step(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),4nat,1nat);
    r::pass_step(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),4nat,0nat);
    r::scan_step(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),4nat);
    r::pass_step(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),3nat,2nat);
    r::pass_step(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),3nat,1nat);
    r::pass_step(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),3nat,0nat);
    r::scan_step(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),3nat);
    r::pass_step(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),2nat,1nat);
    r::pass_step(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),2nat,0nat);
    r::scan_step(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),2nat);
    r::pass_step(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),1nat,1nat);
    r::pass_step(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),1nat,0nat);
    r::scan_step(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),1nat);
    r::pass_step(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),0nat,1nat);
    r::pass_step(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),0nat,0nat);
    r::scan_step(raw_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_ok16_ct3_d8_il1_7x5() ensures r::still(bytes_ok16_ct3_d8_il1_7x5(),STRICT,true)==(r::Projection::Ready {width:7u32,height:5u32,bytes:280nat}),
{
    hide(bytes_ok16_ct3_d8_il1_7x5);hide(z_ok16_ct3_d8_il1_7x5);hide(raw_ok16_ct3_d8_il1_7x5);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    assert(bytes_ok16_ct3_d8_il1_7x5().len()==721nat) by (compute);
    assert(crate::model::mh(bytes_ok16_ct3_d8_il1_7x5())==Ok((h_ok16_ct3_d8_il1_7x5(),33nat))) by {reveal(crate::model::mh);assert(crate::model::mh(bytes_ok16_ct3_d8_il1_7x5())==Ok((h_ok16_ct3_d8_il1_7x5(),33nat))) by (compute);}
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_ok16_ct3_d8_il1_7x5(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:597nat,next:642nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::model::frame(bytes_ok16_ct3_d8_il1_7x5(),33nat)==Ok(crate::model::WireChunk {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:597nat,next:642nat})) by (compute);
            assert(crate::model::be32(bytes_ok16_ct3_d8_il1_7x5(),638)==2990055217u32) by (compute);
            crc_ok16_ct3_d8_il1_7x5_crc_33();
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(597nat,3u8,8u8,false,false,false,false)==crate::metadata_model::Decision::Keep(199nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_ok16_ct3_d8_il1_7x5(),642nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:650nat,len:55nat,next:709nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_ok16_ct3_d8_il1_7x5(),642nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:650nat,len:55nat,next:709nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_ok16_ct3_d8_il1_7x5(),709nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:717nat,len:0nat,next:721nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_ok16_ct3_d8_il1_7x5(),709nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:717nat,len:0nat,next:721nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_ok16_ct3_d8_il1_7x5().subrange(650,705)==z_ok16_ct3_d8_il1_7x5()) by (compute);
    walk_ok16_ct3_d8_il1_7x5(bytes_ok16_ct3_d8_il1_7x5());
    inflate_ok16_ct3_d8_il1_7x5();scan_ok16_ct3_d8_il1_7x5();
    assert(r::raw(a_ok16_ct3_d8_il1_7x5())==46nat) by (compute);
    assert(r::data(z_ok16_ct3_d8_il1_7x5(),a_ok16_ct3_d8_il1_7x5(),STRICT)==Some(Ok(()))) by {reveal(r::data);};
    r::default_ready(bytes_ok16_ct3_d8_il1_7x5(),h_ok16_ct3_d8_il1_7x5(),z_ok16_ct3_d8_il1_7x5(),199nat,true);
}
fn public_ok16_ct3_d8_il1_7x5() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,7u8,0u8,0u8,0u8,5u8,8u8,3u8,0u8,0u8,1u8,201u8,67u8,54u8,124u8,0u8,0u8,2u8,85u8,80u8,76u8,84u8,69u8,200u8,194u8,15u8,93u8,249u8,99u8,72u8,242u8,195u8,209u8,69u8,114u8,44u8,240u8,56u8,43u8,232u8,140u8,35u8,230u8,228u8,130u8,60u8,200u8,187u8,168u8,40u8,154u8,115u8,86u8,56u8,50u8,42u8,0u8,58u8,251u8,250u8,13u8,231u8,27u8,59u8,102u8,230u8,199u8,245u8,48u8,15u8,138u8,234u8,35u8,251u8,127u8,48u8,154u8,131u8,0u8,242u8,214u8,179u8,246u8,32u8,55u8,75u8,163u8,80u8,95u8,116u8,74u8,180u8,33u8,240u8,74u8,171u8,98u8,26u8,110u8,20u8,33u8,139u8,6u8,85u8,39u8,236u8,7u8,180u8,171u8,65u8,32u8,220u8,126u8,252u8,5u8,90u8,56u8,24u8,11u8,231u8,34u8,32u8,174u8,91u8,140u8,91u8,81u8,59u8,36u8,63u8,8u8,237u8,97u8,78u8,36u8,178u8,173u8,101u8,52u8,162u8,15u8,150u8,137u8,33u8,137u8,216u8,130u8,221u8,218u8,85u8,164u8,237u8,15u8,187u8,119u8,7u8,251u8,29u8,245u8,96u8,145u8,140u8,161u8,133u8,83u8,120u8,105u8,159u8,44u8,225u8,10u8,194u8,88u8,132u8,121u8,253u8,41u8,221u8,146u8,147u8,161u8,240u8,112u8,188u8,177u8,197u8,227u8,86u8,151u8,66u8,118u8,202u8,89u8,158u8,142u8,103u8,89u8,192u8,239u8,199u8,69u8,131u8,207u8,14u8,168u8,73u8,30u8,165u8,36u8,26u8,81u8,167u8,93u8,185u8,154u8,112u8,6u8,169u8,105u8,85u8,238u8,228u8,102u8,92u8,236u8,208u8,161u8,57u8,236u8,217u8,53u8,135u8,208u8,132u8,80u8,121u8,122u8,27u8,51u8,107u8,38u8,126u8,175u8,85u8,129u8,197u8,37u8,25u8,212u8,208u8,200u8,108u8,131u8,93u8,21u8,5u8,68u8,88u8,246u8,172u8,45u8,196u8,142u8,187u8,95u8,141u8,30u8,7u8,166u8,220u8,86u8,226u8,16u8,58u8,126u8,226u8,127u8,170u8,188u8,160u8,82u8,200u8,254u8,185u8,171u8,40u8,78u8,173u8,185u8,165u8,171u8,98u8,14u8,186u8,141u8,134u8,158u8,42u8,114u8,208u8,96u8,2u8,161u8,56u8,34u8,129u8,252u8,229u8,91u8,62u8,123u8,174u8,254u8,130u8,88u8,152u8,134u8,249u8,84u8,180u8,208u8,111u8,155u8,4u8,10u8,127u8,20u8,163u8,147u8,170u8,65u8,206u8,59u8,184u8,224u8,204u8,87u8,98u8,246u8,122u8,241u8,65u8,5u8,188u8,52u8,102u8,246u8,38u8,197u8,190u8,156u8,49u8,193u8,22u8,36u8,247u8,10u8,69u8,187u8,11u8,112u8,199u8,18u8,16u8,146u8,75u8,99u8,117u8,49u8,143u8,146u8,155u8,31u8,89u8,251u8,239u8,159u8,76u8,171u8,108u8,252u8,82u8,187u8,44u8,182u8,7u8,209u8,60u8,35u8,60u8,57u8,165u8,218u8,238u8,54u8,2u8,83u8,45u8,127u8,44u8,248u8,174u8,179u8,72u8,89u8,41u8,173u8,130u8,178u8,41u8,146u8,1u8,246u8,87u8,78u8,243u8,121u8,2u8,212u8,53u8,54u8,125u8,57u8,67u8,114u8,157u8,199u8,109u8,152u8,135u8,27u8,29u8,245u8,6u8,130u8,9u8,101u8,60u8,86u8,8u8,109u8,200u8,61u8,221u8,219u8,157u8,112u8,170u8,157u8,22u8,246u8,106u8,166u8,210u8,150u8,207u8,218u8,42u8,242u8,154u8,189u8,226u8,84u8,43u8,26u8,93u8,115u8,209u8,121u8,172u8,83u8,103u8,185u8,83u8,34u8,57u8,85u8,141u8,55u8,95u8,173u8,153u8,234u8,58u8,99u8,63u8,79u8,99u8,192u8,211u8,139u8,74u8,99u8,134u8,214u8,234u8,151u8,23u8,56u8,221u8,159u8,15u8,38u8,69u8,242u8,161u8,130u8,173u8,33u8,8u8,62u8,24u8,55u8,5u8,107u8,154u8,133u8,242u8,205u8,125u8,17u8,170u8,251u8,187u8,92u8,241u8,201u8,144u8,68u8,191u8,151u8,187u8,143u8,215u8,10u8,165u8,254u8,174u8,42u8,129u8,240u8,18u8,13u8,64u8,207u8,52u8,139u8,202u8,32u8,234u8,223u8,19u8,30u8,173u8,192u8,255u8,160u8,139u8,54u8,220u8,122u8,251u8,102u8,140u8,5u8,39u8,207u8,136u8,242u8,254u8,1u8,133u8,83u8,216u8,236u8,67u8,154u8,237u8,153u8,199u8,51u8,230u8,147u8,120u8,153u8,207u8,68u8,199u8,160u8,145u8,222u8,37u8,178u8,33u8,166u8,132u8,181u8,139u8,126u8,153u8,107u8,232u8,105u8,219u8,160u8,179u8,66u8,99u8,132u8,243u8,11u8,32u8,198u8,43u8,199u8,142u8,44u8,168u8,16u8,202u8,178u8,56u8,159u8,49u8,0u8,0u8,0u8,55u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,152u8,207u8,232u8,195u8,100u8,166u8,204u8,236u8,109u8,204u8,18u8,56u8,141u8,33u8,126u8,117u8,141u8,38u8,99u8,207u8,204u8,151u8,76u8,77u8,79u8,19u8,152u8,213u8,202u8,47u8,178u8,232u8,124u8,149u8,248u8,114u8,59u8,212u8,156u8,193u8,159u8,205u8,96u8,145u8,85u8,170u8,0u8,0u8,70u8,150u8,15u8,26u8,77u8,158u8,109u8,110u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_ok16_ct3_d8_il1_7x5());model_ok16_ct3_d8_il1_7x5();}
    let result=crate::png::try_decode_still16(&b);
    assert(match result {Ok(s)=>s.width==7u32 && s.height==5u32 && s.rgba16@.len()==280nat,Err(crate::alloc_guard::TryError::OutOfMemory(_))=>true,_=>false});
}
// ok/ct4-d16-il1-9x10; frozen SHA256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_ok_ct4_d16_il1_9x10() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,9u8,0u8,0u8,0u8,10u8,16u8,4u8,0u8,0u8,1u8,235u8,155u8,80u8,224u8,0u8,0u8,1u8,135u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,124u8,1u8,131u8,254u8,0u8,149u8,156u8,236u8,8u8,123u8,167u8,102u8,91u8,1u8,151u8,152u8,175u8,223u8,92u8,81u8,49u8,104u8,2u8,162u8,37u8,39u8,92u8,3u8,89u8,59u8,114u8,62u8,4u8,214u8,88u8,71u8,226u8,64u8,94u8,3u8,224u8,123u8,65u8,57u8,225u8,0u8,69u8,53u8,65u8,64u8,159u8,144u8,71u8,103u8,1u8,146u8,13u8,212u8,56u8,98u8,225u8,142u8,147u8,2u8,198u8,205u8,188u8,26u8,201u8,30u8,156u8,13u8,3u8,108u8,160u8,71u8,56u8,25u8,134u8,128u8,130u8,172u8,230u8,44u8,226u8,37u8,190u8,86u8,252u8,3u8,211u8,91u8,240u8,4u8,173u8,57u8,243u8,113u8,173u8,67u8,75u8,7u8,9u8,45u8,15u8,147u8,231u8,33u8,138u8,211u8,190u8,10u8,150u8,104u8,0u8,62u8,214u8,209u8,203u8,123u8,20u8,109u8,108u8,14u8,48u8,209u8,118u8,98u8,106u8,5u8,148u8,1u8,79u8,220u8,79u8,45u8,40u8,48u8,175u8,161u8,229u8,11u8,131u8,119u8,89u8,155u8,177u8,171u8,2u8,12u8,228u8,154u8,78u8,177u8,155u8,103u8,5u8,221u8,94u8,143u8,4u8,191u8,68u8,9u8,184u8,3u8,224u8,65u8,54u8,223u8,111u8,221u8,202u8,171u8,126u8,174u8,143u8,37u8,87u8,108u8,187u8,132u8,4u8,51u8,192u8,180u8,119u8,96u8,130u8,113u8,164u8,201u8,187u8,132u8,22u8,78u8,247u8,149u8,229u8,0u8,58u8,163u8,207u8,213u8,201u8,202u8,83u8,47u8,95u8,228u8,163u8,58u8,162u8,219u8,8u8,94u8,249u8,251u8,169u8,78u8,68u8,223u8,189u8,79u8,57u8,14u8,34u8,15u8,82u8,4u8,250u8,245u8,239u8,95u8,5u8,45u8,1u8,237u8,113u8,14u8,103u8,190u8,85u8,78u8,64u8,47u8,230u8,76u8,134u8,202u8,86u8,106u8,42u8,107u8,248u8,39u8,129u8,188u8,35u8,26u8,201u8,107u8,252u8,3u8,41u8,31u8,173u8,151u8,92u8,41u8,109u8,227u8,255u8,2u8,208u8,71u8,12u8,175u8,61u8,178u8,189u8,99u8,117u8,114u8,184u8,102u8,222u8,35u8,114u8,240u8,135u8,18u8,252u8,207u8,63u8,111u8,61u8,95u8,247u8,112u8,151u8,27u8,194u8,229u8,61u8,42u8,136u8,51u8,240u8,5u8,3u8,137u8,189u8,14u8,13u8,176u8,166u8,37u8,116u8,173u8,230u8,238u8,126u8,80u8,89u8,154u8,71u8,44u8,63u8,88u8,48u8,10u8,8u8,123u8,237u8,167u8,175u8,235u8,168u8,196u8,241u8,43u8,155u8,193u8,208u8,169u8,229u8,4u8,2u8,205u8,158u8,174u8,200u8,235u8,173u8,66u8,252u8,216u8,168u8,48u8,244u8,183u8,11u8,224u8,14u8,246u8,161u8,241u8,92u8,249u8,67u8,185u8,140u8,67u8,250u8,140u8,174u8,50u8,243u8,80u8,222u8,159u8,210u8,33u8,15u8,166u8,178u8,116u8,152u8,10u8,172u8,176u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn h_ok_ct4_d16_il1_9x10() -> crate::model::Ihdr {crate::model::Ihdr {width:9u32,height:10u32,depth:16u8,color_type:4u8,interlace:1u8}}
spec fn a_ok_ct4_d16_il1_9x10() -> r::Raster {r::shape(h_ok_ct4_d16_il1_9x10(),9nat,10nat,0nat,STRICT)}
spec fn z_ok_ct4_d16_il1_9x10() -> Seq<u8> {seq![120u8,156u8,1u8,124u8,1u8,131u8,254u8,0u8,149u8,156u8,236u8,8u8,123u8,167u8,102u8,91u8,1u8,151u8,152u8,175u8,223u8,92u8,81u8,49u8,104u8,2u8,162u8,37u8,39u8,92u8,3u8,89u8,59u8,114u8,62u8,4u8,214u8,88u8,71u8,226u8,64u8,94u8,3u8,224u8,123u8,65u8,57u8,225u8,0u8,69u8,53u8,65u8,64u8,159u8,144u8,71u8,103u8,1u8,146u8,13u8,212u8,56u8,98u8,225u8,142u8,147u8,2u8,198u8,205u8,188u8,26u8,201u8,30u8,156u8,13u8,3u8,108u8,160u8,71u8,56u8,25u8,134u8,128u8,130u8,172u8,230u8,44u8,226u8,37u8,190u8,86u8,252u8,3u8,211u8,91u8,240u8,4u8,173u8,57u8,243u8,113u8,173u8,67u8,75u8,7u8,9u8,45u8,15u8,147u8,231u8,33u8,138u8,211u8,190u8,10u8,150u8,104u8,0u8,62u8,214u8,209u8,203u8,123u8,20u8,109u8,108u8,14u8,48u8,209u8,118u8,98u8,106u8,5u8,148u8,1u8,79u8,220u8,79u8,45u8,40u8,48u8,175u8,161u8,229u8,11u8,131u8,119u8,89u8,155u8,177u8,171u8,2u8,12u8,228u8,154u8,78u8,177u8,155u8,103u8,5u8,221u8,94u8,143u8,4u8,191u8,68u8,9u8,184u8,3u8,224u8,65u8,54u8,223u8,111u8,221u8,202u8,171u8,126u8,174u8,143u8,37u8,87u8,108u8,187u8,132u8,4u8,51u8,192u8,180u8,119u8,96u8,130u8,113u8,164u8,201u8,187u8,132u8,22u8,78u8,247u8,149u8,229u8,0u8,58u8,163u8,207u8,213u8,201u8,202u8,83u8,47u8,95u8,228u8,163u8,58u8,162u8,219u8,8u8,94u8,249u8,251u8,169u8,78u8,68u8,223u8,189u8,79u8,57u8,14u8,34u8,15u8,82u8,4u8,250u8,245u8,239u8,95u8,5u8,45u8,1u8,237u8,113u8,14u8,103u8,190u8,85u8,78u8,64u8,47u8,230u8,76u8,134u8,202u8,86u8,106u8,42u8,107u8,248u8,39u8,129u8,188u8,35u8,26u8,201u8,107u8,252u8,3u8,41u8,31u8,173u8,151u8,92u8,41u8,109u8,227u8,255u8,2u8,208u8,71u8,12u8,175u8,61u8,178u8,189u8,99u8,117u8,114u8,184u8,102u8,222u8,35u8,114u8,240u8,135u8,18u8,252u8,207u8,63u8,111u8,61u8,95u8,247u8,112u8,151u8,27u8,194u8,229u8,61u8,42u8,136u8,51u8,240u8,5u8,3u8,137u8,189u8,14u8,13u8,176u8,166u8,37u8,116u8,173u8,230u8,238u8,126u8,80u8,89u8,154u8,71u8,44u8,63u8,88u8,48u8,10u8,8u8,123u8,237u8,167u8,175u8,235u8,168u8,196u8,241u8,43u8,155u8,193u8,208u8,169u8,229u8,4u8,2u8,205u8,158u8,174u8,200u8,235u8,173u8,66u8,252u8,216u8,168u8,48u8,244u8,183u8,11u8,224u8,14u8,246u8,161u8,241u8,92u8,249u8,67u8,185u8,140u8,67u8,250u8,140u8,174u8,50u8,243u8,80u8,222u8,159u8,210u8,33u8,15u8,166u8,178u8,116u8]}
spec fn raw_ok_ct4_d16_il1_9x10() -> Seq<u8> {seq![0u8,149u8,156u8,236u8,8u8,123u8,167u8,102u8,91u8,1u8,151u8,152u8,175u8,223u8,92u8,81u8,49u8,104u8,2u8,162u8,37u8,39u8,92u8,3u8,89u8,59u8,114u8,62u8,4u8,214u8,88u8,71u8,226u8,64u8,94u8,3u8,224u8,123u8,65u8,57u8,225u8,0u8,69u8,53u8,65u8,64u8,159u8,144u8,71u8,103u8,1u8,146u8,13u8,212u8,56u8,98u8,225u8,142u8,147u8,2u8,198u8,205u8,188u8,26u8,201u8,30u8,156u8,13u8,3u8,108u8,160u8,71u8,56u8,25u8,134u8,128u8,130u8,172u8,230u8,44u8,226u8,37u8,190u8,86u8,252u8,3u8,211u8,91u8,240u8,4u8,173u8,57u8,243u8,113u8,173u8,67u8,75u8,7u8,9u8,45u8,15u8,147u8,231u8,33u8,138u8,211u8,190u8,10u8,150u8,104u8,0u8,62u8,214u8,209u8,203u8,123u8,20u8,109u8,108u8,14u8,48u8,209u8,118u8,98u8,106u8,5u8,148u8,1u8,79u8,220u8,79u8,45u8,40u8,48u8,175u8,161u8,229u8,11u8,131u8,119u8,89u8,155u8,177u8,171u8,2u8,12u8,228u8,154u8,78u8,177u8,155u8,103u8,5u8,221u8,94u8,143u8,4u8,191u8,68u8,9u8,184u8,3u8,224u8,65u8,54u8,223u8,111u8,221u8,202u8,171u8,126u8,174u8,143u8,37u8,87u8,108u8,187u8,132u8,4u8,51u8,192u8,180u8,119u8,96u8,130u8,113u8,164u8,201u8,187u8,132u8,22u8,78u8,247u8,149u8,229u8,0u8,58u8,163u8,207u8,213u8,201u8,202u8,83u8,47u8,95u8,228u8,163u8,58u8,162u8,219u8,8u8,94u8,249u8,251u8,169u8,78u8,68u8,223u8,189u8,79u8,57u8,14u8,34u8,15u8,82u8,4u8,250u8,245u8,239u8,95u8,5u8,45u8,1u8,237u8,113u8,14u8,103u8,190u8,85u8,78u8,64u8,47u8,230u8,76u8,134u8,202u8,86u8,106u8,42u8,107u8,248u8,39u8,129u8,188u8,35u8,26u8,201u8,107u8,252u8,3u8,41u8,31u8,173u8,151u8,92u8,41u8,109u8,227u8,255u8,2u8,208u8,71u8,12u8,175u8,61u8,178u8,189u8,99u8,117u8,114u8,184u8,102u8,222u8,35u8,114u8,240u8,135u8,18u8,252u8,207u8,63u8,111u8,61u8,95u8,247u8,112u8,151u8,27u8,194u8,229u8,61u8,42u8,136u8,51u8,240u8,5u8,3u8,137u8,189u8,14u8,13u8,176u8,166u8,37u8,116u8,173u8,230u8,238u8,126u8,80u8,89u8,154u8,71u8,44u8,63u8,88u8,48u8,10u8,8u8,123u8,237u8,167u8,175u8,235u8,168u8,196u8,241u8,43u8,155u8,193u8,208u8,169u8,229u8,4u8,2u8,205u8,158u8,174u8,200u8,235u8,173u8,66u8,252u8,216u8,168u8,48u8,244u8,183u8,11u8,224u8,14u8,246u8,161u8,241u8,92u8,249u8,67u8,185u8,140u8,67u8,250u8,140u8,174u8,50u8,243u8,80u8,222u8,159u8,210u8,33u8]}
#[verifier::spinoff_prover]
proof fn shape_ok_ct4_d16_il1_9x10() ensures z_ok_ct4_d16_il1_9x10().len()==391nat,
        raw_ok_ct4_d16_il1_9x10().len()==380nat,
        z_ok_ct4_d16_il1_9x10()[0]==120u8,
        z_ok_ct4_d16_il1_9x10()[1]==156u8,
        zm::field_value(z_ok_ct4_d16_il1_9x10(),16,1)==1nat,
        zm::field_value(z_ok_ct4_d16_il1_9x10(),17,2)==0nat,
        zm::be_value(z_ok_ct4_d16_il1_9x10(),387nat,4)==262582900nat,
        z_ok_ct4_d16_il1_9x10().subrange(7,387)==raw_ok_ct4_d16_il1_9x10(),
        z_ok_ct4_d16_il1_9x10()[3] as nat+256*z_ok_ct4_d16_il1_9x10()[4] as nat==380nat,
        z_ok_ct4_d16_il1_9x10()[5] as nat+256*z_ok_ct4_d16_il1_9x10()[6] as nat==u16::MAX as nat-380nat,
        zm::be_value(z_ok_ct4_d16_il1_9x10(),0,1)==120nat,
        zm::be_value(z_ok_ct4_d16_il1_9x10(),1,1)==156nat,
{
    assert(z_ok_ct4_d16_il1_9x10().len()==391nat) by (compute);
    assert(raw_ok_ct4_d16_il1_9x10().len()==380nat) by (compute);
    assert(z_ok_ct4_d16_il1_9x10()[0]==120u8) by (compute);
    assert(z_ok_ct4_d16_il1_9x10()[1]==156u8) by (compute);
    assert(zm::field_value(z_ok_ct4_d16_il1_9x10(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_ok_ct4_d16_il1_9x10(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_ok_ct4_d16_il1_9x10(),387nat,4)==262582900nat) by (compute);
    assert(z_ok_ct4_d16_il1_9x10().subrange(7,387)==raw_ok_ct4_d16_il1_9x10()) by (compute);
    assert(z_ok_ct4_d16_il1_9x10()[3] as nat+256*z_ok_ct4_d16_il1_9x10()[4] as nat==380nat) by (compute);
    assert(z_ok_ct4_d16_il1_9x10()[5] as nat+256*z_ok_ct4_d16_il1_9x10()[6] as nat==u16::MAX as nat-380nat) by (compute);
    assert(zm::be_value(z_ok_ct4_d16_il1_9x10(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_ok_ct4_d16_il1_9x10(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_ok_ct4_d16_il1_9x10() ensures zm::Adler(raw_ok_ct4_d16_il1_9x10())==262582900nat,
{
    hide(raw_ok_ct4_d16_il1_9x10);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_ok_ct4_d16_il1_9x10().len()==380nat) by (compute);
    assert(c::sums(raw_ok_ct4_d16_il1_9x10(),0,380nat)==(45683nat,8194131nat)) by (compute);
    c::sums_correct(raw_ok_ct4_d16_il1_9x10(),0,380nat);assert(raw_ok_ct4_d16_il1_9x10().subrange(0,380)=~=raw_ok_ct4_d16_il1_9x10());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((8194131nat%65521)*(u16::MAX as nat+1)+(1+45683nat)%65521==262582900nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_ok_ct4_d16_il1_9x10() ensures zm::Inflate(z_ok_ct4_d16_il1_9x10(),380nat,true,false)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_ok_ct4_d16_il1_9x10(),r2:false}),raw_ok_ct4_d16_il1_9x10().len()==380nat,
{
    hide(z_ok_ct4_d16_il1_9x10);hide(raw_ok_ct4_d16_il1_9x10);hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_ok_ct4_d16_il1_9x10();checksum_ok_ct4_d16_il1_9x10();
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_ok_ct4_d16_il1_9x10(),raw_ok_ct4_d16_il1_9x10(),380nat,120nat,156nat,262582900nat);
}
#[verifier::spinoff_prover]
proof fn crc_ok_ct4_d16_il1_9x10_crc_33()
    ensures crate::model::crc(bytes_ok_ct4_d16_il1_9x10().subrange(37,432))==2550836400u32,
{
    hide(crate::model::crc_register);hide(crc_cert::fold);
    assert(crate::model::crc_register(bytes_ok_ct4_d16_il1_9x10().subrange(37,432).take(0))==0xffff_ffffu32) by {
        assert(bytes_ok_ct4_d16_il1_9x10().subrange(37,432).take(0) =~= Seq::<u8>::empty());
        reveal_with_fuel(crate::model::crc_register,1);
    }
    assert(crc_cert::fold(4294967295u32,bytes_ok_ct4_d16_il1_9x10().subrange(37,432).subrange(0,64))==557789868u32) by (compute);
    crc_cert::suffix(bytes_ok_ct4_d16_il1_9x10().subrange(37,432).take(0),bytes_ok_ct4_d16_il1_9x10().subrange(37,432).subrange(0,64));
    assert(bytes_ok_ct4_d16_il1_9x10().subrange(37,432).take(0)+bytes_ok_ct4_d16_il1_9x10().subrange(37,432).subrange(0,64) =~= bytes_ok_ct4_d16_il1_9x10().subrange(37,432).take(64));
    assert(crate::model::crc_register(bytes_ok_ct4_d16_il1_9x10().subrange(37,432).take(64))==557789868u32);
    assert(crc_cert::fold(557789868u32,bytes_ok_ct4_d16_il1_9x10().subrange(37,432).subrange(64,128))==3038896818u32) by (compute);
    crc_cert::suffix(bytes_ok_ct4_d16_il1_9x10().subrange(37,432).take(64),bytes_ok_ct4_d16_il1_9x10().subrange(37,432).subrange(64,128));
    assert(bytes_ok_ct4_d16_il1_9x10().subrange(37,432).take(64)+bytes_ok_ct4_d16_il1_9x10().subrange(37,432).subrange(64,128) =~= bytes_ok_ct4_d16_il1_9x10().subrange(37,432).take(128));
    assert(crate::model::crc_register(bytes_ok_ct4_d16_il1_9x10().subrange(37,432).take(128))==3038896818u32);
    assert(crc_cert::fold(3038896818u32,bytes_ok_ct4_d16_il1_9x10().subrange(37,432).subrange(128,192))==2499955519u32) by (compute);
    crc_cert::suffix(bytes_ok_ct4_d16_il1_9x10().subrange(37,432).take(128),bytes_ok_ct4_d16_il1_9x10().subrange(37,432).subrange(128,192));
    assert(bytes_ok_ct4_d16_il1_9x10().subrange(37,432).take(128)+bytes_ok_ct4_d16_il1_9x10().subrange(37,432).subrange(128,192) =~= bytes_ok_ct4_d16_il1_9x10().subrange(37,432).take(192));
    assert(crate::model::crc_register(bytes_ok_ct4_d16_il1_9x10().subrange(37,432).take(192))==2499955519u32);
    assert(crc_cert::fold(2499955519u32,bytes_ok_ct4_d16_il1_9x10().subrange(37,432).subrange(192,256))==135386369u32) by (compute);
    crc_cert::suffix(bytes_ok_ct4_d16_il1_9x10().subrange(37,432).take(192),bytes_ok_ct4_d16_il1_9x10().subrange(37,432).subrange(192,256));
    assert(bytes_ok_ct4_d16_il1_9x10().subrange(37,432).take(192)+bytes_ok_ct4_d16_il1_9x10().subrange(37,432).subrange(192,256) =~= bytes_ok_ct4_d16_il1_9x10().subrange(37,432).take(256));
    assert(crate::model::crc_register(bytes_ok_ct4_d16_il1_9x10().subrange(37,432).take(256))==135386369u32);
    assert(crc_cert::fold(135386369u32,bytes_ok_ct4_d16_il1_9x10().subrange(37,432).subrange(256,320))==2807715361u32) by (compute);
    crc_cert::suffix(bytes_ok_ct4_d16_il1_9x10().subrange(37,432).take(256),bytes_ok_ct4_d16_il1_9x10().subrange(37,432).subrange(256,320));
    assert(bytes_ok_ct4_d16_il1_9x10().subrange(37,432).take(256)+bytes_ok_ct4_d16_il1_9x10().subrange(37,432).subrange(256,320) =~= bytes_ok_ct4_d16_il1_9x10().subrange(37,432).take(320));
    assert(crate::model::crc_register(bytes_ok_ct4_d16_il1_9x10().subrange(37,432).take(320))==2807715361u32);
    assert(crc_cert::fold(2807715361u32,bytes_ok_ct4_d16_il1_9x10().subrange(37,432).subrange(320,384))==2937265586u32) by (compute);
    crc_cert::suffix(bytes_ok_ct4_d16_il1_9x10().subrange(37,432).take(320),bytes_ok_ct4_d16_il1_9x10().subrange(37,432).subrange(320,384));
    assert(bytes_ok_ct4_d16_il1_9x10().subrange(37,432).take(320)+bytes_ok_ct4_d16_il1_9x10().subrange(37,432).subrange(320,384) =~= bytes_ok_ct4_d16_il1_9x10().subrange(37,432).take(384));
    assert(crate::model::crc_register(bytes_ok_ct4_d16_il1_9x10().subrange(37,432).take(384))==2937265586u32);
    assert(crc_cert::fold(2937265586u32,bytes_ok_ct4_d16_il1_9x10().subrange(37,432).subrange(384,395))==1744130895u32) by (compute);
    crc_cert::suffix(bytes_ok_ct4_d16_il1_9x10().subrange(37,432).take(384),bytes_ok_ct4_d16_il1_9x10().subrange(37,432).subrange(384,395));
    assert(bytes_ok_ct4_d16_il1_9x10().subrange(37,432).take(384)+bytes_ok_ct4_d16_il1_9x10().subrange(37,432).subrange(384,395) =~= bytes_ok_ct4_d16_il1_9x10().subrange(37,432).take(395));
    assert(crate::model::crc_register(bytes_ok_ct4_d16_il1_9x10().subrange(37,432).take(395))==1744130895u32);
    assert(bytes_ok_ct4_d16_il1_9x10().subrange(37,432).take(395) =~= bytes_ok_ct4_d16_il1_9x10().subrange(37,432));
    assert((1744130895u32 ^ 0xffff_ffffu32)==2550836400u32) by (compute);
}
#[verifier::spinoff_prover]
proof fn walk_ok_ct4_d16_il1_9x10(b: Seq<u8>)
    requires b.len()==448,
        crate::model::mh(b)==Ok((h_ok_ct4_d16_il1_9x10(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:391nat,next:436nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,436nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:444nat,len:0nat,next:448nat,crc_ok:true,synthetic:false})),
        b.subrange(41,432)==z_ok_ct4_d16_il1_9x10(),
    ensures r::default_case(b,h_ok_ct4_d16_il1_9x10(),z_ok_ct4_d16_il1_9x10(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
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
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    reveal_with_fuel(m::drive,3);
    reveal_with_fuel(m::concat_ranges,3);
}
#[verifier::spinoff_prover]
proof fn scan_ok_ct4_d16_il1_9x10() ensures r::scan(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),0)==Ok(()),
{
    hide(raw_ok_ct4_d16_il1_9x10);assert(raw_ok_ct4_d16_il1_9x10().len()==380nat) by (compute);assert(r::count(a_ok_ct4_d16_il1_9x10())==7nat) by (compute);
    assert(r::pw(a_ok_ct4_d16_il1_9x10(),0nat)==2nat && r::ph(a_ok_ct4_d16_il1_9x10(),0nat)==2nat && r::rb(a_ok_ct4_d16_il1_9x10(),0nat)==8nat && r::base(a_ok_ct4_d16_il1_9x10(),0nat)==0nat) by (compute);
    assert(raw_ok_ct4_d16_il1_9x10()[0]==0u8) by (compute);
    assert(r::row_result(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),0nat,0nat)==Ok(()));
    assert(raw_ok_ct4_d16_il1_9x10()[9]==1u8) by (compute);
    assert(r::row_result(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),0nat,1nat)==Ok(()));
    assert(r::pw(a_ok_ct4_d16_il1_9x10(),1nat)==1nat && r::ph(a_ok_ct4_d16_il1_9x10(),1nat)==2nat && r::rb(a_ok_ct4_d16_il1_9x10(),1nat)==4nat && r::base(a_ok_ct4_d16_il1_9x10(),1nat)==18nat) by (compute);
    assert(raw_ok_ct4_d16_il1_9x10()[18]==2u8) by (compute);
    assert(r::row_result(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),1nat,0nat)==Ok(()));
    assert(raw_ok_ct4_d16_il1_9x10()[23]==3u8) by (compute);
    assert(r::row_result(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),1nat,1nat)==Ok(()));
    assert(r::pw(a_ok_ct4_d16_il1_9x10(),2nat)==3nat && r::ph(a_ok_ct4_d16_il1_9x10(),2nat)==1nat && r::rb(a_ok_ct4_d16_il1_9x10(),2nat)==12nat && r::base(a_ok_ct4_d16_il1_9x10(),2nat)==28nat) by (compute);
    assert(raw_ok_ct4_d16_il1_9x10()[28]==4u8) by (compute);
    assert(r::row_result(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),2nat,0nat)==Ok(()));
    assert(r::pw(a_ok_ct4_d16_il1_9x10(),3nat)==2nat && r::ph(a_ok_ct4_d16_il1_9x10(),3nat)==3nat && r::rb(a_ok_ct4_d16_il1_9x10(),3nat)==8nat && r::base(a_ok_ct4_d16_il1_9x10(),3nat)==41nat) by (compute);
    assert(raw_ok_ct4_d16_il1_9x10()[41]==0u8) by (compute);
    assert(r::row_result(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),3nat,0nat)==Ok(()));
    assert(raw_ok_ct4_d16_il1_9x10()[50]==1u8) by (compute);
    assert(r::row_result(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),3nat,1nat)==Ok(()));
    assert(raw_ok_ct4_d16_il1_9x10()[59]==2u8) by (compute);
    assert(r::row_result(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),3nat,2nat)==Ok(()));
    assert(r::pw(a_ok_ct4_d16_il1_9x10(),4nat)==5nat && r::ph(a_ok_ct4_d16_il1_9x10(),4nat)==2nat && r::rb(a_ok_ct4_d16_il1_9x10(),4nat)==20nat && r::base(a_ok_ct4_d16_il1_9x10(),4nat)==68nat) by (compute);
    assert(raw_ok_ct4_d16_il1_9x10()[68]==3u8) by (compute);
    assert(r::row_result(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),4nat,0nat)==Ok(()));
    assert(raw_ok_ct4_d16_il1_9x10()[89]==4u8) by (compute);
    assert(r::row_result(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),4nat,1nat)==Ok(()));
    assert(r::pw(a_ok_ct4_d16_il1_9x10(),5nat)==4nat && r::ph(a_ok_ct4_d16_il1_9x10(),5nat)==5nat && r::rb(a_ok_ct4_d16_il1_9x10(),5nat)==16nat && r::base(a_ok_ct4_d16_il1_9x10(),5nat)==110nat) by (compute);
    assert(raw_ok_ct4_d16_il1_9x10()[110]==0u8) by (compute);
    assert(r::row_result(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),5nat,0nat)==Ok(()));
    assert(raw_ok_ct4_d16_il1_9x10()[127]==1u8) by (compute);
    assert(r::row_result(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),5nat,1nat)==Ok(()));
    assert(raw_ok_ct4_d16_il1_9x10()[144]==2u8) by (compute);
    assert(r::row_result(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),5nat,2nat)==Ok(()));
    assert(raw_ok_ct4_d16_il1_9x10()[161]==3u8) by (compute);
    assert(r::row_result(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),5nat,3nat)==Ok(()));
    assert(raw_ok_ct4_d16_il1_9x10()[178]==4u8) by (compute);
    assert(r::row_result(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),5nat,4nat)==Ok(()));
    assert(r::pw(a_ok_ct4_d16_il1_9x10(),6nat)==9nat && r::ph(a_ok_ct4_d16_il1_9x10(),6nat)==5nat && r::rb(a_ok_ct4_d16_il1_9x10(),6nat)==36nat && r::base(a_ok_ct4_d16_il1_9x10(),6nat)==195nat) by (compute);
    assert(raw_ok_ct4_d16_il1_9x10()[195]==0u8) by (compute);
    assert(r::row_result(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),6nat,0nat)==Ok(()));
    assert(raw_ok_ct4_d16_il1_9x10()[232]==1u8) by (compute);
    assert(r::row_result(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),6nat,1nat)==Ok(()));
    assert(raw_ok_ct4_d16_il1_9x10()[269]==2u8) by (compute);
    assert(r::row_result(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),6nat,2nat)==Ok(()));
    assert(raw_ok_ct4_d16_il1_9x10()[306]==3u8) by (compute);
    assert(r::row_result(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),6nat,3nat)==Ok(()));
    assert(raw_ok_ct4_d16_il1_9x10()[343]==4u8) by (compute);
    assert(r::row_result(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),6nat,4nat)==Ok(()));
    r::scan_step(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),7nat);
    r::pass_step(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),6nat,5nat);
    r::pass_step(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),6nat,4nat);
    r::pass_step(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),6nat,3nat);
    r::pass_step(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),6nat,2nat);
    r::pass_step(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),6nat,1nat);
    r::pass_step(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),6nat,0nat);
    r::scan_step(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),6nat);
    r::pass_step(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),5nat,5nat);
    r::pass_step(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),5nat,4nat);
    r::pass_step(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),5nat,3nat);
    r::pass_step(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),5nat,2nat);
    r::pass_step(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),5nat,1nat);
    r::pass_step(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),5nat,0nat);
    r::scan_step(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),5nat);
    r::pass_step(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),4nat,2nat);
    r::pass_step(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),4nat,1nat);
    r::pass_step(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),4nat,0nat);
    r::scan_step(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),4nat);
    r::pass_step(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),3nat,3nat);
    r::pass_step(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),3nat,2nat);
    r::pass_step(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),3nat,1nat);
    r::pass_step(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),3nat,0nat);
    r::scan_step(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),3nat);
    r::pass_step(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),2nat,1nat);
    r::pass_step(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),2nat,0nat);
    r::scan_step(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),2nat);
    r::pass_step(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),1nat,2nat);
    r::pass_step(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),1nat,1nat);
    r::pass_step(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),1nat,0nat);
    r::scan_step(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),1nat);
    r::pass_step(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),0nat,2nat);
    r::pass_step(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),0nat,1nat);
    r::pass_step(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),0nat,0nat);
    r::scan_step(raw_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_ok_ct4_d16_il1_9x10() ensures r::still(bytes_ok_ct4_d16_il1_9x10(),STRICT,false)==(r::Projection::Ready {width:9u32,height:10u32,bytes:360nat}),
{
    hide(bytes_ok_ct4_d16_il1_9x10);hide(z_ok_ct4_d16_il1_9x10);hide(raw_ok_ct4_d16_il1_9x10);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    assert(bytes_ok_ct4_d16_il1_9x10().len()==448nat) by (compute);
    assert(crate::model::mh(bytes_ok_ct4_d16_il1_9x10())==Ok((h_ok_ct4_d16_il1_9x10(),33nat))) by {reveal(crate::model::mh);assert(crate::model::mh(bytes_ok_ct4_d16_il1_9x10())==Ok((h_ok_ct4_d16_il1_9x10(),33nat))) by (compute);}
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_ok_ct4_d16_il1_9x10(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:391nat,next:436nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::model::frame(bytes_ok_ct4_d16_il1_9x10(),33nat)==Ok(crate::model::WireChunk {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:391nat,next:436nat})) by (compute);
            assert(crate::model::be32(bytes_ok_ct4_d16_il1_9x10(),432)==2550836400u32) by (compute);
            crc_ok_ct4_d16_il1_9x10_crc_33();
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_ok_ct4_d16_il1_9x10(),436nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:444nat,len:0nat,next:448nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_ok_ct4_d16_il1_9x10(),436nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:444nat,len:0nat,next:448nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_ok_ct4_d16_il1_9x10().subrange(41,432)==z_ok_ct4_d16_il1_9x10()) by (compute);
    walk_ok_ct4_d16_il1_9x10(bytes_ok_ct4_d16_il1_9x10());
    inflate_ok_ct4_d16_il1_9x10();scan_ok_ct4_d16_il1_9x10();
    assert(r::raw(a_ok_ct4_d16_il1_9x10())==380nat) by (compute);
    assert(r::data(z_ok_ct4_d16_il1_9x10(),a_ok_ct4_d16_il1_9x10(),STRICT)==Some(Ok(()))) by {reveal(r::data);};
    r::default_ready(bytes_ok_ct4_d16_il1_9x10(),h_ok_ct4_d16_il1_9x10(),z_ok_ct4_d16_il1_9x10(),0nat,false);
}
fn public_ok_ct4_d16_il1_9x10() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,9u8,0u8,0u8,0u8,10u8,16u8,4u8,0u8,0u8,1u8,235u8,155u8,80u8,224u8,0u8,0u8,1u8,135u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,124u8,1u8,131u8,254u8,0u8,149u8,156u8,236u8,8u8,123u8,167u8,102u8,91u8,1u8,151u8,152u8,175u8,223u8,92u8,81u8,49u8,104u8,2u8,162u8,37u8,39u8,92u8,3u8,89u8,59u8,114u8,62u8,4u8,214u8,88u8,71u8,226u8,64u8,94u8,3u8,224u8,123u8,65u8,57u8,225u8,0u8,69u8,53u8,65u8,64u8,159u8,144u8,71u8,103u8,1u8,146u8,13u8,212u8,56u8,98u8,225u8,142u8,147u8,2u8,198u8,205u8,188u8,26u8,201u8,30u8,156u8,13u8,3u8,108u8,160u8,71u8,56u8,25u8,134u8,128u8,130u8,172u8,230u8,44u8,226u8,37u8,190u8,86u8,252u8,3u8,211u8,91u8,240u8,4u8,173u8,57u8,243u8,113u8,173u8,67u8,75u8,7u8,9u8,45u8,15u8,147u8,231u8,33u8,138u8,211u8,190u8,10u8,150u8,104u8,0u8,62u8,214u8,209u8,203u8,123u8,20u8,109u8,108u8,14u8,48u8,209u8,118u8,98u8,106u8,5u8,148u8,1u8,79u8,220u8,79u8,45u8,40u8,48u8,175u8,161u8,229u8,11u8,131u8,119u8,89u8,155u8,177u8,171u8,2u8,12u8,228u8,154u8,78u8,177u8,155u8,103u8,5u8,221u8,94u8,143u8,4u8,191u8,68u8,9u8,184u8,3u8,224u8,65u8,54u8,223u8,111u8,221u8,202u8,171u8,126u8,174u8,143u8,37u8,87u8,108u8,187u8,132u8,4u8,51u8,192u8,180u8,119u8,96u8,130u8,113u8,164u8,201u8,187u8,132u8,22u8,78u8,247u8,149u8,229u8,0u8,58u8,163u8,207u8,213u8,201u8,202u8,83u8,47u8,95u8,228u8,163u8,58u8,162u8,219u8,8u8,94u8,249u8,251u8,169u8,78u8,68u8,223u8,189u8,79u8,57u8,14u8,34u8,15u8,82u8,4u8,250u8,245u8,239u8,95u8,5u8,45u8,1u8,237u8,113u8,14u8,103u8,190u8,85u8,78u8,64u8,47u8,230u8,76u8,134u8,202u8,86u8,106u8,42u8,107u8,248u8,39u8,129u8,188u8,35u8,26u8,201u8,107u8,252u8,3u8,41u8,31u8,173u8,151u8,92u8,41u8,109u8,227u8,255u8,2u8,208u8,71u8,12u8,175u8,61u8,178u8,189u8,99u8,117u8,114u8,184u8,102u8,222u8,35u8,114u8,240u8,135u8,18u8,252u8,207u8,63u8,111u8,61u8,95u8,247u8,112u8,151u8,27u8,194u8,229u8,61u8,42u8,136u8,51u8,240u8,5u8,3u8,137u8,189u8,14u8,13u8,176u8,166u8,37u8,116u8,173u8,230u8,238u8,126u8,80u8,89u8,154u8,71u8,44u8,63u8,88u8,48u8,10u8,8u8,123u8,237u8,167u8,175u8,235u8,168u8,196u8,241u8,43u8,155u8,193u8,208u8,169u8,229u8,4u8,2u8,205u8,158u8,174u8,200u8,235u8,173u8,66u8,252u8,216u8,168u8,48u8,244u8,183u8,11u8,224u8,14u8,246u8,161u8,241u8,92u8,249u8,67u8,185u8,140u8,67u8,250u8,140u8,174u8,50u8,243u8,80u8,222u8,159u8,210u8,33u8,15u8,166u8,178u8,116u8,152u8,10u8,172u8,176u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_ok_ct4_d16_il1_9x10());model_ok_ct4_d16_il1_9x10();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==9u32 && s.height==10u32 && s.rgba@.len()==360nat,Err(crate::alloc_guard::TryError::OutOfMemory(_))=>true,_=>false});
}
// ok/ct6-d16-il0-17x2; frozen SHA256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_ok_ct6_d16_il0_17x2() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,17u8,0u8,0u8,0u8,2u8,16u8,6u8,0u8,0u8,0u8,238u8,191u8,235u8,140u8,0u8,0u8,1u8,29u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,18u8,1u8,237u8,254u8,0u8,200u8,236u8,191u8,210u8,133u8,133u8,243u8,205u8,212u8,35u8,59u8,11u8,12u8,225u8,219u8,53u8,9u8,199u8,61u8,203u8,200u8,17u8,38u8,1u8,251u8,178u8,175u8,19u8,83u8,213u8,63u8,235u8,8u8,28u8,131u8,200u8,74u8,18u8,57u8,195u8,156u8,39u8,85u8,138u8,18u8,193u8,72u8,74u8,145u8,202u8,25u8,17u8,252u8,79u8,98u8,199u8,78u8,49u8,35u8,137u8,109u8,160u8,122u8,246u8,156u8,26u8,101u8,55u8,217u8,214u8,146u8,237u8,31u8,194u8,74u8,10u8,23u8,138u8,136u8,84u8,94u8,170u8,127u8,233u8,253u8,161u8,14u8,33u8,138u8,221u8,209u8,232u8,168u8,82u8,148u8,68u8,246u8,233u8,75u8,80u8,196u8,177u8,147u8,130u8,136u8,244u8,88u8,92u8,210u8,155u8,151u8,47u8,92u8,47u8,146u8,30u8,236u8,175u8,74u8,36u8,220u8,133u8,246u8,35u8,160u8,236u8,47u8,126u8,96u8,31u8,91u8,63u8,233u8,194u8,247u8,25u8,1u8,215u8,143u8,70u8,49u8,150u8,177u8,145u8,38u8,147u8,92u8,8u8,27u8,115u8,227u8,162u8,80u8,154u8,237u8,81u8,239u8,121u8,151u8,161u8,97u8,33u8,209u8,101u8,243u8,251u8,137u8,189u8,157u8,234u8,7u8,138u8,240u8,101u8,119u8,239u8,156u8,199u8,18u8,95u8,213u8,219u8,67u8,135u8,112u8,76u8,249u8,54u8,89u8,82u8,172u8,128u8,222u8,119u8,237u8,203u8,129u8,6u8,200u8,179u8,73u8,86u8,207u8,40u8,217u8,154u8,129u8,103u8,80u8,56u8,249u8,51u8,113u8,124u8,173u8,132u8,29u8,53u8,43u8,30u8,215u8,79u8,139u8,5u8,198u8,211u8,148u8,162u8,26u8,145u8,133u8,81u8,203u8,128u8,228u8,121u8,58u8,209u8,96u8,63u8,17u8,43u8,217u8,88u8,34u8,242u8,62u8,84u8,118u8,163u8,194u8,180u8,162u8,78u8,190u8,172u8,3u8,96u8,247u8,16u8,34u8,253u8,175u8,191u8,141u8,206u8,16u8,66u8,91u8,8u8,52u8,220u8,80u8,60u8,73u8,136u8,138u8,131u8,97u8,50u8,194u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn h_ok_ct6_d16_il0_17x2() -> crate::model::Ihdr {crate::model::Ihdr {width:17u32,height:2u32,depth:16u8,color_type:6u8,interlace:0u8}}
spec fn a_ok_ct6_d16_il0_17x2() -> r::Raster {r::shape(h_ok_ct6_d16_il0_17x2(),17nat,2nat,0nat,STRICT)}
spec fn z_ok_ct6_d16_il0_17x2() -> Seq<u8> {seq![120u8,156u8,1u8,18u8,1u8,237u8,254u8,0u8,200u8,236u8,191u8,210u8,133u8,133u8,243u8,205u8,212u8,35u8,59u8,11u8,12u8,225u8,219u8,53u8,9u8,199u8,61u8,203u8,200u8,17u8,38u8,1u8,251u8,178u8,175u8,19u8,83u8,213u8,63u8,235u8,8u8,28u8,131u8,200u8,74u8,18u8,57u8,195u8,156u8,39u8,85u8,138u8,18u8,193u8,72u8,74u8,145u8,202u8,25u8,17u8,252u8,79u8,98u8,199u8,78u8,49u8,35u8,137u8,109u8,160u8,122u8,246u8,156u8,26u8,101u8,55u8,217u8,214u8,146u8,237u8,31u8,194u8,74u8,10u8,23u8,138u8,136u8,84u8,94u8,170u8,127u8,233u8,253u8,161u8,14u8,33u8,138u8,221u8,209u8,232u8,168u8,82u8,148u8,68u8,246u8,233u8,75u8,80u8,196u8,177u8,147u8,130u8,136u8,244u8,88u8,92u8,210u8,155u8,151u8,47u8,92u8,47u8,146u8,30u8,236u8,175u8,74u8,36u8,220u8,133u8,246u8,35u8,160u8,236u8,47u8,126u8,96u8,31u8,91u8,63u8,233u8,194u8,247u8,25u8,1u8,215u8,143u8,70u8,49u8,150u8,177u8,145u8,38u8,147u8,92u8,8u8,27u8,115u8,227u8,162u8,80u8,154u8,237u8,81u8,239u8,121u8,151u8,161u8,97u8,33u8,209u8,101u8,243u8,251u8,137u8,189u8,157u8,234u8,7u8,138u8,240u8,101u8,119u8,239u8,156u8,199u8,18u8,95u8,213u8,219u8,67u8,135u8,112u8,76u8,249u8,54u8,89u8,82u8,172u8,128u8,222u8,119u8,237u8,203u8,129u8,6u8,200u8,179u8,73u8,86u8,207u8,40u8,217u8,154u8,129u8,103u8,80u8,56u8,249u8,51u8,113u8,124u8,173u8,132u8,29u8,53u8,43u8,30u8,215u8,79u8,139u8,5u8,198u8,211u8,148u8,162u8,26u8,145u8,133u8,81u8,203u8,128u8,228u8,121u8,58u8,209u8,96u8,63u8,17u8,43u8,217u8,88u8,34u8,242u8,62u8,84u8,118u8,163u8,194u8,180u8,162u8,78u8,190u8,172u8,3u8,96u8,247u8,16u8,34u8,253u8,175u8,191u8,141u8,206u8,16u8,66u8,91u8,8u8,52u8,220u8,80u8,60u8,73u8,136u8,138u8]}
spec fn raw_ok_ct6_d16_il0_17x2() -> Seq<u8> {seq![0u8,200u8,236u8,191u8,210u8,133u8,133u8,243u8,205u8,212u8,35u8,59u8,11u8,12u8,225u8,219u8,53u8,9u8,199u8,61u8,203u8,200u8,17u8,38u8,1u8,251u8,178u8,175u8,19u8,83u8,213u8,63u8,235u8,8u8,28u8,131u8,200u8,74u8,18u8,57u8,195u8,156u8,39u8,85u8,138u8,18u8,193u8,72u8,74u8,145u8,202u8,25u8,17u8,252u8,79u8,98u8,199u8,78u8,49u8,35u8,137u8,109u8,160u8,122u8,246u8,156u8,26u8,101u8,55u8,217u8,214u8,146u8,237u8,31u8,194u8,74u8,10u8,23u8,138u8,136u8,84u8,94u8,170u8,127u8,233u8,253u8,161u8,14u8,33u8,138u8,221u8,209u8,232u8,168u8,82u8,148u8,68u8,246u8,233u8,75u8,80u8,196u8,177u8,147u8,130u8,136u8,244u8,88u8,92u8,210u8,155u8,151u8,47u8,92u8,47u8,146u8,30u8,236u8,175u8,74u8,36u8,220u8,133u8,246u8,35u8,160u8,236u8,47u8,126u8,96u8,31u8,91u8,63u8,233u8,194u8,247u8,25u8,1u8,215u8,143u8,70u8,49u8,150u8,177u8,145u8,38u8,147u8,92u8,8u8,27u8,115u8,227u8,162u8,80u8,154u8,237u8,81u8,239u8,121u8,151u8,161u8,97u8,33u8,209u8,101u8,243u8,251u8,137u8,189u8,157u8,234u8,7u8,138u8,240u8,101u8,119u8,239u8,156u8,199u8,18u8,95u8,213u8,219u8,67u8,135u8,112u8,76u8,249u8,54u8,89u8,82u8,172u8,128u8,222u8,119u8,237u8,203u8,129u8,6u8,200u8,179u8,73u8,86u8,207u8,40u8,217u8,154u8,129u8,103u8,80u8,56u8,249u8,51u8,113u8,124u8,173u8,132u8,29u8,53u8,43u8,30u8,215u8,79u8,139u8,5u8,198u8,211u8,148u8,162u8,26u8,145u8,133u8,81u8,203u8,128u8,228u8,121u8,58u8,209u8,96u8,63u8,17u8,43u8,217u8,88u8,34u8,242u8,62u8,84u8,118u8,163u8,194u8,180u8,162u8,78u8,190u8,172u8,3u8,96u8,247u8,16u8,34u8,253u8,175u8,191u8,141u8,206u8,16u8,66u8,91u8,8u8,52u8,220u8,80u8]}
#[verifier::spinoff_prover]
proof fn shape_ok_ct6_d16_il0_17x2() ensures z_ok_ct6_d16_il0_17x2().len()==285nat,
        raw_ok_ct6_d16_il0_17x2().len()==274nat,
        z_ok_ct6_d16_il0_17x2()[0]==120u8,
        z_ok_ct6_d16_il0_17x2()[1]==156u8,
        zm::field_value(z_ok_ct6_d16_il0_17x2(),16,1)==1nat,
        zm::field_value(z_ok_ct6_d16_il0_17x2(),17,2)==0nat,
        zm::be_value(z_ok_ct6_d16_il0_17x2(),281nat,4)==1011452042nat,
        z_ok_ct6_d16_il0_17x2().subrange(7,281)==raw_ok_ct6_d16_il0_17x2(),
        z_ok_ct6_d16_il0_17x2()[3] as nat+256*z_ok_ct6_d16_il0_17x2()[4] as nat==274nat,
        z_ok_ct6_d16_il0_17x2()[5] as nat+256*z_ok_ct6_d16_il0_17x2()[6] as nat==u16::MAX as nat-274nat,
        zm::be_value(z_ok_ct6_d16_il0_17x2(),0,1)==120nat,
        zm::be_value(z_ok_ct6_d16_il0_17x2(),1,1)==156nat,
{
    assert(z_ok_ct6_d16_il0_17x2().len()==285nat) by (compute);
    assert(raw_ok_ct6_d16_il0_17x2().len()==274nat) by (compute);
    assert(z_ok_ct6_d16_il0_17x2()[0]==120u8) by (compute);
    assert(z_ok_ct6_d16_il0_17x2()[1]==156u8) by (compute);
    assert(zm::field_value(z_ok_ct6_d16_il0_17x2(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_ok_ct6_d16_il0_17x2(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_ok_ct6_d16_il0_17x2(),281nat,4)==1011452042nat) by (compute);
    assert(z_ok_ct6_d16_il0_17x2().subrange(7,281)==raw_ok_ct6_d16_il0_17x2()) by (compute);
    assert(z_ok_ct6_d16_il0_17x2()[3] as nat+256*z_ok_ct6_d16_il0_17x2()[4] as nat==274nat) by (compute);
    assert(z_ok_ct6_d16_il0_17x2()[5] as nat+256*z_ok_ct6_d16_il0_17x2()[6] as nat==u16::MAX as nat-274nat) by (compute);
    assert(zm::be_value(z_ok_ct6_d16_il0_17x2(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_ok_ct6_d16_il0_17x2(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_ok_ct6_d16_il0_17x2() ensures zm::Adler(raw_ok_ct6_d16_il0_17x2())==1011452042nat,
{
    hide(raw_ok_ct6_d16_il0_17x2);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_ok_ct6_d16_il0_17x2().len()==274nat) by (compute);
    assert(c::sums(raw_ok_ct6_d16_il0_17x2(),0,274nat)==(34953nat,4798466nat)) by (compute);
    c::sums_correct(raw_ok_ct6_d16_il0_17x2(),0,274nat);assert(raw_ok_ct6_d16_il0_17x2().subrange(0,274)=~=raw_ok_ct6_d16_il0_17x2());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((4798466nat%65521)*(u16::MAX as nat+1)+(1+34953nat)%65521==1011452042nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_ok_ct6_d16_il0_17x2() ensures zm::Inflate(z_ok_ct6_d16_il0_17x2(),274nat,true,false)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_ok_ct6_d16_il0_17x2(),r2:false}),raw_ok_ct6_d16_il0_17x2().len()==274nat,
{
    hide(z_ok_ct6_d16_il0_17x2);hide(raw_ok_ct6_d16_il0_17x2);hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_ok_ct6_d16_il0_17x2();checksum_ok_ct6_d16_il0_17x2();
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_ok_ct6_d16_il0_17x2(),raw_ok_ct6_d16_il0_17x2(),274nat,120nat,156nat,1011452042nat);
}
#[verifier::spinoff_prover]
proof fn crc_ok_ct6_d16_il0_17x2_crc_33()
    ensures crate::model::crc(bytes_ok_ct6_d16_il0_17x2().subrange(37,326))==2204185282u32,
{
    hide(crate::model::crc_register);hide(crc_cert::fold);
    assert(crate::model::crc_register(bytes_ok_ct6_d16_il0_17x2().subrange(37,326).take(0))==0xffff_ffffu32) by {
        assert(bytes_ok_ct6_d16_il0_17x2().subrange(37,326).take(0) =~= Seq::<u8>::empty());
        reveal_with_fuel(crate::model::crc_register,1);
    }
    assert(crc_cert::fold(4294967295u32,bytes_ok_ct6_d16_il0_17x2().subrange(37,326).subrange(0,64))==1344887839u32) by (compute);
    crc_cert::suffix(bytes_ok_ct6_d16_il0_17x2().subrange(37,326).take(0),bytes_ok_ct6_d16_il0_17x2().subrange(37,326).subrange(0,64));
    assert(bytes_ok_ct6_d16_il0_17x2().subrange(37,326).take(0)+bytes_ok_ct6_d16_il0_17x2().subrange(37,326).subrange(0,64) =~= bytes_ok_ct6_d16_il0_17x2().subrange(37,326).take(64));
    assert(crate::model::crc_register(bytes_ok_ct6_d16_il0_17x2().subrange(37,326).take(64))==1344887839u32);
    assert(crc_cert::fold(1344887839u32,bytes_ok_ct6_d16_il0_17x2().subrange(37,326).subrange(64,128))==2088141945u32) by (compute);
    crc_cert::suffix(bytes_ok_ct6_d16_il0_17x2().subrange(37,326).take(64),bytes_ok_ct6_d16_il0_17x2().subrange(37,326).subrange(64,128));
    assert(bytes_ok_ct6_d16_il0_17x2().subrange(37,326).take(64)+bytes_ok_ct6_d16_il0_17x2().subrange(37,326).subrange(64,128) =~= bytes_ok_ct6_d16_il0_17x2().subrange(37,326).take(128));
    assert(crate::model::crc_register(bytes_ok_ct6_d16_il0_17x2().subrange(37,326).take(128))==2088141945u32);
    assert(crc_cert::fold(2088141945u32,bytes_ok_ct6_d16_il0_17x2().subrange(37,326).subrange(128,192))==1277629949u32) by (compute);
    crc_cert::suffix(bytes_ok_ct6_d16_il0_17x2().subrange(37,326).take(128),bytes_ok_ct6_d16_il0_17x2().subrange(37,326).subrange(128,192));
    assert(bytes_ok_ct6_d16_il0_17x2().subrange(37,326).take(128)+bytes_ok_ct6_d16_il0_17x2().subrange(37,326).subrange(128,192) =~= bytes_ok_ct6_d16_il0_17x2().subrange(37,326).take(192));
    assert(crate::model::crc_register(bytes_ok_ct6_d16_il0_17x2().subrange(37,326).take(192))==1277629949u32);
    assert(crc_cert::fold(1277629949u32,bytes_ok_ct6_d16_il0_17x2().subrange(37,326).subrange(192,256))==269795151u32) by (compute);
    crc_cert::suffix(bytes_ok_ct6_d16_il0_17x2().subrange(37,326).take(192),bytes_ok_ct6_d16_il0_17x2().subrange(37,326).subrange(192,256));
    assert(bytes_ok_ct6_d16_il0_17x2().subrange(37,326).take(192)+bytes_ok_ct6_d16_il0_17x2().subrange(37,326).subrange(192,256) =~= bytes_ok_ct6_d16_il0_17x2().subrange(37,326).take(256));
    assert(crate::model::crc_register(bytes_ok_ct6_d16_il0_17x2().subrange(37,326).take(256))==269795151u32);
    assert(crc_cert::fold(269795151u32,bytes_ok_ct6_d16_il0_17x2().subrange(37,326).subrange(256,289))==2090782013u32) by (compute);
    crc_cert::suffix(bytes_ok_ct6_d16_il0_17x2().subrange(37,326).take(256),bytes_ok_ct6_d16_il0_17x2().subrange(37,326).subrange(256,289));
    assert(bytes_ok_ct6_d16_il0_17x2().subrange(37,326).take(256)+bytes_ok_ct6_d16_il0_17x2().subrange(37,326).subrange(256,289) =~= bytes_ok_ct6_d16_il0_17x2().subrange(37,326).take(289));
    assert(crate::model::crc_register(bytes_ok_ct6_d16_il0_17x2().subrange(37,326).take(289))==2090782013u32);
    assert(bytes_ok_ct6_d16_il0_17x2().subrange(37,326).take(289) =~= bytes_ok_ct6_d16_il0_17x2().subrange(37,326));
    assert((2090782013u32 ^ 0xffff_ffffu32)==2204185282u32) by (compute);
}
#[verifier::spinoff_prover]
proof fn walk_ok_ct6_d16_il0_17x2(b: Seq<u8>)
    requires b.len()==342,
        crate::model::mh(b)==Ok((h_ok_ct6_d16_il0_17x2(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:285nat,next:330nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,330nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:338nat,len:0nat,next:342nat,crc_ok:true,synthetic:false})),
        b.subrange(41,326)==z_ok_ct6_d16_il0_17x2(),
    ensures r::default_case(b,h_ok_ct6_d16_il0_17x2(),z_ok_ct6_d16_il0_17x2(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
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
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    reveal_with_fuel(m::drive,3);
    reveal_with_fuel(m::concat_ranges,3);
}
#[verifier::spinoff_prover]
proof fn scan_ok_ct6_d16_il0_17x2() ensures r::scan(raw_ok_ct6_d16_il0_17x2(),a_ok_ct6_d16_il0_17x2(),0)==Ok(()),
{
    hide(raw_ok_ct6_d16_il0_17x2);assert(raw_ok_ct6_d16_il0_17x2().len()==274nat) by (compute);assert(r::count(a_ok_ct6_d16_il0_17x2())==1nat) by (compute);
    assert(r::pw(a_ok_ct6_d16_il0_17x2(),0nat)==17nat && r::ph(a_ok_ct6_d16_il0_17x2(),0nat)==2nat && r::rb(a_ok_ct6_d16_il0_17x2(),0nat)==136nat && r::base(a_ok_ct6_d16_il0_17x2(),0nat)==0nat) by (compute);
    assert(raw_ok_ct6_d16_il0_17x2()[0]==0u8) by (compute);
    assert(r::row_result(raw_ok_ct6_d16_il0_17x2(),a_ok_ct6_d16_il0_17x2(),0nat,0nat)==Ok(()));
    assert(raw_ok_ct6_d16_il0_17x2()[137]==1u8) by (compute);
    assert(r::row_result(raw_ok_ct6_d16_il0_17x2(),a_ok_ct6_d16_il0_17x2(),0nat,1nat)==Ok(()));
    r::scan_step(raw_ok_ct6_d16_il0_17x2(),a_ok_ct6_d16_il0_17x2(),1nat);
    r::pass_step(raw_ok_ct6_d16_il0_17x2(),a_ok_ct6_d16_il0_17x2(),0nat,2nat);
    r::pass_step(raw_ok_ct6_d16_il0_17x2(),a_ok_ct6_d16_il0_17x2(),0nat,1nat);
    r::pass_step(raw_ok_ct6_d16_il0_17x2(),a_ok_ct6_d16_il0_17x2(),0nat,0nat);
    r::scan_step(raw_ok_ct6_d16_il0_17x2(),a_ok_ct6_d16_il0_17x2(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_ok_ct6_d16_il0_17x2() ensures r::still(bytes_ok_ct6_d16_il0_17x2(),STRICT,false)==(r::Projection::Ready {width:17u32,height:2u32,bytes:136nat}),
{
    hide(bytes_ok_ct6_d16_il0_17x2);hide(z_ok_ct6_d16_il0_17x2);hide(raw_ok_ct6_d16_il0_17x2);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    assert(bytes_ok_ct6_d16_il0_17x2().len()==342nat) by (compute);
    assert(crate::model::mh(bytes_ok_ct6_d16_il0_17x2())==Ok((h_ok_ct6_d16_il0_17x2(),33nat))) by {reveal(crate::model::mh);assert(crate::model::mh(bytes_ok_ct6_d16_il0_17x2())==Ok((h_ok_ct6_d16_il0_17x2(),33nat))) by (compute);}
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_ok_ct6_d16_il0_17x2(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:285nat,next:330nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::model::frame(bytes_ok_ct6_d16_il0_17x2(),33nat)==Ok(crate::model::WireChunk {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:285nat,next:330nat})) by (compute);
            assert(crate::model::be32(bytes_ok_ct6_d16_il0_17x2(),326)==2204185282u32) by (compute);
            crc_ok_ct6_d16_il0_17x2_crc_33();
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_ok_ct6_d16_il0_17x2(),330nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:338nat,len:0nat,next:342nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_ok_ct6_d16_il0_17x2(),330nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:338nat,len:0nat,next:342nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_ok_ct6_d16_il0_17x2().subrange(41,326)==z_ok_ct6_d16_il0_17x2()) by (compute);
    walk_ok_ct6_d16_il0_17x2(bytes_ok_ct6_d16_il0_17x2());
    inflate_ok_ct6_d16_il0_17x2();scan_ok_ct6_d16_il0_17x2();
    assert(r::raw(a_ok_ct6_d16_il0_17x2())==274nat) by (compute);
    assert(r::data(z_ok_ct6_d16_il0_17x2(),a_ok_ct6_d16_il0_17x2(),STRICT)==Some(Ok(()))) by {reveal(r::data);};
    r::default_ready(bytes_ok_ct6_d16_il0_17x2(),h_ok_ct6_d16_il0_17x2(),z_ok_ct6_d16_il0_17x2(),0nat,false);
}
fn public_ok_ct6_d16_il0_17x2() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,17u8,0u8,0u8,0u8,2u8,16u8,6u8,0u8,0u8,0u8,238u8,191u8,235u8,140u8,0u8,0u8,1u8,29u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,18u8,1u8,237u8,254u8,0u8,200u8,236u8,191u8,210u8,133u8,133u8,243u8,205u8,212u8,35u8,59u8,11u8,12u8,225u8,219u8,53u8,9u8,199u8,61u8,203u8,200u8,17u8,38u8,1u8,251u8,178u8,175u8,19u8,83u8,213u8,63u8,235u8,8u8,28u8,131u8,200u8,74u8,18u8,57u8,195u8,156u8,39u8,85u8,138u8,18u8,193u8,72u8,74u8,145u8,202u8,25u8,17u8,252u8,79u8,98u8,199u8,78u8,49u8,35u8,137u8,109u8,160u8,122u8,246u8,156u8,26u8,101u8,55u8,217u8,214u8,146u8,237u8,31u8,194u8,74u8,10u8,23u8,138u8,136u8,84u8,94u8,170u8,127u8,233u8,253u8,161u8,14u8,33u8,138u8,221u8,209u8,232u8,168u8,82u8,148u8,68u8,246u8,233u8,75u8,80u8,196u8,177u8,147u8,130u8,136u8,244u8,88u8,92u8,210u8,155u8,151u8,47u8,92u8,47u8,146u8,30u8,236u8,175u8,74u8,36u8,220u8,133u8,246u8,35u8,160u8,236u8,47u8,126u8,96u8,31u8,91u8,63u8,233u8,194u8,247u8,25u8,1u8,215u8,143u8,70u8,49u8,150u8,177u8,145u8,38u8,147u8,92u8,8u8,27u8,115u8,227u8,162u8,80u8,154u8,237u8,81u8,239u8,121u8,151u8,161u8,97u8,33u8,209u8,101u8,243u8,251u8,137u8,189u8,157u8,234u8,7u8,138u8,240u8,101u8,119u8,239u8,156u8,199u8,18u8,95u8,213u8,219u8,67u8,135u8,112u8,76u8,249u8,54u8,89u8,82u8,172u8,128u8,222u8,119u8,237u8,203u8,129u8,6u8,200u8,179u8,73u8,86u8,207u8,40u8,217u8,154u8,129u8,103u8,80u8,56u8,249u8,51u8,113u8,124u8,173u8,132u8,29u8,53u8,43u8,30u8,215u8,79u8,139u8,5u8,198u8,211u8,148u8,162u8,26u8,145u8,133u8,81u8,203u8,128u8,228u8,121u8,58u8,209u8,96u8,63u8,17u8,43u8,217u8,88u8,34u8,242u8,62u8,84u8,118u8,163u8,194u8,180u8,162u8,78u8,190u8,172u8,3u8,96u8,247u8,16u8,34u8,253u8,175u8,191u8,141u8,206u8,16u8,66u8,91u8,8u8,52u8,220u8,80u8,60u8,73u8,136u8,138u8,131u8,97u8,50u8,194u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_ok_ct6_d16_il0_17x2());model_ok_ct6_d16_il0_17x2();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==17u32 && s.height==2u32 && s.rgba@.len()==136nat,Err(crate::alloc_guard::TryError::OutOfMemory(_))=>true,_=>false});
}
// ok16/ct6-d16-il1-7x5; frozen SHA256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_ok16_ct6_d16_il1_7x5() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,7u8,0u8,0u8,0u8,5u8,16u8,6u8,0u8,0u8,1u8,174u8,13u8,26u8,13u8,0u8,0u8,1u8,46u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,35u8,1u8,220u8,254u8,0u8,59u8,108u8,195u8,232u8,238u8,143u8,16u8,60u8,1u8,193u8,247u8,81u8,34u8,168u8,232u8,181u8,62u8,2u8,26u8,42u8,40u8,38u8,92u8,55u8,108u8,51u8,248u8,33u8,126u8,234u8,148u8,216u8,58u8,42u8,3u8,253u8,24u8,209u8,185u8,192u8,209u8,38u8,130u8,211u8,185u8,162u8,86u8,47u8,144u8,95u8,241u8,4u8,179u8,236u8,90u8,224u8,18u8,224u8,245u8,40u8,70u8,46u8,187u8,215u8,64u8,57u8,55u8,241u8,0u8,200u8,27u8,124u8,242u8,111u8,227u8,192u8,186u8,175u8,61u8,134u8,59u8,246u8,55u8,154u8,43u8,96u8,23u8,248u8,248u8,147u8,209u8,67u8,168u8,78u8,23u8,46u8,158u8,2u8,48u8,208u8,208u8,1u8,28u8,60u8,210u8,196u8,141u8,203u8,87u8,242u8,57u8,6u8,74u8,32u8,91u8,69u8,237u8,182u8,121u8,45u8,21u8,46u8,15u8,201u8,85u8,36u8,2u8,177u8,178u8,58u8,254u8,133u8,53u8,235u8,63u8,42u8,73u8,90u8,149u8,176u8,252u8,209u8,219u8,228u8,219u8,178u8,99u8,135u8,238u8,54u8,133u8,3u8,164u8,136u8,220u8,205u8,34u8,154u8,228u8,98u8,105u8,141u8,24u8,27u8,49u8,89u8,146u8,29u8,234u8,53u8,240u8,242u8,247u8,186u8,16u8,179u8,4u8,0u8,220u8,29u8,26u8,118u8,177u8,65u8,45u8,227u8,236u8,228u8,74u8,233u8,229u8,58u8,203u8,21u8,235u8,108u8,67u8,58u8,209u8,75u8,20u8,101u8,96u8,250u8,104u8,244u8,20u8,171u8,181u8,119u8,34u8,97u8,166u8,247u8,160u8,61u8,235u8,136u8,132u8,60u8,80u8,87u8,229u8,47u8,117u8,58u8,121u8,44u8,110u8,129u8,211u8,76u8,216u8,0u8,255u8,174u8,102u8,216u8,219u8,115u8,67u8,254u8,143u8,66u8,237u8,49u8,205u8,243u8,161u8,154u8,173u8,249u8,13u8,58u8,237u8,242u8,86u8,228u8,65u8,111u8,111u8,122u8,112u8,131u8,32u8,168u8,42u8,125u8,185u8,102u8,15u8,63u8,91u8,202u8,84u8,214u8,23u8,236u8,200u8,149u8,225u8,245u8,244u8,185u8,51u8,190u8,132u8,30u8,120u8,145u8,108u8,11u8,145u8,94u8,219u8,87u8,178u8,77u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn h_ok16_ct6_d16_il1_7x5() -> crate::model::Ihdr {crate::model::Ihdr {width:7u32,height:5u32,depth:16u8,color_type:6u8,interlace:1u8}}
spec fn a_ok16_ct6_d16_il1_7x5() -> r::Raster {r::shape(h_ok16_ct6_d16_il1_7x5(),7nat,5nat,0nat,STRICT)}
spec fn z_ok16_ct6_d16_il1_7x5() -> Seq<u8> {seq![120u8,156u8,1u8,35u8,1u8,220u8,254u8,0u8,59u8,108u8,195u8,232u8,238u8,143u8,16u8,60u8,1u8,193u8,247u8,81u8,34u8,168u8,232u8,181u8,62u8,2u8,26u8,42u8,40u8,38u8,92u8,55u8,108u8,51u8,248u8,33u8,126u8,234u8,148u8,216u8,58u8,42u8,3u8,253u8,24u8,209u8,185u8,192u8,209u8,38u8,130u8,211u8,185u8,162u8,86u8,47u8,144u8,95u8,241u8,4u8,179u8,236u8,90u8,224u8,18u8,224u8,245u8,40u8,70u8,46u8,187u8,215u8,64u8,57u8,55u8,241u8,0u8,200u8,27u8,124u8,242u8,111u8,227u8,192u8,186u8,175u8,61u8,134u8,59u8,246u8,55u8,154u8,43u8,96u8,23u8,248u8,248u8,147u8,209u8,67u8,168u8,78u8,23u8,46u8,158u8,2u8,48u8,208u8,208u8,1u8,28u8,60u8,210u8,196u8,141u8,203u8,87u8,242u8,57u8,6u8,74u8,32u8,91u8,69u8,237u8,182u8,121u8,45u8,21u8,46u8,15u8,201u8,85u8,36u8,2u8,177u8,178u8,58u8,254u8,133u8,53u8,235u8,63u8,42u8,73u8,90u8,149u8,176u8,252u8,209u8,219u8,228u8,219u8,178u8,99u8,135u8,238u8,54u8,133u8,3u8,164u8,136u8,220u8,205u8,34u8,154u8,228u8,98u8,105u8,141u8,24u8,27u8,49u8,89u8,146u8,29u8,234u8,53u8,240u8,242u8,247u8,186u8,16u8,179u8,4u8,0u8,220u8,29u8,26u8,118u8,177u8,65u8,45u8,227u8,236u8,228u8,74u8,233u8,229u8,58u8,203u8,21u8,235u8,108u8,67u8,58u8,209u8,75u8,20u8,101u8,96u8,250u8,104u8,244u8,20u8,171u8,181u8,119u8,34u8,97u8,166u8,247u8,160u8,61u8,235u8,136u8,132u8,60u8,80u8,87u8,229u8,47u8,117u8,58u8,121u8,44u8,110u8,129u8,211u8,76u8,216u8,0u8,255u8,174u8,102u8,216u8,219u8,115u8,67u8,254u8,143u8,66u8,237u8,49u8,205u8,243u8,161u8,154u8,173u8,249u8,13u8,58u8,237u8,242u8,86u8,228u8,65u8,111u8,111u8,122u8,112u8,131u8,32u8,168u8,42u8,125u8,185u8,102u8,15u8,63u8,91u8,202u8,84u8,214u8,23u8,236u8,200u8,149u8,225u8,245u8,244u8,185u8,51u8,190u8,132u8,30u8,120u8,145u8,108u8,11u8,145u8,94u8]}
spec fn raw_ok16_ct6_d16_il1_7x5() -> Seq<u8> {seq![0u8,59u8,108u8,195u8,232u8,238u8,143u8,16u8,60u8,1u8,193u8,247u8,81u8,34u8,168u8,232u8,181u8,62u8,2u8,26u8,42u8,40u8,38u8,92u8,55u8,108u8,51u8,248u8,33u8,126u8,234u8,148u8,216u8,58u8,42u8,3u8,253u8,24u8,209u8,185u8,192u8,209u8,38u8,130u8,211u8,185u8,162u8,86u8,47u8,144u8,95u8,241u8,4u8,179u8,236u8,90u8,224u8,18u8,224u8,245u8,40u8,70u8,46u8,187u8,215u8,64u8,57u8,55u8,241u8,0u8,200u8,27u8,124u8,242u8,111u8,227u8,192u8,186u8,175u8,61u8,134u8,59u8,246u8,55u8,154u8,43u8,96u8,23u8,248u8,248u8,147u8,209u8,67u8,168u8,78u8,23u8,46u8,158u8,2u8,48u8,208u8,208u8,1u8,28u8,60u8,210u8,196u8,141u8,203u8,87u8,242u8,57u8,6u8,74u8,32u8,91u8,69u8,237u8,182u8,121u8,45u8,21u8,46u8,15u8,201u8,85u8,36u8,2u8,177u8,178u8,58u8,254u8,133u8,53u8,235u8,63u8,42u8,73u8,90u8,149u8,176u8,252u8,209u8,219u8,228u8,219u8,178u8,99u8,135u8,238u8,54u8,133u8,3u8,164u8,136u8,220u8,205u8,34u8,154u8,228u8,98u8,105u8,141u8,24u8,27u8,49u8,89u8,146u8,29u8,234u8,53u8,240u8,242u8,247u8,186u8,16u8,179u8,4u8,0u8,220u8,29u8,26u8,118u8,177u8,65u8,45u8,227u8,236u8,228u8,74u8,233u8,229u8,58u8,203u8,21u8,235u8,108u8,67u8,58u8,209u8,75u8,20u8,101u8,96u8,250u8,104u8,244u8,20u8,171u8,181u8,119u8,34u8,97u8,166u8,247u8,160u8,61u8,235u8,136u8,132u8,60u8,80u8,87u8,229u8,47u8,117u8,58u8,121u8,44u8,110u8,129u8,211u8,76u8,216u8,0u8,255u8,174u8,102u8,216u8,219u8,115u8,67u8,254u8,143u8,66u8,237u8,49u8,205u8,243u8,161u8,154u8,173u8,249u8,13u8,58u8,237u8,242u8,86u8,228u8,65u8,111u8,111u8,122u8,112u8,131u8,32u8,168u8,42u8,125u8,185u8,102u8,15u8,63u8,91u8,202u8,84u8,214u8,23u8,236u8,200u8,149u8,225u8,245u8,244u8,185u8,51u8,190u8,132u8,30u8,120u8,145u8]}
#[verifier::spinoff_prover]
proof fn shape_ok16_ct6_d16_il1_7x5() ensures z_ok16_ct6_d16_il1_7x5().len()==302nat,
        raw_ok16_ct6_d16_il1_7x5().len()==291nat,
        z_ok16_ct6_d16_il1_7x5()[0]==120u8,
        z_ok16_ct6_d16_il1_7x5()[1]==156u8,
        zm::field_value(z_ok16_ct6_d16_il1_7x5(),16,1)==1nat,
        zm::field_value(z_ok16_ct6_d16_il1_7x5(),17,2)==0nat,
        zm::be_value(z_ok16_ct6_d16_il1_7x5(),298nat,4)==1812697438nat,
        z_ok16_ct6_d16_il1_7x5().subrange(7,298)==raw_ok16_ct6_d16_il1_7x5(),
        z_ok16_ct6_d16_il1_7x5()[3] as nat+256*z_ok16_ct6_d16_il1_7x5()[4] as nat==291nat,
        z_ok16_ct6_d16_il1_7x5()[5] as nat+256*z_ok16_ct6_d16_il1_7x5()[6] as nat==u16::MAX as nat-291nat,
        zm::be_value(z_ok16_ct6_d16_il1_7x5(),0,1)==120nat,
        zm::be_value(z_ok16_ct6_d16_il1_7x5(),1,1)==156nat,
{
    assert(z_ok16_ct6_d16_il1_7x5().len()==302nat) by (compute);
    assert(raw_ok16_ct6_d16_il1_7x5().len()==291nat) by (compute);
    assert(z_ok16_ct6_d16_il1_7x5()[0]==120u8) by (compute);
    assert(z_ok16_ct6_d16_il1_7x5()[1]==156u8) by (compute);
    assert(zm::field_value(z_ok16_ct6_d16_il1_7x5(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_ok16_ct6_d16_il1_7x5(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_ok16_ct6_d16_il1_7x5(),298nat,4)==1812697438nat) by (compute);
    assert(z_ok16_ct6_d16_il1_7x5().subrange(7,298)==raw_ok16_ct6_d16_il1_7x5()) by (compute);
    assert(z_ok16_ct6_d16_il1_7x5()[3] as nat+256*z_ok16_ct6_d16_il1_7x5()[4] as nat==291nat) by (compute);
    assert(z_ok16_ct6_d16_il1_7x5()[5] as nat+256*z_ok16_ct6_d16_il1_7x5()[6] as nat==u16::MAX as nat-291nat) by (compute);
    assert(zm::be_value(z_ok16_ct6_d16_il1_7x5(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_ok16_ct6_d16_il1_7x5(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_ok16_ct6_d16_il1_7x5() ensures zm::Adler(raw_ok16_ct6_d16_il1_7x5())==1812697438nat,
{
    hide(raw_ok16_ct6_d16_il1_7x5);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_ok16_ct6_d16_il1_7x5().len()==291nat) by (compute);
    assert(c::sums(raw_ok16_ct6_d16_il1_7x5(),0,291nat)==(37213nat,5269339nat)) by (compute);
    c::sums_correct(raw_ok16_ct6_d16_il1_7x5(),0,291nat);assert(raw_ok16_ct6_d16_il1_7x5().subrange(0,291)=~=raw_ok16_ct6_d16_il1_7x5());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((5269339nat%65521)*(u16::MAX as nat+1)+(1+37213nat)%65521==1812697438nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_ok16_ct6_d16_il1_7x5() ensures zm::Inflate(z_ok16_ct6_d16_il1_7x5(),291nat,true,false)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_ok16_ct6_d16_il1_7x5(),r2:false}),raw_ok16_ct6_d16_il1_7x5().len()==291nat,
{
    hide(z_ok16_ct6_d16_il1_7x5);hide(raw_ok16_ct6_d16_il1_7x5);hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_ok16_ct6_d16_il1_7x5();checksum_ok16_ct6_d16_il1_7x5();
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_ok16_ct6_d16_il1_7x5(),raw_ok16_ct6_d16_il1_7x5(),291nat,120nat,156nat,1812697438nat);
}
#[verifier::spinoff_prover]
proof fn crc_ok16_ct6_d16_il1_7x5_crc_33()
    ensures crate::model::crc(bytes_ok16_ct6_d16_il1_7x5().subrange(37,343))==3679957581u32,
{
    hide(crate::model::crc_register);hide(crc_cert::fold);
    assert(crate::model::crc_register(bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).take(0))==0xffff_ffffu32) by {
        assert(bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).take(0) =~= Seq::<u8>::empty());
        reveal_with_fuel(crate::model::crc_register,1);
    }
    assert(crc_cert::fold(4294967295u32,bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).subrange(0,64))==3170902788u32) by (compute);
    crc_cert::suffix(bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).take(0),bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).subrange(0,64));
    assert(bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).take(0)+bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).subrange(0,64) =~= bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).take(64));
    assert(crate::model::crc_register(bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).take(64))==3170902788u32);
    assert(crc_cert::fold(3170902788u32,bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).subrange(64,128))==3771341361u32) by (compute);
    crc_cert::suffix(bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).take(64),bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).subrange(64,128));
    assert(bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).take(64)+bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).subrange(64,128) =~= bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).take(128));
    assert(crate::model::crc_register(bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).take(128))==3771341361u32);
    assert(crc_cert::fold(3771341361u32,bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).subrange(128,192))==2954573370u32) by (compute);
    crc_cert::suffix(bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).take(128),bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).subrange(128,192));
    assert(bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).take(128)+bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).subrange(128,192) =~= bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).take(192));
    assert(crate::model::crc_register(bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).take(192))==2954573370u32);
    assert(crc_cert::fold(2954573370u32,bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).subrange(192,256))==2558444316u32) by (compute);
    crc_cert::suffix(bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).take(192),bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).subrange(192,256));
    assert(bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).take(192)+bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).subrange(192,256) =~= bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).take(256));
    assert(crate::model::crc_register(bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).take(256))==2558444316u32);
    assert(crc_cert::fold(2558444316u32,bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).subrange(256,306))==615009714u32) by (compute);
    crc_cert::suffix(bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).take(256),bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).subrange(256,306));
    assert(bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).take(256)+bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).subrange(256,306) =~= bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).take(306));
    assert(crate::model::crc_register(bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).take(306))==615009714u32);
    assert(bytes_ok16_ct6_d16_il1_7x5().subrange(37,343).take(306) =~= bytes_ok16_ct6_d16_il1_7x5().subrange(37,343));
    assert((615009714u32 ^ 0xffff_ffffu32)==3679957581u32) by (compute);
}
#[verifier::spinoff_prover]
proof fn walk_ok16_ct6_d16_il1_7x5(b: Seq<u8>)
    requires b.len()==359,
        crate::model::mh(b)==Ok((h_ok16_ct6_d16_il1_7x5(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:302nat,next:347nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,347nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:355nat,len:0nat,next:359nat,crc_ok:true,synthetic:false})),
        b.subrange(41,343)==z_ok16_ct6_d16_il1_7x5(),
    ensures r::default_case(b,h_ok16_ct6_d16_il1_7x5(),z_ok16_ct6_d16_il1_7x5(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
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
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    reveal_with_fuel(m::drive,3);
    reveal_with_fuel(m::concat_ranges,3);
}
#[verifier::spinoff_prover]
proof fn scan_ok16_ct6_d16_il1_7x5() ensures r::scan(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),0)==Ok(()),
{
    hide(raw_ok16_ct6_d16_il1_7x5);assert(raw_ok16_ct6_d16_il1_7x5().len()==291nat) by (compute);assert(r::count(a_ok16_ct6_d16_il1_7x5())==7nat) by (compute);
    assert(r::pw(a_ok16_ct6_d16_il1_7x5(),0nat)==1nat && r::ph(a_ok16_ct6_d16_il1_7x5(),0nat)==1nat && r::rb(a_ok16_ct6_d16_il1_7x5(),0nat)==8nat && r::base(a_ok16_ct6_d16_il1_7x5(),0nat)==0nat) by (compute);
    assert(raw_ok16_ct6_d16_il1_7x5()[0]==0u8) by (compute);
    assert(r::row_result(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),0nat,0nat)==Ok(()));
    assert(r::pw(a_ok16_ct6_d16_il1_7x5(),1nat)==1nat && r::ph(a_ok16_ct6_d16_il1_7x5(),1nat)==1nat && r::rb(a_ok16_ct6_d16_il1_7x5(),1nat)==8nat && r::base(a_ok16_ct6_d16_il1_7x5(),1nat)==9nat) by (compute);
    assert(raw_ok16_ct6_d16_il1_7x5()[9]==1u8) by (compute);
    assert(r::row_result(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),1nat,0nat)==Ok(()));
    assert(r::pw(a_ok16_ct6_d16_il1_7x5(),2nat)==2nat && r::ph(a_ok16_ct6_d16_il1_7x5(),2nat)==1nat && r::rb(a_ok16_ct6_d16_il1_7x5(),2nat)==16nat && r::base(a_ok16_ct6_d16_il1_7x5(),2nat)==18nat) by (compute);
    assert(raw_ok16_ct6_d16_il1_7x5()[18]==2u8) by (compute);
    assert(r::row_result(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),2nat,0nat)==Ok(()));
    assert(r::pw(a_ok16_ct6_d16_il1_7x5(),3nat)==2nat && r::ph(a_ok16_ct6_d16_il1_7x5(),3nat)==2nat && r::rb(a_ok16_ct6_d16_il1_7x5(),3nat)==16nat && r::base(a_ok16_ct6_d16_il1_7x5(),3nat)==35nat) by (compute);
    assert(raw_ok16_ct6_d16_il1_7x5()[35]==3u8) by (compute);
    assert(r::row_result(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),3nat,0nat)==Ok(()));
    assert(raw_ok16_ct6_d16_il1_7x5()[52]==4u8) by (compute);
    assert(r::row_result(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),3nat,1nat)==Ok(()));
    assert(r::pw(a_ok16_ct6_d16_il1_7x5(),4nat)==4nat && r::ph(a_ok16_ct6_d16_il1_7x5(),4nat)==1nat && r::rb(a_ok16_ct6_d16_il1_7x5(),4nat)==32nat && r::base(a_ok16_ct6_d16_il1_7x5(),4nat)==69nat) by (compute);
    assert(raw_ok16_ct6_d16_il1_7x5()[69]==0u8) by (compute);
    assert(r::row_result(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),4nat,0nat)==Ok(()));
    assert(r::pw(a_ok16_ct6_d16_il1_7x5(),5nat)==3nat && r::ph(a_ok16_ct6_d16_il1_7x5(),5nat)==3nat && r::rb(a_ok16_ct6_d16_il1_7x5(),5nat)==24nat && r::base(a_ok16_ct6_d16_il1_7x5(),5nat)==102nat) by (compute);
    assert(raw_ok16_ct6_d16_il1_7x5()[102]==1u8) by (compute);
    assert(r::row_result(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),5nat,0nat)==Ok(()));
    assert(raw_ok16_ct6_d16_il1_7x5()[127]==2u8) by (compute);
    assert(r::row_result(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),5nat,1nat)==Ok(()));
    assert(raw_ok16_ct6_d16_il1_7x5()[152]==3u8) by (compute);
    assert(r::row_result(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),5nat,2nat)==Ok(()));
    assert(r::pw(a_ok16_ct6_d16_il1_7x5(),6nat)==7nat && r::ph(a_ok16_ct6_d16_il1_7x5(),6nat)==2nat && r::rb(a_ok16_ct6_d16_il1_7x5(),6nat)==56nat && r::base(a_ok16_ct6_d16_il1_7x5(),6nat)==177nat) by (compute);
    assert(raw_ok16_ct6_d16_il1_7x5()[177]==4u8) by (compute);
    assert(r::row_result(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),6nat,0nat)==Ok(()));
    assert(raw_ok16_ct6_d16_il1_7x5()[234]==0u8) by (compute);
    assert(r::row_result(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),6nat,1nat)==Ok(()));
    r::scan_step(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),7nat);
    r::pass_step(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),6nat,2nat);
    r::pass_step(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),6nat,1nat);
    r::pass_step(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),6nat,0nat);
    r::scan_step(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),6nat);
    r::pass_step(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),5nat,3nat);
    r::pass_step(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),5nat,2nat);
    r::pass_step(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),5nat,1nat);
    r::pass_step(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),5nat,0nat);
    r::scan_step(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),5nat);
    r::pass_step(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),4nat,1nat);
    r::pass_step(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),4nat,0nat);
    r::scan_step(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),4nat);
    r::pass_step(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),3nat,2nat);
    r::pass_step(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),3nat,1nat);
    r::pass_step(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),3nat,0nat);
    r::scan_step(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),3nat);
    r::pass_step(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),2nat,1nat);
    r::pass_step(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),2nat,0nat);
    r::scan_step(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),2nat);
    r::pass_step(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),1nat,1nat);
    r::pass_step(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),1nat,0nat);
    r::scan_step(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),1nat);
    r::pass_step(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),0nat,1nat);
    r::pass_step(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),0nat,0nat);
    r::scan_step(raw_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_ok16_ct6_d16_il1_7x5() ensures r::still(bytes_ok16_ct6_d16_il1_7x5(),STRICT,true)==(r::Projection::Ready {width:7u32,height:5u32,bytes:280nat}),
{
    hide(bytes_ok16_ct6_d16_il1_7x5);hide(z_ok16_ct6_d16_il1_7x5);hide(raw_ok16_ct6_d16_il1_7x5);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    assert(bytes_ok16_ct6_d16_il1_7x5().len()==359nat) by (compute);
    assert(crate::model::mh(bytes_ok16_ct6_d16_il1_7x5())==Ok((h_ok16_ct6_d16_il1_7x5(),33nat))) by {reveal(crate::model::mh);assert(crate::model::mh(bytes_ok16_ct6_d16_il1_7x5())==Ok((h_ok16_ct6_d16_il1_7x5(),33nat))) by (compute);}
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_ok16_ct6_d16_il1_7x5(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:302nat,next:347nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::model::frame(bytes_ok16_ct6_d16_il1_7x5(),33nat)==Ok(crate::model::WireChunk {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:302nat,next:347nat})) by (compute);
            assert(crate::model::be32(bytes_ok16_ct6_d16_il1_7x5(),343)==3679957581u32) by (compute);
            crc_ok16_ct6_d16_il1_7x5_crc_33();
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_ok16_ct6_d16_il1_7x5(),347nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:355nat,len:0nat,next:359nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_ok16_ct6_d16_il1_7x5(),347nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:355nat,len:0nat,next:359nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_ok16_ct6_d16_il1_7x5().subrange(41,343)==z_ok16_ct6_d16_il1_7x5()) by (compute);
    walk_ok16_ct6_d16_il1_7x5(bytes_ok16_ct6_d16_il1_7x5());
    inflate_ok16_ct6_d16_il1_7x5();scan_ok16_ct6_d16_il1_7x5();
    assert(r::raw(a_ok16_ct6_d16_il1_7x5())==291nat) by (compute);
    assert(r::data(z_ok16_ct6_d16_il1_7x5(),a_ok16_ct6_d16_il1_7x5(),STRICT)==Some(Ok(()))) by {reveal(r::data);};
    r::default_ready(bytes_ok16_ct6_d16_il1_7x5(),h_ok16_ct6_d16_il1_7x5(),z_ok16_ct6_d16_il1_7x5(),0nat,true);
}
fn public_ok16_ct6_d16_il1_7x5() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,7u8,0u8,0u8,0u8,5u8,16u8,6u8,0u8,0u8,1u8,174u8,13u8,26u8,13u8,0u8,0u8,1u8,46u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,35u8,1u8,220u8,254u8,0u8,59u8,108u8,195u8,232u8,238u8,143u8,16u8,60u8,1u8,193u8,247u8,81u8,34u8,168u8,232u8,181u8,62u8,2u8,26u8,42u8,40u8,38u8,92u8,55u8,108u8,51u8,248u8,33u8,126u8,234u8,148u8,216u8,58u8,42u8,3u8,253u8,24u8,209u8,185u8,192u8,209u8,38u8,130u8,211u8,185u8,162u8,86u8,47u8,144u8,95u8,241u8,4u8,179u8,236u8,90u8,224u8,18u8,224u8,245u8,40u8,70u8,46u8,187u8,215u8,64u8,57u8,55u8,241u8,0u8,200u8,27u8,124u8,242u8,111u8,227u8,192u8,186u8,175u8,61u8,134u8,59u8,246u8,55u8,154u8,43u8,96u8,23u8,248u8,248u8,147u8,209u8,67u8,168u8,78u8,23u8,46u8,158u8,2u8,48u8,208u8,208u8,1u8,28u8,60u8,210u8,196u8,141u8,203u8,87u8,242u8,57u8,6u8,74u8,32u8,91u8,69u8,237u8,182u8,121u8,45u8,21u8,46u8,15u8,201u8,85u8,36u8,2u8,177u8,178u8,58u8,254u8,133u8,53u8,235u8,63u8,42u8,73u8,90u8,149u8,176u8,252u8,209u8,219u8,228u8,219u8,178u8,99u8,135u8,238u8,54u8,133u8,3u8,164u8,136u8,220u8,205u8,34u8,154u8,228u8,98u8,105u8,141u8,24u8,27u8,49u8,89u8,146u8,29u8,234u8,53u8,240u8,242u8,247u8,186u8,16u8,179u8,4u8,0u8,220u8,29u8,26u8,118u8,177u8,65u8,45u8,227u8,236u8,228u8,74u8,233u8,229u8,58u8,203u8,21u8,235u8,108u8,67u8,58u8,209u8,75u8,20u8,101u8,96u8,250u8,104u8,244u8,20u8,171u8,181u8,119u8,34u8,97u8,166u8,247u8,160u8,61u8,235u8,136u8,132u8,60u8,80u8,87u8,229u8,47u8,117u8,58u8,121u8,44u8,110u8,129u8,211u8,76u8,216u8,0u8,255u8,174u8,102u8,216u8,219u8,115u8,67u8,254u8,143u8,66u8,237u8,49u8,205u8,243u8,161u8,154u8,173u8,249u8,13u8,58u8,237u8,242u8,86u8,228u8,65u8,111u8,111u8,122u8,112u8,131u8,32u8,168u8,42u8,125u8,185u8,102u8,15u8,63u8,91u8,202u8,84u8,214u8,23u8,236u8,200u8,149u8,225u8,245u8,244u8,185u8,51u8,190u8,132u8,30u8,120u8,145u8,108u8,11u8,145u8,94u8,219u8,87u8,178u8,77u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_ok16_ct6_d16_il1_7x5());model_ok16_ct6_d16_il1_7x5();}
    let result=crate::png::try_decode_still16(&b);
    assert(match result {Ok(s)=>s.width==7u32 && s.height==5u32 && s.rgba16@.len()==280nat,Err(crate::alloc_guard::TryError::OutOfMemory(_))=>true,_=>false});
}
// trns/grey-d1-masked; frozen SHA256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_trns_grey_d1_masked() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,5u8,0u8,0u8,0u8,3u8,1u8,0u8,0u8,0u8,0u8,115u8,77u8,248u8,85u8,0u8,0u8,0u8,2u8,116u8,82u8,78u8,83u8,255u8,1u8,146u8,176u8,0u8,220u8,0u8,0u8,0u8,14u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,88u8,192u8,184u8,130u8,105u8,3u8,0u8,5u8,214u8,1u8,252u8,84u8,21u8,181u8,192u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn h_trns_grey_d1_masked() -> crate::model::Ihdr {crate::model::Ihdr {width:5u32,height:3u32,depth:1u8,color_type:0u8,interlace:0u8}}
spec fn a_trns_grey_d1_masked() -> r::Raster {r::shape(h_trns_grey_d1_masked(),5nat,3nat,0nat,STRICT)}
spec fn z_trns_grey_d1_masked() -> Seq<u8> {seq![120u8,156u8,99u8,88u8,192u8,184u8,130u8,105u8,3u8,0u8,5u8,214u8,1u8,252u8]}
spec fn raw_trns_grey_d1_masked() -> Seq<u8> {seq![0u8,160u8,1u8,168u8,2u8,176u8]}
spec fn ts_trns_grey_d1_masked() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:36nat,before:1nat,after:2nat,symbol:160nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:36nat,next:44nat,before:2nat,after:3nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:44nat,next:53nat,before:3nat,after:4nat,symbol:168nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:53nat,next:61nat,before:4nat,after:5nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:61nat,next:70nat,before:5nat,after:6nat,symbol:176nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:70nat,next:77nat,before:6nat,after:6nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_trns_grey_d1_masked_0() ensures t::trace(z_trns_grey_d1_masked(),raw_trns_grey_d1_masked(),ts_trns_grey_d1_masked(),0nat,7nat),
{
    hide(t::valid_step);assert(t::trace(z_trns_grey_d1_masked(),raw_trns_grey_d1_masked(),ts_trns_grey_d1_masked(),0nat,7nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn trace_trns_grey_d1_masked() ensures t::trace(z_trns_grey_d1_masked(),raw_trns_grey_d1_masked(),ts_trns_grey_d1_masked(),0,7nat),
{
    hide(z_trns_grey_d1_masked);hide(raw_trns_grey_d1_masked);hide(ts_trns_grey_d1_masked);hide(t::trace);
    trace_trns_grey_d1_masked_0();
    assert(t::trace(z_trns_grey_d1_masked(),raw_trns_grey_d1_masked(),ts_trns_grey_d1_masked(),0,7nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_trns_grey_d1_masked() ensures z_trns_grey_d1_masked().len()==14nat,
        raw_trns_grey_d1_masked().len()==6nat,
        z_trns_grey_d1_masked()[0]==120u8,
        z_trns_grey_d1_masked()[1]==156u8,
        zm::field_value(z_trns_grey_d1_masked(),16,1)==1nat,
        zm::field_value(z_trns_grey_d1_masked(),17,2)==1nat,
        zm::be_value(z_trns_grey_d1_masked(),10nat,4)==97911292nat,
        ts_trns_grey_d1_masked().len()==7nat,
        ts_trns_grey_d1_masked()[0].pos==19nat,
        ts_trns_grey_d1_masked()[0].before==0nat,
        ts_trns_grey_d1_masked().last().next==77nat,
{
    assert(z_trns_grey_d1_masked().len()==14nat) by (compute);
    assert(raw_trns_grey_d1_masked().len()==6nat) by (compute);
    assert(z_trns_grey_d1_masked()[0]==120u8) by (compute);
    assert(z_trns_grey_d1_masked()[1]==156u8) by (compute);
    assert(zm::field_value(z_trns_grey_d1_masked(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_trns_grey_d1_masked(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_trns_grey_d1_masked(),10nat,4)==97911292nat) by (compute);
    assert(ts_trns_grey_d1_masked().len()==7nat) by (compute);
    assert(ts_trns_grey_d1_masked()[0].pos==19nat) by (compute);
    assert(ts_trns_grey_d1_masked()[0].before==0nat) by (compute);
    assert(ts_trns_grey_d1_masked().last().next==77nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_trns_grey_d1_masked() ensures zm::Adler(raw_trns_grey_d1_masked())==97911292nat,
{
    hide(raw_trns_grey_d1_masked);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_trns_grey_d1_masked().len()==6nat) by (compute);
    assert(c::sums(raw_trns_grey_d1_masked(),0,6nat)==(507nat,1494nat)) by (compute);
    c::sums_correct(raw_trns_grey_d1_masked(),0,6nat);assert(raw_trns_grey_d1_masked().subrange(0,6)=~=raw_trns_grey_d1_masked());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((1494nat%65521)*(u16::MAX as nat+1)+(1+507nat)%65521==97911292nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_trns_grey_d1_masked() ensures zm::Inflate(z_trns_grey_d1_masked(),6nat,true,false)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_trns_grey_d1_masked(),r2:false}),raw_trns_grey_d1_masked().len()==6nat,
{
    hide(z_trns_grey_d1_masked);hide(raw_trns_grey_d1_masked);hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);hide(ts_trns_grey_d1_masked);
    shape_trns_grey_d1_masked();checksum_trns_grey_d1_masked();
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_trns_grey_d1_masked();ms::fixed_stream(z_trns_grey_d1_masked(),raw_trns_grey_d1_masked(),ts_trns_grey_d1_masked(),6nat,156nat,97911292nat,97911292nat);
}
#[verifier::spinoff_prover]
proof fn walk_trns_grey_d1_masked(b: Seq<u8>)
    requires b.len()==85,
        crate::model::mh(b)==Ok((h_trns_grey_d1_masked(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:41nat,len:2nat,next:47nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,47nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:55nat,len:14nat,next:73nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,73nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:81nat,len:0nat,next:85nat,crc_ok:true,synthetic:false})),
        b.subrange(55,69)==z_trns_grey_d1_masked(),
    ensures r::default_case(b,h_trns_grey_d1_masked(),z_trns_grey_d1_masked(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert((1u32 << 1u8)==2u32) by (compute);
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by (compute);
        }
        assert(crate::metadata_model::transparency(2nat,0u8,0nat,false)==crate::metadata_model::Decision::Keep(2nat)) by (compute);
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
proof fn scan_trns_grey_d1_masked() ensures r::scan(raw_trns_grey_d1_masked(),a_trns_grey_d1_masked(),0)==Ok(()),
{
    hide(raw_trns_grey_d1_masked);assert(raw_trns_grey_d1_masked().len()==6nat) by (compute);assert(r::count(a_trns_grey_d1_masked())==1nat) by (compute);
    assert(r::pw(a_trns_grey_d1_masked(),0nat)==5nat && r::ph(a_trns_grey_d1_masked(),0nat)==3nat && r::rb(a_trns_grey_d1_masked(),0nat)==1nat && r::base(a_trns_grey_d1_masked(),0nat)==0nat) by (compute);
    assert(raw_trns_grey_d1_masked()[0]==0u8) by (compute);
    assert(r::row_result(raw_trns_grey_d1_masked(),a_trns_grey_d1_masked(),0nat,0nat)==Ok(()));
    assert(raw_trns_grey_d1_masked()[2]==1u8) by (compute);
    assert(r::row_result(raw_trns_grey_d1_masked(),a_trns_grey_d1_masked(),0nat,1nat)==Ok(()));
    assert(raw_trns_grey_d1_masked()[4]==2u8) by (compute);
    assert(r::row_result(raw_trns_grey_d1_masked(),a_trns_grey_d1_masked(),0nat,2nat)==Ok(()));
    r::scan_step(raw_trns_grey_d1_masked(),a_trns_grey_d1_masked(),1nat);
    r::pass_step(raw_trns_grey_d1_masked(),a_trns_grey_d1_masked(),0nat,3nat);
    r::pass_step(raw_trns_grey_d1_masked(),a_trns_grey_d1_masked(),0nat,2nat);
    r::pass_step(raw_trns_grey_d1_masked(),a_trns_grey_d1_masked(),0nat,1nat);
    r::pass_step(raw_trns_grey_d1_masked(),a_trns_grey_d1_masked(),0nat,0nat);
    r::scan_step(raw_trns_grey_d1_masked(),a_trns_grey_d1_masked(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_trns_grey_d1_masked() ensures r::still(bytes_trns_grey_d1_masked(),STRICT,false)==(r::Projection::Ready {width:5u32,height:3u32,bytes:60nat}),
{
    hide(bytes_trns_grey_d1_masked);hide(z_trns_grey_d1_masked);hide(raw_trns_grey_d1_masked);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    assert(bytes_trns_grey_d1_masked().len()==85nat) by (compute);
    assert(crate::model::mh(bytes_trns_grey_d1_masked())==Ok((h_trns_grey_d1_masked(),33nat))) by {reveal(crate::model::mh);assert(crate::model::mh(bytes_trns_grey_d1_masked())==Ok((h_trns_grey_d1_masked(),33nat))) by (compute);}
        assert((1u32 << 1u8)==2u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns_grey_d1_masked(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:41nat,len:2nat,next:47nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns_grey_d1_masked(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:41nat,len:2nat,next:47nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by (compute);
        }
        assert(crate::metadata_model::transparency(2nat,0u8,0nat,false)==crate::metadata_model::Decision::Keep(2nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns_grey_d1_masked(),47nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:55nat,len:14nat,next:73nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns_grey_d1_masked(),47nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:55nat,len:14nat,next:73nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns_grey_d1_masked(),73nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:81nat,len:0nat,next:85nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns_grey_d1_masked(),73nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:81nat,len:0nat,next:85nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_trns_grey_d1_masked().subrange(55,69)==z_trns_grey_d1_masked()) by (compute);
    walk_trns_grey_d1_masked(bytes_trns_grey_d1_masked());
    inflate_trns_grey_d1_masked();scan_trns_grey_d1_masked();
    assert(r::raw(a_trns_grey_d1_masked())==6nat) by (compute);
    assert(r::data(z_trns_grey_d1_masked(),a_trns_grey_d1_masked(),STRICT)==Some(Ok(()))) by {reveal(r::data);};
    r::default_ready(bytes_trns_grey_d1_masked(),h_trns_grey_d1_masked(),z_trns_grey_d1_masked(),0nat,false);
}
fn public_trns_grey_d1_masked() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,5u8,0u8,0u8,0u8,3u8,1u8,0u8,0u8,0u8,0u8,115u8,77u8,248u8,85u8,0u8,0u8,0u8,2u8,116u8,82u8,78u8,83u8,255u8,1u8,146u8,176u8,0u8,220u8,0u8,0u8,0u8,14u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,88u8,192u8,184u8,130u8,105u8,3u8,0u8,5u8,214u8,1u8,252u8,84u8,21u8,181u8,192u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_trns_grey_d1_masked());model_trns_grey_d1_masked();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==5u32 && s.height==3u32 && s.rgba@.len()==60nat,Err(crate::alloc_guard::TryError::OutOfMemory(_))=>true,_=>false});
}
// trns/grey-d16; frozen SHA256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_trns_grey_d16() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,5u8,0u8,0u8,0u8,3u8,16u8,0u8,0u8,0u8,0u8,46u8,205u8,70u8,103u8,0u8,0u8,0u8,2u8,116u8,82u8,78u8,83u8,183u8,234u8,157u8,200u8,254u8,33u8,0u8,0u8,0u8,44u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,33u8,0u8,222u8,255u8,0u8,244u8,148u8,233u8,99u8,183u8,234u8,180u8,143u8,144u8,60u8,1u8,55u8,215u8,38u8,113u8,76u8,77u8,113u8,194u8,179u8,83u8,2u8,121u8,34u8,102u8,231u8,231u8,181u8,192u8,62u8,185u8,66u8,29u8,21u8,16u8,124u8,98u8,205u8,159u8,241u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn h_trns_grey_d16() -> crate::model::Ihdr {crate::model::Ihdr {width:5u32,height:3u32,depth:16u8,color_type:0u8,interlace:0u8}}
spec fn a_trns_grey_d16() -> r::Raster {r::shape(h_trns_grey_d16(),5nat,3nat,0nat,STRICT)}
spec fn z_trns_grey_d16() -> Seq<u8> {seq![120u8,156u8,1u8,33u8,0u8,222u8,255u8,0u8,244u8,148u8,233u8,99u8,183u8,234u8,180u8,143u8,144u8,60u8,1u8,55u8,215u8,38u8,113u8,76u8,77u8,113u8,194u8,179u8,83u8,2u8,121u8,34u8,102u8,231u8,231u8,181u8,192u8,62u8,185u8,66u8,29u8,21u8,16u8,124u8]}
spec fn raw_trns_grey_d16() -> Seq<u8> {seq![0u8,244u8,148u8,233u8,99u8,183u8,234u8,180u8,143u8,144u8,60u8,1u8,55u8,215u8,38u8,113u8,76u8,77u8,113u8,194u8,179u8,83u8,2u8,121u8,34u8,102u8,231u8,231u8,181u8,192u8,62u8,185u8,66u8]}
#[verifier::spinoff_prover]
proof fn shape_trns_grey_d16() ensures z_trns_grey_d16().len()==44nat,
        raw_trns_grey_d16().len()==33nat,
        z_trns_grey_d16()[0]==120u8,
        z_trns_grey_d16()[1]==156u8,
        zm::field_value(z_trns_grey_d16(),16,1)==1nat,
        zm::field_value(z_trns_grey_d16(),17,2)==0nat,
        zm::be_value(z_trns_grey_d16(),40nat,4)==487919740nat,
        z_trns_grey_d16().subrange(7,40)==raw_trns_grey_d16(),
        z_trns_grey_d16()[3] as nat+256*z_trns_grey_d16()[4] as nat==33nat,
        z_trns_grey_d16()[5] as nat+256*z_trns_grey_d16()[6] as nat==u16::MAX as nat-33nat,
        zm::be_value(z_trns_grey_d16(),0,1)==120nat,
        zm::be_value(z_trns_grey_d16(),1,1)==156nat,
{
    assert(z_trns_grey_d16().len()==44nat) by (compute);
    assert(raw_trns_grey_d16().len()==33nat) by (compute);
    assert(z_trns_grey_d16()[0]==120u8) by (compute);
    assert(z_trns_grey_d16()[1]==156u8) by (compute);
    assert(zm::field_value(z_trns_grey_d16(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_trns_grey_d16(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_trns_grey_d16(),40nat,4)==487919740nat) by (compute);
    assert(z_trns_grey_d16().subrange(7,40)==raw_trns_grey_d16()) by (compute);
    assert(z_trns_grey_d16()[3] as nat+256*z_trns_grey_d16()[4] as nat==33nat) by (compute);
    assert(z_trns_grey_d16()[5] as nat+256*z_trns_grey_d16()[6] as nat==u16::MAX as nat-33nat) by (compute);
    assert(zm::be_value(z_trns_grey_d16(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_trns_grey_d16(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_trns_grey_d16() ensures zm::Adler(raw_trns_grey_d16())==487919740nat,
{
    hide(raw_trns_grey_d16);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_trns_grey_d16().len()==33nat) by (compute);
    assert(c::sums(raw_trns_grey_d16(),0,33nat)==(4219nat,72966nat)) by (compute);
    c::sums_correct(raw_trns_grey_d16(),0,33nat);assert(raw_trns_grey_d16().subrange(0,33)=~=raw_trns_grey_d16());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((72966nat%65521)*(u16::MAX as nat+1)+(1+4219nat)%65521==487919740nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_trns_grey_d16() ensures zm::Inflate(z_trns_grey_d16(),33nat,true,false)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_trns_grey_d16(),r2:false}),raw_trns_grey_d16().len()==33nat,
{
    hide(z_trns_grey_d16);hide(raw_trns_grey_d16);hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_trns_grey_d16();checksum_trns_grey_d16();
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_trns_grey_d16(),raw_trns_grey_d16(),33nat,120nat,156nat,487919740nat);
}
#[verifier::spinoff_prover]
proof fn walk_trns_grey_d16(b: Seq<u8>)
    requires b.len()==115,
        crate::model::mh(b)==Ok((h_trns_grey_d16(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:41nat,len:2nat,next:47nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,47nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:55nat,len:44nat,next:103nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,103nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:111nat,len:0nat,next:115nat,crc_ok:true,synthetic:false})),
        b.subrange(55,99)==z_trns_grey_d16(),
    ensures r::default_case(b,h_trns_grey_d16(),z_trns_grey_d16(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by (compute);
        }
        assert(crate::metadata_model::transparency(2nat,0u8,0nat,false)==crate::metadata_model::Decision::Keep(2nat)) by (compute);
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
proof fn scan_trns_grey_d16() ensures r::scan(raw_trns_grey_d16(),a_trns_grey_d16(),0)==Ok(()),
{
    hide(raw_trns_grey_d16);assert(raw_trns_grey_d16().len()==33nat) by (compute);assert(r::count(a_trns_grey_d16())==1nat) by (compute);
    assert(r::pw(a_trns_grey_d16(),0nat)==5nat && r::ph(a_trns_grey_d16(),0nat)==3nat && r::rb(a_trns_grey_d16(),0nat)==10nat && r::base(a_trns_grey_d16(),0nat)==0nat) by (compute);
    assert(raw_trns_grey_d16()[0]==0u8) by (compute);
    assert(r::row_result(raw_trns_grey_d16(),a_trns_grey_d16(),0nat,0nat)==Ok(()));
    assert(raw_trns_grey_d16()[11]==1u8) by (compute);
    assert(r::row_result(raw_trns_grey_d16(),a_trns_grey_d16(),0nat,1nat)==Ok(()));
    assert(raw_trns_grey_d16()[22]==2u8) by (compute);
    assert(r::row_result(raw_trns_grey_d16(),a_trns_grey_d16(),0nat,2nat)==Ok(()));
    r::scan_step(raw_trns_grey_d16(),a_trns_grey_d16(),1nat);
    r::pass_step(raw_trns_grey_d16(),a_trns_grey_d16(),0nat,3nat);
    r::pass_step(raw_trns_grey_d16(),a_trns_grey_d16(),0nat,2nat);
    r::pass_step(raw_trns_grey_d16(),a_trns_grey_d16(),0nat,1nat);
    r::pass_step(raw_trns_grey_d16(),a_trns_grey_d16(),0nat,0nat);
    r::scan_step(raw_trns_grey_d16(),a_trns_grey_d16(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_trns_grey_d16() ensures r::still(bytes_trns_grey_d16(),STRICT,false)==(r::Projection::Ready {width:5u32,height:3u32,bytes:60nat}),
{
    hide(bytes_trns_grey_d16);hide(z_trns_grey_d16);hide(raw_trns_grey_d16);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    assert(bytes_trns_grey_d16().len()==115nat) by (compute);
    assert(crate::model::mh(bytes_trns_grey_d16())==Ok((h_trns_grey_d16(),33nat))) by {reveal(crate::model::mh);assert(crate::model::mh(bytes_trns_grey_d16())==Ok((h_trns_grey_d16(),33nat))) by (compute);}
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns_grey_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:41nat,len:2nat,next:47nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns_grey_d16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:41nat,len:2nat,next:47nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by (compute);
        }
        assert(crate::metadata_model::transparency(2nat,0u8,0nat,false)==crate::metadata_model::Decision::Keep(2nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns_grey_d16(),47nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:55nat,len:44nat,next:103nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns_grey_d16(),47nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:55nat,len:44nat,next:103nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns_grey_d16(),103nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:111nat,len:0nat,next:115nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns_grey_d16(),103nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:111nat,len:0nat,next:115nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_trns_grey_d16().subrange(55,99)==z_trns_grey_d16()) by (compute);
    walk_trns_grey_d16(bytes_trns_grey_d16());
    inflate_trns_grey_d16();scan_trns_grey_d16();
    assert(r::raw(a_trns_grey_d16())==33nat) by (compute);
    assert(r::data(z_trns_grey_d16(),a_trns_grey_d16(),STRICT)==Some(Ok(()))) by {reveal(r::data);};
    r::default_ready(bytes_trns_grey_d16(),h_trns_grey_d16(),z_trns_grey_d16(),0nat,false);
}
fn public_trns_grey_d16() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,5u8,0u8,0u8,0u8,3u8,16u8,0u8,0u8,0u8,0u8,46u8,205u8,70u8,103u8,0u8,0u8,0u8,2u8,116u8,82u8,78u8,83u8,183u8,234u8,157u8,200u8,254u8,33u8,0u8,0u8,0u8,44u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,33u8,0u8,222u8,255u8,0u8,244u8,148u8,233u8,99u8,183u8,234u8,180u8,143u8,144u8,60u8,1u8,55u8,215u8,38u8,113u8,76u8,77u8,113u8,194u8,179u8,83u8,2u8,121u8,34u8,102u8,231u8,231u8,181u8,192u8,62u8,185u8,66u8,29u8,21u8,16u8,124u8,98u8,205u8,159u8,241u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_trns_grey_d16());model_trns_grey_d16();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==5u32 && s.height==3u32 && s.rgba@.len()==60nat,Err(crate::alloc_guard::TryError::OutOfMemory(_))=>true,_=>false});
}
// trns/rgb-d16-low-byte-differs; frozen SHA256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_trns_rgb_d16_low_byte_differs() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,4u8,16u8,2u8,0u8,0u8,0u8,118u8,3u8,213u8,106u8,0u8,0u8,0u8,6u8,116u8,82u8,78u8,83u8,175u8,92u8,86u8,125u8,61u8,197u8,72u8,166u8,83u8,156u8,0u8,0u8,0u8,111u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,100u8,0u8,155u8,255u8,0u8,175u8,93u8,86u8,125u8,61u8,197u8,172u8,112u8,21u8,170u8,184u8,173u8,228u8,241u8,48u8,124u8,144u8,84u8,126u8,162u8,91u8,107u8,60u8,112u8,1u8,210u8,1u8,80u8,236u8,118u8,188u8,221u8,92u8,6u8,145u8,199u8,9u8,155u8,169u8,21u8,162u8,191u8,153u8,111u8,21u8,191u8,186u8,124u8,106u8,2u8,194u8,7u8,41u8,175u8,75u8,10u8,0u8,0u8,0u8,0u8,0u8,0u8,167u8,209u8,181u8,60u8,38u8,75u8,213u8,55u8,155u8,128u8,183u8,33u8,3u8,173u8,210u8,171u8,236u8,96u8,146u8,153u8,149u8,146u8,228u8,199u8,189u8,47u8,72u8,185u8,113u8,233u8,22u8,45u8,133u8,139u8,213u8,150u8,189u8,237u8,93u8,46u8,58u8,55u8,151u8,140u8,241u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn h_trns_rgb_d16_low_byte_differs() -> crate::model::Ihdr {crate::model::Ihdr {width:4u32,height:4u32,depth:16u8,color_type:2u8,interlace:0u8}}
spec fn a_trns_rgb_d16_low_byte_differs() -> r::Raster {r::shape(h_trns_rgb_d16_low_byte_differs(),4nat,4nat,0nat,STRICT)}
spec fn z_trns_rgb_d16_low_byte_differs() -> Seq<u8> {seq![120u8,156u8,1u8,100u8,0u8,155u8,255u8,0u8,175u8,93u8,86u8,125u8,61u8,197u8,172u8,112u8,21u8,170u8,184u8,173u8,228u8,241u8,48u8,124u8,144u8,84u8,126u8,162u8,91u8,107u8,60u8,112u8,1u8,210u8,1u8,80u8,236u8,118u8,188u8,221u8,92u8,6u8,145u8,199u8,9u8,155u8,169u8,21u8,162u8,191u8,153u8,111u8,21u8,191u8,186u8,124u8,106u8,2u8,194u8,7u8,41u8,175u8,75u8,10u8,0u8,0u8,0u8,0u8,0u8,0u8,167u8,209u8,181u8,60u8,38u8,75u8,213u8,55u8,155u8,128u8,183u8,33u8,3u8,173u8,210u8,171u8,236u8,96u8,146u8,153u8,149u8,146u8,228u8,199u8,189u8,47u8,72u8,185u8,113u8,233u8,22u8,45u8,133u8,139u8,213u8,150u8,189u8,237u8,93u8,46u8,58u8]}
spec fn raw_trns_rgb_d16_low_byte_differs() -> Seq<u8> {seq![0u8,175u8,93u8,86u8,125u8,61u8,197u8,172u8,112u8,21u8,170u8,184u8,173u8,228u8,241u8,48u8,124u8,144u8,84u8,126u8,162u8,91u8,107u8,60u8,112u8,1u8,210u8,1u8,80u8,236u8,118u8,188u8,221u8,92u8,6u8,145u8,199u8,9u8,155u8,169u8,21u8,162u8,191u8,153u8,111u8,21u8,191u8,186u8,124u8,106u8,2u8,194u8,7u8,41u8,175u8,75u8,10u8,0u8,0u8,0u8,0u8,0u8,0u8,167u8,209u8,181u8,60u8,38u8,75u8,213u8,55u8,155u8,128u8,183u8,33u8,3u8,173u8,210u8,171u8,236u8,96u8,146u8,153u8,149u8,146u8,228u8,199u8,189u8,47u8,72u8,185u8,113u8,233u8,22u8,45u8,133u8,139u8,213u8,150u8,189u8]}
#[verifier::spinoff_prover]
proof fn shape_trns_rgb_d16_low_byte_differs() ensures z_trns_rgb_d16_low_byte_differs().len()==111nat,
        raw_trns_rgb_d16_low_byte_differs().len()==100nat,
        z_trns_rgb_d16_low_byte_differs()[0]==120u8,
        z_trns_rgb_d16_low_byte_differs()[1]==156u8,
        zm::field_value(z_trns_rgb_d16_low_byte_differs(),16,1)==1nat,
        zm::field_value(z_trns_rgb_d16_low_byte_differs(),17,2)==0nat,
        zm::be_value(z_trns_rgb_d16_low_byte_differs(),107nat,4)==3982306874nat,
        z_trns_rgb_d16_low_byte_differs().subrange(7,107)==raw_trns_rgb_d16_low_byte_differs(),
        z_trns_rgb_d16_low_byte_differs()[3] as nat+256*z_trns_rgb_d16_low_byte_differs()[4] as nat==100nat,
        z_trns_rgb_d16_low_byte_differs()[5] as nat+256*z_trns_rgb_d16_low_byte_differs()[6] as nat==u16::MAX as nat-100nat,
        zm::be_value(z_trns_rgb_d16_low_byte_differs(),0,1)==120nat,
        zm::be_value(z_trns_rgb_d16_low_byte_differs(),1,1)==156nat,
{
    assert(z_trns_rgb_d16_low_byte_differs().len()==111nat) by (compute);
    assert(raw_trns_rgb_d16_low_byte_differs().len()==100nat) by (compute);
    assert(z_trns_rgb_d16_low_byte_differs()[0]==120u8) by (compute);
    assert(z_trns_rgb_d16_low_byte_differs()[1]==156u8) by (compute);
    assert(zm::field_value(z_trns_rgb_d16_low_byte_differs(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_trns_rgb_d16_low_byte_differs(),17,2)==0nat) by (compute);
    assert(zm::be_value(z_trns_rgb_d16_low_byte_differs(),107nat,4)==3982306874nat) by (compute);
    assert(z_trns_rgb_d16_low_byte_differs().subrange(7,107)==raw_trns_rgb_d16_low_byte_differs()) by (compute);
    assert(z_trns_rgb_d16_low_byte_differs()[3] as nat+256*z_trns_rgb_d16_low_byte_differs()[4] as nat==100nat) by (compute);
    assert(z_trns_rgb_d16_low_byte_differs()[5] as nat+256*z_trns_rgb_d16_low_byte_differs()[6] as nat==u16::MAX as nat-100nat) by (compute);
    assert(zm::be_value(z_trns_rgb_d16_low_byte_differs(),0,1)==120nat) by (compute);
    assert(zm::be_value(z_trns_rgb_d16_low_byte_differs(),1,1)==156nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_trns_rgb_d16_low_byte_differs() ensures zm::Adler(raw_trns_rgb_d16_low_byte_differs())==3982306874nat,
{
    hide(raw_trns_rgb_d16_low_byte_differs);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_trns_rgb_d16_low_byte_differs().len()==100nat) by (compute);
    assert(c::sums(raw_trns_rgb_d16_low_byte_differs(),0,100nat)==(11833nat,584933nat)) by (compute);
    c::sums_correct(raw_trns_rgb_d16_low_byte_differs(),0,100nat);assert(raw_trns_rgb_d16_low_byte_differs().subrange(0,100)=~=raw_trns_rgb_d16_low_byte_differs());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((584933nat%65521)*(u16::MAX as nat+1)+(1+11833nat)%65521==3982306874nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_trns_rgb_d16_low_byte_differs() ensures zm::Inflate(z_trns_rgb_d16_low_byte_differs(),100nat,true,false)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_trns_rgb_d16_low_byte_differs(),r2:false}),raw_trns_rgb_d16_low_byte_differs().len()==100nat,
{
    hide(z_trns_rgb_d16_low_byte_differs);hide(raw_trns_rgb_d16_low_byte_differs);hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);
    shape_trns_rgb_d16_low_byte_differs();checksum_trns_rgb_d16_low_byte_differs();
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    stream::stored_stream(z_trns_rgb_d16_low_byte_differs(),raw_trns_rgb_d16_low_byte_differs(),100nat,120nat,156nat,3982306874nat);
}
#[verifier::spinoff_prover]
proof fn crc_trns_rgb_d16_low_byte_differs_crc_51()
    ensures crate::model::crc(bytes_trns_rgb_d16_low_byte_differs().subrange(55,170))==932678897u32,
{
    hide(crate::model::crc_register);hide(crc_cert::fold);
    assert(crate::model::crc_register(bytes_trns_rgb_d16_low_byte_differs().subrange(55,170).take(0))==0xffff_ffffu32) by {
        assert(bytes_trns_rgb_d16_low_byte_differs().subrange(55,170).take(0) =~= Seq::<u8>::empty());
        reveal_with_fuel(crate::model::crc_register,1);
    }
    assert(crc_cert::fold(4294967295u32,bytes_trns_rgb_d16_low_byte_differs().subrange(55,170).subrange(0,64))==10930777u32) by (compute);
    crc_cert::suffix(bytes_trns_rgb_d16_low_byte_differs().subrange(55,170).take(0),bytes_trns_rgb_d16_low_byte_differs().subrange(55,170).subrange(0,64));
    assert(bytes_trns_rgb_d16_low_byte_differs().subrange(55,170).take(0)+bytes_trns_rgb_d16_low_byte_differs().subrange(55,170).subrange(0,64) =~= bytes_trns_rgb_d16_low_byte_differs().subrange(55,170).take(64));
    assert(crate::model::crc_register(bytes_trns_rgb_d16_low_byte_differs().subrange(55,170).take(64))==10930777u32);
    assert(crc_cert::fold(10930777u32,bytes_trns_rgb_d16_low_byte_differs().subrange(55,170).subrange(64,115))==3362288398u32) by (compute);
    crc_cert::suffix(bytes_trns_rgb_d16_low_byte_differs().subrange(55,170).take(64),bytes_trns_rgb_d16_low_byte_differs().subrange(55,170).subrange(64,115));
    assert(bytes_trns_rgb_d16_low_byte_differs().subrange(55,170).take(64)+bytes_trns_rgb_d16_low_byte_differs().subrange(55,170).subrange(64,115) =~= bytes_trns_rgb_d16_low_byte_differs().subrange(55,170).take(115));
    assert(crate::model::crc_register(bytes_trns_rgb_d16_low_byte_differs().subrange(55,170).take(115))==3362288398u32);
    assert(bytes_trns_rgb_d16_low_byte_differs().subrange(55,170).take(115) =~= bytes_trns_rgb_d16_low_byte_differs().subrange(55,170));
    assert((3362288398u32 ^ 0xffff_ffffu32)==932678897u32) by (compute);
}
#[verifier::spinoff_prover]
proof fn walk_trns_rgb_d16_low_byte_differs(b: Seq<u8>)
    requires b.len()==186,
        crate::model::mh(b)==Ok((h_trns_rgb_d16_low_byte_differs(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:41nat,len:6nat,next:51nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,51nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:59nat,len:111nat,next:174nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,174nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:182nat,len:0nat,next:186nat,crc_ok:true,synthetic:false})),
        b.subrange(59,170)==z_trns_rgb_d16_low_byte_differs(),
    ensures r::default_case(b,h_trns_rgb_d16_low_byte_differs(),z_trns_rgb_d16_low_byte_differs(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by (compute);
        }
        assert(crate::metadata_model::transparency(6nat,2u8,0nat,false)==crate::metadata_model::Decision::Keep(6nat)) by (compute);
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
proof fn scan_trns_rgb_d16_low_byte_differs() ensures r::scan(raw_trns_rgb_d16_low_byte_differs(),a_trns_rgb_d16_low_byte_differs(),0)==Ok(()),
{
    hide(raw_trns_rgb_d16_low_byte_differs);assert(raw_trns_rgb_d16_low_byte_differs().len()==100nat) by (compute);assert(r::count(a_trns_rgb_d16_low_byte_differs())==1nat) by (compute);
    assert(r::pw(a_trns_rgb_d16_low_byte_differs(),0nat)==4nat && r::ph(a_trns_rgb_d16_low_byte_differs(),0nat)==4nat && r::rb(a_trns_rgb_d16_low_byte_differs(),0nat)==24nat && r::base(a_trns_rgb_d16_low_byte_differs(),0nat)==0nat) by (compute);
    assert(raw_trns_rgb_d16_low_byte_differs()[0]==0u8) by (compute);
    assert(r::row_result(raw_trns_rgb_d16_low_byte_differs(),a_trns_rgb_d16_low_byte_differs(),0nat,0nat)==Ok(()));
    assert(raw_trns_rgb_d16_low_byte_differs()[25]==1u8) by (compute);
    assert(r::row_result(raw_trns_rgb_d16_low_byte_differs(),a_trns_rgb_d16_low_byte_differs(),0nat,1nat)==Ok(()));
    assert(raw_trns_rgb_d16_low_byte_differs()[50]==2u8) by (compute);
    assert(r::row_result(raw_trns_rgb_d16_low_byte_differs(),a_trns_rgb_d16_low_byte_differs(),0nat,2nat)==Ok(()));
    assert(raw_trns_rgb_d16_low_byte_differs()[75]==3u8) by (compute);
    assert(r::row_result(raw_trns_rgb_d16_low_byte_differs(),a_trns_rgb_d16_low_byte_differs(),0nat,3nat)==Ok(()));
    r::scan_step(raw_trns_rgb_d16_low_byte_differs(),a_trns_rgb_d16_low_byte_differs(),1nat);
    r::pass_step(raw_trns_rgb_d16_low_byte_differs(),a_trns_rgb_d16_low_byte_differs(),0nat,4nat);
    r::pass_step(raw_trns_rgb_d16_low_byte_differs(),a_trns_rgb_d16_low_byte_differs(),0nat,3nat);
    r::pass_step(raw_trns_rgb_d16_low_byte_differs(),a_trns_rgb_d16_low_byte_differs(),0nat,2nat);
    r::pass_step(raw_trns_rgb_d16_low_byte_differs(),a_trns_rgb_d16_low_byte_differs(),0nat,1nat);
    r::pass_step(raw_trns_rgb_d16_low_byte_differs(),a_trns_rgb_d16_low_byte_differs(),0nat,0nat);
    r::scan_step(raw_trns_rgb_d16_low_byte_differs(),a_trns_rgb_d16_low_byte_differs(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_trns_rgb_d16_low_byte_differs() ensures r::still(bytes_trns_rgb_d16_low_byte_differs(),STRICT,false)==(r::Projection::Ready {width:4u32,height:4u32,bytes:64nat}),
{
    hide(bytes_trns_rgb_d16_low_byte_differs);hide(z_trns_rgb_d16_low_byte_differs);hide(raw_trns_rgb_d16_low_byte_differs);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    assert(bytes_trns_rgb_d16_low_byte_differs().len()==186nat) by (compute);
    assert(crate::model::mh(bytes_trns_rgb_d16_low_byte_differs())==Ok((h_trns_rgb_d16_low_byte_differs(),33nat))) by {reveal(crate::model::mh);assert(crate::model::mh(bytes_trns_rgb_d16_low_byte_differs())==Ok((h_trns_rgb_d16_low_byte_differs(),33nat))) by (compute);}
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns_rgb_d16_low_byte_differs(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:41nat,len:6nat,next:51nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns_rgb_d16_low_byte_differs(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:41nat,len:6nat,next:51nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by (compute);
        }
        assert(crate::metadata_model::transparency(6nat,2u8,0nat,false)==crate::metadata_model::Decision::Keep(6nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns_rgb_d16_low_byte_differs(),51nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:59nat,len:111nat,next:174nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::model::frame(bytes_trns_rgb_d16_low_byte_differs(),51nat)==Ok(crate::model::WireChunk {ty:seq![73u8,68u8,65u8,84u8],data:59nat,len:111nat,next:174nat})) by (compute);
            assert(crate::model::be32(bytes_trns_rgb_d16_low_byte_differs(),170)==932678897u32) by (compute);
            crc_trns_rgb_d16_low_byte_differs_crc_51();
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns_rgb_d16_low_byte_differs(),174nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:182nat,len:0nat,next:186nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns_rgb_d16_low_byte_differs(),174nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:182nat,len:0nat,next:186nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_trns_rgb_d16_low_byte_differs().subrange(59,170)==z_trns_rgb_d16_low_byte_differs()) by (compute);
    walk_trns_rgb_d16_low_byte_differs(bytes_trns_rgb_d16_low_byte_differs());
    inflate_trns_rgb_d16_low_byte_differs();scan_trns_rgb_d16_low_byte_differs();
    assert(r::raw(a_trns_rgb_d16_low_byte_differs())==100nat) by (compute);
    assert(r::data(z_trns_rgb_d16_low_byte_differs(),a_trns_rgb_d16_low_byte_differs(),STRICT)==Some(Ok(()))) by {reveal(r::data);};
    r::default_ready(bytes_trns_rgb_d16_low_byte_differs(),h_trns_rgb_d16_low_byte_differs(),z_trns_rgb_d16_low_byte_differs(),0nat,false);
}
fn public_trns_rgb_d16_low_byte_differs() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,4u8,16u8,2u8,0u8,0u8,0u8,118u8,3u8,213u8,106u8,0u8,0u8,0u8,6u8,116u8,82u8,78u8,83u8,175u8,92u8,86u8,125u8,61u8,197u8,72u8,166u8,83u8,156u8,0u8,0u8,0u8,111u8,73u8,68u8,65u8,84u8,120u8,156u8,1u8,100u8,0u8,155u8,255u8,0u8,175u8,93u8,86u8,125u8,61u8,197u8,172u8,112u8,21u8,170u8,184u8,173u8,228u8,241u8,48u8,124u8,144u8,84u8,126u8,162u8,91u8,107u8,60u8,112u8,1u8,210u8,1u8,80u8,236u8,118u8,188u8,221u8,92u8,6u8,145u8,199u8,9u8,155u8,169u8,21u8,162u8,191u8,153u8,111u8,21u8,191u8,186u8,124u8,106u8,2u8,194u8,7u8,41u8,175u8,75u8,10u8,0u8,0u8,0u8,0u8,0u8,0u8,167u8,209u8,181u8,60u8,38u8,75u8,213u8,55u8,155u8,128u8,183u8,33u8,3u8,173u8,210u8,171u8,236u8,96u8,146u8,153u8,149u8,146u8,228u8,199u8,189u8,47u8,72u8,185u8,113u8,233u8,22u8,45u8,133u8,139u8,213u8,150u8,189u8,237u8,93u8,46u8,58u8,55u8,151u8,140u8,241u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_trns_rgb_d16_low_byte_differs());model_trns_rgb_d16_low_byte_differs();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==4u32 && s.height==4u32 && s.rgba@.len()==64nat,Err(crate::alloc_guard::TryError::OutOfMemory(_))=>true,_=>false});
}
// trns16/palette-d8-16-3; frozen SHA256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_trns16_palette_d8_16_3() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,3u8,0u8,0u8,0u8,131u8,42u8,94u8,244u8,0u8,0u8,0u8,48u8,80u8,76u8,84u8,69u8,239u8,230u8,122u8,43u8,7u8,60u8,193u8,85u8,112u8,70u8,58u8,95u8,11u8,10u8,81u8,103u8,32u8,151u8,53u8,236u8,68u8,4u8,168u8,18u8,245u8,59u8,116u8,21u8,182u8,204u8,151u8,3u8,52u8,43u8,132u8,160u8,121u8,173u8,140u8,215u8,88u8,73u8,229u8,242u8,227u8,125u8,35u8,7u8,26u8,211u8,19u8,51u8,0u8,0u8,0u8,3u8,116u8,82u8,78u8,83u8,128u8,94u8,218u8,89u8,125u8,148u8,78u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,224u8,97u8,97u8,100u8,101u8,228u8,96u8,254u8,243u8,147u8,137u8,129u8,133u8,133u8,19u8,0u8,14u8,121u8,2u8,43u8,233u8,40u8,179u8,53u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn h_trns16_palette_d8_16_3() -> crate::model::Ihdr {crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:3u8,interlace:0u8}}
spec fn a_trns16_palette_d8_16_3() -> r::Raster {r::shape(h_trns16_palette_d8_16_3(),4nat,3nat,16nat,STRICT)}
spec fn z_trns16_palette_d8_16_3() -> Seq<u8> {seq![120u8,156u8,99u8,224u8,97u8,97u8,100u8,101u8,228u8,96u8,254u8,243u8,147u8,137u8,129u8,133u8,133u8,19u8,0u8,14u8,121u8,2u8,43u8]}
spec fn raw_trns16_palette_d8_16_3() -> Seq<u8> {seq![0u8,12u8,4u8,1u8,5u8,1u8,8u8,3u8,252u8,249u8,2u8,0u8,4u8,4u8,9u8]}
spec fn ts_trns16_palette_d8_16_3() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:35nat,before:1nat,after:2nat,symbol:12nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:35nat,next:43nat,before:2nat,after:3nat,symbol:4nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:43nat,next:51nat,before:3nat,after:4nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:51nat,next:59nat,before:4nat,after:5nat,symbol:5nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:59nat,next:67nat,before:5nat,after:6nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:67nat,next:75nat,before:6nat,after:7nat,symbol:8nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:75nat,next:83nat,before:7nat,after:8nat,symbol:3nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:83nat,next:92nat,before:8nat,after:9nat,symbol:252nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:92nat,next:101nat,before:9nat,after:10nat,symbol:249nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:101nat,next:109nat,before:10nat,after:11nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:109nat,next:117nat,before:11nat,after:12nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:117nat,next:125nat,before:12nat,after:13nat,symbol:4nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:125nat,next:133nat,before:13nat,after:14nat,symbol:4nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:133nat,next:141nat,before:14nat,after:15nat,symbol:9nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:141nat,next:148nat,before:15nat,after:15nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_trns16_palette_d8_16_3_0() ensures t::trace(z_trns16_palette_d8_16_3(),raw_trns16_palette_d8_16_3(),ts_trns16_palette_d8_16_3(),0nat,16nat),
{
    hide(t::valid_step);assert(t::trace(z_trns16_palette_d8_16_3(),raw_trns16_palette_d8_16_3(),ts_trns16_palette_d8_16_3(),0nat,16nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn trace_trns16_palette_d8_16_3() ensures t::trace(z_trns16_palette_d8_16_3(),raw_trns16_palette_d8_16_3(),ts_trns16_palette_d8_16_3(),0,16nat),
{
    hide(z_trns16_palette_d8_16_3);hide(raw_trns16_palette_d8_16_3);hide(ts_trns16_palette_d8_16_3);hide(t::trace);
    trace_trns16_palette_d8_16_3_0();
    assert(t::trace(z_trns16_palette_d8_16_3(),raw_trns16_palette_d8_16_3(),ts_trns16_palette_d8_16_3(),0,16nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_trns16_palette_d8_16_3() ensures z_trns16_palette_d8_16_3().len()==23nat,
        raw_trns16_palette_d8_16_3().len()==15nat,
        z_trns16_palette_d8_16_3()[0]==120u8,
        z_trns16_palette_d8_16_3()[1]==156u8,
        zm::field_value(z_trns16_palette_d8_16_3(),16,1)==1nat,
        zm::field_value(z_trns16_palette_d8_16_3(),17,2)==1nat,
        zm::be_value(z_trns16_palette_d8_16_3(),19nat,4)==242811435nat,
        ts_trns16_palette_d8_16_3().len()==16nat,
        ts_trns16_palette_d8_16_3()[0].pos==19nat,
        ts_trns16_palette_d8_16_3()[0].before==0nat,
        ts_trns16_palette_d8_16_3().last().next==148nat,
{
    assert(z_trns16_palette_d8_16_3().len()==23nat) by (compute);
    assert(raw_trns16_palette_d8_16_3().len()==15nat) by (compute);
    assert(z_trns16_palette_d8_16_3()[0]==120u8) by (compute);
    assert(z_trns16_palette_d8_16_3()[1]==156u8) by (compute);
    assert(zm::field_value(z_trns16_palette_d8_16_3(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_trns16_palette_d8_16_3(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_trns16_palette_d8_16_3(),19nat,4)==242811435nat) by (compute);
    assert(ts_trns16_palette_d8_16_3().len()==16nat) by (compute);
    assert(ts_trns16_palette_d8_16_3()[0].pos==19nat) by (compute);
    assert(ts_trns16_palette_d8_16_3()[0].before==0nat) by (compute);
    assert(ts_trns16_palette_d8_16_3().last().next==148nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_trns16_palette_d8_16_3() ensures zm::Adler(raw_trns16_palette_d8_16_3())==242811435nat,
{
    hide(raw_trns16_palette_d8_16_3);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_trns16_palette_d8_16_3().len()==15nat) by (compute);
    assert(c::sums(raw_trns16_palette_d8_16_3(),0,15nat)==(554nat,3705nat)) by (compute);
    c::sums_correct(raw_trns16_palette_d8_16_3(),0,15nat);assert(raw_trns16_palette_d8_16_3().subrange(0,15)=~=raw_trns16_palette_d8_16_3());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((3705nat%65521)*(u16::MAX as nat+1)+(1+554nat)%65521==242811435nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_trns16_palette_d8_16_3() ensures zm::Inflate(z_trns16_palette_d8_16_3(),15nat,true,false)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_trns16_palette_d8_16_3(),r2:false}),raw_trns16_palette_d8_16_3().len()==15nat,
{
    hide(z_trns16_palette_d8_16_3);hide(raw_trns16_palette_d8_16_3);hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);hide(ts_trns16_palette_d8_16_3);
    shape_trns16_palette_d8_16_3();checksum_trns16_palette_d8_16_3();
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_trns16_palette_d8_16_3();ms::fixed_stream(z_trns16_palette_d8_16_3(),raw_trns16_palette_d8_16_3(),ts_trns16_palette_d8_16_3(),15nat,156nat,242811435nat,242811435nat);
}
#[verifier::spinoff_prover]
proof fn walk_trns16_palette_d8_16_3(b: Seq<u8>)
    requires b.len()==155,
        crate::model::mh(b)==Ok((h_trns16_palette_d8_16_3(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:101nat,len:3nat,next:108nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,108nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:116nat,len:23nat,next:143nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,143nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:151nat,len:0nat,next:155nat,crc_ok:true,synthetic:false})),
        b.subrange(116,139)==z_trns16_palette_d8_16_3(),
    ensures r::default_case(b,h_trns16_palette_d8_16_3(),z_trns16_palette_d8_16_3(),16nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(48nat,3u8,8u8,false,false,false,false)==crate::metadata_model::Decision::Keep(16nat)) by (compute);
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by (compute);
        }
        assert(crate::metadata_model::transparency(3nat,3u8,16nat,false)==crate::metadata_model::Decision::Keep(3nat)) by (compute);
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
proof fn scan_trns16_palette_d8_16_3() ensures r::scan(raw_trns16_palette_d8_16_3(),a_trns16_palette_d8_16_3(),0)==Ok(()),
{
    hide(raw_trns16_palette_d8_16_3);assert(raw_trns16_palette_d8_16_3().len()==15nat) by (compute);assert(r::count(a_trns16_palette_d8_16_3())==1nat) by (compute);
    assert(r::pw(a_trns16_palette_d8_16_3(),0nat)==4nat && r::ph(a_trns16_palette_d8_16_3(),0nat)==3nat && r::rb(a_trns16_palette_d8_16_3(),0nat)==4nat && r::base(a_trns16_palette_d8_16_3(),0nat)==0nat) by (compute);
    assert(raw_trns16_palette_d8_16_3()[0]==0u8) by (compute);
    f::pass_row_equation(raw_trns16_palette_d8_16_3(),0nat,4nat,1nat,0nat);
    assert(f::previous_row(raw_trns16_palette_d8_16_3(),0nat,4nat,1nat,0nat)==None);
    assert(f::encoded_row(0u8,f::row_data(raw_trns16_palette_d8_16_3(),0nat,4nat,0nat),None,1nat,seq![12u8,4u8,1u8,5u8],0,4nat)) by (compute);
    f::checked_row_hint(0u8,f::row_data(raw_trns16_palette_d8_16_3(),0nat,4nat,0nat),None,1nat,seq![12u8,4u8,1u8,5u8]);
    assert(r::line(raw_trns16_palette_d8_16_3(),a_trns16_palette_d8_16_3(),0nat,0nat)==seq![12u8,4u8,1u8,5u8]);
    assert forall|x: nat| x<4 implies (#[trigger] r::palette_sample(r::line(raw_trns16_palette_d8_16_3(),a_trns16_palette_d8_16_3(),0nat,0nat),8u8,x) as nat)<16 by {
        if x==0 {assert((r::palette_sample(seq![12u8,4u8,1u8,5u8],8u8,0nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d8_16_3(),a_trns16_palette_d8_16_3(),0nat,0nat),8u8,0nat) as nat)<16nat);}
        else if x==1 {assert((r::palette_sample(seq![12u8,4u8,1u8,5u8],8u8,1nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d8_16_3(),a_trns16_palette_d8_16_3(),0nat,0nat),8u8,1nat) as nat)<16nat);}
        else if x==2 {assert((r::palette_sample(seq![12u8,4u8,1u8,5u8],8u8,2nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d8_16_3(),a_trns16_palette_d8_16_3(),0nat,0nat),8u8,2nat) as nat)<16nat);}
        else if x==3 {assert((r::palette_sample(seq![12u8,4u8,1u8,5u8],8u8,3nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d8_16_3(),a_trns16_palette_d8_16_3(),0nat,0nat),8u8,3nat) as nat)<16nat);}
    }
    assert(r::palette_valid(r::line(raw_trns16_palette_d8_16_3(),a_trns16_palette_d8_16_3(),0nat,0nat),8u8,4nat,16nat));
    assert(r::row_result(raw_trns16_palette_d8_16_3(),a_trns16_palette_d8_16_3(),0nat,0nat)==Ok(()));
    assert(raw_trns16_palette_d8_16_3()[5]==1u8) by (compute);
    f::pass_row_equation(raw_trns16_palette_d8_16_3(),0nat,4nat,1nat,1nat);
    assert(f::previous_row(raw_trns16_palette_d8_16_3(),0nat,4nat,1nat,1nat)==Some(seq![12u8,4u8,1u8,5u8]));
    assert(f::encoded_row(1u8,f::row_data(raw_trns16_palette_d8_16_3(),0nat,4nat,1nat),Some(seq![12u8,4u8,1u8,5u8]),1nat,seq![8u8,11u8,7u8,0u8],0,4nat)) by (compute);
    f::checked_row_hint(1u8,f::row_data(raw_trns16_palette_d8_16_3(),0nat,4nat,1nat),Some(seq![12u8,4u8,1u8,5u8]),1nat,seq![8u8,11u8,7u8,0u8]);
    assert(r::line(raw_trns16_palette_d8_16_3(),a_trns16_palette_d8_16_3(),0nat,1nat)==seq![8u8,11u8,7u8,0u8]);
    assert forall|x: nat| x<4 implies (#[trigger] r::palette_sample(r::line(raw_trns16_palette_d8_16_3(),a_trns16_palette_d8_16_3(),0nat,1nat),8u8,x) as nat)<16 by {
        if x==0 {assert((r::palette_sample(seq![8u8,11u8,7u8,0u8],8u8,0nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d8_16_3(),a_trns16_palette_d8_16_3(),0nat,1nat),8u8,0nat) as nat)<16nat);}
        else if x==1 {assert((r::palette_sample(seq![8u8,11u8,7u8,0u8],8u8,1nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d8_16_3(),a_trns16_palette_d8_16_3(),0nat,1nat),8u8,1nat) as nat)<16nat);}
        else if x==2 {assert((r::palette_sample(seq![8u8,11u8,7u8,0u8],8u8,2nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d8_16_3(),a_trns16_palette_d8_16_3(),0nat,1nat),8u8,2nat) as nat)<16nat);}
        else if x==3 {assert((r::palette_sample(seq![8u8,11u8,7u8,0u8],8u8,3nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d8_16_3(),a_trns16_palette_d8_16_3(),0nat,1nat),8u8,3nat) as nat)<16nat);}
    }
    assert(r::palette_valid(r::line(raw_trns16_palette_d8_16_3(),a_trns16_palette_d8_16_3(),0nat,1nat),8u8,4nat,16nat));
    assert(r::row_result(raw_trns16_palette_d8_16_3(),a_trns16_palette_d8_16_3(),0nat,1nat)==Ok(()));
    assert(raw_trns16_palette_d8_16_3()[10]==2u8) by (compute);
    f::pass_row_equation(raw_trns16_palette_d8_16_3(),0nat,4nat,1nat,2nat);
    assert(f::previous_row(raw_trns16_palette_d8_16_3(),0nat,4nat,1nat,2nat)==Some(seq![8u8,11u8,7u8,0u8]));
    assert(f::encoded_row(2u8,f::row_data(raw_trns16_palette_d8_16_3(),0nat,4nat,2nat),Some(seq![8u8,11u8,7u8,0u8]),1nat,seq![8u8,15u8,11u8,9u8],0,4nat)) by (compute);
    f::checked_row_hint(2u8,f::row_data(raw_trns16_palette_d8_16_3(),0nat,4nat,2nat),Some(seq![8u8,11u8,7u8,0u8]),1nat,seq![8u8,15u8,11u8,9u8]);
    assert(r::line(raw_trns16_palette_d8_16_3(),a_trns16_palette_d8_16_3(),0nat,2nat)==seq![8u8,15u8,11u8,9u8]);
    assert forall|x: nat| x<4 implies (#[trigger] r::palette_sample(r::line(raw_trns16_palette_d8_16_3(),a_trns16_palette_d8_16_3(),0nat,2nat),8u8,x) as nat)<16 by {
        if x==0 {assert((r::palette_sample(seq![8u8,15u8,11u8,9u8],8u8,0nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d8_16_3(),a_trns16_palette_d8_16_3(),0nat,2nat),8u8,0nat) as nat)<16nat);}
        else if x==1 {assert((r::palette_sample(seq![8u8,15u8,11u8,9u8],8u8,1nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d8_16_3(),a_trns16_palette_d8_16_3(),0nat,2nat),8u8,1nat) as nat)<16nat);}
        else if x==2 {assert((r::palette_sample(seq![8u8,15u8,11u8,9u8],8u8,2nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d8_16_3(),a_trns16_palette_d8_16_3(),0nat,2nat),8u8,2nat) as nat)<16nat);}
        else if x==3 {assert((r::palette_sample(seq![8u8,15u8,11u8,9u8],8u8,3nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d8_16_3(),a_trns16_palette_d8_16_3(),0nat,2nat),8u8,3nat) as nat)<16nat);}
    }
    assert(r::palette_valid(r::line(raw_trns16_palette_d8_16_3(),a_trns16_palette_d8_16_3(),0nat,2nat),8u8,4nat,16nat));
    assert(r::row_result(raw_trns16_palette_d8_16_3(),a_trns16_palette_d8_16_3(),0nat,2nat)==Ok(()));
    r::scan_step(raw_trns16_palette_d8_16_3(),a_trns16_palette_d8_16_3(),1nat);
    r::pass_step(raw_trns16_palette_d8_16_3(),a_trns16_palette_d8_16_3(),0nat,3nat);
    r::pass_step(raw_trns16_palette_d8_16_3(),a_trns16_palette_d8_16_3(),0nat,2nat);
    r::pass_step(raw_trns16_palette_d8_16_3(),a_trns16_palette_d8_16_3(),0nat,1nat);
    r::pass_step(raw_trns16_palette_d8_16_3(),a_trns16_palette_d8_16_3(),0nat,0nat);
    r::scan_step(raw_trns16_palette_d8_16_3(),a_trns16_palette_d8_16_3(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_trns16_palette_d8_16_3() ensures r::still(bytes_trns16_palette_d8_16_3(),STRICT,true)==(r::Projection::Ready {width:4u32,height:3u32,bytes:96nat}),
{
    hide(bytes_trns16_palette_d8_16_3);hide(z_trns16_palette_d8_16_3);hide(raw_trns16_palette_d8_16_3);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    assert(bytes_trns16_palette_d8_16_3().len()==155nat) by (compute);
    assert(crate::model::mh(bytes_trns16_palette_d8_16_3())==Ok((h_trns16_palette_d8_16_3(),33nat))) by {reveal(crate::model::mh);assert(crate::model::mh(bytes_trns16_palette_d8_16_3())==Ok((h_trns16_palette_d8_16_3(),33nat))) by (compute);}
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns16_palette_d8_16_3(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns16_palette_d8_16_3(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(48nat,3u8,8u8,false,false,false,false)==crate::metadata_model::Decision::Keep(16nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns16_palette_d8_16_3(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:101nat,len:3nat,next:108nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns16_palette_d8_16_3(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:101nat,len:3nat,next:108nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by (compute);
        }
        assert(crate::metadata_model::transparency(3nat,3u8,16nat,false)==crate::metadata_model::Decision::Keep(3nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns16_palette_d8_16_3(),108nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:116nat,len:23nat,next:143nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns16_palette_d8_16_3(),108nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:116nat,len:23nat,next:143nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns16_palette_d8_16_3(),143nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:151nat,len:0nat,next:155nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns16_palette_d8_16_3(),143nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:151nat,len:0nat,next:155nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_trns16_palette_d8_16_3().subrange(116,139)==z_trns16_palette_d8_16_3()) by (compute);
    walk_trns16_palette_d8_16_3(bytes_trns16_palette_d8_16_3());
    inflate_trns16_palette_d8_16_3();scan_trns16_palette_d8_16_3();
    assert(r::raw(a_trns16_palette_d8_16_3())==15nat) by (compute);
    assert(r::data(z_trns16_palette_d8_16_3(),a_trns16_palette_d8_16_3(),STRICT)==Some(Ok(()))) by {reveal(r::data);};
    r::default_ready(bytes_trns16_palette_d8_16_3(),h_trns16_palette_d8_16_3(),z_trns16_palette_d8_16_3(),16nat,true);
}
fn public_trns16_palette_d8_16_3() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,3u8,0u8,0u8,0u8,131u8,42u8,94u8,244u8,0u8,0u8,0u8,48u8,80u8,76u8,84u8,69u8,239u8,230u8,122u8,43u8,7u8,60u8,193u8,85u8,112u8,70u8,58u8,95u8,11u8,10u8,81u8,103u8,32u8,151u8,53u8,236u8,68u8,4u8,168u8,18u8,245u8,59u8,116u8,21u8,182u8,204u8,151u8,3u8,52u8,43u8,132u8,160u8,121u8,173u8,140u8,215u8,88u8,73u8,229u8,242u8,227u8,125u8,35u8,7u8,26u8,211u8,19u8,51u8,0u8,0u8,0u8,3u8,116u8,82u8,78u8,83u8,128u8,94u8,218u8,89u8,125u8,148u8,78u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,224u8,97u8,97u8,100u8,101u8,228u8,96u8,254u8,243u8,147u8,137u8,129u8,133u8,133u8,19u8,0u8,14u8,121u8,2u8,43u8,233u8,40u8,179u8,53u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_trns16_palette_d8_16_3());model_trns16_palette_d8_16_3();}
    let result=crate::png::try_decode_still16(&b);
    assert(match result {Ok(s)=>s.width==4u32 && s.height==3u32 && s.rgba16@.len()==96nat,Err(crate::alloc_guard::TryError::OutOfMemory(_))=>true,_=>false});
}
// trns16/palette-d8-16-16; frozen SHA256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_trns16_palette_d8_16_16() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,3u8,0u8,0u8,0u8,131u8,42u8,94u8,244u8,0u8,0u8,0u8,48u8,80u8,76u8,84u8,69u8,30u8,63u8,230u8,29u8,184u8,54u8,139u8,90u8,2u8,215u8,196u8,213u8,58u8,106u8,191u8,63u8,17u8,236u8,126u8,26u8,138u8,109u8,172u8,29u8,237u8,144u8,208u8,93u8,107u8,245u8,85u8,120u8,102u8,69u8,5u8,92u8,35u8,4u8,21u8,247u8,45u8,173u8,86u8,224u8,175u8,43u8,140u8,151u8,27u8,30u8,182u8,100u8,0u8,0u8,0u8,16u8,116u8,82u8,78u8,83u8,58u8,60u8,187u8,47u8,80u8,86u8,201u8,127u8,204u8,205u8,43u8,214u8,31u8,120u8,159u8,125u8,7u8,107u8,148u8,20u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,224u8,102u8,227u8,102u8,99u8,228u8,98u8,249u8,247u8,155u8,233u8,239u8,183u8,47u8,255u8,0u8,24u8,219u8,6u8,18u8,62u8,200u8,197u8,78u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn h_trns16_palette_d8_16_16() -> crate::model::Ihdr {crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:3u8,interlace:0u8}}
spec fn a_trns16_palette_d8_16_16() -> r::Raster {r::shape(h_trns16_palette_d8_16_16(),4nat,3nat,16nat,STRICT)}
spec fn z_trns16_palette_d8_16_16() -> Seq<u8> {seq![120u8,156u8,99u8,224u8,102u8,227u8,102u8,99u8,228u8,98u8,249u8,247u8,155u8,233u8,239u8,183u8,47u8,255u8,0u8,24u8,219u8,6u8,18u8]}
spec fn raw_trns16_palette_d8_16_16() -> Seq<u8> {seq![0u8,11u8,6u8,11u8,6u8,1u8,10u8,4u8,254u8,251u8,2u8,253u8,246u8,244u8,254u8]}
spec fn ts_trns16_palette_d8_16_16() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:35nat,before:1nat,after:2nat,symbol:11nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:35nat,next:43nat,before:2nat,after:3nat,symbol:6nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:43nat,next:51nat,before:3nat,after:4nat,symbol:11nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:51nat,next:59nat,before:4nat,after:5nat,symbol:6nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:59nat,next:67nat,before:5nat,after:6nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:67nat,next:75nat,before:6nat,after:7nat,symbol:10nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:75nat,next:83nat,before:7nat,after:8nat,symbol:4nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:83nat,next:92nat,before:8nat,after:9nat,symbol:254nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:92nat,next:101nat,before:9nat,after:10nat,symbol:251nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:101nat,next:109nat,before:10nat,after:11nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:109nat,next:118nat,before:11nat,after:12nat,symbol:253nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:118nat,next:127nat,before:12nat,after:13nat,symbol:246nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:127nat,next:136nat,before:13nat,after:14nat,symbol:244nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:136nat,next:145nat,before:14nat,after:15nat,symbol:254nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:145nat,next:152nat,before:15nat,after:15nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_trns16_palette_d8_16_16_0() ensures t::trace(z_trns16_palette_d8_16_16(),raw_trns16_palette_d8_16_16(),ts_trns16_palette_d8_16_16(),0nat,16nat),
{
    hide(t::valid_step);assert(t::trace(z_trns16_palette_d8_16_16(),raw_trns16_palette_d8_16_16(),ts_trns16_palette_d8_16_16(),0nat,16nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn trace_trns16_palette_d8_16_16() ensures t::trace(z_trns16_palette_d8_16_16(),raw_trns16_palette_d8_16_16(),ts_trns16_palette_d8_16_16(),0,16nat),
{
    hide(z_trns16_palette_d8_16_16);hide(raw_trns16_palette_d8_16_16);hide(ts_trns16_palette_d8_16_16);hide(t::trace);
    trace_trns16_palette_d8_16_16_0();
    assert(t::trace(z_trns16_palette_d8_16_16(),raw_trns16_palette_d8_16_16(),ts_trns16_palette_d8_16_16(),0,16nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_trns16_palette_d8_16_16() ensures z_trns16_palette_d8_16_16().len()==23nat,
        raw_trns16_palette_d8_16_16().len()==15nat,
        z_trns16_palette_d8_16_16()[0]==120u8,
        z_trns16_palette_d8_16_16()[1]==156u8,
        zm::field_value(z_trns16_palette_d8_16_16(),16,1)==1nat,
        zm::field_value(z_trns16_palette_d8_16_16(),17,2)==1nat,
        zm::be_value(z_trns16_palette_d8_16_16(),19nat,4)==417007122nat,
        ts_trns16_palette_d8_16_16().len()==16nat,
        ts_trns16_palette_d8_16_16()[0].pos==19nat,
        ts_trns16_palette_d8_16_16()[0].before==0nat,
        ts_trns16_palette_d8_16_16().last().next==152nat,
{
    assert(z_trns16_palette_d8_16_16().len()==23nat) by (compute);
    assert(raw_trns16_palette_d8_16_16().len()==15nat) by (compute);
    assert(z_trns16_palette_d8_16_16()[0]==120u8) by (compute);
    assert(z_trns16_palette_d8_16_16()[1]==156u8) by (compute);
    assert(zm::field_value(z_trns16_palette_d8_16_16(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_trns16_palette_d8_16_16(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_trns16_palette_d8_16_16(),19nat,4)==417007122nat) by (compute);
    assert(ts_trns16_palette_d8_16_16().len()==16nat) by (compute);
    assert(ts_trns16_palette_d8_16_16()[0].pos==19nat) by (compute);
    assert(ts_trns16_palette_d8_16_16()[0].before==0nat) by (compute);
    assert(ts_trns16_palette_d8_16_16().last().next==152nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_trns16_palette_d8_16_16() ensures zm::Adler(raw_trns16_palette_d8_16_16())==417007122nat,
{
    hide(raw_trns16_palette_d8_16_16);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_trns16_palette_d8_16_16().len()==15nat) by (compute);
    assert(c::sums(raw_trns16_palette_d8_16_16(),0,15nat)==(1553nat,6363nat)) by (compute);
    c::sums_correct(raw_trns16_palette_d8_16_16(),0,15nat);assert(raw_trns16_palette_d8_16_16().subrange(0,15)=~=raw_trns16_palette_d8_16_16());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((6363nat%65521)*(u16::MAX as nat+1)+(1+1553nat)%65521==417007122nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_trns16_palette_d8_16_16() ensures zm::Inflate(z_trns16_palette_d8_16_16(),15nat,true,false)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_trns16_palette_d8_16_16(),r2:false}),raw_trns16_palette_d8_16_16().len()==15nat,
{
    hide(z_trns16_palette_d8_16_16);hide(raw_trns16_palette_d8_16_16);hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);hide(ts_trns16_palette_d8_16_16);
    shape_trns16_palette_d8_16_16();checksum_trns16_palette_d8_16_16();
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_trns16_palette_d8_16_16();ms::fixed_stream(z_trns16_palette_d8_16_16(),raw_trns16_palette_d8_16_16(),ts_trns16_palette_d8_16_16(),15nat,156nat,417007122nat,417007122nat);
}
#[verifier::spinoff_prover]
proof fn walk_trns16_palette_d8_16_16(b: Seq<u8>)
    requires b.len()==168,
        crate::model::mh(b)==Ok((h_trns16_palette_d8_16_16(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:101nat,len:16nat,next:121nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,121nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:129nat,len:23nat,next:156nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,156nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:164nat,len:0nat,next:168nat,crc_ok:true,synthetic:false})),
        b.subrange(129,152)==z_trns16_palette_d8_16_16(),
    ensures r::default_case(b,h_trns16_palette_d8_16_16(),z_trns16_palette_d8_16_16(),16nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(48nat,3u8,8u8,false,false,false,false)==crate::metadata_model::Decision::Keep(16nat)) by (compute);
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by (compute);
        }
        assert(crate::metadata_model::transparency(16nat,3u8,16nat,false)==crate::metadata_model::Decision::Keep(16nat)) by (compute);
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
proof fn scan_trns16_palette_d8_16_16() ensures r::scan(raw_trns16_palette_d8_16_16(),a_trns16_palette_d8_16_16(),0)==Ok(()),
{
    hide(raw_trns16_palette_d8_16_16);assert(raw_trns16_palette_d8_16_16().len()==15nat) by (compute);assert(r::count(a_trns16_palette_d8_16_16())==1nat) by (compute);
    assert(r::pw(a_trns16_palette_d8_16_16(),0nat)==4nat && r::ph(a_trns16_palette_d8_16_16(),0nat)==3nat && r::rb(a_trns16_palette_d8_16_16(),0nat)==4nat && r::base(a_trns16_palette_d8_16_16(),0nat)==0nat) by (compute);
    assert(raw_trns16_palette_d8_16_16()[0]==0u8) by (compute);
    f::pass_row_equation(raw_trns16_palette_d8_16_16(),0nat,4nat,1nat,0nat);
    assert(f::previous_row(raw_trns16_palette_d8_16_16(),0nat,4nat,1nat,0nat)==None);
    assert(f::encoded_row(0u8,f::row_data(raw_trns16_palette_d8_16_16(),0nat,4nat,0nat),None,1nat,seq![11u8,6u8,11u8,6u8],0,4nat)) by (compute);
    f::checked_row_hint(0u8,f::row_data(raw_trns16_palette_d8_16_16(),0nat,4nat,0nat),None,1nat,seq![11u8,6u8,11u8,6u8]);
    assert(r::line(raw_trns16_palette_d8_16_16(),a_trns16_palette_d8_16_16(),0nat,0nat)==seq![11u8,6u8,11u8,6u8]);
    assert forall|x: nat| x<4 implies (#[trigger] r::palette_sample(r::line(raw_trns16_palette_d8_16_16(),a_trns16_palette_d8_16_16(),0nat,0nat),8u8,x) as nat)<16 by {
        if x==0 {assert((r::palette_sample(seq![11u8,6u8,11u8,6u8],8u8,0nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d8_16_16(),a_trns16_palette_d8_16_16(),0nat,0nat),8u8,0nat) as nat)<16nat);}
        else if x==1 {assert((r::palette_sample(seq![11u8,6u8,11u8,6u8],8u8,1nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d8_16_16(),a_trns16_palette_d8_16_16(),0nat,0nat),8u8,1nat) as nat)<16nat);}
        else if x==2 {assert((r::palette_sample(seq![11u8,6u8,11u8,6u8],8u8,2nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d8_16_16(),a_trns16_palette_d8_16_16(),0nat,0nat),8u8,2nat) as nat)<16nat);}
        else if x==3 {assert((r::palette_sample(seq![11u8,6u8,11u8,6u8],8u8,3nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d8_16_16(),a_trns16_palette_d8_16_16(),0nat,0nat),8u8,3nat) as nat)<16nat);}
    }
    assert(r::palette_valid(r::line(raw_trns16_palette_d8_16_16(),a_trns16_palette_d8_16_16(),0nat,0nat),8u8,4nat,16nat));
    assert(r::row_result(raw_trns16_palette_d8_16_16(),a_trns16_palette_d8_16_16(),0nat,0nat)==Ok(()));
    assert(raw_trns16_palette_d8_16_16()[5]==1u8) by (compute);
    f::pass_row_equation(raw_trns16_palette_d8_16_16(),0nat,4nat,1nat,1nat);
    assert(f::previous_row(raw_trns16_palette_d8_16_16(),0nat,4nat,1nat,1nat)==Some(seq![11u8,6u8,11u8,6u8]));
    assert(f::encoded_row(1u8,f::row_data(raw_trns16_palette_d8_16_16(),0nat,4nat,1nat),Some(seq![11u8,6u8,11u8,6u8]),1nat,seq![10u8,14u8,12u8,7u8],0,4nat)) by (compute);
    f::checked_row_hint(1u8,f::row_data(raw_trns16_palette_d8_16_16(),0nat,4nat,1nat),Some(seq![11u8,6u8,11u8,6u8]),1nat,seq![10u8,14u8,12u8,7u8]);
    assert(r::line(raw_trns16_palette_d8_16_16(),a_trns16_palette_d8_16_16(),0nat,1nat)==seq![10u8,14u8,12u8,7u8]);
    assert forall|x: nat| x<4 implies (#[trigger] r::palette_sample(r::line(raw_trns16_palette_d8_16_16(),a_trns16_palette_d8_16_16(),0nat,1nat),8u8,x) as nat)<16 by {
        if x==0 {assert((r::palette_sample(seq![10u8,14u8,12u8,7u8],8u8,0nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d8_16_16(),a_trns16_palette_d8_16_16(),0nat,1nat),8u8,0nat) as nat)<16nat);}
        else if x==1 {assert((r::palette_sample(seq![10u8,14u8,12u8,7u8],8u8,1nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d8_16_16(),a_trns16_palette_d8_16_16(),0nat,1nat),8u8,1nat) as nat)<16nat);}
        else if x==2 {assert((r::palette_sample(seq![10u8,14u8,12u8,7u8],8u8,2nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d8_16_16(),a_trns16_palette_d8_16_16(),0nat,1nat),8u8,2nat) as nat)<16nat);}
        else if x==3 {assert((r::palette_sample(seq![10u8,14u8,12u8,7u8],8u8,3nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d8_16_16(),a_trns16_palette_d8_16_16(),0nat,1nat),8u8,3nat) as nat)<16nat);}
    }
    assert(r::palette_valid(r::line(raw_trns16_palette_d8_16_16(),a_trns16_palette_d8_16_16(),0nat,1nat),8u8,4nat,16nat));
    assert(r::row_result(raw_trns16_palette_d8_16_16(),a_trns16_palette_d8_16_16(),0nat,1nat)==Ok(()));
    assert(raw_trns16_palette_d8_16_16()[10]==2u8) by (compute);
    f::pass_row_equation(raw_trns16_palette_d8_16_16(),0nat,4nat,1nat,2nat);
    assert(f::previous_row(raw_trns16_palette_d8_16_16(),0nat,4nat,1nat,2nat)==Some(seq![10u8,14u8,12u8,7u8]));
    assert(f::encoded_row(2u8,f::row_data(raw_trns16_palette_d8_16_16(),0nat,4nat,2nat),Some(seq![10u8,14u8,12u8,7u8]),1nat,seq![7u8,4u8,0u8,5u8],0,4nat)) by (compute);
    f::checked_row_hint(2u8,f::row_data(raw_trns16_palette_d8_16_16(),0nat,4nat,2nat),Some(seq![10u8,14u8,12u8,7u8]),1nat,seq![7u8,4u8,0u8,5u8]);
    assert(r::line(raw_trns16_palette_d8_16_16(),a_trns16_palette_d8_16_16(),0nat,2nat)==seq![7u8,4u8,0u8,5u8]);
    assert forall|x: nat| x<4 implies (#[trigger] r::palette_sample(r::line(raw_trns16_palette_d8_16_16(),a_trns16_palette_d8_16_16(),0nat,2nat),8u8,x) as nat)<16 by {
        if x==0 {assert((r::palette_sample(seq![7u8,4u8,0u8,5u8],8u8,0nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d8_16_16(),a_trns16_palette_d8_16_16(),0nat,2nat),8u8,0nat) as nat)<16nat);}
        else if x==1 {assert((r::palette_sample(seq![7u8,4u8,0u8,5u8],8u8,1nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d8_16_16(),a_trns16_palette_d8_16_16(),0nat,2nat),8u8,1nat) as nat)<16nat);}
        else if x==2 {assert((r::palette_sample(seq![7u8,4u8,0u8,5u8],8u8,2nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d8_16_16(),a_trns16_palette_d8_16_16(),0nat,2nat),8u8,2nat) as nat)<16nat);}
        else if x==3 {assert((r::palette_sample(seq![7u8,4u8,0u8,5u8],8u8,3nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d8_16_16(),a_trns16_palette_d8_16_16(),0nat,2nat),8u8,3nat) as nat)<16nat);}
    }
    assert(r::palette_valid(r::line(raw_trns16_palette_d8_16_16(),a_trns16_palette_d8_16_16(),0nat,2nat),8u8,4nat,16nat));
    assert(r::row_result(raw_trns16_palette_d8_16_16(),a_trns16_palette_d8_16_16(),0nat,2nat)==Ok(()));
    r::scan_step(raw_trns16_palette_d8_16_16(),a_trns16_palette_d8_16_16(),1nat);
    r::pass_step(raw_trns16_palette_d8_16_16(),a_trns16_palette_d8_16_16(),0nat,3nat);
    r::pass_step(raw_trns16_palette_d8_16_16(),a_trns16_palette_d8_16_16(),0nat,2nat);
    r::pass_step(raw_trns16_palette_d8_16_16(),a_trns16_palette_d8_16_16(),0nat,1nat);
    r::pass_step(raw_trns16_palette_d8_16_16(),a_trns16_palette_d8_16_16(),0nat,0nat);
    r::scan_step(raw_trns16_palette_d8_16_16(),a_trns16_palette_d8_16_16(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_trns16_palette_d8_16_16() ensures r::still(bytes_trns16_palette_d8_16_16(),STRICT,true)==(r::Projection::Ready {width:4u32,height:3u32,bytes:96nat}),
{
    hide(bytes_trns16_palette_d8_16_16);hide(z_trns16_palette_d8_16_16);hide(raw_trns16_palette_d8_16_16);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    assert(bytes_trns16_palette_d8_16_16().len()==168nat) by (compute);
    assert(crate::model::mh(bytes_trns16_palette_d8_16_16())==Ok((h_trns16_palette_d8_16_16(),33nat))) by {reveal(crate::model::mh);assert(crate::model::mh(bytes_trns16_palette_d8_16_16())==Ok((h_trns16_palette_d8_16_16(),33nat))) by (compute);}
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns16_palette_d8_16_16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns16_palette_d8_16_16(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(48nat,3u8,8u8,false,false,false,false)==crate::metadata_model::Decision::Keep(16nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns16_palette_d8_16_16(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:101nat,len:16nat,next:121nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns16_palette_d8_16_16(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:101nat,len:16nat,next:121nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by (compute);
        }
        assert(crate::metadata_model::transparency(16nat,3u8,16nat,false)==crate::metadata_model::Decision::Keep(16nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns16_palette_d8_16_16(),121nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:129nat,len:23nat,next:156nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns16_palette_d8_16_16(),121nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:129nat,len:23nat,next:156nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns16_palette_d8_16_16(),156nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:164nat,len:0nat,next:168nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns16_palette_d8_16_16(),156nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:164nat,len:0nat,next:168nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_trns16_palette_d8_16_16().subrange(129,152)==z_trns16_palette_d8_16_16()) by (compute);
    walk_trns16_palette_d8_16_16(bytes_trns16_palette_d8_16_16());
    inflate_trns16_palette_d8_16_16();scan_trns16_palette_d8_16_16();
    assert(r::raw(a_trns16_palette_d8_16_16())==15nat) by (compute);
    assert(r::data(z_trns16_palette_d8_16_16(),a_trns16_palette_d8_16_16(),STRICT)==Some(Ok(()))) by {reveal(r::data);};
    r::default_ready(bytes_trns16_palette_d8_16_16(),h_trns16_palette_d8_16_16(),z_trns16_palette_d8_16_16(),16nat,true);
}
fn public_trns16_palette_d8_16_16() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,3u8,0u8,0u8,0u8,131u8,42u8,94u8,244u8,0u8,0u8,0u8,48u8,80u8,76u8,84u8,69u8,30u8,63u8,230u8,29u8,184u8,54u8,139u8,90u8,2u8,215u8,196u8,213u8,58u8,106u8,191u8,63u8,17u8,236u8,126u8,26u8,138u8,109u8,172u8,29u8,237u8,144u8,208u8,93u8,107u8,245u8,85u8,120u8,102u8,69u8,5u8,92u8,35u8,4u8,21u8,247u8,45u8,173u8,86u8,224u8,175u8,43u8,140u8,151u8,27u8,30u8,182u8,100u8,0u8,0u8,0u8,16u8,116u8,82u8,78u8,83u8,58u8,60u8,187u8,47u8,80u8,86u8,201u8,127u8,204u8,205u8,43u8,214u8,31u8,120u8,159u8,125u8,7u8,107u8,148u8,20u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,224u8,102u8,227u8,102u8,99u8,228u8,98u8,249u8,247u8,155u8,233u8,239u8,183u8,47u8,255u8,0u8,24u8,219u8,6u8,18u8,62u8,200u8,197u8,78u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_trns16_palette_d8_16_16());model_trns16_palette_d8_16_16();}
    let result=crate::png::try_decode_still16(&b);
    assert(match result {Ok(s)=>s.width==4u32 && s.height==3u32 && s.rgba16@.len()==96nat,Err(crate::alloc_guard::TryError::OutOfMemory(_))=>true,_=>false});
}
// trns16/palette-d4-16-1; frozen SHA256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_trns16_palette_d4_16_1() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,4u8,3u8,0u8,0u8,0u8,70u8,218u8,179u8,245u8,0u8,0u8,0u8,48u8,80u8,76u8,84u8,69u8,49u8,9u8,198u8,204u8,83u8,102u8,94u8,119u8,213u8,153u8,235u8,231u8,152u8,109u8,142u8,220u8,170u8,202u8,108u8,211u8,96u8,46u8,48u8,66u8,148u8,242u8,60u8,49u8,229u8,204u8,196u8,9u8,84u8,14u8,19u8,70u8,253u8,16u8,4u8,51u8,54u8,121u8,163u8,238u8,139u8,229u8,247u8,69u8,6u8,31u8,137u8,86u8,0u8,0u8,0u8,1u8,116u8,82u8,78u8,83u8,17u8,42u8,86u8,248u8,148u8,0u8,0u8,0u8,17u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,88u8,23u8,205u8,24u8,121u8,142u8,105u8,143u8,14u8,0u8,14u8,155u8,3u8,28u8,187u8,61u8,227u8,133u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn h_trns16_palette_d4_16_1() -> crate::model::Ihdr {crate::model::Ihdr {width:4u32,height:3u32,depth:4u8,color_type:3u8,interlace:0u8}}
spec fn a_trns16_palette_d4_16_1() -> r::Raster {r::shape(h_trns16_palette_d4_16_1(),4nat,3nat,16nat,STRICT)}
spec fn z_trns16_palette_d4_16_1() -> Seq<u8> {seq![120u8,156u8,99u8,88u8,23u8,205u8,24u8,121u8,142u8,105u8,143u8,14u8,0u8,14u8,155u8,3u8,28u8]}
spec fn raw_trns16_palette_d4_16_1() -> Seq<u8> {seq![0u8,174u8,91u8,1u8,89u8,206u8,2u8,188u8,44u8]}
spec fn ts_trns16_palette_d4_16_1() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:36nat,before:1nat,after:2nat,symbol:174nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:36nat,next:44nat,before:2nat,after:3nat,symbol:91nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:44nat,next:52nat,before:3nat,after:4nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:52nat,next:60nat,before:4nat,after:5nat,symbol:89nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:60nat,next:69nat,before:5nat,after:6nat,symbol:206nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:69nat,next:77nat,before:6nat,after:7nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:77nat,next:86nat,before:7nat,after:8nat,symbol:188nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:86nat,next:94nat,before:8nat,after:9nat,symbol:44nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:94nat,next:101nat,before:9nat,after:9nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_trns16_palette_d4_16_1_0() ensures t::trace(z_trns16_palette_d4_16_1(),raw_trns16_palette_d4_16_1(),ts_trns16_palette_d4_16_1(),0nat,10nat),
{
    hide(t::valid_step);assert(t::trace(z_trns16_palette_d4_16_1(),raw_trns16_palette_d4_16_1(),ts_trns16_palette_d4_16_1(),0nat,10nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn trace_trns16_palette_d4_16_1() ensures t::trace(z_trns16_palette_d4_16_1(),raw_trns16_palette_d4_16_1(),ts_trns16_palette_d4_16_1(),0,10nat),
{
    hide(z_trns16_palette_d4_16_1);hide(raw_trns16_palette_d4_16_1);hide(ts_trns16_palette_d4_16_1);hide(t::trace);
    trace_trns16_palette_d4_16_1_0();
    assert(t::trace(z_trns16_palette_d4_16_1(),raw_trns16_palette_d4_16_1(),ts_trns16_palette_d4_16_1(),0,10nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_trns16_palette_d4_16_1() ensures z_trns16_palette_d4_16_1().len()==17nat,
        raw_trns16_palette_d4_16_1().len()==9nat,
        z_trns16_palette_d4_16_1()[0]==120u8,
        z_trns16_palette_d4_16_1()[1]==156u8,
        zm::field_value(z_trns16_palette_d4_16_1(),16,1)==1nat,
        zm::field_value(z_trns16_palette_d4_16_1(),17,2)==1nat,
        zm::be_value(z_trns16_palette_d4_16_1(),13nat,4)==245039900nat,
        ts_trns16_palette_d4_16_1().len()==10nat,
        ts_trns16_palette_d4_16_1()[0].pos==19nat,
        ts_trns16_palette_d4_16_1()[0].before==0nat,
        ts_trns16_palette_d4_16_1().last().next==101nat,
{
    assert(z_trns16_palette_d4_16_1().len()==17nat) by (compute);
    assert(raw_trns16_palette_d4_16_1().len()==9nat) by (compute);
    assert(z_trns16_palette_d4_16_1()[0]==120u8) by (compute);
    assert(z_trns16_palette_d4_16_1()[1]==156u8) by (compute);
    assert(zm::field_value(z_trns16_palette_d4_16_1(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_trns16_palette_d4_16_1(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_trns16_palette_d4_16_1(),13nat,4)==245039900nat) by (compute);
    assert(ts_trns16_palette_d4_16_1().len()==10nat) by (compute);
    assert(ts_trns16_palette_d4_16_1()[0].pos==19nat) by (compute);
    assert(ts_trns16_palette_d4_16_1()[0].before==0nat) by (compute);
    assert(ts_trns16_palette_d4_16_1().last().next==101nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_trns16_palette_d4_16_1() ensures zm::Adler(raw_trns16_palette_d4_16_1())==245039900nat,
{
    hide(raw_trns16_palette_d4_16_1);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_trns16_palette_d4_16_1().len()==9nat) by (compute);
    assert(c::sums(raw_trns16_palette_d4_16_1(),0,9nat)==(795nat,3739nat)) by (compute);
    c::sums_correct(raw_trns16_palette_d4_16_1(),0,9nat);assert(raw_trns16_palette_d4_16_1().subrange(0,9)=~=raw_trns16_palette_d4_16_1());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((3739nat%65521)*(u16::MAX as nat+1)+(1+795nat)%65521==245039900nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_trns16_palette_d4_16_1() ensures zm::Inflate(z_trns16_palette_d4_16_1(),9nat,true,false)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_trns16_palette_d4_16_1(),r2:false}),raw_trns16_palette_d4_16_1().len()==9nat,
{
    hide(z_trns16_palette_d4_16_1);hide(raw_trns16_palette_d4_16_1);hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);hide(ts_trns16_palette_d4_16_1);
    shape_trns16_palette_d4_16_1();checksum_trns16_palette_d4_16_1();
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_trns16_palette_d4_16_1();ms::fixed_stream(z_trns16_palette_d4_16_1(),raw_trns16_palette_d4_16_1(),ts_trns16_palette_d4_16_1(),9nat,156nat,245039900nat,245039900nat);
}
#[verifier::spinoff_prover]
proof fn walk_trns16_palette_d4_16_1(b: Seq<u8>)
    requires b.len()==147,
        crate::model::mh(b)==Ok((h_trns16_palette_d4_16_1(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:101nat,len:1nat,next:106nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,106nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:114nat,len:17nat,next:135nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,135nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:143nat,len:0nat,next:147nat,crc_ok:true,synthetic:false})),
        b.subrange(114,131)==z_trns16_palette_d4_16_1(),
    ensures r::default_case(b,h_trns16_palette_d4_16_1(),z_trns16_palette_d4_16_1(),16nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert((1u32 << 4u8)==16u32) by (compute);
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(48nat,3u8,4u8,false,false,false,false)==crate::metadata_model::Decision::Keep(16nat)) by (compute);
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by (compute);
        }
        assert(crate::metadata_model::transparency(1nat,3u8,16nat,false)==crate::metadata_model::Decision::Keep(1nat)) by (compute);
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
proof fn scan_trns16_palette_d4_16_1() ensures r::scan(raw_trns16_palette_d4_16_1(),a_trns16_palette_d4_16_1(),0)==Ok(()),
{
    hide(raw_trns16_palette_d4_16_1);assert(raw_trns16_palette_d4_16_1().len()==9nat) by (compute);assert(r::count(a_trns16_palette_d4_16_1())==1nat) by (compute);
    assert(r::pw(a_trns16_palette_d4_16_1(),0nat)==4nat && r::ph(a_trns16_palette_d4_16_1(),0nat)==3nat && r::rb(a_trns16_palette_d4_16_1(),0nat)==2nat && r::base(a_trns16_palette_d4_16_1(),0nat)==0nat) by (compute);
    assert(raw_trns16_palette_d4_16_1()[0]==0u8) by (compute);
    f::pass_row_equation(raw_trns16_palette_d4_16_1(),0nat,2nat,1nat,0nat);
    assert(f::previous_row(raw_trns16_palette_d4_16_1(),0nat,2nat,1nat,0nat)==None);
    assert(f::encoded_row(0u8,f::row_data(raw_trns16_palette_d4_16_1(),0nat,2nat,0nat),None,1nat,seq![174u8,91u8],0,2nat)) by (compute);
    f::checked_row_hint(0u8,f::row_data(raw_trns16_palette_d4_16_1(),0nat,2nat,0nat),None,1nat,seq![174u8,91u8]);
    assert(r::line(raw_trns16_palette_d4_16_1(),a_trns16_palette_d4_16_1(),0nat,0nat)==seq![174u8,91u8]);
    assert forall|x: nat| x<4 implies (#[trigger] r::palette_sample(r::line(raw_trns16_palette_d4_16_1(),a_trns16_palette_d4_16_1(),0nat,0nat),4u8,x) as nat)<16 by {
        if x==0 {assert((r::palette_sample(seq![174u8,91u8],4u8,0nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d4_16_1(),a_trns16_palette_d4_16_1(),0nat,0nat),4u8,0nat) as nat)<16nat);}
        else if x==1 {assert((r::palette_sample(seq![174u8,91u8],4u8,1nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d4_16_1(),a_trns16_palette_d4_16_1(),0nat,0nat),4u8,1nat) as nat)<16nat);}
        else if x==2 {assert((r::palette_sample(seq![174u8,91u8],4u8,2nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d4_16_1(),a_trns16_palette_d4_16_1(),0nat,0nat),4u8,2nat) as nat)<16nat);}
        else if x==3 {assert((r::palette_sample(seq![174u8,91u8],4u8,3nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d4_16_1(),a_trns16_palette_d4_16_1(),0nat,0nat),4u8,3nat) as nat)<16nat);}
    }
    assert(r::palette_valid(r::line(raw_trns16_palette_d4_16_1(),a_trns16_palette_d4_16_1(),0nat,0nat),4u8,4nat,16nat));
    assert(r::row_result(raw_trns16_palette_d4_16_1(),a_trns16_palette_d4_16_1(),0nat,0nat)==Ok(()));
    assert(raw_trns16_palette_d4_16_1()[3]==1u8) by (compute);
    f::pass_row_equation(raw_trns16_palette_d4_16_1(),0nat,2nat,1nat,1nat);
    assert(f::previous_row(raw_trns16_palette_d4_16_1(),0nat,2nat,1nat,1nat)==Some(seq![174u8,91u8]));
    assert(f::encoded_row(1u8,f::row_data(raw_trns16_palette_d4_16_1(),0nat,2nat,1nat),Some(seq![174u8,91u8]),1nat,seq![89u8,39u8],0,2nat)) by (compute);
    f::checked_row_hint(1u8,f::row_data(raw_trns16_palette_d4_16_1(),0nat,2nat,1nat),Some(seq![174u8,91u8]),1nat,seq![89u8,39u8]);
    assert(r::line(raw_trns16_palette_d4_16_1(),a_trns16_palette_d4_16_1(),0nat,1nat)==seq![89u8,39u8]);
    assert forall|x: nat| x<4 implies (#[trigger] r::palette_sample(r::line(raw_trns16_palette_d4_16_1(),a_trns16_palette_d4_16_1(),0nat,1nat),4u8,x) as nat)<16 by {
        if x==0 {assert((r::palette_sample(seq![89u8,39u8],4u8,0nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d4_16_1(),a_trns16_palette_d4_16_1(),0nat,1nat),4u8,0nat) as nat)<16nat);}
        else if x==1 {assert((r::palette_sample(seq![89u8,39u8],4u8,1nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d4_16_1(),a_trns16_palette_d4_16_1(),0nat,1nat),4u8,1nat) as nat)<16nat);}
        else if x==2 {assert((r::palette_sample(seq![89u8,39u8],4u8,2nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d4_16_1(),a_trns16_palette_d4_16_1(),0nat,1nat),4u8,2nat) as nat)<16nat);}
        else if x==3 {assert((r::palette_sample(seq![89u8,39u8],4u8,3nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d4_16_1(),a_trns16_palette_d4_16_1(),0nat,1nat),4u8,3nat) as nat)<16nat);}
    }
    assert(r::palette_valid(r::line(raw_trns16_palette_d4_16_1(),a_trns16_palette_d4_16_1(),0nat,1nat),4u8,4nat,16nat));
    assert(r::row_result(raw_trns16_palette_d4_16_1(),a_trns16_palette_d4_16_1(),0nat,1nat)==Ok(()));
    assert(raw_trns16_palette_d4_16_1()[6]==2u8) by (compute);
    f::pass_row_equation(raw_trns16_palette_d4_16_1(),0nat,2nat,1nat,2nat);
    assert(f::previous_row(raw_trns16_palette_d4_16_1(),0nat,2nat,1nat,2nat)==Some(seq![89u8,39u8]));
    assert(f::encoded_row(2u8,f::row_data(raw_trns16_palette_d4_16_1(),0nat,2nat,2nat),Some(seq![89u8,39u8]),1nat,seq![21u8,83u8],0,2nat)) by (compute);
    f::checked_row_hint(2u8,f::row_data(raw_trns16_palette_d4_16_1(),0nat,2nat,2nat),Some(seq![89u8,39u8]),1nat,seq![21u8,83u8]);
    assert(r::line(raw_trns16_palette_d4_16_1(),a_trns16_palette_d4_16_1(),0nat,2nat)==seq![21u8,83u8]);
    assert forall|x: nat| x<4 implies (#[trigger] r::palette_sample(r::line(raw_trns16_palette_d4_16_1(),a_trns16_palette_d4_16_1(),0nat,2nat),4u8,x) as nat)<16 by {
        if x==0 {assert((r::palette_sample(seq![21u8,83u8],4u8,0nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d4_16_1(),a_trns16_palette_d4_16_1(),0nat,2nat),4u8,0nat) as nat)<16nat);}
        else if x==1 {assert((r::palette_sample(seq![21u8,83u8],4u8,1nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d4_16_1(),a_trns16_palette_d4_16_1(),0nat,2nat),4u8,1nat) as nat)<16nat);}
        else if x==2 {assert((r::palette_sample(seq![21u8,83u8],4u8,2nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d4_16_1(),a_trns16_palette_d4_16_1(),0nat,2nat),4u8,2nat) as nat)<16nat);}
        else if x==3 {assert((r::palette_sample(seq![21u8,83u8],4u8,3nat) as nat)<16nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d4_16_1(),a_trns16_palette_d4_16_1(),0nat,2nat),4u8,3nat) as nat)<16nat);}
    }
    assert(r::palette_valid(r::line(raw_trns16_palette_d4_16_1(),a_trns16_palette_d4_16_1(),0nat,2nat),4u8,4nat,16nat));
    assert(r::row_result(raw_trns16_palette_d4_16_1(),a_trns16_palette_d4_16_1(),0nat,2nat)==Ok(()));
    r::scan_step(raw_trns16_palette_d4_16_1(),a_trns16_palette_d4_16_1(),1nat);
    r::pass_step(raw_trns16_palette_d4_16_1(),a_trns16_palette_d4_16_1(),0nat,3nat);
    r::pass_step(raw_trns16_palette_d4_16_1(),a_trns16_palette_d4_16_1(),0nat,2nat);
    r::pass_step(raw_trns16_palette_d4_16_1(),a_trns16_palette_d4_16_1(),0nat,1nat);
    r::pass_step(raw_trns16_palette_d4_16_1(),a_trns16_palette_d4_16_1(),0nat,0nat);
    r::scan_step(raw_trns16_palette_d4_16_1(),a_trns16_palette_d4_16_1(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_trns16_palette_d4_16_1() ensures r::still(bytes_trns16_palette_d4_16_1(),STRICT,true)==(r::Projection::Ready {width:4u32,height:3u32,bytes:96nat}),
{
    hide(bytes_trns16_palette_d4_16_1);hide(z_trns16_palette_d4_16_1);hide(raw_trns16_palette_d4_16_1);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    assert(bytes_trns16_palette_d4_16_1().len()==147nat) by (compute);
    assert(crate::model::mh(bytes_trns16_palette_d4_16_1())==Ok((h_trns16_palette_d4_16_1(),33nat))) by {reveal(crate::model::mh);assert(crate::model::mh(bytes_trns16_palette_d4_16_1())==Ok((h_trns16_palette_d4_16_1(),33nat))) by (compute);}
        assert((1u32 << 4u8)==16u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns16_palette_d4_16_1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns16_palette_d4_16_1(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:48nat,next:93nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(48nat,3u8,4u8,false,false,false,false)==crate::metadata_model::Decision::Keep(16nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns16_palette_d4_16_1(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:101nat,len:1nat,next:106nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns16_palette_d4_16_1(),93nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:101nat,len:1nat,next:106nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by (compute);
        }
        assert(crate::metadata_model::transparency(1nat,3u8,16nat,false)==crate::metadata_model::Decision::Keep(1nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns16_palette_d4_16_1(),106nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:114nat,len:17nat,next:135nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns16_palette_d4_16_1(),106nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:114nat,len:17nat,next:135nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns16_palette_d4_16_1(),135nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:143nat,len:0nat,next:147nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns16_palette_d4_16_1(),135nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:143nat,len:0nat,next:147nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_trns16_palette_d4_16_1().subrange(114,131)==z_trns16_palette_d4_16_1()) by (compute);
    walk_trns16_palette_d4_16_1(bytes_trns16_palette_d4_16_1());
    inflate_trns16_palette_d4_16_1();scan_trns16_palette_d4_16_1();
    assert(r::raw(a_trns16_palette_d4_16_1())==9nat) by (compute);
    assert(r::data(z_trns16_palette_d4_16_1(),a_trns16_palette_d4_16_1(),STRICT)==Some(Ok(()))) by {reveal(r::data);};
    r::default_ready(bytes_trns16_palette_d4_16_1(),h_trns16_palette_d4_16_1(),z_trns16_palette_d4_16_1(),16nat,true);
}
fn public_trns16_palette_d4_16_1() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,4u8,3u8,0u8,0u8,0u8,70u8,218u8,179u8,245u8,0u8,0u8,0u8,48u8,80u8,76u8,84u8,69u8,49u8,9u8,198u8,204u8,83u8,102u8,94u8,119u8,213u8,153u8,235u8,231u8,152u8,109u8,142u8,220u8,170u8,202u8,108u8,211u8,96u8,46u8,48u8,66u8,148u8,242u8,60u8,49u8,229u8,204u8,196u8,9u8,84u8,14u8,19u8,70u8,253u8,16u8,4u8,51u8,54u8,121u8,163u8,238u8,139u8,229u8,247u8,69u8,6u8,31u8,137u8,86u8,0u8,0u8,0u8,1u8,116u8,82u8,78u8,83u8,17u8,42u8,86u8,248u8,148u8,0u8,0u8,0u8,17u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,88u8,23u8,205u8,24u8,121u8,142u8,105u8,143u8,14u8,0u8,14u8,155u8,3u8,28u8,187u8,61u8,227u8,133u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_trns16_palette_d4_16_1());model_trns16_palette_d4_16_1();}
    let result=crate::png::try_decode_still16(&b);
    assert(match result {Ok(s)=>s.width==4u32 && s.height==3u32 && s.rgba16@.len()==96nat,Err(crate::alloc_guard::TryError::OutOfMemory(_))=>true,_=>false});
}
// trns/palette-d1-2-entries-2-alpha; frozen SHA256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_trns_palette_d1_2_entries_2_alpha() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,1u8,3u8,0u8,0u8,0u8,142u8,58u8,60u8,133u8,0u8,0u8,0u8,6u8,80u8,76u8,84u8,69u8,38u8,51u8,138u8,62u8,80u8,166u8,187u8,12u8,88u8,254u8,0u8,0u8,0u8,2u8,116u8,82u8,78u8,83u8,221u8,74u8,212u8,12u8,222u8,228u8,0u8,0u8,0u8,14u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,112u8,96u8,44u8,96u8,154u8,0u8,0u8,3u8,46u8,1u8,68u8,31u8,154u8,244u8,77u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn h_trns_palette_d1_2_entries_2_alpha() -> crate::model::Ihdr {crate::model::Ihdr {width:4u32,height:3u32,depth:1u8,color_type:3u8,interlace:0u8}}
spec fn a_trns_palette_d1_2_entries_2_alpha() -> r::Raster {r::shape(h_trns_palette_d1_2_entries_2_alpha(),4nat,3nat,2nat,STRICT)}
spec fn z_trns_palette_d1_2_entries_2_alpha() -> Seq<u8> {seq![120u8,156u8,99u8,112u8,96u8,44u8,96u8,154u8,0u8,0u8,3u8,46u8,1u8,68u8]}
spec fn raw_trns_palette_d1_2_entries_2_alpha() -> Seq<u8> {seq![0u8,64u8,1u8,112u8,2u8,144u8]}
spec fn ts_trns_palette_d1_2_entries_2_alpha() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:35nat,before:1nat,after:2nat,symbol:64nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:35nat,next:43nat,before:2nat,after:3nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:43nat,next:51nat,before:3nat,after:4nat,symbol:112nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:51nat,next:59nat,before:4nat,after:5nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:59nat,next:68nat,before:5nat,after:6nat,symbol:144nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:68nat,next:75nat,before:6nat,after:6nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_trns_palette_d1_2_entries_2_alpha_0() ensures t::trace(z_trns_palette_d1_2_entries_2_alpha(),raw_trns_palette_d1_2_entries_2_alpha(),ts_trns_palette_d1_2_entries_2_alpha(),0nat,7nat),
{
    hide(t::valid_step);assert(t::trace(z_trns_palette_d1_2_entries_2_alpha(),raw_trns_palette_d1_2_entries_2_alpha(),ts_trns_palette_d1_2_entries_2_alpha(),0nat,7nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn trace_trns_palette_d1_2_entries_2_alpha() ensures t::trace(z_trns_palette_d1_2_entries_2_alpha(),raw_trns_palette_d1_2_entries_2_alpha(),ts_trns_palette_d1_2_entries_2_alpha(),0,7nat),
{
    hide(z_trns_palette_d1_2_entries_2_alpha);hide(raw_trns_palette_d1_2_entries_2_alpha);hide(ts_trns_palette_d1_2_entries_2_alpha);hide(t::trace);
    trace_trns_palette_d1_2_entries_2_alpha_0();
    assert(t::trace(z_trns_palette_d1_2_entries_2_alpha(),raw_trns_palette_d1_2_entries_2_alpha(),ts_trns_palette_d1_2_entries_2_alpha(),0,7nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_trns_palette_d1_2_entries_2_alpha() ensures z_trns_palette_d1_2_entries_2_alpha().len()==14nat,
        raw_trns_palette_d1_2_entries_2_alpha().len()==6nat,
        z_trns_palette_d1_2_entries_2_alpha()[0]==120u8,
        z_trns_palette_d1_2_entries_2_alpha()[1]==156u8,
        zm::field_value(z_trns_palette_d1_2_entries_2_alpha(),16,1)==1nat,
        zm::field_value(z_trns_palette_d1_2_entries_2_alpha(),17,2)==1nat,
        zm::be_value(z_trns_palette_d1_2_entries_2_alpha(),10nat,4)==53346628nat,
        ts_trns_palette_d1_2_entries_2_alpha().len()==7nat,
        ts_trns_palette_d1_2_entries_2_alpha()[0].pos==19nat,
        ts_trns_palette_d1_2_entries_2_alpha()[0].before==0nat,
        ts_trns_palette_d1_2_entries_2_alpha().last().next==75nat,
{
    assert(z_trns_palette_d1_2_entries_2_alpha().len()==14nat) by (compute);
    assert(raw_trns_palette_d1_2_entries_2_alpha().len()==6nat) by (compute);
    assert(z_trns_palette_d1_2_entries_2_alpha()[0]==120u8) by (compute);
    assert(z_trns_palette_d1_2_entries_2_alpha()[1]==156u8) by (compute);
    assert(zm::field_value(z_trns_palette_d1_2_entries_2_alpha(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_trns_palette_d1_2_entries_2_alpha(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_trns_palette_d1_2_entries_2_alpha(),10nat,4)==53346628nat) by (compute);
    assert(ts_trns_palette_d1_2_entries_2_alpha().len()==7nat) by (compute);
    assert(ts_trns_palette_d1_2_entries_2_alpha()[0].pos==19nat) by (compute);
    assert(ts_trns_palette_d1_2_entries_2_alpha()[0].before==0nat) by (compute);
    assert(ts_trns_palette_d1_2_entries_2_alpha().last().next==75nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_trns_palette_d1_2_entries_2_alpha() ensures zm::Adler(raw_trns_palette_d1_2_entries_2_alpha())==53346628nat,
{
    hide(raw_trns_palette_d1_2_entries_2_alpha);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_trns_palette_d1_2_entries_2_alpha().len()==6nat) by (compute);
    assert(c::sums(raw_trns_palette_d1_2_entries_2_alpha(),0,6nat)==(323nat,814nat)) by (compute);
    c::sums_correct(raw_trns_palette_d1_2_entries_2_alpha(),0,6nat);assert(raw_trns_palette_d1_2_entries_2_alpha().subrange(0,6)=~=raw_trns_palette_d1_2_entries_2_alpha());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((814nat%65521)*(u16::MAX as nat+1)+(1+323nat)%65521==53346628nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_trns_palette_d1_2_entries_2_alpha() ensures zm::Inflate(z_trns_palette_d1_2_entries_2_alpha(),6nat,true,false)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_trns_palette_d1_2_entries_2_alpha(),r2:false}),raw_trns_palette_d1_2_entries_2_alpha().len()==6nat,
{
    hide(z_trns_palette_d1_2_entries_2_alpha);hide(raw_trns_palette_d1_2_entries_2_alpha);hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);hide(ts_trns_palette_d1_2_entries_2_alpha);
    shape_trns_palette_d1_2_entries_2_alpha();checksum_trns_palette_d1_2_entries_2_alpha();
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_trns_palette_d1_2_entries_2_alpha();ms::fixed_stream(z_trns_palette_d1_2_entries_2_alpha(),raw_trns_palette_d1_2_entries_2_alpha(),ts_trns_palette_d1_2_entries_2_alpha(),6nat,156nat,53346628nat,53346628nat);
}
#[verifier::spinoff_prover]
proof fn walk_trns_palette_d1_2_entries_2_alpha(b: Seq<u8>)
    requires b.len()==103,
        crate::model::mh(b)==Ok((h_trns_palette_d1_2_entries_2_alpha(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:6nat,next:51nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,51nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:59nat,len:2nat,next:65nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,65nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:73nat,len:14nat,next:91nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:99nat,len:0nat,next:103nat,crc_ok:true,synthetic:false})),
        b.subrange(73,87)==z_trns_palette_d1_2_entries_2_alpha(),
    ensures r::default_case(b,h_trns_palette_d1_2_entries_2_alpha(),z_trns_palette_d1_2_entries_2_alpha(),2nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert((1u32 << 1u8)==2u32) by (compute);
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(6nat,3u8,1u8,false,false,false,false)==crate::metadata_model::Decision::Keep(2nat)) by (compute);
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by (compute);
        }
        assert(crate::metadata_model::transparency(2nat,3u8,2nat,false)==crate::metadata_model::Decision::Keep(2nat)) by (compute);
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
proof fn scan_trns_palette_d1_2_entries_2_alpha() ensures r::scan(raw_trns_palette_d1_2_entries_2_alpha(),a_trns_palette_d1_2_entries_2_alpha(),0)==Ok(()),
{
    hide(raw_trns_palette_d1_2_entries_2_alpha);assert(raw_trns_palette_d1_2_entries_2_alpha().len()==6nat) by (compute);assert(r::count(a_trns_palette_d1_2_entries_2_alpha())==1nat) by (compute);
    assert(r::pw(a_trns_palette_d1_2_entries_2_alpha(),0nat)==4nat && r::ph(a_trns_palette_d1_2_entries_2_alpha(),0nat)==3nat && r::rb(a_trns_palette_d1_2_entries_2_alpha(),0nat)==1nat && r::base(a_trns_palette_d1_2_entries_2_alpha(),0nat)==0nat) by (compute);
    assert(raw_trns_palette_d1_2_entries_2_alpha()[0]==0u8) by (compute);
    f::pass_row_equation(raw_trns_palette_d1_2_entries_2_alpha(),0nat,1nat,1nat,0nat);
    assert(f::previous_row(raw_trns_palette_d1_2_entries_2_alpha(),0nat,1nat,1nat,0nat)==None);
    assert(f::encoded_row(0u8,f::row_data(raw_trns_palette_d1_2_entries_2_alpha(),0nat,1nat,0nat),None,1nat,seq![64u8],0,1nat)) by (compute);
    f::checked_row_hint(0u8,f::row_data(raw_trns_palette_d1_2_entries_2_alpha(),0nat,1nat,0nat),None,1nat,seq![64u8]);
    assert(r::line(raw_trns_palette_d1_2_entries_2_alpha(),a_trns_palette_d1_2_entries_2_alpha(),0nat,0nat)==seq![64u8]);
    assert forall|x: nat| x<4 implies (#[trigger] r::palette_sample(r::line(raw_trns_palette_d1_2_entries_2_alpha(),a_trns_palette_d1_2_entries_2_alpha(),0nat,0nat),1u8,x) as nat)<2 by {
        if x==0 {assert((r::palette_sample(seq![64u8],1u8,0nat) as nat)<2nat) by (compute);assert((r::palette_sample(r::line(raw_trns_palette_d1_2_entries_2_alpha(),a_trns_palette_d1_2_entries_2_alpha(),0nat,0nat),1u8,0nat) as nat)<2nat);}
        else if x==1 {assert((r::palette_sample(seq![64u8],1u8,1nat) as nat)<2nat) by (compute);assert((r::palette_sample(r::line(raw_trns_palette_d1_2_entries_2_alpha(),a_trns_palette_d1_2_entries_2_alpha(),0nat,0nat),1u8,1nat) as nat)<2nat);}
        else if x==2 {assert((r::palette_sample(seq![64u8],1u8,2nat) as nat)<2nat) by (compute);assert((r::palette_sample(r::line(raw_trns_palette_d1_2_entries_2_alpha(),a_trns_palette_d1_2_entries_2_alpha(),0nat,0nat),1u8,2nat) as nat)<2nat);}
        else if x==3 {assert((r::palette_sample(seq![64u8],1u8,3nat) as nat)<2nat) by (compute);assert((r::palette_sample(r::line(raw_trns_palette_d1_2_entries_2_alpha(),a_trns_palette_d1_2_entries_2_alpha(),0nat,0nat),1u8,3nat) as nat)<2nat);}
    }
    assert(r::palette_valid(r::line(raw_trns_palette_d1_2_entries_2_alpha(),a_trns_palette_d1_2_entries_2_alpha(),0nat,0nat),1u8,4nat,2nat));
    assert(r::row_result(raw_trns_palette_d1_2_entries_2_alpha(),a_trns_palette_d1_2_entries_2_alpha(),0nat,0nat)==Ok(()));
    assert(raw_trns_palette_d1_2_entries_2_alpha()[2]==1u8) by (compute);
    f::pass_row_equation(raw_trns_palette_d1_2_entries_2_alpha(),0nat,1nat,1nat,1nat);
    assert(f::previous_row(raw_trns_palette_d1_2_entries_2_alpha(),0nat,1nat,1nat,1nat)==Some(seq![64u8]));
    assert(f::encoded_row(1u8,f::row_data(raw_trns_palette_d1_2_entries_2_alpha(),0nat,1nat,1nat),Some(seq![64u8]),1nat,seq![112u8],0,1nat)) by (compute);
    f::checked_row_hint(1u8,f::row_data(raw_trns_palette_d1_2_entries_2_alpha(),0nat,1nat,1nat),Some(seq![64u8]),1nat,seq![112u8]);
    assert(r::line(raw_trns_palette_d1_2_entries_2_alpha(),a_trns_palette_d1_2_entries_2_alpha(),0nat,1nat)==seq![112u8]);
    assert forall|x: nat| x<4 implies (#[trigger] r::palette_sample(r::line(raw_trns_palette_d1_2_entries_2_alpha(),a_trns_palette_d1_2_entries_2_alpha(),0nat,1nat),1u8,x) as nat)<2 by {
        if x==0 {assert((r::palette_sample(seq![112u8],1u8,0nat) as nat)<2nat) by (compute);assert((r::palette_sample(r::line(raw_trns_palette_d1_2_entries_2_alpha(),a_trns_palette_d1_2_entries_2_alpha(),0nat,1nat),1u8,0nat) as nat)<2nat);}
        else if x==1 {assert((r::palette_sample(seq![112u8],1u8,1nat) as nat)<2nat) by (compute);assert((r::palette_sample(r::line(raw_trns_palette_d1_2_entries_2_alpha(),a_trns_palette_d1_2_entries_2_alpha(),0nat,1nat),1u8,1nat) as nat)<2nat);}
        else if x==2 {assert((r::palette_sample(seq![112u8],1u8,2nat) as nat)<2nat) by (compute);assert((r::palette_sample(r::line(raw_trns_palette_d1_2_entries_2_alpha(),a_trns_palette_d1_2_entries_2_alpha(),0nat,1nat),1u8,2nat) as nat)<2nat);}
        else if x==3 {assert((r::palette_sample(seq![112u8],1u8,3nat) as nat)<2nat) by (compute);assert((r::palette_sample(r::line(raw_trns_palette_d1_2_entries_2_alpha(),a_trns_palette_d1_2_entries_2_alpha(),0nat,1nat),1u8,3nat) as nat)<2nat);}
    }
    assert(r::palette_valid(r::line(raw_trns_palette_d1_2_entries_2_alpha(),a_trns_palette_d1_2_entries_2_alpha(),0nat,1nat),1u8,4nat,2nat));
    assert(r::row_result(raw_trns_palette_d1_2_entries_2_alpha(),a_trns_palette_d1_2_entries_2_alpha(),0nat,1nat)==Ok(()));
    assert(raw_trns_palette_d1_2_entries_2_alpha()[4]==2u8) by (compute);
    f::pass_row_equation(raw_trns_palette_d1_2_entries_2_alpha(),0nat,1nat,1nat,2nat);
    assert(f::previous_row(raw_trns_palette_d1_2_entries_2_alpha(),0nat,1nat,1nat,2nat)==Some(seq![112u8]));
    assert(f::encoded_row(2u8,f::row_data(raw_trns_palette_d1_2_entries_2_alpha(),0nat,1nat,2nat),Some(seq![112u8]),1nat,seq![0u8],0,1nat)) by (compute);
    f::checked_row_hint(2u8,f::row_data(raw_trns_palette_d1_2_entries_2_alpha(),0nat,1nat,2nat),Some(seq![112u8]),1nat,seq![0u8]);
    assert(r::line(raw_trns_palette_d1_2_entries_2_alpha(),a_trns_palette_d1_2_entries_2_alpha(),0nat,2nat)==seq![0u8]);
    assert forall|x: nat| x<4 implies (#[trigger] r::palette_sample(r::line(raw_trns_palette_d1_2_entries_2_alpha(),a_trns_palette_d1_2_entries_2_alpha(),0nat,2nat),1u8,x) as nat)<2 by {
        if x==0 {assert((r::palette_sample(seq![0u8],1u8,0nat) as nat)<2nat) by (compute);assert((r::palette_sample(r::line(raw_trns_palette_d1_2_entries_2_alpha(),a_trns_palette_d1_2_entries_2_alpha(),0nat,2nat),1u8,0nat) as nat)<2nat);}
        else if x==1 {assert((r::palette_sample(seq![0u8],1u8,1nat) as nat)<2nat) by (compute);assert((r::palette_sample(r::line(raw_trns_palette_d1_2_entries_2_alpha(),a_trns_palette_d1_2_entries_2_alpha(),0nat,2nat),1u8,1nat) as nat)<2nat);}
        else if x==2 {assert((r::palette_sample(seq![0u8],1u8,2nat) as nat)<2nat) by (compute);assert((r::palette_sample(r::line(raw_trns_palette_d1_2_entries_2_alpha(),a_trns_palette_d1_2_entries_2_alpha(),0nat,2nat),1u8,2nat) as nat)<2nat);}
        else if x==3 {assert((r::palette_sample(seq![0u8],1u8,3nat) as nat)<2nat) by (compute);assert((r::palette_sample(r::line(raw_trns_palette_d1_2_entries_2_alpha(),a_trns_palette_d1_2_entries_2_alpha(),0nat,2nat),1u8,3nat) as nat)<2nat);}
    }
    assert(r::palette_valid(r::line(raw_trns_palette_d1_2_entries_2_alpha(),a_trns_palette_d1_2_entries_2_alpha(),0nat,2nat),1u8,4nat,2nat));
    assert(r::row_result(raw_trns_palette_d1_2_entries_2_alpha(),a_trns_palette_d1_2_entries_2_alpha(),0nat,2nat)==Ok(()));
    r::scan_step(raw_trns_palette_d1_2_entries_2_alpha(),a_trns_palette_d1_2_entries_2_alpha(),1nat);
    r::pass_step(raw_trns_palette_d1_2_entries_2_alpha(),a_trns_palette_d1_2_entries_2_alpha(),0nat,3nat);
    r::pass_step(raw_trns_palette_d1_2_entries_2_alpha(),a_trns_palette_d1_2_entries_2_alpha(),0nat,2nat);
    r::pass_step(raw_trns_palette_d1_2_entries_2_alpha(),a_trns_palette_d1_2_entries_2_alpha(),0nat,1nat);
    r::pass_step(raw_trns_palette_d1_2_entries_2_alpha(),a_trns_palette_d1_2_entries_2_alpha(),0nat,0nat);
    r::scan_step(raw_trns_palette_d1_2_entries_2_alpha(),a_trns_palette_d1_2_entries_2_alpha(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_trns_palette_d1_2_entries_2_alpha() ensures r::still(bytes_trns_palette_d1_2_entries_2_alpha(),STRICT,false)==(r::Projection::Ready {width:4u32,height:3u32,bytes:48nat}),
{
    hide(bytes_trns_palette_d1_2_entries_2_alpha);hide(z_trns_palette_d1_2_entries_2_alpha);hide(raw_trns_palette_d1_2_entries_2_alpha);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    assert(bytes_trns_palette_d1_2_entries_2_alpha().len()==103nat) by (compute);
    assert(crate::model::mh(bytes_trns_palette_d1_2_entries_2_alpha())==Ok((h_trns_palette_d1_2_entries_2_alpha(),33nat))) by {reveal(crate::model::mh);assert(crate::model::mh(bytes_trns_palette_d1_2_entries_2_alpha())==Ok((h_trns_palette_d1_2_entries_2_alpha(),33nat))) by (compute);}
        assert((1u32 << 1u8)==2u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns_palette_d1_2_entries_2_alpha(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:6nat,next:51nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns_palette_d1_2_entries_2_alpha(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:6nat,next:51nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(6nat,3u8,1u8,false,false,false,false)==crate::metadata_model::Decision::Keep(2nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns_palette_d1_2_entries_2_alpha(),51nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:59nat,len:2nat,next:65nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns_palette_d1_2_entries_2_alpha(),51nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:59nat,len:2nat,next:65nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by (compute);
        }
        assert(crate::metadata_model::transparency(2nat,3u8,2nat,false)==crate::metadata_model::Decision::Keep(2nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns_palette_d1_2_entries_2_alpha(),65nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:73nat,len:14nat,next:91nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns_palette_d1_2_entries_2_alpha(),65nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:73nat,len:14nat,next:91nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns_palette_d1_2_entries_2_alpha(),91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:99nat,len:0nat,next:103nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns_palette_d1_2_entries_2_alpha(),91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:99nat,len:0nat,next:103nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_trns_palette_d1_2_entries_2_alpha().subrange(73,87)==z_trns_palette_d1_2_entries_2_alpha()) by (compute);
    walk_trns_palette_d1_2_entries_2_alpha(bytes_trns_palette_d1_2_entries_2_alpha());
    inflate_trns_palette_d1_2_entries_2_alpha();scan_trns_palette_d1_2_entries_2_alpha();
    assert(r::raw(a_trns_palette_d1_2_entries_2_alpha())==6nat) by (compute);
    assert(r::data(z_trns_palette_d1_2_entries_2_alpha(),a_trns_palette_d1_2_entries_2_alpha(),STRICT)==Some(Ok(()))) by {reveal(r::data);};
    r::default_ready(bytes_trns_palette_d1_2_entries_2_alpha(),h_trns_palette_d1_2_entries_2_alpha(),z_trns_palette_d1_2_entries_2_alpha(),2nat,false);
}
fn public_trns_palette_d1_2_entries_2_alpha() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,1u8,3u8,0u8,0u8,0u8,142u8,58u8,60u8,133u8,0u8,0u8,0u8,6u8,80u8,76u8,84u8,69u8,38u8,51u8,138u8,62u8,80u8,166u8,187u8,12u8,88u8,254u8,0u8,0u8,0u8,2u8,116u8,82u8,78u8,83u8,221u8,74u8,212u8,12u8,222u8,228u8,0u8,0u8,0u8,14u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,112u8,96u8,44u8,96u8,154u8,0u8,0u8,3u8,46u8,1u8,68u8,31u8,154u8,244u8,77u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_trns_palette_d1_2_entries_2_alpha());model_trns_palette_d1_2_entries_2_alpha();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==4u32 && s.height==3u32 && s.rgba@.len()==48nat,Err(crate::alloc_guard::TryError::OutOfMemory(_))=>true,_=>false});
}
// trns16/palette-d1-2-2; frozen SHA256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_trns16_palette_d1_2_2() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,1u8,3u8,0u8,0u8,0u8,142u8,58u8,60u8,133u8,0u8,0u8,0u8,6u8,80u8,76u8,84u8,69u8,38u8,51u8,138u8,62u8,80u8,166u8,187u8,12u8,88u8,254u8,0u8,0u8,0u8,2u8,116u8,82u8,78u8,83u8,221u8,74u8,212u8,12u8,222u8,228u8,0u8,0u8,0u8,14u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,112u8,96u8,44u8,96u8,154u8,0u8,0u8,3u8,46u8,1u8,68u8,31u8,154u8,244u8,77u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn h_trns16_palette_d1_2_2() -> crate::model::Ihdr {crate::model::Ihdr {width:4u32,height:3u32,depth:1u8,color_type:3u8,interlace:0u8}}
spec fn a_trns16_palette_d1_2_2() -> r::Raster {r::shape(h_trns16_palette_d1_2_2(),4nat,3nat,2nat,STRICT)}
spec fn z_trns16_palette_d1_2_2() -> Seq<u8> {seq![120u8,156u8,99u8,112u8,96u8,44u8,96u8,154u8,0u8,0u8,3u8,46u8,1u8,68u8]}
spec fn raw_trns16_palette_d1_2_2() -> Seq<u8> {seq![0u8,64u8,1u8,112u8,2u8,144u8]}
spec fn ts_trns16_palette_d1_2_2() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:35nat,before:1nat,after:2nat,symbol:64nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:35nat,next:43nat,before:2nat,after:3nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:43nat,next:51nat,before:3nat,after:4nat,symbol:112nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:51nat,next:59nat,before:4nat,after:5nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:59nat,next:68nat,before:5nat,after:6nat,symbol:144nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:68nat,next:75nat,before:6nat,after:6nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_trns16_palette_d1_2_2_0() ensures t::trace(z_trns16_palette_d1_2_2(),raw_trns16_palette_d1_2_2(),ts_trns16_palette_d1_2_2(),0nat,7nat),
{
    hide(t::valid_step);assert(t::trace(z_trns16_palette_d1_2_2(),raw_trns16_palette_d1_2_2(),ts_trns16_palette_d1_2_2(),0nat,7nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn trace_trns16_palette_d1_2_2() ensures t::trace(z_trns16_palette_d1_2_2(),raw_trns16_palette_d1_2_2(),ts_trns16_palette_d1_2_2(),0,7nat),
{
    hide(z_trns16_palette_d1_2_2);hide(raw_trns16_palette_d1_2_2);hide(ts_trns16_palette_d1_2_2);hide(t::trace);
    trace_trns16_palette_d1_2_2_0();
    assert(t::trace(z_trns16_palette_d1_2_2(),raw_trns16_palette_d1_2_2(),ts_trns16_palette_d1_2_2(),0,7nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_trns16_palette_d1_2_2() ensures z_trns16_palette_d1_2_2().len()==14nat,
        raw_trns16_palette_d1_2_2().len()==6nat,
        z_trns16_palette_d1_2_2()[0]==120u8,
        z_trns16_palette_d1_2_2()[1]==156u8,
        zm::field_value(z_trns16_palette_d1_2_2(),16,1)==1nat,
        zm::field_value(z_trns16_palette_d1_2_2(),17,2)==1nat,
        zm::be_value(z_trns16_palette_d1_2_2(),10nat,4)==53346628nat,
        ts_trns16_palette_d1_2_2().len()==7nat,
        ts_trns16_palette_d1_2_2()[0].pos==19nat,
        ts_trns16_palette_d1_2_2()[0].before==0nat,
        ts_trns16_palette_d1_2_2().last().next==75nat,
{
    assert(z_trns16_palette_d1_2_2().len()==14nat) by (compute);
    assert(raw_trns16_palette_d1_2_2().len()==6nat) by (compute);
    assert(z_trns16_palette_d1_2_2()[0]==120u8) by (compute);
    assert(z_trns16_palette_d1_2_2()[1]==156u8) by (compute);
    assert(zm::field_value(z_trns16_palette_d1_2_2(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_trns16_palette_d1_2_2(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_trns16_palette_d1_2_2(),10nat,4)==53346628nat) by (compute);
    assert(ts_trns16_palette_d1_2_2().len()==7nat) by (compute);
    assert(ts_trns16_palette_d1_2_2()[0].pos==19nat) by (compute);
    assert(ts_trns16_palette_d1_2_2()[0].before==0nat) by (compute);
    assert(ts_trns16_palette_d1_2_2().last().next==75nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_trns16_palette_d1_2_2() ensures zm::Adler(raw_trns16_palette_d1_2_2())==53346628nat,
{
    hide(raw_trns16_palette_d1_2_2);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_trns16_palette_d1_2_2().len()==6nat) by (compute);
    assert(c::sums(raw_trns16_palette_d1_2_2(),0,6nat)==(323nat,814nat)) by (compute);
    c::sums_correct(raw_trns16_palette_d1_2_2(),0,6nat);assert(raw_trns16_palette_d1_2_2().subrange(0,6)=~=raw_trns16_palette_d1_2_2());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((814nat%65521)*(u16::MAX as nat+1)+(1+323nat)%65521==53346628nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_trns16_palette_d1_2_2() ensures zm::Inflate(z_trns16_palette_d1_2_2(),6nat,true,false)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_trns16_palette_d1_2_2(),r2:false}),raw_trns16_palette_d1_2_2().len()==6nat,
{
    hide(z_trns16_palette_d1_2_2);hide(raw_trns16_palette_d1_2_2);hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);hide(ts_trns16_palette_d1_2_2);
    shape_trns16_palette_d1_2_2();checksum_trns16_palette_d1_2_2();
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_trns16_palette_d1_2_2();ms::fixed_stream(z_trns16_palette_d1_2_2(),raw_trns16_palette_d1_2_2(),ts_trns16_palette_d1_2_2(),6nat,156nat,53346628nat,53346628nat);
}
#[verifier::spinoff_prover]
proof fn walk_trns16_palette_d1_2_2(b: Seq<u8>)
    requires b.len()==103,
        crate::model::mh(b)==Ok((h_trns16_palette_d1_2_2(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:6nat,next:51nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,51nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:59nat,len:2nat,next:65nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,65nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:73nat,len:14nat,next:91nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:99nat,len:0nat,next:103nat,crc_ok:true,synthetic:false})),
        b.subrange(73,87)==z_trns16_palette_d1_2_2(),
    ensures r::default_case(b,h_trns16_palette_d1_2_2(),z_trns16_palette_d1_2_2(),2nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert((1u32 << 1u8)==2u32) by (compute);
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(6nat,3u8,1u8,false,false,false,false)==crate::metadata_model::Decision::Keep(2nat)) by (compute);
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by (compute);
        }
        assert(crate::metadata_model::transparency(2nat,3u8,2nat,false)==crate::metadata_model::Decision::Keep(2nat)) by (compute);
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
proof fn scan_trns16_palette_d1_2_2() ensures r::scan(raw_trns16_palette_d1_2_2(),a_trns16_palette_d1_2_2(),0)==Ok(()),
{
    hide(raw_trns16_palette_d1_2_2);assert(raw_trns16_palette_d1_2_2().len()==6nat) by (compute);assert(r::count(a_trns16_palette_d1_2_2())==1nat) by (compute);
    assert(r::pw(a_trns16_palette_d1_2_2(),0nat)==4nat && r::ph(a_trns16_palette_d1_2_2(),0nat)==3nat && r::rb(a_trns16_palette_d1_2_2(),0nat)==1nat && r::base(a_trns16_palette_d1_2_2(),0nat)==0nat) by (compute);
    assert(raw_trns16_palette_d1_2_2()[0]==0u8) by (compute);
    f::pass_row_equation(raw_trns16_palette_d1_2_2(),0nat,1nat,1nat,0nat);
    assert(f::previous_row(raw_trns16_palette_d1_2_2(),0nat,1nat,1nat,0nat)==None);
    assert(f::encoded_row(0u8,f::row_data(raw_trns16_palette_d1_2_2(),0nat,1nat,0nat),None,1nat,seq![64u8],0,1nat)) by (compute);
    f::checked_row_hint(0u8,f::row_data(raw_trns16_palette_d1_2_2(),0nat,1nat,0nat),None,1nat,seq![64u8]);
    assert(r::line(raw_trns16_palette_d1_2_2(),a_trns16_palette_d1_2_2(),0nat,0nat)==seq![64u8]);
    assert forall|x: nat| x<4 implies (#[trigger] r::palette_sample(r::line(raw_trns16_palette_d1_2_2(),a_trns16_palette_d1_2_2(),0nat,0nat),1u8,x) as nat)<2 by {
        if x==0 {assert((r::palette_sample(seq![64u8],1u8,0nat) as nat)<2nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d1_2_2(),a_trns16_palette_d1_2_2(),0nat,0nat),1u8,0nat) as nat)<2nat);}
        else if x==1 {assert((r::palette_sample(seq![64u8],1u8,1nat) as nat)<2nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d1_2_2(),a_trns16_palette_d1_2_2(),0nat,0nat),1u8,1nat) as nat)<2nat);}
        else if x==2 {assert((r::palette_sample(seq![64u8],1u8,2nat) as nat)<2nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d1_2_2(),a_trns16_palette_d1_2_2(),0nat,0nat),1u8,2nat) as nat)<2nat);}
        else if x==3 {assert((r::palette_sample(seq![64u8],1u8,3nat) as nat)<2nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d1_2_2(),a_trns16_palette_d1_2_2(),0nat,0nat),1u8,3nat) as nat)<2nat);}
    }
    assert(r::palette_valid(r::line(raw_trns16_palette_d1_2_2(),a_trns16_palette_d1_2_2(),0nat,0nat),1u8,4nat,2nat));
    assert(r::row_result(raw_trns16_palette_d1_2_2(),a_trns16_palette_d1_2_2(),0nat,0nat)==Ok(()));
    assert(raw_trns16_palette_d1_2_2()[2]==1u8) by (compute);
    f::pass_row_equation(raw_trns16_palette_d1_2_2(),0nat,1nat,1nat,1nat);
    assert(f::previous_row(raw_trns16_palette_d1_2_2(),0nat,1nat,1nat,1nat)==Some(seq![64u8]));
    assert(f::encoded_row(1u8,f::row_data(raw_trns16_palette_d1_2_2(),0nat,1nat,1nat),Some(seq![64u8]),1nat,seq![112u8],0,1nat)) by (compute);
    f::checked_row_hint(1u8,f::row_data(raw_trns16_palette_d1_2_2(),0nat,1nat,1nat),Some(seq![64u8]),1nat,seq![112u8]);
    assert(r::line(raw_trns16_palette_d1_2_2(),a_trns16_palette_d1_2_2(),0nat,1nat)==seq![112u8]);
    assert forall|x: nat| x<4 implies (#[trigger] r::palette_sample(r::line(raw_trns16_palette_d1_2_2(),a_trns16_palette_d1_2_2(),0nat,1nat),1u8,x) as nat)<2 by {
        if x==0 {assert((r::palette_sample(seq![112u8],1u8,0nat) as nat)<2nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d1_2_2(),a_trns16_palette_d1_2_2(),0nat,1nat),1u8,0nat) as nat)<2nat);}
        else if x==1 {assert((r::palette_sample(seq![112u8],1u8,1nat) as nat)<2nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d1_2_2(),a_trns16_palette_d1_2_2(),0nat,1nat),1u8,1nat) as nat)<2nat);}
        else if x==2 {assert((r::palette_sample(seq![112u8],1u8,2nat) as nat)<2nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d1_2_2(),a_trns16_palette_d1_2_2(),0nat,1nat),1u8,2nat) as nat)<2nat);}
        else if x==3 {assert((r::palette_sample(seq![112u8],1u8,3nat) as nat)<2nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d1_2_2(),a_trns16_palette_d1_2_2(),0nat,1nat),1u8,3nat) as nat)<2nat);}
    }
    assert(r::palette_valid(r::line(raw_trns16_palette_d1_2_2(),a_trns16_palette_d1_2_2(),0nat,1nat),1u8,4nat,2nat));
    assert(r::row_result(raw_trns16_palette_d1_2_2(),a_trns16_palette_d1_2_2(),0nat,1nat)==Ok(()));
    assert(raw_trns16_palette_d1_2_2()[4]==2u8) by (compute);
    f::pass_row_equation(raw_trns16_palette_d1_2_2(),0nat,1nat,1nat,2nat);
    assert(f::previous_row(raw_trns16_palette_d1_2_2(),0nat,1nat,1nat,2nat)==Some(seq![112u8]));
    assert(f::encoded_row(2u8,f::row_data(raw_trns16_palette_d1_2_2(),0nat,1nat,2nat),Some(seq![112u8]),1nat,seq![0u8],0,1nat)) by (compute);
    f::checked_row_hint(2u8,f::row_data(raw_trns16_palette_d1_2_2(),0nat,1nat,2nat),Some(seq![112u8]),1nat,seq![0u8]);
    assert(r::line(raw_trns16_palette_d1_2_2(),a_trns16_palette_d1_2_2(),0nat,2nat)==seq![0u8]);
    assert forall|x: nat| x<4 implies (#[trigger] r::palette_sample(r::line(raw_trns16_palette_d1_2_2(),a_trns16_palette_d1_2_2(),0nat,2nat),1u8,x) as nat)<2 by {
        if x==0 {assert((r::palette_sample(seq![0u8],1u8,0nat) as nat)<2nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d1_2_2(),a_trns16_palette_d1_2_2(),0nat,2nat),1u8,0nat) as nat)<2nat);}
        else if x==1 {assert((r::palette_sample(seq![0u8],1u8,1nat) as nat)<2nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d1_2_2(),a_trns16_palette_d1_2_2(),0nat,2nat),1u8,1nat) as nat)<2nat);}
        else if x==2 {assert((r::palette_sample(seq![0u8],1u8,2nat) as nat)<2nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d1_2_2(),a_trns16_palette_d1_2_2(),0nat,2nat),1u8,2nat) as nat)<2nat);}
        else if x==3 {assert((r::palette_sample(seq![0u8],1u8,3nat) as nat)<2nat) by (compute);assert((r::palette_sample(r::line(raw_trns16_palette_d1_2_2(),a_trns16_palette_d1_2_2(),0nat,2nat),1u8,3nat) as nat)<2nat);}
    }
    assert(r::palette_valid(r::line(raw_trns16_palette_d1_2_2(),a_trns16_palette_d1_2_2(),0nat,2nat),1u8,4nat,2nat));
    assert(r::row_result(raw_trns16_palette_d1_2_2(),a_trns16_palette_d1_2_2(),0nat,2nat)==Ok(()));
    r::scan_step(raw_trns16_palette_d1_2_2(),a_trns16_palette_d1_2_2(),1nat);
    r::pass_step(raw_trns16_palette_d1_2_2(),a_trns16_palette_d1_2_2(),0nat,3nat);
    r::pass_step(raw_trns16_palette_d1_2_2(),a_trns16_palette_d1_2_2(),0nat,2nat);
    r::pass_step(raw_trns16_palette_d1_2_2(),a_trns16_palette_d1_2_2(),0nat,1nat);
    r::pass_step(raw_trns16_palette_d1_2_2(),a_trns16_palette_d1_2_2(),0nat,0nat);
    r::scan_step(raw_trns16_palette_d1_2_2(),a_trns16_palette_d1_2_2(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_trns16_palette_d1_2_2() ensures r::still(bytes_trns16_palette_d1_2_2(),STRICT,true)==(r::Projection::Ready {width:4u32,height:3u32,bytes:96nat}),
{
    hide(bytes_trns16_palette_d1_2_2);hide(z_trns16_palette_d1_2_2);hide(raw_trns16_palette_d1_2_2);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    assert(bytes_trns16_palette_d1_2_2().len()==103nat) by (compute);
    assert(crate::model::mh(bytes_trns16_palette_d1_2_2())==Ok((h_trns16_palette_d1_2_2(),33nat))) by {reveal(crate::model::mh);assert(crate::model::mh(bytes_trns16_palette_d1_2_2())==Ok((h_trns16_palette_d1_2_2(),33nat))) by (compute);}
        assert((1u32 << 1u8)==2u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns16_palette_d1_2_2(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:6nat,next:51nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns16_palette_d1_2_2(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:6nat,next:51nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(6nat,3u8,1u8,false,false,false,false)==crate::metadata_model::Decision::Keep(2nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns16_palette_d1_2_2(),51nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:59nat,len:2nat,next:65nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns16_palette_d1_2_2(),51nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![116u8,82u8,78u8,83u8],data:59nat,len:2nat,next:65nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((116u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![116u8,82u8,78u8,83u8])==crate::walk_model::Name::Trns) by (compute);
        }
        assert(crate::metadata_model::transparency(2nat,3u8,2nat,false)==crate::metadata_model::Decision::Keep(2nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns16_palette_d1_2_2(),65nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:73nat,len:14nat,next:91nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns16_palette_d1_2_2(),65nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:73nat,len:14nat,next:91nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_trns16_palette_d1_2_2(),91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:99nat,len:0nat,next:103nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_trns16_palette_d1_2_2(),91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:99nat,len:0nat,next:103nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_trns16_palette_d1_2_2().subrange(73,87)==z_trns16_palette_d1_2_2()) by (compute);
    walk_trns16_palette_d1_2_2(bytes_trns16_palette_d1_2_2());
    inflate_trns16_palette_d1_2_2();scan_trns16_palette_d1_2_2();
    assert(r::raw(a_trns16_palette_d1_2_2())==6nat) by (compute);
    assert(r::data(z_trns16_palette_d1_2_2(),a_trns16_palette_d1_2_2(),STRICT)==Some(Ok(()))) by {reveal(r::data);};
    r::default_ready(bytes_trns16_palette_d1_2_2(),h_trns16_palette_d1_2_2(),z_trns16_palette_d1_2_2(),2nat,true);
}
fn public_trns16_palette_d1_2_2() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,1u8,3u8,0u8,0u8,0u8,142u8,58u8,60u8,133u8,0u8,0u8,0u8,6u8,80u8,76u8,84u8,69u8,38u8,51u8,138u8,62u8,80u8,166u8,187u8,12u8,88u8,254u8,0u8,0u8,0u8,2u8,116u8,82u8,78u8,83u8,221u8,74u8,212u8,12u8,222u8,228u8,0u8,0u8,0u8,14u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,112u8,96u8,44u8,96u8,154u8,0u8,0u8,3u8,46u8,1u8,68u8,31u8,154u8,244u8,77u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_trns16_palette_d1_2_2());model_trns16_palette_d1_2_2();}
    let result=crate::png::try_decode_still16(&b);
    assert(match result {Ok(s)=>s.width==4u32 && s.height==3u32 && s.rgba16@.len()==96nat,Err(crate::alloc_guard::TryError::OutOfMemory(_))=>true,_=>false});
}
// depth16/rounding-boundaries; frozen SHA256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_depth16_rounding_boundaries() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,12u8,0u8,0u8,0u8,1u8,16u8,0u8,0u8,0u8,0u8,159u8,16u8,204u8,166u8,0u8,0u8,0u8,29u8,73u8,68u8,65u8,84u8,120u8,156u8,13u8,198u8,177u8,1u8,0u8,0u8,8u8,194u8,176u8,114u8,57u8,253u8,28u8,205u8,20u8,128u8,98u8,26u8,59u8,177u8,186u8,207u8,118u8,95u8,223u8,10u8,249u8,253u8,177u8,165u8,254u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn h_depth16_rounding_boundaries() -> crate::model::Ihdr {crate::model::Ihdr {width:12u32,height:1u32,depth:16u8,color_type:0u8,interlace:0u8}}
spec fn a_depth16_rounding_boundaries() -> r::Raster {r::shape(h_depth16_rounding_boundaries(),12nat,1nat,0nat,STRICT)}
spec fn z_depth16_rounding_boundaries() -> Seq<u8> {seq![120u8,156u8,13u8,198u8,177u8,1u8,0u8,0u8,8u8,194u8,176u8,114u8,57u8,253u8,28u8,205u8,20u8,128u8,98u8,26u8,59u8,177u8,186u8,207u8,118u8,95u8,223u8,10u8,249u8]}
spec fn raw_depth16_rounding_boundaries() -> Seq<u8> {seq![0u8,0u8,0u8,0u8,127u8,0u8,128u8,1u8,127u8,1u8,128u8,127u8,255u8,128u8,0u8,128u8,127u8,128u8,128u8,255u8,127u8,255u8,128u8,255u8,255u8]}
#[verifier::spinoff_prover]
proof fn inflate_depth16_rounding_boundaries() ensures zm::Inflate(z_depth16_rounding_boundaries(),25nat,true,false)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_depth16_rounding_boundaries(),r2:false}),raw_depth16_rounding_boundaries().len()==25nat,
{
    crate::m5_dynamic_witness_model::checked_rounding();
}
#[verifier::spinoff_prover]
proof fn walk_depth16_rounding_boundaries(b: Seq<u8>)
    requires b.len()==86,
        crate::model::mh(b)==Ok((h_depth16_rounding_boundaries(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:29nat,next:74nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,74nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:82nat,len:0nat,next:86nat,crc_ok:true,synthetic:false})),
        b.subrange(41,70)==z_depth16_rounding_boundaries(),
    ensures r::default_case(b,h_depth16_rounding_boundaries(),z_depth16_rounding_boundaries(),0nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
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
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    reveal_with_fuel(m::drive,3);
    reveal_with_fuel(m::concat_ranges,3);
}
#[verifier::spinoff_prover]
proof fn scan_depth16_rounding_boundaries() ensures r::scan(raw_depth16_rounding_boundaries(),a_depth16_rounding_boundaries(),0)==Ok(()),
{
    hide(raw_depth16_rounding_boundaries);assert(raw_depth16_rounding_boundaries().len()==25nat) by (compute);assert(r::count(a_depth16_rounding_boundaries())==1nat) by (compute);
    assert(r::pw(a_depth16_rounding_boundaries(),0nat)==12nat && r::ph(a_depth16_rounding_boundaries(),0nat)==1nat && r::rb(a_depth16_rounding_boundaries(),0nat)==24nat && r::base(a_depth16_rounding_boundaries(),0nat)==0nat) by (compute);
    assert(raw_depth16_rounding_boundaries()[0]==0u8) by (compute);
    assert(r::row_result(raw_depth16_rounding_boundaries(),a_depth16_rounding_boundaries(),0nat,0nat)==Ok(()));
    r::scan_step(raw_depth16_rounding_boundaries(),a_depth16_rounding_boundaries(),1nat);
    r::pass_step(raw_depth16_rounding_boundaries(),a_depth16_rounding_boundaries(),0nat,1nat);
    r::pass_step(raw_depth16_rounding_boundaries(),a_depth16_rounding_boundaries(),0nat,0nat);
    r::scan_step(raw_depth16_rounding_boundaries(),a_depth16_rounding_boundaries(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_depth16_rounding_boundaries() ensures r::still(bytes_depth16_rounding_boundaries(),STRICT,false)==(r::Projection::Ready {width:12u32,height:1u32,bytes:48nat}),
{
    hide(bytes_depth16_rounding_boundaries);hide(z_depth16_rounding_boundaries);hide(raw_depth16_rounding_boundaries);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    assert(bytes_depth16_rounding_boundaries().len()==86nat) by (compute);
    assert(crate::model::mh(bytes_depth16_rounding_boundaries())==Ok((h_depth16_rounding_boundaries(),33nat))) by {reveal(crate::model::mh);assert(crate::model::mh(bytes_depth16_rounding_boundaries())==Ok((h_depth16_rounding_boundaries(),33nat))) by (compute);}
        assert((1u32 << 16u8)==(u16::MAX as u32)+1) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_depth16_rounding_boundaries(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:29nat,next:74nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_depth16_rounding_boundaries(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:29nat,next:74nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_depth16_rounding_boundaries(),74nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:82nat,len:0nat,next:86nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_depth16_rounding_boundaries(),74nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:82nat,len:0nat,next:86nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_depth16_rounding_boundaries().subrange(41,70)==z_depth16_rounding_boundaries()) by (compute);
    walk_depth16_rounding_boundaries(bytes_depth16_rounding_boundaries());
    inflate_depth16_rounding_boundaries();scan_depth16_rounding_boundaries();
    assert(r::raw(a_depth16_rounding_boundaries())==25nat) by (compute);
    assert(r::data(z_depth16_rounding_boundaries(),a_depth16_rounding_boundaries(),STRICT)==Some(Ok(()))) by {reveal(r::data);};
    r::default_ready(bytes_depth16_rounding_boundaries(),h_depth16_rounding_boundaries(),z_depth16_rounding_boundaries(),0nat,false);
}
fn public_depth16_rounding_boundaries() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,12u8,0u8,0u8,0u8,1u8,16u8,0u8,0u8,0u8,0u8,159u8,16u8,204u8,166u8,0u8,0u8,0u8,29u8,73u8,68u8,65u8,84u8,120u8,156u8,13u8,198u8,177u8,1u8,0u8,0u8,8u8,194u8,176u8,114u8,57u8,253u8,28u8,205u8,20u8,128u8,98u8,26u8,59u8,177u8,186u8,207u8,118u8,95u8,223u8,10u8,249u8,253u8,177u8,165u8,254u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_depth16_rounding_boundaries());model_depth16_rounding_boundaries();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==12u32 && s.height==1u32 && s.rgba@.len()==48nat,Err(crate::alloc_guard::TryError::OutOfMemory(_))=>true,_=>false});
}
// image/palette-index-out-of-range; frozen SHA256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_image_palette_index_out_of_range() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,2u8,8u8,3u8,0u8,0u8,0u8,170u8,170u8,150u8,40u8,0u8,0u8,0u8,12u8,80u8,76u8,84u8,69u8,126u8,240u8,116u8,184u8,57u8,97u8,203u8,4u8,217u8,68u8,9u8,86u8,59u8,166u8,158u8,64u8,0u8,0u8,0u8,16u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,96u8,100u8,98u8,100u8,102u8,252u8,3u8,0u8,1u8,35u8,1u8,5u8,211u8,209u8,236u8,242u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn h_image_palette_index_out_of_range() -> crate::model::Ihdr {crate::model::Ihdr {width:3u32,height:2u32,depth:8u8,color_type:3u8,interlace:0u8}}
spec fn a_image_palette_index_out_of_range() -> r::Raster {r::shape(h_image_palette_index_out_of_range(),3nat,2nat,4nat,STRICT)}
spec fn z_image_palette_index_out_of_range() -> Seq<u8> {seq![120u8,156u8,99u8,96u8,96u8,100u8,98u8,100u8,102u8,252u8,3u8,0u8,1u8,35u8,1u8,5u8]}
spec fn raw_image_palette_index_out_of_range() -> Seq<u8> {seq![0u8,0u8,1u8,2u8,1u8,3u8,1u8,252u8]}
spec fn ts_image_palette_index_out_of_range() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:35nat,before:1nat,after:2nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:35nat,next:43nat,before:2nat,after:3nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:43nat,next:51nat,before:3nat,after:4nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:51nat,next:59nat,before:4nat,after:5nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:59nat,next:67nat,before:5nat,after:6nat,symbol:3nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:67nat,next:75nat,before:6nat,after:7nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:75nat,next:84nat,before:7nat,after:8nat,symbol:252nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:84nat,next:91nat,before:8nat,after:8nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_image_palette_index_out_of_range_0() ensures t::trace(z_image_palette_index_out_of_range(),raw_image_palette_index_out_of_range(),ts_image_palette_index_out_of_range(),0nat,9nat),
{
    hide(t::valid_step);assert(t::trace(z_image_palette_index_out_of_range(),raw_image_palette_index_out_of_range(),ts_image_palette_index_out_of_range(),0nat,9nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn trace_image_palette_index_out_of_range() ensures t::trace(z_image_palette_index_out_of_range(),raw_image_palette_index_out_of_range(),ts_image_palette_index_out_of_range(),0,9nat),
{
    hide(z_image_palette_index_out_of_range);hide(raw_image_palette_index_out_of_range);hide(ts_image_palette_index_out_of_range);hide(t::trace);
    trace_image_palette_index_out_of_range_0();
    assert(t::trace(z_image_palette_index_out_of_range(),raw_image_palette_index_out_of_range(),ts_image_palette_index_out_of_range(),0,9nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_image_palette_index_out_of_range() ensures z_image_palette_index_out_of_range().len()==16nat,
        raw_image_palette_index_out_of_range().len()==8nat,
        z_image_palette_index_out_of_range()[0]==120u8,
        z_image_palette_index_out_of_range()[1]==156u8,
        zm::field_value(z_image_palette_index_out_of_range(),16,1)==1nat,
        zm::field_value(z_image_palette_index_out_of_range(),17,2)==1nat,
        zm::be_value(z_image_palette_index_out_of_range(),12nat,4)==19071237nat,
        ts_image_palette_index_out_of_range().len()==9nat,
        ts_image_palette_index_out_of_range()[0].pos==19nat,
        ts_image_palette_index_out_of_range()[0].before==0nat,
        ts_image_palette_index_out_of_range().last().next==91nat,
{
    assert(z_image_palette_index_out_of_range().len()==16nat) by (compute);
    assert(raw_image_palette_index_out_of_range().len()==8nat) by (compute);
    assert(z_image_palette_index_out_of_range()[0]==120u8) by (compute);
    assert(z_image_palette_index_out_of_range()[1]==156u8) by (compute);
    assert(zm::field_value(z_image_palette_index_out_of_range(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_image_palette_index_out_of_range(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_image_palette_index_out_of_range(),12nat,4)==19071237nat) by (compute);
    assert(ts_image_palette_index_out_of_range().len()==9nat) by (compute);
    assert(ts_image_palette_index_out_of_range()[0].pos==19nat) by (compute);
    assert(ts_image_palette_index_out_of_range()[0].before==0nat) by (compute);
    assert(ts_image_palette_index_out_of_range().last().next==91nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_image_palette_index_out_of_range() ensures zm::Adler(raw_image_palette_index_out_of_range())==19071237nat,
{
    hide(raw_image_palette_index_out_of_range);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_image_palette_index_out_of_range().len()==8nat) by (compute);
    assert(c::sums(raw_image_palette_index_out_of_range(),0,8nat)==(260nat,291nat)) by (compute);
    c::sums_correct(raw_image_palette_index_out_of_range(),0,8nat);assert(raw_image_palette_index_out_of_range().subrange(0,8)=~=raw_image_palette_index_out_of_range());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((291nat%65521)*(u16::MAX as nat+1)+(1+260nat)%65521==19071237nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_image_palette_index_out_of_range() ensures zm::Inflate(z_image_palette_index_out_of_range(),8nat,true,false)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_image_palette_index_out_of_range(),r2:false}),raw_image_palette_index_out_of_range().len()==8nat,
{
    hide(z_image_palette_index_out_of_range);hide(raw_image_palette_index_out_of_range);hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);hide(ts_image_palette_index_out_of_range);
    shape_image_palette_index_out_of_range();checksum_image_palette_index_out_of_range();
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_image_palette_index_out_of_range();ms::fixed_stream(z_image_palette_index_out_of_range(),raw_image_palette_index_out_of_range(),ts_image_palette_index_out_of_range(),8nat,156nat,19071237nat,19071237nat);
}
#[verifier::spinoff_prover]
proof fn walk_image_palette_index_out_of_range(b: Seq<u8>)
    requires b.len()==97,
        crate::model::mh(b)==Ok((h_image_palette_index_out_of_range(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:12nat,next:57nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,57nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:65nat,len:16nat,next:85nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,85nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:93nat,len:0nat,next:97nat,crc_ok:true,synthetic:false})),
        b.subrange(65,81)==z_image_palette_index_out_of_range(),
    ensures r::default_case(b,h_image_palette_index_out_of_range(),z_image_palette_index_out_of_range(),4nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert((1u32 << 8u8)==256u32) by (compute);
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(12nat,3u8,8u8,false,false,false,false)==crate::metadata_model::Decision::Keep(4nat)) by (compute);
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
proof fn scan_image_palette_index_out_of_range() ensures r::scan(raw_image_palette_index_out_of_range(),a_image_palette_index_out_of_range(),0)==Err(Error::PaletteIndexOutOfRange),
{
    hide(raw_image_palette_index_out_of_range);assert(raw_image_palette_index_out_of_range().len()==8nat) by (compute);assert(r::count(a_image_palette_index_out_of_range())==1nat) by (compute);
    assert(r::pw(a_image_palette_index_out_of_range(),0nat)==3nat && r::ph(a_image_palette_index_out_of_range(),0nat)==2nat && r::rb(a_image_palette_index_out_of_range(),0nat)==3nat && r::base(a_image_palette_index_out_of_range(),0nat)==0nat) by (compute);
    assert(raw_image_palette_index_out_of_range()[0]==0u8) by (compute);
    f::pass_row_equation(raw_image_palette_index_out_of_range(),0nat,3nat,1nat,0nat);
    assert(f::previous_row(raw_image_palette_index_out_of_range(),0nat,3nat,1nat,0nat)==None);
    assert(f::encoded_row(0u8,f::row_data(raw_image_palette_index_out_of_range(),0nat,3nat,0nat),None,1nat,seq![0u8,1u8,2u8],0,3nat)) by (compute);
    f::checked_row_hint(0u8,f::row_data(raw_image_palette_index_out_of_range(),0nat,3nat,0nat),None,1nat,seq![0u8,1u8,2u8]);
    assert(r::line(raw_image_palette_index_out_of_range(),a_image_palette_index_out_of_range(),0nat,0nat)==seq![0u8,1u8,2u8]);
    assert forall|x: nat| x<3 implies (#[trigger] r::palette_sample(r::line(raw_image_palette_index_out_of_range(),a_image_palette_index_out_of_range(),0nat,0nat),8u8,x) as nat)<4 by {
        if x==0 {assert((r::palette_sample(seq![0u8,1u8,2u8],8u8,0nat) as nat)<4nat) by (compute);assert((r::palette_sample(r::line(raw_image_palette_index_out_of_range(),a_image_palette_index_out_of_range(),0nat,0nat),8u8,0nat) as nat)<4nat);}
        else if x==1 {assert((r::palette_sample(seq![0u8,1u8,2u8],8u8,1nat) as nat)<4nat) by (compute);assert((r::palette_sample(r::line(raw_image_palette_index_out_of_range(),a_image_palette_index_out_of_range(),0nat,0nat),8u8,1nat) as nat)<4nat);}
        else if x==2 {assert((r::palette_sample(seq![0u8,1u8,2u8],8u8,2nat) as nat)<4nat) by (compute);assert((r::palette_sample(r::line(raw_image_palette_index_out_of_range(),a_image_palette_index_out_of_range(),0nat,0nat),8u8,2nat) as nat)<4nat);}
    }
    assert(r::palette_valid(r::line(raw_image_palette_index_out_of_range(),a_image_palette_index_out_of_range(),0nat,0nat),8u8,3nat,4nat));
    assert(r::row_result(raw_image_palette_index_out_of_range(),a_image_palette_index_out_of_range(),0nat,0nat)==Ok(()));
    assert(raw_image_palette_index_out_of_range()[4]==1u8) by (compute);
    f::pass_row_equation(raw_image_palette_index_out_of_range(),0nat,3nat,1nat,1nat);
    assert(f::previous_row(raw_image_palette_index_out_of_range(),0nat,3nat,1nat,1nat)==Some(seq![0u8,1u8,2u8]));
    assert(f::encoded_row(1u8,f::row_data(raw_image_palette_index_out_of_range(),0nat,3nat,1nat),Some(seq![0u8,1u8,2u8]),1nat,seq![3u8,4u8,0u8],0,3nat)) by (compute);
    f::checked_row_hint(1u8,f::row_data(raw_image_palette_index_out_of_range(),0nat,3nat,1nat),Some(seq![0u8,1u8,2u8]),1nat,seq![3u8,4u8,0u8]);
    assert(r::line(raw_image_palette_index_out_of_range(),a_image_palette_index_out_of_range(),0nat,1nat)==seq![3u8,4u8,0u8]);
    assert(r::palette_sample(seq![3u8,4u8,0u8],8u8,1nat)==4u8) by (compute);
    assert(r::palette_sample(r::line(raw_image_palette_index_out_of_range(),a_image_palette_index_out_of_range(),0nat,1nat),8u8,1nat)==4u8);
    assert(!r::palette_valid(r::line(raw_image_palette_index_out_of_range(),a_image_palette_index_out_of_range(),0nat,1nat),8u8,3nat,4nat));
    assert(r::row_result(raw_image_palette_index_out_of_range(),a_image_palette_index_out_of_range(),0nat,1nat)==Err(Error::PaletteIndexOutOfRange));
    r::scan_step(raw_image_palette_index_out_of_range(),a_image_palette_index_out_of_range(),1nat);
    r::pass_step(raw_image_palette_index_out_of_range(),a_image_palette_index_out_of_range(),0nat,2nat);
    r::pass_step(raw_image_palette_index_out_of_range(),a_image_palette_index_out_of_range(),0nat,1nat);
    r::pass_step(raw_image_palette_index_out_of_range(),a_image_palette_index_out_of_range(),0nat,0nat);
    r::scan_step(raw_image_palette_index_out_of_range(),a_image_palette_index_out_of_range(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_image_palette_index_out_of_range() ensures r::still(bytes_image_palette_index_out_of_range(),STRICT,false)==r::Projection::Error(Error::PaletteIndexOutOfRange),
{
    hide(bytes_image_palette_index_out_of_range);hide(z_image_palette_index_out_of_range);hide(raw_image_palette_index_out_of_range);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    assert(bytes_image_palette_index_out_of_range().len()==97nat) by (compute);
    assert(crate::model::mh(bytes_image_palette_index_out_of_range())==Ok((h_image_palette_index_out_of_range(),33nat))) by {reveal(crate::model::mh);assert(crate::model::mh(bytes_image_palette_index_out_of_range())==Ok((h_image_palette_index_out_of_range(),33nat))) by (compute);}
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_image_palette_index_out_of_range(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:12nat,next:57nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_image_palette_index_out_of_range(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:12nat,next:57nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(12nat,3u8,8u8,false,false,false,false)==crate::metadata_model::Decision::Keep(4nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_image_palette_index_out_of_range(),57nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:65nat,len:16nat,next:85nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_image_palette_index_out_of_range(),57nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:65nat,len:16nat,next:85nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_image_palette_index_out_of_range(),85nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:93nat,len:0nat,next:97nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_image_palette_index_out_of_range(),85nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:93nat,len:0nat,next:97nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_image_palette_index_out_of_range().subrange(65,81)==z_image_palette_index_out_of_range()) by (compute);
    walk_image_palette_index_out_of_range(bytes_image_palette_index_out_of_range());
    inflate_image_palette_index_out_of_range();scan_image_palette_index_out_of_range();
    assert(r::raw(a_image_palette_index_out_of_range())==8nat) by (compute);
    assert(r::data(z_image_palette_index_out_of_range(),a_image_palette_index_out_of_range(),STRICT)==Some(Err(Error::PaletteIndexOutOfRange))) by {reveal(r::data);};
    r::default_error(bytes_image_palette_index_out_of_range(),h_image_palette_index_out_of_range(),z_image_palette_index_out_of_range(),4nat,Error::PaletteIndexOutOfRange);
}
fn public_image_palette_index_out_of_range() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,2u8,8u8,3u8,0u8,0u8,0u8,170u8,170u8,150u8,40u8,0u8,0u8,0u8,12u8,80u8,76u8,84u8,69u8,126u8,240u8,116u8,184u8,57u8,97u8,203u8,4u8,217u8,68u8,9u8,86u8,59u8,166u8,158u8,64u8,0u8,0u8,0u8,16u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,96u8,100u8,98u8,100u8,102u8,252u8,3u8,0u8,1u8,35u8,1u8,5u8,211u8,209u8,236u8,242u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_image_palette_index_out_of_range());model_image_palette_index_out_of_range();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Err(crate::alloc_guard::TryError::OutOfMemory(_))=>true,_=>result==Err(crate::alloc_guard::TryError::Format(Error::PaletteIndexOutOfRange))});
}
// image/palette-index-out-of-range-d2; frozen SHA256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_image_palette_index_out_of_range_d2() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,2u8,2u8,3u8,0u8,0u8,0u8,224u8,26u8,142u8,137u8,0u8,0u8,0u8,9u8,80u8,76u8,84u8,69u8,135u8,87u8,218u8,230u8,114u8,114u8,211u8,133u8,142u8,144u8,5u8,72u8,245u8,0u8,0u8,0u8,12u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,144u8,97u8,100u8,0u8,0u8,0u8,90u8,0u8,30u8,179u8,46u8,209u8,13u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn h_image_palette_index_out_of_range_d2() -> crate::model::Ihdr {crate::model::Ihdr {width:3u32,height:2u32,depth:2u8,color_type:3u8,interlace:0u8}}
spec fn a_image_palette_index_out_of_range_d2() -> r::Raster {r::shape(h_image_palette_index_out_of_range_d2(),3nat,2nat,3nat,STRICT)}
spec fn z_image_palette_index_out_of_range_d2() -> Seq<u8> {seq![120u8,156u8,99u8,144u8,97u8,100u8,0u8,0u8,0u8,90u8,0u8,30u8]}
spec fn raw_image_palette_index_out_of_range_d2() -> Seq<u8> {seq![0u8,28u8,1u8,0u8]}
spec fn ts_image_palette_index_out_of_range_d2() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:35nat,before:1nat,after:2nat,symbol:28nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:35nat,next:43nat,before:2nat,after:3nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:43nat,next:51nat,before:3nat,after:4nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:51nat,next:58nat,before:4nat,after:4nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_image_palette_index_out_of_range_d2_0() ensures t::trace(z_image_palette_index_out_of_range_d2(),raw_image_palette_index_out_of_range_d2(),ts_image_palette_index_out_of_range_d2(),0nat,5nat),
{
    hide(t::valid_step);assert(t::trace(z_image_palette_index_out_of_range_d2(),raw_image_palette_index_out_of_range_d2(),ts_image_palette_index_out_of_range_d2(),0nat,5nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn trace_image_palette_index_out_of_range_d2() ensures t::trace(z_image_palette_index_out_of_range_d2(),raw_image_palette_index_out_of_range_d2(),ts_image_palette_index_out_of_range_d2(),0,5nat),
{
    hide(z_image_palette_index_out_of_range_d2);hide(raw_image_palette_index_out_of_range_d2);hide(ts_image_palette_index_out_of_range_d2);hide(t::trace);
    trace_image_palette_index_out_of_range_d2_0();
    assert(t::trace(z_image_palette_index_out_of_range_d2(),raw_image_palette_index_out_of_range_d2(),ts_image_palette_index_out_of_range_d2(),0,5nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_image_palette_index_out_of_range_d2() ensures z_image_palette_index_out_of_range_d2().len()==12nat,
        raw_image_palette_index_out_of_range_d2().len()==4nat,
        z_image_palette_index_out_of_range_d2()[0]==120u8,
        z_image_palette_index_out_of_range_d2()[1]==156u8,
        zm::field_value(z_image_palette_index_out_of_range_d2(),16,1)==1nat,
        zm::field_value(z_image_palette_index_out_of_range_d2(),17,2)==1nat,
        zm::be_value(z_image_palette_index_out_of_range_d2(),8nat,4)==5898270nat,
        ts_image_palette_index_out_of_range_d2().len()==5nat,
        ts_image_palette_index_out_of_range_d2()[0].pos==19nat,
        ts_image_palette_index_out_of_range_d2()[0].before==0nat,
        ts_image_palette_index_out_of_range_d2().last().next==58nat,
{
    assert(z_image_palette_index_out_of_range_d2().len()==12nat) by (compute);
    assert(raw_image_palette_index_out_of_range_d2().len()==4nat) by (compute);
    assert(z_image_palette_index_out_of_range_d2()[0]==120u8) by (compute);
    assert(z_image_palette_index_out_of_range_d2()[1]==156u8) by (compute);
    assert(zm::field_value(z_image_palette_index_out_of_range_d2(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_image_palette_index_out_of_range_d2(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_image_palette_index_out_of_range_d2(),8nat,4)==5898270nat) by (compute);
    assert(ts_image_palette_index_out_of_range_d2().len()==5nat) by (compute);
    assert(ts_image_palette_index_out_of_range_d2()[0].pos==19nat) by (compute);
    assert(ts_image_palette_index_out_of_range_d2()[0].before==0nat) by (compute);
    assert(ts_image_palette_index_out_of_range_d2().last().next==58nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_image_palette_index_out_of_range_d2() ensures zm::Adler(raw_image_palette_index_out_of_range_d2())==5898270nat,
{
    hide(raw_image_palette_index_out_of_range_d2);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_image_palette_index_out_of_range_d2().len()==4nat) by (compute);
    assert(c::sums(raw_image_palette_index_out_of_range_d2(),0,4nat)==(29nat,90nat)) by (compute);
    c::sums_correct(raw_image_palette_index_out_of_range_d2(),0,4nat);assert(raw_image_palette_index_out_of_range_d2().subrange(0,4)=~=raw_image_palette_index_out_of_range_d2());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((90nat%65521)*(u16::MAX as nat+1)+(1+29nat)%65521==5898270nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_image_palette_index_out_of_range_d2() ensures zm::Inflate(z_image_palette_index_out_of_range_d2(),4nat,true,false)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_image_palette_index_out_of_range_d2(),r2:false}),raw_image_palette_index_out_of_range_d2().len()==4nat,
{
    hide(z_image_palette_index_out_of_range_d2);hide(raw_image_palette_index_out_of_range_d2);hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);hide(ts_image_palette_index_out_of_range_d2);
    shape_image_palette_index_out_of_range_d2();checksum_image_palette_index_out_of_range_d2();
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_image_palette_index_out_of_range_d2();ms::fixed_stream(z_image_palette_index_out_of_range_d2(),raw_image_palette_index_out_of_range_d2(),ts_image_palette_index_out_of_range_d2(),4nat,156nat,5898270nat,5898270nat);
}
#[verifier::spinoff_prover]
proof fn walk_image_palette_index_out_of_range_d2(b: Seq<u8>)
    requires b.len()==90,
        crate::model::mh(b)==Ok((h_image_palette_index_out_of_range_d2(),33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:9nat,next:54nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,54nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:62nat,len:12nat,next:78nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,78nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:86nat,len:0nat,next:90nat,crc_ok:true,synthetic:false})),
        b.subrange(62,74)==z_image_palette_index_out_of_range_d2(),
    ensures r::default_case(b,h_image_palette_index_out_of_range_d2(),z_image_palette_index_out_of_range_d2(),3nat),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
        assert((1u32 << 2u8)==4u32) by (compute);
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(9nat,3u8,2u8,false,false,false,false)==crate::metadata_model::Decision::Keep(3nat)) by (compute);
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
proof fn scan_image_palette_index_out_of_range_d2() ensures r::scan(raw_image_palette_index_out_of_range_d2(),a_image_palette_index_out_of_range_d2(),0)==Err(Error::PaletteIndexOutOfRange),
{
    hide(raw_image_palette_index_out_of_range_d2);assert(raw_image_palette_index_out_of_range_d2().len()==4nat) by (compute);assert(r::count(a_image_palette_index_out_of_range_d2())==1nat) by (compute);
    assert(r::pw(a_image_palette_index_out_of_range_d2(),0nat)==3nat && r::ph(a_image_palette_index_out_of_range_d2(),0nat)==2nat && r::rb(a_image_palette_index_out_of_range_d2(),0nat)==1nat && r::base(a_image_palette_index_out_of_range_d2(),0nat)==0nat) by (compute);
    assert(raw_image_palette_index_out_of_range_d2()[0]==0u8) by (compute);
    f::pass_row_equation(raw_image_palette_index_out_of_range_d2(),0nat,1nat,1nat,0nat);
    assert(f::previous_row(raw_image_palette_index_out_of_range_d2(),0nat,1nat,1nat,0nat)==None);
    assert(f::encoded_row(0u8,f::row_data(raw_image_palette_index_out_of_range_d2(),0nat,1nat,0nat),None,1nat,seq![28u8],0,1nat)) by (compute);
    f::checked_row_hint(0u8,f::row_data(raw_image_palette_index_out_of_range_d2(),0nat,1nat,0nat),None,1nat,seq![28u8]);
    assert(r::line(raw_image_palette_index_out_of_range_d2(),a_image_palette_index_out_of_range_d2(),0nat,0nat)==seq![28u8]);
    assert(r::palette_sample(seq![28u8],2u8,2nat)==3u8) by (compute);
    assert(r::palette_sample(r::line(raw_image_palette_index_out_of_range_d2(),a_image_palette_index_out_of_range_d2(),0nat,0nat),2u8,2nat)==3u8);
    assert(!r::palette_valid(r::line(raw_image_palette_index_out_of_range_d2(),a_image_palette_index_out_of_range_d2(),0nat,0nat),2u8,3nat,3nat));
    assert(r::row_result(raw_image_palette_index_out_of_range_d2(),a_image_palette_index_out_of_range_d2(),0nat,0nat)==Err(Error::PaletteIndexOutOfRange));
    r::scan_step(raw_image_palette_index_out_of_range_d2(),a_image_palette_index_out_of_range_d2(),1nat);
    r::pass_step(raw_image_palette_index_out_of_range_d2(),a_image_palette_index_out_of_range_d2(),0nat,1nat);
    r::pass_step(raw_image_palette_index_out_of_range_d2(),a_image_palette_index_out_of_range_d2(),0nat,0nat);
    r::scan_step(raw_image_palette_index_out_of_range_d2(),a_image_palette_index_out_of_range_d2(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_image_palette_index_out_of_range_d2() ensures r::still(bytes_image_palette_index_out_of_range_d2(),STRICT,false)==r::Projection::Error(Error::PaletteIndexOutOfRange),
{
    hide(bytes_image_palette_index_out_of_range_d2);hide(z_image_palette_index_out_of_range_d2);hide(raw_image_palette_index_out_of_range_d2);hide(r::data);
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    assert(bytes_image_palette_index_out_of_range_d2().len()==90nat) by (compute);
    assert(crate::model::mh(bytes_image_palette_index_out_of_range_d2())==Ok((h_image_palette_index_out_of_range_d2(),33nat))) by {reveal(crate::model::mh);assert(crate::model::mh(bytes_image_palette_index_out_of_range_d2())==Ok((h_image_palette_index_out_of_range_d2(),33nat))) by (compute);}
        assert((1u32 << 2u8)==4u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_image_palette_index_out_of_range_d2(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:9nat,next:54nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_image_palette_index_out_of_range_d2(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:9nat,next:54nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((80u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![80u8,76u8,84u8,69u8])==crate::walk_model::Name::Plte) by (compute);
        }
        assert(crate::metadata_model::palette(9nat,3u8,2u8,false,false,false,false)==crate::metadata_model::Decision::Keep(3nat)) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_image_palette_index_out_of_range_d2(),54nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:62nat,len:12nat,next:78nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_image_palette_index_out_of_range_d2(),54nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:62nat,len:12nat,next:78nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_image_palette_index_out_of_range_d2(),78nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:86nat,len:0nat,next:90nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_image_palette_index_out_of_range_d2(),78nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:86nat,len:0nat,next:90nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    assert(bytes_image_palette_index_out_of_range_d2().subrange(62,74)==z_image_palette_index_out_of_range_d2()) by (compute);
    walk_image_palette_index_out_of_range_d2(bytes_image_palette_index_out_of_range_d2());
    inflate_image_palette_index_out_of_range_d2();scan_image_palette_index_out_of_range_d2();
    assert(r::raw(a_image_palette_index_out_of_range_d2())==4nat) by (compute);
    assert(r::data(z_image_palette_index_out_of_range_d2(),a_image_palette_index_out_of_range_d2(),STRICT)==Some(Err(Error::PaletteIndexOutOfRange))) by {reveal(r::data);};
    r::default_error(bytes_image_palette_index_out_of_range_d2(),h_image_palette_index_out_of_range_d2(),z_image_palette_index_out_of_range_d2(),3nat,Error::PaletteIndexOutOfRange);
}
fn public_image_palette_index_out_of_range_d2() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,2u8,2u8,3u8,0u8,0u8,0u8,224u8,26u8,142u8,137u8,0u8,0u8,0u8,9u8,80u8,76u8,84u8,69u8,135u8,87u8,218u8,230u8,114u8,114u8,211u8,133u8,142u8,144u8,5u8,72u8,245u8,0u8,0u8,0u8,12u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,144u8,97u8,100u8,0u8,0u8,0u8,90u8,0u8,30u8,179u8,46u8,209u8,13u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_image_palette_index_out_of_range_d2());model_image_palette_index_out_of_range_d2();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Err(crate::alloc_guard::TryError::OutOfMemory(_))=>true,_=>result==Err(crate::alloc_guard::TryError::Format(Error::PaletteIndexOutOfRange))});
}
// plte/suggested-palette-in-rgb; frozen SHA256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_plte_suggested_palette_in_rgb() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,2u8,88u8,80u8,76u8,84u8,69u8,217u8,5u8,126u8,229u8,40u8,28u8,92u8,184u8,223u8,213u8,121u8,238u8,109u8,36u8,77u8,4u8,167u8,27u8,166u8,48u8,111u8,49u8,91u8,221u8,89u8,53u8,190u8,63u8,235u8,76u8,90u8,66u8,129u8,96u8,87u8,99u8,39u8,127u8,211u8,1u8,24u8,51u8,79u8,48u8,95u8,25u8,251u8,236u8,141u8,58u8,166u8,39u8,245u8,176u8,238u8,21u8,176u8,218u8,3u8,95u8,44u8,200u8,190u8,150u8,158u8,3u8,203u8,176u8,192u8,177u8,86u8,198u8,222u8,55u8,22u8,49u8,216u8,137u8,191u8,32u8,64u8,11u8,236u8,238u8,221u8,93u8,106u8,153u8,119u8,70u8,160u8,70u8,148u8,150u8,144u8,89u8,193u8,82u8,205u8,5u8,90u8,208u8,139u8,82u8,184u8,18u8,158u8,37u8,15u8,12u8,185u8,71u8,81u8,180u8,11u8,151u8,95u8,249u8,148u8,227u8,205u8,172u8,235u8,189u8,229u8,52u8,2u8,44u8,166u8,182u8,186u8,227u8,37u8,103u8,130u8,94u8,104u8,184u8,196u8,230u8,13u8,127u8,235u8,164u8,217u8,129u8,6u8,224u8,233u8,236u8,72u8,246u8,169u8,246u8,62u8,80u8,88u8,219u8,152u8,231u8,189u8,105u8,160u8,27u8,215u8,164u8,173u8,35u8,140u8,75u8,214u8,92u8,33u8,84u8,15u8,83u8,66u8,220u8,58u8,109u8,174u8,187u8,145u8,26u8,93u8,11u8,244u8,15u8,150u8,157u8,165u8,108u8,14u8,34u8,101u8,70u8,42u8,210u8,68u8,224u8,234u8,58u8,144u8,7u8,8u8,200u8,112u8,114u8,9u8,227u8,95u8,168u8,194u8,121u8,243u8,15u8,246u8,20u8,171u8,24u8,54u8,128u8,113u8,96u8,45u8,80u8,75u8,94u8,243u8,169u8,254u8,241u8,217u8,214u8,102u8,180u8,124u8,54u8,47u8,156u8,179u8,173u8,102u8,209u8,90u8,28u8,66u8,17u8,60u8,126u8,113u8,193u8,193u8,66u8,150u8,139u8,200u8,94u8,162u8,111u8,143u8,200u8,56u8,253u8,248u8,90u8,238u8,150u8,233u8,78u8,42u8,134u8,186u8,216u8,70u8,219u8,58u8,151u8,73u8,56u8,238u8,38u8,222u8,47u8,178u8,181u8,4u8,136u8,214u8,241u8,102u8,191u8,39u8,49u8,55u8,195u8,188u8,50u8,105u8,210u8,89u8,27u8,49u8,27u8,141u8,39u8,245u8,231u8,35u8,162u8,106u8,53u8,66u8,44u8,203u8,2u8,112u8,205u8,234u8,232u8,133u8,0u8,87u8,145u8,15u8,0u8,125u8,215u8,173u8,105u8,7u8,91u8,227u8,195u8,221u8,8u8,191u8,160u8,132u8,214u8,106u8,143u8,215u8,2u8,98u8,88u8,220u8,115u8,213u8,193u8,166u8,32u8,233u8,198u8,77u8,22u8,163u8,234u8,97u8,166u8,220u8,183u8,61u8,91u8,16u8,231u8,70u8,168u8,57u8,36u8,111u8,154u8,180u8,231u8,115u8,221u8,117u8,210u8,228u8,244u8,152u8,199u8,195u8,219u8,60u8,15u8,195u8,114u8,200u8,222u8,241u8,5u8,154u8,61u8,251u8,237u8,87u8,7u8,200u8,90u8,30u8,146u8,116u8,162u8,211u8,33u8,75u8,247u8,91u8,245u8,163u8,252u8,212u8,218u8,231u8,68u8,20u8,108u8,86u8,62u8,179u8,124u8,34u8,164u8,253u8,8u8,165u8,157u8,17u8,198u8,157u8,168u8,130u8,98u8,106u8,84u8,83u8,32u8,16u8,240u8,55u8,94u8,26u8,86u8,170u8,69u8,120u8,114u8,244u8,241u8,223u8,14u8,212u8,227u8,219u8,218u8,82u8,90u8,212u8,102u8,200u8,191u8,175u8,62u8,1u8,118u8,236u8,85u8,111u8,228u8,181u8,36u8,156u8,216u8,90u8,236u8,19u8,246u8,19u8,182u8,40u8,196u8,221u8,94u8,247u8,11u8,241u8,38u8,236u8,90u8,45u8,63u8,248u8,237u8,144u8,82u8,35u8,143u8,89u8,30u8,100u8,154u8,194u8,162u8,16u8,140u8,82u8,88u8,69u8,53u8,106u8,170u8,117u8,126u8,213u8,72u8,33u8,243u8,176u8,253u8,209u8,23u8,227u8,0u8,194u8,26u8,251u8,229u8,54u8,130u8,184u8,87u8,126u8,228u8,255u8,39u8,216u8,35u8,81u8,111u8,215u8,222u8,86u8,250u8,173u8,226u8,86u8,44u8,120u8,253u8,66u8,1u8,217u8,1u8,43u8,145u8,75u8,132u8,77u8,196u8,237u8,112u8,162u8,123u8,156u8,80u8,214u8,90u8,240u8,154u8,205u8,49u8,148u8,210u8,253u8,245u8,117u8,255u8,99u8,48u8,82u8,20u8,48u8,201u8,131u8,237u8,156u8,171u8,200u8,8u8,88u8,124u8,80u8,131u8,125u8,108u8,63u8,37u8,245u8,67u8,214u8,20u8,107u8,218u8,0u8,0u8,0u8,6u8,98u8,75u8,71u8,68u8,0u8,1u8,0u8,2u8,0u8,3u8,94u8,174u8,23u8,27u8,0u8,0u8,1u8,144u8,104u8,73u8,83u8,84u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,93u8,124u8,21u8,200u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn h_plte_suggested_palette_in_rgb() -> crate::model::Ihdr {crate::model::Ihdr {width:2u32,height:2u32,depth:8u8,color_type:2u8,interlace:0u8}}
spec fn a_plte_suggested_palette_in_rgb() -> r::Raster {r::shape(h_plte_suggested_palette_in_rgb(),2nat,2nat,200nat,STRICT)}
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
        crate::metadata_model::words16(b.subrange(653,659),3nat)=~=seq![1u16,2u16,3u16],
        crate::metadata_model::parse(crate::walk_model::Name::Bkgd,b.subrange(653,659),2u8,8u8,200nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Bkgd(seq![1u16,2u16,3u16])),
        crate::metadata_model::parse(crate::walk_model::Name::Hist,b.subrange(671,1071),2u8,8u8,200nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored),
        b.subrange(1083,1105)==z_plte_suggested_palette_in_rgb(),
    ensures r::default_case(b,h_plte_suggested_palette_in_rgb(),z_plte_suggested_palette_in_rgb(),200nat),
{
    hide(m::drive);hide(crate::critical_model::read);hide(crate::metadata_model::parse);hide(crate::model::mh);
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
proof fn model_plte_suggested_palette_in_rgb() ensures r::still(bytes_plte_suggested_palette_in_rgb(),STRICT,false)==(r::Projection::Ready {width:2u32,height:2u32,bytes:16nat}),
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
        assert(crate::metadata_model::words16(bytes_plte_suggested_palette_in_rgb().subrange(653,659),3nat)=~=seq![1u16,2u16,3u16]);
        assert(crate::metadata_model::parse(crate::walk_model::Name::Bkgd,bytes_plte_suggested_palette_in_rgb().subrange(653,659),2u8,8u8,200nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Bkgd(seq![1u16,2u16,3u16]))) by {bkgd_rgb_certificate(bytes_plte_suggested_palette_in_rgb().subrange(653,659));};
        assert(crate::metadata_model::parse(crate::walk_model::Name::Hist,bytes_plte_suggested_palette_in_rgb().subrange(671,1071),2u8,8u8,200nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored)) by {reveal(crate::metadata_model::parse);};
    assert(bytes_plte_suggested_palette_in_rgb().subrange(1083,1105)==z_plte_suggested_palette_in_rgb()) by (compute);
    walk_plte_suggested_palette_in_rgb(bytes_plte_suggested_palette_in_rgb());
    inflate_plte_suggested_palette_in_rgb();scan_plte_suggested_palette_in_rgb();
    assert(r::raw(a_plte_suggested_palette_in_rgb())==14nat) by (compute);
    assert(r::data(z_plte_suggested_palette_in_rgb(),a_plte_suggested_palette_in_rgb(),STRICT)==Some(Ok(()))) by {reveal(r::data);};
    r::default_ready(bytes_plte_suggested_palette_in_rgb(),h_plte_suggested_palette_in_rgb(),z_plte_suggested_palette_in_rgb(),200nat,false);
}
fn public_plte_suggested_palette_in_rgb() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,2u8,88u8,80u8,76u8,84u8,69u8,217u8,5u8,126u8,229u8,40u8,28u8,92u8,184u8,223u8,213u8,121u8,238u8,109u8,36u8,77u8,4u8,167u8,27u8,166u8,48u8,111u8,49u8,91u8,221u8,89u8,53u8,190u8,63u8,235u8,76u8,90u8,66u8,129u8,96u8,87u8,99u8,39u8,127u8,211u8,1u8,24u8,51u8,79u8,48u8,95u8,25u8,251u8,236u8,141u8,58u8,166u8,39u8,245u8,176u8,238u8,21u8,176u8,218u8,3u8,95u8,44u8,200u8,190u8,150u8,158u8,3u8,203u8,176u8,192u8,177u8,86u8,198u8,222u8,55u8,22u8,49u8,216u8,137u8,191u8,32u8,64u8,11u8,236u8,238u8,221u8,93u8,106u8,153u8,119u8,70u8,160u8,70u8,148u8,150u8,144u8,89u8,193u8,82u8,205u8,5u8,90u8,208u8,139u8,82u8,184u8,18u8,158u8,37u8,15u8,12u8,185u8,71u8,81u8,180u8,11u8,151u8,95u8,249u8,148u8,227u8,205u8,172u8,235u8,189u8,229u8,52u8,2u8,44u8,166u8,182u8,186u8,227u8,37u8,103u8,130u8,94u8,104u8,184u8,196u8,230u8,13u8,127u8,235u8,164u8,217u8,129u8,6u8,224u8,233u8,236u8,72u8,246u8,169u8,246u8,62u8,80u8,88u8,219u8,152u8,231u8,189u8,105u8,160u8,27u8,215u8,164u8,173u8,35u8,140u8,75u8,214u8,92u8,33u8,84u8,15u8,83u8,66u8,220u8,58u8,109u8,174u8,187u8,145u8,26u8,93u8,11u8,244u8,15u8,150u8,157u8,165u8,108u8,14u8,34u8,101u8,70u8,42u8,210u8,68u8,224u8,234u8,58u8,144u8,7u8,8u8,200u8,112u8,114u8,9u8,227u8,95u8,168u8,194u8,121u8,243u8,15u8,246u8,20u8,171u8,24u8,54u8,128u8,113u8,96u8,45u8,80u8,75u8,94u8,243u8,169u8,254u8,241u8,217u8,214u8,102u8,180u8,124u8,54u8,47u8,156u8,179u8,173u8,102u8,209u8,90u8,28u8,66u8,17u8,60u8,126u8,113u8,193u8,193u8,66u8,150u8,139u8,200u8,94u8,162u8,111u8,143u8,200u8,56u8,253u8,248u8,90u8,238u8,150u8,233u8,78u8,42u8,134u8,186u8,216u8,70u8,219u8,58u8,151u8,73u8,56u8,238u8,38u8,222u8,47u8,178u8,181u8,4u8,136u8,214u8,241u8,102u8,191u8,39u8,49u8,55u8,195u8,188u8,50u8,105u8,210u8,89u8,27u8,49u8,27u8,141u8,39u8,245u8,231u8,35u8,162u8,106u8,53u8,66u8,44u8,203u8,2u8,112u8,205u8,234u8,232u8,133u8,0u8,87u8,145u8,15u8,0u8,125u8,215u8,173u8,105u8,7u8,91u8,227u8,195u8,221u8,8u8,191u8,160u8,132u8,214u8,106u8,143u8,215u8,2u8,98u8,88u8,220u8,115u8,213u8,193u8,166u8,32u8,233u8,198u8,77u8,22u8,163u8,234u8,97u8,166u8,220u8,183u8,61u8,91u8,16u8,231u8,70u8,168u8,57u8,36u8,111u8,154u8,180u8,231u8,115u8,221u8,117u8,210u8,228u8,244u8,152u8,199u8,195u8,219u8,60u8,15u8,195u8,114u8,200u8,222u8,241u8,5u8,154u8,61u8,251u8,237u8,87u8,7u8,200u8,90u8,30u8,146u8,116u8,162u8,211u8,33u8,75u8,247u8,91u8,245u8,163u8,252u8,212u8,218u8,231u8,68u8,20u8,108u8,86u8,62u8,179u8,124u8,34u8,164u8,253u8,8u8,165u8,157u8,17u8,198u8,157u8,168u8,130u8,98u8,106u8,84u8,83u8,32u8,16u8,240u8,55u8,94u8,26u8,86u8,170u8,69u8,120u8,114u8,244u8,241u8,223u8,14u8,212u8,227u8,219u8,218u8,82u8,90u8,212u8,102u8,200u8,191u8,175u8,62u8,1u8,118u8,236u8,85u8,111u8,228u8,181u8,36u8,156u8,216u8,90u8,236u8,19u8,246u8,19u8,182u8,40u8,196u8,221u8,94u8,247u8,11u8,241u8,38u8,236u8,90u8,45u8,63u8,248u8,237u8,144u8,82u8,35u8,143u8,89u8,30u8,100u8,154u8,194u8,162u8,16u8,140u8,82u8,88u8,69u8,53u8,106u8,170u8,117u8,126u8,213u8,72u8,33u8,243u8,176u8,253u8,209u8,23u8,227u8,0u8,194u8,26u8,251u8,229u8,54u8,130u8,184u8,87u8,126u8,228u8,255u8,39u8,216u8,35u8,81u8,111u8,215u8,222u8,86u8,250u8,173u8,226u8,86u8,44u8,120u8,253u8,66u8,1u8,217u8,1u8,43u8,145u8,75u8,132u8,77u8,196u8,237u8,112u8,162u8,123u8,156u8,80u8,214u8,90u8,240u8,154u8,205u8,49u8,148u8,210u8,253u8,245u8,117u8,255u8,99u8,48u8,82u8,20u8,48u8,201u8,131u8,237u8,156u8,171u8,200u8,8u8,88u8,124u8,80u8,131u8,125u8,108u8,63u8,37u8,245u8,67u8,214u8,20u8,107u8,218u8,0u8,0u8,0u8,6u8,98u8,75u8,71u8,68u8,0u8,1u8,0u8,2u8,0u8,3u8,94u8,174u8,23u8,27u8,0u8,0u8,1u8,144u8,104u8,73u8,83u8,84u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,93u8,124u8,21u8,200u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_plte_suggested_palette_in_rgb());model_plte_suggested_palette_in_rgb();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==2u32 && s.height==2u32 && s.rgba@.len()==16nat,Err(crate::alloc_guard::TryError::OutOfMemory(_))=>true,_=>false});
}
// apng/still-is-default-image; frozen SHA256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_apng_still_is_default_image() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,2u8,32u8,227u8,219u8,236u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,5u8,0u8,0u8,0u8,0u8,52u8,100u8,47u8,237u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,0u8,1u8,0u8,10u8,1u8,0u8,13u8,197u8,211u8,225u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,2u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,85u8,246u8,198u8,140u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,3u8,0u8,100u8,0u8,1u8,192u8,66u8,10u8,152u8,0u8,0u8,0u8,7u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,4u8,120u8,156u8,99u8,207u8,160u8,248u8,174u8,0u8,0u8,0u8,15u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,5u8,96u8,96u8,248u8,223u8,0u8,194u8,0u8,11u8,1u8,2u8,255u8,120u8,231u8,82u8,174u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn h_apng_still_is_default_image() -> crate::model::Ihdr {crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:6u8,interlace:0u8}}
spec fn a_apng_still_is_default_image() -> r::Raster {r::shape(h_apng_still_is_default_image(),4nat,3nat,0nat,STRICT)}
spec fn z_apng_still_is_default_image() -> Seq<u8> {seq![120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8]}
spec fn raw_apng_still_is_default_image() -> Seq<u8> {seq![0u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,1u8,255u8,0u8,0u8,255u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8]}
spec fn ts_apng_still_is_default_image() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:36nat,before:1nat,after:2nat,symbol:255nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:36nat,next:44nat,before:2nat,after:3nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:44nat,next:52nat,before:3nat,after:4nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:52nat,next:61nat,before:4nat,after:5nat,symbol:255nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:61nat,next:74nat,before:5nat,after:17nat,symbol:265nat,le:1nat,ds:3nat,de:0nat,length:12nat,distance:4nat},t::Step {pos:74nat,next:82nat,before:17nat,after:18nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:82nat,next:95nat,before:18nat,after:22nat,symbol:258nat,le:0nat,ds:4nat,de:0nat,length:4nat,distance:5nat},t::Step {pos:95nat,next:103nat,before:22nat,after:23nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:103nat,next:116nat,before:23nat,after:34nat,symbol:265nat,le:0nat,ds:0nat,de:0nat,length:11nat,distance:1nat},t::Step {pos:116nat,next:124nat,before:34nat,after:35nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:124nat,next:132nat,before:35nat,after:36nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:132nat,next:145nat,before:36nat,after:51nat,symbol:267nat,le:0nat,ds:0nat,de:0nat,length:15nat,distance:1nat},t::Step {pos:145nat,next:152nat,before:51nat,after:51nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_apng_still_is_default_image_0() ensures t::trace(z_apng_still_is_default_image(),raw_apng_still_is_default_image(),ts_apng_still_is_default_image(),0nat,14nat),
{
    hide(t::valid_step);assert(t::trace(z_apng_still_is_default_image(),raw_apng_still_is_default_image(),ts_apng_still_is_default_image(),0nat,14nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn trace_apng_still_is_default_image() ensures t::trace(z_apng_still_is_default_image(),raw_apng_still_is_default_image(),ts_apng_still_is_default_image(),0,14nat),
{
    hide(z_apng_still_is_default_image);hide(raw_apng_still_is_default_image);hide(ts_apng_still_is_default_image);hide(t::trace);
    trace_apng_still_is_default_image_0();
    assert(t::trace(z_apng_still_is_default_image(),raw_apng_still_is_default_image(),ts_apng_still_is_default_image(),0,14nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_apng_still_is_default_image() ensures z_apng_still_is_default_image().len()==23nat,
        raw_apng_still_is_default_image().len()==51nat,
        z_apng_still_is_default_image()[0]==120u8,
        z_apng_still_is_default_image()[1]==156u8,
        zm::field_value(z_apng_still_is_default_image(),16,1)==1nat,
        zm::field_value(z_apng_still_is_default_image(),17,2)==1nat,
        zm::be_value(z_apng_still_is_default_image(),19nat,4)==2448624122nat,
        ts_apng_still_is_default_image().len()==14nat,
        ts_apng_still_is_default_image()[0].pos==19nat,
        ts_apng_still_is_default_image()[0].before==0nat,
        ts_apng_still_is_default_image().last().next==152nat,
{
    assert(z_apng_still_is_default_image().len()==23nat) by (compute);
    assert(raw_apng_still_is_default_image().len()==51nat) by (compute);
    assert(z_apng_still_is_default_image()[0]==120u8) by (compute);
    assert(z_apng_still_is_default_image()[1]==156u8) by (compute);
    assert(zm::field_value(z_apng_still_is_default_image(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_apng_still_is_default_image(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_apng_still_is_default_image(),19nat,4)==2448624122nat) by (compute);
    assert(ts_apng_still_is_default_image().len()==14nat) by (compute);
    assert(ts_apng_still_is_default_image()[0].pos==19nat) by (compute);
    assert(ts_apng_still_is_default_image()[0].before==0nat) by (compute);
    assert(ts_apng_still_is_default_image().last().next==152nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_apng_still_is_default_image() ensures zm::Adler(raw_apng_still_is_default_image())==2448624122nat,
{
    hide(raw_apng_still_is_default_image);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_apng_still_is_default_image().len()==51nat) by (compute);
    assert(c::sums(raw_apng_still_is_default_image(),0,51nat)==(2553nat,102884nat)) by (compute);
    c::sums_correct(raw_apng_still_is_default_image(),0,51nat);assert(raw_apng_still_is_default_image().subrange(0,51)=~=raw_apng_still_is_default_image());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((102884nat%65521)*(u16::MAX as nat+1)+(1+2553nat)%65521==2448624122nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_apng_still_is_default_image() ensures zm::Inflate(z_apng_still_is_default_image(),51nat,true,false)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_apng_still_is_default_image(),r2:false}),raw_apng_still_is_default_image().len()==51nat,
{
    hide(z_apng_still_is_default_image);hide(raw_apng_still_is_default_image);hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);hide(ts_apng_still_is_default_image);
    shape_apng_still_is_default_image();checksum_apng_still_is_default_image();
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_apng_still_is_default_image();ms::fixed_stream(z_apng_still_is_default_image(),raw_apng_still_is_default_image(),ts_apng_still_is_default_image(),51nat,156nat,2448624122nat,2448624122nat);
}
spec fn split_apng_still_is_default_image_state_0(h: crate::model::Ihdr) -> m::State {m::initial(h,33,m::no_events())}
spec fn split_apng_still_is_default_image_state_1(h: crate::model::Ihdr) -> m::State {
    let s=m::initial(h,33,m::no_events());
    m::State {pos:53nat,seen:set![crate::walk_model::Name::Ihdr,crate::walk_model::Name::Actl],phase:crate::walk_model::Phase::Before,control:crate::apng_model::initial(),
        doc:m::Doc {actl:Some((3u32,2u32)),info:s.doc.info,idat:seq![],frames:seq![],..s.doc},..s}
}
#[verifier::spinoff_prover]
proof fn split_apng_still_is_default_image_step_0(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==292),
        (h==(crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false}))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::apng_model::actl(b.subrange(41,49))==Ok((3u32,2u32))),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:6nat,current:crate::apng_model::Current::Frame(true),frame_count:3nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)+(2nat*1nat)},Some(3u32))==Ok(())),
    ensures m::step(b,m::strict(),false,false,split_apng_still_is_default_image_state_0(h))==m::Step::Next(split_apng_still_is_default_image_state_1(h)),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    let actual=match m::step(b,m::strict(),false,false,split_apng_still_is_default_image_state_0(h)) {m::Step::Next(next)=>next,_=>split_apng_still_is_default_image_state_0(h)};
    assert(m::step(b,m::strict(),false,false,split_apng_still_is_default_image_state_0(h))==m::Step::Next(actual));
    let expected=split_apng_still_is_default_image_state_1(h);
    assert(actual.pos==expected.pos);
    assert(actual.phase==expected.phase);
    assert(actual.control==expected.control);
    assert(actual.doc.info==expected.doc.info);
    assert forall|i: int| 0<=i<expected.doc.frames.len() implies #[trigger] actual.doc.frames[i]==expected.doc.frames[i] by {
        match (actual.doc.frames[i].source,expected.doc.frames[i].source) {
            (crate::walk_model::Source::Fdat(a),crate::walk_model::Source::Fdat(e))=>{assert(a=~=e);},_=>{}
        }
    }
    assert(actual.doc.frames =~= expected.doc.frames);
    assert(actual.doc.idat =~= expected.doc.idat);
}
spec fn split_apng_still_is_default_image_state_2(h: crate::model::Ihdr) -> m::State {
    let s=m::initial(h,33,m::no_events());
    m::State {pos:91nat,seen:set![crate::walk_model::Name::Ihdr,crate::walk_model::Name::Actl],phase:crate::walk_model::Phase::Before,control:crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)},
        doc:m::Doc {actl:Some((3u32,2u32)),info:s.doc.info,idat:seq![],frames:seq![crate::walk_model::Frame {control_offset:53nat,width:4u32,height:3u32,x:0u32,y:0u32,delay_num:5u16,delay_den:0u16,dispose:0u8,blend:0u8,source:crate::walk_model::Source::Idat}],..s.doc},..s}
}
#[verifier::spinoff_prover]
proof fn split_apng_still_is_default_image_step_1(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==292),
        (h==(crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:26nat,next:91nat,crc_ok:true,synthetic:false}))),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::apng_model::fctl(b.subrange(61,87),true,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Ok((crate::apng_model::FrameControl {width:4u32,height:3u32,x:0u32,y:0u32,delay_num:5u16,delay_den:0u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}))),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:6nat,current:crate::apng_model::Current::Frame(true),frame_count:3nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)+(2nat*1nat)},Some(3u32))==Ok(())),
    ensures m::step(b,m::strict(),false,false,split_apng_still_is_default_image_state_1(h))==m::Step::Next(split_apng_still_is_default_image_state_2(h)),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::fctl);hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    let actual=match m::step(b,m::strict(),false,false,split_apng_still_is_default_image_state_1(h)) {m::Step::Next(next)=>next,_=>split_apng_still_is_default_image_state_1(h)};
    assert(m::step(b,m::strict(),false,false,split_apng_still_is_default_image_state_1(h))==m::Step::Next(actual));
    let expected=split_apng_still_is_default_image_state_2(h);
    assert(actual.pos==expected.pos);
    assert(actual.phase==expected.phase);
    assert(actual.control==expected.control);
    assert(actual.doc.info==expected.doc.info);
    assert forall|i: int| 0<=i<expected.doc.frames.len() implies #[trigger] actual.doc.frames[i]==expected.doc.frames[i] by {
        match (actual.doc.frames[i].source,expected.doc.frames[i].source) {
            (crate::walk_model::Source::Fdat(a),crate::walk_model::Source::Fdat(e))=>{assert(a=~=e);},_=>{}
        }
    }
    assert(actual.doc.frames =~= expected.doc.frames);
    assert(actual.doc.idat =~= expected.doc.idat);
}
spec fn split_apng_still_is_default_image_state_3(h: crate::model::Ihdr) -> m::State {
    let s=m::initial(h,33,m::no_events());
    m::State {pos:126nat,seen:set![crate::walk_model::Name::Ihdr,crate::walk_model::Name::Actl],phase:crate::walk_model::Phase::During,control:crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)},
        doc:m::Doc {actl:Some((3u32,2u32)),info:m::Info {default_is_frame:true,..s.doc.info},idat:seq![crate::walk_model::Range {offset:99nat,len:23nat}],frames:seq![crate::walk_model::Frame {control_offset:53nat,width:4u32,height:3u32,x:0u32,y:0u32,delay_num:5u16,delay_den:0u16,dispose:0u8,blend:0u8,source:crate::walk_model::Source::Idat}],..s.doc},..s}
}
#[verifier::spinoff_prover]
proof fn split_apng_still_is_default_image_step_2(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==292),
        (h==(crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:99nat,len:23nat,next:126nat,crc_ok:true,synthetic:false}))),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:6nat,current:crate::apng_model::Current::Frame(true),frame_count:3nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)+(2nat*1nat)},Some(3u32))==Ok(())),
    ensures m::step(b,m::strict(),false,false,split_apng_still_is_default_image_state_2(h))==m::Step::Next(split_apng_still_is_default_image_state_3(h)),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    let actual=match m::step(b,m::strict(),false,false,split_apng_still_is_default_image_state_2(h)) {m::Step::Next(next)=>next,_=>split_apng_still_is_default_image_state_2(h)};
    assert(m::step(b,m::strict(),false,false,split_apng_still_is_default_image_state_2(h))==m::Step::Next(actual));
    let expected=split_apng_still_is_default_image_state_3(h);
    assert(actual.pos==expected.pos);
    assert(actual.phase==expected.phase);
    assert(actual.control==expected.control);
    assert(actual.doc.info==expected.doc.info);
    assert forall|i: int| 0<=i<expected.doc.frames.len() implies #[trigger] actual.doc.frames[i]==expected.doc.frames[i] by {
        match (actual.doc.frames[i].source,expected.doc.frames[i].source) {
            (crate::walk_model::Source::Fdat(a),crate::walk_model::Source::Fdat(e))=>{assert(a=~=e);},_=>{}
        }
    }
    assert(actual.doc.frames =~= expected.doc.frames);
    assert(actual.doc.idat =~= expected.doc.idat);
}
spec fn split_apng_still_is_default_image_state_4(h: crate::model::Ihdr) -> m::State {
    let s=m::initial(h,33,m::no_events());
    m::State {pos:164nat,seen:set![crate::walk_model::Name::Ihdr,crate::walk_model::Name::Actl],phase:crate::walk_model::Phase::After,control:crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)},
        doc:m::Doc {actl:Some((3u32,2u32)),info:m::Info {default_is_frame:true,..s.doc.info},idat:seq![crate::walk_model::Range {offset:99nat,len:23nat}],frames:seq![crate::walk_model::Frame {control_offset:53nat,width:4u32,height:3u32,x:0u32,y:0u32,delay_num:5u16,delay_den:0u16,dispose:0u8,blend:0u8,source:crate::walk_model::Source::Idat},crate::walk_model::Frame {control_offset:126nat,width:2u32,height:2u32,x:1u32,y:1u32,delay_num:1u16,delay_den:10u16,dispose:1u8,blend:0u8,source:crate::walk_model::Source::Fdat(seq![])}],..s.doc},..s}
}
#[verifier::spinoff_prover]
proof fn split_apng_still_is_default_image_step_3(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==292),
        (h==(crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:134nat,len:26nat,next:164nat,crc_ok:true,synthetic:false}))),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}),
        (crate::apng_model::fctl(b.subrange(134,160),true,false,4u32,3u32,crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==Ok((crate::apng_model::FrameControl {width:2u32,height:2u32,x:1u32,y:1u32,delay_num:1u16,delay_den:10u16,dispose:1u8,blend:0u8},crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)}))),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:6nat,current:crate::apng_model::Current::Frame(true),frame_count:3nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)+(2nat*1nat)},Some(3u32))==Ok(())),
    ensures m::step(b,m::strict(),false,false,split_apng_still_is_default_image_state_3(h))==m::Step::Next(split_apng_still_is_default_image_state_4(h)),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::fctl);hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    let actual=match m::step(b,m::strict(),false,false,split_apng_still_is_default_image_state_3(h)) {m::Step::Next(next)=>next,_=>split_apng_still_is_default_image_state_3(h)};
    assert(m::step(b,m::strict(),false,false,split_apng_still_is_default_image_state_3(h))==m::Step::Next(actual));
    let expected=split_apng_still_is_default_image_state_4(h);
    assert(actual.pos==expected.pos);
    assert(actual.phase==expected.phase);
    assert(actual.control==expected.control);
    assert(actual.doc.info==expected.doc.info);
    assert forall|i: int| 0<=i<expected.doc.frames.len() implies #[trigger] actual.doc.frames[i]==expected.doc.frames[i] by {
        match (actual.doc.frames[i].source,expected.doc.frames[i].source) {
            (crate::walk_model::Source::Fdat(a),crate::walk_model::Source::Fdat(e))=>{assert(a=~=e);},_=>{}
        }
    }
    assert(actual.doc.frames =~= expected.doc.frames);
    assert(actual.doc.idat =~= expected.doc.idat);
}
spec fn split_apng_still_is_default_image_state_5(h: crate::model::Ihdr) -> m::State {
    let s=m::initial(h,33,m::no_events());
    m::State {pos:196nat,seen:set![crate::walk_model::Name::Ihdr,crate::walk_model::Name::Actl],phase:crate::walk_model::Phase::After,control:crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)},
        doc:m::Doc {actl:Some((3u32,2u32)),info:m::Info {default_is_frame:true,..s.doc.info},idat:seq![crate::walk_model::Range {offset:99nat,len:23nat}],frames:seq![crate::walk_model::Frame {control_offset:53nat,width:4u32,height:3u32,x:0u32,y:0u32,delay_num:5u16,delay_den:0u16,dispose:0u8,blend:0u8,source:crate::walk_model::Source::Idat},crate::walk_model::Frame {control_offset:126nat,width:2u32,height:2u32,x:1u32,y:1u32,delay_num:1u16,delay_den:10u16,dispose:1u8,blend:0u8,source:crate::walk_model::Source::Fdat(seq![crate::walk_model::Range {offset:176nat,len:16nat}])}],..s.doc},..s}
}
#[verifier::spinoff_prover]
proof fn split_apng_still_is_default_image_step_4(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==292),
        (h==(crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,164nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:172nat,len:20nat,next:196nat,crc_ok:true,synthetic:false}))),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}),
        (crate::apng_model::fdat(b.subrange(172,192),true,false,crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)})==Ok(crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)})),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:6nat,current:crate::apng_model::Current::Frame(true),frame_count:3nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)+(2nat*1nat)},Some(3u32))==Ok(())),
    ensures m::step(b,m::strict(),false,false,split_apng_still_is_default_image_state_4(h))==m::Step::Next(split_apng_still_is_default_image_state_5(h)),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::fdat);hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    let actual=match m::step(b,m::strict(),false,false,split_apng_still_is_default_image_state_4(h)) {m::Step::Next(next)=>next,_=>split_apng_still_is_default_image_state_4(h)};
    assert(m::step(b,m::strict(),false,false,split_apng_still_is_default_image_state_4(h))==m::Step::Next(actual));
    let expected=split_apng_still_is_default_image_state_5(h);
    assert(actual.pos==expected.pos);
    assert(actual.phase==expected.phase);
    assert(actual.control==expected.control);
    assert(actual.doc.info==expected.doc.info);
    assert forall|i: int| 0<=i<expected.doc.frames.len() implies #[trigger] actual.doc.frames[i]==expected.doc.frames[i] by {
        match (actual.doc.frames[i].source,expected.doc.frames[i].source) {
            (crate::walk_model::Source::Fdat(a),crate::walk_model::Source::Fdat(e))=>{assert(a=~=e);},_=>{}
        }
    }
    assert(actual.doc.frames =~= expected.doc.frames);
    assert(actual.doc.idat =~= expected.doc.idat);
}
spec fn split_apng_still_is_default_image_state_6(h: crate::model::Ihdr) -> m::State {
    let s=m::initial(h,33,m::no_events());
    m::State {pos:234nat,seen:set![crate::walk_model::Name::Ihdr,crate::walk_model::Name::Actl],phase:crate::walk_model::Phase::After,control:crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(false),frame_count:3nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)+(2nat*1nat)},
        doc:m::Doc {actl:Some((3u32,2u32)),info:m::Info {default_is_frame:true,..s.doc.info},idat:seq![crate::walk_model::Range {offset:99nat,len:23nat}],frames:seq![crate::walk_model::Frame {control_offset:53nat,width:4u32,height:3u32,x:0u32,y:0u32,delay_num:5u16,delay_den:0u16,dispose:0u8,blend:0u8,source:crate::walk_model::Source::Idat},crate::walk_model::Frame {control_offset:126nat,width:2u32,height:2u32,x:1u32,y:1u32,delay_num:1u16,delay_den:10u16,dispose:1u8,blend:0u8,source:crate::walk_model::Source::Fdat(seq![crate::walk_model::Range {offset:176nat,len:16nat}])},crate::walk_model::Frame {control_offset:196nat,width:2u32,height:1u32,x:0u32,y:0u32,delay_num:3u16,delay_den:100u16,dispose:0u8,blend:1u8,source:crate::walk_model::Source::Fdat(seq![])}],..s.doc},..s}
}
#[verifier::spinoff_prover]
proof fn split_apng_still_is_default_image_step_5(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==292),
        (h==(crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,196nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:204nat,len:26nat,next:234nat,crc_ok:true,synthetic:false}))),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}),
        (crate::apng_model::fctl(b.subrange(204,230),true,false,4u32,3u32,crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)})==Ok((crate::apng_model::FrameControl {width:2u32,height:1u32,x:0u32,y:0u32,delay_num:3u16,delay_den:100u16,dispose:0u8,blend:1u8},crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(false),frame_count:3nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)+(2nat*1nat)}))),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:6nat,current:crate::apng_model::Current::Frame(true),frame_count:3nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)+(2nat*1nat)},Some(3u32))==Ok(())),
    ensures m::step(b,m::strict(),false,false,split_apng_still_is_default_image_state_5(h))==m::Step::Next(split_apng_still_is_default_image_state_6(h)),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::fctl);hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    let actual=match m::step(b,m::strict(),false,false,split_apng_still_is_default_image_state_5(h)) {m::Step::Next(next)=>next,_=>split_apng_still_is_default_image_state_5(h)};
    assert(m::step(b,m::strict(),false,false,split_apng_still_is_default_image_state_5(h))==m::Step::Next(actual));
    let expected=split_apng_still_is_default_image_state_6(h);
    assert(actual.pos==expected.pos);
    assert(actual.phase==expected.phase);
    assert(actual.control==expected.control);
    assert(actual.doc.info==expected.doc.info);
    assert forall|i: int| 0<=i<expected.doc.frames.len() implies #[trigger] actual.doc.frames[i]==expected.doc.frames[i] by {
        match (actual.doc.frames[i].source,expected.doc.frames[i].source) {
            (crate::walk_model::Source::Fdat(a),crate::walk_model::Source::Fdat(e))=>{assert(a=~=e);},_=>{}
        }
    }
    assert(actual.doc.frames =~= expected.doc.frames);
    assert(actual.doc.idat =~= expected.doc.idat);
}
spec fn split_apng_still_is_default_image_state_7(h: crate::model::Ihdr) -> m::State {
    let s=m::initial(h,33,m::no_events());
    m::State {pos:253nat,seen:set![crate::walk_model::Name::Ihdr,crate::walk_model::Name::Actl],phase:crate::walk_model::Phase::After,control:crate::apng_model::Control {sequence:5nat,current:crate::apng_model::Current::Frame(true),frame_count:3nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)+(2nat*1nat)},
        doc:m::Doc {actl:Some((3u32,2u32)),info:m::Info {default_is_frame:true,..s.doc.info},idat:seq![crate::walk_model::Range {offset:99nat,len:23nat}],frames:seq![crate::walk_model::Frame {control_offset:53nat,width:4u32,height:3u32,x:0u32,y:0u32,delay_num:5u16,delay_den:0u16,dispose:0u8,blend:0u8,source:crate::walk_model::Source::Idat},crate::walk_model::Frame {control_offset:126nat,width:2u32,height:2u32,x:1u32,y:1u32,delay_num:1u16,delay_den:10u16,dispose:1u8,blend:0u8,source:crate::walk_model::Source::Fdat(seq![crate::walk_model::Range {offset:176nat,len:16nat}])},crate::walk_model::Frame {control_offset:196nat,width:2u32,height:1u32,x:0u32,y:0u32,delay_num:3u16,delay_den:100u16,dispose:0u8,blend:1u8,source:crate::walk_model::Source::Fdat(seq![crate::walk_model::Range {offset:246nat,len:3nat}])}],..s.doc},..s}
}
#[verifier::spinoff_prover]
proof fn split_apng_still_is_default_image_step_6(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==292),
        (h==(crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,234nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:242nat,len:7nat,next:253nat,crc_ok:true,synthetic:false}))),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}),
        (crate::apng_model::fdat(b.subrange(242,249),true,false,crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(false),frame_count:3nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)+(2nat*1nat)})==Ok(crate::apng_model::Control {sequence:5nat,current:crate::apng_model::Current::Frame(true),frame_count:3nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)+(2nat*1nat)})),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:6nat,current:crate::apng_model::Current::Frame(true),frame_count:3nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)+(2nat*1nat)},Some(3u32))==Ok(())),
    ensures m::step(b,m::strict(),false,false,split_apng_still_is_default_image_state_6(h))==m::Step::Next(split_apng_still_is_default_image_state_7(h)),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::fdat);hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    let actual=match m::step(b,m::strict(),false,false,split_apng_still_is_default_image_state_6(h)) {m::Step::Next(next)=>next,_=>split_apng_still_is_default_image_state_6(h)};
    assert(m::step(b,m::strict(),false,false,split_apng_still_is_default_image_state_6(h))==m::Step::Next(actual));
    let expected=split_apng_still_is_default_image_state_7(h);
    assert(actual.pos==expected.pos);
    assert(actual.phase==expected.phase);
    assert(actual.control==expected.control);
    assert(actual.doc.info==expected.doc.info);
    assert forall|i: int| 0<=i<expected.doc.frames.len() implies #[trigger] actual.doc.frames[i]==expected.doc.frames[i] by {
        match (actual.doc.frames[i].source,expected.doc.frames[i].source) {
            (crate::walk_model::Source::Fdat(a),crate::walk_model::Source::Fdat(e))=>{assert(a=~=e);},_=>{}
        }
    }
    assert(actual.doc.frames =~= expected.doc.frames);
    assert(actual.doc.idat =~= expected.doc.idat);
}
spec fn split_apng_still_is_default_image_state_8(h: crate::model::Ihdr) -> m::State {
    let s=m::initial(h,33,m::no_events());
    m::State {pos:280nat,seen:set![crate::walk_model::Name::Ihdr,crate::walk_model::Name::Actl],phase:crate::walk_model::Phase::After,control:crate::apng_model::Control {sequence:6nat,current:crate::apng_model::Current::Frame(true),frame_count:3nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)+(2nat*1nat)},
        doc:m::Doc {actl:Some((3u32,2u32)),info:m::Info {default_is_frame:true,..s.doc.info},idat:seq![crate::walk_model::Range {offset:99nat,len:23nat}],frames:seq![crate::walk_model::Frame {control_offset:53nat,width:4u32,height:3u32,x:0u32,y:0u32,delay_num:5u16,delay_den:0u16,dispose:0u8,blend:0u8,source:crate::walk_model::Source::Idat},crate::walk_model::Frame {control_offset:126nat,width:2u32,height:2u32,x:1u32,y:1u32,delay_num:1u16,delay_den:10u16,dispose:1u8,blend:0u8,source:crate::walk_model::Source::Fdat(seq![crate::walk_model::Range {offset:176nat,len:16nat}])},crate::walk_model::Frame {control_offset:196nat,width:2u32,height:1u32,x:0u32,y:0u32,delay_num:3u16,delay_den:100u16,dispose:0u8,blend:1u8,source:crate::walk_model::Source::Fdat(seq![crate::walk_model::Range {offset:246nat,len:3nat},crate::walk_model::Range {offset:265nat,len:11nat}])}],..s.doc},..s}
}
#[verifier::spinoff_prover]
proof fn split_apng_still_is_default_image_step_7(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==292),
        (h==(crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,253nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:261nat,len:15nat,next:280nat,crc_ok:true,synthetic:false}))),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}),
        (crate::apng_model::fdat(b.subrange(261,276),true,false,crate::apng_model::Control {sequence:5nat,current:crate::apng_model::Current::Frame(true),frame_count:3nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)+(2nat*1nat)})==Ok(crate::apng_model::Control {sequence:6nat,current:crate::apng_model::Current::Frame(true),frame_count:3nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)+(2nat*1nat)})),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:6nat,current:crate::apng_model::Current::Frame(true),frame_count:3nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)+(2nat*1nat)},Some(3u32))==Ok(())),
    ensures m::step(b,m::strict(),false,false,split_apng_still_is_default_image_state_7(h))==m::Step::Next(split_apng_still_is_default_image_state_8(h)),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::fdat);hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    let actual=match m::step(b,m::strict(),false,false,split_apng_still_is_default_image_state_7(h)) {m::Step::Next(next)=>next,_=>split_apng_still_is_default_image_state_7(h)};
    assert(m::step(b,m::strict(),false,false,split_apng_still_is_default_image_state_7(h))==m::Step::Next(actual));
    let expected=split_apng_still_is_default_image_state_8(h);
    assert(actual.pos==expected.pos);
    assert(actual.phase==expected.phase);
    assert(actual.control==expected.control);
    assert(actual.doc.info==expected.doc.info);
    assert forall|i: int| 0<=i<expected.doc.frames.len() implies #[trigger] actual.doc.frames[i]==expected.doc.frames[i] by {
        match (actual.doc.frames[i].source,expected.doc.frames[i].source) {
            (crate::walk_model::Source::Fdat(a),crate::walk_model::Source::Fdat(e))=>{assert(a=~=e);},_=>{}
        }
    }
    assert(actual.doc.frames =~= expected.doc.frames);
    assert(actual.doc.idat =~= expected.doc.idat);
}
#[verifier::spinoff_prover]
proof fn split_apng_still_is_default_image_step_8(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==292),
        (h==(crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,280nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:288nat,len:0nat,next:292nat,crc_ok:true,synthetic:false}))),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:6nat,current:crate::apng_model::Current::Frame(true),frame_count:3nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)+(2nat*1nat)},Some(3u32))==Ok(())),
    ensures m::step(b,m::strict(),false,false,split_apng_still_is_default_image_state_8(h))==m::Step::Done(m::Doc {info:m::Info {frame_count:split_apng_still_is_default_image_state_8(h).control.frame_count,..split_apng_still_is_default_image_state_8(h).doc.info},..split_apng_still_is_default_image_state_8(h).doc},m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
}
#[verifier::spinoff_prover]
proof fn split_apng_still_is_default_image_compose(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==292),
        (h==(crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false}))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:26nat,next:91nat,crc_ok:true,synthetic:false}))),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:99nat,len:23nat,next:126nat,crc_ok:true,synthetic:false}))),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:134nat,len:26nat,next:164nat,crc_ok:true,synthetic:false}))),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,164nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:172nat,len:20nat,next:196nat,crc_ok:true,synthetic:false}))),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,196nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:204nat,len:26nat,next:234nat,crc_ok:true,synthetic:false}))),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,234nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:242nat,len:7nat,next:253nat,crc_ok:true,synthetic:false}))),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,253nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:261nat,len:15nat,next:280nat,crc_ok:true,synthetic:false}))),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,280nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:288nat,len:0nat,next:292nat,crc_ok:true,synthetic:false}))),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::model::be32(b,16)==4u32),
        (crate::model::be32(b,20)==3u32),
        (crate::apng_model::actl(b.subrange(41,49))==Ok((3u32,2u32))),
        (crate::apng_model::fctl(b.subrange(61,87),true,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Ok((crate::apng_model::FrameControl {width:4u32,height:3u32,x:0u32,y:0u32,delay_num:5u16,delay_den:0u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}))),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}),
        (crate::apng_model::fctl(b.subrange(134,160),true,false,4u32,3u32,crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==Ok((crate::apng_model::FrameControl {width:2u32,height:2u32,x:1u32,y:1u32,delay_num:1u16,delay_den:10u16,dispose:1u8,blend:0u8},crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)}))),
        (crate::apng_model::fdat(b.subrange(172,192),true,false,crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)})==Ok(crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)})),
        (crate::apng_model::fctl(b.subrange(204,230),true,false,4u32,3u32,crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)})==Ok((crate::apng_model::FrameControl {width:2u32,height:1u32,x:0u32,y:0u32,delay_num:3u16,delay_den:100u16,dispose:0u8,blend:1u8},crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(false),frame_count:3nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)+(2nat*1nat)}))),
        (crate::apng_model::fdat(b.subrange(242,249),true,false,crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(false),frame_count:3nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)+(2nat*1nat)})==Ok(crate::apng_model::Control {sequence:5nat,current:crate::apng_model::Current::Frame(true),frame_count:3nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)+(2nat*1nat)})),
        (crate::apng_model::fdat(b.subrange(261,276),true,false,crate::apng_model::Control {sequence:5nat,current:crate::apng_model::Current::Frame(true),frame_count:3nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)+(2nat*1nat)})==Ok(crate::apng_model::Control {sequence:6nat,current:crate::apng_model::Current::Frame(true),frame_count:3nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)+(2nat*1nat)})),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:6nat,current:crate::apng_model::Current::Frame(true),frame_count:3nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)+(2nat*1nat)},Some(3u32))==Ok(())),
    ensures m::MW_p(b,m::strict())==m::Outcome::Done(doc_apng_still_is_default_image(),m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::fctl);hide(crate::apng_model::fdat);hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    hide(m::drive);
    split_apng_still_is_default_image_step_0(b,h);
    split_apng_still_is_default_image_step_1(b,h);
    split_apng_still_is_default_image_step_2(b,h);
    split_apng_still_is_default_image_step_3(b,h);
    split_apng_still_is_default_image_step_4(b,h);
    split_apng_still_is_default_image_step_5(b,h);
    split_apng_still_is_default_image_step_6(b,h);
    split_apng_still_is_default_image_step_7(b,h);
    split_apng_still_is_default_image_step_8(b,h);
    m::drive_step(b,m::strict(),false,false,split_apng_still_is_default_image_state_8(h));
    m::drive_step(b,m::strict(),false,false,split_apng_still_is_default_image_state_7(h));
    m::drive_step(b,m::strict(),false,false,split_apng_still_is_default_image_state_6(h));
    m::drive_step(b,m::strict(),false,false,split_apng_still_is_default_image_state_5(h));
    m::drive_step(b,m::strict(),false,false,split_apng_still_is_default_image_state_4(h));
    m::drive_step(b,m::strict(),false,false,split_apng_still_is_default_image_state_3(h));
    m::drive_step(b,m::strict(),false,false,split_apng_still_is_default_image_state_2(h));
    m::drive_step(b,m::strict(),false,false,split_apng_still_is_default_image_state_1(h));
    m::drive_step(b,m::strict(),false,false,split_apng_still_is_default_image_state_0(h));
}

spec fn doc_apng_still_is_default_image() -> m::Doc {let s=split_apng_still_is_default_image_state_8(h_apng_still_is_default_image());m::Doc {info:m::Info {frame_count:s.control.frame_count,..s.doc.info},..s.doc}}
#[verifier::spinoff_prover]
proof fn walk_apng_still_is_default_image() ensures m::MW_p(bytes_apng_still_is_default_image(),m::strict())==m::Outcome::Done(doc_apng_still_is_default_image(),m::no_events()),
{
    hide(bytes_apng_still_is_default_image);hide(m::MW_p);hide(crate::critical_model::read);hide(crate::walk_model::classify);
    assert(bytes_apng_still_is_default_image().len()==292nat) by (compute);
    assert(crate::model::mh(bytes_apng_still_is_default_image())==Ok((h_apng_still_is_default_image(),33nat))) by {reveal(crate::model::mh);assert(crate::model::mh(bytes_apng_still_is_default_image())==Ok((h_apng_still_is_default_image(),33nat))) by (compute);}
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_still_is_default_image(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_still_is_default_image(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_still_is_default_image().subrange(41,49),0)==3) by (compute);
        assert(crate::model::be32(bytes_apng_still_is_default_image().subrange(41,49),4)==2) by (compute);
        assert(3u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_still_is_default_image(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:26nat,next:91nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_still_is_default_image(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:26nat,next:91nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_still_is_default_image().subrange(61,87),0)==0) by (compute);
        assert(crate::model::be32(bytes_apng_still_is_default_image().subrange(61,87),4)==4) by (compute);
        assert(crate::model::be32(bytes_apng_still_is_default_image().subrange(61,87),8)==3) by (compute);
        assert(crate::model::be32(bytes_apng_still_is_default_image().subrange(61,87),12)==0) by (compute);
        assert(crate::model::be32(bytes_apng_still_is_default_image().subrange(61,87),16)==0) by (compute);
        assert((4u64)*(3u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_still_is_default_image(),91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:99nat,len:23nat,next:126nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_still_is_default_image(),91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:99nat,len:23nat,next:126nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_still_is_default_image(),126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:134nat,len:26nat,next:164nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_still_is_default_image(),126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:134nat,len:26nat,next:164nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_still_is_default_image().subrange(134,160),0)==1) by (compute);
        assert(crate::model::be32(bytes_apng_still_is_default_image().subrange(134,160),4)==2) by (compute);
        assert(crate::model::be32(bytes_apng_still_is_default_image().subrange(134,160),8)==2) by (compute);
        assert(crate::model::be32(bytes_apng_still_is_default_image().subrange(134,160),12)==1) by (compute);
        assert(crate::model::be32(bytes_apng_still_is_default_image().subrange(134,160),16)==1) by (compute);
        assert((2u64)*(2u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_still_is_default_image(),164nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:172nat,len:20nat,next:196nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_still_is_default_image(),164nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:172nat,len:20nat,next:196nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by (compute);
        }
        assert(crate::model::be32(bytes_apng_still_is_default_image().subrange(172,192),0)==2) by (compute);
        assert(crate::model::be32(bytes_apng_still_is_default_image().subrange(172,192),4)==2023514976) by (compute);
        assert(crate::model::be32(bytes_apng_still_is_default_image().subrange(172,192),8)==4161765772) by (compute);
        assert(crate::model::be32(bytes_apng_still_is_default_image().subrange(172,192),12)==1611270144) by (compute);
        assert(crate::model::be32(bytes_apng_still_is_default_image().subrange(172,192),16)==1071318524) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_still_is_default_image(),196nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:204nat,len:26nat,next:234nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_still_is_default_image(),196nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:204nat,len:26nat,next:234nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_still_is_default_image().subrange(204,230),0)==3) by (compute);
        assert(crate::model::be32(bytes_apng_still_is_default_image().subrange(204,230),4)==2) by (compute);
        assert(crate::model::be32(bytes_apng_still_is_default_image().subrange(204,230),8)==1) by (compute);
        assert(crate::model::be32(bytes_apng_still_is_default_image().subrange(204,230),12)==0) by (compute);
        assert(crate::model::be32(bytes_apng_still_is_default_image().subrange(204,230),16)==0) by (compute);
        assert((2u64)*(1u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_still_is_default_image(),234nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:242nat,len:7nat,next:253nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_still_is_default_image(),234nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:242nat,len:7nat,next:253nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by (compute);
        }
        assert(crate::model::be32(bytes_apng_still_is_default_image().subrange(242,249),0)==4) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_still_is_default_image(),253nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:261nat,len:15nat,next:280nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_still_is_default_image(),253nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:261nat,len:15nat,next:280nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by (compute);
        }
        assert(crate::model::be32(bytes_apng_still_is_default_image().subrange(261,276),0)==5) by (compute);
        assert(crate::model::be32(bytes_apng_still_is_default_image().subrange(261,276),4)==1616967903) by (compute);
        assert(crate::model::be32(bytes_apng_still_is_default_image().subrange(261,276),8)==12713995) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_still_is_default_image(),280nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:288nat,len:0nat,next:292nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_still_is_default_image(),280nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:288nat,len:0nat,next:292nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::model::be32(bytes_apng_still_is_default_image(),16)==4u32) by (compute);
        assert(crate::model::be32(bytes_apng_still_is_default_image(),20)==3u32) by (compute);
        assert(crate::apng_model::actl(bytes_apng_still_is_default_image().subrange(41,49))==Ok((3u32,2u32))) by {
            reveal(crate::apng_model::actl);
            assert(crate::apng_model::actl(bytes_apng_still_is_default_image().subrange(41,49))==Ok((3u32,2u32))) by (compute);
        }
        assert(crate::apng_model::fctl(bytes_apng_still_is_default_image().subrange(61,87),true,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Ok((crate::apng_model::FrameControl {width:4u32,height:3u32,x:0u32,y:0u32,delay_num:5u16,delay_den:0u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}))) by {
            reveal(crate::apng_model::fctl);
            assert(crate::apng_model::fctl(bytes_apng_still_is_default_image().subrange(61,87),true,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Ok((crate::apng_model::FrameControl {width:4u32,height:3u32,x:0u32,y:0u32,delay_num:5u16,delay_den:0u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}))) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}) by (compute);
        }
        assert(crate::apng_model::fctl(bytes_apng_still_is_default_image().subrange(134,160),true,false,4u32,3u32,crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==Ok((crate::apng_model::FrameControl {width:2u32,height:2u32,x:1u32,y:1u32,delay_num:1u16,delay_den:10u16,dispose:1u8,blend:0u8},crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)}))) by {
            reveal(crate::apng_model::fctl);
            assert(crate::apng_model::fctl(bytes_apng_still_is_default_image().subrange(134,160),true,false,4u32,3u32,crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==Ok((crate::apng_model::FrameControl {width:2u32,height:2u32,x:1u32,y:1u32,delay_num:1u16,delay_den:10u16,dispose:1u8,blend:0u8},crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)}))) by (compute);
        }
        assert(crate::apng_model::fdat(bytes_apng_still_is_default_image().subrange(172,192),true,false,crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)})==Ok(crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)})) by {
            reveal(crate::apng_model::fdat);
            assert(crate::apng_model::fdat(bytes_apng_still_is_default_image().subrange(172,192),true,false,crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)})==Ok(crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)})) by (compute);
        }
        assert(crate::apng_model::fctl(bytes_apng_still_is_default_image().subrange(204,230),true,false,4u32,3u32,crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)})==Ok((crate::apng_model::FrameControl {width:2u32,height:1u32,x:0u32,y:0u32,delay_num:3u16,delay_den:100u16,dispose:0u8,blend:1u8},crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(false),frame_count:3nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)+(2nat*1nat)}))) by {
            reveal(crate::apng_model::fctl);
            assert(crate::apng_model::fctl(bytes_apng_still_is_default_image().subrange(204,230),true,false,4u32,3u32,crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)})==Ok((crate::apng_model::FrameControl {width:2u32,height:1u32,x:0u32,y:0u32,delay_num:3u16,delay_den:100u16,dispose:0u8,blend:1u8},crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(false),frame_count:3nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)+(2nat*1nat)}))) by (compute);
        }
        assert(crate::apng_model::fdat(bytes_apng_still_is_default_image().subrange(242,249),true,false,crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(false),frame_count:3nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)+(2nat*1nat)})==Ok(crate::apng_model::Control {sequence:5nat,current:crate::apng_model::Current::Frame(true),frame_count:3nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)+(2nat*1nat)})) by {
            reveal(crate::apng_model::fdat);
            assert(crate::apng_model::fdat(bytes_apng_still_is_default_image().subrange(242,249),true,false,crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(false),frame_count:3nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)+(2nat*1nat)})==Ok(crate::apng_model::Control {sequence:5nat,current:crate::apng_model::Current::Frame(true),frame_count:3nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)+(2nat*1nat)})) by (compute);
        }
        assert(crate::apng_model::fdat(bytes_apng_still_is_default_image().subrange(261,276),true,false,crate::apng_model::Control {sequence:5nat,current:crate::apng_model::Current::Frame(true),frame_count:3nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)+(2nat*1nat)})==Ok(crate::apng_model::Control {sequence:6nat,current:crate::apng_model::Current::Frame(true),frame_count:3nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)+(2nat*1nat)})) by {
            reveal(crate::apng_model::fdat);
            assert(crate::apng_model::fdat(bytes_apng_still_is_default_image().subrange(261,276),true,false,crate::apng_model::Control {sequence:5nat,current:crate::apng_model::Current::Frame(true),frame_count:3nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)+(2nat*1nat)})==Ok(crate::apng_model::Control {sequence:6nat,current:crate::apng_model::Current::Frame(true),frame_count:3nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)+(2nat*1nat)})) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:6nat,current:crate::apng_model::Current::Frame(true),frame_count:3nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)+(2nat*1nat)},Some(3u32))==Ok(())) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:6nat,current:crate::apng_model::Current::Frame(true),frame_count:3nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)+(2nat*1nat)},Some(3u32))==Ok(())) by (compute);
        }
    split_apng_still_is_default_image_compose(bytes_apng_still_is_default_image(),h_apng_still_is_default_image());
}
#[verifier::spinoff_prover]
proof fn project_walk_apng_still_is_default_image() ensures r::default_case(bytes_apng_still_is_default_image(),h_apng_still_is_default_image(),z_apng_still_is_default_image(),0nat),
{
    walk_apng_still_is_default_image();
    assert(m::entries(doc_apng_still_is_default_image())==0nat) by (compute);
    assert(m::default_stream(bytes_apng_still_is_default_image(),doc_apng_still_is_default_image())==Some(z_apng_still_is_default_image())) by (compute);
}
#[verifier::spinoff_prover]
proof fn scan_apng_still_is_default_image() ensures r::scan(raw_apng_still_is_default_image(),a_apng_still_is_default_image(),0)==Ok(()),
{
    hide(raw_apng_still_is_default_image);assert(raw_apng_still_is_default_image().len()==51nat) by (compute);assert(r::count(a_apng_still_is_default_image())==1nat) by (compute);
    assert(r::pw(a_apng_still_is_default_image(),0nat)==4nat && r::ph(a_apng_still_is_default_image(),0nat)==3nat && r::rb(a_apng_still_is_default_image(),0nat)==16nat && r::base(a_apng_still_is_default_image(),0nat)==0nat) by (compute);
    assert(raw_apng_still_is_default_image()[0]==0u8) by (compute);
    assert(r::row_result(raw_apng_still_is_default_image(),a_apng_still_is_default_image(),0nat,0nat)==Ok(()));
    assert(raw_apng_still_is_default_image()[17]==1u8) by (compute);
    assert(r::row_result(raw_apng_still_is_default_image(),a_apng_still_is_default_image(),0nat,1nat)==Ok(()));
    assert(raw_apng_still_is_default_image()[34]==2u8) by (compute);
    assert(r::row_result(raw_apng_still_is_default_image(),a_apng_still_is_default_image(),0nat,2nat)==Ok(()));
    r::scan_step(raw_apng_still_is_default_image(),a_apng_still_is_default_image(),1nat);
    r::pass_step(raw_apng_still_is_default_image(),a_apng_still_is_default_image(),0nat,3nat);
    r::pass_step(raw_apng_still_is_default_image(),a_apng_still_is_default_image(),0nat,2nat);
    r::pass_step(raw_apng_still_is_default_image(),a_apng_still_is_default_image(),0nat,1nat);
    r::pass_step(raw_apng_still_is_default_image(),a_apng_still_is_default_image(),0nat,0nat);
    r::scan_step(raw_apng_still_is_default_image(),a_apng_still_is_default_image(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_apng_still_is_default_image() ensures r::still(bytes_apng_still_is_default_image(),STRICT,false)==(r::Projection::Ready {width:4u32,height:3u32,bytes:48nat}),
{
    hide(bytes_apng_still_is_default_image);hide(z_apng_still_is_default_image);hide(raw_apng_still_is_default_image);hide(r::data);

    assert(bytes_apng_still_is_default_image().len()==292nat) by (compute);
    project_walk_apng_still_is_default_image();
    inflate_apng_still_is_default_image();scan_apng_still_is_default_image();
    assert(r::raw(a_apng_still_is_default_image())==51nat) by (compute);
    assert(r::data(z_apng_still_is_default_image(),a_apng_still_is_default_image(),STRICT)==Some(Ok(()))) by {reveal(r::data);};
    r::default_ready(bytes_apng_still_is_default_image(),h_apng_still_is_default_image(),z_apng_still_is_default_image(),0nat,false);
}
fn public_apng_still_is_default_image() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,2u8,32u8,227u8,219u8,236u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,5u8,0u8,0u8,0u8,0u8,52u8,100u8,47u8,237u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,0u8,1u8,0u8,10u8,1u8,0u8,13u8,197u8,211u8,225u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,2u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,85u8,246u8,198u8,140u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,3u8,0u8,100u8,0u8,1u8,192u8,66u8,10u8,152u8,0u8,0u8,0u8,7u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,4u8,120u8,156u8,99u8,207u8,160u8,248u8,174u8,0u8,0u8,0u8,15u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,5u8,96u8,96u8,248u8,223u8,0u8,194u8,0u8,11u8,1u8,2u8,255u8,120u8,231u8,82u8,174u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_apng_still_is_default_image());model_apng_still_is_default_image();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==4u32 && s.height==3u32 && s.rgba@.len()==48nat,Err(crate::alloc_guard::TryError::OutOfMemory(_))=>true,_=>false});
}
// apng/default-not-in-animation-still; frozen SHA256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_apng_default_not_in_animation_still() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,243u8,141u8,147u8,112u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,1u8,0u8,0u8,228u8,1u8,51u8,225u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,1u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,6u8,108u8,157u8,8u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,1u8,0u8,1u8,0u8,0u8,215u8,25u8,34u8,153u8,0u8,0u8,0u8,18u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,3u8,120u8,156u8,99u8,96u8,96u8,248u8,223u8,0u8,194u8,0u8,11u8,1u8,2u8,255u8,241u8,195u8,124u8,94u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn h_apng_default_not_in_animation_still() -> crate::model::Ihdr {crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:6u8,interlace:0u8}}
spec fn a_apng_default_not_in_animation_still() -> r::Raster {r::shape(h_apng_default_not_in_animation_still(),4nat,3nat,0nat,STRICT)}
spec fn z_apng_default_not_in_animation_still() -> Seq<u8> {seq![120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8]}
spec fn raw_apng_default_not_in_animation_still() -> Seq<u8> {seq![0u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,1u8,255u8,0u8,0u8,255u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8]}
spec fn ts_apng_default_not_in_animation_still() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:36nat,before:1nat,after:2nat,symbol:255nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:36nat,next:44nat,before:2nat,after:3nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:44nat,next:52nat,before:3nat,after:4nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:52nat,next:61nat,before:4nat,after:5nat,symbol:255nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:61nat,next:74nat,before:5nat,after:17nat,symbol:265nat,le:1nat,ds:3nat,de:0nat,length:12nat,distance:4nat},t::Step {pos:74nat,next:82nat,before:17nat,after:18nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:82nat,next:95nat,before:18nat,after:22nat,symbol:258nat,le:0nat,ds:4nat,de:0nat,length:4nat,distance:5nat},t::Step {pos:95nat,next:103nat,before:22nat,after:23nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:103nat,next:116nat,before:23nat,after:34nat,symbol:265nat,le:0nat,ds:0nat,de:0nat,length:11nat,distance:1nat},t::Step {pos:116nat,next:124nat,before:34nat,after:35nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:124nat,next:132nat,before:35nat,after:36nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:132nat,next:145nat,before:36nat,after:51nat,symbol:267nat,le:0nat,ds:0nat,de:0nat,length:15nat,distance:1nat},t::Step {pos:145nat,next:152nat,before:51nat,after:51nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_apng_default_not_in_animation_still_0() ensures t::trace(z_apng_default_not_in_animation_still(),raw_apng_default_not_in_animation_still(),ts_apng_default_not_in_animation_still(),0nat,14nat),
{
    hide(t::valid_step);assert(t::trace(z_apng_default_not_in_animation_still(),raw_apng_default_not_in_animation_still(),ts_apng_default_not_in_animation_still(),0nat,14nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn trace_apng_default_not_in_animation_still() ensures t::trace(z_apng_default_not_in_animation_still(),raw_apng_default_not_in_animation_still(),ts_apng_default_not_in_animation_still(),0,14nat),
{
    hide(z_apng_default_not_in_animation_still);hide(raw_apng_default_not_in_animation_still);hide(ts_apng_default_not_in_animation_still);hide(t::trace);
    trace_apng_default_not_in_animation_still_0();
    assert(t::trace(z_apng_default_not_in_animation_still(),raw_apng_default_not_in_animation_still(),ts_apng_default_not_in_animation_still(),0,14nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_apng_default_not_in_animation_still() ensures z_apng_default_not_in_animation_still().len()==23nat,
        raw_apng_default_not_in_animation_still().len()==51nat,
        z_apng_default_not_in_animation_still()[0]==120u8,
        z_apng_default_not_in_animation_still()[1]==156u8,
        zm::field_value(z_apng_default_not_in_animation_still(),16,1)==1nat,
        zm::field_value(z_apng_default_not_in_animation_still(),17,2)==1nat,
        zm::be_value(z_apng_default_not_in_animation_still(),19nat,4)==2448624122nat,
        ts_apng_default_not_in_animation_still().len()==14nat,
        ts_apng_default_not_in_animation_still()[0].pos==19nat,
        ts_apng_default_not_in_animation_still()[0].before==0nat,
        ts_apng_default_not_in_animation_still().last().next==152nat,
{
    assert(z_apng_default_not_in_animation_still().len()==23nat) by (compute);
    assert(raw_apng_default_not_in_animation_still().len()==51nat) by (compute);
    assert(z_apng_default_not_in_animation_still()[0]==120u8) by (compute);
    assert(z_apng_default_not_in_animation_still()[1]==156u8) by (compute);
    assert(zm::field_value(z_apng_default_not_in_animation_still(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_apng_default_not_in_animation_still(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_apng_default_not_in_animation_still(),19nat,4)==2448624122nat) by (compute);
    assert(ts_apng_default_not_in_animation_still().len()==14nat) by (compute);
    assert(ts_apng_default_not_in_animation_still()[0].pos==19nat) by (compute);
    assert(ts_apng_default_not_in_animation_still()[0].before==0nat) by (compute);
    assert(ts_apng_default_not_in_animation_still().last().next==152nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_apng_default_not_in_animation_still() ensures zm::Adler(raw_apng_default_not_in_animation_still())==2448624122nat,
{
    hide(raw_apng_default_not_in_animation_still);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_apng_default_not_in_animation_still().len()==51nat) by (compute);
    assert(c::sums(raw_apng_default_not_in_animation_still(),0,51nat)==(2553nat,102884nat)) by (compute);
    c::sums_correct(raw_apng_default_not_in_animation_still(),0,51nat);assert(raw_apng_default_not_in_animation_still().subrange(0,51)=~=raw_apng_default_not_in_animation_still());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((102884nat%65521)*(u16::MAX as nat+1)+(1+2553nat)%65521==2448624122nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_apng_default_not_in_animation_still() ensures zm::Inflate(z_apng_default_not_in_animation_still(),51nat,true,false)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_apng_default_not_in_animation_still(),r2:false}),raw_apng_default_not_in_animation_still().len()==51nat,
{
    hide(z_apng_default_not_in_animation_still);hide(raw_apng_default_not_in_animation_still);hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);hide(ts_apng_default_not_in_animation_still);
    shape_apng_default_not_in_animation_still();checksum_apng_default_not_in_animation_still();
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_apng_default_not_in_animation_still();ms::fixed_stream(z_apng_default_not_in_animation_still(),raw_apng_default_not_in_animation_still(),ts_apng_default_not_in_animation_still(),51nat,156nat,2448624122nat,2448624122nat);
}
spec fn split_apng_default_not_in_animation_still_state_0(h: crate::model::Ihdr) -> m::State {m::initial(h,33,m::no_events())}
spec fn split_apng_default_not_in_animation_still_state_1(h: crate::model::Ihdr) -> m::State {
    let s=m::initial(h,33,m::no_events());
    m::State {pos:53nat,seen:set![crate::walk_model::Name::Ihdr,crate::walk_model::Name::Actl],phase:crate::walk_model::Phase::Before,control:crate::apng_model::initial(),
        doc:m::Doc {actl:Some((2u32,0u32)),info:s.doc.info,idat:seq![],frames:seq![],..s.doc},..s}
}
#[verifier::spinoff_prover]
proof fn split_apng_default_not_in_animation_still_step_0(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==238),
        (h==(crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false}))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::apng_model::actl(b.subrange(41,49))==Ok((2u32,0u32))),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:false,pixel_total:(2nat*2nat)+(2nat*1nat)},Some(2u32))==Ok(())),
    ensures m::step(b,m::strict(),false,false,split_apng_default_not_in_animation_still_state_0(h))==m::Step::Next(split_apng_default_not_in_animation_still_state_1(h)),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    let actual=match m::step(b,m::strict(),false,false,split_apng_default_not_in_animation_still_state_0(h)) {m::Step::Next(next)=>next,_=>split_apng_default_not_in_animation_still_state_0(h)};
    assert(m::step(b,m::strict(),false,false,split_apng_default_not_in_animation_still_state_0(h))==m::Step::Next(actual));
    let expected=split_apng_default_not_in_animation_still_state_1(h);
    assert(actual.pos==expected.pos);
    assert(actual.phase==expected.phase);
    assert(actual.control==expected.control);
    assert(actual.doc.info==expected.doc.info);
    assert forall|i: int| 0<=i<expected.doc.frames.len() implies #[trigger] actual.doc.frames[i]==expected.doc.frames[i] by {
        match (actual.doc.frames[i].source,expected.doc.frames[i].source) {
            (crate::walk_model::Source::Fdat(a),crate::walk_model::Source::Fdat(e))=>{assert(a=~=e);},_=>{}
        }
    }
    assert(actual.doc.frames =~= expected.doc.frames);
    assert(actual.doc.idat =~= expected.doc.idat);
}
spec fn split_apng_default_not_in_animation_still_state_2(h: crate::model::Ihdr) -> m::State {
    let s=m::initial(h,33,m::no_events());
    m::State {pos:88nat,seen:set![crate::walk_model::Name::Ihdr,crate::walk_model::Name::Actl],phase:crate::walk_model::Phase::During,control:crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat},
        doc:m::Doc {actl:Some((2u32,0u32)),info:s.doc.info,idat:seq![crate::walk_model::Range {offset:61nat,len:23nat}],frames:seq![],..s.doc},..s}
}
#[verifier::spinoff_prover]
proof fn split_apng_default_not_in_animation_still_step_1(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==238),
        (h==(crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false}))),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:false,pixel_total:(2nat*2nat)+(2nat*1nat)},Some(2u32))==Ok(())),
    ensures m::step(b,m::strict(),false,false,split_apng_default_not_in_animation_still_state_1(h))==m::Step::Next(split_apng_default_not_in_animation_still_state_2(h)),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    let actual=match m::step(b,m::strict(),false,false,split_apng_default_not_in_animation_still_state_1(h)) {m::Step::Next(next)=>next,_=>split_apng_default_not_in_animation_still_state_1(h)};
    assert(m::step(b,m::strict(),false,false,split_apng_default_not_in_animation_still_state_1(h))==m::Step::Next(actual));
    let expected=split_apng_default_not_in_animation_still_state_2(h);
    assert(actual.pos==expected.pos);
    assert(actual.phase==expected.phase);
    assert(actual.control==expected.control);
    assert(actual.doc.info==expected.doc.info);
    assert forall|i: int| 0<=i<expected.doc.frames.len() implies #[trigger] actual.doc.frames[i]==expected.doc.frames[i] by {
        match (actual.doc.frames[i].source,expected.doc.frames[i].source) {
            (crate::walk_model::Source::Fdat(a),crate::walk_model::Source::Fdat(e))=>{assert(a=~=e);},_=>{}
        }
    }
    assert(actual.doc.frames =~= expected.doc.frames);
    assert(actual.doc.idat =~= expected.doc.idat);
}
spec fn split_apng_default_not_in_animation_still_state_3(h: crate::model::Ihdr) -> m::State {
    let s=m::initial(h,33,m::no_events());
    m::State {pos:126nat,seen:set![crate::walk_model::Name::Ihdr,crate::walk_model::Name::Actl],phase:crate::walk_model::Phase::After,control:crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:false,pixel_total:(2nat*2nat)},
        doc:m::Doc {actl:Some((2u32,0u32)),info:s.doc.info,idat:seq![crate::walk_model::Range {offset:61nat,len:23nat}],frames:seq![crate::walk_model::Frame {control_offset:88nat,width:2u32,height:2u32,x:0u32,y:0u32,delay_num:1u16,delay_den:1u16,dispose:0u8,blend:0u8,source:crate::walk_model::Source::Fdat(seq![])}],..s.doc},..s}
}
#[verifier::spinoff_prover]
proof fn split_apng_default_not_in_animation_still_step_2(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==238),
        (h==(crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:96nat,len:26nat,next:126nat,crc_ok:true,synthetic:false}))),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}),
        (crate::apng_model::fctl(b.subrange(96,122),true,false,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Ok((crate::apng_model::FrameControl {width:2u32,height:2u32,x:0u32,y:0u32,delay_num:1u16,delay_den:1u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:false,pixel_total:(2nat*2nat)}))),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:false,pixel_total:(2nat*2nat)+(2nat*1nat)},Some(2u32))==Ok(())),
    ensures m::step(b,m::strict(),false,false,split_apng_default_not_in_animation_still_state_2(h))==m::Step::Next(split_apng_default_not_in_animation_still_state_3(h)),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::fctl);hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    let actual=match m::step(b,m::strict(),false,false,split_apng_default_not_in_animation_still_state_2(h)) {m::Step::Next(next)=>next,_=>split_apng_default_not_in_animation_still_state_2(h)};
    assert(m::step(b,m::strict(),false,false,split_apng_default_not_in_animation_still_state_2(h))==m::Step::Next(actual));
    let expected=split_apng_default_not_in_animation_still_state_3(h);
    assert(actual.pos==expected.pos);
    assert(actual.phase==expected.phase);
    assert(actual.control==expected.control);
    assert(actual.doc.info==expected.doc.info);
    assert forall|i: int| 0<=i<expected.doc.frames.len() implies #[trigger] actual.doc.frames[i]==expected.doc.frames[i] by {
        match (actual.doc.frames[i].source,expected.doc.frames[i].source) {
            (crate::walk_model::Source::Fdat(a),crate::walk_model::Source::Fdat(e))=>{assert(a=~=e);},_=>{}
        }
    }
    assert(actual.doc.frames =~= expected.doc.frames);
    assert(actual.doc.idat =~= expected.doc.idat);
}
spec fn split_apng_default_not_in_animation_still_state_4(h: crate::model::Ihdr) -> m::State {
    let s=m::initial(h,33,m::no_events());
    m::State {pos:158nat,seen:set![crate::walk_model::Name::Ihdr,crate::walk_model::Name::Actl],phase:crate::walk_model::Phase::After,control:crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(true),frame_count:1nat,fctl_before_idat:false,pixel_total:(2nat*2nat)},
        doc:m::Doc {actl:Some((2u32,0u32)),info:s.doc.info,idat:seq![crate::walk_model::Range {offset:61nat,len:23nat}],frames:seq![crate::walk_model::Frame {control_offset:88nat,width:2u32,height:2u32,x:0u32,y:0u32,delay_num:1u16,delay_den:1u16,dispose:0u8,blend:0u8,source:crate::walk_model::Source::Fdat(seq![crate::walk_model::Range {offset:138nat,len:16nat}])}],..s.doc},..s}
}
#[verifier::spinoff_prover]
proof fn split_apng_default_not_in_animation_still_step_3(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==238),
        (h==(crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:134nat,len:20nat,next:158nat,crc_ok:true,synthetic:false}))),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}),
        (crate::apng_model::fdat(b.subrange(134,154),true,false,crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:false,pixel_total:(2nat*2nat)})==Ok(crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(true),frame_count:1nat,fctl_before_idat:false,pixel_total:(2nat*2nat)})),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:false,pixel_total:(2nat*2nat)+(2nat*1nat)},Some(2u32))==Ok(())),
    ensures m::step(b,m::strict(),false,false,split_apng_default_not_in_animation_still_state_3(h))==m::Step::Next(split_apng_default_not_in_animation_still_state_4(h)),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::fdat);hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    let actual=match m::step(b,m::strict(),false,false,split_apng_default_not_in_animation_still_state_3(h)) {m::Step::Next(next)=>next,_=>split_apng_default_not_in_animation_still_state_3(h)};
    assert(m::step(b,m::strict(),false,false,split_apng_default_not_in_animation_still_state_3(h))==m::Step::Next(actual));
    let expected=split_apng_default_not_in_animation_still_state_4(h);
    assert(actual.pos==expected.pos);
    assert(actual.phase==expected.phase);
    assert(actual.control==expected.control);
    assert(actual.doc.info==expected.doc.info);
    assert forall|i: int| 0<=i<expected.doc.frames.len() implies #[trigger] actual.doc.frames[i]==expected.doc.frames[i] by {
        match (actual.doc.frames[i].source,expected.doc.frames[i].source) {
            (crate::walk_model::Source::Fdat(a),crate::walk_model::Source::Fdat(e))=>{assert(a=~=e);},_=>{}
        }
    }
    assert(actual.doc.frames =~= expected.doc.frames);
    assert(actual.doc.idat =~= expected.doc.idat);
}
spec fn split_apng_default_not_in_animation_still_state_5(h: crate::model::Ihdr) -> m::State {
    let s=m::initial(h,33,m::no_events());
    m::State {pos:196nat,seen:set![crate::walk_model::Name::Ihdr,crate::walk_model::Name::Actl],phase:crate::walk_model::Phase::After,control:crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:false,pixel_total:(2nat*2nat)+(2nat*1nat)},
        doc:m::Doc {actl:Some((2u32,0u32)),info:s.doc.info,idat:seq![crate::walk_model::Range {offset:61nat,len:23nat}],frames:seq![crate::walk_model::Frame {control_offset:88nat,width:2u32,height:2u32,x:0u32,y:0u32,delay_num:1u16,delay_den:1u16,dispose:0u8,blend:0u8,source:crate::walk_model::Source::Fdat(seq![crate::walk_model::Range {offset:138nat,len:16nat}])},crate::walk_model::Frame {control_offset:158nat,width:2u32,height:1u32,x:2u32,y:2u32,delay_num:1u16,delay_den:1u16,dispose:0u8,blend:0u8,source:crate::walk_model::Source::Fdat(seq![])}],..s.doc},..s}
}
#[verifier::spinoff_prover]
proof fn split_apng_default_not_in_animation_still_step_4(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==238),
        (h==(crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,158nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:166nat,len:26nat,next:196nat,crc_ok:true,synthetic:false}))),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}),
        (crate::apng_model::fctl(b.subrange(166,192),true,false,4u32,3u32,crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(true),frame_count:1nat,fctl_before_idat:false,pixel_total:(2nat*2nat)})==Ok((crate::apng_model::FrameControl {width:2u32,height:1u32,x:2u32,y:2u32,delay_num:1u16,delay_den:1u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:false,pixel_total:(2nat*2nat)+(2nat*1nat)}))),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:false,pixel_total:(2nat*2nat)+(2nat*1nat)},Some(2u32))==Ok(())),
    ensures m::step(b,m::strict(),false,false,split_apng_default_not_in_animation_still_state_4(h))==m::Step::Next(split_apng_default_not_in_animation_still_state_5(h)),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::fctl);hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    let actual=match m::step(b,m::strict(),false,false,split_apng_default_not_in_animation_still_state_4(h)) {m::Step::Next(next)=>next,_=>split_apng_default_not_in_animation_still_state_4(h)};
    assert(m::step(b,m::strict(),false,false,split_apng_default_not_in_animation_still_state_4(h))==m::Step::Next(actual));
    let expected=split_apng_default_not_in_animation_still_state_5(h);
    assert(actual.pos==expected.pos);
    assert(actual.phase==expected.phase);
    assert(actual.control==expected.control);
    assert(actual.doc.info==expected.doc.info);
    assert forall|i: int| 0<=i<expected.doc.frames.len() implies #[trigger] actual.doc.frames[i]==expected.doc.frames[i] by {
        match (actual.doc.frames[i].source,expected.doc.frames[i].source) {
            (crate::walk_model::Source::Fdat(a),crate::walk_model::Source::Fdat(e))=>{assert(a=~=e);},_=>{}
        }
    }
    assert(actual.doc.frames =~= expected.doc.frames);
    assert(actual.doc.idat =~= expected.doc.idat);
}
spec fn split_apng_default_not_in_animation_still_state_6(h: crate::model::Ihdr) -> m::State {
    let s=m::initial(h,33,m::no_events());
    m::State {pos:226nat,seen:set![crate::walk_model::Name::Ihdr,crate::walk_model::Name::Actl],phase:crate::walk_model::Phase::After,control:crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:false,pixel_total:(2nat*2nat)+(2nat*1nat)},
        doc:m::Doc {actl:Some((2u32,0u32)),info:s.doc.info,idat:seq![crate::walk_model::Range {offset:61nat,len:23nat}],frames:seq![crate::walk_model::Frame {control_offset:88nat,width:2u32,height:2u32,x:0u32,y:0u32,delay_num:1u16,delay_den:1u16,dispose:0u8,blend:0u8,source:crate::walk_model::Source::Fdat(seq![crate::walk_model::Range {offset:138nat,len:16nat}])},crate::walk_model::Frame {control_offset:158nat,width:2u32,height:1u32,x:2u32,y:2u32,delay_num:1u16,delay_den:1u16,dispose:0u8,blend:0u8,source:crate::walk_model::Source::Fdat(seq![crate::walk_model::Range {offset:208nat,len:14nat}])}],..s.doc},..s}
}
#[verifier::spinoff_prover]
proof fn split_apng_default_not_in_animation_still_step_5(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==238),
        (h==(crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,196nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:204nat,len:18nat,next:226nat,crc_ok:true,synthetic:false}))),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}),
        (crate::apng_model::fdat(b.subrange(204,222),true,false,crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:false,pixel_total:(2nat*2nat)+(2nat*1nat)})==Ok(crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:false,pixel_total:(2nat*2nat)+(2nat*1nat)})),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:false,pixel_total:(2nat*2nat)+(2nat*1nat)},Some(2u32))==Ok(())),
    ensures m::step(b,m::strict(),false,false,split_apng_default_not_in_animation_still_state_5(h))==m::Step::Next(split_apng_default_not_in_animation_still_state_6(h)),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::fdat);hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    let actual=match m::step(b,m::strict(),false,false,split_apng_default_not_in_animation_still_state_5(h)) {m::Step::Next(next)=>next,_=>split_apng_default_not_in_animation_still_state_5(h)};
    assert(m::step(b,m::strict(),false,false,split_apng_default_not_in_animation_still_state_5(h))==m::Step::Next(actual));
    let expected=split_apng_default_not_in_animation_still_state_6(h);
    assert(actual.pos==expected.pos);
    assert(actual.phase==expected.phase);
    assert(actual.control==expected.control);
    assert(actual.doc.info==expected.doc.info);
    assert forall|i: int| 0<=i<expected.doc.frames.len() implies #[trigger] actual.doc.frames[i]==expected.doc.frames[i] by {
        match (actual.doc.frames[i].source,expected.doc.frames[i].source) {
            (crate::walk_model::Source::Fdat(a),crate::walk_model::Source::Fdat(e))=>{assert(a=~=e);},_=>{}
        }
    }
    assert(actual.doc.frames =~= expected.doc.frames);
    assert(actual.doc.idat =~= expected.doc.idat);
}
#[verifier::spinoff_prover]
proof fn split_apng_default_not_in_animation_still_step_6(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==238),
        (h==(crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,226nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:234nat,len:0nat,next:238nat,crc_ok:true,synthetic:false}))),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:false,pixel_total:(2nat*2nat)+(2nat*1nat)},Some(2u32))==Ok(())),
    ensures m::step(b,m::strict(),false,false,split_apng_default_not_in_animation_still_state_6(h))==m::Step::Done(m::Doc {info:m::Info {frame_count:split_apng_default_not_in_animation_still_state_6(h).control.frame_count,..split_apng_default_not_in_animation_still_state_6(h).doc.info},..split_apng_default_not_in_animation_still_state_6(h).doc},m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
}
#[verifier::spinoff_prover]
proof fn split_apng_default_not_in_animation_still_compose(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==238),
        (h==(crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false}))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false}))),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:96nat,len:26nat,next:126nat,crc_ok:true,synthetic:false}))),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:134nat,len:20nat,next:158nat,crc_ok:true,synthetic:false}))),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,158nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:166nat,len:26nat,next:196nat,crc_ok:true,synthetic:false}))),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,196nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:204nat,len:18nat,next:226nat,crc_ok:true,synthetic:false}))),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,226nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:234nat,len:0nat,next:238nat,crc_ok:true,synthetic:false}))),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::model::be32(b,16)==4u32),
        (crate::model::be32(b,20)==3u32),
        (crate::apng_model::actl(b.subrange(41,49))==Ok((2u32,0u32))),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}),
        (crate::apng_model::fctl(b.subrange(96,122),true,false,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Ok((crate::apng_model::FrameControl {width:2u32,height:2u32,x:0u32,y:0u32,delay_num:1u16,delay_den:1u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:false,pixel_total:(2nat*2nat)}))),
        (crate::apng_model::fdat(b.subrange(134,154),true,false,crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:false,pixel_total:(2nat*2nat)})==Ok(crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(true),frame_count:1nat,fctl_before_idat:false,pixel_total:(2nat*2nat)})),
        (crate::apng_model::fctl(b.subrange(166,192),true,false,4u32,3u32,crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(true),frame_count:1nat,fctl_before_idat:false,pixel_total:(2nat*2nat)})==Ok((crate::apng_model::FrameControl {width:2u32,height:1u32,x:2u32,y:2u32,delay_num:1u16,delay_den:1u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:false,pixel_total:(2nat*2nat)+(2nat*1nat)}))),
        (crate::apng_model::fdat(b.subrange(204,222),true,false,crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:false,pixel_total:(2nat*2nat)+(2nat*1nat)})==Ok(crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:false,pixel_total:(2nat*2nat)+(2nat*1nat)})),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:false,pixel_total:(2nat*2nat)+(2nat*1nat)},Some(2u32))==Ok(())),
    ensures m::MW_p(b,m::strict())==m::Outcome::Done(doc_apng_default_not_in_animation_still(),m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::fctl);hide(crate::apng_model::fdat);hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    hide(m::drive);
    split_apng_default_not_in_animation_still_step_0(b,h);
    split_apng_default_not_in_animation_still_step_1(b,h);
    split_apng_default_not_in_animation_still_step_2(b,h);
    split_apng_default_not_in_animation_still_step_3(b,h);
    split_apng_default_not_in_animation_still_step_4(b,h);
    split_apng_default_not_in_animation_still_step_5(b,h);
    split_apng_default_not_in_animation_still_step_6(b,h);
    m::drive_step(b,m::strict(),false,false,split_apng_default_not_in_animation_still_state_6(h));
    m::drive_step(b,m::strict(),false,false,split_apng_default_not_in_animation_still_state_5(h));
    m::drive_step(b,m::strict(),false,false,split_apng_default_not_in_animation_still_state_4(h));
    m::drive_step(b,m::strict(),false,false,split_apng_default_not_in_animation_still_state_3(h));
    m::drive_step(b,m::strict(),false,false,split_apng_default_not_in_animation_still_state_2(h));
    m::drive_step(b,m::strict(),false,false,split_apng_default_not_in_animation_still_state_1(h));
    m::drive_step(b,m::strict(),false,false,split_apng_default_not_in_animation_still_state_0(h));
}

spec fn doc_apng_default_not_in_animation_still() -> m::Doc {let s=split_apng_default_not_in_animation_still_state_6(h_apng_default_not_in_animation_still());m::Doc {info:m::Info {frame_count:s.control.frame_count,..s.doc.info},..s.doc}}
#[verifier::spinoff_prover]
proof fn walk_apng_default_not_in_animation_still() ensures m::MW_p(bytes_apng_default_not_in_animation_still(),m::strict())==m::Outcome::Done(doc_apng_default_not_in_animation_still(),m::no_events()),
{
    hide(bytes_apng_default_not_in_animation_still);hide(m::MW_p);hide(crate::critical_model::read);hide(crate::walk_model::classify);
    assert(bytes_apng_default_not_in_animation_still().len()==238nat) by (compute);
    assert(crate::model::mh(bytes_apng_default_not_in_animation_still())==Ok((h_apng_default_not_in_animation_still(),33nat))) by {reveal(crate::model::mh);assert(crate::model::mh(bytes_apng_default_not_in_animation_still())==Ok((h_apng_default_not_in_animation_still(),33nat))) by (compute);}
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_default_not_in_animation_still(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_default_not_in_animation_still(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_default_not_in_animation_still().subrange(41,49),0)==2) by (compute);
        assert(crate::model::be32(bytes_apng_default_not_in_animation_still().subrange(41,49),4)==0) by (compute);
        assert(2u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_default_not_in_animation_still(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_default_not_in_animation_still(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:61nat,len:23nat,next:88nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_default_not_in_animation_still(),88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:96nat,len:26nat,next:126nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_default_not_in_animation_still(),88nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:96nat,len:26nat,next:126nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_default_not_in_animation_still().subrange(96,122),0)==0) by (compute);
        assert(crate::model::be32(bytes_apng_default_not_in_animation_still().subrange(96,122),4)==2) by (compute);
        assert(crate::model::be32(bytes_apng_default_not_in_animation_still().subrange(96,122),8)==2) by (compute);
        assert(crate::model::be32(bytes_apng_default_not_in_animation_still().subrange(96,122),12)==0) by (compute);
        assert(crate::model::be32(bytes_apng_default_not_in_animation_still().subrange(96,122),16)==0) by (compute);
        assert((2u64)*(2u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_default_not_in_animation_still(),126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:134nat,len:20nat,next:158nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_default_not_in_animation_still(),126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:134nat,len:20nat,next:158nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by (compute);
        }
        assert(crate::model::be32(bytes_apng_default_not_in_animation_still().subrange(134,154),0)==1) by (compute);
        assert(crate::model::be32(bytes_apng_default_not_in_animation_still().subrange(134,154),4)==2023514976) by (compute);
        assert(crate::model::be32(bytes_apng_default_not_in_animation_still().subrange(134,154),8)==4161765772) by (compute);
        assert(crate::model::be32(bytes_apng_default_not_in_animation_still().subrange(134,154),12)==1611270144) by (compute);
        assert(crate::model::be32(bytes_apng_default_not_in_animation_still().subrange(134,154),16)==1071318524) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_default_not_in_animation_still(),158nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:166nat,len:26nat,next:196nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_default_not_in_animation_still(),158nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:166nat,len:26nat,next:196nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_default_not_in_animation_still().subrange(166,192),0)==2) by (compute);
        assert(crate::model::be32(bytes_apng_default_not_in_animation_still().subrange(166,192),4)==2) by (compute);
        assert(crate::model::be32(bytes_apng_default_not_in_animation_still().subrange(166,192),8)==1) by (compute);
        assert(crate::model::be32(bytes_apng_default_not_in_animation_still().subrange(166,192),12)==2) by (compute);
        assert(crate::model::be32(bytes_apng_default_not_in_animation_still().subrange(166,192),16)==2) by (compute);
        assert((2u64)*(1u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_default_not_in_animation_still(),196nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:204nat,len:18nat,next:226nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_default_not_in_animation_still(),196nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:204nat,len:18nat,next:226nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by (compute);
        }
        assert(crate::model::be32(bytes_apng_default_not_in_animation_still().subrange(204,222),0)==3) by (compute);
        assert(crate::model::be32(bytes_apng_default_not_in_animation_still().subrange(204,222),4)==2023514976) by (compute);
        assert(crate::model::be32(bytes_apng_default_not_in_animation_still().subrange(204,222),8)==1626922752) by (compute);
        assert(crate::model::be32(bytes_apng_default_not_in_animation_still().subrange(204,222),12)==3254782721) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_default_not_in_animation_still(),226nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:234nat,len:0nat,next:238nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_default_not_in_animation_still(),226nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:234nat,len:0nat,next:238nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::model::be32(bytes_apng_default_not_in_animation_still(),16)==4u32) by (compute);
        assert(crate::model::be32(bytes_apng_default_not_in_animation_still(),20)==3u32) by (compute);
        assert(crate::apng_model::actl(bytes_apng_default_not_in_animation_still().subrange(41,49))==Ok((2u32,0u32))) by {
            reveal(crate::apng_model::actl);
            assert(crate::apng_model::actl(bytes_apng_default_not_in_animation_still().subrange(41,49))==Ok((2u32,0u32))) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat}) by (compute);
        }
        assert(crate::apng_model::fctl(bytes_apng_default_not_in_animation_still().subrange(96,122),true,false,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Ok((crate::apng_model::FrameControl {width:2u32,height:2u32,x:0u32,y:0u32,delay_num:1u16,delay_den:1u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:false,pixel_total:(2nat*2nat)}))) by {
            reveal(crate::apng_model::fctl);
            assert(crate::apng_model::fctl(bytes_apng_default_not_in_animation_still().subrange(96,122),true,false,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Ok((crate::apng_model::FrameControl {width:2u32,height:2u32,x:0u32,y:0u32,delay_num:1u16,delay_den:1u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:false,pixel_total:(2nat*2nat)}))) by (compute);
        }
        assert(crate::apng_model::fdat(bytes_apng_default_not_in_animation_still().subrange(134,154),true,false,crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:false,pixel_total:(2nat*2nat)})==Ok(crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(true),frame_count:1nat,fctl_before_idat:false,pixel_total:(2nat*2nat)})) by {
            reveal(crate::apng_model::fdat);
            assert(crate::apng_model::fdat(bytes_apng_default_not_in_animation_still().subrange(134,154),true,false,crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:false,pixel_total:(2nat*2nat)})==Ok(crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(true),frame_count:1nat,fctl_before_idat:false,pixel_total:(2nat*2nat)})) by (compute);
        }
        assert(crate::apng_model::fctl(bytes_apng_default_not_in_animation_still().subrange(166,192),true,false,4u32,3u32,crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(true),frame_count:1nat,fctl_before_idat:false,pixel_total:(2nat*2nat)})==Ok((crate::apng_model::FrameControl {width:2u32,height:1u32,x:2u32,y:2u32,delay_num:1u16,delay_den:1u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:false,pixel_total:(2nat*2nat)+(2nat*1nat)}))) by {
            reveal(crate::apng_model::fctl);
            assert(crate::apng_model::fctl(bytes_apng_default_not_in_animation_still().subrange(166,192),true,false,4u32,3u32,crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(true),frame_count:1nat,fctl_before_idat:false,pixel_total:(2nat*2nat)})==Ok((crate::apng_model::FrameControl {width:2u32,height:1u32,x:2u32,y:2u32,delay_num:1u16,delay_den:1u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:false,pixel_total:(2nat*2nat)+(2nat*1nat)}))) by (compute);
        }
        assert(crate::apng_model::fdat(bytes_apng_default_not_in_animation_still().subrange(204,222),true,false,crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:false,pixel_total:(2nat*2nat)+(2nat*1nat)})==Ok(crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:false,pixel_total:(2nat*2nat)+(2nat*1nat)})) by {
            reveal(crate::apng_model::fdat);
            assert(crate::apng_model::fdat(bytes_apng_default_not_in_animation_still().subrange(204,222),true,false,crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:false,pixel_total:(2nat*2nat)+(2nat*1nat)})==Ok(crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:false,pixel_total:(2nat*2nat)+(2nat*1nat)})) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:false,pixel_total:(2nat*2nat)+(2nat*1nat)},Some(2u32))==Ok(())) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:false,pixel_total:(2nat*2nat)+(2nat*1nat)},Some(2u32))==Ok(())) by (compute);
        }
    split_apng_default_not_in_animation_still_compose(bytes_apng_default_not_in_animation_still(),h_apng_default_not_in_animation_still());
}
#[verifier::spinoff_prover]
proof fn project_walk_apng_default_not_in_animation_still() ensures r::default_case(bytes_apng_default_not_in_animation_still(),h_apng_default_not_in_animation_still(),z_apng_default_not_in_animation_still(),0nat),
{
    walk_apng_default_not_in_animation_still();
    assert(m::entries(doc_apng_default_not_in_animation_still())==0nat) by (compute);
    assert(m::default_stream(bytes_apng_default_not_in_animation_still(),doc_apng_default_not_in_animation_still())==Some(z_apng_default_not_in_animation_still())) by (compute);
}
#[verifier::spinoff_prover]
proof fn scan_apng_default_not_in_animation_still() ensures r::scan(raw_apng_default_not_in_animation_still(),a_apng_default_not_in_animation_still(),0)==Ok(()),
{
    hide(raw_apng_default_not_in_animation_still);assert(raw_apng_default_not_in_animation_still().len()==51nat) by (compute);assert(r::count(a_apng_default_not_in_animation_still())==1nat) by (compute);
    assert(r::pw(a_apng_default_not_in_animation_still(),0nat)==4nat && r::ph(a_apng_default_not_in_animation_still(),0nat)==3nat && r::rb(a_apng_default_not_in_animation_still(),0nat)==16nat && r::base(a_apng_default_not_in_animation_still(),0nat)==0nat) by (compute);
    assert(raw_apng_default_not_in_animation_still()[0]==0u8) by (compute);
    assert(r::row_result(raw_apng_default_not_in_animation_still(),a_apng_default_not_in_animation_still(),0nat,0nat)==Ok(()));
    assert(raw_apng_default_not_in_animation_still()[17]==1u8) by (compute);
    assert(r::row_result(raw_apng_default_not_in_animation_still(),a_apng_default_not_in_animation_still(),0nat,1nat)==Ok(()));
    assert(raw_apng_default_not_in_animation_still()[34]==2u8) by (compute);
    assert(r::row_result(raw_apng_default_not_in_animation_still(),a_apng_default_not_in_animation_still(),0nat,2nat)==Ok(()));
    r::scan_step(raw_apng_default_not_in_animation_still(),a_apng_default_not_in_animation_still(),1nat);
    r::pass_step(raw_apng_default_not_in_animation_still(),a_apng_default_not_in_animation_still(),0nat,3nat);
    r::pass_step(raw_apng_default_not_in_animation_still(),a_apng_default_not_in_animation_still(),0nat,2nat);
    r::pass_step(raw_apng_default_not_in_animation_still(),a_apng_default_not_in_animation_still(),0nat,1nat);
    r::pass_step(raw_apng_default_not_in_animation_still(),a_apng_default_not_in_animation_still(),0nat,0nat);
    r::scan_step(raw_apng_default_not_in_animation_still(),a_apng_default_not_in_animation_still(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_apng_default_not_in_animation_still() ensures r::still(bytes_apng_default_not_in_animation_still(),STRICT,false)==(r::Projection::Ready {width:4u32,height:3u32,bytes:48nat}),
{
    hide(bytes_apng_default_not_in_animation_still);hide(z_apng_default_not_in_animation_still);hide(raw_apng_default_not_in_animation_still);hide(r::data);

    assert(bytes_apng_default_not_in_animation_still().len()==238nat) by (compute);
    project_walk_apng_default_not_in_animation_still();
    inflate_apng_default_not_in_animation_still();scan_apng_default_not_in_animation_still();
    assert(r::raw(a_apng_default_not_in_animation_still())==51nat) by (compute);
    assert(r::data(z_apng_default_not_in_animation_still(),a_apng_default_not_in_animation_still(),STRICT)==Some(Ok(()))) by {reveal(r::data);};
    r::default_ready(bytes_apng_default_not_in_animation_still(),h_apng_default_not_in_animation_still(),z_apng_default_not_in_animation_still(),0nat,false);
}
fn public_apng_default_not_in_animation_still() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,243u8,141u8,147u8,112u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,1u8,0u8,0u8,228u8,1u8,51u8,225u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,1u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,6u8,108u8,157u8,8u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,1u8,0u8,1u8,0u8,0u8,215u8,25u8,34u8,153u8,0u8,0u8,0u8,18u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,3u8,120u8,156u8,99u8,96u8,96u8,248u8,223u8,0u8,194u8,0u8,11u8,1u8,2u8,255u8,241u8,195u8,124u8,94u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_apng_default_not_in_animation_still());model_apng_default_not_in_animation_still();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==4u32 && s.height==3u32 && s.rgba@.len()==48nat,Err(crate::alloc_guard::TryError::OutOfMemory(_))=>true,_=>false});
}
// apng/frame1-error-still-unaffected; frozen SHA256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_apng_frame1_error_still_unaffected() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,243u8,141u8,147u8,112u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,204u8,115u8,12u8,251u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,115u8,39u8,54u8,212u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,2u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,0u8,225u8,253u8,120u8,187u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn h_apng_frame1_error_still_unaffected() -> crate::model::Ihdr {crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:6u8,interlace:0u8}}
spec fn a_apng_frame1_error_still_unaffected() -> r::Raster {r::shape(h_apng_frame1_error_still_unaffected(),4nat,3nat,0nat,STRICT)}
spec fn z_apng_frame1_error_still_unaffected() -> Seq<u8> {seq![120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8]}
spec fn raw_apng_frame1_error_still_unaffected() -> Seq<u8> {seq![0u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,1u8,255u8,0u8,0u8,255u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8]}
spec fn ts_apng_frame1_error_still_unaffected() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:36nat,before:1nat,after:2nat,symbol:255nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:36nat,next:44nat,before:2nat,after:3nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:44nat,next:52nat,before:3nat,after:4nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:52nat,next:61nat,before:4nat,after:5nat,symbol:255nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:61nat,next:74nat,before:5nat,after:17nat,symbol:265nat,le:1nat,ds:3nat,de:0nat,length:12nat,distance:4nat},t::Step {pos:74nat,next:82nat,before:17nat,after:18nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:82nat,next:95nat,before:18nat,after:22nat,symbol:258nat,le:0nat,ds:4nat,de:0nat,length:4nat,distance:5nat},t::Step {pos:95nat,next:103nat,before:22nat,after:23nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:103nat,next:116nat,before:23nat,after:34nat,symbol:265nat,le:0nat,ds:0nat,de:0nat,length:11nat,distance:1nat},t::Step {pos:116nat,next:124nat,before:34nat,after:35nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:124nat,next:132nat,before:35nat,after:36nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:132nat,next:145nat,before:36nat,after:51nat,symbol:267nat,le:0nat,ds:0nat,de:0nat,length:15nat,distance:1nat},t::Step {pos:145nat,next:152nat,before:51nat,after:51nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_apng_frame1_error_still_unaffected_0() ensures t::trace(z_apng_frame1_error_still_unaffected(),raw_apng_frame1_error_still_unaffected(),ts_apng_frame1_error_still_unaffected(),0nat,14nat),
{
    hide(t::valid_step);assert(t::trace(z_apng_frame1_error_still_unaffected(),raw_apng_frame1_error_still_unaffected(),ts_apng_frame1_error_still_unaffected(),0nat,14nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn trace_apng_frame1_error_still_unaffected() ensures t::trace(z_apng_frame1_error_still_unaffected(),raw_apng_frame1_error_still_unaffected(),ts_apng_frame1_error_still_unaffected(),0,14nat),
{
    hide(z_apng_frame1_error_still_unaffected);hide(raw_apng_frame1_error_still_unaffected);hide(ts_apng_frame1_error_still_unaffected);hide(t::trace);
    trace_apng_frame1_error_still_unaffected_0();
    assert(t::trace(z_apng_frame1_error_still_unaffected(),raw_apng_frame1_error_still_unaffected(),ts_apng_frame1_error_still_unaffected(),0,14nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_apng_frame1_error_still_unaffected() ensures z_apng_frame1_error_still_unaffected().len()==23nat,
        raw_apng_frame1_error_still_unaffected().len()==51nat,
        z_apng_frame1_error_still_unaffected()[0]==120u8,
        z_apng_frame1_error_still_unaffected()[1]==156u8,
        zm::field_value(z_apng_frame1_error_still_unaffected(),16,1)==1nat,
        zm::field_value(z_apng_frame1_error_still_unaffected(),17,2)==1nat,
        zm::be_value(z_apng_frame1_error_still_unaffected(),19nat,4)==2448624122nat,
        ts_apng_frame1_error_still_unaffected().len()==14nat,
        ts_apng_frame1_error_still_unaffected()[0].pos==19nat,
        ts_apng_frame1_error_still_unaffected()[0].before==0nat,
        ts_apng_frame1_error_still_unaffected().last().next==152nat,
{
    assert(z_apng_frame1_error_still_unaffected().len()==23nat) by (compute);
    assert(raw_apng_frame1_error_still_unaffected().len()==51nat) by (compute);
    assert(z_apng_frame1_error_still_unaffected()[0]==120u8) by (compute);
    assert(z_apng_frame1_error_still_unaffected()[1]==156u8) by (compute);
    assert(zm::field_value(z_apng_frame1_error_still_unaffected(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_apng_frame1_error_still_unaffected(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_apng_frame1_error_still_unaffected(),19nat,4)==2448624122nat) by (compute);
    assert(ts_apng_frame1_error_still_unaffected().len()==14nat) by (compute);
    assert(ts_apng_frame1_error_still_unaffected()[0].pos==19nat) by (compute);
    assert(ts_apng_frame1_error_still_unaffected()[0].before==0nat) by (compute);
    assert(ts_apng_frame1_error_still_unaffected().last().next==152nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_apng_frame1_error_still_unaffected() ensures zm::Adler(raw_apng_frame1_error_still_unaffected())==2448624122nat,
{
    hide(raw_apng_frame1_error_still_unaffected);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_apng_frame1_error_still_unaffected().len()==51nat) by (compute);
    assert(c::sums(raw_apng_frame1_error_still_unaffected(),0,51nat)==(2553nat,102884nat)) by (compute);
    c::sums_correct(raw_apng_frame1_error_still_unaffected(),0,51nat);assert(raw_apng_frame1_error_still_unaffected().subrange(0,51)=~=raw_apng_frame1_error_still_unaffected());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((102884nat%65521)*(u16::MAX as nat+1)+(1+2553nat)%65521==2448624122nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_apng_frame1_error_still_unaffected() ensures zm::Inflate(z_apng_frame1_error_still_unaffected(),51nat,true,false)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_apng_frame1_error_still_unaffected(),r2:false}),raw_apng_frame1_error_still_unaffected().len()==51nat,
{
    hide(z_apng_frame1_error_still_unaffected);hide(raw_apng_frame1_error_still_unaffected);hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);hide(ts_apng_frame1_error_still_unaffected);
    shape_apng_frame1_error_still_unaffected();checksum_apng_frame1_error_still_unaffected();
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_apng_frame1_error_still_unaffected();ms::fixed_stream(z_apng_frame1_error_still_unaffected(),raw_apng_frame1_error_still_unaffected(),ts_apng_frame1_error_still_unaffected(),51nat,156nat,2448624122nat,2448624122nat);
}
spec fn split_apng_frame1_error_still_unaffected_state_0(h: crate::model::Ihdr) -> m::State {m::initial(h,33,m::no_events())}
spec fn split_apng_frame1_error_still_unaffected_state_1(h: crate::model::Ihdr) -> m::State {
    let s=m::initial(h,33,m::no_events());
    m::State {pos:53nat,seen:set![crate::walk_model::Name::Ihdr,crate::walk_model::Name::Actl],phase:crate::walk_model::Phase::Before,control:crate::apng_model::initial(),
        doc:m::Doc {actl:Some((2u32,0u32)),info:s.doc.info,idat:seq![],frames:seq![],..s.doc},..s}
}
#[verifier::spinoff_prover]
proof fn split_apng_frame1_error_still_unaffected_step_0(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==208),
        (h==(crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false}))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::apng_model::actl(b.subrange(41,49))==Ok((2u32,0u32))),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)},Some(2u32))==Ok(())),
    ensures m::step(b,m::strict(),false,false,split_apng_frame1_error_still_unaffected_state_0(h))==m::Step::Next(split_apng_frame1_error_still_unaffected_state_1(h)),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    let actual=match m::step(b,m::strict(),false,false,split_apng_frame1_error_still_unaffected_state_0(h)) {m::Step::Next(next)=>next,_=>split_apng_frame1_error_still_unaffected_state_0(h)};
    assert(m::step(b,m::strict(),false,false,split_apng_frame1_error_still_unaffected_state_0(h))==m::Step::Next(actual));
    let expected=split_apng_frame1_error_still_unaffected_state_1(h);
    assert(actual.pos==expected.pos);
    assert(actual.phase==expected.phase);
    assert(actual.control==expected.control);
    assert(actual.doc.info==expected.doc.info);
    assert forall|i: int| 0<=i<expected.doc.frames.len() implies #[trigger] actual.doc.frames[i]==expected.doc.frames[i] by {
        match (actual.doc.frames[i].source,expected.doc.frames[i].source) {
            (crate::walk_model::Source::Fdat(a),crate::walk_model::Source::Fdat(e))=>{assert(a=~=e);},_=>{}
        }
    }
    assert(actual.doc.frames =~= expected.doc.frames);
    assert(actual.doc.idat =~= expected.doc.idat);
}
spec fn split_apng_frame1_error_still_unaffected_state_2(h: crate::model::Ihdr) -> m::State {
    let s=m::initial(h,33,m::no_events());
    m::State {pos:91nat,seen:set![crate::walk_model::Name::Ihdr,crate::walk_model::Name::Actl],phase:crate::walk_model::Phase::Before,control:crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)},
        doc:m::Doc {actl:Some((2u32,0u32)),info:s.doc.info,idat:seq![],frames:seq![crate::walk_model::Frame {control_offset:53nat,width:4u32,height:3u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8,source:crate::walk_model::Source::Idat}],..s.doc},..s}
}
#[verifier::spinoff_prover]
proof fn split_apng_frame1_error_still_unaffected_step_1(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==208),
        (h==(crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:26nat,next:91nat,crc_ok:true,synthetic:false}))),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::apng_model::fctl(b.subrange(61,87),true,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Ok((crate::apng_model::FrameControl {width:4u32,height:3u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}))),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)},Some(2u32))==Ok(())),
    ensures m::step(b,m::strict(),false,false,split_apng_frame1_error_still_unaffected_state_1(h))==m::Step::Next(split_apng_frame1_error_still_unaffected_state_2(h)),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::fctl);hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    let actual=match m::step(b,m::strict(),false,false,split_apng_frame1_error_still_unaffected_state_1(h)) {m::Step::Next(next)=>next,_=>split_apng_frame1_error_still_unaffected_state_1(h)};
    assert(m::step(b,m::strict(),false,false,split_apng_frame1_error_still_unaffected_state_1(h))==m::Step::Next(actual));
    let expected=split_apng_frame1_error_still_unaffected_state_2(h);
    assert(actual.pos==expected.pos);
    assert(actual.phase==expected.phase);
    assert(actual.control==expected.control);
    assert(actual.doc.info==expected.doc.info);
    assert forall|i: int| 0<=i<expected.doc.frames.len() implies #[trigger] actual.doc.frames[i]==expected.doc.frames[i] by {
        match (actual.doc.frames[i].source,expected.doc.frames[i].source) {
            (crate::walk_model::Source::Fdat(a),crate::walk_model::Source::Fdat(e))=>{assert(a=~=e);},_=>{}
        }
    }
    assert(actual.doc.frames =~= expected.doc.frames);
    assert(actual.doc.idat =~= expected.doc.idat);
}
spec fn split_apng_frame1_error_still_unaffected_state_3(h: crate::model::Ihdr) -> m::State {
    let s=m::initial(h,33,m::no_events());
    m::State {pos:126nat,seen:set![crate::walk_model::Name::Ihdr,crate::walk_model::Name::Actl],phase:crate::walk_model::Phase::During,control:crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)},
        doc:m::Doc {actl:Some((2u32,0u32)),info:m::Info {default_is_frame:true,..s.doc.info},idat:seq![crate::walk_model::Range {offset:99nat,len:23nat}],frames:seq![crate::walk_model::Frame {control_offset:53nat,width:4u32,height:3u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8,source:crate::walk_model::Source::Idat}],..s.doc},..s}
}
#[verifier::spinoff_prover]
proof fn split_apng_frame1_error_still_unaffected_step_2(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==208),
        (h==(crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:99nat,len:23nat,next:126nat,crc_ok:true,synthetic:false}))),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)},Some(2u32))==Ok(())),
    ensures m::step(b,m::strict(),false,false,split_apng_frame1_error_still_unaffected_state_2(h))==m::Step::Next(split_apng_frame1_error_still_unaffected_state_3(h)),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    let actual=match m::step(b,m::strict(),false,false,split_apng_frame1_error_still_unaffected_state_2(h)) {m::Step::Next(next)=>next,_=>split_apng_frame1_error_still_unaffected_state_2(h)};
    assert(m::step(b,m::strict(),false,false,split_apng_frame1_error_still_unaffected_state_2(h))==m::Step::Next(actual));
    let expected=split_apng_frame1_error_still_unaffected_state_3(h);
    assert(actual.pos==expected.pos);
    assert(actual.phase==expected.phase);
    assert(actual.control==expected.control);
    assert(actual.doc.info==expected.doc.info);
    assert forall|i: int| 0<=i<expected.doc.frames.len() implies #[trigger] actual.doc.frames[i]==expected.doc.frames[i] by {
        match (actual.doc.frames[i].source,expected.doc.frames[i].source) {
            (crate::walk_model::Source::Fdat(a),crate::walk_model::Source::Fdat(e))=>{assert(a=~=e);},_=>{}
        }
    }
    assert(actual.doc.frames =~= expected.doc.frames);
    assert(actual.doc.idat =~= expected.doc.idat);
}
spec fn split_apng_frame1_error_still_unaffected_state_4(h: crate::model::Ihdr) -> m::State {
    let s=m::initial(h,33,m::no_events());
    m::State {pos:164nat,seen:set![crate::walk_model::Name::Ihdr,crate::walk_model::Name::Actl],phase:crate::walk_model::Phase::After,control:crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)},
        doc:m::Doc {actl:Some((2u32,0u32)),info:m::Info {default_is_frame:true,..s.doc.info},idat:seq![crate::walk_model::Range {offset:99nat,len:23nat}],frames:seq![crate::walk_model::Frame {control_offset:53nat,width:4u32,height:3u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8,source:crate::walk_model::Source::Idat},crate::walk_model::Frame {control_offset:126nat,width:2u32,height:2u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8,source:crate::walk_model::Source::Fdat(seq![])}],..s.doc},..s}
}
#[verifier::spinoff_prover]
proof fn split_apng_frame1_error_still_unaffected_step_3(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==208),
        (h==(crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:134nat,len:26nat,next:164nat,crc_ok:true,synthetic:false}))),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}),
        (crate::apng_model::fctl(b.subrange(134,160),true,false,4u32,3u32,crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==Ok((crate::apng_model::FrameControl {width:2u32,height:2u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)}))),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)},Some(2u32))==Ok(())),
    ensures m::step(b,m::strict(),false,false,split_apng_frame1_error_still_unaffected_state_3(h))==m::Step::Next(split_apng_frame1_error_still_unaffected_state_4(h)),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::fctl);hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    let actual=match m::step(b,m::strict(),false,false,split_apng_frame1_error_still_unaffected_state_3(h)) {m::Step::Next(next)=>next,_=>split_apng_frame1_error_still_unaffected_state_3(h)};
    assert(m::step(b,m::strict(),false,false,split_apng_frame1_error_still_unaffected_state_3(h))==m::Step::Next(actual));
    let expected=split_apng_frame1_error_still_unaffected_state_4(h);
    assert(actual.pos==expected.pos);
    assert(actual.phase==expected.phase);
    assert(actual.control==expected.control);
    assert(actual.doc.info==expected.doc.info);
    assert forall|i: int| 0<=i<expected.doc.frames.len() implies #[trigger] actual.doc.frames[i]==expected.doc.frames[i] by {
        match (actual.doc.frames[i].source,expected.doc.frames[i].source) {
            (crate::walk_model::Source::Fdat(a),crate::walk_model::Source::Fdat(e))=>{assert(a=~=e);},_=>{}
        }
    }
    assert(actual.doc.frames =~= expected.doc.frames);
    assert(actual.doc.idat =~= expected.doc.idat);
}
spec fn split_apng_frame1_error_still_unaffected_state_5(h: crate::model::Ihdr) -> m::State {
    let s=m::initial(h,33,m::no_events());
    m::State {pos:196nat,seen:set![crate::walk_model::Name::Ihdr,crate::walk_model::Name::Actl],phase:crate::walk_model::Phase::After,control:crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)},
        doc:m::Doc {actl:Some((2u32,0u32)),info:m::Info {default_is_frame:true,..s.doc.info},idat:seq![crate::walk_model::Range {offset:99nat,len:23nat}],frames:seq![crate::walk_model::Frame {control_offset:53nat,width:4u32,height:3u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8,source:crate::walk_model::Source::Idat},crate::walk_model::Frame {control_offset:126nat,width:2u32,height:2u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8,source:crate::walk_model::Source::Fdat(seq![crate::walk_model::Range {offset:176nat,len:16nat}])}],..s.doc},..s}
}
#[verifier::spinoff_prover]
proof fn split_apng_frame1_error_still_unaffected_step_4(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==208),
        (h==(crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,164nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:172nat,len:20nat,next:196nat,crc_ok:true,synthetic:false}))),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}),
        (crate::apng_model::fdat(b.subrange(172,192),true,false,crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)})==Ok(crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)})),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)},Some(2u32))==Ok(())),
    ensures m::step(b,m::strict(),false,false,split_apng_frame1_error_still_unaffected_state_4(h))==m::Step::Next(split_apng_frame1_error_still_unaffected_state_5(h)),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::fdat);hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    let actual=match m::step(b,m::strict(),false,false,split_apng_frame1_error_still_unaffected_state_4(h)) {m::Step::Next(next)=>next,_=>split_apng_frame1_error_still_unaffected_state_4(h)};
    assert(m::step(b,m::strict(),false,false,split_apng_frame1_error_still_unaffected_state_4(h))==m::Step::Next(actual));
    let expected=split_apng_frame1_error_still_unaffected_state_5(h);
    assert(actual.pos==expected.pos);
    assert(actual.phase==expected.phase);
    assert(actual.control==expected.control);
    assert(actual.doc.info==expected.doc.info);
    assert forall|i: int| 0<=i<expected.doc.frames.len() implies #[trigger] actual.doc.frames[i]==expected.doc.frames[i] by {
        match (actual.doc.frames[i].source,expected.doc.frames[i].source) {
            (crate::walk_model::Source::Fdat(a),crate::walk_model::Source::Fdat(e))=>{assert(a=~=e);},_=>{}
        }
    }
    assert(actual.doc.frames =~= expected.doc.frames);
    assert(actual.doc.idat =~= expected.doc.idat);
}
#[verifier::spinoff_prover]
proof fn split_apng_frame1_error_still_unaffected_step_5(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==208),
        (h==(crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,196nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:204nat,len:0nat,next:208nat,crc_ok:true,synthetic:false}))),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)},Some(2u32))==Ok(())),
    ensures m::step(b,m::strict(),false,false,split_apng_frame1_error_still_unaffected_state_5(h))==m::Step::Done(m::Doc {info:m::Info {frame_count:split_apng_frame1_error_still_unaffected_state_5(h).control.frame_count,..split_apng_frame1_error_still_unaffected_state_5(h).doc.info},..split_apng_frame1_error_still_unaffected_state_5(h).doc},m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
}
#[verifier::spinoff_prover]
proof fn split_apng_frame1_error_still_unaffected_compose(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==208),
        (h==(crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false}))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:26nat,next:91nat,crc_ok:true,synthetic:false}))),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:99nat,len:23nat,next:126nat,crc_ok:true,synthetic:false}))),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:134nat,len:26nat,next:164nat,crc_ok:true,synthetic:false}))),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,164nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:172nat,len:20nat,next:196nat,crc_ok:true,synthetic:false}))),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,196nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:204nat,len:0nat,next:208nat,crc_ok:true,synthetic:false}))),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::model::be32(b,16)==4u32),
        (crate::model::be32(b,20)==3u32),
        (crate::apng_model::actl(b.subrange(41,49))==Ok((2u32,0u32))),
        (crate::apng_model::fctl(b.subrange(61,87),true,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Ok((crate::apng_model::FrameControl {width:4u32,height:3u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}))),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}),
        (crate::apng_model::fctl(b.subrange(134,160),true,false,4u32,3u32,crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==Ok((crate::apng_model::FrameControl {width:2u32,height:2u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)}))),
        (crate::apng_model::fdat(b.subrange(172,192),true,false,crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)})==Ok(crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)})),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)},Some(2u32))==Ok(())),
    ensures m::MW_p(b,m::strict())==m::Outcome::Done(doc_apng_frame1_error_still_unaffected(),m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::fctl);hide(crate::apng_model::fdat);hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    hide(m::drive);
    split_apng_frame1_error_still_unaffected_step_0(b,h);
    split_apng_frame1_error_still_unaffected_step_1(b,h);
    split_apng_frame1_error_still_unaffected_step_2(b,h);
    split_apng_frame1_error_still_unaffected_step_3(b,h);
    split_apng_frame1_error_still_unaffected_step_4(b,h);
    split_apng_frame1_error_still_unaffected_step_5(b,h);
    m::drive_step(b,m::strict(),false,false,split_apng_frame1_error_still_unaffected_state_5(h));
    m::drive_step(b,m::strict(),false,false,split_apng_frame1_error_still_unaffected_state_4(h));
    m::drive_step(b,m::strict(),false,false,split_apng_frame1_error_still_unaffected_state_3(h));
    m::drive_step(b,m::strict(),false,false,split_apng_frame1_error_still_unaffected_state_2(h));
    m::drive_step(b,m::strict(),false,false,split_apng_frame1_error_still_unaffected_state_1(h));
    m::drive_step(b,m::strict(),false,false,split_apng_frame1_error_still_unaffected_state_0(h));
}

spec fn doc_apng_frame1_error_still_unaffected() -> m::Doc {let s=split_apng_frame1_error_still_unaffected_state_5(h_apng_frame1_error_still_unaffected());m::Doc {info:m::Info {frame_count:s.control.frame_count,..s.doc.info},..s.doc}}
#[verifier::spinoff_prover]
proof fn walk_apng_frame1_error_still_unaffected() ensures m::MW_p(bytes_apng_frame1_error_still_unaffected(),m::strict())==m::Outcome::Done(doc_apng_frame1_error_still_unaffected(),m::no_events()),
{
    hide(bytes_apng_frame1_error_still_unaffected);hide(m::MW_p);hide(crate::critical_model::read);hide(crate::walk_model::classify);
    assert(bytes_apng_frame1_error_still_unaffected().len()==208nat) by (compute);
    assert(crate::model::mh(bytes_apng_frame1_error_still_unaffected())==Ok((h_apng_frame1_error_still_unaffected(),33nat))) by {reveal(crate::model::mh);assert(crate::model::mh(bytes_apng_frame1_error_still_unaffected())==Ok((h_apng_frame1_error_still_unaffected(),33nat))) by (compute);}
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_frame1_error_still_unaffected(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_frame1_error_still_unaffected(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_frame1_error_still_unaffected().subrange(41,49),0)==2) by (compute);
        assert(crate::model::be32(bytes_apng_frame1_error_still_unaffected().subrange(41,49),4)==0) by (compute);
        assert(2u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_frame1_error_still_unaffected(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:26nat,next:91nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_frame1_error_still_unaffected(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:26nat,next:91nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_frame1_error_still_unaffected().subrange(61,87),0)==0) by (compute);
        assert(crate::model::be32(bytes_apng_frame1_error_still_unaffected().subrange(61,87),4)==4) by (compute);
        assert(crate::model::be32(bytes_apng_frame1_error_still_unaffected().subrange(61,87),8)==3) by (compute);
        assert(crate::model::be32(bytes_apng_frame1_error_still_unaffected().subrange(61,87),12)==0) by (compute);
        assert(crate::model::be32(bytes_apng_frame1_error_still_unaffected().subrange(61,87),16)==0) by (compute);
        assert((4u64)*(3u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_frame1_error_still_unaffected(),91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:99nat,len:23nat,next:126nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_frame1_error_still_unaffected(),91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:99nat,len:23nat,next:126nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_frame1_error_still_unaffected(),126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:134nat,len:26nat,next:164nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_frame1_error_still_unaffected(),126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:134nat,len:26nat,next:164nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_frame1_error_still_unaffected().subrange(134,160),0)==1) by (compute);
        assert(crate::model::be32(bytes_apng_frame1_error_still_unaffected().subrange(134,160),4)==2) by (compute);
        assert(crate::model::be32(bytes_apng_frame1_error_still_unaffected().subrange(134,160),8)==2) by (compute);
        assert(crate::model::be32(bytes_apng_frame1_error_still_unaffected().subrange(134,160),12)==0) by (compute);
        assert(crate::model::be32(bytes_apng_frame1_error_still_unaffected().subrange(134,160),16)==0) by (compute);
        assert((2u64)*(2u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_frame1_error_still_unaffected(),164nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:172nat,len:20nat,next:196nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_frame1_error_still_unaffected(),164nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:172nat,len:20nat,next:196nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by (compute);
        }
        assert(crate::model::be32(bytes_apng_frame1_error_still_unaffected().subrange(172,192),0)==2) by (compute);
        assert(crate::model::be32(bytes_apng_frame1_error_still_unaffected().subrange(172,192),4)==2023514976) by (compute);
        assert(crate::model::be32(bytes_apng_frame1_error_still_unaffected().subrange(172,192),8)==4161765772) by (compute);
        assert(crate::model::be32(bytes_apng_frame1_error_still_unaffected().subrange(172,192),12)==1611270144) by (compute);
        assert(crate::model::be32(bytes_apng_frame1_error_still_unaffected().subrange(172,192),16)==1071318272) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_frame1_error_still_unaffected(),196nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:204nat,len:0nat,next:208nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_frame1_error_still_unaffected(),196nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:204nat,len:0nat,next:208nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::model::be32(bytes_apng_frame1_error_still_unaffected(),16)==4u32) by (compute);
        assert(crate::model::be32(bytes_apng_frame1_error_still_unaffected(),20)==3u32) by (compute);
        assert(crate::apng_model::actl(bytes_apng_frame1_error_still_unaffected().subrange(41,49))==Ok((2u32,0u32))) by {
            reveal(crate::apng_model::actl);
            assert(crate::apng_model::actl(bytes_apng_frame1_error_still_unaffected().subrange(41,49))==Ok((2u32,0u32))) by (compute);
        }
        assert(crate::apng_model::fctl(bytes_apng_frame1_error_still_unaffected().subrange(61,87),true,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Ok((crate::apng_model::FrameControl {width:4u32,height:3u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}))) by {
            reveal(crate::apng_model::fctl);
            assert(crate::apng_model::fctl(bytes_apng_frame1_error_still_unaffected().subrange(61,87),true,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Ok((crate::apng_model::FrameControl {width:4u32,height:3u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}))) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}) by (compute);
        }
        assert(crate::apng_model::fctl(bytes_apng_frame1_error_still_unaffected().subrange(134,160),true,false,4u32,3u32,crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==Ok((crate::apng_model::FrameControl {width:2u32,height:2u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)}))) by {
            reveal(crate::apng_model::fctl);
            assert(crate::apng_model::fctl(bytes_apng_frame1_error_still_unaffected().subrange(134,160),true,false,4u32,3u32,crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==Ok((crate::apng_model::FrameControl {width:2u32,height:2u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)}))) by (compute);
        }
        assert(crate::apng_model::fdat(bytes_apng_frame1_error_still_unaffected().subrange(172,192),true,false,crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)})==Ok(crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)})) by {
            reveal(crate::apng_model::fdat);
            assert(crate::apng_model::fdat(bytes_apng_frame1_error_still_unaffected().subrange(172,192),true,false,crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)})==Ok(crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)})) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)},Some(2u32))==Ok(())) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)},Some(2u32))==Ok(())) by (compute);
        }
    split_apng_frame1_error_still_unaffected_compose(bytes_apng_frame1_error_still_unaffected(),h_apng_frame1_error_still_unaffected());
}
#[verifier::spinoff_prover]
proof fn project_walk_apng_frame1_error_still_unaffected() ensures r::default_case(bytes_apng_frame1_error_still_unaffected(),h_apng_frame1_error_still_unaffected(),z_apng_frame1_error_still_unaffected(),0nat),
{
    walk_apng_frame1_error_still_unaffected();
    assert(m::entries(doc_apng_frame1_error_still_unaffected())==0nat) by (compute);
    assert(m::default_stream(bytes_apng_frame1_error_still_unaffected(),doc_apng_frame1_error_still_unaffected())==Some(z_apng_frame1_error_still_unaffected())) by (compute);
}
#[verifier::spinoff_prover]
proof fn scan_apng_frame1_error_still_unaffected() ensures r::scan(raw_apng_frame1_error_still_unaffected(),a_apng_frame1_error_still_unaffected(),0)==Ok(()),
{
    hide(raw_apng_frame1_error_still_unaffected);assert(raw_apng_frame1_error_still_unaffected().len()==51nat) by (compute);assert(r::count(a_apng_frame1_error_still_unaffected())==1nat) by (compute);
    assert(r::pw(a_apng_frame1_error_still_unaffected(),0nat)==4nat && r::ph(a_apng_frame1_error_still_unaffected(),0nat)==3nat && r::rb(a_apng_frame1_error_still_unaffected(),0nat)==16nat && r::base(a_apng_frame1_error_still_unaffected(),0nat)==0nat) by (compute);
    assert(raw_apng_frame1_error_still_unaffected()[0]==0u8) by (compute);
    assert(r::row_result(raw_apng_frame1_error_still_unaffected(),a_apng_frame1_error_still_unaffected(),0nat,0nat)==Ok(()));
    assert(raw_apng_frame1_error_still_unaffected()[17]==1u8) by (compute);
    assert(r::row_result(raw_apng_frame1_error_still_unaffected(),a_apng_frame1_error_still_unaffected(),0nat,1nat)==Ok(()));
    assert(raw_apng_frame1_error_still_unaffected()[34]==2u8) by (compute);
    assert(r::row_result(raw_apng_frame1_error_still_unaffected(),a_apng_frame1_error_still_unaffected(),0nat,2nat)==Ok(()));
    r::scan_step(raw_apng_frame1_error_still_unaffected(),a_apng_frame1_error_still_unaffected(),1nat);
    r::pass_step(raw_apng_frame1_error_still_unaffected(),a_apng_frame1_error_still_unaffected(),0nat,3nat);
    r::pass_step(raw_apng_frame1_error_still_unaffected(),a_apng_frame1_error_still_unaffected(),0nat,2nat);
    r::pass_step(raw_apng_frame1_error_still_unaffected(),a_apng_frame1_error_still_unaffected(),0nat,1nat);
    r::pass_step(raw_apng_frame1_error_still_unaffected(),a_apng_frame1_error_still_unaffected(),0nat,0nat);
    r::scan_step(raw_apng_frame1_error_still_unaffected(),a_apng_frame1_error_still_unaffected(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_apng_frame1_error_still_unaffected() ensures r::still(bytes_apng_frame1_error_still_unaffected(),STRICT,false)==(r::Projection::Ready {width:4u32,height:3u32,bytes:48nat}),
{
    hide(bytes_apng_frame1_error_still_unaffected);hide(z_apng_frame1_error_still_unaffected);hide(raw_apng_frame1_error_still_unaffected);hide(r::data);

    assert(bytes_apng_frame1_error_still_unaffected().len()==208nat) by (compute);
    project_walk_apng_frame1_error_still_unaffected();
    inflate_apng_frame1_error_still_unaffected();scan_apng_frame1_error_still_unaffected();
    assert(r::raw(a_apng_frame1_error_still_unaffected())==51nat) by (compute);
    assert(r::data(z_apng_frame1_error_still_unaffected(),a_apng_frame1_error_still_unaffected(),STRICT)==Some(Ok(()))) by {reveal(r::data);};
    r::default_ready(bytes_apng_frame1_error_still_unaffected(),h_apng_frame1_error_still_unaffected(),z_apng_frame1_error_still_unaffected(),0nat,false);
}
fn public_apng_frame1_error_still_unaffected() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,243u8,141u8,147u8,112u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,204u8,115u8,12u8,251u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,115u8,39u8,54u8,212u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,2u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,0u8,225u8,253u8,120u8,187u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_apng_frame1_error_still_unaffected());model_apng_frame1_error_still_unaffected();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==4u32 && s.height==3u32 && s.rgba@.len()==48nat,Err(crate::alloc_guard::TryError::OutOfMemory(_))=>true,_=>false});
}
// apng/empty-fdat-payload-still; frozen SHA256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_apng_empty_fdat_payload_still() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,8u8,6u8,0u8,0u8,0u8,31u8,21u8,196u8,137u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,243u8,141u8,147u8,112u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,90u8,127u8,48u8,208u8,0u8,0u8,0u8,13u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,224u8,18u8,145u8,251u8,15u8,0u8,1u8,164u8,1u8,60u8,147u8,139u8,14u8,183u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,193u8,12u8,218u8,4u8,0u8,0u8,0u8,4u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,2u8,235u8,198u8,207u8,77u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn h_apng_empty_fdat_payload_still() -> crate::model::Ihdr {crate::model::Ihdr {width:1u32,height:1u32,depth:8u8,color_type:6u8,interlace:0u8}}
spec fn a_apng_empty_fdat_payload_still() -> r::Raster {r::shape(h_apng_empty_fdat_payload_still(),1nat,1nat,0nat,STRICT)}
spec fn z_apng_empty_fdat_payload_still() -> Seq<u8> {seq![120u8,156u8,99u8,224u8,18u8,145u8,251u8,15u8,0u8,1u8,164u8,1u8,60u8]}
spec fn raw_apng_empty_fdat_payload_still() -> Seq<u8> {seq![0u8,10u8,20u8,30u8,255u8]}
spec fn ts_apng_empty_fdat_payload_still() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:35nat,before:1nat,after:2nat,symbol:10nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:35nat,next:43nat,before:2nat,after:3nat,symbol:20nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:43nat,next:51nat,before:3nat,after:4nat,symbol:30nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:51nat,next:60nat,before:4nat,after:5nat,symbol:255nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:60nat,next:67nat,before:5nat,after:5nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_apng_empty_fdat_payload_still_0() ensures t::trace(z_apng_empty_fdat_payload_still(),raw_apng_empty_fdat_payload_still(),ts_apng_empty_fdat_payload_still(),0nat,6nat),
{
    hide(t::valid_step);assert(t::trace(z_apng_empty_fdat_payload_still(),raw_apng_empty_fdat_payload_still(),ts_apng_empty_fdat_payload_still(),0nat,6nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn trace_apng_empty_fdat_payload_still() ensures t::trace(z_apng_empty_fdat_payload_still(),raw_apng_empty_fdat_payload_still(),ts_apng_empty_fdat_payload_still(),0,6nat),
{
    hide(z_apng_empty_fdat_payload_still);hide(raw_apng_empty_fdat_payload_still);hide(ts_apng_empty_fdat_payload_still);hide(t::trace);
    trace_apng_empty_fdat_payload_still_0();
    assert(t::trace(z_apng_empty_fdat_payload_still(),raw_apng_empty_fdat_payload_still(),ts_apng_empty_fdat_payload_still(),0,6nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_apng_empty_fdat_payload_still() ensures z_apng_empty_fdat_payload_still().len()==13nat,
        raw_apng_empty_fdat_payload_still().len()==5nat,
        z_apng_empty_fdat_payload_still()[0]==120u8,
        z_apng_empty_fdat_payload_still()[1]==156u8,
        zm::field_value(z_apng_empty_fdat_payload_still(),16,1)==1nat,
        zm::field_value(z_apng_empty_fdat_payload_still(),17,2)==1nat,
        zm::be_value(z_apng_empty_fdat_payload_still(),9nat,4)==27525436nat,
        ts_apng_empty_fdat_payload_still().len()==6nat,
        ts_apng_empty_fdat_payload_still()[0].pos==19nat,
        ts_apng_empty_fdat_payload_still()[0].before==0nat,
        ts_apng_empty_fdat_payload_still().last().next==67nat,
{
    assert(z_apng_empty_fdat_payload_still().len()==13nat) by (compute);
    assert(raw_apng_empty_fdat_payload_still().len()==5nat) by (compute);
    assert(z_apng_empty_fdat_payload_still()[0]==120u8) by (compute);
    assert(z_apng_empty_fdat_payload_still()[1]==156u8) by (compute);
    assert(zm::field_value(z_apng_empty_fdat_payload_still(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_apng_empty_fdat_payload_still(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_apng_empty_fdat_payload_still(),9nat,4)==27525436nat) by (compute);
    assert(ts_apng_empty_fdat_payload_still().len()==6nat) by (compute);
    assert(ts_apng_empty_fdat_payload_still()[0].pos==19nat) by (compute);
    assert(ts_apng_empty_fdat_payload_still()[0].before==0nat) by (compute);
    assert(ts_apng_empty_fdat_payload_still().last().next==67nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_apng_empty_fdat_payload_still() ensures zm::Adler(raw_apng_empty_fdat_payload_still())==27525436nat,
{
    hide(raw_apng_empty_fdat_payload_still);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_apng_empty_fdat_payload_still().len()==5nat) by (compute);
    assert(c::sums(raw_apng_empty_fdat_payload_still(),0,5nat)==(315nat,420nat)) by (compute);
    c::sums_correct(raw_apng_empty_fdat_payload_still(),0,5nat);assert(raw_apng_empty_fdat_payload_still().subrange(0,5)=~=raw_apng_empty_fdat_payload_still());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((420nat%65521)*(u16::MAX as nat+1)+(1+315nat)%65521==27525436nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_apng_empty_fdat_payload_still() ensures zm::Inflate(z_apng_empty_fdat_payload_still(),5nat,true,false)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_apng_empty_fdat_payload_still(),r2:false}),raw_apng_empty_fdat_payload_still().len()==5nat,
{
    hide(z_apng_empty_fdat_payload_still);hide(raw_apng_empty_fdat_payload_still);hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);hide(ts_apng_empty_fdat_payload_still);
    shape_apng_empty_fdat_payload_still();checksum_apng_empty_fdat_payload_still();
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_apng_empty_fdat_payload_still();ms::fixed_stream(z_apng_empty_fdat_payload_still(),raw_apng_empty_fdat_payload_still(),ts_apng_empty_fdat_payload_still(),5nat,156nat,27525436nat,27525436nat);
}
spec fn split_apng_empty_fdat_payload_still_state_0(h: crate::model::Ihdr) -> m::State {m::initial(h,33,m::no_events())}
spec fn split_apng_empty_fdat_payload_still_state_1(h: crate::model::Ihdr) -> m::State {
    let s=m::initial(h,33,m::no_events());
    m::State {pos:53nat,seen:set![crate::walk_model::Name::Ihdr,crate::walk_model::Name::Actl],phase:crate::walk_model::Phase::Before,control:crate::apng_model::initial(),
        doc:m::Doc {actl:Some((2u32,0u32)),info:s.doc.info,idat:seq![],frames:seq![],..s.doc},..s}
}
#[verifier::spinoff_prover]
proof fn split_apng_empty_fdat_payload_still_step_0(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==182),
        (h==(crate::model::Ihdr {width:1u32,height:1u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false}))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::apng_model::actl(b.subrange(41,49))==Ok((2u32,0u32))),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(1nat*1nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(1nat*1nat)}),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(1nat*1nat)+(1nat*1nat)},Some(2u32))==Ok(())),
    ensures m::step(b,m::strict(),false,false,split_apng_empty_fdat_payload_still_state_0(h))==m::Step::Next(split_apng_empty_fdat_payload_still_state_1(h)),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    let actual=match m::step(b,m::strict(),false,false,split_apng_empty_fdat_payload_still_state_0(h)) {m::Step::Next(next)=>next,_=>split_apng_empty_fdat_payload_still_state_0(h)};
    assert(m::step(b,m::strict(),false,false,split_apng_empty_fdat_payload_still_state_0(h))==m::Step::Next(actual));
    let expected=split_apng_empty_fdat_payload_still_state_1(h);
    assert(actual.pos==expected.pos);
    assert(actual.phase==expected.phase);
    assert(actual.control==expected.control);
    assert(actual.doc.info==expected.doc.info);
    assert forall|i: int| 0<=i<expected.doc.frames.len() implies #[trigger] actual.doc.frames[i]==expected.doc.frames[i] by {
        match (actual.doc.frames[i].source,expected.doc.frames[i].source) {
            (crate::walk_model::Source::Fdat(a),crate::walk_model::Source::Fdat(e))=>{assert(a=~=e);},_=>{}
        }
    }
    assert(actual.doc.frames =~= expected.doc.frames);
    assert(actual.doc.idat =~= expected.doc.idat);
}
spec fn split_apng_empty_fdat_payload_still_state_2(h: crate::model::Ihdr) -> m::State {
    let s=m::initial(h,33,m::no_events());
    m::State {pos:91nat,seen:set![crate::walk_model::Name::Ihdr,crate::walk_model::Name::Actl],phase:crate::walk_model::Phase::Before,control:crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(1nat*1nat)},
        doc:m::Doc {actl:Some((2u32,0u32)),info:s.doc.info,idat:seq![],frames:seq![crate::walk_model::Frame {control_offset:53nat,width:1u32,height:1u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8,source:crate::walk_model::Source::Idat}],..s.doc},..s}
}
#[verifier::spinoff_prover]
proof fn split_apng_empty_fdat_payload_still_step_1(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==182),
        (h==(crate::model::Ihdr {width:1u32,height:1u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:26nat,next:91nat,crc_ok:true,synthetic:false}))),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::apng_model::fctl(b.subrange(61,87),true,true,1u32,1u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Ok((crate::apng_model::FrameControl {width:1u32,height:1u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(1nat*1nat)}))),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(1nat*1nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(1nat*1nat)}),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(1nat*1nat)+(1nat*1nat)},Some(2u32))==Ok(())),
    ensures m::step(b,m::strict(),false,false,split_apng_empty_fdat_payload_still_state_1(h))==m::Step::Next(split_apng_empty_fdat_payload_still_state_2(h)),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::fctl);hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    let actual=match m::step(b,m::strict(),false,false,split_apng_empty_fdat_payload_still_state_1(h)) {m::Step::Next(next)=>next,_=>split_apng_empty_fdat_payload_still_state_1(h)};
    assert(m::step(b,m::strict(),false,false,split_apng_empty_fdat_payload_still_state_1(h))==m::Step::Next(actual));
    let expected=split_apng_empty_fdat_payload_still_state_2(h);
    assert(actual.pos==expected.pos);
    assert(actual.phase==expected.phase);
    assert(actual.control==expected.control);
    assert(actual.doc.info==expected.doc.info);
    assert forall|i: int| 0<=i<expected.doc.frames.len() implies #[trigger] actual.doc.frames[i]==expected.doc.frames[i] by {
        match (actual.doc.frames[i].source,expected.doc.frames[i].source) {
            (crate::walk_model::Source::Fdat(a),crate::walk_model::Source::Fdat(e))=>{assert(a=~=e);},_=>{}
        }
    }
    assert(actual.doc.frames =~= expected.doc.frames);
    assert(actual.doc.idat =~= expected.doc.idat);
}
spec fn split_apng_empty_fdat_payload_still_state_3(h: crate::model::Ihdr) -> m::State {
    let s=m::initial(h,33,m::no_events());
    m::State {pos:116nat,seen:set![crate::walk_model::Name::Ihdr,crate::walk_model::Name::Actl],phase:crate::walk_model::Phase::During,control:crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(1nat*1nat)},
        doc:m::Doc {actl:Some((2u32,0u32)),info:m::Info {default_is_frame:true,..s.doc.info},idat:seq![crate::walk_model::Range {offset:99nat,len:13nat}],frames:seq![crate::walk_model::Frame {control_offset:53nat,width:1u32,height:1u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8,source:crate::walk_model::Source::Idat}],..s.doc},..s}
}
#[verifier::spinoff_prover]
proof fn split_apng_empty_fdat_payload_still_step_2(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==182),
        (h==(crate::model::Ihdr {width:1u32,height:1u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:99nat,len:13nat,next:116nat,crc_ok:true,synthetic:false}))),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(1nat*1nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(1nat*1nat)}),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(1nat*1nat)+(1nat*1nat)},Some(2u32))==Ok(())),
    ensures m::step(b,m::strict(),false,false,split_apng_empty_fdat_payload_still_state_2(h))==m::Step::Next(split_apng_empty_fdat_payload_still_state_3(h)),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    let actual=match m::step(b,m::strict(),false,false,split_apng_empty_fdat_payload_still_state_2(h)) {m::Step::Next(next)=>next,_=>split_apng_empty_fdat_payload_still_state_2(h)};
    assert(m::step(b,m::strict(),false,false,split_apng_empty_fdat_payload_still_state_2(h))==m::Step::Next(actual));
    let expected=split_apng_empty_fdat_payload_still_state_3(h);
    assert(actual.pos==expected.pos);
    assert(actual.phase==expected.phase);
    assert(actual.control==expected.control);
    assert(actual.doc.info==expected.doc.info);
    assert forall|i: int| 0<=i<expected.doc.frames.len() implies #[trigger] actual.doc.frames[i]==expected.doc.frames[i] by {
        match (actual.doc.frames[i].source,expected.doc.frames[i].source) {
            (crate::walk_model::Source::Fdat(a),crate::walk_model::Source::Fdat(e))=>{assert(a=~=e);},_=>{}
        }
    }
    assert(actual.doc.frames =~= expected.doc.frames);
    assert(actual.doc.idat =~= expected.doc.idat);
}
spec fn split_apng_empty_fdat_payload_still_state_4(h: crate::model::Ihdr) -> m::State {
    let s=m::initial(h,33,m::no_events());
    m::State {pos:154nat,seen:set![crate::walk_model::Name::Ihdr,crate::walk_model::Name::Actl],phase:crate::walk_model::Phase::After,control:crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:true,pixel_total:(1nat*1nat)+(1nat*1nat)},
        doc:m::Doc {actl:Some((2u32,0u32)),info:m::Info {default_is_frame:true,..s.doc.info},idat:seq![crate::walk_model::Range {offset:99nat,len:13nat}],frames:seq![crate::walk_model::Frame {control_offset:53nat,width:1u32,height:1u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8,source:crate::walk_model::Source::Idat},crate::walk_model::Frame {control_offset:116nat,width:1u32,height:1u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8,source:crate::walk_model::Source::Fdat(seq![])}],..s.doc},..s}
}
#[verifier::spinoff_prover]
proof fn split_apng_empty_fdat_payload_still_step_3(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==182),
        (h==(crate::model::Ihdr {width:1u32,height:1u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,116nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:124nat,len:26nat,next:154nat,crc_ok:true,synthetic:false}))),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(1nat*1nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(1nat*1nat)}),
        (crate::apng_model::fctl(b.subrange(124,150),true,false,1u32,1u32,crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(1nat*1nat)})==Ok((crate::apng_model::FrameControl {width:1u32,height:1u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:true,pixel_total:(1nat*1nat)+(1nat*1nat)}))),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(1nat*1nat)+(1nat*1nat)},Some(2u32))==Ok(())),
    ensures m::step(b,m::strict(),false,false,split_apng_empty_fdat_payload_still_state_3(h))==m::Step::Next(split_apng_empty_fdat_payload_still_state_4(h)),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::fctl);hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    let actual=match m::step(b,m::strict(),false,false,split_apng_empty_fdat_payload_still_state_3(h)) {m::Step::Next(next)=>next,_=>split_apng_empty_fdat_payload_still_state_3(h)};
    assert(m::step(b,m::strict(),false,false,split_apng_empty_fdat_payload_still_state_3(h))==m::Step::Next(actual));
    let expected=split_apng_empty_fdat_payload_still_state_4(h);
    assert(actual.pos==expected.pos);
    assert(actual.phase==expected.phase);
    assert(actual.control==expected.control);
    assert(actual.doc.info==expected.doc.info);
    assert forall|i: int| 0<=i<expected.doc.frames.len() implies #[trigger] actual.doc.frames[i]==expected.doc.frames[i] by {
        match (actual.doc.frames[i].source,expected.doc.frames[i].source) {
            (crate::walk_model::Source::Fdat(a),crate::walk_model::Source::Fdat(e))=>{assert(a=~=e);},_=>{}
        }
    }
    assert(actual.doc.frames =~= expected.doc.frames);
    assert(actual.doc.idat =~= expected.doc.idat);
}
spec fn split_apng_empty_fdat_payload_still_state_5(h: crate::model::Ihdr) -> m::State {
    let s=m::initial(h,33,m::no_events());
    m::State {pos:170nat,seen:set![crate::walk_model::Name::Ihdr,crate::walk_model::Name::Actl],phase:crate::walk_model::Phase::After,control:crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(1nat*1nat)+(1nat*1nat)},
        doc:m::Doc {actl:Some((2u32,0u32)),info:m::Info {default_is_frame:true,..s.doc.info},idat:seq![crate::walk_model::Range {offset:99nat,len:13nat}],frames:seq![crate::walk_model::Frame {control_offset:53nat,width:1u32,height:1u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8,source:crate::walk_model::Source::Idat},crate::walk_model::Frame {control_offset:116nat,width:1u32,height:1u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8,source:crate::walk_model::Source::Fdat(seq![crate::walk_model::Range {offset:166nat,len:0nat}])}],..s.doc},..s}
}
#[verifier::spinoff_prover]
proof fn split_apng_empty_fdat_payload_still_step_4(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==182),
        (h==(crate::model::Ihdr {width:1u32,height:1u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,154nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:162nat,len:4nat,next:170nat,crc_ok:true,synthetic:false}))),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(1nat*1nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(1nat*1nat)}),
        (crate::apng_model::fdat(b.subrange(162,166),true,false,crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:true,pixel_total:(1nat*1nat)+(1nat*1nat)})==Ok(crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(1nat*1nat)+(1nat*1nat)})),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(1nat*1nat)+(1nat*1nat)},Some(2u32))==Ok(())),
    ensures m::step(b,m::strict(),false,false,split_apng_empty_fdat_payload_still_state_4(h))==m::Step::Next(split_apng_empty_fdat_payload_still_state_5(h)),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::fdat);hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    let actual=match m::step(b,m::strict(),false,false,split_apng_empty_fdat_payload_still_state_4(h)) {m::Step::Next(next)=>next,_=>split_apng_empty_fdat_payload_still_state_4(h)};
    assert(m::step(b,m::strict(),false,false,split_apng_empty_fdat_payload_still_state_4(h))==m::Step::Next(actual));
    let expected=split_apng_empty_fdat_payload_still_state_5(h);
    assert(actual.pos==expected.pos);
    assert(actual.phase==expected.phase);
    assert(actual.control==expected.control);
    assert(actual.doc.info==expected.doc.info);
    assert forall|i: int| 0<=i<expected.doc.frames.len() implies #[trigger] actual.doc.frames[i]==expected.doc.frames[i] by {
        match (actual.doc.frames[i].source,expected.doc.frames[i].source) {
            (crate::walk_model::Source::Fdat(a),crate::walk_model::Source::Fdat(e))=>{assert(a=~=e);},_=>{}
        }
    }
    assert(actual.doc.frames =~= expected.doc.frames);
    assert(actual.doc.idat =~= expected.doc.idat);
}
#[verifier::spinoff_prover]
proof fn split_apng_empty_fdat_payload_still_step_5(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==182),
        (h==(crate::model::Ihdr {width:1u32,height:1u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,170nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:178nat,len:0nat,next:182nat,crc_ok:true,synthetic:false}))),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(1nat*1nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(1nat*1nat)}),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(1nat*1nat)+(1nat*1nat)},Some(2u32))==Ok(())),
    ensures m::step(b,m::strict(),false,false,split_apng_empty_fdat_payload_still_state_5(h))==m::Step::Done(m::Doc {info:m::Info {frame_count:split_apng_empty_fdat_payload_still_state_5(h).control.frame_count,..split_apng_empty_fdat_payload_still_state_5(h).doc.info},..split_apng_empty_fdat_payload_still_state_5(h).doc},m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
}
#[verifier::spinoff_prover]
proof fn split_apng_empty_fdat_payload_still_compose(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==182),
        (h==(crate::model::Ihdr {width:1u32,height:1u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false}))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:26nat,next:91nat,crc_ok:true,synthetic:false}))),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:99nat,len:13nat,next:116nat,crc_ok:true,synthetic:false}))),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,116nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:124nat,len:26nat,next:154nat,crc_ok:true,synthetic:false}))),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,154nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:162nat,len:4nat,next:170nat,crc_ok:true,synthetic:false}))),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,170nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:178nat,len:0nat,next:182nat,crc_ok:true,synthetic:false}))),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::model::be32(b,16)==1u32),
        (crate::model::be32(b,20)==1u32),
        (crate::apng_model::actl(b.subrange(41,49))==Ok((2u32,0u32))),
        (crate::apng_model::fctl(b.subrange(61,87),true,true,1u32,1u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Ok((crate::apng_model::FrameControl {width:1u32,height:1u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(1nat*1nat)}))),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(1nat*1nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(1nat*1nat)}),
        (crate::apng_model::fctl(b.subrange(124,150),true,false,1u32,1u32,crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(1nat*1nat)})==Ok((crate::apng_model::FrameControl {width:1u32,height:1u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:true,pixel_total:(1nat*1nat)+(1nat*1nat)}))),
        (crate::apng_model::fdat(b.subrange(162,166),true,false,crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:true,pixel_total:(1nat*1nat)+(1nat*1nat)})==Ok(crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(1nat*1nat)+(1nat*1nat)})),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(1nat*1nat)+(1nat*1nat)},Some(2u32))==Ok(())),
    ensures m::MW_p(b,m::strict())==m::Outcome::Done(doc_apng_empty_fdat_payload_still(),m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::fctl);hide(crate::apng_model::fdat);hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    hide(m::drive);
    split_apng_empty_fdat_payload_still_step_0(b,h);
    split_apng_empty_fdat_payload_still_step_1(b,h);
    split_apng_empty_fdat_payload_still_step_2(b,h);
    split_apng_empty_fdat_payload_still_step_3(b,h);
    split_apng_empty_fdat_payload_still_step_4(b,h);
    split_apng_empty_fdat_payload_still_step_5(b,h);
    m::drive_step(b,m::strict(),false,false,split_apng_empty_fdat_payload_still_state_5(h));
    m::drive_step(b,m::strict(),false,false,split_apng_empty_fdat_payload_still_state_4(h));
    m::drive_step(b,m::strict(),false,false,split_apng_empty_fdat_payload_still_state_3(h));
    m::drive_step(b,m::strict(),false,false,split_apng_empty_fdat_payload_still_state_2(h));
    m::drive_step(b,m::strict(),false,false,split_apng_empty_fdat_payload_still_state_1(h));
    m::drive_step(b,m::strict(),false,false,split_apng_empty_fdat_payload_still_state_0(h));
}

spec fn doc_apng_empty_fdat_payload_still() -> m::Doc {let s=split_apng_empty_fdat_payload_still_state_5(h_apng_empty_fdat_payload_still());m::Doc {info:m::Info {frame_count:s.control.frame_count,..s.doc.info},..s.doc}}
#[verifier::spinoff_prover]
proof fn walk_apng_empty_fdat_payload_still() ensures m::MW_p(bytes_apng_empty_fdat_payload_still(),m::strict())==m::Outcome::Done(doc_apng_empty_fdat_payload_still(),m::no_events()),
{
    hide(bytes_apng_empty_fdat_payload_still);hide(m::MW_p);hide(crate::critical_model::read);hide(crate::walk_model::classify);
    assert(bytes_apng_empty_fdat_payload_still().len()==182nat) by (compute);
    assert(crate::model::mh(bytes_apng_empty_fdat_payload_still())==Ok((h_apng_empty_fdat_payload_still(),33nat))) by {reveal(crate::model::mh);assert(crate::model::mh(bytes_apng_empty_fdat_payload_still())==Ok((h_apng_empty_fdat_payload_still(),33nat))) by (compute);}
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_empty_fdat_payload_still(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_empty_fdat_payload_still(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_empty_fdat_payload_still().subrange(41,49),0)==2) by (compute);
        assert(crate::model::be32(bytes_apng_empty_fdat_payload_still().subrange(41,49),4)==0) by (compute);
        assert(2u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_empty_fdat_payload_still(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:26nat,next:91nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_empty_fdat_payload_still(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:26nat,next:91nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_empty_fdat_payload_still().subrange(61,87),0)==0) by (compute);
        assert(crate::model::be32(bytes_apng_empty_fdat_payload_still().subrange(61,87),4)==1) by (compute);
        assert(crate::model::be32(bytes_apng_empty_fdat_payload_still().subrange(61,87),8)==1) by (compute);
        assert(crate::model::be32(bytes_apng_empty_fdat_payload_still().subrange(61,87),12)==0) by (compute);
        assert(crate::model::be32(bytes_apng_empty_fdat_payload_still().subrange(61,87),16)==0) by (compute);
        assert((1u64)*(1u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_empty_fdat_payload_still(),91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:99nat,len:13nat,next:116nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_empty_fdat_payload_still(),91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:99nat,len:13nat,next:116nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_empty_fdat_payload_still(),116nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:124nat,len:26nat,next:154nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_empty_fdat_payload_still(),116nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:124nat,len:26nat,next:154nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_empty_fdat_payload_still().subrange(124,150),0)==1) by (compute);
        assert(crate::model::be32(bytes_apng_empty_fdat_payload_still().subrange(124,150),4)==1) by (compute);
        assert(crate::model::be32(bytes_apng_empty_fdat_payload_still().subrange(124,150),8)==1) by (compute);
        assert(crate::model::be32(bytes_apng_empty_fdat_payload_still().subrange(124,150),12)==0) by (compute);
        assert(crate::model::be32(bytes_apng_empty_fdat_payload_still().subrange(124,150),16)==0) by (compute);
        assert((1u64)*(1u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_empty_fdat_payload_still(),154nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:162nat,len:4nat,next:170nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_empty_fdat_payload_still(),154nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:162nat,len:4nat,next:170nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by (compute);
        }
        assert(crate::model::be32(bytes_apng_empty_fdat_payload_still().subrange(162,166),0)==2) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_empty_fdat_payload_still(),170nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:178nat,len:0nat,next:182nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_empty_fdat_payload_still(),170nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:178nat,len:0nat,next:182nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::model::be32(bytes_apng_empty_fdat_payload_still(),16)==1u32) by (compute);
        assert(crate::model::be32(bytes_apng_empty_fdat_payload_still(),20)==1u32) by (compute);
        assert(crate::apng_model::actl(bytes_apng_empty_fdat_payload_still().subrange(41,49))==Ok((2u32,0u32))) by {
            reveal(crate::apng_model::actl);
            assert(crate::apng_model::actl(bytes_apng_empty_fdat_payload_still().subrange(41,49))==Ok((2u32,0u32))) by (compute);
        }
        assert(crate::apng_model::fctl(bytes_apng_empty_fdat_payload_still().subrange(61,87),true,true,1u32,1u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Ok((crate::apng_model::FrameControl {width:1u32,height:1u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(1nat*1nat)}))) by {
            reveal(crate::apng_model::fctl);
            assert(crate::apng_model::fctl(bytes_apng_empty_fdat_payload_still().subrange(61,87),true,true,1u32,1u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Ok((crate::apng_model::FrameControl {width:1u32,height:1u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(1nat*1nat)}))) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(1nat*1nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(1nat*1nat)}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(1nat*1nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(1nat*1nat)}) by (compute);
        }
        assert(crate::apng_model::fctl(bytes_apng_empty_fdat_payload_still().subrange(124,150),true,false,1u32,1u32,crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(1nat*1nat)})==Ok((crate::apng_model::FrameControl {width:1u32,height:1u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:true,pixel_total:(1nat*1nat)+(1nat*1nat)}))) by {
            reveal(crate::apng_model::fctl);
            assert(crate::apng_model::fctl(bytes_apng_empty_fdat_payload_still().subrange(124,150),true,false,1u32,1u32,crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(1nat*1nat)})==Ok((crate::apng_model::FrameControl {width:1u32,height:1u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:true,pixel_total:(1nat*1nat)+(1nat*1nat)}))) by (compute);
        }
        assert(crate::apng_model::fdat(bytes_apng_empty_fdat_payload_still().subrange(162,166),true,false,crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:true,pixel_total:(1nat*1nat)+(1nat*1nat)})==Ok(crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(1nat*1nat)+(1nat*1nat)})) by {
            reveal(crate::apng_model::fdat);
            assert(crate::apng_model::fdat(bytes_apng_empty_fdat_payload_still().subrange(162,166),true,false,crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:true,pixel_total:(1nat*1nat)+(1nat*1nat)})==Ok(crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(1nat*1nat)+(1nat*1nat)})) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(1nat*1nat)+(1nat*1nat)},Some(2u32))==Ok(())) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(1nat*1nat)+(1nat*1nat)},Some(2u32))==Ok(())) by (compute);
        }
    split_apng_empty_fdat_payload_still_compose(bytes_apng_empty_fdat_payload_still(),h_apng_empty_fdat_payload_still());
}
#[verifier::spinoff_prover]
proof fn project_walk_apng_empty_fdat_payload_still() ensures r::default_case(bytes_apng_empty_fdat_payload_still(),h_apng_empty_fdat_payload_still(),z_apng_empty_fdat_payload_still(),0nat),
{
    walk_apng_empty_fdat_payload_still();
    assert(m::entries(doc_apng_empty_fdat_payload_still())==0nat) by (compute);
    assert(m::default_stream(bytes_apng_empty_fdat_payload_still(),doc_apng_empty_fdat_payload_still())==Some(z_apng_empty_fdat_payload_still())) by (compute);
}
#[verifier::spinoff_prover]
proof fn scan_apng_empty_fdat_payload_still() ensures r::scan(raw_apng_empty_fdat_payload_still(),a_apng_empty_fdat_payload_still(),0)==Ok(()),
{
    hide(raw_apng_empty_fdat_payload_still);assert(raw_apng_empty_fdat_payload_still().len()==5nat) by (compute);assert(r::count(a_apng_empty_fdat_payload_still())==1nat) by (compute);
    assert(r::pw(a_apng_empty_fdat_payload_still(),0nat)==1nat && r::ph(a_apng_empty_fdat_payload_still(),0nat)==1nat && r::rb(a_apng_empty_fdat_payload_still(),0nat)==4nat && r::base(a_apng_empty_fdat_payload_still(),0nat)==0nat) by (compute);
    assert(raw_apng_empty_fdat_payload_still()[0]==0u8) by (compute);
    assert(r::row_result(raw_apng_empty_fdat_payload_still(),a_apng_empty_fdat_payload_still(),0nat,0nat)==Ok(()));
    r::scan_step(raw_apng_empty_fdat_payload_still(),a_apng_empty_fdat_payload_still(),1nat);
    r::pass_step(raw_apng_empty_fdat_payload_still(),a_apng_empty_fdat_payload_still(),0nat,1nat);
    r::pass_step(raw_apng_empty_fdat_payload_still(),a_apng_empty_fdat_payload_still(),0nat,0nat);
    r::scan_step(raw_apng_empty_fdat_payload_still(),a_apng_empty_fdat_payload_still(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_apng_empty_fdat_payload_still() ensures r::still(bytes_apng_empty_fdat_payload_still(),STRICT,false)==(r::Projection::Ready {width:1u32,height:1u32,bytes:4nat}),
{
    hide(bytes_apng_empty_fdat_payload_still);hide(z_apng_empty_fdat_payload_still);hide(raw_apng_empty_fdat_payload_still);hide(r::data);

    assert(bytes_apng_empty_fdat_payload_still().len()==182nat) by (compute);
    project_walk_apng_empty_fdat_payload_still();
    inflate_apng_empty_fdat_payload_still();scan_apng_empty_fdat_payload_still();
    assert(r::raw(a_apng_empty_fdat_payload_still())==5nat) by (compute);
    assert(r::data(z_apng_empty_fdat_payload_still(),a_apng_empty_fdat_payload_still(),STRICT)==Some(Ok(()))) by {reveal(r::data);};
    r::default_ready(bytes_apng_empty_fdat_payload_still(),h_apng_empty_fdat_payload_still(),z_apng_empty_fdat_payload_still(),0nat,false);
}
fn public_apng_empty_fdat_payload_still() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,8u8,6u8,0u8,0u8,0u8,31u8,21u8,196u8,137u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,243u8,141u8,147u8,112u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,90u8,127u8,48u8,208u8,0u8,0u8,0u8,13u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,224u8,18u8,145u8,251u8,15u8,0u8,1u8,164u8,1u8,60u8,147u8,139u8,14u8,183u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,193u8,12u8,218u8,4u8,0u8,0u8,0u8,4u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,2u8,235u8,198u8,207u8,77u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_apng_empty_fdat_payload_still());model_apng_empty_fdat_payload_still();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==1u32 && s.height==1u32 && s.rgba@.len()==4nat,Err(crate::alloc_guard::TryError::OutOfMemory(_))=>true,_=>false});
}
// apng/empty-fdat-then-data-still; frozen SHA256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_apng_empty_fdat_then_data_still() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,243u8,141u8,147u8,112u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,204u8,115u8,12u8,251u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,115u8,39u8,54u8,212u8,0u8,0u8,0u8,4u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,2u8,235u8,198u8,207u8,77u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,3u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,210u8,80u8,13u8,207u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn h_apng_empty_fdat_then_data_still() -> crate::model::Ihdr {crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:6u8,interlace:0u8}}
spec fn a_apng_empty_fdat_then_data_still() -> r::Raster {r::shape(h_apng_empty_fdat_then_data_still(),4nat,3nat,0nat,STRICT)}
spec fn z_apng_empty_fdat_then_data_still() -> Seq<u8> {seq![120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8]}
spec fn raw_apng_empty_fdat_then_data_still() -> Seq<u8> {seq![0u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,255u8,0u8,0u8,255u8,1u8,255u8,0u8,0u8,255u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8]}
spec fn ts_apng_empty_fdat_then_data_still() -> Seq<t::Step> {seq![t::Step {pos:19nat,next:27nat,before:0nat,after:1nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:27nat,next:36nat,before:1nat,after:2nat,symbol:255nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:36nat,next:44nat,before:2nat,after:3nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:44nat,next:52nat,before:3nat,after:4nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:52nat,next:61nat,before:4nat,after:5nat,symbol:255nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:61nat,next:74nat,before:5nat,after:17nat,symbol:265nat,le:1nat,ds:3nat,de:0nat,length:12nat,distance:4nat},t::Step {pos:74nat,next:82nat,before:17nat,after:18nat,symbol:1nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:82nat,next:95nat,before:18nat,after:22nat,symbol:258nat,le:0nat,ds:4nat,de:0nat,length:4nat,distance:5nat},t::Step {pos:95nat,next:103nat,before:22nat,after:23nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:103nat,next:116nat,before:23nat,after:34nat,symbol:265nat,le:0nat,ds:0nat,de:0nat,length:11nat,distance:1nat},t::Step {pos:116nat,next:124nat,before:34nat,after:35nat,symbol:2nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:124nat,next:132nat,before:35nat,after:36nat,symbol:0nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat},t::Step {pos:132nat,next:145nat,before:36nat,after:51nat,symbol:267nat,le:0nat,ds:0nat,de:0nat,length:15nat,distance:1nat},t::Step {pos:145nat,next:152nat,before:51nat,after:51nat,symbol:256nat,le:0nat,ds:0nat,de:0nat,length:0nat,distance:0nat}]}
#[verifier::spinoff_prover]
proof fn trace_apng_empty_fdat_then_data_still_0() ensures t::trace(z_apng_empty_fdat_then_data_still(),raw_apng_empty_fdat_then_data_still(),ts_apng_empty_fdat_then_data_still(),0nat,14nat),
{
    hide(t::valid_step);assert(t::trace(z_apng_empty_fdat_then_data_still(),raw_apng_empty_fdat_then_data_still(),ts_apng_empty_fdat_then_data_still(),0nat,14nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn trace_apng_empty_fdat_then_data_still() ensures t::trace(z_apng_empty_fdat_then_data_still(),raw_apng_empty_fdat_then_data_still(),ts_apng_empty_fdat_then_data_still(),0,14nat),
{
    hide(z_apng_empty_fdat_then_data_still);hide(raw_apng_empty_fdat_then_data_still);hide(ts_apng_empty_fdat_then_data_still);hide(t::trace);
    trace_apng_empty_fdat_then_data_still_0();
    assert(t::trace(z_apng_empty_fdat_then_data_still(),raw_apng_empty_fdat_then_data_still(),ts_apng_empty_fdat_then_data_still(),0,14nat)) by (compute);
}
#[verifier::spinoff_prover]
proof fn shape_apng_empty_fdat_then_data_still() ensures z_apng_empty_fdat_then_data_still().len()==23nat,
        raw_apng_empty_fdat_then_data_still().len()==51nat,
        z_apng_empty_fdat_then_data_still()[0]==120u8,
        z_apng_empty_fdat_then_data_still()[1]==156u8,
        zm::field_value(z_apng_empty_fdat_then_data_still(),16,1)==1nat,
        zm::field_value(z_apng_empty_fdat_then_data_still(),17,2)==1nat,
        zm::be_value(z_apng_empty_fdat_then_data_still(),19nat,4)==2448624122nat,
        ts_apng_empty_fdat_then_data_still().len()==14nat,
        ts_apng_empty_fdat_then_data_still()[0].pos==19nat,
        ts_apng_empty_fdat_then_data_still()[0].before==0nat,
        ts_apng_empty_fdat_then_data_still().last().next==152nat,
{
    assert(z_apng_empty_fdat_then_data_still().len()==23nat) by (compute);
    assert(raw_apng_empty_fdat_then_data_still().len()==51nat) by (compute);
    assert(z_apng_empty_fdat_then_data_still()[0]==120u8) by (compute);
    assert(z_apng_empty_fdat_then_data_still()[1]==156u8) by (compute);
    assert(zm::field_value(z_apng_empty_fdat_then_data_still(),16,1)==1nat) by (compute);
    assert(zm::field_value(z_apng_empty_fdat_then_data_still(),17,2)==1nat) by (compute);
    assert(zm::be_value(z_apng_empty_fdat_then_data_still(),19nat,4)==2448624122nat) by (compute);
    assert(ts_apng_empty_fdat_then_data_still().len()==14nat) by (compute);
    assert(ts_apng_empty_fdat_then_data_still()[0].pos==19nat) by (compute);
    assert(ts_apng_empty_fdat_then_data_still()[0].before==0nat) by (compute);
    assert(ts_apng_empty_fdat_then_data_still().last().next==152nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn checksum_apng_empty_fdat_then_data_still() ensures zm::Adler(raw_apng_empty_fdat_then_data_still())==2448624122nat,
{
    hide(raw_apng_empty_fdat_then_data_still);hide(zm::sum_bytes);hide(zm::sum_prefixes);
    assert(raw_apng_empty_fdat_then_data_still().len()==51nat) by (compute);
    assert(c::sums(raw_apng_empty_fdat_then_data_still(),0,51nat)==(2553nat,102884nat)) by (compute);
    c::sums_correct(raw_apng_empty_fdat_then_data_still(),0,51nat);assert(raw_apng_empty_fdat_then_data_still().subrange(0,51)=~=raw_apng_empty_fdat_then_data_still());
    assert(inflate_verus::limits::ADLER_MOD==65521) by (compute);assert(zm::pow2(16)==u16::MAX as nat+1) by (compute);
    assert((102884nat%65521)*(u16::MAX as nat+1)+(1+2553nat)%65521==2448624122nat) by (compute);
}
#[verifier::spinoff_prover]
proof fn inflate_apng_empty_fdat_then_data_still() ensures zm::Inflate(z_apng_empty_fdat_then_data_still(),51nat,true,false)==(zm::Inflated {status:zm::InflateStatus::Done,out:raw_apng_empty_fdat_then_data_still(),r2:false}),raw_apng_empty_fdat_then_data_still().len()==51nat,
{
    hide(z_apng_empty_fdat_then_data_still);hide(raw_apng_empty_fdat_then_data_still);hide(zm::Inflate);hide(zm::MEnvelope);hide(zm::MCompressed);hide(zm::MCompressedStep);hide(ts_apng_empty_fdat_then_data_still);
    shape_apng_empty_fdat_then_data_still();checksum_apng_empty_fdat_then_data_still();
    assert((120nat*256+156nat)%31==0nat && (156nat/32)%2==0nat) by (compute);
    trace_apng_empty_fdat_then_data_still();ms::fixed_stream(z_apng_empty_fdat_then_data_still(),raw_apng_empty_fdat_then_data_still(),ts_apng_empty_fdat_then_data_still(),51nat,156nat,2448624122nat,2448624122nat);
}
spec fn split_apng_empty_fdat_then_data_still_state_0(h: crate::model::Ihdr) -> m::State {m::initial(h,33,m::no_events())}
spec fn split_apng_empty_fdat_then_data_still_state_1(h: crate::model::Ihdr) -> m::State {
    let s=m::initial(h,33,m::no_events());
    m::State {pos:53nat,seen:set![crate::walk_model::Name::Ihdr,crate::walk_model::Name::Actl],phase:crate::walk_model::Phase::Before,control:crate::apng_model::initial(),
        doc:m::Doc {actl:Some((2u32,0u32)),info:s.doc.info,idat:seq![],frames:seq![],..s.doc},..s}
}
#[verifier::spinoff_prover]
proof fn split_apng_empty_fdat_then_data_still_step_0(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==224),
        (h==(crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false}))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::apng_model::actl(b.subrange(41,49))==Ok((2u32,0u32))),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)},Some(2u32))==Ok(())),
    ensures m::step(b,m::strict(),false,false,split_apng_empty_fdat_then_data_still_state_0(h))==m::Step::Next(split_apng_empty_fdat_then_data_still_state_1(h)),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    let actual=match m::step(b,m::strict(),false,false,split_apng_empty_fdat_then_data_still_state_0(h)) {m::Step::Next(next)=>next,_=>split_apng_empty_fdat_then_data_still_state_0(h)};
    assert(m::step(b,m::strict(),false,false,split_apng_empty_fdat_then_data_still_state_0(h))==m::Step::Next(actual));
    let expected=split_apng_empty_fdat_then_data_still_state_1(h);
    assert(actual.pos==expected.pos);
    assert(actual.phase==expected.phase);
    assert(actual.control==expected.control);
    assert(actual.doc.info==expected.doc.info);
    assert forall|i: int| 0<=i<expected.doc.frames.len() implies #[trigger] actual.doc.frames[i]==expected.doc.frames[i] by {
        match (actual.doc.frames[i].source,expected.doc.frames[i].source) {
            (crate::walk_model::Source::Fdat(a),crate::walk_model::Source::Fdat(e))=>{assert(a=~=e);},_=>{}
        }
    }
    assert(actual.doc.frames =~= expected.doc.frames);
    assert(actual.doc.idat =~= expected.doc.idat);
}
spec fn split_apng_empty_fdat_then_data_still_state_2(h: crate::model::Ihdr) -> m::State {
    let s=m::initial(h,33,m::no_events());
    m::State {pos:91nat,seen:set![crate::walk_model::Name::Ihdr,crate::walk_model::Name::Actl],phase:crate::walk_model::Phase::Before,control:crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)},
        doc:m::Doc {actl:Some((2u32,0u32)),info:s.doc.info,idat:seq![],frames:seq![crate::walk_model::Frame {control_offset:53nat,width:4u32,height:3u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8,source:crate::walk_model::Source::Idat}],..s.doc},..s}
}
#[verifier::spinoff_prover]
proof fn split_apng_empty_fdat_then_data_still_step_1(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==224),
        (h==(crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:26nat,next:91nat,crc_ok:true,synthetic:false}))),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::apng_model::fctl(b.subrange(61,87),true,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Ok((crate::apng_model::FrameControl {width:4u32,height:3u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}))),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)},Some(2u32))==Ok(())),
    ensures m::step(b,m::strict(),false,false,split_apng_empty_fdat_then_data_still_state_1(h))==m::Step::Next(split_apng_empty_fdat_then_data_still_state_2(h)),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::fctl);hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    let actual=match m::step(b,m::strict(),false,false,split_apng_empty_fdat_then_data_still_state_1(h)) {m::Step::Next(next)=>next,_=>split_apng_empty_fdat_then_data_still_state_1(h)};
    assert(m::step(b,m::strict(),false,false,split_apng_empty_fdat_then_data_still_state_1(h))==m::Step::Next(actual));
    let expected=split_apng_empty_fdat_then_data_still_state_2(h);
    assert(actual.pos==expected.pos);
    assert(actual.phase==expected.phase);
    assert(actual.control==expected.control);
    assert(actual.doc.info==expected.doc.info);
    assert forall|i: int| 0<=i<expected.doc.frames.len() implies #[trigger] actual.doc.frames[i]==expected.doc.frames[i] by {
        match (actual.doc.frames[i].source,expected.doc.frames[i].source) {
            (crate::walk_model::Source::Fdat(a),crate::walk_model::Source::Fdat(e))=>{assert(a=~=e);},_=>{}
        }
    }
    assert(actual.doc.frames =~= expected.doc.frames);
    assert(actual.doc.idat =~= expected.doc.idat);
}
spec fn split_apng_empty_fdat_then_data_still_state_3(h: crate::model::Ihdr) -> m::State {
    let s=m::initial(h,33,m::no_events());
    m::State {pos:126nat,seen:set![crate::walk_model::Name::Ihdr,crate::walk_model::Name::Actl],phase:crate::walk_model::Phase::During,control:crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)},
        doc:m::Doc {actl:Some((2u32,0u32)),info:m::Info {default_is_frame:true,..s.doc.info},idat:seq![crate::walk_model::Range {offset:99nat,len:23nat}],frames:seq![crate::walk_model::Frame {control_offset:53nat,width:4u32,height:3u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8,source:crate::walk_model::Source::Idat}],..s.doc},..s}
}
#[verifier::spinoff_prover]
proof fn split_apng_empty_fdat_then_data_still_step_2(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==224),
        (h==(crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:99nat,len:23nat,next:126nat,crc_ok:true,synthetic:false}))),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)},Some(2u32))==Ok(())),
    ensures m::step(b,m::strict(),false,false,split_apng_empty_fdat_then_data_still_state_2(h))==m::Step::Next(split_apng_empty_fdat_then_data_still_state_3(h)),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    let actual=match m::step(b,m::strict(),false,false,split_apng_empty_fdat_then_data_still_state_2(h)) {m::Step::Next(next)=>next,_=>split_apng_empty_fdat_then_data_still_state_2(h)};
    assert(m::step(b,m::strict(),false,false,split_apng_empty_fdat_then_data_still_state_2(h))==m::Step::Next(actual));
    let expected=split_apng_empty_fdat_then_data_still_state_3(h);
    assert(actual.pos==expected.pos);
    assert(actual.phase==expected.phase);
    assert(actual.control==expected.control);
    assert(actual.doc.info==expected.doc.info);
    assert forall|i: int| 0<=i<expected.doc.frames.len() implies #[trigger] actual.doc.frames[i]==expected.doc.frames[i] by {
        match (actual.doc.frames[i].source,expected.doc.frames[i].source) {
            (crate::walk_model::Source::Fdat(a),crate::walk_model::Source::Fdat(e))=>{assert(a=~=e);},_=>{}
        }
    }
    assert(actual.doc.frames =~= expected.doc.frames);
    assert(actual.doc.idat =~= expected.doc.idat);
}
spec fn split_apng_empty_fdat_then_data_still_state_4(h: crate::model::Ihdr) -> m::State {
    let s=m::initial(h,33,m::no_events());
    m::State {pos:164nat,seen:set![crate::walk_model::Name::Ihdr,crate::walk_model::Name::Actl],phase:crate::walk_model::Phase::After,control:crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)},
        doc:m::Doc {actl:Some((2u32,0u32)),info:m::Info {default_is_frame:true,..s.doc.info},idat:seq![crate::walk_model::Range {offset:99nat,len:23nat}],frames:seq![crate::walk_model::Frame {control_offset:53nat,width:4u32,height:3u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8,source:crate::walk_model::Source::Idat},crate::walk_model::Frame {control_offset:126nat,width:2u32,height:2u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8,source:crate::walk_model::Source::Fdat(seq![])}],..s.doc},..s}
}
#[verifier::spinoff_prover]
proof fn split_apng_empty_fdat_then_data_still_step_3(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==224),
        (h==(crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:134nat,len:26nat,next:164nat,crc_ok:true,synthetic:false}))),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}),
        (crate::apng_model::fctl(b.subrange(134,160),true,false,4u32,3u32,crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==Ok((crate::apng_model::FrameControl {width:2u32,height:2u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)}))),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)},Some(2u32))==Ok(())),
    ensures m::step(b,m::strict(),false,false,split_apng_empty_fdat_then_data_still_state_3(h))==m::Step::Next(split_apng_empty_fdat_then_data_still_state_4(h)),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::fctl);hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    let actual=match m::step(b,m::strict(),false,false,split_apng_empty_fdat_then_data_still_state_3(h)) {m::Step::Next(next)=>next,_=>split_apng_empty_fdat_then_data_still_state_3(h)};
    assert(m::step(b,m::strict(),false,false,split_apng_empty_fdat_then_data_still_state_3(h))==m::Step::Next(actual));
    let expected=split_apng_empty_fdat_then_data_still_state_4(h);
    assert(actual.pos==expected.pos);
    assert(actual.phase==expected.phase);
    assert(actual.control==expected.control);
    assert(actual.doc.info==expected.doc.info);
    assert forall|i: int| 0<=i<expected.doc.frames.len() implies #[trigger] actual.doc.frames[i]==expected.doc.frames[i] by {
        match (actual.doc.frames[i].source,expected.doc.frames[i].source) {
            (crate::walk_model::Source::Fdat(a),crate::walk_model::Source::Fdat(e))=>{assert(a=~=e);},_=>{}
        }
    }
    assert(actual.doc.frames =~= expected.doc.frames);
    assert(actual.doc.idat =~= expected.doc.idat);
}
spec fn split_apng_empty_fdat_then_data_still_state_5(h: crate::model::Ihdr) -> m::State {
    let s=m::initial(h,33,m::no_events());
    m::State {pos:180nat,seen:set![crate::walk_model::Name::Ihdr,crate::walk_model::Name::Actl],phase:crate::walk_model::Phase::After,control:crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)},
        doc:m::Doc {actl:Some((2u32,0u32)),info:m::Info {default_is_frame:true,..s.doc.info},idat:seq![crate::walk_model::Range {offset:99nat,len:23nat}],frames:seq![crate::walk_model::Frame {control_offset:53nat,width:4u32,height:3u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8,source:crate::walk_model::Source::Idat},crate::walk_model::Frame {control_offset:126nat,width:2u32,height:2u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8,source:crate::walk_model::Source::Fdat(seq![crate::walk_model::Range {offset:176nat,len:0nat}])}],..s.doc},..s}
}
#[verifier::spinoff_prover]
proof fn split_apng_empty_fdat_then_data_still_step_4(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==224),
        (h==(crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,164nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:172nat,len:4nat,next:180nat,crc_ok:true,synthetic:false}))),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}),
        (crate::apng_model::fdat(b.subrange(172,176),true,false,crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)})==Ok(crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)})),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)},Some(2u32))==Ok(())),
    ensures m::step(b,m::strict(),false,false,split_apng_empty_fdat_then_data_still_state_4(h))==m::Step::Next(split_apng_empty_fdat_then_data_still_state_5(h)),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::fdat);hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    let actual=match m::step(b,m::strict(),false,false,split_apng_empty_fdat_then_data_still_state_4(h)) {m::Step::Next(next)=>next,_=>split_apng_empty_fdat_then_data_still_state_4(h)};
    assert(m::step(b,m::strict(),false,false,split_apng_empty_fdat_then_data_still_state_4(h))==m::Step::Next(actual));
    let expected=split_apng_empty_fdat_then_data_still_state_5(h);
    assert(actual.pos==expected.pos);
    assert(actual.phase==expected.phase);
    assert(actual.control==expected.control);
    assert(actual.doc.info==expected.doc.info);
    assert forall|i: int| 0<=i<expected.doc.frames.len() implies #[trigger] actual.doc.frames[i]==expected.doc.frames[i] by {
        match (actual.doc.frames[i].source,expected.doc.frames[i].source) {
            (crate::walk_model::Source::Fdat(a),crate::walk_model::Source::Fdat(e))=>{assert(a=~=e);},_=>{}
        }
    }
    assert(actual.doc.frames =~= expected.doc.frames);
    assert(actual.doc.idat =~= expected.doc.idat);
}
spec fn split_apng_empty_fdat_then_data_still_state_6(h: crate::model::Ihdr) -> m::State {
    let s=m::initial(h,33,m::no_events());
    m::State {pos:212nat,seen:set![crate::walk_model::Name::Ihdr,crate::walk_model::Name::Actl],phase:crate::walk_model::Phase::After,control:crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)},
        doc:m::Doc {actl:Some((2u32,0u32)),info:m::Info {default_is_frame:true,..s.doc.info},idat:seq![crate::walk_model::Range {offset:99nat,len:23nat}],frames:seq![crate::walk_model::Frame {control_offset:53nat,width:4u32,height:3u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8,source:crate::walk_model::Source::Idat},crate::walk_model::Frame {control_offset:126nat,width:2u32,height:2u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8,source:crate::walk_model::Source::Fdat(seq![crate::walk_model::Range {offset:176nat,len:0nat},crate::walk_model::Range {offset:192nat,len:16nat}])}],..s.doc},..s}
}
#[verifier::spinoff_prover]
proof fn split_apng_empty_fdat_then_data_still_step_5(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==224),
        (h==(crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,180nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:188nat,len:20nat,next:212nat,crc_ok:true,synthetic:false}))),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}),
        (crate::apng_model::fdat(b.subrange(188,208),true,false,crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)})==Ok(crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)})),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)},Some(2u32))==Ok(())),
    ensures m::step(b,m::strict(),false,false,split_apng_empty_fdat_then_data_still_state_5(h))==m::Step::Next(split_apng_empty_fdat_then_data_still_state_6(h)),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::fdat);hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
    let actual=match m::step(b,m::strict(),false,false,split_apng_empty_fdat_then_data_still_state_5(h)) {m::Step::Next(next)=>next,_=>split_apng_empty_fdat_then_data_still_state_5(h)};
    assert(m::step(b,m::strict(),false,false,split_apng_empty_fdat_then_data_still_state_5(h))==m::Step::Next(actual));
    let expected=split_apng_empty_fdat_then_data_still_state_6(h);
    assert(actual.pos==expected.pos);
    assert(actual.phase==expected.phase);
    assert(actual.control==expected.control);
    assert(actual.doc.info==expected.doc.info);
    assert forall|i: int| 0<=i<expected.doc.frames.len() implies #[trigger] actual.doc.frames[i]==expected.doc.frames[i] by {
        match (actual.doc.frames[i].source,expected.doc.frames[i].source) {
            (crate::walk_model::Source::Fdat(a),crate::walk_model::Source::Fdat(e))=>{assert(a=~=e);},_=>{}
        }
    }
    assert(actual.doc.frames =~= expected.doc.frames);
    assert(actual.doc.idat =~= expected.doc.idat);
}
#[verifier::spinoff_prover]
proof fn split_apng_empty_fdat_then_data_still_step_6(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==224),
        (h==(crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,212nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:220nat,len:0nat,next:224nat,crc_ok:true,synthetic:false}))),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)},Some(2u32))==Ok(())),
    ensures m::step(b,m::strict(),false,false,split_apng_empty_fdat_then_data_still_state_6(h))==m::Step::Done(m::Doc {info:m::Info {frame_count:split_apng_empty_fdat_then_data_still_state_6(h).control.frame_count,..split_apng_empty_fdat_then_data_still_state_6(h).doc.info},..split_apng_empty_fdat_then_data_still_state_6(h).doc},m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    reveal(m::step);reveal(m::ordered);reveal(m::own);
}
#[verifier::spinoff_prover]
proof fn split_apng_empty_fdat_then_data_still_compose(b: Seq<u8>,h: crate::model::Ihdr)
    requires (b.len()==224),
        (h==(crate::model::Ihdr {width:4u32,height:3u32,depth:8u8,color_type:6u8,interlace:0u8})),
        (crate::model::mh(b)==Ok((h,33nat))),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false}))),
        ((97u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:26nat,next:91nat,crc_ok:true,synthetic:false}))),
        ((102u8 & 32u8)==32u8),
        (crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:99nat,len:23nat,next:126nat,crc_ok:true,synthetic:false}))),
        ((73u8 & 32u8)==0u8),
        (crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:134nat,len:26nat,next:164nat,crc_ok:true,synthetic:false}))),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,164nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:172nat,len:4nat,next:180nat,crc_ok:true,synthetic:false}))),
        (crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,180nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:188nat,len:20nat,next:212nat,crc_ok:true,synthetic:false}))),
        ((forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,212nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:220nat,len:0nat,next:224nat,crc_ok:true,synthetic:false}))),
        (crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend),
        (crate::model::be32(b,16)==4u32),
        (crate::model::be32(b,20)==3u32),
        (crate::apng_model::actl(b.subrange(41,49))==Ok((2u32,0u32))),
        (crate::apng_model::fctl(b.subrange(61,87),true,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Ok((crate::apng_model::FrameControl {width:4u32,height:3u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}))),
        (crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}),
        (crate::apng_model::fctl(b.subrange(134,160),true,false,4u32,3u32,crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==Ok((crate::apng_model::FrameControl {width:2u32,height:2u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)}))),
        (crate::apng_model::fdat(b.subrange(172,176),true,false,crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)})==Ok(crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)})),
        (crate::apng_model::fdat(b.subrange(188,208),true,false,crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)})==Ok(crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)})),
        (crate::apng_model::finish(crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)},Some(2u32))==Ok(())),
    ensures m::MW_p(b,m::strict())==m::Outcome::Done(doc_apng_empty_fdat_then_data_still(),m::no_events()),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::apng_model::actl);hide(crate::apng_model::fctl);hide(crate::apng_model::fdat);hide(crate::apng_model::finish);hide(crate::apng_model::begin_idat);
    hide(m::drive);
    split_apng_empty_fdat_then_data_still_step_0(b,h);
    split_apng_empty_fdat_then_data_still_step_1(b,h);
    split_apng_empty_fdat_then_data_still_step_2(b,h);
    split_apng_empty_fdat_then_data_still_step_3(b,h);
    split_apng_empty_fdat_then_data_still_step_4(b,h);
    split_apng_empty_fdat_then_data_still_step_5(b,h);
    split_apng_empty_fdat_then_data_still_step_6(b,h);
    m::drive_step(b,m::strict(),false,false,split_apng_empty_fdat_then_data_still_state_6(h));
    m::drive_step(b,m::strict(),false,false,split_apng_empty_fdat_then_data_still_state_5(h));
    m::drive_step(b,m::strict(),false,false,split_apng_empty_fdat_then_data_still_state_4(h));
    m::drive_step(b,m::strict(),false,false,split_apng_empty_fdat_then_data_still_state_3(h));
    m::drive_step(b,m::strict(),false,false,split_apng_empty_fdat_then_data_still_state_2(h));
    m::drive_step(b,m::strict(),false,false,split_apng_empty_fdat_then_data_still_state_1(h));
    m::drive_step(b,m::strict(),false,false,split_apng_empty_fdat_then_data_still_state_0(h));
}

spec fn doc_apng_empty_fdat_then_data_still() -> m::Doc {let s=split_apng_empty_fdat_then_data_still_state_6(h_apng_empty_fdat_then_data_still());m::Doc {info:m::Info {frame_count:s.control.frame_count,..s.doc.info},..s.doc}}
#[verifier::spinoff_prover]
proof fn walk_apng_empty_fdat_then_data_still() ensures m::MW_p(bytes_apng_empty_fdat_then_data_still(),m::strict())==m::Outcome::Done(doc_apng_empty_fdat_then_data_still(),m::no_events()),
{
    hide(bytes_apng_empty_fdat_then_data_still);hide(m::MW_p);hide(crate::critical_model::read);hide(crate::walk_model::classify);
    assert(bytes_apng_empty_fdat_then_data_still().len()==224nat) by (compute);
    assert(crate::model::mh(bytes_apng_empty_fdat_then_data_still())==Ok((h_apng_empty_fdat_then_data_still(),33nat))) by {reveal(crate::model::mh);assert(crate::model::mh(bytes_apng_empty_fdat_then_data_still())==Ok((h_apng_empty_fdat_then_data_still(),33nat))) by (compute);}
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_empty_fdat_then_data_still(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_empty_fdat_then_data_still(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,99u8,84u8,76u8],data:41nat,len:8nat,next:53nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((97u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![97u8,99u8,84u8,76u8])==crate::walk_model::Name::Actl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_empty_fdat_then_data_still().subrange(41,49),0)==2) by (compute);
        assert(crate::model::be32(bytes_apng_empty_fdat_then_data_still().subrange(41,49),4)==0) by (compute);
        assert(2u32<=crate::limits::MAX_FRAMES) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_empty_fdat_then_data_still(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:26nat,next:91nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_empty_fdat_then_data_still(),53nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:61nat,len:26nat,next:91nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_empty_fdat_then_data_still().subrange(61,87),0)==0) by (compute);
        assert(crate::model::be32(bytes_apng_empty_fdat_then_data_still().subrange(61,87),4)==4) by (compute);
        assert(crate::model::be32(bytes_apng_empty_fdat_then_data_still().subrange(61,87),8)==3) by (compute);
        assert(crate::model::be32(bytes_apng_empty_fdat_then_data_still().subrange(61,87),12)==0) by (compute);
        assert(crate::model::be32(bytes_apng_empty_fdat_then_data_still().subrange(61,87),16)==0) by (compute);
        assert((4u64)*(3u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_empty_fdat_then_data_still(),91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:99nat,len:23nat,next:126nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_empty_fdat_then_data_still(),91nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:99nat,len:23nat,next:126nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_empty_fdat_then_data_still(),126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:134nat,len:26nat,next:164nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_empty_fdat_then_data_still(),126nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,99u8,84u8,76u8],data:134nat,len:26nat,next:164nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,99u8,84u8,76u8])==crate::walk_model::Name::Fctl) by (compute);
        }
        assert(crate::model::be32(bytes_apng_empty_fdat_then_data_still().subrange(134,160),0)==1) by (compute);
        assert(crate::model::be32(bytes_apng_empty_fdat_then_data_still().subrange(134,160),4)==2) by (compute);
        assert(crate::model::be32(bytes_apng_empty_fdat_then_data_still().subrange(134,160),8)==2) by (compute);
        assert(crate::model::be32(bytes_apng_empty_fdat_then_data_still().subrange(134,160),12)==0) by (compute);
        assert(crate::model::be32(bytes_apng_empty_fdat_then_data_still().subrange(134,160),16)==0) by (compute);
        assert((2u64)*(2u64)<=crate::limits::MAX_TOTAL_PIXELS) by (bit_vector);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_empty_fdat_then_data_still(),164nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:172nat,len:4nat,next:180nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_empty_fdat_then_data_still(),164nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:172nat,len:4nat,next:180nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by (compute);
        }
        assert(crate::model::be32(bytes_apng_empty_fdat_then_data_still().subrange(172,176),0)==2) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_empty_fdat_then_data_still(),180nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:188nat,len:20nat,next:212nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_empty_fdat_then_data_still(),180nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![102u8,100u8,65u8,84u8],data:188nat,len:20nat,next:212nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((102u8 & 32u8)==32u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![102u8,100u8,65u8,84u8])==crate::walk_model::Name::Fdat) by (compute);
        }
        assert(crate::model::be32(bytes_apng_empty_fdat_then_data_still().subrange(188,208),0)==3) by (compute);
        assert(crate::model::be32(bytes_apng_empty_fdat_then_data_still().subrange(188,208),4)==2023514976) by (compute);
        assert(crate::model::be32(bytes_apng_empty_fdat_then_data_still().subrange(188,208),8)==4161765772) by (compute);
        assert(crate::model::be32(bytes_apng_empty_fdat_then_data_still().subrange(188,208),12)==1611270144) by (compute);
        assert(crate::model::be32(bytes_apng_empty_fdat_then_data_still().subrange(188,208),16)==1071318524) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_apng_empty_fdat_then_data_still(),212nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:220nat,len:0nat,next:224nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_apng_empty_fdat_then_data_still(),212nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:220nat,len:0nat,next:224nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
        assert(crate::model::be32(bytes_apng_empty_fdat_then_data_still(),16)==4u32) by (compute);
        assert(crate::model::be32(bytes_apng_empty_fdat_then_data_still(),20)==3u32) by (compute);
        assert(crate::apng_model::actl(bytes_apng_empty_fdat_then_data_still().subrange(41,49))==Ok((2u32,0u32))) by {
            reveal(crate::apng_model::actl);
            assert(crate::apng_model::actl(bytes_apng_empty_fdat_then_data_still().subrange(41,49))==Ok((2u32,0u32))) by (compute);
        }
        assert(crate::apng_model::fctl(bytes_apng_empty_fdat_then_data_still().subrange(61,87),true,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Ok((crate::apng_model::FrameControl {width:4u32,height:3u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}))) by {
            reveal(crate::apng_model::fctl);
            assert(crate::apng_model::fctl(bytes_apng_empty_fdat_then_data_still().subrange(61,87),true,true,4u32,3u32,crate::apng_model::Control {sequence:0nat,current:crate::apng_model::Current::None,frame_count:0nat,fctl_before_idat:false,pixel_total:0nat})==Ok((crate::apng_model::FrameControl {width:4u32,height:3u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}))) by (compute);
        }
        assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}) by {
            reveal(crate::apng_model::begin_idat);
            assert(crate::apng_model::begin_idat(crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Frame(false),frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)}) by (compute);
        }
        assert(crate::apng_model::fctl(bytes_apng_empty_fdat_then_data_still().subrange(134,160),true,false,4u32,3u32,crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==Ok((crate::apng_model::FrameControl {width:2u32,height:2u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)}))) by {
            reveal(crate::apng_model::fctl);
            assert(crate::apng_model::fctl(bytes_apng_empty_fdat_then_data_still().subrange(134,160),true,false,4u32,3u32,crate::apng_model::Control {sequence:1nat,current:crate::apng_model::Current::Default,frame_count:1nat,fctl_before_idat:true,pixel_total:(4nat*3nat)})==Ok((crate::apng_model::FrameControl {width:2u32,height:2u32,x:0u32,y:0u32,delay_num:1u16,delay_den:10u16,dispose:0u8,blend:0u8},crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)}))) by (compute);
        }
        assert(crate::apng_model::fdat(bytes_apng_empty_fdat_then_data_still().subrange(172,176),true,false,crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)})==Ok(crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)})) by {
            reveal(crate::apng_model::fdat);
            assert(crate::apng_model::fdat(bytes_apng_empty_fdat_then_data_still().subrange(172,176),true,false,crate::apng_model::Control {sequence:2nat,current:crate::apng_model::Current::Frame(false),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)})==Ok(crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)})) by (compute);
        }
        assert(crate::apng_model::fdat(bytes_apng_empty_fdat_then_data_still().subrange(188,208),true,false,crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)})==Ok(crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)})) by {
            reveal(crate::apng_model::fdat);
            assert(crate::apng_model::fdat(bytes_apng_empty_fdat_then_data_still().subrange(188,208),true,false,crate::apng_model::Control {sequence:3nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)})==Ok(crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)})) by (compute);
        }
        assert(crate::apng_model::finish(crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)},Some(2u32))==Ok(())) by {
            reveal(crate::apng_model::finish);
            assert(crate::apng_model::finish(crate::apng_model::Control {sequence:4nat,current:crate::apng_model::Current::Frame(true),frame_count:2nat,fctl_before_idat:true,pixel_total:(4nat*3nat)+(2nat*2nat)},Some(2u32))==Ok(())) by (compute);
        }
    split_apng_empty_fdat_then_data_still_compose(bytes_apng_empty_fdat_then_data_still(),h_apng_empty_fdat_then_data_still());
}
#[verifier::spinoff_prover]
proof fn project_walk_apng_empty_fdat_then_data_still() ensures r::default_case(bytes_apng_empty_fdat_then_data_still(),h_apng_empty_fdat_then_data_still(),z_apng_empty_fdat_then_data_still(),0nat),
{
    walk_apng_empty_fdat_then_data_still();
    assert(m::entries(doc_apng_empty_fdat_then_data_still())==0nat) by (compute);
    assert(m::default_stream(bytes_apng_empty_fdat_then_data_still(),doc_apng_empty_fdat_then_data_still())==Some(z_apng_empty_fdat_then_data_still())) by (compute);
}
#[verifier::spinoff_prover]
proof fn scan_apng_empty_fdat_then_data_still() ensures r::scan(raw_apng_empty_fdat_then_data_still(),a_apng_empty_fdat_then_data_still(),0)==Ok(()),
{
    hide(raw_apng_empty_fdat_then_data_still);assert(raw_apng_empty_fdat_then_data_still().len()==51nat) by (compute);assert(r::count(a_apng_empty_fdat_then_data_still())==1nat) by (compute);
    assert(r::pw(a_apng_empty_fdat_then_data_still(),0nat)==4nat && r::ph(a_apng_empty_fdat_then_data_still(),0nat)==3nat && r::rb(a_apng_empty_fdat_then_data_still(),0nat)==16nat && r::base(a_apng_empty_fdat_then_data_still(),0nat)==0nat) by (compute);
    assert(raw_apng_empty_fdat_then_data_still()[0]==0u8) by (compute);
    assert(r::row_result(raw_apng_empty_fdat_then_data_still(),a_apng_empty_fdat_then_data_still(),0nat,0nat)==Ok(()));
    assert(raw_apng_empty_fdat_then_data_still()[17]==1u8) by (compute);
    assert(r::row_result(raw_apng_empty_fdat_then_data_still(),a_apng_empty_fdat_then_data_still(),0nat,1nat)==Ok(()));
    assert(raw_apng_empty_fdat_then_data_still()[34]==2u8) by (compute);
    assert(r::row_result(raw_apng_empty_fdat_then_data_still(),a_apng_empty_fdat_then_data_still(),0nat,2nat)==Ok(()));
    r::scan_step(raw_apng_empty_fdat_then_data_still(),a_apng_empty_fdat_then_data_still(),1nat);
    r::pass_step(raw_apng_empty_fdat_then_data_still(),a_apng_empty_fdat_then_data_still(),0nat,3nat);
    r::pass_step(raw_apng_empty_fdat_then_data_still(),a_apng_empty_fdat_then_data_still(),0nat,2nat);
    r::pass_step(raw_apng_empty_fdat_then_data_still(),a_apng_empty_fdat_then_data_still(),0nat,1nat);
    r::pass_step(raw_apng_empty_fdat_then_data_still(),a_apng_empty_fdat_then_data_still(),0nat,0nat);
    r::scan_step(raw_apng_empty_fdat_then_data_still(),a_apng_empty_fdat_then_data_still(),0nat);
}
#[verifier::spinoff_prover]
proof fn model_apng_empty_fdat_then_data_still() ensures r::still(bytes_apng_empty_fdat_then_data_still(),STRICT,false)==(r::Projection::Ready {width:4u32,height:3u32,bytes:48nat}),
{
    hide(bytes_apng_empty_fdat_then_data_still);hide(z_apng_empty_fdat_then_data_still);hide(raw_apng_empty_fdat_then_data_still);hide(r::data);

    assert(bytes_apng_empty_fdat_then_data_still().len()==224nat) by (compute);
    project_walk_apng_empty_fdat_then_data_still();
    inflate_apng_empty_fdat_then_data_still();scan_apng_empty_fdat_then_data_still();
    assert(r::raw(a_apng_empty_fdat_then_data_still())==51nat) by (compute);
    assert(r::data(z_apng_empty_fdat_then_data_still(),a_apng_empty_fdat_then_data_still(),STRICT)==Some(Ok(()))) by {reveal(r::data);};
    r::default_ready(bytes_apng_empty_fdat_then_data_still(),h_apng_empty_fdat_then_data_still(),z_apng_empty_fdat_then_data_still(),0nat,false);
}
fn public_apng_empty_fdat_then_data_still() {
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,8u8,6u8,0u8,0u8,0u8,180u8,244u8,174u8,198u8,0u8,0u8,0u8,8u8,97u8,99u8,84u8,76u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,243u8,141u8,147u8,112u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,4u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,204u8,115u8,12u8,251u8,0u8,0u8,0u8,23u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,248u8,207u8,192u8,240u8,31u8,25u8,51u8,130u8,8u8,6u8,36u8,192u8,196u8,128u8,6u8,0u8,145u8,243u8,9u8,250u8,109u8,246u8,58u8,149u8,0u8,0u8,0u8,26u8,102u8,99u8,84u8,76u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,0u8,10u8,0u8,0u8,115u8,39u8,54u8,212u8,0u8,0u8,0u8,4u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,2u8,235u8,198u8,207u8,77u8,0u8,0u8,0u8,20u8,102u8,100u8,65u8,84u8,0u8,0u8,0u8,3u8,120u8,156u8,99u8,96u8,248u8,15u8,129u8,140u8,96u8,10u8,8u8,0u8,63u8,219u8,5u8,252u8,210u8,80u8,13u8,207u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_apng_empty_fdat_then_data_still());model_apng_empty_fdat_then_data_still();}
    let result=crate::png::try_decode_still(&b);
    assert(match result {Ok(s)=>s.width==4u32 && s.height==3u32 && s.rgba@.len()==48nat,Err(crate::alloc_guard::TryError::OutOfMemory(_))=>true,_=>false});
}
} // verus!
