//! Whole-walk/preparation projections from authored frozen fixtures.
//! M3 projection only; final data outcome is an M4–M6 witness obligation, runtime-conformance-tested meanwhile.
use vstd::prelude::*;
use crate::{document_model as m,png::Events,policy::STRICT};

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
spec fn accepted(outcome: m::Outcome,h: crate::model::Ihdr) -> bool {match outcome {m::Outcome::Done(doc,ev)=>doc.ihdr==h && ev==m::no_events(),_=>false}}
#[verifier::spinoff_prover]
proof fn symbolic_container_minimal_ok(b: Seq<u8>)
    requires b.len()==79,crate::model::mh(b)==Ok((crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:22nat,next:67nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,67nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:75nat,len:0nat,next:79nat,crc_ok:true,synthetic:false})),
    ensures accepted(m::MW_p(b,m::strict()),crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8}),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);


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
}

// container/minimal-ok; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_container_minimal_ok() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn accepted_container_minimal_ok() -> bool {accepted(m::MW_p(bytes_container_minimal_ok(),m::strict()),crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8})}
#[verifier::spinoff_prover]
proof fn model_container_minimal_ok()
    ensures accepted_container_minimal_ok(),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(m::MW_p);


    assert(crate::model::mh(bytes_container_minimal_ok())==Ok((crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_container_minimal_ok())==Ok((crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8},33nat))) by (compute);
    }
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
    symbolic_container_minimal_ok(bytes_container_minimal_ok());
}
// M3 projection only; final data outcome is an M4–M6 witness obligation, runtime-conformance-tested meanwhile
fn projection_container_minimal_ok() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_container_minimal_ok());model_container_minimal_ok();}
    let mut events=Events {r1:false,r3:false,r4:false};
    let prepared=crate::callers::still(&b,&STRICT,&mut events);
    match prepared {
        Ok(doc)=>{assert(doc.ihdr.view()==crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8});
            assert(doc.info.ranges_wf(b.len() as nat)) by {reveal(crate::walk::Doc::wf);}
        },
        Err(_)=>{assert(false);},
    }
    // The actual public call has no asserted final data/pixel outcome.
    let _public_result=crate::png::try_decode_still(&b);
}

#[verifier::spinoff_prover]
proof fn symbolic_container_unknown_ancillary_everywhere(b: Seq<u8>)
    requires b.len()==106,crate::model::mh(b)==Ok((crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,98u8,67u8,100u8],data:41nat,len:3nat,next:48nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,48nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:56nat,len:22nat,next:82nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,82nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![97u8,98u8,99u8,100u8],data:90nat,len:0nat,next:94nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,94nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:102nat,len:0nat,next:106nat,crc_ok:true,synthetic:false})),
    ensures accepted(m::MW_p(b,m::strict()),crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8}),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);


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
}

// container/unknown-ancillary-everywhere; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_container_unknown_ancillary_everywhere() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,3u8,97u8,98u8,67u8,100u8,120u8,121u8,122u8,74u8,73u8,23u8,198u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,97u8,98u8,99u8,100u8,237u8,130u8,205u8,17u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn accepted_container_unknown_ancillary_everywhere() -> bool {accepted(m::MW_p(bytes_container_unknown_ancillary_everywhere(),m::strict()),crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8})}
#[verifier::spinoff_prover]
proof fn model_container_unknown_ancillary_everywhere()
    ensures accepted_container_unknown_ancillary_everywhere(),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(m::MW_p);


    assert(crate::model::mh(bytes_container_unknown_ancillary_everywhere())==Ok((crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_container_unknown_ancillary_everywhere())==Ok((crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8},33nat))) by (compute);
    }
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
    symbolic_container_unknown_ancillary_everywhere(bytes_container_unknown_ancillary_everywhere());
}
// M3 projection only; final data outcome is an M4–M6 witness obligation, runtime-conformance-tested meanwhile
fn projection_container_unknown_ancillary_everywhere() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,3u8,97u8,98u8,67u8,100u8,120u8,121u8,122u8,74u8,73u8,23u8,198u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,97u8,98u8,99u8,100u8,237u8,130u8,205u8,17u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_container_unknown_ancillary_everywhere());model_container_unknown_ancillary_everywhere();}
    let mut events=Events {r1:false,r3:false,r4:false};
    let prepared=crate::callers::still(&b,&STRICT,&mut events);
    match prepared {
        Ok(doc)=>{assert(doc.ihdr.view()==crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8});
            assert(doc.info.ranges_wf(b.len() as nat)) by {reveal(crate::walk::Doc::wf);}
        },
        Err(_)=>{assert(false);},
    }
    // The actual public call has no asserted final data/pixel outcome.
    let _public_result=crate::png::try_decode_still(&b);
}

#[verifier::spinoff_prover]
proof fn symbolic_container_reserved_bit_ancillary(b: Seq<u8>)
    requires b.len()==92,crate::model::mh(b)==Ok((crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![122u8,122u8,122u8,122u8],data:41nat,len:1nat,next:46nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,46nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:54nat,len:22nat,next:80nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,80nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:88nat,len:0nat,next:92nat,crc_ok:true,synthetic:false})),
    ensures accepted(m::MW_p(b,m::strict()),crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8}),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);


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
}

// container/reserved-bit-ancillary; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_container_reserved_bit_ancillary() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,1u8,122u8,122u8,122u8,122u8,49u8,172u8,170u8,51u8,75u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn accepted_container_reserved_bit_ancillary() -> bool {accepted(m::MW_p(bytes_container_reserved_bit_ancillary(),m::strict()),crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8})}
#[verifier::spinoff_prover]
proof fn model_container_reserved_bit_ancillary()
    ensures accepted_container_reserved_bit_ancillary(),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(m::MW_p);


    assert(crate::model::mh(bytes_container_reserved_bit_ancillary())==Ok((crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_container_reserved_bit_ancillary())==Ok((crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8},33nat))) by (compute);
    }
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
    symbolic_container_reserved_bit_ancillary(bytes_container_reserved_bit_ancillary());
}
// M3 projection only; final data outcome is an M4–M6 witness obligation, runtime-conformance-tested meanwhile
fn projection_container_reserved_bit_ancillary() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,1u8,122u8,122u8,122u8,122u8,49u8,172u8,170u8,51u8,75u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_container_reserved_bit_ancillary());model_container_reserved_bit_ancillary();}
    let mut events=Events {r1:false,r3:false,r4:false};
    let prepared=crate::callers::still(&b,&STRICT,&mut events);
    match prepared {
        Ok(doc)=>{assert(doc.ihdr.view()==crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8});
            assert(doc.info.ranges_wf(b.len() as nat)) by {reveal(crate::walk::Doc::wf);}
        },
        Err(_)=>{assert(false);},
    }
    // The actual public call has no asserted final data/pixel outcome.
    let _public_result=crate::png::try_decode_still(&b);
}

#[verifier::spinoff_prover]
proof fn symbolic_plte_ok(b: Seq<u8>)
    requires b.len()==92,crate::model::mh(b)==Ok((crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:3u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:9nat,next:54nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,54nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:62nat,len:14nat,next:80nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,80nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:88nat,len:0nat,next:92nat,crc_ok:true,synthetic:false})),
    ensures accepted(m::MW_p(b,m::strict()),crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:3u8,interlace:0u8}),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);


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
}

// plte/ok; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_plte_ok() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,3u8,0u8,0u8,0u8,69u8,104u8,253u8,22u8,0u8,0u8,0u8,9u8,80u8,76u8,84u8,69u8,47u8,194u8,40u8,139u8,44u8,88u8,228u8,36u8,202u8,64u8,183u8,32u8,36u8,0u8,0u8,0u8,14u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,96u8,100u8,100u8,250u8,15u8,0u8,1u8,16u8,1u8,4u8,216u8,172u8,32u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn accepted_plte_ok() -> bool {accepted(m::MW_p(bytes_plte_ok(),m::strict()),crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:3u8,interlace:0u8})}
#[verifier::spinoff_prover]
proof fn model_plte_ok()
    ensures accepted_plte_ok(),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(m::MW_p);


    assert(crate::model::mh(bytes_plte_ok())==Ok((crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:3u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_plte_ok())==Ok((crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:3u8,interlace:0u8},33nat))) by (compute);
    }
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
    symbolic_plte_ok(bytes_plte_ok());
}
// M3 projection only; final data outcome is an M4–M6 witness obligation, runtime-conformance-tested meanwhile
fn projection_plte_ok() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,3u8,0u8,0u8,0u8,69u8,104u8,253u8,22u8,0u8,0u8,0u8,9u8,80u8,76u8,84u8,69u8,47u8,194u8,40u8,139u8,44u8,88u8,228u8,36u8,202u8,64u8,183u8,32u8,36u8,0u8,0u8,0u8,14u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,96u8,96u8,100u8,100u8,250u8,15u8,0u8,1u8,16u8,1u8,4u8,216u8,172u8,32u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_plte_ok());model_plte_ok();}
    let mut events=Events {r1:false,r3:false,r4:false};
    let prepared=crate::callers::still(&b,&STRICT,&mut events);
    match prepared {
        Ok(doc)=>{assert(doc.ihdr.view()==crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:3u8,interlace:0u8});
            assert(doc.info.ranges_wf(b.len() as nat)) by {reveal(crate::walk::Doc::wf);}
        },
        Err(_)=>{assert(false);},
    }
    // The actual public call has no asserted final data/pixel outcome.
    let _public_result=crate::png::try_decode_still(&b);
}

