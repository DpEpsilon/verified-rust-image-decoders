//! Whole-walk/preparation projections from authored frozen fixtures.
//! M3 projection only; final data outcome is an M4–M6 witness obligation, runtime-conformance-tested meanwhile.
use vstd::prelude::*;
use crate::{document_model as m,png::Events,policy::STRICT};

verus! {
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
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);hide(m::MW_p);
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
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);hide(m::MW_p);
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
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);hide(m::MW_p);
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
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);hide(m::MW_p);
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
        b.subrange(56,59)[2]==8u8,
    ensures accepted(m::MW_p(b,m::strict()),crate::model::Ihdr {width:2,height:2,depth:8u8,color_type:2u8,interlace:0u8}),
{
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);
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
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);hide(m::MW_p);
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
        assert(bytes_order_splt_twice().subrange(56,59)[2]==8u8) by (compute);
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
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);hide(m::MW_p);
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
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);hide(m::MW_p);
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
    hide(crate::critical_model::read);hide(crate::model::mh);hide(m::drive);hide(m::initial_info);hide(crate::walk_model::classify);hide(m::MW_p);
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