#[verifier::spinoff_prover]
proof fn symbolic_plte_suggested_palette_in_rgb(b: Seq<u8>)
    requires b.len()==1121,crate::model::mh(b)==Ok((crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8},33nat)),
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
    ensures accepted(m::MW_p(b,m::strict()),crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8}),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::metadata_model::parse);

    hide(m::store);
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
    let ghost s0=m::initial(crate::model::mh(b).unwrap().0,33,m::no_events());
    let ghost s1=match m::step(b,m::strict(),false,false,s0) {m::Step::Next(next)=>next,_=>s0};
    assert(m::step(b,m::strict(),false,false,s0)==m::Step::Next(s1));
    m::step_progress(b,m::strict(),false,false,s0);
    let ghost s2=match m::step(b,m::strict(),false,false,s1) {m::Step::Next(next)=>next,_=>s1};
    assert(m::step(b,m::strict(),false,false,s1)==m::Step::Next(s2));
    m::step_progress(b,m::strict(),false,false,s1);
    let ghost s3=match m::step(b,m::strict(),false,false,s2) {m::Step::Next(next)=>next,_=>s2};
    assert(m::step(b,m::strict(),false,false,s2)==m::Step::Next(s3));
    m::step_progress(b,m::strict(),false,false,s2);
    let ghost s4=match m::step(b,m::strict(),false,false,s3) {m::Step::Next(next)=>next,_=>s3};
    assert(m::step(b,m::strict(),false,false,s3)==m::Step::Next(s4));
    m::step_progress(b,m::strict(),false,false,s3);
    assert(match m::step(b,m::strict(),false,false,s4) {m::Step::Done(doc,ev)=>doc.ihdr==crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8} && ev==m::no_events(),_=>false});
    m::drive_step(b,m::strict(),false,false,s4);
    assert(accepted(m::drive(b,m::strict(),false,false,s4),crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8}));
    m::drive_step(b,m::strict(),false,false,s3);
    assert(accepted(m::drive(b,m::strict(),false,false,s3),crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8}));
    m::drive_step(b,m::strict(),false,false,s2);
    assert(accepted(m::drive(b,m::strict(),false,false,s2),crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8}));
    m::drive_step(b,m::strict(),false,false,s1);
    assert(accepted(m::drive(b,m::strict(),false,false,s1),crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8}));
    m::drive_step(b,m::strict(),false,false,s0);
    assert(accepted(m::drive(b,m::strict(),false,false,s0),crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8}));
}

// plte/suggested-palette-in-rgb; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_plte_suggested_palette_in_rgb() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,2u8,88u8,80u8,76u8,84u8,69u8,217u8,5u8,126u8,229u8,40u8,28u8,92u8,184u8,223u8,213u8,121u8,238u8,109u8,36u8,77u8,4u8,167u8,27u8,166u8,48u8,111u8,49u8,91u8,221u8,89u8,53u8,190u8,63u8,235u8,76u8,90u8,66u8,129u8,96u8,87u8,99u8,39u8,127u8,211u8,1u8,24u8,51u8,79u8,48u8,95u8,25u8,251u8,236u8,141u8,58u8,166u8,39u8,245u8,176u8,238u8,21u8,176u8,218u8,3u8,95u8,44u8,200u8,190u8,150u8,158u8,3u8,203u8,176u8,192u8,177u8,86u8,198u8,222u8,55u8,22u8,49u8,216u8,137u8,191u8,32u8,64u8,11u8,236u8,238u8,221u8,93u8,106u8,153u8,119u8,70u8,160u8,70u8,148u8,150u8,144u8,89u8,193u8,82u8,205u8,5u8,90u8,208u8,139u8,82u8,184u8,18u8,158u8,37u8,15u8,12u8,185u8,71u8,81u8,180u8,11u8,151u8,95u8,249u8,148u8,227u8,205u8,172u8,235u8,189u8,229u8,52u8,2u8,44u8,166u8,182u8,186u8,227u8,37u8,103u8,130u8,94u8,104u8,184u8,196u8,230u8,13u8,127u8,235u8,164u8,217u8,129u8,6u8,224u8,233u8,236u8,72u8,246u8,169u8,246u8,62u8,80u8,88u8,219u8,152u8,231u8,189u8,105u8,160u8,27u8,215u8,164u8,173u8,35u8,140u8,75u8,214u8,92u8,33u8,84u8,15u8,83u8,66u8,220u8,58u8,109u8,174u8,187u8,145u8,26u8,93u8,11u8,244u8,15u8,150u8,157u8,165u8,108u8,14u8,34u8,101u8,70u8,42u8,210u8,68u8,224u8,234u8,58u8,144u8,7u8,8u8,200u8,112u8,114u8,9u8,227u8,95u8,168u8,194u8,121u8,243u8,15u8,246u8,20u8,171u8,24u8,54u8,128u8,113u8,96u8,45u8,80u8,75u8,94u8,243u8,169u8,254u8,241u8,217u8,214u8,102u8,180u8,124u8,54u8,47u8,156u8,179u8,173u8,102u8,209u8,90u8,28u8,66u8,17u8,60u8,126u8,113u8,193u8,193u8,66u8,150u8,139u8,200u8,94u8,162u8,111u8,143u8,200u8,56u8,253u8,248u8,90u8,238u8,150u8,233u8,78u8,42u8,134u8,186u8,216u8,70u8,219u8,58u8,151u8,73u8,56u8,238u8,38u8,222u8,47u8,178u8,181u8,4u8,136u8,214u8,241u8,102u8,191u8,39u8,49u8,55u8,195u8,188u8,50u8,105u8,210u8,89u8,27u8,49u8,27u8,141u8,39u8,245u8,231u8,35u8,162u8,106u8,53u8,66u8,44u8,203u8,2u8,112u8,205u8,234u8,232u8,133u8,0u8,87u8,145u8,15u8,0u8,125u8,215u8,173u8,105u8,7u8,91u8,227u8,195u8,221u8,8u8,191u8,160u8,132u8,214u8,106u8,143u8,215u8,2u8,98u8,88u8,220u8,115u8,213u8,193u8,166u8,32u8,233u8,198u8,77u8,22u8,163u8,234u8,97u8,166u8,220u8,183u8,61u8,91u8,16u8,231u8,70u8,168u8,57u8,36u8,111u8,154u8,180u8,231u8,115u8,221u8,117u8,210u8,228u8,244u8,152u8,199u8,195u8,219u8,60u8,15u8,195u8,114u8,200u8,222u8,241u8,5u8,154u8,61u8,251u8,237u8,87u8,7u8,200u8,90u8,30u8,146u8,116u8,162u8,211u8,33u8,75u8,247u8,91u8,245u8,163u8,252u8,212u8,218u8,231u8,68u8,20u8,108u8,86u8,62u8,179u8,124u8,34u8,164u8,253u8,8u8,165u8,157u8,17u8,198u8,157u8,168u8,130u8,98u8,106u8,84u8,83u8,32u8,16u8,240u8,55u8,94u8,26u8,86u8,170u8,69u8,120u8,114u8,244u8,241u8,223u8,14u8,212u8,227u8,219u8,218u8,82u8,90u8,212u8,102u8,200u8,191u8,175u8,62u8,1u8,118u8,236u8,85u8,111u8,228u8,181u8,36u8,156u8,216u8,90u8,236u8,19u8,246u8,19u8,182u8,40u8,196u8,221u8,94u8,247u8,11u8,241u8,38u8,236u8,90u8,45u8,63u8,248u8,237u8,144u8,82u8,35u8,143u8,89u8,30u8,100u8,154u8,194u8,162u8,16u8,140u8,82u8,88u8,69u8,53u8,106u8,170u8,117u8,126u8,213u8,72u8,33u8,243u8,176u8,253u8,209u8,23u8,227u8,0u8,194u8,26u8,251u8,229u8,54u8,130u8,184u8,87u8,126u8,228u8,255u8,39u8,216u8,35u8,81u8,111u8,215u8,222u8,86u8,250u8,173u8,226u8,86u8,44u8,120u8,253u8,66u8,1u8,217u8,1u8,43u8,145u8,75u8,132u8,77u8,196u8,237u8,112u8,162u8,123u8,156u8,80u8,214u8,90u8,240u8,154u8,205u8,49u8,148u8,210u8,253u8,245u8,117u8,255u8,99u8,48u8,82u8,20u8,48u8,201u8,131u8,237u8,156u8,171u8,200u8,8u8,88u8,124u8,80u8,131u8,125u8,108u8,63u8,37u8,245u8,67u8,214u8,20u8,107u8,218u8,0u8,0u8,0u8,6u8,98u8,75u8,71u8,68u8,0u8,1u8,0u8,2u8,0u8,3u8,94u8,174u8,23u8,27u8,0u8,0u8,1u8,144u8,104u8,73u8,83u8,84u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,93u8,124u8,21u8,200u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn accepted_plte_suggested_palette_in_rgb() -> bool {accepted(m::MW_p(bytes_plte_suggested_palette_in_rgb(),m::strict()),crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8})}
#[verifier::spinoff_prover]
proof fn model_plte_suggested_palette_in_rgb()
    ensures accepted_plte_suggested_palette_in_rgb(),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(m::MW_p);
    hide(crate::metadata_model::parse);

    assert(crate::model::mh(bytes_plte_suggested_palette_in_rgb())==Ok((crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_plte_suggested_palette_in_rgb())==Ok((crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_plte_suggested_palette_in_rgb(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:600nat,next:645nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_plte_suggested_palette_in_rgb(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![80u8,76u8,84u8,69u8],data:41nat,len:600nat,next:645nat,crc_ok:true,synthetic:false})) by (compute);
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
            assert(crate::critical_model::read(bytes_plte_suggested_palette_in_rgb(),663nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![104u8,73u8,83u8,84u8],data:671nat,len:400nat,next:1075nat,crc_ok:true,synthetic:false})) by (compute);
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
    symbolic_plte_suggested_palette_in_rgb(bytes_plte_suggested_palette_in_rgb());
}
// M3 projection only; final data outcome is an M4–M6 witness obligation, runtime-conformance-tested meanwhile
fn projection_plte_suggested_palette_in_rgb() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,2u8,88u8,80u8,76u8,84u8,69u8,217u8,5u8,126u8,229u8,40u8,28u8,92u8,184u8,223u8,213u8,121u8,238u8,109u8,36u8,77u8,4u8,167u8,27u8,166u8,48u8,111u8,49u8,91u8,221u8,89u8,53u8,190u8,63u8,235u8,76u8,90u8,66u8,129u8,96u8,87u8,99u8,39u8,127u8,211u8,1u8,24u8,51u8,79u8,48u8,95u8,25u8,251u8,236u8,141u8,58u8,166u8,39u8,245u8,176u8,238u8,21u8,176u8,218u8,3u8,95u8,44u8,200u8,190u8,150u8,158u8,3u8,203u8,176u8,192u8,177u8,86u8,198u8,222u8,55u8,22u8,49u8,216u8,137u8,191u8,32u8,64u8,11u8,236u8,238u8,221u8,93u8,106u8,153u8,119u8,70u8,160u8,70u8,148u8,150u8,144u8,89u8,193u8,82u8,205u8,5u8,90u8,208u8,139u8,82u8,184u8,18u8,158u8,37u8,15u8,12u8,185u8,71u8,81u8,180u8,11u8,151u8,95u8,249u8,148u8,227u8,205u8,172u8,235u8,189u8,229u8,52u8,2u8,44u8,166u8,182u8,186u8,227u8,37u8,103u8,130u8,94u8,104u8,184u8,196u8,230u8,13u8,127u8,235u8,164u8,217u8,129u8,6u8,224u8,233u8,236u8,72u8,246u8,169u8,246u8,62u8,80u8,88u8,219u8,152u8,231u8,189u8,105u8,160u8,27u8,215u8,164u8,173u8,35u8,140u8,75u8,214u8,92u8,33u8,84u8,15u8,83u8,66u8,220u8,58u8,109u8,174u8,187u8,145u8,26u8,93u8,11u8,244u8,15u8,150u8,157u8,165u8,108u8,14u8,34u8,101u8,70u8,42u8,210u8,68u8,224u8,234u8,58u8,144u8,7u8,8u8,200u8,112u8,114u8,9u8,227u8,95u8,168u8,194u8,121u8,243u8,15u8,246u8,20u8,171u8,24u8,54u8,128u8,113u8,96u8,45u8,80u8,75u8,94u8,243u8,169u8,254u8,241u8,217u8,214u8,102u8,180u8,124u8,54u8,47u8,156u8,179u8,173u8,102u8,209u8,90u8,28u8,66u8,17u8,60u8,126u8,113u8,193u8,193u8,66u8,150u8,139u8,200u8,94u8,162u8,111u8,143u8,200u8,56u8,253u8,248u8,90u8,238u8,150u8,233u8,78u8,42u8,134u8,186u8,216u8,70u8,219u8,58u8,151u8,73u8,56u8,238u8,38u8,222u8,47u8,178u8,181u8,4u8,136u8,214u8,241u8,102u8,191u8,39u8,49u8,55u8,195u8,188u8,50u8,105u8,210u8,89u8,27u8,49u8,27u8,141u8,39u8,245u8,231u8,35u8,162u8,106u8,53u8,66u8,44u8,203u8,2u8,112u8,205u8,234u8,232u8,133u8,0u8,87u8,145u8,15u8,0u8,125u8,215u8,173u8,105u8,7u8,91u8,227u8,195u8,221u8,8u8,191u8,160u8,132u8,214u8,106u8,143u8,215u8,2u8,98u8,88u8,220u8,115u8,213u8,193u8,166u8,32u8,233u8,198u8,77u8,22u8,163u8,234u8,97u8,166u8,220u8,183u8,61u8,91u8,16u8,231u8,70u8,168u8,57u8,36u8,111u8,154u8,180u8,231u8,115u8,221u8,117u8,210u8,228u8,244u8,152u8,199u8,195u8,219u8,60u8,15u8,195u8,114u8,200u8,222u8,241u8,5u8,154u8,61u8,251u8,237u8,87u8,7u8,200u8,90u8,30u8,146u8,116u8,162u8,211u8,33u8,75u8,247u8,91u8,245u8,163u8,252u8,212u8,218u8,231u8,68u8,20u8,108u8,86u8,62u8,179u8,124u8,34u8,164u8,253u8,8u8,165u8,157u8,17u8,198u8,157u8,168u8,130u8,98u8,106u8,84u8,83u8,32u8,16u8,240u8,55u8,94u8,26u8,86u8,170u8,69u8,120u8,114u8,244u8,241u8,223u8,14u8,212u8,227u8,219u8,218u8,82u8,90u8,212u8,102u8,200u8,191u8,175u8,62u8,1u8,118u8,236u8,85u8,111u8,228u8,181u8,36u8,156u8,216u8,90u8,236u8,19u8,246u8,19u8,182u8,40u8,196u8,221u8,94u8,247u8,11u8,241u8,38u8,236u8,90u8,45u8,63u8,248u8,237u8,144u8,82u8,35u8,143u8,89u8,30u8,100u8,154u8,194u8,162u8,16u8,140u8,82u8,88u8,69u8,53u8,106u8,170u8,117u8,126u8,213u8,72u8,33u8,243u8,176u8,253u8,209u8,23u8,227u8,0u8,194u8,26u8,251u8,229u8,54u8,130u8,184u8,87u8,126u8,228u8,255u8,39u8,216u8,35u8,81u8,111u8,215u8,222u8,86u8,250u8,173u8,226u8,86u8,44u8,120u8,253u8,66u8,1u8,217u8,1u8,43u8,145u8,75u8,132u8,77u8,196u8,237u8,112u8,162u8,123u8,156u8,80u8,214u8,90u8,240u8,154u8,205u8,49u8,148u8,210u8,253u8,245u8,117u8,255u8,99u8,48u8,82u8,20u8,48u8,201u8,131u8,237u8,156u8,171u8,200u8,8u8,88u8,124u8,80u8,131u8,125u8,108u8,63u8,37u8,245u8,67u8,214u8,20u8,107u8,218u8,0u8,0u8,0u8,6u8,98u8,75u8,71u8,68u8,0u8,1u8,0u8,2u8,0u8,3u8,94u8,174u8,23u8,27u8,0u8,0u8,1u8,144u8,104u8,73u8,83u8,84u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,93u8,124u8,21u8,200u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_plte_suggested_palette_in_rgb());model_plte_suggested_palette_in_rgb();}
    let mut events=Events {r1:false,r3:false,r4:false};
    let prepared=crate::callers::still(&b,&STRICT,&mut events);
    match prepared {
        Ok(doc)=>{assert(doc.ihdr.view()==crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8});
            assert(doc.info.ranges_wf(b.len() as nat)) by {reveal(crate::walk::Doc::wf);}
        },
        Err(_)=>{assert(false);},
    }
    // The actual public call has no asserted final data/pixel outcome.
    let _public_result=crate::png::try_decode_still(&b);
}

#[verifier::spinoff_prover]
proof fn symbolic_anc_ok_iccp_303(b: Seq<u8>)
    requires b.len()==394,crate::model::mh(b)==Ok((crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![105u8,67u8,67u8,80u8],data:41nat,len:303nat,next:348nat,crc_ok:true,synthetic:false})),
        crate::model::first_nul(b.subrange(41,344),0)==Some(17nat),
        b.subrange(41,344).subrange(0,17)==seq![115u8,82u8,71u8,66u8,32u8,73u8,69u8,67u8,54u8,49u8,57u8,54u8,54u8,45u8,50u8,46u8,49u8],
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,348nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:356nat,len:22nat,next:382nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,382nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:390nat,len:0nat,next:394nat,crc_ok:true,synthetic:false})),
        b.subrange(41,344)[18]==0u8,
        crate::metadata_model::parse(crate::walk_model::Name::Iccp,b.subrange(41,344),2u8,8u8,0nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Iccp {name_len:17,profile_start:19,profile_len:284}),
    ensures accepted(m::MW_p(b,m::strict()),crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8}),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::metadata_model::parse);

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
}

// anc-ok/iCCP-303; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_anc_ok_iccp_303() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,1u8,47u8,105u8,67u8,67u8,80u8,115u8,82u8,71u8,66u8,32u8,73u8,69u8,67u8,54u8,49u8,57u8,54u8,54u8,45u8,50u8,46u8,49u8,0u8,0u8,120u8,156u8,99u8,96u8,100u8,98u8,102u8,97u8,101u8,99u8,231u8,224u8,228u8,226u8,230u8,225u8,229u8,227u8,23u8,16u8,20u8,18u8,22u8,17u8,21u8,19u8,151u8,144u8,148u8,146u8,150u8,145u8,149u8,147u8,87u8,80u8,84u8,82u8,86u8,81u8,85u8,83u8,215u8,208u8,212u8,210u8,214u8,209u8,213u8,211u8,55u8,48u8,52u8,50u8,54u8,49u8,53u8,51u8,183u8,176u8,180u8,178u8,182u8,177u8,181u8,179u8,119u8,112u8,116u8,114u8,118u8,113u8,117u8,115u8,247u8,240u8,244u8,242u8,246u8,241u8,245u8,243u8,15u8,8u8,12u8,10u8,14u8,9u8,13u8,11u8,143u8,136u8,140u8,138u8,142u8,137u8,141u8,139u8,79u8,72u8,76u8,74u8,78u8,73u8,77u8,75u8,207u8,200u8,204u8,202u8,206u8,201u8,205u8,203u8,47u8,40u8,44u8,42u8,46u8,41u8,45u8,43u8,175u8,168u8,172u8,170u8,174u8,169u8,173u8,171u8,111u8,104u8,108u8,106u8,110u8,105u8,109u8,107u8,239u8,232u8,236u8,234u8,238u8,233u8,237u8,235u8,159u8,48u8,113u8,210u8,228u8,41u8,83u8,167u8,77u8,159u8,49u8,115u8,214u8,236u8,57u8,115u8,231u8,205u8,95u8,176u8,112u8,209u8,226u8,37u8,75u8,151u8,45u8,95u8,177u8,114u8,213u8,234u8,53u8,107u8,215u8,173u8,223u8,176u8,113u8,211u8,230u8,45u8,91u8,183u8,109u8,223u8,177u8,115u8,215u8,238u8,61u8,123u8,247u8,237u8,63u8,112u8,240u8,208u8,225u8,35u8,71u8,143u8,29u8,63u8,113u8,242u8,212u8,233u8,51u8,103u8,207u8,157u8,191u8,112u8,241u8,210u8,229u8,43u8,87u8,175u8,93u8,191u8,113u8,243u8,214u8,237u8,59u8,119u8,239u8,221u8,127u8,240u8,240u8,209u8,227u8,39u8,79u8,159u8,61u8,127u8,241u8,242u8,213u8,235u8,55u8,111u8,223u8,189u8,255u8,240u8,241u8,211u8,231u8,47u8,95u8,191u8,125u8,255u8,241u8,243u8,215u8,239u8,63u8,127u8,255u8,253u8,103u8,24u8,245u8,255u8,136u8,246u8,63u8,0u8,160u8,98u8,126u8,144u8,16u8,123u8,222u8,207u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn accepted_anc_ok_iccp_303() -> bool {accepted(m::MW_p(bytes_anc_ok_iccp_303(),m::strict()),crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8})}
#[verifier::spinoff_prover]
proof fn model_anc_ok_iccp_303()
    ensures accepted_anc_ok_iccp_303(),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(m::MW_p);
    hide(crate::metadata_model::parse);

    assert(crate::model::mh(bytes_anc_ok_iccp_303())==Ok((crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_anc_ok_iccp_303())==Ok((crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_anc_ok_iccp_303(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![105u8,67u8,67u8,80u8],data:41nat,len:303nat,next:348nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_anc_ok_iccp_303(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![105u8,67u8,67u8,80u8],data:41nat,len:303nat,next:348nat,crc_ok:true,synthetic:false})) by (compute);
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
    symbolic_anc_ok_iccp_303(bytes_anc_ok_iccp_303());
}
// M3 projection only; final data outcome is an M4–M6 witness obligation, runtime-conformance-tested meanwhile
fn projection_anc_ok_iccp_303() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,1u8,47u8,105u8,67u8,67u8,80u8,115u8,82u8,71u8,66u8,32u8,73u8,69u8,67u8,54u8,49u8,57u8,54u8,54u8,45u8,50u8,46u8,49u8,0u8,0u8,120u8,156u8,99u8,96u8,100u8,98u8,102u8,97u8,101u8,99u8,231u8,224u8,228u8,226u8,230u8,225u8,229u8,227u8,23u8,16u8,20u8,18u8,22u8,17u8,21u8,19u8,151u8,144u8,148u8,146u8,150u8,145u8,149u8,147u8,87u8,80u8,84u8,82u8,86u8,81u8,85u8,83u8,215u8,208u8,212u8,210u8,214u8,209u8,213u8,211u8,55u8,48u8,52u8,50u8,54u8,49u8,53u8,51u8,183u8,176u8,180u8,178u8,182u8,177u8,181u8,179u8,119u8,112u8,116u8,114u8,118u8,113u8,117u8,115u8,247u8,240u8,244u8,242u8,246u8,241u8,245u8,243u8,15u8,8u8,12u8,10u8,14u8,9u8,13u8,11u8,143u8,136u8,140u8,138u8,142u8,137u8,141u8,139u8,79u8,72u8,76u8,74u8,78u8,73u8,77u8,75u8,207u8,200u8,204u8,202u8,206u8,201u8,205u8,203u8,47u8,40u8,44u8,42u8,46u8,41u8,45u8,43u8,175u8,168u8,172u8,170u8,174u8,169u8,173u8,171u8,111u8,104u8,108u8,106u8,110u8,105u8,109u8,107u8,239u8,232u8,236u8,234u8,238u8,233u8,237u8,235u8,159u8,48u8,113u8,210u8,228u8,41u8,83u8,167u8,77u8,159u8,49u8,115u8,214u8,236u8,57u8,115u8,231u8,205u8,95u8,176u8,112u8,209u8,226u8,37u8,75u8,151u8,45u8,95u8,177u8,114u8,213u8,234u8,53u8,107u8,215u8,173u8,223u8,176u8,113u8,211u8,230u8,45u8,91u8,183u8,109u8,223u8,177u8,115u8,215u8,238u8,61u8,123u8,247u8,237u8,63u8,112u8,240u8,208u8,225u8,35u8,71u8,143u8,29u8,63u8,113u8,242u8,212u8,233u8,51u8,103u8,207u8,157u8,191u8,112u8,241u8,210u8,229u8,43u8,87u8,175u8,93u8,191u8,113u8,243u8,214u8,237u8,59u8,119u8,239u8,221u8,127u8,240u8,240u8,209u8,227u8,39u8,79u8,159u8,61u8,127u8,241u8,242u8,213u8,235u8,55u8,111u8,223u8,189u8,255u8,240u8,241u8,211u8,231u8,47u8,95u8,191u8,125u8,255u8,241u8,243u8,215u8,239u8,63u8,127u8,255u8,253u8,103u8,24u8,245u8,255u8,136u8,246u8,63u8,0u8,160u8,98u8,126u8,144u8,16u8,123u8,222u8,207u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_anc_ok_iccp_303());model_anc_ok_iccp_303();}
    let mut events=Events {r1:false,r3:false,r4:false};
    let prepared=crate::callers::still(&b,&STRICT,&mut events);
    match prepared {
        Ok(doc)=>{assert(doc.ihdr.view()==crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8});
            assert(doc.info.ranges_wf(b.len() as nat)) by {reveal(crate::walk::Doc::wf);}
        },
        Err(_)=>{assert(false);},
    }
    // The actual public call has no asserted final data/pixel outcome.
    let _public_result=crate::png::try_decode_still(&b);
}

#[verifier::spinoff_prover]
proof fn symbolic_anc_ok_time_7_after_idat(b: Seq<u8>)
    requires b.len()==98,crate::model::mh(b)==Ok((crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8},33nat)),
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
    ensures accepted(m::MW_p(b,m::strict()),crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8}),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::metadata_model::parse);

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
}

// anc-ok/tIME-7-after-idat; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_anc_ok_time_7_after_idat() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,7u8,116u8,73u8,77u8,69u8,7u8,234u8,11u8,14u8,23u8,59u8,60u8,230u8,176u8,242u8,44u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn accepted_anc_ok_time_7_after_idat() -> bool {accepted(m::MW_p(bytes_anc_ok_time_7_after_idat(),m::strict()),crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8})}
#[verifier::spinoff_prover]
proof fn model_anc_ok_time_7_after_idat()
    ensures accepted_anc_ok_time_7_after_idat(),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(m::MW_p);
    hide(crate::metadata_model::parse);

    assert(crate::model::mh(bytes_anc_ok_time_7_after_idat())==Ok((crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_anc_ok_time_7_after_idat())==Ok((crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8},33nat))) by (compute);
    }
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
    symbolic_anc_ok_time_7_after_idat(bytes_anc_ok_time_7_after_idat());
}
// M3 projection only; final data outcome is an M4–M6 witness obligation, runtime-conformance-tested meanwhile
fn projection_anc_ok_time_7_after_idat() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,7u8,116u8,73u8,77u8,69u8,7u8,234u8,11u8,14u8,23u8,59u8,60u8,230u8,176u8,242u8,44u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_anc_ok_time_7_after_idat());model_anc_ok_time_7_after_idat();}
    let mut events=Events {r1:false,r3:false,r4:false};
    let prepared=crate::callers::still(&b,&STRICT,&mut events);
    match prepared {
        Ok(doc)=>{assert(doc.ihdr.view()==crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8});
            assert(doc.info.ranges_wf(b.len() as nat)) by {reveal(crate::walk::Doc::wf);}
        },
        Err(_)=>{assert(false);},
    }
    // The actual public call has no asserted final data/pixel outcome.
    let _public_result=crate::png::try_decode_still(&b);
}

#[verifier::spinoff_prover]
proof fn symbolic_anc_ok_srgb_and_iccp_both(b: Seq<u8>)
    requires b.len()==391,crate::model::mh(b)==Ok((crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![115u8,82u8,71u8,66u8],data:41nat,len:1nat,next:46nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,46nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![105u8,67u8,67u8,80u8],data:54nat,len:287nat,next:345nat,crc_ok:true,synthetic:false})),
        crate::model::first_nul(b.subrange(54,341),0)==Some(1nat),
        b.subrange(54,341).subrange(0,1)==seq![120u8],
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,345nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:353nat,len:22nat,next:379nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,379nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:387nat,len:0nat,next:391nat,crc_ok:true,synthetic:false})),
        crate::metadata_model::parse(crate::walk_model::Name::Srgb,b.subrange(41,42),2u8,8u8,0nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Srgb(1u8)),
        b.subrange(54,341)[2]==0u8,
        crate::metadata_model::parse(crate::walk_model::Name::Iccp,b.subrange(54,341),2u8,8u8,0nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Iccp {name_len:1,profile_start:3,profile_len:284}),
    ensures accepted(m::MW_p(b,m::strict()),crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8}),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::metadata_model::parse);

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
}

// anc-ok/srgb-and-iccp-both; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_anc_ok_srgb_and_iccp_both() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,1u8,115u8,82u8,71u8,66u8,1u8,217u8,201u8,44u8,127u8,0u8,0u8,1u8,31u8,105u8,67u8,67u8,80u8,120u8,0u8,0u8,120u8,156u8,99u8,96u8,100u8,98u8,102u8,97u8,101u8,99u8,231u8,224u8,228u8,226u8,230u8,225u8,229u8,227u8,23u8,16u8,20u8,18u8,22u8,17u8,21u8,19u8,151u8,144u8,148u8,146u8,150u8,145u8,149u8,147u8,87u8,80u8,84u8,82u8,86u8,81u8,85u8,83u8,215u8,208u8,212u8,210u8,214u8,209u8,213u8,211u8,55u8,48u8,52u8,50u8,54u8,49u8,53u8,51u8,183u8,176u8,180u8,178u8,182u8,177u8,181u8,179u8,119u8,112u8,116u8,114u8,118u8,113u8,117u8,115u8,247u8,240u8,244u8,242u8,246u8,241u8,245u8,243u8,15u8,8u8,12u8,10u8,14u8,9u8,13u8,11u8,143u8,136u8,140u8,138u8,142u8,137u8,141u8,139u8,79u8,72u8,76u8,74u8,78u8,73u8,77u8,75u8,207u8,200u8,204u8,202u8,206u8,201u8,205u8,203u8,47u8,40u8,44u8,42u8,46u8,41u8,45u8,43u8,175u8,168u8,172u8,170u8,174u8,169u8,173u8,171u8,111u8,104u8,108u8,106u8,110u8,105u8,109u8,107u8,239u8,232u8,236u8,234u8,238u8,233u8,237u8,235u8,159u8,48u8,113u8,210u8,228u8,41u8,83u8,167u8,77u8,159u8,49u8,115u8,214u8,236u8,57u8,115u8,231u8,205u8,95u8,176u8,112u8,209u8,226u8,37u8,75u8,151u8,45u8,95u8,177u8,114u8,213u8,234u8,53u8,107u8,215u8,173u8,223u8,176u8,113u8,211u8,230u8,45u8,91u8,183u8,109u8,223u8,177u8,115u8,215u8,238u8,61u8,123u8,247u8,237u8,63u8,112u8,240u8,208u8,225u8,35u8,71u8,143u8,29u8,63u8,113u8,242u8,212u8,233u8,51u8,103u8,207u8,157u8,191u8,112u8,241u8,210u8,229u8,43u8,87u8,175u8,93u8,191u8,113u8,243u8,214u8,237u8,59u8,119u8,239u8,221u8,127u8,240u8,240u8,209u8,227u8,39u8,79u8,159u8,61u8,127u8,241u8,242u8,213u8,235u8,55u8,111u8,223u8,189u8,255u8,240u8,241u8,211u8,231u8,47u8,95u8,191u8,125u8,255u8,241u8,243u8,215u8,239u8,63u8,127u8,255u8,253u8,103u8,24u8,245u8,255u8,136u8,246u8,63u8,0u8,160u8,98u8,126u8,144u8,17u8,127u8,182u8,55u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn accepted_anc_ok_srgb_and_iccp_both() -> bool {accepted(m::MW_p(bytes_anc_ok_srgb_and_iccp_both(),m::strict()),crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8})}
#[verifier::spinoff_prover]
proof fn model_anc_ok_srgb_and_iccp_both()
    ensures accepted_anc_ok_srgb_and_iccp_both(),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(m::MW_p);
    hide(crate::metadata_model::parse);

    assert(crate::model::mh(bytes_anc_ok_srgb_and_iccp_both())==Ok((crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_anc_ok_srgb_and_iccp_both())==Ok((crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8},33nat))) by (compute);
    }
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
            assert(crate::critical_model::read(bytes_anc_ok_srgb_and_iccp_both(),46nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![105u8,67u8,67u8,80u8],data:54nat,len:287nat,next:345nat,crc_ok:true,synthetic:false})) by (compute);
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
    symbolic_anc_ok_srgb_and_iccp_both(bytes_anc_ok_srgb_and_iccp_both());
}
// M3 projection only; final data outcome is an M4–M6 witness obligation, runtime-conformance-tested meanwhile
fn projection_anc_ok_srgb_and_iccp_both() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,1u8,115u8,82u8,71u8,66u8,1u8,217u8,201u8,44u8,127u8,0u8,0u8,1u8,31u8,105u8,67u8,67u8,80u8,120u8,0u8,0u8,120u8,156u8,99u8,96u8,100u8,98u8,102u8,97u8,101u8,99u8,231u8,224u8,228u8,226u8,230u8,225u8,229u8,227u8,23u8,16u8,20u8,18u8,22u8,17u8,21u8,19u8,151u8,144u8,148u8,146u8,150u8,145u8,149u8,147u8,87u8,80u8,84u8,82u8,86u8,81u8,85u8,83u8,215u8,208u8,212u8,210u8,214u8,209u8,213u8,211u8,55u8,48u8,52u8,50u8,54u8,49u8,53u8,51u8,183u8,176u8,180u8,178u8,182u8,177u8,181u8,179u8,119u8,112u8,116u8,114u8,118u8,113u8,117u8,115u8,247u8,240u8,244u8,242u8,246u8,241u8,245u8,243u8,15u8,8u8,12u8,10u8,14u8,9u8,13u8,11u8,143u8,136u8,140u8,138u8,142u8,137u8,141u8,139u8,79u8,72u8,76u8,74u8,78u8,73u8,77u8,75u8,207u8,200u8,204u8,202u8,206u8,201u8,205u8,203u8,47u8,40u8,44u8,42u8,46u8,41u8,45u8,43u8,175u8,168u8,172u8,170u8,174u8,169u8,173u8,171u8,111u8,104u8,108u8,106u8,110u8,105u8,109u8,107u8,239u8,232u8,236u8,234u8,238u8,233u8,237u8,235u8,159u8,48u8,113u8,210u8,228u8,41u8,83u8,167u8,77u8,159u8,49u8,115u8,214u8,236u8,57u8,115u8,231u8,205u8,95u8,176u8,112u8,209u8,226u8,37u8,75u8,151u8,45u8,95u8,177u8,114u8,213u8,234u8,53u8,107u8,215u8,173u8,223u8,176u8,113u8,211u8,230u8,45u8,91u8,183u8,109u8,223u8,177u8,115u8,215u8,238u8,61u8,123u8,247u8,237u8,63u8,112u8,240u8,208u8,225u8,35u8,71u8,143u8,29u8,63u8,113u8,242u8,212u8,233u8,51u8,103u8,207u8,157u8,191u8,112u8,241u8,210u8,229u8,43u8,87u8,175u8,93u8,191u8,113u8,243u8,214u8,237u8,59u8,119u8,239u8,221u8,127u8,240u8,240u8,209u8,227u8,39u8,79u8,159u8,61u8,127u8,241u8,242u8,213u8,235u8,55u8,111u8,223u8,189u8,255u8,240u8,241u8,211u8,231u8,47u8,95u8,191u8,125u8,255u8,241u8,243u8,215u8,239u8,63u8,127u8,255u8,253u8,103u8,24u8,245u8,255u8,136u8,246u8,63u8,0u8,160u8,98u8,126u8,144u8,17u8,127u8,182u8,55u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_anc_ok_srgb_and_iccp_both());model_anc_ok_srgb_and_iccp_both();}
    let mut events=Events {r1:false,r3:false,r4:false};
    let prepared=crate::callers::still(&b,&STRICT,&mut events);
    match prepared {
        Ok(doc)=>{assert(doc.ihdr.view()==crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8});
            assert(doc.info.ranges_wf(b.len() as nat)) by {reveal(crate::walk::Doc::wf);}
        },
        Err(_)=>{assert(false);},
    }
    // The actual public call has no asserted final data/pixel outcome.
    let _public_result=crate::png::try_decode_still(&b);
}

#[verifier::spinoff_prover]
proof fn symbolic_anc_ok_many_text(b: Seq<u8>)
    requires b.len()==159,crate::model::mh(b)==Ok((crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8},33nat)),
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
    ensures accepted(m::MW_p(b,m::strict()),crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8}),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::metadata_model::parse);

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
}

// anc-ok/many-text; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_anc_ok_many_text() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,4u8,116u8,69u8,88u8,116u8,107u8,48u8,0u8,118u8,198u8,174u8,151u8,211u8,0u8,0u8,0u8,4u8,116u8,69u8,88u8,116u8,107u8,49u8,0u8,118u8,199u8,108u8,253u8,228u8,0u8,0u8,0u8,4u8,116u8,69u8,88u8,116u8,107u8,50u8,0u8,118u8,197u8,42u8,67u8,189u8,0u8,0u8,0u8,4u8,116u8,69u8,88u8,116u8,107u8,51u8,0u8,118u8,196u8,232u8,41u8,138u8,0u8,0u8,0u8,4u8,116u8,69u8,88u8,116u8,107u8,52u8,0u8,118u8,193u8,167u8,63u8,15u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn accepted_anc_ok_many_text() -> bool {accepted(m::MW_p(bytes_anc_ok_many_text(),m::strict()),crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8})}
#[verifier::spinoff_prover]
proof fn model_anc_ok_many_text()
    ensures accepted_anc_ok_many_text(),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(m::MW_p);
    hide(crate::metadata_model::parse);

    assert(crate::model::mh(bytes_anc_ok_many_text())==Ok((crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_anc_ok_many_text())==Ok((crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8},33nat))) by (compute);
    }
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
    symbolic_anc_ok_many_text(bytes_anc_ok_many_text());
}
// M3 projection only; final data outcome is an M4–M6 witness obligation, runtime-conformance-tested meanwhile
fn projection_anc_ok_many_text() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,4u8,116u8,69u8,88u8,116u8,107u8,48u8,0u8,118u8,198u8,174u8,151u8,211u8,0u8,0u8,0u8,4u8,116u8,69u8,88u8,116u8,107u8,49u8,0u8,118u8,199u8,108u8,253u8,228u8,0u8,0u8,0u8,4u8,116u8,69u8,88u8,116u8,107u8,50u8,0u8,118u8,197u8,42u8,67u8,189u8,0u8,0u8,0u8,4u8,116u8,69u8,88u8,116u8,107u8,51u8,0u8,118u8,196u8,232u8,41u8,138u8,0u8,0u8,0u8,4u8,116u8,69u8,88u8,116u8,107u8,52u8,0u8,118u8,193u8,167u8,63u8,15u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_anc_ok_many_text());model_anc_ok_many_text();}
    let mut events=Events {r1:false,r3:false,r4:false};
    let prepared=crate::callers::still(&b,&STRICT,&mut events);
    match prepared {
        Ok(doc)=>{assert(doc.ihdr.view()==crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8});
            assert(doc.info.ranges_wf(b.len() as nat)) by {reveal(crate::walk::Doc::wf);}
        },
        Err(_)=>{assert(false);},
    }
    // The actual public call has no asserted final data/pixel outcome.
    let _public_result=crate::png::try_decode_still(&b);
}

#[verifier::spinoff_prover]
proof fn symbolic_anc_ok_ztxt_with_corrupt_zlib_not_inflated(b: Seq<u8>)
    requires b.len()==101,crate::model::mh(b)==Ok((crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![122u8,84u8,88u8,116u8],data:41nat,len:10nat,next:55nat,crc_ok:true,synthetic:false})),
        crate::model::first_nul(b.subrange(41,51),0)==Some(1nat),
        b.subrange(41,51).subrange(0,1)==seq![107u8],
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,55nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:63nat,len:22nat,next:89nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,89nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:97nat,len:0nat,next:101nat,crc_ok:true,synthetic:false})),
        b.subrange(41,51)[2]==0u8,
        crate::metadata_model::parse(crate::walk_model::Name::Ztxt,b.subrange(41,51),2u8,8u8,0nat)==crate::metadata_model::Outcome::Value(crate::metadata_model::Value::Ignored),
    ensures accepted(m::MW_p(b,m::strict()),crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8}),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::metadata_model::parse);

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
}

// anc-ok/ztxt-with-corrupt-zlib-not-inflated; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_anc_ok_ztxt_with_corrupt_zlib_not_inflated() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,10u8,122u8,84u8,88u8,116u8,107u8,0u8,0u8,103u8,97u8,114u8,98u8,97u8,103u8,101u8,188u8,174u8,84u8,233u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn accepted_anc_ok_ztxt_with_corrupt_zlib_not_inflated() -> bool {accepted(m::MW_p(bytes_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),m::strict()),crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8})}
#[verifier::spinoff_prover]
proof fn model_anc_ok_ztxt_with_corrupt_zlib_not_inflated()
    ensures accepted_anc_ok_ztxt_with_corrupt_zlib_not_inflated(),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(m::MW_p);
    hide(crate::metadata_model::parse);

    assert(crate::model::mh(bytes_anc_ok_ztxt_with_corrupt_zlib_not_inflated())==Ok((crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_anc_ok_ztxt_with_corrupt_zlib_not_inflated())==Ok((crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8},33nat))) by (compute);
    }
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
    symbolic_anc_ok_ztxt_with_corrupt_zlib_not_inflated(bytes_anc_ok_ztxt_with_corrupt_zlib_not_inflated());
}
// M3 projection only; final data outcome is an M4–M6 witness obligation, runtime-conformance-tested meanwhile
fn projection_anc_ok_ztxt_with_corrupt_zlib_not_inflated() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,10u8,122u8,84u8,88u8,116u8,107u8,0u8,0u8,103u8,97u8,114u8,98u8,97u8,103u8,101u8,188u8,174u8,84u8,233u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_anc_ok_ztxt_with_corrupt_zlib_not_inflated());model_anc_ok_ztxt_with_corrupt_zlib_not_inflated();}
    let mut events=Events {r1:false,r3:false,r4:false};
    let prepared=crate::callers::still(&b,&STRICT,&mut events);
    match prepared {
        Ok(doc)=>{assert(doc.ihdr.view()==crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8});
            assert(doc.info.ranges_wf(b.len() as nat)) by {reveal(crate::walk::Doc::wf);}
        },
        Err(_)=>{assert(false);},
    }
    // The actual public call has no asserted final data/pixel outcome.
    let _public_result=crate::png::try_decode_still(&b);
}

#[verifier::spinoff_prover]
proof fn symbolic_order_splt_twice(b: Seq<u8>)
    requires b.len()==109,crate::model::mh(b)==Ok((crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8},33nat)),
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
    ensures accepted(m::MW_p(b,m::strict()),crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8}),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::metadata_model::parse);

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
}

// order/splt-twice; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_order_splt_twice() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,3u8,115u8,80u8,76u8,84u8,97u8,0u8,8u8,210u8,192u8,25u8,167u8,0u8,0u8,0u8,3u8,115u8,80u8,76u8,84u8,98u8,0u8,8u8,208u8,134u8,167u8,254u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn accepted_order_splt_twice() -> bool {accepted(m::MW_p(bytes_order_splt_twice(),m::strict()),crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8})}
#[verifier::spinoff_prover]
proof fn model_order_splt_twice()
    ensures accepted_order_splt_twice(),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(m::MW_p);
    hide(crate::metadata_model::parse);

    assert(crate::model::mh(bytes_order_splt_twice())==Ok((crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_order_splt_twice())==Ok((crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8},33nat))) by (compute);
    }
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
    symbolic_order_splt_twice(bytes_order_splt_twice());
}
// M3 projection only; final data outcome is an M4–M6 witness obligation, runtime-conformance-tested meanwhile
fn projection_order_splt_twice() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,3u8,115u8,80u8,76u8,84u8,97u8,0u8,8u8,210u8,192u8,25u8,167u8,0u8,0u8,0u8,3u8,115u8,80u8,76u8,84u8,98u8,0u8,8u8,208u8,134u8,167u8,254u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_order_splt_twice());model_order_splt_twice();}
    let mut events=Events {r1:false,r3:false,r4:false};
    let prepared=crate::callers::still(&b,&STRICT,&mut events);
    match prepared {
        Ok(doc)=>{assert(doc.ihdr.view()==crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8});
            assert(doc.info.ranges_wf(b.len() as nat)) by {reveal(crate::walk::Doc::wf);}
        },
        Err(_)=>{assert(false);},
    }
    // The actual public call has no asserted final data/pixel outcome.
    let _public_result=crate::png::try_decode_still(&b);
}

#[verifier::spinoff_prover]
proof fn symbolic_order_bkgd_hist_after_plte_before_idat_rgb(b: Seq<u8>)
    requires b.len()==136,crate::model::mh(b)==Ok((crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8},33nat)),
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
    ensures accepted(m::MW_p(b,m::strict()),crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8}),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(crate::metadata_model::parse);

    hide(m::store);
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
}

// order/bkgd-hist-after-plte-before-idat-rgb; fixture sha256 641d9b0fb7d941688591cb07b1b8461b613dcaaaa2c34fb5807f6cfbc5fe763d
spec fn bytes_order_bkgd_hist_after_plte_before_idat_rgb() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,9u8,80u8,76u8,84u8,69u8,47u8,194u8,40u8,139u8,44u8,88u8,228u8,36u8,202u8,64u8,183u8,32u8,36u8,0u8,0u8,0u8,6u8,104u8,73u8,83u8,84u8,0u8,0u8,0u8,0u8,0u8,0u8,95u8,184u8,160u8,47u8,0u8,0u8,0u8,6u8,98u8,75u8,71u8,68u8,0u8,0u8,0u8,0u8,0u8,0u8,249u8,67u8,187u8,127u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn accepted_order_bkgd_hist_after_plte_before_idat_rgb() -> bool {accepted(m::MW_p(bytes_order_bkgd_hist_after_plte_before_idat_rgb(),m::strict()),crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8})}
#[verifier::spinoff_prover]
proof fn model_order_bkgd_hist_after_plte_before_idat_rgb()
    ensures accepted_order_bkgd_hist_after_plte_before_idat_rgb(),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(m::MW_p);
    hide(crate::metadata_model::parse);

    assert(crate::model::mh(bytes_order_bkgd_hist_after_plte_before_idat_rgb())==Ok((crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_order_bkgd_hist_after_plte_before_idat_rgb())==Ok((crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8},33nat))) by (compute);
    }
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
    symbolic_order_bkgd_hist_after_plte_before_idat_rgb(bytes_order_bkgd_hist_after_plte_before_idat_rgb());
}
// M3 projection only; final data outcome is an M4–M6 witness obligation, runtime-conformance-tested meanwhile
fn projection_order_bkgd_hist_after_plte_before_idat_rgb() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,2u8,8u8,2u8,0u8,0u8,0u8,253u8,212u8,154u8,115u8,0u8,0u8,0u8,9u8,80u8,76u8,84u8,69u8,47u8,194u8,40u8,139u8,44u8,88u8,228u8,36u8,202u8,64u8,183u8,32u8,36u8,0u8,0u8,0u8,6u8,104u8,73u8,83u8,84u8,0u8,0u8,0u8,0u8,0u8,0u8,95u8,184u8,160u8,47u8,0u8,0u8,0u8,6u8,98u8,75u8,71u8,68u8,0u8,0u8,0u8,0u8,0u8,0u8,249u8,67u8,187u8,127u8,0u8,0u8,0u8,22u8,73u8,68u8,65u8,84u8,120u8,156u8,99u8,56u8,233u8,105u8,127u8,202u8,123u8,58u8,227u8,67u8,41u8,113u8,233u8,59u8,89u8,0u8,40u8,65u8,5u8,114u8,253u8,235u8,23u8,167u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_order_bkgd_hist_after_plte_before_idat_rgb());model_order_bkgd_hist_after_plte_before_idat_rgb();}
    let mut events=Events {r1:false,r3:false,r4:false};
    let prepared=crate::callers::still(&b,&STRICT,&mut events);
    match prepared {
        Ok(doc)=>{assert(doc.ihdr.view()==crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8});
            assert(doc.info.ranges_wf(b.len() as nat)) by {reveal(crate::walk::Doc::wf);}
        },
        Err(_)=>{assert(false);},
    }
    // The actual public call has no asserted final data/pixel outcome.
    let _public_result=crate::png::try_decode_still(&b);
}

#[verifier::spinoff_prover]
proof fn symbolic_ihdr_8192x8192_accepted_then_truncated_zlib(b: Seq<u8>)
    requires b.len()==59,crate::model::mh(b)==Ok((crate::model::Ihdr {width:crate::model::be32(b,16),height:crate::model::be32(b,20),depth:8u8,color_type:6u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:2nat,next:47nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,47nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:55nat,len:0nat,next:59nat,crc_ok:true,synthetic:false})),
    ensures accepted(m::MW_p(b,m::strict()),crate::model::Ihdr {width:crate::model::be32(b,16),height:crate::model::be32(b,20),depth:8u8,color_type:6u8,interlace:0u8}),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);


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
}

// ihdr/8192x8192-accepted-then-truncated-zlib; fixture sha256 0e80fe9140a18411a00581298a05c50061c80cfb162990770c3a2df6c3965726
spec fn bytes_ihdr_8192x8192_accepted_then_truncated_zlib() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,32u8,0u8,0u8,0u8,32u8,0u8,8u8,6u8,0u8,0u8,0u8,114u8,170u8,202u8,89u8,0u8,0u8,0u8,2u8,73u8,68u8,65u8,84u8,120u8,1u8,236u8,26u8,126u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn accepted_ihdr_8192x8192_accepted_then_truncated_zlib() -> bool {accepted(m::MW_p(bytes_ihdr_8192x8192_accepted_then_truncated_zlib(),m::strict()),crate::model::Ihdr {width:crate::model::be32(bytes_ihdr_8192x8192_accepted_then_truncated_zlib(),16),height:crate::model::be32(bytes_ihdr_8192x8192_accepted_then_truncated_zlib(),20),depth:8u8,color_type:6u8,interlace:0u8})}
#[verifier::spinoff_prover]
proof fn model_ihdr_8192x8192_accepted_then_truncated_zlib()
    ensures accepted_ihdr_8192x8192_accepted_then_truncated_zlib(),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(m::MW_p);


    assert(crate::model::mh(bytes_ihdr_8192x8192_accepted_then_truncated_zlib())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_ihdr_8192x8192_accepted_then_truncated_zlib(),16),height:crate::model::be32(bytes_ihdr_8192x8192_accepted_then_truncated_zlib(),20),depth:8u8,color_type:6u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_ihdr_8192x8192_accepted_then_truncated_zlib())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_ihdr_8192x8192_accepted_then_truncated_zlib(),16),height:crate::model::be32(bytes_ihdr_8192x8192_accepted_then_truncated_zlib(),20),depth:8u8,color_type:6u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_ihdr_8192x8192_accepted_then_truncated_zlib(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:2nat,next:47nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_ihdr_8192x8192_accepted_then_truncated_zlib(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:2nat,next:47nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_ihdr_8192x8192_accepted_then_truncated_zlib(),47nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:55nat,len:0nat,next:59nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_ihdr_8192x8192_accepted_then_truncated_zlib(),47nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:55nat,len:0nat,next:59nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    symbolic_ihdr_8192x8192_accepted_then_truncated_zlib(bytes_ihdr_8192x8192_accepted_then_truncated_zlib());
}
// M3 projection only; final data outcome is an M4–M6 witness obligation, runtime-conformance-tested meanwhile
fn projection_ihdr_8192x8192_accepted_then_truncated_zlib() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,32u8,0u8,0u8,0u8,32u8,0u8,8u8,6u8,0u8,0u8,0u8,114u8,170u8,202u8,89u8,0u8,0u8,0u8,2u8,73u8,68u8,65u8,84u8,120u8,1u8,236u8,26u8,126u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_ihdr_8192x8192_accepted_then_truncated_zlib());model_ihdr_8192x8192_accepted_then_truncated_zlib();}
    let mut events=Events {r1:false,r3:false,r4:false};
    let prepared=crate::callers::still(&b,&STRICT,&mut events);
    match prepared {
        Ok(doc)=>{assert(doc.ihdr.view()==crate::model::Ihdr {width:crate::model::be32(bytes_ihdr_8192x8192_accepted_then_truncated_zlib(),16),height:crate::model::be32(bytes_ihdr_8192x8192_accepted_then_truncated_zlib(),20),depth:8u8,color_type:6u8,interlace:0u8});
            assert(doc.info.ranges_wf(b.len() as nat)) by {reveal(crate::walk::Doc::wf);}
        },
        Err(_)=>{assert(false);},
    }
    // The actual public call has no asserted final data/pixel outcome.
    let _public_result=crate::png::try_decode_still(&b);
}

#[verifier::spinoff_prover]
proof fn symbolic_ihdr_67108864x1_accepted_then_truncated_zlib(b: Seq<u8>)
    requires b.len()==59,crate::model::mh(b)==Ok((crate::model::Ihdr {width:crate::model::be32(b,16),height:crate::model::be32(b,20),depth:8u8,color_type:6u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:2nat,next:47nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,47nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:55nat,len:0nat,next:59nat,crc_ok:true,synthetic:false})),
    ensures accepted(m::MW_p(b,m::strict()),crate::model::Ihdr {width:crate::model::be32(b,16),height:crate::model::be32(b,20),depth:8u8,color_type:6u8,interlace:0u8}),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);


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
}

// ihdr/67108864x1-accepted-then-truncated-zlib; fixture sha256 0e80fe9140a18411a00581298a05c50061c80cfb162990770c3a2df6c3965726
spec fn bytes_ihdr_67108864x1_accepted_then_truncated_zlib() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,4u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,8u8,6u8,0u8,0u8,0u8,48u8,110u8,197u8,33u8,0u8,0u8,0u8,2u8,73u8,68u8,65u8,84u8,120u8,1u8,236u8,26u8,126u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn accepted_ihdr_67108864x1_accepted_then_truncated_zlib() -> bool {accepted(m::MW_p(bytes_ihdr_67108864x1_accepted_then_truncated_zlib(),m::strict()),crate::model::Ihdr {width:crate::model::be32(bytes_ihdr_67108864x1_accepted_then_truncated_zlib(),16),height:crate::model::be32(bytes_ihdr_67108864x1_accepted_then_truncated_zlib(),20),depth:8u8,color_type:6u8,interlace:0u8})}
#[verifier::spinoff_prover]
proof fn model_ihdr_67108864x1_accepted_then_truncated_zlib()
    ensures accepted_ihdr_67108864x1_accepted_then_truncated_zlib(),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(m::MW_p);


    assert(crate::model::mh(bytes_ihdr_67108864x1_accepted_then_truncated_zlib())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_ihdr_67108864x1_accepted_then_truncated_zlib(),16),height:crate::model::be32(bytes_ihdr_67108864x1_accepted_then_truncated_zlib(),20),depth:8u8,color_type:6u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_ihdr_67108864x1_accepted_then_truncated_zlib())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_ihdr_67108864x1_accepted_then_truncated_zlib(),16),height:crate::model::be32(bytes_ihdr_67108864x1_accepted_then_truncated_zlib(),20),depth:8u8,color_type:6u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_ihdr_67108864x1_accepted_then_truncated_zlib(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:2nat,next:47nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_ihdr_67108864x1_accepted_then_truncated_zlib(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:2nat,next:47nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_ihdr_67108864x1_accepted_then_truncated_zlib(),47nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:55nat,len:0nat,next:59nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_ihdr_67108864x1_accepted_then_truncated_zlib(),47nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:55nat,len:0nat,next:59nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    symbolic_ihdr_67108864x1_accepted_then_truncated_zlib(bytes_ihdr_67108864x1_accepted_then_truncated_zlib());
}
// M3 projection only; final data outcome is an M4–M6 witness obligation, runtime-conformance-tested meanwhile
fn projection_ihdr_67108864x1_accepted_then_truncated_zlib() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,4u8,0u8,0u8,0u8,0u8,0u8,0u8,1u8,8u8,6u8,0u8,0u8,0u8,48u8,110u8,197u8,33u8,0u8,0u8,0u8,2u8,73u8,68u8,65u8,84u8,120u8,1u8,236u8,26u8,126u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_ihdr_67108864x1_accepted_then_truncated_zlib());model_ihdr_67108864x1_accepted_then_truncated_zlib();}
    let mut events=Events {r1:false,r3:false,r4:false};
    let prepared=crate::callers::still(&b,&STRICT,&mut events);
    match prepared {
        Ok(doc)=>{assert(doc.ihdr.view()==crate::model::Ihdr {width:crate::model::be32(bytes_ihdr_67108864x1_accepted_then_truncated_zlib(),16),height:crate::model::be32(bytes_ihdr_67108864x1_accepted_then_truncated_zlib(),20),depth:8u8,color_type:6u8,interlace:0u8});
            assert(doc.info.ranges_wf(b.len() as nat)) by {reveal(crate::walk::Doc::wf);}
        },
        Err(_)=>{assert(false);},
    }
    // The actual public call has no asserted final data/pixel outcome.
    let _public_result=crate::png::try_decode_still(&b);
}

#[verifier::spinoff_prover]
proof fn symbolic_ihdr_1x67108864_accepted_then_truncated_zlib(b: Seq<u8>)
    requires b.len()==59,crate::model::mh(b)==Ok((crate::model::Ihdr {width:crate::model::be32(b,16),height:crate::model::be32(b,20),depth:8u8,color_type:6u8,interlace:0u8},33nat)),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:2nat,next:47nat,crc_ok:true,synthetic:false})),
        (forall|phase: crate::walk_model::Phase| crate::critical_model::read(b,47nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:55nat,len:0nat,next:59nat,crc_ok:true,synthetic:false})),
    ensures accepted(m::MW_p(b,m::strict()),crate::model::Ihdr {width:crate::model::be32(b,16),height:crate::model::be32(b,20),depth:8u8,color_type:6u8,interlace:0u8}),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);


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
}

// ihdr/1x67108864-accepted-then-truncated-zlib; fixture sha256 0e80fe9140a18411a00581298a05c50061c80cfb162990770c3a2df6c3965726
spec fn bytes_ihdr_1x67108864_accepted_then_truncated_zlib() -> Seq<u8> {seq![137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,4u8,0u8,0u8,0u8,8u8,6u8,0u8,0u8,0u8,137u8,165u8,70u8,32u8,0u8,0u8,0u8,2u8,73u8,68u8,65u8,84u8,120u8,1u8,236u8,26u8,126u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8]}
spec fn accepted_ihdr_1x67108864_accepted_then_truncated_zlib() -> bool {accepted(m::MW_p(bytes_ihdr_1x67108864_accepted_then_truncated_zlib(),m::strict()),crate::model::Ihdr {width:crate::model::be32(bytes_ihdr_1x67108864_accepted_then_truncated_zlib(),16),height:crate::model::be32(bytes_ihdr_1x67108864_accepted_then_truncated_zlib(),20),depth:8u8,color_type:6u8,interlace:0u8})}
#[verifier::spinoff_prover]
proof fn model_ihdr_1x67108864_accepted_then_truncated_zlib()
    ensures accepted_ihdr_1x67108864_accepted_then_truncated_zlib(),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
    hide(m::MW_p);


    assert(crate::model::mh(bytes_ihdr_1x67108864_accepted_then_truncated_zlib())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_ihdr_1x67108864_accepted_then_truncated_zlib(),16),height:crate::model::be32(bytes_ihdr_1x67108864_accepted_then_truncated_zlib(),20),depth:8u8,color_type:6u8,interlace:0u8},33nat))) by {
        reveal(crate::model::mh);
        assert(crate::model::mh(bytes_ihdr_1x67108864_accepted_then_truncated_zlib())==Ok((crate::model::Ihdr {width:crate::model::be32(bytes_ihdr_1x67108864_accepted_then_truncated_zlib(),16),height:crate::model::be32(bytes_ihdr_1x67108864_accepted_then_truncated_zlib(),20),depth:8u8,color_type:6u8,interlace:0u8},33nat))) by (compute);
    }
        assert((1u32 << 8u8)==256u32) by (compute);
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_ihdr_1x67108864_accepted_then_truncated_zlib(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:2nat,next:47nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_ihdr_1x67108864_accepted_then_truncated_zlib(),33nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,68u8,65u8,84u8],data:41nat,len:2nat,next:47nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,68u8,65u8,84u8])==crate::walk_model::Name::Idat) by (compute);
        }
        assert forall|phase: crate::walk_model::Phase| crate::critical_model::read(bytes_ihdr_1x67108864_accepted_then_truncated_zlib(),47nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:55nat,len:0nat,next:59nat,crc_ok:true,synthetic:false}) by {
            reveal(crate::critical_model::read);
            assert(crate::critical_model::read(bytes_ihdr_1x67108864_accepted_then_truncated_zlib(),47nat,phase,false)==Ok(crate::critical_model::Candidate {ty:seq![73u8,69u8,78u8,68u8],data:55nat,len:0nat,next:59nat,crc_ok:true,synthetic:false})) by (compute);
        }
        assert((73u8 & 32u8)==0u8) by (bit_vector);
        assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by {
            reveal(crate::walk_model::classify);
            assert(crate::walk_model::classify(seq![73u8,69u8,78u8,68u8])==crate::walk_model::Name::Iend) by (compute);
        }
    symbolic_ihdr_1x67108864_accepted_then_truncated_zlib(bytes_ihdr_1x67108864_accepted_then_truncated_zlib());
}
// M3 projection only; final data outcome is an M4–M6 witness obligation, runtime-conformance-tested meanwhile
fn projection_ihdr_1x67108864_accepted_then_truncated_zlib() {
    hide(m::MW_p);
    let b=[137u8,80u8,78u8,71u8,13u8,10u8,26u8,10u8,0u8,0u8,0u8,13u8,73u8,72u8,68u8,82u8,0u8,0u8,0u8,1u8,4u8,0u8,0u8,0u8,8u8,6u8,0u8,0u8,0u8,137u8,165u8,70u8,32u8,0u8,0u8,0u8,2u8,73u8,68u8,65u8,84u8,120u8,1u8,236u8,26u8,126u8,210u8,0u8,0u8,0u8,0u8,73u8,69u8,78u8,68u8,174u8,66u8,96u8,130u8];
    proof {assert(b@==bytes_ihdr_1x67108864_accepted_then_truncated_zlib());model_ihdr_1x67108864_accepted_then_truncated_zlib();}
    let mut events=Events {r1:false,r3:false,r4:false};
    let prepared=crate::callers::still(&b,&STRICT,&mut events);
    match prepared {
        Ok(doc)=>{assert(doc.ihdr.view()==crate::model::Ihdr {width:crate::model::be32(bytes_ihdr_1x67108864_accepted_then_truncated_zlib(),16),height:crate::model::be32(bytes_ihdr_1x67108864_accepted_then_truncated_zlib(),20),depth:8u8,color_type:6u8,interlace:0u8});
            assert(doc.info.ranges_wf(b.len() as nat)) by {reveal(crate::walk::Doc::wf);}
        },
        Err(_)=>{assert(false);},
    }
    // The actual public call has no asserted final data/pixel outcome.
    let _public_result=crate::png::try_decode_still(&b);
}

} // verus!
